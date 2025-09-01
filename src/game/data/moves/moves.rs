use crate::game::{
    constants::{move_constants::Move, move_effect_constants::MoveEffect, type_constants::Type},
    macros::data::percent,
};

macro_rules! define_moves_enum {
    (
        $(
            $name:ident [
                $effect:ident,
                $power:expr,
                $ty:ident,
                $accuracy:expr,
                $pp:expr,
                $eff_chance:expr
            ]
        ),* $(,)?
    ) => {
        impl Move {
            pub fn effect(self) -> MoveEffect {
                match self {
                    $( Move::$name => MoveEffect::$effect, )*
                    Move::Unknown(n) => {
                        log::error!("Unknown move effect for move: {n}");
                        0.into()
                    }
                }
            }

            pub fn base_power(self) -> u8 {
                match self {
                    $( Move::$name => $power, )*
                    Move::Unknown(n) => {
                        log::error!("Unknown base power for move: {n}");
                        0
                    }
                }
            }

            pub fn r#type(self) -> Type {
                match self {
                    $( Move::$name => Type::$ty, )*
                    Move::Unknown(n) => {
                        log::error!("Unknown type for move: {n}");
                        Type::Normal
                    }
                }
            }

            pub fn accuracy(self) -> u8 {
                match self {
                    $( Move::$name => percent($accuracy), )*
                    Move::Unknown(n) => {
                        log::error!("Unknown accuracy for move: {n}");
                        0
                    }
                }
            }

            pub fn pp(self) -> u8 {
                match self {
                    $( Move::$name => $pp, )*
                    _ => {
                        log::error!("Unknown PP for move: {:?}", self);
                        0
                    }
                }
            }

            pub fn effect_chance(self) -> u8 {
                match self {
                    $( Move::$name => percent($eff_chance), )*
                    _ => {
                        log::error!("Unknown effect chance for move: {:?}", self);
                        0
                    }
                }
            }
        }
    };
}

define_moves_enum! {
    Pound          [ NormalHit,        40, Normal,       100, 35,   0 ],
    KarateChop     [ NormalHit,        50, Fighting,     100, 25,   0 ],
    Doubleslap     [ MultiHit,         15, Normal,        85, 10,   0 ],
    CometPunch     [ MultiHit,         18, Normal,        85, 15,   0 ],
    MegaPunch      [ NormalHit,        80, Normal,        85, 20,   0 ],
    PayDay         [ PayDay,           40, Normal,       100, 20,   0 ],
    FirePunch      [ BurnHit,          75, Fire,         100, 15,  10 ],
    IcePunch       [ FreezeHit,        75, Ice,          100, 15,  10 ],
    Thunderpunch   [ ParalyzeHit,      75, Electric,     100, 15,  10 ],
    Scratch        [ NormalHit,        40, Normal,       100, 35,   0 ],
    Vicegrip       [ NormalHit,        55, Normal,       100, 30,   0 ],
    Guillotine     [ Ohko,              0, Normal,        30,  5,   0 ],
    RazorWind      [ RazorWind,        80, Normal,        75, 10,   0 ],
    SwordsDance    [ AttackUp2,         0, Normal,       100, 30,   0 ],
    Cut            [ NormalHit,        50, Normal,        95, 30,   0 ],
    Gust           [ Gust,             40, Flying,       100, 35,   0 ],
    WingAttack     [ NormalHit,        60, Flying,       100, 35,   0 ],
    Whirlwind      [ ForceSwitch,       0, Normal,       100, 20,   0 ],
    Fly            [ Fly,              70, Flying,        95, 15,   0 ],
    Bind           [ TrapTarget,       15, Normal,        75, 20,   0 ],
    Slam           [ NormalHit,        80, Normal,        75, 20,   0 ],
    VineWhip       [ NormalHit,        35, Grass,        100, 10,   0 ],
    Stomp          [ Stomp,            65, Normal,       100, 20,  30 ],
    DoubleKick     [ DoubleHit,        30, Fighting,     100, 30,   0 ],
    MegaKick       [ NormalHit,       120, Normal,        75,  5,   0 ],
    JumpKick       [ JumpKick,         70, Fighting,      95, 25,   0 ],
    RollingKick    [ FlinchHit,        60, Fighting,      85, 15,  30 ],
    SandAttack     [ AccuracyDown,      0, Ground,       100, 15,   0 ],
    Headbutt       [ FlinchHit,        70, Normal,       100, 15,  30 ],
    HornAttack     [ NormalHit,        65, Normal,       100, 25,   0 ],
    FuryAttack     [ MultiHit,         15, Normal,        85, 20,   0 ],
    HornDrill      [ Ohko,              1, Normal,        30,  5,   0 ],
    Tackle         [ NormalHit,        35, Normal,        95, 35,   0 ],
    BodySlam       [ ParalyzeHit,      85, Normal,       100, 15,  30 ],
    Wrap           [ TrapTarget,       15, Normal,        85, 20,   0 ],
    TakeDown       [ RecoilHit,        90, Normal,        85, 20,   0 ],
    Thrash         [ Rampage,          90, Normal,       100, 20,   0 ],
    DoubleEdge     [ RecoilHit,       120, Normal,       100, 15,   0 ],
    TailWhip       [ DefenseDown,       0, Normal,       100, 30,   0 ],
    PoisonSting    [ PoisonHit,        15, Poison,       100, 35,  30 ],
    Twineedle      [ PoisonMultiHit,   25, Bug,          100, 20,  20 ],
    PinMissile     [ MultiHit,         14, Bug,           85, 20,   0 ],
    Leer           [ DefenseDown,       0, Normal,       100, 30,   0 ],
    Bite           [ FlinchHit,        60, Dark,         100, 25,  30 ],
    Growl          [ AttackDown,        0, Normal,       100, 40,   0 ],
    Roar           [ ForceSwitch,       0, Normal,       100, 20,   0 ],
    Sing           [ Sleep,             0, Normal,        55, 15,   0 ],
    Supersonic     [ Confuse,           0, Normal,        55, 20,   0 ],
    Sonicboom      [ StaticDamage,     20, Normal,        90, 20,   0 ],
    Disable        [ Disable,           0, Normal,        55, 20,   0 ],
    Acid           [ DefenseDownHit,   40, Poison,       100, 30,  10 ],
    Ember          [ BurnHit,          40, Fire,         100, 25,  10 ],
    Flamethrower   [ BurnHit,          95, Fire,         100, 15,  10 ],
    Mist           [ Mist,              0, Ice,          100, 30,   0 ],
    WaterGun       [ NormalHit,        40, Water,        100, 25,   0 ],
    HydroPump      [ NormalHit,       120, Water,         80,  5,   0 ],
    Surf           [ NormalHit,        95, Water,        100, 15,   0 ],
    IceBeam        [ FreezeHit,        95, Ice,          100, 10,  10 ],
    Blizzard       [ FreezeHit,       120, Ice,           70,  5,  10 ],
    Psybeam        [ ConfuseHit,       65, Psychic,      100, 20,  10 ],
    Bubblebeam     [ SpeedDownHit,     65, Water,        100, 20,  10 ],
    AuroraBeam     [ AttackDownHit,    65, Ice,          100, 20,  10 ],
    HyperBeam      [ HyperBeam,       150, Normal,        90,  5,   0 ],
    Peck           [ NormalHit,        35, Flying,       100, 35,   0 ],
    DrillPeck      [ NormalHit,        80, Flying,       100, 20,   0 ],
    Submission     [ RecoilHit,        80, Fighting,      80, 25,   0 ],
    LowKick        [ FlinchHit,        50, Fighting,      90, 20,  30 ],
    Counter        [ Counter,           1, Fighting,     100, 20,   0 ],
    SeismicToss    [ LevelDamage,       1, Fighting,     100, 20,   0 ],
    Strength       [ NormalHit,        80, Normal,       100, 15,   0 ],
    Absorb         [ LeechHit,         20, Grass,        100, 20,   0 ],
    MegaDrain      [ LeechHit,         40, Grass,        100, 10,   0 ],
    LeechSeed      [ LeechSeed,         0, Grass,         90, 10,   0 ],
    Growth         [ SpAtkUp,           0, Normal,       100, 40,   0 ],
    RazorLeaf      [ NormalHit,        55, Grass,         95, 25,   0 ],
    Solarbeam      [ Solarbeam,       120, Grass,        100, 10,   0 ],
    Poisonpowder   [ Poison,            0, Poison,        75, 35,   0 ],
    StunSpore      [ Paralyze,          0, Grass,         75, 30,   0 ],
    SleepPowder    [ Sleep,             0, Grass,         75, 15,   0 ],
    PetalDance     [ Rampage,          70, Grass,        100, 20,   0 ],
    StringShot     [ SpeedDown,         0, Bug,           95, 40,   0 ],
    DragonRage     [ StaticDamage,     40, Dragon,       100, 10,   0 ],
    FireSpin       [ TrapTarget,       15, Fire,          70, 15,   0 ],
    Thundershock   [ ParalyzeHit,      40, Electric,     100, 30,  10 ],
    Thunderbolt    [ ParalyzeHit,      95, Electric,     100, 15,  10 ],
    ThunderWave    [ Paralyze,          0, Electric,     100, 20,   0 ],
    Thunder        [ Thunder,         120, Electric,      70, 10,  30 ],
    RockThrow      [ NormalHit,        50, Rock,          90, 15,   0 ],
    Earthquake     [ Earthquake,      100, Ground,       100, 10,   0 ],
    Fissure        [ Ohko,              1, Ground,        30,  5,   0 ],
    Dig            [ Fly,              60, Ground,       100, 10,   0 ],
    Toxic          [ Toxic,             0, Poison,        85, 10,   0 ],
    Confusion      [ ConfuseHit,       50, Psychic,      100, 25,  10 ],
    PsychicM       [ SpDefDownHit,     90, Psychic,      100, 10,  10 ],
    Hypnosis       [ Sleep,             0, Psychic,       60, 20,   0 ],
    Meditate       [ AttackUp,          0, Psychic,      100, 40,   0 ],
    Agility        [ SpeedUp2,          0, Psychic,      100, 30,   0 ],
    QuickAttack    [ PriorityHit,      40, Normal,       100, 30,   0 ],
    Rage           [ Rage,             20, Normal,       100, 20,   0 ],
    Teleport       [ Teleport,          0, Psychic,      100, 20,   0 ],
    NightShade     [ LevelDamage,       1, Ghost,        100, 15,   0 ],
    Mimic          [ Mimic,             0, Normal,       100, 10,   0 ],
    Screech        [ DefenseDown2,      0, Normal,        85, 40,   0 ],
    DoubleTeam     [ EvasionUp,         0, Normal,       100, 15,   0 ],
    Recover        [ Heal,              0, Normal,       100, 20,   0 ],
    Harden         [ DefenseUp,         0, Normal,       100, 30,   0 ],
    Minimize       [ EvasionUp,         0, Normal,       100, 20,   0 ],
    Smokescreen    [ AccuracyDown,      0, Normal,       100, 20,   0 ],
    ConfuseRay     [ Confuse,           0, Ghost,        100, 10,   0 ],
    Withdraw       [ DefenseUp,         0, Water,        100, 40,   0 ],
    DefenseCurl    [ DefenseCurl,       0, Normal,       100, 40,   0 ],
    Barrier        [ DefenseUp2,        0, Psychic,      100, 30,   0 ],
    LightScreen    [ LightScreen,       0, Psychic,      100, 30,   0 ],
    Haze           [ ResetStats,        0, Ice,          100, 30,   0 ],
    Reflect        [ Reflect,           0, Psychic,      100, 20,   0 ],
    FocusEnergy    [ FocusEnergy,       0, Normal,       100, 30,   0 ],
    Bide           [ Bide,              0, Normal,       100, 10,   0 ],
    Metronome      [ Metronome,         0, Normal,       100, 10,   0 ],
    MirrorMove     [ MirrorMove,        0, Flying,       100, 20,   0 ],
    Selfdestruct   [ Selfdestruct,    200, Normal,       100,  5,   0 ],
    EggBomb        [ NormalHit,       100, Normal,        75, 10,   0 ],
    Lick           [ ParalyzeHit,      20, Ghost,        100, 30,  30 ],
    Smog           [ PoisonHit,        20, Poison,        70, 20,  40 ],
    Sludge         [ PoisonHit,        65, Poison,       100, 20,  30 ],
    BoneClub       [ FlinchHit,        65, Ground,        85, 20,  10 ],
    FireBlast      [ BurnHit,         120, Fire,          85,  5,  10 ],
    Waterfall      [ NormalHit,        80, Water,        100, 15,   0 ],
    Clamp          [ TrapTarget,       35, Water,         75, 10,   0 ],
    Swift          [ AlwaysHit,        60, Normal,       100, 20,   0 ],
    SkullBash      [ SkullBash,       100, Normal,       100, 15,   0 ],
    SpikeCannon    [ MultiHit,         20, Normal,       100, 15,   0 ],
    Constrict      [ SpeedDownHit,     10, Normal,       100, 35,  10 ],
    Amnesia        [ SpDefUp2,          0, Psychic,      100, 20,   0 ],
    Kinesis        [ AccuracyDown,      0, Psychic,       80, 15,   0 ],
    Softboiled     [ Heal,              0, Normal,       100, 10,   0 ],
    HiJumpKick     [ JumpKick,         85, Fighting,      90, 20,   0 ],
    Glare          [ Paralyze,          0, Normal,        75, 30,   0 ],
    DreamEater     [ DreamEater,      100, Psychic,      100, 15,   0 ],
    PoisonGas      [ Poison,            0, Poison,        55, 40,   0 ],
    Barrage        [ MultiHit,         15, Normal,        85, 20,   0 ],
    LeechLife      [ LeechHit,         20, Bug,          100, 15,   0 ],
    LovelyKiss     [ Sleep,             0, Normal,        75, 10,   0 ],
    SkyAttack      [ SkyAttack,       140, Flying,        90,  5,   0 ],
    Transform      [ Transform,         0, Normal,       100, 10,   0 ],
    Bubble         [ SpeedDownHit,     20, Water,        100, 30,  10 ],
    DizzyPunch     [ ConfuseHit,       70, Normal,       100, 10,  20 ],
    Spore          [ Sleep,             0, Grass,        100, 15,   0 ],
    Flash          [ AccuracyDown,      0, Normal,        70, 20,   0 ],
    Psywave        [ Psywave,           1, Psychic,       80, 15,   0 ],
    Splash         [ Splash,            0, Normal,       100, 40,   0 ],
    AcidArmor      [ DefenseUp2,        0, Poison,       100, 40,   0 ],
    Crabhammer     [ NormalHit,        90, Water,         85, 10,   0 ],
    Explosion      [ Selfdestruct,    250, Normal,       100,  5,   0 ],
    FurySwipes     [ MultiHit,         18, Normal,        80, 15,   0 ],
    Bonemerang     [ DoubleHit,        50, Ground,        90, 10,   0 ],
    Rest           [ Heal,              0, Psychic,      100, 10,   0 ],
    RockSlide      [ FlinchHit,        75, Rock,          90, 10,  30 ],
    HyperFang      [ FlinchHit,        80, Normal,        90, 15,  10 ],
    Sharpen        [ AttackUp,          0, Normal,       100, 30,   0 ],
    Conversion     [ Conversion,        0, Normal,       100, 30,   0 ],
    TriAttack      [ TriAttack,        80, Normal,       100, 10,  20 ],
    SuperFang      [ SuperFang,         1, Normal,        90, 10,   0 ],
    Slash          [ NormalHit,        70, Normal,       100, 20,   0 ],
    Substitute     [ Substitute,        0, Normal,       100, 10,   0 ],
    Struggle       [ RecoilHit,        50, Normal,       100,  1,   0 ],
    Sketch         [ Sketch,            0, Normal,       100,  1,   0 ],
    TripleKick     [ TripleKick,       10, Fighting,      90, 10,   0 ],
    Thief          [ Thief,            40, Dark,         100, 10, 100 ],
    SpiderWeb      [ MeanLook,          0, Bug,          100, 10,   0 ],
    MindReader     [ LockOn,            0, Normal,       100,  5,   0 ],
    Nightmare      [ Nightmare,         0, Ghost,        100, 15,   0 ],
    FlameWheel     [ FlameWheel,       60, Fire,         100, 25,  10 ],
    Snore          [ Snore,            40, Normal,       100, 15,  30 ],
    Curse          [ Curse,             0, Curse,        100, 10,   0 ],
    Flail          [ Reversal,          1, Normal,       100, 15,   0 ],
    Conversion2    [ Conversion2,       0, Normal,       100, 30,   0 ],
    Aeroblast      [ NormalHit,       100, Flying,        95,  5,   0 ],
    CottonSpore    [ SpeedDown2,        0, Grass,         85, 40,   0 ],
    Reversal       [ Reversal,          1, Fighting,     100, 15,   0 ],
    Spite          [ Spite,             0, Ghost,        100, 10,   0 ],
    PowderSnow     [ FreezeHit,        40, Ice,          100, 25,  10 ],
    Protect        [ Protect,           0, Normal,       100, 10,   0 ],
    MachPunch      [ PriorityHit,      40, Fighting,     100, 30,   0 ],
    ScaryFace      [ SpeedDown2,        0, Normal,        90, 10,   0 ],
    FaintAttack    [ AlwaysHit,        60, Dark,         100, 20,   0 ],
    SweetKiss      [ Confuse,           0, Normal,        75, 10,   0 ],
    BellyDrum      [ BellyDrum,         0, Normal,       100, 10,   0 ],
    SludgeBomb     [ PoisonHit,        90, Poison,       100, 10,  30 ],
    MudSlap        [ AccuracyDownHit,  20, Ground,       100, 10, 100 ],
    Octazooka      [ AccuracyDownHit,  65, Water,         85, 10,  50 ],
    Spikes         [ Spikes,            0, Ground,       100, 20,   0 ],
    ZapCannon      [ ParalyzeHit,     100, Electric,      50,  5, 100 ],
    Foresight      [ Foresight,         0, Normal,       100, 40,   0 ],
    DestinyBond    [ DestinyBond,       0, Ghost,        100,  5,   0 ],
    PerishSong     [ PerishSong,        0, Normal,       100,  5,   0 ],
    IcyWind        [ SpeedDownHit,     55, Ice,           95, 15, 100 ],
    Detect         [ Protect,           0, Fighting,     100,  5,   0 ],
    BoneRush       [ MultiHit,         25, Ground,        80, 10,   0 ],
    LockOn         [ LockOn,            0, Normal,       100,  5,   0 ],
    Outrage        [ Rampage,          90, Dragon,       100, 15,   0 ],
    Sandstorm      [ Sandstorm,         0, Rock,         100, 10,   0 ],
    GigaDrain      [ LeechHit,         60, Grass,        100,  5,   0 ],
    Endure         [ Endure,            0, Normal,       100, 10,   0 ],
    Charm          [ AttackDown2,       0, Normal,       100, 20,   0 ],
    Rollout        [ Rollout,          30, Rock,          90, 20,   0 ],
    FalseSwipe     [ FalseSwipe,       40, Normal,       100, 40,   0 ],
    Swagger        [ Swagger,           0, Normal,        90, 15, 100 ],
    MilkDrink      [ Heal,              0, Normal,       100, 10,   0 ],
    Spark          [ ParalyzeHit,      65, Electric,     100, 20,  30 ],
    FuryCutter     [ FuryCutter,       10, Bug,           95, 20,   0 ],
    SteelWing      [ DefenseUpHit,     70, Steel,         90, 25,  10 ],
    MeanLook       [ MeanLook,          0, Normal,       100,  5,   0 ],
    Attract        [ Attract,           0, Normal,       100, 15,   0 ],
    SleepTalk      [ SleepTalk,         0, Normal,       100, 10,   0 ],
    HealBell       [ HealBell,          0, Normal,       100,  5,   0 ],
    Return         [ Return,            1, Normal,       100, 20,   0 ],
    Present        [ Present,           1, Normal,        90, 15,   0 ],
    Frustration    [ Frustration,       1, Normal,       100, 20,   0 ],
    Safeguard      [ Safeguard,         0, Normal,       100, 25,   0 ],
    PainSplit      [ PainSplit,         0, Normal,       100, 20,   0 ],
    SacredFire     [ SacredFire,      100, Fire,          95,  5,  50 ],
    Magnitude      [ Magnitude,         1, Ground,       100, 30,   0 ],
    Dynamicpunch   [ ConfuseHit,      100, Fighting,      50,  5, 100 ],
    Megahorn       [ NormalHit,       120, Bug,           85, 10,   0 ],
    Dragonbreath   [ ParalyzeHit,      60, Dragon,       100, 20,  30 ],
    BatonPass      [ BatonPass,         0, Normal,       100, 40,   0 ],
    Encore         [ Encore,            0, Normal,       100,  5,   0 ],
    Pursuit        [ Pursuit,          40, Dark,         100, 20,   0 ],
    RapidSpin      [ RapidSpin,        20, Normal,       100, 40,   0 ],
    SweetScent     [ EvasionDown,       0, Normal,       100, 20,   0 ],
    IronTail       [ DefenseDownHit,  100, Steel,         75, 15,  30 ],
    MetalClaw      [ AttackUpHit,      50, Steel,         95, 35,  10 ],
    VitalThrow     [ AlwaysHit,        70, Fighting,     100, 10,   0 ],
    MorningSun     [ MorningSun,        0, Normal,       100,  5,   0 ],
    Synthesis      [ Synthesis,         0, Grass,        100,  5,   0 ],
    Moonlight      [ Moonlight,         0, Normal,       100,  5,   0 ],
    HiddenPower    [ HiddenPower,       1, Normal,       100, 15,   0 ],
    CrossChop      [ NormalHit,       100, Fighting,      80,  5,   0 ],
    Twister        [ Twister,          40, Dragon,       100, 20,  20 ],
    RainDance      [ RainDance,         0, Water,         90,  5,   0 ],
    SunnyDay       [ SunnyDay,          0, Fire,          90,  5,   0 ],
    Crunch         [ SpDefDownHit,     80, Dark,         100, 15,  20 ],
    MirrorCoat     [ MirrorCoat,        1, Psychic,      100, 20,   0 ],
    PsychUp        [ PsychUp,           0, Normal,       100, 10,   0 ],
    Extremespeed   [ PriorityHit,      80, Normal,       100,  5,   0 ],
    Ancientpower   [ AllUpHit,         60, Rock,         100,  5,  10 ],
    ShadowBall     [ SpDefDownHit,     80, Ghost,        100, 15,  20 ],
    FutureSight    [ FutureSight,      80, Psychic,       90, 15,   0 ],
    RockSmash      [ DefenseDownHit,   20, Fighting,     100, 15,  50 ],
    Whirlpool      [ TrapTarget,       15, Water,         70, 15,   0 ],
    BeatUp         [ BeatUp,           10, Dark,         100, 10,   0 ],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moves() {
        assert_eq!(u8::from(Move::BatonPass), 0xe2);
        assert_eq!(Move::BatonPass.effect(), MoveEffect::BatonPass);
        assert_eq!(Move::BatonPass.base_power(), 0);
        assert_eq!(Move::BatonPass.r#type(), Type::Normal);
        assert_eq!(Move::BatonPass.accuracy(), percent(100));
        assert_eq!(Move::BatonPass.pp(), 40);
        assert_eq!(Move::BatonPass.effect_chance(), percent(0));

        assert_eq!(u8::from(Move::RockSmash), 0xf9);
        assert_eq!(Move::RockSmash.effect(), MoveEffect::DefenseDownHit);
        assert_eq!(Move::RockSmash.base_power(), 20);
        assert_eq!(Move::RockSmash.r#type(), Type::Fighting);
        assert_eq!(Move::RockSmash.accuracy(), percent(100));
        assert_eq!(Move::RockSmash.pp(), 15);
        assert_eq!(Move::RockSmash.effect_chance(), percent(50));
    }
}
