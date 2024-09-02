// #[test]
#[allow(arithmetic_overflow)]
fn overflow() {
    let x = 251_u8;
    let y = 10_u8;
    let z = x + y;
    let k = 10 as u32 + x as u32;
}

#[test]
fn overflow_handled() {
    let x = u8::checked_add(251_u8, 1);
    dbg!(x);
    let x = u8::checked_add(251_u8, 10);
    dbg!(x);
}
