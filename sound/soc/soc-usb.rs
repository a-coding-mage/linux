// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2022-2025 Qualcomm Innovation Center, Inc. All rights reserved.
 */

// Dependencies translated from:
// #include <linux/of.h>
// #include <linux/usb.h>
// #include <sound/jack.h>
// #include <sound/soc-usb.h>
// #include "../usb/card.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_void};

pub enum device_node {}
pub enum device {}
pub enum snd_soc_component {}
pub enum snd_soc_jack {}
pub enum snd_soc_usb {}
pub enum snd_pcm_hw_params {}
pub enum snd_usb_stream {}
pub enum snd_soc_usb_device {}
pub enum snd_soc_usb_kctl {}
pub enum mutex {}
pub enum list_head {}

const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const EOPNOTSUPP: c_int = 95;
const SND_JACK_USB: c_int = 0x4000;

static mut ctx_mutex: mutex = unsafe { core::mem::zeroed() };
static mut usb_ctx_list: list_head = unsafe { core::mem::zeroed() };

unsafe extern "C" {
    static mut true_: bool;
    static mut false_: bool;

    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn ERR_PTR(error: c_long) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;

    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);

    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);

    fn snd_soc_card_jack_new(
        card: *mut c_void,
        id: *const c_char,
        typ: c_int,
        jack: *mut snd_soc_jack,
    ) -> c_int;
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_usb_find_suppported_substream(
        card_idx: c_int,
        params: *mut snd_pcm_hw_params,
        direction: c_int,
    ) -> *mut snd_usb_stream;
    fn kzalloc_obj(size: usize) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn snd_usb_rediscover_devices();

    fn snd_soc_usb_component(usb: *mut snd_soc_usb) -> *mut snd_soc_component;
    fn snd_soc_usb_priv_data(usb: *mut snd_soc_usb) -> *mut c_void;
    fn snd_soc_usb_list(usb: *mut snd_soc_usb) -> *mut list_head;
    fn snd_soc_usb_set_component(usb: *mut snd_soc_usb, component: *mut snd_soc_component);
    fn snd_soc_usb_set_priv_data(usb: *mut snd_soc_usb, data: *mut c_void);
    fn snd_soc_usb_update_offload_route_info(
        usb: *mut snd_soc_usb,
    ) -> Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            c_int,
            c_int,
            c_int,
            snd_soc_usb_kctl,
            *mut c_long,
        ) -> c_int,
    >;
    fn snd_soc_usb_connection_status_cb(
        usb: *mut snd_soc_usb,
    ) -> Option<unsafe extern "C" fn(*mut snd_soc_usb, *mut snd_soc_usb_device, bool)>;
    fn snd_soc_component_card(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_dev(component: *mut snd_soc_component) -> *mut device;
    fn snd_soc_card_dev(card: *mut c_void) -> *mut device;
    fn device_of_node(dev: *mut device) -> *mut device_node;
    fn list_for_each_snd_soc_usb(
        head: *mut list_head,
        cb: unsafe extern "C" fn(*mut snd_soc_usb, *mut c_void) -> bool,
        data: *mut c_void,
    ) -> *mut snd_soc_usb;
    fn list_for_each_snd_soc_usb_safe(
        head: *mut list_head,
        cb: unsafe extern "C" fn(*mut snd_soc_usb, *mut snd_soc_usb, *mut c_void) -> bool,
        data: *mut c_void,
    );
}

unsafe fn snd_soc_find_phandle(dev: *mut device) -> *mut device_node {
    let node: *mut device_node;

    node = unsafe {
        of_parse_phandle(
            device_of_node(dev),
            c"usb-soc-be".as_ptr(),
            0,
        )
    };
    if node.is_null() {
        return unsafe { ERR_PTR(-(ENODEV as c_long)) as *mut device_node };
    }

    node
}

unsafe extern "C" fn snd_soc_usb_ctx_lookup_iter(
    ctx: *mut snd_soc_usb,
    data: *mut c_void,
) -> bool {
    let node = data as *mut device_node;

    unsafe {
        device_of_node(snd_soc_component_dev(snd_soc_usb_component(ctx))) == node
    }
}

unsafe fn snd_soc_usb_ctx_lookup(node: *mut device_node) -> *mut snd_soc_usb {
    if node.is_null() {
        return core::ptr::null_mut();
    }

    unsafe {
        list_for_each_snd_soc_usb(
            core::ptr::addr_of_mut!(usb_ctx_list),
            snd_soc_usb_ctx_lookup_iter,
            node as *mut c_void,
        )
    }
}

unsafe fn snd_soc_find_usb_ctx(dev: *mut device) -> *mut snd_soc_usb {
    let ctx: *mut snd_soc_usb;
    let node: *mut device_node;

    node = unsafe { snd_soc_find_phandle(dev) };
    if unsafe { !IS_ERR(node as *const c_void) } {
        ctx = unsafe { snd_soc_usb_ctx_lookup(node) };
        unsafe { of_node_put(node) };
    } else {
        ctx = unsafe { snd_soc_usb_ctx_lookup(device_of_node(dev)) };
    }

    if !ctx.is_null() {
        ctx
    } else {
        core::ptr::null_mut()
    }
}

/* SOC USB sound kcontrols */
/**
 * snd_soc_usb_setup_offload_jack() - Create USB offloading jack
 * @component: USB DPCM backend DAI component
 * @jack: jack structure to create
 *
 * Creates a jack device for notifying userspace of the availability
 * of an offload capable device.
 *
 * Returns 0 on success, negative on error.
 *
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_usb_setup_offload_jack(
    component: *mut snd_soc_component,
    jack: *mut snd_soc_jack,
) -> c_int {
    let mut ret: c_int;

    ret = unsafe {
        snd_soc_card_jack_new(
            snd_soc_component_card(component),
            c"USB Offload Jack".as_ptr(),
            SND_JACK_USB,
            jack,
        )
    };
    if ret < 0 {
        unsafe {
            dev_err(
                snd_soc_card_dev(snd_soc_component_card(component)),
                c"Unable to add USB offload jack: %d\n".as_ptr(),
                ret,
            )
        };
        return ret;
    }

    ret = unsafe { snd_soc_component_set_jack(component, jack, core::ptr::null_mut()) };
    if ret != 0 {
        unsafe {
            dev_err(
                snd_soc_card_dev(snd_soc_component_card(component)),
                c"Failed to set jack: %d\n".as_ptr(),
                ret,
            )
        };
        return ret;
    }

    0
}

/**
 * snd_soc_usb_update_offload_route - Find active USB offload path
 * @dev: USB device to get offload status
 * @card: USB card index
 * @pcm: USB PCM device index
 * @direction: playback or capture direction
 * @path: pcm or card index
 * @route: pointer to route output array
 *
 * Fetch the current status for the USB SND card and PCM device indexes
 * specified.  The "route" argument should be an array of integers being
 * used for a kcontrol output.  The first element should have the selected
 * card index, and the second element should have the selected pcm device
 * index.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_usb_update_offload_route(
    dev: *mut device,
    card: c_int,
    pcm: c_int,
    direction: c_int,
    path: snd_soc_usb_kctl,
    route: *mut c_long,
) -> c_int {
    let ctx: *mut snd_soc_usb;
    let mut ret: c_int = -ENODEV;

    unsafe { mutex_lock(core::ptr::addr_of_mut!(ctx_mutex)) };
    ctx = unsafe { snd_soc_find_usb_ctx(dev) };
    if ctx.is_null() {
        unsafe { mutex_unlock(core::ptr::addr_of_mut!(ctx_mutex)) };
        return ret;
    }

    if let Some(update_offload_route_info) =
        unsafe { snd_soc_usb_update_offload_route_info(ctx) }
    {
        ret = unsafe {
            update_offload_route_info(
                snd_soc_usb_component(ctx),
                card,
                pcm,
                direction,
                path,
                route,
            )
        };
    }
    unsafe { mutex_unlock(core::ptr::addr_of_mut!(ctx_mutex)) };

    ret
}

/**
 * snd_soc_usb_find_priv_data() - Retrieve private data stored
 * @usbdev: device reference
 *
 * Fetch the private data stored in the USB SND SoC structure.
 *
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_usb_find_priv_data(usbdev: *mut device) -> *mut c_void {
    let ctx: *mut snd_soc_usb;

    unsafe { mutex_lock(core::ptr::addr_of_mut!(ctx_mutex)) };
    ctx = unsafe { snd_soc_find_usb_ctx(usbdev) };
    unsafe { mutex_unlock(core::ptr::addr_of_mut!(ctx_mutex)) };

    if !ctx.is_null() {
        unsafe { snd_soc_usb_priv_data(ctx) }
    } else {
        core::ptr::null_mut()
    }
}

/**
 * snd_soc_usb_find_supported_format() - Check if audio format is supported
 * @card_idx: USB sound chip array index
 * @params: PCM parameters
 * @direction: capture or playback
 *
 * Ensure that a requested audio profile from the ASoC side is able to be
 * supported by the USB device.
 *
 * Return 0 on success, negative on error.
 *
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_usb_find_supported_format(
    card_idx: c_int,
    params: *mut snd_pcm_hw_params,
    direction: c_int,
) -> c_int {
    let as_: *mut snd_usb_stream;

    as_ = unsafe { snd_usb_find_suppported_substream(card_idx, params, direction) };
    if as_.is_null() {
        return -EOPNOTSUPP;
    }

    0
}

/**
 * snd_soc_usb_allocate_port() - allocate a SoC USB port for offloading support
 * @component: USB DPCM backend DAI component
 * @data: private data
 *
 * Allocate and initialize a SoC USB port.  The SoC USB port is used to communicate
 * different USB audio devices attached, in order to start audio offloading handled
 * by an ASoC entity.  USB device plug in/out events are signaled with a
 * notification, but don't directly impact the memory allocated for the SoC USB
 * port.
 *
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_usb_allocate_port(
    component: *mut snd_soc_component,
    data: *mut c_void,
) -> *mut snd_soc_usb {
    let usb: *mut snd_soc_usb;

    usb = unsafe { kzalloc_obj(core::mem::size_of::<snd_soc_usb>()) as *mut snd_soc_usb };
    if usb.is_null() {
        return unsafe { ERR_PTR(-(ENOMEM as c_long)) as *mut snd_soc_usb };
    }

    unsafe {
        snd_soc_usb_set_component(usb, component);
        snd_soc_usb_set_priv_data(usb, data);
    }

    usb
}

/**
 * snd_soc_usb_free_port() - free a SoC USB port used for offloading support
 * @usb: allocated SoC USB port
 *
 * Free and remove the SoC USB port from the available list of ports.  This will
 * ensure that the communication between USB SND and ASoC is halted.
 *
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_usb_free_port(usb: *mut snd_soc_usb) {
    unsafe {
        snd_soc_usb_remove_port(usb);
        kfree(usb as *mut c_void);
    }
}

/**
 * snd_soc_usb_add_port() - Add a USB backend port
 * @usb: soc usb port to add
 *
 * Register a USB backend DAI link to the USB SoC framework.  Memory is allocated
 * as part of the USB backend DAI link.
 *
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_usb_add_port(usb: *mut snd_soc_usb) {
    unsafe {
        mutex_lock(core::ptr::addr_of_mut!(ctx_mutex));
        list_add_tail(snd_soc_usb_list(usb), core::ptr::addr_of_mut!(usb_ctx_list));
        mutex_unlock(core::ptr::addr_of_mut!(ctx_mutex));

        snd_usb_rediscover_devices();
    }
}

unsafe extern "C" fn snd_soc_usb_remove_port_iter(
    ctx: *mut snd_soc_usb,
    _tmp: *mut snd_soc_usb,
    data: *mut c_void,
) -> bool {
    let usb = data as *mut snd_soc_usb;

    if ctx == usb {
        unsafe { list_del(snd_soc_usb_list(ctx)) };
        return true;
    }

    false
}

/**
 * snd_soc_usb_remove_port() - Remove a USB backend port
 * @usb: soc usb port to remove
 *
 * Remove a USB backend DAI link from USB SoC.  Memory is freed when USB backend
 * DAI is removed, or when snd_soc_usb_free_port() is called.
 *
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_usb_remove_port(usb: *mut snd_soc_usb) {
    unsafe {
        mutex_lock(core::ptr::addr_of_mut!(ctx_mutex));
        list_for_each_snd_soc_usb_safe(
            core::ptr::addr_of_mut!(usb_ctx_list),
            snd_soc_usb_remove_port_iter,
            usb as *mut c_void,
        );
        mutex_unlock(core::ptr::addr_of_mut!(ctx_mutex));
    }
}

/**
 * snd_soc_usb_connect() - Notification of USB device connection
 * @usbdev: USB bus device
 * @sdev: USB SND device to add
 *
 * Notify of a new USB SND device connection.  The sdev->card_idx can be used to
 * handle how the DPCM backend selects, which device to enable USB offloading
 * on.
 *
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_usb_connect(
    usbdev: *mut device,
    sdev: *mut snd_soc_usb_device,
) -> c_int {
    let ctx: *mut snd_soc_usb;

    if usbdev.is_null() {
        return -ENODEV;
    }

    unsafe { mutex_lock(core::ptr::addr_of_mut!(ctx_mutex)) };
    ctx = unsafe { snd_soc_find_usb_ctx(usbdev) };
    if ctx.is_null() {
        unsafe { mutex_unlock(core::ptr::addr_of_mut!(ctx_mutex)) };
        return 0;
    }

    if let Some(connection_status_cb) = unsafe { snd_soc_usb_connection_status_cb(ctx) } {
        unsafe { connection_status_cb(ctx, sdev, true) };
    }

    unsafe { mutex_unlock(core::ptr::addr_of_mut!(ctx_mutex)) };

    0
}

/**
 * snd_soc_usb_disconnect() - Notification of USB device disconnection
 * @usbdev: USB bus device
 * @sdev: USB SND device to remove
 *
 * Notify of a new USB SND device disconnection to the USB backend.
 *
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_usb_disconnect(
    usbdev: *mut device,
    sdev: *mut snd_soc_usb_device,
) -> c_int {
    let ctx: *mut snd_soc_usb;

    if usbdev.is_null() {
        return -ENODEV;
    }

    unsafe { mutex_lock(core::ptr::addr_of_mut!(ctx_mutex)) };
    ctx = unsafe { snd_soc_find_usb_ctx(usbdev) };
    if ctx.is_null() {
        unsafe { mutex_unlock(core::ptr::addr_of_mut!(ctx_mutex)) };
        return 0;
    }

    if let Some(connection_status_cb) = unsafe { snd_soc_usb_connection_status_cb(ctx) } {
        unsafe { connection_status_cb(ctx, sdev, false) };
    }

    unsafe { mutex_unlock(core::ptr::addr_of_mut!(ctx_mutex)) };

    0
}

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("SoC USB driver for offloading");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
