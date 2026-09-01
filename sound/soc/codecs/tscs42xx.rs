// SPDX-License-Identifier: GPL-2.0
// tscs42xx.c -- TSCS42xx ALSA SoC Audio driver
// Copyright 2017 Tempo Semiconductor, Inc.
// Author: Steven Eckhoff <steven.eckhoff.opensource@gmail.com>

// Translated from soc/codecs/tscs42xx.c.  Linux/ALSA includes and
// "tscs42xx.h" are external dependencies supplied by the surrounding tree.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

const COEFF_SIZE: usize = 3;
const BIQUAD_COEFF_COUNT: usize = 5;
const BIQUAD_SIZE: usize = COEFF_SIZE * BIQUAD_COEFF_COUNT;

const COEFF_RAM_MAX_ADDR: usize = 0xcd;
const COEFF_RAM_COEFF_COUNT: usize = COEFF_RAM_MAX_ADDR + 1;
const COEFF_RAM_SIZE: usize = COEFF_SIZE * COEFF_RAM_COEFF_COUNT;

const MAX_PLL_LOCK_20MS_WAITS: c_int = 1;
const DACCRSTAT_MAX_TRYS: c_int = 10;
const PLL_REG_SETTINGS_COUNT: usize = 13;

#[repr(C)]
struct tscs42xx {
    bclk_ratio: c_int,
    samplerate: c_int,
    audio_params_lock: mutex,

    coeff_ram: [u8; COEFF_RAM_SIZE],
    coeff_ram_synced: bool,
    coeff_ram_lock: mutex,

    pll_lock: mutex,

    regmap: *mut regmap,

    sysclk: *mut clk,
    sysclk_src_id: c_int,
}

#[repr(C)]
struct coeff_ram_ctl {
    addr: c_uint,
    bytes_ext: soc_bytes_ext,
}

#[repr(C)]
struct reg_setting {
    addr: c_uint,
    val: c_uint,
    mask: c_uint,
}

#[repr(C)]
struct pll_ctl {
    input_freq: c_int,
    settings: [reg_setting; PLL_REG_SETTINGS_COUNT],
}

extern "C" {
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_bulk_write(map: *mut regmap, reg: c_uint, val: c_uint, buf: *const c_void, count: usize) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_register_patch(map: *mut regmap, regs: *const reg_sequence, num_regs: c_int) -> c_int;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn mutex_init(lock: *mut mutex);
    fn msleep(msecs: c_uint);
    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

unsafe fn tscs42xx_volatile(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        R_DACCRWRL | R_DACCRWRM | R_DACCRWRH | R_DACCRRDL | R_DACCRRDM | R_DACCRRDH
        | R_DACCRSTAT | R_DACCRADDR | R_PLLCTL0 => true,
        _ => false,
    }
}

unsafe fn tscs42xx_precious(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        R_DACCRWRL | R_DACCRWRM | R_DACCRWRH | R_DACCRRDL | R_DACCRRDM | R_DACCRRDH => true,
        _ => false,
    }
}

static tscs42xx_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    volatile_reg: Some(tscs42xx_volatile),
    precious_reg: Some(tscs42xx_precious),
    max_register: R_DACMBCREL3H,
    cache_type: REGCACHE_RBTREE,
    can_multi_write: true,
};

unsafe fn plls_locked(component: *mut snd_soc_component) -> bool {
    let mut count: c_int = MAX_PLL_LOCK_20MS_WAITS;
    loop {
        let ret = snd_soc_component_read(component, R_PLLCTL0);
        if ret < 0 {
            dev_err!((*component).dev, "Failed to read PLL lock status (%d)\n", ret);
            return false;
        } else if ret > 0 {
            return true;
        }
        msleep(20);
        let old = count;
        count -= 1;
        if old == 0 {
            break;
        }
    }
    false
}

fn sample_rate_to_pll_freq_out(sample_rate: c_int) -> c_int {
    match sample_rate {
        11025 | 22050 | 44100 | 88200 => 112896000,
        8000 | 16000 | 32000 | 48000 | 96000 => 122880000,
        _ => -EINVAL,
    }
}

unsafe fn write_coeff_ram(
    component: *mut snd_soc_component,
    coeff_ram: *mut u8,
    mut addr: c_uint,
    coeff_cnt: c_uint,
) -> c_int {
    let tscs42xx = snd_soc_component_get_drvdata(component) as *mut tscs42xx;
    let mut cnt: c_uint = 0;

    while cnt < coeff_cnt {
        let mut trys: c_int = 0;
        while trys < DACCRSTAT_MAX_TRYS {
            let ret = snd_soc_component_read(component, R_DACCRSTAT);
            if ret < 0 {
                dev_err!((*component).dev, "Failed to read stat (%d)\n", ret);
                return ret;
            }
            if ret == 0 {
                break;
            }
            trys += 1;
        }

        if trys == DACCRSTAT_MAX_TRYS {
            let ret = -EIO;
            dev_err!((*component).dev, "dac coefficient write error (%d)\n", ret);
            return ret;
        }

        let mut ret = regmap_write((*tscs42xx).regmap, R_DACCRADDR, addr);
        if ret < 0 {
            dev_err!((*component).dev, "Failed to write dac ram address (%d)\n", ret);
            return ret;
        }

        ret = regmap_bulk_write(
            (*tscs42xx).regmap,
            R_DACCRWRL,
            coeff_ram.add(addr as usize * COEFF_SIZE) as *const c_void,
            COEFF_SIZE,
        );
        if ret < 0 {
            dev_err!((*component).dev, "Failed to write dac ram (%d)\n", ret);
            return ret;
        }

        cnt += 1;
        addr += 1;
    }

    0
}

unsafe fn power_up_audio_plls(component: *mut snd_soc_component) -> c_int {
    let tscs42xx = snd_soc_component_get_drvdata(component) as *mut tscs42xx;
    let (mask, val) = match sample_rate_to_pll_freq_out((*tscs42xx).samplerate) {
        122880000 => (RM_PLLCTL1C_PDB_PLL1, RV_PLLCTL1C_PDB_PLL1_ENABLE), // 48k
        112896000 => (RM_PLLCTL1C_PDB_PLL2, RV_PLLCTL1C_PDB_PLL2_ENABLE), // 44.1k
        _ => {
            let ret = -EINVAL;
            dev_err!((*component).dev, "Unrecognized PLL output freq (%d)\n", ret);
            return ret;
        }
    };

    let _guard = mutex_guard(&mut (*tscs42xx).pll_lock);

    let ret = snd_soc_component_update_bits(component, R_PLLCTL1C, mask, val);
    if ret < 0 {
        dev_err!((*component).dev, "Failed to turn PLL on (%d)\n", ret);
        return ret;
    }

    if !plls_locked(component) {
        dev_err!((*component).dev, "Failed to lock plls\n");
        return -ENOMSG;
    }

    0
}

unsafe fn power_down_audio_plls(component: *mut snd_soc_component) -> c_int {
    let tscs42xx = snd_soc_component_get_drvdata(component) as *mut tscs42xx;
    let _guard = mutex_guard(&mut (*tscs42xx).pll_lock);

    let mut ret = snd_soc_component_update_bits(
        component,
        R_PLLCTL1C,
        RM_PLLCTL1C_PDB_PLL1,
        RV_PLLCTL1C_PDB_PLL1_DISABLE,
    );
    if ret < 0 {
        dev_err!((*component).dev, "Failed to turn PLL off (%d)\n", ret);
        return ret;
    }
    ret = snd_soc_component_update_bits(
        component,
        R_PLLCTL1C,
        RM_PLLCTL1C_PDB_PLL2,
        RV_PLLCTL1C_PDB_PLL2_DISABLE,
    );
    if ret < 0 {
        dev_err!((*component).dev, "Failed to turn PLL off (%d)\n", ret);
        return ret;
    }

    0
}

unsafe fn coeff_ram_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let tscs42xx = snd_soc_component_get_drvdata(component) as *mut tscs42xx;
    let ctl = (*kcontrol).private_value as *mut coeff_ram_ctl;
    let params = &mut (*ctl).bytes_ext as *mut soc_bytes_ext;

    let _guard = mutex_guard(&mut (*tscs42xx).coeff_ram_lock);

    ptr::copy_nonoverlapping(
        (*tscs42xx).coeff_ram.as_ptr().add((*ctl).addr as usize * COEFF_SIZE),
        (*ucontrol).value.bytes.data.as_mut_ptr(),
        (*params).max as usize,
    );

    0
}

unsafe fn coeff_ram_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let tscs42xx = snd_soc_component_get_drvdata(component) as *mut tscs42xx;
    let ctl = (*kcontrol).private_value as *mut coeff_ram_ctl;
    let params = &mut (*ctl).bytes_ext as *mut soc_bytes_ext;
    let coeff_cnt = (*params).max / COEFF_SIZE as c_uint;

    let _coeff_guard = mutex_guard(&mut (*tscs42xx).coeff_ram_lock);

    (*tscs42xx).coeff_ram_synced = false;
    ptr::copy_nonoverlapping(
        (*ucontrol).value.bytes.data.as_ptr(),
        (*tscs42xx).coeff_ram.as_mut_ptr().add((*ctl).addr as usize * COEFF_SIZE),
        (*params).max as usize,
    );

    let _pll_guard = mutex_guard(&mut (*tscs42xx).pll_lock);
    if plls_locked(component) {
        let ret = write_coeff_ram(component, (*tscs42xx).coeff_ram.as_mut_ptr(), (*ctl).addr, coeff_cnt);
        if ret < 0 {
            dev_err!((*component).dev, "Failed to flush coeff ram cache (%d)\n", ret);
            return ret;
        }
        (*tscs42xx).coeff_ram_synced = true;
    }

    0
}

/* Input L Capture Route */
static input_select_text: [&[u8]; 4] = [b"Line 1\0", b"Line 2\0", b"Line 3\0", b"D2S\0"];
static left_input_select_enum: soc_enum =
    SOC_ENUM_SINGLE!(R_INSELL, FB_INSELL, input_select_text.len(), input_select_text);
static left_input_select: snd_kcontrol_new =
    SOC_DAPM_ENUM!(c"LEFT_INPUT_SELECT_ENUM", left_input_select_enum);

/* Input R Capture Route */
static right_input_select_enum: soc_enum =
    SOC_ENUM_SINGLE!(R_INSELR, FB_INSELR, input_select_text.len(), input_select_text);
static right_input_select: snd_kcontrol_new =
    SOC_DAPM_ENUM!(c"RIGHT_INPUT_SELECT_ENUM", right_input_select_enum);

/* Input Channel Mapping */
static ch_map_select_text: [&[u8]; 4] =
    [b"Normal\0", b"Left to Right\0", b"Right to Left\0", b"Swap\0"];
static ch_map_select_enum: soc_enum =
    SOC_ENUM_SINGLE!(R_AIC2, FB_AIC2_ADCDSEL, ch_map_select_text.len(), ch_map_select_text);

unsafe fn dapm_vref_event(_w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, _event: c_int) -> c_int {
    msleep(20);
    0
}

unsafe fn dapm_micb_event(_w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, _event: c_int) -> c_int {
    msleep(20);
    0
}

unsafe fn pll_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    if SND_SOC_DAPM_EVENT_ON!(event) {
        power_up_audio_plls(component)
    } else {
        power_down_audio_plls(component)
    }
}

unsafe fn dac_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, _event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let tscs42xx = snd_soc_component_get_drvdata(component) as *mut tscs42xx;
    let _guard = mutex_guard(&mut (*tscs42xx).coeff_ram_lock);

    if !(*tscs42xx).coeff_ram_synced {
        let ret = write_coeff_ram(component, (*tscs42xx).coeff_ram.as_mut_ptr(), 0x00, COEFF_RAM_COEFF_COUNT as c_uint);
        if ret < 0 {
            return ret;
        }
        (*tscs42xx).coeff_ram_synced = true;
    }

    0
}

static tscs42xx_dapm_widgets: &[snd_soc_dapm_widget] = &[
    /* Vref */
    SND_SOC_DAPM_SUPPLY_S!(c"Vref", 1, R_PWRM2, FB_PWRM2_VREF, 0, dapm_vref_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    /* PLL */
    SND_SOC_DAPM_SUPPLY!(c"PLL", SND_SOC_NOPM, 0, 0, pll_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    /* Headphone */
    SND_SOC_DAPM_DAC_E!(c"DAC L", c"HiFi Playback", R_PWRM2, FB_PWRM2_HPL, 0, dac_event, SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_DAC_E!(c"DAC R", c"HiFi Playback", R_PWRM2, FB_PWRM2_HPR, 0, dac_event, SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_OUTPUT!(c"Headphone L"),
    SND_SOC_DAPM_OUTPUT!(c"Headphone R"),
    /* Speaker */
    SND_SOC_DAPM_DAC_E!(c"ClassD L", c"HiFi Playback", R_PWRM2, FB_PWRM2_SPKL, 0, dac_event, SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_DAC_E!(c"ClassD R", c"HiFi Playback", R_PWRM2, FB_PWRM2_SPKR, 0, dac_event, SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_OUTPUT!(c"Speaker L"),
    SND_SOC_DAPM_OUTPUT!(c"Speaker R"),
    /* Capture */
    SND_SOC_DAPM_PGA!(c"Analog In PGA L", R_PWRM1, FB_PWRM1_PGAL, 0, ptr::null(), 0),
    SND_SOC_DAPM_PGA!(c"Analog In PGA R", R_PWRM1, FB_PWRM1_PGAR, 0, ptr::null(), 0),
    SND_SOC_DAPM_PGA!(c"Analog Boost L", R_PWRM1, FB_PWRM1_BSTL, 0, ptr::null(), 0),
    SND_SOC_DAPM_PGA!(c"Analog Boost R", R_PWRM1, FB_PWRM1_BSTR, 0, ptr::null(), 0),
    SND_SOC_DAPM_PGA!(c"ADC Mute", R_CNVRTR0, FB_CNVRTR0_HPOR, true, ptr::null(), 0),
    SND_SOC_DAPM_ADC!(c"ADC L", c"HiFi Capture", R_PWRM1, FB_PWRM1_ADCL, 0),
    SND_SOC_DAPM_ADC!(c"ADC R", c"HiFi Capture", R_PWRM1, FB_PWRM1_ADCR, 0),
    /* Capture Input */
    SND_SOC_DAPM_MUX!(c"Input L Capture Route", R_PWRM2, FB_PWRM2_INSELL, 0, &left_input_select),
    SND_SOC_DAPM_MUX!(c"Input R Capture Route", R_PWRM2, FB_PWRM2_INSELR, 0, &right_input_select),
    /* Digital Mic */
    SND_SOC_DAPM_SUPPLY_S!(c"Digital Mic Enable", 2, R_DMICCTL, FB_DMICCTL_DMICEN, 0, ptr::null(), SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    /* Analog Mic */
    SND_SOC_DAPM_SUPPLY_S!(c"Mic Bias", 2, R_PWRM1, FB_PWRM1_MICB, 0, dapm_micb_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    /* Line In */
    SND_SOC_DAPM_INPUT!(c"Line In 1 L"),
    SND_SOC_DAPM_INPUT!(c"Line In 1 R"),
    SND_SOC_DAPM_INPUT!(c"Line In 2 L"),
    SND_SOC_DAPM_INPUT!(c"Line In 2 R"),
    SND_SOC_DAPM_INPUT!(c"Line In 3 L"),
    SND_SOC_DAPM_INPUT!(c"Line In 3 R"),
];

static tscs42xx_intercon: &[snd_soc_dapm_route] = &[
    route!(c"DAC L", ptr::null(), c"PLL"), route!(c"DAC R", ptr::null(), c"PLL"),
    route!(c"DAC L", ptr::null(), c"Vref"), route!(c"DAC R", ptr::null(), c"Vref"),
    route!(c"Headphone L", ptr::null(), c"DAC L"), route!(c"Headphone R", ptr::null(), c"DAC R"),
    route!(c"ClassD L", ptr::null(), c"PLL"), route!(c"ClassD R", ptr::null(), c"PLL"),
    route!(c"ClassD L", ptr::null(), c"Vref"), route!(c"ClassD R", ptr::null(), c"Vref"),
    route!(c"Speaker L", ptr::null(), c"ClassD L"), route!(c"Speaker R", ptr::null(), c"ClassD R"),
    route!(c"Input L Capture Route", ptr::null(), c"Vref"), route!(c"Input R Capture Route", ptr::null(), c"Vref"),
    route!(c"Mic Bias", ptr::null(), c"Vref"),
    route!(c"Input L Capture Route", c"Line 1", c"Line In 1 L"), route!(c"Input R Capture Route", c"Line 1", c"Line In 1 R"),
    route!(c"Input L Capture Route", c"Line 2", c"Line In 2 L"), route!(c"Input R Capture Route", c"Line 2", c"Line In 2 R"),
    route!(c"Input L Capture Route", c"Line 3", c"Line In 3 L"), route!(c"Input R Capture Route", c"Line 3", c"Line In 3 R"),
    route!(c"Analog In PGA L", ptr::null(), c"Input L Capture Route"), route!(c"Analog In PGA R", ptr::null(), c"Input R Capture Route"),
    route!(c"Analog Boost L", ptr::null(), c"Analog In PGA L"), route!(c"Analog Boost R", ptr::null(), c"Analog In PGA R"),
    route!(c"ADC Mute", ptr::null(), c"Analog Boost L"), route!(c"ADC Mute", ptr::null(), c"Analog Boost R"),
    route!(c"ADC L", ptr::null(), c"PLL"), route!(c"ADC R", ptr::null(), c"PLL"),
    route!(c"ADC L", ptr::null(), c"ADC Mute"), route!(c"ADC R", ptr::null(), c"ADC Mute"),
];

/************
 * CONTROLS *
 ************/

static eq_band_enable_text: [&[u8]; 7] = [
    b"Prescale only\0", b"Band1\0", b"Band1:2\0", b"Band1:3\0",
    b"Band1:4\0", b"Band1:5\0", b"Band1:6\0",
];
static level_detection_text: [&[u8]; 2] = [b"Average\0", b"Peak\0"];
static level_detection_window_text: [&[u8]; 2] = [b"512 Samples\0", b"64 Samples\0"];
static compressor_ratio_text: [&[u8]; 21] = [
    b"Reserved\0", b"1.5:1\0", b"2:1\0", b"3:1\0", b"4:1\0", b"5:1\0", b"6:1\0",
    b"7:1\0", b"8:1\0", b"9:1\0", b"10:1\0", b"11:1\0", b"12:1\0", b"13:1\0",
    b"14:1\0", b"15:1\0", b"16:1\0", b"17:1\0", b"18:1\0", b"19:1\0", b"20:1\0",
];

DECLARE_TLV_DB_SCALE!(hpvol_scale, -8850, 75, 0);
DECLARE_TLV_DB_SCALE!(spkvol_scale, -7725, 75, 0);
DECLARE_TLV_DB_SCALE!(dacvol_scale, -9563, 38, 0);
DECLARE_TLV_DB_SCALE!(adcvol_scale, -7125, 38, 0);
DECLARE_TLV_DB_SCALE!(invol_scale, -1725, 75, 0);
DECLARE_TLV_DB_SCALE!(mic_boost_scale, 0, 1000, 0);
DECLARE_TLV_DB_MINMAX!(mugain_scale, 0, 4650);
DECLARE_TLV_DB_MINMAX!(compth_scale, -9562, 0);

static eq1_band_enable_enum: soc_enum =
    SOC_ENUM_SINGLE!(R_CONFIG1, FB_CONFIG1_EQ1_BE, eq_band_enable_text.len(), eq_band_enable_text);
static eq2_band_enable_enum: soc_enum =
    SOC_ENUM_SINGLE!(R_CONFIG1, FB_CONFIG1_EQ2_BE, eq_band_enable_text.len(), eq_band_enable_text);
static cle_level_detection_enum: soc_enum =
    SOC_ENUM_SINGLE!(R_CLECTL, FB_CLECTL_LVL_MODE, level_detection_text.len(), level_detection_text);
static cle_level_detection_window_enum: soc_enum =
    SOC_ENUM_SINGLE!(R_CLECTL, FB_CLECTL_WINDOWSEL, level_detection_window_text.len(), level_detection_window_text);
static mbc_level_detection_enums: [soc_enum; 3] = [
    SOC_ENUM_SINGLE!(R_DACMBCCTL, FB_DACMBCCTL_LVLMODE1, level_detection_text.len(), level_detection_text),
    SOC_ENUM_SINGLE!(R_DACMBCCTL, FB_DACMBCCTL_LVLMODE2, level_detection_text.len(), level_detection_text),
    SOC_ENUM_SINGLE!(R_DACMBCCTL, FB_DACMBCCTL_LVLMODE3, level_detection_text.len(), level_detection_text),
];
static mbc_level_detection_window_enums: [soc_enum; 3] = [
    SOC_ENUM_SINGLE!(R_DACMBCCTL, FB_DACMBCCTL_WINSEL1, level_detection_window_text.len(), level_detection_window_text),
    SOC_ENUM_SINGLE!(R_DACMBCCTL, FB_DACMBCCTL_WINSEL2, level_detection_window_text.len(), level_detection_window_text),
    SOC_ENUM_SINGLE!(R_DACMBCCTL, FB_DACMBCCTL_WINSEL3, level_detection_window_text.len(), level_detection_window_text),
];
static compressor_ratio_enum: soc_enum =
    SOC_ENUM_SINGLE!(R_CMPRAT, FB_CMPRAT, compressor_ratio_text.len(), compressor_ratio_text);
static dac_mbc1_compressor_ratio_enum: soc_enum =
    SOC_ENUM_SINGLE!(R_DACMBCRAT1, FB_DACMBCRAT1_RATIO, compressor_ratio_text.len(), compressor_ratio_text);
static dac_mbc2_compressor_ratio_enum: soc_enum =
    SOC_ENUM_SINGLE!(R_DACMBCRAT2, FB_DACMBCRAT2_RATIO, compressor_ratio_text.len(), compressor_ratio_text);
static dac_mbc3_compressor_ratio_enum: soc_enum =
    SOC_ENUM_SINGLE!(R_DACMBCRAT3, FB_DACMBCRAT3_RATIO, compressor_ratio_text.len(), compressor_ratio_text);

unsafe fn bytes_info_ext(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_info) -> c_int {
    let ctl = (*kcontrol).private_value as *mut coeff_ram_ctl;
    let params = &mut (*ctl).bytes_ext as *mut soc_bytes_ext;
    (*ucontrol).type_ = SNDRV_CTL_ELEM_TYPE_BYTES;
    (*ucontrol).count = (*params).max;
    0
}

macro_rules! COEFF_RAM_CTL {
    ($xname:expr, $xcount:expr, $xaddr:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: $xname.as_ptr() as *const c_char,
            info: Some(bytes_info_ext),
            get: Some(coeff_ram_get),
            put: Some(coeff_ram_put),
            private_value: &coeff_ram_ctl { addr: $xaddr, bytes_ext: soc_bytes_ext { max: $xcount as c_uint } }
                as *const coeff_ram_ctl as c_ulong,
        }
    };
}

static tscs42xx_snd_controls: &[snd_kcontrol_new] = &[
    /* Volumes */
    SOC_DOUBLE_R_TLV!(c"Headphone Volume", R_HPVOLL, R_HPVOLR, FB_HPVOLL, 0x7F, 0, hpvol_scale),
    SOC_DOUBLE_R_TLV!(c"Speaker Volume", R_SPKVOLL, R_SPKVOLR, FB_SPKVOLL, 0x7F, 0, spkvol_scale),
    SOC_DOUBLE_R_TLV!(c"Master Volume", R_DACVOLL, R_DACVOLR, FB_DACVOLL, 0xFF, 0, dacvol_scale),
    SOC_DOUBLE_R_TLV!(c"PCM Volume", R_ADCVOLL, R_ADCVOLR, FB_ADCVOLL, 0xFF, 0, adcvol_scale),
    SOC_DOUBLE_R_TLV!(c"Input Volume", R_INVOLL, R_INVOLR, FB_INVOLL, 0x3F, 0, invol_scale),
    /* INSEL */
    SOC_DOUBLE_R_TLV!(c"Mic Boost Volume", R_INSELL, R_INSELR, FB_INSELL_MICBSTL, FV_INSELL_MICBSTL_30DB, 0, mic_boost_scale),
    /* Input Channel Map */
    SOC_ENUM!(c"Input Channel Map", ch_map_select_enum),
    /* Mic Bias */
    SOC_SINGLE!(c"Mic Bias Boost Switch", 0x71, 0x07, 1, 0),
    /* Headphone Auto Switching */
    SOC_SINGLE!(c"Headphone Auto Switching Switch", R_CTL, FB_CTL_HPSWEN, 1, 0),
    SOC_SINGLE!(c"Headphone Detect Polarity Toggle Switch", R_CTL, FB_CTL_HPSWPOL, 1, 0),
    /* Coefficient Ram */
    COEFF_RAM_CTL!(c"Cascade1L BiQuad1", BIQUAD_SIZE, 0x00), COEFF_RAM_CTL!(c"Cascade1L BiQuad2", BIQUAD_SIZE, 0x05),
    COEFF_RAM_CTL!(c"Cascade1L BiQuad3", BIQUAD_SIZE, 0x0a), COEFF_RAM_CTL!(c"Cascade1L BiQuad4", BIQUAD_SIZE, 0x0f),
    COEFF_RAM_CTL!(c"Cascade1L BiQuad5", BIQUAD_SIZE, 0x14), COEFF_RAM_CTL!(c"Cascade1L BiQuad6", BIQUAD_SIZE, 0x19),
    COEFF_RAM_CTL!(c"Cascade1R BiQuad1", BIQUAD_SIZE, 0x20), COEFF_RAM_CTL!(c"Cascade1R BiQuad2", BIQUAD_SIZE, 0x25),
    COEFF_RAM_CTL!(c"Cascade1R BiQuad3", BIQUAD_SIZE, 0x2a), COEFF_RAM_CTL!(c"Cascade1R BiQuad4", BIQUAD_SIZE, 0x2f),
    COEFF_RAM_CTL!(c"Cascade1R BiQuad5", BIQUAD_SIZE, 0x34), COEFF_RAM_CTL!(c"Cascade1R BiQuad6", BIQUAD_SIZE, 0x39),
    COEFF_RAM_CTL!(c"Cascade1L Prescale", COEFF_SIZE, 0x1f), COEFF_RAM_CTL!(c"Cascade1R Prescale", COEFF_SIZE, 0x3f),
    COEFF_RAM_CTL!(c"Cascade2L BiQuad1", BIQUAD_SIZE, 0x40), COEFF_RAM_CTL!(c"Cascade2L BiQuad2", BIQUAD_SIZE, 0x45),
    COEFF_RAM_CTL!(c"Cascade2L BiQuad3", BIQUAD_SIZE, 0x4a), COEFF_RAM_CTL!(c"Cascade2L BiQuad4", BIQUAD_SIZE, 0x4f),
    COEFF_RAM_CTL!(c"Cascade2L BiQuad5", BIQUAD_SIZE, 0x54), COEFF_RAM_CTL!(c"Cascade2L BiQuad6", BIQUAD_SIZE, 0x59),
    COEFF_RAM_CTL!(c"Cascade2R BiQuad1", BIQUAD_SIZE, 0x60), COEFF_RAM_CTL!(c"Cascade2R BiQuad2", BIQUAD_SIZE, 0x65),
    COEFF_RAM_CTL!(c"Cascade2R BiQuad3", BIQUAD_SIZE, 0x6a), COEFF_RAM_CTL!(c"Cascade2R BiQuad4", BIQUAD_SIZE, 0x6f),
    COEFF_RAM_CTL!(c"Cascade2R BiQuad5", BIQUAD_SIZE, 0x74), COEFF_RAM_CTL!(c"Cascade2R BiQuad6", BIQUAD_SIZE, 0x79),
    COEFF_RAM_CTL!(c"Cascade2L Prescale", COEFF_SIZE, 0x5f), COEFF_RAM_CTL!(c"Cascade2R Prescale", COEFF_SIZE, 0x7f),
    COEFF_RAM_CTL!(c"Bass Extraction BiQuad1", BIQUAD_SIZE, 0x80), COEFF_RAM_CTL!(c"Bass Extraction BiQuad2", BIQUAD_SIZE, 0x85),
    COEFF_RAM_CTL!(c"Bass Non Linear Function 1", COEFF_SIZE, 0x8a), COEFF_RAM_CTL!(c"Bass Non Linear Function 2", COEFF_SIZE, 0x8b),
    COEFF_RAM_CTL!(c"Bass Limiter BiQuad", BIQUAD_SIZE, 0x8c), COEFF_RAM_CTL!(c"Bass Cut Off BiQuad", BIQUAD_SIZE, 0x91),
    COEFF_RAM_CTL!(c"Bass Mix", COEFF_SIZE, 0x96), COEFF_RAM_CTL!(c"Treb Extraction BiQuad1", BIQUAD_SIZE, 0x97),
    COEFF_RAM_CTL!(c"Treb Extraction BiQuad2", BIQUAD_SIZE, 0x9c), COEFF_RAM_CTL!(c"Treb Non Linear Function 1", COEFF_SIZE, 0xa1),
    COEFF_RAM_CTL!(c"Treb Non Linear Function 2", COEFF_SIZE, 0xa2), COEFF_RAM_CTL!(c"Treb Limiter BiQuad", BIQUAD_SIZE, 0xa3),
    COEFF_RAM_CTL!(c"Treb Cut Off BiQuad", BIQUAD_SIZE, 0xa8), COEFF_RAM_CTL!(c"Treb Mix", COEFF_SIZE, 0xad),
    COEFF_RAM_CTL!(c"3D", COEFF_SIZE, 0xae), COEFF_RAM_CTL!(c"3D Mix", COEFF_SIZE, 0xaf),
    COEFF_RAM_CTL!(c"MBC1 BiQuad1", BIQUAD_SIZE, 0xb0), COEFF_RAM_CTL!(c"MBC1 BiQuad2", BIQUAD_SIZE, 0xb5),
    COEFF_RAM_CTL!(c"MBC2 BiQuad1", BIQUAD_SIZE, 0xba), COEFF_RAM_CTL!(c"MBC2 BiQuad2", BIQUAD_SIZE, 0xbf),
    COEFF_RAM_CTL!(c"MBC3 BiQuad1", BIQUAD_SIZE, 0xc4), COEFF_RAM_CTL!(c"MBC3 BiQuad2", BIQUAD_SIZE, 0xc9),
    /* EQ */
    SOC_SINGLE!(c"EQ1 Switch", R_CONFIG1, FB_CONFIG1_EQ1_EN, 1, 0), SOC_SINGLE!(c"EQ2 Switch", R_CONFIG1, FB_CONFIG1_EQ2_EN, 1, 0),
    SOC_ENUM!(c"EQ1 Band Enable", eq1_band_enable_enum), SOC_ENUM!(c"EQ2 Band Enable", eq2_band_enable_enum),
    /* CLE */
    SOC_ENUM!(c"CLE Level Detect", cle_level_detection_enum), SOC_ENUM!(c"CLE Level Detect Win", cle_level_detection_window_enum),
    SOC_SINGLE!(c"Expander Switch", R_CLECTL, FB_CLECTL_EXP_EN, 1, 0), SOC_SINGLE!(c"Limiter Switch", R_CLECTL, FB_CLECTL_LIMIT_EN, 1, 0),
    SOC_SINGLE!(c"Comp Switch", R_CLECTL, FB_CLECTL_COMP_EN, 1, 0),
    SOC_SINGLE_TLV!(c"CLE Make-Up Gain Volume", R_MUGAIN, FB_MUGAIN_CLEMUG, 0x1f, 0, mugain_scale),
    SOC_SINGLE_TLV!(c"Comp Thresh Volume", R_COMPTH, FB_COMPTH, 0xff, 0, compth_scale),
    SOC_ENUM!(c"Comp Ratio", compressor_ratio_enum), SND_SOC_BYTES!(c"Comp Atk Time", R_CATKTCL, 2),
    /* Effects */
    SOC_SINGLE!(c"3D Switch", R_FXCTL, FB_FXCTL_3DEN, 1, 0), SOC_SINGLE!(c"Treble Switch", R_FXCTL, FB_FXCTL_TEEN, 1, 0),
    SOC_SINGLE!(c"Treble Bypass Switch", R_FXCTL, FB_FXCTL_TNLFBYPASS, 1, 0), SOC_SINGLE!(c"Bass Switch", R_FXCTL, FB_FXCTL_BEEN, 1, 0),
    SOC_SINGLE!(c"Bass Bypass Switch", R_FXCTL, FB_FXCTL_BNLFBYPASS, 1, 0),
    /* MBC */
    SOC_SINGLE!(c"MBC Band1 Switch", R_DACMBCEN, FB_DACMBCEN_MBCEN1, 1, 0),
    SOC_SINGLE!(c"MBC Band2 Switch", R_DACMBCEN, FB_DACMBCEN_MBCEN2, 1, 0),
    SOC_SINGLE!(c"MBC Band3 Switch", R_DACMBCEN, FB_DACMBCEN_MBCEN3, 1, 0),
    SOC_ENUM!(c"MBC Band1 Level Detect", mbc_level_detection_enums[0]),
    SOC_ENUM!(c"MBC Band2 Level Detect", mbc_level_detection_enums[1]),
    SOC_ENUM!(c"MBC Band3 Level Detect", mbc_level_detection_enums[2]),
    SOC_ENUM!(c"MBC Band1 Level Detect Win", mbc_level_detection_window_enums[0]),
    SOC_ENUM!(c"MBC Band2 Level Detect Win", mbc_level_detection_window_enums[1]),
    SOC_ENUM!(c"MBC Band3 Level Detect Win", mbc_level_detection_window_enums[2]),
    SOC_SINGLE!(c"MBC1 Phase Invert Switch", R_DACMBCMUG1, FB_DACMBCMUG1_PHASE, 1, 0),
    SOC_SINGLE_TLV!(c"DAC MBC1 Make-Up Gain Volume", R_DACMBCMUG1, FB_DACMBCMUG1_MUGAIN, 0x1f, 0, mugain_scale),
    SOC_SINGLE_TLV!(c"DAC MBC1 Comp Thresh Volume", R_DACMBCTHR1, FB_DACMBCTHR1_THRESH, 0xff, 0, compth_scale),
    SOC_ENUM!(c"DAC MBC1 Comp Ratio", dac_mbc1_compressor_ratio_enum),
    SND_SOC_BYTES!(c"DAC MBC1 Comp Atk Time", R_DACMBCATK1L, 2), SND_SOC_BYTES!(c"DAC MBC1 Comp Rel Time Const", R_DACMBCREL1L, 2),
    SOC_SINGLE!(c"MBC2 Phase Invert Switch", R_DACMBCMUG2, FB_DACMBCMUG2_PHASE, 1, 0),
    SOC_SINGLE_TLV!(c"DAC MBC2 Make-Up Gain Volume", R_DACMBCMUG2, FB_DACMBCMUG2_MUGAIN, 0x1f, 0, mugain_scale),
    SOC_SINGLE_TLV!(c"DAC MBC2 Comp Thresh Volume", R_DACMBCTHR2, FB_DACMBCTHR2_THRESH, 0xff, 0, compth_scale),
    SOC_ENUM!(c"DAC MBC2 Comp Ratio", dac_mbc2_compressor_ratio_enum),
    SND_SOC_BYTES!(c"DAC MBC2 Comp Atk Time", R_DACMBCATK2L, 2), SND_SOC_BYTES!(c"DAC MBC2 Comp Rel Time Const", R_DACMBCREL2L, 2),
    SOC_SINGLE!(c"MBC3 Phase Invert Switch", R_DACMBCMUG3, FB_DACMBCMUG3_PHASE, 1, 0),
    SOC_SINGLE_TLV!(c"DAC MBC3 Make-Up Gain Volume", R_DACMBCMUG3, FB_DACMBCMUG3_MUGAIN, 0x1f, 0, mugain_scale),
    SOC_SINGLE_TLV!(c"DAC MBC3 Comp Thresh Volume", R_DACMBCTHR3, FB_DACMBCTHR3_THRESH, 0xff, 0, compth_scale),
    SOC_ENUM!(c"DAC MBC3 Comp Ratio", dac_mbc3_compressor_ratio_enum),
    SND_SOC_BYTES!(c"DAC MBC3 Comp Atk Time", R_DACMBCATK3L, 2), SND_SOC_BYTES!(c"DAC MBC3 Comp Rel Time Const", R_DACMBCREL3L, 2),
];

unsafe fn setup_sample_format(component: *mut snd_soc_component, format: snd_pcm_format_t) -> c_int {
    let width = match format {
        SNDRV_PCM_FORMAT_S16_LE => RV_AIC1_WL_16,
        SNDRV_PCM_FORMAT_S20_3LE => RV_AIC1_WL_20,
        SNDRV_PCM_FORMAT_S24_LE => RV_AIC1_WL_24,
        SNDRV_PCM_FORMAT_S32_LE => RV_AIC1_WL_32,
        _ => {
            let ret = -EINVAL;
            dev_err!((*component).dev, "Unsupported format width (%d)\n", ret);
            return ret;
        }
    };
    let ret = snd_soc_component_update_bits(component, R_AIC1, RM_AIC1_WL, width);
    if ret < 0 {
        dev_err!((*component).dev, "Failed to set sample width (%d)\n", ret);
        return ret;
    }
    0
}

unsafe fn setup_sample_rate(component: *mut snd_soc_component, rate: c_uint) -> c_int {
    let tscs42xx = snd_soc_component_get_drvdata(component) as *mut tscs42xx;
    let (br, bm) = match rate {
        8000 => (RV_DACSR_DBR_32, RV_DACSR_DBM_PT25),
        16000 => (RV_DACSR_DBR_32, RV_DACSR_DBM_PT5),
        24000 => (RV_DACSR_DBR_48, RV_DACSR_DBM_PT5),
        32000 => (RV_DACSR_DBR_32, RV_DACSR_DBM_1),
        48000 => (RV_DACSR_DBR_48, RV_DACSR_DBM_1),
        96000 => (RV_DACSR_DBR_48, RV_DACSR_DBM_2),
        11025 => (RV_DACSR_DBR_44_1, RV_DACSR_DBM_PT25),
        22050 => (RV_DACSR_DBR_44_1, RV_DACSR_DBM_PT5),
        44100 => (RV_DACSR_DBR_44_1, RV_DACSR_DBM_1),
        88200 => (RV_DACSR_DBR_44_1, RV_DACSR_DBM_2),
        _ => {
            dev_err!((*component).dev, "Unsupported sample rate %d\n", rate);
            return -EINVAL;
        }
    };

    /* DAC and ADC share bit and frame clock */
    for (reg, mask, val) in [
        (R_DACSR, RM_DACSR_DBR, br),
        (R_DACSR, RM_DACSR_DBM, bm),
        (R_ADCSR, RM_DACSR_DBR, br),
        (R_ADCSR, RM_DACSR_DBM, bm),
    ] {
        let ret = snd_soc_component_update_bits(component, reg, mask, val);
        if ret < 0 {
            dev_err!((*component).dev, "Failed to update register (%d)\n", ret);
            return ret;
        }
    }

    let _guard = mutex_guard(&mut (*tscs42xx).audio_params_lock);
    (*tscs42xx).samplerate = rate as c_int;
    0
}

macro_rules! PLL_CTL {
    ($f:expr, $rt:expr, $rd:expr, $r1b_l:expr, $r9:expr, $ra:expr, $rb:expr,
     $rc:expr, $r12:expr, $r1b_h:expr, $re:expr, $rf:expr, $r10:expr, $r11:expr) => {
        pll_ctl {
            input_freq: $f,
            settings: [
                reg_setting { addr: R_TIMEBASE, val: $rt, mask: 0xFF },
                reg_setting { addr: R_PLLCTLD, val: $rd, mask: 0xFF },
                reg_setting { addr: R_PLLCTL1B, val: $r1b_l, mask: 0x0F },
                reg_setting { addr: R_PLLCTL9, val: $r9, mask: 0xFF },
                reg_setting { addr: R_PLLCTLA, val: $ra, mask: 0xFF },
                reg_setting { addr: R_PLLCTLB, val: $rb, mask: 0xFF },
                reg_setting { addr: R_PLLCTLC, val: $rc, mask: 0xFF },
                reg_setting { addr: R_PLLCTL12, val: $r12, mask: 0xFF },
                reg_setting { addr: R_PLLCTL1B, val: $r1b_h, mask: 0xF0 },
                reg_setting { addr: R_PLLCTLE, val: $re, mask: 0xFF },
                reg_setting { addr: R_PLLCTLF, val: $rf, mask: 0xFF },
                reg_setting { addr: R_PLLCTL10, val: $r10, mask: 0xFF },
                reg_setting { addr: R_PLLCTL11, val: $r11, mask: 0xFF },
            ],
        }
    };
}

static pll_ctls: [pll_ctl; 23] = [
    PLL_CTL!(1411200, 0x05, 0x39, 0x04, 0x07, 0x02, 0xC3, 0x04, 0x1B, 0x10, 0x03, 0x03, 0xD0, 0x02),
    PLL_CTL!(1536000, 0x05, 0x1A, 0x04, 0x02, 0x03, 0xE0, 0x01, 0x1A, 0x10, 0x02, 0x03, 0xB9, 0x01),
    PLL_CTL!(2822400, 0x0A, 0x23, 0x04, 0x07, 0x04, 0xC3, 0x04, 0x22, 0x10, 0x05, 0x03, 0x58, 0x02),
    PLL_CTL!(3072000, 0x0B, 0x22, 0x04, 0x07, 0x03, 0x48, 0x03, 0x1A, 0x10, 0x04, 0x03, 0xB9, 0x01),
    PLL_CTL!(5644800, 0x15, 0x23, 0x04, 0x0E, 0x04, 0xC3, 0x04, 0x1A, 0x10, 0x08, 0x03, 0xE0, 0x01),
    PLL_CTL!(6144000, 0x17, 0x1A, 0x04, 0x08, 0x03, 0xE0, 0x01, 0x1A, 0x10, 0x08, 0x03, 0xB9, 0x01),
    PLL_CTL!(12000000, 0x2E, 0x1B, 0x04, 0x19, 0x03, 0x00, 0x03, 0x2A, 0x10, 0x19, 0x05, 0x98, 0x04),
    PLL_CTL!(19200000, 0x4A, 0x13, 0x04, 0x14, 0x03, 0x80, 0x01, 0x1A, 0x10, 0x19, 0x03, 0xB9, 0x01),
    PLL_CTL!(22000000, 0x55, 0x2A, 0x04, 0x37, 0x05, 0x00, 0x06, 0x22, 0x10, 0x26, 0x03, 0x49, 0x02),
    PLL_CTL!(22579200, 0x57, 0x22, 0x04, 0x31, 0x03, 0x20, 0x03, 0x1A, 0x10, 0x1D, 0x03, 0xB3, 0x01),
    PLL_CTL!(24000000, 0x5D, 0x13, 0x04, 0x19, 0x03, 0x80, 0x01, 0x1B, 0x10, 0x19, 0x05, 0x4C, 0x02),
    PLL_CTL!(24576000, 0x5F, 0x13, 0x04, 0x1D, 0x03, 0xB3, 0x01, 0x22, 0x10, 0x40, 0x03, 0x72, 0x03),
    PLL_CTL!(27000000, 0x68, 0x22, 0x04, 0x4B, 0x03, 0x00, 0x04, 0x2A, 0x10, 0x7D, 0x03, 0x20, 0x06),
    PLL_CTL!(36000000, 0x8C, 0x1B, 0x04, 0x4B, 0x03, 0x00, 0x03, 0x2A, 0x10, 0x7D, 0x03, 0x98, 0x04),
    PLL_CTL!(25000000, 0x61, 0x1B, 0x04, 0x37, 0x03, 0x2B, 0x03, 0x1A, 0x10, 0x2A, 0x03, 0x39, 0x02),
    PLL_CTL!(26000000, 0x65, 0x23, 0x04, 0x41, 0x05, 0x00, 0x06, 0x1A, 0x10, 0x26, 0x03, 0xEF, 0x01),
    PLL_CTL!(12288000, 0x2F, 0x1A, 0x04, 0x12, 0x03, 0x1C, 0x02, 0x22, 0x10, 0x20, 0x03, 0x72, 0x03),
    PLL_CTL!(40000000, 0x9B, 0x22, 0x08, 0x7D, 0x03, 0x80, 0x04, 0x23, 0x10, 0x7D, 0x05, 0xE4, 0x06),
    PLL_CTL!(512000, 0x01, 0x22, 0x04, 0x01, 0x03, 0xD0, 0x02, 0x1B, 0x10, 0x01, 0x04, 0x72, 0x03),
    PLL_CTL!(705600, 0x02, 0x22, 0x04, 0x02, 0x03, 0x15, 0x04, 0x22, 0x10, 0x01, 0x04, 0x80, 0x02),
    PLL_CTL!(1024000, 0x03, 0x22, 0x04, 0x02, 0x03, 0xD0, 0x02, 0x1B, 0x10, 0x02, 0x04, 0x72, 0x03),
    PLL_CTL!(2048000, 0x07, 0x22, 0x04, 0x04, 0x03, 0xD0, 0x02, 0x1B, 0x10, 0x04, 0x04, 0x72, 0x03),
    PLL_CTL!(2400000, 0x08, 0x22, 0x04, 0x05, 0x03, 0x00, 0x03, 0x23, 0x10, 0x05, 0x05, 0x98, 0x04),
];

fn get_pll_ctl(input_freq: c_int) -> *const pll_ctl {
    for pll_ctl in pll_ctls.iter() {
        if input_freq == pll_ctl.input_freq {
            return pll_ctl as *const pll_ctl;
        }
    }
    ptr::null()
}

unsafe fn set_pll_ctl_from_input_freq(component: *mut snd_soc_component, input_freq: c_int) -> c_int {
    let pll_ctl = get_pll_ctl(input_freq);
    if pll_ctl.is_null() {
        let ret = -EINVAL;
        dev_err!((*component).dev, "No PLL input entry for %d (%d)\n", input_freq, ret);
        return ret;
    }

    let mut i = 0usize;
    while i < PLL_REG_SETTINGS_COUNT {
        let setting = &(*pll_ctl).settings[i];
        let ret = snd_soc_component_update_bits(component, setting.addr, setting.mask, setting.val);
        if ret < 0 {
            dev_err!((*component).dev, "Failed to set pll ctl (%d)\n", ret);
            return ret;
        }
        i += 1;
    }
    0
}

unsafe fn tscs42xx_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    codec_dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*codec_dai).component;
    let mut ret = setup_sample_format(component, params_format(params));
    if ret < 0 {
        dev_err!((*component).dev, "Failed to setup sample format (%d)\n", ret);
        return ret;
    }
    ret = setup_sample_rate(component, params_rate(params));
    if ret < 0 {
        dev_err!((*component).dev, "Failed to setup sample rate (%d)\n", ret);
        return ret;
    }
    0
}

unsafe fn dac_mute(component: *mut snd_soc_component) -> c_int {
    let ret = snd_soc_component_update_bits(component, R_CNVRTR1, RM_CNVRTR1_DACMU, RV_CNVRTR1_DACMU_ENABLE);
    if ret < 0 { dev_err!((*component).dev, "Failed to mute DAC (%d)\n", ret); return ret; }
    0
}

unsafe fn dac_unmute(component: *mut snd_soc_component) -> c_int {
    let ret = snd_soc_component_update_bits(component, R_CNVRTR1, RM_CNVRTR1_DACMU, RV_CNVRTR1_DACMU_DISABLE);
    if ret < 0 { dev_err!((*component).dev, "Failed to unmute DAC (%d)\n", ret); return ret; }
    0
}

unsafe fn adc_mute(component: *mut snd_soc_component) -> c_int {
    let ret = snd_soc_component_update_bits(component, R_CNVRTR0, RM_CNVRTR0_ADCMU, RV_CNVRTR0_ADCMU_ENABLE);
    if ret < 0 { dev_err!((*component).dev, "Failed to mute ADC (%d)\n", ret); return ret; }
    0
}

unsafe fn adc_unmute(component: *mut snd_soc_component) -> c_int {
    let ret = snd_soc_component_update_bits(component, R_CNVRTR0, RM_CNVRTR0_ADCMU, RV_CNVRTR0_ADCMU_DISABLE);
    if ret < 0 { dev_err!((*component).dev, "Failed to unmute ADC (%d)\n", ret); return ret; }
    0
}

unsafe fn tscs42xx_mute_stream(dai: *mut snd_soc_dai, mute: c_int, stream: c_int) -> c_int {
    let component = (*dai).component;
    if mute != 0 {
        if stream == SNDRV_PCM_STREAM_PLAYBACK { dac_mute(component) } else { adc_mute(component) }
    } else if stream == SNDRV_PCM_STREAM_PLAYBACK {
        dac_unmute(component)
    } else {
        adc_unmute(component)
    }
}

unsafe fn tscs42xx_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    /* Consumer mode not supported since it needs always-on frame clock */
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => {
            let ret = snd_soc_component_update_bits(component, R_AIC1, RM_AIC1_MS, RV_AIC1_MS_MASTER);
            if ret < 0 {
                dev_err!((*component).dev, "Failed to set codec DAI master (%d)\n", ret);
                return ret;
            }
        }
        _ => {
            let ret = -EINVAL;
            dev_err!((*component).dev, "Unsupported format (%d)\n", ret);
            return ret;
        }
    }
    0
}

unsafe fn tscs42xx_set_dai_bclk_ratio(codec_dai: *mut snd_soc_dai, ratio: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let tscs42xx = snd_soc_component_get_drvdata(component) as *mut tscs42xx;
    let value = match ratio {
        32 => RV_DACSR_DBCM_32,
        40 => RV_DACSR_DBCM_40,
        64 => RV_DACSR_DBCM_64,
        _ => {
            let ret = 0;
            dev_err!((*component).dev, "Unsupported bclk ratio (%d)\n", ret);
            return -EINVAL;
        }
    };

    let mut ret = snd_soc_component_update_bits(component, R_DACSR, RM_DACSR_DBCM, value);
    if ret < 0 {
        dev_err!((*component).dev, "Failed to set DAC BCLK ratio (%d)\n", ret);
        return ret;
    }
    ret = snd_soc_component_update_bits(component, R_ADCSR, RM_ADCSR_ABCM, value);
    if ret < 0 {
        dev_err!((*component).dev, "Failed to set ADC BCLK ratio (%d)\n", ret);
        return ret;
    }

    let _guard = mutex_guard(&mut (*tscs42xx).audio_params_lock);
    (*tscs42xx).bclk_ratio = ratio as c_int;
    0
}

static tscs42xx_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(tscs42xx_hw_params),
    mute_stream: Some(tscs42xx_mute_stream),
    set_fmt: Some(tscs42xx_set_dai_fmt),
    set_bclk_ratio: Some(tscs42xx_set_dai_bclk_ratio),
};

unsafe fn part_is_valid(tscs42xx: *mut tscs42xx) -> c_int {
    let mut reg: c_uint = 0;
    let mut ret = regmap_read((*tscs42xx).regmap, R_DEVIDH, &mut reg);
    if ret < 0 { return ret; }
    let mut val: c_int = (reg << 8) as c_int;
    ret = regmap_read((*tscs42xx).regmap, R_DEVIDL, &mut reg);
    if ret < 0 { return ret; }
    val |= reg as c_int;
    match val {
        0x4A74 | 0x4A73 => true as c_int,
        _ => false as c_int,
    }
}

unsafe fn set_sysclk(component: *mut snd_soc_component) -> c_int {
    let tscs42xx = snd_soc_component_get_drvdata(component) as *mut tscs42xx;
    match (*tscs42xx).sysclk_src_id {
        TSCS42XX_PLL_SRC_XTAL | TSCS42XX_PLL_SRC_MCLK1 => {
            let ret = snd_soc_component_write(
                component,
                R_PLLREFSEL,
                RV_PLLREFSEL_PLL1_REF_SEL_XTAL_MCLK1 | RV_PLLREFSEL_PLL2_REF_SEL_XTAL_MCLK1,
            );
            if ret < 0 {
                dev_err!((*component).dev, "Failed to set pll reference input (%d)\n", ret);
                return ret;
            }
        }
        TSCS42XX_PLL_SRC_MCLK2 => {
            let ret = snd_soc_component_write(
                component,
                R_PLLREFSEL,
                RV_PLLREFSEL_PLL1_REF_SEL_MCLK2 | RV_PLLREFSEL_PLL2_REF_SEL_MCLK2,
            );
            if ret < 0 {
                dev_err!((*component).dev, "Failed to set PLL reference (%d)\n", ret);
                return ret;
            }
        }
        _ => {
            dev_err!((*component).dev, "pll src is unsupported\n");
            return -EINVAL;
        }
    }

    let freq = clk_get_rate((*tscs42xx).sysclk);
    let ret = set_pll_ctl_from_input_freq(component, freq as c_int);
    if ret < 0 {
        dev_err!((*component).dev, "Failed to setup PLL input freq (%d)\n", ret);
        return ret;
    }
    0
}

unsafe fn tscs42xx_probe(component: *mut snd_soc_component) -> c_int {
    set_sysclk(component)
}

static soc_codec_dev_tscs42xx: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(tscs42xx_probe),
    dapm_widgets: tscs42xx_dapm_widgets.as_ptr(),
    num_dapm_widgets: tscs42xx_dapm_widgets.len() as c_uint,
    dapm_routes: tscs42xx_intercon.as_ptr(),
    num_dapm_routes: tscs42xx_intercon.len() as c_uint,
    controls: tscs42xx_snd_controls.as_ptr(),
    num_controls: tscs42xx_snd_controls.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe fn init_coeff_ram_cache(tscs42xx: *mut tscs42xx) {
    static norm_addrs: [u8; 45] = [
        0x00, 0x05, 0x0a, 0x0f, 0x14, 0x19, 0x1f, 0x20, 0x25, 0x2a,
        0x2f, 0x34, 0x39, 0x3f, 0x40, 0x45, 0x4a, 0x4f, 0x54, 0x59,
        0x5f, 0x60, 0x65, 0x6a, 0x6f, 0x74, 0x79, 0x7f, 0x80, 0x85,
        0x8c, 0x91, 0x96, 0x97, 0x9c, 0xa3, 0xa8, 0xad, 0xaf, 0xb0,
        0xb5, 0xba, 0xbf, 0xc4, 0xc9,
    ];
    let coeff_ram = (*tscs42xx).coeff_ram.as_mut_ptr();
    let mut i = 0usize;
    while i < norm_addrs.len() {
        *coeff_ram.add(((norm_addrs[i] as usize + 1) * COEFF_SIZE) - 1) = 0x40;
        i += 1;
    }
}

const TSCS42XX_RATES: c_uint = SNDRV_PCM_RATE_8000_96000;
const TSCS42XX_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static mut tscs42xx_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"tscs42xx-HiFi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"HiFi Playback".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: TSCS42XX_RATES,
        formats: TSCS42XX_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"HiFi Capture".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: TSCS42XX_RATES,
        formats: TSCS42XX_FORMATS,
    },
    ops: &tscs42xx_dai_ops,
    symmetric_rate: 1,
    symmetric_channels: 1,
    symmetric_sample_bits: 1,
};

static tscs42xx_patch: [reg_sequence; 1] = [
    reg_sequence { reg: R_AIC2, def: RV_AIC2_BLRCM_DAC_BCLK_LRCLK_SHARED },
];

static src_names: [*const c_char; TSCS42XX_PLL_SRC_CNT as usize] =
    [c"xtal".as_ptr(), c"mclk1".as_ptr(), c"mclk2".as_ptr()];

unsafe fn tscs42xx_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let tscs42xx = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<tscs42xx>(), GFP_KERNEL) as *mut tscs42xx;
    if tscs42xx.is_null() {
        let ret = -ENOMEM;
        dev_err!(&mut (*i2c).dev, "Failed to allocate memory for data (%d)\n", ret);
        return ret;
    }
    i2c_set_clientdata(i2c, tscs42xx as *mut c_void);

    let mut src = TSCS42XX_PLL_SRC_XTAL;
    while src < TSCS42XX_PLL_SRC_CNT {
        (*tscs42xx).sysclk = devm_clk_get(&mut (*i2c).dev, src_names[src as usize]);
        if !IS_ERR((*tscs42xx).sysclk as *const c_void) {
            break;
        } else if PTR_ERR((*tscs42xx).sysclk as *const c_void) != -ENOENT {
            let ret = PTR_ERR((*tscs42xx).sysclk as *const c_void);
            dev_err!(&mut (*i2c).dev, "Failed to get sysclk (%d)\n", ret);
            return ret;
        }
        src += 1;
    }
    if src == TSCS42XX_PLL_SRC_CNT {
        let ret = -EINVAL;
        dev_err!(&mut (*i2c).dev, "Failed to get a valid clock name (%d)\n", ret);
        return ret;
    }
    (*tscs42xx).sysclk_src_id = src;

    (*tscs42xx).regmap = devm_regmap_init_i2c(i2c, &tscs42xx_regmap);
    if IS_ERR((*tscs42xx).regmap as *const c_void) {
        let ret = PTR_ERR((*tscs42xx).regmap as *const c_void);
        dev_err!(&mut (*i2c).dev, "Failed to allocate regmap (%d)\n", ret);
        return ret;
    }

    init_coeff_ram_cache(tscs42xx);

    let mut ret = part_is_valid(tscs42xx);
    if ret <= 0 {
        dev_err!(&mut (*i2c).dev, "No valid part (%d)\n", ret);
        ret = -ENODEV;
        return ret;
    }

    ret = regmap_write((*tscs42xx).regmap, R_RESET, RV_RESET_ENABLE);
    if ret < 0 {
        dev_err!(&mut (*i2c).dev, "Failed to reset device (%d)\n", ret);
        return ret;
    }

    ret = regmap_register_patch((*tscs42xx).regmap, tscs42xx_patch.as_ptr(), tscs42xx_patch.len() as c_int);
    if ret < 0 {
        dev_err!(&mut (*i2c).dev, "Failed to apply patch (%d)\n", ret);
        return ret;
    }

    mutex_init(&mut (*tscs42xx).audio_params_lock);
    mutex_init(&mut (*tscs42xx).coeff_ram_lock);
    mutex_init(&mut (*tscs42xx).pll_lock);

    ret = devm_snd_soc_register_component(&mut (*i2c).dev, &soc_codec_dev_tscs42xx, &mut tscs42xx_dai, 1);
    if ret != 0 {
        dev_err!(&mut (*i2c).dev, "Failed to register codec (%d)\n", ret);
        return ret;
    }

    0
}

static tscs42xx_i2c_id: [i2c_device_id; 3] = [
    i2c_device_id { name: *b"tscs42A1\0" },
    i2c_device_id { name: *b"tscs42A2\0" },
    i2c_device_id::zeroed(),
];
MODULE_DEVICE_TABLE!(i2c, tscs42xx_i2c_id);

static tscs42xx_of_match: [of_device_id; 3] = [
    of_device_id { compatible: c"tempo,tscs42A1".as_ptr() },
    of_device_id { compatible: c"tempo,tscs42A2".as_ptr() },
    of_device_id::zeroed(),
];
MODULE_DEVICE_TABLE!(of, tscs42xx_of_match);

static mut tscs42xx_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"tscs42xx".as_ptr(),
        of_match_table: tscs42xx_of_match.as_ptr(),
    },
    probe: Some(tscs42xx_i2c_probe),
    id_table: tscs42xx_i2c_id.as_ptr(),
};

module_i2c_driver!(tscs42xx_i2c_driver);

MODULE_AUTHOR!(c"Tempo Semiconductor <steven.eckhoff.opensource@gmail.com");
MODULE_DESCRIPTION!(c"ASoC TSCS42xx driver");
MODULE_LICENSE!(c"GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
