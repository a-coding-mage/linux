/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency supplied by the Linux IRQ-domain headers.

pub const GPIO_NO_WAKE_IRQ: u32 = !0u32;

/*
 * QCOM specific IRQ domain flags that distinguishes the handling of wakeup
 * capable interrupts by different interrupt controllers.
 *
 * IRQ_DOMAIN_FLAG_QCOM_PDC_WAKEUP: Line must be masked at TLMM and the
 *                                  interrupt configuration is done at PDC
 * IRQ_DOMAIN_FLAG_QCOM_MPM_WAKEUP: Interrupt configuration is handled at TLMM
 */
pub const IRQ_DOMAIN_FLAG_QCOM_PDC_WAKEUP: usize = IRQ_DOMAIN_FLAG_NONCORE << 0;
pub const IRQ_DOMAIN_FLAG_QCOM_MPM_WAKEUP: usize = IRQ_DOMAIN_FLAG_NONCORE << 1;

/**
 * irq_domain_qcom_handle_wakeup: Return if the domain handles interrupt
 *                                configuration
 * @d: irq domain
 *
 * This QCOM specific irq domain call returns if the interrupt controller
 * requires the interrupt be masked at the child interrupt controller.
 */
#[inline]
pub unsafe fn irq_domain_qcom_handle_wakeup(d: *const irq_domain) -> bool {
    ((*d).flags & IRQ_DOMAIN_FLAG_QCOM_PDC_WAKEUP) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
