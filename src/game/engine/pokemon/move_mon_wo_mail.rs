use crate::{
    cpu::Cpu,
    game::{
        constants::text_constants::{MON_NAME_LENGTH, NAME_LENGTH},
        macros,
    },
    game_state::box_mon::BoxMonOwned,
};

pub fn insert_pokemon_into_box(cpu: &mut Cpu) {
    cpu.pc = 0x5322;

    cpu.a = 0x01; // BANK(sBoxCount)
    cpu.call(0x2fcb); // OpenSRAM

    cpu.set_hl(0xad10); // sBoxCount
    cpu.call(0x53cb); // InsertSpeciesIntoBoxOrParty

    cpu.a = (cpu.borrow_sram().current_box().len() as u8) - 1;
    cpu.write_byte(0xd265, cpu.a); // wNextBoxOrPartyIndex

    cpu.set_bc(MON_NAME_LENGTH as u16);
    cpu.set_de(0xd002); // wBufferMonNickname
    cpu.set_hl(0xb082); // sBoxMonNicknames
    cpu.call(0x53e0); // InsertDataIntoBoxOrParty

    cpu.a = (cpu.borrow_sram().current_box().len() as u8) - 1;
    cpu.write_byte(0xd265, cpu.a); // wNextBoxOrPartyIndex

    cpu.set_bc(NAME_LENGTH as u16);
    cpu.set_de(0xd00d); // wBufferMonOT
    cpu.set_hl(0xafa6); // sBoxMonOTs
    cpu.call(0x53e0); // InsertDataIntoBoxOrParty

    cpu.a = (cpu.borrow_sram().current_box().len() as u8) - 1;
    cpu.write_byte(0xd265, cpu.a); // wNextBoxOrPartyIndex

    cpu.set_bc(BoxMonOwned::LEN as u16);
    cpu.set_de(0xd018); // wBufferMon
    cpu.set_hl(0xad26); // sBoxMons
    cpu.call(0x53e0); // InsertDataIntoBoxOrParty

    let moveset = cpu.borrow_wram().buffer_mon().moves();
    cpu.borrow_wram_mut().temp_mon_mut().set_moves(&moveset);

    let pp = cpu.borrow_wram().buffer_mon().pp();
    cpu.borrow_wram_mut().temp_mon_mut().set_pp(pp);

    cpu.b = cpu.borrow_wram().cur_party_mon();
    macros::farcall::farcall(cpu, 0x03, 0x5cb6); // RestorePPOfDepositedPokemon

    cpu.jump(0x2fe1) // CloseSRAM
}
