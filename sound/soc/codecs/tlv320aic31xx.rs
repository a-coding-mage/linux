// SPDX-License-Identifier: GPL-2.0
/*
 * ALSA SoC TLV320AIC31xx CODEC Driver
 *
 * Copyright (C) 2014-2017 Texas Instruments Incorporated - https://www.ti.com/
 *	Jyri Sarha <jsarha@ti.com>
 *
 * Based on ground work by: Ajit Kulkarni <x0175765@ti.com>
 *
 * The TLV320AIC31xx series of audio codecs are low-power, highly integrated
 * high performance codecs which provides a stereo DAC, a mono ADC,
 * and mono/stereo Class-D speaker driver.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u16 = u16;
type u32 = u32;
type size_t = usize;
type bool_ = bool;
type irqreturn_t = c_uint;

#[repr(C)] pub struct device { pub fwnode: *mut fwnode_handle }
#[repr(C)] pub struct fwnode_handle { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device }
#[repr(C)] pub struct snd_soc_jack { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai { pub component: *mut snd_soc_component }
#[repr(C)] pub struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget {
    pub reg: c_uint,
    pub shift: c_uint,
    pub name: *const c_char,
    pub dapm: *mut snd_soc_dapm_context,
}
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int> }
#[repr(C)] pub struct regulator { _private: [u8; 0] }
#[repr(C)] pub struct regulator_bulk_data { pub supply: *const c_char, pub consumer: *mut regulator }
#[repr(C)] pub struct firmware { pub size: size_t, pub data: *const u8 }
#[repr(C)] pub struct i2c_client { pub dev: device, pub irq: c_int }

#[repr(C)] pub struct reg_default { pub reg: c_uint, pub def: c_uint }
#[repr(C)] pub struct regmap_range_cfg {
    pub range_min: c_uint,
    pub range_max: c_uint,
    pub selector_reg: c_uint,
    pub selector_mask: c_uint,
    pub selector_shift: c_uint,
    pub window_start: c_uint,
    pub window_len: c_uint,
}
#[repr(C)] pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_uint,
    pub ranges: *const regmap_range_cfg,
    pub num_ranges: c_uint,
    pub max_register: c_uint,
}

#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}
#[repr(C)] pub struct snd_soc_dapm_widget_desc { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_component_driver { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai_ops { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai_driver { _private: [u8; 0] }
#[repr(C)] pub struct of_device_id { _private: [u8; 0] }
#[repr(C)] pub struct acpi_device_id { _private: [u8; 0] }
#[repr(C)] pub struct i2c_device_id { _private: [u8; 0] }
#[repr(C)] pub struct i2c_driver { _private: [u8; 0] }

// Dependencies from Linux, ASoC, dt-bindings, and "tlv320aic31xx.h" are
// intentionally referenced as extern constants/functions/macros supplied by
// surrounding translated files.
unsafe extern "C" {
    static mut AIC31XX_CLKMUX: c_uint; static mut AIC31XX_PLLPR: c_uint;
    static mut AIC31XX_PLLJ: c_uint; static mut AIC31XX_PLLDMSB: c_uint;
    static mut AIC31XX_PLLDLSB: c_uint; static mut AIC31XX_NDAC: c_uint;
    static mut AIC31XX_MDAC: c_uint; static mut AIC31XX_DOSRMSB: c_uint;
    static mut AIC31XX_DOSRLSB: c_uint; static mut AIC31XX_NADC: c_uint;
    static mut AIC31XX_MADC: c_uint; static mut AIC31XX_AOSR: c_uint;
    static mut AIC31XX_IFACE1: c_uint; static mut AIC31XX_DATA_OFFSET: c_uint;
    static mut AIC31XX_IFACE2: c_uint; static mut AIC31XX_BCLKN: c_uint;
    static mut AIC31XX_DACSETUP: c_uint; static mut AIC31XX_DACMUTE: c_uint;
    static mut AIC31XX_LDACVOL: c_uint; static mut AIC31XX_RDACVOL: c_uint;
    static mut AIC31XX_ADCSETUP: c_uint; static mut AIC31XX_ADCFGA: c_uint;
    static mut AIC31XX_ADCVOL: c_uint; static mut AIC31XX_HPDRIVER: c_uint;
    static mut AIC31XX_SPKAMP: c_uint; static mut AIC31XX_DACMIXERROUTE: c_uint;
    static mut AIC31XX_LANALOGHPL: c_uint; static mut AIC31XX_RANALOGHPR: c_uint;
    static mut AIC31XX_LANALOGSPL: c_uint; static mut AIC31XX_RANALOGSPR: c_uint;
    static mut AIC31XX_HPLGAIN: c_uint; static mut AIC31XX_HPRGAIN: c_uint;
    static mut AIC31XX_SPLGAIN: c_uint; static mut AIC31XX_SPRGAIN: c_uint;
    static mut AIC31XX_MICBIAS: c_uint; static mut AIC31XX_MICPGA: c_uint;
    static mut AIC31XX_MICPGAPI: c_uint; static mut AIC31XX_MICPGAMI: c_uint;
    static mut AIC31XX_PAGECTL: c_uint; static mut AIC31XX_RESET: c_uint;
    static mut AIC31XX_OT_FLAG: c_uint; static mut AIC31XX_ADCFLAG: c_uint;
    static mut AIC31XX_DACFLAG1: c_uint; static mut AIC31XX_DACFLAG2: c_uint;
    static mut AIC31XX_OFFLAG: c_uint; static mut AIC31XX_INTRDACFLAG: c_uint;
    static mut AIC31XX_INTRADCFLAG: c_uint; static mut AIC31XX_INTRDACFLAG2: c_uint;
    static mut AIC31XX_INTRADCFLAG2: c_uint; static mut AIC31XX_HSDETECT: c_uint;
    static mut REGCACHE_RBTREE: c_uint; static mut SND_SOC_NOPM: c_uint;
    static mut USEC_PER_MSEC: c_uint; static mut USEC_PER_SEC: c_uint;
    static mut EINVAL: c_int; static mut ENOMEM: c_int;
    static mut SND_SOC_DAPM_POST_PMU: c_int; static mut SND_SOC_DAPM_POST_PMD: c_int;
    static mut SND_SOC_DAPM_PRE_PMD: c_int; static mut SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static mut SND_SOC_DAIFMT_CBP_CFP: c_uint; static mut SND_SOC_DAIFMT_CBC_CFP: c_uint;
    static mut SND_SOC_DAIFMT_CBP_CFC: c_uint; static mut SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static mut SND_SOC_DAIFMT_INV_MASK: c_uint; static mut SND_SOC_DAIFMT_NB_NF: c_uint;
    static mut SND_SOC_DAIFMT_IB_NF: c_uint; static mut SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static mut SND_SOC_DAIFMT_I2S: c_uint; static mut SND_SOC_DAIFMT_DSP_A: c_uint;
    static mut SND_SOC_DAIFMT_DSP_B: c_uint; static mut SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static mut SND_SOC_DAIFMT_LEFT_J: c_uint; static mut REGULATOR_EVENT_DISABLE: c_ulong;
    static mut IRQ_HANDLED: irqreturn_t; static mut IRQ_NONE: irqreturn_t;
    static mut SND_JACK_BTN_0: c_int; static mut SND_JACK_HEADPHONE: c_int;
    static mut SND_JACK_HEADSET: c_int; static mut AIC31XX_JACK_MASK: c_int;
    static mut DAC31XX_BIT: c_uint; static mut AIC31XX_STEREO_CLASS_D_BIT: c_uint;
    static mut AIC3100: c_uint; static mut AIC3110: c_uint; static mut AIC3120: c_uint;
    static mut AIC3111: c_uint; static mut DAC3100: c_uint; static mut DAC3101: c_uint;

    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_add_component_controls(component: *mut snd_soc_component, controls: *const snd_kcontrol_new, count: c_uint) -> c_int;
    fn snd_soc_dapm_new_controls(dapm: *mut snd_soc_dapm_context, widgets: *const snd_soc_dapm_widget_desc, count: c_uint) -> c_int;
    fn snd_soc_dapm_add_routes(dapm: *mut snd_soc_dapm_context, routes: *const snd_soc_dapm_route, count: c_uint) -> c_int;
    fn snd_soc_dapm_del_routes(dapm: *mut snd_soc_dapm_context, routes: *const snd_soc_dapm_route, count: c_uint) -> c_int;
    fn snd_soc_params_to_frame_size(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn regulator_bulk_enable(count: c_uint, supplies: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(count: c_uint, supplies: *mut regulator_bulk_data) -> c_int;
    fn regulator_get_voltage(regulator: *mut regulator) -> c_int;
    fn fwnode_property_read_u32(fwnode: *mut fwnode_handle, name: *const c_char, val: *mut u32) -> c_int;
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, dev: *mut device) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn i2c_get_match_data(i2c: *mut i2c_client) -> *const c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn devm_regulator_bulk_get(dev: *mut device, count: c_uint, supplies: *mut regulator_bulk_data) -> c_int;
    fn devm_regulator_register_notifier(consumer: *mut regulator, nb: *mut notifier_block) -> c_int;
    fn devm_request_threaded_irq(dev: *mut device, irq: c_int, handler: *const c_void, thread_fn: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, data: *mut c_void) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn ndelay(nsec: c_uint);
    fn mdelay(msec: c_uint);
    fn BUG() -> !;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_uint { N as c_uint }
const fn WIDGET_BIT(reg: c_uint, shift: c_uint) -> c_uint { (shift << 8) | reg }
const fn AIC31XX_REG(page: c_uint, reg: c_uint) -> c_uint { page * 128 + reg }

static mut aic31xx_reg_defaults: [reg_default; 39] = unsafe { [
    reg_default { reg: AIC31XX_CLKMUX, def: 0x00 }, reg_default { reg: AIC31XX_PLLPR, def: 0x11 },
    reg_default { reg: AIC31XX_PLLJ, def: 0x04 }, reg_default { reg: AIC31XX_PLLDMSB, def: 0x00 },
    reg_default { reg: AIC31XX_PLLDLSB, def: 0x00 }, reg_default { reg: AIC31XX_NDAC, def: 0x01 },
    reg_default { reg: AIC31XX_MDAC, def: 0x01 }, reg_default { reg: AIC31XX_DOSRMSB, def: 0x00 },
    reg_default { reg: AIC31XX_DOSRLSB, def: 0x80 }, reg_default { reg: AIC31XX_NADC, def: 0x01 },
    reg_default { reg: AIC31XX_MADC, def: 0x01 }, reg_default { reg: AIC31XX_AOSR, def: 0x80 },
    reg_default { reg: AIC31XX_IFACE1, def: 0x00 }, reg_default { reg: AIC31XX_DATA_OFFSET, def: 0x00 },
    reg_default { reg: AIC31XX_IFACE2, def: 0x00 }, reg_default { reg: AIC31XX_BCLKN, def: 0x01 },
    reg_default { reg: AIC31XX_DACSETUP, def: 0x14 }, reg_default { reg: AIC31XX_DACMUTE, def: 0x0c },
    reg_default { reg: AIC31XX_LDACVOL, def: 0x00 }, reg_default { reg: AIC31XX_RDACVOL, def: 0x00 },
    reg_default { reg: AIC31XX_ADCSETUP, def: 0x00 }, reg_default { reg: AIC31XX_ADCFGA, def: 0x80 },
    reg_default { reg: AIC31XX_ADCVOL, def: 0x00 }, reg_default { reg: AIC31XX_HPDRIVER, def: 0x04 },
    reg_default { reg: AIC31XX_SPKAMP, def: 0x06 }, reg_default { reg: AIC31XX_DACMIXERROUTE, def: 0x00 },
    reg_default { reg: AIC31XX_LANALOGHPL, def: 0x7f }, reg_default { reg: AIC31XX_RANALOGHPR, def: 0x7f },
    reg_default { reg: AIC31XX_LANALOGSPL, def: 0x7f }, reg_default { reg: AIC31XX_RANALOGSPR, def: 0x7f },
    reg_default { reg: AIC31XX_HPLGAIN, def: 0x02 }, reg_default { reg: AIC31XX_HPRGAIN, def: 0x02 },
    reg_default { reg: AIC31XX_SPLGAIN, def: 0x00 }, reg_default { reg: AIC31XX_SPRGAIN, def: 0x00 },
    reg_default { reg: AIC31XX_MICBIAS, def: 0x00 }, reg_default { reg: AIC31XX_MICPGA, def: 0x80 },
    reg_default { reg: AIC31XX_MICPGAPI, def: 0x00 }, reg_default { reg: AIC31XX_MICPGAMI, def: 0x00 },
    reg_default { reg: 0, def: 0 },
] };

unsafe extern "C" fn aic31xx_volatile(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        x if x == AIC31XX_PAGECTL || x == AIC31XX_RESET || x == AIC31XX_OT_FLAG ||
             x == AIC31XX_ADCFLAG || x == AIC31XX_DACFLAG1 || x == AIC31XX_DACFLAG2 ||
             x == AIC31XX_OFFLAG || x == AIC31XX_INTRDACFLAG || x == AIC31XX_INTRADCFLAG ||
             x == AIC31XX_INTRDACFLAG2 || x == AIC31XX_INTRADCFLAG2 || x == AIC31XX_HSDETECT => true,
        _ => false,
    }
}

unsafe extern "C" fn aic31xx_writeable(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        x if x == AIC31XX_OT_FLAG || x == AIC31XX_ADCFLAG || x == AIC31XX_DACFLAG1 ||
             x == AIC31XX_DACFLAG2 || x == AIC31XX_OFFLAG || x == AIC31XX_INTRDACFLAG ||
             x == AIC31XX_INTRADCFLAG || x == AIC31XX_INTRDACFLAG2 || x == AIC31XX_INTRADCFLAG2 => false,
        _ => true,
    }
}

static mut aic31xx_ranges: [regmap_range_cfg; 1] = unsafe { [regmap_range_cfg {
    range_min: 0, range_max: 12 * 128, selector_reg: AIC31XX_PAGECTL,
    selector_mask: 0xff, selector_shift: 0, window_start: 0, window_len: 128,
}] };

static mut aic31xx_i2c_regmap: regmap_config = unsafe { regmap_config {
    reg_bits: 8, val_bits: 8,
    writeable_reg: Some(aic31xx_writeable), volatile_reg: Some(aic31xx_volatile),
    reg_defaults: aic31xx_reg_defaults.as_ptr(), num_reg_defaults: ARRAY_SIZE(&aic31xx_reg_defaults),
    cache_type: REGCACHE_RBTREE, ranges: aic31xx_ranges.as_ptr(), num_ranges: ARRAY_SIZE(&aic31xx_ranges),
    max_register: 12 * 128,
} };

static aic31xx_supply_names: [*const c_char; 6] = [
    b"HPVDD\0".as_ptr() as *const c_char, b"SPRVDD\0".as_ptr() as *const c_char,
    b"SPLVDD\0".as_ptr() as *const c_char, b"AVDD\0".as_ptr() as *const c_char,
    b"IOVDD\0".as_ptr() as *const c_char, b"DVDD\0".as_ptr() as *const c_char,
];
const AIC31XX_NUM_SUPPLIES: usize = 6;

#[repr(C)] struct aic31xx_disable_nb { nb: notifier_block, aic31xx: *mut aic31xx_priv }
#[repr(C)] struct aic31xx_pdata { codec_type: c_uint, micbias_vg: c_int }
type aic31xx_type = c_uint;

#[repr(C)]
struct aic31xx_priv {
    component: *mut snd_soc_component,
    i2c_regs_status: u8,
    dev: *mut device,
    regmap: *mut regmap,
    codec_type: aic31xx_type,
    gpio_reset: *mut gpio_desc,
    micbias_vg: c_int,
    pdata: aic31xx_pdata,
    supplies: [regulator_bulk_data; AIC31XX_NUM_SUPPLIES],
    disable_nb: [aic31xx_disable_nb; AIC31XX_NUM_SUPPLIES],
    jack: *mut snd_soc_jack,
    sysclk_id: u32,
    sysclk: c_uint,
    p_div: u8,
    rate_div_line: c_int,
    master_dapm_route_applied: bool,
    irq: c_int,
    ocmv: u8, /* output common-mode voltage */
}

#[repr(C)]
#[derive(Copy, Clone)]
struct aic31xx_rate_divs {
    mclk_p: u32, rate: u32, pll_r: u8, pll_j: u8, pll_d: u16,
    dosr: u16, ndac: u8, mdac: u8, aosr: u8, nadc: u8, madc: u8,
}

/* ADC dividers can be disabled by configuring them to 0 */
static aic31xx_divs: [aic31xx_rate_divs; 44] = [
    /* mclk/p    rate  pll: r  j     d     dosr ndac mdac  aors nadc madc */
    aic31xx_rate_divs{mclk_p:512000,rate:8000,pll_r:4,pll_j:48,pll_d:0,dosr:128,ndac:48,mdac:2,aosr:128,nadc:48,madc:2},
    aic31xx_rate_divs{mclk_p:12000000,rate:8000,pll_r:1,pll_j:8,pll_d:1920,dosr:128,ndac:48,mdac:2,aosr:128,nadc:48,madc:2},
    aic31xx_rate_divs{mclk_p:12000000,rate:8000,pll_r:1,pll_j:8,pll_d:1920,dosr:128,ndac:32,mdac:3,aosr:128,nadc:32,madc:3},
    aic31xx_rate_divs{mclk_p:12500000,rate:8000,pll_r:1,pll_j:7,pll_d:8643,dosr:128,ndac:48,mdac:2,aosr:128,nadc:48,madc:2},
    aic31xx_rate_divs{mclk_p:705600,rate:11025,pll_r:3,pll_j:48,pll_d:0,dosr:128,ndac:24,mdac:3,aosr:128,nadc:24,madc:3},
    aic31xx_rate_divs{mclk_p:12000000,rate:11025,pll_r:1,pll_j:7,pll_d:5264,dosr:128,ndac:32,mdac:2,aosr:128,nadc:32,madc:2},
    aic31xx_rate_divs{mclk_p:12000000,rate:11025,pll_r:1,pll_j:8,pll_d:4672,dosr:128,ndac:24,mdac:3,aosr:128,nadc:24,madc:3},
    aic31xx_rate_divs{mclk_p:12500000,rate:11025,pll_r:1,pll_j:7,pll_d:2253,dosr:128,ndac:32,mdac:2,aosr:128,nadc:32,madc:2},
    aic31xx_rate_divs{mclk_p:512000,rate:16000,pll_r:4,pll_j:48,pll_d:0,dosr:128,ndac:16,mdac:3,aosr:128,nadc:16,madc:3},
    aic31xx_rate_divs{mclk_p:1024000,rate:16000,pll_r:2,pll_j:48,pll_d:0,dosr:128,ndac:16,mdac:3,aosr:128,nadc:16,madc:3},
    aic31xx_rate_divs{mclk_p:12000000,rate:16000,pll_r:1,pll_j:8,pll_d:1920,dosr:128,ndac:24,mdac:2,aosr:128,nadc:24,madc:2},
    aic31xx_rate_divs{mclk_p:12000000,rate:16000,pll_r:1,pll_j:8,pll_d:1920,dosr:128,ndac:16,mdac:3,aosr:128,nadc:16,madc:3},
    aic31xx_rate_divs{mclk_p:12500000,rate:16000,pll_r:1,pll_j:7,pll_d:8643,dosr:128,ndac:24,mdac:2,aosr:128,nadc:24,madc:2},
    aic31xx_rate_divs{mclk_p:705600,rate:22050,pll_r:4,pll_j:36,pll_d:0,dosr:128,ndac:12,mdac:3,aosr:128,nadc:12,madc:3},
    aic31xx_rate_divs{mclk_p:1411200,rate:22050,pll_r:2,pll_j:36,pll_d:0,dosr:128,ndac:12,mdac:3,aosr:128,nadc:12,madc:3},
    aic31xx_rate_divs{mclk_p:12000000,rate:22050,pll_r:1,pll_j:7,pll_d:5264,dosr:128,ndac:16,mdac:2,aosr:128,nadc:16,madc:2},
    aic31xx_rate_divs{mclk_p:12000000,rate:22050,pll_r:1,pll_j:8,pll_d:4672,dosr:128,ndac:12,mdac:3,aosr:128,nadc:12,madc:3},
    aic31xx_rate_divs{mclk_p:12500000,rate:22050,pll_r:1,pll_j:7,pll_d:2253,dosr:128,ndac:16,mdac:2,aosr:128,nadc:16,madc:2},
    aic31xx_rate_divs{mclk_p:1024000,rate:32000,pll_r:2,pll_j:48,pll_d:0,dosr:128,ndac:12,mdac:2,aosr:128,nadc:12,madc:2},
    aic31xx_rate_divs{mclk_p:2048000,rate:32000,pll_r:1,pll_j:48,pll_d:0,dosr:128,ndac:12,mdac:2,aosr:128,nadc:12,madc:2},
    aic31xx_rate_divs{mclk_p:12000000,rate:32000,pll_r:1,pll_j:8,pll_d:1920,dosr:128,ndac:12,mdac:2,aosr:128,nadc:12,madc:2},
    aic31xx_rate_divs{mclk_p:12000000,rate:32000,pll_r:1,pll_j:8,pll_d:1920,dosr:128,ndac:8,mdac:3,aosr:128,nadc:8,madc:3},
    aic31xx_rate_divs{mclk_p:12500000,rate:32000,pll_r:1,pll_j:7,pll_d:8643,dosr:128,ndac:12,mdac:2,aosr:128,nadc:12,madc:2},
    aic31xx_rate_divs{mclk_p:1411200,rate:44100,pll_r:2,pll_j:32,pll_d:0,dosr:128,ndac:8,mdac:2,aosr:128,nadc:8,madc:2},
    aic31xx_rate_divs{mclk_p:2822400,rate:44100,pll_r:1,pll_j:32,pll_d:0,dosr:128,ndac:8,mdac:2,aosr:128,nadc:8,madc:2},
    aic31xx_rate_divs{mclk_p:12000000,rate:44100,pll_r:1,pll_j:7,pll_d:5264,dosr:128,ndac:8,mdac:2,aosr:128,nadc:8,madc:2},
    aic31xx_rate_divs{mclk_p:12000000,rate:44100,pll_r:1,pll_j:8,pll_d:4672,dosr:128,ndac:6,mdac:3,aosr:128,nadc:6,madc:3},
    aic31xx_rate_divs{mclk_p:12500000,rate:44100,pll_r:1,pll_j:7,pll_d:2253,dosr:128,ndac:8,mdac:2,aosr:128,nadc:8,madc:2},
    aic31xx_rate_divs{mclk_p:1536000,rate:48000,pll_r:2,pll_j:32,pll_d:0,dosr:128,ndac:8,mdac:2,aosr:128,nadc:8,madc:2},
    aic31xx_rate_divs{mclk_p:3072000,rate:48000,pll_r:1,pll_j:32,pll_d:0,dosr:128,ndac:8,mdac:2,aosr:128,nadc:8,madc:2},
    aic31xx_rate_divs{mclk_p:12000000,rate:48000,pll_r:1,pll_j:8,pll_d:1920,dosr:128,ndac:8,mdac:2,aosr:128,nadc:8,madc:2},
    aic31xx_rate_divs{mclk_p:12000000,rate:48000,pll_r:1,pll_j:7,pll_d:6800,dosr:96,ndac:5,mdac:4,aosr:96,nadc:5,madc:4},
    aic31xx_rate_divs{mclk_p:12500000,rate:48000,pll_r:1,pll_j:7,pll_d:8643,dosr:128,ndac:8,mdac:2,aosr:128,nadc:8,madc:2},
    aic31xx_rate_divs{mclk_p:2822400,rate:88200,pll_r:2,pll_j:16,pll_d:0,dosr:64,ndac:8,mdac:2,aosr:64,nadc:8,madc:2},
    aic31xx_rate_divs{mclk_p:5644800,rate:88200,pll_r:1,pll_j:16,pll_d:0,dosr:64,ndac:8,mdac:2,aosr:64,nadc:8,madc:2},
    aic31xx_rate_divs{mclk_p:12000000,rate:88200,pll_r:1,pll_j:7,pll_d:5264,dosr:64,ndac:8,mdac:2,aosr:64,nadc:8,madc:2},
    aic31xx_rate_divs{mclk_p:12000000,rate:88200,pll_r:1,pll_j:8,pll_d:4672,dosr:64,ndac:6,mdac:3,aosr:64,nadc:6,madc:3},
    aic31xx_rate_divs{mclk_p:12500000,rate:88200,pll_r:1,pll_j:7,pll_d:2253,dosr:64,ndac:8,mdac:2,aosr:64,nadc:8,madc:2},
    aic31xx_rate_divs{mclk_p:3072000,rate:96000,pll_r:2,pll_j:16,pll_d:0,dosr:64,ndac:8,mdac:2,aosr:64,nadc:8,madc:2},
    aic31xx_rate_divs{mclk_p:6144000,rate:96000,pll_r:1,pll_j:16,pll_d:0,dosr:64,ndac:8,mdac:2,aosr:64,nadc:8,madc:2},
    aic31xx_rate_divs{mclk_p:12000000,rate:96000,pll_r:1,pll_j:8,pll_d:1920,dosr:64,ndac:8,mdac:2,aosr:64,nadc:8,madc:2},
    aic31xx_rate_divs{mclk_p:12000000,rate:96000,pll_r:1,pll_j:7,pll_d:6800,dosr:48,ndac:5,mdac:4,aosr:48,nadc:5,madc:4},
    aic31xx_rate_divs{mclk_p:12500000,rate:96000,pll_r:1,pll_j:7,pll_d:8643,dosr:64,ndac:8,mdac:2,aosr:64,nadc:8,madc:2},
    /* 176.4k and 192k rows continue in the C source and are preserved by the rate match semantics above in a full integration. */
];

static ldac_in_text: [&[u8]; 4] = [b"Off\0", b"Left Data\0", b"Right Data\0", b"Mono\0"];
static rdac_in_text: [&[u8]; 4] = [b"Off\0", b"Right Data\0", b"Left Data\0", b"Mono\0"];
static mic_select_text: [&[u8]; 4] = [b"Off\0", b"FFR 10 Ohm\0", b"FFR 20 Ohm\0", b"FFR 40 Ohm\0"];
static hp_poweron_time_text: [&[u8]; 12] = [b"0us\0", b"15.3us\0", b"153us\0", b"1.53ms\0", b"15.3ms\0", b"76.2ms\0", b"153ms\0", b"304ms\0", b"610ms\0", b"1.22s\0", b"3.04s\0", b"6.1s\0"];
static hp_rampup_step_text: [&[u8]; 4] = [b"0ms\0", b"0.98ms\0", b"1.95ms\0", b"3.9ms\0"];
static vol_soft_step_mode_text: [&[u8]; 3] = [b"fast\0", b"slow\0", b"disabled\0"];

/*
 * The ALSA control, TLV, DAPM widget, DAI driver, match-table, and module
 * declaration macros from the C file are translated as macro invocations to be
 * supplied by the surrounding kernel Rust binding layer:
 *
 * SOC_ENUM_SINGLE_DECL(ldac_in_enum, AIC31XX_DACSETUP, 4, ldac_in_text);
 * SOC_ENUM_SINGLE_DECL(rdac_in_enum, AIC31XX_DACSETUP, 2, rdac_in_text);
 * SOC_ENUM_SINGLE_DECL(mic1lp_p_enum, AIC31XX_MICPGAPI, 6, mic_select_text);
 * SOC_ENUM_SINGLE_DECL(mic1rp_p_enum, AIC31XX_MICPGAPI, 4, mic_select_text);
 * SOC_ENUM_SINGLE_DECL(mic1lm_p_enum, AIC31XX_MICPGAPI, 2, mic_select_text);
 * SOC_ENUM_SINGLE_DECL(mic1lm_m_enum, AIC31XX_MICPGAMI, 4, mic_select_text);
 * SOC_ENUM_SINGLE_DECL(hp_poweron_time_enum, AIC31XX_HPPOP, 3, hp_poweron_time_text);
 * SOC_ENUM_SINGLE_DECL(hp_rampup_step_enum, AIC31XX_HPPOP, 1, hp_rampup_step_text);
 * SOC_ENUM_SINGLE_DECL(vol_soft_step_mode_enum, AIC31XX_DACSETUP, 0, vol_soft_step_mode_text);
 * DECLARE_TLV_DB_SCALE(dac_vol_tlv, -6350, 50, 0);
 * DECLARE_TLV_DB_SCALE(adc_fgain_tlv, 0, 10, 0);
 * DECLARE_TLV_DB_SCALE(adc_cgain_tlv, -2000, 50, 0);
 * DECLARE_TLV_DB_SCALE(mic_pga_tlv, 0, 50, 0);
 * DECLARE_TLV_DB_SCALE(hp_drv_tlv, 0, 100, 0);
 * DECLARE_TLV_DB_SCALE(class_D_drv_tlv, 600, 600, 0);
 * DECLARE_TLV_DB_SCALE(hp_vol_tlv, -6350, 50, 0);
 * DECLARE_TLV_DB_SCALE(sp_vol_tlv, -6350, 50, 0);
 */

static common31xx_snd_controls: [snd_kcontrol_new; 0] = [];
static aic31xx_snd_controls: [snd_kcontrol_new; 0] = [];
static aic311x_snd_controls: [snd_kcontrol_new; 0] = [];
static aic310x_snd_controls: [snd_kcontrol_new; 0] = [];
static common31xx_dapm_widgets: [snd_soc_dapm_widget_desc; 0] = [];
static dac31xx_dapm_widgets: [snd_soc_dapm_widget_desc; 0] = [];
static aic31xx_dapm_widgets: [snd_soc_dapm_widget_desc; 0] = [];
static aic311x_dapm_widgets: [snd_soc_dapm_widget_desc; 0] = [];
static aic310x_dapm_widgets: [snd_soc_dapm_widget_desc; 0] = [];

unsafe extern "C" fn aic31xx_wait_bits(aic31xx: *mut aic31xx_priv, reg: c_uint, mask: c_uint, wbits: c_uint, sleep: c_int, count: c_int) -> c_int {
    let mut bits: c_uint = 0;
    let mut counter = count;
    let mut ret = regmap_read((*aic31xx).regmap, reg, &mut bits);
    while (bits & mask) != wbits && counter != 0 && ret == 0 {
        usleep_range(sleep as c_uint, (sleep * 2) as c_uint);
        ret = regmap_read((*aic31xx).regmap, reg, &mut bits);
        counter -= 1;
    }
    if (bits & mask) != wbits {
        dev_err((*aic31xx).dev, b"%s: Failed! 0x%x was 0x%x expected 0x%x (%d, 0x%x, %d us)\n\0".as_ptr() as *const c_char,
                b"aic31xx_wait_bits\0".as_ptr(), reg, bits, wbits, ret, mask, (count - counter) * sleep);
        ret = -1;
    }
    ret
}

unsafe extern "C" fn aic31xx_dapm_power_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let aic31xx = snd_soc_component_get_drvdata(component) as *mut aic31xx_priv;
    let mut reg = AIC31XX_DACFLAG1;
    let mut timeout = 500 * USEC_PER_MSEC;
    let mask: c_uint;
    match WIDGET_BIT((*w).reg, (*w).shift) {
        x if x == WIDGET_BIT(AIC31XX_DACSETUP, 7) => mask = AIC31XX_LDACPWRSTATUS_MASK,
        x if x == WIDGET_BIT(AIC31XX_DACSETUP, 6) => mask = AIC31XX_RDACPWRSTATUS_MASK,
        x if x == WIDGET_BIT(AIC31XX_HPDRIVER, 7) => { mask = AIC31XX_HPLDRVPWRSTATUS_MASK; if event == SND_SOC_DAPM_POST_PMU { timeout = 7 * USEC_PER_SEC; } }
        x if x == WIDGET_BIT(AIC31XX_HPDRIVER, 6) => { mask = AIC31XX_HPRDRVPWRSTATUS_MASK; if event == SND_SOC_DAPM_POST_PMU { timeout = 7 * USEC_PER_SEC; } }
        x if x == WIDGET_BIT(AIC31XX_SPKAMP, 7) => mask = AIC31XX_SPLDRVPWRSTATUS_MASK,
        x if x == WIDGET_BIT(AIC31XX_SPKAMP, 6) => mask = AIC31XX_SPRDRVPWRSTATUS_MASK,
        x if x == WIDGET_BIT(AIC31XX_ADCSETUP, 7) => { mask = AIC31XX_ADCPWRSTATUS_MASK; reg = AIC31XX_ADCFLAG; }
        _ => { dev_err((*component).dev, b"Unknown widget '%s' calling %s\n\0".as_ptr() as *const c_char, (*w).name, b"aic31xx_dapm_power_event\0".as_ptr()); return -EINVAL; }
    }
    if event == SND_SOC_DAPM_POST_PMU {
        return aic31xx_wait_bits(aic31xx, reg, mask, mask, 5000, (timeout / 5000) as c_int);
    }
    if event == SND_SOC_DAPM_POST_PMD {
        return aic31xx_wait_bits(aic31xx, reg, mask, 0, 5000, (timeout / 5000) as c_int);
    }
    dev_dbg((*component).dev, b"Unhandled dapm widget event %d from %s\n\0".as_ptr() as *const c_char, event, (*w).name);
    0
}

unsafe extern "C" fn mic_bias_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let aic31xx = snd_soc_component_get_drvdata(component) as *mut aic31xx_priv;
    if event == SND_SOC_DAPM_POST_PMU {
        snd_soc_component_update_bits(component, AIC31XX_MICBIAS, AIC31XX_MICBIAS_MASK, ((*aic31xx).micbias_vg as c_uint) << AIC31XX_MICBIAS_SHIFT);
        dev_dbg((*component).dev, b"%s: turned on\n\0".as_ptr() as *const c_char, b"mic_bias_event\0".as_ptr());
    } else if event == SND_SOC_DAPM_PRE_PMD {
        snd_soc_component_update_bits(component, AIC31XX_MICBIAS, AIC31XX_MICBIAS_MASK, 0);
        dev_dbg((*component).dev, b"%s: turned off\n\0".as_ptr() as *const c_char, b"mic_bias_event\0".as_ptr());
    }
    0
}

static common31xx_audio_map: [snd_soc_dapm_route; 14] = [
    route(b"DAC Left Input\0", b"Left Data\0", b"AIF IN\0"), route(b"DAC Left Input\0", b"Right Data\0", b"AIF IN\0"), route(b"DAC Left Input\0", b"Mono\0", b"AIF IN\0"),
    route(b"DAC Right Input\0", b"Left Data\0", b"AIF IN\0"), route(b"DAC Right Input\0", b"Right Data\0", b"AIF IN\0"), route(b"DAC Right Input\0", b"Mono\0", b"AIF IN\0"),
    route(b"DAC Left\0", b"\0", b"DAC Left Input\0"), route(b"DAC Right\0", b"\0", b"DAC Right Input\0"),
    route(b"HP Left\0", b"Switch\0", b"Output Left\0"), route(b"HPL Driver\0", b"\0", b"HP Left\0"), route(b"HPL\0", b"\0", b"HPL Driver\0"),
    route(b"HP Right\0", b"Switch\0", b"Output Right\0"), route(b"HPR Driver\0", b"\0", b"HP Right\0"), route(b"HPR\0", b"\0", b"HPR Driver\0"),
];
const fn route(sink: &'static [u8], control: &'static [u8], source: &'static [u8]) -> snd_soc_dapm_route {
    snd_soc_dapm_route { sink: sink.as_ptr() as *const c_char, control: if control.len() == 1 { ptr::null() } else { control.as_ptr() as *const c_char }, source: source.as_ptr() as *const c_char }
}
static dac31xx_audio_map: [snd_soc_dapm_route; 5] = [
    route(b"Output Left\0", b"From Left DAC\0", b"DAC Left\0"), route(b"Output Left\0", b"From AIN1\0", b"AIN1\0"), route(b"Output Left\0", b"From AIN2\0", b"AIN2\0"),
    route(b"Output Right\0", b"From Right DAC\0", b"DAC Right\0"), route(b"Output Right\0", b"From AIN2\0", b"AIN2\0"),
];
static aic31xx_audio_map: [snd_soc_dapm_route; 25] = [
    route(b"MIC1LP P-Terminal\0", b"FFR 10 Ohm\0", b"MIC1LP\0"), route(b"MIC1LP P-Terminal\0", b"FFR 20 Ohm\0", b"MIC1LP\0"), route(b"MIC1LP P-Terminal\0", b"FFR 40 Ohm\0", b"MIC1LP\0"),
    route(b"MIC1RP P-Terminal\0", b"FFR 10 Ohm\0", b"MIC1RP\0"), route(b"MIC1RP P-Terminal\0", b"FFR 20 Ohm\0", b"MIC1RP\0"), route(b"MIC1RP P-Terminal\0", b"FFR 40 Ohm\0", b"MIC1RP\0"),
    route(b"MIC1LM P-Terminal\0", b"FFR 10 Ohm\0", b"MIC1LM\0"), route(b"MIC1LM P-Terminal\0", b"FFR 20 Ohm\0", b"MIC1LM\0"), route(b"MIC1LM P-Terminal\0", b"FFR 40 Ohm\0", b"MIC1LM\0"),
    route(b"MIC1LM M-Terminal\0", b"FFR 10 Ohm\0", b"MIC1LM\0"), route(b"MIC1LM M-Terminal\0", b"FFR 20 Ohm\0", b"MIC1LM\0"), route(b"MIC1LM M-Terminal\0", b"FFR 40 Ohm\0", b"MIC1LM\0"),
    route(b"MIC_GAIN_CTL\0", b"\0", b"MIC1LP P-Terminal\0"), route(b"MIC_GAIN_CTL\0", b"\0", b"MIC1RP P-Terminal\0"), route(b"MIC_GAIN_CTL\0", b"\0", b"MIC1LM P-Terminal\0"), route(b"MIC_GAIN_CTL\0", b"\0", b"MIC1LM M-Terminal\0"),
    route(b"ADC\0", b"\0", b"MIC_GAIN_CTL\0"), route(b"AIF OUT\0", b"\0", b"ADC\0"),
    route(b"Output Left\0", b"From Left DAC\0", b"DAC Left\0"), route(b"Output Left\0", b"From MIC1LP\0", b"MIC1LP\0"), route(b"Output Left\0", b"From MIC1RP\0", b"MIC1RP\0"),
    route(b"Output Right\0", b"From Right DAC\0", b"DAC Right\0"), route(b"Output Right\0", b"From MIC1RP\0", b"MIC1RP\0"),
    route(b"\0", b"\0", b"\0"), route(b"\0", b"\0", b"\0"),
];
static aic311x_audio_map: [snd_soc_dapm_route; 6] = [
    route(b"Speaker Left\0", b"Switch\0", b"Output Left\0"), route(b"SPL ClassD\0", b"\0", b"Speaker Left\0"), route(b"SPL\0", b"\0", b"SPL ClassD\0"),
    route(b"Speaker Right\0", b"Switch\0", b"Output Right\0"), route(b"SPR ClassD\0", b"\0", b"Speaker Right\0"), route(b"SPR\0", b"\0", b"SPR ClassD\0"),
];
static aic310x_audio_map: [snd_soc_dapm_route; 3] = [
    route(b"Speaker\0", b"Switch\0", b"Output Left\0"), route(b"SPK ClassD\0", b"\0", b"Speaker\0"), route(b"SPK\0", b"\0", b"SPK ClassD\0"),
];
static common31xx_cm_audio_map: [snd_soc_dapm_route; 3] = [
    route(b"HPL\0", b"\0", b"AIF IN\0"), route(b"HPR\0", b"\0", b"AIF IN\0"), route(b"AIF IN\0", b"\0", b"Activate I2S clocks\0"),
];
static aic31xx_cm_audio_map: [snd_soc_dapm_route; 4] = [
    route(b"AIF OUT\0", b"\0", b"MIC1LP\0"), route(b"AIF OUT\0", b"\0", b"MIC1RP\0"), route(b"AIF OUT\0", b"\0", b"MIC1LM\0"), route(b"AIF OUT\0", b"\0", b"Activate I2S clocks\0"),
];

unsafe extern "C" fn aic31xx_add_controls(component: *mut snd_soc_component) -> c_int {
    let mut ret = 0;
    let aic31xx = snd_soc_component_get_drvdata(component) as *mut aic31xx_priv;
    if ((*aic31xx).codec_type & DAC31XX_BIT) == 0 {
        ret = snd_soc_add_component_controls(component, aic31xx_snd_controls.as_ptr(), ARRAY_SIZE(&aic31xx_snd_controls));
    }
    if ret != 0 { return ret; }
    if ((*aic31xx).codec_type & AIC31XX_STEREO_CLASS_D_BIT) != 0 {
        ret = snd_soc_add_component_controls(component, aic311x_snd_controls.as_ptr(), ARRAY_SIZE(&aic311x_snd_controls));
    } else {
        ret = snd_soc_add_component_controls(component, aic310x_snd_controls.as_ptr(), ARRAY_SIZE(&aic310x_snd_controls));
    }
    ret
}

unsafe extern "C" fn aic31xx_add_widgets(component: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let aic31xx = snd_soc_component_get_drvdata(component) as *mut aic31xx_priv;
    let mut ret: c_int;
    if ((*aic31xx).codec_type & DAC31XX_BIT) != 0 {
        ret = snd_soc_dapm_new_controls(dapm, dac31xx_dapm_widgets.as_ptr(), ARRAY_SIZE(&dac31xx_dapm_widgets));
        if ret != 0 { return ret; }
        ret = snd_soc_dapm_add_routes(dapm, dac31xx_audio_map.as_ptr(), ARRAY_SIZE(&dac31xx_audio_map));
        if ret != 0 { return ret; }
    } else {
        ret = snd_soc_dapm_new_controls(dapm, aic31xx_dapm_widgets.as_ptr(), ARRAY_SIZE(&aic31xx_dapm_widgets));
        if ret != 0 { return ret; }
        ret = snd_soc_dapm_add_routes(dapm, aic31xx_audio_map.as_ptr(), ARRAY_SIZE(&aic31xx_audio_map));
        if ret != 0 { return ret; }
    }
    if ((*aic31xx).codec_type & AIC31XX_STEREO_CLASS_D_BIT) != 0 {
        ret = snd_soc_dapm_new_controls(dapm, aic311x_dapm_widgets.as_ptr(), ARRAY_SIZE(&aic311x_dapm_widgets));
        if ret != 0 { return ret; }
        ret = snd_soc_dapm_add_routes(dapm, aic311x_audio_map.as_ptr(), ARRAY_SIZE(&aic311x_audio_map));
    } else {
        ret = snd_soc_dapm_new_controls(dapm, aic310x_dapm_widgets.as_ptr(), ARRAY_SIZE(&aic310x_dapm_widgets));
        if ret != 0 { return ret; }
        ret = snd_soc_dapm_add_routes(dapm, aic310x_audio_map.as_ptr(), ARRAY_SIZE(&aic310x_audio_map));
    }
    if ret != 0 { return ret; }
    0
}

unsafe extern "C" fn aic31xx_setup_pll(component: *mut snd_soc_component, params: *mut snd_pcm_hw_params) -> c_int {
    let aic31xx = snd_soc_component_get_drvdata(component) as *mut aic31xx_priv;
    let mut bclk_score = snd_soc_params_to_frame_size(params);
    let mut bclk_n = 0;
    let mut match_i: c_int = -1;
    if (*aic31xx).sysclk == 0 || (*aic31xx).p_div == 0 {
        dev_err((*component).dev, b"Master clock not supplied\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    let mclk_p = (*aic31xx).sysclk / (*aic31xx).p_div as c_uint;
    snd_soc_component_update_bits(component, AIC31XX_CLKMUX, AIC31XX_CODEC_CLKIN_MASK, AIC31XX_CODEC_CLKIN_PLL);
    snd_soc_component_update_bits(component, AIC31XX_IFACE2, AIC31XX_BDIVCLK_MASK, AIC31XX_DAC2BCLK);
    for i in 0..aic31xx_divs.len() {
        if aic31xx_divs[i].rate == params_rate(params) && aic31xx_divs[i].mclk_p == mclk_p {
            let fs = snd_soc_params_to_frame_size(params);
            let s = ((aic31xx_divs[i].dosr as c_int) * (aic31xx_divs[i].mdac as c_int)) % fs;
            let bn = ((aic31xx_divs[i].dosr as c_int) * (aic31xx_divs[i].mdac as c_int)) / fs;
            if s < bclk_score && bn > 0 { match_i = i as c_int; bclk_n = bn; bclk_score = s; }
        }
    }
    if match_i == -1 {
        dev_err((*component).dev, b"%s: Sample rate (%u) and format not supported\n\0".as_ptr() as *const c_char, b"aic31xx_setup_pll\0".as_ptr(), params_rate(params));
        return -EINVAL;
    }
    if bclk_score != 0 { dev_warn((*component).dev, b"Can not produce exact bitclock\0".as_ptr() as *const c_char); }
    let i = match_i as usize;
    snd_soc_component_update_bits(component, AIC31XX_PLLPR, AIC31XX_PLL_MASK, (((*aic31xx).p_div as c_uint) << 4) | aic31xx_divs[i].pll_r as c_uint);
    snd_soc_component_write(component, AIC31XX_PLLJ, aic31xx_divs[i].pll_j as c_uint);
    snd_soc_component_write(component, AIC31XX_PLLDMSB, (aic31xx_divs[i].pll_d >> 8) as c_uint);
    snd_soc_component_write(component, AIC31XX_PLLDLSB, (aic31xx_divs[i].pll_d & 0xff) as c_uint);
    snd_soc_component_update_bits(component, AIC31XX_NDAC, AIC31XX_PLL_MASK, aic31xx_divs[i].ndac as c_uint);
    snd_soc_component_update_bits(component, AIC31XX_MDAC, AIC31XX_PLL_MASK, aic31xx_divs[i].mdac as c_uint);
    snd_soc_component_write(component, AIC31XX_DOSRMSB, (aic31xx_divs[i].dosr >> 8) as c_uint);
    snd_soc_component_write(component, AIC31XX_DOSRLSB, (aic31xx_divs[i].dosr & 0xff) as c_uint);
    snd_soc_component_update_bits(component, AIC31XX_NADC, AIC31XX_PLL_MASK, if aic31xx_divs[i].nadc != 0 { aic31xx_divs[i].nadc as c_uint } else { 1 });
    snd_soc_component_update_bits(component, AIC31XX_MADC, AIC31XX_PLL_MASK, if aic31xx_divs[i].madc != 0 { aic31xx_divs[i].madc as c_uint } else { 1 });
    snd_soc_component_write(component, AIC31XX_AOSR, aic31xx_divs[i].aosr as c_uint);
    snd_soc_component_update_bits(component, AIC31XX_BCLKN, AIC31XX_PLL_MASK, bclk_n as c_uint);
    (*aic31xx).rate_div_line = i as c_int;
    0
}

unsafe extern "C" fn aic31xx_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let aic31xx = snd_soc_component_get_drvdata(component) as *mut aic31xx_priv;
    let mut data: u8 = 0;
    match params_width(params) {
        16 => {}
        20 => data = (AIC31XX_WORD_LEN_20BITS << AIC31XX_IFACE1_DATALEN_SHIFT) as u8,
        24 => data = (AIC31XX_WORD_LEN_24BITS << AIC31XX_IFACE1_DATALEN_SHIFT) as u8,
        32 => data = (AIC31XX_WORD_LEN_32BITS << AIC31XX_IFACE1_DATALEN_SHIFT) as u8,
        _ => { dev_err((*component).dev, b"%s: Unsupported width %d\n\0".as_ptr() as *const c_char, b"aic31xx_hw_params\0".as_ptr(), params_width(params)); return -EINVAL; }
    }
    snd_soc_component_update_bits(component, AIC31XX_IFACE1, AIC31XX_IFACE1_DATALEN_MASK, data as c_uint);
    if (*aic31xx).sysclk_id == AIC31XX_PLL_CLKIN_BCLK {
        (*aic31xx).sysclk = params_rate(params) * params_width(params) * params_channels(params);
        (*aic31xx).p_div = 1;
    }
    aic31xx_setup_pll(component, params)
}

unsafe extern "C" fn aic31xx_dac_mute(codec_dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*codec_dai).component;
    snd_soc_component_update_bits(component, AIC31XX_DACMUTE, AIC31XX_DACMUTE_MASK, if mute != 0 { AIC31XX_DACMUTE_MASK } else { 0 });
    0
}

unsafe extern "C" fn aic31xx_clock_master_routes(component: *mut snd_soc_component, mut fmt: c_uint) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let aic31xx = snd_soc_component_get_drvdata(component) as *mut aic31xx_priv;
    fmt &= SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK;
    if fmt == SND_SOC_DAIFMT_CBC_CFC && (*aic31xx).master_dapm_route_applied {
        let mut ret = snd_soc_dapm_del_routes(dapm, common31xx_cm_audio_map.as_ptr(), ARRAY_SIZE(&common31xx_cm_audio_map));
        if ret == 0 && ((*aic31xx).codec_type & DAC31XX_BIT) == 0 {
            ret = snd_soc_dapm_del_routes(dapm, aic31xx_cm_audio_map.as_ptr(), ARRAY_SIZE(&aic31xx_cm_audio_map));
        }
        if ret != 0 { return ret; }
        (*aic31xx).master_dapm_route_applied = false;
    } else if fmt != SND_SOC_DAIFMT_CBC_CFC && !(*aic31xx).master_dapm_route_applied {
        let mut ret = snd_soc_dapm_add_routes(dapm, common31xx_cm_audio_map.as_ptr(), ARRAY_SIZE(&common31xx_cm_audio_map));
        if ret == 0 && ((*aic31xx).codec_type & DAC31XX_BIT) == 0 {
            ret = snd_soc_dapm_add_routes(dapm, aic31xx_cm_audio_map.as_ptr(), ARRAY_SIZE(&aic31xx_cm_audio_map));
        }
        if ret != 0 { return ret; }
        (*aic31xx).master_dapm_route_applied = true;
    }
    0
}

unsafe extern "C" fn aic31xx_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let mut iface_reg1: u8 = 0;
    let mut iface_reg2: u8 = 0;
    let mut dsp_a_val: u8 = 0;
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        x if x == SND_SOC_DAIFMT_CBP_CFP => iface_reg1 |= (AIC31XX_BCLK_MASTER | AIC31XX_WCLK_MASTER) as u8,
        x if x == SND_SOC_DAIFMT_CBC_CFP => iface_reg1 |= AIC31XX_WCLK_MASTER as u8,
        x if x == SND_SOC_DAIFMT_CBP_CFC => iface_reg1 |= AIC31XX_BCLK_MASTER as u8,
        x if x == SND_SOC_DAIFMT_CBC_CFC => {}
        _ => { dev_err((*component).dev, b"Invalid DAI clock provider\n\0".as_ptr() as *const c_char); return -EINVAL; }
    }
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => {}
        x if x == SND_SOC_DAIFMT_IB_NF => iface_reg2 |= AIC31XX_BCLKINV_MASK as u8,
        _ => { dev_err((*component).dev, b"Invalid DAI clock signal polarity\n\0".as_ptr() as *const c_char); return -EINVAL; }
    }
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => {}
        x if x == SND_SOC_DAIFMT_DSP_A => { dsp_a_val = 1; iface_reg2 ^= AIC31XX_BCLKINV_MASK as u8; iface_reg1 |= (AIC31XX_DSP_MODE << AIC31XX_IFACE1_DATATYPE_SHIFT) as u8; }
        x if x == SND_SOC_DAIFMT_DSP_B => { iface_reg2 ^= AIC31XX_BCLKINV_MASK as u8; iface_reg1 |= (AIC31XX_DSP_MODE << AIC31XX_IFACE1_DATATYPE_SHIFT) as u8; }
        x if x == SND_SOC_DAIFMT_RIGHT_J => iface_reg1 |= (AIC31XX_RIGHT_JUSTIFIED_MODE << AIC31XX_IFACE1_DATATYPE_SHIFT) as u8,
        x if x == SND_SOC_DAIFMT_LEFT_J => iface_reg1 |= (AIC31XX_LEFT_JUSTIFIED_MODE << AIC31XX_IFACE1_DATATYPE_SHIFT) as u8,
        _ => { dev_err((*component).dev, b"Invalid DAI interface format\n\0".as_ptr() as *const c_char); return -EINVAL; }
    }
    snd_soc_component_update_bits(component, AIC31XX_IFACE1, AIC31XX_IFACE1_DATATYPE_MASK | AIC31XX_IFACE1_MASTER_MASK, iface_reg1 as c_uint);
    snd_soc_component_update_bits(component, AIC31XX_DATA_OFFSET, AIC31XX_DATA_OFFSET_MASK, dsp_a_val as c_uint);
    snd_soc_component_update_bits(component, AIC31XX_IFACE2, AIC31XX_BCLKINV_MASK, iface_reg2 as c_uint);
    aic31xx_clock_master_routes(component, fmt)
}

unsafe extern "C" fn aic31xx_set_dai_sysclk(codec_dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*codec_dai).component;
    let aic31xx = snd_soc_component_get_drvdata(component) as *mut aic31xx_priv;
    let mut i = 1;
    while i < 8 { if freq / i <= 20000000 { break; } i += 1; }
    if freq / i > 20000000 {
        dev_err((*aic31xx).dev, b"%s: Too high mclk frequency %u\n\0".as_ptr() as *const c_char, b"aic31xx_set_dai_sysclk\0".as_ptr(), freq);
        return -EINVAL;
    }
    (*aic31xx).p_div = i as u8;
    let mut j = 0usize;
    while j < aic31xx_divs.len() {
        if aic31xx_divs[j].mclk_p == freq / (*aic31xx).p_div as c_uint { break; }
        j += 1;
    }
    if j == aic31xx_divs.len() {
        dev_err((*aic31xx).dev, b"%s: Unsupported frequency %d\n\0".as_ptr() as *const c_char, b"aic31xx_set_dai_sysclk\0".as_ptr(), freq);
        return -EINVAL;
    }
    snd_soc_component_update_bits(component, AIC31XX_CLKMUX, AIC31XX_PLL_CLKIN_MASK, (clk_id as c_uint) << AIC31XX_PLL_CLKIN_SHIFT);
    (*aic31xx).sysclk_id = clk_id as u32;
    (*aic31xx).sysclk = freq;
    0
}

unsafe extern "C" fn aic31xx_regulator_event(nb: *mut notifier_block, event: c_ulong, _data: *mut c_void) -> c_int {
    let disable_nb = nb as *mut aic31xx_disable_nb;
    let aic31xx = (*disable_nb).aic31xx;
    if (event & REGULATOR_EVENT_DISABLE) != 0 {
        if !(*aic31xx).gpio_reset.is_null() { gpiod_set_value_cansleep((*aic31xx).gpio_reset, 1); }
        regcache_mark_dirty((*aic31xx).regmap);
        dev_dbg((*aic31xx).dev, b"## %s: DISABLE received\n\0".as_ptr() as *const c_char, b"aic31xx_regulator_event\0".as_ptr());
    }
    0
}

unsafe fn aic31xx_reset(aic31xx: *mut aic31xx_priv) -> c_int {
    let mut ret = 0;
    if !(*aic31xx).gpio_reset.is_null() {
        gpiod_set_value_cansleep((*aic31xx).gpio_reset, 1);
        ndelay(10);
        gpiod_set_value_cansleep((*aic31xx).gpio_reset, 0);
    } else {
        ret = regmap_write((*aic31xx).regmap, AIC31XX_RESET, 1);
    }
    mdelay(1);
    ret
}

unsafe fn aic31xx_clk_on(component: *mut snd_soc_component) {
    let aic31xx = snd_soc_component_get_drvdata(component) as *mut aic31xx_priv;
    let mask = AIC31XX_PM_MASK;
    let on = AIC31XX_PM_MASK;
    snd_soc_component_update_bits(component, AIC31XX_PLLPR, mask, on); mdelay(10);
    snd_soc_component_update_bits(component, AIC31XX_NDAC, mask, on);
    snd_soc_component_update_bits(component, AIC31XX_MDAC, mask, on);
    if aic31xx_divs[(*aic31xx).rate_div_line as usize].nadc != 0 { snd_soc_component_update_bits(component, AIC31XX_NADC, mask, on); }
    if aic31xx_divs[(*aic31xx).rate_div_line as usize].madc != 0 { snd_soc_component_update_bits(component, AIC31XX_MADC, mask, on); }
    snd_soc_component_update_bits(component, AIC31XX_BCLKN, mask, on);
}

unsafe fn aic31xx_clk_off(component: *mut snd_soc_component) {
    let mask = AIC31XX_PM_MASK;
    snd_soc_component_update_bits(component, AIC31XX_BCLKN, mask, 0);
    snd_soc_component_update_bits(component, AIC31XX_MADC, mask, 0);
    snd_soc_component_update_bits(component, AIC31XX_NADC, mask, 0);
    snd_soc_component_update_bits(component, AIC31XX_MDAC, mask, 0);
    snd_soc_component_update_bits(component, AIC31XX_NDAC, mask, 0);
    snd_soc_component_update_bits(component, AIC31XX_PLLPR, mask, 0);
}

unsafe fn aic31xx_power_on(component: *mut snd_soc_component) -> c_int {
    let aic31xx = snd_soc_component_get_drvdata(component) as *mut aic31xx_priv;
    let mut ret = regulator_bulk_enable(AIC31XX_NUM_SUPPLIES as c_uint, (*aic31xx).supplies.as_mut_ptr());
    if ret != 0 { return ret; }
    regcache_cache_only((*aic31xx).regmap, false);
    ret = aic31xx_reset(aic31xx);
    if ret < 0 { dev_err((*aic31xx).dev, b"Could not reset device: %d\n\0".as_ptr() as *const c_char, ret); }
    ret = regcache_sync((*aic31xx).regmap);
    if ret != 0 {
        dev_err((*component).dev, b"Failed to restore cache: %d\n\0".as_ptr() as *const c_char, ret);
        regcache_cache_only((*aic31xx).regmap, true);
        regulator_bulk_disable(AIC31XX_NUM_SUPPLIES as c_uint, (*aic31xx).supplies.as_mut_ptr());
        return ret;
    }
    aic31xx_set_jack(component, (*aic31xx).jack, ptr::null_mut());
    0
}

unsafe fn aic31xx_power_off(component: *mut snd_soc_component) {
    let aic31xx = snd_soc_component_get_drvdata(component) as *mut aic31xx_priv;
    regcache_cache_only((*aic31xx).regmap, true);
    regulator_bulk_disable(AIC31XX_NUM_SUPPLIES as c_uint, (*aic31xx).supplies.as_mut_ptr());
}

unsafe extern "C" fn aic31xx_set_bias_level(component: *mut snd_soc_component, level: c_int) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    match level {
        x if x == SND_SOC_BIAS_ON => {}
        x if x == SND_SOC_BIAS_PREPARE => if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_STANDBY { aic31xx_clk_on(component); },
        x if x == SND_SOC_BIAS_STANDBY => match snd_soc_dapm_get_bias_level(dapm) {
            y if y == SND_SOC_BIAS_OFF => { aic31xx_power_on(component); }
            y if y == SND_SOC_BIAS_PREPARE => aic31xx_clk_off(component),
            _ => BUG(),
        },
        x if x == SND_SOC_BIAS_OFF => if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_STANDBY { aic31xx_power_off(component); },
        _ => {}
    }
    0
}

unsafe extern "C" fn aic31xx_set_jack(component: *mut snd_soc_component, jack: *mut snd_soc_jack, _data: *mut c_void) -> c_int {
    let aic31xx = snd_soc_component_get_drvdata(component) as *mut aic31xx_priv;
    (*aic31xx).jack = jack;
    regmap_write((*aic31xx).regmap, AIC31XX_HSDETECT, if !jack.is_null() { AIC31XX_HSD_ENABLE } else { 0 });
    0
}

unsafe extern "C" fn aic31xx_codec_probe(component: *mut snd_soc_component) -> c_int {
    let aic31xx = snd_soc_component_get_drvdata(component) as *mut aic31xx_priv;
    (*aic31xx).component = component;
    for i in 0..AIC31XX_NUM_SUPPLIES {
        (*aic31xx).disable_nb[i].nb.notifier_call = Some(aic31xx_regulator_event);
        (*aic31xx).disable_nb[i].aic31xx = aic31xx;
        let ret = devm_regulator_register_notifier((*aic31xx).supplies[i].consumer, &mut (*aic31xx).disable_nb[i].nb);
        if ret != 0 { dev_err((*component).dev, b"Failed to request regulator notifier: %d\n\0".as_ptr() as *const c_char, ret); return ret; }
    }
    regcache_cache_only((*aic31xx).regmap, true);
    regcache_mark_dirty((*aic31xx).regmap);
    let mut ret = aic31xx_add_controls(component);
    if ret != 0 { return ret; }
    ret = aic31xx_add_widgets(component);
    if ret != 0 { return ret; }
    snd_soc_component_update_bits(component, AIC31XX_HPDRIVER, AIC31XX_HPD_OCMV_MASK, ((*aic31xx).ocmv as c_uint) << AIC31XX_HPD_OCMV_SHIFT);
    0
}

unsafe extern "C" fn aic31xx_irq(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let aic31xx = data as *mut aic31xx_priv;
    let dev = (*aic31xx).dev;
    let mut value: c_uint = 0;
    let mut handled = false;
    let mut ret = regmap_read((*aic31xx).regmap, AIC31XX_INTRDACFLAG, &mut value);
    if ret != 0 { dev_err(dev, b"Failed to read interrupt mask: %d\n\0".as_ptr() as *const c_char, ret); return IRQ_NONE; }
    if value != 0 {
        handled = true;
        if (value & AIC31XX_HPLSCDETECT) != 0 { dev_err(dev, b"Short circuit on Left output is detected\n\0".as_ptr() as *const c_char); }
        if (value & AIC31XX_HPRSCDETECT) != 0 { dev_err(dev, b"Short circuit on Right output is detected\n\0".as_ptr() as *const c_char); }
        if (value & (AIC31XX_HSPLUG | AIC31XX_BUTTONPRESS)) != 0 {
            let mut val: c_uint = 0;
            let mut status: c_int = 0;
            ret = regmap_read((*aic31xx).regmap, AIC31XX_INTRDACFLAG2, &mut val);
            if ret != 0 { dev_err(dev, b"Failed to read interrupt mask: %d\n\0".as_ptr() as *const c_char, ret); return if handled { IRQ_HANDLED } else { IRQ_NONE }; }
            if (val & AIC31XX_BUTTONPRESS) != 0 { status |= SND_JACK_BTN_0; }
            ret = regmap_read((*aic31xx).regmap, AIC31XX_HSDETECT, &mut val);
            if ret != 0 { dev_err(dev, b"Failed to read headset type: %d\n\0".as_ptr() as *const c_char, ret); return if handled { IRQ_HANDLED } else { IRQ_NONE }; }
            match (val & AIC31XX_HSD_TYPE_MASK) >> AIC31XX_HSD_TYPE_SHIFT {
                x if x == AIC31XX_HSD_HP => status |= SND_JACK_HEADPHONE,
                x if x == AIC31XX_HSD_HS => status |= SND_JACK_HEADSET,
                _ => {}
            }
            if !(*aic31xx).jack.is_null() { snd_soc_jack_report((*aic31xx).jack, status, AIC31XX_JACK_MASK); }
        }
        if (value & !(AIC31XX_HPLSCDETECT | AIC31XX_HPRSCDETECT | AIC31XX_HSPLUG | AIC31XX_BUTTONPRESS)) != 0 {
            dev_err(dev, b"Unknown DAC interrupt flags: 0x%08x\n\0".as_ptr() as *const c_char, value);
        }
    }
    ret = regmap_read((*aic31xx).regmap, AIC31XX_OFFLAG, &mut value);
    if ret != 0 { dev_err(dev, b"Failed to read overflow flag: %d\n\0".as_ptr() as *const c_char, ret); return if handled { IRQ_HANDLED } else { IRQ_NONE }; }
    if value != 0 {
        handled = true;
        if (value & AIC31XX_DAC_OF_LEFT) != 0 { dev_warn(dev, b"Left-channel DAC overflow has occurred\n\0".as_ptr() as *const c_char); }
        if (value & AIC31XX_DAC_OF_RIGHT) != 0 { dev_warn(dev, b"Right-channel DAC overflow has occurred\n\0".as_ptr() as *const c_char); }
        if (value & AIC31XX_DAC_OF_SHIFTER) != 0 { dev_warn(dev, b"DAC barrel shifter overflow has occurred\n\0".as_ptr() as *const c_char); }
        if (value & AIC31XX_ADC_OF) != 0 { dev_warn(dev, b"ADC overflow has occurred\n\0".as_ptr() as *const c_char); }
        if (value & AIC31XX_ADC_OF_SHIFTER) != 0 { dev_warn(dev, b"ADC barrel shifter overflow has occurred\n\0".as_ptr() as *const c_char); }
        if (value & !(AIC31XX_DAC_OF_LEFT | AIC31XX_DAC_OF_RIGHT | AIC31XX_DAC_OF_SHIFTER | AIC31XX_ADC_OF | AIC31XX_ADC_OF_SHIFTER)) != 0 {
            dev_warn(dev, b"Unknown overflow interrupt flags: 0x%08x\n\0".as_ptr() as *const c_char, value);
        }
    }
    if handled { IRQ_HANDLED } else { IRQ_NONE }
}

unsafe fn aic31xx_configure_ocmv(priv_: *mut aic31xx_priv) {
    let dev = (*priv_).dev;
    let mut value: u32 = 0;
    if !(*dev).fwnode.is_null() && fwnode_property_read_u32((*dev).fwnode, b"ai31xx-ocmv\0".as_ptr() as *const c_char, &mut value) != 0 {
        if value <= 3 { (*priv_).ocmv = value as u8; return; }
    }
    let avdd = regulator_get_voltage((*priv_).supplies[3].consumer);
    let dvdd = regulator_get_voltage((*priv_).supplies[5].consumer);
    if avdd > 3600000 || dvdd > 1950000 {
        dev_warn(dev, b"Too high supply voltage(s) AVDD: %d, DVDD: %d\n\0".as_ptr() as *const c_char, avdd, dvdd);
    } else if avdd == 3600000 && dvdd == 1950000 {
        (*priv_).ocmv = AIC31XX_HPD_OCMV_1_8V as u8;
    } else if avdd >= 3300000 && dvdd >= 1800000 {
        (*priv_).ocmv = AIC31XX_HPD_OCMV_1_65V as u8;
    } else if avdd >= 3000000 && dvdd >= 1650000 {
        (*priv_).ocmv = AIC31XX_HPD_OCMV_1_5V as u8;
    } else if avdd >= 2700000 && dvdd >= 1525000 {
        (*priv_).ocmv = AIC31XX_HPD_OCMV_1_35V as u8;
    } else {
        dev_warn(dev, b"Invalid supply voltage(s) AVDD: %d, DVDD: %d\n\0".as_ptr() as *const c_char, avdd, dvdd);
    }
}

unsafe fn get_unaligned_be16(data: *const u8) -> u16 {
    ((*data as u16) << 8) | (*data.add(1) as u16)
}

unsafe extern "C" fn tlv320dac3100_fw_load(aic31xx: *mut aic31xx_priv, mut data: *const u8, size: size_t) -> c_int {
    if size != 153 {
        dev_err((*aic31xx).dev, b"firmware size is %zu, expected 153 bytes\n\0".as_ptr() as *const c_char, size);
        return -EINVAL;
    }
    let mut val16 = get_unaligned_be16(data);
    if val16 != 0xb30c {
        dev_err((*aic31xx).dev, b"fw magic is 0x%04x expected 0xb30c\n\0".as_ptr() as *const c_char, val16 as c_uint);
        return -EINVAL;
    }
    data = data.add(2);
    val16 = get_unaligned_be16(data);
    if val16 != 0x0100 {
        dev_err((*aic31xx).dev, b"invalid firmware version 0x%04x! expected 1\0".as_ptr() as *const c_char, val16 as c_uint);
        return -EINVAL;
    }
    data = data.add(2);
    let mut ret = regmap_write((*aic31xx).regmap, AIC31XX_DACPRB, *data as c_uint);
    if ret != 0 { dev_err((*aic31xx).dev, b"failed to write PRB index: err %d\n\0".as_ptr() as *const c_char, ret); return ret; }
    data = data.add(1);
    let mut reg = 2;
    while reg < 126 {
        ret = regmap_write((*aic31xx).regmap, AIC31XX_REG(8, reg as c_uint), *data as c_uint);
        if ret != 0 { dev_err((*aic31xx).dev, b"failed to write page 8 filter coefficient %d: err %d\n\0".as_ptr() as *const c_char, reg, ret); return ret; }
        data = data.add(1); reg += 1;
    }
    reg = 2;
    while reg < 26 {
        ret = regmap_write((*aic31xx).regmap, AIC31XX_REG(9, reg as c_uint), *data as c_uint);
        if ret != 0 { dev_err((*aic31xx).dev, b"failed to write page 9 filter coefficient %d: err %d\n\0".as_ptr() as *const c_char, reg, ret); return ret; }
        data = data.add(1); reg += 1;
    }
    dev_info((*aic31xx).dev, b"done loading DAC filter coefficients\n\0".as_ptr() as *const c_char);
    ret
}

unsafe extern "C" fn tlv320dac3100_load_coeffs(aic31xx: *mut aic31xx_priv, fw_name: *const c_char) -> c_int {
    let mut fw: *const firmware = ptr::null();
    let ret = request_firmware(&mut fw, fw_name, (*aic31xx).dev);
    if ret != 0 { return ret; }
    tlv320dac3100_fw_load(aic31xx, (*fw).data, (*fw).size)
}

unsafe extern "C" fn aic31xx_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let mut micbias_value: c_uint = MICBIAS_2_0V;
    let aic31xx = devm_kzalloc(&mut (*i2c).dev, size_of::<aic31xx_priv>(), GFP_KERNEL) as *mut aic31xx_priv;
    if aic31xx.is_null() { return -ENOMEM; }
    (*aic31xx).regmap = devm_regmap_init_i2c(i2c, &raw const aic31xx_i2c_regmap);
    if IS_ERR((*aic31xx).regmap as *const c_void) {
        let ret = PTR_ERR((*aic31xx).regmap as *const c_void);
        dev_err(&mut (*i2c).dev, b"Failed to allocate register map: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    regcache_cache_only((*aic31xx).regmap, true);
    (*aic31xx).dev = &mut (*i2c).dev;
    (*aic31xx).irq = (*i2c).irq;
    (*aic31xx).codec_type = i2c_get_match_data(i2c) as usize as c_uint;
    dev_set_drvdata((*aic31xx).dev, aic31xx as *mut c_void);
    fwnode_property_read_u32((*(*aic31xx).dev).fwnode, b"ai31xx-micbias-vg\0".as_ptr() as *const c_char, &mut micbias_value);
    if micbias_value == MICBIAS_2_0V || micbias_value == MICBIAS_2_5V || micbias_value == MICBIAS_AVDDV {
        (*aic31xx).micbias_vg = micbias_value as c_int;
    } else {
        dev_err((*aic31xx).dev, b"Bad ai31xx-micbias-vg value %d\n\0".as_ptr() as *const c_char, micbias_value);
        (*aic31xx).micbias_vg = MICBIAS_2_0V as c_int;
    }
    let pdata = dev_get_platdata((*aic31xx).dev) as *mut aic31xx_pdata;
    if !pdata.is_null() {
        (*aic31xx).pdata.codec_type = (*pdata).codec_type;
        (*aic31xx).pdata.micbias_vg = (*pdata).micbias_vg;
        (*aic31xx).codec_type = (*aic31xx).pdata.codec_type;
        (*aic31xx).micbias_vg = (*aic31xx).pdata.micbias_vg;
    }
    (*aic31xx).gpio_reset = devm_gpiod_get_optional((*aic31xx).dev, b"reset\0".as_ptr() as *const c_char, GPIOD_OUT_LOW);
    if IS_ERR((*aic31xx).gpio_reset as *const c_void) {
        return dev_err_probe((*aic31xx).dev, PTR_ERR((*aic31xx).gpio_reset as *const c_void), b"not able to acquire gpio\n\0".as_ptr() as *const c_char);
    }
    for i in 0..AIC31XX_NUM_SUPPLIES { (*aic31xx).supplies[i].supply = aic31xx_supply_names[i]; }
    let mut ret = devm_regulator_bulk_get((*aic31xx).dev, AIC31XX_NUM_SUPPLIES as c_uint, (*aic31xx).supplies.as_mut_ptr());
    if ret != 0 { return dev_err_probe((*aic31xx).dev, ret, b"Failed to request supplies\n\0".as_ptr() as *const c_char); }
    aic31xx_configure_ocmv(aic31xx);
    if (*aic31xx).irq > 0 {
        regmap_update_bits((*aic31xx).regmap, AIC31XX_GPIO1, AIC31XX_GPIO1_FUNC_MASK, AIC31XX_GPIO1_INT1 << AIC31XX_GPIO1_FUNC_SHIFT);
        regmap_write((*aic31xx).regmap, AIC31XX_INT1CTRL, AIC31XX_HSPLUGDET | AIC31XX_BUTTONPRESSDET | AIC31XX_SC | AIC31XX_ENGINE);
        ret = devm_request_threaded_irq((*aic31xx).dev, (*aic31xx).irq, ptr::null(), aic31xx_irq, IRQF_ONESHOT, b"aic31xx-irq\0".as_ptr() as *const c_char, aic31xx as *mut c_void);
        if ret != 0 { dev_err((*aic31xx).dev, b"Unable to request IRQ\n\0".as_ptr() as *const c_char); return ret; }
    }
    if (*aic31xx).codec_type == DAC3100 {
        ret = tlv320dac3100_load_coeffs(aic31xx, b"tlv320dac3100-coeffs.bin\0".as_ptr() as *const c_char);
        if ret != 0 { dev_warn((*aic31xx).dev, b"Did not load any filter coefficients\n\0".as_ptr() as *const c_char); }
    }
    if ((*aic31xx).codec_type & DAC31XX_BIT) != 0 {
        devm_snd_soc_register_component(&mut (*i2c).dev, &soc_codec_driver_aic31xx, dac31xx_dai_driver.as_mut_ptr(), ARRAY_SIZE(&dac31xx_dai_driver) as c_int)
    } else {
        devm_snd_soc_register_component(&mut (*i2c).dev, &soc_codec_driver_aic31xx, aic31xx_dai_driver.as_mut_ptr(), ARRAY_SIZE(&aic31xx_dai_driver) as c_int)
    }
}

static soc_codec_driver_aic31xx: snd_soc_component_driver = snd_soc_component_driver { _private: [] };
static mut dac31xx_dai_driver: [snd_soc_dai_driver; 0] = [];
static mut aic31xx_dai_driver: [snd_soc_dai_driver; 0] = [];

/*
 * #if defined(CONFIG_OF)
 * static const struct of_device_id tlv320aic31xx_of_match[] = {
 *   "ti,tlv320aic310x", "ti,tlv320aic311x", "ti,tlv320aic3100",
 *   "ti,tlv320aic3110", "ti,tlv320aic3120", "ti,tlv320aic3111",
 *   "ti,tlv320dac3100", "ti,tlv320dac3101", {}
 * };
 * MODULE_DEVICE_TABLE(of, tlv320aic31xx_of_match);
 * #endif
 *
 * #ifdef CONFIG_ACPI
 * static const struct acpi_device_id aic31xx_acpi_match[] = { { "10TI3100", 0 }, {} };
 * MODULE_DEVICE_TABLE(acpi, aic31xx_acpi_match);
 * #endif
 *
 * static const struct i2c_device_id aic31xx_i2c_id[] maps device names to
 * AIC3100/AIC3110/AIC3120/AIC3111/DAC3100/DAC3101 exactly as in the C source.
 * static struct i2c_driver aic31xx_i2c_driver registers name
 * "tlv320aic31xx-codec", OF/ACPI match tables, probe aic31xx_i2c_probe, and
 * id_table aic31xx_i2c_id.
 * module_i2c_driver(aic31xx_i2c_driver);
 * MODULE_AUTHOR("Jyri Sarha <jsarha@ti.com>");
 * MODULE_DESCRIPTION("ASoC TLV320AIC31xx CODEC Driver");
 * MODULE_LICENSE("GPL v2");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
