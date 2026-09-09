/* SPDX-License-Identifier: GPL-2.0 */

// CONFIG_CONTEXT_TRACKING_IDLE is a build-time configuration condition from
// the original header and is represented here as a Rust cfg feature.
#[cfg(feature = "CONFIG_CONTEXT_TRACKING_IDLE")]
extern "C" {
    pub fn ct_irq_enter();
    pub fn ct_irq_exit();
    pub fn ct_irq_enter_irqson();
    pub fn ct_irq_exit_irqson();
    pub fn ct_nmi_enter();
    pub fn ct_nmi_exit();
}

#[cfg(not(feature = "CONFIG_CONTEXT_TRACKING_IDLE"))]
#[inline(always)]
pub fn ct_irq_enter() {}

#[cfg(not(feature = "CONFIG_CONTEXT_TRACKING_IDLE"))]
#[inline(always)]
pub fn ct_irq_exit() {}

#[cfg(not(feature = "CONFIG_CONTEXT_TRACKING_IDLE"))]
#[inline]
pub fn ct_irq_enter_irqson() {}

#[cfg(not(feature = "CONFIG_CONTEXT_TRACKING_IDLE"))]
#[inline]
pub fn ct_irq_exit_irqson() {}

#[cfg(not(feature = "CONFIG_CONTEXT_TRACKING_IDLE"))]
#[inline(always)]
pub fn ct_nmi_enter() {}

#[cfg(not(feature = "CONFIG_CONTEXT_TRACKING_IDLE"))]
#[inline(always)]
pub fn ct_nmi_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
