use super::raw_data::*;
use csv::{ReaderBuilder, StringRecord};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Default, Clone)]
pub struct FieldLabels {
    pub info_name: String,              // ex) "Name"
    pub status_health: String,          // ex) "Health"
    pub status_mana: String,            // ex) "Mana"
    pub status_stamina: String,         // 파일에 없으면 기본 "Stamina"
    pub attack_attack_power: String,    // ex) "AttackPower"
    pub attack_defense_power: String,   // ex) "DefensePower"
}

pub trait EntityBox {
    fn load_data(&mut self) -> Result<(), Box<dyn Error>>;
    fn save_data(&mut self) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug, Clone)]
pub struct CharacterEntity {
    pub unique: u32,
    pub character_info: RawDataCharacterInfo,
    pub character_status_info: RawDataCharacterStatusInfo,
    pub character_attack_info: RawDataCharacterAttackInfo,
}

pub struct CharacterEntityContainer {
    entities: HashMap<u32, CharacterEntity>,
    character_info_data_path: String,
    character_status_info_data_path: String,
    character_attack_info_data_path: String,

    // 상태 CSV에서 수동으로 키 컬럼을 지정하고 싶을 때 사용 (예: "Unique")
    status_key_column_override: Option<String>,

    pub labels: FieldLabels, // ← 추가
}

impl CharacterEntityContainer {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            character_info_data_path: String::new(),
            character_status_info_data_path: String::new(),
            character_attack_info_data_path: String::new(),
            status_key_column_override: None,
            labels: FieldLabels::default(),
        }
    }

    fn find_header_case_insensitive(headers: &csv::StringRecord, target: &str) -> Option<String> {
    headers.iter().find(|h| h.eq_ignore_ascii_case(target)).map(|s| s.to_string())
    }

    pub fn set_paths(
        &mut self,
        info: impl Into<String>,
        status: impl Into<String>,
        attack: impl Into<String>,
    ) {
        self.character_info_data_path = info.into();
        self.character_status_info_data_path = status.into();
        self.character_attack_info_data_path = attack.into();
    }

    /// 상태 CSV의 키 컬럼명을 수동 지정 (예: "Unique" / "CharacterUnique")
    pub fn set_status_key_column_override(&mut self, col: impl Into<String>) {
        self.status_key_column_override = Some(col.into());
    }

    pub fn add_entity(&mut self, entity: CharacterEntity) {
        self.entities.insert(entity.unique, entity);
    }

    pub fn get_entity(&self, unique: u32) -> Option<&CharacterEntity> {
        self.entities.get(&unique)
    }

    pub fn get_entity_mut(&mut self, unique: u32) -> Option<&mut CharacterEntity> {
        self.entities.get_mut(&unique)
    }

    pub fn update_character_entity_status(&mut self, unique: u32, status_info: RawDataCharacterStatusInfo) {
        if let Some(entity) = self.entities.get_mut(&unique) {
            entity.character_status_info = status_info;
        }
    }

    pub fn update_character_entity_attack_info(&mut self, unique: u32, attack_info: RawDataCharacterAttackInfo) {
        if let Some(entity) = self.entities.get_mut(&unique) {
            entity.character_attack_info = attack_info;
        }
    }

    pub fn remove_entity(&mut self, unique: u32) {
        self.entities.remove(&unique);
    }

    fn load_info_csv(&mut self) -> Result<(), Box<dyn Error>> {
        let file = File::open(&self.character_info_data_path)?;
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

         let headers = rdr.headers()?.clone();
          self.labels.info_name =
              Self::find_header_case_insensitive(&headers, "Name").unwrap_or_else(|| "Name".into());
  

        for result in rdr.deserialize::<RawDataCharacterInfo>() {
            let record = result?;
            let unique = record.unique;
            let entity = CharacterEntity {
                unique,
                character_info: record,
                character_status_info: RawDataCharacterStatusInfo::new_zero(),
                character_attack_info: RawDataCharacterAttackInfo::new_zero(),
            };
            self.add_entity(entity);
        }
        Ok(())
    }

    fn load_status_csv_auto(&mut self) -> Result<(), Box<dyn Error>> {
        // alias 덕분에 Unique/CharacterUnique 모두 허용
        let file = File::open(&self.character_status_info_data_path)?;
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

        let headers = rdr.headers()?.clone();
        self.labels.status_health =
            Self::find_header_case_insensitive(&headers, "Health").unwrap_or_else(|| "Health".into());
        self.labels.status_mana =
            Self::find_header_case_insensitive(&headers, "Mana").unwrap_or_else(|| "Mana".into());
        if self.labels.status_stamina.is_empty() {
            self.labels.status_stamina = "Stamina".into();
        }


        for result in rdr.deserialize::<RawDataCharacterStatusInfo>() {
            let status = result?;
            let unique = status.unique;
            self.update_character_entity_status(unique, status);
        }
        Ok(())
    }

    /// 사용자가 키 컬럼명을 직접 지정해 로드 (진짜 "수동 지정")
    fn load_status_with_key_column(&mut self, key_col: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(&self.character_status_info_data_path)?;
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

        let headers = rdr.headers()?.clone();
        self.labels.status_health =
            Self::find_header_case_insensitive(&headers, "Health").unwrap_or_else(|| "Health".into());
        self.labels.status_mana =
            Self::find_header_case_insensitive(&headers, "Mana").unwrap_or_else(|| "Mana".into());
        if self.labels.status_stamina.is_empty() {
            self.labels.status_stamina = "Stamina".into();
        }

        let headers = rdr.headers()?.clone();
        let key_idx = headers
            .iter()
            .position(|h| h.eq_ignore_ascii_case(key_col))
            .ok_or_else(|| format!("키 컬럼 '{}'을(를) 찾을 수 없습니다.", key_col))?;

        for result in rdr.records() {
            let rec: StringRecord = result?;
            let key_str = rec.get(key_idx).ok_or("키 컬럼 인덱스 범위 초과")?;
            let unique: u32 = key_str.trim().parse()?;

            // 나머지 컬럼은 안전하게 파싱
            let health: u32 = rec
                .get(headers.iter().position(|h| h.eq_ignore_ascii_case("Health")).ok_or("Health 헤더 없음")?)
                .unwrap()
                .trim()
                .parse()?;
            let mana: u32 = rec
                .get(headers.iter().position(|h| h.eq_ignore_ascii_case("Mana")).ok_or("Mana 헤더 없음")?)
                .unwrap()
                .trim()
                .parse()?;

            let status = RawDataCharacterStatusInfo {
                unique,
                health,
                mana,
                stamina: 0,
            };
            self.update_character_entity_status(unique, status);
        }
        Ok(())
    }

    fn load_attack_csv(&mut self) -> Result<(), Box<dyn Error>> {
        // ".txt" 확장자지만 CSV 포맷이므로 그대로 파싱
        let file = File::open(&self.character_attack_info_data_path)?;
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

        let headers = rdr.headers()?.clone();
        self.labels.attack_attack_power =
            Self::find_header_case_insensitive(&headers, "AttackPower").unwrap_or_else(|| "AttackPower".into());
        self.labels.attack_defense_power =
            Self::find_header_case_insensitive(&headers, "DefensePower").unwrap_or_else(|| "DefensePower".into());


        for result in rdr.deserialize::<RawDataCharacterAttackInfo>() {
            let atk = result?;
            self.update_character_entity_attack_info(atk.unique, atk);
        }
        Ok(())
    }
}

impl EntityBox for CharacterEntityContainer {
fn load_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // 1) info 먼저
        self.load_info_csv()?;

        // 2) status: 수동 지정이 있으면 그걸로, 없으면 자동(alias)
        if let Some(key) = self.status_key_column_override.clone() {
            // ↑ clone으로 필드와의 불변 대여를 끊음 (owned String)
            self.load_status_with_key_column(&key)?;
        } else {
            self.load_status_csv_auto()?;
        }

        // 3) attack
        self.load_attack_csv()?;

        Ok(())
    }

    fn save_data(&mut self) -> Result<(), Box<dyn Error>> {
        // 기존 로직 유지 + 필드명 변경 반영
        // Info 저장
        {
            let path = &self.character_info_data_path;
            let mut rdr = csv::Reader::from_path(path)?;
            let mut out: Vec<RawDataCharacterInfo> = vec![];

            for result in rdr.deserialize() {
                let mut rec: RawDataCharacterInfo = result?;
                if let Some(ent) = self.get_entity(rec.unique) {
                    rec.unique = ent.character_info.unique;
                    rec.name = ent.character_info.name.clone();
                }
                out.push(rec);
            }
            let mut w = csv::Writer::from_path(path)?;
            for r in out { w.serialize(r)?; }
            w.flush()?;
        }

        // Status 저장
        {
            let path = &self.character_status_info_data_path;
            let mut rdr = csv::Reader::from_path(path)?;
            let mut out: Vec<RawDataCharacterStatusInfo> = vec![];

            for result in rdr.deserialize() {
                let mut rec: RawDataCharacterStatusInfo = result?;
                if let Some(ent) = self.get_entity(rec.unique) {
                    rec.unique = ent.character_status_info.unique;
                    rec.health = ent.character_status_info.health;
                    rec.mana = ent.character_status_info.mana;
                    rec.stamina = ent.character_status_info.stamina;
                }
                out.push(rec);
            }
            let mut w = csv::Writer::from_path(path)?;
            for r in out { w.serialize(r)?; }
            w.flush()?;
        }

        // Attack 저장
        {
            let path = &self.character_attack_info_data_path;
            let mut rdr = csv::Reader::from_path(path)?;
            let mut out: Vec<RawDataCharacterAttackInfo> = vec![];

            for result in rdr.deserialize() {
                let mut rec: RawDataCharacterAttackInfo = result?;
                if let Some(ent) = self.get_entity(rec.unique) {
                    rec.unique = ent.character_attack_info.unique;
                    rec.attack_power = ent.character_attack_info.attack_power;
                    rec.defense_power = ent.character_attack_info.defense_power;
                }
                out.push(rec);
            }
            let mut w = csv::Writer::from_path(path)?;
            for r in out { w.serialize(r)?; }
            w.flush()?;
        }

        Ok(())
    }
}

