/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2010-2011 Freescale Semiconductor, Inc.
 */

// Translated from the assembler-only portion of kvm_booke_hv_asm.h.
// The original dependency <asm/feature-fixups.h> supplies the feature-fixup
// section machinery used by DO_KVM.

/*
 * All exceptions from guest state must go through KVM
 * (except for those which are delivered directly to the guest) --
 * there are no exceptions for which we fall through directly to the
 * normal host handler.
 *
 * 32-bit host
 * Expected inputs (normal exceptions):
 *   SCRATCH0 = saved r10
 *   r10 = thread struct
 *   r11 = appropriate SRR1 variant (currently used as scratch)
 *   r13 = saved CR
 *   *(r10 + THREAD_NORMSAVE(0)) = saved r11
 *   *(r10 + THREAD_NORMSAVE(2)) = saved r13
 *
 * Expected inputs (crit/mcheck/debug exceptions):
 *   appropriate SCRATCH = saved r8
 *   r8 = exception level stack frame
 *   r9 = *(r8 + _CCR) = saved CR
 *   r11 = appropriate SRR1 variant (currently used as scratch)
 *   *(r8 + GPR9) = saved r9
 *   *(r8 + GPR10) = saved r10 (r10 not yet clobbered)
 *   *(r8 + GPR11) = saved r11
 *
 * 64-bit host
 * Expected inputs (GEN/GDBELL/DBG/CRIT/MC exception types):
 *   r10 = saved CR
 *   r13 = PACA_POINTER
 *   *(r13 + PACA_EX##type + EX_R10) = saved r10
 *   *(r13 + PACA_EX##type + EX_R11) = saved r11
 *   SPRN_SPRG_##type##_SCRATCH = saved r13
 *
 * Expected inputs (TLB exception type):
 *   r10 = saved CR
 *   r12 = extlb pointer
 *   r13 = PACA_POINTER
 *   *(r12 + EX_TLB_R10) = saved r10
 *   *(r12 + EX_TLB_R11) = saved r11
 *   *(r12 + EX_TLB_R13) = saved r13
 *   SPRN_SPRG_GEN_SCRATCH = saved r12
 *
 * Only the bolted version of TLB miss exception handlers is supported now.
 */

// C preprocessor configuration and assembler feature-fixup sections are
// preserved here as conditional intent. The macro expands to the same
// PowerPC instructions when CONFIG_KVM_BOOKE_HV and CPU_FTR_EMB_HV apply.
#[macro_export]
macro_rules! DO_KVM {
    ($intno:tt, $srr1:tt) => {
        #[cfg(feature = "CONFIG_KVM_BOOKE_HV")]
        unsafe {
            core::arch::asm!(
                "mtocrf 0x80, r11",
                "bf 3, 1975f",
                concat!("b kvmppc_handler_", stringify!($intno), "_", stringify!($srr1)),
                "1975:",
                options(nostack)
            );
        }
    };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
