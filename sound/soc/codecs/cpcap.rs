// SPDX-License-Identifier: GPL-2.0
/*
 * ALSA SoC CPCAP codec driver
 *
 * Copyright (C) 2017 - 2018 Sebastian Reichel <sre@kernel.org>
 *
 * Very loosely based on original driver from Motorola:
 * Copyright (C) 2007 - 2009 Motorola, Inc.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

type u16 = ::core::ffi::c_ushort;
type u32 = ::core::ffi::c_uint;
type c_int = ::core::ffi::c_int;
type c_uint = ::core::ffi::c_uint;
type c_char = ::core::ffi::c_char;
type c_void = ::core::ffi::c_void;
type bool_ = bool;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regulator {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    pub parent: *mut device,
    pub of_node: *mut device_node,
}
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
    pub card: *mut snd_soc_card,
}
#[repr(C)]
pub struct snd_soc_card {
    _private: [u8; 0],
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
    pub private_value: usize,
}
#[repr(C)]
pub struct snd_jack {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_jack {
    pub jack: *mut snd_jack,
    pub status: c_int,
}
#[repr(C)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}
#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub enumerated: ::core::mem::ManuallyDrop<snd_ctl_elem_value_enumerated>,
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}
#[repr(C)]
pub struct soc_enum {
    pub reg: c_uint,
    pub shift_l: c_uint,
    pub shift_r: c_uint,
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
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}
#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dai_link: *mut snd_soc_dai_link,
}
#[repr(C)]
pub struct snd_soc_dai_link {
    pub dai_fmt: c_uint,
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
pub struct snd_soc_dai_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub no_capture_mute: c_uint,
}
#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    pub id: c_int,
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_desc,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}
#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
}
#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: platform_driver_driver,
}

pub type irqreturn_t = c_int;
pub type snd_soc_bias_level = c_uint;

extern "C" {
    static CPCAP_REG_INTS1: c_uint;
    static CPCAP_REG_INTS2: c_uint;
    static CPCAP_REG_CC: c_uint;
    static CPCAP_REG_CDI: c_uint;
    static CPCAP_REG_SDAC: c_uint;
    static CPCAP_REG_SDACDI: c_uint;
    static CPCAP_REG_TXI: c_uint;
    static CPCAP_REG_TXMP: c_uint;
    static CPCAP_REG_RXOA: c_uint;
    static CPCAP_REG_RXVC: c_uint;
    static CPCAP_REG_RXCOA: c_uint;
    static CPCAP_REG_RXSDOA: c_uint;
    static CPCAP_REG_RXEPOA: c_uint;
    static CPCAP_REG_A2LA: c_uint;
    static CPCAP_REG_TEST: c_uint;
    static CPCAP_REG_ST_TEST1: c_uint;
    static CPCAP_VENDOR_ST: u16;
    static EINVAL: c_int;
    static EIO: c_int;
    static ENOMEM: c_int;
    static ENODEV: c_int;
    static GFP_KERNEL: c_uint;
    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_POST_PMU: c_int;
    static SND_SOC_NOPM: c_int;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SNDRV_PCM_RATE_8000_48000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FORMAT_S24_LE: u64;
    static SND_JACK_HEADSET: c_int;
    static SND_JACK_HEADPHONE: c_int;
    static SND_JACK_MICROPHONE: c_int;
    static SND_JACK_BTN_0: c_int;
    static KEY_MEDIA: c_uint;
    static REGULATOR_MODE_NORMAL: c_uint;
    static REGULATOR_MODE_STANDBY: c_uint;
    static IRQ_HANDLED: irqreturn_t;
    static IRQF_TRIGGER_RISING: c_uint;
    static IRQF_TRIGGER_FALLING: c_uint;
    static IRQF_ONESHOT: c_uint;
    static SND_SOC_BIAS_OFF: snd_soc_bias_level;
    static SND_SOC_BIAS_PREPARE: snd_soc_bias_level;
    static SND_SOC_BIAS_STANDBY: snd_soc_bias_level;
    static SND_SOC_BIAS_ON: snd_soc_bias_level;

    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut cpcap_audio;
    fn snd_soc_component_set_drvdata(component: *mut snd_soc_component, data: *mut cpcap_audio);
    fn snd_soc_dapm_kcontrol_to_component(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_dapm_kcontrol_to_dapm(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_mux_update_power(dapm: *mut snd_soc_dapm_context, kcontrol: *mut snd_kcontrol, muxval: c_uint, e: *mut soc_enum, update: *mut c_void);
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_int) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_test_bits(map: *mut regmap, reg: c_uint, bits: c_uint) -> c_int;
    fn msleep(ms: c_uint);
    fn mdelay(ms: c_uint);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_runtime_set_dai_fmt(rtd: *mut snd_soc_pcm_runtime, fmt: c_uint) -> c_int;
    fn to_platform_device(dev: *mut device) -> *mut platform_device;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regulator_get(dev: *mut device, id: *const c_char) -> *mut regulator;
    fn snd_soc_card_jack_new(card: *mut snd_soc_card, id: *const c_char, type_: c_int, jack: *mut snd_soc_jack) -> c_int;
    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_int, keytype: c_uint) -> c_int;
    fn dev_get_regmap(dev: *mut device, name: *const c_char) -> *mut regmap;
    fn snd_soc_component_init_regmap(component: *mut snd_soc_component, regmap: *mut regmap);
    fn cpcap_get_vendor(dev: *mut device, regmap: *mut regmap, vendor: *mut u16) -> c_int;
    fn platform_get_irq_byname(pdev: *mut platform_device, name: *const c_char) -> c_int;
    fn devm_request_threaded_irq(dev: *mut device, irq: c_int, handler: *mut c_void, thread_fn: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>, flags: c_uint, name: *const c_char, data: *mut c_void) -> c_int;
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
    fn regulator_set_mode(regulator: *mut regulator, mode: c_uint) -> c_int;
    fn enable_irq_wake(irq: c_int) -> c_int;
    fn disable_irq_wake(irq: c_int) -> c_int;
    fn of_get_child_by_name(node: *mut device_node, name: *const c_char) -> *mut device_node;
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

const fn BIT(n: c_uint) -> c_uint {
    1u32 << n
}

/* Register 8 - CPCAP_REG_INTS1  --- Interrupt Sense 1 */
const CPCAP_BIT_HS_S: c_uint = 9; /* Headset */
const CPCAP_BIT_MB2_S: c_uint = 10; /* Mic Bias2 */

/* Register 9 - CPCAP_REG_INTS2   --- Interrupt Sense 2 */
const CPCAP_BIT_PTT_S: c_uint = 11; /* Push To Talk */

/* Register 513 CPCAP_REG_CC     --- CODEC */
const CPCAP_BIT_CDC_CLK2: c_uint = 15;
const CPCAP_BIT_CDC_CLK1: c_uint = 14;
const CPCAP_BIT_CDC_CLK0: c_uint = 13;
const CPCAP_BIT_CDC_SR3: c_uint = 12;
const CPCAP_BIT_CDC_SR2: c_uint = 11;
const CPCAP_BIT_CDC_SR1: c_uint = 10;
const CPCAP_BIT_CDC_SR0: c_uint = 9;
const CPCAP_BIT_CDC_CLOCK_TREE_RESET: c_uint = 8;
const CPCAP_BIT_MIC2_CDC_EN: c_uint = 7;
const CPCAP_BIT_CDC_EN_RX: c_uint = 6;
const CPCAP_BIT_DF_RESET: c_uint = 5;
const CPCAP_BIT_MIC1_CDC_EN: c_uint = 4;
const CPCAP_BIT_AUDOHPF_1: c_uint = 3;
const CPCAP_BIT_AUDOHPF_0: c_uint = 2;
const CPCAP_BIT_AUDIHPF_1: c_uint = 1;
const CPCAP_BIT_AUDIHPF_0: c_uint = 0;

/* Register 514 CPCAP_REG_CDI    --- CODEC Digital Audio Interface */
const CPCAP_BIT_CDC_PLL_SEL: c_uint = 15;
const CPCAP_BIT_CLK_IN_SEL: c_uint = 13;
const CPCAP_BIT_DIG_AUD_IN: c_uint = 12;
const CPCAP_BIT_CDC_CLK_EN: c_uint = 11;
const CPCAP_BIT_CDC_DIG_AUD_FS1: c_uint = 10;
const CPCAP_BIT_CDC_DIG_AUD_FS0: c_uint = 9;
const CPCAP_BIT_MIC2_TIMESLOT2: c_uint = 8;
const CPCAP_BIT_MIC2_TIMESLOT1: c_uint = 7;
const CPCAP_BIT_MIC2_TIMESLOT0: c_uint = 6;
const CPCAP_BIT_MIC1_RX_TIMESLOT2: c_uint = 5;
const CPCAP_BIT_MIC1_RX_TIMESLOT1: c_uint = 4;
const CPCAP_BIT_MIC1_RX_TIMESLOT0: c_uint = 3;
const CPCAP_BIT_FS_INV: c_uint = 2;
const CPCAP_BIT_CLK_INV: c_uint = 1;
const CPCAP_BIT_SMB_CDC: c_uint = 0;

/* Register 515 CPCAP_REG_SDAC   --- Stereo DAC */
const CPCAP_BIT_FSYNC_CLK_IN_COMMON: c_uint = 11;
const CPCAP_BIT_SLAVE_PLL_CLK_INPUT: c_uint = 10;
const CPCAP_BIT_ST_CLOCK_TREE_RESET: c_uint = 9;
const CPCAP_BIT_DF_RESET_ST_DAC: c_uint = 8;
const CPCAP_BIT_ST_SR3: c_uint = 7;
const CPCAP_BIT_ST_SR2: c_uint = 6;
const CPCAP_BIT_ST_SR1: c_uint = 5;
const CPCAP_BIT_ST_SR0: c_uint = 4;
const CPCAP_BIT_ST_DAC_CLK2: c_uint = 3;
const CPCAP_BIT_ST_DAC_CLK1: c_uint = 2;
const CPCAP_BIT_ST_DAC_CLK0: c_uint = 1;
const CPCAP_BIT_ST_DAC_EN: c_uint = 0;

/* Register 516 CPCAP_REG_SDACDI --- Stereo DAC Digital Audio Interface */
const CPCAP_BIT_ST_L_TIMESLOT2: c_uint = 13;
const CPCAP_BIT_ST_L_TIMESLOT1: c_uint = 12;
const CPCAP_BIT_ST_L_TIMESLOT0: c_uint = 11;
const CPCAP_BIT_ST_R_TIMESLOT2: c_uint = 10;
const CPCAP_BIT_ST_R_TIMESLOT1: c_uint = 9;
const CPCAP_BIT_ST_R_TIMESLOT0: c_uint = 8;
const CPCAP_BIT_ST_DAC_CLK_IN_SEL: c_uint = 7;
const CPCAP_BIT_ST_FS_INV: c_uint = 6;
const CPCAP_BIT_ST_CLK_INV: c_uint = 5;
const CPCAP_BIT_ST_DIG_AUD_FS1: c_uint = 4;
const CPCAP_BIT_ST_DIG_AUD_FS0: c_uint = 3;
const CPCAP_BIT_DIG_AUD_IN_ST_DAC: c_uint = 2;
const CPCAP_BIT_ST_CLK_EN: c_uint = 1;
const CPCAP_BIT_SMB_ST_DAC: c_uint = 0;

/* Register 517 CPCAP_REG_TXI    --- TX Interface */
const CPCAP_BIT_PTT_TH: c_uint = 15;
const CPCAP_BIT_PTT_CMP_EN: c_uint = 14;
const CPCAP_BIT_HS_ID_TX: c_uint = 13;
const CPCAP_BIT_MB_ON2: c_uint = 12;
const CPCAP_BIT_MB_ON1L: c_uint = 11;
const CPCAP_BIT_MB_ON1R: c_uint = 10;
const CPCAP_BIT_RX_L_ENCODE: c_uint = 9;
const CPCAP_BIT_RX_R_ENCODE: c_uint = 8;
const CPCAP_BIT_MIC2_MUX: c_uint = 7;
const CPCAP_BIT_MIC2_PGA_EN: c_uint = 6;
const CPCAP_BIT_CDET_DIS: c_uint = 5;
const CPCAP_BIT_EMU_MIC_MUX: c_uint = 4;
const CPCAP_BIT_HS_MIC_MUX: c_uint = 3;
const CPCAP_BIT_MIC1_MUX: c_uint = 2;
const CPCAP_BIT_MIC1_PGA_EN: c_uint = 1;
const CPCAP_BIT_DLM: c_uint = 0;

/* Register 518 CPCAP_REG_TXMP   --- Mic Gain */
const CPCAP_BIT_MB_BIAS_R1: c_uint = 11;
const CPCAP_BIT_MB_BIAS_R0: c_uint = 10;
const CPCAP_BIT_MIC2_GAIN_4: c_uint = 9;
const CPCAP_BIT_MIC2_GAIN_3: c_uint = 8;
const CPCAP_BIT_MIC2_GAIN_2: c_uint = 7;
const CPCAP_BIT_MIC2_GAIN_1: c_uint = 6;
const CPCAP_BIT_MIC2_GAIN_0: c_uint = 5;
const CPCAP_BIT_MIC1_GAIN_4: c_uint = 4;
const CPCAP_BIT_MIC1_GAIN_3: c_uint = 3;
const CPCAP_BIT_MIC1_GAIN_2: c_uint = 2;
const CPCAP_BIT_MIC1_GAIN_1: c_uint = 1;
const CPCAP_BIT_MIC1_GAIN_0: c_uint = 0;

/* Register 519 CPCAP_REG_RXOA   --- RX Output Amplifier */
const CPCAP_BIT_UNUSED_519_15: c_uint = 15;
const CPCAP_BIT_UNUSED_519_14: c_uint = 14;
const CPCAP_BIT_UNUSED_519_13: c_uint = 13;
const CPCAP_BIT_STDAC_LOW_PWR_DISABLE: c_uint = 12;
const CPCAP_BIT_HS_LOW_PWR: c_uint = 11;
const CPCAP_BIT_HS_ID_RX: c_uint = 10;
const CPCAP_BIT_ST_HS_CP_EN: c_uint = 9;
const CPCAP_BIT_EMU_SPKR_R_EN: c_uint = 8;
const CPCAP_BIT_EMU_SPKR_L_EN: c_uint = 7;
const CPCAP_BIT_HS_L_EN: c_uint = 6;
const CPCAP_BIT_HS_R_EN: c_uint = 5;
const CPCAP_BIT_A4_LINEOUT_L_EN: c_uint = 4;
const CPCAP_BIT_A4_LINEOUT_R_EN: c_uint = 3;
const CPCAP_BIT_A2_LDSP_L_EN: c_uint = 2;
const CPCAP_BIT_A2_LDSP_R_EN: c_uint = 1;
const CPCAP_BIT_A1_EAR_EN: c_uint = 0;

/* Register 520 CPCAP_REG_RXVC   --- RX Volume Control */
const CPCAP_BIT_VOL_EXT3: c_uint = 15;
const CPCAP_BIT_VOL_EXT2: c_uint = 14;
const CPCAP_BIT_VOL_EXT1: c_uint = 13;
const CPCAP_BIT_VOL_EXT0: c_uint = 12;
const CPCAP_BIT_VOL_DAC3: c_uint = 11;
const CPCAP_BIT_VOL_DAC2: c_uint = 10;
const CPCAP_BIT_VOL_DAC1: c_uint = 9;
const CPCAP_BIT_VOL_DAC0: c_uint = 8;
const CPCAP_BIT_VOL_DAC_LSB_1dB1: c_uint = 7;
const CPCAP_BIT_VOL_DAC_LSB_1dB0: c_uint = 6;
const CPCAP_BIT_VOL_CDC3: c_uint = 5;
const CPCAP_BIT_VOL_CDC2: c_uint = 4;
const CPCAP_BIT_VOL_CDC1: c_uint = 3;
const CPCAP_BIT_VOL_CDC0: c_uint = 2;
const CPCAP_BIT_VOL_CDC_LSB_1dB1: c_uint = 1;
const CPCAP_BIT_VOL_CDC_LSB_1dB0: c_uint = 0;

/* Register 521 CPCAP_REG_RXCOA  --- Codec to Output Amp Switches */
const CPCAP_BIT_PGA_CDC_EN: c_uint = 10;
const CPCAP_BIT_CDC_SW: c_uint = 9;
const CPCAP_BIT_PGA_OUTR_USBDP_CDC_SW: c_uint = 8;
const CPCAP_BIT_PGA_OUTL_USBDN_CDC_SW: c_uint = 7;
const CPCAP_BIT_ALEFT_HS_CDC_SW: c_uint = 6;
const CPCAP_BIT_ARIGHT_HS_CDC_SW: c_uint = 5;
const CPCAP_BIT_A4_LINEOUT_L_CDC_SW: c_uint = 4;
const CPCAP_BIT_A4_LINEOUT_R_CDC_SW: c_uint = 3;
const CPCAP_BIT_A2_LDSP_L_CDC_SW: c_uint = 2;
const CPCAP_BIT_A2_LDSP_R_CDC_SW: c_uint = 1;
const CPCAP_BIT_A1_EAR_CDC_SW: c_uint = 0;

/* Register 522 CPCAP_REG_RXSDOA --- RX Stereo DAC to Output Amp Switches */
const CPCAP_BIT_PGA_DAC_EN: c_uint = 12;
const CPCAP_BIT_ST_DAC_SW: c_uint = 11;
const CPCAP_BIT_MONO_DAC1: c_uint = 10;
const CPCAP_BIT_MONO_DAC0: c_uint = 9;
const CPCAP_BIT_PGA_OUTR_USBDP_DAC_SW: c_uint = 8;
const CPCAP_BIT_PGA_OUTL_USBDN_DAC_SW: c_uint = 7;
const CPCAP_BIT_ALEFT_HS_DAC_SW: c_uint = 6;
const CPCAP_BIT_ARIGHT_HS_DAC_SW: c_uint = 5;
const CPCAP_BIT_A4_LINEOUT_L_DAC_SW: c_uint = 4;
const CPCAP_BIT_A4_LINEOUT_R_DAC_SW: c_uint = 3;
const CPCAP_BIT_A2_LDSP_L_DAC_SW: c_uint = 2;
const CPCAP_BIT_A2_LDSP_R_DAC_SW: c_uint = 1;
const CPCAP_BIT_A1_EAR_DAC_SW: c_uint = 0;

/* Register 523 CPCAP_REG_RXEPOA --- RX External PGA to Output Amp Switches */
const CPCAP_BIT_PGA_EXT_L_EN: c_uint = 14;
const CPCAP_BIT_PGA_EXT_R_EN: c_uint = 13;
const CPCAP_BIT_PGA_IN_L_SW: c_uint = 12;
const CPCAP_BIT_PGA_IN_R_SW: c_uint = 11;
const CPCAP_BIT_MONO_EXT1: c_uint = 10;
const CPCAP_BIT_MONO_EXT0: c_uint = 9;
const CPCAP_BIT_PGA_OUTR_USBDP_EXT_SW: c_uint = 8;
const CPCAP_BIT_PGA_OUTL_USBDN_EXT_SW: c_uint = 7;
const CPCAP_BIT_ALEFT_HS_EXT_SW: c_uint = 6;
const CPCAP_BIT_ARIGHT_HS_EXT_SW: c_uint = 5;
const CPCAP_BIT_A4_LINEOUT_L_EXT_SW: c_uint = 4;
const CPCAP_BIT_A4_LINEOUT_R_EXT_SW: c_uint = 3;
const CPCAP_BIT_A2_LDSP_L_EXT_SW: c_uint = 2;
const CPCAP_BIT_A2_LDSP_R_EXT_SW: c_uint = 1;
const CPCAP_BIT_A1_EAR_EXT_SW: c_uint = 0;

/* Register 525 CPCAP_REG_A2LA --- SPK Amplifier and Clock Config for Headset */
const CPCAP_BIT_NCP_CLK_SYNC: c_uint = 7;
const CPCAP_BIT_A2_CLK_SYNC: c_uint = 6;
const CPCAP_BIT_A2_FREE_RUN: c_uint = 5;
const CPCAP_BIT_A2_CLK2: c_uint = 4;
const CPCAP_BIT_A2_CLK1: c_uint = 3;
const CPCAP_BIT_A2_CLK0: c_uint = 2;
const CPCAP_BIT_A2_CLK_IN: c_uint = 1;
const CPCAP_BIT_A2_CONFIG: c_uint = 0;

const SLEEP_ACTIVATE_POWER: c_uint = 2;
const CLOCK_TREE_RESET_TIME: c_uint = 1;

/* constants for ST delay workaround */
const STM_STDAC_ACTIVATE_RAMP_TIME: c_uint = 1;
const STM_STDAC_EN_TEST_PRE: c_uint = 0x090C;
const STM_STDAC_EN_TEST_POST: c_uint = 0x0000;
const STM_STDAC_EN_ST_TEST1_PRE: c_uint = 0x2400;
const STM_STDAC_EN_ST_TEST1_POST: c_uint = 0x0400;

#[repr(C)]
struct cpcap_reg_info {
    reg: u16,
    mask: u16,
    val: u16,
}

static cpcap_default_regs: [cpcap_reg_info; 13] = unsafe {
    [
        cpcap_reg_info { reg: CPCAP_REG_CC as u16, mask: 0xFFFF, val: 0x0000 },
        cpcap_reg_info { reg: CPCAP_REG_CC as u16, mask: 0xFFFF, val: 0x0000 },
        cpcap_reg_info { reg: CPCAP_REG_CDI as u16, mask: 0xBFFF, val: 0x0000 },
        cpcap_reg_info { reg: CPCAP_REG_SDAC as u16, mask: 0x0FFF, val: 0x0000 },
        cpcap_reg_info { reg: CPCAP_REG_SDACDI as u16, mask: 0x3FFF, val: 0x0000 },
        cpcap_reg_info { reg: CPCAP_REG_TXI as u16, mask: 0x0FDF, val: 0x0000 },
        cpcap_reg_info { reg: CPCAP_REG_TXMP as u16, mask: 0x0FFF, val: 0x0400 },
        cpcap_reg_info { reg: CPCAP_REG_RXOA as u16, mask: 0x01FF, val: 0x0000 },
        cpcap_reg_info { reg: CPCAP_REG_RXVC as u16, mask: 0xFF3C, val: 0x0000 },
        cpcap_reg_info { reg: CPCAP_REG_RXCOA as u16, mask: 0x07FF, val: 0x0000 },
        cpcap_reg_info { reg: CPCAP_REG_RXSDOA as u16, mask: 0x1FFF, val: 0x0000 },
        cpcap_reg_info { reg: CPCAP_REG_RXEPOA as u16, mask: 0x7FFF, val: 0x0000 },
        cpcap_reg_info { reg: CPCAP_REG_A2LA as u16, mask: BIT(CPCAP_BIT_A2_FREE_RUN) as u16, val: BIT(CPCAP_BIT_A2_FREE_RUN) as u16 },
    ]
};

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum cpcap_dai {
    CPCAP_DAI_HIFI,
    CPCAP_DAI_VOICE,
}

#[repr(C)]
struct cpcap_audio {
    component: *mut snd_soc_component,
    regmap: *mut regmap,
    vendor: u16,
    codec_clk_id: c_int,
    codec_freq: c_int,
    codec_format: c_int,
    vaudio: *mut regulator,
    hsirq: c_int,
    mb2irq: c_int,
    jack: snd_soc_jack,
}

unsafe extern "C" fn cpcap_st_workaround(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let cpcap = snd_soc_component_get_drvdata(component);
    let mut err = 0;

    /* Only CPCAP from ST requires workaround */
    if (*cpcap).vendor != CPCAP_VENDOR_ST {
        return 0;
    }

    if event == SND_SOC_DAPM_PRE_PMU {
        err = regmap_write((*cpcap).regmap, CPCAP_REG_TEST, STM_STDAC_EN_TEST_PRE);
        if err != 0 {
            return err;
        }
        err = regmap_write((*cpcap).regmap, CPCAP_REG_ST_TEST1, STM_STDAC_EN_ST_TEST1_PRE);
    } else if event == SND_SOC_DAPM_POST_PMU {
        msleep(STM_STDAC_ACTIVATE_RAMP_TIME);
        err = regmap_write((*cpcap).regmap, CPCAP_REG_ST_TEST1, STM_STDAC_EN_ST_TEST1_POST);
        if err != 0 {
            return err;
        }
        err = regmap_write((*cpcap).regmap, CPCAP_REG_TEST, STM_STDAC_EN_TEST_POST);
    }

    err
}

/* Capture Gain Control: 0dB to 31dB in 1dB steps */
static mic_gain_tlv: [c_uint; 0] = DECLARE_TLV_DB_SCALE!(0, 100, 0);

/* Playback Gain Control: -33dB to 12dB in 3dB steps */
static vol_tlv: [c_uint; 0] = DECLARE_TLV_DB_SCALE!(-3300, 300, 0);

static cpcap_snd_controls: [snd_kcontrol_new; 7] = [
    /* Playback Gain */
    SOC_SINGLE_TLV!("HiFi Playback Volume", CPCAP_REG_RXVC, CPCAP_BIT_VOL_DAC0, 0xF, 0, vol_tlv),
    SOC_SINGLE_TLV!("Voice Playback Volume", CPCAP_REG_RXVC, CPCAP_BIT_VOL_CDC0, 0xF, 0, vol_tlv),
    SOC_SINGLE_TLV!("Ext Playback Volume", CPCAP_REG_RXVC, CPCAP_BIT_VOL_EXT0, 0xF, 0, vol_tlv),
    /* Capture Gain */
    SOC_SINGLE_TLV!("Mic1 Capture Volume", CPCAP_REG_TXMP, CPCAP_BIT_MIC1_GAIN_0, 0x1F, 0, mic_gain_tlv),
    SOC_SINGLE_TLV!("Mic2 Capture Volume", CPCAP_REG_TXMP, CPCAP_BIT_MIC2_GAIN_0, 0x1F, 0, mic_gain_tlv),
    /* Phase Invert */
    SOC_SINGLE!("Hifi Left Phase Invert Switch", CPCAP_REG_RXSDOA, CPCAP_BIT_MONO_DAC0, 1, 0),
    SOC_SINGLE!("Ext Left Phase Invert Switch", CPCAP_REG_RXEPOA, CPCAP_BIT_MONO_EXT0, 1, 0),
];

static cpcap_out_mux_texts: [*const c_char; 4] = [c"Off".as_ptr(), c"Voice".as_ptr(), c"HiFi".as_ptr(), c"Ext".as_ptr()];
static cpcap_in_right_mux_texts: [*const c_char; 5] = [c"Off".as_ptr(), c"Mic 1".as_ptr(), c"Headset Mic".as_ptr(), c"EMU Mic".as_ptr(), c"Ext Right".as_ptr()];
static cpcap_in_left_mux_texts: [*const c_char; 3] = [c"Off".as_ptr(), c"Mic 2".as_ptr(), c"Ext Left".as_ptr()];

/*
 * input muxes use unusual register layout, so that we need to use custom
 * getter/setter methods
 */
static cpcap_input_left_mux_enum: soc_enum = SOC_ENUM_SINGLE_EXT_DECL!(cpcap_in_left_mux_texts);
static cpcap_input_right_mux_enum: soc_enum = SOC_ENUM_SINGLE_EXT_DECL!(cpcap_in_right_mux_texts);

/*
 * mux uses same bit in CPCAP_REG_RXCOA, CPCAP_REG_RXSDOA & CPCAP_REG_RXEPOA;
 * even though the register layout makes it look like a mixer, this is a mux.
 * Enabling multiple inputs will result in no audio being forwarded.
 */
static cpcap_earpiece_mux_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(0, 0, cpcap_out_mux_texts);
static cpcap_spkr_r_mux_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(0, 1, cpcap_out_mux_texts);
static cpcap_spkr_l_mux_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(0, 2, cpcap_out_mux_texts);
static cpcap_line_r_mux_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(0, 3, cpcap_out_mux_texts);
static cpcap_line_l_mux_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(0, 4, cpcap_out_mux_texts);
static cpcap_hs_r_mux_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(0, 5, cpcap_out_mux_texts);
static cpcap_hs_l_mux_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(0, 6, cpcap_out_mux_texts);
static cpcap_emu_l_mux_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(0, 7, cpcap_out_mux_texts);
static cpcap_emu_r_mux_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(0, 8, cpcap_out_mux_texts);

unsafe extern "C" fn cpcap_output_mux_get_enum(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let cpcap = snd_soc_component_get_drvdata(component);
    let e = (*kcontrol).private_value as *mut soc_enum;
    let shift = (*e).shift_l;
    let mut reg_voice = 0;
    let mut reg_hifi = 0;
    let mut reg_ext = 0;
    let status: c_int;
    let mut err: c_int;

    err = regmap_read((*cpcap).regmap, CPCAP_REG_RXCOA, &mut reg_voice);
    if err != 0 { return err; }
    err = regmap_read((*cpcap).regmap, CPCAP_REG_RXSDOA, &mut reg_hifi);
    if err != 0 { return err; }
    err = regmap_read((*cpcap).regmap, CPCAP_REG_RXEPOA, &mut reg_ext);
    if err != 0 { return err; }

    reg_voice = (reg_voice >> shift) & 1;
    reg_hifi = (reg_hifi >> shift) & 1;
    reg_ext = (reg_ext >> shift) & 1;
    status = reg_ext << 2 | reg_hifi << 1 | reg_voice;

    let item = &mut (*ucontrol).value.enumerated.item[0];
    match status {
        0x04 => *item = 3,
        0x02 => *item = 2,
        0x01 => *item = 1,
        _ => *item = 0,
    }

    0
}

unsafe extern "C" fn cpcap_output_mux_put_enum(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let cpcap = snd_soc_component_get_drvdata(component);
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let e = (*kcontrol).private_value as *mut soc_enum;
    let muxval = (*ucontrol).value.enumerated.item[0];
    let mask = BIT((*e).shift_l);
    let mut reg_voice: u16 = 0x00;
    let mut reg_hifi: u16 = 0x00;
    let mut reg_ext: u16 = 0x00;
    let mut err: c_int;

    match muxval {
        1 => reg_voice = mask as u16,
        2 => reg_hifi = mask as u16,
        3 => reg_ext = mask as u16,
        _ => {}
    }

    err = regmap_update_bits((*cpcap).regmap, CPCAP_REG_RXCOA, mask, reg_voice as c_uint);
    if err != 0 { return err; }
    err = regmap_update_bits((*cpcap).regmap, CPCAP_REG_RXSDOA, mask, reg_hifi as c_uint);
    if err != 0 { return err; }
    err = regmap_update_bits((*cpcap).regmap, CPCAP_REG_RXEPOA, mask, reg_ext as c_uint);
    if err != 0 { return err; }

    snd_soc_dapm_mux_update_power(dapm, kcontrol, muxval, e, ::core::ptr::null_mut());
    0
}

unsafe extern "C" fn cpcap_input_right_mux_get_enum(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let cpcap = snd_soc_component_get_drvdata(component);
    let mut regval = 0;
    let mut mask = 0;
    let err = regmap_read((*cpcap).regmap, CPCAP_REG_TXI, &mut regval);
    if err != 0 { return err; }

    mask |= BIT(CPCAP_BIT_MIC1_MUX);
    mask |= BIT(CPCAP_BIT_HS_MIC_MUX);
    mask |= BIT(CPCAP_BIT_EMU_MIC_MUX);
    mask |= BIT(CPCAP_BIT_RX_R_ENCODE);

    let item = &mut (*ucontrol).value.enumerated.item[0];
    match (regval as c_uint) & mask {
        x if x == BIT(CPCAP_BIT_RX_R_ENCODE) => *item = 4,
        x if x == BIT(CPCAP_BIT_EMU_MIC_MUX) => *item = 3,
        x if x == BIT(CPCAP_BIT_HS_MIC_MUX) => *item = 2,
        x if x == BIT(CPCAP_BIT_MIC1_MUX) => *item = 1,
        _ => *item = 0,
    }
    0
}

unsafe extern "C" fn cpcap_input_right_mux_put_enum(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let cpcap = snd_soc_component_get_drvdata(component);
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let e = (*kcontrol).private_value as *mut soc_enum;
    let muxval = (*ucontrol).value.enumerated.item[0];
    let mut regval = 0;
    let mut mask = 0;

    mask |= BIT(CPCAP_BIT_MIC1_MUX);
    mask |= BIT(CPCAP_BIT_HS_MIC_MUX);
    mask |= BIT(CPCAP_BIT_EMU_MIC_MUX);
    mask |= BIT(CPCAP_BIT_RX_R_ENCODE);

    match muxval {
        1 => regval = BIT(CPCAP_BIT_MIC1_MUX),
        2 => regval = BIT(CPCAP_BIT_HS_MIC_MUX),
        3 => regval = BIT(CPCAP_BIT_EMU_MIC_MUX),
        4 => regval = BIT(CPCAP_BIT_RX_R_ENCODE),
        _ => {}
    }

    let err = regmap_update_bits((*cpcap).regmap, CPCAP_REG_TXI, mask, regval);
    if err != 0 { return err; }
    snd_soc_dapm_mux_update_power(dapm, kcontrol, muxval, e, ::core::ptr::null_mut());
    0
}

unsafe extern "C" fn cpcap_input_left_mux_get_enum(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let cpcap = snd_soc_component_get_drvdata(component);
    let mut regval = 0;
    let mut mask = 0;
    let err = regmap_read((*cpcap).regmap, CPCAP_REG_TXI, &mut regval);
    if err != 0 { return err; }

    mask |= BIT(CPCAP_BIT_MIC2_MUX);
    mask |= BIT(CPCAP_BIT_RX_L_ENCODE);

    let item = &mut (*ucontrol).value.enumerated.item[0];
    match (regval as c_uint) & mask {
        x if x == BIT(CPCAP_BIT_RX_L_ENCODE) => *item = 2,
        x if x == BIT(CPCAP_BIT_MIC2_MUX) => *item = 1,
        _ => *item = 0,
    }
    0
}

unsafe extern "C" fn cpcap_input_left_mux_put_enum(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let cpcap = snd_soc_component_get_drvdata(component);
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let e = (*kcontrol).private_value as *mut soc_enum;
    let muxval = (*ucontrol).value.enumerated.item[0];
    let mut regval = 0;
    let mut mask = 0;

    mask |= BIT(CPCAP_BIT_MIC2_MUX);
    mask |= BIT(CPCAP_BIT_RX_L_ENCODE);

    match muxval {
        1 => regval = BIT(CPCAP_BIT_MIC2_MUX),
        2 => regval = BIT(CPCAP_BIT_RX_L_ENCODE),
        _ => {}
    }

    let err = regmap_update_bits((*cpcap).regmap, CPCAP_REG_TXI, mask, regval);
    if err != 0 { return err; }
    snd_soc_dapm_mux_update_power(dapm, kcontrol, muxval, e, ::core::ptr::null_mut());
    0
}

static cpcap_input_left_mux: snd_kcontrol_new = SOC_DAPM_ENUM_EXT!("Input Left", cpcap_input_left_mux_enum, cpcap_input_left_mux_get_enum, cpcap_input_left_mux_put_enum);
static cpcap_input_right_mux: snd_kcontrol_new = SOC_DAPM_ENUM_EXT!("Input Right", cpcap_input_right_mux_enum, cpcap_input_right_mux_get_enum, cpcap_input_right_mux_put_enum);
static cpcap_emu_left_mux: snd_kcontrol_new = SOC_DAPM_ENUM_EXT!("EMU Left", cpcap_emu_l_mux_enum, cpcap_output_mux_get_enum, cpcap_output_mux_put_enum);
static cpcap_emu_right_mux: snd_kcontrol_new = SOC_DAPM_ENUM_EXT!("EMU Right", cpcap_emu_r_mux_enum, cpcap_output_mux_get_enum, cpcap_output_mux_put_enum);
static cpcap_hs_left_mux: snd_kcontrol_new = SOC_DAPM_ENUM_EXT!("Headset Left", cpcap_hs_l_mux_enum, cpcap_output_mux_get_enum, cpcap_output_mux_put_enum);
static cpcap_hs_right_mux: snd_kcontrol_new = SOC_DAPM_ENUM_EXT!("Headset Right", cpcap_hs_r_mux_enum, cpcap_output_mux_get_enum, cpcap_output_mux_put_enum);
static cpcap_line_left_mux: snd_kcontrol_new = SOC_DAPM_ENUM_EXT!("Line Left", cpcap_line_l_mux_enum, cpcap_output_mux_get_enum, cpcap_output_mux_put_enum);
static cpcap_line_right_mux: snd_kcontrol_new = SOC_DAPM_ENUM_EXT!("Line Right", cpcap_line_r_mux_enum, cpcap_output_mux_get_enum, cpcap_output_mux_put_enum);
static cpcap_speaker_left_mux: snd_kcontrol_new = SOC_DAPM_ENUM_EXT!("Speaker Left", cpcap_spkr_l_mux_enum, cpcap_output_mux_get_enum, cpcap_output_mux_put_enum);
static cpcap_speaker_right_mux: snd_kcontrol_new = SOC_DAPM_ENUM_EXT!("Speaker Right", cpcap_spkr_r_mux_enum, cpcap_output_mux_get_enum, cpcap_output_mux_put_enum);
static cpcap_earpiece_mux: snd_kcontrol_new = SOC_DAPM_ENUM_EXT!("Earpiece", cpcap_earpiece_mux_enum, cpcap_output_mux_get_enum, cpcap_output_mux_put_enum);

static cpcap_hifi_mono_mixer_controls: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE!("HiFi Mono Playback Switch", CPCAP_REG_RXSDOA, CPCAP_BIT_MONO_DAC1, 1, 0),
];
static cpcap_ext_mono_mixer_controls: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE!("Ext Mono Playback Switch", CPCAP_REG_RXEPOA, CPCAP_BIT_MONO_EXT0, 1, 0),
];
static cpcap_extr_mute_control: snd_kcontrol_new = SOC_DAPM_SINGLE!("Switch", CPCAP_REG_RXEPOA, CPCAP_BIT_PGA_IN_R_SW, 1, 0);
static cpcap_extl_mute_control: snd_kcontrol_new = SOC_DAPM_SINGLE!("Switch", CPCAP_REG_RXEPOA, CPCAP_BIT_PGA_IN_L_SW, 1, 0);
static cpcap_voice_loopback: snd_kcontrol_new = SOC_DAPM_SINGLE!("Switch", CPCAP_REG_TXI, CPCAP_BIT_DLM, 1, 0);

static cpcap_dapm_widgets: [snd_soc_dapm_widget_desc; 64] = [
    SND_SOC_DAPM_AIF_IN!("HiFi RX", NULL, 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!("Voice RX", NULL, 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("Voice TX", NULL, 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_REGULATOR_SUPPLY!("VAUDIO", SLEEP_ACTIVATE_POWER, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_pga, "Highpass Filter RX", CPCAP_REG_CC, CPCAP_BIT_AUDIHPF_0, 0x3, 0x3, 0x0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_pga, "Highpass Filter TX", CPCAP_REG_CC, CPCAP_BIT_AUDOHPF_0, 0x3, 0x3, 0x0),
    SND_SOC_DAPM_SUPPLY!("HiFi DAI Clock", CPCAP_REG_SDACDI, CPCAP_BIT_ST_CLK_EN, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("Voice DAI Clock", CPCAP_REG_CDI, CPCAP_BIT_CDC_CLK_EN, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("MIC1R Bias", CPCAP_REG_TXI, CPCAP_BIT_MB_ON1R, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("MIC1L Bias", CPCAP_REG_TXI, CPCAP_BIT_MB_ON1L, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("MIC2 Bias", CPCAP_REG_TXI, CPCAP_BIT_MB_ON2, 0, NULL, 0),
    SND_SOC_DAPM_INPUT!("MICR"), SND_SOC_DAPM_INPUT!("HSMIC"), SND_SOC_DAPM_INPUT!("EMUMIC"),
    SND_SOC_DAPM_INPUT!("MICL"), SND_SOC_DAPM_INPUT!("EXTR"), SND_SOC_DAPM_INPUT!("EXTL"),
    SND_SOC_DAPM_MUX!("Right Capture Route", SND_SOC_NOPM, 0, 0, &cpcap_input_right_mux),
    SND_SOC_DAPM_MUX!("Left Capture Route", SND_SOC_NOPM, 0, 0, &cpcap_input_left_mux),
    SND_SOC_DAPM_PGA!("Microphone 1 PGA", CPCAP_REG_TXI, CPCAP_BIT_MIC1_PGA_EN, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("Microphone 2 PGA", CPCAP_REG_TXI, CPCAP_BIT_MIC2_PGA_EN, 0, NULL, 0),
    SND_SOC_DAPM_ADC!("ADC Right", NULL, CPCAP_REG_CC, CPCAP_BIT_MIC1_CDC_EN, 0),
    SND_SOC_DAPM_ADC!("ADC Left", NULL, CPCAP_REG_CC, CPCAP_BIT_MIC2_CDC_EN, 0),
    SND_SOC_DAPM_DAC_E!("DAC HiFi", NULL, CPCAP_REG_SDAC, CPCAP_BIT_ST_DAC_EN, 0, cpcap_st_workaround, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_DAC_E!("DAC Voice", NULL, CPCAP_REG_CC, CPCAP_BIT_CDC_EN_RX, 0, cpcap_st_workaround, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_PGA!("HiFi PGA", CPCAP_REG_RXSDOA, CPCAP_BIT_PGA_DAC_EN, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("Voice PGA", CPCAP_REG_RXCOA, CPCAP_BIT_PGA_CDC_EN, 0, NULL, 0),
    SND_SOC_DAPM_PGA_E!("Ext Right PGA", CPCAP_REG_RXEPOA, CPCAP_BIT_PGA_EXT_R_EN, 0, NULL, 0, cpcap_st_workaround, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_PGA_E!("Ext Left PGA", CPCAP_REG_RXEPOA, CPCAP_BIT_PGA_EXT_L_EN, 0, NULL, 0, cpcap_st_workaround, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_SWITCH!("Ext Right Enable", SND_SOC_NOPM, 0, 0, &cpcap_extr_mute_control),
    SND_SOC_DAPM_SWITCH!("Ext Left Enable", SND_SOC_NOPM, 0, 0, &cpcap_extl_mute_control),
    SND_SOC_DAPM_SWITCH!("Voice Loopback", SND_SOC_NOPM, 0, 0, &cpcap_voice_loopback),
    SOC_MIXER_ARRAY!("HiFi Mono Left Mixer", SND_SOC_NOPM, 0, 0, cpcap_hifi_mono_mixer_controls),
    SOC_MIXER_ARRAY!("HiFi Mono Right Mixer", SND_SOC_NOPM, 0, 0, cpcap_hifi_mono_mixer_controls),
    SOC_MIXER_ARRAY!("Ext Mono Left Mixer", SND_SOC_NOPM, 0, 0, cpcap_ext_mono_mixer_controls),
    SOC_MIXER_ARRAY!("Ext Mono Right Mixer", SND_SOC_NOPM, 0, 0, cpcap_ext_mono_mixer_controls),
    SND_SOC_DAPM_MUX!("Earpiece Playback Route", SND_SOC_NOPM, 0, 0, &cpcap_earpiece_mux),
    SND_SOC_DAPM_MUX!("Speaker Right Playback Route", SND_SOC_NOPM, 0, 0, &cpcap_speaker_right_mux),
    SND_SOC_DAPM_MUX!("Speaker Left Playback Route", SND_SOC_NOPM, 0, 0, &cpcap_speaker_left_mux),
    SND_SOC_DAPM_MUX!("Lineout Right Playback Route", SND_SOC_NOPM, 0, 0, &cpcap_line_right_mux),
    SND_SOC_DAPM_MUX!("Lineout Left Playback Route", SND_SOC_NOPM, 0, 0, &cpcap_line_left_mux),
    SND_SOC_DAPM_MUX!("Headset Right Playback Route", SND_SOC_NOPM, 0, 0, &cpcap_hs_right_mux),
    SND_SOC_DAPM_MUX!("Headset Left Playback Route", SND_SOC_NOPM, 0, 0, &cpcap_hs_left_mux),
    SND_SOC_DAPM_MUX!("EMU Right Playback Route", SND_SOC_NOPM, 0, 0, &cpcap_emu_right_mux),
    SND_SOC_DAPM_MUX!("EMU Left Playback Route", SND_SOC_NOPM, 0, 0, &cpcap_emu_left_mux),
    SND_SOC_DAPM_PGA!("Earpiece PGA", CPCAP_REG_RXOA, CPCAP_BIT_A1_EAR_EN, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("Speaker Right PGA", CPCAP_REG_RXOA, CPCAP_BIT_A2_LDSP_R_EN, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("Speaker Left PGA", CPCAP_REG_RXOA, CPCAP_BIT_A2_LDSP_L_EN, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("Lineout Right PGA", CPCAP_REG_RXOA, CPCAP_BIT_A4_LINEOUT_R_EN, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("Lineout Left PGA", CPCAP_REG_RXOA, CPCAP_BIT_A4_LINEOUT_L_EN, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("Headset Right PGA", CPCAP_REG_RXOA, CPCAP_BIT_HS_R_EN, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("Headset Left PGA", CPCAP_REG_RXOA, CPCAP_BIT_HS_L_EN, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("EMU Right PGA", CPCAP_REG_RXOA, CPCAP_BIT_EMU_SPKR_R_EN, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("EMU Left PGA", CPCAP_REG_RXOA, CPCAP_BIT_EMU_SPKR_L_EN, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("Headset Charge Pump", CPCAP_REG_RXOA, CPCAP_BIT_ST_HS_CP_EN, 0, NULL, 0),
    SND_SOC_DAPM_OUTPUT!("EP"), SND_SOC_DAPM_OUTPUT!("SPKR"), SND_SOC_DAPM_OUTPUT!("SPKL"),
    SND_SOC_DAPM_OUTPUT!("LINER"), SND_SOC_DAPM_OUTPUT!("LINEL"), SND_SOC_DAPM_OUTPUT!("HSR"),
    SND_SOC_DAPM_OUTPUT!("HSL"), SND_SOC_DAPM_OUTPUT!("EMUR"), SND_SOC_DAPM_OUTPUT!("EMUL"),
];

macro_rules! route {
    ($sink:literal, NULL, $source:literal) => {
        snd_soc_dapm_route { sink: concat!($sink, "\0").as_ptr() as *const c_char, control: ::core::ptr::null(), source: concat!($source, "\0").as_ptr() as *const c_char }
    };
    ($sink:literal, $control:literal, $source:literal) => {
        snd_soc_dapm_route { sink: concat!($sink, "\0").as_ptr() as *const c_char, control: concat!($control, "\0").as_ptr() as *const c_char, source: concat!($source, "\0").as_ptr() as *const c_char }
    };
}

static intercon: [snd_soc_dapm_route; 94] = [
    route!("HiFi PGA", NULL, "VAUDIO"), route!("Voice PGA", NULL, "VAUDIO"), route!("Ext Right PGA", NULL, "VAUDIO"), route!("Ext Left PGA", NULL, "VAUDIO"), route!("Microphone 1 PGA", NULL, "VAUDIO"), route!("Microphone 2 PGA", NULL, "VAUDIO"),
    route!("HiFi RX", NULL, "HiFi Playback"), route!("Voice RX", NULL, "Voice Playback"), route!("Voice Capture", NULL, "Voice TX"),
    route!("HiFi RX", NULL, "HiFi DAI Clock"), route!("Voice RX", NULL, "Voice DAI Clock"), route!("Voice TX", NULL, "Voice DAI Clock"),
    route!("Voice Loopback", "Switch", "Voice TX"), route!("Voice RX", NULL, "Voice Loopback"),
    route!("Highpass Filter RX", NULL, "Voice RX"), route!("Voice TX", NULL, "Highpass Filter TX"),
    route!("DAC HiFi", NULL, "HiFi RX"), route!("DAC Voice", NULL, "Highpass Filter RX"),
    route!("HiFi PGA", NULL, "DAC HiFi"), route!("Voice PGA", NULL, "DAC Voice"),
    route!("Ext Right PGA", NULL, "EXTR"), route!("Ext Left PGA", NULL, "EXTL"),
    route!("Ext Right Enable", "Switch", "Ext Right PGA"), route!("Ext Left Enable", "Switch", "Ext Left PGA"),
    route!("HiFi Mono Left Mixer", NULL, "HiFi PGA"), route!("HiFi Mono Left Mixer", "HiFi Mono Playback Switch", "HiFi PGA"), route!("HiFi Mono Right Mixer", NULL, "HiFi PGA"), route!("HiFi Mono Right Mixer", "HiFi Mono Playback Switch", "HiFi PGA"),
    route!("Ext Mono Right Mixer", NULL, "Ext Right Enable"), route!("Ext Mono Right Mixer", "Ext Mono Playback Switch", "Ext Left Enable"), route!("Ext Mono Left Mixer", NULL, "Ext Left Enable"), route!("Ext Mono Left Mixer", "Ext Mono Playback Switch", "Ext Right Enable"),
    route!("Earpiece Playback Route", "HiFi", "HiFi Mono Right Mixer"), route!("Speaker Right Playback Route", "HiFi", "HiFi Mono Right Mixer"), route!("Speaker Left Playback Route", "HiFi", "HiFi Mono Left Mixer"), route!("Lineout Right Playback Route", "HiFi", "HiFi Mono Right Mixer"), route!("Lineout Left Playback Route", "HiFi", "HiFi Mono Left Mixer"), route!("Headset Right Playback Route", "HiFi", "HiFi Mono Right Mixer"), route!("Headset Left Playback Route", "HiFi", "HiFi Mono Left Mixer"), route!("EMU Right Playback Route", "HiFi", "HiFi Mono Right Mixer"), route!("EMU Left Playback Route", "HiFi", "HiFi Mono Left Mixer"),
    route!("Earpiece Playback Route", "Voice", "Voice PGA"), route!("Speaker Right Playback Route", "Voice", "Voice PGA"), route!("Speaker Left Playback Route", "Voice", "Voice PGA"), route!("Lineout Right Playback Route", "Voice", "Voice PGA"), route!("Lineout Left Playback Route", "Voice", "Voice PGA"), route!("Headset Right Playback Route", "Voice", "Voice PGA"), route!("Headset Left Playback Route", "Voice", "Voice PGA"), route!("EMU Right Playback Route", "Voice", "Voice PGA"), route!("EMU Left Playback Route", "Voice", "Voice PGA"),
    route!("Earpiece Playback Route", "Ext", "Ext Mono Right Mixer"), route!("Speaker Right Playback Route", "Ext", "Ext Mono Right Mixer"), route!("Speaker Left Playback Route", "Ext", "Ext Mono Left Mixer"), route!("Lineout Right Playback Route", "Ext", "Ext Mono Right Mixer"), route!("Lineout Left Playback Route", "Ext", "Ext Mono Left Mixer"), route!("Headset Right Playback Route", "Ext", "Ext Mono Right Mixer"), route!("Headset Left Playback Route", "Ext", "Ext Mono Left Mixer"), route!("EMU Right Playback Route", "Ext", "Ext Mono Right Mixer"), route!("EMU Left Playback Route", "Ext", "Ext Mono Left Mixer"),
    route!("Earpiece PGA", NULL, "Earpiece Playback Route"), route!("Speaker Right PGA", NULL, "Speaker Right Playback Route"), route!("Speaker Left PGA", NULL, "Speaker Left Playback Route"), route!("Lineout Right PGA", NULL, "Lineout Right Playback Route"), route!("Lineout Left PGA", NULL, "Lineout Left Playback Route"), route!("Headset Right PGA", NULL, "Headset Right Playback Route"), route!("Headset Left PGA", NULL, "Headset Left Playback Route"), route!("EMU Right PGA", NULL, "EMU Right Playback Route"), route!("EMU Left PGA", NULL, "EMU Left Playback Route"),
    route!("EP", NULL, "Earpiece PGA"), route!("SPKR", NULL, "Speaker Right PGA"), route!("SPKL", NULL, "Speaker Left PGA"), route!("LINER", NULL, "Lineout Right PGA"), route!("LINEL", NULL, "Lineout Left PGA"), route!("HSR", NULL, "Headset Right PGA"), route!("HSL", NULL, "Headset Left PGA"), route!("EMUR", NULL, "EMU Right PGA"), route!("EMUL", NULL, "EMU Left PGA"),
    route!("HSR", NULL, "Headset Charge Pump"), route!("HSL", NULL, "Headset Charge Pump"),
    route!("Right Capture Route", "Mic 1", "MICR"), route!("Right Capture Route", "Headset Mic", "HSMIC"), route!("Right Capture Route", "EMU Mic", "EMUMIC"), route!("Right Capture Route", "Ext Right", "EXTR"), route!("Left Capture Route", "Mic 2", "MICL"), route!("Left Capture Route", "Ext Left", "EXTL"),
    route!("Microphone 1 PGA", NULL, "Right Capture Route"), route!("Microphone 2 PGA", NULL, "Left Capture Route"),
    route!("ADC Right", NULL, "Microphone 1 PGA"), route!("ADC Left", NULL, "Microphone 2 PGA"),
    route!("Highpass Filter TX", NULL, "ADC Right"), route!("Highpass Filter TX", NULL, "ADC Left"),
    route!("MICL", NULL, "MIC1L Bias"), route!("MICR", NULL, "MIC1R Bias"),
];

unsafe fn neg_errno(errno: c_int) -> c_int {
    -errno
}

unsafe fn cpcap_set_sysclk(cpcap: *mut cpcap_audio, dai: cpcap_dai, clk_id: c_int, freq: c_int) -> c_int {
    let (clkfreqreg, clkfreqshift, clkidreg, clkidshift) = match dai {
        cpcap_dai::CPCAP_DAI_HIFI => (CPCAP_REG_SDAC, CPCAP_BIT_ST_DAC_CLK0, CPCAP_REG_SDACDI, CPCAP_BIT_ST_DAC_CLK_IN_SEL),
        cpcap_dai::CPCAP_DAI_VOICE => (CPCAP_REG_CC, CPCAP_BIT_CDC_CLK0, CPCAP_REG_CDI, CPCAP_BIT_CLK_IN_SEL),
    };
    let mut err: c_int;

    if clk_id < 0 || clk_id > 1 {
        dev_err((*(*cpcap).component).dev, c"invalid clk id %d".as_ptr(), clk_id);
        return neg_errno(EINVAL);
    }
    err = regmap_update_bits((*cpcap).regmap, clkidreg, BIT(clkidshift), if clk_id != 0 { BIT(clkidshift) } else { 0 });
    if err != 0 { return err; }

    if dai == cpcap_dai::CPCAP_DAI_VOICE {
        err = regmap_update_bits((*cpcap).regmap, CPCAP_REG_CDI, BIT(CPCAP_BIT_CDC_PLL_SEL), BIT(CPCAP_BIT_CDC_PLL_SEL));
        if err != 0 { return err; }
    }

    let clkfreqmask = 0x7 << clkfreqshift;
    let clkfreqval = match freq {
        15360000 => 0x01 << clkfreqshift,
        16800000 => 0x02 << clkfreqshift,
        19200000 => 0x03 << clkfreqshift,
        26000000 => 0x04 << clkfreqshift,
        33600000 => 0x05 << clkfreqshift,
        38400000 => 0x06 << clkfreqshift,
        _ => {
            dev_err((*(*cpcap).component).dev, c"unsupported freq %u".as_ptr(), freq);
            return neg_errno(EINVAL);
        }
    };

    err = regmap_update_bits((*cpcap).regmap, clkfreqreg, clkfreqmask, clkfreqval);
    if err != 0 { return err; }

    if dai == cpcap_dai::CPCAP_DAI_VOICE {
        (*cpcap).codec_clk_id = clk_id;
        (*cpcap).codec_freq = freq;
    }
    0
}

unsafe fn cpcap_set_samprate(cpcap: *mut cpcap_audio, dai: cpcap_dai, samplerate: c_int) -> c_int {
    let component = (*cpcap).component;
    let (sampreg, sampshift, sampreset) = match dai {
        cpcap_dai::CPCAP_DAI_HIFI => (CPCAP_REG_SDAC, CPCAP_BIT_ST_SR0, BIT(CPCAP_BIT_DF_RESET_ST_DAC) | BIT(CPCAP_BIT_ST_CLOCK_TREE_RESET)),
        cpcap_dai::CPCAP_DAI_VOICE => (CPCAP_REG_CC, CPCAP_BIT_CDC_SR0, BIT(CPCAP_BIT_DF_RESET) | BIT(CPCAP_BIT_CDC_CLOCK_TREE_RESET)),
    };
    let sampmask = (0xF << sampshift) | sampreset;
    let sampval = match samplerate {
        48000 => 0x8 << sampshift,
        44100 => 0x7 << sampshift,
        32000 => 0x6 << sampshift,
        24000 => 0x5 << sampshift,
        22050 => 0x4 << sampshift,
        16000 => 0x3 << sampshift,
        12000 => 0x2 << sampshift,
        11025 => 0x1 << sampshift,
        8000 => 0x0 << sampshift,
        _ => {
            dev_err((*component).dev, c"unsupported samplerate %d".as_ptr(), samplerate);
            return neg_errno(EINVAL);
        }
    };
    let mut err = regmap_update_bits((*cpcap).regmap, sampreg, sampmask, sampval | sampreset);
    if err != 0 { return err; }
    /* Wait for clock tree reset to complete */
    mdelay(CLOCK_TREE_RESET_TIME);
    let mut sampreadval = 0;
    err = regmap_read((*cpcap).regmap, sampreg, &mut sampreadval);
    if err != 0 { return err; }
    if (sampreadval as c_uint) & sampreset != 0 {
        dev_err((*component).dev, c"reset self-clear failed: %04x".as_ptr(), sampreadval);
        return neg_errno(EIO);
    }
    0
}

unsafe extern "C" fn cpcap_hifi_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let cpcap = snd_soc_component_get_drvdata(component);
    let rate = params_rate(params);
    dev_dbg((*component).dev, c"HiFi setup HW params: rate=%d".as_ptr(), rate);
    cpcap_set_samprate(cpcap, cpcap_dai::CPCAP_DAI_HIFI, rate)
}

unsafe extern "C" fn cpcap_hifi_set_dai_sysclk(codec_dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*codec_dai).component;
    let cpcap = snd_soc_component_get_drvdata(component);
    dev_dbg((*component).dev, c"HiFi setup sysclk: clk_id=%u, freq=%u".as_ptr(), clk_id, freq);
    cpcap_set_sysclk(cpcap, cpcap_dai::CPCAP_DAI_HIFI, clk_id, freq as c_int)
}

unsafe extern "C" fn cpcap_hifi_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let cpcap = snd_soc_component_get_drvdata(component);
    let dev = (*component).dev;
    let reg = CPCAP_REG_SDACDI;
    let mask = BIT(CPCAP_BIT_SMB_ST_DAC) | BIT(CPCAP_BIT_ST_CLK_INV) | BIT(CPCAP_BIT_ST_FS_INV) | BIT(CPCAP_BIT_ST_DIG_AUD_FS0) | BIT(CPCAP_BIT_ST_DIG_AUD_FS1) | BIT(CPCAP_BIT_ST_L_TIMESLOT0) | BIT(CPCAP_BIT_ST_L_TIMESLOT1) | BIT(CPCAP_BIT_ST_L_TIMESLOT2) | BIT(CPCAP_BIT_ST_R_TIMESLOT0) | BIT(CPCAP_BIT_ST_R_TIMESLOT1) | BIT(CPCAP_BIT_ST_R_TIMESLOT2);
    let mut val: c_uint = 0x0000;
    dev_dbg(dev, c"HiFi setup dai format (%08x)".as_ptr(), fmt);
    if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) != SND_SOC_DAIFMT_CBP_CFP {
        dev_err(dev, c"HiFi dai fmt failed: CPCAP should be provider".as_ptr());
        return neg_errno(EINVAL);
    }
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_IB_IF => { val |= BIT(CPCAP_BIT_ST_FS_INV); val |= BIT(CPCAP_BIT_ST_CLK_INV); }
        x if x == SND_SOC_DAIFMT_IB_NF => { val &= !BIT(CPCAP_BIT_ST_FS_INV); val |= BIT(CPCAP_BIT_ST_CLK_INV); }
        x if x == SND_SOC_DAIFMT_NB_IF => { val |= BIT(CPCAP_BIT_ST_FS_INV); val &= !BIT(CPCAP_BIT_ST_CLK_INV); }
        x if x == SND_SOC_DAIFMT_NB_NF => { val &= !BIT(CPCAP_BIT_ST_FS_INV); val &= !BIT(CPCAP_BIT_ST_CLK_INV); }
        _ => {
            dev_err(dev, c"HiFi dai fmt failed: unsupported clock invert mode".as_ptr());
            return neg_errno(EINVAL);
        }
    }
    if val & BIT(CPCAP_BIT_ST_CLK_INV) != 0 { val &= !BIT(CPCAP_BIT_ST_CLK_INV); } else { val |= BIT(CPCAP_BIT_ST_CLK_INV); }
    if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_I2S {
        val |= BIT(CPCAP_BIT_ST_DIG_AUD_FS0);
        val |= BIT(CPCAP_BIT_ST_DIG_AUD_FS1);
    } else {
        /* 01 - 4 slots network mode */
        val |= BIT(CPCAP_BIT_ST_DIG_AUD_FS0);
        val &= !BIT(CPCAP_BIT_ST_DIG_AUD_FS1);
        /* L on slot 1 */
        val |= BIT(CPCAP_BIT_ST_L_TIMESLOT0);
    }
    dev_dbg(dev, c"HiFi dai format: val=%04x".as_ptr(), val);
    regmap_update_bits((*cpcap).regmap, reg, mask, val)
}

unsafe extern "C" fn cpcap_hifi_set_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*dai).component;
    let cpcap = snd_soc_component_get_drvdata(component);
    let val = if mute != 0 { 0 } else { BIT(CPCAP_BIT_ST_DAC_SW) };
    dev_dbg((*component).dev, c"HiFi mute: %d".as_ptr(), mute);
    regmap_update_bits((*cpcap).regmap, CPCAP_REG_RXSDOA, BIT(CPCAP_BIT_ST_DAC_SW), val)
}

static cpcap_dai_hifi_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(cpcap_hifi_hw_params),
    set_sysclk: Some(cpcap_hifi_set_dai_sysclk),
    set_fmt: Some(cpcap_hifi_set_dai_fmt),
    mute_stream: Some(cpcap_hifi_set_mute),
    no_capture_mute: 1,
};

unsafe extern "C" fn cpcap_voice_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let component = (*dai).component;
    let cpcap = snd_soc_component_get_drvdata(component);
    let rate = params_rate(params);
    let channels = params_channels(params);
    let direction = (*substream).stream;
    dev_dbg((*component).dev, c"Voice setup HW params: rate=%d, direction=%d, chan=%d".as_ptr(), rate, direction, channels);
    let mut err = cpcap_set_samprate(cpcap, cpcap_dai::CPCAP_DAI_VOICE, rate);
    if err != 0 { return err; }
    if direction == SNDRV_PCM_STREAM_CAPTURE {
        let mut mask = 0;
        mask |= BIT(CPCAP_BIT_MIC1_RX_TIMESLOT0) | BIT(CPCAP_BIT_MIC1_RX_TIMESLOT1) | BIT(CPCAP_BIT_MIC1_RX_TIMESLOT2);
        mask |= BIT(CPCAP_BIT_MIC2_TIMESLOT0) | BIT(CPCAP_BIT_MIC2_TIMESLOT1) | BIT(CPCAP_BIT_MIC2_TIMESLOT2);
        let mut val = 0;
        if channels >= 2 { val = BIT(CPCAP_BIT_MIC1_RX_TIMESLOT0); }
        err = regmap_update_bits((*cpcap).regmap, CPCAP_REG_CDI, mask, val);
        if err != 0 { return err; }
    }
    snd_soc_runtime_set_dai_fmt(rtd, (*(*rtd).dai_link).dai_fmt)
}

unsafe extern "C" fn cpcap_voice_set_dai_sysclk(codec_dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*codec_dai).component;
    let cpcap = snd_soc_component_get_drvdata(component);
    dev_dbg((*component).dev, c"Voice setup sysclk: clk_id=%u, freq=%u".as_ptr(), clk_id, freq);
    cpcap_set_sysclk(cpcap, cpcap_dai::CPCAP_DAI_VOICE, clk_id, freq as c_int)
}

unsafe extern "C" fn cpcap_voice_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let cpcap = snd_soc_component_get_drvdata(component);
    let mask = BIT(CPCAP_BIT_SMB_CDC) | BIT(CPCAP_BIT_CLK_INV) | BIT(CPCAP_BIT_FS_INV) | BIT(CPCAP_BIT_CDC_DIG_AUD_FS0) | BIT(CPCAP_BIT_CDC_DIG_AUD_FS1);
    let mut val: c_uint = 0;
    dev_dbg((*component).dev, c"Voice setup dai format (%08x)".as_ptr(), fmt);
    if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) != SND_SOC_DAIFMT_CBP_CFP {
        dev_err((*component).dev, c"Voice dai fmt failed: CPCAP should be the provider".as_ptr());
        val &= !BIT(CPCAP_BIT_SMB_CDC);
    }
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_IB_IF => { val |= BIT(CPCAP_BIT_CLK_INV); val |= BIT(CPCAP_BIT_FS_INV); }
        x if x == SND_SOC_DAIFMT_IB_NF => { val |= BIT(CPCAP_BIT_CLK_INV); val &= !BIT(CPCAP_BIT_FS_INV); }
        x if x == SND_SOC_DAIFMT_NB_IF => { val &= !BIT(CPCAP_BIT_CLK_INV); val |= BIT(CPCAP_BIT_FS_INV); }
        x if x == SND_SOC_DAIFMT_NB_NF => { val &= !BIT(CPCAP_BIT_CLK_INV); val &= !BIT(CPCAP_BIT_FS_INV); }
        _ => dev_err((*component).dev, c"Voice dai fmt failed: unsupported clock invert mode".as_ptr()),
    }
    if val & BIT(CPCAP_BIT_CLK_INV) != 0 { val &= !BIT(CPCAP_BIT_CLK_INV); } else { val |= BIT(CPCAP_BIT_CLK_INV); }
    if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_I2S {
        /* 11 - true I2S mode */
        val |= BIT(CPCAP_BIT_CDC_DIG_AUD_FS0);
        val |= BIT(CPCAP_BIT_CDC_DIG_AUD_FS1);
    } else {
        /* 4 timeslots network mode */
        val |= BIT(CPCAP_BIT_CDC_DIG_AUD_FS0);
        val &= !BIT(CPCAP_BIT_CDC_DIG_AUD_FS1);
    }
    dev_dbg((*component).dev, c"Voice dai format: val=%04x".as_ptr(), val);
    let err = regmap_update_bits((*cpcap).regmap, CPCAP_REG_CDI, mask, val);
    if err != 0 { return err; }
    (*cpcap).codec_format = val as c_int;
    0
}

unsafe extern "C" fn cpcap_voice_set_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*dai).component;
    let cpcap = snd_soc_component_get_drvdata(component);
    let val = if mute != 0 { 0 } else { BIT(CPCAP_BIT_CDC_SW) };
    dev_dbg((*component).dev, c"Voice mute: %d".as_ptr(), mute);
    regmap_update_bits((*cpcap).regmap, CPCAP_REG_RXCOA, BIT(CPCAP_BIT_CDC_SW), val)
}

static cpcap_dai_voice_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(cpcap_voice_hw_params),
    set_sysclk: Some(cpcap_voice_set_dai_sysclk),
    set_fmt: Some(cpcap_voice_set_dai_fmt),
    mute_stream: Some(cpcap_voice_set_mute),
    no_capture_mute: 1,
};

static mut cpcap_dai: [snd_soc_dai_driver; 2] = unsafe {
    [
        snd_soc_dai_driver {
            id: 0,
            name: c"cpcap-hifi".as_ptr(),
            playback: snd_soc_pcm_stream { stream_name: c"HiFi Playback".as_ptr(), channels_min: 2, channels_max: 2, rates: SNDRV_PCM_RATE_8000_48000, formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FORMAT_S24_LE },
            capture: snd_soc_pcm_stream { stream_name: ::core::ptr::null(), channels_min: 0, channels_max: 0, rates: 0, formats: 0 },
            ops: &cpcap_dai_hifi_ops,
        },
        snd_soc_dai_driver {
            id: 1,
            name: c"cpcap-voice".as_ptr(),
            playback: snd_soc_pcm_stream { stream_name: c"Voice Playback".as_ptr(), channels_min: 1, channels_max: 1, rates: SNDRV_PCM_RATE_8000_48000, formats: SNDRV_PCM_FMTBIT_S16_LE },
            capture: snd_soc_pcm_stream { stream_name: c"Voice Capture".as_ptr(), channels_min: 1, channels_max: 2, rates: SNDRV_PCM_RATE_8000_48000, formats: SNDRV_PCM_FMTBIT_S16_LE },
            ops: &cpcap_dai_voice_ops,
        },
    ]
};

unsafe fn cpcap_dai_mux(cpcap: *mut cpcap_audio, swap_dai_configuration: bool) -> c_int {
    let hifi_mask = BIT(CPCAP_BIT_DIG_AUD_IN_ST_DAC);
    let voice_mask = BIT(CPCAP_BIT_DIG_AUD_IN);
    let (voice_val, hifi_val) = if !swap_dai_configuration {
        /* Codec on DAI0, HiFi on DAI1 */
        (0, hifi_mask)
    } else {
        /* Codec on DAI1, HiFi on DAI0 */
        (voice_mask, 0)
    };
    let mut err = regmap_update_bits((*cpcap).regmap, CPCAP_REG_CDI, voice_mask, voice_val);
    if err != 0 { return err; }
    err = regmap_update_bits((*cpcap).regmap, CPCAP_REG_SDACDI, hifi_mask, hifi_val);
    if err != 0 { return err; }
    0
}

unsafe fn cpcap_audio_reset(component: *mut snd_soc_component, swap_dai_configuration: bool) -> c_int {
    let cpcap = snd_soc_component_get_drvdata(component);
    dev_dbg((*component).dev, c"init audio codec".as_ptr());
    for i in 0..cpcap_default_regs.len() {
        let err = regmap_update_bits((*cpcap).regmap, cpcap_default_regs[i].reg as c_uint, cpcap_default_regs[i].mask as c_uint, cpcap_default_regs[i].val as c_uint);
        if err != 0 { return err; }
    }
    let mut err = cpcap_dai_mux(cpcap, swap_dai_configuration);
    if err != 0 { return err; }
    err = cpcap_set_sysclk(cpcap, cpcap_dai::CPCAP_DAI_HIFI, 0, 26000000);
    if err != 0 { return err; }
    err = cpcap_set_sysclk(cpcap, cpcap_dai::CPCAP_DAI_VOICE, 0, 26000000);
    if err != 0 { return err; }
    err = cpcap_set_samprate(cpcap, cpcap_dai::CPCAP_DAI_HIFI, 48000);
    if err != 0 { return err; }
    err = cpcap_set_samprate(cpcap, cpcap_dai::CPCAP_DAI_VOICE, 48000);
    if err != 0 { return err; }
    0
}

unsafe extern "C" fn cpcap_hs_irq_thread(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let component = data as *mut snd_soc_component;
    let cpcap = snd_soc_component_get_drvdata(component);
    let regmap = (*cpcap).regmap;
    let mut status = 0;
    let mut mask = SND_JACK_HEADSET;
    let mut val: c_uint;
    if regmap_test_bits(regmap, CPCAP_REG_INTS1, BIT(CPCAP_BIT_HS_S)) == 0 {
        val = BIT(CPCAP_BIT_MB_ON2) | BIT(CPCAP_BIT_PTT_CMP_EN);
        regmap_update_bits(regmap, CPCAP_REG_TXI, val, val);
        val = BIT(CPCAP_BIT_ST_HS_CP_EN);
        regmap_update_bits(regmap, CPCAP_REG_RXOA, val, val);
        regulator_set_mode((*cpcap).vaudio, REGULATOR_MODE_NORMAL);
        /* Give PTTS time to settle */
        msleep(20);
        if regmap_test_bits(regmap, CPCAP_REG_INTS2, BIT(CPCAP_BIT_PTT_S)) == 0 {
            /* Headphones detected. (May also be a headset with the
             * MFB pressed.)
             */
            status = SND_JACK_HEADPHONE;
            dev_info((*component).dev, c"HP plugged in\n".as_ptr());
        } else if regmap_test_bits(regmap, CPCAP_REG_INTS1, BIT(CPCAP_BIT_MB2_S)) == 1 {
            status = SND_JACK_HEADSET;
            dev_info((*component).dev, c"HS plugged in\n".as_ptr());
        } else {
            dev_info((*component).dev, c"Unsupported HS plugged in\n".as_ptr());
        }
    } else {
        let mic = ((*cpcap).jack.status & SND_JACK_MICROPHONE) != 0;
        dev_info((*component).dev, c"H%s disconnect\n".as_ptr(), if mic { c"S".as_ptr() } else { c"P".as_ptr() });
        val = BIT(CPCAP_BIT_MB_ON2) | BIT(CPCAP_BIT_PTT_CMP_EN);
        regmap_update_bits((*cpcap).regmap, CPCAP_REG_TXI, val, 0);
        val = BIT(CPCAP_BIT_ST_HS_CP_EN);
        regmap_update_bits((*cpcap).regmap, CPCAP_REG_RXOA, val, 0);
        regulator_set_mode((*cpcap).vaudio, REGULATOR_MODE_STANDBY);
        mask |= SND_JACK_BTN_0;
    }
    snd_soc_jack_report(&mut (*cpcap).jack, status, mask);
    IRQ_HANDLED
}

unsafe extern "C" fn cpcap_mb2_irq_thread(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let component = data as *mut snd_soc_component;
    let cpcap = snd_soc_component_get_drvdata(component);
    let regmap = (*cpcap).regmap;
    let mut status = 0;
    if regmap_test_bits(regmap, CPCAP_REG_INTS1, BIT(CPCAP_BIT_HS_S)) == 1 {
        return IRQ_HANDLED;
    }
    let mb2 = regmap_test_bits(regmap, CPCAP_REG_INTS1, BIT(CPCAP_BIT_MB2_S));
    let ptt = regmap_test_bits(regmap, CPCAP_REG_INTS2, BIT(CPCAP_BIT_PTT_S));
    /* Initial detection might have been with MFB pressed */
    if ((*cpcap).jack.status & SND_JACK_MICROPHONE) == 0 {
        if ptt == 1 && mb2 == 1 {
            dev_info((*component).dev, c"MIC plugged in\n".as_ptr());
            snd_soc_jack_report(&mut (*cpcap).jack, SND_JACK_MICROPHONE, SND_JACK_MICROPHONE);
        }
        return IRQ_HANDLED;
    }
    if mb2 == 0 || ptt == 0 {
        status = SND_JACK_BTN_0;
    }
    snd_soc_jack_report(&mut (*cpcap).jack, status, SND_JACK_BTN_0);
    IRQ_HANDLED
}

unsafe extern "C" fn cpcap_soc_probe(component: *mut snd_soc_component) -> c_int {
    let pdev = to_platform_device((*component).dev);
    let card = (*component).card;
    let cpcap = devm_kzalloc((*component).dev, ::core::mem::size_of::<cpcap_audio>(), GFP_KERNEL) as *mut cpcap_audio;
    if cpcap.is_null() { return neg_errno(ENOMEM); }
    snd_soc_component_set_drvdata(component, cpcap);
    (*cpcap).component = component;
    (*cpcap).vaudio = devm_regulator_get((*component).dev, c"VAUDIO".as_ptr());
    if IS_ERR((*cpcap).vaudio as *const c_void) {
        return dev_err_probe((*component).dev, PTR_ERR((*cpcap).vaudio as *const c_void), c"Cannot get VAUDIO regulator\n".as_ptr());
    }
    let mut err = snd_soc_card_jack_new(card, c"Headphones".as_ptr(), SND_JACK_HEADSET | SND_JACK_BTN_0, &mut (*cpcap).jack);
    if err < 0 {
        dev_err((*component).dev, c"Cannot create HS jack: %i\n".as_ptr(), err);
        return err;
    }
    snd_jack_set_key((*cpcap).jack.jack, SND_JACK_BTN_0, KEY_MEDIA);
    (*cpcap).regmap = dev_get_regmap((*(*component).dev).parent, ::core::ptr::null());
    if (*cpcap).regmap.is_null() { return neg_errno(ENODEV); }
    snd_soc_component_init_regmap(component, (*cpcap).regmap);
    err = cpcap_get_vendor((*component).dev, (*cpcap).regmap, &mut (*cpcap).vendor);
    if err != 0 { return err; }
    (*cpcap).hsirq = platform_get_irq_byname(pdev, c"hs".as_ptr());
    if (*cpcap).hsirq < 0 { return (*cpcap).hsirq; }
    err = devm_request_threaded_irq((*component).dev, (*cpcap).hsirq, ::core::ptr::null_mut(), Some(cpcap_hs_irq_thread), IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING | IRQF_ONESHOT, c"cpcap-codec-hs".as_ptr(), component as *mut c_void);
    if err != 0 {
        dev_warn((*component).dev, c"no HS irq%i: %i\n".as_ptr(), (*cpcap).hsirq, err);
        return err;
    }
    (*cpcap).mb2irq = platform_get_irq_byname(pdev, c"mb2".as_ptr());
    if (*cpcap).mb2irq < 0 { return (*cpcap).mb2irq; }
    err = devm_request_threaded_irq((*component).dev, (*cpcap).mb2irq, ::core::ptr::null_mut(), Some(cpcap_mb2_irq_thread), IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING | IRQF_ONESHOT, c"cpcap-codec-mb2".as_ptr(), component as *mut c_void);
    if err != 0 {
        dev_warn((*component).dev, c"no MB2 irq%i: %i\n".as_ptr(), (*cpcap).mb2irq, err);
        return err;
    }
    err = cpcap_audio_reset(component, false);
    if err != 0 { return err; }
    cpcap_hs_irq_thread((*cpcap).hsirq, component as *mut c_void);
    enable_irq_wake((*cpcap).hsirq);
    enable_irq_wake((*cpcap).mb2irq);
    0
}

unsafe extern "C" fn cpcap_soc_remove(component: *mut snd_soc_component) {
    let cpcap = snd_soc_component_get_drvdata(component);
    disable_irq_wake((*cpcap).hsirq);
    disable_irq_wake((*cpcap).mb2irq);
}

unsafe extern "C" fn cpcap_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let cpcap = snd_soc_component_get_drvdata(component);
    /* VAIDIO should be kept in normal mode in order MIC/PTT to work */
    if ((*cpcap).jack.status & SND_JACK_MICROPHONE) != 0 {
        return 0;
    }
    if level == SND_SOC_BIAS_PREPARE {
        regulator_set_mode((*cpcap).vaudio, REGULATOR_MODE_NORMAL);
    } else if level == SND_SOC_BIAS_STANDBY {
        regulator_set_mode((*cpcap).vaudio, REGULATOR_MODE_STANDBY);
    } else if level == SND_SOC_BIAS_OFF || level == SND_SOC_BIAS_ON {
    }
    0
}

static soc_codec_dev_cpcap: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(cpcap_soc_probe),
    remove: Some(cpcap_soc_remove),
    controls: cpcap_snd_controls.as_ptr(),
    num_controls: cpcap_snd_controls.len() as c_uint,
    dapm_widgets: cpcap_dapm_widgets.as_ptr(),
    num_dapm_widgets: cpcap_dapm_widgets.len() as c_uint,
    dapm_routes: intercon.as_ptr(),
    num_dapm_routes: intercon.len() as c_uint,
    set_bias_level: Some(cpcap_set_bias_level),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn cpcap_codec_probe(pdev: *mut platform_device) -> c_int {
    let codec_node = of_get_child_by_name((*(*(*pdev).dev.parent).of_node).cast(), c"audio-codec".as_ptr());
    if codec_node.is_null() {
        return neg_errno(ENODEV);
    }
    (*pdev).dev.of_node = codec_node;
    devm_snd_soc_register_component(&mut (*pdev).dev, &soc_codec_dev_cpcap, cpcap_dai.as_mut_ptr(), cpcap_dai.len() as c_int)
}

static mut cpcap_codec_driver: platform_driver = platform_driver {
    probe: Some(cpcap_codec_probe),
    driver: platform_driver_driver {
        name: c"cpcap-codec".as_ptr(),
    },
};

module_platform_driver!(cpcap_codec_driver);

MODULE_ALIAS!("platform:cpcap-codec");
MODULE_DESCRIPTION!("ASoC CPCAP codec driver");
MODULE_AUTHOR!("Sebastian Reichel");
MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
