/* SPDX-License-Identifier: GPL-2.0 */
// C header guard: _ASM_POWERPC_SIGNAL_H

// #define __ARCH_HAS_SA_RESTORER
pub const __ARCH_HAS_SA_RESTORER: bool = true;

// Dependency: <uapi/asm/signal.h>
// Dependency: <uapi/asm/ptrace.h>

#[repr(C)]
pub struct pt_regs;

extern "C" {
    pub fn get_min_sigframe_size_32() -> core::ffi::c_ulong;
    pub fn get_min_sigframe_size_64() -> core::ffi::c_ulong;
    pub fn get_min_sigframe_size() -> core::ffi::c_ulong;
    pub fn get_min_sigframe_size_compat() -> core::ffi::c_ulong;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
