// SPDX-License-Identifier: GPL-2.0-only
/*
 * tas5720.c - ALSA SoC Texas Instruments TAS5720 Mono Audio Amplifier
 *
 * Copyright (C)2015-2016 Texas Instruments Incorporated -  https://www.ti.com
 *
 * Author: Andreas Dannenberg <dannenberg@ti.com>
 */

// Dependencies from Linux, ALSA SoC, and "tas5720.h" are expected externally.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

const TAS5720_FAULT_CHECK_INTERVAL: c_uint = 200;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tas572x_type {
    TAS5720 = 0,
    TAS5720A_Q1 = 1,
    TAS5722 = 2,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    pub work: work_struct,
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
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: core::mem::ManuallyDrop<snd_ctl_elem_value_integer>,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub cache_type: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
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
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_desc,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int,
    >,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub no_capture_mute: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

const fn cstr<const N: usize>(bytes: &[u8; N]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

static tas5720_supply_names: [*const c_char; 2] = [
    cstr(b"dvdd\0"), /* Digital power supply. Connect to 3.3-V supply. */
    cstr(b"pvdd\0"), /* Class-D amp and analog power supply (connected). */
];

const TAS5720_NUM_SUPPLIES: usize = tas5720_supply_names.len();

#[repr(C)]
pub struct tas5720_data {
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
    pub devtype: tas572x_type,
    pub supplies: [regulator_bulk_data; TAS5720_NUM_SUPPLIES],
    pub fault_check_work: delayed_work,
    pub last_fault: c_uint,
}

unsafe extern "C" {
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn schedule_delayed_work(work: *mut delayed_work, delay: c_ulong) -> c_int;
    fn msecs_to_jiffies(m: c_uint) -> c_ulong;
    fn regulator_bulk_enable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn INIT_DELAYED_WORK(work: *mut delayed_work, func: unsafe extern "C" fn(*mut work_struct));
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;
    fn msleep(msecs: c_uint);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_get_match_data(client: *mut i2c_client) -> *const c_void;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_regulator_bulk_get(
        dev: *mut device,
        num_consumers: c_int,
        consumers: *mut regulator_bulk_data,
    ) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn __ffs(word: c_uint) -> c_uint;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_crit(dev: *mut device, fmt: *const c_char, ...);
    fn dev_vdbg(dev: *mut device, fmt: *const c_char, ...);
    fn of_match_ptr(ptr: *const of_device_id) -> *const of_device_id;
}

unsafe extern "C" {
    static TAS5720_DIGITAL_CTRL1_REG: c_uint;
    static TAS5720_SSZ_DS: c_uint;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static TAS5720_SAIF_I2S: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static TAS5720_SAIF_LEFTJ: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static TAS5720_SAIF_FORMAT_MASK: c_uint;
    static TAS5720_TDM_CFG_SRC: c_uint;
    static TAS5720_DIGITAL_CTRL2_REG: c_uint;
    static TAS5720_TDM_SLOT_SEL_MASK: c_uint;
    static TAS5722_DIGITAL_CTRL2_REG: c_uint;
    static TAS5722_TDM_SLOT_16B: c_uint;
    static TAS5720_Q1_VOLUME_CTRL_CFG_REG: c_uint;
    static TAS5720_Q1_MUTE: c_uint;
    static TAS5720_MUTE: c_uint;
    static TAS5720_FAULT_REG: c_uint;
    static TAS5720_OCE: c_uint;
    static TAS5720_DCE: c_uint;
    static TAS5720_OTE: c_uint;
    static TAS5720_POWER_CTRL_REG: c_uint;
    static TAS5720_SDZ: c_uint;
    static TAS5720_DEVICE_ID_REG: c_uint;
    static TAS5720_DEVICE_ID: c_uint;
    static TAS5720A_Q1_DEVICE_ID: c_uint;
    static TAS5722_DEVICE_ID: c_uint;
    static TAS5720_ANALOG_CTRL_REG: c_uint;
    static TAS5720_Q1_RESERVED7_BIT: c_uint;
    static TAS5720_MAX_REG: c_uint;
    static TAS5722_MAX_REG: c_uint;
    static REGCACHE_RBTREE: c_uint;
    static TAS5720_VOLUME_CTRL_REG: c_uint;
    static TAS5722_VOL_CONTROL_LSB: c_uint;
    static TAS5720_Q1_VOLUME_CTRL_LEFT_REG: c_uint;
    static TAS5720_Q1_VOLUME_CTRL_RIGHT_REG: c_uint;
    static TAS5720_ANALOG_GAIN_SHIFT: c_uint;
    static SND_SOC_NOPM: c_int;
    static SND_SOC_DAPM_POST_PMU: c_int;
    static SND_SOC_DAPM_PRE_PMD: c_int;
    static SNDRV_PCM_RATE_44100: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_RATE_88200: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S18_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S20_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
}

unsafe extern "C" fn tas5720_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let rate = params_rate(params);
    let ssz_ds: bool;
    let ret: c_int;

    match rate {
        44100 | 48000 => {
            ssz_ds = false;
        }
        88200 | 96000 => {
            ssz_ds = true;
        }
        _ => {
            dev_err((*component).dev, cstr(b"unsupported sample rate: %u\n\0"), rate);
            return -EINVAL;
        }
    }

    ret = snd_soc_component_update_bits(component, TAS5720_DIGITAL_CTRL1_REG, TAS5720_SSZ_DS, ssz_ds as c_uint);
    if ret < 0 {
        dev_err((*component).dev, cstr(b"error setting sample rate: %d\n\0"), ret);
        return ret;
    }

    0
}

unsafe extern "C" fn tas5720_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let serial_format: u8;
    let ret: c_int;

    if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) != SND_SOC_DAIFMT_CBC_CFC {
        dev_vdbg((*component).dev, cstr(b"DAI clocking invalid\n\0"));
        return -EINVAL;
    }

    match fmt & (SND_SOC_DAIFMT_FORMAT_MASK | SND_SOC_DAIFMT_INV_MASK) {
        x if x == (SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF) => {
            /* 1st data bit occur one BCLK cycle after the frame sync */
            serial_format = TAS5720_SAIF_I2S as u8;
        }
        x if x == (SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_NB_NF) => {
            /*
             * Note that although the TAS5720 does not have a dedicated DSP
             * mode it doesn't care about the LRCLK duty cycle during TDM
             * operation. Therefore we can use the device's I2S mode with
             * its delaying of the 1st data bit to receive DSP_A formatted
             * data. See device datasheet for additional details.
             */
            serial_format = TAS5720_SAIF_I2S as u8;
        }
        x if x == (SND_SOC_DAIFMT_DSP_B | SND_SOC_DAIFMT_NB_NF) => {
            /*
             * Similar to DSP_A, we can use the fact that the TAS5720 does
             * not care about the LRCLK duty cycle during TDM to receive
             * DSP_B formatted data in LEFTJ mode (no delaying of the 1st
             * data bit).
             */
            serial_format = TAS5720_SAIF_LEFTJ as u8;
        }
        x if x == (SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_NB_NF) => {
            /* No delay after the frame sync */
            serial_format = TAS5720_SAIF_LEFTJ as u8;
        }
        _ => {
            dev_vdbg((*component).dev, cstr(b"DAI Format is not found\n\0"));
            return -EINVAL;
        }
    }

    ret = snd_soc_component_update_bits(
        component,
        TAS5720_DIGITAL_CTRL1_REG,
        TAS5720_SAIF_FORMAT_MASK,
        serial_format as c_uint,
    );
    if ret < 0 {
        dev_err((*component).dev, cstr(b"error setting SAIF format: %d\n\0"), ret);
        return ret;
    }

    0
}

unsafe extern "C" fn tas5720_set_dai_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    _rx_mask: c_uint,
    _slots: c_int,
    slot_width: c_int,
) -> c_int {
    let component = (*dai).component;
    let tas5720 = snd_soc_component_get_drvdata(component) as *mut tas5720_data;
    let first_slot: c_uint;
    let mut ret: c_int;

    if tx_mask == 0 {
        dev_err((*component).dev, cstr(b"tx masks must not be 0\n\0"));
        return -EINVAL;
    }

    /*
     * Determine the first slot that is being requested. We will only
     * use the first slot that is found since the TAS5720 is a mono
     * amplifier.
     */
    first_slot = __ffs(tx_mask);

    if first_slot > 7 {
        dev_err((*component).dev, cstr(b"slot selection out of bounds (%u)\n\0"), first_slot);
        return -EINVAL;
    }

    /*
     * Enable manual TDM slot selection (instead of I2C ID based).
     * This is not applicable to TAS5720A-Q1.
     */
    match (*tas5720).devtype {
        tas572x_type::TAS5720A_Q1 => {}
        _ => {
            ret = snd_soc_component_update_bits(
                component,
                TAS5720_DIGITAL_CTRL1_REG,
                TAS5720_TDM_CFG_SRC,
                TAS5720_TDM_CFG_SRC,
            );
            if ret < 0 {
                dev_err((*component).dev, cstr(b"error configuring TDM mode: %d\n\0"), ret);
                return ret;
            }

            /* Configure the TDM slot to process audio from */
            ret = snd_soc_component_update_bits(
                component,
                TAS5720_DIGITAL_CTRL2_REG,
                TAS5720_TDM_SLOT_SEL_MASK,
                first_slot,
            );
            if ret < 0 {
                dev_err((*component).dev, cstr(b"error configuring TDM mode: %d\n\0"), ret);
                return ret;
            }
        }
    }

    /* Configure TDM slot width. This is only applicable to TAS5722. */
    match (*tas5720).devtype {
        tas572x_type::TAS5722 => {
            ret = snd_soc_component_update_bits(
                component,
                TAS5722_DIGITAL_CTRL2_REG,
                TAS5722_TDM_SLOT_16B,
                if slot_width == 16 { TAS5722_TDM_SLOT_16B } else { 0 },
            );
            if ret < 0 {
                dev_err((*component).dev, cstr(b"error configuring TDM mode: %d\n\0"), ret);
                return ret;
            }
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn tas5720_mute_soc_component(component: *mut snd_soc_component, mute: c_int) -> c_int {
    let tas5720 = snd_soc_component_get_drvdata(component) as *mut tas5720_data;
    let reg: c_uint;
    let mask: c_uint;
    let ret: c_int;

    match (*tas5720).devtype {
        tas572x_type::TAS5720A_Q1 => {
            reg = TAS5720_Q1_VOLUME_CTRL_CFG_REG;
            mask = TAS5720_Q1_MUTE;
        }
        _ => {
            reg = TAS5720_DIGITAL_CTRL2_REG;
            mask = TAS5720_MUTE;
        }
    }

    ret = snd_soc_component_update_bits(component, reg, mask, if mute != 0 { mask } else { 0 });
    if ret < 0 {
        dev_err((*component).dev, cstr(b"error (un-)muting device: %d\n\0"), ret);
        return ret;
    }

    0
}

unsafe extern "C" fn tas5720_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    tas5720_mute_soc_component((*dai).component, mute)
}

unsafe extern "C" fn tas5720_fault_check_work(work: *mut work_struct) {
    let tas5720 = (work as *mut u8).sub(core::mem::offset_of!(tas5720_data, fault_check_work) + core::mem::offset_of!(delayed_work, work)) as *mut tas5720_data;
    let dev = (*(*tas5720).component).dev;
    let mut curr_fault: c_uint = 0;
    let mut ret: c_int;

    ret = regmap_read((*tas5720).regmap, TAS5720_FAULT_REG, &mut curr_fault);
    if ret < 0 {
        dev_err(dev, cstr(b"failed to read FAULT register: %d\n\0"), ret);
        schedule_delayed_work(&mut (*tas5720).fault_check_work, msecs_to_jiffies(TAS5720_FAULT_CHECK_INTERVAL));
        return;
    }

    /* Check/handle all errors except SAIF clock errors */
    curr_fault &= TAS5720_OCE | TAS5720_DCE | TAS5720_OTE;

    /*
     * Only flag errors once for a given occurrence. This is needed as
     * the TAS5720 will take time clearing the fault condition internally
     * during which we don't want to bombard the system with the same
     * error message over and over.
     */
    if (curr_fault & TAS5720_OCE) != 0 && ((*tas5720).last_fault & TAS5720_OCE) == 0 {
        dev_crit(dev, cstr(b"experienced an over current hardware fault\n\0"));
    }

    if (curr_fault & TAS5720_DCE) != 0 && ((*tas5720).last_fault & TAS5720_DCE) == 0 {
        dev_crit(dev, cstr(b"experienced a DC detection fault\n\0"));
    }

    if (curr_fault & TAS5720_OTE) != 0 && ((*tas5720).last_fault & TAS5720_OTE) == 0 {
        dev_crit(dev, cstr(b"experienced an over temperature fault\n\0"));
    }

    /* Store current fault value so we can detect any changes next time */
    (*tas5720).last_fault = curr_fault;

    if curr_fault != 0 {
        /*
         * Periodically toggle SDZ (shutdown bit) H->L->H to clear any latching
         * faults as long as a fault condition persists. Always going through
         * the full sequence no matter the first return value to minimizes
         * chances for the device to end up in shutdown mode.
         */
        ret = regmap_write_bits((*tas5720).regmap, TAS5720_POWER_CTRL_REG, TAS5720_SDZ, 0);
        if ret < 0 {
            dev_err(dev, cstr(b"failed to write POWER_CTRL register: %d\n\0"), ret);
        }

        ret = regmap_write_bits((*tas5720).regmap, TAS5720_POWER_CTRL_REG, TAS5720_SDZ, TAS5720_SDZ);
        if ret < 0 {
            dev_err(dev, cstr(b"failed to write POWER_CTRL register: %d\n\0"), ret);
        }
    }

    /* Schedule the next fault check at the specified interval */
    schedule_delayed_work(&mut (*tas5720).fault_check_work, msecs_to_jiffies(TAS5720_FAULT_CHECK_INTERVAL));
}

unsafe extern "C" fn tas5720_codec_probe(component: *mut snd_soc_component) -> c_int {
    let tas5720 = snd_soc_component_get_drvdata(component) as *mut tas5720_data;
    let mut device_id: c_uint = 0;
    let expected_device_id: c_uint;
    let mut ret: c_int;

    (*tas5720).component = component;

    ret = regulator_bulk_enable(TAS5720_NUM_SUPPLIES as c_int, (*tas5720).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err((*component).dev, cstr(b"failed to enable supplies: %d\n\0"), ret);
        return ret;
    }

    /*
     * Take a liberal approach to checking the device ID to allow the
     * driver to be used even if the device ID does not match, however
     * issue a warning if there is a mismatch.
     */
    ret = regmap_read((*tas5720).regmap, TAS5720_DEVICE_ID_REG, &mut device_id);
    if ret < 0 {
        dev_err((*component).dev, cstr(b"failed to read device ID register: %d\n\0"), ret);
        regulator_bulk_disable(TAS5720_NUM_SUPPLIES as c_int, (*tas5720).supplies.as_mut_ptr());
        return ret;
    }

    match (*tas5720).devtype {
        tas572x_type::TAS5720 => expected_device_id = TAS5720_DEVICE_ID,
        tas572x_type::TAS5720A_Q1 => expected_device_id = TAS5720A_Q1_DEVICE_ID,
        tas572x_type::TAS5722 => expected_device_id = TAS5722_DEVICE_ID,
    }

    if device_id != expected_device_id {
        dev_warn(
            (*component).dev,
            cstr(b"wrong device ID. expected: %u read: %u\n\0"),
            expected_device_id,
            device_id,
        );
    }

    /* Set device to mute */
    ret = tas5720_mute_soc_component(component, 1);
    if ret < 0 {
        dev_err((*component).dev, cstr(b"error configuring device registers: %d\n\0"), ret);
        regulator_bulk_disable(TAS5720_NUM_SUPPLIES as c_int, (*tas5720).supplies.as_mut_ptr());
        return ret;
    }

    /* Set Bit 7 in TAS5720_ANALOG_CTRL_REG to 1 for TAS5720A_Q1 */
    match (*tas5720).devtype {
        tas572x_type::TAS5720A_Q1 => {
            ret = snd_soc_component_update_bits(
                component,
                TAS5720_ANALOG_CTRL_REG,
                TAS5720_Q1_RESERVED7_BIT,
                TAS5720_Q1_RESERVED7_BIT,
            );
        }
        _ => {}
    }
    if ret < 0 {
        dev_err((*component).dev, cstr(b"error configuring device registers: %d\n\0"), ret);
        regulator_bulk_disable(TAS5720_NUM_SUPPLIES as c_int, (*tas5720).supplies.as_mut_ptr());
        return ret;
    }

    /*
     * Enter shutdown mode - our default when not playing audio - to
     * minimize current consumption. On the TAS5720 there is no real down
     * side doing so as all device registers are preserved and the wakeup
     * of the codec is rather quick which we do using a dapm widget.
     */
    ret = snd_soc_component_update_bits(component, TAS5720_POWER_CTRL_REG, TAS5720_SDZ, 0);
    if ret < 0 {
        dev_err((*component).dev, cstr(b"error configuring device registers: %d\n\0"), ret);
        regulator_bulk_disable(TAS5720_NUM_SUPPLIES as c_int, (*tas5720).supplies.as_mut_ptr());
        return ret;
    }

    INIT_DELAYED_WORK(&mut (*tas5720).fault_check_work, tas5720_fault_check_work);

    0
}

unsafe extern "C" fn tas5720_codec_remove(component: *mut snd_soc_component) {
    let tas5720 = snd_soc_component_get_drvdata(component) as *mut tas5720_data;
    let ret: c_int;

    cancel_delayed_work_sync(&mut (*tas5720).fault_check_work);

    ret = regulator_bulk_disable(TAS5720_NUM_SUPPLIES as c_int, (*tas5720).supplies.as_mut_ptr());
    if ret < 0 {
        dev_err((*component).dev, cstr(b"failed to disable supplies: %d\n\0"), ret);
    }
}

unsafe extern "C" fn tas5720_dac_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let tas5720 = snd_soc_component_get_drvdata(component) as *mut tas5720_data;
    let ret: c_int;

    if (event & SND_SOC_DAPM_POST_PMU) != 0 {
        /* Take TAS5720 out of shutdown mode */
        let r = snd_soc_component_update_bits(component, TAS5720_POWER_CTRL_REG, TAS5720_SDZ, TAS5720_SDZ);
        if r < 0 {
            dev_err((*component).dev, cstr(b"error waking component: %d\n\0"), r);
            return r;
        }

        /*
         * Observe codec shutdown-to-active time. The datasheet only
         * lists a nominal value however just use-it as-is without
         * additional padding to minimize the delay introduced in
         * starting to play audio (actually there is other setup done
         * by the ASoC framework that will provide additional delays,
         * so we should always be safe).
         */
        msleep(25);

        /* Turn on TAS5720 periodic fault checking/handling */
        (*tas5720).last_fault = 0;
        schedule_delayed_work(
            &mut (*tas5720).fault_check_work,
            msecs_to_jiffies(TAS5720_FAULT_CHECK_INTERVAL),
        );
    } else if (event & SND_SOC_DAPM_PRE_PMD) != 0 {
        /* Disable TAS5720 periodic fault checking/handling */
        cancel_delayed_work_sync(&mut (*tas5720).fault_check_work);

        /* Place TAS5720 in shutdown mode to minimize current draw */
        ret = snd_soc_component_update_bits(component, TAS5720_POWER_CTRL_REG, TAS5720_SDZ, 0);
        if ret < 0 {
            dev_err((*component).dev, cstr(b"error shutting down component: %d\n\0"), ret);
            return ret;
        }
    }

    0
}

// CONFIG_PM conditional in C maps these callbacks to NULL when disabled.
unsafe extern "C" fn tas5720_suspend(component: *mut snd_soc_component) -> c_int {
    let tas5720 = snd_soc_component_get_drvdata(component) as *mut tas5720_data;
    let ret: c_int;

    regcache_cache_only((*tas5720).regmap, true);
    regcache_mark_dirty((*tas5720).regmap);

    ret = regulator_bulk_disable(TAS5720_NUM_SUPPLIES as c_int, (*tas5720).supplies.as_mut_ptr());
    if ret < 0 {
        dev_err((*component).dev, cstr(b"failed to disable supplies: %d\n\0"), ret);
    }

    ret
}

unsafe extern "C" fn tas5720_resume(component: *mut snd_soc_component) -> c_int {
    let tas5720 = snd_soc_component_get_drvdata(component) as *mut tas5720_data;
    let mut ret: c_int;

    ret = regulator_bulk_enable(TAS5720_NUM_SUPPLIES as c_int, (*tas5720).supplies.as_mut_ptr());
    if ret < 0 {
        dev_err((*component).dev, cstr(b"failed to enable supplies: %d\n\0"), ret);
        return ret;
    }

    regcache_cache_only((*tas5720).regmap, false);

    ret = regcache_sync((*tas5720).regmap);
    if ret < 0 {
        dev_err((*component).dev, cstr(b"failed to sync regcache: %d\n\0"), ret);
        return ret;
    }

    0
}

unsafe extern "C" fn tas5720_is_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    if reg == TAS5720_DEVICE_ID_REG || reg == TAS5720_FAULT_REG {
        true
    } else {
        false
    }
}

static tas5720_regmap_config: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: unsafe { TAS5720_MAX_REG },
    cache_type: unsafe { REGCACHE_RBTREE },
    volatile_reg: Some(tas5720_is_volatile_reg),
};

static tas5720a_q1_regmap_config: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: unsafe { TAS5720_MAX_REG },
    cache_type: unsafe { REGCACHE_RBTREE },
    volatile_reg: Some(tas5720_is_volatile_reg),
};

static tas5722_regmap_config: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: unsafe { TAS5722_MAX_REG },
    cache_type: unsafe { REGCACHE_RBTREE },
    volatile_reg: Some(tas5720_is_volatile_reg),
};

/*
 * DAC analog gain. There are four discrete values to select from, ranging
 * from 19.2 dB to 26.3dB.
 */
static dac_analog_tlv: &[c_uint] = DECLARE_TLV_DB_RANGE!(
    0x0, 0x0, TLV_DB_SCALE_ITEM!(1920, 0, 0),
    0x1, 0x1, TLV_DB_SCALE_ITEM!(2070, 0, 0),
    0x2, 0x2, TLV_DB_SCALE_ITEM!(2350, 0, 0),
    0x3, 0x3, TLV_DB_SCALE_ITEM!(2630, 0, 0),
);

/*
 * DAC analog gain for TAS5720A-Q1. There are three discrete values to select from, ranging
 * from 19.2 dB to 25.0dB.
 */
static dac_analog_tlv_a_q1: &[c_uint] = DECLARE_TLV_DB_RANGE!(
    0x0, 0x0, TLV_DB_SCALE_ITEM!(1920, 0, 0),
    0x1, 0x1, TLV_DB_SCALE_ITEM!(2260, 0, 0),
    0x2, 0x2, TLV_DB_SCALE_ITEM!(2500, 0, 0),
);

/*
 * DAC digital volumes. From -103.5 to 24 dB in 0.5 dB or 0.25 dB steps
 * depending on the device. Note that setting the gain below -100 dB
 * (register value <0x7) is effectively a MUTE as per device datasheet.
 *
 * Note that for the TAS5722 the digital volume controls are actually split
 * over two registers, so we need custom getters/setters for access.
 */
static tas5720_dac_tlv: &[c_uint] = DECLARE_TLV_DB_SCALE!(-10350, 50, 0);
static tas5722_dac_tlv: &[c_uint] = DECLARE_TLV_DB_SCALE!(-10350, 25, 0);

unsafe extern "C" fn tas5722_volume_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let mut val: c_uint;

    val = snd_soc_component_read(component, TAS5720_VOLUME_CTRL_REG);
    (*ucontrol).value.integer.value[0] = (val << 1) as c_long;

    val = snd_soc_component_read(component, TAS5722_DIGITAL_CTRL2_REG);
    (*ucontrol).value.integer.value[0] |= (val & TAS5722_VOL_CONTROL_LSB) as c_long;

    0
}

unsafe extern "C" fn tas5722_volume_set(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let sel = (*ucontrol).value.integer.value[0] as c_uint;

    snd_soc_component_write(component, TAS5720_VOLUME_CTRL_REG, sel >> 1);
    snd_soc_component_update_bits(component, TAS5722_DIGITAL_CTRL2_REG, TAS5722_VOL_CONTROL_LSB, sel);

    0
}

static tas5720_snd_controls: &[snd_kcontrol_new] = &[
    SOC_SINGLE_TLV!("Speaker Driver Playback Volume", TAS5720_VOLUME_CTRL_REG, 0, 0xff, 0, tas5720_dac_tlv),
    SOC_SINGLE_TLV!("Speaker Driver Analog Gain", TAS5720_ANALOG_CTRL_REG, TAS5720_ANALOG_GAIN_SHIFT, 3, 0, dac_analog_tlv),
];

static tas5720a_q1_snd_controls: &[snd_kcontrol_new] = &[
    SOC_DOUBLE_R_TLV!(
        "Speaker Driver Playback Volume",
        TAS5720_Q1_VOLUME_CTRL_LEFT_REG,
        TAS5720_Q1_VOLUME_CTRL_RIGHT_REG,
        0,
        0xff,
        0,
        tas5720_dac_tlv
    ),
    SOC_SINGLE_TLV!("Speaker Driver Analog Gain", TAS5720_ANALOG_CTRL_REG, TAS5720_ANALOG_GAIN_SHIFT, 3, 0, dac_analog_tlv_a_q1),
];

static tas5722_snd_controls: &[snd_kcontrol_new] = &[
    SOC_SINGLE_EXT_TLV!(
        "Speaker Driver Playback Volume",
        0,
        0,
        511,
        0,
        tas5722_volume_get,
        tas5722_volume_set,
        tas5722_dac_tlv
    ),
    SOC_SINGLE_TLV!("Speaker Driver Analog Gain", TAS5720_ANALOG_CTRL_REG, TAS5720_ANALOG_GAIN_SHIFT, 3, 0, dac_analog_tlv),
];

static tas5720_dapm_widgets: &[snd_soc_dapm_widget_desc] = &[
    SND_SOC_DAPM_AIF_IN!("DAC IN", "Playback", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_DAC_E!(
        "DAC",
        ptr::null::<c_char>(),
        SND_SOC_NOPM,
        0,
        0,
        tas5720_dac_event,
        SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD
    ),
    SND_SOC_DAPM_OUTPUT!("OUT"),
];

static tas5720_audio_map: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route { sink: cstr(b"DAC\0"), control: ptr::null(), source: cstr(b"DAC IN\0") },
    snd_soc_dapm_route { sink: cstr(b"OUT\0"), control: ptr::null(), source: cstr(b"DAC\0") },
];

static soc_component_dev_tas5720: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(tas5720_codec_probe),
    remove: Some(tas5720_codec_remove),
    suspend: Some(tas5720_suspend),
    resume: Some(tas5720_resume),
    controls: tas5720_snd_controls.as_ptr(),
    num_controls: tas5720_snd_controls.len() as c_uint,
    dapm_widgets: tas5720_dapm_widgets.as_ptr(),
    num_dapm_widgets: tas5720_dapm_widgets.len() as c_uint,
    dapm_routes: tas5720_audio_map.as_ptr(),
    num_dapm_routes: tas5720_audio_map.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static soc_component_dev_tas5720_a_q1: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(tas5720_codec_probe),
    remove: Some(tas5720_codec_remove),
    suspend: Some(tas5720_suspend),
    resume: Some(tas5720_resume),
    controls: tas5720a_q1_snd_controls.as_ptr(),
    num_controls: tas5720a_q1_snd_controls.len() as c_uint,
    dapm_widgets: tas5720_dapm_widgets.as_ptr(),
    num_dapm_widgets: tas5720_dapm_widgets.len() as c_uint,
    dapm_routes: tas5720_audio_map.as_ptr(),
    num_dapm_routes: tas5720_audio_map.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static soc_component_dev_tas5722: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(tas5720_codec_probe),
    remove: Some(tas5720_codec_remove),
    suspend: Some(tas5720_suspend),
    resume: Some(tas5720_resume),
    controls: tas5722_snd_controls.as_ptr(),
    num_controls: tas5722_snd_controls.len() as c_uint,
    dapm_widgets: tas5720_dapm_widgets.as_ptr(),
    num_dapm_widgets: tas5720_dapm_widgets.len() as c_uint,
    dapm_routes: tas5720_audio_map.as_ptr(),
    num_dapm_routes: tas5720_audio_map.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

/* PCM rates supported by the TAS5720 driver */
static TAS5720_RATES: c_uint = unsafe {
    SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000
};

/* Formats supported by TAS5720 driver */
static TAS5720_FORMATS: c_uint = unsafe {
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S18_3LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE
};

static tas5720_speaker_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(tas5720_hw_params),
    set_fmt: Some(tas5720_set_dai_fmt),
    set_tdm_slot: Some(tas5720_set_dai_tdm_slot),
    mute_stream: Some(tas5720_mute),
    no_capture_mute: 1,
};

/*
 * TAS5720 DAI structure
 *
 * Note that were are advertising .playback.channels_max = 2 despite this being
 * a mono amplifier. The reason for that is that some serial ports such as TI's
 * McASP module have a minimum number of channels (2) that they can output.
 * Advertising more channels than we have will allow us to interface with such
 * a serial port without really any negative side effects as the TAS5720 will
 * simply ignore any extra channel(s) asides from the one channel that is
 * configured to be played back.
 */
static mut tas5720_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: cstr(b"tas5720-amplifier\0"),
    playback: snd_soc_pcm_stream {
        stream_name: cstr(b"Playback\0"),
        channels_min: 1,
        channels_max: 2,
        rates: unsafe { TAS5720_RATES },
        formats: unsafe { TAS5720_FORMATS },
    },
    ops: &tas5720_speaker_dai_ops,
}];

static tas5720_id: [i2c_device_id; 4] = [
    i2c_device_id { name: cstr(b"tas5720\0"), driver_data: tas572x_type::TAS5720 as c_ulong },
    i2c_device_id { name: cstr(b"tas5720a-q1\0"), driver_data: tas572x_type::TAS5720A_Q1 as c_ulong },
    i2c_device_id { name: cstr(b"tas5722\0"), driver_data: tas572x_type::TAS5722 as c_ulong },
    i2c_device_id { name: ptr::null(), driver_data: 0 },
];
MODULE_DEVICE_TABLE!(i2c, tas5720_id);

unsafe extern "C" fn tas5720_probe(client: *mut i2c_client) -> c_int {
    let dev = &mut (*client).dev as *mut device;
    let data: *mut tas5720_data;
    let regmap_config: *const regmap_config;
    let mut ret: c_int;
    let mut i: c_int;

    data = devm_kzalloc(dev, core::mem::size_of::<tas5720_data>(), GFP_KERNEL) as *mut tas5720_data;
    if data.is_null() {
        return -ENOMEM;
    }

    (*data).devtype = core::mem::transmute::<usize, tas572x_type>(i2c_get_match_data(client) as usize);

    match (*data).devtype {
        tas572x_type::TAS5720 => regmap_config = &tas5720_regmap_config,
        tas572x_type::TAS5720A_Q1 => regmap_config = &tas5720a_q1_regmap_config,
        tas572x_type::TAS5722 => regmap_config = &tas5722_regmap_config,
    }
    (*data).regmap = devm_regmap_init_i2c(client, regmap_config);
    if IS_ERR((*data).regmap as *const c_void) {
        ret = PTR_ERR((*data).regmap as *const c_void);
        dev_err(dev, cstr(b"failed to allocate register map: %d\n\0"), ret);
        return ret;
    }

    i = 0;
    while (i as usize) < TAS5720_NUM_SUPPLIES {
        (*data).supplies[i as usize].supply = tas5720_supply_names[i as usize];
        i += 1;
    }

    ret = devm_regulator_bulk_get(dev, TAS5720_NUM_SUPPLIES as c_int, (*data).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, cstr(b"failed to request supplies: %d\n\0"), ret);
        return ret;
    }

    dev_set_drvdata(dev, data as *mut c_void);

    match (*data).devtype {
        tas572x_type::TAS5720 => {
            ret = devm_snd_soc_register_component(
                &mut (*client).dev,
                &soc_component_dev_tas5720,
                tas5720_dai.as_mut_ptr(),
                tas5720_dai.len() as c_int,
            );
        }
        tas572x_type::TAS5720A_Q1 => {
            ret = devm_snd_soc_register_component(
                &mut (*client).dev,
                &soc_component_dev_tas5720_a_q1,
                tas5720_dai.as_mut_ptr(),
                tas5720_dai.len() as c_int,
            );
        }
        tas572x_type::TAS5722 => {
            ret = devm_snd_soc_register_component(
                &mut (*client).dev,
                &soc_component_dev_tas5722,
                tas5720_dai.as_mut_ptr(),
                tas5720_dai.len() as c_int,
            );
        }
    }
    if ret < 0 {
        dev_err(dev, cstr(b"failed to register component: %d\n\0"), ret);
        return ret;
    }

    0
}

// IS_ENABLED(CONFIG_OF)
static tas5720_of_match: [of_device_id; 4] = [
    of_device_id { compatible: cstr(b"ti,tas5720\0") },
    of_device_id { compatible: cstr(b"ti,tas5720a-q1\0") },
    of_device_id { compatible: cstr(b"ti,tas5722\0") },
    of_device_id { compatible: ptr::null() },
];
MODULE_DEVICE_TABLE!(of, tas5720_of_match);

static mut tas5720_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: cstr(b"tas5720\0"),
        of_match_table: unsafe { of_match_ptr(tas5720_of_match.as_ptr()) },
    },
    probe: Some(tas5720_probe),
    id_table: tas5720_id.as_ptr(),
};

module_i2c_driver!(tas5720_i2c_driver);

MODULE_AUTHOR!("Andreas Dannenberg <dannenberg@ti.com>");
MODULE_DESCRIPTION!("TAS5720 Audio amplifier driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
