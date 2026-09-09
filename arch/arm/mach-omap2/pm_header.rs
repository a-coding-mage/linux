/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * OMAP2/3 Power Management Routines
 *
 * Copyright (C) 2008 Nokia Corporation
 * Jouni Hogander
 */

// Dependency intent from <linux/err.h> and "powerdomain.h" is preserved by
// the externally supplied types and constants referenced below.

#[cfg(feature = "CONFIG_CPU_IDLE")]
extern "C" {
    pub fn omap3_idle_init() -> ::core::ffi::c_int;
    pub fn omap4_idle_init() -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_CPU_IDLE"))]
#[inline]
pub fn omap3_idle_init() -> ::core::ffi::c_int { 0 }

#[cfg(not(feature = "CONFIG_CPU_IDLE"))]
#[inline]
pub fn omap4_idle_init() -> ::core::ffi::c_int { 0 }

extern "C" {
    pub static mut omap3_secure_ram_storage: *mut ::core::ffi::c_void;
    pub fn omap3_pm_off_mode_enable(enable: ::core::ffi::c_int);
    pub fn omap_sram_idle(rcuidle: bool);
    pub fn omap_pm_clkdms_setup(clkdm: *mut clockdomain, unused: *mut ::core::ffi::c_void)
        -> ::core::ffi::c_int;

    pub fn omap3_pm_get_suspend_state(pwrdm: *mut powerdomain) -> ::core::ffi::c_int;
    pub fn omap3_pm_set_suspend_state(
        pwrdm: *mut powerdomain,
        state: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub static mut enable_off_mode: u32;
}

#[cfg(all(feature = "CONFIG_PM_DEBUG", feature = "CONFIG_DEBUG_FS"))]
extern "C" {
    pub fn pm_dbg_update_time(pwrdm: *mut powerdomain, prev: ::core::ffi::c_int);
}

#[cfg(not(all(feature = "CONFIG_PM_DEBUG", feature = "CONFIG_DEBUG_FS")))]
#[inline]
pub fn pm_dbg_update_time(_pwrdm: *mut powerdomain, _prev: ::core::ffi::c_int) {}

/* 24xx */
extern "C" {
    pub fn omap24xx_cpu_suspend(
        dll_ctrl: u32,
        sdrc_dlla_ctrl: *mut ::core::ffi::c_void,
        sdrc_power: *mut ::core::ffi::c_void,
    );
    pub static mut omap24xx_cpu_suspend_sz: ::core::ffi::c_uint;

    /* 3xxx */
    pub fn omap34xx_cpu_suspend(save_state: ::core::ffi::c_int);

    /* omap3_do_wfi function pointer and size, for copy to SRAM */
    pub fn omap3_do_wfi();
    pub static mut omap3_do_wfi_sz: ::core::ffi::c_uint;
    /* ... and its pointer from SRAM after copy */
    pub static mut omap3_do_wfi_sram: Option<unsafe extern "C" fn()>;

    pub static mut am33xx_pm_sram: am33xx_pm_sram_addr;
    pub static mut am43xx_pm_sram: am33xx_pm_sram_addr;

    pub fn omap3_save_scratchpad_contents();
}

pub const PM_RTA_ERRATUM_i608: u32 = 1 << 0;
pub const PM_SDRC_WAKEUP_ERRATUM_i583: u32 = 1 << 1;
pub const PM_PER_MEMORIES_ERRATUM_i582: u32 = 1 << 2;

#[cfg(all(feature = "CONFIG_PM", feature = "CONFIG_ARCH_OMAP3"))]
extern "C" {
    pub static mut pm34xx_errata: u16;
    pub fn enable_omap3630_toggle_l2_on_restore();
}

#[cfg(all(feature = "CONFIG_PM", feature = "CONFIG_ARCH_OMAP3"))]
#[inline]
pub unsafe fn IS_PM34XX_ERRATUM(id: u16) -> u16 { pm34xx_errata & id }

#[cfg(not(all(feature = "CONFIG_PM", feature = "CONFIG_ARCH_OMAP3")))]
#[inline]
pub const fn IS_PM34XX_ERRATUM(_id: u16) -> u16 { 0 }

#[cfg(not(all(feature = "CONFIG_PM", feature = "CONFIG_ARCH_OMAP3")))]
#[inline]
pub fn enable_omap3630_toggle_l2_on_restore() {}

pub const PM_OMAP4_ROM_SMP_BOOT_ERRATUM_GICD: u32 = 1 << 0;
pub const PM_OMAP4_CPU_OSWR_DISABLE: u32 = 1 << 1;

#[cfg(all(feature = "CONFIG_PM", any(feature = "CONFIG_ARCH_OMAP4", feature = "CONFIG_SOC_OMAP5", feature = "CONFIG_SOC_DRA7XX")))]
extern "C" { pub static mut pm44xx_errata: u16; }

#[cfg(all(feature = "CONFIG_PM", any(feature = "CONFIG_ARCH_OMAP4", feature = "CONFIG_SOC_OMAP5", feature = "CONFIG_SOC_DRA7XX")))]
#[inline]
pub unsafe fn IS_PM44XX_ERRATUM(id: u16) -> u16 { pm44xx_errata & id }

#[cfg(not(all(feature = "CONFIG_PM", any(feature = "CONFIG_ARCH_OMAP4", feature = "CONFIG_SOC_OMAP5", feature = "CONFIG_SOC_DRA7XX"))))]
#[inline]
pub const fn IS_PM44XX_ERRATUM(_id: u16) -> u16 { 0 }

pub const OMAP4_VP_CONFIG_ERROROFFSET: u32 = 0x00;
pub const OMAP4_VP_VSTEPMIN_VSTEPMIN: u32 = 0x01;
pub const OMAP4_VP_VSTEPMAX_VSTEPMAX: u32 = 0x04;
pub const OMAP4_VP_VLIMITTO_TIMEOUT_US: u32 = 200;

#[cfg(feature = "CONFIG_POWER_AVS_OMAP")]
extern "C" { pub fn omap_devinit_smartreflex() -> ::core::ffi::c_int; }
#[cfg(not(feature = "CONFIG_POWER_AVS_OMAP"))]
#[inline]
pub fn omap_devinit_smartreflex() -> ::core::ffi::c_int { -EINVAL }

#[cfg(feature = "CONFIG_TWL4030_CORE")]
extern "C" {
    pub fn omap3_twl_init() -> ::core::ffi::c_int;
    pub fn omap4_twl_init() -> ::core::ffi::c_int;
}
#[cfg(not(feature = "CONFIG_TWL4030_CORE"))]
#[inline] pub fn omap3_twl_init() -> ::core::ffi::c_int { -EINVAL }
#[cfg(not(feature = "CONFIG_TWL4030_CORE"))]
#[inline] pub fn omap4_twl_init() -> ::core::ffi::c_int { -EINVAL }

#[cfg(feature = "CONFIG_MFD_CPCAP")]
extern "C" { pub fn omap4_cpcap_init() -> ::core::ffi::c_int; }
#[cfg(not(feature = "CONFIG_MFD_CPCAP"))]
#[inline] pub fn omap4_cpcap_init() -> ::core::ffi::c_int { -EINVAL }

#[cfg(feature = "CONFIG_PM")]
extern "C" { pub fn omap_pm_get_oscillator(tstart: *mut u32, tshut: *mut u32); }
#[cfg(not(feature = "CONFIG_PM"))]
#[inline]
pub unsafe fn omap_pm_get_oscillator(tstart: *mut u32, tshut: *mut u32) {
    *tstart = 0;
    *tshut = 0;
}

#[cfg(feature = "CONFIG_SUSPEND")]
extern "C" { pub fn omap_common_suspend_init(pm_suspend: *mut ::core::ffi::c_void); }
#[cfg(not(feature = "CONFIG_SUSPEND"))]
#[inline]
pub fn omap_common_suspend_init(_pm_suspend: *mut ::core::ffi::c_void) {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
