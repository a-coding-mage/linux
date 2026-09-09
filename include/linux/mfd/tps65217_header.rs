/* SPDX-License-Identifier: GPL-2.0-only */
/* Translated from linux/mfd/tps65217.h. */

// C dependencies supplied by other translation units.
pub enum i2c_client {}
pub enum regulator_init_data {}
pub enum device_node {}
pub enum device {}
pub enum regulator_desc {}
pub enum regmap {}
pub enum irq_domain {}
pub enum mutex {}

pub const TPS65217: u8 = 0xF0;
pub const TPS65217_I2C_ID: u8 = 0x24;

pub const TPS65217_REG_CHIPID: u8 = 0x00;
pub const TPS65217_REG_PPATH: u8 = 0x01;
pub const TPS65217_REG_INT: u8 = 0x02;
pub const TPS65217_REG_CHGCONFIG0: u8 = 0x03;
pub const TPS65217_REG_CHGCONFIG1: u8 = 0x04;
pub const TPS65217_REG_CHGCONFIG2: u8 = 0x05;
pub const TPS65217_REG_CHGCONFIG3: u8 = 0x06;
pub const TPS65217_REG_WLEDCTRL1: u8 = 0x07;
pub const TPS65217_REG_WLEDCTRL2: u8 = 0x08;
pub const TPS65217_REG_MUXCTRL: u8 = 0x09;
pub const TPS65217_REG_STATUS: u8 = 0x0A;
pub const TPS65217_REG_PASSWORD: u8 = 0x0B;
pub const TPS65217_REG_PGOOD: u8 = 0x0C;
pub const TPS65217_REG_DEFPG: u8 = 0x0D;
pub const TPS65217_REG_DEFDCDC1: u8 = 0x0E;
pub const TPS65217_REG_DEFDCDC2: u8 = 0x0F;
pub const TPS65217_REG_DEFDCDC3: u8 = 0x10;
pub const TPS65217_REG_DEFSLEW: u8 = 0x11;
pub const TPS65217_REG_DEFLDO1: u8 = 0x12;
pub const TPS65217_REG_DEFLDO2: u8 = 0x13;
pub const TPS65217_REG_DEFLS1: u8 = 0x14;
pub const TPS65217_REG_DEFLS2: u8 = 0x15;
pub const TPS65217_REG_ENABLE: u8 = 0x16;
pub const TPS65217_REG_DEFUVLO: u8 = 0x18;
pub const TPS65217_REG_SEQ1: u8 = 0x19;
pub const TPS65217_REG_SEQ2: u8 = 0x1A;
pub const TPS65217_REG_SEQ3: u8 = 0x1B;
pub const TPS65217_REG_SEQ4: u8 = 0x1C;
pub const TPS65217_REG_SEQ5: u8 = 0x1D;
pub const TPS65217_REG_SEQ6: u8 = 0x1E;
pub const TPS65217_REG_MAX: u8 = TPS65217_REG_SEQ6;

pub const TPS65217_CHIPID_CHIP_MASK: u8 = 0xF0;
pub const TPS65217_CHIPID_REV_MASK: u8 = 0x0F;
pub const TPS65217_PPATH_ACSINK_ENABLE: u8 = 1 << 7;
pub const TPS65217_PPATH_USBSINK_ENABLE: u8 = 1 << 6;
pub const TPS65217_PPATH_AC_PW_ENABLE: u8 = 1 << 5;
pub const TPS65217_PPATH_USB_PW_ENABLE: u8 = 1 << 4;
pub const TPS65217_PPATH_AC_CURRENT_MASK: u8 = 0x0C;
pub const TPS65217_PPATH_USB_CURRENT_MASK: u8 = 0x03;
pub const TPS65217_INT_PBM: u8 = 1 << 6;
pub const TPS65217_INT_ACM: u8 = 1 << 5;
pub const TPS65217_INT_USBM: u8 = 1 << 4;
pub const TPS65217_INT_PBI: u8 = 1 << 2;
pub const TPS65217_INT_ACI: u8 = 1 << 1;
pub const TPS65217_INT_USBI: u8 = 1;
pub const TPS65217_INT_SHIFT: u8 = 4;
pub const TPS65217_INT_MASK: u8 = TPS65217_INT_PBM | TPS65217_INT_ACM | TPS65217_INT_USBM;

pub const TPS65217_CHGCONFIG0_TREG: u8 = 1 << 7;
pub const TPS65217_CHGCONFIG0_DPPM: u8 = 1 << 6;
pub const TPS65217_CHGCONFIG0_TSUSP: u8 = 1 << 5;
pub const TPS65217_CHGCONFIG0_TERMI: u8 = 1 << 4;
pub const TPS65217_CHGCONFIG0_ACTIVE: u8 = 1 << 3;
pub const TPS65217_CHGCONFIG0_CHGTOUT: u8 = 1 << 2;
pub const TPS65217_CHGCONFIG0_PCHGTOUT: u8 = 1 << 1;
pub const TPS65217_CHGCONFIG0_BATTEMP: u8 = 1;
pub const TPS65217_CHGCONFIG1_TMR_MASK: u8 = 0xC0;
pub const TPS65217_CHGCONFIG1_TMR_ENABLE: u8 = 1 << 5;
pub const TPS65217_CHGCONFIG1_NTC_TYPE: u8 = 1 << 4;
pub const TPS65217_CHGCONFIG1_RESET: u8 = 1 << 3;
pub const TPS65217_CHGCONFIG1_TERM: u8 = 1 << 2;
pub const TPS65217_CHGCONFIG1_SUSP: u8 = 1 << 1;
pub const TPS65217_CHGCONFIG1_CHG_EN: u8 = 1;
pub const TPS65217_CHGCONFIG2_DYNTMR: u8 = 1 << 7;
pub const TPS65217_CHGCONFIG2_VPREGHG: u8 = 1 << 6;
pub const TPS65217_CHGCONFIG2_VOREG_MASK: u8 = 0x30;
pub const TPS65217_CHGCONFIG3_ICHRG_MASK: u8 = 0xC0;
pub const TPS65217_CHGCONFIG3_DPPMTH_MASK: u8 = 0x30;
pub const TPS65217_CHGCONFIG2_PCHRGT: u8 = 1 << 3;
pub const TPS65217_CHGCONFIG2_TERMIF: u8 = 0x06;
pub const TPS65217_CHGCONFIG2_TRANGE: u8 = 1;
pub const TPS65217_WLEDCTRL1_ISINK_ENABLE: u8 = 1 << 3;
pub const TPS65217_WLEDCTRL1_ISEL: u8 = 1 << 2;
pub const TPS65217_WLEDCTRL1_FDIM_MASK: u8 = 0x03;
pub const TPS65217_WLEDCTRL2_DUTY_MASK: u8 = 0x7F;
pub const TPS65217_MUXCTRL_MUX_MASK: u8 = 0x07;
pub const TPS65217_STATUS_OFF: u8 = 1 << 7;
pub const TPS65217_STATUS_ACPWR: u8 = 1 << 3;
pub const TPS65217_STATUS_USBPWR: u8 = 1 << 2;
pub const TPS65217_STATUS_PB: u8 = 1;
pub const TPS65217_PASSWORD_REGS_UNLOCK: u8 = 0x7D;
pub const TPS65217_PGOOD_LDO3_PG: u8 = 1 << 6;
pub const TPS65217_PGOOD_LDO4_PG: u8 = 1 << 5;
pub const TPS65217_PGOOD_DC1_PG: u8 = 1 << 4;
pub const TPS65217_PGOOD_DC2_PG: u8 = 1 << 3;
pub const TPS65217_PGOOD_DC3_PG: u8 = 1 << 2;
pub const TPS65217_PGOOD_LDO1_PG: u8 = 1 << 1;
pub const TPS65217_PGOOD_LDO2_PG: u8 = 1;
pub const TPS65217_DEFPG_LDO1PGM: u8 = 1 << 3;
pub const TPS65217_DEFPG_LDO2PGM: u8 = 1 << 2;
pub const TPS65217_DEFPG_PGDLY_MASK: u8 = 3;
pub const TPS65217_DEFDCDCX_XADJX: u8 = 1 << 7;
pub const TPS65217_DEFDCDCX_DCDC_MASK: u8 = 0x3F;
pub const TPS65217_DEFSLEW_GO: u8 = 1 << 7;
pub const TPS65217_DEFSLEW_GODSBL: u8 = 1 << 6;
pub const TPS65217_DEFSLEW_PFM_EN1: u8 = 1 << 5;
pub const TPS65217_DEFSLEW_PFM_EN2: u8 = 1 << 4;
pub const TPS65217_DEFSLEW_PFM_EN3: u8 = 1 << 3;
pub const TPS65217_DEFSLEW_SLEW_MASK: u8 = 7;
pub const TPS65217_DEFLDO1_LDO1_MASK: u8 = 0x0F;
pub const TPS65217_DEFLDO2_TRACK: u8 = 1 << 6;
pub const TPS65217_DEFLDO2_LDO2_MASK: u8 = 0x3F;
pub const TPS65217_DEFLDO3_LDO3_EN: u8 = 1 << 5;
pub const TPS65217_DEFLDO3_LDO3_MASK: u8 = 0x1F;
pub const TPS65217_DEFLDO4_LDO4_EN: u8 = 1 << 5;
pub const TPS65217_DEFLDO4_LDO4_MASK: u8 = 0x1F;
pub const TPS65217_ENABLE_LS1_EN: u8 = 1 << 6;
pub const TPS65217_ENABLE_LS2_EN: u8 = 1 << 5;
pub const TPS65217_ENABLE_DC1_EN: u8 = 1 << 4;
pub const TPS65217_ENABLE_DC2_EN: u8 = 1 << 3;
pub const TPS65217_ENABLE_DC3_EN: u8 = 1 << 2;
pub const TPS65217_ENABLE_LDO1_EN: u8 = 1 << 1;
pub const TPS65217_ENABLE_LDO2_EN: u8 = 1;
pub const TPS65217_DEFUVLO_UVLOHYS: u8 = 1 << 2;
pub const TPS65217_DEFUVLO_UVLO_MASK: u8 = 3;
pub const TPS65217_SEQ1_DC1_SEQ_MASK: u8 = 0xF0;
pub const TPS65217_SEQ1_DC2_SEQ_MASK: u8 = 0x0F;
pub const TPS65217_SEQ2_DC3_SEQ_MASK: u8 = 0xF0;
pub const TPS65217_SEQ2_LDO1_SEQ_MASK: u8 = 0x0F;
pub const TPS65217_SEQ3_LDO2_SEQ_MASK: u8 = 0xF0;
pub const TPS65217_SEQ3_LDO3_SEQ_MASK: u8 = 0x0F;
pub const TPS65217_SEQ4_LDO4_SEQ_MASK: u8 = 0xF0;
pub const TPS65217_SEQ5_DLY1_MASK: u8 = 0xC0;
pub const TPS65217_SEQ5_DLY2_MASK: u8 = 0x30;
pub const TPS65217_SEQ5_DLY3_MASK: u8 = 0x0C;
pub const TPS65217_SEQ5_DLY4_MASK: u8 = 3;
pub const TPS65217_SEQ6_DLY5_MASK: u8 = 0xC0;
pub const TPS65217_SEQ6_DLY6_MASK: u8 = 0x30;
pub const TPS65217_SEQ6_SEQUP: u8 = 1 << 2;
pub const TPS65217_SEQ6_SEQDWN: u8 = 1 << 1;
pub const TPS65217_SEQ6_INSTDWN: u8 = 1;
pub const TPS65217_MAX_REGISTER: u8 = 0x1E;
pub const TPS65217_PROTECT_NONE: u8 = 0;
pub const TPS65217_PROTECT_L1: u8 = 1;
pub const TPS65217_PROTECT_L2: u8 = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum tps65217_regulator_id { TPS65217_DCDC_1, TPS65217_DCDC_2, TPS65217_DCDC_3, TPS65217_LDO_1, TPS65217_LDO_2, TPS65217_LDO_3, TPS65217_LDO_4 }
pub const TPS65217_MAX_REG_ID: tps65217_regulator_id = tps65217_regulator_id::TPS65217_LDO_4;
pub const TPS65217_NUM_DCDC: usize = 3;
pub const TPS65217_NUM_LDO: usize = 4;
pub const TPS65217_NUM_REGULATOR: usize = TPS65217_NUM_DCDC + TPS65217_NUM_LDO;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum tps65217_bl_isel { TPS65217_BL_ISET1 = 1, TPS65217_BL_ISET2 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum tps65217_bl_fdim { TPS65217_BL_FDIM_100HZ, TPS65217_BL_FDIM_200HZ, TPS65217_BL_FDIM_500HZ, TPS65217_BL_FDIM_1000HZ }

#[repr(C)]
pub struct tps65217_bl_pdata { pub isel: tps65217_bl_isel, pub fdim: tps65217_bl_fdim, pub dft_brightness: i32 }

pub const TPS65217_IRQ_USB: i32 = 0;
pub const TPS65217_IRQ_AC: i32 = 1;
pub const TPS65217_IRQ_PB: i32 = 2;
pub const TPS65217_NUM_IRQ: i32 = 3;

#[repr(C)]
pub struct tps65217_board {
    pub tps65217_init_data: [*mut regulator_init_data; TPS65217_NUM_REGULATOR],
    pub of_node: [*mut device_node; TPS65217_NUM_REGULATOR],
    pub bl_pdata: *mut tps65217_bl_pdata,
}

#[repr(C)]
pub struct tps65217 {
    pub dev: *mut device,
    pub pdata: *mut tps65217_board,
    pub desc: [regulator_desc; TPS65217_NUM_REGULATOR],
    pub regmap: *mut regmap,
    pub strobes: *mut u8,
    pub irq_domain: *mut irq_domain,
    pub irq_lock: mutex,
    pub irq_mask: u8,
    pub irq: i32,
}

extern "C" {
    pub fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    pub fn tps65217_reg_read(tps: *mut tps65217, reg: u32, val: *mut u32) -> i32;
    pub fn tps65217_reg_write(tps: *mut tps65217, reg: u32, val: u32, level: u32) -> i32;
    pub fn tps65217_set_bits(tps: *mut tps65217, reg: u32, mask: u32, val: u32, level: u32) -> i32;
    pub fn tps65217_clear_bits(tps: *mut tps65217, reg: u32, mask: u32, level: u32) -> i32;
}

#[inline]
pub unsafe fn dev_to_tps65217(dev: *mut device) -> *mut tps65217 {
    dev_get_drvdata(dev) as *mut tps65217
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
