// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//
// PCM Layer, interface between ALSA and IPC.
//
// Translated from C source file soc/sof/pcm.c.
// Linux/SOF declarations included by the C file are external dependencies.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type size_t = usize;
type snd_pcm_uframes_t = isize;
type snd_pcm_sframes_t = isize;

const EINVAL: c_int = 22;
const EOPNOTSUPP: c_int = 95;
const EACCES: c_int = 13;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_TRIGGER_STOP: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 1;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 2;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
const SNDRV_DMA_TYPE_DEV_SG: c_int = 0;
const SOF_SUSPEND_S0IX: c_int = 1;
const SOF_BE_PCM_BASE: c_int = 16;
static SOF_AUDIO_PCM_DRV_NAME: &[u8] = b"sof-audio-component\0";

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_dma_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_interval {
    min: c_uint,
    max: c_uint,
}

#[repr(C)]
pub struct snd_mask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hardware {
    info: u64,
    formats: u64,
    period_bytes_min: size_t,
    period_bytes_max: size_t,
    periods_min: c_uint,
    periods_max: c_uint,
    buffer_bytes_max: size_t,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    hw: snd_pcm_hardware,
    buffer_changed: bool_,
    dma_bytes: size_t,
}

#[repr(C)]
pub struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
    stream: c_int,
    wait_time: c_int,
}

#[repr(C)]
pub struct snd_pcm_stream {
    substream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct snd_pcm {
    streams: [snd_pcm_stream; 2],
    id: [c_char; 64],
    name: [c_char; 80],
}

#[repr(C)]
pub struct snd_soc_dai_link {
    no_pcm: bool_,
    id: c_int,
    name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    dai_link: *mut snd_soc_dai_link,
    pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_soc_component {
    dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_widget_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_tplg_stream_caps {
    name: *const c_char,
    formats: u64,
    period_size_min: u32,
    period_size_max: u32,
    periods_min: u32,
    periods_max: u32,
    buffer_size_min: u32,
    buffer_size_max: u32,
}

#[repr(C)]
pub struct sof_ipc_pcm {
    pcm_id: u32,
    pcm_name: *const c_char,
    playback: bool_,
    capture: bool_,
    caps: [snd_soc_tplg_stream_caps; 2],
}

#[repr(C)]
pub struct snd_sof_pcm_posn {
    host_posn: u64,
    dai_posn: u64,
}

#[repr(C)]
pub struct page_table {
    area: *mut c_void,
}

#[repr(C)]
pub struct snd_sof_pcm_stream {
    period_elapsed_work: work_struct,
    substream: *mut snd_pcm_substream,
    list: *mut snd_soc_dapm_widget_list,
    suspend_ignored: bool_,
    d0i3_compatible: bool_,
    comp_id: c_int,
    posn: snd_sof_pcm_posn,
    page_table: page_table,
}

#[repr(C)]
pub struct snd_sof_pcm {
    list: list_head,
    scomp: *mut snd_soc_component,
    pcm: sof_ipc_pcm,
    stream: [snd_sof_pcm_stream; 2],
    platform_params: [snd_sof_platform_stream_params; 2],
    params: [snd_pcm_hw_params; 2],
    prepared: [bool_; 2],
    pending_stop: [bool_; 2],
}

#[repr(C)]
pub struct snd_sof_platform_stream_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_widget {
    list: list_head,
    comp_id: c_int,
}

#[repr(C)]
pub struct snd_sof_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_pdata_machine {
    drv_name: *const c_char,
}

#[repr(C)]
pub struct snd_sof_pdata {
    tplg_filename_prefix: *const c_char,
    tplg_filename: *const c_char,
    machine: *mut snd_sof_pdata_machine,
    of_machine: *mut snd_sof_pdata_machine,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    name: *const c_char,
    probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    prepare: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    hw_free: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_int) -> c_int>,
    pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    ack: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    delay: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_sframes_t>,
    compress_ops: *const c_void,
    pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> c_int>,
    ignore_machine: *const c_char,
    be_pcm_base: c_int,
    use_dai_pcm_id: bool_,
    topology_name_prefix: *const c_char,
    module_get_upon_open: c_uint,
    legacy_dai_naming: c_uint,
    be_hw_params_fixup: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct snd_sof_dev {
    dev: *mut device,
    component: *mut snd_soc_component,
    pdata: *mut snd_sof_pdata,
    plat_drv: snd_soc_component_driver,
    pcm_list: list_head,
    widget_list: list_head,
    dspless_mode_selected: bool_,
    system_suspend_target: c_int,
}

#[repr(C)]
pub struct snd_sof_dsp_ops {
    hw_info: u64,
    pcm_pointer: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

#[repr(C)]
pub struct sof_ipc_tplg_ops {
    host_config: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_widget, *mut snd_sof_platform_stream_params)>,
}

#[repr(C)]
pub struct sof_ipc_pcm_ops {
    hw_free: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    platform_stop_during_hw_free: bool_,
    hw_params: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_sof_platform_stream_params) -> c_int>,
    ipc_first_on_start: bool_,
    d0i3_supported_in_s0ix: bool_,
    reset_hw_params_during_stop: bool_,
    trigger: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_int) -> c_int>,
    pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, *mut snd_pcm_uframes_t) -> c_int>,
    dai_link_fixup: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    delay: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_sframes_t>,
}

unsafe extern "C" {
    static mut system_highpri_wq: *mut c_void;
    static sof_compressed_ops: c_void;

    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtdcom_lookup(rtd: *mut snd_soc_pcm_runtime, name: *const c_char) -> *mut snd_soc_component;
    fn snd_sof_find_spcm_dai(component: *mut snd_soc_component, rtd: *mut snd_soc_pcm_runtime) -> *mut snd_sof_pcm;
    fn queue_work(wq: *mut c_void, work: *mut work_struct) -> bool_;
    fn snd_soc_dapm_dai_get_connected_widgets(dai: *mut snd_soc_dai, dir: c_int, list: *mut *mut snd_soc_dapm_widget_list, walk: *const c_void) -> c_int;
    fn snd_soc_dapm_dai_free_widgets(list: *mut *mut snd_soc_dapm_widget_list);
    fn sof_widget_list_prepare(sdev: *mut snd_sof_dev, spcm: *mut snd_sof_pcm, params: *mut snd_pcm_hw_params, platform_params: *mut snd_sof_platform_stream_params, dir: c_int) -> c_int;
    fn sof_widget_list_free(sdev: *mut snd_sof_dev, spcm: *mut snd_sof_pcm, dir: c_int) -> c_int;
    fn sof_widget_list_unprepare(sdev: *mut snd_sof_dev, spcm: *mut snd_sof_pcm, dir: c_int);
    fn sof_widget_list_setup(sdev: *mut snd_sof_dev, spcm: *mut snd_sof_pcm, params: *mut snd_pcm_hw_params, platform_params: *mut snd_sof_platform_stream_params, dir: c_int) -> c_int;
    fn snd_sof_boot_dsp_firmware(sdev: *mut snd_sof_dev) -> c_int;
    fn snd_sof_pcm_platform_hw_params(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, platform_params: *mut snd_sof_platform_stream_params) -> c_int;
    fn snd_sof_pcm_platform_hw_free(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> c_int;
    fn snd_sof_pcm_platform_trigger(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream, cmd: c_int) -> c_int;
    fn snd_sof_pcm_platform_open(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> c_int;
    fn snd_sof_pcm_platform_close(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> c_int;
    fn snd_sof_pcm_platform_ack(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> c_int;
    fn snd_sof_create_page_table(dev: *mut device, dmab: *mut snd_dma_buffer, area: *mut c_void, size: size_t) -> c_int;
    fn snd_pcm_get_dma_buf(substream: *mut snd_pcm_substream) -> *mut snd_dma_buffer;
    fn cancel_work_sync(work: *mut work_struct) -> bool_;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, size: u64) -> snd_pcm_uframes_t;
    fn trace_sof_pcm_pointer_position(sdev: *mut snd_sof_dev, spcm: *mut snd_sof_pcm, substream: *mut snd_pcm_substream, host: snd_pcm_uframes_t, dai: snd_pcm_uframes_t);
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut snd_sof_dev;
    fn snd_pcm_set_managed_buffer(substream: *mut snd_pcm_substream, ty: c_int, dev: *mut device, size: size_t, max: size_t);
    fn snd_sof_find_dai(component: *mut snd_soc_component, name: *mut c_char) -> *mut snd_sof_dai;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_mask;
    fn snd_mask_none(mask: *mut snd_mask);
    fn snd_mask_set_format(mask: *mut snd_mask, format: c_int);
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device);
    fn devm_kasprintf(dev: *mut device, gfp: c_uint, fmt: *const c_char, ...) -> *const c_char;
    fn snd_sof_load_topology(component: *mut snd_soc_component, filename: *const c_char) -> c_int;
    fn snd_soc_tplg_component_remove(component: *mut snd_soc_component);
    fn sof_ops(sdev: *mut snd_sof_dev) -> *const snd_sof_dsp_ops;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char, count: size_t) -> isize;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn spcm_err(spcm: *mut snd_sof_pcm, dir: c_int, fmt: *const c_char, ...);
    fn spcm_dbg(spcm: *mut snd_sof_pcm, dir: c_int, fmt: *const c_char, ...);
    fn snd_pcm_direction_name(dir: c_int) -> *const c_char;
}

unsafe fn le32_to_cpu(v: u32) -> u32 { v }
unsafe fn le64_to_cpu(v: u64) -> u64 { v }

unsafe fn sof_ipc_get_ops_tplg(_sdev: *mut snd_sof_dev) -> *const sof_ipc_tplg_ops {
    ptr::null()
}

unsafe fn sof_ipc_get_ops_pcm(_sdev: *mut snd_sof_dev) -> *const sof_ipc_pcm_ops {
    ptr::null()
}

/*
 * sof pcm period elapse work
 */
unsafe extern "C" fn snd_sof_pcm_period_elapsed_work(work: *mut work_struct) {
    let sps = work as *mut snd_sof_pcm_stream;

    snd_pcm_period_elapsed((*sps).substream);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_sof_pcm_init_elapsed_work(work: *mut work_struct) {
    /* INIT_WORK(work, snd_sof_pcm_period_elapsed_work); */
    let _ = (work, snd_sof_pcm_period_elapsed_work as unsafe extern "C" fn(*mut work_struct));
}

/*
 * sof pcm period elapse, this could be called at irq thread context.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_sof_pcm_period_elapsed(substream: *mut snd_pcm_substream) {
    let rtd = snd_soc_substream_to_rtd(substream);
    let component = snd_soc_rtdcom_lookup(rtd, SOF_AUDIO_PCM_DRV_NAME.as_ptr() as *const c_char);
    let spcm = snd_sof_find_spcm_dai(component, rtd);

    if spcm.is_null() {
        dev_err((*component).dev, b"error: period elapsed for unknown stream!\n\0".as_ptr() as *const c_char);
        return;
    }

    /*
     * snd_pcm_period_elapsed() can be called in interrupt context
     * before IRQ_HANDLED is returned. Inside snd_pcm_period_elapsed(),
     * when the PCM is done draining or xrun happened, a STOP IPC will
     * then be sent and this IPC will hit IPC timeout.
     * To avoid sending IPC before the previous IPC is handled, we
     * schedule delayed work here to call the snd_pcm_period_elapsed().
     */
    queue_work(system_highpri_wq, &mut (*spcm).stream[(*substream).stream as usize].period_elapsed_work);
}

unsafe extern "C" fn sof_pcm_setup_connected_widgets(
    sdev: *mut snd_sof_dev,
    rtd: *mut snd_soc_pcm_runtime,
    spcm: *mut snd_sof_pcm,
    params: *mut snd_pcm_hw_params,
    platform_params: *mut snd_sof_platform_stream_params,
    dir: c_int,
) -> c_int {
    let mut ret: c_int;
    /* for_each_rtd_cpu_dais(rtd, j, dai) */
    let mut dai: *mut snd_soc_dai = ptr::null_mut();
    let _j: c_int = 0;
    let mut list: *mut snd_soc_dapm_widget_list = ptr::null_mut();

    ret = snd_soc_dapm_dai_get_connected_widgets(dai, dir, &mut list, ptr::null());
    if ret < 0 {
        spcm_err(spcm, dir, b"dai %s has no valid %s path\n\0".as_ptr() as *const c_char,
                 if dai.is_null() { ptr::null() } else { (*dai).name },
                 snd_pcm_direction_name(dir));
        return ret;
    }

    (*spcm).stream[dir as usize].list = list;

    ret = sof_widget_list_prepare(sdev, spcm, params, platform_params, dir);
    if ret < 0 {
        spcm_err(spcm, dir, b"widget list prepare failed\n\0".as_ptr() as *const c_char);
        (*spcm).stream[dir as usize].list = ptr::null_mut();
        snd_soc_dapm_dai_free_widgets(&mut list);
        return ret;
    }

    let _ = rtd;
    0
}

unsafe extern "C" fn snd_sof_find_swidget_by_comp_id(
    sdev: *mut snd_sof_dev,
    comp_id: c_int,
) -> *mut snd_sof_widget {
    /* list_for_each_entry(swidget, &sdev->widget_list, list) */
    let mut swidget = (*sdev).widget_list.next as *mut snd_sof_widget;
    while !swidget.is_null() && swidget as *mut list_head != &mut (*sdev).widget_list {
        if comp_id == (*swidget).comp_id {
            return swidget;
        }
        swidget = (*(swidget as *mut list_head)).next as *mut snd_sof_widget;
    }

    ptr::null_mut()
}

unsafe extern "C" fn sof_pcm_hw_params(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let sdev = snd_soc_component_get_drvdata(component);
    let rtd = snd_soc_substream_to_rtd(substream);
    let tplg_ops = sof_ipc_get_ops_tplg(sdev);
    let pcm_ops = sof_ipc_get_ops_pcm(sdev);
    let runtime = (*substream).runtime;
    let mut host_widget: *mut snd_sof_widget;
    let spcm: *mut snd_sof_pcm;
    let mut ret: c_int;

    /* nothing to do for BE */
    if (*(*rtd).dai_link).no_pcm {
        return 0;
    }

    spcm = snd_sof_find_spcm_dai(component, rtd);
    if spcm.is_null() {
        return -EINVAL;
    }

    spcm_dbg(spcm, (*substream).stream, b"Entry: hw_params\n\0".as_ptr() as *const c_char);

    if !(*sdev).dspless_mode_selected {
        /*
         * Make sure that the DSP is booted up, which might not be the
         * case if the on-demand DSP boot is used
         */
        ret = snd_sof_boot_dsp_firmware(sdev);
        if ret != 0 {
            return ret;
        }
    }

    /*
     * Handle repeated calls to hw_params() without free_pcm() in
     * between. At least ALSA OSS emulation depends on this.
     */
    if (*spcm).prepared[(*substream).stream as usize]
        && !pcm_ops.is_null()
        && (*pcm_ops).hw_free.is_some()
    {
        ret = ((*pcm_ops).hw_free.unwrap())(component, substream);
        if ret < 0 {
            return ret;
        }

        (*spcm).prepared[(*substream).stream as usize] = false;
    }

    let platform_params = &mut (*spcm).platform_params[(*substream).stream as usize];
    ret = snd_sof_pcm_platform_hw_params(sdev, substream, params, platform_params);
    if ret < 0 {
        spcm_err(spcm, (*substream).stream, b"platform hw params failed\n\0".as_ptr() as *const c_char);
        return ret;
    }

    /* if this is a repeated hw_params without hw_free, skip setting up widgets */
    if (*spcm).stream[(*substream).stream as usize].list.is_null() {
        ret = sof_pcm_setup_connected_widgets(sdev, rtd, spcm, params, platform_params, (*substream).stream);
        if ret < 0 {
            return ret;
        }
    }

    if !(*sdev).dspless_mode_selected {
        let host_comp_id = (*spcm).stream[(*substream).stream as usize].comp_id;

        host_widget = snd_sof_find_swidget_by_comp_id(sdev, host_comp_id);
        if host_widget.is_null() {
            spcm_err(spcm, (*substream).stream,
                     b"failed to find host widget with comp_id %d\n\0".as_ptr() as *const c_char,
                     host_comp_id);
            return -EINVAL;
        }

        /* set the host DMA ID */
        if !tplg_ops.is_null() {
            if let Some(host_config) = (*tplg_ops).host_config {
                host_config(sdev, host_widget, platform_params);
            }
        }
    }

    /* create compressed page table for audio firmware */
    if (*runtime).buffer_changed {
        let dmab = snd_pcm_get_dma_buf(substream);

        ret = snd_sof_create_page_table(
            (*component).dev,
            dmab,
            (*spcm).stream[(*substream).stream as usize].page_table.area,
            (*runtime).dma_bytes,
        );
        if ret < 0 {
            return ret;
        }
    }

    /* save pcm hw_params */
    memcpy(
        &mut (*spcm).params[(*substream).stream as usize] as *mut _ as *mut c_void,
        params as *const c_void,
        size_of::<snd_pcm_hw_params>(),
    );

    0
}

unsafe extern "C" fn sof_pcm_stream_free(
    sdev: *mut snd_sof_dev,
    substream: *mut snd_pcm_substream,
    spcm: *mut snd_sof_pcm,
    dir: c_int,
    free_widget_list: bool_,
) -> c_int {
    let pcm_ops = sof_ipc_get_ops_pcm(sdev);
    let mut ret: c_int;
    let mut err: c_int = 0;

    if (*spcm).prepared[(*substream).stream as usize] {
        /* stop DMA first if needed */
        if !pcm_ops.is_null() && (*pcm_ops).platform_stop_during_hw_free {
            snd_sof_pcm_platform_trigger(sdev, substream, SNDRV_PCM_TRIGGER_STOP);
        }

        /* free PCM in the DSP */
        if !pcm_ops.is_null() {
            if let Some(hw_free) = (*pcm_ops).hw_free {
                ret = hw_free((*sdev).component, substream);
                if ret < 0 {
                    spcm_err(spcm, (*substream).stream,
                             b"pcm_ops->hw_free failed %d\n\0".as_ptr() as *const c_char, ret);
                    err = ret;
                }
            }
        }

        (*spcm).prepared[(*substream).stream as usize] = false;
        (*spcm).pending_stop[(*substream).stream as usize] = false;
    }

    /* reset the DMA */
    ret = snd_sof_pcm_platform_hw_free(sdev, substream);
    if ret < 0 {
        spcm_err(spcm, (*substream).stream,
                 b"platform hw free failed %d\n\0".as_ptr() as *const c_char, ret);
        if err == 0 {
            err = ret;
        }
    }

    /* free widget list */
    if free_widget_list {
        ret = sof_widget_list_free(sdev, spcm, dir);
        if ret < 0 {
            spcm_err(spcm, (*substream).stream,
                     b"sof_widget_list_free failed %d\n\0".as_ptr() as *const c_char, ret);
            if err == 0 {
                err = ret;
            }
        }
    }

    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sof_pcm_free_all_streams(sdev: *mut snd_sof_dev) -> c_int {
    let mut ret: c_int;

    /* list_for_each_entry(spcm, &sdev->pcm_list, list) */
    let mut spcm = (*sdev).pcm_list.next as *mut snd_sof_pcm;
    while !spcm.is_null() && spcm as *mut list_head != &mut (*sdev).pcm_list {
        for dir in 0..2 {
            let substream = (*spcm).stream[dir].substream;

            if substream.is_null()
                || (*substream).runtime.is_null()
                || (*spcm).stream[dir].suspend_ignored
            {
                continue;
            }

            if !(*spcm).stream[dir].list.is_null() {
                ret = sof_pcm_stream_free(sdev, substream, spcm, dir as c_int, true);
                if ret < 0 {
                    return ret;
                }
            }
        }
        spcm = (*(spcm as *mut list_head)).next as *mut snd_sof_pcm;
    }

    0
}

unsafe extern "C" fn sof_pcm_hw_free(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let sdev = snd_soc_component_get_drvdata(component);
    let spcm: *mut snd_sof_pcm;
    let ret: c_int;

    /* nothing to do for BE */
    if (*(*rtd).dai_link).no_pcm {
        return 0;
    }

    spcm = snd_sof_find_spcm_dai(component, rtd);
    if spcm.is_null() {
        return -EINVAL;
    }

    spcm_dbg(spcm, (*substream).stream, b"Entry: hw_free\n\0".as_ptr() as *const c_char);

    ret = sof_pcm_stream_free(sdev, substream, spcm, (*substream).stream, true);

    /* unprepare and free the list of DAPM widgets */
    sof_widget_list_unprepare(sdev, spcm, (*substream).stream);

    cancel_work_sync(&mut (*spcm).stream[(*substream).stream as usize].period_elapsed_work);

    ret
}

unsafe extern "C" fn sof_pcm_prepare(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let sdev = snd_soc_component_get_drvdata(component);
    let pcm_ops = sof_ipc_get_ops_pcm(sdev);
    let spcm: *mut snd_sof_pcm;
    let dir = (*substream).stream;
    let mut ret: c_int;

    /* nothing to do for BE */
    if (*(*rtd).dai_link).no_pcm {
        return 0;
    }

    spcm = snd_sof_find_spcm_dai(component, rtd);
    if spcm.is_null() {
        return -EINVAL;
    }

    spcm_dbg(spcm, (*substream).stream, b"Entry: prepare\n\0".as_ptr() as *const c_char);

    if (*spcm).prepared[(*substream).stream as usize] {
        if !(*spcm).pending_stop[(*substream).stream as usize] {
            return 0;
        }

        /*
         * this case should be reached in case of xruns where we absolutely
         * want to free-up and reset all PCM/DMA resources
         */
        ret = sof_pcm_stream_free(sdev, substream, spcm, (*substream).stream, true);
        if ret < 0 {
            return ret;
        }
    }

    ret = sof_pcm_hw_params(component, substream, &mut (*spcm).params[(*substream).stream as usize]);
    if ret < 0 {
        spcm_err(spcm, (*substream).stream,
                 b"failed to set hw_params after resume\n\0".as_ptr() as *const c_char);
        return ret;
    }

    let mut list = (*spcm).stream[dir as usize].list;
    let params = &mut (*spcm).params[(*substream).stream as usize];
    let platform_params = &mut (*spcm).platform_params[(*substream).stream as usize];
    ret = sof_widget_list_setup(sdev, spcm, params, platform_params, dir);
    if ret < 0 {
        dev_err((*sdev).dev, b"failed widget list set up for pcm %d dir %u\n\0".as_ptr() as *const c_char,
                le32_to_cpu((*spcm).pcm.pcm_id), dir);
        (*spcm).stream[dir as usize].list = ptr::null_mut();
        snd_soc_dapm_dai_free_widgets(&mut list);
        return ret;
    }

    if !pcm_ops.is_null() {
        if let Some(hw_params) = (*pcm_ops).hw_params {
            ret = hw_params(component, substream, params, platform_params);
            if ret < 0 {
                return ret;
            }
        }
    }

    (*spcm).prepared[(*substream).stream as usize] = true;

    0
}

/*
 * FE dai link trigger actions are always executed in non-atomic context because
 * they involve IPC's.
 */
unsafe extern "C" fn sof_pcm_trigger(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let sdev = snd_soc_component_get_drvdata(component);
    let pcm_ops = sof_ipc_get_ops_pcm(sdev);
    let spcm: *mut snd_sof_pcm;
    let mut reset_hw_params = false;
    let mut ipc_first = false;
    let mut ret: c_int = 0;

    /* nothing to do for BE */
    if (*(*rtd).dai_link).no_pcm {
        return 0;
    }

    spcm = snd_sof_find_spcm_dai(component, rtd);
    if spcm.is_null() {
        return -EINVAL;
    }

    spcm_dbg(spcm, (*substream).stream, b"Entry: trigger (cmd: %d)\n\0".as_ptr() as *const c_char, cmd);

    (*spcm).pending_stop[(*substream).stream as usize] = false;

    match cmd {
        SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            ipc_first = true;
        }
        SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            if !pcm_ops.is_null() && (*pcm_ops).ipc_first_on_start {
                ipc_first = true;
            }
        }
        SNDRV_PCM_TRIGGER_START => {
            if (*spcm).stream[(*substream).stream as usize].suspend_ignored {
                /*
                 * This case will be triggered when INFO_RESUME is
                 * not supported, no need to re-start streams that
                 * remained enabled in D0ix.
                 */
                (*spcm).stream[(*substream).stream as usize].suspend_ignored = false;
                return 0;
            }

            if !pcm_ops.is_null() && (*pcm_ops).ipc_first_on_start {
                ipc_first = true;
            }
        }
        SNDRV_PCM_TRIGGER_SUSPEND => {
            /*
             * If DSP D0I3 is allowed during S0iX, set the suspend_ignored flag for
             * D0I3-compatible streams to keep the firmware pipeline running
             */
            if !pcm_ops.is_null()
                && (*pcm_ops).d0i3_supported_in_s0ix
                && (*sdev).system_suspend_target == SOF_SUSPEND_S0IX
                && (*spcm).stream[(*substream).stream as usize].d0i3_compatible
            {
                (*spcm).stream[(*substream).stream as usize].suspend_ignored = true;
                return 0;
            }

            /* On suspend the DMA must be stopped in DSPless mode */
            if (*sdev).dspless_mode_selected {
                reset_hw_params = true;
            }

            ipc_first = true;
            if !pcm_ops.is_null() && (*pcm_ops).reset_hw_params_during_stop {
                reset_hw_params = true;
            }
        }
        SNDRV_PCM_TRIGGER_STOP => {
            ipc_first = true;
            if !pcm_ops.is_null() && (*pcm_ops).reset_hw_params_during_stop {
                reset_hw_params = true;
            }
        }
        _ => {
            spcm_err(spcm, (*substream).stream, b"Unhandled trigger cmd %d\n\0".as_ptr() as *const c_char, cmd);
            return -EINVAL;
        }
    }

    if !ipc_first {
        snd_sof_pcm_platform_trigger(sdev, substream, cmd);
    }

    if !pcm_ops.is_null() {
        if let Some(trigger) = (*pcm_ops).trigger {
            ret = trigger(component, substream, cmd);
        }
    }

    match cmd {
        SNDRV_PCM_TRIGGER_PAUSE_RELEASE | SNDRV_PCM_TRIGGER_START => {
            /* invoke platform trigger to start DMA only if pcm_ops is successful */
            if ipc_first && ret == 0 {
                snd_sof_pcm_platform_trigger(sdev, substream, cmd);
            }
        }
        SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_STOP => {
            /* invoke platform trigger to stop DMA even if pcm_ops isn't set or if it failed */
            if pcm_ops.is_null() || !(*pcm_ops).platform_stop_during_hw_free {
                snd_sof_pcm_platform_trigger(sdev, substream, cmd);
            }

            /*
             * set the pending_stop flag to indicate that pipeline stop has been delayed.
             * This will be used later to stop the pipelines during prepare when recovering
             * from xruns.
             */
            if !pcm_ops.is_null()
                && (*pcm_ops).platform_stop_during_hw_free
                && cmd == SNDRV_PCM_TRIGGER_STOP
            {
                (*spcm).pending_stop[(*substream).stream as usize] = true;
            }
        }
        _ => {}
    }

    /* free PCM if reset_hw_params is set and the STOP IPC is successful */
    if ret == 0 && reset_hw_params {
        ret = sof_pcm_stream_free(sdev, substream, spcm, (*substream).stream, false);
    }

    ret
}

unsafe extern "C" fn sof_pcm_pointer(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let rtd = snd_soc_substream_to_rtd(substream);
    let sdev = snd_soc_component_get_drvdata(component);
    let pcm_ops = sof_ipc_get_ops_pcm(sdev);
    let spcm: *mut snd_sof_pcm;
    let mut host: snd_pcm_uframes_t = 0;
    let dai: snd_pcm_uframes_t;
    let mut ret: c_int = -EOPNOTSUPP;

    /* nothing to do for BE */
    if (*(*rtd).dai_link).no_pcm {
        return 0;
    }

    if !pcm_ops.is_null() {
        if let Some(pointer) = (*pcm_ops).pointer {
            ret = pointer(component, substream, &mut host);
        }
    }

    if ret != -EOPNOTSUPP {
        return if ret != 0 { ret as snd_pcm_uframes_t } else { host };
    }

    /* use dsp ops pointer callback directly if set */
    if let Some(pcm_pointer) = (*sof_ops(sdev)).pcm_pointer {
        return pcm_pointer(sdev, substream);
    }

    spcm = snd_sof_find_spcm_dai(component, rtd);
    if spcm.is_null() {
        return (-EINVAL) as snd_pcm_uframes_t;
    }

    /* read position from DSP */
    host = bytes_to_frames((*substream).runtime, (*spcm).stream[(*substream).stream as usize].posn.host_posn);
    dai = bytes_to_frames((*substream).runtime, (*spcm).stream[(*substream).stream as usize].posn.dai_posn);

    trace_sof_pcm_pointer_position(sdev, spcm, substream, host, dai);

    host
}

unsafe extern "C" fn sof_pcm_open(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let runtime = (*substream).runtime;
    let sdev = snd_soc_component_get_drvdata(component);
    let ops = sof_ops(sdev);
    let spcm: *mut snd_sof_pcm;
    let caps: *mut snd_soc_tplg_stream_caps;
    let ret: c_int;

    /* nothing to do for BE */
    if (*(*rtd).dai_link).no_pcm {
        return 0;
    }

    spcm = snd_sof_find_spcm_dai(component, rtd);
    if spcm.is_null() {
        return -EINVAL;
    }

    spcm_dbg(spcm, (*substream).stream, b"Entry: open\n\0".as_ptr() as *const c_char);

    caps = &mut (*spcm).pcm.caps[(*substream).stream as usize];

    /* set runtime config */
    (*runtime).hw.info = (*ops).hw_info; /* platform-specific */

    /* set any runtime constraints based on topology */
    (*runtime).hw.formats = le64_to_cpu((*caps).formats);
    (*runtime).hw.period_bytes_min = le32_to_cpu((*caps).period_size_min) as size_t;
    (*runtime).hw.period_bytes_max = le32_to_cpu((*caps).period_size_max) as size_t;
    (*runtime).hw.periods_min = le32_to_cpu((*caps).periods_min);
    (*runtime).hw.periods_max = le32_to_cpu((*caps).periods_max);

    /*
     * caps->buffer_size_min is not used since the
     * snd_pcm_hardware structure only defines buffer_bytes_max
     */
    (*runtime).hw.buffer_bytes_max = le32_to_cpu((*caps).buffer_size_max) as size_t;

    /* set wait time - TODO: come from topology */
    (*substream).wait_time = 500;

    (*spcm).stream[(*substream).stream as usize].posn.host_posn = 0;
    (*spcm).stream[(*substream).stream as usize].posn.dai_posn = 0;
    (*spcm).stream[(*substream).stream as usize].substream = substream;
    (*spcm).prepared[(*substream).stream as usize] = false;

    ret = snd_sof_pcm_platform_open(sdev, substream);
    if ret < 0 {
        spcm_err(spcm, (*substream).stream,
                 b"platform pcm open failed %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    spcm_dbg(spcm, (*substream).stream, b"period bytes min %zd, max %zd\n\0".as_ptr() as *const c_char,
             (*runtime).hw.period_bytes_min, (*runtime).hw.period_bytes_max);
    spcm_dbg(spcm, (*substream).stream, b"period count min %d, max %d\n\0".as_ptr() as *const c_char,
             (*runtime).hw.periods_min, (*runtime).hw.periods_max);
    spcm_dbg(spcm, (*substream).stream, b"buffer bytes max %zd\n\0".as_ptr() as *const c_char,
             (*runtime).hw.buffer_bytes_max);

    0
}

unsafe extern "C" fn sof_pcm_close(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let sdev = snd_soc_component_get_drvdata(component);
    let spcm: *mut snd_sof_pcm;
    let err: c_int;

    /* nothing to do for BE */
    if (*(*rtd).dai_link).no_pcm {
        return 0;
    }

    spcm = snd_sof_find_spcm_dai(component, rtd);
    if spcm.is_null() {
        return -EINVAL;
    }

    spcm_dbg(spcm, (*substream).stream, b"Entry: close\n\0".as_ptr() as *const c_char);

    err = snd_sof_pcm_platform_close(sdev, substream);
    if err < 0 {
        spcm_err(spcm, (*substream).stream,
                 b"platform pcm close failed %d\n\0".as_ptr() as *const c_char, err);
        /*
         * keep going, no point in preventing the close
         * from happening
         */
    }

    (*spcm).stream[(*substream).stream as usize].substream = ptr::null_mut();

    0
}

/*
 * Pre-allocate playback/capture audio buffer pages.
 * no need to explicitly release memory preallocated by sof_pcm_new in pcm_free
 * snd_pcm_lib_preallocate_free_for_all() is called by the core.
 */
unsafe extern "C" fn sof_pcm_new(
    component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    let sdev = snd_soc_component_get_drvdata(component);
    let spcm: *mut snd_sof_pcm;
    let pcm = (*rtd).pcm;
    let mut caps: *mut snd_soc_tplg_stream_caps;
    let mut stream = SNDRV_PCM_STREAM_PLAYBACK;

    /* find SOF PCM for this RTD */
    spcm = snd_sof_find_spcm_dai(component, rtd);
    if spcm.is_null() {
        dev_warn((*component).dev, b"warn: can't find PCM with DAI ID %d\n\0".as_ptr() as *const c_char,
                 (*(*rtd).dai_link).id);
        return 0;
    }

    dev_dbg((*(*spcm).scomp).dev, b"pcm%u (%s): Entry: pcm_new\n\0".as_ptr() as *const c_char,
            le32_to_cpu((*spcm).pcm.pcm_id), (*spcm).pcm.pcm_name);

    /* do we need to pre-allocate playback audio buffer pages */
    if (*spcm).pcm.playback {
        caps = &mut (*spcm).pcm.caps[stream as usize];

        if (*pcm).streams[stream as usize].substream.is_null() {
            spcm_err(spcm, stream, b"NULL playback substream!\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }

        /* pre-allocate playback audio buffer pages */
        spcm_dbg(spcm, stream, b"allocate %s playback DMA buffer size 0x%x max 0x%x\n\0".as_ptr() as *const c_char,
                 (*caps).name, (*caps).buffer_size_min, (*caps).buffer_size_max);

        snd_pcm_set_managed_buffer(
            (*pcm).streams[stream as usize].substream,
            SNDRV_DMA_TYPE_DEV_SG,
            (*sdev).dev,
            0,
            le32_to_cpu((*caps).buffer_size_max) as size_t,
        );

        /* Set the PCM device name for HDMI playback */
        if strncmp((*pcm).id.as_ptr(), b"HDMI\0".as_ptr() as *const c_char, 4) == 0 {
            let mut hdmi_idx: c_int = 0;

            /*
             * Make sure that the name is in"HDMI<SPACE>x" format as this is
             * expected by user space.
             * See alsa-lib's __snd_pcm_info_eld_fixup_check() which is
             * guarding the __snd_pcm_info_eld_fixup() in
             * snd_ctl_hw_pcm_info() and snd_pcm_hw_info() library functions
             */
            if sscanf((*pcm).id.as_ptr(), b"HDMI%d\0".as_ptr() as *const c_char, &mut hdmi_idx) == 1 {
                snprintf((*pcm).name.as_mut_ptr(), (*pcm).name.len(), b"HDMI %d\0".as_ptr() as *const c_char,
                         hdmi_idx);
            } else {
                strscpy((*pcm).name.as_mut_ptr(), (*pcm).id.as_ptr(), (*pcm).name.len());
            }
        }
    }

    stream = SNDRV_PCM_STREAM_CAPTURE;

    /* do we need to pre-allocate capture audio buffer pages */
    if !(*spcm).pcm.capture {
        return 0;
    }

    caps = &mut (*spcm).pcm.caps[stream as usize];

    if (*pcm).streams[stream as usize].substream.is_null() {
        spcm_err(spcm, stream, b"NULL capture substream!\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    /* pre-allocate capture audio buffer pages */
    spcm_dbg(spcm, stream, b"allocate %s capture DMA buffer size 0x%x max 0x%x\n\0".as_ptr() as *const c_char,
             (*caps).name, (*caps).buffer_size_min, (*caps).buffer_size_max);

    snd_pcm_set_managed_buffer(
        (*pcm).streams[stream as usize].substream,
        SNDRV_DMA_TYPE_DEV_SG,
        (*sdev).dev,
        0,
        le32_to_cpu((*caps).buffer_size_max) as size_t,
    );

    0
}

/* fixup the BE DAI link to match any values from topology */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sof_pcm_dai_link_fixup(
    rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rate = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let channels = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let fmt = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);
    let component = snd_soc_rtdcom_lookup(rtd, SOF_AUDIO_PCM_DRV_NAME.as_ptr() as *const c_char);
    let dai = snd_sof_find_dai(component, (*(*rtd).dai_link).name as *mut c_char);
    let sdev = snd_soc_component_get_drvdata(component);
    let pcm_ops = sof_ipc_get_ops_pcm(sdev);

    /* no topology exists for this BE, try a common configuration */
    if dai.is_null() {
        dev_warn((*component).dev,
                 b"warning: no topology found for BE DAI %s config\n\0".as_ptr() as *const c_char,
                 (*(*rtd).dai_link).name);

        /*  set 48k, stereo, 16bits by default */
        (*rate).min = 48000;
        (*rate).max = 48000;

        (*channels).min = 2;
        (*channels).max = 2;

        snd_mask_none(fmt);
        snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S16_LE);

        return 0;
    }

    if !pcm_ops.is_null() {
        if let Some(dai_link_fixup) = (*pcm_ops).dai_link_fixup {
            return dai_link_fixup(rtd, params);
        }
    }

    0
}

unsafe extern "C" fn sof_pcm_probe(component: *mut snd_soc_component) -> c_int {
    let sdev = snd_soc_component_get_drvdata(component);
    let plat_data = (*sdev).pdata;
    let tplg_filename: *const c_char;
    let mut ret: c_int;

    /*
     * make sure the device is pm_runtime_active before loading the
     * topology and initiating IPC or bus transactions
     */
    ret = pm_runtime_resume_and_get((*component).dev);
    if ret < 0 && ret != -EACCES {
        return ret;
    }

    /* load the default topology */
    (*sdev).component = component;

    tplg_filename = devm_kasprintf(
        (*sdev).dev,
        GFP_KERNEL,
        b"%s/%s\0".as_ptr() as *const c_char,
        (*plat_data).tplg_filename_prefix,
        (*plat_data).tplg_filename,
    );
    if tplg_filename.is_null() {
        ret = -ENOMEM;
        pm_runtime_put_autosuspend((*component).dev);
        return ret;
    }

    ret = snd_sof_load_topology(component, tplg_filename);
    if ret < 0 {
        dev_err((*component).dev, b"error: failed to load DSP topology %d\n\0".as_ptr() as *const c_char, ret);
    }

    pm_runtime_put_autosuspend((*component).dev);

    ret
}

unsafe extern "C" fn sof_pcm_remove(component: *mut snd_soc_component) {
    /* remove topology */
    snd_soc_tplg_component_remove(component);
}

unsafe extern "C" fn sof_pcm_ack(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let sdev = snd_soc_component_get_drvdata(component);

    snd_sof_pcm_platform_ack(sdev, substream)
}

unsafe extern "C" fn sof_pcm_delay(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_sframes_t {
    let sdev = snd_soc_component_get_drvdata(component);
    let pcm_ops = sof_ipc_get_ops_pcm(sdev);

    if !pcm_ops.is_null() {
        if let Some(delay) = (*pcm_ops).delay {
            return delay(component, substream);
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_sof_new_platform_drv(sdev: *mut snd_sof_dev) {
    let pd = &mut (*sdev).plat_drv;
    let plat_data = (*sdev).pdata;
    let drv_name: *const c_char;

    if !(*plat_data).machine.is_null() {
        drv_name = (*(*plat_data).machine).drv_name;
    } else if !(*plat_data).of_machine.is_null() {
        drv_name = (*(*plat_data).of_machine).drv_name;
    } else {
        drv_name = ptr::null();
    }

    pd.name = b"sof-audio-component\0".as_ptr() as *const c_char;
    pd.probe = Some(sof_pcm_probe);
    pd.remove = Some(sof_pcm_remove);
    pd.open = Some(sof_pcm_open);
    pd.close = Some(sof_pcm_close);
    pd.hw_params = Some(sof_pcm_hw_params);
    pd.prepare = Some(sof_pcm_prepare);
    pd.hw_free = Some(sof_pcm_hw_free);
    pd.trigger = Some(sof_pcm_trigger);
    pd.pointer = Some(sof_pcm_pointer);
    pd.ack = Some(sof_pcm_ack);
    pd.delay = Some(sof_pcm_delay);

    /*
     * C conditional:
     * #if IS_ENABLED(CONFIG_SND_SOC_SOF_COMPRESS)
     *     pd->compress_ops = &sof_compressed_ops;
     * #endif
     */
    pd.compress_ops = &sof_compressed_ops as *const _ as *const c_void;

    pd.pcm_new = Some(sof_pcm_new);
    pd.ignore_machine = drv_name;
    pd.be_pcm_base = SOF_BE_PCM_BASE;
    pd.use_dai_pcm_id = true;
    pd.topology_name_prefix = b"sof\0".as_ptr() as *const c_char;

    /* increment module refcount when a pcm is opened */
    pd.module_get_upon_open = 1;

    pd.legacy_dai_naming = 1;

    /*
     * The fixup is only needed when the DSP is in use as with the DSPless
     * mode we are directly using the audio interface
     */
    if !(*sdev).dspless_mode_selected {
        pd.be_hw_params_fixup = Some(sof_pcm_dai_link_fixup);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
