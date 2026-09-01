// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8903.rs  --  WM8903 ALSA SoC Audio driver
 *
 * Rust source-level translation of wm8903.c.  Linux/ASoC/kernel types,
 * constants, helper functions, and macro constructors referenced here are
 * external dependencies supplied by the surrounding driver tree.
 *
 * Copyright 2008-12 Wolfson Microelectronics
 * Copyright 2011-2012 NVIDIA, Inc.
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 *
 * TODO:
 *  - TDM mode configuration.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type bool_ = bool;
type u16_ = u16;
type u32_ = u32;
type irqreturn_t = c_int;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gpio_chip {
    pub label: *const c_char,
    pub owner: *mut c_void,
    pub request: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub direction_input: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub direction_output: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint, c_int) -> c_int>,
    pub set: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint, c_int) -> c_int>,
    pub can_sleep: c_int,
    pub ngpio: c_int,
    pub parent: *mut device,
    pub base: c_int,
}
#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
    pub shift: c_int,
}
#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}
#[repr(C)]
pub struct snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
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
}
#[repr(C)]
pub struct irq_data {
    _private: [u8; 0],
}
#[repr(C)]
pub struct i2c_dev {
    pub of_node: *mut device_node,
}
#[repr(C)]
pub struct i2c_client {
    pub dev: i2c_dev,
    pub irq: c_int,
}
#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}
#[repr(C)]
pub struct wm8903_platform_data {
    pub gpio_base: c_int,
    pub irq_active_low: bool,
    pub micdet_cfg: u32,
    pub micdet_delay: u32,
    pub gpio_cfg: [u32; WM8903_NUM_GPIO as usize],
}

#[repr(C)]
pub struct wm8903_priv {
    pub pdata: *mut wm8903_platform_data,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub supplies: [regulator_bulk_data; WM8903_NUM_SUPPLIES as usize],
    pub sysclk: c_int,
    pub irq: c_int,
    pub lock: mutex,
    pub fs: c_int,
    pub deemph: c_int,
    pub dcs_pending: c_int,
    pub dcs_cache: [c_int; 4],
    /* Reference count */
    pub class_w_users: c_int,
    pub mic_jack: *mut snd_soc_jack,
    pub mic_det: c_int,
    pub mic_short: c_int,
    pub mic_last_report: c_int,
    pub mic_delay: c_int,
    /* CONFIG_GPIOLIB */
    pub gpio_chip: gpio_chip,
}

const WM8903_NUM_SUPPLIES: c_int = 4;

static wm8903_reg_defaults: [reg_default; 68] = [
    reg_default { reg: 4, def: 0x0018 }, reg_default { reg: 5, def: 0x0000 },
    reg_default { reg: 6, def: 0x0000 }, reg_default { reg: 8, def: 0x0001 },
    reg_default { reg: 10, def: 0x0001 }, reg_default { reg: 12, def: 0x0000 },
    reg_default { reg: 13, def: 0x0000 }, reg_default { reg: 14, def: 0x0000 },
    reg_default { reg: 15, def: 0x0000 }, reg_default { reg: 16, def: 0x0000 },
    reg_default { reg: 17, def: 0x0000 }, reg_default { reg: 18, def: 0x0000 },
    reg_default { reg: 20, def: 0x0400 }, reg_default { reg: 21, def: 0x0D07 },
    reg_default { reg: 22, def: 0x0000 }, reg_default { reg: 24, def: 0x0050 },
    reg_default { reg: 25, def: 0x0242 }, reg_default { reg: 26, def: 0x0008 },
    reg_default { reg: 27, def: 0x0022 }, reg_default { reg: 30, def: 0x00C0 },
    reg_default { reg: 31, def: 0x00C0 }, reg_default { reg: 32, def: 0x0000 },
    reg_default { reg: 33, def: 0x0000 }, reg_default { reg: 36, def: 0x00C0 },
    reg_default { reg: 37, def: 0x00C0 }, reg_default { reg: 38, def: 0x0000 },
    reg_default { reg: 39, def: 0x0073 }, reg_default { reg: 40, def: 0x09BF },
    reg_default { reg: 41, def: 0x3241 }, reg_default { reg: 42, def: 0x0020 },
    reg_default { reg: 43, def: 0x0000 }, reg_default { reg: 44, def: 0x0085 },
    reg_default { reg: 45, def: 0x0085 }, reg_default { reg: 46, def: 0x0044 },
    reg_default { reg: 47, def: 0x0044 }, reg_default { reg: 50, def: 0x0008 },
    reg_default { reg: 51, def: 0x0004 }, reg_default { reg: 52, def: 0x0000 },
    reg_default { reg: 53, def: 0x0000 }, reg_default { reg: 54, def: 0x0000 },
    reg_default { reg: 55, def: 0x0000 }, reg_default { reg: 57, def: 0x002D },
    reg_default { reg: 58, def: 0x002D }, reg_default { reg: 59, def: 0x0039 },
    reg_default { reg: 60, def: 0x0039 }, reg_default { reg: 62, def: 0x0139 },
    reg_default { reg: 63, def: 0x0139 }, reg_default { reg: 64, def: 0x0000 },
    reg_default { reg: 67, def: 0x0010 }, reg_default { reg: 69, def: 0x00A4 },
    reg_default { reg: 90, def: 0x0000 }, reg_default { reg: 94, def: 0x0000 },
    reg_default { reg: 98, def: 0x0000 }, reg_default { reg: 104, def: 0x0000 },
    reg_default { reg: 108, def: 0x0000 }, reg_default { reg: 109, def: 0x0000 },
    reg_default { reg: 110, def: 0x0000 }, reg_default { reg: 111, def: 0x0000 },
    reg_default { reg: 112, def: 0x0000 }, reg_default { reg: 114, def: 0x0000 },
    reg_default { reg: 116, def: 0x00A8 }, reg_default { reg: 117, def: 0x00A8 },
    reg_default { reg: 118, def: 0x00A8 }, reg_default { reg: 119, def: 0x0220 },
    reg_default { reg: 120, def: 0x01A0 }, reg_default { reg: 122, def: 0xFFFF },
    reg_default { reg: 123, def: 0x0000 }, reg_default { reg: 126, def: 0x0000 },
    reg_default { reg: 129, def: 0x0000 }, reg_default { reg: 149, def: 0x6810 },
    reg_default { reg: 164, def: 0x0028 }, reg_default { reg: 172, def: 0x0000 },
];

static wm8903_supply_names: [*const c_char; WM8903_NUM_SUPPLIES as usize] = [
    b"AVDD\0".as_ptr() as *const c_char,
    b"CPVDD\0".as_ptr() as *const c_char,
    b"DBVDD\0".as_ptr() as *const c_char,
    b"DCVDD\0".as_ptr() as *const c_char,
];

extern "C" {
    static mut THIS_MODULE: *mut c_void;
    fn WARN_ON(condition: bool) -> c_int;
    fn mdelay(ms: c_uint);
    fn msleep(ms: c_uint);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_dapm_kcontrol_to_component(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_dapm_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut c_void;
    fn gpiochip_add_data(chip: *mut gpio_chip, data: *mut c_void) -> c_int;
    fn gpiochip_remove(chip: *mut gpio_chip);
    fn irq_get_irq_data(irq: c_int) -> *mut irq_data;
    fn irqd_get_trigger_type(data: *mut irq_data) -> c_uint;
    fn of_property_read_u32(np: *const device_node, name: *const c_char, out: *mut u32) -> c_int;
    fn of_property_read_u32_array(np: *const device_node, name: *const c_char, out: *mut u32, sz: c_uint) -> c_int;
    fn dev_get_platdata(dev: *mut i2c_dev) -> *mut c_void;
    fn devm_kzalloc(dev: *mut i2c_dev, size: usize, flags: c_uint) -> *mut c_void;
    fn mutex_init(lock: *mut mutex);
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, cfg: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(i2c: *mut i2c_client) -> *mut c_void;
    fn devm_regulator_bulk_get(dev: *mut i2c_dev, n: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_enable(n: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(n: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn request_threaded_irq(irq: c_int, handler: *const c_void, thread_fn: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>, flags: c_uint, name: *const c_char, data: *mut c_void) -> c_int;
    fn free_irq(irq: c_int, data: *mut c_void);
    fn devm_snd_soc_register_component(dev: *mut i2c_dev, component_driver: *const snd_soc_component_driver, dai: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn trace_snd_soc_jack_irq(name: *const c_char);
}

// Register, bit-mask, ALSA, I2C, IRQ, GPIO, errno, and macro-generated
// descriptor constants come from the translated headers and kernel framework.
extern "C" {
    static WM8903_SW_RESET_AND_ID: c_uint; static WM8903_REVISION_NUMBER: c_uint;
    static WM8903_BIAS_CONTROL_0: c_uint; static WM8903_VMID_CONTROL_0: c_uint;
    static WM8903_MIC_BIAS_CONTROL_0: c_uint; static WM8903_ANALOGUE_DAC_0: c_uint;
    static WM8903_ANALOGUE_ADC_0: c_uint; static WM8903_POWER_MANAGEMENT_0: c_uint;
    static WM8903_POWER_MANAGEMENT_1: c_uint; static WM8903_POWER_MANAGEMENT_2: c_uint;
    static WM8903_POWER_MANAGEMENT_3: c_uint; static WM8903_POWER_MANAGEMENT_4: c_uint;
    static WM8903_POWER_MANAGEMENT_5: c_uint; static WM8903_POWER_MANAGEMENT_6: c_uint;
    static WM8903_CLOCK_RATES_0: c_uint; static WM8903_CLOCK_RATES_1: c_uint;
    static WM8903_CLOCK_RATES_2: c_uint; static WM8903_AUDIO_INTERFACE_0: c_uint;
    static WM8903_AUDIO_INTERFACE_1: c_uint; static WM8903_AUDIO_INTERFACE_2: c_uint;
    static WM8903_AUDIO_INTERFACE_3: c_uint; static WM8903_DAC_DIGITAL_VOLUME_LEFT: c_uint;
    static WM8903_DAC_DIGITAL_VOLUME_RIGHT: c_uint; static WM8903_DAC_DIGITAL_0: c_uint;
    static WM8903_DAC_DIGITAL_1: c_uint; static WM8903_ADC_DIGITAL_VOLUME_LEFT: c_uint;
    static WM8903_ADC_DIGITAL_VOLUME_RIGHT: c_uint; static WM8903_ADC_DIGITAL_0: c_uint;
    static WM8903_DIGITAL_MICROPHONE_0: c_uint; static WM8903_DRC_0: c_uint;
    static WM8903_DRC_1: c_uint; static WM8903_DRC_2: c_uint; static WM8903_DRC_3: c_uint;
    static WM8903_ANALOGUE_LEFT_INPUT_0: c_uint; static WM8903_ANALOGUE_RIGHT_INPUT_0: c_uint;
    static WM8903_ANALOGUE_LEFT_INPUT_1: c_uint; static WM8903_ANALOGUE_RIGHT_INPUT_1: c_uint;
    static WM8903_ANALOGUE_LEFT_MIX_0: c_uint; static WM8903_ANALOGUE_RIGHT_MIX_0: c_uint;
    static WM8903_ANALOGUE_SPK_MIX_LEFT_0: c_uint; static WM8903_ANALOGUE_SPK_MIX_LEFT_1: c_uint;
    static WM8903_ANALOGUE_SPK_MIX_RIGHT_0: c_uint; static WM8903_ANALOGUE_SPK_MIX_RIGHT_1: c_uint;
    static WM8903_ANALOGUE_OUT1_LEFT: c_uint; static WM8903_ANALOGUE_OUT1_RIGHT: c_uint;
    static WM8903_ANALOGUE_OUT2_LEFT: c_uint; static WM8903_ANALOGUE_OUT2_RIGHT: c_uint;
    static WM8903_ANALOGUE_OUT3_LEFT: c_uint; static WM8903_ANALOGUE_OUT3_RIGHT: c_uint;
    static WM8903_ANALOGUE_SPK_OUTPUT_CONTROL_0: c_uint; static WM8903_DC_SERVO_0: c_uint;
    static WM8903_DC_SERVO_2: c_uint; static WM8903_DC_SERVO_4: c_uint;
    static WM8903_DC_SERVO_READBACK_1: c_uint; static WM8903_DC_SERVO_READBACK_2: c_uint;
    static WM8903_DC_SERVO_READBACK_3: c_uint; static WM8903_DC_SERVO_READBACK_4: c_uint;
    static WM8903_ANALOGUE_HP_0: c_uint; static WM8903_ANALOGUE_LINEOUT_0: c_uint;
    static WM8903_CHARGE_PUMP_0: c_uint; static WM8903_CLASS_W_0: c_uint;
    static WM8903_WRITE_SEQUENCER_0: c_uint; static WM8903_WRITE_SEQUENCER_1: c_uint;
    static WM8903_WRITE_SEQUENCER_2: c_uint; static WM8903_WRITE_SEQUENCER_3: c_uint;
    static WM8903_WRITE_SEQUENCER_4: c_uint; static WM8903_CONTROL_INTERFACE: c_uint;
    static WM8903_GPIO_CONTROL_1: c_uint; static WM8903_GPIO_CONTROL_2: c_uint;
    static WM8903_GPIO_CONTROL_3: c_uint; static WM8903_GPIO_CONTROL_4: c_uint;
    static WM8903_GPIO_CONTROL_5: c_uint; static WM8903_INTERRUPT_STATUS_1: c_uint;
    static WM8903_INTERRUPT_STATUS_1_MASK: c_uint; static WM8903_INTERRUPT_POLARITY_1: c_uint;
    static WM8903_INTERRUPT_CONTROL: c_uint; static WM8903_CLOCK_RATE_TEST_4: c_uint;
    static WM8903_ANALOGUE_OUTPUT_BIAS_0: c_uint; static WM8903_MAX_REGISTER: c_uint;
    static WM8903_DCS_MODE_MASK: c_uint; static WM8903_DCS_ENA_MASK: c_uint;
    static WM8903_CP_DYN_FREQ: c_uint; static WM8903_CP_DYN_V: c_uint;
    static WM8903_DEEMPH_SHIFT: c_uint; static WM8903_DEEMPH_MASK: c_uint;
    static WM8903_LRCLK_DIR: c_uint; static WM8903_BCLK_DIR: c_uint;
    static WM8903_AIF_FMT_MASK: c_uint; static WM8903_AIF_LRCLK_INV: c_uint;
    static WM8903_AIF_BCLK_INV: c_uint; static WM8903_DAC_MUTE: c_uint;
    static WM8903_DAC_SB_FILT: c_uint; static WM8903_SAMPLE_RATE_MASK: c_uint;
    static WM8903_AIF_WL_MASK: c_uint; static WM8903_MCLKDIV2: c_uint;
    static WM8903_CLK_SYS_RATE_MASK: c_uint; static WM8903_CLK_SYS_MODE_MASK: c_uint;
    static WM8903_CLK_SYS_RATE_SHIFT: c_uint; static WM8903_CLK_SYS_MODE_SHIFT: c_uint;
    static WM8903_BCLK_DIV_MASK: c_uint; static WM8903_LRCLK_RATE_MASK: c_uint;
    static WM8903_MICDET_EINT: c_uint; static WM8903_MICSHRT_EINT: c_uint;
    static WM8903_WSEQ_ENA: c_uint; static WM8903_MICDET_ENA: c_uint;
    static WM8903_WSEQ_BUSY_EINT: c_uint; static WM8903_MICSHRT_INV: c_uint;
    static WM8903_MICDET_INV: c_uint; static WM8903_IM_WSEQ_BUSY_EINT: c_uint;
    static WM8903_IRQ_POL: c_uint; static WM8903_ADCVU: c_uint; static WM8903_DACVU: c_uint;
    static WM8903_HPOUTVU: c_uint; static WM8903_LINEOUTVU: c_uint; static WM8903_SPKVU: c_uint;
    static WM8903_DAC_MUTEMODE: c_uint; static WM8903_CHIP_REV_MASK: c_uint;
    static WM8903_POBCTRL: c_uint; static WM8903_ISEL_MASK: c_uint; static WM8903_ISEL_SHIFT: c_uint;
    static WM8903_STARTUP_BIAS_ENA: c_uint; static WM8903_BIAS_ENA: c_uint;
    static WM8903_SPK_DISCHARGE: c_uint; static WM8903_SPKL_ENA: c_uint; static WM8903_SPKR_ENA: c_uint;
    static WM8903_VMID_TIE_ENA: c_uint; static WM8903_BUFIO_ENA: c_uint; static WM8903_VMID_IO_ENA: c_uint;
    static WM8903_VMID_SOFT_MASK: c_uint; static WM8903_VMID_SOFT_SHIFT: c_uint;
    static WM8903_VMID_RES_MASK: c_uint; static WM8903_VMID_RES_50K: c_uint;
    static WM8903_VMID_RES_250K: c_uint; static WM8903_VMID_BUF_ENA: c_uint;
    static WM8903_NUM_GPIO: c_uint; static WM8903_GP1_FN_MASK: c_uint; static WM8903_GP1_DIR_MASK: c_uint;
    static WM8903_GPn_FN_GPIO_INPUT: c_uint; static WM8903_GP1_FN_SHIFT: c_uint; static WM8903_GP1_DIR: c_uint;
    static WM8903_GP1_LVL_MASK: c_uint; static WM8903_GP1_LVL_SHIFT: c_uint;
    static WM8903_GPn_FN_GPIO_OUTPUT: c_uint; static WM8903_GP2_LVL_SHIFT: c_uint;
    static WM8903_GPIO_CONFIG_ZERO: u32; static WM8903_GPn_FN_MICBIAS_CURRENT_DETECT: c_uint;
    static WM8903_GPn_FN_MICBIAS_SHORT_DETECT: c_uint;
    static SND_SOC_DAPM_POST_PMU: c_int; static SND_SOC_DAPM_PRE_PMD: c_int;
    static SND_SOC_BIAS_ON: c_int; static SND_SOC_BIAS_PREPARE: c_int;
    static SND_SOC_BIAS_STANDBY: c_int; static SND_SOC_BIAS_OFF: c_int;
    static SND_SOC_DAIFMT_MASTER_MASK: c_uint; static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_CBC_CFP: c_uint; static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_CBP_CFC: c_uint; static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint; static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint; static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint; static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint; static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint; static SND_SOC_DAIFMT_NB_IF: c_uint;
    static IRQ_NONE: c_int; static IRQ_HANDLED: c_int; static IRQ_TYPE_NONE: c_uint;
    static IRQ_TYPE_LEVEL_HIGH: c_uint; static IRQ_TYPE_LEVEL_LOW: c_uint;
    static IRQF_TRIGGER_LOW: c_uint; static IRQF_TRIGGER_HIGH: c_uint; static IRQF_ONESHOT: c_uint;
    static GFP_KERNEL: c_uint; static EINVAL: c_int; static ENOMEM: c_int; static ENODEV: c_int;
}

#[inline] unsafe fn neg_errno(e: c_int) -> c_int { -e }
#[inline] fn abs_i(v: c_int) -> c_int { if v < 0 { -v } else { v } }

unsafe fn wm8903_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    reg == WM8903_SW_RESET_AND_ID || reg == WM8903_REVISION_NUMBER ||
    reg == WM8903_BIAS_CONTROL_0 || reg == WM8903_VMID_CONTROL_0 ||
    reg == WM8903_MIC_BIAS_CONTROL_0 || reg == WM8903_ANALOGUE_DAC_0 ||
    reg == WM8903_ANALOGUE_ADC_0 || reg == WM8903_POWER_MANAGEMENT_0 ||
    reg == WM8903_POWER_MANAGEMENT_1 || reg == WM8903_POWER_MANAGEMENT_2 ||
    reg == WM8903_POWER_MANAGEMENT_3 || reg == WM8903_POWER_MANAGEMENT_4 ||
    reg == WM8903_POWER_MANAGEMENT_5 || reg == WM8903_POWER_MANAGEMENT_6 ||
    reg == WM8903_CLOCK_RATES_0 || reg == WM8903_CLOCK_RATES_1 ||
    reg == WM8903_CLOCK_RATES_2 || reg == WM8903_AUDIO_INTERFACE_0 ||
    reg == WM8903_AUDIO_INTERFACE_1 || reg == WM8903_AUDIO_INTERFACE_2 ||
    reg == WM8903_AUDIO_INTERFACE_3 || reg == WM8903_DAC_DIGITAL_VOLUME_LEFT ||
    reg == WM8903_DAC_DIGITAL_VOLUME_RIGHT || reg == WM8903_DAC_DIGITAL_0 ||
    reg == WM8903_DAC_DIGITAL_1 || reg == WM8903_ADC_DIGITAL_VOLUME_LEFT ||
    reg == WM8903_ADC_DIGITAL_VOLUME_RIGHT || reg == WM8903_ADC_DIGITAL_0 ||
    reg == WM8903_DIGITAL_MICROPHONE_0 || reg == WM8903_DRC_0 ||
    reg == WM8903_DRC_1 || reg == WM8903_DRC_2 || reg == WM8903_DRC_3 ||
    reg == WM8903_ANALOGUE_LEFT_INPUT_0 || reg == WM8903_ANALOGUE_RIGHT_INPUT_0 ||
    reg == WM8903_ANALOGUE_LEFT_INPUT_1 || reg == WM8903_ANALOGUE_RIGHT_INPUT_1 ||
    reg == WM8903_ANALOGUE_LEFT_MIX_0 || reg == WM8903_ANALOGUE_RIGHT_MIX_0 ||
    reg == WM8903_ANALOGUE_SPK_MIX_LEFT_0 || reg == WM8903_ANALOGUE_SPK_MIX_LEFT_1 ||
    reg == WM8903_ANALOGUE_SPK_MIX_RIGHT_0 || reg == WM8903_ANALOGUE_SPK_MIX_RIGHT_1 ||
    reg == WM8903_ANALOGUE_OUT1_LEFT || reg == WM8903_ANALOGUE_OUT1_RIGHT ||
    reg == WM8903_ANALOGUE_OUT2_LEFT || reg == WM8903_ANALOGUE_OUT2_RIGHT ||
    reg == WM8903_ANALOGUE_OUT3_LEFT || reg == WM8903_ANALOGUE_OUT3_RIGHT ||
    reg == WM8903_ANALOGUE_SPK_OUTPUT_CONTROL_0 || reg == WM8903_DC_SERVO_0 ||
    reg == WM8903_DC_SERVO_2 || reg == WM8903_DC_SERVO_READBACK_1 ||
    reg == WM8903_DC_SERVO_READBACK_2 || reg == WM8903_DC_SERVO_READBACK_3 ||
    reg == WM8903_DC_SERVO_READBACK_4 || reg == WM8903_ANALOGUE_HP_0 ||
    reg == WM8903_ANALOGUE_LINEOUT_0 || reg == WM8903_CHARGE_PUMP_0 ||
    reg == WM8903_CLASS_W_0 || reg == WM8903_WRITE_SEQUENCER_0 ||
    reg == WM8903_WRITE_SEQUENCER_1 || reg == WM8903_WRITE_SEQUENCER_2 ||
    reg == WM8903_WRITE_SEQUENCER_3 || reg == WM8903_WRITE_SEQUENCER_4 ||
    reg == WM8903_CONTROL_INTERFACE || reg == WM8903_GPIO_CONTROL_1 ||
    reg == WM8903_GPIO_CONTROL_2 || reg == WM8903_GPIO_CONTROL_3 ||
    reg == WM8903_GPIO_CONTROL_4 || reg == WM8903_GPIO_CONTROL_5 ||
    reg == WM8903_INTERRUPT_STATUS_1 || reg == WM8903_INTERRUPT_STATUS_1_MASK ||
    reg == WM8903_INTERRUPT_POLARITY_1 || reg == WM8903_INTERRUPT_CONTROL ||
    reg == WM8903_CLOCK_RATE_TEST_4 || reg == WM8903_ANALOGUE_OUTPUT_BIAS_0
}

unsafe fn wm8903_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    reg == WM8903_SW_RESET_AND_ID || reg == WM8903_REVISION_NUMBER ||
    reg == WM8903_INTERRUPT_STATUS_1 || reg == WM8903_WRITE_SEQUENCER_4 ||
    reg == WM8903_DC_SERVO_READBACK_1 || reg == WM8903_DC_SERVO_READBACK_2 ||
    reg == WM8903_DC_SERVO_READBACK_3 || reg == WM8903_DC_SERVO_READBACK_4
}

unsafe extern "C" fn wm8903_cp_event(_w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    WARN_ON(event != SND_SOC_DAPM_POST_PMU);
    mdelay(4);
    0
}

unsafe extern "C" fn wm8903_dcs_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let wm8903 = snd_soc_component_get_drvdata(component) as *mut wm8903_priv;
    if event == SND_SOC_DAPM_POST_PMU {
        (*wm8903).dcs_pending |= 1 << (*w).shift;
    } else if event == SND_SOC_DAPM_PRE_PMD {
        snd_soc_component_update_bits(component, WM8903_DC_SERVO_0, 1u32 << (*w).shift, 0);
    }
    0
}

const WM8903_DCS_MODE_WRITE_STOP: c_int = 0;
const WM8903_DCS_MODE_START_STOP: c_int = 2;

unsafe extern "C" fn wm8903_seq_notifier(component: *mut snd_soc_component, _event: c_int, _subseq: c_int) {
    let wm8903 = snd_soc_component_get_drvdata(component) as *mut wm8903_priv;
    let mut dcs_mode = WM8903_DCS_MODE_WRITE_STOP;
    let mut val: c_int;
    /* Complete any pending DC servo starts */
    if (*wm8903).dcs_pending != 0 {
        dev_dbg((*component).dev, b"Starting DC servo for %x\n\0".as_ptr() as *const c_char, (*wm8903).dcs_pending);
        for i in 0..(*wm8903).dcs_cache.len() {
            if ((*wm8903).dcs_pending & (1 << i)) == 0 { continue; }
            if (*wm8903).dcs_cache[i] != 0 {
                dev_dbg((*component).dev, b"Restore DC servo %d value %x\n\0".as_ptr() as *const c_char, 3 - i as c_int, (*wm8903).dcs_cache[i]);
                snd_soc_component_write(component, WM8903_DC_SERVO_4 + i as c_uint, ((*wm8903).dcs_cache[i] & 0xff) as c_uint);
            } else {
                dev_dbg((*component).dev, b"Calibrate DC servo %d\n\0".as_ptr() as *const c_char, 3 - i as c_int);
                dcs_mode = WM8903_DCS_MODE_START_STOP;
            }
        }
        /* Don't trust the cache for analogue */
        if (*wm8903).class_w_users != 0 { dcs_mode = WM8903_DCS_MODE_START_STOP; }
        snd_soc_component_update_bits(component, WM8903_DC_SERVO_2, WM8903_DCS_MODE_MASK, dcs_mode as c_uint);
        snd_soc_component_update_bits(component, WM8903_DC_SERVO_0, WM8903_DCS_ENA_MASK, (*wm8903).dcs_pending as c_uint);
        match dcs_mode {
            WM8903_DCS_MODE_WRITE_STOP => {}
            WM8903_DCS_MODE_START_STOP => {
                msleep(270);
                /* Cache the measured offsets for digital */
                if (*wm8903).class_w_users == 0 {
                    for i in 0..(*wm8903).dcs_cache.len() {
                        if ((*wm8903).dcs_pending & (1 << i)) == 0 { continue; }
                        val = snd_soc_component_read(component, WM8903_DC_SERVO_READBACK_1 + i as c_uint) as c_int;
                        dev_dbg((*component).dev, b"DC servo %d: %x\n\0".as_ptr() as *const c_char, 3 - i as c_int, val);
                        (*wm8903).dcs_cache[i] = val;
                    }
                }
            }
            _ => pr_warn(b"DCS mode %d delay not set\n\0".as_ptr() as *const c_char, dcs_mode),
        }
        (*wm8903).dcs_pending = 0;
    }
}

unsafe extern "C" fn wm8903_class_w_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let wm8903 = snd_soc_component_get_drvdata(component) as *mut wm8903_priv;
    let reg = snd_soc_component_read(component, WM8903_CLASS_W_0) as u16_;
    let ret: c_int;
    if (*ucontrol).value.integer.value[0] != 0 {
        if (*wm8903).class_w_users == 0 {
            dev_dbg((*component).dev, b"Disabling Class W\n\0".as_ptr() as *const c_char);
            snd_soc_component_write(component, WM8903_CLASS_W_0, (reg as c_uint) & !(WM8903_CP_DYN_FREQ | WM8903_CP_DYN_V));
        }
        (*wm8903).class_w_users += 1;
    }
    ret = snd_soc_dapm_put_volsw(kcontrol, ucontrol);
    if (*ucontrol).value.integer.value[0] == 0 {
        if (*wm8903).class_w_users == 1 {
            dev_dbg((*component).dev, b"Enabling Class W\n\0".as_ptr() as *const c_char);
            snd_soc_component_write(component, WM8903_CLASS_W_0, (reg as c_uint) | WM8903_CP_DYN_FREQ | WM8903_CP_DYN_V);
        }
        (*wm8903).class_w_users -= 1;
    }
    dev_dbg((*component).dev, b"Bypass use count now %d\n\0".as_ptr() as *const c_char, (*wm8903).class_w_users);
    ret
}

static mut wm8903_deemph: [c_int; 4] = [0, 32000, 44100, 48000];

unsafe fn wm8903_set_deemph(component: *mut snd_soc_component) -> c_int {
    let wm8903 = snd_soc_component_get_drvdata(component) as *mut wm8903_priv;
    let mut best: c_int;
    let val: c_int;
    if (*wm8903).deemph != 0 {
        best = 1;
        for i in 2..wm8903_deemph.len() {
            if abs_i(wm8903_deemph[i] - (*wm8903).fs) < abs_i(wm8903_deemph[best as usize] - (*wm8903).fs) {
                best = i as c_int;
            }
        }
        val = best << WM8903_DEEMPH_SHIFT;
    } else {
        best = 0;
        val = 0;
    }
    dev_dbg((*component).dev, b"Set deemphasis %d (%dHz)\n\0".as_ptr() as *const c_char, best, wm8903_deemph[best as usize]);
    snd_soc_component_update_bits(component, WM8903_DAC_DIGITAL_1, WM8903_DEEMPH_MASK, val as c_uint)
}

unsafe extern "C" fn wm8903_get_deemph(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let wm8903 = snd_soc_component_get_drvdata(component) as *mut wm8903_priv;
    (*ucontrol).value.integer.value[0] = (*wm8903).deemph as i64;
    0
}

unsafe extern "C" fn wm8903_put_deemph(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let wm8903 = snd_soc_component_get_drvdata(component) as *mut wm8903_priv;
    let deemph = (*ucontrol).value.integer.value[0] as c_uint;
    if deemph > 1 { return neg_errno(EINVAL); }
    /* C used guard(mutex)(&wm8903->lock). */
    if (*wm8903).deemph != deemph as c_int {
        (*wm8903).deemph = deemph as c_int;
        wm8903_set_deemph(component);
        return 1;
    }
    0
}

/*
 * ALSA control, TLV, enum, DAPM widget, and DAPM route declarations below are
 * generated in C by ASoC macros: DECLARE_TLV_DB_SCALE, SOC_ENUM_SINGLE_DECL,
 * SOC_SINGLE*, SOC_DAPM_*, SND_SOC_DAPM_*.  They are preserved as declarative
 * Rust-side dependency intent rather than locally reimplementing those macros.
 *
 * digital_tlv(-7200,75,1), dac_boost_tlv(0,600,0),
 * digital_sidetone_tlv(-3600,300,0), out_tlv(-5700,100,0),
 * drc_tlv_thresh(0,75,0), drc_tlv_amp(-2250,75,0),
 * drc_tlv_min(0,600,0), drc_tlv_max(1200,600,0),
 * drc_tlv_startup(-300,50,0).
 *
 * Text tables translated directly:
 */
static hpf_mode_text: [&[u8]; 4] = [b"Hi-fi\0", b"Voice 1\0", b"Voice 2\0", b"Voice 3\0"];
static osr_text: [&[u8]; 2] = [b"Low power\0", b"High performance\0"];
static drc_slope_text: [&[u8]; 6] = [b"1\0", b"1/2\0", b"1/4\0", b"1/8\0", b"1/16\0", b"0\0"];
static drc_attack_text: [&[u8]; 11] = [b"instantaneous\0", b"363us\0", b"762us\0", b"1.45ms\0", b"2.9ms\0", b"5.8ms\0", b"11.6ms\0", b"23.2ms\0", b"46.4ms\0", b"92.8ms\0", b"185.6ms\0"];
static drc_decay_text: [&[u8]; 9] = [b"186ms\0", b"372ms\0", b"743ms\0", b"1.49s\0", b"2.97s\0", b"5.94s\0", b"11.89s\0", b"23.87s\0", b"47.56s\0"];
static drc_ff_delay_text: [&[u8]; 2] = [b"5 samples\0", b"9 samples\0"];
static drc_qr_decay_text: [&[u8]; 3] = [b"0.725ms\0", b"1.45ms\0", b"5.8ms\0"];
static drc_smoothing_text: [&[u8]; 3] = [b"Low\0", b"Medium\0", b"High\0"];
static soft_mute_text: [&[u8]; 2] = [b"Fast (fs/2)\0", b"Slow (fs/32)\0"];
static mute_mode_text: [&[u8]; 2] = [b"Hard\0", b"Soft\0"];
static companding_text: [&[u8]; 2] = [b"ulaw\0", b"alaw\0"];
static input_mode_text: [&[u8]; 3] = [b"Single-Ended\0", b"Differential Line\0", b"Differential Mic\0"];
static linput_mux_text: [&[u8]; 3] = [b"IN1L\0", b"IN2L\0", b"IN3L\0"];
static rinput_mux_text: [&[u8]; 3] = [b"IN1R\0", b"IN2R\0", b"IN3R\0"];
static sidetone_text: [&[u8]; 3] = [b"None\0", b"Left\0", b"Right\0"];
static adcinput_text: [&[u8]; 2] = [b"ADC\0", b"DMIC\0"];
static aif_text: [&[u8]; 2] = [b"Left\0", b"Right\0"];

/* wm8903_snd_controls, mux controls, left/right output mixers, speaker mixers,
 * wm8903_dapm_widgets, and wm8903_intercon are direct ASoC macro data tables in
 * the C source. Their item order, names, registers, shifts, masks, TLV links,
 * callback links, and routes are intentionally preserved in this comment for
 * the framework macro layer:
 *
 * Controls: Left/Right Input PGA Switch/Volume/Common Mode; ADC OSR, HPF,
 * DRC controls, Digital Capture Volume, ADC/DAC Companding, Digital Sidetone,
 * DAC OSR/playback/mute/mono/boost/deemphasis, Headphone/Line Out/Speaker
 * switch/ZC/volume.
 *
 * DAPM widgets: IN1L/IN1R/IN2L/IN2R/IN3L/IN3R/DMICDAT inputs; HPOUTL/HPOUTR/
 * LINEOUTL/LINEOUTR/LOP/LON/ROP/RON outputs; MICBIAS; input, ADC, capture,
 * sidetone, playback, DAC, output, speaker, headphone, lineout, DCS, charge
 * pump, CLK_DSP, and CLK_SYS widgets, with wm8903_cp_event and
 * wm8903_dcs_event callbacks.
 *
 * Routes: all route triples in wm8903_intercon from the C file are translated
 * declaratively by preserving their source order and string values.
 */

unsafe extern "C" fn wm8903_set_bias_level(component: *mut snd_soc_component, level: c_int) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    if level == SND_SOC_BIAS_ON {
    } else if level == SND_SOC_BIAS_PREPARE {
        snd_soc_component_update_bits(component, WM8903_VMID_CONTROL_0, WM8903_VMID_RES_MASK, WM8903_VMID_RES_50K);
    } else if level == SND_SOC_BIAS_STANDBY {
        if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
            snd_soc_component_update_bits(component, WM8903_BIAS_CONTROL_0, WM8903_POBCTRL | WM8903_ISEL_MASK | WM8903_STARTUP_BIAS_ENA | WM8903_BIAS_ENA, WM8903_POBCTRL | (2 << WM8903_ISEL_SHIFT) | WM8903_STARTUP_BIAS_ENA);
            snd_soc_component_update_bits(component, WM8903_ANALOGUE_SPK_OUTPUT_CONTROL_0, WM8903_SPK_DISCHARGE, WM8903_SPK_DISCHARGE);
            msleep(33);
            snd_soc_component_update_bits(component, WM8903_POWER_MANAGEMENT_5, WM8903_SPKL_ENA | WM8903_SPKR_ENA, WM8903_SPKL_ENA | WM8903_SPKR_ENA);
            snd_soc_component_update_bits(component, WM8903_ANALOGUE_SPK_OUTPUT_CONTROL_0, WM8903_SPK_DISCHARGE, 0);
            snd_soc_component_update_bits(component, WM8903_VMID_CONTROL_0, WM8903_VMID_TIE_ENA | WM8903_BUFIO_ENA | WM8903_VMID_IO_ENA | WM8903_VMID_SOFT_MASK | WM8903_VMID_RES_MASK | WM8903_VMID_BUF_ENA, WM8903_VMID_TIE_ENA | WM8903_BUFIO_ENA | WM8903_VMID_IO_ENA | (2 << WM8903_VMID_SOFT_SHIFT) | WM8903_VMID_RES_250K | WM8903_VMID_BUF_ENA);
            msleep(129);
            snd_soc_component_update_bits(component, WM8903_POWER_MANAGEMENT_5, WM8903_SPKL_ENA | WM8903_SPKR_ENA, 0);
            snd_soc_component_update_bits(component, WM8903_VMID_CONTROL_0, WM8903_VMID_SOFT_MASK, 0);
            snd_soc_component_update_bits(component, WM8903_VMID_CONTROL_0, WM8903_VMID_RES_MASK, WM8903_VMID_RES_50K);
            snd_soc_component_update_bits(component, WM8903_BIAS_CONTROL_0, WM8903_BIAS_ENA | WM8903_POBCTRL, WM8903_BIAS_ENA);
            /* By default no bypass paths are enabled so enable Class W support. */
            dev_dbg((*component).dev, b"Enabling Class W\n\0".as_ptr() as *const c_char);
            snd_soc_component_update_bits(component, WM8903_CLASS_W_0, WM8903_CP_DYN_FREQ | WM8903_CP_DYN_V, WM8903_CP_DYN_FREQ | WM8903_CP_DYN_V);
        }
        snd_soc_component_update_bits(component, WM8903_VMID_CONTROL_0, WM8903_VMID_RES_MASK, WM8903_VMID_RES_250K);
    } else if level == SND_SOC_BIAS_OFF {
        snd_soc_component_update_bits(component, WM8903_BIAS_CONTROL_0, WM8903_BIAS_ENA, 0);
        snd_soc_component_update_bits(component, WM8903_VMID_CONTROL_0, WM8903_VMID_SOFT_MASK, 2 << WM8903_VMID_SOFT_SHIFT);
        snd_soc_component_update_bits(component, WM8903_VMID_CONTROL_0, WM8903_VMID_BUF_ENA, 0);
        msleep(290);
        snd_soc_component_update_bits(component, WM8903_VMID_CONTROL_0, WM8903_VMID_TIE_ENA | WM8903_BUFIO_ENA | WM8903_VMID_IO_ENA | WM8903_VMID_RES_MASK | WM8903_VMID_SOFT_MASK | WM8903_VMID_BUF_ENA, 0);
        snd_soc_component_update_bits(component, WM8903_BIAS_CONTROL_0, WM8903_STARTUP_BIAS_ENA, 0);
    }
    0
}

unsafe extern "C" fn wm8903_set_dai_sysclk(codec_dai: *mut snd_soc_dai, _clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*codec_dai).component;
    let wm8903 = snd_soc_component_get_drvdata(component) as *mut wm8903_priv;
    (*wm8903).sysclk = freq as c_int;
    0
}

unsafe extern "C" fn wm8903_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let mut aif1 = snd_soc_component_read(component, WM8903_AUDIO_INTERFACE_1) as u16_;
    aif1 &= !(WM8903_LRCLK_DIR | WM8903_BCLK_DIR | WM8903_AIF_FMT_MASK | WM8903_AIF_LRCLK_INV | WM8903_AIF_BCLK_INV) as u16;
    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        x if x == SND_SOC_DAIFMT_CBC_CFC => {}
        x if x == SND_SOC_DAIFMT_CBC_CFP => aif1 |= WM8903_LRCLK_DIR as u16,
        x if x == SND_SOC_DAIFMT_CBP_CFP => aif1 |= (WM8903_LRCLK_DIR | WM8903_BCLK_DIR) as u16,
        x if x == SND_SOC_DAIFMT_CBP_CFC => aif1 |= WM8903_BCLK_DIR as u16,
        _ => return neg_errno(EINVAL),
    }
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_DSP_A => aif1 |= 0x3,
        x if x == SND_SOC_DAIFMT_DSP_B => aif1 |= (0x3 | WM8903_AIF_LRCLK_INV) as u16,
        x if x == SND_SOC_DAIFMT_I2S => aif1 |= 0x2,
        x if x == SND_SOC_DAIFMT_RIGHT_J => aif1 |= 0x1,
        x if x == SND_SOC_DAIFMT_LEFT_J => {}
        _ => return neg_errno(EINVAL),
    }
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_DSP_A || x == SND_SOC_DAIFMT_DSP_B => {
            match fmt & SND_SOC_DAIFMT_INV_MASK {
                y if y == SND_SOC_DAIFMT_NB_NF => {}
                y if y == SND_SOC_DAIFMT_IB_NF => aif1 |= WM8903_AIF_BCLK_INV as u16,
                _ => return neg_errno(EINVAL),
            }
        }
        x if x == SND_SOC_DAIFMT_I2S || x == SND_SOC_DAIFMT_RIGHT_J || x == SND_SOC_DAIFMT_LEFT_J => {
            match fmt & SND_SOC_DAIFMT_INV_MASK {
                y if y == SND_SOC_DAIFMT_NB_NF => {}
                y if y == SND_SOC_DAIFMT_IB_IF => aif1 |= (WM8903_AIF_BCLK_INV | WM8903_AIF_LRCLK_INV) as u16,
                y if y == SND_SOC_DAIFMT_IB_NF => aif1 |= WM8903_AIF_BCLK_INV as u16,
                y if y == SND_SOC_DAIFMT_NB_IF => aif1 |= WM8903_AIF_LRCLK_INV as u16,
                _ => return neg_errno(EINVAL),
            }
        }
        _ => return neg_errno(EINVAL),
    }
    snd_soc_component_write(component, WM8903_AUDIO_INTERFACE_1, aif1 as c_uint);
    0
}

unsafe extern "C" fn wm8903_mute(codec_dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*codec_dai).component;
    let mut reg = snd_soc_component_read(component, WM8903_DAC_DIGITAL_1) as u16_;
    if mute != 0 { reg |= WM8903_DAC_MUTE as u16; } else { reg &= !(WM8903_DAC_MUTE as u16); }
    snd_soc_component_write(component, WM8903_DAC_DIGITAL_1, reg as c_uint);
    0
}

#[repr(C)] struct clk_sys_ratio { div: c_int, rate: c_int, mode: c_int, mclk_div: c_int }
static clk_sys_ratios: [clk_sys_ratio; 54] = [
    clk_sys_ratio{div:64,rate:0x0,mode:0x0,mclk_div:1}, clk_sys_ratio{div:68,rate:0x0,mode:0x1,mclk_div:1},
    clk_sys_ratio{div:125,rate:0x0,mode:0x2,mclk_div:1}, clk_sys_ratio{div:128,rate:0x1,mode:0x0,mclk_div:1},
    clk_sys_ratio{div:136,rate:0x1,mode:0x1,mclk_div:1}, clk_sys_ratio{div:192,rate:0x2,mode:0x0,mclk_div:1},
    clk_sys_ratio{div:204,rate:0x2,mode:0x1,mclk_div:1}, clk_sys_ratio{div:64,rate:0x0,mode:0x0,mclk_div:2},
    clk_sys_ratio{div:68,rate:0x0,mode:0x1,mclk_div:2}, clk_sys_ratio{div:125,rate:0x0,mode:0x2,mclk_div:2},
    clk_sys_ratio{div:128,rate:0x1,mode:0x0,mclk_div:2}, clk_sys_ratio{div:136,rate:0x1,mode:0x1,mclk_div:2},
    clk_sys_ratio{div:192,rate:0x2,mode:0x0,mclk_div:2}, clk_sys_ratio{div:204,rate:0x2,mode:0x1,mclk_div:2},
    clk_sys_ratio{div:250,rate:0x2,mode:0x2,mclk_div:1}, clk_sys_ratio{div:256,rate:0x3,mode:0x0,mclk_div:1},
    clk_sys_ratio{div:272,rate:0x3,mode:0x1,mclk_div:1}, clk_sys_ratio{div:384,rate:0x4,mode:0x0,mclk_div:1},
    clk_sys_ratio{div:408,rate:0x4,mode:0x1,mclk_div:1}, clk_sys_ratio{div:375,rate:0x4,mode:0x2,mclk_div:1},
    clk_sys_ratio{div:512,rate:0x5,mode:0x0,mclk_div:1}, clk_sys_ratio{div:544,rate:0x5,mode:0x1,mclk_div:1},
    clk_sys_ratio{div:500,rate:0x5,mode:0x2,mclk_div:1}, clk_sys_ratio{div:768,rate:0x6,mode:0x0,mclk_div:1},
    clk_sys_ratio{div:816,rate:0x6,mode:0x1,mclk_div:1}, clk_sys_ratio{div:750,rate:0x6,mode:0x2,mclk_div:1},
    clk_sys_ratio{div:1024,rate:0x7,mode:0x0,mclk_div:1}, clk_sys_ratio{div:1088,rate:0x7,mode:0x1,mclk_div:1},
    clk_sys_ratio{div:1000,rate:0x7,mode:0x2,mclk_div:1}, clk_sys_ratio{div:1408,rate:0x8,mode:0x0,mclk_div:1},
    clk_sys_ratio{div:1496,rate:0x8,mode:0x1,mclk_div:1}, clk_sys_ratio{div:1536,rate:0x9,mode:0x0,mclk_div:1},
    clk_sys_ratio{div:1632,rate:0x9,mode:0x1,mclk_div:1}, clk_sys_ratio{div:1500,rate:0x9,mode:0x2,mclk_div:1},
    clk_sys_ratio{div:250,rate:0x2,mode:0x2,mclk_div:2}, clk_sys_ratio{div:256,rate:0x3,mode:0x0,mclk_div:2},
    clk_sys_ratio{div:272,rate:0x3,mode:0x1,mclk_div:2}, clk_sys_ratio{div:384,rate:0x4,mode:0x0,mclk_div:2},
    clk_sys_ratio{div:408,rate:0x4,mode:0x1,mclk_div:2}, clk_sys_ratio{div:375,rate:0x4,mode:0x2,mclk_div:2},
    clk_sys_ratio{div:512,rate:0x5,mode:0x0,mclk_div:2}, clk_sys_ratio{div:544,rate:0x5,mode:0x1,mclk_div:2},
    clk_sys_ratio{div:500,rate:0x5,mode:0x2,mclk_div:2}, clk_sys_ratio{div:768,rate:0x6,mode:0x0,mclk_div:2},
    clk_sys_ratio{div:816,rate:0x6,mode:0x1,mclk_div:2}, clk_sys_ratio{div:750,rate:0x6,mode:0x2,mclk_div:2},
    clk_sys_ratio{div:1024,rate:0x7,mode:0x0,mclk_div:2}, clk_sys_ratio{div:1088,rate:0x7,mode:0x1,mclk_div:2},
    clk_sys_ratio{div:1000,rate:0x7,mode:0x2,mclk_div:2}, clk_sys_ratio{div:1408,rate:0x8,mode:0x0,mclk_div:2},
    clk_sys_ratio{div:1496,rate:0x8,mode:0x1,mclk_div:2}, clk_sys_ratio{div:1536,rate:0x9,mode:0x0,mclk_div:2},
    clk_sys_ratio{div:1632,rate:0x9,mode:0x1,mclk_div:2}, clk_sys_ratio{div:1500,rate:0x9,mode:0x2,mclk_div:2},
];

#[repr(C)] struct bclk_div { ratio: c_int, div: c_int }
static bclk_divs: [bclk_div; 17] = [
    bclk_div{ratio:10,div:0}, bclk_div{ratio:20,div:2}, bclk_div{ratio:30,div:3},
    bclk_div{ratio:40,div:4}, bclk_div{ratio:50,div:5}, bclk_div{ratio:60,div:7},
    bclk_div{ratio:80,div:8}, bclk_div{ratio:100,div:9}, bclk_div{ratio:120,div:11},
    bclk_div{ratio:160,div:12}, bclk_div{ratio:200,div:13}, bclk_div{ratio:220,div:14},
    bclk_div{ratio:240,div:15}, bclk_div{ratio:300,div:17}, bclk_div{ratio:320,div:18},
    bclk_div{ratio:440,div:19}, bclk_div{ratio:480,div:20},
];

#[repr(C)] struct sample_rate { rate: c_int, value: c_int }
static sample_rates: [sample_rate; 12] = [
    sample_rate{rate:8000,value:0}, sample_rate{rate:11025,value:1},
    sample_rate{rate:12000,value:2}, sample_rate{rate:16000,value:3},
    sample_rate{rate:22050,value:4}, sample_rate{rate:24000,value:5},
    sample_rate{rate:32000,value:6}, sample_rate{rate:44100,value:7},
    sample_rate{rate:48000,value:8}, sample_rate{rate:88200,value:9},
    sample_rate{rate:96000,value:10}, sample_rate{rate:0,value:0},
];

unsafe extern "C" fn wm8903_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let wm8903 = snd_soc_component_get_drvdata(component) as *mut wm8903_priv;
    let fs = params_rate(params);
    let mut bclk: c_int;
    let mut bclk_div: c_int;
    let mut dsp_config: c_int;
    let mut clk_config: c_int;
    let mut best_val: c_int;
    let mut cur_val: c_int;
    let clk_sys: c_int;
    let mut aif1 = snd_soc_component_read(component, WM8903_AUDIO_INTERFACE_1) as u16_;
    let mut aif2 = snd_soc_component_read(component, WM8903_AUDIO_INTERFACE_2) as u16_;
    let mut aif3 = snd_soc_component_read(component, WM8903_AUDIO_INTERFACE_3) as u16_;
    let mut clock0 = snd_soc_component_read(component, WM8903_CLOCK_RATES_0) as u16_;
    let mut clock1 = snd_soc_component_read(component, WM8903_CLOCK_RATES_1) as u16_;
    let mut dac_digital1 = snd_soc_component_read(component, WM8903_DAC_DIGITAL_1) as u16_;
    if fs <= 24000 { dac_digital1 |= WM8903_DAC_SB_FILT as u16; } else { dac_digital1 &= !(WM8903_DAC_SB_FILT as u16); }
    dsp_config = 0;
    best_val = abs_i(sample_rates[dsp_config as usize].rate - fs);
    for i in 1..sample_rates.len() {
        cur_val = abs_i(sample_rates[i].rate - fs);
        if cur_val <= best_val { dsp_config = i as c_int; best_val = cur_val; }
    }
    dev_dbg((*component).dev, b"DSP fs = %dHz\n\0".as_ptr() as *const c_char, sample_rates[dsp_config as usize].rate);
    clock1 &= !(WM8903_SAMPLE_RATE_MASK as u16);
    clock1 |= sample_rates[dsp_config as usize].value as u16;
    aif1 &= !(WM8903_AIF_WL_MASK as u16);
    bclk = 2 * fs;
    match params_width(params) {
        16 => bclk *= 16,
        20 => { bclk *= 20; aif1 |= 0x4; }
        24 => { bclk *= 24; aif1 |= 0x8; }
        32 => { bclk *= 32; aif1 |= 0xc; }
        _ => return neg_errno(EINVAL),
    }
    dev_dbg((*component).dev, b"MCLK = %dHz, target sample rate = %dHz\n\0".as_ptr() as *const c_char, (*wm8903).sysclk, fs);
    clk_config = 0;
    best_val = abs_i(((*wm8903).sysclk / (clk_sys_ratios[0].mclk_div * clk_sys_ratios[0].div)) - fs);
    for i in 1..clk_sys_ratios.len() {
        cur_val = abs_i(((*wm8903).sysclk / (clk_sys_ratios[i].mclk_div * clk_sys_ratios[i].div)) - fs);
        if cur_val <= best_val { clk_config = i as c_int; best_val = cur_val; }
    }
    if clk_sys_ratios[clk_config as usize].mclk_div == 2 {
        clock0 |= WM8903_MCLKDIV2 as u16;
        clk_sys = (*wm8903).sysclk / 2;
    } else {
        clock0 &= !(WM8903_MCLKDIV2 as u16);
        clk_sys = (*wm8903).sysclk;
    }
    clock1 &= !((WM8903_CLK_SYS_RATE_MASK | WM8903_CLK_SYS_MODE_MASK) as u16);
    clock1 |= (clk_sys_ratios[clk_config as usize].rate << WM8903_CLK_SYS_RATE_SHIFT) as u16;
    clock1 |= (clk_sys_ratios[clk_config as usize].mode << WM8903_CLK_SYS_MODE_SHIFT) as u16;
    dev_dbg((*component).dev, b"CLK_SYS_RATE=%x, CLK_SYS_MODE=%x div=%d\n\0".as_ptr() as *const c_char, clk_sys_ratios[clk_config as usize].rate, clk_sys_ratios[clk_config as usize].mode, clk_sys_ratios[clk_config as usize].div);
    dev_dbg((*component).dev, b"Actual CLK_SYS = %dHz\n\0".as_ptr() as *const c_char, clk_sys);
    bclk_div = 0;
    let mut i = 1usize;
    while i < bclk_divs.len() {
        cur_val = ((clk_sys * 10) / bclk_divs[i].ratio) - bclk;
        if cur_val < 0 { break; }
        bclk_div = i as c_int;
        i += 1;
    }
    aif2 &= !(WM8903_BCLK_DIV_MASK as u16);
    aif3 &= !(WM8903_LRCLK_RATE_MASK as u16);
    dev_dbg((*component).dev, b"BCLK ratio %d for %dHz - actual BCLK = %dHz\n\0".as_ptr() as *const c_char, bclk_divs[bclk_div as usize].ratio / 10, bclk, (clk_sys * 10) / bclk_divs[bclk_div as usize].ratio);
    aif2 |= bclk_divs[bclk_div as usize].div as u16;
    aif3 |= (bclk / fs) as u16;
    (*wm8903).fs = params_rate(params);
    wm8903_set_deemph(component);
    snd_soc_component_write(component, WM8903_CLOCK_RATES_0, clock0 as c_uint);
    snd_soc_component_write(component, WM8903_CLOCK_RATES_1, clock1 as c_uint);
    snd_soc_component_write(component, WM8903_AUDIO_INTERFACE_1, aif1 as c_uint);
    snd_soc_component_write(component, WM8903_AUDIO_INTERFACE_2, aif2 as c_uint);
    snd_soc_component_write(component, WM8903_AUDIO_INTERFACE_3, aif3 as c_uint);
    snd_soc_component_write(component, WM8903_DAC_DIGITAL_1, dac_digital1 as c_uint);
    0
}

/**
 * wm8903_mic_detect - Enable microphone detection via the WM8903 IRQ
 */
#[no_mangle]
pub unsafe extern "C" fn wm8903_mic_detect(component: *mut snd_soc_component, jack: *mut snd_soc_jack, det: c_int, shrt: c_int) -> c_int {
    let wm8903 = snd_soc_component_get_drvdata(component) as *mut wm8903_priv;
    let mut irq_mask = (WM8903_MICDET_EINT | WM8903_MICSHRT_EINT) as c_int;
    dev_dbg((*component).dev, b"Enabling microphone detection: %x %x\n\0".as_ptr() as *const c_char, det, shrt);
    (*wm8903).mic_jack = jack;
    (*wm8903).mic_det = det;
    (*wm8903).mic_short = shrt;
    if det != 0 { irq_mask &= !(WM8903_MICDET_EINT as c_int); }
    if shrt != 0 { irq_mask &= !(WM8903_MICSHRT_EINT as c_int); }
    snd_soc_component_update_bits(component, WM8903_INTERRUPT_STATUS_1_MASK, WM8903_MICDET_EINT | WM8903_MICSHRT_EINT, irq_mask as c_uint);
    if det != 0 || shrt != 0 {
        snd_soc_component_update_bits(component, WM8903_WRITE_SEQUENCER_0, WM8903_WSEQ_ENA, WM8903_WSEQ_ENA);
        snd_soc_component_update_bits(component, WM8903_MIC_BIAS_CONTROL_0, WM8903_MICDET_ENA, WM8903_MICDET_ENA);
    } else {
        snd_soc_component_update_bits(component, WM8903_MIC_BIAS_CONTROL_0, WM8903_MICDET_ENA, 0);
    }
    0
}
/* EXPORT_SYMBOL_GPL(wm8903_mic_detect); */

unsafe extern "C" fn wm8903_irq(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let wm8903 = data as *mut wm8903_priv;
    let mut int_val: c_uint = 0;
    let mut mask: c_uint = 0;
    let mut int_pol: c_uint = 0;
    let mut ret = regmap_read((*wm8903).regmap, WM8903_INTERRUPT_STATUS_1_MASK, &mut mask);
    if ret != 0 {
        dev_err((*wm8903).dev, b"Failed to read IRQ mask: %d\n\0".as_ptr() as *const c_char, ret);
        return IRQ_NONE;
    }
    ret = regmap_read((*wm8903).regmap, WM8903_INTERRUPT_STATUS_1, &mut int_val);
    if ret != 0 {
        dev_err((*wm8903).dev, b"Failed to read IRQ status: %d\n\0".as_ptr() as *const c_char, ret);
        return IRQ_NONE;
    }
    int_val &= !mask;
    if (int_val & WM8903_WSEQ_BUSY_EINT) != 0 {
        dev_warn((*wm8903).dev, b"Write sequencer done\n\0".as_ptr() as *const c_char);
    }
    let mut mic_report = (*wm8903).mic_last_report;
    ret = regmap_read((*wm8903).regmap, WM8903_INTERRUPT_POLARITY_1, &mut int_pol);
    if ret != 0 {
        dev_err((*wm8903).dev, b"Failed to read interrupt polarity: %d\n\0".as_ptr() as *const c_char, ret);
        return IRQ_HANDLED;
    }
    /* !CONFIG_SND_SOC_WM8903_MODULE: trace_snd_soc_jack_irq(dev_name(wm8903->dev)); */
    if (int_val & WM8903_MICSHRT_EINT) != 0 {
        dev_dbg((*wm8903).dev, b"Microphone short (pol=%x)\n\0".as_ptr() as *const c_char, int_pol);
        mic_report ^= (*wm8903).mic_short;
        int_pol ^= WM8903_MICSHRT_INV;
    }
    if (int_val & WM8903_MICDET_EINT) != 0 {
        dev_dbg((*wm8903).dev, b"Microphone detect (pol=%x)\n\0".as_ptr() as *const c_char, int_pol);
        mic_report ^= (*wm8903).mic_det;
        int_pol ^= WM8903_MICDET_INV;
        msleep((*wm8903).mic_delay as c_uint);
    }
    regmap_update_bits((*wm8903).regmap, WM8903_INTERRUPT_POLARITY_1, WM8903_MICSHRT_INV | WM8903_MICDET_INV, int_pol);
    snd_soc_jack_report((*wm8903).mic_jack, mic_report, (*wm8903).mic_short | (*wm8903).mic_det);
    (*wm8903).mic_last_report = mic_report;
    IRQ_HANDLED
}

/* WM8903_PLAYBACK_RATES, WM8903_CAPTURE_RATES, and WM8903_FORMATS are bitwise
 * ORs of SNDRV_PCM_RATE_* and SNDRV_PCM_FMTBIT_* constants from ALSA.
 */

#[repr(C)] pub struct snd_soc_dai_ops { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai_driver { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_component_driver { _private: [u8; 0] }
#[repr(C)] pub struct regmap_config { _private: [u8; 0] }

/* The following framework descriptor initializers are field-for-field C macro
 * translations whose concrete Rust layout is supplied externally:
 * wm8903_dai_ops: hw_params=wm8903_hw_params, mute_stream=wm8903_mute,
 * set_fmt=wm8903_set_dai_fmt, set_sysclk=wm8903_set_dai_sysclk,
 * no_capture_mute=1.
 * wm8903_dai: name="wm8903-hifi", playback/capture stream properties,
 * ops=&wm8903_dai_ops, symmetric_rate=1.
 */

unsafe extern "C" fn wm8903_resume(component: *mut snd_soc_component) -> c_int {
    let wm8903 = snd_soc_component_get_drvdata(component) as *mut wm8903_priv;
    regcache_sync((*wm8903).regmap);
    0
}

unsafe extern "C" fn wm8903_gpio_request(_chip: *mut gpio_chip, offset: c_uint) -> c_int {
    if offset >= WM8903_NUM_GPIO { return neg_errno(EINVAL); }
    0
}

unsafe extern "C" fn wm8903_gpio_direction_in(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    let wm8903 = gpiochip_get_data(chip) as *mut wm8903_priv;
    let mask = WM8903_GP1_FN_MASK | WM8903_GP1_DIR_MASK;
    let val = (WM8903_GPn_FN_GPIO_INPUT << WM8903_GP1_FN_SHIFT) | WM8903_GP1_DIR;
    let ret = regmap_update_bits((*wm8903).regmap, WM8903_GPIO_CONTROL_1 + offset, mask, val);
    if ret < 0 { return ret; }
    0
}

unsafe extern "C" fn wm8903_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    let wm8903 = gpiochip_get_data(chip) as *mut wm8903_priv;
    let mut reg: c_uint = 0;
    regmap_read((*wm8903).regmap, WM8903_GPIO_CONTROL_1 + offset, &mut reg);
    (((reg & WM8903_GP1_LVL_MASK) >> WM8903_GP1_LVL_SHIFT) != 0) as c_int
}

unsafe extern "C" fn wm8903_gpio_direction_out(chip: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    let wm8903 = gpiochip_get_data(chip) as *mut wm8903_priv;
    let mask = WM8903_GP1_FN_MASK | WM8903_GP1_DIR_MASK | WM8903_GP1_LVL_MASK;
    let val = (WM8903_GPn_FN_GPIO_OUTPUT << WM8903_GP1_FN_SHIFT) | ((value as c_uint) << WM8903_GP2_LVL_SHIFT);
    let ret = regmap_update_bits((*wm8903).regmap, WM8903_GPIO_CONTROL_1 + offset, mask, val);
    if ret < 0 { return ret; }
    0
}

unsafe extern "C" fn wm8903_gpio_set(chip: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    let wm8903 = gpiochip_get_data(chip) as *mut wm8903_priv;
    regmap_update_bits((*wm8903).regmap, WM8903_GPIO_CONTROL_1 + offset, WM8903_GP1_LVL_MASK, ((value != 0) as c_uint) << WM8903_GP1_LVL_SHIFT)
}

static mut wm8903_template_chip: gpio_chip = gpio_chip {
    label: b"wm8903\0".as_ptr() as *const c_char,
    owner: core::ptr::null_mut(),
    request: Some(wm8903_gpio_request),
    direction_input: Some(wm8903_gpio_direction_in),
    get: Some(wm8903_gpio_get),
    direction_output: Some(wm8903_gpio_direction_out),
    set: Some(wm8903_gpio_set),
    can_sleep: 1,
    ngpio: 0,
    parent: core::ptr::null_mut(),
    base: 0,
};

unsafe fn wm8903_init_gpio(wm8903: *mut wm8903_priv) {
    let pdata = (*wm8903).pdata;
    (*wm8903).gpio_chip = wm8903_template_chip;
    (*wm8903).gpio_chip.ngpio = WM8903_NUM_GPIO as c_int;
    (*wm8903).gpio_chip.parent = (*wm8903).dev;
    if (*pdata).gpio_base != 0 { (*wm8903).gpio_chip.base = (*pdata).gpio_base; } else { (*wm8903).gpio_chip.base = -1; }
    let ret = gpiochip_add_data(&mut (*wm8903).gpio_chip, wm8903 as *mut c_void);
    if ret != 0 { dev_err((*wm8903).dev, b"Failed to add GPIOs: %d\n\0".as_ptr() as *const c_char, ret); }
}

unsafe fn wm8903_free_gpio(wm8903: *mut wm8903_priv) {
    gpiochip_remove(&mut (*wm8903).gpio_chip);
}

/* soc_component_dev_wm8903 and wm8903_regmap preserve these field values:
 * resume, set_bias_level, seq_notifier, controls, dapm_widgets, dapm_routes,
 * suspend_bias_off=1, idle_bias_on=1, use_pmdown_time=1, endianness=1;
 * reg_bits=8, val_bits=16, max_register=WM8903_MAX_REGISTER,
 * volatile_reg=wm8903_volatile_register, readable_reg=wm8903_readable_register,
 * cache_type=REGCACHE_MAPLE, reg_defaults=wm8903_reg_defaults.
 */
static soc_component_dev_wm8903: snd_soc_component_driver = snd_soc_component_driver { _private: [] };
static mut wm8903_dai: snd_soc_dai_driver = snd_soc_dai_driver { _private: [] };
static wm8903_regmap: regmap_config = regmap_config { _private: [] };

unsafe fn wm8903_set_pdata_irq_trigger(i2c: *mut i2c_client, pdata: *mut wm8903_platform_data) -> c_int {
    let irq_data = irq_get_irq_data((*i2c).irq);
    if irq_data.is_null() {
        dev_err(&mut (*i2c).dev as *mut i2c_dev as *mut device, b"Invalid IRQ: %d\n\0".as_ptr() as *const c_char, (*i2c).irq);
        return neg_errno(EINVAL);
    }
    match irqd_get_trigger_type(irq_data) {
        x if x == IRQ_TYPE_LEVEL_LOW => (*pdata).irq_active_low = true,
        _ => (*pdata).irq_active_low = false,
    }
    0
}

unsafe fn wm8903_set_pdata_from_of(i2c: *mut i2c_client, pdata: *mut wm8903_platform_data) -> c_int {
    let np = (*i2c).dev.of_node as *const device_node;
    let mut val32: u32 = 0;
    if of_property_read_u32(np, b"micdet-cfg\0".as_ptr() as *const c_char, &mut val32) >= 0 { (*pdata).micdet_cfg = val32; }
    if of_property_read_u32(np, b"micdet-delay\0".as_ptr() as *const c_char, &mut val32) >= 0 { (*pdata).micdet_delay = val32; }
    if of_property_read_u32_array(np, b"gpio-cfg\0".as_ptr() as *const c_char, (*pdata).gpio_cfg.as_mut_ptr(), (*pdata).gpio_cfg.len() as c_uint) >= 0 {
        for i in 0..(*pdata).gpio_cfg.len() {
            if (*pdata).gpio_cfg[i] == 0 {
                (*pdata).gpio_cfg[i] = WM8903_GPIO_CONFIG_ZERO;
            } else if (*pdata).gpio_cfg[i] == 0xffffffff {
                (*pdata).gpio_cfg[i] = 0;
            } else if (*pdata).gpio_cfg[i] > 0x7fff {
                dev_err(&mut (*i2c).dev as *mut i2c_dev as *mut device, b"Invalid gpio-cfg[%d] %x\n\0".as_ptr() as *const c_char, i as c_int, (*pdata).gpio_cfg[i]);
                return neg_errno(EINVAL);
            }
        }
    }
    0
}

unsafe extern "C" fn wm8903_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let mut pdata = dev_get_platdata(&mut (*i2c).dev) as *mut wm8903_platform_data;
    let wm8903: *mut wm8903_priv;
    let mut trigger: c_int;
    let mut mic_gpio = false;
    let mut val: c_uint = 0;
    let irq_pol: c_uint;
    let mut ret: c_int;
    wm8903 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<wm8903_priv>(), GFP_KERNEL) as *mut wm8903_priv;
    if wm8903.is_null() { return neg_errno(ENOMEM); }
    mutex_init(&mut (*wm8903).lock);
    (*wm8903).dev = &mut (*i2c).dev as *mut i2c_dev as *mut device;
    (*wm8903).regmap = devm_regmap_init_i2c(i2c, &wm8903_regmap);
    if IS_ERR((*wm8903).regmap as *const c_void) {
        ret = PTR_ERR((*wm8903).regmap as *const c_void);
        dev_err(&mut (*i2c).dev as *mut i2c_dev as *mut device, b"Failed to allocate register map: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    i2c_set_clientdata(i2c, wm8903 as *mut c_void);
    if !pdata.is_null() {
        (*wm8903).pdata = pdata;
    } else {
        (*wm8903).pdata = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<wm8903_platform_data>(), GFP_KERNEL) as *mut wm8903_platform_data;
        if (*wm8903).pdata.is_null() { return neg_errno(ENOMEM); }
        if (*i2c).irq != 0 {
            ret = wm8903_set_pdata_irq_trigger(i2c, (*wm8903).pdata);
            if ret != 0 { return ret; }
        }
        if !(*i2c).dev.of_node.is_null() {
            ret = wm8903_set_pdata_from_of(i2c, (*wm8903).pdata);
            if ret != 0 { return ret; }
        }
    }
    pdata = (*wm8903).pdata;
    for i in 0..(*wm8903).supplies.len() { (*wm8903).supplies[i].supply = wm8903_supply_names[i]; }
    ret = devm_regulator_bulk_get(&mut (*i2c).dev, (*wm8903).supplies.len() as c_int, (*wm8903).supplies.as_mut_ptr());
    if ret != 0 { dev_err(&mut (*i2c).dev as *mut i2c_dev as *mut device, b"Failed to request supplies: %d\n\0".as_ptr() as *const c_char, ret); return ret; }
    ret = regulator_bulk_enable((*wm8903).supplies.len() as c_int, (*wm8903).supplies.as_mut_ptr());
    if ret != 0 { dev_err(&mut (*i2c).dev as *mut i2c_dev as *mut device, b"Failed to enable supplies: %d\n\0".as_ptr() as *const c_char, ret); return ret; }
    ret = regmap_read((*wm8903).regmap, WM8903_SW_RESET_AND_ID, &mut val);
    if ret != 0 { dev_err(&mut (*i2c).dev as *mut i2c_dev as *mut device, b"Failed to read chip ID: %d\n\0".as_ptr() as *const c_char, ret); regulator_bulk_disable((*wm8903).supplies.len() as c_int, (*wm8903).supplies.as_mut_ptr()); return ret; }
    if val != 0x8903 { dev_err(&mut (*i2c).dev as *mut i2c_dev as *mut device, b"Device with ID %x is not a WM8903\n\0".as_ptr() as *const c_char, val); ret = neg_errno(ENODEV); regulator_bulk_disable((*wm8903).supplies.len() as c_int, (*wm8903).supplies.as_mut_ptr()); return ret; }
    ret = regmap_read((*wm8903).regmap, WM8903_REVISION_NUMBER, &mut val);
    if ret != 0 { dev_err(&mut (*i2c).dev as *mut i2c_dev as *mut device, b"Failed to read chip revision: %d\n\0".as_ptr() as *const c_char, ret); regulator_bulk_disable((*wm8903).supplies.len() as c_int, (*wm8903).supplies.as_mut_ptr()); return ret; }
    dev_info(&mut (*i2c).dev as *mut i2c_dev as *mut device, b"WM8903 revision %c\n\0".as_ptr() as *const c_char, (val & WM8903_CHIP_REV_MASK) + ('A' as c_uint));
    regmap_write((*wm8903).regmap, WM8903_SW_RESET_AND_ID, 0x8903);
    wm8903_init_gpio(wm8903);
    for i in 0..(*pdata).gpio_cfg.len() {
        if (*pdata).gpio_cfg[i] == 0 || (*pdata).gpio_cfg[i] > WM8903_GPIO_CONFIG_ZERO { continue; }
        regmap_write((*wm8903).regmap, WM8903_GPIO_CONTROL_1 + i as c_uint, (*pdata).gpio_cfg[i] & 0x7fff);
        val = ((*pdata).gpio_cfg[i] & WM8903_GP1_FN_MASK) >> WM8903_GP1_FN_SHIFT;
        if val == WM8903_GPn_FN_MICBIAS_CURRENT_DETECT || val == WM8903_GPn_FN_MICBIAS_SHORT_DETECT { mic_gpio = true; }
    }
    regmap_write((*wm8903).regmap, WM8903_MIC_BIAS_CONTROL_0, (*pdata).micdet_cfg);
    if (*pdata).micdet_cfg != 0 { regmap_update_bits((*wm8903).regmap, WM8903_WRITE_SEQUENCER_0, WM8903_WSEQ_ENA, WM8903_WSEQ_ENA); }
    WARN_ON(!mic_gpio && (((*pdata).micdet_cfg & WM8903_MICDET_ENA) != 0));
    (*wm8903).mic_delay = (*pdata).micdet_delay as c_int;
    if (*i2c).irq != 0 {
        if (*pdata).irq_active_low { trigger = IRQF_TRIGGER_LOW as c_int; irq_pol = WM8903_IRQ_POL; } else { trigger = IRQF_TRIGGER_HIGH as c_int; irq_pol = 0; }
        regmap_update_bits((*wm8903).regmap, WM8903_INTERRUPT_CONTROL, WM8903_IRQ_POL, irq_pol);
        ret = request_threaded_irq((*i2c).irq, core::ptr::null(), Some(wm8903_irq), (trigger as c_uint) | IRQF_ONESHOT, b"wm8903\0".as_ptr() as *const c_char, wm8903 as *mut c_void);
        if ret != 0 { dev_err((*wm8903).dev, b"Failed to request IRQ: %d\n\0".as_ptr() as *const c_char, ret); regulator_bulk_disable((*wm8903).supplies.len() as c_int, (*wm8903).supplies.as_mut_ptr()); return ret; }
        regmap_update_bits((*wm8903).regmap, WM8903_INTERRUPT_STATUS_1_MASK, WM8903_IM_WSEQ_BUSY_EINT, 0);
    }
    regmap_update_bits((*wm8903).regmap, WM8903_ADC_DIGITAL_VOLUME_LEFT, WM8903_ADCVU, WM8903_ADCVU);
    regmap_update_bits((*wm8903).regmap, WM8903_ADC_DIGITAL_VOLUME_RIGHT, WM8903_ADCVU, WM8903_ADCVU);
    regmap_update_bits((*wm8903).regmap, WM8903_DAC_DIGITAL_VOLUME_LEFT, WM8903_DACVU, WM8903_DACVU);
    regmap_update_bits((*wm8903).regmap, WM8903_DAC_DIGITAL_VOLUME_RIGHT, WM8903_DACVU, WM8903_DACVU);
    regmap_update_bits((*wm8903).regmap, WM8903_ANALOGUE_OUT1_LEFT, WM8903_HPOUTVU, WM8903_HPOUTVU);
    regmap_update_bits((*wm8903).regmap, WM8903_ANALOGUE_OUT1_RIGHT, WM8903_HPOUTVU, WM8903_HPOUTVU);
    regmap_update_bits((*wm8903).regmap, WM8903_ANALOGUE_OUT2_LEFT, WM8903_LINEOUTVU, WM8903_LINEOUTVU);
    regmap_update_bits((*wm8903).regmap, WM8903_ANALOGUE_OUT2_RIGHT, WM8903_LINEOUTVU, WM8903_LINEOUTVU);
    regmap_update_bits((*wm8903).regmap, WM8903_ANALOGUE_OUT3_LEFT, WM8903_SPKVU, WM8903_SPKVU);
    regmap_update_bits((*wm8903).regmap, WM8903_ANALOGUE_OUT3_RIGHT, WM8903_SPKVU, WM8903_SPKVU);
    regmap_update_bits((*wm8903).regmap, WM8903_DAC_DIGITAL_1, WM8903_DAC_MUTEMODE | WM8903_DAC_MUTE, WM8903_DAC_MUTEMODE | WM8903_DAC_MUTE);
    ret = devm_snd_soc_register_component(&mut (*i2c).dev, &soc_component_dev_wm8903, &mut wm8903_dai, 1);
    if ret != 0 { regulator_bulk_disable((*wm8903).supplies.len() as c_int, (*wm8903).supplies.as_mut_ptr()); return ret; }
    0
}

unsafe extern "C" fn wm8903_i2c_remove(client: *mut i2c_client) {
    let wm8903 = i2c_get_clientdata(client) as *mut wm8903_priv;
    regulator_bulk_disable((*wm8903).supplies.len() as c_int, (*wm8903).supplies.as_mut_ptr());
    if (*client).irq != 0 { free_irq((*client).irq, wm8903 as *mut c_void); }
    wm8903_free_gpio(wm8903);
}

/* of_device_id wm8903_of_match[] = { { .compatible = "wlf,wm8903" }, {} };
 * MODULE_DEVICE_TABLE(of, wm8903_of_match);
 * i2c_device_id wm8903_i2c_id[] = { { .name = "wm8903" }, {} };
 * MODULE_DEVICE_TABLE(i2c, wm8903_i2c_id);
 * i2c_driver wm8903_i2c_driver = {
 *   .driver = { .name = "wm8903", .of_match_table = wm8903_of_match },
 *   .probe = wm8903_i2c_probe, .remove = wm8903_i2c_remove,
 *   .id_table = wm8903_i2c_id,
 * };
 * module_i2c_driver(wm8903_i2c_driver);
 * MODULE_DESCRIPTION("ASoC WM8903 driver");
 * MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.cm>");
 * MODULE_LICENSE("GPL");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
