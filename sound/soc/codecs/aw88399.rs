// SPDX-License-Identifier: GPL-2.0-only
//
// aw88399.c --  ALSA SoC AW88399 codec support
//
// Copyright (c) 2023 AWINIC Technology CO., LTD
//
// Author: Weidong Wang <wangweidong.a@awinic.com>
//

// Translated from the implementation source. Linux, ALSA, regmap, GPIO, I2C,
// and AW88399/AW88395 declarations are external dependencies supplied by the
// surrounding driver tree.

static mut aw88399_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c_str!("aw88399-aif"),
    id: 1,
    playback: snd_soc_pcm_stream {
        stream_name: c_str!("Speaker_Playback"),
        channels_min: 1,
        channels_max: 2,
        rates: AW88399_RATES,
        formats: AW88399_FORMATS,
        ..unsafe { core::mem::zeroed() }
    },
    capture: snd_soc_pcm_stream {
        stream_name: c_str!("Speaker_Capture"),
        channels_min: 1,
        channels_max: 2,
        rates: AW88399_RATES,
        formats: AW88399_FORMATS,
        ..unsafe { core::mem::zeroed() }
    },
    ..unsafe { core::mem::zeroed() }
}];

unsafe fn aw_cali_svc_run_mute(aw_dev: *mut aw_device, cali_result: u16) {
    if cali_result == CALI_RESULT_ERROR {
        aw88399_dev_mute(aw_dev, true);
    } else if cali_result == CALI_RESULT_NORMAL {
        aw88399_dev_mute(aw_dev, false);
    }
}

unsafe fn aw_cali_svc_get_cali_cfg(aw_dev: *mut aw_device) -> i32 {
    let cali_cfg: *mut cali_cfg = &mut (*aw_dev).cali_desc.cali_cfg;
    let mut ret: i32;

    ret = aw_dev_dsp_read(
        aw_dev,
        AW88399_DSP_REG_CFG_MBMEC_ACTAMPTH,
        &mut (*cali_cfg).data[0],
        AW_DSP_32_DATA,
    );
    if ret != 0 {
        return ret;
    }

    ret = aw_dev_dsp_read(
        aw_dev,
        AW88399_DSP_REG_CFG_MBMEC_NOISEAMPTH,
        &mut (*cali_cfg).data[1],
        AW_DSP_32_DATA,
    );
    if ret != 0 {
        return ret;
    }

    ret = aw_dev_dsp_read(
        aw_dev,
        AW88399_DSP_REG_CFG_ADPZ_USTEPN,
        &mut (*cali_cfg).data[2],
        AW_DSP_16_DATA,
    );
    if ret != 0 {
        return ret;
    }

    ret = aw_dev_dsp_read(
        aw_dev,
        AW88399_DSP_REG_CFG_RE_ALPHA,
        &mut (*cali_cfg).data[3],
        AW_DSP_16_DATA,
    );

    ret
}

unsafe fn aw_cali_svc_set_cali_cfg(aw_dev: *mut aw_device, cali_cfg: cali_cfg) -> i32 {
    let mut ret: i32;

    ret = aw_dev_dsp_write(
        aw_dev,
        AW88399_DSP_REG_CFG_MBMEC_ACTAMPTH,
        cali_cfg.data[0],
        AW_DSP_32_DATA,
    );
    if ret != 0 {
        return ret;
    }

    ret = aw_dev_dsp_write(
        aw_dev,
        AW88399_DSP_REG_CFG_MBMEC_NOISEAMPTH,
        cali_cfg.data[1],
        AW_DSP_32_DATA,
    );
    if ret != 0 {
        return ret;
    }

    ret = aw_dev_dsp_write(
        aw_dev,
        AW88399_DSP_REG_CFG_ADPZ_USTEPN,
        cali_cfg.data[2],
        AW_DSP_16_DATA,
    );
    if ret != 0 {
        return ret;
    }

    ret = aw_dev_dsp_write(
        aw_dev,
        AW88399_DSP_REG_CFG_RE_ALPHA,
        cali_cfg.data[3],
        AW_DSP_16_DATA,
    );

    ret
}

unsafe fn aw_cali_svc_cali_en(aw_dev: *mut aw_device, cali_en: bool) -> i32 {
    let mut set_cfg: cali_cfg = core::mem::zeroed();
    let mut ret: i32;

    aw_dev_dsp_enable(aw_dev, false);
    if cali_en {
        regmap_update_bits(
            (*aw_dev).regmap,
            AW88399_DBGCTRL_REG,
            !AW883XX_DSP_NG_EN_MASK,
            AW883XX_DSP_NG_EN_DISABLE_VALUE,
        );
        aw_dev_dsp_write(
            aw_dev,
            AW88399_DSP_LOW_POWER_SWITCH_CFG_ADDR,
            AW88399_DSP_LOW_POWER_SWITCH_DISABLE,
            AW_DSP_16_DATA,
        );

        ret = aw_cali_svc_get_cali_cfg(aw_dev);
        if ret != 0 {
            dev_err((*aw_dev).dev, c_str!("get cali cfg failed\n"));
            aw_dev_dsp_enable(aw_dev, true);
            return ret;
        }
        set_cfg.data[0] = 0;
        set_cfg.data[1] = 0;
        set_cfg.data[2] = -1i32 as _;
        set_cfg.data[3] = 1;

        ret = aw_cali_svc_set_cali_cfg(aw_dev, set_cfg);
        if ret != 0 {
            dev_err((*aw_dev).dev, c_str!("set cali cfg failed\n"));
            aw_cali_svc_set_cali_cfg(aw_dev, (*aw_dev).cali_desc.cali_cfg);
            aw_dev_dsp_enable(aw_dev, true);
            return ret;
        }
    } else {
        aw_cali_svc_set_cali_cfg(aw_dev, (*aw_dev).cali_desc.cali_cfg);
    }

    aw_dev_dsp_enable(aw_dev, true);
    0
}

unsafe fn aw_cali_svc_cali_run_dsp_vol(aw_dev: *mut aw_device, enable: bool) -> i32 {
    let mut reg_val: u32 = 0;
    let ret: i32;

    if enable {
        ret = regmap_read((*aw_dev).regmap, AW88399_DSPCFG_REG, &mut reg_val);
        if ret != 0 {
            dev_err(
                (*aw_dev).dev,
                c_str!("read reg 0x%x failed\n"),
                AW88399_DSPCFG_REG,
            );
            return ret;
        }

        (*aw_dev).cali_desc.store_vol = reg_val & !AW88399_DSP_VOL_MASK;
        regmap_update_bits(
            (*aw_dev).regmap,
            AW88399_DSPCFG_REG,
            !AW88399_DSP_VOL_MASK,
            AW88399_DSP_VOL_MUTE,
        )
    } else {
        regmap_update_bits(
            (*aw_dev).regmap,
            AW88399_DSPCFG_REG,
            !AW88399_DSP_VOL_MASK,
            (*aw_dev).cali_desc.store_vol,
        )
    }
}

unsafe fn aw_cali_svc_backup_info(aw_dev: *mut aw_device) {
    let backup_desc: *mut aw_cali_backup_desc = &mut (*aw_dev).cali_desc.backup_info;
    let mut reg_val: u32 = 0;
    let mut dsp_val: u32 = 0;

    regmap_read((*aw_dev).regmap, AW88399_DBGCTRL_REG, &mut reg_val);
    (*backup_desc).dsp_ng_cfg = reg_val & !AW883XX_DSP_NG_EN_MASK;

    aw_dev_dsp_read(
        aw_dev,
        AW88399_DSP_LOW_POWER_SWITCH_CFG_ADDR,
        &mut dsp_val,
        AW_DSP_16_DATA,
    );

    (*backup_desc).dsp_lp_cfg = dsp_val;
}

unsafe fn aw_cali_svc_recover_info(aw_dev: *mut aw_device) {
    let backup_desc: *mut aw_cali_backup_desc = &mut (*aw_dev).cali_desc.backup_info;

    regmap_update_bits(
        (*aw_dev).regmap,
        AW88399_DBGCTRL_REG,
        !AW883XX_DSP_NG_EN_MASK,
        (*backup_desc).dsp_ng_cfg,
    );

    aw_dev_dsp_write(
        aw_dev,
        AW88399_DSP_LOW_POWER_SWITCH_CFG_ADDR,
        (*backup_desc).dsp_lp_cfg,
        AW_DSP_16_DATA,
    );
}

unsafe fn aw_cali_svc_cali_re_mode_enable(aw_dev: *mut aw_device, is_enable: bool) -> i32 {
    let mut ret: i32;

    if is_enable {
        ret = aw_dev_check_syspll(aw_dev);
        if ret != 0 {
            dev_err((*aw_dev).dev, c_str!("pll check failed cannot start\n"));
            return ret;
        }

        ret = aw_dev_get_dsp_status(aw_dev);
        if ret != 0 {
            dev_err((*aw_dev).dev, c_str!("dsp status error\n"));
            return ret;
        }

        aw_cali_svc_backup_info(aw_dev);
        ret = aw_cali_svc_cali_en(aw_dev, true);
        if ret != 0 {
            dev_err((*aw_dev).dev, c_str!("aw_cali_svc_cali_en failed\n"));
            return ret;
        }

        ret = aw_cali_svc_cali_run_dsp_vol(aw_dev, true);
        if ret != 0 {
            aw_cali_svc_cali_en(aw_dev, false);
            return ret;
        }
    } else {
        aw_cali_svc_cali_run_dsp_vol(aw_dev, false);
        aw_cali_svc_recover_info(aw_dev);
        aw_cali_svc_cali_en(aw_dev, false);
    }

    0
}

unsafe fn aw_cali_svc_get_dev_re(aw_dev: *mut aw_device, re: *mut u32) -> i32 {
    let mut dsp_re: u32 = 0;
    let show_re: u32;
    let ret: i32;

    ret = aw_dev_dsp_read(aw_dev, AW88399_DSP_REG_CALRE, &mut dsp_re, AW_DSP_16_DATA);
    if ret != 0 {
        return ret;
    }

    show_re = AW88399_DSP_RE_TO_SHOW_RE(dsp_re, AW88399_DSP_REG_CALRE_SHIFT);
    *re = (show_re.wrapping_sub((*aw_dev).cali_desc.ra)) as u32;

    0
}

unsafe fn aw_cali_svc_del_max_min_ave_algo(data: *mut u32, data_size: i32, dsp_re: *mut u32) {
    let mut sum: i32 = 0;
    let mut i: i32 = 1;

    while i < data_size - 1 {
        sum = sum.wrapping_add(*data.offset(i as isize) as i32);
        i += 1;
    }

    *dsp_re = (sum / (data_size - AW_CALI_DATA_SUM_RM)) as u32;
}

unsafe fn aw_cali_svc_get_iv_st(aw_dev: *mut aw_device) -> i32 {
    let mut reg_data: u32 = 0;
    let mut ret: i32;
    let mut i: i32 = 0;

    while i < AW_GET_IV_CNT_MAX {
        ret = regmap_read((*aw_dev).regmap, AW88399_ASR1_REG, &mut reg_data);
        if ret != 0 {
            dev_err((*aw_dev).dev, c_str!("read 0x%x failed\n"), AW88399_ASR1_REG);
            return ret;
        }

        reg_data &= !AW88399_REABS_MASK;
        if reg_data == 0 {
            return 0;
        }
        msleep(30);
        i += 1;
    }

    dev_err((*aw_dev).dev, c_str!("IV data abnormal, please check\n"));
    -EINVAL
}

unsafe extern "C" fn compare_ints(a: *const core::ffi::c_void, b: *const core::ffi::c_void) -> i32 {
    (*(a as *const i32)).wrapping_sub(*(b as *const i32))
}

unsafe fn aw_cali_svc_get_smooth_cali_re(aw_dev: *mut aw_device) -> i32 {
    let mut re_temp: [u32; AW_CALI_READ_CNT_MAX as usize] = [0; AW_CALI_READ_CNT_MAX as usize];
    let mut dsp_re: u32 = 0;
    let mut ret: i32;
    let mut i: i32 = 0;

    while i < AW_CALI_READ_CNT_MAX {
        ret = aw_cali_svc_get_dev_re(aw_dev, &mut re_temp[i as usize]);
        if ret != 0 {
            (*aw_dev).cali_desc.cali_result = CALI_RESULT_ERROR;
            aw_cali_svc_run_mute(aw_dev, (*aw_dev).cali_desc.cali_result);
            return -EINVAL;
        }
        msleep(30);
        i += 1;
    }

    sort(
        re_temp.as_mut_ptr() as *mut core::ffi::c_void,
        AW_CALI_READ_CNT_MAX,
        core::mem::size_of::<u32>(),
        Some(compare_ints),
        None,
    );

    aw_cali_svc_del_max_min_ave_algo(re_temp.as_mut_ptr(), AW_CALI_READ_CNT_MAX, &mut dsp_re);

    ret = aw_cali_svc_get_iv_st(aw_dev);
    if ret != 0 {
        dev_err((*aw_dev).dev, c_str!("get iv data failed"));
        (*aw_dev).cali_desc.cali_result = CALI_RESULT_ERROR;
        aw_cali_svc_run_mute(aw_dev, (*aw_dev).cali_desc.cali_result);
        return -EINVAL;
    }

    if dsp_re < AW88399_CALI_RE_MIN || dsp_re > AW88399_CALI_RE_MAX {
        dev_err((*aw_dev).dev, c_str!("out range re value: [%d]mohm\n"), dsp_re);
        (*aw_dev).cali_desc.cali_re = dsp_re;
        (*aw_dev).cali_desc.cali_result = CALI_RESULT_ERROR;
        aw_cali_svc_run_mute(aw_dev, (*aw_dev).cali_desc.cali_result);
        return 0;
    }

    (*aw_dev).cali_desc.cali_result = CALI_RESULT_NORMAL;
    (*aw_dev).cali_desc.cali_re = dsp_re;
    dev_dbg((*aw_dev).dev, c_str!("re[%d]mohm\n"), (*aw_dev).cali_desc.cali_re);

    aw_dev_dsp_enable(aw_dev, false);
    aw_dev_update_cali_re(&mut (*aw_dev).cali_desc);
    aw_dev_dsp_enable(aw_dev, true);

    0
}

unsafe fn aw_cali_svc_dev_cali_re(aw88399: *mut aw88399) -> i32 {
    let aw_dev: *mut aw_device = (*aw88399).aw_pa;
    let cali_desc: *mut aw_cali_desc = &mut (*aw_dev).cali_desc;
    let mut ret: i32;

    if (*cali_desc).cali_running {
        dev_err((*aw_dev).dev, c_str!("calibration in progress\n"));
        return -EINVAL;
    }

    (*cali_desc).cali_running = true;
    aw_cali_svc_run_mute(aw_dev, CALI_RESULT_NORMAL);

    ret = aw_cali_svc_cali_re_mode_enable(aw_dev, true);
    if ret != 0 {
        dev_err((*aw_dev).dev, c_str!("start cali re failed\n"));
        (*cali_desc).cali_running = false;
        return ret;
    }

    msleep(1000);

    ret = aw_cali_svc_get_smooth_cali_re(aw_dev);
    if ret != 0 {
        dev_err((*aw_dev).dev, c_str!("get cali re failed\n"));
    }

    aw_cali_svc_cali_re_mode_enable(aw_dev, false);
    (*cali_desc).cali_running = false;

    ret
}

unsafe fn aw88399_get_fade_in_time(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let aw88399: *mut aw88399 = snd_soc_component_get_drvdata(component);
    let aw_dev: *mut aw_device = (*aw88399).aw_pa;

    (*ucontrol).value.integer.value[0] = (*aw_dev).fade_in_time as _;
    0
}

unsafe fn aw88399_set_fade_in_time(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let aw88399: *mut aw88399 = snd_soc_component_get_drvdata(component);
    let mc: *mut soc_mixer_control = (*kcontrol).private_value as *mut soc_mixer_control;
    let aw_dev: *mut aw_device = (*aw88399).aw_pa;
    let time: i32 = (*ucontrol).value.integer.value[0] as i32;

    if time < (*mc).min || time > (*mc).max {
        return -EINVAL;
    }

    if time != (*aw_dev).fade_in_time {
        (*aw_dev).fade_in_time = time;
        return 1;
    }

    0
}

unsafe fn aw88399_get_fade_out_time(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let aw88399: *mut aw88399 = snd_soc_component_get_drvdata(component);
    let aw_dev: *mut aw_device = (*aw88399).aw_pa;

    (*ucontrol).value.integer.value[0] = (*aw_dev).fade_out_time as _;
    0
}

unsafe fn aw88399_set_fade_out_time(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let aw88399: *mut aw88399 = snd_soc_component_get_drvdata(component);
    let mc: *mut soc_mixer_control = (*kcontrol).private_value as *mut soc_mixer_control;
    let aw_dev: *mut aw_device = (*aw88399).aw_pa;
    let time: i32 = (*ucontrol).value.integer.value[0] as i32;

    if time < (*mc).min || time > (*mc).max {
        return -EINVAL;
    }

    if time != (*aw_dev).fade_out_time {
        (*aw_dev).fade_out_time = time;
        return 1;
    }

    0
}

unsafe fn aw88399_dev_set_profile_index(aw_dev: *mut aw_device, index: i32) -> i32 {
    /* check the index whether is valid */
    if index >= (*aw_dev).prof_info.count || index < 0 {
        return -EINVAL;
    }
    /* check the index whether change */
    if (*aw_dev).prof_index == index {
        return -EINVAL;
    }

    (*aw_dev).prof_index = index;
    dev_dbg(
        (*aw_dev).dev,
        c_str!("set prof[%s]"),
        (*aw_dev).prof_info.prof_name_list[(*aw_dev).prof_info.prof_desc[index as usize].id as usize],
    );

    0
}

unsafe fn aw88399_profile_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    let codec: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let aw88399: *mut aw88399 = snd_soc_component_get_drvdata(codec);
    let mut prof_name: *mut i8 = core::ptr::null_mut();
    let mut count: i32;
    let ret: i32;

    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_ENUMERATED;
    (*uinfo).count = 1;

    count = (*(*aw88399).aw_pa).prof_info.count;
    if count <= 0 {
        (*uinfo).value.enumerated.items = 0;
        return 0;
    }

    (*uinfo).value.enumerated.items = count as _;

    if (*uinfo).value.enumerated.item >= count as _ {
        (*uinfo).value.enumerated.item = (count - 1) as _;
    }

    count = (*uinfo).value.enumerated.item as i32;
    ret = aw88399_dev_get_prof_name((*aw88399).aw_pa, count, &mut prof_name);
    if ret != 0 {
        strscpy((*uinfo).value.enumerated.name.as_mut_ptr(), c_str!("null"));
        return 0;
    }

    strscpy((*uinfo).value.enumerated.name.as_mut_ptr(), prof_name);
    0
}

unsafe fn aw88399_profile_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let codec: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let aw88399: *mut aw88399 = snd_soc_component_get_drvdata(codec);

    (*ucontrol).value.integer.value[0] = (*(*aw88399).aw_pa).prof_index as _;
    0
}

unsafe fn aw88399_profile_set(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let codec: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let aw88399: *mut aw88399 = snd_soc_component_get_drvdata(codec);
    let mut ret: i32;

    mutex_lock(&mut (*aw88399).lock);
    ret = aw88399_dev_set_profile_index((*aw88399).aw_pa, (*ucontrol).value.integer.value[0] as i32);
    if ret != 0 {
        dev_dbg((*codec).dev, c_str!("profile index does not change"));
        mutex_unlock(&mut (*aw88399).lock);
        return 0;
    }

    if (*(*aw88399).aw_pa).status {
        aw88399_stop((*aw88399).aw_pa);
        aw88399_start(aw88399, AW88399_SYNC_START);
    }

    mutex_unlock(&mut (*aw88399).lock);
    1
}

unsafe fn aw88399_volume_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let codec: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let aw88399: *mut aw88399 = snd_soc_component_get_drvdata(codec);
    let vol_desc: *mut aw_volume_desc = &mut (*(*aw88399).aw_pa).volume_desc;

    (*ucontrol).value.integer.value[0] = (*vol_desc).ctl_volume as _;
    0
}

unsafe fn aw88399_volume_set(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let codec: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let aw88399: *mut aw88399 = snd_soc_component_get_drvdata(codec);
    let vol_desc: *mut aw_volume_desc = &mut (*(*aw88399).aw_pa).volume_desc;
    let mc: *mut soc_mixer_control = (*kcontrol).private_value as *mut soc_mixer_control;
    let value: i32 = (*ucontrol).value.integer.value[0] as i32;

    if value < (*mc).min || value > (*mc).max {
        return -EINVAL;
    }

    if (*vol_desc).ctl_volume != value {
        (*vol_desc).ctl_volume = value;
        aw_dev_set_volume((*aw88399).aw_pa, (*vol_desc).ctl_volume);
        return 1;
    }

    0
}

unsafe fn aw88399_get_fade_step(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let codec: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let aw88399: *mut aw88399 = snd_soc_component_get_drvdata(codec);

    (*ucontrol).value.integer.value[0] = (*(*aw88399).aw_pa).fade_step as _;
    0
}

unsafe fn aw88399_set_fade_step(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let codec: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let aw88399: *mut aw88399 = snd_soc_component_get_drvdata(codec);
    let mc: *mut soc_mixer_control = (*kcontrol).private_value as *mut soc_mixer_control;
    let value: i32 = (*ucontrol).value.integer.value[0] as i32;

    if value < (*mc).min || value > (*mc).max {
        return -EINVAL;
    }

    if (*(*aw88399).aw_pa).fade_step != value {
        (*(*aw88399).aw_pa).fade_step = value;
        return 1;
    }

    0
}

unsafe fn aw88399_re_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let codec: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let aw88399: *mut aw88399 = snd_soc_component_get_drvdata(codec);
    let aw_dev: *mut aw_device = (*aw88399).aw_pa;

    (*ucontrol).value.integer.value[0] = (*aw_dev).cali_desc.cali_re as _;
    0
}

unsafe fn aw88399_re_set(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let codec: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let aw88399: *mut aw88399 = snd_soc_component_get_drvdata(codec);
    let mc: *mut soc_mixer_control = (*kcontrol).private_value as *mut soc_mixer_control;
    let aw_dev: *mut aw_device = (*aw88399).aw_pa;
    let value: i32 = (*ucontrol).value.integer.value[0] as i32;

    if value < (*mc).min || value > (*mc).max {
        return -EINVAL;
    }

    if (*aw_dev).cali_desc.cali_re != value as u32 {
        (*aw_dev).cali_desc.cali_re = value as u32;
        return 1;
    }

    0
}

unsafe fn aw88399_calib_switch_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let codec: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let aw88399: *mut aw88399 = snd_soc_component_get_drvdata(codec);
    let aw_dev: *mut aw_device = (*aw88399).aw_pa;

    (*ucontrol).value.integer.value[0] = (*aw_dev).cali_desc.cali_switch as _;
    0
}

unsafe fn aw88399_calib_switch_set(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let codec: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let aw88399: *mut aw88399 = snd_soc_component_get_drvdata(codec);
    let aw_dev: *mut aw_device = (*aw88399).aw_pa;

    if (*aw_dev).cali_desc.cali_switch == (*ucontrol).value.integer.value[0] as _ {
        return 0;
    }

    (*aw_dev).cali_desc.cali_switch = (*ucontrol).value.integer.value[0] as _;
    1
}

unsafe fn aw88399_calib_get(
    _kcontrol: *mut snd_kcontrol,
    _ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    /* do nothing */
    0
}

unsafe fn aw88399_calib_set(
    kcontrol: *mut snd_kcontrol,
    _ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let codec: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let aw88399: *mut aw88399 = snd_soc_component_get_drvdata(codec);
    let aw_dev: *mut aw_device = (*aw88399).aw_pa;

    if (*aw_dev).status && (*aw_dev).cali_desc.cali_switch {
        aw_cali_svc_dev_cali_re(aw88399);
    }

    0
}

static aw88399_controls: &[snd_kcontrol_new] = &[
    SOC_SINGLE_EXT!(c_str!("PCM Playback Volume"), AW88399_SYSCTRL2_REG, 6, AW88399_MUTE_VOL, 0, aw88399_volume_get, aw88399_volume_set),
    SOC_SINGLE_EXT!(c_str!("Fade Step"), 0, 0, AW88399_MUTE_VOL, 0, aw88399_get_fade_step, aw88399_set_fade_step),
    SOC_SINGLE_EXT!(c_str!("Volume Ramp Up Step"), 0, 0, FADE_TIME_MAX, FADE_TIME_MIN, aw88399_get_fade_in_time, aw88399_set_fade_in_time),
    SOC_SINGLE_EXT!(c_str!("Volume Ramp Down Step"), 0, 0, FADE_TIME_MAX, FADE_TIME_MIN, aw88399_get_fade_out_time, aw88399_set_fade_out_time),
    SOC_SINGLE_EXT!(c_str!("Calib"), 0, 0, AW88399_CALI_RE_MAX, 0, aw88399_re_get, aw88399_re_set),
    SOC_SINGLE_BOOL_EXT!(c_str!("Calib Switch"), 0, aw88399_calib_switch_get, aw88399_calib_switch_set),
    SOC_SINGLE_EXT!(c_str!("Trigger Calib"), SND_SOC_NOPM, 0, 1, 0, aw88399_calib_get, aw88399_calib_set),
    AW88399_PROFILE_EXT!(c_str!("AW88399 Profile Set"), aw88399_profile_info, aw88399_profile_get, aw88399_profile_set),
];

unsafe fn aw88399_playback_event(
    w: *mut snd_soc_dapm_widget,
    _k: *mut snd_kcontrol,
    event: i32,
) -> i32 {
    let component: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
    let aw88399: *mut aw88399 = snd_soc_component_get_drvdata(component);

    mutex_lock(&mut (*aw88399).lock);
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            aw88399_start(aw88399, AW88399_ASYNC_START);
        }
        SND_SOC_DAPM_POST_PMD => {
            aw88399_stop((*aw88399).aw_pa);
        }
        _ => {}
    }
    mutex_unlock(&mut (*aw88399).lock);

    0
}

static aw88399_dapm_widgets: &[snd_soc_dapm_widget] = &[
    /* playback */
    SND_SOC_DAPM_AIF_IN_E!(
        c_str!("AIF_RX"),
        c_str!("Speaker_Playback"),
        0,
        0,
        0,
        0,
        aw88399_playback_event,
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD
    ),
    SND_SOC_DAPM_OUTPUT!(c_str!("DAC Output")),
    /* capture */
    SND_SOC_DAPM_AIF_OUT!(c_str!("AIF_TX"), c_str!("Speaker_Capture"), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_INPUT!(c_str!("ADC Input")),
];

static aw88399_audio_map: &[snd_soc_dapm_route] = &[
    snd_soc_dapm_route {
        sink: c_str!("DAC Output"),
        control: core::ptr::null(),
        source: c_str!("AIF_RX"),
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dapm_route {
        sink: c_str!("AIF_TX"),
        control: core::ptr::null(),
        source: c_str!("ADC Input"),
        ..unsafe { core::mem::zeroed() }
    },
];

unsafe fn aw88399_codec_probe(component: *mut snd_soc_component) -> i32 {
    let aw88399: *mut aw88399 = snd_soc_component_get_drvdata(component);
    let ret: i32;

    INIT_DELAYED_WORK!(&mut (*aw88399).start_work, aw88399_startup_work);

    ret = aw88399_request_firmware_file(aw88399);
    if ret != 0 {
        dev_err((*(*aw88399).aw_pa).dev, c_str!("%s failed\n"), __func__!());
    }

    ret
}

unsafe fn aw88399_codec_remove(aw_codec: *mut snd_soc_component) {
    let aw88399: *mut aw88399 = snd_soc_component_get_drvdata(aw_codec);

    cancel_delayed_work_sync(&mut (*aw88399).start_work);
}

static soc_codec_dev_aw88399: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(aw88399_codec_probe),
    remove: Some(aw88399_codec_remove),
    dapm_widgets: aw88399_dapm_widgets.as_ptr(),
    num_dapm_widgets: aw88399_dapm_widgets.len() as _,
    dapm_routes: aw88399_audio_map.as_ptr(),
    num_dapm_routes: aw88399_audio_map.len() as _,
    controls: aw88399_controls.as_ptr(),
    num_controls: aw88399_controls.len() as _,
    ..unsafe { core::mem::zeroed() }
};

unsafe fn aw88399_i2c_probe(i2c: *mut i2c_client) -> i32 {
    let mut aw88399: *mut aw88399;
    let mut ret: i32;

    if !i2c_check_functionality((*i2c).adapter, I2C_FUNC_I2C) {
        return dev_err_probe(&mut (*i2c).dev, -ENXIO, c_str!("check_functionality failed"));
    }

    aw88399 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<aw88399>(), GFP_KERNEL) as *mut aw88399;
    if aw88399.is_null() {
        return -ENOMEM;
    }

    mutex_init(&mut (*aw88399).lock);
    i2c_set_clientdata(i2c, aw88399 as *mut core::ffi::c_void);

    (*aw88399).reset_gpio = devm_gpiod_get_optional(&mut (*i2c).dev, c_str!("reset"), GPIOD_OUT_LOW);
    if IS_ERR((*aw88399).reset_gpio as *const core::ffi::c_void) {
        return dev_err_probe(
            &mut (*i2c).dev,
            PTR_ERR((*aw88399).reset_gpio as *const core::ffi::c_void),
            c_str!("reset gpio not defined\n"),
        );
    }
    aw88399_hw_reset(aw88399);

    (*aw88399).regmap = devm_regmap_init_i2c(i2c, &aw88399_remap_config);
    if IS_ERR((*aw88399).regmap as *const core::ffi::c_void) {
        return dev_err_probe(
            &mut (*i2c).dev,
            PTR_ERR((*aw88399).regmap as *const core::ffi::c_void),
            c_str!("failed to init regmap\n"),
        );
    }

    /* aw pa init */
    ret = aw88399_init(aw88399, i2c, (*aw88399).regmap);
    if ret != 0 {
        return ret;
    }

    ret = devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &soc_codec_dev_aw88399,
        unsafe { aw88399_dai.as_mut_ptr() },
        ARRAY_SIZE!(aw88399_dai),
    );
    if ret != 0 {
        dev_err(&mut (*i2c).dev, c_str!("failed to register aw88399: %d"), ret);
    }

    ret
}

static aw88399_i2c_id: &[i2c_device_id] = &[
    i2c_device_id {
        name: AW88399_I2C_NAME,
        ..unsafe { core::mem::zeroed() }
    },
    i2c_device_id {
        ..unsafe { core::mem::zeroed() }
    },
];
MODULE_DEVICE_TABLE!(i2c, aw88399_i2c_id);

// Original C code conditionally compiled this ACPI table under CONFIG_ACPI.
#[cfg(CONFIG_ACPI)]
static aw88399_acpi_match: &[acpi_device_id] = &[
    acpi_device_id {
        id: c_str!("AWDZ8399"),
        driver_data: 0,
        ..unsafe { core::mem::zeroed() }
    },
    acpi_device_id {
        ..unsafe { core::mem::zeroed() }
    },
];
#[cfg(CONFIG_ACPI)]
MODULE_DEVICE_TABLE!(acpi, aw88399_acpi_match);

static mut aw88399_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: AW88399_I2C_NAME,
        acpi_match_table: ACPI_PTR!(aw88399_acpi_match),
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(aw88399_i2c_probe),
    id_table: aw88399_i2c_id.as_ptr(),
    ..unsafe { core::mem::zeroed() }
};
module_i2c_driver!(aw88399_i2c_driver);

MODULE_DESCRIPTION!(c_str!("ASoC AW88399 Smart PA Driver"));
MODULE_LICENSE!(c_str!("GPL v2"));

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
