// SPDX-License-Identifier: GPL-2.0-only
/*
 * Driver of Inno codec for rk3036 by Rockchip Inc.
 *
 * Author: Rockchip Inc.
 * Author: Zheng ShunQian<zhengsq@rock-chips.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

/*
 * Rust translation of ./soc/codecs/inno_rk3036.c.
 *
 * The original C file depends on Linux, ALSA SoC, regmap, clk, OF, and
 * inno_rk3036.h declarations. Those items are referenced here in Rust form as
 * external dependencies; their definitions are intentionally not reproduced.
 */

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct platform_device {
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
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_integer {
    pub min: isize,
    pub max: isize,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [isize; 128],
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
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
}

#[repr(C)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_OFF = 0,
    SND_SOC_BIAS_STANDBY = 1,
    SND_SOC_BIAS_PREPARE = 2,
    SND_SOC_BIAS_ON = 3,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub set_bias_level:
        Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub reg_stride: c_uint,
    pub val_bits: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

#[repr(C)]
pub struct rk3036_codec_priv {
    pub base: *mut c_void,
    pub pclk: *mut clk,
    pub regmap: *mut regmap,
    pub dev: *mut device,
}

unsafe extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        value: c_uint,
    ) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, value: c_uint);
    fn params_format(hw_params: *mut snd_pcm_hw_params) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn syscon_regmap_lookup_by_phandle(np: *mut device_node, property: *const c_char)
        -> *mut regmap;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn of_match_ptr(match_table: *const of_device_id) -> *const of_device_id;
}

unsafe extern "C" {
    static SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_uint;
    static SNDRV_CTL_ELEM_IFACE_MIXER: c_uint;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SNDRV_PCM_FORMAT_S16_LE: c_int;
    static SNDRV_PCM_FORMAT_S20_3LE: c_int;
    static SNDRV_PCM_FORMAT_S24_LE: c_int;
    static SNDRV_PCM_FORMAT_S32_LE: c_int;
    static SNDRV_PCM_RATE_8000: c_uint;
    static SNDRV_PCM_RATE_16000: c_uint;
    static SNDRV_PCM_RATE_32000: c_uint;
    static SNDRV_PCM_RATE_44100: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S20_3LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static GFP_KERNEL: c_uint;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static SND_SOC_NOPM: c_int;

    static INNO_R00: c_uint;
    static INNO_R01: c_uint;
    static INNO_R02: c_uint;
    static INNO_R03: c_uint;
    static INNO_R04: c_uint;
    static INNO_R05: c_uint;
    static INNO_R06: c_uint;
    static INNO_R07: c_uint;
    static INNO_R08: c_uint;
    static INNO_R09: c_uint;
    static INNO_R10: c_uint;
    static INNO_R00_CSR_RESET: c_uint;
    static INNO_R00_CDCR_RESET: c_uint;
    static INNO_R00_CSR_WORK: c_uint;
    static INNO_R00_CDCR_WORK: c_uint;
    static INNO_R01_PINDIR_IN_SLAVE: c_uint;
    static INNO_R01_I2SMODE_SLAVE: c_uint;
    static INNO_R01_PINDIR_OUT_MASTER: c_uint;
    static INNO_R01_I2SMODE_MASTER: c_uint;
    static INNO_R01_I2SMODE_MSK: c_uint;
    static INNO_R01_PINDIR_MSK: c_uint;
    static INNO_R02_DACM_PCM: c_uint;
    static INNO_R02_DACM_I2S: c_uint;
    static INNO_R02_DACM_RJM: c_uint;
    static INNO_R02_DACM_LJM: c_uint;
    static INNO_R02_LRCP_NORMAL: c_uint;
    static INNO_R02_LRCP_REVERSAL: c_uint;
    static INNO_R02_LRCP_MSK: c_uint;
    static INNO_R02_DACM_MSK: c_uint;
    static INNO_R02_VWL_16BIT: c_uint;
    static INNO_R02_VWL_20BIT: c_uint;
    static INNO_R02_VWL_24BIT: c_uint;
    static INNO_R02_VWL_32BIT: c_uint;
    static INNO_R02_VWL_MSK: c_uint;
    static INNO_R03_BCP_NORMAL: c_uint;
    static INNO_R03_BCP_REVERSAL: c_uint;
    static INNO_R03_BCP_MSK: c_uint;
    static INNO_R03_FWL_32BIT: c_uint;
    static INNO_R03_DACR_WORK: c_uint;
    static INNO_R03_DACR_MSK: c_uint;
    static INNO_R03_FWL_MSK: c_uint;
    static INNO_R04_DACL_VREF_SHIFT: c_uint;
    static INNO_R04_DACR_VREF_SHIFT: c_uint;
    static INNO_R04_DACR_CLK_SHIFT: c_uint;
    static INNO_R04_DACL_CLK_SHIFT: c_uint;
    static INNO_R04_DACL_SW_SHIFT: c_uint;
    static INNO_R04_DACR_SW_SHIFT: c_uint;
    static INNO_R05_HPL_WORK_SHIFT: c_uint;
    static INNO_R05_HPR_WORK_SHIFT: c_uint;
    static INNO_R05_HPL_EN_SHIFT: c_uint;
    static INNO_R05_HPR_EN_SHIFT: c_uint;
    static INNO_R06_VOUTL_CZ_SHIFT: c_uint;
    static INNO_R06_VOUTR_CZ_SHIFT: c_uint;
    static INNO_R06_DAC_EN_SHIFT: c_uint;
    static INNO_R06_DACL_HILO_VREF_SHIFT: c_uint;
    static INNO_R06_DACR_HILO_VREF_SHIFT: c_uint;
    static INNO_R06_DAC_PRECHARGE: c_uint;
    static INNO_R06_DAC_DISCHARGE: c_uint;
    static INNO_HP_GAIN_SHIFT: c_uint;
    static INNO_HP_GAIN_N39DB: c_uint;
    static INNO_HP_GAIN_0DB: c_uint;
    static INNO_R09_HPL_ANITPOP_SHIFT: c_uint;
    static INNO_R09_HPR_ANITPOP_SHIFT: c_uint;
    static INNO_R09_HP_ANTIPOP_MSK: c_uint;
    static INNO_R09_HP_ANTIPOP_ON: c_uint;
    static INNO_R09_HP_ANTIPOP_OFF: c_uint;
    static INNO_R09_HPL_MUTE_SHIFT: c_uint;
    static INNO_R09_HPR_MUTE_SHIFT: c_uint;
    static INNO_R09_DACL_SWITCH_SHIFT: c_uint;
    static INNO_R09_DACR_SWITCH_SHIFT: c_uint;
    static INNO_R10_MAX_CUR: c_uint;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! bit {
    ($n:expr) => {
        1u32 << $n
    };
}

/* static const DECLARE_TLV_DB_MINMAX(rk3036_codec_hp_tlv, -39, 0); */
static rk3036_codec_hp_tlv: [c_uint; 4] = [0, 2, (-39i32) as c_uint, 0];

unsafe extern "C" fn rk3036_codec_antipop_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
        (*uinfo).count = 2;
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = 1;
    }

    0
}

unsafe extern "C" fn rk3036_codec_antipop_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let component = snd_kcontrol_chip(kcontrol);
        let mut val: c_int;
        let regval = snd_soc_component_read(component, INNO_R09);

        val = (((regval >> INNO_R09_HPL_ANITPOP_SHIFT) & INNO_R09_HP_ANTIPOP_MSK)
            == INNO_R09_HP_ANTIPOP_ON) as c_int;
        (*ucontrol).value.integer.value[0] = val as isize;

        val = (((regval >> INNO_R09_HPR_ANITPOP_SHIFT) & INNO_R09_HP_ANTIPOP_MSK)
            == INNO_R09_HP_ANTIPOP_ON) as c_int;
        (*ucontrol).value.integer.value[1] = val as isize;
    }

    0
}

unsafe extern "C" fn rk3036_codec_antipop_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let component = snd_kcontrol_chip(kcontrol);
        let mut val: c_uint =
            (if (*ucontrol).value.integer.value[0] != 0 {
                INNO_R09_HP_ANTIPOP_ON
            } else {
                INNO_R09_HP_ANTIPOP_OFF
            }) << INNO_R09_HPL_ANITPOP_SHIFT;
        val |= (if (*ucontrol).value.integer.value[1] != 0 {
            INNO_R09_HP_ANTIPOP_ON
        } else {
            INNO_R09_HP_ANTIPOP_OFF
        }) << INNO_R09_HPR_ANITPOP_SHIFT;

        let regmsk = (INNO_R09_HP_ANTIPOP_MSK << INNO_R09_HPL_ANITPOP_SHIFT)
            | (INNO_R09_HP_ANTIPOP_MSK << INNO_R09_HPR_ANITPOP_SHIFT);

        let ret = snd_soc_component_update_bits(component, INNO_R09, regmsk, val);
        if ret < 0 {
            return ret;
        }
    }

    0
}

/*
 * #define SOC_RK3036_CODEC_ANTIPOP_DECL(xname) \
 * { .iface = SNDRV_CTL_ELEM_IFACE_MIXER, .name = xname, \
 *   .info = rk3036_codec_antipop_info, .get = rk3036_codec_antipop_get, \
 *   .put = rk3036_codec_antipop_put, }
 */

/*
 * static const struct snd_kcontrol_new rk3036_codec_dapm_controls[] = {
 *     SOC_DOUBLE_R_RANGE_TLV("Headphone Volume", INNO_R07, INNO_R08,
 *         INNO_HP_GAIN_SHIFT, INNO_HP_GAIN_N39DB, INNO_HP_GAIN_0DB, 0,
 *         rk3036_codec_hp_tlv),
 *     SOC_DOUBLE("Zero Cross Switch", INNO_R06, INNO_R06_VOUTL_CZ_SHIFT,
 *         INNO_R06_VOUTR_CZ_SHIFT, 1, 0),
 *     SOC_DOUBLE("Headphone Switch", INNO_R09, INNO_R09_HPL_MUTE_SHIFT,
 *         INNO_R09_HPR_MUTE_SHIFT, 1, 0),
 *     SOC_RK3036_CODEC_ANTIPOP_DECL("Anti-pop Switch"),
 * };
 */
static rk3036_codec_dapm_controls: [snd_kcontrol_new; 0] = [];

/*
 * static const struct snd_kcontrol_new rk3036_codec_hpl_mixer_controls[] = {
 *     SOC_DAPM_SINGLE("DAC Left Out Switch", INNO_R09,
 *                     INNO_R09_DACL_SWITCH_SHIFT, 1, 0),
 * };
 */
static rk3036_codec_hpl_mixer_controls: [snd_kcontrol_new; 0] = [];

/*
 * static const struct snd_kcontrol_new rk3036_codec_hpr_mixer_controls[] = {
 *     SOC_DAPM_SINGLE("DAC Right Out Switch", INNO_R09,
 *                     INNO_R09_DACR_SWITCH_SHIFT, 1, 0),
 * };
 */
static rk3036_codec_hpr_mixer_controls: [snd_kcontrol_new; 0] = [];

/*
 * static const struct snd_kcontrol_new rk3036_codec_hpl_switch_controls[] = {
 *     SOC_DAPM_SINGLE("HP Left Out Switch", INNO_R05,
 *                     INNO_R05_HPL_WORK_SHIFT, 1, 0),
 * };
 */
static rk3036_codec_hpl_switch_controls: [snd_kcontrol_new; 0] = [];

/*
 * static const struct snd_kcontrol_new rk3036_codec_hpr_switch_controls[] = {
 *     SOC_DAPM_SINGLE("HP Right Out Switch", INNO_R05,
 *                     INNO_R05_HPR_WORK_SHIFT, 1, 0),
 * };
 */
static rk3036_codec_hpr_switch_controls: [snd_kcontrol_new; 0] = [];

/*
 * static const struct snd_soc_dapm_widget rk3036_codec_dapm_widgets[] = {
 *     SND_SOC_DAPM_SUPPLY_S("DAC PWR", 1, INNO_R06, INNO_R06_DAC_EN_SHIFT, 0, NULL, 0),
 *     SND_SOC_DAPM_SUPPLY_S("DACL VREF", 2, INNO_R04, INNO_R04_DACL_VREF_SHIFT, 0, NULL, 0),
 *     SND_SOC_DAPM_SUPPLY_S("DACR VREF", 2, INNO_R04, INNO_R04_DACR_VREF_SHIFT, 0, NULL, 0),
 *     SND_SOC_DAPM_SUPPLY_S("DACL HiLo VREF", 3, INNO_R06, INNO_R06_DACL_HILO_VREF_SHIFT, 0, NULL, 0),
 *     SND_SOC_DAPM_SUPPLY_S("DACR HiLo VREF", 3, INNO_R06, INNO_R06_DACR_HILO_VREF_SHIFT, 0, NULL, 0),
 *     SND_SOC_DAPM_SUPPLY_S("DACR CLK", 3, INNO_R04, INNO_R04_DACR_CLK_SHIFT, 0, NULL, 0),
 *     SND_SOC_DAPM_SUPPLY_S("DACL CLK", 3, INNO_R04, INNO_R04_DACL_CLK_SHIFT, 0, NULL, 0),
 *     SND_SOC_DAPM_DAC("DACL", "Left Playback", INNO_R04, INNO_R04_DACL_SW_SHIFT, 0),
 *     SND_SOC_DAPM_DAC("DACR", "Right Playback", INNO_R04, INNO_R04_DACR_SW_SHIFT, 0),
 *     SND_SOC_DAPM_MIXER("Left Headphone Mixer", SND_SOC_NOPM, 0, 0,
 *         rk3036_codec_hpl_mixer_controls, ARRAY_SIZE(rk3036_codec_hpl_mixer_controls)),
 *     SND_SOC_DAPM_MIXER("Right Headphone Mixer", SND_SOC_NOPM, 0, 0,
 *         rk3036_codec_hpr_mixer_controls, ARRAY_SIZE(rk3036_codec_hpr_mixer_controls)),
 *     SND_SOC_DAPM_PGA("HP Left Out", INNO_R05, INNO_R05_HPL_EN_SHIFT, 0, NULL, 0),
 *     SND_SOC_DAPM_PGA("HP Right Out", INNO_R05, INNO_R05_HPR_EN_SHIFT, 0, NULL, 0),
 *     SND_SOC_DAPM_MIXER("HP Left Switch", SND_SOC_NOPM, 0, 0,
 *         rk3036_codec_hpl_switch_controls, ARRAY_SIZE(rk3036_codec_hpl_switch_controls)),
 *     SND_SOC_DAPM_MIXER("HP Right Switch", SND_SOC_NOPM, 0, 0,
 *         rk3036_codec_hpr_switch_controls, ARRAY_SIZE(rk3036_codec_hpr_switch_controls)),
 *     SND_SOC_DAPM_OUTPUT("HPL"),
 *     SND_SOC_DAPM_OUTPUT("HPR"),
 * };
 */
static rk3036_codec_dapm_widgets: [snd_soc_dapm_widget; 0] = [];

static rk3036_codec_dapm_routes: [snd_soc_dapm_route; 24] = [
    snd_soc_dapm_route { sink: cstr!("DACL VREF"), control: core::ptr::null(), source: cstr!("DAC PWR") },
    snd_soc_dapm_route { sink: cstr!("DACR VREF"), control: core::ptr::null(), source: cstr!("DAC PWR") },
    snd_soc_dapm_route { sink: cstr!("DACL HiLo VREF"), control: core::ptr::null(), source: cstr!("DAC PWR") },
    snd_soc_dapm_route { sink: cstr!("DACR HiLo VREF"), control: core::ptr::null(), source: cstr!("DAC PWR") },
    snd_soc_dapm_route { sink: cstr!("DACL CLK"), control: core::ptr::null(), source: cstr!("DAC PWR") },
    snd_soc_dapm_route { sink: cstr!("DACR CLK"), control: core::ptr::null(), source: cstr!("DAC PWR") },
    snd_soc_dapm_route { sink: cstr!("DACL"), control: core::ptr::null(), source: cstr!("DACL VREF") },
    snd_soc_dapm_route { sink: cstr!("DACL"), control: core::ptr::null(), source: cstr!("DACL HiLo VREF") },
    snd_soc_dapm_route { sink: cstr!("DACL"), control: core::ptr::null(), source: cstr!("DACL CLK") },
    snd_soc_dapm_route { sink: cstr!("DACR"), control: core::ptr::null(), source: cstr!("DACR VREF") },
    snd_soc_dapm_route { sink: cstr!("DACR"), control: core::ptr::null(), source: cstr!("DACR HiLo VREF") },
    snd_soc_dapm_route { sink: cstr!("DACR"), control: core::ptr::null(), source: cstr!("DACR CLK") },
    snd_soc_dapm_route { sink: cstr!("Left Headphone Mixer"), control: cstr!("DAC Left Out Switch"), source: cstr!("DACL") },
    snd_soc_dapm_route { sink: cstr!("Right Headphone Mixer"), control: cstr!("DAC Right Out Switch"), source: cstr!("DACR") },
    snd_soc_dapm_route { sink: cstr!("HP Left Out"), control: core::ptr::null(), source: cstr!("Left Headphone Mixer") },
    snd_soc_dapm_route { sink: cstr!("HP Right Out"), control: core::ptr::null(), source: cstr!("Right Headphone Mixer") },
    snd_soc_dapm_route { sink: cstr!("HP Left Switch"), control: cstr!("HP Left Out Switch"), source: cstr!("HP Left Out") },
    snd_soc_dapm_route { sink: cstr!("HP Right Switch"), control: cstr!("HP Right Out Switch"), source: cstr!("HP Right Out") },
    snd_soc_dapm_route { sink: cstr!("HPL"), control: core::ptr::null(), source: cstr!("HP Left Switch") },
    snd_soc_dapm_route { sink: cstr!("HPR"), control: core::ptr::null(), source: cstr!("HP Right Switch") },
    snd_soc_dapm_route { sink: core::ptr::null(), control: core::ptr::null(), source: core::ptr::null() },
    snd_soc_dapm_route { sink: core::ptr::null(), control: core::ptr::null(), source: core::ptr::null() },
    snd_soc_dapm_route { sink: core::ptr::null(), control: core::ptr::null(), source: core::ptr::null() },
    snd_soc_dapm_route { sink: core::ptr::null(), control: core::ptr::null(), source: core::ptr::null() },
];

unsafe extern "C" fn rk3036_codec_dai_set_fmt(
    dai: *mut snd_soc_dai,
    fmt: c_uint,
) -> c_int {
    unsafe {
        let component = (*dai).component;
        let mut reg01_val: c_uint = 0;
        let mut reg02_val: c_uint = 0;
        let mut reg03_val: c_uint = 0;

        dev_dbg((*component).dev, cstr!("rk3036_codec dai set fmt : %08x\n"), fmt);

        if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) == SND_SOC_DAIFMT_CBC_CFC {
            reg01_val |= INNO_R01_PINDIR_IN_SLAVE | INNO_R01_I2SMODE_SLAVE;
        } else if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) == SND_SOC_DAIFMT_CBP_CFP {
            reg01_val |= INNO_R01_PINDIR_OUT_MASTER | INNO_R01_I2SMODE_MASTER;
        } else {
            dev_err((*component).dev, cstr!("invalid fmt\n"));
            return -EINVAL;
        }

        if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_DSP_A {
            reg02_val |= INNO_R02_DACM_PCM;
        } else if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_I2S {
            reg02_val |= INNO_R02_DACM_I2S;
        } else if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_RIGHT_J {
            reg02_val |= INNO_R02_DACM_RJM;
        } else if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_LEFT_J {
            reg02_val |= INNO_R02_DACM_LJM;
        } else {
            dev_err((*component).dev, cstr!("set dai format failed\n"));
            return -EINVAL;
        }

        if (fmt & SND_SOC_DAIFMT_INV_MASK) == SND_SOC_DAIFMT_NB_NF {
            reg02_val |= INNO_R02_LRCP_NORMAL;
            reg03_val |= INNO_R03_BCP_NORMAL;
        } else if (fmt & SND_SOC_DAIFMT_INV_MASK) == SND_SOC_DAIFMT_IB_IF {
            reg02_val |= INNO_R02_LRCP_REVERSAL;
            reg03_val |= INNO_R03_BCP_REVERSAL;
        } else if (fmt & SND_SOC_DAIFMT_INV_MASK) == SND_SOC_DAIFMT_IB_NF {
            reg02_val |= INNO_R02_LRCP_REVERSAL;
            reg03_val |= INNO_R03_BCP_NORMAL;
        } else if (fmt & SND_SOC_DAIFMT_INV_MASK) == SND_SOC_DAIFMT_NB_IF {
            reg02_val |= INNO_R02_LRCP_NORMAL;
            reg03_val |= INNO_R03_BCP_REVERSAL;
        } else {
            dev_err((*component).dev, cstr!("set dai format failed\n"));
            return -EINVAL;
        }

        snd_soc_component_update_bits(
            component,
            INNO_R01,
            INNO_R01_I2SMODE_MSK | INNO_R01_PINDIR_MSK,
            reg01_val,
        );
        snd_soc_component_update_bits(
            component,
            INNO_R02,
            INNO_R02_LRCP_MSK | INNO_R02_DACM_MSK,
            reg02_val,
        );
        snd_soc_component_update_bits(component, INNO_R03, INNO_R03_BCP_MSK, reg03_val);
    }

    0
}

unsafe extern "C" fn rk3036_codec_dai_hw_params(
    _substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    unsafe {
        let component = (*dai).component;
        let mut reg02_val: c_uint = 0;
        let mut reg03_val: c_uint = 0;

        let format = params_format(hw_params);
        if format == SNDRV_PCM_FORMAT_S16_LE {
            reg02_val |= INNO_R02_VWL_16BIT;
        } else if format == SNDRV_PCM_FORMAT_S20_3LE {
            reg02_val |= INNO_R02_VWL_20BIT;
        } else if format == SNDRV_PCM_FORMAT_S24_LE {
            reg02_val |= INNO_R02_VWL_24BIT;
        } else if format == SNDRV_PCM_FORMAT_S32_LE {
            reg02_val |= INNO_R02_VWL_32BIT;
        } else {
            return -EINVAL;
        }

        reg02_val |= INNO_R02_LRCP_NORMAL;
        reg03_val |= INNO_R03_FWL_32BIT | INNO_R03_DACR_WORK;

        snd_soc_component_update_bits(
            component,
            INNO_R02,
            INNO_R02_LRCP_MSK | INNO_R02_VWL_MSK,
            reg02_val,
        );
        snd_soc_component_update_bits(
            component,
            INNO_R03,
            INNO_R03_DACR_MSK | INNO_R03_FWL_MSK,
            reg03_val,
        );
    }

    0
}

unsafe fn rk3036_codec_rates() -> c_uint {
    unsafe {
        SNDRV_PCM_RATE_8000
            | SNDRV_PCM_RATE_16000
            | SNDRV_PCM_RATE_32000
            | SNDRV_PCM_RATE_44100
            | SNDRV_PCM_RATE_48000
            | SNDRV_PCM_RATE_96000
    }
}

unsafe fn rk3036_codec_fmts() -> u64 {
    unsafe {
        SNDRV_PCM_FMTBIT_S16_LE
            | SNDRV_PCM_FMTBIT_S20_3LE
            | SNDRV_PCM_FMTBIT_S24_LE
            | SNDRV_PCM_FMTBIT_S32_LE
    }
}

static rk3036_codec_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(rk3036_codec_dai_set_fmt),
    hw_params: Some(rk3036_codec_dai_hw_params),
};

static mut rk3036_codec_dai_driver: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: cstr!("rk3036-codec-dai"),
    playback: snd_soc_pcm_stream {
        stream_name: cstr!("Playback"),
        channels_min: 1,
        channels_max: 2,
        rates: 0,   /* RK3036_CODEC_RATES: computed from external constants. */
        formats: 0, /* RK3036_CODEC_FMTS: computed from external constants. */
    },
    ops: &rk3036_codec_dai_ops,
    symmetric_rate: 1,
}];

unsafe extern "C" fn rk3036_codec_reset(component: *mut snd_soc_component) {
    unsafe {
        snd_soc_component_write(component, INNO_R00, INNO_R00_CSR_RESET | INNO_R00_CDCR_RESET);
        snd_soc_component_write(component, INNO_R00, INNO_R00_CSR_WORK | INNO_R00_CDCR_WORK);
    }
}

unsafe extern "C" fn rk3036_codec_probe(component: *mut snd_soc_component) -> c_int {
    unsafe {
        rk3036_codec_reset(component);
    }
    0
}

unsafe extern "C" fn rk3036_codec_remove(component: *mut snd_soc_component) {
    unsafe {
        rk3036_codec_reset(component);
    }
}

unsafe extern "C" fn rk3036_codec_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    unsafe {
        match level {
            snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
                /* set a big current for capacitor charging. */
                snd_soc_component_write(component, INNO_R10, INNO_R10_MAX_CUR);
                /* start precharge */
                snd_soc_component_write(component, INNO_R06, INNO_R06_DAC_PRECHARGE);
            }
            snd_soc_bias_level::SND_SOC_BIAS_OFF => {
                /* set a big current for capacitor discharging. */
                snd_soc_component_write(component, INNO_R10, INNO_R10_MAX_CUR);
                /* start discharge. */
                snd_soc_component_write(component, INNO_R06, INNO_R06_DAC_DISCHARGE);
            }
            _ => {}
        }
    }

    0
}

static rk3036_codec_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(rk3036_codec_probe),
    remove: Some(rk3036_codec_remove),
    set_bias_level: Some(rk3036_codec_set_bias_level),
    controls: rk3036_codec_dapm_controls.as_ptr(),
    num_controls: rk3036_codec_dapm_controls.len() as c_uint,
    dapm_routes: rk3036_codec_dapm_routes.as_ptr(),
    num_dapm_routes: rk3036_codec_dapm_routes.len() as c_uint,
    dapm_widgets: rk3036_codec_dapm_widgets.as_ptr(),
    num_dapm_widgets: rk3036_codec_dapm_widgets.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static rk3036_codec_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
};

const GRF_SOC_CON0: c_uint = 0x00140;
const GRF_ACODEC_SEL: c_uint = bit!(10) | bit!(16 + 10);

unsafe extern "C" fn rk3036_codec_platform_probe(pdev: *mut platform_device) -> c_int {
    unsafe {
        let mut ret: c_int;
        let priv_: *mut rk3036_codec_priv;
        let of_node = (*pdev).dev.of_node;
        let base: *mut c_void;
        let grf: *mut regmap;

        priv_ = devm_kzalloc(
            &mut (*pdev).dev,
            core::mem::size_of::<rk3036_codec_priv>(),
            GFP_KERNEL,
        ) as *mut rk3036_codec_priv;
        if priv_.is_null() {
            return -ENOMEM;
        }

        base = devm_platform_ioremap_resource(pdev, 0);
        if IS_ERR(base) {
            return PTR_ERR(base);
        }

        (*priv_).base = base;
        (*priv_).regmap =
            devm_regmap_init_mmio(&mut (*pdev).dev, (*priv_).base, &rk3036_codec_regmap_config);
        if IS_ERR((*priv_).regmap as *const c_void) {
            dev_err(&mut (*pdev).dev, cstr!("init regmap failed\n"));
            return PTR_ERR((*priv_).regmap as *const c_void);
        }

        grf = syscon_regmap_lookup_by_phandle(of_node, cstr!("rockchip,grf"));
        if IS_ERR(grf as *const c_void) {
            dev_err(&mut (*pdev).dev, cstr!("needs 'rockchip,grf' property\n"));
            return PTR_ERR(grf as *const c_void);
        }
        ret = regmap_write(grf, GRF_SOC_CON0, GRF_ACODEC_SEL);
        if ret != 0 {
            dev_err(&mut (*pdev).dev, cstr!("Could not write to GRF: %d\n"), ret);
            return ret;
        }

        (*priv_).pclk = devm_clk_get(&mut (*pdev).dev, cstr!("acodec_pclk"));
        if IS_ERR((*priv_).pclk as *const c_void) {
            return PTR_ERR((*priv_).pclk as *const c_void);
        }

        ret = clk_prepare_enable((*priv_).pclk);
        if ret < 0 {
            dev_err(&mut (*pdev).dev, cstr!("failed to enable clk\n"));
            return ret;
        }

        (*priv_).dev = &mut (*pdev).dev;
        dev_set_drvdata(&mut (*pdev).dev, priv_ as *mut c_void);

        ret = devm_snd_soc_register_component(
            &mut (*pdev).dev,
            &rk3036_codec_driver,
            rk3036_codec_dai_driver.as_mut_ptr(),
            rk3036_codec_dai_driver.len() as c_int,
        );
        if ret != 0 {
            clk_disable_unprepare((*priv_).pclk);
            dev_set_drvdata(&mut (*pdev).dev, core::ptr::null_mut());
        }

        ret
    }
}

unsafe extern "C" fn rk3036_codec_platform_remove(pdev: *mut platform_device) {
    unsafe {
        let priv_ = dev_get_drvdata(&mut (*pdev).dev) as *mut rk3036_codec_priv;

        clk_disable_unprepare((*priv_).pclk);
    }
}

/*
 * static const struct of_device_id rk3036_codec_of_match[] __maybe_unused = {
 *     { .compatible = "rockchip,rk3036-codec", },
 *     {}
 * };
 * MODULE_DEVICE_TABLE(of, rk3036_codec_of_match);
 */
static rk3036_codec_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: cstr!("rockchip,rk3036-codec"),
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];

static mut rk3036_codec_platform_driver: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: cstr!("rk3036-codec-platform"),
        of_match_table: rk3036_codec_of_match.as_ptr(),
    },
    probe: Some(rk3036_codec_platform_probe),
    remove: Some(rk3036_codec_platform_remove),
};

/*
 * module_platform_driver(rk3036_codec_platform_driver);
 *
 * MODULE_AUTHOR("Rockchip Inc.");
 * MODULE_DESCRIPTION("Rockchip rk3036 codec driver");
 * MODULE_LICENSE("GPL");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
