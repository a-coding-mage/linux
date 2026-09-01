// SPDX-License-Identifier: GPL-2.0-only
// This file incorporates work covered by the following copyright notice:
// Copyright (c) 2020 Intel Corporation
// Copyright (c) 2024 Advanced Micro Devices, Inc.
//
// soc_sdw_maxim - Helpers to handle maxim codecs
// codec devices from generic machine driver

use core::ffi::{c_char, c_int};

const EINVAL: c_int = 22;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;

static mut maxim_part_id: c_int = 0;
const SOC_SDW_PART_ID_MAX98363: c_int = 0x8363;
const SOC_SDW_PART_ID_MAX98373: c_int = 0x8373;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub name_prefix: *const c_char,
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dev: *mut device,
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub ops: *const snd_soc_ops,
}

#[repr(C)]
pub struct asoc_sdw_codec_info {
    pub amp_num: c_int,
    pub part_id: c_int,
    pub codec_card_late_probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream)>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut core::ffi::c_void) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
}

unsafe extern "C" {
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: c_int,
    ) -> c_int;
    fn snd_soc_dapm_enable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_sync(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_codec_dai_count(rtd: *mut snd_soc_pcm_runtime) -> c_int;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);

    fn asoc_sdw_startup(substream: *mut snd_pcm_substream) -> c_int;
    fn asoc_sdw_prepare(substream: *mut snd_pcm_substream) -> c_int;
    fn asoc_sdw_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int;
    fn asoc_sdw_hw_params(
        substream: *mut snd_pcm_substream,
        params: *mut core::ffi::c_void,
    ) -> c_int;
    fn asoc_sdw_hw_free(substream: *mut snd_pcm_substream) -> c_int;
    fn asoc_sdw_shutdown(substream: *mut snd_pcm_substream);
}

static max_98373_dapm_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: b"Left Spk\0".as_ptr() as *const c_char,
        control: core::ptr::null(),
        source: b"Left BE_OUT\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Right Spk\0".as_ptr() as *const c_char,
        control: core::ptr::null(),
        source: b"Right BE_OUT\0".as_ptr() as *const c_char,
    },
];

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_maxim_spk_rtd_init(
    rtd: *mut snd_soc_pcm_runtime,
    _dai: *mut snd_soc_dai,
) -> c_int {
    let card: *mut snd_soc_card = unsafe { (*rtd).card };
    let dapm: *mut snd_soc_dapm_context = unsafe { snd_soc_card_to_dapm(card) };
    let ret: c_int;

    ret = unsafe { snd_soc_dapm_add_routes(dapm, max_98373_dapm_routes.as_ptr(), 2) };
    if ret != 0 {
        unsafe {
            dev_err(
                (*rtd).dev,
                b"failed to add first SPK map: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
        }
    }

    ret
}
// EXPORT_SYMBOL_NS(asoc_sdw_maxim_spk_rtd_init, "SND_SOC_SDW_UTILS");

unsafe extern "C" fn asoc_sdw_mx8373_enable_spk_pin(
    substream: *mut snd_pcm_substream,
    enable: bool,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = unsafe { snd_soc_substream_to_rtd(substream) };
    let mut codec_dai: *mut snd_soc_dai;
    let cpu_dai: *mut snd_soc_dai;
    let mut ret: c_int;
    let mut j: c_int;

    /* set spk pin by playback only */
    if unsafe { (*substream).stream } == SNDRV_PCM_STREAM_CAPTURE {
        return 0;
    }

    cpu_dai = unsafe { snd_soc_rtd_to_cpu(rtd, 0) };
    j = 0;
    while j < unsafe { snd_soc_rtd_codec_dai_count(rtd) } {
        codec_dai = unsafe { snd_soc_rtd_to_codec(rtd, j) };
        let dapm: *mut snd_soc_dapm_context =
            unsafe { snd_soc_component_to_dapm((*cpu_dai).component) };
        let mut pin_name: [c_char; 16] = [0; 16];

        unsafe {
            snprintf(
                pin_name.as_mut_ptr(),
                pin_name.len(),
                b"%s Spk\0".as_ptr() as *const c_char,
                (*(*codec_dai).component).name_prefix,
            );
        }

        if enable {
            ret = unsafe { snd_soc_dapm_enable_pin(dapm, pin_name.as_ptr()) };
        } else {
            ret = unsafe { snd_soc_dapm_disable_pin(dapm, pin_name.as_ptr()) };
        }

        if ret == 0 {
            unsafe {
                snd_soc_dapm_sync(dapm);
            }
        }

        j += 1;
    }

    0
}

unsafe extern "C" fn asoc_sdw_mx8373_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let ret: c_int;

    /* according to soc_pcm_prepare dai link prepare is called first */
    ret = unsafe { asoc_sdw_prepare(substream) };
    if ret < 0 {
        return ret;
    }

    unsafe { asoc_sdw_mx8373_enable_spk_pin(substream, true) }
}

unsafe extern "C" fn asoc_sdw_mx8373_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let ret: c_int;

    /* according to soc_pcm_hw_free dai link free is called first */
    ret = unsafe { asoc_sdw_hw_free(substream) };
    if ret < 0 {
        return ret;
    }

    unsafe { asoc_sdw_mx8373_enable_spk_pin(substream, false) }
}

static max_98373_sdw_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(asoc_sdw_startup),
    prepare: Some(asoc_sdw_mx8373_prepare),
    trigger: Some(asoc_sdw_trigger),
    hw_params: Some(asoc_sdw_hw_params),
    hw_free: Some(asoc_sdw_mx8373_hw_free),
    shutdown: Some(asoc_sdw_shutdown),
};

unsafe extern "C" fn asoc_sdw_mx8373_sdw_late_probe(card: *mut snd_soc_card) -> c_int {
    let dapm: *mut snd_soc_dapm_context = unsafe { snd_soc_card_to_dapm(card) };

    /* Disable Left and Right Spk pin after boot */
    unsafe {
        snd_soc_dapm_disable_pin(dapm, b"Left Spk\0".as_ptr() as *const c_char);
        snd_soc_dapm_disable_pin(dapm, b"Right Spk\0".as_ptr() as *const c_char);
        snd_soc_dapm_sync(dapm)
    }
}

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_maxim_init(
    card: *mut snd_soc_card,
    dai_links: *mut snd_soc_dai_link,
    info: *mut asoc_sdw_codec_info,
    _playback: bool,
) -> c_int {
    unsafe {
        (*info).amp_num += 1;

        maxim_part_id = (*info).part_id;
        match maxim_part_id {
            SOC_SDW_PART_ID_MAX98363 => {
                /* Default ops are set in function init_dai_link.
                 * called as part of function create_sdw_dailink
                 */
            }
            SOC_SDW_PART_ID_MAX98373 => {
                (*info).codec_card_late_probe = Some(asoc_sdw_mx8373_sdw_late_probe);
                (*dai_links).ops = &max_98373_sdw_ops;
            }
            _ => {
                dev_err(
                    (*card).dev,
                    b"Invalid maxim_part_id %#x\n\0".as_ptr() as *const c_char,
                    maxim_part_id,
                );
                return -EINVAL;
            }
        }
    }
    0
}
// EXPORT_SYMBOL_NS(asoc_sdw_maxim_init, "SND_SOC_SDW_UTILS");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
