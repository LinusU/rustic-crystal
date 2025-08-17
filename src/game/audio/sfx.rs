use pokemon_synthesizer::gen2::SoundIterator;
use rodio::Source;

use crate::{rom::ROM, sound2::Sfx as SfxTrait};

#[derive(Debug, Clone, Copy)]
pub struct Sfx {
    bank: u8,
    addr: u16,
    pitch: i16,
    length: u16,
}

impl Sfx {
    pub const fn new(bank: u8, addr: u16) -> Self {
        Self {
            bank,
            addr,
            pitch: 0,
            length: 0x100,
        }
    }

    #[rustfmt::skip]
    pub fn from_sfx_id(id: u8) -> Option<Sfx> {
        match id {
            0x00 => None, // Sfx_DexFanfare5079
            0x01 => None, // Sfx_Item
            0x02 => None, // Sfx_CaughtMon
            0x03 => Some(Sfx::new(0x3c, 0x4941)), // Sfx_PokeballsPlacedOnTable
            0x04 => Some(Sfx::new(0x3c, 0x4947)), // Sfx_Potion
            0x05 => Some(Sfx::new(0x3c, 0x494a)), // Sfx_FullHeal
            0x06 => Some(Sfx::new(0x3c, 0x494d)), // Sfx_Menu
            0x07 => Some(Sfx::new(0x3c, 0x4950)), // Sfx_ReadText
            0x08 => Some(Sfx::new(0x3c, 0x4950)), // Sfx_ReadText2
            0x09 => None, // Sfx_DexFanfare2049
            0x0a => None, // Sfx_DexFanfare80109
            0x0b => Some(Sfx::new(0x3c, 0x4953)), // Sfx_Poison
            0x0c => Some(Sfx::new(0x3c, 0x4956)), // Sfx_GotSafariBalls
            0x0d => Some(Sfx::new(0x3c, 0x4959)), // Sfx_BootPc
            0x0e => Some(Sfx::new(0x3c, 0x495c)), // Sfx_ShutDownPc
            0x0f => Some(Sfx::new(0x3c, 0x495f)), // Sfx_ChoosePcOption
            0x10 => Some(Sfx::new(0x3c, 0x4962)), // Sfx_EscapeRope
            0x11 => Some(Sfx::new(0x3c, 0x4965)), // Sfx_PushButton
            0x12 => Some(Sfx::new(0x3c, 0x4968)), // Sfx_SecondPartOfItemfinder
            0x13 => Some(Sfx::new(0x3c, 0x496b)), // Sfx_WarpTo
            0x14 => Some(Sfx::new(0x3c, 0x496e)), // Sfx_WarpFrom
            0x15 => Some(Sfx::new(0x3c, 0x4971)), // Sfx_ChangeDexMode
            0x16 => Some(Sfx::new(0x3c, 0x4974)), // Sfx_JumpOverLedge
            0x17 => Some(Sfx::new(0x3c, 0x4977)), // Sfx_GrassRustle
            0x18 => Some(Sfx::new(0x3c, 0x497a)), // Sfx_Fly
            0x19 => Some(Sfx::new(0x3c, 0x497d)), // Sfx_Wrong
            0x1a => Some(Sfx::new(0x3c, 0x4983)), // Sfx_Squeak
            0x1b => Some(Sfx::new(0x3c, 0x4986)), // Sfx_Strength
            0x1c => Some(Sfx::new(0x3c, 0x4989)), // Sfx_Boat
            0x1d => Some(Sfx::new(0x3c, 0x498f)), // Sfx_WallOpen
            0x1e => Some(Sfx::new(0x3c, 0x4992)), // Sfx_PlacePuzzlePieceDown
            0x1f => Some(Sfx::new(0x3c, 0x4995)), // Sfx_EnterDoor
            0x20 => Some(Sfx::new(0x3c, 0x4998)), // Sfx_SwitchPokemon
            0x21 => Some(Sfx::new(0x3c, 0x499e)), // Sfx_Tally
            0x22 => Some(Sfx::new(0x3c, 0x49a4)), // Sfx_Transaction
            0x23 => Some(Sfx::new(0x3c, 0x49ad)), // Sfx_ExitBuilding
            0x24 => Some(Sfx::new(0x3c, 0x49aa)), // Sfx_Bump
            0x25 => Some(Sfx::new(0x3c, 0x49b0)), // Sfx_Save
            0x26 => None, // Sfx_Pokeflute
            0x27 => Some(Sfx::new(0x3c, 0x49fb)), // Sfx_ElevatorEnd
            0x28 => Some(Sfx::new(0x3c, 0x49fe)), // Sfx_ThrowBall
            0x29 => Some(Sfx::new(0x3c, 0x4a04)), // Sfx_BallPoof
            0x2a => Some(Sfx::new(0x3c, 0x4a0a)), // Sfx_Faint
            0x2b => Some(Sfx::new(0x3c, 0x4a10)), // Sfx_Run
            0x2c => Some(Sfx::new(0x3c, 0x4a13)), // Sfx_SlotMachineStart
            0x2d => None, // Sfx_Fanfare
            0x2e => Some(Sfx::new(0x3c, 0x4a3d)), // Sfx_Peck
            0x2f => Some(Sfx::new(0x3c, 0x4a40)), // Sfx_Kinesis
            0x30 => Some(Sfx::new(0x3c, 0x4a43)), // Sfx_Lick
            0x31 => Some(Sfx::new(0x3c, 0x4a46)), // Sfx_Pound
            0x32 => Some(Sfx::new(0x3c, 0x4a49)), // Sfx_MovePuzzlePiece
            0x33 => Some(Sfx::new(0x3c, 0x4a4c)), // Sfx_CometPunch
            0x34 => Some(Sfx::new(0x3c, 0x4a4f)), // Sfx_MegaPunch
            0x35 => Some(Sfx::new(0x3c, 0x4a52)), // Sfx_Scratch
            0x36 => Some(Sfx::new(0x3c, 0x4a55)), // Sfx_Vicegrip
            0x37 => Some(Sfx::new(0x3c, 0x4a58)), // Sfx_RazorWind
            0x38 => Some(Sfx::new(0x3c, 0x4a5b)), // Sfx_Cut
            0x39 => Some(Sfx::new(0x3c, 0x4a5e)), // Sfx_WingAttack
            0x3a => Some(Sfx::new(0x3c, 0x4a61)), // Sfx_Whirlwind
            0x3b => Some(Sfx::new(0x3c, 0x4a64)), // Sfx_Bind
            0x3c => Some(Sfx::new(0x3c, 0x4a67)), // Sfx_VineWhip
            0x3d => Some(Sfx::new(0x3c, 0x4a6a)), // Sfx_DoubleKick
            0x3e => Some(Sfx::new(0x3c, 0x4a6d)), // Sfx_MegaKick
            0x3f => Some(Sfx::new(0x3c, 0x4a70)), // Sfx_Headbutt
            0x40 => Some(Sfx::new(0x3c, 0x4a73)), // Sfx_HornAttack
            0x41 => Some(Sfx::new(0x3c, 0x4a76)), // Sfx_Tackle
            0x42 => Some(Sfx::new(0x3c, 0x4a79)), // Sfx_PoisonSting
            0x43 => Some(Sfx::new(0x3c, 0x4a7c)), // Sfx_Powder
            0x44 => Some(Sfx::new(0x3c, 0x4a7f)), // Sfx_Doubleslap
            0x45 => Some(Sfx::new(0x3c, 0x4a82)), // Sfx_Bite
            0x46 => Some(Sfx::new(0x3c, 0x4a88)), // Sfx_JumpKick
            0x47 => Some(Sfx::new(0x3c, 0x4a8b)), // Sfx_Stomp
            0x48 => Some(Sfx::new(0x3c, 0x4a8e)), // Sfx_TailWhip
            0x49 => Some(Sfx::new(0x3c, 0x4a91)), // Sfx_KarateChop
            0x4a => Some(Sfx::new(0x3c, 0x4a94)), // Sfx_Submission
            0x4b => Some(Sfx::new(0x3c, 0x4a97)), // Sfx_WaterGun
            0x4c => Some(Sfx::new(0x3c, 0x4a9d)), // Sfx_SwordsDance
            0x4d => Some(Sfx::new(0x3c, 0x4aa0)), // Sfx_Thunder
            0x4e => Some(Sfx::new(0x3c, 0x4aa3)), // Sfx_Supersonic
            0x4f => Some(Sfx::new(0x3c, 0x4aac)), // Sfx_Leer
            0x50 => Some(Sfx::new(0x3c, 0x4ab5)), // Sfx_Ember
            0x51 => Some(Sfx::new(0x3c, 0x4abb)), // Sfx_Bubblebeam
            0x52 => Some(Sfx::new(0x3c, 0x4ac4)), // Sfx_HydroPump
            0x53 => Some(Sfx::new(0x3c, 0x4aca)), // Sfx_Surf
            0x54 => Some(Sfx::new(0x3c, 0x4ad3)), // Sfx_Psybeam
            0x55 => Some(Sfx::new(0x3c, 0x4adc)), // Sfx_Charge
            0x56 => Some(Sfx::new(0x3c, 0x4ae5)), // Sfx_Thundershock
            0x57 => Some(Sfx::new(0x3c, 0x4aee)), // Sfx_Psychic
            0x58 => Some(Sfx::new(0x3c, 0x4af7)), // Sfx_Screech
            0x59 => Some(Sfx::new(0x3c, 0x4afd)), // Sfx_BoneClub
            0x5a => Some(Sfx::new(0x3c, 0x4b03)), // Sfx_Sharpen
            0x5b => Some(Sfx::new(0x3c, 0x4b09)), // Sfx_EggBomb
            0x5c => None, // Sfx_Sing
            0x5d => Some(Sfx::new(0x3c, 0x4b18)), // Sfx_HyperBeam
            0x5e => Some(Sfx::new(0x3c, 0x4b21)), // Sfx_Shine
            0x5f => Some(Sfx::new(0x3c, 0x4b24)), // Sfx_Unknown5F
            0x60 => Some(Sfx::new(0x3c, 0x4a1c)), // Sfx_Unknown60
            0x61 => Some(Sfx::new(0x3c, 0x4a1f)), // Sfx_Unknown61
            0x62 => Some(Sfx::new(0x3c, 0x4a22)), // Sfx_SwitchPockets
            0x63 => Some(Sfx::new(0x3c, 0x4a25)), // Sfx_Unknown63
            0x64 => Some(Sfx::new(0x3c, 0x4a28)), // Sfx_Burn
            0x65 => Some(Sfx::new(0x3c, 0x4a2b)), // Sfx_TitleScreenEntrance
            0x66 => Some(Sfx::new(0x3c, 0x4a2e)), // Sfx_Unknown66
            0x67 => Some(Sfx::new(0x3c, 0x4a31)), // Sfx_GetCoinFromSlots
            0x68 => Some(Sfx::new(0x3c, 0x4a34)), // Sfx_PayDay
            0x69 => Some(Sfx::new(0x3c, 0x4a3a)), // Sfx_Metronome
            0x6a => Some(Sfx::new(0x3c, 0x4a19)), // Sfx_Call
            0x6b => Some(Sfx::new(0x3c, 0x4b2d)), // Sfx_HangUp
            0x6c => Some(Sfx::new(0x3c, 0x4b30)), // Sfx_NoSignal
            0x6d => Some(Sfx::new(0x3c, 0x4b2a)), // Sfx_Sandstorm
            0x6e => None, // Sfx_Elevator
            0x6f => None, // Sfx_Protect
            0x70 => Some(Sfx::new(0x3c, 0x52f6)), // Sfx_Sketch
            0x71 => Some(Sfx::new(0x3c, 0x5314)), // Sfx_RainDance
            0x72 => Some(Sfx::new(0x3c, 0x5334)), // Sfx_Aeroblast
            0x73 => Some(Sfx::new(0x3c, 0x5352)), // Sfx_Spark
            0x74 => Some(Sfx::new(0x3c, 0x5360)), // Sfx_Curse
            0x75 => Some(Sfx::new(0x3c, 0x537d)), // Sfx_Rage
            0x76 => Some(Sfx::new(0x3c, 0x539c)), // Sfx_Thief
            0x77 => None, // Sfx_Thief2
            0x78 => Some(Sfx::new(0x3c, 0x53ca)), // Sfx_SpiderWeb
            0x79 => None, // Sfx_MindReader
            0x7a => Some(Sfx::new(0x3c, 0x541d)), // Sfx_Nightmare
            0x7b => Some(Sfx::new(0x3c, 0x5453)), // Sfx_Snore
            0x7c => Some(Sfx::new(0x3c, 0x5469)), // Sfx_SweetKiss
            0x7d => Some(Sfx::new(0x3c, 0x547f)), // Sfx_SweetKiss2
            0x7e => Some(Sfx::new(0x3c, 0x54a5)), // Sfx_BellyDrum
            0x7f => Some(Sfx::new(0x3c, 0x54ba)), // Sfx_Toxic
            0x80 => Some(Sfx::new(0x3c, 0x54d0)), // Sfx_SludgeBomb
            0x81 => Some(Sfx::new(0x3c, 0x54f5)), // Sfx_Foresight
            0x82 => None, // Sfx_Spite
            0x83 => Some(Sfx::new(0x3c, 0x553a)), // Sfx_Outrage
            0x84 => None, // Sfx_PerishSong
            0x85 => Some(Sfx::new(0x3c, 0x5570)), // Sfx_GigaDrain
            0x86 => Some(Sfx::new(0x3c, 0x55b4)), // Sfx_Attract
            0x87 => Some(Sfx::new(0x3c, 0x55cc)), // Sfx_Kinesis2
            0x88 => Some(Sfx::new(0x3c, 0x55de)), // Sfx_ZapCannon
            0x89 => Some(Sfx::new(0x3c, 0x55ef)), // Sfx_MeanLook
            0x8a => Some(Sfx::new(0x3c, 0x5621)), // Sfx_HealBell
            0x8b => Some(Sfx::new(0x3c, 0x5637)), // Sfx_Return
            0x8c => Some(Sfx::new(0x3c, 0x5653)), // Sfx_ExpBar
            0x8d => Some(Sfx::new(0x3c, 0x567f)), // Sfx_MilkDrink
            0x8e => Some(Sfx::new(0x3c, 0x569f)), // Sfx_Present
            0x8f => Some(Sfx::new(0x3c, 0x56b9)), // Sfx_MorningSun
            0x90 => None, // Sfx_LevelUp
            0x91 => None, // Sfx_KeyItem
            0x92 => None, // Sfx_Fanfare2
            0x93 => None, // Sfx_RegisterPhoneNumber
            0x94 => None, // Sfx_3rdPlace
            0x95 => None, // Sfx_GetEgg
            0x96 => None, // Sfx_GetEgg
            0x97 => None, // Sfx_MoveDeleted
            0x98 => None, // Sfx_2ndPlace
            0x99 => None, // Sfx_1stPlace
            0x9a => None, // Sfx_ChooseACard
            0x9b => None, // Sfx_GetTm
            0x9c => None, // Sfx_GetBadge
            0x9d => None, // Sfx_QuitSlots
            0x9e => None, // Sfx_EggCrack
            0x9f => None, // Sfx_DexFanfareLessThan20
            0xa0 => None, // Sfx_DexFanfare140169
            0xa1 => None, // Sfx_DexFanfare170199
            0xa2 => None, // Sfx_DexFanfare200229
            0xa3 => None, // Sfx_DexFanfare230Plus
            0xa4 => None, // Sfx_Evolved
            0xa5 => None, // Sfx_MasterBall
            0xa6 => None, // Sfx_EggHatch
            0xa7 => Some(Sfx::new(0x3c, 0x57d9)), // Sfx_GsIntroCharizardFireball
            0xa8 => Some(Sfx::new(0x3c, 0x57ff)), // Sfx_GsIntroPokemonAppears
            0xa9 => Some(Sfx::new(0x3c, 0x5818)), // Sfx_Flash
            0xaa => Some(Sfx::new(0x3c, 0x5846)), // Sfx_GameFreakLogoGs
            0xab => Some(Sfx::new(0x3c, 0x5b33)), // Sfx_NotVeryEffective
            0xac => Some(Sfx::new(0x3c, 0x5b40)), // Sfx_Damage
            0xad => Some(Sfx::new(0x3c, 0x5b50)), // Sfx_SuperEffective
            0xae => Some(Sfx::new(0x3c, 0x5b63)), // Sfx_BallBounce
            0xaf => Some(Sfx::new(0x3c, 0x56df)), // Sfx_Moonlight
            0xb0 => Some(Sfx::new(0x3c, 0x56fd)), // Sfx_Encore
            0xb1 => Some(Sfx::new(0x3c, 0x5721)), // Sfx_BeatUp
            0xb2 => Some(Sfx::new(0x3c, 0x574c)), // Sfx_BatonPass
            0xb3 => Some(Sfx::new(0x3c, 0x4944)), // Sfx_BallWobble
            0xb4 => Some(Sfx::new(0x3c, 0x5734)), // Sfx_SweetScent
            0xb5 => Some(Sfx::new(0x3c, 0x5bb3)), // Sfx_SweetScent2
            0xb6 => Some(Sfx::new(0x3c, 0x5bec)), // Sfx_HitEndOfExpBar
            0xb7 => Some(Sfx::new(0x3c, 0x5c10)), // Sfx_GiveTrademon
            0xb8 => Some(Sfx::new(0x3c, 0x5c3e)), // Sfx_GetTrademon
            0xb9 => None, // Sfx_TrainArrived
            0xba => Some(Sfx::new(0x3c, 0x675b)), // Sfx_StopSlot
            0xbb => Some(Sfx::new(0x3c, 0x5cb4)), // Sfx_2Boops

            // New to Crystal
            0xbc => Some(Sfx::new(0x3c, 0x6769)), // Sfx_GlassTing
            0xbd => Some(Sfx::new(0x3c, 0x6773)), // Sfx_GlassTing2
            0xbe => None, // Sfx_IntroUnown1
            0xbf => None, // Sfx_IntroUnown2
            0xc0 => None, // Sfx_IntroUnown3
            0xc1 => Some(Sfx::new(0x5e, 0x586e)), // Sfx_DittoPopUp
            0xc2 => Some(Sfx::new(0x5e, 0x5888)), // Sfx_DittoTransform
            0xc3 => Some(Sfx::new(0x5e, 0x58a0)), // Sfx_IntroSuicune1
            0xc4 => Some(Sfx::new(0x5e, 0x58aa)), // Sfx_IntroPichu
            0xc5 => Some(Sfx::new(0x5e, 0x58c0)), // Sfx_IntroSuicune2
            0xc6 => Some(Sfx::new(0x5e, 0x58f4)), // Sfx_IntroSuicune3
            0xc7 => Some(Sfx::new(0x5e, 0x5907)), // Sfx_DittoBounce
            0xc8 => Some(Sfx::new(0x5e, 0x591d)), // Sfx_IntroSuicune4
            0xc9 => None, // Sfx_GameFreakPresents
            0xca => None, // Sfx_Tingle
            0xcb => Some(Sfx::new(0x3c, 0x5cd0)), // Sfx_IntroWhoosh
            0xcc => Some(Sfx::new(0x5e, 0x597c)), // Sfx_TwoPcBeeps
            0xcd => None, // Sfx_4NoteDitty
            0xce => None, // Sfx_Twinkle

            _ => None
        }
    }

    pub fn tweak(&mut self, pitch: i16, length: u16) {
        self.pitch = pitch;
        self.length = length;
    }

    pub fn tweaked(&self, pitch: i16, length: u16) -> Self {
        Self {
            bank: self.bank,
            addr: self.addr,
            pitch,
            length,
        }
    }
}

pub struct SynthesizerSource<'a>(SoundIterator<'a>);

impl<'a> SynthesizerSource<'a> {
    fn new(source: SoundIterator<'a>) -> SynthesizerSource<'a> {
        SynthesizerSource(source)
    }
}

impl Iterator for SynthesizerSource<'_> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl Source for SynthesizerSource<'_> {
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        self.0.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.0.sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

impl SfxTrait<SynthesizerSource<'static>> for Sfx {
    fn open(self) -> SynthesizerSource<'static> {
        SynthesizerSource::new(
            pokemon_synthesizer::gen2::synthesis(
                ROM,
                self.bank,
                self.addr,
                self.pitch,
                self.length,
            )
            .iter(),
        )
    }
}
