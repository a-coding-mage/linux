// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2015-2021, The Linux Foundation. All rights reserved.
 *
 * Rust source-level translation of soc/codecs/wsa883x.c.
 * Linux kernel, ALSA SoC, SoundWire, regmap, hwmon, PM, GPIO, reset,
 * regulator, and macro-provided items are external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

type bool_ = bool;
type u32 = u32;
type umode_t = u16;

const fn BIT(n: u32) -> u32 { 1u32 << n }
const fn GENMASK(h: u32, l: u32) -> u32 { ((!0u32) << l) & ((!0u32) >> (31 - h)) }
const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize { N }
const fn FIELD_PREP(mask: u32, val: u32) -> u32 { (val << (mask.trailing_zeros())) & mask }
fn in_range(val: c_long, start: c_long, range: c_int) -> bool { val >= start && val < start + range as c_long }

#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct device { pub of_node: *mut c_void }
#[repr(C)] pub struct regulator { _private: [u8; 0] }
#[repr(C)] pub struct sdw_stream_runtime { _private: [u8; 0] }
#[repr(C)] pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)] pub struct reset_control { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { pub private_value: usize }
#[repr(C)] pub struct snd_soc_component { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { pub dapm: *mut snd_soc_dapm_context }
#[repr(C)] pub struct snd_soc_dai { pub dev: *mut device, pub component: *mut snd_soc_component }
#[repr(C)] pub struct sdw_device_id { _private: [u8; 0] }
#[repr(C)] pub struct soc_enum { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_route { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget_def { _private: [u8; 0] }
#[repr(C)] pub struct hwmon_channel_info { _private: [u8; 0] }
#[repr(C)] pub struct hwmon_chip_info { pub ops: *const hwmon_ops, pub info: *const *const hwmon_channel_info }
#[repr(C)] pub struct hwmon_ops { pub is_visible: Option<unsafe extern "C" fn(*const c_void, hwmon_sensor_types, u32, c_int) -> umode_t>, pub read: Option<unsafe extern "C" fn(*mut device, hwmon_sensor_types, u32, c_int, *mut c_long) -> c_int> }
#[repr(C)] pub struct soc_mixer_control { pub reg: c_int }
#[repr(C)] pub struct reg_default { pub reg: c_uint, pub def: c_uint }
#[repr(C)] pub struct reg_sequence { pub reg: c_uint, pub def: c_uint }
#[repr(C)] pub struct sdw_stream_config { pub frame_rate: c_uint, pub ch_count: c_uint, pub bps: c_uint, pub direction: c_int, pub type_: c_int }
#[repr(C)] #[derive(Copy, Clone)] pub struct sdw_port_config { pub num: c_uint, pub ch_mask: c_uint }
#[repr(C)] #[derive(Copy, Clone)] pub struct sdw_dpn_prop { pub num: c_uint, pub type_: c_int, pub min_ch: c_uint, pub max_ch: c_uint, pub simple_ch_prep_sm: bool, pub read_only_wordlength: bool }
#[repr(C)] pub struct sdw_slave_prop { pub sink_ports: c_uint, pub simple_clk_stop_capable: bool, pub sink_dpn_prop: *mut sdw_dpn_prop, pub scp_int1_mask: c_uint }
#[repr(C)] pub struct sdw_slave { pub dev: device, pub dev_num: c_uint, pub m_port_map: [u32; 8], pub prop: sdw_slave_prop }
#[repr(C)] pub struct sdw_prepare_ch { pub num: c_uint }
#[repr(C)] pub struct snd_soc_component_driver { pub name: *const c_char, pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>, pub controls: *const snd_kcontrol_new, pub num_controls: c_uint, pub dapm_widgets: *const snd_soc_dapm_widget_def, pub num_dapm_widgets: c_uint, pub dapm_routes: *const snd_soc_dapm_route, pub num_dapm_routes: c_uint }
#[repr(C)] pub struct snd_soc_dai_ops { pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream,*mut snd_pcm_hw_params,*mut snd_soc_dai)->c_int>, pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream,*mut snd_soc_dai)->c_int>, pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai,c_int,c_int)->c_int>, pub set_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai,*mut c_void,c_int)->c_int>, pub mute_unmute_on_trigger: bool }
#[repr(C)] pub struct snd_soc_pcm_stream { pub stream_name: *const c_char, pub rates: c_uint, pub formats: c_uint, pub rate_min: c_uint, pub rate_max: c_uint, pub channels_min: c_uint, pub channels_max: c_uint }
#[repr(C)] pub struct snd_soc_dai_driver { pub name: *const c_char, pub playback: snd_soc_pcm_stream, pub ops: *const snd_soc_dai_ops }
#[repr(C)] pub struct dev_pm_ops { _private: [u8; 0] }
#[repr(C)] pub struct device_driver { pub name: *const c_char, pub pm: *const dev_pm_ops, pub suppress_bind_attrs: bool }
#[repr(C)] pub struct sdw_slave_ops { pub update_status: Option<unsafe extern "C" fn(*mut sdw_slave, sdw_slave_status)->c_int>, pub port_prep: Option<unsafe extern "C" fn(*mut sdw_slave,*mut sdw_prepare_ch,sdw_port_prep_ops)->c_int> }
#[repr(C)] pub struct sdw_driver { pub driver: device_driver, pub probe: Option<unsafe extern "C" fn(*mut sdw_slave,*const sdw_device_id)->c_int>, pub ops: *const sdw_slave_ops, pub id_table: *const sdw_device_id }
#[repr(C)] pub struct regmap_config { pub reg_bits: c_uint, pub val_bits: c_uint, pub cache_type: c_int, pub reg_defaults: *const reg_default, pub max_register: c_uint, pub num_reg_defaults: c_uint, pub volatile_reg: Option<unsafe extern "C" fn(*mut device,c_uint)->bool>, pub writeable_reg: Option<unsafe extern "C" fn(*mut device,c_uint)->bool>, pub reg_format_endian: c_int, pub val_format_endian: c_int, pub use_single_read: bool }
#[repr(C)] pub enum sdw_slave_status { SDW_SLAVE_UNATTACHED = 0, SDW_SLAVE_ATTACHED = 1 }
#[repr(C)] pub enum sdw_port_prep_ops { SDW_OPS_PORT_POST_PREP = 0 }
#[repr(C)] pub enum hwmon_sensor_types { hwmon_temp = 0 }

#[repr(C)] pub union snd_ctl_elem_value_value { pub integer: snd_ctl_elem_value_integer, pub enumerated: snd_ctl_elem_value_enumerated }
#[repr(C)] pub struct snd_ctl_elem_value_integer { pub value: [c_long; 4] }
#[repr(C)] pub struct snd_ctl_elem_value_enumerated { pub item: [c_uint; 4] }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }

extern "C" {
    fn regmap_read(map:*mut regmap, reg:c_uint, val:*mut c_uint)->c_int; fn regmap_update_bits(map:*mut regmap, reg:c_uint, mask:c_uint, val:c_uint)->c_int; fn regmap_multi_reg_write(map:*mut regmap, regs:*const reg_sequence, num:c_uint)->c_int;
    fn dev_get_drvdata(dev:*mut device)->*mut c_void; fn dev_set_drvdata(dev:*mut device, data:*mut c_void); fn dev_get_regmap(dev:*mut device, name:*const c_char)->*mut regmap;
    fn dev_dbg(dev:*mut device, fmt:*const c_char, ...); fn dev_warn(dev:*mut device, fmt:*const c_char, ...); fn dev_err_probe(dev:*mut device, err:c_long, fmt:*const c_char, ...)->c_int;
    fn snd_kcontrol_chip(kcontrol:*mut snd_kcontrol)->*mut snd_soc_component; fn snd_soc_component_get_drvdata(component:*mut snd_soc_component)->*mut c_void; fn snd_soc_component_init_regmap(comp:*mut snd_soc_component, map:*mut regmap); fn snd_soc_dapm_to_component(dapm:*mut snd_soc_dapm_context)->*mut snd_soc_component; fn snd_soc_component_write_field(component:*mut snd_soc_component, reg:c_uint, mask:c_uint, val:c_uint)->c_int;
    fn params_rate(params:*mut snd_pcm_hw_params)->c_uint; fn sdw_stream_add_slave(slave:*mut sdw_slave, sconfig:*mut sdw_stream_config, pconfig:*mut sdw_port_config, num:c_int, sruntime:*mut sdw_stream_runtime)->c_int; fn sdw_stream_remove_slave(slave:*mut sdw_slave, sruntime:*mut sdw_stream_runtime);
    fn pm_runtime_resume_and_get(dev:*mut device)->c_int; fn pm_runtime_put_autosuspend(dev:*mut device)->c_int; fn pm_runtime_set_autosuspend_delay(dev:*mut device, delay:c_int); fn pm_runtime_use_autosuspend(dev:*mut device); fn pm_runtime_mark_last_busy(dev:*mut device); fn pm_runtime_set_active(dev:*mut device); fn pm_runtime_enable(dev:*mut device);
    fn reset_control_assert(r:*mut reset_control)->c_int; fn reset_control_deassert(r:*mut reset_control)->c_int; fn gpiod_direction_output(g:*mut gpio_desc, value:c_int)->c_int; fn devm_reset_control_get_optional_shared(dev:*mut device, id:*const c_char)->*mut reset_control; fn devm_gpiod_get_optional(dev:*mut device, con_id:*const c_char, flags:c_int)->*mut gpio_desc;
    fn devm_kzalloc(dev:*mut device, size:usize, flags:c_uint)->*mut c_void; fn devm_regulator_get(dev:*mut device, id:*const c_char)->*mut regulator; fn regulator_enable(r:*mut regulator)->c_int; fn regulator_disable(r:*mut regulator)->c_int; fn of_property_read_u32_array(node:*mut c_void, prop:*const c_char, out:*mut u32, sz:usize)->c_int; fn devm_add_action_or_reset(dev:*mut device, action:unsafe extern "C" fn(*mut c_void), data:*mut c_void)->c_int; fn devm_regmap_init_sdw(slave:*mut sdw_slave, cfg:*const regmap_config)->*mut regmap; fn devm_hwmon_device_register_with_info(dev:*mut device, name:*const c_char, data:*mut c_void, info:*const hwmon_chip_info, groups:*const c_void)->*mut device; fn devm_snd_soc_register_component(dev:*mut device, drv:*const snd_soc_component_driver, dais:*mut snd_soc_dai_driver, num:c_uint)->c_int;
    fn mutex_init(m:*mut mutex); fn mutex_lock(m:*mut mutex); fn mutex_unlock(m:*mut mutex); fn regcache_cache_only(map:*mut regmap, enable:bool); fn regcache_mark_dirty(map:*mut regmap); fn regcache_sync(map:*mut regmap)->c_int;
}
fn IS_ERR<T>(p:*mut T)->bool { (p as isize) < 0 && (p as isize) > -4096 } fn PTR_ERR<T>(p:*mut T)->c_long { p as isize as c_long }
const GFP_KERNEL:c_uint=0; const ENOMEM:c_int=12; const EINVAL:c_int=22; const EAGAIN:c_int=11; const EOPNOTSUPP:c_int=95; const REGCACHE_MAPLE:c_int=0; const REGMAP_ENDIAN_NATIVE:c_int=0; const SDW_DPN_SIMPLE:c_int=0; const SDW_DATA_DIR_RX:c_int=0; const SDW_STREAM_PDM:c_int=0; const SDW_SCP_INT1_BUS_CLASH:c_uint=1; const SDW_SCP_INT1_PARITY:c_uint=2; const GPIOD_OUT_HIGH:c_int=1; const SND_SOC_DAPM_POST_PMU:c_int=1; const SND_SOC_DAPM_PRE_PMD:c_int=2; const hwmon_temp_input:u32=0;
const SNDRV_PCM_RATE_8000:u32=1<<0; const SNDRV_PCM_RATE_16000:u32=1<<1; const SNDRV_PCM_RATE_32000:u32=1<<2; const SNDRV_PCM_RATE_48000:u32=1<<3; const SNDRV_PCM_RATE_96000:u32=1<<4; const SNDRV_PCM_RATE_192000:u32=1<<5; const SNDRV_PCM_RATE_384000:u32=1<<6; const SNDRV_PCM_RATE_44100:u32=1<<7; const SNDRV_PCM_RATE_88200:u32=1<<8; const SNDRV_PCM_RATE_176400:u32=1<<9; const SNDRV_PCM_RATE_352800:u32=1<<10; const SNDRV_PCM_FMTBIT_S16_LE:u32=1<<0; const SNDRV_PCM_FMTBIT_S24_LE:u32=1<<1; const SNDRV_PCM_FMTBIT_S24_3LE:u32=1<<2; const SNDRV_PCM_FMTBIT_S32_LE:u32=1<<3;



pub const WSA883X_BASE: c_long = 0x3000;
pub const WSA883X_ANA_BG_TSADC_BASE: u32 = (WSA883X_BASE + 0x00000001) as u32;
pub const WSA883X_REF_CTRL: u32 = (WSA883X_ANA_BG_TSADC_BASE + 0x0000) as u32;
pub const WSA883X_TEST_CTL_0: u32 = (WSA883X_ANA_BG_TSADC_BASE + 0x0001) as u32;
pub const WSA883X_BIAS_0: u32 = (WSA883X_ANA_BG_TSADC_BASE + 0x0002) as u32;
pub const WSA883X_OP_CTL: u32 = (WSA883X_ANA_BG_TSADC_BASE + 0x0003) as u32;
pub const WSA883X_IREF_CTL: u32 = (WSA883X_ANA_BG_TSADC_BASE + 0x0004) as u32;
pub const WSA883X_ISENS_CTL: u32 = (WSA883X_ANA_BG_TSADC_BASE + 0x0005) as u32;
pub const WSA883X_CLK_CTL: u32 = (WSA883X_ANA_BG_TSADC_BASE + 0x0006) as u32;
pub const WSA883X_TEST_CTL_1: u32 = (WSA883X_ANA_BG_TSADC_BASE + 0x0007) as u32;
pub const WSA883X_BIAS_1: u32 = (WSA883X_ANA_BG_TSADC_BASE + 0x0008) as u32;
pub const WSA883X_ADC_CTL: u32 = (WSA883X_ANA_BG_TSADC_BASE + 0x0009) as u32;
pub const WSA883X_DOUT_MSB: u32 = (WSA883X_ANA_BG_TSADC_BASE + 0x000A) as u32;
pub const WSA883X_DOUT_LSB: u32 = (WSA883X_ANA_BG_TSADC_BASE + 0x000B) as u32;
pub const WSA883X_VBAT_SNS: u32 = (WSA883X_ANA_BG_TSADC_BASE + 0x000C) as u32;
pub const WSA883X_ITRIM_CODE: u32 = (WSA883X_ANA_BG_TSADC_BASE + 0x000D) as u32;

pub const WSA883X_ANA_IVSENSE_BASE: u32 = (WSA883X_BASE + 0x0000000F) as u32;
pub const WSA883X_EN: u32 = (WSA883X_ANA_IVSENSE_BASE + 0x0000) as u32;
pub const WSA883X_OVERRIDE1: u32 = (WSA883X_ANA_IVSENSE_BASE + 0x0001) as u32;
pub const WSA883X_OVERRIDE2: u32 = (WSA883X_ANA_IVSENSE_BASE + 0x0002) as u32;
pub const WSA883X_VSENSE1: u32 = (WSA883X_ANA_IVSENSE_BASE + 0x0003) as u32;
pub const WSA883X_ISENSE1: u32 = (WSA883X_ANA_IVSENSE_BASE + 0x0004) as u32;
pub const WSA883X_ISENSE2: u32 = (WSA883X_ANA_IVSENSE_BASE + 0x0005) as u32;
pub const WSA883X_ISENSE_CAL: u32 = (WSA883X_ANA_IVSENSE_BASE + 0x0006) as u32;
pub const WSA883X_MISC: u32 = (WSA883X_ANA_IVSENSE_BASE + 0x0007) as u32;
pub const WSA883X_ADC_0: u32 = (WSA883X_ANA_IVSENSE_BASE + 0x0008) as u32;
pub const WSA883X_ADC_1: u32 = (WSA883X_ANA_IVSENSE_BASE + 0x0009) as u32;
pub const WSA883X_ADC_2: u32 = (WSA883X_ANA_IVSENSE_BASE + 0x000A) as u32;
pub const WSA883X_ADC_3: u32 = (WSA883X_ANA_IVSENSE_BASE + 0x000B) as u32;
pub const WSA883X_ADC_4: u32 = (WSA883X_ANA_IVSENSE_BASE + 0x000C) as u32;
pub const WSA883X_ADC_5: u32 = (WSA883X_ANA_IVSENSE_BASE + 0x000D) as u32;
pub const WSA883X_ADC_6: u32 = (WSA883X_ANA_IVSENSE_BASE + 0x000E) as u32;
pub const WSA883X_ADC_7: u32 = (WSA883X_ANA_IVSENSE_BASE + 0x000F) as u32;
pub const WSA883X_STATUS: u32 = (WSA883X_ANA_IVSENSE_BASE + 0x0010) as u32;

pub const WSA883X_ANA_SPK_TOP_BASE: u32 = (WSA883X_BASE + 0x00000025) as u32;
pub const WSA883X_DAC_CTRL_REG: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x0000) as u32;
pub const WSA883X_DAC_EN_DEBUG_REG: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x0001) as u32;
pub const WSA883X_DAC_OPAMP_BIAS1_REG: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x0002) as u32;
pub const WSA883X_DAC_OPAMP_BIAS2_REG: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x0003) as u32;
pub const WSA883X_DAC_VCM_CTRL_REG: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x0004) as u32;
pub const WSA883X_DAC_VOLTAGE_CTRL_REG: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x0005) as u32;
pub const WSA883X_ATEST1_REG: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x0006) as u32;
pub const WSA883X_ATEST2_REG: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x0007) as u32;
pub const WSA883X_SPKR_TOP_BIAS_REG1: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x0008) as u32;
pub const WSA883X_SPKR_TOP_BIAS_REG2: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x0009) as u32;
pub const WSA883X_SPKR_TOP_BIAS_REG3: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x000A) as u32;
pub const WSA883X_SPKR_TOP_BIAS_REG4: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x000B) as u32;
pub const WSA883X_SPKR_CLIP_DET_REG: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x000C) as u32;
pub const WSA883X_SPKR_DRV_LF_BLK_EN: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x000D) as u32;
pub const WSA883X_SPKR_DRV_LF_EN: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x000E) as u32;
pub const WSA883X_SPKR_DRV_LF_MASK_DCC_CTL: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x000F) as u32;
pub const WSA883X_SPKR_DRV_LF_MISC_CTL: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x0010) as u32;
pub const WSA883X_SPKR_DRV_LF_REG_GAIN: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x0011) as u32;
pub const WSA883X_SPKR_DRV_OS_CAL_CTL: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x0012) as u32;
pub const WSA883X_SPKR_DRV_OS_CAL_CTL1: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x0013) as u32;
pub const WSA883X_SPKR_PWM_CLK_CTL: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x0014) as u32;
pub const WSA883X_SPKR_PWM_FREQ_SEL_MASK: u32 = BIT(3);
pub const WSA883X_SPKR_PWM_FREQ_F300KHZ: c_long = 0;
pub const WSA883X_SPKR_PWM_FREQ_F600KHZ: c_long = 1;
pub const WSA883X_SPKR_PDRV_HS_CTL: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x0015) as u32;
pub const WSA883X_SPKR_PDRV_LS_CTL: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x0016) as u32;
pub const WSA883X_SPKR_PWRSTG_DBG: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x0017) as u32;
pub const WSA883X_SPKR_OCP_CTL: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x0018) as u32;
pub const WSA883X_SPKR_BBM_CTL: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x0019) as u32;
pub const WSA883X_PA_STATUS0: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x001A) as u32;
pub const WSA883X_PA_STATUS1: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x001B) as u32;
pub const WSA883X_PA_STATUS2: u32 = (WSA883X_ANA_SPK_TOP_BASE + 0x001C) as u32;

pub const WSA883X_ANA_BOOST_BASE: u32 = (WSA883X_BASE + 0x00000043) as u32;
pub const WSA883X_EN_CTRL: u32 = (WSA883X_ANA_BOOST_BASE + 0x0000) as u32;
pub const WSA883X_CURRENT_LIMIT: u32 = (WSA883X_ANA_BOOST_BASE + 0x0001) as u32;
pub const WSA883X_IBIAS1: u32 = (WSA883X_ANA_BOOST_BASE + 0x0002) as u32;
pub const WSA883X_IBIAS2: u32 = (WSA883X_ANA_BOOST_BASE + 0x0003) as u32;
pub const WSA883X_IBIAS3: u32 = (WSA883X_ANA_BOOST_BASE + 0x0004) as u32;
pub const WSA883X_LDO_PROG: u32 = (WSA883X_ANA_BOOST_BASE + 0x0005) as u32;
pub const WSA883X_STABILITY_CTRL1: u32 = (WSA883X_ANA_BOOST_BASE + 0x0006) as u32;
pub const WSA883X_STABILITY_CTRL2: u32 = (WSA883X_ANA_BOOST_BASE + 0x0007) as u32;
pub const WSA883X_PWRSTAGE_CTRL1: u32 = (WSA883X_ANA_BOOST_BASE + 0x0008) as u32;
pub const WSA883X_PWRSTAGE_CTRL2: u32 = (WSA883X_ANA_BOOST_BASE + 0x0009) as u32;
pub const WSA883X_BYPASS_1: u32 = (WSA883X_ANA_BOOST_BASE + 0x000A) as u32;
pub const WSA883X_BYPASS_2: u32 = (WSA883X_ANA_BOOST_BASE + 0x000B) as u32;
pub const WSA883X_ZX_CTRL_1: u32 = (WSA883X_ANA_BOOST_BASE + 0x000C) as u32;
pub const WSA883X_ZX_CTRL_2: u32 = (WSA883X_ANA_BOOST_BASE + 0x000D) as u32;
pub const WSA883X_MISC1: u32 = (WSA883X_ANA_BOOST_BASE + 0x000E) as u32;
pub const WSA883X_MISC2: u32 = (WSA883X_ANA_BOOST_BASE + 0x000F) as u32;
pub const WSA883X_GMAMP_SUP1: u32 = (WSA883X_ANA_BOOST_BASE + 0x0010) as u32;
pub const WSA883X_PWRSTAGE_CTRL3: u32 = (WSA883X_ANA_BOOST_BASE + 0x0011) as u32;
pub const WSA883X_PWRSTAGE_CTRL4: u32 = (WSA883X_ANA_BOOST_BASE + 0x0012) as u32;
pub const WSA883X_TEST1: u32 = (WSA883X_ANA_BOOST_BASE + 0x0013) as u32;
pub const WSA883X_SPARE1: u32 = (WSA883X_ANA_BOOST_BASE + 0x0014) as u32;
pub const WSA883X_SPARE2: u32 = (WSA883X_ANA_BOOST_BASE + 0x0015) as u32;

pub const WSA883X_ANA_PON_LDOL_BASE: u32 = (WSA883X_BASE + 0x00000059) as u32;
pub const WSA883X_PON_CTL_0: u32 = (WSA883X_ANA_PON_LDOL_BASE + 0x0000) as u32;
pub const WSA883X_PON_CLT_1: u32 = (WSA883X_ANA_PON_LDOL_BASE + 0x0001) as u32;
pub const WSA883X_PON_CTL_2: u32 = (WSA883X_ANA_PON_LDOL_BASE + 0x0002) as u32;
pub const WSA883X_PON_CTL_3: u32 = (WSA883X_ANA_PON_LDOL_BASE + 0x0003) as u32;
pub const WSA883X_CKWD_CTL_0: u32 = (WSA883X_ANA_PON_LDOL_BASE + 0x0004) as u32;
pub const WSA883X_CKWD_CTL_1: u32 = (WSA883X_ANA_PON_LDOL_BASE + 0x0005) as u32;
pub const WSA883X_CKWD_CTL_2: u32 = (WSA883X_ANA_PON_LDOL_BASE + 0x0006) as u32;
pub const WSA883X_CKSK_CTL_0: u32 = (WSA883X_ANA_PON_LDOL_BASE + 0x0007) as u32;
pub const WSA883X_PADSW_CTL_0: u32 = (WSA883X_ANA_PON_LDOL_BASE + 0x0008) as u32;
pub const WSA883X_TEST_0: u32 = (WSA883X_ANA_PON_LDOL_BASE + 0x0009) as u32;
pub const WSA883X_TEST_1: u32 = (WSA883X_ANA_PON_LDOL_BASE + 0x000A) as u32;
pub const WSA883X_STATUS_0: u32 = (WSA883X_ANA_PON_LDOL_BASE + 0x000B) as u32;
pub const WSA883X_STATUS_1: u32 = (WSA883X_ANA_PON_LDOL_BASE + 0x000C) as u32;

pub const WSA883X_DIG_CTRL_BASE: u32 = (WSA883X_BASE + 0x00000400) as u32;
pub const WSA883X_CHIP_ID0: u32 = (WSA883X_DIG_CTRL_BASE + 0x0001) as u32;
pub const WSA883X_CHIP_ID1: u32 = (WSA883X_DIG_CTRL_BASE + 0x0002) as u32;
pub const WSA883X_CHIP_ID2: u32 = (WSA883X_DIG_CTRL_BASE + 0x0003) as u32;
pub const WSA883X_CHIP_ID3: u32 = (WSA883X_DIG_CTRL_BASE + 0x0004) as u32;
pub const WSA883X_BUS_ID: u32 = (WSA883X_DIG_CTRL_BASE + 0x0005) as u32;
pub const WSA883X_CDC_RST_CTL: u32 = (WSA883X_DIG_CTRL_BASE + 0x0006) as u32;
pub const WSA883X_TOP_CLK_CFG: u32 = (WSA883X_DIG_CTRL_BASE + 0x0007) as u32;
pub const WSA883X_CDC_PATH_MODE: u32 = (WSA883X_DIG_CTRL_BASE + 0x0008) as u32;
pub const WSA883X_RXD_MODE_MASK: u32 = BIT(1);
pub const WSA883X_RXD_MODE_NORMAL: c_long = 0;
pub const WSA883X_RXD_MODE_HIFI: c_long = 1;
pub const WSA883X_CDC_CLK_CTL: u32 = (WSA883X_DIG_CTRL_BASE + 0x0009) as u32;
pub const WSA883X_SWR_RESET_EN: u32 = (WSA883X_DIG_CTRL_BASE + 0x000A) as u32;
pub const WSA883X_RESET_CTL: u32 = (WSA883X_DIG_CTRL_BASE + 0x000B) as u32;
pub const WSA883X_PA_FSM_CTL: u32 = (WSA883X_DIG_CTRL_BASE + 0x0010) as u32;
pub const WSA883X_GLOBAL_PA_EN_MASK: u32 = BIT(0);
pub const WSA883X_GLOBAL_PA_ENABLE: c_long = 1;
pub const WSA883X_PA_FSM_TIMER0: u32 = (WSA883X_DIG_CTRL_BASE + 0x0011) as u32;
pub const WSA883X_PA_FSM_TIMER1: u32 = (WSA883X_DIG_CTRL_BASE + 0x0012) as u32;
pub const WSA883X_PA_FSM_STA: u32 = (WSA883X_DIG_CTRL_BASE + 0x0013) as u32;
pub const WSA883X_PA_FSM_ERR_COND: u32 = (WSA883X_DIG_CTRL_BASE + 0x0014) as u32;
pub const WSA883X_PA_FSM_MSK: u32 = (WSA883X_DIG_CTRL_BASE + 0x0015) as u32;
pub const WSA883X_PA_FSM_BYP: u32 = (WSA883X_DIG_CTRL_BASE + 0x0016) as u32;
pub const WSA883X_PA_FSM_BYP_DC_CAL_EN_MASK: c_long = 0x01;
pub const WSA883X_PA_FSM_BYP_DC_CAL_EN_SHIFT: c_long = 0;
pub const WSA883X_PA_FSM_BYP_CLK_WD_EN_MASK: c_long = 0x02;
pub const WSA883X_PA_FSM_BYP_CLK_WD_EN_SHIFT: c_long = 1;
pub const WSA883X_PA_FSM_BYP_BG_EN_MASK: c_long = 0x04;
pub const WSA883X_PA_FSM_BYP_BG_EN_SHIFT: c_long = 2;
pub const WSA883X_PA_FSM_BYP_BOOST_EN_MASK: c_long = 0x08;
pub const WSA883X_PA_FSM_BYP_BOOST_EN_SHIFT: c_long = 3;
pub const WSA883X_PA_FSM_BYP_PA_EN_MASK: c_long = 0x10;
pub const WSA883X_PA_FSM_BYP_PA_EN_SHIFT: c_long = 4;
pub const WSA883X_PA_FSM_BYP_D_UNMUTE_MASK: c_long = 0x20;
pub const WSA883X_PA_FSM_BYP_D_UNMUTE_SHIFT: c_long = 5;
pub const WSA883X_PA_FSM_BYP_SPKR_PROT_EN_MASK: c_long = 0x40;
pub const WSA883X_PA_FSM_BYP_SPKR_PROT_EN_SHIFT: c_long = 6;
pub const WSA883X_PA_FSM_BYP_TSADC_EN_MASK: c_long = 0x80;
pub const WSA883X_PA_FSM_BYP_TSADC_EN_SHIFT: c_long = 7;
pub const WSA883X_PA_FSM_DBG: u32 = (WSA883X_DIG_CTRL_BASE + 0x0017) as u32;
pub const WSA883X_TADC_VALUE_CTL: u32 = (WSA883X_DIG_CTRL_BASE + 0x0020) as u32;
pub const WSA883X_TADC_VALUE_CTL_TEMP_VALUE_RD_EN_MASK: c_long = 0x01;
pub const WSA883X_TADC_VALUE_CTL_TEMP_VALUE_RD_EN_SHIFT: c_long = 0;
pub const WSA883X_TADC_VALUE_CTL_VBAT_VALUE_RD_EN_MASK: c_long = 0x02;
pub const WSA883X_TADC_VALUE_CTL_VBAT_VALUE_RD_EN_SHIFT: c_long = 1;
pub const WSA883X_TEMP_DETECT_CTL: u32 = (WSA883X_DIG_CTRL_BASE + 0x0021) as u32;
pub const WSA883X_TEMP_MSB: u32 = (WSA883X_DIG_CTRL_BASE + 0x0022) as u32;
pub const WSA883X_TEMP_LSB: u32 = (WSA883X_DIG_CTRL_BASE + 0x0023) as u32;
pub const WSA883X_TEMP_CONFIG0: u32 = (WSA883X_DIG_CTRL_BASE + 0x0024) as u32;
pub const WSA883X_TEMP_CONFIG1: u32 = (WSA883X_DIG_CTRL_BASE + 0x0025) as u32;
pub const WSA883X_VBAT_ADC_FLT_CTL: u32 = (WSA883X_DIG_CTRL_BASE + 0x0026) as u32;
pub const WSA883X_VBAT_ADC_FLT_EN_MASK: u32 = BIT(0);
pub const WSA883X_VBAT_ADC_COEF_SEL_MASK: u32 = GENMASK(3, 1);
pub const WSA883X_VBAT_ADC_COEF_F_1DIV2: c_long = 0x0;
pub const WSA883X_VBAT_ADC_COEF_F_1DIV16: c_long = 0x3;
pub const WSA883X_VBAT_DIN_MSB: u32 = (WSA883X_DIG_CTRL_BASE + 0x0027) as u32;
pub const WSA883X_VBAT_DIN_LSB: u32 = (WSA883X_DIG_CTRL_BASE + 0x0028) as u32;
pub const WSA883X_VBAT_DOUT: u32 = (WSA883X_DIG_CTRL_BASE + 0x0029) as u32;
pub const WSA883X_SDM_PDM9_LSB: u32 = (WSA883X_DIG_CTRL_BASE + 0x002A) as u32;
pub const WSA883X_SDM_PDM9_MSB: u32 = (WSA883X_DIG_CTRL_BASE + 0x002B) as u32;
pub const WSA883X_CDC_RX_CTL: u32 = (WSA883X_DIG_CTRL_BASE + 0x0030) as u32;
pub const WSA883X_CDC_SPK_DSM_A1_0: u32 = (WSA883X_DIG_CTRL_BASE + 0x0031) as u32;
pub const WSA883X_CDC_SPK_DSM_A1_1: u32 = (WSA883X_DIG_CTRL_BASE + 0x0032) as u32;
pub const WSA883X_CDC_SPK_DSM_A2_0: u32 = (WSA883X_DIG_CTRL_BASE + 0x0033) as u32;
pub const WSA883X_CDC_SPK_DSM_A2_1: u32 = (WSA883X_DIG_CTRL_BASE + 0x0034) as u32;
pub const WSA883X_CDC_SPK_DSM_A3_0: u32 = (WSA883X_DIG_CTRL_BASE + 0x0035) as u32;
pub const WSA883X_CDC_SPK_DSM_A3_1: u32 = (WSA883X_DIG_CTRL_BASE + 0x0036) as u32;
pub const WSA883X_CDC_SPK_DSM_A4_0: u32 = (WSA883X_DIG_CTRL_BASE + 0x0037) as u32;
pub const WSA883X_CDC_SPK_DSM_A4_1: u32 = (WSA883X_DIG_CTRL_BASE + 0x0038) as u32;
pub const WSA883X_CDC_SPK_DSM_A5_0: u32 = (WSA883X_DIG_CTRL_BASE + 0x0039) as u32;
pub const WSA883X_CDC_SPK_DSM_A5_1: u32 = (WSA883X_DIG_CTRL_BASE + 0x003A) as u32;
pub const WSA883X_CDC_SPK_DSM_A6_0: u32 = (WSA883X_DIG_CTRL_BASE + 0x003B) as u32;
pub const WSA883X_CDC_SPK_DSM_A7_0: u32 = (WSA883X_DIG_CTRL_BASE + 0x003C) as u32;
pub const WSA883X_CDC_SPK_DSM_C_0: u32 = (WSA883X_DIG_CTRL_BASE + 0x003D) as u32;
pub const WSA883X_CDC_SPK_DSM_C_1: u32 = (WSA883X_DIG_CTRL_BASE + 0x003E) as u32;
pub const WSA883X_CDC_SPK_DSM_C_2: u32 = (WSA883X_DIG_CTRL_BASE + 0x003F) as u32;
pub const WSA883X_CDC_SPK_DSM_C_3: u32 = (WSA883X_DIG_CTRL_BASE + 0x0040) as u32;
pub const WSA883X_CDC_SPK_DSM_R1: u32 = (WSA883X_DIG_CTRL_BASE + 0x0041) as u32;
pub const WSA883X_CDC_SPK_DSM_R2: u32 = (WSA883X_DIG_CTRL_BASE + 0x0042) as u32;
pub const WSA883X_CDC_SPK_DSM_R3: u32 = (WSA883X_DIG_CTRL_BASE + 0x0043) as u32;
pub const WSA883X_CDC_SPK_DSM_R4: u32 = (WSA883X_DIG_CTRL_BASE + 0x0044) as u32;
pub const WSA883X_CDC_SPK_DSM_R5: u32 = (WSA883X_DIG_CTRL_BASE + 0x0045) as u32;
pub const WSA883X_CDC_SPK_DSM_R6: u32 = (WSA883X_DIG_CTRL_BASE + 0x0046) as u32;
pub const WSA883X_CDC_SPK_DSM_R7: u32 = (WSA883X_DIG_CTRL_BASE + 0x0047) as u32;
pub const WSA883X_CDC_SPK_GAIN_PDM_0: u32 = (WSA883X_DIG_CTRL_BASE + 0x0048) as u32;
pub const WSA883X_CDC_SPK_GAIN_PDM_1: u32 = (WSA883X_DIG_CTRL_BASE + 0x0049) as u32;
pub const WSA883X_CDC_SPK_GAIN_PDM_2: u32 = (WSA883X_DIG_CTRL_BASE + 0x004A) as u32;
pub const WSA883X_PDM_WD_CTL: u32 = (WSA883X_DIG_CTRL_BASE + 0x004B) as u32;
pub const WSA883X_PDM_EN_MASK: u32 = BIT(0);
pub const WSA883X_PDM_ENABLE: u32 = BIT(0);
pub const WSA883X_DEM_BYPASS_DATA0: u32 = (WSA883X_DIG_CTRL_BASE + 0x004C) as u32;
pub const WSA883X_DEM_BYPASS_DATA1: u32 = (WSA883X_DIG_CTRL_BASE + 0x004D) as u32;
pub const WSA883X_DEM_BYPASS_DATA2: u32 = (WSA883X_DIG_CTRL_BASE + 0x004E) as u32;
pub const WSA883X_DEM_BYPASS_DATA3: u32 = (WSA883X_DIG_CTRL_BASE + 0x004F) as u32;
pub const WSA883X_WAVG_CTL: u32 = (WSA883X_DIG_CTRL_BASE + 0x0050) as u32;
pub const WSA883X_WAVG_LRA_PER_0: u32 = (WSA883X_DIG_CTRL_BASE + 0x0051) as u32;
pub const WSA883X_WAVG_LRA_PER_1: u32 = (WSA883X_DIG_CTRL_BASE + 0x0052) as u32;
pub const WSA883X_WAVG_DELTA_THETA_0: u32 = (WSA883X_DIG_CTRL_BASE + 0x0053) as u32;
pub const WSA883X_WAVG_DELTA_THETA_1: u32 = (WSA883X_DIG_CTRL_BASE + 0x0054) as u32;
pub const WSA883X_WAVG_DIRECT_AMP_0: u32 = (WSA883X_DIG_CTRL_BASE + 0x0055) as u32;
pub const WSA883X_WAVG_DIRECT_AMP_1: u32 = (WSA883X_DIG_CTRL_BASE + 0x0056) as u32;
pub const WSA883X_WAVG_PTRN_AMP0_0: u32 = (WSA883X_DIG_CTRL_BASE + 0x0057) as u32;
pub const WSA883X_WAVG_PTRN_AMP0_1: u32 = (WSA883X_DIG_CTRL_BASE + 0x0058) as u32;
pub const WSA883X_WAVG_PTRN_AMP1_0: u32 = (WSA883X_DIG_CTRL_BASE + 0x0059) as u32;
pub const WSA883X_WAVG_PTRN_AMP1_1: u32 = (WSA883X_DIG_CTRL_BASE + 0x005A) as u32;
pub const WSA883X_WAVG_PTRN_AMP2_0: u32 = (WSA883X_DIG_CTRL_BASE + 0x005B) as u32;
pub const WSA883X_WAVG_PTRN_AMP2_1: u32 = (WSA883X_DIG_CTRL_BASE + 0x005C) as u32;
pub const WSA883X_WAVG_PTRN_AMP3_0: u32 = (WSA883X_DIG_CTRL_BASE + 0x005D) as u32;
pub const WSA883X_WAVG_PTRN_AMP3_1: u32 = (WSA883X_DIG_CTRL_BASE + 0x005E) as u32;
pub const WSA883X_WAVG_PTRN_AMP4_0: u32 = (WSA883X_DIG_CTRL_BASE + 0x005F) as u32;
pub const WSA883X_WAVG_PTRN_AMP4_1: u32 = (WSA883X_DIG_CTRL_BASE + 0x0060) as u32;
pub const WSA883X_WAVG_PTRN_AMP5_0: u32 = (WSA883X_DIG_CTRL_BASE + 0x0061) as u32;
pub const WSA883X_WAVG_PTRN_AMP5_1: u32 = (WSA883X_DIG_CTRL_BASE + 0x0062) as u32;
pub const WSA883X_WAVG_PTRN_AMP6_0: u32 = (WSA883X_DIG_CTRL_BASE + 0x0063) as u32;
pub const WSA883X_WAVG_PTRN_AMP6_1: u32 = (WSA883X_DIG_CTRL_BASE + 0x0064) as u32;
pub const WSA883X_WAVG_PTRN_AMP7_0: u32 = (WSA883X_DIG_CTRL_BASE + 0x0065) as u32;
pub const WSA883X_WAVG_PTRN_AMP7_1: u32 = (WSA883X_DIG_CTRL_BASE + 0x0066) as u32;
pub const WSA883X_WAVG_PER_0_1: u32 = (WSA883X_DIG_CTRL_BASE + 0x0067) as u32;
pub const WSA883X_WAVG_PER_2_3: u32 = (WSA883X_DIG_CTRL_BASE + 0x0068) as u32;
pub const WSA883X_WAVG_PER_4_5: u32 = (WSA883X_DIG_CTRL_BASE + 0x0069) as u32;
pub const WSA883X_WAVG_PER_6_7: u32 = (WSA883X_DIG_CTRL_BASE + 0x006A) as u32;
pub const WSA883X_WAVG_STA: u32 = (WSA883X_DIG_CTRL_BASE + 0x006B) as u32;
pub const WSA883X_DRE_CTL_0: u32 = (WSA883X_DIG_CTRL_BASE + 0x006C) as u32;
pub const WSA883X_DRE_OFFSET_MASK: u32 = GENMASK(2, 0);
pub const WSA883X_DRE_PROG_DELAY_MASK: u32 = GENMASK(7, 4);
pub const WSA883X_DRE_CTL_1: u32 = (WSA883X_DIG_CTRL_BASE + 0x006D) as u32;
pub const WSA883X_DRE_GAIN_EN_MASK: u32 = BIT(0);
pub const WSA883X_DRE_GAIN_FROM_CSR: c_long = 1;
pub const WSA883X_DRE_IDLE_DET_CTL: u32 = (WSA883X_DIG_CTRL_BASE + 0x006E) as u32;
pub const WSA883X_CLSH_CTL_0: u32 = (WSA883X_DIG_CTRL_BASE + 0x0070) as u32;
pub const WSA883X_CLSH_CTL_1: u32 = (WSA883X_DIG_CTRL_BASE + 0x0071) as u32;
pub const WSA883X_CLSH_V_HD_PA: u32 = (WSA883X_DIG_CTRL_BASE + 0x0072) as u32;
pub const WSA883X_CLSH_V_PA_MIN: u32 = (WSA883X_DIG_CTRL_BASE + 0x0073) as u32;
pub const WSA883X_CLSH_OVRD_VAL: u32 = (WSA883X_DIG_CTRL_BASE + 0x0074) as u32;
pub const WSA883X_CLSH_HARD_MAX: u32 = (WSA883X_DIG_CTRL_BASE + 0x0075) as u32;
pub const WSA883X_CLSH_SOFT_MAX: u32 = (WSA883X_DIG_CTRL_BASE + 0x0076) as u32;
pub const WSA883X_CLSH_SIG_DP: u32 = (WSA883X_DIG_CTRL_BASE + 0x0077) as u32;
pub const WSA883X_TAGC_CTL: u32 = (WSA883X_DIG_CTRL_BASE + 0x0078) as u32;
pub const WSA883X_TAGC_TIME: u32 = (WSA883X_DIG_CTRL_BASE + 0x0079) as u32;
pub const WSA883X_TAGC_E2E_GAIN: u32 = (WSA883X_DIG_CTRL_BASE + 0x007A) as u32;
pub const WSA883X_TAGC_FORCE_VAL: u32 = (WSA883X_DIG_CTRL_BASE + 0x007B) as u32;
pub const WSA883X_VAGC_CTL: u32 = (WSA883X_DIG_CTRL_BASE + 0x007C) as u32;
pub const WSA883X_VAGC_TIME: u32 = (WSA883X_DIG_CTRL_BASE + 0x007D) as u32;
pub const WSA883X_VAGC_ATTN_LVL_1_2: u32 = (WSA883X_DIG_CTRL_BASE + 0x007E) as u32;
pub const WSA883X_VAGC_ATTN_LVL_3: u32 = (WSA883X_DIG_CTRL_BASE + 0x007F) as u32;
pub const WSA883X_INTR_MODE: u32 = (WSA883X_DIG_CTRL_BASE + 0x0080) as u32;
pub const WSA883X_INTR_MASK0: u32 = (WSA883X_DIG_CTRL_BASE + 0x0081) as u32;
pub const WSA883X_INTR_MASK1: u32 = (WSA883X_DIG_CTRL_BASE + 0x0082) as u32;
pub const WSA883X_INTR_STATUS0: u32 = (WSA883X_DIG_CTRL_BASE + 0x0083) as u32;
pub const WSA883X_INTR_STATUS1: u32 = (WSA883X_DIG_CTRL_BASE + 0x0084) as u32;
pub const WSA883X_INTR_CLEAR0: u32 = (WSA883X_DIG_CTRL_BASE + 0x0085) as u32;
pub const WSA883X_INTR_CLEAR1: u32 = (WSA883X_DIG_CTRL_BASE + 0x0086) as u32;
pub const WSA883X_INTR_LEVEL0: u32 = (WSA883X_DIG_CTRL_BASE + 0x0087) as u32;
pub const WSA883X_INTR_LEVEL1: u32 = (WSA883X_DIG_CTRL_BASE + 0x0088) as u32;
pub const WSA883X_INTR_SET0: u32 = (WSA883X_DIG_CTRL_BASE + 0x0089) as u32;
pub const WSA883X_INTR_SET1: u32 = (WSA883X_DIG_CTRL_BASE + 0x008A) as u32;
pub const WSA883X_INTR_TEST0: u32 = (WSA883X_DIG_CTRL_BASE + 0x008B) as u32;
pub const WSA883X_INTR_TEST1: u32 = (WSA883X_DIG_CTRL_BASE + 0x008C) as u32;
pub const WSA883X_OTP_CTRL0: u32 = (WSA883X_DIG_CTRL_BASE + 0x0090) as u32;
pub const WSA883X_OTP_CTRL1: u32 = (WSA883X_DIG_CTRL_BASE + 0x0091) as u32;
pub const WSA883X_HDRIVE_CTL_GROUP1: u32 = (WSA883X_DIG_CTRL_BASE + 0x0092) as u32;
pub const WSA883X_PIN_CTL: u32 = (WSA883X_DIG_CTRL_BASE + 0x0093) as u32;
pub const WSA883X_PIN_CTL_OE: u32 = (WSA883X_DIG_CTRL_BASE + 0x0094) as u32;
pub const WSA883X_PIN_WDATA_IOPAD: u32 = (WSA883X_DIG_CTRL_BASE + 0x0095) as u32;
pub const WSA883X_PIN_STATUS: u32 = (WSA883X_DIG_CTRL_BASE + 0x0096) as u32;
pub const WSA883X_I2C_SLAVE_CTL: u32 = (WSA883X_DIG_CTRL_BASE + 0x0097) as u32;
pub const WSA883X_PDM_TEST_MODE: u32 = (WSA883X_DIG_CTRL_BASE + 0x00A0) as u32;
pub const WSA883X_ATE_TEST_MODE: u32 = (WSA883X_DIG_CTRL_BASE + 0x00A1) as u32;
pub const WSA883X_DIG_DEBUG_MODE: u32 = (WSA883X_DIG_CTRL_BASE + 0x00A3) as u32;
pub const WSA883X_DIG_DEBUG_SEL: u32 = (WSA883X_DIG_CTRL_BASE + 0x00A4) as u32;
pub const WSA883X_DIG_DEBUG_EN: u32 = (WSA883X_DIG_CTRL_BASE + 0x00A5) as u32;
pub const WSA883X_SWR_HM_TEST0: u32 = (WSA883X_DIG_CTRL_BASE + 0x00A6) as u32;
pub const WSA883X_SWR_HM_TEST1: u32 = (WSA883X_DIG_CTRL_BASE + 0x00A7) as u32;
pub const WSA883X_SWR_PAD_CTL: u32 = (WSA883X_DIG_CTRL_BASE + 0x00A8) as u32;
pub const WSA883X_TADC_DETECT_DBG_CTL: u32 = (WSA883X_DIG_CTRL_BASE + 0x00A9) as u32;
pub const WSA883X_TADC_DEBUG_MSB: u32 = (WSA883X_DIG_CTRL_BASE + 0x00AA) as u32;
pub const WSA883X_TADC_DEBUG_LSB: u32 = (WSA883X_DIG_CTRL_BASE + 0x00AB) as u32;
pub const WSA883X_SAMPLE_EDGE_SEL: u32 = (WSA883X_DIG_CTRL_BASE + 0x00AC) as u32;
pub const WSA883X_SWR_EDGE_SEL: u32 = (WSA883X_DIG_CTRL_BASE + 0x00AD) as u32;
pub const WSA883X_TEST_MODE_CTL: u32 = (WSA883X_DIG_CTRL_BASE + 0x00AE) as u32;
pub const WSA883X_IOPAD_CTL: u32 = (WSA883X_DIG_CTRL_BASE + 0x00AF) as u32;
pub const WSA883X_ANA_CSR_DBG_ADD: u32 = (WSA883X_DIG_CTRL_BASE + 0x00B0) as u32;
pub const WSA883X_ANA_CSR_DBG_CTL: u32 = (WSA883X_DIG_CTRL_BASE + 0x00B1) as u32;
pub const WSA883X_SPARE_R: u32 = (WSA883X_DIG_CTRL_BASE + 0x00BC) as u32;
pub const WSA883X_SPARE_0: u32 = (WSA883X_DIG_CTRL_BASE + 0x00BD) as u32;
pub const WSA883X_SPARE_1: u32 = (WSA883X_DIG_CTRL_BASE + 0x00BE) as u32;
pub const WSA883X_SPARE_2: u32 = (WSA883X_DIG_CTRL_BASE + 0x00BF) as u32;
pub const WSA883X_SCODE: u32 = (WSA883X_DIG_CTRL_BASE + 0x00C0) as u32;

pub const WSA883X_DIG_TRIM_BASE: u32 = (WSA883X_BASE + 0x00000500) as u32;
pub const WSA883X_OTP_REG_0: u32 = (WSA883X_DIG_TRIM_BASE + 0x0080) as u32;
pub const WSA883X_ID_MASK: u32 = GENMASK(3, 0);
pub const WSA883X_OTP_REG_1: u32 = (WSA883X_DIG_TRIM_BASE + 0x0081) as u32;
pub const WSA883X_OTP_REG_2: u32 = (WSA883X_DIG_TRIM_BASE + 0x0082) as u32;
pub const WSA883X_OTP_REG_3: u32 = (WSA883X_DIG_TRIM_BASE + 0x0083) as u32;
pub const WSA883X_OTP_REG_4: u32 = (WSA883X_DIG_TRIM_BASE + 0x0084) as u32;
pub const WSA883X_OTP_REG_5: u32 = (WSA883X_DIG_TRIM_BASE + 0x0085) as u32;
pub const WSA883X_OTP_REG_6: u32 = (WSA883X_DIG_TRIM_BASE + 0x0086) as u32;
pub const WSA883X_OTP_REG_7: u32 = (WSA883X_DIG_TRIM_BASE + 0x0087) as u32;
pub const WSA883X_OTP_REG_8: u32 = (WSA883X_DIG_TRIM_BASE + 0x0088) as u32;
pub const WSA883X_OTP_REG_9: u32 = (WSA883X_DIG_TRIM_BASE + 0x0089) as u32;
pub const WSA883X_OTP_REG_10: u32 = (WSA883X_DIG_TRIM_BASE + 0x008A) as u32;
pub const WSA883X_OTP_REG_11: u32 = (WSA883X_DIG_TRIM_BASE + 0x008B) as u32;
pub const WSA883X_OTP_REG_12: u32 = (WSA883X_DIG_TRIM_BASE + 0x008C) as u32;
pub const WSA883X_OTP_REG_13: u32 = (WSA883X_DIG_TRIM_BASE + 0x008D) as u32;
pub const WSA883X_OTP_REG_14: u32 = (WSA883X_DIG_TRIM_BASE + 0x008E) as u32;
pub const WSA883X_OTP_REG_15: u32 = (WSA883X_DIG_TRIM_BASE + 0x008F) as u32;
pub const WSA883X_OTP_REG_16: u32 = (WSA883X_DIG_TRIM_BASE + 0x0090) as u32;
pub const WSA883X_OTP_REG_17: u32 = (WSA883X_DIG_TRIM_BASE + 0x0091) as u32;
pub const WSA883X_OTP_REG_18: u32 = (WSA883X_DIG_TRIM_BASE + 0x0092) as u32;
pub const WSA883X_OTP_REG_19: u32 = (WSA883X_DIG_TRIM_BASE + 0x0093) as u32;
pub const WSA883X_OTP_REG_20: u32 = (WSA883X_DIG_TRIM_BASE + 0x0094) as u32;
pub const WSA883X_OTP_REG_21: u32 = (WSA883X_DIG_TRIM_BASE + 0x0095) as u32;
pub const WSA883X_OTP_REG_22: u32 = (WSA883X_DIG_TRIM_BASE + 0x0096) as u32;
pub const WSA883X_OTP_REG_23: u32 = (WSA883X_DIG_TRIM_BASE + 0x0097) as u32;
pub const WSA883X_OTP_REG_24: u32 = (WSA883X_DIG_TRIM_BASE + 0x0098) as u32;
pub const WSA883X_OTP_REG_25: u32 = (WSA883X_DIG_TRIM_BASE + 0x0099) as u32;
pub const WSA883X_OTP_REG_26: u32 = (WSA883X_DIG_TRIM_BASE + 0x009A) as u32;
pub const WSA883X_OTP_REG_27: u32 = (WSA883X_DIG_TRIM_BASE + 0x009B) as u32;
pub const WSA883X_OTP_REG_28: u32 = (WSA883X_DIG_TRIM_BASE + 0x009C) as u32;
pub const WSA883X_OTP_REG_29: u32 = (WSA883X_DIG_TRIM_BASE + 0x009D) as u32;
pub const WSA883X_OTP_REG_30: u32 = (WSA883X_DIG_TRIM_BASE + 0x009E) as u32;
pub const WSA883X_OTP_REG_31: u32 = (WSA883X_DIG_TRIM_BASE + 0x009F) as u32;
pub const WSA883X_OTP_REG_32: u32 = (WSA883X_DIG_TRIM_BASE + 0x00A0) as u32;
pub const WSA883X_OTP_REG_33: u32 = (WSA883X_DIG_TRIM_BASE + 0x00A1) as u32;
pub const WSA883X_OTP_REG_34: u32 = (WSA883X_DIG_TRIM_BASE + 0x00A2) as u32;
pub const WSA883X_OTP_REG_35: u32 = (WSA883X_DIG_TRIM_BASE + 0x00A3) as u32;
pub const WSA883X_OTP_REG_63: u32 = (WSA883X_DIG_TRIM_BASE + 0x00BF) as u32;

pub const WSA883X_DIG_EMEM_BASE: u32 = (WSA883X_BASE + 0x000005C0) as u32;
pub const WSA883X_EMEM_0: u32 = (WSA883X_DIG_EMEM_BASE + 0x0000) as u32;
pub const WSA883X_EMEM_1: u32 = (WSA883X_DIG_EMEM_BASE + 0x0001) as u32;
pub const WSA883X_EMEM_2: u32 = (WSA883X_DIG_EMEM_BASE + 0x0002) as u32;
pub const WSA883X_EMEM_3: u32 = (WSA883X_DIG_EMEM_BASE + 0x0003) as u32;
pub const WSA883X_EMEM_4: u32 = (WSA883X_DIG_EMEM_BASE + 0x0004) as u32;
pub const WSA883X_EMEM_5: u32 = (WSA883X_DIG_EMEM_BASE + 0x0005) as u32;
pub const WSA883X_EMEM_6: u32 = (WSA883X_DIG_EMEM_BASE + 0x0006) as u32;
pub const WSA883X_EMEM_7: u32 = (WSA883X_DIG_EMEM_BASE + 0x0007) as u32;
pub const WSA883X_EMEM_8: u32 = (WSA883X_DIG_EMEM_BASE + 0x0008) as u32;
pub const WSA883X_EMEM_9: u32 = (WSA883X_DIG_EMEM_BASE + 0x0009) as u32;
pub const WSA883X_EMEM_10: u32 = (WSA883X_DIG_EMEM_BASE + 0x000A) as u32;
pub const WSA883X_EMEM_11: u32 = (WSA883X_DIG_EMEM_BASE + 0x000B) as u32;
pub const WSA883X_EMEM_12: u32 = (WSA883X_DIG_EMEM_BASE + 0x000C) as u32;
pub const WSA883X_EMEM_13: u32 = (WSA883X_DIG_EMEM_BASE + 0x000D) as u32;
pub const WSA883X_EMEM_14: u32 = (WSA883X_DIG_EMEM_BASE + 0x000E) as u32;
pub const WSA883X_EMEM_15: u32 = (WSA883X_DIG_EMEM_BASE + 0x000F) as u32;
pub const WSA883X_EMEM_16: u32 = (WSA883X_DIG_EMEM_BASE + 0x0010) as u32;
pub const WSA883X_EMEM_17: u32 = (WSA883X_DIG_EMEM_BASE + 0x0011) as u32;
pub const WSA883X_EMEM_18: u32 = (WSA883X_DIG_EMEM_BASE + 0x0012) as u32;
pub const WSA883X_EMEM_19: u32 = (WSA883X_DIG_EMEM_BASE + 0x0013) as u32;
pub const WSA883X_EMEM_20: u32 = (WSA883X_DIG_EMEM_BASE + 0x0014) as u32;
pub const WSA883X_EMEM_21: u32 = (WSA883X_DIG_EMEM_BASE + 0x0015) as u32;
pub const WSA883X_EMEM_22: u32 = (WSA883X_DIG_EMEM_BASE + 0x0016) as u32;
pub const WSA883X_EMEM_23: u32 = (WSA883X_DIG_EMEM_BASE + 0x0017) as u32;
pub const WSA883X_EMEM_24: u32 = (WSA883X_DIG_EMEM_BASE + 0x0018) as u32;
pub const WSA883X_EMEM_25: u32 = (WSA883X_DIG_EMEM_BASE + 0x0019) as u32;
pub const WSA883X_EMEM_26: u32 = (WSA883X_DIG_EMEM_BASE + 0x001A) as u32;
pub const WSA883X_EMEM_27: u32 = (WSA883X_DIG_EMEM_BASE + 0x001B) as u32;
pub const WSA883X_EMEM_28: u32 = (WSA883X_DIG_EMEM_BASE + 0x001C) as u32;
pub const WSA883X_EMEM_29: u32 = (WSA883X_DIG_EMEM_BASE + 0x001D) as u32;
pub const WSA883X_EMEM_30: u32 = (WSA883X_DIG_EMEM_BASE + 0x001E) as u32;
pub const WSA883X_EMEM_31: u32 = (WSA883X_DIG_EMEM_BASE + 0x001F) as u32;
pub const WSA883X_EMEM_32: u32 = (WSA883X_DIG_EMEM_BASE + 0x0020) as u32;
pub const WSA883X_EMEM_33: u32 = (WSA883X_DIG_EMEM_BASE + 0x0021) as u32;
pub const WSA883X_EMEM_34: u32 = (WSA883X_DIG_EMEM_BASE + 0x0022) as u32;
pub const WSA883X_EMEM_35: u32 = (WSA883X_DIG_EMEM_BASE + 0x0023) as u32;
pub const WSA883X_EMEM_36: u32 = (WSA883X_DIG_EMEM_BASE + 0x0024) as u32;
pub const WSA883X_EMEM_37: u32 = (WSA883X_DIG_EMEM_BASE + 0x0025) as u32;
pub const WSA883X_EMEM_38: u32 = (WSA883X_DIG_EMEM_BASE + 0x0026) as u32;
pub const WSA883X_EMEM_39: u32 = (WSA883X_DIG_EMEM_BASE + 0x0027) as u32;
pub const WSA883X_EMEM_40: u32 = (WSA883X_DIG_EMEM_BASE + 0x0028) as u32;
pub const WSA883X_EMEM_41: u32 = (WSA883X_DIG_EMEM_BASE + 0x0029) as u32;
pub const WSA883X_EMEM_42: u32 = (WSA883X_DIG_EMEM_BASE + 0x002A) as u32;
pub const WSA883X_EMEM_43: u32 = (WSA883X_DIG_EMEM_BASE + 0x002B) as u32;
pub const WSA883X_EMEM_44: u32 = (WSA883X_DIG_EMEM_BASE + 0x002C) as u32;
pub const WSA883X_EMEM_45: u32 = (WSA883X_DIG_EMEM_BASE + 0x002D) as u32;
pub const WSA883X_EMEM_46: u32 = (WSA883X_DIG_EMEM_BASE + 0x002E) as u32;
pub const WSA883X_EMEM_47: u32 = (WSA883X_DIG_EMEM_BASE + 0x002F) as u32;
pub const WSA883X_EMEM_48: u32 = (WSA883X_DIG_EMEM_BASE + 0x0030) as u32;
pub const WSA883X_EMEM_49: u32 = (WSA883X_DIG_EMEM_BASE + 0x0031) as u32;
pub const WSA883X_EMEM_50: u32 = (WSA883X_DIG_EMEM_BASE + 0x0032) as u32;
pub const WSA883X_EMEM_51: u32 = (WSA883X_DIG_EMEM_BASE + 0x0033) as u32;
pub const WSA883X_EMEM_52: u32 = (WSA883X_DIG_EMEM_BASE + 0x0034) as u32;
pub const WSA883X_EMEM_53: u32 = (WSA883X_DIG_EMEM_BASE + 0x0035) as u32;
pub const WSA883X_EMEM_54: u32 = (WSA883X_DIG_EMEM_BASE + 0x0036) as u32;
pub const WSA883X_EMEM_55: u32 = (WSA883X_DIG_EMEM_BASE + 0x0037) as u32;
pub const WSA883X_EMEM_56: u32 = (WSA883X_DIG_EMEM_BASE + 0x0038) as u32;
pub const WSA883X_EMEM_57: u32 = (WSA883X_DIG_EMEM_BASE + 0x0039) as u32;
pub const WSA883X_EMEM_58: u32 = (WSA883X_DIG_EMEM_BASE + 0x003A) as u32;
pub const WSA883X_EMEM_59: u32 = (WSA883X_DIG_EMEM_BASE + 0x003B) as u32;
pub const WSA883X_EMEM_60: u32 = (WSA883X_DIG_EMEM_BASE + 0x003C) as u32;
pub const WSA883X_EMEM_61: u32 = (WSA883X_DIG_EMEM_BASE + 0x003D) as u32;
pub const WSA883X_EMEM_62: u32 = (WSA883X_DIG_EMEM_BASE + 0x003E) as u32;
pub const WSA883X_EMEM_63: u32 = (WSA883X_DIG_EMEM_BASE + 0x003F) as u32;

pub const WSA883X_NUM_REGISTERS: u32 = (WSA883X_EMEM_63 + 1) as u32;
pub const WSA883X_MAX_REGISTER: u32 = (WSA883X_NUM_REGISTERS - 1) as u32;

pub const WSA883X_VERSION_1_0: c_long = 0;
pub const WSA883X_VERSION_1_1: c_long = 1;

pub const WSA883X_MAX_SWR_PORTS: c_long = 4;
// translated macro: #define WSA883X_RATES (SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 |\
			SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_48000 |\
			SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000 |\
			SNDRV_PCM_RATE_384000)
/* Fractional Rates */
// translated macro: #define WSA883X_FRAC_RATES (SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_88200 |\
				SNDRV_PCM_RATE_176400 | SNDRV_PCM_RATE_352800)

// translated macro: #define WSA883X_FORMATS (SNDRV_PCM_FMTBIT_S16_LE |\
		SNDRV_PCM_FMTBIT_S24_LE |\
		SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_S32_LE)

/* Two-point trimming for temperature calibration */
pub const WSA883X_T1_TEMP: c_long = -10;
pub const WSA883X_T2_TEMP: c_long = 150;

/*
 * Device will report senseless data in many cases, so discard any measurements
 * outside of valid range.
 */
pub const WSA883X_LOW_TEMP_THRESHOLD: c_long = 5;
pub const WSA883X_HIGH_TEMP_THRESHOLD: c_long = 45;

#[repr(C)]
pub struct wsa883x_priv {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub vdd: *mut regulator,
    pub slave: *mut sdw_slave,
    pub sconfig: sdw_stream_config,
    pub sruntime: *mut sdw_stream_runtime,
    pub port_config: [sdw_port_config; WSA883X_MAX_SWR_PORTS as usize],
    pub sd_n: *mut gpio_desc,
    pub sd_reset: *mut reset_control,
    pub port_prepared: [bool; WSA883X_MAX_SWR_PORTS as usize],
    pub port_enable: [bool; WSA883X_MAX_SWR_PORTS as usize],
    pub active_ports: c_int,
    pub dev_mode: c_int,
    pub comp_offset: c_int,
    pub hw_init: bool,
	/*
	 * Protects temperature reading code (related to speaker protection) and
	 * fields: temperature and pa_on.
	 */
    pub sp_lock: mutex,
    pub temperature: c_uint,
    pub pa_on: bool,
}

// anonymous C enum
	WSA8830 = 0,
	WSA8835,
	WSA8832,
	WSA8835_V2 = 5,
}

// anonymous C enum
	COMP_OFFSET0,
	COMP_OFFSET1,
	COMP_OFFSET2,
	COMP_OFFSET3,
	COMP_OFFSET4,
}

#[repr(C)]
pub enum wsa_port_ids {
	WSA883X_PORT_DAC,
	WSA883X_PORT_COMP,
	WSA883X_PORT_BOOST,
	WSA883X_PORT_VISENSE,
}

static const char * const wsa_dev_mode_text[] = {
	"Speaker", "Receiver", "Ultrasound"
}

// anonymous C enum
	SPEAKER,
	RECEIVER,
	ULTRASOUND,
}

static const struct soc_enum wsa_dev_mode_enum =
	SOC_ENUM_SINGLE_EXT(ARRAY_SIZE(wsa_dev_mode_text), wsa_dev_mode_text);

/* 4 ports */
static mut wsa_sink_dpn_prop: [sdw_dpn_prop; WSA883X_MAX_SWR_PORTS as usize] = [
	/* WSA883X_PORT_DAC */ DAC {
		num: WSA883X_PORT_DAC + 1,
		type_: SDW_DPN_SIMPLE,
		min_ch: 1,
		max_ch: 1,
		simple_ch_prep_sm: true,
		read_only_wordlength: true,
	},
	/* WSA883X_PORT_COMP */ COMP {
		num: WSA883X_PORT_COMP + 1,
		type_: SDW_DPN_SIMPLE,
		min_ch: 1,
		max_ch: 1,
		simple_ch_prep_sm: true,
		read_only_wordlength: true,
	},
	/* WSA883X_PORT_BOOST */ BOOST {
		num: WSA883X_PORT_BOOST + 1,
		type_: SDW_DPN_SIMPLE,
		min_ch: 1,
		max_ch: 1,
		simple_ch_prep_sm: true,
		read_only_wordlength: true,
	},
	/* WSA883X_PORT_VISENSE */ VISENSE {
		num: WSA883X_PORT_VISENSE + 1,
		type_: SDW_DPN_SIMPLE,
		min_ch: 1,
		max_ch: 1,
		simple_ch_prep_sm: true,
		read_only_wordlength: true,
	}
}

static mut wsa883x_pconfig: [sdw_port_config; WSA883X_MAX_SWR_PORTS as usize] = [
	/* WSA883X_PORT_DAC */ DAC {
		num: WSA883X_PORT_DAC + 1,
		ch_mask: 0x1,
	},
	/* WSA883X_PORT_COMP */ COMP {
		num: WSA883X_PORT_COMP + 1,
		ch_mask: 0xf,
	},
	/* WSA883X_PORT_BOOST */ BOOST {
		num: WSA883X_PORT_BOOST + 1,
		ch_mask: 0x3,
	},
	/* WSA883X_PORT_VISENSE */ VISENSE {
		num: WSA883X_PORT_VISENSE + 1,
		ch_mask: 0x1,
	},
}

static wsa883x_defaults: &[reg_default] = &[
	{ WSA883X_REF_CTRL, 0xD5 },
	{ WSA883X_TEST_CTL_0, 0x06 },
	{ WSA883X_BIAS_0, 0xD2 },
	{ WSA883X_OP_CTL, 0xE0 },
	{ WSA883X_IREF_CTL, 0x57 },
	{ WSA883X_ISENS_CTL, 0x47 },
	{ WSA883X_CLK_CTL, 0x87 },
	{ WSA883X_TEST_CTL_1, 0x00 },
	{ WSA883X_BIAS_1, 0x51 },
	{ WSA883X_ADC_CTL, 0x01 },
	{ WSA883X_DOUT_MSB, 0x00 },
	{ WSA883X_DOUT_LSB, 0x00 },
	{ WSA883X_VBAT_SNS, 0x40 },
	{ WSA883X_ITRIM_CODE, 0x9F },
	{ WSA883X_EN, 0x20 },
	{ WSA883X_OVERRIDE1, 0x00 },
	{ WSA883X_OVERRIDE2, 0x08 },
	{ WSA883X_VSENSE1, 0xD3 },
	{ WSA883X_ISENSE1, 0xD4 },
	{ WSA883X_ISENSE2, 0x20 },
	{ WSA883X_ISENSE_CAL, 0x00 },
	{ WSA883X_MISC, 0x08 },
	{ WSA883X_ADC_0, 0x00 },
	{ WSA883X_ADC_1, 0x00 },
	{ WSA883X_ADC_2, 0x40 },
	{ WSA883X_ADC_3, 0x80 },
	{ WSA883X_ADC_4, 0x25 },
	{ WSA883X_ADC_5, 0x25 },
	{ WSA883X_ADC_6, 0x08 },
	{ WSA883X_ADC_7, 0x81 },
	{ WSA883X_STATUS, 0x00 },
	{ WSA883X_DAC_CTRL_REG, 0x53 },
	{ WSA883X_DAC_EN_DEBUG_REG, 0x00 },
	{ WSA883X_DAC_OPAMP_BIAS1_REG, 0x48 },
	{ WSA883X_DAC_OPAMP_BIAS2_REG, 0x48 },
	{ WSA883X_DAC_VCM_CTRL_REG, 0x88 },
	{ WSA883X_DAC_VOLTAGE_CTRL_REG, 0xA5 },
	{ WSA883X_ATEST1_REG, 0x00 },
	{ WSA883X_ATEST2_REG, 0x00 },
	{ WSA883X_SPKR_TOP_BIAS_REG1, 0x6A },
	{ WSA883X_SPKR_TOP_BIAS_REG2, 0x65 },
	{ WSA883X_SPKR_TOP_BIAS_REG3, 0x55 },
	{ WSA883X_SPKR_TOP_BIAS_REG4, 0xA9 },
	{ WSA883X_SPKR_CLIP_DET_REG, 0x9C },
	{ WSA883X_SPKR_DRV_LF_BLK_EN, 0x0F },
	{ WSA883X_SPKR_DRV_LF_EN, 0x0A },
	{ WSA883X_SPKR_DRV_LF_MASK_DCC_CTL, 0x00 },
	{ WSA883X_SPKR_DRV_LF_MISC_CTL, 0x3A },
	{ WSA883X_SPKR_DRV_LF_REG_GAIN, 0x00 },
	{ WSA883X_SPKR_DRV_OS_CAL_CTL, 0x00 },
	{ WSA883X_SPKR_DRV_OS_CAL_CTL1, 0x90 },
	{ WSA883X_SPKR_PWM_CLK_CTL, 0x00 },
	{ WSA883X_SPKR_PDRV_HS_CTL, 0x52 },
	{ WSA883X_SPKR_PDRV_LS_CTL, 0x48 },
	{ WSA883X_SPKR_PWRSTG_DBG, 0x08 },
	{ WSA883X_SPKR_OCP_CTL, 0xE2 },
	{ WSA883X_SPKR_BBM_CTL, 0x92 },
	{ WSA883X_PA_STATUS0, 0x00 },
	{ WSA883X_PA_STATUS1, 0x00 },
	{ WSA883X_PA_STATUS2, 0x80 },
	{ WSA883X_EN_CTRL, 0x44 },
	{ WSA883X_CURRENT_LIMIT, 0xCC },
	{ WSA883X_IBIAS1, 0x00 },
	{ WSA883X_IBIAS2, 0x00 },
	{ WSA883X_IBIAS3, 0x00 },
	{ WSA883X_LDO_PROG, 0x02 },
	{ WSA883X_STABILITY_CTRL1, 0x8E },
	{ WSA883X_STABILITY_CTRL2, 0x10 },
	{ WSA883X_PWRSTAGE_CTRL1, 0x06 },
	{ WSA883X_PWRSTAGE_CTRL2, 0x00 },
	{ WSA883X_BYPASS_1, 0x19 },
	{ WSA883X_BYPASS_2, 0x13 },
	{ WSA883X_ZX_CTRL_1, 0xF0 },
	{ WSA883X_ZX_CTRL_2, 0x04 },
	{ WSA883X_MISC1, 0x06 },
	{ WSA883X_MISC2, 0xA0 },
	{ WSA883X_GMAMP_SUP1, 0x82 },
	{ WSA883X_PWRSTAGE_CTRL3, 0x39 },
	{ WSA883X_PWRSTAGE_CTRL4, 0x5F },
	{ WSA883X_TEST1, 0x00 },
	{ WSA883X_SPARE1, 0x00 },
	{ WSA883X_SPARE2, 0x00 },
	{ WSA883X_PON_CTL_0, 0x10 },
	{ WSA883X_PON_CLT_1, 0xE0 },
	{ WSA883X_PON_CTL_2, 0x90 },
	{ WSA883X_PON_CTL_3, 0x70 },
	{ WSA883X_CKWD_CTL_0, 0x34 },
	{ WSA883X_CKWD_CTL_1, 0x0F },
	{ WSA883X_CKWD_CTL_2, 0x00 },
	{ WSA883X_CKSK_CTL_0, 0x00 },
	{ WSA883X_PADSW_CTL_0, 0x00 },
	{ WSA883X_TEST_0, 0x00 },
	{ WSA883X_TEST_1, 0x00 },
	{ WSA883X_STATUS_0, 0x00 },
	{ WSA883X_STATUS_1, 0x00 },
	{ WSA883X_CHIP_ID0, 0x00 },
	{ WSA883X_CHIP_ID1, 0x00 },
	{ WSA883X_CHIP_ID2, 0x02 },
	{ WSA883X_CHIP_ID3, 0x02 },
	{ WSA883X_BUS_ID, 0x00 },
	{ WSA883X_CDC_RST_CTL, 0x01 },
	{ WSA883X_TOP_CLK_CFG, 0x00 },
	{ WSA883X_CDC_PATH_MODE, 0x00 },
	{ WSA883X_CDC_CLK_CTL, 0xFF },
	{ WSA883X_SWR_RESET_EN, 0x00 },
	{ WSA883X_RESET_CTL, 0x00 },
	{ WSA883X_PA_FSM_CTL, 0x00 },
	{ WSA883X_PA_FSM_TIMER0, 0x80 },
	{ WSA883X_PA_FSM_TIMER1, 0x80 },
	{ WSA883X_PA_FSM_STA, 0x00 },
	{ WSA883X_PA_FSM_ERR_COND, 0x00 },
	{ WSA883X_PA_FSM_MSK, 0x00 },
	{ WSA883X_PA_FSM_BYP, 0x01 },
	{ WSA883X_PA_FSM_DBG, 0x00 },
	{ WSA883X_TADC_VALUE_CTL, 0x03 },
	{ WSA883X_TEMP_DETECT_CTL, 0x01 },
	{ WSA883X_TEMP_MSB, 0x00 },
	{ WSA883X_TEMP_LSB, 0x00 },
	{ WSA883X_TEMP_CONFIG0, 0x00 },
	{ WSA883X_TEMP_CONFIG1, 0x00 },
	{ WSA883X_VBAT_ADC_FLT_CTL, 0x00 },
	{ WSA883X_VBAT_DIN_MSB, 0x00 },
	{ WSA883X_VBAT_DIN_LSB, 0x00 },
	{ WSA883X_VBAT_DOUT, 0x00 },
	{ WSA883X_SDM_PDM9_LSB, 0x00 },
	{ WSA883X_SDM_PDM9_MSB, 0x00 },
	{ WSA883X_CDC_RX_CTL, 0xFE },
	{ WSA883X_CDC_SPK_DSM_A1_0, 0x00 },
	{ WSA883X_CDC_SPK_DSM_A1_1, 0x01 },
	{ WSA883X_CDC_SPK_DSM_A2_0, 0x96 },
	{ WSA883X_CDC_SPK_DSM_A2_1, 0x09 },
	{ WSA883X_CDC_SPK_DSM_A3_0, 0xAB },
	{ WSA883X_CDC_SPK_DSM_A3_1, 0x05 },
	{ WSA883X_CDC_SPK_DSM_A4_0, 0x1C },
	{ WSA883X_CDC_SPK_DSM_A4_1, 0x02 },
	{ WSA883X_CDC_SPK_DSM_A5_0, 0x17 },
	{ WSA883X_CDC_SPK_DSM_A5_1, 0x02 },
	{ WSA883X_CDC_SPK_DSM_A6_0, 0xAA },
	{ WSA883X_CDC_SPK_DSM_A7_0, 0xE3 },
	{ WSA883X_CDC_SPK_DSM_C_0, 0x69 },
	{ WSA883X_CDC_SPK_DSM_C_1, 0x54 },
	{ WSA883X_CDC_SPK_DSM_C_2, 0x02 },
	{ WSA883X_CDC_SPK_DSM_C_3, 0x15 },
	{ WSA883X_CDC_SPK_DSM_R1, 0xA4 },
	{ WSA883X_CDC_SPK_DSM_R2, 0xB5 },
	{ WSA883X_CDC_SPK_DSM_R3, 0x86 },
	{ WSA883X_CDC_SPK_DSM_R4, 0x85 },
	{ WSA883X_CDC_SPK_DSM_R5, 0xAA },
	{ WSA883X_CDC_SPK_DSM_R6, 0xE2 },
	{ WSA883X_CDC_SPK_DSM_R7, 0x62 },
	{ WSA883X_CDC_SPK_GAIN_PDM_0, 0x00 },
	{ WSA883X_CDC_SPK_GAIN_PDM_1, 0xFC },
	{ WSA883X_CDC_SPK_GAIN_PDM_2, 0x05 },
	{ WSA883X_PDM_WD_CTL, 0x00 },
	{ WSA883X_DEM_BYPASS_DATA0, 0x00 },
	{ WSA883X_DEM_BYPASS_DATA1, 0x00 },
	{ WSA883X_DEM_BYPASS_DATA2, 0x00 },
	{ WSA883X_DEM_BYPASS_DATA3, 0x00 },
	{ WSA883X_WAVG_CTL, 0x06 },
	{ WSA883X_WAVG_LRA_PER_0, 0xD1 },
	{ WSA883X_WAVG_LRA_PER_1, 0x00 },
	{ WSA883X_WAVG_DELTA_THETA_0, 0xE6 },
	{ WSA883X_WAVG_DELTA_THETA_1, 0x04 },
	{ WSA883X_WAVG_DIRECT_AMP_0, 0x50 },
	{ WSA883X_WAVG_DIRECT_AMP_1, 0x00 },
	{ WSA883X_WAVG_PTRN_AMP0_0, 0x50 },
	{ WSA883X_WAVG_PTRN_AMP0_1, 0x00 },
	{ WSA883X_WAVG_PTRN_AMP1_0, 0x50 },
	{ WSA883X_WAVG_PTRN_AMP1_1, 0x00 },
	{ WSA883X_WAVG_PTRN_AMP2_0, 0x50 },
	{ WSA883X_WAVG_PTRN_AMP2_1, 0x00 },
	{ WSA883X_WAVG_PTRN_AMP3_0, 0x50 },
	{ WSA883X_WAVG_PTRN_AMP3_1, 0x00 },
	{ WSA883X_WAVG_PTRN_AMP4_0, 0x50 },
	{ WSA883X_WAVG_PTRN_AMP4_1, 0x00 },
	{ WSA883X_WAVG_PTRN_AMP5_0, 0x50 },
	{ WSA883X_WAVG_PTRN_AMP5_1, 0x00 },
	{ WSA883X_WAVG_PTRN_AMP6_0, 0x50 },
	{ WSA883X_WAVG_PTRN_AMP6_1, 0x00 },
	{ WSA883X_WAVG_PTRN_AMP7_0, 0x50 },
	{ WSA883X_WAVG_PTRN_AMP7_1, 0x00 },
	{ WSA883X_WAVG_PER_0_1, 0x88 },
	{ WSA883X_WAVG_PER_2_3, 0x88 },
	{ WSA883X_WAVG_PER_4_5, 0x88 },
	{ WSA883X_WAVG_PER_6_7, 0x88 },
	{ WSA883X_WAVG_STA, 0x00 },
	{ WSA883X_DRE_CTL_0, 0x70 },
	{ WSA883X_DRE_CTL_1, 0x08 },
	{ WSA883X_DRE_IDLE_DET_CTL, 0x1F },
	{ WSA883X_CLSH_CTL_0, 0x37 },
	{ WSA883X_CLSH_CTL_1, 0x81 },
	{ WSA883X_CLSH_V_HD_PA, 0x0F },
	{ WSA883X_CLSH_V_PA_MIN, 0x00 },
	{ WSA883X_CLSH_OVRD_VAL, 0x00 },
	{ WSA883X_CLSH_HARD_MAX, 0xFF },
	{ WSA883X_CLSH_SOFT_MAX, 0xF5 },
	{ WSA883X_CLSH_SIG_DP, 0x00 },
	{ WSA883X_TAGC_CTL, 0x10 },
	{ WSA883X_TAGC_TIME, 0x20 },
	{ WSA883X_TAGC_E2E_GAIN, 0x02 },
	{ WSA883X_TAGC_FORCE_VAL, 0x00 },
	{ WSA883X_VAGC_CTL, 0x00 },
	{ WSA883X_VAGC_TIME, 0x08 },
	{ WSA883X_VAGC_ATTN_LVL_1_2, 0x21 },
	{ WSA883X_VAGC_ATTN_LVL_3, 0x03 },
	{ WSA883X_INTR_MODE, 0x00 },
	{ WSA883X_INTR_MASK0, 0x90 },
	{ WSA883X_INTR_MASK1, 0x00 },
	{ WSA883X_INTR_STATUS0, 0x00 },
	{ WSA883X_INTR_STATUS1, 0x00 },
	{ WSA883X_INTR_CLEAR0, 0x00 },
	{ WSA883X_INTR_CLEAR1, 0x00 },
	{ WSA883X_INTR_LEVEL0, 0x00 },
	{ WSA883X_INTR_LEVEL1, 0x00 },
	{ WSA883X_INTR_SET0, 0x00 },
	{ WSA883X_INTR_SET1, 0x00 },
	{ WSA883X_INTR_TEST0, 0x00 },
	{ WSA883X_INTR_TEST1, 0x00 },
	{ WSA883X_OTP_CTRL0, 0x00 },
	{ WSA883X_OTP_CTRL1, 0x00 },
	{ WSA883X_HDRIVE_CTL_GROUP1, 0x00 },
	{ WSA883X_PIN_CTL, 0x04 },
	{ WSA883X_PIN_CTL_OE, 0x00 },
	{ WSA883X_PIN_WDATA_IOPAD, 0x00 },
	{ WSA883X_PIN_STATUS, 0x00 },
	{ WSA883X_I2C_SLAVE_CTL, 0x00 },
	{ WSA883X_PDM_TEST_MODE, 0x00 },
	{ WSA883X_ATE_TEST_MODE, 0x00 },
	{ WSA883X_DIG_DEBUG_MODE, 0x00 },
	{ WSA883X_DIG_DEBUG_SEL, 0x00 },
	{ WSA883X_DIG_DEBUG_EN, 0x00 },
	{ WSA883X_SWR_HM_TEST0, 0x08 },
	{ WSA883X_SWR_HM_TEST1, 0x00 },
	{ WSA883X_SWR_PAD_CTL, 0x37 },
	{ WSA883X_TADC_DETECT_DBG_CTL, 0x00 },
	{ WSA883X_TADC_DEBUG_MSB, 0x00 },
	{ WSA883X_TADC_DEBUG_LSB, 0x00 },
	{ WSA883X_SAMPLE_EDGE_SEL, 0x7F },
	{ WSA883X_SWR_EDGE_SEL, 0x00 },
	{ WSA883X_TEST_MODE_CTL, 0x04 },
	{ WSA883X_IOPAD_CTL, 0x00 },
	{ WSA883X_ANA_CSR_DBG_ADD, 0x00 },
	{ WSA883X_ANA_CSR_DBG_CTL, 0x12 },
	{ WSA883X_SPARE_R, 0x00 },
	{ WSA883X_SPARE_0, 0x00 },
	{ WSA883X_SPARE_1, 0x00 },
	{ WSA883X_SPARE_2, 0x00 },
	{ WSA883X_SCODE, 0x00 },
	{ WSA883X_OTP_REG_0, 0x05 },
	{ WSA883X_OTP_REG_1, 0xFF },
	{ WSA883X_OTP_REG_2, 0xC0 },
	{ WSA883X_OTP_REG_3, 0xFF },
	{ WSA883X_OTP_REG_4, 0xC0 },
	{ WSA883X_OTP_REG_5, 0xFF },
	{ WSA883X_OTP_REG_6, 0xFF },
	{ WSA883X_OTP_REG_7, 0xFF },
	{ WSA883X_OTP_REG_8, 0xFF },
	{ WSA883X_OTP_REG_9, 0xFF },
	{ WSA883X_OTP_REG_10, 0xFF },
	{ WSA883X_OTP_REG_11, 0xFF },
	{ WSA883X_OTP_REG_12, 0xFF },
	{ WSA883X_OTP_REG_13, 0xFF },
	{ WSA883X_OTP_REG_14, 0xFF },
	{ WSA883X_OTP_REG_15, 0xFF },
	{ WSA883X_OTP_REG_16, 0xFF },
	{ WSA883X_OTP_REG_17, 0xFF },
	{ WSA883X_OTP_REG_18, 0xFF },
	{ WSA883X_OTP_REG_19, 0xFF },
	{ WSA883X_OTP_REG_20, 0xFF },
	{ WSA883X_OTP_REG_21, 0xFF },
	{ WSA883X_OTP_REG_22, 0xFF },
	{ WSA883X_OTP_REG_23, 0xFF },
	{ WSA883X_OTP_REG_24, 0x37 },
	{ WSA883X_OTP_REG_25, 0x3F },
	{ WSA883X_OTP_REG_26, 0x03 },
	{ WSA883X_OTP_REG_27, 0x00 },
	{ WSA883X_OTP_REG_28, 0x00 },
	{ WSA883X_OTP_REG_29, 0x00 },
	{ WSA883X_OTP_REG_30, 0x00 },
	{ WSA883X_OTP_REG_31, 0x03 },
	{ WSA883X_OTP_REG_32, 0x00 },
	{ WSA883X_OTP_REG_33, 0xFF },
	{ WSA883X_OTP_REG_34, 0x00 },
	{ WSA883X_OTP_REG_35, 0x00 },
	{ WSA883X_OTP_REG_63, 0x40 },
	{ WSA883X_EMEM_0, 0x00 },
	{ WSA883X_EMEM_1, 0x00 },
	{ WSA883X_EMEM_2, 0x00 },
	{ WSA883X_EMEM_3, 0x00 },
	{ WSA883X_EMEM_4, 0x00 },
	{ WSA883X_EMEM_5, 0x00 },
	{ WSA883X_EMEM_6, 0x00 },
	{ WSA883X_EMEM_7, 0x00 },
	{ WSA883X_EMEM_8, 0x00 },
	{ WSA883X_EMEM_9, 0x00 },
	{ WSA883X_EMEM_10, 0x00 },
	{ WSA883X_EMEM_11, 0x00 },
	{ WSA883X_EMEM_12, 0x00 },
	{ WSA883X_EMEM_13, 0x00 },
	{ WSA883X_EMEM_14, 0x00 },
	{ WSA883X_EMEM_15, 0x00 },
	{ WSA883X_EMEM_16, 0x00 },
	{ WSA883X_EMEM_17, 0x00 },
	{ WSA883X_EMEM_18, 0x00 },
	{ WSA883X_EMEM_19, 0x00 },
	{ WSA883X_EMEM_20, 0x00 },
	{ WSA883X_EMEM_21, 0x00 },
	{ WSA883X_EMEM_22, 0x00 },
	{ WSA883X_EMEM_23, 0x00 },
	{ WSA883X_EMEM_24, 0x00 },
	{ WSA883X_EMEM_25, 0x00 },
	{ WSA883X_EMEM_26, 0x00 },
	{ WSA883X_EMEM_27, 0x00 },
	{ WSA883X_EMEM_28, 0x00 },
	{ WSA883X_EMEM_29, 0x00 },
	{ WSA883X_EMEM_30, 0x00 },
	{ WSA883X_EMEM_31, 0x00 },
	{ WSA883X_EMEM_32, 0x00 },
	{ WSA883X_EMEM_33, 0x00 },
	{ WSA883X_EMEM_34, 0x00 },
	{ WSA883X_EMEM_35, 0x00 },
	{ WSA883X_EMEM_36, 0x00 },
	{ WSA883X_EMEM_37, 0x00 },
	{ WSA883X_EMEM_38, 0x00 },
	{ WSA883X_EMEM_39, 0x00 },
	{ WSA883X_EMEM_40, 0x00 },
	{ WSA883X_EMEM_41, 0x00 },
	{ WSA883X_EMEM_42, 0x00 },
	{ WSA883X_EMEM_43, 0x00 },
	{ WSA883X_EMEM_44, 0x00 },
	{ WSA883X_EMEM_45, 0x00 },
	{ WSA883X_EMEM_46, 0x00 },
	{ WSA883X_EMEM_47, 0x00 },
	{ WSA883X_EMEM_48, 0x00 },
	{ WSA883X_EMEM_49, 0x00 },
	{ WSA883X_EMEM_50, 0x00 },
	{ WSA883X_EMEM_51, 0x00 },
	{ WSA883X_EMEM_52, 0x00 },
	{ WSA883X_EMEM_53, 0x00 },
	{ WSA883X_EMEM_54, 0x00 },
	{ WSA883X_EMEM_55, 0x00 },
	{ WSA883X_EMEM_56, 0x00 },
	{ WSA883X_EMEM_57, 0x00 },
	{ WSA883X_EMEM_58, 0x00 },
	{ WSA883X_EMEM_59, 0x00 },
	{ WSA883X_EMEM_60, 0x00 },
	{ WSA883X_EMEM_61, 0x00 },
	{ WSA883X_EMEM_62, 0x00 },
	{ WSA883X_EMEM_63, 0x00 },
}

unsafe extern "C" fn wsa883x_readonly_register(dev: *mut device, reg: c_uint) . bool
{
	match reg {
	WSA883X_DOUT_MSB => {
	WSA883X_DOUT_LSB => {
	WSA883X_STATUS => {
	WSA883X_PA_STATUS0 => {
	WSA883X_PA_STATUS1 => {
	WSA883X_PA_STATUS2 => {
	WSA883X_STATUS_0 => {
	WSA883X_STATUS_1 => {
	WSA883X_CHIP_ID0 => {
	WSA883X_CHIP_ID1 => {
	WSA883X_CHIP_ID2 => {
	WSA883X_CHIP_ID3 => {
	WSA883X_BUS_ID => {
	WSA883X_PA_FSM_STA => {
	WSA883X_PA_FSM_ERR_COND => {
	WSA883X_TEMP_MSB => {
	WSA883X_TEMP_LSB => {
	WSA883X_VBAT_DIN_MSB => {
	WSA883X_VBAT_DIN_LSB => {
	WSA883X_VBAT_DOUT => {
	WSA883X_SDM_PDM9_LSB => {
	WSA883X_SDM_PDM9_MSB => {
	WSA883X_WAVG_STA => {
	WSA883X_INTR_STATUS0 => {
	WSA883X_INTR_STATUS1 => {
	WSA883X_OTP_CTRL1 => {
	WSA883X_PIN_STATUS => {
	WSA883X_ATE_TEST_MODE => {
	WSA883X_SWR_HM_TEST1 => {
	WSA883X_SPARE_R => {
	WSA883X_OTP_REG_0 => {
	    return true;
	}
    return false;
}

unsafe extern "C" fn wsa883x_writeable_register(dev: *mut device, reg: c_uint) . bool
{
    return !wsa883x_readonly_register(dev, reg);
}

unsafe extern "C" fn wsa883x_volatile_register(dev: *mut device, reg: c_uint) . bool
{
    return wsa883x_readonly_register(dev, reg);
}

static const struct regmap_config wsa883x_regmap_config = {
	reg_bits: 32,
	val_bits: 8,
	cache_type: REGCACHE_MAPLE,
	reg_defaults: wsa883x_defaults,
	max_register: WSA883X_MAX_REGISTER,
	num_reg_defaults: ARRAY_SIZE(wsa883x_defaults),
	volatile_reg: wsa883x_volatile_register,
	writeable_reg: wsa883x_writeable_register,
	reg_format_endian: REGMAP_ENDIAN_NATIVE,
	val_format_endian: REGMAP_ENDIAN_NATIVE,
	use_single_read: true,
}

static reg_init: &[reg_sequence] = &[
	{WSA883X_PA_FSM_BYP, 0x00},
	{WSA883X_ADC_6, 0x02},
	{WSA883X_CDC_SPK_DSM_A2_0, 0x0A},
	{WSA883X_CDC_SPK_DSM_A2_1, 0x08},
	{WSA883X_CDC_SPK_DSM_A3_0, 0xF3},
	{WSA883X_CDC_SPK_DSM_A3_1, 0x07},
	{WSA883X_CDC_SPK_DSM_A4_0, 0x79},
	{WSA883X_CDC_SPK_DSM_A4_1, 0x02},
	{WSA883X_CDC_SPK_DSM_A5_0, 0x0B},
	{WSA883X_CDC_SPK_DSM_A5_1, 0x02},
	{WSA883X_CDC_SPK_DSM_A6_0, 0x8A},
	{WSA883X_CDC_SPK_DSM_A7_0, 0x9B},
	{WSA883X_CDC_SPK_DSM_C_0, 0x68},
	{WSA883X_CDC_SPK_DSM_C_1, 0x54},
	{WSA883X_CDC_SPK_DSM_C_2, 0xF2},
	{WSA883X_CDC_SPK_DSM_C_3, 0x20},
	{WSA883X_CDC_SPK_DSM_R1, 0x83},
	{WSA883X_CDC_SPK_DSM_R2, 0x7F},
	{WSA883X_CDC_SPK_DSM_R3, 0x9D},
	{WSA883X_CDC_SPK_DSM_R4, 0x82},
	{WSA883X_CDC_SPK_DSM_R5, 0x8B},
	{WSA883X_CDC_SPK_DSM_R6, 0x9B},
	{WSA883X_CDC_SPK_DSM_R7, 0x3F},
	{WSA883X_VBAT_SNS, 0x20},
	{WSA883X_DRE_CTL_0, 0x92},
	{WSA883X_DRE_IDLE_DET_CTL, 0x0F},
	{WSA883X_CURRENT_LIMIT, 0xC4},
	{WSA883X_VAGC_TIME, 0x0F},
	{WSA883X_VAGC_ATTN_LVL_1_2, 0x00},
	{WSA883X_VAGC_ATTN_LVL_3, 0x01},
	{WSA883X_VAGC_CTL, 0x01},
	{WSA883X_TAGC_CTL, 0x1A},
	{WSA883X_TAGC_TIME, 0x2C},
	{WSA883X_TEMP_CONFIG0, 0x02},
	{WSA883X_TEMP_CONFIG1, 0x02},
	{WSA883X_OTP_REG_1, 0x49},
	{WSA883X_OTP_REG_2, 0x80},
	{WSA883X_OTP_REG_3, 0xC9},
	{WSA883X_OTP_REG_4, 0x40},
	{WSA883X_TAGC_CTL, 0x1B},
	{WSA883X_ADC_2, 0x00},
	{WSA883X_ADC_7, 0x85},
	{WSA883X_ADC_7, 0x87},
	{WSA883X_CKWD_CTL_0, 0x14},
	{WSA883X_CKWD_CTL_1, 0x1B},
	{WSA883X_GMAMP_SUP1, 0xE2},
}

unsafe extern "C" fn wsa883x_init(wsa883x: *mut wsa883x_priv) . c_int
{
	let regmap: *mut regmap = wsa883x.regmap;
	int variant, version, ret;

	if wsa883x.hw_init
	    return 0;

	ret = regmap_read(regmap, WSA883X_OTP_REG_0, &variant);
	if ret
	    return ret;
	variant = variant & WSA883X_ID_MASK;

	ret = regmap_read(regmap, WSA883X_CHIP_ID0, &version);
	if ret
	    return ret;

	match variant {
	WSA8830 => {
		dev_dbg(wsa883x.dev, "WSA883X Version 1_%d, Variant: WSA8830\n",
			version);
		},
	WSA8835 => {
		dev_dbg(wsa883x.dev, "WSA883X Version 1_%d, Variant: WSA8835\n",
			version);
		},
	WSA8832 => {
		dev_dbg(wsa883x.dev, "WSA883X Version 1_%d, Variant: WSA8832\n",
			version);
		},
	WSA8835_V2 => {
		dev_dbg(wsa883x.dev, "WSA883X Version 1_%d, Variant: WSA8835_V2\n",
			version);
		},
	_ => {
		dev_warn(wsa883x.dev, "unknown variant: %d\n", variant);
		},
	}

	wsa883x.comp_offset = COMP_OFFSET2;

	/* Initial settings */
	regmap_multi_reg_write(regmap, reg_init, ARRAY_SIZE(reg_init));

	if variant == WSA8830 || variant == WSA8832 {
		wsa883x.comp_offset = COMP_OFFSET3;
		regmap_update_bits(regmap, WSA883X_DRE_CTL_0,
				   WSA883X_DRE_OFFSET_MASK,
				   wsa883x.comp_offset);
	}

	wsa883x.hw_init = true;

    return 0;
}

unsafe extern "C" fn wsa883x_update_status(let slave: *mut sdw_slave,
				 enum sdw_slave_status status)
{
	let wsa883x: *mut wsa883x_priv = dev_get_drvdata(&slave.dev);

	if status == SDW_SLAVE_UNATTACHED
		wsa883x.hw_init = false;

	if status == SDW_SLAVE_ATTACHED && slave.dev_num > 0
	    return wsa883x_init(wsa883x);

    return 0;
}

unsafe extern "C" fn wsa883x_port_prep(let slave: *mut sdw_slave,
			     let prepare_ch: *mut sdw_prepare_ch,
			     enum sdw_port_prep_ops state)
{
	let wsa883x: *mut wsa883x_priv = dev_get_drvdata(&slave.dev);

	if state == SDW_OPS_PORT_POST_PREP
		wsa883x.port_prepared[prepare_ch.num - 1] = true;
	else
		wsa883x.port_prepared[prepare_ch.num - 1] = false;

    return 0;
}

static const struct sdw_slave_ops wsa883x_slave_ops = {
	update_status: wsa883x_update_status,
	port_prep: wsa883x_port_prep,
}

unsafe extern "C" fn wsa_dev_mode_get(let kcontrol: *mut snd_kcontrol,
			    let ucontrol: *mut snd_ctl_elem_value)
{
	let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
	let wsa883x: *mut wsa883x_priv = snd_soc_component_get_drvdata(component);

	ucontrol.value.enumerated.item[0] = wsa883x.dev_mode;

    return 0;
}

unsafe extern "C" fn wsa_dev_mode_put(let kcontrol: *mut snd_kcontrol,
			    let ucontrol: *mut snd_ctl_elem_value)
{
	let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
	let wsa883x: *mut wsa883x_priv = snd_soc_component_get_drvdata(component);

	if wsa883x.dev_mode == ucontrol.value.enumerated.item[0]
	    return 0;

	wsa883x.dev_mode = ucontrol.value.enumerated.item[0];

    return 1;
}

static const SNDRV_CTL_TLVD_DECLARE_DB_RANGE(pa_gain,
	0, 14, TLV_DB_SCALE_ITEM(-300, 0, 0),
	15, 29, TLV_DB_SCALE_ITEM(-300, 150, 0),
	30, 31, TLV_DB_SCALE_ITEM(1800, 0, 0),
);

unsafe extern "C" fn wsa883x_get_swr_port(let kcontrol: *mut snd_kcontrol,
				let ucontrol: *mut snd_ctl_elem_value)
{
	let comp: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
	let data: *mut wsa883x_priv = snd_soc_component_get_drvdata(comp);
	let mixer: *mut soc_mixer_control = (struct soc_mixer_control *)kcontrol.private_value;
	int portidx = mixer.reg;

	ucontrol.value.integer.value[0] = data.port_enable[portidx];

    return 0;
}

unsafe extern "C" fn wsa883x_set_swr_port(let kcontrol: *mut snd_kcontrol,
				let ucontrol: *mut snd_ctl_elem_value)
{
	let comp: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
	let data: *mut wsa883x_priv = snd_soc_component_get_drvdata(comp);
	let mixer: *mut soc_mixer_control = (struct soc_mixer_control *)kcontrol.private_value;
	int portidx = mixer.reg;

	if ucontrol.value.integer.value[0] {
		if data.port_enable[portidx]
		    return 0;

		data.port_enable[portidx] = true;
	} else {
		if !data.port_enable[portidx]
		    return 0;

		data.port_enable[portidx] = false;
	}

    return 1;
}

unsafe extern "C" fn wsa883x_get_comp_offset(let kcontrol: *mut snd_kcontrol,
				   let ucontrol: *mut snd_ctl_elem_value)
{
	let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
	let wsa883x: *mut wsa883x_priv = snd_soc_component_get_drvdata(component);

	ucontrol.value.integer.value[0] = wsa883x.comp_offset;

    return 0;
}

unsafe extern "C" fn wsa883x_set_comp_offset(let kcontrol: *mut snd_kcontrol,
				   let ucontrol: *mut snd_ctl_elem_value)
{
	let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
	let wsa883x: *mut wsa883x_priv = snd_soc_component_get_drvdata(component);

	if wsa883x.comp_offset == ucontrol.value.integer.value[0]
	    return 0;

	wsa883x.comp_offset = ucontrol.value.integer.value[0];

    return 1;
}

unsafe extern "C" fn wsa883x_codec_probe(let comp: *mut snd_soc_component)
{
	let wsa883x: *mut wsa883x_priv = snd_soc_component_get_drvdata(comp);

	snd_soc_component_init_regmap(comp, wsa883x.regmap);

    return 0;
}

unsafe extern "C" fn wsa883x_spkr_event(let w: *mut snd_soc_dapm_widget,
			      let kcontrol: *mut snd_kcontrol, int event)
{
	let component: *mut snd_soc_component = snd_soc_dapm_to_component(w.dapm);
	let wsa883x: *mut wsa883x_priv = snd_soc_component_get_drvdata(component);

	match event {
	SND_SOC_DAPM_POST_PMU => {
		scoped_guard(mutex, &wsa883x.sp_lock)
			wsa883x.pa_on = true;

		match wsa883x.dev_mode {
		RECEIVER => {
			snd_soc_component_write_field(component, WSA883X_CDC_PATH_MODE,
						      WSA883X_RXD_MODE_MASK,
						      WSA883X_RXD_MODE_HIFI);
			snd_soc_component_write_field(component, WSA883X_SPKR_PWM_CLK_CTL,
						      WSA883X_SPKR_PWM_FREQ_SEL_MASK,
						      WSA883X_SPKR_PWM_FREQ_F600KHZ);
			snd_soc_component_write_field(component, WSA883X_DRE_CTL_0,
						       WSA883X_DRE_PROG_DELAY_MASK, 0x0);
			},
		SPEAKER => {
			snd_soc_component_write_field(component, WSA883X_CDC_PATH_MODE,
						      WSA883X_RXD_MODE_MASK,
						      WSA883X_RXD_MODE_NORMAL);
			snd_soc_component_write_field(component, WSA883X_SPKR_PWM_CLK_CTL,
						      WSA883X_SPKR_PWM_FREQ_SEL_MASK,
						      WSA883X_SPKR_PWM_FREQ_F300KHZ);
			snd_soc_component_write_field(component, WSA883X_DRE_CTL_0,
						       WSA883X_DRE_PROG_DELAY_MASK, 0x9);
			},
		_ => {
			},
		}

		if wsa883x.port_enable[WSA883X_PORT_COMP]
			snd_soc_component_write_field(component, WSA883X_DRE_CTL_0,
						      WSA883X_DRE_OFFSET_MASK,
						      wsa883x.comp_offset);
		snd_soc_component_write_field(component, WSA883X_VBAT_ADC_FLT_CTL,
					      WSA883X_VBAT_ADC_COEF_SEL_MASK,
					      WSA883X_VBAT_ADC_COEF_F_1DIV16);
		snd_soc_component_write_field(component, WSA883X_VBAT_ADC_FLT_CTL,
					      WSA883X_VBAT_ADC_FLT_EN_MASK, 0x1);
		snd_soc_component_write_field(component, WSA883X_PDM_WD_CTL,
					      WSA883X_PDM_EN_MASK,
					      WSA883X_PDM_ENABLE);

		},
	SND_SOC_DAPM_PRE_PMD => {
		snd_soc_component_write_field(component, WSA883X_VBAT_ADC_FLT_CTL,
					      WSA883X_VBAT_ADC_FLT_EN_MASK, 0x0);
		snd_soc_component_write_field(component, WSA883X_VBAT_ADC_FLT_CTL,
					      WSA883X_VBAT_ADC_COEF_SEL_MASK,
					      WSA883X_VBAT_ADC_COEF_F_1DIV2);
		snd_soc_component_write_field(component, WSA883X_PA_FSM_CTL,
					      WSA883X_GLOBAL_PA_EN_MASK, 0);
		snd_soc_component_write_field(component, WSA883X_PDM_WD_CTL,
					      WSA883X_PDM_EN_MASK, 0);
		scoped_guard(mutex, &wsa883x.sp_lock)
			wsa883x.pa_on = false;
		},
	}
    return 0;
}

static wsa883x_dapm_widgets: &[snd_soc_dapm_widget] = &[
	SND_SOC_DAPM_INPUT("IN"),
	SND_SOC_DAPM_SPK("SPKR", wsa883x_spkr_event),
}

static wsa883x_snd_controls: &[snd_kcontrol_new] = &[
	SOC_SINGLE_RANGE_TLV("PA Volume", WSA883X_DRE_CTL_1, 1,
			     0x0, 0x1f, 1, pa_gain),
	SOC_ENUM_EXT("WSA MODE", wsa_dev_mode_enum,
		     wsa_dev_mode_get, wsa_dev_mode_put),
	SOC_SINGLE_EXT("COMP Offset", SND_SOC_NOPM, 0, 4, 0,
		       wsa883x_get_comp_offset, wsa883x_set_comp_offset),
	SOC_SINGLE_EXT("DAC Switch", WSA883X_PORT_DAC, 0, 1, 0,
		       wsa883x_get_swr_port, wsa883x_set_swr_port),
	SOC_SINGLE_EXT("COMP Switch", WSA883X_PORT_COMP, 0, 1, 0,
		       wsa883x_get_swr_port, wsa883x_set_swr_port),
	SOC_SINGLE_EXT("BOOST Switch", WSA883X_PORT_BOOST, 0, 1, 0,
		       wsa883x_get_swr_port, wsa883x_set_swr_port),
	SOC_SINGLE_EXT("VISENSE Switch", WSA883X_PORT_VISENSE, 0, 1, 0,
		       wsa883x_get_swr_port, wsa883x_set_swr_port),
}

static wsa883x_audio_map: &[snd_soc_dapm_route] = &[
	{"SPKR", core::ptr::null_mut(), "IN"},
}

static const struct snd_soc_component_driver wsa883x_component_drv = {
	name: "WSA883x",
	probe: wsa883x_codec_probe,
	controls: wsa883x_snd_controls,
	num_controls: ARRAY_SIZE(wsa883x_snd_controls),
	dapm_widgets: wsa883x_dapm_widgets,
	num_dapm_widgets: ARRAY_SIZE(wsa883x_dapm_widgets),
	dapm_routes: wsa883x_audio_map,
	num_dapm_routes: ARRAY_SIZE(wsa883x_audio_map),
}

unsafe extern "C" fn wsa883x_hw_params(let substream: *mut snd_pcm_substream,
			     let params: *mut snd_pcm_hw_params,
			     let dai: *mut snd_soc_dai)
{
	let wsa883x: *mut wsa883x_priv = dev_get_drvdata(dai.dev);
	let mut i: c_int;

	wsa883x.active_ports = 0;
	for (i = 0; i < WSA883X_MAX_SWR_PORTS; i++) {
		if !wsa883x.port_enable[i]
			continue;

		wsa883x.port_config[wsa883x.active_ports] = wsa883x_pconfig[i];
		wsa883x.active_ports++;
	}

	wsa883x.sconfigframe_rate: params_rate(params);

    return sdw_stream_add_slave(wsa883x.slave, &wsa883x.sconfig,
				    wsa883x.port_config, wsa883x.active_ports,
				    wsa883x.sruntime);
}

unsafe extern "C" fn wsa883x_hw_free(let substream: *mut snd_pcm_substream,
			   let dai: *mut snd_soc_dai)
{
	let wsa883x: *mut wsa883x_priv = dev_get_drvdata(dai.dev);

	sdw_stream_remove_slave(wsa883x.slave, wsa883x.sruntime);

    return 0;
}

unsafe extern "C" fn wsa883x_set_sdw_stream(let dai: *mut snd_soc_dai,
				  void *stream, int direction)
{
	let wsa883x: *mut wsa883x_priv = dev_get_drvdata(dai.dev);

	wsa883x.sruntime = stream;

    return 0;
}

unsafe extern "C" fn wsa883x_digital_mute(let dai: *mut snd_soc_dai, int mute, int stream)
{
	let component: *mut snd_soc_component = dai.component;

	if mute {
		snd_soc_component_write_field(component, WSA883X_DRE_CTL_1,
					      WSA883X_DRE_GAIN_EN_MASK, 0);
		snd_soc_component_write_field(component, WSA883X_PA_FSM_CTL,
					      WSA883X_GLOBAL_PA_EN_MASK, 0);

	} else {
		snd_soc_component_write_field(component, WSA883X_DRE_CTL_1,
					      WSA883X_DRE_GAIN_EN_MASK,
					      WSA883X_DRE_GAIN_FROM_CSR);
		snd_soc_component_write_field(component, WSA883X_PA_FSM_CTL,
					      WSA883X_GLOBAL_PA_EN_MASK,
					      WSA883X_GLOBAL_PA_ENABLE);

	}

    return 0;
}

static const struct snd_soc_dai_ops wsa883x_dai_ops = {
	hw_params: wsa883x_hw_params,
	hw_free: wsa883x_hw_free,
	mute_stream: wsa883x_digital_mute,
	set_stream: wsa883x_set_sdw_stream,
	mute_unmute_on_trigger: true,
}

static struct snd_soc_dai_driver wsa883x_dais[] = {
	{
		name: "SPKR",
		playback: {
			stream_name: "SPKR Playback",
			rates: WSA883X_RATES | WSA883X_FRAC_RATES,
			formats: WSA883X_FORMATS,
			rate_min: 8000,
			rate_max: 352800,
			channels_min: 1,
			channels_max: 1,
		},
		ops: &wsa883x_dai_ops,
	},
}

unsafe extern "C" fn wsa883x_get_temp(let wsa883x: *mut wsa883x_priv, long *temp)
{
	let mut d1_msb = 0, d1_lsb = 0, d2_msb = 0, d2_lsb = 0: c_uint;
	let mut dmeas_msb = 0, dmeas_lsb = 0: c_uint;
	int d1, d2, dmeas;
	unsigned let mut mask: c_int;
	int ret, range;
	let mut val: c_long;

	guard(mutex)(&wsa883x.sp_lock);

	if wsa883x.pa_on {
		/*
		 * Reading temperature is possible only when Power Amplifier is
		 * off. Report last cached data.
		 */
		*temp = wsa883x.temperature * 1000;
	    return 0;
	}

	ret = pm_runtime_resume_and_get(wsa883x.dev);
	if ret < 0
	    return ret;

	mask = WSA883X_PA_FSM_BYP_DC_CAL_EN_MASK |
	       WSA883X_PA_FSM_BYP_CLK_WD_EN_MASK |
	       WSA883X_PA_FSM_BYP_BG_EN_MASK |
	       WSA883X_PA_FSM_BYP_D_UNMUTE_MASK |
	       WSA883X_PA_FSM_BYP_SPKR_PROT_EN_MASK |
	       WSA883X_PA_FSM_BYP_TSADC_EN_MASK;

	/*
	 * Here and further do not care about read or update failures.
	 * For example, before turning the amplifier on for the first
	 * time, reading WSA883X_TEMP_DIN_MSB will always return 0.
	 * Instead, check if returned value is within reasonable
	 * thresholds.
	 */
	regmap_update_bits(wsa883x.regmap, WSA883X_PA_FSM_BYP, mask, mask);

	regmap_update_bits(wsa883x.regmap, WSA883X_TADC_VALUE_CTL,
			   WSA883X_TADC_VALUE_CTL_TEMP_VALUE_RD_EN_MASK,
			   FIELD_PREP(WSA883X_TADC_VALUE_CTL_TEMP_VALUE_RD_EN_MASK, 0x0));

	regmap_read(wsa883x.regmap, WSA883X_TEMP_MSB, &dmeas_msb);
	regmap_read(wsa883x.regmap, WSA883X_TEMP_LSB, &dmeas_lsb);

	regmap_update_bits(wsa883x.regmap, WSA883X_TADC_VALUE_CTL,
			   WSA883X_TADC_VALUE_CTL_TEMP_VALUE_RD_EN_MASK,
			   FIELD_PREP(WSA883X_TADC_VALUE_CTL_TEMP_VALUE_RD_EN_MASK, 0x1));

	regmap_read(wsa883x.regmap, WSA883X_OTP_REG_1, &d1_msb);
	regmap_read(wsa883x.regmap, WSA883X_OTP_REG_2, &d1_lsb);
	regmap_read(wsa883x.regmap, WSA883X_OTP_REG_3, &d2_msb);
	regmap_read(wsa883x.regmap, WSA883X_OTP_REG_4, &d2_lsb);

	regmap_update_bits(wsa883x.regmap, WSA883X_PA_FSM_BYP, mask, 0x0);

	dmeas = (((dmeas_msb & 0xff) << 0x8) | (dmeas_lsb & 0xff)) >> 0x6;
	d1 = (((d1_msb & 0xff) << 0x8) | (d1_lsb & 0xff)) >> 0x6;
	d2 = (((d2_msb & 0xff) << 0x8) | (d2_lsb & 0xff)) >> 0x6;

	if d1 == d2 {
		/* Incorrect data in OTP? */
		ret = -EINVAL;
		goto out;
	}

	val = WSA883X_T1_TEMP + (((dmeas - d1) * (WSA883X_T2_TEMP - WSA883X_T1_TEMP)) / (d2 - d1));
	range = WSA883X_HIGH_TEMP_THRESHOLD - WSA883X_LOW_TEMP_THRESHOLD;
	if in_range(val, WSA883X_LOW_TEMP_THRESHOLD, range) {
		wsa883x.temperature = val;
		*temp = val * 1000;
		ret = 0;
	} else {
		ret = -EAGAIN;
	}
out:
	pm_runtime_put_autosuspend(wsa883x.dev);

    return ret;
}

unsafe extern "C" fn wsa883x_hwmon_is_visible(const void *data,
					enum hwmon_sensor_types type, u32 attr,
					int channel)
{
	if type != hwmon_temp
	    return 0;

	match attr {
	hwmon_temp_input => {
	    return 0444;
	_ => {
		},
	}

    return 0;
}

unsafe extern "C" fn wsa883x_hwmon_read(let dev: *mut device,
			      enum hwmon_sensor_types type,
			      u32 attr, int channel, long *temp)
{
	let mut ret: c_int;

	match attr {
	hwmon_temp_input => {
		ret = wsa883x_get_temp(dev_get_drvdata(dev), temp);
		},
	_ => {
		ret = -EOPNOTSUPP;
		},
	}

    return ret;
}

static const let const: *mut hwmon_channel_info wsa883x_hwmon_info[] = {
	HWMON_CHANNEL_INFO(temp, HWMON_T_INPUT),
	core::ptr::null_mut()
}

static const struct hwmon_ops wsa883x_hwmon_ops = {
	.is_visible	= wsa883x_hwmon_is_visible,
	.read		= wsa883x_hwmon_read,
}

static const struct hwmon_chip_info wsa883x_hwmon_chip_info = {
	.ops	= &wsa883x_hwmon_ops,
	.info	= wsa883x_hwmon_info,
}

unsafe extern "C" fn wsa883x_reset_assert(data: *mut c_void)
{
	let wsa883x: *mut wsa883x_priv = data;

	if wsa883x.sd_reset
		reset_control_assert(wsa883x.sd_reset);
	else
		gpiod_direction_output(wsa883x.sd_n, 1);
}

unsafe extern "C" fn wsa883x_reset_deassert(wsa883x: *mut wsa883x_priv)
{
	if wsa883x.sd_reset
		reset_control_deassert(wsa883x.sd_reset);
	else
		gpiod_direction_output(wsa883x.sd_n, 0);
}

unsafe extern "C" fn wsa883x_get_reset(let dev: *mut device, let wsa883x: *mut wsa883x_priv)
{
	wsa883x.sd_reset = devm_reset_control_get_optional_shared(dev, core::ptr::null_mut());
	if IS_ERR(wsa883x.sd_reset)
	    return dev_err_probe(dev, PTR_ERR(wsa883x.sd_reset),
				     "Failed to get reset\n");

	 /* if sd_reset: core::ptr::null_mut(), so use the backwards compatible way for powerdown-gpios */
	if !wsa883x.sd_reset {
		wsa883x.sd_n = devm_gpiod_get_optional(dev, "powerdown",
							GPIOD_OUT_HIGH);
		if IS_ERR(wsa883x.sd_n)
		    return dev_err_probe(dev, PTR_ERR(wsa883x.sd_n),
					     "Shutdown Control GPIO not found\n");
	}

    return 0;
}

unsafe extern "C" fn wsa883x_probe(let pdev: *mut sdw_slave,
			 const let id: *mut sdw_device_id)
{
	let wsa883x: *mut wsa883x_priv;
	let dev: *mut device = &pdev.dev;
	let mut ret: c_int;

	wsa883x = devm_kzalloc(dev, sizeof(*wsa883x), GFP_KERNEL);
	if !wsa883x
	    return -ENOMEM;

	wsa883x.vdd = devm_regulator_get(dev, "vdd");
	if IS_ERR(wsa883x.vdd)
	    return dev_err_probe(dev, PTR_ERR(wsa883x.vdd),
				     "No vdd regulator found\n");

	ret = regulator_enable(wsa883x.vdd);
	if ret
	    return dev_err_probe(dev, ret, "Failed to enable vdd regulator\n");

	ret = wsa883x_get_reset(dev, wsa883x);
	if ret
		goto err;

	dev_set_drvdata(dev, wsa883x);
	wsa883x.slave = pdev;
	wsa883x.dev = dev;
	wsa883x.sconfigch_count: 1;
	wsa883x.sconfigbps: 1;
	wsa883x.sconfigdirection: SDW_DATA_DIR_RX;
	wsa883x.sconfigtype_: SDW_STREAM_PDM;
	mutex_init(&wsa883x.sp_lock);

	/*
	 * Port map index starts with 0, however the data port for this codec
	 * are from index 1
	 */
	if (of_property_read_u32_array(dev.of_node, "qcom,port-mapping", &pdev.m_port_map[1],
					WSA883X_MAX_SWR_PORTS))
		dev_dbg(dev, "Static Port mapping not specified\n");

	pdev.propsink_ports: GENMASK(WSA883X_MAX_SWR_PORTS - 1, 0);
	pdev.propsimple_clk_stop_capable: true;
	pdev.propsink_dpn_prop: wsa_sink_dpn_prop;
	pdev.propscp_int1_mask: SDW_SCP_INT1_BUS_CLASH | SDW_SCP_INT1_PARITY;

	wsa883x_reset_deassert(wsa883x);
	ret = devm_add_action_or_reset(dev, wsa883x_reset_assert, wsa883x);
	if ret
	    return ret;

	wsa883x.regmap = devm_regmap_init_sdw(pdev, &wsa883x_regmap_config);
	if IS_ERR(wsa883x.regmap) {
		ret = dev_err_probe(dev, PTR_ERR(wsa883x.regmap),
				    "regmap_init failed\n");
		goto err;
	}

	if IS_REACHABLE(CONFIG_HWMON) {
		let hwmon: *mut device;

		hwmon = devm_hwmon_device_register_with_info(dev, "wsa883x",
							     wsa883x,
							     &wsa883x_hwmon_chip_info,
							     core::ptr::null_mut());
		if IS_ERR(hwmon)
		    return dev_err_probe(dev, PTR_ERR(hwmon),
					     "Failed to register hwmon sensor\n");
	}

	pm_runtime_set_autosuspend_delay(dev, 3000);
	pm_runtime_use_autosuspend(dev);
	pm_runtime_mark_last_busy(dev);
	pm_runtime_set_active(dev);
	pm_runtime_enable(dev);

	ret = devm_snd_soc_register_component(dev,
					      &wsa883x_component_drv,
					       wsa883x_dais,
					       ARRAY_SIZE(wsa883x_dais));
err:
	if ret
		regulator_disable(wsa883x.vdd);

    return ret;

}

unsafe extern "C" fn wsa883x_runtime_suspend(let dev: *mut device)
{
	let regmap: *mut regmap = dev_get_regmap(dev, core::ptr::null_mut());

	regcache_cache_only(regmap, true);
	regcache_mark_dirty(regmap);

    return 0;
}

unsafe extern "C" fn wsa883x_runtime_resume(let dev: *mut device)
{
	let regmap: *mut regmap = dev_get_regmap(dev, core::ptr::null_mut());
	let mut ret: c_int;

	regcache_cache_only(regmap, false);
	ret = regcache_sync(regmap);
	if ret {
		regcache_cache_only(regmap, true);
		regcache_mark_dirty(regmap);
	    return ret;
	}

    return 0;
}

static const struct dev_pm_ops wsa883x_pm_ops = {
	RUNTIME_PM_OPS(wsa883x_runtime_suspend, wsa883x_runtime_resume, core::ptr::null_mut())
}

static wsa883x_swr_id: &[sdw_device_id] = &[
	SDW_SLAVE_ENTRY(0x0217, 0x0202, 0),
	{},
}


static struct sdw_driver wsa883x_codec_driver = {
	driver: {
		name: "wsa883x-codec",
		pm: pm_ptr(&wsa883x_pm_ops),
		suppress_bind_attrs: true,
	},
	probe: wsa883x_probe,
	ops: &wsa883x_slave_ops,
	id_table: wsa883x_swr_id,
}



// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
