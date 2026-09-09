// SPDX-License-Identifier: GPL-2.0
/* Kernel Debugger Architecture Independent Console I/O handler */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const CMD_BUFLEN: usize = 256;
pub static mut kdb_prompt_str: [c_char; CMD_BUFLEN] = [0; CMD_BUFLEN];
pub static mut kdb_trap_printk: c_int = 0;
pub static mut kdb_printf_cpu: c_int = -1;

type GetCharFunc = unsafe extern "C" fn() -> c_int;
extern "C" {
    static mut kdb_poll_funcs: *mut GetCharFunc;
    static mut kdb_nextline: c_int;
    static mut kdb_grepping_flag: c_int;
    static mut kdb_grep_leading: bool;
    static mut kdb_grep_trailing: bool;
    static mut kdb_grep_string: *mut c_char;
    static mut dbg_kdb_mode: bool;
    static mut kgdb_connected: bool;
    static mut console_loglevel: c_int;
    static mut dbg_io_ops: *mut c_void;
    fn kdb_gdb_state_pass(s: *const c_char);
    fn touch_nmi_watchdog();
    fn udelay(x: c_uint);
    fn mdelay(x: c_uint);
    fn kdbgetintenv(name: *const c_char, value: *mut c_int) -> c_int;
    fn kdbgetenv(name: *const c_char) -> *mut c_char;
    fn kdb_input_flush();
    fn kdb_getstr(buffer: *mut c_char, size: usize, prompt: *const c_char) -> *mut c_char;
    fn kallsyms_symbol_complete(buf: *mut c_char, size: usize) -> c_int;
    fn kallsyms_symbol_next(buf: *mut c_char, idx: c_int, size: usize) -> c_int;
    fn vkdb_printf(src: c_int, fmt: *const c_char, ap: *mut c_void) -> c_int;
}

const E2BIG: c_int = 7;
const KDB_MSGSRC_INTERNAL: c_int = 0;
const KDB_MSGSRC_PRINTK: c_int = 1;
const KDB_GREPPING_FLAG_SEARCH: c_int = 2;
const CON_NBCON: c_int = 1;
const CONSOLE_LOGLEVEL_SILENT: c_int = 1;

unsafe fn kgdb_transition_check(buffer: *mut c_char) -> c_int {
    if *buffer != b'+' as c_char && *buffer != b'$' as c_char {
        kdb_printf(b"%s\0".as_ptr() as *const c_char, buffer);
    } else {
        let slen = libc::strlen(buffer);
        if slen > 3 && *buffer.add(slen - 3) == b'#' as c_char {
            kdb_gdb_state_pass(buffer);
            libc::strcpy(buffer, b"kgdb\0".as_ptr() as *const c_char);
            return 1;
        }
    }
    0
}

unsafe fn kdb_handle_escape(buf: *mut c_char, sz: usize) -> c_int {
    let last = *buf.add(sz - 1);
    match sz {
        1 if last == 27 => 0,
        2 if last == b'[' as c_char => 0,
        3 => match last {
            c if c == b'A' as c_char => 16,
            c if c == b'B' as c_char => 14,
            c if c == b'C' as c_char => 6,
            c if c == b'D' as c_char => 2,
            c if c == b'1' as c_char || c == b'3' as c_char || c == b'4' as c_char => 0,
            _ => -1,
        },
        4 if last == b'~' as c_char => match *buf.add(2) {
            c if c == b'1' as c_char => 1,
            c if c == b'3' as c_char => 4,
            c if c == b'4' as c_char => 5,
            _ => -1,
        },
        _ => -1,
    }
}

pub unsafe fn kdb_getchar() -> c_char {
    let mut buf = [0 as c_char; 4];
    let mut p = 0usize;
    let mut delay = 0;
    let mut previous: *mut GetCharFunc = core::ptr::null_mut();
    static mut last_cr: bool = false;
    loop {
        let mut f = kdb_poll_funcs;
        loop {
            if (*f).is_null() { touch_nmi_watchdog(); f = kdb_poll_funcs; }
            let key = (*f)();
            if key != -1 {
                if last_cr && key == 10 { last_cr = false; continue; }
                last_cr = key == 13;
                if previous != f { previous = f; p = 0; delay = 2000; }
                buf[p] = key as c_char; p += 1;
                let mapped = kdb_handle_escape(buf.as_mut_ptr(), p);
                if mapped < 0 { return buf[if p == 2 { 1 } else { 0 }]; }
                if mapped > 0 { return mapped as c_char; }
            } else if delay != 0 {
                udelay(1000); delay -= 1; if delay == 0 { return 27; }
            }
            f = f.add(1);
        }
    }
}

unsafe fn kdb_position_cursor(prompt: *mut c_char, buffer: *mut c_char, cp: *mut c_char) {
    kdb_printf(b"\r%s\0".as_ptr() as *const c_char, prompt);
    if cp > buffer { kdb_printf(b"%.*s\0".as_ptr() as *const c_char, cp.offset_from(buffer) as c_int, buffer); }
}

unsafe fn kdb_read(buffer: *mut c_char, bufsize: usize) -> *mut c_char {
    let mut cp = buffer;
    let bufend = buffer.add(bufsize - 2);
    let mut lastchar = cp;
    let mut tab = 0;
    let mut tmpbuffer = [0 as c_char; CMD_BUFLEN];
    let mut dtab = 0;
    if kdbgetintenv(b"DTABCOUNT\0".as_ptr() as *const c_char, &mut dtab) != 0 { dtab = 30; }
    let len = libc::strlen(buffer);
    cp = cp.add(len);
    if len > 0 && *buffer.add(len - 1) == 10 { cp = cp.sub(1); }
    lastchar = cp; *cp = 0; kdb_printf(b"%s\0".as_ptr() as *const c_char, buffer);
    loop {
        let key = kdb_getchar() as c_int; if key != 9 { tab = 0; }
        match key {
            8 if cp > buffer => { libc::memmove(cp.sub(1) as *mut c_void, cp as *const c_void, lastchar.offset_from(cp) as usize + 1); lastchar = lastchar.sub(1); cp = cp.sub(1); kdb_printf(b"\b%s \0".as_ptr() as *const c_char, cp); kdb_position_cursor(kdb_prompt_str.as_mut_ptr(), buffer, cp); }
            10 | 13 => { *lastchar = 10; *lastchar.add(1) = 0; kdb_printf(b"\n\0".as_ptr() as *const c_char); return buffer; }
            4 if cp < lastchar => { libc::memmove(cp as *mut c_void, cp.add(1) as *const c_void, lastchar.offset_from(cp) as usize); lastchar = lastchar.sub(1); kdb_printf(b"%s \0".as_ptr() as *const c_char, cp); kdb_position_cursor(kdb_prompt_str.as_mut_ptr(), buffer, cp); }
            1 => { cp = buffer; kdb_position_cursor(kdb_prompt_str.as_mut_ptr(), buffer, cp); }
            5 => { kdb_printf(b"%s\0".as_ptr() as *const c_char, cp); cp = lastchar; }
            2 if cp > buffer => { kdb_printf(b"\b\0".as_ptr() as *const c_char); cp = cp.sub(1); }
            6 if cp < lastchar => { kdb_printf(b"%c\0".as_ptr() as *const c_char, *cp as c_int); cp = cp.add(1); }
            14 | 16 => { *lastchar = key as c_char; *lastchar.add(1) = 0; return lastchar; }
            9 => { if tab < 2 { tab += 1; } let old = *cp; *cp = 0; let start = libc::strrchr(buffer, b' ' as c_int); let start = if start.is_null() { buffer } else { start.add(1) }; libc::strcpy(tmpbuffer.as_mut_ptr(), start); *cp = old; let oldlen = libc::strlen(tmpbuffer.as_mut_ptr()); let count = kallsyms_symbol_complete(tmpbuffer.as_mut_ptr(), CMD_BUFLEN); if tab != 2 && count > 0 { let mut n = libc::strlen(tmpbuffer.as_mut_ptr()) - oldlen; if lastchar.add(n) >= bufend { n = bufend.offset_from(lastchar) as usize; } if n != 0 { libc::memmove(cp.add(n) as *mut c_void, cp as *const c_void, lastchar.offset_from(cp) as usize + 1); libc::memcpy(cp as *mut c_void, tmpbuffer.as_ptr().add(oldlen) as *const c_void, n); cp = cp.add(n); lastchar = lastchar.add(n); } } }
            c if c >= 32 && lastchar < bufend => { if cp < lastchar { libc::memmove(cp.add(1) as *mut c_void, cp as *const c_void, lastchar.offset_from(cp) as usize + 1); lastchar = lastchar.add(1); *cp = c as c_char; cp = cp.add(1); } else { lastchar = lastchar.add(1); *lastchar = 0; *cp = c as c_char; cp = cp.add(1); } }
            _ => {}
        }
    }
}

pub unsafe fn kdb_getstr(buffer: *mut c_char, bufsize: usize, prompt: *const c_char) -> *mut c_char {
    if !prompt.is_null() { libc::strcpy(kdb_prompt_str.as_mut_ptr(), prompt); }
    kdb_printf(b"%s\0".as_ptr() as *const c_char, kdb_prompt_str.as_mut_ptr());
    kdb_read(buffer, bufsize)
}

#[no_mangle]
pub unsafe extern "C" fn kdb_printf(_fmt: *const c_char, ...) -> c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
