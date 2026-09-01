// SPDX-License-Identifier: GPL-2.0-or-later
// Driver for Digigram pcxhr compatible soundcards
//
// mixer callbacks
//
// Copyright (c) 2004 by Digigram <alsa@digigram.com>
//
// Translated from pcxhr_mixer.c. C include dependencies are expected to be
// provided by the surrounding Rust translation/bindings.

const PCXHR_LINE_CAPTURE_LEVEL_MIN: i32 = 0; /* -112.0 dB */
const PCXHR_LINE_CAPTURE_LEVEL_MAX: i32 = 255; /* +15.5 dB */
const PCXHR_LINE_CAPTURE_ZERO_LEVEL: i32 = 224; /* 0.0 dB ( 0 dBu -> 0 dBFS ) */

const PCXHR_LINE_PLAYBACK_LEVEL_MIN: i32 = 0; /* -104.0 dB */
const PCXHR_LINE_PLAYBACK_LEVEL_MAX: i32 = 128; /* +24.0 dB */
const PCXHR_LINE_PLAYBACK_ZERO_LEVEL: i32 = 104; /* 0.0 dB ( 0 dBFS -> 0 dBu ) */

static db_scale_analog_capture: [u32; 4] = declare_tlv_db_scale(-11200, 50, 1550);
static db_scale_analog_playback: [u32; 4] = declare_tlv_db_scale(-10400, 100, 2400);

static db_scale_a_hr222_capture: [u32; 4] = declare_tlv_db_scale(-11150, 50, 1600);
static db_scale_a_hr222_playback: [u32; 4] = declare_tlv_db_scale(-2550, 50, 2400);

unsafe fn pcxhr_update_analog_audio_level(chip: *mut snd_pcxhr, is_capture: i32, channel: i32) -> i32 {
    let mut err: i32;
    let mut vol: i32;
    let mut rmh: pcxhr_rmh = core::mem::zeroed();

    pcxhr_init_rmh(&mut rmh, CMD_ACCESS_IO_WRITE);
    if is_capture != 0 {
        rmh.cmd[0] |= IO_NUM_REG_IN_ANA_LEVEL;
        rmh.cmd[2] = (*chip).analog_capture_volume[channel as usize];
    } else {
        rmh.cmd[0] |= IO_NUM_REG_OUT_ANA_LEVEL;
        if (*chip).analog_playback_active[channel as usize] != 0 {
            vol = (*chip).analog_playback_volume[channel as usize];
        } else {
            vol = PCXHR_LINE_PLAYBACK_LEVEL_MIN;
        }
        /* playback analog levels are inversed */
        rmh.cmd[2] = PCXHR_LINE_PLAYBACK_LEVEL_MAX - vol;
    }
    rmh.cmd[1] = 1 << ((2 * (*chip).chip_idx) + channel); /* audio mask */
    rmh.cmd_len = 3;
    err = pcxhr_send_msg((*chip).mgr, &mut rmh);
    if err < 0 {
        dev_dbg((*(*chip).card).dev, c_str!("error update_analog_audio_level card(%d) is_capture(%d) err(%x)\n"), (*chip).chip_idx, is_capture, err);
        return -EINVAL;
    }
    0
}

/*
 * analog level control
 */
unsafe fn pcxhr_analog_vol_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> i32 {
    let chip: *mut snd_pcxhr = snd_kcontrol_chip(kcontrol) as *mut snd_pcxhr;

    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    if (*kcontrol).private_value == 0 {
        /* playback */
        if (*(*chip).mgr).is_hr_stereo != 0 {
            (*uinfo).value.integer.min = HR222_LINE_PLAYBACK_LEVEL_MIN; /* -25 dB */
            (*uinfo).value.integer.max = HR222_LINE_PLAYBACK_LEVEL_MAX; /* +24 dB */
        } else {
            (*uinfo).value.integer.min = PCXHR_LINE_PLAYBACK_LEVEL_MIN; /*-104 dB */
            (*uinfo).value.integer.max = PCXHR_LINE_PLAYBACK_LEVEL_MAX; /* +24 dB */
        }
    } else {
        /* capture */
        if (*(*chip).mgr).is_hr_stereo != 0 {
            (*uinfo).value.integer.min = HR222_LINE_CAPTURE_LEVEL_MIN; /*-112 dB */
            (*uinfo).value.integer.max = HR222_LINE_CAPTURE_LEVEL_MAX; /* +15.5 dB */
        } else {
            (*uinfo).value.integer.min = PCXHR_LINE_CAPTURE_LEVEL_MIN; /*-112 dB */
            (*uinfo).value.integer.max = PCXHR_LINE_CAPTURE_LEVEL_MAX; /* +15.5 dB */
        }
    }
    0
}

unsafe fn pcxhr_analog_vol_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let chip: *mut snd_pcxhr = snd_kcontrol_chip(kcontrol) as *mut snd_pcxhr;

    let _guard = mutex_guard(&mut (*(*chip).mgr).mixer_mutex);
    if (*kcontrol).private_value == 0 {
        /* playback */
        (*ucontrol).value.integer.value[0] = (*chip).analog_playback_volume[0] as _;
        (*ucontrol).value.integer.value[1] = (*chip).analog_playback_volume[1] as _;
    } else {
        /* capture */
        (*ucontrol).value.integer.value[0] = (*chip).analog_capture_volume[0] as _;
        (*ucontrol).value.integer.value[1] = (*chip).analog_capture_volume[1] as _;
    }
    0
}

unsafe fn pcxhr_analog_vol_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let chip: *mut snd_pcxhr = snd_kcontrol_chip(kcontrol) as *mut snd_pcxhr;
    let mut changed: i32 = 0;
    let is_capture: i32;

    let _guard = mutex_guard(&mut (*(*chip).mgr).mixer_mutex);
    is_capture = ((*kcontrol).private_value != 0) as i32;
    for i in 0..2 {
        let new_volume: i32 = (*ucontrol).value.integer.value[i] as i32;
        let stored_volume: *mut i32 = if is_capture != 0 {
            &mut (*chip).analog_capture_volume[i]
        } else {
            &mut (*chip).analog_playback_volume[i]
        };
        if is_capture != 0 {
            if (*(*chip).mgr).is_hr_stereo != 0 {
                if new_volume < HR222_LINE_CAPTURE_LEVEL_MIN || new_volume > HR222_LINE_CAPTURE_LEVEL_MAX {
                    continue;
                }
            } else if new_volume < PCXHR_LINE_CAPTURE_LEVEL_MIN || new_volume > PCXHR_LINE_CAPTURE_LEVEL_MAX {
                continue;
            }
        } else if (*(*chip).mgr).is_hr_stereo != 0 {
            if new_volume < HR222_LINE_PLAYBACK_LEVEL_MIN || new_volume > HR222_LINE_PLAYBACK_LEVEL_MAX {
                continue;
            }
        } else if new_volume < PCXHR_LINE_PLAYBACK_LEVEL_MIN || new_volume > PCXHR_LINE_PLAYBACK_LEVEL_MAX {
            continue;
        }
        if *stored_volume != new_volume {
            *stored_volume = new_volume;
            changed = 1;
            if (*(*chip).mgr).is_hr_stereo != 0 {
                hr222_update_analog_audio_level(chip, is_capture, i as i32);
            } else {
                pcxhr_update_analog_audio_level(chip, is_capture, i as i32);
            }
        }
    }
    changed
}

static pcxhr_control_analog_level: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
    name: core::ptr::null(),
    info: Some(pcxhr_analog_vol_info),
    get: Some(pcxhr_analog_vol_get),
    put: Some(pcxhr_analog_vol_put),
    ..unsafe { core::mem::zeroed() }
};

/* shared */
const pcxhr_sw_info: snd_kcontrol_info_t = snd_ctl_boolean_stereo_info;

unsafe fn pcxhr_audio_sw_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let chip: *mut snd_pcxhr = snd_kcontrol_chip(kcontrol) as *mut snd_pcxhr;

    let _guard = mutex_guard(&mut (*(*chip).mgr).mixer_mutex);
    (*ucontrol).value.integer.value[0] = (*chip).analog_playback_active[0] as _;
    (*ucontrol).value.integer.value[1] = (*chip).analog_playback_active[1] as _;
    0
}

unsafe fn pcxhr_audio_sw_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let chip: *mut snd_pcxhr = snd_kcontrol_chip(kcontrol) as *mut snd_pcxhr;
    let mut changed: i32 = 0;

    let _guard = mutex_guard(&mut (*(*chip).mgr).mixer_mutex);
    for i in 0..2 {
        if (*chip).analog_playback_active[i] != (*ucontrol).value.integer.value[i] as i32 {
            (*chip).analog_playback_active[i] = ((*ucontrol).value.integer.value[i] != 0) as i32;
            changed = 1;
            /* update playback levels */
            if (*(*chip).mgr).is_hr_stereo != 0 {
                hr222_update_analog_audio_level(chip, 0, i as i32);
            } else {
                pcxhr_update_analog_audio_level(chip, 0, i as i32);
            }
        }
    }
    changed
}

static pcxhr_control_output_switch: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c_str!("Master Playback Switch"),
    info: Some(pcxhr_sw_info),
    get: Some(pcxhr_audio_sw_get),
    put: Some(pcxhr_audio_sw_put),
    ..unsafe { core::mem::zeroed() }
};

const PCXHR_DIGITAL_LEVEL_MIN: i32 = 0x000; /* -110 dB */
const PCXHR_DIGITAL_LEVEL_MAX: i32 = 0x1ff; /* +18 dB */
const PCXHR_DIGITAL_ZERO_LEVEL: i32 = 0x1b7; /*  0 dB */

static db_scale_digital: [u32; 4] = declare_tlv_db_scale(-10975, 25, 1800);

const MORE_THAN_ONE_STREAM_LEVEL: i32 = 0x000001;
const VALID_STREAM_PAN_LEVEL_MASK: i32 = 0x800000;
const VALID_STREAM_LEVEL_MASK: i32 = 0x400000;
const VALID_STREAM_LEVEL_1_MASK: i32 = 0x200000;
const VALID_STREAM_LEVEL_2_MASK: i32 = 0x100000;

unsafe fn pcxhr_update_playback_stream_level(chip: *mut snd_pcxhr, idx: i32) -> i32 {
    let mut err: i32;
    let mut rmh: pcxhr_rmh = core::mem::zeroed();
    let pipe: *mut pcxhr_pipe = &mut (*chip).playback_pipe;
    let left: i32;
    let right: i32;

    if (*chip).digital_playback_active[idx as usize][0] != 0 {
        left = (*chip).digital_playback_volume[idx as usize][0];
    } else {
        left = PCXHR_DIGITAL_LEVEL_MIN;
    }
    if (*chip).digital_playback_active[idx as usize][1] != 0 {
        right = (*chip).digital_playback_volume[idx as usize][1];
    } else {
        right = PCXHR_DIGITAL_LEVEL_MIN;
    }

    pcxhr_init_rmh(&mut rmh, CMD_STREAM_OUT_LEVEL_ADJUST);
    /* add pipe and stream mask */
    pcxhr_set_pipe_cmd_params(&mut rmh, 0, (*pipe).first_audio, 0, 1 << idx);
    /* volume left->left / right->right panoramic level */
    rmh.cmd[0] |= MORE_THAN_ONE_STREAM_LEVEL;
    rmh.cmd[2] = VALID_STREAM_PAN_LEVEL_MASK | VALID_STREAM_LEVEL_1_MASK;
    rmh.cmd[2] |= left << 10;
    rmh.cmd[3] = VALID_STREAM_PAN_LEVEL_MASK | VALID_STREAM_LEVEL_2_MASK;
    rmh.cmd[3] |= right;
    rmh.cmd_len = 4;

    err = pcxhr_send_msg((*chip).mgr, &mut rmh);
    if err < 0 {
        dev_dbg((*(*chip).card).dev, c_str!("error update_playback_stream_level card(%d) err(%x)\n"), (*chip).chip_idx, err);
        return -EINVAL;
    }
    0
}

const AUDIO_IO_HAS_MUTE_LEVEL: i32 = 0x400000;
const AUDIO_IO_HAS_MUTE_MONITOR_1: i32 = 0x200000;
const VALID_AUDIO_IO_DIGITAL_LEVEL: i32 = 0x000001;
const VALID_AUDIO_IO_MONITOR_LEVEL: i32 = 0x000002;
const VALID_AUDIO_IO_MUTE_LEVEL: i32 = 0x000004;
const VALID_AUDIO_IO_MUTE_MONITOR_1: i32 = 0x000008;

unsafe fn pcxhr_update_audio_pipe_level(chip: *mut snd_pcxhr, capture: i32, channel: i32) -> i32 {
    let mut err: i32;
    let mut rmh: pcxhr_rmh = core::mem::zeroed();
    let pipe: *mut pcxhr_pipe;

    if capture != 0 {
        pipe = &mut (*chip).capture_pipe[0];
    } else {
        pipe = &mut (*chip).playback_pipe;
    }

    pcxhr_init_rmh(&mut rmh, CMD_AUDIO_LEVEL_ADJUST);
    /* add channel mask */
    pcxhr_set_pipe_cmd_params(&mut rmh, capture, 0, 0, 1 << (channel + (*pipe).first_audio));
    /* TODO : if mask (3 << pipe->first_audio) is used, left and right
     * channel will be programmed to the same params */
    if capture != 0 {
        rmh.cmd[0] |= VALID_AUDIO_IO_DIGITAL_LEVEL;
        /* VALID_AUDIO_IO_MUTE_LEVEL not yet handled
         * (capture pipe level) */
        rmh.cmd[2] = (*chip).digital_capture_volume[channel as usize];
    } else {
        rmh.cmd[0] |= VALID_AUDIO_IO_MONITOR_LEVEL | VALID_AUDIO_IO_MUTE_MONITOR_1;
        /* VALID_AUDIO_IO_DIGITAL_LEVEL and VALID_AUDIO_IO_MUTE_LEVEL
         * not yet handled (playback pipe level)
         */
        rmh.cmd[2] = (*chip).monitoring_volume[channel as usize] << 10;
        if (*chip).monitoring_active[channel as usize] == 0 {
            rmh.cmd[2] |= AUDIO_IO_HAS_MUTE_MONITOR_1;
        }
    }
    rmh.cmd_len = 3;

    err = pcxhr_send_msg((*chip).mgr, &mut rmh);
    if err < 0 {
        dev_dbg((*(*chip).card).dev, c_str!("error update_audio_level(%d) err=%x\n"), (*chip).chip_idx, err);
        return -EINVAL;
    }
    0
}

/* shared */
unsafe fn pcxhr_digital_vol_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> i32 {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = PCXHR_DIGITAL_LEVEL_MIN; /* -109.5 dB */
    (*uinfo).value.integer.max = PCXHR_DIGITAL_LEVEL_MAX; /*   18.0 dB */
    0
}

unsafe fn pcxhr_pcm_vol_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let chip: *mut snd_pcxhr = snd_kcontrol_chip(kcontrol) as *mut snd_pcxhr;
    let idx: i32 = snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id); /* index */
    let stored_volume: *mut i32;
    let is_capture: i32 = (*kcontrol).private_value as i32;

    let _guard = mutex_guard(&mut (*(*chip).mgr).mixer_mutex);
    if is_capture != 0 {
        /* digital capture */
        stored_volume = (*chip).digital_capture_volume.as_mut_ptr();
    } else {
        /* digital playback */
        stored_volume = (*chip).digital_playback_volume[idx as usize].as_mut_ptr();
    }
    (*ucontrol).value.integer.value[0] = *stored_volume.add(0) as _;
    (*ucontrol).value.integer.value[1] = *stored_volume.add(1) as _;
    0
}

unsafe fn pcxhr_pcm_vol_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let chip: *mut snd_pcxhr = snd_kcontrol_chip(kcontrol) as *mut snd_pcxhr;
    let idx: i32 = snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id); /* index */
    let mut changed: i32 = 0;
    let is_capture: i32 = (*kcontrol).private_value as i32;
    let stored_volume: *mut i32;

    let _guard = mutex_guard(&mut (*(*chip).mgr).mixer_mutex);
    if is_capture != 0 {
        /* digital capture */
        stored_volume = (*chip).digital_capture_volume.as_mut_ptr();
    } else {
        /* digital playback */
        stored_volume = (*chip).digital_playback_volume[idx as usize].as_mut_ptr();
    }
    for i in 0..2 {
        let vol: i32 = (*ucontrol).value.integer.value[i] as i32;
        if vol < PCXHR_DIGITAL_LEVEL_MIN || vol > PCXHR_DIGITAL_LEVEL_MAX {
            continue;
        }
        if *stored_volume.add(i) != vol {
            *stored_volume.add(i) = vol;
            changed = 1;
            if is_capture != 0 {
                /* update capture volume */
                pcxhr_update_audio_pipe_level(chip, 1, i as i32);
            }
        }
    }
    if is_capture == 0 && changed != 0 {
        /* update playback volume */
        pcxhr_update_playback_stream_level(chip, idx);
    }
    changed
}

static snd_pcxhr_pcm_vol: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
    info: Some(pcxhr_digital_vol_info),
    get: Some(pcxhr_pcm_vol_get),
    put: Some(pcxhr_pcm_vol_put),
    tlv: snd_kcontrol_tlv { p: db_scale_digital.as_ptr() },
    ..unsafe { core::mem::zeroed() }
};

unsafe fn pcxhr_pcm_sw_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let chip: *mut snd_pcxhr = snd_kcontrol_chip(kcontrol) as *mut snd_pcxhr;
    let idx: i32 = snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id); /* index */

    let _guard = mutex_guard(&mut (*(*chip).mgr).mixer_mutex);
    (*ucontrol).value.integer.value[0] = (*chip).digital_playback_active[idx as usize][0] as _;
    (*ucontrol).value.integer.value[1] = (*chip).digital_playback_active[idx as usize][1] as _;
    0
}

unsafe fn pcxhr_pcm_sw_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let chip: *mut snd_pcxhr = snd_kcontrol_chip(kcontrol) as *mut snd_pcxhr;
    let mut changed: i32 = 0;
    let idx: i32 = snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id); /* index */
    let j: i32;

    let _guard = mutex_guard(&mut (*(*chip).mgr).mixer_mutex);
    j = idx;
    for i in 0..2 {
        if (*chip).digital_playback_active[j as usize][i] != (*ucontrol).value.integer.value[i] as i32 {
            (*chip).digital_playback_active[j as usize][i] = ((*ucontrol).value.integer.value[i] != 0) as i32;
            changed = 1;
        }
    }
    if changed != 0 {
        pcxhr_update_playback_stream_level(chip, idx);
    }
    changed
}

static pcxhr_control_pcm_switch: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c_str!("PCM Playback Switch"),
    count: PCXHR_PLAYBACK_STREAMS,
    info: Some(pcxhr_sw_info),
    get: Some(pcxhr_pcm_sw_get),
    put: Some(pcxhr_pcm_sw_put),
    ..unsafe { core::mem::zeroed() }
};

/*
 * monitoring level control
 */
unsafe fn pcxhr_monitor_vol_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let chip: *mut snd_pcxhr = snd_kcontrol_chip(kcontrol) as *mut snd_pcxhr;

    let _guard = mutex_guard(&mut (*(*chip).mgr).mixer_mutex);
    (*ucontrol).value.integer.value[0] = (*chip).monitoring_volume[0] as _;
    (*ucontrol).value.integer.value[1] = (*chip).monitoring_volume[1] as _;
    0
}

unsafe fn pcxhr_monitor_vol_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let chip: *mut snd_pcxhr = snd_kcontrol_chip(kcontrol) as *mut snd_pcxhr;
    let mut changed: i32 = 0;

    let _guard = mutex_guard(&mut (*(*chip).mgr).mixer_mutex);
    for i in 0..2 {
        if (*chip).monitoring_volume[i] != (*ucontrol).value.integer.value[i] as i32 {
            (*chip).monitoring_volume[i] = (*ucontrol).value.integer.value[i] as i32;
            if (*chip).monitoring_active[i] != 0 {
                /* update monitoring volume and mute */
                /* do only when monitoring is unmuted */
                pcxhr_update_audio_pipe_level(chip, 0, i as i32);
            }
            changed = 1;
        }
    }
    changed
}

static pcxhr_control_monitor_vol: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
    name: c_str!("Monitoring Playback Volume"),
    info: Some(pcxhr_digital_vol_info),
    get: Some(pcxhr_monitor_vol_get),
    put: Some(pcxhr_monitor_vol_put),
    tlv: snd_kcontrol_tlv { p: db_scale_digital.as_ptr() },
    ..unsafe { core::mem::zeroed() }
};

/*
 * monitoring switch control
 */
unsafe fn pcxhr_monitor_sw_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let chip: *mut snd_pcxhr = snd_kcontrol_chip(kcontrol) as *mut snd_pcxhr;

    let _guard = mutex_guard(&mut (*(*chip).mgr).mixer_mutex);
    (*ucontrol).value.integer.value[0] = (*chip).monitoring_active[0] as _;
    (*ucontrol).value.integer.value[1] = (*chip).monitoring_active[1] as _;
    0
}

unsafe fn pcxhr_monitor_sw_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let chip: *mut snd_pcxhr = snd_kcontrol_chip(kcontrol) as *mut snd_pcxhr;
    let mut changed: i32 = 0;

    let _guard = mutex_guard(&mut (*(*chip).mgr).mixer_mutex);
    for i in 0..2 {
        if (*chip).monitoring_active[i] != (*ucontrol).value.integer.value[i] as i32 {
            (*chip).monitoring_active[i] = ((*ucontrol).value.integer.value[i] != 0) as i32;
            changed |= 1 << i; /* mask 0x01 and 0x02 */
        }
    }
    if changed & 0x01 != 0 {
        /* update left monitoring volume and mute */
        pcxhr_update_audio_pipe_level(chip, 0, 0);
    }
    if changed & 0x02 != 0 {
        /* update right monitoring volume and mute */
        pcxhr_update_audio_pipe_level(chip, 0, 1);
    }

    (changed != 0) as i32
}

static pcxhr_control_monitor_sw: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c_str!("Monitoring Playback Switch"),
    info: Some(pcxhr_sw_info),
    get: Some(pcxhr_monitor_sw_get),
    put: Some(pcxhr_monitor_sw_put),
    ..unsafe { core::mem::zeroed() }
};

/*
 * audio source select
 */
const PCXHR_SOURCE_AUDIO01_UER: u32 = 0x000100;
const PCXHR_SOURCE_AUDIO01_SYNC: u32 = 0x000200;
const PCXHR_SOURCE_AUDIO23_UER: u32 = 0x000400;
const PCXHR_SOURCE_AUDIO45_UER: u32 = 0x001000;
const PCXHR_SOURCE_AUDIO67_UER: u32 = 0x040000;

unsafe fn pcxhr_set_audio_source(chip: *mut snd_pcxhr) -> i32 {
    let mut rmh: pcxhr_rmh = core::mem::zeroed();
    let mask: u32;
    let reg: u32;
    let codec: u32;
    let mut err: i32;
    let mut changed: i32 = 0;

    match (*chip).chip_idx {
        0 => {
            mask = PCXHR_SOURCE_AUDIO01_UER;
            codec = CS8420_01_CS;
        }
        1 => {
            mask = PCXHR_SOURCE_AUDIO23_UER;
            codec = CS8420_23_CS;
        }
        2 => {
            mask = PCXHR_SOURCE_AUDIO45_UER;
            codec = CS8420_45_CS;
        }
        3 => {
            mask = PCXHR_SOURCE_AUDIO67_UER;
            codec = CS8420_67_CS;
        }
        _ => return -EINVAL,
    }
    if (*chip).audio_capture_source != 0 {
        reg = mask; /* audio source from digital plug */
    } else {
        reg = 0; /* audio source from analog plug */
    }
    /* set the input source */
    pcxhr_write_io_num_reg_cont((*chip).mgr, mask, reg, &mut changed);
    /* resync them (otherwise channel inversion possible) */
    if changed != 0 {
        pcxhr_init_rmh(&mut rmh, CMD_RESYNC_AUDIO_INPUTS);
        rmh.cmd[0] |= 1 << (*chip).chip_idx;
        err = pcxhr_send_msg((*chip).mgr, &mut rmh);
        if err != 0 {
            return err;
        }
    }
    if (*(*chip).mgr).board_aes_in_192k != 0 {
        let mut src_config: u32 = 0xC0;
        /* update all src configs with one call */
        let mut i = 0;
        while i < 4 && i < (*(*chip).mgr).capture_chips {
            if (*(*(*chip).mgr).chip[i as usize]).audio_capture_source == 2 {
                src_config |= 1 << (3 - i);
            }
            i += 1;
        }
        /* set codec SRC on off */
        pcxhr_init_rmh(&mut rmh, CMD_ACCESS_IO_WRITE);
        rmh.cmd_len = 2;
        rmh.cmd[0] |= IO_NUM_REG_CONFIG_SRC;
        rmh.cmd[1] = src_config as i32;
        err = pcxhr_send_msg((*chip).mgr, &mut rmh);
    } else {
        let mut use_src: i32 = 0;
        if (*chip).audio_capture_source == 2 {
            use_src = 1;
        }
        /* set codec SRC on off */
        pcxhr_init_rmh(&mut rmh, CMD_ACCESS_IO_WRITE);
        rmh.cmd_len = 3;
        rmh.cmd[0] |= IO_NUM_UER_CHIP_REG;
        rmh.cmd[1] = codec as i32;
        rmh.cmd[2] = ((CS8420_DATA_FLOW_CTL & CHIP_SIG_AND_MAP_SPI) | if use_src != 0 { 0x41 } else { 0x54 }) as i32;
        err = pcxhr_send_msg((*chip).mgr, &mut rmh);
        if err != 0 {
            return err;
        }
        rmh.cmd[2] = ((CS8420_CLOCK_SRC_CTL & CHIP_SIG_AND_MAP_SPI) | if use_src != 0 { 0x41 } else { 0x49 }) as i32;
        err = pcxhr_send_msg((*chip).mgr, &mut rmh);
    }
    err
}

unsafe fn pcxhr_audio_src_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> i32 {
    static texts: [*const i8; 5] = [
        c_str!("Line"),
        c_str!("Digital"),
        c_str!("Digi+SRC"),
        c_str!("Mic"),
        c_str!("Line+Mic"),
    ];
    let mut i: i32;
    let chip: *mut snd_pcxhr = snd_kcontrol_chip(kcontrol) as *mut snd_pcxhr;

    i = 2; /* no SRC, no Mic available */
    if (*(*chip).mgr).board_has_aes1 != 0 {
        i = 3; /* SRC available */
        if (*(*chip).mgr).board_has_mic != 0 {
            i = 5; /* Mic and MicroMix available */
        }
    }
    snd_ctl_enum_info(uinfo, 1, i, texts.as_ptr())
}

unsafe fn pcxhr_audio_src_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let chip: *mut snd_pcxhr = snd_kcontrol_chip(kcontrol) as *mut snd_pcxhr;
    (*ucontrol).value.enumerated.item[0] = (*chip).audio_capture_source as _;
    0
}

unsafe fn pcxhr_audio_src_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let chip: *mut snd_pcxhr = snd_kcontrol_chip(kcontrol) as *mut snd_pcxhr;
    let mut ret: i32 = 0;
    let mut i: i32 = 2; /* no SRC, no Mic available */
    if (*(*chip).mgr).board_has_aes1 != 0 {
        i = 3; /* SRC available */
        if (*(*chip).mgr).board_has_mic != 0 {
            i = 5; /* Mic and MicroMix available */
        }
    }
    if (*ucontrol).value.enumerated.item[0] >= i as _ {
        return -EINVAL;
    }
    let _guard = mutex_guard(&mut (*(*chip).mgr).mixer_mutex);
    if (*chip).audio_capture_source != (*ucontrol).value.enumerated.item[0] as i32 {
        (*chip).audio_capture_source = (*ucontrol).value.enumerated.item[0] as i32;
        if (*(*chip).mgr).is_hr_stereo != 0 {
            hr222_set_audio_source(chip);
        } else {
            pcxhr_set_audio_source(chip);
        }
        ret = 1;
    }
    ret
}

static pcxhr_control_audio_src: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c_str!("Capture Source"),
    info: Some(pcxhr_audio_src_info),
    get: Some(pcxhr_audio_src_get),
    put: Some(pcxhr_audio_src_put),
    ..unsafe { core::mem::zeroed() }
};

/*
 * clock type selection
 * enum pcxhr_clock_type {
 *      PCXHR_CLOCK_TYPE_INTERNAL = 0,
 *      PCXHR_CLOCK_TYPE_WORD_CLOCK,
 *      PCXHR_CLOCK_TYPE_AES_SYNC,
 *      PCXHR_CLOCK_TYPE_AES_1,
 *      PCXHR_CLOCK_TYPE_AES_2,
 *      PCXHR_CLOCK_TYPE_AES_3,
 *      PCXHR_CLOCK_TYPE_AES_4,
 *      PCXHR_CLOCK_TYPE_MAX = PCXHR_CLOCK_TYPE_AES_4,
 *      HR22_CLOCK_TYPE_INTERNAL = PCXHR_CLOCK_TYPE_INTERNAL,
 *      HR22_CLOCK_TYPE_AES_SYNC,
 *      HR22_CLOCK_TYPE_AES_1,
 *      HR22_CLOCK_TYPE_MAX = HR22_CLOCK_TYPE_AES_1,
 * };
 */

unsafe fn pcxhr_clock_type_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> i32 {
    static textsPCXHR: [*const i8; 7] = [
        c_str!("Internal"),
        c_str!("WordClock"),
        c_str!("AES Sync"),
        c_str!("AES 1"),
        c_str!("AES 2"),
        c_str!("AES 3"),
        c_str!("AES 4"),
    ];
    static textsHR22: [*const i8; 3] = [c_str!("Internal"), c_str!("AES Sync"), c_str!("AES 1")];
    let texts: *const *const i8;
    let mgr: *mut pcxhr_mgr = snd_kcontrol_chip(kcontrol) as *mut pcxhr_mgr;
    let mut clock_items: i32 = 2; /* at least Internal and AES Sync clock */
    if (*mgr).board_has_aes1 != 0 {
        clock_items += (*mgr).capture_chips; /* add AES x */
        if (*mgr).is_hr_stereo == 0 {
            clock_items += 1; /* add word clock */
        }
    }
    if (*mgr).is_hr_stereo != 0 {
        texts = textsHR22.as_ptr();
        snd_BUG_ON((clock_items > (HR22_CLOCK_TYPE_MAX + 1)) as i32);
    } else {
        texts = textsPCXHR.as_ptr();
        snd_BUG_ON((clock_items > (PCXHR_CLOCK_TYPE_MAX + 1)) as i32);
    }
    snd_ctl_enum_info(uinfo, 1, clock_items, texts)
}

unsafe fn pcxhr_clock_type_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let mgr: *mut pcxhr_mgr = snd_kcontrol_chip(kcontrol) as *mut pcxhr_mgr;
    (*ucontrol).value.enumerated.item[0] = (*mgr).use_clock_type as _;
    0
}

unsafe fn pcxhr_clock_type_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let mgr: *mut pcxhr_mgr = snd_kcontrol_chip(kcontrol) as *mut pcxhr_mgr;
    let mut rate: i32;
    let mut ret: i32 = 0;
    let mut clock_items: u32 = 2; /* at least Internal and AES Sync clock */
    if (*mgr).board_has_aes1 != 0 {
        clock_items += (*mgr).capture_chips as u32; /* add AES x */
        if (*mgr).is_hr_stereo == 0 {
            clock_items += 1; /* add word clock */
        }
    }
    if (*ucontrol).value.enumerated.item[0] >= clock_items as _ {
        return -EINVAL;
    }
    let _guard = mutex_guard(&mut (*mgr).mixer_mutex);
    if (*mgr).use_clock_type != (*ucontrol).value.enumerated.item[0] as i32 {
        let _setup_guard = mutex_guard(&mut (*mgr).setup_mutex);
        (*mgr).use_clock_type = (*ucontrol).value.enumerated.item[0] as i32;
        rate = 0;
        if (*mgr).use_clock_type != PCXHR_CLOCK_TYPE_INTERNAL {
            pcxhr_get_external_clock(mgr, (*mgr).use_clock_type, &mut rate);
        } else {
            rate = (*mgr).sample_rate;
            if rate == 0 {
                rate = 48000;
            }
        }
        if rate != 0 {
            pcxhr_set_clock(mgr, rate);
            if (*mgr).sample_rate != 0 {
                (*mgr).sample_rate = rate;
            }
        }
        ret = 1; /* return 1 even if the set was not done. ok ? */
    }
    ret
}

static pcxhr_control_clock_type: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c_str!("Clock Mode"),
    info: Some(pcxhr_clock_type_info),
    get: Some(pcxhr_clock_type_get),
    put: Some(pcxhr_clock_type_put),
    ..unsafe { core::mem::zeroed() }
};

/*
 * clock rate control
 * specific control that scans the sample rates on the external plugs
 */
unsafe fn pcxhr_clock_rate_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> i32 {
    let mgr: *mut pcxhr_mgr = snd_kcontrol_chip(kcontrol) as *mut pcxhr_mgr;
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 3 + (*mgr).capture_chips;
    (*uinfo).value.integer.min = 0; /* clock not present */
    (*uinfo).value.integer.max = 192000; /* max sample rate 192 kHz */
    0
}

unsafe fn pcxhr_clock_rate_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let mgr: *mut pcxhr_mgr = snd_kcontrol_chip(kcontrol) as *mut pcxhr_mgr;
    let mut err: i32;
    let mut rate: i32 = 0;

    let _guard = mutex_guard(&mut (*mgr).mixer_mutex);
    let mut i = 0;
    while i < 3 + (*mgr).capture_chips {
        if i == PCXHR_CLOCK_TYPE_INTERNAL {
            rate = (*mgr).sample_rate_real;
        } else {
            err = pcxhr_get_external_clock(mgr, i, &mut rate);
            if err != 0 {
                break;
            }
        }
        (*ucontrol).value.integer.value[i as usize] = rate as _;
        i += 1;
    }
    0
}

static pcxhr_control_clock_rate: snd_kcontrol_new = snd_kcontrol_new {
    access: SNDRV_CTL_ELEM_ACCESS_READ,
    iface: SNDRV_CTL_ELEM_IFACE_CARD,
    name: c_str!("Clock Rates"),
    info: Some(pcxhr_clock_rate_info),
    get: Some(pcxhr_clock_rate_get),
    ..unsafe { core::mem::zeroed() }
};

/*
 * IEC958 status bits
 */
unsafe fn pcxhr_iec958_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> i32 {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
    (*uinfo).count = 1;
    0
}

unsafe fn pcxhr_iec958_capture_byte(chip: *mut snd_pcxhr, aes_idx: i32, aes_bits: *mut u8) -> i32 {
    let mut err: i32;
    let mut temp: u8;
    let mut rmh: pcxhr_rmh = core::mem::zeroed();

    pcxhr_init_rmh(&mut rmh, CMD_ACCESS_IO_READ);
    rmh.cmd[0] |= IO_NUM_UER_CHIP_REG;
    match (*chip).chip_idx {
        /* instead of CS8420_01_CS use CS8416_01_CS for AES SYNC plug */
        0 => rmh.cmd[1] = CS8420_01_CS as i32,
        1 => rmh.cmd[1] = CS8420_23_CS as i32,
        2 => rmh.cmd[1] = CS8420_45_CS as i32,
        3 => rmh.cmd[1] = CS8420_67_CS as i32,
        _ => return -EINVAL,
    }
    if (*(*chip).mgr).board_aes_in_192k != 0 {
        match aes_idx {
            0 => rmh.cmd[2] = CS8416_CSB0 as i32,
            1 => rmh.cmd[2] = CS8416_CSB1 as i32,
            2 => rmh.cmd[2] = CS8416_CSB2 as i32,
            3 => rmh.cmd[2] = CS8416_CSB3 as i32,
            4 => rmh.cmd[2] = CS8416_CSB4 as i32,
            _ => return -EINVAL,
        }
    } else {
        match aes_idx {
            /* instead of CS8420_CSB0 use CS8416_CSBx for AES SYNC plug */
            0 => rmh.cmd[2] = CS8420_CSB0 as i32,
            1 => rmh.cmd[2] = CS8420_CSB1 as i32,
            2 => rmh.cmd[2] = CS8420_CSB2 as i32,
            3 => rmh.cmd[2] = CS8420_CSB3 as i32,
            4 => rmh.cmd[2] = CS8420_CSB4 as i32,
            _ => return -EINVAL,
        }
    }
    /* size and code the chip id for the fpga */
    rmh.cmd[1] &= 0x0fffff;
    /* chip signature + map for spi read */
    rmh.cmd[2] &= CHIP_SIG_AND_MAP_SPI as i32;
    rmh.cmd_len = 3;
    err = pcxhr_send_msg((*chip).mgr, &mut rmh);
    if err != 0 {
        return err;
    }

    if (*(*chip).mgr).board_aes_in_192k != 0 {
        temp = rmh.stat[1] as u8;
    } else {
        temp = 0;
        /* reversed bit order (not with CS8416_01_CS) */
        for i in 0..8 {
            temp <<= 1;
            if rmh.stat[1] & (1 << i) != 0 {
                temp |= 1;
            }
        }
    }
    dev_dbg((*(*chip).card).dev, c_str!("read iec958 AES %d byte %d = 0x%x\n"), (*chip).chip_idx, aes_idx, temp as i32);
    *aes_bits = temp;
    0
}

unsafe fn pcxhr_iec958_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let chip: *mut snd_pcxhr = snd_kcontrol_chip(kcontrol) as *mut snd_pcxhr;
    let mut aes_bits: u8 = 0;
    let mut err: i32;

    let _guard = mutex_guard(&mut (*(*chip).mgr).mixer_mutex);
    for i in 0..5 {
        if (*kcontrol).private_value == 0 {
            /* playback */
            aes_bits = (*chip).aes_bits[i];
        } else {
            /* capture */
            if (*(*chip).mgr).is_hr_stereo != 0 {
                err = hr222_iec958_capture_byte(chip, i as i32, &mut aes_bits);
            } else {
                err = pcxhr_iec958_capture_byte(chip, i as i32, &mut aes_bits);
            }
            if err != 0 {
                break;
            }
        }
        (*ucontrol).value.iec958.status[i] = aes_bits;
    }
    0
}

unsafe fn pcxhr_iec958_mask_get(_kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    for i in 0..5 {
        (*ucontrol).value.iec958.status[i] = 0xff;
    }
    0
}

unsafe fn pcxhr_iec958_update_byte(chip: *mut snd_pcxhr, aes_idx: i32, aes_bits: u8) -> i32 {
    let mut err: i32;
    let mut cmd: i32;
    let mut new_bits: u8 = aes_bits;
    let mut old_bits: u8 = (*chip).aes_bits[aes_idx as usize];
    let mut rmh: pcxhr_rmh = core::mem::zeroed();

    for i in 0..8 {
        if (old_bits & 0x01) != (new_bits & 0x01) {
            cmd = (*chip).chip_idx & 0x03; /* chip index 0..3 */
            if (*chip).chip_idx > 3 {
                /* new bit used if chip_idx>3 (PCX1222HR) */
                cmd |= 1 << 22;
            }
            cmd |= ((aes_idx << 3) + i) << 2; /* add bit offset */
            cmd |= ((new_bits & 0x01) as i32) << 23; /* add bit value */
            pcxhr_init_rmh(&mut rmh, CMD_ACCESS_IO_WRITE);
            rmh.cmd[0] |= IO_NUM_REG_CUER;
            rmh.cmd[1] = cmd;
            rmh.cmd_len = 2;
            dev_dbg((*(*chip).card).dev, c_str!("write iec958 AES %d byte %d bit %d (cmd %x)\n"), (*chip).chip_idx, aes_idx, i, cmd);
            err = pcxhr_send_msg((*chip).mgr, &mut rmh);
            if err != 0 {
                return err;
            }
        }
        old_bits >>= 1;
        new_bits >>= 1;
    }
    (*chip).aes_bits[aes_idx as usize] = aes_bits;
    0
}

unsafe fn pcxhr_iec958_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let chip: *mut snd_pcxhr = snd_kcontrol_chip(kcontrol) as *mut snd_pcxhr;
    let mut changed: i32 = 0;

    /* playback */
    let _guard = mutex_guard(&mut (*(*chip).mgr).mixer_mutex);
    for i in 0..5 {
        if (*ucontrol).value.iec958.status[i] != (*chip).aes_bits[i] {
            if (*(*chip).mgr).is_hr_stereo != 0 {
                hr222_iec958_update_byte(chip, i as i32, (*ucontrol).value.iec958.status[i]);
            } else {
                pcxhr_iec958_update_byte(chip, i as i32, (*ucontrol).value.iec958.status[i]);
            }
            changed = 1;
        }
    }
    changed
}

static pcxhr_control_playback_iec958_mask: snd_kcontrol_new = snd_kcontrol_new {
    access: SNDRV_CTL_ELEM_ACCESS_READ,
    iface: SNDRV_CTL_ELEM_IFACE_PCM,
    name: SNDRV_CTL_NAME_IEC958_PLAYBACK_MASK,
    info: Some(pcxhr_iec958_info),
    get: Some(pcxhr_iec958_mask_get),
    ..unsafe { core::mem::zeroed() }
};

static pcxhr_control_playback_iec958: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_PCM,
    name: SNDRV_CTL_NAME_IEC958_PLAYBACK_DEFAULT,
    info: Some(pcxhr_iec958_info),
    get: Some(pcxhr_iec958_get),
    put: Some(pcxhr_iec958_put),
    private_value: 0, /* playback */
    ..unsafe { core::mem::zeroed() }
};

static pcxhr_control_capture_iec958_mask: snd_kcontrol_new = snd_kcontrol_new {
    access: SNDRV_CTL_ELEM_ACCESS_READ,
    iface: SNDRV_CTL_ELEM_IFACE_PCM,
    name: SNDRV_CTL_NAME_IEC958_CAPTURE_MASK,
    info: Some(pcxhr_iec958_info),
    get: Some(pcxhr_iec958_mask_get),
    ..unsafe { core::mem::zeroed() }
};

static pcxhr_control_capture_iec958: snd_kcontrol_new = snd_kcontrol_new {
    access: SNDRV_CTL_ELEM_ACCESS_READ,
    iface: SNDRV_CTL_ELEM_IFACE_PCM,
    name: SNDRV_CTL_NAME_IEC958_CAPTURE_DEFAULT,
    info: Some(pcxhr_iec958_info),
    get: Some(pcxhr_iec958_get),
    private_value: 1, /* capture */
    ..unsafe { core::mem::zeroed() }
};

unsafe fn pcxhr_init_audio_levels(chip: *mut snd_pcxhr) {
    for i in 0..2 {
        if (*chip).nb_streams_play != 0 {
            /* at boot time the digital volumes are unmuted 0dB */
            for j in 0..PCXHR_PLAYBACK_STREAMS {
                (*chip).digital_playback_active[j as usize][i] = 1;
                (*chip).digital_playback_volume[j as usize][i] = PCXHR_DIGITAL_ZERO_LEVEL;
            }
            /* after boot, only two bits are set on the uer
             * interface
             */
            (*chip).aes_bits[0] = (IEC958_AES0_PROFESSIONAL | IEC958_AES0_PRO_FS_48000) as u8;
            /* CONFIG_SND_DEBUG:
             * analog volumes for playback (is LEVEL_MIN after boot)
             *
             * chip->analog_playback_active[i] = 1;
             * if (chip->mgr->is_hr_stereo)
             *     chip->analog_playback_volume[i] = HR222_LINE_PLAYBACK_ZERO_LEVEL;
             * else {
             *     chip->analog_playback_volume[i] = PCXHR_LINE_PLAYBACK_ZERO_LEVEL;
             *     pcxhr_update_analog_audio_level(chip, 0, i);
             * }
             */
            /* stereo cards need to be initialised after boot */
            if (*(*chip).mgr).is_hr_stereo != 0 {
                hr222_update_analog_audio_level(chip, 0, i as i32);
            }
        }
        if (*chip).nb_streams_capt != 0 {
            /* at boot time the digital volumes are unmuted 0dB */
            (*chip).digital_capture_volume[i] = PCXHR_DIGITAL_ZERO_LEVEL;
            (*chip).analog_capture_active = 1;
            /* CONFIG_SND_DEBUG:
             * analog volumes for playback (is LEVEL_MIN after boot)
             *
             * if (chip->mgr->is_hr_stereo)
             *     chip->analog_capture_volume[i] = HR222_LINE_CAPTURE_ZERO_LEVEL;
             * else {
             *     chip->analog_capture_volume[i] = PCXHR_LINE_CAPTURE_ZERO_LEVEL;
             *     pcxhr_update_analog_audio_level(chip, 1, i);
             * }
             */
            /* stereo cards need to be initialised after boot */
            if (*(*chip).mgr).is_hr_stereo != 0 {
                hr222_update_analog_audio_level(chip, 1, i as i32);
            }
        }
    }
}

pub unsafe fn pcxhr_create_mixer(mgr: *mut pcxhr_mgr) -> i32 {
    let mut chip: *mut snd_pcxhr;
    let mut err: i32;

    mutex_init(&mut (*mgr).mixer_mutex); /* can be in another place */

    for i in 0..(*mgr).num_cards {
        let mut temp: snd_kcontrol_new;
        chip = (*mgr).chip[i as usize];

        if (*chip).nb_streams_play != 0 {
            /* analog output level control */
            temp = pcxhr_control_analog_level;
            temp.name = c_str!("Master Playback Volume");
            temp.private_value = 0; /* playback */
            if (*mgr).is_hr_stereo != 0 {
                temp.tlv.p = db_scale_a_hr222_playback.as_ptr();
            } else {
                temp.tlv.p = db_scale_analog_playback.as_ptr();
            }
            err = snd_ctl_add((*chip).card, snd_ctl_new1(&mut temp, chip as *mut core::ffi::c_void));
            if err < 0 {
                return err;
            }

            /* output mute controls */
            err = snd_ctl_add((*chip).card, snd_ctl_new1(&pcxhr_control_output_switch, chip as *mut core::ffi::c_void));
            if err < 0 {
                return err;
            }

            temp = snd_pcxhr_pcm_vol;
            temp.name = c_str!("PCM Playback Volume");
            temp.count = PCXHR_PLAYBACK_STREAMS;
            temp.private_value = 0; /* playback */
            err = snd_ctl_add((*chip).card, snd_ctl_new1(&mut temp, chip as *mut core::ffi::c_void));
            if err < 0 {
                return err;
            }

            err = snd_ctl_add((*chip).card, snd_ctl_new1(&pcxhr_control_pcm_switch, chip as *mut core::ffi::c_void));
            if err < 0 {
                return err;
            }

            /* IEC958 controls */
            err = snd_ctl_add((*chip).card, snd_ctl_new1(&pcxhr_control_playback_iec958_mask, chip as *mut core::ffi::c_void));
            if err < 0 {
                return err;
            }

            err = snd_ctl_add((*chip).card, snd_ctl_new1(&pcxhr_control_playback_iec958, chip as *mut core::ffi::c_void));
            if err < 0 {
                return err;
            }
        }
        if (*chip).nb_streams_capt != 0 {
            /* analog input level control */
            temp = pcxhr_control_analog_level;
            temp.name = c_str!("Line Capture Volume");
            temp.private_value = 1; /* capture */
            if (*mgr).is_hr_stereo != 0 {
                temp.tlv.p = db_scale_a_hr222_capture.as_ptr();
            } else {
                temp.tlv.p = db_scale_analog_capture.as_ptr();
            }

            err = snd_ctl_add((*chip).card, snd_ctl_new1(&mut temp, chip as *mut core::ffi::c_void));
            if err < 0 {
                return err;
            }

            temp = snd_pcxhr_pcm_vol;
            temp.name = c_str!("PCM Capture Volume");
            temp.count = 1;
            temp.private_value = 1; /* capture */

            err = snd_ctl_add((*chip).card, snd_ctl_new1(&mut temp, chip as *mut core::ffi::c_void));
            if err < 0 {
                return err;
            }

            /* Audio source */
            err = snd_ctl_add((*chip).card, snd_ctl_new1(&pcxhr_control_audio_src, chip as *mut core::ffi::c_void));
            if err < 0 {
                return err;
            }

            /* IEC958 controls */
            err = snd_ctl_add((*chip).card, snd_ctl_new1(&pcxhr_control_capture_iec958_mask, chip as *mut core::ffi::c_void));
            if err < 0 {
                return err;
            }

            err = snd_ctl_add((*chip).card, snd_ctl_new1(&pcxhr_control_capture_iec958, chip as *mut core::ffi::c_void));
            if err < 0 {
                return err;
            }

            if (*mgr).is_hr_stereo != 0 {
                err = hr222_add_mic_controls(chip);
                if err < 0 {
                    return err;
                }
            }
        }
        /* monitoring only if playback and capture device available */
        if (*chip).nb_streams_capt > 0 && (*chip).nb_streams_play > 0 {
            /* monitoring */
            err = snd_ctl_add((*chip).card, snd_ctl_new1(&pcxhr_control_monitor_vol, chip as *mut core::ffi::c_void));
            if err < 0 {
                return err;
            }

            err = snd_ctl_add((*chip).card, snd_ctl_new1(&pcxhr_control_monitor_sw, chip as *mut core::ffi::c_void));
            if err < 0 {
                return err;
            }
        }

        if i == 0 {
            /* clock mode only one control per pcxhr */
            err = snd_ctl_add((*chip).card, snd_ctl_new1(&pcxhr_control_clock_type, mgr as *mut core::ffi::c_void));
            if err < 0 {
                return err;
            }
            /* non standard control used to scan
             * the external clock presence/frequencies
             */
            err = snd_ctl_add((*chip).card, snd_ctl_new1(&pcxhr_control_clock_rate, mgr as *mut core::ffi::c_void));
            if err < 0 {
                return err;
            }
        }

        /* init values for the mixer data */
        pcxhr_init_audio_levels(chip);
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
