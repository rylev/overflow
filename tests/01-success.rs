#![feature(proc_macro_hygiene)]
use overflow::wrapping;

fn main() {
    let num = 2u8;
    let result = wrapping!{ ((num.pow(20) << 20) + 255) + 2u8 * 2u8 };
    assert!(result == 3);
    let result = wrapping!{ -std::i8::MIN };
    assert!(result == -128);
}
