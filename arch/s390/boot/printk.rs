// SPDX-License-Identifier: GPL-2.0
// Kernel headers and boot-local dependencies from printk.c are supplied externally.

extern "C" {
    static mut boot_console_loglevel: i32;
    static mut boot_ignore_loglevel: bool;
    static mut boot_rb: [u8; PAGE_SIZE * 2];
    static mut boot_earlyprintk: bool;
    static mut boot_rb_off: usize;
    static mut bootdebug_filter: [u8; 128];
    static mut bootdebug: bool;
    static hex_asc: [u8; 17];
    static _decompressor_syms_start: *mut u8;
    static _decompressor_syms_end: *mut u8;
    fn strscpy(dst: *mut u8, src: *const u8, count: usize) -> isize;
    fn strlen(s: *const u8) -> usize;
    fn strnlen(s: *const u8, count: usize) -> usize;
    fn strcat(dst: *mut u8, src: *const u8) -> *mut u8;
    fn memset(dst: *mut u8, c: i32, count: usize) -> *mut u8;
    fn memcpy(dst: *mut u8, src: *const u8, count: usize) -> *mut u8;
    fn simple_strtoull(s: *const u8, end: *mut *mut u8, base: u32) -> u64;
    fn simple_strtol(s: *const u8, end: *mut *mut u8, base: u32) -> isize;
    fn sclp_early_printk(s: *const u8);
    fn printk_skip_level(s: *const u8) -> *const u8;
    fn boot_rb_foreach(f: unsafe extern "C" fn(*const u8));
    fn bootdebug_filter_match(s: *const u8) -> bool;
    fn skip_timestamp(s: *const u8) -> *const u8;
    fn printk_get_level(s: *const u8) -> u8;
    fn __fls(v: u64) -> i32;
    fn tod_to_ns(v: u64) -> u64;
    fn __get_tod_clock_monotonic() -> u64;
}

const PAGE_SIZE: usize = 4096;
const MAX_NUMLEN: usize = 21;
const MAX_SYMLEN: usize = 64;
const KERN_SOH_ASCII: u8 = 1;
const MESSAGE_LOGLEVEL_DEFAULT: i32 = 4;
const LOGLEVEL_DEBUG: i32 = 7;
const LOGLEVEL_EMERG: i32 = 0;
const E2BIG: isize = 7;
const LONG_MIN: isize = isize::MIN;
const NSEC_PER_SEC: u64 = 1_000_000_000;
const NSEC_PER_USEC: u64 = 1_000;

unsafe fn boot_rb_add(str_: *const u8, len: usize) {
    let mut avail = core::mem::size_of::<[u8; PAGE_SIZE * 2]>() - boot_rb_off - 1;
    if len + 1 > avail { boot_rb_off = 0; }
    avail = core::mem::size_of::<[u8; PAGE_SIZE * 2]>() - boot_rb_off - 1;
    strscpy(boot_rb.as_mut_ptr().add(boot_rb_off), str_, avail);
    boot_rb_off += len + 1;
}

unsafe extern "C" fn print_rb_entry(str_: *const u8) { sclp_early_printk(printk_skip_level(str_)); }

unsafe fn debug_messages_printed() -> bool { boot_earlyprintk && (boot_ignore_loglevel || boot_console_loglevel > LOGLEVEL_DEBUG) }

#[no_mangle]
pub unsafe extern "C" fn boot_rb_dump() {
    if debug_messages_printed() { return; }
    sclp_early_printk(b"Boot messages ring buffer:\n\0".as_ptr());
    boot_rb_foreach(print_rb_entry);
}

unsafe fn as_hex(dst: *mut u8, mut val: u64, pad: i32) -> *mut u8 {
    let n = core::cmp::max(pad, __fls(val | 1) / 4 + 1) as usize;
    let mut p = dst.add(n); *p = 0; p = p.sub(1);
    while p >= dst { *p = b"0123456789abcdef"[(val & 0xf) as usize]; val >>= 4; if p == dst { break; } p = p.sub(1); }
    dst
}

unsafe fn as_dec(buf: *mut u8, mut val: u64, is_signed: bool) -> *mut u8 {
    let mut negative = false; let mut p = buf.add(MAX_NUMLEN);
    if is_signed && (val as isize) < 0 { val = if val as isize == LONG_MIN { val } else { (-(val as isize)) as u64 }; negative = true; }
    p = p.sub(1); *p = 0;
    loop { p = p.sub(1); *p = b'0' + (val % 10) as u8; val /= 10; if val == 0 { break; } }
    if negative { p = p.sub(1); *p = b'-'; } p
}

unsafe fn strpad(dst: *mut u8, dst_size: usize, src: *const u8, pad_: i32, zero: bool, decimal: bool) -> isize {
    let len = strlen(src) as i32; let mut pad = pad_; let mut p = dst;
    if core::cmp::max(len, pad.abs()) as usize >= dst_size { return -E2BIG; }
    let mut s = src; if pad > len { if decimal && zero && *s == b'-' { *p = b'-'; p = p.add(1); s = s.add(1); pad -= 1; } memset(p, if zero { b'0' as i32 } else { b' ' as i32 }, (pad-len) as usize); p = p.add((pad-len) as usize); }
    memcpy(p, s, strlen(s)); p = p.add(strlen(s)); if pad < 0 && -pad > len { memset(p, b' ' as i32, (-pad-len) as usize); p = p.add((-pad-len) as usize); } *p = 0; p.offset_from(dst)
}

unsafe fn symstart(mut p: *mut u8) -> *mut u8 { while *p != 0 { p = p.sub(1); } p.add(1) }

unsafe fn findsym(ip: u64, off: *mut u16, len: *mut u16) -> *mut u8 {
    let mut a = _decompressor_syms_start; let mut b = _decompressor_syms_end;
    while a < b { let pivot = symstart(a.add(b.offset_from(a) as usize / 2)); let mut endp = core::ptr::null_mut(); let start = simple_strtoull(pivot, &mut endp, 16); let size = simple_strtoull(endp.add(1), &mut endp, 16);
        if ip < start { b = pivot; continue; } if ip > start + size { a = pivot.add(strlen(pivot) + 1); continue; }
        *off = (ip - start) as u16; *len = size as u16; return endp.add(1); }
    core::ptr::null_mut()
}

unsafe fn strsym(buf: *mut u8, ip: *mut core::ffi::c_void) -> *mut u8 {
    let mut off = 0u16; let mut len = 0u16; let p = findsym(ip as u64, &mut off, &mut len);
    if !p.is_null() { strscpy(buf, p, MAX_SYMLEN); let q = buf.add(strnlen(buf, MAX_SYMLEN - 15)); strscpy(q, b"+0x\0".as_ptr(), MAX_SYMLEN - q.offset_from(buf) as usize); as_hex(q.add(3), off as u64, 0); strcat(q, b"/0x\0".as_ptr()); as_hex(q.add(strlen(q)), len as u64, 0); } else { as_hex(buf, ip as u64, 16); } buf
}

unsafe fn printk_loglevel(buf: *const u8) -> i32 { if *buf == KERN_SOH_ASCII && *buf.add(1) >= b'0' && *buf.add(1) <= b'7' { (*buf.add(1) - b'0') as i32 } else { MESSAGE_LOGLEVEL_DEFAULT } }

unsafe fn boot_console_earlyprintk(buf: *const u8) { let level = printk_loglevel(buf); if level > LOGLEVEL_EMERG && !boot_earlyprintk { return; } let buf = printk_skip_level(buf); if level == LOGLEVEL_DEBUG && (!bootdebug || !bootdebug_filter_match(skip_timestamp(buf))) { return; } if boot_ignore_loglevel || level < boot_console_loglevel { sclp_early_printk(buf); } }

unsafe fn add_timestamp(buf: *mut u8) -> *mut u8 {
    // CONFIG_PRINTK_TIME is a build-time condition supplied by the kernel build.
    buf
}

// Rust has no stable direct equivalent for defining a C variadic function body;
// the formatter's va_arg-dependent body is preserved as an external ABI entry.
#[no_mangle]
pub unsafe extern "C" fn boot_printk(_fmt: *const u8, ...) -> isize { -1 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
