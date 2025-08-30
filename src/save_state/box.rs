use crate::{
    game::constants::pokemon_data_constants::MONS_PER_BOX,
    game_state::{
        box_mon::{BoxMonMut, BoxMonRef},
        mon_list::{MonList, MonListMut},
    },
};

pub type Box<'a> = MonList<'a, BoxMonRef<'a>, MONS_PER_BOX>;
pub type BoxMut<'a> = MonListMut<'a, BoxMonRef<'a>, BoxMonMut<'a>, MONS_PER_BOX>;

#[cfg(test)]
mod test {
    use crate::{
        game::constants::pokemon_constants::PokemonSpecies,
        game_state::{
            battle_mon::{BattleMon, BattleMonMut},
            box_mon::BoxMonOwned,
            mon_list::MonListEntry,
        },
        save_state::string::PokeString,
    };

    use super::*;

    #[test]
    fn test_push_front() {
        let mut a_vec = vec![0; 100];
        let mut b_vec = vec![0; 100];
        let mut c_vec = vec![0; 100];

        let mut a = BattleMonMut::new(&mut a_vec);

        a.set_species(PokemonSpecies::Eevee);
        a.set_level(15);

        let mut b = BattleMonMut::new(&mut b_vec);

        b.set_species(PokemonSpecies::Grimer);
        b.set_level(20);

        let mut c = BattleMonMut::new(&mut c_vec);

        c.set_species(PokemonSpecies::Phanpy);
        c.set_level(18);

        let a_ot = PokeString::new([
            0x80, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50,
        ]);
        let b_ot = PokeString::new([
            0x81, 0x81, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50,
        ]);
        let c_ot = PokeString::new([
            0x82, 0x82, 0x82, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50,
        ]);

        let a_name = PokeString::new([
            0xa0, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50,
        ]);
        let b_name = PokeString::new([
            0xa1, 0xa1, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50,
        ]);
        let c_name = PokeString::new([
            0xa2, 0xa2, 0xa2, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50,
        ]);

        let a = BoxMonOwned::from_battle_mon(BattleMon::new(&a_vec), 10101);
        let b = BoxMonOwned::from_battle_mon(BattleMon::new(&b_vec), 20202);
        let c = BoxMonOwned::from_battle_mon(BattleMon::new(&c_vec), 30303);

        let mut box_vec = vec![0; 1200];
        let mut r#box = BoxMut::new(&mut box_vec);

        assert_eq!(r#box.len(), 0);
        assert_eq!(r#box.get(0), None);

        r#box.push_front(MonListEntry::Mon(c.as_ref(), c_ot.clone(), c_name.clone()));

        assert_eq!(r#box.len(), 1);
        assert_eq!(
            r#box.get(0),
            Some(MonListEntry::Mon(c.as_ref(), c_ot.clone(), c_name.clone()))
        );
        assert_eq!(r#box.get(1), None);

        r#box.push_front(MonListEntry::Mon(b.as_ref(), b_ot.clone(), b_name.clone()));

        assert_eq!(r#box.len(), 2);
        assert_eq!(
            r#box.get(0),
            Some(MonListEntry::Mon(b.as_ref(), b_ot.clone(), b_name.clone()))
        );
        assert_eq!(
            r#box.get(1),
            Some(MonListEntry::Mon(c.as_ref(), c_ot.clone(), c_name.clone()))
        );
        assert_eq!(r#box.get(2), None);

        r#box.push_front(MonListEntry::Mon(a.as_ref(), a_ot.clone(), a_name.clone()));

        assert_eq!(r#box.len(), 3);
        assert_eq!(
            r#box.get(0),
            Some(MonListEntry::Mon(a.as_ref(), a_ot.clone(), a_name.clone()))
        );
        assert_eq!(
            r#box.get(1),
            Some(MonListEntry::Mon(b.as_ref(), b_ot.clone(), b_name.clone()))
        );
        assert_eq!(
            r#box.get(2),
            Some(MonListEntry::Mon(c.as_ref(), c_ot.clone(), c_name.clone()))
        );
        assert_eq!(r#box.get(3), None);
    }
}
