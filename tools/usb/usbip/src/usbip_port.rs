// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2011 matt mooney <mfm@muteddisk.com>
 *               2005-2007 Takahiro Hirofuchi
 */

// C dependencies: "vhci_driver.h", "usbip_common.h"

use std::os::raw::{c_char, c_int};

#[repr(C)]
pub struct usbip_imported_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct usbip_vhci_driver {
    pub nports: c_int,
    pub idev: *mut usbip_imported_device,
}

unsafe extern "C" {
    static USBIDS_FILE: *const c_char;
    static mut vhci_driver: *mut usbip_vhci_driver;

    fn usbip_names_init(filename: *const c_char) -> c_int;
    fn usbip_names_free();
    fn usbip_vhci_driver_open() -> c_int;
    fn usbip_vhci_driver_close();
    fn usbip_vhci_imported_device_dump(idev: *mut usbip_imported_device) -> c_int;

    fn printf(format: *const c_char, ...) -> c_int;
    fn err(format: *const c_char, ...);
}

unsafe fn list_imported_devices() -> c_int {
    let mut i: c_int;
    let mut idev: *mut usbip_imported_device;
    let ret: c_int;

    if usbip_names_init(USBIDS_FILE) != 0 {
        err(c"failed to open %s".as_ptr(), USBIDS_FILE);
    }

    ret = usbip_vhci_driver_open();
    if ret < 0 {
        err(c"open vhci_driver (is vhci_hcd loaded?)".as_ptr());
        goto_err_names_free();
        return -1;
    }

    printf(c"Imported USB devices\n".as_ptr());
    printf(c"====================\n".as_ptr());

    i = 0;
    while i < (*vhci_driver).nports {
        idev = (*vhci_driver).idev.offset(i as isize);

        if usbip_vhci_imported_device_dump(idev) < 0 {
            usbip_vhci_driver_close();
            goto_err_names_free();
            return -1;
        }

        i += 1;
    }

    usbip_vhci_driver_close();
    usbip_names_free();

    ret
}

unsafe fn goto_err_names_free() {
    usbip_names_free();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn usbip_port_show(
    _argc: c_int,
    _argv: *mut *mut c_char,
) -> c_int {
    let ret: c_int;

    ret = list_imported_devices();
    if ret < 0 {
        err(c"list imported devices".as_ptr());
    }

    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
