// SPDX-License-Identifier: GPL-2.0-only
/*
 * cs35l35.c -- CS35L35 ALSA SoC audio driver
 *
 * Copyright 2017 Cirrus Logic, Inc.
 *
 * Author: Brian Austin <brian.austin@cirrus.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

/* Dependencies from Linux, ALSA SoC, cs35l35.h, and cirrus_legacy.h are external. */

type bool_ = bool;
type u8 = u8;
type irqreturn_t = c_int;

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
pub struct completion {
    _private: [u8; 0],
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
pub struct snd_kcontrol {
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
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: c_int,
}
#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct i2c_client {
    pub dev: device_with_of,
    pub irq: c_int,
}
#[repr(C)]
pub struct device_with_of {
    pub of_node: *mut device_node,
}
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
}
#[repr(C)]
pub struct classh_cfg {
    pub classh_algo_enable: bool,
    pub classh_bst_override: c_uint,
    pub classh_bst_max_limit: c_uint,
    pub classh_mem_depth: c_uint,
    pub classh_headroom: c_uint,
    pub classh_release_rate: c_uint,
    pub classh_wk_fet_disable: c_uint,
    pub classh_wk_fet_delay: c_uint,
    pub classh_wk_fet_thld: c_uint,
    pub classh_vpch_auto: c_uint,
    pub classh_vpch_rate: c_uint,
    pub classh_vpch_man: c_uint,
}
#[repr(C)]
pub struct monitor_cfg {
    pub is_present: bool,
    pub vmon_specs: bool,
    pub imon_specs: bool,
    pub vpmon_specs: bool,
    pub vbstmon_specs: bool,
    pub vpbrstat_specs: bool,
    pub zerofill_specs: bool,
    pub vmon_dpth: u8,
    pub vmon_loc: u8,
    pub vmon_frm: u8,
    pub imon_dpth: u8,
    pub imon_loc: u8,
    pub imon_frm: u8,
    pub imon_scale: u8,
    pub vpmon_dpth: u8,
    pub vpmon_loc: u8,
    pub vpmon_frm: u8,
    pub vbstmon_dpth: u8,
    pub vbstmon_loc: u8,
    pub vbstmon_frm: u8,
    pub vpbrstat_dpth: u8,
    pub vpbrstat_loc: u8,
    pub vpbrstat_frm: u8,
    pub zerofill_dpth: u8,
    pub zerofill_loc: u8,
    pub zerofill_frm: u8,
}
#[repr(C)]
pub struct cs35l35_platform_data {
    pub ext_bst: bool,
    pub bst_pdn_fet_on: bool,
    pub bst_vctl: c_uint,
    pub bst_ipk: c_uint,
    pub boost_ind: c_uint,
    pub gain_zc: bool,
    pub aud_channel: c_uint,
    pub adv_channel: c_uint,
    pub stereo: bool,
    pub shared_bst: bool,
    pub sp_drv_str: c_uint,
    pub sp_drv_unused: c_uint,
    pub classh_algo: classh_cfg,
    pub mon_cfg: monitor_cfg,
}
#[repr(C)]
pub struct cs35l35_private {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub reset_gpio: *mut gpio_desc,
    pub pdata: cs35l35_platform_data,
    pub pdn_done: completion,
    pub pdm_mode: bool,
    pub i2s_mode: bool,
    pub clock_consumer: bool,
    pub sysclk: c_uint,
    pub sclk: c_uint,
    pub supplies: [regulator_bulk_data; 0],
    pub num_supplies: c_int,
}
#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}
#[repr(C)]
pub struct reg_sequence {
    pub reg: c_uint,
    pub def: c_uint,
}
#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}
#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
    _private: [u8; 0],
}
type snd_soc_dapm_widget_item = snd_soc_dapm_widget_desc;
#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *const c_uint,
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
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
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_int) -> c_int>,
    pub dapm_widgets: *const snd_soc_dapm_widget_item,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}
#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub precious_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_uint,
    pub use_single_read: bool,
    pub use_single_write: bool,
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
pub struct driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}
#[repr(C)]
pub struct i2c_driver {
    pub driver: driver_inner,
    pub id_table: *const i2c_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
}

unsafe extern "C" {
    static cs35l35_supplies: [*const c_char; 0];

    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn usleep_range(min: c_uint, max: c_uint);
    fn reinit_completion(x: *mut completion);
    fn wait_for_completion_timeout(x: *mut completion, timeout: c_uint) -> c_int;
    fn msecs_to_jiffies(msecs: c_uint) -> c_uint;
    fn complete(x: *mut completion);
    fn init_completion(x: *mut completion);
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_bulk_read(map: *mut regmap, reg: c_uint, val: *mut c_void, val_count: usize) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_register_patch(map: *mut regmap, patch: *const reg_sequence, patch_regs: usize) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_uint, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn snd_soc_add_component_controls(component: *mut snd_soc_component, controls: *const snd_kcontrol_new, num_controls: usize) -> c_int;
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut c_void;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_regulator_bulk_get(dev: *mut device, num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_enable(num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn devm_request_threaded_irq(dev: *mut device, irq: c_int, handler: *const c_void, thread_fn: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_uint, name: *const c_char, data: *mut c_void) -> c_int;
    fn cirrus_read_device_id(map: *mut regmap, reg: c_uint) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: usize) -> c_int;
    fn of_property_read_bool(np: *mut device_node, propname: *const c_char) -> bool;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut c_uint) -> c_int;
    fn of_property_read_u8_array(np: *mut device_node, propname: *const c_char, out_values: *mut u8, sz: c_int) -> c_int;
    fn of_get_child_by_name(np: *mut device_node, name: *const c_char) -> *mut device_node;
    fn of_node_put(np: *mut device_node);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_crit(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

/*
 * Some fields take zero as a valid value so use a high bit flag that won't
 * get written to the device to mark those.
 */
const CS35L35_VALID_PDATA: c_uint = 0x80000000;

static cs35l35_reg: [reg_default; 61] = [
    reg_default { reg: CS35L35_PWRCTL1, def: 0x01 },
    reg_default { reg: CS35L35_PWRCTL2, def: 0x11 },
    reg_default { reg: CS35L35_PWRCTL3, def: 0x00 },
    reg_default { reg: CS35L35_CLK_CTL1, def: 0x04 },
    reg_default { reg: CS35L35_CLK_CTL2, def: 0x12 },
    reg_default { reg: CS35L35_CLK_CTL3, def: 0xCF },
    reg_default { reg: CS35L35_SP_FMT_CTL1, def: 0x20 },
    reg_default { reg: CS35L35_SP_FMT_CTL2, def: 0x00 },
    reg_default { reg: CS35L35_SP_FMT_CTL3, def: 0x02 },
    reg_default { reg: CS35L35_MAG_COMP_CTL, def: 0x00 },
    reg_default { reg: CS35L35_AMP_INP_DRV_CTL, def: 0x01 },
    reg_default { reg: CS35L35_AMP_DIG_VOL_CTL, def: 0x12 },
    reg_default { reg: CS35L35_AMP_DIG_VOL, def: 0x00 },
    reg_default { reg: CS35L35_ADV_DIG_VOL, def: 0x00 },
    reg_default { reg: CS35L35_PROTECT_CTL, def: 0x06 },
    reg_default { reg: CS35L35_AMP_GAIN_AUD_CTL, def: 0x13 },
    reg_default { reg: CS35L35_AMP_GAIN_PDM_CTL, def: 0x00 },
    reg_default { reg: CS35L35_AMP_GAIN_ADV_CTL, def: 0x00 },
    reg_default { reg: CS35L35_GPI_CTL, def: 0x00 },
    reg_default { reg: CS35L35_BST_CVTR_V_CTL, def: 0x00 },
    reg_default { reg: CS35L35_BST_PEAK_I, def: 0x07 },
    reg_default { reg: CS35L35_BST_RAMP_CTL, def: 0x85 },
    reg_default { reg: CS35L35_BST_CONV_COEF_1, def: 0x24 },
    reg_default { reg: CS35L35_BST_CONV_COEF_2, def: 0x24 },
    reg_default { reg: CS35L35_BST_CONV_SLOPE_COMP, def: 0x4E },
    reg_default { reg: CS35L35_BST_CONV_SW_FREQ, def: 0x04 },
    reg_default { reg: CS35L35_CLASS_H_CTL, def: 0x0B },
    reg_default { reg: CS35L35_CLASS_H_HEADRM_CTL, def: 0x0B },
    reg_default { reg: CS35L35_CLASS_H_RELEASE_RATE, def: 0x08 },
    reg_default { reg: CS35L35_CLASS_H_FET_DRIVE_CTL, def: 0x41 },
    reg_default { reg: CS35L35_CLASS_H_VP_CTL, def: 0xC5 },
    reg_default { reg: CS35L35_VPBR_CTL, def: 0x0A },
    reg_default { reg: CS35L35_VPBR_VOL_CTL, def: 0x90 },
    reg_default { reg: CS35L35_VPBR_TIMING_CTL, def: 0x6A },
    reg_default { reg: CS35L35_VPBR_MODE_VOL_CTL, def: 0x00 },
    reg_default { reg: CS35L35_SPKR_MON_CTL, def: 0xC0 },
    reg_default { reg: CS35L35_IMON_SCALE_CTL, def: 0x30 },
    reg_default { reg: CS35L35_AUDIN_RXLOC_CTL, def: 0x00 },
    reg_default { reg: CS35L35_ADVIN_RXLOC_CTL, def: 0x80 },
    reg_default { reg: CS35L35_VMON_TXLOC_CTL, def: 0x00 },
    reg_default { reg: CS35L35_IMON_TXLOC_CTL, def: 0x80 },
    reg_default { reg: CS35L35_VPMON_TXLOC_CTL, def: 0x04 },
    reg_default { reg: CS35L35_VBSTMON_TXLOC_CTL, def: 0x84 },
    reg_default { reg: CS35L35_VPBR_STATUS_TXLOC_CTL, def: 0x04 },
    reg_default { reg: CS35L35_ZERO_FILL_LOC_CTL, def: 0x00 },
    reg_default { reg: CS35L35_AUDIN_DEPTH_CTL, def: 0x0F },
    reg_default { reg: CS35L35_SPKMON_DEPTH_CTL, def: 0x0F },
    reg_default { reg: CS35L35_SUPMON_DEPTH_CTL, def: 0x0F },
    reg_default { reg: CS35L35_ZEROFILL_DEPTH_CTL, def: 0x00 },
    reg_default { reg: CS35L35_MULT_DEV_SYNCH1, def: 0x02 },
    reg_default { reg: CS35L35_MULT_DEV_SYNCH2, def: 0x80 },
    reg_default { reg: CS35L35_PROT_RELEASE_CTL, def: 0x00 },
    reg_default { reg: CS35L35_DIAG_MODE_REG_LOCK, def: 0x00 },
    reg_default { reg: CS35L35_DIAG_MODE_CTL_1, def: 0x40 },
    reg_default { reg: CS35L35_DIAG_MODE_CTL_2, def: 0x00 },
    reg_default { reg: CS35L35_INT_MASK_1, def: 0xFF },
    reg_default { reg: CS35L35_INT_MASK_2, def: 0xFF },
    reg_default { reg: CS35L35_INT_MASK_3, def: 0xFF },
    reg_default { reg: CS35L35_INT_MASK_4, def: 0xFF },
];

unsafe extern "C" fn cs35l35_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        CS35L35_INT_STATUS_1 | CS35L35_INT_STATUS_2 | CS35L35_INT_STATUS_3 | CS35L35_INT_STATUS_4
        | CS35L35_PLL_STATUS | CS35L35_OTP_TRIM_STATUS => true,
        _ => false,
    }
}

unsafe extern "C" fn cs35l35_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        CS35L35_DEVID_AB..=CS35L35_PWRCTL3
        | CS35L35_CLK_CTL1..=CS35L35_SP_FMT_CTL3
        | CS35L35_MAG_COMP_CTL..=CS35L35_AMP_GAIN_AUD_CTL
        | CS35L35_AMP_GAIN_PDM_CTL..=CS35L35_BST_PEAK_I
        | CS35L35_BST_RAMP_CTL..=CS35L35_BST_CONV_SW_FREQ
        | CS35L35_CLASS_H_CTL..=CS35L35_CLASS_H_VP_CTL
        | CS35L35_CLASS_H_STATUS
        | CS35L35_VPBR_CTL..=CS35L35_VPBR_MODE_VOL_CTL
        | CS35L35_VPBR_ATTEN_STATUS
        | CS35L35_SPKR_MON_CTL
        | CS35L35_IMON_SCALE_CTL..=CS35L35_ZEROFILL_DEPTH_CTL
        | CS35L35_MULT_DEV_SYNCH1..=CS35L35_PROT_RELEASE_CTL
        | CS35L35_DIAG_MODE_REG_LOCK..=CS35L35_DIAG_MODE_CTL_2
        | CS35L35_INT_MASK_1..=CS35L35_PLL_STATUS
        | CS35L35_OTP_TRIM_STATUS => true,
        _ => false,
    }
}

unsafe extern "C" fn cs35l35_precious_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        CS35L35_INT_STATUS_1 | CS35L35_INT_STATUS_2 | CS35L35_INT_STATUS_3 | CS35L35_INT_STATUS_4
        | CS35L35_PLL_STATUS | CS35L35_OTP_TRIM_STATUS => true,
        _ => false,
    }
}

unsafe fn cs35l35_reset(cs35l35: *mut cs35l35_private) {
    unsafe {
        gpiod_set_value_cansleep((*cs35l35).reset_gpio, 0);
        usleep_range(2000, 2100);
        gpiod_set_value_cansleep((*cs35l35).reset_gpio, 1);
        usleep_range(1000, 1100);
    }
}

unsafe fn cs35l35_wait_for_pdn(cs35l35: *mut cs35l35_private) -> c_int {
    unsafe {
        let ret: c_int;
        if (*cs35l35).pdata.ext_bst {
            usleep_range(5000, 5500);
            return 0;
        }
        reinit_completion(&mut (*cs35l35).pdn_done);
        ret = wait_for_completion_timeout(&mut (*cs35l35).pdn_done, msecs_to_jiffies(100));
        if ret == 0 {
            dev_err((*cs35l35).dev, c"PDN_DONE did not complete\n".as_ptr());
            return -ETIMEDOUT;
        }
        0
    }
}

unsafe extern "C" fn cs35l35_sdin_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    unsafe {
        let component = snd_soc_dapm_to_component((*w).dapm);
        let cs35l35 = snd_soc_component_get_drvdata(component) as *mut cs35l35_private;
        let mut ret = 0;
        match event {
            SND_SOC_DAPM_PRE_PMU => {
                regmap_update_bits((*cs35l35).regmap, CS35L35_CLK_CTL1, CS35L35_MCLK_DIS_MASK, 0 << CS35L35_MCLK_DIS_SHIFT);
                regmap_update_bits((*cs35l35).regmap, CS35L35_PWRCTL1, CS35L35_DISCHG_FILT_MASK, 0 << CS35L35_DISCHG_FILT_SHIFT);
                regmap_update_bits((*cs35l35).regmap, CS35L35_PWRCTL1, CS35L35_PDN_ALL_MASK, 0);
            }
            SND_SOC_DAPM_POST_PMD => {
                regmap_update_bits((*cs35l35).regmap, CS35L35_PWRCTL1, CS35L35_DISCHG_FILT_MASK, 1 << CS35L35_DISCHG_FILT_SHIFT);
                regmap_update_bits((*cs35l35).regmap, CS35L35_PWRCTL1, CS35L35_PDN_ALL_MASK, 1);
                /* Already muted, so disable volume ramp for faster shutdown */
                regmap_update_bits((*cs35l35).regmap, CS35L35_AMP_DIG_VOL_CTL, CS35L35_AMP_DIGSFT_MASK, 0);
                ret = cs35l35_wait_for_pdn(cs35l35);
                regmap_update_bits((*cs35l35).regmap, CS35L35_CLK_CTL1, CS35L35_MCLK_DIS_MASK, 1 << CS35L35_MCLK_DIS_SHIFT);
                regmap_update_bits((*cs35l35).regmap, CS35L35_AMP_DIG_VOL_CTL, CS35L35_AMP_DIGSFT_MASK, 1 << CS35L35_AMP_DIGSFT_SHIFT);
            }
            _ => {
                dev_err((*component).dev, c"Invalid event = 0x%x\n".as_ptr(), event);
                ret = -EINVAL;
            }
        }
        ret
    }
}

unsafe extern "C" fn cs35l35_main_amp_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    unsafe {
        let component = snd_soc_dapm_to_component((*w).dapm);
        let cs35l35 = snd_soc_component_get_drvdata(component) as *mut cs35l35_private;
        let mut reg: [c_uint; 4] = [0; 4];
        match event {
            SND_SOC_DAPM_PRE_PMU => {
                if (*cs35l35).pdata.bst_pdn_fet_on {
                    regmap_update_bits((*cs35l35).regmap, CS35L35_PWRCTL2, CS35L35_PDN_BST_MASK, 0 << CS35L35_PDN_BST_FETON_SHIFT);
                } else {
                    regmap_update_bits((*cs35l35).regmap, CS35L35_PWRCTL2, CS35L35_PDN_BST_MASK, 0 << CS35L35_PDN_BST_FETOFF_SHIFT);
                }
            }
            SND_SOC_DAPM_POST_PMU => {
                usleep_range(5000, 5100);
                /* If in PDM mode we must use VP for Voltage control */
                if (*cs35l35).pdm_mode {
                    regmap_update_bits((*cs35l35).regmap, CS35L35_BST_CVTR_V_CTL, CS35L35_BST_CTL_MASK, 0 << CS35L35_BST_CTL_SHIFT);
                }
                regmap_update_bits((*cs35l35).regmap, CS35L35_PROTECT_CTL, CS35L35_AMP_MUTE_MASK, 0);
                for _i in 0..2 {
                    regmap_bulk_read((*cs35l35).regmap, CS35L35_INT_STATUS_1, reg.as_mut_ptr() as *mut c_void, ARRAY_SIZE(&reg));
                }
            }
            SND_SOC_DAPM_PRE_PMD => {
                regmap_update_bits((*cs35l35).regmap, CS35L35_PROTECT_CTL, CS35L35_AMP_MUTE_MASK, 1 << CS35L35_AMP_MUTE_SHIFT);
                if (*cs35l35).pdata.bst_pdn_fet_on {
                    regmap_update_bits((*cs35l35).regmap, CS35L35_PWRCTL2, CS35L35_PDN_BST_MASK, 1 << CS35L35_PDN_BST_FETON_SHIFT);
                } else {
                    regmap_update_bits((*cs35l35).regmap, CS35L35_PWRCTL2, CS35L35_PDN_BST_MASK, 1 << CS35L35_PDN_BST_FETOFF_SHIFT);
                }
            }
            SND_SOC_DAPM_POST_PMD => {
                usleep_range(5000, 5100);
                /*
                 * If PDM mode we should switch back to pdata value
                 * for Voltage control when we go down
                 */
                if (*cs35l35).pdm_mode {
                    regmap_update_bits((*cs35l35).regmap, CS35L35_BST_CVTR_V_CTL, CS35L35_BST_CTL_MASK, (*cs35l35).pdata.bst_vctl << CS35L35_BST_CTL_SHIFT);
                }
            }
            _ => dev_err((*component).dev, c"Invalid event = 0x%x\n".as_ptr(), event),
        }
        0
    }
}

/* static DECLARE_TLV_DB_SCALE(amp_gain_tlv, 0, 1, 1); */
/* static DECLARE_TLV_DB_SCALE(dig_vol_tlv, -10200, 50, 0); */
extern "C" {
    static amp_gain_tlv: c_uint;
    static dig_vol_tlv: c_uint;
}

/* SOC_* and SND_SOC_DAPM_* macro initializer expansions are external to this isolated file. */
static cs35l35_aud_controls: [snd_kcontrol_new; 0] = [];
static cs35l35_adv_controls: [snd_kcontrol_new; 0] = [];
static cs35l35_dapm_widgets: [snd_soc_dapm_widget_item; 0] = [];

static cs35l35_audio_map: [snd_soc_dapm_route; 13] = [
    snd_soc_dapm_route { sink: c"VPMON ADC".as_ptr(), control: ptr::null(), source: c"VP".as_ptr() },
    snd_soc_dapm_route { sink: c"VBSTMON ADC".as_ptr(), control: ptr::null(), source: c"VBST".as_ptr() },
    snd_soc_dapm_route { sink: c"IMON ADC".as_ptr(), control: ptr::null(), source: c"ISENSE".as_ptr() },
    snd_soc_dapm_route { sink: c"VMON ADC".as_ptr(), control: ptr::null(), source: c"VSENSE".as_ptr() },
    snd_soc_dapm_route { sink: c"SDOUT".as_ptr(), control: ptr::null(), source: c"IMON ADC".as_ptr() },
    snd_soc_dapm_route { sink: c"SDOUT".as_ptr(), control: ptr::null(), source: c"VMON ADC".as_ptr() },
    snd_soc_dapm_route { sink: c"SDOUT".as_ptr(), control: ptr::null(), source: c"VBSTMON ADC".as_ptr() },
    snd_soc_dapm_route { sink: c"SDOUT".as_ptr(), control: ptr::null(), source: c"VPMON ADC".as_ptr() },
    snd_soc_dapm_route { sink: c"AMP Capture".as_ptr(), control: ptr::null(), source: c"SDOUT".as_ptr() },
    snd_soc_dapm_route { sink: c"SDIN".as_ptr(), control: ptr::null(), source: c"AMP Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"CLASS H".as_ptr(), control: ptr::null(), source: c"SDIN".as_ptr() },
    snd_soc_dapm_route { sink: c"Main AMP".as_ptr(), control: ptr::null(), source: c"CLASS H".as_ptr() },
    snd_soc_dapm_route { sink: c"SPK".as_ptr(), control: ptr::null(), source: c"Main AMP".as_ptr() },
];

unsafe extern "C" fn cs35l35_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    unsafe {
        let component = (*codec_dai).component;
        let cs35l35 = snd_soc_component_get_drvdata(component) as *mut cs35l35_private;
        match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
            SND_SOC_DAIFMT_CBP_CFP => {
                regmap_update_bits((*cs35l35).regmap, CS35L35_CLK_CTL1, CS35L35_MS_MASK, 1 << CS35L35_MS_SHIFT);
                (*cs35l35).clock_consumer = false;
            }
            SND_SOC_DAIFMT_CBC_CFC => {
                regmap_update_bits((*cs35l35).regmap, CS35L35_CLK_CTL1, CS35L35_MS_MASK, 0 << CS35L35_MS_SHIFT);
                (*cs35l35).clock_consumer = true;
            }
            _ => return -EINVAL,
        }
        match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
            SND_SOC_DAIFMT_I2S => {
                (*cs35l35).i2s_mode = true;
                (*cs35l35).pdm_mode = false;
            }
            SND_SOC_DAIFMT_PDM => {
                (*cs35l35).pdm_mode = true;
                (*cs35l35).i2s_mode = false;
            }
            _ => return -EINVAL,
        }
        0
    }
}

#[repr(C)]
struct cs35l35_sysclk_config {
    sysclk: c_int,
    srate: c_int,
    clk_cfg: u8,
}

static mut cs35l35_clk_ctl: [cs35l35_sysclk_config; 40] = [
    /* SYSCLK, Sample Rate, Serial Port Cfg */
    cs35l35_sysclk_config { sysclk: 5644800, srate: 44100, clk_cfg: 0x00 },
    cs35l35_sysclk_config { sysclk: 5644800, srate: 88200, clk_cfg: 0x40 },
    cs35l35_sysclk_config { sysclk: 6144000, srate: 48000, clk_cfg: 0x10 },
    cs35l35_sysclk_config { sysclk: 6144000, srate: 96000, clk_cfg: 0x50 },
    cs35l35_sysclk_config { sysclk: 11289600, srate: 44100, clk_cfg: 0x01 },
    cs35l35_sysclk_config { sysclk: 11289600, srate: 88200, clk_cfg: 0x41 },
    cs35l35_sysclk_config { sysclk: 11289600, srate: 176400, clk_cfg: 0x81 },
    cs35l35_sysclk_config { sysclk: 12000000, srate: 44100, clk_cfg: 0x03 },
    cs35l35_sysclk_config { sysclk: 12000000, srate: 48000, clk_cfg: 0x13 },
    cs35l35_sysclk_config { sysclk: 12000000, srate: 88200, clk_cfg: 0x43 },
    cs35l35_sysclk_config { sysclk: 12000000, srate: 96000, clk_cfg: 0x53 },
    cs35l35_sysclk_config { sysclk: 12000000, srate: 176400, clk_cfg: 0x83 },
    cs35l35_sysclk_config { sysclk: 12000000, srate: 192000, clk_cfg: 0x93 },
    cs35l35_sysclk_config { sysclk: 12288000, srate: 48000, clk_cfg: 0x11 },
    cs35l35_sysclk_config { sysclk: 12288000, srate: 96000, clk_cfg: 0x51 },
    cs35l35_sysclk_config { sysclk: 12288000, srate: 192000, clk_cfg: 0x91 },
    cs35l35_sysclk_config { sysclk: 13000000, srate: 44100, clk_cfg: 0x07 },
    cs35l35_sysclk_config { sysclk: 13000000, srate: 48000, clk_cfg: 0x17 },
    cs35l35_sysclk_config { sysclk: 13000000, srate: 88200, clk_cfg: 0x47 },
    cs35l35_sysclk_config { sysclk: 13000000, srate: 96000, clk_cfg: 0x57 },
    cs35l35_sysclk_config { sysclk: 13000000, srate: 176400, clk_cfg: 0x87 },
    cs35l35_sysclk_config { sysclk: 13000000, srate: 192000, clk_cfg: 0x97 },
    cs35l35_sysclk_config { sysclk: 22579200, srate: 44100, clk_cfg: 0x02 },
    cs35l35_sysclk_config { sysclk: 22579200, srate: 88200, clk_cfg: 0x42 },
    cs35l35_sysclk_config { sysclk: 22579200, srate: 176400, clk_cfg: 0x82 },
    cs35l35_sysclk_config { sysclk: 24000000, srate: 44100, clk_cfg: 0x0B },
    cs35l35_sysclk_config { sysclk: 24000000, srate: 48000, clk_cfg: 0x1B },
    cs35l35_sysclk_config { sysclk: 24000000, srate: 88200, clk_cfg: 0x4B },
    cs35l35_sysclk_config { sysclk: 24000000, srate: 96000, clk_cfg: 0x5B },
    cs35l35_sysclk_config { sysclk: 24000000, srate: 176400, clk_cfg: 0x8B },
    cs35l35_sysclk_config { sysclk: 24000000, srate: 192000, clk_cfg: 0x9B },
    cs35l35_sysclk_config { sysclk: 24576000, srate: 48000, clk_cfg: 0x12 },
    cs35l35_sysclk_config { sysclk: 24576000, srate: 96000, clk_cfg: 0x52 },
    cs35l35_sysclk_config { sysclk: 24576000, srate: 192000, clk_cfg: 0x92 },
    cs35l35_sysclk_config { sysclk: 26000000, srate: 44100, clk_cfg: 0x0F },
    cs35l35_sysclk_config { sysclk: 26000000, srate: 48000, clk_cfg: 0x1F },
    cs35l35_sysclk_config { sysclk: 26000000, srate: 88200, clk_cfg: 0x4F },
    cs35l35_sysclk_config { sysclk: 26000000, srate: 96000, clk_cfg: 0x5F },
    cs35l35_sysclk_config { sysclk: 26000000, srate: 176400, clk_cfg: 0x8F },
    cs35l35_sysclk_config { sysclk: 26000000, srate: 192000, clk_cfg: 0x9F },
];

unsafe fn cs35l35_get_clk_config(sysclk: c_int, srate: c_int) -> c_int {
    unsafe {
        let mut i = 0usize;
        while i < cs35l35_clk_ctl.len() {
            if cs35l35_clk_ctl[i].sysclk == sysclk && cs35l35_clk_ctl[i].srate == srate {
                return cs35l35_clk_ctl[i].clk_cfg as c_int;
            }
            i += 1;
        }
        -EINVAL
    }
}

unsafe extern "C" fn cs35l35_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    unsafe {
        let component = (*dai).component;
        let cs35l35 = snd_soc_component_get_drvdata(component) as *mut cs35l35_private;
        let classh = &mut (*cs35l35).pdata.classh_algo as *mut classh_cfg;
        let srate = params_rate(params);
        let mut ret = 0;
        let sp_sclks: u8;
        let audin_format: c_int;
        let errata_chk: c_int;
        let clk_ctl = cs35l35_get_clk_config((*cs35l35).sysclk as c_int, srate);
        if clk_ctl < 0 {
            dev_err((*component).dev, c"Invalid CLK:Rate %d:%d\n".as_ptr(), (*cs35l35).sysclk, srate);
            return -EINVAL;
        }
        ret = regmap_update_bits((*cs35l35).regmap, CS35L35_CLK_CTL2, CS35L35_CLK_CTL2_MASK, clk_ctl as c_uint);
        if ret != 0 {
            dev_err((*component).dev, c"Failed to set port config %d\n".as_ptr(), ret);
            return ret;
        }
        /*
         * Rev A0 Errata
         * When configured for the weak-drive detection path (CH_WKFET_DIS = 0)
         * the Class H algorithm does not enable weak-drive operation for
         * nonzero values of CH_WKFET_DELAY if SP_RATE = 01 or 10
         */
        errata_chk = ((clk_ctl as c_uint & CS35L35_SP_RATE_MASK) >> CS35L35_SP_RATE_SHIFT) as c_int;
        if (*classh).classh_wk_fet_disable == 0x00 && (errata_chk == 0x01 || errata_chk == 0x02) {
            ret = regmap_update_bits((*cs35l35).regmap, CS35L35_CLASS_H_FET_DRIVE_CTL, CS35L35_CH_WKFET_DEL_MASK, 0 << CS35L35_CH_WKFET_DEL_SHIFT);
            if ret != 0 {
                dev_err((*component).dev, c"Failed to set fet config %d\n".as_ptr(), ret);
                return ret;
            }
        }
        /*
         * You can pull more Monitor data from the SDOUT pin than going to SDIN
         * Just make sure your SCLK is fast enough to fill the frame
         */
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            audin_format = match params_width(params) {
                8 => CS35L35_SDIN_DEPTH_8 as c_int,
                16 => CS35L35_SDIN_DEPTH_16 as c_int,
                24 => CS35L35_SDIN_DEPTH_24 as c_int,
                _ => {
                    dev_err((*component).dev, c"Unsupported Width %d\n".as_ptr(), params_width(params));
                    return -EINVAL;
                }
            };
            regmap_update_bits((*cs35l35).regmap, CS35L35_AUDIN_DEPTH_CTL, CS35L35_AUDIN_DEPTH_MASK, (audin_format as c_uint) << CS35L35_AUDIN_DEPTH_SHIFT);
            if (*cs35l35).pdata.stereo {
                regmap_update_bits((*cs35l35).regmap, CS35L35_AUDIN_DEPTH_CTL, CS35L35_ADVIN_DEPTH_MASK, (audin_format as c_uint) << CS35L35_ADVIN_DEPTH_SHIFT);
            }
        }
        if (*cs35l35).i2s_mode {
            /* We have to take the SCLK to derive num sclks
             * to configure the CLOCK_CTL3 register correctly
             */
            if (((*cs35l35).sclk / srate as c_uint) % 4) != 0 {
                dev_err((*component).dev, c"Unsupported sclk/fs ratio %d:%d\n".as_ptr(), (*cs35l35).sclk, srate);
                return -EINVAL;
            }
            sp_sclks = ((((*cs35l35).sclk / srate as c_uint) / 4) - 1) as u8;
            if (*cs35l35).clock_consumer {
                match sp_sclks as c_uint {
                    CS35L35_SP_SCLKS_32FS | CS35L35_SP_SCLKS_48FS | CS35L35_SP_SCLKS_64FS => {}
                    _ => {
                        dev_err((*component).dev, c"ratio not supported\n".as_ptr());
                        return -EINVAL;
                    }
                }
            } else {
                /* Only certain ratios supported when device is a clock provider */
                match sp_sclks as c_uint {
                    CS35L35_SP_SCLKS_32FS | CS35L35_SP_SCLKS_64FS => {}
                    _ => {
                        dev_err((*component).dev, c"ratio not supported\n".as_ptr());
                        return -EINVAL;
                    }
                }
            }
            ret = regmap_update_bits((*cs35l35).regmap, CS35L35_CLK_CTL3, CS35L35_SP_SCLKS_MASK, (sp_sclks as c_uint) << CS35L35_SP_SCLKS_SHIFT);
            if ret != 0 {
                dev_err((*component).dev, c"Failed to set fsclk %d\n".as_ptr(), ret);
                return ret;
            }
        }
        ret
    }
}

static cs35l35_src_rates: [c_uint; 6] = [44100, 48000, 88200, 96000, 176400, 192000];
static cs35l35_constraints: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: 6,
    list: cs35l35_src_rates.as_ptr(),
};

unsafe extern "C" fn cs35l35_pcm_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    unsafe {
        let component = (*dai).component;
        let cs35l35 = snd_soc_component_get_drvdata(component) as *mut cs35l35_private;
        if (*substream).runtime.is_null() {
            return 0;
        }
        snd_pcm_hw_constraint_list((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &cs35l35_constraints);
        regmap_update_bits((*cs35l35).regmap, CS35L35_AMP_INP_DRV_CTL, CS35L35_PDM_MODE_MASK, 0 << CS35L35_PDM_MODE_SHIFT);
        0
    }
}

static cs35l35_pdm_rates: [c_uint; 4] = [44100, 48000, 88200, 96000];
static cs35l35_pdm_constraints: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: 4,
    list: cs35l35_pdm_rates.as_ptr(),
};

unsafe extern "C" fn cs35l35_pdm_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    unsafe {
        let component = (*dai).component;
        let cs35l35 = snd_soc_component_get_drvdata(component) as *mut cs35l35_private;
        if (*substream).runtime.is_null() {
            return 0;
        }
        snd_pcm_hw_constraint_list((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &cs35l35_pdm_constraints);
        regmap_update_bits((*cs35l35).regmap, CS35L35_AMP_INP_DRV_CTL, CS35L35_PDM_MODE_MASK, 1 << CS35L35_PDM_MODE_SHIFT);
        0
    }
}

unsafe extern "C" fn cs35l35_dai_set_sysclk(dai: *mut snd_soc_dai, _clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    unsafe {
        let component = (*dai).component;
        let cs35l35 = snd_soc_component_get_drvdata(component) as *mut cs35l35_private;
        /* Need the SCLK Frequency regardless of sysclk source for I2S */
        (*cs35l35).sclk = freq;
        0
    }
}

static cs35l35_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(cs35l35_pcm_startup),
    set_fmt: Some(cs35l35_set_dai_fmt),
    hw_params: Some(cs35l35_hw_params),
    set_sysclk: Some(cs35l35_dai_set_sysclk),
};

static cs35l35_pdm_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(cs35l35_pdm_startup),
    set_fmt: Some(cs35l35_set_dai_fmt),
    hw_params: Some(cs35l35_hw_params),
    set_sysclk: None,
};

static mut cs35l35_dai: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c"cs35l35-pcm".as_ptr(),
        id: 0,
        playback: snd_soc_pcm_stream { stream_name: c"AMP Playback".as_ptr(), channels_min: 1, channels_max: 8, rates: SNDRV_PCM_RATE_KNOT, formats: CS35L35_FORMATS },
        capture: snd_soc_pcm_stream { stream_name: c"AMP Capture".as_ptr(), channels_min: 1, channels_max: 8, rates: SNDRV_PCM_RATE_KNOT, formats: CS35L35_FORMATS },
        ops: &cs35l35_ops,
        symmetric_rate: 1,
    },
    snd_soc_dai_driver {
        name: c"cs35l35-pdm".as_ptr(),
        id: 1,
        playback: snd_soc_pcm_stream { stream_name: c"PDM Playback".as_ptr(), channels_min: 1, channels_max: 2, rates: SNDRV_PCM_RATE_KNOT, formats: CS35L35_FORMATS },
        capture: snd_soc_pcm_stream { stream_name: ptr::null(), channels_min: 0, channels_max: 0, rates: 0, formats: 0 },
        ops: &cs35l35_pdm_ops,
        symmetric_rate: 0,
    },
];

unsafe extern "C" fn cs35l35_component_set_sysclk(component: *mut snd_soc_component, clk_id: c_int, _source: c_int, freq: c_uint, _dir: c_int) -> c_int {
    unsafe {
        let cs35l35 = snd_soc_component_get_drvdata(component) as *mut cs35l35_private;
        let clksrc: c_int = match clk_id {
            0 => CS35L35_CLK_SOURCE_MCLK as c_int,
            1 => CS35L35_CLK_SOURCE_SCLK as c_int,
            2 => CS35L35_CLK_SOURCE_PDM as c_int,
            _ => {
                dev_err((*component).dev, c"Invalid CLK Source\n".as_ptr());
                return -EINVAL;
            }
        };
        match freq {
            5644800 | 6144000 | 11289600 | 12000000 | 12288000 | 13000000 | 22579200 | 24000000 | 24576000 | 26000000 => (*cs35l35).sysclk = freq,
            _ => {
                dev_err((*component).dev, c"Invalid CLK Frequency Input : %d\n".as_ptr(), freq);
                return -EINVAL;
            }
        }
        let ret = regmap_update_bits((*cs35l35).regmap, CS35L35_CLK_CTL1, CS35L35_CLK_SOURCE_MASK, (clksrc as c_uint) << CS35L35_CLK_SOURCE_SHIFT);
        if ret != 0 {
            dev_err((*component).dev, c"Failed to set sysclk %d\n".as_ptr(), ret);
            return ret;
        }
        ret
    }
}

unsafe fn cs35l35_boost_inductor(cs35l35: *mut cs35l35_private, inductor: c_int) -> c_int {
    unsafe {
        let regmap_p = (*cs35l35).regmap;
        let mut bst_ipk: c_uint = 0;
        /*
         * Digital Boost Converter Configuration for feedback,
         * ramping, switching frequency, and estimation block seeding.
         */
        regmap_update_bits(regmap_p, CS35L35_BST_CONV_SW_FREQ, CS35L35_BST_CONV_SWFREQ_MASK, 0x00);
        regmap_read(regmap_p, CS35L35_BST_PEAK_I, &mut bst_ipk);
        bst_ipk &= CS35L35_BST_IPK_MASK;
        match inductor {
            1000 => {
                /* 1 uH */
                regmap_write(regmap_p, CS35L35_BST_CONV_COEF_1, 0x24);
                regmap_write(regmap_p, CS35L35_BST_CONV_COEF_2, 0x24);
                regmap_update_bits(regmap_p, CS35L35_BST_CONV_SW_FREQ, CS35L35_BST_CONV_LBST_MASK, 0x00);
                if bst_ipk < 0x04 { regmap_write(regmap_p, CS35L35_BST_CONV_SLOPE_COMP, 0x1B); } else { regmap_write(regmap_p, CS35L35_BST_CONV_SLOPE_COMP, 0x4E); }
            }
            1200 => {
                /* 1.2 uH */
                regmap_write(regmap_p, CS35L35_BST_CONV_COEF_1, 0x20);
                regmap_write(regmap_p, CS35L35_BST_CONV_COEF_2, 0x20);
                regmap_update_bits(regmap_p, CS35L35_BST_CONV_SW_FREQ, CS35L35_BST_CONV_LBST_MASK, 0x01);
                if bst_ipk < 0x04 { regmap_write(regmap_p, CS35L35_BST_CONV_SLOPE_COMP, 0x1B); } else { regmap_write(regmap_p, CS35L35_BST_CONV_SLOPE_COMP, 0x47); }
            }
            1500 => {
                /* 1.5uH */
                regmap_write(regmap_p, CS35L35_BST_CONV_COEF_1, 0x20);
                regmap_write(regmap_p, CS35L35_BST_CONV_COEF_2, 0x20);
                regmap_update_bits(regmap_p, CS35L35_BST_CONV_SW_FREQ, CS35L35_BST_CONV_LBST_MASK, 0x02);
                if bst_ipk < 0x04 { regmap_write(regmap_p, CS35L35_BST_CONV_SLOPE_COMP, 0x1B); } else { regmap_write(regmap_p, CS35L35_BST_CONV_SLOPE_COMP, 0x3C); }
            }
            2200 => {
                /* 2.2uH */
                regmap_write(regmap_p, CS35L35_BST_CONV_COEF_1, 0x19);
                regmap_write(regmap_p, CS35L35_BST_CONV_COEF_2, 0x25);
                regmap_update_bits(regmap_p, CS35L35_BST_CONV_SW_FREQ, CS35L35_BST_CONV_LBST_MASK, 0x03);
                if bst_ipk < 0x04 { regmap_write(regmap_p, CS35L35_BST_CONV_SLOPE_COMP, 0x1B); } else { regmap_write(regmap_p, CS35L35_BST_CONV_SLOPE_COMP, 0x23); }
            }
            _ => {
                dev_err((*cs35l35).dev, c"Invalid Inductor Value %d uH\n".as_ptr(), inductor);
                return -EINVAL;
            }
        }
        0
    }
}

unsafe extern "C" fn cs35l35_component_probe(component: *mut snd_soc_component) -> c_int {
    unsafe {
        let cs35l35 = snd_soc_component_get_drvdata(component) as *mut cs35l35_private;
        let classh = &mut (*cs35l35).pdata.classh_algo as *mut classh_cfg;
        let monitor_config = &mut (*cs35l35).pdata.mon_cfg as *mut monitor_cfg;
        let mut ret: c_int;
        /* Set Platform Data */
        if (*cs35l35).pdata.bst_vctl != 0 { regmap_update_bits((*cs35l35).regmap, CS35L35_BST_CVTR_V_CTL, CS35L35_BST_CTL_MASK, (*cs35l35).pdata.bst_vctl); }
        if (*cs35l35).pdata.bst_ipk != 0 { regmap_update_bits((*cs35l35).regmap, CS35L35_BST_PEAK_I, CS35L35_BST_IPK_MASK, (*cs35l35).pdata.bst_ipk << CS35L35_BST_IPK_SHIFT); }
        ret = cs35l35_boost_inductor(cs35l35, (*cs35l35).pdata.boost_ind as c_int);
        if ret != 0 { return ret; }
        if (*cs35l35).pdata.gain_zc { regmap_update_bits((*cs35l35).regmap, CS35L35_PROTECT_CTL, CS35L35_AMP_GAIN_ZC_MASK, ((*cs35l35).pdata.gain_zc as c_uint) << CS35L35_AMP_GAIN_ZC_SHIFT); }
        if (*cs35l35).pdata.aud_channel != 0 { regmap_update_bits((*cs35l35).regmap, CS35L35_AUDIN_RXLOC_CTL, CS35L35_AUD_IN_LR_MASK, (*cs35l35).pdata.aud_channel << CS35L35_AUD_IN_LR_SHIFT); }
        if (*cs35l35).pdata.stereo {
            regmap_update_bits((*cs35l35).regmap, CS35L35_ADVIN_RXLOC_CTL, CS35L35_ADV_IN_LR_MASK, (*cs35l35).pdata.adv_channel << CS35L35_ADV_IN_LR_SHIFT);
            if (*cs35l35).pdata.shared_bst { regmap_update_bits((*cs35l35).regmap, CS35L35_CLASS_H_CTL, CS35L35_CH_STEREO_MASK, 1 << CS35L35_CH_STEREO_SHIFT); }
            ret = snd_soc_add_component_controls(component, cs35l35_adv_controls.as_ptr(), ARRAY_SIZE(&cs35l35_adv_controls));
            if ret != 0 { return ret; }
        }
        if (*cs35l35).pdata.sp_drv_str != 0 { regmap_update_bits((*cs35l35).regmap, CS35L35_CLK_CTL1, CS35L35_SP_DRV_MASK, (*cs35l35).pdata.sp_drv_str << CS35L35_SP_DRV_SHIFT); }
        if (*cs35l35).pdata.sp_drv_unused != 0 { regmap_update_bits((*cs35l35).regmap, CS35L35_SP_FMT_CTL3, CS35L35_SP_I2S_DRV_MASK, (*cs35l35).pdata.sp_drv_unused << CS35L35_SP_I2S_DRV_SHIFT); }
        if (*classh).classh_algo_enable {
            if (*classh).classh_bst_override != 0 { regmap_update_bits((*cs35l35).regmap, CS35L35_CLASS_H_CTL, CS35L35_CH_BST_OVR_MASK, (*classh).classh_bst_override << CS35L35_CH_BST_OVR_SHIFT); }
            if (*classh).classh_bst_max_limit != 0 { regmap_update_bits((*cs35l35).regmap, CS35L35_CLASS_H_CTL, CS35L35_CH_BST_LIM_MASK, (*classh).classh_bst_max_limit << CS35L35_CH_BST_LIM_SHIFT); }
            if (*classh).classh_mem_depth != 0 { regmap_update_bits((*cs35l35).regmap, CS35L35_CLASS_H_CTL, CS35L35_CH_MEM_DEPTH_MASK, (*classh).classh_mem_depth << CS35L35_CH_MEM_DEPTH_SHIFT); }
            if (*classh).classh_headroom != 0 { regmap_update_bits((*cs35l35).regmap, CS35L35_CLASS_H_HEADRM_CTL, CS35L35_CH_HDRM_CTL_MASK, (*classh).classh_headroom << CS35L35_CH_HDRM_CTL_SHIFT); }
            if (*classh).classh_release_rate != 0 { regmap_update_bits((*cs35l35).regmap, CS35L35_CLASS_H_RELEASE_RATE, CS35L35_CH_REL_RATE_MASK, (*classh).classh_release_rate << CS35L35_CH_REL_RATE_SHIFT); }
            if (*classh).classh_wk_fet_disable != 0 { regmap_update_bits((*cs35l35).regmap, CS35L35_CLASS_H_FET_DRIVE_CTL, CS35L35_CH_WKFET_DIS_MASK, (*classh).classh_wk_fet_disable << CS35L35_CH_WKFET_DIS_SHIFT); }
            if (*classh).classh_wk_fet_delay != 0 { regmap_update_bits((*cs35l35).regmap, CS35L35_CLASS_H_FET_DRIVE_CTL, CS35L35_CH_WKFET_DEL_MASK, (*classh).classh_wk_fet_delay << CS35L35_CH_WKFET_DEL_SHIFT); }
            if (*classh).classh_wk_fet_thld != 0 { regmap_update_bits((*cs35l35).regmap, CS35L35_CLASS_H_FET_DRIVE_CTL, CS35L35_CH_WKFET_THLD_MASK, (*classh).classh_wk_fet_thld << CS35L35_CH_WKFET_THLD_SHIFT); }
            if (*classh).classh_vpch_auto != 0 { regmap_update_bits((*cs35l35).regmap, CS35L35_CLASS_H_VP_CTL, CS35L35_CH_VP_AUTO_MASK, (*classh).classh_vpch_auto << CS35L35_CH_VP_AUTO_SHIFT); }
            if (*classh).classh_vpch_rate != 0 { regmap_update_bits((*cs35l35).regmap, CS35L35_CLASS_H_VP_CTL, CS35L35_CH_VP_RATE_MASK, (*classh).classh_vpch_rate << CS35L35_CH_VP_RATE_SHIFT); }
            if (*classh).classh_vpch_man != 0 { regmap_update_bits((*cs35l35).regmap, CS35L35_CLASS_H_VP_CTL, CS35L35_CH_VP_MAN_MASK, (*classh).classh_vpch_man << CS35L35_CH_VP_MAN_SHIFT); }
        }
        if (*monitor_config).is_present {
            if (*monitor_config).vmon_specs {
                regmap_update_bits((*cs35l35).regmap, CS35L35_SPKMON_DEPTH_CTL, CS35L35_VMON_DEPTH_MASK, ((*monitor_config).vmon_dpth as c_uint) << CS35L35_VMON_DEPTH_SHIFT);
                regmap_update_bits((*cs35l35).regmap, CS35L35_VMON_TXLOC_CTL, CS35L35_MON_TXLOC_MASK, ((*monitor_config).vmon_loc as c_uint) << CS35L35_MON_TXLOC_SHIFT);
                regmap_update_bits((*cs35l35).regmap, CS35L35_VMON_TXLOC_CTL, CS35L35_MON_FRM_MASK, ((*monitor_config).vmon_frm as c_uint) << CS35L35_MON_FRM_SHIFT);
            }
            if (*monitor_config).imon_specs {
                regmap_update_bits((*cs35l35).regmap, CS35L35_SPKMON_DEPTH_CTL, CS35L35_IMON_DEPTH_MASK, ((*monitor_config).imon_dpth as c_uint) << CS35L35_IMON_DEPTH_SHIFT);
                regmap_update_bits((*cs35l35).regmap, CS35L35_IMON_TXLOC_CTL, CS35L35_MON_TXLOC_MASK, ((*monitor_config).imon_loc as c_uint) << CS35L35_MON_TXLOC_SHIFT);
                regmap_update_bits((*cs35l35).regmap, CS35L35_IMON_TXLOC_CTL, CS35L35_MON_FRM_MASK, ((*monitor_config).imon_frm as c_uint) << CS35L35_MON_FRM_SHIFT);
                regmap_update_bits((*cs35l35).regmap, CS35L35_IMON_SCALE_CTL, CS35L35_IMON_SCALE_MASK, ((*monitor_config).imon_scale as c_uint) << CS35L35_IMON_SCALE_SHIFT);
            }
            if (*monitor_config).vpmon_specs {
                regmap_update_bits((*cs35l35).regmap, CS35L35_SUPMON_DEPTH_CTL, CS35L35_VPMON_DEPTH_MASK, ((*monitor_config).vpmon_dpth as c_uint) << CS35L35_VPMON_DEPTH_SHIFT);
                regmap_update_bits((*cs35l35).regmap, CS35L35_VPMON_TXLOC_CTL, CS35L35_MON_TXLOC_MASK, ((*monitor_config).vpmon_loc as c_uint) << CS35L35_MON_TXLOC_SHIFT);
                regmap_update_bits((*cs35l35).regmap, CS35L35_VPMON_TXLOC_CTL, CS35L35_MON_FRM_MASK, ((*monitor_config).vpmon_frm as c_uint) << CS35L35_MON_FRM_SHIFT);
            }
            if (*monitor_config).vbstmon_specs {
                regmap_update_bits((*cs35l35).regmap, CS35L35_SUPMON_DEPTH_CTL, CS35L35_VBSTMON_DEPTH_MASK, ((*monitor_config).vpmon_dpth as c_uint) << CS35L35_VBSTMON_DEPTH_SHIFT);
                regmap_update_bits((*cs35l35).regmap, CS35L35_VBSTMON_TXLOC_CTL, CS35L35_MON_TXLOC_MASK, ((*monitor_config).vbstmon_loc as c_uint) << CS35L35_MON_TXLOC_SHIFT);
                regmap_update_bits((*cs35l35).regmap, CS35L35_VBSTMON_TXLOC_CTL, CS35L35_MON_FRM_MASK, ((*monitor_config).vbstmon_frm as c_uint) << CS35L35_MON_FRM_SHIFT);
            }
            if (*monitor_config).vpbrstat_specs {
                regmap_update_bits((*cs35l35).regmap, CS35L35_SUPMON_DEPTH_CTL, CS35L35_VPBRSTAT_DEPTH_MASK, ((*monitor_config).vpbrstat_dpth as c_uint) << CS35L35_VPBRSTAT_DEPTH_SHIFT);
                regmap_update_bits((*cs35l35).regmap, CS35L35_VPBR_STATUS_TXLOC_CTL, CS35L35_MON_TXLOC_MASK, ((*monitor_config).vpbrstat_loc as c_uint) << CS35L35_MON_TXLOC_SHIFT);
                regmap_update_bits((*cs35l35).regmap, CS35L35_VPBR_STATUS_TXLOC_CTL, CS35L35_MON_FRM_MASK, ((*monitor_config).vpbrstat_frm as c_uint) << CS35L35_MON_FRM_SHIFT);
            }
            if (*monitor_config).zerofill_specs {
                regmap_update_bits((*cs35l35).regmap, CS35L35_SUPMON_DEPTH_CTL, CS35L35_ZEROFILL_DEPTH_MASK, ((*monitor_config).zerofill_dpth as c_uint) << CS35L35_ZEROFILL_DEPTH_SHIFT);
                regmap_update_bits((*cs35l35).regmap, CS35L35_ZERO_FILL_LOC_CTL, CS35L35_MON_TXLOC_MASK, ((*monitor_config).zerofill_loc as c_uint) << CS35L35_MON_TXLOC_SHIFT);
                regmap_update_bits((*cs35l35).regmap, CS35L35_ZERO_FILL_LOC_CTL, CS35L35_MON_FRM_MASK, ((*monitor_config).zerofill_frm as c_uint) << CS35L35_MON_FRM_SHIFT);
            }
        }
        0
    }
}

static soc_component_dev_cs35l35: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(cs35l35_component_probe),
    set_sysclk: Some(cs35l35_component_set_sysclk),
    dapm_widgets: cs35l35_dapm_widgets.as_ptr(),
    num_dapm_widgets: 0,
    dapm_routes: cs35l35_audio_map.as_ptr(),
    num_dapm_routes: 13,
    controls: cs35l35_aud_controls.as_ptr(),
    num_controls: 0,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static cs35l35_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: CS35L35_MAX_REGISTER,
    reg_defaults: cs35l35_reg.as_ptr(),
    num_reg_defaults: 61,
    volatile_reg: Some(cs35l35_volatile_register),
    readable_reg: Some(cs35l35_readable_register),
    precious_reg: Some(cs35l35_precious_register),
    cache_type: REGCACHE_MAPLE,
    use_single_read: true,
    use_single_write: true,
};

unsafe extern "C" fn cs35l35_irq(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    unsafe {
        let cs35l35 = data as *mut cs35l35_private;
        let (mut sticky1, mut sticky2, mut sticky3, mut sticky4) = (0, 0, 0, 0);
        let (mut mask1, mut mask2, mut mask3, mut mask4, mut current1) = (0, 0, 0, 0, 0);
        /* ack the irq by reading all status registers */
        regmap_read((*cs35l35).regmap, CS35L35_INT_STATUS_4, &mut sticky4);
        regmap_read((*cs35l35).regmap, CS35L35_INT_STATUS_3, &mut sticky3);
        regmap_read((*cs35l35).regmap, CS35L35_INT_STATUS_2, &mut sticky2);
        regmap_read((*cs35l35).regmap, CS35L35_INT_STATUS_1, &mut sticky1);
        regmap_read((*cs35l35).regmap, CS35L35_INT_MASK_4, &mut mask4);
        regmap_read((*cs35l35).regmap, CS35L35_INT_MASK_3, &mut mask3);
        regmap_read((*cs35l35).regmap, CS35L35_INT_MASK_2, &mut mask2);
        regmap_read((*cs35l35).regmap, CS35L35_INT_MASK_1, &mut mask1);
        /* Check to see if unmasked bits are active */
        if (sticky1 & !mask1) == 0 && (sticky2 & !mask2) == 0 && (sticky3 & !mask3) == 0 && (sticky4 & !mask4) == 0 {
            return IRQ_NONE;
        }
        if (sticky2 & CS35L35_PDN_DONE) != 0 {
            complete(&mut (*cs35l35).pdn_done);
        }
        /* read the current values */
        regmap_read((*cs35l35).regmap, CS35L35_INT_STATUS_1, &mut current1);
        /* handle the interrupts */
        if (sticky1 & CS35L35_CAL_ERR) != 0 {
            dev_crit((*cs35l35).dev, c"Calibration Error\n".as_ptr());
            /* error is no longer asserted; safe to reset */
            if (current1 & CS35L35_CAL_ERR) == 0 {
                pr_debug(c"%s : Cal error release\n".as_ptr(), c"cs35l35_irq".as_ptr());
                regmap_update_bits((*cs35l35).regmap, CS35L35_PROT_RELEASE_CTL, CS35L35_CAL_ERR_RLS, 0);
                regmap_update_bits((*cs35l35).regmap, CS35L35_PROT_RELEASE_CTL, CS35L35_CAL_ERR_RLS, CS35L35_CAL_ERR_RLS);
                regmap_update_bits((*cs35l35).regmap, CS35L35_PROT_RELEASE_CTL, CS35L35_CAL_ERR_RLS, 0);
            }
        }
        if (sticky1 & CS35L35_AMP_SHORT) != 0 {
            dev_crit((*cs35l35).dev, c"AMP Short Error\n".as_ptr());
            if (current1 & CS35L35_AMP_SHORT) == 0 {
                dev_dbg((*cs35l35).dev, c"Amp short error release\n".as_ptr());
                regmap_update_bits((*cs35l35).regmap, CS35L35_PROT_RELEASE_CTL, CS35L35_SHORT_RLS, 0);
                regmap_update_bits((*cs35l35).regmap, CS35L35_PROT_RELEASE_CTL, CS35L35_SHORT_RLS, CS35L35_SHORT_RLS);
                regmap_update_bits((*cs35l35).regmap, CS35L35_PROT_RELEASE_CTL, CS35L35_SHORT_RLS, 0);
            }
        }
        if (sticky1 & CS35L35_OTW) != 0 {
            dev_warn((*cs35l35).dev, c"Over temperature warning\n".as_ptr());
            if (current1 & CS35L35_OTW) == 0 {
                dev_dbg((*cs35l35).dev, c"Over temperature warn release\n".as_ptr());
                regmap_update_bits((*cs35l35).regmap, CS35L35_PROT_RELEASE_CTL, CS35L35_OTW_RLS, 0);
                regmap_update_bits((*cs35l35).regmap, CS35L35_PROT_RELEASE_CTL, CS35L35_OTW_RLS, CS35L35_OTW_RLS);
                regmap_update_bits((*cs35l35).regmap, CS35L35_PROT_RELEASE_CTL, CS35L35_OTW_RLS, 0);
            }
        }
        if (sticky1 & CS35L35_OTE) != 0 {
            dev_crit((*cs35l35).dev, c"Over temperature error\n".as_ptr());
            if (current1 & CS35L35_OTE) == 0 {
                dev_dbg((*cs35l35).dev, c"Over temperature error release\n".as_ptr());
                regmap_update_bits((*cs35l35).regmap, CS35L35_PROT_RELEASE_CTL, CS35L35_OTE_RLS, 0);
                regmap_update_bits((*cs35l35).regmap, CS35L35_PROT_RELEASE_CTL, CS35L35_OTE_RLS, CS35L35_OTE_RLS);
                regmap_update_bits((*cs35l35).regmap, CS35L35_PROT_RELEASE_CTL, CS35L35_OTE_RLS, 0);
            }
        }
        if (sticky3 & CS35L35_BST_HIGH) != 0 {
            dev_crit((*cs35l35).dev, c"VBST error: powering off!\n".as_ptr());
            regmap_update_bits((*cs35l35).regmap, CS35L35_PWRCTL2, CS35L35_PDN_AMP, CS35L35_PDN_AMP);
            regmap_update_bits((*cs35l35).regmap, CS35L35_PWRCTL1, CS35L35_PDN_ALL, CS35L35_PDN_ALL);
        }
        if (sticky3 & CS35L35_LBST_SHORT) != 0 {
            dev_crit((*cs35l35).dev, c"LBST error: powering off!\n".as_ptr());
            regmap_update_bits((*cs35l35).regmap, CS35L35_PWRCTL2, CS35L35_PDN_AMP, CS35L35_PDN_AMP);
            regmap_update_bits((*cs35l35).regmap, CS35L35_PWRCTL1, CS35L35_PDN_ALL, CS35L35_PDN_ALL);
        }
        if (sticky2 & CS35L35_VPBR_ERR) != 0 { dev_dbg((*cs35l35).dev, c"Error: Reactive Brownout\n".as_ptr()); }
        if (sticky4 & CS35L35_VMON_OVFL) != 0 { dev_dbg((*cs35l35).dev, c"Error: VMON overflow\n".as_ptr()); }
        if (sticky4 & CS35L35_IMON_OVFL) != 0 { dev_dbg((*cs35l35).dev, c"Error: IMON overflow\n".as_ptr()); }
        IRQ_HANDLED
    }
}

unsafe fn cs35l35_handle_of_data(i2c_client: *mut i2c_client, pdata: *mut cs35l35_platform_data) -> c_int {
    unsafe {
        let np = (*i2c_client).dev.of_node;
        let mut classh: *mut device_node;
        let mut signal_format: *mut device_node;
        let classh_config = &mut (*pdata).classh_algo as *mut classh_cfg;
        let monitor_config = &mut (*pdata).mon_cfg as *mut monitor_cfg;
        let mut val32: c_uint = 0;
        let mut monitor_array: [u8; 4] = [0; 4];
        let imon_array_size: c_int = 4;
        let mon_array_size: c_int = imon_array_size - 1;
        let mut ret: c_int = 0;
        if np.is_null() { return 0; }
        (*pdata).bst_pdn_fet_on = of_property_read_bool(np, c"cirrus,boost-pdn-fet-on".as_ptr());
        ret = of_property_read_u32(np, c"cirrus,boost-ctl-millivolt".as_ptr(), &mut val32);
        if ret >= 0 {
            if val32 < 2600 || val32 > 9000 {
                dev_err(&mut (*i2c_client).dev as *mut device_with_of as *mut device, c"Invalid Boost Voltage %d mV\n".as_ptr(), val32);
                return -EINVAL;
            }
            (*pdata).bst_vctl = ((val32 - 2600) / 100) + 1;
        }
        ret = of_property_read_u32(np, c"cirrus,boost-peak-milliamp".as_ptr(), &mut val32);
        if ret >= 0 {
            if val32 < 1680 || val32 > 4480 {
                dev_err(&mut (*i2c_client).dev as *mut device_with_of as *mut device, c"Invalid Boost Peak Current %u mA\n".as_ptr(), val32);
                return -EINVAL;
            }
            (*pdata).bst_ipk = ((val32 - 1680) / 110) | CS35L35_VALID_PDATA;
        }
        ret = of_property_read_u32(np, c"cirrus,boost-ind-nanohenry".as_ptr(), &mut val32);
        if ret >= 0 { (*pdata).boost_ind = val32; } else {
            dev_err(&mut (*i2c_client).dev as *mut device_with_of as *mut device, c"Inductor not specified.\n".as_ptr());
            return -EINVAL;
        }
        if of_property_read_u32(np, c"cirrus,sp-drv-strength".as_ptr(), &mut val32) >= 0 { (*pdata).sp_drv_str = val32; }
        if of_property_read_u32(np, c"cirrus,sp-drv-unused".as_ptr(), &mut val32) >= 0 { (*pdata).sp_drv_unused = val32 | CS35L35_VALID_PDATA; }
        (*pdata).stereo = of_property_read_bool(np, c"cirrus,stereo-config".as_ptr());
        if (*pdata).stereo {
            ret = of_property_read_u32(np, c"cirrus,audio-channel".as_ptr(), &mut val32);
            if ret >= 0 { (*pdata).aud_channel = val32; }
            ret = of_property_read_u32(np, c"cirrus,advisory-channel".as_ptr(), &mut val32);
            if ret >= 0 { (*pdata).adv_channel = val32; }
            (*pdata).shared_bst = of_property_read_bool(np, c"cirrus,shared-boost".as_ptr());
        }
        (*pdata).ext_bst = of_property_read_bool(np, c"cirrus,external-boost".as_ptr());
        (*pdata).gain_zc = of_property_read_bool(np, c"cirrus,amp-gain-zc".as_ptr());
        classh = of_get_child_by_name(np, c"cirrus,classh-internal-algo".as_ptr());
        (*classh_config).classh_algo_enable = !classh.is_null();
        if (*classh_config).classh_algo_enable {
            (*classh_config).classh_bst_override = of_property_read_bool(np, c"cirrus,classh-bst-overide".as_ptr()) as c_uint;
            ret = of_property_read_u32(classh, c"cirrus,classh-bst-max-limit".as_ptr(), &mut val32);
            if ret >= 0 { val32 |= CS35L35_VALID_PDATA; (*classh_config).classh_bst_max_limit = val32; }
            ret = of_property_read_u32(classh, c"cirrus,classh-bst-max-limit".as_ptr(), &mut val32);
            if ret >= 0 { val32 |= CS35L35_VALID_PDATA; (*classh_config).classh_bst_max_limit = val32; }
            ret = of_property_read_u32(classh, c"cirrus,classh-mem-depth".as_ptr(), &mut val32);
            if ret >= 0 { val32 |= CS35L35_VALID_PDATA; (*classh_config).classh_mem_depth = val32; }
            ret = of_property_read_u32(classh, c"cirrus,classh-release-rate".as_ptr(), &mut val32);
            if ret >= 0 { (*classh_config).classh_release_rate = val32; }
            ret = of_property_read_u32(classh, c"cirrus,classh-headroom".as_ptr(), &mut val32);
            if ret >= 0 { val32 |= CS35L35_VALID_PDATA; (*classh_config).classh_headroom = val32; }
            ret = of_property_read_u32(classh, c"cirrus,classh-wk-fet-disable".as_ptr(), &mut val32);
            if ret >= 0 { (*classh_config).classh_wk_fet_disable = val32; }
            ret = of_property_read_u32(classh, c"cirrus,classh-wk-fet-delay".as_ptr(), &mut val32);
            if ret >= 0 { val32 |= CS35L35_VALID_PDATA; (*classh_config).classh_wk_fet_delay = val32; }
            ret = of_property_read_u32(classh, c"cirrus,classh-wk-fet-thld".as_ptr(), &mut val32);
            if ret >= 0 { (*classh_config).classh_wk_fet_thld = val32; }
            ret = of_property_read_u32(classh, c"cirrus,classh-vpch-auto".as_ptr(), &mut val32);
            if ret >= 0 { val32 |= CS35L35_VALID_PDATA; (*classh_config).classh_vpch_auto = val32; }
            ret = of_property_read_u32(classh, c"cirrus,classh-vpch-rate".as_ptr(), &mut val32);
            if ret >= 0 { val32 |= CS35L35_VALID_PDATA; (*classh_config).classh_vpch_rate = val32; }
            ret = of_property_read_u32(classh, c"cirrus,classh-vpch-man".as_ptr(), &mut val32);
            if ret >= 0 { (*classh_config).classh_vpch_man = val32; }
        }
        of_node_put(classh);
        /* frame depth location */
        signal_format = of_get_child_by_name(np, c"cirrus,monitor-signal-format".as_ptr());
        (*monitor_config).is_present = !signal_format.is_null();
        if (*monitor_config).is_present {
            ret = of_property_read_u8_array(signal_format, c"cirrus,imon".as_ptr(), monitor_array.as_mut_ptr(), imon_array_size);
            if ret == 0 { (*monitor_config).imon_specs = true; (*monitor_config).imon_dpth = monitor_array[0]; (*monitor_config).imon_loc = monitor_array[1]; (*monitor_config).imon_frm = monitor_array[2]; (*monitor_config).imon_scale = monitor_array[3]; }
            ret = of_property_read_u8_array(signal_format, c"cirrus,vmon".as_ptr(), monitor_array.as_mut_ptr(), mon_array_size);
            if ret == 0 { (*monitor_config).vmon_specs = true; (*monitor_config).vmon_dpth = monitor_array[0]; (*monitor_config).vmon_loc = monitor_array[1]; (*monitor_config).vmon_frm = monitor_array[2]; }
            ret = of_property_read_u8_array(signal_format, c"cirrus,vpmon".as_ptr(), monitor_array.as_mut_ptr(), mon_array_size);
            if ret == 0 { (*monitor_config).vpmon_specs = true; (*monitor_config).vpmon_dpth = monitor_array[0]; (*monitor_config).vpmon_loc = monitor_array[1]; (*monitor_config).vpmon_frm = monitor_array[2]; }
            ret = of_property_read_u8_array(signal_format, c"cirrus,vbstmon".as_ptr(), monitor_array.as_mut_ptr(), mon_array_size);
            if ret == 0 { (*monitor_config).vbstmon_specs = true; (*monitor_config).vbstmon_dpth = monitor_array[0]; (*monitor_config).vbstmon_loc = monitor_array[1]; (*monitor_config).vbstmon_frm = monitor_array[2]; }
            ret = of_property_read_u8_array(signal_format, c"cirrus,vpbrstat".as_ptr(), monitor_array.as_mut_ptr(), mon_array_size);
            if ret == 0 { (*monitor_config).vpbrstat_specs = true; (*monitor_config).vpbrstat_dpth = monitor_array[0]; (*monitor_config).vpbrstat_loc = monitor_array[1]; (*monitor_config).vpbrstat_frm = monitor_array[2]; }
            ret = of_property_read_u8_array(signal_format, c"cirrus,zerofill".as_ptr(), monitor_array.as_mut_ptr(), mon_array_size);
            if ret == 0 { (*monitor_config).zerofill_specs = true; (*monitor_config).zerofill_dpth = monitor_array[0]; (*monitor_config).zerofill_loc = monitor_array[1]; (*monitor_config).zerofill_frm = monitor_array[2]; }
        }
        of_node_put(signal_format);
        0
    }
}

/* Errata Rev A0 */
static cs35l35_errata_patch: [reg_sequence; 11] = [
    reg_sequence { reg: 0x7F, def: 0x99 },
    reg_sequence { reg: 0x00, def: 0x99 },
    reg_sequence { reg: 0x52, def: 0x22 },
    reg_sequence { reg: 0x04, def: 0x14 },
    reg_sequence { reg: 0x6D, def: 0x44 },
    reg_sequence { reg: 0x24, def: 0x10 },
    reg_sequence { reg: 0x58, def: 0xC4 },
    reg_sequence { reg: 0x00, def: 0x98 },
    reg_sequence { reg: 0x18, def: 0x08 },
    reg_sequence { reg: 0x00, def: 0x00 },
    reg_sequence { reg: 0x7F, def: 0x00 },
];

unsafe extern "C" fn cs35l35_i2c_probe(i2c_client: *mut i2c_client) -> c_int {
    unsafe {
        let mut cs35l35: *mut cs35l35_private;
        let dev = &mut (*i2c_client).dev as *mut device_with_of as *mut device;
        let mut pdata = dev_get_platdata(dev) as *mut cs35l35_platform_data;
        let mut devid: c_int;
        let mut ret: c_int;
        let mut reg: c_uint = 0;
        cs35l35 = devm_kzalloc(dev, core::mem::size_of::<cs35l35_private>(), GFP_KERNEL) as *mut cs35l35_private;
        if cs35l35.is_null() { return -ENOMEM; }
        (*cs35l35).dev = dev;
        i2c_set_clientdata(i2c_client, cs35l35 as *mut c_void);
        (*cs35l35).regmap = devm_regmap_init_i2c(i2c_client, &cs35l35_regmap);
        if IS_ERR((*cs35l35).regmap as *const c_void) {
            ret = PTR_ERR((*cs35l35).regmap as *const c_void);
            dev_err(dev, c"regmap_init() failed: %d\n".as_ptr(), ret);
            return ret;
        }
        let mut i = 0usize;
        while i < cs35l35_supplies.len() {
            (*cs35l35).supplies[i].supply = cs35l35_supplies[i];
            i += 1;
        }
        (*cs35l35).num_supplies = cs35l35_supplies.len() as c_int;
        ret = devm_regulator_bulk_get(dev, (*cs35l35).num_supplies, (*cs35l35).supplies.as_mut_ptr());
        if ret != 0 {
            dev_err(dev, c"Failed to request core supplies: %d\n".as_ptr(), ret);
            return ret;
        }
        if !pdata.is_null() {
            (*cs35l35).pdata = ptr::read(pdata);
        } else {
            pdata = devm_kzalloc(dev, core::mem::size_of::<cs35l35_platform_data>(), GFP_KERNEL) as *mut cs35l35_platform_data;
            if pdata.is_null() { return -ENOMEM; }
            if !(*i2c_client).dev.of_node.is_null() {
                ret = cs35l35_handle_of_data(i2c_client, pdata);
                if ret != 0 { return ret; }
            }
            (*cs35l35).pdata = ptr::read(pdata);
        }
        ret = regulator_bulk_enable((*cs35l35).num_supplies, (*cs35l35).supplies.as_mut_ptr());
        if ret != 0 {
            dev_err(dev, c"Failed to enable core supplies: %d\n".as_ptr(), ret);
            return ret;
        }
        /* returning NULL can be valid if in stereo mode */
        (*cs35l35).reset_gpio = devm_gpiod_get_optional(dev, c"reset".as_ptr(), GPIOD_OUT_LOW);
        if IS_ERR((*cs35l35).reset_gpio as *const c_void) {
            ret = PTR_ERR((*cs35l35).reset_gpio as *const c_void);
            (*cs35l35).reset_gpio = ptr::null_mut();
            if ret == -EBUSY {
                dev_info(dev, c"Reset line busy, assuming shared reset\n".as_ptr());
            } else {
                dev_err(dev, c"Failed to get reset GPIO: %d\n".as_ptr(), ret);
                regulator_bulk_disable((*cs35l35).num_supplies, (*cs35l35).supplies.as_mut_ptr());
                gpiod_set_value_cansleep((*cs35l35).reset_gpio, 0);
                return ret;
            }
        }
        cs35l35_reset(cs35l35);
        init_completion(&mut (*cs35l35).pdn_done);
        ret = devm_request_threaded_irq(dev, (*i2c_client).irq, ptr::null(), cs35l35_irq, IRQF_ONESHOT | IRQF_TRIGGER_LOW | IRQF_SHARED, c"cs35l35".as_ptr(), cs35l35 as *mut c_void);
        if ret != 0 {
            dev_err(dev, c"Failed to request IRQ: %d\n".as_ptr(), ret);
            regulator_bulk_disable((*cs35l35).num_supplies, (*cs35l35).supplies.as_mut_ptr());
            gpiod_set_value_cansleep((*cs35l35).reset_gpio, 0);
            return ret;
        }
        /* initialize codec */
        devid = cirrus_read_device_id((*cs35l35).regmap, CS35L35_DEVID_AB);
        if devid < 0 {
            ret = devid;
            dev_err(dev, c"Failed to read device ID: %d\n".as_ptr(), ret);
            regulator_bulk_disable((*cs35l35).num_supplies, (*cs35l35).supplies.as_mut_ptr());
            gpiod_set_value_cansleep((*cs35l35).reset_gpio, 0);
            return ret;
        }
        if devid != CS35L35_CHIP_ID as c_int {
            dev_err(dev, c"CS35L35 Device ID (%X). Expected ID %X\n".as_ptr(), devid, CS35L35_CHIP_ID);
            ret = -ENODEV;
            regulator_bulk_disable((*cs35l35).num_supplies, (*cs35l35).supplies.as_mut_ptr());
            gpiod_set_value_cansleep((*cs35l35).reset_gpio, 0);
            return ret;
        }
        ret = regmap_read((*cs35l35).regmap, CS35L35_REV_ID, &mut reg);
        if ret < 0 {
            dev_err(dev, c"Get Revision ID failed: %d\n".as_ptr(), ret);
            regulator_bulk_disable((*cs35l35).num_supplies, (*cs35l35).supplies.as_mut_ptr());
            gpiod_set_value_cansleep((*cs35l35).reset_gpio, 0);
            return ret;
        }
        ret = regmap_register_patch((*cs35l35).regmap, cs35l35_errata_patch.as_ptr(), ARRAY_SIZE(&cs35l35_errata_patch));
        if ret < 0 {
            dev_err(dev, c"Failed to apply errata patch: %d\n".as_ptr(), ret);
            regulator_bulk_disable((*cs35l35).num_supplies, (*cs35l35).supplies.as_mut_ptr());
            gpiod_set_value_cansleep((*cs35l35).reset_gpio, 0);
            return ret;
        }
        dev_info(dev, c"Cirrus Logic CS35L35 (%x), Revision: %02X\n".as_ptr(), devid, reg & 0xFF);
        /* Set the INT Masks for critical errors */
        regmap_write((*cs35l35).regmap, CS35L35_INT_MASK_1, CS35L35_INT1_CRIT_MASK);
        regmap_write((*cs35l35).regmap, CS35L35_INT_MASK_2, CS35L35_INT2_CRIT_MASK);
        regmap_write((*cs35l35).regmap, CS35L35_INT_MASK_3, CS35L35_INT3_CRIT_MASK);
        regmap_write((*cs35l35).regmap, CS35L35_INT_MASK_4, CS35L35_INT4_CRIT_MASK);
        regmap_update_bits((*cs35l35).regmap, CS35L35_PWRCTL2, CS35L35_PWR2_PDN_MASK, CS35L35_PWR2_PDN_MASK);
        if (*cs35l35).pdata.bst_pdn_fet_on {
            regmap_update_bits((*cs35l35).regmap, CS35L35_PWRCTL2, CS35L35_PDN_BST_MASK, 1 << CS35L35_PDN_BST_FETON_SHIFT);
        } else {
            regmap_update_bits((*cs35l35).regmap, CS35L35_PWRCTL2, CS35L35_PDN_BST_MASK, 1 << CS35L35_PDN_BST_FETOFF_SHIFT);
        }
        regmap_update_bits((*cs35l35).regmap, CS35L35_PWRCTL3, CS35L35_PWR3_PDN_MASK, CS35L35_PWR3_PDN_MASK);
        regmap_update_bits((*cs35l35).regmap, CS35L35_PROTECT_CTL, CS35L35_AMP_MUTE_MASK, 1 << CS35L35_AMP_MUTE_SHIFT);
        ret = devm_snd_soc_register_component(dev, &soc_component_dev_cs35l35, cs35l35_dai.as_mut_ptr(), ARRAY_SIZE(&cs35l35_dai));
        if ret < 0 {
            dev_err(dev, c"Failed to register component: %d\n".as_ptr(), ret);
            regulator_bulk_disable((*cs35l35).num_supplies, (*cs35l35).supplies.as_mut_ptr());
            gpiod_set_value_cansleep((*cs35l35).reset_gpio, 0);
            return ret;
        }
        0
    }
}

unsafe extern "C" fn cs35l35_i2c_remove(i2c_client: *mut i2c_client) {
    unsafe {
        let cs35l35 = i2c_get_clientdata(i2c_client) as *mut cs35l35_private;
        regulator_bulk_disable((*cs35l35).num_supplies, (*cs35l35).supplies.as_mut_ptr());
        gpiod_set_value_cansleep((*cs35l35).reset_gpio, 0);
    }
}

static cs35l35_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c"cirrus,cs35l35".as_ptr() },
    of_device_id { compatible: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, cs35l35_of_match); */

static cs35l35_id: [i2c_device_id; 2] = [
    i2c_device_id { name: c"cs35l35".as_ptr() },
    i2c_device_id { name: ptr::null() },
];
/* MODULE_DEVICE_TABLE(i2c, cs35l35_id); */

static mut cs35l35_i2c_driver: i2c_driver = i2c_driver {
    driver: driver_inner {
        name: c"cs35l35".as_ptr(),
        of_match_table: cs35l35_of_match.as_ptr(),
    },
    id_table: cs35l35_id.as_ptr(),
    probe: Some(cs35l35_i2c_probe),
    remove: Some(cs35l35_i2c_remove),
};

/* module_i2c_driver(cs35l35_i2c_driver); */
/* MODULE_DESCRIPTION("ASoC CS35L35 driver"); */
/* MODULE_AUTHOR("Brian Austin, Cirrus Logic Inc, <brian.austin@cirrus.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
