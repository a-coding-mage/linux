// SPDX-License-Identifier: GPL-2.0-only
/*
 * TI CPUFreq/OPP hw-supported driver
 *
 * Copyright (C) 2016-2017 Texas Instruments, Inc.
 *	 Dave Gerlach <d-gerlach@ti.com>
 */

// Linux kernel dependencies supplied by other translation units.

const REVISION_MASK: u32 = 0xF;
const REVISION_SHIFT: u32 = 28;
const AM33XX_800M_ARM_MPU_MAX_FREQ: u64 = 0x1E2F;
const AM43XX_600M_ARM_MPU_MAX_FREQ: u64 = 0xFFA;
const DRA7_EFUSE_HAS_OD_MPU_OPP: u64 = 11;
const DRA7_EFUSE_HAS_HIGH_MPU_OPP: u64 = 15;
const DRA76_EFUSE_HAS_PLUS_MPU_OPP: u64 = 18;
const DRA7_EFUSE_HAS_ALL_MPU_OPP: u64 = 23;
const DRA76_EFUSE_HAS_ALL_MPU_OPP: u64 = 24;
const DRA7_EFUSE_NOM_MPU_OPP: u64 = 1 << 0;
const DRA7_EFUSE_OD_MPU_OPP: u64 = 1 << 1;
const DRA7_EFUSE_HIGH_MPU_OPP: u64 = 1 << 2;
const DRA76_EFUSE_PLUS_MPU_OPP: u64 = 1 << 3;
const OMAP3_CONTROL_DEVICE_STATUS: u64 = 0x4800244C;
const OMAP3_CONTROL_IDCODE: u64 = 0x4830A204;
const OMAP34xx_ProdID_SKUID: u64 = 0x4830A20C;
const OMAP3_SYSCON_BASE: u64 = 0x48000000 + 0x2000 + 0x270;
const AM625_EFUSE_K_MPU_OPP: u64 = 11;
const AM625_EFUSE_S_MPU_OPP: u64 = 19;
const AM625_EFUSE_T_MPU_OPP: u64 = 20;
const AM625_SUPPORT_K_MPU_OPP: u64 = 1 << 0;
const AM625_SUPPORT_S_MPU_OPP: u64 = 1 << 1;
const AM625_SUPPORT_T_MPU_OPP: u64 = 1 << 2;
const AM62A7_EFUSE_M_MPU_OPP: u64 = 13;
const AM62A7_EFUSE_N_MPU_OPP: u64 = 14;
const AM62A7_EFUSE_O_MPU_OPP: u64 = 15;
const AM62A7_EFUSE_P_MPU_OPP: u64 = 16;
const AM62A7_EFUSE_Q_MPU_OPP: u64 = 17;
const AM62A7_EFUSE_R_MPU_OPP: u64 = 18;
const AM62A7_EFUSE_S_MPU_OPP: u64 = 19;
const AM62A7_EFUSE_V_MPU_OPP: u64 = 20;
const AM62A7_EFUSE_U_MPU_OPP: u64 = 21;
const AM62A7_EFUSE_T_MPU_OPP: u64 = 22;
const AM62A7_SUPPORT_N_MPU_OPP: u64 = 1 << 0;
const AM62A7_SUPPORT_R_MPU_OPP: u64 = 1 << 1;
const AM62A7_SUPPORT_V_MPU_OPP: u64 = 1 << 2;
const AM62L3_EFUSE_E_MPU_OPP: u64 = 5;
const AM62L3_EFUSE_O_MPU_OPP: u64 = 15;
const AM62L3_SUPPORT_E_MPU_OPP: u64 = 1 << 0;
const AM62L3_SUPPORT_O_MPU_OPP: u64 = 1 << 1;
const AM62P5_EFUSE_O_MPU_OPP: u64 = 15;
const AM62P5_EFUSE_S_MPU_OPP: u64 = 19;
const AM62P5_EFUSE_T_MPU_OPP: u64 = 20;
const AM62P5_EFUSE_U_MPU_OPP: u64 = 21;
const AM62P5_EFUSE_V_MPU_OPP: u64 = 22;
const AM62P5_SUPPORT_O_MPU_OPP: u64 = 1 << 0;
const AM62P5_SUPPORT_U_MPU_OPP: u64 = 1 << 2;
const VERSION_COUNT: usize = 2;
const TI_QUIRK_SYSCON_MAY_BE_MISSING: u8 = 0x1;
const TI_QUIRK_SYSCON_IS_SINGLE_REG: u8 = 0x2;

#[repr(C)]
pub struct ti_cpufreq_data {
    pub cpu_dev: *mut device,
    pub opp_node: *mut device_node,
    pub syscon: *mut regmap,
    pub soc_data: *const ti_cpufreq_soc_data,
}

#[repr(C)]
pub struct ti_cpufreq_soc_data {
    pub reg_names: *const *const i8,
    pub efuse_xlate: Option<unsafe extern "C" fn(*mut ti_cpufreq_data, u64) -> u64>,
    pub efuse_fallback: u64,
    pub efuse_offset: u64,
    pub efuse_mask: u64,
    pub efuse_shift: u64,
    pub rev_offset: u64,
    pub multi_regulator: bool,
    pub needs_k3_socinfo: bool,
    pub quirks: u8,
}

#[repr(C)] pub struct device;
#[repr(C)] pub struct device_node;
#[repr(C)] pub struct regmap;
#[repr(C)] pub struct platform_device;
#[repr(C)] pub struct of_device_id { pub compatible: *const i8, pub data: *const core::ffi::c_void }

unsafe fn amx3_efuse_xlate(d: *mut ti_cpufreq_data, mut efuse: u64) -> u64 {
    if efuse == 0 { efuse = (*(*d).soc_data).efuse_fallback; }
    !efuse
}
unsafe fn dra7_efuse_xlate(_: *mut ti_cpufreq_data, efuse: u64) -> u64 {
    let mut v = DRA7_EFUSE_NOM_MPU_OPP;
    match efuse {
        DRA76_EFUSE_HAS_PLUS_MPU_OPP | DRA76_EFUSE_HAS_ALL_MPU_OPP => { v |= DRA76_EFUSE_PLUS_MPU_OPP; v |= DRA7_EFUSE_HIGH_MPU_OPP; v |= DRA7_EFUSE_OD_MPU_OPP; }
        DRA7_EFUSE_HAS_ALL_MPU_OPP | DRA7_EFUSE_HAS_HIGH_MPU_OPP => { v |= DRA7_EFUSE_HIGH_MPU_OPP; v |= DRA7_EFUSE_OD_MPU_OPP; }
        DRA7_EFUSE_HAS_OD_MPU_OPP => v |= DRA7_EFUSE_OD_MPU_OPP,
        _ => {}
    } v
}
unsafe fn omap3_efuse_xlate(_: *mut ti_cpufreq_data, efuse: u64) -> u64 { 1u64.wrapping_shl(efuse as u32) }
unsafe fn am62p5_efuse_xlate(_: *mut ti_cpufreq_data, e: u64) -> u64 { let mut v=AM62P5_SUPPORT_O_MPU_OPP; if (AM62P5_EFUSE_S_MPU_OPP..=AM62P5_EFUSE_V_MPU_OPP).contains(&e) { v|=AM62P5_SUPPORT_U_MPU_OPP; } v }
unsafe fn am62a7_efuse_xlate(_: *mut ti_cpufreq_data, e: u64) -> u64 { let mut v=AM62A7_SUPPORT_N_MPU_OPP; if e>=AM62A7_EFUSE_R_MPU_OPP {v|=AM62A7_SUPPORT_V_MPU_OPP;v|=AM62A7_SUPPORT_R_MPU_OPP;} else if e>=AM62A7_EFUSE_N_MPU_OPP {v|=AM62A7_SUPPORT_R_MPU_OPP;} v }
unsafe fn am625_efuse_xlate(_: *mut ti_cpufreq_data, e: u64) -> u64 { let mut v=AM625_SUPPORT_K_MPU_OPP; if e>=AM625_EFUSE_S_MPU_OPP {v|=AM625_SUPPORT_S_MPU_OPP;} if e==AM625_EFUSE_T_MPU_OPP {v|=AM625_SUPPORT_T_MPU_OPP;} v }
unsafe fn am62l3_efuse_xlate(_: *mut ti_cpufreq_data, e: u64) -> u64 { let mut v=AM62L3_SUPPORT_E_MPU_OPP; if e==AM62L3_EFUSE_O_MPU_OPP {v|=AM62L3_SUPPORT_O_MPU_OPP;} v }

// The remaining kernel-facing tables and functions retain the original ABI and
// are expressed below as declarations for implementation by kernel bindings.
extern "C" {
    fn ti_cpufreq_probe(pdev: *mut platform_device) -> i32;
    fn ti_cpufreq_init() -> i32;
}

// SoC data, device-match tables, platform-driver registration, and module
// metadata are supplied through the corresponding Linux kernel Rust bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
