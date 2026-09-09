/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2017 Linaro Ltd. <ard.biesheuvel@linaro.org>
 * Copyright (C) 2023 SiFive
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/compiler.h, linux/irqflags.h, linux/percpu.h, linux/preempt.h,
// linux/types.h, linux/thread_info.h, and asm/vector.h.

#[cfg(feature = "CONFIG_RISCV_ISA_V")]
extern "C" {
    fn in_hardirq() -> bool;
    fn in_nmi() -> bool;
    fn riscv_v_flags() -> u64;
    static RISCV_KERNEL_MODE_V: u64;
}

#[cfg(feature = "CONFIG_RISCV_ISA_V")]
#[inline]
pub unsafe fn may_use_simd() -> bool {
    /*
     * RISCV_KERNEL_MODE_V is only set while preemption is disabled,
     * and is clear whenever preemption is enabled.
     */
    if in_hardirq() || in_nmi() {
        return false;
    }

    /*
     * Nesting is achieved in preempt_v by spreading the control for
     * preemptible and non-preemptible kernel-mode Vector into two fields.
     * Only non-preempt_v can nest on top of preempt_v, if non-preempt_v is
     * unavailable, then preempt_v is not allowed.
     */
    (riscv_v_flags() & RISCV_KERNEL_MODE_V) == 0
}

#[cfg(not(feature = "CONFIG_RISCV_ISA_V"))]
#[inline]
pub fn may_use_simd() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
