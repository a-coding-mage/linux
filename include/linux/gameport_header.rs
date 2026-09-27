/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Copyright (c) 1999-2002 Vojtech Pavlik
 */

// Dependencies correspond to the C header's kernel includes.

#[repr(C)]
pub struct gameport {
    pub port_data: *mut kernel::ffi::c_void,
    pub name: [kernel::ffi::c_char; 32],
    pub phys: [kernel::ffi::c_char; 32],

    pub io: kernel::ffi::c_int,
    pub speed: kernel::ffi::c_int,
    pub fuzz: kernel::ffi::c_int,

    pub trigger: Option<unsafe extern "C" fn(*mut gameport)>,
    pub read: Option<unsafe extern "C" fn(*mut gameport) -> kernel::ffi::c_uchar>,
    pub cooked_read: Option<unsafe extern "C" fn(*mut gameport, *mut kernel::ffi::c_int, *mut kernel::ffi::c_int) -> kernel::ffi::c_int>,
    pub calibrate: Option<unsafe extern "C" fn(*mut gameport, *mut kernel::ffi::c_int, *mut kernel::ffi::c_int) -> kernel::ffi::c_int>,
    pub open: Option<unsafe extern "C" fn(*mut gameport, kernel::ffi::c_int) -> kernel::ffi::c_int>,
    pub close: Option<unsafe extern "C" fn(*mut gameport)>,

    pub poll_timer: crate::timer_list,
    pub poll_interval: kernel::ffi::c_uint,
    pub timer_lock: crate::spinlock_t,
    pub poll_cnt: kernel::ffi::c_uint,
    pub poll_handler: Option<unsafe extern "C" fn(*mut gameport)>,

    pub parent: *mut gameport,
    pub child: *mut gameport,
    pub drv: *mut gameport_driver,
    pub drv_mutex: crate::mutex,
    pub dev: crate::device,
    pub node: crate::list_head,
}

// C macro: container_of(d, struct gameport, dev)

#[repr(C)]
pub struct gameport_driver {
    pub description: *const kernel::ffi::c_char,
    pub connect: Option<unsafe extern "C" fn(*mut gameport, *mut gameport_driver) -> kernel::ffi::c_int>,
    pub reconnect: Option<unsafe extern "C" fn(*mut gameport) -> kernel::ffi::c_int>,
    pub disconnect: Option<unsafe extern "C" fn(*mut gameport)>,
    pub driver: crate::device_driver,
    pub ignore: bool,
}

// C macro: container_of_const(d, struct gameport_driver, driver)

extern "C" {
    pub fn gameport_open(gameport: *mut gameport, drv: *mut gameport_driver, mode: kernel::ffi::c_int) -> kernel::ffi::c_int;
    pub fn gameport_close(gameport: *mut gameport);

    pub fn __gameport_register_port(gameport: *mut gameport, owner: *mut crate::module);
    pub fn gameport_unregister_port(gameport: *mut gameport);
    pub fn gameport_set_phys(gameport: *mut gameport, fmt: *const kernel::ffi::c_char, ...);

    pub fn gameport_register_driver(drv: *mut gameport_driver, owner: *mut crate::module, mod_name: *const kernel::ffi::c_char) -> kernel::ffi::c_int;
    pub fn gameport_unregister_driver(drv: *mut gameport_driver);

    pub fn gameport_start_polling(gameport: *mut gameport);
    pub fn gameport_stop_polling(gameport: *mut gameport);
}

#[inline]
pub unsafe fn gameport_trigger(gameport: *mut gameport) {
    ((*gameport).trigger.unwrap())(gameport);
}

#[inline]
pub unsafe fn gameport_read(gameport: *mut gameport) -> kernel::ffi::c_uchar {
    ((*gameport).read.unwrap())(gameport)
}

#[inline]
pub unsafe fn gameport_cooked_read(gameport: *mut gameport, axes: *mut kernel::ffi::c_int, buttons: *mut kernel::ffi::c_int) -> kernel::ffi::c_int {
    match (*gameport).cooked_read {
        Some(f) => f(gameport, axes, buttons),
        None => -1,
    }
}

#[inline]
pub unsafe fn gameport_calibrate(gameport: *mut gameport, axes: *mut kernel::ffi::c_int, max: *mut kernel::ffi::c_int) -> kernel::ffi::c_int {
    match (*gameport).calibrate {
        Some(f) => f(gameport, axes, max),
        None => -1,
    }
}

#[inline]
pub unsafe fn gameport_time(gameport: *mut gameport, time: kernel::ffi::c_int) -> kernel::ffi::c_int {
    time.wrapping_mul((*gameport).speed) / 1000
}

#[inline]
pub unsafe fn gameport_set_poll_handler(gameport: *mut gameport, handler: Option<unsafe extern "C" fn(*mut gameport)>) {
    (*gameport).poll_handler = handler;
}

#[inline]
pub unsafe fn gameport_set_poll_interval(gameport: *mut gameport, msecs: kernel::ffi::c_uint) {
    (*gameport).poll_interval = msecs;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
