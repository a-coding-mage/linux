// SPDX-License-Identifier: GPL-2.0-only
/*
 * Machine driver for AMD Pink Sardine platform using DMIC
 *
 * Copyright 2022 Advanced Micro Devices, Inc.
 */

// C dependencies:
// sound/soc.h
// sound/soc-dapm.h
// linux/module.h
// sound/pcm.h
// sound/pcm_params.h
// linux/io.h
// linux/dmi.h
// acp63.h

const DRV_NAME: &[u8] = b"acp_ps_mach\0";

snd_soc_dailink_def!(
    acp63_pdm,
    dailink_comp_array!(comp_cpu!(b"acp_ps_pdm_dma.0\0"))
);

snd_soc_dailink_def!(
    dmic_codec,
    dailink_comp_array!(comp_codec!(b"dmic-codec.0\0", b"dmic-hifi\0"))
);

snd_soc_dailink_def!(
    pdm_platform,
    dailink_comp_array!(comp_platform!(b"acp_ps_pdm_dma.0\0"))
);

static mut acp63_dai_pdm: [snd_soc_dai_link; 1] = [snd_soc_dai_link {
    name: b"acp63-dmic-capture\0".as_ptr() as *const core::ffi::c_char,
    stream_name: b"DMIC capture\0".as_ptr() as *const core::ffi::c_char,
    capture_only: 1,
    snd_soc_dailink_reg!(acp63_pdm, dmic_codec, pdm_platform)
}];

static mut acp63_card: snd_soc_card = snd_soc_card {
    name: b"acp63\0".as_ptr() as *const core::ffi::c_char,
    owner: THIS_MODULE,
    dai_link: unsafe { acp63_dai_pdm.as_mut_ptr() },
    num_links: 1,
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn acp63_probe(pdev: *mut platform_device) -> core::ffi::c_int {
    let mut card: *mut snd_soc_card;
    let ret: core::ffi::c_int;

    platform_set_drvdata(pdev, core::ptr::addr_of_mut!(acp63_card) as *mut core::ffi::c_void);
    card = platform_get_drvdata(pdev) as *mut snd_soc_card;
    acp63_card.dev = core::ptr::addr_of_mut!((*pdev).dev);

    ret = devm_snd_soc_register_card(core::ptr::addr_of_mut!((*pdev).dev), card);
    if ret != 0 {
        return dev_err_probe(
            core::ptr::addr_of_mut!((*pdev).dev),
            ret,
            b"snd_soc_register_card(%s) failed\n\0".as_ptr() as *const core::ffi::c_char,
            (*card).name,
        );
    }

    0
}

static mut acp63_mach_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"acp_ps_mach\0".as_ptr() as *const core::ffi::c_char,
        pm: unsafe { core::ptr::addr_of!(snd_soc_pm_ops) },
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(acp63_probe),
    ..unsafe { core::mem::zeroed() }
};

module_platform_driver!(acp63_mach_driver);

module_author!(b"Syed.SabaKareem@amd.com\0");
module_description!(b"AMD Pink Sardine support for DMIC\0");
module_license!(b"GPL v2\0");
module_alias!(b"platform:acp_ps_mach\0");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
