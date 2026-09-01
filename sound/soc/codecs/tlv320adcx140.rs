// SPDX-License-Identifier: GPL-2.0
// TLV320ADCX140 Sound driver
// Copyright (C) 2020 Texas Instruments Incorporated - https://www.ti.com/
//
// Rust translation of soc/codecs/tlv320adcx140.c.
// Kernel, ASoC, regmap, GPIO, regulator, OF, and codec register symbols are
// provided by external bindings corresponding to the original C includes and
// "tlv320adcx140.h".

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

extern "C" {
    static ADCX140_PAGE_SELECT: c_uint;
    static ADCX140_SW_RESET: c_uint;
    static ADCX140_SLEEP_CFG: c_uint;
    static ADCX140_SHDN_CFG: c_uint;
    static ADCX140_ASI_CFG0: c_uint;
    static ADCX140_ASI_CFG1: c_uint;
    static ADCX140_ASI_CFG2: c_uint;
    static ADCX140_ASI_CH1: c_uint;
    static ADCX140_ASI_CH2: c_uint;
    static ADCX140_ASI_CH3: c_uint;
    static ADCX140_ASI_CH4: c_uint;
    static ADCX140_ASI_CH5: c_uint;
    static ADCX140_ASI_CH6: c_uint;
    static ADCX140_ASI_CH7: c_uint;
    static ADCX140_ASI_CH8: c_uint;
    static ADCX140_MST_CFG0: c_uint;
    static ADCX140_MST_CFG1: c_uint;
    static ADCX140_ASI_STS: c_uint;
    static ADCX140_CLK_SRC: c_uint;
    static ADCX140_PDMCLK_CFG: c_uint;
    static ADCX140_PDM_CFG: c_uint;
    static ADCX140_GPIO_CFG0: c_uint;
    static ADCX140_GPO_CFG0: c_uint;
    static ADCX140_GPO_CFG1: c_uint;
    static ADCX140_GPO_CFG2: c_uint;
    static ADCX140_GPO_CFG3: c_uint;
    static ADCX140_GPO_VAL: c_uint;
    static ADCX140_GPIO_MON: c_uint;
    static ADCX140_GPI_CFG0: c_uint;
    static ADCX140_GPI_CFG1: c_uint;
    static ADCX140_GPI_MON: c_uint;
    static ADCX140_INT_CFG: c_uint;
    static ADCX140_INT_MASK0: c_uint;
    static ADCX140_INT_LTCH0: c_uint;
    static ADCX140_BIAS_CFG: c_uint;
    static ADCX140_CH1_CFG0: c_uint;
    static ADCX140_CH1_CFG1: c_uint;
    static ADCX140_CH1_CFG2: c_uint;
    static ADCX140_CH1_CFG3: c_uint;
    static ADCX140_CH1_CFG4: c_uint;
    static ADCX140_CH2_CFG0: c_uint;
    static ADCX140_CH2_CFG1: c_uint;
    static ADCX140_CH2_CFG2: c_uint;
    static ADCX140_CH2_CFG3: c_uint;
    static ADCX140_CH2_CFG4: c_uint;
    static ADCX140_CH3_CFG0: c_uint;
    static ADCX140_CH3_CFG1: c_uint;
    static ADCX140_CH3_CFG2: c_uint;
    static ADCX140_CH3_CFG3: c_uint;
    static ADCX140_CH3_CFG4: c_uint;
    static ADCX140_CH4_CFG0: c_uint;
    static ADCX140_CH4_CFG1: c_uint;
    static ADCX140_CH4_CFG2: c_uint;
    static ADCX140_CH4_CFG3: c_uint;
    static ADCX140_CH4_CFG4: c_uint;
    static ADCX140_CH5_CFG2: c_uint;
    static ADCX140_CH5_CFG3: c_uint;
    static ADCX140_CH5_CFG4: c_uint;
    static ADCX140_CH6_CFG2: c_uint;
    static ADCX140_CH6_CFG3: c_uint;
    static ADCX140_CH6_CFG4: c_uint;
    static ADCX140_CH7_CFG2: c_uint;
    static ADCX140_CH7_CFG3: c_uint;
    static ADCX140_CH7_CFG4: c_uint;
    static ADCX140_CH8_CFG2: c_uint;
    static ADCX140_CH8_CFG3: c_uint;
    static ADCX140_CH8_CFG4: c_uint;
    static ADCX140_DSP_CFG0: c_uint;
    static ADCX140_DSP_CFG1: c_uint;
    static ADCX140_DRE_CFG0: c_uint;
    static ADCX140_AGC_CFG0: c_uint;
    static ADCX140_IN_CH_EN: c_uint;
    static ADCX140_ASI_OUT_CH_EN: c_uint;
    static ADCX140_PWR_CFG: c_uint;
    static ADCX140_DEV_STS0: c_uint;
    static ADCX140_DEV_STS1: c_uint;
    static ADCX140_PHASE_CALIB: c_uint;
}

#[repr(C)]
pub struct regulator;
#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
}
#[repr(C)]
pub struct gpio_desc;
#[repr(C)]
pub struct regmap;
#[repr(C)]
pub struct device;
#[repr(C)]
pub struct snd_kcontrol;
#[repr(C)]
pub struct snd_ctl_elem_info;
#[repr(C)]
pub struct snd_ctl_elem_value;
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_pcm_substream;
#[repr(C)]
pub struct snd_pcm_hw_params;
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}
#[repr(C)]
pub struct snd_soc_dapm_context;
#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct adcx140_priv {
    pub supply_areg: *mut regulator,
    pub supplies: [regulator_bulk_data; ADCX140_NUM_SUPPLIES],
    pub gpio_reset: *mut gpio_desc,
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub micbias_vg: bool,
    pub phase_calib_on: bool,
    pub dai_fmt: c_uint,
    pub slot_width: c_uint,
}

const ADCX140_NUM_SUPPLIES: usize = 2;
static adcx140_supply_names: [*const c_char; ADCX140_NUM_SUPPLIES] =
    [b"avdd\0".as_ptr() as *const c_char, b"iovdd\0".as_ptr() as *const c_char];

static gpo_config_names: [*const c_char; 4] = [
    b"ti,gpo-config-1\0".as_ptr() as *const c_char,
    b"ti,gpo-config-2\0".as_ptr() as *const c_char,
    b"ti,gpo-config-3\0".as_ptr() as *const c_char,
    b"ti,gpo-config-4\0".as_ptr() as *const c_char,
];

// The original file declares static regmap defaults/ranges/access tables and
// many ASoC control/widget/route tables using Linux macros such as
// regmap_reg_range(), DECLARE_TLV_DB_SCALE(), SOC_ENUM_SINGLE_DECL(),
// SOC_DAPM_*(), SOC_SINGLE*(), SND_SOC_DAPM_*(), and MODULE_*().
// These declarations are preserved as external macro-shaped dependencies in
// this source-level translation; the function bodies below reference the same
// symbols and preserve the driver behavior.

unsafe extern "C" {
    fn gpiod_direction_output(desc: *mut gpio_desc, value: c_int) -> c_int;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn usleep_range(min: c_uint, max: c_uint);
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regulator_bulk_disable(num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_enable(num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn regulator_disable(r: *mut regulator) -> c_int;
    fn regulator_enable(r: *mut regulator) -> c_int;
    fn device_property_read_u32(dev: *mut device, prop: *const c_char, val: *mut u32) -> c_int;
    fn device_property_read_u32_array(
        dev: *mut device,
        prop: *const c_char,
        vals: *mut u32,
        nval: c_int,
    ) -> c_int;
    fn device_property_count_u32(dev: *mut device, prop: *const c_char) -> c_int;
    fn device_property_read_bool(dev: *mut device, prop: *const c_char) -> bool;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut adcx140_priv;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn params_physical_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regulator_bulk_get(
        dev: *mut device,
        num: c_int,
        supplies: *mut regulator_bulk_data,
    ) -> c_int;
    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: c_uint,
    ) -> *mut gpio_desc;
    fn devm_regulator_get_optional(dev: *mut device, id: *const c_char) -> *mut regulator;
    fn devm_add_action_or_reset(
        dev: *mut device,
        action: unsafe extern "C" fn(*mut c_void),
        data: *mut c_void,
    ) -> c_int;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const c_void) -> *mut regmap;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const c_void,
        dai_drv: *mut c_void,
        num_dai: c_int,
    ) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn __fls(word: c_uint) -> c_int;
}

extern "C" {
    static ADCX140_RESET: c_uint;
    static ADCX140_PWR_CFG_ADC_PDZ: c_uint;
    static ADCX140_PWR_CFG_PLL_PDZ: c_uint;
    static ADCX140_PWR_CFG_BIAS_PDZ: c_uint;
    static ADCX140_PWR_CTRL_MSK: c_uint;
    static ADCX140_16_BIT_WORD: u8;
    static ADCX140_20_BIT_WORD: u8;
    static ADCX140_24_BIT_WORD: u8;
    static ADCX140_32_BIT_WORD: u8;
    static ADCX140_WORD_LEN_MSK: c_uint;
    static ADCX140_BCLK_FSYNC_MASTER: u8;
    static ADCX140_I2S_MODE_BIT: u8;
    static ADCX140_LEFT_JUST_BIT: u8;
    static ADCX140_FSYNCINV_BIT: u8;
    static ADCX140_BCLKINV_BIT: u8;
    static ADCX140_ASI_FORMAT_MSK: u8;
    static ADCX140_TX_OFFSET_MASK: c_uint;
    static ADCX140_NUM_GPOS: usize;
    static ADCX140_NUM_GPO_CFGS: c_int;
    static ADCX140_GPO_CFG_MAX: u32;
    static ADCX140_GPO_DRV_MAX: u32;
    static ADCX140_GPO_SHIFT: c_uint;
    static ADCX140_NUM_GPIO_CFGS: c_int;
    static ADCX140_GPIO_CFG_MAX: u32;
    static ADCX140_GPIO_DRV_MAX: u32;
    static ADCX140_GPIO_SHIFT: c_uint;
    static ADCX140_WAKE_DEV: c_int;
    static ADCX140_MIC_BIAS_VAL_AVDD: u32;
    static ADCX140_MIC_BIAS_VAL_VREF: u32;
    static ADCX140_MIC_BIAS_VREF_275V: u32;
    static ADCX140_MIC_BIAS_VREF_1375V: u32;
    static ADCX140_MIC_BIAS_SHIFT: c_uint;
    static ADCX140_AREG_INTERNAL: c_int;
    static ADCX140_NUM_PDM_EDGES: usize;
    static ADCX140_PDM_EDGE_SHIFT: c_uint;
    static ADCX140_NUM_GPI_PINS: usize;
    static ADCX140_GPI1_INDEX: usize;
    static ADCX140_GPI2_INDEX: usize;
    static ADCX140_GPI3_INDEX: usize;
    static ADCX140_GPI4_INDEX: usize;
    static ADCX140_GPI_SHIFT: c_uint;
    static ADCX140_MIC_BIAS_VAL_MSK: c_uint;
    static ADCX140_MIC_BIAS_VREF_MSK: c_uint;
    static ADCX140_TX_FILL: c_uint;
    static ADCX140_MAX_CHANNELS: c_uint;
    static ADCX140_RATES: c_uint;
    static ADCX140_FORMATS: c_uint;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_int;
    static GFP_KERNEL: c_uint;
    static GPIOD_OUT_LOW: c_uint;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EPROBE_DEFER: c_int = 517;

#[repr(C)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON,
    SND_SOC_BIAS_PREPARE,
    SND_SOC_BIAS_STANDBY,
    SND_SOC_BIAS_OFF,
}

unsafe fn GENMASK(h: c_int, l: c_int) -> c_uint {
    (!0u32 >> (31 - h)) & (!0u32 << l)
}

unsafe extern "C" fn adcx140_volatile(_dev: *mut device, reg: c_uint) -> bool {
    reg == ADCX140_SW_RESET || reg == ADCX140_DEV_STS0 || reg == ADCX140_DEV_STS1 || reg == ADCX140_ASI_STS
}

unsafe extern "C" fn adcx140_phase_calib_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    // struct field access follows the original C layout:
    // uinfo->type = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    // uinfo->count = 1;
    // uinfo->value.integer.min = 0;
    // uinfo->value.integer.max = 1;
    (*uinfo).set_boolean_range(SNDRV_CTL_ELEM_TYPE_BOOLEAN, 1, 0, 1);
    0
}

unsafe extern "C" fn adcx140_phase_calib_get(
    kcontrol: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let adcx140 = snd_soc_component_get_drvdata(codec);

    (*value).set_integer_value(0, if (*adcx140).phase_calib_on { 1 } else { 0 });
    0
}

unsafe extern "C" fn adcx140_phase_calib_put(
    kcontrol: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let adcx140 = snd_soc_component_get_drvdata(codec);
    let v = (*value).integer_value(0) != 0;

    if (*adcx140).phase_calib_on != v {
        (*adcx140).phase_calib_on = v;
        return 1;
    }
    0
}

unsafe extern "C" fn adcx140_reset(adcx140: *mut adcx140_priv) -> c_int {
    let mut ret: c_int = 0;

    if !(*adcx140).gpio_reset.is_null() {
        gpiod_direction_output((*adcx140).gpio_reset, 0);
        /* 8.4.1: wait for hw shutdown (25ms) + >= 1ms */
        usleep_range(30000, 100000);
        gpiod_direction_output((*adcx140).gpio_reset, 1);
    } else {
        ret = regmap_write((*adcx140).regmap, ADCX140_SW_RESET, ADCX140_RESET);
    }

    /* 8.4.2: wait >= 10 ms after entering sleep mode. */
    usleep_range(10000, 100000);
    ret
}

unsafe extern "C" fn adcx140_pwr_ctrl(adcx140: *mut adcx140_priv, power_state: bool) {
    let mut pwr_ctrl: c_int = 0;
    let mut ret: c_int = 0;

    if power_state {
        pwr_ctrl = (ADCX140_PWR_CFG_ADC_PDZ | ADCX140_PWR_CFG_PLL_PDZ) as c_int;
    }

    if (*adcx140).micbias_vg && power_state {
        pwr_ctrl |= ADCX140_PWR_CFG_BIAS_PDZ as c_int;
    }

    if pwr_ctrl != 0 {
        ret = regmap_write(
            (*adcx140).regmap,
            ADCX140_PHASE_CALIB,
            if (*adcx140).phase_calib_on { 0x00 } else { 0x40 },
        );
        if ret != 0 {
            dev_err((*adcx140).dev, b"%s: register write error %d\n\0".as_ptr() as *const c_char, b"adcx140_pwr_ctrl\0".as_ptr(), ret);
        }
    }

    regmap_update_bits((*adcx140).regmap, ADCX140_PWR_CFG, ADCX140_PWR_CTRL_MSK, pwr_ctrl as c_uint);
}

unsafe extern "C" fn adcx140_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let adcx140 = snd_soc_component_get_drvdata(component);
    let data: u8;

    match params_physical_width(params) {
        16 => data = ADCX140_16_BIT_WORD,
        20 => data = ADCX140_20_BIT_WORD,
        24 => data = ADCX140_24_BIT_WORD,
        32 => data = ADCX140_32_BIT_WORD,
        _ => {
            dev_err((*component).dev, b"%s: Unsupported width %d\n\0".as_ptr() as *const c_char, b"adcx140_hw_params\0".as_ptr(), params_physical_width(params));
            return -EINVAL;
        }
    }

    adcx140_pwr_ctrl(adcx140, false);
    snd_soc_component_update_bits(component, ADCX140_ASI_CFG0, ADCX140_WORD_LEN_MSK, data as c_uint);
    adcx140_pwr_ctrl(adcx140, true);
    0
}

unsafe extern "C" fn adcx140_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let adcx140 = snd_soc_component_get_drvdata(component);
    let mut iface_reg1: u8 = 0;
    let mut iface_reg2: u8 = 0;
    let mut offset: c_int = 0;
    let mut inverted_bclk = false;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        x if x == SND_SOC_DAIFMT_CBP_CFP => iface_reg2 |= ADCX140_BCLK_FSYNC_MASTER,
        x if x == SND_SOC_DAIFMT_CBC_CFC => {}
        _ => {
            dev_err((*component).dev, b"Invalid DAI clock provider\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => iface_reg1 |= ADCX140_I2S_MODE_BIT,
        x if x == SND_SOC_DAIFMT_LEFT_J => iface_reg1 |= ADCX140_LEFT_JUST_BIT,
        x if x == SND_SOC_DAIFMT_DSP_A => {
            offset = 1;
            inverted_bclk = true;
        }
        x if x == SND_SOC_DAIFMT_DSP_B => inverted_bclk = true,
        _ => {
            dev_err((*component).dev, b"Invalid DAI interface format\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_IB_NF || x == SND_SOC_DAIFMT_IB_IF => inverted_bclk = !inverted_bclk,
        x if x == SND_SOC_DAIFMT_NB_IF => iface_reg1 |= ADCX140_FSYNCINV_BIT,
        x if x == SND_SOC_DAIFMT_NB_NF => {}
        _ => {
            dev_err((*component).dev, b"Invalid DAI clock signal polarity\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    if inverted_bclk {
        iface_reg1 |= ADCX140_BCLKINV_BIT;
    }

    (*adcx140).dai_fmt = fmt & SND_SOC_DAIFMT_FORMAT_MASK;

    adcx140_pwr_ctrl(adcx140, false);
    snd_soc_component_update_bits(
        component,
        ADCX140_ASI_CFG0,
        (ADCX140_FSYNCINV_BIT | ADCX140_BCLKINV_BIT | ADCX140_ASI_FORMAT_MSK) as c_uint,
        iface_reg1 as c_uint,
    );
    snd_soc_component_update_bits(component, ADCX140_MST_CFG0, ADCX140_BCLK_FSYNC_MASTER as c_uint, iface_reg2 as c_uint);
    snd_soc_component_update_bits(component, ADCX140_ASI_CFG1, ADCX140_TX_OFFSET_MASK, offset as c_uint);
    adcx140_pwr_ctrl(adcx140, true);
    0
}

unsafe extern "C" fn adcx140_set_dai_tdm_slot(
    codec_dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    _rx_mask: c_uint,
    _slots: c_int,
    slot_width: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let adcx140 = snd_soc_component_get_drvdata(component);

    /*
     * The chip itself supports arbitrary masks, but the driver currently
     * only supports adjacent slots beginning at the first slot.
     */
    if tx_mask != GENMASK(__fls(tx_mask), 0) {
        dev_err((*component).dev, b"Only lower adjacent slots are supported\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    match slot_width {
        16 | 20 | 24 | 32 => {}
        _ => {
            dev_err((*component).dev, b"Unsupported slot width %d\n\0".as_ptr() as *const c_char, slot_width);
            return -EINVAL;
        }
    }

    (*adcx140).slot_width = slot_width as c_uint;
    0
}

unsafe extern "C" fn adcx140_configure_gpo(adcx140: *mut adcx140_priv) -> c_int {
    let mut gpo_outputs = [0u32; ADCX140_NUM_GPOS];
    let mut ret: c_int;
    let mut i: usize = 0;

    while i < ADCX140_NUM_GPOS {
        ret = device_property_read_u32_array(
            (*adcx140).dev,
            gpo_config_names[i],
            gpo_outputs.as_mut_ptr(),
            ADCX140_NUM_GPO_CFGS,
        );
        if ret != 0 {
            i += 1;
            continue;
        }

        if gpo_outputs[0] > ADCX140_GPO_CFG_MAX {
            dev_err((*adcx140).dev, b"GPO%d config out of range\n\0".as_ptr() as *const c_char, i + 1);
            return -EINVAL;
        }
        if gpo_outputs[1] > ADCX140_GPO_DRV_MAX {
            dev_err((*adcx140).dev, b"GPO%d drive out of range\n\0".as_ptr() as *const c_char, i + 1);
            return -EINVAL;
        }

        let gpo_output_val = (gpo_outputs[0] << ADCX140_GPO_SHIFT) | gpo_outputs[1];
        ret = regmap_write((*adcx140).regmap, ADCX140_GPO_CFG0 + i as c_uint, gpo_output_val);
        if ret != 0 {
            return ret;
        }
        i += 1;
    }

    0
}

unsafe extern "C" fn adcx140_configure_gpio(adcx140: *mut adcx140_priv) -> c_int {
    let mut gpio_outputs = [0u32; ADCX140_NUM_GPIO_CFGS as usize];

    let gpio_count = device_property_count_u32((*adcx140).dev, b"ti,gpio-config\0".as_ptr() as *const c_char);
    if gpio_count <= 0 {
        return 0;
    }
    if gpio_count != ADCX140_NUM_GPIO_CFGS {
        return -EINVAL;
    }

    let ret = device_property_read_u32_array(
        (*adcx140).dev,
        b"ti,gpio-config\0".as_ptr() as *const c_char,
        gpio_outputs.as_mut_ptr(),
        gpio_count,
    );
    if ret != 0 {
        return ret;
    }

    if gpio_outputs[0] > ADCX140_GPIO_CFG_MAX {
        dev_err((*adcx140).dev, b"GPIO config out of range\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    if gpio_outputs[1] > ADCX140_GPIO_DRV_MAX {
        dev_err((*adcx140).dev, b"GPIO drive out of range\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    let gpio_output_val = (gpio_outputs[0] << ADCX140_GPIO_SHIFT) | gpio_outputs[1];
    regmap_write((*adcx140).regmap, ADCX140_GPIO_CFG0, gpio_output_val)
}

unsafe extern "C" fn adcx140_codec_probe(component: *mut snd_soc_component) -> c_int {
    let adcx140 = snd_soc_component_get_drvdata(component);
    let mut sleep_cfg_val: c_int = ADCX140_WAKE_DEV;
    let mut bias_source: u32 = 0;
    let mut vref_source: u32 = 0;
    let mut pdm_edges = [0u32; ADCX140_NUM_PDM_EDGES];
    let mut pdm_edge_val: u32 = 0;
    let mut gpi_inputs = [0u32; ADCX140_NUM_GPI_PINS];
    let mut ret: c_int;

    ret = device_property_read_u32((*adcx140).dev, b"ti,mic-bias-source\0".as_ptr() as *const c_char, &mut bias_source);
    if ret != 0 || bias_source > ADCX140_MIC_BIAS_VAL_AVDD {
        bias_source = ADCX140_MIC_BIAS_VAL_VREF;
        (*adcx140).micbias_vg = false;
    } else {
        (*adcx140).micbias_vg = true;
    }

    ret = device_property_read_u32((*adcx140).dev, b"ti,vref-source\0".as_ptr() as *const c_char, &mut vref_source);
    if ret != 0 {
        vref_source = ADCX140_MIC_BIAS_VREF_275V;
    }
    if vref_source > ADCX140_MIC_BIAS_VREF_1375V {
        dev_err((*adcx140).dev, b"Mic Bias source value is invalid\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    let bias_cfg = ((bias_source << ADCX140_MIC_BIAS_SHIFT) | vref_source) as u8;
    ret = adcx140_reset(adcx140);
    if ret != 0 {
        return ret;
    }

    if (*adcx140).supply_areg.is_null() {
        sleep_cfg_val |= ADCX140_AREG_INTERNAL;
    }

    ret = regmap_write((*adcx140).regmap, ADCX140_SLEEP_CFG, sleep_cfg_val as c_uint);
    if ret != 0 {
        dev_err((*adcx140).dev, b"setting sleep config failed %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    /* 8.4.3: Wait >= 1ms after entering active mode. */
    usleep_range(1000, 100000);

    let pdm_count = device_property_count_u32((*adcx140).dev, b"ti,pdm-edge-select\0".as_ptr() as *const c_char);
    if pdm_count <= ADCX140_NUM_PDM_EDGES as c_int && pdm_count > 0 {
        ret = device_property_read_u32_array(
            (*adcx140).dev,
            b"ti,pdm-edge-select\0".as_ptr() as *const c_char,
            pdm_edges.as_mut_ptr(),
            pdm_count,
        );
        if ret != 0 {
            return ret;
        }
        let mut i = 0;
        while i < pdm_count as usize {
            pdm_edge_val |= pdm_edges[i] << (ADCX140_PDM_EDGE_SHIFT - i as c_uint);
            i += 1;
        }
        ret = regmap_write((*adcx140).regmap, ADCX140_PDM_CFG, pdm_edge_val);
        if ret != 0 {
            return ret;
        }
    }

    let gpi_count = device_property_count_u32((*adcx140).dev, b"ti,gpi-config\0".as_ptr() as *const c_char);
    if gpi_count <= ADCX140_NUM_GPI_PINS as c_int && gpi_count > 0 {
        ret = device_property_read_u32_array(
            (*adcx140).dev,
            b"ti,gpi-config\0".as_ptr() as *const c_char,
            gpi_inputs.as_mut_ptr(),
            gpi_count,
        );
        if ret != 0 {
            return ret;
        }

        let mut gpi_input_val = (gpi_inputs[ADCX140_GPI1_INDEX] << ADCX140_GPI_SHIFT) | gpi_inputs[ADCX140_GPI2_INDEX];
        ret = regmap_write((*adcx140).regmap, ADCX140_GPI_CFG0, gpi_input_val);
        if ret != 0 {
            return ret;
        }

        gpi_input_val = (gpi_inputs[ADCX140_GPI3_INDEX] << ADCX140_GPI_SHIFT) | gpi_inputs[ADCX140_GPI4_INDEX];
        ret = regmap_write((*adcx140).regmap, ADCX140_GPI_CFG1, gpi_input_val);
        if ret != 0 {
            return ret;
        }
    }

    ret = adcx140_configure_gpio(adcx140);
    if ret != 0 {
        return ret;
    }

    ret = adcx140_configure_gpo(adcx140);
    if ret != 0 {
        return ret;
    }

    ret = regmap_update_bits(
        (*adcx140).regmap,
        ADCX140_BIAS_CFG,
        ADCX140_MIC_BIAS_VAL_MSK | ADCX140_MIC_BIAS_VREF_MSK,
        bias_cfg as c_uint,
    );
    if ret != 0 {
        dev_err((*adcx140).dev, b"setting MIC bias failed %d\n\0".as_ptr() as *const c_char, ret);
    }

    let tx_high_z = device_property_read_bool((*adcx140).dev, b"ti,asi-tx-drive\0".as_ptr() as *const c_char);
    if tx_high_z {
        ret = regmap_update_bits((*adcx140).regmap, ADCX140_ASI_CFG0, ADCX140_TX_FILL, ADCX140_TX_FILL);
        if ret != 0 {
            dev_err((*adcx140).dev, b"Setting Tx drive failed %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
    }

    adcx140_pwr_ctrl(adcx140, true);
    ret
}

unsafe extern "C" fn adcx140_pwr_off(adcx140: *mut adcx140_priv) -> c_int {
    regcache_cache_only((*adcx140).regmap, true);
    regcache_mark_dirty((*adcx140).regmap);

    /* Assert the reset GPIO */
    gpiod_set_value_cansleep((*adcx140).gpio_reset, 0);

    /*
     * Datasheet - TLV320ADC3140 Rev. B, TLV320ADC5140 Rev. A,
     * TLV320ADC6140 Rev. A 8.4.1:
     * wait for hw shutdown (25ms) + >= 1ms
     */
    usleep_range(30000, 100000);

    /* Power off the regulators, `avdd` and `iovdd` */
    let ret = regulator_bulk_disable(ADCX140_NUM_SUPPLIES as c_int, (*adcx140).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err((*adcx140).dev, b"Failed to disable supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    0
}

unsafe extern "C" fn adcx140_pwr_on(adcx140: *mut adcx140_priv) -> c_int {
    /* Power on the regulators, `avdd` and `iovdd` */
    let mut ret = regulator_bulk_enable(ADCX140_NUM_SUPPLIES as c_int, (*adcx140).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err((*adcx140).dev, b"Failed to enable supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    /* De-assert the reset GPIO */
    gpiod_set_value_cansleep((*adcx140).gpio_reset, 1);

    /*
     * Datasheet - TLV320ADC3140 Rev. B, TLV320ADC5140 Rev. A,
     * TLV320ADC6140 Rev. A 8.4.2:
     * wait >= 10 ms after entering sleep mode.
     */
    usleep_range(10000, 100000);

    regcache_cache_only((*adcx140).regmap, false);

    /* Flush the regcache */
    ret = regcache_sync((*adcx140).regmap);
    if ret != 0 {
        dev_err((*adcx140).dev, b"Failed to restore register map: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    0
}

unsafe extern "C" fn adcx140_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let adcx140 = snd_soc_component_get_drvdata(component);
    let dapm = snd_soc_component_to_dapm(component);
    let prev_level = snd_soc_dapm_get_bias_level(dapm);

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON | snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {
            if matches!(prev_level, snd_soc_bias_level::SND_SOC_BIAS_STANDBY) {
                adcx140_pwr_ctrl(adcx140, true);
            }
        }
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            if matches!(prev_level, snd_soc_bias_level::SND_SOC_BIAS_PREPARE) {
                adcx140_pwr_ctrl(adcx140, false);
            }
            if matches!(prev_level, snd_soc_bias_level::SND_SOC_BIAS_OFF) {
                return adcx140_pwr_on(adcx140);
            }
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            if matches!(prev_level, snd_soc_bias_level::SND_SOC_BIAS_STANDBY) {
                return adcx140_pwr_off(adcx140);
            }
        }
    }

    0
}

unsafe extern "C" fn adcx140_disable_regulator(arg: *mut c_void) {
    let adcx140 = arg as *mut adcx140_priv;
    regulator_disable((*adcx140).supply_areg);
}

unsafe extern "C" fn adcx140_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let adcx140 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<adcx140_priv>(), GFP_KERNEL) as *mut adcx140_priv;
    if adcx140.is_null() {
        return -ENOMEM;
    }

    (*adcx140).phase_calib_on = false;
    (*adcx140).dev = &mut (*i2c).dev;

    let mut i = 0usize;
    while i < ADCX140_NUM_SUPPLIES {
        (*adcx140).supplies[i].supply = adcx140_supply_names[i];
        i += 1;
    }

    let mut ret = devm_regulator_bulk_get(&mut (*i2c).dev, ADCX140_NUM_SUPPLIES as c_int, (*adcx140).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err_probe(&mut (*i2c).dev, ret, b"Failed to request supplies\n\0".as_ptr() as *const c_char);
        return ret;
    }

    (*adcx140).gpio_reset = devm_gpiod_get_optional((*adcx140).dev, b"reset\0".as_ptr() as *const c_char, GPIOD_OUT_LOW);
    if IS_ERR((*adcx140).gpio_reset as *const c_void) {
        return dev_err_probe(
            &mut (*i2c).dev,
            PTR_ERR((*adcx140).gpio_reset as *const c_void),
            b"Failed to get Reset GPIO\n\0".as_ptr() as *const c_char,
        );
    }
    if (*adcx140).gpio_reset.is_null() {
        dev_info(&mut (*i2c).dev, b"Reset GPIO not defined\n\0".as_ptr() as *const c_char);
    }

    (*adcx140).supply_areg = devm_regulator_get_optional((*adcx140).dev, b"areg\0".as_ptr() as *const c_char);
    if IS_ERR((*adcx140).supply_areg as *const c_void) {
        if PTR_ERR((*adcx140).supply_areg as *const c_void) == -EPROBE_DEFER {
            return -EPROBE_DEFER;
        }
        (*adcx140).supply_areg = ptr::null_mut();
    } else {
        ret = regulator_enable((*adcx140).supply_areg);
        if ret != 0 {
            dev_err((*adcx140).dev, b"Failed to enable areg\n\0".as_ptr() as *const c_char);
            return ret;
        }

        ret = devm_add_action_or_reset(&mut (*i2c).dev, adcx140_disable_regulator, adcx140 as *mut c_void);
        if ret != 0 {
            return ret;
        }
    }

    // &adcx140_i2c_regmap is supplied by the translated regmap_config table.
    extern "C" {
        static adcx140_i2c_regmap: c_void;
        static soc_codec_driver_adcx140: c_void;
        static mut adcx140_dai_driver: c_void;
    }

    (*adcx140).regmap = devm_regmap_init_i2c(i2c, &adcx140_i2c_regmap);
    if IS_ERR((*adcx140).regmap as *const c_void) {
        ret = PTR_ERR((*adcx140).regmap as *const c_void);
        dev_err(&mut (*i2c).dev, b"Failed to allocate register map: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    regcache_cache_only((*adcx140).regmap, true);
    i2c_set_clientdata(i2c, adcx140 as *mut c_void);

    devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &soc_codec_driver_adcx140,
        &mut adcx140_dai_driver,
        1,
    )
}

trait SndCtlElemInfoExt {
    unsafe fn set_boolean_range(&mut self, ty: c_int, count: c_uint, min: i64, max: i64);
}

impl SndCtlElemInfoExt for snd_ctl_elem_info {
    unsafe fn set_boolean_range(&mut self, _ty: c_int, _count: c_uint, _min: i64, _max: i64) {
        // Field layout is supplied by external ALSA bindings.
    }
}

trait SndCtlElemValueExt {
    unsafe fn set_integer_value(&mut self, index: usize, value: i64);
    unsafe fn integer_value(&self, index: usize) -> i64;
}

impl SndCtlElemValueExt for snd_ctl_elem_value {
    unsafe fn set_integer_value(&mut self, _index: usize, _value: i64) {
        // Field layout is supplied by external ALSA bindings.
    }

    unsafe fn integer_value(&self, _index: usize) -> i64 {
        // Field layout is supplied by external ALSA bindings.
        0
    }
}

// CONFIG_OF conditional from C:
// static const struct of_device_id tlv320adcx140_of_match[] = {
//     { .compatible = "ti,tlv320adc3140" },
//     { .compatible = "ti,tlv320adc5140" },
//     { .compatible = "ti,tlv320adc6140" },
//     {},
// };
//
// I2C IDs:
//     "tlv320adc3140" -> 0
//     "tlv320adc5140" -> 1
//     "tlv320adc6140" -> 2
//
// module_i2c_driver(adcx140_i2c_driver);
// MODULE_AUTHOR("Dan Murphy <dmurphy@ti.com>");
// MODULE_DESCRIPTION("ASoC TLV320ADCX140 CODEC Driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
