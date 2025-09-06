use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{
            item_constants::Item,
            mart_constants::{Mart, MartType},
            ram_constants::{DailyFlags, StatusFlags},
        },
        data::items::{bargain_shop, marts::MARTS, rooftop_sale},
    },
};

pub fn open_mart_dialog(cpu: &mut Cpu) {
    let mart_type = MartType::from(cpu.c);
    let mart = Mart::from(cpu.e);

    log::info!("open_mart_dialog({mart_type:?}, {mart:?})");

    cpu.borrow_wram_mut().set_mart_type(mart_type);

    let ptr = get_mart(mart);
    load_mart_pointer(cpu, ptr);

    match mart_type {
        MartType::Standard => mart_dialog(cpu),
        MartType::Bitter => herb_shop(cpu),
        MartType::Bargain => bargain_shop(cpu),
        MartType::Pharmacy => pharmacist(cpu),
        MartType::Rooftop => rooftop_sale(cpu),
        MartType::Unknown(n) => unreachable!("Invalid mart type: {n}"),
    }

    cpu.pc = cpu.stack_pop(); // ret
}

fn mart_dialog(cpu: &mut Cpu) {
    cpu.borrow_wram_mut().set_mart_type(MartType::Standard);
    standard_mart(cpu);
}

fn herb_shop(cpu: &mut Cpu) {
    cpu.call(0x5bbb); // FarReadMart
    cpu.call(0x1d6e); // LoadStandardMenuHeader

    cpu.set_hl(0x5e4a); // HerbShopLadyIntroText
    cpu.call(0x5fcd); // MartTextbox

    cpu.call(0x5c62); // BuyMenu

    cpu.set_hl(0x5e68); // HerbalLadyComeAgainText
    cpu.call(0x5fcd); // MartTextbox
}

fn bargain_shop(cpu: &mut Cpu) {
    load_mart_pointer(cpu, bargain_shop::BARGAIN_SHOP_DATA);
    read_mart(cpu, bargain_shop::BARGAIN_SHOP_DATA);
    cpu.call(0x1d6e); // LoadStandardMenuHeader

    cpu.set_hl(0x5e6d); // BargainShopIntroText
    cpu.call(0x5fcd); // MartTextbox

    cpu.call(0x5c62); // BuyMenu

    if cpu.borrow_wram().bargain_shop_flags() != 0 {
        let mut flags = cpu.borrow_wram().daily_flags();
        flags.insert(DailyFlags::GOLDENROD_UNDERGROUND_BARGAIN);
        cpu.borrow_wram_mut().set_daily_flags(flags);
    }

    cpu.set_hl(0x5e8b); // BargainShopComeAgainText
    cpu.call(0x5fcd); // MartTextbox
}

fn pharmacist(cpu: &mut Cpu) {
    cpu.call(0x5bbb); // FarReadMart
    cpu.call(0x1d6e); // LoadStandardMenuHeader

    cpu.set_hl(0x5e90); // PharmacyIntroText
    cpu.call(0x5fcd); // MartTextbox

    cpu.call(0x5c62); // BuyMenu

    cpu.set_hl(0x5eae); // PharmacyComeAgainText
    cpu.call(0x5fcd); // MartTextbox
}

fn rooftop_sale(cpu: &mut Cpu) {
    let ptr = if !cpu
        .borrow_wram()
        .status_flags()
        .contains(StatusFlags::HALL_OF_FAME)
    {
        rooftop_sale::ROOFTOP_SALE_MART_1
    } else {
        rooftop_sale::ROOFTOP_SALE_MART_2
    };

    load_mart_pointer(cpu, ptr);
    read_mart(cpu, ptr);
    cpu.call(0x1d6e); // LoadStandardMenuHeader

    cpu.set_hl(0x5f83); // MartWelcomeText
    cpu.call(0x5fcd); // MartTextbox

    cpu.call(0x5c62); // BuyMenu

    cpu.set_hl(0x5fb4); // MartComeAgainText
    cpu.call(0x5fcd); // MartTextbox
}

fn load_mart_pointer(cpu: &mut Cpu, ptr: (u8, u16)) {
    cpu.borrow_wram_mut().set_mart_pointer(ptr);

    cpu.a = 0;
    cpu.set_bc(16);
    cpu.set_hl(0xd0f0); // wCurMartCount
    cpu.call(0x3041); // ByteFill: fill bc bytes with the value of a, starting at hl

    cpu.borrow_wram_mut().set_bargain_shop_flags(0);
    cpu.a = 0;
}

fn get_mart(mart: Mart) -> (u8, u16) {
    match mart {
        Mart::Unknown(_) => {
            (0x05, 0x6214) // BANK(DefaultMart), DefaultMart
        }

        _ => {
            (0x05, MARTS[u8::from(mart) as usize]) // BANK(Marts)
        }
    }
}

fn standard_mart(cpu: &mut Cpu) {
    enum StandardMartJumptableIndex {
        TopMenu,
        Buy,
        Sell,
        AnythingElse,
        Quit,
    }

    cpu.call(0x1d6e); // LoadStandardMenuHeader
    cpu.set_hl(0x5f83); // MartWelcomeText
    cpu.call(0x1057); // PrintText

    let mut index = StandardMartJumptableIndex::TopMenu;

    loop {
        index = match index {
            StandardMartJumptableIndex::TopMenu => {
                cpu.set_hl(0x5f88); // MenuHeader_BuySell
                cpu.call(0x1d3c); // CopyMenuHeader

                cpu.call(0x1d81); // VerticalMenu

                if cpu.flag(CpuFlag::C) {
                    StandardMartJumptableIndex::Quit
                } else {
                    match cpu.borrow_wram().menu_cursor_y() {
                        1 => StandardMartJumptableIndex::Buy,
                        2 => StandardMartJumptableIndex::Sell,
                        _ => break,
                    }
                }
            }

            StandardMartJumptableIndex::Buy => {
                cpu.call(0x1c07); // ExitMenu
                cpu.call(0x5bbb); // FarReadMart
                cpu.call(0x5c62); // BuyMenu
                StandardMartJumptableIndex::AnythingElse
            }

            StandardMartJumptableIndex::Sell => {
                cpu.call(0x1c07); // ExitMenu
                cpu.call(0x5eb3); // SellMenu
                StandardMartJumptableIndex::AnythingElse
            }

            StandardMartJumptableIndex::AnythingElse => {
                cpu.call(0x1d6e); // LoadStandardMenuHeader
                cpu.set_hl(0x5fb9); // MartAskMoreText
                cpu.call(0x1057); // PrintText
                StandardMartJumptableIndex::TopMenu
            }

            StandardMartJumptableIndex::Quit => {
                cpu.call(0x1c07); // ExitMenu
                cpu.set_hl(0x5fb4); // MartComeAgainText
                cpu.call(0x5fcd); // MartTextbox
                break;
            }
        };
    }
}

fn read_mart(cpu: &mut Cpu, ptr: (u8, u16)) {
    let count = cpu.read_byte(ptr.1);
    let items = ptr.1 + 1;

    cpu.write_byte(0xd0f0, count); // wCurMartCount

    for i in 0..(count as u16) {
        let item = Item::from(cpu.read_byte(items + (i * 3)));
        let price = cpu.read_byte(items + (i * 3) + 1) as u16
            | ((cpu.read_byte(items + (i * 3) + 2) as u16) << 8);

        cpu.write_byte(0xd0f1 + i, item.into()); // wCurMartItems + i

        cpu.set_de(price);
        cpu.set_hl(0xd002 + (i * 3)); // wMartItem{i}BCD
        cpu.call(0x5bf0); // GetMartPrice
    }
}
