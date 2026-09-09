/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ti_sysc_module_type {
    TI_SYSC_OMAP2,
    TI_SYSC_OMAP2_TIMER,
    TI_SYSC_OMAP3_SHAM,
    TI_SYSC_OMAP3_AES,
    TI_SYSC_OMAP4,
    TI_SYSC_OMAP4_TIMER,
    TI_SYSC_OMAP4_SIMPLE,
    TI_SYSC_OMAP34XX_SR,
    TI_SYSC_OMAP36XX_SR,
    TI_SYSC_OMAP4_SR,
    TI_SYSC_OMAP4_MCASP,
    TI_SYSC_OMAP4_USB_HOST_FS,
    TI_SYSC_DRA7_MCAN,
    TI_SYSC_PRUSS,
}

#[repr(C)]
pub struct ti_sysc_cookie {
    pub data: *mut core::ffi::c_void,
    pub clkdm: *mut core::ffi::c_void,
}

/**
 * struct sysc_regbits - TI OCP_SYSCONFIG register field offsets
 * @midle_shift: Offset of the midle bit
 * @clkact_shift: Offset of the clockactivity bit
 * @sidle_shift: Offset of the sidle bit
 * @enwkup_shift: Offset of the enawakeup bit
 * @srst_shift: Offset of the softreset bit
 * @autoidle_shift: Offset of the autoidle bit
 * @dmadisable_shift: Offset of the dmadisable bit
 * @emufree_shift; Offset of the emufree bit
 *
 * Note that 0 is a valid shift, and for ti-sysc.c -ENODEV can be used if a
 * feature is not available.
 */
#[repr(C)]
pub struct sysc_regbits {
    pub midle_shift: i8,
    pub clkact_shift: i8,
    pub sidle_shift: i8,
    pub enwkup_shift: i8,
    pub srst_shift: i8,
    pub autoidle_shift: i8,
    pub dmadisable_shift: i8,
    pub emufree_shift: i8,
}

pub const SYSC_MODULE_QUIRK_OTG: u32 = 1u32 << 30;
pub const SYSC_QUIRK_RESET_ON_CTX_LOST: u32 = 1u32 << 29;
pub const SYSC_QUIRK_REINIT_ON_CTX_LOST: u32 = 1u32 << 28;
pub const SYSC_QUIRK_REINIT_ON_RESUME: u32 = 1u32 << 27;
pub const SYSC_QUIRK_GPMC_DEBUG: u32 = 1u32 << 26;
pub const SYSC_MODULE_QUIRK_ENA_RESETDONE: u32 = 1u32 << 25;
pub const SYSC_MODULE_QUIRK_PRUSS: u32 = 1u32 << 24;
pub const SYSC_MODULE_QUIRK_DSS_RESET: u32 = 1u32 << 23;
pub const SYSC_MODULE_QUIRK_RTC_UNLOCK: u32 = 1u32 << 22;
pub const SYSC_QUIRK_CLKDM_NOAUTO: u32 = 1u32 << 21;
pub const SYSC_QUIRK_FORCE_MSTANDBY: u32 = 1u32 << 20;
pub const SYSC_MODULE_QUIRK_AESS: u32 = 1u32 << 19;
pub const SYSC_MODULE_QUIRK_SGX: u32 = 1u32 << 18;
pub const SYSC_MODULE_QUIRK_HDQ1W: u32 = 1u32 << 17;
pub const SYSC_MODULE_QUIRK_I2C: u32 = 1u32 << 16;
pub const SYSC_MODULE_QUIRK_WDT: u32 = 1u32 << 15;
pub const SYSS_QUIRK_RESETDONE_INVERTED: u32 = 1u32 << 14;
pub const SYSC_QUIRK_SWSUP_MSTANDBY: u32 = 1u32 << 13;
pub const SYSC_QUIRK_SWSUP_SIDLE_ACT: u32 = 1u32 << 12;
pub const SYSC_QUIRK_SWSUP_SIDLE: u32 = 1u32 << 11;
pub const SYSC_QUIRK_EXT_OPT_CLOCK: u32 = 1u32 << 10;
pub const SYSC_QUIRK_RESET_STATUS: u32 = 1u32 << 8;
pub const SYSC_QUIRK_NO_IDLE: u32 = 1u32 << 7;
pub const SYSC_QUIRK_NO_IDLE_ON_INIT: u32 = 1u32 << 6;
pub const SYSC_QUIRK_NO_RESET_ON_INIT: u32 = 1u32 << 5;
pub const SYSC_QUIRK_OPT_CLKS_NEEDED: u32 = 1u32 << 4;
pub const SYSC_QUIRK_OPT_CLKS_IN_RESET: u32 = 1u32 << 3;
pub const SYSC_QUIRK_16BIT: u32 = 1u32 << 2;
pub const SYSC_QUIRK_UNCACHED: u32 = 1u32 << 1;
pub const SYSC_QUIRK_USE_CLOCKACT: u32 = 1u32 << 0;

pub const SYSC_NR_IDLEMODES: u32 = 4;

/**
 * struct sysc_capabilities - capabilities for an interconnect target module
 * @type: sysc type identifier for the module
 * @sysc_mask: bitmask of supported SYSCONFIG register bits
 * @regbits: bitmask of SYSCONFIG register bits
 * @mod_quirks: bitmask of module specific quirks
 */
#[repr(C)]
pub struct sysc_capabilities {
    pub type_: ti_sysc_module_type,
    pub sysc_mask: u32,
    pub regbits: *const sysc_regbits,
    pub mod_quirks: u32,
}

/**
 * struct sysc_config - configuration for an interconnect target module
 * @sysc_val: configured value for sysc register
 * @syss_mask: configured mask value for SYSSTATUS register
 * @midlemodes: bitmask of supported master idle modes
 * @sidlemodes: bitmask of supported slave idle modes
 * @srst_udelay: optional delay needed after OCP soft reset
 * @quirks: bitmask of enabled quirks
 */
#[repr(C)]
pub struct sysc_config {
    pub sysc_val: u32,
    pub syss_mask: u32,
    pub midlemodes: u8,
    pub sidlemodes: u8,
    pub srst_udelay: u8,
    pub quirks: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sysc_registers {
    SYSC_REVISION,
    SYSC_SYSCONFIG,
    SYSC_SYSSTATUS,
    SYSC_MAX_REGS,
}

/**
 * struct ti_sysc_module_data - ti-sysc to hwmod translation data for a module
 * @name: legacy "ti,hwmods" module name
 * @module_pa: physical address of the interconnect target module
 * @module_size: size of the interconnect target module
 * @offsets: array of register offsets as listed in enum sysc_registers
 * @nr_offsets: number of registers
 * @cap: interconnect target module capabilities
 * @cfg: interconnect target module configuration
 *
 * This data is enough to allocate a new struct omap_hwmod_class_sysconfig
 * based on device tree data parsed by ti-sysc driver.
 */
#[repr(C)]
pub struct ti_sysc_module_data {
    pub name: *const core::ffi::c_char,
    pub module_pa: u64,
    pub module_size: u32,
    pub offsets: *mut i32,
    pub nr_offsets: i32,
    pub cap: *const sysc_capabilities,
    pub cfg: *mut sysc_config,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_dev_auxdata {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ti_sysc_platform_data {
    pub auxdata: *mut of_dev_auxdata,
    pub soc_type_gp: Option<unsafe extern "C" fn() -> bool>,
    pub init_clockdomain: Option<unsafe extern "C" fn(
        dev: *mut device,
        fck: *mut clk,
        ick: *mut clk,
        cookie: *mut ti_sysc_cookie,
    ) -> i32>,
    pub clkdm_deny_idle: Option<unsafe extern "C" fn(
        dev: *mut device,
        cookie: *const ti_sysc_cookie,
    )>,
    pub clkdm_allow_idle: Option<unsafe extern "C" fn(
        dev: *mut device,
        cookie: *const ti_sysc_cookie,
    )>,
    pub init_module: Option<unsafe extern "C" fn(
        dev: *mut device,
        data: *const ti_sysc_module_data,
        cookie: *mut ti_sysc_cookie,
    ) -> i32>,
    pub enable_module: Option<unsafe extern "C" fn(
        dev: *mut device,
        cookie: *const ti_sysc_cookie,
    ) -> i32>,
    pub idle_module: Option<unsafe extern "C" fn(
        dev: *mut device,
        cookie: *const ti_sysc_cookie,
    ) -> i32>,
    pub shutdown_module: Option<unsafe extern "C" fn(
        dev: *mut device,
        cookie: *const ti_sysc_cookie,
    ) -> i32>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
