use serde::Deserialize;



pub struct CharacterBase {
    unique: u32,
    name: String,
    health: u32,
    mana: u32,
    stamina: u32,
    attack_power: u32,
    attack_speed: f32,
}