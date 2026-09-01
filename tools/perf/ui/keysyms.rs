// SPDX-License-Identifier: GPL-2.0

// Translated from keysyms.c; external dependencies come from keysyms.h and Linux helpers.

extern "C" {
    fn isprint(c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    fn scnprintf(
        buf: *mut ::std::os::raw::c_char,
        size: usize,
        fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;

    static K_DOWN: ::std::os::raw::c_int;
    static K_END: ::std::os::raw::c_int;
    static K_ENTER: ::std::os::raw::c_int;
    static K_ESC: ::std::os::raw::c_int;
    static K_F1: ::std::os::raw::c_int;
    static K_HOME: ::std::os::raw::c_int;
    static K_LEFT: ::std::os::raw::c_int;
    static K_PGDN: ::std::os::raw::c_int;
    static K_PGUP: ::std::os::raw::c_int;
    static K_RIGHT: ::std::os::raw::c_int;
    static K_TAB: ::std::os::raw::c_int;
    static K_UNTAB: ::std::os::raw::c_int;
    static K_UP: ::std::os::raw::c_int;
    static K_BKSPC: ::std::os::raw::c_int;
    static K_DEL: ::std::os::raw::c_int;
}

extern "C" {
    fn SL_KEY_F(n: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn key_name(
    key: ::std::os::raw::c_int,
    bf: *mut ::std::os::raw::c_char,
    size: usize,
) -> *const ::std::os::raw::c_char {
    if isprint(key) != 0 {
        scnprintf(bf, size, b"%c\0".as_ptr() as *const ::std::os::raw::c_char, key);
    } else if key < 32 {
        scnprintf(
            bf,
            size,
            b"Ctrl+%c\0".as_ptr() as *const ::std::os::raw::c_char,
            key + '@' as ::std::os::raw::c_int,
        );
    } else {
        let mut name: *const ::std::os::raw::c_char = ::std::ptr::null();

        if key == K_DOWN {
            name = b"Down\0".as_ptr() as *const ::std::os::raw::c_char;
        } else if key == K_END {
            name = b"End\0".as_ptr() as *const ::std::os::raw::c_char;
        } else if key == K_ENTER {
            name = b"Enter\0".as_ptr() as *const ::std::os::raw::c_char;
        } else if key == K_ESC {
            name = b"ESC\0".as_ptr() as *const ::std::os::raw::c_char;
        } else if key == K_F1 {
            name = b"F1\0".as_ptr() as *const ::std::os::raw::c_char;
        } else if key == K_HOME {
            name = b"Home\0".as_ptr() as *const ::std::os::raw::c_char;
        } else if key == K_LEFT {
            name = b"Left\0".as_ptr() as *const ::std::os::raw::c_char;
        } else if key == K_PGDN {
            name = b"PgDown\0".as_ptr() as *const ::std::os::raw::c_char;
        } else if key == K_PGUP {
            name = b"PgUp\0".as_ptr() as *const ::std::os::raw::c_char;
        } else if key == K_RIGHT {
            name = b"Right\0".as_ptr() as *const ::std::os::raw::c_char;
        } else if key == K_TAB {
            name = b"Tab\0".as_ptr() as *const ::std::os::raw::c_char;
        } else if key == K_UNTAB {
            name = b"Untab\0".as_ptr() as *const ::std::os::raw::c_char;
        } else if key == K_UP {
            name = b"Up\0".as_ptr() as *const ::std::os::raw::c_char;
        } else if key == K_BKSPC {
            name = b"Backspace\0".as_ptr() as *const ::std::os::raw::c_char;
        } else if key == K_DEL {
            name = b"Del\0".as_ptr() as *const ::std::os::raw::c_char;
        } else if key >= SL_KEY_F(1) && key <= SL_KEY_F(63) {
            scnprintf(
                bf,
                size,
                b"F%d\0".as_ptr() as *const ::std::os::raw::c_char,
                key - SL_KEY_F(0),
            );
        } else {
            scnprintf(
                bf,
                size,
                b"Unknown (%d)\0".as_ptr() as *const ::std::os::raw::c_char,
                key,
            );
        }

        if !name.is_null() {
            scnprintf(bf, size, b"%s\0".as_ptr() as *const ::std::os::raw::c_char, name);
        }
    }

    bf
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
