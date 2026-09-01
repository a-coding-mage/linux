// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8804.c  --  WM8804 S/PDIF transceiver driver
 *
 * Copyright 2010-11 Wolfson Microelectronics plc
 *
 * Author: Dimitris Papastamos <dp@opensource.wolfsonmicro.com>
 */

// C include dependencies translated as external Rust dependencies:
// linux/module.h, linux/moduleparam.h, linux/init.h, linux/gpio/consumer.h,
// linux/delay.h, linux/pm.h, linux/pm_runtime.h, linux/regulator/consumer.h,
// linux/slab.h, sound/core.h, sound/pcm.h, sound/pcm_params.h, sound/soc.h,
// sound/initval.h, sound/tlv.h, sound/soc-dapm.h, and "wm8804.h".

use core::ffi::{c_char, c_int, c_ulong, c_void};

type bool_ = bool;
type u16 = u16;
type u32 = u32;
type u64 = u64;

const WM8804_NUM_SUPPLIES: usize = 2;

extern "C" {
    static WM8804_SPDTX4: c_uint;
    static WM8804_PWRDN: c_uint;
    static WM8804_RST_DEVID1: c_uint;
    static WM8804_DEVID2: c_uint;
    static WM8804_DEVREV: c_uint;
    static WM8804_INTSTAT: c_uint;
    static WM8804_SPDSTAT: c_uint;
    static WM8804_RXCHAN1: c_uint;
    static WM8804_RXCHAN2: c_uint;
    static WM8804_RXCHAN3: c_uint;
    static WM8804_RXCHAN4: c_uint;
    static WM8804_RXCHAN5: c_uint;
    static WM8804_AIFTX: c_uint;
    static WM8804_AIFRX: c_uint;
    static WM8804_PLL1: c_uint;
    static WM8804_PLL2: c_uint;
    static WM8804_PLL3: c_uint;
    static WM8804_PLL4: c_uint;
    static WM8804_PLL5: c_uint;
    static WM8804_PLL6: c_uint;
    static WM8804_MAX_REGISTER: c_uint;
    static WM8804_TX_CLKSRC_MCLK: c_int;
    static WM8804_TX_CLKSRC_PLL: c_int;
    static WM8804_CLKOUT_SRC_CLK1: c_int;
    static WM8804_CLKOUT_SRC_OSCCLK: c_int;
    static WM8804_CLKOUT_DIV: c_int;
    static WM8804_MCLK_DIV: c_int;
}

type c_uint = u32;

const REGULATOR_EVENT_DISABLE: c_ulong = 0;
const SND_SOC_DAPM_POST_PMU: c_int = 0;
const SND_SOC_DAPM_POST_PMD: c_int = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_RIGHT_J: c_uint = 0;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 0;
const SND_SOC_DAIFMT_DSP_A: c_uint = 0;
const SND_SOC_DAIFMT_DSP_B: c_uint = 0;
const SND_SOC_DAIFMT_MASTER_MASK: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_IB_IF: c_uint = 0;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0;
const SND_SOC_DAIFMT_NB_IF: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S20_3LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S24_LE: c_uint = 0;
const SNDRV_PCM_RATE_32000: c_uint = 0;
const SNDRV_PCM_RATE_44100: c_uint = 0;
const SNDRV_PCM_RATE_48000: c_uint = 0;
const SNDRV_PCM_RATE_64000: c_uint = 0;
const SNDRV_PCM_RATE_88200: c_uint = 0;
const SNDRV_PCM_RATE_96000: c_uint = 0;
const SNDRV_PCM_RATE_176400: c_uint = 0;
const SNDRV_PCM_RATE_192000: c_uint = 0;
const SND_SOC_NOPM: c_int = 0;
const GPIOD_OUT_LOW: c_int = 0;
const GFP_KERNEL: c_int = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const REGCACHE_MAPLE: c_int = 0;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regulator {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}
#[repr(C)]
pub struct snd_ctl_elem_value_value {
    pub enumerated: snd_ctl_elem_value_enumerated,
}
#[repr(C)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 4],
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}
#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
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
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub dev: *mut device,
}
#[repr(C)]
pub struct soc_enum {
    pub reg: c_uint,
    pub shift_l: c_uint,
}
#[repr(C)]
pub struct notifier_block {
    pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int>,
}
#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
    pub consumer: *mut regulator,
}
#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}
#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget_init {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_clkdiv: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub set_pll: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int, c_uint, c_uint) -> c_int>,
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
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub dapm_widgets: *const snd_soc_dapm_widget_init,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}
#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub cache_type: c_int,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
}

#[repr(C)]
struct wm8804_priv {
    dev: *mut device,
    regmap: *mut regmap,
    supplies: [regulator_bulk_data; WM8804_NUM_SUPPLIES],
    disable_nb: [notifier_block; WM8804_NUM_SUPPLIES],
    mclk_div: c_int,
    reset: *mut gpio_desc,
    aif_pwr: c_int,
}

extern "C" {
    fn regcache_mark_dirty(map: *mut regmap);
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_dapm_kcontrol_to_component(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_mutex_lock(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_component_test_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> bool_;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_dapm_mutex_unlock(dapm: *mut snd_soc_dapm_context);
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn regmap_update_bits_check(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint, change: *mut bool_) -> c_int;
    fn pm_runtime_put(dev: *mut device) -> c_int;
    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_int) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_regulator_bulk_get(dev: *mut device, num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn devm_regulator_register_notifier(regulator: *mut regulator, nb: *mut notifier_block) -> c_int;
    fn regulator_bulk_enable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn pm_runtime_set_active(dev: *mut device) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_idle(dev: *mut device) -> c_int;
    fn regulator_bulk_disable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn pm_runtime_disable(dev: *mut device);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
}

static wm8804_supply_names: [*const c_char; WM8804_NUM_SUPPLIES] = [
    b"PVDD\0".as_ptr() as *const c_char,
    b"DVDD\0".as_ptr() as *const c_char,
];

static wm8804_reg_defaults: [reg_default; 20] = [
    reg_default { reg: 3, def: 0x21 },     /* R3  - PLL1 */
    reg_default { reg: 4, def: 0xFD },     /* R4  - PLL2 */
    reg_default { reg: 5, def: 0x36 },     /* R5  - PLL3 */
    reg_default { reg: 6, def: 0x07 },     /* R6  - PLL4 */
    reg_default { reg: 7, def: 0x16 },     /* R7  - PLL5 */
    reg_default { reg: 8, def: 0x18 },     /* R8  - PLL6 */
    reg_default { reg: 9, def: 0xFF },     /* R9  - SPDMODE */
    reg_default { reg: 10, def: 0x00 },    /* R10 - INTMASK */
    reg_default { reg: 18, def: 0x00 },    /* R18 - SPDTX1 */
    reg_default { reg: 19, def: 0x00 },    /* R19 - SPDTX2 */
    reg_default { reg: 20, def: 0x00 },    /* R20 - SPDTX3 */
    reg_default { reg: 21, def: 0x71 },    /* R21 - SPDTX4 */
    reg_default { reg: 22, def: 0x0B },    /* R22 - SPDTX5 */
    reg_default { reg: 23, def: 0x70 },    /* R23 - GPO0 */
    reg_default { reg: 24, def: 0x57 },    /* R24 - GPO1 */
    reg_default { reg: 26, def: 0x42 },    /* R26 - GPO2 */
    reg_default { reg: 27, def: 0x06 },    /* R27 - AIFTX */
    reg_default { reg: 28, def: 0x06 },    /* R28 - AIFRX */
    reg_default { reg: 29, def: 0x80 },    /* R29 - SPDRX1 */
    reg_default { reg: 30, def: 0x07 },    /* R30 - PWRDN */
];

unsafe extern "C" fn wm8804_regulator_event_0(nb: *mut notifier_block, event: c_ulong, _data: *mut c_void) -> c_int {
    let wm8804 = (nb as *mut u8).offset(-(core::mem::offset_of!(wm8804_priv, disable_nb) as isize)) as *mut wm8804_priv;
    if (event & REGULATOR_EVENT_DISABLE) != 0 {
        regcache_mark_dirty((*wm8804).regmap);
    }
    0
}

unsafe extern "C" fn wm8804_regulator_event_1(nb: *mut notifier_block, event: c_ulong, _data: *mut c_void) -> c_int {
    let base = core::mem::offset_of!(wm8804_priv, disable_nb) + core::mem::size_of::<notifier_block>();
    let wm8804 = (nb as *mut u8).offset(-(base as isize)) as *mut wm8804_priv;
    if (event & REGULATOR_EVENT_DISABLE) != 0 {
        regcache_mark_dirty((*wm8804).regmap);
    }
    0
}

static txsrc_text: [*const c_char; 2] = [
    b"S/PDIF RX\0".as_ptr() as *const c_char,
    b"AIF\0".as_ptr() as *const c_char,
];

// static SOC_ENUM_SINGLE_DECL(txsrc, WM8804_SPDTX4, 6, txsrc_text);
static mut txsrc: soc_enum = soc_enum { reg: 0, shift_l: 6 };

// static const struct snd_kcontrol_new wm8804_tx_source_mux[] =
//     SOC_DAPM_ENUM_EXT("Input Source", txsrc, snd_soc_dapm_get_enum_double, txsrc_put);
static wm8804_tx_source_mux: [snd_kcontrol_new; 0] = [];

// The following DAPM widget array preserves the C declarations' intent:
// SND_SOC_DAPM_OUTPUT("SPDIF Out")
// SND_SOC_DAPM_INPUT("SPDIF In")
// SND_SOC_DAPM_PGA("SPDIFTX", WM8804_PWRDN, 2, 1, NULL, 0)
// SND_SOC_DAPM_PGA("SPDIFRX", WM8804_PWRDN, 1, 1, NULL, 0)
// SND_SOC_DAPM_MUX("Tx Source", SND_SOC_NOPM, 6, 0, wm8804_tx_source_mux)
// SND_SOC_DAPM_AIF_OUT_E("AIFTX", NULL, 0, SND_SOC_NOPM, 0, 0, wm8804_aif_event,
//                        SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD)
// SND_SOC_DAPM_AIF_IN_E("AIFRX", NULL, 0, SND_SOC_NOPM, 0, 0, wm8804_aif_event,
//                       SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD)
static wm8804_dapm_widgets: [snd_soc_dapm_widget_init; 0] = [];

static wm8804_dapm_routes: [snd_soc_dapm_route; 8] = [
    snd_soc_dapm_route { sink: b"AIFRX\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Playback\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Tx Source\0".as_ptr() as *const c_char, control: b"AIF\0".as_ptr() as *const c_char, source: b"AIFRX\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SPDIFRX\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"SPDIF In\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Tx Source\0".as_ptr() as *const c_char, control: b"S/PDIF RX\0".as_ptr() as *const c_char, source: b"SPDIFRX\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SPDIFTX\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Tx Source\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SPDIF Out\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"SPDIFTX\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"AIFTX\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"SPDIFRX\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Capture\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"AIFTX\0".as_ptr() as *const c_char },
];

unsafe extern "C" fn wm8804_aif_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let wm8804 = snd_soc_component_get_drvdata(component) as *mut wm8804_priv;

    match event {
        SND_SOC_DAPM_POST_PMU => {
            /* power up the aif */
            if (*wm8804).aif_pwr == 0 {
                snd_soc_component_update_bits(component, WM8804_PWRDN, 0x10, 0x0);
            }
            (*wm8804).aif_pwr += 1;
        }
        SND_SOC_DAPM_POST_PMD => {
            /* power down only both paths are disabled */
            (*wm8804).aif_pwr -= 1;
            if (*wm8804).aif_pwr == 0 {
                snd_soc_component_update_bits(component, WM8804_PWRDN, 0x10, 0x10);
            }
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn txsrc_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let dapm = snd_soc_component_to_dapm(component);
    let e = (*kcontrol).private_value as *mut soc_enum;
    let val = (*ucontrol).value.enumerated.item[0] << (*e).shift_l;
    let mask = 1u32 << (*e).shift_l;
    let txpwr: c_uint;

    if val != 0 && val != mask {
        return -EINVAL;
    }

    snd_soc_dapm_mutex_lock(dapm);

    if snd_soc_component_test_bits(component, (*e).reg, mask, val) {
        /* save the current power state of the transmitter */
        txpwr = snd_soc_component_read(component, WM8804_PWRDN) & 0x4;

        /* power down the transmitter */
        snd_soc_component_update_bits(component, WM8804_PWRDN, 0x4, 0x4);

        /* set the tx source */
        snd_soc_component_update_bits(component, (*e).reg, mask, val);

        /* restore the transmitter's configuration */
        snd_soc_component_update_bits(component, WM8804_PWRDN, 0x4, txpwr);
    }

    snd_soc_dapm_mutex_unlock(dapm);

    0
}

unsafe extern "C" fn wm8804_volatile(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        x if x == WM8804_RST_DEVID1 => true,
        x if x == WM8804_DEVID2 => true,
        x if x == WM8804_DEVREV => true,
        x if x == WM8804_INTSTAT => true,
        x if x == WM8804_SPDSTAT => true,
        x if x == WM8804_RXCHAN1 => true,
        x if x == WM8804_RXCHAN2 => true,
        x if x == WM8804_RXCHAN3 => true,
        x if x == WM8804_RXCHAN4 => true,
        x if x == WM8804_RXCHAN5 => true,
        _ => false,
    }
}

unsafe extern "C" fn wm8804_soft_reset(wm8804: *mut wm8804_priv) -> c_int {
    regmap_write((*wm8804).regmap, WM8804_RST_DEVID1, 0x0)
}

unsafe extern "C" fn wm8804_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let format: u16;
    let master: u16;
    let mut bcp: u16;
    let mut lrp: u16;

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => format = 0x2,
        x if x == SND_SOC_DAIFMT_RIGHT_J => format = 0x0,
        x if x == SND_SOC_DAIFMT_LEFT_J => format = 0x1,
        x if x == SND_SOC_DAIFMT_DSP_A => format = 0x3,
        x if x == SND_SOC_DAIFMT_DSP_B => format = 0x3,
        _ => {
            dev_err((*dai).dev, b"Unknown dai format\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    /* set data format */
    snd_soc_component_update_bits(component, WM8804_AIFTX, 0x3, format as c_uint);
    snd_soc_component_update_bits(component, WM8804_AIFRX, 0x3, format as c_uint);

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        x if x == SND_SOC_DAIFMT_CBP_CFP => master = 1,
        x if x == SND_SOC_DAIFMT_CBC_CFC => master = 0,
        _ => {
            dev_err((*dai).dev, b"Unknown master/slave configuration\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    /* set master/slave mode */
    snd_soc_component_update_bits(component, WM8804_AIFRX, 0x40, (master << 6) as c_uint);

    bcp = 0;
    lrp = 0;
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => {}
        x if x == SND_SOC_DAIFMT_IB_IF => {
            bcp = 1;
            lrp = 1;
        }
        x if x == SND_SOC_DAIFMT_IB_NF => bcp = 1,
        x if x == SND_SOC_DAIFMT_NB_IF => lrp = 1,
        _ => {
            dev_err((*dai).dev, b"Unknown polarity configuration\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    /* set frame inversion */
    snd_soc_component_update_bits(component, WM8804_AIFTX, 0x10 | 0x20, ((bcp << 4) | (lrp << 5)) as c_uint);
    snd_soc_component_update_bits(component, WM8804_AIFRX, 0x10 | 0x20, ((bcp << 4) | (lrp << 5)) as c_uint);
    0
}

unsafe extern "C" fn wm8804_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let blen: u16;

    match params_width(params) {
        16 => blen = 0x0,
        20 => blen = 0x1,
        24 => blen = 0x2,
        _ => {
            dev_err((*dai).dev, b"Unsupported word length: %u\n\0".as_ptr() as *const c_char, params_width(params));
            return -EINVAL;
        }
    }

    /* set word length */
    snd_soc_component_update_bits(component, WM8804_AIFTX, 0xc, (blen << 2) as c_uint);
    snd_soc_component_update_bits(component, WM8804_AIFRX, 0xc, (blen << 2) as c_uint);

    0
}

#[repr(C)]
struct pll_div {
    prescale: u32,
    mclkdiv: u32,
    freqmode: u32,
    n: u32,
    k: u32,
}

#[repr(C)]
struct post_table_entry {
    div: c_uint,
    freqmode: c_uint,
    mclkdiv: c_uint,
}

/* PLL rate to output rate divisions */
static mut post_table: [post_table_entry; 8] = [
    post_table_entry { div: 2, freqmode: 0, mclkdiv: 0 },
    post_table_entry { div: 4, freqmode: 0, mclkdiv: 1 },
    post_table_entry { div: 4, freqmode: 1, mclkdiv: 0 },
    post_table_entry { div: 8, freqmode: 1, mclkdiv: 1 },
    post_table_entry { div: 8, freqmode: 2, mclkdiv: 0 },
    post_table_entry { div: 16, freqmode: 2, mclkdiv: 1 },
    post_table_entry { div: 12, freqmode: 3, mclkdiv: 0 },
    post_table_entry { div: 24, freqmode: 3, mclkdiv: 1 },
];

const FIXED_PLL_SIZE: u64 = (1u64 << 22) * 10;

unsafe fn pll_factors(pll_div: *mut pll_div, mut target: c_uint, mut source: c_uint, mclk_div: c_uint) -> c_int {
    let mut kpart: u64;
    let mut k: c_ulong;
    let mut ndiv: c_ulong;
    let nmod: c_ulong;
    let mut tmp: c_ulong;
    let mut i: usize;

    /*
     * Scale the output frequency up; the PLL should run in the
     * region of 90-100MHz.
     */
    i = 0;
    while i < post_table.len() {
        tmp = (target as c_ulong).wrapping_mul(post_table[i].div as c_ulong);
        if tmp >= 90000000 && tmp <= 100000000 && mclk_div == post_table[i].mclkdiv {
            (*pll_div).freqmode = post_table[i].freqmode;
            (*pll_div).mclkdiv = post_table[i].mclkdiv;
            target = target.wrapping_mul(post_table[i].div);
            break;
        }
        i += 1;
    }

    if i == post_table.len() {
        pr_err(b"%s: Unable to scale output frequency: %uHz\n\0".as_ptr() as *const c_char, b"pll_factors\0".as_ptr() as *const c_char, target);
        return -EINVAL;
    }

    (*pll_div).prescale = 0;
    ndiv = (target / source) as c_ulong;
    if ndiv < 5 {
        source >>= 1;
        (*pll_div).prescale = 1;
        ndiv = (target / source) as c_ulong;
    }

    if ndiv < 5 || ndiv > 13 {
        pr_err(b"%s: WM8804 N value is not within the recommended range: %lu\n\0".as_ptr() as *const c_char, b"pll_factors\0".as_ptr() as *const c_char, ndiv);
        return -EINVAL;
    }
    (*pll_div).n = ndiv as u32;

    nmod = (target % source) as c_ulong;
    kpart = FIXED_PLL_SIZE.wrapping_mul(nmod as u64);
    kpart /= source as u64;

    k = (kpart & 0xffffffff) as c_ulong;
    if (k % 10) >= 5 {
        k = k.wrapping_add(5);
    }
    k /= 10;
    (*pll_div).k = k as u32;

    0
}

unsafe extern "C" fn wm8804_set_pll(dai: *mut snd_soc_dai, _pll_id: c_int, _source: c_int, freq_in: c_uint, freq_out: c_uint) -> c_int {
    let component = (*dai).component;
    let wm8804 = snd_soc_component_get_drvdata(component) as *mut wm8804_priv;
    let mut change: bool_ = false;

    if freq_in == 0 || freq_out == 0 {
        /* disable the PLL */
        regmap_update_bits_check((*wm8804).regmap, WM8804_PWRDN, 0x1, 0x1, &mut change);
        if change {
            pm_runtime_put((*wm8804).dev);
        }
    } else {
        let ret: c_int;
        let mut divs = pll_div {
            prescale: 0,
            mclkdiv: 0,
            freqmode: 0,
            n: 0,
            k: 0,
        };

        ret = pll_factors(&mut divs, freq_out, freq_in, (*wm8804).mclk_div as c_uint);
        if ret != 0 {
            return ret;
        }

        /* power down the PLL before reprogramming it */
        regmap_update_bits_check((*wm8804).regmap, WM8804_PWRDN, 0x1, 0x1, &mut change);
        if !change {
            pm_runtime_get_sync((*wm8804).dev);
        }

        /* set PLLN and PRESCALE */
        snd_soc_component_update_bits(component, WM8804_PLL4, 0xf | 0x10, divs.n | (divs.prescale << 4));
        /* set mclkdiv and freqmode */
        snd_soc_component_update_bits(component, WM8804_PLL5, 0x3 | 0x8, divs.freqmode | (divs.mclkdiv << 3));
        /* set PLLK */
        snd_soc_component_write(component, WM8804_PLL1, divs.k & 0xff);
        snd_soc_component_write(component, WM8804_PLL2, (divs.k >> 8) & 0xff);
        snd_soc_component_write(component, WM8804_PLL3, divs.k >> 16);

        /* power up the PLL */
        snd_soc_component_update_bits(component, WM8804_PWRDN, 0x1, 0);
    }

    0
}

unsafe extern "C" fn wm8804_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;

    match clk_id {
        x if x == WM8804_TX_CLKSRC_MCLK => {
            if (freq >= 10000000 && freq <= 14400000) || (freq >= 16280000 && freq <= 27000000) {
                snd_soc_component_update_bits(component, WM8804_PLL6, 0x80, 0x80);
            } else {
                dev_err((*dai).dev, b"OSCCLOCK is not within the recommended range: %uHz\n\0".as_ptr() as *const c_char, freq);
                return -EINVAL;
            }
        }
        x if x == WM8804_TX_CLKSRC_PLL => {
            snd_soc_component_update_bits(component, WM8804_PLL6, 0x80, 0);
        }
        x if x == WM8804_CLKOUT_SRC_CLK1 => {
            snd_soc_component_update_bits(component, WM8804_PLL6, 0x8, 0);
        }
        x if x == WM8804_CLKOUT_SRC_OSCCLK => {
            snd_soc_component_update_bits(component, WM8804_PLL6, 0x8, 0x8);
        }
        _ => {
            dev_err((*dai).dev, b"Unknown clock source: %d\n\0".as_ptr() as *const c_char, clk_id);
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn wm8804_set_clkdiv(dai: *mut snd_soc_dai, div_id: c_int, div: c_int) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let wm8804: *mut wm8804_priv;

    match div_id {
        x if x == WM8804_CLKOUT_DIV => {
            snd_soc_component_update_bits(component, WM8804_PLL5, 0x30, ((div & 0x3) << 4) as c_uint);
        }
        x if x == WM8804_MCLK_DIV => {
            wm8804 = snd_soc_component_get_drvdata(component) as *mut wm8804_priv;
            (*wm8804).mclk_div = div;
        }
        _ => {
            dev_err((*dai).dev, b"Unknown clock divider: %d\n\0".as_ptr() as *const c_char, div_id);
            return -EINVAL;
        }
    }
    0
}

static wm8804_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(wm8804_hw_params),
    set_fmt: Some(wm8804_set_fmt),
    set_sysclk: Some(wm8804_set_sysclk),
    set_clkdiv: Some(wm8804_set_clkdiv),
    set_pll: Some(wm8804_set_pll),
};

const WM8804_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE;

const WM8804_RATES: c_uint = SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000
    | SNDRV_PCM_RATE_64000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_176400
    | SNDRV_PCM_RATE_192000;

static mut wm8804_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"wm8804-spdif\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 2,
        rates: WM8804_RATES,
        formats: WM8804_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 2,
        rates: WM8804_RATES,
        formats: WM8804_FORMATS,
    },
    ops: &wm8804_dai_ops,
    symmetric_rate: 1,
};

static soc_component_dev_wm8804: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: wm8804_dapm_widgets.as_ptr(),
    num_dapm_widgets: wm8804_dapm_widgets.len() as c_uint,
    dapm_routes: wm8804_dapm_routes.as_ptr(),
    num_dapm_routes: wm8804_dapm_routes.len() as c_uint,
    use_pmdown_time: 1,
    endianness: 1,
};

#[no_mangle]
pub static wm8804_regmap_config: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: unsafe { WM8804_MAX_REGISTER },
    volatile_reg: Some(wm8804_volatile),
    cache_type: REGCACHE_MAPLE,
    reg_defaults: wm8804_reg_defaults.as_ptr(),
    num_reg_defaults: wm8804_reg_defaults.len() as c_uint,
};
// EXPORT_SYMBOL_GPL(wm8804_regmap_config);

#[no_mangle]
pub unsafe extern "C" fn wm8804_probe(dev: *mut device, regmap: *mut regmap) -> c_int {
    let wm8804: *mut wm8804_priv;
    let mut id1: c_uint = 0;
    let mut id2: c_uint = 0;
    let mut i: usize;
    let mut ret: c_int;

    wm8804 = devm_kzalloc(dev, core::mem::size_of::<wm8804_priv>(), GFP_KERNEL) as *mut wm8804_priv;
    if wm8804.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(dev, wm8804 as *mut c_void);

    (*wm8804).dev = dev;
    (*wm8804).regmap = regmap;

    (*wm8804).reset = devm_gpiod_get_optional(dev, b"wlf,reset\0".as_ptr() as *const c_char, GPIOD_OUT_LOW);
    if IS_ERR((*wm8804).reset as *const c_void) {
        ret = PTR_ERR((*wm8804).reset as *const c_void);
        dev_err(dev, b"Failed to get reset line: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    i = 0;
    while i < WM8804_NUM_SUPPLIES {
        (*wm8804).supplies[i].supply = wm8804_supply_names[i];
        i += 1;
    }

    ret = devm_regulator_bulk_get(dev, WM8804_NUM_SUPPLIES as c_int, (*wm8804).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, b"Failed to request supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    (*wm8804).disable_nb[0].notifier_call = Some(wm8804_regulator_event_0);
    (*wm8804).disable_nb[1].notifier_call = Some(wm8804_regulator_event_1);

    /* This should really be moved into the regulator core */
    i = 0;
    while i < WM8804_NUM_SUPPLIES {
        let regulator = (*wm8804).supplies[i].consumer;

        ret = devm_regulator_register_notifier(regulator, &mut (*wm8804).disable_nb[i]);
        if ret != 0 {
            dev_err(dev, b"Failed to register regulator notifier: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
        i += 1;
    }

    ret = regulator_bulk_enable(WM8804_NUM_SUPPLIES as c_int, (*wm8804).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, b"Failed to enable supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    gpiod_set_value_cansleep((*wm8804).reset, 1);

    ret = regmap_read(regmap, WM8804_RST_DEVID1, &mut id1);
    if ret < 0 {
        dev_err(dev, b"Failed to read device ID: %d\n\0".as_ptr() as *const c_char, ret);
        regulator_bulk_disable(WM8804_NUM_SUPPLIES as c_int, (*wm8804).supplies.as_mut_ptr());
        return ret;
    }

    ret = regmap_read(regmap, WM8804_DEVID2, &mut id2);
    if ret < 0 {
        dev_err(dev, b"Failed to read device ID: %d\n\0".as_ptr() as *const c_char, ret);
        regulator_bulk_disable(WM8804_NUM_SUPPLIES as c_int, (*wm8804).supplies.as_mut_ptr());
        return ret;
    }

    id2 = (id2 << 8) | id1;

    if id2 != 0x8805 {
        dev_err(dev, b"Invalid device ID: %#x\n\0".as_ptr() as *const c_char, id2);
        ret = -EINVAL;
        regulator_bulk_disable(WM8804_NUM_SUPPLIES as c_int, (*wm8804).supplies.as_mut_ptr());
        return ret;
    }

    ret = regmap_read(regmap, WM8804_DEVREV, &mut id1);
    if ret < 0 {
        dev_err(dev, b"Failed to read device revision: %d\n\0".as_ptr() as *const c_char, ret);
        regulator_bulk_disable(WM8804_NUM_SUPPLIES as c_int, (*wm8804).supplies.as_mut_ptr());
        return ret;
    }
    dev_info(dev, b"revision %c\n\0".as_ptr() as *const c_char, id1 + b'A' as c_uint);

    if (*wm8804).reset.is_null() {
        ret = wm8804_soft_reset(wm8804);
        if ret < 0 {
            dev_err(dev, b"Failed to issue reset: %d\n\0".as_ptr() as *const c_char, ret);
            regulator_bulk_disable(WM8804_NUM_SUPPLIES as c_int, (*wm8804).supplies.as_mut_ptr());
            return ret;
        }
    }

    ret = devm_snd_soc_register_component(dev, &soc_component_dev_wm8804, &mut wm8804_dai, 1);
    if ret < 0 {
        dev_err(dev, b"Failed to register CODEC: %d\n\0".as_ptr() as *const c_char, ret);
        regulator_bulk_disable(WM8804_NUM_SUPPLIES as c_int, (*wm8804).supplies.as_mut_ptr());
        return ret;
    }

    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    pm_runtime_idle(dev);

    0
}
// EXPORT_SYMBOL_GPL(wm8804_probe);

#[no_mangle]
pub unsafe extern "C" fn wm8804_remove(dev: *mut device) {
    pm_runtime_disable(dev);
}
// EXPORT_SYMBOL_GPL(wm8804_remove);

unsafe extern "C" fn wm8804_runtime_resume(dev: *mut device) -> c_int {
    let wm8804 = dev_get_drvdata(dev) as *mut wm8804_priv;
    let ret: c_int;

    ret = regulator_bulk_enable(WM8804_NUM_SUPPLIES as c_int, (*wm8804).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err((*wm8804).dev, b"Failed to enable supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    regcache_sync((*wm8804).regmap);

    /* Power up OSCCLK */
    regmap_update_bits((*wm8804).regmap, WM8804_PWRDN, 0x8, 0x0);

    0
}

unsafe extern "C" fn wm8804_runtime_suspend(dev: *mut device) -> c_int {
    let wm8804 = dev_get_drvdata(dev) as *mut wm8804_priv;

    /* Power down OSCCLK */
    regmap_update_bits((*wm8804).regmap, WM8804_PWRDN, 0x8, 0x8);

    regulator_bulk_disable(WM8804_NUM_SUPPLIES as c_int, (*wm8804).supplies.as_mut_ptr());

    0
}

// EXPORT_GPL_DEV_PM_OPS(wm8804_pm) = {
//     RUNTIME_PM_OPS(wm8804_runtime_suspend, wm8804_runtime_resume, NULL)
// };

// MODULE_DESCRIPTION("ASoC WM8804 driver");
// MODULE_AUTHOR("Dimitris Papastamos <dp@opensource.wolfsonmicro.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
