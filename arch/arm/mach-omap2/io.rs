// SPDX-License-Identifier: GPL-2.0-only
/* Translated from linux/arch/arm/mach-omap2/io.c. */

// C preprocessor configuration conditions are represented by Rust cfg attributes below.

static mut OMAP_CLK_SOC_INIT: Option<unsafe extern "C" fn() -> i32> = None;

#[repr(C)]
struct MapDesc { virtual_: usize, pfn: usize, length: usize, type_: usize }

#[cfg(any(feature = "CONFIG_SOC_OMAP2420", feature = "CONFIG_SOC_OMAP2430"))]
static mut OMAP24XX_IO_DESC: [MapDesc; 2] = [
    MapDesc { virtual_: L3_24XX_VIRT, pfn: __phys_to_pfn(L3_24XX_PHYS), length: L3_24XX_SIZE, type_: MT_DEVICE },
    MapDesc { virtual_: L4_24XX_VIRT, pfn: __phys_to_pfn(L4_24XX_PHYS), length: L4_24XX_SIZE, type_: MT_DEVICE },
];
#[cfg(feature = "CONFIG_SOC_OMAP2420")]
static mut OMAP242X_IO_DESC: [MapDesc; 3] = [
    MapDesc { virtual_: DSP_MEM_2420_VIRT, pfn: __phys_to_pfn(DSP_MEM_2420_PHYS), length: DSP_MEM_2420_SIZE, type_: MT_DEVICE },
    MapDesc { virtual_: DSP_IPI_2420_VIRT, pfn: __phys_to_pfn(DSP_IPI_2420_PHYS), length: DSP_IPI_2420_SIZE, type_: MT_DEVICE },
    MapDesc { virtual_: DSP_MMU_2420_VIRT, pfn: __phys_to_pfn(DSP_MMU_2420_PHYS), length: DSP_MMU_2420_SIZE, type_: MT_DEVICE },
];
#[cfg(feature = "CONFIG_SOC_OMAP2430")]
static mut OMAP243X_IO_DESC: [MapDesc; 4] = [
    MapDesc { virtual_: L4_WK_243X_VIRT, pfn: __phys_to_pfn(L4_WK_243X_PHYS), length: L4_WK_243X_SIZE, type_: MT_DEVICE },
    MapDesc { virtual_: OMAP243X_GPMC_VIRT, pfn: __phys_to_pfn(OMAP243X_GPMC_PHYS), length: OMAP243X_GPMC_SIZE, type_: MT_DEVICE },
    MapDesc { virtual_: OMAP243X_SDRC_VIRT, pfn: __phys_to_pfn(OMAP243X_SDRC_PHYS), length: OMAP243X_SDRC_SIZE, type_: MT_DEVICE },
    MapDesc { virtual_: OMAP243X_SMS_VIRT, pfn: __phys_to_pfn(OMAP243X_SMS_PHYS), length: OMAP243X_SMS_SIZE, type_: MT_DEVICE },
];

// The remaining mapping tables retain the original conditional availability.
#[cfg(feature = "CONFIG_ARCH_OMAP3")]
static mut OMAP34XX_IO_DESC: [MapDesc; 7] = [
    MapDesc { virtual_: L3_34XX_VIRT, pfn: __phys_to_pfn(L3_34XX_PHYS), length: L3_34XX_SIZE, type_: MT_DEVICE },
    MapDesc { virtual_: L4_34XX_VIRT, pfn: __phys_to_pfn(L4_34XX_PHYS), length: L4_34XX_SIZE, type_: MT_DEVICE },
    MapDesc { virtual_: OMAP34XX_GPMC_VIRT, pfn: __phys_to_pfn(OMAP34XX_GPMC_PHYS), length: OMAP34XX_GPMC_SIZE, type_: MT_DEVICE },
    MapDesc { virtual_: OMAP343X_SMS_VIRT, pfn: __phys_to_pfn(OMAP343X_SMS_PHYS), length: OMAP343X_SMS_SIZE, type_: MT_DEVICE },
    MapDesc { virtual_: OMAP343X_SDRC_VIRT, pfn: __phys_to_pfn(OMAP343X_SDRC_PHYS), length: OMAP343X_SDRC_SIZE, type_: MT_DEVICE },
    MapDesc { virtual_: L4_PER_34XX_VIRT, pfn: __phys_to_pfn(L4_PER_34XX_PHYS), length: L4_PER_34XX_SIZE, type_: MT_DEVICE },
    MapDesc { virtual_: L4_EMU_34XX_VIRT, pfn: __phys_to_pfn(L4_EMU_34XX_PHYS), length: L4_EMU_34XX_SIZE, type_: MT_DEVICE },
];
#[cfg(feature = "CONFIG_SOC_TI81XX")]
static mut OMAPTI81XX_IO_DESC: [MapDesc; 1] = [MapDesc { virtual_: L4_34XX_VIRT, pfn: __phys_to_pfn(L4_34XX_PHYS), length: L4_34XX_SIZE, type_: MT_DEVICE }];
#[cfg(any(feature = "CONFIG_SOC_AM33XX", feature = "CONFIG_SOC_AM43XX"))]
static mut OMAPAM33XX_IO_DESC: [MapDesc; 2] = [
    MapDesc { virtual_: L4_34XX_VIRT, pfn: __phys_to_pfn(L4_34XX_PHYS), length: L4_34XX_SIZE, type_: MT_DEVICE },
    MapDesc { virtual_: L4_WK_AM33XX_VIRT, pfn: __phys_to_pfn(L4_WK_AM33XX_PHYS), length: L4_WK_AM33XX_SIZE, type_: MT_DEVICE },
];

#[cfg(feature = "CONFIG_ARCH_OMAP4")]
static mut OMAP44XX_IO_DESC: [MapDesc; 3] = [
    MapDesc { virtual_: L3_44XX_VIRT, pfn: __phys_to_pfn(L3_44XX_PHYS), length: L3_44XX_SIZE, type_: MT_DEVICE },
    MapDesc { virtual_: L4_44XX_VIRT, pfn: __phys_to_pfn(L4_44XX_PHYS), length: L4_44XX_SIZE, type_: MT_DEVICE },
    MapDesc { virtual_: L4_PER_44XX_VIRT, pfn: __phys_to_pfn(L4_PER_44XX_PHYS), length: L4_PER_44XX_SIZE, type_: MT_DEVICE },
];
#[cfg(feature = "CONFIG_SOC_OMAP5")]
static mut OMAP54XX_IO_DESC: [MapDesc; 4] = [
    MapDesc { virtual_: L3_54XX_VIRT, pfn: __phys_to_pfn(L3_54XX_PHYS), length: L3_54XX_SIZE, type_: MT_DEVICE },
    MapDesc { virtual_: L4_54XX_VIRT, pfn: __phys_to_pfn(L4_54XX_PHYS), length: L4_54XX_SIZE, type_: MT_DEVICE },
    MapDesc { virtual_: L4_WK_54XX_VIRT, pfn: __phys_to_pfn(L4_WK_54XX_PHYS), length: L4_WK_54XX_SIZE, type_: MT_DEVICE },
    MapDesc { virtual_: L4_PER_54XX_VIRT, pfn: __phys_to_pfn(L4_PER_54XX_PHYS), length: L4_PER_54XX_SIZE, type_: MT_DEVICE },
];
#[cfg(feature = "CONFIG_SOC_DRA7XX")]
static mut DRA7XX_IO_DESC: [MapDesc; 7] = [
    MapDesc { virtual_: L4_CFG_MPU_DRA7XX_VIRT, pfn: __phys_to_pfn(L4_CFG_MPU_DRA7XX_PHYS), length: L4_CFG_MPU_DRA7XX_SIZE, type_: MT_DEVICE },
    MapDesc { virtual_: L3_MAIN_SN_DRA7XX_VIRT, pfn: __phys_to_pfn(L3_MAIN_SN_DRA7XX_PHYS), length: L3_MAIN_SN_DRA7XX_SIZE, type_: MT_DEVICE },
    MapDesc { virtual_: L4_PER1_DRA7XX_VIRT, pfn: __phys_to_pfn(L4_PER1_DRA7XX_PHYS), length: L4_PER1_DRA7XX_SIZE, type_: MT_DEVICE },
    MapDesc { virtual_: L4_PER2_DRA7XX_VIRT, pfn: __phys_to_pfn(L4_PER2_DRA7XX_PHYS), length: L4_PER2_DRA7XX_SIZE, type_: MT_DEVICE },
    MapDesc { virtual_: L4_PER3_DRA7XX_VIRT, pfn: __phys_to_pfn(L4_PER3_DRA7XX_PHYS), length: L4_PER3_DRA7XX_SIZE, type_: MT_DEVICE },
    MapDesc { virtual_: L4_CFG_DRA7XX_VIRT, pfn: __phys_to_pfn(L4_CFG_DRA7XX_PHYS), length: L4_CFG_DRA7XX_SIZE, type_: MT_DEVICE },
    MapDesc { virtual_: L4_WKUP_DRA7XX_VIRT, pfn: __phys_to_pfn(L4_WKUP_DRA7XX_PHYS), length: L4_WKUP_DRA7XX_SIZE, type_: MT_DEVICE },
];

unsafe fn map_io(desc: *mut MapDesc, n: usize) { iotable_init(desc, n); }
#[cfg(feature = "CONFIG_SOC_OMAP2420")] pub unsafe fn omap242x_map_io() { map_io(OMAP24XX_IO_DESC.as_mut_ptr(), 2); map_io(OMAP242X_IO_DESC.as_mut_ptr(), 3); }
#[cfg(feature = "CONFIG_SOC_OMAP2430")] pub unsafe fn omap243x_map_io() { map_io(OMAP24XX_IO_DESC.as_mut_ptr(), 2); map_io(OMAP243X_IO_DESC.as_mut_ptr(), 4); }
#[cfg(feature = "CONFIG_ARCH_OMAP3")] pub unsafe fn omap3_map_io() { map_io(OMAP34XX_IO_DESC.as_mut_ptr(), 7); }
#[cfg(feature = "CONFIG_SOC_TI81XX")] pub unsafe fn ti81xx_map_io() { map_io(OMAPTI81XX_IO_DESC.as_mut_ptr(), 1); }
#[cfg(any(feature = "CONFIG_SOC_AM33XX", feature = "CONFIG_SOC_AM43XX"))] pub unsafe fn am33xx_map_io() { map_io(OMAPAM33XX_IO_DESC.as_mut_ptr(), 2); }
#[cfg(feature = "CONFIG_ARCH_OMAP4")] pub unsafe fn omap4_map_io() { map_io(OMAP44XX_IO_DESC.as_mut_ptr(), 3); omap_barriers_init(); }
#[cfg(feature = "CONFIG_SOC_OMAP5")] pub unsafe fn omap5_map_io() { map_io(OMAP54XX_IO_DESC.as_mut_ptr(), 4); omap_barriers_init(); }
#[cfg(feature = "CONFIG_SOC_DRA7XX")] pub unsafe fn dra7xx_map_io() { map_io(DRA7XX_IO_DESC.as_mut_ptr(), 7); omap_barriers_init(); }

unsafe fn _omap2_init_reprogram_sdrc() -> i32 {
    if !cpu_is_omap34xx() { return 0; }
    let ck = clk_get(core::ptr::null(), "dpll3_m2_ck\0".as_ptr() as *const i8);
    if IS_ERR(ck) { return -EINVAL; }
    let rate = clk_get_rate(ck);
    pr_info("Reprogramming SDRC clock to %ld Hz\n\0".as_ptr() as *const i8, rate);
    let v = clk_set_rate(ck, rate);
    if v != 0 { pr_err("dpll3_m2_clk rate change failed: %d\n\0".as_ptr() as *const i8, v); }
    clk_put(ck); v
}

#[cfg(feature = "CONFIG_OMAP_HWMOD")]
unsafe fn _set_hwmod_postsetup_state(oh: *mut omap_hwmod, data: *mut core::ffi::c_void) -> i32 { omap_hwmod_set_postsetup_state(oh, *(data as *mut u8)) }
unsafe fn omap_hwmod_init_postsetup() {
    #[cfg(feature = "CONFIG_OMAP_HWMOD")] { let mut state: u8 = _HWMOD_STATE_DEFAULT; omap_hwmod_for_each(Some(_set_hwmod_postsetup_state), &mut state as *mut _ as *mut _); }
}

// SoC early/late initialization routines, translated with their original call ordering.
#[cfg(feature = "CONFIG_SOC_OMAP2420")] pub unsafe fn omap2420_init_early() { omap2_set_globals_tap(OMAP242X_CLASS, OMAP2_L4_IO_ADDRESS(0x48014000)); omap2_set_globals_sdrc(OMAP2_L3_IO_ADDRESS(OMAP2420_SDRC_BASE), OMAP2_L3_IO_ADDRESS(OMAP2420_SMS_BASE)); omap2_control_base_init(); omap2xxx_check_revision(); omap2_prcm_base_init(); omap2xxx_voltagedomains_init(); omap242x_powerdomains_init(); omap242x_clockdomains_init(); omap2420_hwmod_init(); omap_hwmod_init_postsetup(); OMAP_CLK_SOC_INIT = Some(omap2420_dt_clk_init); rate_table = omap2420_rate_table; }
#[cfg(feature = "CONFIG_SOC_OMAP2430")] pub unsafe fn omap2430_init_early() { omap2_set_globals_tap(OMAP243X_CLASS, OMAP2_L4_IO_ADDRESS(0x4900a000)); omap2_set_globals_sdrc(OMAP2_L3_IO_ADDRESS(OMAP243X_SDRC_BASE), OMAP2_L3_IO_ADDRESS(OMAP243X_SMS_BASE)); omap2_control_base_init(); omap2xxx_check_revision(); omap2_prcm_base_init(); omap2xxx_voltagedomains_init(); omap243x_powerdomains_init(); omap243x_clockdomains_init(); omap2430_hwmod_init(); omap_hwmod_init_postsetup(); OMAP_CLK_SOC_INIT = Some(omap2430_dt_clk_init); rate_table = omap2430_rate_table; }

#[cfg(feature = "CONFIG_ARCH_OMAP3")] unsafe fn omap3_init_early() { omap2_set_globals_tap(OMAP343X_CLASS, OMAP2_L4_IO_ADDRESS(0x4830A000)); omap2_set_globals_sdrc(OMAP2_L3_IO_ADDRESS(OMAP343X_SDRC_BASE), OMAP2_L3_IO_ADDRESS(OMAP343X_SMS_BASE)); omap2_control_base_init(); omap3xxx_check_revision(); omap3xxx_check_features(); omap2_prcm_base_init(); omap3xxx_voltagedomains_init(); omap3xxx_powerdomains_init(); omap3xxx_clockdomains_init(); omap3xxx_hwmod_init(); omap_hwmod_init_postsetup(); omap_secure_init(); }
#[cfg(feature = "CONFIG_ARCH_OMAP3")] pub unsafe fn omap3430_init_early() { omap3_init_early(); OMAP_CLK_SOC_INIT = Some(omap3430_dt_clk_init); }
#[cfg(feature = "CONFIG_ARCH_OMAP3")] pub unsafe fn omap3630_init_early() { omap3_init_early(); OMAP_CLK_SOC_INIT = Some(omap3630_dt_clk_init); }
#[cfg(feature = "CONFIG_ARCH_OMAP3")] pub unsafe fn am35xx_init_early() { omap3_init_early(); OMAP_CLK_SOC_INIT = Some(am35xx_dt_clk_init); }
#[cfg(feature = "CONFIG_ARCH_OMAP3")] pub unsafe fn omap3_init_late() { omap_pm_soc_init = Some(omap3_pm_init); }
#[cfg(feature = "CONFIG_ARCH_OMAP3")] pub unsafe fn ti81xx_init_late() { omap_pm_soc_init = Some(omap_pm_nop_init); }

#[cfg(feature = "CONFIG_SOC_TI81XX")] pub unsafe fn ti814x_init_early() { omap2_set_globals_tap(TI814X_CLASS, OMAP2_L4_IO_ADDRESS(TI81XX_TAP_BASE)); omap2_control_base_init(); omap3xxx_check_revision(); ti81xx_check_features(); omap2_prcm_base_init(); omap3xxx_voltagedomains_init(); omap3xxx_powerdomains_init(); ti814x_clockdomains_init(); dm814x_hwmod_init(); omap_hwmod_init_postsetup(); OMAP_CLK_SOC_INIT = Some(dm814x_dt_clk_init); omap_secure_init(); }
#[cfg(feature = "CONFIG_SOC_TI81XX")] pub unsafe fn ti816x_init_early() { omap2_set_globals_tap(TI816X_CLASS, OMAP2_L4_IO_ADDRESS(TI81XX_TAP_BASE)); omap2_control_base_init(); omap3xxx_check_revision(); ti81xx_check_features(); omap2_prcm_base_init(); omap3xxx_voltagedomains_init(); omap3xxx_powerdomains_init(); ti816x_clockdomains_init(); dm816x_hwmod_init(); omap_hwmod_init_postsetup(); OMAP_CLK_SOC_INIT = Some(dm816x_dt_clk_init); omap_secure_init(); }

#[cfg(feature = "CONFIG_SOC_AM33XX")] pub unsafe fn am33xx_init_early() { omap2_set_globals_tap(AM335X_CLASS, AM33XX_L4_WK_IO_ADDRESS(AM33XX_TAP_BASE)); omap2_control_base_init(); omap3xxx_check_revision(); am33xx_check_features(); omap2_prcm_base_init(); am33xx_powerdomains_init(); am33xx_clockdomains_init(); OMAP_CLK_SOC_INIT = Some(am33xx_dt_clk_init); omap_secure_init(); }
#[cfg(feature = "CONFIG_SOC_AM33XX")] pub unsafe fn am33xx_init_late() { omap_pm_soc_init = Some(amx3_common_pm_init); }
#[cfg(feature = "CONFIG_SOC_AM43XX")] pub unsafe fn am43xx_init_early() { omap2_set_globals_tap(AM335X_CLASS, AM33XX_L4_WK_IO_ADDRESS(AM33XX_TAP_BASE)); omap2_control_base_init(); omap3xxx_check_revision(); am33xx_check_features(); omap2_prcm_base_init(); am43xx_powerdomains_init(); am43xx_clockdomains_init(); omap_l2_cache_init(); OMAP_CLK_SOC_INIT = Some(am43xx_dt_clk_init); omap_secure_init(); }
#[cfg(feature = "CONFIG_SOC_AM43XX")] pub unsafe fn am43xx_init_late() { omap_pm_soc_init = Some(amx3_common_pm_init); }

#[cfg(feature = "CONFIG_ARCH_OMAP4")] pub unsafe fn omap4430_init_early() { omap2_set_globals_tap(OMAP443X_CLASS, OMAP2_L4_IO_ADDRESS(OMAP443X_SCM_BASE)); omap2_set_globals_prcm_mpu(OMAP2_L4_IO_ADDRESS(OMAP4430_PRCM_MPU_BASE)); omap2_control_base_init(); omap4xxx_check_revision(); omap4xxx_check_features(); omap2_prcm_base_init(); omap4_sar_ram_init(); omap4_mpuss_early_init(); omap4_pm_init_early(); omap44xx_voltagedomains_init(); omap44xx_powerdomains_init(); omap44xx_clockdomains_init(); omap_l2_cache_init(); OMAP_CLK_SOC_INIT = Some(omap4xxx_dt_clk_init); omap_secure_init(); }
#[cfg(feature = "CONFIG_ARCH_OMAP4")] pub unsafe fn omap4430_init_late() { omap_pm_soc_init = Some(omap4_pm_init); }
#[cfg(feature = "CONFIG_SOC_OMAP5")] pub unsafe fn omap5_init_early() { omap2_set_globals_tap(OMAP54XX_CLASS, OMAP2_L4_IO_ADDRESS(OMAP54XX_SCM_BASE)); omap2_set_globals_prcm_mpu(OMAP2_L4_IO_ADDRESS(OMAP54XX_PRCM_MPU_BASE)); omap2_control_base_init(); omap2_prcm_base_init(); omap5xxx_check_revision(); omap4_sar_ram_init(); omap4_mpuss_early_init(); omap4_pm_init_early(); omap54xx_voltagedomains_init(); omap54xx_powerdomains_init(); omap54xx_clockdomains_init(); OMAP_CLK_SOC_INIT = Some(omap5xxx_dt_clk_init); omap_secure_init(); }
#[cfg(feature = "CONFIG_SOC_OMAP5")] pub unsafe fn omap5_init_late() { omap_pm_soc_init = Some(omap4_pm_init); }
#[cfg(feature = "CONFIG_SOC_DRA7XX")] pub unsafe fn dra7xx_init_early() { omap2_set_globals_tap(DRA7XX_CLASS, OMAP2_L4_IO_ADDRESS(DRA7XX_TAP_BASE)); omap2_set_globals_prcm_mpu(OMAP2_L4_IO_ADDRESS(OMAP54XX_PRCM_MPU_BASE)); omap2_control_base_init(); omap4_pm_init_early(); omap2_prcm_base_init(); dra7xxx_check_revision(); dra7xx_powerdomains_init(); dra7xx_clockdomains_init(); OMAP_CLK_SOC_INIT = Some(dra7xx_dt_clk_init); omap_secure_init(); }
#[cfg(feature = "CONFIG_SOC_DRA7XX")] pub unsafe fn dra7xx_init_late() { omap_pm_soc_init = Some(omap4_pm_init); }

pub unsafe fn omap_sdrc_init(sdrc_cs0: *mut omap_sdrc_params, sdrc_cs1: *mut omap_sdrc_params) { omap_sram_init(); if cpu_is_omap24xx() || omap3_has_sdrc() { omap2_sdrc_init(sdrc_cs0, sdrc_cs1); _omap2_init_reprogram_sdrc(); } }
pub unsafe fn omap_clk_init() -> i32 { if OMAP_CLK_SOC_INIT.is_none() { return 0; } ti_clk_init_features(); omap2_clk_setup_ll_ops(); let mut ret = omap_control_init(); if ret != 0 { return ret; } ret = omap_prcm_init(); if ret != 0 { return ret; } of_clk_init(core::ptr::null()); ti_dt_clk_init_retry_clks(); ti_dt_clockdomains_setup(); ret = (OMAP_CLK_SOC_INIT.unwrap())(); ret }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
