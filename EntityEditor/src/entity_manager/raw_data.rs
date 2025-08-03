use std::collections::HashMap;
use serde::Deserialize;


#[derive(Debug, Deserialize, Clone)]
pub struct RawDataCharacterInfo {
    unique: u32,
    name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RawDataCharacterStatusInfo {
    unique: u32,
    health: u32,
    mana: u32,
    stamina: u32,
}


#[derive(Debug, Deserialize, Clone)]
pub struct RawDataCharacterAttackInfo {
    unique: u32,
    attack_power: u32,
    attack_speed: f32,
}


