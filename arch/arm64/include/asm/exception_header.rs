/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Based on arch/arm/include/asm/exception.h
 *
 * Copyright (C) 2012 ARM Ltd.
 */

// Dependencies supplied by the surrounding kernel translation:
// asm/esr.h, asm/ptrace.h, and linux/interrupt.h.

// #define __exception_irq_entry __irq_entry
// The C attribute alias is preserved as intent; Rust declarations below use
// the corresponding external ABI directly.

#[inline]
pub unsafe fn disr_to_esr(disr: u64) -> ::core::ffi::c_ulong {
    let mut esr: ::core::ffi::c_ulong =
        (ESR_ELx_EC_SERROR << ESR_ELx_EC_SHIFT) as ::core::ffi::c_ulong;

    if (disr & DISR_EL1_IDS) == 0 {
        esr |= (disr & DISR_EL1_ESR_MASK) as ::core::ffi::c_ulong;
    } else {
        esr |= (disr & ESR_ELx_ISS_MASK) as ::core::ffi::c_ulong;
    }

    esr
}

extern "C" {
    pub fn handle_bad_stack(regs: *mut pt_regs) -> !;

    pub fn el1t_64_sync_handler(regs: *mut pt_regs);
    pub fn el1t_64_irq_handler(regs: *mut pt_regs);
    pub fn el1t_64_fiq_handler(regs: *mut pt_regs);
    pub fn el1t_64_error_handler(regs: *mut pt_regs);

    pub fn el1h_64_sync_handler(regs: *mut pt_regs);
    pub fn el1h_64_irq_handler(regs: *mut pt_regs);
    pub fn el1h_64_fiq_handler(regs: *mut pt_regs);
    pub fn el1h_64_error_handler(regs: *mut pt_regs);

    pub fn el0t_64_sync_handler(regs: *mut pt_regs);
    pub fn el0t_64_irq_handler(regs: *mut pt_regs);
    pub fn el0t_64_fiq_handler(regs: *mut pt_regs);
    pub fn el0t_64_error_handler(regs: *mut pt_regs);

    pub fn el0t_32_sync_handler(regs: *mut pt_regs);
    pub fn el0t_32_irq_handler(regs: *mut pt_regs);
    pub fn el0t_32_fiq_handler(regs: *mut pt_regs);
    pub fn el0t_32_error_handler(regs: *mut pt_regs);

    pub fn call_on_irq_stack(regs: *mut pt_regs, func: Option<unsafe extern "C" fn(*mut pt_regs)>);
    pub fn asm_exit_to_user_mode(regs: *mut pt_regs);

    pub fn do_mem_abort(far: ::core::ffi::c_ulong, esr: ::core::ffi::c_ulong, regs: *mut pt_regs);
    pub fn do_el0_undef(regs: *mut pt_regs, esr: ::core::ffi::c_ulong);
    pub fn do_el1_undef(regs: *mut pt_regs, esr: ::core::ffi::c_ulong);
    pub fn do_el0_bti(regs: *mut pt_regs);
    pub fn do_el1_bti(regs: *mut pt_regs, esr: ::core::ffi::c_ulong);
    pub fn do_el0_gcs(regs: *mut pt_regs, esr: ::core::ffi::c_ulong);
    pub fn do_el1_gcs(regs: *mut pt_regs, esr: ::core::ffi::c_ulong);

    // CONFIG_HAVE_HW_BREAKPOINT controls these declarations in the C header.
    #[cfg(feature = "CONFIG_HAVE_HW_BREAKPOINT")]
    pub fn do_breakpoint(esr: ::core::ffi::c_ulong, regs: *mut pt_regs);
    #[cfg(feature = "CONFIG_HAVE_HW_BREAKPOINT")]
    pub fn do_watchpoint(addr: ::core::ffi::c_ulong, esr: ::core::ffi::c_ulong, regs: *mut pt_regs);

    pub fn do_el0_softstep(esr: ::core::ffi::c_ulong, regs: *mut pt_regs);
    pub fn do_el1_softstep(esr: ::core::ffi::c_ulong, regs: *mut pt_regs);
    pub fn do_el0_brk64(esr: ::core::ffi::c_ulong, regs: *mut pt_regs);
    pub fn do_el1_brk64(esr: ::core::ffi::c_ulong, regs: *mut pt_regs);
    pub fn do_bkpt32(esr: ::core::ffi::c_ulong, regs: *mut pt_regs);
    pub fn do_fpsimd_acc(esr: ::core::ffi::c_ulong, regs: *mut pt_regs);
    pub fn do_sve_acc(esr: ::core::ffi::c_ulong, regs: *mut pt_regs);
    pub fn do_sme_acc(esr: ::core::ffi::c_ulong, regs: *mut pt_regs);
    pub fn do_fpsimd_exc(esr: ::core::ffi::c_ulong, regs: *mut pt_regs);
    pub fn do_el0_sys(esr: ::core::ffi::c_ulong, regs: *mut pt_regs);
    pub fn do_sp_pc_abort(addr: ::core::ffi::c_ulong, esr: ::core::ffi::c_ulong, regs: *mut pt_regs);
    pub fn bad_el0_sync(regs: *mut pt_regs, reason: ::core::ffi::c_int, esr: ::core::ffi::c_ulong);
    pub fn do_el0_cp15(esr: ::core::ffi::c_ulong, regs: *mut pt_regs);
    pub fn do_compat_alignment_fixup(addr: ::core::ffi::c_ulong, regs: *mut pt_regs) -> ::core::ffi::c_int;
    pub fn do_el0_svc(regs: *mut pt_regs);
    pub fn do_el0_svc_compat(regs: *mut pt_regs);
    pub fn do_el0_fpac(regs: *mut pt_regs, esr: ::core::ffi::c_ulong);
    pub fn do_el1_fpac(regs: *mut pt_regs, esr: ::core::ffi::c_ulong);
    pub fn do_el0_mops(regs: *mut pt_regs, esr: ::core::ffi::c_ulong);
    pub fn do_el1_mops(regs: *mut pt_regs, esr: ::core::ffi::c_ulong);
    pub fn do_serror(regs: *mut pt_regs, esr: ::core::ffi::c_ulong);
    pub fn panic_bad_stack(regs: *mut pt_regs, esr: ::core::ffi::c_ulong, far: ::core::ffi::c_ulong) -> !;
}

#[cfg(not(feature = "CONFIG_HAVE_HW_BREAKPOINT"))]
#[inline]
pub unsafe fn do_breakpoint(_esr: ::core::ffi::c_ulong, _regs: *mut pt_regs) {}

#[cfg(not(feature = "CONFIG_HAVE_HW_BREAKPOINT"))]
#[inline]
pub unsafe fn do_watchpoint(
    _addr: ::core::ffi::c_ulong,
    _esr: ::core::ffi::c_ulong,
    _regs: *mut pt_regs,
) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
