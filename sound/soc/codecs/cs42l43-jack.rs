// SPDX-License-Identifier: GPL-2.0
//
// CS42L43 CODEC driver jack handling
//
// Copyright (C) 2022-2023 Cirrus Logic, Inc. and
//                         Cirrus Logic International Semiconductor Ltd.

// Rust translation of implementation source ./soc/codecs/cs42l43-jack.c.
// Original C includes:
// linux/build_bug.h, cleanup.h, completion.h, delay.h, errno.h, irq.h,
// jiffies.h, mfd/cs42l43.h, mfd/cs42l43-regs.h, mutex.h, pm_runtime.h,
// property.h, regmap.h, time.h, workqueue.h, sound/control.h, sound/jack.h,
// sound/pcm.h, sound/pcm_params.h, sound/soc-component.h, sound/soc-jack.h,
// sound/soc.h, and local "cs42l43.h".

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const cs42l43_accdet_us: [c_uint; 8] = [
    20, 100, 1000, 10000, 50000, 75000, 100000, 200000,
];

const cs42l43_accdet_db_ms: [c_uint; 8] = [
    0, 125, 250, 500, 750, 1000, 1250, 1500,
];

const cs42l43_accdet_ramp_ms: [c_uint; 4] = [10, 40, 90, 170];

const cs42l43_accdet_bias_sense: [c_uint; 9] = [
    14, 24, 43, 52, 61, 71, 90, 99, 0,
];

unsafe fn cs42l43_find_index(
    priv_: *mut cs42l43_codec,
    prop: *const c_char,
    mut defval: c_uint,
    val: *mut c_uint,
    values: *const c_uint,
    nvalues: c_int,
) -> c_int {
    let cs42l43 = (*priv_).core;
    let mut i: c_int;
    let ret: c_int;

    ret = device_property_read_u32((*cs42l43).dev, prop, &mut defval);
    if ret != -EINVAL && ret < 0 {
        dev_err((*priv_).dev, c_str!("Property %s malformed: %d\n"), prop, ret);
        return ret;
    }

    if !val.is_null() {
        *val = defval;
    }

    i = 0;
    while i < nvalues {
        if defval == *values.add(i as usize) {
            return i;
        }
        i += 1;
    }

    dev_err(
        (*priv_).dev,
        c_str!("Invalid value for property %s: %d\n"),
        prop,
        defval,
    );
    -EINVAL
}

unsafe fn cs42l43_apply_accdet_config(
    priv_: *mut cs42l43_codec,
    autocontrol: c_uint,
    pdncntl: c_uint,
) {
    let cs42l43 = (*priv_).core;

    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_HS_BIAS_SENSE_AND_CLAMP_AUTOCONTROL,
        CS42L43_JACKDET_MODE_MASK
            | CS42L43_S0_AUTO_ADCMUTE_DISABLE_MASK
            | CS42L43_HSBIAS_SENSE_TRIP_MASK,
        autocontrol,
    );
    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_PDNCNTL,
        CS42L43_RING_SENSE_EN_MASK,
        pdncntl,
    );

    dev_dbg((*priv_).dev, c_str!("Successfully configured accessory detect\n"));
}

#[no_mangle]
pub unsafe extern "C" fn cs42l43_set_jack(
    component: *mut snd_soc_component,
    jack: *mut snd_soc_jack,
    d: *mut c_void,
) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs42l43_codec;
    let cs42l43 = (*priv_).core;
    /* This tip sense invert is always set, HW wants an inverted signal */
    let mut tip_deb: c_uint = CS42L43_TIPSENSE_INV_MASK;
    let mut hs2: c_uint = 0x2 << CS42L43_HSDET_MODE_SHIFT;
    let mut autocontrol: c_uint = 0;
    let mut pdncntl: c_uint = 0;
    let mut ret: c_int;

    let _ = d;

    dev_dbg((*priv_).dev, c_str!("Configure accessory detect\n"));

    PM_RUNTIME_ACQUIRE_IF_ENABLED_AUTOSUSPEND((*priv_).dev, pm);
    ret = PM_RUNTIME_ACQUIRE_ERR(&pm);
    if ret != 0 {
        dev_err(
            (*priv_).dev,
            c_str!("Failed to resume for jack config: %d\n"),
            ret,
        );
        return ret;
    }

    let _guard = mutex_guard(&mut (*priv_).jack_lock);

    (*priv_).jack_hp = jack;

    if jack.is_null() {
        cs42l43_apply_accdet_config(priv_, autocontrol, pdncntl);
        return 0;
    }

    ret = device_property_count_u32((*cs42l43).dev, c_str!("cirrus,buttons-ohms"));
    if ret != -EINVAL {
        if ret < 0 {
            dev_err(
                (*priv_).dev,
                c_str!("Property cirrus,buttons-ohms malformed: %d\n"),
                ret,
            );
            return ret;
        }

        if ret > CS42L43_N_BUTTONS {
            ret = -EINVAL;
            dev_err(
                (*priv_).dev,
                c_str!("Property cirrus,buttons-ohms too many entries\n"),
            );
            return ret;
        }

        ret = device_property_read_u32_array(
            (*cs42l43).dev,
            c_str!("cirrus,buttons-ohms"),
            (*priv_).buttons.as_mut_ptr(),
            ret,
        );
        if ret < 0 {
            dev_err(
                (*priv_).dev,
                c_str!("Property cirrus,button-ohms malformed: %d\n"),
                ret,
            );
            return ret;
        }
    } else {
        (*priv_).buttons[0] = 70;
        (*priv_).buttons[1] = 185;
        (*priv_).buttons[2] = 355;
        (*priv_).buttons[3] = 735;
    }

    ret = cs42l43_find_index(
        priv_,
        c_str!("cirrus,detect-us"),
        50000,
        &mut (*priv_).detect_us,
        cs42l43_accdet_us.as_ptr(),
        cs42l43_accdet_us.len() as c_int,
    );
    if ret < 0 {
        return ret;
    }

    hs2 |= (ret as c_uint) << CS42L43_AUTO_HSDET_TIME_SHIFT;

    (*priv_).bias_low = device_property_read_bool((*cs42l43).dev, c_str!("cirrus,bias-low"));

    ret = cs42l43_find_index(
        priv_,
        c_str!("cirrus,bias-ramp-ms"),
        170,
        &mut (*priv_).bias_ramp_ms,
        cs42l43_accdet_ramp_ms.as_ptr(),
        cs42l43_accdet_ramp_ms.len() as c_int,
    );
    if ret < 0 {
        return ret;
    }

    hs2 |= (ret as c_uint) << CS42L43_HSBIAS_RAMP_SHIFT;

    ret = cs42l43_find_index(
        priv_,
        c_str!("cirrus,bias-sense-microamp"),
        14,
        &mut (*priv_).bias_sense_ua,
        cs42l43_accdet_bias_sense.as_ptr(),
        cs42l43_accdet_bias_sense.len() as c_int,
    );
    if ret < 0 {
        return ret;
    }

    if (*priv_).bias_sense_ua != 0 {
        autocontrol |= (ret as c_uint) << CS42L43_HSBIAS_SENSE_TRIP_SHIFT;
    }

    if !device_property_read_bool((*cs42l43).dev, c_str!("cirrus,button-automute")) {
        autocontrol |= CS42L43_S0_AUTO_ADCMUTE_DISABLE_MASK;
    }

    ret = device_property_read_u32(
        (*cs42l43).dev,
        c_str!("cirrus,tip-debounce-ms"),
        &mut (*priv_).tip_debounce_ms,
    );
    if ret < 0 && ret != -EINVAL {
        dev_err(
            (*priv_).dev,
            c_str!("Property cirrus,tip-debounce-ms malformed: %d\n"),
            ret,
        );
        return ret;
    }

    /* This tip sense invert is set normally, as TIPSENSE_INV already inverted */
    if device_property_read_bool((*cs42l43).dev, c_str!("cirrus,tip-invert")) {
        autocontrol |= 0x1 << CS42L43_JACKDET_INV_SHIFT;
    }

    if device_property_read_bool((*cs42l43).dev, c_str!("cirrus,tip-disable-pullup")) {
        autocontrol |= 0x1 << CS42L43_JACKDET_MODE_SHIFT;
    } else {
        autocontrol |= 0x3 << CS42L43_JACKDET_MODE_SHIFT;
    }

    ret = cs42l43_find_index(
        priv_,
        c_str!("cirrus,tip-fall-db-ms"),
        500,
        &mut (*priv_).tip_fall_db_ms,
        cs42l43_accdet_db_ms.as_ptr(),
        cs42l43_accdet_db_ms.len() as c_int,
    );
    if ret < 0 {
        return ret;
    }

    tip_deb |= (ret as c_uint) << CS42L43_TIPSENSE_FALLING_DB_TIME_SHIFT;

    ret = cs42l43_find_index(
        priv_,
        c_str!("cirrus,tip-rise-db-ms"),
        500,
        &mut (*priv_).tip_rise_db_ms,
        cs42l43_accdet_db_ms.as_ptr(),
        cs42l43_accdet_db_ms.len() as c_int,
    );
    if ret < 0 {
        return ret;
    }

    tip_deb |= (ret as c_uint) << CS42L43_TIPSENSE_RISING_DB_TIME_SHIFT;

    if device_property_read_bool((*cs42l43).dev, c_str!("cirrus,use-ring-sense")) {
        let mut ring_deb: c_uint = 0;

        (*priv_).use_ring_sense = true;

        /* HW wants an inverted signal, so invert the invert */
        if !device_property_read_bool((*cs42l43).dev, c_str!("cirrus,ring-invert")) {
            ring_deb |= CS42L43_RINGSENSE_INV_MASK;
        }

        if !device_property_read_bool((*cs42l43).dev, c_str!("cirrus,ring-disable-pullup")) {
            ring_deb |= CS42L43_RINGSENSE_PULLUP_PDNB_MASK;
        }

        ret = cs42l43_find_index(
            priv_,
            c_str!("cirrus,ring-fall-db-ms"),
            500,
            core::ptr::null_mut(),
            cs42l43_accdet_db_ms.as_ptr(),
            cs42l43_accdet_db_ms.len() as c_int,
        );
        if ret < 0 {
            return ret;
        }

        ring_deb |= (ret as c_uint) << CS42L43_RINGSENSE_FALLING_DB_TIME_SHIFT;

        ret = cs42l43_find_index(
            priv_,
            c_str!("cirrus,ring-rise-db-ms"),
            500,
            core::ptr::null_mut(),
            cs42l43_accdet_db_ms.as_ptr(),
            cs42l43_accdet_db_ms.len() as c_int,
        );
        if ret < 0 {
            return ret;
        }

        ring_deb |= (ret as c_uint) << CS42L43_RINGSENSE_RISING_DB_TIME_SHIFT;
        pdncntl |= CS42L43_RING_SENSE_EN_MASK;

        regmap_update_bits(
            (*cs42l43).regmap,
            CS42L43_RINGSENSE_DEB_CTRL,
            CS42L43_RINGSENSE_INV_MASK
                | CS42L43_RINGSENSE_PULLUP_PDNB_MASK
                | CS42L43_RINGSENSE_FALLING_DB_TIME_MASK
                | CS42L43_RINGSENSE_RISING_DB_TIME_MASK,
            ring_deb,
        );
    }

    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_TIPSENSE_DEB_CTRL,
        CS42L43_TIPSENSE_INV_MASK
            | CS42L43_TIPSENSE_FALLING_DB_TIME_MASK
            | CS42L43_TIPSENSE_RISING_DB_TIME_MASK,
        tip_deb,
    );
    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_HS2,
        CS42L43_HSBIAS_RAMP_MASK | CS42L43_HSDET_MODE_MASK | CS42L43_AUTO_HSDET_TIME_MASK,
        hs2,
    );

    cs42l43_apply_accdet_config(priv_, autocontrol, pdncntl);

    0
}

unsafe fn cs42l43_start_hs_bias(priv_: *mut cs42l43_codec, type_detect: bool) {
    let cs42l43 = (*priv_).core;
    let mut val: c_uint = 0x3 << CS42L43_HSBIAS_MODE_SHIFT;

    dev_dbg((*priv_).dev, c_str!("Start headset bias\n"));

    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_HS2,
        CS42L43_HS_CLAMP_DISABLE_MASK,
        CS42L43_HS_CLAMP_DISABLE_MASK,
    );

    if !type_detect {
        if (*priv_).bias_low {
            val = 0x2 << CS42L43_HSBIAS_MODE_SHIFT;
        }

        if (*priv_).bias_sense_ua != 0 {
            regmap_update_bits(
                (*cs42l43).regmap,
                CS42L43_HS_BIAS_SENSE_AND_CLAMP_AUTOCONTROL,
                CS42L43_HSBIAS_SENSE_EN_MASK | CS42L43_AUTO_HSBIAS_CLAMP_EN_MASK,
                CS42L43_HSBIAS_SENSE_EN_MASK | CS42L43_AUTO_HSBIAS_CLAMP_EN_MASK,
            );
        }
    }

    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_MIC_DETECT_CONTROL_1,
        CS42L43_HSBIAS_MODE_MASK,
        val,
    );

    msleep((*priv_).bias_ramp_ms);
}

unsafe fn cs42l43_stop_hs_bias(priv_: *mut cs42l43_codec) {
    let cs42l43 = (*priv_).core;

    dev_dbg((*priv_).dev, c_str!("Stop headset bias\n"));

    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_MIC_DETECT_CONTROL_1,
        CS42L43_HSBIAS_MODE_MASK,
        0x1 << CS42L43_HSBIAS_MODE_SHIFT,
    );

    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_HS2,
        CS42L43_HS_CLAMP_DISABLE_MASK,
        0,
    );

    if (*priv_).bias_sense_ua != 0 {
        regmap_update_bits(
            (*cs42l43).regmap,
            CS42L43_HS_BIAS_SENSE_AND_CLAMP_AUTOCONTROL,
            CS42L43_HSBIAS_SENSE_EN_MASK | CS42L43_AUTO_HSBIAS_CLAMP_EN_MASK,
            0,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn cs42l43_bias_detect_clamp(
    irq: c_int,
    data: *mut c_void,
) -> irqreturn_t {
    let priv_ = data as *mut cs42l43_codec;

    let _ = irq;

    queue_delayed_work(
        system_dfl_wq,
        &mut (*priv_).bias_sense_timeout,
        msecs_to_jiffies(1000),
    );

    IRQ_HANDLED
}

const CS42L43_JACK_PRESENT: c_uint = 0x3;
const CS42L43_JACK_ABSENT: c_uint = 0x0;

const CS42L43_JACK_OPTICAL: c_int = SND_JACK_MECHANICAL | SND_JACK_AVOUT;
const CS42L43_JACK_MICROPHONE: c_int = SND_JACK_MECHANICAL | SND_JACK_MICROPHONE;
const CS42L43_JACK_HEADPHONE: c_int = SND_JACK_MECHANICAL | SND_JACK_HEADPHONE;
const CS42L43_JACK_HEADSET: c_int = SND_JACK_MECHANICAL | SND_JACK_HEADSET;
const CS42L43_JACK_LINEOUT: c_int = SND_JACK_MECHANICAL | SND_JACK_LINEOUT;
const CS42L43_JACK_LINEIN: c_int = SND_JACK_MECHANICAL | SND_JACK_LINEIN;
const CS42L43_JACK_EXTENSION: c_int = SND_JACK_MECHANICAL;
const CS42L43_JACK_BUTTONS: c_int = SND_JACK_BTN_0
    | SND_JACK_BTN_1
    | SND_JACK_BTN_2
    | SND_JACK_BTN_3
    | SND_JACK_BTN_4
    | SND_JACK_BTN_5;

unsafe fn cs42l43_jack_present(priv_: *mut cs42l43_codec) -> bool {
    let cs42l43 = (*priv_).core;
    let mut sts: c_uint = 0;

    regmap_read(
        (*cs42l43).regmap,
        CS42L43_TIP_RING_SENSE_INTERRUPT_STATUS,
        &mut sts,
    );

    sts = (sts >> CS42L43_TIPSENSE_PLUG_DB_STS_SHIFT) & CS42L43_JACK_PRESENT;

    sts == CS42L43_JACK_PRESENT
}

unsafe fn cs42l43_start_button_detect(priv_: *mut cs42l43_codec) {
    let cs42l43 = (*priv_).core;
    let mut val: c_uint = 0x3 << CS42L43_BUTTON_DETECT_MODE_SHIFT;

    dev_dbg((*priv_).dev, c_str!("Start button detect\n"));

    (*priv_).button_detect_running = true;

    if (*priv_).bias_low {
        val = 0x1 << CS42L43_BUTTON_DETECT_MODE_SHIFT;
    }

    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_MIC_DETECT_CONTROL_1,
        CS42L43_BUTTON_DETECT_MODE_MASK | CS42L43_MIC_LVL_DET_DISABLE_MASK,
        val,
    );
}

unsafe fn cs42l43_stop_button_detect(priv_: *mut cs42l43_codec) {
    let cs42l43 = (*priv_).core;

    dev_dbg((*priv_).dev, c_str!("Stop button detect\n"));

    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_MIC_DETECT_CONTROL_1,
        CS42L43_BUTTON_DETECT_MODE_MASK | CS42L43_MIC_LVL_DET_DISABLE_MASK,
        CS42L43_MIC_LVL_DET_DISABLE_MASK,
    );

    (*priv_).button_detect_running = false;
}

const CS42L43_BUTTON_COMB_US: c_uint = 11000;
const CS42L43_BUTTON_COMB_MAX: c_uint = 512;
const CS42L43_BUTTON_ROUT: c_uint = 2210;

#[no_mangle]
pub unsafe extern "C" fn cs42l43_button_press(irq: c_int, data: *mut c_void) -> irqreturn_t {
    let priv_ = data as *mut cs42l43_codec;
    let cs42l43 = (*priv_).core;
    let mut buttons: c_uint = 0;
    let mut val: c_uint = 0;
    let mut i: c_int;
    let ret: c_int;

    let _ = irq;

    PM_RUNTIME_ACQUIRE_IF_ENABLED_AUTOSUSPEND((*priv_).dev, pm);
    ret = PM_RUNTIME_ACQUIRE_ERR(&pm);
    if ret != 0 {
        dev_err(
            (*priv_).dev,
            c_str!("Failed to resume for button press: %d\n"),
            ret,
        );
        return IRQ_NONE;
    }

    let _guard = mutex_guard(&mut (*priv_).jack_lock);

    if !(*priv_).button_detect_running {
        dev_dbg((*priv_).dev, c_str!("Spurious button press IRQ\n"));
        return IRQ_NONE;
    }

    // Wait for 2 full cycles of comb filter to ensure good reading
    usleep_range(
        2 * CS42L43_BUTTON_COMB_US,
        2 * CS42L43_BUTTON_COMB_US + 50,
    );

    regmap_read((*cs42l43).regmap, CS42L43_DETECT_STATUS_1, &mut val);

    /* Bail if jack removed, the button is irrelevant and likely invalid */
    if !cs42l43_jack_present(priv_) {
        dev_dbg((*priv_).dev, c_str!("Button ignored due to removal\n"));
        return IRQ_NONE;
    }

    if (val & CS42L43_HSBIAS_CLAMP_STS_MASK) != 0 {
        dev_dbg((*priv_).dev, c_str!("Button ignored due to bias sense\n"));
        return IRQ_NONE;
    }

    val = (val & CS42L43_HSDET_DC_STS_MASK) >> CS42L43_HSDET_DC_STS_SHIFT;
    val = ((CS42L43_BUTTON_COMB_MAX << 20) / (val + 1)) - (1 << 20);
    if val != 0 {
        val = (CS42L43_BUTTON_ROUT << 20) / val;
    } else {
        val = UINT_MAX;
    }

    i = 0;
    while i < CS42L43_N_BUTTONS {
        if val < (*priv_).buttons[i as usize] {
            buttons = (SND_JACK_BTN_0 as c_uint) >> i;
            dev_dbg(
                (*priv_).dev,
                c_str!("Detected button %d at %d Ohms\n"),
                i,
                val,
            );
            break;
        }
        i += 1;
    }

    if buttons == 0 {
        dev_dbg((*priv_).dev, c_str!("Unrecognised button: %d Ohms\n"), val);
    }

    snd_soc_jack_report((*priv_).jack_hp, buttons as c_int, CS42L43_JACK_BUTTONS);

    IRQ_HANDLED
}

#[no_mangle]
pub unsafe extern "C" fn cs42l43_button_release(irq: c_int, data: *mut c_void) -> irqreturn_t {
    let priv_ = data as *mut cs42l43_codec;
    let ret: c_int;

    let _ = irq;

    PM_RUNTIME_ACQUIRE_IF_ENABLED_AUTOSUSPEND((*priv_).dev, pm);
    ret = PM_RUNTIME_ACQUIRE_ERR(&pm);
    if ret != 0 {
        dev_err(
            (*priv_).dev,
            c_str!("Failed to resume for button release: %d\n"),
            ret,
        );
        return IRQ_NONE;
    }

    let _guard = mutex_guard(&mut (*priv_).jack_lock);

    if !(*priv_).button_detect_running {
        dev_dbg((*priv_).dev, c_str!("Spurious button release IRQ\n"));
        return IRQ_NONE;
    }

    dev_dbg((*priv_).dev, c_str!("Button release IRQ\n"));
    snd_soc_jack_report((*priv_).jack_hp, 0, CS42L43_JACK_BUTTONS);

    IRQ_HANDLED
}

#[no_mangle]
pub unsafe extern "C" fn cs42l43_bias_sense_timeout(work: *mut work_struct) {
    let priv_ = container_of!(
        work,
        cs42l43_codec,
        bias_sense_timeout.work
    );
    let cs42l43 = (*priv_).core;
    let ret: c_int;

    PM_RUNTIME_ACQUIRE_IF_ENABLED_AUTOSUSPEND((*priv_).dev, pm);
    ret = PM_RUNTIME_ACQUIRE_ERR(&pm);
    if ret != 0 {
        dev_err(
            (*priv_).dev,
            c_str!("Failed to resume for bias sense: %d\n"),
            ret,
        );
        return;
    }

    let _guard = mutex_guard(&mut (*priv_).jack_lock);

    if cs42l43_jack_present(priv_) && (*priv_).button_detect_running {
        dev_dbg((*priv_).dev, c_str!("Bias sense timeout out, restore bias\n"));

        regmap_update_bits(
            (*cs42l43).regmap,
            CS42L43_HS_BIAS_SENSE_AND_CLAMP_AUTOCONTROL,
            CS42L43_AUTO_HSBIAS_CLAMP_EN_MASK,
            0,
        );
        regmap_update_bits(
            (*cs42l43).regmap,
            CS42L43_HS_BIAS_SENSE_AND_CLAMP_AUTOCONTROL,
            CS42L43_AUTO_HSBIAS_CLAMP_EN_MASK,
            CS42L43_AUTO_HSBIAS_CLAMP_EN_MASK,
        );
    }
}

const cs42l43_3pole_patch: [reg_sequence; 5] = [
    reg_sequence { reg: 0x4000, def: 0x00000055 },
    reg_sequence { reg: 0x4000, def: 0x000000AA },
    reg_sequence { reg: 0x17420, def: 0x8500F300 },
    reg_sequence { reg: 0x17424, def: 0x36003E00 },
    reg_sequence { reg: 0x4000, def: 0x00000000 },
];

const cs42l43_4pole_patch: [reg_sequence; 5] = [
    reg_sequence { reg: 0x4000, def: 0x00000055 },
    reg_sequence { reg: 0x4000, def: 0x000000AA },
    reg_sequence { reg: 0x17420, def: 0x7800E600 },
    reg_sequence { reg: 0x17424, def: 0x36003800 },
    reg_sequence { reg: 0x4000, def: 0x00000000 },
];

unsafe fn cs42l43_start_load_detect(priv_: *mut cs42l43_codec, mic: bool) {
    let cs42l43 = (*priv_).core;

    dev_dbg((*priv_).dev, c_str!("Start load detect\n"));

    snd_soc_dapm_mutex_lock(snd_soc_component_to_dapm((*priv_).component));

    (*priv_).load_detect_running = true;

    if (*priv_).hp_ena != 0 && !(*priv_).hp_ilimited {
        let time_left: c_ulong;

        reinit_completion(&mut (*priv_).hp_shutdown);

        regmap_update_bits(
            (*cs42l43).regmap,
            CS42L43_BLOCK_EN8,
            CS42L43_HP_EN_MASK,
            0,
        );

        time_left = wait_for_completion_timeout(
            &mut (*priv_).hp_shutdown,
            msecs_to_jiffies(CS42L43_HP_TIMEOUT_MS),
        );
        if time_left == 0 {
            dev_err((*priv_).dev, c_str!("Load detect HP power down timed out\n"));
        }
    }

    if mic {
        regmap_multi_reg_write_bypassed(
            (*cs42l43).regmap,
            cs42l43_4pole_patch.as_ptr(),
            cs42l43_4pole_patch.len() as c_int,
        );
    } else {
        regmap_multi_reg_write_bypassed(
            (*cs42l43).regmap,
            cs42l43_3pole_patch.as_ptr(),
            cs42l43_3pole_patch.len() as c_int,
        );
    }

    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_BLOCK_EN3,
        CS42L43_ADC1_EN_MASK | CS42L43_ADC2_EN_MASK,
        0,
    );
    regmap_update_bits((*cs42l43).regmap, CS42L43_DACCNFG2, CS42L43_HP_HPF_EN_MASK, 0);
    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_MIC_DETECT_CONTROL_1,
        CS42L43_HSBIAS_MODE_MASK,
        0,
    );
    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_CTRL,
        CS42L43_ADPTPWR_MODE_MASK,
        0x4 << CS42L43_ADPTPWR_MODE_SHIFT,
    );
    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_PGAVOL,
        CS42L43_HP_DIG_VOL_RAMP_MASK | CS42L43_HP_ANA_VOL_RAMP_MASK,
        0x6,
    );
    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_DACCNFG1,
        CS42L43_HP_MSTR_VOL_CTRL_EN_MASK,
        0,
    );

    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_HS2,
        CS42L43_HS_CLAMP_DISABLE_MASK,
        CS42L43_HS_CLAMP_DISABLE_MASK,
    );

    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_LOADDETENA,
        CS42L43_HPLOAD_DET_EN_MASK,
        CS42L43_HPLOAD_DET_EN_MASK,
    );

    snd_soc_dapm_mutex_unlock(snd_soc_component_to_dapm((*priv_).component));
}

unsafe fn cs42l43_stop_load_detect(priv_: *mut cs42l43_codec) {
    let cs42l43 = (*priv_).core;

    dev_dbg((*priv_).dev, c_str!("Stop load detect\n"));

    snd_soc_dapm_mutex_lock(snd_soc_component_to_dapm((*priv_).component));

    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_LOADDETENA,
        CS42L43_HPLOAD_DET_EN_MASK,
        0,
    );
    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_HS2,
        CS42L43_HS_CLAMP_DISABLE_MASK,
        0,
    );
    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_DACCNFG1,
        CS42L43_HP_MSTR_VOL_CTRL_EN_MASK,
        CS42L43_HP_MSTR_VOL_CTRL_EN_MASK,
    );
    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_PGAVOL,
        CS42L43_HP_DIG_VOL_RAMP_MASK | CS42L43_HP_ANA_VOL_RAMP_MASK,
        0x4 << CS42L43_HP_DIG_VOL_RAMP_SHIFT,
    );
    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_CTRL,
        CS42L43_ADPTPWR_MODE_MASK,
        0x7 << CS42L43_ADPTPWR_MODE_SHIFT,
    );
    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_MIC_DETECT_CONTROL_1,
        CS42L43_HSBIAS_MODE_MASK,
        0x1 << CS42L43_HSBIAS_MODE_SHIFT,
    );
    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_DACCNFG2,
        CS42L43_HP_HPF_EN_MASK,
        CS42L43_HP_HPF_EN_MASK,
    );

    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_BLOCK_EN3,
        CS42L43_ADC1_EN_MASK | CS42L43_ADC2_EN_MASK,
        (*priv_).adc_ena,
    );

    if (*priv_).hp_ena != 0 && !(*priv_).hp_ilimited {
        let time_left: c_ulong;

        reinit_completion(&mut (*priv_).hp_startup);

        regmap_update_bits(
            (*cs42l43).regmap,
            CS42L43_BLOCK_EN8,
            CS42L43_HP_EN_MASK,
            (*priv_).hp_ena,
        );

        time_left = wait_for_completion_timeout(
            &mut (*priv_).hp_startup,
            msecs_to_jiffies(CS42L43_HP_TIMEOUT_MS),
        );
        if time_left == 0 {
            dev_err((*priv_).dev, c_str!("Load detect HP restore timed out\n"));
        }
    }

    (*priv_).load_detect_running = false;

    snd_soc_dapm_mutex_unlock(snd_soc_component_to_dapm((*priv_).component));
}

unsafe fn cs42l43_run_load_detect(priv_: *mut cs42l43_codec, mic: bool) -> c_int {
    let cs42l43 = (*priv_).core;
    let mut val: c_uint = 0;
    let time_left: c_ulong;

    reinit_completion(&mut (*priv_).load_detect);

    cs42l43_start_load_detect(priv_, mic);
    time_left = wait_for_completion_timeout(
        &mut (*priv_).load_detect,
        msecs_to_jiffies(CS42L43_LOAD_TIMEOUT_MS),
    );
    cs42l43_stop_load_detect(priv_);

    if time_left == 0 {
        return -ETIMEDOUT;
    }

    regmap_read((*cs42l43).regmap, CS42L43_LOADDETRESULTS, &mut val);

    dev_dbg((*priv_).dev, c_str!("Headphone load detect: 0x%x\n"), val);

    /* Bail if jack removed, the load is irrelevant and likely invalid */
    if !cs42l43_jack_present(priv_) {
        return -ENODEV;
    }

    if mic {
        cs42l43_start_hs_bias(priv_, false);
        cs42l43_start_button_detect(priv_);

        return CS42L43_JACK_HEADSET;
    }

    match val & CS42L43_AMP3_RES_DET_MASK {
        0x0 => CS42L43_JACK_HEADPHONE, /* < 22 Ohm impedance */
        0x1 => CS42L43_JACK_HEADPHONE, /* < 150 Ohm impedance */
        0x2 => CS42L43_JACK_HEADPHONE, /* < 1000 Ohm impedance */
        0x3 => CS42L43_JACK_LINEOUT,   /* > 1000 Ohm impedance */
        _ => -EINVAL,
    }
}

unsafe fn cs42l43_run_type_detect(priv_: *mut cs42l43_codec) -> c_int {
    let cs42l43 = (*priv_).core;
    let timeout_ms: c_int = ((2 * (*priv_).detect_us) / USEC_PER_MSEC) as c_int + 200;
    let mut type_: c_uint = 0xff;
    let time_left: c_ulong;

    reinit_completion(&mut (*priv_).type_detect);

    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_STEREO_MIC_CLAMP_CTRL,
        CS42L43_SMIC_HPAMP_CLAMP_DIS_FRC_VAL_MASK,
        CS42L43_SMIC_HPAMP_CLAMP_DIS_FRC_VAL_MASK,
    );

    cs42l43_start_hs_bias(priv_, true);
    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_HS2,
        CS42L43_HSDET_MODE_MASK,
        0x3 << CS42L43_HSDET_MODE_SHIFT,
    );

    time_left = wait_for_completion_timeout(
        &mut (*priv_).type_detect,
        msecs_to_jiffies(timeout_ms as c_uint),
    );

    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_HS2,
        CS42L43_HSDET_MODE_MASK,
        0x2 << CS42L43_HSDET_MODE_SHIFT,
    );
    cs42l43_stop_hs_bias(priv_);

    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_STEREO_MIC_CLAMP_CTRL,
        CS42L43_SMIC_HPAMP_CLAMP_DIS_FRC_VAL_MASK,
        0,
    );

    if time_left == 0 {
        return -ETIMEDOUT;
    }

    regmap_read((*cs42l43).regmap, CS42L43_HS_STAT, &mut type_);

    dev_dbg((*priv_).dev, c_str!("Type detect: 0x%x\n"), type_);

    /* Bail if jack removed, the type is irrelevant and likely invalid */
    if !cs42l43_jack_present(priv_) {
        return -ENODEV;
    }

    match type_ & CS42L43_HSDET_TYPE_STS_MASK {
        0x0 => cs42l43_run_load_detect(priv_, true),  /* CTIA */
        0x1 => cs42l43_run_load_detect(priv_, true),  /* OMTP */
        0x4 => cs42l43_run_load_detect(priv_, true),
        0x2 => cs42l43_run_load_detect(priv_, false), /* 3-pole */
        0x3 => CS42L43_JACK_EXTENSION,                /* Open-circuit */
        _ => -EINVAL,
    }
}

#[no_mangle]
pub unsafe extern "C" fn cs42l43_clear_jack(priv_: *mut cs42l43_codec) {
    let cs42l43 = (*priv_).core;

    cs42l43_stop_button_detect(priv_);
    cs42l43_stop_hs_bias(priv_);

    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_ADC_B_CTRL1,
        CS42L43_PGA_WIDESWING_MODE_EN_MASK,
        0,
    );
    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_ADC_B_CTRL2,
        CS42L43_PGA_WIDESWING_MODE_EN_MASK,
        0,
    );
    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_STEREO_MIC_CTRL,
        CS42L43_JACK_STEREO_CONFIG_MASK,
        0,
    );
    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_STEREO_MIC_CLAMP_CTRL,
        CS42L43_SMIC_HPAMP_CLAMP_DIS_FRC_MASK,
        CS42L43_SMIC_HPAMP_CLAMP_DIS_FRC_MASK,
    );
    regmap_update_bits(
        (*cs42l43).regmap,
        CS42L43_HS2,
        CS42L43_HSDET_MODE_MASK | CS42L43_HSDET_MANUAL_MODE_MASK,
        0x2 << CS42L43_HSDET_MODE_SHIFT,
    );
}

#[no_mangle]
pub unsafe extern "C" fn cs42l43_tip_sense_work(work: *mut work_struct) {
    let priv_ = container_of!(
        work,
        cs42l43_codec,
        tip_sense_work.work
    );
    let cs42l43 = (*priv_).core;
    let mut sts: c_uint = 0;
    let tip: c_uint;
    let ring: c_uint;
    let ret: c_int;
    let report: c_int;

    ret = pm_runtime_resume_and_get((*priv_).dev);
    if ret != 0 {
        dev_err(
            (*priv_).dev,
            c_str!("Failed to resume for tip work: %d\n"),
            ret,
        );
        return;
    }

    mutex_lock(&mut (*priv_).jack_lock);

    regmap_read(
        (*cs42l43).regmap,
        CS42L43_TIP_RING_SENSE_INTERRUPT_STATUS,
        &mut sts,
    );

    dev_dbg((*priv_).dev, c_str!("Tip sense: 0x%x\n"), sts);

    tip = (sts >> CS42L43_TIPSENSE_PLUG_DB_STS_SHIFT) & CS42L43_JACK_PRESENT;
    ring = (sts >> CS42L43_RINGSENSE_PLUG_DB_STS_SHIFT) & CS42L43_JACK_PRESENT;

    if tip == CS42L43_JACK_PRESENT {
        if !(*cs42l43).sdw.is_null() && !(*priv_).jack_present {
            (*priv_).jack_present = true;
            pm_runtime_get((*priv_).dev);
        }

        if (*priv_).use_ring_sense && ring == CS42L43_JACK_ABSENT {
            report = CS42L43_JACK_OPTICAL;
        } else {
            report = cs42l43_run_type_detect(priv_);
            if report < 0 {
                dev_err((*priv_).dev, c_str!("Jack detect failed: %d\n"), report);
                goto_error(priv_);
                return;
            }
        }

        snd_soc_jack_report((*priv_).jack_hp, report, report);
    } else {
        (*priv_).jack_override = 0;

        cs42l43_clear_jack(priv_);

        snd_soc_jack_report((*priv_).jack_hp, 0, 0xFFFF);

        if !(*cs42l43).sdw.is_null() && (*priv_).jack_present {
            pm_runtime_put((*priv_).dev);
            (*priv_).jack_present = false;
        }
    }

    goto_error(priv_);
}

unsafe fn goto_error(priv_: *mut cs42l43_codec) {
    mutex_unlock(&mut (*priv_).jack_lock);

    (*priv_).suspend_jack_debounce = false;

    pm_runtime_put_autosuspend((*priv_).dev);
}

#[no_mangle]
pub unsafe extern "C" fn cs42l43_tip_sense(irq: c_int, data: *mut c_void) -> irqreturn_t {
    let priv_ = data as *mut cs42l43_codec;
    let mut db_delay: c_uint = (*priv_).tip_debounce_ms;

    let _ = irq;

    cancel_delayed_work(&mut (*priv_).bias_sense_timeout);
    cancel_delayed_work(&mut (*priv_).tip_sense_work);

    // Ensure delay after suspend is long enough to avoid false detection
    if (*priv_).suspend_jack_debounce {
        db_delay += (*priv_).tip_fall_db_ms + (*priv_).tip_rise_db_ms;
    }

    queue_delayed_work(
        system_dfl_long_wq,
        &mut (*priv_).tip_sense_work,
        msecs_to_jiffies(db_delay),
    );

    IRQ_HANDLED
}

#[repr(C)]
enum cs42l43_raw_jack {
    CS42L43_JACK_RAW_CTIA = 0,
    CS42L43_JACK_RAW_OMTP,
    CS42L43_JACK_RAW_HEADPHONE,
    CS42L43_JACK_RAW_LINE_OUT,
    CS42L43_JACK_RAW_LINE_IN,
    CS42L43_JACK_RAW_MICROPHONE,
    CS42L43_JACK_RAW_OPTICAL,
}

const CS42L43_JACK_3_POLE_SWITCHES: c_uint =
    (0x2 << CS42L43_HSDET_MANUAL_MODE_SHIFT)
        | CS42L43_AMP3_4_GNDREF_HS3_SEL_MASK
        | CS42L43_AMP3_4_GNDREF_HS4_SEL_MASK
        | CS42L43_HSBIAS_GNDREF_HS3_SEL_MASK
        | CS42L43_HSBIAS_GNDREF_HS4_SEL_MASK
        | CS42L43_HSGND_HS3_SEL_MASK
        | CS42L43_HSGND_HS4_SEL_MASK;

#[repr(C)]
struct cs42l43_jack_override_mode {
    hsdet_mode: c_uint,
    mic_ctrl: c_uint,
    clamp_ctrl: c_uint,
    report: c_int,
}

const cs42l43_jack_override_modes: [cs42l43_jack_override_mode; 7] = [
    cs42l43_jack_override_mode {
        hsdet_mode: CS42L43_AMP3_4_GNDREF_HS3_SEL_MASK
            | CS42L43_HSBIAS_GNDREF_HS3_SEL_MASK
            | CS42L43_HSBIAS_OUT_HS4_SEL_MASK
            | CS42L43_HSGND_HS3_SEL_MASK,
        mic_ctrl: 0,
        clamp_ctrl: CS42L43_SMIC_HPAMP_CLAMP_DIS_FRC_MASK,
        report: CS42L43_JACK_HEADSET,
    },
    cs42l43_jack_override_mode {
        hsdet_mode: (0x1 << CS42L43_HSDET_MANUAL_MODE_SHIFT)
            | CS42L43_AMP3_4_GNDREF_HS4_SEL_MASK
            | CS42L43_HSBIAS_GNDREF_HS4_SEL_MASK
            | CS42L43_HSBIAS_OUT_HS3_SEL_MASK
            | CS42L43_HSGND_HS4_SEL_MASK,
        mic_ctrl: 0,
        clamp_ctrl: CS42L43_SMIC_HPAMP_CLAMP_DIS_FRC_MASK,
        report: CS42L43_JACK_HEADSET,
    },
    cs42l43_jack_override_mode {
        hsdet_mode: CS42L43_JACK_3_POLE_SWITCHES,
        mic_ctrl: 0,
        clamp_ctrl: CS42L43_SMIC_HPAMP_CLAMP_DIS_FRC_MASK,
        report: CS42L43_JACK_HEADPHONE,
    },
    cs42l43_jack_override_mode {
        hsdet_mode: CS42L43_JACK_3_POLE_SWITCHES,
        mic_ctrl: 0,
        clamp_ctrl: CS42L43_SMIC_HPAMP_CLAMP_DIS_FRC_MASK,
        report: CS42L43_JACK_LINEOUT,
    },
    cs42l43_jack_override_mode {
        hsdet_mode: CS42L43_JACK_3_POLE_SWITCHES,
        mic_ctrl: 0x2 << CS42L43_JACK_STEREO_CONFIG_SHIFT,
        clamp_ctrl: 0,
        report: CS42L43_JACK_LINEIN,
    },
    cs42l43_jack_override_mode {
        hsdet_mode: CS42L43_JACK_3_POLE_SWITCHES,
        mic_ctrl: (0x3 << CS42L43_JACK_STEREO_CONFIG_SHIFT)
            | CS42L43_HS1_BIAS_EN_MASK
            | CS42L43_HS2_BIAS_EN_MASK,
        clamp_ctrl: 0,
        report: CS42L43_JACK_MICROPHONE,
    },
    cs42l43_jack_override_mode {
        hsdet_mode: CS42L43_JACK_3_POLE_SWITCHES,
        mic_ctrl: 0,
        clamp_ctrl: CS42L43_SMIC_HPAMP_CLAMP_DIS_FRC_MASK,
        report: CS42L43_JACK_OPTICAL,
    },
];

const cs42l43_jack_text: [*const c_char; 8] = [
    c_str!("None"),
    c_str!("CTIA"),
    c_str!("OMTP"),
    c_str!("Headphone"),
    c_str!("Line-Out"),
    c_str!("Line-In"),
    c_str!("Microphone"),
    c_str!("Optical"),
];

const _: () = assert!(cs42l43_jack_override_modes.len() == cs42l43_jack_text.len() - 1);

SOC_ENUM_SINGLE_VIRT_DECL!(cs42l43_jack_enum, cs42l43_jack_text);

#[no_mangle]
pub unsafe extern "C" fn cs42l43_jack_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs42l43_codec;

    let _guard = mutex_guard(&mut (*priv_).jack_lock);
    (*ucontrol).value.integer.value[0] = (*priv_).jack_override as _;

    0
}

#[no_mangle]
pub unsafe extern "C" fn cs42l43_jack_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs42l43_codec;
    let cs42l43 = (*priv_).core;
    let e = (*kcontrol).private_value as *mut soc_enum;
    let mut override_: c_uint = (*ucontrol).value.integer.value[0] as c_uint;

    if override_ >= (*e).items {
        return -EINVAL;
    }

    let _guard = mutex_guard(&mut (*priv_).jack_lock);

    if !cs42l43_jack_present(priv_) {
        return -EBUSY;
    }

    if override_ == (*priv_).jack_override {
        return 0;
    }

    (*priv_).jack_override = override_;

    cs42l43_clear_jack(priv_);

    snd_soc_jack_report((*priv_).jack_hp, 0, 0xFFFF);

    if override_ == 0 {
        queue_delayed_work(
            system_dfl_long_wq,
            &mut (*priv_).tip_sense_work,
            0,
        );
    } else {
        override_ -= 1;

        regmap_update_bits(
            (*cs42l43).regmap,
            CS42L43_HS2,
            CS42L43_HSDET_MODE_MASK
                | CS42L43_HSDET_MANUAL_MODE_MASK
                | CS42L43_AMP3_4_GNDREF_HS3_SEL_MASK
                | CS42L43_AMP3_4_GNDREF_HS4_SEL_MASK
                | CS42L43_HSBIAS_GNDREF_HS3_SEL_MASK
                | CS42L43_HSBIAS_GNDREF_HS4_SEL_MASK
                | CS42L43_HSBIAS_OUT_HS3_SEL_MASK
                | CS42L43_HSBIAS_OUT_HS4_SEL_MASK
                | CS42L43_HSGND_HS3_SEL_MASK
                | CS42L43_HSGND_HS4_SEL_MASK,
            cs42l43_jack_override_modes[override_ as usize].hsdet_mode,
        );
        regmap_update_bits(
            (*cs42l43).regmap,
            CS42L43_STEREO_MIC_CTRL,
            CS42L43_HS2_BIAS_EN_MASK | CS42L43_HS1_BIAS_EN_MASK | CS42L43_JACK_STEREO_CONFIG_MASK,
            cs42l43_jack_override_modes[override_ as usize].mic_ctrl,
        );
        regmap_update_bits(
            (*cs42l43).regmap,
            CS42L43_STEREO_MIC_CLAMP_CTRL,
            CS42L43_SMIC_HPAMP_CLAMP_DIS_FRC_MASK,
            cs42l43_jack_override_modes[override_ as usize].clamp_ctrl,
        );

        match override_ {
            x if x == cs42l43_raw_jack::CS42L43_JACK_RAW_CTIA as c_uint
                || x == cs42l43_raw_jack::CS42L43_JACK_RAW_OMTP as c_uint =>
            {
                cs42l43_start_hs_bias(priv_, false);
                cs42l43_start_button_detect(priv_);
            }
            x if x == cs42l43_raw_jack::CS42L43_JACK_RAW_LINE_IN as c_uint => {
                regmap_update_bits(
                    (*cs42l43).regmap,
                    CS42L43_ADC_B_CTRL1,
                    CS42L43_PGA_WIDESWING_MODE_EN_MASK,
                    CS42L43_PGA_WIDESWING_MODE_EN_MASK,
                );
                regmap_update_bits(
                    (*cs42l43).regmap,
                    CS42L43_ADC_B_CTRL2,
                    CS42L43_PGA_WIDESWING_MODE_EN_MASK,
                    CS42L43_PGA_WIDESWING_MODE_EN_MASK,
                );
            }
            x if x == cs42l43_raw_jack::CS42L43_JACK_RAW_MICROPHONE as c_uint => {
                cs42l43_start_hs_bias(priv_, false);
            }
            _ => {}
        }

        snd_soc_jack_report(
            (*priv_).jack_hp,
            cs42l43_jack_override_modes[override_ as usize].report,
            cs42l43_jack_override_modes[override_ as usize].report,
        );
    }

    1
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
