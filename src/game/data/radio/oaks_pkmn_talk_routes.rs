use crate::rom::ROM;

const OAKS_PKMNTALK_ROUTES_ADDR: usize = (0x2e * 0x4000) | (0x47f2 & 0x3fff);
const OAKS_PKMNTALK_ROUTES_COUNT: usize = 15;

/// Oak's Pokémon Talk will list wild Pokémon on these maps.
pub const OAKS_PKMN_TALK_ROUTES: [(u8, u8); OAKS_PKMNTALK_ROUTES_COUNT] = {
    let mut routes = [(0u8, 0u8); OAKS_PKMNTALK_ROUTES_COUNT];
    let mut i = 0;

    while i < OAKS_PKMNTALK_ROUTES_COUNT {
        let base = OAKS_PKMNTALK_ROUTES_ADDR + i * 2;
        routes[i] = (ROM[base], ROM[base + 1]);
        i += 1;
    }

    routes
};
