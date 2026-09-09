// SPDX-License-Identifier: GPL-2.0-only
/*
 * Maxim Integrated MAX3355 USB OTG chip extcon driver
 *
 * Copyright (C)  2014-2015 Cogent Embedded, Inc.
 * Author: Sergei Shtylyov <sergei.shtylyov@cogentembedded.com>
 */

// Dependencies supplied by the surrounding kernel bindings.

#[repr(C)]
pub struct max3355_data {
    pub edev: *mut extcon_dev,
    pub id_gpiod: *mut gpio_desc,
    pub shdn_gpiod: *mut gpio_desc,
}

extern "C" {
    type extcon_dev;
    type gpio_desc;
    type platform_device;

    static EXTCON_USB: ::core::ffi::c_uint;
    static EXTCON_USB_HOST: ::core::ffi::c_uint;
    static EXTCON_NONE: ::core::ffi::c_uint;

    fn gpiod_get_value_cansleep(desc: *mut gpio_desc) -> ::core::ffi::c_int;
    fn extcon_set_state_sync(
        edev: *mut extcon_dev,
        cable: ::core::ffi::c_uint,
        state: bool,
    ) -> ::core::ffi::c_int;
    fn devm_kzalloc(
        dev: *mut core::ffi::c_void,
        size: usize,
        flags: ::core::ffi::c_uint,
    ) -> *mut core::ffi::c_void;
    fn devm_gpiod_get(
        dev: *mut core::ffi::c_void,
        con_id: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_ulong,
    ) -> *mut gpio_desc;
    fn devm_extcon_dev_allocate(
        dev: *mut core::ffi::c_void,
        supported_cables: *const ::core::ffi::c_uint,
    ) -> *mut extcon_dev;
    fn devm_extcon_dev_register(
        dev: *mut core::ffi::c_void,
        edev: *mut extcon_dev,
    ) -> ::core::ffi::c_int;
    fn gpiod_to_irq(desc: *mut gpio_desc) -> ::core::ffi::c_int;
    fn devm_request_threaded_irq(
        dev: *mut core::ffi::c_void,
        irq: ::core::ffi::c_int,
        handler: Option<unsafe extern "C" fn(::core::ffi::c_int, *mut core::ffi::c_void) -> irqreturn_t>,
        thread_fn: Option<unsafe extern "C" fn(::core::ffi::c_int, *mut core::ffi::c_void) -> irqreturn_t>,
        flags: ::core::ffi::c_ulong,
        name: *const ::core::ffi::c_char,
        data: *mut core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut core::ffi::c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut max3355_data;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: ::core::ffi::c_int);
}

pub type irqreturn_t = ::core::ffi::c_uint;
pub const IRQ_HANDLED: irqreturn_t = 1;

static MAX3355_CABLE: [::core::ffi::c_uint; 3] = [
    unsafe { EXTCON_USB },
    unsafe { EXTCON_USB_HOST },
    unsafe { EXTCON_NONE },
];

pub unsafe extern "C" fn max3355_id_irq(
    _irq: ::core::ffi::c_int,
    dev_id: *mut core::ffi::c_void,
) -> irqreturn_t {
    let data = dev_id as *mut max3355_data;
    let id = gpiod_get_value_cansleep((*data).id_gpiod);

    if id != 0 {
        /* ID = 1 means USB HOST cable detached. */
        extcon_set_state_sync((*data).edev, EXTCON_USB_HOST, false);
        extcon_set_state_sync((*data).edev, EXTCON_USB, true);
    } else {
        /* ID = 0 means USB HOST cable attached. */
        extcon_set_state_sync((*data).edev, EXTCON_USB, false);
        extcon_set_state_sync((*data).edev, EXTCON_USB_HOST, true);
    }

    IRQ_HANDLED
}

pub unsafe extern "C" fn max3355_probe(pdev: *mut platform_device) -> ::core::ffi::c_int {
    let data = devm_kzalloc(
        pdev as *mut core::ffi::c_void,
        core::mem::size_of::<max3355_data>(),
        0,
    ) as *mut max3355_data;
    if data.is_null() {
        return -12;
    }

    (*data).id_gpiod = devm_gpiod_get(pdev as *mut core::ffi::c_void, b"id\0".as_ptr() as _, 0);
    if (*data).id_gpiod.is_null() {
        return -1;
    }
    (*data).shdn_gpiod = devm_gpiod_get(
        pdev as *mut core::ffi::c_void,
        b"maxim,shdn\0".as_ptr() as _,
        0,
    );
    if (*data).shdn_gpiod.is_null() {
        return -1;
    }

    (*data).edev = devm_extcon_dev_allocate(pdev as *mut core::ffi::c_void, MAX3355_CABLE.as_ptr());
    if (*data).edev.is_null() {
        return -1;
    }
    let mut err = devm_extcon_dev_register(pdev as *mut core::ffi::c_void, (*data).edev);
    if err < 0 {
        return err;
    }

    let irq = gpiod_to_irq((*data).id_gpiod);
    if irq < 0 {
        return irq;
    }
    err = devm_request_threaded_irq(
        pdev as *mut core::ffi::c_void,
        irq,
        None,
        Some(max3355_id_irq),
        0,
        core::ptr::null(),
        data as *mut core::ffi::c_void,
    );
    if err < 0 {
        return err;
    }
    platform_set_drvdata(pdev, data as *mut core::ffi::c_void);
    max3355_id_irq(irq, data as *mut core::ffi::c_void);
    0
}

pub unsafe extern "C" fn max3355_remove(pdev: *mut platform_device) {
    let data = platform_get_drvdata(pdev);
    gpiod_set_value_cansleep((*data).shdn_gpiod, 0);
}

// Device-match table, platform-driver registration, and module metadata are
// supplied by the kernel's build-time registration macros.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
