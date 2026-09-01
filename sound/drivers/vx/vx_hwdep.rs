// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Digigram VX soundcards
 *
 * DSP firmware management
 *
 * Copyright (c) 2002 by Takashi Iwai <tiwai@suse.de>
 */

/* C dependencies:
 * linux/device.h
 * linux/firmware.h
 * linux/slab.h
 * linux/vmalloc.h
 * linux/module.h
 * sound/core.h
 * sound/hwdep.h
 * sound/vx_core.h
 */

/* MODULE_FIRMWARE("vx/bx_1_vxp.b56"); */
/* MODULE_FIRMWARE("vx/bx_1_vp4.b56"); */
/* MODULE_FIRMWARE("vx/x1_1_vx2.xlx"); */
/* MODULE_FIRMWARE("vx/x1_2_v22.xlx"); */
/* MODULE_FIRMWARE("vx/x1_1_vxp.xlx"); */
/* MODULE_FIRMWARE("vx/x1_1_vp4.xlx"); */
/* MODULE_FIRMWARE("vx/bd56002.boot"); */
/* MODULE_FIRMWARE("vx/bd563v2.boot"); */
/* MODULE_FIRMWARE("vx/bd563s3.boot"); */
/* MODULE_FIRMWARE("vx/l_1_vx2.d56"); */
/* MODULE_FIRMWARE("vx/l_1_v22.d56"); */
/* MODULE_FIRMWARE("vx/l_1_vxp.d56"); */
/* MODULE_FIRMWARE("vx/l_1_vp4.d56"); */

use core::ffi::{c_char, c_int};

extern "C" {
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn request_firmware(
        fw: *mut *const firmware,
        name: *const c_char,
        device: *mut device,
    ) -> c_int;
    fn release_firmware(fw: *const firmware);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_vx_pcm_new(chip: *mut vx_core) -> c_int;
    fn snd_vx_mixer_new(chip: *mut vx_core) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
}

pub unsafe extern "C" fn snd_vx_setup_firmware(chip: *mut vx_core) -> c_int {
    static FW_FILES: [[*const c_char; 4]; VX_TYPE_NUMS] = {
        let mut fw_files = [[core::ptr::null(); 4]; VX_TYPE_NUMS];
        fw_files[VX_TYPE_BOARD] = [
            core::ptr::null(),
            b"x1_1_vx2.xlx\0".as_ptr() as *const c_char,
            b"bd56002.boot\0".as_ptr() as *const c_char,
            b"l_1_vx2.d56\0".as_ptr() as *const c_char,
        ];
        fw_files[VX_TYPE_V2] = [
            core::ptr::null(),
            b"x1_2_v22.xlx\0".as_ptr() as *const c_char,
            b"bd563v2.boot\0".as_ptr() as *const c_char,
            b"l_1_v22.d56\0".as_ptr() as *const c_char,
        ];
        fw_files[VX_TYPE_MIC] = [
            core::ptr::null(),
            b"x1_2_v22.xlx\0".as_ptr() as *const c_char,
            b"bd563v2.boot\0".as_ptr() as *const c_char,
            b"l_1_v22.d56\0".as_ptr() as *const c_char,
        ];
        fw_files[VX_TYPE_VXPOCKET] = [
            b"bx_1_vxp.b56\0".as_ptr() as *const c_char,
            b"x1_1_vxp.xlx\0".as_ptr() as *const c_char,
            b"bd563s3.boot\0".as_ptr() as *const c_char,
            b"l_1_vxp.d56\0".as_ptr() as *const c_char,
        ];
        fw_files[VX_TYPE_VXP440] = [
            b"bx_1_vp4.b56\0".as_ptr() as *const c_char,
            b"x1_1_vp4.xlx\0".as_ptr() as *const c_char,
            b"bd563s3.boot\0".as_ptr() as *const c_char,
            b"l_1_vp4.d56\0".as_ptr() as *const c_char,
        ];
        fw_files
    };

    let mut err: c_int;

    for i in 0..4 {
        let mut path = [0 as c_char; 32];
        let mut fw: *const firmware = core::ptr::null();

        if FW_FILES[(*chip).r#type as usize][i].is_null() {
            continue;
        }

        sprintf(
            path.as_mut_ptr(),
            b"vx/%s\0".as_ptr() as *const c_char,
            FW_FILES[(*chip).r#type as usize][i],
        );

        if request_firmware(&mut fw, path.as_ptr(), (*(*chip).card).dev) != 0 {
            dev_err(
                (*(*chip).card).dev,
                b"vx: can't load firmware %s\n\0".as_ptr() as *const c_char,
                path.as_ptr(),
            );
            return -ENOENT;
        }

        err = ((*(*chip).ops).load_dsp.unwrap())(chip, i as c_int, fw);
        if err < 0 {
            release_firmware(fw);
            return err;
        }

        if i == 1 {
            (*chip).chip_status |= VX_STAT_XILINX_LOADED;
        }

        /* CONFIG_PM: keep firmware for suspend/resume; otherwise release it now. */
        #[cfg(CONFIG_PM)]
        {
            (*chip).firmware[i] = fw;
        }
        #[cfg(not(CONFIG_PM))]
        {
            release_firmware(fw);
        }
    }

    /* ok, we reached to the last one */
    /* create the devices if not built yet */
    err = snd_vx_pcm_new(chip);
    if err < 0 {
        return err;
    }

    err = snd_vx_mixer_new(chip);
    if err < 0 {
        return err;
    }

    if (*(*chip).ops).add_controls.is_some() {
        err = ((*(*chip).ops).add_controls.unwrap())(chip);
        if err < 0 {
            return err;
        }
    }

    (*chip).chip_status |= VX_STAT_DEVICE_INIT;
    (*chip).chip_status |= VX_STAT_CHIP_INIT;

    snd_card_register((*chip).card)
}

/* exported */
pub unsafe extern "C" fn snd_vx_free_firmware(chip: *mut vx_core) {
    /* CONFIG_PM */
    #[cfg(CONFIG_PM)]
    {
        for i in 0..4 {
            release_firmware((*chip).firmware[i]);
        }
    }

    #[cfg(not(CONFIG_PM))]
    {
        let _ = chip;
    }
}

/* EXPORT_SYMBOL(snd_vx_setup_firmware); */
/* EXPORT_SYMBOL(snd_vx_free_firmware); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
