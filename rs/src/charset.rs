use core::intrinsics::{volatile_store};

const CHARSET_BASE: *mut u8 = 0xc000 as _;

#[no_mangle]
pub extern "C" fn make_charset() {
    for c in 0..16 {
        let top =
              (if c & 0x1 != 0 { 0x0f } else { 0x00 }) |
              (if c & 0x2 != 0 { 0xf0 } else { 0x00 });
        let bot =
              (if c & 0x4 != 0 { 0x0f } else { 0x00 }) |
              (if c & 0x8 != 0 { 0xf0 } else { 0x00 });
        for y in 0..4 {
            unsafe{ volatile_store(CHARSET_BASE.offset(c * 8 + y), top) }
        }
        for y in 4..8 {
            unsafe{ volatile_store(CHARSET_BASE.offset(c * 8 + y), bot) }
        }
    }
}
