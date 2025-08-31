use crate::{
    game::constants::{move_constants::Move, text_constants::MOVE_NAME_LENGTH},
    rom::ROM,
    save_state::string::PokeString,
};

impl Move {
    pub fn name(&self) -> PokeString<MOVE_NAME_LENGTH> {
        let mut skip = u8::from(*self) - 1;
        let mut bytes = Vec::with_capacity(MOVE_NAME_LENGTH);

        const START: usize = (0x72 * 0x4000) | (0x5f29 & 0x3fff);

        for &byte in &ROM[START..] {
            if byte == 0x50 {
                if skip > 0 {
                    skip -= 1;
                    continue;
                }

                break;
            }

            if skip == 0 {
                bytes.push(byte);
            }
        }

        assert!(bytes.len() <= MOVE_NAME_LENGTH);
        bytes.resize(MOVE_NAME_LENGTH, 0x50);

        PokeString::new(bytes.try_into().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::game::constants::move_constants::Move;

    #[test]
    fn test_move_names() {
        assert_eq!(format!("{}", Move::Pound.name()), "POUND");
        assert_eq!(format!("{}", Move::KarateChop.name()), "KARATE CHOP");
        assert_eq!(format!("{}", Move::Doubleslap.name()), "DOUBLESLAP");
        assert_eq!(format!("{}", Move::MegaPunch.name()), "MEGA PUNCH");
        assert_eq!(format!("{}", Move::PayDay.name()), "PAY DAY");
        assert_eq!(format!("{}", Move::FirePunch.name()), "FIRE PUNCH");
        assert_eq!(format!("{}", Move::IcePunch.name()), "ICE PUNCH");
        assert_eq!(format!("{}", Move::Thunderpunch.name()), "THUNDERPUNCH");
        assert_eq!(format!("{}", Move::Scratch.name()), "SCRATCH");
        assert_eq!(format!("{}", Move::Vicegrip.name()), "VICEGRIP");
        assert_eq!(format!("{}", Move::Guillotine.name()), "GUILLOTINE");
        assert_eq!(format!("{}", Move::RazorWind.name()), "RAZOR WIND");
        assert_eq!(format!("{}", Move::SwordsDance.name()), "SWORDS DANCE");
        assert_eq!(format!("{}", Move::Cut.name()), "CUT");
        assert_eq!(format!("{}", Move::Gust.name()), "GUST");
        assert_eq!(format!("{}", Move::WingAttack.name()), "WING ATTACK");
        assert_eq!(format!("{}", Move::Whirlwind.name()), "WHIRLWIND");
        assert_eq!(format!("{}", Move::Fly.name()), "FLY");
        assert_eq!(format!("{}", Move::Bind.name()), "BIND");
        assert_eq!(format!("{}", Move::Slam.name()), "SLAM");
        assert_eq!(format!("{}", Move::VineWhip.name()), "VINE WHIP");
        assert_eq!(format!("{}", Move::Stomp.name()), "STOMP");
        assert_eq!(format!("{}", Move::DoubleKick.name()), "DOUBLE KICK");
        assert_eq!(format!("{}", Move::MegaKick.name()), "MEGA KICK");
    }
}
