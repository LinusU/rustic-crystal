use crate::{
    game::{
        audio::music::Music,
        constants::{
            battle_constants::{self, BattleMode, BattleResult, BattleType},
            input_constants::JoypadButtons,
            item_constants::Item,
            pokemon_constants::PokemonSpecies,
            ram_constants::{MonType, TimeOfDay},
            text_constants::NAME_LENGTH,
        },
    },
    save_state::string::PokeString,
};

pub mod battle_mon;

const WRAM_SIZE: usize = 0x8000;

fn fill_random(slice: &mut [u8], start: u32) {
    // Simple LCG to generate (non-cryptographic) random values
    // Each distinct invocation should use a different start value
    const A: u32 = 1103515245;
    const C: u32 = 12345;

    let mut x = start;
    for v in slice.iter_mut() {
        x = x.wrapping_mul(A).wrapping_add(C);
        *v = ((x >> 23) & 0xFF) as u8;
    }
}

pub struct GameState {
    data: [u8; WRAM_SIZE],
}

impl GameState {
    pub fn new() -> GameState {
        let mut data = [0; WRAM_SIZE];

        fill_random(&mut data, 42);

        GameState { data }
    }

    pub fn byte(&self, addr: usize) -> u8 {
        self.data[addr]
    }

    pub fn set_byte(&mut self, addr: usize, value: u8) {
        self.data[addr] = value;
    }
}

impl GameState {
    pub fn cry_pitch(&self) -> i16 {
        i16::from_le_bytes([self.data[0x02b0], self.data[0x02b1]])
    }

    pub fn cry_length(&self) -> u16 {
        u16::from_le_bytes([self.data[0x02b2], self.data[0x02b3]])
    }

    pub fn set_map_music(&mut self, value: Option<Music>) {
        self.data[0x02c0] = value.map_or(0, |value| value as u8);
    }

    pub fn disable_text_acceleration(&self) -> bool {
        self.data[0x02d7] != 0
    }

    pub fn set_disable_text_acceleration(&mut self, value: bool) {
        self.data[0x02d7] = if value { 1 } else { 0 };
    }

    pub fn set_script_var(&mut self, value: u8) {
        self.data[0x02dd] = value;
    }

    pub fn battle_mon(&self) -> battle_mon::BattleMon<'_> {
        battle_mon::BattleMon::new(&self.data[0x062c..])
    }

    pub fn wild_mon(&self) -> Option<PokemonSpecies> {
        match self.data[0x064e] {
            0 => None,
            n => Some(n.into()),
        }
    }

    pub fn set_wild_mon(&mut self, value: Option<PokemonSpecies>) {
        self.data[0x064e] = value.map_or(0, Into::into);
    }

    pub fn enemy_sub_status_is_transformed(&self) -> bool {
        (self.data[0x0671] & (1 << battle_constants::SUBSTATUS_TRANSFORMED)) != 0
    }

    pub fn set_enemy_sub_status_is_transformed(&mut self, value: bool) {
        if value {
            self.data[0x0671] |= 1 << battle_constants::SUBSTATUS_TRANSFORMED;
        } else {
            self.data[0x0671] &= !(1 << battle_constants::SUBSTATUS_TRANSFORMED);
        }
    }

    pub fn set_battle_anim_param(&mut self, value: u8) {
        self.data[0x0689] = value;
    }

    pub fn set_enemy_backup_dvs(&mut self, value: u16) {
        [self.data[0x06f2], self.data[0x06f3]] = value.to_be_bytes();
    }

    pub fn player_link_action(&self) -> u8 {
        self.data[0x0f56]
    }

    pub fn set_player_link_action(&mut self, value: u8) {
        self.data[0x0f56] = value;
    }

    pub fn link_timeout_frames(&self) -> u16 {
        u16::from_le_bytes([self.data[0x0f5b], self.data[0x0f5c]])
    }

    pub fn set_link_timeout_frames(&mut self, value: u16) {
        [self.data[0x0f5b], self.data[0x0f5c]] = value.to_le_bytes();
    }

    pub fn set_mon_type(&mut self, value: MonType) {
        self.data[0x0f5f] = value.into();
    }

    pub fn set_cur_species(&mut self, value: Option<PokemonSpecies>) {
        self.data[0x0f60] = value.map_or(0, Into::into);
    }

    pub fn menu_joypad(&self) -> JoypadButtons {
        JoypadButtons::from_bits(self.data[0x0f73]).unwrap()
    }

    pub fn menu_selection(&self) -> u8 {
        self.data[0x0f74]
    }

    pub fn which_index_set(&self) -> u8 {
        self.data[0x0f76]
    }

    pub fn set_which_index_set(&mut self, value: u8) {
        self.data[0x0f76] = value;
    }

    pub fn menu_cursor_y(&self) -> u8 {
        self.data[0x0fa9]
    }

    pub fn game_timer_paused(&self) -> bool {
        (self.data[0x0fbc] & 1) != 0
    }

    pub fn set_game_timer_paused(&mut self, value: bool) {
        if value {
            self.data[0x0fbc] |= 1;
        } else {
            self.data[0x0fbc] &= !1;
        }
    }

    pub fn set_fx_anim_id(&mut self, value: u16) {
        self.data[0x0fc2] = (value & 0xff) as u8;
        self.data[0x0fc3] = (value >> 8) as u8;
    }

    pub fn set_num_hits(&mut self, value: u8) {
        self.data[0x0fca] = value;
    }

    pub fn set_no_text_scroll(&mut self, value: bool) {
        if value {
            self.data[0x0fcc] |= 1 << 4;
        } else {
            self.data[0x0fcc] &= !(1 << 4);
        }
    }

    pub fn save_file_exists(&self) -> bool {
        self.data[0x0fcd] != 0
    }

    pub fn set_save_file_exists(&mut self, value: bool) {
        self.data[0x0fcd] = if value { 1 } else { 0 };
    }

    pub fn battle_result(&self) -> BattleResult {
        BattleResult::from_bits(self.data[0x10ee]).unwrap()
    }

    pub fn set_battle_result(&mut self, value: BattleResult) {
        self.data[0x10ee] = value.bits();
    }

    pub fn cur_item(&self) -> Item {
        self.data[0x1106].into()
    }

    pub fn cur_party_species(&self) -> Option<PokemonSpecies> {
        match self.data[0x1108] {
            0 => None,
            n => Some(n.into()),
        }
    }

    pub fn set_cur_party_species(&mut self, value: Option<PokemonSpecies>) {
        self.data[0x1108] = value.map_or(0, Into::into);
    }

    pub fn set_cur_party_mon(&mut self, value: u8) {
        self.data[0x1109] = value;
    }

    pub fn set_item_quantity_change(&mut self, value: u8) {
        self.data[0x110c] = value;
    }

    pub fn cur_party_level(&self) -> u8 {
        self.data[0x1143]
    }

    pub fn set_cur_party_level(&mut self, value: u8) {
        self.data[0x1143] = value;
    }

    pub fn set_final_catch_rate(&mut self, value: u8) {
        self.data[0x11ea] = value;
    }

    pub fn thrown_ball_wobble_count(&self) -> u8 {
        self.data[0x11eb]
    }

    pub fn set_thrown_ball_wobble_count(&mut self, value: u8) {
        self.data[0x11eb] = value;
    }

    pub fn temp_enemy_mon_species(&self) -> Option<PokemonSpecies> {
        match self.data[0x1204] {
            0 => None,
            n => Some(n.into()),
        }
    }

    pub fn set_temp_enemy_mon_species(&mut self, value: Option<PokemonSpecies>) {
        self.data[0x1204] = value.map_or(0, Into::into);
    }

    pub fn enemy_mon(&self) -> battle_mon::BattleMon<'_> {
        battle_mon::BattleMon::new(&self.data[0x1206..])
    }

    pub fn enemy_mon_mut(&mut self) -> battle_mon::BattleMonMut<'_> {
        battle_mon::BattleMonMut::new(&mut self.data[0x1206..])
    }

    pub fn enemy_mon_catch_rate(&self) -> u8 {
        self.data[0x122b]
    }

    pub fn battle_mode(&self) -> Option<BattleMode> {
        match self.data[0x122d] {
            0 => None,
            n => Some(n.into()),
        }
    }

    pub fn battle_type(&self) -> BattleType {
        self.data[0x1230].into()
    }

    pub fn set_chosen_cable_club_room(&mut self, value: u8) {
        self.data[0x1265] = value;
    }

    pub fn set_named_object_index(&mut self, value: u8) {
        self.data[0x1265] = value;
    }

    pub fn temp_species(&self) -> Option<PokemonSpecies> {
        match self.data[0x1265] {
            0 => None,
            n => Some(n.into()),
        }
    }

    pub fn set_temp_species(&mut self, value: Option<PokemonSpecies>) {
        self.data[0x1265] = value.map_or(0, Into::into);
    }

    pub fn time_of_day(&self) -> TimeOfDay {
        self.data[0x1269].into()
    }

    pub fn player_id(&self) -> u16 {
        u16::from_be_bytes([self.data[0x147b], self.data[0x147c]])
    }

    pub fn player_name(&self) -> PokeString {
        PokeString::from_bytes(&self.data[0x147d..], NAME_LENGTH)
    }

    pub fn cur_box(&self) -> u8 {
        self.data[0x1b72]
    }

    pub fn set_cur_box(&mut self, value: u8) {
        self.data[0x1b72] = value;
    }

    pub fn park_balls_remaining(&self) -> u8 {
        self.data[0x1c79]
    }

    pub fn set_park_balls_remaining(&mut self, value: u8) {
        self.data[0x1c79] = value;
    }

    pub fn party_count(&self) -> u8 {
        self.data[0x1cd7]
    }
}
