// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2019 Intel Corporation

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

// C dependencies removed from executable Rust:
// <linux/module.h>, <sound/pcm.h>, <sound/soc.h>, <sound/hda_codec.h>,
// <sound/hda_i915.h>, "../../codecs/hdac_hda.h", "hda_dsp_common.h"

// Original C conditional:
// #if IS_ENABLED(CONFIG_SND_SOC_SOF_HDA_AUDIO_CODEC)

extern "C" {
    static HDA_CODEC_IDX_CONTROLLER: c_int;
    static SNDRV_PCM_INVALID_DEVICE: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: usize;
    static EINVAL: c_int;

    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;

    fn snd_soc_component_get_drvdata(comp: *mut snd_soc_component) -> *mut c_void;
    fn snd_hdac_display_power(bus: *mut c_void, idx: c_int, enable: bool);
    fn snd_hda_codec_build_controls(hcodec: *mut hda_codec) -> c_int;

    fn dev_dbg(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);

    fn for_each_card_rtds(card: *mut snd_soc_card) -> ForEachCardRtds;
    fn list_for_each_hda_pcm(head: *mut list_head) -> ListForEachHdaPcm;
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut c_void,
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dai_link: *mut snd_soc_dai_link,
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub no_pcm: bool,
}

#[repr(C)]
pub struct snd_pcm {
    pub id: *const c_char,
    pub streams: [snd_pcm_str; 2],
    pub device: c_int,
}

#[repr(C)]
pub struct snd_pcm_str {
    pub substream: *mut c_void,
}

#[repr(C)]
pub struct hdac_hda_priv {
    pub codec: *mut hda_codec,
}

#[repr(C)]
pub struct hda_codec {
    pub core: hda_codec_core,
    pub pcm_list_head: list_head,
}

#[repr(C)]
pub struct hda_codec_core {
    pub bus: *mut c_void,
}

#[repr(C)]
pub struct hda_pcm {
    pub pcm: *mut snd_pcm,
    pub device: c_int,
    pub list: list_head,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

pub struct ForEachCardRtds;

impl Iterator for ForEachCardRtds {
    type Item = *mut snd_soc_pcm_runtime;

    fn next(&mut self) -> Option<Self::Item> {
        extern "C" {
            fn for_each_card_rtds_next(iter: *mut ForEachCardRtds) -> *mut snd_soc_pcm_runtime;
        }

        let rtd = unsafe { for_each_card_rtds_next(self) };
        if rtd.is_null() {
            None
        } else {
            Some(rtd)
        }
    }
}

pub struct ListForEachHdaPcm;

impl Iterator for ListForEachHdaPcm {
    type Item = *mut hda_pcm;

    fn next(&mut self) -> Option<Self::Item> {
        extern "C" {
            fn list_for_each_hda_pcm_next(iter: *mut ListForEachHdaPcm) -> *mut hda_pcm;
        }

        let hpcm = unsafe { list_for_each_hda_pcm_next(self) };
        if hpcm.is_null() {
            None
        } else {
            Some(hpcm)
        }
    }
}

/*
 * Search card topology and return PCM device number
 * matching Nth playback HDMI device (zero-based index).
 */
unsafe fn hda_dsp_hdmi_pcm_handle(
    card: *mut snd_soc_card,
    hdmi_idx: c_int,
) -> *mut snd_pcm {
    let mut i: c_int = 0;

    for rtd in for_each_card_rtds(card) {
        /* ignore BE PCMs */
        if !(*rtd).dai_link.is_null() && (*(*rtd).dai_link).no_pcm {
            continue;
        }

        let spcm = (*rtd).pcm;

        /* ignore PCMs with no playback streams */
        if spcm.is_null() || (*spcm).streams[SNDRV_PCM_STREAM_PLAYBACK].substream.is_null() {
            continue;
        }

        /* look for FE PCMs with name "HDMI x" */
        if !spcm.is_null() && !strstr((*spcm).id, b"HDMI\0".as_ptr() as *const c_char).is_null() {
            if i == hdmi_idx {
                return (*rtd).pcm;
            }
            i += 1;
        }
    }

    ptr::null_mut()
}

/*
 * Search card topology and register HDMI PCM related controls
 * to codec driver.
 */
#[no_mangle]
pub unsafe extern "C" fn hda_dsp_hdmi_build_controls(
    card: *mut snd_soc_card,
    comp: *mut snd_soc_component,
) -> c_int {
    let hda_pvt: *mut hdac_hda_priv;
    let hcodec: *mut hda_codec;
    let mut err: c_int = 0;
    let mut i: c_int = 0;

    if comp.is_null() {
        return -EINVAL;
    }

    hda_pvt = snd_soc_component_get_drvdata(comp) as *mut hdac_hda_priv;
    hcodec = (*hda_pvt).codec;

    for hpcm in list_for_each_hda_pcm(&mut (*hcodec).pcm_list_head) {
        let spcm = hda_dsp_hdmi_pcm_handle(card, i);
        if !spcm.is_null() {
            (*hpcm).pcm = spcm;
            (*hpcm).device = (*spcm).device;
            dev_dbg(
                (*card).dev,
                b"mapping HDMI converter %d to PCM %d (%p)\n\0".as_ptr() as *const c_char,
                i,
                (*hpcm).device,
                spcm,
            );
        } else {
            (*hpcm).pcm = ptr::null_mut();
            (*hpcm).device = SNDRV_PCM_INVALID_DEVICE;
            dev_warn(
                (*card).dev,
                b"%s: no PCM in topology for HDMI converter %d\n\0".as_ptr() as *const c_char,
                b"hda_dsp_hdmi_build_controls\0".as_ptr() as *const c_char,
                i,
            );
        }
        i += 1;
    }
    snd_hdac_display_power((*hcodec).core.bus, HDA_CODEC_IDX_CONTROLLER, true);
    err = snd_hda_codec_build_controls(hcodec);
    if err < 0 {
        dev_err(
            (*card).dev,
            b"unable to create controls %d\n\0".as_ptr() as *const c_char,
            err,
        );
    }
    snd_hdac_display_power((*hcodec).core.bus, HDA_CODEC_IDX_CONTROLLER, false);

    err
}

// EXPORT_SYMBOL_NS(hda_dsp_hdmi_build_controls, "SND_SOC_INTEL_HDA_DSP_COMMON");

// #endif

pub const MODULE_DESCRIPTION: &[u8] = b"ASoC Intel HDMI helpers\0";
pub const MODULE_LICENSE: &[u8] = b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
