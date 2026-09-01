// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Codec driver for ST STA350 2.1-channel high-efficiency digital audio system
 *
 * Copyright: 2014 Raumfeld GmbH
 * Author: Sven Brandau <info@brandau.biz>
 *
 * based on code from:
 *	Raumfeld GmbH
 *	  Johannes Stezenbach <js@sig21.net>
 *	Wolfson Microelectronics PLC.
 *	  Mark Brown <broonie@opensource.wolfsonmicro.com>
 *	Freescale Semiconductor, Inc.
 *	  Timur Tabi <timur@freescale.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

// pr_fmt(fmt) was defined in C as:
// KBUILD_MODNAME ":%s:%d: " fmt, __func__, __LINE__
// C include dependencies removed: linux kernel, ALSA SoC, sound/sta350.h, and "sta350.h".

const STA350_RATES: c_uint = SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_176400
    | SNDRV_PCM_RATE_192000;

const STA350_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S18_3LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_3LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct regmap_range {
    pub range_min: c_uint,
    pub range_max: c_uint,
}

#[repr(C)]
pub struct regmap_access_table {
    pub yes_ranges: *const regmap_range,
    pub n_yes_ranges: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_int,
    pub val_bits: c_int,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_int,
    pub wr_table: *const regmap_access_table,
    pub rd_table: *const regmap_access_table,
    pub volatile_table: *const regmap_access_table,
}

#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
}

#[repr(C)]
pub struct sta350_platform_data {
    pub output_conf: u8,
    pub ch1_output_mapping: u8,
    pub ch2_output_mapping: u8,
    pub ch3_output_mapping: u8,
    pub thermal_warning_recovery: bool,
    pub thermal_warning_adjustment: bool,
    pub fault_detect_recovery: bool,
    pub ffx_power_output_mode: c_int,
    pub drop_compensation_ns: c_int,
    pub oc_warning_adjustment: bool,
    pub max_power_use_mpcc: bool,
    pub max_power_correction: bool,
    pub am_reduction_mode: bool,
    pub odd_pwm_speed_mode: bool,
    pub distortion_compensation: bool,
    pub invalid_input_detect_mute: bool,
    pub activate_mute_output: bool,
    pub bridge_immediate_off: bool,
    pub noise_shape_dc_cut: bool,
    pub powerdown_master_vol: bool,
    pub powerdown_delay_divider: c_int,
}

#[repr(C)]
pub struct regmap(c_void);
#[repr(C)]
pub struct gpio_desc(c_void);
#[repr(C)]
pub struct mutex(c_void);
#[repr(C)]
pub struct device_node(c_void);

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
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
pub struct snd_pcm_substream(c_void);
#[repr(C)]
pub struct snd_pcm_hw_params(c_void);
#[repr(C)]
pub struct snd_soc_dapm_context(c_void);
#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: usize,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
}

#[repr(C)]
pub struct snd_ctl_elem_value_bytes {
    pub data: [u8; 512],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub bytes: core::mem::ManuallyDrop<snd_ctl_elem_value_bytes>,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: usize,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub name: *const c_char,
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
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub suspend_bias_off: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON = 0,
    SND_SOC_BIAS_PREPARE = 1,
    SND_SOC_BIAS_STANDBY = 2,
    SND_SOC_BIAS_OFF = 3,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
}

#[repr(C)]
pub struct i2c_driver_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: i2c_driver_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

#[repr(C)]
struct sta350_priv {
    regmap: *mut regmap,
    supplies: [regulator_bulk_data; sta350_supply_names.len()],
    pdata: *mut sta350_platform_data,
    mclk: c_uint,
    format: c_uint,
    coef_shadow: [u32; STA350_COEF_COUNT as usize],
    shutdown: c_int,
    gpiod_nreset: *mut gpio_desc,
    gpiod_power_down: *mut gpio_desc,
    coeff_lock: mutex,
}

extern "C" {
    static SNDRV_PCM_RATE_32000: c_uint;
    static SNDRV_PCM_RATE_44100: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_RATE_88200: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_RATE_176400: c_uint;
    static SNDRV_PCM_RATE_192000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S18_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S20_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;
}

extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_soc_dapm_force_bias_level(dapm: *mut snd_soc_dapm_context, level: snd_soc_bias_level) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regulator_bulk_enable(num_consumers: c_uint, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num_consumers: c_uint, consumers: *mut regulator_bulk_data) -> c_int;
    fn gpiod_set_value(desc: *mut gpio_desc, value: c_int);
    fn mdelay(msecs: c_uint);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_get_platdata(dev: *mut device) -> *mut sta350_platform_data;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_regulator_bulk_get(dev: *mut device, num_consumers: c_uint, consumers: *mut regulator_bulk_data) -> c_int;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn of_property_read_u8(np: *mut device_node, propname: *const c_char, out_value: *mut u8) -> c_int;
    fn of_property_read_u16(np: *mut device_node, propname: *const c_char, out_value: *mut u16) -> c_int;
    fn of_property_read_bool(np: *mut device_node, propname: *const c_char) -> bool;
    fn of_property_read_string(np: *mut device_node, propname: *const c_char, out_string: *mut *const c_char) -> c_int;
    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn is_power_of_2(n: u8) -> bool;
    fn ilog2(n: u8) -> c_int;
}

extern "C" {
    static EINVAL: c_int;
    static EIO: c_int;
    static ENOMEM: c_int;
    static GFP_KERNEL: c_uint;
    static GPIOD_OUT_LOW: c_uint;
    static REGCACHE_MAPLE: c_int;
    static SNDRV_CTL_ELEM_TYPE_BYTES: c_uint;
    static SNDRV_CTL_ELEM_IFACE_MIXER: c_uint;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SND_SOC_NOPM: c_int;
}

extern "C" {
    static STA350_COEF_COUNT: c_uint;
    static STA350_CONFA: c_uint;
    static STA350_CONFB: c_uint;
    static STA350_CONFC: c_uint;
    static STA350_CONFD: c_uint;
    static STA350_CONFE: c_uint;
    static STA350_CONFF: c_uint;
    static STA350_MMUTE: c_uint;
    static STA350_MVOL: c_uint;
    static STA350_C1VOL: c_uint;
    static STA350_C2VOL: c_uint;
    static STA350_C3VOL: c_uint;
    static STA350_AUTO1: c_uint;
    static STA350_AUTO2: c_uint;
    static STA350_C1CFG: c_uint;
    static STA350_C2CFG: c_uint;
    static STA350_C3CFG: c_uint;
    static STA350_TONE: c_uint;
    static STA350_L1AR: c_uint;
    static STA350_L2AR: c_uint;
    static STA350_L1ATRT: c_uint;
    static STA350_L2ATRT: c_uint;
    static STA350_EQCFG: c_uint;
    static STA350_EVOLRES: c_uint;
    static STA350_NSHAPE: c_uint;
    static STA350_MISC1: c_uint;
    static STA350_MISC2: c_uint;
    static STA350_CFADDR2: c_uint;
    static STA350_CFUD: c_uint;
    static STA350_STATUS: c_uint;
    static STA350_B1CF1: c_uint;
    static STA350_B1CF2: c_uint;
    static STA350_B1CF3: c_uint;
    static STA350_FDRC2: c_uint;
    static STA350_CONFA_TWAB: c_uint;
    static STA350_CONFA_TWRB: c_uint;
    static STA350_CONFA_FDRB: c_uint;
    static STA350_CONFA_IR_SHIFT: c_uint;
    static STA350_CONFA_MCS_SHIFT: c_uint;
    static STA350_CONFA_MCS_MASK: c_uint;
    static STA350_CONFA_IR_MASK: c_uint;
    static STA350_CONFB_C1IM: c_uint;
    static STA350_CONFB_C2IM: c_uint;
    static STA350_CONFB_SAI_MASK: c_uint;
    static STA350_CONFB_SAIFB: c_uint;
    static STA350_CONFC_OM_MASK: c_uint;
    static STA350_CONFC_OM_SHIFT: c_uint;
    static STA350_CONFC_CSZ_MASK: c_uint;
    static STA350_CONFC_CSZ_SHIFT: c_uint;
    static STA350_CONFC_OCRB: c_uint;
    static STA350_CONFD_DRC_SHIFT: c_uint;
    static STA350_CONFD_HPB_SHIFT: c_uint;
    static STA350_CONFD_DEMP_SHIFT: c_uint;
    static STA350_CONFD_DSPB_SHIFT: c_uint;
    static STA350_CONFD_PSL_SHIFT: c_uint;
    static STA350_CONFD_BQL_SHIFT: c_uint;
    static STA350_CONFD_ZDE_SHIFT: c_uint;
    static STA350_CONFD_SME_SHIFT: c_uint;
    static STA350_CONFE_NSBW_SHIFT: c_uint;
    static STA350_CONFE_ZCE_SHIFT: c_uint;
    static STA350_CONFE_SVE_SHIFT: c_uint;
    static STA350_CONFE_MPCV: c_uint;
    static STA350_CONFE_MPC: c_uint;
    static STA350_CONFE_AME: c_uint;
    static STA350_CONFE_PWMS: c_uint;
    static STA350_CONFE_DCCV: c_uint;
    static STA350_CONFF_PWDN: c_uint;
    static STA350_CONFF_EAPD: c_uint;
    static STA350_CONFF_IDE: c_uint;
    static STA350_CONFF_OCFG_MASK: c_uint;
    static STA350_CONFF_OCFG_SHIFT: c_uint;
    static STA350_MMUTE_MMUTE: c_uint;
    static STA350_MMUTE_MMUTE_SHIFT: c_uint;
    static STA350_MMUTE_C1M_SHIFT: c_uint;
    static STA350_MMUTE_C2M_SHIFT: c_uint;
    static STA350_MMUTE_C3M_SHIFT: c_uint;
    static STA350_AUTO1_AMGC_SHIFT: c_uint;
    static STA350_AUTO2_XO_SHIFT: c_uint;
    static STA350_CxCFG_TCB_SHIFT: c_uint;
    static STA350_CxCFG_EQBP_SHIFT: c_uint;
    static STA350_CxCFG_VBP_SHIFT: c_uint;
    static STA350_CxCFG_BO_SHIFT: c_uint;
    static STA350_CxCFG_LS_SHIFT: c_uint;
    static STA350_CxCFG_OM_MASK: c_uint;
    static STA350_CxCFG_OM_SHIFT: c_uint;
    static STA350_TONE_BTC_SHIFT: c_uint;
    static STA350_TONE_TTC_SHIFT: c_uint;
    static STA350_LxA_SHIFT: c_uint;
    static STA350_LxR_SHIFT: c_uint;
    static STA350_MISC1_CPWMEN: c_uint;
    static STA350_MISC1_BRIDGOFF: c_uint;
    static STA350_MISC1_NSHHPEN: c_uint;
    static STA350_MISC1_RPDNEN: c_uint;
    static STA350_MISC2_PNDLSL_MASK: c_uint;
    static STA350_MISC2_PNDLSL_SHIFT: c_uint;
    static STA350_FFX_PM_DROP_COMP: c_uint;
    static STA350_FFX_PM_TAPERED_COMP: c_uint;
    static STA350_FFX_PM_FULL_POWER: c_uint;
    static STA350_FFX_PM_VARIABLE_DROP_COMP: c_uint;
}

const fn regmap_reg_range(range_min: c_uint, range_max: c_uint) -> regmap_range {
    regmap_range { range_min, range_max }
}

/* Power-up register defaults */
static sta350_regs: [reg_default; 60] = [
    reg_default { reg: 0x0, def: 0x63 },
    reg_default { reg: 0x1, def: 0x80 },
    reg_default { reg: 0x2, def: 0xdf },
    reg_default { reg: 0x3, def: 0x40 },
    reg_default { reg: 0x4, def: 0xc2 },
    reg_default { reg: 0x5, def: 0x5c },
    reg_default { reg: 0x6, def: 0x00 },
    reg_default { reg: 0x7, def: 0xff },
    reg_default { reg: 0x8, def: 0x60 },
    reg_default { reg: 0x9, def: 0x60 },
    reg_default { reg: 0xa, def: 0x60 },
    reg_default { reg: 0xb, def: 0x00 },
    reg_default { reg: 0xc, def: 0x00 },
    reg_default { reg: 0xd, def: 0x00 },
    reg_default { reg: 0xe, def: 0x00 },
    reg_default { reg: 0xf, def: 0x40 },
    reg_default { reg: 0x10, def: 0x80 },
    reg_default { reg: 0x11, def: 0x77 },
    reg_default { reg: 0x12, def: 0x6a },
    reg_default { reg: 0x13, def: 0x69 },
    reg_default { reg: 0x14, def: 0x6a },
    reg_default { reg: 0x15, def: 0x69 },
    reg_default { reg: 0x16, def: 0x00 },
    reg_default { reg: 0x17, def: 0x00 },
    reg_default { reg: 0x18, def: 0x00 },
    reg_default { reg: 0x19, def: 0x00 },
    reg_default { reg: 0x1a, def: 0x00 },
    reg_default { reg: 0x1b, def: 0x00 },
    reg_default { reg: 0x1c, def: 0x00 },
    reg_default { reg: 0x1d, def: 0x00 },
    reg_default { reg: 0x1e, def: 0x00 },
    reg_default { reg: 0x1f, def: 0x00 },
    reg_default { reg: 0x20, def: 0x00 },
    reg_default { reg: 0x21, def: 0x00 },
    reg_default { reg: 0x22, def: 0x00 },
    reg_default { reg: 0x23, def: 0x00 },
    reg_default { reg: 0x24, def: 0x00 },
    reg_default { reg: 0x25, def: 0x00 },
    reg_default { reg: 0x26, def: 0x00 },
    reg_default { reg: 0x27, def: 0x2a },
    reg_default { reg: 0x28, def: 0xc0 },
    reg_default { reg: 0x29, def: 0xf3 },
    reg_default { reg: 0x2a, def: 0x33 },
    reg_default { reg: 0x2b, def: 0x00 },
    reg_default { reg: 0x2c, def: 0x0c },
    reg_default { reg: 0x31, def: 0x00 },
    reg_default { reg: 0x36, def: 0x00 },
    reg_default { reg: 0x37, def: 0x00 },
    reg_default { reg: 0x38, def: 0x00 },
    reg_default { reg: 0x39, def: 0x01 },
    reg_default { reg: 0x3a, def: 0xee },
    reg_default { reg: 0x3b, def: 0xff },
    reg_default { reg: 0x3c, def: 0x7e },
    reg_default { reg: 0x3d, def: 0xc0 },
    reg_default { reg: 0x3e, def: 0x26 },
    reg_default { reg: 0x3f, def: 0x00 },
    reg_default { reg: 0x48, def: 0x00 },
    reg_default { reg: 0x49, def: 0x00 },
    reg_default { reg: 0x4a, def: 0x00 },
    reg_default { reg: 0x4b, def: 0x04 },
];

static sta350_write_regs_range: [regmap_range; 4] = [
    regmap_reg_range(STA350_CONFA, STA350_AUTO2),
    regmap_reg_range(STA350_C1CFG, STA350_FDRC2),
    regmap_reg_range(STA350_EQCFG, STA350_EVOLRES),
    regmap_reg_range(STA350_NSHAPE, STA350_MISC2),
];

static sta350_read_regs_range: [regmap_range; 4] = [
    regmap_reg_range(STA350_CONFA, STA350_AUTO2),
    regmap_reg_range(STA350_C1CFG, STA350_STATUS),
    regmap_reg_range(STA350_EQCFG, STA350_EVOLRES),
    regmap_reg_range(STA350_NSHAPE, STA350_MISC2),
];

static sta350_volatile_regs_range: [regmap_range; 2] = [
    regmap_reg_range(STA350_CFADDR2, STA350_CFUD),
    regmap_reg_range(STA350_STATUS, STA350_STATUS),
];

static sta350_write_regs: regmap_access_table = regmap_access_table {
    yes_ranges: sta350_write_regs_range.as_ptr(),
    n_yes_ranges: sta350_write_regs_range.len() as c_uint,
};

static sta350_read_regs: regmap_access_table = regmap_access_table {
    yes_ranges: sta350_read_regs_range.as_ptr(),
    n_yes_ranges: sta350_read_regs_range.len() as c_uint,
};

static sta350_volatile_regs: regmap_access_table = regmap_access_table {
    yes_ranges: sta350_volatile_regs_range.as_ptr(),
    n_yes_ranges: sta350_volatile_regs_range.len() as c_uint,
};

/* regulator power supply names */
static sta350_supply_names: [*const c_char; 3] = [
    b"vdd-dig\0".as_ptr() as *const c_char, /* digital supply, 3.3V */
    b"vdd-pll\0".as_ptr() as *const c_char, /* pll supply, 3.3V */
    b"vcc\0".as_ptr() as *const c_char,     /* power amp supply, 5V - 26V */
];

static mvol_tlv: [c_uint; 4] = [0, (-12750i32) as c_uint, 50, 1];
static chvol_tlv: [c_uint; 4] = [0, (-7950i32) as c_uint, 50, 1];
static tone_tlv: [c_uint; 4] = [0, (-1200i32) as c_uint, 200, 0];

static sta350_drc_ac: [*const c_char; 2] = [
    b"Anti-Clipping\0".as_ptr() as *const c_char,
    b"Dynamic Range Compression\0".as_ptr() as *const c_char,
];
static sta350_auto_gc_mode: [*const c_char; 4] = [
    b"User\0".as_ptr() as *const c_char,
    b"AC no clipping\0".as_ptr() as *const c_char,
    b"AC limited clipping (10%)\0".as_ptr() as *const c_char,
    b"DRC nighttime listening mode\0".as_ptr() as *const c_char,
];
static sta350_auto_xo_mode: [*const c_char; 16] = [
    b"User\0".as_ptr() as *const c_char,
    b"80Hz\0".as_ptr() as *const c_char,
    b"100Hz\0".as_ptr() as *const c_char,
    b"120Hz\0".as_ptr() as *const c_char,
    b"140Hz\0".as_ptr() as *const c_char,
    b"160Hz\0".as_ptr() as *const c_char,
    b"180Hz\0".as_ptr() as *const c_char,
    b"200Hz\0".as_ptr() as *const c_char,
    b"220Hz\0".as_ptr() as *const c_char,
    b"240Hz\0".as_ptr() as *const c_char,
    b"260Hz\0".as_ptr() as *const c_char,
    b"280Hz\0".as_ptr() as *const c_char,
    b"300Hz\0".as_ptr() as *const c_char,
    b"320Hz\0".as_ptr() as *const c_char,
    b"340Hz\0".as_ptr() as *const c_char,
    b"360Hz\0".as_ptr() as *const c_char,
];
static sta350_binary_output: [*const c_char; 2] = [
    b"FFX 3-state output - normal operation\0".as_ptr() as *const c_char,
    b"Binary output\0".as_ptr() as *const c_char,
];
static sta350_limiter_select: [*const c_char; 3] = [
    b"Limiter Disabled\0".as_ptr() as *const c_char,
    b"Limiter #1\0".as_ptr() as *const c_char,
    b"Limiter #2\0".as_ptr() as *const c_char,
];
static sta350_limiter_attack_rate: [*const c_char; 16] = [
    b"3.1584\0".as_ptr() as *const c_char, b"2.7072\0".as_ptr() as *const c_char,
    b"2.2560\0".as_ptr() as *const c_char, b"1.8048\0".as_ptr() as *const c_char,
    b"1.3536\0".as_ptr() as *const c_char, b"0.9024\0".as_ptr() as *const c_char,
    b"0.4512\0".as_ptr() as *const c_char, b"0.2256\0".as_ptr() as *const c_char,
    b"0.1504\0".as_ptr() as *const c_char, b"0.1123\0".as_ptr() as *const c_char,
    b"0.0902\0".as_ptr() as *const c_char, b"0.0752\0".as_ptr() as *const c_char,
    b"0.0645\0".as_ptr() as *const c_char, b"0.0564\0".as_ptr() as *const c_char,
    b"0.0501\0".as_ptr() as *const c_char, b"0.0451\0".as_ptr() as *const c_char,
];
static sta350_limiter_release_rate: [*const c_char; 16] = [
    b"0.5116\0".as_ptr() as *const c_char, b"0.1370\0".as_ptr() as *const c_char,
    b"0.0744\0".as_ptr() as *const c_char, b"0.0499\0".as_ptr() as *const c_char,
    b"0.0360\0".as_ptr() as *const c_char, b"0.0299\0".as_ptr() as *const c_char,
    b"0.0264\0".as_ptr() as *const c_char, b"0.0208\0".as_ptr() as *const c_char,
    b"0.0198\0".as_ptr() as *const c_char, b"0.0172\0".as_ptr() as *const c_char,
    b"0.0147\0".as_ptr() as *const c_char, b"0.0137\0".as_ptr() as *const c_char,
    b"0.0134\0".as_ptr() as *const c_char, b"0.0117\0".as_ptr() as *const c_char,
    b"0.0110\0".as_ptr() as *const c_char, b"0.0104\0".as_ptr() as *const c_char,
];
static sta350_noise_shaper_type: [*const c_char; 2] = [
    b"Third order\0".as_ptr() as *const c_char,
    b"Fourth order\0".as_ptr() as *const c_char,
];

// TLV_DB_RANGE and SOC_ENUM_SINGLE_DECL macro expansions are represented as
// file-local placeholders preserving the original declaration names.
static sta350_limiter_ac_attack_tlv: [c_int; 9] = [0, 7, -1200, 200, 0, 8, 16, 300, 100];
static sta350_limiter_ac_release_tlv: [c_int; 20] = [0, 0, -999999, 0, 0, 1, 1, -2900, 0, 0, 2, 2, -2000, 0, 0, 3, 8, -1400, 200, 0];
static sta350_limiter_drc_attack_tlv: [c_int; 15] = [0, 7, -3100, 200, 0, 8, 13, -1600, 100, 0, 14, 16, -1000, 300, 0];
static sta350_limiter_drc_release_tlv: [c_int; 25] = [0, 0, -999999, 0, 0, 1, 2, -3800, 200, 0, 3, 4, -3300, 200, 0, 5, 12, -3000, 200, 0, 13, 16, -1500, 300, 0];

#[repr(C)]
struct soc_enum_decl {
    reg: c_uint,
    shift: c_uint,
    texts: *const *const c_char,
    items: c_uint,
}

macro_rules! SOC_ENUM_SINGLE_DECL {
    ($name:ident, $reg:expr, $shift:expr, $texts:ident) => {
        static $name: soc_enum_decl = soc_enum_decl {
            reg: $reg,
            shift: $shift,
            texts: $texts.as_ptr(),
            items: $texts.len() as c_uint,
        };
    };
}

SOC_ENUM_SINGLE_DECL!(sta350_drc_ac_enum, STA350_CONFD, STA350_CONFD_DRC_SHIFT, sta350_drc_ac);
SOC_ENUM_SINGLE_DECL!(sta350_noise_shaper_enum, STA350_CONFE, STA350_CONFE_NSBW_SHIFT, sta350_noise_shaper_type);
SOC_ENUM_SINGLE_DECL!(sta350_auto_gc_enum, STA350_AUTO1, STA350_AUTO1_AMGC_SHIFT, sta350_auto_gc_mode);
SOC_ENUM_SINGLE_DECL!(sta350_auto_xo_enum, STA350_AUTO2, STA350_AUTO2_XO_SHIFT, sta350_auto_xo_mode);
SOC_ENUM_SINGLE_DECL!(sta350_binary_output_ch1_enum, STA350_C1CFG, STA350_CxCFG_BO_SHIFT, sta350_binary_output);
SOC_ENUM_SINGLE_DECL!(sta350_binary_output_ch2_enum, STA350_C2CFG, STA350_CxCFG_BO_SHIFT, sta350_binary_output);
SOC_ENUM_SINGLE_DECL!(sta350_binary_output_ch3_enum, STA350_C3CFG, STA350_CxCFG_BO_SHIFT, sta350_binary_output);
SOC_ENUM_SINGLE_DECL!(sta350_limiter_ch1_enum, STA350_C1CFG, STA350_CxCFG_LS_SHIFT, sta350_limiter_select);
SOC_ENUM_SINGLE_DECL!(sta350_limiter_ch2_enum, STA350_C2CFG, STA350_CxCFG_LS_SHIFT, sta350_limiter_select);
SOC_ENUM_SINGLE_DECL!(sta350_limiter_ch3_enum, STA350_C3CFG, STA350_CxCFG_LS_SHIFT, sta350_limiter_select);
SOC_ENUM_SINGLE_DECL!(sta350_limiter1_attack_rate_enum, STA350_L1AR, STA350_LxA_SHIFT, sta350_limiter_attack_rate);
SOC_ENUM_SINGLE_DECL!(sta350_limiter2_attack_rate_enum, STA350_L2AR, STA350_LxA_SHIFT, sta350_limiter_attack_rate);
SOC_ENUM_SINGLE_DECL!(sta350_limiter1_release_rate_enum, STA350_L1AR, STA350_LxR_SHIFT, sta350_limiter_release_rate);
SOC_ENUM_SINGLE_DECL!(sta350_limiter2_release_rate_enum, STA350_L2AR, STA350_LxR_SHIFT, sta350_limiter_release_rate);

/*
 * byte array controls for setting biquad, mixer, scaling coefficients;
 * for biquads all five coefficients need to be set in one go,
 * mixer and pre/postscale coefs can be set individually;
 * each coef is 24bit, the bytes are ordered in the same way
 * as given in the STA350 data sheet (big endian; b1, b2, a1, a2, b0)
 */

unsafe extern "C" fn sta350_coefficient_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let numcoef = ((*kcontrol).private_value >> 16) as c_int;
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BYTES;
    (*uinfo).count = (3 * numcoef) as c_uint;
    0
}

unsafe extern "C" fn sta350_coefficient_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let sta350 = snd_soc_component_get_drvdata(component) as *mut sta350_priv;
    let numcoef = ((*kcontrol).private_value >> 16) as c_int;
    let index = ((*kcontrol).private_value & 0xffff) as c_int;
    let mut cfud: c_uint = 0;
    let mut val: c_uint = 0;

    mutex_lock(&mut (*sta350).coeff_lock);

    /* preserve reserved bits in STA350_CFUD */
    regmap_read((*sta350).regmap, STA350_CFUD, &mut cfud);
    cfud &= 0xf0;
    /*
     * chip documentation does not say if the bits are self clearing,
     * so do it explicitly
     */
    regmap_write((*sta350).regmap, STA350_CFUD, cfud);

    regmap_write((*sta350).regmap, STA350_CFADDR2, index as c_uint);
    if numcoef == 1 {
        regmap_write((*sta350).regmap, STA350_CFUD, cfud | 0x04);
    } else if numcoef == 5 {
        regmap_write((*sta350).regmap, STA350_CFUD, cfud | 0x08);
    } else {
        mutex_unlock(&mut (*sta350).coeff_lock);
        return -EINVAL;
    }

    let mut i = 0;
    while i < 3 * numcoef {
        regmap_read((*sta350).regmap, STA350_B1CF1 + i as c_uint, &mut val);
        (*ucontrol).value.bytes.data[i as usize] = val as u8;
        i += 1;
    }

    mutex_unlock(&mut (*sta350).coeff_lock);
    0
}

unsafe extern "C" fn sta350_coefficient_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let sta350 = snd_soc_component_get_drvdata(component) as *mut sta350_priv;
    let numcoef = ((*kcontrol).private_value >> 16) as c_int;
    let index = ((*kcontrol).private_value & 0xffff) as c_int;
    let mut cfud: c_uint = 0;

    /* preserve reserved bits in STA350_CFUD */
    regmap_read((*sta350).regmap, STA350_CFUD, &mut cfud);
    cfud &= 0xf0;
    /*
     * chip documentation does not say if the bits are self clearing,
     * so do it explicitly
     */
    regmap_write((*sta350).regmap, STA350_CFUD, cfud);

    regmap_write((*sta350).regmap, STA350_CFADDR2, index as c_uint);
    let mut i = 0;
    while i < numcoef && index + i < STA350_COEF_COUNT as c_int {
        (*sta350).coef_shadow[(index + i) as usize] =
            (((*ucontrol).value.bytes.data[(3 * i) as usize] as u32) << 16)
                | (((*ucontrol).value.bytes.data[(3 * i + 1) as usize] as u32) << 8)
                | ((*ucontrol).value.bytes.data[(3 * i + 2) as usize] as u32);
        i += 1;
    }
    i = 0;
    while i < 3 * numcoef {
        regmap_write((*sta350).regmap, STA350_B1CF1 + i as c_uint, (*ucontrol).value.bytes.data[i as usize] as c_uint);
        i += 1;
    }
    if numcoef == 1 {
        regmap_write((*sta350).regmap, STA350_CFUD, cfud | 0x01);
    } else if numcoef == 5 {
        regmap_write((*sta350).regmap, STA350_CFUD, cfud | 0x02);
    } else {
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn sta350_sync_coef_shadow(component: *mut snd_soc_component) -> c_int {
    let sta350 = snd_soc_component_get_drvdata(component) as *mut sta350_priv;
    let mut cfud: c_uint = 0;

    /* preserve reserved bits in STA350_CFUD */
    regmap_read((*sta350).regmap, STA350_CFUD, &mut cfud);
    cfud &= 0xf0;

    let mut i = 0;
    while i < STA350_COEF_COUNT as c_int {
        regmap_write((*sta350).regmap, STA350_CFADDR2, i as c_uint);
        regmap_write((*sta350).regmap, STA350_B1CF1, ((*sta350).coef_shadow[i as usize] >> 16) & 0xff);
        regmap_write((*sta350).regmap, STA350_B1CF2, ((*sta350).coef_shadow[i as usize] >> 8) & 0xff);
        regmap_write((*sta350).regmap, STA350_B1CF3, (*sta350).coef_shadow[i as usize] & 0xff);
        /*
         * chip documentation does not say if the bits are
         * self-clearing, so do it explicitly
         */
        regmap_write((*sta350).regmap, STA350_CFUD, cfud);
        regmap_write((*sta350).regmap, STA350_CFUD, cfud | 0x01);
        i += 1;
    }
    0
}

unsafe extern "C" fn sta350_cache_sync(component: *mut snd_soc_component) -> c_int {
    let sta350 = snd_soc_component_get_drvdata(component) as *mut sta350_priv;
    let mut mute: c_uint = 0;

    /* mute during register sync */
    regmap_read((*sta350).regmap, STA350_CFUD, &mut mute);
    regmap_write((*sta350).regmap, STA350_MMUTE, mute | STA350_MMUTE_MMUTE);
    sta350_sync_coef_shadow(component);
    let rc = regcache_sync((*sta350).regmap);
    regmap_write((*sta350).regmap, STA350_MMUTE, mute);
    rc
}

macro_rules! SINGLE_COEF {
    ($xname:expr, $index:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: concat!($xname, "\0").as_ptr() as *const c_char,
            info: Some(sta350_coefficient_info),
            get: Some(sta350_coefficient_get),
            put: Some(sta350_coefficient_put),
            private_value: ($index | (1 << 16)) as usize,
        }
    };
}

macro_rules! BIQUAD_COEFS {
    ($xname:expr, $index:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: concat!($xname, "\0").as_ptr() as *const c_char,
            info: Some(sta350_coefficient_info),
            get: Some(sta350_coefficient_get),
            put: Some(sta350_coefficient_put),
            private_value: ($index | (5 << 16)) as usize,
        }
    };
}

// ALSA SOC_* control macros are preserved as external macro-intent comments;
// controls requiring custom callbacks are translated below.
static sta350_snd_controls: [snd_kcontrol_new; 22] = [
    BIQUAD_COEFS!("Ch1 - Biquad 1", 0),
    BIQUAD_COEFS!("Ch1 - Biquad 2", 5),
    BIQUAD_COEFS!("Ch1 - Biquad 3", 10),
    BIQUAD_COEFS!("Ch1 - Biquad 4", 15),
    BIQUAD_COEFS!("Ch2 - Biquad 1", 20),
    BIQUAD_COEFS!("Ch2 - Biquad 2", 25),
    BIQUAD_COEFS!("Ch2 - Biquad 3", 30),
    BIQUAD_COEFS!("Ch2 - Biquad 4", 35),
    BIQUAD_COEFS!("High-pass", 40),
    BIQUAD_COEFS!("Low-pass", 45),
    SINGLE_COEF!("Ch1 - Prescale", 50),
    SINGLE_COEF!("Ch2 - Prescale", 51),
    SINGLE_COEF!("Ch1 - Postscale", 52),
    SINGLE_COEF!("Ch2 - Postscale", 53),
    SINGLE_COEF!("Ch3 - Postscale", 54),
    SINGLE_COEF!("Thermal warning - Postscale", 55),
    SINGLE_COEF!("Ch1 - Mix 1", 56),
    SINGLE_COEF!("Ch1 - Mix 2", 57),
    SINGLE_COEF!("Ch2 - Mix 1", 58),
    SINGLE_COEF!("Ch2 - Mix 2", 59),
    SINGLE_COEF!("Ch3 - Mix 1", 60),
    SINGLE_COEF!("Ch3 - Mix 2", 61),
];

static sta350_dapm_widgets: [snd_soc_dapm_widget; 4] = [
    snd_soc_dapm_widget { name: b"DAC\0".as_ptr() as *const c_char },
    snd_soc_dapm_widget { name: b"LEFT\0".as_ptr() as *const c_char },
    snd_soc_dapm_widget { name: b"RIGHT\0".as_ptr() as *const c_char },
    snd_soc_dapm_widget { name: b"SUB\0".as_ptr() as *const c_char },
];

static sta350_dapm_routes: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route { sink: b"LEFT\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"DAC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"RIGHT\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"DAC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SUB\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"DAC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Playback\0".as_ptr() as *const c_char },
];

/* MCLK interpolation ratio per fs */
#[repr(C)]
struct interpolation_ratio {
    fs: c_int,
    ir: c_int,
}

static mut interpolation_ratios: [interpolation_ratio; 7] = [
    interpolation_ratio { fs: 32000, ir: 0 },
    interpolation_ratio { fs: 44100, ir: 0 },
    interpolation_ratio { fs: 48000, ir: 0 },
    interpolation_ratio { fs: 88200, ir: 1 },
    interpolation_ratio { fs: 96000, ir: 1 },
    interpolation_ratio { fs: 176400, ir: 2 },
    interpolation_ratio { fs: 192000, ir: 2 },
];

/* MCLK to fs clock ratios */
static mut mcs_ratio_table: [[c_int; 6]; 3] = [
    [768, 512, 384, 256, 128, 576],
    [384, 256, 192, 128, 64, 0],
    [192, 128, 96, 64, 32, 0],
];

/**
 * sta350_set_dai_sysclk - configure MCLK
 * @codec_dai: the codec DAI
 * @clk_id: the clock ID (ignored)
 * @freq: the MCLK input frequency
 * @dir: the clock direction (ignored)
 *
 * The value of MCLK is used to determine which sample rates are supported
 * by the STA350, based on the mcs_ratio_table.
 *
 * This function must be called by the machine driver's 'startup' function,
 * otherwise the list of supported sample rates will not be available in
 * time for ALSA.
 */
unsafe extern "C" fn sta350_set_dai_sysclk(codec_dai: *mut snd_soc_dai, _clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*codec_dai).component;
    let sta350 = snd_soc_component_get_drvdata(component) as *mut sta350_priv;

    (*sta350).mclk = freq;

    0
}

/**
 * sta350_set_dai_fmt - configure the codec for the selected audio format
 * @codec_dai: the codec DAI
 * @fmt: a SND_SOC_DAIFMT_x value indicating the data format
 *
 * This function takes a bitmask of SND_SOC_DAIFMT_x bits and programs the
 * codec accordingly.
 */
unsafe extern "C" fn sta350_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let sta350 = snd_soc_component_get_drvdata(component) as *mut sta350_priv;
    let mut confb: c_uint = 0;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        x if x == SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S || x == SND_SOC_DAIFMT_RIGHT_J || x == SND_SOC_DAIFMT_LEFT_J => {
            (*sta350).format = fmt & SND_SOC_DAIFMT_FORMAT_MASK;
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => confb |= STA350_CONFB_C2IM,
        x if x == SND_SOC_DAIFMT_NB_IF => confb |= STA350_CONFB_C1IM,
        _ => return -EINVAL,
    }

    regmap_update_bits((*sta350).regmap, STA350_CONFB, STA350_CONFB_C1IM | STA350_CONFB_C2IM, confb)
}

/**
 * sta350_hw_params - program the STA350 with the given hardware parameters.
 * @substream: the audio stream
 * @params: the hardware parameters to set
 * @dai: the SOC DAI (ignored)
 *
 * This function programs the hardware with the values provided.
 * Specifically, the sample rate and the data format.
 */
unsafe extern "C" fn sta350_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let sta350 = snd_soc_component_get_drvdata(component) as *mut sta350_priv;
    let mut mcs: c_int = -EINVAL;
    let mut ir: c_int = -EINVAL;
    let rate: c_uint;
    let ratio: c_uint;
    let mut confa: c_uint;
    let mut confb: c_uint;
    let mut ret: c_int;

    if (*sta350).mclk == 0 {
        return -EIO;
    }

    rate = params_rate(params);
    ratio = (*sta350).mclk / rate;

    let mut i = 0usize;
    while i < interpolation_ratios.len() {
        if interpolation_ratios[i].fs as c_uint == rate {
            ir = interpolation_ratios[i].ir;
            break;
        }
        i += 1;
    }

    if ir < 0 {
        return -EINVAL;
    }

    i = 0;
    while i < 6 {
        if mcs_ratio_table[ir as usize][i] as c_uint == ratio {
            mcs = i as c_int;
            break;
        }
        i += 1;
    }

    if mcs < 0 {
        return -EINVAL;
    }

    confa = ((ir as c_uint) << STA350_CONFA_IR_SHIFT) | ((mcs as c_uint) << STA350_CONFA_MCS_SHIFT);
    confb = 0;

    match params_width(params) {
        24 | 32 => match (*sta350).format {
            x if x == SND_SOC_DAIFMT_I2S => confb |= 0x0,
            x if x == SND_SOC_DAIFMT_LEFT_J => confb |= 0x1,
            x if x == SND_SOC_DAIFMT_RIGHT_J => confb |= 0x2,
            _ => {}
        },
        20 => match (*sta350).format {
            x if x == SND_SOC_DAIFMT_I2S => confb |= 0x4,
            x if x == SND_SOC_DAIFMT_LEFT_J => confb |= 0x5,
            x if x == SND_SOC_DAIFMT_RIGHT_J => confb |= 0x6,
            _ => {}
        },
        18 => match (*sta350).format {
            x if x == SND_SOC_DAIFMT_I2S => confb |= 0x8,
            x if x == SND_SOC_DAIFMT_LEFT_J => confb |= 0x9,
            x if x == SND_SOC_DAIFMT_RIGHT_J => confb |= 0xa,
            _ => {}
        },
        16 => match (*sta350).format {
            x if x == SND_SOC_DAIFMT_I2S => confb |= 0x0,
            x if x == SND_SOC_DAIFMT_LEFT_J => confb |= 0xd,
            x if x == SND_SOC_DAIFMT_RIGHT_J => confb |= 0xe,
            _ => {}
        },
        _ => return -EINVAL,
    }

    ret = regmap_update_bits((*sta350).regmap, STA350_CONFA, STA350_CONFA_MCS_MASK | STA350_CONFA_IR_MASK, confa);
    if ret < 0 {
        return ret;
    }

    ret = regmap_update_bits((*sta350).regmap, STA350_CONFB, STA350_CONFB_SAI_MASK | STA350_CONFB_SAIFB, confb);
    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn sta350_startup_sequence(sta350: *mut sta350_priv) -> c_int {
    if !(*sta350).gpiod_power_down.is_null() {
        gpiod_set_value((*sta350).gpiod_power_down, 1);
    }

    if !(*sta350).gpiod_nreset.is_null() {
        gpiod_set_value((*sta350).gpiod_nreset, 0);
        mdelay(1);
        gpiod_set_value((*sta350).gpiod_nreset, 1);
        mdelay(1);
    }

    0
}

unsafe extern "C" fn sta350_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let sta350 = snd_soc_component_get_drvdata(component) as *mut sta350_priv;
    let dapm = snd_soc_component_to_dapm(component);
    let mut ret: c_int;

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {}
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {
            /* Full power on */
            regmap_update_bits((*sta350).regmap, STA350_CONFF, STA350_CONFF_PWDN | STA350_CONFF_EAPD, STA350_CONFF_PWDN | STA350_CONFF_EAPD);
        }
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == snd_soc_bias_level::SND_SOC_BIAS_OFF {
                ret = regulator_bulk_enable((*sta350).supplies.len() as c_uint, (*sta350).supplies.as_mut_ptr());
                if ret < 0 {
                    return ret;
                }
                sta350_startup_sequence(sta350);
                sta350_cache_sync(component);
            }

            /* Power down */
            regmap_update_bits((*sta350).regmap, STA350_CONFF, STA350_CONFF_PWDN | STA350_CONFF_EAPD, 0);
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            /* The chip runs through the power down sequence for us */
            regmap_update_bits((*sta350).regmap, STA350_CONFF, STA350_CONFF_PWDN | STA350_CONFF_EAPD, 0);

            /* power down: low */
            if !(*sta350).gpiod_power_down.is_null() {
                gpiod_set_value((*sta350).gpiod_power_down, 0);
            }

            if !(*sta350).gpiod_nreset.is_null() {
                gpiod_set_value((*sta350).gpiod_nreset, 0);
            }

            regulator_bulk_disable((*sta350).supplies.len() as c_uint, (*sta350).supplies.as_mut_ptr());
        }
    }
    0
}

static sta350_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(sta350_hw_params),
    set_sysclk: Some(sta350_set_dai_sysclk),
    set_fmt: Some(sta350_set_dai_fmt),
};

static mut sta350_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"sta350-hifi\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 2,
        rates: STA350_RATES,
        formats: STA350_FORMATS,
    },
    ops: &sta350_dai_ops,
};

unsafe extern "C" fn sta350_probe(component: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let sta350 = snd_soc_component_get_drvdata(component) as *mut sta350_priv;
    let pdata = (*sta350).pdata;
    let mut ret: c_int;
    let mut thermal: c_uint = 0;

    ret = regulator_bulk_enable((*sta350).supplies.len() as c_uint, (*sta350).supplies.as_mut_ptr());
    if ret < 0 {
        return ret;
    }

    ret = sta350_startup_sequence(sta350);
    if ret < 0 {
        return ret;
    }

    /* CONFA */
    if !(*pdata).thermal_warning_recovery {
        thermal |= STA350_CONFA_TWAB;
    }
    if !(*pdata).thermal_warning_adjustment {
        thermal |= STA350_CONFA_TWRB;
    }
    if !(*pdata).fault_detect_recovery {
        thermal |= STA350_CONFA_FDRB;
    }
    regmap_update_bits((*sta350).regmap, STA350_CONFA, STA350_CONFA_TWAB | STA350_CONFA_TWRB | STA350_CONFA_FDRB, thermal);

    /* CONFC */
    regmap_update_bits((*sta350).regmap, STA350_CONFC, STA350_CONFC_OM_MASK, ((*pdata).ffx_power_output_mode as c_uint) << STA350_CONFC_OM_SHIFT);
    regmap_update_bits((*sta350).regmap, STA350_CONFC, STA350_CONFC_CSZ_MASK, ((*pdata).drop_compensation_ns as c_uint) << STA350_CONFC_CSZ_SHIFT);
    regmap_update_bits((*sta350).regmap, STA350_CONFC, STA350_CONFC_OCRB, if (*pdata).oc_warning_adjustment { STA350_CONFC_OCRB } else { 0 });

    /* CONFE */
    regmap_update_bits((*sta350).regmap, STA350_CONFE, STA350_CONFE_MPCV, if (*pdata).max_power_use_mpcc { STA350_CONFE_MPCV } else { 0 });
    regmap_update_bits((*sta350).regmap, STA350_CONFE, STA350_CONFE_MPC, if (*pdata).max_power_correction { STA350_CONFE_MPC } else { 0 });
    regmap_update_bits((*sta350).regmap, STA350_CONFE, STA350_CONFE_AME, if (*pdata).am_reduction_mode { STA350_CONFE_AME } else { 0 });
    regmap_update_bits((*sta350).regmap, STA350_CONFE, STA350_CONFE_PWMS, if (*pdata).odd_pwm_speed_mode { STA350_CONFE_PWMS } else { 0 });
    regmap_update_bits((*sta350).regmap, STA350_CONFE, STA350_CONFE_DCCV, if (*pdata).distortion_compensation { STA350_CONFE_DCCV } else { 0 });
    /*  CONFF */
    regmap_update_bits((*sta350).regmap, STA350_CONFF, STA350_CONFF_IDE, if (*pdata).invalid_input_detect_mute { STA350_CONFF_IDE } else { 0 });
    regmap_update_bits((*sta350).regmap, STA350_CONFF, STA350_CONFF_OCFG_MASK, ((*pdata).output_conf as c_uint) << STA350_CONFF_OCFG_SHIFT);

    /* channel to output mapping */
    regmap_update_bits((*sta350).regmap, STA350_C1CFG, STA350_CxCFG_OM_MASK, ((*pdata).ch1_output_mapping as c_uint) << STA350_CxCFG_OM_SHIFT);
    regmap_update_bits((*sta350).regmap, STA350_C2CFG, STA350_CxCFG_OM_MASK, ((*pdata).ch2_output_mapping as c_uint) << STA350_CxCFG_OM_SHIFT);
    regmap_update_bits((*sta350).regmap, STA350_C3CFG, STA350_CxCFG_OM_MASK, ((*pdata).ch3_output_mapping as c_uint) << STA350_CxCFG_OM_SHIFT);

    /* miscellaneous registers */
    regmap_update_bits((*sta350).regmap, STA350_MISC1, STA350_MISC1_CPWMEN, if (*pdata).activate_mute_output { STA350_MISC1_CPWMEN } else { 0 });
    regmap_update_bits((*sta350).regmap, STA350_MISC1, STA350_MISC1_BRIDGOFF, if (*pdata).bridge_immediate_off { STA350_MISC1_BRIDGOFF } else { 0 });
    regmap_update_bits((*sta350).regmap, STA350_MISC1, STA350_MISC1_NSHHPEN, if (*pdata).noise_shape_dc_cut { STA350_MISC1_NSHHPEN } else { 0 });
    regmap_update_bits((*sta350).regmap, STA350_MISC1, STA350_MISC1_RPDNEN, if (*pdata).powerdown_master_vol { STA350_MISC1_RPDNEN } else { 0 });

    regmap_update_bits((*sta350).regmap, STA350_MISC2, STA350_MISC2_PNDLSL_MASK, ((*pdata).powerdown_delay_divider as c_uint) << STA350_MISC2_PNDLSL_SHIFT);

    /* initialize coefficient shadow RAM with reset values */
    let mut i = 4usize;
    while i <= 49 {
        (*sta350).coef_shadow[i] = 0x400000;
        i += 5;
    }
    i = 50;
    while i <= 54 {
        (*sta350).coef_shadow[i] = 0x7fffff;
        i += 1;
    }
    (*sta350).coef_shadow[55] = 0x5a9df7;
    (*sta350).coef_shadow[56] = 0x7fffff;
    (*sta350).coef_shadow[59] = 0x7fffff;
    (*sta350).coef_shadow[60] = 0x400000;
    (*sta350).coef_shadow[61] = 0x400000;

    snd_soc_dapm_force_bias_level(dapm, snd_soc_bias_level::SND_SOC_BIAS_STANDBY);
    /* Bias level configuration will have done an extra enable */
    regulator_bulk_disable((*sta350).supplies.len() as c_uint, (*sta350).supplies.as_mut_ptr());

    0
}

unsafe extern "C" fn sta350_remove(component: *mut snd_soc_component) {
    let sta350 = snd_soc_component_get_drvdata(component) as *mut sta350_priv;

    regulator_bulk_disable((*sta350).supplies.len() as c_uint, (*sta350).supplies.as_mut_ptr());
}

static sta350_component: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(sta350_probe),
    remove: Some(sta350_remove),
    set_bias_level: Some(sta350_set_bias_level),
    controls: sta350_snd_controls.as_ptr(),
    num_controls: sta350_snd_controls.len() as c_uint,
    dapm_widgets: sta350_dapm_widgets.as_ptr(),
    num_dapm_widgets: sta350_dapm_widgets.len() as c_uint,
    dapm_routes: sta350_dapm_routes.as_ptr(),
    num_dapm_routes: sta350_dapm_routes.len() as c_uint,
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static sta350_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: STA350_MISC2,
    reg_defaults: sta350_regs.as_ptr(),
    num_reg_defaults: sta350_regs.len() as c_uint,
    cache_type: REGCACHE_MAPLE,
    wr_table: &sta350_write_regs,
    rd_table: &sta350_read_regs,
    volatile_table: &sta350_volatile_regs,
};

// Original C conditional: #ifdef CONFIG_OF
static st350_dt_ids: [of_device_id; 2] = [
    of_device_id { compatible: b"st,sta350\0".as_ptr() as *const c_char },
    of_device_id { compatible: core::ptr::null() },
];
// MODULE_DEVICE_TABLE(of, st350_dt_ids);

static sta350_ffx_modes: [*const c_char; 4] = [
    b"drop-compensation\0".as_ptr() as *const c_char,
    b"tapered-compensation\0".as_ptr() as *const c_char,
    b"full-power-mode\0".as_ptr() as *const c_char,
    b"variable-drop-compensation\0".as_ptr() as *const c_char,
];

unsafe extern "C" fn sta350_probe_dt(dev: *mut device, sta350: *mut sta350_priv) -> c_int {
    let np = (*dev).of_node;
    let pdata: *mut sta350_platform_data;
    let mut ffx_power_mode: *const c_char = core::ptr::null();
    let mut tmp: u16;
    let mut tmp8: u8 = 0;

    pdata = devm_kzalloc(dev, core::mem::size_of::<sta350_platform_data>(), GFP_KERNEL) as *mut sta350_platform_data;
    if pdata.is_null() {
        return -ENOMEM;
    }

    of_property_read_u8(np, b"st,output-conf\0".as_ptr() as *const c_char, &mut (*pdata).output_conf);
    of_property_read_u8(np, b"st,ch1-output-mapping\0".as_ptr() as *const c_char, &mut (*pdata).ch1_output_mapping);
    of_property_read_u8(np, b"st,ch2-output-mapping\0".as_ptr() as *const c_char, &mut (*pdata).ch2_output_mapping);
    of_property_read_u8(np, b"st,ch3-output-mapping\0".as_ptr() as *const c_char, &mut (*pdata).ch3_output_mapping);

    (*pdata).thermal_warning_recovery = of_property_read_bool(np, b"st,thermal-warning-recovery\0".as_ptr() as *const c_char);
    (*pdata).thermal_warning_adjustment = of_property_read_bool(np, b"st,thermal-warning-adjustment\0".as_ptr() as *const c_char);
    (*pdata).fault_detect_recovery = of_property_read_bool(np, b"st,fault-detect-recovery\0".as_ptr() as *const c_char);

    (*pdata).ffx_power_output_mode = STA350_FFX_PM_VARIABLE_DROP_COMP as c_int;
    if of_property_read_string(np, b"st,ffx-power-output-mode\0".as_ptr() as *const c_char, &mut ffx_power_mode) == 0 {
        let mut i: usize = 0;
        let mut mode: c_int = -EINVAL;

        while i < sta350_ffx_modes.len() {
            if strcasecmp(ffx_power_mode, sta350_ffx_modes[i]) == 0 {
                mode = i as c_int;
            }
            i += 1;
        }

        if mode >= 0 {
            (*pdata).ffx_power_output_mode = mode;
        }
    }

    tmp = 140;
    of_property_read_u16(np, b"st,drop-compensation-ns\0".as_ptr() as *const c_char, &mut tmp);
    (*pdata).drop_compensation_ns = (tmp.clamp(0, 300) / 20) as c_int;

    (*pdata).oc_warning_adjustment = of_property_read_bool(np, b"st,overcurrent-warning-adjustment\0".as_ptr() as *const c_char);

    /* CONFE */
    (*pdata).max_power_use_mpcc = of_property_read_bool(np, b"st,max-power-use-mpcc\0".as_ptr() as *const c_char);
    (*pdata).max_power_correction = of_property_read_bool(np, b"st,max-power-correction\0".as_ptr() as *const c_char);
    (*pdata).am_reduction_mode = of_property_read_bool(np, b"st,am-reduction-mode\0".as_ptr() as *const c_char);
    (*pdata).odd_pwm_speed_mode = of_property_read_bool(np, b"st,odd-pwm-speed-mode\0".as_ptr() as *const c_char);
    (*pdata).distortion_compensation = of_property_read_bool(np, b"st,distortion-compensation\0".as_ptr() as *const c_char);

    /* CONFF */
    (*pdata).invalid_input_detect_mute = of_property_read_bool(np, b"st,invalid-input-detect-mute\0".as_ptr() as *const c_char);

    /* MISC */
    (*pdata).activate_mute_output = of_property_read_bool(np, b"st,activate-mute-output\0".as_ptr() as *const c_char);
    (*pdata).bridge_immediate_off = of_property_read_bool(np, b"st,bridge-immediate-off\0".as_ptr() as *const c_char);
    (*pdata).noise_shape_dc_cut = of_property_read_bool(np, b"st,noise-shape-dc-cut\0".as_ptr() as *const c_char);
    (*pdata).powerdown_master_vol = of_property_read_bool(np, b"st,powerdown-master-volume\0".as_ptr() as *const c_char);

    if of_property_read_u8(np, b"st,powerdown-delay-divider\0".as_ptr() as *const c_char, &mut tmp8) == 0 {
        if is_power_of_2(tmp8) && tmp8 >= 1 && tmp8 <= 128 {
            (*pdata).powerdown_delay_divider = ilog2(tmp8);
        }
    }

    (*sta350).pdata = pdata;

    0
}
// End original CONFIG_OF block.

unsafe extern "C" fn sta350_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let dev = &mut (*i2c).dev as *mut device;
    let sta350: *mut sta350_priv;
    let mut ret: c_int;
    let mut i: usize;

    sta350 = devm_kzalloc(dev, core::mem::size_of::<sta350_priv>(), GFP_KERNEL) as *mut sta350_priv;
    if sta350.is_null() {
        return -ENOMEM;
    }

    mutex_init(&mut (*sta350).coeff_lock);
    (*sta350).pdata = dev_get_platdata(dev);

    // Original C conditional: #ifdef CONFIG_OF
    if !(*dev).of_node.is_null() {
        ret = sta350_probe_dt(dev, sta350);
        if ret < 0 {
            return ret;
        }
    }

    /* GPIOs */
    (*sta350).gpiod_nreset = devm_gpiod_get_optional(dev, b"reset\0".as_ptr() as *const c_char, GPIOD_OUT_LOW);
    if IS_ERR((*sta350).gpiod_nreset as *const c_void) {
        return PTR_ERR((*sta350).gpiod_nreset as *const c_void);
    }

    (*sta350).gpiod_power_down = devm_gpiod_get_optional(dev, b"power-down\0".as_ptr() as *const c_char, GPIOD_OUT_LOW);
    if IS_ERR((*sta350).gpiod_power_down as *const c_void) {
        return PTR_ERR((*sta350).gpiod_power_down as *const c_void);
    }

    /* regulators */
    i = 0;
    while i < (*sta350).supplies.len() {
        (*sta350).supplies[i].supply = sta350_supply_names[i];
        i += 1;
    }

    ret = devm_regulator_bulk_get(dev, (*sta350).supplies.len() as c_uint, (*sta350).supplies.as_mut_ptr());
    if ret < 0 {
        return ret;
    }

    (*sta350).regmap = devm_regmap_init_i2c(i2c, &sta350_regmap);
    if IS_ERR((*sta350).regmap as *const c_void) {
        ret = PTR_ERR((*sta350).regmap as *const c_void);
        return ret;
    }

    i2c_set_clientdata(i2c, sta350 as *mut c_void);

    ret = devm_snd_soc_register_component(dev, &sta350_component, &mut sta350_dai, 1);
    ret
}

static sta350_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: b"sta350\0".as_ptr() as *const c_char },
    i2c_device_id { name: core::ptr::null() },
];
// MODULE_DEVICE_TABLE(i2c, sta350_i2c_id);

static mut sta350_i2c_driver: i2c_driver = i2c_driver {
    driver: i2c_driver_driver {
        name: b"sta350\0".as_ptr() as *const c_char,
        of_match_table: st350_dt_ids.as_ptr(),
    },
    probe: Some(sta350_i2c_probe),
    id_table: sta350_i2c_id.as_ptr(),
};

// module_i2c_driver(sta350_i2c_driver);
// MODULE_DESCRIPTION("ASoC STA350 driver");
// MODULE_AUTHOR("Sven Brandau <info@brandau.biz>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
