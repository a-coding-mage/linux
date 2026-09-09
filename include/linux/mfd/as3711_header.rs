/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * AS3711 PMIC MFC driver header
 *
 * Copyright (C) 2012 Renesas Electronics Corporation
 * Author: Guennadi Liakhovetski, <g.liakhovetski@gmx.de>
 */

/* Client data */

/* Register addresses */
pub const AS3711_SD_1_VOLTAGE: u32 = 0; /* Digital Step-Down */
pub const AS3711_SD_2_VOLTAGE: u32 = 1;
pub const AS3711_SD_3_VOLTAGE: u32 = 2;
pub const AS3711_SD_4_VOLTAGE: u32 = 3;
pub const AS3711_LDO_1_VOLTAGE: u32 = 4; /* Analog LDO */
pub const AS3711_LDO_2_VOLTAGE: u32 = 5;
pub const AS3711_LDO_3_VOLTAGE: u32 = 6; /* Digital LDO */
pub const AS3711_LDO_4_VOLTAGE: u32 = 7;
pub const AS3711_LDO_5_VOLTAGE: u32 = 8;
pub const AS3711_LDO_6_VOLTAGE: u32 = 9;
pub const AS3711_LDO_7_VOLTAGE: u32 = 0xa;
pub const AS3711_LDO_8_VOLTAGE: u32 = 0xb;
pub const AS3711_SD_CONTROL: u32 = 0x10;
pub const AS3711_GPIO_SIGNAL_OUT: u32 = 0x20;
pub const AS3711_GPIO_SIGNAL_IN: u32 = 0x21;
pub const AS3711_SD_CONTROL_1: u32 = 0x30;
pub const AS3711_SD_CONTROL_2: u32 = 0x31;
pub const AS3711_CURR_CONTROL: u32 = 0x40;
pub const AS3711_CURR1_VALUE: u32 = 0x43;
pub const AS3711_CURR2_VALUE: u32 = 0x44;
pub const AS3711_CURR3_VALUE: u32 = 0x45;
pub const AS3711_STEPUP_CONTROL_1: u32 = 0x50;
pub const AS3711_STEPUP_CONTROL_2: u32 = 0x51;
pub const AS3711_STEPUP_CONTROL_4: u32 = 0x53;
pub const AS3711_STEPUP_CONTROL_5: u32 = 0x54;
pub const AS3711_REG_STATUS: u32 = 0x73;
pub const AS3711_INTERRUPT_STATUS_1: u32 = 0x77;
pub const AS3711_INTERRUPT_STATUS_2: u32 = 0x78;
pub const AS3711_INTERRUPT_STATUS_3: u32 = 0x79;
pub const AS3711_CHARGER_STATUS_1: u32 = 0x86;
pub const AS3711_CHARGER_STATUS_2: u32 = 0x87;
pub const AS3711_ASIC_ID_1: u32 = 0x90;
pub const AS3711_ASIC_ID_2: u32 = 0x91;

pub const AS3711_MAX_REG: u32 = AS3711_ASIC_ID_2;
pub const AS3711_NUM_REGS: u32 = AS3711_MAX_REG + 1;

/* Regulators */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum As3711Regulator {
    AS3711_REGULATOR_SD_1,
    AS3711_REGULATOR_SD_2,
    AS3711_REGULATOR_SD_3,
    AS3711_REGULATOR_SD_4,
    AS3711_REGULATOR_LDO_1,
    AS3711_REGULATOR_LDO_2,
    AS3711_REGULATOR_LDO_3,
    AS3711_REGULATOR_LDO_4,
    AS3711_REGULATOR_LDO_5,
    AS3711_REGULATOR_LDO_6,
    AS3711_REGULATOR_LDO_7,
    AS3711_REGULATOR_LDO_8,
    AS3711_REGULATOR_MAX,
}

#[repr(C)]
pub struct Device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct RegulatorInitData {
    _private: [u8; 0],
}

#[repr(C)]
pub struct As3711 {
    pub dev: *mut Device,
    pub regmap: *mut Regmap,
}

pub const AS3711_MAX_STEPDOWN: usize = 4;
pub const AS3711_MAX_STEPUP: usize = 2;
pub const AS3711_MAX_LDO: usize = 8;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum As3711Su2Feedback {
    AS3711_SU2_VOLTAGE,
    AS3711_SU2_CURR1,
    AS3711_SU2_CURR2,
    AS3711_SU2_CURR3,
    AS3711_SU2_CURR_AUTO,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum As3711Su2Fbprot {
    AS3711_SU2_LX_SD4,
    AS3711_SU2_GPIO2,
    AS3711_SU2_GPIO3,
    AS3711_SU2_GPIO4,
}

/* Platform data */

#[repr(C)]
pub struct As3711RegulatorPdata {
    pub init_data: [*mut RegulatorInitData; As3711Regulator::AS3711_REGULATOR_MAX as usize],
}

#[repr(C)]
pub struct As3711BlPdata {
    pub su1_fb: bool,
    pub su1_max_uA: i32,
    pub su2_fb: bool,
    pub su2_max_uA: i32,
    pub su2_feedback: As3711Su2Feedback,
    pub su2_fbprot: As3711Su2Fbprot,
    pub su2_auto_curr1: bool,
    pub su2_auto_curr2: bool,
    pub su2_auto_curr3: bool,
}

#[repr(C)]
pub struct As3711PlatformData {
    pub regulator: As3711RegulatorPdata,
    pub backlight: As3711BlPdata,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
