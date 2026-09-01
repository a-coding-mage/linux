// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * HD audio codec driver for Cirrus Logic CS8409 HDA bridge chip
 *
 * Copyright (C) 2021 Cirrus Logic, Inc. and
 *                    Cirrus Logic International Semiconductor Ltd.
 */

// C dependencies from:
// <linux/acpi.h>, <linux/cleanup.h>, <linux/i2c.h>, <linux/init.h>,
// <linux/slab.h>, <linux/module.h>, <linux/spi/spi.h>, <sound/core.h>,
// <linux/mutex.h>, <linux/iopoll.h>, "cs8409.h",
// "../side-codecs/hda_component.h"
use crate::*;

/******************************************************************************
 *                        CS8409 Specific Functions
 ******************************************************************************/

unsafe fn cs8409_parse_auto_config(codec: *mut hda_codec) -> c_int {
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;
    let mut err: c_int;
    let mut i: c_int;

    err = snd_hda_parse_pin_defcfg(codec, &mut (*spec).gen.autocfg, core::ptr::null_mut(), 0);
    if err < 0 {
        return err;
    }

    err = snd_hda_gen_parse_auto_config(codec, &mut (*spec).gen.autocfg);
    if err < 0 {
        return err;
    }

    /* keep the ADCs powered up when it's dynamically switchable */
    if (*spec).gen.dyn_adc_switch != 0 {
        let mut done: c_uint = 0;

        i = 0;
        while i < (*spec).gen.input_mux.num_items {
            let idx: c_int = (*spec).gen.dyn_adc_idx[i as usize];

            if (done & (1 << idx)) != 0 {
                i += 1;
                continue;
            }
            snd_hda_gen_fix_pin_power(codec, (*spec).gen.adc_nids[idx as usize]);
            done |= 1 << idx;
            i += 1;
        }
    }

    0
}

unsafe fn cs8409_disable_i2c_clock_worker(work: *mut work_struct);

unsafe fn cs8409_alloc_spec(codec: *mut hda_codec) -> *mut cs8409_spec {
    let spec: *mut cs8409_spec;

    spec = kzalloc_obj::<cs8409_spec>();
    if spec.is_null() {
        return core::ptr::null_mut();
    }
    (*codec).spec = spec as *mut _;
    (*spec).codec = codec;
    (*codec).power_save_node = 1;
    mutex_init(&mut (*spec).i2c_mux);
    INIT_DELAYED_WORK(&mut (*spec).i2c_clk_work, Some(cs8409_disable_i2c_clock_worker));
    snd_hda_gen_spec_init(&mut (*spec).gen);

    spec
}

#[inline]
unsafe fn cs8409_vendor_coef_get(codec: *mut hda_codec, idx: c_uint) -> c_int {
    snd_hda_codec_write(codec, CS8409_PIN_VENDOR_WIDGET, 0, AC_VERB_SET_COEF_INDEX, idx);
    snd_hda_codec_read(codec, CS8409_PIN_VENDOR_WIDGET, 0, AC_VERB_GET_PROC_COEF, 0)
}

#[inline]
unsafe fn cs8409_vendor_coef_set(codec: *mut hda_codec, idx: c_uint, coef: c_uint) {
    snd_hda_codec_write(codec, CS8409_PIN_VENDOR_WIDGET, 0, AC_VERB_SET_COEF_INDEX, idx);
    snd_hda_codec_write(codec, CS8409_PIN_VENDOR_WIDGET, 0, AC_VERB_SET_PROC_COEF, coef);
}

/*
 * cs8409_enable_i2c_clock - Disable I2C clocks
 * @codec: the codec instance
 * Disable I2C clocks.
 * This must be called when the i2c mutex is unlocked.
 */
unsafe fn cs8409_disable_i2c_clock(codec: *mut hda_codec) {
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;

    let _guard = guard_mutex(&mut (*spec).i2c_mux);
    if (*spec).i2c_clck_enabled != 0 {
        cs8409_vendor_coef_set(
            (*spec).codec,
            0x0,
            (cs8409_vendor_coef_get((*spec).codec, 0x0) as c_uint) & 0xfffffff7,
        );
        (*spec).i2c_clck_enabled = 0;
    }
}

/*
 * cs8409_disable_i2c_clock_worker - Worker that disable the I2C Clock after 25ms without use
 */
unsafe fn cs8409_disable_i2c_clock_worker(work: *mut work_struct) {
    let spec: *mut cs8409_spec =
        container_of!(work, cs8409_spec, i2c_clk_work.work);

    cs8409_disable_i2c_clock((*spec).codec);
}

/*
 * cs8409_enable_i2c_clock - Enable I2C clocks
 * @codec: the codec instance
 * Enable I2C clocks.
 * This must be called when the i2c mutex is locked.
 */
unsafe fn cs8409_enable_i2c_clock(codec: *mut hda_codec) {
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;

    /* Cancel the disable timer, but do not wait for any running disable functions to finish.
     * If the disable timer runs out before cancel, the delayed work thread will be blocked,
     * waiting for the mutex to become unlocked. This mutex will be locked for the duration of
     * any i2c transaction, so the disable function will run to completion immediately
     * afterwards in the scenario. The next enable call will re-enable the clock, regardless.
     */
    cancel_delayed_work(&mut (*spec).i2c_clk_work);

    if (*spec).i2c_clck_enabled == 0 {
        cs8409_vendor_coef_set(codec, 0x0, (cs8409_vendor_coef_get(codec, 0x0) as c_uint) | 0x8);
        (*spec).i2c_clck_enabled = 1;
    }
    queue_delayed_work(system_power_efficient_wq, &mut (*spec).i2c_clk_work, msecs_to_jiffies(25));
}

/**
 * cs8409_i2c_wait_complete - Wait for I2C transaction
 * @codec: the codec instance
 *
 * Wait for I2C transaction to complete.
 * Return -ETIMEDOUT if transaction wait times out.
 */
unsafe fn cs8409_i2c_wait_complete(codec: *mut hda_codec) -> c_int {
    let mut retval: c_uint = 0;

    read_poll_timeout!(
        cs8409_vendor_coef_get,
        retval,
        (retval & 0x18) != 0,
        CS42L42_I2C_SLEEP_US,
        CS42L42_I2C_TIMEOUT_US,
        false,
        codec,
        CS8409_I2C_STS
    )
}

/**
 * cs8409_set_i2c_dev_addr - Set i2c address for transaction
 * @codec: the codec instance
 * @addr: I2C Address
 */
unsafe fn cs8409_set_i2c_dev_addr(codec: *mut hda_codec, addr: c_uint) {
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;

    if (*spec).dev_addr != addr {
        cs8409_vendor_coef_set(codec, CS8409_I2C_ADDR, addr);
        (*spec).dev_addr = addr;
    }
}

/**
 * cs8409_i2c_set_page - CS8409 I2C set page register.
 * @scodec: the codec instance
 * @i2c_reg: Page register
 *
 * Returns negative on error.
 */
unsafe fn cs8409_i2c_set_page(scodec: *mut sub_codec, i2c_reg: c_uint) -> c_int {
    let codec: *mut hda_codec = (*scodec).codec;

    if (*scodec).paged != 0 && (*scodec).last_page != (i2c_reg >> 8) {
        cs8409_vendor_coef_set(codec, CS8409_I2C_QWRITE, i2c_reg >> 8);
        if cs8409_i2c_wait_complete(codec) < 0 {
            return -EIO;
        }
        (*scodec).last_page = i2c_reg >> 8;
    }

    0
}

/**
 * cs8409_i2c_read - CS8409 I2C Read.
 * @scodec: the codec instance
 * @addr: Register to read
 *
 * Returns negative on error, otherwise returns read value in bits 0-7.
 */
unsafe fn cs8409_i2c_read(scodec: *mut sub_codec, addr: c_uint) -> c_int {
    let codec: *mut hda_codec = (*scodec).codec;
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;
    let i2c_reg_data: c_uint;
    let read_data: c_uint;

    if (*scodec).suspended != 0 {
        return -EPERM;
    }

    let _guard = guard_mutex(&mut (*spec).i2c_mux);
    cs8409_enable_i2c_clock(codec);
    cs8409_set_i2c_dev_addr(codec, (*scodec).addr);

    if cs8409_i2c_set_page(scodec, addr) != 0 {
        codec_err(codec, c_str!("%s() Failed 0x%02x : 0x%04x\n"), c_str!(__func__), (*scodec).addr, addr);
        return -EIO;
    }

    i2c_reg_data = (addr << 8) & 0x0ffff;
    cs8409_vendor_coef_set(codec, CS8409_I2C_QREAD, i2c_reg_data);
    if cs8409_i2c_wait_complete(codec) < 0 {
        codec_err(codec, c_str!("%s() Failed 0x%02x : 0x%04x\n"), c_str!(__func__), (*scodec).addr, addr);
        return -EIO;
    }

    /* Register in bits 15-8 and the data in 7-0 */
    read_data = cs8409_vendor_coef_get(codec, CS8409_I2C_QREAD) as c_uint;

    (read_data & 0x0ff) as c_int
}

/**
 * cs8409_i2c_bulk_read - CS8409 I2C Read Sequence.
 * @scodec: the codec instance
 * @seq: Register Sequence to read
 * @count: Number of registeres to read
 *
 * Returns negative on error, values are read into value element of cs8409_i2c_param sequence.
 */
unsafe fn cs8409_i2c_bulk_read(
    scodec: *mut sub_codec,
    seq: *mut cs8409_i2c_param,
    count: c_int,
) -> c_int {
    let codec: *mut hda_codec = (*scodec).codec;
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;
    let mut i2c_reg_data: c_uint;
    let mut i: c_int;

    if (*scodec).suspended != 0 {
        return -EPERM;
    }

    let _guard = guard_mutex(&mut (*spec).i2c_mux);
    cs8409_set_i2c_dev_addr(codec, (*scodec).addr);

    i = 0;
    while i < count {
        cs8409_enable_i2c_clock(codec);
        if cs8409_i2c_set_page(scodec, (*seq.add(i as usize)).addr) != 0 {
            codec_err(codec, c_str!("I2C Bulk Read Failed 0x%02x\n"), (*scodec).addr);
            return -EIO;
        }

        i2c_reg_data = ((*seq.add(i as usize)).addr << 8) & 0x0ffff;
        cs8409_vendor_coef_set(codec, CS8409_I2C_QREAD, i2c_reg_data);

        if cs8409_i2c_wait_complete(codec) < 0 {
            codec_err(codec, c_str!("I2C Bulk Read Failed 0x%02x\n"), (*scodec).addr);
            return -EIO;
        }

        (*seq.add(i as usize)).value = (cs8409_vendor_coef_get(codec, CS8409_I2C_QREAD) as c_uint) & 0xff;
        i += 1;
    }

    0
}

/**
 * cs8409_i2c_write - CS8409 I2C Write.
 * @scodec: the codec instance
 * @addr: Register to write to
 * @value: Data to write
 *
 * Returns negative on error, otherwise returns 0.
 */
unsafe fn cs8409_i2c_write(scodec: *mut sub_codec, addr: c_uint, value: c_uint) -> c_int {
    let codec: *mut hda_codec = (*scodec).codec;
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;
    let i2c_reg_data: c_uint;

    if (*scodec).suspended != 0 {
        return -EPERM;
    }

    let _guard = guard_mutex(&mut (*spec).i2c_mux);

    cs8409_enable_i2c_clock(codec);
    cs8409_set_i2c_dev_addr(codec, (*scodec).addr);

    if cs8409_i2c_set_page(scodec, addr) != 0 {
        codec_err(codec, c_str!("%s() Failed 0x%02x : 0x%04x\n"), c_str!(__func__), (*scodec).addr, addr);
        return -EIO;
    }

    i2c_reg_data = ((addr << 8) & 0x0ff00) | (value & 0x0ff);
    cs8409_vendor_coef_set(codec, CS8409_I2C_QWRITE, i2c_reg_data);

    if cs8409_i2c_wait_complete(codec) < 0 {
        codec_err(codec, c_str!("%s() Failed 0x%02x : 0x%04x\n"), c_str!(__func__), (*scodec).addr, addr);
        return -EIO;
    }

    0
}

/**
 * cs8409_i2c_bulk_write - CS8409 I2C Write Sequence.
 * @scodec: the codec instance
 * @seq: Register Sequence to write
 * @count: Number of registeres to write
 *
 * Returns negative on error.
 */
unsafe fn cs8409_i2c_bulk_write(
    scodec: *mut sub_codec,
    seq: *const cs8409_i2c_param,
    count: c_int,
) -> c_int {
    let codec: *mut hda_codec = (*scodec).codec;
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;
    let mut i2c_reg_data: c_uint;
    let mut i: c_int;

    if (*scodec).suspended != 0 {
        return -EPERM;
    }

    let _guard = guard_mutex(&mut (*spec).i2c_mux);
    cs8409_set_i2c_dev_addr(codec, (*scodec).addr);

    i = 0;
    while i < count {
        cs8409_enable_i2c_clock(codec);
        if cs8409_i2c_set_page(scodec, (*seq.add(i as usize)).addr) != 0 {
            codec_err(codec, c_str!("I2C Bulk Write Failed 0x%02x\n"), (*scodec).addr);
            return -EIO;
        }

        i2c_reg_data = (((*seq.add(i as usize)).addr << 8) & 0x0ff00) |
            ((*seq.add(i as usize)).value & 0x0ff);
        cs8409_vendor_coef_set(codec, CS8409_I2C_QWRITE, i2c_reg_data);

        if cs8409_i2c_wait_complete(codec) < 0 {
            codec_err(codec, c_str!("I2C Bulk Write Failed 0x%02x\n"), (*scodec).addr);
            return -EIO;
        }
        /* Certain use cases may require a delay
         * after a write operation before proceeding.
         */
        if (*seq.add(i as usize)).delay != 0 {
            fsleep((*seq.add(i as usize)).delay);
        }
        i += 1;
    }

    0
}

unsafe fn cs8409_init(codec: *mut hda_codec) -> c_int {
    let ret: c_int = snd_hda_gen_init(codec);

    if ret == 0 {
        snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_INIT);
    }

    ret
}

unsafe fn cs8409_build_controls(codec: *mut hda_codec) -> c_int {
    let err: c_int;

    err = snd_hda_gen_build_controls(codec);
    if err < 0 {
        return err;
    }
    snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_BUILD);

    0
}

/* Enable/Disable Unsolicited Response */
unsafe fn cs8409_enable_ur(codec: *mut hda_codec, flag: c_int) {
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;
    let mut ur_gpios: c_uint = 0;
    let mut i: c_int;

    i = 0;
    while i < (*spec).num_scodecs {
        ur_gpios |= (*(*spec).scodecs[i as usize]).irq_mask;
        i += 1;
    }

    snd_hda_codec_write(
        codec,
        CS8409_PIN_AFG,
        0,
        AC_VERB_SET_GPIO_UNSOLICITED_RSP_MASK,
        if flag != 0 { ur_gpios } else { 0 },
    );

    snd_hda_codec_write(
        codec,
        CS8409_PIN_AFG,
        0,
        AC_VERB_SET_UNSOLICITED_ENABLE,
        if flag != 0 { AC_UNSOL_ENABLED } else { 0 },
    );
}

unsafe fn cs8409_fix_caps(codec: *mut hda_codec, nid: c_uint) {
    let caps: c_int;

    /* CS8409 is simple HDA bridge and intended to be used with a remote
     * companion codec. Most of input/output PIN(s) have only basic
     * capabilities. Receive and Transmit NID(s) have only OUTC and INC
     * capabilities and no presence detect capable (PDC) and call to
     * snd_hda_gen_build_controls() will mark them as non detectable
     * phantom jacks. However, a companion codec may be
     * connected to these pins which supports jack detect
     * capabilities. We have to override pin capabilities,
     * otherwise they will not be created as input devices.
     */
    caps = snd_hdac_read_parm(&mut (*codec).core, nid, AC_PAR_PIN_CAP);
    if caps >= 0 {
        snd_hdac_override_parm(
            &mut (*codec).core,
            nid,
            AC_PAR_PIN_CAP,
            (caps as c_uint) | (AC_PINCAP_IMP_SENSE | AC_PINCAP_PRES_DETECT),
        );
    }

    snd_hda_override_wcaps(codec, nid, get_wcaps(codec, nid) | AC_WCAP_UNSOL_CAP);
}

unsafe fn cs8409_spk_sw_gpio_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec: *mut hda_codec = snd_kcontrol_chip(kcontrol);
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;

    (*ucontrol).value.integer.value[0] =
        if ((*spec).gpio_data & (*spec).speaker_pdn_gpio) != 0 { 1 } else { 0 };
    0
}

unsafe fn cs8409_spk_sw_gpio_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec: *mut hda_codec = snd_kcontrol_chip(kcontrol);
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;
    let gpio_data: c_uint;

    gpio_data = ((*spec).gpio_data & !(*spec).speaker_pdn_gpio) |
        if (*ucontrol).value.integer.value[0] != 0 { (*spec).speaker_pdn_gpio } else { 0 };
    if gpio_data == (*spec).gpio_data {
        return 0;
    }
    (*spec).gpio_data = gpio_data;
    snd_hda_codec_write(codec, CS8409_PIN_AFG, 0, AC_VERB_SET_GPIO_DATA, (*spec).gpio_data);
    1
}

static_const! {
    static cs8409_spk_sw_ctrl: snd_kcontrol_new = snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        info: Some(snd_ctl_boolean_mono_info),
        get: Some(cs8409_spk_sw_gpio_get),
        put: Some(cs8409_spk_sw_gpio_put),
        ..ZEROED
    };
}

/******************************************************************************
 *                        CS42L42 Specific Functions
 ******************************************************************************/

pub unsafe fn cs42l42_volume_info(kctrl: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let ofs: c_uint = get_amp_offset(kctrl);
    let chs: u8 = get_amp_channels(kctrl);

    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).value.integer.step = 1;
    (*uinfo).count = if chs == 3 { 2 } else { 1 };

    match ofs {
        CS42L42_VOL_DAC => {
            (*uinfo).value.integer.min = CS42L42_HP_VOL_REAL_MIN;
            (*uinfo).value.integer.max = CS42L42_HP_VOL_REAL_MAX;
        }
        CS42L42_VOL_ADC => {
            (*uinfo).value.integer.min = CS42L42_AMIC_VOL_REAL_MIN;
            (*uinfo).value.integer.max = CS42L42_AMIC_VOL_REAL_MAX;
        }
        _ => {}
    }

    0
}

pub unsafe fn cs42l42_volume_get(kctrl: *mut snd_kcontrol, uctrl: *mut snd_ctl_elem_value) -> c_int {
    let codec: *mut hda_codec = snd_kcontrol_chip(kctrl);
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;
    let cs42l42: *mut sub_codec = (*spec).scodecs[get_amp_index(kctrl) as usize];
    let chs: c_int = get_amp_channels(kctrl) as c_int;
    let ofs: c_uint = get_amp_offset(kctrl);
    let mut valp: *mut c_long = (*uctrl).value.integer.value.as_mut_ptr();

    match ofs {
        CS42L42_VOL_DAC => {
            if (chs & BIT(0)) != 0 {
                *valp = (*cs42l42).vol[ofs as usize];
                valp = valp.add(1);
            }
            if (chs & BIT(1)) != 0 {
                *valp = (*cs42l42).vol[(ofs + 1) as usize];
            }
        }
        CS42L42_VOL_ADC => {
            if (chs & BIT(0)) != 0 {
                *valp = (*cs42l42).vol[ofs as usize];
            }
        }
        _ => {}
    }

    0
}

unsafe fn cs42l42_mute(cs42l42: *mut sub_codec, vol_type: c_int, chs: c_uint, mute: bool) {
    if mute {
        if vol_type == CS42L42_VOL_DAC as c_int {
            if (chs & BIT(0) as c_uint) != 0 {
                cs8409_i2c_write(cs42l42, CS42L42_MIXER_CHA_VOL, 0x3f);
            }
            if (chs & BIT(1) as c_uint) != 0 {
                cs8409_i2c_write(cs42l42, CS42L42_MIXER_CHB_VOL, 0x3f);
            }
        } else if vol_type == CS42L42_VOL_ADC as c_int {
            if (chs & BIT(0) as c_uint) != 0 {
                cs8409_i2c_write(cs42l42, CS42L42_ADC_VOLUME, 0x9f);
            }
        }
    } else if vol_type == CS42L42_VOL_DAC as c_int {
        if (chs & BIT(0) as c_uint) != 0 {
            cs8409_i2c_write(
                cs42l42,
                CS42L42_MIXER_CHA_VOL,
                (-( (*cs42l42).vol[CS42L42_DAC_CH0_VOL_OFFSET as usize] ) & CS42L42_MIXER_CH_VOL_MASK as c_long) as c_uint,
            );
        }
        if (chs & BIT(1) as c_uint) != 0 {
            cs8409_i2c_write(
                cs42l42,
                CS42L42_MIXER_CHB_VOL,
                (-( (*cs42l42).vol[CS42L42_DAC_CH1_VOL_OFFSET as usize] ) & CS42L42_MIXER_CH_VOL_MASK as c_long) as c_uint,
            );
        }
    } else if vol_type == CS42L42_VOL_ADC as c_int {
        if (chs & BIT(0) as c_uint) != 0 {
            cs8409_i2c_write(
                cs42l42,
                CS42L42_ADC_VOLUME,
                ((*cs42l42).vol[CS42L42_ADC_VOL_OFFSET as usize] as c_uint) & CS42L42_REG_AMIC_VOL_MASK,
            );
        }
    }
}

pub unsafe fn cs42l42_volume_put(kctrl: *mut snd_kcontrol, uctrl: *mut snd_ctl_elem_value) -> c_int {
    let codec: *mut hda_codec = snd_kcontrol_chip(kctrl);
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;
    let cs42l42: *mut sub_codec = (*spec).scodecs[get_amp_index(kctrl) as usize];
    let chs: c_int = get_amp_channels(kctrl) as c_int;
    let ofs: c_uint = get_amp_offset(kctrl);
    let mut valp: *mut c_long = (*uctrl).value.integer.value.as_mut_ptr();

    match ofs {
        CS42L42_VOL_DAC => {
            if (chs & BIT(0)) != 0 {
                (*cs42l42).vol[ofs as usize] = *valp;
            }
            if (chs & BIT(1)) != 0 {
                valp = valp.add(1);
                (*cs42l42).vol[(ofs + 1) as usize] = *valp;
            }
            if (*spec).playback_started != 0 {
                cs42l42_mute(cs42l42, CS42L42_VOL_DAC as c_int, chs as c_uint, false);
            }
        }
        CS42L42_VOL_ADC => {
            if (chs & BIT(0)) != 0 {
                (*cs42l42).vol[ofs as usize] = *valp;
            }
            if (*spec).capture_started != 0 {
                cs42l42_mute(cs42l42, CS42L42_VOL_ADC as c_int, chs as c_uint, false);
            }
        }
        _ => {}
    }

    0
}

unsafe fn cs42l42_playback_pcm_hook(
    _hinfo: *mut hda_pcm_stream,
    codec: *mut hda_codec,
    _substream: *mut snd_pcm_substream,
    action: c_int,
) {
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;
    let mut cs42l42: *mut sub_codec;
    let mut i: c_int;
    let mute: bool;

    match action {
        HDA_GEN_PCM_ACT_PREPARE => {
            mute = false;
            (*spec).playback_started = 1;
        }
        HDA_GEN_PCM_ACT_CLEANUP => {
            mute = true;
            (*spec).playback_started = 0;
        }
        _ => return,
    }

    i = 0;
    while i < (*spec).num_scodecs {
        cs42l42 = (*spec).scodecs[i as usize];
        cs42l42_mute(cs42l42, CS42L42_VOL_DAC as c_int, 0x3, mute);
        i += 1;
    }
}

unsafe fn cs42l42_capture_pcm_hook(
    _hinfo: *mut hda_pcm_stream,
    codec: *mut hda_codec,
    _substream: *mut snd_pcm_substream,
    action: c_int,
) {
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;
    let mut cs42l42: *mut sub_codec;
    let mut i: c_int;
    let mute: bool;

    match action {
        HDA_GEN_PCM_ACT_PREPARE => {
            mute = false;
            (*spec).capture_started = 1;
        }
        HDA_GEN_PCM_ACT_CLEANUP => {
            mute = true;
            (*spec).capture_started = 0;
        }
        _ => return,
    }

    i = 0;
    while i < (*spec).num_scodecs {
        cs42l42 = (*spec).scodecs[i as usize];
        cs42l42_mute(cs42l42, CS42L42_VOL_ADC as c_int, 0x3, mute);
        i += 1;
    }
}

/* Configure CS42L42 slave codec for jack autodetect */
unsafe fn cs42l42_enable_jack_detect(cs42l42: *mut sub_codec) {
    cs8409_i2c_write(cs42l42, CS42L42_HSBIAS_SC_AUTOCTL, (*cs42l42).hsbias_hiz);
    /* Clear WAKE# */
    cs8409_i2c_write(cs42l42, CS42L42_WAKE_CTL, 0x00C1);
    /* Wait ~2.5ms */
    usleep_range(2500, 3000);
    /* Set mode WAKE# output follows the combination logic directly */
    cs8409_i2c_write(cs42l42, CS42L42_WAKE_CTL, 0x00C0);
    /* Clear interrupts status */
    cs8409_i2c_read(cs42l42, CS42L42_TSRS_PLUG_STATUS);
    /* Enable interrupt */
    cs8409_i2c_write(cs42l42, CS42L42_TSRS_PLUG_INT_MASK, 0xF3);
}

/* Enable and run CS42L42 slave codec jack auto detect */
unsafe fn cs42l42_run_jack_detect(cs42l42: *mut sub_codec) {
    /* Clear interrupts */
    cs8409_i2c_read(cs42l42, CS42L42_CODEC_STATUS);
    cs8409_i2c_read(cs42l42, CS42L42_DET_STATUS1);
    cs8409_i2c_write(cs42l42, CS42L42_TSRS_PLUG_INT_MASK, 0xFF);
    cs8409_i2c_read(cs42l42, CS42L42_TSRS_PLUG_STATUS);

    cs8409_i2c_write(cs42l42, CS42L42_PWR_CTL2, 0x87);
    cs8409_i2c_write(cs42l42, CS42L42_DAC_CTL2, 0x86);
    cs8409_i2c_write(cs42l42, CS42L42_MISC_DET_CTL, 0x07);
    cs8409_i2c_write(cs42l42, CS42L42_CODEC_INT_MASK, 0xFD);
    cs8409_i2c_write(cs42l42, CS42L42_HSDET_CTL2, 0x80);
    /* Wait ~20ms*/
    usleep_range(20000, 25000);
    cs8409_i2c_write(cs42l42, CS42L42_HSDET_CTL1, 0x77);
    cs8409_i2c_write(cs42l42, CS42L42_HSDET_CTL2, 0xc0);
}

unsafe fn cs42l42_manual_hs_det(cs42l42: *mut sub_codec) -> c_int {
    let mut hs_det_status: c_uint;
    let mut hs_det_comp1: c_uint;
    let mut hs_det_comp2: c_uint;
    let hs_det_sw: c_uint;
    let hs_type: c_uint;

    /* Set hs detect to manual, active mode */
    cs8409_i2c_write(
        cs42l42,
        CS42L42_HSDET_CTL2,
        (1 << CS42L42_HSDET_CTRL_SHIFT)
            | (0 << CS42L42_HSDET_SET_SHIFT)
            | (0 << CS42L42_HSBIAS_REF_SHIFT)
            | (0 << CS42L42_HSDET_AUTO_TIME_SHIFT),
    );

    /* Configure HS DET comparator reference levels. */
    cs8409_i2c_write(
        cs42l42,
        CS42L42_HSDET_CTL1,
        (CS42L42_HSDET_COMP1_LVL_VAL << CS42L42_HSDET_COMP1_LVL_SHIFT)
            | (CS42L42_HSDET_COMP2_LVL_VAL << CS42L42_HSDET_COMP2_LVL_SHIFT),
    );

    /* Open the SW_HSB_HS3 switch and close SW_HSB_HS4 for a Type 1 headset. */
    cs8409_i2c_write(cs42l42, CS42L42_HS_SWITCH_CTL, CS42L42_HSDET_SW_COMP1);

    msleep(100);

    hs_det_status = cs8409_i2c_read(cs42l42, CS42L42_HS_DET_STATUS) as c_uint;

    hs_det_comp1 = (hs_det_status & CS42L42_HSDET_COMP1_OUT_MASK) >> CS42L42_HSDET_COMP1_OUT_SHIFT;
    hs_det_comp2 = (hs_det_status & CS42L42_HSDET_COMP2_OUT_MASK) >> CS42L42_HSDET_COMP2_OUT_SHIFT;

    /* Close the SW_HSB_HS3 switch for a Type 2 headset. */
    cs8409_i2c_write(cs42l42, CS42L42_HS_SWITCH_CTL, CS42L42_HSDET_SW_COMP2);

    msleep(100);

    hs_det_status = cs8409_i2c_read(cs42l42, CS42L42_HS_DET_STATUS) as c_uint;

    hs_det_comp1 |= ((hs_det_status & CS42L42_HSDET_COMP1_OUT_MASK)
        >> CS42L42_HSDET_COMP1_OUT_SHIFT) << 1;
    hs_det_comp2 |= ((hs_det_status & CS42L42_HSDET_COMP2_OUT_MASK)
        >> CS42L42_HSDET_COMP2_OUT_SHIFT) << 1;

    /* Use Comparator 1 with 1.25V Threshold. */
    match hs_det_comp1 {
        CS42L42_HSDET_COMP_TYPE1 => {
            hs_type = CS42L42_PLUG_CTIA;
            hs_det_sw = CS42L42_HSDET_SW_TYPE1;
        }
        CS42L42_HSDET_COMP_TYPE2 => {
            hs_type = CS42L42_PLUG_OMTP;
            hs_det_sw = CS42L42_HSDET_SW_TYPE2;
        }
        _ => {
            /* Fallback to Comparator 2 with 1.75V Threshold. */
            match hs_det_comp2 {
                CS42L42_HSDET_COMP_TYPE1 => {
                    hs_type = CS42L42_PLUG_CTIA;
                    hs_det_sw = CS42L42_HSDET_SW_TYPE1;
                }
                CS42L42_HSDET_COMP_TYPE2 => {
                    hs_type = CS42L42_PLUG_OMTP;
                    hs_det_sw = CS42L42_HSDET_SW_TYPE2;
                }
                CS42L42_HSDET_COMP_TYPE3 => {
                    hs_type = CS42L42_PLUG_HEADPHONE;
                    hs_det_sw = CS42L42_HSDET_SW_TYPE3;
                }
                _ => {
                    hs_type = CS42L42_PLUG_INVALID;
                    hs_det_sw = CS42L42_HSDET_SW_TYPE4;
                }
            }
        }
    }

    /* Set Switches */
    cs8409_i2c_write(cs42l42, CS42L42_HS_SWITCH_CTL, hs_det_sw);

    /* Set HSDET mode to Manual—Disabled */
    cs8409_i2c_write(
        cs42l42,
        CS42L42_HSDET_CTL2,
        (0 << CS42L42_HSDET_CTRL_SHIFT)
            | (0 << CS42L42_HSDET_SET_SHIFT)
            | (0 << CS42L42_HSBIAS_REF_SHIFT)
            | (0 << CS42L42_HSDET_AUTO_TIME_SHIFT),
    );

    /* Configure HS DET comparator reference levels. */
    cs8409_i2c_write(
        cs42l42,
        CS42L42_HSDET_CTL1,
        (CS42L42_HSDET_COMP1_LVL_DEFAULT << CS42L42_HSDET_COMP1_LVL_SHIFT)
            | (CS42L42_HSDET_COMP2_LVL_DEFAULT << CS42L42_HSDET_COMP2_LVL_SHIFT),
    );

    hs_type as c_int
}

unsafe fn cs42l42_handle_tip_sense(cs42l42: *mut sub_codec, reg_ts_status: c_uint) -> c_int {
    let mut status_changed: c_int = 0;

    /* TIP_SENSE INSERT/REMOVE */
    match reg_ts_status {
        CS42L42_TS_PLUG => {
            if (*cs42l42).no_type_dect != 0 {
                status_changed = 1;
                (*cs42l42).hp_jack_in = 1;
                (*cs42l42).mic_jack_in = 0;
            } else {
                cs42l42_run_jack_detect(cs42l42);
            }
        }
        CS42L42_TS_UNPLUG => {
            status_changed = 1;
            (*cs42l42).hp_jack_in = 0;
            (*cs42l42).mic_jack_in = 0;
        }
        _ => {
            /* jack in transition */
        }
    }

    codec_dbg((*cs42l42).codec, c_str!("Tip Sense Detection: (%d)\n"), reg_ts_status);

    status_changed
}

unsafe fn cs42l42_jack_unsol_event(cs42l42: *mut sub_codec) -> c_int {
    let current_plug_status: c_int;
    let mut status_changed: c_int = 0;
    let reg_cdc_status: c_int;
    let reg_hs_status: c_int;
    let reg_ts_status: c_int;
    let mut type_: c_int;

    /* Read jack detect status registers */
    reg_cdc_status = cs8409_i2c_read(cs42l42, CS42L42_CODEC_STATUS);
    reg_hs_status = cs8409_i2c_read(cs42l42, CS42L42_HS_DET_STATUS);
    reg_ts_status = cs8409_i2c_read(cs42l42, CS42L42_TSRS_PLUG_STATUS);

    /* If status values are < 0, read error has occurred. */
    if reg_cdc_status < 0 || reg_hs_status < 0 || reg_ts_status < 0 {
        return -EIO;
    }

    current_plug_status = ((reg_ts_status as c_uint & (CS42L42_TS_PLUG_MASK | CS42L42_TS_UNPLUG_MASK))
        >> CS42L42_TS_PLUG_SHIFT) as c_int;

    /* HSDET_AUTO_DONE */
    if (reg_cdc_status as c_uint & CS42L42_HSDET_AUTO_DONE_MASK) != 0 {
        /* Disable HSDET_AUTO_DONE */
        cs8409_i2c_write(cs42l42, CS42L42_CODEC_INT_MASK, 0xFF);

        type_ = ((reg_hs_status as c_uint & CS42L42_HSDET_TYPE_MASK) >> CS42L42_HSDET_TYPE_SHIFT) as c_int;

        /* Configure the HSDET mode. */
        cs8409_i2c_write(cs42l42, CS42L42_HSDET_CTL2, 0x80);

        if (*cs42l42).no_type_dect != 0 {
            status_changed = cs42l42_handle_tip_sense(cs42l42, current_plug_status as c_uint);
        } else {
            if type_ == CS42L42_PLUG_INVALID as c_int || type_ == CS42L42_PLUG_HEADPHONE as c_int {
                codec_dbg(
                    (*cs42l42).codec,
                    c_str!("Auto detect value not valid (%d), running manual det\n"),
                    type_,
                );
                type_ = cs42l42_manual_hs_det(cs42l42);
            }

            match type_ as c_uint {
                CS42L42_PLUG_CTIA | CS42L42_PLUG_OMTP => {
                    status_changed = 1;
                    (*cs42l42).hp_jack_in = 1;
                    (*cs42l42).mic_jack_in = 1;
                }
                CS42L42_PLUG_HEADPHONE => {
                    status_changed = 1;
                    (*cs42l42).hp_jack_in = 1;
                    (*cs42l42).mic_jack_in = 0;
                }
                _ => {
                    status_changed = 1;
                    (*cs42l42).hp_jack_in = 0;
                    (*cs42l42).mic_jack_in = 0;
                }
            }
            codec_dbg((*cs42l42).codec, c_str!("Detection done (%d)\n"), type_);
        }

        /* Enable the HPOUT ground clamp and configure the HP pull-down */
        cs8409_i2c_write(cs42l42, CS42L42_DAC_CTL2, 0x02);
        /* Re-Enable Tip Sense Interrupt */
        cs8409_i2c_write(cs42l42, CS42L42_TSRS_PLUG_INT_MASK, 0xF3);
    } else {
        status_changed = cs42l42_handle_tip_sense(cs42l42, current_plug_status as c_uint);
    }

    status_changed
}

unsafe fn cs42l42_resume(cs42l42: *mut sub_codec) {
    let codec: *mut hda_codec = (*cs42l42).codec;
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;
    let mut irq_regs = [
        cs8409_i2c_param { addr: CS42L42_CODEC_STATUS, value: 0x00, ..ZEROED },
        cs8409_i2c_param { addr: CS42L42_DET_INT_STATUS1, value: 0x00, ..ZEROED },
        cs8409_i2c_param { addr: CS42L42_DET_INT_STATUS2, value: 0x00, ..ZEROED },
        cs8409_i2c_param { addr: CS42L42_TSRS_PLUG_STATUS, value: 0x00, ..ZEROED },
    ];
    let mut fsv: c_uint;

    /* Bring CS42L42 out of Reset */
    (*spec).gpio_data = snd_hda_codec_read(codec, CS8409_PIN_AFG, 0, AC_VERB_GET_GPIO_DATA, 0) as c_uint;
    (*spec).gpio_data |= (*cs42l42).reset_gpio;
    snd_hda_codec_write(codec, CS8409_PIN_AFG, 0, AC_VERB_SET_GPIO_DATA, (*spec).gpio_data);
    usleep_range(10000, 15000);

    (*cs42l42).suspended = 0;

    /* Initialize CS42L42 companion codec */
    cs8409_i2c_bulk_write(cs42l42, (*cs42l42).init_seq, (*cs42l42).init_seq_num);

    /* Clear interrupts, by reading interrupt status registers */
    cs8409_i2c_bulk_read(cs42l42, irq_regs.as_mut_ptr(), ARRAY_SIZE(&irq_regs) as c_int);

    fsv = cs8409_i2c_read(cs42l42, CS42L42_HP_CTL) as c_uint;
    if (*cs42l42).full_scale_vol != 0 {
        // Set the full scale volume bit
        fsv |= CS42L42_FULL_SCALE_VOL_MASK;
        cs8409_i2c_write(cs42l42, CS42L42_HP_CTL, fsv);
    }
    // Unmute analog channels A and B
    fsv &= !CS42L42_ANA_MUTE_AB;
    cs8409_i2c_write(cs42l42, CS42L42_HP_CTL, fsv);

    /* we have to explicitly allow unsol event handling even during the
     * resume phase so that the jack event is processed properly
     */
    snd_hda_codec_allow_unsol_events((*cs42l42).codec);

    cs42l42_enable_jack_detect(cs42l42);
}

unsafe fn cs42l42_suspend(cs42l42: *mut sub_codec) {
    let codec: *mut hda_codec = (*cs42l42).codec;
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;
    let mut reg_cdc_status: c_int = 0;
    let cs42l42_pwr_down_seq = [
        cs8409_i2c_param { addr: CS42L42_DAC_CTL2, value: 0x02, ..ZEROED },
        cs8409_i2c_param { addr: CS42L42_HS_CLAMP_DISABLE, value: 0x00, ..ZEROED },
        cs8409_i2c_param { addr: CS42L42_MIXER_CHA_VOL, value: 0x3F, ..ZEROED },
        cs8409_i2c_param { addr: CS42L42_MIXER_ADC_VOL, value: 0x3F, ..ZEROED },
        cs8409_i2c_param { addr: CS42L42_MIXER_CHB_VOL, value: 0x3F, ..ZEROED },
        cs8409_i2c_param { addr: CS42L42_HP_CTL, value: 0x0D, ..ZEROED },
        cs8409_i2c_param { addr: CS42L42_ASP_RX_DAI0_EN, value: 0x00, ..ZEROED },
        cs8409_i2c_param { addr: CS42L42_ASP_CLK_CFG, value: 0x00, ..ZEROED },
        cs8409_i2c_param { addr: CS42L42_PWR_CTL1, value: 0xFE, ..ZEROED },
        cs8409_i2c_param { addr: CS42L42_PWR_CTL2, value: 0x8C, ..ZEROED },
        cs8409_i2c_param { addr: CS42L42_PWR_CTL1, value: 0xFF, ..ZEROED },
    ];

    cs8409_i2c_bulk_write(cs42l42, cs42l42_pwr_down_seq.as_ptr(), ARRAY_SIZE(&cs42l42_pwr_down_seq) as c_int);

    if read_poll_timeout!(
        cs8409_i2c_read,
        reg_cdc_status,
        (reg_cdc_status & 0x1) != 0,
        CS42L42_PDN_SLEEP_US,
        CS42L42_PDN_TIMEOUT_US,
        true,
        cs42l42,
        CS42L42_CODEC_STATUS
    ) < 0 {
        codec_warn(codec, c_str!("Timeout waiting for PDN_DONE for CS42L42\n"));
    }

    /* Power down CS42L42 ASP/EQ/MIX/HP */
    cs8409_i2c_write(cs42l42, CS42L42_PWR_CTL2, 0x9C);
    (*cs42l42).suspended = 1;
    (*cs42l42).last_page = 0;
    (*cs42l42).hp_jack_in = 0;
    (*cs42l42).mic_jack_in = 0;

    /* Put CS42L42 into Reset */
    (*spec).gpio_data = snd_hda_codec_read(codec, CS8409_PIN_AFG, 0, AC_VERB_GET_GPIO_DATA, 0) as c_uint;
    (*spec).gpio_data &= !(*cs42l42).reset_gpio;
    snd_hda_codec_write(codec, CS8409_PIN_AFG, 0, AC_VERB_SET_GPIO_DATA, (*spec).gpio_data);
}

unsafe fn cs8409_remove(codec: *mut hda_codec) {
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;

    /* Cancel i2c clock disable timer, and disable clock if left enabled */
    cancel_delayed_work_sync(&mut (*spec).i2c_clk_work);
    cs8409_disable_i2c_clock(codec);

    snd_hda_gen_remove(codec);
}

/******************************************************************************
 *                   BULLSEYE / WARLOCK / CYBORG Specific Functions
 *                               CS8409/CS42L42
 ******************************************************************************/

/*
 * In the case of CS8409 we do not have unsolicited events from NID's 0x24
 * and 0x34 where hs mic and hp are connected. Companion codec CS42L42 will
 * generate interrupt via gpio 4 to notify jack events. We have to overwrite
 * generic snd_hda_jack_unsol_event(), read CS42L42 jack detect status registers
 * and then notify status via generic snd_hda_jack_unsol_event() call.
 */
unsafe fn cs8409_cs42l42_jack_unsol_event(codec: *mut hda_codec, res: c_uint) {
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;
    let cs42l42: *mut sub_codec = (*spec).scodecs[CS8409_CODEC0 as usize];
    let mut jk: *mut hda_jack_tbl;

    /* jack_unsol_event() will be called every time gpio line changing state.
     * In this case gpio4 line goes up as a result of reading interrupt status
     * registers in previous cs8409_jack_unsol_event() call.
     * We don't need to handle this event, ignoring...
     */
    if (res & (*cs42l42).irq_mask) != 0 {
        return;
    }

    if cs42l42_jack_unsol_event(cs42l42) != 0 {
        snd_hda_set_pin_ctl(
            codec,
            CS8409_CS42L42_SPK_PIN_NID,
            if (*cs42l42).hp_jack_in != 0 { 0 } else { PIN_OUT },
        );
        /* Report jack*/
        jk = snd_hda_jack_tbl_get_mst(codec, CS8409_CS42L42_HP_PIN_NID, 0);
        if !jk.is_null() {
            snd_hda_jack_unsol_event(codec, ((*jk).tag << AC_UNSOL_RES_TAG_SHIFT) & AC_UNSOL_RES_TAG);
        }
        /* Report jack*/
        jk = snd_hda_jack_tbl_get_mst(codec, CS8409_CS42L42_AMIC_PIN_NID, 0);
        if !jk.is_null() {
            snd_hda_jack_unsol_event(codec, ((*jk).tag << AC_UNSOL_RES_TAG_SHIFT) & AC_UNSOL_RES_TAG);
        }
    }
}

unsafe fn cs8409_unsol_event(codec: *mut hda_codec, res: c_uint) {
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;

    if (*spec).unsol_event.is_some() {
        ((*spec).unsol_event.unwrap())(codec, res);
    } else {
        cs8409_cs42l42_jack_unsol_event(codec, res);
    }
}

/* Manage PDREF, when transition to D3hot */
unsafe fn cs8409_cs42l42_suspend(codec: *mut hda_codec) -> c_int {
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;
    let mut i: c_int;

    (*spec).init_done = 0;

    cs8409_enable_ur(codec, 0);

    i = 0;
    while i < (*spec).num_scodecs {
        cs42l42_suspend((*spec).scodecs[i as usize]);
        i += 1;
    }

    /* Cancel i2c clock disable timer, and disable clock if left enabled */
    cancel_delayed_work_sync(&mut (*spec).i2c_clk_work);
    cs8409_disable_i2c_clock(codec);

    snd_hda_shutup_pins(codec);

    0
}

/* Vendor specific HW configuration
 * PLL, ASP, I2C, SPI, GPIOs, DMIC etc...
 */
unsafe fn cs8409_cs42l42_hw_init(codec: *mut hda_codec) {
    let mut seq: *const cs8409_cir_param = cs8409_cs42l42_hw_cfg.as_ptr();
    let mut seq_bullseye: *const cs8409_cir_param = cs8409_cs42l42_bullseye_atn.as_ptr();
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;
    let cs42l42: *mut sub_codec = (*spec).scodecs[CS8409_CODEC0 as usize];

    if (*spec).gpio_mask != 0 {
        snd_hda_codec_set_gpio(codec, (*spec).gpio_mask, (*spec).gpio_dir, (*spec).gpio_data, 0);
    }

    while (*seq).nid != 0 {
        cs8409_vendor_coef_set(codec, (*seq).cir, (*seq).coeff);
        seq = seq.add(1);
    }

    if (*codec).fixup_id == CS8409_BULLSEYE {
        while (*seq_bullseye).nid != 0 {
            cs8409_vendor_coef_set(codec, (*seq_bullseye).cir, (*seq_bullseye).coeff);
            seq_bullseye = seq_bullseye.add(1);
        }
    }

    match (*codec).fixup_id {
        CS8409_CYBORG | CS8409_WARLOCK_MLK_DUAL_MIC => {
            /* DMIC1_MO=00b, DMIC1/2_SR=1 */
            cs8409_vendor_coef_set(codec, CS8409_DMIC_CFG, 0x0003);
        }
        CS8409_ODIN => {
            /* ASP1/2_xxx_EN=1, ASP1/2_MCLK_EN=0, DMIC1_SCL_EN=0 */
            cs8409_vendor_coef_set(codec, CS8409_PAD_CFG_SLW_RATE_CTRL, 0xfc00);
        }
        _ => {}
    }

    cs42l42_resume(cs42l42);

    /* Enable Unsolicited Response */
    cs8409_enable_ur(codec, 1);
}

unsafe fn cs8409_cs42l42_exec_verb(
    dev: *mut hdac_device,
    cmd: c_uint,
    flags: c_uint,
    res: *mut c_uint,
) -> c_int {
    let codec: *mut hda_codec = container_of!(dev, hda_codec, core);
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;
    let cs42l42: *mut sub_codec = (*spec).scodecs[CS8409_CODEC0 as usize];

    let nid: c_uint = (cmd >> 20) & 0x07f;
    let verb: c_uint = (cmd >> 8) & 0x0fff;

    /* CS8409 pins have no AC_PINSENSE_PRESENCE
     * capabilities. We have to intercept 2 calls for pins 0x24 and 0x34
     * and return correct pin sense values for read_pin_sense() call from
     * hda_jack based on CS42L42 jack detect status.
     */
    match nid {
        CS8409_CS42L42_HP_PIN_NID => {
            if verb == AC_VERB_GET_PIN_SENSE {
                *res = if (*cs42l42).hp_jack_in != 0 { AC_PINSENSE_PRESENCE } else { 0 };
                return 0;
            }
        }
        CS8409_CS42L42_AMIC_PIN_NID => {
            if verb == AC_VERB_GET_PIN_SENSE {
                *res = if (*cs42l42).mic_jack_in != 0 { AC_PINSENSE_PRESENCE } else { 0 };
                return 0;
            }
        }
        _ => {}
    }

    ((*spec).exec_verb.unwrap())(dev, cmd, flags, res)
}

pub unsafe fn cs8409_cs42l42_fixups(codec: *mut hda_codec, _fix: *const hda_fixup, action: c_int) {
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;

    match action {
        HDA_FIXUP_ACT_PRE_PROBE => {
            snd_hda_add_verbs(codec, cs8409_cs42l42_init_verbs.as_ptr());
            /* verb exec op override */
            (*spec).exec_verb = (*codec).core.exec_verb;
            (*codec).core.exec_verb = Some(cs8409_cs42l42_exec_verb);

            (*spec).scodecs[CS8409_CODEC0 as usize] = &mut cs8409_cs42l42_codec;
            (*spec).num_scodecs = 1;
            (*(*spec).scodecs[CS8409_CODEC0 as usize]).codec = codec;

            (*spec).gen.suppress_auto_mute = 1;
            (*spec).gen.no_primary_hp = 1;
            (*spec).gen.suppress_vmaster = 1;

            (*spec).speaker_pdn_gpio = 0;

            /* GPIO 5 out, 3,4 in */
            (*spec).gpio_dir = (*(*spec).scodecs[CS8409_CODEC0 as usize]).reset_gpio;
            (*spec).gpio_data = 0;
            (*spec).gpio_mask = 0x03f;

            /* Basic initial sequence for specific hw configuration */
            snd_hda_sequence_write(codec, cs8409_cs42l42_init_verbs.as_ptr());

            cs8409_fix_caps(codec, CS8409_CS42L42_HP_PIN_NID);
            cs8409_fix_caps(codec, CS8409_CS42L42_AMIC_PIN_NID);

            (*(*spec).scodecs[CS8409_CODEC0 as usize]).hsbias_hiz = 0x0020;

            match (*codec).fixup_id {
                CS8409_CYBORG => {
                    (*(*spec).scodecs[CS8409_CODEC0 as usize]).full_scale_vol =
                        CS42L42_FULL_SCALE_VOL_MINUS6DB;
                    (*spec).speaker_pdn_gpio = CS8409_CYBORG_SPEAKER_PDN;
                }
                CS8409_ODIN => {
                    (*(*spec).scodecs[CS8409_CODEC0 as usize]).full_scale_vol = CS42L42_FULL_SCALE_VOL_0DB;
                    (*spec).speaker_pdn_gpio = CS8409_CYBORG_SPEAKER_PDN;
                }
                CS8409_WARLOCK_MLK | CS8409_WARLOCK_MLK_DUAL_MIC => {
                    (*(*spec).scodecs[CS8409_CODEC0 as usize]).full_scale_vol = CS42L42_FULL_SCALE_VOL_0DB;
                    (*spec).speaker_pdn_gpio = CS8409_WARLOCK_SPEAKER_PDN;
                }
                _ => {
                    (*(*spec).scodecs[CS8409_CODEC0 as usize]).full_scale_vol =
                        CS42L42_FULL_SCALE_VOL_MINUS6DB;
                    (*spec).speaker_pdn_gpio = CS8409_WARLOCK_SPEAKER_PDN;
                }
            }

            if (*spec).speaker_pdn_gpio > 0 {
                (*spec).gpio_dir |= (*spec).speaker_pdn_gpio;
                (*spec).gpio_data |= (*spec).speaker_pdn_gpio;
            }
        }
        HDA_FIXUP_ACT_PROBE => {
            /* Fix Sample Rate to 48kHz */
            (*spec).gen.stream_analog_playback = &cs42l42_48k_pcm_analog_playback;
            (*spec).gen.stream_analog_capture = &cs42l42_48k_pcm_analog_capture;
            /* add hooks */
            (*spec).gen.pcm_playback_hook = Some(cs42l42_playback_pcm_hook);
            (*spec).gen.pcm_capture_hook = Some(cs42l42_capture_pcm_hook);
            if (*codec).fixup_id != CS8409_ODIN {
                /* Set initial DMIC volume to -26 dB */
                snd_hda_codec_amp_init_stereo(codec, CS8409_CS42L42_DMIC_ADC_PIN_NID, HDA_INPUT, 0, 0xff, 0x19);
            }
            snd_hda_gen_add_kctl(&mut (*spec).gen, c_str!("Headphone Playback Volume"), &cs42l42_dac_volume_mixer);
            snd_hda_gen_add_kctl(&mut (*spec).gen, c_str!("Mic Capture Volume"), &cs42l42_adc_volume_mixer);
            if (*spec).speaker_pdn_gpio > 0 {
                snd_hda_gen_add_kctl(&mut (*spec).gen, c_str!("Speaker Playback Switch"), &cs8409_spk_sw_ctrl);
            }
            /* Disable Unsolicited Response during boot */
            cs8409_enable_ur(codec, 0);
            snd_hda_codec_set_name(codec, c_str!("CS8409/CS42L42"));
        }
        HDA_FIXUP_ACT_INIT => {
            cs8409_cs42l42_hw_init(codec);
            (*spec).init_done = 1;
            if (*spec).init_done != 0
                && (*spec).build_ctrl_done != 0
                && (*(*spec).scodecs[CS8409_CODEC0 as usize]).hp_jack_in == 0
            {
                cs42l42_run_jack_detect((*spec).scodecs[CS8409_CODEC0 as usize]);
            }
        }
        HDA_FIXUP_ACT_BUILD => {
            (*spec).build_ctrl_done = 1;
            /* Run jack auto detect first time on boot
             * after controls have been added, to check if jack has
             * been already plugged in.
             * Run immediately after init.
             */
            if (*spec).init_done != 0
                && (*spec).build_ctrl_done != 0
                && (*(*spec).scodecs[CS8409_CODEC0 as usize]).hp_jack_in == 0
            {
                cs42l42_run_jack_detect((*spec).scodecs[CS8409_CODEC0 as usize]);
            }
        }
        _ => {}
    }
}

unsafe fn cs8409_comp_bind(dev: *mut device) -> c_int {
    let codec: *mut hda_codec = dev_to_hda_codec(dev);
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;

    hda_component_manager_bind(codec, &mut (*spec).comps)
}

unsafe fn cs8409_comp_unbind(dev: *mut device) {
    let codec: *mut hda_codec = dev_to_hda_codec(dev);
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;

    hda_component_manager_unbind(codec, &mut (*spec).comps);
}

static_const! {
    static cs8409_comp_master_ops: component_master_ops = component_master_ops {
        bind: Some(cs8409_comp_bind),
        unbind: Some(cs8409_comp_unbind),
        ..ZEROED
    };
}

unsafe fn cs8409_comp_playback_hook(
    _hinfo: *mut hda_pcm_stream,
    codec: *mut hda_codec,
    _sub: *mut snd_pcm_substream,
    action: c_int,
) {
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;

    hda_component_manager_playback_hook(&mut (*spec).comps, action);
}

unsafe fn cs8409_cdb35l56_four_hw_init(codec: *mut hda_codec) {
    let mut seq: *const cs8409_cir_param = cs8409_cdb35l56_four_hw_cfg.as_ptr();

    while (*seq).nid != 0 {
        cs8409_vendor_coef_set(codec, (*seq).cir, (*seq).coeff);
        seq = seq.add(1);
    }
}

unsafe fn cs8409_spk_sw_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec: *mut hda_codec = snd_kcontrol_chip(kcontrol);
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;

    (*ucontrol).value.integer.value[0] = if (*spec).speaker_muted == 0 { 1 } else { 0 };

    0
}

unsafe fn cs8409_spk_sw_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec: *mut hda_codec = snd_kcontrol_chip(kcontrol);
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;
    let muted: bool = (*ucontrol).value.integer.value[0] == 0;

    if muted == ((*spec).speaker_muted != 0) {
        return 0;
    }

    (*spec).speaker_muted = muted as c_int;

    1
}

static_const! {
    static cs8409_spk_sw_component_ctrl: snd_kcontrol_new = snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        info: Some(snd_ctl_boolean_mono_info),
        get: Some(cs8409_spk_sw_get),
        put: Some(cs8409_spk_sw_put),
        ..ZEROED
    };
}

pub unsafe fn cs8409_cdb35l56_four_autodet_fixup(
    codec: *mut hda_codec,
    _fix: *const hda_fixup,
    action: c_int,
) {
    let dev: *mut device = hda_codec_dev(codec);
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;
    let mut adev: *mut acpi_device;
    let mut bus: *const c_char = core::ptr::null();
    #[repr(C)]
    struct acpi_id_entry {
        hid: *const c_char,
        name: *const c_char,
    }
    static acpi_ids: [acpi_id_entry; 3] = [
        acpi_id_entry { hid: c_str!("CSC3554"), name: c_str!("cs35l54-hda") },
        acpi_id_entry { hid: c_str!("CSC3556"), name: c_str!("cs35l56-hda") },
        acpi_id_entry { hid: c_str!("CSC3557"), name: c_str!("cs35l57-hda") },
    ];
    let mut match_: *mut c_char;
    let mut i: c_int;
    let mut count: c_int = 0;
    let mut count_devindex: c_int = 0;
    let ret: c_int;

    match action {
        HDA_FIXUP_ACT_PRE_PROBE => {
            i = 0;
            while i < ARRAY_SIZE(&acpi_ids) as c_int {
                adev = acpi_dev_get_first_match_dev(acpi_ids[i as usize].hid, core::ptr::null(), -1);
                if !adev.is_null() {
                    break;
                }
                i += 1;
            }
            if adev.is_null() {
                dev_err(dev, c_str!("Failed to find ACPI entry for a Cirrus Amp\n"));
                return;
            }

            count = i2c_acpi_client_count(adev);
            if count > 0 {
                bus = c_str!("i2c");
            } else {
                count = acpi_spi_count_resources(adev);
                if count > 0 {
                    bus = c_str!("spi");
                }
            }

            let fwnode: *mut fwnode_handle = fwnode_handle_get(acpi_fwnode_handle(adev));
            acpi_dev_put(adev);

            if bus.is_null() {
                dev_err(dev, c_str!("Did not find any buses for %s\n"), acpi_ids[i as usize].hid);
                return;
            }

            if fwnode.is_null() {
                dev_err(dev, c_str!("Could not get fwnode for %s\n"), acpi_ids[i as usize].hid);
                return;
            }

            /*
             * When available the cirrus,dev-index property is an accurate
             * count of the amps in a system and is used in preference to
             * the count of bus devices that can contain additional address
             * alias entries.
             */
            count_devindex = fwnode_property_count_u32(fwnode, c_str!("cirrus,dev-index"));
            if count_devindex > 0 {
                count = count_devindex;
            }

            match_ = devm_kasprintf(dev, GFP_KERNEL, c_str!("-%s:00-%s.%d"), bus, acpi_ids[i as usize].name);
            if match_.is_null() {
                return;
            }
            dev_info(dev, c_str!("Found %d %s on %s (%s)\n"), count, acpi_ids[i as usize].hid, bus, match_);

            ret = hda_component_manager_init(
                codec,
                &mut (*spec).comps,
                count,
                bus,
                acpi_ids[i as usize].hid,
                match_,
                &cs8409_comp_master_ops,
            );
            if ret != 0 {
                return;
            }

            (*spec).gen.pcm_playback_hook = Some(cs8409_comp_playback_hook);

            snd_hda_add_verbs(codec, cs8409_cdb35l56_four_init_verbs.as_ptr());
            snd_hda_sequence_write(codec, cs8409_cdb35l56_four_init_verbs.as_ptr());
        }
        HDA_FIXUP_ACT_PROBE => {
            (*spec).speaker_muted = 0; /* speakers begin enabled */
            snd_hda_gen_add_kctl(&mut (*spec).gen, c_str!("Speaker Playback Switch"), &cs8409_spk_sw_component_ctrl);
            (*spec).gen.stream_analog_playback = &cs42l42_48k_pcm_analog_playback;
            snd_hda_codec_set_name(codec, c_str!("CS8409/CS35L56"));
        }
        HDA_FIXUP_ACT_INIT => {
            cs8409_cdb35l56_four_hw_init(codec);
        }
        HDA_FIXUP_ACT_FREE => {
            hda_component_manager_free(&mut (*spec).comps, &cs8409_comp_master_ops);
        }
        _ => {}
    }
}

/******************************************************************************
 *                          Dolphin Specific Functions
 *                               CS8409/ 2 X CS42L42
 ******************************************************************************/

/*
 * In the case of CS8409 we do not have unsolicited events when
 * hs mic and hp are connected. Companion codec CS42L42 will
 * generate interrupt via irq_mask to notify jack events. We have to overwrite
 * generic snd_hda_jack_unsol_event(), read CS42L42 jack detect status registers
 * and then notify status via generic snd_hda_jack_unsol_event() call.
 */
unsafe fn dolphin_jack_unsol_event(codec: *mut hda_codec, res: c_uint) {
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;
    let mut cs42l42: *mut sub_codec;
    let mut jk: *mut hda_jack_tbl;

    cs42l42 = (*spec).scodecs[CS8409_CODEC0 as usize];
    if (*cs42l42).suspended == 0 && ((!res & (*cs42l42).irq_mask) != 0)
        && cs42l42_jack_unsol_event(cs42l42) != 0
    {
        jk = snd_hda_jack_tbl_get_mst(codec, DOLPHIN_HP_PIN_NID, 0);
        if !jk.is_null() {
            snd_hda_jack_unsol_event(codec, ((*jk).tag << AC_UNSOL_RES_TAG_SHIFT) & AC_UNSOL_RES_TAG);
        }

        jk = snd_hda_jack_tbl_get_mst(codec, DOLPHIN_AMIC_PIN_NID, 0);
        if !jk.is_null() {
            snd_hda_jack_unsol_event(codec, ((*jk).tag << AC_UNSOL_RES_TAG_SHIFT) & AC_UNSOL_RES_TAG);
        }
    }

    cs42l42 = (*spec).scodecs[CS8409_CODEC1 as usize];
    if (*cs42l42).suspended == 0 && ((!res & (*cs42l42).irq_mask) != 0)
        && cs42l42_jack_unsol_event(cs42l42) != 0
    {
        jk = snd_hda_jack_tbl_get_mst(codec, DOLPHIN_LO_PIN_NID, 0);
        if !jk.is_null() {
            snd_hda_jack_unsol_event(codec, ((*jk).tag << AC_UNSOL_RES_TAG_SHIFT) & AC_UNSOL_RES_TAG);
        }
    }
}

/* Vendor specific HW configuration
 * PLL, ASP, I2C, SPI, GPIOs, DMIC etc...
 */
unsafe fn dolphin_hw_init(codec: *mut hda_codec) {
    let mut seq: *const cs8409_cir_param = dolphin_hw_cfg.as_ptr();
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;
    let mut cs42l42: *mut sub_codec;
    let mut i: c_int;

    if (*spec).gpio_mask != 0 {
        snd_hda_codec_set_gpio(codec, (*spec).gpio_mask, (*spec).gpio_dir, (*spec).gpio_data, 0);
    }

    while (*seq).nid != 0 {
        cs8409_vendor_coef_set(codec, (*seq).cir, (*seq).coeff);
        seq = seq.add(1);
    }

    i = 0;
    while i < (*spec).num_scodecs {
        cs42l42 = (*spec).scodecs[i as usize];
        cs42l42_resume(cs42l42);
        i += 1;
    }

    /* Enable Unsolicited Response */
    cs8409_enable_ur(codec, 1);
}

unsafe fn dolphin_exec_verb(
    dev: *mut hdac_device,
    cmd: c_uint,
    flags: c_uint,
    res: *mut c_uint,
) -> c_int {
    let codec: *mut hda_codec = container_of!(dev, hda_codec, core);
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;
    let mut cs42l42: *mut sub_codec = (*spec).scodecs[CS8409_CODEC0 as usize];

    let nid: c_uint = (cmd >> 20) & 0x07f;
    let verb: c_uint = (cmd >> 8) & 0x0fff;

    /* CS8409 pins have no AC_PINSENSE_PRESENCE
     * capabilities. We have to intercept calls for CS42L42 pins
     * and return correct pin sense values for read_pin_sense() call from
     * hda_jack based on CS42L42 jack detect status.
     */
    match nid {
        DOLPHIN_HP_PIN_NID | DOLPHIN_LO_PIN_NID => {
            if nid == DOLPHIN_LO_PIN_NID {
                cs42l42 = (*spec).scodecs[CS8409_CODEC1 as usize];
            }
            if verb == AC_VERB_GET_PIN_SENSE {
                *res = if (*cs42l42).hp_jack_in != 0 { AC_PINSENSE_PRESENCE } else { 0 };
                return 0;
            }
        }
        DOLPHIN_AMIC_PIN_NID => {
            if verb == AC_VERB_GET_PIN_SENSE {
                *res = if (*cs42l42).mic_jack_in != 0 { AC_PINSENSE_PRESENCE } else { 0 };
                return 0;
            }
        }
        _ => {}
    }

    ((*spec).exec_verb.unwrap())(dev, cmd, flags, res)
}

pub unsafe fn dolphin_fixups(codec: *mut hda_codec, _fix: *const hda_fixup, action: c_int) {
    let spec: *mut cs8409_spec = (*codec).spec as *mut cs8409_spec;
    let mut kctrl: *mut snd_kcontrol_new;
    let mut i: c_int;

    match action {
        HDA_FIXUP_ACT_PRE_PROBE => {
            snd_hda_add_verbs(codec, dolphin_init_verbs.as_ptr());
            /* verb exec op override */
            (*spec).exec_verb = (*codec).core.exec_verb;
            (*codec).core.exec_verb = Some(dolphin_exec_verb);

            (*spec).scodecs[CS8409_CODEC0 as usize] = &mut dolphin_cs42l42_0;
            (*(*spec).scodecs[CS8409_CODEC0 as usize]).codec = codec;
            (*spec).scodecs[CS8409_CODEC1 as usize] = &mut dolphin_cs42l42_1;
            (*(*spec).scodecs[CS8409_CODEC1 as usize]).codec = codec;
            (*spec).num_scodecs = 2;
            (*spec).gen.suppress_vmaster = 1;

            (*spec).unsol_event = Some(dolphin_jack_unsol_event);

            /* GPIO 1,5 out, 0,4 in */
            (*spec).gpio_dir = (*(*spec).scodecs[CS8409_CODEC0 as usize]).reset_gpio
                | (*(*spec).scodecs[CS8409_CODEC1 as usize]).reset_gpio;
            (*spec).gpio_data = 0;
            (*spec).gpio_mask = 0x03f;

            /* Basic initial sequence for specific hw configuration */
            snd_hda_sequence_write(codec, dolphin_init_verbs.as_ptr());

            snd_hda_jack_add_kctl(codec, DOLPHIN_LO_PIN_NID, c_str!("Line Out"), true, SND_JACK_HEADPHONE, core::ptr::null_mut());

            snd_hda_jack_add_kctl(codec, DOLPHIN_AMIC_PIN_NID, c_str!("Microphone"), true, SND_JACK_MICROPHONE, core::ptr::null_mut());

            cs8409_fix_caps(codec, DOLPHIN_HP_PIN_NID);
            cs8409_fix_caps(codec, DOLPHIN_LO_PIN_NID);
            cs8409_fix_caps(codec, DOLPHIN_AMIC_PIN_NID);

            (*(*spec).scodecs[CS8409_CODEC0 as usize]).full_scale_vol = CS42L42_FULL_SCALE_VOL_MINUS6DB;
            (*(*spec).scodecs[CS8409_CODEC1 as usize]).full_scale_vol = CS42L42_FULL_SCALE_VOL_MINUS6DB;
        }
        HDA_FIXUP_ACT_PROBE => {
            /* Fix Sample Rate to 48kHz */
            (*spec).gen.stream_analog_playback = &cs42l42_48k_pcm_analog_playback;
            (*spec).gen.stream_analog_capture = &cs42l42_48k_pcm_analog_capture;
            /* add hooks */
            (*spec).gen.pcm_playback_hook = Some(cs42l42_playback_pcm_hook);
            (*spec).gen.pcm_capture_hook = Some(cs42l42_capture_pcm_hook);
            snd_hda_gen_add_kctl(&mut (*spec).gen, c_str!("Headphone Playback Volume"), &cs42l42_dac_volume_mixer);
            snd_hda_gen_add_kctl(&mut (*spec).gen, c_str!("Mic Capture Volume"), &cs42l42_adc_volume_mixer);
            kctrl = snd_hda_gen_add_kctl(&mut (*spec).gen, c_str!("Line Out Playback Volume"), &cs42l42_dac_volume_mixer);
            /* Update Line Out kcontrol template */
            if !kctrl.is_null() {
                (*kctrl).private_value = HDA_COMPOSE_AMP_VAL_OFS(
                    DOLPHIN_HP_PIN_NID,
                    3,
                    CS8409_CODEC1,
                    HDA_OUTPUT,
                    CS42L42_VOL_DAC,
                ) | HDA_AMP_VAL_MIN_MUTE;
            }
            cs8409_enable_ur(codec, 0);
            snd_hda_codec_set_name(codec, c_str!("CS8409/CS42L42"));
        }
        HDA_FIXUP_ACT_INIT => {
            dolphin_hw_init(codec);
            (*spec).init_done = 1;
            if (*spec).init_done != 0 && (*spec).build_ctrl_done != 0 {
                i = 0;
                while i < (*spec).num_scodecs {
                    if (*(*spec).scodecs[i as usize]).hp_jack_in == 0 {
                        cs42l42_run_jack_detect((*spec).scodecs[i as usize]);
                    }
                    i += 1;
                }
            }
        }
        HDA_FIXUP_ACT_BUILD => {
            (*spec).build_ctrl_done = 1;
            /* Run jack auto detect first time on boot
             * after controls have been added, to check if jack has
             * been already plugged in.
             * Run immediately after init.
             */
            if (*spec).init_done != 0 && (*spec).build_ctrl_done != 0 {
                i = 0;
                while i < (*spec).num_scodecs {
                    if (*(*spec).scodecs[i as usize]).hp_jack_in == 0 {
                        cs42l42_run_jack_detect((*spec).scodecs[i as usize]);
                    }
                    i += 1;
                }
            }
        }
        _ => {}
    }
}

unsafe fn cs8409_probe(codec: *mut hda_codec, _id: *const hda_device_id) -> c_int {
    let err: c_int;

    if cs8409_alloc_spec(codec).is_null() {
        return -ENOMEM;
    }

    snd_hda_pick_fixup(codec, cs8409_models.as_ptr(), cs8409_fixup_tbl.as_ptr(), cs8409_fixups.as_ptr());

    codec_dbg(
        codec,
        c_str!("Picked ID=%d, VID=%08x, DEV=%08x\n"),
        (*codec).fixup_id,
        (*(*(*codec).bus).pci).subsystem_vendor,
        (*(*(*codec).bus).pci).subsystem_device,
    );

    snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PRE_PROBE);

    err = cs8409_parse_auto_config(codec);
    if err < 0 {
        cs8409_remove(codec);
        return err;
    }

    snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PROBE);
    0
}

static_const! {
    static cs8409_codec_ops: hda_codec_ops = hda_codec_ops {
        probe: Some(cs8409_probe),
        remove: Some(cs8409_remove),
        build_controls: Some(cs8409_build_controls),
        build_pcms: Some(snd_hda_gen_build_pcms),
        init: Some(cs8409_init),
        unsol_event: Some(cs8409_unsol_event),
        suspend: Some(cs8409_cs42l42_suspend),
        stream_pm: Some(snd_hda_gen_stream_pm),
        ..ZEROED
    };
}

static_const! {
    static snd_hda_id_cs8409: [hda_device_id; 2] = [
        HDA_CODEC_ID(0x10138409, c_str!("CS8409")),
        hda_device_id { ..ZEROED }, /* terminator */
    ];
}
MODULE_DEVICE_TABLE!(hdaudio, snd_hda_id_cs8409);

static_mut! {
    static mut cs8409_driver: hda_codec_driver = hda_codec_driver {
        id: snd_hda_id_cs8409.as_ptr(),
        ops: &cs8409_codec_ops,
        ..ZEROED
    };
}
module_hda_codec_driver!(cs8409_driver);

MODULE_LICENSE!(c_str!("GPL"));
MODULE_DESCRIPTION!(c_str!("Cirrus Logic HDA bridge"));
MODULE_IMPORT_NS!(c_str!("SND_HDA_SCODEC_COMPONENT"));

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
