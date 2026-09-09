/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * Further private data for which no space exists in mips_fpu_struct.
 * This should be subsumed into the mips_fpu_struct structure as
 * defined in processor.h as soon as the absurd wired absolute assembler
 * offsets become dynamic at compile time.
 *
 * Kevin D. Kissell, kevink@mips.com and Carsten Langgaard, carstenl@mips.com
 * Copyright (C) 2000 MIPS Technologies, Inc.  All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external, as in the original header.

#[cfg(feature = "CONFIG_DEBUG_FS")]
#[repr(C)]
pub struct mips_fpu_emulator_stats {
    pub emulated: ::core::ffi::c_ulong,
    pub loads: ::core::ffi::c_ulong,
    pub stores: ::core::ffi::c_ulong,
    pub branches: ::core::ffi::c_ulong,
    pub cp1ops: ::core::ffi::c_ulong,
    pub cp1xops: ::core::ffi::c_ulong,
    pub errors: ::core::ffi::c_ulong,
    pub ieee754_inexact: ::core::ffi::c_ulong,
    pub ieee754_underflow: ::core::ffi::c_ulong,
    pub ieee754_overflow: ::core::ffi::c_ulong,
    pub ieee754_zerodiv: ::core::ffi::c_ulong,
    pub ieee754_invalidop: ::core::ffi::c_ulong,
    pub ds_emul: ::core::ffi::c_ulong,
    pub abs_s: ::core::ffi::c_ulong, pub abs_d: ::core::ffi::c_ulong,
    pub add_s: ::core::ffi::c_ulong, pub add_d: ::core::ffi::c_ulong,
    pub bc1eqz: ::core::ffi::c_ulong, pub bc1nez: ::core::ffi::c_ulong,
    pub ceil_w_s: ::core::ffi::c_ulong, pub ceil_w_d: ::core::ffi::c_ulong,
    pub ceil_l_s: ::core::ffi::c_ulong, pub ceil_l_d: ::core::ffi::c_ulong,
    pub class_s: ::core::ffi::c_ulong, pub class_d: ::core::ffi::c_ulong,
    pub cmp_af_s: ::core::ffi::c_ulong, pub cmp_af_d: ::core::ffi::c_ulong,
    pub cmp_eq_s: ::core::ffi::c_ulong, pub cmp_eq_d: ::core::ffi::c_ulong,
    pub cmp_le_s: ::core::ffi::c_ulong, pub cmp_le_d: ::core::ffi::c_ulong,
    pub cmp_lt_s: ::core::ffi::c_ulong, pub cmp_lt_d: ::core::ffi::c_ulong,
    pub cmp_ne_s: ::core::ffi::c_ulong, pub cmp_ne_d: ::core::ffi::c_ulong,
    pub cmp_or_s: ::core::ffi::c_ulong, pub cmp_or_d: ::core::ffi::c_ulong,
    pub cmp_ueq_s: ::core::ffi::c_ulong, pub cmp_ueq_d: ::core::ffi::c_ulong,
    pub cmp_ule_s: ::core::ffi::c_ulong, pub cmp_ule_d: ::core::ffi::c_ulong,
    pub cmp_ult_s: ::core::ffi::c_ulong, pub cmp_ult_d: ::core::ffi::c_ulong,
    pub cmp_un_s: ::core::ffi::c_ulong, pub cmp_un_d: ::core::ffi::c_ulong,
    pub cmp_une_s: ::core::ffi::c_ulong, pub cmp_une_d: ::core::ffi::c_ulong,
    pub cmp_saf_s: ::core::ffi::c_ulong, pub cmp_saf_d: ::core::ffi::c_ulong,
    pub cmp_seq_s: ::core::ffi::c_ulong, pub cmp_seq_d: ::core::ffi::c_ulong,
    pub cmp_sle_s: ::core::ffi::c_ulong, pub cmp_sle_d: ::core::ffi::c_ulong,
    pub cmp_slt_s: ::core::ffi::c_ulong, pub cmp_slt_d: ::core::ffi::c_ulong,
    pub cmp_sne_s: ::core::ffi::c_ulong, pub cmp_sne_d: ::core::ffi::c_ulong,
    pub cmp_sor_s: ::core::ffi::c_ulong, pub cmp_sor_d: ::core::ffi::c_ulong,
    pub cmp_sueq_s: ::core::ffi::c_ulong, pub cmp_sueq_d: ::core::ffi::c_ulong,
    pub cmp_sule_s: ::core::ffi::c_ulong, pub cmp_sule_d: ::core::ffi::c_ulong,
    pub cmp_sult_s: ::core::ffi::c_ulong, pub cmp_sult_d: ::core::ffi::c_ulong,
    pub cmp_sun_s: ::core::ffi::c_ulong, pub cmp_sun_d: ::core::ffi::c_ulong,
    pub cmp_sune_s: ::core::ffi::c_ulong, pub cmp_sune_d: ::core::ffi::c_ulong,
    pub cvt_d_l: ::core::ffi::c_ulong, pub cvt_d_s: ::core::ffi::c_ulong,
    pub cvt_d_w: ::core::ffi::c_ulong, pub cvt_l_s: ::core::ffi::c_ulong,
    pub cvt_l_d: ::core::ffi::c_ulong, pub cvt_s_d: ::core::ffi::c_ulong,
    pub cvt_s_l: ::core::ffi::c_ulong, pub cvt_s_w: ::core::ffi::c_ulong,
    pub cvt_w_s: ::core::ffi::c_ulong, pub cvt_w_d: ::core::ffi::c_ulong,
    pub div_s: ::core::ffi::c_ulong, pub div_d: ::core::ffi::c_ulong,
    pub floor_w_s: ::core::ffi::c_ulong, pub floor_w_d: ::core::ffi::c_ulong,
    pub floor_l_s: ::core::ffi::c_ulong, pub floor_l_d: ::core::ffi::c_ulong,
    pub maddf_s: ::core::ffi::c_ulong, pub maddf_d: ::core::ffi::c_ulong,
    pub max_s: ::core::ffi::c_ulong, pub max_d: ::core::ffi::c_ulong,
    pub maxa_s: ::core::ffi::c_ulong, pub maxa_d: ::core::ffi::c_ulong,
    pub min_s: ::core::ffi::c_ulong, pub min_d: ::core::ffi::c_ulong,
    pub mina_s: ::core::ffi::c_ulong, pub mina_d: ::core::ffi::c_ulong,
    pub mov_s: ::core::ffi::c_ulong, pub mov_d: ::core::ffi::c_ulong,
    pub msubf_s: ::core::ffi::c_ulong, pub msubf_d: ::core::ffi::c_ulong,
    pub mul_s: ::core::ffi::c_ulong, pub mul_d: ::core::ffi::c_ulong,
    pub neg_s: ::core::ffi::c_ulong, pub neg_d: ::core::ffi::c_ulong,
    pub recip_s: ::core::ffi::c_ulong, pub recip_d: ::core::ffi::c_ulong,
    pub rint_s: ::core::ffi::c_ulong, pub rint_d: ::core::ffi::c_ulong,
    pub round_w_s: ::core::ffi::c_ulong, pub round_w_d: ::core::ffi::c_ulong,
    pub round_l_s: ::core::ffi::c_ulong, pub round_l_d: ::core::ffi::c_ulong,
    pub rsqrt_s: ::core::ffi::c_ulong, pub rsqrt_d: ::core::ffi::c_ulong,
    pub sel_s: ::core::ffi::c_ulong, pub sel_d: ::core::ffi::c_ulong,
    pub seleqz_s: ::core::ffi::c_ulong, pub seleqz_d: ::core::ffi::c_ulong,
    pub selnez_s: ::core::ffi::c_ulong, pub selnez_d: ::core::ffi::c_ulong,
    pub sqrt_s: ::core::ffi::c_ulong, pub sqrt_d: ::core::ffi::c_ulong,
    pub sub_s: ::core::ffi::c_ulong, pub sub_d: ::core::ffi::c_ulong,
    pub trunc_w_s: ::core::ffi::c_ulong, pub trunc_w_d: ::core::ffi::c_ulong,
    pub trunc_l_s: ::core::ffi::c_ulong, pub trunc_l_d: ::core::ffi::c_ulong,
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
extern "C" {
    pub static mut fpuemustats: mips_fpu_emulator_stats;
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
#[macro_export]
macro_rules! MIPS_FPU_EMU_INC_STATS {
    ($m:ident) => {{
        unsafe {
            preempt_disable();
            ::core::ptr::addr_of_mut!(fpuemustats.$m).write(
                ::core::ptr::read_volatile(::core::ptr::addr_of!(fpuemustats.$m)).wrapping_add(1),
            );
            preempt_enable();
        }
    }};
}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[macro_export]
macro_rules! MIPS_FPU_EMU_INC_STATS {
    ($m:ident) => {{}};
}

extern "C" {
    pub fn fpu_emulator_cop1Handler(
        xcp: *mut pt_regs,
        ctx: *mut mips_fpu_struct,
        has_fpu: ::core::ffi::c_int,
        fault_addr: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    pub fn force_fcr31_sig(
        fcr31: ::core::ffi::c_ulong,
        fault_addr: *mut ::core::ffi::c_void,
        tsk: *mut task_struct,
    );
    pub fn process_fpemu_return(
        sig: ::core::ffi::c_int,
        fault_addr: *mut ::core::ffi::c_void,
        fcr31: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
}

/* Mask the FCSR Cause bits according to the Enable bits; Unimplemented is always enabled. */
#[inline]
pub unsafe fn mask_fcr31_x(fcr31: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    fcr31 & (FPU_CSR_UNI_X
        | ((fcr31 & FPU_CSR_ALL_E) << (ffs(FPU_CSR_ALL_X) - ffs(FPU_CSR_ALL_E))))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
