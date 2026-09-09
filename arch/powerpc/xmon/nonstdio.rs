// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 1996-2005 Paul Mackerras.
 */

use core::ffi::{c_char, c_int};

// Supplied by the platform/debugger dependencies.
unsafe extern "C" {
    static mut udbg_getc: Option<unsafe extern "C" fn() -> c_int>;
    fn udbg_write(ptr: *const c_char, nb: c_int) -> c_int;
    fn vsnprintf(buf: *mut c_char, size: usize, format: *const c_char, args: *mut core::ffi::c_void) -> c_int;
}

static mut PAGINATING: bool = false;
static mut PAGINATE_SKIPPING: bool = false;
static mut PAGINATE_LPP: usize = 0; // Lines Per Page
static mut PAGINATE_POS: usize = 0;

pub unsafe extern "C" fn xmon_start_pagination() {
    PAGINATING = true;
    PAGINATE_SKIPPING = false;
    PAGINATE_POS = 0;
}

pub unsafe extern "C" fn xmon_end_pagination() {
    PAGINATING = false;
}

pub unsafe extern "C" fn xmon_set_pagination_lpp(lpp: usize) {
    PAGINATE_LPP = lpp;
}

unsafe fn xmon_readchar() -> c_int {
    match udbg_getc {
        Some(getc) => getc(),
        None => -1,
    }
}

unsafe fn xmon_write(ptr: *const c_char, nb: c_int) -> c_int {
    let mut rv: c_int = 0;
    let mut p = ptr;
    let msg = b"[Hit a key (a:all, q:truncate, any:next page)]\0";

    if nb <= 0 {
        return rv;
    }
    if PAGINATING && PAGINATE_SKIPPING {
        return nb;
    }

    if PAGINATE_LPP != 0 {
        let mut remaining = nb as usize;
        while PAGINATING && remaining != 0 {
            let mut q = p;
            while *q != 0 && *q != b'\n' as c_char {
                q = q.add(1);
            }
            if *q != b'\n' as c_char {
                break;
            }
            let count = q.offset_from(p) as c_int + 1;
            rv += udbg_write(p, count);
            p = q.add(1);
            remaining -= count as usize;
            PAGINATE_POS += 1;

            if PAGINATE_POS >= PAGINATE_LPP {
                udbg_write(msg.as_ptr() as *const c_char, msg.len() as c_int - 1);
                match xmon_readchar() {
                    b'a' as c_int => PAGINATING = false,
                    b'q' as c_int => PAGINATE_SKIPPING = true,
                    _ => {},
                }
                PAGINATE_POS = 0;
                udbg_write(b"\r\n".as_ptr() as *const c_char, 2);
                if PAGINATE_SKIPPING {
                    return nb;
                }
            }
        }
    }

    rv + udbg_write(p, nb - p.offset_from(ptr) as c_int)
}

pub unsafe extern "C" fn xmon_putchar(c: c_int) -> c_int {
    let ch = c as c_char;
    if c == b'\n' as c_int {
        xmon_putchar(b'\r' as c_int);
    }
    if xmon_write(&ch, 1) == 1 { c } else { -1 }
}

static mut LINE: [c_char; 256] = [0; 256];
static mut LINEPTR: usize = 0;
static mut LINELEFT: usize = 0;

unsafe fn xmon_getchar() -> c_int {
    if LINELEFT == 0 {
        LINEPTR = 0;
        loop {
            let c = xmon_readchar();
            if c == -1 || c == 4 { break; }
            if c == b'\r' as c_int || c == b'\n' as c_int {
                LINE[LINEPTR] = b'\n' as c_char;
                LINEPTR += 1;
                xmon_putchar(b'\n' as c_int);
                break;
            }
            match c {
                0o177 | 8 => {
                    if LINEPTR != 0 {
                        xmon_putchar(8); xmon_putchar(b' ' as c_int); xmon_putchar(8);
                        LINEPTR -= 1;
                    }
                }
                0x15 => while LINEPTR != 0 {
                    xmon_putchar(8); xmon_putchar(b' ' as c_int); xmon_putchar(8);
                    LINEPTR -= 1;
                },
                _ => {
                    if LINEPTR >= LINE.len() - 1 { xmon_putchar(7); }
                    else { xmon_putchar(c); LINE[LINEPTR] = c as c_char; LINEPTR += 1; }
                }
            }
        }
        LINELEFT = LINEPTR;
        LINEPTR = 0;
    }
    if LINELEFT == 0 { return -1; }
    LINELEFT -= 1;
    let c = LINE[LINEPTR] as u8 as c_int;
    LINEPTR += 1;
    c
}

pub unsafe extern "C" fn xmon_gets(str_: *mut c_char, nb: c_int) -> *mut c_char {
    let mut p = str_;
    while p < str_.add(nb as usize - 1) {
        let c = xmon_getchar();
        if c == -1 { if p == str_ { return core::ptr::null_mut(); } break; }
        *p = c as c_char; p = p.add(1);
        if c == b'\n' as c_int { break; }
    }
    *p = 0;
    str_
}

// The C variadic interface and va_list formatting are retained as an external ABI boundary.
pub unsafe extern "C" fn xmon_printf(format: *const c_char, mut args: ...) {
    let mut xmon_outbuf = [0 as c_char; 1024];
    let n = vsnprintf(xmon_outbuf.as_mut_ptr(), xmon_outbuf.len(), format, &mut args as *mut _ as *mut core::ffi::c_void);
    let rc = xmon_write(xmon_outbuf.as_ptr(), n);
    if n != 0 && rc == 0 { }
}

pub unsafe extern "C" fn xmon_puts(str_: *const c_char) {
    let mut len = 0usize;
    while *str_.add(len) != 0 { len += 1; }
    xmon_write(str_, len as c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
