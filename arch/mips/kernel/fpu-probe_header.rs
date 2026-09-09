/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependencies supplied by the surrounding kernel translation:
// linux/kernel.h, asm/cpu.h, and asm/cpu-info.h

#[cfg(feature = "CONFIG_MIPS_FP_SUPPORT")]
extern "C" {
    pub static mut mips_fpu_disabled: ::core::ffi::c_int;

    pub fn __cpu_has_fpu() -> ::core::ffi::c_int;
    pub fn cpu_set_fpu_opts(c: *mut cpuinfo_mips);
    pub fn cpu_set_nofpu_opts(c: *mut cpuinfo_mips);
}

#[cfg(not(feature = "CONFIG_MIPS_FP_SUPPORT"))]
pub const mips_fpu_disabled: ::core::ffi::c_int = 1;

#[cfg(not(feature = "CONFIG_MIPS_FP_SUPPORT"))]
#[inline]
pub unsafe fn cpu_get_fpu_id() -> ::core::ffi::c_ulong {
    FPIR_IMP_NONE
}

#[cfg(not(feature = "CONFIG_MIPS_FP_SUPPORT"))]
#[inline]
pub unsafe fn __cpu_has_fpu() -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_MIPS_FP_SUPPORT"))]
#[inline]
pub unsafe fn cpu_set_fpu_opts(_c: *mut cpuinfo_mips) {
    /* no-op */
}

#[cfg(not(feature = "CONFIG_MIPS_FP_SUPPORT"))]
#[inline]
pub unsafe fn cpu_set_nofpu_opts(_c: *mut cpuinfo_mips) {
    /* no-op */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
