// SPDX-License-Identifier: GPL-2.0
// pr_fmt(fmt) KBUILD_MODNAME ": " fmt

use core::ffi::c_char;

// Declarations supplied by the kernel headers and the other translation units.
#[repr(C)]
pub struct console {
    pub flags: u32,
}

#[repr(C)]
pub struct console_cmdline {
    pub brl_options: *mut c_char,
    pub index: i32,
    pub options: *mut c_char,
}

unsafe extern "C" {
    fn str_has_prefix(str_: *const c_char, prefix: *const c_char) -> usize;
    fn strchr(s: *const c_char, c: i32) -> *mut c_char;
    fn braille_register_console(
        console: *mut console,
        index: i32,
        options: *mut c_char,
        brl_options: *mut c_char,
    ) -> i32;
    fn braille_unregister_console(console: *mut console) -> i32;
}

// CON_BRL is supplied by the kernel console headers.

pub unsafe fn _braille_console_setup(
    str_: *mut *mut c_char,
    brl_options: *mut *mut c_char,
) -> i32 {
    let mut len: usize;

    len = str_has_prefix(*str_, b"brl,\0".as_ptr() as *const c_char);
    if len != 0 {
        *brl_options = b"\0".as_ptr() as *mut c_char;
        *str_ = (*str_).add(len);
        return 0;
    }

    len = str_has_prefix(*str_, b"brl=\0".as_ptr() as *const c_char);
    if len != 0 {
        *brl_options = (*str_).add(len);
        *str_ = strchr(*brl_options, b',' as i32);
        if (*str_).is_null() {
            // pr_err("need port name after brl=\n");
            return -22;
        }
        let comma = *str_;
        *str_ = (*str_).add(1);
        *comma = 0;
    }

    0
}

pub unsafe fn _braille_register_console(
    console: *mut console,
    c: *mut console_cmdline,
) -> i32 {
    let mut rtn: i32 = 0;

    if !(*c).brl_options.is_null() {
        (*console).flags |= CON_BRL;
        rtn = braille_register_console(
            console,
            (*c).index,
            (*c).options,
            (*c).brl_options,
        );
    }

    rtn
}

pub unsafe fn _braille_unregister_console(console: *mut console) -> i32 {
    if (*console).flags & CON_BRL != 0 {
        return braille_unregister_console(console);
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
