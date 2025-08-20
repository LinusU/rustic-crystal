use crate::game::constants::{move_constants::Move, pokemon_constants::PokemonSpecies};

macro_rules! define_egg_moves {
    (
        $(
            $species:ident => [ $( $mv:ident ),* $(,)? ]
        ),* $(,)?
    ) => {
        impl PokemonSpecies {
            /// Returns the egg moves for this species (empty slice if none).
            pub fn egg_moves(self) -> &'static [Move] {
                match self {
                    $(
                        PokemonSpecies::$species => &[
                            $( Move::$mv ),*
                        ],
                    )*
                    _ => &[],
                }
            }
        }
    };
}

define_egg_moves! {
    Bulbasaur => [LightScreen, SkullBash, Safeguard, RazorWind, PetalDance],
    Charmander => [BellyDrum, Ancientpower, RockSlide, Bite, Outrage, BeatUp],
    Squirtle => [MirrorCoat, Haze, Mist, Confusion, Foresight, Flail],
    Pidgey => [Pursuit, FaintAttack, Foresight],
    Rattata => [Screech, FlameWheel, FurySwipes, Bite, Counter, Reversal],
    Spearow => [FaintAttack, FalseSwipe, ScaryFace, QuickAttack, TriAttack],
    Ekans => [Pursuit, Slam, Spite, BeatUp, Crunch],
    Sandshrew => [Flail, Safeguard, Counter, RapidSpin, MetalClaw],
    NidoranF => [Supersonic, Disable, TakeDown, FocusEnergy, Charm, Counter, BeatUp],
    NidoranM => [Supersonic, Disable, TakeDown, Confusion, Amnesia, Counter, BeatUp],
    Vulpix => [FaintAttack, Hypnosis, Flail, Spite, Disable],
    Zubat => [QuickAttack, Pursuit, FaintAttack, Gust, Whirlwind],
    Oddish => [SwordsDance, RazorLeaf, Flail, Synthesis],
    Paras => [FalseSwipe, Screech, Counter, Psybeam, Flail, LightScreen, Pursuit],
    Venonat => [BatonPass, Screech, GigaDrain],
    Diglett => [FaintAttack, Screech, Ancientpower, Pursuit, BeatUp],
    Meowth => [Spite, Charm, Hypnosis, Amnesia],
    Psyduck => [IceBeam, Hypnosis, Psybeam, Foresight, LightScreen, FutureSight, PsychicM, CrossChop],
    Mankey => [RockSlide, Foresight, Meditate, Counter, Reversal, BeatUp],
    Growlithe => [BodySlam, Safeguard, Crunch, Thrash, FireSpin],
    Poliwag => [Mist, Splash, Bubblebeam, Haze, MindReader],
    Abra => [LightScreen, Encore, Barrier],
    Machop => [LightScreen, Meditate, RollingKick, Encore],
    Bellsprout => [SwordsDance, Encore, Reflect, Synthesis, LeechLife],
    Tentacool => [AuroraBeam, MirrorCoat, RapidSpin, Haze, Safeguard],
    Geodude => [MegaPunch, RockSlide],
    Ponyta => [FlameWheel, Thrash, DoubleKick, Hypnosis, Charm, QuickAttack],
    Slowpoke => [Safeguard, BellyDrum, FutureSight, Stomp],
    Farfetchd => [Foresight, MirrorMove, Gust, QuickAttack, Flail],
    Doduo => [QuickAttack, Supersonic, Haze, FaintAttack, Flail],
    Seel => [Lick, PerishSong, Disable, Peck, Slam, Encore],
    Grimer => [Haze, MeanLook, Lick],
    Shellder => [Bubblebeam, TakeDown, Barrier, RapidSpin, Screech],
    Gastly => [Psywave, PerishSong, Haze],
    Onix => [RockSlide, Flail],
    Drowzee => [LightScreen, Barrier],
    Krabby => [Dig, Haze, Amnesia, Flail, Slam],
    Exeggcute => [Synthesis, Moonlight, Reflect, MegaDrain, Ancientpower],
    Cubone => [RockSlide, Ancientpower, BellyDrum, Screech, SkullBash, PerishSong, SwordsDance],
    Lickitung => [BellyDrum, Magnitude, BodySlam],
    Koffing => [Screech, Psywave, Psybeam, DestinyBond, PainSplit],
    Rhyhorn => [Crunch, Reversal, RockSlide, Thrash, Pursuit, Counter, Magnitude],
    Chansey => [Present, Metronome, HealBell],
    Tangela => [Flail, Confusion, MegaDrain, Reflect, Amnesia],
    Kangaskhan => [Stomp, Foresight, FocusEnergy, Safeguard, Disable],
    Horsea => [Flail, AuroraBeam, Octazooka, Disable, Splash, DragonRage],
    Goldeen => [Psybeam, Haze, HydroPump],
    MrMime => [FutureSight, Hypnosis, Mimic],
    Scyther => [Counter, Safeguard, BatonPass, RazorWind, Reversal, LightScreen],
    Pinsir => [FuryAttack, Flail],
    Lapras => [AuroraBeam, Foresight],
    Eevee => [Flail, Charm],
    Omanyte => [Bubblebeam, AuroraBeam, Slam, Supersonic, Haze],
    Kabuto => [Bubblebeam, AuroraBeam, RapidSpin, Dig, Flail],
    Aerodactyl => [Whirlwind, Pursuit, Foresight],
    Snorlax => [Lick],
    Dratini => [LightScreen, Mist, Haze, Supersonic],
    Chikorita => [VineWhip, LeechSeed, Counter, Ancientpower, Flail, SwordsDance],
    Cyndaquil => [FurySwipes, QuickAttack, Reversal, Thrash, Foresight, Submission],
    Totodile => [Crunch, Thrash, HydroPump, Ancientpower, RazorWind, RockSlide],
    Sentret => [DoubleEdge, Pursuit, Slash, FocusEnergy, Reversal],
    Hoothoot => [MirrorMove, Supersonic, FaintAttack, WingAttack, Whirlwind, SkyAttack],
    Ledyba => [Psybeam, Bide, LightScreen],
    Spinarak => [Psybeam, Disable, Sonicboom, BatonPass, Pursuit],
    Chinchou => [Flail, Supersonic, Screech],
    Pichu => [Reversal, Bide, Present, Encore, Doubleslap],
    Cleffa => [Present, Metronome, Amnesia, BellyDrum, Splash, Mimic],
    Igglybuff => [PerishSong, Present, FaintAttack],
    Togepi => [Present, MirrorMove, Peck, Foresight, FutureSight],
    Natu => [Haze, DrillPeck, QuickAttack, FaintAttack, SteelWing],
    Mareep => [Thunderbolt, TakeDown, BodySlam, Safeguard, Screech, Reflect],
    Marill => [LightScreen, Present, Amnesia, FutureSight, BellyDrum, PerishSong, Supersonic, Foresight],
    Sudowoodo => [Selfdestruct],
    Hoppip => [Confusion, Growl, Encore, DoubleEdge, Reflect, Amnesia, PayDay],
    Aipom => [Counter, Screech, Pursuit, Agility, Spite, Slam, Doubleslap, BeatUp],
    Yanma => [Whirlwind, Reversal, LeechLife],
    Wooper => [BodySlam, Ancientpower, Safeguard],
    Murkrow => [Whirlwind, DrillPeck, QuickAttack, MirrorMove, WingAttack, SkyAttack],
    Misdreavus => [Screech, DestinyBond],
    Girafarig => [TakeDown, Amnesia, Foresight, FutureSight, BeatUp],
    Pineco => [Reflect, PinMissile, Flail, Swift],
    Dunsparce => [Bide, Ancientpower, RockSlide, Bite, Rage],
    Gligar => [MetalClaw, WingAttack, RazorWind, Counter],
    Snubbull => [Metronome, FaintAttack, Reflect, Present, Crunch, HealBell, Lick, Leer],
    Qwilfish => [Flail, Haze, Bubblebeam, Supersonic],
    Shuckle => [SweetScent],
    Heracross => [Harden, Bide, Flail],
    Sneasel => [Counter, Spite, Foresight, Reflect, Bite],
    Teddiursa => [Crunch, TakeDown, SeismicToss, FocusEnergy, Counter, MetalClaw],
    Slugma => [AcidArmor],
    Swinub => [TakeDown, Bite, BodySlam, RockSlide, Ancientpower],
    Corsola => [RockSlide, Safeguard, Screech, Mist, Amnesia],
    Remoraid => [AuroraBeam, Octazooka, Supersonic, Haze, Screech],
    Delibird => [AuroraBeam, QuickAttack, FutureSight, Splash, RapidSpin],
    Mantine => [Twister, HydroPump, Haze, Slam],
    Skarmory => [DrillPeck, Pursuit, Whirlwind, SkyAttack],
    Houndour => [FireSpin, Rage, Pursuit, Counter, Spite, Reversal, BeatUp],
    Phanpy => [FocusEnergy, BodySlam, Ancientpower, WaterGun],
    Stantler => [Reflect, Spite, Disable, LightScreen, Bite],
    Tyrogue => [RapidSpin, HiJumpKick, MachPunch, MindReader],
    Smoochum => [Meditate],
    Elekid => [KarateChop, Barrier, RollingKick, Meditate, CrossChop],
    Magby => [KarateChop, MegaPunch, Barrier, Screech, CrossChop],
    Miltank => [Present, Reversal, SeismicToss],
    Larvitar => [Pursuit, Stomp, Outrage, FocusEnergy, Ancientpower],
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_egg_moves() {
        assert_eq!(
            PokemonSpecies::Bulbasaur.egg_moves(),
            &[
                Move::LightScreen,
                Move::SkullBash,
                Move::Safeguard,
                Move::RazorWind,
                Move::PetalDance
            ]
        );

        assert_eq!(
            PokemonSpecies::Heracross.egg_moves(),
            &[Move::Harden, Move::Bide, Move::Flail]
        );

        assert_eq!(PokemonSpecies::Smoochum.egg_moves(), &[Move::Meditate]);

        assert!(PokemonSpecies::Pikachu.egg_moves().is_empty());
    }
}
