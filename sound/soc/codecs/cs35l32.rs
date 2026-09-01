// SPDX-License-Identifier: GPL-2.0-only
/*
 * cs35l32.c -- CS35L32 ALSA SoC audio driver
 *
 * Copyright 2014 CirrusLogic, Inc.
 *
 * Author: Brian Austin <brian.austin@cirrus.com>
 */

// C includes translated as external dependencies:
// linux/module.h, linux/moduleparam.h, linux/kernel.h, linux/init.h,
// linux/delay.h, linux/i2c.h, linux/regmap.h, linux/slab.h,
// linux/platform_device.h, linux/regulator/consumer.h,
// linux/gpio/consumer.h, linux/of.h, sound/core.h, sound/pcm.h,
// sound/pcm_params.h, sound/soc.h, sound/soc-dapm.h, sound/initval.h,
// sound/tlv.h, dt-bindings/sound/cs35l32.h, cs35l32.h,
// cirrus_legacy.h.

use core::ffi::{c_char, c_int, c_uint, c_void};

const CS35L32_NUM_SUPPLIES: usize = 2;

type bool_t = bool;

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
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    set_tristate: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    name: *const c_char,
    id: c_int,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
    symmetric_rate: c_uint,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    set_sysclk: Option<
        unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_int) -> c_int,
    >,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    idle_bias_on: c_uint,
    use_pmdown_time: c_uint,
    endianness: c_uint,
}

#[repr(C)]
pub struct regulator_bulk_data {
    supply: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_default {
    reg: c_uint,
    def: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_sequence {
    reg: c_uint,
    def: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    reg_bits: c_uint,
    val_bits: c_uint,
    max_register: c_uint,
    reg_defaults: *const reg_default,
    num_reg_defaults: c_uint,
    volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_t>,
    readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_t>,
    precious_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_t>,
    cache_type: c_uint,
    use_single_read: bool_t,
    use_single_write: bool_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l32_platform_data {
    sdout_share: c_uint,
    boost_mng: c_uint,
    sdout_datacfg: c_uint,
    batt_thresh: c_uint,
    batt_recov: c_uint,
}

#[repr(C)]
pub struct cs35l32_private {
    regmap: *mut regmap,
    component: *mut snd_soc_component,
    supplies: [regulator_bulk_data; CS35L32_NUM_SUPPLIES],
    pdata: cs35l32_platform_data,
    reset_gpio: *mut gpio_desc,
}

#[repr(C)]
pub struct i2c_client_dev {
    of_node: *mut device_node,
}

#[repr(C)]
pub struct i2c_client {
    dev: i2c_client_dev,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
pub struct i2c_device_id {
    name: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    name: *const c_char,
    pm: *const dev_pm_ops,
    of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    driver: device_driver,
    id_table: *const i2c_device_id,
    probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
}

unsafe extern "C" {
    static CS35L32_DEVID_AB: c_uint;
    static CS35L32_AUDIO_LED_MNGR: c_uint;
    static CS35L32_ADSP_CTL: c_uint;
    static CS35L32_FLASH_INHIBIT: c_uint;
    static CS35L32_REV_ID: c_uint;
    static CS35L32_INT_STATUS_1: c_uint;
    static CS35L32_LED_STATUS: c_uint;
    static CS35L32_PWRCTL1: c_uint;
    static CS35L32_PWRCTL2: c_uint;
    static CS35L32_CLASSD_CTL: c_uint;
    static CS35L32_CLK_CTL: c_uint;
    static CS35L32_BATT_THRESHOLD: c_uint;
    static CS35L32_MAX_REGISTER: c_uint;
    static CS35L32_ADSP_MASTER_MASK: c_uint;
    static CS35L32_SDOUT_3ST: c_uint;
    static CS35L32_RATES: c_uint;
    static CS35L32_FORMATS: u64;
    static CS35L32_MCLK_RATIO: c_uint;
    static CS35L32_MCLK_DIV2_MASK: c_uint;
    static CS35L32_MCLK_RATIO_MASK: c_uint;
    static CS35L32_BOOST_MGR_AUTO: c_uint;
    static CS35L32_BOOST_MGR_AUTO_AUDIO: c_uint;
    static CS35L32_BOOST_MGR_BYPASS: c_uint;
    static CS35L32_BOOST_MGR_FIXED: c_uint;
    static CS35L32_DATA_CFG_LR_VP: c_uint;
    static CS35L32_DATA_CFG_LR_STAT: c_uint;
    static CS35L32_DATA_CFG_LR: c_uint;
    static CS35L32_DATA_CFG_LR_VPSTAT: c_uint;
    static CS35L32_BATT_THRESH_3_1V: c_uint;
    static CS35L32_BATT_THRESH_3_2V: c_uint;
    static CS35L32_BATT_THRESH_3_3V: c_uint;
    static CS35L32_BATT_THRESH_3_4V: c_uint;
    static CS35L32_BATT_RECOV_3_1V: c_uint;
    static CS35L32_BATT_RECOV_3_2V: c_uint;
    static CS35L32_BATT_RECOV_3_3V: c_uint;
    static CS35L32_BATT_RECOV_3_4V: c_uint;
    static CS35L32_BATT_RECOV_3_5V: c_uint;
    static CS35L32_BATT_RECOV_3_6V: c_uint;
    static CS35L32_CHIP_ID: c_int;
    static CS35L32_BOOST_MASK: c_uint;
    static CS35L32_ADSP_SHARE_MASK: c_uint;
    static CS35L32_ADSP_DATACFG_MASK: c_uint;
    static CS35L32_BATT_REC_MASK: c_uint;
    static CS35L32_BATT_THRESH_MASK: c_uint;
    static CS35L32_PDN_AMP: c_uint;
    static SND_SOC_DAIFMT_MASTER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static REGCACHE_MAPLE: c_uint;
    static GPIOD_OUT_LOW: c_uint;
    static GFP_KERNEL: c_uint;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static ENODEV: c_int;

    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut c_uint) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_info(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut c_void;
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool_t;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_regulator_bulk_get(dev: *mut device, num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_enable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn cirrus_read_device_id(map: *mut regmap, reg: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_register_patch(map: *mut regmap, regs: *const reg_sequence, num_regs: c_int) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool_t);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn pm_ptr(ops: *const dev_pm_ops) -> *const dev_pm_ops;
}

const VA: &[u8] = b"VA\0";
const VP: &[u8] = b"VP\0";

static cs35l32_supply_names: [*const c_char; CS35L32_NUM_SUPPLIES] = [
    VA.as_ptr() as *const c_char,
    VP.as_ptr() as *const c_char,
];

static cs35l32_reg_defaults: [reg_default; 18] = [
    reg_default { reg: 0x06, def: 0x04 }, /* Power Ctl 1 */
    reg_default { reg: 0x07, def: 0xE8 }, /* Power Ctl 2 */
    reg_default { reg: 0x08, def: 0x40 }, /* Clock Ctl */
    reg_default { reg: 0x09, def: 0x20 }, /* Low Battery Threshold */
    reg_default { reg: 0x0A, def: 0x00 }, /* Voltage Monitor [RO] */
    reg_default { reg: 0x0B, def: 0x40 }, /* Conv Peak Curr Protection CTL */
    reg_default { reg: 0x0C, def: 0x07 }, /* IMON Scaling */
    reg_default { reg: 0x0D, def: 0x03 }, /* Audio/LED Pwr Manager */
    reg_default { reg: 0x0F, def: 0x20 }, /* Serial Port Control */
    reg_default { reg: 0x10, def: 0x14 }, /* Class D Amp CTL */
    reg_default { reg: 0x11, def: 0x00 }, /* Protection Release CTL */
    reg_default { reg: 0x12, def: 0xFF }, /* Interrupt Mask 1 */
    reg_default { reg: 0x13, def: 0xFF }, /* Interrupt Mask 2 */
    reg_default { reg: 0x14, def: 0xFF }, /* Interrupt Mask 3 */
    reg_default { reg: 0x19, def: 0x00 }, /* LED Flash Mode Current */
    reg_default { reg: 0x1A, def: 0x00 }, /* LED Movie Mode Current */
    reg_default { reg: 0x1B, def: 0x20 }, /* LED Flash Timer */
    reg_default { reg: 0x1C, def: 0x00 }, /* LED Flash Inhibit Current */
];

unsafe extern "C" fn cs35l32_readable_register(_dev: *mut device, reg: c_uint) -> bool_t {
    unsafe {
        if (reg >= CS35L32_DEVID_AB && reg <= CS35L32_AUDIO_LED_MNGR)
            || (reg >= CS35L32_ADSP_CTL && reg <= CS35L32_FLASH_INHIBIT)
        {
            true
        } else {
            false
        }
    }
}

unsafe extern "C" fn cs35l32_volatile_register(_dev: *mut device, reg: c_uint) -> bool_t {
    unsafe {
        if (reg >= CS35L32_DEVID_AB && reg <= CS35L32_REV_ID)
            || (reg >= CS35L32_INT_STATUS_1 && reg <= CS35L32_LED_STATUS)
        {
            true
        } else {
            false
        }
    }
}

unsafe extern "C" fn cs35l32_precious_register(_dev: *mut device, reg: c_uint) -> bool_t {
    unsafe {
        if reg >= CS35L32_INT_STATUS_1 && reg <= CS35L32_LED_STATUS {
            true
        } else {
            false
        }
    }
}

// static DECLARE_TLV_DB_SCALE(classd_ctl_tlv, 900, 300, 0);
static classd_ctl_tlv: [c_uint; 4] = [0, 900, 300, 0];

// SOC_DAPM_SINGLE("Switch", CS35L32_PWRCTL2, 6, 1, 1)
static imon_ctl: snd_kcontrol_new = snd_kcontrol_new { _private: [] };

// SOC_DAPM_SINGLE("Switch", CS35L32_PWRCTL2, 7, 1, 1)
static vmon_ctl: snd_kcontrol_new = snd_kcontrol_new { _private: [] };

// SOC_DAPM_SINGLE("Switch", CS35L32_PWRCTL2, 5, 1, 1)
static vpmon_ctl: snd_kcontrol_new = snd_kcontrol_new { _private: [] };

// SOC_SINGLE_TLV/SOC_SINGLE controls.
static cs35l32_snd_controls: [snd_kcontrol_new; 3] = [
    snd_kcontrol_new { _private: [] }, /* "Speaker Volume" */
    snd_kcontrol_new { _private: [] }, /* "Zero Cross Switch" */
    snd_kcontrol_new { _private: [] }, /* "Gain Manager Switch" */
];

// SND_SOC_DAPM_* widgets.
static cs35l32_dapm_widgets: [snd_soc_dapm_widget; 9] = [
    snd_soc_dapm_widget { _private: [] }, /* BOOST */
    snd_soc_dapm_widget { _private: [] }, /* Speaker */
    snd_soc_dapm_widget { _private: [] }, /* SDOUT */
    snd_soc_dapm_widget { _private: [] }, /* VP */
    snd_soc_dapm_widget { _private: [] }, /* ISENSE */
    snd_soc_dapm_widget { _private: [] }, /* VSENSE */
    snd_soc_dapm_widget { _private: [] }, /* VMON ADC */
    snd_soc_dapm_widget { _private: [] }, /* IMON ADC */
    snd_soc_dapm_widget { _private: [] }, /* VPMON ADC */
];

// DAPM routes:
// {"Speaker", NULL, "BOOST"}
// {"VMON ADC", NULL, "VSENSE"}
// {"IMON ADC", NULL, "ISENSE"}
// {"VPMON ADC", NULL, "VP"}
// {"SDOUT", "Switch", "VMON ADC"}
// {"SDOUT", "Switch", "IMON ADC"}
// {"SDOUT", "Switch", "VPMON ADC"}
// {"Capture", NULL, "SDOUT"}
static cs35l32_audio_map: [snd_soc_dapm_route; 8] = [
    snd_soc_dapm_route { _private: [] },
    snd_soc_dapm_route { _private: [] },
    snd_soc_dapm_route { _private: [] },
    snd_soc_dapm_route { _private: [] },
    snd_soc_dapm_route { _private: [] },
    snd_soc_dapm_route { _private: [] },
    snd_soc_dapm_route { _private: [] },
    snd_soc_dapm_route { _private: [] },
];

unsafe extern "C" fn cs35l32_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component: *mut snd_soc_component = unsafe { (*codec_dai).component };

    unsafe {
        match fmt & SND_SOC_DAIFMT_MASTER_MASK {
            x if x == SND_SOC_DAIFMT_CBP_CFP => {
                snd_soc_component_update_bits(
                    component,
                    CS35L32_ADSP_CTL,
                    CS35L32_ADSP_MASTER_MASK,
                    CS35L32_ADSP_MASTER_MASK,
                );
            }
            x if x == SND_SOC_DAIFMT_CBC_CFC => {
                snd_soc_component_update_bits(
                    component,
                    CS35L32_ADSP_CTL,
                    CS35L32_ADSP_MASTER_MASK,
                    0,
                );
            }
            _ => return -EINVAL,
        }
    }

    0
}

unsafe extern "C" fn cs35l32_set_tristate(dai: *mut snd_soc_dai, tristate: c_int) -> c_int {
    let component: *mut snd_soc_component = unsafe { (*dai).component };

    unsafe {
        snd_soc_component_update_bits(
            component,
            CS35L32_PWRCTL2,
            CS35L32_SDOUT_3ST,
            (tristate << 3) as c_uint,
        )
    }
}

static cs35l32_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(cs35l32_set_dai_fmt),
    set_tristate: Some(cs35l32_set_tristate),
};

const CS35L32_MONITOR: &[u8] = b"cs35l32-monitor\0";
const CAPTURE: &[u8] = b"Capture\0";

static mut cs35l32_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: CS35L32_MONITOR.as_ptr() as *const c_char,
    id: 0,
    capture: snd_soc_pcm_stream {
        stream_name: CAPTURE.as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 2,
        rates: unsafe { CS35L32_RATES },
        formats: unsafe { CS35L32_FORMATS },
    },
    ops: &cs35l32_ops,
    symmetric_rate: 1,
}];

unsafe extern "C" fn cs35l32_component_set_sysclk(
    component: *mut snd_soc_component,
    _clk_id: c_int,
    _source: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let val: c_uint;

    unsafe {
        match freq {
            6000000 => {
                val = CS35L32_MCLK_RATIO;
            }
            12000000 => {
                val = CS35L32_MCLK_DIV2_MASK | CS35L32_MCLK_RATIO;
            }
            6144000 => {
                val = 0;
            }
            12288000 => {
                val = CS35L32_MCLK_DIV2_MASK;
            }
            _ => return -EINVAL,
        }

        snd_soc_component_update_bits(
            component,
            CS35L32_CLK_CTL,
            CS35L32_MCLK_DIV2_MASK | CS35L32_MCLK_RATIO_MASK,
            val,
        )
    }
}

static soc_component_dev_cs35l32: snd_soc_component_driver = snd_soc_component_driver {
    set_sysclk: Some(cs35l32_component_set_sysclk),
    controls: cs35l32_snd_controls.as_ptr(),
    num_controls: cs35l32_snd_controls.len() as c_uint,
    dapm_widgets: cs35l32_dapm_widgets.as_ptr(),
    num_dapm_widgets: cs35l32_dapm_widgets.len() as c_uint,
    dapm_routes: cs35l32_audio_map.as_ptr(),
    num_dapm_routes: cs35l32_audio_map.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

/* Current and threshold powerup sequence Pg37 in datasheet */
static cs35l32_monitor_patch: [reg_sequence; 7] = [
    reg_sequence { reg: 0x00, def: 0x99 },
    reg_sequence { reg: 0x48, def: 0x17 },
    reg_sequence { reg: 0x49, def: 0x56 },
    reg_sequence { reg: 0x43, def: 0x01 },
    reg_sequence { reg: 0x3B, def: 0x62 },
    reg_sequence { reg: 0x3C, def: 0x80 },
    reg_sequence { reg: 0x00, def: 0x00 },
];

static cs35l32_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: unsafe { CS35L32_MAX_REGISTER },
    reg_defaults: cs35l32_reg_defaults.as_ptr(),
    num_reg_defaults: cs35l32_reg_defaults.len() as c_uint,
    volatile_reg: Some(cs35l32_volatile_register),
    readable_reg: Some(cs35l32_readable_register),
    precious_reg: Some(cs35l32_precious_register),
    cache_type: unsafe { REGCACHE_MAPLE },
    use_single_read: true,
    use_single_write: true,
};

unsafe extern "C" fn cs35l32_handle_of_data(
    i2c_client: *mut i2c_client,
    pdata: *mut cs35l32_platform_data,
) -> c_int {
    let np: *mut device_node = unsafe { (*i2c_client).dev.of_node };
    let mut val: c_uint = 0;

    unsafe {
        if of_property_read_u32(np, c"cirrus,sdout-share".as_ptr(), &mut val) >= 0 {
            (*pdata).sdout_share = val;
        }

        if of_property_read_u32(np, c"cirrus,boost-manager".as_ptr(), &mut val) != 0 {
            val = !0u32;
        }

        if val == CS35L32_BOOST_MGR_AUTO
            || val == CS35L32_BOOST_MGR_AUTO_AUDIO
            || val == CS35L32_BOOST_MGR_BYPASS
            || val == CS35L32_BOOST_MGR_FIXED
        {
            (*pdata).boost_mng = val;
        } else {
            dev_err(
                &mut (*i2c_client).dev as *mut i2c_client_dev as *mut device,
                c"Wrong cirrus,boost-manager DT value %d\n".as_ptr(),
                val,
            );
            (*pdata).boost_mng = CS35L32_BOOST_MGR_BYPASS;
        }

        if of_property_read_u32(np, c"cirrus,sdout-datacfg".as_ptr(), &mut val) != 0 {
            val = !0u32;
        }
        if val == CS35L32_DATA_CFG_LR_VP
            || val == CS35L32_DATA_CFG_LR_STAT
            || val == CS35L32_DATA_CFG_LR
            || val == CS35L32_DATA_CFG_LR_VPSTAT
        {
            (*pdata).sdout_datacfg = val;
        } else {
            dev_err(
                &mut (*i2c_client).dev as *mut i2c_client_dev as *mut device,
                c"Wrong cirrus,sdout-datacfg DT value %d\n".as_ptr(),
                val,
            );
            (*pdata).sdout_datacfg = CS35L32_DATA_CFG_LR;
        }

        if of_property_read_u32(np, c"cirrus,battery-threshold".as_ptr(), &mut val) != 0 {
            val = !0u32;
        }
        if val == CS35L32_BATT_THRESH_3_1V
            || val == CS35L32_BATT_THRESH_3_2V
            || val == CS35L32_BATT_THRESH_3_3V
            || val == CS35L32_BATT_THRESH_3_4V
        {
            (*pdata).batt_thresh = val;
        } else {
            dev_err(
                &mut (*i2c_client).dev as *mut i2c_client_dev as *mut device,
                c"Wrong cirrus,battery-threshold DT value %d\n".as_ptr(),
                val,
            );
            (*pdata).batt_thresh = CS35L32_BATT_THRESH_3_3V;
        }

        if of_property_read_u32(np, c"cirrus,battery-recovery".as_ptr(), &mut val) != 0 {
            val = !0u32;
        }
        if val == CS35L32_BATT_RECOV_3_1V
            || val == CS35L32_BATT_RECOV_3_2V
            || val == CS35L32_BATT_RECOV_3_3V
            || val == CS35L32_BATT_RECOV_3_4V
            || val == CS35L32_BATT_RECOV_3_5V
            || val == CS35L32_BATT_RECOV_3_6V
        {
            (*pdata).batt_recov = val;
        } else {
            dev_err(
                &mut (*i2c_client).dev as *mut i2c_client_dev as *mut device,
                c"Wrong cirrus,battery-recovery DT value %d\n".as_ptr(),
                val,
            );
            (*pdata).batt_recov = CS35L32_BATT_RECOV_3_4V;
        }
    }

    0
}

unsafe extern "C" fn cs35l32_i2c_probe(i2c_client: *mut i2c_client) -> c_int {
    let mut cs35l32: *mut cs35l32_private;
    let mut pdata: *mut cs35l32_platform_data =
        unsafe { dev_get_platdata(&mut (*i2c_client).dev as *mut i2c_client_dev as *mut device) as *mut cs35l32_platform_data };
    let mut ret: c_int;
    let mut i: c_int;
    let mut devid: c_int;
    let mut reg: c_uint = 0;

    unsafe {
        cs35l32 = devm_kzalloc(
            &mut (*i2c_client).dev as *mut i2c_client_dev as *mut device,
            core::mem::size_of::<cs35l32_private>(),
            GFP_KERNEL,
        ) as *mut cs35l32_private;
        if cs35l32.is_null() {
            return -ENOMEM;
        }

        i2c_set_clientdata(i2c_client, cs35l32 as *mut c_void);

        (*cs35l32).regmap = devm_regmap_init_i2c(i2c_client, &cs35l32_regmap);
        if IS_ERR((*cs35l32).regmap as *const c_void) {
            ret = PTR_ERR((*cs35l32).regmap as *const c_void);
            dev_err(
                &mut (*i2c_client).dev as *mut i2c_client_dev as *mut device,
                c"regmap_init() failed: %d\n".as_ptr(),
                ret,
            );
            return ret;
        }

        if !pdata.is_null() {
            (*cs35l32).pdata = *pdata;
        } else {
            pdata = devm_kzalloc(
                &mut (*i2c_client).dev as *mut i2c_client_dev as *mut device,
                core::mem::size_of::<cs35l32_platform_data>(),
                GFP_KERNEL,
            ) as *mut cs35l32_platform_data;
            if pdata.is_null() {
                return -ENOMEM;
            }

            if !(*i2c_client).dev.of_node.is_null() {
                ret = cs35l32_handle_of_data(i2c_client, &mut (*cs35l32).pdata);
                if ret != 0 {
                    return ret;
                }
            }
        }

        i = 0;
        while (i as usize) < (*cs35l32).supplies.len() {
            (*cs35l32).supplies[i as usize].supply = cs35l32_supply_names[i as usize];
            i += 1;
        }

        ret = devm_regulator_bulk_get(
            &mut (*i2c_client).dev as *mut i2c_client_dev as *mut device,
            (*cs35l32).supplies.len() as c_int,
            (*cs35l32).supplies.as_mut_ptr(),
        );
        if ret != 0 {
            dev_err(
                &mut (*i2c_client).dev as *mut i2c_client_dev as *mut device,
                c"Failed to request supplies: %d\n".as_ptr(),
                ret,
            );
            return ret;
        }

        ret = regulator_bulk_enable(
            (*cs35l32).supplies.len() as c_int,
            (*cs35l32).supplies.as_mut_ptr(),
        );
        if ret != 0 {
            dev_err(
                &mut (*i2c_client).dev as *mut i2c_client_dev as *mut device,
                c"Failed to enable supplies: %d\n".as_ptr(),
                ret,
            );
            return ret;
        }

        /* Reset the Device */
        (*cs35l32).reset_gpio = devm_gpiod_get_optional(
            &mut (*i2c_client).dev as *mut i2c_client_dev as *mut device,
            c"reset".as_ptr(),
            GPIOD_OUT_LOW,
        );
        if IS_ERR((*cs35l32).reset_gpio as *const c_void) {
            ret = PTR_ERR((*cs35l32).reset_gpio as *const c_void);
            regulator_bulk_disable(
                (*cs35l32).supplies.len() as c_int,
                (*cs35l32).supplies.as_mut_ptr(),
            );
            return ret;
        }

        gpiod_set_value_cansleep((*cs35l32).reset_gpio, 1);

        /* initialize codec */
        devid = cirrus_read_device_id((*cs35l32).regmap, CS35L32_DEVID_AB);
        if devid < 0 {
            ret = devid;
            dev_err(
                &mut (*i2c_client).dev as *mut i2c_client_dev as *mut device,
                c"Failed to read device ID: %d\n".as_ptr(),
                ret,
            );
            gpiod_set_value_cansleep((*cs35l32).reset_gpio, 0);
            regulator_bulk_disable(
                (*cs35l32).supplies.len() as c_int,
                (*cs35l32).supplies.as_mut_ptr(),
            );
            return ret;
        }

        if devid != CS35L32_CHIP_ID {
            ret = -ENODEV;
            dev_err(
                &mut (*i2c_client).dev as *mut i2c_client_dev as *mut device,
                c"CS35L32 Device ID (%X). Expected %X\n".as_ptr(),
                devid,
                CS35L32_CHIP_ID,
            );
            gpiod_set_value_cansleep((*cs35l32).reset_gpio, 0);
            regulator_bulk_disable(
                (*cs35l32).supplies.len() as c_int,
                (*cs35l32).supplies.as_mut_ptr(),
            );
            return ret;
        }

        ret = regmap_read((*cs35l32).regmap, CS35L32_REV_ID, &mut reg);
        if ret < 0 {
            dev_err(
                &mut (*i2c_client).dev as *mut i2c_client_dev as *mut device,
                c"Get Revision ID failed\n".as_ptr(),
            );
            gpiod_set_value_cansleep((*cs35l32).reset_gpio, 0);
            regulator_bulk_disable(
                (*cs35l32).supplies.len() as c_int,
                (*cs35l32).supplies.as_mut_ptr(),
            );
            return ret;
        }

        ret = regmap_register_patch(
            (*cs35l32).regmap,
            cs35l32_monitor_patch.as_ptr(),
            cs35l32_monitor_patch.len() as c_int,
        );
        if ret < 0 {
            dev_err(
                &mut (*i2c_client).dev as *mut i2c_client_dev as *mut device,
                c"Failed to apply errata patch\n".as_ptr(),
            );
            gpiod_set_value_cansleep((*cs35l32).reset_gpio, 0);
            regulator_bulk_disable(
                (*cs35l32).supplies.len() as c_int,
                (*cs35l32).supplies.as_mut_ptr(),
            );
            return ret;
        }

        dev_info(
            &mut (*i2c_client).dev as *mut i2c_client_dev as *mut device,
            c"Cirrus Logic CS35L32, Revision: %02X\n".as_ptr(),
            reg & 0xFF,
        );

        /* Setup VBOOST Management */
        if (*cs35l32).pdata.boost_mng != 0 {
            regmap_update_bits(
                (*cs35l32).regmap,
                CS35L32_AUDIO_LED_MNGR,
                CS35L32_BOOST_MASK,
                (*cs35l32).pdata.boost_mng,
            );
        }

        /* Setup ADSP Format Config */
        if (*cs35l32).pdata.sdout_share != 0 {
            regmap_update_bits(
                (*cs35l32).regmap,
                CS35L32_ADSP_CTL,
                CS35L32_ADSP_SHARE_MASK,
                (*cs35l32).pdata.sdout_share << 3,
            );
        }

        /* Setup ADSP Data Configuration */
        if (*cs35l32).pdata.sdout_datacfg != 0 {
            regmap_update_bits(
                (*cs35l32).regmap,
                CS35L32_ADSP_CTL,
                CS35L32_ADSP_DATACFG_MASK,
                (*cs35l32).pdata.sdout_datacfg << 4,
            );
        }

        /* Setup Low Battery Recovery  */
        if (*cs35l32).pdata.batt_recov != 0 {
            regmap_update_bits(
                (*cs35l32).regmap,
                CS35L32_BATT_THRESHOLD,
                CS35L32_BATT_REC_MASK,
                (*cs35l32).pdata.batt_recov << 1,
            );
        }

        /* Setup Low Battery Threshold */
        if (*cs35l32).pdata.batt_thresh != 0 {
            regmap_update_bits(
                (*cs35l32).regmap,
                CS35L32_BATT_THRESHOLD,
                CS35L32_BATT_THRESH_MASK,
                (*cs35l32).pdata.batt_thresh << 4,
            );
        }

        /* Power down the AMP */
        regmap_update_bits(
            (*cs35l32).regmap,
            CS35L32_PWRCTL1,
            CS35L32_PDN_AMP,
            CS35L32_PDN_AMP,
        );

        /* Clear MCLK Error Bit since we don't have the clock yet */
        regmap_read((*cs35l32).regmap, CS35L32_INT_STATUS_1, &mut reg);

        ret = devm_snd_soc_register_component(
            &mut (*i2c_client).dev as *mut i2c_client_dev as *mut device,
            &soc_component_dev_cs35l32,
            cs35l32_dai.as_mut_ptr(),
            cs35l32_dai.len() as c_int,
        );
        if ret < 0 {
            gpiod_set_value_cansleep((*cs35l32).reset_gpio, 0);
            regulator_bulk_disable(
                (*cs35l32).supplies.len() as c_int,
                (*cs35l32).supplies.as_mut_ptr(),
            );
            return ret;
        }

        0
    }
}

unsafe extern "C" fn cs35l32_i2c_remove(i2c_client: *mut i2c_client) {
    let cs35l32: *mut cs35l32_private = unsafe { i2c_get_clientdata(i2c_client) as *mut cs35l32_private };

    unsafe {
        /* Hold down reset */
        gpiod_set_value_cansleep((*cs35l32).reset_gpio, 0);
    }
}

unsafe extern "C" fn cs35l32_runtime_suspend(dev: *mut device) -> c_int {
    let cs35l32: *mut cs35l32_private = unsafe { dev_get_drvdata(dev) as *mut cs35l32_private };

    unsafe {
        regcache_cache_only((*cs35l32).regmap, true);
        regcache_mark_dirty((*cs35l32).regmap);

        /* Hold down reset */
        gpiod_set_value_cansleep((*cs35l32).reset_gpio, 0);

        /* remove power */
        regulator_bulk_disable(
            (*cs35l32).supplies.len() as c_int,
            (*cs35l32).supplies.as_mut_ptr(),
        );
    }

    0
}

unsafe extern "C" fn cs35l32_runtime_resume(dev: *mut device) -> c_int {
    let cs35l32: *mut cs35l32_private = unsafe { dev_get_drvdata(dev) as *mut cs35l32_private };
    let mut ret: c_int;

    unsafe {
        /* Enable power */
        ret = regulator_bulk_enable(
            (*cs35l32).supplies.len() as c_int,
            (*cs35l32).supplies.as_mut_ptr(),
        );
        if ret != 0 {
            dev_err(dev, c"Failed to enable supplies: %d\n".as_ptr(), ret);
            return ret;
        }

        gpiod_set_value_cansleep((*cs35l32).reset_gpio, 1);

        regcache_cache_only((*cs35l32).regmap, false);
        ret = regcache_sync((*cs35l32).regmap);
        if ret != 0 {
            regcache_cache_only((*cs35l32).regmap, true);
            regcache_mark_dirty((*cs35l32).regmap);
            gpiod_set_value_cansleep((*cs35l32).reset_gpio, 0);
            regulator_bulk_disable(
                (*cs35l32).supplies.len() as c_int,
                (*cs35l32).supplies.as_mut_ptr(),
            );
            return ret;
        }
    }

    0
}

// static const struct dev_pm_ops cs35l32_runtime_pm = {
//     RUNTIME_PM_OPS(cs35l32_runtime_suspend, cs35l32_runtime_resume, NULL)
// };
static cs35l32_runtime_pm: dev_pm_ops = dev_pm_ops { _private: [] };

const CIRRUS_CS35L32: &[u8] = b"cirrus,cs35l32\0";
static cs35l32_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: CIRRUS_CS35L32.as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, cs35l32_of_match);

const CS35L32_NAME: &[u8] = b"cs35l32\0";
static cs35l32_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: CS35L32_NAME.as_ptr() as *const c_char,
    },
    i2c_device_id {
        name: core::ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(i2c, cs35l32_id);

static mut cs35l32_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: CS35L32_NAME.as_ptr() as *const c_char,
        pm: unsafe { pm_ptr(&cs35l32_runtime_pm) },
        of_match_table: cs35l32_of_match.as_ptr(),
    },
    id_table: cs35l32_id.as_ptr(),
    probe: Some(cs35l32_i2c_probe),
    remove: Some(cs35l32_i2c_remove),
};

// module_i2c_driver(cs35l32_i2c_driver);
// MODULE_DESCRIPTION("ASoC CS35L32 driver");
// MODULE_AUTHOR("Brian Austin, Cirrus Logic Inc, <brian.austin@cirrus.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
