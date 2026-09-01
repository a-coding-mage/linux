// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8958-dsp2.rs  --  WM8958 DSP2 support
 *
 * Copyright 2011 Wolfson Microelectronics plc
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

// C dependencies from:
// linux/cleanup.h, linux/module.h, linux/moduleparam.h, linux/init.h,
// linux/delay.h, linux/pm.h, linux/i2c.h, linux/platform_device.h,
// linux/slab.h, sound/soc.h, sound/initval.h, sound/tlv.h,
// trace/events/asoc.h, linux/mfd/wm8994/*.h, linux/unaligned.h, wm8994.h.

const WM_FW_BLOCK_INFO: u32 = 0xff;
const WM_FW_BLOCK_PM: u32 = 0x00;
const WM_FW_BLOCK_X: u32 = 0x01;
const WM_FW_BLOCK_Y: u32 = 0x02;
const WM_FW_BLOCK_Z: u32 = 0x03;
const WM_FW_BLOCK_I: u32 = 0x06;
const WM_FW_BLOCK_A: u32 = 0x08;
const WM_FW_BLOCK_C: u32 = 0x0c;

unsafe fn wm8958_dsp2_fw(
    component: *mut snd_soc_component,
    name: *const c_char,
    fw: *const firmware,
    check: bool,
) -> c_int {
    let wm8994: *mut wm8994_priv = snd_soc_component_get_drvdata(component);
    let mut data64: u64;
    let mut data32: u32;
    let mut data: *const u8;
    let mut str_: *mut c_char;
    let mut block_len: usize;
    let mut len: usize;
    let mut ret: c_int = 0;

    /* Suppress unneeded downloads */
    if (*wm8994).cur_fw == fw {
        return 0;
    }

    if (*fw).size < 32 {
        dev_err(
            (*component).dev,
            c"%s: firmware too short (%zd bytes)\n".as_ptr(),
            name,
            (*fw).size,
        );
        ret = -EINVAL;
        if !check {
            snd_soc_component_write(component, 0x900, 0x0);
            snd_soc_component_write(component, 0x102, 0x0);
        }
        return ret;
    }

    if memcmp((*fw).data as *const c_void, c"WMFW".as_ptr() as *const c_void, 4) != 0 {
        data32 = get_unaligned_be32((*fw).data);
        dev_err(
            (*component).dev,
            c"%s: firmware has bad file magic %08x\n".as_ptr(),
            name,
            data32,
        );
        ret = -EINVAL;
        if !check {
            snd_soc_component_write(component, 0x900, 0x0);
            snd_soc_component_write(component, 0x102, 0x0);
        }
        return ret;
    }

    len = get_unaligned_be32((*fw).data.add(4)) as usize;
    data32 = get_unaligned_be32((*fw).data.add(8));

    if ((data32 >> 24) & 0xff) != 0 {
        dev_err(
            (*component).dev,
            c"%s: unsupported firmware version %d\n".as_ptr(),
            name,
            (data32 >> 24) & 0xff,
        );
        ret = -EINVAL;
        if !check {
            snd_soc_component_write(component, 0x900, 0x0);
            snd_soc_component_write(component, 0x102, 0x0);
        }
        return ret;
    }
    if (data32 & 0xffff) != 8958 {
        dev_err(
            (*component).dev,
            c"%s: unsupported target device %d\n".as_ptr(),
            name,
            data32 & 0xffff,
        );
        ret = -EINVAL;
        if !check {
            snd_soc_component_write(component, 0x900, 0x0);
            snd_soc_component_write(component, 0x102, 0x0);
        }
        return ret;
    }
    if ((data32 >> 16) & 0xff) != 0xc {
        dev_err(
            (*component).dev,
            c"%s: unsupported target core %d\n".as_ptr(),
            name,
            (data32 >> 16) & 0xff,
        );
        ret = -EINVAL;
        if !check {
            snd_soc_component_write(component, 0x900, 0x0);
            snd_soc_component_write(component, 0x102, 0x0);
        }
        return ret;
    }

    if check {
        data64 = get_unaligned_be64((*fw).data.add(24));
        dev_info((*component).dev, c"%s timestamp %llx\n".as_ptr(), name, data64);
    } else {
        snd_soc_component_write(component, 0x102, 0x2);
        snd_soc_component_write(component, 0x900, 0x2);
    }

    data = (*fw).data.add(len);
    len = (*fw).size - len;
    while len != 0 {
        if len < 12 {
            dev_err(
                (*component).dev,
                c"%s short data block of %zd\n".as_ptr(),
                name,
                len,
            );
            ret = -EINVAL;
            if !check {
                snd_soc_component_write(component, 0x900, 0x0);
                snd_soc_component_write(component, 0x102, 0x0);
            }
            return ret;
        }

        block_len = get_unaligned_be32(data.add(4)) as usize;
        if block_len + 8 > len {
            dev_err(
                (*component).dev,
                c"%zd byte block longer than file\n".as_ptr(),
                block_len,
            );
            ret = -EINVAL;
            if !check {
                snd_soc_component_write(component, 0x900, 0x0);
                snd_soc_component_write(component, 0x102, 0x0);
            }
            return ret;
        }
        if block_len == 0 {
            dev_err((*component).dev, c"Zero length block\n".as_ptr());
            ret = -EINVAL;
            if !check {
                snd_soc_component_write(component, 0x900, 0x0);
                snd_soc_component_write(component, 0x102, 0x0);
            }
            return ret;
        }

        data32 = get_unaligned_be32(data);

        match (data32 >> 24) & 0xff {
            WM_FW_BLOCK_INFO => {
                /* Informational text */
                if check {
                    str_ = kzalloc(block_len + 1, GFP_KERNEL) as *mut c_char;
                    if !str_.is_null() {
                        memcpy(str_ as *mut c_void, data.add(8) as *const c_void, block_len);
                        dev_info((*component).dev, c"%s: %s\n".as_ptr(), name, str_);
                        kfree(str_ as *const c_void);
                    } else {
                        dev_err((*component).dev, c"Out of memory\n".as_ptr());
                    }
                }
            }
            WM_FW_BLOCK_PM | WM_FW_BLOCK_X | WM_FW_BLOCK_Y | WM_FW_BLOCK_Z
            | WM_FW_BLOCK_I | WM_FW_BLOCK_A | WM_FW_BLOCK_C => {
                dev_dbg(
                    (*component).dev,
                    c"%s: %zd bytes of %x@%x\n".as_ptr(),
                    name,
                    block_len,
                    (data32 >> 24) & 0xff,
                    data32 & 0xffffff,
                );

                if !check {
                    data32 &= 0xffffff;
                    wm8994_bulk_write(
                        (*wm8994).wm8994,
                        data32 & 0xffffff,
                        (block_len / 2) as c_int,
                        data.add(8) as *mut c_void,
                    );
                }
            }
            _ => {
                dev_warn(
                    (*component).dev,
                    c"%s: unknown block type %d\n".as_ptr(),
                    name,
                    (data32 >> 24) & 0xff,
                );
            }
        }

        /* Round up to the next 32 bit word */
        block_len += block_len % 4;

        data = data.add(block_len + 8);
        len -= block_len + 8;
    }

    if !check {
        dev_dbg((*component).dev, c"%s: download done\n".as_ptr(), name);
        (*wm8994).cur_fw = fw;
    } else {
        dev_info((*component).dev, c"%s: got firmware\n".as_ptr(), name);
    }

    if !check {
        snd_soc_component_write(component, 0x900, 0x0);
        snd_soc_component_write(component, 0x102, 0x0);
    }

    ret
}

unsafe fn wm8958_dsp_start_mbc(component: *mut snd_soc_component, path: c_int) {
    let wm8994: *mut wm8994_priv = snd_soc_component_get_drvdata(component);
    let control: *mut wm8994 = (*wm8994).wm8994;

    /* If the DSP is already running then noop */
    if (snd_soc_component_read(component, WM8958_DSP2_PROGRAM) & WM8958_DSP2_ENA) != 0 {
        return;
    }

    /* If we have MBC firmware download it */
    if !(*wm8994).mbc.is_null() {
        wm8958_dsp2_fw(component, c"MBC".as_ptr(), (*wm8994).mbc, false);
    }

    snd_soc_component_update_bits(
        component,
        WM8958_DSP2_PROGRAM,
        WM8958_DSP2_ENA,
        WM8958_DSP2_ENA,
    );

    /* If we've got user supplied MBC settings use them */
    if (*control).pdata.num_mbc_cfgs != 0 {
        let cfg: *mut wm8958_mbc_cfg =
            &mut *(*control).pdata.mbc_cfgs.add((*wm8994).mbc_cfg as usize);

        for i in 0..(*cfg).coeff_regs.len() {
            snd_soc_component_write(
                component,
                i as c_int + WM8958_MBC_BAND_1_K_1,
                (*cfg).coeff_regs[i],
            );
        }

        for i in 0..(*cfg).cutoff_regs.len() {
            snd_soc_component_write(
                component,
                i as c_int + WM8958_MBC_BAND_2_LOWER_CUTOFF_C1_1,
                (*cfg).cutoff_regs[i],
            );
        }
    }

    /* Run the DSP */
    snd_soc_component_write(component, WM8958_DSP2_EXECCONTROL, WM8958_DSP2_RUNR);

    /* And we're off! */
    snd_soc_component_update_bits(
        component,
        WM8958_DSP2_CONFIG,
        WM8958_MBC_ENA | WM8958_MBC_SEL_MASK,
        (path << WM8958_MBC_SEL_SHIFT) | WM8958_MBC_ENA,
    );
}

unsafe fn wm8958_dsp_start_vss(component: *mut snd_soc_component, path: c_int) {
    let wm8994: *mut wm8994_priv = snd_soc_component_get_drvdata(component);
    let control: *mut wm8994 = (*wm8994).wm8994;
    let mut ena: c_int;

    if !(*wm8994).mbc_vss.is_null() {
        wm8958_dsp2_fw(component, c"MBC+VSS".as_ptr(), (*wm8994).mbc_vss, false);
    }

    snd_soc_component_update_bits(
        component,
        WM8958_DSP2_PROGRAM,
        WM8958_DSP2_ENA,
        WM8958_DSP2_ENA,
    );

    /* If we've got user supplied settings use them */
    if (*control).pdata.num_mbc_cfgs != 0 {
        let cfg: *mut wm8958_mbc_cfg =
            &mut *(*control).pdata.mbc_cfgs.add((*wm8994).mbc_cfg as usize);

        for i in 0..(*cfg).combined_regs.len() {
            snd_soc_component_write(component, i as c_int + 0x2800, (*cfg).combined_regs[i]);
        }
    }

    if (*control).pdata.num_vss_cfgs != 0 {
        let cfg: *mut wm8958_vss_cfg =
            &mut *(*control).pdata.vss_cfgs.add((*wm8994).vss_cfg as usize);

        for i in 0..(*cfg).regs.len() {
            snd_soc_component_write(component, i as c_int + 0x2600, (*cfg).regs[i]);
        }
    }

    if (*control).pdata.num_vss_hpf_cfgs != 0 {
        let cfg: *mut wm8958_vss_hpf_cfg =
            &mut *(*control).pdata.vss_hpf_cfgs.add((*wm8994).vss_hpf_cfg as usize);

        for i in 0..(*cfg).regs.len() {
            snd_soc_component_write(component, i as c_int + 0x2400, (*cfg).regs[i]);
        }
    }

    /* Run the DSP */
    snd_soc_component_write(component, WM8958_DSP2_EXECCONTROL, WM8958_DSP2_RUNR);

    /* Enable the algorithms we've selected */
    ena = 0;
    if (*wm8994).mbc_ena[path as usize] != 0 {
        ena |= 0x8;
    }
    if (*wm8994).hpf2_ena[path as usize] != 0 {
        ena |= 0x4;
    }
    if (*wm8994).hpf1_ena[path as usize] != 0 {
        ena |= 0x2;
    }
    if (*wm8994).vss_ena[path as usize] != 0 {
        ena |= 0x1;
    }

    snd_soc_component_write(component, 0x2201, ena);

    /* Switch the DSP into the data path */
    snd_soc_component_update_bits(
        component,
        WM8958_DSP2_CONFIG,
        WM8958_MBC_SEL_MASK | WM8958_MBC_ENA,
        (path << WM8958_MBC_SEL_SHIFT) | WM8958_MBC_ENA,
    );
}

unsafe fn wm8958_dsp_start_enh_eq(component: *mut snd_soc_component, path: c_int) {
    let wm8994: *mut wm8994_priv = snd_soc_component_get_drvdata(component);
    let control: *mut wm8994 = (*wm8994).wm8994;

    wm8958_dsp2_fw(component, c"ENH_EQ".as_ptr(), (*wm8994).enh_eq, false);

    snd_soc_component_update_bits(
        component,
        WM8958_DSP2_PROGRAM,
        WM8958_DSP2_ENA,
        WM8958_DSP2_ENA,
    );

    /* If we've got user supplied settings use them */
    if (*control).pdata.num_enh_eq_cfgs != 0 {
        let cfg: *mut wm8958_enh_eq_cfg =
            &mut *(*control).pdata.enh_eq_cfgs.add((*wm8994).enh_eq_cfg as usize);

        for i in 0..(*cfg).regs.len() {
            snd_soc_component_write(component, i as c_int + 0x2200, (*cfg).regs[i]);
        }
    }

    /* Run the DSP */
    snd_soc_component_write(component, WM8958_DSP2_EXECCONTROL, WM8958_DSP2_RUNR);

    /* Switch the DSP into the data path */
    snd_soc_component_update_bits(
        component,
        WM8958_DSP2_CONFIG,
        WM8958_MBC_SEL_MASK | WM8958_MBC_ENA,
        (path << WM8958_MBC_SEL_SHIFT) | WM8958_MBC_ENA,
    );
}

unsafe fn wm8958_dsp_apply(component: *mut snd_soc_component, path: c_int, start: c_int) {
    let wm8994: *mut wm8994_priv = snd_soc_component_get_drvdata(component);
    let mut pwr_reg: c_int = snd_soc_component_read(component, WM8994_POWER_MANAGEMENT_5);
    let mut ena: c_int;
    let reg: c_int;
    let aif: c_int;

    match path {
        0 => {
            pwr_reg &= WM8994_AIF1DAC1L_ENA | WM8994_AIF1DAC1R_ENA;
            aif = 0;
        }
        1 => {
            pwr_reg &= WM8994_AIF1DAC2L_ENA | WM8994_AIF1DAC2R_ENA;
            aif = 0;
        }
        2 => {
            pwr_reg &= WM8994_AIF2DACL_ENA | WM8994_AIF2DACR_ENA;
            aif = 1;
        }
        _ => {
            WARN(1, c"Invalid path %d\n".as_ptr(), path);
            return;
        }
    }

    /* Do we have both an active AIF and an active algorithm? */
    ena = ((*wm8994).mbc_ena[path as usize] != 0
        || (*wm8994).vss_ena[path as usize] != 0
        || (*wm8994).hpf1_ena[path as usize] != 0
        || (*wm8994).hpf2_ena[path as usize] != 0
        || (*wm8994).enh_eq_ena[path as usize] != 0) as c_int;
    if pwr_reg == 0 {
        ena = 0;
    }

    reg = snd_soc_component_read(component, WM8958_DSP2_PROGRAM);

    dev_dbg(
        (*component).dev,
        c"DSP path %d %d startup: %d, power: %x, DSP: %x\n".as_ptr(),
        path,
        (*wm8994).dsp_active,
        start,
        pwr_reg,
        reg,
    );

    if start != 0 && ena != 0 {
        /* If the DSP is already running then noop */
        if (reg & WM8958_DSP2_ENA) != 0 {
            return;
        }

        /* If either AIFnCLK is not yet enabled postpone */
        if (snd_soc_component_read(component, WM8994_AIF1_CLOCKING_1) & WM8994_AIF1CLK_ENA_MASK)
            == 0
            && (snd_soc_component_read(component, WM8994_AIF2_CLOCKING_1)
                & WM8994_AIF2CLK_ENA_MASK)
                == 0
        {
            return;
        }

        /* Switch the clock over to the appropriate AIF */
        snd_soc_component_update_bits(
            component,
            WM8994_CLOCKING_1,
            WM8958_DSP2CLK_SRC | WM8958_DSP2CLK_ENA,
            (aif << WM8958_DSP2CLK_SRC_SHIFT) | WM8958_DSP2CLK_ENA,
        );

        if (*wm8994).enh_eq_ena[path as usize] != 0 {
            wm8958_dsp_start_enh_eq(component, path);
        } else if (*wm8994).vss_ena[path as usize] != 0
            || (*wm8994).hpf1_ena[path as usize] != 0
            || (*wm8994).hpf2_ena[path as usize] != 0
        {
            wm8958_dsp_start_vss(component, path);
        } else if (*wm8994).mbc_ena[path as usize] != 0 {
            wm8958_dsp_start_mbc(component, path);
        }

        (*wm8994).dsp_active = path;

        dev_dbg((*component).dev, c"DSP running in path %d\n".as_ptr(), path);
    }

    if start == 0 && (*wm8994).dsp_active == path {
        /* If the DSP is already stopped then noop */
        if (reg & WM8958_DSP2_ENA) == 0 {
            return;
        }

        snd_soc_component_update_bits(component, WM8958_DSP2_CONFIG, WM8958_MBC_ENA, 0);
        snd_soc_component_write(component, WM8958_DSP2_EXECCONTROL, WM8958_DSP2_STOP);
        snd_soc_component_update_bits(component, WM8958_DSP2_PROGRAM, WM8958_DSP2_ENA, 0);
        snd_soc_component_update_bits(component, WM8994_CLOCKING_1, WM8958_DSP2CLK_ENA, 0);

        (*wm8994).dsp_active = -1;

        dev_dbg((*component).dev, c"DSP stopped\n".as_ptr());
    }
}

pub unsafe extern "C" fn wm8958_aif_ev(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
    let control: *mut wm8994 = dev_get_drvdata((*(*component).dev).parent);

    if (*control).type_ != WM8958 {
        return 0;
    }

    match event {
        SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMU => {
            for i in 0..3 {
                wm8958_dsp_apply(component, i, 1);
            }
        }
        SND_SOC_DAPM_POST_PMD | SND_SOC_DAPM_PRE_PMD => {
            for i in 0..3 {
                wm8958_dsp_apply(component, i, 0);
            }
        }
        _ => {}
    }

    0
}

/* Check if DSP2 is in use on another AIF */
unsafe fn wm8958_dsp2_busy(wm8994: *mut wm8994_priv, aif: c_int) -> c_int {
    for i in 0..(*wm8994).mbc_ena.len() {
        if i as c_int == aif {
            continue;
        }
        if (*wm8994).mbc_ena[i] != 0
            || (*wm8994).vss_ena[i] != 0
            || (*wm8994).hpf1_ena[i] != 0
            || (*wm8994).hpf2_ena[i] != 0
        {
            return 1;
        }
    }

    0
}

unsafe fn wm8958_put_mbc_enum(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let wm8994: *mut wm8994_priv = snd_soc_component_get_drvdata(component);
    let control: *mut wm8994 = (*wm8994).wm8994;
    let value: c_int = (*ucontrol).value.enumerated.item[0] as c_int;
    let reg: c_int;

    /* Don't allow on the fly reconfiguration */
    reg = snd_soc_component_read(component, WM8994_CLOCKING_1);
    if reg < 0 || (reg & WM8958_DSP2CLK_ENA) != 0 {
        return -EBUSY;
    }

    if value >= (*control).pdata.num_mbc_cfgs {
        return -EINVAL;
    }

    (*wm8994).mbc_cfg = value;

    0
}

unsafe fn wm8958_get_mbc_enum(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let wm8994: *mut wm8994_priv = snd_soc_component_get_drvdata(component);

    (*ucontrol).value.enumerated.item[0] = (*wm8994).mbc_cfg as c_uint;

    0
}

unsafe fn wm8958_mbc_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 1;
    0
}

unsafe fn wm8958_mbc_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let mbc: c_int = (*kcontrol).private_value as c_int;
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let wm8994: *mut wm8994_priv = snd_soc_component_get_drvdata(component);

    (*ucontrol).value.integer.value[0] = (*wm8994).mbc_ena[mbc as usize] as c_long;

    0
}

unsafe fn wm8958_mbc_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let mbc: c_int = (*kcontrol).private_value as c_int;
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let wm8994: *mut wm8994_priv = snd_soc_component_get_drvdata(component);
    let value = (*ucontrol).value.integer.value[0] as c_int;

    if (*wm8994).mbc_ena[mbc as usize] == value {
        return 0;
    }

    if value > 1 {
        return -EINVAL;
    }

    if wm8958_dsp2_busy(wm8994, mbc) != 0 {
        dev_dbg((*component).dev, c"DSP2 active on %d already\n".as_ptr(), mbc);
        return -EBUSY;
    }

    if (*wm8994).enh_eq_ena[mbc as usize] != 0 {
        return -EBUSY;
    }

    (*wm8994).mbc_ena[mbc as usize] = value;

    wm8958_dsp_apply(component, mbc, (*wm8994).mbc_ena[mbc as usize]);

    1
}

// Translation of WM8958_MBC_SWITCH(xname, xval).
const fn WM8958_MBC_SWITCH(xname: *const c_char, xval: c_long) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: xname,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
        info: Some(wm8958_mbc_info),
        get: Some(wm8958_mbc_get),
        put: Some(wm8958_mbc_put),
        private_value: xval,
    }
}

unsafe fn wm8958_put_vss_enum(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let wm8994: *mut wm8994_priv = snd_soc_component_get_drvdata(component);
    let control: *mut wm8994 = (*wm8994).wm8994;
    let value: c_int = (*ucontrol).value.enumerated.item[0] as c_int;
    let reg: c_int;

    /* Don't allow on the fly reconfiguration */
    reg = snd_soc_component_read(component, WM8994_CLOCKING_1);
    if reg < 0 || (reg & WM8958_DSP2CLK_ENA) != 0 {
        return -EBUSY;
    }

    if value >= (*control).pdata.num_vss_cfgs {
        return -EINVAL;
    }

    (*wm8994).vss_cfg = value;

    0
}

unsafe fn wm8958_get_vss_enum(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let wm8994: *mut wm8994_priv = snd_soc_component_get_drvdata(component);

    (*ucontrol).value.enumerated.item[0] = (*wm8994).vss_cfg as c_uint;

    0
}

unsafe fn wm8958_put_vss_hpf_enum(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let wm8994: *mut wm8994_priv = snd_soc_component_get_drvdata(component);
    let control: *mut wm8994 = (*wm8994).wm8994;
    let value: c_int = (*ucontrol).value.enumerated.item[0] as c_int;
    let reg: c_int;

    /* Don't allow on the fly reconfiguration */
    reg = snd_soc_component_read(component, WM8994_CLOCKING_1);
    if reg < 0 || (reg & WM8958_DSP2CLK_ENA) != 0 {
        return -EBUSY;
    }

    if value >= (*control).pdata.num_vss_hpf_cfgs {
        return -EINVAL;
    }

    (*wm8994).vss_hpf_cfg = value;

    0
}

unsafe fn wm8958_get_vss_hpf_enum(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let wm8994: *mut wm8994_priv = snd_soc_component_get_drvdata(component);

    (*ucontrol).value.enumerated.item[0] = (*wm8994).vss_hpf_cfg as c_uint;

    0
}

unsafe fn wm8958_vss_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 1;
    0
}

unsafe fn wm8958_vss_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let vss: c_int = (*kcontrol).private_value as c_int;
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let wm8994: *mut wm8994_priv = snd_soc_component_get_drvdata(component);

    (*ucontrol).value.integer.value[0] = (*wm8994).vss_ena[vss as usize] as c_long;

    0
}

unsafe fn wm8958_vss_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let vss: c_int = (*kcontrol).private_value as c_int;
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let wm8994: *mut wm8994_priv = snd_soc_component_get_drvdata(component);
    let value = (*ucontrol).value.integer.value[0] as c_int;

    if (*wm8994).vss_ena[vss as usize] == value {
        return 0;
    }

    if value > 1 {
        return -EINVAL;
    }

    if (*wm8994).mbc_vss.is_null() {
        return -ENODEV;
    }

    if wm8958_dsp2_busy(wm8994, vss) != 0 {
        dev_dbg((*component).dev, c"DSP2 active on %d already\n".as_ptr(), vss);
        return -EBUSY;
    }

    if (*wm8994).enh_eq_ena[vss as usize] != 0 {
        return -EBUSY;
    }

    (*wm8994).vss_ena[vss as usize] = value;

    wm8958_dsp_apply(component, vss, (*wm8994).vss_ena[vss as usize]);

    1
}

// Translation of WM8958_VSS_SWITCH(xname, xval).
const fn WM8958_VSS_SWITCH(xname: *const c_char, xval: c_long) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: xname,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
        info: Some(wm8958_vss_info),
        get: Some(wm8958_vss_get),
        put: Some(wm8958_vss_put),
        private_value: xval,
    }
}

unsafe fn wm8958_hpf_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 1;
    0
}

unsafe fn wm8958_hpf_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let hpf: c_int = (*kcontrol).private_value as c_int;
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let wm8994: *mut wm8994_priv = snd_soc_component_get_drvdata(component);

    if hpf < 3 {
        (*ucontrol).value.integer.value[0] = (*wm8994).hpf1_ena[(hpf % 3) as usize] as c_long;
    } else {
        (*ucontrol).value.integer.value[0] = (*wm8994).hpf2_ena[(hpf % 3) as usize] as c_long;
    }

    0
}

unsafe fn wm8958_hpf_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let hpf: c_int = (*kcontrol).private_value as c_int;
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let wm8994: *mut wm8994_priv = snd_soc_component_get_drvdata(component);
    let value = (*ucontrol).value.integer.value[0] as c_int;

    if hpf < 3 {
        if (*wm8994).hpf1_ena[(hpf % 3) as usize] == value {
            return 0;
        }
    } else if (*wm8994).hpf2_ena[(hpf % 3) as usize] == value {
        return 0;
    }

    if value > 1 {
        return -EINVAL;
    }

    if (*wm8994).mbc_vss.is_null() {
        return -ENODEV;
    }

    if wm8958_dsp2_busy(wm8994, hpf % 3) != 0 {
        dev_dbg((*component).dev, c"DSP2 active on %d already\n".as_ptr(), hpf);
        return -EBUSY;
    }

    if (*wm8994).enh_eq_ena[(hpf % 3) as usize] != 0 {
        return -EBUSY;
    }

    if hpf < 3 {
        (*wm8994).hpf1_ena[(hpf % 3) as usize] = value;
    } else {
        (*wm8994).hpf2_ena[(hpf % 3) as usize] = value;
    }

    wm8958_dsp_apply(component, hpf % 3, value);

    1
}

// Translation of WM8958_HPF_SWITCH(xname, xval).
const fn WM8958_HPF_SWITCH(xname: *const c_char, xval: c_long) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: xname,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
        info: Some(wm8958_hpf_info),
        get: Some(wm8958_hpf_get),
        put: Some(wm8958_hpf_put),
        private_value: xval,
    }
}

unsafe fn wm8958_put_enh_eq_enum(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let wm8994: *mut wm8994_priv = snd_soc_component_get_drvdata(component);
    let control: *mut wm8994 = (*wm8994).wm8994;
    let value: c_int = (*ucontrol).value.enumerated.item[0] as c_int;
    let reg: c_int;

    /* Don't allow on the fly reconfiguration */
    reg = snd_soc_component_read(component, WM8994_CLOCKING_1);
    if reg < 0 || (reg & WM8958_DSP2CLK_ENA) != 0 {
        return -EBUSY;
    }

    if value >= (*control).pdata.num_enh_eq_cfgs {
        return -EINVAL;
    }

    (*wm8994).enh_eq_cfg = value;

    0
}

unsafe fn wm8958_get_enh_eq_enum(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let wm8994: *mut wm8994_priv = snd_soc_component_get_drvdata(component);

    (*ucontrol).value.enumerated.item[0] = (*wm8994).enh_eq_cfg as c_uint;

    0
}

unsafe fn wm8958_enh_eq_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 1;
    0
}

unsafe fn wm8958_enh_eq_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let eq: c_int = (*kcontrol).private_value as c_int;
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let wm8994: *mut wm8994_priv = snd_soc_component_get_drvdata(component);

    (*ucontrol).value.integer.value[0] = (*wm8994).enh_eq_ena[eq as usize] as c_long;

    0
}

unsafe fn wm8958_enh_eq_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let eq: c_int = (*kcontrol).private_value as c_int;
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let wm8994: *mut wm8994_priv = snd_soc_component_get_drvdata(component);
    let value = (*ucontrol).value.integer.value[0] as c_int;

    if (*wm8994).enh_eq_ena[eq as usize] == value {
        return 0;
    }

    if value > 1 {
        return -EINVAL;
    }

    if (*wm8994).enh_eq.is_null() {
        return -ENODEV;
    }

    if wm8958_dsp2_busy(wm8994, eq) != 0 {
        dev_dbg((*component).dev, c"DSP2 active on %d already\n".as_ptr(), eq);
        return -EBUSY;
    }

    if (*wm8994).mbc_ena[eq as usize] != 0
        || (*wm8994).vss_ena[eq as usize] != 0
        || (*wm8994).hpf1_ena[eq as usize] != 0
        || (*wm8994).hpf2_ena[eq as usize] != 0
    {
        return -EBUSY;
    }

    (*wm8994).enh_eq_ena[eq as usize] = value;

    wm8958_dsp_apply(component, eq, value);

    1
}

// Translation of WM8958_ENH_EQ_SWITCH(xname, xval).
const fn WM8958_ENH_EQ_SWITCH(xname: *const c_char, xval: c_long) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: xname,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
        info: Some(wm8958_enh_eq_info),
        get: Some(wm8958_enh_eq_get),
        put: Some(wm8958_enh_eq_put),
        private_value: xval,
    }
}

static wm8958_mbc_snd_controls: [snd_kcontrol_new; 3] = [
    WM8958_MBC_SWITCH(c"AIF1DAC1 MBC Switch".as_ptr(), 0),
    WM8958_MBC_SWITCH(c"AIF1DAC2 MBC Switch".as_ptr(), 1),
    WM8958_MBC_SWITCH(c"AIF2DAC MBC Switch".as_ptr(), 2),
];

static wm8958_vss_snd_controls: [snd_kcontrol_new; 9] = [
    WM8958_VSS_SWITCH(c"AIF1DAC1 VSS Switch".as_ptr(), 0),
    WM8958_VSS_SWITCH(c"AIF1DAC2 VSS Switch".as_ptr(), 1),
    WM8958_VSS_SWITCH(c"AIF2DAC VSS Switch".as_ptr(), 2),
    WM8958_HPF_SWITCH(c"AIF1DAC1 HPF1 Switch".as_ptr(), 0),
    WM8958_HPF_SWITCH(c"AIF1DAC2 HPF1 Switch".as_ptr(), 1),
    WM8958_HPF_SWITCH(c"AIF2DAC HPF1 Switch".as_ptr(), 2),
    WM8958_HPF_SWITCH(c"AIF1DAC1 HPF2 Switch".as_ptr(), 3),
    WM8958_HPF_SWITCH(c"AIF1DAC2 HPF2 Switch".as_ptr(), 4),
    WM8958_HPF_SWITCH(c"AIF2DAC HPF2 Switch".as_ptr(), 5),
];

static wm8958_enh_eq_snd_controls: [snd_kcontrol_new; 3] = [
    WM8958_ENH_EQ_SWITCH(c"AIF1DAC1 Enhanced EQ Switch".as_ptr(), 0),
    WM8958_ENH_EQ_SWITCH(c"AIF1DAC2 Enhanced EQ Switch".as_ptr(), 1),
    WM8958_ENH_EQ_SWITCH(c"AIF2DAC Enhanced EQ Switch".as_ptr(), 2),
];

unsafe extern "C" fn wm8958_enh_eq_loaded(fw: *const firmware, context: *mut c_void) {
    let component: *mut snd_soc_component = context as *mut snd_soc_component;
    let wm8994: *mut wm8994_priv = snd_soc_component_get_drvdata(component);

    if !fw.is_null() && wm8958_dsp2_fw(component, c"ENH_EQ".as_ptr(), fw, true) == 0 {
        /* C used guard(mutex)(&wm8994->fw_lock) for scoped locking. */
        let _guard = mutex_guard(&mut (*wm8994).fw_lock);
        (*wm8994).enh_eq = fw;
    }
}

unsafe extern "C" fn wm8958_mbc_vss_loaded(fw: *const firmware, context: *mut c_void) {
    let component: *mut snd_soc_component = context as *mut snd_soc_component;
    let wm8994: *mut wm8994_priv = snd_soc_component_get_drvdata(component);

    if !fw.is_null() && wm8958_dsp2_fw(component, c"MBC+VSS".as_ptr(), fw, true) == 0 {
        /* C used guard(mutex)(&wm8994->fw_lock) for scoped locking. */
        let _guard = mutex_guard(&mut (*wm8994).fw_lock);
        (*wm8994).mbc_vss = fw;
    }
}

unsafe extern "C" fn wm8958_mbc_loaded(fw: *const firmware, context: *mut c_void) {
    let component: *mut snd_soc_component = context as *mut snd_soc_component;
    let wm8994: *mut wm8994_priv = snd_soc_component_get_drvdata(component);

    if !fw.is_null() && wm8958_dsp2_fw(component, c"MBC".as_ptr(), fw, true) == 0 {
        /* C used guard(mutex)(&wm8994->fw_lock) for scoped locking. */
        let _guard = mutex_guard(&mut (*wm8994).fw_lock);
        (*wm8994).mbc = fw;
    }
}

pub unsafe extern "C" fn wm8958_dsp2_init(component: *mut snd_soc_component) {
    let wm8994: *mut wm8994_priv = snd_soc_component_get_drvdata(component);
    let control: *mut wm8994 = (*wm8994).wm8994;
    let pdata: *mut wm8994_pdata = &mut (*control).pdata;
    let mut ret: c_int;

    (*wm8994).dsp_active = -1;

    snd_soc_add_component_controls(
        component,
        wm8958_mbc_snd_controls.as_ptr(),
        wm8958_mbc_snd_controls.len() as c_uint,
    );
    snd_soc_add_component_controls(
        component,
        wm8958_vss_snd_controls.as_ptr(),
        wm8958_vss_snd_controls.len() as c_uint,
    );
    snd_soc_add_component_controls(
        component,
        wm8958_enh_eq_snd_controls.as_ptr(),
        wm8958_enh_eq_snd_controls.len() as c_uint,
    );

    /* We don't *require* firmware and don't want to delay boot */
    request_firmware_nowait(
        THIS_MODULE,
        FW_ACTION_UEVENT,
        c"wm8958_mbc.wfw".as_ptr(),
        (*component).dev,
        GFP_KERNEL,
        component as *mut c_void,
        Some(wm8958_mbc_loaded),
    );
    request_firmware_nowait(
        THIS_MODULE,
        FW_ACTION_UEVENT,
        c"wm8958_mbc_vss.wfw".as_ptr(),
        (*component).dev,
        GFP_KERNEL,
        component as *mut c_void,
        Some(wm8958_mbc_vss_loaded),
    );
    request_firmware_nowait(
        THIS_MODULE,
        FW_ACTION_UEVENT,
        c"wm8958_enh_eq.wfw".as_ptr(),
        (*component).dev,
        GFP_KERNEL,
        component as *mut c_void,
        Some(wm8958_enh_eq_loaded),
    );

    if (*pdata).num_mbc_cfgs != 0 {
        let mbc_control = [SOC_ENUM_EXT(
            c"MBC Mode".as_ptr(),
            &mut (*wm8994).mbc_enum,
            Some(wm8958_get_mbc_enum),
            Some(wm8958_put_mbc_enum),
        )];

        /* We need an array of texts for the enum API */
        (*wm8994).mbc_texts = kmalloc_array(
            (*pdata).num_mbc_cfgs as usize,
            core::mem::size_of::<*mut c_char>(),
            GFP_KERNEL,
        ) as *mut *const c_char;
        if (*wm8994).mbc_texts.is_null() {
            return;
        }

        for i in 0..(*pdata).num_mbc_cfgs as usize {
            *(*wm8994).mbc_texts.add(i) = (*(*pdata).mbc_cfgs.add(i)).name;
        }

        (*wm8994).mbc_enum.items = (*pdata).num_mbc_cfgs;
        (*wm8994).mbc_enum.texts = (*wm8994).mbc_texts;

        ret = snd_soc_add_component_controls((*wm8994).hubs.component, mbc_control.as_ptr(), 1);
        if ret != 0 {
            dev_err(
                (*(*wm8994).hubs.component).dev,
                c"Failed to add MBC mode controls: %d\n".as_ptr(),
                ret,
            );
        }
    }

    if (*pdata).num_vss_cfgs != 0 {
        let vss_control = [SOC_ENUM_EXT(
            c"VSS Mode".as_ptr(),
            &mut (*wm8994).vss_enum,
            Some(wm8958_get_vss_enum),
            Some(wm8958_put_vss_enum),
        )];

        /* We need an array of texts for the enum API */
        (*wm8994).vss_texts = kmalloc_array(
            (*pdata).num_vss_cfgs as usize,
            core::mem::size_of::<*mut c_char>(),
            GFP_KERNEL,
        ) as *mut *const c_char;
        if (*wm8994).vss_texts.is_null() {
            return;
        }

        for i in 0..(*pdata).num_vss_cfgs as usize {
            *(*wm8994).vss_texts.add(i) = (*(*pdata).vss_cfgs.add(i)).name;
        }

        (*wm8994).vss_enum.items = (*pdata).num_vss_cfgs;
        (*wm8994).vss_enum.texts = (*wm8994).vss_texts;

        ret = snd_soc_add_component_controls((*wm8994).hubs.component, vss_control.as_ptr(), 1);
        if ret != 0 {
            dev_err(
                (*(*wm8994).hubs.component).dev,
                c"Failed to add VSS mode controls: %d\n".as_ptr(),
                ret,
            );
        }
    }

    if (*pdata).num_vss_hpf_cfgs != 0 {
        let hpf_control = [SOC_ENUM_EXT(
            c"VSS HPF Mode".as_ptr(),
            &mut (*wm8994).vss_hpf_enum,
            Some(wm8958_get_vss_hpf_enum),
            Some(wm8958_put_vss_hpf_enum),
        )];

        /* We need an array of texts for the enum API */
        (*wm8994).vss_hpf_texts = kmalloc_array(
            (*pdata).num_vss_hpf_cfgs as usize,
            core::mem::size_of::<*mut c_char>(),
            GFP_KERNEL,
        ) as *mut *const c_char;
        if (*wm8994).vss_hpf_texts.is_null() {
            return;
        }

        for i in 0..(*pdata).num_vss_hpf_cfgs as usize {
            *(*wm8994).vss_hpf_texts.add(i) = (*(*pdata).vss_hpf_cfgs.add(i)).name;
        }

        (*wm8994).vss_hpf_enum.items = (*pdata).num_vss_hpf_cfgs;
        (*wm8994).vss_hpf_enum.texts = (*wm8994).vss_hpf_texts;

        ret = snd_soc_add_component_controls((*wm8994).hubs.component, hpf_control.as_ptr(), 1);
        if ret != 0 {
            dev_err(
                (*(*wm8994).hubs.component).dev,
                c"Failed to add VSS HPFmode controls: %d\n".as_ptr(),
                ret,
            );
        }
    }

    if (*pdata).num_enh_eq_cfgs != 0 {
        let eq_control = [SOC_ENUM_EXT(
            c"Enhanced EQ Mode".as_ptr(),
            &mut (*wm8994).enh_eq_enum,
            Some(wm8958_get_enh_eq_enum),
            Some(wm8958_put_enh_eq_enum),
        )];

        /* We need an array of texts for the enum API */
        (*wm8994).enh_eq_texts = kmalloc_array(
            (*pdata).num_enh_eq_cfgs as usize,
            core::mem::size_of::<*mut c_char>(),
            GFP_KERNEL,
        ) as *mut *const c_char;
        if (*wm8994).enh_eq_texts.is_null() {
            return;
        }

        for i in 0..(*pdata).num_enh_eq_cfgs as usize {
            *(*wm8994).enh_eq_texts.add(i) = (*(*pdata).enh_eq_cfgs.add(i)).name;
        }

        (*wm8994).enh_eq_enum.items = (*pdata).num_enh_eq_cfgs;
        (*wm8994).enh_eq_enum.texts = (*wm8994).enh_eq_texts;

        ret = snd_soc_add_component_controls((*wm8994).hubs.component, eq_control.as_ptr(), 1);
        if ret != 0 {
            dev_err(
                (*(*wm8994).hubs.component).dev,
                c"Failed to add enhanced EQ controls: %d\n".as_ptr(),
                ret,
            );
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
