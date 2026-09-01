// SPDX-License-Identifier: GPL-2.0
//
// rt1016.c  --  RT1016 ALSA SoC audio amplifier driver
//
// Copyright 2020 Realtek Semiconductor Corp.
// Author: Oder Chiou <oder_chiou@realtek.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

// C includes removed. The following items are supplied by the surrounding
// Linux/ALSA/Realtek driver bindings.

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
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
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub id: c_int,
}

#[repr(C)]
pub struct rl6231_pll_code {
    pub m_bp: c_int,
    pub m_code: c_int,
    pub n_code: c_int,
    pub k_bp: c_int,
    pub k_code: c_int,
}

#[repr(C)]
pub struct rt1016_priv {
    pub regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub sysclk: c_uint,
    pub sysclk_src: c_int,
    pub lrck: c_uint,
    pub bclk: c_uint,
    pub master: c_int,
    pub pll_in: c_uint,
    pub pll_out: c_uint,
    pub pll_src: c_int,
}

#[repr(C)]
pub struct reg_sequence {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct reg_default {
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
    pub connected: Option<
        unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_soc_dapm_widget) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
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
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const c_void,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub set_sysclk:
        Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_int) -> c_int>,
    pub set_pll: Option<
        unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_uint) -> c_int,
    >,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: [c_char; 9],
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub acpi_match_table: *const acpi_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut i2c_client)>,
    pub id_table: *const i2c_device_id,
}

unsafe extern "C" {
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn rl6231_get_clk_info(sysclk: c_uint, lrck: c_uint) -> c_int;
    fn snd_soc_params_to_frame_size(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint)
        -> c_int;
    fn rl6231_pll_calc(freq_in: c_uint, freq_out: c_uint, pll_code: *mut rl6231_pll_code)
        -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut c_void;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_register_patch(
        map: *mut regmap,
        regs: *const reg_sequence,
        num_regs: c_uint,
    ) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;

// Register, bit-mask, DAI format, PCM format, cache, allocation, and device ID
// constants are supplied by rt1016.h, rl6231.h, and Linux/ALSA bindings.

static rt1016_patch: [reg_sequence; 7] = [
    reg_sequence { reg: RT1016_VOL_CTRL_3, def: 0x8900 },
    reg_sequence { reg: RT1016_ANA_CTRL_1, def: 0xa002 },
    reg_sequence { reg: RT1016_ANA_CTRL_2, def: 0x0002 },
    reg_sequence { reg: RT1016_CLOCK_4, def: 0x6700 },
    reg_sequence { reg: RT1016_CLASSD_3, def: 0xdc55 },
    reg_sequence { reg: RT1016_CLASSD_4, def: 0x376a },
    reg_sequence { reg: RT1016_CLASSD_5, def: 0x009f },
];

static rt1016_reg: [reg_default; 48] = [
    reg_default { reg: 0x00, def: 0x0000 },
    reg_default { reg: 0x01, def: 0x5400 },
    reg_default { reg: 0x02, def: 0x5506 },
    reg_default { reg: 0x03, def: 0xf800 },
    reg_default { reg: 0x04, def: 0x0000 },
    reg_default { reg: 0x05, def: 0xbfbf },
    reg_default { reg: 0x06, def: 0x8900 },
    reg_default { reg: 0x07, def: 0xa002 },
    reg_default { reg: 0x08, def: 0x0000 },
    reg_default { reg: 0x09, def: 0x0000 },
    reg_default { reg: 0x0a, def: 0x0000 },
    reg_default { reg: 0x0c, def: 0x0000 },
    reg_default { reg: 0x0d, def: 0x0000 },
    reg_default { reg: 0x0e, def: 0x10ec },
    reg_default { reg: 0x0f, def: 0x6595 },
    reg_default { reg: 0x11, def: 0x0002 },
    reg_default { reg: 0x1c, def: 0x0000 },
    reg_default { reg: 0x1d, def: 0x0000 },
    reg_default { reg: 0x1e, def: 0x0000 },
    reg_default { reg: 0x1f, def: 0xf000 },
    reg_default { reg: 0x20, def: 0x0000 },
    reg_default { reg: 0x21, def: 0x6000 },
    reg_default { reg: 0x22, def: 0x0000 },
    reg_default { reg: 0x23, def: 0x6700 },
    reg_default { reg: 0x24, def: 0x0000 },
    reg_default { reg: 0x25, def: 0x0000 },
    reg_default { reg: 0x26, def: 0x0000 },
    reg_default { reg: 0x40, def: 0x0018 },
    reg_default { reg: 0x60, def: 0x00a5 },
    reg_default { reg: 0x80, def: 0x0010 },
    reg_default { reg: 0x81, def: 0x0009 },
    reg_default { reg: 0x82, def: 0x0000 },
    reg_default { reg: 0x83, def: 0x0000 },
    reg_default { reg: 0xa0, def: 0x0700 },
    reg_default { reg: 0xc0, def: 0x0080 },
    reg_default { reg: 0xc1, def: 0x02a0 },
    reg_default { reg: 0xc2, def: 0x1400 },
    reg_default { reg: 0xc3, def: 0x0a4a },
    reg_default { reg: 0xc4, def: 0x552a },
    reg_default { reg: 0xc5, def: 0x087e },
    reg_default { reg: 0xc6, def: 0x0020 },
    reg_default { reg: 0xc7, def: 0xa833 },
    reg_default { reg: 0xc8, def: 0x0433 },
    reg_default { reg: 0xc9, def: 0x8040 },
    reg_default { reg: 0xca, def: 0xdc55 },
    reg_default { reg: 0xcb, def: 0x376a },
    reg_default { reg: 0xcc, def: 0x009f },
    reg_default { reg: 0xcf, def: 0x0020 },
];

unsafe extern "C" fn rt1016_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        RT1016_ANA_FLAG
        | RT1016_VERSION2_ID
        | RT1016_VERSION1_ID
        | RT1016_VENDER_ID
        | RT1016_DEVICE_ID
        | RT1016_TEST_SIGNAL
        | RT1016_SC_CTRL_1 => true,
        _ => false,
    }
}

unsafe extern "C" fn rt1016_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        RT1016_RESET
        | RT1016_PADS_CTRL_1
        | RT1016_PADS_CTRL_2
        | RT1016_I2C_CTRL
        | RT1016_VOL_CTRL_1
        | RT1016_VOL_CTRL_2
        | RT1016_VOL_CTRL_3
        | RT1016_ANA_CTRL_1
        | RT1016_MUX_SEL
        | RT1016_RX_I2S_CTRL
        | RT1016_ANA_FLAG
        | RT1016_VERSION2_ID
        | RT1016_VERSION1_ID
        | RT1016_VENDER_ID
        | RT1016_DEVICE_ID
        | RT1016_ANA_CTRL_2
        | RT1016_TEST_SIGNAL
        | RT1016_TEST_CTRL_1
        | RT1016_TEST_CTRL_2
        | RT1016_TEST_CTRL_3
        | RT1016_CLOCK_1
        | RT1016_CLOCK_2
        | RT1016_CLOCK_3
        | RT1016_CLOCK_4
        | RT1016_CLOCK_5
        | RT1016_CLOCK_6
        | RT1016_CLOCK_7
        | RT1016_I2S_CTRL
        | RT1016_DAC_CTRL_1
        | RT1016_SC_CTRL_1
        | RT1016_SC_CTRL_2
        | RT1016_SC_CTRL_3
        | RT1016_SC_CTRL_4
        | RT1016_SIL_DET
        | RT1016_SYS_CLK
        | RT1016_BIAS_CUR
        | RT1016_DAC_CTRL_2
        | RT1016_LDO_CTRL
        | RT1016_CLASSD_1
        | RT1016_PLL1
        | RT1016_PLL2
        | RT1016_PLL3
        | RT1016_CLASSD_2
        | RT1016_CLASSD_OUT
        | RT1016_CLASSD_3
        | RT1016_CLASSD_4
        | RT1016_CLASSD_5
        | RT1016_PWR_CTRL => true,
        _ => false,
    }
}

// static const DECLARE_TLV_DB_SCALE(dac_vol_tlv, -9550, 50, 0);
static dac_vol_tlv: [c_uint; 4] = [0, (-9550i32) as c_uint, 50, 0];

// The C controls/widgets are built by ALSA macros. Preserve their source-level
// definitions as external macro invocations expected from the driver binding.
macro_rules! SOC_DOUBLE_TLV {
    ($($tt:tt)*) => {
        snd_kcontrol_new { _private: [] }
    };
}
macro_rules! SOC_DOUBLE {
    ($($tt:tt)*) => {
        snd_kcontrol_new { _private: [] }
    };
}

static rt1016_snd_controls: [snd_kcontrol_new; 2] = [
    SOC_DOUBLE_TLV!(
        "DAC Playback Volume",
        RT1016_VOL_CTRL_2,
        RT1016_L_VOL_SFT,
        RT1016_R_VOL_SFT,
        191,
        0,
        dac_vol_tlv
    ),
    SOC_DOUBLE!(
        "DAC Playback Switch",
        RT1016_VOL_CTRL_1,
        RT1016_DA_MUTE_L_SFT,
        RT1016_DA_MUTE_R_SFT,
        1,
        1
    ),
];

unsafe extern "C" fn rt1016_is_sys_clk_from_pll(
    source: *mut snd_soc_dapm_widget,
    _sink: *mut snd_soc_dapm_widget,
) -> c_int {
    let component = unsafe { snd_soc_dapm_to_component((*source).dapm) };
    let rt1016 = unsafe { snd_soc_component_get_drvdata(component) as *mut rt1016_priv };

    if unsafe { (*rt1016).sysclk_src } == RT1016_SCLK_S_PLL {
        1
    } else {
        0
    }
}

/* Interface data select */
static rt1016_data_select: [*const c_char; 4] = [
    b"L/R\0".as_ptr() as *const c_char,
    b"R/L\0".as_ptr() as *const c_char,
    b"L/L\0".as_ptr() as *const c_char,
    b"R/R\0".as_ptr() as *const c_char,
];

// static SOC_ENUM_SINGLE_DECL(rt1016_if_data_swap_enum,
//     RT1016_I2S_CTRL, RT1016_I2S_DATA_SWAP_SFT, rt1016_data_select);
// static const struct snd_kcontrol_new rt1016_if_data_swap_mux =
//     SOC_DAPM_ENUM("Data Swap Mux", rt1016_if_data_swap_enum);
// static const struct snd_soc_dapm_widget rt1016_dapm_widgets[] = { ... };
//
// The ALSA DAPM widget constructors are C macros whose concrete struct layout is
// provided by external bindings. Keep the meaningful widget list in source form:
// SND_SOC_DAPM_MUX("Data Swap Mux", SND_SOC_NOPM, 0, 0,
//         &rt1016_if_data_swap_mux),
// SND_SOC_DAPM_SUPPLY("DAC Filter", RT1016_CLOCK_3,
//     RT1016_PWR_DAC_FILTER_BIT, 0, NULL, 0),
// SND_SOC_DAPM_SUPPLY("DAMOD", RT1016_CLOCK_3, RT1016_PWR_DACMOD_BIT, 0,
//     NULL, 0),
// SND_SOC_DAPM_SUPPLY("FIFO", RT1016_CLOCK_3, RT1016_PWR_CLK_FIFO_BIT, 0,
//     NULL, 0),
// SND_SOC_DAPM_SUPPLY("Pure DC", RT1016_CLOCK_3,
//     RT1016_PWR_CLK_PUREDC_BIT, 0, NULL, 0),
// SND_SOC_DAPM_SUPPLY("CLK Silence Det", RT1016_CLOCK_3,
//     RT1016_PWR_SIL_DET_BIT, 0, NULL, 0),
// SND_SOC_DAPM_SUPPLY("RC 25M", RT1016_CLOCK_3, RT1016_PWR_RC_25M_BIT, 0,
//     NULL, 0),
// SND_SOC_DAPM_SUPPLY("PLL1", RT1016_CLOCK_3, RT1016_PWR_PLL1_BIT, 0,
//     NULL, 0),
// SND_SOC_DAPM_SUPPLY("ANA CTRL", RT1016_CLOCK_3, RT1016_PWR_ANA_CTRL_BIT,
//     0, NULL, 0),
// SND_SOC_DAPM_SUPPLY("CLK SYS", RT1016_CLOCK_3, RT1016_PWR_CLK_SYS_BIT,
//     0, NULL, 0),
// SND_SOC_DAPM_SUPPLY("LRCK Det", RT1016_CLOCK_4, RT1016_PWR_LRCK_DET_BIT,
//     0, NULL, 0),
// SND_SOC_DAPM_SUPPLY("BCLK Det", RT1016_CLOCK_4, RT1016_PWR_BCLK_DET_BIT,
//     0, NULL, 0),
// SND_SOC_DAPM_SUPPLY("CKGEN DAC", RT1016_DAC_CTRL_2,
//     RT1016_CKGEN_DAC_BIT, 0, NULL, 0),
// SND_SOC_DAPM_SUPPLY("VCM SLOW", RT1016_CLASSD_1, RT1016_VCM_SLOW_BIT, 0,
//     NULL, 0),
// SND_SOC_DAPM_SUPPLY("Silence Det", RT1016_SIL_DET,
//     RT1016_SIL_DET_EN_BIT, 0, NULL, 0),
// SND_SOC_DAPM_SUPPLY("PLL2", RT1016_PLL2, RT1016_PLL2_EN_BIT, 0, NULL, 0),
// SND_SOC_DAPM_SUPPLY_S("BG1 BG2", 1, RT1016_PWR_CTRL,
//     RT1016_PWR_BG_1_2_BIT, 0, NULL, 0),
// SND_SOC_DAPM_SUPPLY_S("MBIAS BG", 1, RT1016_PWR_CTRL,
//     RT1016_PWR_MBIAS_BG_BIT, 0, NULL, 0),
// SND_SOC_DAPM_SUPPLY_S("PLL", 1, RT1016_PWR_CTRL, RT1016_PWR_PLL_BIT, 0,
//     NULL, 0),
// SND_SOC_DAPM_SUPPLY_S("BASIC", 1, RT1016_PWR_CTRL, RT1016_PWR_BASIC_BIT,
//     0, NULL, 0),
// SND_SOC_DAPM_SUPPLY_S("CLASS D", 1, RT1016_PWR_CTRL,
//     RT1016_PWR_CLSD_BIT, 0, NULL, 0),
// SND_SOC_DAPM_SUPPLY_S("25M", 1, RT1016_PWR_CTRL, RT1016_PWR_25M_BIT, 0,
//     NULL, 0),
// SND_SOC_DAPM_SUPPLY_S("DACL", 1, RT1016_PWR_CTRL, RT1016_PWR_DACL_BIT,
//     0, NULL, 0),
// SND_SOC_DAPM_SUPPLY_S("DACR", 1, RT1016_PWR_CTRL, RT1016_PWR_DACR_BIT,
//     0, NULL, 0),
// SND_SOC_DAPM_SUPPLY_S("LDO2", 1, RT1016_PWR_CTRL, RT1016_PWR_LDO2_BIT,
//     0, NULL, 0),
// SND_SOC_DAPM_SUPPLY_S("VREF", 1, RT1016_PWR_CTRL, RT1016_PWR_VREF_BIT,
//     0, NULL, 0),
// SND_SOC_DAPM_SUPPLY_S("MBIAS", 1, RT1016_PWR_CTRL, RT1016_PWR_MBIAS_BIT,
//     0, NULL, 0),
// SND_SOC_DAPM_AIF_IN("AIFRX", "AIF Playback", 0, SND_SOC_NOPM, 0, 0),
// SND_SOC_DAPM_DAC("DAC", NULL, SND_SOC_NOPM, 0, 0),
// SND_SOC_DAPM_OUTPUT("SPO"),
static rt1016_dapm_widgets: [c_void; 0] = [];

static rt1016_dapm_routes: [snd_soc_dapm_route; 34] = [
    snd_soc_dapm_route { sink: b"Data Swap Mux\0".as_ptr() as *const c_char, control: b"L/R\0".as_ptr() as *const c_char, source: b"AIFRX\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"Data Swap Mux\0".as_ptr() as *const c_char, control: b"R/L\0".as_ptr() as *const c_char, source: b"AIFRX\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"Data Swap Mux\0".as_ptr() as *const c_char, control: b"L/L\0".as_ptr() as *const c_char, source: b"AIFRX\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"Data Swap Mux\0".as_ptr() as *const c_char, control: b"R/R\0".as_ptr() as *const c_char, source: b"AIFRX\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DAC Filter\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DAMOD\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"FIFO\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Pure DC\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Silence Det\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ANA CTRL\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"CLK SYS\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"LRCK Det\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"BCLK Det\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"CKGEN DAC\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"VCM SLOW\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"PLL\0".as_ptr() as *const c_char, control: ptr::null(), source: b"PLL1\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"PLL\0".as_ptr() as *const c_char, control: ptr::null(), source: b"PLL2\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"25M\0".as_ptr() as *const c_char, control: ptr::null(), source: b"RC 25M\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"Silence Det\0".as_ptr() as *const c_char, control: ptr::null(), source: b"CLK Silence Det\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Data Swap Mux\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"BG1 BG2\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"MBIAS BG\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"PLL\0".as_ptr() as *const c_char, connected: Some(rt1016_is_sys_clk_from_pll) },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"BASIC\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"CLASS D\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"25M\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DACL\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DACR\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"LDO2\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"VREF\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"MBIAS\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"SPO\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DAC\0".as_ptr() as *const c_char, connected: None },
];

unsafe extern "C" fn rt1016_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = unsafe { (*dai).component };
    let rt1016 = unsafe { snd_soc_component_get_drvdata(component) as *mut rt1016_priv };
    let mut val_len: c_uint = 0;

    unsafe {
        (*rt1016).lrck = params_rate(params);
    }
    let pre_div = unsafe { rl6231_get_clk_info((*rt1016).sysclk, (*rt1016).lrck) };
    if pre_div < 0 {
        unsafe { dev_err((*component).dev, b"Unsupported clock rate\n\0".as_ptr() as *const c_char) };
        return -EINVAL;
    }

    let frame_size = unsafe { snd_soc_params_to_frame_size(params) };
    if frame_size < 0 {
        unsafe {
            dev_err(
                (*component).dev,
                b"Unsupported frame size: %d\n\0".as_ptr() as *const c_char,
                frame_size,
            )
        };
        return -EINVAL;
    }

    let bclk_ms = (frame_size > 32) as c_int;
    unsafe {
        (*rt1016).bclk = (*rt1016)
            .lrck
            .wrapping_mul((32u32).wrapping_shl(bclk_ms as u32));
    }

    if bclk_ms != 0 && unsafe { (*rt1016).master != 0 } {
        unsafe {
            snd_soc_component_update_bits(
                component,
                RT1016_I2S_CTRL,
                RT1016_I2S_BCLK_MS_MASK,
                RT1016_I2S_BCLK_MS_64,
            );
        }
    }

    unsafe {
        dev_dbg(
            (*component).dev,
            b"lrck is %dHz and pre_div is %d for iis %d\n\0".as_ptr() as *const c_char,
            (*rt1016).lrck,
            pre_div,
            (*dai).id,
        )
    };

    match unsafe { params_width(params) } {
        16 => val_len = RT1016_I2S_DL_16,
        20 => val_len = RT1016_I2S_DL_20,
        24 => val_len = RT1016_I2S_DL_24,
        32 => val_len = RT1016_I2S_DL_32,
        _ => return -EINVAL,
    }

    unsafe {
        snd_soc_component_update_bits(component, RT1016_I2S_CTRL, RT1016_I2S_DL_MASK, val_len);
        snd_soc_component_update_bits(
            component,
            RT1016_CLOCK_2,
            RT1016_FS_PD_MASK | RT1016_OSR_PD_MASK,
            (((pre_div + 3) as c_uint) << RT1016_FS_PD_SFT)
                | ((pre_div as c_uint) << RT1016_OSR_PD_SFT),
        );
    }

    0
}

unsafe extern "C" fn rt1016_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = unsafe { (*dai).component };
    let rt1016 = unsafe { snd_soc_component_get_drvdata(component) as *mut rt1016_priv };
    let mut reg_val: c_uint = 0;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => {
            reg_val |= RT1016_I2S_MS_M;
            unsafe {
                (*rt1016).master = 1;
            }
        }
        SND_SOC_DAIFMT_CBC_CFC => {
            reg_val |= RT1016_I2S_MS_S;
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_NF => {
            reg_val |= RT1016_I2S_BCLK_POL_INV;
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {}
        SND_SOC_DAIFMT_LEFT_J => {
            reg_val |= RT1016_I2S_DF_LEFT;
        }
        SND_SOC_DAIFMT_DSP_A => {
            reg_val |= RT1016_I2S_DF_PCM_A;
        }
        SND_SOC_DAIFMT_DSP_B => {
            reg_val |= RT1016_I2S_DF_PCM_B;
        }
        _ => return -EINVAL,
    }

    unsafe {
        snd_soc_component_update_bits(
            component,
            RT1016_I2S_CTRL,
            RT1016_I2S_MS_MASK | RT1016_I2S_BCLK_POL_MASK | RT1016_I2S_DF_MASK,
            reg_val,
        );
    }

    0
}

unsafe extern "C" fn rt1016_set_component_sysclk(
    component: *mut snd_soc_component,
    clk_id: c_int,
    _source: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let rt1016 = unsafe { snd_soc_component_get_drvdata(component) as *mut rt1016_priv };
    let mut reg_val: c_uint = 0;

    if freq == unsafe { (*rt1016).sysclk } && clk_id == unsafe { (*rt1016).sysclk_src } {
        return 0;
    }

    match clk_id {
        RT1016_SCLK_S_MCLK => {
            reg_val |= RT1016_CLK_SYS_SEL_MCLK;
        }
        RT1016_SCLK_S_PLL => {
            reg_val |= RT1016_CLK_SYS_SEL_PLL;
        }
        _ => {
            unsafe {
                dev_err(
                    (*component).dev,
                    b"Invalid clock id (%d)\n\0".as_ptr() as *const c_char,
                    clk_id,
                )
            };
            return -EINVAL;
        }
    }

    unsafe {
        (*rt1016).sysclk = freq;
        (*rt1016).sysclk_src = clk_id;
        dev_dbg(
            (*component).dev,
            b"Sysclk is %dHz and clock id is %d\n\0".as_ptr() as *const c_char,
            freq,
            clk_id,
        );
        snd_soc_component_update_bits(
            component,
            RT1016_CLOCK_1,
            RT1016_CLK_SYS_SEL_MASK,
            reg_val,
        );
    }

    0
}

unsafe extern "C" fn rt1016_set_component_pll(
    component: *mut snd_soc_component,
    _pll_id: c_int,
    source: c_int,
    freq_in: c_uint,
    freq_out: c_uint,
) -> c_int {
    let rt1016 = unsafe { snd_soc_component_get_drvdata(component) as *mut rt1016_priv };
    let mut pll_code = rl6231_pll_code {
        m_bp: 0,
        m_code: 0,
        n_code: 0,
        k_bp: 0,
        k_code: 0,
    };

    if freq_in == 0 || freq_out == 0 {
        unsafe {
            dev_dbg((*component).dev, b"PLL disabled\n\0".as_ptr() as *const c_char);
            (*rt1016).pll_in = 0;
            (*rt1016).pll_out = 0;
        }
        return 0;
    }

    if source == unsafe { (*rt1016).pll_src }
        && freq_in == unsafe { (*rt1016).pll_in }
        && freq_out == unsafe { (*rt1016).pll_out }
    {
        return 0;
    }

    match source {
        RT1016_PLL_S_MCLK => unsafe {
            snd_soc_component_update_bits(
                component,
                RT1016_CLOCK_1,
                RT1016_PLL_SEL_MASK,
                RT1016_PLL_SEL_MCLK,
            );
        },
        RT1016_PLL_S_BCLK => unsafe {
            snd_soc_component_update_bits(
                component,
                RT1016_CLOCK_1,
                RT1016_PLL_SEL_MASK,
                RT1016_PLL_SEL_BCLK,
            );
        },
        _ => {
            unsafe {
                dev_err(
                    (*component).dev,
                    b"Unknown PLL Source %d\n\0".as_ptr() as *const c_char,
                    source,
                )
            };
            return -EINVAL;
        }
    }

    let ret = unsafe { rl6231_pll_calc(freq_in, freq_out.wrapping_mul(4), &mut pll_code) };
    if ret < 0 {
        unsafe {
            dev_err(
                (*component).dev,
                b"Unsupported input clock %d\n\0".as_ptr() as *const c_char,
                freq_in,
            )
        };
        return ret;
    }

    unsafe {
        dev_dbg(
            (*component).dev,
            b"mbypass=%d m=%d n=%d kbypass=%d k=%d\n\0".as_ptr() as *const c_char,
            pll_code.m_bp,
            if pll_code.m_bp != 0 { 0 } else { pll_code.m_code },
            pll_code.n_code,
            pll_code.k_bp,
            if pll_code.k_bp != 0 { 0 } else { pll_code.k_code },
        );
        snd_soc_component_write(
            component,
            RT1016_PLL1,
            (((if pll_code.m_bp != 0 { 0 } else { pll_code.m_code }) as c_uint)
                << RT1016_PLL_M_SFT)
                | ((pll_code.m_bp as c_uint) << RT1016_PLL_M_BP_SFT)
                | (pll_code.n_code as c_uint),
        );
        snd_soc_component_write(
            component,
            RT1016_PLL2,
            ((pll_code.k_bp as c_uint) << RT1016_PLL_K_BP_SFT)
                | (if pll_code.k_bp != 0 { 0 } else { pll_code.k_code } as c_uint),
        );

        (*rt1016).pll_in = freq_in;
        (*rt1016).pll_out = freq_out;
        (*rt1016).pll_src = source;
    }

    0
}

unsafe extern "C" fn rt1016_probe(component: *mut snd_soc_component) -> c_int {
    let rt1016 = unsafe { snd_soc_component_get_drvdata(component) as *mut rt1016_priv };
    unsafe {
        (*rt1016).component = component;
    }
    0
}

unsafe extern "C" fn rt1016_remove(component: *mut snd_soc_component) {
    let rt1016 = unsafe { snd_soc_component_get_drvdata(component) as *mut rt1016_priv };
    unsafe {
        regmap_write((*rt1016).regmap, RT1016_RESET, 0);
    }
}

const RT1016_STEREO_RATES: c_uint = SNDRV_PCM_RATE_8000_48000;
const RT1016_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S8;

static rt1016_aif_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(rt1016_hw_params),
    set_fmt: Some(rt1016_set_dai_fmt),
};

static mut rt1016_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: b"rt1016-aif\0".as_ptr() as *const c_char,
    id: 0,
    playback: snd_soc_pcm_stream {
        stream_name: b"AIF Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: RT1016_STEREO_RATES,
        formats: RT1016_FORMATS,
    },
    ops: &rt1016_aif_dai_ops,
}];

// CONFIG_PM guarded in C.
#[cfg(CONFIG_PM)]
unsafe extern "C" fn rt1016_suspend(component: *mut snd_soc_component) -> c_int {
    let rt1016 = unsafe { snd_soc_component_get_drvdata(component) as *mut rt1016_priv };
    unsafe {
        regcache_cache_only((*rt1016).regmap, true);
        regcache_mark_dirty((*rt1016).regmap);
    }
    0
}

#[cfg(CONFIG_PM)]
unsafe extern "C" fn rt1016_resume(component: *mut snd_soc_component) -> c_int {
    let rt1016 = unsafe { snd_soc_component_get_drvdata(component) as *mut rt1016_priv };
    unsafe {
        regcache_cache_only((*rt1016).regmap, false);
        regcache_sync((*rt1016).regmap);
    }
    0
}

#[cfg(CONFIG_PM)]
const rt1016_suspend_ptr: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int> =
    Some(rt1016_suspend);
#[cfg(not(CONFIG_PM))]
const rt1016_suspend_ptr: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int> = None;
#[cfg(CONFIG_PM)]
const rt1016_resume_ptr: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int> =
    Some(rt1016_resume);
#[cfg(not(CONFIG_PM))]
const rt1016_resume_ptr: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int> = None;

static soc_component_dev_rt1016: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(rt1016_probe),
    remove: Some(rt1016_remove),
    suspend: rt1016_suspend_ptr,
    resume: rt1016_resume_ptr,
    controls: rt1016_snd_controls.as_ptr(),
    num_controls: rt1016_snd_controls.len() as c_uint,
    dapm_widgets: rt1016_dapm_widgets.as_ptr() as *const c_void,
    num_dapm_widgets: rt1016_dapm_widgets.len() as c_uint,
    dapm_routes: rt1016_dapm_routes.as_ptr(),
    num_dapm_routes: rt1016_dapm_routes.len() as c_uint,
    set_sysclk: Some(rt1016_set_component_sysclk),
    set_pll: Some(rt1016_set_component_pll),
    use_pmdown_time: 1,
    endianness: 1,
};

static rt1016_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 16,
    max_register: RT1016_PWR_CTRL,
    volatile_reg: Some(rt1016_volatile_register),
    readable_reg: Some(rt1016_readable_register),
    cache_type: REGCACHE_RBTREE,
    reg_defaults: rt1016_reg.as_ptr(),
    num_reg_defaults: rt1016_reg.len() as c_uint,
};

static rt1016_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: *b"rt1016\0\0\0\0\0\0\0\0\0\0\0\0\0\0" as [c_char; 20] },
    i2c_device_id { name: [0; 20] },
];
// MODULE_DEVICE_TABLE(i2c, rt1016_i2c_id);

// CONFIG_OF guarded in C.
#[cfg(CONFIG_OF)]
static rt1016_of_match: [of_device_id; 2] = [
    of_device_id { compatible: b"realtek,rt1016\0".as_ptr() as *const c_char },
    of_device_id { compatible: ptr::null() },
];
// MODULE_DEVICE_TABLE(of, rt1016_of_match);

// CONFIG_ACPI guarded in C.
#[cfg(CONFIG_ACPI)]
static rt1016_acpi_match: [acpi_device_id; 2] = [
    acpi_device_id { id: *b"10EC1016\0" as [c_char; 9] },
    acpi_device_id { id: [0; 9] },
];
// MODULE_DEVICE_TABLE(acpi, rt1016_acpi_match);

unsafe extern "C" fn rt1016_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let mut ret: c_int;
    let mut val: c_uint = 0;

    let rt1016 = unsafe {
        devm_kzalloc(
            &mut (*i2c).dev,
            core::mem::size_of::<rt1016_priv>(),
            GFP_KERNEL,
        ) as *mut rt1016_priv
    };
    if rt1016.is_null() {
        return -ENOMEM;
    }

    unsafe {
        i2c_set_clientdata(i2c, rt1016 as *mut c_void);

        (*rt1016).regmap = devm_regmap_init_i2c(i2c, &rt1016_regmap);
        if IS_ERR((*rt1016).regmap as *const c_void) {
            ret = PTR_ERR((*rt1016).regmap as *const c_void);
            dev_err(
                &mut (*i2c).dev,
                b"Failed to allocate register map: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }

        regmap_read((*rt1016).regmap, RT1016_DEVICE_ID, &mut val);
        if val != RT1016_DEVICE_ID_VAL {
            dev_err(
                &mut (*i2c).dev,
                b"Device with ID register %x is not rt1016\n\0".as_ptr() as *const c_char,
                val,
            );
            return -ENODEV;
        }

        regmap_write((*rt1016).regmap, RT1016_RESET, 0);

        ret = regmap_register_patch(
            (*rt1016).regmap,
            rt1016_patch.as_ptr(),
            rt1016_patch.len() as c_uint,
        );
        if ret != 0 {
            dev_warn(
                &mut (*i2c).dev,
                b"Failed to apply regmap patch: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
        }

        devm_snd_soc_register_component(
            &mut (*i2c).dev,
            &soc_component_dev_rt1016,
            rt1016_dai.as_mut_ptr(),
            rt1016_dai.len() as c_int,
        )
    }
}

unsafe extern "C" fn rt1016_i2c_shutdown(client: *mut i2c_client) {
    let rt1016 = unsafe { i2c_get_clientdata(client) as *mut rt1016_priv };
    unsafe {
        regmap_write((*rt1016).regmap, RT1016_RESET, 0);
    }
}

static mut rt1016_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"rt1016\0".as_ptr() as *const c_char,
        #[cfg(CONFIG_OF)]
        of_match_table: rt1016_of_match.as_ptr(),
        #[cfg(not(CONFIG_OF))]
        of_match_table: ptr::null(),
        #[cfg(CONFIG_ACPI)]
        acpi_match_table: rt1016_acpi_match.as_ptr(),
        #[cfg(not(CONFIG_ACPI))]
        acpi_match_table: ptr::null(),
    },
    probe: Some(rt1016_i2c_probe),
    shutdown: Some(rt1016_i2c_shutdown),
    id_table: rt1016_i2c_id.as_ptr(),
};
// module_i2c_driver(rt1016_i2c_driver);

// MODULE_DESCRIPTION("ASoC RT1016 driver");
// MODULE_AUTHOR("Oder Chiou <oder_chiou@realtek.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
