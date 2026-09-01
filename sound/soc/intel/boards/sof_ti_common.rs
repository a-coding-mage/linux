// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2025 Intel Corporation

// Dependencies from the original C includes:
// linux/module.h, linux/string.h, sound/pcm.h, sound/pcm_params.h,
// sound/soc.h, sound/soc-acpi.h, sound/soc-dai.h, sound/soc-dapm.h,
// sound/sof.h, uapi/sound/asound.h, ../common/soc-intel-quirks.h,
// and sof_ti_common.h.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const TAS2563_DEV0_NAME: *const c_char = b"TAS2563_DEV0\0".as_ptr() as *const c_char;
const TAS2563_CODEC_DAI: *const c_char = b"tas2563-codec\0".as_ptr() as *const c_char;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
}

unsafe extern "C" {
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_new_controls(
        dapm: *mut snd_soc_dapm_context,
        widget: *const snd_soc_dapm_widget,
        num: c_int,
    ) -> c_int;
    fn snd_soc_add_card_controls(
        card: *mut snd_soc_card,
        controls: *const snd_kcontrol_new,
        num_controls: c_int,
    ) -> c_int;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

/*
 * Texas Instruments TAS2563 just mount one device to manage multiple devices,
 * so the kcontrols, widgets and routes just keep one item, respectively.
 */
static TAS2563_SPK_KCONTROLS: [snd_kcontrol_new; 1] = [
    // SOC_DAPM_PIN_SWITCH("Spk")
    snd_kcontrol_new { _private: [] },
];

static TAS2563_SPK_DAPM_WIDGETS: [snd_soc_dapm_widget; 1] = [
    // SND_SOC_DAPM_SPK("Spk", NULL)
    snd_soc_dapm_widget { _private: [] },
];

static TAS2563_SPK_DAPM_ROUTES: [snd_soc_dapm_route; 1] = [snd_soc_dapm_route {
    sink: b"Spk\0".as_ptr() as *const c_char,
    control: ptr::null(),
    source: b"OUT\0".as_ptr() as *const c_char,
}];

static mut TAS2563_DAI_LINK_COMPONENTS: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component {
        name: TAS2563_DEV0_NAME,
        dai_name: TAS2563_CODEC_DAI,
    }];

unsafe extern "C" fn tas2563_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card: *mut snd_soc_card = unsafe { (*rtd).card };
    let dapm: *mut snd_soc_dapm_context = unsafe { snd_soc_card_to_dapm(card) };
    let mut ret: c_int;

    ret = unsafe {
        snd_soc_dapm_new_controls(
            dapm,
            TAS2563_SPK_DAPM_WIDGETS.as_ptr(),
            TAS2563_SPK_DAPM_WIDGETS.len() as c_int,
        )
    };
    if ret != 0 {
        unsafe {
            dev_err(
                (*rtd).dev,
                b"unable to add dapm widgets, ret %d\n\0".as_ptr() as *const c_char,
                ret,
            );
        }
        return ret;
    }

    ret = unsafe {
        snd_soc_add_card_controls(
            card,
            TAS2563_SPK_KCONTROLS.as_ptr(),
            TAS2563_SPK_KCONTROLS.len() as c_int,
        )
    };
    if ret != 0 {
        unsafe {
            dev_err(
                (*rtd).dev,
                b"unable to add controls, ret %d\n\0".as_ptr() as *const c_char,
                ret,
            );
        }
        return ret;
    }

    ret = unsafe {
        snd_soc_dapm_add_routes(
            dapm,
            TAS2563_SPK_DAPM_ROUTES.as_ptr(),
            TAS2563_SPK_DAPM_ROUTES.len() as c_int,
        )
    };
    if ret != 0 {
        unsafe {
            dev_err(
                (*rtd).dev,
                b"unable to add dapm routes, ret %d\n\0".as_ptr() as *const c_char,
                ret,
            );
        }
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sof_tas2563_dai_link(link: *mut snd_soc_dai_link) {
    unsafe {
        (*link).codecs = core::ptr::addr_of_mut!(TAS2563_DAI_LINK_COMPONENTS) as *mut _;
        (*link).num_codecs = TAS2563_DAI_LINK_COMPONENTS.len() as c_uint;
        (*link).init = Some(tas2563_init);
    }
}

// EXPORT_SYMBOL_NS(sof_tas2563_dai_link, "SND_SOC_INTEL_SOF_TI_COMMON");

// MODULE_DESCRIPTION("ASoC Intel SOF Texas Instruments helpers");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
