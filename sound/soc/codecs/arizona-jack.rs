// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * extcon-arizona.c - Extcon driver Wolfson Arizona devices
 *
 *  Copyright (C) 2012-2014 Wolfson Microelectronics plc
 */

// C include dependencies translated as external crate/module dependencies:
// linux/cleanup.h, linux/kernel.h, linux/module.h, linux/slab.h,
// linux/interrupt.h, linux/err.h, linux/gpio/consumer.h, linux/input.h,
// linux/pm_runtime.h, linux/property.h, linux/regulator/consumer.h,
// sound/jack.h, sound/soc.h, linux/mfd/arizona/*, dt-bindings/mfd/arizona.h,
// and "arizona.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(improper_ctypes)]

use crate::*;

const ARIZONA_MAX_MICD_RANGE: usize = 8;

/*
 * The hardware supports 8 ranges / buttons, but the snd-jack interface
 * only supports 6 buttons (button 0-5).
 */
const ARIZONA_MAX_MICD_BUTTONS: i32 = 6;

const ARIZONA_MICD_CLAMP_MODE_JDL: u32 = 0x4;
const ARIZONA_MICD_CLAMP_MODE_JDH: u32 = 0x5;
const ARIZONA_MICD_CLAMP_MODE_JDL_GP5H: u32 = 0x9;
const ARIZONA_MICD_CLAMP_MODE_JDH_GP5H: u32 = 0xb;

const ARIZONA_TST_CAP_DEFAULT: u32 = 0x3;
const ARIZONA_TST_CAP_CLAMP: u32 = 0x1;

const ARIZONA_HPDET_MAX: i32 = 10000;

const HPDET_DEBOUNCE: u32 = 500;
const DEFAULT_MICD_TIMEOUT: u32 = 2000;

const ARIZONA_HPDET_WAIT_COUNT: i32 = 15;
const ARIZONA_HPDET_WAIT_DELAY_MS: u32 = 20;

const QUICK_HEADPHONE_MAX_OHM: u32 = 3;
const MICROPHONE_MIN_OHM: u32 = 1257;
const MICROPHONE_MAX_OHM: u32 = 30000;

const MICD_DBTIME_TWO_READINGS: u32 = 2;
const MICD_DBTIME_FOUR_READINGS: u32 = 4;

const MICD_LVL_1_TO_7: u32 = ARIZONA_MICD_LVL_1 | ARIZONA_MICD_LVL_2 |
    ARIZONA_MICD_LVL_3 | ARIZONA_MICD_LVL_4 |
    ARIZONA_MICD_LVL_5 | ARIZONA_MICD_LVL_6 |
    ARIZONA_MICD_LVL_7;

const MICD_LVL_0_TO_7: u32 = ARIZONA_MICD_LVL_0 | MICD_LVL_1_TO_7;

const MICD_LVL_0_TO_8: u32 = MICD_LVL_0_TO_7 | ARIZONA_MICD_LVL_8;

static micd_default_modes: [arizona_micd_config; 2] = [
    arizona_micd_config { src: ARIZONA_ACCDET_SRC, bias: 1, gpio: 0 },
    arizona_micd_config { src: 0, bias: 2, gpio: 1 },
];

static micd_default_ranges: [arizona_micd_range; 6] = [
    arizona_micd_range { max: 11, key: BTN_0 },
    arizona_micd_range { max: 28, key: BTN_1 },
    arizona_micd_range { max: 54, key: BTN_2 },
    arizona_micd_range { max: 100, key: BTN_3 },
    arizona_micd_range { max: 186, key: BTN_4 },
    arizona_micd_range { max: 430, key: BTN_5 },
];

/* The number of levels in arizona_micd_levels valid for button thresholds */
const ARIZONA_NUM_MICD_BUTTON_LEVELS: usize = 64;

static arizona_micd_levels: [i32; 66] = [
    3, 6, 8, 11, 13, 16, 18, 21, 23, 26, 28, 31, 34, 36, 39, 41, 44, 46,
    49, 52, 54, 57, 60, 62, 65, 67, 70, 73, 75, 78, 81, 83, 89, 94, 100,
    105, 111, 116, 122, 127, 139, 150, 161, 173, 186, 196, 209, 220, 245,
    270, 295, 321, 348, 375, 402, 430, 489, 550, 614, 681, 752, 903, 1071,
    1257, 30000,
];

unsafe fn arizona_start_hpdet_acc_id(info: *mut arizona_priv);

unsafe fn arizona_extcon_hp_clamp(info: *mut arizona_priv, clamp: bool) {
    let arizona = (*info).arizona;
    let mut mask: u32 = 0;
    let mut val: u32 = 0;
    let mut cap_sel: u32 = 0;
    let mut ret: i32;

    match (*arizona).type_ {
    WM8998 | WM1814 => {
        mask = 0;
    }
    WM5110 | WM8280 => {
        mask = ARIZONA_HP1L_SHRTO | ARIZONA_HP1L_FLWR | ARIZONA_HP1L_SHRTI;
        if clamp {
            val = ARIZONA_HP1L_SHRTO;
            cap_sel = ARIZONA_TST_CAP_CLAMP;
        } else {
            val = ARIZONA_HP1L_FLWR | ARIZONA_HP1L_SHRTI;
            cap_sel = ARIZONA_TST_CAP_DEFAULT;
        }

        ret = regmap_update_bits((*arizona).regmap, ARIZONA_HP_TEST_CTRL_1,
                                 ARIZONA_HP1_TST_CAP_SEL_MASK, cap_sel);
        if ret != 0 {
            dev_warn((*arizona).dev, c"Failed to set TST_CAP_SEL: %d\n".as_ptr(), ret);
        }
    }
    _ => {
        mask = ARIZONA_RMV_SHRT_HP1L;
        if clamp {
            val = ARIZONA_RMV_SHRT_HP1L;
        }
    }
    }

    snd_soc_dapm_mutex_lock((*arizona).dapm);
    (*arizona).hpdet_clamp = clamp;

    /* Keep the HP output stages disabled while doing the clamp */
    if clamp {
        ret = regmap_update_bits((*arizona).regmap, ARIZONA_OUTPUT_ENABLES_1,
                                 ARIZONA_OUT1L_ENA | ARIZONA_OUT1R_ENA, 0);
        if ret != 0 {
            dev_warn((*arizona).dev, c"Failed to disable headphone outputs: %d\n".as_ptr(), ret);
        }
    }

    if mask != 0 {
        ret = regmap_update_bits((*arizona).regmap, ARIZONA_HP_CTRL_1L, mask, val);
        if ret != 0 {
            dev_warn((*arizona).dev, c"Failed to do clamp: %d\n".as_ptr(), ret);
        }

        ret = regmap_update_bits((*arizona).regmap, ARIZONA_HP_CTRL_1R, mask, val);
        if ret != 0 {
            dev_warn((*arizona).dev, c"Failed to do clamp: %d\n".as_ptr(), ret);
        }
    }

    /* Restore the desired state while not doing the clamp */
    if !clamp {
        ret = regmap_update_bits((*arizona).regmap, ARIZONA_OUTPUT_ENABLES_1,
                                 ARIZONA_OUT1L_ENA | ARIZONA_OUT1R_ENA,
                                 (*arizona).hp_ena);
        if ret != 0 {
            dev_warn((*arizona).dev, c"Failed to restore headphone outputs: %d\n".as_ptr(), ret);
        }
    }

    snd_soc_dapm_mutex_unlock((*arizona).dapm);
}

unsafe fn arizona_extcon_set_mode(info: *mut arizona_priv, mut mode: i32) {
    let arizona = (*info).arizona;

    mode %= (*info).micd_num_modes;

    gpiod_set_value_cansleep((*info).micd_pol_gpio,
                             (*(*info).micd_modes.add(mode as usize)).gpio);

    regmap_update_bits((*arizona).regmap, ARIZONA_MIC_DETECT_1,
                       ARIZONA_MICD_BIAS_SRC_MASK,
                       (*(*info).micd_modes.add(mode as usize)).bias << ARIZONA_MICD_BIAS_SRC_SHIFT);
    regmap_update_bits((*arizona).regmap, ARIZONA_ACCESSORY_DETECT_MODE_1,
                       ARIZONA_ACCDET_SRC, (*(*info).micd_modes.add(mode as usize)).src);

    (*info).micd_mode = mode;
    dev_dbg((*arizona).dev, c"Set jack polarity to %d\n".as_ptr(), mode);
}

unsafe fn arizona_extcon_get_micbias(info: *mut arizona_priv) -> *const c_char {
    match (*(*info).micd_modes).bias {
    1 => c"MICBIAS1".as_ptr(),
    2 => c"MICBIAS2".as_ptr(),
    3 => c"MICBIAS3".as_ptr(),
    _ => c"MICVDD".as_ptr(),
    }
}

unsafe fn arizona_extcon_pulse_micbias(info: *mut arizona_priv) {
    let arizona = (*info).arizona;
    let widget = arizona_extcon_get_micbias(info);
    let dapm = (*arizona).dapm;
    let mut ret: i32;

    ret = snd_soc_dapm_force_enable_pin(dapm, widget);
    if ret != 0 {
        dev_warn((*arizona).dev, c"Failed to enable %s: %d\n".as_ptr(), widget, ret);
    }

    snd_soc_dapm_sync(dapm);

    if !(*arizona).pdata.micd_force_micbias {
        ret = snd_soc_dapm_disable_pin(dapm, widget);
        if ret != 0 {
            dev_warn((*arizona).dev, c"Failed to disable %s: %d\n".as_ptr(), widget, ret);
        }
        snd_soc_dapm_sync(dapm);
    }
}

unsafe fn arizona_start_mic(info: *mut arizona_priv) {
    let arizona = (*info).arizona;
    let mut change: bool = false;
    let mut ret: i32;
    let mode: u32;

    /* Microphone detection can't use idle mode */
    pm_runtime_get_sync((*arizona).dev);

    if (*info).detecting {
        ret = regulator_allow_bypass((*info).micvdd, false);
        if ret != 0 {
            dev_err((*arizona).dev, c"Failed to regulate MICVDD: %d\n".as_ptr(), ret);
        }
    }

    ret = regulator_enable((*info).micvdd);
    if ret != 0 {
        dev_err((*arizona).dev, c"Failed to enable MICVDD: %d\n".as_ptr(), ret);
    }

    if (*info).micd_reva {
        let reva = [
            reg_sequence { reg: 0x80, def: 0x3 },
            reg_sequence { reg: 0x294, def: 0x0 },
            reg_sequence { reg: 0x80, def: 0x0 },
        ];
        regmap_multi_reg_write((*arizona).regmap, reva.as_ptr(), reva.len());
    }

    if (*info).detecting && (*arizona).pdata.micd_software_compare {
        mode = ARIZONA_ACCDET_MODE_ADC;
    } else {
        mode = ARIZONA_ACCDET_MODE_MIC;
    }

    regmap_update_bits((*arizona).regmap, ARIZONA_ACCESSORY_DETECT_MODE_1,
                       ARIZONA_ACCDET_MODE_MASK, mode);

    arizona_extcon_pulse_micbias(info);

    ret = regmap_update_bits_check((*arizona).regmap, ARIZONA_MIC_DETECT_1,
                                   ARIZONA_MICD_ENA, ARIZONA_MICD_ENA,
                                   &mut change);
    if ret < 0 {
        dev_err((*arizona).dev, c"Failed to enable micd: %d\n".as_ptr(), ret);
    } else if !change {
        regulator_disable((*info).micvdd);
        pm_runtime_put_autosuspend((*arizona).dev);
    }
}

unsafe fn arizona_stop_mic(info: *mut arizona_priv) {
    let arizona = (*info).arizona;
    let widget = arizona_extcon_get_micbias(info);
    let dapm = (*arizona).dapm;
    let mut change = false;
    let mut ret: i32;

    ret = regmap_update_bits_check((*arizona).regmap, ARIZONA_MIC_DETECT_1,
                                   ARIZONA_MICD_ENA, 0, &mut change);
    if ret < 0 {
        dev_err((*arizona).dev, c"Failed to disable micd: %d\n".as_ptr(), ret);
    }

    ret = snd_soc_dapm_disable_pin(dapm, widget);
    if ret != 0 {
        dev_warn((*arizona).dev, c"Failed to disable %s: %d\n".as_ptr(), widget, ret);
    }
    snd_soc_dapm_sync(dapm);

    if (*info).micd_reva {
        let reva = [
            reg_sequence { reg: 0x80, def: 0x3 },
            reg_sequence { reg: 0x294, def: 0x2 },
            reg_sequence { reg: 0x80, def: 0x0 },
        ];
        regmap_multi_reg_write((*arizona).regmap, reva.as_ptr(), reva.len());
    }

    ret = regulator_allow_bypass((*info).micvdd, true);
    if ret != 0 {
        dev_err((*arizona).dev, c"Failed to bypass MICVDD: %d\n".as_ptr(), ret);
    }

    if change {
        regulator_disable((*info).micvdd);
        pm_runtime_put_autosuspend((*arizona).dev);
    }
}

#[repr(C)]
struct arizona_hpdet_b_range {
    threshold: u32,
    factor_a: u32,
    factor_b: u32,
}

static mut arizona_hpdet_b_ranges: [arizona_hpdet_b_range; 3] = [
    arizona_hpdet_b_range { threshold: 100, factor_a: 5528, factor_b: 362464 },
    arizona_hpdet_b_range { threshold: 169, factor_a: 11084, factor_b: 6186851 },
    arizona_hpdet_b_range { threshold: 169, factor_a: 11065, factor_b: 65460395 },
];

const ARIZONA_HPDET_B_RANGE_MAX: u32 = 0x3fb;

#[repr(C)]
struct arizona_hpdet_c_range {
    min: i32,
    max: i32,
}

static mut arizona_hpdet_c_ranges: [arizona_hpdet_c_range; 4] = [
    arizona_hpdet_c_range { min: 0, max: 30 },
    arizona_hpdet_c_range { min: 8, max: 100 },
    arizona_hpdet_c_range { min: 100, max: 1000 },
    arizona_hpdet_c_range { min: 1000, max: 10000 },
];

unsafe fn arizona_hpdet_read(info: *mut arizona_priv) -> i32 {
    let arizona = (*info).arizona;
    let mut val: u32 = 0;
    let mut range: u32 = 0;
    let mut ret: i32;

    ret = regmap_read((*arizona).regmap, ARIZONA_HEADPHONE_DETECT_2, &mut val);
    if ret != 0 {
        dev_err((*arizona).dev, c"Failed to read HPDET status: %d\n".as_ptr(), ret);
        return ret;
    }

    match (*info).hpdet_ip_version {
    0 => {
        if (val & ARIZONA_HP_DONE) == 0 {
            dev_err((*arizona).dev, c"HPDET did not complete: %x\n".as_ptr(), val);
            return -EAGAIN;
        }
        val &= ARIZONA_HP_LVL_MASK;
    }
    1 => {
        if (val & ARIZONA_HP_DONE_B) == 0 {
            dev_err((*arizona).dev, c"HPDET did not complete: %x\n".as_ptr(), val);
            return -EAGAIN;
        }

        ret = regmap_read((*arizona).regmap, ARIZONA_HP_DACVAL, &mut val);
        if ret != 0 {
            dev_err((*arizona).dev, c"Failed to read HP value: %d\n".as_ptr(), ret);
            return -EAGAIN;
        }

        regmap_read((*arizona).regmap, ARIZONA_HEADPHONE_DETECT_1, &mut range);
        range = (range & ARIZONA_HP_IMPEDANCE_RANGE_MASK) >> ARIZONA_HP_IMPEDANCE_RANGE_SHIFT;

        if (range as usize) < arizona_hpdet_b_ranges.len() - 1 &&
           (val < arizona_hpdet_b_ranges[range as usize].threshold ||
            val >= ARIZONA_HPDET_B_RANGE_MAX) {
            range += 1;
            dev_dbg((*arizona).dev, c"Moving to HPDET range %d\n".as_ptr(), range);
            regmap_update_bits((*arizona).regmap, ARIZONA_HEADPHONE_DETECT_1,
                               ARIZONA_HP_IMPEDANCE_RANGE_MASK,
                               range << ARIZONA_HP_IMPEDANCE_RANGE_SHIFT);
            return -EAGAIN;
        }

        /* If we go out of range report top of range */
        if val < arizona_hpdet_b_ranges[range as usize].threshold ||
           val >= ARIZONA_HPDET_B_RANGE_MAX {
            dev_dbg((*arizona).dev, c"Measurement out of range\n".as_ptr());
            return ARIZONA_HPDET_MAX;
        }

        dev_dbg((*arizona).dev, c"HPDET read %d in range %d\n".as_ptr(), val, range);
        val = arizona_hpdet_b_ranges[range as usize].factor_b /
              ((val * 100) - arizona_hpdet_b_ranges[range as usize].factor_a);
    }
    2 => {
        if (val & ARIZONA_HP_DONE_B) == 0 {
            dev_err((*arizona).dev, c"HPDET did not complete: %x\n".as_ptr(), val);
            return -EAGAIN;
        }

        val &= ARIZONA_HP_LVL_B_MASK;
        /* Convert to ohms, the value is in 0.5 ohm increments */
        val /= 2;

        regmap_read((*arizona).regmap, ARIZONA_HEADPHONE_DETECT_1, &mut range);
        range = (range & ARIZONA_HP_IMPEDANCE_RANGE_MASK) >> ARIZONA_HP_IMPEDANCE_RANGE_SHIFT;

        /* Skip up a range, or report? */
        if (range as usize) < arizona_hpdet_c_ranges.len() - 1 &&
           (val as i32 >= arizona_hpdet_c_ranges[range as usize].max) {
            range += 1;
            dev_dbg((*arizona).dev, c"Moving to HPDET range %d-%d\n".as_ptr(),
                    arizona_hpdet_c_ranges[range as usize].min,
                    arizona_hpdet_c_ranges[range as usize].max);
            regmap_update_bits((*arizona).regmap, ARIZONA_HEADPHONE_DETECT_1,
                               ARIZONA_HP_IMPEDANCE_RANGE_MASK,
                               range << ARIZONA_HP_IMPEDANCE_RANGE_SHIFT);
            return -EAGAIN;
        }

        if range != 0 && (val as i32) < arizona_hpdet_c_ranges[range as usize].min {
            dev_dbg((*arizona).dev, c"Reporting range boundary %d\n".as_ptr(),
                    arizona_hpdet_c_ranges[range as usize].min);
            val = arizona_hpdet_c_ranges[range as usize].min as u32;
        }
    }
    _ => {
        dev_warn((*arizona).dev, c"Unknown HPDET IP revision %d\n".as_ptr(),
                 (*info).hpdet_ip_version);
        return -EINVAL;
    }
    }

    dev_dbg((*arizona).dev, c"HP impedance %d ohms\n".as_ptr(), val);
    val as i32
}

unsafe fn arizona_hpdet_do_id(info: *mut arizona_priv, reading: *mut i32,
                              mic: *mut bool) -> i32 {
    let arizona = (*info).arizona;

    if !(*arizona).pdata.hpdet_acc_id {
        return 0;
    }

    /*
     * If we're using HPDET for accessory identification we need
     * to take multiple measurements, step through them in sequence.
     */
    (*info).hpdet_res[(*info).num_hpdet_res as usize] = *reading;
    (*info).num_hpdet_res += 1;

    /* Only check the mic directly if we didn't already ID it */
    if !(*info).hpdet_id_gpio.is_null() && (*info).num_hpdet_res == 1 {
        dev_dbg((*arizona).dev, c"Measuring mic\n".as_ptr());

        regmap_update_bits((*arizona).regmap, ARIZONA_ACCESSORY_DETECT_MODE_1,
                           ARIZONA_ACCDET_MODE_MASK | ARIZONA_ACCDET_SRC,
                           ARIZONA_ACCDET_MODE_HPR | (*(*info).micd_modes).src);

        gpiod_set_value_cansleep((*info).hpdet_id_gpio, 1);

        regmap_update_bits((*arizona).regmap, ARIZONA_HEADPHONE_DETECT_1,
                           ARIZONA_HP_POLL, ARIZONA_HP_POLL);
        return -EAGAIN;
    }

    /* OK, got both.  Now, compare... */
    dev_dbg((*arizona).dev, c"HPDET measured %d %d\n".as_ptr(),
            (*info).hpdet_res[0], (*info).hpdet_res[1]);

    /* Take the headphone impedance for the main report */
    *reading = (*info).hpdet_res[0];

    /* Sometimes we get false readings due to slow insert */
    if *reading >= ARIZONA_HPDET_MAX && !(*info).hpdet_retried {
        dev_dbg((*arizona).dev, c"Retrying high impedance\n".as_ptr());
        (*info).num_hpdet_res = 0;
        (*info).hpdet_retried = true;
        arizona_start_hpdet_acc_id(info);
        pm_runtime_put((*arizona).dev);
        return -EAGAIN;
    }

    /*
     * If we measure the mic as high impedance
     */
    if (*info).hpdet_id_gpio.is_null() || (*info).hpdet_res[1] > 50 {
        dev_dbg((*arizona).dev, c"Detected mic\n".as_ptr());
        *mic = true;
        (*info).detecting = true;
    } else {
        dev_dbg((*arizona).dev, c"Detected headphone\n".as_ptr());
    }

    /* Make sure everything is reset back to the real polarity */
    regmap_update_bits((*arizona).regmap, ARIZONA_ACCESSORY_DETECT_MODE_1,
                       ARIZONA_ACCDET_SRC, (*(*info).micd_modes).src);

    0
}

unsafe extern "C" fn arizona_hpdet_irq(irq: i32, data: *mut c_void) -> irqreturn_t {
    let info = data as *mut arizona_priv;
    let arizona = (*info).arizona;
    let mut ret: i32;
    let mut reading: i32;
    let state: i32;
    let report: i32;
    let mut mic = false;

    mutex_lock(&mut (*info).lock);

    /* If we got a spurious IRQ for some reason then ignore it */
    if !(*info).hpdet_active {
        dev_warn((*arizona).dev, c"Spurious HPDET IRQ\n".as_ptr());
        mutex_unlock(&mut (*info).lock);
        return IRQ_NONE;
    }

    /* If the cable was removed while measuring ignore the result */
    state = (*(*info).jack).status & SND_JACK_MECHANICAL;
    if state == 0 {
        dev_dbg((*arizona).dev, c"Ignoring HPDET for removed cable\n".as_ptr());
        goto_done_hpdet_irq(info, arizona, state, mic);
        mutex_unlock(&mut (*info).lock);
        return IRQ_HANDLED;
    }

    ret = arizona_hpdet_read(info);
    if ret == -EAGAIN {
        mutex_unlock(&mut (*info).lock);
        return IRQ_HANDLED;
    } else if ret < 0 {
        goto_done_hpdet_irq(info, arizona, state, mic);
        mutex_unlock(&mut (*info).lock);
        return IRQ_HANDLED;
    }
    reading = ret;

    /* Reset back to starting range */
    regmap_update_bits((*arizona).regmap, ARIZONA_HEADPHONE_DETECT_1,
                       ARIZONA_HP_IMPEDANCE_RANGE_MASK | ARIZONA_HP_POLL, 0);

    ret = arizona_hpdet_do_id(info, &mut reading, &mut mic);
    if ret == -EAGAIN {
        mutex_unlock(&mut (*info).lock);
        return IRQ_HANDLED;
    } else if ret < 0 {
        goto_done_hpdet_irq(info, arizona, state, mic);
        mutex_unlock(&mut (*info).lock);
        return IRQ_HANDLED;
    }

    /* Report high impedence cables as line outputs */
    if reading >= 5000 {
        report = SND_JACK_LINEOUT;
    } else {
        report = SND_JACK_HEADPHONE;
    }

    snd_soc_jack_report((*info).jack, report, SND_JACK_LINEOUT | SND_JACK_HEADPHONE);

    goto_done_hpdet_irq(info, arizona, state, mic);
    mutex_unlock(&mut (*info).lock);
    IRQ_HANDLED
}

unsafe fn goto_done_hpdet_irq(info: *mut arizona_priv, arizona: *mut arizona,
                              state: i32, mic: bool) {
    /* Reset back to starting range */
    regmap_update_bits((*arizona).regmap, ARIZONA_HEADPHONE_DETECT_1,
                       ARIZONA_HP_IMPEDANCE_RANGE_MASK | ARIZONA_HP_POLL, 0);

    arizona_extcon_hp_clamp(info, false);

    if !(*info).hpdet_id_gpio.is_null() {
        gpiod_set_value_cansleep((*info).hpdet_id_gpio, 0);
    }

    /* If we have a mic then reenable MICDET */
    if state != 0 && (mic || (*info).mic) {
        arizona_start_mic(info);
    }

    if (*info).hpdet_active {
        pm_runtime_put_autosuspend((*arizona).dev);
        (*info).hpdet_active = false;
    }

    /* Do not set hp_det done when the cable has been unplugged */
    if state != 0 {
        (*info).hpdet_done = true;
    }
}

unsafe fn arizona_identify_headphone(info: *mut arizona_priv) {
    let arizona = (*info).arizona;
    let mut ret: i32;

    if (*info).hpdet_done {
        return;
    }

    dev_dbg((*arizona).dev, c"Starting HPDET\n".as_ptr());

    /* Make sure we keep the device enabled during the measurement */
    pm_runtime_get_sync((*arizona).dev);
    (*info).hpdet_active = true;
    arizona_stop_mic(info);
    arizona_extcon_hp_clamp(info, true);

    ret = regmap_update_bits((*arizona).regmap, ARIZONA_ACCESSORY_DETECT_MODE_1,
                             ARIZONA_ACCDET_MODE_MASK, (*arizona).pdata.hpdet_channel);
    if ret != 0 {
        dev_err((*arizona).dev, c"Failed to set HPDET mode: %d\n".as_ptr(), ret);
        arizona_extcon_hp_clamp(info, false);
        pm_runtime_put_autosuspend((*arizona).dev);
        snd_soc_jack_report((*info).jack, SND_JACK_HEADPHONE,
                            SND_JACK_LINEOUT | SND_JACK_HEADPHONE);
        if (*info).mic {
            arizona_start_mic(info);
        }
        (*info).hpdet_active = false;
        return;
    }

    ret = regmap_update_bits((*arizona).regmap, ARIZONA_HEADPHONE_DETECT_1,
                             ARIZONA_HP_POLL, ARIZONA_HP_POLL);
    if ret != 0 {
        dev_err((*arizona).dev, c"Can't start HPDETL measurement: %d\n".as_ptr(), ret);
        arizona_extcon_hp_clamp(info, false);
        pm_runtime_put_autosuspend((*arizona).dev);
        snd_soc_jack_report((*info).jack, SND_JACK_HEADPHONE,
                            SND_JACK_LINEOUT | SND_JACK_HEADPHONE);
        if (*info).mic {
            arizona_start_mic(info);
        }
        (*info).hpdet_active = false;
    }
}

unsafe fn arizona_start_hpdet_acc_id(info: *mut arizona_priv) {
    let arizona = (*info).arizona;
    let mut hp_reading: i32 = 32;
    let mut mic: bool = false;
    let mut ret: i32;

    dev_dbg((*arizona).dev, c"Starting identification via HPDET\n".as_ptr());

    /* Make sure we keep the device enabled during the measurement */
    pm_runtime_get_sync((*arizona).dev);
    (*info).hpdet_active = true;
    arizona_extcon_hp_clamp(info, true);

    ret = regmap_update_bits((*arizona).regmap, ARIZONA_ACCESSORY_DETECT_MODE_1,
                             ARIZONA_ACCDET_SRC | ARIZONA_ACCDET_MODE_MASK,
                             (*(*info).micd_modes).src | (*arizona).pdata.hpdet_channel);
    if ret != 0 {
        dev_err((*arizona).dev, c"Failed to set HPDET mode: %d\n".as_ptr(), ret);
        snd_soc_jack_report((*info).jack, SND_JACK_HEADPHONE,
                            SND_JACK_LINEOUT | SND_JACK_HEADPHONE);
        (*info).hpdet_active = false;
        return;
    }

    if (*arizona).pdata.hpdet_acc_id_line {
        ret = regmap_update_bits((*arizona).regmap, ARIZONA_HEADPHONE_DETECT_1,
                                 ARIZONA_HP_POLL, ARIZONA_HP_POLL);
        if ret != 0 {
            dev_err((*arizona).dev, c"Can't start HPDETL measurement: %d\n".as_ptr(), ret);
            snd_soc_jack_report((*info).jack, SND_JACK_HEADPHONE,
                                SND_JACK_LINEOUT | SND_JACK_HEADPHONE);
            (*info).hpdet_active = false;
        }
    } else {
        arizona_hpdet_do_id(info, &mut hp_reading, &mut mic);
    }
}

unsafe extern "C" fn arizona_micd_timeout_work(work: *mut work_struct) {
    let info = container_of!(work, arizona_priv, micd_timeout_work.work);
    mutex_lock(&mut (*info).lock);
    dev_dbg((*(*info).arizona).dev, c"MICD timed out, reporting HP\n".as_ptr());
    (*info).detecting = false;
    arizona_identify_headphone(info);
    mutex_unlock(&mut (*info).lock);
}

unsafe fn arizona_micd_adc_read(info: *mut arizona_priv) -> i32 {
    let arizona = (*info).arizona;
    let mut val: u32 = 0;
    let ret: i32;

    /* Must disable MICD before we read the ADCVAL */
    regmap_update_bits((*arizona).regmap, ARIZONA_MIC_DETECT_1, ARIZONA_MICD_ENA, 0);

    ret = regmap_read((*arizona).regmap, ARIZONA_MIC_DETECT_4, &mut val);
    if ret != 0 {
        dev_err((*arizona).dev, c"Failed to read MICDET_ADCVAL: %d\n".as_ptr(), ret);
        return ret;
    }

    dev_dbg((*arizona).dev, c"MICDET_ADCVAL: %x\n".as_ptr(), val);

    val &= ARIZONA_MICDET_ADCVAL_MASK;
    if (val as usize) < arizona_micd_levels.len() {
        val = arizona_micd_levels[val as usize] as u32;
    } else {
        val = INT_MAX as u32;
    }

    if val <= QUICK_HEADPHONE_MAX_OHM {
        val = ARIZONA_MICD_STS | ARIZONA_MICD_LVL_0;
    } else if val <= MICROPHONE_MIN_OHM {
        val = ARIZONA_MICD_STS | ARIZONA_MICD_LVL_1;
    } else if val <= MICROPHONE_MAX_OHM {
        val = ARIZONA_MICD_STS | ARIZONA_MICD_LVL_8;
    } else {
        val = ARIZONA_MICD_LVL_8;
    }

    val as i32
}

unsafe fn arizona_micd_read(info: *mut arizona_priv) -> i32 {
    let arizona = (*info).arizona;
    let mut val: u32 = 0;
    let mut ret: i32;
    let mut i: i32 = 0;

    while i < 10 && (val & MICD_LVL_0_TO_8) == 0 {
        ret = regmap_read((*arizona).regmap, ARIZONA_MIC_DETECT_3, &mut val);
        if ret != 0 {
            dev_err((*arizona).dev, c"Failed to read MICDET: %d\n".as_ptr(), ret);
            return ret;
        }

        dev_dbg((*arizona).dev, c"MICDET: %x\n".as_ptr(), val);

        if (val & ARIZONA_MICD_VALID) == 0 {
            dev_warn((*arizona).dev, c"Microphone detection state invalid\n".as_ptr());
            return -EINVAL;
        }
        i += 1;
    }

    if i == 10 && (val & MICD_LVL_0_TO_8) == 0 {
        dev_err((*arizona).dev, c"Failed to get valid MICDET value\n".as_ptr());
        return -EINVAL;
    }

    val as i32
}

unsafe extern "C" fn arizona_micdet_reading(priv_: *mut c_void) -> i32 {
    let info = priv_ as *mut arizona_priv;
    let arizona = (*info).arizona;
    let mut ret: i32;
    let val: i32;

    if (*info).detecting && (*arizona).pdata.micd_software_compare {
        ret = arizona_micd_adc_read(info);
    } else {
        ret = arizona_micd_read(info);
    }
    if ret < 0 {
        return ret;
    }

    val = ret;

    /* Due to jack detect this should never happen */
    if (val as u32 & ARIZONA_MICD_STS) == 0 {
        dev_warn((*arizona).dev, c"Detected open circuit\n".as_ptr());
        (*info).mic = false;
        (*info).detecting = false;
        arizona_identify_headphone(info);
        return 0;
    }

    /* If we got a high impedence we should have a headset, report it. */
    if (val as u32 & ARIZONA_MICD_LVL_8) != 0 {
        (*info).mic = true;
        (*info).detecting = false;
        arizona_identify_headphone(info);
        snd_soc_jack_report((*info).jack, SND_JACK_MICROPHONE, SND_JACK_MICROPHONE);

        /* Don't need to regulate for button detection */
        ret = regulator_allow_bypass((*info).micvdd, true);
        if ret != 0 {
            dev_err((*arizona).dev, c"Failed to bypass MICVDD: %d\n".as_ptr(), ret);
        }
        return 0;
    }

    /* If we detected a lower impedence during initial startup
     * then we probably have the wrong polarity, flip it.  Don't
     * do this for the lowest impedences to speed up detection of
     * plain headphones.  If both polarities report a low
     * impedence then give up and report headphones.
     */
    if (val as u32 & MICD_LVL_1_TO_7) != 0 {
        if (*info).jack_flips >= (*info).micd_num_modes * 10 {
            dev_dbg((*arizona).dev, c"Detected HP/line\n".as_ptr());
            (*info).detecting = false;
            arizona_identify_headphone(info);
        } else {
            (*info).micd_mode += 1;
            if (*info).micd_mode == (*info).micd_num_modes {
                (*info).micd_mode = 0;
            }
            arizona_extcon_set_mode(info, (*info).micd_mode);
            (*info).jack_flips += 1;

            if (*arizona).pdata.micd_software_compare {
                regmap_update_bits((*arizona).regmap, ARIZONA_MIC_DETECT_1,
                                   ARIZONA_MICD_ENA, ARIZONA_MICD_ENA);
            }

            queue_delayed_work(system_power_efficient_wq, &mut (*info).micd_timeout_work,
                               msecs_to_jiffies((*arizona).pdata.micd_timeout));
        }
        return 0;
    }

    /*
     * If we're still detecting and we detect a short then we've
     * got a headphone.
     */
    dev_dbg((*arizona).dev, c"Headphone detected\n".as_ptr());
    (*info).detecting = false;
    arizona_identify_headphone(info);
    0
}

unsafe extern "C" fn arizona_button_reading(priv_: *mut c_void) -> i32 {
    let info = priv_ as *mut arizona_priv;
    let arizona = (*info).arizona;
    let val: i32;
    let mut key: i32;
    let mut lvl: i32;

    val = arizona_micd_read(info);
    if val < 0 {
        return val;
    }

    /*
     * If we're still detecting and we detect a short then we've
     * got a headphone.  Otherwise it's a button press.
     */
    if (val as u32 & MICD_LVL_0_TO_7) != 0 {
        if (*info).mic {
            dev_dbg((*arizona).dev, c"Mic button detected\n".as_ptr());
            lvl = (val as u32 & ARIZONA_MICD_LVL_MASK) as i32;
            lvl >>= ARIZONA_MICD_LVL_SHIFT;

            if lvl != 0 && ffs(lvl) - 1 < (*info).num_micd_ranges {
                key = ffs(lvl) - 1;
                snd_soc_jack_report((*info).jack, SND_JACK_BTN_0 >> key,
                                    (*info).micd_button_mask);
            } else {
                dev_err((*arizona).dev, c"Button out of range\n".as_ptr());
            }
        } else {
            dev_warn((*arizona).dev, c"Button with no mic: %x\n".as_ptr(), val);
        }
    } else {
        dev_dbg((*arizona).dev, c"Mic button released\n".as_ptr());
        snd_soc_jack_report((*info).jack, 0, (*info).micd_button_mask);
        arizona_extcon_pulse_micbias(info);
    }

    0
}

unsafe extern "C" fn arizona_micd_detect(work: *mut work_struct) {
    let info = container_of!(work, arizona_priv, micd_detect_work.work);
    let arizona = (*info).arizona;

    cancel_delayed_work_sync(&mut (*info).micd_timeout_work);

    mutex_lock(&mut (*info).lock);

    /* If the cable was removed while measuring ignore the result */
    if ((*(*info).jack).status & SND_JACK_MECHANICAL) == 0 {
        dev_dbg((*arizona).dev, c"Ignoring MICDET for removed cable\n".as_ptr());
        mutex_unlock(&mut (*info).lock);
        return;
    }

    if (*info).detecting {
        arizona_micdet_reading(info as *mut c_void);
    } else {
        arizona_button_reading(info as *mut c_void);
    }

    pm_runtime_mark_last_busy((*arizona).dev);
    mutex_unlock(&mut (*info).lock);
}

unsafe extern "C" fn arizona_micdet(irq: i32, data: *mut c_void) -> irqreturn_t {
    let info = data as *mut arizona_priv;
    let arizona = (*info).arizona;
    let mut debounce = (*arizona).pdata.micd_detect_debounce;

    cancel_delayed_work_sync(&mut (*info).micd_detect_work);
    cancel_delayed_work_sync(&mut (*info).micd_timeout_work);

    mutex_lock(&mut (*info).lock);
    if !(*info).detecting {
        debounce = 0;
    }
    mutex_unlock(&mut (*info).lock);

    if debounce != 0 {
        queue_delayed_work(system_power_efficient_wq, &mut (*info).micd_detect_work,
                           msecs_to_jiffies(debounce));
    } else {
        arizona_micd_detect(&mut (*info).micd_detect_work.work);
    }

    IRQ_HANDLED
}

unsafe extern "C" fn arizona_hpdet_work(work: *mut work_struct) {
    let info = container_of!(work, arizona_priv, hpdet_work.work);
    mutex_lock(&mut (*info).lock);
    arizona_start_hpdet_acc_id(info);
    mutex_unlock(&mut (*info).lock);
}

unsafe fn arizona_hpdet_wait(info: *mut arizona_priv) -> i32 {
    let arizona = (*info).arizona;
    let mut val: u32 = 0;
    let mut i: i32 = 0;
    let mut ret: i32;

    while i < ARIZONA_HPDET_WAIT_COUNT {
        ret = regmap_read((*arizona).regmap, ARIZONA_HEADPHONE_DETECT_2, &mut val);
        if ret != 0 {
            dev_err((*arizona).dev, c"Failed to read HPDET state: %d\n".as_ptr(), ret);
            return ret;
        }

        match (*info).hpdet_ip_version {
        0 => {
            if (val & ARIZONA_HP_DONE) != 0 {
                return 0;
            }
        }
        _ => {
            if (val & ARIZONA_HP_DONE_B) != 0 {
                return 0;
            }
        }
        }

        msleep(ARIZONA_HPDET_WAIT_DELAY_MS);
        i += 1;
    }

    dev_warn((*arizona).dev, c"HPDET did not appear to complete\n".as_ptr());
    -ETIMEDOUT
}

unsafe extern "C" fn arizona_jackdet(irq: i32, data: *mut c_void) -> irqreturn_t {
    let info = data as *mut arizona_priv;
    let arizona = (*info).arizona;
    let mut val: u32 = 0;
    let present: u32;
    let mask: u32;
    let cancelled_hp: bool;
    let cancelled_mic: bool;
    let mut ret: i32;
    let mut i: usize;

    cancelled_hp = cancel_delayed_work_sync(&mut (*info).hpdet_work);
    cancelled_mic = cancel_delayed_work_sync(&mut (*info).micd_timeout_work);

    pm_runtime_get_sync((*arizona).dev);
    mutex_lock(&mut (*info).lock);

    if (*info).micd_clamp {
        mask = ARIZONA_MICD_CLAMP_STS;
        present = 0;
    } else {
        mask = ARIZONA_JD1_STS;
        if (*arizona).pdata.jd_invert {
            present = 0;
        } else {
            present = ARIZONA_JD1_STS;
        }
    }

    ret = regmap_read((*arizona).regmap, ARIZONA_AOD_IRQ_RAW_STATUS, &mut val);
    if ret != 0 {
        dev_err((*arizona).dev, c"Failed to read jackdet status: %d\n".as_ptr(), ret);
        mutex_unlock(&mut (*info).lock);
        pm_runtime_put_autosuspend((*arizona).dev);
        return IRQ_NONE;
    }

    val &= mask;
    if val == (*info).last_jackdet {
        dev_dbg((*arizona).dev, c"Suppressing duplicate JACKDET\n".as_ptr());
        if cancelled_hp {
            queue_delayed_work(system_power_efficient_wq, &mut (*info).hpdet_work,
                               msecs_to_jiffies(HPDET_DEBOUNCE));
        }

        if cancelled_mic {
            let micd_timeout = (*arizona).pdata.micd_timeout;
            queue_delayed_work(system_power_efficient_wq, &mut (*info).micd_timeout_work,
                               msecs_to_jiffies(micd_timeout));
        }
    } else {
        (*info).last_jackdet = val;

        if (*info).last_jackdet == present {
            dev_dbg((*arizona).dev, c"Detected jack\n".as_ptr());
            snd_soc_jack_report((*info).jack, SND_JACK_MECHANICAL, SND_JACK_MECHANICAL);

            (*info).detecting = true;
            (*info).mic = false;
            (*info).jack_flips = 0;

            if !(*arizona).pdata.hpdet_acc_id {
                arizona_start_mic(info);
            } else {
                queue_delayed_work(system_power_efficient_wq, &mut (*info).hpdet_work,
                                   msecs_to_jiffies(HPDET_DEBOUNCE));
            }

            if (*info).micd_clamp || !(*arizona).pdata.jd_invert {
                regmap_update_bits((*arizona).regmap, ARIZONA_JACK_DETECT_DEBOUNCE,
                                   ARIZONA_MICD_CLAMP_DB | ARIZONA_JD1_DB, 0);
            }
        } else {
            dev_dbg((*arizona).dev, c"Detected jack removal\n".as_ptr());

            arizona_stop_mic(info);

            (*info).num_hpdet_res = 0;
            i = 0;
            while i < (*info).hpdet_res.len() {
                (*info).hpdet_res[i] = 0;
                i += 1;
            }
            (*info).mic = false;
            (*info).hpdet_done = false;
            (*info).hpdet_retried = false;

            snd_soc_jack_report((*info).jack, 0,
                                ARIZONA_JACK_MASK | (*info).micd_button_mask);

            /*
             * If the jack was removed during a headphone detection we
             * need to wait for the headphone detection to finish, as
             * it can not be aborted. We don't want to be able to start
             * a new headphone detection from a fresh insert until this
             * one is finished.
             */
            arizona_hpdet_wait(info);

            regmap_update_bits((*arizona).regmap, ARIZONA_JACK_DETECT_DEBOUNCE,
                               ARIZONA_MICD_CLAMP_DB | ARIZONA_JD1_DB,
                               ARIZONA_MICD_CLAMP_DB | ARIZONA_JD1_DB);
        }
    }

    /* Clear trig_sts to make sure DCVDD is not forced up */
    regmap_write((*arizona).regmap, ARIZONA_AOD_WKUP_AND_TRIG,
                 ARIZONA_MICD_CLAMP_FALL_TRIG_STS |
                 ARIZONA_MICD_CLAMP_RISE_TRIG_STS |
                 ARIZONA_JD1_FALL_TRIG_STS |
                 ARIZONA_JD1_RISE_TRIG_STS);

    mutex_unlock(&mut (*info).lock);
    pm_runtime_put_autosuspend((*arizona).dev);
    IRQ_HANDLED
}

/* Map a level onto a slot in the register bank */
unsafe fn arizona_micd_set_level(arizona: *mut arizona, index: i32, mut level: u32) {
    let reg: i32;
    let mask: u32;

    reg = ARIZONA_MIC_DETECT_LEVEL_4 - (index / 2);

    if (index % 2) == 0 {
        mask = 0x3f00;
        level <<= 8;
    } else {
        mask = 0x3f;
    }

    /* Program the level itself */
    regmap_update_bits((*arizona).regmap, reg, mask, level);
}

unsafe fn arizona_extcon_get_micd_configs(dev: *mut device, arizona: *mut arizona) -> i32 {
    let prop = c"wlf,micd-configs".as_ptr();
    let entries_per_config: i32 = 3;
    let micd_configs: *mut arizona_micd_config;
    let mut nconfs: i32;
    let mut ret: i32 = 0;
    let mut i: i32;
    let mut j: i32;
    let vals: *mut u32;

    nconfs = device_property_count_u32((*arizona).dev, prop);
    if nconfs <= 0 {
        return 0;
    }

    vals = kcalloc(nconfs as usize, core::mem::size_of::<u32>(), GFP_KERNEL) as *mut u32;
    if vals.is_null() {
        return -ENOMEM;
    }

    ret = device_property_read_u32_array((*arizona).dev, prop, vals, nconfs);
    if ret < 0 {
        kfree(vals as *mut c_void);
        return ret;
    }

    nconfs /= entries_per_config;
    micd_configs = devm_kcalloc(dev, nconfs as usize,
                                core::mem::size_of::<arizona_micd_config>(),
                                GFP_KERNEL) as *mut arizona_micd_config;
    if micd_configs.is_null() {
        ret = -ENOMEM;
        kfree(vals as *mut c_void);
        return ret;
    }

    i = 0;
    j = 0;
    while i < nconfs {
        (*micd_configs.add(i as usize)).src =
            if *vals.add(j as usize) != 0 { ARIZONA_ACCDET_SRC } else { 0 };
        j += 1;
        (*micd_configs.add(i as usize)).bias = *vals.add(j as usize);
        j += 1;
        (*micd_configs.add(i as usize)).gpio = *vals.add(j as usize);
        j += 1;
        i += 1;
    }

    (*arizona).pdata.micd_configs = micd_configs;
    (*arizona).pdata.num_micd_configs = nconfs;

    kfree(vals as *mut c_void);
    ret
}

unsafe fn arizona_extcon_device_get_pdata(dev: *mut device, arizona: *mut arizona) -> i32 {
    let pdata = &mut (*arizona).pdata as *mut arizona_pdata;
    let mut val: u32 = ARIZONA_ACCDET_MODE_HPL;
    let ret: i32;

    device_property_read_u32((*arizona).dev, c"wlf,hpdet-channel".as_ptr(), &mut val);
    match val {
    ARIZONA_ACCDET_MODE_HPL | ARIZONA_ACCDET_MODE_HPR => {
        (*pdata).hpdet_channel = val;
    }
    _ => {
        dev_err((*arizona).dev, c"Wrong wlf,hpdet-channel DT value %d\n".as_ptr(), val);
        (*pdata).hpdet_channel = ARIZONA_ACCDET_MODE_HPL;
    }
    }

    device_property_read_u32((*arizona).dev, c"wlf,micd-detect-debounce".as_ptr(),
                             &mut (*pdata).micd_detect_debounce);
    device_property_read_u32((*arizona).dev, c"wlf,micd-bias-start-time".as_ptr(),
                             &mut (*pdata).micd_bias_start_time);
    device_property_read_u32((*arizona).dev, c"wlf,micd-rate".as_ptr(),
                             &mut (*pdata).micd_rate);
    device_property_read_u32((*arizona).dev, c"wlf,micd-dbtime".as_ptr(),
                             &mut (*pdata).micd_dbtime);
    device_property_read_u32((*arizona).dev, c"wlf,micd-timeout-ms".as_ptr(),
                             &mut (*pdata).micd_timeout);

    (*pdata).micd_force_micbias =
        device_property_read_bool((*arizona).dev, c"wlf,micd-force-micbias".as_ptr());
    (*pdata).micd_software_compare =
        device_property_read_bool((*arizona).dev, c"wlf,micd-software-compare".as_ptr());
    (*pdata).jd_invert =
        device_property_read_bool((*arizona).dev, c"wlf,jd-invert".as_ptr());

    device_property_read_u32((*arizona).dev, c"wlf,gpsw".as_ptr(), &mut (*pdata).gpsw);

    (*pdata).jd_gpio5 = device_property_read_bool((*arizona).dev, c"wlf,use-jd2".as_ptr());
    (*pdata).jd_gpio5_nopull =
        device_property_read_bool((*arizona).dev, c"wlf,use-jd2-nopull".as_ptr());

    ret = arizona_extcon_get_micd_configs(dev, arizona);
    if ret < 0 {
        dev_err((*arizona).dev, c"Failed to read micd configs: %d\n".as_ptr(), ret);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn arizona_jack_codec_dev_probe(info: *mut arizona_priv,
                                                       dev: *mut device) -> i32 {
    let arizona = (*info).arizona;
    let pdata = &mut (*arizona).pdata as *mut arizona_pdata;
    let mut ret: i32;
    let mode: i32;

    if dev_get_platdata((*arizona).dev).is_null() {
        arizona_extcon_device_get_pdata(dev, arizona);
    }

    (*info).micvdd = devm_regulator_get(dev, c"MICVDD".as_ptr());
    if IS_ERR((*info).micvdd) {
        return dev_err_probe((*arizona).dev, PTR_ERR((*info).micvdd),
                             c"getting MICVDD\n".as_ptr());
    }

    mutex_init(&mut (*info).lock);
    (*info).last_jackdet = !(ARIZONA_MICD_CLAMP_STS | ARIZONA_JD1_STS);
    INIT_DELAYED_WORK(&mut (*info).hpdet_work, Some(arizona_hpdet_work));
    INIT_DELAYED_WORK(&mut (*info).micd_detect_work, Some(arizona_micd_detect));
    INIT_DELAYED_WORK(&mut (*info).micd_timeout_work, Some(arizona_micd_timeout_work));

    match (*arizona).type_ {
    WM5102 => {
        match (*arizona).rev {
        0 => (*info).micd_reva = true,
        _ => {
            (*info).micd_clamp = true;
            (*info).hpdet_ip_version = 1;
        }
        }
    }
    WM5110 | WM8280 => {
        match (*arizona).rev {
        0..=2 => {}
        _ => {
            (*info).micd_clamp = true;
            (*info).hpdet_ip_version = 2;
        }
        }
    }
    WM8998 | WM1814 => {
        (*info).micd_clamp = true;
        (*info).hpdet_ip_version = 2;
    }
    _ => {}
    }

    if (*pdata).micd_timeout == 0 {
        (*pdata).micd_timeout = DEFAULT_MICD_TIMEOUT;
    }

    if (*pdata).num_micd_configs != 0 {
        (*info).micd_modes = (*pdata).micd_configs;
        (*info).micd_num_modes = (*pdata).num_micd_configs;
    } else {
        (*info).micd_modes = micd_default_modes.as_ptr() as *mut arizona_micd_config;
        (*info).micd_num_modes = micd_default_modes.len() as i32;
    }

    if (*arizona).pdata.gpsw > 0 {
        regmap_update_bits((*arizona).regmap, ARIZONA_GP_SWITCH_1,
                           ARIZONA_SW1_MODE_MASK, (*arizona).pdata.gpsw);
    }

    if (*(*info).micd_modes).gpio != 0 {
        mode = GPIOD_OUT_HIGH;
    } else {
        mode = GPIOD_OUT_LOW;
    }

    /* We can't use devm here because we need to do the get
     * against the MFD device, as that is where the of_node
     * will reside, but if we devm against that the GPIO
     * will not be freed if the extcon driver is unloaded.
     */
    (*info).micd_pol_gpio = gpiod_get_optional((*arizona).dev, c"wlf,micd-pol".as_ptr(), mode);
    if IS_ERR((*info).micd_pol_gpio) {
        ret = PTR_ERR((*info).micd_pol_gpio);
        dev_err_probe((*arizona).dev, ret, c"getting microphone polarity GPIO\n".as_ptr());
        return ret;
    }

    (*info).hpdet_id_gpio = gpiod_get_optional((*arizona).dev, c"wlf,hpdet-id-gpio".as_ptr(), mode);
    if IS_ERR((*info).hpdet_id_gpio) {
        ret = PTR_ERR((*info).hpdet_id_gpio);
        dev_err_probe((*arizona).dev, ret, c"getting headphone detect ID GPIO\n".as_ptr());
        return ret;
    }

    0
}
// EXPORT_SYMBOL_GPL(arizona_jack_codec_dev_probe);

#[no_mangle]
pub unsafe extern "C" fn arizona_jack_codec_dev_remove(info: *mut arizona_priv) -> i32 {
    gpiod_put((*info).micd_pol_gpio);
    gpiod_put((*info).hpdet_id_gpio);
    0
}
// EXPORT_SYMBOL_GPL(arizona_jack_codec_dev_remove);

unsafe fn arizona_jack_enable_jack_detect(info: *mut arizona_priv,
                                          jack: *mut snd_soc_jack) -> i32 {
    let arizona = (*info).arizona;
    let pdata = &mut (*arizona).pdata as *mut arizona_pdata;
    let mut val: u32;
    let clamp_mode: u32;
    let jack_irq_fall: i32;
    let jack_irq_rise: i32;
    let mut ret: i32;
    let mut i: i32;
    let mut j: usize;

    if (*arizona).pdata.micd_bias_start_time != 0 {
        regmap_update_bits((*arizona).regmap, ARIZONA_MIC_DETECT_1,
                           ARIZONA_MICD_BIAS_STARTTIME_MASK,
                           (*arizona).pdata.micd_bias_start_time <<
                           ARIZONA_MICD_BIAS_STARTTIME_SHIFT);
    }

    if (*arizona).pdata.micd_rate != 0 {
        regmap_update_bits((*arizona).regmap, ARIZONA_MIC_DETECT_1,
                           ARIZONA_MICD_RATE_MASK,
                           (*arizona).pdata.micd_rate << ARIZONA_MICD_RATE_SHIFT);
    }

    match (*arizona).pdata.micd_dbtime {
    MICD_DBTIME_FOUR_READINGS => {
        regmap_update_bits((*arizona).regmap, ARIZONA_MIC_DETECT_1,
                           ARIZONA_MICD_DBTIME_MASK, ARIZONA_MICD_DBTIME);
    }
    MICD_DBTIME_TWO_READINGS => {
        regmap_update_bits((*arizona).regmap, ARIZONA_MIC_DETECT_1,
                           ARIZONA_MICD_DBTIME_MASK, 0);
    }
    _ => {}
    }

    /* BUILD_BUG_ON(ARRAY_SIZE(arizona_micd_levels) < ARIZONA_NUM_MICD_BUTTON_LEVELS); */

    if (*arizona).pdata.num_micd_ranges != 0 {
        (*info).micd_ranges = (*pdata).micd_ranges;
        (*info).num_micd_ranges = (*pdata).num_micd_ranges;
    } else {
        (*info).micd_ranges = micd_default_ranges.as_ptr() as *mut arizona_micd_range;
        (*info).num_micd_ranges = micd_default_ranges.len() as i32;
    }

    if (*arizona).pdata.num_micd_ranges > ARIZONA_MAX_MICD_BUTTONS {
        dev_err((*arizona).dev, c"Too many MICD ranges: %d > %d\n".as_ptr(),
                (*arizona).pdata.num_micd_ranges, ARIZONA_MAX_MICD_BUTTONS);
        return -EINVAL;
    }

    if (*info).num_micd_ranges > 1 {
        i = 1;
        while i < (*info).num_micd_ranges {
            if (*(*info).micd_ranges.add((i - 1) as usize)).max >
               (*(*info).micd_ranges.add(i as usize)).max {
                dev_err((*arizona).dev, c"MICD ranges must be sorted\n".as_ptr());
                return -EINVAL;
            }
            i += 1;
        }
    }

    /* Disable all buttons by default */
    regmap_update_bits((*arizona).regmap, ARIZONA_MIC_DETECT_2,
                       ARIZONA_MICD_LVL_SEL_MASK, 0x81);

    /* Set up all the buttons the user specified */
    i = 0;
    while i < (*info).num_micd_ranges {
        j = 0;
        while j < ARIZONA_NUM_MICD_BUTTON_LEVELS {
            if arizona_micd_levels[j] >= (*(*info).micd_ranges.add(i as usize)).max {
                break;
            }
            j += 1;
        }

        if j == ARIZONA_NUM_MICD_BUTTON_LEVELS {
            dev_err((*arizona).dev, c"Unsupported MICD level %d\n".as_ptr(),
                    (*(*info).micd_ranges.add(i as usize)).max);
            return -EINVAL;
        }

        dev_dbg((*arizona).dev, c"%d ohms for MICD threshold %d\n".as_ptr(),
                arizona_micd_levels[j], i);

        arizona_micd_set_level(arizona, i, j as u32);

        /* SND_JACK_BTN_# masks start with the most significant bit */
        (*info).micd_button_mask |= SND_JACK_BTN_0 >> i;
        snd_jack_set_key((*jack).jack, SND_JACK_BTN_0 >> i,
                         (*(*info).micd_ranges.add(i as usize)).key);

        /* Enable reporting of that range */
        regmap_update_bits((*arizona).regmap, ARIZONA_MIC_DETECT_2,
                           1 << i, 1 << i);
        i += 1;
    }

    /* Set all the remaining keys to a maximum */
    while i < ARIZONA_MAX_MICD_RANGE as i32 {
        arizona_micd_set_level(arizona, i, 0x3f);
        i += 1;
    }

    /*
     * If we have a clamp use it, activating in conjunction with
     * GPIO5 if that is connected for jack detect operation.
     */
    if (*info).micd_clamp {
        if (*arizona).pdata.jd_gpio5 {
            /* Put the GPIO into input mode with optional pull */
            val = 0xc101;
            if (*arizona).pdata.jd_gpio5_nopull {
                val &= !ARIZONA_GPN_PU;
            }

            regmap_write((*arizona).regmap, ARIZONA_GPIO5_CTRL, val);

            if (*arizona).pdata.jd_invert {
                clamp_mode = ARIZONA_MICD_CLAMP_MODE_JDH_GP5H;
            } else {
                clamp_mode = ARIZONA_MICD_CLAMP_MODE_JDL_GP5H;
            }
        } else {
            if (*arizona).pdata.jd_invert {
                clamp_mode = ARIZONA_MICD_CLAMP_MODE_JDH;
            } else {
                clamp_mode = ARIZONA_MICD_CLAMP_MODE_JDL;
            }
        }

        regmap_update_bits((*arizona).regmap, ARIZONA_MICD_CLAMP_CONTROL,
                           ARIZONA_MICD_CLAMP_MODE_MASK, clamp_mode);

        regmap_update_bits((*arizona).regmap, ARIZONA_JACK_DETECT_DEBOUNCE,
                           ARIZONA_MICD_CLAMP_DB, ARIZONA_MICD_CLAMP_DB);
    }

    arizona_extcon_set_mode(info, 0);
    (*info).jack = jack;

    pm_runtime_get_sync((*arizona).dev);

    if (*info).micd_clamp {
        jack_irq_rise = ARIZONA_IRQ_MICD_CLAMP_RISE;
        jack_irq_fall = ARIZONA_IRQ_MICD_CLAMP_FALL;
    } else {
        jack_irq_rise = ARIZONA_IRQ_JD_RISE;
        jack_irq_fall = ARIZONA_IRQ_JD_FALL;
    }

    ret = arizona_request_irq(arizona, jack_irq_rise,
                              c"JACKDET rise".as_ptr(), Some(arizona_jackdet), info as *mut c_void);
    if ret != 0 {
        dev_err((*arizona).dev, c"Failed to get JACKDET rise IRQ: %d\n".as_ptr(), ret);
        pm_runtime_put((*arizona).dev);
        (*info).jack = core::ptr::null_mut();
        return ret;
    }

    ret = arizona_set_irq_wake(arizona, jack_irq_rise, 1);
    if ret != 0 {
        dev_err((*arizona).dev, c"Failed to set JD rise IRQ wake: %d\n".as_ptr(), ret);
        arizona_free_irq(arizona, jack_irq_rise, info as *mut c_void);
        pm_runtime_put((*arizona).dev);
        (*info).jack = core::ptr::null_mut();
        return ret;
    }

    ret = arizona_request_irq(arizona, jack_irq_fall,
                              c"JACKDET fall".as_ptr(), Some(arizona_jackdet), info as *mut c_void);
    if ret != 0 {
        dev_err((*arizona).dev, c"Failed to get JD fall IRQ: %d\n".as_ptr(), ret);
        arizona_set_irq_wake(arizona, jack_irq_rise, 0);
        arizona_free_irq(arizona, jack_irq_rise, info as *mut c_void);
        pm_runtime_put((*arizona).dev);
        (*info).jack = core::ptr::null_mut();
        return ret;
    }

    ret = arizona_set_irq_wake(arizona, jack_irq_fall, 1);
    if ret != 0 {
        dev_err((*arizona).dev, c"Failed to set JD fall IRQ wake: %d\n".as_ptr(), ret);
        arizona_free_irq(arizona, jack_irq_fall, info as *mut c_void);
        arizona_set_irq_wake(arizona, jack_irq_rise, 0);
        arizona_free_irq(arizona, jack_irq_rise, info as *mut c_void);
        pm_runtime_put((*arizona).dev);
        (*info).jack = core::ptr::null_mut();
        return ret;
    }

    ret = arizona_request_irq(arizona, ARIZONA_IRQ_MICDET,
                              c"MICDET".as_ptr(), Some(arizona_micdet), info as *mut c_void);
    if ret != 0 {
        dev_err((*arizona).dev, c"Failed to get MICDET IRQ: %d\n".as_ptr(), ret);
        arizona_set_irq_wake(arizona, jack_irq_fall, 0);
        arizona_free_irq(arizona, jack_irq_fall, info as *mut c_void);
        arizona_set_irq_wake(arizona, jack_irq_rise, 0);
        arizona_free_irq(arizona, jack_irq_rise, info as *mut c_void);
        pm_runtime_put((*arizona).dev);
        (*info).jack = core::ptr::null_mut();
        return ret;
    }

    ret = arizona_request_irq(arizona, ARIZONA_IRQ_HPDET,
                              c"HPDET".as_ptr(), Some(arizona_hpdet_irq), info as *mut c_void);
    if ret != 0 {
        dev_err((*arizona).dev, c"Failed to get HPDET IRQ: %d\n".as_ptr(), ret);
        arizona_free_irq(arizona, ARIZONA_IRQ_MICDET, info as *mut c_void);
        arizona_set_irq_wake(arizona, jack_irq_fall, 0);
        arizona_free_irq(arizona, jack_irq_fall, info as *mut c_void);
        arizona_set_irq_wake(arizona, jack_irq_rise, 0);
        arizona_free_irq(arizona, jack_irq_rise, info as *mut c_void);
        pm_runtime_put((*arizona).dev);
        (*info).jack = core::ptr::null_mut();
        return ret;
    }

    arizona_clk32k_enable(arizona);
    regmap_update_bits((*arizona).regmap, ARIZONA_JACK_DETECT_DEBOUNCE,
                       ARIZONA_JD1_DB, ARIZONA_JD1_DB);
    regmap_update_bits((*arizona).regmap, ARIZONA_JACK_DETECT_ANALOGUE,
                       ARIZONA_JD1_ENA, ARIZONA_JD1_ENA);

    ret = regulator_allow_bypass((*info).micvdd, true);
    if ret != 0 {
        dev_warn((*arizona).dev, c"Failed to set MICVDD to bypass: %d\n".as_ptr(), ret);
    }

    pm_runtime_put((*arizona).dev);
    0
}

unsafe fn arizona_jack_disable_jack_detect(info: *mut arizona_priv) -> i32 {
    let arizona = (*info).arizona;
    let jack_irq_rise: i32;
    let jack_irq_fall: i32;
    let mut change: bool = false;
    let ret: i32;

    if (*info).jack.is_null() {
        return 0;
    }

    if (*info).micd_clamp {
        jack_irq_rise = ARIZONA_IRQ_MICD_CLAMP_RISE;
        jack_irq_fall = ARIZONA_IRQ_MICD_CLAMP_FALL;
    } else {
        jack_irq_rise = ARIZONA_IRQ_JD_RISE;
        jack_irq_fall = ARIZONA_IRQ_JD_FALL;
    }

    arizona_set_irq_wake(arizona, jack_irq_rise, 0);
    arizona_set_irq_wake(arizona, jack_irq_fall, 0);
    arizona_free_irq(arizona, ARIZONA_IRQ_HPDET, info as *mut c_void);
    arizona_free_irq(arizona, ARIZONA_IRQ_MICDET, info as *mut c_void);
    arizona_free_irq(arizona, jack_irq_rise, info as *mut c_void);
    arizona_free_irq(arizona, jack_irq_fall, info as *mut c_void);
    cancel_delayed_work_sync(&mut (*info).hpdet_work);
    cancel_delayed_work_sync(&mut (*info).micd_detect_work);
    cancel_delayed_work_sync(&mut (*info).micd_timeout_work);

    ret = regmap_update_bits_check((*arizona).regmap, ARIZONA_MIC_DETECT_1,
                                   ARIZONA_MICD_ENA, 0, &mut change);
    if ret < 0 {
        dev_err((*arizona).dev, c"Failed to disable micd on remove: %d\n".as_ptr(), ret);
    } else if change {
        regulator_disable((*info).micvdd);
        pm_runtime_put((*arizona).dev);
    }

    regmap_update_bits((*arizona).regmap, ARIZONA_MICD_CLAMP_CONTROL,
                       ARIZONA_MICD_CLAMP_MODE_MASK, 0);
    regmap_update_bits((*arizona).regmap, ARIZONA_JACK_DETECT_ANALOGUE,
                       ARIZONA_JD1_ENA, 0);
    arizona_clk32k_disable(arizona);
    (*info).jack = core::ptr::null_mut();

    0
}

#[no_mangle]
pub unsafe extern "C" fn arizona_jack_set_jack(component: *mut snd_soc_component,
                                                jack: *mut snd_soc_jack,
                                                data: *mut c_void) -> i32 {
    let info = snd_soc_component_get_drvdata(component) as *mut arizona_priv;

    if !jack.is_null() {
        arizona_jack_enable_jack_detect(info, jack)
    } else {
        arizona_jack_disable_jack_detect(info)
    }
}
// EXPORT_SYMBOL_GPL(arizona_jack_set_jack);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
