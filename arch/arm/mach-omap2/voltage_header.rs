/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * OMAP Voltage Management Routines
 *
 * Author: Thara Gopinath	<thara@ti.com>
 *
 * Copyright (C) 2009 Texas Instruments, Inc.
 * Thara Gopinath <thara@ti.com>
 */

// C dependencies: <linux/err.h>, <linux/platform_data/voltage-omap.h>, "vc.h", and "vp.h".

pub const OMAP3_CLKSETUP: u32 = 0xff;
pub const OMAP3_VOLTOFFSET: u32 = 0xff;
pub const OMAP3_VOLTSETUP2: u32 = 0xff;

/**
 * struct omap_vfsm_instance - per-voltage manager FSM register/bitfield
 * data
 * @voltsetup_mask: SETUP_TIME* bitmask in the PRM_VOLTSETUP* register
 * @voltsetup_reg: register offset of PRM_VOLTSETUP from PRM base
 * @voltsetup_off_reg: register offset of PRM_VOLTSETUP_OFF from PRM base
 *
 * XXX What about VOLTOFFSET/VOLTCTRL?
 */
#[repr(C)]
pub struct omap_vfsm_instance {
    pub voltsetup_mask: u32,
    pub voltsetup_reg: u8,
    pub voltsetup_off_reg: u8,
}

/**
 * struct voltagedomain - omap voltage domain global structure.
 * @name: Name of the voltage domain which can be used as a unique identifier.
 * @scalable: Whether or not this voltage domain is scalable
 * @node: list_head linking all voltage domains
 * @vc: pointer to VC channel associated with this voltagedomain
 * @vp: pointer to VP associated with this voltagedomain
 * @read: read a VC/VP register
 * @write: write a VC/VP register
 * @read: read-modify-write a VC/VP register
 * @sys_clk: system clock name/frequency, used for various timing calculations
 * @scale: function used to scale the voltage of the voltagedomain
 * @nominal_volt: current nominal voltage for this voltage domain
 * @volt_data: voltage table having the distinct voltages supported
 *             by the domain and other associated per voltage data.
 */
#[repr(C)]
pub struct voltagedomain {
    pub name: *mut core::ffi::c_char,
    pub scalable: bool,
    pub node: list_head,
    pub vc: *mut omap_vc_channel,
    pub vfsm: *const omap_vfsm_instance,
    pub vp: *mut omap_vp_instance,
    pub pmic: *mut omap_voltdm_pmic,
    pub vp_param: *mut omap_vp_param,
    pub vc_param: *mut omap_vc_param,
    pub read: Option<unsafe extern "C" fn(offset: u8) -> u32>,
    pub write: Option<unsafe extern "C" fn(val: u32, offset: u8)>,
    pub rmw: Option<unsafe extern "C" fn(mask: u32, bits: u32, offset: u8) -> u32>,
    pub sys_clk: voltagedomain_sys_clk,
    pub scale: Option<unsafe extern "C" fn(voltdm: *mut voltagedomain, target_volt: c_ulong) -> i32>,
    pub nominal_volt: u32,
    pub volt_data: *mut omap_volt_data,
}

#[repr(C)]
pub union voltagedomain_sys_clk {
    pub name: *const core::ffi::c_char,
    pub rate: u32,
}

pub const OMAP3430_VP1_VLIMITTO_VDDMIN: u32 = 850000;
pub const OMAP3430_VP1_VLIMITTO_VDDMAX: u32 = 1425000;
pub const OMAP3430_VP2_VLIMITTO_VDDMIN: u32 = 900000;
pub const OMAP3430_VP2_VLIMITTO_VDDMAX: u32 = 1150000;
pub const OMAP3630_VP1_VLIMITTO_VDDMIN: u32 = 900000;
pub const OMAP3630_VP1_VLIMITTO_VDDMAX: u32 = 1350000;
pub const OMAP3630_VP2_VLIMITTO_VDDMIN: u32 = 900000;
pub const OMAP3630_VP2_VLIMITTO_VDDMAX: u32 = 1200000;
pub const OMAP4_VP_MPU_VLIMITTO_VDDMIN: u32 = 830000;
pub const OMAP4_VP_MPU_VLIMITTO_VDDMAX: u32 = 1410000;
pub const OMAP4_VP_IVA_VLIMITTO_VDDMIN: u32 = 830000;
pub const OMAP4_VP_IVA_VLIMITTO_VDDMAX: u32 = 1260000;
pub const OMAP4_VP_CORE_VLIMITTO_VDDMIN: u32 = 830000;
pub const OMAP4_VP_CORE_VLIMITTO_VDDMAX: u32 = 1200000;

#[repr(C)]
pub struct omap_voltdm_pmic {
    pub slew_rate: i32,
    pub step_size: i32,
    pub i2c_slave_addr: u16,
    pub volt_reg_addr: u16,
    pub cmd_reg_addr: u16,
    pub vp_erroroffset: u8,
    pub vp_vstepmin: u8,
    pub vp_vstepmax: u8,
    pub vddmin: u32,
    pub vddmax: u32,
    pub vp_timeout_us: u8,
    pub i2c_high_speed: bool,
    pub i2c_pad_load: u32,
    pub i2c_mcode: u8,
    pub vsel_to_uv: Option<unsafe extern "C" fn(vsel: u8) -> c_ulong>,
    pub uv_to_vsel: Option<unsafe extern "C" fn(uV: c_ulong) -> u8>,
}

#[repr(C)]
pub struct omap_vp_param { pub vddmax: u32, pub vddmin: u32 }

#[repr(C)]
pub struct omap_vc_param { pub on: u32, pub onlp: u32, pub ret: u32, pub off: u32 }

extern "C" {
    pub fn omap_voltage_get_volttable(voltdm: *mut voltagedomain, volt_data: *mut *mut omap_volt_data);
    pub fn omap_voltage_get_voltdata(voltdm: *mut voltagedomain, volt: c_ulong) -> *mut omap_volt_data;
    pub fn omap_voltage_register_pmic(voltdm: *mut voltagedomain, pmic: *mut omap_voltdm_pmic) -> i32;
    pub fn omap_voltage_late_init() -> i32;
    pub fn omap2xxx_voltagedomains_init();
    pub fn omap3xxx_voltagedomains_init();
    pub fn omap44xx_voltagedomains_init();
    pub fn omap54xx_voltagedomains_init();
    pub fn voltdm_lookup(name: *const core::ffi::c_char) -> *mut voltagedomain;
    pub fn voltdm_init(voltdm_list: *mut *mut voltagedomain);
    pub fn voltdm_reset(voltdm: *mut voltagedomain);
    pub fn voltdm_get_voltage(voltdm: *mut voltagedomain) -> c_ulong;
}

// Types supplied by the included kernel headers.
pub type c_ulong = core::ffi::c_ulong;
extern "C" {
    pub type list_head;
    pub type omap_vc_channel;
    pub type omap_vp_instance;
    pub type omap_volt_data;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
