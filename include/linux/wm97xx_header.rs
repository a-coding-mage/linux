/* SPDX-License-Identifier: GPL-2.0 */
/* Register bits and API for Wolfson WM97xx series of codecs. */

// Dependencies supplied by the surrounding kernel translation.

pub const WM97XX_GENERIC: u16 = 0x0000;
pub const WM97XX_WM1613: u16 = 0x1613;
pub const AC97_WM97XX_DIGITISER1: u16 = 0x76;
pub const AC97_WM97XX_DIGITISER2: u16 = 0x78;
pub const AC97_WM97XX_DIGITISER_RD: u16 = 0x7a;
pub const AC97_WM9713_DIG1: u16 = 0x74;
pub const AC97_WM9713_DIG2: u16 = AC97_WM97XX_DIGITISER1;
pub const AC97_WM9713_DIG3: u16 = AC97_WM97XX_DIGITISER2;

pub const WM97XX_POLL: u16 = 0x8000;
pub const WM97XX_ADCSEL_X: u16 = 0x1000;
pub const WM97XX_ADCSEL_Y: u16 = 0x2000;
pub const WM97XX_ADCSEL_PRES: u16 = 0x3000;
pub const WM97XX_AUX_ID1: u16 = 0x4000;
pub const WM97XX_AUX_ID2: u16 = 0x5000;
pub const WM97XX_AUX_ID3: u16 = 0x6000;
pub const WM97XX_AUX_ID4: u16 = 0x7000;
pub const WM97XX_ADCSEL_MASK: u16 = 0x7000;
pub const WM97XX_COO: u16 = 0x0800;
pub const WM97XX_CTC: u16 = 0x0400;
pub const WM97XX_CM_RATE_93: u16 = 0x0000;
pub const WM97XX_CM_RATE_187: u16 = 0x0100;
pub const WM97XX_CM_RATE_375: u16 = 0x0200;
pub const WM97XX_CM_RATE_750: u16 = 0x0300;
pub const WM97XX_CM_RATE_8K: u16 = 0x00f0;
pub const WM97XX_CM_RATE_12K: u16 = 0x01f0;
pub const WM97XX_CM_RATE_24K: u16 = 0x02f0;
pub const WM97XX_CM_RATE_48K: u16 = 0x03f0;
pub const WM97XX_CM_RATE_MASK: u16 = 0x03f0;
#[inline] pub const fn WM97XX_RATE(i: u16) -> u16 { ((i & 3) << 8) | if (i & 4) != 0 { 0xf0 } else { 0 } }
#[inline] pub const fn WM97XX_DELAY(i: u16) -> u16 { (i << 4) & 0x00f0 }
pub const WM97XX_DELAY_MASK: u16 = 0x00f0;
pub const WM97XX_SLEN: u16 = 0x0008;
#[inline] pub const fn WM97XX_SLT(i: u16) -> u16 { (i.wrapping_sub(5)) & 0x7 }
pub const WM97XX_SLT_MASK: u16 = 0x0007;
pub const WM97XX_PRP_DETW: u16 = 0x4000;
pub const WM97XX_PRP_DET: u16 = 0x8000;
pub const WM97XX_PRP_DET_DIG: u16 = 0xc000;
pub const WM97XX_RPR: u16 = 0x2000;
pub const WM97XX_PEN_DOWN: u16 = 0x8000;

pub const WM9712_45W: u16 = 0x1000; pub const WM9712_PDEN: u16 = 0x0800;
pub const WM9712_WAIT: u16 = 0x0200; pub const WM9712_PIL: u16 = 0x0100;
pub const WM9712_MASK_HI: u16 = 0x0040; pub const WM9712_MASK_EDGE: u16 = 0x0080;
pub const WM9712_MASK_SYNC: u16 = 0x00c0;
#[inline] pub const fn WM9712_RPU(i: u16) -> u16 { i & 0x3f }
#[inline] pub const fn WM9712_PD(i: u16) -> u16 { 1u16 << i }
pub const AC97_WM9712_POWER: u16 = 0x24; pub const AC97_WM9712_REV: u16 = 0x58;
pub const WM9705_PDEN: u16 = 0x1000; pub const WM9705_PINV: u16 = 0x0800;
pub const WM9705_BSEN: u16 = 0x0400; pub const WM9705_BINV: u16 = 0x0200;
pub const WM9705_WAIT: u16 = 0x0100; pub const WM9705_PIL: u16 = 0x0080;
pub const WM9705_PHIZ: u16 = 0x0040; pub const WM9705_MASK_HI: u16 = 0x0010;
pub const WM9705_MASK_EDGE: u16 = 0x0020; pub const WM9705_MASK_SYNC: u16 = 0x0030;
#[inline] pub const fn WM9705_PDD(i: u16) -> u16 { i & 0x000f }
pub const WM9713_PDPOL: u16 = 0x0400; pub const WM9713_POLL: u16 = 0x0200;
pub const WM9713_CTC: u16 = 0x0100; pub const WM9713_ADCSEL_X: u16 = 0x0002;
pub const WM9713_ADCSEL_Y: u16 = 0x0004; pub const WM9713_ADCSEL_PRES: u16 = 0x0008;
pub const WM9713_COO: u16 = 0x0001; pub const WM9713_45W: u16 = 0x1000;
pub const WM9713_PDEN: u16 = 0x0800; pub const WM9713_ADCSEL_MASK: u16 = 0x00fe;
pub const WM9713_WAIT: u16 = 0x0200;
pub const TS_COMP1: u16 = 0; pub const TS_COMP2: u16 = 1; pub const TS_BMON: u16 = 2; pub const TS_WIPER: u16 = 3;
pub const WM97XX_ID1: u16 = 0x574d; pub const WM9712_ID2: u16 = 0x4c12; pub const WM9705_ID2: u16 = 0x4c05; pub const WM9713_ID2: u16 = 0x4c13;
pub const WM97XX_MAX_GPIO: u32 = 16;
pub const AC97_LINK_FRAME: u32 = 21;
pub const RC_AGAIN: u32 = 1; pub const RC_VALID: u32 = 2; pub const RC_PENUP: u32 = 4; pub const RC_PENDOWN: u32 = 8;

#[repr(C)] pub struct wm97xx_data { pub x: i32, pub y: i32, pub p: i32 }
#[repr(C)] pub enum wm97xx_gpio_status { WM97XX_GPIO_HIGH, WM97XX_GPIO_LOW }
#[repr(C)] pub enum wm97xx_gpio_dir { WM97XX_GPIO_IN, WM97XX_GPIO_OUT }
#[repr(C)] pub enum wm97xx_gpio_pol { WM97XX_GPIO_POL_HIGH, WM97XX_GPIO_POL_LOW }
#[repr(C)] pub enum wm97xx_gpio_sticky { WM97XX_GPIO_STICKY, WM97XX_GPIO_NOTSTICKY }
#[repr(C)] pub enum wm97xx_gpio_wake { WM97XX_GPIO_WAKE, WM97XX_GPIO_NOWAKE }
pub const WM97XX_DIG_START: u32 = 1; pub const WM97XX_DIG_STOP: u32 = 2; pub const WM97XX_PHY_INIT: u32 = 3; pub const WM97XX_AUX_PREPARE: u32 = 4; pub const WM97XX_DIG_RESTORE: u32 = 5;

pub enum wm97xx {}
pub struct wm97xx_codec_drv;
extern "C" { pub static mut wm9705_codec: wm97xx_codec_drv; pub static mut wm9712_codec: wm97xx_codec_drv; pub static mut wm9713_codec: wm97xx_codec_drv; }
#[repr(C)] pub struct wm97xx_codec_drv {
    pub id: u16, pub name: *mut i8,
    pub poll_sample: Option<unsafe extern "C" fn(*mut wm97xx, i32, *mut i32) -> i32>,
    pub poll_touch: Option<unsafe extern "C" fn(*mut wm97xx, *mut wm97xx_data) -> i32>,
    pub acc_enable: Option<unsafe extern "C" fn(*mut wm97xx, i32) -> i32>,
    pub phy_init: Option<unsafe extern "C" fn(*mut wm97xx)>, pub dig_enable: Option<unsafe extern "C" fn(*mut wm97xx, i32)>,
    pub dig_restore: Option<unsafe extern "C" fn(*mut wm97xx)>, pub aux_prepare: Option<unsafe extern "C" fn(*mut wm97xx)>,
}

#[repr(C)] pub struct wm97xx_mach_ops {
    pub acc_enabled: i32, pub acc_pen_up: Option<unsafe extern "C" fn(*mut wm97xx)>, pub acc_pen_down: Option<unsafe extern "C" fn(*mut wm97xx) -> i32>, pub acc_startup: Option<unsafe extern "C" fn(*mut wm97xx) -> i32>, pub acc_shutdown: Option<unsafe extern "C" fn(*mut wm97xx)>,
    pub irq_gpio: i32, pub pre_sample: Option<unsafe extern "C" fn(i32)>, pub post_sample: Option<unsafe extern "C" fn(i32)>,
}

// Kernel-provided types are intentionally referenced rather than reimplemented here.
#[repr(C)] pub struct wm97xx {
    pub dig: [u16; 3], pub id: u16, pub gpio: [u16; 6], pub misc: u16, pub dig_save: [u16; 3],
    pub codec: *mut wm97xx_codec_drv, pub input_dev: *mut input_dev, pub ac97: *mut snd_ac97, pub dev: *mut device,
    pub battery_dev: *mut platform_device, pub touch_dev: *mut platform_device, pub mach_ops: *mut wm97xx_mach_ops,
    pub codec_mutex: mutex, pub ts_reader: delayed_work, pub ts_reader_interval: c_ulong, pub ts_reader_min_interval: c_ulong,
    pub pen_irq: c_uint, pub ts_workq: *mut workqueue_struct, pub acc_slot: u16, pub acc_rate: u16,
    pub pen_is_down: u8, pub aux_waiting: u8, pub pen_probably_down: u8, pub variant: u16, pub suspend_mode: u16,
}
#[repr(C)] pub struct wm97xx_batt_pdata { pub batt_aux: i32, pub temp_aux: i32, pub min_voltage: i32, pub max_voltage: i32, pub batt_div: i32, pub batt_mult: i32, pub temp_div: i32, pub temp_mult: i32, pub batt_tech: i32, pub batt_name: *mut i8 }
#[repr(C)] pub struct wm97xx_pdata { pub batt_pdata: *mut wm97xx_batt_pdata }

extern "C" {
    pub fn wm97xx_get_gpio(wm: *mut wm97xx, gpio: u32) -> wm97xx_gpio_status;
    pub fn wm97xx_set_gpio(wm: *mut wm97xx, gpio: u32, status: wm97xx_gpio_status);
    pub fn wm97xx_config_gpio(wm: *mut wm97xx, gpio: u32, dir: wm97xx_gpio_dir, pol: wm97xx_gpio_pol, sticky: wm97xx_gpio_sticky, wake: wm97xx_gpio_wake);
    pub fn wm97xx_set_suspend_mode(wm: *mut wm97xx, mode: u16);
    pub fn wm97xx_reg_read(wm: *mut wm97xx, reg: u16) -> i32;
    pub fn wm97xx_reg_write(wm: *mut wm97xx, reg: u16, val: u16);
    pub fn wm97xx_read_aux_adc(wm: *mut wm97xx, adcsel: u16) -> i32;
    pub fn wm97xx_register_mach_ops(wm: *mut wm97xx, ops: *mut wm97xx_mach_ops) -> i32;
    pub fn wm97xx_unregister_mach_ops(wm: *mut wm97xx);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
