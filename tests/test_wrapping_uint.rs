#[allow(unused_imports)]
extern crate wrapping_uint;
use wrapping_uint::wrappit;

#[test]
fn test_mul() {
    #[wrappit]
    fn mul() -> u32 {
        let a: u32 = std::u32::MAX;
        a * 2
    }
    assert_eq!(0xfffffffeu32, mul());
}

#[test]
fn test_add() {
    #[wrappit]
    fn add() -> u32 {
        let a: u32 = std::u32::MAX;
        a + 2
    }
    assert_eq!(1u32, add());
    // TODO: assert_eq!(1u32, a + b as u32); won't work because the macro expansion get's in the way.
}

#[test]
fn test_mul_assign() {
    #[wrappit]
    fn mul() -> u32 {
        let mut a: u32 = std::u32::MAX;
        a *= 2;
        a
    }
    assert_eq!(0xfffffffeu32, mul());
}

#[test]
fn test_add_assign() {
    #[wrappit]
    fn add() -> u32 {
        let mut a: u32 = std::u32::MAX;
        a += 2;
        a
    }
    assert_eq!(1u32, add());
}
