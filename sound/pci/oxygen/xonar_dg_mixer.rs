// SPDX-License-Identifier: GPL-2.0-only
/*
 * Mixer controls for the Xonar DG/DGX
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 * Copyright (c) Roman Volkov <v1ron@mail.ru>
 */

use core::ffi::{c_char, c_int, c_long, c_uint};

/* analog output select */

unsafe fn output_select_apply(chip: *mut oxygen) -> c_int {
    let data = (*chip).model_data as *mut dg;

    (*data).cs4245_shadow[CS4245_SIGNAL_SEL as usize] &= !CS4245_A_OUT_SEL_MASK;
    if (*data).output_sel == PLAYBACK_DST_HP {
        /* mute FP (aux output) amplifier, switch rear jack to CS4245 */
        oxygen_set_bits8(chip, OXYGEN_GPIO_DATA, GPIO_HP_REAR);
    } else if (*data).output_sel == PLAYBACK_DST_HP_FP {
        /*
         * Unmute FP amplifier, switch rear jack to CS4361;
         * I2S channels 2,3,4 should be inactive.
         */
        oxygen_clear_bits8(chip, OXYGEN_GPIO_DATA, GPIO_HP_REAR);
        (*data).cs4245_shadow[CS4245_SIGNAL_SEL as usize] |= CS4245_A_OUT_SEL_DAC;
    } else {
        /*
         * 2.0, 4.0, 5.1: switch to CS4361, mute FP amp.,
         * and change playback routing.
         */
        oxygen_clear_bits8(chip, OXYGEN_GPIO_DATA, GPIO_HP_REAR);
    }
    cs4245_write_spi(chip, CS4245_SIGNAL_SEL)
}

unsafe fn output_select_info(
    _ctl: *mut snd_kcontrol,
    info: *mut snd_ctl_elem_info,
) -> c_int {
    static NAMES: [*const c_char; 3] = [
        b"Stereo Headphones\0".as_ptr() as *const c_char,
        b"Stereo Headphones FP\0".as_ptr() as *const c_char,
        b"Multichannel\0".as_ptr() as *const c_char,
    ];

    snd_ctl_enum_info(info, 1, 3, NAMES.as_ptr())
}

unsafe fn output_select_get(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = (*ctl).private_data as *mut oxygen;
    let data = (*chip).model_data as *mut dg;

    let _guard = guard_mutex(&mut (*chip).mutex);
    (*value).value.enumerated.item[0] = (*data).output_sel;
    0
}

unsafe fn output_select_put(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = (*ctl).private_data as *mut oxygen;
    let data = (*chip).model_data as *mut dg;
    let new = (*value).value.enumerated.item[0] as c_uint;
    let mut changed = 0;
    let ret: c_int;

    let _guard = guard_mutex(&mut (*chip).mutex);
    if (*data).output_sel != new {
        (*data).output_sel = new;
        ret = output_select_apply(chip);
        changed = if ret >= 0 { 1 } else { ret };
        oxygen_update_dac_routing(chip);
    }

    changed
}

/* CS4245 Headphone Channels A&B Volume Control */

unsafe fn hp_stereo_volume_info(
    _ctl: *mut snd_kcontrol,
    info: *mut snd_ctl_elem_info,
) -> c_int {
    (*info).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*info).count = 2;
    (*info).value.integer.min = 0;
    (*info).value.integer.max = 255;
    0
}

unsafe fn hp_stereo_volume_get(
    ctl: *mut snd_kcontrol,
    val: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = (*ctl).private_data as *mut oxygen;
    let data = (*chip).model_data as *mut dg;
    let mut tmp: c_uint;

    let _guard = guard_mutex(&mut (*chip).mutex);
    tmp = (!((*data).cs4245_shadow[CS4245_DAC_A_CTRL as usize] as c_uint)) & 255;
    (*val).value.integer.value[0] = tmp as c_long;
    tmp = (!((*data).cs4245_shadow[CS4245_DAC_B_CTRL as usize] as c_uint)) & 255;
    (*val).value.integer.value[1] = tmp as c_long;
    0
}

unsafe fn hp_stereo_volume_put(
    ctl: *mut snd_kcontrol,
    val: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = (*ctl).private_data as *mut oxygen;
    let data = (*chip).model_data as *mut dg;
    let mut ret: c_int;
    let mut changed = 0;
    let new1 = (*val).value.integer.value[0];
    let new2 = (*val).value.integer.value[1];

    if new1 > 255 || new1 < 0 || new2 > 255 || new2 < 0 {
        return -EINVAL;
    }

    let _guard = guard_mutex(&mut (*chip).mutex);
    if (*data).cs4245_shadow[CS4245_DAC_A_CTRL as usize] != (!new1) as u8
        || (*data).cs4245_shadow[CS4245_DAC_B_CTRL as usize] != (!new2) as u8
    {
        (*data).cs4245_shadow[CS4245_DAC_A_CTRL as usize] = (!new1) as u8;
        (*data).cs4245_shadow[CS4245_DAC_B_CTRL as usize] = (!new2) as u8;
        ret = cs4245_write_spi(chip, CS4245_DAC_A_CTRL);
        if ret >= 0 {
            ret = cs4245_write_spi(chip, CS4245_DAC_B_CTRL);
        }
        changed = if ret >= 0 { 1 } else { ret };
    }

    changed
}

/* Headphone Mute */

unsafe fn hp_mute_get(
    ctl: *mut snd_kcontrol,
    val: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = (*ctl).private_data as *mut oxygen;
    let data = (*chip).model_data as *mut dg;

    let _guard = guard_mutex(&mut (*chip).mutex);
    (*val).value.integer.value[0] =
        (((*data).cs4245_shadow[CS4245_DAC_CTRL_1 as usize] & CS4245_MUTE_DAC) == 0) as c_long;
    0
}

unsafe fn hp_mute_put(
    ctl: *mut snd_kcontrol,
    val: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = (*ctl).private_data as *mut oxygen;
    let data = (*chip).model_data as *mut dg;
    let ret: c_int;
    let changed: c_int;

    if (*val).value.integer.value[0] > 1 {
        return -EINVAL;
    }
    let _guard = guard_mutex(&mut (*chip).mutex);
    (*data).cs4245_shadow[CS4245_DAC_CTRL_1 as usize] &= !CS4245_MUTE_DAC;
    (*data).cs4245_shadow[CS4245_DAC_CTRL_1 as usize] |=
        ((!(*val).value.integer.value[0] << 2) as u8) & CS4245_MUTE_DAC;
    ret = cs4245_write_spi(chip, CS4245_DAC_CTRL_1);
    changed = if ret >= 0 { 1 } else { ret };
    changed
}

/* capture volume for all sources */

unsafe fn input_volume_apply(chip: *mut oxygen, left: c_char, right: c_char) -> c_int {
    let data = (*chip).model_data as *mut dg;
    let ret: c_int;

    (*data).cs4245_shadow[CS4245_PGA_A_CTRL as usize] = left as u8;
    (*data).cs4245_shadow[CS4245_PGA_B_CTRL as usize] = right as u8;
    ret = cs4245_write_spi(chip, CS4245_PGA_A_CTRL);
    if ret < 0 {
        return ret;
    }
    cs4245_write_spi(chip, CS4245_PGA_B_CTRL)
}

unsafe fn input_vol_info(
    _ctl: *mut snd_kcontrol,
    info: *mut snd_ctl_elem_info,
) -> c_int {
    (*info).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*info).count = 2;
    (*info).value.integer.min = 2 * -12;
    (*info).value.integer.max = 2 * 12;
    0
}

unsafe fn input_vol_get(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = (*ctl).private_data as *mut oxygen;
    let data = (*chip).model_data as *mut dg;
    let idx = (*ctl).private_value as c_uint;

    let _guard = guard_mutex(&mut (*chip).mutex);
    (*value).value.integer.value[0] = (*data).input_vol[idx as usize][0] as c_long;
    (*value).value.integer.value[1] = (*data).input_vol[idx as usize][1] as c_long;
    0
}

unsafe fn input_vol_put(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = (*ctl).private_data as *mut oxygen;
    let data = (*chip).model_data as *mut dg;
    let idx = (*ctl).private_value as c_uint;
    let mut changed = 0;
    let mut ret = 0;

    if (*value).value.integer.value[0] < 2 * -12
        || (*value).value.integer.value[0] > 2 * 12
        || (*value).value.integer.value[1] < 2 * -12
        || (*value).value.integer.value[1] > 2 * 12
    {
        return -EINVAL;
    }
    let _guard = guard_mutex(&mut (*chip).mutex);
    changed = ((*data).input_vol[idx as usize][0] as c_long != (*value).value.integer.value[0]
        || (*data).input_vol[idx as usize][1] as c_long != (*value).value.integer.value[1])
        as c_int;
    if changed != 0 {
        (*data).input_vol[idx as usize][0] = (*value).value.integer.value[0] as c_char;
        (*data).input_vol[idx as usize][1] = (*value).value.integer.value[1] as c_char;
        if idx == (*data).input_sel {
            ret = input_volume_apply(
                chip,
                (*data).input_vol[idx as usize][0],
                (*data).input_vol[idx as usize][1],
            );
        }
        changed = if ret >= 0 { 1 } else { ret };
    }
    changed
}

/* Capture Source */

unsafe fn input_source_apply(chip: *mut oxygen) -> c_int {
    let data = (*chip).model_data as *mut dg;

    (*data).cs4245_shadow[CS4245_ANALOG_IN as usize] &= !CS4245_SEL_MASK;
    if (*data).input_sel == CAPTURE_SRC_FP_MIC {
        (*data).cs4245_shadow[CS4245_ANALOG_IN as usize] |= CS4245_SEL_INPUT_2;
    } else if (*data).input_sel == CAPTURE_SRC_LINE {
        (*data).cs4245_shadow[CS4245_ANALOG_IN as usize] |= CS4245_SEL_INPUT_4;
    } else if (*data).input_sel != CAPTURE_SRC_MIC {
        (*data).cs4245_shadow[CS4245_ANALOG_IN as usize] |= CS4245_SEL_INPUT_1;
    }
    cs4245_write_spi(chip, CS4245_ANALOG_IN)
}

unsafe fn input_sel_info(
    _ctl: *mut snd_kcontrol,
    info: *mut snd_ctl_elem_info,
) -> c_int {
    static NAMES: [*const c_char; 4] = [
        b"Mic\0".as_ptr() as *const c_char,
        b"Front Mic\0".as_ptr() as *const c_char,
        b"Line\0".as_ptr() as *const c_char,
        b"Aux\0".as_ptr() as *const c_char,
    ];

    snd_ctl_enum_info(info, 1, 4, NAMES.as_ptr())
}

unsafe fn input_sel_get(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = (*ctl).private_data as *mut oxygen;
    let data = (*chip).model_data as *mut dg;

    let _guard = guard_mutex(&mut (*chip).mutex);
    (*value).value.enumerated.item[0] = (*data).input_sel;
    0
}

unsafe fn input_sel_put(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = (*ctl).private_data as *mut oxygen;
    let data = (*chip).model_data as *mut dg;
    let mut changed: c_int;
    let mut ret: c_int;

    if (*value).value.enumerated.item[0] > 3 {
        return -EINVAL;
    }

    let _guard = guard_mutex(&mut (*chip).mutex);
    changed = ((*value).value.enumerated.item[0] != (*data).input_sel) as c_int;
    if changed != 0 {
        (*data).input_sel = (*value).value.enumerated.item[0];

        ret = input_source_apply(chip);
        if ret >= 0 {
            ret = input_volume_apply(
                chip,
                (*data).input_vol[(*data).input_sel as usize][0],
                (*data).input_vol[(*data).input_sel as usize][1],
            );
        }
        changed = if ret >= 0 { 1 } else { ret };
    }
    changed
}

/* ADC high-pass filter */

unsafe fn hpf_info(_ctl: *mut snd_kcontrol, info: *mut snd_ctl_elem_info) -> c_int {
    static NAMES: [*const c_char; 2] = [
        b"Active\0".as_ptr() as *const c_char,
        b"Frozen\0".as_ptr() as *const c_char,
    ];

    snd_ctl_enum_info(info, 1, 2, NAMES.as_ptr())
}

unsafe fn hpf_get(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = (*ctl).private_data as *mut oxygen;
    let data = (*chip).model_data as *mut dg;

    (*value).value.enumerated.item[0] =
        (((*data).cs4245_shadow[CS4245_ADC_CTRL as usize] & CS4245_HPF_FREEZE) != 0) as c_uint;
    0
}

unsafe fn hpf_put(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = (*ctl).private_data as *mut oxygen;
    let data = (*chip).model_data as *mut dg;
    let mut reg: u8;
    let changed: c_int;

    let _guard = guard_mutex(&mut (*chip).mutex);
    reg = (*data).cs4245_shadow[CS4245_ADC_CTRL as usize] & !CS4245_HPF_FREEZE;
    if (*value).value.enumerated.item[0] != 0 {
        reg |= CS4245_HPF_FREEZE;
    }
    changed = (reg != (*data).cs4245_shadow[CS4245_ADC_CTRL as usize]) as c_int;
    if changed != 0 {
        (*data).cs4245_shadow[CS4245_ADC_CTRL as usize] = reg;
        cs4245_write_spi(chip, CS4245_ADC_CTRL);
    }
    changed
}

macro_rules! INPUT_VOLUME {
    ($xname:expr, $index:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: $xname.as_ptr() as *const c_char,
            access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
            info: Some(input_vol_info),
            get: Some(input_vol_get),
            put: Some(input_vol_put),
            tlv: snd_kcontrol_new_tlv {
                p: pga_db_scale.as_ptr(),
            },
            private_value: $index as _,
            ..unsafe { core::mem::zeroed() }
        }
    };
}

static hp_db_scale: [u32; 4] = TLV_DB_MINMAX_ITEM(-12550, 0);
static pga_db_scale: [u32; 4] = TLV_DB_MINMAX_ITEM(-1200, 1200);

static dg_controls: [snd_kcontrol_new; 9] = [
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Analog Output Playback Enum\0".as_ptr() as *const c_char,
        info: Some(output_select_info),
        get: Some(output_select_get),
        put: Some(output_select_put),
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Headphone Playback Volume\0".as_ptr() as *const c_char,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
        info: Some(hp_stereo_volume_info),
        get: Some(hp_stereo_volume_get),
        put: Some(hp_stereo_volume_put),
        tlv: snd_kcontrol_new_tlv {
            p: hp_db_scale.as_ptr(),
        },
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Headphone Playback Switch\0".as_ptr() as *const c_char,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
        info: Some(snd_ctl_boolean_mono_info),
        get: Some(hp_mute_get),
        put: Some(hp_mute_put),
        ..unsafe { core::mem::zeroed() }
    },
    INPUT_VOLUME!(b"Mic Capture Volume\0", CAPTURE_SRC_MIC),
    INPUT_VOLUME!(b"Front Mic Capture Volume\0", CAPTURE_SRC_FP_MIC),
    INPUT_VOLUME!(b"Line Capture Volume\0", CAPTURE_SRC_LINE),
    INPUT_VOLUME!(b"Aux Capture Volume\0", CAPTURE_SRC_AUX),
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Capture Source\0".as_ptr() as *const c_char,
        info: Some(input_sel_info),
        get: Some(input_sel_get),
        put: Some(input_sel_put),
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"ADC High-pass Filter Capture Enum\0".as_ptr() as *const c_char,
        info: Some(hpf_info),
        get: Some(hpf_get),
        put: Some(hpf_put),
        ..unsafe { core::mem::zeroed() }
    },
];

unsafe fn dg_control_filter(template: *mut snd_kcontrol_new) -> c_int {
    if strncmp((*template).name, b"Master Playback \0".as_ptr() as *const c_char, 16) == 0 {
        return 1;
    }
    0
}

unsafe fn dg_mixer_init(chip: *mut oxygen) -> c_int {
    let mut i: c_uint;
    let mut err: c_int;

    output_select_apply(chip);
    input_source_apply(chip);
    oxygen_update_dac_routing(chip);

    i = 0;
    while (i as usize) < dg_controls.len() {
        err = snd_ctl_add(
            (*chip).card,
            snd_ctl_new1(&dg_controls[i as usize], chip as *mut _),
        );
        if err < 0 {
            return err;
        }
        i += 1;
    }

    0
}

pub static model_xonar_dg: oxygen_model = oxygen_model {
    longname: b"C-Media Oxygen HD Audio\0".as_ptr() as *const c_char,
    chip: b"CMI8786\0".as_ptr() as *const c_char,
    init: Some(dg_init),
    control_filter: Some(dg_control_filter),
    mixer_init: Some(dg_mixer_init),
    cleanup: Some(dg_cleanup),
    suspend: Some(dg_suspend),
    resume: Some(dg_resume),
    set_dac_params: Some(set_cs4245_dac_params),
    set_adc_params: Some(set_cs4245_adc_params),
    adjust_dac_routing: Some(adjust_dg_dac_routing),
    dump_registers: Some(dump_cs4245_registers),
    model_data_size: core::mem::size_of::<dg>(),
    device_config: PLAYBACK_0_TO_I2S
        | PLAYBACK_1_TO_SPDIF
        | CAPTURE_0_FROM_I2S_1
        | CAPTURE_1_FROM_SPDIF,
    dac_channels_pcm: 6,
    dac_channels_mixer: 0,
    function_flags: OXYGEN_FUNCTION_SPI,
    dac_mclks: OXYGEN_MCLKS(256, 128, 128),
    adc_mclks: OXYGEN_MCLKS(256, 128, 128),
    dac_i2s_format: OXYGEN_I2S_FORMAT_LJUST,
    adc_i2s_format: OXYGEN_I2S_FORMAT_LJUST,
    ..unsafe { core::mem::zeroed() }
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
