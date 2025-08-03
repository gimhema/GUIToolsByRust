use std::collections::HashMap;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawDataCharacterInfo {
    pub unique: u32,
    pub name: String,
}

impl RawDataCharacterInfo {
    pub fn new_zero() -> Self {
        RawDataCharacterInfo {
            unique: 0,
            name: String::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize,  Clone)]
pub struct RawDataCharacterStatusInfo {
    pub unique: u32,
    pub health: u32,
    pub mana: u32,
    pub stamina: u32,
}

impl RawDataCharacterStatusInfo {
    pub fn new_zero() -> Self {
        RawDataCharacterStatusInfo {
            unique: 0,
            health: 0,
            mana: 0,
            stamina: 0,
        }
    }
}


#[derive(Debug, Deserialize, Serialize,  Clone)]
pub struct RawDataCharacterAttackInfo {
    pub unique: u32,
    pub attack_power: u32,
    pub attack_speed: f32,
}

impl RawDataCharacterAttackInfo {
    pub fn new_zero() -> Self {
        RawDataCharacterAttackInfo {
            unique: 0,
            attack_power: 0,
            attack_speed: 0.0,
        }
    }
}

