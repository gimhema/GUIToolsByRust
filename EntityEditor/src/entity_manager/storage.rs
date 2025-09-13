use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::Write,
    path::Path,
};

use anyhow::{Context, Result};
use csv::{ReaderBuilder, StringRecord, WriterBuilder};

use super::dyn_entity::DynRow;
use super::schema::{ColumnDef, DataType, TableSchema};

/// 아주 가벼운 타입 추론: 전부 Int면 Int, 전부 수치면 Float, 그 외 Text
fn infer_dtype(samples: &[&str]) -> DataType {
    let is_int = |s: &str| s.parse::<i64>().is_ok();
    let is_float = |s: &str| s.parse::<f64>().is_ok();

    if !samples.is_empty() && samples.iter().all(|s| is_int(s)) {
        DataType::Int
    } else if samples.iter().all(|s| s.is_empty() || is_float(s)) {
        DataType::Float
    } else {
        DataType::Text
    }
}

/// CSV를 헤더/미지의 컬럼까지 보존하여 읽기.
/// 반환: (스키마, key->행)
pub fn load_table(path: &str, key_hint: &str) -> Result<(TableSchema, BTreeMap<String, DynRow>)> {
    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .from_path(path)
        .with_context(|| format!("open {}", path))?;

    let headers: StringRecord = rdr.headers()?.clone();

    // 키 컬럼 선택: 정확 일치 > 대소문자 무시 일치 > 첫 컬럼
    let key_col = headers
        .iter()
        .find(|h| *h == key_hint)
        .or_else(|| headers.iter().find(|h| h.eq_ignore_ascii_case(key_hint)))
        .unwrap_or_else(|| headers.get(0).unwrap_or("id"));

    // dtype 추론을 위한 샘플 수집
    let mut col_samples: HashMap<String, Vec<String>> = HashMap::new();

    // key -> DynRow
    let mut rows_by_key: BTreeMap<String, DynRow> = BTreeMap::new();

    // key 컬럼의 인덱스
    let key_idx = headers
        .iter()
        .position(|h| h == key_col)
        .unwrap_or(0);

    for rec in rdr.records() {
        let rec = rec?;
        let mut cells = HashMap::new();

        for (i, h) in headers.iter().enumerate() {
            let v = rec.get(i).unwrap_or("").to_string();
            cells.insert(h.to_string(), v.clone());

            col_samples.entry(h.to_string()).or_default();
            if col_samples[&h.to_string()].len() < 8 {
                col_samples.get_mut(&h.to_string()).unwrap().push(v);
            }
        }

        let key = rec.get(key_idx).unwrap_or("").to_string();
        rows_by_key.insert(
            key.clone(),
            DynRow {
                key,
                cells,
            },
        );
    }

    // 스키마 구성
    let columns: Vec<ColumnDef> = headers
        .iter()
        .map(|h: &str| {
            let samples: Vec<&str> = col_samples
                .get(h)
                .map(|v| v.iter().map(|s| s.as_str()).collect())
                .unwrap_or_else(|| Vec::new());
            let dtype = if samples.is_empty() {
                DataType::Text
            } else {
                infer_dtype(&samples)
            };
            ColumnDef {
                key: h.to_string(),
                label: h.to_string(),
                dtype,
            }
        })
        .collect();

    let schema = TableSchema {
        name: Path::new(path)
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string(),
        key_column: key_col.to_string(),
        columns,
    };

    Ok((schema, rows_by_key))
}

/// 원본을 다시 읽어, 같은 key의 컬럼들만 교체한 뒤 전체를 기록.
/// - 헤더/추가 컬럼/행 순서 보존
pub fn save_table(path: &str, key_col: &str, updates: &BTreeMap<String, DynRow>) -> Result<()> {
    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .from_path(path)
        .with_context(|| format!("open {}", path))?;
    let headers: StringRecord = rdr.headers()?.clone();

    // 키 인덱스
    let key_idx = headers
        .iter()
        .position(|h| h == key_col)
        .unwrap_or(0);

    // 메모리 버퍼에 먼저 작성 후 파일로 플러시
    let mut out = Vec::<u8>::new();
    {
        let mut w = WriterBuilder::new().has_headers(true).from_writer(&mut out);

        // 헤더 그대로 기록
        w.write_record(headers.iter())?;

        for rec in rdr.records() {
            let rec = rec?;

            // StringRecord를 Vec<String>으로 복사하여 수정
            let mut fields: Vec<String> = headers
                .iter()
                .enumerate()
                .map(|(i, _)| rec.get(i).unwrap_or("").to_string())
                .collect();

            let key_val = fields.get(key_idx).map(|s| s.as_str()).unwrap_or("");

            if let Some(newrow) = updates.get(key_val) {
                // 헤더 이름 기준으로 교체
                for (i, h) in headers.iter().enumerate() {
                    if let Some(v) = newrow.cells.get(h) {
                        fields[i] = v.clone();
                    }
                }
            }

            w.write_record(&fields)?;
        }

        w.flush()?;
    }

    File::create(path)?.write_all(&out)?;
    Ok(())
}
