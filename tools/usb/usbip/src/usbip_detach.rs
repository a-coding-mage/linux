// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2011 matt mooney <mfm@muteddisk.com>
 *               2005-2007 Takahiro Hirofuchi
 */

use core::ffi::{c_char, c_int, c_uint, c_uchar, c_void};

// C dependencies from:
// <ctype.h>, <limits.h>, <stdint.h>, <stdio.h>, <stdlib.h>, <string.h>,
// <getopt.h>, <unistd.h>, "vhci_driver.h", "usbip_common.h",
// "usbip_network.h", "usbip.h"

const PATH_MAX: usize = 4096;
const VDEV_ST_NULL: c_int = 0;

const USBIP_VHCI_DRV_NAME: &str = "vhci_hcd";
const VHCI_STATE_PATH: &str = "/var/run/vhci_hcd";

#[repr(C)]
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

#[repr(C)]
pub struct usbip_imported_device {
    pub port: c_int,
    pub status: c_int,
}

#[repr(C)]
pub struct usbip_vhci_driver {
    pub nports: c_int,
    pub idev: *mut usbip_imported_device,
}

unsafe extern "C" {
    static mut vhci_driver: *mut usbip_vhci_driver;
    static mut optarg: *mut c_char;

    fn printf(format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn atoi(nptr: *const c_char) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn remove(pathname: *const c_char) -> c_int;
    fn rmdir(path: *const c_char) -> c_int;
    fn getopt_long(
        argc: c_int,
        argv: *const *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;

    fn usbip_vhci_driver_open() -> c_int;
    fn usbip_vhci_driver_close();
    fn usbip_vhci_detach_device(port: u8) -> c_int;

    fn err(format: *const c_char, ...);
    fn info(format: *const c_char, ...);
}

static USBIP_DETACH_USAGE_STRING: &[u8] = concat!(
    "usbip detach <args>\n",
    "    -p, --port=<port>    ",
    "vhci_hcd",
    " port the device is on\n\0"
)
.as_bytes();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn usbip_detach_usage() {
    unsafe {
        printf(
            c"usage: %s".as_ptr(),
            USBIP_DETACH_USAGE_STRING.as_ptr() as *const c_char,
        );
    }
}

unsafe fn isdigit_c(c: c_char) -> bool {
    (c as u8).wrapping_sub(b'0') <= 9
}

unsafe fn detach_port(port: *mut c_char) -> c_int {
    let mut ret: c_int = 0;
    let portnum: u8;
    let mut path: [c_char; PATH_MAX + 1] = [0; PATH_MAX + 1];
    let mut i: c_int;
    let mut idev: *mut usbip_imported_device;
    let mut found: c_int = 0;

    let port_len: c_uint = unsafe { strlen(port) as c_uint };

    let mut scan_i: c_uint = 0;
    while scan_i < port_len {
        if unsafe { !isdigit_c(*port.add(scan_i as usize)) } {
            unsafe {
                err(c"invalid port %s".as_ptr(), port);
            }
            return -1;
        }
        scan_i = scan_i.wrapping_add(1);
    }

    portnum = unsafe { atoi(port) as u8 };

    ret = unsafe { usbip_vhci_driver_open() };
    if ret < 0 {
        unsafe {
            err(c"open vhci_driver (is vhci_hcd loaded?)".as_ptr());
        }
        return -1;
    }

    /* check for invalid port */
    i = 0;
    while unsafe { i < (*vhci_driver).nports } {
        unsafe {
            idev = (*vhci_driver).idev.add(i as usize);

            if (*idev).port == portnum as c_int {
                found = 1;
                if (*idev).status != VDEV_ST_NULL {
                    break;
                }
                info(c"Port %d is already detached!\n".as_ptr(), (*idev).port);
                goto_call_driver_close(&mut ret);
                return ret;
            }
        }
        i += 1;
    }

    if found == 0 {
        ret = -1;
        unsafe {
            err(
                c"Invalid port %s > maxports %d".as_ptr(),
                port,
                (*vhci_driver).nports,
            );
        }
        unsafe {
            usbip_vhci_driver_close();
        }
        return ret;
    }

    /* remove the port state file */
    unsafe {
        snprintf(
            path.as_mut_ptr(),
            PATH_MAX,
            c"/var/run/vhci_hcd/port%d".as_ptr(),
            portnum as c_int,
        );

        remove(path.as_ptr());
        rmdir(c"/var/run/vhci_hcd".as_ptr());
    }

    ret = unsafe { usbip_vhci_detach_device(portnum) };
    if ret < 0 {
        ret = -1;
        unsafe {
            err(c"Port %d detach request failed!\n".as_ptr(), portnum as c_int);
            usbip_vhci_driver_close();
        }
        return ret;
    }
    unsafe {
        info(c"Port %d is now detached!\n".as_ptr(), portnum as c_int);
    }

    unsafe {
        usbip_vhci_driver_close();
    }

    ret
}

unsafe fn goto_call_driver_close(ret: &mut c_int) {
    unsafe {
        usbip_vhci_driver_close();
    }

    let _ = ret;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn usbip_detach(argc: c_int, argv: *mut *mut c_char) -> c_int {
    static OPTS: [option; 2] = [
        option {
            name: c"port".as_ptr(),
            has_arg: 1,
            flag: core::ptr::null_mut(),
            val: b'p' as c_int,
        },
        option {
            name: core::ptr::null(),
            has_arg: 0,
            flag: core::ptr::null_mut(),
            val: 0,
        },
    ];
    let mut opt: c_int;
    let mut ret: c_int = -1;

    loop {
        opt = unsafe {
            getopt_long(
                argc,
                argv,
                c"p:".as_ptr(),
                OPTS.as_ptr(),
                core::ptr::null_mut(),
            )
        };

        if opt == -1 {
            break;
        }

        match opt {
            x if x == b'p' as c_int => {
                ret = unsafe { detach_port(optarg) };
                return ret;
            }
            _ => {
                unsafe {
                    usbip_detach_usage();
                }
                return ret;
            }
        }
    }

    unsafe {
        usbip_detach_usage();
    }
    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
