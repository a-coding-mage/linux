// SPDX-License-Identifier: GPL-2.0-only
//
// rt5682-sdw.c  --  RT5682 ALSA SoC audio component driver
//
// Copyright 2019 Realtek Semiconductor Corp.
// Author: Oder Chiou <oder_chiou@realtek.com>
//

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

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_MAPLE: c_uint = 0;
const REGCACHE_NONE: c_uint = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SDW_SLAVE_UNATTACHED: sdw_slave_status = 0;
const SDW_SLAVE_ATTACHED: sdw_slave_status = 1;
const SDW_DPN_FULL: c_uint = 0;
const SDW_SLAVE_QUIRKS_INVALID_INITIAL_PARITY: c_uint = 0;
const SDW_SCP_INTMASK1: c_uint = 0;

const RT5682_SDW_ADDR_L: c_uint = 0x3000;
const RT5682_SDW_ADDR_H: c_uint = 0x3001;
const RT5682_SDW_DATA_L: c_uint = 0x3004;
const RT5682_SDW_DATA_H: c_uint = 0x3005;
const RT5682_SDW_CMD: c_uint = 0x3008;

/* Bus clock frequency */
const RT5682_CLK_FREQ_9600000HZ: c_uint = 9600000;
const RT5682_CLK_FREQ_12000000HZ: c_uint = 12000000;
const RT5682_CLK_FREQ_6000000HZ: c_uint = 6000000;
const RT5682_CLK_FREQ_4800000HZ: c_uint = 4800000;
const RT5682_CLK_FREQ_2400000HZ: c_uint = 2400000;
const RT5682_CLK_FREQ_12288000HZ: c_uint = 12288000;

type sdw_slave_status = c_int;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    _private: [u8; 0],
}

#[repr(C)]
pub struct workqueue_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
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
    pub name: *const c_char,
    pub id: c_int,
    pub component: *mut snd_soc_component,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_ulong,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub set_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, *mut c_void, c_int) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct reg_default {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
    pub name: *const c_char,
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub cache_type: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub use_single_read: bool_,
    pub use_single_write: bool_,
    pub reg_read: Option<unsafe extern "C" fn(*mut c_void, c_uint, *mut c_uint) -> c_int>,
    pub reg_write: Option<unsafe extern "C" fn(*mut c_void, c_uint, c_uint) -> c_int>,
}

#[repr(C)]
pub struct sdw_stream_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_port_config {
    pub num: c_uint,
}

#[repr(C)]
pub struct sdw_stream_runtime {
    _private: [u8; 0],
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
    pub source_ports: u32,
    pub sink_ports: u32,
    pub src_dpn_prop: *mut sdw_dpn_prop,
    pub sink_dpn_prop: *mut sdw_dpn_prop,
    pub clk_stop_timeout: c_uint,
    pub wake_capable: c_uint,
}

#[repr(C)]
pub struct sdw_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_slave {
    pub dev: device,
    pub prop: sdw_slave_prop,
    pub unattach_request: bool_,
    pub bus: *mut sdw_bus,
}

#[repr(C)]
pub struct sdw_slave_intr_status {
    pub control_port: c_uint,
}

#[repr(C)]
pub struct sdw_device_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_slave_ops {
    pub read_prop: Option<unsafe extern "C" fn(*mut sdw_slave) -> c_int>,
    pub interrupt_callback: Option<unsafe extern "C" fn(*mut sdw_slave, *mut sdw_slave_intr_status) -> c_int>,
    pub update_status: Option<unsafe extern "C" fn(*mut sdw_slave, sdw_slave_status) -> c_int>,
    pub bus_config: Option<unsafe extern "C" fn(*mut sdw_slave, *mut sdw_bus_params) -> c_int>,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub system_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub system_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_idle: Option<unsafe extern "C" fn(*mut device) -> c_int>,
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
pub struct rt5682_priv {
    pub slave: *mut sdw_slave,
    pub sdw_regmap: *mut regmap,
    pub is_sdw: bool_,
    pub disable_irq_lock: mutex,
    pub regmap: *mut regmap,
    pub hw_init: bool_,
    pub first_hw_init: bool_,
    pub calibrate_mutex: mutex,
    pub jack_detect_work: delayed_work,
    pub disable_irq: bool_,
    pub params: sdw_bus_params,
    pub irq_work_delay_time: c_uint,
}

unsafe extern "C" {
    static rt5682_reg: *const reg_default;
    static rt5682_aif1_dai_ops: snd_soc_dai_ops;
    static rt5682_aif2_dai_ops: snd_soc_dai_ops;
    static rt5682_soc_component_dev: snd_soc_component_driver;
    static mut system_power_efficient_wq: *mut workqueue_struct;

    static RT5682_I2C_MODE: c_uint;
    static RT5682_REG_NUM: c_uint;
    static RT5682_AIF1: c_int;
    static RT5682_AIF2: c_int;
    static RT5682_SDW: c_int;
    static RT5682_STEREO_RATES: c_uint;
    static RT5682_FORMATS: c_ulong;
    static RT5682_SDW_REF_1_48K: c_uint;
    static RT5682_SDW_REF_2_48K: c_uint;
    static RT5682_SDW_REF_1_96K: c_uint;
    static RT5682_SDW_REF_2_96K: c_uint;
    static RT5682_SDW_REF_1_192K: c_uint;
    static RT5682_SDW_REF_2_192K: c_uint;
    static RT5682_SDW_REF_1_32K: c_uint;
    static RT5682_SDW_REF_2_32K: c_uint;
    static RT5682_SDW_REF_1_24K: c_uint;
    static RT5682_SDW_REF_2_24K: c_uint;
    static RT5682_SDW_REF_1_16K: c_uint;
    static RT5682_SDW_REF_2_16K: c_uint;
    static RT5682_SDW_REF_1_12K: c_uint;
    static RT5682_SDW_REF_2_12K: c_uint;
    static RT5682_SDW_REF_1_8K: c_uint;
    static RT5682_SDW_REF_2_8K: c_uint;
    static RT5682_SDW_REF_1_44K: c_uint;
    static RT5682_SDW_REF_2_44K: c_uint;
    static RT5682_SDW_REF_1_88K: c_uint;
    static RT5682_SDW_REF_2_88K: c_uint;
    static RT5682_SDW_REF_1_176K: c_uint;
    static RT5682_SDW_REF_2_176K: c_uint;
    static RT5682_SDW_REF_1_22K: c_uint;
    static RT5682_SDW_REF_2_22K: c_uint;
    static RT5682_SDW_REF_1_11K: c_uint;
    static RT5682_SDW_REF_2_11K: c_uint;
    static RT5682_DAC_OSR_D_8: c_uint;
    static RT5682_ADC_OSR_D_8: c_uint;
    static RT5682_DAC_OSR_D_4: c_uint;
    static RT5682_ADC_OSR_D_4: c_uint;
    static RT5682_DAC_OSR_D_2: c_uint;
    static RT5682_ADC_OSR_D_2: c_uint;
    static RT5682_SDW_REF_CLK: c_uint;
    static RT5682_SDW_REF_1_MASK: c_uint;
    static RT5682_SDW_REF_2_MASK: c_uint;
    static RT5682_ADDA_CLK_1: c_uint;
    static RT5682_DAC_OSR_MASK: c_uint;
    static RT5682_ADC_OSR_MASK: c_uint;
    static RT5682_DEVICE_ID: c_uint;
    static DEVICE_ID: c_uint;
    static RT5682_CBJ_CTRL_2: c_uint;
    static RT5682_EXT_JD_SRC: c_uint;
    static RT5682_EXT_JD_SRC_MANUAL: c_uint;
    static RT5682_DEPOP_1: c_uint;
    static RT5682_PWR_ANLG_1: c_uint;
    static RT5682_LDO1_DVO_MASK: c_uint;
    static RT5682_HP_DRIVER_MASK: c_uint;
    static RT5682_LDO1_DVO_12: c_uint;
    static RT5682_HP_DRIVER_5X: c_uint;
    static RT5682_MICBIAS_2: c_uint;
    static RT5682_TEST_MODE_CTRL_1: c_uint;
    static RT5682_BIAS_CUR_CTRL_8: c_uint;
    static RT5682_HPA_CP_BIAS_CTRL_MASK: c_uint;
    static RT5682_HPA_CP_BIAS_3UA: c_uint;
    static RT5682_CHARGE_PUMP_1: c_uint;
    static RT5682_CP_CLK_HP_MASK: c_uint;
    static RT5682_CP_CLK_HP_300KHZ: c_uint;
    static RT5682_HP_CHARGE_PUMP_1: c_uint;
    static RT5682_PM_HP_MASK: c_uint;
    static RT5682_PM_HP_HV: c_uint;
    static RT5682_PLL2_INTERNAL: c_uint;
    static RT5682_PLL2_CTRL_1: c_uint;
    static RT5682_PLL2_CTRL_2: c_uint;
    static RT5682_PLL2_CTRL_3: c_uint;
    static RT5682_PLL2_CTRL_4: c_uint;
    static RT5682_PLL_TRACK_2: c_uint;
    static RT5682_PLL_TRACK_3: c_uint;
    static RT5682_GLB_CLK: c_uint;
    static RT5682_SCLK_SRC_MASK: c_uint;
    static RT5682_PLL2_SRC_MASK: c_uint;
    static RT5682_SCLK_SRC_PLL2: c_uint;
    static RT5682_PLL2_SRC_SDW: c_uint;
    static RT5682_CBJ_CTRL_1: c_uint;
    static RT5682_CBJ_CTRL_5: c_uint;
    static RT5682_CBJ_CTRL_3: c_uint;
    static RT5682_CBJ_IN_BUF_EN: c_uint;
    static RT5682_SAR_IL_CMD_1: c_uint;
    static RT5682_SAR_POW_MASK: c_uint;
    static RT5682_SAR_POW_EN: c_uint;
    static RT5682_RC_CLK_CTRL: c_uint;
    static RT5682_POW_IRQ: c_uint;
    static RT5682_POW_JDH: c_uint;
    static RT5682_POW_ANA: c_uint;
    static RT5682_PWR_ANLG_2: c_uint;
    static RT5682_PWR_JDH: c_uint;
    static RT5682_IRQ_CTRL_2: c_uint;
    static RT5682_JD1_EN_MASK: c_uint;
    static RT5682_JD1_IRQ_MASK: c_uint;
    static RT5682_JD1_EN: c_uint;
    static RT5682_JD1_IRQ_PUL: c_uint;
    static SDW_SCP_INT1_IMPL_DEF: c_uint;
    static SDW_SCP_INT1_BUS_CLASH: c_uint;
    static SDW_SCP_INT1_PARITY: c_uint;
    static RT5682_PROBE_TIMEOUT: c_uint;

    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: c_int, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init(dev: *mut device, bus: *const c_void, bus_context: *mut device, config: *const regmap_config) -> *mut regmap;
    fn devm_regmap_init_sdw(slave: *mut sdw_slave, config: *const regmap_config) -> *mut regmap;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regcache_cache_bypass(map: *mut regmap, enable: bool_);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn rt5682_volatile_register(dev: *mut device, reg: c_uint) -> bool_;
    fn rt5682_readable_register(dev: *mut device, reg: c_uint) -> bool_;
    fn snd_soc_dai_dma_data_set(dai: *mut snd_soc_dai, direction: c_int, data: *mut c_void);
    fn snd_soc_dai_set_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream, data: *mut c_void);
    fn snd_soc_dai_get_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream) -> *mut sdw_stream_runtime;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_sdw_params_to_config(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, stream_config: *mut sdw_stream_config, port_config: *mut sdw_port_config);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn sdw_stream_add_slave(slave: *mut sdw_slave, stream_config: *mut sdw_stream_config, port_config: *mut sdw_port_config, count: c_uint, stream: *mut sdw_stream_runtime) -> c_int;
    fn sdw_stream_remove_slave(slave: *mut sdw_slave, stream: *mut sdw_stream_runtime);
    fn rt5682_get_ldo1(rt5682: *mut rt5682_priv, dev: *mut device) -> c_int;
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn rt5682_jack_detect_handler(work: *mut c_void);
    fn INIT_DELAYED_WORK(work: *mut delayed_work, func: unsafe extern "C" fn(*mut c_void));
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
    fn pm_runtime_put_autosuspend(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn rt5682_calibrate(rt5682: *mut rt5682_priv);
    fn rt5682_apply_patch_list(rt5682: *mut rt5682_priv, dev: *mut device);
    fn mod_delayed_work(wq: *mut workqueue_struct, dwork: *mut delayed_work, delay: c_ulong) -> bool_;
    fn msecs_to_jiffies(m: c_uint) -> c_ulong;
    fn hweight32(w: u32) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn cancel_delayed_work_sync(dwork: *mut delayed_work) -> bool_;
    fn dev_to_sdw_dev(dev: *mut device) -> *mut sdw_slave;
    fn sdw_update_no_pm(slave: *mut sdw_slave, addr: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn sdw_write_no_pm(slave: *mut sdw_slave, addr: c_uint, val: c_uint) -> c_int;
    fn sdw_slave_wait_for_init(slave: *mut sdw_slave, timeout: c_uint) -> c_int;
    fn sdw_show_ping_status(bus: *mut sdw_bus, status: bool_);
    fn PTR_ERR(ptr: *mut c_void) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn dev_vdbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

unsafe extern "C" fn rt5682_sdw_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    let dev = context as *mut device;
    let rt5682 = dev_get_drvdata(dev) as *mut rt5682_priv;
    let mut data_l: c_uint = 0;
    let mut data_h: c_uint = 0;

    regmap_write((*rt5682).sdw_regmap, RT5682_SDW_CMD, 0);
    regmap_write((*rt5682).sdw_regmap, RT5682_SDW_ADDR_H, (reg >> 8) & 0xff);
    regmap_write((*rt5682).sdw_regmap, RT5682_SDW_ADDR_L, reg & 0xff);
    regmap_read((*rt5682).sdw_regmap, RT5682_SDW_DATA_H, &mut data_h);
    regmap_read((*rt5682).sdw_regmap, RT5682_SDW_DATA_L, &mut data_l);

    *val = (data_h << 8) | data_l;

    dev_vdbg(dev, c"[%s] %04x => %04x\n".as_ptr(), c"rt5682_sdw_read".as_ptr(), reg, *val);

    0
}

unsafe extern "C" fn rt5682_sdw_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    let dev = context as *mut device;
    let rt5682 = dev_get_drvdata(dev) as *mut rt5682_priv;

    regmap_write((*rt5682).sdw_regmap, RT5682_SDW_CMD, 1);
    regmap_write((*rt5682).sdw_regmap, RT5682_SDW_ADDR_H, (reg >> 8) & 0xff);
    regmap_write((*rt5682).sdw_regmap, RT5682_SDW_ADDR_L, reg & 0xff);
    regmap_write((*rt5682).sdw_regmap, RT5682_SDW_DATA_H, (val >> 8) & 0xff);
    regmap_write((*rt5682).sdw_regmap, RT5682_SDW_DATA_L, val & 0xff);

    dev_vdbg(dev, c"[%s] %04x <= %04x\n".as_ptr(), c"rt5682_sdw_write".as_ptr(), reg, val);

    0
}

static rt5682_sdw_indirect_regmap: regmap_config = regmap_config {
    name: ptr::null(),
    reg_bits: 16,
    val_bits: 16,
    max_register: unsafe { RT5682_I2C_MODE },
    volatile_reg: Some(rt5682_volatile_register),
    readable_reg: Some(rt5682_readable_register),
    cache_type: REGCACHE_MAPLE,
    reg_defaults: unsafe { rt5682_reg },
    num_reg_defaults: unsafe { RT5682_REG_NUM },
    use_single_read: true,
    use_single_write: true,
    reg_read: Some(rt5682_sdw_read),
    reg_write: Some(rt5682_sdw_write),
};

unsafe extern "C" fn rt5682_set_sdw_stream(dai: *mut snd_soc_dai, sdw_stream: *mut c_void, direction: c_int) -> c_int {
    snd_soc_dai_dma_data_set(dai, direction, sdw_stream);
    0
}

unsafe extern "C" fn rt5682_sdw_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    snd_soc_dai_set_dma_data(dai, substream, ptr::null_mut());
}

unsafe extern "C" fn rt5682_sdw_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let rt5682 = snd_soc_component_get_drvdata(component) as *mut rt5682_priv;
    let mut stream_config: sdw_stream_config = core::mem::zeroed();
    let mut port_config: sdw_port_config = core::mem::zeroed();
    let sdw_stream: *mut sdw_stream_runtime;
    let mut retval: c_int;
    let mut val_p: c_uint = 0;
    let mut val_c: c_uint = 0;
    let mut osr_p: c_uint = 0;
    let mut osr_c: c_uint = 0;

    dev_dbg((*dai).dev, c"%s %s".as_ptr(), c"rt5682_sdw_hw_params".as_ptr(), (*dai).name);

    sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
    if sdw_stream.is_null() {
        return -ENOMEM;
    }

    if (*rt5682).slave.is_null() {
        return -EINVAL;
    }

    /* SoundWire specific configuration */
    snd_sdw_params_to_config(substream, params, &mut stream_config, &mut port_config);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        port_config.num = 1;
    } else {
        port_config.num = 2;
    }

    retval = sdw_stream_add_slave((*rt5682).slave, &mut stream_config, &mut port_config, 1, sdw_stream);
    if retval != 0 {
        dev_err((*dai).dev, c"%s: Unable to configure port\n".as_ptr(), c"rt5682_sdw_hw_params".as_ptr());
        return retval;
    }

    match params_rate(params) {
        48000 => {
            val_p = RT5682_SDW_REF_1_48K;
            val_c = RT5682_SDW_REF_2_48K;
        }
        96000 => {
            val_p = RT5682_SDW_REF_1_96K;
            val_c = RT5682_SDW_REF_2_96K;
        }
        192000 => {
            val_p = RT5682_SDW_REF_1_192K;
            val_c = RT5682_SDW_REF_2_192K;
        }
        32000 => {
            val_p = RT5682_SDW_REF_1_32K;
            val_c = RT5682_SDW_REF_2_32K;
        }
        24000 => {
            val_p = RT5682_SDW_REF_1_24K;
            val_c = RT5682_SDW_REF_2_24K;
        }
        16000 => {
            val_p = RT5682_SDW_REF_1_16K;
            val_c = RT5682_SDW_REF_2_16K;
        }
        12000 => {
            val_p = RT5682_SDW_REF_1_12K;
            val_c = RT5682_SDW_REF_2_12K;
        }
        8000 => {
            val_p = RT5682_SDW_REF_1_8K;
            val_c = RT5682_SDW_REF_2_8K;
        }
        44100 => {
            val_p = RT5682_SDW_REF_1_44K;
            val_c = RT5682_SDW_REF_2_44K;
        }
        88200 => {
            val_p = RT5682_SDW_REF_1_88K;
            val_c = RT5682_SDW_REF_2_88K;
        }
        176400 => {
            val_p = RT5682_SDW_REF_1_176K;
            val_c = RT5682_SDW_REF_2_176K;
        }
        22050 => {
            val_p = RT5682_SDW_REF_1_22K;
            val_c = RT5682_SDW_REF_2_22K;
        }
        11025 => {
            val_p = RT5682_SDW_REF_1_11K;
            val_c = RT5682_SDW_REF_2_11K;
        }
        _ => return -EINVAL,
    }

    if params_rate(params) <= 48000 {
        osr_p = RT5682_DAC_OSR_D_8;
        osr_c = RT5682_ADC_OSR_D_8;
    } else if params_rate(params) <= 96000 {
        osr_p = RT5682_DAC_OSR_D_4;
        osr_c = RT5682_ADC_OSR_D_4;
    } else {
        osr_p = RT5682_DAC_OSR_D_2;
        osr_c = RT5682_ADC_OSR_D_2;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        regmap_update_bits((*rt5682).regmap, RT5682_SDW_REF_CLK, RT5682_SDW_REF_1_MASK, val_p);
        regmap_update_bits((*rt5682).regmap, RT5682_ADDA_CLK_1, RT5682_DAC_OSR_MASK, osr_p);
    } else {
        regmap_update_bits((*rt5682).regmap, RT5682_SDW_REF_CLK, RT5682_SDW_REF_2_MASK, val_c);
        regmap_update_bits((*rt5682).regmap, RT5682_ADDA_CLK_1, RT5682_ADC_OSR_MASK, osr_c);
    }

    retval
}

unsafe extern "C" fn rt5682_sdw_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let rt5682 = snd_soc_component_get_drvdata(component) as *mut rt5682_priv;
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);

    if (*rt5682).slave.is_null() {
        return -EINVAL;
    }

    sdw_stream_remove_slave((*rt5682).slave, sdw_stream);
    0
}

static rt5682_sdw_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(rt5682_sdw_hw_params),
    hw_free: Some(rt5682_sdw_hw_free),
    set_stream: Some(rt5682_set_sdw_stream),
    shutdown: Some(rt5682_sdw_shutdown),
};

static mut rt5682_dai: [snd_soc_dai_driver; 3] = unsafe {
    [
        snd_soc_dai_driver {
            name: c"rt5682-aif1".as_ptr(),
            id: RT5682_AIF1,
            playback: snd_soc_pcm_stream {
                stream_name: c"AIF1 Playback".as_ptr(),
                channels_min: 1,
                channels_max: 2,
                rates: RT5682_STEREO_RATES,
                formats: RT5682_FORMATS,
            },
            capture: snd_soc_pcm_stream {
                stream_name: c"AIF1 Capture".as_ptr(),
                channels_min: 1,
                channels_max: 2,
                rates: RT5682_STEREO_RATES,
                formats: RT5682_FORMATS,
            },
            ops: &rt5682_aif1_dai_ops,
        },
        snd_soc_dai_driver {
            name: c"rt5682-aif2".as_ptr(),
            id: RT5682_AIF2,
            playback: core::mem::zeroed(),
            capture: snd_soc_pcm_stream {
                stream_name: c"AIF2 Capture".as_ptr(),
                channels_min: 1,
                channels_max: 2,
                rates: RT5682_STEREO_RATES,
                formats: RT5682_FORMATS,
            },
            ops: &rt5682_aif2_dai_ops,
        },
        snd_soc_dai_driver {
            name: c"rt5682-sdw".as_ptr(),
            id: RT5682_SDW,
            playback: snd_soc_pcm_stream {
                stream_name: c"SDW Playback".as_ptr(),
                channels_min: 1,
                channels_max: 2,
                rates: RT5682_STEREO_RATES,
                formats: RT5682_FORMATS,
            },
            capture: snd_soc_pcm_stream {
                stream_name: c"SDW Capture".as_ptr(),
                channels_min: 1,
                channels_max: 2,
                rates: RT5682_STEREO_RATES,
                formats: RT5682_FORMATS,
            },
            ops: &rt5682_sdw_ops,
        },
    ]
};

unsafe extern "C" fn rt5682_sdw_init(dev: *mut device, regmap: *mut regmap, slave: *mut sdw_slave) -> c_int {
    let rt5682: *mut rt5682_priv;
    let mut ret: c_int;

    rt5682 = devm_kzalloc(dev, size_of::<rt5682_priv>(), GFP_KERNEL) as *mut rt5682_priv;
    if rt5682.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(dev, rt5682 as *mut c_void);
    (*rt5682).slave = slave;
    (*rt5682).sdw_regmap = regmap;
    (*rt5682).is_sdw = true;

    mutex_init(&mut (*rt5682).disable_irq_lock);

    (*rt5682).regmap = devm_regmap_init(dev, ptr::null(), dev, &rt5682_sdw_indirect_regmap);
    if IS_ERR((*rt5682).regmap as *const c_void) {
        ret = PTR_ERR((*rt5682).regmap as *mut c_void);
        dev_err(dev, c"%s: Failed to allocate register map: %d\n".as_ptr(), c"rt5682_sdw_init".as_ptr(), ret);
        return ret;
    }

    ret = rt5682_get_ldo1(rt5682, dev);
    if ret != 0 {
        return ret;
    }

    regcache_cache_only((*rt5682).sdw_regmap, true);
    regcache_cache_only((*rt5682).regmap, true);

    /*
     * Mark hw_init to false
     * HW init will be performed when device reports present
     */
    (*rt5682).hw_init = false;
    (*rt5682).first_hw_init = false;

    mutex_init(&mut (*rt5682).calibrate_mutex);
    INIT_DELAYED_WORK(&mut (*rt5682).jack_detect_work, rt5682_jack_detect_handler);

    ret = devm_snd_soc_register_component(dev, &rt5682_soc_component_dev, rt5682_dai.as_mut_ptr(), rt5682_dai.len() as c_int);
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

    dev_dbg(dev, c"%s\n".as_ptr(), c"rt5682_sdw_init".as_ptr());

    ret
}

unsafe extern "C" fn rt5682_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int {
    let rt5682 = dev_get_drvdata(dev) as *mut rt5682_priv;
    let mut ret: c_int = 0;
    let mut loop_: c_int = 10;
    let mut val: c_uint = 0;

    (*rt5682).disable_irq = false;

    if (*rt5682).hw_init {
        return 0;
    }

    regcache_cache_only((*rt5682).sdw_regmap, false);
    regcache_cache_only((*rt5682).regmap, false);
    if (*rt5682).first_hw_init {
        regcache_cache_bypass((*rt5682).regmap, true);
    }

    /*
     * PM runtime status is marked as 'active' only when a Slave reports as Attached
     */
    if !(*rt5682).first_hw_init {
        /* update count of parent 'active' children */
        pm_runtime_set_active(&mut (*slave).dev);
    }

    pm_runtime_get_noresume(&mut (*slave).dev);

    while loop_ > 0 {
        regmap_read((*rt5682).regmap, RT5682_DEVICE_ID, &mut val);
        if val == DEVICE_ID {
            break;
        }
        dev_warn(dev, c"Device with ID register %x is not rt5682\n".as_ptr(), val);
        usleep_range(30000, 30005);
        loop_ -= 1;
    }

    if val != DEVICE_ID {
        dev_err(dev, c"%s: Device with ID register %x is not rt5682\n".as_ptr(), c"rt5682_io_init".as_ptr(), val);
        ret = -ENODEV;
        goto_out(rt5682, slave, ret, false);
        return ret;
    }

    rt5682_calibrate(rt5682);

    if (*rt5682).first_hw_init {
        regcache_cache_bypass((*rt5682).regmap, false);
        regcache_mark_dirty((*rt5682).regmap);
        ret = regcache_sync((*rt5682).regmap);
        if ret != 0 {
            regcache_cache_bypass((*rt5682).regmap, false);
            regcache_cache_only((*rt5682).sdw_regmap, true);
            regcache_cache_only((*rt5682).regmap, true);
            regcache_mark_dirty((*rt5682).regmap);
            goto_out(rt5682, slave, ret, true);
            return ret;
        }

        /* volatile registers */
        regmap_update_bits((*rt5682).regmap, RT5682_CBJ_CTRL_2, RT5682_EXT_JD_SRC, RT5682_EXT_JD_SRC_MANUAL);

        reinit(rt5682);
        goto_out(rt5682, slave, ret, true);
        return ret;
    }

    rt5682_apply_patch_list(rt5682, dev);

    regmap_write((*rt5682).regmap, RT5682_DEPOP_1, 0x0000);

    regmap_update_bits((*rt5682).regmap, RT5682_PWR_ANLG_1, RT5682_LDO1_DVO_MASK | RT5682_HP_DRIVER_MASK, RT5682_LDO1_DVO_12 | RT5682_HP_DRIVER_5X);
    regmap_write((*rt5682).regmap, RT5682_MICBIAS_2, 0x0080);
    regmap_write((*rt5682).regmap, RT5682_TEST_MODE_CTRL_1, 0x0000);
    regmap_update_bits((*rt5682).regmap, RT5682_BIAS_CUR_CTRL_8, RT5682_HPA_CP_BIAS_CTRL_MASK, RT5682_HPA_CP_BIAS_3UA);
    regmap_update_bits((*rt5682).regmap, RT5682_CHARGE_PUMP_1, RT5682_CP_CLK_HP_MASK, RT5682_CP_CLK_HP_300KHZ);
    regmap_update_bits((*rt5682).regmap, RT5682_HP_CHARGE_PUMP_1, RT5682_PM_HP_MASK, RT5682_PM_HP_HV);

    /* Soundwire */
    regmap_write((*rt5682).regmap, RT5682_PLL2_INTERNAL, 0xa266);
    regmap_write((*rt5682).regmap, RT5682_PLL2_CTRL_1, 0x1700);
    regmap_write((*rt5682).regmap, RT5682_PLL2_CTRL_2, 0x0006);
    regmap_write((*rt5682).regmap, RT5682_PLL2_CTRL_3, 0x2600);
    regmap_write((*rt5682).regmap, RT5682_PLL2_CTRL_4, 0x0c8f);
    regmap_write((*rt5682).regmap, RT5682_PLL_TRACK_2, 0x3000);
    regmap_write((*rt5682).regmap, RT5682_PLL_TRACK_3, 0x4000);
    regmap_update_bits((*rt5682).regmap, RT5682_GLB_CLK, RT5682_SCLK_SRC_MASK | RT5682_PLL2_SRC_MASK, RT5682_SCLK_SRC_PLL2 | RT5682_PLL2_SRC_SDW);

    regmap_update_bits((*rt5682).regmap, RT5682_CBJ_CTRL_2, RT5682_EXT_JD_SRC, RT5682_EXT_JD_SRC_MANUAL);
    regmap_write((*rt5682).regmap, RT5682_CBJ_CTRL_1, 0xd142);
    regmap_update_bits((*rt5682).regmap, RT5682_CBJ_CTRL_5, 0x0700, 0x0600);
    regmap_update_bits((*rt5682).regmap, RT5682_CBJ_CTRL_3, RT5682_CBJ_IN_BUF_EN, RT5682_CBJ_IN_BUF_EN);
    regmap_update_bits((*rt5682).regmap, RT5682_SAR_IL_CMD_1, RT5682_SAR_POW_MASK, RT5682_SAR_POW_EN);
    regmap_update_bits((*rt5682).regmap, RT5682_RC_CLK_CTRL, RT5682_POW_IRQ | RT5682_POW_JDH | RT5682_POW_ANA, RT5682_POW_IRQ | RT5682_POW_JDH | RT5682_POW_ANA);
    regmap_update_bits((*rt5682).regmap, RT5682_PWR_ANLG_2, RT5682_PWR_JDH, RT5682_PWR_JDH);
    regmap_update_bits((*rt5682).regmap, RT5682_IRQ_CTRL_2, RT5682_JD1_EN_MASK | RT5682_JD1_IRQ_MASK, RT5682_JD1_EN | RT5682_JD1_IRQ_PUL);

    reinit(rt5682);
    goto_out(rt5682, slave, ret, true);
    ret
}

unsafe fn reinit(rt5682: *mut rt5682_priv) {
    mod_delayed_work(system_power_efficient_wq, &mut (*rt5682).jack_detect_work, msecs_to_jiffies(250));

    /* Mark Slave initialization complete */
    (*rt5682).hw_init = true;
    (*rt5682).first_hw_init = true;
}

unsafe fn goto_out(rt5682: *mut rt5682_priv, slave: *mut sdw_slave, ret: c_int, _from_reinit: bool_) {
    pm_runtime_put_autosuspend(&mut (*slave).dev);

    dev_dbg(&mut (*slave).dev, c"%s hw_init complete: %d\n".as_ptr(), c"rt5682_io_init".as_ptr(), ret);
}

unsafe extern "C" fn rt5682_sdw_readable_register(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        0x00e0 | 0x00f0 | 0x3000 | 0x3001 | 0x3004 | 0x3005 | 0x3008 => true,
        _ => false,
    }
}

static rt5682_sdw_regmap: regmap_config = regmap_config {
    name: c"sdw".as_ptr(),
    reg_bits: 32,
    val_bits: 8,
    max_register: unsafe { RT5682_I2C_MODE },
    volatile_reg: None,
    readable_reg: Some(rt5682_sdw_readable_register),
    cache_type: REGCACHE_NONE,
    reg_defaults: ptr::null(),
    num_reg_defaults: 0,
    use_single_read: true,
    use_single_write: true,
    reg_read: None,
    reg_write: None,
};

unsafe extern "C" fn rt5682_update_status(slave: *mut sdw_slave, status: sdw_slave_status) -> c_int {
    let rt5682 = dev_get_drvdata(&mut (*slave).dev) as *mut rt5682_priv;

    if status == SDW_SLAVE_UNATTACHED {
        (*rt5682).hw_init = false;
    }

    /*
     * Perform initialization only if slave status is present and
     * hw_init flag is false
     */
    if (*rt5682).hw_init || status != SDW_SLAVE_ATTACHED {
        return 0;
    }

    /* perform I/O transfers required for Slave initialization */
    rt5682_io_init(&mut (*slave).dev, slave)
}

unsafe extern "C" fn rt5682_read_prop(slave: *mut sdw_slave) -> c_int {
    let prop = &mut (*slave).prop;
    let mut nval: c_int;
    let mut i: c_int;
    let mut bit: u32;
    let mut addr: c_ulong;
    let mut dpn: *mut sdw_dpn_prop;

    prop.scp_int1_mask = SDW_SCP_INT1_IMPL_DEF | SDW_SCP_INT1_BUS_CLASH | SDW_SCP_INT1_PARITY;
    prop.quirks = SDW_SLAVE_QUIRKS_INVALID_INITIAL_PARITY;

    prop.paging_support = false;

    /* first we need to allocate memory for set bits in port lists */
    prop.source_ports = 0x4; /* BITMAP: 00000100 */
    prop.sink_ports = 0x2; /* BITMAP: 00000010 */

    nval = hweight32(prop.source_ports);
    prop.src_dpn_prop = devm_kcalloc(&mut (*slave).dev, nval, size_of::<sdw_dpn_prop>(), GFP_KERNEL) as *mut sdw_dpn_prop;
    if prop.src_dpn_prop.is_null() {
        return -ENOMEM;
    }

    i = 0;
    dpn = prop.src_dpn_prop;
    addr = prop.source_ports as c_ulong;
    bit = 0;
    while bit < 32 {
        if (addr & (1 as c_ulong).wrapping_shl(bit)) != 0 {
            (*dpn.offset(i as isize)).num = bit;
            (*dpn.offset(i as isize)).type_ = SDW_DPN_FULL;
            (*dpn.offset(i as isize)).simple_ch_prep_sm = true;
            (*dpn.offset(i as isize)).ch_prep_timeout = 10;
            i += 1;
        }
        bit += 1;
    }

    /* do this again for sink now */
    nval = hweight32(prop.sink_ports);
    prop.sink_dpn_prop = devm_kcalloc(&mut (*slave).dev, nval, size_of::<sdw_dpn_prop>(), GFP_KERNEL) as *mut sdw_dpn_prop;
    if prop.sink_dpn_prop.is_null() {
        return -ENOMEM;
    }

    i = 0;
    dpn = prop.sink_dpn_prop;
    addr = prop.sink_ports as c_ulong;
    bit = 0;
    while bit < 32 {
        if (addr & (1 as c_ulong).wrapping_shl(bit)) != 0 {
            (*dpn.offset(i as isize)).num = bit;
            (*dpn.offset(i as isize)).type_ = SDW_DPN_FULL;
            (*dpn.offset(i as isize)).simple_ch_prep_sm = true;
            (*dpn.offset(i as isize)).ch_prep_timeout = 10;
            i += 1;
        }
        bit += 1;
    }

    /* set the timeout values */
    prop.clk_stop_timeout = 20;

    /* wake-up event */
    prop.wake_capable = 1;

    0
}

unsafe extern "C" fn rt5682_clock_config(dev: *mut device) -> c_int {
    let rt5682 = dev_get_drvdata(dev) as *mut rt5682_priv;
    let clk_freq: c_uint;
    let value: c_uint;

    clk_freq = (*rt5682).params.curr_dr_freq >> 1;

    match clk_freq {
        RT5682_CLK_FREQ_12000000HZ => value = 0x0,
        RT5682_CLK_FREQ_6000000HZ => value = 0x1,
        RT5682_CLK_FREQ_9600000HZ => value = 0x2,
        RT5682_CLK_FREQ_4800000HZ => value = 0x3,
        RT5682_CLK_FREQ_2400000HZ => value = 0x4,
        RT5682_CLK_FREQ_12288000HZ => value = 0x5,
        _ => return -EINVAL,
    }

    regmap_write((*rt5682).sdw_regmap, 0xe0, value);
    regmap_write((*rt5682).sdw_regmap, 0xf0, value);

    dev_dbg(dev, c"%s complete, clk_freq=%d\n".as_ptr(), c"rt5682_clock_config".as_ptr(), clk_freq);

    0
}

unsafe extern "C" fn rt5682_bus_config(slave: *mut sdw_slave, params: *mut sdw_bus_params) -> c_int {
    let rt5682 = dev_get_drvdata(&mut (*slave).dev) as *mut rt5682_priv;
    let ret: c_int;

    memcpy(&mut (*rt5682).params as *mut _ as *mut c_void, params as *const c_void, size_of::<sdw_bus_params>());

    ret = rt5682_clock_config(&mut (*slave).dev);
    if ret < 0 {
        dev_err(&mut (*slave).dev, c"%s: Invalid clk config".as_ptr(), c"rt5682_bus_config".as_ptr());
    }

    ret
}

unsafe extern "C" fn rt5682_interrupt_callback(slave: *mut sdw_slave, status: *mut sdw_slave_intr_status) -> c_int {
    let rt5682 = dev_get_drvdata(&mut (*slave).dev) as *mut rt5682_priv;

    dev_dbg(&mut (*slave).dev, c"%s control_port_stat=%x".as_ptr(), c"rt5682_interrupt_callback".as_ptr(), (*status).control_port);

    mutex_lock(&mut (*rt5682).disable_irq_lock);
    if ((*status).control_port & 0x4) != 0 && !(*rt5682).disable_irq {
        mod_delayed_work(system_power_efficient_wq, &mut (*rt5682).jack_detect_work, msecs_to_jiffies((*rt5682).irq_work_delay_time));
    }
    mutex_unlock(&mut (*rt5682).disable_irq_lock);

    0
}

static rt5682_slave_ops: sdw_slave_ops = sdw_slave_ops {
    read_prop: Some(rt5682_read_prop),
    interrupt_callback: Some(rt5682_interrupt_callback),
    update_status: Some(rt5682_update_status),
    bus_config: Some(rt5682_bus_config),
};

unsafe extern "C" fn rt5682_sdw_probe(slave: *mut sdw_slave, _id: *const sdw_device_id) -> c_int {
    let regmap: *mut regmap;

    /* Regmap Initialization */
    regmap = devm_regmap_init_sdw(slave, &rt5682_sdw_regmap);
    if IS_ERR(regmap as *const c_void) {
        return -EINVAL;
    }

    rt5682_sdw_init(&mut (*slave).dev, regmap, slave)
}

unsafe extern "C" fn rt5682_sdw_remove(slave: *mut sdw_slave) {
    let rt5682 = dev_get_drvdata(&mut (*slave).dev) as *mut rt5682_priv;

    if (*rt5682).hw_init {
        cancel_delayed_work_sync(&mut (*rt5682).jack_detect_work);
    }

    pm_runtime_disable(&mut (*slave).dev);
}

static rt5682_id: [sdw_device_id; 2] = [
    /* SDW_SLAVE_ENTRY_EXT(0x025d, 0x5682, 0x2, 0, 0), */
    sdw_device_id { _private: [] },
    sdw_device_id { _private: [] },
];
/* MODULE_DEVICE_TABLE(sdw, rt5682_id); */

unsafe extern "C" fn rt5682_dev_suspend(dev: *mut device) -> c_int {
    let rt5682 = dev_get_drvdata(dev) as *mut rt5682_priv;

    if !(*rt5682).hw_init {
        return 0;
    }

    cancel_delayed_work_sync(&mut (*rt5682).jack_detect_work);

    regcache_cache_only((*rt5682).sdw_regmap, true);
    regcache_cache_only((*rt5682).regmap, true);
    regcache_mark_dirty((*rt5682).regmap);

    0
}

unsafe extern "C" fn rt5682_dev_system_suspend(dev: *mut device) -> c_int {
    let rt5682 = dev_get_drvdata(dev) as *mut rt5682_priv;
    let slave = dev_to_sdw_dev(dev);
    let ret: c_int;

    if !(*rt5682).hw_init {
        return 0;
    }

    /*
     * prevent new interrupts from being handled after the
     * deferred work completes and before the parent disables
     * interrupts on the link
     */
    mutex_lock(&mut (*rt5682).disable_irq_lock);
    (*rt5682).disable_irq = true;
    ret = sdw_update_no_pm(slave, SDW_SCP_INTMASK1, SDW_SCP_INT1_IMPL_DEF, 0);
    mutex_unlock(&mut (*rt5682).disable_irq_lock);

    if ret < 0 {
        /* log but don't prevent suspend from happening */
        dev_dbg(&mut (*slave).dev, c"%s: could not disable imp-def interrupts\n:".as_ptr(), c"rt5682_dev_system_suspend".as_ptr());
    }

    rt5682_dev_suspend(dev)
}

unsafe extern "C" fn rt5682_dev_resume(dev: *mut device) -> c_int {
    let slave = dev_to_sdw_dev(dev);
    let rt5682 = dev_get_drvdata(dev) as *mut rt5682_priv;
    let mut ret: c_int;

    if !(*rt5682).first_hw_init {
        return 0;
    }

    if !(*slave).unattach_request {
        mutex_lock(&mut (*rt5682).disable_irq_lock);
        if (*rt5682).disable_irq {
            sdw_write_no_pm(slave, SDW_SCP_INTMASK1, SDW_SCP_INT1_IMPL_DEF);
            (*rt5682).disable_irq = false;
        }
        mutex_unlock(&mut (*rt5682).disable_irq_lock);
    }

    ret = sdw_slave_wait_for_init(slave, RT5682_PROBE_TIMEOUT);
    if ret != 0 {
        sdw_show_ping_status((*slave).bus, true);
        return ret;
    }

    regcache_cache_only((*rt5682).sdw_regmap, false);
    regcache_cache_only((*rt5682).regmap, false);
    ret = regcache_sync((*rt5682).regmap);
    if ret != 0 {
        regcache_cache_only((*rt5682).sdw_regmap, true);
        regcache_cache_only((*rt5682).regmap, true);
        regcache_mark_dirty((*rt5682).regmap);
        return ret;
    }

    0
}

static rt5682_pm: dev_pm_ops = dev_pm_ops {
    /* SYSTEM_SLEEP_PM_OPS(rt5682_dev_system_suspend, rt5682_dev_resume) */
    system_suspend: Some(rt5682_dev_system_suspend),
    system_resume: Some(rt5682_dev_resume),
    /* RUNTIME_PM_OPS(rt5682_dev_suspend, rt5682_dev_resume, NULL) */
    runtime_suspend: Some(rt5682_dev_suspend),
    runtime_resume: Some(rt5682_dev_resume),
    runtime_idle: None,
};

static mut rt5682_sdw_driver: sdw_driver = sdw_driver {
    driver: device_driver {
        name: c"rt5682".as_ptr(),
        pm: &rt5682_pm,
    },
    probe: Some(rt5682_sdw_probe),
    remove: Some(rt5682_sdw_remove),
    ops: &rt5682_slave_ops,
    id_table: rt5682_id.as_ptr(),
};
/* module_sdw_driver(rt5682_sdw_driver); */

/* MODULE_DESCRIPTION("ASoC RT5682 driver SDW"); */
/* MODULE_AUTHOR("Oder Chiou <oder_chiou@realtek.com>"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
