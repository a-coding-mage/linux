/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by <asm/proc-fns.h>.
// The CONFIG_ARM64_PSEUDO_NMI branch is preserved below.

#[cfg(feature = "CONFIG_ARM64_PSEUDO_NMI")]
#[repr(C)]
pub struct arm_cpuidle_irq_context {
    pub pmr: ::core::ffi::c_ulong,
    pub daif_bits: ::core::ffi::c_ulong,
}

#[cfg(feature = "CONFIG_ARM64_PSEUDO_NMI")]
extern "C" {
    fn system_uses_irq_prio_masking() -> bool;
    fn read_sysreg_daif() -> ::core::ffi::c_ulong;
    fn write_sysreg_daif(value: ::core::ffi::c_ulong);
    fn gic_read_pmr() -> ::core::ffi::c_ulong;
    fn gic_write_pmr(value: ::core::ffi::c_ulong);
}

#[cfg(feature = "CONFIG_ARM64_PSEUDO_NMI")]
pub unsafe fn arm_cpuidle_save_irq_context(
    context: *mut arm_cpuidle_irq_context,
) {
    let c = context;
    if system_uses_irq_prio_masking() {
        (*c).daif_bits = read_sysreg_daif();
        write_sysreg_daif((*c).daif_bits | PSR_I_BIT | PSR_F_BIT);
        (*c).pmr = gic_read_pmr();
        gic_write_pmr(GIC_PRIO_IRQON | GIC_PRIO_PSR_I_SET);
    }
}

#[cfg(feature = "CONFIG_ARM64_PSEUDO_NMI")]
pub unsafe fn arm_cpuidle_restore_irq_context(
    context: *mut arm_cpuidle_irq_context,
) {
    let c = context;
    if system_uses_irq_prio_masking() {
        gic_write_pmr((*c).pmr);
        write_sysreg_daif((*c).daif_bits);
    }
}

#[cfg(not(feature = "CONFIG_ARM64_PSEUDO_NMI"))]
#[repr(C)]
pub struct arm_cpuidle_irq_context {}

#[cfg(not(feature = "CONFIG_ARM64_PSEUDO_NMI"))]
#[inline]
pub unsafe fn arm_cpuidle_save_irq_context<T>(context: *mut T) {
    let _ = context;
}

#[cfg(not(feature = "CONFIG_ARM64_PSEUDO_NMI"))]
#[inline]
pub unsafe fn arm_cpuidle_restore_irq_context<T>(context: *mut T) {
    let _ = context;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
