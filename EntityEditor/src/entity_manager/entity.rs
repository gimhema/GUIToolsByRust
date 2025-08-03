use serde::Deserialize;
use super::raw_data::*;



pub struct CharacterBase {
    unique: u32,
    character_info : RawDataCharacterInfo,
    character_status_info: RawDataCharacterStatusInfo,
    character_attack_info: RawDataCharacterAttackInfo,
}