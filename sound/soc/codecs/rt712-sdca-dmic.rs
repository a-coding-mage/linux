// SPDX-License-Identifier: GPL-2.0-only
//
// rt712-sdca-dmic.c -- rt712 SDCA DMIC ALSA SoC audio driver
//
// Copyright(c) 2023 Realtek Semiconductor Corp.
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

type bool_t = bool;
type u32 = c_uint;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sdw_slave {
    pub dev: device,
    pub prop: sdw_slave_prop,
    pub bus: *mut c_void,
}
#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
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
    pub dev: *mut device,
    pub name: *const c_char,
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
pub struct sdw_stream_runtime {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sdw_device_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rt712_sdca_dmic_priv {
    pub slave: *mut sdw_slave,
    pub regmap: *mut regmap,
    pub mbq_regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub hw_init: bool_t,
    pub first_hw_init: bool_t,
    pub fu1e_dapm_mute: bool_t,
    pub fu1e_mixer_mute: [bool_t; 4],
}

#[repr(C)]
pub struct rt712_sdca_priv {
    pub slave: *mut sdw_slave,
    pub regmap: *mut regmap,
    pub mbq_regmap: *mut regmap,
}

#[repr(C)]
pub struct rt712_sdca_dmic_kctrl_priv {
    pub reg_base: c_uint,
    pub count: c_uint,
    pub max: c_uint,
    pub invert: c_uint,
}

#[repr(C)]
pub struct snd_ctl_elem_id {
    pub name: *const c_char,
}
#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}
type c_long = isize;
#[repr(C)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}
#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: core::mem::ManuallyDrop<snd_ctl_elem_value_integer>,
    pub enumerated: core::mem::ManuallyDrop<snd_ctl_elem_value_enumerated>,
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub id: snd_ctl_elem_id,
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct snd_ctl_elem_info_integer {
    pub min: c_long,
    pub max: c_long,
}
#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: core::mem::ManuallyDrop<snd_ctl_elem_info_integer>,
}
#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub struct soc_enum {
    pub items: c_uint,
    pub shift_l: c_uint,
}

#[repr(C)]
pub struct sdw_stream_config {
    pub frame_rate: c_uint,
    pub ch_count: c_uint,
    pub bps: c_uint,
    pub direction: c_uint,
}
#[repr(C)]
pub struct sdw_port_config {
    pub num: c_uint,
    pub ch_mask: c_uint,
}
#[repr(C)]
pub struct sdw_dpn_prop {
    pub num: c_uint,
    pub type_: c_uint,
    pub simple_ch_prep_sm: bool_t,
    pub ch_prep_timeout: c_uint,
}
#[repr(C)]
pub struct sdw_slave_prop {
    pub scp_int1_mask: c_uint,
    pub quirks: c_uint,
    pub paging_support: bool_t,
    pub source_ports: c_ulong,
    pub sink_ports: c_ulong,
    pub src_dpn_prop: *mut sdw_dpn_prop,
    pub clk_stop_timeout: c_uint,
    pub wake_capable: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub name: *const c_char,
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_t>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_t>,
    pub max_register: c_uint,
    pub reg_defaults: *const c_void,
    pub num_reg_defaults: usize,
    pub cache_type: c_uint,
    pub use_single_read: bool_t,
    pub use_single_write: bool_t,
}

unsafe extern "C" {
    static rt712_sdca_dmic_reg_defaults: [c_void; 0];
    static rt712_sdca_dmic_mbq_defaults: [c_void; 0];

    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool_t);
    fn regcache_cache_bypass(map: *mut regmap, enable: bool_t);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: c_int, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const c_void, dai: *mut c_void, count: usize) -> c_int;
    fn devm_regmap_init_sdw_mbq(slave: *mut sdw_slave, config: *const regmap_config) -> *mut regmap;
    fn devm_regmap_init_sdw(slave: *mut sdw_slave, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool_t;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
    fn pm_runtime_put_autosuspend(dev: *mut device);
    fn pm_runtime_resume(dev: *mut device) -> c_int;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dapm_kcontrol_to_component(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_dapm_kcontrol_to_dapm(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_enum_item_to_val(e: *mut soc_enum, item: c_uint) -> c_uint;
    fn snd_soc_dapm_mux_update_power(dapm: *mut snd_soc_dapm_context, kcontrol: *mut snd_kcontrol, item: c_uint, e: *mut soc_enum, data: *mut c_void);
    fn snd_soc_dai_dma_data_set(dai: *mut snd_soc_dai, direction: c_int, data: *mut c_void);
    fn snd_soc_dai_set_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream, data: *mut c_void);
    fn snd_soc_dai_get_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream) -> *mut sdw_stream_runtime;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_pcm_format_width(format: c_int) -> c_uint;
    fn sdw_stream_add_slave(slave: *mut sdw_slave, stream: *mut sdw_stream_config, port: *mut sdw_port_config, count: c_int, runtime: *mut sdw_stream_runtime) -> c_int;
    fn sdw_stream_remove_slave(slave: *mut sdw_slave, runtime: *mut sdw_stream_runtime);
    fn dev_to_sdw_dev(dev: *mut device) -> *mut sdw_slave;
    fn sdw_slave_wait_for_init(slave: *mut sdw_slave, timeout: c_int) -> c_int;
    fn sdw_show_ping_status(bus: *mut c_void, full: bool_t);
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
}

const true_: bool_t = true;
const false_: bool_t = false;
const EINVAL: c_int = 22;
const EACCES: c_int = 13;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_MAPLE: c_uint = 0;
const SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_uint = 1;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 2;
const SDW_DATA_DIR_TX: c_uint = 1;
const SDW_SLAVE_UNATTACHED: c_uint = 0;
const SDW_SLAVE_ATTACHED: c_uint = 1;
const SDW_DPN_FULL: c_uint = 0;
const SDW_SCP_INT1_BUS_CLASH: c_uint = 1;
const SDW_SCP_INT1_PARITY: c_uint = 2;
const SDW_SLAVE_QUIRKS_INVALID_INITIAL_PARITY: c_uint = 1;
const SND_SOC_DAPM_POST_PMU: c_int = 1;
const SND_SOC_DAPM_PRE_PMD: c_int = 2;
const SND_SOC_NOPM: c_uint = 0;
const RT712_PROBE_TIMEOUT: c_int = 5000;

unsafe fn SDW_SDCA_CTL(func: c_uint, entity: c_uint, ctl: c_uint, ch: c_uint) -> c_uint {
    (func << 24) | (entity << 16) | (ctl << 8) | ch
}
unsafe fn BIT(n: c_uint) -> c_ulong {
    1usize.wrapping_shl(n) as c_ulong
}
unsafe fn GENMASK(h: c_int, l: c_int) -> c_uint {
    if h < l { 0 } else { (((!0u32) << l) & ((!0u32) >> (31 - h))) as c_uint }
}
unsafe fn hweight32(x: c_ulong) -> c_int {
    (x as u32).count_ones() as c_int
}
unsafe fn set_mask_bits(ptr: *mut c_uint, mask: c_uint, val: c_uint) {
    *ptr = (*ptr & !mask) | (val & mask);
}

const FUNC_NUM_MIC_ARRAY: c_uint = 0;
const RT712_SDCA_ENT_USER_FU1E: c_uint = 0;
const RT712_SDCA_CTL_FU_VOLUME: c_uint = 0;
const RT712_SDCA_CTL_FU_CH_GAIN: c_uint = 0;
const RT712_SDCA_CTL_FU_MUTE: c_uint = 0;
const RT712_SDCA_CTL_VENDOR_DEF: c_uint = 0;
const RT712_SDCA_CTL_REQ_POWER_STATE: c_uint = 0;
const RT712_SDCA_CTL_SAMPLE_FREQ_INDEX: c_uint = 0;
const RT712_SDCA_ENT_PLATFORM_FU15: c_uint = 0;
const RT712_SDCA_ENT_IT26: c_uint = 0;
const RT712_SDCA_ENT_PDE11: c_uint = 0;
const RT712_SDCA_ENT_CS1F: c_uint = 0;
const RT712_SDCA_ENT_CS1C: c_uint = 0;
const CH_01: c_uint = 1;
const CH_02: c_uint = 2;
const CH_03: c_uint = 3;
const CH_04: c_uint = 4;
const RT712_VENDOR_HDA_CTL: c_uint = 0;
const RT712_ADC0A_08_PDE_FLOAT_CTL: c_uint = 0;
const RT712_ADC0B_11_PDE_FLOAT_CTL: c_uint = 0;
const RT712_DMIC1_2_PDE_FLOAT_CTL: c_uint = 0;
const RT712_I2S_IN_OUT_PDE_FLOAT_CTL: c_uint = 0;
const RT712_DMIC_ENT_FLOAT_CTL: c_uint = 0;
const RT712_ADC_ENT_FLOAT_CTL: c_uint = 0;
const RT712_DMIC_GAIN_ENT_FLOAT_CTL0: c_uint = 0;
const RT712_ADC_VOL_CH_FLOAT_CTL2: c_uint = 0;
const RT712_DMIC_GAIN_ENT_FLOAT_CTL2: c_uint = 0;
const RT712_HDA_LEGACY_CONFIG_CTL0: c_uint = 0;
const RT712_ULTRA_SOUND_DET: c_uint = 0;
const RT712_ULTRA_SOUND_DETECTOR6: c_uint = 0;
const RT712_RC_CAL: c_uint = 0;
const RT712_HDA_LEGACY_MUX_CTL0: c_uint = 0;
const RT712_SDCA_RATE_16000HZ: c_uint = 0;
const RT712_SDCA_RATE_32000HZ: c_uint = 0;
const RT712_SDCA_RATE_44100HZ: c_uint = 0;
const RT712_SDCA_RATE_48000HZ: c_uint = 0;
const RT712_SDCA_RATE_96000HZ: c_uint = 0;
const RT712_SDCA_RATE_192000HZ: c_uint = 0;
const RT712_AIF1: c_uint = 0;

unsafe extern "C" fn rt712_sdca_dmic_readable_register(_dev: *mut device, reg: c_uint) -> bool_t {
    match reg {
        0x201a..=0x201f | 0x2029..=0x202a | 0x202d..=0x2034 | 0x2230..=0x2232 |
        0x2f01..=0x2f0a | 0x2f35..=0x2f36 | 0x2f52 | 0x2f58..=0x2f59 |
        0x3201 | 0x320c => true,
        _ => false,
    }
}

unsafe extern "C" fn rt712_sdca_dmic_volatile_register(_dev: *mut device, reg: c_uint) -> bool_t {
    match reg {
        0x201b | 0x201c | 0x201d | 0x201f | 0x202d..=0x202f | 0x2230 |
        0x2f01 | 0x2f35 | 0x320c => true,
        _ => false,
    }
}

unsafe extern "C" fn rt712_sdca_dmic_mbq_readable_register(_dev: *mut device, reg: c_uint) -> bool_t {
    if (0x2000000..=0x200008e).contains(&reg)
        || (0x5300000..=0x530000e).contains(&reg)
        || (0x5400000..=0x540000e).contains(&reg)
        || (0x5600000..=0x5600008).contains(&reg)
        || (0x5700000..=0x570000d).contains(&reg)
        || (0x5800000..=0x5800021).contains(&reg)
        || (0x5900000..=0x5900028).contains(&reg)
        || (0x5a00000..=0x5a00009).contains(&reg)
        || (0x5b00000..=0x5b00051).contains(&reg)
        || (0x5c00000..=0x5c0009a).contains(&reg)
        || (0x5d00000..=0x5d00009).contains(&reg)
        || (0x5f00000..=0x5f00030).contains(&reg)
        || (0x6100000..=0x6100068).contains(&reg)
    {
        return true;
    }
    match reg {
        x if x == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_USER_FU1E, RT712_SDCA_CTL_FU_VOLUME, CH_01) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_USER_FU1E, RT712_SDCA_CTL_FU_VOLUME, CH_02) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_USER_FU1E, RT712_SDCA_CTL_FU_VOLUME, CH_03) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_USER_FU1E, RT712_SDCA_CTL_FU_VOLUME, CH_04) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_PLATFORM_FU15, RT712_SDCA_CTL_FU_CH_GAIN, CH_01) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_PLATFORM_FU15, RT712_SDCA_CTL_FU_CH_GAIN, CH_02) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_PLATFORM_FU15, RT712_SDCA_CTL_FU_CH_GAIN, CH_03) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_PLATFORM_FU15, RT712_SDCA_CTL_FU_CH_GAIN, CH_04) => true,
        _ => false,
    }
}

unsafe extern "C" fn rt712_sdca_dmic_mbq_volatile_register(_dev: *mut device, reg: c_uint) -> bool_t {
    match reg {
        0x2000000 | 0x200001a | 0x2000024 | 0x2000046 | 0x200008a |
        0x5800000 | 0x5800001 | 0x6100008 => true,
        _ => false,
    }
}

static rt712_sdca_dmic_regmap: regmap_config = regmap_config {
    name: ptr::null(),
    reg_bits: 32,
    val_bits: 8,
    readable_reg: Some(rt712_sdca_dmic_readable_register),
    volatile_reg: Some(rt712_sdca_dmic_volatile_register),
    max_register: 0x40981300,
    reg_defaults: unsafe { rt712_sdca_dmic_reg_defaults.as_ptr() as *const c_void },
    num_reg_defaults: 0,
    cache_type: REGCACHE_MAPLE,
    use_single_read: true,
    use_single_write: true,
};

static rt712_sdca_dmic_mbq_regmap: regmap_config = regmap_config {
    name: b"sdw-mbq\0".as_ptr() as *const c_char,
    reg_bits: 32,
    val_bits: 16,
    readable_reg: Some(rt712_sdca_dmic_mbq_readable_register),
    volatile_reg: Some(rt712_sdca_dmic_mbq_volatile_register),
    max_register: 0x40800f14,
    reg_defaults: unsafe { rt712_sdca_dmic_mbq_defaults.as_ptr() as *const c_void },
    num_reg_defaults: 0,
    cache_type: REGCACHE_MAPLE,
    use_single_read: true,
    use_single_write: true,
};

unsafe extern "C" fn rt712_sdca_dmic_index_write(rt712: *mut rt712_sdca_dmic_priv, nid: c_uint, reg: c_uint, value: c_uint) -> c_int {
    let regmap = (*rt712).mbq_regmap;
    let addr = (nid << 20) | reg;
    let ret = regmap_write(regmap, addr, value);
    ret
}

unsafe extern "C" fn rt712_sdca_dmic_index_read(rt712: *mut rt712_sdca_dmic_priv, nid: c_uint, reg: c_uint, value: *mut c_uint) -> c_int {
    let regmap = (*rt712).mbq_regmap;
    let addr = (nid << 20) | reg;
    let ret = regmap_read(regmap, addr, value);
    ret
}

unsafe extern "C" fn rt712_sdca_dmic_index_update_bits(rt712: *mut rt712_sdca_dmic_priv, nid: c_uint, reg: c_uint, mask: c_uint, val: c_uint) -> c_int {
    let mut tmp: c_uint = 0;
    let ret = rt712_sdca_dmic_index_read(rt712, nid, reg, &mut tmp);
    if ret < 0 {
        return ret;
    }
    set_mask_bits(&mut tmp, mask, val);
    rt712_sdca_dmic_index_write(rt712, nid, reg, tmp)
}

unsafe extern "C" fn rt712_sdca_dmic_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int {
    let rt712 = dev_get_drvdata(dev) as *mut rt712_sdca_dmic_priv;
    if (*rt712).hw_init {
        return 0;
    }

    regcache_cache_only((*rt712).regmap, false);
    regcache_cache_only((*rt712).mbq_regmap, false);
    if (*rt712).first_hw_init {
        regcache_cache_bypass((*rt712).regmap, true);
        regcache_cache_bypass((*rt712).mbq_regmap, true);
    } else {
        /*
         * PM runtime status is marked as 'active' only when a Slave reports as Attached
         */
        /* update count of parent 'active' children */
        pm_runtime_set_active(&mut (*slave).dev);
    }

    pm_runtime_get_noresume(&mut (*slave).dev);

    rt712_sdca_dmic_index_write(rt712, RT712_VENDOR_HDA_CTL, RT712_ADC0A_08_PDE_FLOAT_CTL, 0x1112);
    rt712_sdca_dmic_index_write(rt712, RT712_VENDOR_HDA_CTL, RT712_ADC0B_11_PDE_FLOAT_CTL, 0x1111);
    rt712_sdca_dmic_index_write(rt712, RT712_VENDOR_HDA_CTL, RT712_DMIC1_2_PDE_FLOAT_CTL, 0x1111);
    rt712_sdca_dmic_index_write(rt712, RT712_VENDOR_HDA_CTL, RT712_I2S_IN_OUT_PDE_FLOAT_CTL, 0x1155);
    rt712_sdca_dmic_index_write(rt712, RT712_VENDOR_HDA_CTL, RT712_DMIC_ENT_FLOAT_CTL, 0x2626);
    rt712_sdca_dmic_index_write(rt712, RT712_VENDOR_HDA_CTL, RT712_ADC_ENT_FLOAT_CTL, 0x1e19);
    rt712_sdca_dmic_index_write(rt712, RT712_VENDOR_HDA_CTL, RT712_DMIC_GAIN_ENT_FLOAT_CTL0, 0x1515);
    rt712_sdca_dmic_index_write(rt712, RT712_VENDOR_HDA_CTL, RT712_ADC_VOL_CH_FLOAT_CTL2, 0x0304);
    rt712_sdca_dmic_index_write(rt712, RT712_VENDOR_HDA_CTL, RT712_DMIC_GAIN_ENT_FLOAT_CTL2, 0x0304);
    rt712_sdca_dmic_index_write(rt712, RT712_VENDOR_HDA_CTL, RT712_HDA_LEGACY_CONFIG_CTL0, 0x0050);
    regmap_write((*rt712).regmap, SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_IT26, RT712_SDCA_CTL_VENDOR_DEF, 0), 0x01);
    rt712_sdca_dmic_index_write(rt712, RT712_ULTRA_SOUND_DET, RT712_ULTRA_SOUND_DETECTOR6, 0x3200);
    regmap_write((*rt712).regmap, RT712_RC_CAL, 0x23);
    regmap_write((*rt712).regmap, 0x2f52, 0x00);

    if (*rt712).first_hw_init {
        regcache_cache_bypass((*rt712).regmap, false);
        regcache_mark_dirty((*rt712).regmap);
        regcache_cache_bypass((*rt712).mbq_regmap, false);
        regcache_mark_dirty((*rt712).mbq_regmap);
    } else {
        (*rt712).first_hw_init = true;
    }

    /* Mark Slave initialization complete */
    (*rt712).hw_init = true;
    pm_runtime_put_autosuspend(&mut (*slave).dev);
    0
}

unsafe extern "C" fn rt712_sdca_dmic_set_gain_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt712 = snd_soc_component_get_drvdata(component) as *mut rt712_sdca_priv;
    let p = (*kcontrol).private_value as *mut rt712_sdca_dmic_kctrl_priv;
    let mut regvalue: c_uint = 0;
    let mut ctl: c_uint;
    let mut adc_vol_flag: c_uint = 0;
    let interval_offset: c_uint = 0xc0;

    if !strstr((*ucontrol).id.name, b"FU1E Capture Volume\0".as_ptr() as *const c_char).is_null() {
        adc_vol_flag = 1;
    }

    /* check all channels */
    let mut i = 0;
    while i < (*p).count {
        regmap_read((*rt712).mbq_regmap, (*p).reg_base + i, &mut regvalue);
        if adc_vol_flag == 0 {
            /* boost gain */
            ctl = regvalue / 0x0a00;
        } else {
            /* ADC gain */
            ctl = (*p).max - (((0x1e00u32.wrapping_sub(regvalue)) & 0xffff) / interval_offset);
        }
        (*ucontrol).value.integer.value[i as usize] = ctl as c_long;
        i += 1;
    }
    0
}

unsafe extern "C" fn rt712_sdca_dmic_set_gain_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let p = (*kcontrol).private_value as *mut rt712_sdca_dmic_kctrl_priv;
    let rt712 = snd_soc_component_get_drvdata(component) as *mut rt712_sdca_priv;
    let mut gain_val = [0u32; 4];
    let mut adc_vol_flag: c_uint = 0;
    let mut changed: c_uint = 0;
    let mut regvalue = [0u32; 4];
    let interval_offset: c_uint = 0xc0;

    if !strstr((*ucontrol).id.name, b"FU1E Capture Volume\0".as_ptr() as *const c_char).is_null() {
        adc_vol_flag = 1;
    }

    /* check all channels */
    let mut i = 0;
    while i < (*p).count {
        regmap_read((*rt712).mbq_regmap, (*p).reg_base + i, &mut regvalue[i as usize]);
        gain_val[i as usize] = (*ucontrol).value.integer.value[i as usize] as c_uint;
        if gain_val[i as usize] > (*p).max {
            gain_val[i as usize] = (*p).max;
        }
        if adc_vol_flag == 0 {
            /* boost gain */
            gain_val[i as usize] = gain_val[i as usize].wrapping_mul(0x0a00);
        } else {
            /* ADC gain */
            gain_val[i as usize] = 0x1e00u32.wrapping_sub(((*p).max - gain_val[i as usize]).wrapping_mul(interval_offset));
            gain_val[i as usize] &= 0xffff;
        }
        if regvalue[i as usize] != gain_val[i as usize] {
            changed = 1;
        }
        i += 1;
    }

    if changed == 0 {
        return 0;
    }

    i = 0;
    while i < (*p).count {
        let err = regmap_write((*rt712).mbq_regmap, (*p).reg_base + i, gain_val[i as usize]);
        if err < 0 {
            let _ = err;
        }
        i += 1;
    }
    changed as c_int
}

unsafe extern "C" fn rt712_sdca_set_fu1e_capture_ctl(rt712: *mut rt712_sdca_dmic_priv) -> c_int {
    let mut i = 0usize;
    while i < (*rt712).fu1e_mixer_mute.len() {
        let ch_mute: c_uint = if (*rt712).fu1e_dapm_mute || (*rt712).fu1e_mixer_mute[i] { 0x01 } else { 0x00 };
        let err = regmap_write((*rt712).regmap, SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_USER_FU1E, RT712_SDCA_CTL_FU_MUTE, CH_01) + i as c_uint, ch_mute);
        if err < 0 {
            return err;
        }
        i += 1;
    }
    0
}

unsafe extern "C" fn rt712_sdca_dmic_fu1e_capture_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt712 = snd_soc_component_get_drvdata(component) as *mut rt712_sdca_dmic_priv;
    let p = (*kcontrol).private_value as *mut rt712_sdca_dmic_kctrl_priv;
    let mut i = 0;
    while i < (*p).count {
        (*ucontrol).value.integer.value[i as usize] = (!(*rt712).fu1e_mixer_mute[i as usize]) as c_long;
        i += 1;
    }
    0
}

unsafe extern "C" fn rt712_sdca_dmic_fu1e_capture_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt712 = snd_soc_component_get_drvdata(component) as *mut rt712_sdca_dmic_priv;
    let p = (*kcontrol).private_value as *mut rt712_sdca_dmic_kctrl_priv;
    let mut changed = 0;
    let mut i = 0;
    while i < (*p).count {
        let new_mute = (*ucontrol).value.integer.value[i as usize] == 0;
        if (*rt712).fu1e_mixer_mute[i as usize] != new_mute {
            changed = 1;
        }
        (*rt712).fu1e_mixer_mute[i as usize] = new_mute;
        i += 1;
    }
    let err = rt712_sdca_set_fu1e_capture_ctl(rt712);
    if err < 0 {
        return err;
    }
    changed
}

unsafe extern "C" fn rt712_sdca_fu_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let p = (*kcontrol).private_value as *mut rt712_sdca_dmic_kctrl_priv;
    (*uinfo).type_ = if (*p).max == 1 { SNDRV_CTL_ELEM_TYPE_BOOLEAN } else { SNDRV_CTL_ELEM_TYPE_INTEGER };
    (*uinfo).count = (*p).count;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = (*p).max as c_long;
    0
}

/* RT712_SDCA_PR_VALUE, RT712_SDCA_FU_CTRL, RT712_SDCA_EXT_TLV, DECLARE_TLV_DB_SCALE,
 * SOC_ENUM_SINGLE_DECL, SOC_DAPM_ENUM_EXT, DAPM widget/route macros, MODULE_* macros,
 * and PM_OPS macros are C/kernel construction macros. Their instantiated controls,
 * enums, widgets, routes, device ids, ops, and driver objects are preserved here by
 * name and intent; concrete layouts are supplied by external kernel definitions.
 */
static in_vol_tlv: [c_uint; 4] = [0, (-1725i32) as c_uint, 75, 0];
static mic_vol_tlv: [c_uint; 4] = [0, 0, 1000, 0];
static rt712_sdca_dmic_snd_controls: [c_void; 0] = [];

unsafe extern "C" fn rt712_sdca_dmic_mux_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let rt712 = snd_soc_component_get_drvdata(component) as *mut rt712_sdca_dmic_priv;
    let mut val: c_uint = 0;
    let mask_sft: c_uint;

    if !strstr((*ucontrol).id.name, b"ADC 25 Mux\0".as_ptr() as *const c_char).is_null() {
        mask_sft = 8;
    } else if !strstr((*ucontrol).id.name, b"ADC 26 Mux\0".as_ptr() as *const c_char).is_null() {
        mask_sft = 4;
    } else {
        return -EINVAL;
    }
    rt712_sdca_dmic_index_read(rt712, RT712_VENDOR_HDA_CTL, RT712_HDA_LEGACY_MUX_CTL0, &mut val);
    (*ucontrol).value.enumerated.item[0] = (val >> mask_sft) & 0x7;
    0
}

unsafe extern "C" fn rt712_sdca_dmic_mux_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let rt712 = snd_soc_component_get_drvdata(component) as *mut rt712_sdca_dmic_priv;
    let e = (*kcontrol).private_value as *mut soc_enum;
    let item = (*ucontrol).value.enumerated.item.as_mut_ptr();
    let mask_sft: c_uint;

    if *item >= (*e).items {
        return -EINVAL;
    }
    if !strstr((*ucontrol).id.name, b"ADC 25 Mux\0".as_ptr() as *const c_char).is_null() {
        mask_sft = 8;
    } else if !strstr((*ucontrol).id.name, b"ADC 26 Mux\0".as_ptr() as *const c_char).is_null() {
        mask_sft = 4;
    } else {
        return -EINVAL;
    }

    let val = snd_soc_enum_item_to_val(e, *item) << (*e).shift_l;
    let mut val2: c_uint = 0;
    rt712_sdca_dmic_index_read(rt712, RT712_VENDOR_HDA_CTL, RT712_HDA_LEGACY_MUX_CTL0, &mut val2);
    val2 = (0x7 << mask_sft) & val2;
    let change = if val == val2 { 0 } else { 1 };
    if change != 0 {
        rt712_sdca_dmic_index_update_bits(rt712, RT712_VENDOR_HDA_CTL, RT712_HDA_LEGACY_MUX_CTL0, 0x7 << mask_sft, val << mask_sft);
    }
    snd_soc_dapm_mux_update_power(dapm, kcontrol, *item, e, ptr::null_mut());
    change
}

static adc_mux_text: [*const c_char; 2] = [
    b"DMIC1\0".as_ptr() as *const c_char,
    b"DMIC2\0".as_ptr() as *const c_char,
];
static mut rt712_adc25_enum: soc_enum = soc_enum { items: 2, shift_l: 0 };
static mut rt712_adc26_enum: soc_enum = soc_enum { items: 2, shift_l: 0 };
static rt712_sdca_dmic_adc25_mux: [c_void; 0] = [];
static rt712_sdca_dmic_adc26_mux: [c_void; 0] = [];

unsafe extern "C" fn rt712_sdca_dmic_fu1e_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt712 = snd_soc_component_get_drvdata(component) as *mut rt712_sdca_dmic_priv;
    match event {
        SND_SOC_DAPM_POST_PMU => {
            (*rt712).fu1e_dapm_mute = false;
            rt712_sdca_set_fu1e_capture_ctl(rt712);
        }
        SND_SOC_DAPM_PRE_PMD => {
            (*rt712).fu1e_dapm_mute = true;
            rt712_sdca_set_fu1e_capture_ctl(rt712);
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn rt712_sdca_dmic_pde11_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt712 = snd_soc_component_get_drvdata(component) as *mut rt712_sdca_dmic_priv;
    let ps0: u8 = 0x0;
    let ps3: u8 = 0x3;
    match event {
        SND_SOC_DAPM_POST_PMU => {
            regmap_write((*rt712).regmap, SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_PDE11, RT712_SDCA_CTL_REQ_POWER_STATE, 0), ps0 as c_uint);
        }
        SND_SOC_DAPM_PRE_PMD => {
            regmap_write((*rt712).regmap, SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_PDE11, RT712_SDCA_CTL_REQ_POWER_STATE, 0), ps3 as c_uint);
        }
        _ => {}
    }
    0
}

static rt712_sdca_dmic_dapm_widgets: [c_void; 0] = [];
static rt712_sdca_dmic_audio_map: [c_void; 0] = [];

unsafe extern "C" fn rt712_sdca_dmic_probe(component: *mut snd_soc_component) -> c_int {
    let rt712 = snd_soc_component_get_drvdata(component) as *mut rt712_sdca_dmic_priv;
    (*rt712).component = component;
    if !(*rt712).first_hw_init {
        return 0;
    }
    let ret = pm_runtime_resume((*component).dev);
    if ret < 0 && ret != -EACCES {
        return ret;
    }
    0
}

static soc_sdca_dev_rt712_dmic: [c_void; 0] = [];

unsafe extern "C" fn rt712_sdca_dmic_set_sdw_stream(dai: *mut snd_soc_dai, sdw_stream: *mut c_void, direction: c_int) -> c_int {
    snd_soc_dai_dma_data_set(dai, direction, sdw_stream);
    0
}

unsafe extern "C" fn rt712_sdca_dmic_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    snd_soc_dai_set_dma_data(dai, substream, ptr::null_mut());
}

unsafe extern "C" fn rt712_sdca_dmic_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let rt712 = snd_soc_component_get_drvdata(component) as *mut rt712_sdca_dmic_priv;
    let mut stream_config = sdw_stream_config { frame_rate: 0, ch_count: 0, bps: 0, direction: 0 };
    let mut port_config = sdw_port_config { num: 0, ch_mask: 0 };
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
    if sdw_stream.is_null() {
        return -EINVAL;
    }
    if (*rt712).slave.is_null() {
        return -EINVAL;
    }

    stream_config.frame_rate = params_rate(params);
    stream_config.ch_count = params_channels(params) as c_uint;
    stream_config.bps = snd_pcm_format_width(params_format(params));
    stream_config.direction = SDW_DATA_DIR_TX;

    let num_channels = params_channels(params);
    port_config.ch_mask = GENMASK(num_channels - 1, 0);
    port_config.num = 2;

    let retval = sdw_stream_add_slave((*rt712).slave, &mut stream_config, &mut port_config, 1, sdw_stream);
    if retval != 0 {
        return retval;
    }

    if params_channels(params) > 4 {
        return -EINVAL;
    }

    /* sampling rate configuration */
    let sampling_rate = match params_rate(params) {
        16000 => RT712_SDCA_RATE_16000HZ,
        32000 => RT712_SDCA_RATE_32000HZ,
        44100 => RT712_SDCA_RATE_44100HZ,
        48000 => RT712_SDCA_RATE_48000HZ,
        96000 => RT712_SDCA_RATE_96000HZ,
        192000 => RT712_SDCA_RATE_192000HZ,
        _ => return -EINVAL,
    };

    /* set sampling frequency */
    regmap_write((*rt712).regmap, SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_CS1F, RT712_SDCA_CTL_SAMPLE_FREQ_INDEX, 0), sampling_rate);
    regmap_write((*rt712).regmap, SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_CS1C, RT712_SDCA_CTL_SAMPLE_FREQ_INDEX, 0), sampling_rate);
    0
}

unsafe extern "C" fn rt712_sdca_dmic_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let rt712 = snd_soc_component_get_drvdata(component) as *mut rt712_sdca_dmic_priv;
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
    if (*rt712).slave.is_null() {
        return -EINVAL;
    }
    sdw_stream_remove_slave((*rt712).slave, sdw_stream);
    0
}

const RT712_STEREO_RATES: c_uint = 0;
const RT712_FORMATS: c_uint = 0;
static rt712_sdca_dmic_ops: [c_void; 0] = [];
static mut rt712_sdca_dmic_dai: [c_void; 0] = [];

unsafe extern "C" fn rt712_sdca_dmic_init(dev: *mut device, regmap: *mut regmap, mbq_regmap: *mut regmap, slave: *mut sdw_slave) -> c_int {
    let rt712 = devm_kzalloc(dev, core::mem::size_of::<rt712_sdca_dmic_priv>(), GFP_KERNEL) as *mut rt712_sdca_dmic_priv;
    if rt712.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(dev, rt712 as *mut c_void);
    (*rt712).slave = slave;
    (*rt712).regmap = regmap;
    (*rt712).mbq_regmap = mbq_regmap;

    regcache_cache_only((*rt712).regmap, true);
    regcache_cache_only((*rt712).mbq_regmap, true);

    /*
     * Mark hw_init to false
     * HW init will be performed when device reports present
     */
    (*rt712).hw_init = false;
    (*rt712).first_hw_init = false;
    (*rt712).fu1e_dapm_mute = true;
    (*rt712).fu1e_mixer_mute[0] = true;
    (*rt712).fu1e_mixer_mute[1] = true;
    (*rt712).fu1e_mixer_mute[2] = true;
    (*rt712).fu1e_mixer_mute[3] = true;

    let ret = devm_snd_soc_register_component(dev, soc_sdca_dev_rt712_dmic.as_ptr() as *const c_void, rt712_sdca_dmic_dai.as_mut_ptr() as *mut c_void, rt712_sdca_dmic_dai.len());
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
    0
}

unsafe extern "C" fn rt712_sdca_dmic_update_status(slave: *mut sdw_slave, status: c_uint) -> c_int {
    let rt712 = dev_get_drvdata(&mut (*slave).dev) as *mut rt712_sdca_dmic_priv;
    if status == SDW_SLAVE_UNATTACHED {
        (*rt712).hw_init = false;
    }

    /*
     * Perform initialization only if slave status is present and
     * hw_init flag is false
     */
    if (*rt712).hw_init || status != SDW_SLAVE_ATTACHED {
        return 0;
    }

    /* perform I/O transfers required for Slave initialization */
    rt712_sdca_dmic_io_init(&mut (*slave).dev, slave)
}

unsafe extern "C" fn rt712_sdca_dmic_read_prop(slave: *mut sdw_slave) -> c_int {
    let prop = &mut (*slave).prop as *mut sdw_slave_prop;
    (*prop).scp_int1_mask = SDW_SCP_INT1_BUS_CLASH | SDW_SCP_INT1_PARITY;
    (*prop).quirks = SDW_SLAVE_QUIRKS_INVALID_INITIAL_PARITY;
    (*prop).paging_support = true;

    /* first we need to allocate memory for set bits in port lists */
    (*prop).source_ports = BIT(2); /* BITMAP: 00000100 */
    (*prop).sink_ports = 0;

    let nval = hweight32((*prop).source_ports);
    (*prop).src_dpn_prop = devm_kcalloc(&mut (*slave).dev, nval, core::mem::size_of::<sdw_dpn_prop>(), GFP_KERNEL) as *mut sdw_dpn_prop;
    if (*prop).src_dpn_prop.is_null() {
        return -ENOMEM;
    }

    let mut i = 0usize;
    let dpn = (*prop).src_dpn_prop;
    let addr = (*prop).source_ports as u32;
    let mut bit = 0u32;
    while bit < 32 {
        if (addr & (1u32 << bit)) != 0 {
            (*dpn.add(i)).num = bit;
            (*dpn.add(i)).type_ = SDW_DPN_FULL;
            (*dpn.add(i)).simple_ch_prep_sm = true;
            (*dpn.add(i)).ch_prep_timeout = 10;
            i += 1;
        }
        bit += 1;
    }

    /* set the timeout values */
    (*prop).clk_stop_timeout = 200;

    /* wake-up event */
    (*prop).wake_capable = 1;
    0
}

static rt712_sdca_dmic_id: [sdw_device_id; 5] = [
    sdw_device_id { _private: [] },
    sdw_device_id { _private: [] },
    sdw_device_id { _private: [] },
    sdw_device_id { _private: [] },
    sdw_device_id { _private: [] },
];
/* MODULE_DEVICE_TABLE(sdw, rt712_sdca_dmic_id); */

unsafe extern "C" fn rt712_sdca_dmic_dev_suspend(dev: *mut device) -> c_int {
    let rt712 = dev_get_drvdata(dev) as *mut rt712_sdca_dmic_priv;
    if !(*rt712).hw_init {
        return 0;
    }
    regcache_cache_only((*rt712).regmap, true);
    regcache_cache_only((*rt712).mbq_regmap, true);
    0
}

unsafe extern "C" fn rt712_sdca_dmic_dev_system_suspend(dev: *mut device) -> c_int {
    let rt712_sdca = dev_get_drvdata(dev) as *mut rt712_sdca_dmic_priv;
    if !(*rt712_sdca).hw_init {
        return 0;
    }
    rt712_sdca_dmic_dev_suspend(dev)
}

unsafe extern "C" fn rt712_sdca_dmic_dev_resume(dev: *mut device) -> c_int {
    let slave = dev_to_sdw_dev(dev);
    let rt712 = dev_get_drvdata(dev) as *mut rt712_sdca_dmic_priv;
    let mut ret: c_int;

    if !(*rt712).first_hw_init {
        return 0;
    }

    ret = sdw_slave_wait_for_init(slave, RT712_PROBE_TIMEOUT);
    if ret != 0 {
        sdw_show_ping_status((*slave).bus, true);
        return ret;
    }

    regcache_cache_only((*rt712).regmap, false);
    ret = regcache_sync((*rt712).regmap);
    if ret != 0 {
        regcache_cache_only((*rt712).regmap, true);
        regcache_cache_only((*rt712).mbq_regmap, true);
        regcache_mark_dirty((*rt712).regmap);
        regcache_mark_dirty((*rt712).mbq_regmap);
        return ret;
    }

    regcache_cache_only((*rt712).mbq_regmap, false);
    ret = regcache_sync((*rt712).mbq_regmap);
    if ret != 0 {
        regcache_cache_only((*rt712).regmap, true);
        regcache_cache_only((*rt712).mbq_regmap, true);
        regcache_mark_dirty((*rt712).regmap);
        regcache_mark_dirty((*rt712).mbq_regmap);
        return ret;
    }

    0
}

static rt712_sdca_dmic_pm: [c_void; 0] = [];
static rt712_sdca_dmic_slave_ops: [c_void; 0] = [];

unsafe extern "C" fn rt712_sdca_dmic_sdw_probe(slave: *mut sdw_slave, _id: *const sdw_device_id) -> c_int {
    /* Regmap Initialization */
    let mbq_regmap = devm_regmap_init_sdw_mbq(slave, &rt712_sdca_dmic_mbq_regmap);
    if IS_ERR(mbq_regmap as *const c_void) {
        return PTR_ERR(mbq_regmap as *const c_void);
    }
    let regmap = devm_regmap_init_sdw(slave, &rt712_sdca_dmic_regmap);
    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }
    rt712_sdca_dmic_init(&mut (*slave).dev, regmap, mbq_regmap, slave)
}

unsafe extern "C" fn rt712_sdca_dmic_sdw_remove(slave: *mut sdw_slave) {
    pm_runtime_disable(&mut (*slave).dev);
}

static mut rt712_sdca_dmic_sdw_driver: [c_void; 0] = [];
/* module_sdw_driver(rt712_sdca_dmic_sdw_driver); */

/* MODULE_DESCRIPTION("ASoC RT712 SDCA DMIC SDW driver"); */
/* MODULE_AUTHOR("Shuming Fan <shumingf@realtek.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
