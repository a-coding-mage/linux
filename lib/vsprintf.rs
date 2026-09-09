// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of linux/lib/vsprintf.c.  Kernel-provided symbols remain
 * external dependencies, as they do in the original implementation. */

use core::{ffi::c_char, mem, ptr};

pub type U16 = u16;
pub type U32 = u32;
pub type U64 = u64;

#[no_mangle]
pub static mut no_hash_pointers: bool = false;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum HashPointersPolicy { HashPtrAuto = 0, HashPtrAlways, HashPtrNever }

static mut hash_pointers_mode: HashPointersPolicy = HashPointersPolicy::HashPtrAuto;

extern "C" {
    fn _parse_integer_fixup_radix(s: *const c_char, base: *mut u32) -> *const c_char;
    fn _parse_integer(s: *const c_char, base: u32, result: *mut u64, max: usize) -> u32;
}

const KSTRTOX_OVERFLOW: u32 = 0x8000_0000;

unsafe fn simple_strntoull(startp: *const c_char, endp: *mut *mut c_char,
                           mut base: u32, max_chars: usize) -> u64 {
    let cp = _parse_integer_fixup_radix(startp, &mut base);
    let prefix_chars = cp.offset_from(startp) as usize;
    let mut result = 0u64;
    let cp = if prefix_chars < max_chars {
        let rv = _parse_integer(cp, base, &mut result, max_chars - prefix_chars);
        cp.add((rv & !KSTRTOX_OVERFLOW) as usize)
    } else { startp.add(max_chars) };
    if !endp.is_null() { *endp = cp as *mut c_char; }
    result
}

#[no_mangle]
pub unsafe extern "C" fn simple_strtoull(cp: *const c_char, endp: *mut *mut c_char, base: u32) -> u64 {
    simple_strntoull(cp, endp, base, i32::MAX as usize)
}

#[no_mangle]
pub unsafe extern "C" fn simple_strtoul(cp: *const c_char, endp: *mut *mut c_char, base: u32) -> usize {
    simple_strtoull(cp, endp, base) as usize
}

#[no_mangle]
pub unsafe extern "C" fn simple_strtol(cp: *const c_char, endp: *mut *mut c_char, base: u32) -> isize {
    if *cp as u8 == b'-' { -(simple_strtoul(cp.add(1), endp, base) as isize) }
    else { simple_strtoul(cp, endp, base) as isize }
}

unsafe fn simple_strntoll(cp: *const c_char, endp: *mut *mut c_char, base: u32, max_chars: usize) -> i64 {
    if *cp as u8 == b'-' && max_chars > 0 {
        -(simple_strntoull(cp.add(1), endp, base, max_chars - 1) as i64)
    } else { simple_strntoull(cp, endp, base, max_chars) as i64 }
}

#[no_mangle]
pub unsafe extern "C" fn simple_strtoll(cp: *const c_char, endp: *mut *mut c_char, base: u32) -> i64 {
    simple_strntoll(cp, endp, base, i32::MAX as usize)
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PrintfSpec { pub flags: u8, pub base: u8, pub precision: i16, pub field_width: i32 }

pub const SIGN: u8 = 1;
pub const LEFT: u8 = 2;
pub const PLUS: u8 = 4;
pub const SPACE: u8 = 8;
pub const ZEROPAD: u8 = 16;
pub const SMALL: u8 = 32;
pub const SPECIAL: u8 = 64;
pub const FIELD_WIDTH_MAX: i32 = (1 << 23) - 1;
pub const PRECISION_MAX: i32 = (1 << 15) - 1;

#[no_mangle]
pub unsafe extern "C" fn num_to_str(buf: *mut c_char, size: i32, num: u64, width: u32) -> i32 {
    let mut tmp = [0u8; 24];
    let mut n = num;
    let mut len = 0usize;
    loop { tmp[len] = b'0' + (n % 10) as u8; len += 1; n /= 10; if n == 0 { break; } }
    if len as i32 > size || width > size as u32 { return 0; }
    let pad = width.saturating_sub(len as u32) as usize;
    for i in 0..pad { *buf.add(i) = b' ' as c_char; }
    for i in 0..len { *buf.add(pad + i) = tmp[len - i - 1] as c_char; }
    (len + pad) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
