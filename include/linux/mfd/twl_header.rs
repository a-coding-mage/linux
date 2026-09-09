/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * twl4030.h - header for TWL4030 PM and audio CODEC device
 * Copyright (C) 2005-2006 Texas Instruments, Inc.
 * Based on tlv320aic23.c: Copyright (c) by Kai Svahn <kai.svahn@nokia.com>
 */

// C dependencies: linux/types.h and linux/input/matrix_keypad.h.
// Their supplied Rust representations are external dependencies.

pub type U8 = u8;
pub type U16 = u16;
pub type U32 = u32;

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TwlModuleIds {
    TwlModuleUsb,
    TwlModulePih,
    TwlModuleMainCharge,
    TwlModulePmMaster,
    TwlModulePmReceiver,
    TwlModuleRtc,
    TwlModulePwm,
    TwlModuleLed,
    TwlModuleSecuredReg,
    TwlModuleLast,
}
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Twl4030ModuleIds {
    Twl4030ModuleAudioVoice = TwlModuleIds::TwlModuleLast as u32,
    Twl4030ModuleGpio,
    Twl4030ModuleIntbr,
    Twl4030ModuleTest,
    Twl4030ModuleKeypad,
    Twl4030ModuleMadc,
    Twl4030ModuleInterrupts,
    Twl4030ModulePrecharge,
    Twl4030ModuleBackup,
    Twl4030ModuleInt,
    Twl5031ModuleAccessory,
    Twl5031ModuleInterrupts,
    Twl4030ModuleLast,
}
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Twl6030ModuleIds {
    Twl6030ModuleId0 = TwlModuleIds::TwlModuleLast as u32,
    Twl6030ModuleId1,
    Twl6030ModuleId2,
    Twl6030ModuleGpadc,
    Twl6030ModuleGasgauge,
    Twl6032ModuleCharge,
    Twl6030ModuleLast,
}

macro_rules! c_consts { ($($name:ident = $value:expr),* $(,)?) => { $(pub const $name: u32 = $value;)* }; }
c_consts! {
    TWL4030_MODULE_LED = TwlModuleIds::TwlModuleLed as u32,
    GPIO_INTR_OFFSET=0, KEYPAD_INTR_OFFSET=1, BCI_INTR_OFFSET=2, MADC_INTR_OFFSET=3,
    USB_INTR_OFFSET=4, CHARGERFAULT_INTR_OFFSET=5, BCI_PRES_INTR_OFFSET=9,
    USB_PRES_INTR_OFFSET=10, RTC_INTR_OFFSET=11, PWR_INTR_OFFSET=0, HOTDIE_INTR_OFFSET=12,
    SMPSLDO_INTR_OFFSET=13, BATDETECT_INTR_OFFSET=14, SIMDETECT_INTR_OFFSET=15,
    MMCDETECT_INTR_OFFSET=16, GASGAUGE_INTR_OFFSET=17, USBOTG_INTR_OFFSET=4,
    CHARGER_INTR_OFFSET=2, RSV_INTR_OFFSET=0,
    REG_INT_STS_A=0x00, REG_INT_STS_B=0x01, REG_INT_STS_C=0x02,
    REG_INT_MSK_LINE_A=0x03, REG_INT_MSK_LINE_B=0x04, REG_INT_MSK_LINE_C=0x05,
    REG_INT_MSK_STS_A=0x06, REG_INT_MSK_STS_B=0x07, REG_INT_MSK_STS_C=0x08,
    TWL6030_PWR_INT_MASK=0x07, TWL6030_RTC_INT_MASK=0x18, TWL6030_HOTDIE_INT_MASK=0x20,
    TWL6030_SMPSLDOA_INT_MASK=0xC0, TWL6030_SMPSLDOB_INT_MASK=0x01,
    TWL6030_BATDETECT_INT_MASK=0x02, TWL6030_SIMDETECT_INT_MASK=0x04,
    TWL6030_MMCDETECT_INT_MASK=0x08, TWL6030_GPADC_INT_MASK=0x60,
    TWL6030_GASGAUGE_INT_MASK=0x80, TWL6030_USBOTG_INT_MASK=0x0F,
    TWL6030_CHARGER_CTRL_INT_MASK=0x10, TWL6030_CHARGER_FAULT_INT_MASK=0x60,
    TWL6030_MMCCTRL=0xEE, VMMC_AUTO_OFF=1<<3, SW_FC=1<<2, STS_MMC=1,
    TWL6030_CFG_INPUT_PUPD3=0xF2, MMC_PU=1<<3, MMC_PD=1<<2,
    TWL_SIL_5030=0x09002F, TWL5030_REV_1_0=0x00, TWL5030_REV_1_1=0x10,
    TWL5030_REV_1_2=0x30, TWL4030_CLASS_ID=0x4030, TWL6030_CLASS_ID=0x6030,
    TWL4030_GPIO_MAX=18, TWL_EEPROM_R_UNLOCK=0x49,
    TWL4030_KEYPAD_KEYP_ISR1=0x11, TWL4030_KEYPAD_KEYP_IMR1=0x12,
    TWL4030_KEYPAD_KEYP_ISR2=0x13, TWL4030_KEYPAD_KEYP_IMR2=0x14,
    TWL4030_KEYPAD_KEYP_SIR=0x15, TWL4030_KEYPAD_KEYP_EDR=0x16,
    TWL4030_KEYPAD_KEYP_SIH_CTRL=0x17, TWL4030_MADC_ISR1=0x61,
    TWL4030_MADC_IMR1=0x62, TWL4030_MADC_ISR2=0x63, TWL4030_MADC_IMR2=0x64,
    TWL4030_MADC_SIR=0x65, TWL4030_MADC_EDR=0x66, TWL4030_MADC_SIH_CTRL=0x67,
    TWL6030_PHOENIX_DEV_ON=0x06, DEV_GRP_NULL=0, DEV_GRP_P1=1, DEV_GRP_P2=2, DEV_GRP_P3=4,
    RES_GRP_RES=0, RES_GRP_PP=1, RES_GRP_RC=2, RES_GRP_PP_RC=3, RES_GRP_PR=4,
    RES_GRP_PP_PR=5, RES_GRP_RC_PR=6, RES_GRP_ALL=7, RES_TYPE2_R0=0, RES_TYPE2_R1=1,
    RES_TYPE2_R2=2, RES_TYPE_R0=0, RES_TYPE_ALL=7, RES_STATE_WRST=0xF,
    RES_STATE_ACTIVE=0xE, RES_STATE_SLEEP=0x8, RES_STATE_OFF=0,
    RES_VAUX1=1, RES_VAUX2=2, RES_VAUX3=3, RES_VAUX4=4, RES_VMMC1=5, RES_VMMC2=6,
    RES_VPLL1=7, RES_VPLL2=8, RES_VSIM=9, RES_VDAC=10, RES_VINTANA1=11, RES_VINTANA2=12,
    RES_VINTDIG=13, RES_VIO=14, RES_VDD1=15, RES_VDD2=16, RES_VUSB_1V5=17,
    RES_VUSB_1V8=18, RES_VUSB_3V1=19, RES_VUSBCP=20, RES_REGEN=21, RES_NRES_PWRON=22,
    RES_CLKEN=23, RES_SYSEN=24, RES_HFCLKOUT=25, RES_32KCLKOUT=26, RES_RESET=27,
    RES_MAIN_REF=28, TOTAL_RESOURCES=28
}

#[inline] pub const fn twl_sil_type(rev: u32) -> u32 { rev & 0x00ffffff }
#[inline] pub const fn twl_sil_rev(rev: u32) -> u32 { rev >> 24 }
#[inline] pub const fn msg_broadcast(devgrp:u32, grp:u32, typ:u32, type2:u32, state:u32)->u32 { (devgrp<<13)|(1<<12)|(grp<<9)|(type2<<7)|(typ<<4)|state }
#[inline] pub const fn msg_singular(devgrp:u32, id:u32, state:u32)->u32 { (devgrp<<13)|(id<<4)|state }
#[inline] pub const fn msg_broadcast_all(devgrp:u32, state:u32)->u32 { (devgrp<<5)|state }
// C aliases for MSG_BROADCAST_ALL.
#[inline] pub const fn msg_broadcast_ref(devgrp:u32,state:u32)->u32 { msg_broadcast_all(devgrp,state) }
#[inline] pub const fn msg_broadcast_prov(devgrp:u32,state:u32)->u32 { msg_broadcast_all(devgrp,state) }
#[inline] pub const fn msg_broadcast_clk_rst(devgrp:u32,state:u32)->u32 { msg_broadcast_all(devgrp,state) }

// GPIO, interrupt, power, and regulator register offsets.
c_consts! {
    TWL4030_SIH_CTRL_EXCLEN_MASK=1<<0, TWL4030_SIH_CTRL_PENDDIS_MASK=1<<1, TWL4030_SIH_CTRL_COR_MASK=1<<2,
    REG_GPIODATAIN1=0x0,REG_GPIODATAIN2=0x1,REG_GPIODATAIN3=0x2,REG_GPIODATADIR1=0x3,REG_GPIODATADIR2=0x4,REG_GPIODATADIR3=0x5,
    REG_GPIODATAOUT1=0x6,REG_GPIODATAOUT2=0x7,REG_GPIODATAOUT3=0x8,REG_CLEARGPIODATAOUT1=0x9,REG_CLEARGPIODATAOUT2=0xA,REG_CLEARGPIODATAOUT3=0xB,
    REG_SETGPIODATAOUT1=0xC,REG_SETGPIODATAOUT2=0xD,REG_SETGPIODATAOUT3=0xE,REG_GPIO_DEBEN1=0xF,REG_GPIO_DEBEN2=0x10,REG_GPIO_DEBEN3=0x11,REG_GPIO_CTRL=0x12,
    REG_GPIOPUPDCTR1=0x13,REG_GPIOPUPDCTR2=0x14,REG_GPIOPUPDCTR3=0x15,REG_GPIOPUPDCTR4=0x16,REG_GPIOPUPDCTR5=0x17,
    REG_GPIO_ISR1A=0x19,REG_GPIO_ISR2A=0x1A,REG_GPIO_ISR3A=0x1B,REG_GPIO_IMR1A=0x1C,REG_GPIO_IMR2A=0x1D,REG_GPIO_IMR3A=0x1E,
    REG_GPIO_ISR1B=0x1F,REG_GPIO_ISR2B=0x20,REG_GPIO_ISR3B=0x21,REG_GPIO_IMR1B=0x22,REG_GPIO_IMR2B=0x23,REG_GPIO_IMR3B=0x24,
    REG_GPIO_EDR1=0x28,REG_GPIO_EDR2=0x29,REG_GPIO_EDR3=0x2A,REG_GPIO_EDR4=0x2B,REG_GPIO_EDR5=0x2C,REG_GPIO_SIH_CTRL=0x2D,
    REG_IDCODE_7_0=0,REG_IDCODE_15_8=1,REG_IDCODE_16_23=2,REG_IDCODE_31_24=3,REG_GPPUPDCTR1=0x0F,REG_UNLOCK_TEST_REG=0x12,
    I2C_SCL_CTRL_PU=1<<0,I2C_SDA_CTRL_PU=1<<2,SR_I2C_SCL_CTRL_PU=1<<4,SR_I2C_SDA_CTRL_PU=1<<6,
    TWL4030_INTERRUPTS_BCIISR1A=0,TWL4030_INTERRUPTS_BCIISR2A=1,TWL4030_INTERRUPTS_BCIIMR1A=2,TWL4030_INTERRUPTS_BCIIMR2A=3,
    TWL4030_INTERRUPTS_BCIISR1B=4,TWL4030_INTERRUPTS_BCIISR2B=5,TWL4030_INTERRUPTS_BCIIMR1B=6,TWL4030_INTERRUPTS_BCIIMR2B=7,
    TWL4030_INTERRUPTS_BCISIR1=8,TWL4030_INTERRUPTS_BCISIR2=9,TWL4030_INTERRUPTS_BCIEDR1=0xa,TWL4030_INTERRUPTS_BCIEDR2=0xb,TWL4030_INTERRUPTS_BCIEDR3=0xc,TWL4030_INTERRUPTS_BCISIHCTRL=0xd,
    TWL4030_INT_PWR_ISR1=0,TWL4030_INT_PWR_IMR1=1,TWL4030_INT_PWR_ISR2=2,TWL4030_INT_PWR_IMR2=3,TWL4030_INT_PWR_SIR=4,TWL4030_INT_PWR_EDR1=5,TWL4030_INT_PWR_EDR2=6,TWL4030_INT_PWR_SIH_CTRL=7,
    TWL4030_VDAC_DEV_GRP=0x3B,TWL4030_VDAC_DEDICATED=0x3E,TWL4030_VAUX1_DEV_GRP=0x17,TWL4030_VAUX1_DEDICATED=0x1A,TWL4030_VAUX2_DEV_GRP=0x1B,TWL4030_VAUX2_DEDICATED=0x1E,TWL4030_VAUX3_DEV_GRP=0x1F,TWL4030_VAUX3_DEDICATED=0x22,
    TWL4030_REG_VDD1=0,TWL4030_REG_VDD2=1,TWL4030_REG_VIO=2,TWL4030_REG_VDAC=3,TWL4030_REG_VPLL1=4,TWL4030_REG_VPLL2=5,TWL4030_REG_VMMC1=6,TWL4030_REG_VMMC2=7,TWL4030_REG_VSIM=8,TWL4030_REG_VAUX1=9,TWL4030_REG_VAUX2_4030=10,TWL4030_REG_VAUX2=11,TWL4030_REG_VAUX3=12,TWL4030_REG_VAUX4=13,
    TWL4030_REG_VINTANA1=14,TWL4030_REG_VINTANA2=15,TWL4030_REG_VINTDIG=16,TWL4030_REG_VUSB1V5=17,TWL4030_REG_VUSB1V8=18,TWL4030_REG_VUSB3V1=19,
    TWL6030_REG_VDD1=30,TWL6030_REG_VDD2=31,TWL6030_REG_VDD3=32,TWL6030_REG_VMEM=33,TWL6030_REG_V2V1=34,TWL6030_REG_V1V29=35,TWL6030_REG_V1V8=36,TWL6030_REG_VAUX1_6030=37,TWL6030_REG_VAUX2_6030=38,TWL6030_REG_VAUX3_6030=39,TWL6030_REG_VMMC=40,TWL6030_REG_VPP=41,TWL6030_REG_VUSIM=42,TWL6030_REG_VANA=43,TWL6030_REG_VCXIO=44,TWL6030_REG_VDAC=45,TWL6030_REG_VUSB=46,TWL6030_REG_VRTC=47,TWL6030_REG_CLK32KG=48,
    TWL6032_REG_LDO2=49,TWL6032_REG_LDO4=50,TWL6032_REG_LDO3=51,TWL6032_REG_LDO5=52,TWL6032_REG_LDO1=53,TWL6032_REG_LDO7=54,TWL6032_REG_LDO6=55,TWL6032_REG_LDOLN=56,TWL6032_REG_LDOUSB=57,TWL6032_REG_SMPS3=58,TWL6032_REG_SMPS4=59,TWL6032_REG_VIO=60
}

extern "C" {
    pub fn twl_rev() -> u32;
    pub fn twl_set_regcache_bypass(mod_no: u8, enable: bool) -> i32;
    pub fn twl_i2c_write(mod_no: u8, value: *mut u8, reg: u8, num_bytes: u32) -> i32;
    pub fn twl_i2c_read(mod_no: u8, value: *mut u8, reg: u8, num_bytes: u32) -> i32;
    pub fn twl_get_type() -> i32; pub fn twl_get_version() -> i32; pub fn twl_get_hfclk_rate() -> i32;
    pub fn twl6030_interrupt_unmask(bit_mask:u8, offset:u8)->i32;
    pub fn twl6030_interrupt_mask(bit_mask:u8, offset:u8)->i32;
    pub fn twl4030_remove_script(flags:u8)->i32; pub fn twl4030_power_off();
}
#[inline] pub unsafe fn twl_i2c_write_u8(m:u8,v:u8,r:u8)->i32 { twl_i2c_write(m,&v as *const u8 as *mut u8,r,1) }
#[inline] pub unsafe fn twl_i2c_read_u8(m:u8,v:*mut u8,r:u8)->i32 { twl_i2c_read(m,v,r,1) }
#[inline] pub unsafe fn twl_i2c_write_u16(m:u8,v:u16,r:u8)->i32 { let mut x=v.to_le(); twl_i2c_write(m,&mut x as *mut u16 as *mut u8,r,2) }
#[inline] pub unsafe fn twl_i2c_read_u16(m:u8,v:*mut u16,r:u8)->i32 { let mut x=0u16; let ret=twl_i2c_read(m,&mut x as *mut u16 as *mut u8,r,2); *v=x.to_le(); ret }

#[repr(C)] pub struct Twl4030ClockInitData { pub ck32k_lowpwr_enable: bool }
#[repr(C)] pub struct Twl4030BciPlatformData { pub battery_tmp_tbl:*mut i32, pub tblsize:u32, pub bb_uvolt:i32, pub bb_uamp:i32 }
#[repr(C)] pub struct Twl4030GpioPlatformData { pub use_leds:bool, pub mmc_cd:u8, pub debounce:u32, pub pullups:u32, pub pulldowns:u32 }
#[repr(C)] pub struct Twl4030MadcPlatformData { pub irq_line:i32 }
#[repr(C)] pub struct MatrixKeymapData { _private: [u8;0] }
#[repr(C)] pub struct Twl4030KeypadData { pub keymap_data:*const MatrixKeymapData, pub rows:u32, pub cols:u32, pub rep:bool }
#[repr(C)] pub struct Device { _private:[u8;0] }
#[repr(u32)] pub enum Twl4030UsbMode { T2UsbModeUlpi=1, T2UsbModeCea2011_3pin=2 }
#[repr(C)] pub struct Twl4030UsbData { pub usb_mode:Twl4030UsbMode, pub features:usize, pub phy_init:Option<unsafe extern "C" fn(*mut Device)->i32>, pub phy_exit:Option<unsafe extern "C" fn(*mut Device)->i32>, pub phy_power:Option<unsafe extern "C" fn(*mut Device,i32,i32)->i32>, pub phy_set_clock:Option<unsafe extern "C" fn(*mut Device,i32)->i32>, pub phy_suspend:Option<unsafe extern "C" fn(*mut Device,i32)->i32> }
#[repr(C)] pub struct Twl4030Ins { pub pmb_message:u16, pub delay:u8 }
#[repr(C)] pub struct Twl4030Script { pub script:*mut Twl4030Ins, pub size:u32, pub flags:u8 }
pub const TWL4030_WRST_SCRIPT:u8=1<<0; pub const TWL4030_WAKEUP12_SCRIPT:u8=1<<1; pub const TWL4030_WAKEUP3_SCRIPT:u8=1<<2; pub const TWL4030_SLEEP_SCRIPT:u8=1<<3;
#[repr(C)] pub struct Twl4030Resconfig { pub resource:u8,pub devgroup:u8,pub typ:u8,pub type2:u8,pub remap_off:u8,pub remap_sleep:u8 }
#[repr(C)] pub struct Twl4030PowerData { pub scripts:*mut *mut Twl4030Script,pub num:u32,pub resource_config:*mut Twl4030Resconfig,pub board_config:*mut Twl4030Resconfig,pub use_poweroff:bool,pub ac_charger_quirk:bool }
#[repr(C)] pub struct Twl4030CodecData { pub digimic_delay:u32,pub ramp_delay_value:u32,pub offset_cncl_path:u32,pub hs_extmute:u32,pub hs_extmute_gpio:i32 }
#[repr(C)] pub struct Twl4030VibraData { pub coexist:u32 }
#[repr(C)] pub struct Twl4030AudioData { pub audio_mclk:u32,pub codec:*mut Twl4030CodecData,pub vibra:*mut Twl4030VibraData,pub audpwron_gpio:i32,pub naudint_irq:i32,pub irq_base:u32 }
#[repr(C)] pub struct TwlRegulatorDriverData { pub set_voltage:Option<unsafe extern "C" fn(*mut core::ffi::c_void,i32)->i32>,pub get_voltage:Option<unsafe extern "C" fn(*mut core::ffi::c_void)->i32>,pub data:*mut core::ffi::c_void,pub features:usize }
extern "C" { pub fn twl4030_sih_setup(dev:*mut Device,module:i32,irq_base:i32)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
