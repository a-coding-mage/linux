/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: declarations supplied by <asm/kdebug.h>.

#[repr(C)]
pub struct notifier_block;

#[repr(C)]
pub struct pt_regs;

#[repr(C)]
pub struct die_args {
    pub regs: *mut pt_regs,
    pub str_: *const ::core::ffi::c_char,
    pub err: ::core::ffi::c_long,
    pub trapnr: ::core::ffi::c_int,
    pub signr: ::core::ffi::c_int,
}

extern "C" {
    pub fn register_die_notifier(nb: *mut notifier_block) -> ::core::ffi::c_int;
    pub fn unregister_die_notifier(nb: *mut notifier_block) -> ::core::ffi::c_int;

    pub fn notify_die(
        val: die_val,
        str_: *const ::core::ffi::c_char,
        regs: *mut pt_regs,
        err: ::core::ffi::c_long,
        trap: ::core::ffi::c_int,
        sig: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
