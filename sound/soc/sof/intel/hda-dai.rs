// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Authors: Keyon Jie <yang.jie@linux.intel.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;

const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const ENODEV: c_int = 19;

extern "C" {
    static mut hda_dai_ops: snd_soc_dai_ops;
    static mut ssp_dai_ops: snd_soc_dai_ops;
    static mut dmic_dai_ops: snd_soc_dai_ops;
}

extern "C" {
    fn widget_to_sdev(w: *mut snd_soc_dapm_widget) -> *mut snd_sof_dev;
    fn sof_ipc_get_ops(sdev: *mut snd_sof_dev, id: c_int) -> *const sof_ipc_tplg_ops;
    fn snd_soc_dai_get_widget(cpu_dai: *mut snd_soc_dai, stream: c_int) -> *mut snd_soc_dapm_widget;
    fn hda_select_dai_widget_ops(
        sdev: *mut snd_sof_dev,
        swidget: *mut snd_sof_widget,
    ) -> *const hda_dai_widget_dma_ops;
    fn hdac_stream(hext_stream: *mut hdac_ext_stream) -> *mut hdac_stream;
    fn snd_hdac_ext_bus_link_clear_stream_id(hlink: *mut hdac_ext_link, stream_tag: c_int);
    fn snd_hdac_ext_bus_link_set_stream_id(hlink: *mut hdac_ext_link, stream_tag: c_int);
    fn hstream_to_sof_hda_stream(hext_stream: *mut hdac_ext_stream) -> *mut sof_intel_hda_stream;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn sof_to_bus(sdev: *mut snd_sof_dev) -> *mut hdac_bus;
    fn hdac_bus_eml_sdw_map_stream_ch(
        bus: *mut hdac_bus,
        link_id: c_int,
        dai_id: c_int,
        ch_mask: c_int,
        stream_tag: c_int,
        stream: c_int,
    ) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn stream_to_hdac_ext_stream(s: *mut hdac_stream) -> *mut hdac_ext_stream;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn get_chip_info(pdata: *mut sof_dev_desc) -> *const sof_intel_dsp_desc;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn intel_nhlt_init(dev: *mut device) -> *mut c_void;
    fn intel_nhlt_free(nhlt: *mut c_void);
    fn kfree(ptr: *mut c_void);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dobj: snd_soc_dobj,
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dobj {
    pub private: *mut c_void,
}

#[repr(C)]
pub struct snd_sof_widget {
    pub private: *mut c_void,
}

#[repr(C)]
pub struct snd_sof_dai {
    pub private: *mut c_void,
    pub platform_private: *const hda_dai_widget_dma_ops,
}

#[repr(C)]
pub struct snd_sof_dev {
    pub dev: *mut device,
    pub dspless_mode_selected: bool_,
    pub pdata: *mut sof_dev_desc,
    pub private: *mut c_void,
}

#[repr(C)]
pub struct sof_dev_desc {
    pub ipc_type: c_int,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sof_ipc_tplg_ops {
    pub dai_config: Option<
        unsafe extern "C" fn(
            sdev: *mut snd_sof_dev,
            swidget: *mut snd_sof_widget,
            flags: c_uint,
            data: *mut snd_sof_dai_config_data,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_sof_dai_config_data {
    pub dai_data: c_int,
    pub dai_index: c_int,
    pub dai_node_id: c_int,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
    pub name: *const c_char,
    pub id: c_int,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hdac_ext_stream {
    pub hstream: hdac_stream,
    pub link_prepared: c_int,
    pub link_substream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct hdac_stream {
    pub stream_tag: c_int,
    pub direction: c_int,
    pub list: list_head,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct hdac_ext_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hdac_bus {
    pub stream_list: list_head,
}

#[repr(C)]
pub struct sof_intel_hda_stream {
    pub host_reserved: c_int,
}

#[repr(C)]
pub struct hda_dai_widget_dma_ops {
    pub get_hlink:
        Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> *mut hdac_ext_link>,
    pub get_hext_stream: Option<
        unsafe extern "C" fn(
            sdev: *mut snd_sof_dev,
            cpu_dai: *mut snd_soc_dai,
            substream: *mut snd_pcm_substream,
        ) -> *mut hdac_ext_stream,
    >,
    pub assign_hext_stream: Option<
        unsafe extern "C" fn(
            sdev: *mut snd_sof_dev,
            cpu_dai: *mut snd_soc_dai,
            substream: *mut snd_pcm_substream,
            hlink: *mut hdac_ext_link,
        ) -> *mut hdac_ext_stream,
    >,
    pub release_hext_stream: Option<
        unsafe extern "C" fn(
            sdev: *mut snd_sof_dev,
            cpu_dai: *mut snd_soc_dai,
            substream: *mut snd_pcm_substream,
        ),
    >,
    pub codec_dai_set_stream: Option<
        unsafe extern "C" fn(
            sdev: *mut snd_sof_dev,
            substream: *mut snd_pcm_substream,
            hstream: *mut hdac_stream,
        ),
    >,
    pub reset_hext_stream:
        Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, hext_stream: *mut hdac_ext_stream)>,
    pub calc_stream_format: Option<
        unsafe extern "C" fn(
            sdev: *mut snd_sof_dev,
            substream: *mut snd_pcm_substream,
            params: *mut snd_pcm_hw_params,
        ) -> c_uint,
    >,
    pub setup_hext_stream: Option<
        unsafe extern "C" fn(
            sdev: *mut snd_sof_dev,
            hext_stream: *mut hdac_ext_stream,
            format_val: c_uint,
        ),
    >,
    pub pre_trigger: Option<
        unsafe extern "C" fn(
            sdev: *mut snd_sof_dev,
            dai: *mut snd_soc_dai,
            substream: *mut snd_pcm_substream,
            cmd: c_int,
        ) -> c_int,
    >,
    pub trigger: Option<
        unsafe extern "C" fn(
            sdev: *mut snd_sof_dev,
            dai: *mut snd_soc_dai,
            substream: *mut snd_pcm_substream,
            cmd: c_int,
        ) -> c_int,
    >,
    pub post_trigger: Option<
        unsafe extern "C" fn(
            sdev: *mut snd_sof_dev,
            dai: *mut snd_soc_dai,
            substream: *mut snd_pcm_substream,
            cmd: c_int,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dpcm: *mut snd_soc_dpcm,
    pub num_cpus: c_int,
    pub cpu_dais: *mut *mut snd_soc_dai,
}

#[repr(C)]
pub struct snd_soc_dpcm {
    pub hw_params: snd_pcm_hw_params,
    pub state: c_int,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            params: *mut snd_pcm_hw_params,
            dai: *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub hw_free:
        Option<unsafe extern "C" fn(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int>,
    pub trigger: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            cmd: c_int,
            dai: *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub prepare:
        Option<unsafe extern "C" fn(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
pub struct sof_ipc4_copier {
    pub dma_config_tlv: *mut sof_ipc4_dma_config_tlv,
}

#[repr(C)]
pub struct sof_ipc4_dma_config_tlv {
    pub type_: c_uint,
    pub length: usize,
    pub dma_config: sof_ipc4_dma_config,
}

#[repr(C)]
pub struct sof_ipc4_dma_config {
    pub dma_method: c_uint,
    pub pre_allocated_by_host: c_uint,
    pub dma_channel_id: c_int,
    pub stream_id: c_int,
    pub dma_stream_channel_map: sof_ipc4_dma_stream_channel_map,
    pub dma_priv_config_size: c_uint,
}

#[repr(C)]
pub struct sof_ipc4_dma_stream_channel_map {
    pub device_count: c_uint,
    pub mapping: [sof_ipc4_dma_stream_channel_mapping; 1],
}

#[repr(C)]
pub struct sof_ipc4_dma_stream_channel_mapping {
    pub device: c_int,
    pub channel_mask: c_int,
}

#[repr(C)]
pub struct sof_intel_dsp_desc {
    pub hw_ip_version: c_int,
}

#[repr(C)]
pub struct snd_sof_dsp_ops {
    pub num_drv: c_int,
    pub drv: *mut snd_soc_dai_driver,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_pcm_stream {
    pub channels_min: c_uint,
    pub channels_max: c_uint,
}

#[repr(C)]
pub struct sof_ipc4_fw_data {
    pub nhlt: *mut c_void,
}

const tplg: c_int = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 0;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 4;
const SND_SOC_DPCM_STATE_PAUSED: c_int = 5;
const SOF_DAI_CONFIG_FLAGS_HW_PARAMS: c_uint = 1;
const SOF_DAI_CONFIG_FLAGS_2_STEP_STOP: c_uint = 1;
const SOF_DAI_CONFIG_FLAGS_QUIRK_SHIFT: c_uint = 16;
const SOF_IPC4_GTW_DMA_CONFIG_ID: c_uint = 0;
const SOF_IPC4_DMA_METHOD_HDA: c_uint = 1;
const SOF_INTEL_ACE_2_0: c_int = 2;
const SOF_IPC_TYPE_4: c_int = 4;

const fn GENMASK(h: c_int, l: c_int) -> c_int {
    if h < l {
        0
    } else {
        (((!0u32) << (l as u32)) & ((!0u32) >> (31 - h as u32))) as c_int
    }
}

/*
 * The default method is to fetch NHLT from BIOS. With this parameter set
 * it is possible to override that with NHLT in the SOF topology manifest.
 */
static mut hda_use_tplg_nhlt: bool_ = false;
/* module_param_named(sof_use_tplg_nhlt, hda_use_tplg_nhlt, bool, 0444); */
/* MODULE_PARM_DESC(sof_use_tplg_nhlt, "SOF topology nhlt override"); */

#[no_mangle]
pub unsafe extern "C" fn hda_dai_config(
    w: *mut snd_soc_dapm_widget,
    mut flags: c_uint,
    data: *mut snd_sof_dai_config_data,
) -> c_int {
    let swidget = (*w).dobj.private as *mut snd_sof_widget;
    let tplg_ops: *const sof_ipc_tplg_ops;
    let sdev: *mut snd_sof_dev;
    let ret: c_int;

    if swidget.is_null() {
        return 0;
    }

    sdev = widget_to_sdev(w);
    tplg_ops = sof_ipc_get_ops(sdev, tplg);

    if !tplg_ops.is_null() && (*tplg_ops).dai_config.is_some() {
        ret = ((*tplg_ops).dai_config.unwrap())(sdev, swidget, flags, data);
        if ret < 0 {
            dev_err(
                (*sdev).dev,
                c"DAI config with flags %x failed for widget %s\n".as_ptr(),
                flags,
                (*w).name,
            );
            return ret;
        }
    }

    0
}
/* EXPORT_SYMBOL_NS(hda_dai_config, "SND_SOC_SOF_INTEL_HDA_COMMON"); */

/* #if IS_ENABLED(CONFIG_SND_SOC_SOF_HDA_LINK) */

unsafe fn dai_to_sdev(
    substream: *mut snd_pcm_substream,
    cpu_dai: *mut snd_soc_dai,
) -> *mut snd_sof_dev {
    let w = snd_soc_dai_get_widget(cpu_dai, (*substream).stream);

    widget_to_sdev(w)
}

unsafe fn hda_dai_get_ops(
    substream: *mut snd_pcm_substream,
    cpu_dai: *mut snd_soc_dai,
) -> *const hda_dai_widget_dma_ops {
    let w = snd_soc_dai_get_widget(cpu_dai, (*substream).stream);
    let swidget: *mut snd_sof_widget;
    let sdev: *mut snd_sof_dev;
    let sdai: *mut snd_sof_dai;

    /*
     * this is unlikely if the topology and the machine driver DAI links match.
     * But if there's a missing DAI link in topology, this will prevent a NULL pointer
     * dereference later on.
     */
    if w.is_null() {
        dev_err((*cpu_dai).dev, c"%s: widget is NULL\n".as_ptr(), c"hda_dai_get_ops".as_ptr());
        return ptr::null();
    }

    sdev = widget_to_sdev(w);
    swidget = (*w).dobj.private as *mut snd_sof_widget;
    if swidget.is_null() {
        dev_err((*sdev).dev, c"%s: swidget is NULL\n".as_ptr(), c"hda_dai_get_ops".as_ptr());
        return ptr::null();
    }

    if (*sdev).dspless_mode_selected {
        return hda_select_dai_widget_ops(sdev, swidget);
    }

    sdai = (*swidget).private as *mut snd_sof_dai;

    /* select and set the DAI widget ops if not set already */
    if (*sdai).platform_private.is_null() {
        let ops = hda_select_dai_widget_ops(sdev, swidget);
        if ops.is_null() {
            return ptr::null();
        }

        /* check if mandatory ops are set */
        if ops.is_null() || (*ops).get_hext_stream.is_none() {
            return ptr::null();
        }

        (*sdai).platform_private = ops;
    }

    (*sdai).platform_private
}

unsafe fn hda_link_dma_cleanup(
    substream: *mut snd_pcm_substream,
    hext_stream: *mut hdac_ext_stream,
    cpu_dai: *mut snd_soc_dai,
    release: bool_,
) -> c_int {
    let ops = hda_dai_get_ops(substream, cpu_dai);
    let hda_stream: *mut sof_intel_hda_stream;
    let hlink: *mut hdac_ext_link;
    let sdev: *mut snd_sof_dev;
    let stream_tag: c_int;

    if ops.is_null() {
        dev_err((*cpu_dai).dev, c"DAI widget ops not set\n".as_ptr());
        return -EINVAL;
    }

    sdev = dai_to_sdev(substream, cpu_dai);

    hlink = ((*ops).get_hlink.unwrap())(sdev, substream);
    if hlink.is_null() {
        return -EINVAL;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        stream_tag = (*hdac_stream(hext_stream)).stream_tag;
        snd_hdac_ext_bus_link_clear_stream_id(hlink, stream_tag);
    }

    if !release {
        /*
         * Force stream reconfiguration without releasing the channel on
         * subsequent stream restart (without free), including LinkDMA
         * reset.
         * The stream is released via hda_dai_hw_free()
         */
        (*hext_stream).link_prepared = 0;
        return 0;
    }

    if let Some(release_hext_stream) = (*ops).release_hext_stream {
        release_hext_stream(sdev, cpu_dai, substream);
    }

    (*hext_stream).link_prepared = 0;

    /* free the host DMA channel reserved by hostless streams */
    hda_stream = hstream_to_sof_hda_stream(hext_stream);
    (*hda_stream).host_reserved = 0;

    0
}

unsafe fn hda_link_dma_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let ops = hda_dai_get_ops(substream, cpu_dai);
    let mut hext_stream: *mut hdac_ext_stream;
    let hstream: *mut hdac_stream;
    let hlink: *mut hdac_ext_link;
    let sdev: *mut snd_sof_dev;
    let stream_tag: c_int;

    if ops.is_null() {
        dev_err((*cpu_dai).dev, c"DAI widget ops not set\n".as_ptr());
        return -EINVAL;
    }

    sdev = dai_to_sdev(substream, cpu_dai);

    hlink = ((*ops).get_hlink.unwrap())(sdev, substream);
    if hlink.is_null() {
        return -EINVAL;
    }

    hext_stream = ((*ops).get_hext_stream.unwrap())(sdev, cpu_dai, substream);

    if hext_stream.is_null() {
        if let Some(assign_hext_stream) = (*ops).assign_hext_stream {
            hext_stream = assign_hext_stream(sdev, cpu_dai, substream, hlink);
        }
    }

    if hext_stream.is_null() {
        return -EBUSY;
    }

    hstream = &mut (*hext_stream).hstream;
    stream_tag = (*hstream).stream_tag;

    if (*hext_stream).hstream.direction == SNDRV_PCM_STREAM_PLAYBACK {
        snd_hdac_ext_bus_link_set_stream_id(hlink, stream_tag);
    }

    /* set the hdac_stream in the codec dai */
    if let Some(codec_dai_set_stream) = (*ops).codec_dai_set_stream {
        codec_dai_set_stream(sdev, substream, hstream);
    }

    if let Some(reset_hext_stream) = (*ops).reset_hext_stream {
        reset_hext_stream(sdev, hext_stream);
    }

    if let (Some(calc_stream_format), Some(setup_hext_stream)) =
        ((*ops).calc_stream_format, (*ops).setup_hext_stream)
    {
        let format_val = calc_stream_format(sdev, substream, params);

        setup_hext_stream(sdev, hext_stream, format_val);
    }

    (*hext_stream).link_prepared = 1;

    0
}

unsafe extern "C" fn hda_dai_hw_free(
    substream: *mut snd_pcm_substream,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let ops = hda_dai_get_ops(substream, cpu_dai);
    let hext_stream: *mut hdac_ext_stream;
    let sdev = dai_to_sdev(substream, cpu_dai);

    if ops.is_null() {
        dev_err((*cpu_dai).dev, c"DAI widget ops not set\n".as_ptr());
        return -EINVAL;
    }

    hext_stream = ((*ops).get_hext_stream.unwrap())(sdev, cpu_dai, substream);
    if hext_stream.is_null() {
        return 0;
    }

    hda_link_dma_cleanup(substream, hext_stream, cpu_dai, true)
}

unsafe fn hda_dai_hw_params_data(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
    data: *mut snd_sof_dai_config_data,
    mut flags: c_uint,
) -> c_int {
    let w = snd_soc_dai_get_widget(dai, (*substream).stream);
    let ops = hda_dai_get_ops(substream, dai);
    let mut hext_stream: *mut hdac_ext_stream;
    let sdev = widget_to_sdev(w);
    let ret: c_int;

    if ops.is_null() {
        dev_err((*sdev).dev, c"DAI widget ops not set\n".as_ptr());
        return -EINVAL;
    }

    hext_stream = ((*ops).get_hext_stream.unwrap())(sdev, dai, substream);
    if !hext_stream.is_null() && (*hext_stream).link_prepared != 0 {
        return 0;
    }

    ret = hda_link_dma_hw_params(substream, params, dai);
    if ret < 0 {
        return ret;
    }

    hext_stream = ((*ops).get_hext_stream.unwrap())(sdev, dai, substream);

    flags |= SOF_DAI_CONFIG_FLAGS_2_STEP_STOP << SOF_DAI_CONFIG_FLAGS_QUIRK_SHIFT;
    (*data).dai_data = (*hdac_stream(hext_stream)).stream_tag - 1;

    hda_dai_config(w, flags, data)
}

unsafe extern "C" fn hda_dai_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let mut data: snd_sof_dai_config_data = core::mem::zeroed();
    let flags = SOF_DAI_CONFIG_FLAGS_HW_PARAMS;

    hda_dai_hw_params_data(substream, params, dai, &mut data, flags)
}

/*
 * In contrast to IPC3, the dai trigger in IPC4 mixes pipeline state changes
 * (over IPC channel) and DMA state change (direct host register changes).
 */
unsafe extern "C" fn hda_dai_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let ops = hda_dai_get_ops(substream, dai);
    let hext_stream: *mut hdac_ext_stream;
    let sdev: *mut snd_sof_dev;
    let mut ret: c_int;

    if ops.is_null() {
        dev_err((*dai).dev, c"DAI widget ops not set\n".as_ptr());
        return -EINVAL;
    }

    dev_dbg((*dai).dev, c"cmd=%d dai %s direction %d\n".as_ptr(), cmd, (*dai).name, (*substream).stream);

    sdev = dai_to_sdev(substream, dai);

    hext_stream = ((*ops).get_hext_stream.unwrap())(sdev, dai, substream);
    if hext_stream.is_null() {
        return -EINVAL;
    }

    if let Some(pre_trigger) = (*ops).pre_trigger {
        ret = pre_trigger(sdev, dai, substream, cmd);
        if ret < 0 {
            return ret;
        }
    }

    if let Some(trigger) = (*ops).trigger {
        ret = trigger(sdev, dai, substream, cmd);
        if ret < 0 {
            return ret;
        }
    }

    if let Some(post_trigger) = (*ops).post_trigger {
        ret = post_trigger(sdev, dai, substream, cmd);
        if ret < 0 {
            return ret;
        }
    }

    match cmd {
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
            ret = hda_link_dma_cleanup(
                substream,
                hext_stream,
                dai,
                cmd != SNDRV_PCM_TRIGGER_STOP,
            );
            if ret < 0 {
                dev_err((*sdev).dev, c"%s: failed to clean up link DMA\n".as_ptr(), c"hda_dai_trigger".as_ptr());
                return ret;
            }
        }
        _ => {}
    }

    0
}

/* #if IS_ENABLED(CONFIG_SND_SOC_SOF_HDA_AUDIO_CODEC) */

unsafe extern "C" fn hda_dai_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let stream = (*substream).stream;

    hda_dai_hw_params(substream, &mut (*(*rtd).dpcm.add(stream as usize)).hw_params, dai)
}

#[no_mangle]
pub static hda_dai_ops_def: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(hda_dai_hw_params),
    hw_free: Some(hda_dai_hw_free),
    trigger: Some(hda_dai_trigger),
    prepare: Some(hda_dai_prepare),
};

/* #endif */

unsafe fn widget_to_copier(w: *mut snd_soc_dapm_widget) -> *mut sof_ipc4_copier {
    let swidget = (*w).dobj.private as *mut snd_sof_widget;
    let sdai = (*swidget).private as *mut snd_sof_dai;
    let ipc4_copier = (*sdai).private as *mut sof_ipc4_copier;

    ipc4_copier
}

unsafe fn non_hda_dai_hw_params_data(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    cpu_dai: *mut snd_soc_dai,
    data: *mut snd_sof_dai_config_data,
    flags: c_uint,
) -> c_int {
    let w = snd_soc_dai_get_widget(cpu_dai, (*substream).stream);
    let rtd = snd_soc_substream_to_rtd(substream);
    let dma_config_tlv: *mut sof_ipc4_dma_config_tlv;
    let ops: *const hda_dai_widget_dma_ops;
    let dma_config: *mut sof_ipc4_dma_config;
    let ipc4_copier: *mut sof_ipc4_copier;
    let mut hext_stream: *mut hdac_ext_stream;
    let hstream: *mut hdac_stream;
    let sdev: *mut snd_sof_dev;
    let mut cpu_dai_id: c_int;
    let stream_id: c_int;
    let ret: c_int;

    ops = hda_dai_get_ops(substream, cpu_dai);
    if ops.is_null() {
        dev_err((*cpu_dai).dev, c"DAI widget ops not set\n".as_ptr());
        return -EINVAL;
    }

    sdev = widget_to_sdev(w);
    hext_stream = ((*ops).get_hext_stream.unwrap())(sdev, cpu_dai, substream);

    /* nothing more to do if the link is already prepared */
    if !hext_stream.is_null() && (*hext_stream).link_prepared != 0 {
        return 0;
    }

    /* use HDaudio stream handling */
    ret = hda_dai_hw_params_data(substream, params, cpu_dai, data, flags);
    if ret < 0 {
        dev_err((*cpu_dai).dev, c"%s: hda_dai_hw_params_data failed: %d\n".as_ptr(), c"non_hda_dai_hw_params_data".as_ptr(), ret);
        return ret;
    }

    if (*sdev).dspless_mode_selected {
        return 0;
    }

    /* get stream_id */
    hext_stream = ((*ops).get_hext_stream.unwrap())(sdev, cpu_dai, substream);

    if hext_stream.is_null() {
        dev_err((*cpu_dai).dev, c"%s: no hext_stream found\n".as_ptr(), c"non_hda_dai_hw_params_data".as_ptr());
        return -ENODEV;
    }

    hstream = &mut (*hext_stream).hstream;
    stream_id = (*hstream).stream_tag;

    if stream_id == 0 {
        dev_err((*cpu_dai).dev, c"%s: no stream_id allocated\n".as_ptr(), c"non_hda_dai_hw_params_data".as_ptr());
        return -ENODEV;
    }

    /* configure TLV */
    ipc4_copier = widget_to_copier(w);

    cpu_dai_id = 0;
    while cpu_dai_id < (*rtd).num_cpus {
        let dai = *(*rtd).cpu_dais.add(cpu_dai_id as usize);
        if dai == cpu_dai {
            break;
        }
        cpu_dai_id += 1;
    }

    dma_config_tlv = (*ipc4_copier).dma_config_tlv.add(cpu_dai_id as usize);
    (*dma_config_tlv).type_ = SOF_IPC4_GTW_DMA_CONFIG_ID;
    /* dma_config_priv_size is zero */
    (*dma_config_tlv).length = size_of::<sof_ipc4_dma_config>();

    dma_config = &mut (*dma_config_tlv).dma_config;

    (*dma_config).dma_method = SOF_IPC4_DMA_METHOD_HDA;
    (*dma_config).pre_allocated_by_host = 1;
    (*dma_config).dma_channel_id = stream_id - 1;
    (*dma_config).stream_id = stream_id;
    /*
     * Currently we use a DMA for each device in ALH blob. The device will
     * be copied in sof_ipc4_prepare_copier_module.
     */
    (*dma_config).dma_stream_channel_map.device_count = 1;
    (*dma_config).dma_priv_config_size = 0;

    0
}

unsafe extern "C" fn non_hda_dai_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let mut data: snd_sof_dai_config_data = core::mem::zeroed();
    let flags = SOF_DAI_CONFIG_FLAGS_HW_PARAMS;

    non_hda_dai_hw_params_data(substream, params, cpu_dai, &mut data, flags)
}

unsafe extern "C" fn non_hda_dai_prepare(
    substream: *mut snd_pcm_substream,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let stream = (*substream).stream;

    non_hda_dai_hw_params(substream, &mut (*(*rtd).dpcm.add(stream as usize)).hw_params, cpu_dai)
}

#[no_mangle]
pub static ssp_dai_ops_def: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(non_hda_dai_hw_params),
    hw_free: Some(hda_dai_hw_free),
    trigger: Some(hda_dai_trigger),
    prepare: Some(non_hda_dai_prepare),
};

#[no_mangle]
pub static dmic_dai_ops_def: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(non_hda_dai_hw_params),
    hw_free: Some(hda_dai_hw_free),
    trigger: Some(hda_dai_trigger),
    prepare: Some(non_hda_dai_prepare),
};

#[no_mangle]
pub unsafe extern "C" fn sdw_hda_dai_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    cpu_dai: *mut snd_soc_dai,
    link_id: c_int,
    intel_alh_id: c_int,
) -> c_int {
    let mut w = snd_soc_dai_get_widget(cpu_dai, (*substream).stream);
    let rtd = snd_soc_substream_to_rtd(substream);
    let mut dma_config_tlv: *mut sof_ipc4_dma_config_tlv;
    let mut data: snd_sof_dai_config_data = core::mem::zeroed();
    let flags = SOF_DAI_CONFIG_FLAGS_HW_PARAMS;
    let ops: *const hda_dai_widget_dma_ops;
    let dma_config: *mut sof_ipc4_dma_config;
    let mut ipc4_copier: *mut sof_ipc4_copier;
    let mut hext_stream: *mut hdac_ext_stream;
    let sdev: *mut snd_sof_dev;
    let mut cpu_dai_found = false;
    let mut cpu_dai_id: c_int;
    let ch_mask: c_int;
    let mut ret: c_int;
    let mut i: c_int;

    if w.is_null() {
        dev_err(
            (*cpu_dai).dev,
            c"%s widget not found, check amp link num in the topology\n".as_ptr(),
            (*cpu_dai).name,
        );
        return -EINVAL;
    }

    ops = hda_dai_get_ops(substream, cpu_dai);
    if ops.is_null() {
        dev_err((*cpu_dai).dev, c"DAI widget ops not set\n".as_ptr());
        return -EINVAL;
    }

    sdev = widget_to_sdev(w);
    hext_stream = ((*ops).get_hext_stream.unwrap())(sdev, cpu_dai, substream);

    /* nothing more to do if the link is already prepared */
    if !hext_stream.is_null() && (*hext_stream).link_prepared != 0 {
        return 0;
    }

    /*
     * reset the PCMSyCM registers to handle a prepare callback when the PCM is restarted
     * due to xruns or after a call to snd_pcm_drain/drop()
     */
    ret = hdac_bus_eml_sdw_map_stream_ch(
        sof_to_bus(sdev),
        link_id,
        (*cpu_dai).id,
        0,
        0,
        (*substream).stream,
    );
    if ret < 0 {
        dev_err((*cpu_dai).dev, c"%s:  hdac_bus_eml_sdw_map_stream_ch failed %d\n".as_ptr(), c"sdw_hda_dai_hw_params".as_ptr(), ret);
        return ret;
    }

    data.dai_index = (link_id << 8) | (*cpu_dai).id;
    data.dai_node_id = intel_alh_id;
    ret = non_hda_dai_hw_params_data(substream, params, cpu_dai, &mut data, flags);
    if ret < 0 {
        dev_err((*cpu_dai).dev, c"%s: non_hda_dai_hw_params failed %d\n".as_ptr(), c"sdw_hda_dai_hw_params".as_ptr(), ret);
        return ret;
    }

    hext_stream = ((*ops).get_hext_stream.unwrap())(sdev, cpu_dai, substream);
    if hext_stream.is_null() {
        return -ENODEV;
    }

    /*
     * in the case of SoundWire we need to program the PCMSyCM registers. In case
     * of aggregated devices, we need to define the channel mask for each sublink
     * by reconstructing the split done in soc-pcm.c
     */
    cpu_dai_id = 0;
    while cpu_dai_id < (*rtd).num_cpus {
        let dai = *(*rtd).cpu_dais.add(cpu_dai_id as usize);
        if dai == cpu_dai {
            cpu_dai_found = true;
            break;
        }
        cpu_dai_id += 1;
    }

    if !cpu_dai_found {
        return -ENODEV;
    }

    ch_mask = GENMASK(params_channels(params) - 1, 0);

    ret = hdac_bus_eml_sdw_map_stream_ch(
        sof_to_bus(sdev),
        link_id,
        (*cpu_dai).id,
        ch_mask,
        (*hdac_stream(hext_stream)).stream_tag,
        (*substream).stream,
    );
    if ret < 0 {
        dev_err((*cpu_dai).dev, c"%s:  hdac_bus_eml_sdw_map_stream_ch failed %d\n".as_ptr(), c"sdw_hda_dai_hw_params".as_ptr(), ret);
        return ret;
    }

    if (*sdev).dspless_mode_selected {
        return 0;
    }

    ipc4_copier = widget_to_copier(w);
    dma_config_tlv = (*ipc4_copier).dma_config_tlv.add(cpu_dai_id as usize);
    dma_config = &mut (*dma_config_tlv).dma_config;
    (*dma_config).dma_stream_channel_map.mapping[0].device = data.dai_index;
    (*dma_config).dma_stream_channel_map.mapping[0].channel_mask = ch_mask;

    /*
     * copy the dma_config_tlv to all ipc4_copier in the same link. Because only one copier
     * will be handled in sof_ipc4_prepare_copier_module.
     */
    i = 0;
    while i < (*rtd).num_cpus {
        let dai = *(*rtd).cpu_dais.add(i as usize);
        w = snd_soc_dai_get_widget(dai, (*substream).stream);
        if w.is_null() {
            dev_err(
                (*cpu_dai).dev,
                c"%s widget not found, check amp link num in the topology\n".as_ptr(),
                (*dai).name,
            );
            return -EINVAL;
        }
        ipc4_copier = widget_to_copier(w);
        memcpy(
            (*ipc4_copier).dma_config_tlv.add(cpu_dai_id as usize) as *mut c_void,
            dma_config_tlv as *const c_void,
            size_of::<sof_ipc4_dma_config_tlv>(),
        );
        i += 1;
    }
    0
}
/* EXPORT_SYMBOL_NS(sdw_hda_dai_hw_params, "SND_SOC_SOF_INTEL_HDA_COMMON"); */

#[no_mangle]
pub unsafe extern "C" fn sdw_hda_dai_hw_free(
    substream: *mut snd_pcm_substream,
    cpu_dai: *mut snd_soc_dai,
    link_id: c_int,
) -> c_int {
    let w = snd_soc_dai_get_widget(cpu_dai, (*substream).stream);
    let sdev: *mut snd_sof_dev;
    let mut ret: c_int;

    ret = hda_dai_hw_free(substream, cpu_dai);
    if ret < 0 {
        dev_err((*cpu_dai).dev, c"%s: non_hda_dai_hw_free failed %d\n".as_ptr(), c"sdw_hda_dai_hw_free".as_ptr(), ret);
        return ret;
    }

    sdev = widget_to_sdev(w);

    /* in the case of SoundWire we need to reset the PCMSyCM registers */
    ret = hdac_bus_eml_sdw_map_stream_ch(
        sof_to_bus(sdev),
        link_id,
        (*cpu_dai).id,
        0,
        0,
        (*substream).stream,
    );
    if ret < 0 {
        dev_err((*cpu_dai).dev, c"%s:  hdac_bus_eml_sdw_map_stream_ch failed %d\n".as_ptr(), c"sdw_hda_dai_hw_free".as_ptr(), ret);
        return ret;
    }

    0
}
/* EXPORT_SYMBOL_NS(sdw_hda_dai_hw_free, "SND_SOC_SOF_INTEL_HDA_COMMON"); */

#[no_mangle]
pub unsafe extern "C" fn sdw_hda_dai_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    hda_dai_trigger(substream, cmd, cpu_dai)
}
/* EXPORT_SYMBOL_NS(sdw_hda_dai_trigger, "SND_SOC_SOF_INTEL_HDA_COMMON"); */

unsafe fn hda_dai_suspend(bus: *mut hdac_bus) -> c_int {
    let mut rtd: *mut snd_soc_pcm_runtime;
    let mut hext_stream: *mut hdac_ext_stream;
    let mut s: *mut hdac_stream;
    let mut ret: c_int;

    /* set internal flag for BE */
    s = (*bus).stream_list.next as *mut hdac_stream;
    while !s.is_null() && &mut (*s).list as *mut list_head != &mut (*bus).stream_list {
        hext_stream = stream_to_hdac_ext_stream(s);

        /*
         * clear stream. This should already be taken care for running
         * streams when the SUSPEND trigger is called. But paused
         * streams do not get suspended, so this needs to be done
         * explicitly during suspend.
         */
        if !(*hext_stream).link_substream.is_null() {
            let ops: *const hda_dai_widget_dma_ops;
            let swidget: *mut snd_sof_widget;
            let w: *mut snd_soc_dapm_widget;
            let cpu_dai: *mut snd_soc_dai;
            let sdev: *mut snd_sof_dev;
            let sdai: *mut snd_sof_dai;

            rtd = snd_soc_substream_to_rtd((*hext_stream).link_substream);
            cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
            w = snd_soc_dai_get_widget(cpu_dai, (*hdac_stream(hext_stream)).direction);
            swidget = (*w).dobj.private as *mut snd_sof_widget;
            sdev = widget_to_sdev(w);
            sdai = (*swidget).private as *mut snd_sof_dai;
            ops = (*sdai).platform_private;

            if (*(*rtd).dpcm.add((*(*hext_stream).link_substream).stream as usize)).state
                != SND_SOC_DPCM_STATE_PAUSED
            {
                s = (*(*s).list.next).prev as *mut hdac_stream;
                continue;
            }

            /* for consistency with TRIGGER_SUSPEND  */
            if let Some(post_trigger) = (*ops).post_trigger {
                ret = post_trigger(
                    sdev,
                    cpu_dai,
                    (*hext_stream).link_substream,
                    SNDRV_PCM_TRIGGER_SUSPEND,
                );
                if ret < 0 {
                    return ret;
                }
            }

            ret = hda_link_dma_cleanup((*hext_stream).link_substream, hext_stream, cpu_dai, true);
            if ret < 0 {
                return ret;
            }
        }
        s = (*(*s).list.next).prev as *mut hdac_stream;
    }

    0
}

unsafe fn ssp_set_dai_drv_ops(sdev: *mut snd_sof_dev, ops: *mut snd_sof_dsp_ops) {
    let chip: *const sof_intel_dsp_desc;
    let mut i: c_int;

    chip = get_chip_info((*sdev).pdata);

    if (*chip).hw_ip_version >= SOF_INTEL_ACE_2_0 {
        i = 0;
        while i < (*ops).num_drv {
            let drv = (*ops).drv.add(i as usize);
            if !strstr((*drv).name, c"SSP".as_ptr()).is_null() {
                (*drv).ops = &ssp_dai_ops as *const snd_soc_dai_ops;
            }
            i += 1;
        }
    }
}

unsafe fn dmic_set_dai_drv_ops(sdev: *mut snd_sof_dev, ops: *mut snd_sof_dsp_ops) {
    let chip: *const sof_intel_dsp_desc;
    let mut i: c_int;

    chip = get_chip_info((*sdev).pdata);

    if (*chip).hw_ip_version >= SOF_INTEL_ACE_2_0 {
        i = 0;
        while i < (*ops).num_drv {
            let drv = (*ops).drv.add(i as usize);
            if !strstr((*drv).name, c"DMIC".as_ptr()).is_null() {
                (*drv).ops = &dmic_dai_ops as *const snd_soc_dai_ops;
            }
            i += 1;
        }
    }
}

/* #else */
/* static inline void ssp_set_dai_drv_ops(struct snd_sof_dev *sdev, struct snd_sof_dsp_ops *ops) {} */
/* static inline void dmic_set_dai_drv_ops(struct snd_sof_dev *sdev, struct snd_sof_dsp_ops *ops) {} */
/* #endif CONFIG_SND_SOC_SOF_HDA_LINK */

#[no_mangle]
pub unsafe extern "C" fn hda_set_dai_drv_ops(sdev: *mut snd_sof_dev, ops: *mut snd_sof_dsp_ops) {
    let mut i: c_int;

    i = 0;
    while i < (*ops).num_drv {
        /* #if IS_ENABLED(CONFIG_SND_SOC_SOF_HDA_AUDIO_CODEC) */
        let drv = (*ops).drv.add(i as usize);
        if !strstr((*drv).name, c"iDisp".as_ptr()).is_null()
            || !strstr((*drv).name, c"Analog".as_ptr()).is_null()
            || !strstr((*drv).name, c"Digital".as_ptr()).is_null()
        {
            (*drv).ops = &hda_dai_ops as *const snd_soc_dai_ops;
        }
        /* #endif */
        i += 1;
    }

    ssp_set_dai_drv_ops(sdev, ops);
    dmic_set_dai_drv_ops(sdev, ops);

    if (*(*sdev).pdata).ipc_type == SOF_IPC_TYPE_4 && !hda_use_tplg_nhlt {
        let ipc4_data = (*sdev).private as *mut sof_ipc4_fw_data;

        (*ipc4_data).nhlt = intel_nhlt_init((*sdev).dev);
    }
}
/* EXPORT_SYMBOL_NS(hda_set_dai_drv_ops, "SND_SOC_SOF_INTEL_HDA_COMMON"); */

#[no_mangle]
pub unsafe extern "C" fn hda_ops_free(sdev: *mut snd_sof_dev) {
    if (*(*sdev).pdata).ipc_type == SOF_IPC_TYPE_4 {
        let ipc4_data = (*sdev).private as *mut sof_ipc4_fw_data;

        if !hda_use_tplg_nhlt {
            intel_nhlt_free((*ipc4_data).nhlt);
        }

        kfree((*sdev).private);
        (*sdev).private = ptr::null_mut();
    }
}
/* EXPORT_SYMBOL_NS(hda_ops_free, "SND_SOC_SOF_INTEL_HDA_COMMON"); */

/*
 * common dai driver for skl+ platforms.
 * some products who use this DAI array only physically have a subset of
 * the DAIs, but no harm is done here by adding the whole set.
 */
#[no_mangle]
pub static mut skl_dai: [snd_soc_dai_driver; 16] = [
    snd_soc_dai_driver {
        name: c"SSP0 Pin".as_ptr(),
        playback: snd_soc_pcm_stream { channels_min: 1, channels_max: 8 },
        capture: snd_soc_pcm_stream { channels_min: 1, channels_max: 8 },
        ops: ptr::null(),
    },
    snd_soc_dai_driver {
        name: c"SSP1 Pin".as_ptr(),
        playback: snd_soc_pcm_stream { channels_min: 1, channels_max: 8 },
        capture: snd_soc_pcm_stream { channels_min: 1, channels_max: 8 },
        ops: ptr::null(),
    },
    snd_soc_dai_driver {
        name: c"SSP2 Pin".as_ptr(),
        playback: snd_soc_pcm_stream { channels_min: 1, channels_max: 8 },
        capture: snd_soc_pcm_stream { channels_min: 1, channels_max: 8 },
        ops: ptr::null(),
    },
    snd_soc_dai_driver {
        name: c"SSP3 Pin".as_ptr(),
        playback: snd_soc_pcm_stream { channels_min: 1, channels_max: 8 },
        capture: snd_soc_pcm_stream { channels_min: 1, channels_max: 8 },
        ops: ptr::null(),
    },
    snd_soc_dai_driver {
        name: c"SSP4 Pin".as_ptr(),
        playback: snd_soc_pcm_stream { channels_min: 1, channels_max: 8 },
        capture: snd_soc_pcm_stream { channels_min: 1, channels_max: 8 },
        ops: ptr::null(),
    },
    snd_soc_dai_driver {
        name: c"SSP5 Pin".as_ptr(),
        playback: snd_soc_pcm_stream { channels_min: 1, channels_max: 8 },
        capture: snd_soc_pcm_stream { channels_min: 1, channels_max: 8 },
        ops: ptr::null(),
    },
    snd_soc_dai_driver {
        name: c"DMIC01 Pin".as_ptr(),
        playback: snd_soc_pcm_stream { channels_min: 0, channels_max: 0 },
        capture: snd_soc_pcm_stream { channels_min: 1, channels_max: 4 },
        ops: ptr::null(),
    },
    snd_soc_dai_driver {
        name: c"DMIC16k Pin".as_ptr(),
        playback: snd_soc_pcm_stream { channels_min: 0, channels_max: 0 },
        capture: snd_soc_pcm_stream { channels_min: 1, channels_max: 4 },
        ops: ptr::null(),
    },
    snd_soc_dai_driver {
        /* Virtual CPU DAI for Echo reference */
        name: c"Loopback Virtual Pin".as_ptr(),
        playback: snd_soc_pcm_stream { channels_min: 0, channels_max: 0 },
        capture: snd_soc_pcm_stream { channels_min: 1, channels_max: 2 },
        ops: ptr::null(),
    },
    /* #if IS_ENABLED(CONFIG_SND_SOC_SOF_HDA_AUDIO_CODEC) */
    snd_soc_dai_driver {
        name: c"iDisp1 Pin".as_ptr(),
        playback: snd_soc_pcm_stream { channels_min: 1, channels_max: 8 },
        capture: snd_soc_pcm_stream { channels_min: 0, channels_max: 0 },
        ops: ptr::null(),
    },
    snd_soc_dai_driver {
        name: c"iDisp2 Pin".as_ptr(),
        playback: snd_soc_pcm_stream { channels_min: 1, channels_max: 8 },
        capture: snd_soc_pcm_stream { channels_min: 0, channels_max: 0 },
        ops: ptr::null(),
    },
    snd_soc_dai_driver {
        name: c"iDisp3 Pin".as_ptr(),
        playback: snd_soc_pcm_stream { channels_min: 1, channels_max: 8 },
        capture: snd_soc_pcm_stream { channels_min: 0, channels_max: 0 },
        ops: ptr::null(),
    },
    snd_soc_dai_driver {
        name: c"iDisp4 Pin".as_ptr(),
        playback: snd_soc_pcm_stream { channels_min: 1, channels_max: 8 },
        capture: snd_soc_pcm_stream { channels_min: 0, channels_max: 0 },
        ops: ptr::null(),
    },
    snd_soc_dai_driver {
        name: c"Analog CPU DAI".as_ptr(),
        playback: snd_soc_pcm_stream { channels_min: 1, channels_max: 16 },
        capture: snd_soc_pcm_stream { channels_min: 1, channels_max: 16 },
        ops: ptr::null(),
    },
    snd_soc_dai_driver {
        name: c"Digital CPU DAI".as_ptr(),
        playback: snd_soc_pcm_stream { channels_min: 1, channels_max: 16 },
        capture: snd_soc_pcm_stream { channels_min: 1, channels_max: 16 },
        ops: ptr::null(),
    },
    snd_soc_dai_driver {
        name: c"Alt Analog CPU DAI".as_ptr(),
        playback: snd_soc_pcm_stream { channels_min: 1, channels_max: 16 },
        capture: snd_soc_pcm_stream { channels_min: 1, channels_max: 16 },
        ops: ptr::null(),
    },
    /* #endif */
];
/* EXPORT_SYMBOL_NS(skl_dai, "SND_SOC_SOF_INTEL_HDA_COMMON"); */

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_dais_suspend(sdev: *mut snd_sof_dev) -> c_int {
    /*
     * In the corner case where a SUSPEND happens during a PAUSE, the ALSA core
     * does not throw the TRIGGER_SUSPEND. This leaves the DAIs in an unbalanced state.
     * Since the component suspend is called last, we can trap this corner case
     * and force the DAIs to release their resources.
     */
    /* #if IS_ENABLED(CONFIG_SND_SOC_SOF_HDA_LINK) */
    let ret: c_int;

    ret = hda_dai_suspend(sof_to_bus(sdev));
    if ret < 0 {
        return ret;
    }
    /* #endif */

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
