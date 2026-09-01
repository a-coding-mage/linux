// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2021 Advanced Micro Devices, Inc.
//
// Authors: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>
//

/*
 * Machine Driver Legacy Support for ACP HW block
 */

// C dependencies:
// <sound/core.h>, <sound/pcm_params.h>, <sound/soc-acpi.h>,
// <sound/soc-dapm.h>, <linux/dmi.h>, <linux/module.h>,
// "acp-mach.h", "acp3x-es83xx/acp3x-es83xx.h"

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type kernel_ulong_t = c_ulong;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_int = 0;

const I2S_SP: c_int = 0;
const I2S_HS: c_int = 1;
const DMIC: c_int = 2;
const RT5682: c_int = 3;
const RT5682S: c_int = 4;
const RT1019: c_int = 5;
const MAX98360A: c_int = 6;
const ES83XX: c_int = 7;
const NAU8825: c_int = 8;
const QUIRK_TDM_MODE_ENABLE: usize = 1;

#[repr(C)]
pub struct acp_card_drvdata {
    pub hs_cpu_id: c_int,
    pub amp_cpu_id: c_int,
    pub dmic_cpu_id: c_int,
    pub hs_codec_id: c_int,
    pub amp_codec_id: c_int,
    pub dmic_codec_id: c_int,
    pub tdm_mode: bool,
    pub soc_mclk: bool,
    pub acpi_mach: *mut snd_soc_acpi_mach,
    pub acp_rev: c_int,
    pub ops: acp_ops,
}

#[repr(C)]
pub struct acp_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    pub drvdata: *mut c_void,
    pub dev: *mut device,
    pub owner: *mut c_void,
    pub name: *const c_char,
    pub suspend_pre: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub resume_post: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
}

#[repr(C)]
pub struct device {
    pub platform_data: *mut c_void,
}

#[repr(C)]
pub struct platform_device {
    pub name: *const c_char,
    pub dev: device,
    pub id_entry: *const platform_device_id,
}

#[repr(C)]
pub struct platform_device_id {
    pub name: *const c_char,
    pub driver_data: kernel_ulong_t,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub id_table: *const platform_device_id,
}

#[repr(C)]
pub struct driver {
    pub pm: *const c_void,
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub mach_params: snd_soc_acpi_mach_params,
}

#[repr(C)]
pub struct snd_soc_acpi_mach_params {
    pub subsystem_rev: c_int,
}

#[repr(C)]
pub struct dmi_system_id {
    pub driver_data: *mut c_void,
}

static mut rt5682_rt1019_data: acp_card_drvdata = acp_card_drvdata {
    hs_cpu_id: I2S_SP,
    amp_cpu_id: I2S_SP,
    dmic_cpu_id: DMIC,
    hs_codec_id: RT5682,
    amp_codec_id: RT1019,
    dmic_codec_id: DMIC,
    tdm_mode: false,
    soc_mclk: false,
    acpi_mach: ptr::null_mut(),
    acp_rev: 0,
    ops: acp_ops { _private: [] },
};

static mut rt5682s_max_data: acp_card_drvdata = acp_card_drvdata {
    hs_cpu_id: I2S_SP,
    amp_cpu_id: I2S_SP,
    dmic_cpu_id: DMIC,
    hs_codec_id: RT5682S,
    amp_codec_id: MAX98360A,
    dmic_codec_id: DMIC,
    tdm_mode: false,
    soc_mclk: false,
    acpi_mach: ptr::null_mut(),
    acp_rev: 0,
    ops: acp_ops { _private: [] },
};

static mut rt5682s_rt1019_data: acp_card_drvdata = acp_card_drvdata {
    hs_cpu_id: I2S_SP,
    amp_cpu_id: I2S_SP,
    dmic_cpu_id: DMIC,
    hs_codec_id: RT5682S,
    amp_codec_id: RT1019,
    dmic_codec_id: DMIC,
    tdm_mode: false,
    soc_mclk: false,
    acpi_mach: ptr::null_mut(),
    acp_rev: 0,
    ops: acp_ops { _private: [] },
};

static mut es83xx_rn_data: acp_card_drvdata = acp_card_drvdata {
    hs_cpu_id: I2S_SP,
    amp_cpu_id: 0,
    dmic_cpu_id: DMIC,
    hs_codec_id: ES83XX,
    amp_codec_id: 0,
    dmic_codec_id: DMIC,
    tdm_mode: false,
    soc_mclk: false,
    acpi_mach: ptr::null_mut(),
    acp_rev: 0,
    ops: acp_ops { _private: [] },
};

static mut max_nau8825_data: acp_card_drvdata = acp_card_drvdata {
    hs_cpu_id: I2S_HS,
    amp_cpu_id: I2S_HS,
    dmic_cpu_id: DMIC,
    hs_codec_id: NAU8825,
    amp_codec_id: MAX98360A,
    dmic_codec_id: DMIC,
    soc_mclk: true,
    tdm_mode: false,
    acpi_mach: ptr::null_mut(),
    acp_rev: 0,
    ops: acp_ops { _private: [] },
};

static mut rt5682s_rt1019_rmb_data: acp_card_drvdata = acp_card_drvdata {
    hs_cpu_id: I2S_HS,
    amp_cpu_id: I2S_HS,
    dmic_cpu_id: DMIC,
    hs_codec_id: RT5682S,
    amp_codec_id: RT1019,
    dmic_codec_id: DMIC,
    soc_mclk: true,
    tdm_mode: false,
    acpi_mach: ptr::null_mut(),
    acp_rev: 0,
    ops: acp_ops { _private: [] },
};

static mut acp_dmic_data: acp_card_drvdata = acp_card_drvdata {
    hs_cpu_id: 0,
    amp_cpu_id: 0,
    dmic_cpu_id: DMIC,
    hs_codec_id: 0,
    amp_codec_id: 0,
    dmic_codec_id: DMIC,
    tdm_mode: false,
    soc_mclk: false,
    acpi_mach: ptr::null_mut(),
    acp_rev: 0,
    ops: acp_ops { _private: [] },
};

unsafe fn acp_asoc_init_ops(priv_: *mut acp_card_drvdata) -> bool {
    let mut has_ops = false;

    if (*priv_).hs_codec_id == ES83XX {
        has_ops = true;
        acp3x_es83xx_init_ops(&mut (*priv_).ops);
    }
    has_ops
}

unsafe extern "C" fn acp_asoc_suspend_pre(card: *mut snd_soc_card) -> c_int {
    let ret: c_int;

    ret = acp_ops_suspend_pre(card);
    if ret == 1 {
        0
    } else {
        ret
    }
}

unsafe extern "C" fn acp_asoc_resume_post(card: *mut snd_soc_card) -> c_int {
    let ret: c_int;

    ret = acp_ops_resume_post(card);
    if ret == 1 {
        0
    } else {
        ret
    }
}

unsafe extern "C" fn acp_asoc_probe(pdev: *mut platform_device) -> c_int {
    let mut card: *mut snd_soc_card = ptr::null_mut();
    let dev: *mut device = &mut (*pdev).dev;
    let mach: *mut snd_soc_acpi_mach = dev_get_platdata(&mut (*pdev).dev) as *mut snd_soc_acpi_mach;
    let mut dmi_id: *const dmi_system_id;
    let acp_card_drvdata: *mut acp_card_drvdata;
    let mut ret: c_int;

    if (*pdev).id_entry.is_null() {
        ret = -EINVAL;
        return ret;
    }

    card = devm_kzalloc(dev, size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
    if card.is_null() {
        ret = -ENOMEM;
        return ret;
    }

    (*card).drvdata = (*(*pdev).id_entry).driver_data as *mut acp_card_drvdata as *mut c_void;
    acp_card_drvdata = (*card).drvdata as *mut acp_card_drvdata;
    (*acp_card_drvdata).acpi_mach = (*pdev).dev.platform_data as *mut snd_soc_acpi_mach;
    (*card).dev = dev;
    (*card).owner = THIS_MODULE;
    (*card).name = (*(*pdev).id_entry).name;

    acp_asoc_init_ops((*card).drvdata as *mut acp_card_drvdata);

    /* If widgets and controls are not set in specific callback,
     * they will be added per-codec in acp-mach-common.c
     */
    ret = acp_ops_configure_widgets(card);
    if ret < 0 {
        dev_err(
            &mut (*pdev).dev,
            b"Cannot configure widgets for card (%s): %d\n\0".as_ptr() as *const c_char,
            (*card).name,
            ret,
        );
        return ret;
    }
    (*card).suspend_pre = Some(acp_asoc_suspend_pre);
    (*card).resume_post = Some(acp_asoc_resume_post);

    ret = acp_ops_probe(card);
    if ret < 0 {
        dev_err(
            &mut (*pdev).dev,
            b"Cannot probe card (%s): %d\n\0".as_ptr() as *const c_char,
            (*card).name,
            ret,
        );
        return ret;
    }
    if strcmp((*pdev).name, b"acp-pdm-mach\0".as_ptr() as *const c_char) == 0 {
        (*acp_card_drvdata).acp_rev = *((*dev).platform_data as *mut c_int);
    } else {
        (*acp_card_drvdata).acp_rev = (*mach).mach_params.subsystem_rev;
    }

    dmi_id = dmi_first_match(acp_quirk_table);
    if !dmi_id.is_null()
        && (*dmi_id).driver_data == QUIRK_TDM_MODE_ENABLE as *mut c_void
    {
        (*acp_card_drvdata).tdm_mode = (*dmi_id).driver_data as usize != 0;
    }

    ret = acp_legacy_dai_links_create(card);
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            b"Cannot create dai links for card (%s): %d\n\0".as_ptr() as *const c_char,
            (*card).name,
            ret,
        );
        return ret;
    }

    ret = devm_snd_soc_register_card(&mut (*pdev).dev, card);
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            b"devm_snd_soc_register_card(%s) failed: %d\n\0".as_ptr() as *const c_char,
            (*card).name,
            ret,
        );
        return ret;
    }

    ret
}

static board_ids: [platform_device_id; 8] = [
    platform_device_id {
        name: b"acp3xalc56821019\0".as_ptr() as *const c_char,
        driver_data: unsafe { &mut rt5682_rt1019_data as *mut acp_card_drvdata as kernel_ulong_t },
    },
    platform_device_id {
        name: b"acp3xalc5682sm98360\0".as_ptr() as *const c_char,
        driver_data: unsafe { &mut rt5682s_max_data as *mut acp_card_drvdata as kernel_ulong_t },
    },
    platform_device_id {
        name: b"acp3xalc5682s1019\0".as_ptr() as *const c_char,
        driver_data: unsafe { &mut rt5682s_rt1019_data as *mut acp_card_drvdata as kernel_ulong_t },
    },
    platform_device_id {
        name: b"acp3x-es83xx\0".as_ptr() as *const c_char,
        driver_data: unsafe { &mut es83xx_rn_data as *mut acp_card_drvdata as kernel_ulong_t },
    },
    platform_device_id {
        name: b"rmb-nau8825-max\0".as_ptr() as *const c_char,
        driver_data: unsafe { &mut max_nau8825_data as *mut acp_card_drvdata as kernel_ulong_t },
    },
    platform_device_id {
        name: b"rmb-rt5682s-rt1019\0".as_ptr() as *const c_char,
        driver_data: unsafe { &mut rt5682s_rt1019_rmb_data as *mut acp_card_drvdata as kernel_ulong_t },
    },
    platform_device_id {
        name: b"acp-pdm-mach\0".as_ptr() as *const c_char,
        driver_data: unsafe { &mut acp_dmic_data as *mut acp_card_drvdata as kernel_ulong_t },
    },
    platform_device_id {
        name: ptr::null(),
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(platform, board_ids);

static acp_asoc_audio: platform_driver = platform_driver {
    driver: driver {
        pm: unsafe { &snd_soc_pm_ops as *const _ as *const c_void },
        name: b"acp_mach\0".as_ptr() as *const c_char,
    },
    probe: Some(acp_asoc_probe),
    id_table: board_ids.as_ptr(),
};

// module_platform_driver(acp_asoc_audio);
// MODULE_IMPORT_NS("SND_SOC_AMD_MACH");
// MODULE_DESCRIPTION("ACP chrome audio support");
// MODULE_LICENSE("GPL v2");

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: c_void;
    static acp_quirk_table: *const dmi_system_id;

    fn acp3x_es83xx_init_ops(ops: *mut acp_ops);
    fn acp_ops_suspend_pre(card: *mut snd_soc_card) -> c_int;
    fn acp_ops_resume_post(card: *mut snd_soc_card) -> c_int;
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
    fn acp_ops_configure_widgets(card: *mut snd_soc_card) -> c_int;
    fn acp_ops_probe(card: *mut snd_soc_card) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn dmi_first_match(list: *const dmi_system_id) -> *const dmi_system_id;
    fn acp_legacy_dai_links_create(card: *mut snd_soc_card) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
