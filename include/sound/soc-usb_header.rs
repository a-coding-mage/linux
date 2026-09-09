/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (c) 2022-2025 Qualcomm Innovation Center, Inc. All rights reserved.
 */

// Dependency: definitions supplied by the surrounding sound/soc headers.

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum snd_soc_usb_kctl {
    SND_SOC_USB_KCTL_CARD_ROUTE,
    SND_SOC_USB_KCTL_PCM_ROUTE,
}

#[repr(C)]
pub struct snd_soc_usb_device {
    pub card_idx: ::core::ffi::c_int,
    pub chip_idx: ::core::ffi::c_int,

    // PCM index arrays; capture path is not tested yet.
    pub cpcm_idx: *mut ::core::ffi::c_uint,
    pub ppcm_idx: *mut ::core::ffi::c_uint,
    pub num_capture: ::core::ffi::c_int,
    pub num_playback: ::core::ffi::c_int,

    pub list: list_head,
}

#[repr(C)]
pub struct snd_soc_usb {
    pub list: list_head,
    pub component: *mut snd_soc_component,
    pub connection_status_cb: Option<unsafe extern "C" fn(
        usb: *mut snd_soc_usb,
        sdev: *mut snd_soc_usb_device,
        connected: bool,
    ) -> ::core::ffi::c_int>,
    pub update_offload_route_info: Option<unsafe extern "C" fn(
        component: *mut snd_soc_component,
        card: ::core::ffi::c_int,
        pcm: ::core::ffi::c_int,
        direction: ::core::ffi::c_int,
        path: snd_soc_usb_kctl,
        route: *mut ::core::ffi::c_long,
    ) -> ::core::ffi::c_int>,
    pub priv_data: *mut ::core::ffi::c_void,
}

// External types and constants are supplied by other translated headers.
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}
#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_SND_SOC_USB")]
extern "C" {
    pub fn snd_soc_usb_find_supported_format(
        card_idx: ::core::ffi::c_int,
        params: *mut snd_pcm_hw_params,
        direction: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn snd_soc_usb_connect(usbdev: *mut device, sdev: *mut snd_soc_usb_device) -> ::core::ffi::c_int;
    pub fn snd_soc_usb_disconnect(usbdev: *mut device, sdev: *mut snd_soc_usb_device) -> ::core::ffi::c_int;
    pub fn snd_soc_usb_find_priv_data(usbdev: *mut device) -> *mut ::core::ffi::c_void;
    pub fn snd_soc_usb_setup_offload_jack(component: *mut snd_soc_component, jack: *mut snd_soc_jack) -> ::core::ffi::c_int;
    pub fn snd_soc_usb_update_offload_route(
        dev: *mut device, card: ::core::ffi::c_int, pcm: ::core::ffi::c_int,
        direction: ::core::ffi::c_int, path: snd_soc_usb_kctl,
        route: *mut ::core::ffi::c_long,
    ) -> ::core::ffi::c_int;
    pub fn snd_soc_usb_allocate_port(component: *mut snd_soc_component, data: *mut ::core::ffi::c_void) -> *mut snd_soc_usb;
    pub fn snd_soc_usb_free_port(usb: *mut snd_soc_usb);
    pub fn snd_soc_usb_add_port(usb: *mut snd_soc_usb);
    pub fn snd_soc_usb_remove_port(usb: *mut snd_soc_usb);
}

#[cfg(not(feature = "CONFIG_SND_SOC_USB"))]
pub unsafe fn snd_soc_usb_find_supported_format(_: ::core::ffi::c_int, _: *mut snd_pcm_hw_params, _: ::core::ffi::c_int) -> ::core::ffi::c_int { -22 }
#[cfg(not(feature = "CONFIG_SND_SOC_USB"))]
pub unsafe fn snd_soc_usb_connect(_: *mut device, _: *mut snd_soc_usb_device) -> ::core::ffi::c_int { -19 }
#[cfg(not(feature = "CONFIG_SND_SOC_USB"))]
pub unsafe fn snd_soc_usb_disconnect(_: *mut device, _: *mut snd_soc_usb_device) -> ::core::ffi::c_int { -22 }
#[cfg(not(feature = "CONFIG_SND_SOC_USB"))]
pub unsafe fn snd_soc_usb_find_priv_data(_: *mut device) -> *mut ::core::ffi::c_void { ::core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_SND_SOC_USB"))]
pub unsafe fn snd_soc_usb_setup_offload_jack(_: *mut snd_soc_component, _: *mut snd_soc_jack) -> ::core::ffi::c_int { 0 }
#[cfg(not(feature = "CONFIG_SND_SOC_USB"))]
pub unsafe fn snd_soc_usb_update_offload_route(_: *mut device, _: ::core::ffi::c_int, _: ::core::ffi::c_int, _: ::core::ffi::c_int, _: snd_soc_usb_kctl, _: *mut ::core::ffi::c_long) -> ::core::ffi::c_int { -19 }
#[cfg(not(feature = "CONFIG_SND_SOC_USB"))]
pub unsafe fn snd_soc_usb_allocate_port(_: *mut snd_soc_component, _: *mut ::core::ffi::c_void) -> *mut snd_soc_usb { ::core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_SND_SOC_USB"))]
pub unsafe fn snd_soc_usb_free_port(_: *mut snd_soc_usb) {}
#[cfg(not(feature = "CONFIG_SND_SOC_USB"))]
pub unsafe fn snd_soc_usb_add_port(_: *mut snd_soc_usb) {}
#[cfg(not(feature = "CONFIG_SND_SOC_USB"))]
pub unsafe fn snd_soc_usb_remove_port(_: *mut snd_soc_usb) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
