/* SPDX-License-Identifier: GPL-2.0 */

// Dependency corresponding to <asm/vdso/vsyscall.h>.

#[cfg(not(feature = "assembler"))]
unsafe extern "C" {
    pub fn vdso_update_begin() -> core::ffi::c_ulong;
    pub fn vdso_update_end(flags: core::ffi::c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
