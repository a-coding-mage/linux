/* SPDX-License-Identifier: GPL-2.0 */

pub const SCU_PM_NORMAL: i32 = 0;
pub const SCU_PM_DORMANT: i32 = 2;
pub const SCU_PM_POWEROFF: i32 = 3;

/* C dependencies: linux/errno.h and asm/cputype.h. */
extern "C" {
    fn read_cpuid_part() -> u32;
}

/* ARM_CPU_PART_CORTEX_A9, supplied by asm/cputype.h. */
const ARM_CPU_PART_CORTEX_A9: u32 = 0xC09;

#[inline]
pub fn scu_a9_has_base() -> bool {
    unsafe { read_cpuid_part() == ARM_CPU_PART_CORTEX_A9 }
}

#[inline]
pub unsafe fn scu_a9_get_base() -> libc::c_ulong {
    let mut pa: libc::c_ulong;

    core::arch::asm!(
        "mrc p15, 4, {0}, c15, c0, 0",
        out(reg) pa,
    );

    pa
}

/* __iomem is a kernel address-space annotation; represented here as a raw pointer. */
pub type Iomem = *mut core::ffi::c_void;

/* CONFIG_HAVE_ARM_SCU selects these external declarations in the C header. */
#[cfg(feature = "CONFIG_HAVE_ARM_SCU")]
extern "C" {
    pub fn scu_get_core_count(scu_base: Iomem) -> u32;
    pub fn scu_power_mode(scu_base: Iomem, mode: u32) -> i32;
    pub fn scu_cpu_power_enable(scu_base: Iomem, mode: u32) -> i32;
    pub fn scu_get_cpu_power_mode(scu_base: Iomem, logical_cpu: u32) -> i32;
}

/* Fallbacks when CONFIG_HAVE_ARM_SCU is not enabled. -EINVAL comes from linux/errno.h. */
#[cfg(not(feature = "CONFIG_HAVE_ARM_SCU"))]
#[inline]
pub fn scu_get_core_count(_scu_base: Iomem) -> u32 {
    0
}

#[cfg(not(feature = "CONFIG_HAVE_ARM_SCU"))]
#[inline]
pub fn scu_power_mode(_scu_base: Iomem, _mode: u32) -> i32 {
    -(libc::EINVAL as i32)
}

#[cfg(not(feature = "CONFIG_HAVE_ARM_SCU"))]
#[inline]
pub fn scu_cpu_power_enable(_scu_base: Iomem, _mode: u32) -> i32 {
    -(libc::EINVAL as i32)
}

#[cfg(not(feature = "CONFIG_HAVE_ARM_SCU"))]
#[inline]
pub fn scu_get_cpu_power_mode(_scu_base: Iomem, _logical_cpu: u32) -> i32 {
    -(libc::EINVAL as i32)
}

/* CONFIG_SMP && CONFIG_HAVE_ARM_SCU selects the external declaration in C. */
#[cfg(all(feature = "CONFIG_SMP", feature = "CONFIG_HAVE_ARM_SCU"))]
extern "C" {
    pub fn scu_enable(scu_base: Iomem);
}

#[cfg(not(all(feature = "CONFIG_SMP", feature = "CONFIG_HAVE_ARM_SCU")))]
#[inline]
pub fn scu_enable(_scu_base: Iomem) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
