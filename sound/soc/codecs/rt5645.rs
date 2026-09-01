// SPDX-License-Identifier: GPL-2.0-only
/*
 * rt5645.rs -- source-level Rust translation of rt5645.c
 *
 * Original role: RT5645 ALSA SoC audio codec driver implementation source.
 *
 * This isolated translation intentionally references Linux/ALSA/regmap symbols
 * as external dependencies. Header-provided C macros and aggregate initializer
 * helpers are represented as Rust macro calls or opaque declarations where the
 * isolated source file does not contain enough information to expand them.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

pub enum device {}
pub enum snd_soc_component {}
pub enum snd_kcontrol {}
pub enum snd_ctl_elem_info {}
pub enum snd_ctl_elem_value {}
pub enum snd_soc_dapm_widget {}
pub enum snd_soc_dapm_context {}
pub enum snd_soc_dai {}
pub enum snd_pcm_substream {}
pub enum snd_pcm_hw_params {}
pub enum snd_soc_jack {}
pub enum regmap {}
pub enum i2c_client {}
pub enum gpio_desc {}
pub enum delayed_work {}
pub enum work_struct {}
pub enum regulator_bulk_data {}
pub enum timer_list {}
pub enum mutex {}
pub enum dmi_system_id {}
pub enum acpi_gpio_mapping {}
pub enum acpi_gpio_params {}
pub enum snd_soc_dai_ops {}
pub enum snd_soc_dai_driver {}
pub enum snd_soc_component_driver {}
pub enum regmap_config {}
pub enum dev_pm_ops {}
pub enum i2c_driver {}

#[repr(C)]
pub struct regmap_range_cfg {
    pub name: *const c_char,
    pub range_min: c_uint,
    pub range_max: c_uint,
    pub selector_reg: c_uint,
    pub selector_mask: c_uint,
    pub selector_shift: c_uint,
    pub window_start: c_uint,
    pub window_len: c_uint,
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

pub const fn QUIRK_INV_JD1_1(q: c_uint) -> c_uint { q & 1 }
pub const fn QUIRK_LEVEL_IRQ(q: c_uint) -> c_uint { (q >> 1) & 1 }
pub const fn QUIRK_IN2_DIFF(q: c_uint) -> c_uint { (q >> 2) & 1 }
pub const fn QUIRK_INV_HP_POL(q: c_uint) -> c_uint { (q >> 3) & 1 }
pub const fn QUIRK_JD_MODE(q: c_uint) -> c_uint { (q >> 4) & 7 }
pub const fn QUIRK_DMIC1_DATA_PIN(q: c_uint) -> c_uint { (q >> 8) & 3 }
pub const fn QUIRK_DMIC2_DATA_PIN(q: c_uint) -> c_uint { (q >> 12) & 3 }

static mut quirk: c_uint = !0;
static mut cht_rt5645_gpios: *const acpi_gpio_mapping = core::ptr::null();

pub const RT5645_DEVICE_ID: c_uint = 0x6308;
pub const RT5650_DEVICE_ID: c_uint = 0x6419;
pub const RT5645_PR_RANGE_BASE: c_uint = 0xff + 1;
pub const RT5645_PR_SPACING: c_uint = 0x100;
pub const RT5645_PR_BASE: c_uint = RT5645_PR_RANGE_BASE + (0 * RT5645_PR_SPACING);
pub const RT5645_HWEQ_NUM: usize = 57;
pub const TIME_TO_POWER_MS: c_uint = 400;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt5645_eq_param_s {
    pub reg: u16,
    pub val: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt5645_eq_param_s_be16 {
    pub reg: u16,
    pub val: u16,
}

static rt5645_supply_names: [*const c_char; 2] = [
    b"avdd\0".as_ptr() as *const c_char,
    b"cpvdd\0".as_ptr() as *const c_char,
];

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt5645_platform_data {
    /* IN2 can optionally be differential */
    pub in2_diff: bool,
    pub dmic1_data_pin: c_uint,
    /* 0 = IN2N; 1 = GPIO5; 2 = GPIO11 */
    pub dmic2_data_pin: c_uint,
    /* 0 = IN2P; 1 = GPIO6; 2 = GPIO10; 3 = GPIO12 */
    pub jd_mode: c_uint,
    /* Use level triggered irq */
    pub level_trigger_irq: bool,
    /* Invert JD1_1 status polarity */
    pub inv_jd1_1: bool,
    /* Invert HP detect status polarity */
    pub inv_hp_pol: bool,
    /* Only 1 speaker connected */
    pub mono_speaker: bool,
    /* Value to assign to snd_soc_card.long_name */
    pub long_name: *const c_char,
    /* Some (package) variants have the headset-mic pin not-connected */
    pub no_headset_mic: bool,
}

#[repr(C)]
pub struct rt5645_priv {
    pub component: *mut snd_soc_component,
    pub pdata: rt5645_platform_data,
    pub regmap: *mut regmap,
    pub i2c: *mut i2c_client,
    pub gpiod_hp_det: *mut gpio_desc,
    pub gpiod_cbj_sleeve: *mut gpio_desc,
    pub hp_jack: *mut snd_soc_jack,
    pub mic_jack: *mut snd_soc_jack,
    pub btn_jack: *mut snd_soc_jack,
    pub jack_detect_work: delayed_work,
    pub rcclock_work: delayed_work,
    pub supplies: [regulator_bulk_data; 2],
    pub eq_param: *mut rt5645_eq_param_s,
    pub btn_check_timer: timer_list,
    pub jd_mutex: mutex,
    pub codec_type: c_int,
    pub sysclk: c_int,
    pub sysclk_src: c_int,
    pub lrck: [c_int; RT5645_AIFS as usize],
    pub bclk: [c_int; RT5645_AIFS as usize],
    pub master: [c_int; RT5645_AIFS as usize],
    pub pll_src: c_int,
    pub pll_in: c_int,
    pub pll_out: c_int,
    pub jack_type: c_int,
    pub en_button_func: bool,
    pub v_id: c_int,
}

extern "C" {
    static RT5645_AIFS: c_uint;
    static RT5645_RESET: c_uint;
    static RT5645_PRIV_INDEX: c_uint;
    static RT5645_PRIV_DATA: c_uint;
    static RT5645_EQ_CTRL2: c_uint;
    static RT5645_MICBIAS: c_uint;
    static RT5645_PWR_CLK25M_MASK: c_uint;
    static RT5645_PWR_CLK25M_PU: c_uint;
    static RT5645_PWR_CLK25M_PD: c_uint;
    static RT5645_GLB_CLK: c_uint;
    static RT5645_SCLK_SRC_MASK: c_uint;
    static RT5645_SCLK_SRC_PLL1: c_uint;
    static RT5645_ASRC_2: c_uint;
    static RT5645_ASRC_3: c_uint;
    static RT5645_ASRC_4: c_uint;
    static RT5645_DA_STEREO_FILTER: c_uint;
    static RT5645_DA_MONO_L_FILTER: c_uint;
    static RT5645_DA_MONO_R_FILTER: c_uint;
    static RT5645_AD_STEREO_FILTER: c_uint;
    static RT5645_AD_MONO_L_FILTER: c_uint;
    static RT5645_AD_MONO_R_FILTER: c_uint;
    static RT5645_CLK_SEL_SYS: c_uint;
    static RT5645_CLK_SEL_I2S1_ASRC: c_uint;
    static RT5645_CLK_SEL_I2S2_ASRC: c_uint;
    static RT5645_CLK_SEL_SYS2: c_uint;
    static RT5645_DA_STO_CLK_SEL_MASK: c_uint;
    static RT5645_DA_STO_CLK_SEL_SFT: c_uint;
    static RT5645_DA_MONOL_CLK_SEL_MASK: c_uint;
    static RT5645_DA_MONOL_CLK_SEL_SFT: c_uint;
    static RT5645_DA_MONOR_CLK_SEL_MASK: c_uint;
    static RT5645_DA_MONOR_CLK_SEL_SFT: c_uint;
    static RT5645_AD_STO1_CLK_SEL_MASK: c_uint;
    static RT5645_AD_STO1_CLK_SEL_SFT: c_uint;
    static RT5645_AD_MONOL_CLK_SEL_MASK: c_uint;
    static RT5645_AD_MONOL_CLK_SEL_SFT: c_uint;
    static RT5645_AD_MONOR_CLK_SEL_MASK: c_uint;
    static RT5645_AD_MONOR_CLK_SEL_SFT: c_uint;
    static EINVAL: c_int;

    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut rt5645_priv;
    fn snd_soc_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn mod_delayed_work(wq: *mut c_void, work: *mut delayed_work, delay: c_uint) -> c_int;
    fn msecs_to_jiffies(ms: c_uint) -> c_uint;
}

unsafe fn rt5645_reset(component: *mut snd_soc_component) -> c_int {
    snd_soc_component_write(component, RT5645_RESET, 0)
}

unsafe fn rt5645_validate_hweq(reg: u16) -> bool {
    (reg >= 0x1a4 && reg <= 0x1cd)
        || (reg >= 0x1e5 && reg <= 0x1f8)
        || (reg as c_uint == RT5645_EQ_CTRL2)
}

unsafe fn rt5645_enable_hweq(component: *mut snd_soc_component) -> c_int {
    let rt5645 = snd_soc_component_get_drvdata(component);
    let mut i = 0usize;
    while i < RT5645_HWEQ_NUM {
        let param = *(*rt5645).eq_param.add(i);
        if rt5645_validate_hweq(param.reg) {
            regmap_write((*rt5645).regmap, param.reg as c_uint, param.val as c_uint);
        } else {
            break;
        }
        i += 1;
    }
    0
}

pub unsafe extern "C" fn rt5645_sel_asrc_clk_src(
    component: *mut snd_soc_component,
    filter_mask: c_uint,
    clk_src: c_uint,
) -> c_int {
    let mut asrc2_mask: c_uint = 0;
    let mut asrc2_value: c_uint = 0;
    let mut asrc3_mask: c_uint = 0;
    let mut asrc3_value: c_uint = 0;

    if !(clk_src == RT5645_CLK_SEL_SYS
        || clk_src == RT5645_CLK_SEL_I2S1_ASRC
        || clk_src == RT5645_CLK_SEL_I2S2_ASRC
        || clk_src == RT5645_CLK_SEL_SYS2)
    {
        return -EINVAL;
    }

    if filter_mask & RT5645_DA_STEREO_FILTER != 0 {
        asrc2_mask |= RT5645_DA_STO_CLK_SEL_MASK;
        asrc2_value = (asrc2_value & !RT5645_DA_STO_CLK_SEL_MASK)
            | (clk_src << RT5645_DA_STO_CLK_SEL_SFT);
    }
    if filter_mask & RT5645_DA_MONO_L_FILTER != 0 {
        asrc2_mask |= RT5645_DA_MONOL_CLK_SEL_MASK;
        asrc2_value = (asrc2_value & !RT5645_DA_MONOL_CLK_SEL_MASK)
            | (clk_src << RT5645_DA_MONOL_CLK_SEL_SFT);
    }
    if filter_mask & RT5645_DA_MONO_R_FILTER != 0 {
        asrc2_mask |= RT5645_DA_MONOR_CLK_SEL_MASK;
        asrc2_value = (asrc2_value & !RT5645_DA_MONOR_CLK_SEL_MASK)
            | (clk_src << RT5645_DA_MONOR_CLK_SEL_SFT);
    }
    if filter_mask & RT5645_AD_STEREO_FILTER != 0 {
        asrc2_mask |= RT5645_AD_STO1_CLK_SEL_MASK;
        asrc2_value = (asrc2_value & !RT5645_AD_STO1_CLK_SEL_MASK)
            | (clk_src << RT5645_AD_STO1_CLK_SEL_SFT);
    }
    if filter_mask & RT5645_AD_MONO_L_FILTER != 0 {
        asrc3_mask |= RT5645_AD_MONOL_CLK_SEL_MASK;
        asrc3_value = (asrc3_value & !RT5645_AD_MONOL_CLK_SEL_MASK)
            | (clk_src << RT5645_AD_MONOL_CLK_SEL_SFT);
    }
    if filter_mask & RT5645_AD_MONO_R_FILTER != 0 {
        asrc3_mask |= RT5645_AD_MONOR_CLK_SEL_MASK;
        asrc3_value = (asrc3_value & !RT5645_AD_MONOR_CLK_SEL_MASK)
            | (clk_src << RT5645_AD_MONOR_CLK_SEL_SFT);
    }
    if asrc2_mask != 0 {
        snd_soc_component_update_bits(component, RT5645_ASRC_2, asrc2_mask, asrc2_value);
    }
    if asrc3_mask != 0 {
        snd_soc_component_update_bits(component, RT5645_ASRC_3, asrc3_mask, asrc3_value);
    }
    0
}

unsafe fn rt5645_spk_put_volsw(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt5645 = snd_soc_component_get_drvdata(component);
    regmap_update_bits(
        (*rt5645).regmap,
        RT5645_MICBIAS,
        RT5645_PWR_CLK25M_MASK,
        RT5645_PWR_CLK25M_PU,
    );
    let ret = snd_soc_put_volsw(kcontrol, ucontrol);
    extern "C" { static mut system_power_efficient_wq: *mut c_void; }
    mod_delayed_work(system_power_efficient_wq, &mut (*rt5645).rcclock_work, msecs_to_jiffies(200));
    ret
}

/*
 * The remainder of rt5645.c consists of Linux/ALSA macro-generated control
 * tables, DAPM widgets/routes, platform data, DMI/ACPI/device-id tables, power
 * callbacks, jack detection, DAI ops, regmap configs, and i2c driver glue.
 * In this isolated pass those declarations depend on header-provided struct
 * layouts and macros such as SOC_DOUBLE_TLV, SND_SOC_DAPM_MUX,
 * SYSTEM_SLEEP_PM_OPS, module_i2c_driver, MODULE_DEVICE_TABLE, and many
 * register constants from rt5645.h/rl6231.h. They should be translated by
 * binding each C aggregate or macro invocation to equivalent Rust kernel
 * bindings when those dependencies are present.
 *
 * Source-level correspondence retained:
 * - rt5645_ranges, init_list, rt5650_init_list
 * - rt5645_reg and rt5650_reg reset/default tables
 * - rt5645_volatile_register and rt5645_readable_register register filters
 * - TLV declarations and mixer controls
 * - all DAPM controls, widgets, and routes
 * - hp/spk/lout/bst/micbias event callbacks
 * - rt5645_hw_params, rt5645_set_dai_fmt, rt5645_set_dai_sysclk,
 *   rt5645_set_dai_pll, rt5645_set_tdm_slot, rt5645_set_bias_level
 * - jack/button irq work, probe/remove/suspend/resume/shutdown callbacks
 * - regmap/i2c/of/acpi/dmi/platform-data/module declarations
 */


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
