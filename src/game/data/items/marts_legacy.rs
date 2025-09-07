use crate::game::constants::{item_constants::Item, mart_constants::Mart};

impl Mart {
    pub fn items(&self) -> &'static [Item] {
        match self {
            Mart::Cherrygrove => &[
                Item::Potion,
                Item::Antidote,
                Item::ParlyzHeal,
                Item::Awakening,
            ],

            Mart::CherrygroveDex => &[
                Item::PokeBall,
                Item::Potion,
                Item::Antidote,
                Item::ParlyzHeal,
                Item::Awakening,
            ],

            Mart::Violet => &[
                Item::PokeBall,
                Item::Potion,
                Item::Repel,
                Item::EscapeRope,
                Item::Antidote,
                Item::ParlyzHeal,
                Item::Awakening,
                Item::FlowerMail,
            ],

            Mart::Azalea => &[
                Item::PokeBall,
                Item::Potion,
                Item::SuperPotion,
                Item::SuperRepel,
                Item::EscapeRope,
                Item::Antidote,
                Item::ParlyzHeal,
                Item::Awakening,
                Item::Charcoal,
            ],

            Mart::Cianwood => &[
                Item::BerryJuice,
                Item::Ether,
                Item::EnergyPowder,
                Item::EnergyRoot,
                Item::HealPowder,
                Item::RevivalHerb,
                Item::WaterStone,
                Item::FireStone,
                Item::LeafStone,
                Item::Thunderstone,
                Item::SunStone,
                Item::MoonStone,
            ],

            Mart::Goldenrod2F1 => &[
                Item::Potion,
                Item::SuperPotion,
                Item::Antidote,
                Item::ParlyzHeal,
                Item::Awakening,
                Item::BurnHeal,
                Item::IceHeal,
            ],

            Mart::Goldenrod2F2 => &[
                Item::PokeBall,
                Item::GreatBall,
                Item::EscapeRope,
                Item::Repel,
                Item::SuperRepel,
                Item::Revive,
                Item::FullHeal,
                Item::PokeDoll,
                Item::FlowerMail,
            ],

            Mart::Goldenrod3F => &[
                Item::XSpeed,
                Item::XSpecial,
                Item::XDefend,
                Item::XAttack,
                Item::DireHit,
                Item::GuardSpec,
                Item::XAccuracy,
            ],

            Mart::Goldenrod4F => &[
                Item::Protein,
                Item::Iron,
                Item::Carbos,
                Item::Calcium,
                Item::HPUp,
            ],

            Mart::Goldenrod5F1 => &[Item::TmThunderpunch, Item::TmFirePunch, Item::TmIcePunch],

            Mart::Goldenrod5F2 => &[
                Item::TmThunderpunch,
                Item::TmFirePunch,
                Item::TmIcePunch,
                Item::TmHeadbutt,
            ],

            Mart::Goldenrod5F3 => &[
                Item::TmThunderpunch,
                Item::TmFirePunch,
                Item::TmIcePunch,
                Item::TmRockSmash,
            ],

            Mart::Goldenrod5F4 => &[
                Item::TmThunderpunch,
                Item::TmFirePunch,
                Item::TmIcePunch,
                Item::TmHeadbutt,
                Item::TmRockSmash,
            ],

            Mart::Olivine => &[
                Item::GreatBall,
                Item::SuperPotion,
                Item::HyperPotion,
                Item::FullHeal,
                Item::Antidote,
                Item::ParlyzHeal,
                Item::Awakening,
                Item::IceHeal,
                Item::SuperRepel,
                Item::SurfMail,
            ],

            Mart::Ecruteak => &[
                Item::PokeBall,
                Item::GreatBall,
                Item::Potion,
                Item::SuperPotion,
                Item::FullHeal,
                Item::Antidote,
                Item::ParlyzHeal,
                Item::Awakening,
                Item::BurnHeal,
                Item::IceHeal,
                Item::Revive,
            ],

            Mart::Mahogany1 => &[
                Item::TmFrustration,
                Item::TmRoar,
                Item::Tinymushroom,
                Item::Slowpoketail,
                Item::PokeBall,
                Item::Potion,
            ],

            Mart::Mahogany2 => &[
                Item::Ragecandybar,
                Item::MetalCoat,
                Item::UpGrade,
                Item::BrickPiece,
                Item::TmDig,
                Item::TmRollout,
                Item::TmSwift,
                Item::TmNightmare,
                Item::TmDefenseCurl,
                Item::TmDetect,
                Item::TmFrustration,
                Item::TmRoar,
            ],

            Mart::Blackthorn => &[
                Item::UltraBall,
                Item::HyperPotion,
                Item::MaxRepel,
                Item::FullHeal,
                Item::TmSludgeBomb,
                Item::TmSteelWing,
                Item::TmThief,
                Item::TmPsychUp,
                Item::TmRainDance,
                Item::TmSunnyDay,
            ],

            Mart::Viridian => &[
                Item::UltraBall,
                Item::HyperPotion,
                Item::FullHeal,
                Item::Revive,
                Item::Antidote,
                Item::ParlyzHeal,
                Item::Awakening,
                Item::BurnHeal,
                Item::FlowerMail,
            ],

            Mart::Pewter => &[
                Item::GreatBall,
                Item::SuperPotion,
                Item::SuperRepel,
                Item::Antidote,
                Item::ParlyzHeal,
                Item::Awakening,
                Item::BurnHeal,
            ],

            Mart::Cerulean => &[
                Item::GreatBall,
                Item::UltraBall,
                Item::SuperPotion,
                Item::SuperRepel,
                Item::FullHeal,
                Item::XDefend,
                Item::XAttack,
                Item::DireHit,
                Item::SurfMail,
            ],

            Mart::Lavender => &[
                Item::GreatBall,
                Item::Potion,
                Item::SuperPotion,
                Item::MaxRepel,
                Item::Antidote,
                Item::ParlyzHeal,
                Item::Awakening,
                Item::BurnHeal,
            ],

            Mart::Vermilion => &[
                Item::UltraBall,
                Item::SuperPotion,
                Item::HyperPotion,
                Item::Revive,
                Item::ParlyzHeal,
                Item::Awakening,
                Item::BurnHeal,
                Item::LiteBlueMail,
            ],

            Mart::Celadon2F1 => &[
                Item::Potion,
                Item::SuperPotion,
                Item::HyperPotion,
                Item::MaxPotion,
                Item::Revive,
                Item::SuperRepel,
                Item::MaxRepel,
            ],

            Mart::Celadon2F2 => &[
                Item::PokeBall,
                Item::GreatBall,
                Item::UltraBall,
                Item::EscapeRope,
                Item::FullHeal,
                Item::Antidote,
                Item::BurnHeal,
                Item::IceHeal,
                Item::Awakening,
                Item::ParlyzHeal,
            ],

            Mart::Celadon3F => &[
                Item::TmSunnyDay,
                Item::TmRainDance,
                Item::TmSandstorm,
                Item::TmProtect,
                Item::TmEndure,
                Item::TmEarthquake,
                Item::TmSolarbeam,
                Item::TmPsychic,
                Item::TmZapCannon,
            ],

            Mart::Celadon4F => &[
                Item::HPUp,
                Item::Protein,
                Item::Iron,
                Item::Carbos,
                Item::Calcium,
            ],

            Mart::Celadon5F1 => &[
                Item::TmMudSlap,
                Item::TmFuryCutter,
                Item::TmAttract,
                Item::TmShadowBall,
                Item::TmDynamicpunch,
                Item::TmIcyWind,
                Item::TmIronTail,
                Item::TmDragonbreath,
                Item::TmToxic,
                Item::TmGigaDrain,
            ],

            Mart::Celadon5F2 => &[
                Item::TmHiddenPower,
                Item::TmSwagger,
                Item::TmSnore,
                Item::TmSleepTalk,
                Item::TmSweetScent,
                Item::TmReturn,
                Item::TmDreamEater,
                Item::TmCurse,
                Item::TmRest,
            ],

            Mart::Fuchsia => &[
                Item::GreatBall,
                Item::UltraBall,
                Item::SuperPotion,
                Item::HyperPotion,
                Item::FullHeal,
                Item::MaxRepel,
                Item::FlowerMail,
            ],

            Mart::Saffron => &[
                Item::GreatBall,
                Item::UltraBall,
                Item::HyperPotion,
                Item::MaxPotion,
                Item::FullHeal,
                Item::XAttack,
                Item::XDefend,
                Item::FlowerMail,
            ],

            Mart::MtMoon => &[
                Item::PokeDoll,
                Item::FreshWater,
                Item::SodaPop,
                Item::Lemonade,
                Item::Repel,
                Item::PortraitMail,
            ],

            Mart::IndigoPlateau => &[
                Item::UltraBall,
                Item::MaxRepel,
                Item::HyperPotion,
                Item::MaxPotion,
                Item::FullRestore,
                Item::Revive,
                Item::FullHeal,
            ],

            Mart::Underground => &[
                Item::EnergyPowder,
                Item::EnergyRoot,
                Item::HealPowder,
                Item::RevivalHerb,
            ],

            Mart::Unknown(_) => &[Item::PokeBall, Item::Potion],
        }
    }
}
