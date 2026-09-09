/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Performance event support - hardware-specific disambiguation
 *
 * For now this is a compile-time decision, but eventually it should be
 * runtime.  This would allow multiplatform perf event support for e300 (fsl
 * embedded perf counters) plus server/classic, and would accommodate
 * devices other than the core which provide their own performance counters.
 *
 * Copyright 2010 Freescale Semiconductor, Inc.
 */

/* CONFIG_PPC_PERF_CTRS selects the declarations from asm/perf_event_server.h. */
#[cfg(not(feature = "CONFIG_PPC_PERF_CTRS"))]
#[inline]
pub fn is_sier_available() -> bool {
    false
}

#[cfg(not(feature = "CONFIG_PPC_PERF_CTRS"))]
#[inline]
pub fn get_pmcs_ext_regs(_idx: i32) -> libc::c_ulong {
    0
}

/* CONFIG_FSL_EMB_PERF_EVENT selects declarations from asm/perf_event_fsl_emb.h. */

/* CONFIG_PERF_EVENTS selects the perf event support declarations below. */
#[cfg(feature = "CONFIG_PERF_EVENTS")]
macro_rules! perf_arch_bpf_user_pt_regs {
    ($regs:expr) => {
        &mut $regs.user_regs
    };
}

/*
 * Overload regs->result to specify whether we should use the MSR (result
 * is zero) or the SIAR (result is non zero).
 */
#[cfg(feature = "CONFIG_PERF_EVENTS")]
macro_rules! perf_arch_fetch_caller_regs {
    ($regs:expr, $ip:expr) => {{
        $regs.result = 0;
        $regs.nip = $ip;
        $regs.gpr[1] = current_stack_frame();
        unsafe {
            core::arch::asm!("mfmsr {0}", out(reg) $regs.msr);
        }
    }};
}

/* To support perf_regs sier update */
#[cfg(feature = "CONFIG_PERF_EVENTS")]
unsafe extern "C" {
    pub fn is_sier_available() -> bool;
    pub fn get_pmcs_ext_regs(idx: i32) -> libc::c_ulong;
}

/* To define perf extended regs mask value */
#[cfg(feature = "CONFIG_PERF_EVENTS")]
unsafe extern "C" {
    pub static mut PERF_REG_EXTENDED_MASK: u64;
}

#[cfg(feature = "CONFIG_PERF_EVENTS")]
pub const PERF_REG_EXTENDED_MASK: u64 = unsafe { PERF_REG_EXTENDED_MASK };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
