pub const fn percent(value: u8) -> u8 {
    assert!(value <= 100, "Percent value cannot exceed 100");

    ((value as u16) * 0xff / 100) as u8
}
