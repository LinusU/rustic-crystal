use crate::{
    game::constants::{
        item_constants::Item, move_constants::Move, pokemon_constants::PokemonSpecies,
        text_constants::NAME_LENGTH, trainer_constants::Trainer,
    },
    save_state::string::PokeString,
};

pub struct TrainerPartyMon {
    pub level: u8,
    pub species: PokemonSpecies,
    pub held_item: Option<Item>,
    pub moves: Option<&'static [Move]>,
}

pub struct TrainerParty {
    pub name: PokeString<NAME_LENGTH>,
    pub mons: &'static [TrainerPartyMon],
}

macro_rules! parties {
    (
        $(
            $trainer:ident => {
                $name:literal
                $(
                    $level:literal $species:ident $(+ $item:ident)? $( [ $( $move:ident ),* ] )?
                )*
            }
        )*
    ) => {
        impl Trainer {
            pub fn party(&self) -> &'static TrainerParty {
                match self {
                    $(
                        Trainer::$trainer => {
                            const PARTY: TrainerParty = TrainerParty {
                                name: PokeString::from_ascii($name.as_bytes()),
                                mons: &[
                                    $(
                                        TrainerPartyMon {
                                            level: $level,
                                            species: PokemonSpecies::$species,
                                            held_item: parties!(@item $( $item )? ),
                                            moves: parties!(@moves $( $( $move ),* )? ),
                                        }
                                    ),*
                                ],
                            };
                            &PARTY
                        }
                    ),*
                    Trainer::Unknown(x, y) => unimplemented!("No party defined for trainer {x}:{y}")
                }
            }
        }
    };

    // Helper for optional item
    (@item) => { None };
    (@item $item:ident) => {
        Some($crate::game::constants::item_constants::Item::$item)
    };

    // Helper for optional move list
    (@moves) => { None };
    (@moves $($mv:ident),+ $(,)?) => {
        Some(&[
            $( $crate::game::constants::move_constants::Move::$mv ),+
        ])
    };
}

parties! {
    Falkner1 => {
        "FALKNER"
         8 Pidgey          [ Tackle, MudSlap, QuickAttack ]
        10 Noctowl + Berry [ Tackle, MudSlap, Growl, Peck ]
    }

    Falkner2 => {
        "FALKNER"
        54 Noctowl + GoldBerry [ Psychic, WingAttack, Hypnosis, DreamEater ]
        53 Skarmory            [ Spikes, DrillPeck, MudSlap, SteelWing ]
        53 Dodrio + FocusBand  [ Endure, Flail, DrillPeck, TriAttack ]
        54 Murkrow             [ DrillPeck, FaintAttack, Haze, IcyWind ]
        55 Pidgeot             [ SkyAttack, Extremespeed, DoubleTeam, SteelWing ]
    }

    Whitney1 => {
        "WHITNEY"
        20 Clefairy               [ Headbutt, Charm, IcePunch, Metronome ]
        20 Teddiursa              [ Headbutt, MetalClaw, Lick, MudSlap ]
        21 Miltank + Miracleberry [ Rollout, Attract, Stomp, MilkDrink ]
    }

    Whitney2 => {
        "WHITNEY"
        53 Clefable             [ IcePunch, FirePunch, Thunderpunch, Encore ]
        54 Ursaring + MintBerry [ Strength, Earthquake, Rest, MetalClaw ]
        53 Blissey              [ EggBomb, Attract, Softboiled, LightScreen ]
        54 Tauros               [ Thrash, Earthquake, Rage, Pursuit ]
        55 Miltank + PinkBow    [ IronTail, Attract, BodySlam, MilkDrink ]
    }

    Bugsy1 => {
        "BUGSY"
        15 Pineco               [ PinMissile, Headbutt, MegaDrain, Harden ]
        15 Ledian               [ Thunderpunch, CometPunch, Supersonic, Reflect ]
        16 Scyther + BerryJuice [ QuickAttack, FuryCutter, Cut, Pursuit ]
    }

    Bugsy2 => {
        "BUGSY"
        54 Ledian + FocusBand   [ GigaDrain, Agility, BatonPass, LightScreen ]
        53 Forretress           [ Curse, Explosion, PinMissile, SpikeCannon ]
        53 Shuckle + BerryJuice [ Toxic, Protect, Sandstorm, Wrap ]
        54 Scizor               [ MetalClaw, Twineedle, Agility, Substitute ]
        55 Scyther + ScopeLens  [ Slash, Cut, WingAttack, SwordsDance ]
    }

    Morty1 => {
        "MORTY"
        24 Haunter               [ Hypnosis, Nightmare, Curse, NightShade ]
        23 Stantler              [ Swift, Hypnosis, Nightmare, Leer ]
        24 Misdreavus            [ ShadowBall, PainSplit, PerishSong, Psywave ]
        25 Gengar + Miracleberry [ Hypnosis, ShadowBall, MeanLook, DreamEater ]
    }

    Morty2 => {
        "MORTY"
        54 Gengar             [ ShadowBall, DestinyBond, MeanLook, ConfuseRay ]
        53 Stantler + PinkBow [ ShadowBall, Headbutt, Hypnosis, DreamEater ]
        54 Gengar + SpellTag  [ ShadowBall, GigaDrain, SludgeBomb, Psychic ]
        53 Misdreavus         [ ShadowBall, MeanLook, PainSplit, PerishSong ]
        55 Gengar + GoldBerry [ ShadowBall, Hypnosis, Thunderbolt, DreamEater ]
    }

    Pryce1 => {
        "PRYCE"
        31 Dewgong               [ Surf, AuroraBeam, RainDance, Rest ]
        32 Sneasel               [ FaintAttack, MetalClaw, Reflect, IcyWind ]
        31 Jynx                  [ IcePunch, Psychic, RainDance, LovelyKiss ]
        33 Piloswine + GoldBerry [ Headbutt, Earthquake, RockSmash, Blizzard ]
    }

    Pryce2 => {
        "PRYCE"
        33 Cloyster              [ Surf, Spikes, AuroraBeam, RainDance ]
        34 Sneasel               [ FaintAttack, MetalClaw, Reflect, IcePunch ]
        33 Jynx                  [ IcePunch, Psychic, RainDance, LovelyKiss ]
        34 Dewgong               [ Surf, AuroraBeam, Snore, Rest ]
        35 Piloswine + GoldBerry [ Headbutt, Earthquake, RockSmash, Blizzard ]
    }

    Pryce3 => {
        "PRYCE"
        37 Cloyster              [ Surf, Spikes, AuroraBeam, RainDance ]
        37 Sneasel               [ FaintAttack, MetalClaw, Reflect, Blizzard ]
        37 Jynx                  [ IcePunch, Psychic, ShadowBall, LovelyKiss ]
        37 Dewgong               [ Surf, AuroraBeam, Snore, Rest ]
        38 Piloswine + GoldBerry [ Headbutt, Earthquake, RockSmash, Blizzard ]
    }

    Pryce4 => {
        "PRYCE"
        54 Sneasel                [ FaintAttack, MetalClaw, BeatUp, Blizzard ]
        53 Dewgong + Miracleberry [ Surf, IceBeam, Rest, Encore ]
        53 Jynx                   [ IceBeam, Psychic, ShadowBall, LovelyKiss ]
        54 Lapras                 [ Surf, IceBeam, BodySlam, ConfuseRay ]
        55 Piloswine + SoftSand   [ Strength, Earthquake, RockSmash, Blizzard ]
    }

    Jasmine1 => {
        "JASMINE"
        35 Skarmory            [ Spikes, DrillPeck, Swagger, SteelWing ]
        35 Corsola             [ RainDance, Surf, Recover, Ancientpower ]
        35 Magneton            [ Thunderbolt, TriAttack, RainDance, Thunder ]
        36 Scizor + FocusBand  [ Twineedle, MetalClaw, Agility, SwordsDance ]
        36 Steelix + QuickClaw [ Crunch, Earthquake, RockSlide, IronTail ]
    }

    Jasmine2 => {
        "JASMINE"
        35 Skarmory            [ Spikes, DrillPeck, Swagger, SteelWing ]
        35 Corsola             [ RainDance, Surf, Recover, Ancientpower ]
        35 Magneton            [ Thunderbolt, TriAttack, RainDance, Thunder ]
        36 Scizor + FocusBand  [ Twineedle, MetalClaw, Agility, SwordsDance ]
        36 Steelix + QuickClaw [ Crunch, Earthquake, RockSlide, IronTail ]
    }

    Jasmine3 => {
        "JASMINE"
        37 Skarmory            [ Spikes, DrillPeck, Swagger, SteelWing ]
        37 Corsola             [ RainDance, Surf, Recover, Ancientpower ]
        37 Magneton            [ Thunderbolt, TriAttack, RainDance, Thunder ]
        38 Scizor + FocusBand  [ Twineedle, MetalClaw, Agility, SwordsDance ]
        38 Steelix + QuickClaw [ Crunch, Earthquake, RockSlide, IronTail ]
    }

    Jasmine4 => {
        "JASMINE"
        53 Skarmory            [ Spikes, DrillPeck, Swagger, SteelWing ]
        53 Corsola             [ RainDance, Surf, Recover, Ancientpower ]
        54 Magneton            [ DoubleEdge, Substitute, RainDance, Thunder ]
        55 Scizor + FocusBand  [ Twineedle, SwordsDance, SteelWing, LightScreen ]
        55 Steelix + QuickClaw [ Crunch, Earthquake, RockSlide, IronTail ]
    }

    Chuck1 => {
        "CHUCK"
        30 Hitmontop             [ TripleKick, RollingKick, Dig, Pursuit ]
        29 Primeape              [ LowKick, KarateChop, IcePunch, Rage ]
        30 Sudowoodo             [ RockSlide, Thunderpunch, LowKick, FaintAttack ]
        31 Poliwrath + Blackbelt [ Hypnosis, MindReader, Surf, Dynamicpunch ]
    }

    Chuck2 => {
        "CHUCK"
        35 Hitmontop              [ TripleKick, RollingKick, Dig, Pursuit ]
        35 Sudowoodo             [ RockSlide, Thunderpunch, LowKick, FaintAttack ]
        34 Primeape              [ LowKick, KarateChop, IcePunch, Rage ]
        35 Pinsir                [ RockSmash, Twineedle, Vicegrip, Harden ]
        36 Poliwrath + Blackbelt [ Hypnosis, MindReader, Surf, Dynamicpunch ]
    }

    Chuck3 => {
        "CHUCK"
        37 Hitmontop              [ TripleKick, RollingKick, Dig, Pursuit ]
        37 Sudowoodo             [ RockSlide, Thunderpunch, LowKick, FaintAttack ]
        37 Primeape              [ Dynamicpunch, KarateChop, RockSlide, Rage ]
        37 Pinsir                [ Submission, Twineedle, Strength, SwordsDance ]
        38 Poliwrath + Blackbelt [ IcePunch, MindReader, Surf, Dynamicpunch ]
    }

    Chuck4 => {
        "CHUCK"
        54 Hitmontop + ScopeLens [ TripleKick, DoubleKick, Dig, Pursuit ]
        53 Sudowoodo             [ RockSlide, Thunderpunch, LowKick, FaintAttack ]
        54 Primeape              [ CrossChop, Thunderpunch, RockSlide, Meditate ]
        53 Pinsir                [ Submission, Twineedle, Strength, SwordsDance ]
        55 Poliwrath + GoldBerry [ Surf, Dynamicpunch, BellyDrum, BodySlam ]
    }

    Clair1 => {
        "CLAIR"
        42 Dragonair             [ ThunderWave, Thunderbolt, Flamethrower, Dragonbreath ]
        43 Gyarados + FocusBand  [ Bite, Waterfall, Fly, Twister ]
        43 Lapras + MintBerry    [ Surf, IceBeam, BodySlam, Rest ]
        44 Dragonair + GoldBerry [ ThunderWave, Thunderbolt, IceBeam, Dragonbreath ]
        45 Kingdra + Leftovers   [ Smokescreen, Surf, HyperBeam, Dragonbreath ]
    }

    Clair2 => {
        "CLAIR"
        54 Dragonair + GoldBerry [ ThunderWave, Thunderbolt, Flamethrower, Dragonbreath ]
        53 Gyarados + FocusBand  [ Bite, Surf, Fly, Twister ]
        53 Lapras + Miracleberry [ Surf, IceBeam, BodySlam, Rest ]
        54 Dragonair + GoldBerry [ ThunderWave, Thunderbolt, IceBeam, Dragonbreath ]
        55 Kingdra + Leftovers   [ Agility, HydroPump, IceBeam, Dragonbreath ]
    }

    Rival11Chikorita => {
        "?"
        5 Chikorita
    }

    Rival11Cyndaquil => {
        "?"
        5 Cyndaquil
    }

    Rival11Totodile => {
        "?"
        5 Totodile
    }

    Rival12Chikorita => {
        "?"
        15 Zubat
        16 Larvitar
        18 Bayleef
    }

    Rival12Cyndaquil => {
        "?"
        15 Zubat
        16 Larvitar
        18 Quilava
    }

    Rival12Totodile => {
        "?"
        15 Zubat
        16 Larvitar
        18 Croconaw
    }

    Rival13Chikorita => {
        "?"
        22 Larvitar [ Bite, RockThrow, Screech ]
        23 Remoraid [ Bubblebeam, AuroraBeam, Psybeam ]
        22 Golbat   [ Bite, ConfuseRay, LeechLife, Gust ]
        24 Bayleef  [ MegaDrain, RazorLeaf, Tackle, LeechSeed ]
    }

    Rival13Cyndaquil => {
        "?"
        22 Larvitar   [ Bite, RockThrow, Screech ]
        23 Weepinbell [ RazorLeaf, Growth, SleepPowder, StunSpore ]
        22 Golbat     [ Bite, ConfuseRay, LeechLife, Gust ]
        24 Quilava    [ FlameWheel, Dig, QuickAttack, Smokescreen ]
    }

    Rival13Totodile => {
        "?"
        22 Larvitar [ Bite, RockThrow, Screech ]
        23 Houndour [ Ember, Bite, Smog, Roar ]
        22 Golbat   [ Bite, ConfuseRay, LeechLife, Gust ]
        24 Croconaw [ Bite, IcePunch, WaterGun, MudSlap ]
    }

    Rival14Chikorita => {
        "?"
        39 Pupitar   [ Thrash, RockSlide, Screech, Bite ]
        39 Houndoom  [ Flamethrower, IronTail, Bite, DoubleTeam ]
        38 Octillery [ Octazooka, Psybeam, AuroraBeam ]
        39 Golbat    [ Toxic, DoubleTeam, ConfuseRay, WingAttack ]
        40 Meganium  [ Synthesis, RazorLeaf, SunnyDay, BodySlam ]
    }

    Rival14Cyndaquil => {
        "?"
        39 Pupitar    [ Thrash, RockSlide, Screech, Bite ]
        39 Victreebel [ Sludge, RazorLeaf, SleepPowder, Growth ]
        38 Octillery  [ Octazooka, Psybeam, AuroraBeam ]
        39 Golbat     [ Toxic, DoubleTeam, ConfuseRay, WingAttack ]
        40 Typhlosion [ Thunderpunch, Flamethrower, QuickAttack, IronTail ]
    }

    Rival14Totodile => {
        "?"
        39 Pupitar    [ Thrash, RockSlide, Screech, Bite ]
        39 Victreebel [ Sludge, RazorLeaf, SleepPowder, Growth ]
        38 Houndoom   [ Flamethrower, IronTail, Bite, DoubleTeam ]
        39 Golbat     [ Toxic, DoubleTeam, ConfuseRay, WingAttack ]
        40 Feraligatr [ IcePunch, Surf, Slash, Bite ]
    }

    Rival15Chikorita => {
        "?"
        45 Ursaring + Miracleberry [ Slash, FaintAttack, RockSmash, Rest ]
        45 Golbat                  [ Toxic, DoubleTeam, ConfuseRay, WingAttack ]
        46 Octillery + QuickClaw   [ Surf, IceBeam, Psybeam, HyperBeam ]
        46 Houndoom                [ Flamethrower, IronTail, Crunch, SunnyDay ]
        47 Meganium + MiracleSeed  [ SunnyDay, GigaDrain, BodySlam, Synthesis ]
        48 Tyranitar               [ Crunch, Earthquake, RockSlide, FirePunch ]
    }

    Rival15Cyndaquil => {
        "?"
        45 Ursaring + Miracleberry [ Slash, FaintAttack, RockSmash, Rest ]
        45 Golbat                  [ Toxic, DoubleTeam, ConfuseRay, WingAttack ]
        46 Octillery + QuickClaw   [ Surf, IceBeam, Psybeam, HyperBeam ]
        46 Victreebel              [ SludgeBomb, GigaDrain, SleepPowder, Growth ]
        47 Typhlosion + Charcoal   [ Flamethrower, QuickAttack, IronTail, Thunderpunch ]
        48 Tyranitar               [ Crunch, Earthquake, RockSlide, FirePunch ]
    }

    Rival15Totodile => {
        "?"
        45 Ursaring + Miracleberry  [ Slash, FaintAttack, RockSmash, Rest ]
        45 Golbat                   [ Toxic, DoubleTeam, ConfuseRay, WingAttack ]
        46 Victreebel + QuickClaw   [ SludgeBomb, GigaDrain, SleepPowder, Growth ]
        46 Houndoom                 [ Flamethrower, IronTail, Crunch, DoubleTeam ]
        47 Feraligatr + MysticWater [ IcePunch, Surf, Slash, Bite ]
        48 Tyranitar                [ Crunch, Earthquake, RockSlide, FirePunch ]
    }

    Will1 => {
        "WILL"
        48 Girafarig           [ Psychic, RockSmash, Agility, BatonPass ]
        49 Slowking            [ Amnesia, Flamethrower, Surf, Psychic ]
        48 Espeon              [ Psychic, ShadowBall, Reflect, Headbutt ]
        49 Slowbro + QuickClaw [ Curse, RockSmash, BodySlam, Psychic ]
        50 Xatu + Twistedspoon [ DrillPeck, Recover, ConfuseRay, Psychic ]
    }

    Will2 => {
        "WILL"
        66 Stantler + ScopeLens     [ DoubleEdge, Earthquake, HiddenPower, Reflect ]
        67 Slowking + Leftovers     [ Flamethrower, Surf, Rest, SleepTalk ]
        67 Exeggutor + Miracleberry [ Ancientpower, SleepPowder, DreamEater, Softboiled ]
        67 Ninetales + Charcoal     [ FireBlast, ShadowBall, ConfuseRay, IronTail ]
        66 Slowbro + QuickClaw      [ Flamethrower, Curse, Earthquake, RockSmash ]
        68 Xatu + Leftovers         [ FutureSight, Fly, ConfuseRay, Protect ]
    }

    Cal1 => {
        "CAL"
        10 Chikorita
        10 Cyndaquil
        10 Totodile
    }

    Cal2 => {
        "CAL"
        30 Bayleef
        30 Quilava
        30 Croconaw
    }

    Cal3 => {
        "CAL"
        50 Meganium
        50 Typhlosion
        50 Feraligatr
    }

    Smith => {
        "SMITH"
        70 Raikou + Magnet           [ Thunderbolt, Crunch, Roar, DoubleTeam ]
        70 Venusaur + GoldBerry      [ LeechSeed, Toxic, Protect, GigaDrain ]
        70 Alakazam + Twistedspoon   [ Psychic, Thunderpunch, Dynamicpunch, ShadowBall ]
        70 Scizor + MetalCoat        [ Twineedle, SwordsDance, SteelWing, LightScreen ]
        70 Dragonite + BitterBerry   [ Outrage, Flamethrower, Thunderbolt, Extremespeed ]
        70 Feraligatr + Miracleberry [ Surf, Earthquake, IcePunch, Crunch ]
    }

    Craig => {
        "CRAIG"
        70 Jolteon + MintBerry       [ BatonPass, Growth, Thunderbolt, HiddenPower ]
        70 Meganium + Leftovers      [ GigaDrain, LightScreen, LeechSeed, Synthesis ]
        70 Aerodactyl + GoldBerry    [ Curse, Earthquake, Ancientpower, Substitute ]
        70 Misdreavus + Miracleberry [ MeanLook, PerishSong, Protect, Sing ]
        70 Suicune + MysticWater     [ Surf, IceBeam, Rest, SleepTalk ]
        70 Arcanine + Charcoal       [ FireBlast, Extremespeed, Curse, HiddenPower ]
    }

    Bruno1 => {
        "BRUNO"
        53 Hitmonchan + ScopeLens [ MachPunch, Pursuit, DizzyPunch, Thunderpunch ]
        52 Heracross              [ CrossChop, Megahorn, TakeDown, Earthquake ]
        53 Hitmonlee + PinkBow    [ BodySlam, Meditate, Reversal, HiJumpKick ]
        53 Steelix                [ Crunch, Earthquake, RockSlide, IronTail ]
        54 Machamp + Blackbelt    [ RockSlide, FirePunch, VitalThrow, CrossChop ]
    }

    Bruno2 => {
        "BRUNO"
        67 Steelix + QuickClaw   [ Crunch, Earthquake, Explosion, IronTail ]
        68 Poliwrath + ScopeLens [ Submission, Blizzard, HydroPump, Psychic ]
        67 Heracross + QuickClaw [ CrossChop, Megahorn, Reversal, Earthquake ]
        68 Donphan + ScopeLens   [ Curse, Earthquake, Ancientpower, RockSmash ]
        67 Granbull + Leftovers  [ HiddenPower, Crunch, Rest, Snore ]
        68 Machamp + Leftovers   [ RockSlide, Earthquake, BodySlam, CrossChop ]
    }

    Karen1 => {
        "KAREN"
        54 Umbreon + Miracleberry  [ FaintAttack, DoubleTeam, BatonPass, Moonlight ]
        53 Gengar                  [ ShadowBall, Hypnosis, DreamEater, ConfuseRay ]
        53 Vileplume + Leftovers   [ SleepPowder, GigaDrain, LeechSeed, Substitute ]
        53 Murkrow + SharpBeak     [ DrillPeck, FaintAttack, SteelWing, Haze ]
        55 Houndoom + Blackglasses [ Flamethrower, Crunch, IronTail, Reversal ]
    }

    Karen2 => {
        "KAREN"
        68 Umbreon + Miracleberry [ Growth, ShadowBall, HiddenPower, BatonPass ]
        67 Gengar + ScopeLens     [ ShadowBall, Psychic, GigaDrain, DestinyBond ]
        68 Persian + ScopeLens    [ Slash, Cut, IronTail, Hypnosis ]
        68 Murkrow + SharpBeak    [ SkyAttack, Pursuit, Swagger, PsychUp ]
        67 Blissey + Leftovers    [ Psychic, Softboiled, Attract, ZapCannon ]
        69 Houndoom + Charcoal    [ Flamethrower, Crunch, IronTail, HiddenPower ]
    }

    Koga1 => {
        "KOGA"
        50 Ariados + KingsRock   [ Megahorn, GigaDrain, DoubleTeam, Toxic ]
        51 Qwilfish              [ Surf, Toxic, Protect, SludgeBomb ]
        50 Muk + Leftovers       [ Minimize, FireBlast, SludgeBomb, Toxic ]
        51 Venomoth              [ LeechLife, Psychic, SludgeBomb, SleepPowder ]
        52 Crobat + Brightpowder [ DoubleTeam, Toxic, Bite, ConfuseRay ]
    }

    Koga2 => {
        "KOGA"
        67 Tentacruel + MintBerry [ Waterfall, Blizzard, GigaDrain, Rest ]
        67 Muk + Leftovers        [ Minimize, FireBlast, SludgeBomb, Toxic ]
        67 Gligar + ScopeLens     [ IronTail, SludgeBomb, Earthquake, FaintAttack ]
        67 Nidoking + QuickClaw   [ LovelyKiss, FireBlast, Surf, Earthquake ]
        67 Hypno + Brightpowder   [ FirePunch, ThunderWave, Psychic, ShadowBall ]
        68 Crobat + Leftovers     [ Protect, Fly, Toxic, ConfuseRay ]
    }

    Lance1 => {
        "LANCE"
        54 Gyarados + Leftovers     [ Surf, RainDance, HyperBeam, RockSmash ]
        55 Dragonite + GoldBerry    [ Blizzard, FireBlast, Thunder, Extremespeed ]
        54 Charizard + Charcoal     [ Flamethrower, WingAttack, DoubleTeam, SteelWing ]
        55 Zapdos + Magnet          [ DrillPeck, SkyAttack, Thunderbolt, ThunderWave ]
        54 Aerodactyl + PinkBow     [ WingAttack, RockSlide, HyperBeam, Earthquake ]
        56 Dragonite + Miracleberry [ Thunder, Safeguard, Outrage, HyperBeam ]
    }

    Lance2 => {
        "LANCE"
        69 Tyranitar + Magnet       [ Crunch, RockSlide, Earthquake, Thunderbolt ]
        68 Dragonite + Miracleberry [ Blizzard, FireBlast, Thunder, Rest ]
        69 Gyarados + QuickClaw     [ HiddenPower, HyperBeam, HydroPump, FireBlast ]
        68 Charizard + Leftovers    [ FireBlast, Crunch, Earthquake, SwordsDance ]
        69 Aerodactyl + ScopeLens   [ SkyAttack, RockSlide, Earthquake, IronTail ]
        70 Dragonite + PinkBow      [ IronTail, Curse, Extremespeed, HyperBeam ]
    }

    Brock1 => {
        "BROCK"
        66 Golem + QuickClaw      [ Curse, RockSlide, BodySlam, Earthquake ]
        66 Aerodactyl + HardStone [ Ancientpower, SkyAttack, Earthquake, FireBlast ]
        66 Kabutops + ScopeLens   [ Surf, Ancientpower, Cut, SwordsDance ]
        66 Omastar + FocusBand    [ Ancientpower, IceBeam, Surf, Toxic ]
        66 Tyranitar + Magnet     [ Crunch, RockSlide, Curse, Thunderbolt ]
        66 Steelix + Leftovers    [ Curse, Earthquake, RockSlide, IronTail ]
    }

    Misty1 => {
        "MISTY"
        62 Golduck               [ Surf, Psychic, Hypnosis, CrossChop ]
        62 Quagsire + QuickClaw  [ Surf, Amnesia, Earthquake, RainDance ]
        61 Vaporeon + Leftovers  [ Surf, IceBeam, AcidArmor, ShadowBall ]
        61 Kingdra               [ HydroPump, IceBeam, Rest, SleepTalk ]
        62 Lapras + Nevermeltice [ Surf, IceBeam, RainDance, Reflect ]
        63 Starmie + MysticWater [ Surf, Psychic, Recover, Thunderbolt ]
    }

    LtSurge1 => {
        "LT.SURGE"
        57 Electrode + FocusBand  [ Thunder, RainDance, ThunderWave, Explosion ]
        59 Magneton               [ Thunder, Reflect, RainDance, DoubleEdge ]
        58 Lanturn + Leftovers    [ Surf, Thunderbolt, IceBeam, ConfuseRay ]
        58 Ampharos               [ RainDance, Thunder, ThunderWave, IronTail ]
        59 Electabuzz + ScopeLens [ Thunderbolt, FirePunch, IcePunch, Submission ]
        60 Raichu + Magnet        [ Thunder, RainDance, BodySlam, Surf ]
    }

    Ross => {
        "ROSS"
        28 Koffing
        28 Raichu
    }

    Mitch => {
        "MITCH"
        28 Electrode
    }

    Jed => {
        "JED"
        28 Magnemite
        28 Porygon
    }

    Marc => {
        "MARC"
        37 Omastar
    }

    Rich => {
        "RICH"
        40 Porygon [ Conversion, Conversion2, Recover, TriAttack ]
    }

    Erika1 => {
        "ERIKA"
        61 Jumpluff + FocusBand    [ GigaDrain, SleepPowder, Encore, LeechSeed ]
        60 Sudowoodo               [ RockSlide, Earthquake, FaintAttack, Thunderpunch ]
        60 Exeggutor               [ GigaDrain, Psychic, StunSpore, Ancientpower ]
        61 Venusaur + Leftovers    [ SunnyDay, Solarbeam, SludgeBomb, SleepPowder ]
        62 Victreebel + PoisonBarb [ SludgeBomb, GigaDrain, SwordsDance, SleepPowder ]
        62 Bellossom + MiracleSeed [ SunnyDay, Synthesis, SleepPowder, Solarbeam ]
    }

    Joey1 => {
        "JOEY"
        4 Rattata
    }

    Mikey => {
        "MIKEY"
        2 Hoothoot
        4 Sentret
    }

    Albert => {
        "ALBERT"
        7 Sentret
        7 Zubat
    }

    Gordon => {
        "GORDON"
        10 Wooper
    }

    Samuel => {
        "SAMUEL"
        12 Teddiursa
        10 Sandshrew
        12 Spearow
    }

    Ian => {
        "IAN"
        12 Mankey
        14 Diglett
    }

    Joey2 => {
        "JOEY"
        27 Rattata + PinkBow [ QuickAttack, HyperFang, Pursuit, MudSlap ]
    }

    Joey3 => {
        "JOEY"
        30 Rattata + PinkBow [ SuperFang, HyperFang, Pursuit, IronTail ]
    }

    Warren => {
        "WARREN"
        60 Fearow
    }

    Jimmy => {
        "JIMMY"
        59 Raticate
        59 Arbok
    }

    Owen => {
        "OWEN"
        55 Arcanine
    }

    Jason => {
        "JASON"
        55 Octillery
        55 Crobat
    }

    Joey4 => {
        "JOEY"
        40 Rattata + PinkBow [ BodySlam, Thunderbolt, Pursuit, IronTail ]
    }

    Joey5 => {
        "JOEY"
        65 Rattata + PinkBow [ BodySlam, Thunderbolt, Pursuit, IronTail ]
    }

    Jack1 => {
        "JACK"
        16 Sunflora
        17 Voltorb
    }

    Kipp => {
        "KIPP"
        59 Electrode
        59 Magnemite
    }

    Alan1 => {
        "ALAN"
        20 Tangela
        20 Growlithe
    }

    Johnny => {
        "JOHNNY"
        59 Tauros
        59 Victreebel
    }

    Danny => {
        "DANNY"
        55 Jynx
        55 Electabuzz
        55 Magmar
    }

    Tommy => {
        "TOMMY"
        60 Xatu
        58 Alakazam
    }

    Dudley => {
        "DUDLEY"
        56 Vileplume
    }

    Joe => {
        "JOE"
        56 Tangela
        56 Vaporeon
    }

    Billy => {
        "BILLY"
        57 Parasect
        57 Poliwrath
        60 Ditto
    }

    Chad1 => {
        "CHAD"
        22 MrMime
        22 Magnemite
    }

    Nate => {
        "NATE"
        62 Ledian
        62 Exeggutor
    }

    Ricky => {
        "RICKY"
        62 Aipom
        62 Ditto
    }

    Jack2 => {
        "JACK"
        24 Sunflora
        23 Voltorb
    }

    Jack3 => {
        "JACK"
        28 Gloom
        31 Electrode
    }

    Alan2 => {
        "ALAN"
        24 Tangela
        25 Growlithe
    }

    Alan3 => {
        "ALAN"
        30 Natu
        35 Tangela
        33 Quagsire
        32 Arcanine
    }

    Chad2 => {
        "CHAD"
        28 MrMime
        26 Magnemite
    }

    Chad3 => {
        "CHAD"
        37 MrMime
        35 Magneton
    }

    Jack4 => {
        "JACK"
        40 Sunflora
        43 Arcanine
        43 Electrode
    }

    Jack5 => {
        "JACK"
        35 Electrode [ Screech, Sonicboom, Rollout, LightScreen ]
        35 Growlithe [ SunnyDay, Leer, TakeDown, FlameWheel ]
        37 Vileplume [ Solarbeam, SleepPowder, Acid, Moonlight ]
    }

    Alan4 => {
        "ALAN"
        27 Natu
        27 Tangela
        30 Quagsire
        30 Yanma
    }

    Alan5 => {
        "ALAN"
        45 Xatu
        45 Tangela
        43 Quagsire
        44 Arcanine
    }

    Chad4 => {
        "CHAD"
        46 MrMime
        44 Magneton
    }

    Chad5 => {
        "CHAD"
        54 MrMime   [ Psychic, LightScreen, Reflect, Encore ]
        58 Magneton [ Thunder, ThunderWave, LockOn, TriAttack ]
    }

    Rod => {
        "ROD"
        7 Pidgey
        7 Natu
    }

    Abe => {
        "ABE"
        9 Spearow
    }

    Bryan => {
        "BRYAN"
        16 Pidgey
        18 Pidgeotto
        18 Spearow
    }

    Theo => {
        "THEO"
        23 Murkrow
    }

    Toby => {
        "TOBY"
        22 Doduo
        22 Doduo
    }

    Denis => {
        "DENIS"
        24 Pidgeotto
        24 Fearow
    }

    Vance1 => {
        "VANCE"
        36 Pidgeot
        36 Murkrow
        36 Xatu
        37 Skarmory
    }

    Hank => {
        "HANK"
        59 Murkrow
        59 Pidgeot
    }

    Roy => {
        "ROY"
        59 Fearow
        59 Fearow
    }

    Boris => {
        "BORIS"
        60 Yanma
        58 Murkrow
        60 Dodrio
    }

    Bob => {
        "BOB"
        61 Noctowl
    }

    Jose1 => {
        "JOSE"
        45 Fearow
        45 Farfetchd
        45 Pidgeot
        45 Dodrio
    }

    Peter => {
        "PETER"
        8 Spearow
        8 Natu
    }

    Jose2 => {
        "JOSE"
        44 Fearow
        44 Farfetchd
        43 Pidgeot
        44 Skarmory
    }

    Perry => {
        "PERRY"
        61 Farfetchd
    }

    Bret => {
        "BRET"
        58 Skarmory
        58 Fearow
    }

    Jose3 => {
        "JOSE"
        48 Fearow
        48 Farfetchd
        48 Pidgeot
        48 Dodrio
        48 Skarmory
    }

    Vance2 => {
        "VANCE"
        46 Pidgeot
        48 Murkrow
        47 Xatu
        49 Skarmory
    }

    Vance3 => {
        "VANCE"
        55 Pidgeot
        56 Murkrow
        54 Xatu
        55 Skarmory
    }

    Carrie => {
        "CARRIE"
        18 Snubbull [ ScaryFace, Charm, Bite, Lick ]
    }

    Bridget => {
        "BRIDGET"
        16 Aipom
        16 Togepi
    }

    Alice => {
        "ALICE"
        58 Vileplume
        59 Arbok
        58 Vileplume
    }

    Krise => {
        "KRISE"
        17 Skiploom
        16 Cubone
    }

    Connie1 => {
        "CONNIE"
        21 Ponyta
        22 Weepinbell
    }

    Linda => {
        "LINDA"
        58 Venusaur
        58 Qwilfish
        58 Muk
    }

    Laura => {
        "LAURA"
        55 Bellossom
        55 Pidgeot
        55 Politoed
    }

    Shannon => {
        "SHANNON"
        56 Parasect
        55 Tangela
        56 Parasect
    }

    Michelle => {
        "MICHELLE"
        57 Jumpluff
        58 Jumpluff
        57 Jumpluff
    }

    Dana1 => {
        "DANA"
        19 Flaaffy [ Tackle, Growl, Thundershock, ThunderWave ]
        20 Psyduck [ Scratch, TailWhip, Disable, Confusion ]
    }

    Ellen => {
        "ELLEN"
        55 Wigglytuff
        55 Granbull
    }

    Connie2 => {
        "CONNIE"
        21 Marill
    }

    Connie3 => {
        "CONNIE"
        21 Marill
        22 Nidorina
    }

    Dana2 => {
        "DANA"
        25 Flaaffy [ Tackle, Growl, Thundershock, ThunderWave ]
        25 Psyduck [ Scratch, TailWhip, Disable, Confusion ]
    }

    Dana3 => {
        "DANA"
        37 Golduck  [ Surf, Disable, Confusion, Screech ]
        37 Ampharos [ Headbutt, Thunderpunch, ThunderWave, CottonSpore ]
    }

    Dana4 => {
        "DANA"
        44 Golduck  [ Surf, Disable, Psychic, Screech ]
        45 Ampharos [ Headbutt, Thunderpunch, ThunderWave, CottonSpore ]
    }

    Dana5 => {
        "DANA"
        54 Golduck  [ Surf, Disable, Psychic, Screech ]
        55 Ampharos [ Headbutt, Thunderpunch, ThunderWave, CottonSpore ]
    }

    Janine1 => {
        "JANINE"
        63 Weezing + Leftovers      [ SludgeBomb, FireBlast, DestinyBond, Amnesia ]
        62 Muk                      [ Minimize, SludgeBomb, Toxic, AcidArmor ]
        61 Tentacruel + MysticWater [ Surf, IceBeam, Toxic, ConfuseRay ]
        62 Nidoqueen + QuickClaw    [ Earthquake, SludgeBomb, Thunderbolt, Submission ]
        63 Crobat                   [ Pursuit, SludgeBomb, Toxic, ConfuseRay ]
        64 Venomoth + Brightpowder  [ GigaDrain, Psychic, Toxic, DoubleTeam ]
    }

    Nick => {
        "NICK"
        26 Charmander [ Ember, Smokescreen, Rage, ScaryFace ]
        26 Squirtle   [ Withdraw, WaterGun, Bite, Curse ]
        26 Bulbasaur  [ LeechSeed, Poisonpowder, SleepPowder, RazorLeaf ]
    }

    Aaron => {
        "AARON"
        27 Ivysaur
        27 Charmeleon
        27 Wartortle
    }

    Paul => {
        "PAUL"
        34 Dratini
        40 Dragonair
        40 Dragonair
        40 Dragonair
    }

    Cody => {
        "CODY"
        42 Seadra
        41 Dragonair
        43 Golduck
        42 Dragonair
    }

    Mike => {
        "MIKE"
        42 Gyarados
        43 Dragonair
        42 Vaporeon
    }

    Gaven1 => {
        "GAVEN"
        50 Victreebel [ Protect, Toxic, Acid, RazorLeaf ]
        50 Kingler    [ Vicegrip, Stomp, Surf, Protect ]
        50 Flareon    [ SandAttack, QuickAttack, Bite, Flamethrower ]
        50 Donphan    [ FuryAttack, Earthquake, Rollout, RockSmash ]
        50 Dragonair  [ Dragonbreath, Surf, Wrap, DragonRage ]
    }

    Gaven2 => {
        "GAVEN"
        55 Victreebel [ Protect, Toxic, Acid, RazorLeaf ]
        55 Kingler    [ Vicegrip, Stomp, Surf, Protect ]
        55 Flareon    [ SandAttack, QuickAttack, Bite, Flamethrower ]
        55 Donphan    [ FuryAttack, Earthquake, Rollout, RockSmash ]
        55 Dragonite  [ Dragonbreath, Fly, Wrap, Thunderbolt ]
    }

    Ryan => {
        "RYAN"
        37 Pidgeot    [ SandAttack, QuickAttack, Swift, WingAttack ]
        37 Electabuzz [ Thunderpunch, Swift, ThunderWave ]
        37 Magmar     [ FirePunch, Smog, Swift ]
    }

    Jake => {
        "JAKE"
        45 Cloyster [ Surf, IceBeam, SpikeCannon, Spikes ]
        47 Pidgeot  [ Extremespeed, WingAttack, SkyAttack, SteelWing ]
        45 Alakazam [ Psychic, Recover, ShadowBall, FutureSight ]
        45 Arcanine [ Flamethrower, Extremespeed, TakeDown, IronTail ]
        45 Jolteon  [ Thunderbolt, Bite, PinMissile, DoubleKick ]
    }

    Gaven3 => {
        "GAVEN"
        45 Victreebel [ Protect, Toxic, Acid, RazorLeaf ]
        46 Kingler    [ Vicegrip, Stomp, Surf, Protect ]
        45 Flareon    [ SandAttack, QuickAttack, Bite, Flamethrower ]
        45 Dragonair  [ Dragonbreath, Surf, Wrap, DragonRage ]
        44 Porygon2   [ TriAttack, IceBeam, DefenseCurl, Recover ]
    }

    Blake => {
        "BLAKE"
        43 Magneton  [ Thunderbolt, Supersonic, Swift, Screech ]
        44 Quagsire  [ WaterGun, Slam, Amnesia, Earthquake ]
        43 Exeggutor [ LeechSeed, Confusion, SleepPowder, Solarbeam ]
        44 Piloswine [ IcyWind, Headbutt, Dig, DefenseCurl ]
    }

    Brian => {
        "BRIAN"
        45 Sandslash [ SandAttack, PoisonSting, Slash, Swift ]
        44 Scizor    [ Cut, WingAttack, Slash, MetalClaw ]
        45 Sneasel   [ Slash, IcyWind, FaintAttack, QuickAttack ]
        43 Ursaring  [ Slash, Rest, FaintAttack ]
    }

    Erick => {
        "ERICK"
        10 Bulbasaur
        10 Charmander
        10 Squirtle
    }

    Andy => {
        "ANDY"
        10 Bulbasaur
        10 Charmander
        10 Squirtle
    }

    Tyler => {
        "TYLER"
        10 Bulbasaur
        10 Charmander
        10 Squirtle
    }

    Sean => {
        "SEAN"
        60 Flareon
        60 Tangela
        60 Tauros
    }

    Kevin => {
        "KEVIN"
        58 Rhyhorn
        57 Lanturn
        57 Espeon
        58 Charizard
    }

    Steve => {
        "STEVE"
        14 Bulbasaur
        14 Charmander
        14 Squirtle
    }

    Allen => {
        "ALLEN"
        35 Charmeleon [ Flamethrower, Slash ]
        37 Electabuzz [ Thunderpunch, ThunderWave, Swift ]
    }

    Darin => {
        "DARIN"
        42 Dragonair [ Dragonbreath, Surf, DragonRage, Slam ]
        42 Dragonair [ Flamethrower, Surf, DragonRage, Slam ]
        42 Dragonair [ Thunderbolt, Surf, DragonRage, Slam ]
        42 Dragonair [ IceBeam, Surf, DragonRage, Slam ]
    }

    Gwen => {
        "GWEN"
        26 Eevee
        23 Flareon
        24 Vaporeon
        25 Jolteon
    }

    Lois => {
        "LOIS"
        28 Skiploom  [ Synthesis, Poisonpowder, MegaDrain, LeechSeed ]
        27 Ninetales [ Ember, QuickAttack, ConfuseRay, Safeguard ]
    }

    Fran => {
        "FRAN"
        41 Seadra
        41 Gyarados
        41 Dragonair
    }

    Lola => {
        "LOLA"
        41 Dragonair
        42 Mantine
        41 Lanturn
        42 Dragonair
    }

    Kate => {
        "KATE"
        26 Shellder
        28 Cloyster
    }

    Irene => {
        "IRENE"
        22 Goldeen
        24 Seaking
    }

    Kelly => {
        "KELLY"
        37 Togetic
        37 Ampharos
        37 Tangela
        38 Blastoise
    }

    Joyce => {
        "JOYCE"
        44 Blastoise [ Bite, Curse, Surf, RainDance ]
        46 Raichu    [ QuickAttack, DoubleTeam, Thunderbolt, Thunder ]
        45 Tauros    [ Thrash, Pursuit, RockSmash, Rage ]
        45 Rhydon    [ RockSlide, Earthquake, Thunderpunch, ScaryFace ]
        45 Jumpluff  [ GigaDrain, Gust, SleepPowder, Synthesis ]
    }

    Beth1 => {
        "BETH"
        45 Rapidash [ Stomp, FireSpin, FuryAttack, Agility ]
        45 Ampharos [ Swift, Thunderpunch, ThunderWave, CottonSpore ]
        45 Miltank  [ Rollout, Attract, Stomp, MilkDrink ]
        45 Lanturn  [ Surf, IceBeam, Thunderbolt, ThunderWave ]
        45 Gengar   [ ShadowBall, Psychic, Thunderpunch ]
    }

    Reena1 => {
        "REENA"
        44 Starmie
        43 Nidoqueen
        44 Vileplume
        43 Electrode
        45 Starmie
    }

    Megan => {
        "MEGAN"
        44 Tangela    [ SunnyDay, LeechSeed, GigaDrain, StunSpore ]
        44 Venusaur   [ BodySlam, SleepPowder, RazorLeaf, Solarbeam ]
        44 Victreebel [ RazorLeaf, Solarbeam, SunnyDay, Acid ]
        44 Bellossom  [ SunnyDay, Solarbeam, RazorLeaf, GigaDrain ]
    }

    Beth2 => {
        "BETH"
        50 Rapidash [ Stomp, FireSpin, FuryAttack, Agility ]
        50 Ampharos [ Swift, Thunderpunch, ThunderWave, CottonSpore ]
        50 Miltank  [ Rollout, Attract, Stomp, MilkDrink ]
        50 Lanturn  [ Surf, IceBeam, Thunderbolt, ThunderWave ]
        50 Gengar   [ ShadowBall, Psychic, Thunderpunch ]
    }

    Carol => {
        "CAROL"
        60 Electrode
        61 Starmie
        60 Ninetales
    }

    Quinn => {
        "QUINN"
        58 Ivysaur
        58 Starmie
    }

    Emma => {
        "EMMA"
        28 Poliwhirl
    }

    Cybil => {
        "CYBIL"
        40 Butterfree [ Confusion, SleepPowder, Whirlwind, Gust ]
        37 Bellossom  [ Absorb, StunSpore, Acid, Solarbeam ]
        38 Quagsire   [ Surf, Slam, MudSlap ]
    }

    Jenn => {
        "JENN"
        24 Staryu
        26 Starmie
    }

    Beth3 => {
        "BETH"
        55 Rapidash [ Stomp, FireSpin, FuryAttack, Agility ]
        55 Ampharos [ Swift, Thunderpunch, ThunderWave, CottonSpore ]
        55 Miltank  [ Rollout, Attract, Stomp, MilkDrink ]
        55 Lanturn  [ Surf, IceBeam, Thunderbolt, ThunderWave ]
        55 Gengar   [ ShadowBall, Psychic, Thunderpunch ]
    }

    Reena2 => {
        "REENA"
        48 Starmie
        47 Nidoqueen
        48 Vileplume
        47 Electrode
        49 Starmie
    }

    Reena3 => {
        "REENA"
        55 Starmie
        55 Nidoqueen
        54 Vileplume
        56 Electrode
        57 Starmie
    }

    Cara => {
        "CARA"
        40 Seadra   [ Smokescreen, Leer, Twineedle, Twister ]
        41 Seadra   [ Swift, Leer, Waterfall, Twister ]
        42 Gyarados [ Dragonbreath, Leer, Gust, Surf ]
    }

    Victoria => {
        "VICTORIA"
        15 Teddiursa
        17 Furret
    }

    Samantha => {
        "SAMANTHA"
        18 Meowth [ Scratch, Growl, Bite, PayDay ]
    }

    Julie => {
        "JULIE"
        15 Sentret [ Tackle, DefenseCurl, QuickAttack, FurySwipes ]
    }

    Jaclyn => {
        "JACLYN"
        15 Sentret [ Tackle, DefenseCurl, QuickAttack, FurySwipes ]
    }

    Brenda => {
        "BRENDA"
        16 Furret
    }

    Cassie => {
        "CASSIE"
        60 Vileplume
        62 Jynx
    }

    Caroline => {
        "CAROLINE"
        30 Marill
        32 Seel
        30 Marill
    }

    Carlene => {
        "CARLENE"
        15 Sentret [ Tackle, DefenseCurl, QuickAttack, FurySwipes ]
    }

    Jessica => {
        "JESSICA"
        15 Sentret [ Tackle, DefenseCurl, QuickAttack, FurySwipes ]
    }

    Rachael => {
        "RACHAEL"
        15 Sentret [ Tackle, DefenseCurl, QuickAttack, FurySwipes ]
    }

    Angelica => {
        "ANGELICA"
        15 Sentret [ Tackle, DefenseCurl, QuickAttack, FurySwipes ]
    }

    Kendra => {
        "KENDRA"
        15 Sentret [ Tackle, DefenseCurl, QuickAttack, FurySwipes ]
    }

    Veronica => {
        "VERONICA"
        15 Sentret [ Tackle, DefenseCurl, QuickAttack, FurySwipes ]
    }

    Julia => {
        "JULIA"
        58 Parasect
        56 Exeggutor
        58 Sunflora
    }

    Theresa => {
        "THERESA"
        15 Sentret [ Tackle, DefenseCurl, QuickAttack, FurySwipes ]
    }

    Valerie => {
        "VALERIE"
        22 Skiploom
        21 Miltank
    }

    Olivia => {
        "OLIVIA"
        21 Corsola
    }

    Larry => {
        "LARRY"
        10 Larvitar
    }

    Andrew => {
        "ANDREW"
        24 Marowak
        24 Marowak
    }

    Calvin => {
        "CALVIN"
        26 Kangaskhan
    }

    Shane => {
        "SHANE"
        24 Nidorino
        24 Nidoking
    }

    Ben => {
        "BEN"
        26 Slowbro
        26 Wartortle
    }

    Brent1 => {
        "BRENT"
        26 Lickitung
        26 Ivysaur
    }

    Ron => {
        "RON"
        26 Nidoking
        26 Charmeleon
    }

    Ethan => {
        "ETHAN"
        61 Haunter
        61 Rhydon
    }

    Brent2 => {
        "BRENT"
        33 Kangaskhan
    }

    Brent3 => {
        "BRENT"
        46 Porygon [ Recover, Psychic, Conversion2, TriAttack ]
    }

    Issac => {
        "ISSAC"
        14 Lickitung [ Lick, Supersonic, DefenseCurl ]
    }

    Donald => {
        "DONALD"
        15 Slowpoke
        15 Slowpoke
    }

    Zach => {
        "ZACH"
        40 Rhydon
        37 Pupitar
        38 Heracross
    }

    Brent4 => {
        "BRENT"
        53 Chansey [ Rollout, Attract, EggBomb, Softboiled ]
    }

    Miller => {
        "MILLER"
        20 Nidoking
        20 Nidoqueen
    }

    GruntM1 => {
        "GRUNT"
        12 Koffing
        13 Slowpoke
        15 Houndour
    }

    GruntM2 => {
        "ETO"
        11 Elekid
        11 Magby
        11 Smoochum
    }

    GruntM3 => {
        "GRUNT"
        33 Raticate
        33 Raticate
        33 Raticate
    }

    GruntM4 => {
        "GRUNT"
        33 Weezing
        33 Muk
    }

    GruntM5 => {
        "GRUNT"
        34 Aipom
        34 Koffing
        34 Aipom
    }

    GruntM6 => {
        "GRUNT"
        34 Gligar
        34 Hypno
    }

    GruntM7 => {
        "GRUNT"
        34 Aipom
        34 Murkrow
        34 Forretress
    }

    GruntM8 => {
        "GRUNT"
        34 Muk
        34 Venomoth
    }

    GruntM9 => {
        "GRUNT"
        35 Tauros
        35 Slowbro
    }

    GruntM10 => {
        "GRUNT"
        35 Exeggutor
        35 Electrode
        35 Electabuzz
    }

    GruntM11 => {
        "GRUNT"
        36 Golbat
        37 Muk
    }

    GruntM12 => {
        "EXECUTIVE"
        33 Houndoom
    }

    GruntM13 => {
        "GRUNT"
        36 Golbat
        37 Weezing
    }

    GruntM14 => {
        "GRUNT"
        35 Weezing
        37 Gligar
        35 Hypno
    }

    GruntM15 => {
        "ETO"
        36 Poliwrath
        36 Magmar
        36 Jynx
        36 Electabuzz
        36 Rhydon
    }

    GruntM16 => {
        "GRUNT"
        28 Kangaskhan
    }

    GruntM17 => {
        "GRUNT"
        29 Golbat
    }

    GruntM18 => {
        "GRUNT"
        26 Raticate
        27 Grimer
        26 Golbat
    }

    GruntM19 => {
        "GRUNT"
        31 Venomoth
    }

    GruntM20 => {
        "GRUNT"
        26 Drowzee
        26 Gligar
    }

    GruntM21 => {
        "GRUNT"
        27 Golbat
        27 Ariados
        27 Raticate
    }

    GruntM22 => {
        "EXECUTIVE"
        36 Golbat
    }

    GruntM23 => {
        "EXECUTIVE"
        30 Koffing
    }

    GruntM24 => {
        "GRUNT"
        37 Weezing
    }

    GruntM25 => {
        "GRUNT"
        36 Golbat
        37 Arbok
    }

    GruntM26 => {
        "GRUNT"
        15 Rattata
        15 Rattata
    }

    GruntM27 => {
        "EXECUTIVE"
        22 Zubat
    }

    GruntM28 => {
        "ETO"
        28 Jynx
        28 Poliwhirl
        28 Magmar
        28 Electabuzz
    }

    GruntM29 => {
        "GRUNT"
        9 Rattata
        9 Zubat
    }

    GruntM30 => {
        "GRUNT"
        25 Golbat
        25 Golbat
        30 Arbok
    }

    GruntM31 => {
        "GRUNT"
        58 Crobat
        58 Weezing
        58 Marowak
        58 Hypno
        58 Cloyster
    }

    Preston => {
        "PRESTON"
        22 Growlithe
        22 Vulpix
    }

    Edward => {
        "EDWARD"
        63 Persian
    }

    Gregory => {
        "GREGORY"
        57 Pikachu + LightBall [ Thunderbolt, Surf, ThunderWave, DoubleTeam ]
        56 Ampharos + Magnet   [ Thunderbolt, CottonSpore, FirePunch, IronTail ]
    }

    Virgil => {
        "VIRGIL"
        20 Ponyta
    }

    Alfred => {
        "ALFRED"
        22 Noctowl
    }

    Roxanne => {
        "ROXANNE"
        30 Jynx
    }

    Clarissa => {
        "CLARISSA"
        31 Dewgong
    }

    Colette => {
        "COLETTE"
        60 Clefairy
    }

    Hillary => {
        "HILLARY"
        61 Aipom
        61 Cubone
    }

    Shirley => {
        "SHIRLEY"
        61 Jigglypuff
        63 Wigglytuff
    }

    Sabrina1 => {
        "SABRINA"
        65 MrMime + QuickClaw      [ Psychic, ThunderWave, Encore, IcePunch ]
        64 Jynx                    [ Psychic, IceBeam, LovelyKiss, Bubblebeam ]
        64 Slowbro + MintBerry     [ Rest, SleepTalk, Submission, Surf ]
        64 Wobbuffet + Leftovers   [ MirrorCoat, Counter, Safeguard, DestinyBond ]
        65 Hypno + FocusBand       [ DreamEater, Hypnosis, FirePunch, Submission ]
        66 Alakazam + Twistedspoon [ ShadowBall, Psychic, Recover, Thunderpunch ]
    }

    Don => {
        "DON"
        3 Ledyba
        3 Spinarak
    }

    Rob => {
        "ROB"
        62 Beedrill
        62 Butterfree
    }

    Ed => {
        "ED"
        60 Beedrill
        60 Beedrill
        60 Beedrill
    }

    Wade1 => {
        "WADE"
        4 Weedle
        5 Pineco
    }

    BugCatcherBenny => {
        "BENNY"
         9 Kakuna
        12 Beedrill
    }

    Al => {
        "AL"
         9 Metapod
        12 Butterfree
    }

    Josh => {
        "JOSH"
        12 Yanma
    }

    Arnie1 => {
        "ARNIE"
        18 Yanma
        18 Venonat
    }

    Ken => {
        "KEN"
        60 Ariados
        62 Pinsir
    }

    Wade2 => {
        "WADE"
        20 Beedrill
        20 Pineco
    }

    Wade3 => {
        "WADE"
        25 Butterfree
        25 Beedrill
        25 Pineco
    }

    Doug => {
        "DOUG"
        62 Ariados
    }

    Arnie2 => {
        "ARNIE"
        28 Yanma
        28 Venonat
    }

    Arnie3 => {
        "ARNIE"
        38 Venomoth
        37 Yanma
        38 Scyther
    }

    Wade4 => {
        "WADE"
        35 Butterfree
        34 Beedrill
        35 Forretress
        36 Ariados
    }

    Wade5 => {
        "WADE"
        46 Butterfree
        45 Beedrill
        46 Ledian
        45 Forretress
        47 Ariados
    }

    Arnie4 => {
        "ARNIE"
        48 Venomoth
        47 Yanma
        48 Scizor
        48 Pinsir
    }

    Arnie5 => {
        "ARNIE"
        58 Venomoth
        56 Yanma
        57 Scizor
        56 Pinsir
    }

    Wayne => {
        "WAYNE"
        15 Paras
        15 Oddish
    }

    Justin => {
        "JUSTIN"
        7 Tentacool
    }

    Ralph1 => {
        "RALPH"
        8 Goldeen
    }

    Arnold => {
        "ARNOLD"
        61 Lanturn
        61 Quagsire
    }

    Kyle => {
        "KYLE"
        58 Seaking
        58 Poliwhirl
        59 Seaking
    }

    Henry => {
        "HENRY"
        7 Marill
        7 Poliwag
    }

    Marvin => {
        "MARVIN"
        25 Gyarados
        25 Gyarados
    }

    Tully1 => {
        "TULLY"
        24 Qwilfish
    }

    Andre => {
        "ANDRE"
        27 Gyarados
    }

    Raymond => {
        "RAYMOND"
        28 Vaporeon
    }

    Wilton1 => {
        "WILTON"
        38 Qwilfish
        36 Octillery
        36 Seaking
    }

    Edgar => {
        "EDGAR"
        38 Octillery [ LockOn, Psybeam, AuroraBeam, Bubblebeam ]
        36 Gyarados  [ Surf, DragonRage, Leer, Twister ]
        36 Seaking   [ Waterfall, HornAttack, Peck ]
    }

    Jonah => {
        "JONAH"
        55 Shellder
        59 Octillery
        55 Remoraid
        59 Cloyster
    }

    Martin => {
        "MARTIN"
        58 Remoraid
        60 Octillery
    }

    Stephen => {
        "STEPHEN"
        75 Magikarp
        57 Gyarados
        58 Qwilfish
        57 Tentacruel
    }

    Barney => {
        "BARNEY"
        58 Gyarados
        58 Gyarados
        58 Gyarados
    }

    Ralph2 => {
        "RALPH"
        24 Goldeen
    }

    Ralph3 => {
        "RALPH"
        28 Qwilfish
        28 Seaking
    }

    Tully2 => {
        "TULLY"
        24 Qwilfish
    }

    Tully3 => {
        "TULLY"
        42 Seaking
        45 Qwilfish
        42 Seaking
    }

    Wilton2 => {
        "WILTON"
        48 Qwilfish
        46 Octillery
        48 Seaking
    }

    Scott => {
        "SCOTT"
        45 Qwilfish
        44 Seaking
        43 Gyarados
        43 Quagsire
    }

    Wilton3 => {
        "WILTON"
        58 Qwilfish
        56 Octillery
        56 Seaking
    }

    Ralph4 => {
        "RALPH"
        45 Qwilfish   [ Toxic, Minimize, Surf, PinMissile ]
        48 Seaking    [ Waterfall, Megahorn, RainDance, DrillPeck ]
        45 Tentacruel [ SludgeBomb, Surf, AcidArmor, IceBeam ]
    }

    Ralph5 => {
        "RALPH"
        55 Qwilfish   [ Toxic, Minimize, Surf, PinMissile ]
        58 Seaking    [ Waterfall, Megahorn, RainDance, DrillPeck ]
        55 Tentacruel [ SludgeBomb, Surf, AcidArmor, IceBeam ]
    }

    Tully4 => {
        "TULLY"
        52 Seaking
        55 Qwilfish
        52 Seaking
    }

    Harold => {
        "HAROLD"
        62 Octillery
        61 Seadra
    }

    Simon => {
        "SIMON"
        25 Tentacool
        25 Tentacool
    }

    Randall => {
        "RANDALL"
        25 Shellder
        25 Wartortle
    }

    Charlie => {
        "CHARLIE"
        26 Cloyster
        26 Tentacruel
    }

    George => {
        "GEORGE"
        26 Tentacool
        26 Remoraid
        26 Staryu
    }

    Berke => {
        "BERKE"
        27 Qwilfish
    }

    Kirk => {
        "KIRK"
        24 Gyarados
        24 Gyarados
    }

    Mathew => {
        "MATHEW"
        26 Krabby
        26 Qwilfish
    }

    Hal => {
        "HAL"
        24 Seel
        25 Dewgong
        24 Seel
    }

    Paton => {
        "PATON"
        26 Piloswine
        26 Piloswine
    }

    Daryl => {
        "DARYL"
        24 Shellder
        25 Cloyster
        24 Shellder
    }

    Walter => {
        "WALTER"
        15 Horsea
        15 Horsea
        20 Seadra
    }

    Tony => {
        "TONY"
        13 Staryu
        18 Starmie
        16 Horsea
    }

    Jerome => {
        "JEROME"
        58 Vaporeon
        58 Tentacruel
        58 Seaking
    }

    Tucker => {
        "TUCKER"
        60 Qwilfish
        60 Cloyster
    }

    Rick => {
        "RICK"
        13 Staryu
        18 Starmie
        16 Horsea
    }

    Cameron => {
        "CAMERON"
        64 Azumarill
    }

    Seth => {
        "SETH"
        59 Quagsire
        59 Octillery
        59 Gyarados
    }

    James => {
        "JAMES"
        13 Staryu
        18 Starmie
        16 Horsea
    }

    Lewis => {
        "LEWIS"
        13 Staryu
        18 Starmie
        16 Horsea
    }

    Parker => {
        "PARKER"
        57 Seadra
        57 Gyarados
        57 Seadra
    }

    Elaine => {
        "ELAINE"
        25 Starmie
    }

    Paula => {
        "PAULA"
        25 Staryu
        26 Shellder
    }

    Kaylee => {
        "KAYLEE"
        24 Seaking
        24 Lanturn
        24 Quagsire
    }

    Susie => {
        "SUSIE"
        27 Psyduck [ Scratch, TailWhip, Disable, Confusion ]
        26 Seaking [ Peck, Surf, Supersonic, HornAttack ]
    }

    Denise => {
        "DENISE"
        27 Lapras
    }

    Kara => {
        "KARA"
        25 Horsea
        26 Starmie
    }

    Wendy => {
        "WENDY"
        26 Horsea [ Bubblebeam, Smokescreen, Leer, WaterGun ]
        27 Seadra [ DragonRage, Smokescreen, Twister, Bubblebeam ]
    }

    Lisa => {
        "LISA"
        28 Jynx
    }

    Jill => {
        "JILL"
        28 Dewgong
    }

    Mary => {
        "MARY"
        20 Seaking
    }

    Katie => {
        "KATIE"
        33 Dewgong
    }

    Dawn => {
        "DAWN"
        60 Seaking
    }

    Tara => {
        "TARA"
        20 Seaking
    }

    Nicole => {
        "NICOLE"
        60 Mantine
        63 Lapras
    }

    Lori => {
        "LORI"
        62 Starmie
        62 Starmie
    }

    Jody => {
        "JODY"
        20 Seaking
    }

    Nikki => {
        "NIKKI"
        58 Dewgong
        58 Cloyster
        58 Dewgong
    }

    Diana => {
        "DIANA"
        57 Golduck
        57 Cloyster
        57 Corsola
    }

    Briana => {
        "BRIANA"
        58 Seaking
        58 Azumarill
        58 Seaking
    }

    Eugene => {
        "EUGENE"
        20 Poliwhirl
        22 Tauros
    }

    Huey1 => {
        "HUEY"
        20 Poliwhirl
        22 Machop
    }

    Terrell => {
        "TERRELL"
        24 Poliwhirl
    }

    Kent => {
        "KENT"
        23 Shellder
        23 Chinchou
    }

    Ernest => {
        "ERNEST"
        22 Machop
        24 Poliwhirl
        24 Quagsire
    }

    Jeff => {
        "JEFF"
        58 Raticate
        58 Furret
    }

    Garrett => {
        "GARRETT"
        64 Kingler
    }

    Kenneth => {
        "KENNETH"
        58 Machop
        58 Machoke
        58 Poliwrath
        58 Machamp
    }

    Stanly => {
        "STANLY"
        53 Qwilfish
        53 Machoke
        54 Golduck
    }

    Harry => {
        "HARRY"
        23 Quagsire
    }

    Huey2 => {
        "HUEY"
        28 Poliwhirl
        28 Poliwhirl
    }

    Huey3 => {
        "HUEY"
        34 Poliwhirl
        34 Poliwrath
    }

    Huey4 => {
        "HUEY"
        38 Politoed  [ Whirlpool, RainDance, BodySlam, PerishSong ]
        38 Poliwrath [ Surf, Strength, IcePunch, Submission ]
    }

    Stan => {
        "STAN"
        20 Grimer
    }

    Eric => {
        "ERIC"
        15 Grimer
        15 Cubone
    }

    Gregg => {
        "GREGG"
        20 Magnemite
        20 Magnemite
        20 Magnemite
    }

    Jay => {
        "JAY"
        22 Koffing
        22 Koffing
    }

    Dave => {
        "DAVE"
        24 Ditto
    }

    Sam => {
        "SAM"
        62 Porygon2
    }

    Tom => {
        "TOM"
        56 Magnemite
        56 Magnemite
        56 Steelix
    }

    Pat => {
        "PAT"
        56 Porygon
        56 Magneton
        56 Porygon2
    }

    Shawn => {
        "SHAWN"
        57 Arcanine
        58 Muk
        59 Magneton
    }

    Teru => {
        "TERU"
        14 Voltorb
        14 Magnemite
        14 Porygon
    }

    Russ => {
        "RUSS"
        27 Magnemite
        27 Magnemite
        27 Magnemite
    }

    Norton => {
        "NORTON"
        30 Porygon [ Conversion, Conversion2, Recover, TriAttack ]
    }

    Hugh => {
        "HUGH"
        39 Seadra [ Smokescreen, Twister, Surf, Waterfall ]
    }

    Markus => {
        "MARKUS"
        19 Slowpoke [ Curse, WaterGun, Growl, Strength ]
    }

    Rival21Chikorita => {
        "?"
        64 Ursaring + ScopeLens     [ Slash, FaintAttack, RockSmash, HyperBeam ]
        64 Crobat + Leftovers       [ Toxic, DoubleTeam, ConfuseRay, WingAttack ]
        64 Octillery + Nevermeltice [ Surf, IceBeam, Psybeam, HyperBeam ]
        64 Houndoom + Charcoal      [ FireBlast, IronTail, Crunch, SunnyDay ]
        64 Meganium + MiracleSeed   [ Reflect, GigaDrain, Solarbeam, BodySlam ]
        64 Tyranitar + QuickClaw    [ Crunch, Earthquake, RockSlide, FireBlast ]
    }

    Rival21Cyndaquil => {
        "?"
        64 Ursaring + PinkBow       [ Slash, FaintAttack, RockSmash, HyperBeam ]
        64 Crobat + Leftovers       [ Toxic, DoubleTeam, ConfuseRay, WingAttack ]
        64 Octillery + Nevermeltice [ Surf, IceBeam, Psybeam, HyperBeam ]
        64 Victreebel + ScopeLens   [ SludgeBomb, RazorLeaf, SleepPowder, Growth ]
        64 Typhlosion + Charcoal    [ FireBlast, Earthquake, IronTail, Thunderpunch ]
        64 Tyranitar + QuickClaw    [ Crunch, Earthquake, RockSlide, FireBlast ]
    }

    Rival21Totodile => {
        "?"
        64 Ursaring + PinkBow     [ Slash, FaintAttack, RockSmash, HyperBeam ]
        64 Crobat + Leftovers     [ Toxic, DoubleTeam, ConfuseRay, WingAttack ]
        64 Victreebel + ScopeLens [ SludgeBomb, RazorLeaf, SleepPowder, Growth ]
        64 Houndoom + Charcoal    [ FireBlast, IronTail, Crunch, DoubleTeam ]
        64 Feraligatr + ScopeLens [ IceBeam, Surf, Slash, Earthquake ]
        64 Tyranitar + QuickClaw  [ Crunch, Earthquake, RockSlide, FireBlast ]
    }

    Rival22Chikorita => {
        "?"
        68 Ursaring + ScopeLens     [ Slash, FaintAttack, RockSmash, HyperBeam ]
        68 Crobat + Leftovers       [ Toxic, DoubleTeam, ConfuseRay, Fly ]
        68 Octillery + Nevermeltice [ Surf, IceBeam, Psybeam, HyperBeam ]
        68 Houndoom + Charcoal      [ FireBlast, IronTail, Crunch, SunnyDay ]
        68 Meganium + MiracleSeed   [ Reflect, GigaDrain, Solarbeam, BodySlam ]
        68 Tyranitar + QuickClaw    [ Crunch, Earthquake, RockSlide, FireBlast ]
    }

    Rival22Cyndaquil => {
        "?"
        68 Ursaring + PinkBow       [ Slash, FaintAttack, RockSmash, HyperBeam ]
        68 Crobat + Leftovers       [ Toxic, DoubleTeam, ConfuseRay, Fly ]
        68 Octillery + Nevermeltice [ Surf, IceBeam, Psybeam, HyperBeam ]
        68 Victreebel + ScopeLens   [ SludgeBomb, RazorLeaf, SleepPowder, Growth ]
        68 Typhlosion + Charcoal    [ FireBlast, Earthquake, IronTail, Thunderpunch ]
        68 Tyranitar + QuickClaw    [ Crunch, Earthquake, RockSlide, FireBlast ]
    }

    Rival22Totodile => {
        "?"
        68 Ursaring + PinkBow     [ Slash, FaintAttack, RockSmash, HyperBeam ]
        68 Crobat + Leftovers     [ Toxic, DoubleTeam, ConfuseRay, Fly ]
        68 Victreebel + ScopeLens [ SludgeBomb, RazorLeaf, SleepPowder, Growth ]
        68 Houndoom + Charcoal    [ FireBlast, IronTail, Crunch, DoubleTeam ]
        68 Feraligatr + ScopeLens [ IceBeam, Surf, Slash, Earthquake ]
        68 Tyranitar + QuickClaw  [ Crunch, Earthquake, RockSlide, FireBlast ]
    }

    Clyde => {
        "CLYDE"
        61 Electabuzz
    }

    Vincent => {
        "VINCENT"
        57 Magneton  [ Thunderbolt, RainDance, TriAttack, Screech ]
        55 Electrode [ Screech, Thunder, Rollout, LightScreen ]
        57 Magneton  [ Thunder, RainDance, TriAttack, Screech ]
        55 Jolteon   [ Thunderbolt, PinMissile, DoubleKick, Thunder ]
    }

    Anthony1 => {
        "ANTHONY"
        25 Graveler
        25 Machoke
    }

    Russell => {
        "RUSSELL"
        9 Geodude
        8 Cubone
    }

    Phillip => {
        "PHILLIP"
        23 Geodude
        23 Geodude
        23 Graveler
    }

    Leonard => {
        "LEONARD"
        23 Geodude
        25 Machop
    }

    Anthony2 => {
        "ANTHONY"
        11 Geodude
        11 Machop
    }

    Benjamin => {
        "BENJAMIN"
        24 Graveler
        24 Dugtrio
    }

    Erik => {
        "ERIK"
        37 Machoke
        38 Golem
        37 Kangaskhan
    }

    Michael => {
        "MICHAEL"
        38 Rhyhorn
        38 Donphan
        38 Golem
    }

    Parry1 => {
        "PARRY"
        45 Piloswine [ Earthquake, Blizzard, Rest, TakeDown ]
        45 Dugtrio   [ Magnitude, Dig, MudSlap, Slash ]
        48 Steelix   [ Dig, IronTail, Sandstorm, Slam ]
    }

    Timothy => {
        "TIMOTHY"
        38 Dugtrio  [ Magnitude, Dig, SandAttack, Slash ]
        39 Gligar   [ Slash, FaintAttack, WingAttack, PoisonSting ]
        38 Graveler [ Magnitude, RockThrow, Rollout ]
        38 Dugtrio  [ Magnitude, Dig, SandAttack, Slash ]
    }

    Bailey => {
        "BAILEY"
        40 Golem
        40 Golem
        40 Golem
        40 Golem
        40 Golem
    }

    Anthony3 => {
        "ANTHONY"
        35 Graveler
        34 Machoke
        35 Graveler
    }

    Tim => {
        "TIM"
        55 Golem
        55 Kabutops
        55 Quagsire
    }

    Noland => {
        "NOLAND"
        55 Sandslash
        55 Golem
    }

    Sidney => {
        "SIDNEY"
        56 Dugtrio
        56 Steelix
    }

    Kenny => {
        "KENNY"
        56 Sandslash
        59 Graveler
        57 Golem
        59 Graveler
    }

    Jim => {
        "JIM"
        58 Machamp
    }

    Daniel => {
        "DANIEL"
        11 Onix
    }

    Parry2 => {
        "PARRY"
        50 Piloswine [ Earthquake, Blizzard, Rest, TakeDown ]
        50 Dugtrio   [ Magnitude, Dig, MudSlap, Slash ]
        50 Steelix   [ Dig, IronTail, Sandstorm, Slam ]
    }

    Parry3 => {
        "PARRY"
        38 Piloswine
        38 Dugtrio
        38 Steelix
    }

    Anthony4 => {
        "ANTHONY"
        48 Golem
        50 Machamp
        48 Golem
    }

    Anthony5 => {
        "ANTHONY"
        56 Golem   [ Earthquake, Explosion, RockSlide, Rollout ]
        54 Machamp [ CrossChop, VitalThrow, Headbutt, Dig ]
        56 Golem   [ Earthquake, Explosion, RockSlide, Rollout ]
    }

    BikerBenny => {
        "BENNY"
        20 Koffing
        20 Koffing
        20 Koffing
    }

    Kazu => {
        "KAZU"
        20 Koffing
        20 Koffing
        20 Koffing
    }

    Dwayne => {
        "DWAYNE"
        57 Koffing
        58 Koffing
        59 Koffing
        60 Koffing
    }

    Harris => {
        "HARRIS"
        59 Flareon
    }

    Zeke => {
        "ZEKE"
        58 Crobat
        56 Muk
    }

    Charles => {
        "CHARLES"
        56 Poliwrath
        56 Charizard
        56 Weezing
    }

    Riley => {
        "RILEY"
        60 Weezing
    }

    Joel => {
        "JOEL"
        58 Magmar
        58 Electabuzz
    }

    Glenn => {
        "GLENN"
        57 Koffing
        55 Magmar
        58 Weezing
    }

    Blaine1 => {
        "BLAINE"
        66 Rapidash + QuickClaw   [ DoubleEdge, SunnyDay, FireBlast, Solarbeam ]
        65 Magmar + GoldBerry     [ Thunderpunch, FireBlast, Psychic, ConfuseRay ]
        66 Houndoom + MiracleSeed [ FireBlast, SunnyDay, Solarbeam, Crunch ]
        65 Ninetales + Leftovers  [ FireBlast, ShadowBall, SunnyDay, Hypnosis ]
        67 Moltres + SharpBeak    [ FireBlast, SkyAttack, Solarbeam, SunnyDay ]
        67 Arcanine + Charcoal    [ Crunch, Dragonbreath, FireBlast, Extremespeed ]
    }

    Duncan => {
        "DUNCAN"
        35 Delibird
        35 Magmar
    }

    Eddie => {
        "EDDIE"
        35 Arcanine [ Roar, Ember, Leer, TakeDown ]
        35 Weezing  [ Tackle, Smog, Sludge, Smokescreen ]
    }

    Corey => {
        "COREY"
        55 Koffing
        58 Magmar
        55 Koffing
        60 Koffing
    }

    Otis => {
        "OTIS"
        58 Magmar
        60 Weezing
        58 Magmar
    }

    Dick => {
        "DICK"
        17 Charmeleon
    }

    Ned => {
        "NED"
        15 Koffing
        16 Growlithe
        15 Koffing
    }

    Burt => {
        "BURT"
        60 Weezing
        60 Magcargo
    }

    Bill => {
        "BILL"
        11 Growlithe
    }

    Walt => {
        "WALT"
        15 Magmar
        15 Magmar
    }

    Ray => {
        "RAY"
        9 Vulpix
    }

    Lyle => {
        "LYLE"
        54 Weezing
        54 Flareon
        54 Ninetales
    }

    Irwin1 => {
        "IRWIN"
        16 Voltorb
        16 Pineco
        16 Voltorb
    }

    Fritz => {
        "FRITZ"
        54 MrMime
        54 Magmar
        56 Machoke
    }

    Horton => {
        "HORTON"
        56 Electrode
        57 Electrode
        56 Electrode
        58 Electrode
    }

    Irwin2 => {
        "IRWIN"
        26 Voltorb
        26 Pineco
        26 Voltorb
    }

    Irwin3 => {
        "IRWIN"
        34 Electrode
        31 Forretress
        33 Electrode
        35 Electrode
    }

    Irwin4 => {
        "IRWIN"
        18 Voltorb
        22 Voltorb
        26 Voltorb
        30 Electrode
    }

    Kenji1 => {
        "KENJI"
        33 Onix      [ Bind, RockThrow, Toxic, Dig ]
        38 Machamp   [ Headbutt, Swagger, Foresight, VitalThrow ]
        33 Steelix   [ Earthquake, RockThrow, IronTail, Sandstorm ]
        36 Hitmonlee [ DoubleTeam, HiJumpKick, MudSlap, Swift ]
    }

    Yoshi => {
        "YOSHI"
        29 Hitmonlee [ DoubleKick, Meditate, JumpKick, FocusEnergy ]
    }

    Kenji2 => {
        "KENJI"
        33 Onix      [ Bind, RockThrow, Toxic, Dig ]
        38 Machamp   [ Headbutt, Swagger, Foresight, VitalThrow ]
        33 Steelix   [ Earthquake, RockThrow, IronTail, Sandstorm ]
        36 Hitmonlee [ DoubleTeam, HiJumpKick, MudSlap, Swift ]
    }

    Lao => {
        "LAO"
        29 Hitmonchan [ MachPunch, Thunderpunch, IcePunch, CometPunch ]
    }

    Nob => {
        "NOB"
        27 Machop  [ Leer, FocusEnergy, KarateChop, SeismicToss ]
        27 Machoke [ Leer, KarateChop, SeismicToss, RockSlide ]
    }

    Kiyo => {
        "KIYO"
        24 Hitmonlee  [ RollingKick, JumpKick, Meditate, Strength ]
        24 Hitmonchan [ MachPunch, Pursuit, DizzyPunch, Thunderpunch ]
    }

    Lung => {
        "LUNG"
        27 Mankey
        27 Machoke
        27 Primeape
    }

    Kenji3 => {
        "KENJI"
        33 Onix      [ Bind, RockThrow, Toxic, Dig ]
        38 Machamp   [ Headbutt, Swagger, Foresight, VitalThrow ]
        33 Steelix   [ Earthquake, RockThrow, IronTail, Sandstorm ]
        36 Hitmonlee [ DoubleTeam, HiJumpKick, MudSlap, Swift ]
    }

    Wai => {
        "WAI"
        56 Machamp
        58 Machoke
        60 Machamp
    }

    ExecutiveM1 => {
        "ARCHER"
        39 Weezing  [ SludgeBomb, Selfdestruct, Smog, Smokescreen ]
        38 Tauros   [ Frustration, Headbutt, RockSmash, Pursuit ]
        39 Gyarados [ Surf, Bite, Strength, Gust ]
        38 Houndoom [ Flamethrower, Bite, Smog, Roar ]
        40 Slowbro  [ Surf, Confusion, Curse, Amnesia ]
    }

    ExecutiveM2 => {
        "EXECUTIVE"
        38 Forretress [ SpikeCannon, PinMissile, Explosion, Spikes ]
        38 Pupitar    [ Thrash, RockSlide, Screech, Bite ]
        39 Magcargo   [ Curse, Smog, Flamethrower, RockSlide ]
        38 Cloyster   [ Surf, IceBeam, Clamp ]
    }

    ExecutiveM3 => {
        "EXECUTIVE"
        36 Muk       [ Minimize, SludgeBomb, Pound, AcidArmor ]
        37 Nidoqueen [ BodySlam, SludgeBomb, DoubleKick, Thunderpunch ]
        36 Weezing   [ Tackle, SludgeBomb, Selfdestruct, Haze ]
        37 Nidoking  [ Thrash, SludgeBomb, DoubleKick, IcePunch ]
        38 Rhydon    [ RockSlide, Earthquake, Surf ]
    }

    ExecutiveM4 => {
        "ARCHER"
        30 Weezing  [ Sludge, Smokescreen, Tackle, Toxic ]
        29 Tauros   [ Rage, Frustration, HornAttack, Pursuit ]
        30 Houndoom [ FlameWheel, Bite, Smog, Roar ]
        30 Slowbro  [ Headbutt, Curse, Confusion, Surf ]
    }

    Nathan => {
        "NATHAN"
        26 Girafarig
    }

    Franklin => {
        "FRANKLIN"
        60 Alakazam [ Psychic, Recover, ShadowBall, Thunderpunch ]
    }

    Herman => {
        "HERMAN"
        55 Exeggutor
        55 Xatu
        55 Starmie
    }

    Fidel => {
        "FIDEL"
        54 Xatu
    }

    Greg => {
        "GREG"
        22 Stantler [ Nightmare, Hypnosis, Leer, Headbutt ]
    }

    Norman => {
        "NORMAN"
        22 Slowpoke [ Tackle, Growl, WaterGun, Confusion ]
        23 Slowpoke [ Curse, Amnesia, WaterGun, Confusion ]
    }

    Mark => {
        "MARK"
        15 Abra    [ Teleport, Flash ]
        15 Abra    [ Teleport, Flash ]
        16 Kadabra [ Teleport, Kinesis, Confusion ]
    }

    Phil => {
        "PHIL"
        36 Xatu      [ Psychic, NightShade, FutureSight, ConfuseRay ]
        36 Kadabra   [ Disable, Psybeam, Recover ]
        36 Girafarig [ Psybeam, Stomp ]
    }

    Richard => {
        "RICHARD"
        45 Espeon
        45 MrMime
        45 Slowking
    }

    Gilbert => {
        "GILBERT"
        42 Xatu
        44 Exeggutor
        44 Girafarig
    }

    Jared => {
        "JARED"
        58 Unown
        58 MrMime
        58 Exeggutor
    }

    Rodney => {
        "RODNEY"
        59 Drowzee
        63 Hypno
    }

    Liz1 => {
        "LIZ"
        9 NidoranF
    }

    Gina1 => {
        "GINA"
        14 Skiploom
        14 Bulbasaur
    }

    Brooke => {
        "BROOKE"
        18 Pikachu [ Thundershock, Growl, QuickAttack, DoubleTeam ]
    }

    Kim => {
        "KIM"
        18 Vulpix
    }

    Cindy => {
        "CINDY"
        57 Tentacruel
        60 Nidoqueen
    }

    Hope => {
        "HOPE"
        61 Ampharos
    }

    Sharon => {
        "SHARON"
        61 Furret
        58 Rapidash
    }

    Debra => {
        "DEBRA"
        58 Seaking
    }

    Gina2 => {
        "GINA"
        25 Skiploom
        25 Skiploom
        27 Ivysaur
    }

    Erin1 => {
        "ERIN"
        40 Rapidash
        38 Nidoqueen
        38 Raichu
        40 Rapidash
    }

    Liz2 => {
        "LIZ"
        24 Weepinbell
        25 Nidorina
    }

    Liz3 => {
        "LIZ"
        29 Weepinbell
        29 Nidorino
        30 Nidoqueen
    }

    Heidi => {
        "HEIDI"
        56 Jumpluff
        58 Seadra
    }

    Edna => {
        "EDNA"
        56 Nidoqueen
        56 Raichu
    }

    Gina3 => {
        "GINA"
        36 Jumpluff
        36 Jumpluff
        35 Ivysaur
    }

    Tiffany1 => {
        "TIFFANY"
        34 Clefable   [ Encore, Sing, Doubleslap, Minimize ]
        34 Wigglytuff [ Sing, Doubleslap, DefenseCurl, Headbutt ]
    }

    Tiffany2 => {
        "TIFFANY"
        37 Clefairy [ Encore, Doubleslap, Minimize, Metronome ]
    }

    Erin2 => {
        "ERIN"
        44 Rapidash
        42 Nidoqueen
        42 Raichu
        44 Rapidash
    }

    Tanya => {
        "TANYA"
        57 Bellossom
        57 Exeggutor
        57 Sunflora
    }

    Tiffany3 => {
        "TIFFANY"
        29 Clefable [ Encore, Sing, Doubleslap, Minimize ]
    }

    Erin3 => {
        "ERIN"
        44 Rapidash  [ DoubleTeam, Stomp, Flamethrower, SunnyDay ]
        42 Nidoqueen [ BodySlam, Surf, Earthquake, SludgeBomb ]
        40 Raichu    [ Swift, MudSlap, QuickAttack, Thunderbolt ]
        44 Rapidash  [ DoubleTeam, Stomp, Flamethrower, SunnyDay ]
    }

    Liz4 => {
        "LIZ"
        34 Victreebel
        36 Nidoking
        36 Nidoqueen
    }

    Liz5 => {
        "LIZ"
        50 Victreebel [ SleepPowder, Poisonpowder, StunSpore, SludgeBomb ]
        52 Nidoking   [ Earthquake, DoubleKick, SludgeBomb, IronTail ]
        53 Nidoqueen  [ Earthquake, DoubleKick, SludgeBomb, BodySlam ]
    }

    Gina4 => {
        "GINA"
        45 Skiploom
        45 Skiploom
        48 Venusaur
    }

    Gina5 => {
        "GINA"
        52 Jumpluff [ StunSpore, SunnyDay, LeechSeed, GigaDrain ]
        52 Jumpluff [ SunnyDay, SleepPowder, LeechSeed, GigaDrain ]
        55 Venusaur [ Solarbeam, RazorLeaf, BodySlam, MudSlap ]
    }

    Tiffany4 => {
        "TIFFANY"
        53 Clefable   [ DoubleEdge, Encore, Moonlight, Minimize ]
        54 Wigglytuff [ Sing, Doubleslap, DefenseCurl, Headbutt ]
    }

    Roland => {
        "ROLAND"
        9 NidoranM
    }

    Todd1 => {
        "TODD"
        15 Azumarill
    }

    Ivan => {
        "IVAN"
        16 Eevee
        16 Diglett
    }

    Elliot => {
        "ELLIOT"
        16 Sandshrew
        16 Marill
    }

    Barry => {
        "BARRY"
        57 Tentacruel
        60 Nidoking
    }

    Lloyd => {
        "LLOYD"
        57 Nidoking
    }

    Dean => {
        "DEAN"
        56 Heracross
        57 Kangaskhan
    }

    Sid => {
        "SID"
        56 Dugtrio
        55 Primeape
        56 Poliwrath
    }

    Harvey => {
        "HARVEY"
        15 Nidorino
    }

    Dale => {
        "DALE"
        15 Nidorino
    }

    Ted => {
        "TED"
        40 Ursaring
        40 Primeape
        40 Nidoking
    }

    Todd2 => {
        "TODD"
        25 Azumarill
        22 Geodude
        24 Psyduck
    }

    Todd3 => {
        "TODD"
        36 Azumarill
        35 Graveler
        35 Golduck
    }

    Thomas => {
        "THOMAS"
        33 Graveler
        36 Graveler
        40 Golbat
        42 Golduck
    }

    Leroy => {
        "LEROY"
        33 Graveler
        36 Graveler
        40 Golbat
        42 Golduck
    }

    David => {
        "DAVID"
        33 Graveler
        36 Graveler
        40 Golbat
        42 Golduck
    }

    John => {
        "JOHN"
        33 Graveler
        36 Graveler
        40 Golbat
        42 Golduck
    }

    Jerry => {
        "JERRY"
        60 Dugtrio
        58 Sandslash
        60 Donphan
    }

    Spencer => {
        "SPENCER"
        25 Sandslash
        25 Golbat
    }

    Todd4 => {
        "TODD"
        46 Azumarill
        45 Golem
        46 Golduck
        45 Magcargo
    }

    Todd5 => {
        "TODD"
        56 Azumarill
        55 Golem
        56 Golduck
        55 Magcargo
    }

    Quentin => {
        "QUENTIN"
        38 Fearow
        38 Primeape
        38 Tauros
        38 Raichu
    }

    ExecutiveF1 => {
        "ARIANA"
        39 Arbok     [ SludgeBomb, Screech, Bite, Glare ]
        40 Persian   [ Slash, Bite, Screech, Charm ]
        39 Vileplume [ GigaDrain, SleepPowder, SludgeBomb ]
        40 Gyarados  [ Surf, Strength, Twister, Gust ]
        40 Murkrow   [ Fly, Pursuit, Toxic, FaintAttack ]
    }

    ExecutiveF2 => {
        "ARIANA"
        31 Arbok     [ Wrap, Leer, SludgeBomb, Bite ]
        30 Persian   [ Slash, FaintAttack, PayDay, MudSlap ]
        30 Vileplume [ GigaDrain, SweetScent, SleepPowder, SludgeBomb ]
        31 Murkrow   [ Fly, Pursuit, Toxic, FaintAttack ]
    }

    Chow => {
        "CHOW"
        3 Bellsprout
        3 Sunkern
        3 Bellsprout
    }

    Nico => {
        "NICO"
        3 Bellsprout
        4 Hoppip
        3 Bellsprout
    }

    Jin => {
        "JIN"
        6 Bellsprout
    }

    Troy => {
        "TROY"
        7 Hoppip
        7 Bellsprout
    }

    Jeffrey => {
        "JEFFREY"
        21 Gastly
        21 Haunter
        21 Gastly
    }

    Ping => {
        "PING"
        22 Misdreavus
    }

    Edmond => {
        "EDMOND"
        3 Bellsprout
        3 Bellsprout
        3 Bellsprout
    }

    Neal => {
        "NEAL"
        6 Bellsprout
    }

    Li => {
        "LI"
        7 Oddish
        8 Sunkern
        9 Bellsprout
    }

    Gaku => {
        "GAKU"
        40 Noctowl
        40 Flareon
        40 Victreebel
    }

    Masa => {
        "MASA"
        40 Noctowl
        40 Jolteon
        40 Victreebel
    }

    Koji => {
        "KOJI"
        40 Noctowl
        40 Vaporeon
        40 Victreebel
    }

    Martha => {
        "MARTHA"
        20 Haunter
        20 Houndour
    }

    Grace => {
        "GRACE"
        20 Haunter
        20 Haunter
    }

    Bethany => {
        "BETHANY"
        25 Haunter
    }

    Margret => {
        "MARGRET"
        25 Haunter
    }

    Ethel => {
        "ETHEL"
        25 Haunter
    }

    Rebecca => {
        "REBECCA"
        58 Hypno
        58 Jynx
        58 Hypno
    }

    Doris => {
        "DORIS"
        58 Noctowl
        59 Slowbro
        58 Xatu
    }

    Ronald => {
        "RONALD"
        29 Dewgong
        30 Delibird
    }

    Brad => {
        "BRAD"
        30 Swinub
        30 Sneasel
    }

    Douglas => {
        "DOUGLAS"
        28 Shellder
        28 Seel
        30 Cloyster
    }

    William => {
        "WILLIAM"
        15 Raichu + Berry
    }

    Derek1 => {
        "DEREK"
        22 Pikachu + Berry
        22 Ponyta + Berry
    }

    Robert => {
        "ROBERT"
        60 Kangaskhan + Berry
    }

    Joshua => {
        "JOSHUA"
        60 Pikachu + Berry
        60 Pikachu + Berry
        60 Pikachu + Berry
    }

    Carter => {
        "CARTER"
        65 Chikorita + Berry
        65 Cyndaquil + Berry
        65 Totodile + Berry
    }

    Trevor => {
        "TREVOR"
        60 Scizor + Berry
    }

    Brandon => {
        "BRANDON"
        15 Snubbull + Berry
    }

    Jeremy => {
        "JEREMY"
        58 Meowth + Berry
        60 Persian + Berry
        58 Meowth + Berry
    }

    Colin => {
        "COLIN"
        56 Delibird + Berry
    }

    Derek2 => {
        "DEREK"
        22 Pikachu + Berry
        22 Ponyta + Berry
    }

    Derek3 => {
        "DEREK"
        36 Pikachu + Berry
        36 Ponyta + Berry
    }

    Alex => {
        "ALEX"
        56 Nidoking + Berry
        57 Slowking + Berry
        58 Seaking + Berry
    }

    Rex => {
        "REX"
        55 Stantler + GoldBerry
    }

    Allan => {
        "ALLAN"
        55 Granbull + GoldBerry
    }

    NaokoUnused => {
        "NAOKO"
        20 Skiploom
        20 Vulpix
        18 Skiploom
    }

    Naoko => {
        "NAOKO"
        21 Flareon
    }

    Sayo => {
        "SAYO"
        21 Espeon
    }

    Zuki => {
        "ZUKI"
        21 Umbreon
    }

    Kuni => {
        "KUNI"
        21 Vaporeon
    }

    Miki => {
        "MIKI"
        21 Jolteon
    }

    AmyAndMay1 => {
        "AMY & MAY"
        12 Ledyba
        12 Spinarak
    }

    AnnAndAnne1 => {
        "ANN & ANNE"
        18 Clefairy [ Growl, Encore, Doubleslap, Metronome ]
        18 Furret   [ QuickAttack, DefenseCurl, FurySwipes ]
    }

    AnnAndAnne2 => {
        "ANN & ANNE"
        18 Furret   [ QuickAttack, DefenseCurl, FurySwipes ]
        18 Clefairy [ Growl, Encore, Doubleslap, Metronome ]
    }

    AmyAndMay2 => {
        "AMY & MAY"
        12 Spinarak
        12 Ledyba
    }

    JoAndZoe1 => {
        "JO & ZOE"
        58 Victreebel
        58 Vileplume
    }

    JoAndZoe2 => {
        "JO & ZOE"
        58 Vileplume
        58 Victreebel
    }

    MegAndPeg1 => {
        "MEG & PEG"
        54 Ursaring
        54 Donphan
    }

    MegAndPeg2 => {
        "MEG & PEG"
        54 Donphan
        54 Ursaring
    }

    LeaAndPia1 => {
        "LEA & PIA"
        41 Dragonair [ ThunderWave, Twister, Flamethrower, Dragonbreath ]
        42 Gyarados  [ Surf, Strength, Fly, Twister ]
        41 Dragonair [ ThunderWave, Twister, IceBeam, Dragonbreath ]
        42 Gyarados  [ Surf, Strength, Fly, Twister ]
    }

    LeaAndPia2 => {
        "LEA & PIA"
        41 Dragonair [ ThunderWave, Twister, IceBeam, Dragonbreath ]
        42 Gyarados  [ Surf, Strength, Fly, Twister ]
        41 Dragonair [ ThunderWave, Twister, Flamethrower, Dragonbreath ]
        42 Gyarados  [ Surf, Strength, Fly, Twister ]
    }

    Beverly1 => {
        "BEVERLY"
        20 Snubbull + Berry
    }

    Ruth => {
        "RUTH"
        23 Pikachu + Berry
    }

    Beverly2 => {
        "BEVERLY"
        18 Snubbull + Berry
    }

    Beverly3 => {
        "BEVERLY"
        30 Granbull + Berry
    }

    Georgia => {
        "GEORGIA"
        61 Sentret + Berry
        61 Sentret + Berry
        61 Sentret + Berry
        63 Furret + Berry
        61 Sentret + Berry
    }

    Jaime => {
        "JAIME"
        20 Umbreon + Berry
    }

    Red1 => {
        "RED"
        93 Pikachu + LightBall    [ Thunderbolt, Surf, IronTail, DoubleTeam ]
        75 Snorlax + Leftovers    [ Amnesia, Curse, BodySlam, Earthquake ]
        77 Charizard + Charcoal   [ FireBlast, WingAttack, Outrage, SteelWing ]
        77 Venusaur + MiracleSeed [ GigaDrain, BodySlam, SleepPowder, LeechSeed ]
        77 Blastoise + QuickClaw  [ IceBeam, HydroPump, BodySlam, Earthquake ]
        75 Mewtwo + Miracleberry  [ Recover, Submission, Flamethrower, Psychic ]
    }

    Red2 => {
        "RED"
        93 Pikachu + LightBall    [ Thunderbolt, Surf, IronTail, DoubleTeam ]
        75 Snorlax + Leftovers    [ Amnesia, Curse, BodySlam, Earthquake ]
        77 Charizard + Charcoal   [ FireBlast, WingAttack, Outrage, SteelWing ]
        77 Venusaur + MiracleSeed [ GigaDrain, BodySlam, SleepPowder, LeechSeed ]
        77 Blastoise + QuickClaw  [ IceBeam, HydroPump, BodySlam, Earthquake ]
        80 Espeon + Miracleberry  [ Psychic, ShadowBall, HiddenPower, MorningSun ]
    }

    Blue1 => {
        "BLUE"
        69 Articuno + Miracleberry [ IceBeam, SkyAttack, Rest, Toxic ]
        68 Alakazam + Twistedspoon [ Thunderpunch, Recover, Psychic, ShadowBall ]
        67 Machamp + ScopeLens     [ CrossChop, RockSlide, Earthquake, BodySlam ]
        68 Exeggutor + Leftovers   [ LeechSeed, SleepPowder, Psychic, GigaDrain ]
        68 Gyarados + FocusBand    [ HydroPump, DoubleTeam, BodySlam, Reversal ]
        69 Arcanine + PinkBow      [ Flamethrower, Curse, Crunch, Extremespeed ]
    }

    Blue2 => {
        "BLUE"
        69 Articuno + Miracleberry [ IceBeam, SkyAttack, Rest, Toxic ]
        68 Alakazam + Twistedspoon [ Thunderbolt, Recover, Psychic, ShadowBall ]
        67 Rhydon + QuickClaw      [ RockSlide, Earthquake, IronTail, Crunch ]
        68 Exeggutor + Leftovers   [ LeechSeed, SleepPowder, Psychic, GigaDrain ]
        68 Gyarados + FocusBand    [ HydroPump, DoubleTeam, BodySlam, Reversal ]
        69 Arcanine + PinkBow      [ Flamethrower, Curse, Crunch, Extremespeed ]
    }

    Keith => {
        "KEITH"
        16 Growlithe
    }

    Dirk => {
        "DIRK"
        16 Growlithe
        16 Houndour
    }

    GruntF1 => {
        "GRUNT"
         9 Zubat
        11 Ekans
    }

    GruntF2 => {
        "GRUNT"
        35 Arbok
        35 Gloom
    }

    GruntF3 => {
        "GRUNT"
        35 Vileplume
        35 Pinsir
        35 Arbok
        36 Murkrow
    }

    GruntF4 => {
        "GRUNT"
        36 Arbok
        35 Gloom
        36 Ariados
        36 Vileplume
    }

    GruntF5 => {
        "GRUNT"
        28 Arbok
        28 Sneasel
    }

    Eusine => {
        "EUSINE"
        27 Politoed [ IceBeam, Bubblebeam, RainDance, Hypnosis ]
        27 Hypno    [ DreamEater, Hypnosis, Disable, Confusion ]
        27 Flaaffy  [ Thunderpunch, ThunderWave, Thundershock, BodySlam ]
    }

    Giovanni => {
        "GIOVANNI"
        40 Kangaskhan [ MegaPunch, DizzyPunch, FirePunch, Safeguard ]
        41 Nidoqueen  [ BodySlam, IcePunch, SludgeBomb, Bubblebeam ]
        41 Persian    [ Slash, FaintAttack, Swagger, Screech ]
        40 Steelix    [ IronTail, RockThrow, Sandstorm, Dig ]
        42 Nidoking   [ Thrash, Thunderpunch, Earthquake, RockSlide ]
    }

    Archer1 => {
        "ARCHER"
        39 Weezing  [ SludgeBomb, Selfdestruct, Smog, Smokescreen ]
        38 Tauros   [ Frustration, Headbutt, RockSmash, Pursuit ]
        39 Gyarados [ Surf, Bite, Strength, Gust ]
        38 Houndoom [ Flamethrower, Bite, Smog, Roar ]
        40 Slowbro  [ Surf, Confusion, Curse, Amnesia ]
    }

    Archer2 => {
        "ARCHER"
        30 Weezing  [ Sludge, Smokescreen, Tackle, Toxic ]
        29 Tauros   [ Rage, Frustration, HornAttack, Pursuit ]
        30 Houndoom [ FlameWheel, Bite, Smog, Roar ]
        30 Slowbro  [ Headbutt, Curse, Confusion, Surf ]
    }

    Weebra => {
        "WEEBRA"
        70 Forretress + QuickClaw  [ Toxic, Rollout, Sandstorm, Protect ]
        70 Quagsire + QuickClaw    [ IcePunch, SludgeBomb, Surf, Earthquake ]
        70 Aerodactyl + ScopeLens  [ SkyAttack, RockSlide, Flamethrower, IronTail ]
        70 Snorlax + Leftovers     [ Rest, Earthquake, Curse, SleepTalk ]
        70 Celebi + Miracleberry   [ Psychic, LeechSeed, GigaDrain, ShadowBall ]
        70 Hitmontop + BerserkGene [ TripleKick, Thief, HiddenPower, Dig ]
    }
}
