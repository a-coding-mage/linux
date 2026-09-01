// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Authors: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//	    Ranjani Sridharan <ranjani.sridharan@linux.intel.com>
//	    Rander Wang <rander.wang@intel.com>
//          Keyon Jie <yang.jie@linux.intel.com>
//

/*
 * Hardware interface for generic Intel audio DSP HDA IP
 */

// C dependencies:
// <linux/moduleparam.h>
// <sound/hda_register.h>
// <sound/pcm_params.h>
// <trace/events/sof_intel.h>
// "../sof-audio.h"
// "../ops.h"
// "hda.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

pub type u32 = u32;
pub type ssize_t = isize;
pub type snd_pcm_uframes_t = usize;

const fn SDnFMT_BASE(x: u32) -> u32 {
    x << 14
}

const fn SDnFMT_MULT(x: u32) -> u32 {
    (x.wrapping_sub(1)) << 11
}

const fn SDnFMT_DIV(x: u32) -> u32 {
    (x.wrapping_sub(1)) << 8
}

const fn SDnFMT_BITS(x: u32) -> u32 {
    x << 4
}

const fn SDnFMT_CHAN(x: u32) -> u32 {
    x << 0
}

const HDA_MAX_PERIOD_TIME_HEADROOM: c_uint = 10;

static mut hda_always_enable_dmi_l1: bool = false;
// module_param_named(always_enable_dmi_l1, hda_always_enable_dmi_l1, bool, 0444);
// MODULE_PARM_DESC(always_enable_dmi_l1, "SOF HDA always enable DMI l1");

static mut hda_disable_rewinds: bool = false;
// module_param_named(disable_rewinds, hda_disable_rewinds, bool, 0444);
// MODULE_PARM_DESC(disable_rewinds, "SOF HDA disable rewinds");

static mut hda_force_pause_support: c_int = -1;
// module_param_named(force_pause_support, hda_force_pause_support, int, 0444);
// MODULE_PARM_DESC(force_pause_support,
//		 "Pause support: -1: Use default, 0: Disable, 1: Enable (default -1)");

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub hw_pdata: *mut sof_intel_hda_dev,
}

#[repr(C)]
pub struct snd_sof_dev {
    pub dev: *mut device,
    pub pdata: *mut snd_sof_pdata,
    pub dspless_mode_selected: bool,
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub id: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dai_link: *mut snd_soc_dai_link,
}

#[repr(C)]
pub struct snd_pcm_control {
    pub appl_ptr: snd_pcm_uframes_t,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: u32,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub private_data: *mut hdac_stream,
    pub dma_buffer_p: *mut snd_dma_buffer,
    pub control: *mut snd_pcm_control,
    pub buffer_size: snd_pcm_uframes_t,
    pub hw: snd_pcm_hardware,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: c_int,
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    pub info: u32,
    pub flags: u32,
}

#[repr(C)]
pub struct snd_sof_platform_stream_params {
    pub no_ipc_position: bool,
    pub stream_tag: c_int,
}

#[repr(C)]
pub struct hdac_stream {
    pub substream: *mut snd_pcm_substream,
    pub format_val: u32,
    pub bufsize: usize,
    pub period_bytes: usize,
    pub no_period_wakeup: bool,
    pub stream_tag: c_int,
    pub spib_addr: u32,
}

#[repr(C)]
pub struct hdac_ext_stream {
    pub hstream: hdac_stream,
    pub pplcllpl: u32,
    pub pplcllpu: u32,
}

#[repr(C)]
pub struct sof_intel_hda_dev {
    pub no_ipc_position: bool,
}

#[repr(C)]
pub struct sof_intel_dsp_desc {
    pub hw_ip_version: c_uint,
}

#[repr(C)]
pub struct snd_dma_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_host_posn {
    pub host_posn: snd_pcm_uframes_t,
}

#[repr(C)]
pub struct snd_sof_pcm_stream {
    pub posn: snd_sof_host_posn,
    pub pause_supported: bool,
    pub d0i3_compatible: bool,
    pub dsp_max_burst_size_in_ms: c_uint,
}

#[repr(C)]
pub struct snd_sof_pcm {
    pub stream: [snd_sof_pcm_stream; 2],
}

unsafe extern "C" {
    static SNDRV_PCM_INFO_NO_PERIOD_WAKEUP: u32;
    static SNDRV_PCM_HW_PARAMS_NO_PERIOD_WAKEUP: u32;
    static SNDRV_PCM_INFO_NO_REWINDS: u32;
    static SNDRV_PCM_INFO_SYNC_APPLPTR: u32;
    static SNDRV_PCM_INFO_PAUSE: u32;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SOF_HDA_STREAM_DMI_L1_COMPATIBLE: u32;
    static SOF_INTEL_ACE_4_0: c_uint;
    static SNDRV_PCM_HW_PARAM_PERIOD_BYTES: c_int;
    static SNDRV_PCM_HW_PARAM_PERIODS: c_int;
    static SNDRV_PCM_HW_PARAM_FORMAT: c_int;
    static SNDRV_PCM_HW_PARAM_PERIOD_TIME: c_int;
    static HDA_DSP_MAX_BDL_ENTRIES: c_uint;
    static SNDRV_PCM_FMTBIT_S16: u64;
    static SNDRV_PCM_FMTBIT_S32: u64;
    static HDA_DSP_SPIB_ENABLE: c_int;
    static HDA_DSP_SPIB_DISABLE: c_int;
    static USEC_PER_MSEC: c_uint;
    static UINT_MAX: c_uint;

    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn_ratelimited(dev: *mut device, fmt: *const c_char, ...);

    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_buffer_bytes(params: *mut snd_pcm_hw_params) -> usize;
    fn params_period_bytes(params: *mut snd_pcm_hw_params) -> usize;

    fn stream_to_hdac_ext_stream(hstream: *mut hdac_stream) -> *mut hdac_ext_stream;
    fn hda_dsp_stream_hw_params(
        sdev: *mut snd_sof_dev,
        hext_stream: *mut hdac_ext_stream,
        dmab: *mut snd_dma_buffer,
        params: *mut snd_pcm_hw_params,
    ) -> c_int;
    fn hda_dsp_stream_spib_config(
        sdev: *mut snd_sof_dev,
        hext_stream: *mut hdac_ext_stream,
        enable: c_int,
        value: c_int,
    );
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, frames: snd_pcm_uframes_t) -> ssize_t;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: snd_pcm_uframes_t)
        -> snd_pcm_uframes_t;
    fn sof_io_write(sdev: *mut snd_sof_dev, addr: u32, value: u32);
    fn hda_dsp_stream_trigger(
        sdev: *mut snd_sof_dev,
        hext_stream: *mut hdac_ext_stream,
        cmd: c_int,
    ) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_sof_find_spcm_dai(
        scomp: *mut snd_soc_component,
        rtd: *mut snd_soc_pcm_runtime,
    ) -> *mut snd_sof_pcm;
    fn hda_dsp_stream_get_position(
        hstream: *mut hdac_stream,
        stream: c_int,
        can_sleep: bool,
    ) -> snd_pcm_uframes_t;
    fn trace_sof_intel_hda_dsp_pcm(
        sdev: *mut snd_sof_dev,
        hstream: *mut hdac_stream,
        substream: *mut snd_pcm_substream,
        pos: snd_pcm_uframes_t,
    );
    fn get_chip_info(pdata: *mut snd_sof_pdata) -> *const sof_intel_dsp_desc;
    fn hda_dsp_stream_get(
        sdev: *mut snd_sof_dev,
        direction: c_int,
        flags: u32,
    ) -> *mut hdac_ext_stream;
    fn snd_pcm_hw_constraint_step(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        step: c_uint,
    ) -> c_int;
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, var: c_int) -> c_int;
    fn snd_pcm_hw_constraint_minmax(
        runtime: *mut snd_pcm_runtime,
        var: c_int,
        min: c_uint,
        max: c_uint,
    ) -> c_int;
    fn snd_pcm_hw_constraint_mask64(
        runtime: *mut snd_pcm_runtime,
        var: c_int,
        mask: u64,
    ) -> c_int;
    fn hda_dsp_stream_put(
        sdev: *mut snd_sof_dev,
        direction: c_int,
        stream_tag: c_int,
    ) -> c_int;
}

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_get_mult_div(
    sdev: *mut snd_sof_dev,
    rate: c_int,
) -> u32 {
    match rate {
        8000 => SDnFMT_DIV(6),
        9600 => SDnFMT_DIV(5),
        11025 => SDnFMT_BASE(1) | SDnFMT_DIV(4),
        16000 => SDnFMT_DIV(3),
        22050 => SDnFMT_BASE(1) | SDnFMT_DIV(2),
        32000 => SDnFMT_DIV(3) | SDnFMT_MULT(2),
        44100 => SDnFMT_BASE(1),
        48000 => 0,
        88200 => SDnFMT_BASE(1) | SDnFMT_MULT(2),
        96000 => SDnFMT_MULT(2),
        176400 => SDnFMT_BASE(1) | SDnFMT_MULT(4),
        192000 => SDnFMT_MULT(4),
        _ => {
            dev_warn(
                (*sdev).dev,
                b"can't find div rate %d using 48kHz\n\0".as_ptr() as *const c_char,
                rate,
            );
            0 /* use 48KHz if not found */
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_get_bits(
    sdev: *mut snd_sof_dev,
    sample_bits: c_int,
) -> u32 {
    match sample_bits {
        8 => SDnFMT_BITS(0),
        16 => SDnFMT_BITS(1),
        20 => SDnFMT_BITS(2),
        24 => SDnFMT_BITS(3),
        32 => SDnFMT_BITS(4),
        _ => {
            dev_warn(
                (*sdev).dev,
                b"can't find %d bits using 16bit\n\0".as_ptr() as *const c_char,
                sample_bits,
            );
            SDnFMT_BITS(1) /* use 16bits format if not found */
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_pcm_hw_params(
    sdev: *mut snd_sof_dev,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    platform_params: *mut snd_sof_platform_stream_params,
) -> c_int {
    let hstream = (*(*substream).runtime).private_data;
    let hext_stream = stream_to_hdac_ext_stream(hstream);
    let hda = (*(*sdev).pdata).hw_pdata;
    let dmab: *mut snd_dma_buffer;
    let ret: c_int;

    (*hstream).substream = substream;

    dmab = (*(*substream).runtime).dma_buffer_p;

    /*
     * Use the codec required format val (which is link_bps adjusted) when
     * the DSP is not in use
     */
    if !(*sdev).dspless_mode_selected {
        let rate: u32 = hda_dsp_get_mult_div(sdev, params_rate(params));
        let bits: u32 = hda_dsp_get_bits(sdev, params_width(params));

        (*hstream).format_val = rate | bits | (params_channels(params) - 1) as u32;
    }

    (*hstream).bufsize = params_buffer_bytes(params);
    (*hstream).period_bytes = params_period_bytes(params);
    (*hstream).no_period_wakeup =
        ((*params).info & SNDRV_PCM_INFO_NO_PERIOD_WAKEUP) != 0
            && ((*params).flags & SNDRV_PCM_HW_PARAMS_NO_PERIOD_WAKEUP) != 0;

    ret = hda_dsp_stream_hw_params(sdev, hext_stream, dmab, params);
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            b"error: hdac prepare failed: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    /* enable SPIB when rewinds are disabled */
    if hda_disable_rewinds {
        hda_dsp_stream_spib_config(sdev, hext_stream, HDA_DSP_SPIB_ENABLE, 0);
    } else {
        hda_dsp_stream_spib_config(sdev, hext_stream, HDA_DSP_SPIB_DISABLE, 0);
    }

    if !hda.is_null() {
        (*platform_params).no_ipc_position = (*hda).no_ipc_position;
    }

    (*platform_params).stream_tag = (*hstream).stream_tag;

    0
}
// EXPORT_SYMBOL_NS(hda_dsp_pcm_hw_params, "SND_SOC_SOF_INTEL_HDA_COMMON");

/* update SPIB register with appl position */
#[no_mangle]
pub unsafe extern "C" fn hda_dsp_pcm_ack(
    sdev: *mut snd_sof_dev,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let hstream = (*(*substream).runtime).private_data;
    let runtime = (*substream).runtime;
    let appl_pos: ssize_t;
    let buf_size: ssize_t;
    let mut spib: u32;

    appl_pos = frames_to_bytes(runtime, (*(*runtime).control).appl_ptr);
    buf_size = frames_to_bytes(runtime, (*runtime).buffer_size);

    spib = (appl_pos % buf_size) as u32;

    /* Allowable value for SPIB is 1 byte to max buffer size */
    if spib == 0 {
        spib = buf_size as u32;
    }

    sof_io_write(sdev, (*hstream).spib_addr, spib);

    0
}
// EXPORT_SYMBOL_NS(hda_dsp_pcm_ack, "SND_SOC_SOF_INTEL_HDA_COMMON");

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_pcm_trigger(
    sdev: *mut snd_sof_dev,
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let hstream = (*(*substream).runtime).private_data;
    let hext_stream = stream_to_hdac_ext_stream(hstream);

    hda_dsp_stream_trigger(sdev, hext_stream, cmd)
}
// EXPORT_SYMBOL_NS(hda_dsp_pcm_trigger, "SND_SOC_SOF_INTEL_HDA_COMMON");

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_pcm_pointer(
    sdev: *mut snd_sof_dev,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let rtd = snd_soc_substream_to_rtd(substream);
    let scomp = (*sdev).component;
    let hstream = (*(*substream).runtime).private_data;
    let hda = (*(*sdev).pdata).hw_pdata;
    let spcm: *mut snd_sof_pcm;
    let mut pos: snd_pcm_uframes_t;

    spcm = snd_sof_find_spcm_dai(scomp, rtd);
    if spcm.is_null() {
        dev_warn_ratelimited(
            (*sdev).dev,
            b"warn: can't find PCM with DAI ID %d\n\0".as_ptr() as *const c_char,
            (*(*rtd).dai_link).id,
        );
        return 0;
    }

    if !hda.is_null() && !(*hda).no_ipc_position {
        /* read position from IPC position */
        pos = (*spcm).stream[(*substream).stream as usize].posn.host_posn;
    } else {
        pos = hda_dsp_stream_get_position(hstream, (*substream).stream, true);
    }

    pos = bytes_to_frames((*substream).runtime, pos);

    trace_sof_intel_hda_dsp_pcm(sdev, hstream, substream, pos);
    pos
}
// EXPORT_SYMBOL_NS(hda_dsp_pcm_pointer, "SND_SOC_SOF_INTEL_HDA_COMMON");

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_pcm_open(
    sdev: *mut snd_sof_dev,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let chip_info = get_chip_info((*sdev).pdata);
    let rtd = snd_soc_substream_to_rtd(substream);
    let runtime = (*substream).runtime;
    let scomp = (*sdev).component;
    let dsp_stream: *mut hdac_ext_stream;
    let spcm: *mut snd_sof_pcm;
    let direction: c_int = (*substream).stream;
    let mut flags: u32 = 0;

    spcm = snd_sof_find_spcm_dai(scomp, rtd);
    if spcm.is_null() {
        dev_err(
            (*sdev).dev,
            b"error: can't find PCM with DAI ID %d\n\0".as_ptr() as *const c_char,
            (*(*rtd).dai_link).id,
        );
        return -EINVAL;
    }

    /*
     * if we want the .ack to work, we need to prevent the control from being mapped.
     * The status can still be mapped.
     */
    if hda_disable_rewinds {
        (*runtime).hw.info |= SNDRV_PCM_INFO_NO_REWINDS | SNDRV_PCM_INFO_SYNC_APPLPTR;
    }

    /*
     * All playback streams are DMI L1 capable, capture streams need
     * pause push/release to be disabled
     */
    if hda_always_enable_dmi_l1 && direction == SNDRV_PCM_STREAM_CAPTURE {
        (*runtime).hw.info &= !SNDRV_PCM_INFO_PAUSE;
    }

    /*
     * Do not advertise the PAUSE support if it is forced to be disabled via
     * module parameter or if the pause_supported is false for the PCM
     * device
     */
    if hda_force_pause_support == 0
        || (hda_force_pause_support == -1
            && !(*spcm).stream[(*substream).stream as usize].pause_supported)
    {
        (*runtime).hw.info &= !SNDRV_PCM_INFO_PAUSE;
    }

    if hda_always_enable_dmi_l1
        || direction == SNDRV_PCM_STREAM_PLAYBACK
        || (*spcm).stream[(*substream).stream as usize].d0i3_compatible
    {
        flags |= SOF_HDA_STREAM_DMI_L1_COMPATIBLE;
    }

    dsp_stream = hda_dsp_stream_get(sdev, direction, flags);
    if dsp_stream.is_null() {
        dev_err(
            (*sdev).dev,
            b"error: no stream available\n\0".as_ptr() as *const c_char,
        );
        return -ENODEV;
    }

    /*
     * Set period size constraint to ensure BDLE buffer length and
     * start address alignment requirements are met. Align to 128
     * bytes for newer Intel platforms, with older ones using 4 byte alignment.
     */
    if (*chip_info).hw_ip_version >= SOF_INTEL_ACE_4_0 {
        snd_pcm_hw_constraint_step(
            (*substream).runtime,
            0,
            SNDRV_PCM_HW_PARAM_PERIOD_BYTES,
            128,
        );
    } else {
        snd_pcm_hw_constraint_step(
            (*substream).runtime,
            0,
            SNDRV_PCM_HW_PARAM_PERIOD_BYTES,
            4,
        );
    }

    /* avoid circular buffer wrap in middle of period */
    snd_pcm_hw_constraint_integer((*substream).runtime, SNDRV_PCM_HW_PARAM_PERIODS);

    /* Limit the maximum number of periods to not exceed the BDL entries count */
    if (*runtime).hw.periods_max > HDA_DSP_MAX_BDL_ENTRIES {
        snd_pcm_hw_constraint_minmax(
            runtime,
            SNDRV_PCM_HW_PARAM_PERIODS,
            (*runtime).hw.periods_min,
            HDA_DSP_MAX_BDL_ENTRIES,
        );
    }

    /* Only S16 and S32 supported by HDA hardware when used without DSP */
    if (*sdev).dspless_mode_selected {
        snd_pcm_hw_constraint_mask64(
            (*substream).runtime,
            SNDRV_PCM_HW_PARAM_FORMAT,
            SNDRV_PCM_FMTBIT_S16 | SNDRV_PCM_FMTBIT_S32,
        );
    }

    /*
     * The dsp_max_burst_size_in_ms is the length of the maximum burst size
     * of the host DMA in the ALSA buffer.
     *
     * On playback start the DMA will transfer dsp_max_burst_size_in_ms
     * amount of data in one initial burst to fill up the host DMA buffer.
     * Consequent DMA burst sizes are shorter and their length can vary.
     * To avoid immediate xrun by the initial burst we need to place
     * constraint on the period size (via PERIOD_TIME) to cover the size of
     * the host buffer.
     * We need to add headroom of max 10ms as the firmware needs time to
     * settle to the 1ms pacing and initially it can run faster for few
     * internal periods.
     *
     * On capture the DMA will transfer 1ms chunks.
     */
    if (*spcm).stream[direction as usize].dsp_max_burst_size_in_ms != 0 {
        let mut period_time: c_uint =
            (*spcm).stream[direction as usize].dsp_max_burst_size_in_ms;

        /*
         * add headroom over the maximum burst size to cover the time
         * needed for the DMA pace to settle.
         * Limit the headroom time to HDA_MAX_PERIOD_TIME_HEADROOM
         */
        period_time += core::cmp::min(period_time, HDA_MAX_PERIOD_TIME_HEADROOM);

        snd_pcm_hw_constraint_minmax(
            (*substream).runtime,
            SNDRV_PCM_HW_PARAM_PERIOD_TIME,
            period_time * USEC_PER_MSEC,
            UINT_MAX,
        );
    }

    /* binding pcm substream to hda stream */
    (*(*substream).runtime).private_data = &mut (*dsp_stream).hstream;

    /*
     * Reset the llp cache values (they are used for LLP compensation in
     * case the counter is not reset)
     */
    (*dsp_stream).pplcllpl = 0;
    (*dsp_stream).pplcllpu = 0;

    0
}
// EXPORT_SYMBOL_NS(hda_dsp_pcm_open, "SND_SOC_SOF_INTEL_HDA_COMMON");

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_pcm_close(
    sdev: *mut snd_sof_dev,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let hstream = (*(*substream).runtime).private_data;
    let direction: c_int = (*substream).stream;
    let ret: c_int;

    ret = hda_dsp_stream_put(sdev, direction, (*hstream).stream_tag);

    if ret != 0 {
        dev_dbg(
            (*sdev).dev,
            b"stream %s not opened!\n\0".as_ptr() as *const c_char,
            (*substream).name,
        );
        return -ENODEV;
    }

    /* unbinding pcm substream to hda stream */
    (*(*substream).runtime).private_data = core::ptr::null_mut();
    0
}
// EXPORT_SYMBOL_NS(hda_dsp_pcm_close, "SND_SOC_SOF_INTEL_HDA_COMMON");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
