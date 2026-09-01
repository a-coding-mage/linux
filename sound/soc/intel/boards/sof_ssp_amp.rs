// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2022 Intel Corporation

/*
 * sof_ssp_amp.c - ASoc Machine driver for Intel platforms
 * with RT1308/CS35L41 codec.
 */

// C dependencies:
// linux/acpi.h, linux/delay.h, linux/dmi.h, linux/module.h,
// linux/platform_device.h, sound/core.h, sound/jack.h, sound/pcm.h,
// sound/pcm_params.h, sound/sof.h, sof_board_helpers.h,
// sof_realtek_common.h, sof_cirrus_common.h

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

type KernelUlongT = c_ulong;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

const fn BIT(nr: c_ulong) -> c_ulong {
    1_c_ulong << nr
}

extern "C" {
    static THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: dev_pm_ops;

    fn sof_intel_board_card_late_probe(card: *mut snd_soc_card) -> c_int;
    fn sof_intel_board_set_dai_link(
        dev: *mut device,
        card: *mut snd_soc_card,
        ctx: *mut sof_card_private,
    ) -> c_int;
    fn sof_intel_board_get_ctx(dev: *mut device, quirk: c_ulong) -> *mut sof_card_private;
    fn dmi_check_system(list: *const dmi_system_id) -> c_int;
    fn cs35l41_set_dai_link(link: *mut snd_soc_dai_link);
    fn sof_rt1308_dai_link(link: *mut snd_soc_dai_link);
    fn cs35l41_set_codec_conf(card: *mut snd_soc_card);
    fn snd_soc_fixup_dai_links_platform_name(
        card: *mut snd_soc_card,
        platform: *const c_char,
    ) -> c_int;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);

    fn SOF_SSP_PORT_AMP(port: c_ulong) -> c_ulong;
    fn SOF_SSP_MASK_HDMI_CAPTURE(mask: c_ulong) -> c_ulong;
    fn SOF_NUM_IDISP_HDMI(num: c_ulong) -> c_ulong;
    fn SOF_SSP_PORT_BT_OFFLOAD(port: c_ulong) -> c_ulong;
    fn SOF_LINK_ORDER(
        link0: c_ulong,
        link1: c_ulong,
        link2: c_ulong,
        link3: c_ulong,
        link4: c_ulong,
        link5: c_ulong,
        link6: c_ulong,
    ) -> c_ulong;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    name: *const c_char,
    owner: *mut c_void,
    fully_routed: bool,
    late_probe: Option<unsafe extern "C" fn(card: *mut snd_soc_card) -> c_int>,
    dev: *mut device,
}

#[repr(C)]
pub struct mach_params {
    dmic_num: c_int,
    codec_mask: c_ulong,
    platform: *const c_char,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    mach_params: mach_params,
}

#[repr(C)]
pub struct platform_device_id {
    name: [c_char; 32],
    driver_data: KernelUlongT,
}

#[repr(C)]
pub struct platform_device {
    dev: device_with_platform_data,
    id_entry: *const platform_device_id,
}

#[repr(C)]
pub struct device_with_platform_data {
    platform_data: *mut c_void,
}

#[repr(C)]
pub struct platform_driver {
    probe: Option<unsafe extern "C" fn(pdev: *mut platform_device) -> c_int>,
    driver: device_driver,
    id_table: *const platform_device_id,
}

#[repr(C)]
pub struct device_driver {
    name: *const c_char,
    pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct dmi_system_id {
    ident: *const c_char,
    matches: [dmi_strmatch; 4],
}

#[repr(C)]
pub struct dmi_strmatch {
    slot: c_int,
    substr: *const c_char,
}

#[repr(C)]
pub struct sof_card_private {
    amp_type: c_int,
    amp_link: *mut snd_soc_dai_link,
    dmic_be_num: c_int,
    hdmi: sof_hdmi_private,
    hdmi_num: c_int,
    link_order_overwrite: c_ulong,
    ssp_mask_hdmi_in: c_ulong,
    link_id_overwrite: c_ulong,
}

#[repr(C)]
pub struct sof_hdmi_private {
    idisp_codec: bool,
}

const DMI_SYS_VENDOR: c_int = 1;
const CODEC_NONE: c_int = 0;
const CODEC_CS35L41: c_int = 1;
const CODEC_RT1308: c_int = 2;
const IDISP_CODEC_MASK: c_ulong = 1;
const SOF_LINK_HDMI_IN: c_ulong = 0;
const SOF_LINK_AMP: c_ulong = 1;
const SOF_LINK_DMIC01: c_ulong = 2;
const SOF_LINK_DMIC16K: c_ulong = 3;
const SOF_LINK_IDISP_HDMI: c_ulong = 4;
const SOF_LINK_BT_OFFLOAD: c_ulong = 5;
const SOF_LINK_NONE: c_ulong = 6;
const SOF_BT_OFFLOAD_PRESENT: c_ulong = BIT(1);

/* Driver-specific board quirks: from bit 0 to 7 */
const SOF_HDMI_PLAYBACK_PRESENT: c_ulong = BIT(0);

/* Default: SSP2  */
static mut sof_ssp_amp_quirk: c_ulong = unsafe { SOF_SSP_PORT_AMP(2) };

static chromebook_platforms: [dmi_system_id; 2] = [
    dmi_system_id {
        ident: b"Google Chromebooks\0".as_ptr() as *const c_char,
        matches: [
            dmi_strmatch {
                slot: DMI_SYS_VENDOR,
                substr: b"Google\0".as_ptr() as *const c_char,
            },
            dmi_strmatch {
                slot: 0,
                substr: ptr::null(),
            },
            dmi_strmatch {
                slot: 0,
                substr: ptr::null(),
            },
            dmi_strmatch {
                slot: 0,
                substr: ptr::null(),
            },
        ],
    },
    dmi_system_id {
        ident: ptr::null(),
        matches: [
            dmi_strmatch {
                slot: 0,
                substr: ptr::null(),
            },
            dmi_strmatch {
                slot: 0,
                substr: ptr::null(),
            },
            dmi_strmatch {
                slot: 0,
                substr: ptr::null(),
            },
            dmi_strmatch {
                slot: 0,
                substr: ptr::null(),
            },
        ],
    },
];

unsafe extern "C" fn sof_card_late_probe(card: *mut snd_soc_card) -> c_int {
    sof_intel_board_card_late_probe(card)
}

static mut sof_ssp_amp_card: snd_soc_card = snd_soc_card {
    name: b"ssp_amp\0".as_ptr() as *const c_char,
    owner: unsafe { THIS_MODULE },
    fully_routed: true,
    late_probe: Some(sof_card_late_probe),
    dev: ptr::null_mut(),
};

/* BE ID defined in sof-tgl-rt1308-hdmi-ssp.m4 */
const HDMI_IN_BE_ID: c_ulong = 0;
const SPK_BE_ID: c_ulong = 2;
const DMIC01_BE_ID: c_ulong = 3;
const INTEL_HDMI_BE_ID: c_ulong = 5;
/* extra BE links to support no-hdmi-in boards */
const DMIC16K_BE_ID: c_ulong = 4;
const BT_OFFLOAD_BE_ID: c_ulong = 8;

static mut SSP_AMP_LINK_ORDER: c_ulong = unsafe {
    SOF_LINK_ORDER(
        SOF_LINK_HDMI_IN,
        SOF_LINK_AMP,
        SOF_LINK_DMIC01,
        SOF_LINK_DMIC16K,
        SOF_LINK_IDISP_HDMI,
        SOF_LINK_BT_OFFLOAD,
        SOF_LINK_NONE,
    )
};

static mut SSP_AMP_LINK_IDS: c_ulong = unsafe {
    SOF_LINK_ORDER(
        HDMI_IN_BE_ID,
        SPK_BE_ID,
        DMIC01_BE_ID,
        DMIC16K_BE_ID,
        INTEL_HDMI_BE_ID,
        BT_OFFLOAD_BE_ID,
        0,
    )
};

unsafe extern "C" fn sof_card_dai_links_create(
    dev: *mut device,
    card: *mut snd_soc_card,
    ctx: *mut sof_card_private,
) -> c_int {
    let mut ret: c_int;

    ret = sof_intel_board_set_dai_link(dev, card, ctx);
    if ret != 0 {
        return ret;
    }

    if (*ctx).amp_type == CODEC_NONE {
        return 0;
    }

    if (*ctx).amp_link.is_null() {
        dev_err(dev, b"amp link not available\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    /* codec-specific fields for speaker amplifier */
    match (*ctx).amp_type {
        CODEC_CS35L41 => {
            cs35l41_set_dai_link((*ctx).amp_link);
        }
        CODEC_RT1308 => {
            sof_rt1308_dai_link((*ctx).amp_link);
        }
        _ => {
            dev_err(
                dev,
                b"invalid amp type %d\n\0".as_ptr() as *const c_char,
                (*ctx).amp_type,
            );
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn sof_ssp_amp_probe(pdev: *mut platform_device) -> c_int {
    let mach: *mut snd_soc_acpi_mach = (*pdev).dev.platform_data as *mut snd_soc_acpi_mach;
    let ctx: *mut sof_card_private;
    let mut ret: c_int;

    if !(*pdev).id_entry.is_null() && (*(*pdev).id_entry).driver_data != 0 {
        sof_ssp_amp_quirk = (*(*pdev).id_entry).driver_data as c_ulong;
    }

    dev_dbg(
        &mut (*pdev).dev as *mut device_with_platform_data as *mut device,
        b"sof_ssp_amp_quirk = %lx\n\0".as_ptr() as *const c_char,
        sof_ssp_amp_quirk,
    );

    /* initialize ctx with board quirk */
    ctx = sof_intel_board_get_ctx(
        &mut (*pdev).dev as *mut device_with_platform_data as *mut device,
        sof_ssp_amp_quirk,
    );
    if ctx.is_null() {
        return -ENOMEM;
    }

    if dmi_check_system(chromebook_platforms.as_ptr()) == 0 && (*mach).mach_params.dmic_num == 0 {
        (*ctx).dmic_be_num = 0;
    }

    if (sof_ssp_amp_quirk & SOF_HDMI_PLAYBACK_PRESENT) != 0 {
        if ((*mach).mach_params.codec_mask & IDISP_CODEC_MASK) != 0 {
            (*ctx).hdmi.idisp_codec = true;
        }
    } else {
        (*ctx).hdmi_num = 0;
    }

    (*ctx).link_order_overwrite = SSP_AMP_LINK_ORDER;

    if (*ctx).ssp_mask_hdmi_in != 0 {
        /* the topology supports HDMI-IN uses fixed BE ID for DAI links */
        (*ctx).link_id_overwrite = SSP_AMP_LINK_IDS;
    }

    /* update dai_link */
    ret = sof_card_dai_links_create(
        &mut (*pdev).dev as *mut device_with_platform_data as *mut device,
        &mut sof_ssp_amp_card,
        ctx,
    );
    if ret != 0 {
        return ret;
    }

    /* update codec_conf */
    match (*ctx).amp_type {
        CODEC_CS35L41 => {
            cs35l41_set_codec_conf(&mut sof_ssp_amp_card);
        }
        CODEC_RT1308 | CODEC_NONE => {
            /* no codec conf required */
        }
        _ => {
            dev_err(
                &mut (*pdev).dev as *mut device_with_platform_data as *mut device,
                b"invalid amp type %d\n\0".as_ptr() as *const c_char,
                (*ctx).amp_type,
            );
            return -EINVAL;
        }
    }

    sof_ssp_amp_card.dev = &mut (*pdev).dev as *mut device_with_platform_data as *mut device;

    /* set platform name for each dailink */
    ret = snd_soc_fixup_dai_links_platform_name(&mut sof_ssp_amp_card, (*mach).mach_params.platform);
    if ret != 0 {
        return ret;
    }

    snd_soc_card_set_drvdata(&mut sof_ssp_amp_card, ctx as *mut c_void);

    devm_snd_soc_register_card(
        &mut (*pdev).dev as *mut device_with_platform_data as *mut device,
        &mut sof_ssp_amp_card,
    )
}

static board_ids: [platform_device_id; 10] = [
    platform_device_id {
        name: *b"sof_ssp_amp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        driver_data: 0,
    },
    platform_device_id {
        name: *b"tgl_rt1308_hdmi_ssp\0\0\0\0\0\0\0\0\0\0\0\0",
        driver_data: unsafe { (SOF_SSP_PORT_AMP(2) | SOF_SSP_MASK_HDMI_CAPTURE(0x22)) as KernelUlongT },
        /* SSP 1 and SSP 5 are used for HDMI IN */
    },
    platform_device_id {
        name: *b"adl_cs35l41\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        driver_data: unsafe {
            (SOF_SSP_PORT_AMP(1)
                | SOF_NUM_IDISP_HDMI(4)
                | SOF_HDMI_PLAYBACK_PRESENT
                | SOF_SSP_PORT_BT_OFFLOAD(2)
                | SOF_BT_OFFLOAD_PRESENT) as KernelUlongT
        },
    },
    platform_device_id {
        name: *b"adl_lt6911_hdmi_ssp\0\0\0\0\0\0\0\0\0\0\0\0",
        driver_data: unsafe {
            (SOF_SSP_MASK_HDMI_CAPTURE(0x5)
                /* SSP 0 and SSP 2 are used for HDMI IN */
                | SOF_HDMI_PLAYBACK_PRESENT) as KernelUlongT
        },
    },
    platform_device_id {
        name: *b"rpl_lt6911_hdmi_ssp\0\0\0\0\0\0\0\0\0\0\0\0",
        driver_data: unsafe {
            (SOF_SSP_MASK_HDMI_CAPTURE(0x5)
                /* SSP 0 and SSP 2 are used for HDMI IN */
                | SOF_HDMI_PLAYBACK_PRESENT) as KernelUlongT
        },
    },
    platform_device_id {
        name: *b"mtl_lt6911_hdmi_ssp\0\0\0\0\0\0\0\0\0\0\0\0",
        driver_data: unsafe {
            (SOF_SSP_MASK_HDMI_CAPTURE(0x5)
                /* SSP 0 and SSP 2 are used for HDMI IN */
                | SOF_HDMI_PLAYBACK_PRESENT) as KernelUlongT
        },
    },
    platform_device_id {
        name: *b"arl_lt6911_hdmi_ssp\0\0\0\0\0\0\0\0\0\0\0\0",
        driver_data: unsafe {
            (SOF_SSP_MASK_HDMI_CAPTURE(0x5)
                /* SSP 0 and SSP 2 are used for HDMI IN */
                | SOF_HDMI_PLAYBACK_PRESENT) as KernelUlongT
        },
    },
    platform_device_id {
        name: *b"ptl_lt6911_hdmi_ssp\0\0\0\0\0\0\0\0\0\0\0\0",
        driver_data: unsafe {
            (SOF_SSP_MASK_HDMI_CAPTURE(0x5)
                /* SSP 0 and SSP 2 are used for HDMI IN */
                | SOF_HDMI_PLAYBACK_PRESENT) as KernelUlongT
        },
    },
    platform_device_id {
        name: *b"nvl_lt6911_hdmi_ssp\0\0\0\0\0\0\0\0\0\0\0\0",
        driver_data: unsafe {
            (SOF_SSP_MASK_HDMI_CAPTURE(0x5)
                /* SSP 0 and SSP 2 are used for HDMI IN */
                | SOF_HDMI_PLAYBACK_PRESENT) as KernelUlongT
        },
    },
    platform_device_id {
        name: [0; 32],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(platform, board_ids);

static mut sof_ssp_amp_driver: platform_driver = platform_driver {
    probe: Some(sof_ssp_amp_probe),
    driver: device_driver {
        name: b"sof_ssp_amp\0".as_ptr() as *const c_char,
        pm: unsafe { &snd_soc_pm_ops },
    },
    id_table: board_ids.as_ptr(),
};
// module_platform_driver(sof_ssp_amp_driver);

// MODULE_DESCRIPTION("ASoC Intel(R) SOF Amplifier Machine driver");
// MODULE_AUTHOR("Balamurugan C <balamurugan.c@intel.com>");
// MODULE_AUTHOR("Brent Lu <brent.lu@intel.com>");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("SND_SOC_INTEL_SOF_BOARD_HELPERS");
// MODULE_IMPORT_NS("SND_SOC_INTEL_SOF_REALTEK_COMMON");
// MODULE_IMPORT_NS("SND_SOC_INTEL_SOF_CIRRUS_COMMON");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
