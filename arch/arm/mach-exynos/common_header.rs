/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2011 Samsung Electronics Co., Ltd.
 *		http://www.samsung.com
 *
 * Common Header for Exynos machines
 */

/* Dependency: linux/platform_data/cpuidle-exynos.h */

pub const EXYNOS3250_SOC_ID: u32 = 0xE3472000;
pub const EXYNOS3_SOC_MASK: u32 = 0xFFFFF000;

pub const EXYNOS4210_CPU_ID: u32 = 0x43210000;
pub const EXYNOS4212_CPU_ID: u32 = 0x43220000;
pub const EXYNOS4412_CPU_ID: u32 = 0xE4412200;
pub const EXYNOS4_CPU_MASK: u32 = 0xFFFE0000;

pub const EXYNOS5250_SOC_ID: u32 = 0x43520000;
pub const EXYNOS5410_SOC_ID: u32 = 0xE5410000;
pub const EXYNOS5420_SOC_ID: u32 = 0xE5420000;
pub const EXYNOS5800_SOC_ID: u32 = 0xE5422000;
pub const EXYNOS5_SOC_MASK: u32 = 0xFFFFF000;

extern "C" {
    pub static mut exynos_cpu_id: core::ffi::c_ulong;
}

#[inline]
pub unsafe fn is_samsung_exynos3250() -> core::ffi::c_int {
    ((exynos_cpu_id as u32 & EXYNOS3_SOC_MASK) == (EXYNOS3250_SOC_ID & EXYNOS3_SOC_MASK)) as core::ffi::c_int
}
#[inline]
pub unsafe fn is_samsung_exynos4210() -> core::ffi::c_int {
    ((exynos_cpu_id as u32 & EXYNOS4_CPU_MASK) == (EXYNOS4210_CPU_ID & EXYNOS4_CPU_MASK)) as core::ffi::c_int
}
#[inline]
pub unsafe fn is_samsung_exynos4212() -> core::ffi::c_int {
    ((exynos_cpu_id as u32 & EXYNOS4_CPU_MASK) == (EXYNOS4212_CPU_ID & EXYNOS4_CPU_MASK)) as core::ffi::c_int
}
#[inline]
pub unsafe fn is_samsung_exynos4412() -> core::ffi::c_int {
    ((exynos_cpu_id as u32 & EXYNOS4_CPU_MASK) == (EXYNOS4412_CPU_ID & EXYNOS4_CPU_MASK)) as core::ffi::c_int
}
#[inline]
pub unsafe fn is_samsung_exynos5250() -> core::ffi::c_int {
    ((exynos_cpu_id as u32 & EXYNOS5_SOC_MASK) == (EXYNOS5250_SOC_ID & EXYNOS5_SOC_MASK)) as core::ffi::c_int
}
#[inline]
pub unsafe fn is_samsung_exynos5410() -> core::ffi::c_int {
    ((exynos_cpu_id as u32 & EXYNOS5_SOC_MASK) == (EXYNOS5410_SOC_ID & EXYNOS5_SOC_MASK)) as core::ffi::c_int
}
#[inline]
pub unsafe fn is_samsung_exynos5420() -> core::ffi::c_int {
    ((exynos_cpu_id as u32 & EXYNOS5_SOC_MASK) == (EXYNOS5420_SOC_ID & EXYNOS5_SOC_MASK)) as core::ffi::c_int
}
#[inline]
pub unsafe fn is_samsung_exynos5800() -> core::ffi::c_int {
    ((exynos_cpu_id as u32 & EXYNOS5_SOC_MASK) == (EXYNOS5800_SOC_ID & EXYNOS5_SOC_MASK)) as core::ffi::c_int
}

/* CONFIG_* conditions are preserved as Rust cfg conditions. */
#[cfg(CONFIG_SOC_EXYNOS3250)] pub unsafe fn soc_is_exynos3250() -> core::ffi::c_int { is_samsung_exynos3250() }
#[cfg(not(CONFIG_SOC_EXYNOS3250))] pub const fn soc_is_exynos3250() -> core::ffi::c_int { 0 }
#[cfg(CONFIG_CPU_EXYNOS4210)] pub unsafe fn soc_is_exynos4210() -> core::ffi::c_int { is_samsung_exynos4210() }
#[cfg(not(CONFIG_CPU_EXYNOS4210))] pub const fn soc_is_exynos4210() -> core::ffi::c_int { 0 }
#[cfg(CONFIG_SOC_EXYNOS4212)] pub unsafe fn soc_is_exynos4212() -> core::ffi::c_int { is_samsung_exynos4212() }
#[cfg(not(CONFIG_SOC_EXYNOS4212))] pub const fn soc_is_exynos4212() -> core::ffi::c_int { 0 }
#[cfg(CONFIG_SOC_EXYNOS4412)] pub unsafe fn soc_is_exynos4412() -> core::ffi::c_int { is_samsung_exynos4412() }
#[cfg(not(CONFIG_SOC_EXYNOS4412))] pub const fn soc_is_exynos4412() -> core::ffi::c_int { 0 }

pub const EXYNOS4210_REV_0: u32 = 0x0;
pub const EXYNOS4210_REV_1_0: u32 = 0x10;
pub const EXYNOS4210_REV_1_1: u32 = 0x11;

#[cfg(CONFIG_SOC_EXYNOS5250)] pub unsafe fn soc_is_exynos5250() -> core::ffi::c_int { is_samsung_exynos5250() }
#[cfg(not(CONFIG_SOC_EXYNOS5250))] pub const fn soc_is_exynos5250() -> core::ffi::c_int { 0 }
#[cfg(CONFIG_SOC_EXYNOS5410)] pub unsafe fn soc_is_exynos5410() -> core::ffi::c_int { is_samsung_exynos5410() }
#[cfg(not(CONFIG_SOC_EXYNOS5410))] pub const fn soc_is_exynos5410() -> core::ffi::c_int { 0 }
#[cfg(CONFIG_SOC_EXYNOS5420)] pub unsafe fn soc_is_exynos5420() -> core::ffi::c_int { is_samsung_exynos5420() }
#[cfg(not(CONFIG_SOC_EXYNOS5420))] pub const fn soc_is_exynos5420() -> core::ffi::c_int { 0 }
#[cfg(CONFIG_SOC_EXYNOS5800)] pub unsafe fn soc_is_exynos5800() -> core::ffi::c_int { is_samsung_exynos5800() }
#[cfg(not(CONFIG_SOC_EXYNOS5800))] pub const fn soc_is_exynos5800() -> core::ffi::c_int { 0 }

extern "C" {
    pub static mut cp15_save_diag: u32;
    pub static mut cp15_save_power: u32;
    pub static mut sysram_ns_base_addr: *mut core::ffi::c_void;
    pub static mut sysram_base_addr: *mut core::ffi::c_void;
    pub static mut sysram_base_phys: phys_addr_t;
    pub static mut pmu_base_addr: *mut core::ffi::c_void;
    pub fn exynos_sysram_init();
}

pub const FW_DO_IDLE_SLEEP: core::ffi::c_int = 0;
pub const FW_DO_IDLE_AFTR: core::ffi::c_int = 1;

extern "C" {
    pub fn exynos_firmware_init();
    pub fn exynos_secure_firmware_available() -> bool;
    pub fn exynos_set_boot_flag(cpu: core::ffi::c_uint, mode: core::ffi::c_uint);
    pub fn exynos_clear_boot_flag(cpu: core::ffi::c_uint, mode: core::ffi::c_uint);
    pub fn exynos_cpu_resume();
    pub fn exynos_cpu_resume_ns();
    pub static exynos_smp_ops: smp_operations;
    pub fn exynos_cpu_power_down(cpu: core::ffi::c_int);
    pub fn exynos_cpu_power_up(cpu: core::ffi::c_int);
    pub fn exynos_cpu_power_state(cpu: core::ffi::c_int) -> core::ffi::c_int;
    pub fn exynos_cluster_power_down(cluster: core::ffi::c_int);
    pub fn exynos_cluster_power_up(cluster: core::ffi::c_int);
    pub fn exynos_cluster_power_state(cluster: core::ffi::c_int) -> core::ffi::c_int;
    pub fn exynos_cpu_save_register();
    pub fn exynos_cpu_restore_register();
    pub fn exynos_pm_central_suspend();
    pub fn exynos_pm_central_resume() -> core::ffi::c_int;
    pub fn exynos_enter_aftr();
    pub fn exynos_scu_enable();
    pub static mut cpuidle_coupled_exynos_data: cpuidle_exynos_data;
    pub fn exynos_set_delayed_reset_assertion(enable: bool);
    pub fn exynos_rev() -> core::ffi::c_uint;
    pub fn exynos_core_restart(core_id: u32);
    pub fn exynos_set_boot_addr(core_id: u32, boot_addr: core::ffi::c_ulong) -> core::ffi::c_int;
    pub fn exynos_get_boot_addr(core_id: u32, boot_addr: *mut core::ffi::c_ulong) -> core::ffi::c_int;
}

pub const C2_STATE: u32 = 1 << 3;
pub const EXYNOS_SLEEP_MAGIC: u32 = 0x00000bad;
pub const EXYNOS_AFTR_MAGIC: u32 = 0xfcba0d10;

#[inline]
pub unsafe fn pmu_raw_writel(val: u32, offset: u32) {
    writel_relaxed(val, pmu_base_addr.add(offset as usize));
}

#[inline]
pub unsafe fn pmu_raw_readl(offset: u32) -> u32 {
    readl_relaxed(pmu_base_addr.add(offset as usize))
}

extern "C" {
    fn writel_relaxed(val: u32, addr: *mut core::ffi::c_void);
    fn readl_relaxed(addr: *mut core::ffi::c_void) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
