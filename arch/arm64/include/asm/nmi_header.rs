/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation:
// use crate::linux::cpumask::cpumask_t;

pub struct pt_regs;

/*
 * Cross-CPU NMI provider hooks, consulted by the arm64 arch code before
 * its regular-IRQ / pseudo-NMI IPI paths. The SDEI provider in
 * drivers/firmware/arm_sdei_nmi.c implements them when active; a future
 * FEAT_NMI provider could slot in here too. The stubs let callers stay
 * unconditional when ARM_SDEI_NMI is off.
 *
 * sdei_nmi_active() lets a caller test for the service before committing
 * to (and waiting on) the SDEI stop rung; sdei_nmi_stop_cpus() then signals
 * the targets, which ack by going offline.
 */
// `CONFIG_ARM_SDEI_NMI` is a build-time configuration condition preserved
// from the C header.
#[cfg(CONFIG_ARM_SDEI_NMI)]
extern "C" {
    pub fn sdei_nmi_trigger_cpumask_backtrace(
        mask: *const cpumask_t,
        exclude_cpu: core::ffi::c_int,
    ) -> bool;
    pub fn sdei_nmi_active() -> bool;
    pub fn sdei_nmi_stop_cpus(mask: *const cpumask_t);
}

#[cfg(not(CONFIG_ARM_SDEI_NMI))]
#[inline]
pub unsafe fn sdei_nmi_trigger_cpumask_backtrace(
    _mask: *const cpumask_t,
    _exclude_cpu: core::ffi::c_int,
) -> bool {
    false
}

#[cfg(not(CONFIG_ARM_SDEI_NMI))]
#[inline]
pub unsafe fn sdei_nmi_active() -> bool {
    false
}

#[cfg(not(CONFIG_ARM_SDEI_NMI))]
#[inline]
pub unsafe fn sdei_nmi_stop_cpus(_mask: *const cpumask_t) {}

/*
 * The common "stop this CPU" entry every arm64 stop path funnels through:
 * the regular/pseudo-NMI stop IPI handlers, panic_smp_self_stop(), and the
 * SDEI cross-CPU NMI handler. @die_on_crash powers the CPU off on the kdump
 * crash path (IPI handlers) instead of parking it (SDEI / self-stop).
 * Defined in arch/arm64/kernel/smp.c.
 */
extern "C" {
    pub fn arm64_nmi_cpu_stop(
        regs: *mut pt_regs,
        die_on_crash: bool,
    ) -> !;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
