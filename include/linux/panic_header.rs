/* SPDX-License-Identifier: GPL-2.0 */

// Declarations corresponding to linux/compiler_attributes.h,
// linux/stdarg.h, and linux/types.h are supplied by other translation units.

use core::ffi::{c_char, c_int, c_uint, c_ulong, VaList};

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    pub static mut panic_blink: Option<unsafe extern "C" fn(state: c_int) -> c_long>;

    pub fn panic(fmt: *const c_char, ...) -> !;
    pub fn vpanic(fmt: *const c_char, args: VaList<'_>) -> !;
    pub fn nmi_panic(regs: *mut pt_regs, msg: *const c_char);
    pub fn check_panic_on_warn(origin: *const c_char);
    pub fn oops_enter();
    pub fn oops_exit();
    pub fn oops_may_print() -> bool;

    pub static mut panic_triggering_all_cpu_backtrace: bool;
    pub static mut panic_timeout: c_int;
    pub static mut panic_print: c_ulong;
    pub static mut panic_on_oops: c_int;
    pub static mut panic_on_warn: c_int;

    pub static mut panic_on_taint: c_ulong;
    pub static mut panic_on_taint_nousertaint: bool;

    pub static mut sysctl_panic_on_stackoverflow: c_int;

    pub static mut crash_kexec_post_notifiers: bool;

    pub fn __stack_chk_fail();
    pub fn abort();

    pub static mut panic_cpu: atomic_t;
    pub static mut panic_redirect_cpu: atomic_t;

    pub fn panic_try_start() -> bool;
    pub fn panic_reset();
    pub fn panic_in_progress() -> bool;
    pub fn panic_on_this_cpu() -> bool;
    pub fn panic_on_other_cpu() -> bool;

    pub static taint_flags: [taint_flag; TAINT_FLAGS_COUNT as usize];

    pub fn print_tainted() -> *const c_char;
    pub fn print_tainted_verbose() -> *const c_char;
    pub fn add_taint(flag: c_uint, lockdep: lockdep_ok);
    pub fn test_taint(flag: c_uint) -> c_int;
    pub fn get_taint() -> c_ulong;
}

// Supplied by linux/types.h.
#[repr(C)]
pub struct atomic_t {
    pub counter: c_int,
}

pub const PANIC_CPU_INVALID: c_int = -1;

/// Only to be used by arch init code. If the user over-wrote the default
/// CONFIG_PANIC_TIMEOUT, honor it.
#[inline]
pub unsafe fn set_arch_panic_timeout(timeout: c_int, arch_default_timeout: c_int) {
    if panic_timeout == arch_default_timeout {
        panic_timeout = timeout;
    }
}

/* This cannot be an enum because some may be used in assembly source. */
pub const TAINT_PROPRIETARY_MODULE: c_uint = 0;
pub const TAINT_FORCED_MODULE: c_uint = 1;
pub const TAINT_CPU_OUT_OF_SPEC: c_uint = 2;
pub const TAINT_FORCED_RMMOD: c_uint = 3;
pub const TAINT_MACHINE_CHECK: c_uint = 4;
pub const TAINT_BAD_PAGE: c_uint = 5;
pub const TAINT_USER: c_uint = 6;
pub const TAINT_DIE: c_uint = 7;
pub const TAINT_OVERRIDDEN_ACPI_TABLE: c_uint = 8;
pub const TAINT_WARN: c_uint = 9;
pub const TAINT_CRAP: c_uint = 10;
pub const TAINT_FIRMWARE_WORKAROUND: c_uint = 11;
pub const TAINT_OOT_MODULE: c_uint = 12;
pub const TAINT_UNSIGNED_MODULE: c_uint = 13;
pub const TAINT_SOFTLOCKUP: c_uint = 14;
pub const TAINT_LIVEPATCH: c_uint = 15;
pub const TAINT_AUX: c_uint = 16;
pub const TAINT_RANDSTRUCT: c_uint = 17;
pub const TAINT_TEST: c_uint = 18;
pub const TAINT_FWCTL: c_uint = 19;
pub const TAINT_FLAGS_COUNT: c_uint = 20;
pub const TAINT_FLAGS_MAX: c_ulong = (1u64 << TAINT_FLAGS_COUNT) as c_ulong - 1;

#[repr(C)]
pub struct taint_flag {
    pub c_true: c_char,
    pub c_false: c_char,
    pub desc: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum lockdep_ok {
    LOCKDEP_STILL_OK,
    LOCKDEP_NOW_UNRELIABLE,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
