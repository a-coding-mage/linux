// SPDX-License-Identifier: GPL-2.0
// Translated from printk.h. Configuration-dependent declarations retain their
// original intent; symbols from included kernel headers are external deps.

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)] pub struct console { _private: [u8; 0] }
#[repr(C)] pub struct dev_printk_info { _private: [u8; 0] }
#[repr(C)] pub struct file_operations { _private: [u8; 0] }
pub type va_list = c_void;
pub type size_t = usize;
pub type u32 = u32;
pub type ulong = usize;

extern "C" {
    pub static linux_banner: [c_char; 0];
    pub static linux_proc_banner: [c_char; 0];
    pub static mut oops_in_progress: c_int;
    pub static mut console_printk: [c_int; 4];
    pub static mut devkmsg_log_str: [c_char; DEVKMSG_STR_MAX_SIZE];
    pub static mut suppress_printk: c_int;
    pub static mut printk_delay_msec: c_int;
    pub static mut dmesg_restrict: c_int;
    pub static kmsg_fops: file_operations;
}

pub const PRINTK_MAX_SINGLE_HEADER_LEN: c_int = 2;
pub const CONSOLE_LOGLEVEL_SILENT: c_int = 0;
pub const CONSOLE_LOGLEVEL_MIN: c_int = 1;
pub const CONSOLE_LOGLEVEL_DEBUG: c_int = 10;
pub const CONSOLE_LOGLEVEL_MOTORMOUTH: c_int = 15;
pub const DEVKMSG_STR_MAX_SIZE: usize = 10;

#[inline] pub unsafe fn printk_get_level(buffer: *const c_char) -> c_int {
    if *buffer == KERN_SOH_ASCII && *buffer.add(1) != 0 {
        let b = *buffer.add(1);
        if (b >= b'0' as c_char && b <= b'7' as c_char) || b == b'c' as c_char { return b as c_int; }
    }
    0
}
#[inline] pub unsafe fn printk_skip_level(buffer: *const c_char) -> *const c_char {
    if printk_get_level(buffer) != 0 { buffer.add(2) } else { buffer }
}
#[inline] pub unsafe fn printk_skip_headers(mut buffer: *const c_char) -> *const c_char {
    while printk_get_level(buffer) != 0 { buffer = printk_skip_level(buffer); }
    buffer
}

pub const MESSAGE_LOGLEVEL_DEFAULT: c_int = CONFIG_MESSAGE_LOGLEVEL_DEFAULT;
pub const CONSOLE_LOGLEVEL_DEFAULT: c_int = CONFIG_CONSOLE_LOGLEVEL_DEFAULT;
pub const CONSOLE_LOGLEVEL_QUIET: c_int = CONFIG_CONSOLE_LOGLEVEL_QUIET;

#[no_mangle] pub unsafe extern "C" fn console_loglevel() -> *mut c_int { console_printk.as_mut_ptr() }
#[no_mangle] pub unsafe extern "C" fn default_message_loglevel() -> *mut c_int { console_printk.as_mut_ptr().add(1) }
#[no_mangle] pub unsafe extern "C" fn minimum_console_loglevel() -> *mut c_int { console_printk.as_mut_ptr().add(2) }
#[no_mangle] pub unsafe extern "C" fn default_console_loglevel() -> *mut c_int { console_printk.as_mut_ptr().add(3) }

extern "C" {
    pub fn match_devname_and_update_preferred_console(match_: *const c_char, name: *const c_char, idx: i16) -> c_int;
    pub fn console_verbose();
    pub fn early_printk(fmt: *const c_char, ...);
    pub fn vprintk_emit(facility: c_int, level: c_int, dev_info: *const dev_printk_info, fmt: *const c_char, args: *mut va_list) -> c_int;
    pub fn vprintk(fmt: *const c_char, args: *mut va_list) -> c_int;
    pub fn vprintk_deferred(fmt: *const c_char, args: *mut va_list) -> c_int;
    pub fn _printk(fmt: *const c_char, ... ) -> c_int;
    pub fn _printk_deferred(fmt: *const c_char, ... ) -> c_int;
    pub fn __printk_deferred_enter(); pub fn __printk_deferred_exit();
    pub fn printk_force_console_enter(); pub fn printk_force_console_exit();
    pub fn __printk_ratelimit(func: *const c_char) -> c_int;
    pub fn printk_timed_ratelimit(caller_jiffies: *mut ulong, interval_msec: c_uint) -> bool;
    pub fn wake_up_klogd();
    pub fn log_buf_addr_get() -> *mut c_char;
    pub fn log_buf_len_get() -> u32;
    pub fn log_buf_vmcoreinfo_setup(); pub fn setup_log_buf(early: c_int);
    pub fn dump_stack_set_arch_desc(fmt: *const c_char, ...);
    pub fn dump_stack_print_info(log_lvl: *const c_char); pub fn show_regs_print_info(log_lvl: *const c_char);
    pub fn dump_stack_lvl(log_lvl: *const c_char); pub fn dump_stack();
    pub fn printk_trigger_flush(); pub fn console_try_replay_all(); pub fn printk_legacy_allow_panic_sync();
    pub fn nbcon_device_try_acquire(con: *mut console) -> bool; pub fn nbcon_device_release(con: *mut console);
    pub fn nbcon_atomic_flush_unsafe(); pub fn pr_flush(timeout_ms: c_int, reset_on_progress: bool) -> bool;
}

#[repr(C)] pub struct va_format { pub fmt: *const c_char, pub va: *mut va_list }
pub const FW_BUG: &[u8] = b"[Firmware Bug]: \0";
pub const FW_WARN: &[u8] = b"[Firmware Warn]: \0";
pub const FW_INFO: &[u8] = b"[Firmware Info]: \0";
pub const HW_ERR: &[u8] = b"[Hardware Error]: \0";
pub const DEPRECATED: &[u8] = b"[Deprecated]: \0";

#[inline] pub unsafe fn printk_deferred_enter() { __printk_deferred_enter(); }
#[inline] pub unsafe fn printk_deferred_exit() { __printk_deferred_exit(); }
#[inline] pub unsafe fn printk_ratelimit() -> c_int { __printk_ratelimit(core::ptr::null()) }

#[cfg(not(CONFIG_PRINTK))]
pub mod no_printk_config {
    use super::*;
    #[inline] pub unsafe fn vprintk(_: *const c_char, _: *mut va_list) -> c_int { 0 }
    #[inline] pub unsafe fn vprintk_deferred(_: *const c_char, _: *mut va_list) -> c_int { 0 }
    #[inline] pub unsafe fn _printk(_: *const c_char, ...) -> c_int { 0 }
    #[inline] pub unsafe fn _printk_deferred(_: *const c_char, ...) -> c_int { 0 }
    #[inline] pub fn printk_deferred_enter() {} #[inline] pub fn printk_deferred_exit() {}
    #[inline] pub fn printk_force_console_enter() {} #[inline] pub fn printk_force_console_exit() {}
    #[inline] pub fn printk_ratelimit() -> c_int { 0 }
    #[inline] pub fn printk_timed_ratelimit(_: *mut ulong, _: c_uint) -> bool { false }
    #[inline] pub fn wake_up_klogd() {} #[inline] pub fn log_buf_addr_get() -> *mut c_char { core::ptr::null_mut() }
    #[inline] pub fn log_buf_len_get() -> u32 { 0 } #[inline] pub fn log_buf_vmcoreinfo_setup() {}
    #[inline] pub fn setup_log_buf(_: c_int) {} #[inline] pub fn dump_stack_set_arch_desc(_: *const c_char, ...) {}
    #[inline] pub fn dump_stack_print_info(_: *const c_char) {} #[inline] pub fn show_regs_print_info(_: *const c_char) {}
    #[inline] pub fn dump_stack_lvl(_: *const c_char) {} #[inline] pub fn dump_stack() {}
    #[inline] pub fn printk_trigger_flush() {} #[inline] pub fn console_try_replay_all() {}
    #[inline] pub fn printk_legacy_allow_panic_sync() {} #[inline] pub fn nbcon_device_try_acquire(_: *mut console) -> bool { false }
    #[inline] pub fn nbcon_device_release(_: *mut console) {} #[inline] pub fn nbcon_atomic_flush_unsafe() {}
    #[inline] pub fn pr_flush(_: c_int, _: bool) -> bool { true }
}

#[repr(C)] pub enum DumpPrefix { None, Address, Offset }
extern "C" {
    pub fn hex_dump_to_buffer(buf: *const c_void, len: size_t, rowsize: c_int, groupsize: c_int, linebuf: *mut c_char, linebuflen: size_t, ascii: bool) -> c_int;
    pub fn print_hex_dump(level: *const c_char, prefix_str: *const c_char, prefix_type: c_int, rowsize: c_int, groupsize: c_int, buf: *const c_void, len: size_t, ascii: bool);
}

// C preprocessor interfaces are represented as Rust macros, preserving call shape.
#[macro_export] macro_rules! no_printk { ($fmt:expr $(, $arg:expr)*) => {{ let _ = ($fmt $(, $arg)*); 0 }}; }
#[macro_export] macro_rules! pr_fmt { ($fmt:expr) => { $fmt }; }
#[macro_export] macro_rules! printk { ($fmt:expr $(, $arg:expr)*) => { $crate::_printk($fmt $(, $arg)*) }; }
#[macro_export] macro_rules! printk_deferred { ($fmt:expr $(, $arg:expr)*) => { $crate::_printk_deferred($fmt $(, $arg)*) }; }
#[macro_export] macro_rules! pr_emerg { ($fmt:expr $(, $arg:expr)*) => { printk!(concat!(KERN_EMERG, $fmt) $(, $arg)*) }; }
#[macro_export] macro_rules! pr_alert { ($fmt:expr $(, $arg:expr)*) => { printk!(concat!(KERN_ALERT, $fmt) $(, $arg)*) }; }
#[macro_export] macro_rules! pr_crit { ($fmt:expr $(, $arg:expr)*) => { printk!(concat!(KERN_CRIT, $fmt) $(, $arg)*) }; }
#[macro_export] macro_rules! pr_err { ($fmt:expr $(, $arg:expr)*) => { printk!(concat!(KERN_ERR, $fmt) $(, $arg)*) }; }
#[macro_export] macro_rules! pr_warn { ($fmt:expr $(, $arg:expr)*) => { printk!(concat!(KERN_WARNING, $fmt) $(, $arg)*) }; }
#[macro_export] macro_rules! pr_notice { ($fmt:expr $(, $arg:expr)*) => { printk!(concat!(KERN_NOTICE, $fmt) $(, $arg)*) }; }
#[macro_export] macro_rules! pr_info { ($fmt:expr $(, $arg:expr)*) => { printk!(concat!(KERN_INFO, $fmt) $(, $arg)*) }; }
#[macro_export] macro_rules! pr_cont { ($fmt:expr $(, $arg:expr)*) => { printk!(concat!(KERN_CONT, $fmt) $(, $arg)*) }; }
#[macro_export] macro_rules! pr_devel { ($fmt:expr $(, $arg:expr)*) => { no_printk!(concat!(KERN_DEBUG, $fmt) $(, $arg)*) }; }
#[macro_export] macro_rules! pr_debug { ($fmt:expr $(, $arg:expr)*) => { no_printk!(concat!(KERN_DEBUG, $fmt) $(, $arg)*) }; }
#[macro_export] macro_rules! print_hex_dump_bytes { ($prefix:expr, $ptype:expr, $buf:expr, $len:expr) => { print_hex_dump_debug!($prefix, $ptype, 16, 1, $buf, $len, true) }; }
#[macro_export] macro_rules! print_hex_dump_debug { ($prefix:expr, $ptype:expr, $rows:expr, $groups:expr, $buf:expr, $len:expr, $ascii:expr) => { () }; }
#[macro_export] macro_rules! print_hex_dump_devel { ($prefix:expr, $ptype:expr, $rows:expr, $groups:expr, $buf:expr, $len:expr, $ascii:expr) => { () }; }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
