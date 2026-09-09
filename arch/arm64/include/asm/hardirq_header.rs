/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 ARM Ltd.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external. The original header includes cache, percpu, threads, barrier,
// irq, KVM, and system-register interfaces.

pub const __ARCH_IRQ_EXIT_IRQS_DISABLED: i32 = 1;

#[repr(C)]
pub struct nmi_ctx {
    pub hcr: u64,
    pub cnt: u32,
}

extern "C" {
    pub static mut nmi_contexts: nmi_ctx;
    pub static mut irq_err_count: usize;

    fn is_kernel_in_hyp_mode() -> bool;
    fn this_cpu_ptr(ctx: *mut nmi_ctx) -> *mut nmi_ctx;
    fn read_sysreg_hcr_el2() -> u64;
    fn write_sysreg_hcr(value: u64);
    fn isb();
    fn barrier();
}

// The C macro is self-referential to ensure the generic declaration is kept.
// The generic hardirq declarations are supplied by the surrounding translation.
pub const ack_bad_irq: &str = "ack_bad_irq";

pub const HCR_TGE: u64 = 1 << 27;

/// Translation of `arch_nmi_enter()`.
#[inline]
pub unsafe fn arch_nmi_enter() {
    if !is_kernel_in_hyp_mode() {
        return;
    }

    let ctx = this_cpu_ptr(&raw mut nmi_contexts);
    if (*ctx).cnt != 0 {
        (*ctx).cnt = (*ctx).cnt.wrapping_add(1);
        return;
    }

    let hcr = read_sysreg_hcr_el2();
    if (hcr & HCR_TGE) == 0 {
        write_sysreg_hcr(hcr | HCR_TGE);
        isb();
    }
    /*
     * Make sure the sysreg write is performed before ctx->cnt is set to 1.
     * NMIs that see cnt == 1 will rely on us.
     */
    barrier();
    (*ctx).cnt = 1;
    /*
     * Make sure ctx->cnt is set before we save hcr. We don't want ctx->hcr
     * to be overwritten.
     */
    barrier();
    (*ctx).hcr = hcr;
}

/// Translation of `arch_nmi_exit()`.
#[inline]
pub unsafe fn arch_nmi_exit() {
    if !is_kernel_in_hyp_mode() {
        return;
    }

    let ctx = this_cpu_ptr(&raw mut nmi_contexts);
    let hcr = (*ctx).hcr;
    /*
     * Make sure we read ctx->hcr before we release ctx->cnt as it makes
     * ctx->hcr updatable again.
     */
    barrier();
    (*ctx).cnt = (*ctx).cnt.wrapping_sub(1);
    /*
     * Make sure ctx->cnt release is visible before we restore the sysreg.
     * Otherwise a new NMI occurring right after write_sysreg() can be fooled
     * and think we secured things for it.
     */
    barrier();
    if (*ctx).cnt == 0 && (hcr & HCR_TGE) == 0 {
        write_sysreg_hcr(hcr);
    }
}

#[inline]
pub unsafe fn ack_bad_irq(irq: u32) {
    let _ = irq;
    irq_err_count = irq_err_count.wrapping_add(1);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
