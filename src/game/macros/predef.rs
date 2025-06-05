#[rustfmt::skip]
macro_rules! predef_id {
	(LearnMove) => { 0 };
	(DummyPredef1) => { 1 };
	(HealParty) => { 2 }; // this is both a special and a predef
	(SmallFarFlagAction) => { 3 };
	(ComputeHPBarPixels) => { 4 };
	(FillPP) => { 5 };
	(TryAddMonToParty) => { 6 };
	(AddTempmonToParty) => { 7 };
	(SendGetMonIntoFromBox) => { 8 };
	(SendMonIntoBox) => { 9 };
	(GiveEgg) => { 10 };
	(AnimateHPBar) => { 11 };
	(CalcMonStats) => { 12 };
	(CalcMonStatC) => { 13 };
	(CanLearnTMHMMove) => { 14 };
	(GetTMHMMove) => { 15 };
	(LinkTextboxAtHL) => { 16 };
	(PrintMoveDescription) => { 17 };
	(UpdatePlayerHUD) => { 18 };
	(PlaceGraphic) => { 19 };
	(CheckPlayerPartyForFitMon) => { 20 };
	(UpdateEnemyHUD) => { 21 };
	(StartBattle) => { 22 };
	(FillInExpBar) => { 23 };
	(GetBattleMonBackpic) => { 24 };
	(GetEnemyMonFrontpic) => { 25 };
	(LearnLevelMoves) => { 26 };
	(FillMoves) => { 27 };
	(EvolveAfterBattle) => { 28 };
	(TradeAnimationPlayer2) => { 29 };
	(TradeAnimation) => { 30 };
	(CopyMonToTempMon) => { 31 };
	(ListMoves) => { 32 };
	(PlaceNonFaintStatus) => { 33 };
	(Unused_PlaceEnemyHPLevel) => { 34 };
	(ListMovePP) => { 35 };
	(GetGender) => { 36 };
	(StatsScreenInit) => { 37 };
	(DrawPlayerHP) => { 38 };
	(DrawEnemyHP) => { 39 };
	(PrintTempMonStats) => { 40 };
	(GetTypeName) => { 41 };
	(PrintMoveType) => { 42 };
	(PrintType) => { 43 };
	(PrintMonTypes) => { 44 };
	(GetUnownLetter) => { 45 };
	(LoadPoisonBGPals) => { 46 };
	(DummyPredef2F) => { 47 };
	(InitSGBBorder) => { 48 };
	(LoadSGBLayout) => { 49 };
	(Pokedex_GetArea) => { 50 };
	(Unused_CheckShininess) => { 51 };
	(DoBattleTransition) => { 52 };
	(DummyPredef35) => { 53 };
	(DummyPredef36) => { 54 };
	(PlayBattleAnim) => { 55 };
	(DummyPredef38) => { 56 };
	(DummyPredef39) => { 57 };
	(DummyPredef3A) => { 58 };
	(PartyMonItemName) => { 59 };
	(GetMonFrontpic) => { 60 };
	(GetMonBackpic) => { 61 };
	(GetAnimatedFrontpic) => { 62 };
	(GetTrainerPic) => { 63 };
	(DecompressGet2bpp) => { 64 };
	(CheckTypeMatchup) => { 65 };
	(ConvertMon_1to2) => { 66 };
	(NewPokedexEntry) => { 67 };
	(Unused_AnimateMon_Slow_Normal) => { 68 };
	(PlaceStatusString) => { 69 };
	(LoadMonAnimation) => { 70 };
	(AnimateFrontpic) => { 71 };
	(Unused_HOF_AnimateAlignedFrontpic) => { 72 };
	(HOF_AnimateFrontpic) => { 73 };
}

macro_rules! predef_call {
    ($cpu:ident, $id:ident) => {
        // LD A,u8
        $cpu.a = crate::game::macros::predef::predef_id!($id);
        $cpu.pc += 2;
        $cpu.cycle(8);

        // CALL u16
        $cpu.pc += 3;
        let pc = $cpu.pc;
        $cpu.cycle(24);
        $cpu.call(0x2d83); // Predef
        $cpu.pc = pc;
    };
}

pub(crate) use predef_call;
pub(crate) use predef_id;
