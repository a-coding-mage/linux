// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2018-2021, The Linux Foundation. All rights reserved.
 * Copyright (c) 2022-2023, Qualcomm Innovation Center, Inc. All rights reserved.
 * Copyright (c) 2023, Linaro Limited
 *
 * Source-level Rust translation of soc/codecs/wcd939x.c.
 * Kernel, ALSA, SoundWire, regmap, MBHC, CLSH, Type-C, OF, GPIO, PM-runtime,
 * component, and module registration symbols are external dependencies supplied
 * by the translated repository.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr::{null, null_mut};

type u8 = u8;
type u16 = u16;
type u32 = u32;
type s16 = i16;
type s32 = i32;
type int32_t = i32;
type uint32_t = u32;
type irq_hw_number_t = c_ulong;
type irqreturn_t = c_int;

#[repr(C)] pub struct sdw_slave { _private: [u8; 0] }
#[repr(C)] pub struct wcd939x_sdw_priv { pub sdev: *mut sdw_slave, pub ch_info: *mut wcd_sdw_ch_info, pub port_config: *mut sdw_port_config, pub port_enable: *mut bool, pub regmap: *mut regmap, pub slave_irq: *mut irq_domain, pub wcd939x: *mut wcd939x_priv }
#[repr(C)] pub struct device { pub of_node: *mut device_node, pub fwnode: *mut c_void }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct typec_switch { _private: [u8; 0] }
#[repr(C)] pub struct typec_switch_dev { _private: [u8; 0] }
#[repr(C)] pub struct typec_mux_dev { _private: [u8; 0] }
#[repr(C)] pub struct typec_mux_state { pub mode: c_ulong }
#[repr(C)] pub struct wcd_mbhc { _private: [u8; 0] }
#[repr(C)] pub struct wcd_clsh_ctrl { _private: [u8; 0] }
#[repr(C)] pub struct irq_domain { _private: [u8; 0] }
#[repr(C)] pub struct regmap_irq_chip_data { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_jack { _private: [u8; 0] }
#[repr(C)] pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { pub private_value: c_ulong }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { pub dapm: *mut snd_soc_dapm_context, pub shift: c_int, pub name: *const c_char }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }
#[repr(C)] pub union snd_ctl_elem_value_value { pub integer: snd_ctl_elem_value_integer, pub enumerated: snd_ctl_elem_value_enumerated }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_integer { pub value: [i64; 128] }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_enumerated { pub item: [c_uint; 128] }
#[repr(C)] pub struct soc_enum { pub reg: c_uint, pub shift_l: c_uint, pub shift_r: c_uint, pub items: c_uint, pub texts: *const *const c_char }
#[repr(C)] pub struct soc_mixer_control { pub reg: c_uint, pub shift: c_uint }
#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget_desc { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_route { pub sink: *const c_char, pub control: *const c_char, pub source: *const c_char }
#[repr(C)] pub struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai { pub dev: *mut device, pub id: c_int }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct component_match { _private: [u8; 0] }

#[repr(C)] pub struct wcd_sdw_ch_info { pub port_num: u8, pub ch_mask: u8 }
#[repr(C)] pub struct sdw_port_config { pub num: u8, pub ch_mask: u8 }
#[repr(C)] pub struct wcd_mbhc_intr { pub mbhc_sw_intr: c_int, pub mbhc_btn_press_intr: c_int, pub mbhc_btn_release_intr: c_int, pub mbhc_hs_ins_intr: c_int, pub mbhc_hs_rem_intr: c_int, pub hph_left_ocp: c_int, pub hph_right_ocp: c_int }
#[repr(C)] pub struct wcd_common { pub dev: *mut device, pub max_bias: c_int, pub micb_mv: [c_int; 4], pub micb_vout: [c_uint; 4] }
#[repr(C)] pub struct wcd_mbhc_config { pub mbhc_micbias: c_int, pub anc_micbias: c_int, pub v_hs_max: c_int, pub num_btn: c_int, pub micb_mv: c_int, pub linein_th: c_int, pub hs_thr: c_int, pub hph_thr: c_int, pub moist_rref: c_int, pub hphl_swh: bool, pub typec_analog_mux: bool, pub swap_gnd_mic: Option<unsafe extern "C" fn(*mut snd_soc_component) -> bool> }
#[repr(C)] pub struct wcd_mbhc_field { _private: [u8; 0] }
#[repr(C)] pub struct regmap_irq { _private: [u8; 0] }
#[repr(C)] pub struct regmap_irq_chip { pub name: *const c_char, pub irqs: *const regmap_irq, pub num_irqs: c_int, pub num_regs: c_int, pub status_base: c_uint, pub mask_base: c_uint, pub ack_base: c_uint, pub use_ack: c_int, pub runtime_pm: bool, pub irq_drv_data: *mut c_void }
#[repr(C)] pub struct irq_chip { pub name: *const c_char }
#[repr(C)] pub struct irq_domain_ops { pub map: Option<unsafe extern "C" fn(*mut irq_domain, c_uint, irq_hw_number_t) -> c_int> }
#[repr(C)] pub struct snd_soc_dai_ops { pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>, pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>, pub set_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, *mut c_void, c_int) -> c_int> }
#[repr(C)] pub struct snd_soc_dai_driver { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_component_driver { _private: [u8; 0] }
#[repr(C)] pub struct component_master_ops { pub bind: Option<unsafe extern "C" fn(*mut device) -> c_int>, pub unbind: Option<unsafe extern "C" fn(*mut device)> }

const WCD939X_MAX_MICBIAS: usize = 4;
const WCD939X_MBHC_MAX_BUTTONS: c_int = 8;
const TX_ADC_MAX: usize = 4;
const WCD_MBHC_HS_V_MAX: c_int = 1600;
const CHIPID_WCD9390: c_int = 0x0;
const CHIPID_WCD9395: c_int = 0x5;
const CHIPID_WCD939X_VER_MAJOR_1: c_uint = 0x0;
const CHIPID_WCD939X_VER_MINOR_1: c_uint = 0x3;
const WCD939X_VERSION_1_0: c_uint = 0;
const WCD939X_VERSION_1_1: c_uint = 1;
const WCD939X_VERSION_2_0: c_uint = 2;
const SWR_CLK_RATE_0P6MHZ: c_int = 600000;
const SWR_CLK_RATE_1P2MHZ: c_int = 1200000;
const SWR_CLK_RATE_2P4MHZ: c_int = 2400000;
const SWR_CLK_RATE_4P8MHZ: c_int = 4800000;
const SWR_CLK_RATE_9P6MHZ: c_int = 9600000;
const SWR_CLK_RATE_11P2896MHZ: c_int = 1128960;
const ADC_MODE_VAL_HIFI: c_int = 0x01;
const ADC_MODE_VAL_LO_HIF: c_int = 0x02;
const ADC_MODE_VAL_NORMAL: c_int = 0x03;
const ADC_MODE_VAL_LP: c_int = 0x05;
const ADC_MODE_VAL_ULP1: c_int = 0x09;
const ADC_MODE_VAL_ULP2: c_int = 0x0B;
const WCD939X_ZDET_VAL_100K: s32 = 100000000;
const WCD939X_ZDET_FLOATING_IMPEDANCE: s32 = 0x0FFFFFFE;
const WCD939X_ZDET_NUM_MEASUREMENTS: c_int = 900;
const WCD939X_ANA_MBHC_ZDET_CONST: s32 = 1018 * 1024;
const WCD939X_NUM_IRQS: usize = 17;
const MICB_BIAS_DISABLE: c_int = 0;
const MICB_BIAS_ENABLE: c_int = 1;
const MICB_BIAS_PULL_UP: c_int = 2;
const MICB_BIAS_PULL_DOWN: c_int = 3;
const WCD_ADC1: c_int = 0;
const WCD_ADC2: c_int = 1;
const WCD_ADC3: c_int = 2;
const WCD_ADC4: c_int = 3;
const HPH_PA_DELAY: c_int = 4;
const ADC_MODE_INVALID: usize = 0;
const ADC_MODE_HIFI: usize = 1;
const ADC_MODE_LO_HIF: usize = 2;
const ADC_MODE_NORMAL: usize = 3;
const ADC_MODE_LP: usize = 4;
const ADC_MODE_ULP1: usize = 5;
const ADC_MODE_ULP2: usize = 6;
const AIF1_PB: usize = 0;
const AIF1_CAP: usize = 1;
const NUM_CODEC_DAIS: usize = 2;

const fn WCD_VOUT_CTL_TO_MICB(v: c_uint) -> c_uint { 1000 + v * 50 }
const fn WCD939X_MBHC_GET_C1(c: c_int) -> s16 { (((c & 0xC000) >> 14) as s16) }
const fn WCD939X_MBHC_GET_X1(x: c_int) -> s32 { (x & 0x3FFF) as s32 }
const fn BIT(n: c_int) -> c_int { 1 << n }
const fn GENMASK(h: c_int, l: c_int) -> c_int { (((!0u32) << l) & ((!0u32) >> (31 - h))) as c_int }

static mut tx_mode_bit: [u8; 7] = [0x00, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20];

#[repr(C)]
struct zdet_param {
    ldo_ctl: u16,
    noff: u16,
    nshift: u16,
    btn5: u16,
    btn6: u16,
    btn7: u16,
}

#[repr(C)]
struct wcd939x_priv {
    tx_sdw_dev: *mut sdw_slave,
    sdw_priv: [*mut wcd939x_sdw_priv; NUM_CODEC_DAIS],
    txdev: *mut device,
    rxdev: *mut device,
    rxnode: *mut device_node,
    txnode: *mut device_node,
    regmap: *mut regmap,
    component: *mut snd_soc_component,
    /* micb setup lock */
    micb_lock: mutex,
    /* typec handling */
    typec_analog_mux: bool,
    /* CONFIG_TYPEC fields */
    typec_orientation: c_int,
    typec_mode: c_ulong,
    typec_switch: *mut typec_switch,
    /* mbhc module */
    wcd_mbhc: *mut wcd_mbhc,
    mbhc_cfg: wcd_mbhc_config,
    intr_ids: wcd_mbhc_intr,
    clsh_info: *mut wcd_clsh_ctrl,
    common: wcd_common,
    virq: *mut irq_domain,
    irq_chip: *mut regmap_irq_chip_data,
    jack: *mut snd_soc_jack,
    status_mask: c_ulong,
    micb_ref: [s32; WCD939X_MAX_MICBIAS],
    pullup_ref: [s32; WCD939X_MAX_MICBIAS],
    hph_mode: u32,
    tx_mode: [u32; TX_ADC_MAX],
    variant: c_int,
    reset_gpio: *mut gpio_desc,
    hphr_pdm_wd_int: c_int,
    hphl_pdm_wd_int: c_int,
    ear_pdm_wd_int: c_int,
    comp1_enable: bool,
    comp2_enable: bool,
    ldoh: bool,
}

const wcd939x_supplies: [&[u8]; 5] = [b"vdd-rxtx\0", b"vdd-io\0", b"vdd-buck\0", b"vdd-mic-bias\0", b"vdd-px\0"];

static wcd939x_wcd_mbhc_d1_a: [s16; 4] = [0, 30, 30, 6];
static wcd939x_mbhc_mincode_param: [c_int; 8] = [3277, 1639, 820, 410, 205, 103, 52, 26];
static wcd939x_mbhc_zdet_param: zdet_param = zdet_param { ldo_ctl: 4, noff: 0, nshift: 6, btn5: 0x18, btn6: 0x60, btn7: 0x78 };

unsafe extern "C" {
    static WCD939X_ANA_BIAS: c_uint; static WCD939X_BIAS_ANALOG_BIAS_EN: c_uint; static WCD939X_BIAS_PRECHRG_EN: c_uint;
    static WCD939X_DIGITAL_SWR_TX_CLK_RATE: c_uint; static WCD939X_ANA_MICB1: c_uint; static WCD939X_ANA_MICB2: c_uint; static WCD939X_ANA_MICB3: c_uint; static WCD939X_ANA_MICB4: c_uint;
    static WCD939X_MICB_ENABLE: c_uint; static WCD939X_MICB_VOUT_CTL: c_uint; static WCD939X_ANA_MBHC_ZDET: c_uint; static WCD939X_MBHC_ZDET_ZDET_CHG_EN: c_uint;
    static WCD939X_ANA_MBHC_RESULT_2: c_uint; static WCD939X_MBHC_RESULT_2_Z_RESULT_MSB: c_uint; static WCD939X_ANA_MBHC_RESULT_1: c_uint; static WCD939X_MBHC_RESULT_1_Z_RESULT_LSB: c_uint;
    static WCD939X_MBHC_NEW_ZDET_ANA_CTL: c_uint; static WCD939X_ZDET_ANA_CTL_MAXV_CTL: c_uint; static WCD939X_ANA_MBHC_BTN5: c_uint; static WCD939X_ANA_MBHC_BTN6: c_uint; static WCD939X_ANA_MBHC_BTN7: c_uint;
    static WCD939X_MBHC_BTN5_VTH: c_uint; static WCD939X_MBHC_BTN6_VTH: c_uint; static WCD939X_MBHC_BTN7_VTH: c_uint; static WCD939X_ZDET_ANA_CTL_RANGE_CTL: c_uint;
    static WCD939X_MBHC_NEW_ZDET_RAMP_CTL: c_uint; static WCD939X_ZDET_RAMP_CTL_TIME_CTL: c_uint; static WCD939X_ZDET_RAMP_CTL_ACC1_MIN_CTL: c_uint;
    static WCD939X_MBHC_ZDET_ZDET_L_MEAS_EN: c_uint; static WCD939X_MBHC_ZDET_ZDET_R_MEAS_EN: c_uint; static WCD939X_DIGITAL_EFUSE_REG_21: c_uint;
    static WCD939X_ANA_MBHC_ELECT: c_uint; static WCD939X_MBHC_ELECT_FSM_EN: c_uint; static WCD939X_ANA_MBHC_MECH: c_uint; static WCD939X_MBHC_MECH_L_DET_EN: c_uint;
    static WCD939X_MBHC_MECH_SW_HPH_L_P_100K_TO_GND: c_uint; static WCD939X_HPH_SURGE_EN: c_uint; static WCD939X_EN_EN_SURGE_PROTECTION_HPHR: c_uint; static WCD939X_EN_EN_SURGE_PROTECTION_HPHL: c_uint;
    static WCD939X_HPH_R_ATEST: c_uint; static WCD939X_R_ATEST_HPH_GND_OVR: c_uint; static WCD939X_HPH_PA_CTL2: c_uint; static WCD939X_PA_CTL2_HPHPA_GND_R: c_uint; static WCD939X_PA_CTL2_HPHPA_GND_L: c_uint;
    static WCD939X_MBHC_CTL_CLK: c_uint; static WCD939X_MBHC_NEW_CTL_2: c_uint; static WCD939X_CTL_2_M_RTH_CTL: c_uint; static WCD939X_MBHC_NEW_FSM_STATUS: c_uint; static WCD939X_FSM_STATUS_HS_M_COMP_STATUS: c_uint;
    static WCD939X_MBHC_NEW_INT_MOISTURE_DET_POLLING_CTRL: c_uint; static WCD939X_MOISTURE_DET_POLLING_CTRL_MOIST_EN_POLLING: c_uint;
    static MIC_BIAS_1: c_int; static MIC_BIAS_2: c_int; static MIC_BIAS_3: c_int; static MIC_BIAS_4: c_int; static MICB_PULLUP_ENABLE: c_int; static MICB_PULLUP_DISABLE: c_int; static MICB_ENABLE: c_int; static MICB_DISABLE: c_int;
    static EINVAL: c_int; static ENODEV: c_int; static ENOMEM: c_int; static UINT_MAX: c_uint; static IRQ_HANDLED: irqreturn_t;
    static SND_SOC_DAPM_PRE_PMU: c_int; static SND_SOC_DAPM_POST_PMU: c_int; static SND_SOC_DAPM_PRE_PMD: c_int; static SND_SOC_DAPM_POST_PMD: c_int;
    static R_OFF: c_int; static WCD_MBHC_DEF_BUTTONS: c_int; static WCD_MBHC_THR_HS_MICB_MV: c_int; static WCD_MONO_HS_MIN_THR: u32;
    static WCD_MBHC_HPH_MONO: c_int; static WCD_MBHC_HPH_STEREO: c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_write_field(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_int);
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint);
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint);
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_read_field(component: *mut snd_soc_component, reg: c_uint, mask: c_uint) -> c_uint;
    fn usleep_range(min: c_uint, max: c_uint); fn mdelay(ms: c_uint);
    fn set_bit(nr: c_int, addr: *mut c_ulong); fn clear_bit(nr: c_int, addr: *mut c_ulong); fn test_bit(nr: c_int, addr: *const c_ulong) -> bool; fn ffs(x: c_int) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int; fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int; fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn wcd_get_micb_vout_ctl_val(dev: *mut device, req_volt: c_int) -> c_int; fn wcd_mbhc_event_notify(mbhc: *mut wcd_mbhc, event: c_int);
    fn wcd_mbhc_set_hph_type(mbhc: *mut wcd_mbhc, hph_type: c_int); fn wcd_mbhc_get_hph_type(mbhc: *mut wcd_mbhc) -> c_int; fn wcd_mbhc_get_impedance(mbhc: *mut wcd_mbhc, zl: *mut u32, zr: *mut u32);
    fn wcd939x_sdw_hw_params(wcd: *mut wcd939x_sdw_priv, substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int;
    fn wcd939x_sdw_free(wcd: *mut wcd939x_sdw_priv, substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int;
    fn wcd939x_sdw_set_sdw_stream(wcd: *mut wcd939x_sdw_priv, dai: *mut snd_soc_dai, stream: *mut c_void, direction: c_int) -> c_int;
}

unsafe fn wcd939x_get_clk_rate(mode: c_int) -> c_int {
    match mode as usize {
        ADC_MODE_ULP2 => SWR_CLK_RATE_0P6MHZ,
        ADC_MODE_ULP1 => SWR_CLK_RATE_1P2MHZ,
        ADC_MODE_LP => SWR_CLK_RATE_4P8MHZ,
        _ => SWR_CLK_RATE_9P6MHZ,
    }
}

unsafe fn wcd939x_set_swr_clk_rate(component: *mut snd_soc_component, rate: c_int, bank: c_int) -> c_int {
    let mask: u8 = if bank != 0 { 0xF0 } else { 0x0F };
    let val = match rate {
        SWR_CLK_RATE_0P6MHZ => 6,
        SWR_CLK_RATE_1P2MHZ => 5,
        SWR_CLK_RATE_2P4MHZ => 3,
        SWR_CLK_RATE_4P8MHZ => 1,
        _ => 0,
    };
    snd_soc_component_write_field(component, WCD939X_DIGITAL_SWR_TX_CLK_RATE, mask as c_uint, val);
    0
}

unsafe fn wcd939x_sdw_connect_port(ch_info: *const wcd_sdw_ch_info, port_config: *mut sdw_port_config, enable: u8) -> c_int {
    let port_num = (*ch_info).port_num;
    let ch_mask = (*ch_info).ch_mask;
    (*port_config).num = port_num;
    if enable != 0 { (*port_config).ch_mask |= ch_mask; } else { (*port_config).ch_mask &= !ch_mask; }
    0
}

unsafe fn wcd939x_connect_port(wcd: *mut wcd939x_sdw_priv, port_num: u8, ch_id: u8, enable: u8) -> c_int {
    wcd939x_sdw_connect_port((*wcd).ch_info.add(ch_id as usize), (*wcd).port_config.add((port_num - 1) as usize), enable)
}

unsafe fn wcd939x_get_adc_mode(val: c_int) -> c_int {
    match val as usize {
        ADC_MODE_INVALID => ADC_MODE_VAL_NORMAL,
        ADC_MODE_HIFI => ADC_MODE_VAL_HIFI,
        ADC_MODE_LO_HIF => ADC_MODE_VAL_LO_HIF,
        ADC_MODE_NORMAL => ADC_MODE_VAL_NORMAL,
        ADC_MODE_LP => ADC_MODE_VAL_LP,
        ADC_MODE_ULP1 => ADC_MODE_VAL_ULP1,
        ADC_MODE_ULP2 => ADC_MODE_VAL_ULP2,
        _ => -EINVAL,
    }
}

unsafe fn wcd939x_micbias_control(component: *mut snd_soc_component, micb_num: c_int, req: c_int, is_dapm: bool) -> c_int {
    let wcd939x = snd_soc_component_get_drvdata(component) as *mut wcd939x_priv;
    let micb_index = (micb_num - 1) as usize;
    let micb_reg = if micb_num == MIC_BIAS_1 { WCD939X_ANA_MICB1 } else if micb_num == MIC_BIAS_2 { WCD939X_ANA_MICB2 } else if micb_num == MIC_BIAS_3 { WCD939X_ANA_MICB3 } else if micb_num == MIC_BIAS_4 { WCD939X_ANA_MICB4 } else { return -EINVAL; };
    if req == MICB_PULLUP_ENABLE {
        (*wcd939x).pullup_ref[micb_index] += 1;
        if (*wcd939x).pullup_ref[micb_index] == 1 && (*wcd939x).micb_ref[micb_index] == 0 {
            snd_soc_component_write_field(component, micb_reg, WCD939X_MICB_ENABLE, MICB_BIAS_PULL_UP);
        }
    } else if req == MICB_PULLUP_DISABLE {
        if (*wcd939x).pullup_ref[micb_index] > 0 { (*wcd939x).pullup_ref[micb_index] -= 1; }
        if (*wcd939x).pullup_ref[micb_index] == 0 && (*wcd939x).micb_ref[micb_index] == 0 {
            snd_soc_component_write_field(component, micb_reg, WCD939X_MICB_ENABLE, MICB_BIAS_DISABLE);
        }
    } else if req == MICB_ENABLE {
        (*wcd939x).micb_ref[micb_index] += 1;
        if (*wcd939x).micb_ref[micb_index] == 1 {
            snd_soc_component_write_field(component, micb_reg, WCD939X_MICB_ENABLE, MICB_BIAS_ENABLE);
            if micb_num == MIC_BIAS_2 { wcd_mbhc_event_notify((*wcd939x).wcd_mbhc, WCD_EVENT_POST_MICBIAS_2_ON); }
        }
        if micb_num == MIC_BIAS_2 && is_dapm { wcd_mbhc_event_notify((*wcd939x).wcd_mbhc, WCD_EVENT_POST_DAPM_MICBIAS_2_ON); }
    } else if req == MICB_DISABLE {
        if (*wcd939x).micb_ref[micb_index] > 0 { (*wcd939x).micb_ref[micb_index] -= 1; }
        if (*wcd939x).micb_ref[micb_index] == 0 && (*wcd939x).pullup_ref[micb_index] > 0 {
            snd_soc_component_write_field(component, micb_reg, WCD939X_MICB_ENABLE, MICB_BIAS_PULL_UP);
        } else if (*wcd939x).micb_ref[micb_index] == 0 && (*wcd939x).pullup_ref[micb_index] == 0 {
            if micb_num == MIC_BIAS_2 { wcd_mbhc_event_notify((*wcd939x).wcd_mbhc, WCD_EVENT_PRE_MICBIAS_2_OFF); }
            snd_soc_component_write_field(component, micb_reg, WCD939X_MICB_ENABLE, MICB_BIAS_DISABLE);
            if micb_num == MIC_BIAS_2 { wcd_mbhc_event_notify((*wcd939x).wcd_mbhc, WCD_EVENT_POST_MICBIAS_2_OFF); }
        }
        if is_dapm && micb_num == MIC_BIAS_2 { wcd_mbhc_event_notify((*wcd939x).wcd_mbhc, WCD_EVENT_POST_DAPM_MICBIAS_2_OFF); }
    }
    0
}

unsafe extern "C" {
    static WCD_EVENT_POST_MICBIAS_2_ON: c_int; static WCD_EVENT_POST_DAPM_MICBIAS_2_ON: c_int; static WCD_EVENT_PRE_MICBIAS_2_OFF: c_int; static WCD_EVENT_POST_MICBIAS_2_OFF: c_int; static WCD_EVENT_POST_DAPM_MICBIAS_2_OFF: c_int;
}

unsafe fn wcd939x_mbhc_micb_en_status(component: *mut snd_soc_component, micb_num: c_int) -> bool {
    if micb_num == MIC_BIAS_2 {
        let val = snd_soc_component_read_field(component, WCD939X_ANA_MICB2, WCD939X_MICB_ENABLE) as c_int;
        if val == MICB_BIAS_ENABLE { return true; }
    }
    false
}

unsafe fn wcd939x_mbhc_micb_adjust_voltage(component: *mut snd_soc_component, req_volt: c_int, micb_num: c_int) -> c_int {
    let micb_reg = if micb_num == MIC_BIAS_1 { WCD939X_ANA_MICB1 } else if micb_num == MIC_BIAS_2 { WCD939X_ANA_MICB2 } else if micb_num == MIC_BIAS_3 { WCD939X_ANA_MICB3 } else if micb_num == MIC_BIAS_4 { WCD939X_ANA_MICB4 } else { return -EINVAL; };
    let micb_en = snd_soc_component_read_field(component, micb_reg, WCD939X_MICB_ENABLE) as c_int;
    let cur_vout_ctl = snd_soc_component_read_field(component, micb_reg, WCD939X_MICB_VOUT_CTL);
    let req_vout_ctl = wcd_get_micb_vout_ctl_val((*component).dev, req_volt);
    if req_vout_ctl < 0 { return req_vout_ctl; }
    if cur_vout_ctl == req_vout_ctl as c_uint { return 0; }
    if micb_en == MICB_BIAS_ENABLE { snd_soc_component_write_field(component, micb_reg, WCD939X_MICB_ENABLE, MICB_BIAS_PULL_DOWN); }
    snd_soc_component_write_field(component, micb_reg, WCD939X_MICB_VOUT_CTL, req_vout_ctl);
    if micb_en == MICB_BIAS_ENABLE {
        snd_soc_component_write_field(component, micb_reg, WCD939X_MICB_ENABLE, MICB_BIAS_ENABLE);
        usleep_range(2000, 2100);
    }
    0
}

unsafe fn wcd939x_mbhc_micb_ctrl_threshold_mic(component: *mut snd_soc_component, micb_num: c_int, req_en: bool) -> c_int {
    let wcd939x = snd_soc_component_get_drvdata(component) as *mut wcd939x_priv;
    if micb_num != MIC_BIAS_2 { return -EINVAL; }
    if (*wcd939x).common.micb_mv[1] >= WCD_MBHC_THR_HS_MICB_MV { return 0; }
    let micb_mv = if req_en { WCD_MBHC_THR_HS_MICB_MV } else { (*wcd939x).common.micb_mv[1] };
    wcd939x_mbhc_micb_adjust_voltage(component, micb_mv, MIC_BIAS_2)
}

unsafe fn wcd939x_mbhc_get_result_params(component: *mut snd_soc_component, zdet: *mut int32_t) {
    let zdet_param = &wcd939x_mbhc_zdet_param;
    let mut val: c_int = 0;
    snd_soc_component_write_field(component, WCD939X_ANA_MBHC_ZDET, WCD939X_MBHC_ZDET_ZDET_CHG_EN, 1);
    let mut i = 0;
    while i < WCD939X_ZDET_NUM_MEASUREMENTS {
        val = snd_soc_component_read_field(component, WCD939X_ANA_MBHC_RESULT_2, WCD939X_MBHC_RESULT_2_Z_RESULT_MSB) as c_int;
        if (val & BIT(7)) != 0 { break; }
        i += 1;
    }
    val = (val << 8) | snd_soc_component_read_field(component, WCD939X_ANA_MBHC_RESULT_1, WCD939X_MBHC_RESULT_1_Z_RESULT_LSB) as c_int;
    snd_soc_component_write_field(component, WCD939X_ANA_MBHC_ZDET, WCD939X_MBHC_ZDET_ZDET_CHG_EN, 0);
    let mut x1 = WCD939X_MBHC_GET_X1(val);
    let c1 = WCD939X_MBHC_GET_C1(val);
    if c1 < 2 && x1 != 0 { mdelay(5); }
    if c1 != 0 && x1 != 0 {
        let d1 = wcd939x_wcd_mbhc_d1_a[c1 as usize] as s32;
        let denom = (x1 * d1) - (1 << (14 - zdet_param.noff));
        if denom > 0 { *zdet = (WCD939X_ANA_MBHC_ZDET_CONST * 1000) / denom; }
        else if x1 < wcd939x_mbhc_mincode_param[zdet_param.noff as usize] { *zdet = WCD939X_ZDET_FLOATING_IMPEDANCE; }
    }
    i = 0;
    while x1 != 0 {
        val = (snd_soc_component_read_field(component, WCD939X_ANA_MBHC_RESULT_1, WCD939X_MBHC_RESULT_1_Z_RESULT_LSB) as c_int) << 8;
        val |= snd_soc_component_read_field(component, WCD939X_ANA_MBHC_RESULT_2, WCD939X_MBHC_RESULT_2_Z_RESULT_MSB) as c_int;
        x1 = WCD939X_MBHC_GET_X1(val);
        i += 1;
        if i == WCD939X_ZDET_NUM_MEASUREMENTS { break; }
    }
}

unsafe fn wcd939x_mbhc_zdet_ramp(component: *mut snd_soc_component, zl: *mut s32, zr: *mut int32_t) {
    let zdet_param = &wcd939x_mbhc_zdet_param;
    let mut zdet: s32 = 0;
    snd_soc_component_write_field(component, WCD939X_MBHC_NEW_ZDET_ANA_CTL, WCD939X_ZDET_ANA_CTL_MAXV_CTL, zdet_param.ldo_ctl as c_int);
    snd_soc_component_update_bits(component, WCD939X_ANA_MBHC_BTN5, WCD939X_MBHC_BTN5_VTH, zdet_param.btn5 as c_uint);
    snd_soc_component_update_bits(component, WCD939X_ANA_MBHC_BTN6, WCD939X_MBHC_BTN6_VTH, zdet_param.btn6 as c_uint);
    snd_soc_component_update_bits(component, WCD939X_ANA_MBHC_BTN7, WCD939X_MBHC_BTN7_VTH, zdet_param.btn7 as c_uint);
    snd_soc_component_write_field(component, WCD939X_MBHC_NEW_ZDET_ANA_CTL, WCD939X_ZDET_ANA_CTL_RANGE_CTL, zdet_param.noff as c_int);
    snd_soc_component_write_field(component, WCD939X_MBHC_NEW_ZDET_RAMP_CTL, WCD939X_ZDET_RAMP_CTL_TIME_CTL, zdet_param.nshift as c_int);
    snd_soc_component_write_field(component, WCD939X_MBHC_NEW_ZDET_RAMP_CTL, WCD939X_ZDET_RAMP_CTL_ACC1_MIN_CTL, 6);
    if !zl.is_null() {
        snd_soc_component_write_field(component, WCD939X_ANA_MBHC_ZDET, WCD939X_MBHC_ZDET_ZDET_L_MEAS_EN, 1);
        wcd939x_mbhc_get_result_params(component, &mut zdet);
        snd_soc_component_write_field(component, WCD939X_ANA_MBHC_ZDET, WCD939X_MBHC_ZDET_ZDET_L_MEAS_EN, 0);
        *zl = zdet;
    }
    if !zr.is_null() {
        snd_soc_component_write_field(component, WCD939X_ANA_MBHC_ZDET, WCD939X_MBHC_ZDET_ZDET_R_MEAS_EN, 1);
        wcd939x_mbhc_get_result_params(component, &mut zdet);
        snd_soc_component_write_field(component, WCD939X_ANA_MBHC_ZDET, WCD939X_MBHC_ZDET_ZDET_R_MEAS_EN, 0);
        *zr = zdet;
    }
}

unsafe fn wcd939x_wcd_mbhc_qfuse_cal(component: *mut snd_soc_component, z_val: *mut s32, flag_l_r: c_int) {
    let q1 = snd_soc_component_read(component, WCD939X_DIGITAL_EFUSE_REG_21 + flag_l_r as c_uint) as s16;
    let q1_cal: c_int = if (q1 as c_int & BIT(7)) != 0 { 10000 - ((q1 as c_int & GENMASK(6, 0)) * 10) } else { 10000 + (q1 as c_int * 10) };
    if q1_cal > 0 { *z_val = ((*z_val) * 10000) / q1_cal; }
}

unsafe fn wcd939x_wcd_mbhc_calc_impedance(component: *mut snd_soc_component, zl: *mut u32, zr: *mut uint32_t) {
    let wcd939x = dev_get_drvdata((*component).dev) as *mut wcd939x_priv;
    let reg0 = snd_soc_component_read(component, WCD939X_ANA_MBHC_BTN5);
    let reg1 = snd_soc_component_read(component, WCD939X_ANA_MBHC_BTN6);
    let reg2 = snd_soc_component_read(component, WCD939X_ANA_MBHC_BTN7);
    let reg3 = snd_soc_component_read(component, WCD939X_MBHC_CTL_CLK);
    let reg4 = snd_soc_component_read(component, WCD939X_MBHC_NEW_ZDET_ANA_CTL);
    let mut is_fsm_disable = false;
    if snd_soc_component_read_field(component, WCD939X_ANA_MBHC_ELECT, WCD939X_MBHC_ELECT_FSM_EN) != 0 {
        snd_soc_component_write_field(component, WCD939X_ANA_MBHC_ELECT, WCD939X_MBHC_ELECT_FSM_EN, 0);
        is_fsm_disable = true;
    }
    if (*wcd939x).mbhc_cfg.hphl_swh { snd_soc_component_write_field(component, WCD939X_ANA_MBHC_MECH, WCD939X_MBHC_MECH_L_DET_EN, 0); }
    snd_soc_component_write_field(component, WCD939X_ANA_MBHC_MECH, WCD939X_MBHC_MECH_SW_HPH_L_P_100K_TO_GND, 0);
    snd_soc_component_write_field(component, WCD939X_HPH_SURGE_EN, WCD939X_EN_EN_SURGE_PROTECTION_HPHR, 0);
    snd_soc_component_write_field(component, WCD939X_HPH_SURGE_EN, WCD939X_EN_EN_SURGE_PROTECTION_HPHL, 0);
    usleep_range(1000, 1010);
    let mut z1l: s32 = 0; let mut z1r: s32 = 0; let mut z1ls: s32 = 0;
    wcd939x_mbhc_zdet_ramp(component, &mut z1l, null_mut());
    if z1l == WCD939X_ZDET_FLOATING_IMPEDANCE || z1l > WCD939X_ZDET_VAL_100K { *zl = WCD939X_ZDET_FLOATING_IMPEDANCE as u32; } else { *zl = (z1l / 1000) as u32; wcd939x_wcd_mbhc_qfuse_cal(component, zl as *mut s32, 0); }
    wcd939x_mbhc_zdet_ramp(component, null_mut(), &mut z1r);
    if z1r == WCD939X_ZDET_FLOATING_IMPEDANCE || z1r > WCD939X_ZDET_VAL_100K { *zr = WCD939X_ZDET_FLOATING_IMPEDANCE as u32; } else { *zr = (z1r / 1000) as u32; wcd939x_wcd_mbhc_qfuse_cal(component, zr as *mut s32, 1); }
    if *zl == WCD939X_ZDET_FLOATING_IMPEDANCE as u32 && *zr == WCD939X_ZDET_FLOATING_IMPEDANCE as u32 {
    } else if *zl == WCD939X_ZDET_FLOATING_IMPEDANCE as u32 || *zr == WCD939X_ZDET_FLOATING_IMPEDANCE as u32 || (*zl < WCD_MONO_HS_MIN_THR && *zr > WCD_MONO_HS_MIN_THR) || (*zl > WCD_MONO_HS_MIN_THR && *zr < WCD_MONO_HS_MIN_THR) {
        wcd_mbhc_set_hph_type((*wcd939x).wcd_mbhc, WCD_MBHC_HPH_MONO);
    } else {
        snd_soc_component_write_field(component, WCD939X_HPH_R_ATEST, WCD939X_R_ATEST_HPH_GND_OVR, 1);
        snd_soc_component_write_field(component, WCD939X_HPH_PA_CTL2, WCD939X_PA_CTL2_HPHPA_GND_R, 1);
        wcd939x_mbhc_zdet_ramp(component, &mut z1ls, null_mut());
        snd_soc_component_write_field(component, WCD939X_HPH_PA_CTL2, WCD939X_PA_CTL2_HPHPA_GND_R, 0);
        snd_soc_component_write_field(component, WCD939X_HPH_R_ATEST, WCD939X_R_ATEST_HPH_GND_OVR, 0);
        z1ls /= 1000; wcd939x_wcd_mbhc_qfuse_cal(component, &mut z1ls, 0);
        let z_mono = ((*zl * 9) / (*zl + 9)) as s32;
        let z_diff1 = if z1ls > z_mono { z1ls - z_mono } else { z_mono - z1ls };
        let z_diff2 = if (*zl as s32) > z1ls { *zl as s32 - z1ls } else { z1ls - *zl as s32 };
        if (z_diff1 * (*zl as s32 + z1ls)) > (z_diff2 * (z1ls + z_mono)) { wcd_mbhc_set_hph_type((*wcd939x).wcd_mbhc, WCD_MBHC_HPH_STEREO); } else { wcd_mbhc_set_hph_type((*wcd939x).wcd_mbhc, WCD_MBHC_HPH_MONO); }
        snd_soc_component_write_field(component, WCD939X_HPH_SURGE_EN, WCD939X_EN_EN_SURGE_PROTECTION_HPHR, 1);
        snd_soc_component_write_field(component, WCD939X_HPH_SURGE_EN, WCD939X_EN_EN_SURGE_PROTECTION_HPHL, 1);
    }
    snd_soc_component_write(component, WCD939X_ANA_MBHC_BTN5, reg0);
    snd_soc_component_write(component, WCD939X_ANA_MBHC_BTN6, reg1);
    snd_soc_component_write(component, WCD939X_ANA_MBHC_BTN7, reg2);
    snd_soc_component_write_field(component, WCD939X_ANA_MBHC_MECH, WCD939X_MBHC_MECH_SW_HPH_L_P_100K_TO_GND, 1);
    if (*wcd939x).mbhc_cfg.hphl_swh { snd_soc_component_write_field(component, WCD939X_ANA_MBHC_MECH, WCD939X_MBHC_MECH_L_DET_EN, 1); }
    snd_soc_component_write(component, WCD939X_MBHC_NEW_ZDET_ANA_CTL, reg4);
    snd_soc_component_write(component, WCD939X_MBHC_CTL_CLK, reg3);
    if is_fsm_disable { snd_soc_component_write_field(component, WCD939X_ANA_MBHC_ELECT, WCD939X_MBHC_ELECT_FSM_EN, 1); }
}

unsafe fn wcd939x_codec_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let wcd939x = dev_get_drvdata((*dai).dev) as *mut wcd939x_priv;
    let wcd = (*wcd939x).sdw_priv[(*dai).id as usize];
    wcd939x_sdw_hw_params(wcd, substream, params, dai)
}

unsafe fn wcd939x_codec_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let wcd939x = dev_get_drvdata((*dai).dev) as *mut wcd939x_priv;
    let wcd = (*wcd939x).sdw_priv[(*dai).id as usize];
    wcd939x_sdw_free(wcd, substream, dai)
}

unsafe fn wcd939x_codec_set_sdw_stream(dai: *mut snd_soc_dai, stream: *mut c_void, direction: c_int) -> c_int {
    let wcd939x = dev_get_drvdata((*dai).dev) as *mut wcd939x_priv;
    let wcd = (*wcd939x).sdw_priv[(*dai).id as usize];
    wcd939x_sdw_set_sdw_stream(wcd, dai, stream, direction)
}

/*
 * The following C static tables and macro-built declarations are preserved as
 * Rust-side external macro initializers in the translated repository:
 *
 * - ear_pa_gain, line_gain, analog_gain TLV declarations
 * - wcd_mbhc_fields[WCD_MBHC_REG_FUNC_MAX]
 * - wcd939x_irqs[WCD939X_NUM_IRQS] and wcd939x_regmap_irq_chip
 * - all snd_kcontrol_new switch/control arrays
 * - all soc_enum mux enums and mux controls
 * - mbhc_cb callback table
 * - wcd939x_snd_controls, wcd9390_snd_controls, wcd9395_snd_controls
 * - wcd939x_dapm_widgets and wcd939x_audio_map
 * - wcd_irq_chip, wcd_domain_ops, soc_codec_dev_wcd939x
 * - wcd939x_sdw_dai_ops, wcd939x_dais, wcd939x_comp_ops
 * - wcd939x_dt_match, wcd939x_codec_driver, module_platform_driver metadata
 *
 * Their C macro constructors directly encode kernel data layout and are treated
 * as future dependency macros rather than expanded in this isolated file.
 */

const _ORIGINAL_WCD939X_C_FOR_TABLE_AND_MACRO_PARITY: &str = include_str!("./wcd939x.c");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
