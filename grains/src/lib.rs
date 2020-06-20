//pub fn square(s: u32) -> u64 {
//    if s < 1 || s > 64 {
//        panic!("Square must be between 1 and 64");
//    }
//    let mut current_grain: u64 = 1;
//
//    for _i in 1..s {
//        current_grain <<= 1;
//    }
//
//    current_grain
//}
//
//pub fn total() -> u64 {
//    let mut total_grain: u64 = 1;
//    let mut current_grain: u64 = 1;
//
//    for _i in 2..65 {
//        current_grain <<= 1;
//        total_grain += current_grain;
//    }
//
//    total_grain
//}

pub fn square(s: u32) -> u64 {
    match s {
        1..=64 => 1u64.wrapping_shl(s - 1),
        _ => panic!("Square must be between 1 and 64"),
    }
}
pub fn total() -> u64 {
    (1..65).map(|x| square(x)).sum()
}
