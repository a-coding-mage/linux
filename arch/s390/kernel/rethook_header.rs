/* SPDX-License-Identifier: GPL-2.0-only */

// Declaration supplied by the architecture's register definitions.
#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    pub fn arch_rethook_trampoline_callback(regs: *mut pt_regs) -> core::ffi::c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
