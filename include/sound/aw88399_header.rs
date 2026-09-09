/* SPDX-License-Identifier: GPL-2.0-only
 * Rust translation of linux/sound/aw88399.h.
 * C preprocessor expressions are represented as typed constants below.
 */

// Register map
pub const AW88399_ID_REG: i32 = 0x00; pub const AW88399_SYSST_REG: i32 = 0x01;
pub const AW88399_SYSINT_REG: i32 = 0x02; pub const AW88399_SYSINTM_REG: i32 = 0x03;
pub const AW88399_SYSCTRL_REG: i32 = 0x04; pub const AW88399_SYSCTRL2_REG: i32 = 0x05;
pub const AW88399_I2SCTRL1_REG: i32 = 0x06; pub const AW88399_I2SCTRL2_REG: i32 = 0x07;
pub const AW88399_I2SCTRL3_REG: i32 = 0x08; pub const AW88399_DACCFG1_REG: i32 = 0x09;
pub const AW88399_DACCFG2_REG: i32 = 0x0a; pub const AW88399_DACCFG3_REG: i32 = 0x0b;
pub const AW88399_DACCFG4_REG: i32 = 0x0c; pub const AW88399_DACCFG5_REG: i32 = 0x0d;
pub const AW88399_DACCFG6_REG: i32 = 0x0e; pub const AW88399_DACCFG7_REG: i32 = 0x0f;
pub const AW88399_MPDCFG1_REG: i32 = 0x10; pub const AW88399_MPDCFG2_REG: i32 = 0x11;
pub const AW88399_MPDCFG3_REG: i32 = 0x12; pub const AW88399_MPDCFG4_REG: i32 = 0x13;
pub const AW88399_PWMCTRL1_REG: i32 = 0x14; pub const AW88399_PWMCTRL2_REG: i32 = 0x15;
pub const AW88399_PWMCTRL3_REG: i32 = 0x16; pub const AW88399_I2SCFG1_REG: i32 = 0x17;
pub const AW88399_DBGCTRL_REG: i32 = 0x18; pub const AW88399_HAGCST_REG: i32 = 0x20;
pub const AW88399_VBAT_REG: i32 = 0x21; pub const AW88399_TEMP_REG: i32 = 0x22;
pub const AW88399_PVDD_REG: i32 = 0x23; pub const AW88399_ISNDAT_REG: i32 = 0x24;
pub const AW88399_VSNDAT_REG: i32 = 0x25; pub const AW88399_I2SINT_REG: i32 = 0x26;
pub const AW88399_I2SCAPCNT_REG: i32 = 0x27; pub const AW88399_ANASTA1_REG: i32 = 0x28;
pub const AW88399_ANASTA2_REG: i32 = 0x29; pub const AW88399_ANASTA3_REG: i32 = 0x2a;
pub const AW88399_TESTDET_REG: i32 = 0x2b; pub const AW88399_DSMCFG1_REG: i32 = 0x30;
pub const AW88399_DSMCFG2_REG: i32 = 0x31; pub const AW88399_DSMCFG3_REG: i32 = 0x32;
pub const AW88399_DSMCFG4_REG: i32 = 0x33; pub const AW88399_DSMCFG5_REG: i32 = 0x34;
pub const AW88399_DSMCFG6_REG: i32 = 0x35; pub const AW88399_DSMCFG7_REG: i32 = 0x36;
pub const AW88399_DSMCFG8_REG: i32 = 0x37; pub const AW88399_TESTIN_REG: i32 = 0x38;
pub const AW88399_TESTOUT_REG: i32 = 0x39; pub const AW88399_MEMTEST_REG: i32 = 0x3a;
pub const AW88399_DSPMADD_REG: i32 = 0x40; pub const AW88399_DSPMDAT_REG: i32 = 0x41;
pub const AW88399_WDT_REG: i32 = 0x42; pub const AW88399_ACR1_REG: i32 = 0x43;
pub const AW88399_ACR2_REG: i32 = 0x44; pub const AW88399_ASR1_REG: i32 = 0x45;
pub const AW88399_ASR2_REG: i32 = 0x46; pub const AW88399_DSPCFG_REG: i32 = 0x47;
pub const AW88399_REG_MAX: i32 = 0x7e; pub const AW88399_MUTE_VOL: i32 = 1023;
pub const AW88399_DSP_CFG_ADDR: i32 = 0x9b00; pub const AW88399_DSP_REG_CFG_ADPZ_RA: i32 = 0x9b68;
pub const AW88399_DSP_FW_ADDR: i32 = 0x8980; pub const AW88399_DSP_ROM_CHECK_ADDR: i32 = 0x1f40;
pub const AW88399_DSP_ROM_CHECK_DATA: i32 = 0x4638; pub const AW88399_VOLUME_STEP_DB: i32 = 64;
pub const AW88399_CHIP_ID: i32 = 0x2183; pub const AW88399_ACF_FILE: &str = "aw88399_acf.bin";

pub const AW88399_I2STXEN_START_BIT: i32 = 9; pub const AW88399_I2STXEN_BITS_LEN: i32 = 1;
pub const AW88399_I2STXEN_MASK: i32 = !(((1 << AW88399_I2STXEN_BITS_LEN) - 1) << AW88399_I2STXEN_START_BIT);
pub const AW88399_VOL_START_BIT: i32 = 0; pub const AW88399_VOL_BITS_LEN: i32 = 10;
pub const AW88399_VOL_MASK: i32 = !(((1 << AW88399_VOL_BITS_LEN) - 1) << AW88399_VOL_START_BIT);
pub const AW88399_PWDN_START_BIT: i32 = 0; pub const AW88399_PWDN_BITS_LEN: i32 = 1;
pub const AW88399_PWDN_MASK: i32 = !(((1 << AW88399_PWDN_BITS_LEN) - 1) << AW88399_PWDN_START_BIT);
pub const AW88399_PWDN_POWER_DOWN: i32 = 1; pub const AW88399_PWDN_WORKING: i32 = 0;
pub const AW88399_DEV_DEFAULT_CH: i32 = 0; pub const AW88399_DEV_DSP_CHECK_MAX: i32 = 5;
pub const AW88399_MAX_RAM_WRITE_BYTE_SIZE: i32 = 128; pub const AW88399_DSP_RE_SHIFT: i32 = 12;
pub const AW88399_CALI_RE_MAX: i32 = 15000; pub const AW88399_CALI_RE_MIN: i32 = 4000;
pub const AW_FW_ADDR_LEN: i32 = 4; pub const AW88399_CRC_CHECK_PASS_VAL: i32 = 4;
pub const AW88399_START_RETRIES: i32 = 5; pub const AW88399_START_WORK_DELAY_MS: i32 = 0;

#[repr(C)] pub struct mutex;
#[repr(C)] pub struct delayed_work;
#[repr(C)] pub struct work_struct;
#[repr(C)] pub struct aw_device;
#[repr(C)] pub struct aw_container;
#[repr(C)] pub struct aw_cali_desc;
#[repr(C)] pub struct gpio_desc;
#[repr(C)] pub struct i2c_client;
#[repr(C)] pub struct regmap;

#[repr(C)] pub struct aw88399 {
    pub aw_pa: *mut aw_device, pub lock: mutex, pub reset_gpio: *mut gpio_desc,
    pub start_work: delayed_work, pub regmap: *mut regmap, pub aw_cfg: *mut aw_container,
    pub check_val: u32, pub crc_init_val: u32, pub vcalb_init_val: u32,
    pub dither_st: bool, pub bsts_unreliable: bool, pub fw_needs_reload: bool,
}

extern "C" {
    pub fn aw_dev_check_syspll(aw_dev: *mut aw_device) -> i32;
    pub fn aw_dev_dsp_enable(aw_dev: *mut aw_device, is_enable: bool);
    pub fn aw_dev_get_dsp_status(aw_dev: *mut aw_device) -> i32;
    pub fn aw_dev_set_volume(aw_dev: *mut aw_device, value: u32) -> i32;
    pub fn aw_dev_update_cali_re(cali_desc: *mut aw_cali_desc) -> i32;
    pub fn aw88399_dev_get_prof_name(aw_dev: *mut aw_device, index: i32, prof_name: *mut *mut i8) -> i32;
    pub fn aw88399_dev_mute(aw_dev: *mut aw_device, is_mute: bool);
    pub fn aw88399_dev_set_channel(aw88399: *mut aw88399, channel: i32);
    pub fn aw88399_hw_reset(aw88399: *mut aw88399);
    pub fn aw88399_init(aw88399: *mut aw88399, i2c: *mut i2c_client, regmap: *mut regmap) -> i32;
    pub fn aw88399_request_firmware_file(aw88399: *mut aw88399) -> i32;
    pub fn aw88399_start(aw88399: *mut aw88399, sync_start: bool);
    pub fn aw88399_startup_work(work: *mut work_struct);
    pub fn aw88399_stop(aw_dev: *mut aw_device) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
