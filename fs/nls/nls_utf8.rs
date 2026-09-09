/*
 * Module for handling utf8 just like any other charset.
 * By Urban Widmark 2000
 */

use core::ffi::{c_char, c_int, c_uchar};

// Linux kernel declarations supplied by the surrounding repository.
unsafe extern "C" {
    fn utf32_to_utf8(uni: u32, out: *mut c_uchar, boundlen: c_int) -> c_int;
    fn utf8_to_utf32(rawstring: *const c_uchar, boundlen: c_int, uni: *mut u32) -> c_int;
    fn register_nls(table: *mut NlsTable) -> c_int;
    fn unregister_nls(table: *mut NlsTable);
}

const ENAMETOOLONG: c_int = 36;
const EINVAL: c_int = 22;
const MAX_WCHAR_T: u32 = 0x7fff_ffff;

static mut identity: [c_uchar; 256] = [0; 256];

#[repr(C)]
struct NlsTable {
    charset: *const c_char,
    uni2char: Option<unsafe extern "C" fn(c_int, *mut c_uchar, c_int) -> c_int>,
    char2uni: Option<unsafe extern "C" fn(*const c_uchar, c_int, *mut c_int) -> c_int>,
    charset2lower: *mut c_uchar,
    charset2upper: *mut c_uchar,
}

unsafe extern "C" fn uni2char(uni: c_int, out: *mut c_uchar, boundlen: c_int) -> c_int {
    let n: c_int;

    if boundlen <= 0 {
        return -ENAMETOOLONG;
    }

    n = unsafe { utf32_to_utf8(uni as u32, out, boundlen) };
    if n < 0 {
        unsafe { *out = b'?'; }
        return -EINVAL;
    }
    n
}

unsafe extern "C" fn char2uni(
    rawstring: *const c_uchar,
    boundlen: c_int,
    uni: *mut c_int,
) -> c_int {
    let n: c_int;
    let mut u: u32 = 0;

    n = unsafe { utf8_to_utf32(rawstring, boundlen, &mut u) };
    if n < 0 || u > MAX_WCHAR_T {
        unsafe { *uni = 0x003f; } /* ? */
        return -EINVAL;
    }
    unsafe { *uni = u as c_int; }
    n
}

static mut table: NlsTable = NlsTable {
    charset: b"utf8\0".as_ptr() as *const c_char,
    uni2char: Some(uni2char),
    char2uni: Some(char2uni),
    charset2lower: core::ptr::addr_of_mut!(identity) as *mut c_uchar,
    charset2upper: core::ptr::addr_of_mut!(identity) as *mut c_uchar,
};

unsafe extern "C" fn init_nls_utf8() -> c_int {
    let mut i: c_int = 0;
    while i < 256 {
        identity[i as usize] = i as c_uchar;
        i += 1;
    }

    unsafe { register_nls(core::ptr::addr_of_mut!(table)) }
}

unsafe extern "C" fn exit_nls_utf8() {
    unsafe { unregister_nls(core::ptr::addr_of_mut!(table)); }
}

// module_init(init_nls_utf8)
// module_exit(exit_nls_utf8)
// MODULE_DESCRIPTION("NLS UTF-8")
// MODULE_LICENSE("Dual BSD/GPL")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
