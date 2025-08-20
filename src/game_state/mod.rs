use crate::{
    game::{
        audio::music::Music,
        constants::{
            battle_constants::{self, BattleMode, BattleResult, BattleType, TypeEffectiveness},
            engine_flags::UnlockedUnowns,
            input_constants::JoypadButtons,
            item_constants::Item,
            map_constants::Map,
            move_constants::Move,
            pokemon_constants::PokemonSpecies,
            ram_constants::{MonType, SwarmFlags, TimeOfDay},
            text_constants::NAME_LENGTH,
        },
    },
    save_state::string::PokeString,
};

pub mod battle_mon;
pub mod box_mon;
pub mod moveset;
pub mod party_mon;

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

    pub fn set_player_move_struct_type(&mut self, value: u8) {
        self.data[0x0612] = value;
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

    pub fn prev_party_level(&self) -> u8 {
        self.data[0x1002]
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

    /// index of mon's party location (0-5)
    pub fn cur_party_mon(&self) -> u8 {
        self.data[0x1109]
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

    pub fn evolution_old_species(&self) -> Option<PokemonSpecies> {
        match self.data[0x11ea] {
            0 => None,
            n => Some(n.into()),
        }
    }

    pub fn skip_moves_before_level_up(&self) -> u8 {
        self.data[0x11ea]
    }

    pub fn enemy_effectiveness_vs_player_mons(&self, n: u8) -> bool {
        self.data[0x11ea] & (1 << n) != 0
    }

    pub fn set_enemy_effectiveness_vs_player_mons(&mut self, n: u8, value: bool) {
        if value {
            self.data[0x11ea] |= 1 << n;
        } else {
            self.data[0x11ea] &= !(1 << n);
        }
    }

    pub fn set_player_effectiveness_vs_enemy_mons(&mut self, n: u8, value: bool) {
        if value {
            self.data[0x11eb] |= 1 << n;
        } else {
            self.data[0x11eb] &= !(1 << n);
        }
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

    pub fn set_temp_wild_mon_species(&mut self, value: Option<PokemonSpecies>) {
        self.data[0x122e] = value.map_or(0, Into::into);
    }

    pub fn battle_type(&self) -> BattleType {
        self.data[0x1230].into()
    }

    pub fn set_battle_type(&mut self, value: BattleType) {
        self.data[0x1230] = value.into();
    }

    pub fn set_morn_encounter_rate(&mut self, value: u8) {
        self.data[0x125a] = value;
    }

    pub fn set_day_encounter_rate(&mut self, value: u8) {
        self.data[0x125b] = value;
    }

    pub fn set_nite_encounter_rate(&mut self, value: u8) {
        self.data[0x125c] = value;
    }

    pub fn set_water_encounter_rate(&mut self, value: u8) {
        self.data[0x125d] = value;
    }

    pub fn set_putative_tm_hm_move(&mut self, value: Move) {
        self.data[0x1262] = value.into();
    }

    pub fn set_chosen_cable_club_room(&mut self, value: u8) {
        self.data[0x1265] = value;
    }

    pub fn named_object_index(&self) -> u8 {
        self.data[0x1265]
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

    pub fn type_matchup(&self) -> TypeEffectiveness {
        self.data[0x1265].into()
    }

    pub fn time_of_day(&self) -> TimeOfDay {
        self.data[0x1269].into()
    }

    pub fn ot_party_species(&self) -> Vec<PokemonSpecies> {
        let count = self.data[0x1280] as usize;

        self.data[0x1281..(0x1281 + count)]
            .iter()
            .take_while(|&n| *n != 0)
            .map(|&n| n.into())
            .collect::<Vec<_>>()
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

    pub fn swarm_flags(&self) -> SwarmFlags {
        SwarmFlags::from_bits_retain(self.data[0x1c20])
    }

    pub fn yanma_map(&self) -> Map {
        (self.data[0x1c5a], self.data[0x1c5b]).into()
    }

    pub fn park_balls_remaining(&self) -> u8 {
        self.data[0x1c79]
    }

    pub fn set_park_balls_remaining(&mut self, value: u8) {
        self.data[0x1c79] = value;
    }

    pub fn map(&self) -> Map {
        (self.data[0x1cb5], self.data[0x1cb6]).into()
    }

    pub fn party_count(&self) -> u8 {
        self.data[0x1cd7]
    }

    pub fn party_mon(&self, index: usize) -> party_mon::PartyMon<'_> {
        assert!(index < 6);
        let offset = 0x1cdf + (index * party_mon::PARTYMON_STRUCT_LENGTH);
        party_mon::PartyMon::new(&self.data[offset..])
    }

    pub fn party_mon_mut(&mut self, index: usize) -> party_mon::PartyMonMut<'_> {
        assert!(index < 6);
        let offset = 0x1cdf + (index * party_mon::PARTYMON_STRUCT_LENGTH);
        party_mon::PartyMonMut::new(&mut self.data[offset..])
    }

    pub fn unlocked_unowns(&self) -> UnlockedUnowns {
        UnlockedUnowns::from_bits_retain(self.data[0x1ef3])
    }

    pub fn breed_mon_1(&self) -> Option<box_mon::BoxMon<'_>> {
        match self.data[0x1f0c] {
            0 => None,
            _ => Some(box_mon::BoxMon::new(&self.data[0x1f0c..])),
        }
    }

    pub fn breed_mother_or_non_ditto(&self) -> bool {
        // z: yes, nz: no
        self.data[0x1f2e] == 0
    }

    pub fn breed_mon_2(&self) -> Option<box_mon::BoxMon<'_>> {
        match self.data[0x1f45] {
            0 => None,
            _ => Some(box_mon::BoxMon::new(&self.data[0x1f45..])),
        }
    }

    pub fn egg_mon(&self) -> Option<box_mon::BoxMon<'_>> {
        match self.data[0x1f7b] {
            0 => None,
            _ => Some(box_mon::BoxMon::new(&self.data[0x1f7b..])),
        }
    }

    pub fn dunsparce_map(&self) -> Map {
        (self.data[0x1fcc], self.data[0x1fcd]).into()
    }

    pub fn roam_mon_1_species(&self) -> Option<PokemonSpecies> {
        match self.data[0x1fcf] {
            0 => None,
            n => Some(n.into()),
        }
    }

    pub fn roam_mon_1_level(&self) -> u8 {
        self.data[0x1fd0]
    }

    pub fn roam_mon_1_map(&self) -> Map {
        (self.data[0x1fd1], self.data[0x1fd2]).into()
    }

    pub fn roam_mon_2_species(&self) -> Option<PokemonSpecies> {
        match self.data[0x1fd6] {
            0 => None,
            n => Some(n.into()),
        }
    }

    pub fn roam_mon_2_level(&self) -> u8 {
        self.data[0x1fd7]
    }

    pub fn roam_mon_2_map(&self) -> Map {
        (self.data[0x1fd8], self.data[0x1fd9]).into()
    }

    pub fn roam_mon_3_species(&self) -> Option<PokemonSpecies> {
        match self.data[0x1fdd] {
            0 => None,
            n => Some(n.into()),
        }
    }

    pub fn roam_mon_3_level(&self) -> u8 {
        self.data[0x1fde]
    }

    pub fn roam_mon_3_map(&self) -> Map {
        (self.data[0x1fdf], self.data[0x1fe0]).into()
    }
}
