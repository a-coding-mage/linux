// SPDX-License-Identifier: GPL-2.0-only
/*
 * C-Media CMI8788 driver - mixer code
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 */

// C dependencies: linux/mutex.h, sound/ac97_codec.h, sound/asoundef.h,
// sound/control.h, sound/tlv.h, oxygen.h, cm9780.h

unsafe extern "C" fn dac_volume_info(
    ctl: *mut snd_kcontrol,
    info: *mut snd_ctl_elem_info,
) -> c_int {
    let chip = unsafe { (*ctl).private_data as *mut oxygen };

    unsafe {
        (*info).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*info).count = (*chip).model.dac_channels_mixer;
        (*info).value.integer.min = (*chip).model.dac_volume_min;
        (*info).value.integer.max = (*chip).model.dac_volume_max;
    }
    0
}

unsafe extern "C" fn dac_volume_get(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = unsafe { (*ctl).private_data as *mut oxygen };
    let mut i: c_uint;

    guard_mutex!(unsafe { &mut (*chip).mutex });
    i = 0;
    while unsafe { i < (*chip).model.dac_channels_mixer } {
        unsafe {
            (*value).value.integer.value[i as usize] = (*chip).dac_volume[i as usize];
        }
        i += 1;
    }
    0
}

unsafe extern "C" fn dac_volume_put(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = unsafe { (*ctl).private_data as *mut oxygen };
    let mut i: c_uint;
    let mut changed: c_int;

    changed = 0;
    guard_mutex!(unsafe { &mut (*chip).mutex });
    i = 0;
    while unsafe { i < (*chip).model.dac_channels_mixer } {
        unsafe {
            if (*value).value.integer.value[i as usize] != (*chip).dac_volume[i as usize] {
                (*chip).dac_volume[i as usize] = (*value).value.integer.value[i as usize];
                changed = 1;
            }
        }
        i += 1;
    }
    if changed != 0 {
        unsafe {
            ((*chip).model.update_dac_volume.unwrap())(chip);
        }
    }
    changed
}

unsafe extern "C" fn dac_mute_get(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = unsafe { (*ctl).private_data as *mut oxygen };

    guard_mutex!(unsafe { &mut (*chip).mutex });
    unsafe {
        (*value).value.integer.value[0] = (!((*chip).dac_mute != 0)) as c_long;
    }
    0
}

unsafe extern "C" fn dac_mute_put(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = unsafe { (*ctl).private_data as *mut oxygen };
    let changed: c_int;

    guard_mutex!(unsafe { &mut (*chip).mutex });
    unsafe {
        changed = (((*value).value.integer.value[0] == 0) != ((*chip).dac_mute != 0)) as c_int;
        if changed != 0 {
            (*chip).dac_mute = ((*value).value.integer.value[0] == 0) as _;
            ((*chip).model.update_dac_mute.unwrap())(chip);
        }
    }
    changed
}

unsafe extern "C" fn upmix_item_count(chip: *mut oxygen) -> c_uint {
    unsafe {
        if (*chip).model.dac_channels_pcm < 8 {
            2
        } else if (*chip).model.update_center_lfe_mix.is_some() {
            5
        } else {
            3
        }
    }
}

unsafe extern "C" fn upmix_info(
    ctl: *mut snd_kcontrol,
    info: *mut snd_ctl_elem_info,
) -> c_int {
    static NAMES: [*const c_char; 5] = [
        c"Front".as_ptr(),
        c"Front+Surround".as_ptr(),
        c"Front+Surround+Back".as_ptr(),
        c"Front+Surround+Center/LFE".as_ptr(),
        c"Front+Surround+Center/LFE+Back".as_ptr(),
    ];
    let chip = unsafe { (*ctl).private_data as *mut oxygen };
    let count = unsafe { upmix_item_count(chip) };

    unsafe { snd_ctl_enum_info(info, 1, count, NAMES.as_ptr()) }
}

unsafe extern "C" fn upmix_get(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = unsafe { (*ctl).private_data as *mut oxygen };

    guard_mutex!(unsafe { &mut (*chip).mutex });
    unsafe {
        (*value).value.enumerated.item[0] = (*chip).dac_routing;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn oxygen_update_dac_routing(chip: *mut oxygen) {
    /* DAC 0: front, DAC 1: surround, DAC 2: center/LFE, DAC 3: back */
    static REG_VALUES: [c_uint; 5] = [
        /* stereo -> front */
        (0 << OXYGEN_PLAY_DAC0_SOURCE_SHIFT)
            | (1 << OXYGEN_PLAY_DAC1_SOURCE_SHIFT)
            | (2 << OXYGEN_PLAY_DAC2_SOURCE_SHIFT)
            | (3 << OXYGEN_PLAY_DAC3_SOURCE_SHIFT),
        /* stereo -> front+surround */
        (0 << OXYGEN_PLAY_DAC0_SOURCE_SHIFT)
            | (0 << OXYGEN_PLAY_DAC1_SOURCE_SHIFT)
            | (2 << OXYGEN_PLAY_DAC2_SOURCE_SHIFT)
            | (3 << OXYGEN_PLAY_DAC3_SOURCE_SHIFT),
        /* stereo -> front+surround+back */
        (0 << OXYGEN_PLAY_DAC0_SOURCE_SHIFT)
            | (0 << OXYGEN_PLAY_DAC1_SOURCE_SHIFT)
            | (2 << OXYGEN_PLAY_DAC2_SOURCE_SHIFT)
            | (0 << OXYGEN_PLAY_DAC3_SOURCE_SHIFT),
        /* stereo -> front+surround+center/LFE */
        (0 << OXYGEN_PLAY_DAC0_SOURCE_SHIFT)
            | (0 << OXYGEN_PLAY_DAC1_SOURCE_SHIFT)
            | (0 << OXYGEN_PLAY_DAC2_SOURCE_SHIFT)
            | (3 << OXYGEN_PLAY_DAC3_SOURCE_SHIFT),
        /* stereo -> front+surround+center/LFE+back */
        (0 << OXYGEN_PLAY_DAC0_SOURCE_SHIFT)
            | (0 << OXYGEN_PLAY_DAC1_SOURCE_SHIFT)
            | (0 << OXYGEN_PLAY_DAC2_SOURCE_SHIFT)
            | (0 << OXYGEN_PLAY_DAC3_SOURCE_SHIFT),
    ];
    let channels: u8;
    let mut reg_value: c_uint;

    channels = unsafe { oxygen_read8(chip, OXYGEN_PLAY_CHANNELS) & OXYGEN_PLAY_CHANNELS_MASK };
    if channels == OXYGEN_PLAY_CHANNELS_2 {
        reg_value = REG_VALUES[unsafe { (*chip).dac_routing as usize }];
    } else if channels == OXYGEN_PLAY_CHANNELS_8 {
        /* in 7.1 mode, "rear" channels go to the "back" jack */
        reg_value = (0 << OXYGEN_PLAY_DAC0_SOURCE_SHIFT)
            | (3 << OXYGEN_PLAY_DAC1_SOURCE_SHIFT)
            | (2 << OXYGEN_PLAY_DAC2_SOURCE_SHIFT)
            | (1 << OXYGEN_PLAY_DAC3_SOURCE_SHIFT);
    } else {
        reg_value = (0 << OXYGEN_PLAY_DAC0_SOURCE_SHIFT)
            | (1 << OXYGEN_PLAY_DAC1_SOURCE_SHIFT)
            | (2 << OXYGEN_PLAY_DAC2_SOURCE_SHIFT)
            | (3 << OXYGEN_PLAY_DAC3_SOURCE_SHIFT);
    }
    unsafe {
        if let Some(adjust_dac_routing) = (*chip).model.adjust_dac_routing {
            reg_value = adjust_dac_routing(chip, reg_value);
        }
        oxygen_write16_masked(
            chip,
            OXYGEN_PLAY_ROUTING,
            reg_value,
            OXYGEN_PLAY_DAC0_SOURCE_MASK
                | OXYGEN_PLAY_DAC1_SOURCE_MASK
                | OXYGEN_PLAY_DAC2_SOURCE_MASK
                | OXYGEN_PLAY_DAC3_SOURCE_MASK,
        );
        if let Some(update_center_lfe_mix) = (*chip).model.update_center_lfe_mix {
            update_center_lfe_mix(chip, (*chip).dac_routing > 2);
        }
    }
}

// EXPORT_SYMBOL(oxygen_update_dac_routing);

unsafe extern "C" fn upmix_put(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = unsafe { (*ctl).private_data as *mut oxygen };
    let count = unsafe { upmix_item_count(chip) };
    let changed: c_int;

    if unsafe { (*value).value.enumerated.item[0] >= count } {
        return -EINVAL;
    }
    guard_mutex!(unsafe { &mut (*chip).mutex });
    unsafe {
        changed = ((*value).value.enumerated.item[0] != (*chip).dac_routing) as c_int;
        if changed != 0 {
            (*chip).dac_routing = (*value).value.enumerated.item[0];
            oxygen_update_dac_routing(chip);
        }
    }
    changed
}

unsafe extern "C" fn spdif_switch_get(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = unsafe { (*ctl).private_data as *mut oxygen };

    guard_mutex!(unsafe { &mut (*chip).mutex });
    unsafe {
        (*value).value.integer.value[0] = (*chip).spdif_playback_enable as c_long;
    }
    0
}

unsafe extern "C" fn oxygen_spdif_rate(oxygen_rate: c_uint) -> c_uint {
    match oxygen_rate {
        OXYGEN_RATE_32000 => IEC958_AES3_CON_FS_32000 << OXYGEN_SPDIF_CS_RATE_SHIFT,
        OXYGEN_RATE_44100 => IEC958_AES3_CON_FS_44100 << OXYGEN_SPDIF_CS_RATE_SHIFT,
        OXYGEN_RATE_64000 => 0xb << OXYGEN_SPDIF_CS_RATE_SHIFT,
        OXYGEN_RATE_88200 => IEC958_AES3_CON_FS_88200 << OXYGEN_SPDIF_CS_RATE_SHIFT,
        OXYGEN_RATE_96000 => IEC958_AES3_CON_FS_96000 << OXYGEN_SPDIF_CS_RATE_SHIFT,
        OXYGEN_RATE_176400 => IEC958_AES3_CON_FS_176400 << OXYGEN_SPDIF_CS_RATE_SHIFT,
        OXYGEN_RATE_192000 => IEC958_AES3_CON_FS_192000 << OXYGEN_SPDIF_CS_RATE_SHIFT,
        _ => IEC958_AES3_CON_FS_48000 << OXYGEN_SPDIF_CS_RATE_SHIFT,
    }
}

#[no_mangle]
pub unsafe extern "C" fn oxygen_update_spdif_source(chip: *mut oxygen) {
    let old_control: u32;
    let mut new_control: u32;
    let old_routing: u16;
    let mut new_routing: u16;
    let oxygen_rate: c_uint;

    old_control = unsafe { oxygen_read32(chip, OXYGEN_SPDIF_CONTROL) };
    old_routing = unsafe { oxygen_read16(chip, OXYGEN_PLAY_ROUTING) };
    unsafe {
        if ((*chip).pcm_active & (1 << PCM_SPDIF)) != 0 {
            new_control = old_control | OXYGEN_SPDIF_OUT_ENABLE;
            new_routing = (old_routing & !OXYGEN_PLAY_SPDIF_MASK) | OXYGEN_PLAY_SPDIF_SPDIF;
            oxygen_rate =
                (old_control >> OXYGEN_SPDIF_OUT_RATE_SHIFT) & OXYGEN_I2S_RATE_MASK;
            /* S/PDIF rate was already set by the caller */
        } else if ((*chip).pcm_active & (1 << PCM_MULTICH)) != 0
            && (*chip).spdif_playback_enable != 0
        {
            new_routing =
                (old_routing & !OXYGEN_PLAY_SPDIF_MASK) | OXYGEN_PLAY_SPDIF_MULTICH_01;
            oxygen_rate =
                oxygen_read16(chip, OXYGEN_I2S_MULTICH_FORMAT) & OXYGEN_I2S_RATE_MASK;
            new_control = (old_control & !OXYGEN_SPDIF_OUT_RATE_MASK)
                | (oxygen_rate << OXYGEN_SPDIF_OUT_RATE_SHIFT)
                | OXYGEN_SPDIF_OUT_ENABLE;
        } else {
            new_control = old_control & !OXYGEN_SPDIF_OUT_ENABLE;
            new_routing = old_routing;
            oxygen_rate = OXYGEN_RATE_44100;
        }
        if old_routing != new_routing {
            oxygen_write32(
                chip,
                OXYGEN_SPDIF_CONTROL,
                new_control & !OXYGEN_SPDIF_OUT_ENABLE,
            );
            oxygen_write16(chip, OXYGEN_PLAY_ROUTING, new_routing);
        }
        if (new_control & OXYGEN_SPDIF_OUT_ENABLE) != 0 {
            oxygen_write32(
                chip,
                OXYGEN_SPDIF_OUTPUT_BITS,
                oxygen_spdif_rate(oxygen_rate)
                    | if ((*chip).pcm_active & (1 << PCM_SPDIF)) != 0 {
                        (*chip).spdif_pcm_bits
                    } else {
                        (*chip).spdif_bits
                    },
            );
        }
        oxygen_write32(chip, OXYGEN_SPDIF_CONTROL, new_control);
    }
}

unsafe extern "C" fn spdif_switch_put(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = unsafe { (*ctl).private_data as *mut oxygen };
    let changed: c_int;

    guard_mutex!(unsafe { &mut (*chip).mutex });
    unsafe {
        changed = ((*value).value.integer.value[0] != (*chip).spdif_playback_enable as c_long)
            as c_int;
        if changed != 0 {
            (*chip).spdif_playback_enable = ((*value).value.integer.value[0] != 0) as _;
            spin_lock_irq(&mut (*chip).reg_lock);
            oxygen_update_spdif_source(chip);
            spin_unlock_irq(&mut (*chip).reg_lock);
        }
    }
    changed
}

unsafe extern "C" fn spdif_info(
    _ctl: *mut snd_kcontrol,
    info: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe {
        (*info).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
        (*info).count = 1;
    }
    0
}

unsafe extern "C" fn oxygen_to_iec958(bits: u32, value: *mut snd_ctl_elem_value) {
    unsafe {
        (*value).value.iec958.status[0] =
            bits & (OXYGEN_SPDIF_NONAUDIO | OXYGEN_SPDIF_C | OXYGEN_SPDIF_PREEMPHASIS);
        (*value).value.iec958.status[1] = bits >> OXYGEN_SPDIF_CATEGORY_SHIFT;
    }
}

unsafe extern "C" fn iec958_to_oxygen(value: *mut snd_ctl_elem_value) -> u32 {
    let mut bits: u32;

    unsafe {
        bits = (*value).value.iec958.status[0]
            & (OXYGEN_SPDIF_NONAUDIO | OXYGEN_SPDIF_C | OXYGEN_SPDIF_PREEMPHASIS);
        bits |= (*value).value.iec958.status[1] << OXYGEN_SPDIF_CATEGORY_SHIFT;
        if (bits & OXYGEN_SPDIF_NONAUDIO) != 0 {
            bits |= OXYGEN_SPDIF_V;
        }
    }
    bits
}

unsafe extern "C" fn write_spdif_bits(chip: *mut oxygen, bits: u32) {
    unsafe {
        oxygen_write32_masked(
            chip,
            OXYGEN_SPDIF_OUTPUT_BITS,
            bits,
            OXYGEN_SPDIF_NONAUDIO
                | OXYGEN_SPDIF_C
                | OXYGEN_SPDIF_PREEMPHASIS
                | OXYGEN_SPDIF_CATEGORY_MASK
                | OXYGEN_SPDIF_ORIGINAL
                | OXYGEN_SPDIF_V,
        );
    }
}

unsafe extern "C" fn spdif_default_get(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = unsafe { (*ctl).private_data as *mut oxygen };

    guard_mutex!(unsafe { &mut (*chip).mutex });
    unsafe {
        oxygen_to_iec958((*chip).spdif_bits, value);
    }
    0
}

unsafe extern "C" fn spdif_default_put(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = unsafe { (*ctl).private_data as *mut oxygen };
    let new_bits: u32;
    let changed: c_int;

    new_bits = unsafe { iec958_to_oxygen(value) };
    guard_mutex!(unsafe { &mut (*chip).mutex });
    unsafe {
        changed = (new_bits != (*chip).spdif_bits) as c_int;
        if changed != 0 {
            (*chip).spdif_bits = new_bits;
            if ((*chip).pcm_active & (1 << PCM_SPDIF)) == 0 {
                write_spdif_bits(chip, new_bits);
            }
        }
    }
    changed
}

unsafe extern "C" fn spdif_mask_get(
    _ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        (*value).value.iec958.status[0] =
            IEC958_AES0_NONAUDIO | IEC958_AES0_CON_NOT_COPYRIGHT | IEC958_AES0_CON_EMPHASIS;
        (*value).value.iec958.status[1] = IEC958_AES1_CON_CATEGORY | IEC958_AES1_CON_ORIGINAL;
    }
    0
}

unsafe extern "C" fn spdif_pcm_get(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = unsafe { (*ctl).private_data as *mut oxygen };

    guard_mutex!(unsafe { &mut (*chip).mutex });
    unsafe {
        oxygen_to_iec958((*chip).spdif_pcm_bits, value);
    }
    0
}

unsafe extern "C" fn spdif_pcm_put(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = unsafe { (*ctl).private_data as *mut oxygen };
    let new_bits: u32;
    let changed: c_int;

    new_bits = unsafe { iec958_to_oxygen(value) };
    guard_mutex!(unsafe { &mut (*chip).mutex });
    unsafe {
        changed = (new_bits != (*chip).spdif_pcm_bits) as c_int;
        if changed != 0 {
            (*chip).spdif_pcm_bits = new_bits;
            if ((*chip).pcm_active & (1 << PCM_SPDIF)) != 0 {
                write_spdif_bits(chip, new_bits);
            }
        }
    }
    changed
}

unsafe extern "C" fn spdif_input_mask_get(
    _ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        (*value).value.iec958.status[0] = 0xff;
        (*value).value.iec958.status[1] = 0xff;
        (*value).value.iec958.status[2] = 0xff;
        (*value).value.iec958.status[3] = 0xff;
    }
    0
}

unsafe extern "C" fn spdif_input_default_get(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = unsafe { (*ctl).private_data as *mut oxygen };
    let bits: u32;

    bits = unsafe { oxygen_read32(chip, OXYGEN_SPDIF_INPUT_BITS) };
    unsafe {
        (*value).value.iec958.status[0] = bits;
        (*value).value.iec958.status[1] = bits >> 8;
        (*value).value.iec958.status[2] = bits >> 16;
        (*value).value.iec958.status[3] = bits >> 24;
    }
    0
}

unsafe extern "C" fn spdif_bit_switch_get(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = unsafe { (*ctl).private_data as *mut oxygen };
    let bit: u32 = unsafe { (*ctl).private_value as u32 };

    unsafe {
        (*value).value.integer.value[0] =
            ((oxygen_read32(chip, OXYGEN_SPDIF_CONTROL) & bit) != 0) as c_long;
    }
    0
}

unsafe extern "C" fn spdif_bit_switch_put(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = unsafe { (*ctl).private_data as *mut oxygen };
    let bit: u32 = unsafe { (*ctl).private_value as u32 };
    let oldreg: u32;
    let newreg: u32;
    let changed: c_int;

    guard_spinlock_irq!(unsafe { &mut (*chip).reg_lock });
    oldreg = unsafe { oxygen_read32(chip, OXYGEN_SPDIF_CONTROL) };
    if unsafe { (*value).value.integer.value[0] != 0 } {
        newreg = oldreg | bit;
    } else {
        newreg = oldreg & !bit;
    }
    changed = (newreg != oldreg) as c_int;
    if changed != 0 {
        unsafe { oxygen_write32(chip, OXYGEN_SPDIF_CONTROL, newreg) };
    }
    changed
}

unsafe extern "C" fn monitor_volume_info(
    _ctl: *mut snd_kcontrol,
    info: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe {
        (*info).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*info).count = 1;
        (*info).value.integer.min = 0;
        (*info).value.integer.max = 1;
    }
    0
}

unsafe extern "C" fn monitor_get(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = unsafe { (*ctl).private_data as *mut oxygen };
    let bit: u8 = unsafe { (*ctl).private_value as u8 };
    let invert: c_int = unsafe { ((*ctl).private_value & (1 << 8)) as c_int };

    unsafe {
        (*value).value.integer.value[0] =
            (((invert != 0) as c_int) ^ ((oxygen_read8(chip, OXYGEN_ADC_MONITOR) & bit) != 0) as c_int)
                as c_long;
    }
    0
}

unsafe extern "C" fn monitor_put(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = unsafe { (*ctl).private_data as *mut oxygen };
    let bit: u8 = unsafe { (*ctl).private_value as u8 };
    let invert: c_int = unsafe { ((*ctl).private_value & (1 << 8)) as c_int };
    let oldreg: u8;
    let newreg: u8;
    let changed: c_int;

    guard_spinlock_irq!(unsafe { &mut (*chip).reg_lock });
    oldreg = unsafe { oxygen_read8(chip, OXYGEN_ADC_MONITOR) };
    if unsafe { (((*value).value.integer.value[0] != 0) as c_int ^ (invert != 0) as c_int) != 0 } {
        newreg = oldreg | bit;
    } else {
        newreg = oldreg & !bit;
    }
    changed = (newreg != oldreg) as c_int;
    if changed != 0 {
        unsafe { oxygen_write8(chip, OXYGEN_ADC_MONITOR, newreg) };
    }
    changed
}

unsafe extern "C" fn ac97_switch_get(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = unsafe { (*ctl).private_data as *mut oxygen };
    let codec: c_uint = unsafe { (((*ctl).private_value >> 24) & 1) as c_uint };
    let index: c_uint = unsafe { ((*ctl).private_value & 0xff) as c_uint };
    let bitnr: c_uint = unsafe { (((*ctl).private_value >> 8) & 0xff) as c_uint };
    let invert: c_int = unsafe { ((*ctl).private_value & (1 << 16)) as c_int };
    let reg: u16;

    guard_mutex!(unsafe { &mut (*chip).mutex });
    reg = unsafe { oxygen_read_ac97(chip, codec, index) };
    unsafe {
        if (((reg & (1 << bitnr)) == 0) as c_int ^ (invert == 0) as c_int) != 0 {
            (*value).value.integer.value[0] = 1;
        } else {
            (*value).value.integer.value[0] = 0;
        }
    }
    0
}

unsafe extern "C" fn mute_ac97_ctl(chip: *mut oxygen, control: c_uint) {
    let priv_idx: c_uint;
    let value: u16;

    unsafe {
        if (*chip).controls[control as usize].is_null() {
            return;
        }
        priv_idx = ((*(*chip).controls[control as usize]).private_value & 0xff) as c_uint;
        value = oxygen_read_ac97(chip, 0, priv_idx);
        if (value & 0x8000) == 0 {
            oxygen_write_ac97(chip, 0, priv_idx, value | 0x8000);
            if let Some(ac97_switch) = (*chip).model.ac97_switch {
                ac97_switch(chip, priv_idx, 0x8000);
            }
            snd_ctl_notify(
                (*chip).card,
                SNDRV_CTL_EVENT_MASK_VALUE,
                &mut (*(*chip).controls[control as usize]).id,
            );
        }
    }
}

unsafe extern "C" fn ac97_switch_put(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = unsafe { (*ctl).private_data as *mut oxygen };
    let codec: c_uint = unsafe { (((*ctl).private_value >> 24) & 1) as c_uint };
    let index: c_uint = unsafe { ((*ctl).private_value & 0xff) as c_uint };
    let bitnr: c_uint = unsafe { (((*ctl).private_value >> 8) & 0xff) as c_uint };
    let invert: c_int = unsafe { ((*ctl).private_value & (1 << 16)) as c_int };
    let oldreg: u16;
    let mut newreg: u16;
    let change: c_int;

    guard_mutex!(unsafe { &mut (*chip).mutex });
    oldreg = unsafe { oxygen_read_ac97(chip, codec, index) };
    newreg = oldreg;
    if unsafe { (((*value).value.integer.value[0] == 0) as c_int ^ (invert == 0) as c_int) != 0 } {
        newreg |= 1 << bitnr;
    } else {
        newreg &= !(1 << bitnr);
    }
    change = (newreg != oldreg) as c_int;
    if change != 0 {
        unsafe {
            oxygen_write_ac97(chip, codec, index, newreg);
            if codec == 0 {
                if let Some(ac97_switch) = (*chip).model.ac97_switch {
                    ac97_switch(chip, index, newreg & 0x8000);
                }
            }
            if index == AC97_LINE {
                oxygen_write_ac97_masked(
                    chip,
                    0,
                    CM9780_GPIO_STATUS,
                    if (newreg & 0x8000) != 0 { CM9780_GPO0 } else { 0 },
                    CM9780_GPO0,
                );
                if (newreg & 0x8000) == 0 {
                    mute_ac97_ctl(chip, CONTROL_MIC_CAPTURE_SWITCH);
                    mute_ac97_ctl(chip, CONTROL_CD_CAPTURE_SWITCH);
                    mute_ac97_ctl(chip, CONTROL_AUX_CAPTURE_SWITCH);
                }
            } else if (index == AC97_MIC
                || index == AC97_CD
                || index == AC97_VIDEO
                || index == AC97_AUX)
                && bitnr == 15
                && (newreg & 0x8000) == 0
            {
                mute_ac97_ctl(chip, CONTROL_LINE_CAPTURE_SWITCH);
                oxygen_write_ac97_masked(chip, 0, CM9780_GPIO_STATUS, CM9780_GPO0, CM9780_GPO0);
            }
        }
    }
    change
}

unsafe extern "C" fn ac97_volume_info(
    ctl: *mut snd_kcontrol,
    info: *mut snd_ctl_elem_info,
) -> c_int {
    let stereo: c_int = unsafe { (((*ctl).private_value >> 16) & 1) as c_int };

    unsafe {
        (*info).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*info).count = if stereo != 0 { 2 } else { 1 };
        (*info).value.integer.min = 0;
        (*info).value.integer.max = 0x1f;
    }
    0
}

unsafe extern "C" fn ac97_volume_get(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = unsafe { (*ctl).private_data as *mut oxygen };
    let codec: c_uint = unsafe { (((*ctl).private_value >> 24) & 1) as c_uint };
    let stereo: c_int = unsafe { (((*ctl).private_value >> 16) & 1) as c_int };
    let index: c_uint = unsafe { ((*ctl).private_value & 0xff) as c_uint };
    let reg: u16;

    guard_mutex!(unsafe { &mut (*chip).mutex });
    reg = unsafe { oxygen_read_ac97(chip, codec, index) };
    unsafe {
        if stereo == 0 {
            (*value).value.integer.value[0] = (31 - (reg & 0x1f)) as c_long;
        } else {
            (*value).value.integer.value[0] = (31 - ((reg >> 8) & 0x1f)) as c_long;
            (*value).value.integer.value[1] = (31 - (reg & 0x1f)) as c_long;
        }
    }
    0
}

unsafe extern "C" fn ac97_volume_put(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = unsafe { (*ctl).private_data as *mut oxygen };
    let codec: c_uint = unsafe { (((*ctl).private_value >> 24) & 1) as c_uint };
    let stereo: c_int = unsafe { (((*ctl).private_value >> 16) & 1) as c_int };
    let index: c_uint = unsafe { ((*ctl).private_value & 0xff) as c_uint };
    let oldreg: u16;
    let mut newreg: u16;
    let change: c_int;

    guard_mutex!(unsafe { &mut (*chip).mutex });
    oldreg = unsafe { oxygen_read_ac97(chip, codec, index) };
    unsafe {
        if stereo == 0 {
            newreg = oldreg & !0x1f;
            newreg |= (31 - ((*value).value.integer.value[0] & 0x1f)) as u16;
        } else {
            newreg = oldreg & !0x1f1f;
            newreg |= ((31 - ((*value).value.integer.value[0] & 0x1f)) << 8) as u16;
            newreg |= (31 - ((*value).value.integer.value[1] & 0x1f)) as u16;
        }
        change = (newreg != oldreg) as c_int;
        if change != 0 {
            oxygen_write_ac97(chip, codec, index, newreg);
        }
    }
    change
}

unsafe extern "C" fn mic_fmic_source_info(
    _ctl: *mut snd_kcontrol,
    info: *mut snd_ctl_elem_info,
) -> c_int {
    static NAMES: [*const c_char; 2] = [c"Mic Jack".as_ptr(), c"Front Panel".as_ptr()];

    unsafe { snd_ctl_enum_info(info, 1, 2, NAMES.as_ptr()) }
}

unsafe extern "C" fn mic_fmic_source_get(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = unsafe { (*ctl).private_data as *mut oxygen };

    guard_mutex!(unsafe { &mut (*chip).mutex });
    unsafe {
        (*value).value.enumerated.item[0] =
            ((oxygen_read_ac97(chip, 0, CM9780_JACK) & CM9780_FMIC2MIC) != 0) as c_uint;
    }
    0
}

unsafe extern "C" fn mic_fmic_source_put(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = unsafe { (*ctl).private_data as *mut oxygen };
    let oldreg: u16;
    let newreg: u16;
    let change: c_int;

    guard_mutex!(unsafe { &mut (*chip).mutex });
    oldreg = unsafe { oxygen_read_ac97(chip, 0, CM9780_JACK) };
    if unsafe { (*value).value.enumerated.item[0] != 0 } {
        newreg = oldreg | CM9780_FMIC2MIC;
    } else {
        newreg = oldreg & !CM9780_FMIC2MIC;
    }
    change = (newreg != oldreg) as c_int;
    if change != 0 {
        unsafe { oxygen_write_ac97(chip, 0, CM9780_JACK, newreg) };
    }
    change
}

unsafe extern "C" fn ac97_fp_rec_volume_info(
    _ctl: *mut snd_kcontrol,
    info: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe {
        (*info).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*info).count = 2;
        (*info).value.integer.min = 0;
        (*info).value.integer.max = 7;
    }
    0
}

unsafe extern "C" fn ac97_fp_rec_volume_get(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = unsafe { (*ctl).private_data as *mut oxygen };
    let reg: u16;

    guard_mutex!(unsafe { &mut (*chip).mutex });
    reg = unsafe { oxygen_read_ac97(chip, 1, AC97_REC_GAIN) };
    unsafe {
        (*value).value.integer.value[0] = (reg & 7) as c_long;
        (*value).value.integer.value[1] = ((reg >> 8) & 7) as c_long;
    }
    0
}

unsafe extern "C" fn ac97_fp_rec_volume_put(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = unsafe { (*ctl).private_data as *mut oxygen };
    let oldreg: u16;
    let mut newreg: u16;
    let change: c_int;

    guard_mutex!(unsafe { &mut (*chip).mutex });
    oldreg = unsafe { oxygen_read_ac97(chip, 1, AC97_REC_GAIN) };
    unsafe {
        newreg = oldreg & !0x0707;
        newreg = newreg | ((*value).value.integer.value[0] & 7) as u16;
        newreg = newreg | (((*value).value.integer.value[1] & 7) << 8) as u16;
        change = (newreg != oldreg) as c_int;
        if change != 0 {
            oxygen_write_ac97(chip, 1, AC97_REC_GAIN, newreg);
        }
    }
    change
}

macro_rules! AC97_SWITCH {
    ($xname:expr, $codec:expr, $index:expr, $bitnr:expr, $invert:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: $xname,
            info: Some(snd_ctl_boolean_mono_info),
            get: Some(ac97_switch_get),
            put: Some(ac97_switch_put),
            private_value: (($codec) << 24) | (($invert) << 16) | (($bitnr) << 8) | ($index),
            ..unsafe { core::mem::zeroed() }
        }
    };
}

macro_rules! AC97_VOLUME {
    ($xname:expr, $codec:expr, $index:expr, $stereo:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: $xname,
            access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
            info: Some(ac97_volume_info),
            get: Some(ac97_volume_get),
            put: Some(ac97_volume_put),
            tlv: snd_kcontrol_new_tlv { p: ac97_db_scale.as_ptr() },
            private_value: (($codec) << 24) | (($stereo) << 16) | ($index),
            ..unsafe { core::mem::zeroed() }
        }
    };
}

static monitor_db_scale: [c_uint; 4] = TLV_DB_SCALE_ITEM!(-600, 600, 0);
static ac97_db_scale: [c_uint; 4] = TLV_DB_SCALE_ITEM!(-3450, 150, 0);
static ac97_rec_db_scale: [c_uint; 4] = TLV_DB_SCALE_ITEM!(0, 150, 0);

static CONTROLS: [snd_kcontrol_new; 3] = [
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c"Master Playback Volume".as_ptr(),
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
        info: Some(dac_volume_info),
        get: Some(dac_volume_get),
        put: Some(dac_volume_put),
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c"Master Playback Switch".as_ptr(),
        info: Some(snd_ctl_boolean_mono_info),
        get: Some(dac_mute_get),
        put: Some(dac_mute_put),
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c"Stereo Upmixing".as_ptr(),
        info: Some(upmix_info),
        get: Some(upmix_get),
        put: Some(upmix_put),
        ..unsafe { core::mem::zeroed() }
    },
];

static SPDIF_OUTPUT_CONTROLS: [snd_kcontrol_new; 4] = [
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: SNDRV_CTL_NAME_IEC958!("", PLAYBACK, SWITCH),
        info: Some(snd_ctl_boolean_mono_info),
        get: Some(spdif_switch_get),
        put: Some(spdif_switch_put),
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        device: 1,
        name: SNDRV_CTL_NAME_IEC958!("", PLAYBACK, DEFAULT),
        info: Some(spdif_info),
        get: Some(spdif_default_get),
        put: Some(spdif_default_put),
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        device: 1,
        name: SNDRV_CTL_NAME_IEC958!("", PLAYBACK, CON_MASK),
        access: SNDRV_CTL_ELEM_ACCESS_READ,
        info: Some(spdif_info),
        get: Some(spdif_mask_get),
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        device: 1,
        name: SNDRV_CTL_NAME_IEC958!("", PLAYBACK, PCM_STREAM),
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_INACTIVE,
        info: Some(spdif_info),
        get: Some(spdif_pcm_get),
        put: Some(spdif_pcm_put),
        ..unsafe { core::mem::zeroed() }
    },
];

static SPDIF_INPUT_CONTROLS: [snd_kcontrol_new; 4] = [
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        device: 1,
        name: SNDRV_CTL_NAME_IEC958!("", CAPTURE, MASK),
        access: SNDRV_CTL_ELEM_ACCESS_READ,
        info: Some(spdif_info),
        get: Some(spdif_input_mask_get),
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        device: 1,
        name: SNDRV_CTL_NAME_IEC958!("", CAPTURE, DEFAULT),
        access: SNDRV_CTL_ELEM_ACCESS_READ,
        info: Some(spdif_info),
        get: Some(spdif_input_default_get),
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: SNDRV_CTL_NAME_IEC958!("Loopback ", NONE, SWITCH),
        info: Some(snd_ctl_boolean_mono_info),
        get: Some(spdif_bit_switch_get),
        put: Some(spdif_bit_switch_put),
        private_value: OXYGEN_SPDIF_LOOPBACK,
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: SNDRV_CTL_NAME_IEC958!("Validity Check ", CAPTURE, SWITCH),
        info: Some(snd_ctl_boolean_mono_info),
        get: Some(spdif_bit_switch_get),
        put: Some(spdif_bit_switch_put),
        private_value: OXYGEN_SPDIF_SPDVALID,
        ..unsafe { core::mem::zeroed() }
    },
];

#[repr(C)]
struct monitor_control {
    pcm_dev: c_uint,
    controls: [snd_kcontrol_new; 2],
}

static MONITOR_CONTROLS: [monitor_control; 5] = [
    monitor_control {
        pcm_dev: CAPTURE_0_FROM_I2S_1,
        controls: [
            snd_kcontrol_new {
                iface: SNDRV_CTL_ELEM_IFACE_MIXER,
                name: c"Analog Input Monitor Playback Switch".as_ptr(),
                info: Some(snd_ctl_boolean_mono_info),
                get: Some(monitor_get),
                put: Some(monitor_put),
                private_value: OXYGEN_ADC_MONITOR_A,
                ..unsafe { core::mem::zeroed() }
            },
            snd_kcontrol_new {
                iface: SNDRV_CTL_ELEM_IFACE_MIXER,
                name: c"Analog Input Monitor Playback Volume".as_ptr(),
                access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
                info: Some(monitor_volume_info),
                get: Some(monitor_get),
                put: Some(monitor_put),
                private_value: OXYGEN_ADC_MONITOR_A_HALF_VOL | (1 << 8),
                tlv: snd_kcontrol_new_tlv { p: monitor_db_scale.as_ptr() },
                ..unsafe { core::mem::zeroed() }
            },
        ],
    },
    monitor_control {
        pcm_dev: CAPTURE_0_FROM_I2S_2,
        controls: [
            snd_kcontrol_new {
                iface: SNDRV_CTL_ELEM_IFACE_MIXER,
                name: c"Analog Input Monitor Playback Switch".as_ptr(),
                info: Some(snd_ctl_boolean_mono_info),
                get: Some(monitor_get),
                put: Some(monitor_put),
                private_value: OXYGEN_ADC_MONITOR_B,
                ..unsafe { core::mem::zeroed() }
            },
            snd_kcontrol_new {
                iface: SNDRV_CTL_ELEM_IFACE_MIXER,
                name: c"Analog Input Monitor Playback Volume".as_ptr(),
                access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
                info: Some(monitor_volume_info),
                get: Some(monitor_get),
                put: Some(monitor_put),
                private_value: OXYGEN_ADC_MONITOR_B_HALF_VOL | (1 << 8),
                tlv: snd_kcontrol_new_tlv { p: monitor_db_scale.as_ptr() },
                ..unsafe { core::mem::zeroed() }
            },
        ],
    },
    monitor_control {
        pcm_dev: CAPTURE_2_FROM_I2S_2,
        controls: [
            snd_kcontrol_new {
                iface: SNDRV_CTL_ELEM_IFACE_MIXER,
                name: c"Analog Input Monitor Playback Switch".as_ptr(),
                index: 1,
                info: Some(snd_ctl_boolean_mono_info),
                get: Some(monitor_get),
                put: Some(monitor_put),
                private_value: OXYGEN_ADC_MONITOR_B,
                ..unsafe { core::mem::zeroed() }
            },
            snd_kcontrol_new {
                iface: SNDRV_CTL_ELEM_IFACE_MIXER,
                name: c"Analog Input Monitor Playback Volume".as_ptr(),
                index: 1,
                access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
                info: Some(monitor_volume_info),
                get: Some(monitor_get),
                put: Some(monitor_put),
                private_value: OXYGEN_ADC_MONITOR_B_HALF_VOL | (1 << 8),
                tlv: snd_kcontrol_new_tlv { p: monitor_db_scale.as_ptr() },
                ..unsafe { core::mem::zeroed() }
            },
        ],
    },
    monitor_control {
        pcm_dev: CAPTURE_3_FROM_I2S_3,
        controls: [
            snd_kcontrol_new {
                iface: SNDRV_CTL_ELEM_IFACE_MIXER,
                name: c"Analog Input Monitor Playback Switch".as_ptr(),
                index: 2,
                info: Some(snd_ctl_boolean_mono_info),
                get: Some(monitor_get),
                put: Some(monitor_put),
                private_value: OXYGEN_ADC_MONITOR_C,
                ..unsafe { core::mem::zeroed() }
            },
            snd_kcontrol_new {
                iface: SNDRV_CTL_ELEM_IFACE_MIXER,
                name: c"Analog Input Monitor Playback Volume".as_ptr(),
                index: 2,
                access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
                info: Some(monitor_volume_info),
                get: Some(monitor_get),
                put: Some(monitor_put),
                private_value: OXYGEN_ADC_MONITOR_C_HALF_VOL | (1 << 8),
                tlv: snd_kcontrol_new_tlv { p: monitor_db_scale.as_ptr() },
                ..unsafe { core::mem::zeroed() }
            },
        ],
    },
    monitor_control {
        pcm_dev: CAPTURE_1_FROM_SPDIF,
        controls: [
            snd_kcontrol_new {
                iface: SNDRV_CTL_ELEM_IFACE_MIXER,
                name: c"Digital Input Monitor Playback Switch".as_ptr(),
                info: Some(snd_ctl_boolean_mono_info),
                get: Some(monitor_get),
                put: Some(monitor_put),
                private_value: OXYGEN_ADC_MONITOR_C,
                ..unsafe { core::mem::zeroed() }
            },
            snd_kcontrol_new {
                iface: SNDRV_CTL_ELEM_IFACE_MIXER,
                name: c"Digital Input Monitor Playback Volume".as_ptr(),
                access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
                info: Some(monitor_volume_info),
                get: Some(monitor_get),
                put: Some(monitor_put),
                private_value: OXYGEN_ADC_MONITOR_C_HALF_VOL | (1 << 8),
                tlv: snd_kcontrol_new_tlv { p: monitor_db_scale.as_ptr() },
                ..unsafe { core::mem::zeroed() }
            },
        ],
    },
];

static AC97_CONTROLS: [snd_kcontrol_new; 9] = [
    AC97_VOLUME!(c"Mic Capture Volume".as_ptr(), 0, AC97_MIC, 0),
    AC97_SWITCH!(c"Mic Capture Switch".as_ptr(), 0, AC97_MIC, 15, 1),
    AC97_SWITCH!(c"Mic Boost (+20dB)".as_ptr(), 0, AC97_MIC, 6, 0),
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c"Mic Source Capture Enum".as_ptr(),
        info: Some(mic_fmic_source_info),
        get: Some(mic_fmic_source_get),
        put: Some(mic_fmic_source_put),
        ..unsafe { core::mem::zeroed() }
    },
    AC97_SWITCH!(c"Line Capture Switch".as_ptr(), 0, AC97_LINE, 15, 1),
    AC97_VOLUME!(c"CD Capture Volume".as_ptr(), 0, AC97_CD, 1),
    AC97_SWITCH!(c"CD Capture Switch".as_ptr(), 0, AC97_CD, 15, 1),
    AC97_VOLUME!(c"Aux Capture Volume".as_ptr(), 0, AC97_AUX, 1),
    AC97_SWITCH!(c"Aux Capture Switch".as_ptr(), 0, AC97_AUX, 15, 1),
];

static AC97_FP_CONTROLS: [snd_kcontrol_new; 4] = [
    AC97_VOLUME!(c"Front Panel Playback Volume".as_ptr(), 1, AC97_HEADPHONE, 1),
    AC97_SWITCH!(c"Front Panel Playback Switch".as_ptr(), 1, AC97_HEADPHONE, 15, 1),
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c"Front Panel Capture Volume".as_ptr(),
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
        info: Some(ac97_fp_rec_volume_info),
        get: Some(ac97_fp_rec_volume_get),
        put: Some(ac97_fp_rec_volume_put),
        tlv: snd_kcontrol_new_tlv { p: ac97_rec_db_scale.as_ptr() },
        ..unsafe { core::mem::zeroed() }
    },
    AC97_SWITCH!(c"Front Panel Capture Switch".as_ptr(), 1, AC97_REC_GAIN, 15, 1),
];

unsafe extern "C" fn oxygen_any_ctl_free(ctl: *mut snd_kcontrol) {
    let chip = unsafe { (*ctl).private_data as *mut oxygen };
    let mut i: c_uint;

    /* I'm too lazy to write a function for each control :-) */
    i = 0;
    while i < unsafe { ARRAY_SIZE!((*chip).controls) as c_uint } {
        unsafe {
            (*chip).controls[i as usize] = core::ptr::null_mut();
        }
        i += 1;
    }
}

unsafe extern "C" fn add_controls(
    chip: *mut oxygen,
    controls: *const snd_kcontrol_new,
    count: c_uint,
) -> c_int {
    static KNOWN_CTL_NAMES: [*const c_char; CONTROL_COUNT as usize] = {
        let mut names = [core::ptr::null(); CONTROL_COUNT as usize];
        names[CONTROL_SPDIF_PCM as usize] = SNDRV_CTL_NAME_IEC958!("", PLAYBACK, PCM_STREAM);
        names[CONTROL_SPDIF_INPUT_BITS as usize] = SNDRV_CTL_NAME_IEC958!("", CAPTURE, DEFAULT);
        names[CONTROL_MIC_CAPTURE_SWITCH as usize] = c"Mic Capture Switch".as_ptr();
        names[CONTROL_LINE_CAPTURE_SWITCH as usize] = c"Line Capture Switch".as_ptr();
        names[CONTROL_CD_CAPTURE_SWITCH as usize] = c"CD Capture Switch".as_ptr();
        names[CONTROL_AUX_CAPTURE_SWITCH as usize] = c"Aux Capture Switch".as_ptr();
        names
    };
    let mut i: c_uint;
    let mut template: snd_kcontrol_new;
    let ctl: *mut snd_kcontrol;
    let mut j: c_int;
    let mut err: c_int;

    i = 0;
    while i < count {
        unsafe {
            template = *controls.add(i as usize);
            if let Some(control_filter) = (*chip).model.control_filter {
                err = control_filter(&mut template);
                if err < 0 {
                    return err;
                }
                if err == 1 {
                    i += 1;
                    continue;
                }
            }
            if strcmp(template.name, c"Stereo Upmixing".as_ptr()) == 0
                && (*chip).model.dac_channels_pcm == 2
            {
                i += 1;
                continue;
            }
            if strcmp(template.name, c"Mic Source Capture Enum".as_ptr()) == 0
                && ((*chip).model.device_config & AC97_FMIC_SWITCH) == 0
            {
                i += 1;
                continue;
            }
            if strncmp(template.name, c"CD Capture ".as_ptr(), 11) == 0
                && ((*chip).model.device_config & AC97_CD_INPUT) == 0
            {
                i += 1;
                continue;
            }
            if strcmp(template.name, c"Master Playback Volume".as_ptr()) == 0
                && !(*chip).model.dac_tlv.is_null()
            {
                template.tlv.p = (*chip).model.dac_tlv;
                template.access |= SNDRV_CTL_ELEM_ACCESS_TLV_READ;
            }
            ctl = snd_ctl_new1(&mut template, chip as *mut c_void);
            if ctl.is_null() {
                return -ENOMEM;
            }
            err = snd_ctl_add((*chip).card, ctl);
            if err < 0 {
                return err;
            }
            j = match_string(KNOWN_CTL_NAMES.as_ptr(), CONTROL_COUNT, (*ctl).id.name.as_ptr());
            if j >= 0 {
                (*chip).controls[j as usize] = ctl;
                (*ctl).private_free = Some(oxygen_any_ctl_free);
            }
        }
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn oxygen_mixer_init(chip: *mut oxygen) -> c_int {
    let mut i: c_uint;
    let mut err: c_int;

    err = unsafe { add_controls(chip, CONTROLS.as_ptr(), ARRAY_SIZE!(CONTROLS) as c_uint) };
    if err < 0 {
        return err;
    }
    unsafe {
        if ((*chip).model.device_config & PLAYBACK_1_TO_SPDIF) != 0 {
            err = add_controls(
                chip,
                SPDIF_OUTPUT_CONTROLS.as_ptr(),
                ARRAY_SIZE!(SPDIF_OUTPUT_CONTROLS) as c_uint,
            );
            if err < 0 {
                return err;
            }
        }
        if ((*chip).model.device_config & CAPTURE_1_FROM_SPDIF) != 0 {
            err = add_controls(
                chip,
                SPDIF_INPUT_CONTROLS.as_ptr(),
                ARRAY_SIZE!(SPDIF_INPUT_CONTROLS) as c_uint,
            );
            if err < 0 {
                return err;
            }
        }
        i = 0;
        while i < ARRAY_SIZE!(MONITOR_CONTROLS) as c_uint {
            if ((*chip).model.device_config & MONITOR_CONTROLS[i as usize].pcm_dev) == 0 {
                i += 1;
                continue;
            }
            err = add_controls(
                chip,
                MONITOR_CONTROLS[i as usize].controls.as_ptr(),
                ARRAY_SIZE!(MONITOR_CONTROLS[i as usize].controls) as c_uint,
            );
            if err < 0 {
                return err;
            }
            i += 1;
        }
        if (*chip).has_ac97_0 != 0 {
            err = add_controls(chip, AC97_CONTROLS.as_ptr(), ARRAY_SIZE!(AC97_CONTROLS) as c_uint);
            if err < 0 {
                return err;
            }
        }
        if (*chip).has_ac97_1 != 0 {
            err = add_controls(
                chip,
                AC97_FP_CONTROLS.as_ptr(),
                ARRAY_SIZE!(AC97_FP_CONTROLS) as c_uint,
            );
            if err < 0 {
                return err;
            }
        }
        if let Some(mixer_init) = (*chip).model.mixer_init {
            mixer_init(chip)
        } else {
            0
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
