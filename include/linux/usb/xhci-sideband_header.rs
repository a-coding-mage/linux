/* SPDX-License-Identifier: GPL-2.0 */
/*
 * xHCI host controller sideband support
 *
 * Copyright (c) 2023-2025, Intel Corporation.
 *
 * Author: Mathias Nyman <mathias.nyman@linux.intel.com>
 */

// Dependencies supplied by the surrounding kernel bindings.

pub const EP_CTX_PER_DEV: usize = 31; /* FIXME defined twice, from xhci.h */

pub struct xhci_sideband;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xhci_sideband_type {
    XHCI_SIDEBAND_AUDIO,
    XHCI_SIDEBAND_VENDOR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xhci_sideband_notify_type {
    XHCI_SIDEBAND_XFER_RING_FREE,
}

/**
 * struct xhci_sideband_event - sideband event
 * @type: notifier type
 * @evt_data: event data
 */
#[repr(C)]
pub struct xhci_sideband_event {
    pub type_: xhci_sideband_notify_type,
    pub evt_data: *mut core::ffi::c_void,
}

/**
 * struct xhci_sideband - representation of a sideband accessed usb device.
 * @xhci: The xhci host controller the usb device is connected to
 * @vdev: the usb device accessed via sideband
 * @eps: array of endpoints controlled via sideband
 * @ir: event handling and buffer for sideband accessed device
 * @type: xHCI sideband type
 * @mutex: mutex for sideband operations
 * @intf: USB sideband client interface
 * @notify_client: callback for xHCI sideband sequences
 *
 * FIXME usb device accessed via sideband Keeping track of sideband accessed usb devices.
 */
#[repr(C)]
pub struct xhci_sideband {
    pub xhci: *mut xhci_hcd,
    pub vdev: *mut xhci_virt_device,
    pub eps: [*mut xhci_virt_ep; EP_CTX_PER_DEV],
    pub ir: *mut xhci_interrupter,
    pub type_: xhci_sideband_type,
    /* Synchronizing xHCI sideband operations with client drivers operations */
    pub mutex: mutex,
    pub intf: *mut usb_interface,
    pub notify_client: Option<unsafe extern "C" fn(
        intf: *mut usb_interface,
        evt: *mut xhci_sideband_event,
    ) -> core::ffi::c_int>,
}

extern "C" {
    pub fn xhci_sideband_register(
        intf: *mut usb_interface,
        type_: xhci_sideband_type,
        notify_client: Option<unsafe extern "C" fn(
            intf: *mut usb_interface,
            evt: *mut xhci_sideband_event,
        ) -> core::ffi::c_int>,
    ) -> *mut xhci_sideband;
    pub fn xhci_sideband_unregister(sb: *mut xhci_sideband);
    pub fn xhci_sideband_add_endpoint(
        sb: *mut xhci_sideband,
        host_ep: *mut usb_host_endpoint,
    ) -> core::ffi::c_int;
    pub fn xhci_sideband_remove_endpoint(
        sb: *mut xhci_sideband,
        host_ep: *mut usb_host_endpoint,
    ) -> core::ffi::c_int;
    pub fn xhci_sideband_stop_endpoint(
        sb: *mut xhci_sideband,
        host_ep: *mut usb_host_endpoint,
    ) -> core::ffi::c_int;
    pub fn xhci_sideband_get_endpoint_buffer(
        sb: *mut xhci_sideband,
        host_ep: *mut usb_host_endpoint,
    ) -> *mut sg_table;
    pub fn xhci_sideband_get_event_buffer(sb: *mut xhci_sideband) -> *mut sg_table;
    pub fn xhci_sideband_check(hcd: *mut usb_hcd) -> bool;
    pub fn xhci_sideband_create_interrupter(
        sb: *mut xhci_sideband,
        num_seg: core::ffi::c_int,
        ip_autoclear: bool,
        imod_interval: u32,
        intr_num: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn xhci_sideband_remove_interrupter(sb: *mut xhci_sideband);
    pub fn xhci_sideband_interrupter_id(sb: *mut xhci_sideband) -> core::ffi::c_int;
    pub fn xhci_sideband_notify_ep_ring_free(sb: *mut xhci_sideband, ep_index: u32);
}

// When CONFIG_USB_XHCI_SIDEBAND is disabled, the C header supplies these inline fallbacks.
#[cfg(not(feature = "CONFIG_USB_XHCI_SIDEBAND"))]
#[inline]
pub unsafe fn xhci_sideband_check_disabled(_hcd: *mut usb_hcd) -> bool { false }

#[cfg(not(feature = "CONFIG_USB_XHCI_SIDEBAND"))]
#[inline]
pub unsafe fn xhci_sideband_notify_ep_ring_free_disabled(
    _sb: *mut xhci_sideband,
    _ep_index: u32,
) { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
