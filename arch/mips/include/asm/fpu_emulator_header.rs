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

#[cfg(CONFIG_DEBUG_FS)]
#[repr(C)]
pub struct mips_fpu_emulator_stats {
    pub emulated: ::kernel::ffi::c_ulong,
    pub loads: ::kernel::ffi::c_ulong,
    pub stores: ::kernel::ffi::c_ulong,
    pub branches: ::kernel::ffi::c_ulong,
    pub cp1ops: ::kernel::ffi::c_ulong,
    pub cp1xops: ::kernel::ffi::c_ulong,
    pub errors: ::kernel::ffi::c_ulong,
    pub ieee754_inexact: ::kernel::ffi::c_ulong,
    pub ieee754_underflow: ::kernel::ffi::c_ulong,
    pub ieee754_overflow: ::kernel::ffi::c_ulong,
    pub ieee754_zerodiv: ::kernel::ffi::c_ulong,
    pub ieee754_invalidop: ::kernel::ffi::c_ulong,
    pub ds_emul: ::kernel::ffi::c_ulong,
    pub abs_s: ::kernel::ffi::c_ulong, pub abs_d: ::kernel::ffi::c_ulong,
    pub add_s: ::kernel::ffi::c_ulong, pub add_d: ::kernel::ffi::c_ulong,
    pub bc1eqz: ::kernel::ffi::c_ulong, pub bc1nez: ::kernel::ffi::c_ulong,
    pub ceil_w_s: ::kernel::ffi::c_ulong, pub ceil_w_d: ::kernel::ffi::c_ulong,
    pub ceil_l_s: ::kernel::ffi::c_ulong, pub ceil_l_d: ::kernel::ffi::c_ulong,
    pub class_s: ::kernel::ffi::c_ulong, pub class_d: ::kernel::ffi::c_ulong,
    pub cmp_af_s: ::kernel::ffi::c_ulong, pub cmp_af_d: ::kernel::ffi::c_ulong,
    pub cmp_eq_s: ::kernel::ffi::c_ulong, pub cmp_eq_d: ::kernel::ffi::c_ulong,
    pub cmp_le_s: ::kernel::ffi::c_ulong, pub cmp_le_d: ::kernel::ffi::c_ulong,
    pub cmp_lt_s: ::kernel::ffi::c_ulong, pub cmp_lt_d: ::kernel::ffi::c_ulong,
    pub cmp_ne_s: ::kernel::ffi::c_ulong, pub cmp_ne_d: ::kernel::ffi::c_ulong,
    pub cmp_or_s: ::kernel::ffi::c_ulong, pub cmp_or_d: ::kernel::ffi::c_ulong,
    pub cmp_ueq_s: ::kernel::ffi::c_ulong, pub cmp_ueq_d: ::kernel::ffi::c_ulong,
    pub cmp_ule_s: ::kernel::ffi::c_ulong, pub cmp_ule_d: ::kernel::ffi::c_ulong,
    pub cmp_ult_s: ::kernel::ffi::c_ulong, pub cmp_ult_d: ::kernel::ffi::c_ulong,
    pub cmp_un_s: ::kernel::ffi::c_ulong, pub cmp_un_d: ::kernel::ffi::c_ulong,
    pub cmp_une_s: ::kernel::ffi::c_ulong, pub cmp_une_d: ::kernel::ffi::c_ulong,
    pub cmp_saf_s: ::kernel::ffi::c_ulong, pub cmp_saf_d: ::kernel::ffi::c_ulong,
    pub cmp_seq_s: ::kernel::ffi::c_ulong, pub cmp_seq_d: ::kernel::ffi::c_ulong,
    pub cmp_sle_s: ::kernel::ffi::c_ulong, pub cmp_sle_d: ::kernel::ffi::c_ulong,
    pub cmp_slt_s: ::kernel::ffi::c_ulong, pub cmp_slt_d: ::kernel::ffi::c_ulong,
    pub cmp_sne_s: ::kernel::ffi::c_ulong, pub cmp_sne_d: ::kernel::ffi::c_ulong,
    pub cmp_sor_s: ::kernel::ffi::c_ulong, pub cmp_sor_d: ::kernel::ffi::c_ulong,
    pub cmp_sueq_s: ::kernel::ffi::c_ulong, pub cmp_sueq_d: ::kernel::ffi::c_ulong,
    pub cmp_sule_s: ::kernel::ffi::c_ulong, pub cmp_sule_d: ::kernel::ffi::c_ulong,
    pub cmp_sult_s: ::kernel::ffi::c_ulong, pub cmp_sult_d: ::kernel::ffi::c_ulong,
    pub cmp_sun_s: ::kernel::ffi::c_ulong, pub cmp_sun_d: ::kernel::ffi::c_ulong,
    pub cmp_sune_s: ::kernel::ffi::c_ulong, pub cmp_sune_d: ::kernel::ffi::c_ulong,
    pub cvt_d_l: ::kernel::ffi::c_ulong, pub cvt_d_s: ::kernel::ffi::c_ulong,
    pub cvt_d_w: ::kernel::ffi::c_ulong, pub cvt_l_s: ::kernel::ffi::c_ulong,
    pub cvt_l_d: ::kernel::ffi::c_ulong, pub cvt_s_d: ::kernel::ffi::c_ulong,
    pub cvt_s_l: ::kernel::ffi::c_ulong, pub cvt_s_w: ::kernel::ffi::c_ulong,
    pub cvt_w_s: ::kernel::ffi::c_ulong, pub cvt_w_d: ::kernel::ffi::c_ulong,
    pub div_s: ::kernel::ffi::c_ulong, pub div_d: ::kernel::ffi::c_ulong,
    pub floor_w_s: ::kernel::ffi::c_ulong, pub floor_w_d: ::kernel::ffi::c_ulong,
    pub floor_l_s: ::kernel::ffi::c_ulong, pub floor_l_d: ::kernel::ffi::c_ulong,
    pub maddf_s: ::kernel::ffi::c_ulong, pub maddf_d: ::kernel::ffi::c_ulong,
    pub max_s: ::kernel::ffi::c_ulong, pub max_d: ::kernel::ffi::c_ulong,
    pub maxa_s: ::kernel::ffi::c_ulong, pub maxa_d: ::kernel::ffi::c_ulong,
    pub min_s: ::kernel::ffi::c_ulong, pub min_d: ::kernel::ffi::c_ulong,
    pub mina_s: ::kernel::ffi::c_ulong, pub mina_d: ::kernel::ffi::c_ulong,
    pub mov_s: ::kernel::ffi::c_ulong, pub mov_d: ::kernel::ffi::c_ulong,
    pub msubf_s: ::kernel::ffi::c_ulong, pub msubf_d: ::kernel::ffi::c_ulong,
    pub mul_s: ::kernel::ffi::c_ulong, pub mul_d: ::kernel::ffi::c_ulong,
    pub neg_s: ::kernel::ffi::c_ulong, pub neg_d: ::kernel::ffi::c_ulong,
    pub recip_s: ::kernel::ffi::c_ulong, pub recip_d: ::kernel::ffi::c_ulong,
    pub rint_s: ::kernel::ffi::c_ulong, pub rint_d: ::kernel::ffi::c_ulong,
    pub round_w_s: ::kernel::ffi::c_ulong, pub round_w_d: ::kernel::ffi::c_ulong,
    pub round_l_s: ::kernel::ffi::c_ulong, pub round_l_d: ::kernel::ffi::c_ulong,
    pub rsqrt_s: ::kernel::ffi::c_ulong, pub rsqrt_d: ::kernel::ffi::c_ulong,
    pub sel_s: ::kernel::ffi::c_ulong, pub sel_d: ::kernel::ffi::c_ulong,
    pub seleqz_s: ::kernel::ffi::c_ulong, pub seleqz_d: ::kernel::ffi::c_ulong,
    pub selnez_s: ::kernel::ffi::c_ulong, pub selnez_d: ::kernel::ffi::c_ulong,
    pub sqrt_s: ::kernel::ffi::c_ulong, pub sqrt_d: ::kernel::ffi::c_ulong,
    pub sub_s: ::kernel::ffi::c_ulong, pub sub_d: ::kernel::ffi::c_ulong,
    pub trunc_w_s: ::kernel::ffi::c_ulong, pub trunc_w_d: ::kernel::ffi::c_ulong,
    pub trunc_l_s: ::kernel::ffi::c_ulong, pub trunc_l_d: ::kernel::ffi::c_ulong,
}

#[cfg(CONFIG_DEBUG_FS)]
extern "C" {
    pub static mut fpuemustats: mips_fpu_emulator_stats;
}

#[cfg(CONFIG_DEBUG_FS)]
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

#[cfg(not(CONFIG_DEBUG_FS))]
#[macro_export]
macro_rules! MIPS_FPU_EMU_INC_STATS {
    ($m:ident) => {{}};
}

extern "C" {
    pub fn fpu_emulator_cop1Handler(
        xcp: *mut pt_regs,
        ctx: *mut mips_fpu_struct,
        has_fpu: ::kernel::ffi::c_int,
        fault_addr: *mut *mut ::kernel::ffi::c_void,
    ) -> ::kernel::ffi::c_int;
    pub fn force_fcr31_sig(
        fcr31: ::kernel::ffi::c_ulong,
        fault_addr: *mut ::kernel::ffi::c_void,
        tsk: *mut task_struct,
    );
    pub fn process_fpemu_return(
        sig: ::kernel::ffi::c_int,
        fault_addr: *mut ::kernel::ffi::c_void,
        fcr31: ::kernel::ffi::c_ulong,
    ) -> ::kernel::ffi::c_int;
}

/* Mask the FCSR Cause bits according to the Enable bits; Unimplemented is always enabled. */
#[inline]
pub unsafe fn mask_fcr31_x(fcr31: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong {
    fcr31 & (FPU_CSR_UNI_X
        | ((fcr31 & FPU_CSR_ALL_E) << (ffs(FPU_CSR_ALL_X) - ffs(FPU_CSR_ALL_E))))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
