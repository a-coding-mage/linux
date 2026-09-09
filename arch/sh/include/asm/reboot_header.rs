/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <linux/kdebug.h>

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine_ops {
    pub restart: Option<unsafe extern "C" fn(cmd: *mut ::core::ffi::c_char)>,
    pub halt: Option<unsafe extern "C" fn()>,
    pub power_off: Option<unsafe extern "C" fn()>,
    pub shutdown: Option<unsafe extern "C" fn()>,
    pub crash_shutdown: Option<unsafe extern "C" fn(regs: *mut pt_regs)>,
}

extern "C" {
    pub static mut machine_ops: machine_ops;

    /* arch/sh/kernel/machine_kexec.c */
    pub fn native_machine_crash_shutdown(regs: *mut pt_regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
