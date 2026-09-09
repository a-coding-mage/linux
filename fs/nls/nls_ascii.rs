/*
 * linux/fs/nls/nls_ascii.c
 *
 * Charset ascii translation tables.
 * Generated automatically from the Unicode and charset
 * tables from the Unicode Organization (www.unicode.org).
 * The Unicode to charset table has only exact mappings.
 */

use core::ffi::c_char;

const ENAMETOOLONG: i32 = 36;
const EINVAL: i32 = 22;

static CHARSET2UNI: [u32; 256] = [
    0x0000,0x0001,0x0002,0x0003,0x0004,0x0005,0x0006,0x0007,
    0x0008,0x0009,0x000a,0x000b,0x000c,0x000d,0x000e,0x000f,
    0x0010,0x0011,0x0012,0x0013,0x0014,0x0015,0x0016,0x0017,
    0x0018,0x0019,0x001a,0x001b,0x001c,0x001d,0x001e,0x001f,
    0x0020,0x0021,0x0022,0x0023,0x0024,0x0025,0x0026,0x0027,
    0x0028,0x0029,0x002a,0x002b,0x002c,0x002d,0x002e,0x002f,
    0x0030,0x0031,0x0032,0x0033,0x0034,0x0035,0x0036,0x0037,
    0x0038,0x0039,0x003a,0x003b,0x003c,0x003d,0x003e,0x003f,
    0x0040,0x0041,0x0042,0x0043,0x0044,0x0045,0x0046,0x0047,
    0x0048,0x0049,0x004a,0x004b,0x004c,0x004d,0x004e,0x004f,
    0x0050,0x0051,0x0052,0x0053,0x0054,0x0055,0x0056,0x0057,
    0x0058,0x0059,0x005a,0x005b,0x005c,0x005d,0x005e,0x005f,
    0x0060,0x0061,0x0062,0x0063,0x0064,0x0065,0x0066,0x0067,
    0x0068,0x0069,0x006a,0x006b,0x006c,0x006d,0x006e,0x006f,
    0x0070,0x0071,0x0072,0x0073,0x0074,0x0075,0x0076,0x0077,
    0x0078,0x0079,0x007a,0x007b,0x007c,0x007d,0x007e,0x007f,
    0; 128
];

const IDENTITY: [u8; 128] = {
    let mut a = [0u8; 128];
    let mut i = 0;
    while i < 128 { a[i] = i as u8; i += 1; }
    a
};

static PAGE00: [u8; 256] = {
    let mut a = [0u8; 256];
    let mut i = 0;
    while i < 128 { a[i] = i as u8; i += 1; }
    a
};

static PAGE_UNI2CHARSET: [*const u8; 256] = {
    let mut a = [core::ptr::null(); 256];
    a[0] = PAGE00.as_ptr();
    a
};

const fn lower_table() -> [u8; 256] {
    let mut a = [0u8; 256];
    let mut i = 0;
    while i < 256 { a[i] = i as u8; i += 1; }
    let mut c = b'A';
    while c <= b'Z' { a[c as usize] = c + 32; c += 1; }
    a
}

const fn upper_table() -> [u8; 256] {
    let mut a = [0u8; 256];
    let mut i = 0;
    while i < 256 { a[i] = i as u8; i += 1; }
    let mut c = b'a';
    while c <= b'z' { a[c as usize] = c - 32; c += 1; }
    a
}

static CHARSET2LOWER: [u8; 256] = lower_table();
static CHARSET2UPPER: [u8; 256] = upper_table();

unsafe fn uni2char(uni: u32, out: *mut u8, boundlen: i32) -> i32 {
    if boundlen <= 0 { return -ENAMETOOLONG; }
    let cl = (uni & 0x00ff) as usize;
    let ch = ((uni & 0xff00) >> 8) as usize;
    let uni2charset = PAGE_UNI2CHARSET[ch];
    if !uni2charset.is_null() && (*uni2charset.add(cl)) != 0 {
        *out = *uni2charset.add(cl);
    } else { return -EINVAL; }
    1
}

unsafe fn char2uni(rawstring: *const u8, _boundlen: i32, uni: *mut u32) -> i32 {
    *uni = CHARSET2UNI[*rawstring as usize];
    if *uni == 0x0000 { return -EINVAL; }
    1
}

#[repr(C)]
struct NlsTable {
    charset: *const c_char,
    uni2char: unsafe fn(u32, *mut u8, i32) -> i32,
    char2uni: unsafe fn(*const u8, i32, *mut u32) -> i32,
    charset2lower: *const u8,
    charset2upper: *const u8,
}

static mut TABLE: NlsTable = NlsTable {
    charset: b"ascii\0".as_ptr() as *const c_char,
    uni2char,
    char2uni,
    charset2lower: CHARSET2LOWER.as_ptr(),
    charset2upper: CHARSET2UPPER.as_ptr(),
};

extern "C" {
    fn register_nls(table: *mut NlsTable) -> i32;
    fn unregister_nls(table: *mut NlsTable);
}

unsafe fn init_nls_ascii() -> i32 { register_nls(&raw mut TABLE) }
unsafe fn exit_nls_ascii() { unregister_nls(&raw mut TABLE); }

// module_init(init_nls_ascii)
// module_exit(exit_nls_ascii)
// MODULE_DESCRIPTION("NLS ASCII (United States)")
// MODULE_LICENSE("Dual BSD/GPL")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
