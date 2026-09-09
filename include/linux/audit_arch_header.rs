/* SPDX-License-Identifier: GPL-2.0-or-later */
/* audit_arch_header.rs -- Arch layer specific support for audit
 *
 * Copyright 2021 Red Hat Inc., Durham, North Carolina.
 * All Rights Reserved.
 *
 * Author: Richard Guy Briggs <rgb@redhat.com>
 */

use core::ffi::{c_int, c_uint};

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum auditsc_class_t {
    AUDITSC_NATIVE = 0,
    AUDITSC_COMPAT,
    AUDITSC_OPEN,
    AUDITSC_OPENAT,
    AUDITSC_SOCKETCALL,
    AUDITSC_EXECVE,
    AUDITSC_OPENAT2,

    AUDITSC_NVALS, /* count */
}

unsafe extern "C" {
    pub fn audit_classify_compat_syscall(abi: c_int, syscall: c_uint) -> c_int;

    /* only for compat system calls */
    pub static mut compat_write_class: [c_uint; 0];
    pub static mut compat_read_class: [c_uint; 0];
    pub static mut compat_dir_class: [c_uint; 0];
    pub static mut compat_chattr_class: [c_uint; 0];
    pub static mut compat_signal_class: [c_uint; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
