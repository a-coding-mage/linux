// SPDX-License-Identifier: GPL-2.0
/*
 * USB CDC Device Management subdriver
 *
 * Copyright (c) 2012  Bjørn Mork <bjorn@mork.no>
 */

// Dependencies supplied by the corresponding Linux USB/WWAN headers:
// `usb_driver`, `usb_interface`, `usb_endpoint_descriptor`, and
// `wwan_port_type`.

extern "C" {
    pub fn usb_cdc_wdm_register(
        intf: *mut usb_interface,
        ep: *mut usb_endpoint_descriptor,
        bufsize: ::core::ffi::c_int,
        type_: wwan_port_type,
        manage_power: Option<
            unsafe extern "C" fn(
                intf: *mut usb_interface,
                on: ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
    ) -> *mut usb_driver;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
