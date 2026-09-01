// SPDX-License-Identifier: GPL-2.0
//
// TAS2781 HDA Shared Lib for I2C&SPI driver
//
// Copyright 2025 - 2026 Texas Instruments, Inc.
//
// Author: Shenghao Ding <shenghao-ding@ti.com>

// C dependencies translated as external crate/module dependencies:
// linux/component.h, linux/crc8.h, linux/crc32.h, linux/efi.h,
// linux/firmware.h, linux/i2c.h, linux/pm_runtime.h, sound/soc.h,
// sound/tas2781.h, and "tas2781_hda.h".

pub const CALIBRATION_DATA_AREA_NUM: usize = 2;

pub static tasdev_fct_efi_guid: [efi_guid_t; 3] = [
    /* DELL */
    EFI_GUID(
        0xcc92382d, 0x6337, 0x41cb, 0xa8, 0x8b, 0x8e, 0xce, 0x74, 0x91, 0xea,
        0x9f,
    ),
    /* HP */
    EFI_GUID(
        0x02f9af02, 0x7734, 0x4233, 0xb4, 0x3d, 0x93, 0xfe, 0x5a, 0xa3, 0x5d,
        0xb3,
    ),
    /* LENOVO & OTHERS */
    EFI_GUID(
        0x1f52d2a1, 0xbb3a, 0x457d, 0xbc, 0x09, 0x43, 0xa3, 0xf4, 0x31, 0x0a,
        0x92,
    ),
];
// EXPORT_SYMBOL_NS_GPL(tasdev_fct_efi_guid, "SND_HDA_SCODEC_TAS2781");

/*
 * The order of calibrated-data writing function is a bit different from the
 * order in UEFI. Here is the conversion to match the order of calibrated-data
 * writing function.
 */
unsafe fn cali_cnv(data: *mut u8, base: u32, offset: i32) {
    let mut reg_data: cali_reg = core::mem::zeroed();

    memcpy(
        &mut reg_data as *mut cali_reg as *mut c_void,
        data.add(base as usize) as *const c_void,
        core::mem::size_of::<cali_reg>(),
    );
    /* the data order has to be swapped between r0_low_reg and inv0_reg */
    core::mem::swap(&mut reg_data.r0_low_reg, &mut reg_data.invr0_reg);

    cpu_to_be32_array(
        data.add(offset as usize + 1) as *mut __be32,
        &mut reg_data as *mut cali_reg as *mut u32,
        TASDEV_CALIB_N,
    );
}

unsafe fn tas2781_apply_calib(p: *mut tasdevice_priv) {
    let cali_data: *mut calidata = &mut (*p).cali_data;
    let r: *mut cali_reg = &mut (*cali_data).cali_reg_array;
    let data: *mut u8 = (*cali_data).data;
    let tmp_val: *mut u32 = data as *mut u32;
    let mut cali_reg: [u32; TASDEV_CALIB_N as usize] = [
        TASDEVICE_REG(0, 0x17, 0x74),
        TASDEVICE_REG(0, 0x18, 0x0c),
        TASDEVICE_REG(0, 0x18, 0x14),
        TASDEVICE_REG(0, 0x13, 0x70),
        TASDEVICE_REG(0, 0x18, 0x7c),
    ];
    let mut crc: u32;
    let mut oft: u32;
    let node_num: u32;
    let mut buf: *mut u8;
    let mut i: i32;
    let mut j: i32;
    let mut k: i32;
    let mut l: i32;

    if *tmp_val.add(0) == 2781 {
        /*
         * New features were added in calibrated Data V3:
         *     1. Added calibration registers address define in
         *	    a node, marked as Device id == 0x80.
         * New features were added in calibrated Data V2:
         *     1. Added some the fields to store the link_id and
         *	    uniqie_id for multi-link solutions
         *     2. Support flexible number of devices instead of
         *	    fixed one in V1.
         * Layout of calibrated data V2 in UEFI(total 256 bytes):
         *     ChipID (2781, 4 bytes)
         *     Data-Group-Sum (4 bytes)
         *     TimeStamp of Calibration (4 bytes)
         *     for (i = 0; i < Data-Group-Sum; i++) {
         *	    if (Data type != 0x80) (4 bytes)
         *		 Calibrated Data of Device #i (20 bytes)
         *	    else
         *		 Calibration registers address (5*4 = 20 bytes)
         *		 # V2: No reg addr in data grp section.
         *		 # V3: Normally the last grp is the reg addr.
         *     }
         *     CRC (4 bytes)
         *     Reserved (the rest)
         */
        crc = crc32(
            !0u32,
            data,
            (3u32.wrapping_add((*tmp_val.add(1)).wrapping_mul(6))).wrapping_mul(4),
        ) ^ !0u32;

        if crc != *tmp_val.add((3u32.wrapping_add((*tmp_val.add(1)).wrapping_mul(6))) as usize) {
            (*cali_data).total_sz = 0;
            dev_err((*p).dev, c_str!("%s: CRC error\n"), c_str!("tas2781_apply_calib"));
            return;
        }
        node_num = *tmp_val.add(1);

        j = 0;
        k = 0;
        while j < node_num as i32 {
            oft = (j as u32).wrapping_mul(6).wrapping_add(3);
            if *tmp_val.add(oft as usize) == TASDEV_UEFI_CALI_REG_ADDR_FLG {
                i = 0;
                while i < TASDEV_CALIB_N as i32 {
                    buf = data.add((oft.wrapping_add(i as u32).wrapping_add(1)).wrapping_mul(4) as usize);
                    cali_reg[i as usize] = TASDEVICE_REG(
                        *buf.add(1) as u32,
                        *buf.add(2) as u32,
                        *buf.add(3) as u32,
                    );
                    i += 1;
                }
            } else {
                l = j * ((*cali_data).cali_dat_sz_per_dev as i32 + 1);
                if k >= (*p).ndev || l > oft as i32 * 4 {
                    dev_err(
                        (*p).dev,
                        c_str!("%s: dev sum error\n"),
                        c_str!("tas2781_apply_calib"),
                    );
                    (*cali_data).total_sz = 0;
                    return;
                }

                *data.add(l as usize) = k as u8;
                oft = oft.wrapping_add(1);
                cali_cnv(data, 4u32.wrapping_mul(oft), l);
                k += 1;
            }
            j += 1;
        }
    } else {
        /*
         * Calibration data is in V1 format.
         * struct cali_data {
         *     char cali_data[20];
         * }
         *
         * struct {
         *     struct cali_data cali_data[4];
         *     int  TimeStamp of Calibration (4 bytes)
         *     int CRC (4 bytes)
         * } ueft;
         */
        crc = crc32(!0u32, data, 84) ^ !0u32;
        if crc != *tmp_val.add(21) {
            (*cali_data).total_sz = 0;
            dev_err(
                (*p).dev,
                c_str!("%s: V1 CRC error\n"),
                c_str!("tas2781_apply_calib"),
            );
            return;
        }

        j = (*p).ndev - 1;
        while j >= 0 {
            l = j * ((*cali_data).cali_dat_sz_per_dev as i32 + 1);
            cali_cnv(data, (*cali_data).cali_dat_sz_per_dev.wrapping_mul(j as u32), l);
            *data.add(l as usize) = j as u8;
            j -= 1;
        }
    }

    if (*p).dspbin_typ == TASDEV_BASIC {
        (*r).r0_reg = cali_reg[0];
        (*r).invr0_reg = cali_reg[1];
        (*r).r0_low_reg = cali_reg[2];
        (*r).pow_reg = cali_reg[3];
        (*r).tlimit_reg = cali_reg[4];
    }

    (*cali_data).total_sz = ((*p).ndev as u64)
        .wrapping_mul(((*cali_data).cali_dat_sz_per_dev + 1) as u64) as _;
}

/*
 * Update the calibration data, including speaker impedance, f0, etc,
 * into algo. Calibrate data is done by manufacturer in the factory.
 * The data is used by Algo for calculating the speaker temperature,
 * speaker membrane excursion and f0 in real time during playback.
 * Calibration data format in EFI is V2, since 2024.
 */
pub unsafe extern "C" fn tas2781_save_calibration(hda: *mut tas2781_hda) -> i32 {
    /*
     * GUID was used for data access in BIOS, it was provided by board
     * manufactory.
     */
    let mut efi_guid: efi_guid_t = tasdev_fct_efi_guid[LENOVO as usize];
    /*
     * Some devices save the calibrated data into L"CALI_DATA",
     * and others into L"SmartAmpCalibrationData".
     */
    static mut EFI_NAME_0: [efi_char16_t; 10] = [
        b'C' as efi_char16_t,
        b'A' as efi_char16_t,
        b'L' as efi_char16_t,
        b'I' as efi_char16_t,
        b'_' as efi_char16_t,
        b'D' as efi_char16_t,
        b'A' as efi_char16_t,
        b'T' as efi_char16_t,
        b'A' as efi_char16_t,
        0,
    ];
    static mut EFI_NAME_1: [efi_char16_t; 24] = [
        b'S' as efi_char16_t,
        b'm' as efi_char16_t,
        b'a' as efi_char16_t,
        b'r' as efi_char16_t,
        b't' as efi_char16_t,
        b'A' as efi_char16_t,
        b'm' as efi_char16_t,
        b'p' as efi_char16_t,
        b'C' as efi_char16_t,
        b'a' as efi_char16_t,
        b'l' as efi_char16_t,
        b'i' as efi_char16_t,
        b'b' as efi_char16_t,
        b'r' as efi_char16_t,
        b'a' as efi_char16_t,
        b't' as efi_char16_t,
        b'i' as efi_char16_t,
        b'o' as efi_char16_t,
        b'n' as efi_char16_t,
        b'D' as efi_char16_t,
        b'a' as efi_char16_t,
        b't' as efi_char16_t,
        b'a' as efi_char16_t,
        0,
    ];
    let efi_name: [*mut efi_char16_t; CALIBRATION_DATA_AREA_NUM] =
        [EFI_NAME_0.as_mut_ptr(), EFI_NAME_1.as_mut_ptr()];
    let p: *mut tasdevice_priv = (*hda).priv;
    let cali_data: *mut calidata = &mut (*p).cali_data;
    let mut total_sz: c_ulong = 0;
    let mut attr: u32 = 0;
    let mut size: u32;
    let mut data: *mut u8;
    let mut status: efi_status_t;
    let mut i: i32;

    if !efi_rt_services_supported(EFI_RT_SUPPORTED_GET_VARIABLE) {
        dev_err((*p).dev, c_str!("%s: NO EFI FOUND!\n"), c_str!("tas2781_save_calibration"));
        return -EINVAL;
    }

    if (*hda).catlog_id < LENOVO {
        efi_guid = tasdev_fct_efi_guid[(*hda).catlog_id as usize];
    }

    (*cali_data).cali_dat_sz_per_dev = 20;
    size = ((*p).ndev as u32).wrapping_mul((*cali_data).cali_dat_sz_per_dev.wrapping_add(1));
    i = 0;
    loop {
        if i >= CALIBRATION_DATA_AREA_NUM as i32 {
            break;
        }
        /* Get real size of UEFI variable */
        status = ((*efi).get_variable)(
            efi_name[i as usize],
            &mut efi_guid,
            &mut attr,
            &mut total_sz,
            core::ptr::null_mut(),
        );
        (*cali_data).total_sz = if total_sz > size as c_ulong {
            total_sz
        } else {
            size as c_ulong
        };
        if status == EFI_BUFFER_TOO_SMALL {
            /* Allocate data buffer of data_size bytes */
            data = devm_kzalloc((*p).dev, (*cali_data).total_sz, GFP_KERNEL) as *mut u8;
            (*cali_data).data = data;
            if data.is_null() {
                status = -ENOMEM as efi_status_t;
                i += 1;
                continue;
            }
            /*
             * Set to an invalid value before the calibrated data
             * is stored into it, for the default value is 0, which
             * means the first device.
             */
            *data.add(0) = 0xff;
            /* Get variable contents into buffer */
            status = ((*efi).get_variable)(
                efi_name[i as usize],
                &mut efi_guid,
                &mut attr,
                &mut (*cali_data).total_sz,
                data as *mut c_void,
            );
        }
        /* Check whether get the calibrated data */
        if status == EFI_SUCCESS {
            break;
        }
        i += 1;
    }

    if status != EFI_SUCCESS {
        (*cali_data).total_sz = 0;
        return status as i32;
    }

    tas2781_apply_calib(p);

    0
}
// EXPORT_SYMBOL_NS_GPL(tas2781_save_calibration, "SND_HDA_SCODEC_TAS2781");

pub unsafe extern "C" fn tas2781_hda_remove(dev: *mut device, ops: *const component_ops) {
    let tas_hda: *mut tas2781_hda = dev_get_drvdata(dev) as *mut tas2781_hda;

    component_del((*tas_hda).dev, ops);

    pm_runtime_get_sync((*tas_hda).dev);
    pm_runtime_disable((*tas_hda).dev);

    pm_runtime_put_noidle((*tas_hda).dev);

    tasdevice_remove((*tas_hda).priv);
}
// EXPORT_SYMBOL_NS_GPL(tas2781_hda_remove, "SND_HDA_SCODEC_TAS2781");

pub unsafe extern "C" fn tasdevice_info_profile(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    let tas_priv: *mut tasdevice_priv = snd_kcontrol_chip(kcontrol) as *mut tasdevice_priv;

    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = (*tas_priv).rcabin.ncfgs - 1;

    0
}
// EXPORT_SYMBOL_NS_GPL(tasdevice_info_profile, "SND_HDA_SCODEC_TAS2781");

pub unsafe extern "C" fn tasdevice_info_programs(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    let tas_priv: *mut tasdevice_priv = snd_kcontrol_chip(kcontrol) as *mut tasdevice_priv;

    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = (*(*tas_priv).fmw).nr_programs - 1;

    0
}
// EXPORT_SYMBOL_NS_GPL(tasdevice_info_programs, "SND_HDA_SCODEC_TAS2781");

pub unsafe extern "C" fn tasdevice_info_config(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    let tas_priv: *mut tasdevice_priv = snd_kcontrol_chip(kcontrol) as *mut tasdevice_priv;
    let tas_fw: *mut tasdevice_fw = (*tas_priv).fmw;

    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = (*tas_fw).nr_configurations - 1;

    0
}
// EXPORT_SYMBOL_NS_GPL(tasdevice_info_config, "SND_HDA_SCODEC_TAS2781");

pub unsafe extern "C" fn tasdevice_get_profile_id(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tas_priv: *mut tasdevice_priv = snd_kcontrol_chip(kcontrol) as *mut tasdevice_priv;

    (*ucontrol).value.integer.value[0] = (*tas_priv).rcabin.profile_cfg_id;

    dev_dbg(
        (*tas_priv).dev,
        c_str!("%s: kcontrol %s: %d\n"),
        c_str!("tasdevice_get_profile_id"),
        (*kcontrol).id.name,
        (*tas_priv).rcabin.profile_cfg_id,
    );

    0
}
// EXPORT_SYMBOL_NS_GPL(tasdevice_get_profile_id, "SND_HDA_SCODEC_TAS2781");

pub unsafe extern "C" fn tasdevice_set_profile_id(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tas_priv: *mut tasdevice_priv = snd_kcontrol_chip(kcontrol) as *mut tasdevice_priv;
    let profile_id: i32 = (*ucontrol).value.integer.value[0];
    let max: i32 = (*tas_priv).rcabin.ncfgs - 1;
    let val: i32;
    let mut ret: i32 = 0;

    val = clamp(profile_id, 0, max);

    let _guard = guard_mutex(&mut (*tas_priv).codec_lock);

    dev_dbg(
        (*tas_priv).dev,
        c_str!("%s: kcontrol %s: %d -> %d\n"),
        c_str!("tasdevice_set_profile_id"),
        (*kcontrol).id.name,
        (*tas_priv).rcabin.profile_cfg_id,
        val,
    );

    if (*tas_priv).rcabin.profile_cfg_id != val {
        (*tas_priv).rcabin.profile_cfg_id = val;
        ret = 1;
    }

    ret
}
// EXPORT_SYMBOL_NS_GPL(tasdevice_set_profile_id, "SND_HDA_SCODEC_TAS2781");

pub unsafe extern "C" fn tasdevice_program_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tas_priv: *mut tasdevice_priv = snd_kcontrol_chip(kcontrol) as *mut tasdevice_priv;

    (*ucontrol).value.integer.value[0] = (*tas_priv).cur_prog;

    dev_dbg(
        (*tas_priv).dev,
        c_str!("%s: kcontrol %s: %d\n"),
        c_str!("tasdevice_program_get"),
        (*kcontrol).id.name,
        (*tas_priv).cur_prog,
    );

    0
}
// EXPORT_SYMBOL_NS_GPL(tasdevice_program_get, "SND_HDA_SCODEC_TAS2781");

pub unsafe extern "C" fn tasdevice_program_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tas_priv: *mut tasdevice_priv = snd_kcontrol_chip(kcontrol) as *mut tasdevice_priv;
    let tas_fw: *mut tasdevice_fw = (*tas_priv).fmw;
    let nr_program: i32 = (*ucontrol).value.integer.value[0];
    let max: i32 = (*tas_fw).nr_programs - 1;
    let val: i32;
    let mut ret: i32 = 0;

    val = clamp(nr_program, 0, max);

    let _guard = guard_mutex(&mut (*tas_priv).codec_lock);

    dev_dbg(
        (*tas_priv).dev,
        c_str!("%s: kcontrol %s: %d -> %d\n"),
        c_str!("tasdevice_program_put"),
        (*kcontrol).id.name,
        (*tas_priv).cur_prog,
        val,
    );

    if (*tas_priv).cur_prog != val {
        (*tas_priv).cur_prog = val;
        ret = 1;
    }

    ret
}
// EXPORT_SYMBOL_NS_GPL(tasdevice_program_put, "SND_HDA_SCODEC_TAS2781");

pub unsafe extern "C" fn tasdevice_config_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tas_priv: *mut tasdevice_priv = snd_kcontrol_chip(kcontrol) as *mut tasdevice_priv;

    (*ucontrol).value.integer.value[0] = (*tas_priv).cur_conf;

    dev_dbg(
        (*tas_priv).dev,
        c_str!("%s: kcontrol %s: %d\n"),
        c_str!("tasdevice_config_get"),
        (*kcontrol).id.name,
        (*tas_priv).cur_conf,
    );

    0
}
// EXPORT_SYMBOL_NS_GPL(tasdevice_config_get, "SND_HDA_SCODEC_TAS2781");

pub unsafe extern "C" fn tasdevice_config_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tas_priv: *mut tasdevice_priv = snd_kcontrol_chip(kcontrol) as *mut tasdevice_priv;
    let tas_fw: *mut tasdevice_fw = (*tas_priv).fmw;
    let nr_config: i32 = (*ucontrol).value.integer.value[0];
    let max: i32 = (*tas_fw).nr_configurations - 1;
    let val: i32;
    let mut ret: i32 = 0;

    val = clamp(nr_config, 0, max);

    let _guard = guard_mutex(&mut (*tas_priv).codec_lock);

    dev_dbg(
        (*tas_priv).dev,
        c_str!("%s: kcontrol %s: %d -> %d\n"),
        c_str!("tasdevice_config_put"),
        (*kcontrol).id.name,
        (*tas_priv).cur_conf,
        val,
    );

    if (*tas_priv).cur_conf != val {
        (*tas_priv).cur_conf = val;
        ret = 1;
    }

    ret
}
// EXPORT_SYMBOL_NS_GPL(tasdevice_config_put, "SND_HDA_SCODEC_TAS2781");

// MODULE_DESCRIPTION("TAS2781 HDA Driver");
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Shenghao Ding, TI, <shenghao-ding@ti.com>");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
