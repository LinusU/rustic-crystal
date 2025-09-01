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
        7 Pidgey    [ Tackle, MudSlap ]
        9 Pidgeotto [ Tackle, MudSlap, Gust ]
    }

    Whitney1 => {
        "WHITNEY"
        18 Clefairy [ Doubleslap, Mimic, Encore, Metronome ]
        20 Miltank  [ Rollout, Attract, Stomp, MilkDrink ]
    }

    Bugsy1 => {
        "BUGSY"
        14 Metapod [ Tackle, StringShot, Harden ]
        14 Kakuna  [ PoisonSting, StringShot, Harden ]
        16 Scyther [ QuickAttack, Leer, FuryCutter ]
    }

    Morty1 => {
        "MORTY"
        21 Gastly  [ Lick, Spite, MeanLook, Curse ]
        21 Haunter [ Hypnosis, Mimic, Curse, NightShade ]
        25 Gengar  [ Hypnosis, ShadowBall, MeanLook, DreamEater ]
        23 Haunter [ Spite, MeanLook, Mimic, NightShade ]
    }

    Pryce1 => {
        "PRYCE"
        27 Seel      [ Headbutt, IcyWind, AuroraBeam, Rest ]
        29 Dewgong   [ Headbutt, IcyWind, AuroraBeam, Rest ]
        31 Piloswine [ IcyWind, FuryAttack, Mist, Blizzard ]
    }

    Jasmine1 => {
        "JASMINE"
        30 Magnemite [ Thunderbolt, Supersonic, Sonicboom, ThunderWave ]
        30 Magnemite [ Thunderbolt, Supersonic, Sonicboom, ThunderWave ]
        35 Steelix   [ Screech, SunnyDay, RockThrow, IronTail ]
    }

    Chuck1 => {
        "CHUCK"
        27 Primeape  [ Leer, Rage, KarateChop, FurySwipes ]
        30 Poliwrath [ Hypnosis, MindReader, Surf, Dynamicpunch ]
    }

    Clair1 => {
        "CLAIR"
        37 Dragonair [ ThunderWave, Surf, Slam, Dragonbreath ]
        37 Dragonair [ ThunderWave, Thunderbolt, Slam, Dragonbreath ]
        37 Dragonair [ ThunderWave, IceBeam, Slam, Dragonbreath ]
        40 Kingdra   [ Smokescreen, Surf, HyperBeam, Dragonbreath ]
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
        12 Gastly
        14 Zubat
        16 Bayleef
    }

    Rival12Cyndaquil => {
        "?"
        12 Gastly
        14 Zubat
        16 Quilava
    }

    Rival12Totodile => {
        "?"
        12 Gastly
        14 Zubat
        16 Croconaw
    }

    Rival13Chikorita => {
        "?"
        20 Haunter   [ Lick, Spite, MeanLook, Curse ]
        18 Magnemite [ Tackle, Thundershock, Supersonic, Sonicboom ]
        20 Zubat     [ LeechLife, Supersonic, Bite, ConfuseRay ]
        22 Bayleef   [ Growl, Reflect, RazorLeaf, Poisonpowder ]
    }

    Rival13Cyndaquil => {
        "?"
        20 Haunter   [ Lick, Spite, MeanLook, Curse ]
        18 Magnemite [ Tackle, Thundershock, Supersonic, Sonicboom ]
        20 Zubat     [ LeechLife, Supersonic, Bite, ConfuseRay ]
        22 Quilava   [ Leer, Smokescreen, Ember, QuickAttack ]
    }

    Rival13Totodile => {
        "?"
        20 Haunter   [ Lick, Spite, MeanLook, Curse ]
        18 Magnemite [ Tackle, Thundershock, Supersonic, Sonicboom ]
        20 Zubat     [ LeechLife, Supersonic, Bite, ConfuseRay ]
        22 Croconaw  [ Leer, Rage, WaterGun, Bite ]
    }

    Rival14Chikorita => {
        "?"
        30 Golbat    [ LeechLife, Bite, ConfuseRay, WingAttack ]
        28 Magnemite [ Tackle, Thundershock, Sonicboom, ThunderWave ]
        30 Haunter   [ Lick, MeanLook, Curse, ShadowBall ]
        32 Sneasel   [ Leer, QuickAttack, Screech, FaintAttack ]
        32 Meganium  [ Reflect, RazorLeaf, Poisonpowder, BodySlam ]
    }

    Rival14Cyndaquil => {
        "?"
        30 Golbat    [ LeechLife, Bite, ConfuseRay, WingAttack ]
        28 Magnemite [ Tackle, Thundershock, Sonicboom, ThunderWave ]
        30 Haunter   [ Lick, MeanLook, Curse, ShadowBall ]
        32 Sneasel   [ Leer, QuickAttack, Screech, FaintAttack ]
        32 Quilava   [ Smokescreen, Ember, QuickAttack, FlameWheel ]
    }

    Rival14Totodile => {
        "?"
        30 Golbat     [ LeechLife, Bite, ConfuseRay, WingAttack ]
        28 Magnemite  [ Tackle, Thundershock, Sonicboom, ThunderWave ]
        30 Haunter    [ Lick, MeanLook, Curse, ShadowBall ]
        32 Sneasel    [ Leer, QuickAttack, Screech, FaintAttack ]
        32 Feraligatr [ Rage, WaterGun, Bite, ScaryFace ]
    }

    Rival15Chikorita => {
        "?"
        34 Sneasel  [ QuickAttack, Screech, FaintAttack, FuryCutter ]
        36 Golbat   [ LeechLife, Bite, ConfuseRay, WingAttack ]
        35 Magneton [ Thundershock, Sonicboom, ThunderWave, Swift ]
        35 Haunter  [ MeanLook, Curse, ShadowBall, ConfuseRay ]
        35 Kadabra  [ Disable, Psybeam, Recover, FutureSight ]
        38 Meganium [ Reflect, RazorLeaf, Poisonpowder, BodySlam ]
    }

    Rival15Cyndaquil => {
        "?"
        34 Sneasel    [ QuickAttack, Screech, FaintAttack, FuryCutter ]
        36 Golbat     [ LeechLife, Bite, ConfuseRay, WingAttack ]
        35 Magneton   [ Thundershock, Sonicboom, ThunderWave, Swift ]
        35 Haunter    [ MeanLook, Curse, ShadowBall, ConfuseRay ]
        35 Kadabra    [ Disable, Psybeam, Recover, FutureSight ]
        38 Typhlosion [ Smokescreen, Ember, QuickAttack, FlameWheel ]
    }

    Rival15Totodile => {
        "?"
        34 Sneasel    [ QuickAttack, Screech, FaintAttack, FuryCutter ]
        36 Golbat     [ LeechLife, Bite, ConfuseRay, WingAttack ]
        34 Magneton   [ Thundershock, Sonicboom, ThunderWave, Swift ]
        35 Haunter    [ MeanLook, Curse, ShadowBall, ConfuseRay ]
        35 Kadabra    [ Disable, Psybeam, Recover, FutureSight ]
        38 Feraligatr [ Rage, WaterGun, ScaryFace, Slash ]
    }

    Will1 => {
        "WILL"
        40 Xatu      [ QuickAttack, FutureSight, ConfuseRay, PsychicM ]
        41 Jynx      [ Doubleslap, LovelyKiss, IcePunch, PsychicM ]
        41 Exeggutor [ Reflect, LeechSeed, EggBomb, PsychicM ]
        41 Slowbro   [ Curse, Amnesia, BodySlam, PsychicM ]
        42 Xatu      [ QuickAttack, FutureSight, ConfuseRay, PsychicM ]
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

    Bruno1 => {
        "BRUNO"
        42 Hitmontop  [ Pursuit, QuickAttack, Dig, Detect ]
        42 Hitmonlee  [ Swagger, DoubleKick, HiJumpKick, Foresight ]
        42 Hitmonchan [ Thunderpunch, IcePunch, FirePunch, MachPunch ]
        43 Onix       [ Bind, Earthquake, Sandstorm, RockSlide ]
        46 Machamp    [ RockSlide, Foresight, VitalThrow, CrossChop ]
    }

    Karen1 => {
        "KAREN"
        42 Umbreon   [ SandAttack, ConfuseRay, FaintAttack, MeanLook ]
        42 Vileplume [ StunSpore, Acid, Moonlight, PetalDance ]
        45 Gengar    [ Lick, Spite, Curse, DestinyBond ]
        44 Murkrow   [ QuickAttack, Whirlwind, Pursuit, FaintAttack ]
        47 Houndoom  [ Roar, Pursuit, Flamethrower, Crunch ]
    }

    Koga1 => {
        "KOGA"
        40 Ariados    [ DoubleTeam, SpiderWeb, BatonPass, GigaDrain ]
        41 Venomoth   [ Supersonic, Gust, PsychicM, Toxic ]
        43 Forretress [ Protect, Swift, Explosion, Spikes ]
        42 Muk        [ Minimize, AcidArmor, SludgeBomb, Toxic ]
        44 Crobat     [ DoubleTeam, QuickAttack, WingAttack, Toxic ]
    }

    Lance => {
        "LANCE"
        44 Gyarados   [ Flail, RainDance, Surf, HyperBeam ]
        47 Dragonite  [ ThunderWave, Twister, Thunder, HyperBeam ]
        47 Dragonite  [ ThunderWave, Twister, Blizzard, HyperBeam ]
        46 Aerodactyl [ WingAttack, Ancientpower, RockSlide, HyperBeam ]
        46 Charizard  [ Flamethrower, WingAttack, Slash, HyperBeam ]
        50 Dragonite  [ FireBlast, Safeguard, Outrage, HyperBeam ]
    }

    Brock1 => {
        "BROCK"
        41 Graveler [ DefenseCurl, RockSlide, Rollout, Earthquake ]
        41 Rhyhorn  [ FuryAttack, ScaryFace, Earthquake, HornDrill ]
        42 Omastar  [ Bite, Surf, Protect, SpikeCannon ]
        44 Onix     [ Bind, RockSlide, Bide, Sandstorm ]
        42 Kabutops [ Slash, Surf, Endure, GigaDrain ]
    }

    Misty1 => {
        "MISTY"
        42 Golduck  [ Surf, Disable, PsychUp, PsychicM ]
        42 Quagsire [ Surf, Amnesia, Earthquake, RainDance ]
        44 Lapras   [ Surf, PerishSong, Blizzard, RainDance ]
        47 Starmie  [ Surf, ConfuseRay, Recover, IceBeam ]
    }

    LtSurge1 => {
        "LT.SURGE"
        44 Raichu     [ ThunderWave, QuickAttack, Thunderbolt, Thunder ]
        40 Electrode  [ Screech, DoubleTeam, Swift, Explosion ]
        40 Magneton   [ LockOn, DoubleTeam, Swift, ZapCannon ]
        40 Electrode  [ Screech, DoubleTeam, Swift, Explosion ]
        46 Electabuzz [ QuickAttack, Thunderpunch, LightScreen, Thunder ]
    }

    Ross => {
        "ROSS"
        22 Koffing
        22 Koffing
    }

    Mitch => {
        "MITCH"
        24 Ditto
    }

    Jed => {
        "JED"
        20 Magnemite
        20 Magnemite
        20 Magnemite
    }

    Marc => {
        "MARC"
        27 Magnemite
        27 Magnemite
        27 Magnemite
    }

    Rich => {
        "RICH"
        30 Porygon [ Conversion, Conversion2, Recover, TriAttack ]
    }

    Erika1 => {
        "ERIKA"
        42 Tangela    [ VineWhip, Bind, GigaDrain, SleepPowder ]
        41 Jumpluff   [ MegaDrain, LeechSeed, CottonSpore, GigaDrain ]
        46 Victreebel [ SunnyDay, Synthesis, Acid, RazorLeaf ]
        46 Bellossom  [ SunnyDay, Synthesis, PetalDance, Solarbeam ]
    }

    Joey1 => {
        "JOEY"
        4 Rattata
    }

    Mikey => {
        "MIKEY"
        2 Pidgey
        4 Rattata
    }

    Albert => {
        "ALBERT"
        6 Rattata
        8 Zubat
    }

    Gordon => {
        "GORDON"
        10 Wooper
    }

    Samuel => {
        "SAMUEL"
         7 Rattata
        10 Sandshrew
         8 Spearow
         8 Spearow
    }

    Ian => {
        "IAN"
        10 Mankey
        12 Diglett
    }

    Joey2 => {
        "JOEY"
        15 Rattata
    }

    Joey3 => {
        "JOEY"
        21 Raticate [ TailWhip, QuickAttack, HyperFang, ScaryFace ]
    }

    Warren => {
        "WARREN"
        35 Fearow
    }

    Jimmy => {
        "JIMMY"
        33 Raticate
        33 Arbok
    }

    Owen => {
        "OWEN"
        35 Growlithe
    }

    Jason => {
        "JASON"
        33 Sandslash
        33 Crobat
    }

    Joey4 => {
        "JOEY"
        30 Raticate [ TailWhip, QuickAttack, HyperFang, Pursuit ]
    }

    Joey5 => {
        "JOEY"
        37 Raticate [ HyperBeam, QuickAttack, HyperFang, Pursuit ]
    }

    Jack1 => {
        "JACK"
        12 Oddish
        15 Voltorb
    }

    Kipp => {
        "KIPP"
        27 Voltorb
        27 Magnemite
        31 Voltorb
        31 Magneton
    }

    Alan1 => {
        "ALAN"
        16 Tangela
    }

    Johnny => {
        "JOHNNY"
        29 Bellsprout
        31 Weepinbell
        33 Victreebel
    }

    Danny => {
        "DANNY"
        31 Jynx
        31 Electabuzz
        31 Magmar
    }

    Tommy => {
        "TOMMY"
        32 Xatu
        34 Alakazam
    }

    Dudley => {
        "DUDLEY"
        35 Oddish
    }

    Joe => {
        "JOE"
        33 Tangela
        33 Vaporeon
    }

    Billy => {
        "BILLY"
        27 Paras
        27 Paras
        27 Poliwhirl
        35 Ditto
    }

    Chad1 => {
        "CHAD"
        19 MrMime
    }

    Nate => {
        "NATE"
        32 Ledian
        32 Exeggutor
    }

    Ricky => {
        "RICKY"
        32 Aipom
        32 Ditto
    }

    Jack2 => {
        "JACK"
        14 Oddish
        17 Voltorb
    }

    Jack3 => {
        "JACK"
        28 Gloom
        31 Electrode
    }

    Alan2 => {
        "ALAN"
        17 Tangela
        17 Yanma
    }

    Alan3 => {
        "ALAN"
        20 Natu
        22 Tangela
        20 Quagsire
        25 Yanma
    }

    Chad2 => {
        "CHAD"
        19 MrMime
        19 Magnemite
    }

    Chad3 => {
        "CHAD"
        27 MrMime
        31 Magneton
    }

    Jack4 => {
        "JACK"
        30 Gloom
        33 Growlithe
        33 Electrode
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
        35 Xatu     [ Peck, NightShade, Swift, FutureSight ]
        32 Tangela  [ Poisonpowder, VineWhip, Bind, MegaDrain ]
        32 Yanma    [ QuickAttack, DoubleTeam, Sonicboom, Supersonic ]
        35 Quagsire [ TailWhip, Slam, Amnesia, Earthquake ]
    }

    Chad4 => {
        "CHAD"
        30 MrMime
        34 Magneton
    }

    Chad5 => {
        "CHAD"
        34 MrMime   [ PsychicM, LightScreen, Reflect, Encore ]
        38 Magneton [ ZapCannon, ThunderWave, LockOn, Swift ]
    }

    Rod => {
        "ROD"
        7 Pidgey
        7 Pidgey
    }

    Abe => {
        "ABE"
        9 Spearow
    }

    Bryan => {
        "BRYAN"
        12 Pidgey
        14 Pidgeotto
    }

    Theo => {
        "THEO"
        17 Pidgey
        15 Pidgey
        19 Pidgey
        15 Pidgey
        15 Pidgey
    }

    Toby => {
        "TOBY"
        15 Doduo
        16 Doduo
        17 Doduo
    }

    Denis => {
        "DENIS"
        18 Spearow
        20 Fearow
        18 Spearow
    }

    Vance1 => {
        "VANCE"
        25 Pidgeotto
        25 Pidgeotto
    }

    Hank => {
        "HANK"
        12 Pidgey
        34 Pidgeot
    }

    Roy => {
        "ROY"
        29 Fearow
        35 Fearow
    }

    Boris => {
        "BORIS"
        30 Doduo
        28 Doduo
        32 Dodrio
    }

    Bob => {
        "BOB"
        34 Noctowl
    }

    Jose1 => {
        "JOSE"
        36 Farfetchd
    }

    Peter => {
        "PETER"
        6 Pidgey
        6 Pidgey
        8 Spearow
    }

    Jose2 => {
        "JOSE"
        34 Farfetchd
    }

    Perry => {
        "PERRY"
        34 Farfetchd
    }

    Bret => {
        "BRET"
        32 Pidgeotto
        32 Fearow
    }

    Jose3 => {
        "JOSE"
        40 Farfetchd [ FuryAttack, Detect, Fly, Slash ]
    }

    Vance2 => {
        "VANCE"
        32 Pidgeotto
        32 Pidgeotto
    }

    Vance3 => {
        "VANCE"
        38 Pidgeot [ Toxic, QuickAttack, Whirlwind, Fly ]
        38 Pidgeot [ Swift, Detect, SteelWing, Fly ]
    }

    Carrie => {
        "CARRIE"
        18 Snubbull [ ScaryFace, Charm, Bite, Lick ]
    }

    Bridget => {
        "BRIDGET"
        15 Jigglypuff
        15 Jigglypuff
        15 Jigglypuff
    }

    Alice => {
        "ALICE"
        30 Gloom
        34 Arbok
        30 Gloom
    }

    Krise => {
        "KRISE"
        12 Oddish
        15 Cubone
    }

    Connie1 => {
        "CONNIE"
        21 Marill
    }

    Linda => {
        "LINDA"
        30 Bulbasaur
        32 Ivysaur
        34 Venusaur
    }

    Laura => {
        "LAURA"
        28 Gloom
        31 Pidgeotto
        31 Bellossom
    }

    Shannon => {
        "SHANNON"
        29 Paras
        29 Paras
        32 Parasect
    }

    Michelle => {
        "MICHELLE"
        32 Skiploom
        33 Hoppip
        34 Jumpluff
    }

    Dana1 => {
        "DANA"
        18 Flaaffy [ Tackle, Growl, Thundershock, ThunderWave ]
        18 Psyduck [ Scratch, TailWhip, Disable, Confusion ]
    }

    Ellen => {
        "ELLEN"
        30 Wigglytuff
        34 Granbull
    }

    Connie2 => {
        "CONNIE"
        21 Marill
    }

    Connie3 => {
        "CONNIE"
        21 Marill
    }

    Dana2 => {
        "DANA"
        21 Flaaffy [ Tackle, Growl, Thundershock, ThunderWave ]
        21 Psyduck [ Scratch, TailWhip, Disable, Confusion ]
    }

    Dana3 => {
        "DANA"
        29 Psyduck  [ Scratch, Disable, Confusion, Screech ]
        29 Ampharos [ Tackle, Thundershock, ThunderWave, CottonSpore ]
    }

    Dana4 => {
        "DANA"
        32 Psyduck  [ Scratch, Disable, Confusion, Screech ]
        32 Ampharos [ Tackle, Thunderpunch, ThunderWave, CottonSpore ]
    }

    Dana5 => {
        "DANA"
        36 Ampharos [ Swift, Thunderpunch, ThunderWave, CottonSpore ]
        36 Golduck  [ Disable, Surf, PsychicM, Screech ]
    }

    Janine1 => {
        "JANINE"
        36 Crobat   [ Screech, Supersonic, ConfuseRay, WingAttack ]
        36 Weezing  [ Smog, SludgeBomb, Toxic, Explosion ]
        36 Weezing  [ Smog, SludgeBomb, Toxic, Explosion ]
        33 Ariados  [ ScaryFace, GigaDrain, StringShot, NightShade ]
        39 Venomoth [ Foresight, DoubleTeam, Gust, PsychicM ]
    }

    Nick => {
        "NICK"
        26 Charmander [ Ember, Smokescreen, Rage, ScaryFace ]
        26 Squirtle   [ Withdraw, WaterGun, Bite, Curse ]
        26 Bulbasaur  [ LeechSeed, Poisonpowder, SleepPowder, RazorLeaf ]
    }

    Aaron => {
        "AARON"
        24 Ivysaur
        24 Charmeleon
        24 Wartortle
    }

    Paul => {
        "PAUL"
        34 Dratini
        34 Dratini
        34 Dratini
    }

    Cody => {
        "CODY"
        34 Horsea
        36 Seadra
    }

    Mike => {
        "MIKE"
        37 Dragonair
    }

    Gaven1 => {
        "GAVEN"
        35 Victreebel [ Wrap, Toxic, Acid, RazorLeaf ]
        35 Kingler    [ Bubblebeam, Stomp, Guillotine, Protect ]
        35 Flareon    [ SandAttack, QuickAttack, Bite, FireSpin ]
    }

    Gaven2 => {
        "GAVEN"
        39 Victreebel          [ GigaDrain, Toxic, SludgeBomb, RazorLeaf ]
        39 Kingler + KingsRock [ Surf, Stomp, Guillotine, Blizzard ]
        39 Flareon             [ Flamethrower, QuickAttack, Bite, FireSpin ]
    }

    Ryan => {
        "RYAN"
        25 Pidgeot    [ SandAttack, QuickAttack, Whirlwind, WingAttack ]
        27 Electabuzz [ Thunderpunch, LightScreen, Swift, Screech ]
    }

    Jake => {
        "JAKE"
        33 Parasect [ LeechLife, Spore, Slash, SwordsDance ]
        35 Golduck  [ Confusion, Screech, PsychUp, FurySwipes ]
    }

    Gaven3 => {
        "GAVEN"
        32 Victreebel [ Wrap, Toxic, Acid, RazorLeaf ]
        32 Kingler    [ Bubblebeam, Stomp, Guillotine, Protect ]
        32 Flareon    [ SandAttack, QuickAttack, Bite, FireSpin ]
    }

    Blake => {
        "BLAKE"
        33 Magneton  [ Thunderbolt, Supersonic, Swift, Screech ]
        31 Quagsire  [ WaterGun, Slam, Amnesia, Earthquake ]
        31 Exeggcute [ LeechSeed, Confusion, SleepPowder, Solarbeam ]
    }

    Brian => {
        "BRIAN"
        35 Sandslash [ SandAttack, PoisonSting, Slash, Swift ]
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
        35 Flareon
        35 Tangela
        35 Tauros
    }

    Kevin => {
        "KEVIN"
        38 Rhyhorn
        35 Charmeleon
        35 Wartortle
    }

    Steve => {
        "STEVE"
        14 Bulbasaur
        14 Charmander
        14 Squirtle
    }

    Allen => {
        "ALLEN"
        27 Charmeleon [ Ember, Smokescreen, Rage, ScaryFace ]
    }

    Darin => {
        "DARIN"
        37 Dragonair [ Wrap, Surf, DragonRage, Slam ]
    }

    Gwen => {
        "GWEN"
        26 Eevee
        22 Flareon
        22 Vaporeon
        22 Jolteon
    }

    Lois => {
        "LOIS"
        25 Skiploom  [ Synthesis, Poisonpowder, MegaDrain, LeechSeed ]
        25 Ninetales [ Ember, QuickAttack, ConfuseRay, Safeguard ]
    }

    Fran => {
        "FRAN"
        37 Seadra
    }

    Lola => {
        "LOLA"
        34 Dratini
        36 Dragonair
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
        27 Marill
        24 Wartortle
        24 Wartortle
    }

    Joyce => {
        "JOYCE"
        36 Pikachu   [ QuickAttack, DoubleTeam, Thunderbolt, Thunder ]
        32 Blastoise [ Bite, Curse, Surf, RainDance ]
    }

    Beth1 => {
        "BETH"
        36 Rapidash [ Stomp, FireSpin, FuryAttack, Agility ]
    }

    Reena1 => {
        "REENA"
        31 Starmie
        33 Nidoqueen
        31 Starmie
    }

    Megan => {
        "MEGAN"
        32 Bulbasaur [ Growl, LeechSeed, Poisonpowder, RazorLeaf ]
        32 Ivysaur   [ Growl, LeechSeed, Poisonpowder, RazorLeaf ]
        32 Venusaur  [ BodySlam, SleepPowder, RazorLeaf, SweetScent ]
    }

    Beth2 => {
        "BETH"
        39 Rapidash [ Stomp, FireSpin, FuryAttack, Agility ]
    }

    Carol => {
        "CAROL"
        35 Electrode
        35 Starmie
        35 Ninetales
    }

    Quinn => {
        "QUINN"
        38 Ivysaur
        38 Starmie
    }

    Emma => {
        "EMMA"
        28 Poliwhirl
    }

    Cybil => {
        "CYBIL"
        25 Butterfree [ Confusion, SleepPowder, Whirlwind, Gust ]
        25 Bellossom  [ Absorb, StunSpore, Acid, Solarbeam ]
    }

    Jenn => {
        "JENN"
        24 Staryu
        26 Starmie
    }

    Beth3 => {
        "BETH"
        43 Rapidash + FocusBand [ Stomp, FireSpin, FuryAttack, FireBlast ]
    }

    Reena2 => {
        "REENA"
        34 Starmie
        36 Nidoqueen
        34 Starmie
    }

    Reena3 => {
        "REENA"
        38 Starmie             [ DoubleTeam, PsychicM, Waterfall, ConfuseRay ]
        40 Nidoqueen + PinkBow [ Earthquake, DoubleKick, Toxic, BodySlam ]
        38 Starmie             [ Blizzard, PsychicM, Waterfall, Recover ]
    }

    Cara => {
        "CARA"
        33 Horsea [ Smokescreen, Leer, Whirlpool, Twister ]
        33 Horsea [ Smokescreen, Leer, Whirlpool, Twister ]
        35 Seadra [ Swift, Leer, Waterfall, Twister ]
    }

    Victoria => {
        "VICTORIA"
         9 Sentret
        13 Sentret
        17 Sentret
    }

    Samantha => {
        "SAMANTHA"
        16 Meowth [ Scratch, Growl, Bite, PayDay ]
        16 Meowth [ Scratch, Growl, Bite, Slash ]
    }

    Julie => {
        "JULIE"
        15 Sentret
    }

    Jaclyn => {
        "JACLYN"
        15 Sentret
    }

    Brenda => {
        "BRENDA"
        16 Furret
    }

    Cassie => {
        "CASSIE"
        28 Vileplume
        34 Butterfree
    }

    Caroline => {
        "CAROLINE"
        30 Marill
        32 Seel
        30 Marill
    }

    Carlene => {
        "CARLENE"
        15 Sentret
    }

    Jessica => {
        "JESSICA"
        15 Sentret
    }

    Rachael => {
        "RACHAEL"
        15 Sentret
    }

    Angelica => {
        "ANGELICA"
        15 Sentret
    }

    Kendra => {
        "KENDRA"
        15 Sentret
    }

    Veronica => {
        "VERONICA"
        15 Sentret
    }

    Julia => {
        "JULIA"
        32 Paras
        32 Exeggcute
        35 Parasect
    }

    Theresa => {
        "THERESA"
        15 Sentret
    }

    Valerie => {
        "VALERIE"
        17 Hoppip   [ Synthesis, TailWhip, Tackle, Poisonpowder ]
        17 Skiploom [ Synthesis, TailWhip, Tackle, StunSpore ]
    }

    Olivia => {
        "OLIVIA"
        19 Corsola
    }

    Larry => {
        "LARRY"
        10 Slowpoke
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
        16 Nidorina
        16 Nidorino
    }

    Ben => {
        "BEN"
        19 Slowbro
    }

    Brent1 => {
        "BRENT"
        19 Lickitung
    }

    Ron => {
        "RON"
        19 Nidoking
    }

    Ethan => {
        "ETHAN"
        31 Rhyhorn
        31 Rhydon
    }

    Brent2 => {
        "BRENT"
        25 Kangaskhan
    }

    Brent3 => {
        "BRENT"
        36 Porygon [ Recover, PsychicM, Conversion2, TriAttack ]
    }

    Issac => {
        "ISSAC"
        12 Lickitung [ Lick, Supersonic, Cut ]
    }

    Donald => {
        "DONALD"
        10 Slowpoke
        10 Slowpoke
    }

    Zach => {
        "ZACH"
        27 Rhyhorn
    }

    Brent4 => {
        "BRENT"
        41 Chansey [ Rollout, Attract, EggBomb, Softboiled ]
    }

    Miller => {
        "MILLER"
        17 Nidoking
        17 Nidoqueen
    }

    GruntM1 => {
        "GRUNT"
        14 Koffing
    }

    GruntM2 => {
        "GRUNT"
        7 Rattata
        9 Zubat
        9 Zubat
    }

    GruntM3 => {
        "GRUNT"
        24 Raticate
        24 Raticate
    }

    GruntM4 => {
        "GRUNT"
        23 Grimer
        23 Grimer
        25 Muk
    }

    GruntM5 => {
        "GRUNT"
        21 Rattata
        21 Rattata
        23 Rattata
        23 Rattata
        23 Rattata
    }

    GruntM6 => {
        "GRUNT"
        26 Zubat
        26 Zubat
    }

    GruntM7 => {
        "GRUNT"
        23 Koffing
        23 Grimer
        23 Zubat
        23 Rattata
    }

    GruntM8 => {
        "GRUNT"
        26 Weezing
    }

    GruntM9 => {
        "GRUNT"
        24 Raticate
        26 Koffing
    }

    GruntM10 => {
        "GRUNT"
        22 Zubat
        24 Golbat
        22 Grimer
    }

    GruntM11 => {
        "GRUNT"
        23 Muk
        23 Koffing
        25 Rattata
    }

    GruntM12 => {
        "EXECUTIVE"
        33 Houndour
    }

    GruntM13 => {
        "GRUNT"
        27 Rattata
    }

    GruntM14 => {
        "GRUNT"
        24 Raticate
        24 Golbat
    }

    GruntM15 => {
        "GRUNT"
        26 Grimer
        23 Weezing
    }

    GruntM16 => {
        "GRUNT"
        16 Rattata
        16 Rattata
        16 Rattata
        16 Rattata
    }

    GruntM17 => {
        "GRUNT"
        18 Golbat
    }

    GruntM18 => {
        "GRUNT"
        17 Rattata
        17 Zubat
        17 Rattata
    }

    GruntM19 => {
        "GRUNT"
        18 Venonat
        18 Venonat
    }

    GruntM20 => {
        "GRUNT"
        17 Drowzee
        19 Zubat
    }

    GruntM21 => {
        "GRUNT"
        16 Zubat
        17 Grimer
        18 Rattata
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
        25 Koffing
        25 Koffing
    }

    GruntM25 => {
        "GRUNT"
        24 Koffing
        24 Muk
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
        "GRUNT"
        19 Raticate
    }

    GruntM29 => {
        "GRUNT"
        9 Rattata
        9 Rattata
    }

    GruntM30 => {
        "GRUNT"
        25 Golbat
        25 Golbat
        30 Arbok
    }

    GruntM31 => {
        "GRUNT"
        30 Golbat
    }

    Preston => {
        "PRESTON"
        18 Growlithe
        18 Growlithe
    }

    Edward => {
        "EDWARD"
        33 Persian
    }

    Gregory => {
        "GREGORY"
        37 Pikachu
        33 Flaaffy
    }

    Virgil => {
        "VIRGIL"
        20 Ponyta
    }

    Alfred => {
        "ALFRED"
        20 Noctowl
    }

    Roxanne => {
        "ROXANNE"
        28 Jynx
    }

    Clarissa => {
        "CLARISSA"
        28 Dewgong
    }

    Colette => {
        "COLETTE"
        36 Clefairy
    }

    Hillary => {
        "HILLARY"
        32 Aipom
        36 Cubone
    }

    Shirley => {
        "SHIRLEY"
        35 Jigglypuff
    }

    Sabrina1 => {
        "SABRINA"
        46 Espeon   [ SandAttack, QuickAttack, Swift, PsychicM ]
        46 MrMime   [ Barrier, Reflect, BatonPass, PsychicM ]
        48 Alakazam [ Recover, FutureSight, PsychicM, Reflect ]
    }

    Don => {
        "DON"
        3 Caterpie
        3 Caterpie
    }

    Rob => {
        "ROB"
        32 Beedrill
        32 Butterfree
    }

    Ed => {
        "ED"
        30 Beedrill
        30 Beedrill
        30 Beedrill
    }

    Wade1 => {
        "WADE"
        2 Caterpie
        2 Caterpie
        3 Weedle
        2 Caterpie
    }

    BugCatcherBenny => {
        "BENNY"
         7 Weedle
         9 Kakuna
        12 Beedrill
    }

    Al => {
        "AL"
        12 Caterpie
        12 Weedle
    }

    Josh => {
        "JOSH"
        13 Paras
    }

    Arnie1 => {
        "ARNIE"
        15 Venonat
    }

    Ken => {
        "KEN"
        30 Ariados
        32 Pinsir
    }

    Wade2 => {
        "WADE"
         9 Metapod
         9 Metapod
        10 Kakuna
         9 Metapod
    }

    Wade3 => {
        "WADE"
        14 Butterfree
        14 Butterfree
        15 Beedrill
        14 Butterfree
    }

    Doug => {
        "DOUG"
        34 Ariados
    }

    Arnie2 => {
        "ARNIE"
        19 Venonat
    }

    Arnie3 => {
        "ARNIE"
        28 Venomoth [ Disable, Supersonic, Confusion, LeechLife ]
    }

    Wade4 => {
        "WADE"
        24 Butterfree [ Confusion, Poisonpowder, Supersonic, Whirlwind ]
        24 Butterfree [ Confusion, StunSpore, Supersonic, Whirlwind ]
        25 Beedrill   [ FuryAttack, FocusEnergy, Twineedle, Rage ]
        24 Butterfree [ Confusion, SleepPowder, Supersonic, Whirlwind ]
    }

    Wade5 => {
        "WADE"
        30 Butterfree [ Confusion, Poisonpowder, Supersonic, Gust ]
        30 Butterfree [ Confusion, StunSpore, Supersonic, Gust ]
        32 Beedrill   [ FuryAttack, Pursuit, Twineedle, DoubleTeam ]
        34 Butterfree [ Psybeam, SleepPowder, Gust, Whirlwind ]
    }

    Arnie4 => {
        "ARNIE"
        36 Venomoth [ Gust, Supersonic, Psybeam, LeechLife ]
    }

    Arnie5 => {
        "ARNIE"
        40 Venomoth [ Gust, Supersonic, PsychicM, Toxic ]
    }

    Wayne => {
        "WAYNE"
         8 Ledyba
        10 Paras
    }

    Justin => {
        "JUSTIN"
         5 Magikarp
         5 Magikarp
        15 Magikarp
         5 Magikarp
    }

    Ralph1 => {
        "RALPH"
        10 Goldeen
    }

    Arnold => {
        "ARNOLD"
        34 Tentacruel
    }

    Kyle => {
        "KYLE"
        28 Seaking
        31 Poliwhirl
        31 Seaking
    }

    Henry => {
        "HENRY"
        8 Poliwag
        8 Poliwag
    }

    Marvin => {
        "MARVIN"
        10 Magikarp
        10 Gyarados
        15 Magikarp
        15 Gyarados
    }

    Tully1 => {
        "TULLY"
        18 Qwilfish
    }

    Andre => {
        "ANDRE"
        27 Gyarados
    }

    Raymond => {
        "RAYMOND"
        22 Magikarp
        22 Magikarp
        22 Magikarp
        22 Magikarp
    }

    Wilton1 => {
        "WILTON"
        23 Goldeen
        23 Goldeen
        25 Seaking
    }

    Edgar => {
        "EDGAR"
        25 Remoraid [ LockOn, Psybeam, AuroraBeam, Bubblebeam ]
        25 Remoraid [ LockOn, Psybeam, AuroraBeam, Bubblebeam ]
    }

    Jonah => {
        "JONAH"
        25 Shellder
        29 Octillery
        25 Remoraid
        29 Cloyster
    }

    Martin => {
        "MARTIN"
        32 Remoraid
        32 Remoraid
    }

    Stephen => {
        "STEPHEN"
        25 Magikarp
        25 Magikarp
        31 Qwilfish
        31 Tentacruel
    }

    Barney => {
        "BARNEY"
        30 Gyarados
        30 Gyarados
        30 Gyarados
    }

    Ralph2 => {
        "RALPH"
        17 Goldeen
    }

    Ralph3 => {
        "RALPH"
        17 Qwilfish
        19 Goldeen
    }

    Tully2 => {
        "TULLY"
        23 Qwilfish
    }

    Tully3 => {
        "TULLY"
        32 Goldeen
        32 Goldeen
        32 Qwilfish
    }

    Wilton2 => {
        "WILTON"
        29 Goldeen
        29 Goldeen
        32 Seaking
    }

    Scott => {
        "SCOTT"
        30 Qwilfish
        30 Qwilfish
        34 Seaking
    }

    Wilton3 => {
        "WILTON"
        34 Seaking  [ Supersonic, Waterfall, Flail, FuryAttack ]
        34 Seaking  [ Supersonic, Waterfall, Flail, FuryAttack ]
        38 Remoraid [ Psybeam, AuroraBeam, Bubblebeam, HyperBeam ]
    }

    Ralph4 => {
        "RALPH"
        30 Qwilfish
        32 Goldeen
    }

    Ralph5 => {
        "RALPH"
        35 Qwilfish [ Toxic, Minimize, Surf, PinMissile ]
        39 Seaking  [ Endure, Flail, FuryAttack, Waterfall ]
    }

    Tully4 => {
        "TULLY"
        34 Seaking  [ Supersonic, RainDance, Waterfall, FuryAttack ]
        34 Seaking  [ Supersonic, RainDance, Waterfall, FuryAttack ]
        37 Qwilfish [ Rollout, Surf, PinMissile, TakeDown ]
    }

    Harold => {
        "HAROLD"
        32 Remoraid
        30 Seadra
    }

    Simon => {
        "SIMON"
        20 Tentacool
        20 Tentacool
    }

    Randall => {
        "RANDALL"
        18 Shellder
        20 Wartortle
        18 Shellder
    }

    Charlie => {
        "CHARLIE"
        21 Shellder
        19 Tentacool
        19 Tentacruel
    }

    George => {
        "GEORGE"
        16 Tentacool
        17 Tentacool
        16 Tentacool
        19 Staryu
        17 Tentacool
        19 Remoraid
    }

    Berke => {
        "BERKE"
        23 Qwilfish
    }

    Kirk => {
        "KIRK"
        20 Gyarados
        20 Gyarados
    }

    Mathew => {
        "MATHEW"
        23 Krabby
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
        26 Seadra
        28 Tentacool
        30 Tentacruel
        28 Goldeen
    }

    Tucker => {
        "TUCKER"
        30 Shellder
        34 Cloyster
    }

    Rick => {
        "RICK"
        13 Staryu
        18 Starmie
        16 Horsea
    }

    Cameron => {
        "CAMERON"
        34 Marill
    }

    Seth => {
        "SETH"
        29 Quagsire
        29 Octillery
        32 Quagsire
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
        32 Horsea
        32 Horsea
        35 Seadra
    }

    Elaine => {
        "ELAINE"
        21 Staryu
    }

    Paula => {
        "PAULA"
        19 Staryu
        19 Shellder
    }

    Kaylee => {
        "KAYLEE"
        18 Goldeen
        20 Goldeen
        20 Seaking
    }

    Susie => {
        "SUSIE"
        20 Psyduck [ Scratch, TailWhip, Disable, Confusion ]
        22 Goldeen [ Peck, TailWhip, Supersonic, HornAttack ]
    }

    Denise => {
        "DENISE"
        22 Seel
    }

    Kara => {
        "KARA"
        20 Staryu
        20 Starmie
    }

    Wendy => {
        "WENDY"
        21 Horsea [ Bubble, Smokescreen, Leer, WaterGun ]
        21 Horsea [ DragonRage, Smokescreen, Leer, WaterGun ]
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
        34 Seaking
    }

    Tara => {
        "TARA"
        20 Seaking
    }

    Nicole => {
        "NICOLE"
        29 Marill
        29 Marill
        32 Lapras
    }

    Lori => {
        "LORI"
        32 Starmie
        32 Starmie
    }

    Jody => {
        "JODY"
        20 Seaking
    }

    Nikki => {
        "NIKKI"
        28 Seel
        28 Seel
        28 Seel
        28 Dewgong
    }

    Diana => {
        "DIANA"
        37 Golduck
    }

    Briana => {
        "BRIANA"
        35 Seaking
        35 Seaking
    }

    Eugene => {
        "EUGENE"
        17 Poliwhirl
        17 Raticate
        19 Krabby
    }

    Huey1 => {
        "HUEY"
        18 Poliwag
        18 Poliwhirl
    }

    Terrell => {
        "TERRELL"
        20 Poliwhirl
    }

    Kent => {
        "KENT"
        18 Krabby [ Bubble, Leer, Vicegrip, Harden ]
        20 Krabby [ Bubblebeam, Leer, Vicegrip, Harden ]
    }

    Ernest => {
        "ERNEST"
        18 Machop
        18 Machop
        18 Poliwhirl
    }

    Jeff => {
        "JEFF"
        32 Raticate
        32 Raticate
    }

    Garrett => {
        "GARRETT"
        34 Kingler
    }

    Kenneth => {
        "KENNETH"
        28 Machop
        28 Machop
        28 Poliwrath
        28 Machop
    }

    Stanly => {
        "STANLY"
        31 Machop
        33 Machoke
        26 Psyduck
    }

    Harry => {
        "HARRY"
        19 Wooper
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
        11 Grimer
        11 Grimer
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
        34 Grimer
        34 Muk
    }

    Tom => {
        "TOM"
        32 Magnemite
        32 Magnemite
        32 Magnemite
    }

    Pat => {
        "PAT"
        36 Porygon
    }

    Shawn => {
        "SHAWN"
        31 Magnemite
        33 Muk
        31 Magnemite
    }

    Teru => {
        "TERU"
         7 Magnemite
        11 Voltorb
         7 Magnemite
         9 Magnemite
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
        41 Sneasel  [ QuickAttack, Screech, FaintAttack, FuryCutter ]
        42 Golbat   [ LeechLife, Bite, ConfuseRay, WingAttack ]
        41 Magneton [ Thundershock, Sonicboom, ThunderWave, Swift ]
        43 Gengar   [ MeanLook, Curse, ShadowBall, ConfuseRay ]
        43 Alakazam [ Disable, Recover, FutureSight, PsychicM ]
        45 Meganium [ RazorLeaf, Poisonpowder, BodySlam, LightScreen ]
    }

    Rival21Cyndaquil => {
        "?"
        41 Sneasel    [ QuickAttack, Screech, FaintAttack, FuryCutter ]
        42 Golbat     [ LeechLife, Bite, ConfuseRay, WingAttack ]
        41 Magneton   [ Thundershock, Sonicboom, ThunderWave, Swift ]
        43 Gengar     [ MeanLook, Curse, ShadowBall, ConfuseRay ]
        43 Alakazam   [ Disable, Recover, FutureSight, PsychicM ]
        45 Typhlosion [ Smokescreen, QuickAttack, FlameWheel, Swift ]
    }

    Rival21Totodile => {
        "?"
        41 Sneasel    [ QuickAttack, Screech, FaintAttack, FuryCutter ]
        42 Golbat     [ LeechLife, Bite, ConfuseRay, WingAttack ]
        41 Magneton   [ Thundershock, Sonicboom, ThunderWave, Swift ]
        43 Gengar     [ MeanLook, Curse, ShadowBall, ConfuseRay ]
        43 Alakazam   [ Disable, Recover, FutureSight, PsychicM ]
        45 Feraligatr [ Rage, WaterGun, ScaryFace, Slash ]
    }

    Rival22Chikorita => {
        "?"
        45 Sneasel  [ QuickAttack, Screech, FaintAttack, FuryCutter ]
        48 Crobat   [ Toxic, Bite, ConfuseRay, WingAttack ]
        45 Magneton [ Thunder, Sonicboom, ThunderWave, Swift ]
        46 Gengar   [ MeanLook, Curse, ShadowBall, ConfuseRay ]
        46 Alakazam [ Recover, FutureSight, PsychicM, Reflect ]
        50 Meganium [ GigaDrain, BodySlam, LightScreen, Safeguard ]
    }

    Rival22Cyndaquil => {
        "?"
        45 Sneasel    [ QuickAttack, Screech, FaintAttack, FuryCutter ]
        48 Crobat     [ Toxic, Bite, ConfuseRay, WingAttack ]
        45 Magneton   [ Thunder, Sonicboom, ThunderWave, Swift ]
        46 Gengar     [ MeanLook, Curse, ShadowBall, ConfuseRay ]
        46 Alakazam   [ Recover, FutureSight, PsychicM, Reflect ]
        50 Typhlosion [ Smokescreen, QuickAttack, FireBlast, Swift ]
    }

    Rival22Totodile => {
        "?"
        45 Sneasel    [ QuickAttack, Screech, FaintAttack, FuryCutter ]
        48 Crobat     [ Toxic, Bite, ConfuseRay, WingAttack ]
        45 Magneton   [ Thunder, Sonicboom, ThunderWave, Swift ]
        46 Gengar     [ MeanLook, Curse, ShadowBall, ConfuseRay ]
        46 Alakazam   [ Recover, FutureSight, PsychicM, Reflect ]
        50 Feraligatr [ Surf, RainDance, Slash, Screech ]
    }

    Clyde => {
        "CLYDE"
        34 Electabuzz
    }

    Vincent => {
        "VINCENT"
        27 Magnemite
        33 Voltorb
        32 Magnemite
        32 Magnemite
    }

    Anthony1 => {
        "ANTHONY"
        16 Geodude
        18 Machamp
    }

    Russell => {
        "RUSSELL"
        4 Geodude
        6 Geodude
        8 Geodude
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
        14 Diglett
        14 Geodude
        16 Dugtrio
    }

    Erik => {
        "ERIK"
        24 Machop
        27 Graveler
        27 Machop
    }

    Michael => {
        "MICHAEL"
        25 Geodude
        25 Graveler
        25 Golem
    }

    Parry1 => {
        "PARRY"
        35 Onix
        33 Swinub
    }

    Timothy => {
        "TIMOTHY"
        27 Diglett [ Magnitude, Dig, SandAttack, Slash ]
        27 Dugtrio [ Magnitude, Dig, SandAttack, Slash ]
    }

    Bailey => {
        "BAILEY"
        13 Geodude
        13 Geodude
        13 Geodude
        13 Geodude
        13 Geodude
    }

    Anthony3 => {
        "ANTHONY"
        25 Graveler
        27 Graveler
        29 Machoke
    }

    Tim => {
        "TIM"
        31 Graveler
        31 Graveler
        31 Graveler
    }

    Noland => {
        "NOLAND"
        31 Sandslash
        33 Golem
    }

    Sidney => {
        "SIDNEY"
        34 Dugtrio
        32 Onix
    }

    Kenny => {
        "KENNY"
        27 Sandslash
        29 Graveler
        31 Golem
        29 Graveler
    }

    Jim => {
        "JIM"
        35 Machamp
    }

    Daniel => {
        "DANIEL"
        11 Onix
    }

    Parry2 => {
        "PARRY"
        35 Piloswine [ Earthquake, Blizzard, Rest, TakeDown ]
        35 Dugtrio   [ Magnitude, Dig, MudSlap, Slash ]
        38 Steelix   [ Dig, IronTail, Sandstorm, Slam ]
    }

    Parry3 => {
        "PARRY"
        29 Onix
    }

    Anthony4 => {
        "ANTHONY"
        30 Graveler
        30 Graveler
        32 Machoke
    }

    Anthony5 => {
        "ANTHONY"
        34 Graveler [ Magnitude, Selfdestruct, DefenseCurl, Rollout ]
        36 Golem    [ Magnitude, Selfdestruct, DefenseCurl, Rollout ]
        34 Machoke  [ KarateChop, VitalThrow, Headbutt, Dig ]
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
        27 Koffing
        28 Koffing
        29 Koffing
        30 Koffing
    }

    Harris => {
        "HARRIS"
        34 Flareon
    }

    Zeke => {
        "ZEKE"
        32 Koffing
        32 Koffing
    }

    Charles => {
        "CHARLES"
        30 Koffing
        30 Charmeleon
        30 Weezing
    }

    Riley => {
        "RILEY"
        34 Weezing
    }

    Joel => {
        "JOEL"
        32 Magmar
        32 Magmar
    }

    Glenn => {
        "GLENN"
        28 Koffing
        30 Magmar
        32 Weezing
    }

    Blaine1 => {
        "BLAINE"
        45 Magcargo [ Curse, Smog, Flamethrower, RockSlide ]
        45 Magmar   [ Thunderpunch, FirePunch, SunnyDay, ConfuseRay ]
        50 Rapidash [ QuickAttack, FireSpin, FuryAttack, FireBlast ]
    }

    Duncan => {
        "DUNCAN"
        23 Koffing
        25 Magmar
        23 Koffing
    }

    Eddie => {
        "EDDIE"
        26 Growlithe [ Roar, Ember, Leer, TakeDown ]
        24 Koffing   [ Tackle, Smog, Sludge, Smokescreen ]
    }

    Corey => {
        "COREY"
        25 Koffing
        28 Magmar
        25 Koffing
        30 Koffing
    }

    Otis => {
        "OTIS"
        29 Magmar
        32 Weezing
        29 Magmar
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
        32 Koffing
        32 Slugma
    }

    Bill => {
        "BILL"
        6 Koffing
        6 Koffing
    }

    Walt => {
        "WALT"
        11 Magmar
        13 Magmar
    }

    Ray => {
        "RAY"
        9 Vulpix
    }

    Lyle => {
        "LYLE"
        28 Koffing
        31 Flareon
        28 Koffing
    }

    Irwin1 => {
        "IRWIN"
         2 Voltorb
         6 Voltorb
        10 Voltorb
        14 Voltorb
    }

    Fritz => {
        "FRITZ"
        29 MrMime
        29 Magmar
        29 Machoke
    }

    Horton => {
        "HORTON"
        33 Electrode
        33 Electrode
        33 Electrode
        33 Electrode
    }

    Irwin2 => {
        "IRWIN"
         6 Voltorb
        10 Voltorb
        14 Voltorb
        18 Voltorb
    }

    Irwin3 => {
        "IRWIN"
        18 Voltorb
        22 Voltorb
        26 Voltorb
        30 Electrode
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
        27 Onix
        30 Hitmonlee
        27 Onix
        32 Machoke
    }

    Yoshi => {
        "YOSHI"
        27 Hitmonlee [ DoubleKick, Meditate, JumpKick, FocusEnergy ]
    }

    Kenji2 => {
        "KENJI"
        33 Onix      [ Bind, RockThrow, Toxic, Dig ]
        38 Machamp   [ Headbutt, Swagger, Thunderpunch, VitalThrow ]
        33 Steelix   [ Earthquake, RockThrow, IronTail, Sandstorm ]
        36 Hitmonlee [ DoubleTeam, HiJumpKick, MudSlap, Swift ]
    }

    Lao => {
        "LAO"
        27 Hitmonchan [ CometPunch, Thunderpunch, IcePunch, FirePunch ]
    }

    Nob => {
        "NOB"
        25 Machop  [ Leer, FocusEnergy, KarateChop, SeismicToss ]
        25 Machoke [ Leer, KarateChop, SeismicToss, RockSlide ]
    }

    Kiyo => {
        "KIYO"
        34 Hitmonlee
        34 Hitmonchan
    }

    Lung => {
        "LUNG"
        23 Mankey
        23 Mankey
        25 Primeape
    }

    Kenji3 => {
        "KENJI"
        28 Machoke
    }

    Wai => {
        "WAI"
        30 Machoke
        32 Machoke
        34 Machoke
    }

    ExecutiveM1 => {
        "EXECUTIVE"
        33 Houndour [ Ember, Roar, Bite, FaintAttack ]
        33 Koffing  [ Tackle, Sludge, Smokescreen, Haze ]
        35 Houndoom [ Ember, Smog, Bite, FaintAttack ]
    }

    ExecutiveM2 => {
        "EXECUTIVE"
        36 Golbat [ LeechLife, Bite, ConfuseRay, WingAttack ]
    }

    ExecutiveM3 => {
        "EXECUTIVE"
        30 Koffing [ Tackle, Selfdestruct, Sludge, Smokescreen ]
        30 Koffing [ Tackle, Selfdestruct, Sludge, Smokescreen ]
        30 Koffing [ Tackle, Selfdestruct, Sludge, Smokescreen ]
        32 Weezing [ Tackle, Explosion, Sludge, Smokescreen ]
        30 Koffing [ Tackle, Selfdestruct, Sludge, Smokescreen ]
        30 Koffing [ Tackle, Smog, Sludge, Smokescreen ]
    }

    ExecutiveM4 => {
        "EXECUTIVE"
        22 Zubat
        24 Raticate
        22 Koffing
    }

    Nathan => {
        "NATHAN"
        26 Girafarig
    }

    Franklin => {
        "FRANKLIN"
        37 Kadabra
    }

    Herman => {
        "HERMAN"
        30 Exeggcute
        30 Exeggcute
        30 Exeggutor
    }

    Fidel => {
        "FIDEL"
        34 Xatu
    }

    Greg => {
        "GREG"
        17 Drowzee [ Hypnosis, Disable, DreamEater ]
    }

    Norman => {
        "NORMAN"
        17 Slowpoke [ Tackle, Growl, WaterGun ]
        20 Slowpoke [ Curse, BodySlam, WaterGun, Confusion ]
    }

    Mark => {
        "MARK"
        13 Abra    [ Teleport, Flash ]
        13 Abra    [ Teleport, Flash ]
        15 Kadabra [ Teleport, Kinesis, Confusion ]
    }

    Phil => {
        "PHIL"
        24 Natu    [ Leer, NightShade, FutureSight, ConfuseRay ]
        26 Kadabra [ Disable, Psybeam, Recover, FutureSight ]
    }

    Richard => {
        "RICHARD"
        36 Espeon
    }

    Gilbert => {
        "GILBERT"
        30 Starmie
        30 Exeggcute
        34 Girafarig
    }

    Jared => {
        "JARED"
        32 MrMime
        32 Exeggcute
        35 Exeggcute
    }

    Rodney => {
        "RODNEY"
        29 Drowzee
        33 Hypno
    }

    Liz1 => {
        "LIZ"
        9 NidoranF
    }

    Gina1 => {
        "GINA"
         9 Hoppip
         9 Hoppip
        12 Bulbasaur
    }

    Brooke => {
        "BROOKE"
        16 Pikachu [ Thundershock, Growl, QuickAttack, DoubleTeam ]
    }

    Kim => {
        "KIM"
        15 Vulpix
    }

    Cindy => {
        "CINDY"
        36 Nidoqueen
    }

    Hope => {
        "HOPE"
        34 Flaaffy
    }

    Sharon => {
        "SHARON"
        31 Furret
        33 Rapidash
    }

    Debra => {
        "DEBRA"
        33 Seaking
    }

    Gina2 => {
        "GINA"
        14 Hoppip
        14 Hoppip
        17 Ivysaur
    }

    Erin1 => {
        "ERIN"
        16 Ponyta
        16 Ponyta
    }

    Liz2 => {
        "LIZ"
        15 Weepinbell
        15 Nidorina
    }

    Liz3 => {
        "LIZ"
        19 Weepinbell
        19 Nidorino
        21 Nidoqueen
    }

    Heidi => {
        "HEIDI"
        32 Skiploom
        32 Skiploom
    }

    Edna => {
        "EDNA"
        30 Nidorina
        34 Raichu
    }

    Gina3 => {
        "GINA"
        26 Skiploom
        26 Skiploom
        29 Ivysaur
    }

    Tiffany1 => {
        "TIFFANY"
        31 Clefairy [ Encore, Sing, Doubleslap, Minimize ]
    }

    Tiffany2 => {
        "TIFFANY"
        37 Clefairy [ Encore, Doubleslap, Minimize, Metronome ]
    }

    Erin2 => {
        "ERIN"
        32 Ponyta
        32 Ponyta
    }

    Tanya => {
        "TANYA"
        37 Exeggutor
    }

    Tiffany3 => {
        "TIFFANY"
        20 Clefairy [ Encore, Sing, Doubleslap, Minimize ]
    }

    Erin3 => {
        "ERIN"
        36 Ponyta [ DoubleTeam, Stomp, FireSpin, SunnyDay ]
        34 Raichu [ Swift, MudSlap, QuickAttack, Thunderbolt ]
        36 Ponyta [ DoubleTeam, Stomp, FireSpin, SunnyDay ]
    }

    Liz4 => {
        "LIZ"
        24 Weepinbell
        26 Nidorino
        26 Nidoqueen
    }

    Liz5 => {
        "LIZ"
        30 Weepinbell [ SleepPowder, Poisonpowder, StunSpore, SludgeBomb ]
        32 Nidoking   [ Earthquake, DoubleKick, PoisonSting, IronTail ]
        32 Nidoqueen  [ Earthquake, DoubleKick, TailWhip, BodySlam ]
    }

    Gina4 => {
        "GINA"
        30 Skiploom
        30 Skiploom
        32 Ivysaur
    }

    Gina5 => {
        "GINA"
        33 Jumpluff [ StunSpore, SunnyDay, LeechSeed, CottonSpore ]
        33 Jumpluff [ SunnyDay, SleepPowder, LeechSeed, CottonSpore ]
        38 Venusaur [ Solarbeam, RazorLeaf, Headbutt, MudSlap ]
    }

    Tiffany4 => {
        "TIFFANY"
        43 Clefairy [ Metronome, Encore, Moonlight, Minimize ]
    }

    Roland => {
        "ROLAND"
        9 NidoranM
    }

    Todd1 => {
        "TODD"
        14 Psyduck
    }

    Ivan => {
        "IVAN"
        10 Diglett
        10 Zubat
        14 Diglett
    }

    Elliot => {
        "ELLIOT"
        13 Sandshrew
        15 Marill
    }

    Barry => {
        "BARRY"
        36 Nidoking
    }

    Lloyd => {
        "LLOYD"
        34 Nidoking
    }

    Dean => {
        "DEAN"
        33 Golduck
        31 Sandslash
    }

    Sid => {
        "SID"
        32 Dugtrio
        29 Primeape
        29 Poliwrath
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
        17 Mankey
    }

    Todd2 => {
        "TODD"
        17 Geodude
        17 Geodude
        23 Psyduck
    }

    Todd3 => {
        "TODD"
        23 Geodude
        23 Geodude
        26 Psyduck
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
        37 Sandslash
    }

    Spencer => {
        "SPENCER"
        17 Sandshrew
        17 Sandslash
        19 Zubat
    }

    Todd4 => {
        "TODD"
        30 Graveler
        30 Graveler
        30 Slugma
        32 Psyduck
    }

    Todd5 => {
        "TODD"
        33 Graveler [ Selfdestruct, RockThrow, Harden, Magnitude ]
        33 Graveler [ Selfdestruct, RockThrow, Harden, Magnitude ]
        36 Magcargo [ RockThrow, Harden, Amnesia, Flamethrower ]
        34 Golduck  [ Disable, PsychicM, Surf, PsychUp ]
    }

    Quentin => {
        "QUENTIN"
        30 Fearow
        30 Primeape
        30 Tauros
    }

    ExecutiveF1 => {
        "EXECUTIVE"
        32 Arbok     [ Wrap, PoisonSting, Bite, Glare ]
        32 Vileplume [ Absorb, SweetScent, SleepPowder, Acid ]
        32 Murkrow   [ Peck, Pursuit, Haze, NightShade ]
    }

    ExecutiveF2 => {
        "EXECUTIVE"
        23 Arbok   [ Wrap, Leer, PoisonSting, Bite ]
        23 Gloom   [ Absorb, SweetScent, SleepPowder, Acid ]
        25 Murkrow [ Peck, Pursuit, Haze ]
    }

    Chow => {
        "CHOW"
        3 Bellsprout
        3 Bellsprout
        3 Bellsprout
    }

    Nico => {
        "NICO"
        3 Bellsprout
        3 Bellsprout
        3 Bellsprout
    }

    Jin => {
        "JIN"
        6 Bellsprout
    }

    Troy => {
        "TROY"
        7 Bellsprout
        7 Hoothoot
    }

    Jeffrey => {
        "JEFFREY"
        22 Haunter
    }

    Ping => {
        "PING"
        16 Gastly
        16 Gastly
        16 Gastly
        16 Gastly
        16 Gastly
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
         7 Bellsprout
         7 Bellsprout
        10 Hoothoot
    }

    Gaku => {
        "GAKU"
        32 Noctowl
        32 Flareon
    }

    Masa => {
        "MASA"
        32 Noctowl
        32 Jolteon
    }

    Koji => {
        "KOJI"
        32 Noctowl
        32 Vaporeon
    }

    Martha => {
        "MARTHA"
        18 Gastly
        20 Haunter
        20 Gastly
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
        35 Drowzee
        35 Hypno
    }

    Doris => {
        "DORIS"
        34 Slowpoke
        36 Slowbro
    }

    Ronald => {
        "RONALD"
        24 Seel
        25 Dewgong
        24 Seel
    }

    Brad => {
        "BRAD"
        26 Swinub
        26 Swinub
    }

    Douglas => {
        "DOUGLAS"
        24 Shellder
        25 Cloyster
        24 Shellder
    }

    William => {
        "WILLIAM"
        14 Raichu + Berry
    }

    Derek1 => {
        "DEREK"
        17 Pikachu + Berry
    }

    Robert => {
        "ROBERT"
        33 Quagsire + Berry
    }

    Joshua => {
        "JOSHUA"
        23 Pikachu + Berry
        23 Pikachu + Berry
        23 Pikachu + Berry
        23 Pikachu + Berry
        23 Pikachu + Berry
        23 Pikachu + Berry
    }

    Carter => {
        "CARTER"
        29 Bulbasaur + Berry
        29 Charmander + Berry
        29 Squirtle + Berry
    }

    Trevor => {
        "TREVOR"
        33 Psyduck + Berry
    }

    Brandon => {
        "BRANDON"
        13 Snubbull + Berry
    }

    Jeremy => {
        "JEREMY"
        28 Meowth + Berry
        28 Meowth + Berry
        28 Meowth + Berry
    }

    Colin => {
        "COLIN"
        32 Delibird + Berry
    }

    Derek2 => {
        "DEREK"
        19 Pikachu + Berry
    }

    Derek3 => {
        "DEREK"
        36 Pikachu + Berry
    }

    Alex => {
        "ALEX"
        29 Nidoking + Berry
        29 Slowking + Berry
        29 Seaking + Berry
    }

    Rex => {
        "REX"
        35 Phanpy + Berry
    }

    Allan => {
        "ALLAN"
        35 Teddiursa + Berry
    }

    NaokoUnused => {
        "NAOKO"
        20 Skiploom
        20 Vulpix
        18 Skiploom
    }

    Naoko => {
        "NAOKO"
        17 Flareon
    }

    Sayo => {
        "SAYO"
        17 Espeon
    }

    Zuki => {
        "ZUKI"
        17 Umbreon
    }

    Kuni => {
        "KUNI"
        17 Vaporeon
    }

    Miki => {
        "MIKI"
        17 Jolteon
    }

    AmyAndMay1 => {
        "AMY & MAY"
        10 Spinarak
        10 Ledyba
    }

    AnnAndAnne1 => {
        "ANN & ANNE"
        16 Clefairy   [ Growl, Encore, Doubleslap, Metronome ]
        16 Jigglypuff [ Sing, DefenseCurl, Pound, Disable ]
    }

    AnnAndAnne2 => {
        "ANN & ANNE"
        16 Jigglypuff [ Sing, DefenseCurl, Pound, Disable ]
        16 Clefairy   [ Growl, Encore, Doubleslap, Metronome ]
    }

    AmyAndMay2 => {
        "AMY & MAY"
        10 Ledyba
        10 Spinarak
    }

    JoAndZoe1 => {
        "JO & ZOE"
        35 Victreebel
        35 Vileplume
    }

    JoAndZoe2 => {
        "JO & ZOE"
        35 Vileplume
        35 Victreebel
    }

    MegAndPeg1 => {
        "MEG & PEG"
        31 Teddiursa
        31 Phanpy
    }

    MegAndPeg2 => {
        "MEG & PEG"
        31 Phanpy
        31 Teddiursa
    }

    LeaAndPia1 => {
        "LEA & PIA"
        35 Dratini [ ThunderWave, Twister, Flamethrower, Headbutt ]
        35 Dratini [ ThunderWave, Twister, IceBeam, Headbutt ]
    }

    LeaAndPia2 => {
        "LEA & PIA"
        38 Dratini [ ThunderWave, Twister, IceBeam, Headbutt ]
        38 Dratini [ ThunderWave, Twister, Flamethrower, Headbutt ]
    }

    Beverly1 => {
        "BEVERLY"
        14 Snubbull + Berry
    }

    Ruth => {
        "RUTH"
        17 Pikachu + Berry
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
        23 Sentret + Berry
        23 Sentret + Berry
        23 Sentret + Berry
        28 Furret + Berry
        23 Sentret + Berry
    }

    Jaime => {
        "JAIME"
        16 Meowth + Berry
    }

    Red1 => {
        "RED"
        81 Pikachu   [ Charm, QuickAttack, Thunderbolt, Thunder ]
        73 Espeon    [ MudSlap, Reflect, Swift, PsychicM ]
        75 Snorlax   [ Amnesia, Snore, Rest, BodySlam ]
        77 Venusaur  [ SunnyDay, GigaDrain, Synthesis, Solarbeam ]
        77 Charizard [ Flamethrower, WingAttack, Slash, FireSpin ]
        77 Blastoise [ RainDance, Surf, Blizzard, Whirlpool ]
    }

    Blue1 => {
        "BLUE"
        56 Pidgeot   [ QuickAttack, Whirlwind, WingAttack, MirrorMove ]
        54 Alakazam  [ Disable, Recover, PsychicM, Reflect ]
        56 Rhydon    [ FuryAttack, Sandstorm, RockSlide, Earthquake ]
        58 Gyarados  [ Twister, HydroPump, RainDance, HyperBeam ]
        58 Exeggutor [ SunnyDay, LeechSeed, EggBomb, Solarbeam ]
        58 Arcanine  [ Roar, Swift, Flamethrower, Extremespeed ]
    }

    Keith => {
        "KEITH"
        17 Growlithe
    }

    Dirk => {
        "DIRK"
        14 Growlithe
        14 Growlithe
    }

    GruntF1 => {
        "GRUNT"
         9 Zubat
        11 Ekans
    }

    GruntF2 => {
        "GRUNT"
        26 Arbok
    }

    GruntF3 => {
        "GRUNT"
        25 Gloom
        25 Gloom
    }

    GruntF4 => {
        "GRUNT"
        21 Ekans
        23 Oddish
        21 Ekans
        24 Gloom
    }

    GruntF5 => {
        "GRUNT"
        18 Ekans [ Wrap, Leer, PoisonSting, Bite ]
        18 Gloom [ Absorb, SweetScent, StunSpore, SleepPowder ]
    }

    Eusine => {
        "EUSINE"
        23 Drowzee   [ DreamEater, Hypnosis, Disable, Confusion ]
        23 Haunter   [ Lick, Hypnosis, MeanLook, Curse ]
        25 Electrode [ Screech, Sonicboom, Thunder, Rollout ]
    }
}
