/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: linux/string.h, asm/setup.h, and asm/ipl.h.

use core::ffi::{c_char, c_int, c_ulong, c_void};

extern "C" {
    pub static mut early_command_line: [c_char; COMMAND_LINE_SIZE];
    pub static mut ipl_block: ipl_parameter_block;
    pub static mut ipl_block_valid: c_int;
    pub static mut ipl_secure_flag: c_int;

    pub static mut ipl_cert_list_addr: c_ulong;
    pub static mut ipl_cert_list_size: c_ulong;

    pub static mut early_ipl_comp_list_addr: c_ulong;
    pub static mut early_ipl_comp_list_size: c_ulong;

    pub static mut boot_rb: [c_char; PAGE_SIZE * 2];
    pub static mut boot_earlyprintk: bool;
    pub static mut boot_rb_off: usize;
    pub static mut bootdebug_filter: [c_char; 128];
    pub static mut bootdebug: bool;

    pub fn strlen(s: *const c_char) -> usize;
    pub fn skip_spaces(s: *mut c_char) -> *mut c_char;
    pub fn memscan(addr: *const c_void, size: usize, c: c_int) -> *mut c_void;
    pub fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    pub fn memchr(s: *const c_void, c: c_int, n: usize) -> *mut c_void;
}

// `COMMAND_LINE_SIZE`, `PAGE_SIZE`, and `ipl_parameter_block` are supplied by
// the corresponding translated dependency headers.

#[macro_export]
macro_rules! boot_rb_foreach {
    ($cb:expr) => {{
        unsafe {
            let mut off = boot_rb_off
                + strlen(boot_rb.as_ptr().add(boot_rb_off))
                + 1;
            let mut len: usize;
            while off < boot_rb.len()
                && {
                    len = strlen(boot_rb.as_ptr().add(off));
                    len != 0
                }
            {
                $cb(boot_rb.as_mut_ptr().add(off));
                off += len + 1;
            }
            off = 0;
            while off < boot_rb_off
                && {
                    len = strlen(boot_rb.as_ptr().add(off));
                    len != 0
                }
            {
                $cb(boot_rb.as_mut_ptr().add(off));
                off += len + 1;
            }
        }
    }};
}

/*
 * bootdebug_filter is a comma separated list of strings,
 * where each string can be a prefix of the message.
 */
#[inline]
pub unsafe fn bootdebug_filter_match(buf: *const c_char) -> bool {
    let mut p = bootdebug_filter.as_mut_ptr();
    let s: *mut c_char;
    let end = p.add(strlen(p));

    if *p == 0 {
        return true;
    }

    while p < end {
        p = skip_spaces(p);
        s = memscan(p, end.offset_from(p) as usize, b',' as c_int) as *mut c_char;
        if strncmp(p, buf, s.offset_from(p) as usize) == 0 {
            return true;
        }
        p = s.add(1);
    }
    false
}

#[inline]
pub unsafe fn skip_timestamp(buf: *const c_char) -> *const c_char {
    // CONFIG_PRINTK_TIME conditional from the C header; enable this block
    // when that build-time configuration is present.
    #[cfg(CONFIG_PRINTK_TIME)]
    {
        let p = memchr(buf as *const c_void, b']' as c_int, strlen(buf)) as *const c_char;
        if !p.is_null() && *p.add(1) == b' ' as c_char {
            return p.add(2);
        }
    }
    buf
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
