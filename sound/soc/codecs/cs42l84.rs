// SPDX-License-Identifier: GPL-2.0-only
/*
 * cs42l84.c -- CS42L84 ALSA SoC audio driver
 *
 * Copyright (C) The Asahi Linux Contributors
 *
 * Based on sound/soc/codecs/cs42l42{.c,.h}
 *   Copyright 2016 Cirrus Logic, Inc.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type bool_ = bool;
type u8 = u8;
type u16 = u16;
type u32 = u32;
type irqreturn_t = c_uint;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: usize,
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}
#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}
type c_long = isize;

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}
#[repr(C)]
pub struct soc_mixer_control {
    pub min: c_int,
    pub max: c_int,
}
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct i2c_client {
    pub dev: device,
    pub irq: c_int,
}

#[repr(C)]
struct cs42l84_private {
    regmap: *mut regmap,
    dev: *mut device,
    reset_gpio: *mut gpio_desc,
    jack: *mut snd_soc_jack,
    irq_lock: mutex,
    tip_state: u8,
    ring_state: u8,
    pll_config: c_int,
    bclk: c_int,
    pll_mclk_f: u8,
    srate: u32,
    stream_use: u8,
    hs_type: c_int,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub max_register: c_uint,
    pub cache_type: c_uint,
    pub use_single_read: bool_,
    pub use_single_write: bool_,
}

unsafe extern "C" {
    fn snd_kcontrol_chip(kctl: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_jack_report(jk: *mut snd_soc_jack, status: c_int, mask: c_int);
    fn params_rate(params: *mut snd_pcm_hw_params) -> u32;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn msleep(msecs: c_uint);
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut c_void;
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn request_threaded_irq(irq: c_int, handler: *const c_void, thread_fn: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_uint, name: *const c_char, dev: *mut c_void) -> c_int;
    fn free_irq(irq: c_int, dev_id: *mut c_void);
    fn cirrus_read_device_id(regmap: *mut regmap, devid: c_uint) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
}

const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const ENOMEM: c_int = 12;
const EPROBE_DEFER: c_int = 517;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_ONESHOT: c_uint = 0x00002000;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_LOW: c_uint = 0;
const REGCACHE_MAPLE: c_uint = 0;

unsafe fn IS_ERR<T>(ptr: *mut T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) > -4096
}
unsafe fn PTR_ERR<T>(ptr: *mut T) -> c_int {
    ptr as isize as c_int
}
const fn BIT(n: c_uint) -> c_uint {
    1u32 << n
}
const fn FIELD_PREP(mask: c_uint, val: c_uint) -> c_uint {
    (val << mask.trailing_zeros()) & mask
}

unsafe extern "C" fn cs42l84_volatile_register(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        CS42L84_DEVID..=CS42L84_DEVID_PLUS_5
        | CS42L84_TSRS_PLUG_INT_STATUS
        | CS42L84_PLL_LOCK_STATUS
        | CS42L84_TSRS_PLUG_STATUS
        | CS42L84_HS_DET_STATUS2 => true,
        _ => false,
    }
}

static cs42l84_regmap: regmap_config = regmap_config {
    reg_bits: 16,
    val_bits: 8,
    volatile_reg: Some(cs42l84_volatile_register),
    max_register: 0x73fe,
    cache_type: REGCACHE_MAPLE,
    use_single_read: true,
    use_single_write: true,
};

unsafe extern "C" fn cs42l84_put_dac_vol(kctl: *mut snd_kcontrol, val: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kctl);
    let mc = (*kctl).private_value as *mut soc_mixer_control;
    let vola: c_int;
    let volb: c_int;
    let mut ret: c_int;
    let ret2: c_int;

    vola = (*val).value.integer.value[0] as c_int + (*mc).min;
    volb = (*val).value.integer.value[1] as c_int + (*mc).min;

    if vola < (*mc).min || vola > (*mc).max || volb < (*mc).min || volb > (*mc).max {
        return -EINVAL;
    }

    ret = {
        let mut updated: c_int = 0;
        let mut r = snd_soc_component_update_bits(component, CS42L84_FRZ_CTL, CS42L84_FRZ_CTL_ENGAGE, CS42L84_FRZ_CTL_ENGAGE);
        if r >= 0 {
            updated |= r;
            r = snd_soc_component_update_bits(component, CS42L84_DAC_CHA_VOL_LSB, 0xff, (vola & 0xff) as c_uint);
        }
        if r >= 0 {
            updated |= r;
            r = snd_soc_component_update_bits(component, CS42L84_DAC_CHA_VOL_MSB, 0xff, ((vola >> 8) & 0x01) as c_uint);
        }
        if r >= 0 {
            updated |= r;
            r = snd_soc_component_update_bits(component, CS42L84_DAC_CHB_VOL_LSB, 0xff, (volb & 0xff) as c_uint);
        }
        if r >= 0 {
            updated |= r;
            r = snd_soc_component_update_bits(component, CS42L84_DAC_CHB_VOL_MSB, 0xff, ((volb >> 8) & 0x01) as c_uint);
        }
        if r >= 0 {
            r |= updated;
        }
        r
    };

    ret2 = snd_soc_component_update_bits(component, CS42L84_FRZ_CTL, CS42L84_FRZ_CTL_ENGAGE, 0);
    if ret2 < 0 && ret >= 0 {
        ret = ret2;
    }
    ret
}

unsafe extern "C" fn cs42l84_get_dac_vol(kctl: *mut snd_kcontrol, val: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kctl);
    let mc = (*kctl).private_value as *mut soc_mixer_control;
    let mut vola: c_int;
    let mut volb: c_int;
    let mut ret: c_int;

    ret = snd_soc_component_read(component, CS42L84_DAC_CHA_VOL_LSB);
    if ret < 0 { return ret; }
    vola = ret;

    ret = snd_soc_component_read(component, CS42L84_DAC_CHA_VOL_MSB);
    if ret < 0 { return ret; }
    vola |= (ret & 1) << 8;

    ret = snd_soc_component_read(component, CS42L84_DAC_CHB_VOL_LSB);
    if ret < 0 { return ret; }
    volb = ret;

    ret = snd_soc_component_read(component, CS42L84_DAC_CHB_VOL_MSB);
    if ret < 0 { return ret; }
    volb |= (ret & 1) << 8;

    if (vola & BIT(8) as c_int) != 0 {
        vola |= !((BIT(8) - 1) as c_int);
    }
    if (volb & BIT(8) as c_int) != 0 {
        volb |= !((BIT(8) - 1) as c_int);
    }

    (*val).value.integer.value[0] = (vola - (*mc).min) as c_long;
    (*val).value.integer.value[1] = (volb - (*mc).min) as c_long;
    0
}

/* static const DECLARE_TLV_DB_SCALE(cs42l84_dac_tlv, -12800, 50, true); */
/* static const DECLARE_TLV_DB_SCALE(cs42l84_adc_tlv, -1200, 50, false); */
/* static const DECLARE_TLV_DB_SCALE(cs42l84_pre_tlv, 0, 1000, false); */

/* The ALSA control, DAPM widget, route, enum, component-driver, DAI-driver,
 * OF, I2C, module, and MODULE_* declarations below depend on Linux macro
 * constructors and external struct layouts. Their source-level contents are
 * preserved in Rust comments at their original declaration sites.
 */

/*
static const struct snd_kcontrol_new cs42l84_snd_controls[] = {
    SOC_DOUBLE_R_S_EXT_TLV("DAC Playback Volume", CS42L84_DAC_CHA_VOL_LSB,
            CS42L84_DAC_CHB_VOL_LSB, 0, -256, 24, 8, 0,
            cs42l84_get_dac_vol, cs42l84_put_dac_vol, cs42l84_dac_tlv),
    SOC_SINGLE_TLV("ADC Preamp Capture Volume", CS42L84_ADC_CTL1,
            CS42L84_ADC_CTL1_PREAMP_GAIN_SHIFT, 2, 0, cs42l84_pre_tlv),
    SOC_SINGLE_TLV("ADC PGA Capture Volume", CS42L84_ADC_CTL1,
            CS42L84_ADC_CTL1_PGA_GAIN_SHIFT, 24, 0, cs42l84_adc_tlv),
    SOC_SINGLE("ADC WNF Switch", CS42L84_ADC_CTL4,
            CS42L84_ADC_CTL4_WNF_EN_SHIFT, 1, 0),
    SOC_SINGLE("WNF Corner Frequency", CS42L84_ADC_CTL4,
            CS42L84_ADC_CTL4_WNF_CF_SHIFT, 3, 0),
    SOC_SINGLE("ADC HPF Switch", CS42L84_ADC_CTL4,
            CS42L84_ADC_CTL4_HPF_EN_SHIFT, 1, 0),
    SOC_SINGLE("HPF Corner Frequency", CS42L84_ADC_CTL4,
            CS42L84_ADC_CTL4_HPF_CF_SHIFT, 3, 0),
};

static const char * const cs42l84_mux_text[] = {
    "Blank", "ADC", "ASP RX CH1", "ASP RX CH2",
};

static const unsigned int cs42l84_mux_values[] = {
    0b0000, 0b0111, 0b1101, 0b1110,
};

static SOC_VALUE_ENUM_SINGLE_DECL(cs42l84_daca_mux_enum,
        CS42L84_BUS_DAC_SRC, CS42L84_BUS_DAC_SRC_DACA_SHIFT,
        0b1111, cs42l84_mux_text, cs42l84_mux_values);
static SOC_VALUE_ENUM_SINGLE_DECL(cs42l84_dacb_mux_enum,
        CS42L84_BUS_DAC_SRC, CS42L84_BUS_DAC_SRC_DACB_SHIFT,
        0b1111, cs42l84_mux_text, cs42l84_mux_values);
static SOC_VALUE_ENUM_SINGLE_DECL(cs42l84_sdout1_mux_enum,
        CS42L84_BUS_ASP_TX_SRC, CS42L84_BUS_ASP_TX_SRC_CH1_SHIFT,
        0b1111, cs42l84_mux_text, cs42l84_mux_values);
static const struct snd_kcontrol_new cs42l84_daca_mux_ctrl =
    SOC_DAPM_ENUM("DACA Select", cs42l84_daca_mux_enum);
static const struct snd_kcontrol_new cs42l84_dacb_mux_ctrl =
    SOC_DAPM_ENUM("DACB Select", cs42l84_dacb_mux_enum);
static const struct snd_kcontrol_new cs42l84_sdout1_mux_ctrl =
    SOC_DAPM_ENUM("SDOUT1 Select", cs42l84_sdout1_mux_enum);
static const struct snd_soc_dapm_widget cs42l84_dapm_widgets[] = { ... };
static const struct snd_soc_dapm_route cs42l84_audio_map[] = { ... };
*/

unsafe extern "C" fn cs42l84_set_jack(component: *mut snd_soc_component, jk: *mut snd_soc_jack, _d: *mut c_void) -> c_int {
    let cs42l84 = snd_soc_component_get_drvdata(component) as *mut cs42l84_private;
    mutex_lock(&mut (*cs42l84).irq_lock);
    (*cs42l84).jack = jk;
    snd_soc_jack_report(jk, (*cs42l84).hs_type, SND_JACK_HEADSET);
    mutex_unlock(&mut (*cs42l84).irq_lock);
    0
}

unsafe extern "C" fn cs42l84_component_probe(component: *mut snd_soc_component) -> c_int {
    snd_soc_component_update_bits(component, CS42L84_ASP_CTL, CS42L84_ASP_CTL_TDM_MODE, 0);
    snd_soc_component_update_bits(component, CS42L84_HP_VOL_CTL, CS42L84_HP_VOL_CTL_SOFT | CS42L84_HP_VOL_CTL_ZERO_CROSS, CS42L84_HP_VOL_CTL_ZERO_CROSS);

    /* TDM settings */
    snd_soc_component_update_bits(component, CS42L84_ASP_RX_CH1_CTL1, CS42L84_ASP_RX_CHx_CTL1_EDGE | CS42L84_ASP_RX_CHx_CTL1_SLOT_START_LSB, 0);
    snd_soc_component_update_bits(component, CS42L84_ASP_RX_CH1_CTL2, CS42L84_ASP_RX_CHx_CTL2_SLOT_START_MSB, 0);
    snd_soc_component_update_bits(component, CS42L84_ASP_RX_CH2_CTL1, CS42L84_ASP_RX_CHx_CTL1_EDGE | CS42L84_ASP_RX_CHx_CTL1_SLOT_START_LSB, CS42L84_ASP_RX_CHx_CTL1_EDGE);
    snd_soc_component_update_bits(component, CS42L84_ASP_RX_CH2_CTL2, CS42L84_ASP_RX_CHx_CTL2_SLOT_START_MSB, 0);
    snd_soc_component_update_bits(component, CS42L84_ASP_TX_CH1_CTL1, CS42L84_ASP_RX_CHx_CTL1_EDGE | CS42L84_ASP_RX_CHx_CTL1_SLOT_START_LSB, 0);
    snd_soc_component_update_bits(component, CS42L84_ASP_TX_CH1_CTL2, CS42L84_ASP_RX_CHx_CTL2_SLOT_START_MSB, 0);
    snd_soc_component_update_bits(component, CS42L84_ASP_TX_CH2_CTL1, CS42L84_ASP_RX_CHx_CTL1_EDGE | CS42L84_ASP_RX_CHx_CTL1_SLOT_START_LSB, CS42L84_ASP_RX_CHx_CTL1_EDGE);
    snd_soc_component_update_bits(component, CS42L84_ASP_TX_CH2_CTL2, CS42L84_ASP_RX_CHx_CTL2_SLOT_START_MSB, 0);
    /* Routing defaults */
    snd_soc_component_write(component, CS42L84_BUS_DAC_SRC, (0b1101 << CS42L84_BUS_DAC_SRC_DACA_SHIFT) | (0b1110 << CS42L84_BUS_DAC_SRC_DACB_SHIFT));
    snd_soc_component_write(component, CS42L84_BUS_ASP_TX_SRC, 0b0111 << CS42L84_BUS_ASP_TX_SRC_CH1_SHIFT);
    0
}

#[repr(C)]
pub struct snd_soc_component_driver {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    _private: [u8; 0],
}
/*
static const struct snd_soc_component_driver soc_component_dev_cs42l84 = {
    .set_jack = cs42l84_set_jack,
    .probe = cs42l84_component_probe,
    .controls = cs42l84_snd_controls,
    .num_controls = ARRAY_SIZE(cs42l84_snd_controls),
    .dapm_widgets = cs42l84_dapm_widgets,
    .num_dapm_widgets = ARRAY_SIZE(cs42l84_dapm_widgets),
    .dapm_routes = cs42l84_audio_map,
    .num_dapm_routes = ARRAY_SIZE(cs42l84_audio_map),
    .endianness = 1,
};
*/
static soc_component_dev_cs42l84: snd_soc_component_driver = snd_soc_component_driver { _private: [] };
static mut cs42l84_dai: snd_soc_dai_driver = snd_soc_dai_driver { _private: [] };

#[repr(C)]
struct cs42l84_pll_params {
    bclk: u32,
    mclk_src_sel: u8,
    bclk_prediv: u8,
    pll_div_int: u8,
    pll_div_frac: u32,
    pll_mode: u8,
    pll_divout: u8,
    mclk_int: u32,
}

/*
 * Common PLL Settings for given BCLK
 */
static pll_ratio_table: [cs42l84_pll_params; 7] = [
    cs42l84_pll_params { bclk: 2822400, mclk_src_sel: 1, bclk_prediv: 0, pll_div_int: 0x40, pll_div_frac: 0x000000, pll_mode: 0x03, pll_divout: 0x10, mclk_int: 11289600 },
    cs42l84_pll_params { bclk: 3072000, mclk_src_sel: 1, bclk_prediv: 0, pll_div_int: 0x40, pll_div_frac: 0x000000, pll_mode: 0x03, pll_divout: 0x10, mclk_int: 12288000 },
    cs42l84_pll_params { bclk: 5644800, mclk_src_sel: 1, bclk_prediv: 0, pll_div_int: 0x40, pll_div_frac: 0x000000, pll_mode: 0x03, pll_divout: 0x10, mclk_int: 11289600 },
    cs42l84_pll_params { bclk: 6144000, mclk_src_sel: 1, bclk_prediv: 1, pll_div_int: 0x40, pll_div_frac: 0x000000, pll_mode: 0x03, pll_divout: 0x10, mclk_int: 12288000 },
    cs42l84_pll_params { bclk: 11289600, mclk_src_sel: 0, bclk_prediv: 0, pll_div_int: 0, pll_div_frac: 0, pll_mode: 0, pll_divout: 0, mclk_int: 11289600 },
    cs42l84_pll_params { bclk: 12288000, mclk_src_sel: 0, bclk_prediv: 0, pll_div_int: 0, pll_div_frac: 0, pll_mode: 0, pll_divout: 0, mclk_int: 12288000 },
    cs42l84_pll_params { bclk: 24576000, mclk_src_sel: 1, bclk_prediv: 3, pll_div_int: 0x40, pll_div_frac: 0x000000, pll_mode: 0x03, pll_divout: 0x10, mclk_int: 12288000 },
];

unsafe fn cs42l84_pll_config(component: *mut snd_soc_component) -> c_int {
    let cs42l84 = snd_soc_component_get_drvdata(component) as *mut cs42l84_private;
    let mut i: usize;
    let clk: u32 = (*cs42l84).bclk as u32;
    let fsync: u32;

    if (*cs42l84).stream_use != 0 {
        if pll_ratio_table[(*cs42l84).pll_config as usize].bclk == clk {
            return 0;
        } else {
            return -EBUSY;
        }
    }

    i = 0;
    while i < pll_ratio_table.len() {
        if pll_ratio_table[i].bclk == clk {
            (*cs42l84).pll_config = i as c_int;
            break;
        }
        i += 1;
    }
    if i == pll_ratio_table.len() {
        return -EINVAL;
    }

    fsync = clk / (*cs42l84).srate;
    if fsync * (*cs42l84).srate != clk || fsync % 2 != 0 {
        dev_err((*component).dev, c"Unsupported bclk %d/sample rate %d\n".as_ptr(), clk as c_int, (*cs42l84).srate as c_int);
        return -EINVAL;
    }

    snd_soc_component_update_bits(component, CS42L84_ASP_FSYNC_CTL2, CS42L84_ASP_FSYNC_CTL2_BCLK_PERIOD_LO, FIELD_PREP(CS42L84_ASP_FSYNC_CTL2_BCLK_PERIOD_LO, fsync & 0x7f));
    snd_soc_component_update_bits(component, CS42L84_ASP_FSYNC_CTL3, CS42L84_ASP_FSYNC_CTL3_BCLK_PERIOD_HI, FIELD_PREP(CS42L84_ASP_FSYNC_CTL3_BCLK_PERIOD_HI, fsync >> 7));

    match pll_ratio_table[i].mclk_int {
        12000000 => (*cs42l84).pll_mclk_f = CS42L84_CCM_CTL1_MCLK_F_12MHZ as u8,
        11289600 | 12288000 => (*cs42l84).pll_mclk_f = CS42L84_CCM_CTL1_MCLK_F_12_288KHZ as u8,
        24000000 => (*cs42l84).pll_mclk_f = CS42L84_CCM_CTL1_MCLK_F_24MHZ as u8,
        24576000 => (*cs42l84).pll_mclk_f = CS42L84_CCM_CTL1_MCLK_F_24_576KHZ as u8,
        _ => {}
    }

    snd_soc_component_update_bits(component, CS42L84_PLL_CTL1, CS42L84_PLL_CTL1_EN, 0);
    if pll_ratio_table[i].mclk_src_sel != 0 {
        snd_soc_component_update_bits(component, CS42L84_CCM_CTL3, CS42L84_CCM_CTL3_REFCLK_DIV, FIELD_PREP(CS42L84_CCM_CTL3_REFCLK_DIV, pll_ratio_table[i].bclk_prediv as c_uint));
        snd_soc_component_write(component, CS42L84_PLL_DIV_INT, pll_ratio_table[i].pll_div_int as c_uint);
        snd_soc_component_write(component, CS42L84_PLL_DIV_FRAC0, pll_ratio_table[i].pll_div_frac);
        snd_soc_component_write(component, CS42L84_PLL_DIV_FRAC1, pll_ratio_table[i].pll_div_frac >> 8);
        snd_soc_component_write(component, CS42L84_PLL_DIV_FRAC2, pll_ratio_table[i].pll_div_frac >> 16);
        snd_soc_component_update_bits(component, CS42L84_PLL_CTL1, CS42L84_PLL_CTL1_MODE, FIELD_PREP(CS42L84_PLL_CTL1_MODE, pll_ratio_table[i].pll_mode as c_uint));
        snd_soc_component_write(component, CS42L84_PLL_DIVOUT, pll_ratio_table[i].pll_divout as c_uint);
    }
    0
}

unsafe extern "C" fn cs42l84_set_dai_fmt(_codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BC_FC => {}
        _ => return -EINVAL,
    }
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {}
        _ => return -EINVAL,
    }
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_IB_IF => {}
        _ => return -EINVAL,
    }
    0
}

unsafe extern "C" fn cs42l84_pcm_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let cs42l84 = snd_soc_component_get_drvdata(component) as *mut cs42l84_private;
    let mut ret: c_int;
    let ccm_samp_rate: u32;

    (*cs42l84).srate = params_rate(params);
    ret = cs42l84_pll_config(component);
    if ret != 0 { return ret; }

    ccm_samp_rate = match params_rate(params) {
        44100 => CS42L84_CCM_SAMP_RATE_RATE_44K1HZ,
        48000 => CS42L84_CCM_SAMP_RATE_RATE_48KHZ,
        88200 => CS42L84_CCM_SAMP_RATE_RATE_88K2HZ,
        96000 => CS42L84_CCM_SAMP_RATE_RATE_96KHZ,
        176400 => CS42L84_CCM_SAMP_RATE_RATE_176K4HZ,
        192000 => CS42L84_CCM_SAMP_RATE_RATE_192KHZ,
        _ => return -EINVAL,
    };

    snd_soc_component_write(component, CS42L84_CCM_SAMP_RATE, ccm_samp_rate);
    match (*substream).stream {
        SNDRV_PCM_STREAM_PLAYBACK => {
            snd_soc_component_write(component, CS42L84_ASP_RX_CH1_WIDTH, (params_width(params) - 1) as c_uint);
            snd_soc_component_write(component, CS42L84_ASP_RX_CH2_WIDTH, (params_width(params) - 1) as c_uint);
        }
        SNDRV_PCM_STREAM_CAPTURE => {
            snd_soc_component_write(component, CS42L84_ASP_TX_CH1_WIDTH, (params_width(params) - 1) as c_uint);
            snd_soc_component_write(component, CS42L84_ASP_TX_CH2_WIDTH, (params_width(params) - 1) as c_uint);
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn cs42l84_set_sysclk(dai: *mut snd_soc_dai, _clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*dai).component;
    let cs42l84 = snd_soc_component_get_drvdata(component) as *mut cs42l84_private;
    let mut i = 0usize;
    if freq == 0 {
        (*cs42l84).bclk = 0;
        return 0;
    }
    while i < pll_ratio_table.len() {
        if pll_ratio_table[i].bclk == freq {
            (*cs42l84).bclk = freq as c_int;
            return 0;
        }
        i += 1;
    }
    dev_err((*component).dev, c"BCLK %u not supported\n".as_ptr(), freq);
    -EINVAL
}

unsafe extern "C" fn cs42l84_mute_stream(dai: *mut snd_soc_dai, mute: c_int, stream: c_int) -> c_int {
    let component = (*dai).component;
    let cs42l84 = snd_soc_component_get_drvdata(component) as *mut cs42l84_private;
    let mut regval: c_uint = 0;
    let mut ret: c_int;

    if mute != 0 {
        if stream == SNDRV_PCM_STREAM_PLAYBACK {
            snd_soc_component_update_bits(component, CS42L84_DAC_CTL1, CS42L84_DAC_CTL1_UNMUTE, 0);
        }
        (*cs42l84).stream_use &= !(1u8 << stream);
        if (*cs42l84).stream_use == 0 {
            snd_soc_component_write(component, CS42L84_CCM_CTL1, CS42L84_CCM_CTL1_RCO);
            usleep_range(150, 300);
            snd_soc_component_update_bits(component, CS42L84_PLL_CTL1, CS42L84_PLL_CTL1_EN, 0);
            snd_soc_component_update_bits(component, CS42L84_CCM_CTL4, CS42L84_CCM_CTL4_REFCLK_EN, 0);
        }
    } else {
        if (*cs42l84).stream_use == 0 {
            snd_soc_component_update_bits(component, CS42L84_CCM_CTL4, CS42L84_CCM_CTL4_REFCLK_EN, CS42L84_CCM_CTL4_REFCLK_EN);
            if pll_ratio_table[(*cs42l84).pll_config as usize].mclk_src_sel != 0 {
                snd_soc_component_update_bits(component, CS42L84_PLL_CTL1, CS42L84_PLL_CTL1_EN, CS42L84_PLL_CTL1_EN);
                /* TODO: should we be doing something with divout here? */
                ret = regmap_read_poll_timeout((*cs42l84).regmap, CS42L84_PLL_LOCK_STATUS, &mut regval, CS42L84_PLL_LOCK_STATUS_LOCKED, CS42L84_PLL_LOCK_POLL_US, CS42L84_PLL_LOCK_TIMEOUT_US);
                if ret < 0 {
                    dev_warn((*component).dev, c"PLL failed to lock: %d\n".as_ptr(), ret);
                }
                if (regval & CS42L84_PLL_LOCK_STATUS_ERROR) != 0 {
                    dev_warn((*component).dev, c"PLL lock error\n".as_ptr());
                }
                snd_soc_component_update_bits(component, CS42L84_CCM_CTL1, CS42L84_CCM_CTL1_MCLK_SRC | CS42L84_CCM_CTL1_MCLK_FREQ, FIELD_PREP(CS42L84_CCM_CTL1_MCLK_SRC, CS42L84_CCM_CTL1_MCLK_SRC_PLL) | FIELD_PREP(CS42L84_CCM_CTL1_MCLK_FREQ, (*cs42l84).pll_mclk_f as c_uint));
                usleep_range(CS42L84_CLOCK_SWITCH_DELAY_US, CS42L84_CLOCK_SWITCH_DELAY_US * 2);
            } else {
                snd_soc_component_update_bits(component, CS42L84_CCM_CTL1, CS42L84_CCM_CTL1_MCLK_SRC | CS42L84_CCM_CTL1_MCLK_FREQ, FIELD_PREP(CS42L84_CCM_CTL1_MCLK_SRC, CS42L84_CCM_CTL1_MCLK_SRC_BCLK) | FIELD_PREP(CS42L84_CCM_CTL1_MCLK_FREQ, (*cs42l84).pll_mclk_f as c_uint));
                usleep_range(CS42L84_CLOCK_SWITCH_DELAY_US, CS42L84_CLOCK_SWITCH_DELAY_US * 2);
            }
        }
        (*cs42l84).stream_use |= 1u8 << stream;
        if stream == SNDRV_PCM_STREAM_PLAYBACK {
            snd_soc_component_update_bits(component, CS42L84_DAC_CTL1, CS42L84_DAC_CTL1_UNMUTE, CS42L84_DAC_CTL1_UNMUTE);
        }
    }
    0
}

unsafe fn regmap_read_poll_timeout(map: *mut regmap, addr: c_uint, regval: *mut c_uint, cond_mask: c_uint, _sleep_us: c_uint, _timeout_us: c_uint) -> c_int {
    let ret = regmap_read(map, addr, regval);
    if ret < 0 { return ret; }
    if (*regval & cond_mask) != 0 { 0 } else { -ETIMEDOUT }
}
const ETIMEDOUT: c_int = 110;

/*
static const struct snd_soc_dai_ops cs42l84_ops = {
    .hw_params = cs42l84_pcm_hw_params,
    .set_fmt = cs42l84_set_dai_fmt,
    .set_sysclk = cs42l84_set_sysclk,
    .mute_stream = cs42l84_mute_stream,
};
#define CS42L84_FORMATS (SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE)
static struct snd_soc_dai_driver cs42l84_dai = { ... };
*/
const CS42L84_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

#[repr(C)]
struct cs42l84_irq_params {
    status_addr: u16,
    mask_addr: u16,
    mask: u8,
}

static irq_params_table: [cs42l84_irq_params; 1] = [
    cs42l84_irq_params {
        status_addr: CS42L84_TSRS_PLUG_INT_STATUS as u16,
        mask_addr: CS42L84_TSRS_PLUG_INT_MASK as u16,
        mask: CS42L84_TSRS_PLUG_VAL_MASK as u8,
    },
];

unsafe fn cs42l84_detect_hs(cs42l84: *mut cs42l84_private) {
    let mut reg: c_uint = 0;
    regmap_update_bits((*cs42l84).regmap, CS42L84_MISC_DET_CTL, CS42L84_MISC_DET_CTL_HSBIAS_CTL | CS42L84_MISC_DET_CTL_DETECT_MODE, FIELD_PREP(CS42L84_MISC_DET_CTL_HSBIAS_CTL, 3) | FIELD_PREP(CS42L84_MISC_DET_CTL_DETECT_MODE, 0));
    regmap_update_bits((*cs42l84).regmap, CS42L84_MISC_DET_CTL, CS42L84_MISC_DET_CTL_PDN_MIC_LVL_DET, 0);
    /* TODO: Optimize */
    msleep(50);
    regmap_write((*cs42l84).regmap, CS42L84_HS_SWITCH_CTL, CS42L84_HS_SWITCH_CTL_REF_HS3 | CS42L84_HS_SWITCH_CTL_HSB_FILT_HS3 | CS42L84_HS_SWITCH_CTL_GNDHS_HS3 | CS42L84_HS_SWITCH_CTL_HSB_HS4);
    regmap_update_bits((*cs42l84).regmap, CS42L84_HS_DET_CTL2, CS42L84_HS_DET_CTL2_SET, FIELD_PREP(CS42L84_HS_DET_CTL2_SET, 0));
    regmap_update_bits((*cs42l84).regmap, CS42L84_MISC_DET_CTL, CS42L84_MISC_DET_CTL_DETECT_MODE, FIELD_PREP(CS42L84_MISC_DET_CTL_DETECT_MODE, 3));
    /* TODO: Optimize */
    msleep(50);
    regmap_read((*cs42l84).regmap, CS42L84_HS_DET_STATUS2, &mut reg);
    regmap_update_bits((*cs42l84).regmap, CS42L84_MISC_DET_CTL, CS42L84_MISC_DET_CTL_PDN_MIC_LVL_DET, CS42L84_MISC_DET_CTL_PDN_MIC_LVL_DET);

    match reg & 0b11 {
        0b11 | 0b00 => {
            regmap_update_bits((*cs42l84).regmap, CS42L84_MISC_DET_CTL, CS42L84_MISC_DET_CTL_HSBIAS_CTL, FIELD_PREP(CS42L84_MISC_DET_CTL_HSBIAS_CTL, 1));
        }
        _ => {}
    }
    match reg & 0b11 {
        0b10 => {
            dev_dbg((*cs42l84).dev, c"Detected mic\n".as_ptr());
            (*cs42l84).hs_type = SND_JACK_HEADSET;
            snd_soc_jack_report((*cs42l84).jack, SND_JACK_HEADSET, SND_JACK_HEADSET);
        }
        0b00 | 0b11 | _ => {
            if (reg & 0b11) == 0b00 {
                dev_dbg((*cs42l84).dev, c"Detected open circuit on HS4\n".as_ptr());
            }
            snd_soc_jack_report((*cs42l84).jack, SND_JACK_HEADPHONE, SND_JACK_HEADSET);
            (*cs42l84).hs_type = SND_JACK_HEADPHONE;
            dev_dbg((*cs42l84).dev, c"Detected bare headphone (no mic)\n".as_ptr());
        }
    }
}

unsafe fn cs42l84_revert_hs(cs42l84: *mut cs42l84_private) {
    regmap_update_bits((*cs42l84).regmap, CS42L84_MISC_DET_CTL, CS42L84_MISC_DET_CTL_HSBIAS_CTL | CS42L84_MISC_DET_CTL_DETECT_MODE, FIELD_PREP(CS42L84_MISC_DET_CTL_HSBIAS_CTL, 1) | FIELD_PREP(CS42L84_MISC_DET_CTL_DETECT_MODE, 0));
    regmap_write((*cs42l84).regmap, CS42L84_HS_SWITCH_CTL, CS42L84_HS_SWITCH_CTL_REF_HS3 | CS42L84_HS_SWITCH_CTL_REF_HS4 | CS42L84_HS_SWITCH_CTL_HSB_FILT_HS3 | CS42L84_HS_SWITCH_CTL_HSB_FILT_HS4 | CS42L84_HS_SWITCH_CTL_GNDHS_HS3 | CS42L84_HS_SWITCH_CTL_GNDHS_HS4);
    regmap_update_bits((*cs42l84).regmap, CS42L84_HS_DET_CTL2, CS42L84_HS_DET_CTL2_SET, FIELD_PREP(CS42L84_HS_DET_CTL2_SET, 2));
}

unsafe fn cs42l84_set_interrupt_masks(cs42l84: *mut cs42l84_private, val: c_uint) {
    regmap_update_bits((*cs42l84).regmap, CS42L84_TSRS_PLUG_INT_MASK, CS42L84_RS_PLUG | CS42L84_RS_UNPLUG | CS42L84_TS_PLUG | CS42L84_TS_UNPLUG, val);
}

unsafe extern "C" fn cs42l84_irq_thread(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let cs42l84 = data as *mut cs42l84_private;
    let mut stickies = [0u32; 1];
    let mut masks = [0u32; 1];
    let mut reg: c_uint = 0;
    let mut current_tip_state: u8;
    let mut current_ring_state: u8;
    let mut i = 0usize;

    mutex_lock(&mut (*cs42l84).irq_lock);
    while i < stickies.len() {
        regmap_read((*cs42l84).regmap, irq_params_table[i].status_addr as c_uint, &mut stickies[i]);
        regmap_read((*cs42l84).regmap, irq_params_table[i].mask_addr as c_uint, &mut masks[i]);
        stickies[i] = stickies[i] & !masks[i] & irq_params_table[i].mask as c_uint;
        i += 1;
    }

    if ((!masks[0]) & irq_params_table[0].mask as c_uint) != 0 {
        regmap_read((*cs42l84).regmap, CS42L84_TSRS_PLUG_STATUS, &mut reg);
        current_tip_state = (((reg as c_char as c_int) as c_uint & (CS42L84_TS_PLUG | CS42L84_TS_UNPLUG)) >> CS42L84_TS_PLUG_SHIFT) as u8;
        if current_tip_state != (*cs42l84).tip_state {
            (*cs42l84).tip_state = current_tip_state;
            match current_tip_state as c_uint {
                CS42L84_PLUG => {
                    dev_dbg((*cs42l84).dev, c"Plug event\n".as_ptr());
                    cs42l84_detect_hs(cs42l84);
                    regmap_read((*cs42l84).regmap, CS42L84_TSRS_PLUG_STATUS, &mut reg);
                    current_tip_state = (((reg as c_char as c_int) as c_uint & (CS42L84_TS_PLUG | CS42L84_TS_UNPLUG)) >> CS42L84_TS_PLUG_SHIFT) as u8;
                    if current_tip_state as c_uint != CS42L84_PLUG {
                        dev_dbg((*cs42l84).dev, c"Wobbly connection, detection invalidated\n".as_ptr());
                        (*cs42l84).tip_state = CS42L84_UNPLUG as u8;
                        cs42l84_revert_hs(cs42l84);
                    }
                    cs42l84_set_interrupt_masks(cs42l84, 0);
                }
                CS42L84_UNPLUG => {
                    (*cs42l84).ring_state = CS42L84_UNPLUG as u8;
                    dev_dbg((*cs42l84).dev, c"Unplug event\n".as_ptr());
                    cs42l84_revert_hs(cs42l84);
                    (*cs42l84).hs_type = 0;
                    snd_soc_jack_report((*cs42l84).jack, 0, SND_JACK_HEADSET);
                    cs42l84_set_interrupt_masks(cs42l84, CS42L84_RS_PLUG | CS42L84_RS_UNPLUG);
                }
                _ => {
                    (*cs42l84).ring_state = CS42L84_TRANS as u8;
                }
            }
            mutex_unlock(&mut (*cs42l84).irq_lock);
            return IRQ_HANDLED;
        }

        current_ring_state = (((reg as c_char as c_int) as c_uint & (CS42L84_RS_PLUG | CS42L84_RS_UNPLUG)) >> CS42L84_RS_PLUG_SHIFT) as u8;
        if current_ring_state != (*cs42l84).ring_state {
            (*cs42l84).ring_state = current_ring_state;
            if current_ring_state as c_uint == CS42L84_PLUG {
                cs42l84_detect_hs(cs42l84);
            }
        }
    }
    mutex_unlock(&mut (*cs42l84).irq_lock);
    IRQ_HANDLED
}

unsafe fn cs42l84_setup_plug_detect(cs42l84: *mut cs42l84_private) {
    let mut reg: c_uint = 0;
    regmap_update_bits((*cs42l84).regmap, CS42L84_MIC_DET_CTL4, CS42L84_MIC_DET_CTL4_LATCH_TO_VP, CS42L84_MIC_DET_CTL4_LATCH_TO_VP);
    regmap_update_bits((*cs42l84).regmap, CS42L84_TIP_SENSE_CTL2, CS42L84_TIP_SENSE_CTL2_MODE, FIELD_PREP(CS42L84_TIP_SENSE_CTL2_MODE, CS42L84_TIP_SENSE_CTL2_MODE_SHORT_DET));
    regmap_update_bits((*cs42l84).regmap, CS42L84_RING_SENSE_CTL, CS42L84_RING_SENSE_CTL_INV | CS42L84_RING_SENSE_CTL_UNK1 | CS42L84_RING_SENSE_CTL_RISETIME | CS42L84_RING_SENSE_CTL_FALLTIME, CS42L84_RING_SENSE_CTL_INV | CS42L84_RING_SENSE_CTL_UNK1 | FIELD_PREP(CS42L84_RING_SENSE_CTL_RISETIME, CS42L84_DEBOUNCE_TIME_125MS) | FIELD_PREP(CS42L84_RING_SENSE_CTL_FALLTIME, CS42L84_DEBOUNCE_TIME_125MS));
    regmap_update_bits((*cs42l84).regmap, CS42L84_TIP_SENSE_CTL, CS42L84_TIP_SENSE_CTL_INV | CS42L84_TIP_SENSE_CTL_RISETIME | CS42L84_TIP_SENSE_CTL_FALLTIME, CS42L84_TIP_SENSE_CTL_INV | FIELD_PREP(CS42L84_TIP_SENSE_CTL_RISETIME, CS42L84_DEBOUNCE_TIME_500MS) | FIELD_PREP(CS42L84_TIP_SENSE_CTL_FALLTIME, CS42L84_DEBOUNCE_TIME_125MS));
    regmap_update_bits((*cs42l84).regmap, CS42L84_MSM_BLOCK_EN3, CS42L84_MSM_BLOCK_EN3_TR_SENSE, CS42L84_MSM_BLOCK_EN3_TR_SENSE);
    regmap_read((*cs42l84).regmap, CS42L84_TSRS_PLUG_STATUS, &mut reg);
    (*cs42l84).tip_state = (((reg as c_char as c_int) as c_uint & (CS42L84_TS_PLUG | CS42L84_TS_UNPLUG)) >> CS42L84_TS_PLUG_SHIFT) as u8;
    regmap_update_bits((*cs42l84).regmap, CS42L84_MIC_DET_CTL1, CS42L84_MIC_DET_CTL1_HS_DET_LEVEL, FIELD_PREP(CS42L84_MIC_DET_CTL1_HS_DET_LEVEL, 0x2c));
    regmap_write((*cs42l84).regmap, CS42L84_HS_SWITCH_CTL, CS42L84_HS_SWITCH_CTL_REF_HS3 | CS42L84_HS_SWITCH_CTL_REF_HS4 | CS42L84_HS_SWITCH_CTL_HSB_FILT_HS3 | CS42L84_HS_SWITCH_CTL_HSB_FILT_HS4 | CS42L84_HS_SWITCH_CTL_GNDHS_HS3 | CS42L84_HS_SWITCH_CTL_GNDHS_HS4);
    regmap_update_bits((*cs42l84).regmap, CS42L84_HS_DET_CTL2, CS42L84_HS_DET_CTL2_SET | CS42L84_HS_DET_CTL2_CTL, FIELD_PREP(CS42L84_HS_DET_CTL2_SET, 2) | FIELD_PREP(CS42L84_HS_DET_CTL2_CTL, 0));
    regmap_update_bits((*cs42l84).regmap, CS42L84_HS_CLAMP_DISABLE, 1, 1);
}

unsafe extern "C" fn cs42l84_i2c_probe(i2c_client: *mut i2c_client) -> c_int {
    let cs42l84: *mut cs42l84_private;
    let mut ret: c_int;
    let devid: c_int;
    let mut reg: c_uint = 0;

    cs42l84 = devm_kzalloc(&mut (*i2c_client).dev, core::mem::size_of::<cs42l84_private>(), GFP_KERNEL) as *mut cs42l84_private;
    if cs42l84.is_null() {
        return -ENOMEM;
    }

    (*cs42l84).dev = &mut (*i2c_client).dev;
    i2c_set_clientdata(i2c_client, cs42l84 as *mut c_void);
    mutex_init(&mut (*cs42l84).irq_lock);

    (*cs42l84).regmap = devm_regmap_init_i2c(i2c_client, &cs42l84_regmap);
    if IS_ERR((*cs42l84).regmap) {
        ret = PTR_ERR((*cs42l84).regmap);
        dev_err(&mut (*i2c_client).dev, c"regmap_init() failed: %d\n".as_ptr(), ret);
        return ret;
    }

    (*cs42l84).reset_gpio = devm_gpiod_get_optional(&mut (*i2c_client).dev, c"reset".as_ptr(), GPIOD_OUT_LOW);
    if IS_ERR((*cs42l84).reset_gpio) {
        ret = PTR_ERR((*cs42l84).reset_gpio);
        return ret;
    }
    if !(*cs42l84).reset_gpio.is_null() {
        dev_dbg(&mut (*i2c_client).dev, c"Found reset GPIO\n".as_ptr());
        gpiod_set_value_cansleep((*cs42l84).reset_gpio, 1);
    }
    usleep_range(CS42L84_BOOT_TIME_US, CS42L84_BOOT_TIME_US * 2);

    if (*i2c_client).irq != 0 {
        ret = request_threaded_irq((*i2c_client).irq, ptr::null(), cs42l84_irq_thread, IRQF_ONESHOT, c"cs42l84".as_ptr(), cs42l84 as *mut c_void);
        if ret == -EPROBE_DEFER {
            gpiod_set_value_cansleep((*cs42l84).reset_gpio, 0);
            return ret;
        } else if ret != 0 {
            dev_err(&mut (*i2c_client).dev, c"Failed to request IRQ: %d\n".as_ptr(), ret);
            gpiod_set_value_cansleep((*cs42l84).reset_gpio, 0);
            return ret;
        }
    }

    devid = cirrus_read_device_id((*cs42l84).regmap, CS42L84_DEVID);
    if devid < 0 {
        ret = devid;
        dev_err(&mut (*i2c_client).dev, c"Failed to read device ID: %d\n".as_ptr(), ret);
        if (*i2c_client).irq != 0 { free_irq((*i2c_client).irq, cs42l84 as *mut c_void); }
        gpiod_set_value_cansleep((*cs42l84).reset_gpio, 0);
        return ret;
    }
    if devid != CS42L84_CHIP_ID as c_int {
        dev_err(&mut (*i2c_client).dev, c"CS42L84 Device ID (%X). Expected %X\n".as_ptr(), devid, CS42L84_CHIP_ID);
        ret = -EINVAL;
        if (*i2c_client).irq != 0 { free_irq((*i2c_client).irq, cs42l84 as *mut c_void); }
        gpiod_set_value_cansleep((*cs42l84).reset_gpio, 0);
        return ret;
    }
    ret = regmap_read((*cs42l84).regmap, CS42L84_REVID, &mut reg);
    if ret < 0 {
        dev_err(&mut (*i2c_client).dev, c"Get Revision ID failed\n".as_ptr());
        gpiod_set_value_cansleep((*cs42l84).reset_gpio, 0);
        return ret;
    }
    dev_info(&mut (*i2c_client).dev, c"Cirrus Logic CS42L84, Revision: %02X\n".as_ptr(), reg & 0xFF);
    cs42l84_setup_plug_detect(cs42l84);
    cs42l84_set_interrupt_masks(cs42l84, CS42L84_RS_PLUG | CS42L84_RS_UNPLUG);
    ret = devm_snd_soc_register_component(&mut (*i2c_client).dev, &soc_component_dev_cs42l84, &raw mut cs42l84_dai, 1);
    if ret < 0 {
        gpiod_set_value_cansleep((*cs42l84).reset_gpio, 0);
        return ret;
    }
    0
}

unsafe extern "C" fn cs42l84_i2c_remove(i2c_client: *mut i2c_client) {
    let cs42l84 = i2c_get_clientdata(i2c_client) as *mut cs42l84_private;
    if (*i2c_client).irq != 0 {
        free_irq((*i2c_client).irq, cs42l84 as *mut c_void);
    }
    gpiod_set_value_cansleep((*cs42l84).reset_gpio, 0);
}

/*
static const struct of_device_id cs42l84_of_match[] = {
    { .compatible = "cirrus,cs42l84", },
    {}
};
MODULE_DEVICE_TABLE(of, cs42l84_of_match);

static const struct i2c_device_id cs42l84_id[] = {
    { .name = "cs42l84" },
    { }
};
MODULE_DEVICE_TABLE(i2c, cs42l84_id);

static struct i2c_driver cs42l84_i2c_driver = {
    .driver = {
        .name = "cs42l84",
        .of_match_table = cs42l84_of_match,
    },
    .id_table = cs42l84_id,
    .probe = cs42l84_i2c_probe,
    .remove = cs42l84_i2c_remove,
};

module_i2c_driver(cs42l84_i2c_driver);

MODULE_DESCRIPTION("ASoC CS42L84 driver");
MODULE_AUTHOR("Martin Povišer <povik+lin@cutebit.org>");
MODULE_AUTHOR("Hector Martin <marcan@marcan.st>");
MODULE_AUTHOR("James Calligeros <jcalligeros99@gmail.com>");
MODULE_LICENSE("GPL");
*/

unsafe extern "C" {
    static CS42L84_DEVID: c_uint;
}

/* Constants below are supplied by cs42l84.h, ALSA, Linux, and Cirrus support headers. */
unsafe extern "C" {
    static CS42L84_DEVID_PLUS_5: c_uint;
    static CS42L84_TSRS_PLUG_INT_STATUS: c_uint;
    static CS42L84_PLL_LOCK_STATUS: c_uint;
    static CS42L84_TSRS_PLUG_STATUS: c_uint;
    static CS42L84_HS_DET_STATUS2: c_uint;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
