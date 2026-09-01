// SPDX-License-Identifier: GPL-2.0-only
/*
 * cs35l34.c -- CS35l34 ALSA SoC audio driver
 *
 * Copyright 2016 Cirrus Logic, Inc.
 *
 * Author: Paul Handrigan <Paul.Handrigan@cirrus.com>
 */

// C dependencies translated from:
// linux/module.h, linux/moduleparam.h, linux/kernel.h, linux/init.h,
// linux/delay.h, linux/i2c.h, linux/slab.h, linux/workqueue.h,
// linux/platform_device.h, linux/regulator/consumer.h,
// linux/regulator/machine.h, linux/pm_runtime.h, linux/of.h,
// linux/of_irq.h, sound/core.h, sound/pcm.h, sound/pcm_params.h,
// sound/soc.h, sound/soc-dapm.h, linux/gpio/consumer.h,
// sound/initval.h, sound/tlv.h, sound/cs35l34.h, cs35l34.h,
// and cirrus_legacy.h.

use core::ffi::{c_char, c_int, c_uint, c_void};

const PDN_DONE_ATTEMPTS: c_int = 10;
const CS35L34_START_DELAY: c_uint = 50;

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
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
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
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device_with_of_node,
    pub irq: c_int,
}

#[repr(C)]
pub struct device_with_of_node {
    pub of_node: *mut device_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l34_platform_data {
    pub boost_vtge: c_uint,
    pub boost_peak: c_uint,
    pub boost_ind: c_uint,
    pub gain_zc_disable: bool,
    pub aif_half_drv: bool,
    pub digsft_disable: bool,
    pub amp_inv: bool,
    pub i2s_sdinloc: c_uint,
    pub tdm_rising_edge: c_uint,
}

#[repr(C)]
pub struct cs35l34_private {
    pub component: *mut snd_soc_component,
    pub pdata: cs35l34_platform_data,
    pub regmap: *mut regmap,
    pub core_supplies: [regulator_bulk_data; 2],
    pub num_core_supplies: c_int,
    pub mclk_int: c_int,
    pub tdm_mode: bool,
    pub irq_requested: bool,
    pub reset_gpio: *mut gpio_desc, /* Active-low reset GPIO */
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
    pub set_tristate: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_tdm_slot:
        Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
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
    pub dapm_widgets: *const snd_soc_dapm_widget_desc,
    pub num_dapm_widgets: usize,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: usize,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: usize,
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
    pub num_reg_defaults: usize,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub precious_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_uint,
    pub use_single_read: bool,
    pub use_single_write: bool,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
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
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub id_table: *const i2c_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
}

type irqreturn_t = c_uint;

extern "C" {
    static CS35L34_PWRCTL1: c_uint;
    static CS35L34_PWRCTL2: c_uint;
    static CS35L34_PWRCTL3: c_uint;
    static CS35L34_ADSP_CLK_CTL: c_uint;
    static CS35L34_MCLK_CTL: c_uint;
    static CS35L34_AMP_INP_DRV_CTL: c_uint;
    static CS35L34_AMP_DIG_VOL_CTL: c_uint;
    static CS35L34_AMP_DIG_VOL: c_uint;
    static CS35L34_AMP_ANLG_GAIN_CTL: c_uint;
    static CS35L34_PROTECT_CTL: c_uint;
    static CS35L34_AMP_KEEP_ALIVE_CTL: c_uint;
    static CS35L34_BST_CVTR_V_CTL: c_uint;
    static CS35L34_BST_PEAK_I: c_uint;
    static CS35L34_BST_RAMP_CTL: c_uint;
    static CS35L34_BST_CONV_COEF_1: c_uint;
    static CS35L34_BST_CONV_COEF_2: c_uint;
    static CS35L34_BST_CONV_SLOPE_COMP: c_uint;
    static CS35L34_BST_CONV_SW_FREQ: c_uint;
    static CS35L34_CLASS_H_CTL: c_uint;
    static CS35L34_CLASS_H_HEADRM_CTL: c_uint;
    static CS35L34_CLASS_H_RELEASE_RATE: c_uint;
    static CS35L34_CLASS_H_FET_DRIVE_CTL: c_uint;
    static CS35L34_CLASS_H_STATUS: c_uint;
    static CS35L34_VPBR_CTL: c_uint;
    static CS35L34_VPBR_VOL_CTL: c_uint;
    static CS35L34_VPBR_TIMING_CTL: c_uint;
    static CS35L34_PRED_MAX_ATTEN_SPK_LOAD: c_uint;
    static CS35L34_PRED_BROWNOUT_THRESH: c_uint;
    static CS35L34_PRED_BROWNOUT_VOL_CTL: c_uint;
    static CS35L34_PRED_BROWNOUT_RATE_CTL: c_uint;
    static CS35L34_PRED_WAIT_CTL: c_uint;
    static CS35L34_PRED_ZVP_INIT_IMP_CTL: c_uint;
    static CS35L34_PRED_MAN_SAFE_VPI_CTL: c_uint;
    static CS35L34_VPBR_ATTEN_STATUS: c_uint;
    static CS35L34_PRED_BRWNOUT_ATT_STATUS: c_uint;
    static CS35L34_SPKR_MON_CTL: c_uint;
    static CS35L34_ADSP_I2S_CTL: c_uint;
    static CS35L34_ADSP_TDM_CTL: c_uint;
    static CS35L34_TDM_TX_CTL_1_VMON: c_uint;
    static CS35L34_TDM_TX_CTL_2_IMON: c_uint;
    static CS35L34_TDM_TX_CTL_3_VPMON: c_uint;
    static CS35L34_TDM_TX_CTL_4_VBSTMON: c_uint;
    static CS35L34_TDM_TX_CTL_5_FLAG1: c_uint;
    static CS35L34_TDM_TX_CTL_6_FLAG2: c_uint;
    static CS35L34_TDM_TX_SLOT_EN_1: c_uint;
    static CS35L34_TDM_TX_SLOT_EN_2: c_uint;
    static CS35L34_TDM_TX_SLOT_EN_3: c_uint;
    static CS35L34_TDM_TX_SLOT_EN_4: c_uint;
    static CS35L34_TDM_RX_CTL_1_AUDIN: c_uint;
    static CS35L34_TDM_RX_CTL_3_ALIVE: c_uint;
    static CS35L34_MULT_DEV_SYNCH1: c_uint;
    static CS35L34_MULT_DEV_SYNCH2: c_uint;
    static CS35L34_PROT_RELEASE_CTL: c_uint;
    static CS35L34_DIAG_MODE_REG_LOCK: c_uint;
    static CS35L34_DIAG_MODE_CTL_1: c_uint;
    static CS35L34_DIAG_MODE_CTL_2: c_uint;
    static CS35L34_INT_MASK_1: c_uint;
    static CS35L34_INT_MASK_2: c_uint;
    static CS35L34_INT_MASK_3: c_uint;
    static CS35L34_INT_MASK_4: c_uint;
    static CS35L34_INT_STATUS_1: c_uint;
    static CS35L34_INT_STATUS_2: c_uint;
    static CS35L34_INT_STATUS_3: c_uint;
    static CS35L34_INT_STATUS_4: c_uint;
    static CS35L34_OTP_TRIM_STATUS: c_uint;
    static CS35L34_DEVID_AB: c_uint;
    static CS35L34_DEVID_CD: c_uint;
    static CS35L34_DEVID_E: c_uint;
    static CS35L34_FAB_ID: c_uint;
    static CS35L34_REV_ID: c_uint;
    static CS35L34_PDN_TDM: c_uint;
    static CS35L34_PDN_ALL: c_uint;
    static CS35L34_X_LOC: c_uint;
    static CS35L34_X_STATE: c_uint;
    static CS35L34_BST_CVTL_MASK: c_uint;
    static CS35L34_MUTE: c_uint;
    static CS35L34_AMP_DIGSFT: c_uint;
    static CS35L34_ADSP_RATE: c_uint;
    static CS35L34_PDN_DONE: c_uint;
    static CS35L34_PDN_SDOUT: c_uint;
    static CS35L34_MCLK_5644: c_uint;
    static CS35L34_MCLK_6: c_uint;
    static CS35L34_MCLK_6144: c_uint;
    static CS35L34_MCLK_11289: c_uint;
    static CS35L34_MCLK_12: c_uint;
    static CS35L34_MCLK_12288: c_uint;
    static CS35L34_MCLK_RATE_5P6448: c_uint;
    static CS35L34_MCLK_RATE_6P0000: c_uint;
    static CS35L34_MCLK_RATE_6P1440: c_uint;
    static CS35L34_MCLK_DIV: c_uint;
    static CS35L34_MCLK_RATE_MASK: c_uint;
    static CS35L34_RATES: c_uint;
    static CS35L34_FORMATS: c_uint;
    static CS35L34_BST_PEAK_MASK: c_uint;
    static CS35L34_GAIN_ZC_MASK: c_uint;
    static CS35L34_ADSP_DRIVE: c_uint;
    static CS35L34_INV: c_uint;
    static CS35L34_I2S_LOC_MASK: c_uint;
    static CS35L34_I2S_LOC_SHIFT: c_uint;
    static CS35L34_MAX_REGISTER: c_uint;
    static CS35L34_CAL_ERR: c_uint;
    static CS35L34_CAL_ERR_RLS: c_uint;
    static CS35L34_ALIVE_ERR: c_uint;
    static CS35L34_AMP_SHORT: c_uint;
    static CS35L34_SHORT_RLS: c_uint;
    static CS35L34_OTW: c_uint;
    static CS35L34_OTW_RLS: c_uint;
    static CS35L34_OTE: c_uint;
    static CS35L34_OTE_RLS: c_uint;
    static CS35L34_BST_HIGH: c_uint;
    static CS35L34_LBST_SHORT: c_uint;
    static CS35L34_PDN_AMP: c_uint;
    static CS35L34_M_CAL_ERR: c_uint;
    static CS35L34_M_ALIVE_ERR: c_uint;
    static CS35L34_M_AMP_SHORT: c_uint;
    static CS35L34_M_OTW: c_uint;
    static CS35L34_M_OTE: c_uint;
    static CS35L34_M_BST_HIGH: c_uint;
    static CS35L34_M_LBST_SHORT: c_uint;
    static CS35L34_CHIP_ID: c_int;

    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;
    static SND_SOC_DAPM_POST_PMU: c_int;
    static SND_SOC_DAPM_PRE_PMD: c_int;
    static SND_SOC_DAIFMT_MASTER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static REGCACHE_MAPLE: c_uint;
    static IRQ_NONE: irqreturn_t;
    static IRQ_HANDLED: irqreturn_t;
    static IRQF_ONESHOT: c_uint;
    static IRQF_TRIGGER_LOW: c_uint;
    static GPIOD_OUT_LOW: c_uint;
    static GFP_KERNEL: c_uint;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static ENODEV: c_int;

    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn msleep(ms: c_uint);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_crit(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn pm_runtime_put_sync(dev: *mut device) -> c_int;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut c_uint)
        -> c_int;
    fn of_property_read_bool(np: *mut device_node, propname: *const c_char) -> bool;
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_regulator_bulk_get(
        dev: *mut device,
        num_consumers: c_int,
        consumers: *mut regulator_bulk_data,
    ) -> c_int;
    fn regulator_bulk_enable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn devm_request_threaded_irq(
        dev: *mut device,
        irq: c_int,
        handler: *const c_void,
        thread_fn: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        irqflags: c_uint,
        devname: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint)
        -> *mut gpio_desc;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn cirrus_read_device_id(map: *mut regmap, reg: c_uint) -> c_int;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn enable_irq(irq: c_int);
    fn disable_irq(irq: c_int);
    fn to_i2c_client(dev: *mut device) -> *mut i2c_client;
    fn i2c_add_driver(driver: *mut i2c_driver) -> c_int;
    fn i2c_del_driver(driver: *mut i2c_driver);
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn ffs(x: c_uint) -> c_int {
    if x == 0 {
        0
    } else {
        x.trailing_zeros() as c_int + 1
    }
}

static cs35l34_reg: [reg_default; 66] = unsafe {
    [
        reg_default { reg: CS35L34_PWRCTL1, def: 0x01 },
        reg_default { reg: CS35L34_PWRCTL2, def: 0x19 },
        reg_default { reg: CS35L34_PWRCTL3, def: 0x01 },
        reg_default { reg: CS35L34_ADSP_CLK_CTL, def: 0x08 },
        reg_default { reg: CS35L34_MCLK_CTL, def: 0x11 },
        reg_default { reg: CS35L34_AMP_INP_DRV_CTL, def: 0x01 },
        reg_default { reg: CS35L34_AMP_DIG_VOL_CTL, def: 0x12 },
        reg_default { reg: CS35L34_AMP_DIG_VOL, def: 0x00 },
        reg_default { reg: CS35L34_AMP_ANLG_GAIN_CTL, def: 0x0F },
        reg_default { reg: CS35L34_PROTECT_CTL, def: 0x06 },
        reg_default { reg: CS35L34_AMP_KEEP_ALIVE_CTL, def: 0x04 },
        reg_default { reg: CS35L34_BST_CVTR_V_CTL, def: 0x00 },
        reg_default { reg: CS35L34_BST_PEAK_I, def: 0x10 },
        reg_default { reg: CS35L34_BST_RAMP_CTL, def: 0x87 },
        reg_default { reg: CS35L34_BST_CONV_COEF_1, def: 0x24 },
        reg_default { reg: CS35L34_BST_CONV_COEF_2, def: 0x24 },
        reg_default { reg: CS35L34_BST_CONV_SLOPE_COMP, def: 0x4E },
        reg_default { reg: CS35L34_BST_CONV_SW_FREQ, def: 0x08 },
        reg_default { reg: CS35L34_CLASS_H_CTL, def: 0x0D },
        reg_default { reg: CS35L34_CLASS_H_HEADRM_CTL, def: 0x0D },
        reg_default { reg: CS35L34_CLASS_H_RELEASE_RATE, def: 0x08 },
        reg_default { reg: CS35L34_CLASS_H_FET_DRIVE_CTL, def: 0x41 },
        reg_default { reg: CS35L34_CLASS_H_STATUS, def: 0x05 },
        reg_default { reg: CS35L34_VPBR_CTL, def: 0x0A },
        reg_default { reg: CS35L34_VPBR_VOL_CTL, def: 0x90 },
        reg_default { reg: CS35L34_VPBR_TIMING_CTL, def: 0x6A },
        reg_default { reg: CS35L34_PRED_MAX_ATTEN_SPK_LOAD, def: 0x95 },
        reg_default { reg: CS35L34_PRED_BROWNOUT_THRESH, def: 0x1C },
        reg_default { reg: CS35L34_PRED_BROWNOUT_VOL_CTL, def: 0x00 },
        reg_default { reg: CS35L34_PRED_BROWNOUT_RATE_CTL, def: 0x10 },
        reg_default { reg: CS35L34_PRED_WAIT_CTL, def: 0x10 },
        reg_default { reg: CS35L34_PRED_ZVP_INIT_IMP_CTL, def: 0x08 },
        reg_default { reg: CS35L34_PRED_MAN_SAFE_VPI_CTL, def: 0x80 },
        reg_default { reg: CS35L34_VPBR_ATTEN_STATUS, def: 0x00 },
        reg_default { reg: CS35L34_PRED_BRWNOUT_ATT_STATUS, def: 0x00 },
        reg_default { reg: CS35L34_SPKR_MON_CTL, def: 0xC6 },
        reg_default { reg: CS35L34_ADSP_I2S_CTL, def: 0x00 },
        reg_default { reg: CS35L34_ADSP_TDM_CTL, def: 0x00 },
        reg_default { reg: CS35L34_TDM_TX_CTL_1_VMON, def: 0x00 },
        reg_default { reg: CS35L34_TDM_TX_CTL_2_IMON, def: 0x04 },
        reg_default { reg: CS35L34_TDM_TX_CTL_3_VPMON, def: 0x03 },
        reg_default { reg: CS35L34_TDM_TX_CTL_4_VBSTMON, def: 0x07 },
        reg_default { reg: CS35L34_TDM_TX_CTL_5_FLAG1, def: 0x08 },
        reg_default { reg: CS35L34_TDM_TX_CTL_6_FLAG2, def: 0x09 },
        reg_default { reg: CS35L34_TDM_TX_SLOT_EN_1, def: 0x00 },
        reg_default { reg: CS35L34_TDM_TX_SLOT_EN_2, def: 0x00 },
        reg_default { reg: CS35L34_TDM_TX_SLOT_EN_3, def: 0x00 },
        reg_default { reg: CS35L34_TDM_TX_SLOT_EN_4, def: 0x00 },
        reg_default { reg: CS35L34_TDM_RX_CTL_1_AUDIN, def: 0x40 },
        reg_default { reg: CS35L34_TDM_RX_CTL_3_ALIVE, def: 0x04 },
        reg_default { reg: CS35L34_MULT_DEV_SYNCH1, def: 0x00 },
        reg_default { reg: CS35L34_MULT_DEV_SYNCH2, def: 0x80 },
        reg_default { reg: CS35L34_PROT_RELEASE_CTL, def: 0x00 },
        reg_default { reg: CS35L34_DIAG_MODE_REG_LOCK, def: 0x00 },
        reg_default { reg: CS35L34_DIAG_MODE_CTL_1, def: 0x00 },
        reg_default { reg: CS35L34_DIAG_MODE_CTL_2, def: 0x00 },
        reg_default { reg: CS35L34_INT_MASK_1, def: 0xFF },
        reg_default { reg: CS35L34_INT_MASK_2, def: 0xFF },
        reg_default { reg: CS35L34_INT_MASK_3, def: 0xFF },
        reg_default { reg: CS35L34_INT_MASK_4, def: 0xFF },
        reg_default { reg: CS35L34_INT_STATUS_1, def: 0x30 },
        reg_default { reg: CS35L34_INT_STATUS_2, def: 0x05 },
        reg_default { reg: CS35L34_INT_STATUS_3, def: 0x00 },
        reg_default { reg: CS35L34_INT_STATUS_4, def: 0x00 },
        reg_default { reg: CS35L34_OTP_TRIM_STATUS, def: 0x00 },
    ]
};

unsafe extern "C" fn cs35l34_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        x if x == CS35L34_DEVID_AB || x == CS35L34_DEVID_CD || x == CS35L34_DEVID_E ||
            x == CS35L34_FAB_ID || x == CS35L34_REV_ID || x == CS35L34_INT_STATUS_1 ||
            x == CS35L34_INT_STATUS_2 || x == CS35L34_INT_STATUS_3 ||
            x == CS35L34_INT_STATUS_4 || x == CS35L34_CLASS_H_STATUS ||
            x == CS35L34_VPBR_ATTEN_STATUS || x == CS35L34_OTP_TRIM_STATUS => true,
        _ => false,
    }
}

unsafe extern "C" fn cs35l34_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    for default_reg in cs35l34_reg.iter() {
        if default_reg.reg == reg {
            return true;
        }
    }
    reg == CS35L34_DEVID_AB || reg == CS35L34_DEVID_CD || reg == CS35L34_DEVID_E ||
        reg == CS35L34_FAB_ID || reg == CS35L34_REV_ID
}

unsafe extern "C" fn cs35l34_precious_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        x if x == CS35L34_INT_STATUS_1 || x == CS35L34_INT_STATUS_2 ||
            x == CS35L34_INT_STATUS_3 || x == CS35L34_INT_STATUS_4 => true,
        _ => false,
    }
}

unsafe extern "C" fn cs35l34_sdin_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs35l34_private;
    let mut ret: c_int;

    if event == SND_SOC_DAPM_PRE_PMU {
        if (*priv_).tdm_mode {
            regmap_update_bits((*priv_).regmap, CS35L34_PWRCTL3, CS35L34_PDN_TDM, 0x00);
        }
        ret = regmap_update_bits((*priv_).regmap, CS35L34_PWRCTL1, CS35L34_PDN_ALL, 0);
        if ret < 0 {
            dev_err((*component).dev, cstr!("Cannot set Power bits %d\n"), ret);
            return ret;
        }
        usleep_range(5000, 5100);
    } else if event == SND_SOC_DAPM_POST_PMD {
        if (*priv_).tdm_mode {
            regmap_update_bits(
                (*priv_).regmap,
                CS35L34_PWRCTL3,
                CS35L34_PDN_TDM,
                CS35L34_PDN_TDM,
            );
        }
        ret = regmap_update_bits(
            (*priv_).regmap,
            CS35L34_PWRCTL1,
            CS35L34_PDN_ALL,
            CS35L34_PDN_ALL,
        );
        let _ = ret;
    } else {
        pr_err(cstr!("Invalid event = 0x%x\n"), event);
    }
    0
}

unsafe extern "C" fn cs35l34_set_tdm_slot(
    dai: *mut snd_soc_dai,
    mut tx_mask: c_uint,
    rx_mask: c_uint,
    _slots: c_int,
    slot_width: c_int,
) -> c_int {
    let component = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs35l34_private;
    let mut reg: c_uint;
    let mut bit_pos: c_uint;
    let mut slot: c_int;
    let mut slot_num: c_int;

    if slot_width != 8 {
        return -EINVAL;
    }

    (*priv_).tdm_mode = true;
    /* scan rx_mask for aud slot */
    slot = ffs(rx_mask) - 1;
    if slot >= 0 {
        snd_soc_component_update_bits(
            component,
            CS35L34_TDM_RX_CTL_1_AUDIN,
            CS35L34_X_LOC,
            slot as c_uint,
        );
    }

    /* scan tx_mask: vmon(2 slots); imon (2 slots); vpmon (1 slot)
     * vbstmon (1 slot)
     */
    slot = ffs(tx_mask) - 1;
    slot_num = 0;

    /* disable vpmon/vbstmon: enable later if set in tx_mask */
    snd_soc_component_update_bits(
        component,
        CS35L34_TDM_TX_CTL_3_VPMON,
        CS35L34_X_STATE | CS35L34_X_LOC,
        CS35L34_X_STATE | CS35L34_X_LOC,
    );
    snd_soc_component_update_bits(
        component,
        CS35L34_TDM_TX_CTL_4_VBSTMON,
        CS35L34_X_STATE | CS35L34_X_LOC,
        CS35L34_X_STATE | CS35L34_X_LOC,
    );

    /* disconnect {vp,vbst}_mon routes: eanble later if set in tx_mask*/
    while slot >= 0 {
        /* configure VMON_TX_LOC */
        if slot_num == 0 {
            snd_soc_component_update_bits(
                component,
                CS35L34_TDM_TX_CTL_1_VMON,
                CS35L34_X_STATE | CS35L34_X_LOC,
                slot as c_uint,
            );
        }

        /* configure IMON_TX_LOC */
        if slot_num == 4 {
            snd_soc_component_update_bits(
                component,
                CS35L34_TDM_TX_CTL_2_IMON,
                CS35L34_X_STATE | CS35L34_X_LOC,
                slot as c_uint,
            );
        }
        /* configure VPMON_TX_LOC */
        if slot_num == 3 {
            snd_soc_component_update_bits(
                component,
                CS35L34_TDM_TX_CTL_3_VPMON,
                CS35L34_X_STATE | CS35L34_X_LOC,
                slot as c_uint,
            );
        }
        /* configure VBSTMON_TX_LOC */
        if slot_num == 7 {
            snd_soc_component_update_bits(
                component,
                CS35L34_TDM_TX_CTL_4_VBSTMON,
                CS35L34_X_STATE | CS35L34_X_LOC,
                slot as c_uint,
            );
        }

        /* Enable the relevant tx slot */
        reg = CS35L34_TDM_TX_SLOT_EN_4 - ((slot / 8) as c_uint);
        bit_pos = (slot - ((slot / 8) * 8)) as c_uint;
        snd_soc_component_update_bits(component, reg, 1 << bit_pos, 1 << bit_pos);

        tx_mask &= !(1 << slot);
        slot = ffs(tx_mask) - 1;
        slot_num += 1;
    }

    0
}

unsafe extern "C" fn cs35l34_main_amp_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs35l34_private;

    if event == SND_SOC_DAPM_POST_PMU {
        regmap_update_bits(
            (*priv_).regmap,
            CS35L34_BST_CVTR_V_CTL,
            CS35L34_BST_CVTL_MASK,
            (*priv_).pdata.boost_vtge,
        );
        usleep_range(5000, 5100);
        regmap_update_bits((*priv_).regmap, CS35L34_PROTECT_CTL, CS35L34_MUTE, 0);
    } else if event == SND_SOC_DAPM_POST_PMD {
        regmap_update_bits(
            (*priv_).regmap,
            CS35L34_BST_CVTR_V_CTL,
            CS35L34_BST_CVTL_MASK,
            0,
        );
        regmap_update_bits(
            (*priv_).regmap,
            CS35L34_PROTECT_CTL,
            CS35L34_MUTE,
            CS35L34_MUTE,
        );
        usleep_range(5000, 5100);
    } else {
        pr_err(cstr!("Invalid event = 0x%x\n"), event);
    }
    0
}

static dig_vol_tlv: [c_uint; 4] = [0, (-10200i32) as c_uint, 50, 0];
static amp_gain_tlv: [c_uint; 4] = [0, 300, 100, 0];

// SOC_SINGLE_SX_TLV/SOC_SINGLE_TLV initializers require ALSA macro expansion.
static cs35l34_snd_controls: [snd_kcontrol_new; 0] = [];

unsafe extern "C" fn cs35l34_mclk_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs35l34_private;
    let mut ret: c_int;
    let mut i: c_int;
    let mut reg: c_uint = 0;

    if event == SND_SOC_DAPM_PRE_PMD {
        ret = regmap_read((*priv_).regmap, CS35L34_AMP_DIG_VOL_CTL, &mut reg);
        if ret != 0 {
            pr_err(cstr!("%s regmap read failure %d\n"), cstr!("cs35l34_mclk_event"), ret);
            return ret;
        }
        if (reg & CS35L34_AMP_DIGSFT) != 0 {
            msleep(40);
        } else {
            usleep_range(2000, 2100);
        }

        i = 0;
        while i < PDN_DONE_ATTEMPTS {
            ret = regmap_read((*priv_).regmap, CS35L34_INT_STATUS_2, &mut reg);
            if ret != 0 {
                pr_err(cstr!("%s regmap read failure %d\n"), cstr!("cs35l34_mclk_event"), ret);
                return ret;
            }
            if (reg & CS35L34_PDN_DONE) != 0 {
                break;
            }
            usleep_range(5000, 5100);
            i += 1;
        }
        if i == PDN_DONE_ATTEMPTS {
            pr_err(
                cstr!("%s Device did not power down properly\n"),
                cstr!("cs35l34_mclk_event"),
            );
        }
    } else {
        pr_err(cstr!("Invalid event = 0x%x\n"), event);
    }
    0
}

// SND_SOC_DAPM_* widget macro initializers require ALSA macro expansion.
static cs35l34_dapm_widgets: [snd_soc_dapm_widget_desc; 0] = [];

static cs35l34_audio_map: [snd_soc_dapm_route; 17] = [
    snd_soc_dapm_route { sink: cstr!("SDIN"), control: core::ptr::null(), source: cstr!("AMP Playback") },
    snd_soc_dapm_route { sink: cstr!("BOOST"), control: core::ptr::null(), source: cstr!("SDIN") },
    snd_soc_dapm_route { sink: cstr!("CLASS H"), control: core::ptr::null(), source: cstr!("BOOST") },
    snd_soc_dapm_route { sink: cstr!("Main AMP"), control: core::ptr::null(), source: cstr!("CLASS H") },
    snd_soc_dapm_route { sink: cstr!("SPK"), control: core::ptr::null(), source: cstr!("Main AMP") },
    snd_soc_dapm_route { sink: cstr!("VPMON ADC"), control: core::ptr::null(), source: cstr!("CLASS H") },
    snd_soc_dapm_route { sink: cstr!("VBSTMON ADC"), control: core::ptr::null(), source: cstr!("CLASS H") },
    snd_soc_dapm_route { sink: cstr!("SPK"), control: core::ptr::null(), source: cstr!("VPMON ADC") },
    snd_soc_dapm_route { sink: cstr!("SPK"), control: core::ptr::null(), source: cstr!("VBSTMON ADC") },
    snd_soc_dapm_route { sink: cstr!("IMON ADC"), control: core::ptr::null(), source: cstr!("ISENSE") },
    snd_soc_dapm_route { sink: cstr!("VMON ADC"), control: core::ptr::null(), source: cstr!("VSENSE") },
    snd_soc_dapm_route { sink: cstr!("SDOUT"), control: core::ptr::null(), source: cstr!("IMON ADC") },
    snd_soc_dapm_route { sink: cstr!("SDOUT"), control: core::ptr::null(), source: cstr!("VMON ADC") },
    snd_soc_dapm_route { sink: cstr!("AMP Capture"), control: core::ptr::null(), source: cstr!("SDOUT") },
    snd_soc_dapm_route { sink: cstr!("SDIN"), control: core::ptr::null(), source: cstr!("EXTCLK") },
    snd_soc_dapm_route { sink: cstr!("SDOUT"), control: core::ptr::null(), source: cstr!("EXTCLK") },
    snd_soc_dapm_route { sink: core::ptr::null(), control: core::ptr::null(), source: core::ptr::null() },
];

#[repr(C)]
#[derive(Copy, Clone)]
struct cs35l34_mclk_div {
    mclk: c_int,
    srate: c_int,
    adsp_rate: u8,
}

static mut cs35l34_mclk_coeffs: [cs35l34_mclk_div; 21] = [
    /* MCLK, Sample Rate, adsp_rate */
    cs35l34_mclk_div { mclk: 5644800, srate: 11025, adsp_rate: 0x1 },
    cs35l34_mclk_div { mclk: 5644800, srate: 22050, adsp_rate: 0x4 },
    cs35l34_mclk_div { mclk: 5644800, srate: 44100, adsp_rate: 0x7 },
    cs35l34_mclk_div { mclk: 6000000, srate: 8000, adsp_rate: 0x0 },
    cs35l34_mclk_div { mclk: 6000000, srate: 11025, adsp_rate: 0x1 },
    cs35l34_mclk_div { mclk: 6000000, srate: 12000, adsp_rate: 0x2 },
    cs35l34_mclk_div { mclk: 6000000, srate: 16000, adsp_rate: 0x3 },
    cs35l34_mclk_div { mclk: 6000000, srate: 22050, adsp_rate: 0x4 },
    cs35l34_mclk_div { mclk: 6000000, srate: 24000, adsp_rate: 0x5 },
    cs35l34_mclk_div { mclk: 6000000, srate: 32000, adsp_rate: 0x6 },
    cs35l34_mclk_div { mclk: 6000000, srate: 44100, adsp_rate: 0x7 },
    cs35l34_mclk_div { mclk: 6000000, srate: 48000, adsp_rate: 0x8 },
    cs35l34_mclk_div { mclk: 6144000, srate: 8000, adsp_rate: 0x0 },
    cs35l34_mclk_div { mclk: 6144000, srate: 11025, adsp_rate: 0x1 },
    cs35l34_mclk_div { mclk: 6144000, srate: 12000, adsp_rate: 0x2 },
    cs35l34_mclk_div { mclk: 6144000, srate: 16000, adsp_rate: 0x3 },
    cs35l34_mclk_div { mclk: 6144000, srate: 22050, adsp_rate: 0x4 },
    cs35l34_mclk_div { mclk: 6144000, srate: 24000, adsp_rate: 0x5 },
    cs35l34_mclk_div { mclk: 6144000, srate: 32000, adsp_rate: 0x6 },
    cs35l34_mclk_div { mclk: 6144000, srate: 44100, adsp_rate: 0x7 },
    cs35l34_mclk_div { mclk: 6144000, srate: 48000, adsp_rate: 0x8 },
];

unsafe fn cs35l34_get_mclk_coeff(mclk: c_int, srate: c_int) -> c_int {
    let mut i: usize = 0;
    while i < cs35l34_mclk_coeffs.len() {
        if cs35l34_mclk_coeffs[i].mclk == mclk && cs35l34_mclk_coeffs[i].srate == srate {
            return i as c_int;
        }
        i += 1;
    }
    -EINVAL
}

unsafe extern "C" fn cs35l34_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs35l34_private;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        x if x == SND_SOC_DAIFMT_CBP_CFP => {
            regmap_update_bits((*priv_).regmap, CS35L34_ADSP_CLK_CTL, 0x80, 0x80);
        }
        x if x == SND_SOC_DAIFMT_CBC_CFC => {
            regmap_update_bits((*priv_).regmap, CS35L34_ADSP_CLK_CTL, 0x80, 0x00);
        }
        _ => return -EINVAL,
    }
    0
}

unsafe extern "C" fn cs35l34_pcm_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs35l34_private;
    let srate = params_rate(params);
    let coeff = cs35l34_get_mclk_coeff((*priv_).mclk_int, srate);

    if coeff < 0 {
        dev_err(
            (*component).dev,
            cstr!("ERROR: Invalid mclk %d and/or srate %d\n"),
            (*priv_).mclk_int,
            srate,
        );
        return coeff;
    }

    let ret = regmap_update_bits(
        (*priv_).regmap,
        CS35L34_ADSP_CLK_CTL,
        CS35L34_ADSP_RATE,
        cs35l34_mclk_coeffs[coeff as usize].adsp_rate as c_uint,
    );
    if ret != 0 {
        dev_err((*component).dev, cstr!("Failed to set clock state %d\n"), ret);
    }

    ret
}

unsafe extern "C" fn cs35l34_set_tristate(dai: *mut snd_soc_dai, tristate: c_int) -> c_int {
    let component = (*dai).component;

    if tristate != 0 {
        snd_soc_component_update_bits(
            component,
            CS35L34_PWRCTL3,
            CS35L34_PDN_SDOUT,
            CS35L34_PDN_SDOUT,
        );
    } else {
        snd_soc_component_update_bits(component, CS35L34_PWRCTL3, CS35L34_PDN_SDOUT, 0);
    }
    0
}

unsafe extern "C" fn cs35l34_dai_set_sysclk(
    dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*dai).component;
    let cs35l34 = snd_soc_component_get_drvdata(component) as *mut cs35l34_private;
    let value: c_uint;

    if freq == CS35L34_MCLK_5644 {
        value = CS35L34_MCLK_RATE_5P6448;
        (*cs35l34).mclk_int = freq as c_int;
    } else if freq == CS35L34_MCLK_6 {
        value = CS35L34_MCLK_RATE_6P0000;
        (*cs35l34).mclk_int = freq as c_int;
    } else if freq == CS35L34_MCLK_6144 {
        value = CS35L34_MCLK_RATE_6P1440;
        (*cs35l34).mclk_int = freq as c_int;
    } else if freq == CS35L34_MCLK_11289 {
        value = CS35L34_MCLK_DIV | CS35L34_MCLK_RATE_5P6448;
        (*cs35l34).mclk_int = (freq / 2) as c_int;
    } else if freq == CS35L34_MCLK_12 {
        value = CS35L34_MCLK_DIV | CS35L34_MCLK_RATE_6P0000;
        (*cs35l34).mclk_int = (freq / 2) as c_int;
    } else if freq == CS35L34_MCLK_12288 {
        value = CS35L34_MCLK_DIV | CS35L34_MCLK_RATE_6P1440;
        (*cs35l34).mclk_int = (freq / 2) as c_int;
    } else {
        dev_err((*component).dev, cstr!("ERROR: Invalid Frequency %d\n"), freq);
        (*cs35l34).mclk_int = 0;
        return -EINVAL;
    }
    regmap_update_bits(
        (*cs35l34).regmap,
        CS35L34_MCLK_CTL,
        CS35L34_MCLK_DIV | CS35L34_MCLK_RATE_MASK,
        value,
    );
    0
}

static cs35l34_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_tristate: Some(cs35l34_set_tristate),
    set_fmt: Some(cs35l34_set_dai_fmt),
    hw_params: Some(cs35l34_pcm_hw_params),
    set_sysclk: Some(cs35l34_dai_set_sysclk),
    set_tdm_slot: Some(cs35l34_set_tdm_slot),
};

static mut cs35l34_dai: snd_soc_dai_driver = unsafe {
    snd_soc_dai_driver {
        name: cstr!("cs35l34"),
        id: 0,
        playback: snd_soc_pcm_stream {
            stream_name: cstr!("AMP Playback"),
            channels_min: 1,
            channels_max: 8,
            rates: CS35L34_RATES,
            formats: CS35L34_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: cstr!("AMP Capture"),
            channels_min: 1,
            channels_max: 8,
            rates: CS35L34_RATES,
            formats: CS35L34_FORMATS,
        },
        ops: &cs35l34_ops,
        symmetric_rate: 1,
    }
};

unsafe fn cs35l34_boost_inductor(cs35l34: *mut cs35l34_private, inductor: c_uint) -> c_int {
    let component = (*cs35l34).component;

    match inductor {
        1000 => {
            /* 1 uH */
            regmap_write((*cs35l34).regmap, CS35L34_BST_CONV_COEF_1, 0x24);
            regmap_write((*cs35l34).regmap, CS35L34_BST_CONV_COEF_2, 0x24);
            regmap_write((*cs35l34).regmap, CS35L34_BST_CONV_SLOPE_COMP, 0x4E);
            regmap_write((*cs35l34).regmap, CS35L34_BST_CONV_SW_FREQ, 0);
        }
        1200 => {
            /* 1.2 uH */
            regmap_write((*cs35l34).regmap, CS35L34_BST_CONV_COEF_1, 0x20);
            regmap_write((*cs35l34).regmap, CS35L34_BST_CONV_COEF_2, 0x20);
            regmap_write((*cs35l34).regmap, CS35L34_BST_CONV_SLOPE_COMP, 0x47);
            regmap_write((*cs35l34).regmap, CS35L34_BST_CONV_SW_FREQ, 1);
        }
        1500 => {
            /* 1.5uH */
            regmap_write((*cs35l34).regmap, CS35L34_BST_CONV_COEF_1, 0x20);
            regmap_write((*cs35l34).regmap, CS35L34_BST_CONV_COEF_2, 0x20);
            regmap_write((*cs35l34).regmap, CS35L34_BST_CONV_SLOPE_COMP, 0x3C);
            regmap_write((*cs35l34).regmap, CS35L34_BST_CONV_SW_FREQ, 2);
        }
        2200 => {
            /* 2.2uH */
            regmap_write((*cs35l34).regmap, CS35L34_BST_CONV_COEF_1, 0x19);
            regmap_write((*cs35l34).regmap, CS35L34_BST_CONV_COEF_2, 0x25);
            regmap_write((*cs35l34).regmap, CS35L34_BST_CONV_SLOPE_COMP, 0x23);
            regmap_write((*cs35l34).regmap, CS35L34_BST_CONV_SW_FREQ, 3);
        }
        _ => {
            dev_err(
                (*component).dev,
                cstr!("%s Invalid Inductor Value %d uH\n"),
                cstr!("cs35l34_boost_inductor"),
                inductor,
            );
            return -EINVAL;
        }
    }
    0
}

unsafe extern "C" fn cs35l34_probe(component: *mut snd_soc_component) -> c_int {
    let mut ret: c_int = 0;
    let cs35l34 = snd_soc_component_get_drvdata(component) as *mut cs35l34_private;

    pm_runtime_get_sync((*component).dev);
    regmap_update_bits((*cs35l34).regmap, CS35L34_PROTECT_CTL, CS35L34_OTW_ATTN_MASK, 0x8);
    regmap_write((*cs35l34).regmap, CS35L34_PWRCTL2, 0xFD);
    regmap_write((*cs35l34).regmap, CS35L34_PWRCTL3, 0x1F);
    regmap_update_bits((*cs35l34).regmap, CS35L34_PROTECT_CTL, CS35L34_MUTE, CS35L34_MUTE);

    if (*cs35l34).pdata.boost_peak != 0 {
        regmap_update_bits((*cs35l34).regmap, CS35L34_BST_PEAK_I, CS35L34_BST_PEAK_MASK, (*cs35l34).pdata.boost_peak);
    }
    if (*cs35l34).pdata.gain_zc_disable {
        regmap_update_bits((*cs35l34).regmap, CS35L34_PROTECT_CTL, CS35L34_GAIN_ZC_MASK, 0);
    } else {
        regmap_update_bits((*cs35l34).regmap, CS35L34_PROTECT_CTL, CS35L34_GAIN_ZC_MASK, CS35L34_GAIN_ZC_MASK);
    }
    if (*cs35l34).pdata.aif_half_drv {
        regmap_update_bits((*cs35l34).regmap, CS35L34_ADSP_CLK_CTL, CS35L34_ADSP_DRIVE, 0);
    }
    if (*cs35l34).pdata.digsft_disable {
        regmap_update_bits((*cs35l34).regmap, CS35L34_AMP_DIG_VOL_CTL, CS35L34_AMP_DIGSFT, 0);
    }
    if (*cs35l34).pdata.amp_inv {
        regmap_update_bits((*cs35l34).regmap, CS35L34_AMP_DIG_VOL_CTL, CS35L34_INV, CS35L34_INV);
    }
    if (*cs35l34).pdata.boost_ind != 0 {
        ret = cs35l34_boost_inductor(cs35l34, (*cs35l34).pdata.boost_ind);
    }
    if (*cs35l34).pdata.i2s_sdinloc != 0 {
        regmap_update_bits(
            (*cs35l34).regmap,
            CS35L34_ADSP_I2S_CTL,
            CS35L34_I2S_LOC_MASK,
            (*cs35l34).pdata.i2s_sdinloc << CS35L34_I2S_LOC_SHIFT,
        );
    }
    if (*cs35l34).pdata.tdm_rising_edge != 0 {
        regmap_update_bits((*cs35l34).regmap, CS35L34_ADSP_TDM_CTL, 1, 1);
    }

    pm_runtime_put_sync((*component).dev);
    ret
}

extern "C" {
    static CS35L34_OTW_ATTN_MASK: c_uint;
}

static soc_component_dev_cs35l34: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(cs35l34_probe),
    dapm_widgets: cs35l34_dapm_widgets.as_ptr(),
    num_dapm_widgets: cs35l34_dapm_widgets.len(),
    dapm_routes: cs35l34_audio_map.as_ptr(),
    num_dapm_routes: cs35l34_audio_map.len(),
    controls: cs35l34_snd_controls.as_ptr(),
    num_controls: cs35l34_snd_controls.len(),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static cs35l34_regmap: regmap_config = unsafe {
    regmap_config {
        reg_bits: 8,
        val_bits: 8,
        max_register: CS35L34_MAX_REGISTER,
        reg_defaults: cs35l34_reg.as_ptr(),
        num_reg_defaults: cs35l34_reg.len(),
        volatile_reg: Some(cs35l34_volatile_register),
        readable_reg: Some(cs35l34_readable_register),
        precious_reg: Some(cs35l34_precious_register),
        cache_type: REGCACHE_MAPLE,
        use_single_read: true,
        use_single_write: true,
    }
};

unsafe fn cs35l34_handle_of_data(i2c_client: *mut i2c_client, pdata: *mut cs35l34_platform_data) -> c_int {
    let np = (*i2c_client).dev.of_node;
    let dev = &mut (*i2c_client).dev as *mut device_with_of_node as *mut device;
    let mut val: c_uint = 0;

    if of_property_read_u32(np, cstr!("cirrus,boost-vtge-millivolt"), &mut val) >= 0 {
        /* Boost Voltage has a maximum of 8V */
        if val > 8000 || (val < 3300 && val > 0) {
            dev_err(dev, cstr!("Invalid Boost Voltage %d mV\n"), val);
            return -EINVAL;
        }
        if val == 0 {
            (*pdata).boost_vtge = 0; /* Use VP */
        } else {
            (*pdata).boost_vtge = ((val - 3300) / 100) + 1;
        }
    } else {
        dev_warn(dev, cstr!("Boost Voltage not specified. Using VP\n"));
    }

    if of_property_read_u32(np, cstr!("cirrus,boost-ind-nanohenry"), &mut val) >= 0 {
        (*pdata).boost_ind = val;
    } else {
        dev_err(dev, cstr!("Inductor not specified.\n"));
        return -EINVAL;
    }

    if of_property_read_u32(np, cstr!("cirrus,boost-peak-milliamp"), &mut val) >= 0 {
        if val > 3840 || val < 1200 {
            dev_err(dev, cstr!("Invalid Boost Peak Current %d mA\n"), val);
            return -EINVAL;
        }
        (*pdata).boost_peak = ((val - 1200) / 80) + 1;
    }

    (*pdata).aif_half_drv = of_property_read_bool(np, cstr!("cirrus,aif-half-drv"));
    (*pdata).digsft_disable = of_property_read_bool(np, cstr!("cirrus,digsft-disable"));
    (*pdata).gain_zc_disable = of_property_read_bool(np, cstr!("cirrus,gain-zc-disable"));
    (*pdata).amp_inv = of_property_read_bool(np, cstr!("cirrus,amp-inv"));

    if of_property_read_u32(np, cstr!("cirrus,i2s-sdinloc"), &mut val) >= 0 {
        (*pdata).i2s_sdinloc = val;
    }
    if of_property_read_u32(np, cstr!("cirrus,tdm-rising-edge"), &mut val) >= 0 {
        (*pdata).tdm_rising_edge = val;
    }

    0
}

unsafe extern "C" fn cs35l34_irq_thread(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let cs35l34 = data as *mut cs35l34_private;
    let component = (*cs35l34).component;
    let mut sticky1: c_uint = 0;
    let mut sticky2: c_uint = 0;
    let mut sticky3: c_uint = 0;
    let mut sticky4: c_uint = 0;
    let mut mask1: c_uint = 0;
    let mut mask2: c_uint = 0;
    let mut mask3: c_uint = 0;
    let mut mask4: c_uint = 0;
    let mut current1: c_uint = 0;

    /* ack the irq by reading all status registers */
    regmap_read((*cs35l34).regmap, CS35L34_INT_STATUS_4, &mut sticky4);
    regmap_read((*cs35l34).regmap, CS35L34_INT_STATUS_3, &mut sticky3);
    regmap_read((*cs35l34).regmap, CS35L34_INT_STATUS_2, &mut sticky2);
    regmap_read((*cs35l34).regmap, CS35L34_INT_STATUS_1, &mut sticky1);

    regmap_read((*cs35l34).regmap, CS35L34_INT_MASK_4, &mut mask4);
    regmap_read((*cs35l34).regmap, CS35L34_INT_MASK_3, &mut mask3);
    regmap_read((*cs35l34).regmap, CS35L34_INT_MASK_2, &mut mask2);
    regmap_read((*cs35l34).regmap, CS35L34_INT_MASK_1, &mut mask1);

    if (sticky1 & !mask1) == 0 && (sticky2 & !mask2) == 0 && (sticky3 & !mask3) == 0 && (sticky4 & !mask4) == 0 {
        return IRQ_NONE;
    }

    regmap_read((*cs35l34).regmap, CS35L34_INT_STATUS_1, &mut current1);

    if (sticky1 & CS35L34_CAL_ERR) != 0 {
        dev_err((*component).dev, cstr!("Cal error\n"));
        /* error is no longer asserted; safe to reset */
        if (current1 & CS35L34_CAL_ERR) == 0 {
            dev_dbg((*component).dev, cstr!("Cal error release\n"));
            regmap_update_bits((*cs35l34).regmap, CS35L34_PROT_RELEASE_CTL, CS35L34_CAL_ERR_RLS, 0);
            regmap_update_bits((*cs35l34).regmap, CS35L34_PROT_RELEASE_CTL, CS35L34_CAL_ERR_RLS, CS35L34_CAL_ERR_RLS);
            regmap_update_bits((*cs35l34).regmap, CS35L34_PROT_RELEASE_CTL, CS35L34_CAL_ERR_RLS, 0);
            /* note: amp will re-calibrate on next resume */
        }
    }
    if (sticky1 & CS35L34_ALIVE_ERR) != 0 {
        dev_err((*component).dev, cstr!("Alive error\n"));
    }
    if (sticky1 & CS35L34_AMP_SHORT) != 0 {
        dev_crit((*component).dev, cstr!("Amp short error\n"));
        if (current1 & CS35L34_AMP_SHORT) == 0 {
            dev_dbg((*component).dev, cstr!("Amp short error release\n"));
            regmap_update_bits((*cs35l34).regmap, CS35L34_PROT_RELEASE_CTL, CS35L34_SHORT_RLS, 0);
            regmap_update_bits((*cs35l34).regmap, CS35L34_PROT_RELEASE_CTL, CS35L34_SHORT_RLS, CS35L34_SHORT_RLS);
            regmap_update_bits((*cs35l34).regmap, CS35L34_PROT_RELEASE_CTL, CS35L34_SHORT_RLS, 0);
        }
    }
    if (sticky1 & CS35L34_OTW) != 0 {
        dev_crit((*component).dev, cstr!("Over temperature warning\n"));
        if (current1 & CS35L34_OTW) == 0 {
            dev_dbg((*component).dev, cstr!("Over temperature warning release\n"));
            regmap_update_bits((*cs35l34).regmap, CS35L34_PROT_RELEASE_CTL, CS35L34_OTW_RLS, 0);
            regmap_update_bits((*cs35l34).regmap, CS35L34_PROT_RELEASE_CTL, CS35L34_OTW_RLS, CS35L34_OTW_RLS);
            regmap_update_bits((*cs35l34).regmap, CS35L34_PROT_RELEASE_CTL, CS35L34_OTW_RLS, 0);
        }
    }
    if (sticky1 & CS35L34_OTE) != 0 {
        dev_crit((*component).dev, cstr!("Over temperature error\n"));
        if (current1 & CS35L34_OTE) == 0 {
            dev_dbg((*component).dev, cstr!("Over temperature error release\n"));
            regmap_update_bits((*cs35l34).regmap, CS35L34_PROT_RELEASE_CTL, CS35L34_OTE_RLS, 0);
            regmap_update_bits((*cs35l34).regmap, CS35L34_PROT_RELEASE_CTL, CS35L34_OTE_RLS, CS35L34_OTE_RLS);
            regmap_update_bits((*cs35l34).regmap, CS35L34_PROT_RELEASE_CTL, CS35L34_OTE_RLS, 0);
        }
    }
    if (sticky3 & CS35L34_BST_HIGH) != 0 {
        dev_crit((*component).dev, cstr!("VBST too high error; powering off!\n"));
        regmap_update_bits((*cs35l34).regmap, CS35L34_PWRCTL2, CS35L34_PDN_AMP, CS35L34_PDN_AMP);
        regmap_update_bits((*cs35l34).regmap, CS35L34_PWRCTL1, CS35L34_PDN_ALL, CS35L34_PDN_ALL);
    }
    if (sticky3 & CS35L34_LBST_SHORT) != 0 {
        dev_crit((*component).dev, cstr!("LBST short error; powering off!\n"));
        regmap_update_bits((*cs35l34).regmap, CS35L34_PWRCTL2, CS35L34_PDN_AMP, CS35L34_PDN_AMP);
        regmap_update_bits((*cs35l34).regmap, CS35L34_PWRCTL1, CS35L34_PDN_ALL, CS35L34_PDN_ALL);
    }

    IRQ_HANDLED
}

static cs35l34_core_supplies: [*const c_char; 2] = [cstr!("VA"), cstr!("VP")];

unsafe extern "C" fn cs35l34_i2c_probe(i2c_client: *mut i2c_client) -> c_int {
    let mut cs35l34: *mut cs35l34_private;
    let mut pdata = dev_get_platdata(&mut (*i2c_client).dev as *mut device_with_of_node as *mut device)
        as *mut cs35l34_platform_data;
    let mut i: usize;
    let mut devid: c_int;
    let mut ret: c_int;
    let mut reg: c_uint = 0;
    let dev = &mut (*i2c_client).dev as *mut device_with_of_node as *mut device;

    cs35l34 = devm_kzalloc(dev, core::mem::size_of::<cs35l34_private>(), GFP_KERNEL) as *mut cs35l34_private;
    if cs35l34.is_null() {
        return -ENOMEM;
    }

    i2c_set_clientdata(i2c_client, cs35l34 as *mut c_void);
    (*cs35l34).regmap = devm_regmap_init_i2c(i2c_client, &cs35l34_regmap);
    if IS_ERR((*cs35l34).regmap as *const c_void) {
        ret = PTR_ERR((*cs35l34).regmap as *const c_void);
        dev_err(dev, cstr!("regmap_init() failed: %d\n"), ret);
        return ret;
    }

    (*cs35l34).num_core_supplies = cs35l34_core_supplies.len() as c_int;
    i = 0;
    while i < cs35l34_core_supplies.len() {
        (*cs35l34).core_supplies[i].supply = cs35l34_core_supplies[i];
        i += 1;
    }

    ret = devm_regulator_bulk_get(dev, (*cs35l34).num_core_supplies, (*cs35l34).core_supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, cstr!("Failed to request core supplies %d\n"), ret);
        return ret;
    }

    ret = regulator_bulk_enable((*cs35l34).num_core_supplies, (*cs35l34).core_supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, cstr!("Failed to enable core supplies: %d\n"), ret);
        return ret;
    }

    if !pdata.is_null() {
        (*cs35l34).pdata = *pdata;
    } else {
        pdata = devm_kzalloc(dev, core::mem::size_of::<cs35l34_platform_data>(), GFP_KERNEL)
            as *mut cs35l34_platform_data;
        if pdata.is_null() {
            ret = -ENOMEM;
            goto_err_regulator(cs35l34);
            return ret;
        }

        if !(*i2c_client).dev.of_node.is_null() {
            ret = cs35l34_handle_of_data(i2c_client, pdata);
            if ret != 0 {
                goto_err_regulator(cs35l34);
                return ret;
            }
        }
        (*cs35l34).pdata = *pdata;
    }

    ret = devm_request_threaded_irq(
        dev,
        (*i2c_client).irq,
        core::ptr::null(),
        Some(cs35l34_irq_thread),
        IRQF_ONESHOT | IRQF_TRIGGER_LOW,
        cstr!("cs35l34"),
        cs35l34 as *mut c_void,
    );
    if ret != 0 {
        dev_err(dev, cstr!("Failed to request IRQ: %d\n"), ret);
    } else {
        (*cs35l34).irq_requested = true;
    }

    (*cs35l34).reset_gpio = devm_gpiod_get_optional(dev, cstr!("reset"), GPIOD_OUT_LOW);
    if IS_ERR((*cs35l34).reset_gpio as *const c_void) {
        ret = PTR_ERR((*cs35l34).reset_gpio as *const c_void);
        goto_err_regulator(cs35l34);
        return ret;
    }

    gpiod_set_value_cansleep((*cs35l34).reset_gpio, 1);
    msleep(CS35L34_START_DELAY);

    devid = cirrus_read_device_id((*cs35l34).regmap, CS35L34_DEVID_AB);
    if devid < 0 {
        ret = devid;
        dev_err(dev, cstr!("Failed to read device ID: %d\n"), ret);
        goto_err_reset(cs35l34);
        return ret;
    }

    if devid != CS35L34_CHIP_ID {
        dev_err(dev, cstr!("CS35l34 Device ID (%X). Expected ID %X\n"), devid, CS35L34_CHIP_ID);
        ret = -ENODEV;
        goto_err_reset(cs35l34);
        return ret;
    }

    ret = regmap_read((*cs35l34).regmap, CS35L34_REV_ID, &mut reg);
    if ret < 0 {
        dev_err(dev, cstr!("Get Revision ID failed\n"));
        goto_err_reset(cs35l34);
        return ret;
    }

    dev_info(dev, cstr!("Cirrus Logic CS35l34 (%x), Revision: %02X\n"), devid, reg & 0xFF);

    /* Unmask critical interrupts */
    regmap_update_bits(
        (*cs35l34).regmap,
        CS35L34_INT_MASK_1,
        CS35L34_M_CAL_ERR | CS35L34_M_ALIVE_ERR | CS35L34_M_AMP_SHORT | CS35L34_M_OTW | CS35L34_M_OTE,
        0,
    );
    regmap_update_bits(
        (*cs35l34).regmap,
        CS35L34_INT_MASK_3,
        CS35L34_M_BST_HIGH | CS35L34_M_LBST_SHORT,
        0,
    );

    pm_runtime_set_autosuspend_delay(dev, 100);
    pm_runtime_use_autosuspend(dev);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);

    ret = devm_snd_soc_register_component(dev, &soc_component_dev_cs35l34, &mut cs35l34_dai, 1);
    if ret < 0 {
        dev_err(dev, cstr!("%s: Register component failed\n"), cstr!("cs35l34_i2c_probe"));
        goto_err_reset(cs35l34);
        return ret;
    }

    0
}

unsafe fn goto_err_reset(cs35l34: *mut cs35l34_private) {
    gpiod_set_value_cansleep((*cs35l34).reset_gpio, 0);
    goto_err_regulator(cs35l34);
}

unsafe fn goto_err_regulator(cs35l34: *mut cs35l34_private) {
    regulator_bulk_disable((*cs35l34).num_core_supplies, (*cs35l34).core_supplies.as_mut_ptr());
}

unsafe extern "C" fn cs35l34_i2c_remove(client: *mut i2c_client) {
    let cs35l34 = i2c_get_clientdata(client) as *mut cs35l34_private;

    gpiod_set_value_cansleep((*cs35l34).reset_gpio, 0);
    pm_runtime_disable(&mut (*client).dev as *mut device_with_of_node as *mut device);
    regulator_bulk_disable((*cs35l34).num_core_supplies, (*cs35l34).core_supplies.as_mut_ptr());
}

unsafe extern "C" fn cs35l34_runtime_resume(dev: *mut device) -> c_int {
    let cs35l34 = dev_get_drvdata(dev) as *mut cs35l34_private;
    let mut ret: c_int;

    ret = regulator_bulk_enable((*cs35l34).num_core_supplies, (*cs35l34).core_supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, cstr!("Failed to enable core supplies: %d\n"), ret);
        return ret;
    }

    regcache_cache_only((*cs35l34).regmap, false);
    gpiod_set_value_cansleep((*cs35l34).reset_gpio, 1);
    msleep(CS35L34_START_DELAY);

    ret = regcache_sync((*cs35l34).regmap);
    if ret != 0 {
        dev_err(dev, cstr!("Failed to restore register cache\n"));
        regcache_cache_only((*cs35l34).regmap, true);
        regulator_bulk_disable((*cs35l34).num_core_supplies, (*cs35l34).core_supplies.as_mut_ptr());
        return ret;
    }

    if (*cs35l34).irq_requested {
        enable_irq((*to_i2c_client(dev)).irq);
    }
    0
}

unsafe extern "C" fn cs35l34_runtime_suspend(dev: *mut device) -> c_int {
    let cs35l34 = dev_get_drvdata(dev) as *mut cs35l34_private;

    /* Drain and block the threaded IRQ before cache_only/power-off. */
    if (*cs35l34).irq_requested {
        disable_irq((*to_i2c_client(dev)).irq);
    }

    regcache_cache_only((*cs35l34).regmap, true);
    regcache_mark_dirty((*cs35l34).regmap);
    gpiod_set_value_cansleep((*cs35l34).reset_gpio, 0);
    regulator_bulk_disable((*cs35l34).num_core_supplies, (*cs35l34).core_supplies.as_mut_ptr());

    0
}

static cs35l34_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(cs35l34_runtime_suspend),
    runtime_resume: Some(cs35l34_runtime_resume),
};

static cs35l34_of_match: [of_device_id; 2] = [
    of_device_id { compatible: cstr!("cirrus,cs35l34") },
    of_device_id { compatible: core::ptr::null() },
];
// MODULE_DEVICE_TABLE(of, cs35l34_of_match);

static cs35l34_id: [i2c_device_id; 2] = [
    i2c_device_id { name: cstr!("cs35l34") },
    i2c_device_id { name: core::ptr::null() },
];
// MODULE_DEVICE_TABLE(i2c, cs35l34_id);

static mut cs35l34_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: cstr!("cs35l34"),
        pm: &cs35l34_pm_ops,
        of_match_table: cs35l34_of_match.as_ptr(),
    },
    id_table: cs35l34_id.as_ptr(),
    probe: Some(cs35l34_i2c_probe),
    remove: Some(cs35l34_i2c_remove),
};

unsafe extern "C" fn cs35l34_modinit() -> c_int {
    let ret = i2c_add_driver(&mut cs35l34_i2c_driver);
    if ret != 0 {
        pr_err(cstr!("Failed to register CS35l34 I2C driver: %d\n"), ret);
        return ret;
    }
    0
}
// module_init(cs35l34_modinit);

unsafe extern "C" fn cs35l34_exit() {
    i2c_del_driver(&mut cs35l34_i2c_driver);
}
// module_exit(cs35l34_exit);

// MODULE_DESCRIPTION("ASoC CS35l34 driver");
// MODULE_AUTHOR("Paul Handrigan, Cirrus Logic Inc, <Paul.Handrigan@cirrus.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
