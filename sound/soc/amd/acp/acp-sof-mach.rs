// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2021, 2023 Advanced Micro Devices, Inc.
//
// Authors: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>
//

/*
 * SOF Machine Driver Support for ACP HW block
 */

// C dependencies:
// #include <sound/core.h>
// #include <sound/pcm_params.h>
// #include <sound/soc-acpi.h>
// #include <sound/soc-dapm.h>
// #include <linux/dmi.h>
// #include <linux/module.h>
// #include "acp-mach.h"

type c_int = i32;
type c_char = i8;
type c_void = core::ffi::c_void;
type kernel_ulong_t = usize;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: gfp_t = 0;
const QUIRK_TDM_MODE_ENABLE: kernel_ulong_t = 1;

const I2S_SP: u32 = 0;
const I2S_HS: u32 = 1;
const I2S_BT: u32 = 2;
const DMIC: u32 = 3;
const RT5682: u32 = 4;
const RT1019: u32 = 5;
const MAX98360A: u32 = 6;
const RT5682S: u32 = 7;
const NAU8825: u32 = 8;
const NAU8821: u32 = 9;
const MAX98388: u32 = 10;

type gfp_t = u32;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mach_params {
    pub subsystem_rev: u32,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub mach_params: mach_params,
}

#[repr(C)]
pub struct acp_card_drvdata {
    pub hs_cpu_id: u32,
    pub amp_cpu_id: u32,
    pub dmic_cpu_id: u32,
    pub bt_cpu_id: u32,
    pub hs_codec_id: u32,
    pub amp_codec_id: u32,
    pub dmic_codec_id: u32,
    pub soc_mclk: bool,
    pub tdm_mode: *const c_void,
    pub acp_rev: u32,
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub owner: *mut module,
    pub name: *const c_char,
    pub drvdata: *mut acp_card_drvdata,
}

#[repr(C)]
pub struct platform_device_id {
    pub name: *const c_char,
    pub driver_data: kernel_ulong_t,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
    pub id_entry: *const platform_device_id,
}

#[repr(C)]
pub struct dmi_system_id {
    pub driver_data: *const c_void,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub id_table: *const platform_device_id,
}

unsafe extern "C" {
    static mut THIS_MODULE: module;
    static snd_soc_pm_ops: dev_pm_ops;
    static acp_quirk_table: [dmi_system_id; 0];

    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: gfp_t) -> *mut c_void;
    fn dmi_first_match(list: *const dmi_system_id) -> *const dmi_system_id;
    fn acp_sofdsp_dai_links_create(card: *mut snd_soc_card) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
}

static mut sof_rt5682_rt1019_data: acp_card_drvdata = acp_card_drvdata {
    hs_cpu_id: I2S_SP,
    amp_cpu_id: I2S_SP,
    dmic_cpu_id: DMIC,
    bt_cpu_id: 0,
    hs_codec_id: RT5682,
    amp_codec_id: RT1019,
    dmic_codec_id: DMIC,
    soc_mclk: false,
    tdm_mode: core::ptr::null(),
    acp_rev: 0,
};

static mut sof_rt5682_max_data: acp_card_drvdata = acp_card_drvdata {
    hs_cpu_id: I2S_SP,
    amp_cpu_id: I2S_SP,
    dmic_cpu_id: DMIC,
    bt_cpu_id: 0,
    hs_codec_id: RT5682,
    amp_codec_id: MAX98360A,
    dmic_codec_id: DMIC,
    soc_mclk: false,
    tdm_mode: core::ptr::null(),
    acp_rev: 0,
};

static mut sof_rt5682s_rt1019_data: acp_card_drvdata = acp_card_drvdata {
    hs_cpu_id: I2S_SP,
    amp_cpu_id: I2S_SP,
    dmic_cpu_id: DMIC,
    bt_cpu_id: 0,
    hs_codec_id: RT5682S,
    amp_codec_id: RT1019,
    dmic_codec_id: DMIC,
    soc_mclk: false,
    tdm_mode: core::ptr::null(),
    acp_rev: 0,
};

static mut sof_rt5682s_max_data: acp_card_drvdata = acp_card_drvdata {
    hs_cpu_id: I2S_SP,
    amp_cpu_id: I2S_SP,
    dmic_cpu_id: DMIC,
    bt_cpu_id: 0,
    hs_codec_id: RT5682S,
    amp_codec_id: MAX98360A,
    dmic_codec_id: DMIC,
    soc_mclk: false,
    tdm_mode: core::ptr::null(),
    acp_rev: 0,
};

static mut sof_nau8825_data: acp_card_drvdata = acp_card_drvdata {
    hs_cpu_id: I2S_HS,
    amp_cpu_id: I2S_HS,
    dmic_cpu_id: DMIC,
    bt_cpu_id: 0,
    hs_codec_id: NAU8825,
    amp_codec_id: MAX98360A,
    dmic_codec_id: DMIC,
    soc_mclk: true,
    tdm_mode: core::ptr::null(),
    acp_rev: 0,
};

static mut sof_rt5682s_hs_rt1019_data: acp_card_drvdata = acp_card_drvdata {
    hs_cpu_id: I2S_HS,
    amp_cpu_id: I2S_HS,
    dmic_cpu_id: DMIC,
    bt_cpu_id: 0,
    hs_codec_id: RT5682S,
    amp_codec_id: RT1019,
    dmic_codec_id: DMIC,
    soc_mclk: true,
    tdm_mode: core::ptr::null(),
    acp_rev: 0,
};

static mut sof_nau8821_max98388_data: acp_card_drvdata = acp_card_drvdata {
    hs_cpu_id: I2S_SP,
    amp_cpu_id: I2S_HS,
    dmic_cpu_id: 0,
    bt_cpu_id: I2S_BT,
    hs_codec_id: NAU8821,
    amp_codec_id: MAX98388,
    dmic_codec_id: 0,
    soc_mclk: true,
    tdm_mode: core::ptr::null(),
    acp_rev: 0,
};

unsafe extern "C" fn acp_sof_probe(pdev: *mut platform_device) -> c_int {
    let card: *mut snd_soc_card;
    let dev: *mut device = core::ptr::addr_of_mut!((*pdev).dev);
    let mach: *mut snd_soc_acpi_mach =
        dev_get_platdata(core::ptr::addr_of_mut!((*pdev).dev)) as *mut snd_soc_acpi_mach;
    let dmi_id: *const dmi_system_id;
    let acp_card_drvdata: *mut acp_card_drvdata;
    let mut ret: c_int;

    if (*pdev).id_entry.is_null() {
        return -EINVAL;
    }

    card = devm_kzalloc(dev, core::mem::size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
    if card.is_null() {
        return -ENOMEM;
    }

    (*card).dev = dev;
    (*card).owner = core::ptr::addr_of_mut!(THIS_MODULE);
    (*card).name = (*(*pdev).id_entry).name;
    (*card).drvdata = (*(*pdev).id_entry).driver_data as *mut acp_card_drvdata;
    /* Widgets and controls added per-codec in acp-mach-common.c */

    acp_card_drvdata = (*card).drvdata;
    dmi_id = dmi_first_match(acp_quirk_table.as_ptr());
    if !dmi_id.is_null()
        && (*dmi_id).driver_data == QUIRK_TDM_MODE_ENABLE as *const c_void
    {
        (*acp_card_drvdata).tdm_mode = (*dmi_id).driver_data;
    }

    (*acp_card_drvdata).acp_rev = (*mach).mach_params.subsystem_rev;
    ret = acp_sofdsp_dai_links_create(card);
    if ret != 0 {
        return dev_err_probe(
            core::ptr::addr_of_mut!((*pdev).dev),
            ret,
            b"Failed to create DAI links\n\0".as_ptr() as *const c_char,
        );
    }

    ret = devm_snd_soc_register_card(core::ptr::addr_of_mut!((*pdev).dev), card);
    if ret != 0 {
        return dev_err_probe(
            core::ptr::addr_of_mut!((*pdev).dev),
            ret,
            b"Failed to register card(%s)\n\0".as_ptr() as *const c_char,
            (*card).name,
        );
    }
    0
}

static board_ids: [platform_device_id; 8] = [
    platform_device_id {
        name: b"rt5682-rt1019\0".as_ptr() as *const c_char,
        driver_data: unsafe { core::ptr::addr_of_mut!(sof_rt5682_rt1019_data) as kernel_ulong_t },
    },
    platform_device_id {
        name: b"rt5682-max\0".as_ptr() as *const c_char,
        driver_data: unsafe { core::ptr::addr_of_mut!(sof_rt5682_max_data) as kernel_ulong_t },
    },
    platform_device_id {
        name: b"rt5682s-max\0".as_ptr() as *const c_char,
        driver_data: unsafe { core::ptr::addr_of_mut!(sof_rt5682s_max_data) as kernel_ulong_t },
    },
    platform_device_id {
        name: b"rt5682s-rt1019\0".as_ptr() as *const c_char,
        driver_data: unsafe { core::ptr::addr_of_mut!(sof_rt5682s_rt1019_data) as kernel_ulong_t },
    },
    platform_device_id {
        name: b"nau8825-max\0".as_ptr() as *const c_char,
        driver_data: unsafe { core::ptr::addr_of_mut!(sof_nau8825_data) as kernel_ulong_t },
    },
    platform_device_id {
        name: b"rt5682s-hs-rt1019\0".as_ptr() as *const c_char,
        driver_data: unsafe { core::ptr::addr_of_mut!(sof_rt5682s_hs_rt1019_data) as kernel_ulong_t },
    },
    platform_device_id {
        name: b"nau8821-max\0".as_ptr() as *const c_char,
        driver_data: unsafe { core::ptr::addr_of_mut!(sof_nau8821_max98388_data) as kernel_ulong_t },
    },
    platform_device_id {
        name: core::ptr::null(),
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(platform, board_ids);

static mut acp_asoc_audio: platform_driver = platform_driver {
    driver: device_driver {
        name: b"sof_mach\0".as_ptr() as *const c_char,
        pm: unsafe { core::ptr::addr_of!(snd_soc_pm_ops) },
    },
    probe: Some(acp_sof_probe),
    id_table: board_ids.as_ptr(),
};

// module_platform_driver(acp_asoc_audio);
// MODULE_IMPORT_NS("SND_SOC_AMD_MACH");
// MODULE_DESCRIPTION("ACP SOF Machine Driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
