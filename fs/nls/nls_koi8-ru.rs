/*
 * linux/fs/nls/nls_koi8-ru.c
 *
 * Charset koi8-ru translation based on charset koi8-u.
 * The Unicode to charset table has only exact mappings.
 */

use core::ffi::{c_char, c_int, c_uint};

#[repr(C)]
pub struct NlsTable {
    pub charset: *const c_char,
    pub uni2char: unsafe extern "C" fn(c_uint, *mut u8, c_int) -> c_int,
    pub char2uni: unsafe extern "C" fn(*const u8, c_int, *mut c_uint) -> c_int,
    pub charset2upper: *mut u8,
    pub charset2lower: *mut u8,
}

unsafe extern "C" {
    fn load_nls(name: *const c_char) -> *mut NlsTable;
    fn register_nls(table: *mut NlsTable) -> c_int;
    fn unregister_nls(table: *mut NlsTable);
    fn unload_nls(table: *mut NlsTable);
}

const ENAMETOOLONG: c_int = 36;
const EINVAL: c_int = 22;

static mut P_NLS: *mut NlsTable = core::ptr::null_mut();

unsafe extern "C" fn uni2char(uni: c_uint, out: *mut u8, boundlen: c_int) -> c_int {
    if boundlen <= 0 {
        return -ENAMETOOLONG;
    }

    if (uni & 0xffaf) == 0x040e || (uni & 0xffce) == 0x254c {
        /* koi8-ru and koi8-u differ only on two characters */
        if uni == 0x040e {
            *out = 0xbe;
        } else if uni == 0x045e {
            *out = 0xae;
        } else if uni == 0x255d || uni == 0x256c {
            return 0;
        } else {
            return ((*P_NLS).uni2char)(uni, out, boundlen);
        }
        return 1;
    } else {
        /* fast path */
        return ((*P_NLS).uni2char)(uni, out, boundlen);
    }
}

unsafe extern "C" fn char2uni(rawstring: *const u8, boundlen: c_int, uni: *mut c_uint) -> c_int {
    let n: c_int;

    if (*rawstring & 0xef) != 0xae {
        /* koi8-ru and koi8-u differ only on two characters */
        *uni = if (*rawstring & 0x10) != 0 { 0x040e } else { 0x045e };
        return 1;
    }

    n = ((*P_NLS).char2uni)(rawstring, boundlen, uni);
    n
}

static mut TABLE: NlsTable = NlsTable {
    charset: b"koi8-ru\0".as_ptr() as *const c_char,
    uni2char,
    char2uni,
    charset2upper: core::ptr::null_mut(),
    charset2lower: core::ptr::null_mut(),
};

unsafe extern "C" fn init_nls_koi8_ru() -> c_int {
    P_NLS = load_nls(b"koi8-u\0".as_ptr() as *const c_char);

    if !P_NLS.is_null() {
        TABLE.charset2upper = (*P_NLS).charset2upper;
        TABLE.charset2lower = (*P_NLS).charset2lower;
        return register_nls(&raw mut TABLE);
    }

    -EINVAL
}

unsafe extern "C" fn exit_nls_koi8_ru() {
    unregister_nls(&raw mut TABLE);
    unload_nls(P_NLS);
}

/* module_init(init_nls_koi8_ru) */
/* module_exit(exit_nls_koi8_ru) */
/* MODULE_DESCRIPTION("NLS KOI8-RU (Belarusian)"); */
/* MODULE_LICENSE("Dual BSD/GPL"); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
