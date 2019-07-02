#![feature(proc_macro_hygiene)]
use overflow::{wrapping, checked};

fn main() {
    let num = 2u8;
    let result = wrapping!{ ((num.pow(20) << 20) + 255) + 2u8 * 2u8 };
    assert!(result == 3);

    let result = wrapping!{ -std::i8::MIN };
    assert!(result == -128);

    let result = checked!{  (2u8 + (2u8 / 10)) * 5 };
    assert!(result == Some(10));

    let result = checked!{  245u8 + (2u8 * 10) };
    assert!(result == None);

    let result = checked!{ -std::i8::MIN };
    assert!(result == None);
}
