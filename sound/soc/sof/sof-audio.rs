// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2019 Intel Corporation
//
// Author: Ranjani Sridharan <ranjani.sridharan@linux.intel.com>
//

// Rust translation of soc/sof/sof-audio.c.
// C include dependencies:
// linux/bitfield.h, trace/events/sof.h, sof-audio.h, ops.h

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;

    fn sof_ipc_get_ops(sdev: *mut snd_sof_dev, ops: SofIpcOpsKind) -> *const sof_ipc_tplg_ops;
    fn snd_sof_dsp_core_put(sdev: *mut snd_sof_dev, core: c_int) -> c_int;
    fn snd_sof_dsp_core_get(sdev: *mut snd_sof_dev, core: c_int) -> c_int;
    fn snd_soc_component_get_drvdata(scomp: *mut snd_soc_component) -> *mut snd_sof_dev;
    fn snd_soc_rtdcom_lookup(
        rtd: *mut snd_soc_pcm_runtime,
        name: *const c_char,
    ) -> *mut snd_soc_component;
    fn snd_soc_dapm_dai_free_widgets(list: *mut *mut snd_soc_dapm_widget_list);

    fn trace_sof_widget_free(swidget: *mut snd_sof_widget);
    fn trace_sof_widget_setup(swidget: *mut snd_sof_widget);
    fn dev_dbg(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);

    fn WIDGET_IS_DAI(id: snd_soc_dapm_type) -> bool;
    fn widget_in_list(
        list: *mut snd_soc_dapm_widget_list,
        widget: *mut snd_soc_dapm_widget,
    ) -> bool;
    fn isdigit(c: c_int) -> c_int;
}

extern "C" {
    static SOF_AUDIO_PCM_DRV_NAME: *const c_char;
}

const EINVAL: c_int = 22;
const DMA_CHAN_INVALID: c_uint = !0;
const SOF_DAI_CONFIG_FLAGS_HW_FREE: c_uint = 1;
const SOF_DAI_CONFIG_FLAGS_HW_PARAMS: c_uint = 2;
const SOF_DAI_PARAM_INTEL_SSP_MCLK: c_int = 0;
const SOF_DAI_PARAM_INTEL_SSP_BCLK: c_int = 1;
const SOF_DAI_PARAM_INTEL_SSP_TDM_SLOTS: c_int = 2;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;

#[repr(C)]
pub struct snd_sof_dev {
    pub dev: *mut c_void,
    pub route_list: list_head,
    pub pcm_list: list_head,
    pub widget_list: list_head,
    pub dai_list: list_head,
    pub num_cores: c_int,
    pub dspless_mode_selected: bool,
}

#[repr(C)]
pub struct snd_sof_widget {
    pub list: list_head,
    pub id: snd_soc_dapm_type,
    pub widget: *mut snd_soc_dapm_widget,
    pub private: *mut c_void,
    pub use_count: c_int,
    pub prepared: bool,
    pub spipe: *mut snd_sof_pipeline,
    pub dynamic_pipeline_widget: bool,
    pub setup_mutex: mutex,
}

#[repr(C)]
pub struct snd_sof_pipeline {
    pub core_mask: c_ulong,
    pub complete: c_int,
    pub pipe_widget: *mut snd_sof_widget,
    pub direction: c_int,
    pub direction_valid: bool,
}

#[repr(C)]
pub struct snd_sof_route {
    pub list: list_head,
    pub src_widget: *mut snd_sof_widget,
    pub sink_widget: *mut snd_sof_widget,
    pub setup: bool,
}

#[repr(C)]
pub struct snd_sof_pcm {
    pub list: list_head,
    pub pcm: sof_ipc_pcm,
    pub stream: [snd_sof_pcm_stream; 2],
    pub setup_done: [bool; 2],
}

#[repr(C)]
pub struct snd_sof_pcm_stream {
    pub list: *mut snd_soc_dapm_widget_list,
    pub pipeline_list: snd_sof_pcm_stream_pipeline_list,
    pub substream: *mut snd_pcm_substream,
    pub d0i3_compatible: bool,
    pub suspend_ignored: bool,
    pub comp_id: c_uint,
}

#[repr(C)]
pub struct snd_sof_pcm_stream_pipeline_list {
    pub pipelines: *mut *mut snd_sof_pipeline,
    pub count: c_int,
}

#[repr(C)]
pub struct sof_ipc_pcm {
    pub dai_name: *const c_char,
    pub caps: [sof_ipc_pcm_caps; 2],
}

#[repr(C)]
pub struct sof_ipc_pcm_caps {
    pub name: [c_char; 0],
}

#[repr(C)]
pub struct snd_sof_dai {
    pub list: list_head,
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub id: snd_soc_dapm_type,
    pub name: *const c_char,
    pub sname: *const c_char,
    pub dobj: snd_soc_dobj,
}

#[repr(C)]
pub struct snd_soc_dobj {
    pub private: *mut c_void,
}

#[repr(C)]
pub struct snd_soc_dapm_widget_list {
    pub num_widgets: c_int,
    pub widgets: *mut *mut snd_soc_dapm_widget,
}

#[repr(C)]
pub struct snd_soc_dapm_path {
    pub source: *mut snd_soc_dapm_widget,
    pub sink: *mut snd_soc_dapm_widget,
    pub walking: bool,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_platform_stream_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dai_link: *mut snd_soc_dai_link,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut c_void,
}

#[repr(C)]
pub struct sof_ipc_tplg_ops {
    pub route_free: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_route)>,
    pub dai_config: Option<
        unsafe extern "C" fn(
            *mut snd_sof_dev,
            *mut snd_sof_widget,
            c_uint,
            *mut snd_sof_dai_config_data,
        ) -> c_int,
    >,
    pub widget_free: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_widget) -> c_int>,
    pub widget_setup: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_widget) -> c_int>,
    pub control: *const sof_ipc_tplg_control_ops,
    pub widget: *const sof_ipc_tplg_widget_ops,
    pub route_setup: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_route) -> c_int>,
    pub pipeline_complete:
        Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_widget) -> c_int>,
    pub dai_get_param: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_dai, c_int) -> c_int>,
}

#[repr(C)]
pub struct sof_ipc_tplg_control_ops {
    pub widget_kcontrol_setup:
        Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_widget) -> c_int>,
}

#[repr(C)]
pub struct sof_ipc_tplg_widget_ops {
    pub ipc_prepare: Option<
        unsafe extern "C" fn(
            *mut snd_sof_widget,
            *mut snd_pcm_hw_params,
            *mut snd_sof_platform_stream_params,
            *mut snd_pcm_hw_params,
            c_int,
        ) -> c_int,
    >,
    pub ipc_unprepare: Option<unsafe extern "C" fn(*mut snd_sof_widget)>,
}

#[repr(C)]
pub struct snd_sof_dai_config_data {
    pub dai_data: c_uint,
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_dapm_type {
    snd_soc_dapm_out_drv = 0,
    snd_soc_dapm_output = 1,
    snd_soc_dapm_input = 2,
    snd_soc_dapm_scheduler = 3,
    snd_soc_dapm_aif_in = 4,
    snd_soc_dapm_aif_out = 5,
    snd_soc_dapm_dai_out = 6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_widget_op {
    SOF_WIDGET_SETUP = 0,
    SOF_WIDGET_FREE = 1,
    SOF_WIDGET_PREPARE = 2,
    SOF_WIDGET_UNPREPARE = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SofIpcOpsKind {
    tplg = 0,
}

type c_ulong = usize;

// External Linux list/DAPM/bitmap iteration macros are preserved as Rust macro calls.
// They are expected to be supplied by the surrounding translation unit/bindings.

/*
 * Check if a DAI widget is an aggregated DAI. Aggregated DAI's have names ending in numbers
 * starting with 0. For example: in the case of a SDW speaker with 2 amps, the topology contains
 * 2 DAI's names alh-copier.SDW1.Playback.0 and alh-copier-SDW1.Playback.1. In this case, only the
 * DAI alh-copier.SDW1.Playback.0 is set up in the firmware. The other DAI,
 * alh-copier.SDW1.Playback.1 in topology is for the sake of completeness to show aggregation for
 * the speaker amp and does not need any firmware configuration.
 */
unsafe fn is_aggregated_dai(swidget: *mut snd_sof_widget) -> bool {
    let name = (*(*swidget).widget).name;
    let last = *name.add(strlen(name).wrapping_sub(1));

    WIDGET_IS_DAI((*swidget).id) && isdigit(last as c_int) != 0 && last != b'0' as c_char
}

unsafe fn is_virtual_widget(
    sdev: *mut snd_sof_dev,
    widget: *mut snd_soc_dapm_widget,
    func: *const c_char,
) -> bool {
    match (*widget).id {
        snd_soc_dapm_type::snd_soc_dapm_out_drv
        | snd_soc_dapm_type::snd_soc_dapm_output
        | snd_soc_dapm_type::snd_soc_dapm_input => {
            dev_dbg(
                (*sdev).dev,
                b"%s: %s is a virtual widget\n\0".as_ptr() as *const c_char,
                func,
                (*widget).name,
            );
            true
        }
        _ => false,
    }
}

unsafe fn sof_reset_route_setup_status(sdev: *mut snd_sof_dev, widget: *mut snd_sof_widget) {
    let tplg_ops = sof_ipc_get_ops(sdev, SofIpcOpsKind::tplg);
    let mut sroute: *mut snd_sof_route;

    list_for_each_entry!(sroute, &mut (*sdev).route_list, list, {
        if (*sroute).src_widget == widget || (*sroute).sink_widget == widget {
            if (*sroute).setup && !tplg_ops.is_null() {
                if let Some(route_free) = (*tplg_ops).route_free {
                    route_free(sdev, sroute);
                }
            }

            (*sroute).setup = false;
        }
    });
}

unsafe fn sof_widget_free_unlocked(
    sdev: *mut snd_sof_dev,
    swidget: *mut snd_sof_widget,
) -> c_int {
    let tplg_ops = sof_ipc_get_ops(sdev, SofIpcOpsKind::tplg);
    let spipe = (*swidget).spipe;
    let mut err: c_int = 0;
    let mut ret: c_int;

    if (*swidget).private.is_null() {
        return 0;
    }

    trace_sof_widget_free(swidget);

    /* only free when use_count is 0 */
    (*swidget).use_count -= 1;
    if (*swidget).use_count != 0 {
        return 0;
    }

    /* reset route setup status for all routes that contain this widget */
    sof_reset_route_setup_status(sdev, swidget);

    /* free DAI config and continue to free widget even if it fails */
    if WIDGET_IS_DAI((*swidget).id) {
        let mut data = snd_sof_dai_config_data {
            dai_data: DMA_CHAN_INVALID,
        };
        let flags: c_uint = SOF_DAI_CONFIG_FLAGS_HW_FREE;

        if !tplg_ops.is_null() {
            if let Some(dai_config) = (*tplg_ops).dai_config {
                err = dai_config(sdev, swidget, flags, &mut data);
                if err < 0 {
                    dev_err(
                        (*sdev).dev,
                        b"failed to free config for widget %s\n\0".as_ptr() as *const c_char,
                        (*(*swidget).widget).name,
                    );
                }
            }
        }
    }

    /* continue to disable core even if IPC fails */
    if !tplg_ops.is_null() {
        if let Some(widget_free) = (*tplg_ops).widget_free {
            ret = widget_free(sdev, swidget);
            if ret < 0 && err == 0 {
                err = ret;
            }
        }
    }

    /*
     * decrement ref count for cores associated with all modules in the pipeline and clear
     * the complete flag
     */
    if (*swidget).id == snd_soc_dapm_type::snd_soc_dapm_scheduler {
        let mut i: c_int;

        for_each_set_bit!(i, &mut (*spipe).core_mask, (*sdev).num_cores, {
            ret = snd_sof_dsp_core_put(sdev, i);
            if ret < 0 {
                dev_err(
                    (*sdev).dev,
                    b"failed to disable target core: %d for pipeline %s\n\0".as_ptr()
                        as *const c_char,
                    i,
                    (*(*swidget).widget).name,
                );
                if err == 0 {
                    err = ret;
                }
            }
        });
        (*(*swidget).spipe).complete = 0;
    }

    /*
     * free the scheduler widget (same as pipe_widget) associated with the current swidget.
     * skip for static pipelines
     */
    if !(*swidget).spipe.is_null()
        && (*swidget).dynamic_pipeline_widget
        && (*swidget).id != snd_soc_dapm_type::snd_soc_dapm_scheduler
    {
        ret = sof_widget_free_unlocked(sdev, (*(*swidget).spipe).pipe_widget);
        if ret < 0 && err == 0 {
            err = ret;
        }
    }

    if err == 0 {
        dev_dbg(
            (*sdev).dev,
            b"widget %s freed\n\0".as_ptr() as *const c_char,
            (*(*swidget).widget).name,
        );
    }

    err
}

#[no_mangle]
pub unsafe extern "C" fn sof_widget_free(
    sdev: *mut snd_sof_dev,
    swidget: *mut snd_sof_widget,
) -> c_int {
    guard_mutex!(&mut (*swidget).setup_mutex, {
        sof_widget_free_unlocked(sdev, swidget)
    })
}

unsafe fn sof_widget_setup_unlocked(
    sdev: *mut snd_sof_dev,
    swidget: *mut snd_sof_widget,
) -> c_int {
    let tplg_ops = sof_ipc_get_ops(sdev, SofIpcOpsKind::tplg);
    let spipe = (*swidget).spipe;
    let mut ret: c_int;
    let mut i: c_int = 0;

    /* skip if there is no private data */
    if (*swidget).private.is_null() {
        return 0;
    }

    trace_sof_widget_setup(swidget);

    /* widget already set up */
    (*swidget).use_count += 1;
    if (*swidget).use_count > 1 {
        return 0;
    }

    /*
     * The scheduler widget for a pipeline is not part of the connected DAPM
     * widget list and it needs to be set up before the widgets in the pipeline
     * are set up. The use_count for the scheduler widget is incremented for every
     * widget in a given pipeline to ensure that it is freed only after the last
     * widget in the pipeline is freed. Skip setting up scheduler widget for static pipelines.
     */
    if (*swidget).dynamic_pipeline_widget
        && (*swidget).id != snd_soc_dapm_type::snd_soc_dapm_scheduler
    {
        if (*swidget).spipe.is_null() || (*(*swidget).spipe).pipe_widget.is_null() {
            dev_err(
                (*sdev).dev,
                b"No pipeline set for %s\n\0".as_ptr() as *const c_char,
                (*(*swidget).widget).name,
            );
            ret = -EINVAL;
            goto_use_count_dec!(ret, sdev, swidget);
        }

        ret = sof_widget_setup_unlocked(sdev, (*(*swidget).spipe).pipe_widget);
        if ret < 0 {
            (*swidget).use_count -= 1;
            return ret;
        }
    }

    /* update ref count for cores associated with all modules in the pipeline */
    if (*swidget).id == snd_soc_dapm_type::snd_soc_dapm_scheduler {
        for_each_set_bit!(i, &mut (*spipe).core_mask, (*sdev).num_cores, {
            ret = snd_sof_dsp_core_get(sdev, i);
            if ret < 0 {
                dev_err(
                    (*sdev).dev,
                    b"failed to enable target core %d for pipeline %s\n\0".as_ptr()
                        as *const c_char,
                    i,
                    (*(*swidget).widget).name,
                );
                if (*swidget).id != snd_soc_dapm_type::snd_soc_dapm_scheduler {
                    sof_widget_free_unlocked(sdev, (*(*swidget).spipe).pipe_widget);
                } else {
                    let mut j: c_int;
                    for_each_set_bit!(j, &mut (*spipe).core_mask, (*sdev).num_cores, {
                        if j >= i {
                            break;
                        }
                        snd_sof_dsp_core_put(sdev, j);
                    });
                }
                (*swidget).use_count -= 1;
                return ret;
            }
        });
    }

    /* setup widget in the DSP */
    if !tplg_ops.is_null() {
        if let Some(widget_setup) = (*tplg_ops).widget_setup {
            ret = widget_setup(sdev, swidget);
            if ret < 0 {
                if (*swidget).id != snd_soc_dapm_type::snd_soc_dapm_scheduler {
                    sof_widget_free_unlocked(sdev, (*(*swidget).spipe).pipe_widget);
                } else {
                    let mut j: c_int;
                    for_each_set_bit!(j, &mut (*spipe).core_mask, (*sdev).num_cores, {
                        if j >= i {
                            break;
                        }
                        snd_sof_dsp_core_put(sdev, j);
                    });
                }
                (*swidget).use_count -= 1;
                return ret;
            }
        }
    }

    /* send config for DAI components */
    if WIDGET_IS_DAI((*swidget).id) {
        let flags: c_uint = SOF_DAI_CONFIG_FLAGS_HW_PARAMS;

        /*
         * The config flags saved during BE DAI hw_params will be used for IPC3. IPC4 does
         * not use the flags argument.
         */
        if !tplg_ops.is_null() {
            if let Some(dai_config) = (*tplg_ops).dai_config {
                ret = dai_config(sdev, swidget, flags, ptr::null_mut());
                if ret < 0 {
                    sof_widget_free_unlocked(sdev, swidget);
                    return ret;
                }
            }
        }
    }

    /* restore kcontrols for widget */
    if !tplg_ops.is_null() && !(*tplg_ops).control.is_null() {
        if let Some(widget_kcontrol_setup) = (*(*tplg_ops).control).widget_kcontrol_setup {
            ret = widget_kcontrol_setup(sdev, swidget);
            if ret < 0 {
                sof_widget_free_unlocked(sdev, swidget);
                return ret;
            }
        }
    }

    dev_dbg(
        (*sdev).dev,
        b"widget %s setup complete\n\0".as_ptr() as *const c_char,
        (*(*swidget).widget).name,
    );

    0
}

#[no_mangle]
pub unsafe extern "C" fn sof_widget_setup(
    sdev: *mut snd_sof_dev,
    swidget: *mut snd_sof_widget,
) -> c_int {
    guard_mutex!(&mut (*swidget).setup_mutex, {
        sof_widget_setup_unlocked(sdev, swidget)
    })
}

#[no_mangle]
pub unsafe extern "C" fn sof_route_setup(
    sdev: *mut snd_sof_dev,
    wsource: *mut snd_soc_dapm_widget,
    wsink: *mut snd_soc_dapm_widget,
) -> c_int {
    let tplg_ops = sof_ipc_get_ops(sdev, SofIpcOpsKind::tplg);
    let src_widget = (*wsource).dobj.private as *mut snd_sof_widget;
    let sink_widget = (*wsink).dobj.private as *mut snd_sof_widget;
    let mut sroute: *mut snd_sof_route = ptr::null_mut();
    let mut route_found = false;

    /* ignore routes involving virtual widgets in topology */
    if is_virtual_widget(sdev, (*src_widget).widget, b"sof_route_setup\0".as_ptr() as *const c_char)
        || is_virtual_widget(sdev, (*sink_widget).widget, b"sof_route_setup\0".as_ptr() as *const c_char)
    {
        return 0;
    }

    /* skip route if source/sink widget is not set up */
    if (*src_widget).use_count == 0 || (*sink_widget).use_count == 0 {
        return 0;
    }

    /* find route matching source and sink widgets */
    list_for_each_entry!(sroute, &mut (*sdev).route_list, list, {
        if (*sroute).src_widget == src_widget && (*sroute).sink_widget == sink_widget {
            route_found = true;
            break;
        }
    });

    if !route_found {
        dev_err(
            (*sdev).dev,
            b"error: cannot find SOF route for source %s -> %s sink\n\0".as_ptr()
                as *const c_char,
            (*wsource).name,
            (*wsink).name,
        );
        return -EINVAL;
    }

    /* nothing to do if route is already set up */
    if (*sroute).setup {
        return 0;
    }

    if !tplg_ops.is_null() {
        if let Some(route_setup) = (*tplg_ops).route_setup {
            let ret = route_setup(sdev, sroute);

            if ret < 0 {
                return ret;
            }
        }
    }

    (*sroute).setup = true;
    0
}

unsafe fn sof_widget_in_same_direction(swidget: *mut snd_sof_widget, dir: c_int) -> bool {
    (*(*swidget).spipe).direction == dir
}

unsafe fn sof_set_up_same_dir_widget_routes(
    sdev: *mut snd_sof_dev,
    wsource: *mut snd_soc_dapm_widget,
    wsink: *mut snd_soc_dapm_widget,
) -> c_int {
    let src_widget = (*wsource).dobj.private as *mut snd_sof_widget;
    let sink_widget = (*wsink).dobj.private as *mut snd_sof_widget;

    /*
     * skip setting up route if source and sink are in different directions (ex. playback and
     * echo ref) if the direction is set in topology. These will be set up later. It is enough
     * to check if the direction_valid is set for one of the widgets as all widgets will have
     * the direction set in topology if one is set.
     */
    if !(*sink_widget).spipe.is_null()
        && (*(*sink_widget).spipe).direction_valid
        && !sof_widget_in_same_direction(sink_widget, (*(*src_widget).spipe).direction)
    {
        return 0;
    }

    sof_route_setup(sdev, wsource, wsink)
}

unsafe fn sof_setup_pipeline_connections(
    sdev: *mut snd_sof_dev,
    list: *mut snd_soc_dapm_widget_list,
    dir: c_int,
) -> c_int {
    let mut widget: *mut snd_soc_dapm_widget;
    let mut sroute: *mut snd_sof_route;
    let mut p: *mut snd_soc_dapm_path;
    let mut ret: c_int;
    let mut i: c_int;

    /*
     * Set up connections between widgets in the sink/source paths based on direction.
     * Some non-SOF widgets exist in topology either for compatibility or for the
     * purpose of connecting a pipeline from a host to a DAI in order to receive the DAPM
     * events. But they are not handled by the firmware. So ignore them.
     */
    if dir == SNDRV_PCM_STREAM_PLAYBACK {
        for_each_dapm_widgets!(list, i, widget, {
            if (*widget).dobj.private.is_null() {
                continue;
            }

            snd_soc_dapm_widget_for_each_sink_path!(widget, p, {
                if !widget_in_list(list, (*p).sink) {
                    continue;
                }

                if !(*(*p).sink).dobj.private.is_null() {
                    ret = sof_set_up_same_dir_widget_routes(sdev, widget, (*p).sink);
                    if ret < 0 {
                        return ret;
                    }
                }
            });
        });
    } else {
        for_each_dapm_widgets!(list, i, widget, {
            if (*widget).dobj.private.is_null() {
                continue;
            }

            snd_soc_dapm_widget_for_each_source_path!(widget, p, {
                if !widget_in_list(list, (*p).source) {
                    continue;
                }

                if !(*(*p).source).dobj.private.is_null() {
                    ret = sof_set_up_same_dir_widget_routes(sdev, (*p).source, widget);
                    if ret < 0 {
                        return ret;
                    }
                }
            });
        });
    }

    /*
     * The above loop handles connections between widgets that belong to the DAPM widget list.
     * This is not sufficient to handle loopback cases between pipelines configured with
     * different directions, e.g. a sidetone or an amplifier feedback connected to a speaker
     * protection module.
     */
    list_for_each_entry!(sroute, &mut (*sdev).route_list, list, {
        let src_widget_in_dapm_list: bool;
        let sink_widget_in_dapm_list: bool;

        if (*sroute).setup {
            continue;
        }

        src_widget_in_dapm_list = widget_in_list(list, (*(*sroute).src_widget).widget);
        sink_widget_in_dapm_list = widget_in_list(list, (*(*sroute).sink_widget).widget);

        /*
         * no need to set up the route if both the source and sink widgets are not in the
         * DAPM list
         */
        if !src_widget_in_dapm_list && !sink_widget_in_dapm_list {
            continue;
        }

        /*
         * set up the route only if both the source and sink widgets are in the DAPM list
         * but are in different directions. The ones in the same direction would already
         * have been set up in the previous loop.
         */
        if src_widget_in_dapm_list && sink_widget_in_dapm_list {
            let src_widget =
                (*(*(*sroute).src_widget).widget).dobj.private as *mut snd_sof_widget;
            let sink_widget =
                (*(*(*sroute).sink_widget).widget).dobj.private as *mut snd_sof_widget;

            /*
             * it is enough to check if the direction_valid is set for one of the
             * widgets as all widgets will have the direction set in topology if one
             * is set.
             */
            if !src_widget.is_null()
                && !sink_widget.is_null()
                && !(*src_widget).spipe.is_null()
                && (*(*src_widget).spipe).direction_valid
                && sof_widget_in_same_direction(sink_widget, (*(*src_widget).spipe).direction)
            {
                continue;
            }
        }

        ret = sof_route_setup(
            sdev,
            (*(*sroute).src_widget).widget,
            (*(*sroute).sink_widget).widget,
        );

        if ret < 0 {
            return ret;
        }
    });

    0
}

unsafe fn sof_unprepare_widgets_in_path(
    sdev: *mut snd_sof_dev,
    widget: *mut snd_soc_dapm_widget,
    list: *mut snd_soc_dapm_widget_list,
    dir: c_int,
) {
    let tplg_ops = sof_ipc_get_ops(sdev, SofIpcOpsKind::tplg);
    let swidget = (*widget).dobj.private as *mut snd_sof_widget;
    let mut widget_ops: *const sof_ipc_tplg_widget_ops;
    let mut p: *mut snd_soc_dapm_path;

    if is_virtual_widget(sdev, widget, b"sof_unprepare_widgets_in_path\0".as_ptr() as *const c_char)
    {
        return;
    }

    if swidget.is_null() {
        goto_sink_unprepare!(sdev, widget, list, dir);
        return;
    }

    if !(*swidget).spipe.is_null()
        && (*(*swidget).spipe).direction_valid
        && !sof_widget_in_same_direction(swidget, dir)
    {
        return;
    }

    /* skip widgets in use, those already unprepared or aggregated DAIs */
    if !(*swidget).prepared || (*swidget).use_count > 0 || is_aggregated_dai(swidget) {
        goto_sink_unprepare!(sdev, widget, list, dir);
        return;
    }

    widget_ops = if !tplg_ops.is_null() {
        (*tplg_ops).widget
    } else {
        ptr::null()
    };
    if !widget_ops.is_null() {
        let ops = widget_ops.add((*widget).id as usize);
        if let Some(ipc_unprepare) = (*ops).ipc_unprepare {
            /* unprepare the source widget */
            ipc_unprepare(swidget);
        }
    }

    (*swidget).prepared = false;

    /* unprepare all widgets in the sink paths */
    snd_soc_dapm_widget_for_each_sink_path!(widget, p, {
        if !widget_in_list(list, (*p).sink) {
            continue;
        }

        if !(*p).walking && !(*(*p).sink).dobj.private.is_null() {
            (*p).walking = true;
            sof_unprepare_widgets_in_path(sdev, (*p).sink, list, dir);
            (*p).walking = false;
        }
    });
}

unsafe fn sof_prepare_widgets_in_path(
    sdev: *mut snd_sof_dev,
    widget: *mut snd_soc_dapm_widget,
    fe_params: *mut snd_pcm_hw_params,
    platform_params: *mut snd_sof_platform_stream_params,
    pipeline_params: *mut snd_pcm_hw_params,
    dir: c_int,
    list: *mut snd_soc_dapm_widget_list,
) -> c_int {
    let tplg_ops = sof_ipc_get_ops(sdev, SofIpcOpsKind::tplg);
    let swidget = (*widget).dobj.private as *mut snd_sof_widget;
    let widget_ops: *const sof_ipc_tplg_widget_ops;
    let mut p: *mut snd_soc_dapm_path;
    let mut ret: c_int;

    if is_virtual_widget(sdev, widget, b"sof_prepare_widgets_in_path\0".as_ptr() as *const c_char)
    {
        return 0;
    }

    if swidget.is_null() {
        goto_sink_prepare!(sdev, widget, fe_params, platform_params, pipeline_params, dir, list);
        return 0;
    }

    widget_ops = if !tplg_ops.is_null() {
        (*tplg_ops).widget
    } else {
        ptr::null()
    };
    if widget_ops.is_null() {
        return 0;
    }

    if !(*swidget).spipe.is_null()
        && (*(*swidget).spipe).direction_valid
        && !sof_widget_in_same_direction(swidget, dir)
    {
        return 0;
    }

    /* skip widgets aggregated DAI widgets */
    let ops = widget_ops.add((*widget).id as usize);
    if (*ops).ipc_prepare.is_none() || is_aggregated_dai(swidget) {
        goto_sink_prepare!(sdev, widget, fe_params, platform_params, pipeline_params, dir, list);
        return 0;
    }

    /* prepare the source widget */
    ret = (*ops).ipc_prepare.unwrap()(swidget, fe_params, platform_params, pipeline_params, dir);
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            b"failed to prepare widget %s\n\0".as_ptr() as *const c_char,
            (*widget).name,
        );
        return ret;
    }

    (*swidget).prepared = true;

    /* prepare all widgets in the sink paths */
    snd_soc_dapm_widget_for_each_sink_path!(widget, p, {
        if !widget_in_list(list, (*p).sink) {
            continue;
        }

        if !(*p).walking && !(*(*p).sink).dobj.private.is_null() {
            (*p).walking = true;
            ret = sof_prepare_widgets_in_path(
                sdev,
                (*p).sink,
                fe_params,
                platform_params,
                pipeline_params,
                dir,
                list,
            );
            (*p).walking = false;
            if ret < 0 {
                /* unprepare the source widget */
                if let Some(ipc_unprepare) = (*ops).ipc_unprepare {
                    if !swidget.is_null() && (*swidget).prepared && (*swidget).use_count == 0 {
                        ipc_unprepare(swidget);
                        (*swidget).prepared = false;
                    }
                }
                return ret;
            }
        }
    });

    0
}

/*
 * free all widgets in the sink path starting from the source widget
 * (DAI type for capture, AIF type for playback)
 */
unsafe fn sof_free_widgets_in_path(
    sdev: *mut snd_sof_dev,
    widget: *mut snd_soc_dapm_widget,
    dir: c_int,
    spcm: *mut snd_sof_pcm,
) -> c_int {
    let list = (*spcm).stream[dir as usize].list;
    let swidget = (*widget).dobj.private as *mut snd_sof_widget;
    let mut p: *mut snd_soc_dapm_path;
    let mut err: c_int;
    let mut ret: c_int = 0;

    if is_virtual_widget(sdev, widget, b"sof_free_widgets_in_path\0".as_ptr() as *const c_char) {
        return 0;
    }

    if swidget.is_null() {
        goto_sink_free!(sdev, widget, dir, spcm);
        return ret;
    }

    if !(*swidget).spipe.is_null()
        && (*(*swidget).spipe).direction_valid
        && !sof_widget_in_same_direction(swidget, dir)
    {
        return 0;
    }

    /* skip aggregated DAIs */
    if is_aggregated_dai(swidget) {
        goto_sink_free!(sdev, widget, dir, spcm);
        return ret;
    }

    err = sof_widget_free(sdev, (*widget).dobj.private as *mut snd_sof_widget);
    if err < 0 {
        ret = err;
    }

    /* free all widgets in the sink paths even in case of error to keep use counts balanced */
    snd_soc_dapm_widget_for_each_sink_path!(widget, p, {
        if !(*p).walking {
            if !widget_in_list(list, (*p).sink) {
                continue;
            }

            (*p).walking = true;

            err = sof_free_widgets_in_path(sdev, (*p).sink, dir, spcm);
            if err < 0 {
                ret = err;
            }
            (*p).walking = false;
        }
    });

    ret
}

/*
 * set up all widgets in the sink path starting from the source widget
 * (DAI type for capture, AIF type for playback).
 * The error path in this function ensures that all successfully set up widgets getting freed.
 */
unsafe fn sof_set_up_widgets_in_path(
    sdev: *mut snd_sof_dev,
    widget: *mut snd_soc_dapm_widget,
    dir: c_int,
    spcm: *mut snd_sof_pcm,
) -> c_int {
    let pipeline_list = &mut (*spcm).stream[dir as usize].pipeline_list;
    let list = (*spcm).stream[dir as usize].list;
    let swidget = (*widget).dobj.private as *mut snd_sof_widget;
    let mut spipe: *mut snd_sof_pipeline;
    let mut p: *mut snd_soc_dapm_path;
    let mut ret: c_int;

    if is_virtual_widget(sdev, widget, b"sof_set_up_widgets_in_path\0".as_ptr() as *const c_char) {
        return 0;
    }

    if !swidget.is_null() {
        let mut i: c_int;

        if !(*swidget).spipe.is_null()
            && (*(*swidget).spipe).direction_valid
            && !sof_widget_in_same_direction(swidget, dir)
        {
            return 0;
        }

        /* skip aggregated DAIs */
        if !is_aggregated_dai(swidget) {
            ret = sof_widget_setup(sdev, swidget);
            if ret < 0 {
                return ret;
            }

            /* skip populating the pipe_widgets array if it is NULL */
            if !pipeline_list.pipelines.is_null() {
                /*
                 * Add the widget's pipe_widget to the list of pipelines to be triggered if not
                 * already in the list. This will result in the pipelines getting added in the
                 * order source to sink.
                 */
                i = 0;
                while i < pipeline_list.count {
                    spipe = *pipeline_list.pipelines.add(i as usize);
                    if spipe == (*swidget).spipe {
                        break;
                    }
                    i += 1;
                }

                if i == pipeline_list.count {
                    pipeline_list.count += 1;
                    *pipeline_list.pipelines.add(i as usize) = (*swidget).spipe;
                }
            }
        }
    }

    snd_soc_dapm_widget_for_each_sink_path!(widget, p, {
        if !(*p).walking {
            if !widget_in_list(list, (*p).sink) {
                continue;
            }

            (*p).walking = true;

            ret = sof_set_up_widgets_in_path(sdev, (*p).sink, dir, spcm);
            (*p).walking = false;
            if ret < 0 {
                if !swidget.is_null() {
                    sof_widget_free(sdev, swidget);
                }
                return ret;
            }
        }
    });

    0
}

unsafe fn sof_walk_widgets_in_order(
    sdev: *mut snd_sof_dev,
    spcm: *mut snd_sof_pcm,
    fe_params: *mut snd_pcm_hw_params,
    platform_params: *mut snd_sof_platform_stream_params,
    dir: c_int,
    op: sof_widget_op,
) -> c_int {
    let list = (*spcm).stream[dir as usize].list;
    let mut widget: *mut snd_soc_dapm_widget;
    let mut str_: *const c_char = ptr::null();
    let mut ret: c_int = 0;
    let mut i: c_int;

    if list.is_null() {
        return 0;
    }

    for_each_dapm_widgets!(list, i, widget, {
        /* starting widget for playback is of AIF type */
        if dir == SNDRV_PCM_STREAM_PLAYBACK
            && (*widget).id != snd_soc_dapm_type::snd_soc_dapm_aif_in
        {
            continue;
        }

        /* starting widget for capture is DAI type */
        if dir == SNDRV_PCM_STREAM_CAPTURE
            && (*widget).id != snd_soc_dapm_type::snd_soc_dapm_dai_out
            && (*widget).id != snd_soc_dapm_type::snd_soc_dapm_output
        {
            continue;
        }

        match op {
            sof_widget_op::SOF_WIDGET_SETUP => {
                ret = sof_set_up_widgets_in_path(sdev, widget, dir, spcm);
                str_ = b"set up\0".as_ptr() as *const c_char;
            }
            sof_widget_op::SOF_WIDGET_FREE => {
                ret = sof_free_widgets_in_path(sdev, widget, dir, spcm);
                str_ = b"free\0".as_ptr() as *const c_char;
            }
            sof_widget_op::SOF_WIDGET_PREPARE => {
                let mut pipeline_params = core::mem::MaybeUninit::<snd_pcm_hw_params>::uninit();

                str_ = b"prepare\0".as_ptr() as *const c_char;
                /*
                 * When walking the list of connected widgets, the pipeline_params for each
                 * widget is modified by the source widget in the path. Use a local
                 * copy of the runtime params as the pipeline_params so that the runtime
                 * params does not get overwritten.
                 */
                memcpy(
                    pipeline_params.as_mut_ptr() as *mut c_void,
                    fe_params as *const c_void,
                    core::mem::size_of::<snd_pcm_hw_params>(),
                );

                ret = sof_prepare_widgets_in_path(
                    sdev,
                    widget,
                    fe_params,
                    platform_params,
                    pipeline_params.as_mut_ptr(),
                    dir,
                    list,
                );
            }
            sof_widget_op::SOF_WIDGET_UNPREPARE => {
                sof_unprepare_widgets_in_path(sdev, widget, list, dir);
            }
        }
        if ret < 0 {
            dev_err(
                (*sdev).dev,
                b"Failed to %s connected widgets\n\0".as_ptr() as *const c_char,
                str_,
            );
            return ret;
        }
    });

    0
}

#[no_mangle]
pub unsafe extern "C" fn sof_widget_list_prepare(
    sdev: *mut snd_sof_dev,
    spcm: *mut snd_sof_pcm,
    fe_params: *mut snd_pcm_hw_params,
    platform_params: *mut snd_sof_platform_stream_params,
    dir: c_int,
) -> c_int {
    /*
     * Prepare widgets for set up. The prepare step is used to allocate memory, assign
     * instance ID and pick the widget configuration based on the runtime PCM params.
     */
    sof_walk_widgets_in_order(
        sdev,
        spcm,
        fe_params,
        platform_params,
        dir,
        sof_widget_op::SOF_WIDGET_PREPARE,
    )
}

#[no_mangle]
pub unsafe extern "C" fn sof_widget_list_unprepare(
    sdev: *mut snd_sof_dev,
    spcm: *mut snd_sof_pcm,
    dir: c_int,
) {
    let mut list = (*spcm).stream[dir as usize].list;

    /* unprepare the widget */
    sof_walk_widgets_in_order(
        sdev,
        spcm,
        ptr::null_mut(),
        ptr::null_mut(),
        dir,
        sof_widget_op::SOF_WIDGET_UNPREPARE,
    );

    snd_soc_dapm_dai_free_widgets(&mut list);
    (*spcm).stream[dir as usize].list = ptr::null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn sof_widget_list_setup(
    sdev: *mut snd_sof_dev,
    spcm: *mut snd_sof_pcm,
    fe_params: *mut snd_pcm_hw_params,
    platform_params: *mut snd_sof_platform_stream_params,
    dir: c_int,
) -> c_int {
    let tplg_ops = sof_ipc_get_ops(sdev, SofIpcOpsKind::tplg);
    let list = (*spcm).stream[dir as usize].list;
    let mut widget: *mut snd_soc_dapm_widget;
    let mut ret: c_int;
    let mut i: c_int;

    /* nothing to set up or setup has been already done */
    if list.is_null() || (*spcm).setup_done[dir as usize] {
        return 0;
    }

    /* Set up is used to send the IPC to the DSP to create the widget */
    ret = sof_walk_widgets_in_order(
        sdev,
        spcm,
        fe_params,
        platform_params,
        dir,
        sof_widget_op::SOF_WIDGET_SETUP,
    );
    if ret < 0 {
        sof_walk_widgets_in_order(
            sdev,
            spcm,
            fe_params,
            platform_params,
            dir,
            sof_widget_op::SOF_WIDGET_UNPREPARE,
        );
        return ret;
    }

    /*
     * error in setting pipeline connections will result in route status being reset for
     * routes that were successfully set up when the widgets are freed.
     */
    ret = sof_setup_pipeline_connections(sdev, list, dir);
    if ret < 0 {
        sof_walk_widgets_in_order(sdev, spcm, fe_params, platform_params, dir, sof_widget_op::SOF_WIDGET_FREE);
        sof_walk_widgets_in_order(sdev, spcm, ptr::null_mut(), ptr::null_mut(), dir, sof_widget_op::SOF_WIDGET_UNPREPARE);
        return ret;
    }

    /* complete pipelines */
    for_each_dapm_widgets!(list, i, widget, {
        let swidget = (*widget).dobj.private as *mut snd_sof_widget;
        let pipe_widget: *mut snd_sof_widget;
        let spipe: *mut snd_sof_pipeline;

        if swidget.is_null() || (*sdev).dspless_mode_selected {
            continue;
        }

        spipe = (*swidget).spipe;
        if spipe.is_null() {
            dev_err(
                (*sdev).dev,
                b"no pipeline found for %s\n\0".as_ptr() as *const c_char,
                (*(*swidget).widget).name,
            );
            ret = -EINVAL;
            sof_walk_widgets_in_order(sdev, spcm, fe_params, platform_params, dir, sof_widget_op::SOF_WIDGET_FREE);
            sof_walk_widgets_in_order(sdev, spcm, ptr::null_mut(), ptr::null_mut(), dir, sof_widget_op::SOF_WIDGET_UNPREPARE);
            return ret;
        }

        pipe_widget = (*spipe).pipe_widget;
        if pipe_widget.is_null() {
            dev_err(
                (*sdev).dev,
                b"error: no pipeline widget found for %s\n\0".as_ptr() as *const c_char,
                (*(*swidget).widget).name,
            );
            ret = -EINVAL;
            sof_walk_widgets_in_order(sdev, spcm, fe_params, platform_params, dir, sof_widget_op::SOF_WIDGET_FREE);
            sof_walk_widgets_in_order(sdev, spcm, ptr::null_mut(), ptr::null_mut(), dir, sof_widget_op::SOF_WIDGET_UNPREPARE);
            return ret;
        }

        if (*spipe).complete != 0 {
            continue;
        }

        if !tplg_ops.is_null() {
            if let Some(pipeline_complete) = (*tplg_ops).pipeline_complete {
                (*spipe).complete = pipeline_complete(sdev, pipe_widget);
                if (*spipe).complete < 0 {
                    ret = (*spipe).complete;
                    sof_walk_widgets_in_order(sdev, spcm, fe_params, platform_params, dir, sof_widget_op::SOF_WIDGET_FREE);
                    sof_walk_widgets_in_order(sdev, spcm, ptr::null_mut(), ptr::null_mut(), dir, sof_widget_op::SOF_WIDGET_UNPREPARE);
                    return ret;
                }
            }
        }
    });

    (*spcm).setup_done[dir as usize] = true;

    0
}

#[no_mangle]
pub unsafe extern "C" fn sof_widget_list_free(
    sdev: *mut snd_sof_dev,
    spcm: *mut snd_sof_pcm,
    dir: c_int,
) -> c_int {
    let pipeline_list = &mut (*spcm).stream[dir as usize].pipeline_list;
    let list = (*spcm).stream[dir as usize].list;
    let ret: c_int;

    /* nothing to free */
    if list.is_null() || !(*spcm).setup_done[dir as usize] {
        return 0;
    }

    /* send IPC to free widget in the DSP */
    ret = sof_walk_widgets_in_order(
        sdev,
        spcm,
        ptr::null_mut(),
        ptr::null_mut(),
        dir,
        sof_widget_op::SOF_WIDGET_FREE,
    );

    (*spcm).setup_done[dir as usize] = false;
    pipeline_list.count = 0;

    ret
}

/*
 * helper to determine if there are only D0i3 compatible
 * streams active
 */
#[no_mangle]
pub unsafe extern "C" fn snd_sof_dsp_only_d0i3_compatible_stream_active(
    sdev: *mut snd_sof_dev,
) -> bool {
    let mut substream: *mut snd_pcm_substream;
    let mut spcm: *mut snd_sof_pcm;
    let mut d0i3_compatible_active = false;
    let mut dir: c_int;

    list_for_each_entry!(spcm, &mut (*sdev).pcm_list, list, {
        for_each_pcm_streams!(dir, {
            substream = (*spcm).stream[dir as usize].substream;
            if substream.is_null() || (*substream).runtime.is_null() {
                continue;
            }

            /*
             * substream->runtime being not NULL indicates
             * that the stream is open. No need to check the
             * stream state.
             */
            if !(*spcm).stream[dir as usize].d0i3_compatible {
                return false;
            }

            d0i3_compatible_active = true;
        });
    });

    d0i3_compatible_active
}

#[no_mangle]
pub unsafe extern "C" fn snd_sof_stream_suspend_ignored(sdev: *mut snd_sof_dev) -> bool {
    let mut spcm: *mut snd_sof_pcm;

    list_for_each_entry!(spcm, &mut (*sdev).pcm_list, list, {
        if (*spcm).stream[SNDRV_PCM_STREAM_PLAYBACK as usize].suspend_ignored
            || (*spcm).stream[SNDRV_PCM_STREAM_CAPTURE as usize].suspend_ignored
        {
            return true;
        }
    });

    false
}

/*
 * Generic object lookup APIs.
 */

#[no_mangle]
pub unsafe extern "C" fn snd_sof_find_spcm_name(
    scomp: *mut snd_soc_component,
    name: *const c_char,
) -> *mut snd_sof_pcm {
    let sdev = snd_soc_component_get_drvdata(scomp);
    let mut spcm: *mut snd_sof_pcm;

    list_for_each_entry!(spcm, &mut (*sdev).pcm_list, list, {
        /* match with PCM dai name */
        if strcmp((*spcm).pcm.dai_name, name) == 0 {
            return spcm;
        }

        /* match with playback caps name if set */
        if *(*spcm).pcm.caps[0].name.as_ptr() != 0
            && strcmp((*spcm).pcm.caps[0].name.as_ptr(), name) == 0
        {
            return spcm;
        }

        /* match with capture caps name if set */
        if *(*spcm).pcm.caps[1].name.as_ptr() != 0
            && strcmp((*spcm).pcm.caps[1].name.as_ptr(), name) == 0
        {
            return spcm;
        }
    });

    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn snd_sof_find_spcm_comp(
    scomp: *mut snd_soc_component,
    comp_id: c_uint,
    direction: *mut c_int,
) -> *mut snd_sof_pcm {
    let sdev = snd_soc_component_get_drvdata(scomp);
    let mut spcm: *mut snd_sof_pcm;
    let mut dir: c_int;

    list_for_each_entry!(spcm, &mut (*sdev).pcm_list, list, {
        for_each_pcm_streams!(dir, {
            if (*spcm).stream[dir as usize].comp_id == comp_id {
                *direction = dir;
                return spcm;
            }
        });
    });

    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn snd_sof_find_swidget(
    scomp: *mut snd_soc_component,
    name: *const c_char,
) -> *mut snd_sof_widget {
    let sdev = snd_soc_component_get_drvdata(scomp);
    let mut swidget: *mut snd_sof_widget;

    list_for_each_entry!(swidget, &mut (*sdev).widget_list, list, {
        if strcmp(name, (*(*swidget).widget).name) == 0 {
            return swidget;
        }
    });

    ptr::null_mut()
}

/* find widget by stream name and direction */
#[no_mangle]
pub unsafe extern "C" fn snd_sof_find_swidget_sname(
    scomp: *mut snd_soc_component,
    pcm_name: *const c_char,
    dir: c_int,
) -> *mut snd_sof_widget {
    let sdev = snd_soc_component_get_drvdata(scomp);
    let mut swidget: *mut snd_sof_widget;
    let type_: snd_soc_dapm_type;

    if dir == SNDRV_PCM_STREAM_PLAYBACK {
        type_ = snd_soc_dapm_type::snd_soc_dapm_aif_in;
    } else {
        type_ = snd_soc_dapm_type::snd_soc_dapm_aif_out;
    }

    list_for_each_entry!(swidget, &mut (*sdev).widget_list, list, {
        if strcmp(pcm_name, (*(*swidget).widget).sname) == 0 && (*swidget).id == type_ {
            return swidget;
        }
    });

    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn snd_sof_find_dai(
    scomp: *mut snd_soc_component,
    name: *const c_char,
) -> *mut snd_sof_dai {
    let sdev = snd_soc_component_get_drvdata(scomp);
    let mut dai: *mut snd_sof_dai;

    list_for_each_entry!(dai, &mut (*sdev).dai_list, list, {
        if !(*dai).name.is_null() && strcmp(name, (*dai).name) == 0 {
            return dai;
        }
    });

    ptr::null_mut()
}

unsafe fn sof_dai_get_param(rtd: *mut snd_soc_pcm_runtime, param_type: c_int) -> c_int {
    let component = snd_soc_rtdcom_lookup(rtd, SOF_AUDIO_PCM_DRV_NAME);
    let dai = snd_sof_find_dai(component, (*(*rtd).dai_link).name as *mut c_char);
    let sdev = snd_soc_component_get_drvdata(component);
    let tplg_ops = sof_ipc_get_ops(sdev, SofIpcOpsKind::tplg);

    /* use the tplg configured mclk if existed */
    if dai.is_null() {
        return 0;
    }

    if !tplg_ops.is_null() {
        if let Some(dai_get_param) = (*tplg_ops).dai_get_param {
            return dai_get_param(sdev, dai, param_type);
        }
    }

    0
}

/*
 * Helper to get SSP MCLK from a pcm_runtime.
 * Return 0 if not exist.
 */
#[no_mangle]
pub unsafe extern "C" fn sof_dai_get_mclk(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    sof_dai_get_param(rtd, SOF_DAI_PARAM_INTEL_SSP_MCLK)
}

/*
 * Helper to get SSP BCLK from a pcm_runtime.
 * Return 0 if not exist.
 */
#[no_mangle]
pub unsafe extern "C" fn sof_dai_get_bclk(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    sof_dai_get_param(rtd, SOF_DAI_PARAM_INTEL_SSP_BCLK)
}

/*
 * Helper to get SSP TDM slot number from a pcm_runtime.
 * Return 0 if not exist.
 */
#[no_mangle]
pub unsafe extern "C" fn sof_dai_get_tdm_slots(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    sof_dai_get_param(rtd, SOF_DAI_PARAM_INTEL_SSP_TDM_SLOTS)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
