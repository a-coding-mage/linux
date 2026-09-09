/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _SPARC64_SYSCALLS_H

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    // asmlinkage
    pub fn sparc_fork(regs: *mut pt_regs) -> core::ffi::c_long;
    // asmlinkage
    pub fn sparc_vfork(regs: *mut pt_regs) -> core::ffi::c_long;
    // asmlinkage
    pub fn sparc_clone(regs: *mut pt_regs) -> core::ffi::c_long;
    // asmlinkage
    pub fn sparc_clone3(regs: *mut pt_regs) -> core::ffi::c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
