use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawDataCharacterInfo {
    #[serde(rename = "CharacterUnique")]
    pub unique: u32,
    #[serde(rename = "Name")]
    pub name: String,
}

impl RawDataCharacterInfo {
    pub fn new_zero() -> Self {
        Self { unique: 0, name: String::new() }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawDataCharacterStatusInfo {
    // 자동 허용: "Unique" 또는 "CharacterUnique" 둘 다 매칭
    #[serde(alias = "Unique", alias = "CharacterUnique")]
    pub unique: u32,
    #[serde(rename = "Health")]
    pub health: u32,
    #[serde(rename = "Mana")]
    pub mana: u32,
    // 파일엔 없으니 기본값 0
    #[serde(default)]
    pub stamina: u32,
}

impl RawDataCharacterStatusInfo {
    pub fn new_zero() -> Self {
        Self { unique: 0, health: 0, mana: 0, stamina: 0 }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawDataCharacterAttackInfo {
    #[serde(rename = "CharacterUnique")]
    pub unique: u32,
    #[serde(rename = "AttackPower")]
    pub attack_power: u32,
    #[serde(rename = "DefensePower")]
    pub defense_power: u32,
}

impl RawDataCharacterAttackInfo {
    pub fn new_zero() -> Self {
        Self { unique: 0, attack_power: 0, defense_power: 0 }
    }
}
