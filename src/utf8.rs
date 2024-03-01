// https://github.com/rust-lang/rust/blob/6db96de66c2c0ea3f4f2f348ed1a83c2c507687d/library/core/src/str/validations.rs

macro_rules! next_in_slice {
    ($ident:ident) => {
        match $ident.split_first() {
            Some((first, rest)) => {
                $ident = rest;
                Some(*first)
            }
            None => None,
        }
    };
}

pub enum Pop {
    Empty,
    Truncated,
    Invalid,
    Ok(char),
}

pub const fn pop(mut bytes: &[u8]) -> Pop {
    let Some(x) = next_in_slice!(bytes) else {
        return Pop::Empty;
    };
    if x < 128 {
        return match char::from_u32(x as u32) {
            Some(c) => Pop::Ok(c),
            None => Pop::Invalid,
        };
    };

    let init = utf8_first_byte(x, 2);
    let Some(y) = next_in_slice!(bytes) else {
        return Pop::Truncated;
    };
    let mut ch = utf8_acc_cont_byte(init, y);
    if x >= 0xE0 {
        let Some(z) = next_in_slice!(bytes) else {
            return Pop::Truncated;
        };
        let y_z = utf8_acc_cont_byte((y & CONT_MASK) as u32, z);
        ch = init << 12 | y_z;
        if x >= 0xF0 {
            let Some(w) = next_in_slice!(bytes) else {
                return Pop::Truncated;
            };
            ch = (init & 7) << 18 | utf8_acc_cont_byte(y_z, w)
        }
    }

    let _ = bytes; // done
    match char::from_u32(ch) {
        Some(c) => Pop::Ok(c),
        None => Pop::Invalid,
    }
}

/// Mask of the value bits of a continuation byte.
const CONT_MASK: u8 = 0b0011_1111;

/// Returns the initial codepoint accumulator for the first byte.
/// The first byte is special, only want bottom 5 bits for width 2, 4 bits
/// for width 3, and 3 bits for width 4.
#[inline]
const fn utf8_first_byte(byte: u8, width: u32) -> u32 {
    (byte & (0x7F >> width)) as u32
}

/// Returns the value of `ch` updated with continuation byte `byte`.
#[inline]
const fn utf8_acc_cont_byte(ch: u32, byte: u8) -> u32 {
    (ch << 6) | (byte & CONT_MASK) as u32
}
