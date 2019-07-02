#![feature(proc_macro_hygiene)]
use overflow::{wrapping, checked, overflowing};

fn main() {
    let num = 2u8;
    let result = wrapping!{ ((num.pow(20) << 20) + 255) + 2u8 * 2u8 };
    assert!(result == 3);

    let result = wrapping!{ -std::i8::MIN };
    assert!(result == -128);

    let result = wrapping!{ 12u8 + 6u8 / 3};
    assert!(result == 14);

    let result = checked!{ (num + (2u8 / 10)) * 5 };
    assert!(result == Some(10));

    let result = checked!{ ((num.pow(20) << 20) + 255) + 2u8 * 2u8 };
    assert!(result == None);

    let result = checked!{ -std::i8::MIN };
    assert!(result == None);

    let result = checked!{ 12u8 + 6u8 / 3};
    assert!(result == Some(14));

    let result = overflowing!{ ((num.pow(20) << 20) + 255) + 2u8 * 2u8 };
    assert!(result == (3, true));

    let result = overflowing!{ -std::i8::MIN };
    assert!(result == (-128, true));

    let result = overflowing!{ (num + 10) + 6u8 / 3};
    assert!(result == (14, false));
}
