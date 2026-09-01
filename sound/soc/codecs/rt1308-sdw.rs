// SPDX-License-Identifier: GPL-2.0
//
// rt1308-sdw.c -- rt1308 ALSA SoC audio driver
//
// Copyright(c) 2019 Realtek Semiconductor Corp.
//
//
// Dependencies from the original C file:
// linux/delay.h, linux/device.h, linux/pm_runtime.h, linux/soundwire/sdw.h,
// linux/soundwire/sdw_type.h, linux/soundwire/sdw_registers.h, linux/module.h,
// linux/regmap.h, sound/core.h, sound/pcm.h, sound/pcm_params.h, sound/sdw.h,
// sound/soc.h, sound/soc-dapm.h, sound/initval.h, rt1308.h, rt1308-sdw.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u32 = u32;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct reg_default {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_uint,
    pub use_single_read: bool_,
    pub use_single_write: bool_,
}

#[repr(C)]
pub struct sdw_bus_params {
    pub curr_dr_freq: c_uint,
}

#[repr(C)]
pub struct sdw_dpn_prop {
    pub num: u32,
    pub type_: c_uint,
    pub simple_ch_prep_sm: bool_,
    pub ch_prep_timeout: c_uint,
}

#[repr(C)]
pub struct sdw_slave_prop {
    pub scp_int1_mask: c_uint,
    pub quirks: c_uint,
    pub paging_support: bool_,
    pub source_ports: c_ulong,
    pub sink_ports: c_ulong,
    pub sink_dpn_prop: *mut sdw_dpn_prop,
    pub clk_stop_timeout: c_uint,
}

#[repr(C)]
pub struct sdw_slave {
    pub dev: device,
    pub prop: sdw_slave_prop,
    pub bus: *mut sdw_bus,
}

#[repr(C)]
pub struct sdw_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_slave_intr_status {
    pub control_port: c_uint,
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
pub struct snd_soc_component {
    pub dev: *mut device,
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

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub dev: *mut device,
    pub name: *const c_char,
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
pub struct sdw_stream_config {
    pub ch_count: c_uint,
}

#[repr(C)]
pub struct sdw_port_config {
    pub num: c_uint,
    pub ch_mask: c_uint,
}

#[repr(C)]
pub struct sdw_stream_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_slave_ops {
    pub read_prop: Option<unsafe extern "C" fn(*mut sdw_slave) -> c_int>,
    pub interrupt_callback:
        Option<unsafe extern "C" fn(*mut sdw_slave, *mut sdw_slave_intr_status) -> c_int>,
    pub update_status: Option<unsafe extern "C" fn(*mut sdw_slave, sdw_slave_status) -> c_int>,
    pub bus_config: Option<unsafe extern "C" fn(*mut sdw_slave, *mut sdw_bus_params) -> c_int>,
}

pub type sdw_slave_status = c_uint;

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_desc,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub set_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, *mut c_void, c_int) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
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
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct sdw_device_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct sdw_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut sdw_slave, *const sdw_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut sdw_slave)>,
    pub ops: *const sdw_slave_ops,
    pub id_table: *const sdw_device_id,
}

#[repr(C)]
pub struct rt1308_sdw_priv {
    pub regmap: *mut regmap,
    pub params: sdw_bus_params,
    pub sdw_slave: *mut sdw_slave,
    pub hw_ver: c_uint,
    pub bq_params_cnt: c_uint,
    pub bq_params: *mut u8,
    pub hw_init: bool_,
    pub first_hw_init: bool_,
    pub component: *mut snd_soc_component,
    pub rx_mask: c_uint,
    pub slots: c_int,
}

unsafe extern "C" {
    static rt1308_reg_defaults: [reg_default; 0];

    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn device_property_read_u32(dev: *mut device, name: *const c_char, val: *mut c_uint) -> c_int;
    fn device_property_read_u8_array(
        dev: *mut device,
        name: *const c_char,
        val: *mut u8,
        nval: c_uint,
    ) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regcache_cache_bypass(map: *mut regmap, enable: bool_);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync_region(map: *mut regmap, min: c_uint, max: c_uint) -> c_int;
    fn msleep(msecs: c_uint);
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn pm_runtime_set_active(dev: *mut device) -> c_int;
    fn pm_runtime_get_noresume(dev: *mut device);
    fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn pm_runtime_resume(dev: *mut device) -> c_int;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_dai_dma_data_set(dai: *mut snd_soc_dai, direction: c_int, data: *mut c_void);
    fn snd_soc_dai_set_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
        data: *mut c_void,
    );
    fn snd_soc_dai_get_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
    ) -> *mut sdw_stream_runtime;
    fn snd_sdw_params_to_config(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        stream: *mut sdw_stream_config,
        port: *mut sdw_port_config,
    );
    fn sdw_stream_add_slave(
        slave: *mut sdw_slave,
        stream: *mut sdw_stream_config,
        port: *mut sdw_port_config,
        num_ports: c_uint,
        runtime: *mut sdw_stream_runtime,
    ) -> c_int;
    fn sdw_stream_remove_slave(slave: *mut sdw_slave, runtime: *mut sdw_stream_runtime);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_uint,
    ) -> c_int;
    fn devm_regmap_init_sdw(slave: *mut sdw_slave, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_to_sdw_dev(dev: *mut device) -> *mut sdw_slave;
    fn sdw_slave_wait_for_init(slave: *mut sdw_slave, timeout: c_int) -> c_int;
    fn sdw_show_ping_status(bus: *mut sdw_bus, status: bool_);
    fn pm_ptr(pm: *const dev_pm_ops) -> *const dev_pm_ops;
}

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! dev_dbg {
    ($($arg:tt)*) => {};
}

macro_rules! dev_err {
    ($($arg:tt)*) => {};
}

macro_rules! SOC_ENUM_SINGLE_DECL {
    ($name:ident, $reg:expr, $shift:expr, $texts:expr) => {
        static $name: c_uint = 0;
    };
}

macro_rules! SOC_ENUM {
    ($name:literal, $enum_:expr) => {
        snd_kcontrol_new { _private: [] }
    };
}

macro_rules! SOC_DAPM_SINGLE_AUTODISABLE {
    ($name:literal, $reg:expr, $shift:expr, $max:expr, $invert:expr) => {
        snd_kcontrol_new { _private: [] }
    };
}

macro_rules! SND_SOC_DAPM_AIF_IN {
    ($($arg:tt)*) => {
        snd_soc_dapm_widget_desc { _private: [] }
    };
}

macro_rules! SND_SOC_DAPM_SUPPLY {
    ($($arg:tt)*) => {
        snd_soc_dapm_widget_desc { _private: [] }
    };
}

macro_rules! SND_SOC_DAPM_DAC {
    ($($arg:tt)*) => {
        snd_soc_dapm_widget_desc { _private: [] }
    };
}

macro_rules! SND_SOC_DAPM_SWITCH {
    ($($arg:tt)*) => {
        snd_soc_dapm_widget_desc { _private: [] }
    };
}

macro_rules! SND_SOC_DAPM_PGA_E {
    ($($arg:tt)*) => {
        snd_soc_dapm_widget_desc { _private: [] }
    };
}

macro_rules! SND_SOC_DAPM_OUTPUT {
    ($($arg:tt)*) => {
        snd_soc_dapm_widget_desc { _private: [] }
    };
}

macro_rules! SDW_SLAVE_ENTRY_EXT {
    ($mfg:expr, $part:expr, $class:expr, $unique:expr, $version:expr) => {
        sdw_device_id { _private: [] }
    };
}

macro_rules! MODULE_DEVICE_TABLE {
    ($bus:ident, $name:ident) => {};
}

macro_rules! SYSTEM_SLEEP_PM_OPS {
    ($suspend:ident, $resume:ident) => {};
}

macro_rules! RUNTIME_PM_OPS {
    ($suspend:ident, $resume:ident, $idle:expr) => {};
}

macro_rules! module_sdw_driver {
    ($driver:ident) => {};
}

macro_rules! MODULE_DESCRIPTION {
    ($s:literal) => {};
}

macro_rules! MODULE_AUTHOR {
    ($s:literal) => {};
}

macro_rules! MODULE_LICENSE {
    ($s:literal) => {};
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EACCES: c_int = 13;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_MAPLE: c_uint = 0;
const SDW_SCP_INT1_BUS_CLASH: c_uint = 0;
const SDW_SCP_INT1_PARITY: c_uint = 0;
const SDW_SLAVE_QUIRKS_INVALID_INITIAL_PARITY: c_uint = 0;
const SDW_DPN_FULL: c_uint = 0;
const SDW_SLAVE_UNATTACHED: sdw_slave_status = 0;
const SDW_SLAVE_ATTACHED: sdw_slave_status = 1;
const SND_SOC_DAPM_POST_PMU: c_int = 0;
const SND_SOC_DAPM_PRE_PMD: c_int = 0;
const SND_SOC_NOPM: c_uint = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_RATE_48000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S8: c_uint = 0;
const SNDRV_PCM_FMTBIT_S20_3LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S24_LE: c_uint = 0;
const RT1308_SDW_RESET: c_uint = 0;
const RT1308_VER_C: c_uint = 0;
const RT1308_SDW_OFFSET: c_uint = 0;
const RT1308_SDW_OFFSET_BYTE1: c_uint = 0;
const RT1308_SDW_OFFSET_BYTE2: c_uint = 0;
const RT1308_SDW_OFFSET_BYTE3: c_uint = 0;
const RT1308_POWER_STATUS: c_uint = 0;
const RT1308_POWER: c_uint = 0;
const RT1308_DATA_PATH: c_uint = 0;
const RT1308_DAC_SET: c_uint = 0;
const RT1308_DVOL_MUTE_L_EN_SFT: c_uint = 0;
const RT1308_DVOL_MUTE_R_EN_SFT: c_uint = 0;

unsafe extern "C" fn rt1308_readable_register(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        0x00e0 | 0x00f0 | 0x2f01..=0x2f07 | 0x3000..=0x3001 | 0x3004..=0x3005 | 0x3008
        | 0x300a | 0xc000..=0xcff3 => true,
        _ => false,
    }
}

unsafe extern "C" fn rt1308_volatile_register(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        0x2f01..=0x2f07 | 0x3000..=0x3001 | 0x3004..=0x3005 | 0x3008 | 0x300a | 0xc000
        | 0xc710 | 0xcf01 | 0xc860..=0xc863 | 0xc870..=0xc873 => true,
        _ => false,
    }
}

static rt1308_sdw_regmap: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 8,
    readable_reg: Some(rt1308_readable_register),
    volatile_reg: Some(rt1308_volatile_register),
    max_register: 0xcfff,
    reg_defaults: unsafe { rt1308_reg_defaults.as_ptr() },
    num_reg_defaults: 0,
    cache_type: REGCACHE_MAPLE,
    use_single_read: true,
    use_single_write: true,
};

/* Bus clock frequency */
const RT1308_CLK_FREQ_9600000HZ: c_uint = 9600000;
const RT1308_CLK_FREQ_12000000HZ: c_uint = 12000000;
const RT1308_CLK_FREQ_6000000HZ: c_uint = 6000000;
const RT1308_CLK_FREQ_4800000HZ: c_uint = 4800000;
const RT1308_CLK_FREQ_2400000HZ: c_uint = 2400000;
const RT1308_CLK_FREQ_12288000HZ: c_uint = 12288000;

unsafe extern "C" fn rt1308_clock_config(dev: *mut device) -> c_int {
    let rt1308 = dev_get_drvdata(dev) as *mut rt1308_sdw_priv;
    let clk_freq: c_uint;
    let value: c_uint;

    clk_freq = (*rt1308).params.curr_dr_freq >> 1;

    match clk_freq {
        RT1308_CLK_FREQ_12000000HZ => value = 0x0,
        RT1308_CLK_FREQ_6000000HZ => value = 0x1,
        RT1308_CLK_FREQ_9600000HZ => value = 0x2,
        RT1308_CLK_FREQ_4800000HZ => value = 0x3,
        RT1308_CLK_FREQ_2400000HZ => value = 0x4,
        RT1308_CLK_FREQ_12288000HZ => value = 0x5,
        _ => return -EINVAL,
    }

    regmap_write((*rt1308).regmap, 0xe0, value);
    regmap_write((*rt1308).regmap, 0xf0, value);

    dev_dbg!(dev, "%s complete, clk_freq=%d\n", "rt1308_clock_config", clk_freq);

    0
}

unsafe extern "C" fn rt1308_read_prop(slave: *mut sdw_slave) -> c_int {
    let prop = &mut (*slave).prop as *mut sdw_slave_prop;
    let mut nval: c_int;
    let mut i: c_int;
    let mut bit: u32 = 0;
    let addr: c_ulong;
    let dpn: *mut sdw_dpn_prop;

    (*prop).scp_int1_mask = SDW_SCP_INT1_BUS_CLASH | SDW_SCP_INT1_PARITY;
    (*prop).quirks = SDW_SLAVE_QUIRKS_INVALID_INITIAL_PARITY;

    (*prop).paging_support = true;

    /* first we need to allocate memory for set bits in port lists */
    (*prop).source_ports = 0x00; /* BITMAP: 00010100 (not enable yet) */
    (*prop).sink_ports = 0x2; /* BITMAP:  00000010 */

    /* for sink */
    nval = (*prop).sink_ports.count_ones() as c_int;
    (*prop).sink_dpn_prop = devm_kcalloc(
        &mut (*slave).dev,
        nval as usize,
        size_of::<sdw_dpn_prop>(),
        GFP_KERNEL,
    ) as *mut sdw_dpn_prop;
    if (*prop).sink_dpn_prop.is_null() {
        return -ENOMEM;
    }

    i = 0;
    dpn = (*prop).sink_dpn_prop;
    addr = (*prop).sink_ports;
    while bit < 32 {
        if (addr & (1_c_ulong << bit)) != 0 {
            (*dpn.add(i as usize)).num = bit;
            (*dpn.add(i as usize)).type_ = SDW_DPN_FULL;
            (*dpn.add(i as usize)).simple_ch_prep_sm = true;
            (*dpn.add(i as usize)).ch_prep_timeout = 10;
            i += 1;
        }
        bit += 1;
    }

    /* set the timeout values */
    (*prop).clk_stop_timeout = 20;

    dev_dbg!(&mut (*slave).dev, "%s\n", "rt1308_read_prop");

    0
}

unsafe extern "C" fn rt1308_apply_calib_params(rt1308: *mut rt1308_sdw_priv) {
    let mut efuse_m_btl_l: c_uint;
    let mut efuse_m_btl_r: c_uint;
    let mut tmp: c_uint = 0;
    let mut efuse_c_btl_l: c_uint;
    let mut efuse_c_btl_r: c_uint;

    /* read efuse to apply calibration parameters */
    regmap_write((*rt1308).regmap, 0xc7f0, 0x04);
    regmap_write((*rt1308).regmap, 0xc7f1, 0xfe);
    msleep(100);
    regmap_write((*rt1308).regmap, 0xc7f0, 0x44);
    msleep(20);
    regmap_write((*rt1308).regmap, 0xc240, 0x10);

    regmap_read((*rt1308).regmap, 0xc861, &mut tmp);
    efuse_m_btl_l = tmp;
    regmap_read((*rt1308).regmap, 0xc860, &mut tmp);
    efuse_m_btl_l = efuse_m_btl_l | (tmp << 8);
    regmap_read((*rt1308).regmap, 0xc863, &mut tmp);
    efuse_c_btl_l = tmp;
    regmap_read((*rt1308).regmap, 0xc862, &mut tmp);
    efuse_c_btl_l = efuse_c_btl_l | (tmp << 8);
    regmap_read((*rt1308).regmap, 0xc871, &mut tmp);
    efuse_m_btl_r = tmp;
    regmap_read((*rt1308).regmap, 0xc870, &mut tmp);
    efuse_m_btl_r = efuse_m_btl_r | (tmp << 8);
    regmap_read((*rt1308).regmap, 0xc873, &mut tmp);
    efuse_c_btl_r = tmp;
    regmap_read((*rt1308).regmap, 0xc872, &mut tmp);
    efuse_c_btl_r = efuse_c_btl_r | (tmp << 8);
    dev_dbg!(&mut (*(*rt1308).sdw_slave).dev, "%s m_btl_l=0x%x, m_btl_r=0x%x\n", "rt1308_apply_calib_params", efuse_m_btl_l, efuse_m_btl_r);
    dev_dbg!(&mut (*(*rt1308).sdw_slave).dev, "%s c_btl_l=0x%x, c_btl_r=0x%x\n", "rt1308_apply_calib_params", efuse_c_btl_l, efuse_c_btl_r);
}

unsafe extern "C" fn rt1308_apply_bq_params(rt1308: *mut rt1308_sdw_priv) {
    let mut i: c_uint = 0;
    let mut reg: c_uint;
    let mut data: c_uint;

    while i < (*rt1308).bq_params_cnt {
        reg = *(*rt1308).bq_params.add(i as usize) as c_uint
            | ((*(*rt1308).bq_params.add((i + 1) as usize) as c_uint) << 8);
        data = *(*rt1308).bq_params.add((i + 2) as usize) as c_uint;
        regmap_write((*rt1308).regmap, reg, data);
        i += 3;
    }
}

unsafe extern "C" fn rt1308_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int {
    let rt1308 = dev_get_drvdata(dev) as *mut rt1308_sdw_priv;
    let ret: c_int = 0;
    let mut tmp: c_uint = 0;
    let mut hibernation_flag: c_uint = 0;

    if (*rt1308).hw_init {
        return 0;
    }

    regcache_cache_only((*rt1308).regmap, false);
    if (*rt1308).first_hw_init {
        regcache_cache_bypass((*rt1308).regmap, true);
    }

    /*
     * PM runtime status is marked as 'active' only when a Slave reports as Attached
     */
    if !(*rt1308).first_hw_init {
        /* update count of parent 'active' children */
        pm_runtime_set_active(&mut (*slave).dev);
    }

    pm_runtime_get_noresume(&mut (*slave).dev);

    regmap_read((*rt1308).regmap, 0xcf01, &mut hibernation_flag);
    if (hibernation_flag != 0x00) && (*rt1308).first_hw_init {
    } else {
        /* sw reset */
        regmap_write((*rt1308).regmap, RT1308_SDW_RESET, 0);

        regmap_read((*rt1308).regmap, 0xc710, &mut tmp);
        (*rt1308).hw_ver = tmp;
        dev_dbg!(dev, "%s, hw_ver=0x%x\n", "rt1308_io_init", (*rt1308).hw_ver);

        /* initial settings */
        regmap_write((*rt1308).regmap, 0xc103, 0xc0);
        regmap_write((*rt1308).regmap, 0xc030, 0x17);
        regmap_write((*rt1308).regmap, 0xc031, 0x81);
        regmap_write((*rt1308).regmap, 0xc032, 0x26);
        regmap_write((*rt1308).regmap, 0xc040, 0x80);
        regmap_write((*rt1308).regmap, 0xc041, 0x80);
        regmap_write((*rt1308).regmap, 0xc042, 0x06);
        regmap_write((*rt1308).regmap, 0xc052, 0x0a);
        regmap_write((*rt1308).regmap, 0xc080, 0x0a);
        regmap_write((*rt1308).regmap, 0xc060, 0x02);
        regmap_write((*rt1308).regmap, 0xc061, 0x75);
        regmap_write((*rt1308).regmap, 0xc062, 0x05);
        regmap_write((*rt1308).regmap, 0xc171, 0x07);
        regmap_write((*rt1308).regmap, 0xc173, 0x0d);
        if (*rt1308).hw_ver == RT1308_VER_C {
            regmap_write((*rt1308).regmap, 0xc311, 0x7f);
            regmap_write((*rt1308).regmap, 0xc300, 0x09);
        } else {
            regmap_write((*rt1308).regmap, 0xc311, 0x4f);
            regmap_write((*rt1308).regmap, 0xc300, 0x0b);
        }
        regmap_write((*rt1308).regmap, 0xc900, 0x5a);
        regmap_write((*rt1308).regmap, 0xc1a0, 0x84);
        regmap_write((*rt1308).regmap, 0xc1a1, 0x01);
        regmap_write((*rt1308).regmap, 0xc360, 0x78);
        regmap_write((*rt1308).regmap, 0xc361, 0x87);
        regmap_write((*rt1308).regmap, 0xc0a1, 0x71);
        regmap_write((*rt1308).regmap, 0xc210, 0x00);
        regmap_write((*rt1308).regmap, 0xc070, 0x00);
        regmap_write((*rt1308).regmap, 0xc100, 0xd7);
        regmap_write((*rt1308).regmap, 0xc101, 0xd7);

        /* apply BQ params */
        rt1308_apply_bq_params(rt1308);

        regmap_write((*rt1308).regmap, 0xcf01, 0x01);
    }

    if (*rt1308).first_hw_init {
        regcache_cache_bypass((*rt1308).regmap, false);
        regcache_mark_dirty((*rt1308).regmap);
    } else {
        (*rt1308).first_hw_init = true;
    }

    /* Mark Slave initialization complete */
    (*rt1308).hw_init = true;

    pm_runtime_put_autosuspend(&mut (*slave).dev);

    dev_dbg!(&mut (*slave).dev, "%s hw_init complete\n", "rt1308_io_init");

    ret
}

unsafe extern "C" fn rt1308_update_status(
    slave: *mut sdw_slave,
    status: sdw_slave_status,
) -> c_int {
    let rt1308 = dev_get_drvdata(&mut (*slave).dev) as *mut rt1308_sdw_priv;

    if status == SDW_SLAVE_UNATTACHED {
        (*rt1308).hw_init = false;
    }

    /*
     * Perform initialization only if slave status is present and
     * hw_init flag is false
     */
    if (*rt1308).hw_init || status != SDW_SLAVE_ATTACHED {
        return 0;
    }

    /* perform I/O transfers required for Slave initialization */
    rt1308_io_init(&mut (*slave).dev, slave)
}

unsafe extern "C" fn rt1308_bus_config(
    slave: *mut sdw_slave,
    params: *mut sdw_bus_params,
) -> c_int {
    let rt1308 = dev_get_drvdata(&mut (*slave).dev) as *mut rt1308_sdw_priv;
    let ret: c_int;

    ptr::copy_nonoverlapping(params, &mut (*rt1308).params, 1);

    ret = rt1308_clock_config(&mut (*slave).dev);
    if ret < 0 {
        dev_err!(&mut (*slave).dev, "Invalid clk config");
    }

    ret
}

unsafe extern "C" fn rt1308_interrupt_callback(
    slave: *mut sdw_slave,
    status: *mut sdw_slave_intr_status,
) -> c_int {
    dev_dbg!(&mut (*slave).dev, "%s control_port_stat=%x", "rt1308_interrupt_callback", (*status).control_port);

    0
}

unsafe extern "C" fn rt1308_classd_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt1308 = snd_soc_component_get_drvdata(component) as *mut rt1308_sdw_priv;

    match event {
        SND_SOC_DAPM_POST_PMU => {
            msleep(30);
            snd_soc_component_update_bits(
                component,
                RT1308_SDW_OFFSET | (RT1308_POWER_STATUS << 4),
                0x3,
                0x3,
            );
            msleep(40);
            rt1308_apply_calib_params(rt1308);
        }
        SND_SOC_DAPM_PRE_PMD => {
            snd_soc_component_update_bits(
                component,
                RT1308_SDW_OFFSET | (RT1308_POWER_STATUS << 4),
                0x3,
                0,
            );
            usleep_range(150000, 200000);
        }
        _ => {}
    }

    0
}

static rt1308_rx_data_ch_select: [*const c_char; 4] = [
    c_str!("LR"),
    c_str!("LL"),
    c_str!("RL"),
    c_str!("RR"),
];

SOC_ENUM_SINGLE_DECL!(
    rt1308_rx_data_ch_enum,
    RT1308_SDW_OFFSET | (RT1308_DATA_PATH << 4),
    0,
    rt1308_rx_data_ch_select
);

static rt1308_snd_controls: [snd_kcontrol_new; 1] = [
    /* I2S Data Channel Selection */
    SOC_ENUM!("RX Channel Select", rt1308_rx_data_ch_enum),
];

static rt1308_sto_dac_l: snd_kcontrol_new = SOC_DAPM_SINGLE_AUTODISABLE!(
    "Switch",
    RT1308_SDW_OFFSET_BYTE3 | (RT1308_DAC_SET << 4),
    RT1308_DVOL_MUTE_L_EN_SFT,
    1,
    1
);

static rt1308_sto_dac_r: snd_kcontrol_new = SOC_DAPM_SINGLE_AUTODISABLE!(
    "Switch",
    RT1308_SDW_OFFSET_BYTE3 | (RT1308_DAC_SET << 4),
    RT1308_DVOL_MUTE_R_EN_SFT,
    1,
    1
);

static rt1308_dapm_widgets: [snd_soc_dapm_widget_desc; 29] = [
    /* Audio Interface */
    SND_SOC_DAPM_AIF_IN!("AIF1RX", "DP1 Playback", 0, SND_SOC_NOPM, 0, 0),
    /* Supply Widgets */
    SND_SOC_DAPM_SUPPLY!("MBIAS20U", RT1308_SDW_OFFSET | (RT1308_POWER << 4), 7, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("ALDO", RT1308_SDW_OFFSET | (RT1308_POWER << 4), 6, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("DBG", RT1308_SDW_OFFSET | (RT1308_POWER << 4), 5, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("DACL", RT1308_SDW_OFFSET | (RT1308_POWER << 4), 4, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("CLK25M", RT1308_SDW_OFFSET | (RT1308_POWER << 4), 2, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("ADC_R", RT1308_SDW_OFFSET | (RT1308_POWER << 4), 1, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("ADC_L", RT1308_SDW_OFFSET | (RT1308_POWER << 4), 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("DAC Power", RT1308_SDW_OFFSET | (RT1308_POWER << 4), 3, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("DLDO", RT1308_SDW_OFFSET_BYTE1 | (RT1308_POWER << 4), 5, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("VREF", RT1308_SDW_OFFSET_BYTE1 | (RT1308_POWER << 4), 4, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("MIXER_R", RT1308_SDW_OFFSET_BYTE1 | (RT1308_POWER << 4), 2, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("MIXER_L", RT1308_SDW_OFFSET_BYTE1 | (RT1308_POWER << 4), 1, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("MBIAS4U", RT1308_SDW_OFFSET_BYTE1 | (RT1308_POWER << 4), 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("PLL2_LDO", RT1308_SDW_OFFSET_BYTE2 | (RT1308_POWER << 4), 4, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("PLL2B", RT1308_SDW_OFFSET_BYTE2 | (RT1308_POWER << 4), 3, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("PLL2F", RT1308_SDW_OFFSET_BYTE2 | (RT1308_POWER << 4), 2, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("PLL2F2", RT1308_SDW_OFFSET_BYTE2 | (RT1308_POWER << 4), 1, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("PLL2B2", RT1308_SDW_OFFSET_BYTE2 | (RT1308_POWER << 4), 0, 0, ptr::null(), 0),
    /* Digital Interface */
    SND_SOC_DAPM_DAC!("DAC", ptr::null(), SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_SWITCH!("DAC L", SND_SOC_NOPM, 0, 0, &rt1308_sto_dac_l),
    SND_SOC_DAPM_SWITCH!("DAC R", SND_SOC_NOPM, 0, 0, &rt1308_sto_dac_r),
    /* Output Lines */
    SND_SOC_DAPM_PGA_E!("CLASS D", SND_SOC_NOPM, 0, 0, ptr::null(), 0, rt1308_classd_event, SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_OUTPUT!("SPOL"),
    SND_SOC_DAPM_OUTPUT!("SPOR"),
];

static rt1308_dapm_routes: [snd_soc_dapm_route; 28] = [
    snd_soc_dapm_route { sink: c_str!("DAC"), control: ptr::null(), source: c_str!("AIF1RX") },
    snd_soc_dapm_route { sink: c_str!("DAC"), control: ptr::null(), source: c_str!("MBIAS20U") },
    snd_soc_dapm_route { sink: c_str!("DAC"), control: ptr::null(), source: c_str!("ALDO") },
    snd_soc_dapm_route { sink: c_str!("DAC"), control: ptr::null(), source: c_str!("DBG") },
    snd_soc_dapm_route { sink: c_str!("DAC"), control: ptr::null(), source: c_str!("DACL") },
    snd_soc_dapm_route { sink: c_str!("DAC"), control: ptr::null(), source: c_str!("CLK25M") },
    snd_soc_dapm_route { sink: c_str!("DAC"), control: ptr::null(), source: c_str!("ADC_R") },
    snd_soc_dapm_route { sink: c_str!("DAC"), control: ptr::null(), source: c_str!("ADC_L") },
    snd_soc_dapm_route { sink: c_str!("DAC"), control: ptr::null(), source: c_str!("DLDO") },
    snd_soc_dapm_route { sink: c_str!("DAC"), control: ptr::null(), source: c_str!("VREF") },
    snd_soc_dapm_route { sink: c_str!("DAC"), control: ptr::null(), source: c_str!("MIXER_R") },
    snd_soc_dapm_route { sink: c_str!("DAC"), control: ptr::null(), source: c_str!("MIXER_L") },
    snd_soc_dapm_route { sink: c_str!("DAC"), control: ptr::null(), source: c_str!("MBIAS4U") },
    snd_soc_dapm_route { sink: c_str!("DAC"), control: ptr::null(), source: c_str!("PLL2_LDO") },
    snd_soc_dapm_route { sink: c_str!("DAC"), control: ptr::null(), source: c_str!("PLL2B") },
    snd_soc_dapm_route { sink: c_str!("DAC"), control: ptr::null(), source: c_str!("PLL2F") },
    snd_soc_dapm_route { sink: c_str!("DAC"), control: ptr::null(), source: c_str!("PLL2F2") },
    snd_soc_dapm_route { sink: c_str!("DAC"), control: ptr::null(), source: c_str!("PLL2B2") },
    snd_soc_dapm_route { sink: c_str!("DAC L"), control: c_str!("Switch"), source: c_str!("DAC") },
    snd_soc_dapm_route { sink: c_str!("DAC R"), control: c_str!("Switch"), source: c_str!("DAC") },
    snd_soc_dapm_route { sink: c_str!("DAC L"), control: ptr::null(), source: c_str!("DAC Power") },
    snd_soc_dapm_route { sink: c_str!("DAC R"), control: ptr::null(), source: c_str!("DAC Power") },
    snd_soc_dapm_route { sink: c_str!("CLASS D"), control: ptr::null(), source: c_str!("DAC L") },
    snd_soc_dapm_route { sink: c_str!("CLASS D"), control: ptr::null(), source: c_str!("DAC R") },
    snd_soc_dapm_route { sink: c_str!("SPOL"), control: ptr::null(), source: c_str!("CLASS D") },
    snd_soc_dapm_route { sink: c_str!("SPOR"), control: ptr::null(), source: c_str!("CLASS D") },
];

unsafe extern "C" fn rt1308_set_sdw_stream(
    dai: *mut snd_soc_dai,
    sdw_stream: *mut c_void,
    direction: c_int,
) -> c_int {
    snd_soc_dai_dma_data_set(dai, direction, sdw_stream);

    0
}

unsafe extern "C" fn rt1308_sdw_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    snd_soc_dai_set_dma_data(dai, substream, ptr::null_mut());
}

unsafe extern "C" fn rt1308_sdw_set_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    rx_mask: c_uint,
    slots: c_int,
    _slot_width: c_int,
) -> c_int {
    let component = (*dai).component;
    let rt1308 = snd_soc_component_get_drvdata(component) as *mut rt1308_sdw_priv;

    if tx_mask != 0 {
        return -EINVAL;
    }

    if slots > 2 {
        return -EINVAL;
    }

    (*rt1308).rx_mask = rx_mask;
    (*rt1308).slots = slots;
    /* slot_width is not used since it's irrelevant for SoundWire */

    0
}

unsafe extern "C" fn rt1308_sdw_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let rt1308 = snd_soc_component_get_drvdata(component) as *mut rt1308_sdw_priv;
    let mut stream_config: sdw_stream_config = core::mem::zeroed();
    let mut port_config: sdw_port_config = core::mem::zeroed();
    let sdw_stream: *mut sdw_stream_runtime;
    let retval: c_int;

    dev_dbg!((*dai).dev, "%s %s", "rt1308_sdw_hw_params", (*dai).name);
    sdw_stream = snd_soc_dai_get_dma_data(dai, substream);

    if sdw_stream.is_null() {
        return -EINVAL;
    }

    if (*rt1308).sdw_slave.is_null() {
        return -EINVAL;
    }

    /* SoundWire specific configuration */
    snd_sdw_params_to_config(substream, params, &mut stream_config, &mut port_config);

    /* port 1 for playback */
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        port_config.num = 1;
    } else {
        return -EINVAL;
    }

    if (*rt1308).slots != 0 {
        stream_config.ch_count = (*rt1308).slots as c_uint;
        port_config.ch_mask = (*rt1308).rx_mask;
    }

    retval = sdw_stream_add_slave(
        (*rt1308).sdw_slave,
        &mut stream_config,
        &mut port_config,
        1,
        sdw_stream,
    );
    if retval != 0 {
        dev_err!((*dai).dev, "Unable to configure port\n");
        return retval;
    }

    retval
}

unsafe extern "C" fn rt1308_sdw_pcm_hw_free(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let rt1308 = snd_soc_component_get_drvdata(component) as *mut rt1308_sdw_priv;
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);

    if (*rt1308).sdw_slave.is_null() {
        return -EINVAL;
    }

    sdw_stream_remove_slave((*rt1308).sdw_slave, sdw_stream);
    0
}

/*
 * slave_ops: callbacks for get_clock_stop_mode, clock_stop and
 * port_prep are not defined for now
 */
static rt1308_slave_ops: sdw_slave_ops = sdw_slave_ops {
    read_prop: Some(rt1308_read_prop),
    interrupt_callback: Some(rt1308_interrupt_callback),
    update_status: Some(rt1308_update_status),
    bus_config: Some(rt1308_bus_config),
};

unsafe extern "C" fn rt1308_sdw_parse_dt(rt1308: *mut rt1308_sdw_priv, dev: *mut device) -> c_int {
    let mut ret: c_int = 0;

    device_property_read_u32(dev, c_str!("realtek,bq-params-cnt"), &mut (*rt1308).bq_params_cnt);
    if (*rt1308).bq_params_cnt != 0 {
        (*rt1308).bq_params = devm_kzalloc(dev, (*rt1308).bq_params_cnt as usize, GFP_KERNEL) as *mut u8;
        if (*rt1308).bq_params.is_null() {
            dev_err!(dev, "Could not allocate bq_params memory\n");
            ret = -ENOMEM;
        } else {
            ret = device_property_read_u8_array(
                dev,
                c_str!("realtek,bq-params"),
                (*rt1308).bq_params,
                (*rt1308).bq_params_cnt,
            );
            if ret < 0 {
                dev_err!(dev, "Could not read list of realtek,bq-params\n");
            }
        }
    }

    dev_dbg!(dev, "bq_params_cnt=%d\n", (*rt1308).bq_params_cnt);
    ret
}

unsafe extern "C" fn rt1308_sdw_component_probe(component: *mut snd_soc_component) -> c_int {
    let rt1308 = snd_soc_component_get_drvdata(component) as *mut rt1308_sdw_priv;
    let ret: c_int;

    (*rt1308).component = component;
    rt1308_sdw_parse_dt(rt1308, &mut (*(*rt1308).sdw_slave).dev);

    if !(*rt1308).first_hw_init {
        return 0;
    }

    ret = pm_runtime_resume((*component).dev);
    if ret < 0 && ret != -EACCES {
        return ret;
    }

    /* apply BQ params */
    rt1308_apply_bq_params(rt1308);

    0
}

static soc_component_sdw_rt1308: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(rt1308_sdw_component_probe),
    controls: rt1308_snd_controls.as_ptr(),
    num_controls: rt1308_snd_controls.len() as c_uint,
    dapm_widgets: rt1308_dapm_widgets.as_ptr(),
    num_dapm_widgets: rt1308_dapm_widgets.len() as c_uint,
    dapm_routes: rt1308_dapm_routes.as_ptr(),
    num_dapm_routes: rt1308_dapm_routes.len() as c_uint,
    endianness: 1,
};

static rt1308_aif_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(rt1308_sdw_hw_params),
    hw_free: Some(rt1308_sdw_pcm_hw_free),
    set_stream: Some(rt1308_set_sdw_stream),
    shutdown: Some(rt1308_sdw_shutdown),
    set_tdm_slot: Some(rt1308_sdw_set_tdm_slot),
};

const RT1308_STEREO_RATES: c_uint = SNDRV_PCM_RATE_48000;
const RT1308_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S8
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S24_LE;

static mut rt1308_sdw_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c_str!("rt1308-aif"),
    playback: snd_soc_pcm_stream {
        stream_name: c_str!("DP1 Playback"),
        channels_min: 1,
        channels_max: 2,
        rates: RT1308_STEREO_RATES,
        formats: RT1308_FORMATS,
    },
    ops: &rt1308_aif_dai_ops,
}];

unsafe extern "C" fn rt1308_sdw_init(
    dev: *mut device,
    regmap: *mut regmap,
    slave: *mut sdw_slave,
) -> c_int {
    let rt1308: *mut rt1308_sdw_priv;
    let ret: c_int;

    rt1308 = devm_kzalloc(dev, size_of::<rt1308_sdw_priv>(), GFP_KERNEL) as *mut rt1308_sdw_priv;
    if rt1308.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(dev, rt1308 as *mut c_void);
    (*rt1308).sdw_slave = slave;
    (*rt1308).regmap = regmap;

    regcache_cache_only((*rt1308).regmap, true);

    /*
     * Mark hw_init to false
     * HW init will be performed when device reports present
     */
    (*rt1308).hw_init = false;
    (*rt1308).first_hw_init = false;

    ret = devm_snd_soc_register_component(
        dev,
        &soc_component_sdw_rt1308,
        rt1308_sdw_dai.as_mut_ptr(),
        rt1308_sdw_dai.len() as c_uint,
    );
    if ret < 0 {
        return ret;
    }

    /* set autosuspend parameters */
    pm_runtime_set_autosuspend_delay(dev, 3000);
    pm_runtime_use_autosuspend(dev);

    /* make sure the device does not suspend immediately */
    pm_runtime_mark_last_busy(dev);

    pm_runtime_enable(dev);

    /* important note: the device is NOT tagged as 'active' and will remain
     * 'suspended' until the hardware is enumerated/initialized. This is required
     * to make sure the ASoC framework use of pm_runtime_get_sync() does not silently
     * fail with -EACCESS because of race conditions between card creation and enumeration
     */

    dev_dbg!(dev, "%s\n", "rt1308_sdw_init");

    0
}

unsafe extern "C" fn rt1308_sdw_probe(
    slave: *mut sdw_slave,
    _id: *const sdw_device_id,
) -> c_int {
    let regmap: *mut regmap;

    /* Regmap Initialization */
    regmap = devm_regmap_init_sdw(slave, &rt1308_sdw_regmap);
    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }

    rt1308_sdw_init(&mut (*slave).dev, regmap, slave)
}

unsafe extern "C" fn rt1308_sdw_remove(slave: *mut sdw_slave) {
    pm_runtime_disable(&mut (*slave).dev);
}

static rt1308_id: [sdw_device_id; 2] = [
    SDW_SLAVE_ENTRY_EXT!(0x025d, 0x1308, 0x2, 0, 0),
    sdw_device_id { _private: [] },
];
MODULE_DEVICE_TABLE!(sdw, rt1308_id);

unsafe extern "C" fn rt1308_dev_suspend(dev: *mut device) -> c_int {
    let rt1308 = dev_get_drvdata(dev) as *mut rt1308_sdw_priv;

    if !(*rt1308).hw_init {
        return 0;
    }

    regcache_cache_only((*rt1308).regmap, true);

    0
}

const RT1308_PROBE_TIMEOUT: c_int = 5000;

unsafe extern "C" fn rt1308_dev_resume(dev: *mut device) -> c_int {
    let slave = dev_to_sdw_dev(dev);
    let rt1308 = dev_get_drvdata(dev) as *mut rt1308_sdw_priv;
    let ret: c_int;

    if !(*rt1308).first_hw_init {
        return 0;
    }

    ret = sdw_slave_wait_for_init(slave, RT1308_PROBE_TIMEOUT);
    if ret != 0 {
        sdw_show_ping_status((*slave).bus, true);
        return ret;
    }

    regcache_cache_only((*rt1308).regmap, false);
    regcache_sync_region((*rt1308).regmap, 0xc000, 0xcfff);

    0
}

static rt1308_pm: dev_pm_ops = dev_pm_ops { _private: [] };
SYSTEM_SLEEP_PM_OPS!(rt1308_dev_suspend, rt1308_dev_resume);
RUNTIME_PM_OPS!(rt1308_dev_suspend, rt1308_dev_resume, ptr::null());

static mut rt1308_sdw_driver: sdw_driver = sdw_driver {
    driver: device_driver {
        name: c_str!("rt1308"),
        pm: unsafe { pm_ptr(&rt1308_pm) },
    },
    probe: Some(rt1308_sdw_probe),
    remove: Some(rt1308_sdw_remove),
    ops: &rt1308_slave_ops,
    id_table: rt1308_id.as_ptr(),
};
module_sdw_driver!(rt1308_sdw_driver);

MODULE_DESCRIPTION!("ASoC RT1308 driver SDW");
MODULE_AUTHOR!("Shuming Fan <shumingf@realtek.com>");
MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
