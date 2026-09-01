// SPDX-License-Identifier: GPL-2.0-only
/*
 * ALSA SoC Texas Instruments TLV320DAC33 codec driver
 *
 * Author: Peter Ujfalusi <peter.ujfalusi@ti.com>
 *
 * Copyright:   (C) 2009 Nokia Corporation
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::MaybeUninit;
use core::ptr;

type u8 = u8;
type irqreturn_t = c_int;
type snd_pcm_sframes_t = isize;

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
}
#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct i2c_client {
    pub dev: device,
    pub irq: c_int,
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
    pub name: *const c_char,
}
#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}
#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}
#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub enumerated: snd_ctl_elem_value_enumerated,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 4],
}
#[repr(C)]
pub struct snd_pcm_runtime {
    pub rate: c_uint,
    pub period_size: c_uint,
    pub format: c_int,
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}
#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON = 0,
    SND_SOC_BIAS_PREPARE,
    SND_SOC_BIAS_STANDBY,
    SND_SOC_BIAS_OFF,
}

const DAC33_FIFO_SIZE_16BIT: c_uint = 6144;
const DAC33_FIFO_SIZE_24BIT: c_uint = 4096;
const DAC33_MODE7_MARGIN: c_uint = 10;
const BURST_BASEFREQ_HZ: c_uint = 49152000;
const DAC33_NUM_SUPPLIES: usize = 3;
const DAC33_I2C_ADDR_AUTOINC: u8 = 0x80;

fn SAMPLES_TO_US(rate: c_uint, samples: c_uint) -> c_uint {
    1000000000u32 / (((rate).wrapping_mul(1000)) / samples)
}

fn US_TO_SAMPLES(rate: c_uint, us: c_uint) -> c_uint {
    rate / (1000000 / if us < 1000000 { us } else { 1000000 })
}

fn UTHR_FROM_PERIOD_SIZE(samples: c_uint, playrate: c_uint, burstrate: c_uint) -> c_uint {
    samples.wrapping_mul(5000) / (burstrate.wrapping_mul(5000) / (burstrate - playrate))
}

fn CALC_BURST_RATE(bclkdiv: u8, bclk_per_sample: c_uint) -> c_uint {
    BURST_BASEFREQ_HZ / bclkdiv as c_uint / bclk_per_sample
}

fn CALC_OSCSET(rate: c_uint, refclk: c_uint) -> c_uint {
    ((((rate.wrapping_mul(10000)) / refclk) * 4096) + 7000) / 10000
}

fn CALC_RATIOSET(rate: c_uint, refclk: c_uint) -> c_uint {
    ((((refclk.wrapping_mul(100000)) / rate) * 16384) + 50000) / 100000
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum dac33_state {
    DAC33_IDLE = 0,
    DAC33_PREFILL,
    DAC33_PLAYBACK,
    DAC33_FLUSH,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum dac33_fifo_modes {
    DAC33_FIFO_BYPASS = 0,
    DAC33_FIFO_MODE1,
    DAC33_FIFO_MODE7,
    DAC33_FIFO_LAST_MODE,
}

static dac33_supply_names: [*const c_char; DAC33_NUM_SUPPLIES] = [
    b"AVDD\0".as_ptr() as *const c_char,
    b"DVDD\0".as_ptr() as *const c_char,
    b"IOVDD\0".as_ptr() as *const c_char,
];

#[repr(C)]
struct tlv320dac33_priv {
    mutex: mutex,
    work: work_struct,
    component: *mut snd_soc_component,
    supplies: [regulator_bulk_data; DAC33_NUM_SUPPLIES],
    substream: *mut snd_pcm_substream,
    reset_gpiod: *mut gpio_desc,
    chip_power: c_int,
    irq: c_int,
    refclk: c_uint,
    alarm_threshold: c_uint,
    fifo_mode: dac33_fifo_modes,
    fifo_size: c_uint,
    nsample: c_uint,
    mode1_latency: c_int,
    burst_bclkdiv: u8,
    burst_rate: c_uint,
    keep_bclk: c_int,
    lock: spinlock_t,
    t_stamp1: u64,
    t_stamp2: u64,
    mode1_us_burst: c_uint,
    mode7_us_to_lthr: c_uint,
    uthr: c_uint,
    state: dac33_state,
    i2c: *mut i2c_client,
    reg_cache: [u8; 0],
}

static dac33_reg: [u8; DAC33_CACHEREGNUM as usize] = [
    0x00, 0x00, 0x00, 0x00, /* 0x00 - 0x03 */
    0x00, 0x00, 0x00, 0x00, /* 0x04 - 0x07 */
    0x00, 0x00, 0x00, 0x00, /* 0x08 - 0x0b */
    0x00, 0x00, 0x00, 0x00, /* 0x0c - 0x0f */
    0x00, 0x00, 0x00, 0x00, /* 0x10 - 0x13 */
    0x00, 0x00, 0x00, 0x00, /* 0x14 - 0x17 */
    0x00, 0x00, 0x00, 0x00, /* 0x18 - 0x1b */
    0x00, 0x00, 0x00, 0x00, /* 0x1c - 0x1f */
    0x00, 0x00, 0x00, 0x00, /* 0x20 - 0x23 */
    0x00, 0x00, 0x00, 0x00, /* 0x24 - 0x27 */
    0x00, 0x00, 0x00, 0x00, /* 0x28 - 0x2b */
    0x00, 0x00, 0x00, 0x80, /* 0x2c - 0x2f */
    0x80, 0x00, 0x00, 0x00, /* 0x30 - 0x33 */
    0x00, 0x00, 0x00, 0x00, /* 0x34 - 0x37 */
    0x00, 0x00, 0x00, 0x00, /* 0x38 - 0x3b; 0x3a - 0x3f reserved */
    0x00, 0x00, 0x00, 0x00, /* 0x3c - 0x3f */
    0x00, 0x00, 0x00, 0x00, /* 0x40 - 0x43 */
    0x00, 0x80, 0x80, 0x80, /* 0x44 - 0x47; 0x46 - 0x47 reserved */
    0x80, 0x00, 0x00, 0x00, /* 0x48 - 0x4b; 0x4b - 0x7c reserved */
    0x00, 0x00, 0x00, 0x00, /* 0x4c - 0x4f */
    0x00, 0x00, 0x00, 0x00, /* 0x50 - 0x53 */
    0x00, 0x00, 0x00, 0x00, /* 0x54 - 0x57 */
    0x00, 0x00, 0x00, 0x00, /* 0x58 - 0x5b */
    0x00, 0x00, 0x00, 0x00, /* 0x5c - 0x5f */
    0x00, 0x00, 0x00, 0x00, /* 0x60 - 0x63 */
    0x00, 0x00, 0x00, 0x00, /* 0x64 - 0x67 */
    0x00, 0x00, 0x00, 0x00, /* 0x68 - 0x6b */
    0x00, 0x00, 0x00, 0x00, /* 0x6c - 0x6f */
    0x00, 0x00, 0x00, 0x00, /* 0x70 - 0x73 */
    0x00, 0x00, 0x00, 0x00, /* 0x74 - 0x77 */
    0x00, 0x00, 0x00, 0x00, /* 0x78 - 0x7b */
    0x00, 0xda, 0x33, 0x03, /* 0x7c - 0x7f */
];

unsafe fn dac33_priv(component: *mut snd_soc_component) -> *mut tlv320dac33_priv {
    snd_soc_component_get_drvdata(component) as *mut tlv320dac33_priv
}

unsafe fn dac33_read_reg_cache(component: *mut snd_soc_component, reg: c_uint) -> c_uint {
    let dac33 = dac33_priv(component);
    let cache = (*dac33).reg_cache.as_ptr();
    if reg >= DAC33_CACHEREGNUM {
        return 0;
    }
    *cache.add(reg as usize) as c_uint
}

unsafe fn dac33_write_reg_cache(component: *mut snd_soc_component, reg: u8, value: u8) {
    let dac33 = dac33_priv(component);
    let cache = (*dac33).reg_cache.as_mut_ptr();
    if reg as c_uint >= DAC33_CACHEREGNUM {
        return;
    }
    *cache.add(reg as usize) = value;
}

unsafe fn dac33_read(component: *mut snd_soc_component, reg: c_uint, value: *mut u8) -> c_int {
    let dac33 = dac33_priv(component);
    let mut ret = 0;
    *value = (reg & 0xff) as u8;
    if (*dac33).chip_power != 0 {
        let val = i2c_smbus_read_byte_data((*dac33).i2c, *value);
        if val < 0 {
            dev_err((*component).dev, b"Read failed (%d)\n\0".as_ptr() as *const c_char, val);
            *value = dac33_read_reg_cache(component, reg) as u8;
            ret = val;
        } else {
            *value = val as u8;
            dac33_write_reg_cache(component, reg as u8, val as u8);
        }
    } else {
        *value = dac33_read_reg_cache(component, reg) as u8;
    }
    ret
}

unsafe fn dac33_write(component: *mut snd_soc_component, reg: c_uint, value: c_uint) -> c_int {
    let dac33 = dac33_priv(component);
    let mut data = [(reg & 0xff) as u8, (value & 0xff) as u8];
    dac33_write_reg_cache(component, data[0], data[1]);
    let mut ret = 0;
    if (*dac33).chip_power != 0 {
        ret = i2c_master_send((*dac33).i2c, data.as_mut_ptr(), 2);
        if ret != 2 {
            dev_err((*component).dev, b"Write failed (%d)\n\0".as_ptr() as *const c_char, ret);
        } else {
            ret = 0;
        }
    }
    ret
}

unsafe fn dac33_write_locked(component: *mut snd_soc_component, reg: c_uint, value: c_uint) -> c_int {
    let dac33 = dac33_priv(component);
    mutex_lock(&mut (*dac33).mutex);
    let ret = dac33_write(component, reg, value);
    mutex_unlock(&mut (*dac33).mutex);
    ret
}

unsafe fn dac33_write16(component: *mut snd_soc_component, reg: c_uint, value: c_uint) -> c_int {
    let dac33 = dac33_priv(component);
    let mut data = [(reg & 0xff) as u8, ((value >> 8) & 0xff) as u8, (value & 0xff) as u8];
    dac33_write_reg_cache(component, data[0], data[1]);
    dac33_write_reg_cache(component, data[0].wrapping_add(1), data[2]);
    let mut ret = 0;
    if (*dac33).chip_power != 0 {
        data[0] |= DAC33_I2C_ADDR_AUTOINC;
        ret = i2c_master_send((*dac33).i2c, data.as_mut_ptr(), 3);
        if ret != 3 {
            dev_err((*component).dev, b"Write failed (%d)\n\0".as_ptr() as *const c_char, ret);
        } else {
            ret = 0;
        }
    }
    ret
}

unsafe fn dac33_init_chip(component: *mut snd_soc_component) {
    let dac33 = dac33_priv(component);
    if (*dac33).chip_power == 0 {
        return;
    }
    dac33_write(component, DAC33_DAC_CTRL_A, DAC33_DACRATE(0));
    dac33_write(component, DAC33_DAC_CTRL_B, DAC33_DACSRCR_RIGHT | DAC33_DACSRCL_LEFT);
    dac33_write(component, DAC33_DAC_CTRL_C, 0x00);
    dac33_write(component, DAC33_ANA_VOL_SOFT_STEP_CTRL, DAC33_VOLCLKEN);
    dac33_write(component, DAC33_LDAC_DIG_VOL_CTRL, dac33_read_reg_cache(component, DAC33_LDAC_DIG_VOL_CTRL));
    dac33_write(component, DAC33_RDAC_DIG_VOL_CTRL, dac33_read_reg_cache(component, DAC33_RDAC_DIG_VOL_CTRL));
    dac33_write(component, DAC33_LINEL_TO_LLO_VOL, dac33_read_reg_cache(component, DAC33_LINEL_TO_LLO_VOL));
    dac33_write(component, DAC33_LINER_TO_RLO_VOL, dac33_read_reg_cache(component, DAC33_LINER_TO_RLO_VOL));
    dac33_write(component, DAC33_OUT_AMP_CTRL, dac33_read_reg_cache(component, DAC33_OUT_AMP_CTRL));
    dac33_write(component, DAC33_LDAC_PWR_CTRL, dac33_read_reg_cache(component, DAC33_LDAC_PWR_CTRL));
    dac33_write(component, DAC33_RDAC_PWR_CTRL, dac33_read_reg_cache(component, DAC33_RDAC_PWR_CTRL));
}

unsafe fn dac33_read_id(component: *mut snd_soc_component) -> c_int {
    let mut ret = 0;
    let mut reg = 0u8;
    for i in 0..3 {
        ret = dac33_read(component, DAC33_DEVICE_ID_MSB + i, &mut reg);
        if ret < 0 {
            break;
        }
    }
    ret
}

unsafe fn dac33_soft_power(component: *mut snd_soc_component, power: c_int) {
    let mut reg = dac33_read_reg_cache(component, DAC33_PWR_CTRL) as u8;
    if power != 0 {
        reg |= DAC33_PDNALLB as u8;
    } else {
        reg &= !(DAC33_PDNALLB | DAC33_OSCPDNB | DAC33_DACRPDNB | DAC33_DACLPDNB) as u8;
    }
    dac33_write(component, DAC33_PWR_CTRL, reg as c_uint);
}

unsafe fn dac33_disable_digital(component: *mut snd_soc_component) {
    let mut reg = dac33_read_reg_cache(component, DAC33_SER_AUDIOIF_CTRL_B) as u8;
    reg &= !(DAC33_BCLKON as u8);
    dac33_write(component, DAC33_SER_AUDIOIF_CTRL_B, reg as c_uint);
    reg = dac33_read_reg_cache(component, DAC33_PWR_CTRL) as u8;
    reg &= !(DAC33_OSCPDNB | DAC33_DACRPDNB | DAC33_DACLPDNB) as u8;
    dac33_write(component, DAC33_PWR_CTRL, reg as c_uint);
}

unsafe fn dac33_hard_power(component: *mut snd_soc_component, power: c_int) -> c_int {
    let dac33 = dac33_priv(component);
    let mut ret = 0;
    mutex_lock(&mut (*dac33).mutex);
    if power == (*dac33).chip_power {
        dev_dbg((*component).dev, b"Trying to set the same power state: %s\n\0".as_ptr() as *const c_char, if power != 0 { b"ON\0".as_ptr() } else { b"OFF\0".as_ptr() } as *const c_char);
        mutex_unlock(&mut (*dac33).mutex);
        return ret;
    }
    if power != 0 {
        ret = regulator_bulk_enable(DAC33_NUM_SUPPLIES as c_int, (*dac33).supplies.as_mut_ptr());
        if ret != 0 {
            dev_err((*component).dev, b"Failed to enable supplies: %d\n\0".as_ptr() as *const c_char, ret);
            mutex_unlock(&mut (*dac33).mutex);
            return ret;
        }
        if !(*dac33).reset_gpiod.is_null() {
            ret = gpiod_set_value((*dac33).reset_gpiod, 1);
            if ret < 0 {
                dev_err(&mut (*(*dac33).i2c).dev, b"Failed to set reset GPIO: %d\n\0".as_ptr() as *const c_char, ret);
                mutex_unlock(&mut (*dac33).mutex);
                return ret;
            }
        }
        (*dac33).chip_power = 1;
    } else {
        dac33_soft_power(component, 0);
        if !(*dac33).reset_gpiod.is_null() {
            ret = gpiod_set_value((*dac33).reset_gpiod, 0);
            if ret < 0 {
                dev_err(&mut (*(*dac33).i2c).dev, b"Failed to set reset GPIO: %d\n\0".as_ptr() as *const c_char, ret);
                mutex_unlock(&mut (*dac33).mutex);
                return ret;
            }
        }
        ret = regulator_bulk_disable(DAC33_NUM_SUPPLIES as c_int, (*dac33).supplies.as_mut_ptr());
        if ret != 0 {
            dev_err((*component).dev, b"Failed to disable supplies: %d\n\0".as_ptr() as *const c_char, ret);
            mutex_unlock(&mut (*dac33).mutex);
            return ret;
        }
        (*dac33).chip_power = 0;
    }
    mutex_unlock(&mut (*dac33).mutex);
    ret
}

unsafe fn dac33_playback_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let dac33 = dac33_priv(component);
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            if !(*dac33).substream.is_null() {
                dac33_calculate_times((*dac33).substream, component);
                dac33_prepare_chip((*dac33).substream, component);
            }
        }
        SND_SOC_DAPM_POST_PMD => dac33_disable_digital(component),
        _ => {}
    }
    0
}

unsafe fn dac33_get_fifo_mode(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let dac33 = dac33_priv(component);
    (*ucontrol).value.enumerated.item[0] = (*dac33).fifo_mode as c_uint;
    0
}

unsafe fn dac33_set_fifo_mode(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let dac33 = dac33_priv(component);
    let item = (*ucontrol).value.enumerated.item[0];
    if (*dac33).fifo_mode as c_uint == item {
        return 0;
    }
    if snd_soc_component_active(component) != 0 {
        return -EPERM;
    }
    if item >= dac33_fifo_modes::DAC33_FIFO_LAST_MODE as c_uint {
        -EINVAL
    } else {
        (*dac33).fifo_mode = core::mem::transmute::<c_uint, dac33_fifo_modes>(item);
        0
    }
}

static dac33_fifo_mode_texts: [*const c_char; 3] = [
    b"Bypass\0".as_ptr() as *const c_char,
    b"Mode 1\0".as_ptr() as *const c_char,
    b"Mode 7\0".as_ptr() as *const c_char,
];

static lr_lineout_gain_texts: [*const c_char; 4] = [
    b"Line -12dB DAC 0dB\0".as_ptr() as *const c_char,
    b"Line -6dB DAC 6dB\0".as_ptr() as *const c_char,
    b"Line 0dB DAC 12dB\0".as_ptr() as *const c_char,
    b"Line 6dB DAC 18dB\0".as_ptr() as *const c_char,
];

/* ALSA control/widget declarations from SOC_* and SND_SOC_DAPM_* macros are
 * preserved by name and initializer intent; their concrete layout is supplied
 * by external ALSA definitions in the full repository.
 */
extern "C" {
    static dac33_fifo_mode_enum: c_void;
    static l_lineout_gain_enum: c_void;
    static r_lineout_gain_enum: c_void;
    static dac_digivol_tlv: c_void;
    static dac33_snd_controls: [snd_kcontrol_new; 5];
    static dac33_mode_snd_controls: [snd_kcontrol_new; 1];
    static dac33_dapm_abypassl_control: snd_kcontrol_new;
    static dac33_dapm_abypassr_control: snd_kcontrol_new;
    static dac33_left_lom_enum: c_void;
    static dac33_right_lom_enum: c_void;
    static dac33_dapm_left_lom_control: snd_kcontrol_new;
    static dac33_dapm_right_lom_control: snd_kcontrol_new;
    static dac33_dapm_widgets: [snd_soc_dapm_widget_desc; 18];
}

static audio_map: [snd_soc_dapm_route; 22] = [
    route(b"Analog Left Bypass\0", b"Switch\0", b"LINEL\0"),
    route(b"Analog Right Bypass\0", b"Switch\0", b"LINER\0"),
    route_null(b"Output Left Amplifier\0", b"DACL\0"),
    route_null(b"Output Right Amplifier\0", b"DACR\0"),
    route_null(b"Left Bypass PGA\0", b"Analog Left Bypass\0"),
    route_null(b"Right Bypass PGA\0", b"Analog Right Bypass\0"),
    route(b"Left LOM Inverted From\0", b"DAC\0", b"Left Bypass PGA\0"),
    route(b"Right LOM Inverted From\0", b"DAC\0", b"Right Bypass PGA\0"),
    route(b"Left LOM Inverted From\0", b"LOP\0", b"Analog Left Bypass\0"),
    route(b"Right LOM Inverted From\0", b"LOP\0", b"Analog Right Bypass\0"),
    route_null(b"Output Left Amplifier\0", b"Left LOM Inverted From\0"),
    route_null(b"Output Right Amplifier\0", b"Right LOM Inverted From\0"),
    route_null(b"DACL\0", b"Left DAC Power\0"),
    route_null(b"DACR\0", b"Right DAC Power\0"),
    route_null(b"Left Bypass PGA\0", b"Left DAC Power\0"),
    route_null(b"Right Bypass PGA\0", b"Right DAC Power\0"),
    route_null(b"LEFT_LO\0", b"Output Left Amplifier\0"),
    route_null(b"RIGHT_LO\0", b"Output Right Amplifier\0"),
    route_null(b"LEFT_LO\0", b"Codec Power\0"),
    route_null(b"RIGHT_LO\0", b"Codec Power\0"),
    route_null(b"\0", b"\0"),
    route_null(b"\0", b"\0"),
];

const fn route(sink: &'static [u8], control: &'static [u8], source: &'static [u8]) -> snd_soc_dapm_route {
    snd_soc_dapm_route { sink: sink.as_ptr() as *const c_char, control: control.as_ptr() as *const c_char, source: source.as_ptr() as *const c_char }
}
const fn route_null(sink: &'static [u8], source: &'static [u8]) -> snd_soc_dapm_route {
    snd_soc_dapm_route { sink: sink.as_ptr() as *const c_char, control: ptr::null(), source: source.as_ptr() as *const c_char }
}

unsafe fn dac33_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let mut ret;
    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {}
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {}
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == snd_soc_bias_level::SND_SOC_BIAS_OFF {
                ret = dac33_hard_power(component, 1);
                if ret != 0 {
                    return ret;
                }
                dac33_init_chip(component);
            }
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            if snd_soc_dapm_get_bias_level(dapm) == snd_soc_bias_level::SND_SOC_BIAS_OFF {
                return 0;
            }
            ret = dac33_hard_power(component, 0);
            if ret != 0 {
                return ret;
            }
        }
    }
    0
}

unsafe fn dac33_prefill_handler(dac33: *mut tlv320dac33_priv) {
    let component = (*dac33).component;
    let delay: c_uint;
    match (*dac33).fifo_mode {
        dac33_fifo_modes::DAC33_FIFO_MODE1 => {
            dac33_write16(component, DAC33_NSAMPLE_MSB, DAC33_THRREG((*dac33).nsample));
            spin_lock_irqsave(&mut (*dac33).lock);
            (*dac33).t_stamp2 = ktime_to_us(ktime_get()) as u64;
            (*dac33).t_stamp1 = (*dac33).t_stamp2;
            spin_unlock_irqrestore(&mut (*dac33).lock);
            dac33_write16(component, DAC33_PREFILL_MSB, DAC33_THRREG((*dac33).alarm_threshold));
            delay = SAMPLES_TO_US((*dac33).burst_rate, (*dac33).alarm_threshold) + 1000;
            usleep_range(delay, delay + 500);
            dac33_write(component, DAC33_FIFO_IRQ_MASK, DAC33_MAT);
        }
        dac33_fifo_modes::DAC33_FIFO_MODE7 => {
            spin_lock_irqsave(&mut (*dac33).lock);
            (*dac33).t_stamp1 = ktime_to_us(ktime_get()) as u64;
            (*dac33).t_stamp1 = (*dac33).t_stamp1.wrapping_sub((*dac33).mode7_us_to_lthr as u64);
            spin_unlock_irqrestore(&mut (*dac33).lock);
            dac33_write16(component, DAC33_PREFILL_MSB, DAC33_THRREG(DAC33_MODE7_MARGIN));
            dac33_write(component, DAC33_FIFO_IRQ_MASK, DAC33_MUT);
        }
        _ => dev_warn((*component).dev, b"Unhandled FIFO mode: %d\n\0".as_ptr() as *const c_char, (*dac33).fifo_mode as c_int),
    }
}

unsafe fn dac33_playback_handler(dac33: *mut tlv320dac33_priv) {
    let component = (*dac33).component;
    match (*dac33).fifo_mode {
        dac33_fifo_modes::DAC33_FIFO_MODE1 => {
            spin_lock_irqsave(&mut (*dac33).lock);
            (*dac33).t_stamp2 = ktime_to_us(ktime_get()) as u64;
            spin_unlock_irqrestore(&mut (*dac33).lock);
            dac33_write16(component, DAC33_NSAMPLE_MSB, DAC33_THRREG((*dac33).nsample));
        }
        dac33_fifo_modes::DAC33_FIFO_MODE7 => {}
        _ => dev_warn((*component).dev, b"Unhandled FIFO mode: %d\n\0".as_ptr() as *const c_char, (*dac33).fifo_mode as c_int),
    }
}

unsafe fn dac33_work(work: *mut work_struct) {
    let dac33 = container_of_tlv320dac33_priv_work(work);
    let component = (*dac33).component;
    mutex_lock(&mut (*dac33).mutex);
    match (*dac33).state {
        dac33_state::DAC33_PREFILL => {
            (*dac33).state = dac33_state::DAC33_PLAYBACK;
            dac33_prefill_handler(dac33);
        }
        dac33_state::DAC33_PLAYBACK => dac33_playback_handler(dac33),
        dac33_state::DAC33_IDLE => {}
        dac33_state::DAC33_FLUSH => {
            (*dac33).state = dac33_state::DAC33_IDLE;
            dac33_write(component, DAC33_FIFO_IRQ_MASK, 0);
            let mut reg = dac33_read_reg_cache(component, DAC33_FIFO_CTRL_A) as u8;
            reg |= DAC33_FIFOFLUSH as u8;
            dac33_write(component, DAC33_FIFO_CTRL_A, reg as c_uint);
        }
    }
    mutex_unlock(&mut (*dac33).mutex);
}

unsafe fn dac33_interrupt_handler(_irq: c_int, dev: *mut c_void) -> irqreturn_t {
    let component = dev as *mut snd_soc_component;
    let dac33 = dac33_priv(component);
    spin_lock_irqsave(&mut (*dac33).lock);
    (*dac33).t_stamp1 = ktime_to_us(ktime_get()) as u64;
    spin_unlock_irqrestore(&mut (*dac33).lock);
    if (*dac33).fifo_mode != dac33_fifo_modes::DAC33_FIFO_MODE7 {
        schedule_work(&mut (*dac33).work);
    }
    IRQ_HANDLED
}

unsafe fn dac33_oscwait(component: *mut snd_soc_component) {
    let mut timeout = 60;
    let mut reg = 0u8;
    loop {
        usleep_range(1000, 2000);
        dac33_read(component, DAC33_INT_OSC_STATUS, &mut reg);
        let done = (reg & 0x03) as c_uint == DAC33_OSCSTATUS_NORMAL;
        let old = timeout;
        timeout -= 1;
        if done || old == 0 {
            break;
        }
    }
    if (reg & 0x03) as c_uint != DAC33_OSCSTATUS_NORMAL {
        dev_err((*component).dev, b"internal oscillator calibration failed\n\0".as_ptr() as *const c_char);
    }
}

unsafe fn dac33_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let dac33 = dac33_priv(component);
    (*dac33).substream = substream;
    0
}

unsafe fn dac33_shutdown(_substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let component = (*dai).component;
    let dac33 = dac33_priv(component);
    (*dac33).substream = ptr::null_mut();
}

unsafe fn dac33_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let dac33 = dac33_priv(component);
    match params_rate(params) {
        44100 | 48000 => {}
        rate => {
            dev_err((*component).dev, b"unsupported rate %d\n\0".as_ptr() as *const c_char, rate);
            return -EINVAL;
        }
    }
    match params_width(params) {
        16 => {
            (*dac33).fifo_size = DAC33_FIFO_SIZE_16BIT;
            (*dac33).burst_rate = CALC_BURST_RATE((*dac33).burst_bclkdiv, 32);
        }
        32 => {
            (*dac33).fifo_size = DAC33_FIFO_SIZE_24BIT;
            (*dac33).burst_rate = CALC_BURST_RATE((*dac33).burst_bclkdiv, 64);
        }
        width => {
            dev_err((*component).dev, b"unsupported width %d\n\0".as_ptr() as *const c_char, width);
            return -EINVAL;
        }
    }
    0
}

unsafe fn dac33_prepare_chip(substream: *mut snd_pcm_substream, component: *mut snd_soc_component) -> c_int {
    let dac33 = dac33_priv(component);
    let (oscset, ratioset) = match (*(*substream).runtime).rate {
        44100 | 48000 => (
            CALC_OSCSET((*(*substream).runtime).rate, (*dac33).refclk),
            CALC_RATIOSET((*(*substream).runtime).rate, (*dac33).refclk),
        ),
        rate => {
            dev_err((*component).dev, b"unsupported rate %d\n\0".as_ptr() as *const c_char, rate);
            return -EINVAL;
        }
    };
    let mut aictrl_a = dac33_read_reg_cache(component, DAC33_SER_AUDIOIF_CTRL_A) as u8;
    aictrl_a &= !(DAC33_NCYCL_MASK | DAC33_WLEN_MASK) as u8;
    let mut fifoctrl_a = dac33_read_reg_cache(component, DAC33_FIFO_CTRL_A) as u8;
    fifoctrl_a &= !(DAC33_FIFOFLUSH as u8);
    fifoctrl_a &= !(DAC33_WIDTH as u8);
    match (*(*substream).runtime).format {
        SNDRV_PCM_FORMAT_S16_LE => {
            aictrl_a |= (DAC33_NCYCL_16 | DAC33_WLEN_16) as u8;
            fifoctrl_a |= DAC33_WIDTH as u8;
        }
        SNDRV_PCM_FORMAT_S32_LE => aictrl_a |= (DAC33_NCYCL_32 | DAC33_WLEN_24) as u8,
        fmt => {
            dev_err((*component).dev, b"unsupported format %d\n\0".as_ptr() as *const c_char, fmt);
            return -EINVAL;
        }
    }
    mutex_lock(&mut (*dac33).mutex);
    if (*dac33).chip_power == 0 {
        mutex_unlock(&mut (*dac33).mutex);
        return 0;
    }
    dac33_soft_power(component, 0);
    dac33_soft_power(component, 1);
    let mut reg_tmp = dac33_read_reg_cache(component, DAC33_INT_OSC_CTRL);
    dac33_write(component, DAC33_INT_OSC_CTRL, reg_tmp);
    dac33_write16(component, DAC33_INT_OSC_FREQ_RAT_A, oscset);
    dac33_write(component, DAC33_CALIB_TIME, 96);
    dac33_write(component, DAC33_INT_OSC_CTRL_B, DAC33_ADJTHRSHLD(2) | DAC33_ADJSTEP(1));
    dac33_write(component, DAC33_INT_OSC_CTRL_C, DAC33_REFDIV(4));
    reg_tmp = dac33_read_reg_cache(component, DAC33_PWR_CTRL);
    let pwr_ctrl = reg_tmp | DAC33_OSCPDNB | DAC33_DACRPDNB | DAC33_DACLPDNB;
    dac33_write(component, DAC33_PWR_CTRL, pwr_ctrl);
    dac33_oscwait(component);
    if (*dac33).fifo_mode as c_uint != 0 {
        dac33_write(component, DAC33_ASRC_CTRL_A, DAC33_SRCLKDIV(1));
        dac33_write(component, DAC33_ASRC_CTRL_B, 1);
        dac33_write16(component, DAC33_SRC_REF_CLK_RATIO_A, ratioset);
        dac33_write(component, DAC33_INTP_CTRL_A, DAC33_INTPM_AHIGH);
    } else {
        dac33_write(component, DAC33_ASRC_CTRL_A, DAC33_SRCBYP);
        dac33_write(component, DAC33_ASRC_CTRL_B, 0);
    }
    match (*dac33).fifo_mode {
        dac33_fifo_modes::DAC33_FIFO_MODE1 => { dac33_write(component, DAC33_FIFO_IRQ_MODE_B, DAC33_ATM(DAC33_FIFO_IRQ_MODE_LEVEL)); }
        dac33_fifo_modes::DAC33_FIFO_MODE7 => { dac33_write(component, DAC33_FIFO_IRQ_MODE_A, DAC33_UTM(DAC33_FIFO_IRQ_MODE_LEVEL)); }
        _ => {}
    }
    let mut aictrl_b = dac33_read_reg_cache(component, DAC33_SER_AUDIOIF_CTRL_B) as u8;
    match (*dac33).fifo_mode {
        dac33_fifo_modes::DAC33_FIFO_MODE1 => {
            fifoctrl_a &= !(DAC33_FBYPAS as u8);
            fifoctrl_a &= !(DAC33_FAUTO as u8);
            if (*dac33).keep_bclk != 0 { aictrl_b |= DAC33_BCLKON as u8; } else { aictrl_b &= !(DAC33_BCLKON as u8); }
        }
        dac33_fifo_modes::DAC33_FIFO_MODE7 => {
            fifoctrl_a &= !(DAC33_FBYPAS as u8);
            fifoctrl_a |= DAC33_FAUTO as u8;
            if (*dac33).keep_bclk != 0 { aictrl_b |= DAC33_BCLKON as u8; } else { aictrl_b &= !(DAC33_BCLKON as u8); }
        }
        _ => {
            fifoctrl_a |= DAC33_FBYPAS as u8;
            aictrl_b |= DAC33_BCLKON as u8;
        }
    }
    dac33_write(component, DAC33_FIFO_CTRL_A, fifoctrl_a as c_uint);
    dac33_write(component, DAC33_SER_AUDIOIF_CTRL_A, aictrl_a as c_uint);
    dac33_write(component, DAC33_SER_AUDIOIF_CTRL_B, aictrl_b as c_uint);
    if (*dac33).fifo_mode as c_uint != 0 {
        dac33_write(component, DAC33_SER_AUDIOIF_CTRL_C, (*dac33).burst_bclkdiv as c_uint);
    } else if (*(*substream).runtime).format == SNDRV_PCM_FORMAT_S16_LE {
        dac33_write(component, DAC33_SER_AUDIOIF_CTRL_C, 32);
    } else {
        dac33_write(component, DAC33_SER_AUDIOIF_CTRL_C, 16);
    }
    match (*dac33).fifo_mode {
        dac33_fifo_modes::DAC33_FIFO_MODE1 => { dac33_write16(component, DAC33_ATHR_MSB, DAC33_THRREG((*dac33).alarm_threshold)); }
        dac33_fifo_modes::DAC33_FIFO_MODE7 => {
            dac33_write16(component, DAC33_UTHR_MSB, DAC33_THRREG((*dac33).uthr));
            dac33_write16(component, DAC33_LTHR_MSB, DAC33_THRREG(DAC33_MODE7_MARGIN));
        }
        _ => {}
    }
    mutex_unlock(&mut (*dac33).mutex);
    0
}

unsafe fn dac33_calculate_times(substream: *mut snd_pcm_substream, component: *mut snd_soc_component) {
    let dac33 = dac33_priv(component);
    let period_size = (*(*substream).runtime).period_size;
    let rate = (*(*substream).runtime).rate;
    if (*dac33).fifo_mode as c_uint == 0 {
        return;
    }
    match (*dac33).fifo_mode {
        dac33_fifo_modes::DAC33_FIFO_MODE1 => {
            (*dac33).alarm_threshold = US_TO_SAMPLES(rate, (*dac33).mode1_latency as c_uint);
            let nsample_limit = (*dac33).fifo_size - (*dac33).alarm_threshold;
            if period_size <= (*dac33).alarm_threshold {
                (*dac33).nsample = period_size * (((*dac33).alarm_threshold / period_size) + if ((*dac33).alarm_threshold % period_size) != 0 { 1 } else { 0 });
            } else if period_size > nsample_limit {
                (*dac33).nsample = nsample_limit;
            } else {
                (*dac33).nsample = period_size;
            }
            (*dac33).mode1_us_burst = SAMPLES_TO_US((*dac33).burst_rate, (*dac33).nsample);
            (*dac33).t_stamp1 = 0;
            (*dac33).t_stamp2 = 0;
        }
        dac33_fifo_modes::DAC33_FIFO_MODE7 => {
            (*dac33).uthr = UTHR_FROM_PERIOD_SIZE(period_size, rate, (*dac33).burst_rate) + 9;
            if (*dac33).uthr > ((*dac33).fifo_size - DAC33_MODE7_MARGIN) {
                (*dac33).uthr = (*dac33).fifo_size - DAC33_MODE7_MARGIN;
            }
            if (*dac33).uthr < DAC33_MODE7_MARGIN + 10 {
                (*dac33).uthr = DAC33_MODE7_MARGIN + 10;
            }
            (*dac33).mode7_us_to_lthr = SAMPLES_TO_US((*(*substream).runtime).rate, (*dac33).uthr - DAC33_MODE7_MARGIN + 1);
            (*dac33).t_stamp1 = 0;
        }
        _ => {}
    }
}

unsafe fn dac33_pcm_trigger(_substream: *mut snd_pcm_substream, cmd: c_int, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let dac33 = dac33_priv(component);
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            if (*dac33).fifo_mode as c_uint != 0 {
                (*dac33).state = dac33_state::DAC33_PREFILL;
                schedule_work(&mut (*dac33).work);
            }
            0
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            if (*dac33).fifo_mode as c_uint != 0 {
                (*dac33).state = dac33_state::DAC33_FLUSH;
                schedule_work(&mut (*dac33).work);
            }
            0
        }
        _ => -EINVAL,
    }
}

unsafe fn dac33_dai_delay(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> snd_pcm_sframes_t {
    let component = (*dai).component;
    let dac33 = dac33_priv(component);
    let mut delay: snd_pcm_sframes_t = 0;
    match (*dac33).fifo_mode {
        dac33_fifo_modes::DAC33_FIFO_BYPASS => {}
        dac33_fifo_modes::DAC33_FIFO_MODE1 => {
            spin_lock_irqsave(&mut (*dac33).lock);
            let t0 = (*dac33).t_stamp1;
            let t1 = (*dac33).t_stamp2;
            spin_unlock_irqrestore(&mut (*dac33).lock);
            let t_now = ktime_to_us(ktime_get()) as u64;
            if t1 == 0 { return 0; }
            if t0 > t1 {
                let time_delta = (t_now - t0) as c_uint;
                let samples_out = if time_delta != 0 { US_TO_SAMPLES((*(*substream).runtime).rate, time_delta) as c_int } else { 0 };
                delay = if (*dac33).alarm_threshold as c_int > samples_out { ((*dac33).alarm_threshold as c_int - samples_out) as snd_pcm_sframes_t } else { 0 };
            } else if (t_now - t1) as c_uint <= (*dac33).mode1_us_burst {
                let mut time_delta = (t_now - t0) as c_uint;
                let samples_out = if time_delta != 0 { US_TO_SAMPLES((*(*substream).runtime).rate, time_delta) as c_int } else { 0 };
                time_delta = (t_now - t1) as c_uint;
                let samples_in = if time_delta != 0 { US_TO_SAMPLES((*dac33).burst_rate, time_delta) as c_int } else { 0 };
                let samples = (*dac33).alarm_threshold as c_int + (samples_in - samples_out);
                delay = if samples > 0 { samples as snd_pcm_sframes_t } else { 0 };
            } else {
                let time_delta = (t_now - t0) as c_uint;
                let samples_out = if time_delta != 0 { US_TO_SAMPLES((*(*substream).runtime).rate, time_delta) as c_int } else { 0 };
                let samples_in = (*dac33).nsample as c_int;
                let samples = (*dac33).alarm_threshold as c_int + (samples_in - samples_out);
                delay = if samples > 0 {
                    if samples as c_uint > (*dac33).fifo_size { (*dac33).fifo_size as snd_pcm_sframes_t } else { samples as snd_pcm_sframes_t }
                } else { 0 };
            }
        }
        dac33_fifo_modes::DAC33_FIFO_MODE7 => {
            spin_lock_irqsave(&mut (*dac33).lock);
            let t0 = (*dac33).t_stamp1;
            let uthr = (*dac33).uthr;
            spin_unlock_irqrestore(&mut (*dac33).lock);
            let t_now = ktime_to_us(ktime_get()) as u64;
            if t0 == 0 { return 0; }
            if t_now <= t0 { return uthr as snd_pcm_sframes_t; }
            let mut time_delta = (t_now - t0) as c_uint;
            if time_delta <= (*dac33).mode7_us_to_lthr {
                let samples_out = US_TO_SAMPLES((*(*substream).runtime).rate, time_delta);
                delay = if uthr > samples_out { (uthr - samples_out) as snd_pcm_sframes_t } else { 0 };
            } else {
                time_delta -= (*dac33).mode7_us_to_lthr;
                let samples_out = US_TO_SAMPLES((*(*substream).runtime).rate, time_delta) as snd_pcm_sframes_t;
                let samples_in = US_TO_SAMPLES((*dac33).burst_rate, time_delta) as snd_pcm_sframes_t;
                delay = DAC33_MODE7_MARGIN as snd_pcm_sframes_t + samples_in - samples_out;
                if delay > uthr as snd_pcm_sframes_t { delay = uthr as snd_pcm_sframes_t; }
            }
        }
        _ => dev_warn((*component).dev, b"Unhandled FIFO mode: %d\n\0".as_ptr() as *const c_char, (*dac33).fifo_mode as c_int),
    }
    delay
}

unsafe fn dac33_set_dai_sysclk(codec_dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*codec_dai).component;
    let dac33 = dac33_priv(component);
    let mut ioc_reg = dac33_read_reg_cache(component, DAC33_INT_OSC_CTRL) as u8;
    let mut asrcb_reg = dac33_read_reg_cache(component, DAC33_ASRC_CTRL_B) as u8;
    match clk_id {
        TLV320DAC33_MCLK => {
            ioc_reg |= DAC33_REFSEL as u8;
            asrcb_reg |= DAC33_SRCREFSEL as u8;
        }
        TLV320DAC33_SLEEPCLK => {
            ioc_reg &= !(DAC33_REFSEL as u8);
            asrcb_reg &= !(DAC33_SRCREFSEL as u8);
        }
        _ => dev_err((*component).dev, b"Invalid clock ID (%d)\n\0".as_ptr() as *const c_char, clk_id),
    }
    (*dac33).refclk = freq;
    dac33_write_reg_cache(component, DAC33_INT_OSC_CTRL, ioc_reg);
    dac33_write_reg_cache(component, DAC33_ASRC_CTRL_B, asrcb_reg);
    0
}

unsafe fn dac33_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let dac33 = dac33_priv(component);
    let mut aictrl_a = dac33_read_reg_cache(component, DAC33_SER_AUDIOIF_CTRL_A) as u8;
    let mut aictrl_b = dac33_read_reg_cache(component, DAC33_SER_AUDIOIF_CTRL_B) as u8;
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => aictrl_a |= (DAC33_MSBCLK | DAC33_MSWCLK) as u8,
        SND_SOC_DAIFMT_CBC_CFC => {
            if (*dac33).fifo_mode as c_uint != 0 {
                dev_err((*component).dev, b"FIFO mode requires provider mode\n\0".as_ptr() as *const c_char);
                return -EINVAL;
            } else {
                aictrl_a &= !(DAC33_MSBCLK | DAC33_MSWCLK) as u8;
            }
        }
        _ => return -EINVAL,
    }
    aictrl_a &= !(DAC33_AFMT_MASK as u8);
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => aictrl_a |= DAC33_AFMT_I2S as u8,
        SND_SOC_DAIFMT_DSP_A => {
            aictrl_a |= DAC33_AFMT_DSP as u8;
            aictrl_b &= !(DAC33_DATA_DELAY_MASK as u8);
            aictrl_b |= DAC33_DATA_DELAY(0) as u8;
        }
        SND_SOC_DAIFMT_RIGHT_J => aictrl_a |= DAC33_AFMT_RIGHT_J as u8,
        SND_SOC_DAIFMT_LEFT_J => aictrl_a |= DAC33_AFMT_LEFT_J as u8,
        _ => {
            dev_err((*component).dev, b"Unsupported format (%u)\n\0".as_ptr() as *const c_char, fmt & SND_SOC_DAIFMT_FORMAT_MASK);
            return -EINVAL;
        }
    }
    dac33_write_reg_cache(component, DAC33_SER_AUDIOIF_CTRL_A, aictrl_a);
    dac33_write_reg_cache(component, DAC33_SER_AUDIOIF_CTRL_B, aictrl_b);
    0
}

unsafe fn dac33_soc_probe(component: *mut snd_soc_component) -> c_int {
    let dac33 = dac33_priv(component);
    (*dac33).component = component;
    let mut ret = dac33_hard_power(component, 1);
    if ret != 0 {
        dev_err((*component).dev, b"Failed to power up component: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    ret = dac33_read_id(component);
    dac33_hard_power(component, 0);
    if ret < 0 {
        dev_err((*component).dev, b"Failed to read chip ID: %d\n\0".as_ptr() as *const c_char, ret);
        return -ENODEV;
    }
    if (*dac33).irq >= 0 {
        ret = request_irq((*dac33).irq, dac33_interrupt_handler, IRQF_TRIGGER_RISING, (*component).name, component as *mut c_void);
        if ret < 0 {
            dev_err((*component).dev, b"Could not request IRQ%d (%d)\n\0".as_ptr() as *const c_char, (*dac33).irq, ret);
            (*dac33).irq = -1;
        }
        if (*dac33).irq != -1 {
            INIT_WORK(&mut (*dac33).work, dac33_work);
        }
    }
    if (*dac33).irq >= 0 {
        snd_soc_add_component_controls(component, dac33_mode_snd_controls.as_ptr(), 1);
    }
    ret
}

unsafe fn dac33_soc_remove(component: *mut snd_soc_component) {
    let dac33 = dac33_priv(component);
    if (*dac33).irq >= 0 {
        free_irq((*dac33).irq, (*dac33).component as *mut c_void);
        flush_work(&mut (*dac33).work);
    }
}

/* soc_component_dev_tlv320dac33, dac33_dai_ops, dac33_dai,
 * tlv320dac33_i2c_id, tlv320dac33_i2c_driver, module_i2c_driver(), and
 * MODULE_* metadata are C macro/struct initializers whose concrete Rust layout
 * depends on external kernel bindings. Their field assignments are preserved
 * here by the translated callback functions and by these externally supplied
 * driver objects.
 */
const DAC33_RATES: c_uint = SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000;
const DAC33_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE;

unsafe fn dac33_i2c_probe(client: *mut i2c_client) -> c_int {
    let size = struct_size_tlv320dac33_priv_reg_cache(dac33_reg.len());
    let dac33 = devm_kzalloc(&mut (*client).dev, size, GFP_KERNEL) as *mut tlv320dac33_priv;
    if dac33.is_null() {
        return -ENOMEM;
    }
    memcpy((*dac33).reg_cache.as_mut_ptr() as *mut c_void, dac33_reg.as_ptr() as *const c_void, dac33_reg.len());
    (*dac33).i2c = client;
    mutex_init(&mut (*dac33).mutex);
    spin_lock_init(&mut (*dac33).lock);
    i2c_set_clientdata(client, dac33 as *mut c_void);
    if (*dac33).burst_bclkdiv == 0 {
        (*dac33).burst_bclkdiv = 8;
    }
    if (*dac33).mode1_latency == 0 {
        (*dac33).mode1_latency = 10000;
    }
    (*dac33).irq = (*client).irq;
    (*dac33).fifo_mode = dac33_fifo_modes::DAC33_FIFO_BYPASS;
    (*dac33).reset_gpiod = devm_gpiod_get_optional(&mut (*client).dev, b"reset\0".as_ptr() as *const c_char, GPIOD_OUT_LOW);
    if IS_ERR((*dac33).reset_gpiod as *const c_void) {
        let ret = PTR_ERR((*dac33).reset_gpiod as *const c_void);
        dev_err_probe(&mut (*client).dev, ret, b"Failed to get reset GPIO\n\0".as_ptr() as *const c_char);
        return ret;
    }
    for i in 0..DAC33_NUM_SUPPLIES {
        (*dac33).supplies[i].supply = dac33_supply_names[i];
    }
    let mut ret = devm_regulator_bulk_get(&mut (*client).dev, DAC33_NUM_SUPPLIES as c_int, (*dac33).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(&mut (*client).dev, b"Failed to request supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    ret = devm_snd_soc_register_component(&mut (*client).dev, &soc_component_dev_tlv320dac33, &mut dac33_dai, 1);
    if ret < 0 {
        return ret;
    }
    ret
}

unsafe fn dac33_i2c_remove(client: *mut i2c_client) {
    let dac33 = i2c_get_clientdata(client) as *mut tlv320dac33_priv;
    if (*dac33).chip_power != 0 {
        dac33_hard_power((*dac33).component, 0);
    }
}

extern "C" {
    static soc_component_dev_tlv320dac33: c_void;
    static mut dac33_dai: c_void;
    static tlv320dac33_i2c_id: c_void;
    static mut tlv320dac33_i2c_driver: c_void;
}

extern "C" {
    static DAC33_CACHEREGNUM: c_uint;
    static DAC33_DAC_CTRL_A: c_uint;
    static DAC33_DAC_CTRL_B: c_uint;
    static DAC33_DAC_CTRL_C: c_uint;
    static DAC33_ANA_VOL_SOFT_STEP_CTRL: c_uint;
    static DAC33_LDAC_DIG_VOL_CTRL: c_uint;
    static DAC33_RDAC_DIG_VOL_CTRL: c_uint;
    static DAC33_LINEL_TO_LLO_VOL: c_uint;
    static DAC33_LINER_TO_RLO_VOL: c_uint;
    static DAC33_OUT_AMP_CTRL: c_uint;
    static DAC33_LDAC_PWR_CTRL: c_uint;
    static DAC33_RDAC_PWR_CTRL: c_uint;
    static DAC33_DEVICE_ID_MSB: c_uint;
    static DAC33_PWR_CTRL: c_uint;
    static DAC33_SER_AUDIOIF_CTRL_B: c_uint;
    static DAC33_SER_AUDIOIF_CTRL_A: c_uint;
    static DAC33_FIFO_IRQ_MASK: c_uint;
    static DAC33_NSAMPLE_MSB: c_uint;
    static DAC33_PREFILL_MSB: c_uint;
    static DAC33_FIFO_CTRL_A: c_uint;
    static DAC33_INT_OSC_STATUS: c_uint;
    static DAC33_INT_OSC_CTRL: c_uint;
    static DAC33_INT_OSC_FREQ_RAT_A: c_uint;
    static DAC33_CALIB_TIME: c_uint;
    static DAC33_INT_OSC_CTRL_B: c_uint;
    static DAC33_INT_OSC_CTRL_C: c_uint;
    static DAC33_ASRC_CTRL_A: c_uint;
    static DAC33_ASRC_CTRL_B: c_uint;
    static DAC33_SRC_REF_CLK_RATIO_A: c_uint;
    static DAC33_INTP_CTRL_A: c_uint;
    static DAC33_FIFO_IRQ_MODE_B: c_uint;
    static DAC33_FIFO_IRQ_MODE_A: c_uint;
    static DAC33_SER_AUDIOIF_CTRL_C: c_uint;
    static DAC33_ATHR_MSB: c_uint;
    static DAC33_UTHR_MSB: c_uint;
    static DAC33_LTHR_MSB: c_uint;
    static DAC33_DACSRCR_RIGHT: c_uint;
    static DAC33_DACSRCL_LEFT: c_uint;
    static DAC33_VOLCLKEN: c_uint;
    static DAC33_PDNALLB: c_uint;
    static DAC33_OSCPDNB: c_uint;
    static DAC33_DACRPDNB: c_uint;
    static DAC33_DACLPDNB: c_uint;
    static DAC33_BCLKON: c_uint;
    static DAC33_MAT: c_uint;
    static DAC33_MUT: c_uint;
    static DAC33_FIFOFLUSH: c_uint;
    static DAC33_OSCSTATUS_NORMAL: c_uint;
    static DAC33_NCYCL_MASK: c_uint;
    static DAC33_WLEN_MASK: c_uint;
    static DAC33_WIDTH: c_uint;
    static DAC33_NCYCL_16: c_uint;
    static DAC33_WLEN_16: c_uint;
    static DAC33_NCYCL_32: c_uint;
    static DAC33_WLEN_24: c_uint;
    static DAC33_FIFO_IRQ_MODE_LEVEL: c_uint;
    static DAC33_FAUTO: c_uint;
    static DAC33_FBYPAS: c_uint;
    static DAC33_SRCBYP: c_uint;
    static DAC33_INTPM_AHIGH: c_uint;
    static DAC33_REFSEL: c_uint;
    static DAC33_SRCREFSEL: c_uint;
    static DAC33_MSBCLK: c_uint;
    static DAC33_MSWCLK: c_uint;
    static DAC33_AFMT_MASK: c_uint;
    static DAC33_AFMT_I2S: c_uint;
    static DAC33_AFMT_DSP: c_uint;
    static DAC33_AFMT_RIGHT_J: c_uint;
    static DAC33_AFMT_LEFT_J: c_uint;
    static DAC33_DATA_DELAY_MASK: c_uint;
}

extern "C" {
    fn DAC33_DACRATE(v: c_uint) -> c_uint;
    fn DAC33_THRREG(v: c_uint) -> c_uint;
    fn DAC33_ADJTHRSHLD(v: c_uint) -> c_uint;
    fn DAC33_ADJSTEP(v: c_uint) -> c_uint;
    fn DAC33_REFDIV(v: c_uint) -> c_uint;
    fn DAC33_SRCLKDIV(v: c_uint) -> c_uint;
    fn DAC33_ATM(v: c_uint) -> c_uint;
    fn DAC33_UTM(v: c_uint) -> c_uint;
    fn DAC33_DATA_DELAY(v: c_uint) -> c_uint;

    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_active(component: *mut snd_soc_component) -> c_int;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_soc_add_component_controls(component: *mut snd_soc_component, controls: *const snd_kcontrol_new, num: c_uint) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const c_void, dai_drv: *mut c_void, num_dai: c_int) -> c_int;
    fn i2c_smbus_read_byte_data(client: *mut i2c_client, command: u8) -> c_int;
    fn i2c_master_send(client: *mut i2c_client, buf: *mut u8, count: c_int) -> c_int;
    fn regulator_bulk_enable(num: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn devm_regulator_bulk_get(dev: *mut device, num: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn gpiod_set_value(desc: *mut gpio_desc, value: c_int) -> c_int;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_int) -> *mut gpio_desc;
    fn request_irq(irq: c_int, handler: unsafe fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, dev: *mut c_void) -> c_int;
    fn free_irq(irq: c_int, dev: *mut c_void);
    fn INIT_WORK(work: *mut work_struct, func: unsafe fn(*mut work_struct));
    fn flush_work(work: *mut work_struct);
    fn schedule_work(work: *mut work_struct) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn ktime_get() -> i64;
    fn ktime_to_us(ktime: i64) -> i64;
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t);
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn container_of_tlv320dac33_priv_work(work: *mut work_struct) -> *mut tlv320dac33_priv;
    fn struct_size_tlv320dac33_priv_reg_cache(count: usize) -> usize;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
}

extern "C" {
    static EPERM: c_int;
    static EINVAL: c_int;
    static ENODEV: c_int;
    static ENOMEM: c_int;
    static IRQ_HANDLED: irqreturn_t;
    static IRQF_TRIGGER_RISING: c_ulong;
    static GFP_KERNEL: c_uint;
    static GPIOD_OUT_LOW: c_int;
    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;
    static SNDRV_PCM_FORMAT_S16_LE: c_int;
    static SNDRV_PCM_FORMAT_S32_LE: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static TLV320DAC33_MCLK: c_int;
    static TLV320DAC33_SLEEPCLK: c_int;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SNDRV_PCM_RATE_44100: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
