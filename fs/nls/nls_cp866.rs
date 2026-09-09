/* linux/fs/nls/nls_cp866.c -- Charset cp866 translation tables. */

const fn charset2uni_table() -> [u16; 256] {
    let mut a = [0u16; 256];
    let mut i = 0;
    while i < 128 { a[i] = i as u16; i += 1; }
    i = 0; while i < 32 { a[0x80 + i] = 0x0410 + i as u16; i += 1; }
    i = 0; while i < 32 { a[0xa0 + i] = 0x0430 + i as u16; i += 1; }
    let b = [0x2591,0x2592,0x2593,0x2502,0x2524,0x2561,0x2562,0x2556,0x2555,0x2563,0x2551,0x2557,0x255d,0x255c,0x255b,0x2510,0x2514,0x2534,0x252c,0x251c,0x2500,0x253c,0x255e,0x255f,0x255a,0x2554,0x2569,0x2566,0x2560,0x2550,0x256c,0x2567,0x2568,0x2564,0x2565,0x2559,0x2558,0x2552,0x2553,0x256b,0x256a,0x2518,0x250c,0x2588,0x2584,0x258c,0x2590,0x2580];
    i = 0; while i < 48 { a[0xb0+i] = b[i]; i += 1; }
    i = 0; while i < 16 { a[0xe0+i] = 0x0440 + i as u16; i += 1; }
    let t = [0x0401,0x0451,0x0404,0x0454,0x0407,0x0457,0x040e,0x045e,0x00b0,0x2219,0x00b7,0x221a,0x2116,0x00a4,0x25a0,0x00a0];
    i = 0; while i < 16 { a[0xf0+i] = t[i]; i += 1; } a
}
static CHARSET2UNI: [u16; 256] = charset2uni_table();

const fn case_table(upper: bool) -> [u8; 256] {
    let mut a = [0u8; 256]; let mut i = 0; while i < 256 { a[i] = i as u8; i += 1; }
    i = 0x61; while i <= 0x7a { a[i] = if upper { i as u8 - 0x20 } else { i as u8 }; i += 1; }
    i = 0x41; while i <= 0x5a { a[i] = if upper { i as u8 } else { i as u8 + 0x20 }; i += 1; }
    i = 0; while i < 16 { a[0x80+i] = if upper { 0x80+i as u8 } else { 0xa0+i as u8 }; a[0xa0+i] = if upper { 0x80+i as u8 } else { 0xa0+i as u8 }; i += 1; } a
}
static CHARSET2LOWER: [u8; 256] = case_table(false);
static CHARSET2UPPER: [u8; 256] = case_table(true);

extern "C" { fn register_nls(table: *mut NlsTable) -> i32; fn unregister_nls(table: *mut NlsTable); }
#[repr(C)] struct NlsTable { charset: *const u8, uni2char: unsafe extern "C" fn(u16,*mut u8,i32)->i32, char2uni: unsafe extern "C" fn(*const u8,i32,*mut u16)->i32, charset2lower: *const u8, charset2upper: *const u8 }

unsafe extern "C" fn uni2char(uni: u16, out: *mut u8, boundlen: i32) -> i32 {
    if boundlen <= 0 { return -36; }
    let mut i = 0; while i < 256 { if CHARSET2UNI[i] == uni { *out = i as u8; return 1; } i += 1; } -22
}
unsafe extern "C" fn char2uni(rawstring: *const u8, _boundlen: i32, uni: *mut u16) -> i32 { *uni = CHARSET2UNI[*rawstring as usize]; if *uni == 0 { -22 } else { 1 } }
static CHARSET: &[u8] = b"cp866\0";
static mut TABLE: NlsTable = NlsTable { charset: CHARSET.as_ptr(), uni2char, char2uni, charset2lower: CHARSET2LOWER.as_ptr(), charset2upper: CHARSET2UPPER.as_ptr() };
unsafe extern "C" fn init_nls_cp866() -> i32 { register_nls(&mut TABLE) }
unsafe extern "C" fn exit_nls_cp866() { unregister_nls(&mut TABLE) }
// module_init(init_nls_cp866); module_exit(exit_nls_cp866)
// MODULE_DESCRIPTION("NLS Codepage 866 (Cyrillic/Russian)"); MODULE_LICENSE("Dual BSD/GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
