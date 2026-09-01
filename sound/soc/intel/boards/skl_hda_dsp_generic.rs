// SPDX-License-Identifier: GPL-2.0-only
// Copyright(c) 2015-18 Intel Corporation.

/*
 * Machine Driver for SKL+ platforms with DSP and iDisp, HDA Codecs
 */

/* C include dependencies:
 * linux/module.h, linux/platform_device.h, sound/core.h, sound/hda_codec.h,
 * sound/jack.h, sound/pcm.h, sound/pcm_params.h, sound/soc.h,
 * sound/soc-acpi.h, ../../codecs/hdac_hda.h, ../../sof/intel/hda.h,
 * sof_board_helpers.h
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut module,
    pub fully_routed: bool,
    pub late_probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub add_dai_link:
        Option<unsafe extern "C" fn(*mut snd_soc_card, *mut snd_soc_dai_link) -> c_int>,
    pub dev: *mut device,
    pub components: *mut c_char,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dai_link: *mut snd_soc_dai_link,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub codecs: *mut snd_soc_dai_link_component,
    pub stream_name: *const c_char,
    pub ignore: bool,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hdac_hda_priv {
    pub codec: *mut hda_codec,
}

#[repr(C)]
pub struct hda_codec {
    pub bus: *mut hda_bus,
}

#[repr(C)]
pub struct hda_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_acpi_mach_params {
    pub bt_link_mask: c_ulong,
    pub codec_mask: c_ulong,
    pub dmic_num: c_int,
    pub platform: *const c_char,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub mach_params: snd_soc_acpi_mach_params,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct device {
    pub platform_data: *mut c_void,
}

#[repr(C)]
pub struct sof_card_private {
    pub hdmi: sof_hdmi_private,
    pub hda_codec_present: bool,
    pub link_order_overwrite: c_ulong,
    pub link_id_overwrite: c_ulong,
}

#[repr(C)]
pub struct sof_hdmi_private {
    pub idisp_codec: bool,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;
    static snd_soc_pm_ops: dev_pm_ops;

    fn sof_intel_board_card_late_probe(card: *mut snd_soc_card) -> c_int;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_hda_set_power_save(bus: *mut hda_bus, delay: c_int);
    fn hweight_long(w: c_ulong) -> c_int;
    fn fls(x: c_ulong) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn sof_intel_board_get_ctx(dev: *mut device, board_quirk: c_ulong) -> *mut sof_card_private;
    fn sof_intel_board_set_dai_link(
        dev: *mut device,
        card: *mut snd_soc_card,
        ctx: *mut sof_card_private,
    ) -> c_int;
    fn devm_kasprintf(dev: *mut device, flags: c_int, fmt: *const c_char, ...) -> *mut c_char;
    fn snd_soc_fixup_dai_links_platform_name(
        card: *mut snd_soc_card,
        platform: *const c_char,
    ) -> c_int;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;

    /* Rust translation of for_each_card_rtds(card, rtd) depends on external
     * kernel list layout supplied by sound/soc.h.
     */
    fn for_each_card_rtds_next(
        card: *mut snd_soc_card,
        previous: *mut snd_soc_pcm_runtime,
    ) -> *mut snd_soc_pcm_runtime;

    fn __platform_driver_register(driver: *mut platform_driver, owner: *mut module) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);
}

const GFP_KERNEL: c_int = 0;
const ENOMEM: c_int = 12;

const HDA_CODEC_AUTOSUSPEND_DELAY_MS: c_int = 1000;

const IDISP_HDMI_BE_ID: c_ulong = 1;
const HDA_BE_ID: c_ulong = 4;
const DMIC01_BE_ID: c_ulong = 6;
const DMIC16K_BE_ID: c_ulong = 7;
const BT_OFFLOAD_BE_ID: c_ulong = 8;

const SOF_LINK_IDISP_HDMI: c_ulong = 0;
const SOF_LINK_HDA: c_ulong = 1;
const SOF_LINK_DMIC01: c_ulong = 2;
const SOF_LINK_DMIC16K: c_ulong = 3;
const SOF_LINK_BT_OFFLOAD: c_ulong = 4;
const SOF_LINK_NONE: c_ulong = 0xff;

const SOF_BT_OFFLOAD_PRESENT: c_ulong = 1 << 0;
const IDISP_CODEC_MASK: c_ulong = 1 << 2;

const fn sof_link_order(
    a: c_ulong,
    b: c_ulong,
    c: c_ulong,
    d: c_ulong,
    e: c_ulong,
    f: c_ulong,
    g: c_ulong,
) -> c_ulong {
    a | (b << 8) | (c << 16) | (d << 24) | (e << 32) | (f << 40) | (g << 48)
}

const fn sof_ssp_port_bt_offload(ssp: c_int) -> c_ulong {
    (ssp as c_ulong) << 8
}

const fn hda_ext_codec(codec_mask: c_ulong) -> bool {
    codec_mask != 0
}

const HDA_LINK_ORDER: c_ulong = sof_link_order(
    SOF_LINK_IDISP_HDMI,
    SOF_LINK_HDA,
    SOF_LINK_DMIC01,
    SOF_LINK_DMIC16K,
    SOF_LINK_BT_OFFLOAD,
    SOF_LINK_NONE,
    SOF_LINK_NONE,
);

const HDA_LINK_IDS: c_ulong = sof_link_order(
    IDISP_HDMI_BE_ID,
    HDA_BE_ID,
    DMIC01_BE_ID,
    DMIC16K_BE_ID,
    BT_OFFLOAD_BE_ID,
    0,
    0,
);

const EHDAUDIO0D0: &[u8] = b"ehdaudio0D0\0";
const HDMI: &[u8] = b"HDMI\0";
const CARD_NAME: &[u8] = b"hda-dsp\0";
const DRIVER_NAME: &[u8] = b"skl_hda_dsp_generic\0";
const BOARD_QUIRK_FMT: &[u8] = b"board_quirk = %lx\n\0";
const CFG_DMICS_FMT: &[u8] = b"cfg-dmics:%d\0";

unsafe extern "C" fn skl_hda_card_late_probe(card: *mut snd_soc_card) -> c_int {
    unsafe { sof_intel_board_card_late_probe(card) }
}

unsafe fn skl_set_hda_codec_autosuspend_delay(card: *mut snd_soc_card) {
    let mut rtd: *mut snd_soc_pcm_runtime = ptr::null_mut();

    loop {
        rtd = unsafe { for_each_card_rtds_next(card, rtd) };
        if rtd.is_null() {
            break;
        }

        if unsafe {
            strstr(
                (*(*rtd).dai_link).codecs.as_ref().unwrap().name,
                EHDAUDIO0D0.as_ptr() as *const c_char,
            )
        }
        .is_null()
        {
            continue;
        }

        let dai = unsafe { snd_soc_rtd_to_codec(rtd, 0) };
        let hda_pvt =
            unsafe { snd_soc_component_get_drvdata((*dai).component) as *mut hdac_hda_priv };
        if !hda_pvt.is_null() {
            /*
             * all codecs are on the same bus, so it's sufficient
             * to look up only the first one
             */
            unsafe {
                snd_hda_set_power_save(
                    (*(*hda_pvt).codec).bus,
                    HDA_CODEC_AUTOSUSPEND_DELAY_MS,
                );
            }
            break;
        }
    }
}

unsafe fn skl_hda_get_board_quirk(mach_params: *mut snd_soc_acpi_mach_params) -> c_ulong {
    let mut board_quirk: c_ulong = 0;
    let ssp_bt: c_int;

    if unsafe { hweight_long((*mach_params).bt_link_mask) } == 1 {
        ssp_bt = unsafe { fls((*mach_params).bt_link_mask) - 1 };
        board_quirk |= sof_ssp_port_bt_offload(ssp_bt) | SOF_BT_OFFLOAD_PRESENT;
    }

    board_quirk
}

unsafe extern "C" fn skl_hda_add_dai_link(
    card: *mut snd_soc_card,
    link: *mut snd_soc_dai_link,
) -> c_int {
    let ctx = unsafe { snd_soc_card_get_drvdata(card) as *mut sof_card_private };

    /* Ignore the HDMI PCM link if iDisp is not present */
    if unsafe { !strstr((*link).stream_name, HDMI.as_ptr() as *const c_char).is_null() }
        && unsafe { !(*ctx).hdmi.idisp_codec }
    {
        unsafe {
            (*link).ignore = true;
        }
    }

    0
}

unsafe extern "C" fn skl_hda_audio_probe(pdev: *mut platform_device) -> c_int {
    let mach = unsafe { (*pdev).dev.platform_data as *mut snd_soc_acpi_mach };
    let ctx: *mut sof_card_private;
    let card: *mut snd_soc_card;
    let board_quirk =
        unsafe { skl_hda_get_board_quirk(&mut (*mach).mach_params as *mut snd_soc_acpi_mach_params) };
    let mut ret: c_int;

    card = unsafe {
        devm_kzalloc(
            &mut (*pdev).dev as *mut device,
            size_of::<snd_soc_card>(),
            GFP_KERNEL,
        ) as *mut snd_soc_card
    };
    if card.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*card).name = CARD_NAME.as_ptr() as *const c_char;
        (*card).owner = THIS_MODULE;
        (*card).fully_routed = true;
        (*card).late_probe = Some(skl_hda_card_late_probe);
        (*card).add_dai_link = Some(skl_hda_add_dai_link);
    }

    unsafe {
        dev_dbg(
            &mut (*pdev).dev as *mut device,
            BOARD_QUIRK_FMT.as_ptr() as *const c_char,
            board_quirk,
        );
    }

    /* initialize ctx with board quirk */
    ctx = unsafe { sof_intel_board_get_ctx(&mut (*pdev).dev as *mut device, board_quirk) };
    if ctx.is_null() {
        return -ENOMEM;
    }

    if unsafe { hda_ext_codec((*mach).mach_params.codec_mask) } {
        unsafe {
            (*ctx).hda_codec_present = true;
        }
    }

    if unsafe { ((*mach).mach_params.codec_mask & IDISP_CODEC_MASK) != 0 } {
        unsafe {
            (*ctx).hdmi.idisp_codec = true;
        }
    }

    unsafe {
        (*ctx).link_order_overwrite = HDA_LINK_ORDER;
        (*ctx).link_id_overwrite = HDA_LINK_IDS;
    }

    /* update dai_link */
    ret = unsafe { sof_intel_board_set_dai_link(&mut (*pdev).dev as *mut device, card, ctx) };
    if ret != 0 {
        return ret;
    }

    unsafe {
        (*card).dev = &mut (*pdev).dev as *mut device;
    }

    if unsafe { (*mach).mach_params.dmic_num > 0 } {
        unsafe {
            (*card).components = devm_kasprintf(
                (*card).dev,
                GFP_KERNEL,
                CFG_DMICS_FMT.as_ptr() as *const c_char,
                (*mach).mach_params.dmic_num,
            );
        }
        if unsafe { (*card).components.is_null() } {
            return -ENOMEM;
        }
    }

    ret = unsafe {
        snd_soc_fixup_dai_links_platform_name(card, (*mach).mach_params.platform)
    };
    if ret != 0 {
        return ret;
    }

    unsafe {
        snd_soc_card_set_drvdata(card, ctx as *mut c_void);
    }

    ret = unsafe { devm_snd_soc_register_card(&mut (*pdev).dev as *mut device, card) };
    if ret == 0 {
        unsafe {
            skl_set_hda_codec_autosuspend_delay(card);
        }
    }

    ret
}

#[unsafe(no_mangle)]
pub static mut skl_hda_audio: platform_driver = platform_driver {
    probe: Some(skl_hda_audio_probe),
    driver: device_driver {
        name: DRIVER_NAME.as_ptr() as *const c_char,
        pm: unsafe { &snd_soc_pm_ops as *const dev_pm_ops },
    },
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_module() -> c_int {
    unsafe { __platform_driver_register(&mut skl_hda_audio as *mut platform_driver, THIS_MODULE) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cleanup_module() {
    unsafe {
        platform_driver_unregister(&mut skl_hda_audio as *mut platform_driver);
    }
}

/* Module information */
/* MODULE_DESCRIPTION("SKL/KBL/BXT/APL HDA Generic Machine driver"); */
/* MODULE_AUTHOR("Rakesh Ughreja <rakesh.a.ughreja@intel.com>"); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_ALIAS("platform:skl_hda_dsp_generic"); */
/* MODULE_IMPORT_NS("SND_SOC_INTEL_SOF_BOARD_HELPERS"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
