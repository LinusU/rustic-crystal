use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{
            item_constants::Item,
            mart_constants::{Mart, MartType},
            ram_constants::{DailyFlags, StatusFlags},
        },
        data::items::{bargain_shop, rooftop_sale},
    },
};

pub fn open_mart_dialog(cpu: &mut Cpu) {
    let mart_type = MartType::from(cpu.c);
    let mart = Mart::from(cpu.e);

    log::info!("open_mart_dialog({mart_type:?}, {mart:?})");

    cpu.borrow_wram_mut().set_mart_type(mart_type);

    match mart_type {
        MartType::Standard => mart_dialog(cpu, mart),
        MartType::Bitter => herb_shop(cpu, mart),
        MartType::Bargain => bargain_shop(cpu),
        MartType::Pharmacy => pharmacist(cpu, mart),
        MartType::Rooftop => rooftop_sale(cpu),
        MartType::Unknown(n) => unreachable!("Invalid mart type: {n}"),
    }

    cpu.pc = cpu.stack_pop(); // ret
}

fn mart_dialog(cpu: &mut Cpu, mart: Mart) {
    cpu.borrow_wram_mut().set_mart_type(MartType::Standard);
    standard_mart(cpu, mart);
}

fn herb_shop(cpu: &mut Cpu, mart: Mart) {
    far_read_mart(cpu, mart.items());
    cpu.call(0x1d6e); // LoadStandardMenuHeader

    cpu.set_hl(0x5e4a); // HerbShopLadyIntroText
    cpu.call(0x5fcd); // MartTextbox

    cpu.call(0x5c62); // BuyMenu

    cpu.set_hl(0x5e68); // HerbalLadyComeAgainText
    cpu.call(0x5fcd); // MartTextbox
}

fn bargain_shop(cpu: &mut Cpu) {
    load_mart_pointer(cpu, (0x05, 0x5c51));
    read_mart(cpu, &bargain_shop::BARGAIN_SHOP_DATA);
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

fn pharmacist(cpu: &mut Cpu, mart: Mart) {
    far_read_mart(cpu, mart.items());
    cpu.call(0x1d6e); // LoadStandardMenuHeader

    cpu.set_hl(0x5e90); // PharmacyIntroText
    cpu.call(0x5fcd); // MartTextbox

    cpu.call(0x5c62); // BuyMenu

    cpu.set_hl(0x5eae); // PharmacyComeAgainText
    cpu.call(0x5fcd); // MartTextbox
}

fn rooftop_sale(cpu: &mut Cpu) {
    let (ptr, data) = if !cpu
        .borrow_wram()
        .status_flags()
        .contains(StatusFlags::HALL_OF_FAME)
    {
        ((0x05, 0x5aee), rooftop_sale::ROOFTOP_SALE_MART_1)
    } else {
        ((0x05, 0x5aff), rooftop_sale::ROOFTOP_SALE_MART_2)
    };

    load_mart_pointer(cpu, ptr);
    read_mart(cpu, &data);
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

fn standard_mart(cpu: &mut Cpu, mart: Mart) {
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
                far_read_mart(cpu, mart.items());
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

fn far_read_mart(cpu: &mut Cpu, data: &[Item]) {
    cpu.write_byte(0xd0f0, data.len() as u8); // wCurMartCount

    for (i, &item) in data.iter().enumerate() {
        cpu.write_byte(0xd0f1 + i as u16, item.into()); // wCurMartItems + i

        cpu.a = item.into();
        cpu.set_hl(0xd002 + (i as u16 * 3)); // wMartItem{i}BCD
        cpu.call(0x5be5); // GetMartItemPrice: Return the price of item `a` in BCD at `hl` and in tiles at `wStringBuffer1`
    }

    cpu.write_byte(0xd0f1 + data.len() as u16, 0xff); // terminator
}

fn read_mart(cpu: &mut Cpu, data: &[(Item, u16)]) {
    cpu.write_byte(0xd0f0, data.len() as u8); // wCurMartCount

    for (i, &(item, price)) in data.iter().enumerate() {
        cpu.write_byte(0xd0f1 + i as u16, item.into()); // wCurMartItems + i

        cpu.set_de(price);
        cpu.set_hl(0xd002 + (i as u16 * 3)); // wMartItem{i}BCD
        cpu.call(0x5bf0); // GetMartPrice
    }

    cpu.write_byte(0xd0f1 + data.len() as u16, 0xff); // terminator
}
