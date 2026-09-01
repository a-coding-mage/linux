// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2011 matt mooney <mfm@muteddisk.com>
 *               2005-2007 Takahiro Hirofuchi
 * Copyright (C) 2015-2016 Samsung Electronics
 *               Igor Kotrasinski <i.kotrasinsk@samsung.com>
 *               Krzysztof Opasiak <k.opasiak@samsung.com>
 */

use std::mem;
use std::os::raw::{c_char, c_int, c_long, c_uint, c_void};
use std::ptr;

#[repr(C)]
pub struct udev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct udev_enumerate {
    _private: [u8; 0],
}

#[repr(C)]
pub struct udev_list_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct udev_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

#[repr(C)]
pub struct op_devlist_reply {
    pub ndev: u32,
}

#[repr(C)]
pub struct usbip_usb_device {
    pub path: [c_char; 256],
    pub busid: [c_char; 32],
    pub busnum: u32,
    pub devnum: u32,
    pub speed: u32,
    pub idVendor: u16,
    pub idProduct: u16,
    pub bcdDevice: u16,
    pub bDeviceClass: u8,
    pub bDeviceSubClass: u8,
    pub bDeviceProtocol: u8,
    pub bConfigurationValue: u8,
    pub bNumConfigurations: u8,
    pub bNumInterfaces: u8,
}

#[repr(C)]
pub struct usbip_usb_interface {
    pub bInterfaceClass: u8,
    pub bInterfaceSubClass: u8,
    pub bInterfaceProtocol: u8,
    pub padding: u8,
}

#[repr(C, packed)]
pub struct usb_device_descriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bcdUSB: u16,
    pub bDeviceClass: u8,
    pub bDeviceSubClass: u8,
    pub bDeviceProtocol: u8,
    pub bMaxPacketSize0: u8,
    pub idVendor: u16,
    pub idProduct: u16,
    pub bcdDevice: u16,
    pub iManufacturer: u8,
    pub iProduct: u8,
    pub iSerialNumber: u8,
    pub bNumConfigurations: u8,
}

const OP_REP_DEVLIST: u16 = 0x8005;
const OP_REQ_DEVLIST: u16 = 0x8005;
const NO_ARGUMENT: c_int = 0;
const REQUIRED_ARGUMENT: c_int = 1;

const USBIP_LIST_USAGE_STRING: &[u8] = b"usbip list [-p|--parsable] <args>\n    -p, --parsable         Parsable list format\n    -r, --remote=<host>    List the exportable USB devices on <host>\n    -l, --local            List the local USB devices\n    -d, --device           List the local USB gadgets bound to usbip-vudc\n\0";

extern "C" {
    static mut errno: c_int;
    static mut optarg: *mut c_char;
    static usbip_port_string: *const c_char;

    static USBIP_VHCI_DRV_NAME: *const c_char;
    static USBIP_DEVICE_DRV_NAME: *const c_char;
    static VUDC_DEVICE_DESCR_FILE: *const c_char;
    static USBIDS_FILE: *const c_char;

    fn printf(fmt: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn gai_strerror(errcode: c_int) -> *const c_char;
    fn close(fd: c_int) -> c_int;
    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;

    fn udev_new() -> *mut udev;
    fn udev_unref(udev: *mut udev) -> *mut udev;
    fn udev_enumerate_new(udev: *mut udev) -> *mut udev_enumerate;
    fn udev_enumerate_unref(udev_enumerate: *mut udev_enumerate) -> *mut udev_enumerate;
    fn udev_enumerate_add_match_subsystem(
        udev_enumerate: *mut udev_enumerate,
        subsystem: *const c_char,
    ) -> c_int;
    fn udev_enumerate_add_nomatch_sysattr(
        udev_enumerate: *mut udev_enumerate,
        sysattr: *const c_char,
        value: *const c_char,
    ) -> c_int;
    fn udev_enumerate_scan_devices(udev_enumerate: *mut udev_enumerate) -> c_int;
    fn udev_enumerate_get_list_entry(
        udev_enumerate: *mut udev_enumerate,
    ) -> *mut udev_list_entry;
    fn udev_list_entry_get_next(list_entry: *mut udev_list_entry) -> *mut udev_list_entry;
    fn udev_list_entry_get_name(list_entry: *mut udev_list_entry) -> *const c_char;
    fn udev_device_new_from_syspath(
        udev: *mut udev,
        syspath: *const c_char,
    ) -> *mut udev_device;
    fn udev_device_unref(udev_device: *mut udev_device) -> *mut udev_device;
    fn udev_device_get_devpath(udev_device: *mut udev_device) -> *const c_char;
    fn udev_device_get_sysattr_value(
        udev_device: *mut udev_device,
        sysattr: *const c_char,
    ) -> *const c_char;
    fn udev_device_get_sysname(udev_device: *mut udev_device) -> *const c_char;
    fn udev_device_get_driver(udev_device: *mut udev_device) -> *const c_char;

    fn usbip_net_send_op_common(sockfd: c_int, code: u32, status: c_int) -> c_int;
    fn usbip_net_recv_op_common(sockfd: c_int, code: *mut u16, status: *mut c_int) -> c_int;
    fn usbip_net_recv(sockfd: c_int, buff: *mut c_void, bufflen: usize) -> c_int;
    fn usbip_net_pack_usb_device(pack: c_int, udev: *mut usbip_usb_device);
    fn usbip_net_pack_usb_interface(pack: c_int, uintf: *mut usbip_usb_interface);
    fn usbip_net_tcp_connect(host: *mut c_char, service: *const c_char) -> c_int;

    fn usbip_names_init(f: *const c_char) -> c_int;
    fn usbip_names_free();
    fn usbip_names_get_product(
        buff: *mut c_char,
        size: usize,
        vendor: u32,
        product: u32,
    );
    fn usbip_names_get_class(
        buff: *mut c_char,
        size: usize,
        class: u8,
        subclass: u8,
        protocol: u8,
    );

    fn usbip_op_common_status_string(status: c_int) -> *const c_char;
    fn dbg(fmt: *const c_char, ...);
    fn err(fmt: *const c_char, ...);
    fn info(fmt: *const c_char, ...);
    /* External macro supplied by usbip_network.h in the original C source. */
    fn PACK_OP_DEVLIST_REPLY(pack: c_int, reply: *mut op_devlist_reply);
}

unsafe fn le16toh(x: u16) -> u16 {
    u16::from_le(x)
}

#[no_mangle]
pub unsafe extern "C" fn usbip_list_usage() {
    printf(
        b"usage: %s\0".as_ptr() as *const c_char,
        USBIP_LIST_USAGE_STRING.as_ptr() as *const c_char,
    );
}

unsafe fn get_exported_devices(host: *mut c_char, sockfd: c_int) -> c_int {
    let mut product_name = [0 as c_char; 100];
    let mut class_name = [0 as c_char; 100];
    let mut reply: op_devlist_reply = mem::zeroed();
    let mut code: u16 = OP_REP_DEVLIST;
    let mut udev_dev: usbip_usb_device = mem::zeroed();
    let mut uintf: usbip_usb_interface = mem::zeroed();
    let mut i: c_uint;
    let mut rc: c_int;
    let mut j: c_int;
    let mut status: c_int = 0;

    rc = usbip_net_send_op_common(sockfd, OP_REQ_DEVLIST as u32, 0);
    if rc < 0 {
        dbg(b"usbip_net_send_op_common failed\0".as_ptr() as *const c_char);
        return -1;
    }

    rc = usbip_net_recv_op_common(sockfd, &mut code, &mut status);
    if rc < 0 {
        err(
            b"Exported Device List Request failed - %s\n\0".as_ptr() as *const c_char,
            usbip_op_common_status_string(status),
        );
        return -1;
    }

    memset(
        &mut reply as *mut _ as *mut c_void,
        0,
        mem::size_of_val(&reply),
    );
    rc = usbip_net_recv(
        sockfd,
        &mut reply as *mut _ as *mut c_void,
        mem::size_of_val(&reply),
    );
    if rc < 0 {
        dbg(b"usbip_net_recv_op_devlist failed\0".as_ptr() as *const c_char);
        return -1;
    }
    PACK_OP_DEVLIST_REPLY(0, &mut reply);
    dbg(
        b"exportable devices: %d\n\0".as_ptr() as *const c_char,
        reply.ndev as c_int,
    );

    if reply.ndev == 0 {
        info(
            b"no exportable devices found on %s\0".as_ptr() as *const c_char,
            host,
        );
        return 0;
    }

    printf(b"Exportable USB devices\n\0".as_ptr() as *const c_char);
    printf(b"======================\n\0".as_ptr() as *const c_char);
    printf(b" - %s\n\0".as_ptr() as *const c_char, host);

    i = 0;
    while i < reply.ndev {
        memset(
            &mut udev_dev as *mut _ as *mut c_void,
            0,
            mem::size_of_val(&udev_dev),
        );
        rc = usbip_net_recv(
            sockfd,
            &mut udev_dev as *mut _ as *mut c_void,
            mem::size_of_val(&udev_dev),
        );
        if rc < 0 {
            dbg(
                b"usbip_net_recv failed: usbip_usb_device[%d]\0".as_ptr() as *const c_char,
                i as c_int,
            );
            return -1;
        }
        usbip_net_pack_usb_device(0, &mut udev_dev);

        usbip_names_get_product(
            product_name.as_mut_ptr(),
            product_name.len(),
            udev_dev.idVendor as u32,
            udev_dev.idProduct as u32,
        );
        usbip_names_get_class(
            class_name.as_mut_ptr(),
            class_name.len(),
            udev_dev.bDeviceClass,
            udev_dev.bDeviceSubClass,
            udev_dev.bDeviceProtocol,
        );
        printf(
            b"%11s: %s\n\0".as_ptr() as *const c_char,
            udev_dev.busid.as_ptr(),
            product_name.as_ptr(),
        );
        printf(
            b"%11s: %s\n\0".as_ptr() as *const c_char,
            b"\0".as_ptr() as *const c_char,
            udev_dev.path.as_ptr(),
        );
        printf(
            b"%11s: %s\n\0".as_ptr() as *const c_char,
            b"\0".as_ptr() as *const c_char,
            class_name.as_ptr(),
        );

        j = 0;
        while j < udev_dev.bNumInterfaces as c_int {
            rc = usbip_net_recv(
                sockfd,
                &mut uintf as *mut _ as *mut c_void,
                mem::size_of_val(&uintf),
            );
            if rc < 0 {
                err(
                    b"usbip_net_recv failed: usbip_usb_intf[%d]\0".as_ptr() as *const c_char,
                    j,
                );

                return -1;
            }
            usbip_net_pack_usb_interface(0, &mut uintf);

            usbip_names_get_class(
                class_name.as_mut_ptr(),
                class_name.len(),
                uintf.bInterfaceClass,
                uintf.bInterfaceSubClass,
                uintf.bInterfaceProtocol,
            );
            printf(
                b"%11s: %2d - %s\n\0".as_ptr() as *const c_char,
                b"\0".as_ptr() as *const c_char,
                j,
                class_name.as_ptr(),
            );
            j += 1;
        }

        printf(b"\n\0".as_ptr() as *const c_char);
        i += 1;
    }

    0
}

unsafe fn list_exported_devices(host: *mut c_char) -> c_int {
    let rc: c_int;
    let sockfd: c_int;

    sockfd = usbip_net_tcp_connect(host, usbip_port_string);
    if sockfd < 0 {
        err(
            b"could not connect to %s:%s: %s\0".as_ptr() as *const c_char,
            host,
            usbip_port_string,
            gai_strerror(sockfd),
        );
        return -1;
    }
    dbg(
        b"connected to %s:%s\0".as_ptr() as *const c_char,
        host,
        usbip_port_string,
    );

    rc = get_exported_devices(host, sockfd);
    if rc < 0 {
        err(
            b"failed to get device list from %s\0".as_ptr() as *const c_char,
            host,
        );
        return -1;
    }

    close(sockfd);

    0
}

unsafe fn print_device(
    busid: *const c_char,
    vendor: *const c_char,
    product: *const c_char,
    parsable: bool,
) {
    if parsable {
        printf(
            b"busid=%s#usbid=%.4s:%.4s#\0".as_ptr() as *const c_char,
            busid,
            vendor,
            product,
        );
    } else {
        printf(
            b" - busid %s (%.4s:%.4s)\n\0".as_ptr() as *const c_char,
            busid,
            vendor,
            product,
        );
    }
}

unsafe fn print_product_name(product_name: *mut c_char, parsable: bool) {
    if !parsable {
        printf(b"   %s\n\0".as_ptr() as *const c_char, product_name);
    }
}

unsafe fn list_devices(parsable: bool) -> c_int {
    let mut udev_ctx: *mut udev;
    let mut enumerate: *mut udev_enumerate;
    let mut devices: *mut udev_list_entry;
    let mut dev_list_entry: *mut udev_list_entry;
    let mut dev: *mut udev_device;
    let mut path: *const c_char;
    let mut idVendor: *const c_char;
    let mut idProduct: *const c_char;
    let mut bConfValue: *const c_char;
    let mut bNumIntfs: *const c_char;
    let mut busid: *const c_char;
    let mut product_name = [0 as c_char; 128];
    let mut ret: c_int = -1;
    let mut devpath: *const c_char;

    /* Create libudev context. */
    udev_ctx = udev_new();

    /* Create libudev device enumeration. */
    enumerate = udev_enumerate_new(udev_ctx);

    /* Take only USB devices that are not hubs and do not have
     * the bInterfaceNumber attribute, i.e. are not interfaces.
     */
    udev_enumerate_add_match_subsystem(enumerate, b"usb\0".as_ptr() as *const c_char);
    udev_enumerate_add_nomatch_sysattr(
        enumerate,
        b"bDeviceClass\0".as_ptr() as *const c_char,
        b"09\0".as_ptr() as *const c_char,
    );
    udev_enumerate_add_nomatch_sysattr(
        enumerate,
        b"bInterfaceNumber\0".as_ptr() as *const c_char,
        ptr::null(),
    );
    udev_enumerate_scan_devices(enumerate);

    devices = udev_enumerate_get_list_entry(enumerate);

    /* Show information about each device. */
    dev_list_entry = devices;
    while !dev_list_entry.is_null() {
        path = udev_list_entry_get_name(dev_list_entry);
        dev = udev_device_new_from_syspath(udev_ctx, path);

        /* Ignore devices attached to vhci_hcd */
        devpath = udev_device_get_devpath(dev);
        if !strstr(devpath, USBIP_VHCI_DRV_NAME).is_null() {
            dbg(
                b"Skip the device %s already attached to %s\n\0".as_ptr() as *const c_char,
                devpath,
                USBIP_VHCI_DRV_NAME,
            );
            dev_list_entry = udev_list_entry_get_next(dev_list_entry);
            continue;
        }

        /* Get device information. */
        idVendor = udev_device_get_sysattr_value(dev, b"idVendor\0".as_ptr() as *const c_char);
        idProduct = udev_device_get_sysattr_value(dev, b"idProduct\0".as_ptr() as *const c_char);
        bConfValue = udev_device_get_sysattr_value(
            dev,
            b"bConfigurationValue\0".as_ptr() as *const c_char,
        );
        bNumIntfs =
            udev_device_get_sysattr_value(dev, b"bNumInterfaces\0".as_ptr() as *const c_char);
        busid = udev_device_get_sysname(dev);
        if idVendor.is_null() || idProduct.is_null() || bConfValue.is_null() || bNumIntfs.is_null()
        {
            err(
                b"problem getting device attributes: %s\0".as_ptr() as *const c_char,
                strerror(errno),
            );
            goto_err_out_list_devices(enumerate, udev_ctx);
            return ret;
        }

        /* Get product name. */
        usbip_names_get_product(
            product_name.as_mut_ptr(),
            product_name.len(),
            strtol(idVendor, ptr::null_mut(), 16) as u32,
            strtol(idProduct, ptr::null_mut(), 16) as u32,
        );

        /* Print information. */
        print_device(busid, idVendor, idProduct, parsable);
        print_product_name(product_name.as_mut_ptr(), parsable);

        printf(b"\n\0".as_ptr() as *const c_char);

        udev_device_unref(dev);
        dev_list_entry = udev_list_entry_get_next(dev_list_entry);
    }

    ret = 0;

    goto_err_out_list_devices(enumerate, udev_ctx);

    ret
}

unsafe fn goto_err_out_list_devices(enumerate: *mut udev_enumerate, udev_ctx: *mut udev) {
    udev_enumerate_unref(enumerate);
    udev_unref(udev_ctx);
}

unsafe fn list_gadget_devices(parsable: bool) -> c_int {
    let mut ret: c_int = -1;
    let mut udev_ctx: *mut udev;
    let mut enumerate: *mut udev_enumerate;
    let mut devices: *mut udev_list_entry;
    let mut dev_list_entry: *mut udev_list_entry;
    let mut dev: *mut udev_device;
    let mut path: *const c_char;
    let mut driver: *const c_char;

    let mut d_desc: *const usb_device_descriptor;
    let mut descriptors: *const c_char;
    let mut product_name = [0 as c_char; 128];

    let mut idVendor: u16;
    let mut idVendor_buf = [0 as c_char; 8];
    let mut idProduct: u16;
    let mut idProduct_buf = [0 as c_char; 8];
    let mut busid: *const c_char;

    udev_ctx = udev_new();
    enumerate = udev_enumerate_new(udev_ctx);

    udev_enumerate_add_match_subsystem(enumerate, b"platform\0".as_ptr() as *const c_char);

    udev_enumerate_scan_devices(enumerate);
    devices = udev_enumerate_get_list_entry(enumerate);

    dev_list_entry = devices;
    while !dev_list_entry.is_null() {
        path = udev_list_entry_get_name(dev_list_entry);
        dev = udev_device_new_from_syspath(udev_ctx, path);

        driver = udev_device_get_driver(dev);
        /* We only have mechanism to enumerate gadgets bound to vudc */
        if driver.is_null() || strcmp(driver, USBIP_DEVICE_DRV_NAME) != 0 {
            dev_list_entry = udev_list_entry_get_next(dev_list_entry);
            continue;
        }

        /* Get device information. */
        descriptors = udev_device_get_sysattr_value(dev, VUDC_DEVICE_DESCR_FILE);

        if descriptors.is_null() {
            err(
                b"problem getting device attributes: %s\0".as_ptr() as *const c_char,
                strerror(errno),
            );
            goto_err_out_list_gadget_devices(enumerate, udev_ctx);
            return ret;
        }

        d_desc = descriptors as *const usb_device_descriptor;

        idVendor = le16toh(ptr::addr_of!((*d_desc).idVendor).read_unaligned());
        sprintf(
            idVendor_buf.as_mut_ptr(),
            b"0x%4x\0".as_ptr() as *const c_char,
            idVendor as c_int,
        );
        idProduct = le16toh(ptr::addr_of!((*d_desc).idProduct).read_unaligned());
        sprintf(
            idProduct_buf.as_mut_ptr(),
            b"0x%4x\0".as_ptr() as *const c_char,
            idVendor as c_int,
        );
        busid = udev_device_get_sysname(dev);

        /* Get product name. */
        usbip_names_get_product(
            product_name.as_mut_ptr(),
            product_name.len(),
            le16toh(idVendor) as u32,
            le16toh(idProduct) as u32,
        );

        /* Print information. */
        print_device(
            busid,
            idVendor_buf.as_ptr(),
            idProduct_buf.as_ptr(),
            parsable,
        );
        print_product_name(product_name.as_mut_ptr(), parsable);

        printf(b"\n\0".as_ptr() as *const c_char);

        udev_device_unref(dev);
        dev_list_entry = udev_list_entry_get_next(dev_list_entry);
    }
    ret = 0;

    goto_err_out_list_gadget_devices(enumerate, udev_ctx);

    ret
}

unsafe fn goto_err_out_list_gadget_devices(enumerate: *mut udev_enumerate, udev_ctx: *mut udev) {
    udev_enumerate_unref(enumerate);
    udev_unref(udev_ctx);
}

#[no_mangle]
pub unsafe extern "C" fn usbip_list(argc: c_int, argv: *mut *mut c_char) -> c_int {
    static mut OPTS: [option; 5] = [
        option {
            name: b"parsable\0".as_ptr() as *const c_char,
            has_arg: NO_ARGUMENT,
            flag: ptr::null_mut(),
            val: 'p' as c_int,
        },
        option {
            name: b"remote\0".as_ptr() as *const c_char,
            has_arg: REQUIRED_ARGUMENT,
            flag: ptr::null_mut(),
            val: 'r' as c_int,
        },
        option {
            name: b"local\0".as_ptr() as *const c_char,
            has_arg: NO_ARGUMENT,
            flag: ptr::null_mut(),
            val: 'l' as c_int,
        },
        option {
            name: b"device\0".as_ptr() as *const c_char,
            has_arg: NO_ARGUMENT,
            flag: ptr::null_mut(),
            val: 'd' as c_int,
        },
        option {
            name: ptr::null(),
            has_arg: 0,
            flag: ptr::null_mut(),
            val: 0,
        },
    ];

    let mut parsable = false;
    let mut opt: c_int;
    let mut ret: c_int = -1;

    if usbip_names_init(USBIDS_FILE) != 0 {
        err(b"failed to open %s\0".as_ptr() as *const c_char, USBIDS_FILE);
    }

    loop {
        opt = getopt_long(
            argc,
            argv,
            b"pr:ld\0".as_ptr() as *const c_char,
            OPTS.as_ptr(),
            ptr::null_mut(),
        );

        if opt == -1 {
            break;
        }

        match opt {
            x if x == 'p' as c_int => {
                parsable = true;
            }
            x if x == 'r' as c_int => {
                ret = list_exported_devices(optarg);
                usbip_names_free();
                return ret;
            }
            x if x == 'l' as c_int => {
                ret = list_devices(parsable);
                usbip_names_free();
                return ret;
            }
            x if x == 'd' as c_int => {
                ret = list_gadget_devices(parsable);
                usbip_names_free();
                return ret;
            }
            _ => {
                usbip_list_usage();
                usbip_names_free();
                return ret;
            }
        }
    }

    usbip_list_usage();
    usbip_names_free();

    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
