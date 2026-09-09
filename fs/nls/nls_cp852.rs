
unsafe fn uni2char(uni: u16, out: *mut u8, boundlen: i32) -> i32 {
    let cl = (uni & 0x00ff) as usize;
    let ch = ((uni & 0xff00) >> 8) as usize;
    if boundlen <= 0 { return -ENAMETOOLONG; }
    let uni2charset = page_uni2charset[ch];
    if !uni2charset.is_null() && *uni2charset.add(cl) != 0 { *out = *uni2charset.add(cl); } else { return -EINVAL; }
    1
}
unsafe fn char2uni(rawstring: *const u8, _boundlen: i32, uni: *mut u16) -> i32 {
    *uni = charset2uni[*rawstring as usize];
    if *uni == 0x0000 { return -EINVAL; }
    1
}
extern "C" {
    fn register_nls(table: *mut nls_table) -> i32;
    fn unregister_nls(table: *mut nls_table);
}
#[repr(C)]
struct nls_table {
    charset: *const u8,
    uni2char: unsafe fn(u16, *mut u8, i32) -> i32,
    char2uni: unsafe fn(*const u8, i32, *mut u16) -> i32,
    charset2lower: *const u8,
    charset2upper: *const u8,
}
static mut table: nls_table = nls_table {
    charset: b"cp852\0".as_ptr(), uni2char, char2uni,
    charset2lower: charset2lower.as_ptr(), charset2upper: charset2upper.as_ptr(),
};
unsafe fn init_nls_cp852() -> i32 { register_nls(&mut table) }
unsafe fn exit_nls_cp852() { unregister_nls(&mut table); }
const ENAMETOOLONG: i32 = 36;
const EINVAL: i32 = 22;
// MODULE_DESCRIPTION("NLS Codepage 852 (Central/Eastern Europe)");
// MODULE_LICENSE("Dual BSD/GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
