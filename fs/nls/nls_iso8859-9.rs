/*
 * linux/fs/nls/nls_iso8859-9.c
 *
 * Charset iso8859-9 translation tables.
 * Generated automatically from the Unicode and charset
 * tables from the Unicode Organization (www.unicode.org).
 * The Unicode to charset table has only exact mappings.
 */

use core::ffi::{c_char, c_int};

const ENAMETOOLONG: c_int = 36;
const EINVAL: c_int = 22;

const fn charset2uni_at(i: usize) -> u32 {
    match i {
        0xd0 => 0x011e, 0xdd => 0x0130, 0xde => 0x015e,
        0xf0 => 0x011f, 0xfd => 0x0131, 0xfe => 0x015f,
        _ => i as u32,
    }
}
const fn make_charset2uni() -> [u32; 256] {
    let mut a = [0u32; 256]; let mut i = 0;
    while i < 256 { a[i] = charset2uni_at(i); i += 1; } a
}
static CHARSET2UNI: [u32; 256] = make_charset2uni();

const fn page00_at(i: usize) -> u8 {
    match i { 0xd0 | 0xdd | 0xde | 0xf0 | 0xfd | 0xfe => 0, _ => i as u8 }
}
const fn make_page00() -> [u8; 256] {
    let mut a = [0u8; 256]; let mut i = 0;
    while i < 256 { a[i] = page00_at(i); i += 1; } a
}
static PAGE00: [u8; 256] = make_page00();
const fn page01_at(i: usize) -> u8 {
    match i { 0x1e => 0xd0, 0x1f => 0xf0, 0x30 => 0xdd, 0x31 => 0xfd,
        0x5e => 0xde, 0x5f => 0xfe, _ => 0 }
}
const fn make_page01() -> [u8; 256] {
    let mut a = [0u8; 256]; let mut i = 0;
    while i < 256 { a[i] = page01_at(i); i += 1; } a
}
static PAGE01: [u8; 256] = make_page01();

const fn lower_at(i: usize) -> u8 {
    match i { 0xc0..=0xcf => (i + 0x20) as u8,
        0xd0 => 0xf0, 0xd7 => 0xd7, 0xd8..=0xdc => (i + 0x20) as u8,
        0xdd => 0x69, 0xde => 0xfe, 0xdf => 0xdf, _ => i as u8 }
}
const fn make_lower() -> [u8; 256] {
    let mut a = [0u8; 256]; let mut i = 0;
    while i < 256 { a[i] = lower_at(i); i += 1; } a
}
static CHARSET2LOWER: [u8; 256] = make_lower();

const fn upper_at(i: usize) -> u8 {
    match i { 0x61..=0x7a => (i - 0x20) as u8,
        0xb5 => 0, 0xe0..=0xef => (i - 0x20) as u8,
        0xf0 => 0xd0, 0xf1..=0xf6 => (i - 0x20) as u8,
        0xf7 => 0xf7, 0xf8..=0xfc => (i - 0x20) as u8,
        0xfd => 0x49, 0xfe => 0xde, 0xff => 0, _ => i as u8 }
}
const fn make_upper() -> [u8; 256] {
    let mut a = [0u8; 256]; let mut i = 0;
    while i < 256 { a[i] = upper_at(i); i += 1; } a
}
static CHARSET2UPPER: [u8; 256] = make_upper();

#[repr(C)]
pub struct nls_table {
    pub charset: *const c_char,
    pub uni2char: Option<unsafe extern "C" fn(u32, *mut u8, c_int) -> c_int>,
    pub char2uni: Option<unsafe extern "C" fn(*const u8, c_int, *mut u32) -> c_int>,
    pub charset2lower: *const u8,
    pub charset2upper: *const u8,
}

extern "C" {
    fn register_nls(table: *mut nls_table) -> c_int;
    fn unregister_nls(table: *mut nls_table);
}

#[no_mangle]
pub unsafe extern "C" fn uni2char(uni: u32, out: *mut u8, boundlen: c_int) -> c_int {
    if boundlen <= 0 { return -ENAMETOOLONG; }
    let cl = (uni & 0xff) as usize; let ch = ((uni & 0xff00) >> 8) as usize;
    let value = match ch { 0 => PAGE00[cl], 1 => PAGE01[cl], _ => 0 };
    if value == 0 { -EINVAL } else { *out = value; 1 }
}

#[no_mangle]
pub unsafe extern "C" fn char2uni(rawstring: *const u8, _boundlen: c_int, uni: *mut u32) -> c_int {
    *uni = CHARSET2UNI[*rawstring as usize];
    if *uni == 0 { -EINVAL } else { 1 }
}

static mut TABLE: nls_table = nls_table {
    charset: b"iso8859-9\0".as_ptr() as *const c_char,
    uni2char: Some(uni2char), char2uni: Some(char2uni),
    charset2lower: CHARSET2LOWER.as_ptr(), charset2upper: CHARSET2UPPER.as_ptr(),
};

#[no_mangle]
pub unsafe extern "C" fn init_nls_iso8859_9() -> c_int { register_nls(&raw mut TABLE) }

#[no_mangle]
pub unsafe extern "C" fn exit_nls_iso8859_9() { unregister_nls(&raw mut TABLE); }

/* MODULE_DESCRIPTION("NLS ISO 8859-9 (Latin 5; Turkish)"); */
/* MODULE_LICENSE("Dual BSD/GPL"); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
