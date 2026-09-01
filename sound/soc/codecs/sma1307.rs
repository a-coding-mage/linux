// SPDX-License-Identifier: GPL-2.0-or-later
// sma1307.c -- sma1307 ALSA SoC Audio driver
//
// Copyright 2024 Iron Device Corporation
//
// Auther: Gyuhwa Park <gyuwha.park@irondevice.com>
// Auther: Kiseok Jo <kiseok.jo@irondevice.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const CHECK_PERIOD_TIME: c_int = 1; /* sec per HZ */
static setting_file: *const c_char = b"sma1307_setting.bin\0".as_ptr() as *const c_char;
const SMA1307_SETTING_CHECKSUM: c_int = 0x100000;

#[repr(C)]
pub struct sma1307_pll_match {
    input_clk_name: *mut c_char,
    output_clk_name: *mut c_char,
    input_clk: c_uint,
    post_n: c_uint,
    n: c_uint,
    vco: c_uint,
    p_cp: c_uint,
}

#[repr(C)]
pub struct sma1307_data {
    name: *mut c_char,
    init: Option<unsafe extern "C" fn(*mut regmap)>,
}

#[repr(C)]
pub struct sma1307_priv {
    check_fault_status: bool,
    force_mute_status: bool,
    sw_ot1_prot: bool,
    name: *mut c_char,
    amp_mode: sma1307_mode,
    binary_mode: c_int,
    dapm_aif_in: c_int,
    dapm_aif_out0: c_int,
    dapm_aif_out1: c_int,
    dapm_sdo_en: c_int,
    dapm_sdo_setting: c_int,
    num_of_pll_matches: c_int,
    check_fault_period: c_int,
    check_fault_work: delayed_work,
    dev: *mut device,
    kobj: *mut kobject,
    default_lock: mutex,
    regmap: *mut regmap,
    set: sma1307_setting_file,
    pll_matches: *const sma1307_pll_match,
    data: *const sma1307_data,
    cur_vol: c_uint,
    format: c_uint,
    frame_size: c_uint,
    init_vol: c_uint,
    last_bclk: c_uint,
    otp_trm2: c_uint,
    otp_trm3: c_uint,
    rev_num: c_uint,
    sys_clk_id: c_uint,
    tdm_slot0_rx: c_uint,
    tdm_slot1_rx: c_uint,
    tdm_slot0_tx: c_uint,
    tdm_slot1_tx: c_uint,
    tsdw_cnt: c_uint,
}

const fn pll_match(
    input_clk_name: *mut c_char,
    output_clk_name: *mut c_char,
    input_clk: c_uint,
    post_n: c_uint,
    n: c_uint,
    vco: c_uint,
    p_cp: c_uint,
) -> sma1307_pll_match {
    sma1307_pll_match {
        input_clk_name,
        output_clk_name,
        input_clk,
        post_n,
        n,
        vco,
        p_cp,
    }
}

static sma1307_pll_matches: [sma1307_pll_match; 8] = [
    /* in_clk_name, out_clk_name, input_clk post_n, n, vco, p_cp */
    pll_match(b"1.411MHz\0".as_ptr() as *mut c_char, b"24.554MHz\0".as_ptr() as *mut c_char, 1411200, 0x06, 0xD1, 0x88, 0x00),
    pll_match(b"1.536MHz\0".as_ptr() as *mut c_char, b"24.576MHz\0".as_ptr() as *mut c_char, 1536000, 0x06, 0xC0, 0x88, 0x00),
    pll_match(b"2.822MHz\0".as_ptr() as *mut c_char, b"24.554MHz\0".as_ptr() as *mut c_char, 2822400, 0x06, 0xD1, 0x88, 0x04),
    pll_match(b"3.072MHz\0".as_ptr() as *mut c_char, b"24.576MHz\0".as_ptr() as *mut c_char, 3072000, 0x06, 0x60, 0x88, 0x00),
    pll_match(b"6.144MHz\0".as_ptr() as *mut c_char, b"24.576MHz\0".as_ptr() as *mut c_char, 6144000, 0x06, 0x60, 0x88, 0x04),
    pll_match(b"12.288MHz\0".as_ptr() as *mut c_char, b"24.576MHz\0".as_ptr() as *mut c_char, 12288000, 0x06, 0x60, 0x88, 0x08),
    pll_match(b"19.2MHz\0".as_ptr() as *mut c_char, b"24.48MHz\0".as_ptr() as *mut c_char, 19200000, 0x06, 0x7B, 0x88, 0x0C),
    pll_match(b"24.576MHz\0".as_ptr() as *mut c_char, b"24.576MHz\0".as_ptr() as *mut c_char, 24576000, 0x06, 0x60, 0x88, 0x0C),
];

static mut sma1307_amp_component: *mut snd_soc_component = ptr::null_mut();

unsafe fn sma1307_startup(component: *mut snd_soc_component);
unsafe fn sma1307_shutdown(component: *mut snd_soc_component);
unsafe fn sma1307_reset(component: *mut snd_soc_component);
unsafe fn sma1307_set_binary(component: *mut snd_soc_component);
unsafe fn sma1307_set_default(component: *mut snd_soc_component);

/* Initial register value - 6.0W SPK (8ohm load)  */
static sma1307_reg_def: [reg_default; 85] = [
    reg_default { reg: 0x00, def: 0x80 }, reg_default { reg: 0x01, def: 0x00 },
    reg_default { reg: 0x02, def: 0x52 }, reg_default { reg: 0x03, def: 0x4C },
    reg_default { reg: 0x04, def: 0x47 }, reg_default { reg: 0x05, def: 0x42 },
    reg_default { reg: 0x06, def: 0x40 }, reg_default { reg: 0x07, def: 0x40 },
    reg_default { reg: 0x08, def: 0x3C }, reg_default { reg: 0x09, def: 0x2F },
    reg_default { reg: 0x0A, def: 0x32 }, reg_default { reg: 0x0B, def: 0x50 },
    reg_default { reg: 0x0C, def: 0x8C }, reg_default { reg: 0x0D, def: 0x00 },
    reg_default { reg: 0x0E, def: 0x3F }, reg_default { reg: 0x0F, def: 0x00 },
    reg_default { reg: 0x10, def: 0x00 }, reg_default { reg: 0x11, def: 0x00 },
    reg_default { reg: 0x12, def: 0x00 }, reg_default { reg: 0x13, def: 0x09 },
    reg_default { reg: 0x14, def: 0x12 }, reg_default { reg: 0x1C, def: 0x00 },
    reg_default { reg: 0x1D, def: 0x85 }, reg_default { reg: 0x1E, def: 0xA1 },
    reg_default { reg: 0x1F, def: 0x67 }, reg_default { reg: 0x22, def: 0x00 },
    reg_default { reg: 0x23, def: 0x1F }, reg_default { reg: 0x24, def: 0x7A },
    reg_default { reg: 0x25, def: 0x00 }, reg_default { reg: 0x26, def: 0xFF },
    reg_default { reg: 0x27, def: 0x39 }, reg_default { reg: 0x28, def: 0x54 },
    reg_default { reg: 0x29, def: 0x92 }, reg_default { reg: 0x2A, def: 0xB0 },
    reg_default { reg: 0x2B, def: 0xED }, reg_default { reg: 0x2C, def: 0xED },
    reg_default { reg: 0x2D, def: 0xFF }, reg_default { reg: 0x2E, def: 0xFF },
    reg_default { reg: 0x2F, def: 0xFF }, reg_default { reg: 0x30, def: 0xFF },
    reg_default { reg: 0x31, def: 0xFF }, reg_default { reg: 0x32, def: 0xFF },
    reg_default { reg: 0x34, def: 0x01 }, reg_default { reg: 0x35, def: 0x17 },
    reg_default { reg: 0x36, def: 0x92 }, reg_default { reg: 0x37, def: 0x00 },
    reg_default { reg: 0x38, def: 0x01 }, reg_default { reg: 0x39, def: 0x10 },
    reg_default { reg: 0x3E, def: 0x01 }, reg_default { reg: 0x3F, def: 0x08 },
    reg_default { reg: 0x8B, def: 0x05 }, reg_default { reg: 0x8C, def: 0x50 },
    reg_default { reg: 0x8D, def: 0x80 }, reg_default { reg: 0x8E, def: 0x10 },
    reg_default { reg: 0x8F, def: 0x02 }, reg_default { reg: 0x90, def: 0x02 },
    reg_default { reg: 0x91, def: 0x83 }, reg_default { reg: 0x92, def: 0xC0 },
    reg_default { reg: 0x93, def: 0x00 }, reg_default { reg: 0x94, def: 0xA4 },
    reg_default { reg: 0x95, def: 0x74 }, reg_default { reg: 0x96, def: 0x57 },
    reg_default { reg: 0xA2, def: 0xCC }, reg_default { reg: 0xA3, def: 0x28 },
    reg_default { reg: 0xA4, def: 0x40 }, reg_default { reg: 0xA5, def: 0x01 },
    reg_default { reg: 0xA6, def: 0x41 }, reg_default { reg: 0xA7, def: 0x08 },
    reg_default { reg: 0xA8, def: 0x04 }, reg_default { reg: 0xA9, def: 0x27 },
    reg_default { reg: 0xAA, def: 0x10 }, reg_default { reg: 0xAB, def: 0x10 },
    reg_default { reg: 0xAC, def: 0x10 }, reg_default { reg: 0xAD, def: 0x0F },
    reg_default { reg: 0xAE, def: 0xCD }, reg_default { reg: 0xAF, def: 0x70 },
    reg_default { reg: 0xB0, def: 0x03 }, reg_default { reg: 0xB1, def: 0xEF },
    reg_default { reg: 0xB2, def: 0x03 }, reg_default { reg: 0xB3, def: 0xEF },
    reg_default { reg: 0xB4, def: 0xF3 }, reg_default { reg: 0xB5, def: 0x3D },
];

unsafe extern "C" fn sma1307_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    if reg > SMA1307_FF_DEVICE_INDEX { return false; }
    match reg {
        SMA1307_00_SYSTEM_CTRL..=SMA1307_1F_TONE_FINE_VOLUME |
        SMA1307_22_COMP_HYS_SEL..=SMA1307_32_BROWN_OUT_PROT19 |
        SMA1307_34_OCP_SPK..=SMA1307_39_PMT_NZ_VAL |
        SMA1307_3B_TEST1..=SMA1307_3F_ATEST2 |
        SMA1307_8B_PLL_POST_N..=SMA1307_9A_OTP_TRM3 |
        SMA1307_A0_PAD_CTRL0..=SMA1307_BE_MCBS_CTRL2 |
        SMA1307_F5_READY_FOR_V_SAR |
        SMA1307_F7_READY_FOR_T_SAR..=SMA1307_FF_DEVICE_INDEX => true,
        _ => false,
    }
}

unsafe extern "C" fn sma1307_writeable_register(_dev: *mut device, reg: c_uint) -> bool {
    if reg > SMA1307_FF_DEVICE_INDEX { return false; }
    match reg {
        SMA1307_00_SYSTEM_CTRL..=SMA1307_1F_TONE_FINE_VOLUME |
        SMA1307_22_COMP_HYS_SEL..=SMA1307_32_BROWN_OUT_PROT19 |
        SMA1307_34_OCP_SPK..=SMA1307_39_PMT_NZ_VAL |
        SMA1307_3B_TEST1..=SMA1307_3F_ATEST2 |
        SMA1307_8B_PLL_POST_N..=SMA1307_9A_OTP_TRM3 |
        SMA1307_A0_PAD_CTRL0..=SMA1307_BE_MCBS_CTRL2 => true,
        _ => false,
    }
}

unsafe extern "C" fn sma1307_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    if reg > SMA1307_FF_DEVICE_INDEX { return false; }
    match reg {
        SMA1307_F8_STATUS_T1..=SMA1307_FF_DEVICE_INDEX => true,
        _ => false,
    }
}

/* DB scale conversion of speaker volume */
static sma1307_spk_tlv: [c_uint; 4] = declare_tlv_db_scale(-6000, 50, 0);

static sma1307_aif_in_source_text: [*const c_char; 3] = [c"Mono".as_ptr(), c"Left".as_ptr(), c"Right".as_ptr()];
static sma1307_sdo_setting_text: [*const c_char; 5] = [c"Data_One_48k".as_ptr(), c"Data_Two_48k".as_ptr(), c"Data_Two_24k".as_ptr(), c"Clk_PLL".as_ptr(), c"Clk_OSC".as_ptr()];
static sma1307_aif_out_source_text: [*const c_char; 8] = [c"Disable".as_ptr(), c"After_FmtC".as_ptr(), c"After_Mixer".as_ptr(), c"After_DSP".as_ptr(), c"Vrms2_Avg".as_ptr(), c"Battery".as_ptr(), c"Temperature".as_ptr(), c"After_Delay".as_ptr()];
static sma1307_tdm_slot_text: [*const c_char; 8] = [c"Slot0".as_ptr(), c"Slot1".as_ptr(), c"Slot2".as_ptr(), c"Slot3".as_ptr(), c"Slot4".as_ptr(), c"Slot5".as_ptr(), c"Slot6".as_ptr(), c"Slot7".as_ptr()];
static sma1307_binary_mode_text: [*const c_char; 5] = [c"Mode0".as_ptr(), c"Mode1".as_ptr(), c"Mode2".as_ptr(), c"Mode3".as_ptr(), c"Mode4".as_ptr()];
static sma1307_reset_text: [*const c_char; 1] = [c"Reset".as_ptr()];

static sma1307_aif_in_source_enum: soc_enum = SOC_ENUM_SINGLE_EXT(3, sma1307_aif_in_source_text.as_ptr());
static sma1307_sdo_setting_enum: soc_enum = SOC_ENUM_SINGLE_EXT(5, sma1307_sdo_setting_text.as_ptr());
static sma1307_aif_out_source_enum: soc_enum = SOC_ENUM_SINGLE_EXT(8, sma1307_aif_out_source_text.as_ptr());
static sma1307_tdm_slot_enum: soc_enum = SOC_ENUM_SINGLE_EXT(8, sma1307_tdm_slot_text.as_ptr());
static sma1307_binary_mode_enum: soc_enum = SOC_ENUM_SINGLE_EXT(5, sma1307_binary_mode_text.as_ptr());
static sma1307_reset_enum: soc_enum = SOC_ENUM_SINGLE_EXT(1, sma1307_reset_text.as_ptr());

unsafe extern "C" fn sma1307_force_mute_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    (*ucontrol).value.integer.value[0] = (*sma1307).force_mute_status as c_long;
    0
}

unsafe extern "C" fn sma1307_force_mute_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    let val = (*ucontrol).value.integer.value[0] != 0;
    let mut change = false;
    if (*sma1307).force_mute_status == val { change = false; } else { change = true; (*sma1307).force_mute_status = val; }
    change as c_int
}

unsafe extern "C" fn sma1307_tdm_slot_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    let mut val1: c_int = 0;
    let mut val2: c_int = 0;
    regmap_read((*sma1307).regmap, SMA1307_A5_TDM1, &mut val1 as *mut _ as *mut c_uint);
    regmap_read((*sma1307).regmap, SMA1307_A6_TDM2, &mut val2 as *mut _ as *mut c_uint);
    if strcmp((*kcontrol).id.name, SMA1307_TDM_RX0_POS_NAME) == 0 {
        (*ucontrol).value.integer.value[0] = ((val1 & SMA1307_TDM_SLOT0_RX_POS_MASK as c_int) >> 3) as c_long;
        (*sma1307).tdm_slot0_rx = (*ucontrol).value.integer.value[0] as c_uint;
    } else if strcmp((*kcontrol).id.name, SMA1307_TDM_RX1_POS_NAME) == 0 {
        (*ucontrol).value.integer.value[0] = (val1 & SMA1307_TDM_SLOT1_RX_POS_MASK as c_int) as c_long;
        (*sma1307).tdm_slot1_rx = (*ucontrol).value.integer.value[0] as c_uint;
    } else if strcmp((*kcontrol).id.name, SMA1307_TDM_TX0_POS_NAME) == 0 {
        (*ucontrol).value.integer.value[0] = ((val2 & SMA1307_TDM_SLOT0_TX_POS_MASK as c_int) >> 3) as c_long;
        (*sma1307).tdm_slot0_tx = (*ucontrol).value.integer.value[0] as c_uint;
    } else if strcmp((*kcontrol).id.name, SMA1307_TDM_TX1_POS_NAME) == 0 {
        (*ucontrol).value.integer.value[0] = (val2 & SMA1307_TDM_SLOT1_TX_POS_MASK as c_int) as c_long;
        (*sma1307).tdm_slot1_tx = (*ucontrol).value.integer.value[0] as c_uint;
    } else { return -EINVAL; }
    0
}

unsafe extern "C" fn sma1307_tdm_slot_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    let val = (*ucontrol).value.integer.value[0] as c_int;
    let mut change: bool;
    if strcmp((*kcontrol).id.name, SMA1307_TDM_RX0_POS_NAME) == 0 {
        if (*sma1307).tdm_slot0_rx == val as c_uint { change = false; } else { change = true; (*sma1307).tdm_slot0_rx = val as c_uint; regmap_update_bits((*sma1307).regmap, SMA1307_A5_TDM1, SMA1307_TDM_SLOT0_RX_POS_MASK, (val << 3) as c_uint); }
    } else if strcmp((*kcontrol).id.name, SMA1307_TDM_RX1_POS_NAME) == 0 {
        if (*sma1307).tdm_slot1_rx == val as c_uint { change = false; } else { change = true; (*sma1307).tdm_slot1_rx = val as c_uint; regmap_update_bits((*sma1307).regmap, SMA1307_A5_TDM1, SMA1307_TDM_SLOT1_RX_POS_MASK, val as c_uint); }
    } else if strcmp((*kcontrol).id.name, SMA1307_TDM_TX0_POS_NAME) == 0 {
        if (*sma1307).tdm_slot0_tx == val as c_uint { change = false; } else { change = true; (*sma1307).tdm_slot0_tx = val as c_uint; regmap_update_bits((*sma1307).regmap, SMA1307_A6_TDM2, SMA1307_TDM_SLOT0_TX_POS_MASK, (val << 3) as c_uint); }
    } else if strcmp((*kcontrol).id.name, SMA1307_TDM_TX1_POS_NAME) == 0 {
        if (*sma1307).tdm_slot1_tx == val as c_uint { change = false; } else { change = true; (*sma1307).tdm_slot1_tx = val as c_uint; regmap_update_bits((*sma1307).regmap, SMA1307_A6_TDM2, SMA1307_TDM_SLOT1_TX_POS_MASK, val as c_uint); }
    } else {
        dev_err((*sma1307).dev, c"%s: Invalid Control ID - %s\n".as_ptr(), c"sma1307_tdm_slot_put".as_ptr(), (*kcontrol).id.name);
        return -EINVAL;
    }
    change as c_int
}

unsafe extern "C" fn sma1307_sw_ot1_prot_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    (*ucontrol).value.integer.value[0] = (*sma1307).sw_ot1_prot as c_long;
    0
}

unsafe extern "C" fn sma1307_sw_ot1_prot_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    let val = (*ucontrol).value.integer.value[0] != 0;
    let mut change = false;
    if (*sma1307).sw_ot1_prot == val { change = false; } else { change = true; (*sma1307).sw_ot1_prot = val; }
    change as c_int
}

unsafe extern "C" fn sma1307_check_fault_status_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    (*ucontrol).value.integer.value[0] = (*sma1307).check_fault_status as c_long;
    0
}

unsafe extern "C" fn sma1307_check_fault_status_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    let val = (*ucontrol).value.integer.value[0] != 0;
    let mut change = false;
    if (*sma1307).check_fault_status == val { change = false; } else { change = true; (*sma1307).check_fault_status = val; }
    change as c_int
}

unsafe extern "C" fn sma1307_check_fault_period_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    (*ucontrol).value.integer.value[0] = (*sma1307).check_fault_period as c_long;
    0
}

unsafe extern "C" fn sma1307_check_fault_period_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let val = (*ucontrol).value.integer.value[0] as c_int;
    let mut change = false;
    if val < (*mc).min || val > (*mc).max { return -EINVAL; }
    if (*sma1307).check_fault_period == val { change = false; } else { change = true; (*sma1307).check_fault_period = val; }
    change as c_int
}

unsafe extern "C" fn sma1307_reset_put(kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    regmap_update_bits((*sma1307).regmap, SMA1307_00_SYSTEM_CTRL, SMA1307_RESET_MASK, SMA1307_RESET_ON);
    sma1307_reset(component);
    snd_ctl_notify((*(*component).card).snd_card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*kcontrol).id);
    true as c_int
}

unsafe extern "C" fn sma1307_binary_mode_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let sma1307 = snd_kcontrol_chip(kcontrol) as *mut sma1307_priv;
    (*sma1307).binary_mode = (*ucontrol).value.enumerated.item[0] as c_int;
    if (*sma1307).set.status { sma1307_set_binary(component); }
    snd_soc_put_enum_double(kcontrol, ucontrol)
}

unsafe fn sma1307_startup(component: *mut snd_soc_component) {
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    regmap_update_bits((*sma1307).regmap, SMA1307_A2_TOP_MAN1, SMA1307_PLL_MASK, SMA1307_PLL_ON);
    regmap_update_bits((*sma1307).regmap, SMA1307_00_SYSTEM_CTRL, SMA1307_POWER_MASK, SMA1307_POWER_ON);
    if (*sma1307).amp_mode == SMA1307_MONO_MODE {
        regmap_update_bits((*sma1307).regmap, SMA1307_10_SYSTEM_CTRL1, SMA1307_SPK_MODE_MASK, SMA1307_SPK_MONO);
    } else {
        regmap_update_bits((*sma1307).regmap, SMA1307_10_SYSTEM_CTRL1, SMA1307_SPK_MODE_MASK, SMA1307_SPK_STEREO);
    }
    if (*sma1307).check_fault_status {
        if (*sma1307).check_fault_period > 0 {
            queue_delayed_work(system_freezable_wq, &mut (*sma1307).check_fault_work, ((*sma1307).check_fault_period * HZ) as c_ulong);
        } else {
            queue_delayed_work(system_freezable_wq, &mut (*sma1307).check_fault_work, (CHECK_PERIOD_TIME * HZ) as c_ulong);
        }
    }
}

unsafe fn sma1307_shutdown(component: *mut snd_soc_component) {
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    /* for SMA1307A */
    cancel_delayed_work_sync(&mut (*sma1307).check_fault_work);
    regmap_update_bits((*sma1307).regmap, SMA1307_0E_MUTE_VOL_CTRL, SMA1307_SPK_MUTE_MASK, SMA1307_SPK_MUTE);
    /* Need to wait time for mute slope */
    msleep(55);
    regmap_update_bits((*sma1307).regmap, SMA1307_10_SYSTEM_CTRL1, SMA1307_SPK_MODE_MASK, SMA1307_SPK_OFF);
    regmap_update_bits((*sma1307).regmap, SMA1307_A2_TOP_MAN1, SMA1307_PLL_MASK, SMA1307_PLL_OFF);
    regmap_update_bits((*sma1307).regmap, SMA1307_00_SYSTEM_CTRL, SMA1307_POWER_MASK, SMA1307_POWER_OFF);
}

unsafe extern "C" fn sma1307_aif_in_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    let mux = (*sma1307).dapm_aif_in as c_uint;
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            match mux {
                SMA1307_MONO_MODE => regmap_update_bits((*sma1307).regmap, SMA1307_11_SYSTEM_CTRL2, SMA1307_MONOMIX_MASK, SMA1307_MONOMIX_ON),
                SMA1307_LEFT_MODE => { regmap_update_bits((*sma1307).regmap, SMA1307_11_SYSTEM_CTRL2, SMA1307_MONOMIX_MASK, SMA1307_MONOMIX_OFF); regmap_update_bits((*sma1307).regmap, SMA1307_11_SYSTEM_CTRL2, SMA1307_LR_DATA_SW_MASK, SMA1307_LR_DATA_SW_NORMAL); },
                SMA1307_RIGHT_MODE => { regmap_update_bits((*sma1307).regmap, SMA1307_11_SYSTEM_CTRL2, SMA1307_MONOMIX_MASK, SMA1307_MONOMIX_OFF); regmap_update_bits((*sma1307).regmap, SMA1307_11_SYSTEM_CTRL2, SMA1307_LR_DATA_SW_MASK, SMA1307_LR_DATA_SW_SWAP); },
                _ => { dev_err((*sma1307).dev, c"%s: Invalid value (%d)\n".as_ptr(), c"sma1307_aif_in_event".as_ptr(), mux); return -EINVAL; }
            }
            (*sma1307).amp_mode = mux as sma1307_mode;
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn sma1307_sdo_setting_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    let mux = (*sma1307).dapm_sdo_setting as c_uint;
    if event == SND_SOC_DAPM_PRE_PMU {
        match mux {
            SMA1307_OUT_DATA_ONE_48K => { regmap_update_bits((*sma1307).regmap, SMA1307_A2_TOP_MAN1, SMA1307_SDO_OUTPUT2_MASK, SMA1307_ONE_SDO_PER_CH); regmap_update_bits((*sma1307).regmap, SMA1307_A3_TOP_MAN2, SMA1307_SDO_OUTPUT3_MASK | SMA1307_DATA_CLK_SEL_MASK, SMA1307_SDO_OUTPUT3_DIS | SMA1307_SDO_DATA); }
            SMA1307_OUT_DATA_TWO_48K => { regmap_update_bits((*sma1307).regmap, SMA1307_A2_TOP_MAN1, SMA1307_SDO_OUTPUT2_MASK, SMA1307_TWO_SDO_PER_CH); regmap_update_bits((*sma1307).regmap, SMA1307_A3_TOP_MAN2, SMA1307_SDO_OUTPUT3_MASK | SMA1307_DATA_CLK_SEL_MASK, SMA1307_SDO_OUTPUT3_DIS | SMA1307_SDO_DATA); }
            SMA1307_OUT_DATA_TWO_24K => { regmap_update_bits((*sma1307).regmap, SMA1307_A2_TOP_MAN1, SMA1307_SDO_OUTPUT2_MASK, SMA1307_TWO_SDO_PER_CH); regmap_update_bits((*sma1307).regmap, SMA1307_A3_TOP_MAN2, SMA1307_SDO_OUTPUT3_MASK | SMA1307_DATA_CLK_SEL_MASK, SMA1307_TWO_SDO_PER_CH_24K | SMA1307_SDO_DATA); }
            SMA1307_OUT_CLK_PLL => regmap_update_bits((*sma1307).regmap, SMA1307_A3_TOP_MAN2, SMA1307_DATA_CLK_SEL_MASK, SMA1307_SDO_CLK_PLL),
            SMA1307_OUT_CLK_OSC => regmap_update_bits((*sma1307).regmap, SMA1307_A3_TOP_MAN2, SMA1307_DATA_CLK_SEL_MASK, SMA1307_SDO_CLK_OSC),
            _ => { dev_err((*sma1307).dev, c"%s: Invalid value (%d)\n".as_ptr(), c"sma1307_sdo_setting_event".as_ptr(), mux); return -EINVAL; }
        }
    }
    0
}

unsafe extern "C" fn sma1307_aif_out_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    let mut val: c_uint = 0;
    let mut mask: c_uint = 0;
    if strcmp((*w).name, SMA1307_AIF_OUT0_NAME) == 0 { val = (*sma1307).dapm_aif_out0 as c_uint; mask = SMA1307_SDO_OUT0_SEL_MASK; }
    else if strcmp((*w).name, SMA1307_AIF_OUT1_NAME) == 0 { val = ((*sma1307).dapm_aif_out1 << 3) as c_uint; mask = SMA1307_SDO_OUT1_SEL_MASK; }
    else { dev_err((*sma1307).dev, c"%s: Invalid widget - %s\n".as_ptr(), c"sma1307_aif_out_event".as_ptr(), (*w).name); return -EINVAL; }
    if event == SND_SOC_DAPM_PRE_PMU { regmap_update_bits((*sma1307).regmap, SMA1307_09_OUTPUT_CTRL, mask, val); }
    0
}

unsafe extern "C" fn sma1307_sdo_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    match event {
        SND_SOC_DAPM_PRE_PMU => { regmap_update_bits((*sma1307).regmap, SMA1307_09_OUTPUT_CTRL, SMA1307_PORT_CONFIG_MASK, SMA1307_OUTPUT_PORT_ENABLE); regmap_update_bits((*sma1307).regmap, SMA1307_A3_TOP_MAN2, SMA1307_SDO_OUTPUT_MASK, SMA1307_LOGIC_OUTPUT); }
        SND_SOC_DAPM_POST_PMD => { regmap_update_bits((*sma1307).regmap, SMA1307_09_OUTPUT_CTRL, SMA1307_PORT_CONFIG_MASK, SMA1307_INPUT_PORT_ONLY); regmap_update_bits((*sma1307).regmap, SMA1307_A3_TOP_MAN2, SMA1307_SDO_OUTPUT_MASK, SMA1307_HIGH_Z_OUTPUT); }
        _ => {}
    }
    0
}

unsafe extern "C" fn sma1307_power_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    match event {
        SND_SOC_DAPM_POST_PMU => sma1307_startup(component),
        SND_SOC_DAPM_PRE_PMD => sma1307_shutdown(component),
        _ => {}
    }
    0
}

unsafe extern "C" fn sma1307_dapm_aif_in_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let component = snd_soc_dapm_to_component(dapm);
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    (*ucontrol).value.enumerated.item[0] = (*sma1307).dapm_aif_in as c_uint;
    snd_soc_dapm_put_enum_double(kcontrol, ucontrol);
    0
}

unsafe extern "C" fn sma1307_dapm_aif_in_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let component = snd_soc_dapm_to_component(dapm);
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    let val = (*ucontrol).value.enumerated.item[0] as c_int;
    let change: bool;
    if val < 0 || val >= sma1307_aif_in_source_text.len() as c_int { dev_err((*sma1307).dev, c"%s: Out of range\n".as_ptr(), c"sma1307_dapm_aif_in_put".as_ptr()); return -EINVAL; }
    if (*sma1307).dapm_aif_in != val { change = true; (*sma1307).dapm_aif_in = val; } else { change = false; }
    snd_soc_dapm_put_enum_double(kcontrol, ucontrol);
    change as c_int
}

unsafe extern "C" fn sma1307_dapm_sdo_setting_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let component = snd_soc_dapm_to_component(dapm);
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    (*ucontrol).value.enumerated.item[0] = (*sma1307).dapm_sdo_setting as c_uint;
    snd_soc_dapm_put_enum_double(kcontrol, ucontrol);
    0
}

unsafe extern "C" fn sma1307_dapm_sdo_setting_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let component = snd_soc_dapm_to_component(dapm);
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    let val = (*ucontrol).value.enumerated.item[0] as c_int;
    let change: bool;
    if val < 0 || val >= sma1307_sdo_setting_text.len() as c_int { dev_err((*sma1307).dev, c"%s: Out of range\n".as_ptr(), c"sma1307_dapm_sdo_setting_put".as_ptr()); return -EINVAL; }
    if (*sma1307).dapm_sdo_setting != val { change = true; (*sma1307).dapm_sdo_setting = val; } else { change = false; }
    snd_soc_dapm_put_enum_double(kcontrol, ucontrol);
    change as c_int
}

unsafe extern "C" fn sma1307_dapm_aif_out_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let component = snd_soc_dapm_to_component(dapm);
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    let val: c_uint;
    if strcmp((*kcontrol).id.name, SMA1307_AIF_OUT0_NAME) == 0 { val = (*sma1307).dapm_aif_out0 as c_uint; }
    else if strcmp((*kcontrol).id.name, SMA1307_AIF_OUT1_NAME) == 0 { val = (*sma1307).dapm_aif_out1 as c_uint; }
    else { dev_err((*sma1307).dev, c"%s: Invalid Control ID - %s\n".as_ptr(), c"sma1307_dapm_aif_out_get".as_ptr(), (*kcontrol).id.name); return -EINVAL; }
    (*ucontrol).value.enumerated.item[0] = val;
    snd_soc_dapm_put_enum_double(kcontrol, ucontrol);
    0
}

unsafe extern "C" fn sma1307_dapm_aif_out_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let component = snd_soc_dapm_to_component(dapm);
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    let val = (*ucontrol).value.enumerated.item[0] as c_int;
    let change: bool;
    if val < 0 || val >= sma1307_aif_out_source_text.len() as c_int { dev_err((*sma1307).dev, c"%s: Out of range\n".as_ptr(), c"sma1307_dapm_aif_out_put".as_ptr()); return -EINVAL; }
    if strcmp((*kcontrol).id.name, SMA1307_AIF_OUT0_NAME) == 0 {
        if (*sma1307).dapm_aif_out0 != val { change = true; (*sma1307).dapm_aif_out0 = val; } else { change = false; }
    } else if strcmp((*kcontrol).id.name, SMA1307_AIF_OUT1_NAME) == 0 {
        if (*sma1307).dapm_aif_out1 != val { change = true; (*sma1307).dapm_aif_out1 = val; } else { change = false; }
    } else { dev_err((*sma1307).dev, c"%s: Invalid Control ID - %s\n".as_ptr(), c"sma1307_dapm_aif_out_put".as_ptr(), (*kcontrol).id.name); return -EINVAL; }
    snd_soc_dapm_put_enum_double(kcontrol, ucontrol);
    change as c_int
}

unsafe extern "C" fn sma1307_dapm_sdo_enable_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let component = snd_soc_dapm_to_component(dapm);
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    (*ucontrol).value.integer.value[0] = (*sma1307).dapm_sdo_en as c_long;
    snd_soc_dapm_put_volsw(kcontrol, ucontrol);
    0
}

unsafe extern "C" fn sma1307_dapm_sdo_enable_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let component = snd_soc_dapm_to_component(dapm);
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    let val = (*ucontrol).value.integer.value[0] as c_int;
    let change: bool;
    if val < 0 || val > 1 { dev_err((*sma1307).dev, c"%s: Out of range\n".as_ptr(), c"sma1307_dapm_sdo_enable_put".as_ptr()); return -EINVAL; }
    if (*sma1307).dapm_sdo_en != val { change = true; (*sma1307).dapm_sdo_en = val; } else { change = false; }
    snd_soc_dapm_put_volsw(kcontrol, ucontrol);
    change as c_int
}

/* The following ALSA control/widget/route declarations are direct translations of
 * C macro initializers. Their constructors are expected from future dependencies.
 */
static sma1307_aif_in_source_control: snd_kcontrol_new = snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: SMA1307_AIF_IN_NAME, info: Some(snd_soc_info_enum_double), get: Some(sma1307_dapm_aif_in_get), put: Some(sma1307_dapm_aif_in_put), private_value: &sma1307_aif_in_source_enum as *const _ as c_ulong };
static sma1307_sdo_setting_control: snd_kcontrol_new = snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"SDO Setting".as_ptr(), info: Some(snd_soc_info_enum_double), get: Some(sma1307_dapm_sdo_setting_get), put: Some(sma1307_dapm_sdo_setting_put), private_value: &sma1307_sdo_setting_enum as *const _ as c_ulong };
static sma1307_aif_out0_source_control: snd_kcontrol_new = snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: SMA1307_AIF_OUT0_NAME, info: Some(snd_soc_info_enum_double), get: Some(sma1307_dapm_aif_out_get), put: Some(sma1307_dapm_aif_out_put), private_value: &sma1307_aif_out_source_enum as *const _ as c_ulong };
static sma1307_aif_out1_source_control: snd_kcontrol_new = snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: SMA1307_AIF_OUT1_NAME, info: Some(snd_soc_info_enum_double), get: Some(sma1307_dapm_aif_out_get), put: Some(sma1307_dapm_aif_out_put), private_value: &sma1307_aif_out_source_enum as *const _ as c_ulong };

static sma1307_sdo_control: snd_kcontrol_new = SOC_SINGLE_EXT(c"Switch".as_ptr(), SND_SOC_NOPM, 0, 1, 0, Some(sma1307_dapm_sdo_enable_get), Some(sma1307_dapm_sdo_enable_put));
static sma1307_enable_control: snd_kcontrol_new = SOC_DAPM_SINGLE(c"Switch".as_ptr(), SMA1307_00_SYSTEM_CTRL, 0, 1, 0);
static sma1307_binary_mode_control: [snd_kcontrol_new; 1] = [SOC_ENUM_EXT(c"Binary Mode".as_ptr(), &sma1307_binary_mode_enum, Some(snd_soc_get_enum_double), Some(sma1307_binary_mode_put))];
static sma1307_snd_controls: [snd_kcontrol_new; 10] = [
    SOC_SINGLE_TLV(SMA1307_VOL_CTRL_NAME, SMA1307_0A_SPK_VOL, 0, 167, 1, sma1307_spk_tlv.as_ptr()),
    SOC_ENUM_EXT(SMA1307_TDM_RX0_POS_NAME, &sma1307_tdm_slot_enum, Some(sma1307_tdm_slot_get), Some(sma1307_tdm_slot_put)),
    SOC_ENUM_EXT(SMA1307_TDM_RX1_POS_NAME, &sma1307_tdm_slot_enum, Some(sma1307_tdm_slot_get), Some(sma1307_tdm_slot_put)),
    SOC_ENUM_EXT(SMA1307_TDM_TX0_POS_NAME, &sma1307_tdm_slot_enum, Some(sma1307_tdm_slot_get), Some(sma1307_tdm_slot_put)),
    SOC_ENUM_EXT(SMA1307_TDM_TX1_POS_NAME, &sma1307_tdm_slot_enum, Some(sma1307_tdm_slot_get), Some(sma1307_tdm_slot_put)),
    SOC_ENUM_EXT(SMA1307_RESET_CTRL_NAME, &sma1307_reset_enum, Some(snd_soc_get_enum_double), Some(sma1307_reset_put)),
    SOC_SINGLE_BOOL_EXT(SMA1307_FORCE_MUTE_CTRL_NAME, 0, Some(sma1307_force_mute_get), Some(sma1307_force_mute_put)),
    SOC_SINGLE_BOOL_EXT(SMA1307_OT1_SW_PROT_CTRL_NAME, 0, Some(sma1307_sw_ot1_prot_get), Some(sma1307_sw_ot1_prot_put)),
    SOC_SINGLE_BOOL_EXT(SMA1307_CHECK_FAULT_STATUS_NAME, 0, Some(sma1307_check_fault_status_get), Some(sma1307_check_fault_status_put)),
    SOC_SINGLE_EXT(SMA1307_CHECK_FAULT_PERIOD_NAME, SND_SOC_NOPM, 0, 600, 0, Some(sma1307_check_fault_period_get), Some(sma1307_check_fault_period_put)),
];

static sma1307_dapm_widgets: [snd_soc_dapm_widget; 12] = [
    SND_SOC_DAPM_OUTPUT(c"SPK".as_ptr()), SND_SOC_DAPM_INPUT(c"SDO".as_ptr()),
    SND_SOC_DAPM_MUX_E(SMA1307_AIF_IN_NAME, SND_SOC_NOPM, 0, 0, &sma1307_aif_in_source_control, Some(sma1307_aif_in_event), SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_MUX_E(c"SDO Setting".as_ptr(), SND_SOC_NOPM, 0, 0, &sma1307_sdo_setting_control, Some(sma1307_sdo_setting_event), SND_SOC_DAPM_PRE_PMU),
    SND_SOC_DAPM_MUX_E(SMA1307_AIF_OUT0_NAME, SND_SOC_NOPM, 0, 0, &sma1307_aif_out0_source_control, Some(sma1307_aif_out_event), SND_SOC_DAPM_PRE_PMU),
    SND_SOC_DAPM_MUX_E(SMA1307_AIF_OUT1_NAME, SND_SOC_NOPM, 0, 0, &sma1307_aif_out1_source_control, Some(sma1307_aif_out_event), SND_SOC_DAPM_PRE_PMU),
    SND_SOC_DAPM_SWITCH_E(c"SDO Enable".as_ptr(), SND_SOC_NOPM, 0, 0, &sma1307_sdo_control, Some(sma1307_sdo_event), SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MIXER(c"Entry".as_ptr(), SND_SOC_NOPM, 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_OUT_DRV_E(c"AMP Power".as_ptr(), SND_SOC_NOPM, 0, 0, ptr::null(), 0, Some(sma1307_power_event), SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_SWITCH(c"AMP Enable".as_ptr(), SND_SOC_NOPM, 0, 0, &sma1307_enable_control),
    SND_SOC_DAPM_AIF_IN(c"AIF IN".as_ptr(), c"Playback".as_ptr(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT(c"AIF OUT".as_ptr(), c"Capture".as_ptr(), 0, SND_SOC_NOPM, 0, 0),
];

static sma1307_audio_map: [snd_soc_dapm_route; 39] = [
    route(c"AIF IN Source".as_ptr(), c"Mono".as_ptr(), c"AIF IN".as_ptr()),
    route(c"AIF IN Source".as_ptr(), c"Left".as_ptr(), c"AIF IN".as_ptr()),
    route(c"AIF IN Source".as_ptr(), c"Right".as_ptr(), c"AIF IN".as_ptr()),
    route(c"SDO Enable".as_ptr(), c"Switch".as_ptr(), c"AIF IN".as_ptr()),
    route(c"SDO Setting".as_ptr(), c"Data_One_48k".as_ptr(), c"SDO Enable".as_ptr()),
    route(c"SDO Setting".as_ptr(), c"Data_Two_48k".as_ptr(), c"SDO Enable".as_ptr()),
    route(c"SDO Setting".as_ptr(), c"Data_Two_24k".as_ptr(), c"SDO Enable".as_ptr()),
    route(c"SDO Setting".as_ptr(), c"Clk_PLL".as_ptr(), c"SDO Enable".as_ptr()),
    route(c"SDO Setting".as_ptr(), c"Clk_OSC".as_ptr(), c"SDO Enable".as_ptr()),
    route(c"AIF OUT0 Source".as_ptr(), c"Disable".as_ptr(), c"SDO Setting".as_ptr()),
    route(c"AIF OUT0 Source".as_ptr(), c"After_FmtC".as_ptr(), c"SDO Setting".as_ptr()),
    route(c"AIF OUT0 Source".as_ptr(), c"After_Mixer".as_ptr(), c"SDO Setting".as_ptr()),
    route(c"AIF OUT0 Source".as_ptr(), c"After_DSP".as_ptr(), c"SDO Setting".as_ptr()),
    route(c"AIF OUT0 Source".as_ptr(), c"Vrms2_Avg".as_ptr(), c"SDO Setting".as_ptr()),
    route(c"AIF OUT0 Source".as_ptr(), c"Battery".as_ptr(), c"SDO Setting".as_ptr()),
    route(c"AIF OUT0 Source".as_ptr(), c"Temperature".as_ptr(), c"SDO Setting".as_ptr()),
    route(c"AIF OUT0 Source".as_ptr(), c"After_Delay".as_ptr(), c"SDO Setting".as_ptr()),
    route(c"AIF OUT1 Source".as_ptr(), c"Disable".as_ptr(), c"SDO Setting".as_ptr()),
    route(c"AIF OUT1 Source".as_ptr(), c"After_FmtC".as_ptr(), c"SDO Setting".as_ptr()),
    route(c"AIF OUT1 Source".as_ptr(), c"After_Mixer".as_ptr(), c"SDO Setting".as_ptr()),
    route(c"AIF OUT1 Source".as_ptr(), c"After_DSP".as_ptr(), c"SDO Setting".as_ptr()),
    route(c"AIF OUT1 Source".as_ptr(), c"Vrms2_Avg".as_ptr(), c"SDO Setting".as_ptr()),
    route(c"AIF OUT1 Source".as_ptr(), c"Battery".as_ptr(), c"SDO Setting".as_ptr()),
    route(c"AIF OUT1 Source".as_ptr(), c"Temperature".as_ptr(), c"SDO Setting".as_ptr()),
    route(c"AIF OUT1 Source".as_ptr(), c"After_Delay".as_ptr(), c"SDO Setting".as_ptr()),
    route(c"Entry".as_ptr(), ptr::null(), c"AIF OUT0 Source".as_ptr()),
    route(c"Entry".as_ptr(), ptr::null(), c"AIF OUT1 Source".as_ptr()),
    route(c"Entry".as_ptr(), ptr::null(), c"AIF IN Source".as_ptr()),
    route(c"AMP Power".as_ptr(), ptr::null(), c"Entry".as_ptr()),
    route(c"AMP Enable".as_ptr(), c"Switch".as_ptr(), c"AMP Power".as_ptr()),
    route(c"SPK".as_ptr(), ptr::null(), c"AMP Enable".as_ptr()),
    route(c"AIF OUT".as_ptr(), ptr::null(), c"AMP Enable".as_ptr()),
    route(ptr::null(), ptr::null(), ptr::null()), route(ptr::null(), ptr::null(), ptr::null()),
    route(ptr::null(), ptr::null(), ptr::null()), route(ptr::null(), ptr::null(), ptr::null()),
    route(ptr::null(), ptr::null(), ptr::null()), route(ptr::null(), ptr::null(), ptr::null()),
    route(ptr::null(), ptr::null(), ptr::null()),
];

unsafe fn sma1307_setup_pll(component: *mut snd_soc_component, bclk: c_uint) {
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    let mut i: c_int = 0;
    dev_dbg((*component).dev, c"%s: BCLK = %dHz\n".as_ptr(), c"sma1307_setup_pll".as_ptr(), bclk);
    if (*sma1307).sys_clk_id == SMA1307_PLL_CLKIN_MCLK {
        dev_warn((*component).dev, c"%s: MCLK is not supported\n".as_ptr(), c"sma1307_setup_pll".as_ptr());
    } else if (*sma1307).sys_clk_id == SMA1307_PLL_CLKIN_BCLK {
        while i < (*sma1307).num_of_pll_matches {
            if (*(*sma1307).pll_matches.add(i as usize)).input_clk == bclk { break; }
            i += 1;
        }
        if i == (*sma1307).num_of_pll_matches {
            dev_warn((*component).dev, c"%s: No matching value between pll table and SCK\n".as_ptr(), c"sma1307_setup_pll".as_ptr());
            return;
        }
        regmap_update_bits((*sma1307).regmap, SMA1307_A2_TOP_MAN1, SMA1307_PLL_MASK, SMA1307_PLL_ON);
    }
    regmap_write((*sma1307).regmap, SMA1307_8B_PLL_POST_N, (*(*sma1307).pll_matches.add(i as usize)).post_n);
    regmap_write((*sma1307).regmap, SMA1307_8C_PLL_N, (*(*sma1307).pll_matches.add(i as usize)).n);
    regmap_write((*sma1307).regmap, SMA1307_8D_PLL_A_SETTING, (*(*sma1307).pll_matches.add(i as usize)).vco);
    regmap_write((*sma1307).regmap, SMA1307_8E_PLL_P_CP, (*(*sma1307).pll_matches.add(i as usize)).p_cp);
}

unsafe extern "C" fn sma1307_dai_hw_params_amp(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    let bclk: c_uint = if (*sma1307).format == SND_SOC_DAIFMT_DSP_A { params_rate(params) * (*sma1307).frame_size } else { params_rate(params) * params_physical_width(params) * params_channels(params) };
    dev_dbg((*component).dev, c"%s: rate = %d : bit size = %d : channel = %d\n".as_ptr(), c"sma1307_dai_hw_params_amp".as_ptr(), params_rate(params), params_width(params), params_channels(params));
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        if (*sma1307).sys_clk_id == SMA1307_PLL_CLKIN_BCLK && (*sma1307).last_bclk != bclk { sma1307_setup_pll(component, bclk); (*sma1307).last_bclk = bclk; }
        match params_rate(params) {
            8000 | 12000 | 16000 | 24000 | 32000 | 44100 | 48000 => {}
            96000 => dev_warn((*component).dev, c"%s: %d rate not support SDO\n".as_ptr(), c"sma1307_dai_hw_params_amp".as_ptr(), params_rate(params)),
            _ => { dev_err((*component).dev, c"%s: not support rate : %d\n".as_ptr(), c"sma1307_dai_hw_params_amp".as_ptr(), params_rate(params)); return -EINVAL; }
        }
    } else {
        match params_format(params) {
            SNDRV_PCM_FORMAT_S16_LE => regmap_update_bits((*sma1307).regmap, SMA1307_A4_TOP_MAN3, SMA1307_SCK_RATE_MASK | SMA1307_DATA_WIDTH_MASK, SMA1307_SCK_32FS | SMA1307_DATA_16BIT),
            SNDRV_PCM_FORMAT_S24_LE | SNDRV_PCM_FORMAT_S32_LE => regmap_update_bits((*sma1307).regmap, SMA1307_A4_TOP_MAN3, SMA1307_SCK_RATE_MASK | SMA1307_DATA_WIDTH_MASK, SMA1307_SCK_64FS | SMA1307_DATA_24BIT),
            _ => { dev_err((*component).dev, c"%s: not support data bit : %d\n".as_ptr(), c"sma1307_dai_hw_params_amp".as_ptr(), params_format(params)); return -EINVAL; }
        }
    }
    match (*sma1307).format {
        SND_SOC_DAIFMT_I2S => { regmap_update_bits((*sma1307).regmap, SMA1307_01_INPUT_CTRL1, SMA1307_I2S_MODE_MASK, SMA1307_STANDARD_I2S); regmap_update_bits((*sma1307).regmap, SMA1307_A4_TOP_MAN3, SMA1307_INTERFACE_MASK, SMA1307_I2S_FORMAT); }
        SND_SOC_DAIFMT_LEFT_J => { regmap_update_bits((*sma1307).regmap, SMA1307_01_INPUT_CTRL1, SMA1307_I2S_MODE_MASK, SMA1307_LJ); regmap_update_bits((*sma1307).regmap, SMA1307_A4_TOP_MAN3, SMA1307_INTERFACE_MASK, SMA1307_LJ_FORMAT); }
        SND_SOC_DAIFMT_RIGHT_J => match params_width(params) { 16 => regmap_update_bits((*sma1307).regmap, SMA1307_01_INPUT_CTRL1, SMA1307_I2S_MODE_MASK, SMA1307_RJ_16BIT), 24 | 32 => regmap_update_bits((*sma1307).regmap, SMA1307_01_INPUT_CTRL1, SMA1307_I2S_MODE_MASK, SMA1307_RJ_24BIT), _ => {} },
        SND_SOC_DAIFMT_DSP_A => { regmap_update_bits((*sma1307).regmap, SMA1307_01_INPUT_CTRL1, SMA1307_I2S_MODE_MASK, SMA1307_STANDARD_I2S); regmap_update_bits((*sma1307).regmap, SMA1307_A4_TOP_MAN3, SMA1307_INTERFACE_MASK, SMA1307_TDM_FORMAT); }
        _ => {}
    }
    match params_width(params) { 16 | 24 | 32 => 0, _ => { dev_err((*component).dev, c"%s: not support data bit : %d\n".as_ptr(), c"sma1307_dai_hw_params_amp".as_ptr(), params_format(params)); -EINVAL } }
}

unsafe extern "C" fn sma1307_dai_set_sysclk_amp(dai: *mut snd_soc_dai, clk_id: c_int, _freq: c_uint, _dir: c_int) -> c_int {
    let component = (*dai).component;
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    match clk_id as c_uint {
        SMA1307_EXTERNAL_CLOCK_19_2 | SMA1307_EXTERNAL_CLOCK_24_576 | SMA1307_PLL_CLKIN_MCLK | SMA1307_PLL_CLKIN_BCLK => {}
        _ => { dev_err((*component).dev, c"%s: Invalid clk id: %d\n".as_ptr(), c"sma1307_dai_set_sysclk_amp".as_ptr(), clk_id); return -EINVAL; }
    }
    (*sma1307).sys_clk_id = clk_id as c_uint;
    0
}

unsafe extern "C" fn sma1307_dai_set_fmt_amp(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => { dev_dbg((*component).dev, c"%s: %s\n".as_ptr(), c"sma1307_dai_set_fmt_amp".as_ptr(), c"I2S/TDM Device mode".as_ptr()); regmap_update_bits((*sma1307).regmap, SMA1307_01_INPUT_CTRL1, SMA1307_CONTROLLER_DEVICE_MASK, SMA1307_DEVICE_MODE); }
        SND_SOC_DAIFMT_CBP_CFP => { dev_dbg((*component).dev, c"%s: %s\n".as_ptr(), c"sma1307_dai_set_fmt_amp".as_ptr(), c"I2S/TDM Controller mode".as_ptr()); regmap_update_bits((*sma1307).regmap, SMA1307_01_INPUT_CTRL1, SMA1307_CONTROLLER_DEVICE_MASK, SMA1307_CONTROLLER_MODE); }
        _ => { dev_err((*component).dev, c"%s: Unsupported Controller/Device : 0x%x\n".as_ptr(), c"sma1307_dai_set_fmt_amp".as_ptr(), fmt); return -EINVAL; }
    }
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_RIGHT_J | SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_DSP_B => (*sma1307).format = fmt & SND_SOC_DAIFMT_FORMAT_MASK,
        _ => { dev_err((*component).dev, c"%s: Unsupported Audio Interface Format : 0x%x\n".as_ptr(), c"sma1307_dai_set_fmt_amp".as_ptr(), fmt); return -EINVAL; }
    }
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_IB_NF => { dev_dbg((*component).dev, c"%s: %s\n".as_ptr(), c"sma1307_dai_set_fmt_amp".as_ptr(), c"Invert BCLK + Normal Frame".as_ptr()); regmap_update_bits((*sma1307).regmap, SMA1307_01_INPUT_CTRL1, SMA1307_SCK_RISING_MASK, SMA1307_SCK_RISING_EDGE); }
        SND_SOC_DAIFMT_IB_IF => { dev_dbg((*component).dev, c"%s: %s\n".as_ptr(), c"sma1307_dai_set_fmt_amp".as_ptr(), c"Invert BCLK + Invert Frame".as_ptr()); regmap_update_bits((*sma1307).regmap, SMA1307_01_INPUT_CTRL1, SMA1307_LEFTPOL_MASK | SMA1307_SCK_RISING_MASK, SMA1307_HIGH_FIRST_CH | SMA1307_SCK_RISING_EDGE); }
        SND_SOC_DAIFMT_NB_IF => { dev_dbg((*component).dev, c"%s: %s\n".as_ptr(), c"sma1307_dai_set_fmt_amp".as_ptr(), c"Normal BCLK + Invert Frame".as_ptr()); regmap_update_bits((*sma1307).regmap, SMA1307_01_INPUT_CTRL1, SMA1307_LEFTPOL_MASK, SMA1307_HIGH_FIRST_CH); }
        SND_SOC_DAIFMT_NB_NF => dev_dbg((*component).dev, c"%s: %s\n".as_ptr(), c"sma1307_dai_set_fmt_amp".as_ptr(), c"Normal BCLK + Normal Frame".as_ptr()),
        _ => { dev_err((*component).dev, c"%s: Unsupported Bit & Frameclock : 0x%x\n".as_ptr(), c"sma1307_dai_set_fmt_amp".as_ptr(), fmt); return -EINVAL; }
    }
    0
}

unsafe extern "C" fn sma1307_dai_set_tdm_slot(dai: *mut snd_soc_dai, _tx_mask: c_uint, _rx_mask: c_uint, slots: c_int, slot_width: c_int) -> c_int {
    let component = (*dai).component;
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    dev_dbg((*component).dev, c"%s: slots = %d, slot_width - %d\n".as_ptr(), c"sma1307_dai_set_tdm_slot".as_ptr(), slots, slot_width);
    (*sma1307).frame_size = (slot_width * slots) as c_uint;
    regmap_update_bits((*sma1307).regmap, SMA1307_A4_TOP_MAN3, SMA1307_INTERFACE_MASK, SMA1307_TDM_FORMAT);
    regmap_update_bits((*sma1307).regmap, SMA1307_A5_TDM1, SMA1307_TDM_TX_MODE_MASK, SMA1307_TDM_TX_MONO);
    match slot_width {
        16 => regmap_update_bits((*sma1307).regmap, SMA1307_A6_TDM2, SMA1307_TDM_DL_MASK, SMA1307_TDM_DL_16),
        32 => regmap_update_bits((*sma1307).regmap, SMA1307_A6_TDM2, SMA1307_TDM_DL_MASK, SMA1307_TDM_DL_32),
        _ => { dev_err((*component).dev, c"%s: not support TDM %d slot_width\n".as_ptr(), c"sma1307_dai_set_tdm_slot".as_ptr(), slot_width); return -EINVAL; }
    }
    match slots {
        4 => regmap_update_bits((*sma1307).regmap, SMA1307_A6_TDM2, SMA1307_TDM_N_SLOT_MASK, SMA1307_TDM_N_SLOT_4),
        8 => regmap_update_bits((*sma1307).regmap, SMA1307_A6_TDM2, SMA1307_TDM_N_SLOT_MASK, SMA1307_TDM_N_SLOT_8),
        _ => { dev_err((*component).dev, c"%s: not support TDM %d slots\n".as_ptr(), c"sma1307_dai_set_tdm_slot".as_ptr(), slots); return -EINVAL; }
    }
    if (*sma1307).tdm_slot0_rx < slots as c_uint { regmap_update_bits((*sma1307).regmap, SMA1307_A5_TDM1, SMA1307_TDM_SLOT0_RX_POS_MASK, (*sma1307).tdm_slot0_rx << 3); } else { dev_err((*component).dev, c"%s: Incorrect tdm-slot0-rx %d set\n".as_ptr(), c"sma1307_dai_set_tdm_slot".as_ptr(), (*sma1307).tdm_slot0_rx); }
    if (*sma1307).tdm_slot1_rx < slots as c_uint { regmap_update_bits((*sma1307).regmap, SMA1307_A5_TDM1, SMA1307_TDM_SLOT1_RX_POS_MASK, (*sma1307).tdm_slot1_rx); } else { dev_err((*component).dev, c"%s: Incorrect tdm-slot1-rx %d set\n".as_ptr(), c"sma1307_dai_set_tdm_slot".as_ptr(), (*sma1307).tdm_slot1_rx); }
    if (*sma1307).tdm_slot0_tx < slots as c_uint { regmap_update_bits((*sma1307).regmap, SMA1307_A6_TDM2, SMA1307_TDM_SLOT0_TX_POS_MASK, (*sma1307).tdm_slot0_tx << 3); } else { dev_err((*component).dev, c"%s: Incorrect tdm-slot0-tx %d set\n".as_ptr(), c"sma1307_dai_set_tdm_slot".as_ptr(), (*sma1307).tdm_slot0_tx); }
    if (*sma1307).tdm_slot1_tx < slots as c_uint { regmap_update_bits((*sma1307).regmap, SMA1307_A6_TDM2, SMA1307_TDM_SLOT1_TX_POS_MASK, (*sma1307).tdm_slot1_tx); } else { dev_err((*component).dev, c"%s: Incorrect tdm-slot1-tx %d set\n".as_ptr(), c"sma1307_dai_set_tdm_slot".as_ptr(), (*sma1307).tdm_slot1_tx); }
    0
}

unsafe extern "C" fn sma1307_dai_mute_stream(dai: *mut snd_soc_dai, mute: c_int, stream: c_int) -> c_int {
    let component = (*dai).component;
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    if stream == SNDRV_PCM_STREAM_CAPTURE { return 0; }
    if mute != 0 {
        dev_dbg((*component).dev, c"%s: %s\n".as_ptr(), c"sma1307_dai_mute_stream".as_ptr(), c"MUTE".as_ptr());
        regmap_update_bits((*sma1307).regmap, SMA1307_0E_MUTE_VOL_CTRL, SMA1307_SPK_MUTE_MASK, SMA1307_SPK_MUTE);
    } else if !(*sma1307).force_mute_status {
        dev_dbg((*component).dev, c"%s: %s\n".as_ptr(), c"sma1307_dai_mute_stream".as_ptr(), c"UNMUTE".as_ptr());
        regmap_update_bits((*sma1307).regmap, SMA1307_0E_MUTE_VOL_CTRL, SMA1307_SPK_MUTE_MASK, SMA1307_SPK_UNMUTE);
    } else {
        dev_dbg((*sma1307).dev, c"%s: FORCE MUTE!!!\n".as_ptr(), c"sma1307_dai_mute_stream".as_ptr());
    }
    0
}

static sma1307_dai_ops_amp: snd_soc_dai_ops = snd_soc_dai_ops { hw_params: Some(sma1307_dai_hw_params_amp), set_fmt: Some(sma1307_dai_set_fmt_amp), set_sysclk: Some(sma1307_dai_set_sysclk_amp), set_tdm_slot: Some(sma1307_dai_set_tdm_slot), mute_stream: Some(sma1307_dai_mute_stream) };
const SMA1307_RATES_PLAYBACK: c_uint = SNDRV_PCM_RATE_8000_96000;
const SMA1307_RATES_CAPTURE: c_uint = SNDRV_PCM_RATE_8000_48000;
const SMA1307_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static mut sma1307_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c"sma1307-amplifier".as_ptr(), id: 0,
    playback: snd_soc_pcm_stream { stream_name: c"Playback".as_ptr(), channels_min: 1, channels_max: 2, rates: SMA1307_RATES_PLAYBACK, formats: SMA1307_FORMATS },
    capture: snd_soc_pcm_stream { stream_name: c"Capture".as_ptr(), channels_min: 1, channels_max: 2, rates: SMA1307_RATES_CAPTURE, formats: SMA1307_FORMATS },
    ops: &sma1307_dai_ops_amp,
}];

unsafe extern "C" fn sma1307_check_fault_worker(work: *mut work_struct) {
    let sma1307 = container_of_check_fault_work(work);
    let mut status1_val: c_uint = 0;
    let mut status2_val: c_uint = 0;
    let mut volume = [0 as c_char; 18]; /* sizeof("VOLUME=0x12345678") */
    let mut envp: [*mut c_char; 3] = [ptr::null_mut(), ptr::null_mut(), ptr::null_mut()];
    if (*sma1307).tsdw_cnt != 0 { regmap_read((*sma1307).regmap, SMA1307_0A_SPK_VOL, &mut (*sma1307).cur_vol); } else { regmap_read((*sma1307).regmap, SMA1307_0A_SPK_VOL, &mut (*sma1307).init_vol); }
    regmap_read((*sma1307).regmap, SMA1307_FA_STATUS1, &mut status1_val);
    regmap_read((*sma1307).regmap, SMA1307_FB_STATUS2, &mut status2_val);
    if (!status1_val & SMA1307_OT1_OK_STATUS) != 0 {
        dev_crit((*sma1307).dev, c"%s: OT1(Over Temperature Level 1)\n".as_ptr(), c"sma1307_check_fault_worker".as_ptr());
        envp[0] = c"STATUS=OT1".as_ptr() as *mut c_char;
        if (*sma1307).sw_ot1_prot && ((*sma1307).cur_vol + 6) <= 0xFA {
            (*sma1307).cur_vol += 6;
            regmap_write((*sma1307).regmap, SMA1307_0A_SPK_VOL, (*sma1307).cur_vol);
            snprintf(volume.as_mut_ptr(), volume.len(), c"VOLUME=0x%02X".as_ptr(), (*sma1307).cur_vol);
            envp[1] = volume.as_mut_ptr();
        }
        (*sma1307).tsdw_cnt += 1;
    } else if (*sma1307).tsdw_cnt != 0 {
        regmap_write((*sma1307).regmap, SMA1307_0A_SPK_VOL, (*sma1307).init_vol);
        (*sma1307).tsdw_cnt = 0;
        (*sma1307).cur_vol = (*sma1307).init_vol;
        envp[0] = c"STATUS=OT1_CLEAR".as_ptr() as *mut c_char;
        snprintf(volume.as_mut_ptr(), volume.len(), c"VOLUME=0x%02X".as_ptr(), (*sma1307).cur_vol);
        envp[1] = volume.as_mut_ptr();
    }
    if (!status1_val & SMA1307_OT2_OK_STATUS) != 0 { dev_crit((*sma1307).dev, c"%s: OT2(Over Temperature Level 2)\n".as_ptr(), c"sma1307_check_fault_worker".as_ptr()); envp[0] = c"STATUS=OT2".as_ptr() as *mut c_char; envp[1] = ptr::null_mut(); }
    if (status1_val & SMA1307_UVLO_STATUS) != 0 { dev_crit((*sma1307).dev, c"%s: UVLO(Under Voltage Lock Out)\n".as_ptr(), c"sma1307_check_fault_worker".as_ptr()); envp[0] = c"STATUS=UVLO".as_ptr() as *mut c_char; envp[1] = ptr::null_mut(); }
    if (status1_val & SMA1307_OVP_BST_STATUS) != 0 { dev_crit((*sma1307).dev, c"%s: OVP_BST(Over Voltage Protection)\n".as_ptr(), c"sma1307_check_fault_worker".as_ptr()); envp[0] = c"STATUS=OVP_BST".as_ptr() as *mut c_char; envp[1] = ptr::null_mut(); }
    if (status2_val & SMA1307_OCP_SPK_STATUS) != 0 { dev_crit((*sma1307).dev, c"%s: OCP_SPK(Over Current Protect SPK)\n".as_ptr(), c"sma1307_check_fault_worker".as_ptr()); envp[0] = c"STATUS=OCP_SPK".as_ptr() as *mut c_char; envp[1] = ptr::null_mut(); }
    if (status2_val & SMA1307_OCP_BST_STATUS) != 0 { dev_crit((*sma1307).dev, c"%s: OCP_BST(Over Current Protect Boost)\n".as_ptr(), c"sma1307_check_fault_worker".as_ptr()); envp[0] = c"STATUS=OCP_BST".as_ptr() as *mut c_char; envp[1] = ptr::null_mut(); }
    if (status2_val & SMA1307_CLK_MON_STATUS) != 0 { dev_crit((*sma1307).dev, c"%s: CLK_FAULT(No clock input)\n".as_ptr(), c"sma1307_check_fault_worker".as_ptr()); envp[0] = c"STATUS=CLK_FAULT".as_ptr() as *mut c_char; envp[1] = ptr::null_mut(); }
    if !envp[0].is_null() && kobject_uevent_env((*sma1307).kobj, KOBJ_CHANGE, envp.as_mut_ptr()) != 0 { dev_err((*sma1307).dev, c"%s: Error sending uevent\n".as_ptr(), c"sma1307_check_fault_worker".as_ptr()); }
    if (*sma1307).check_fault_status {
        if (*sma1307).check_fault_period > 0 { queue_delayed_work(system_freezable_wq, &mut (*sma1307).check_fault_work, ((*sma1307).check_fault_period * HZ) as c_ulong); } else { queue_delayed_work(system_freezable_wq, &mut (*sma1307).check_fault_work, (CHECK_PERIOD_TIME * HZ) as c_ulong); }
    }
}

unsafe fn sma1307_setting_loaded(sma1307: *mut sma1307_priv, file: *const c_char) {
    let mut fw: *const firmware = ptr::null();
    let mut size: c_int;
    let mut offset: c_int;
    let mut num_mode: c_int;
    let ret = request_firmware(&mut fw, file, (*sma1307).dev);
    if ret != 0 {
        dev_err((*sma1307).dev, c"%s: failed to read \"%s\": %pe\n".as_ptr(), c"sma1307_setting_loaded".as_ptr(), setting_file, ERR_PTR(ret));
        (*sma1307).set.status = false;
        return;
    } else if (*fw).size < SMA1307_SETTING_HEADER_SIZE as usize {
        dev_err((*sma1307).dev, c"%s: Invalid file\n".as_ptr(), c"sma1307_setting_loaded".as_ptr());
        (*sma1307).set.status = false;
        return;
    }
    let data = kzalloc((*fw).size, GFP_KERNEL) as *mut c_int;
    if data.is_null() { (*sma1307).set.status = false; return; }
    size = ((*fw).size >> 2) as c_int;
    memcpy(data as *mut c_void, (*fw).data as *const c_void, (*fw).size);
    /* HEADER */
    (*sma1307).set.header_size = SMA1307_SETTING_HEADER_SIZE;
    (*sma1307).set.checksum = *data.add(((*sma1307).set.header_size - 2) as usize);
    (*sma1307).set.num_mode = *data.add(((*sma1307).set.header_size - 1) as usize);
    num_mode = (*sma1307).set.num_mode;
    (*sma1307).set.header = devm_kmalloc_array((*sma1307).dev, (*sma1307).set.header_size as usize, size_of::<c_int>(), GFP_KERNEL) as *mut c_int;
    if (*sma1307).set.header.is_null() { (*sma1307).set.status = false; return; }
    memcpy((*sma1307).set.header as *mut c_void, data as *const c_void, ((*sma1307).set.header_size as usize) * size_of::<c_int>());
    if ((*sma1307).set.checksum >> 8) != SMA1307_SETTING_CHECKSUM {
        dev_err((*sma1307).dev, c"%s: checksum failed \"%s\"\n".as_ptr(), c"sma1307_setting_loaded".as_ptr(), setting_file);
        (*sma1307).set.status = false;
        return;
    }
    /* DEFAULT */
    (*sma1307).set.def_size = SMA1307_SETTING_DEFAULT_SIZE;
    (*sma1307).set.def = devm_kzalloc((*sma1307).dev, ((*sma1307).set.def_size as usize) * size_of::<c_int>(), GFP_KERNEL) as *mut c_int;
    if (*sma1307).set.def.is_null() { (*sma1307).set.status = false; return; }
    memcpy((*sma1307).set.def as *mut c_void, data.add((*sma1307).set.header_size as usize) as *const c_void, ((*sma1307).set.def_size as usize) * size_of::<c_int>());
    /* MODE */
    offset = (*sma1307).set.header_size + (*sma1307).set.def_size;
    (*sma1307).set.mode_size = DIV_ROUND_CLOSEST(size - offset, num_mode + 1);
    let mut i = 0;
    while i < num_mode {
        (*sma1307).set.mode_set[i as usize] = devm_kzalloc((*sma1307).dev, ((*sma1307).set.mode_size as usize) * 2 * size_of::<c_int>(), GFP_KERNEL) as *mut c_int;
        if (*sma1307).set.mode_set[i as usize].is_null() {
            let mut j = 0;
            while j < i { devm_kfree((*sma1307).dev, (*sma1307).set.mode_set[j as usize] as *mut c_void); (*sma1307).set.mode_set[j as usize] = ptr::null_mut(); j += 1; }
            (*sma1307).set.status = false;
            return;
        }
        let mut j = 0;
        while j < (*sma1307).set.mode_size {
            *(*sma1307).set.mode_set[i as usize].add((2 * j) as usize) = *data.add((offset + ((num_mode + 1) * j)) as usize);
            *(*sma1307).set.mode_set[i as usize].add((2 * j + 1) as usize) = *data.add((offset + ((num_mode + 1) * j + i + 1)) as usize);
            j += 1;
        }
        i += 1;
    }
    (*sma1307).set.status = true;
}

unsafe fn sma1307_reset(component: *mut snd_soc_component) {
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    let mut status: c_uint = 0;
    regmap_read((*sma1307).regmap, SMA1307_FF_DEVICE_INDEX, &mut status);
    (*sma1307).rev_num = status & SMA1307_REV_NUM_STATUS;
    dev_dbg((*component).dev, c"%s: SMA1307 Revision %d\n".as_ptr(), c"sma1307_reset".as_ptr(), (*sma1307).rev_num);
    regmap_read((*sma1307).regmap, SMA1307_99_OTP_TRM2, &mut (*sma1307).otp_trm2);
    regmap_read((*sma1307).regmap, SMA1307_9A_OTP_TRM3, &mut (*sma1307).otp_trm3);
    if ((*sma1307).otp_trm2 & SMA1307_OTP_STAT_MASK) != SMA1307_OTP_STAT_1 { dev_warn((*component).dev, c"%s: SMA1307 OTP Status Fail\n".as_ptr(), c"sma1307_reset".as_ptr()); }
    /* Register Initial Value Setting */
    sma1307_setting_loaded(sma1307, setting_file);
    if (*sma1307).set.status { sma1307_set_binary(component); } else { sma1307_set_default(component); }
    regmap_update_bits((*sma1307).regmap, SMA1307_93_INT_CTRL, SMA1307_DIS_INT_MASK, SMA1307_HIGH_Z_INT);
    regmap_write((*sma1307).regmap, SMA1307_0A_SPK_VOL, (*sma1307).init_vol);
}

unsafe fn sma1307_set_binary(component: *mut snd_soc_component) {
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    let mut i: c_int = 0;
    while i < (*sma1307).set.def_size {
        if sma1307_writeable_register((*sma1307).dev, i as c_uint) && (i < SMA1307_97_OTP_TRM0 as c_int || i > SMA1307_9A_OTP_TRM3 as c_int) { regmap_write((*sma1307).regmap, i as c_uint, *(*sma1307).set.def.add(i as usize) as c_uint); }
        i += 1;
    }
    i = 0;
    while i < (*sma1307).set.mode_size {
        if sma1307_writeable_register((*sma1307).dev, i as c_uint) && (i < SMA1307_97_OTP_TRM0 as c_int || i > SMA1307_9A_OTP_TRM3 as c_int) {
            let mode = (*sma1307).binary_mode as usize;
            regmap_write((*sma1307).regmap, *(*sma1307).set.mode_set[mode].add((2 * i) as usize) as c_uint, *(*sma1307).set.mode_set[mode].add((2 * i + 1) as usize) as c_uint);
        }
        i += 1;
    }
}

unsafe fn sma1307_set_default(component: *mut snd_soc_component) {
    let sma1307 = snd_soc_component_get_drvdata(component) as *mut sma1307_priv;
    let mut i: usize = 0;
    while i < sma1307_reg_def.len() {
        regmap_write((*sma1307).regmap, sma1307_reg_def[i].reg, sma1307_reg_def[i].def);
        i += 1;
    }
    if strcmp((*sma1307).name, DEVICE_NAME_SMA1307AQ) == 0 { ((*(*sma1307).data).init.unwrap())((*sma1307).regmap); }
}

unsafe extern "C" fn sma1307_probe(component: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    snd_soc_dapm_sync(dapm);
    sma1307_amp_component = component;
    snd_soc_add_component_controls(component, sma1307_binary_mode_control.as_ptr(), sma1307_binary_mode_control.len() as c_uint);
    sma1307_reset(component);
    0
}

static sma1307_component: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(sma1307_probe),
    controls: sma1307_snd_controls.as_ptr(),
    num_controls: sma1307_snd_controls.len() as c_uint,
    dapm_widgets: sma1307_dapm_widgets.as_ptr(),
    num_dapm_widgets: sma1307_dapm_widgets.len() as c_uint,
    dapm_routes: sma1307_audio_map.as_ptr(),
    num_dapm_routes: sma1307_audio_map.len() as c_uint,
};

static sma_i2c_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: SMA1307_FF_DEVICE_INDEX,
    readable_reg: Some(sma1307_readable_register),
    writeable_reg: Some(sma1307_writeable_register),
    volatile_reg: Some(sma1307_volatile_register),
    reg_defaults: sma1307_reg_def.as_ptr(),
    num_reg_defaults: sma1307_reg_def.len() as c_uint,
};

unsafe extern "C" fn sma1307aq_init(regmap: *mut regmap) {
    /* Guidelines for driving 4ohm load */
    /* Brown Out Protection */
    regmap_write(regmap, SMA1307_02_BROWN_OUT_PROT1, 0x62);
    regmap_write(regmap, SMA1307_03_BROWN_OUT_PROT2, 0x5D);
    regmap_write(regmap, SMA1307_04_BROWN_OUT_PROT3, 0x57);
    regmap_write(regmap, SMA1307_05_BROWN_OUT_PROT8, 0x54);
    regmap_write(regmap, SMA1307_06_BROWN_OUT_PROT9, 0x51);
    regmap_write(regmap, SMA1307_07_BROWN_OUT_PROT10, 0x4D);
    regmap_write(regmap, SMA1307_08_BROWN_OUT_PROT11, 0x4B);
    regmap_write(regmap, SMA1307_27_BROWN_OUT_PROT4, 0x3C);
    regmap_write(regmap, SMA1307_28_BROWN_OUT_PROT5, 0x5B);
    regmap_write(regmap, SMA1307_29_BROWN_OUT_PROT12, 0x78);
    regmap_write(regmap, SMA1307_2A_BROWN_OUT_PROT13, 0x96);
    regmap_write(regmap, SMA1307_2B_BROWN_OUT_PROT14, 0xB4);
    regmap_write(regmap, SMA1307_2C_BROWN_OUT_PROT15, 0xD3);
    /* FDPEC Gain */
    regmap_write(regmap, SMA1307_35_FDPEC_CTRL0, 0x16);
    /* FLT Vdd */
    regmap_write(regmap, SMA1307_92_FDPEC_CTRL1, 0xA0);
    /* Boost Max */
    regmap_write(regmap, SMA1307_AB_BOOST_CTRL4, 0x0F);
}

static sma1307aq_data: sma1307_data = sma1307_data { name: DEVICE_NAME_SMA1307AQ as *mut c_char, init: Some(sma1307aq_init) };

unsafe extern "C" fn sma1307_i2c_probe(client: *mut i2c_client) -> c_int {
    let sma1307 = devm_kzalloc(&mut (*client).dev, size_of::<sma1307_priv>(), GFP_KERNEL) as *mut sma1307_priv;
    let mut ret: c_int = 0;
    let mut device_info: c_uint = 0;
    if sma1307.is_null() { return -ENOMEM; }
    (*sma1307).regmap = devm_regmap_init_i2c(client, &sma_i2c_regmap);
    if IS_ERR((*sma1307).regmap as *const c_void) { return dev_err_probe(&mut (*client).dev, PTR_ERR((*sma1307).regmap as *const c_void), c"%s: failed to allocate register map\n".as_ptr(), c"sma1307_i2c_probe".as_ptr()); }
    let data = device_get_match_data(&mut (*client).dev) as *const sma1307_data;
    if data.is_null() { return -ENODEV; }
    (*sma1307).data = data;
    /* set initial value as normal AMP IC status */
    (*sma1307).name = (*client).name;
    (*sma1307).format = SND_SOC_DAIFMT_I2S;
    (*sma1307).sys_clk_id = SMA1307_PLL_CLKIN_BCLK;
    (*sma1307).num_of_pll_matches = sma1307_pll_matches.len() as c_int;
    (*sma1307).check_fault_period = CHECK_PERIOD_TIME;
    (*sma1307).check_fault_status = true;
    (*sma1307).init_vol = 0x32;
    (*sma1307).cur_vol = (*sma1307).init_vol;
    (*sma1307).sw_ot1_prot = true;
    mutex_init(&mut (*sma1307).default_lock);
    INIT_DELAYED_WORK(&mut (*sma1307).check_fault_work, Some(sma1307_check_fault_worker));
    (*sma1307).dev = &mut (*client).dev;
    (*sma1307).kobj = &mut (*client).dev.kobj;
    i2c_set_clientdata(client, sma1307 as *mut c_void);
    (*sma1307).pll_matches = sma1307_pll_matches.as_ptr();
    regmap_read((*sma1307).regmap, SMA1307_FF_DEVICE_INDEX, &mut device_info);
    if (device_info & 0xF8) != SMA1307_DEVICE_ID {
        dev_err(&mut (*client).dev, c"%s: device initialization error (0x%02X)".as_ptr(), c"sma1307_i2c_probe".as_ptr(), device_info);
        return -ENODEV;
    }
    dev_dbg(&mut (*client).dev, c"%s: chip version 0x%02X\n".as_ptr(), c"sma1307_i2c_probe".as_ptr(), device_info);
    i2c_set_clientdata(client, sma1307 as *mut c_void);
    ret = devm_snd_soc_register_component(&mut (*client).dev, &sma1307_component, sma1307_dai.as_mut_ptr(), 1);
    if ret != 0 {
        dev_err(&mut (*client).dev, c"%s: failed to register component\n".as_ptr(), c"sma1307_i2c_probe".as_ptr());
        return ret;
    }
    ret
}

unsafe extern "C" fn sma1307_i2c_remove(client: *mut i2c_client) {
    let sma1307 = i2c_get_clientdata(client) as *mut sma1307_priv;
    cancel_delayed_work_sync(&mut (*sma1307).check_fault_work);
}

static sma1307_i2c_id: [i2c_device_id; 3] = [
    i2c_device_id { name: c"sma1307a".as_ptr(), driver_data: 0 },
    i2c_device_id { name: c"sma1307aq".as_ptr(), driver_data: 0 },
    i2c_device_id { name: ptr::null(), driver_data: 0 },
];
/* MODULE_DEVICE_TABLE(i2c, sma1307_i2c_id); */

static sma1307_of_match: [of_device_id; 3] = [
    of_device_id { compatible: c"irondevice,sma1307a".as_ptr(), data: ptr::null() },
    of_device_id { compatible: c"irondevice,sma1307aq".as_ptr(), data: &sma1307aq_data as *const _ as *const c_void }, //AEC-Q100 Qualificated
    of_device_id { compatible: ptr::null(), data: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, sma1307_of_match); */

static mut sma1307_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver { name: c"sma1307".as_ptr(), of_match_table: sma1307_of_match.as_ptr() },
    probe: Some(sma1307_i2c_probe),
    remove: Some(sma1307_i2c_remove),
    id_table: sma1307_i2c_id.as_ptr(),
};

/* module_i2c_driver(sma1307_i2c_driver); */
/* MODULE_DESCRIPTION("ALSA SoC SMA1307 driver"); */
/* MODULE_AUTHOR("Gyuhwa Park, <gyuhwa.park@irondevice.com>"); */
/* MODULE_AUTHOR("KS Jo, <kiseok.jo@irondevice.com>"); */
/* MODULE_LICENSE("GPL"); */

type c_long = isize;
type sma1307_mode = c_uint;

#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct device { pub kobj: kobject }
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { pub work: work_struct }
#[repr(C)] pub struct firmware { pub size: usize, pub data: *const u8 }
#[repr(C)] pub struct reg_default { pub reg: c_uint, pub def: c_uint }
#[repr(C)] pub struct soc_enum { _private: [u8; 0] }
#[repr(C)] pub struct soc_mixer_control { pub min: c_int, pub max: c_int }
#[repr(C)] pub struct snd_ctl_elem_id { pub name: *const c_char }
#[repr(C)] pub struct snd_kcontrol { pub id: snd_ctl_elem_id, pub private_value: c_ulong }
#[repr(C)] pub struct snd_soc_card { pub snd_card: *mut c_void }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device, pub card: *mut snd_soc_card }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { pub name: *const c_char, pub dapm: *mut snd_soc_dapm_context }
#[repr(C)] pub struct snd_pcm_substream { pub stream: c_int }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai { pub component: *mut snd_soc_component }
#[repr(C)] pub struct snd_kcontrol_new { pub iface: c_uint, pub name: *const c_char, pub info: Option<unsafe extern "C" fn()>, pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, pub private_value: c_ulong }
#[repr(C)] pub struct snd_soc_dapm_widget_list { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_route { pub sink: *const c_char, pub control: *const c_char, pub source: *const c_char }
#[repr(C)] pub struct snd_soc_dai_ops { pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>, pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>, pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>, pub set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>, pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int> }
#[repr(C)] pub struct snd_soc_pcm_stream { pub stream_name: *const c_char, pub channels_min: c_uint, pub channels_max: c_uint, pub rates: c_uint, pub formats: c_uint }
#[repr(C)] pub struct snd_soc_dai_driver { pub name: *const c_char, pub id: c_int, pub playback: snd_soc_pcm_stream, pub capture: snd_soc_pcm_stream, pub ops: *const snd_soc_dai_ops }
#[repr(C)] pub struct snd_soc_component_driver { pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>, pub controls: *const snd_kcontrol_new, pub num_controls: c_uint, pub dapm_widgets: *const snd_soc_dapm_widget, pub num_dapm_widgets: c_uint, pub dapm_routes: *const snd_soc_dapm_route, pub num_dapm_routes: c_uint }
#[repr(C)] pub struct regmap_config { pub reg_bits: c_uint, pub val_bits: c_uint, pub max_register: c_uint, pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>, pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>, pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>, pub reg_defaults: *const reg_default, pub num_reg_defaults: c_uint }
#[repr(C)] pub struct i2c_client { pub dev: device, pub name: *mut c_char }
#[repr(C)] pub struct i2c_device_id { pub name: *const c_char, pub driver_data: c_ulong }
#[repr(C)] pub struct of_device_id { pub compatible: *const c_char, pub data: *const c_void }
#[repr(C)] pub struct device_driver { pub name: *const c_char, pub of_match_table: *const of_device_id }
#[repr(C)] pub struct i2c_driver { pub driver: device_driver, pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>, pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>, pub id_table: *const i2c_device_id }

#[repr(C)] pub struct sma1307_setting_file { pub status: bool, pub header_size: c_int, pub checksum: c_int, pub num_mode: c_int, pub header: *mut c_int, pub def_size: c_int, pub def: *mut c_int, pub mode_size: c_int, pub mode_set: [*mut c_int; 5] }

#[repr(C)] pub union snd_ctl_elem_value_value { pub integer: snd_ctl_elem_value_integer, pub enumerated: snd_ctl_elem_value_enumerated }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }
#[repr(C)] pub struct snd_ctl_elem_value_integer { pub value: [c_long; 128] }
#[repr(C)] pub struct snd_ctl_elem_value_enumerated { pub item: [c_uint; 128] }

const fn route(sink: *const c_char, control: *const c_char, source: *const c_char) -> snd_soc_dapm_route { snd_soc_dapm_route { sink, control, source } }
const fn declare_tlv_db_scale(_min: c_int, _step: c_int, _mute: c_int) -> [c_uint; 4] { [0; 4] }
const fn SOC_ENUM_SINGLE_EXT(_items: usize, _texts: *const *const c_char) -> soc_enum { soc_enum { _private: [] } }

extern "C" {
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_crit(dev: *mut device, fmt: *const c_char, ...);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_ctl_notify(card: *mut c_void, mask: c_uint, id: *mut snd_ctl_elem_id);
    fn snd_soc_put_enum_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_get_enum_double() ;
    fn snd_soc_info_enum_double();
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_dapm_kcontrol_to_dapm(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_put_enum_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_dapm_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_sync(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_soc_add_component_controls(component: *mut snd_soc_component, controls: *const snd_kcontrol_new, num: c_uint) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_physical_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_uint;
    fn queue_delayed_work(wq: *mut c_void, work: *mut delayed_work, delay: c_ulong) -> bool;
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;
    fn msleep(msecs: c_uint);
    fn kobject_uevent_env(kobj: *mut kobject, action: c_int, envp: *mut *mut c_char) -> c_int;
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, device: *mut device) -> c_int;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kmalloc_array(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kfree(dev: *mut device, p: *mut c_void);
    fn ERR_PTR(error: c_int) -> *mut c_void;
    fn DIV_ROUND_CLOSEST(x: c_int, divisor: c_int) -> c_int;
    fn mutex_init(lock: *mut mutex);
    fn INIT_DELAYED_WORK(work: *mut delayed_work, func: Option<unsafe extern "C" fn(*mut work_struct)>);
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn device_get_match_data(dev: *mut device) -> *const c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut c_void;
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn container_of_check_fault_work(work: *mut work_struct) -> *mut sma1307_priv;
    fn SOC_SINGLE_EXT(name: *const c_char, reg: c_uint, shift: c_uint, max: c_uint, invert: c_uint, get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>) -> snd_kcontrol_new;
    fn SOC_DAPM_SINGLE(name: *const c_char, reg: c_uint, shift: c_uint, max: c_uint, invert: c_uint) -> snd_kcontrol_new;
    fn SOC_ENUM_EXT(name: *const c_char, e: *const soc_enum, get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>) -> snd_kcontrol_new;
    fn SOC_SINGLE_TLV(name: *const c_char, reg: c_uint, shift: c_uint, max: c_uint, invert: c_uint, tlv: *const c_uint) -> snd_kcontrol_new;
    fn SOC_SINGLE_BOOL_EXT(name: *const c_char, xdata: c_uint, get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>) -> snd_kcontrol_new;
    fn SND_SOC_DAPM_OUTPUT(name: *const c_char) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_INPUT(name: *const c_char) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_MUX_E(name: *const c_char, reg: c_uint, shift: c_uint, invert: c_uint, kcontrol: *const snd_kcontrol_new, event: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>, flags: c_int) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_SWITCH_E(name: *const c_char, reg: c_uint, shift: c_uint, invert: c_uint, kcontrol: *const snd_kcontrol_new, event: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>, flags: c_int) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_MIXER(name: *const c_char, reg: c_uint, shift: c_uint, invert: c_uint, controls: *const snd_kcontrol_new, num_controls: c_uint) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_OUT_DRV_E(name: *const c_char, reg: c_uint, shift: c_uint, invert: c_uint, controls: *const snd_kcontrol_new, num_controls: c_uint, event: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>, flags: c_int) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_SWITCH(name: *const c_char, reg: c_uint, shift: c_uint, invert: c_uint, kcontrol: *const snd_kcontrol_new) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_AIF_IN(name: *const c_char, stream: *const c_char, slot: c_uint, reg: c_uint, shift: c_uint, invert: c_uint) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_AIF_OUT(name: *const c_char, stream: *const c_char, slot: c_uint, reg: c_uint, shift: c_uint, invert: c_uint) -> snd_soc_dapm_widget;
}

extern "C" {
    static mut system_freezable_wq: *mut c_void;
}


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
