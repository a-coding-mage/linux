/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2026 Qualcomm Technologies, Inc.
 */

// Dependency intent from <linux/types.h> is preserved through Rust's C ABI
// types and the externally supplied KVM_RISCV_ISA_EXT_* constants.

use core::ffi::{c_ulong, c_int};

extern "C" {
    pub fn kvm_riscv_base2isa_ext(base_ext: c_ulong) -> c_ulong;

    pub fn __kvm_riscv_isa_check_host(ext: c_ulong, base_ext: *mut c_ulong) -> c_int;

    pub fn kvm_riscv_isa_enable_allowed(ext: c_ulong) -> bool;
    pub fn kvm_riscv_isa_disable_allowed(ext: c_ulong) -> bool;
}

macro_rules! kvm_riscv_isa_check_host {
    ($ext:expr) => {
        unsafe {
            __kvm_riscv_isa_check_host($ext, core::ptr::null_mut())
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
