// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2015-2017, The Linux Foundation.
// Copyright (c) 2019, Linaro Limited
//
// Rust translation of soc/codecs/wsa881x.c. Linux kernel headers and their
// macros/types are represented as external dependencies or compact Rust
// equivalents where the file-local meaning is clear.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

const fn BIT(n: c_uint) -> c_uint {
    1u32 << n
}

const fn GENMASK(h: c_uint, l: c_uint) -> c_uint {
    if h == 31 {
        (!0u32) << l
    } else {
        ((1u32 << (h + 1)) - 1) & ((!0u32) << l)
    }
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

fn fls(x: c_int) -> c_int {
    if x == 0 {
        0
    } else {
        c_int::BITS as c_int - x.leading_zeros() as c_int
    }
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
pub struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
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
    pub integer: snd_ctl_elem_value_integer,
}
#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}
type c_long = isize;

#[repr(C)]
pub struct soc_mixer_control {
    pub reg: c_int,
    pub max: c_int,
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
pub struct snd_pcm_substream {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
}
#[repr(C)]
pub struct sdw_stream_config {
    pub ch_count: c_uint,
    pub bps: c_uint,
    pub frame_rate: c_uint,
    pub direction: c_uint,
    pub type_: c_uint,
}
#[repr(C)]
pub struct sdw_stream_runtime {
    _private: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_port_config {
    pub num: c_uint,
    pub ch_mask: c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_dpn_prop {
    pub num: c_uint,
    pub type_: c_uint,
    pub min_ch: c_uint,
    pub max_ch: c_uint,
    pub simple_ch_prep_sm: bool,
    pub read_only_wordlength: bool,
}
#[repr(C)]
pub struct sdw_slave_prop {
    pub sink_ports: c_uint,
    pub sink_dpn_prop: *mut sdw_dpn_prop,
    pub scp_int1_mask: c_uint,
    pub clk_stop_mode1: bool,
}
#[repr(C)]
pub struct sdw_slave {
    pub dev: device,
    pub dev_num: c_uint,
    pub prop: sdw_slave_prop,
}
#[repr(C)]
pub struct sdw_device_id {
    pub mfg_id: c_uint,
    pub part_id: c_uint,
    pub class_id: c_uint,
}
#[repr(C)]
pub struct sdw_bus_params {
    pub next_bank: c_uint,
}
#[repr(C)]
pub struct sdw_prepare_ch {
    pub num: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_sequence {
    pub reg: c_uint,
    pub def: c_uint,
    pub delay_us: c_uint,
}
#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub cache_type: c_uint,
    pub reg_defaults: *const reg_default,
    pub max_register: c_uint,
    pub num_reg_defaults: usize,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub reg_format_endian: c_uint,
    pub val_format_endian: c_uint,
}

#[repr(C)]
pub struct soc_enum {
    pub reg: c_uint,
    pub shift_l: c_uint,
    pub items: c_uint,
    pub texts: *const *const c_char,
}
#[repr(C)]
pub struct snd_kcontrol_new {
    pub _opaque: usize,
}
#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}
#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
    pub _opaque: usize,
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub set_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, *mut c_void, c_int) -> c_int>,
}
#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub rates: c_uint,
    pub formats: c_uint,
    pub rate_max: c_uint,
    pub rate_min: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: usize,
    pub dapm_widgets: *const snd_soc_dapm_widget_desc,
    pub num_dapm_widgets: usize,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: usize,
    pub endianness: c_uint,
}
#[repr(C)]
pub struct sdw_slave_ops {
    pub update_status: Option<unsafe extern "C" fn(*mut sdw_slave, sdw_slave_status) -> c_int>,
    pub bus_config: Option<unsafe extern "C" fn(*mut sdw_slave, *mut sdw_bus_params) -> c_int>,
    pub port_prep: Option<unsafe extern "C" fn(*mut sdw_slave, *mut sdw_prepare_ch, sdw_port_prep_ops) -> c_int>,
}
#[repr(C)]
pub struct dev_pm_ops {
    pub _opaque: usize,
}
#[repr(C)]
pub struct sdw_driver_inner {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}
#[repr(C)]
pub struct sdw_driver {
    pub probe: Option<unsafe extern "C" fn(*mut sdw_slave, *const sdw_device_id) -> c_int>,
    pub ops: *const sdw_slave_ops,
    pub id_table: *const sdw_device_id,
    pub driver: sdw_driver_inner,
}

type sdw_slave_status = c_uint;
type sdw_port_prep_ops = c_uint;

extern "C" {
    fn regmap_register_patch(map: *mut regmap, regs: *const reg_sequence, num_regs: usize) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_multi_reg_write(map: *mut regmap, regs: *const reg_sequence, num_regs: usize) -> c_int;
    fn snd_soc_component_get_drvdata(comp: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_init_regmap(comp: *mut snd_soc_component, map: *mut regmap);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_update_bits(comp: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn sdw_stream_add_slave(slave: *mut sdw_slave, config: *mut sdw_stream_config, ports: *mut sdw_port_config, num_ports: c_int, stream: *mut sdw_stream_runtime) -> c_int;
    fn sdw_stream_remove_slave(slave: *mut sdw_slave, stream: *mut sdw_stream_runtime);
    fn sdw_write(slave: *mut sdw_slave, reg: c_uint, val: c_uint) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn devm_regmap_init_sdw(slave: *mut sdw_slave, config: *const regmap_config) -> *mut regmap;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: usize) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn gpiod_direction_output(desc: *mut gpio_desc, value: c_int) -> c_int;
    fn dev_get_regmap(dev: *mut device, name: *const c_char) -> *mut regmap;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn dev_to_sdw_dev(dev: *mut device) -> *mut sdw_slave;
    fn sdw_slave_wait_for_init(slave: *mut sdw_slave, timeout: c_int) -> c_int;
    fn regcache_sync(map: *mut regmap) -> c_int;
}

const REGCACHE_MAPLE: c_uint = 0;
const REGMAP_ENDIAN_NATIVE: c_uint = 0;
const SDW_DPN_SIMPLE: c_uint = 0;
const SDW_DATA_DIR_RX: c_uint = 0;
const SDW_STREAM_PDM: c_uint = 0;
const SDW_SCP_INT1_BUS_CLASH: c_uint = BIT(0);
const SDW_SCP_INT1_PARITY: c_uint = BIT(1);
const GPIOD_OUT_LOW: c_uint = 0;
const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EACCES: c_int = 13;
const SND_SOC_DAPM_PRE_PMU: c_int = 1 << 0;
const SND_SOC_DAPM_POST_PMU: c_int = 1 << 1;
const SND_SOC_DAPM_POST_PMD: c_int = 1 << 2;
const SND_SOC_NOPM: c_uint = 0;
const SNDRV_PCM_RATE_48000: c_uint = 0x0000_0001;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 0x0000_0001;
const SDW_SLAVE_UNATTACHED: sdw_slave_status = 0;
const SDW_SLAVE_ATTACHED: sdw_slave_status = 1;
const SDW_OPS_PORT_POST_PREP: sdw_port_prep_ops = 1;

const WSA881X_DIGITAL_BASE: c_uint = 0x3000;
const WSA881X_ANALOG_BASE: c_uint = 0x3100;

/* Digital register address space */
const WSA881X_CHIP_ID0: c_uint = WSA881X_DIGITAL_BASE + 0x0000;
const WSA881X_CHIP_ID1: c_uint = WSA881X_DIGITAL_BASE + 0x0001;
const WSA881X_CHIP_ID2: c_uint = WSA881X_DIGITAL_BASE + 0x0002;
const WSA881X_CHIP_ID3: c_uint = WSA881X_DIGITAL_BASE + 0x0003;
const WSA881X_BUS_ID: c_uint = WSA881X_DIGITAL_BASE + 0x0004;
const WSA881X_CDC_RST_CTL: c_uint = WSA881X_DIGITAL_BASE + 0x0005;
const WSA881X_CDC_TOP_CLK_CTL: c_uint = WSA881X_DIGITAL_BASE + 0x0006;
const WSA881X_CDC_ANA_CLK_CTL: c_uint = WSA881X_DIGITAL_BASE + 0x0007;
const WSA881X_CDC_DIG_CLK_CTL: c_uint = WSA881X_DIGITAL_BASE + 0x0008;
const WSA881X_CLOCK_CONFIG: c_uint = WSA881X_DIGITAL_BASE + 0x0009;
const WSA881X_ANA_CTL: c_uint = WSA881X_DIGITAL_BASE + 0x000A;
const WSA881X_SWR_RESET_EN: c_uint = WSA881X_DIGITAL_BASE + 0x000B;
const WSA881X_RESET_CTL: c_uint = WSA881X_DIGITAL_BASE + 0x000C;
const WSA881X_TADC_VALUE_CTL: c_uint = WSA881X_DIGITAL_BASE + 0x000F;
const WSA881X_TEMP_DETECT_CTL: c_uint = WSA881X_DIGITAL_BASE + 0x0010;
const WSA881X_TEMP_MSB: c_uint = WSA881X_DIGITAL_BASE + 0x0011;
const WSA881X_TEMP_LSB: c_uint = WSA881X_DIGITAL_BASE + 0x0012;
const WSA881X_TEMP_CONFIG0: c_uint = WSA881X_DIGITAL_BASE + 0x0013;
const WSA881X_TEMP_CONFIG1: c_uint = WSA881X_DIGITAL_BASE + 0x0014;
const WSA881X_CDC_CLIP_CTL: c_uint = WSA881X_DIGITAL_BASE + 0x0015;
const WSA881X_SDM_PDM9_LSB: c_uint = WSA881X_DIGITAL_BASE + 0x0016;
const WSA881X_SDM_PDM9_MSB: c_uint = WSA881X_DIGITAL_BASE + 0x0017;
const WSA881X_CDC_RX_CTL: c_uint = WSA881X_DIGITAL_BASE + 0x0018;
const WSA881X_DEM_BYPASS_DATA0: c_uint = WSA881X_DIGITAL_BASE + 0x0019;
const WSA881X_DEM_BYPASS_DATA1: c_uint = WSA881X_DIGITAL_BASE + 0x001A;
const WSA881X_DEM_BYPASS_DATA2: c_uint = WSA881X_DIGITAL_BASE + 0x001B;
const WSA881X_DEM_BYPASS_DATA3: c_uint = WSA881X_DIGITAL_BASE + 0x001C;
const WSA881X_OTP_CTRL0: c_uint = WSA881X_DIGITAL_BASE + 0x001D;
const WSA881X_OTP_CTRL1: c_uint = WSA881X_DIGITAL_BASE + 0x001E;
const WSA881X_HDRIVE_CTL_GROUP1: c_uint = WSA881X_DIGITAL_BASE + 0x001F;
const WSA881X_INTR_MODE: c_uint = WSA881X_DIGITAL_BASE + 0x0020;
const WSA881X_INTR_MASK: c_uint = WSA881X_DIGITAL_BASE + 0x0021;
const WSA881X_INTR_STATUS: c_uint = WSA881X_DIGITAL_BASE + 0x0022;
const WSA881X_INTR_CLEAR: c_uint = WSA881X_DIGITAL_BASE + 0x0023;
const WSA881X_INTR_LEVEL: c_uint = WSA881X_DIGITAL_BASE + 0x0024;
const WSA881X_INTR_SET: c_uint = WSA881X_DIGITAL_BASE + 0x0025;
const WSA881X_INTR_TEST: c_uint = WSA881X_DIGITAL_BASE + 0x0026;
const WSA881X_PDM_TEST_MODE: c_uint = WSA881X_DIGITAL_BASE + 0x0030;
const WSA881X_ATE_TEST_MODE: c_uint = WSA881X_DIGITAL_BASE + 0x0031;
const WSA881X_PIN_CTL_MODE: c_uint = WSA881X_DIGITAL_BASE + 0x0032;
const WSA881X_PIN_CTL_OE: c_uint = WSA881X_DIGITAL_BASE + 0x0033;
const WSA881X_PIN_WDATA_IOPAD: c_uint = WSA881X_DIGITAL_BASE + 0x0034;
const WSA881X_PIN_STATUS: c_uint = WSA881X_DIGITAL_BASE + 0x0035;
const WSA881X_DIG_DEBUG_MODE: c_uint = WSA881X_DIGITAL_BASE + 0x0037;
const WSA881X_DIG_DEBUG_SEL: c_uint = WSA881X_DIGITAL_BASE + 0x0038;
const WSA881X_DIG_DEBUG_EN: c_uint = WSA881X_DIGITAL_BASE + 0x0039;
const WSA881X_SWR_HM_TEST1: c_uint = WSA881X_DIGITAL_BASE + 0x003B;
const WSA881X_SWR_HM_TEST2: c_uint = WSA881X_DIGITAL_BASE + 0x003C;
const WSA881X_TEMP_DETECT_DBG_CTL: c_uint = WSA881X_DIGITAL_BASE + 0x003D;
const WSA881X_TEMP_DEBUG_MSB: c_uint = WSA881X_DIGITAL_BASE + 0x003E;
const WSA881X_TEMP_DEBUG_LSB: c_uint = WSA881X_DIGITAL_BASE + 0x003F;
const WSA881X_SAMPLE_EDGE_SEL: c_uint = WSA881X_DIGITAL_BASE + 0x0044;
const WSA881X_IOPAD_CTL: c_uint = WSA881X_DIGITAL_BASE + 0x0045;
const WSA881X_SPARE_0: c_uint = WSA881X_DIGITAL_BASE + 0x0050;
const WSA881X_SPARE_1: c_uint = WSA881X_DIGITAL_BASE + 0x0051;
const WSA881X_SPARE_2: c_uint = WSA881X_DIGITAL_BASE + 0x0052;
const WSA881X_OTP_REG_0: c_uint = WSA881X_DIGITAL_BASE + 0x0080;
const WSA881X_OTP_REG_1: c_uint = WSA881X_DIGITAL_BASE + 0x0081;
const WSA881X_OTP_REG_2: c_uint = WSA881X_DIGITAL_BASE + 0x0082;
const WSA881X_OTP_REG_3: c_uint = WSA881X_DIGITAL_BASE + 0x0083;
const WSA881X_OTP_REG_4: c_uint = WSA881X_DIGITAL_BASE + 0x0084;
const WSA881X_OTP_REG_5: c_uint = WSA881X_DIGITAL_BASE + 0x0085;
const WSA881X_OTP_REG_6: c_uint = WSA881X_DIGITAL_BASE + 0x0086;
const WSA881X_OTP_REG_7: c_uint = WSA881X_DIGITAL_BASE + 0x0087;
const WSA881X_OTP_REG_8: c_uint = WSA881X_DIGITAL_BASE + 0x0088;
const WSA881X_OTP_REG_9: c_uint = WSA881X_DIGITAL_BASE + 0x0089;
const WSA881X_OTP_REG_10: c_uint = WSA881X_DIGITAL_BASE + 0x008A;
const WSA881X_OTP_REG_11: c_uint = WSA881X_DIGITAL_BASE + 0x008B;
const WSA881X_OTP_REG_12: c_uint = WSA881X_DIGITAL_BASE + 0x008C;
const WSA881X_OTP_REG_13: c_uint = WSA881X_DIGITAL_BASE + 0x008D;
const WSA881X_OTP_REG_14: c_uint = WSA881X_DIGITAL_BASE + 0x008E;
const WSA881X_OTP_REG_15: c_uint = WSA881X_DIGITAL_BASE + 0x008F;
const WSA881X_OTP_REG_16: c_uint = WSA881X_DIGITAL_BASE + 0x0090;
const WSA881X_OTP_REG_17: c_uint = WSA881X_DIGITAL_BASE + 0x0091;
const WSA881X_OTP_REG_18: c_uint = WSA881X_DIGITAL_BASE + 0x0092;
const WSA881X_OTP_REG_19: c_uint = WSA881X_DIGITAL_BASE + 0x0093;
const WSA881X_OTP_REG_20: c_uint = WSA881X_DIGITAL_BASE + 0x0094;
const WSA881X_OTP_REG_21: c_uint = WSA881X_DIGITAL_BASE + 0x0095;
const WSA881X_OTP_REG_22: c_uint = WSA881X_DIGITAL_BASE + 0x0096;
const WSA881X_OTP_REG_23: c_uint = WSA881X_DIGITAL_BASE + 0x0097;
const WSA881X_OTP_REG_24: c_uint = WSA881X_DIGITAL_BASE + 0x0098;
const WSA881X_OTP_REG_25: c_uint = WSA881X_DIGITAL_BASE + 0x0099;
const WSA881X_OTP_REG_26: c_uint = WSA881X_DIGITAL_BASE + 0x009A;
const WSA881X_OTP_REG_27: c_uint = WSA881X_DIGITAL_BASE + 0x009B;
const WSA881X_OTP_REG_28: c_uint = WSA881X_DIGITAL_BASE + 0x009C;
const WSA881X_OTP_REG_29: c_uint = WSA881X_DIGITAL_BASE + 0x009D;
const WSA881X_OTP_REG_30: c_uint = WSA881X_DIGITAL_BASE + 0x009E;
const WSA881X_OTP_REG_31: c_uint = WSA881X_DIGITAL_BASE + 0x009F;
const WSA881X_OTP_REG_63: c_uint = WSA881X_DIGITAL_BASE + 0x00BF;

/* Analog Register address space */
const WSA881X_BIAS_REF_CTRL: c_uint = WSA881X_ANALOG_BASE + 0x0000;
const WSA881X_BIAS_TEST: c_uint = WSA881X_ANALOG_BASE + 0x0001;
const WSA881X_BIAS_BIAS: c_uint = WSA881X_ANALOG_BASE + 0x0002;
const WSA881X_TEMP_OP: c_uint = WSA881X_ANALOG_BASE + 0x0003;
const WSA881X_TEMP_IREF_CTRL: c_uint = WSA881X_ANALOG_BASE + 0x0004;
const WSA881X_TEMP_ISENS_CTRL: c_uint = WSA881X_ANALOG_BASE + 0x0005;
const WSA881X_TEMP_CLK_CTRL: c_uint = WSA881X_ANALOG_BASE + 0x0006;
const WSA881X_TEMP_TEST: c_uint = WSA881X_ANALOG_BASE + 0x0007;
const WSA881X_TEMP_BIAS: c_uint = WSA881X_ANALOG_BASE + 0x0008;
const WSA881X_TEMP_ADC_CTRL: c_uint = WSA881X_ANALOG_BASE + 0x0009;
const WSA881X_TEMP_DOUT_MSB: c_uint = WSA881X_ANALOG_BASE + 0x000A;
const WSA881X_TEMP_DOUT_LSB: c_uint = WSA881X_ANALOG_BASE + 0x000B;
const WSA881X_ADC_EN_MODU_V: c_uint = WSA881X_ANALOG_BASE + 0x0010;
const WSA881X_ADC_EN_MODU_I: c_uint = WSA881X_ANALOG_BASE + 0x0011;
const WSA881X_ADC_EN_DET_TEST_V: c_uint = WSA881X_ANALOG_BASE + 0x0012;
const WSA881X_ADC_EN_DET_TEST_I: c_uint = WSA881X_ANALOG_BASE + 0x0013;
const WSA881X_ADC_SEL_IBIAS: c_uint = WSA881X_ANALOG_BASE + 0x0014;
const WSA881X_ADC_EN_SEL_IBAIS: c_uint = WSA881X_ANALOG_BASE + 0x0015;
const WSA881X_SPKR_DRV_EN: c_uint = WSA881X_ANALOG_BASE + 0x001A;
const WSA881X_SPKR_DRV_GAIN: c_uint = WSA881X_ANALOG_BASE + 0x001B;
const WSA881X_PA_GAIN_SEL_MASK: c_uint = BIT(3);
const WSA881X_PA_GAIN_SEL_REG: c_uint = BIT(3);
const WSA881X_PA_GAIN_SEL_DRE: c_uint = 0;
const WSA881X_SPKR_PAG_GAIN_MASK: c_uint = GENMASK(7, 4);
const WSA881X_SPKR_DAC_CTL: c_uint = WSA881X_ANALOG_BASE + 0x001C;
const WSA881X_SPKR_DRV_DBG: c_uint = WSA881X_ANALOG_BASE + 0x001D;
const WSA881X_SPKR_PWRSTG_DBG: c_uint = WSA881X_ANALOG_BASE + 0x001E;
const WSA881X_SPKR_OCP_CTL: c_uint = WSA881X_ANALOG_BASE + 0x001F;
const WSA881X_SPKR_OCP_MASK: c_uint = GENMASK(7, 6);
const WSA881X_SPKR_OCP_EN: c_uint = BIT(7);
const WSA881X_SPKR_OCP_HOLD: c_uint = BIT(6);
const WSA881X_SPKR_CLIP_CTL: c_uint = WSA881X_ANALOG_BASE + 0x0020;
const WSA881X_SPKR_BBM_CTL: c_uint = WSA881X_ANALOG_BASE + 0x0021;
const WSA881X_SPKR_MISC_CTL1: c_uint = WSA881X_ANALOG_BASE + 0x0022;
const WSA881X_SPKR_MISC_CTL2: c_uint = WSA881X_ANALOG_BASE + 0x0023;
const WSA881X_SPKR_BIAS_INT: c_uint = WSA881X_ANALOG_BASE + 0x0024;
const WSA881X_SPKR_PA_INT: c_uint = WSA881X_ANALOG_BASE + 0x0025;
const WSA881X_SPKR_BIAS_CAL: c_uint = WSA881X_ANALOG_BASE + 0x0026;
const WSA881X_SPKR_BIAS_PSRR: c_uint = WSA881X_ANALOG_BASE + 0x0027;
const WSA881X_SPKR_STATUS1: c_uint = WSA881X_ANALOG_BASE + 0x0028;
const WSA881X_SPKR_STATUS2: c_uint = WSA881X_ANALOG_BASE + 0x0029;
const WSA881X_BOOST_EN_CTL: c_uint = WSA881X_ANALOG_BASE + 0x002A;
const WSA881X_BOOST_EN_MASK: c_uint = BIT(7);
const WSA881X_BOOST_EN: c_uint = BIT(7);
const WSA881X_BOOST_CURRENT_LIMIT: c_uint = WSA881X_ANALOG_BASE + 0x002B;
const WSA881X_BOOST_PS_CTL: c_uint = WSA881X_ANALOG_BASE + 0x002C;
const WSA881X_BOOST_PRESET_OUT1: c_uint = WSA881X_ANALOG_BASE + 0x002D;
const WSA881X_BOOST_PRESET_OUT2: c_uint = WSA881X_ANALOG_BASE + 0x002E;
const WSA881X_BOOST_FORCE_OUT: c_uint = WSA881X_ANALOG_BASE + 0x002F;
const WSA881X_BOOST_LDO_PROG: c_uint = WSA881X_ANALOG_BASE + 0x0030;
const WSA881X_BOOST_SLOPE_COMP_ISENSE_FB: c_uint = WSA881X_ANALOG_BASE + 0x0031;
const WSA881X_BOOST_RON_CTL: c_uint = WSA881X_ANALOG_BASE + 0x0032;
const WSA881X_BOOST_LOOP_STABILITY: c_uint = WSA881X_ANALOG_BASE + 0x0033;
const WSA881X_BOOST_ZX_CTL: c_uint = WSA881X_ANALOG_BASE + 0x0034;
const WSA881X_BOOST_START_CTL: c_uint = WSA881X_ANALOG_BASE + 0x0035;
const WSA881X_BOOST_MISC1_CTL: c_uint = WSA881X_ANALOG_BASE + 0x0036;
const WSA881X_BOOST_MISC2_CTL: c_uint = WSA881X_ANALOG_BASE + 0x0037;
const WSA881X_BOOST_MISC3_CTL: c_uint = WSA881X_ANALOG_BASE + 0x0038;
const WSA881X_BOOST_ATEST_CTL: c_uint = WSA881X_ANALOG_BASE + 0x0039;
const WSA881X_SPKR_PROT_FE_GAIN: c_uint = WSA881X_ANALOG_BASE + 0x003A;
const WSA881X_SPKR_PROT_FE_CM_LDO_SET: c_uint = WSA881X_ANALOG_BASE + 0x003B;
const WSA881X_SPKR_PROT_FE_ISENSE_BIAS_SET1: c_uint = WSA881X_ANALOG_BASE + 0x003C;
const WSA881X_SPKR_PROT_FE_ISENSE_BIAS_SET2: c_uint = WSA881X_ANALOG_BASE + 0x003D;
const WSA881X_SPKR_PROT_ATEST1: c_uint = WSA881X_ANALOG_BASE + 0x003E;
const WSA881X_SPKR_PROT_ATEST2: c_uint = WSA881X_ANALOG_BASE + 0x003F;
const WSA881X_SPKR_PROT_FE_VSENSE_VCM: c_uint = WSA881X_ANALOG_BASE + 0x0040;
const WSA881X_SPKR_PROT_FE_VSENSE_BIAS_SET1: c_uint = WSA881X_ANALOG_BASE + 0x0041;
const WSA881X_BONGO_RESRV_REG1: c_uint = WSA881X_ANALOG_BASE + 0x0042;
const WSA881X_BONGO_RESRV_REG2: c_uint = WSA881X_ANALOG_BASE + 0x0043;
const WSA881X_SPKR_PROT_SAR: c_uint = WSA881X_ANALOG_BASE + 0x0044;
const WSA881X_SPKR_STATUS3: c_uint = WSA881X_ANALOG_BASE + 0x0045;

const fn SWRS_SCP_FRAME_CTRL_BANK(m: c_uint) -> c_uint {
    0x60 + 0x10 * m
}
const fn SWRS_SCP_HOST_CLK_DIV2_CTL_BANK(m: c_uint) -> c_uint {
    0xE0 + 0x10 * m
}
const SWR_SLV_MAX_REG_ADDR: c_uint = 0x390;
const SWR_SLV_START_REG_ADDR: c_uint = 0x40;
const SWR_SLV_MAX_BUF_LEN: c_uint = 20;
const BYTES_PER_LINE: c_uint = 12;
const SWR_SLV_RD_BUF_LEN: c_uint = 8;
const SWR_SLV_WR_BUF_LEN: c_uint = 32;
const SWR_SLV_MAX_DEVICES: c_uint = 2;
const WSA881X_MAX_SWR_PORTS: usize = 4;
const WSA881X_VERSION_ENTRY_SIZE: c_uint = 27;
const WSA881X_OCP_CTL_TIMER_SEC: c_uint = 2;
const WSA881X_OCP_CTL_TEMP_CELSIUS: c_uint = 25;
const WSA881X_OCP_CTL_POLL_TIMER_SEC: c_uint = 60;
const WSA881X_PROBE_TIMEOUT: c_int = 1000;

// WSA881X_PA_GAIN_TLV maps to SOC_SINGLE_EXT_TLV(..., snd_soc_get_volsw,
// wsa881x_put_pa_gain, ...); ALSA control macro expansion is external.

static wsa881x_defaults: [reg_default; 102] = [
    reg_default { reg: WSA881X_CHIP_ID0, def: 0x00 }, reg_default { reg: WSA881X_CHIP_ID1, def: 0x00 },
    reg_default { reg: WSA881X_CHIP_ID2, def: 0x00 }, reg_default { reg: WSA881X_CHIP_ID3, def: 0x02 },
    reg_default { reg: WSA881X_BUS_ID, def: 0x00 }, reg_default { reg: WSA881X_CDC_RST_CTL, def: 0x00 },
    reg_default { reg: WSA881X_CDC_TOP_CLK_CTL, def: 0x03 }, reg_default { reg: WSA881X_CDC_ANA_CLK_CTL, def: 0x00 },
    reg_default { reg: WSA881X_CDC_DIG_CLK_CTL, def: 0x00 }, reg_default { reg: WSA881X_CLOCK_CONFIG, def: 0x00 },
    reg_default { reg: WSA881X_ANA_CTL, def: 0x08 }, reg_default { reg: WSA881X_SWR_RESET_EN, def: 0x00 },
    reg_default { reg: WSA881X_TEMP_DETECT_CTL, def: 0x01 }, reg_default { reg: WSA881X_TEMP_MSB, def: 0x00 },
    reg_default { reg: WSA881X_TEMP_LSB, def: 0x00 }, reg_default { reg: WSA881X_TEMP_CONFIG0, def: 0x00 },
    reg_default { reg: WSA881X_TEMP_CONFIG1, def: 0x00 }, reg_default { reg: WSA881X_CDC_CLIP_CTL, def: 0x03 },
    reg_default { reg: WSA881X_SDM_PDM9_LSB, def: 0x00 }, reg_default { reg: WSA881X_SDM_PDM9_MSB, def: 0x00 },
    reg_default { reg: WSA881X_CDC_RX_CTL, def: 0x7E }, reg_default { reg: WSA881X_DEM_BYPASS_DATA0, def: 0x00 },
    reg_default { reg: WSA881X_DEM_BYPASS_DATA1, def: 0x00 }, reg_default { reg: WSA881X_DEM_BYPASS_DATA2, def: 0x00 },
    reg_default { reg: WSA881X_DEM_BYPASS_DATA3, def: 0x00 }, reg_default { reg: WSA881X_OTP_CTRL0, def: 0x00 },
    reg_default { reg: WSA881X_OTP_CTRL1, def: 0x00 }, reg_default { reg: WSA881X_HDRIVE_CTL_GROUP1, def: 0x00 },
    reg_default { reg: WSA881X_INTR_MODE, def: 0x00 }, reg_default { reg: WSA881X_INTR_STATUS, def: 0x00 },
    reg_default { reg: WSA881X_INTR_CLEAR, def: 0x00 }, reg_default { reg: WSA881X_INTR_LEVEL, def: 0x00 },
    reg_default { reg: WSA881X_INTR_SET, def: 0x00 }, reg_default { reg: WSA881X_INTR_TEST, def: 0x00 },
    reg_default { reg: WSA881X_PDM_TEST_MODE, def: 0x00 }, reg_default { reg: WSA881X_ATE_TEST_MODE, def: 0x00 },
    reg_default { reg: WSA881X_PIN_CTL_MODE, def: 0x00 }, reg_default { reg: WSA881X_PIN_CTL_OE, def: 0x00 },
    reg_default { reg: WSA881X_PIN_WDATA_IOPAD, def: 0x00 }, reg_default { reg: WSA881X_PIN_STATUS, def: 0x00 },
    reg_default { reg: WSA881X_DIG_DEBUG_MODE, def: 0x00 }, reg_default { reg: WSA881X_DIG_DEBUG_SEL, def: 0x00 },
    reg_default { reg: WSA881X_DIG_DEBUG_EN, def: 0x00 }, reg_default { reg: WSA881X_SWR_HM_TEST1, def: 0x08 },
    reg_default { reg: WSA881X_SWR_HM_TEST2, def: 0x00 }, reg_default { reg: WSA881X_TEMP_DETECT_DBG_CTL, def: 0x00 },
    reg_default { reg: WSA881X_TEMP_DEBUG_MSB, def: 0x00 }, reg_default { reg: WSA881X_TEMP_DEBUG_LSB, def: 0x00 },
    reg_default { reg: WSA881X_SAMPLE_EDGE_SEL, def: 0x0C }, reg_default { reg: WSA881X_SPARE_0, def: 0x00 },
    reg_default { reg: WSA881X_SPARE_1, def: 0x00 }, reg_default { reg: WSA881X_SPARE_2, def: 0x00 },
    reg_default { reg: WSA881X_OTP_REG_0, def: 0x01 }, reg_default { reg: WSA881X_OTP_REG_1, def: 0xFF },
    reg_default { reg: WSA881X_OTP_REG_2, def: 0xC0 }, reg_default { reg: WSA881X_OTP_REG_3, def: 0xFF },
    reg_default { reg: WSA881X_OTP_REG_4, def: 0xC0 }, reg_default { reg: WSA881X_OTP_REG_5, def: 0xFF },
    reg_default { reg: WSA881X_OTP_REG_6, def: 0xFF }, reg_default { reg: WSA881X_OTP_REG_7, def: 0xFF },
    reg_default { reg: WSA881X_OTP_REG_8, def: 0xFF }, reg_default { reg: WSA881X_OTP_REG_9, def: 0xFF },
    reg_default { reg: WSA881X_OTP_REG_10, def: 0xFF }, reg_default { reg: WSA881X_OTP_REG_11, def: 0xFF },
    reg_default { reg: WSA881X_OTP_REG_12, def: 0xFF }, reg_default { reg: WSA881X_OTP_REG_13, def: 0xFF },
    reg_default { reg: WSA881X_OTP_REG_14, def: 0xFF }, reg_default { reg: WSA881X_OTP_REG_15, def: 0xFF },
    reg_default { reg: WSA881X_OTP_REG_16, def: 0xFF }, reg_default { reg: WSA881X_OTP_REG_17, def: 0xFF },
    reg_default { reg: WSA881X_OTP_REG_18, def: 0xFF }, reg_default { reg: WSA881X_OTP_REG_19, def: 0xFF },
    reg_default { reg: WSA881X_OTP_REG_20, def: 0xFF }, reg_default { reg: WSA881X_OTP_REG_21, def: 0xFF },
    reg_default { reg: WSA881X_OTP_REG_22, def: 0xFF }, reg_default { reg: WSA881X_OTP_REG_23, def: 0xFF },
    reg_default { reg: WSA881X_OTP_REG_24, def: 0x03 }, reg_default { reg: WSA881X_OTP_REG_25, def: 0x01 },
    reg_default { reg: WSA881X_OTP_REG_26, def: 0x03 }, reg_default { reg: WSA881X_OTP_REG_27, def: 0x11 },
    reg_default { reg: WSA881X_OTP_REG_63, def: 0x40 }, reg_default { reg: WSA881X_BIAS_REF_CTRL, def: 0x6C },
    reg_default { reg: WSA881X_BIAS_TEST, def: 0x16 }, reg_default { reg: WSA881X_BIAS_BIAS, def: 0xF0 },
    reg_default { reg: WSA881X_TEMP_OP, def: 0x00 }, reg_default { reg: WSA881X_TEMP_IREF_CTRL, def: 0x56 },
    reg_default { reg: WSA881X_TEMP_ISENS_CTRL, def: 0x47 }, reg_default { reg: WSA881X_TEMP_CLK_CTRL, def: 0x87 },
    reg_default { reg: WSA881X_TEMP_TEST, def: 0x00 }, reg_default { reg: WSA881X_TEMP_BIAS, def: 0x51 },
    reg_default { reg: WSA881X_TEMP_DOUT_MSB, def: 0x00 }, reg_default { reg: WSA881X_TEMP_DOUT_LSB, def: 0x00 },
    reg_default { reg: WSA881X_ADC_EN_MODU_V, def: 0x00 }, reg_default { reg: WSA881X_ADC_EN_MODU_I, def: 0x00 },
    reg_default { reg: WSA881X_ADC_EN_DET_TEST_V, def: 0x00 }, reg_default { reg: WSA881X_ADC_EN_DET_TEST_I, def: 0x00 },
    reg_default { reg: WSA881X_ADC_EN_SEL_IBAIS, def: 0x10 }, reg_default { reg: WSA881X_SPKR_DRV_EN, def: 0x74 },
    reg_default { reg: WSA881X_SPKR_DRV_DBG, def: 0x15 }, reg_default { reg: WSA881X_SPKR_PWRSTG_DBG, def: 0x00 },
    reg_default { reg: WSA881X_SPKR_OCP_CTL, def: 0xD4 }, reg_default { reg: WSA881X_SPKR_CLIP_CTL, def: 0x90 },
];

static wsa881x_pre_pmu_pa_2_0: [reg_sequence; 2] = [
    reg_sequence { reg: WSA881X_SPKR_DRV_GAIN, def: 0x41, delay_us: 0 },
    reg_sequence { reg: WSA881X_SPKR_MISC_CTL1, def: 0x87, delay_us: 0 },
];

static wsa881x_vi_txfe_en_2_0: [reg_sequence; 3] = [
    reg_sequence { reg: WSA881X_SPKR_PROT_FE_VSENSE_VCM, def: 0x85, delay_us: 0 },
    reg_sequence { reg: WSA881X_SPKR_PROT_ATEST2, def: 0x0A, delay_us: 0 },
    reg_sequence { reg: WSA881X_SPKR_PROT_FE_GAIN, def: 0x47, delay_us: 0 },
];

/* Default register reset values for WSA881x rev 2.0 */
static wsa881x_rev_2_0: [reg_sequence; 23] = [
    reg_sequence { reg: WSA881X_RESET_CTL, def: 0x00, delay_us: 0x00 },
    reg_sequence { reg: WSA881X_TADC_VALUE_CTL, def: 0x01, delay_us: 0x00 },
    reg_sequence { reg: WSA881X_INTR_MASK, def: 0x1B, delay_us: 0x00 },
    reg_sequence { reg: WSA881X_IOPAD_CTL, def: 0x00, delay_us: 0x00 },
    reg_sequence { reg: WSA881X_OTP_REG_28, def: 0x3F, delay_us: 0x00 },
    reg_sequence { reg: WSA881X_OTP_REG_29, def: 0x3F, delay_us: 0x00 },
    reg_sequence { reg: WSA881X_OTP_REG_30, def: 0x01, delay_us: 0x00 },
    reg_sequence { reg: WSA881X_OTP_REG_31, def: 0x01, delay_us: 0x00 },
    reg_sequence { reg: WSA881X_TEMP_ADC_CTRL, def: 0x03, delay_us: 0x00 },
    reg_sequence { reg: WSA881X_ADC_SEL_IBIAS, def: 0x45, delay_us: 0x00 },
    reg_sequence { reg: WSA881X_SPKR_DRV_GAIN, def: 0xC1, delay_us: 0x00 },
    reg_sequence { reg: WSA881X_SPKR_DAC_CTL, def: 0x42, delay_us: 0x00 },
    reg_sequence { reg: WSA881X_SPKR_BBM_CTL, def: 0x02, delay_us: 0x00 },
    reg_sequence { reg: WSA881X_SPKR_MISC_CTL1, def: 0x40, delay_us: 0x00 },
    reg_sequence { reg: WSA881X_SPKR_MISC_CTL2, def: 0x07, delay_us: 0x00 },
    reg_sequence { reg: WSA881X_SPKR_BIAS_INT, def: 0x5F, delay_us: 0x00 },
    reg_sequence { reg: WSA881X_SPKR_BIAS_PSRR, def: 0x44, delay_us: 0x00 },
    reg_sequence { reg: WSA881X_BOOST_PS_CTL, def: 0xA0, delay_us: 0x00 },
    reg_sequence { reg: WSA881X_BOOST_PRESET_OUT1, def: 0xB7, delay_us: 0x00 },
    reg_sequence { reg: WSA881X_BOOST_LOOP_STABILITY, def: 0x8D, delay_us: 0x00 },
    reg_sequence { reg: WSA881X_SPKR_PROT_ATEST2, def: 0x02, delay_us: 0x00 },
    reg_sequence { reg: WSA881X_BONGO_RESRV_REG1, def: 0x5E, delay_us: 0x00 },
    reg_sequence { reg: WSA881X_BONGO_RESRV_REG2, def: 0x07, delay_us: 0x00 },
];

const WSA881X_PORT_DAC: usize = 0;
const WSA881X_PORT_COMP: usize = 1;
const WSA881X_PORT_BOOST: usize = 2;
const WSA881X_PORT_VISENSE: usize = 3;

/* 4 ports */
static mut wsa_sink_dpn_prop: [sdw_dpn_prop; WSA881X_MAX_SWR_PORTS] = [
    sdw_dpn_prop { num: WSA881X_PORT_DAC as c_uint + 1, type_: SDW_DPN_SIMPLE, min_ch: 1, max_ch: 1, simple_ch_prep_sm: true, read_only_wordlength: true },
    sdw_dpn_prop { num: WSA881X_PORT_COMP as c_uint + 1, type_: SDW_DPN_SIMPLE, min_ch: 1, max_ch: 1, simple_ch_prep_sm: true, read_only_wordlength: true },
    sdw_dpn_prop { num: WSA881X_PORT_BOOST as c_uint + 1, type_: SDW_DPN_SIMPLE, min_ch: 1, max_ch: 1, simple_ch_prep_sm: true, read_only_wordlength: true },
    sdw_dpn_prop { num: WSA881X_PORT_VISENSE as c_uint + 1, type_: SDW_DPN_SIMPLE, min_ch: 1, max_ch: 1, simple_ch_prep_sm: true, read_only_wordlength: true },
];

static wsa881x_pconfig: [sdw_port_config; WSA881X_MAX_SWR_PORTS] = [
    sdw_port_config { num: WSA881X_PORT_DAC as c_uint + 1, ch_mask: 0x1 },
    sdw_port_config { num: WSA881X_PORT_COMP as c_uint + 1, ch_mask: 0xf },
    sdw_port_config { num: WSA881X_PORT_BOOST as c_uint + 1, ch_mask: 0x3 },
    sdw_port_config { num: WSA881X_PORT_VISENSE as c_uint + 1, ch_mask: 0x3 },
];

unsafe extern "C" fn wsa881x_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    matches!(reg,
        WSA881X_CHIP_ID0 | WSA881X_CHIP_ID1 | WSA881X_CHIP_ID2 | WSA881X_CHIP_ID3 |
        WSA881X_BUS_ID | WSA881X_CDC_RST_CTL | WSA881X_CDC_TOP_CLK_CTL |
        WSA881X_CDC_ANA_CLK_CTL | WSA881X_CDC_DIG_CLK_CTL | WSA881X_CLOCK_CONFIG |
        WSA881X_ANA_CTL | WSA881X_SWR_RESET_EN | WSA881X_RESET_CTL |
        WSA881X_TADC_VALUE_CTL | WSA881X_TEMP_DETECT_CTL | WSA881X_TEMP_MSB |
        WSA881X_TEMP_LSB | WSA881X_TEMP_CONFIG0 | WSA881X_TEMP_CONFIG1 |
        WSA881X_CDC_CLIP_CTL | WSA881X_SDM_PDM9_LSB | WSA881X_SDM_PDM9_MSB |
        WSA881X_CDC_RX_CTL | WSA881X_DEM_BYPASS_DATA0 | WSA881X_DEM_BYPASS_DATA1 |
        WSA881X_DEM_BYPASS_DATA2 | WSA881X_DEM_BYPASS_DATA3 | WSA881X_OTP_CTRL0 |
        WSA881X_OTP_CTRL1 | WSA881X_HDRIVE_CTL_GROUP1 | WSA881X_INTR_MODE |
        WSA881X_INTR_MASK | WSA881X_INTR_STATUS | WSA881X_INTR_CLEAR |
        WSA881X_INTR_LEVEL | WSA881X_INTR_SET | WSA881X_INTR_TEST |
        WSA881X_PDM_TEST_MODE | WSA881X_ATE_TEST_MODE | WSA881X_PIN_CTL_MODE |
        WSA881X_PIN_CTL_OE | WSA881X_PIN_WDATA_IOPAD | WSA881X_PIN_STATUS |
        WSA881X_DIG_DEBUG_MODE | WSA881X_DIG_DEBUG_SEL | WSA881X_DIG_DEBUG_EN |
        WSA881X_SWR_HM_TEST1 | WSA881X_SWR_HM_TEST2 | WSA881X_TEMP_DETECT_DBG_CTL |
        WSA881X_TEMP_DEBUG_MSB | WSA881X_TEMP_DEBUG_LSB | WSA881X_SAMPLE_EDGE_SEL |
        WSA881X_IOPAD_CTL | WSA881X_SPARE_0 | WSA881X_SPARE_1 | WSA881X_SPARE_2 |
        WSA881X_OTP_REG_0 | WSA881X_OTP_REG_1 | WSA881X_OTP_REG_2 | WSA881X_OTP_REG_3 |
        WSA881X_OTP_REG_4 | WSA881X_OTP_REG_5 | WSA881X_OTP_REG_6 | WSA881X_OTP_REG_7 |
        WSA881X_OTP_REG_8 | WSA881X_OTP_REG_9 | WSA881X_OTP_REG_10 | WSA881X_OTP_REG_11 |
        WSA881X_OTP_REG_12 | WSA881X_OTP_REG_13 | WSA881X_OTP_REG_14 | WSA881X_OTP_REG_15 |
        WSA881X_OTP_REG_16 | WSA881X_OTP_REG_17 | WSA881X_OTP_REG_18 | WSA881X_OTP_REG_19 |
        WSA881X_OTP_REG_20 | WSA881X_OTP_REG_21 | WSA881X_OTP_REG_22 | WSA881X_OTP_REG_23 |
        WSA881X_OTP_REG_24 | WSA881X_OTP_REG_25 | WSA881X_OTP_REG_26 | WSA881X_OTP_REG_27 |
        WSA881X_OTP_REG_28 | WSA881X_OTP_REG_29 | WSA881X_OTP_REG_30 | WSA881X_OTP_REG_31 |
        WSA881X_OTP_REG_63 | WSA881X_BIAS_REF_CTRL | WSA881X_BIAS_TEST | WSA881X_BIAS_BIAS |
        WSA881X_TEMP_OP | WSA881X_TEMP_IREF_CTRL | WSA881X_TEMP_ISENS_CTRL | WSA881X_TEMP_CLK_CTRL |
        WSA881X_TEMP_TEST | WSA881X_TEMP_BIAS | WSA881X_TEMP_ADC_CTRL | WSA881X_TEMP_DOUT_MSB |
        WSA881X_TEMP_DOUT_LSB | WSA881X_ADC_EN_MODU_V | WSA881X_ADC_EN_MODU_I |
        WSA881X_ADC_EN_DET_TEST_V | WSA881X_ADC_EN_DET_TEST_I | WSA881X_ADC_SEL_IBIAS |
        WSA881X_ADC_EN_SEL_IBAIS | WSA881X_SPKR_DRV_EN | WSA881X_SPKR_DRV_GAIN |
        WSA881X_SPKR_DAC_CTL | WSA881X_SPKR_DRV_DBG | WSA881X_SPKR_PWRSTG_DBG |
        WSA881X_SPKR_OCP_CTL | WSA881X_SPKR_CLIP_CTL | WSA881X_SPKR_BBM_CTL |
        WSA881X_SPKR_MISC_CTL1 | WSA881X_SPKR_MISC_CTL2 | WSA881X_SPKR_BIAS_INT |
        WSA881X_SPKR_PA_INT | WSA881X_SPKR_BIAS_CAL | WSA881X_SPKR_BIAS_PSRR |
        WSA881X_SPKR_STATUS1 | WSA881X_SPKR_STATUS2 | WSA881X_BOOST_EN_CTL |
        WSA881X_BOOST_CURRENT_LIMIT | WSA881X_BOOST_PS_CTL | WSA881X_BOOST_PRESET_OUT1 |
        WSA881X_BOOST_PRESET_OUT2 | WSA881X_BOOST_FORCE_OUT | WSA881X_BOOST_LDO_PROG |
        WSA881X_BOOST_SLOPE_COMP_ISENSE_FB | WSA881X_BOOST_RON_CTL | WSA881X_BOOST_LOOP_STABILITY |
        WSA881X_BOOST_ZX_CTL | WSA881X_BOOST_START_CTL | WSA881X_BOOST_MISC1_CTL |
        WSA881X_BOOST_MISC2_CTL | WSA881X_BOOST_MISC3_CTL | WSA881X_BOOST_ATEST_CTL |
        WSA881X_SPKR_PROT_FE_GAIN | WSA881X_SPKR_PROT_FE_CM_LDO_SET |
        WSA881X_SPKR_PROT_FE_ISENSE_BIAS_SET1 | WSA881X_SPKR_PROT_FE_ISENSE_BIAS_SET2 |
        WSA881X_SPKR_PROT_ATEST1 | WSA881X_SPKR_PROT_ATEST2 | WSA881X_SPKR_PROT_FE_VSENSE_VCM |
        WSA881X_SPKR_PROT_FE_VSENSE_BIAS_SET1 | WSA881X_BONGO_RESRV_REG1 |
        WSA881X_BONGO_RESRV_REG2 | WSA881X_SPKR_PROT_SAR | WSA881X_SPKR_STATUS3)
}

unsafe extern "C" fn wsa881x_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    matches!(reg,
        WSA881X_CHIP_ID0 | WSA881X_CHIP_ID1 | WSA881X_CHIP_ID2 | WSA881X_CHIP_ID3 |
        WSA881X_BUS_ID | WSA881X_TEMP_MSB | WSA881X_TEMP_LSB | WSA881X_SDM_PDM9_LSB |
        WSA881X_SDM_PDM9_MSB | WSA881X_OTP_CTRL1 | WSA881X_INTR_STATUS |
        WSA881X_ATE_TEST_MODE | WSA881X_PIN_STATUS | WSA881X_SWR_HM_TEST2 |
        WSA881X_SPKR_STATUS1 | WSA881X_SPKR_STATUS2 | WSA881X_SPKR_STATUS3 |
        WSA881X_OTP_REG_0 | WSA881X_OTP_REG_1 | WSA881X_OTP_REG_2 |
        WSA881X_OTP_REG_3 | WSA881X_OTP_REG_4 | WSA881X_OTP_REG_5 |
        WSA881X_OTP_REG_31 | WSA881X_TEMP_DOUT_MSB | WSA881X_TEMP_DOUT_LSB |
        WSA881X_TEMP_OP | WSA881X_SPKR_PROT_SAR)
}

static wsa881x_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 8,
    cache_type: REGCACHE_MAPLE,
    reg_defaults: wsa881x_defaults.as_ptr(),
    max_register: WSA881X_SPKR_STATUS3,
    num_reg_defaults: wsa881x_defaults.len(),
    volatile_reg: Some(wsa881x_volatile_register),
    readable_reg: Some(wsa881x_readable_register),
    reg_format_endian: REGMAP_ENDIAN_NATIVE,
    val_format_endian: REGMAP_ENDIAN_NATIVE,
};

const G_18DB: c_int = 0;
const G_16P5DB: c_int = 1;
const G_15DB: c_int = 2;
const G_13P5DB: c_int = 3;
const G_12DB: c_int = 4;
const G_10P5DB: c_int = 5;
const G_9DB: c_int = 6;
const G_7P5DB: c_int = 7;
const G_6DB: c_int = 8;
const G_4P5DB: c_int = 9;
const G_3DB: c_int = 10;
const G_1P5DB: c_int = 11;
const G_0DB: c_int = 12;

/*
 * Private data Structure for wsa881x. All parameters related to
 * WSA881X codec needs to be defined here.
 */
#[repr(C)]
pub struct wsa881x_priv {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub slave: *mut sdw_slave,
    pub sconfig: sdw_stream_config,
    pub sruntime: *mut sdw_stream_runtime,
    pub port_config: [sdw_port_config; WSA881X_MAX_SWR_PORTS],
    pub sd_n: *mut gpio_desc,
    pub active_ports: c_int,
    pub hw_init: bool,
    pub port_prepared: [bool; WSA881X_MAX_SWR_PORTS],
    pub port_enable: [bool; WSA881X_MAX_SWR_PORTS],
}

unsafe fn wsa881x_init(wsa881x: *mut wsa881x_priv) {
    let rm = (*wsa881x).regmap;
    let mut val: c_uint = 0;

    if (*wsa881x).hw_init {
        return;
    }

    regmap_register_patch((*wsa881x).regmap, wsa881x_rev_2_0.as_ptr(), wsa881x_rev_2_0.len());

    /* Enable software reset output from soundwire slave */
    regmap_update_bits(rm, WSA881X_SWR_RESET_EN, 0x07, 0x07);

    /* Bring out of analog reset */
    regmap_update_bits(rm, WSA881X_CDC_RST_CTL, 0x02, 0x02);

    /* Bring out of digital reset */
    regmap_update_bits(rm, WSA881X_CDC_RST_CTL, 0x01, 0x01);
    regmap_update_bits(rm, WSA881X_CLOCK_CONFIG, 0x10, 0x10);
    regmap_update_bits(rm, WSA881X_SPKR_OCP_CTL, 0x02, 0x02);
    regmap_update_bits(rm, WSA881X_SPKR_MISC_CTL1, 0xC0, 0x80);
    regmap_update_bits(rm, WSA881X_SPKR_MISC_CTL1, 0x06, 0x06);
    regmap_update_bits(rm, WSA881X_SPKR_BIAS_INT, 0xFF, 0x00);
    regmap_update_bits(rm, WSA881X_SPKR_PA_INT, 0xF0, 0x40);
    regmap_update_bits(rm, WSA881X_SPKR_PA_INT, 0x0E, 0x0E);
    regmap_update_bits(rm, WSA881X_BOOST_LOOP_STABILITY, 0x03, 0x03);
    regmap_update_bits(rm, WSA881X_BOOST_MISC2_CTL, 0xFF, 0x14);
    regmap_update_bits(rm, WSA881X_BOOST_START_CTL, 0x80, 0x80);
    regmap_update_bits(rm, WSA881X_BOOST_START_CTL, 0x03, 0x00);
    regmap_update_bits(rm, WSA881X_BOOST_SLOPE_COMP_ISENSE_FB, 0x0C, 0x04);
    regmap_update_bits(rm, WSA881X_BOOST_SLOPE_COMP_ISENSE_FB, 0x03, 0x00);

    regmap_read(rm, WSA881X_OTP_REG_0, &mut val);
    if val != 0 {
        regmap_update_bits(rm, WSA881X_BOOST_PRESET_OUT1, 0xF0, 0x70);
    }

    regmap_update_bits(rm, WSA881X_BOOST_PRESET_OUT2, 0xF0, 0x30);
    regmap_update_bits(rm, WSA881X_SPKR_DRV_EN, 0x08, 0x08);
    regmap_update_bits(rm, WSA881X_BOOST_CURRENT_LIMIT, 0x0F, 0x08);
    regmap_update_bits(rm, WSA881X_SPKR_OCP_CTL, 0x30, 0x30);
    regmap_update_bits(rm, WSA881X_SPKR_OCP_CTL, 0x0C, 0x00);
    regmap_update_bits(rm, WSA881X_OTP_REG_28, 0x3F, 0x3A);
    regmap_update_bits(rm, WSA881X_BONGO_RESRV_REG1, 0xFF, 0xB2);
    regmap_update_bits(rm, WSA881X_BONGO_RESRV_REG2, 0xFF, 0x05);

    (*wsa881x).hw_init = true;
}

unsafe extern "C" fn wsa881x_component_probe(comp: *mut snd_soc_component) -> c_int {
    let wsa881x = snd_soc_component_get_drvdata(comp) as *mut wsa881x_priv;
    snd_soc_component_init_regmap(comp, (*wsa881x).regmap);
    0
}

unsafe extern "C" fn wsa881x_put_pa_gain(kc: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kc);
    let mc = (*kc).private_value as *mut soc_mixer_control;
    let max = (*mc).max;
    let mask: c_uint = ((1 << fls(max)) - 1) as c_uint;
    let mut ret: c_int;
    let min_gain: c_int;

    ret = pm_runtime_resume_and_get((*comp).dev);
    if ret < 0 && ret != -EACCES {
        return ret;
    }

    let max_gain = ((max as c_long - (*ucontrol).value.integer.value[0]) as c_int & mask as c_int) as c_int;
    /*
     * Gain has to set incrementally in 4 steps
     * as per HW sequence
     */
    if max_gain > G_4P5DB {
        min_gain = G_0DB;
    } else {
        min_gain = max_gain + 3;
    }
    /*
     * 1ms delay is needed before change in gain
     * as per HW requirement.
     */
    usleep_range(1000, 1010);

    let mut val = min_gain;
    while max_gain <= val {
        ret = snd_soc_component_update_bits(comp, WSA881X_SPKR_DRV_GAIN, WSA881X_SPKR_PAG_GAIN_MASK, (val << 4) as c_uint);
        if ret < 0 {
            dev_err((*comp).dev, b"Failed to change PA gain\0".as_ptr() as *const c_char);
        }
        usleep_range(1000, 1010);
        val -= 1;
    }

    pm_runtime_put_autosuspend((*comp).dev);
    1
}

unsafe extern "C" fn wsa881x_get_port(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let data = snd_soc_component_get_drvdata(comp) as *mut wsa881x_priv;
    let mixer = (*kcontrol).private_value as *mut soc_mixer_control;
    let portidx = (*mixer).reg as usize;

    (*ucontrol).value.integer.value[0] = (*data).port_enable[portidx] as c_long;
    0
}

unsafe fn wsa881x_boost_ctrl(comp: *mut snd_soc_component, enable: bool) -> c_int {
    if enable {
        snd_soc_component_update_bits(comp, WSA881X_BOOST_EN_CTL, WSA881X_BOOST_EN_MASK, WSA881X_BOOST_EN);
    } else {
        snd_soc_component_update_bits(comp, WSA881X_BOOST_EN_CTL, WSA881X_BOOST_EN_MASK, 0);
    }
    /*
     * 1.5ms sleep is needed after boost enable/disable as per
     * HW requirement
     */
    usleep_range(1500, 1510);
    0
}

unsafe extern "C" fn wsa881x_set_port(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let data = snd_soc_component_get_drvdata(comp) as *mut wsa881x_priv;
    let mixer = (*kcontrol).private_value as *mut soc_mixer_control;
    let portidx = (*mixer).reg as usize;

    if (*ucontrol).value.integer.value[0] != 0 {
        if (*data).port_enable[portidx] {
            return 0;
        }
        (*data).port_enable[portidx] = true;
    } else {
        if !(*data).port_enable[portidx] {
            return 0;
        }
        (*data).port_enable[portidx] = false;
    }

    if portidx == WSA881X_PORT_BOOST {
        /* Boost Switch */
        wsa881x_boost_ctrl(comp, (*data).port_enable[portidx]);
    }
    1
}

static smart_boost_lvl_text: [&[u8]; 16] = [
    b"6.625 V\0", b"6.750 V\0", b"6.875 V\0", b"7.000 V\0",
    b"7.125 V\0", b"7.250 V\0", b"7.375 V\0", b"7.500 V\0",
    b"7.625 V\0", b"7.750 V\0", b"7.875 V\0", b"8.000 V\0",
    b"8.125 V\0", b"8.250 V\0", b"8.375 V\0", b"8.500 V\0",
];

static smart_boost_lvl_text_ptrs: [*const c_char; 16] = [
    smart_boost_lvl_text[0].as_ptr() as *const c_char, smart_boost_lvl_text[1].as_ptr() as *const c_char,
    smart_boost_lvl_text[2].as_ptr() as *const c_char, smart_boost_lvl_text[3].as_ptr() as *const c_char,
    smart_boost_lvl_text[4].as_ptr() as *const c_char, smart_boost_lvl_text[5].as_ptr() as *const c_char,
    smart_boost_lvl_text[6].as_ptr() as *const c_char, smart_boost_lvl_text[7].as_ptr() as *const c_char,
    smart_boost_lvl_text[8].as_ptr() as *const c_char, smart_boost_lvl_text[9].as_ptr() as *const c_char,
    smart_boost_lvl_text[10].as_ptr() as *const c_char, smart_boost_lvl_text[11].as_ptr() as *const c_char,
    smart_boost_lvl_text[12].as_ptr() as *const c_char, smart_boost_lvl_text[13].as_ptr() as *const c_char,
    smart_boost_lvl_text[14].as_ptr() as *const c_char, smart_boost_lvl_text[15].as_ptr() as *const c_char,
];

static smart_boost_lvl_enum: soc_enum = soc_enum {
    reg: WSA881X_BOOST_PRESET_OUT1,
    shift_l: 0,
    items: smart_boost_lvl_text_ptrs.len() as c_uint,
    texts: smart_boost_lvl_text_ptrs.as_ptr(),
};

static pa_gain: [c_uint; 4] = [0, 150, 0, 0]; // DECLARE_TLV_DB_SCALE(pa_gain, 0, 150, 0)

// ALSA control/widget macro initializers have no file-local expansion here.
static wsa881x_snd_controls: [snd_kcontrol_new; 6] = [
    snd_kcontrol_new { _opaque: 0 }, snd_kcontrol_new { _opaque: 0 }, snd_kcontrol_new { _opaque: 0 },
    snd_kcontrol_new { _opaque: 0 }, snd_kcontrol_new { _opaque: 0 }, snd_kcontrol_new { _opaque: 0 },
];

static wsa881x_audio_map: [snd_soc_dapm_route; 6] = [
    snd_soc_dapm_route { sink: b"RDAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"IN\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"RDAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DCLK\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"RDAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ACLK\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"RDAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Bandgap\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SPKR PGA\0".as_ptr() as *const c_char, control: ptr::null(), source: b"RDAC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SPKR\0".as_ptr() as *const c_char, control: ptr::null(), source: b"SPKR PGA\0".as_ptr() as *const c_char },
];

unsafe fn wsa881x_visense_txfe_ctrl(comp: *mut snd_soc_component, enable: bool) -> c_int {
    let wsa881x = snd_soc_component_get_drvdata(comp) as *mut wsa881x_priv;

    if enable {
        regmap_multi_reg_write((*wsa881x).regmap, wsa881x_vi_txfe_en_2_0.as_ptr(), wsa881x_vi_txfe_en_2_0.len());
    } else {
        snd_soc_component_update_bits(comp, WSA881X_SPKR_PROT_FE_VSENSE_VCM, 0x08, 0x08);
        /*
         * 200us sleep is needed after visense txfe disable as per
         * HW requirement.
         */
        usleep_range(200, 210);
        snd_soc_component_update_bits(comp, WSA881X_SPKR_PROT_FE_GAIN, 0x01, 0x00);
    }
    0
}

unsafe fn wsa881x_visense_adc_ctrl(comp: *mut snd_soc_component, enable: bool) -> c_int {
    snd_soc_component_update_bits(comp, WSA881X_ADC_EN_MODU_V, BIT(7), (enable as c_uint) << 7);
    snd_soc_component_update_bits(comp, WSA881X_ADC_EN_MODU_I, BIT(7), (enable as c_uint) << 7);
    0
}

unsafe extern "C" fn wsa881x_spkr_pa_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let comp = snd_soc_dapm_to_component((*w).dapm);
    let wsa881x = snd_soc_component_get_drvdata(comp) as *mut wsa881x_priv;

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            snd_soc_component_update_bits(comp, WSA881X_SPKR_OCP_CTL, WSA881X_SPKR_OCP_MASK, WSA881X_SPKR_OCP_EN);
            regmap_multi_reg_write((*wsa881x).regmap, wsa881x_pre_pmu_pa_2_0.as_ptr(), wsa881x_pre_pmu_pa_2_0.len());
            snd_soc_component_update_bits(comp, WSA881X_SPKR_DRV_GAIN, WSA881X_PA_GAIN_SEL_MASK, WSA881X_PA_GAIN_SEL_REG);
        }
        SND_SOC_DAPM_POST_PMU => {
            if (*wsa881x).port_prepared[WSA881X_PORT_VISENSE] {
                wsa881x_visense_txfe_ctrl(comp, true);
                snd_soc_component_update_bits(comp, WSA881X_ADC_EN_SEL_IBAIS, 0x07, 0x01);
                wsa881x_visense_adc_ctrl(comp, true);
            }
        }
        SND_SOC_DAPM_POST_PMD => {
            if (*wsa881x).port_prepared[WSA881X_PORT_VISENSE] {
                wsa881x_visense_adc_ctrl(comp, false);
                wsa881x_visense_txfe_ctrl(comp, false);
            }
            snd_soc_component_update_bits(comp, WSA881X_SPKR_OCP_CTL, WSA881X_SPKR_OCP_MASK, WSA881X_SPKR_OCP_EN | WSA881X_SPKR_OCP_HOLD);
        }
        _ => {}
    }
    0
}

static wsa881x_dapm_widgets: [snd_soc_dapm_widget_desc; 7] = [
    snd_soc_dapm_widget_desc { _opaque: 0 }, snd_soc_dapm_widget_desc { _opaque: 0 },
    snd_soc_dapm_widget_desc { _opaque: 0 }, snd_soc_dapm_widget_desc { _opaque: 0 },
    snd_soc_dapm_widget_desc { _opaque: 0 }, snd_soc_dapm_widget_desc { _opaque: 0 },
    snd_soc_dapm_widget_desc { _opaque: 0 },
];

unsafe extern "C" fn wsa881x_hw_params(_substream: *mut snd_pcm_substream, _params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let wsa881x = dev_get_drvdata((*dai).dev) as *mut wsa881x_priv;

    (*wsa881x).active_ports = 0;
    for i in 0..WSA881X_MAX_SWR_PORTS {
        if !(*wsa881x).port_enable[i] {
            continue;
        }
        (*wsa881x).port_config[(*wsa881x).active_ports as usize] = wsa881x_pconfig[i];
        (*wsa881x).active_ports += 1;
    }

    sdw_stream_add_slave((*wsa881x).slave, &mut (*wsa881x).sconfig, (*wsa881x).port_config.as_mut_ptr(), (*wsa881x).active_ports, (*wsa881x).sruntime)
}

unsafe extern "C" fn wsa881x_hw_free(_substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let wsa881x = dev_get_drvdata((*dai).dev) as *mut wsa881x_priv;
    sdw_stream_remove_slave((*wsa881x).slave, (*wsa881x).sruntime);
    0
}

unsafe extern "C" fn wsa881x_set_sdw_stream(dai: *mut snd_soc_dai, stream: *mut c_void, _direction: c_int) -> c_int {
    let wsa881x = dev_get_drvdata((*dai).dev) as *mut wsa881x_priv;
    (*wsa881x).sruntime = stream as *mut sdw_stream_runtime;
    0
}

unsafe extern "C" fn wsa881x_digital_mute(dai: *mut snd_soc_dai, mute: c_int, _stream: c_int) -> c_int {
    let wsa881x = dev_get_drvdata((*dai).dev) as *mut wsa881x_priv;
    if mute != 0 {
        regmap_update_bits((*wsa881x).regmap, WSA881X_SPKR_DRV_EN, 0x80, 0x00);
    } else {
        regmap_update_bits((*wsa881x).regmap, WSA881X_SPKR_DRV_EN, 0x80, 0x80);
    }
    0
}

static wsa881x_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(wsa881x_hw_params),
    hw_free: Some(wsa881x_hw_free),
    mute_stream: Some(wsa881x_digital_mute),
    set_stream: Some(wsa881x_set_sdw_stream),
};

static mut wsa881x_dais: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: b"SPKR\0".as_ptr() as *const c_char,
    id: 0,
    playback: snd_soc_pcm_stream {
        stream_name: b"SPKR Playback\0".as_ptr() as *const c_char,
        rates: SNDRV_PCM_RATE_48000,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
        rate_max: 48000,
        rate_min: 48000,
        channels_min: 1,
        channels_max: 1,
    },
    ops: &wsa881x_dai_ops,
}];

static wsa881x_component_drv: snd_soc_component_driver = snd_soc_component_driver {
    name: b"WSA881x\0".as_ptr() as *const c_char,
    probe: Some(wsa881x_component_probe),
    controls: wsa881x_snd_controls.as_ptr(),
    num_controls: wsa881x_snd_controls.len(),
    dapm_widgets: wsa881x_dapm_widgets.as_ptr(),
    num_dapm_widgets: wsa881x_dapm_widgets.len(),
    dapm_routes: wsa881x_audio_map.as_ptr(),
    num_dapm_routes: wsa881x_audio_map.len(),
    endianness: 1,
};

unsafe extern "C" fn wsa881x_update_status(slave: *mut sdw_slave, status: sdw_slave_status) -> c_int {
    let wsa881x = dev_get_drvdata(&mut (*slave).dev) as *mut wsa881x_priv;
    if status == SDW_SLAVE_UNATTACHED {
        (*wsa881x).hw_init = false;
    }
    if status == SDW_SLAVE_ATTACHED && (*slave).dev_num > 0 {
        wsa881x_init(wsa881x);
    }
    0
}

unsafe extern "C" fn wsa881x_port_prep(slave: *mut sdw_slave, prepare_ch: *mut sdw_prepare_ch, state: sdw_port_prep_ops) -> c_int {
    let wsa881x = dev_get_drvdata(&mut (*slave).dev) as *mut wsa881x_priv;
    if state == SDW_OPS_PORT_POST_PREP {
        (*wsa881x).port_prepared[((*prepare_ch).num - 1) as usize] = true;
    } else {
        (*wsa881x).port_prepared[((*prepare_ch).num - 1) as usize] = false;
    }
    0
}

unsafe extern "C" fn wsa881x_bus_config(slave: *mut sdw_slave, params: *mut sdw_bus_params) -> c_int {
    sdw_write(slave, SWRS_SCP_HOST_CLK_DIV2_CTL_BANK((*params).next_bank), 0x01);
    0
}

static wsa881x_slave_ops: sdw_slave_ops = sdw_slave_ops {
    update_status: Some(wsa881x_update_status),
    bus_config: Some(wsa881x_bus_config),
    port_prep: Some(wsa881x_port_prep),
};

unsafe extern "C" fn wsa881x_probe(pdev: *mut sdw_slave, _id: *const sdw_device_id) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let wsa881x = devm_kzalloc(dev, core::mem::size_of::<wsa881x_priv>(), GFP_KERNEL) as *mut wsa881x_priv;
    if wsa881x.is_null() {
        return -ENOMEM;
    }

    (*wsa881x).sd_n = devm_gpiod_get_optional(dev, b"powerdown\0".as_ptr() as *const c_char, GPIOD_OUT_LOW);
    if IS_ERR((*wsa881x).sd_n as *const c_void) {
        return dev_err_probe(dev, PTR_ERR((*wsa881x).sd_n as *const c_void), b"Shutdown Control GPIO not found\n\0".as_ptr() as *const c_char);
    }

    dev_set_drvdata(dev, wsa881x as *mut c_void);
    (*wsa881x).slave = pdev;
    (*wsa881x).dev = dev;
    (*wsa881x).sconfig.ch_count = 1;
    (*wsa881x).sconfig.bps = 1;
    (*wsa881x).sconfig.frame_rate = 48000;
    (*wsa881x).sconfig.direction = SDW_DATA_DIR_RX;
    (*wsa881x).sconfig.type_ = SDW_STREAM_PDM;
    (*pdev).prop.sink_ports = GENMASK((WSA881X_MAX_SWR_PORTS - 1) as c_uint, 0);
    (*pdev).prop.sink_dpn_prop = wsa_sink_dpn_prop.as_mut_ptr();
    (*pdev).prop.scp_int1_mask = SDW_SCP_INT1_BUS_CLASH | SDW_SCP_INT1_PARITY;
    (*pdev).prop.clk_stop_mode1 = true;

    (*wsa881x).regmap = devm_regmap_init_sdw(pdev, &wsa881x_regmap_config);
    if IS_ERR((*wsa881x).regmap as *const c_void) {
        return dev_err_probe(dev, PTR_ERR((*wsa881x).regmap as *const c_void), b"regmap_init failed\n\0".as_ptr() as *const c_char);
    }

    pm_runtime_set_autosuspend_delay(dev, 3000);
    pm_runtime_use_autosuspend(dev);
    pm_runtime_mark_last_busy(dev);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);

    devm_snd_soc_register_component(dev, &wsa881x_component_drv, wsa881x_dais.as_mut_ptr(), wsa881x_dais.len())
}

unsafe extern "C" fn wsa881x_runtime_suspend(dev: *mut device) -> c_int {
    let regmap = dev_get_regmap(dev, ptr::null());
    let wsa881x = dev_get_drvdata(dev) as *mut wsa881x_priv;

    gpiod_direction_output((*wsa881x).sd_n, 1);
    regcache_cache_only(regmap, true);
    regcache_mark_dirty(regmap);
    0
}

unsafe extern "C" fn wsa881x_runtime_resume(dev: *mut device) -> c_int {
    let slave = dev_to_sdw_dev(dev);
    let regmap = dev_get_regmap(dev, ptr::null());
    let wsa881x = dev_get_drvdata(dev) as *mut wsa881x_priv;
    let mut ret: c_int;

    gpiod_direction_output((*wsa881x).sd_n, 0);

    ret = sdw_slave_wait_for_init(slave, WSA881X_PROBE_TIMEOUT);
    if ret != 0 {
        gpiod_direction_output((*wsa881x).sd_n, 1);
        return ret;
    }

    regcache_cache_only(regmap, false);
    ret = regcache_sync(regmap);
    if ret != 0 {
        regcache_cache_only(regmap, true);
        regcache_mark_dirty(regmap);
        gpiod_direction_output((*wsa881x).sd_n, 1);
        return ret;
    }

    0
}

// RUNTIME_PM_OPS(wsa881x_runtime_suspend, wsa881x_runtime_resume, NULL)
static wsa881x_pm_ops: dev_pm_ops = dev_pm_ops { _opaque: 0 };

static wsa881x_slave_id: [sdw_device_id; 3] = [
    sdw_device_id { mfg_id: 0x0217, part_id: 0x2010, class_id: 0 },
    sdw_device_id { mfg_id: 0x0217, part_id: 0x2110, class_id: 0 },
    sdw_device_id { mfg_id: 0, part_id: 0, class_id: 0 },
];
// MODULE_DEVICE_TABLE(sdw, wsa881x_slave_id);

static wsa881x_codec_driver: sdw_driver = sdw_driver {
    probe: Some(wsa881x_probe),
    ops: &wsa881x_slave_ops,
    id_table: wsa881x_slave_id.as_ptr(),
    driver: sdw_driver_inner {
        name: b"wsa881x-codec\0".as_ptr() as *const c_char,
        pm: &wsa881x_pm_ops,
    },
};
// module_sdw_driver(wsa881x_codec_driver);

// MODULE_DESCRIPTION("WSA881x codec driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
