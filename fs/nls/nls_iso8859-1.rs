/*
 * linux/fs/nls/nls_iso8859-1.c
 *
 * Charset iso8859-1 translation tables.
 * Generated automatically from the Unicode and charset tables from the
 * Unicode Organization (www.unicode.org). The Unicode to charset table has
 * only exact mappings.
 */

const fn charset2uni_init() -> [u16; 256] {
    let mut a = [0u16; 256]; let mut i = 0;
    while i < 256 { a[i] = i as u16; i += 1; } a
}
const fn page00_init() -> [u8; 256] {
    let mut a = [0u8; 256]; let mut i = 0;
    while i < 256 { a[i] = i as u8; i += 1; } a
}
const fn lower_init() -> [u8; 256] {
    let mut a = page00_init(); let mut i = b'A' as usize;
    while i <= b'Z' as usize { a[i] = (i as u8) + 0x20; i += 1; }
    a[0xd0] = 0xf0; a[0xd7] = 0xd7; a[0xde] = 0xfe; a[0xdf] = 0xdf;
    a[0xc0] = 0xe0; a[0xc1] = 0xe1; a[0xc2] = 0xe2; a[0xc3] = 0xe3;
    a[0xc4] = 0xe4; a[0xc5] = 0xe5; a[0xc6] = 0xe6; a[0xc7] = 0xe7;
    a[0xc8] = 0xe8; a[0xc9] = 0xe9; a[0xca] = 0xea; a[0xcb] = 0xeb;
    a[0xcc] = 0xec; a[0xcd] = 0xed; a[0xce] = 0xee; a[0xcf] = 0xef;
    a[0xd8] = 0xf8; a[0xd9] = 0xf9; a[0xda] = 0xfa; a[0xdb] = 0xfb;
    a[0xdc] = 0xfc; a[0xdd] = 0xfd; a
}
const fn upper_init() -> [u8; 256] {
    let mut a = page00_init(); let mut i = b'a' as usize;
    while i <= b'z' as usize { a[i] = (i as u8) - 0x20; i += 1; }
    a[0xb5] = 0; a[0xd7] = 0xd7; a[0xf7] = 0xf7; a[0xdf] = 0;
    a[0xe0] = 0xc0; a[0xe1] = 0xc1; a[0xe2] = 0xc2; a[0xe3] = 0xc3;
    a[0xe4] = 0xc4; a[0xe5] = 0xc5; a[0xe6] = 0xc6; a[0xe7] = 0xc7;
    a[0xe8] = 0xc8; a[0xe9] = 0xc9; a[0xea] = 0xca; a[0xeb] = 0xcb;
    a[0xec] = 0xcc; a[0xed] = 0xcd; a[0xee] = 0xce; a[0xef] = 0xcf;
    a[0xf0] = 0xd0; a[0xf1] = 0xd1; a[0xf2] = 0xd2; a[0xf3] = 0xd3;
    a[0xf4] = 0xd4; a[0xf5] = 0xd5; a[0xf6] = 0xd6; a[0xf8] = 0xd8;
    a[0xf9] = 0xd9; a[0xfa] = 0xda; a[0xfb] = 0xdb; a[0xfc] = 0xdc;
    a[0xfd] = 0xdd; a[0xfe] = 0xde; a
}

static charset2uni: [u16; 256] = charset2uni_init();
static page00: [u8; 256] = page00_init();
// The remaining pages are NULL in the C table; represented by null pointers.
static page_uni2charset: [*const u8; 256] = [core::ptr::null(); 256];
static charset2lower: [u8; 256] = lower_init();
static charset2upper: [u8; 256] = upper_init();

unsafe fn uni2char(uni: u16, out: *mut u8, boundlen: i32) -> i32 {
    if boundlen <= 0 { return -ENAMETOOLONG; }
    let cl = (uni & 0x00ff) as usize; let ch = ((uni & 0xff00) >> 8) as usize;
    if ch == 0 { *out = page00[cl]; if *out != 0 { return 1; } }
    -EINVAL
}

unsafe fn char2uni(rawstring: *const u8, _boundlen: i32, uni: *mut u16) -> i32 {
    *uni = charset2uni[*rawstring as usize];
    if *uni == 0 { return -EINVAL; } 1
}

static mut table: nls_table = nls_table {
    charset: "iso8859-1", uni2char: uni2char, char2uni: char2uni,
    charset2lower: charset2lower, charset2upper: charset2upper,
};

unsafe fn init_nls_iso8859_1() -> i32 { register_nls(&mut table) }
unsafe fn exit_nls_iso8859_1() { unregister_nls(&mut table); }

// module_init(init_nls_iso8859_1)
// module_exit(exit_nls_iso8859_1)
// MODULE_DESCRIPTION("NLS ISO 8859-1 (Latin 1; Western European Languages)")
// MODULE_LICENSE("Dual BSD/GPL")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
