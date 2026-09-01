/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Apple Onboard Audio GPIO definitions
 *
 * Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
 */

/* Dependencies from C includes:
 * <linux/workqueue.h> provides struct delayed_work.
 * <linux/mutex.h> provides struct mutex.
 */

pub type notify_func_t = Option<unsafe extern "C" fn(data: *mut ::core::ffi::c_void)>;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum notify_type {
    AOA_NOTIFY_HEADPHONE,
    AOA_NOTIFY_LINE_IN,
    AOA_NOTIFY_LINE_OUT,
}

#[repr(C)]
pub struct gpio_runtime {
    /*
     * to be assigned by fabric
     */
    pub node: *mut device_node,
    /*
     * since everyone needs this pointer anyway...
     */
    pub methods: *mut gpio_methods,
    /*
     * to be used by the gpio implementation
     */
    pub implementation_private: ::core::ffi::c_int,
    pub headphone_notify: gpio_notification,
    pub line_in_notify: gpio_notification,
    pub line_out_notify: gpio_notification,
}

#[repr(C)]
pub struct gpio_methods {
    /*
     * for initialisation/de-initialisation of the GPIO layer
     */
    pub init: Option<unsafe extern "C" fn(rt: *mut gpio_runtime)>,
    pub exit: Option<unsafe extern "C" fn(rt: *mut gpio_runtime)>,

    /*
     * turn off headphone, speakers, lineout
     */
    pub all_amps_off: Option<unsafe extern "C" fn(rt: *mut gpio_runtime)>,
    /*
     * turn headphone, speakers, lineout back to previous setting
     */
    pub all_amps_restore: Option<unsafe extern "C" fn(rt: *mut gpio_runtime)>,

    pub set_headphone:
        Option<unsafe extern "C" fn(rt: *mut gpio_runtime, on: ::core::ffi::c_int)>,
    pub set_speakers:
        Option<unsafe extern "C" fn(rt: *mut gpio_runtime, on: ::core::ffi::c_int)>,
    pub set_lineout:
        Option<unsafe extern "C" fn(rt: *mut gpio_runtime, on: ::core::ffi::c_int)>,
    pub set_master:
        Option<unsafe extern "C" fn(rt: *mut gpio_runtime, on: ::core::ffi::c_int)>,

    pub get_headphone: Option<unsafe extern "C" fn(rt: *mut gpio_runtime) -> ::core::ffi::c_int>,
    pub get_speakers: Option<unsafe extern "C" fn(rt: *mut gpio_runtime) -> ::core::ffi::c_int>,
    pub get_lineout: Option<unsafe extern "C" fn(rt: *mut gpio_runtime) -> ::core::ffi::c_int>,
    pub get_master: Option<unsafe extern "C" fn(rt: *mut gpio_runtime) -> ::core::ffi::c_int>,

    pub set_hw_reset:
        Option<unsafe extern "C" fn(rt: *mut gpio_runtime, on: ::core::ffi::c_int)>,

    /*
     * use this to be notified of any events. The notification
     * function is passed the data, and is called in process
     * context by the use of schedule_work.
     * The interface for it is that setting a function to NULL
     * removes it, and they return 0 if the operation succeeded,
     * and -EBUSY if the notification is already assigned by
     * someone else.
     */
    pub set_notify: Option<
        unsafe extern "C" fn(
            rt: *mut gpio_runtime,
            type_: notify_type,
            notify: notify_func_t,
            data: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    /*
     * returns 0 if not plugged in, 1 if plugged in
     * or a negative error code
     */
    pub get_detect: Option<
        unsafe extern "C" fn(rt: *mut gpio_runtime, type_: notify_type) -> ::core::ffi::c_int,
    >,
}

#[repr(C)]
pub struct gpio_notification {
    pub work: delayed_work,
    pub notify: notify_func_t,
    pub data: *mut ::core::ffi::c_void,
    pub gpio_private: *mut ::core::ffi::c_void,
    pub mutex: mutex,
}

#[repr(C)]
pub struct delayed_work {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _unused: [u8; 0],
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
