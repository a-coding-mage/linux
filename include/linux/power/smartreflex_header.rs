/* SPDX-License-Identifier: GPL-2.0 */
/* OMAP Smartreflex Defines and Routines */

// C dependencies: linux/types.h, linux/platform_device.h, linux/delay.h,
// and linux/platform_data/voltage-omap.h.

pub const SR_TYPE_V1: u32 = 1;
pub const SR_TYPE_V2: u32 = 2;

pub const SRCONFIG: u32 = 0x00;
pub const SRSTATUS: u32 = 0x04;
pub const SENVAL: u32 = 0x08;
pub const SENMIN: u32 = 0x0C;
pub const SENMAX: u32 = 0x10;
pub const SENAVG: u32 = 0x14;
pub const AVGWEIGHT: u32 = 0x18;
pub const NVALUERECIPROCAL: u32 = 0x1c;
pub const SENERROR_V1: u32 = 0x20;
pub const ERRCONFIG_V1: u32 = 0x24;
pub const IRQ_EOI: u32 = 0x20;
pub const IRQSTATUS_RAW: u32 = 0x24;
pub const IRQSTATUS: u32 = 0x28;
pub const IRQENABLE_SET: u32 = 0x2C;
pub const IRQENABLE_CLR: u32 = 0x30;
pub const SENERROR_V2: u32 = 0x34;
pub const ERRCONFIG_V2: u32 = 0x38;

pub const SRCONFIG_ACCUMDATA_SHIFT: u32 = 22;
pub const SRCONFIG_SRCLKLENGTH_SHIFT: u32 = 12;
pub const SRCONFIG_SENNENABLE_V1_SHIFT: u32 = 5;
pub const SRCONFIG_SENPENABLE_V1_SHIFT: u32 = 3;
pub const SRCONFIG_SENNENABLE_V2_SHIFT: u32 = 1;
pub const SRCONFIG_SENPENABLE_V2_SHIFT: u32 = 0;
pub const SRCONFIG_CLKCTRL_SHIFT: u32 = 0;
pub const SRCONFIG_ACCUMDATA_MASK: u32 = 0x3ff << 22;
pub const SRCONFIG_SRENABLE: u32 = 1 << 11;
pub const SRCONFIG_SENENABLE: u32 = 1 << 10;
pub const SRCONFIG_ERRGEN_EN: u32 = 1 << 9;
pub const SRCONFIG_MINMAXAVG_EN: u32 = 1 << 8;
pub const SRCONFIG_DELAYCTRL: u32 = 1 << 2;

pub const AVGWEIGHT_SENPAVGWEIGHT_SHIFT: u32 = 2;
pub const AVGWEIGHT_SENNAVGWEIGHT_SHIFT: u32 = 0;
pub const NVALUERECIPROCAL_SENPGAIN_SHIFT: u32 = 20;
pub const NVALUERECIPROCAL_SENNGAIN_SHIFT: u32 = 16;
pub const NVALUERECIPROCAL_RNSENP_SHIFT: u32 = 8;
pub const NVALUERECIPROCAL_RNSENN_SHIFT: u32 = 0;
pub const ERRCONFIG_ERRWEIGHT_SHIFT: u32 = 16;
pub const ERRCONFIG_ERRMAXLIMIT_SHIFT: u32 = 8;
pub const ERRCONFIG_ERRMINLIMIT_SHIFT: u32 = 0;
pub const SR_ERRWEIGHT_MASK: u32 = 0x07 << 16;
pub const SR_ERRMAXLIMIT_MASK: u32 = 0xff << 8;
pub const SR_ERRMINLIMIT_MASK: u32 = 0xff;
pub const ERRCONFIG_VPBOUNDINTEN_V1: u32 = 1 << 31;
pub const ERRCONFIG_VPBOUNDINTST_V1: u32 = 1 << 30;
pub const ERRCONFIG_MCUACCUMINTEN: u32 = 1 << 29;
pub const ERRCONFIG_MCUACCUMINTST: u32 = 1 << 28;
pub const ERRCONFIG_MCUVALIDINTEN: u32 = 1 << 27;
pub const ERRCONFIG_MCUVALIDINTST: u32 = 1 << 26;
pub const ERRCONFIG_MCUBOUNDINTEN: u32 = 1 << 25;
pub const ERRCONFIG_MCUBOUNDINTST: u32 = 1 << 24;
pub const ERRCONFIG_MCUDISACKINTEN: u32 = 1 << 23;
pub const ERRCONFIG_VPBOUNDINTST_V2: u32 = 1 << 23;
pub const ERRCONFIG_MCUDISACKINTST: u32 = 1 << 22;
pub const ERRCONFIG_VPBOUNDINTEN_V2: u32 = 1 << 22;
pub const ERRCONFIG_STATUS_V1_MASK: u32 = ERRCONFIG_VPBOUNDINTST_V1 | ERRCONFIG_MCUACCUMINTST | ERRCONFIG_MCUVALIDINTST | ERRCONFIG_MCUBOUNDINTST | ERRCONFIG_MCUDISACKINTST;
pub const IRQSTATUS_MCUACCUMINT: u32 = 1 << 3;
pub const IRQSTATUS_MCVALIDINT: u32 = 1 << 2;
pub const IRQSTATUS_MCBOUNDSINT: u32 = 1 << 1;
pub const IRQSTATUS_MCUDISABLEACKINT: u32 = 1;
pub const IRQENABLE_MCUACCUMINT: u32 = 1 << 3;
pub const IRQENABLE_MCUVALIDINT: u32 = 1 << 2;
pub const IRQENABLE_MCUBOUNDSINT: u32 = 1 << 1;
pub const IRQENABLE_MCUDISABLEACKINT: u32 = 1;

pub const SRCLKLENGTH_12MHZ_SYSCLK: u32 = 0x3c;
pub const SRCLKLENGTH_13MHZ_SYSCLK: u32 = 0x41;
pub const SRCLKLENGTH_19MHZ_SYSCLK: u32 = 0x60;
pub const SRCLKLENGTH_26MHZ_SYSCLK: u32 = 0x82;
pub const SRCLKLENGTH_38MHZ_SYSCLK: u32 = 0xC0;
pub const OMAP3430_SR_ACCUMDATA: u32 = 0x1f4;
pub const OMAP3430_SR1_SENPAVGWEIGHT: u32 = 0x03;
pub const OMAP3430_SR1_SENNAVGWEIGHT: u32 = 0x03;
pub const OMAP3430_SR2_SENPAVGWEIGHT: u32 = 0x01;
pub const OMAP3430_SR2_SENNAVGWEIGHT: u32 = 0x01;
pub const OMAP3430_SR_ERRWEIGHT: u32 = 0x04;
pub const OMAP3430_SR_ERRMAXLIMIT: u32 = 0x02;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sr_instance { OMAP_SR_MPU, OMAP_SR_CORE, OMAP_SR_IVA, OMAP_SR_NR }

pub enum list_head {}
pub enum platform_device {}
pub enum voltagedomain {}
pub enum dentry {}
pub enum clk {}

#[repr(C)]
pub struct omap_sr {
    pub name: *mut core::ffi::c_char,
    pub node: list_head,
    pub pdev: *mut platform_device,
    pub nvalue_table: *mut omap_sr_nvalue_table,
    pub voltdm: *mut voltagedomain,
    pub dbg_dir: *mut dentry,
    pub irq: u32,
    pub fck: *mut clk,
    pub srid: i32,
    pub ip_type: i32,
    pub nvalue_count: i32,
    pub autocomp_active: bool,
    pub clk_length: u32,
    pub err_weight: u32,
    pub err_minlimit: u32,
    pub err_maxlimit: u32,
    pub accum_data: u32,
    pub senn_avgweight: u32,
    pub senp_avgweight: u32,
    pub senp_mod: u32,
    pub senn_mod: u32,
    pub base: *mut core::ffi::c_void,
    pub enabled: u8,
}

extern "C" { pub fn udelay(usecs: u32); }

#[macro_export]
macro_rules! sr_test_cond_timeout {
    ($cond:expr, $timeout:expr, $index:ident) => {{
        for $index in 0..$timeout {
            if $cond { break; }
            unsafe { $crate::udelay(1); }
        }
    }};
}

#[repr(C)]
pub struct omap_sr_pmic_data { pub sr_pmic_init: Option<unsafe extern "C" fn()> }

#[repr(C)]
pub struct omap_smartreflex_dev_attr { pub sensor_voltdm_name: *const core::ffi::c_char }

pub const SR_CLASS1: u32 = 0x1;
pub const SR_CLASS2: u32 = 0x2;
pub const SR_CLASS3: u32 = 0x3;

#[repr(C)]
pub struct omap_sr_class_data {
    pub enable: Option<unsafe extern "C" fn(*mut omap_sr) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut omap_sr, i32) -> i32>,
    pub configure: Option<unsafe extern "C" fn(*mut omap_sr) -> i32>,
    pub notify: Option<unsafe extern "C" fn(*mut omap_sr, u32) -> i32>,
    pub notify_flags: u8,
    pub class_type: u8,
}

#[repr(C)]
pub struct omap_sr_nvalue_table {
    pub efuse_offs: u32,
    pub nvalue: u32,
    pub errminlimit: u32,
    pub volt_nominal: usize,
}

#[repr(C)]
pub struct omap_sr_data {
    pub name: *const core::ffi::c_char,
    pub ip_type: i32,
    pub senp_mod: u32,
    pub senn_mod: u32,
    pub err_weight: u32,
    pub err_maxlimit: u32,
    pub accum_data: u32,
    pub senn_avgweight: u32,
    pub senp_avgweight: u32,
    pub nvalue_count: i32,
    pub nvalue_table: *mut omap_sr_nvalue_table,
    pub voltdm: *mut voltagedomain,
}

pub const OMAP_SR_NR: usize = sr_instance::OMAP_SR_NR as usize;
extern "C" { pub static mut omap_sr_pdata: [omap_sr_data; OMAP_SR_NR]; }

// CONFIG_POWER_AVS_OMAP declarations; the disabled branch supplies no-op APIs.
extern "C" {
    pub fn omap_sr_enable(voltdm: *mut voltagedomain);
    pub fn omap_sr_disable(voltdm: *mut voltagedomain);
    pub fn omap_sr_disable_reset_volt(voltdm: *mut voltagedomain);
    pub fn sr_enable(sr: *mut omap_sr, volt: usize) -> i32;
    pub fn sr_disable(sr: *mut omap_sr);
    pub fn sr_configure_errgen(sr: *mut omap_sr) -> i32;
    pub fn sr_disable_errgen(sr: *mut omap_sr) -> i32;
    pub fn sr_configure_minmax(sr: *mut omap_sr) -> i32;
    pub fn sr_register_class(class_data: *mut omap_sr_class_data) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
