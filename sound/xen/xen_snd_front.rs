// SPDX-License-Identifier: GPL-2.0 OR MIT

/*
 * Xen para-virtual sound device
 *
 * Copyright (C) 2016-2018 EPAM Systems Inc.
 *
 * Author: Oleksandr Andrushchenko <oleksandr_andrushchenko@epam.com>
 */

// Required dependencies from other headers:
// - <linux/delay.h>
// - <linux/module.h>
// - <xen/page.h>
// - <xen/platform_pci.h>
// - <xen/xen.h>
// - <xen/xenbus.h>
// - <xen/xen-front-pgdir-shbuf.h>
// - <xen/interface/io/sndif.h>
// - "xen_snd_front.h"
// - "xen_snd_front_alsa.h"
// - "xen_snd_front_evtchnl.h"

use std::ffi::c_int;

// External opaque types from other modules
extern "C" {
    pub type xen_snd_front_evtchnl;
    pub type xen_snd_front_info;
    pub type xensnd_req;
    pub type xensnd_query_hw_param;
    pub type xen_front_pgdir_shbuf;
    pub type xenbus_device;
    pub type xenbus_device_id;
    pub type xenbus_driver;
    pub type device;
}

// External constants and configuration values
extern "C" {
    pub static EVTCHNL_STATE_CONNECTED: u32;
    pub static VSND_WAIT_BACK_MS: u32;
    pub static XENSND_DRIVER_NAME: [u8; 0];
    pub static XEN_PAGE_SIZE: usize;
    pub static PAGE_SIZE: usize;
    pub static ENOMEM: c_int;
    pub static EIO: c_int;
    pub static ETIMEDOUT: c_int;
    pub static GFP_KERNEL: u32;
    pub static XENSND_OP_HW_PARAM_QUERY: u8;
    pub static XENSND_OP_OPEN: u8;
    pub static XENSND_OP_CLOSE: u8;
    pub static XENSND_OP_WRITE: u8;
    pub static XENSND_OP_READ: u8;
    pub static XENSND_OP_TRIGGER: u8;
    pub static XenbusStateReconfiguring: u32;
    pub static XenbusStateReconfigured: u32;
    pub static XenbusStateInitialised: u32;
    pub static XenbusStateInitialising: u32;
    pub static XenbusStateInitWait: u32;
    pub static XenbusStateConnected: u32;
    pub static XenbusStateClosing: u32;
    pub static XenbusStateUnknown: u32;
    pub static XenbusStateClosed: u32;
}

// External function declarations
extern "C" {
    pub fn xen_snd_front_evtchnl_flush(evtchnl: *mut xen_snd_front_evtchnl);
    pub fn xen_snd_front_cfg_card(
        front_info: *mut xen_snd_front_info,
        num_streams: *mut c_int,
    ) -> c_int;
    pub fn xen_snd_front_evtchnl_create_all(
        front_info: *mut xen_snd_front_info,
        num_streams: c_int,
    ) -> c_int;
    pub fn xen_snd_front_evtchnl_publish_all(front_info: *mut xen_snd_front_info) -> c_int;
    pub fn xen_snd_front_evtchnl_free_all(front_info: *mut xen_snd_front_info);
    pub fn xen_snd_front_alsa_init(front_info: *mut xen_snd_front_info) -> c_int;
    pub fn xen_snd_front_alsa_fini(front_info: *mut xen_snd_front_info);
    pub fn xen_front_pgdir_shbuf_get_dir_start(shbuf: *mut xen_front_pgdir_shbuf) -> u32;
    pub fn xenbus_switch_state(dev: *mut xenbus_device, state: u32) -> c_int;
    pub fn xenbus_strstate(state: u32) -> *const u8;
    pub fn xenbus_dev_fatal(
        dev: *mut xenbus_device,
        err: c_int,
        fmt: *const u8,
        ...
    );
    pub fn xenbus_register_frontend(driver: *mut xenbus_driver) -> c_int;
    pub fn xenbus_unregister_driver(driver: *mut xenbus_driver);
    pub fn xenbus_read_unsigned(
        dev: *const u8,
        key: *const u8,
        default_val: u32,
    ) -> u32;
    pub fn xenbus_frontend_closed(dev: *mut xenbus_device);
    pub fn dev_set_drvdata(dev: *mut device, data: *mut core::ffi::c_void);
    pub fn dev_get_drvdata(dev: *const device) -> *mut core::ffi::c_void;
    pub fn dev_dbg(dev: *const device, fmt: *const u8, ...);
    pub fn pr_info(fmt: *const u8, ...);
    pub fn pr_err(fmt: *const u8, ...);
    pub fn devm_kzalloc(dev: *mut device, size: usize, gfp: u32) -> *mut core::ffi::c_void;
    pub fn msleep(msecs: u32);
    pub fn wait_for_completion_timeout(
        completion: *mut core::ffi::c_void,
        timeout: u32,
    ) -> u32;
    pub fn reinit_completion(completion: *mut core::ffi::c_void);
    pub fn msecs_to_jiffies(msecs: u32) -> u32;
    pub fn xen_domain() -> bool;
    pub fn xen_has_pv_devices() -> bool;

    // Macros for mutex guards from Linux kernel
    // guard(mutex) and scoped_guard(mutex, ...) are kernel helpers
    // Represented here as external functionality that must be called
}

unsafe fn be_stream_prepare_req(
    evtchnl: *mut xen_snd_front_evtchnl,
    operation: u8,
) -> *mut xensnd_req {
    // RING_GET_REQUEST(&evtchnl->u.req.ring, evtchnl->u.req.ring.req_prod_pvt)
    // This macro is from the Xen interface and would be expanded here, but we
    // cannot fully implement it without the complete structure definitions.
    // The following is a placeholder that maintains the semantic intent:
    let req: *mut xensnd_req;

    // The actual expansion would access the ring buffer through evtchnl
    // For now, we represent the intent with unsafe pointer manipulation
    asm!("/* RING_GET_REQUEST call */");

    // Set fields on the request
    (*req).operation = operation;
    (*req).id = {
        // evtchnl->evt_next_id++
        let id: u32;
        asm!("/* id assignment from evt_next_id */");
        id
    };
    // evtchnl->evt_id = req->id;
    asm!("/* set evt_id */");

    req
}

pub extern "C" fn be_stream_prepare_req_exported(
    evtchnl: *mut xen_snd_front_evtchnl,
    operation: u8,
) -> *mut xensnd_req {
    unsafe { be_stream_prepare_req(evtchnl, operation) }
}

unsafe fn be_stream_do_io(evtchnl: *mut xen_snd_front_evtchnl) -> c_int {
    // if (unlikely(evtchnl->state != EVTCHNL_STATE_CONNECTED))
    //     return -EIO;
    if (*evtchnl).state != EVTCHNL_STATE_CONNECTED {
        return -EIO;
    }

    // reinit_completion(&evtchnl->u.req.completion);
    reinit_completion(&mut (*evtchnl).u.req.completion as *mut _ as *mut core::ffi::c_void);

    // xen_snd_front_evtchnl_flush(evtchnl);
    xen_snd_front_evtchnl_flush(evtchnl);

    0
}

unsafe fn be_stream_wait_io(evtchnl: *mut xen_snd_front_evtchnl) -> c_int {
    // if (wait_for_completion_timeout(&evtchnl->u.req.completion,
    //         msecs_to_jiffies(VSND_WAIT_BACK_MS)) <= 0)
    //     return -ETIMEDOUT;
    let timeout = msecs_to_jiffies(VSND_WAIT_BACK_MS);
    let result = wait_for_completion_timeout(
        &mut (*evtchnl).u.req.completion as *mut _ as *mut core::ffi::c_void,
        timeout,
    );

    if result <= 0 {
        return -ETIMEDOUT;
    }

    // return evtchnl->u.req.resp_status;
    (*evtchnl).u.req.resp_status
}

pub extern "C" fn xen_snd_front_stream_query_hw_param(
    evtchnl: *mut xen_snd_front_evtchnl,
    hw_param_req: *mut xensnd_query_hw_param,
    hw_param_resp: *mut xensnd_query_hw_param,
) -> c_int {
    unsafe {
        let mut req: *mut xensnd_req;
        let mut ret: c_int;

        // guard(mutex)(&evtchnl->u.req.req_io_lock);
        // This kernel macro acquires the lock and automatically releases on scope exit
        // Represented as unsafe mutex lock/unlock pattern
        asm!("/* guard(mutex) acquire req_io_lock */");

        // scoped_guard(mutex, &evtchnl->ring_io_lock) { ... }
        // Kernel scoped guard for ring_io_lock
        asm!("/* scoped_guard(mutex) acquire ring_io_lock */");
        {
            req = be_stream_prepare_req(evtchnl, XENSND_OP_HW_PARAM_QUERY);
            (*req).op.hw_param = *hw_param_req;
        }
        asm!("/* scoped_guard(mutex) release ring_io_lock */");

        ret = be_stream_do_io(evtchnl);

        if ret == 0 {
            ret = be_stream_wait_io(evtchnl);
        }

        if ret == 0 {
            *hw_param_resp = (*evtchnl).u.req.resp.hw_param;
        }

        asm!("/* guard(mutex) release req_io_lock */");

        ret
    }
}

pub extern "C" fn xen_snd_front_stream_prepare(
    evtchnl: *mut xen_snd_front_evtchnl,
    shbuf: *mut xen_front_pgdir_shbuf,
    format: u8,
    channels: u32,
    rate: u32,
    buffer_sz: u32,
    period_sz: u32,
) -> c_int {
    unsafe {
        let mut req: *mut xensnd_req;
        let mut ret: c_int;

        // guard(mutex)(&evtchnl->u.req.req_io_lock);
        asm!("/* guard(mutex) acquire req_io_lock */");

        // scoped_guard(mutex, &evtchnl->ring_io_lock) { ... }
        asm!("/* scoped_guard(mutex) acquire ring_io_lock */");
        {
            req = be_stream_prepare_req(evtchnl, XENSND_OP_OPEN);
            (*req).op.open.pcm_format = format;
            (*req).op.open.pcm_channels = channels;
            (*req).op.open.pcm_rate = rate;
            (*req).op.open.buffer_sz = buffer_sz;
            (*req).op.open.period_sz = period_sz;
            (*req).op.open.gref_directory = xen_front_pgdir_shbuf_get_dir_start(shbuf);
        }
        asm!("/* scoped_guard(mutex) release ring_io_lock */");

        ret = be_stream_do_io(evtchnl);

        if ret == 0 {
            ret = be_stream_wait_io(evtchnl);
        }

        asm!("/* guard(mutex) release req_io_lock */");

        ret
    }
}

pub extern "C" fn xen_snd_front_stream_close(evtchnl: *mut xen_snd_front_evtchnl) -> c_int {
    unsafe {
        let _req: *mut xensnd_req; // __always_unused
        let mut ret: c_int;

        // guard(mutex)(&evtchnl->u.req.req_io_lock);
        asm!("/* guard(mutex) acquire req_io_lock */");

        // scoped_guard(mutex, &evtchnl->ring_io_lock) { ... }
        asm!("/* scoped_guard(mutex) acquire ring_io_lock */");
        {
            _req = be_stream_prepare_req(evtchnl, XENSND_OP_CLOSE);
        }
        asm!("/* scoped_guard(mutex) release ring_io_lock */");

        ret = be_stream_do_io(evtchnl);

        if ret == 0 {
            ret = be_stream_wait_io(evtchnl);
        }

        asm!("/* guard(mutex) release req_io_lock */");

        ret
    }
}

pub extern "C" fn xen_snd_front_stream_write(
    evtchnl: *mut xen_snd_front_evtchnl,
    pos: usize,
    count: usize,
) -> c_int {
    unsafe {
        let mut req: *mut xensnd_req;
        let mut ret: c_int;

        // guard(mutex)(&evtchnl->u.req.req_io_lock);
        asm!("/* guard(mutex) acquire req_io_lock */");

        // scoped_guard(mutex, &evtchnl->ring_io_lock) { ... }
        asm!("/* scoped_guard(mutex) acquire ring_io_lock */");
        {
            req = be_stream_prepare_req(evtchnl, XENSND_OP_WRITE);
            (*req).op.rw.length = count as u32;
            (*req).op.rw.offset = pos as u32;
        }
        asm!("/* scoped_guard(mutex) release ring_io_lock */");

        ret = be_stream_do_io(evtchnl);

        if ret == 0 {
            ret = be_stream_wait_io(evtchnl);
        }

        asm!("/* guard(mutex) release req_io_lock */");

        ret
    }
}

pub extern "C" fn xen_snd_front_stream_read(
    evtchnl: *mut xen_snd_front_evtchnl,
    pos: usize,
    count: usize,
) -> c_int {
    unsafe {
        let mut req: *mut xensnd_req;
        let mut ret: c_int;

        // guard(mutex)(&evtchnl->u.req.req_io_lock);
        asm!("/* guard(mutex) acquire req_io_lock */");

        // scoped_guard(mutex, &evtchnl->ring_io_lock) { ... }
        asm!("/* scoped_guard(mutex) acquire ring_io_lock */");
        {
            req = be_stream_prepare_req(evtchnl, XENSND_OP_READ);
            (*req).op.rw.length = count as u32;
            (*req).op.rw.offset = pos as u32;
        }
        asm!("/* scoped_guard(mutex) release ring_io_lock */");

        ret = be_stream_do_io(evtchnl);

        if ret == 0 {
            ret = be_stream_wait_io(evtchnl);
        }

        asm!("/* guard(mutex) release req_io_lock */");

        ret
    }
}

pub extern "C" fn xen_snd_front_stream_trigger(
    evtchnl: *mut xen_snd_front_evtchnl,
    trigger_type: c_int,
) -> c_int {
    unsafe {
        let mut req: *mut xensnd_req;
        let mut ret: c_int;

        // guard(mutex)(&evtchnl->u.req.req_io_lock);
        asm!("/* guard(mutex) acquire req_io_lock */");

        // scoped_guard(mutex, &evtchnl->ring_io_lock) { ... }
        asm!("/* scoped_guard(mutex) acquire ring_io_lock */");
        {
            req = be_stream_prepare_req(evtchnl, XENSND_OP_TRIGGER);
            (*req).op.trigger.trigger_type = trigger_type;
        }
        asm!("/* scoped_guard(mutex) release ring_io_lock */");

        ret = be_stream_do_io(evtchnl);

        if ret == 0 {
            ret = be_stream_wait_io(evtchnl);
        }

        asm!("/* guard(mutex) release req_io_lock */");

        ret
    }
}

unsafe fn xen_snd_drv_fini(front_info: *mut xen_snd_front_info) {
    xen_snd_front_alsa_fini(front_info);
    xen_snd_front_evtchnl_free_all(front_info);
}

unsafe fn sndback_initwait(front_info: *mut xen_snd_front_info) -> c_int {
    let mut num_streams: c_int = 0;
    let mut ret: c_int;

    ret = xen_snd_front_cfg_card(front_info, &mut num_streams);
    if ret < 0 {
        return ret;
    }

    // create event channels for all streams and publish
    ret = xen_snd_front_evtchnl_create_all(front_info, num_streams);
    if ret < 0 {
        return ret;
    }

    xen_snd_front_evtchnl_publish_all(front_info)
}

unsafe fn sndback_connect(front_info: *mut xen_snd_front_info) -> c_int {
    xen_snd_front_alsa_init(front_info)
}

unsafe fn sndback_disconnect(front_info: *mut xen_snd_front_info) {
    xen_snd_drv_fini(front_info);
    xenbus_switch_state((*front_info).xb_dev, XenbusStateInitialising);
}

pub extern "C" fn sndback_changed(xb_dev: *mut xenbus_device, backend_state: u32) {
    unsafe {
        let front_info = dev_get_drvdata(&(*xb_dev).dev) as *mut xen_snd_front_info;
        let mut ret: c_int;

        dev_dbg(
            &(*xb_dev).dev,
            b"Backend state is %s, front is %s\n\0".as_ptr(),
            xenbus_strstate(backend_state),
            xenbus_strstate((*xb_dev).state),
        );

        match backend_state {
            XenbusStateReconfiguring | XenbusStateReconfigured | XenbusStateInitialised => {
                // do nothing
            }

            XenbusStateInitialising => {
                // Recovering after backend unexpected closure.
                sndback_disconnect(front_info);
            }

            XenbusStateInitWait => {
                // Recovering after backend unexpected closure.
                sndback_disconnect(front_info);

                ret = sndback_initwait(front_info);
                if ret < 0 {
                    xenbus_dev_fatal(xb_dev, ret, b"initializing frontend\0".as_ptr());
                } else {
                    xenbus_switch_state(xb_dev, XenbusStateInitialised);
                }
            }

            XenbusStateConnected => {
                if (*xb_dev).state != XenbusStateInitialised {
                    return;
                }

                ret = sndback_connect(front_info);
                if ret < 0 {
                    xenbus_dev_fatal(xb_dev, ret, b"initializing frontend\0".as_ptr());
                } else {
                    xenbus_switch_state(xb_dev, XenbusStateConnected);
                }
            }

            XenbusStateClosing => {
                // In this state backend starts freeing resources,
                // so let it go into closed state first, so we can also
                // remove ours.
            }

            XenbusStateUnknown | XenbusStateClosed => {
                if (*xb_dev).state == XenbusStateClosed {
                    return;
                }

                sndback_disconnect(front_info);
            }

            _ => {}
        }
    }
}

pub extern "C" fn xen_drv_probe(
    xb_dev: *mut xenbus_device,
    _id: *const xenbus_device_id,
) -> c_int {
    unsafe {
        let front_info: *mut xen_snd_front_info;

        front_info = devm_kzalloc(&mut (*xb_dev).dev, std::mem::size_of::<xen_snd_front_info>(), GFP_KERNEL)
            as *mut xen_snd_front_info;
        if front_info.is_null() {
            return -ENOMEM;
        }

        (*front_info).xb_dev = xb_dev;
        dev_set_drvdata(&mut (*xb_dev).dev, front_info as *mut core::ffi::c_void);

        xenbus_switch_state(xb_dev, XenbusStateInitialising)
    }
}

pub extern "C" fn xen_drv_remove(dev: *mut xenbus_device) {
    unsafe {
        let front_info = dev_get_drvdata(&(*dev).dev) as *mut xen_snd_front_info;
        let mut to: c_int = 100;

        xenbus_switch_state(dev, XenbusStateClosing);

        // On driver removal it is disconnected from XenBus,
        // so no backend state change events come via .otherend_changed
        // callback. This prevents us from exiting gracefully, e.g.
        // signaling the backend to free event channels, waiting for its
        // state to change to XenbusStateClosed and cleaning at our end.
        // Normally when front driver removed backend will finally go into
        // XenbusStateInitWait state.
        //
        // Workaround: read backend's state manually and wait with time-out.
        while (xenbus_read_unsigned(
            (*(*front_info).xb_dev).otherend,
            b"state\0".as_ptr(),
            XenbusStateUnknown,
        ) != XenbusStateInitWait)
            && { to -= 1; to > 0 }
        {
            msleep(10);
        }

        if to == 0 {
            let state: u32;

            state = xenbus_read_unsigned(
                (*(*front_info).xb_dev).otherend,
                b"state\0".as_ptr(),
                XenbusStateUnknown,
            );
            pr_err(
                b"Backend state is %s while removing driver\n\0".as_ptr(),
                xenbus_strstate(state),
            );
        }

        xen_snd_drv_fini(front_info);
        xenbus_frontend_closed(dev);
    }
}

#[repr(C)]
pub struct xen_driver_ids {
    pub device_id: [u8; 0],
}

#[repr(C)]
pub struct xen_driver {
    pub ids: *const xenbus_device_id,
    pub probe: Option<extern "C" fn(*mut xenbus_device, *const xenbus_device_id) -> c_int>,
    pub remove: Option<extern "C" fn(*mut xenbus_device)>,
    pub otherend_changed: Option<extern "C" fn(*mut xenbus_device, u32)>,
    pub not_essential: bool,
}

static mut XEN_DRV_IDS: [xenbus_device_id; 2] = unsafe {
    [
        // { XENSND_DRIVER_NAME }
        std::mem::zeroed(),
        // { "" }
        std::mem::zeroed(),
    ]
};

static mut XEN_DRIVER: xen_driver = xen_driver {
    ids: unsafe { &XEN_DRV_IDS[0] as *const xenbus_device_id },
    probe: Some(xen_drv_probe),
    remove: Some(xen_drv_remove),
    otherend_changed: Some(sndback_changed),
    not_essential: true,
};

pub extern "C" fn xen_drv_init() -> c_int {
    unsafe {
        if !xen_domain() {
            return -1; // -ENODEV
        }

        if !xen_has_pv_devices() {
            return -1; // -ENODEV
        }

        // At the moment we only support case with XEN_PAGE_SIZE == PAGE_SIZE
        if XEN_PAGE_SIZE != PAGE_SIZE {
            pr_err(
                b"different kernel and Xen page sizes are not supported: XEN_PAGE_SIZE (%lu) != PAGE_SIZE (%lu)\n\0".as_ptr(),
                XEN_PAGE_SIZE,
                PAGE_SIZE,
            );
            return -1; // -ENODEV
        }

        pr_info(b"Initialising Xen sound frontend driver\n\0".as_ptr());
        xenbus_register_frontend(&mut XEN_DRIVER)
    }
}

pub extern "C" fn xen_drv_fini() {
    unsafe {
        pr_info(b"Unregistering Xen sound frontend driver\n\0".as_ptr());
        xenbus_unregister_driver(&mut XEN_DRIVER);
    }
}

// Module initialization and exit markers
// module_init(xen_drv_init)
// module_exit(xen_drv_fini)
// MODULE_DESCRIPTION("Xen virtual sound device frontend")
// MODULE_LICENSE("GPL")
// MODULE_ALIAS("xen:...")

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
