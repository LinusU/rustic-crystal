macro_rules! define_map_enum {
    (
        $( group $gname:ident { $( $variant:ident ),* $(,)? } )+ $(,)?
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Map {
            $( $( $variant, )* )+
            Unknown(u8, u8),
        }

        impl From<(u8, u8)> for Map {
            fn from(id: (u8, u8)) -> Self {
                {
                    let (__x, __y) = id;
                    // expand to a sequence of `if ... { return Map::... }` statements
                    define_map_enum!(@emit_from_statements __x __y [] ; $( group $gname { $( $variant ),* } )+);
                    Map::Unknown(__x, __y)
                }
            }
        }

        impl From<Map> for (u8, u8) {
            fn from(map: Map) -> Self {
                {
                    let __m = map;
                    // expand to a sequence of `if let Map::... = __m { return (...) }`
                    define_map_enum!(@emit_to_statements __m [] ; $( group $gname { $( $variant ),* } )+);
                    match __m {
                        Map::Unknown(x, y) => (x, y),
                        _ => unreachable!("all named variants handled by generated code"),
                    }
                }
            }
        }
    };

    // ===== (u8,u8) -> Map =====
    (@emit_from_statements $x:ident $y:ident [$($gacc:tt)*] ; ) => {};
    (@emit_from_statements $x:ident $y:ident [$($gacc:tt)*] ;
        group $gname:ident { $( $v:ident ),* $(,)? } $($rest:tt)*
    ) => {
        define_map_enum!(@emit_from_items $x $y [$($gacc)*] [] $( $v ),*);
        define_map_enum!(@emit_from_statements $x $y [$($gacc)* _] ; $($rest)*);
    };

    (@emit_from_items $x:ident $y:ident [$($gacc:tt)*] [$($iacc:tt)*]) => {};
    (@emit_from_items $x:ident $y:ident [$($gacc:tt)*] [$($iacc:tt)*] $head:ident $(, $tail:ident )* ) => {
        if ($x as usize) == (define_map_enum!(@n $($gacc)*) + 1)
           && ($y as usize) == (define_map_enum!(@n $($iacc)*) + 1)
        {
            return Map::$head;
        }
        define_map_enum!(@emit_from_items $x $y [$($gacc)*] [$($iacc)* _] $( $tail ),*);
    };

    // ===== Map -> (u8,u8) =====
    (@emit_to_statements $m:ident [$($gacc:tt)*] ; ) => {};
    (@emit_to_statements $m:ident [$($gacc:tt)*] ;
        group $gname:ident { $( $v:ident ),* $(,)? } $($rest:tt)*
    ) => {
        define_map_enum!(@emit_to_items $m [$($gacc)*] [] $( $v ),*);
        define_map_enum!(@emit_to_statements $m [$($gacc)* _] ; $($rest)*);
    };

    (@emit_to_items $m:ident [$($gacc:tt)*] [$($iacc:tt)*] $head:ident $(, $tail:ident )* ) => {
        if let Map::$head = $m {
            return (
                (define_map_enum!(@n $($gacc)*) + 1) as u8,
                (define_map_enum!(@n $($iacc)*) + 1) as u8,
            );
        }
        define_map_enum!(@emit_to_items $m [$($gacc)*] [$($iacc)* _] $( $tail ),*);
    };
    (@emit_to_items $m:ident [$($gacc:tt)*] [$($iacc:tt)*]) => {};

    // ===== tiny counter: `_ _ _` -> N =====
    (@n) => { 0usize };
    (@n $_head:tt $($rest:tt)*) => { 1usize + define_map_enum!(@n $($rest)*) };
}

define_map_enum! {
    group Olivine {
        OlivinePokecenter1F,
        OlivineGym,
        OlivineTimsHouse,
        OlivineHouseBeta,
        OlivinePunishmentSpeechHouse,
        OlivineGoodRodHouse,
        OlivineCafe,
        OlivineMart,
        Route38EcruteakGate,
        Route39Barn,
        Route39Farmhouse,
        Route38,
        Route39,
        OlivineCity,
    }

    group Mahogany {
        MahoganyRedGyaradosSpeechHouse,
        MahoganyGym,
        MahoganyPokecenter1F,
        Route42EcruteakGate,
        Route42,
        Route44,
        MahoganyTown,
    }

    group Dungeons {
        SproutTower1F,
        SproutTower2F,
        SproutTower3F,
        TinTower1F,
        TinTower2F,
        TinTower3F,
        TinTower4F,
        TinTower5F,
        TinTower6F,
        TinTower7F,
        TinTower8F,
        TinTower9F,
        BurnedTower1F,
        BurnedTowerB1F,
        NationalPark,
        NationalParkBugContest,
        RadioTower1F,
        RadioTower2F,
        RadioTower3F,
        RadioTower4F,
        RadioTower5F,
        RuinsOfAlphOutside,
        RuinsOfAlphHoOhChamber,
        RuinsOfAlphKabutoChamber,
        RuinsOfAlphOmanyteChamber,
        RuinsOfAlphAerodactylChamber,
        RuinsOfAlphInnerChamber,
        RuinsOfAlphResearchCenter,
        RuinsOfAlphHoOhItemRoom,
        RuinsOfAlphKabutoItemRoom,
        RuinsOfAlphOmanyteItemRoom,
        RuinsOfAlphAerodactylItemRoom,
        RuinsOfAlphHoOhWordRoom,
        RuinsOfAlphKabutoWordRoom,
        RuinsOfAlphOmanyteWordRoom,
        RuinsOfAlphAerodactylWordRoom,
        UnionCave1F,
        UnionCaveB1F,
        UnionCaveB2F,
        SlowpokeWellB1F,
        SlowpokeWellB2F,
        OlivineLighthouse1F,
        OlivineLighthouse2F,
        OlivineLighthouse3F,
        OlivineLighthouse4F,
        OlivineLighthouse5F,
        OlivineLighthouse6F,
        MahoganyMart1F,
        TeamRocketBaseB1F,
        TeamRocketBaseB2F,
        TeamRocketBaseB3F,
        IlexForest,
        GoldenrodUnderground,
        GoldenrodUndergroundSwitchRoomEntrances,
        GoldenrodDeptStoreB1F,
        GoldenrodUndergroundWarehouse,
        MountMortar1FOutside,
        MountMortar1FInside,
        MountMortar2FInside,
        MountMortarB1F,
        IcePath1F,
        IcePathB1F,
        IcePathB2FMahoganySide,
        IcePathB2FBlackthornSide,
        IcePathB3F,
        WhirlIslandNw,
        WhirlIslandNe,
        WhirlIslandSw,
        WhirlIslandCave,
        WhirlIslandSe,
        WhirlIslandB1F,
        WhirlIslandB2F,
        WhirlIslandLugiaChamber,
        SilverCaveRoom1,
        SilverCaveRoom2,
        SilverCaveRoom3,
        SilverCaveItemRooms,
        DarkCaveVioletEntrance,
        DarkCaveBlackthornEntrance,
        DragonsDen1F,
        DragonsDenB1F,
        DragonShrine,
        TohjoFalls,
        DiglettsCave,
        MountMoon,
        UndergroundPath,
        RockTunnel1F,
        RockTunnelB1F,
        SafariZoneFuchsiaGateBeta,
        SafariZoneBeta,
        VictoryRoad,
    }

    group Ecruteak {
        EcruteakTinTowerEntrance,
        WiseTriosRoom,
        EcruteakPokecenter1F,
        EcruteakLugiaSpeechHouse,
        DanceTheater,
        EcruteakMart,
        EcruteakGym,
        EcruteakItemfinderHouse,
        EcruteakCity,
    }

    group Blackthorn {
        BlackthornGym1F,
        BlackthornGym2F,
        BlackthornDragonSpeechHouse,
        BlackthornEmysHouse,
        BlackthornMart,
        BlackthornPokecenter1F,
        MoveDeletersHouse,
        Route45,
        Route46,
        BlackthornCity,
    }

    group Cinnabar {
        CinnabarPokecenter1F,
        CinnabarPokecenter2FBeta,
        Route19FuchsiaGate,
        SeafoamGym,
        Route19,
        Route20,
        Route21,
        CinnabarIsland,
    }

    group Cerulean {
        CeruleanGymBadgeSpeechHouse,
        CeruleanPoliceStation,
        CeruleanTradeSpeechHouse,
        CeruleanPokecenter1F,
        CeruleanPokecenter2FBeta,
        CeruleanGym,
        CeruleanMart,
        Route10Pokecenter1F,
        Route10Pokecenter2FBeta,
        PowerPlant,
        BillsHouse,
        Route4,
        Route9,
        Route10North,
        Route24,
        Route25,
        CeruleanCity,
    }

    group Azalea {
        AzaleaPokecenter1F,
        CharcoalKiln,
        AzaleaMart,
        KurtsHouse,
        AzaleaGym,
        Route33,
        AzaleaTown,
    }

    group LakeOfRage {
        LakeOfRageHiddenPowerHouse,
        LakeOfRageMagikarpHouse,
        Route43MahoganyGate,
        Route43Gate,
        Route43,
        LakeOfRage,
    }

    group Violet {
        Route32,
        Route35,
        Route36,
        Route37,
        VioletCity,
        VioletMart,
        VioletGym,
        EarlsPokemonAcademy,
        VioletNicknameSpeechHouse,
        VioletPokecenter1F,
        VioletKylesHouse,
        Route32RuinsOfAlphGate,
        Route32Pokecenter1F,
        Route35GoldenrodGate,
        Route35NationalParkGate,
        Route36RuinsOfAlphGate,
        Route36NationalParkGate,
    }

    group Goldenrod {
        Route34,
        GoldenrodCity,
        GoldenrodGym,
        GoldenrodBikeShop,
        GoldenrodHappinessRater,
        BillsFamilysHouse,
        GoldenrodMagnetTrainStation,
        GoldenrodFlowerShop,
        GoldenrodPpSpeechHouse,
        GoldenrodNameRater,
        GoldenrodDeptStore1F,
        GoldenrodDeptStore2F,
        GoldenrodDeptStore3F,
        GoldenrodDeptStore4F,
        GoldenrodDeptStore5F,
        GoldenrodDeptStore6F,
        GoldenrodDeptStoreElevator,
        GoldenrodDeptStoreRoof,
        GoldenrodGameCorner,
        GoldenrodPokecenter1F,
        PokecomCenterAdminOfficeMobile,
        IlexForestAzaleaGate,
        Route34IlexForestGate,
        DayCare,
    }

    group Vermilion {
        Route6,
        Route11,
        VermilionCity,
        VermilionFishingSpeechHouse,
        VermilionPokecenter1F,
        VermilionPokecenter2FBeta,
        PokemonFanClub,
        VermilionMagnetTrainSpeechHouse,
        VermilionMart,
        VermilionDiglettsCaveSpeechHouse,
        VermilionGym,
        Route6SaffronGate,
        Route6UndergroundPathEntrance,
    }

    group Pallet {
        Route1,
        PalletTown,
        RedsHouse1F,
        RedsHouse2F,
        BluesHouse,
        OaksLab,
    }

    group Pewter {
        Route3,
        PewterCity,
        PewterNidoranSpeechHouse,
        PewterGym,
        PewterMart,
        PewterPokecenter1F,
        PewterPokecenter2FBeta,
        PewterSnoozeSpeechHouse,
    }

    group FastShip {
        OlivinePort,
        VermilionPort,
        FastShip1F,
        FastShipCabinsNnwNneNe,
        FastShipCabinsSwSswNw,
        FastShipCabinsSeSseCaptainsCabin,
        FastShipB1F,
        OlivinePortPassage,
        VermilionPortPassage,
        MountMoonSquare,
        MountMoonGiftShop,
        TinTowerRoof,
    }

    group Indigo {
        Route23,
        IndigoPlateauPokecenter1F,
        WillsRoom,
        KogasRoom,
        BrunosRoom,
        KarensRoom,
        LancesRoom,
        HallOfFame,
    }

    group Fuchsia {
        Route13,
        Route14,
        Route15,
        Route18,
        FuchsiaCity,
        FuchsiaMart,
        SafariZoneMainOffice,
        FuchsiaGym,
        BillsOlderSistersHouse,
        FuchsiaPokecenter1F,
        FuchsiaPokecenter2FBeta,
        SafariZoneWardensHome,
        Route15FuchsiaGate,
    }

    group Lavender {
        Route8,
        Route12,
        Route10South,
        LavenderTown,
        LavenderPokecenter1F,
        LavenderPokecenter2FBeta,
        MrFujisHouse,
        LavenderSpeechHouse,
        LavenderNameRater,
        LavenderMart,
        SoulHouse,
        LavRadioTower1F,
        Route8SaffronGate,
        Route12SuperRodHouse,
    }

    group Silver {
        Route28,
        SilverCaveOutside,
        SilverCavePokecenter1F,
        Route28SteelWingHouse,
    }

    group CableClub {
        Pokecenter2F,
        TradeCenter,
        Colosseum,
        TimeCapsule,
        MobileTradeRoom,
        MobileBattleRoom,
    }

    group Celadon {
        Route7,
        Route16,
        Route17,
        CeladonCity,
        CeladonDeptStore1F,
        CeladonDeptStore2F,
        CeladonDeptStore3F,
        CeladonDeptStore4F,
        CeladonDeptStore5F,
        CeladonDeptStore6F,
        CeladonDeptStoreElevator,
        CeladonMansion1F,
        CeladonMansion2F,
        CeladonMansion3F,
        CeladonMansionRoof,
        CeladonMansionRoofHouse,
        CeladonPokecenter1F,
        CeladonPokecenter2FBeta,
        CeladonGameCorner,
        CeladonGameCornerPrizeRoom,
        CeladonGym,
        CeladonCafe,
        Route16FuchsiaSpeechHouse,
        Route16Gate,
        Route7SaffronGate,
        Route17Route18Gate,
    }

    group Cianwood {
        Route40,
        Route41,
        CianwoodCity,
        ManiasHouse,
        CianwoodGym,
        CianwoodPokecenter1F,
        CianwoodPharmacy,
        CianwoodPhotoStudio,
        CianwoodLugiaSpeechHouse,
        PokeSeersHouse,
        BattleTower1F,
        BattleTowerBattleRoom,
        BattleTowerElevator,
        BattleTowerHallway,
        Route40BattleTowerGate,
        BattleTowerOutside,
    }

    group Viridian {
        Route2,
        Route22,
        ViridianCity,
        ViridianGym,
        ViridianNicknameSpeechHouse,
        TrainerHouse1F,
        TrainerHouseB1F,
        ViridianMart,
        ViridianPokecenter1F,
        ViridianPokecenter2FBeta,
        Route2NuggetHouse,
        Route2Gate,
        VictoryRoadGate,
    }

    group NewBark {
        Route26,
        Route27,
        Route29,
        NewBarkTown,
        ElmsLab,
        PlayersHouse1F,
        PlayersHouse2F,
        PlayersNeighborsHouse,
        ElmsHouse,
        Route26HealHouse,
        DayOfWeekSiblingsHouse,
        Route27SandstormHouse,
        Route29Route46Gate,
    }

    group Saffron {
        Route5,
        SaffronCity,
        FightingDojo,
        SaffronGym,
        SaffronMart,
        SaffronPokecenter1F,
        SaffronPokecenter2FBeta,
        MrPsychicsHouse,
        SaffronMagnetTrainStation,
        SilphCo1F,
        CopycatsHouse1F,
        CopycatsHouse2F,
        Route5UndergroundPathEntrance,
        Route5SaffronGate,
        Route5CleanseTagHouse,
    }

    group Cherrygrove {
        Route30,
        Route31,
        CherrygroveCity,
        CherrygroveMart,
        CherrygrovePokecenter1F,
        CherrygroveGymSpeechHouse,
        GuideGentsHouse,
        CherrygroveEvolutionSpeechHouse,
        Route30BerryHouse,
        MrPokemonsHouse,
        Route31VioletGate,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_enum() {
        assert_eq!((1, 1), Map::OlivinePokecenter1F.into());
        assert_eq!(Map::OlivinePokecenter1F, Map::from((1, 1)));

        assert_eq!((2, 1), Map::MahoganyRedGyaradosSpeechHouse.into());
        assert_eq!(Map::MahoganyRedGyaradosSpeechHouse, Map::from((2, 1)));

        assert_eq!((5, 4), Map::BlackthornEmysHouse.into());
        assert_eq!(Map::BlackthornEmysHouse, Map::from((5, 4)));

        assert_eq!((15, 9), Map::VermilionPortPassage.into());
        assert_eq!(Map::VermilionPortPassage, Map::from((15, 9)));

        assert_eq!((18, 13), Map::Route8SaffronGate.into());
        assert_eq!(Map::Route8SaffronGate, Map::from((18, 13)));

        assert_eq!((19, 2), Map::SilverCaveOutside.into());
        assert_eq!(Map::SilverCaveOutside, Map::from((19, 2)));

        assert_eq!((26, 11), Map::Route31VioletGate.into());
        assert_eq!(Map::Route31VioletGate, Map::from((26, 11)));
    }
}
