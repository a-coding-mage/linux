/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Based on arch/arm/include/asm/traps.h
 *
 * Copyright (C) 2012 ARM Ltd.
 */

// C dependencies: linux/list.h, asm/esr.h, asm/ptrace.h, asm/sections.h

#[cfg(CONFIG_ARMV8_DEPRECATED)]
extern "C" {
    static __irqentry_text_start: u8;
    static __irqentry_text_end: u8;
    static __entry_text_start: u8;
    static __entry_text_end: u8;
    pub fn try_emulate_armv8_deprecated(regs: *mut pt_regs, insn: u32) -> bool;
}

#[cfg(not(CONFIG_ARMV8_DEPRECATED))]
#[inline]
pub unsafe fn try_emulate_armv8_deprecated(_regs: *mut pt_regs, _insn: u32) -> bool {
    false
}

extern "C" {
    pub fn force_signal_inject(signal: core::ffi::c_int, code: core::ffi::c_int, address: libc::c_ulong, err: libc::c_ulong);
    pub fn arm64_notify_segfault(addr: libc::c_ulong);
    pub fn arm64_force_sig_fault(signo: core::ffi::c_int, code: core::ffi::c_int, far: libc::c_ulong, string: *const core::ffi::c_char);
    pub fn arm64_force_sig_fault_pkey(far: libc::c_ulong, string: *const core::ffi::c_char, pkey: core::ffi::c_int);
    pub fn arm64_force_sig_mceerr(code: core::ffi::c_int, far: libc::c_ulong, lsb: i16, string: *const core::ffi::c_char);
    pub fn arm64_force_sig_ptrace_errno_trap(errno: core::ffi::c_int, far: libc::c_ulong, string: *const core::ffi::c_char);

    pub fn bug_brk_handler(regs: *mut pt_regs, esr: libc::c_ulong) -> core::ffi::c_int;
    pub fn cfi_brk_handler(regs: *mut pt_regs, esr: libc::c_ulong) -> core::ffi::c_int;
    pub fn reserved_fault_brk_handler(regs: *mut pt_regs, esr: libc::c_ulong) -> core::ffi::c_int;
    pub fn kasan_brk_handler(regs: *mut pt_regs, esr: libc::c_ulong) -> core::ffi::c_int;
    pub fn ubsan_brk_handler(regs: *mut pt_regs, esr: libc::c_ulong) -> core::ffi::c_int;

    pub fn early_brk64(addr: libc::c_ulong, esr: libc::c_ulong, regs: *mut pt_regs) -> core::ffi::c_int;
    pub fn dump_kernel_instr(kaddr: libc::c_ulong);
    pub fn arm64_skip_faulting_instruction(regs: *mut pt_regs, size: libc::c_ulong);
    pub fn arm64_is_fatal_ras_serror(regs: *mut pt_regs, esr: libc::c_ulong) -> bool;
    pub fn arm64_serror_panic(regs: *mut pt_regs, esr: libc::c_ulong) -> !;
}

#[inline]
pub unsafe fn __in_irqentry_text(ptr: libc::c_ulong) -> core::ffi::c_int {
    (ptr >= (&__irqentry_text_start as *const _ as libc::c_ulong)
        && ptr < (&__irqentry_text_end as *const _ as libc::c_ulong)) as core::ffi::c_int
}

#[inline]
pub unsafe fn in_entry_text(ptr: libc::c_ulong) -> core::ffi::c_int {
    (ptr >= (&__entry_text_start as *const _ as libc::c_ulong)
        && ptr < (&__entry_text_end as *const _ as libc::c_ulong)) as core::ffi::c_int
}

#[inline]
pub unsafe fn arm64_is_ras_serror(esr: libc::c_ulong) -> bool {
    // CPUs with RAS use the implementation-defined syndrome bit; CPUs
    // without it use the ISS-valid bit in the same position.
    WARN_ON(preemptible());
    if esr & ESR_ELx_IDS != 0 {
        return false;
    }
    if this_cpu_has_cap(ARM64_HAS_RAS_EXTN) {
        true
    } else {
        false
    }
}

#[inline]
pub unsafe fn arm64_ras_serror_get_severity(esr: libc::c_ulong) -> libc::c_ulong {
    // Uncategorized errors are treated as uncontainable.
    let aet = esr & ESR_ELx_AET;
    if !arm64_is_ras_serror(esr) {
        return ESR_ELx_AET_UC;
    }
    if (esr & ESR_ELx_FSC) != ESR_ELx_FSC_SERROR {
        return ESR_ELx_AET_UC;
    }
    aet
}

#[inline]
pub unsafe fn arm64_mops_reset_regs(regs: *mut user_pt_regs, esr: libc::c_ulong) {
    let wrong_option = esr & ESR_ELx_MOPS_ISS_WRONG_OPTION != 0;
    let option_a = esr & ESR_ELx_MOPS_ISS_OPTION_A != 0;
    let dstreg = ESR_ELx_MOPS_ISS_DESTREG(esr) as usize;
    let srcreg = ESR_ELx_MOPS_ISS_SRCREG(esr) as usize;
    let sizereg = ESR_ELx_MOPS_ISS_SIZEREG(esr) as usize;
    let (dst, size);
    dst = (*regs).regs[dstreg];
    size = (*regs).regs[sizereg];

    // Restore the registers to the original format suitable for a prologue
    // instruction, following the generic return rules from the Arm ARM.
    if esr & ESR_ELx_MOPS_ISS_MEM_INST != 0 {
        if option_a ^ wrong_option {
            (*regs).regs[dstreg] = dst.wrapping_add(size);
            (*regs).regs[sizereg] = size.wrapping_neg();
        }
    } else {
        let src = (*regs).regs[srcreg];
        if !(option_a ^ wrong_option) {
            if (*regs).pstate & PSR_N_BIT != 0 {
                (*regs).regs[dstreg] = dst.wrapping_sub(size);
                (*regs).regs[srcreg] = src.wrapping_sub(size);
            }
        } else if size & BIT(63) != 0 {
            (*regs).regs[dstreg] = dst.wrapping_add(size);
            (*regs).regs[srcreg] = src.wrapping_add(size);
            (*regs).regs[sizereg] = size.wrapping_neg();
        }
    }

    if esr & ESR_ELx_MOPS_ISS_FROM_EPILOGUE != 0 {
        (*regs).pc = (*regs).pc.wrapping_sub(8);
    } else {
        (*regs).pc = (*regs).pc.wrapping_sub(4);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
