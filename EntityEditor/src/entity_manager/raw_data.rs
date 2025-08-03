use std::collections::HashMap;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
struct CharacterInfo {
    unique: u32,
    name: String,
}

#[derive(Debug, Deserialize)]
struct CharacterStatusInfo {
    unique: u32,
    health: u32,
    mana: u32,
    stamina: u32,
}


#[derive(Debug, Deserialize)]
struct CharacterAttackInfo {
    unique: u32,
    attack_power: u32,
    attack_speed: f32,
}


