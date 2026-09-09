/* SPDX-License-Identifier: GPL-2.0
 *
 * linux/sound/soc-topology.h -- ALSA SoC Firmware Controls and DAPM
 *
 * Copyright (C) 2012 Texas Instruments Inc.
 * Copyright (C) 2015 Intel Corporation.
 *
 * Simple file API to load FW that includes mixers, coefficients, DAPM graphs,
 * algorithms, equalisers, DAIs, widgets, FE caps, BE caps, codec link caps etc.
 */

use core::ffi::c_void;

/* External types supplied by the surrounding ALSA headers. */
#[repr(C)]
pub struct firmware { _unused: [u8; 0] }
#[repr(C)]
pub struct snd_kcontrol { _unused: [u8; 0] }
#[repr(C)]
pub struct snd_soc_tplg_pcm_be { _unused: [u8; 0] }
#[repr(C)]
pub struct snd_ctl_elem_value { _unused: [u8; 0] }
#[repr(C)]
pub struct snd_ctl_elem_info { _unused: [u8; 0] }
#[repr(C)]
pub struct snd_soc_dapm_widget { _unused: [u8; 0] }
#[repr(C)]
pub struct snd_soc_component { _unused: [u8; 0] }
#[repr(C)]
pub struct snd_soc_tplg_pcm_fe { _unused: [u8; 0] }
#[repr(C)]
pub struct snd_soc_dapm_context { _unused: [u8; 0] }
#[repr(C)]
pub struct snd_soc_card { _unused: [u8; 0] }
#[repr(C)]
pub struct snd_kcontrol_new { _unused: [u8; 0] }
#[repr(C)]
pub struct snd_soc_dai_link { _unused: [u8; 0] }
#[repr(C)]
pub struct snd_soc_dai_driver { _unused: [u8; 0] }
#[repr(C)]
pub struct snd_soc_dai { _unused: [u8; 0] }
#[repr(C)]
pub struct snd_soc_dapm_route { _unused: [u8; 0] }
#[repr(C)]
pub struct snd_soc_tplg_ctl_hdr { _unused: [u8; 0] }
#[repr(C)]
pub struct snd_soc_tplg_dapm_widget { _unused: [u8; 0] }
#[repr(C)]
pub struct snd_soc_tplg_pcm { _unused: [u8; 0] }
#[repr(C)]
pub struct snd_soc_tplg_link_config { _unused: [u8; 0] }
#[repr(C)]
pub struct snd_soc_tplg_hdr { _unused: [u8; 0] }
#[repr(C)]
pub struct snd_soc_tplg_manifest { _unused: [u8; 0] }
#[repr(C)]
pub struct list_head { _unused: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum snd_soc_dobj_type {
    SND_SOC_DOBJ_NONE = 0,
    SND_SOC_DOBJ_MIXER,
    SND_SOC_DOBJ_BYTES,
    SND_SOC_DOBJ_ENUM,
    SND_SOC_DOBJ_GRAPH,
    SND_SOC_DOBJ_WIDGET,
    SND_SOC_DOBJ_DAI_LINK,
    SND_SOC_DOBJ_PCM,
    SND_SOC_DOBJ_CODEC_LINK,
    SND_SOC_DOBJ_BACKEND_LINK,
}

#[repr(C)]
pub struct snd_soc_dobj_control {
    pub kcontrol: *mut snd_kcontrol,
    pub dtexts: *mut *mut i8,
    pub dvalues: *mut libc::c_ulong,
}

#[repr(C)]
pub struct snd_soc_dobj_widget {
    pub kcontrol_type: *mut u32,
}

#[repr(C)]
pub union snd_soc_dobj__bindgen_ty_1 {
    pub control: snd_soc_dobj_control,
    pub widget: snd_soc_dobj_widget,
}

#[repr(C)]
pub struct snd_soc_dobj {
    pub type_: snd_soc_dobj_type,
    pub index: u32,
    pub list: list_head,
    pub unload: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_dobj) -> i32>,
    pub __bindgen_anon_1: snd_soc_dobj__bindgen_ty_1,
    pub private: *mut c_void,
}

#[repr(C)]
pub struct snd_soc_tplg_kcontrol_ops {
    pub id: u32,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> i32>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> i32>,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> i32>,
}

#[repr(C)]
pub struct snd_soc_tplg_bytes_ext_ops {
    pub id: u32,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut u32, u32) -> i32>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *const u32, u32) -> i32>,
}

#[repr(C)]
pub struct snd_soc_tplg_widget_events {
    pub type_: u16,
    pub event_handler: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, i32) -> i32>,
}

#[repr(C)]
pub struct snd_soc_tplg_ops {
    pub control_load: Option<unsafe extern "C" fn(*mut snd_soc_component, i32, *mut snd_kcontrol_new, *mut snd_soc_tplg_ctl_hdr) -> i32>,
    pub control_unload: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_dobj) -> i32>,
    pub dapm_route_load: Option<unsafe extern "C" fn(*mut snd_soc_component, i32, *mut snd_soc_dapm_route) -> i32>,
    pub dapm_route_unload: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_dobj) -> i32>,
    pub widget_load: Option<unsafe extern "C" fn(*mut snd_soc_component, i32, *mut snd_soc_dapm_widget, *mut snd_soc_tplg_dapm_widget) -> i32>,
    pub widget_ready: Option<unsafe extern "C" fn(*mut snd_soc_component, i32, *mut snd_soc_dapm_widget, *mut snd_soc_tplg_dapm_widget) -> i32>,
    pub widget_unload: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_dobj) -> i32>,
    pub dai_load: Option<unsafe extern "C" fn(*mut snd_soc_component, i32, *mut snd_soc_dai_driver, *mut snd_soc_tplg_pcm, *mut snd_soc_dai) -> i32>,
    pub dai_unload: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_dobj) -> i32>,
    pub link_load: Option<unsafe extern "C" fn(*mut snd_soc_component, i32, *mut snd_soc_dai_link, *mut snd_soc_tplg_link_config) -> i32>,
    pub link_unload: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_dobj) -> i32>,
    pub vendor_load: Option<unsafe extern "C" fn(*mut snd_soc_component, i32, *mut snd_soc_tplg_hdr) -> i32>,
    pub vendor_unload: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_tplg_hdr) -> i32>,
    pub complete: Option<unsafe extern "C" fn(*mut snd_soc_component) -> i32>,
    pub manifest: Option<unsafe extern "C" fn(*mut snd_soc_component, i32, *mut snd_soc_tplg_manifest) -> i32>,
    pub io_ops: *const snd_soc_tplg_kcontrol_ops,
    pub io_ops_count: i32,
    pub bytes_ext_ops: *const snd_soc_tplg_bytes_ext_ops,
    pub bytes_ext_ops_count: i32,
}

#[cfg(feature = "CONFIG_SND_SOC_TOPOLOGY")]
pub unsafe fn snd_soc_tplg_get_data(hdr: *mut snd_soc_tplg_hdr) -> *const c_void {
    (hdr as *const u8).add(core::mem::size_of::<snd_soc_tplg_hdr>()) as *const c_void
}

#[cfg(feature = "CONFIG_SND_SOC_TOPOLOGY")]
extern "C" {
    pub fn snd_soc_tplg_component_load(comp: *mut snd_soc_component, ops: *const snd_soc_tplg_ops, fw: *const firmware) -> i32;
    pub fn snd_soc_tplg_component_remove(comp: *mut snd_soc_component) -> i32;
    pub fn snd_soc_tplg_widget_bind_event(w: *mut snd_soc_dapm_widget, events: *const snd_soc_tplg_widget_events, num_events: i32, event_type: u16) -> i32;
}

#[cfg(not(feature = "CONFIG_SND_SOC_TOPOLOGY"))]
pub unsafe fn snd_soc_tplg_component_remove(_comp: *mut snd_soc_component) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
