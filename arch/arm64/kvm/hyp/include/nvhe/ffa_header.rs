/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2022 - Google LLC
 * Author: Andrew Walbran <qwandor@google.com>
 */

// Dependency equivalent of <asm/kvm_host.h>.

use core::ffi::c_void;

#[repr(C)]
pub struct kvm_cpu_context {
    _private: [u8; 0],
}

pub const FFA_MIN_FUNC_NUM: u32 = 0x60;
pub const FFA_MAX_FUNC_NUM: u32 = 0xFF;

unsafe extern "C" {
    pub fn hyp_ffa_init(pages: *mut c_void) -> i32;
    pub fn kvm_host_ffa_handler(
        host_ctxt: *mut kvm_cpu_context,
        func_id: u32,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
