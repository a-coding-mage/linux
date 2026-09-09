/* SPDX-License-Identifier: GPL-2.0-only */
/* Definitions for the DS1685/DS1687-series RTC chips. */

/* C dependencies: linux/rtc.h, linux/platform_device.h, linux/workqueue.h. */

#[repr(C)]
pub struct ds1685_priv {
    pub dev: *mut rtc_device,
    pub regs: *mut core::ffi::c_void,
    pub data: *mut core::ffi::c_void,
    pub regstep: u32,
    pub irq_num: i32,
    pub bcd_mode: bool,
    pub read: Option<unsafe extern "C" fn(*mut ds1685_priv, i32) -> u8>,
    pub write: Option<unsafe extern "C" fn(*mut ds1685_priv, i32, u8)>,
    pub prepare_poweroff: Option<unsafe extern "C" fn()>,
    pub wake_alarm: Option<unsafe extern "C" fn()>,
    pub post_ram_clear: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct ds1685_rtc_platform_data {
    pub regstep: u32,
    pub bcd_mode: bool,
    pub no_irq: bool,
    pub uie_unsupported: bool,
    pub plat_prepare_poweroff: Option<unsafe extern "C" fn()>,
    pub plat_wake_alarm: Option<unsafe extern "C" fn()>,
    pub plat_post_ram_clear: Option<unsafe extern "C" fn()>,
    pub access_type: ds1685_reg_access_type,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ds1685_reg_access_type {
    ds1685_reg_direct,
    ds1685_reg_indirect,
}

macro_rules! bit { ($n:expr) => { 1u32 << $n }; }

pub const RTC_SECS: u32 = 0x00; pub const RTC_SECS_ALARM: u32 = 0x01;
pub const RTC_MINS: u32 = 0x02; pub const RTC_MINS_ALARM: u32 = 0x03;
pub const RTC_HRS: u32 = 0x04; pub const RTC_HRS_ALARM: u32 = 0x05;
pub const RTC_WDAY: u32 = 0x06; pub const RTC_MDAY: u32 = 0x07;
pub const RTC_MONTH: u32 = 0x08; pub const RTC_YEAR: u32 = 0x09;
pub const RTC_CENTURY: u32 = 0x48; pub const RTC_MDAY_ALARM: u32 = 0x49;

pub const RTC_SECS_BCD_MASK: u32 = 0x7f; pub const RTC_MINS_BCD_MASK: u32 = 0x7f;
pub const RTC_HRS_12_BCD_MASK: u32 = 0x1f; pub const RTC_HRS_24_BCD_MASK: u32 = 0x3f;
pub const RTC_MDAY_BCD_MASK: u32 = 0x3f; pub const RTC_MONTH_BCD_MASK: u32 = 0x1f;
pub const RTC_YEAR_BCD_MASK: u32 = 0xff;
pub const RTC_SECS_BIN_MASK: u32 = 0x3f; pub const RTC_MINS_BIN_MASK: u32 = 0x3f;
pub const RTC_HRS_12_BIN_MASK: u32 = 0x0f; pub const RTC_HRS_24_BIN_MASK: u32 = 0x1f;
pub const RTC_MDAY_BIN_MASK: u32 = 0x1f; pub const RTC_MONTH_BIN_MASK: u32 = 0x0f;
pub const RTC_YEAR_BIN_MASK: u32 = 0x7f;
pub const RTC_WDAY_MASK: u32 = 0x07; pub const RTC_CENTURY_MASK: u32 = 0xff;
pub const RTC_MDAY_ALARM_MASK: u32 = 0xff; pub const RTC_HRS_AMPM_MASK: u32 = bit!(7);

pub const RTC_CTRL_A: u32 = 0x0a; pub const RTC_CTRL_B: u32 = 0x0b;
pub const RTC_CTRL_C: u32 = 0x0c; pub const RTC_CTRL_D: u32 = 0x0d;
pub const RTC_EXT_CTRL_4A: u32 = 0x4a; pub const RTC_EXT_CTRL_4B: u32 = 0x4b;
pub const RTC_CTRL_A_UIP: u32 = bit!(7); pub const RTC_CTRL_A_DV2: u32 = bit!(6);
pub const RTC_CTRL_A_DV1: u32 = bit!(5); pub const RTC_CTRL_A_DV0: u32 = bit!(4);
pub const RTC_CTRL_A_RS2: u32 = bit!(2); pub const RTC_CTRL_A_RS3: u32 = bit!(3);
pub const RTC_CTRL_A_RS1: u32 = bit!(1); pub const RTC_CTRL_A_RS0: u32 = bit!(0);
pub const RTC_CTRL_A_RS_MASK: u32 = 0x0f;
pub const RTC_CTRL_B_SET: u32 = bit!(7); pub const RTC_CTRL_B_PIE: u32 = bit!(6);
pub const RTC_CTRL_B_AIE: u32 = bit!(5); pub const RTC_CTRL_B_UIE: u32 = bit!(4);
pub const RTC_CTRL_B_SQWE: u32 = bit!(3); pub const RTC_CTRL_B_DM: u32 = bit!(2);
pub const RTC_CTRL_B_2412: u32 = bit!(1); pub const RTC_CTRL_B_DSE: u32 = bit!(0);
pub const RTC_CTRL_B_PAU_MASK: u32 = 0x70;
pub const RTC_CTRL_C_IRQF: u32 = bit!(7); pub const RTC_CTRL_C_PF: u32 = bit!(6);
pub const RTC_CTRL_C_AF: u32 = bit!(5); pub const RTC_CTRL_C_UF: u32 = bit!(4);
pub const RTC_CTRL_C_PAU_MASK: u32 = 0x70; pub const RTC_CTRL_D_VRT: u32 = bit!(7);
pub const RTC_CTRL_4A_VRT2: u32 = bit!(7); pub const RTC_CTRL_4A_INCR: u32 = bit!(6);
pub const RTC_CTRL_4A_PAB: u32 = bit!(3); pub const RTC_CTRL_4A_RF: u32 = bit!(2);
pub const RTC_CTRL_4A_WF: u32 = bit!(1); pub const RTC_CTRL_4A_KF: u32 = bit!(0);
#[cfg(not(any(feature = "CONFIG_RTC_DRV_DS1685", feature = "CONFIG_RTC_DRV_DS1689")))]
pub const RTC_CTRL_4A_BME: u32 = bit!(5);
pub const RTC_CTRL_4A_RWK_MASK: u32 = 0x07;
pub const RTC_CTRL_4B_ABE: u32 = bit!(7); pub const RTC_CTRL_4B_E32K: u32 = bit!(6);
pub const RTC_CTRL_4B_CS: u32 = bit!(5); pub const RTC_CTRL_4B_RCE: u32 = bit!(4);
pub const RTC_CTRL_4B_PRS: u32 = bit!(3); pub const RTC_CTRL_4B_RIE: u32 = bit!(2);
pub const RTC_CTRL_4B_WIE: u32 = bit!(1); pub const RTC_CTRL_4B_KSE: u32 = bit!(0);
pub const RTC_CTRL_4B_RWK_MASK: u32 = 0x07;

pub const RTC_BANK1_SSN_MODEL: u32 = 0x40; pub const RTC_BANK1_SSN_BYTE_1: u32 = 0x41;
pub const RTC_BANK1_SSN_BYTE_2: u32 = 0x42; pub const RTC_BANK1_SSN_BYTE_3: u32 = 0x43;
pub const RTC_BANK1_SSN_BYTE_4: u32 = 0x44; pub const RTC_BANK1_SSN_BYTE_5: u32 = 0x45;
pub const RTC_BANK1_SSN_BYTE_6: u32 = 0x46; pub const RTC_BANK1_SSN_CRC: u32 = 0x47;
pub const RTC_BANK1_RAM_DATA_PORT: u32 = 0x53;

#[cfg(feature = "CONFIG_RTC_DRV_DS1685")] pub const RTC_BANK1_RAM_ADDR: u32 = 0x50;
#[cfg(feature = "CONFIG_RTC_DRV_DS1689")] pub const RTC_BANK1_VCC_CTR_LSB: u32 = 0x54;
#[cfg(feature = "CONFIG_RTC_DRV_DS1689")] pub const RTC_BANK1_VCC_CTR_MSB: u32 = 0x57;
#[cfg(feature = "CONFIG_RTC_DRV_DS1689")] pub const RTC_BANK1_VBAT_CTR_LSB: u32 = 0x58;
#[cfg(feature = "CONFIG_RTC_DRV_DS1689")] pub const RTC_BANK1_VBAT_CTR_MSB: u32 = 0x5b;
#[cfg(feature = "CONFIG_RTC_DRV_DS1689")] pub const RTC_BANK1_PWR_CTR_LSB: u32 = 0x5c;
#[cfg(feature = "CONFIG_RTC_DRV_DS1689")] pub const RTC_BANK1_PWR_CTR_MSB: u32 = 0x5d;
#[cfg(feature = "CONFIG_RTC_DRV_DS1689")] pub const RTC_BANK1_UNIQ_SN: u32 = 0x60;
#[cfg(not(any(feature = "CONFIG_RTC_DRV_DS1685", feature = "CONFIG_RTC_DRV_DS1689")))] pub const RTC_BANK1_RAM_ADDR_LSB: u32 = 0x50;
#[cfg(not(any(feature = "CONFIG_RTC_DRV_DS1685", feature = "CONFIG_RTC_DRV_DS1689")))] pub const RTC_BANK1_RAM_ADDR_MSB: u32 = 0x51;
#[cfg(not(any(feature = "CONFIG_RTC_DRV_DS1685", feature = "CONFIG_RTC_DRV_DS1689")))] pub const RTC_BANK1_WRITE_CTR: u32 = 0x5e;

pub const RTC_MODEL_DS1685: u32 = 0x71; pub const RTC_MODEL_DS17285: u32 = 0x72;
pub const RTC_MODEL_DS1689: u32 = 0x73; pub const RTC_MODEL_DS17485: u32 = 0x74;
pub const RTC_MODEL_DS17885: u32 = 0x78;
pub const RTC_SQW_8192HZ: u32 = 0x03; pub const RTC_SQW_4096HZ: u32 = 0x04;
pub const RTC_SQW_2048HZ: u32 = 0x05; pub const RTC_SQW_1024HZ: u32 = 0x06;
pub const RTC_SQW_512HZ: u32 = 0x07; pub const RTC_SQW_256HZ: u32 = 0x08;
pub const RTC_SQW_128HZ: u32 = 0x09; pub const RTC_SQW_64HZ: u32 = 0x0a;
pub const RTC_SQW_32HZ: u32 = 0x0b; pub const RTC_SQW_16HZ: u32 = 0x0c;
pub const RTC_SQW_8HZ: u32 = 0x0d; pub const RTC_SQW_4HZ: u32 = 0x0e;
pub const RTC_SQW_2HZ: u32 = 0x0f; pub const RTC_SQW_0HZ: u32 = 0x00;
pub const RTC_SQW_32768HZ: u32 = 32768;
pub const NVRAM_TIME_BASE: u32 = 0x0e; pub const NVRAM_BANK0_BASE: u32 = 0x40;
pub const NVRAM_SZ_TIME: u32 = 50; pub const NVRAM_SZ_BANK0: u32 = 64;
#[cfg(feature = "CONFIG_RTC_DRV_DS1685")] pub const NVRAM_SZ_EXTND: u32 = 128;
#[cfg(feature = "CONFIG_RTC_DRV_DS1689")] pub const NVRAM_SZ_EXTND: u32 = 0;
#[cfg(feature = "CONFIG_RTC_DRV_DS17285")] pub const NVRAM_SZ_EXTND: u32 = 2048;
#[cfg(feature = "CONFIG_RTC_DRV_DS17485")] pub const NVRAM_SZ_EXTND: u32 = 4096;
#[cfg(feature = "CONFIG_RTC_DRV_DS17885")] pub const NVRAM_SZ_EXTND: u32 = 8192;
pub const NVRAM_TOTAL_SZ_BANK0: u32 = NVRAM_SZ_TIME + NVRAM_SZ_BANK0;
pub const NVRAM_TOTAL_SZ: u32 = NVRAM_TOTAL_SZ_BANK0 + NVRAM_SZ_EXTND;

pub type rtc_device = core::ffi::c_void;
pub type platform_device = core::ffi::c_void;
extern "C" {
    pub fn ds1685_rtc_poweroff(pdev: *mut platform_device) -> !;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
