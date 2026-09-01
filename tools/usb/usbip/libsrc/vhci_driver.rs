// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2005-2007 Takahiro Hirofuchi
 */

// Translated from vhci_driver.c.  C header dependencies:
// usbip_common.h, vhci_driver.h, limits.h, netdb.h, libudev.h, dirent.h,
// sysfs_utils.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type uint8_t = u8;
type uint32_t = u32;

const PROGNAME: &[u8] = b"libusbip\0";

const MAX_STATUS_NAME: usize = 18;

// Constants supplied by the original headers.
const SYSFS_BUS_ID_SIZE: usize = 32;
const SYSFS_PATH_MAX: usize = 256;
const PATH_MAX: usize = 4096;
const NI_MAXHOST: usize = 1025;
const NI_MAXSERV: usize = 32;

const HUB_SPEED_HIGH: c_int = 0;
const HUB_SPEED_SUPER: c_int = 1;

const VDEV_ST_NULL: c_int = 0;
const VDEV_ST_NOTASSIGNED: c_int = 1;

const USB_SPEED_SUPER: uint32_t = 5;
const USB_SPEED_SUPER_PLUS: uint32_t = 6;

const USBIP_VHCI_BUS_TYPE: *const c_char = b"platform\0".as_ptr() as *const c_char;
const USBIP_VHCI_DEVICE_NAME: *const c_char = b"vhci_hcd.0\0".as_ptr() as *const c_char;
const VHCI_STATE_PATH: &[u8] = b"/var/run/vhci_hcd\0";

#[repr(C)]
pub struct udev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct udev_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dirent {
    pub d_name: [c_char; 256],
}

#[repr(C)]
pub struct usbip_usb_device {
    pub busid: [c_char; SYSFS_BUS_ID_SIZE],
    pub speed: c_int,
    pub idVendor: u16,
    pub idProduct: u16,
}

#[repr(C)]
pub struct usbip_imported_device {
    pub udev: usbip_usb_device,
    pub hub: c_int,
    pub port: c_int,
    pub status: c_int,
    pub devid: c_int,
    pub busnum: c_int,
    pub devnum: c_int,
}

#[repr(C)]
pub struct usbip_vhci_driver {
    pub nports: c_int,
    pub ncontrollers: c_int,
    pub hc_device: *mut udev_device,
    pub idev: [usbip_imported_device; 0],
}

pub static mut vhci_driver: *mut usbip_vhci_driver = ptr::null_mut();
pub static mut udev_context: *mut udev = ptr::null_mut();

unsafe extern "C" {
    fn udev_new() -> *mut udev;
    fn udev_unref(udev: *mut udev) -> *mut udev;
    fn udev_device_new_from_subsystem_sysname(
        udev: *mut udev,
        subsystem: *const c_char,
        sysname: *const c_char,
    ) -> *mut udev_device;
    fn udev_device_unref(udev_device: *mut udev_device) -> *mut udev_device;
    fn udev_device_get_sysattr_value(
        udev_device: *mut udev_device,
        sysattr: *const c_char,
    ) -> *const c_char;
    fn udev_device_get_parent(udev_device: *mut udev_device) -> *mut udev_device;
    fn udev_device_get_syspath(udev_device: *mut udev_device) -> *const c_char;

    fn read_usb_device(sudev: *mut udev_device, udev: *mut usbip_usb_device);
    fn write_sysfs_attribute(path: *const c_char, new_value: *const c_char, len: size_t) -> c_int;
    fn usbip_status_string(status: c_int) -> *const c_char;
    fn usbip_speed_string(speed: c_int) -> *const c_char;
    fn usbip_names_get_product(
        product_name: *mut c_char,
        product_name_size: size_t,
        vendor: u16,
        product: u16,
    );

    fn dbg(fmt: *const c_char, ...);
    fn err(fmt: *const c_char, ...);
    fn BUG();

    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn fclose(stream: *mut FILE) -> c_int;
    fn scandir(
        dirp: *const c_char,
        namelist: *mut *mut *mut dirent,
        filter: Option<unsafe extern "C" fn(*const dirent) -> c_int>,
        compar: Option<unsafe extern "C" fn(*const *const dirent, *const *const dirent) -> c_int>,
    ) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
}

unsafe fn idev_at(port: c_int) -> *mut usbip_imported_device {
    (*vhci_driver).idev.as_mut_ptr().add(port as usize)
}

unsafe fn imported_device_init(
    idev: *mut usbip_imported_device,
    busid: *mut c_char,
) -> *mut usbip_imported_device {
    let mut sudev: *mut udev_device;

    sudev = udev_device_new_from_subsystem_sysname(udev_context, b"usb\0".as_ptr() as *const c_char, busid);
    if sudev.is_null() {
        dbg(
            b"udev_device_new_from_subsystem_sysname failed: %s\0".as_ptr() as *const c_char,
            busid,
        );
        return ptr::null_mut();
    }
    read_usb_device(sudev, &mut (*idev).udev);
    udev_device_unref(sudev);

    idev
}

unsafe fn parse_status(value: *const c_char) -> c_int {
    let mut ret: c_int = 0;
    let mut c: *mut c_char;

    /* skip a header line */
    c = strchr(value, '\n' as c_int);
    if c.is_null() {
        return -1;
    }
    c = c.add(1);

    while *c != 0 {
        let mut port: c_int = 0;
        let mut status: c_int = 0;
        let mut speed: c_int = 0;
        let mut devid: c_int = 0;
        let mut sockfd: c_int = 0;
        let mut lbusid: [c_char; SYSFS_BUS_ID_SIZE] = [0; SYSFS_BUS_ID_SIZE];
        let mut idev: *mut usbip_imported_device;
        let mut hub: [c_char; 3] = [0; 3];

        ret = sscanf(
            c,
            b"%2s  %d %d %d %x %u %31s\n\0".as_ptr() as *const c_char,
            hub.as_mut_ptr(),
            &mut port,
            &mut status,
            &mut speed,
            &mut devid,
            &mut sockfd,
            lbusid.as_mut_ptr(),
        );

        if ret < 5 {
            dbg(b"sscanf failed: %d\0".as_ptr() as *const c_char, ret);
            BUG();
        }

        dbg(
            b"hub %s port %d status %d speed %d devid %x\0".as_ptr() as *const c_char,
            hub.as_mut_ptr(),
            port,
            status,
            speed,
            devid,
        );
        dbg(
            b"sockfd %u lbusid %s\0".as_ptr() as *const c_char,
            sockfd,
            lbusid.as_mut_ptr(),
        );

        /* if a device is connected, look at it */
        idev = idev_at(port);
        memset(idev as *mut c_void, 0, size_of::<usbip_imported_device>());

        if strncmp(b"hs\0".as_ptr() as *const c_char, hub.as_ptr(), 2) == 0 {
            (*idev).hub = HUB_SPEED_HIGH;
        } else {
            /* strncmp("ss", hub, 2) == 0 */
            (*idev).hub = HUB_SPEED_SUPER;
        }

        (*idev).port = port;
        (*idev).status = status;

        (*idev).devid = devid;

        (*idev).busnum = devid >> 16;
        (*idev).devnum = devid & 0x0000ffff;

        if (*idev).status != VDEV_ST_NULL && (*idev).status != VDEV_ST_NOTASSIGNED {
            idev = imported_device_init(idev, lbusid.as_mut_ptr());
            if idev.is_null() {
                dbg(b"imported_device_init failed\0".as_ptr() as *const c_char);
                return -1;
            }
        }

        /* go to the next line */
        c = strchr(c, '\n' as c_int);
        if c.is_null() {
            break;
        }
        c = c.add(1);
    }

    dbg(b"exit\0".as_ptr() as *const c_char);

    0
}

unsafe fn refresh_imported_device_list() -> c_int {
    let mut attr_status: *const c_char;
    let mut status: [c_char; MAX_STATUS_NAME + 1] = [0; MAX_STATUS_NAME + 1];
    let mut i: c_int;
    let mut ret: c_int;

    ptr::copy_nonoverlapping(b"status\0".as_ptr() as *const c_char, status.as_mut_ptr(), 7);

    i = 0;
    while i < (*vhci_driver).ncontrollers {
        if i > 0 {
            snprintf(
                status.as_mut_ptr(),
                status.len(),
                b"status.%d\0".as_ptr() as *const c_char,
                i,
            );
        }

        attr_status = udev_device_get_sysattr_value((*vhci_driver).hc_device, status.as_ptr());
        if attr_status.is_null() {
            err(b"udev_device_get_sysattr_value failed\0".as_ptr() as *const c_char);
            return -1;
        }

        dbg(b"controller %d\0".as_ptr() as *const c_char, i);

        ret = parse_status(attr_status);
        if ret != 0 {
            return ret;
        }
        i += 1;
    }

    0
}

unsafe fn get_nports(hc_device: *mut udev_device) -> c_int {
    let attr_nports: *const c_char;

    attr_nports = udev_device_get_sysattr_value(hc_device, b"nports\0".as_ptr() as *const c_char);
    if attr_nports.is_null() {
        err(b"udev_device_get_sysattr_value nports failed\0".as_ptr() as *const c_char);
        return -1;
    }

    strtoul(attr_nports, ptr::null_mut(), 10) as c_int
}

unsafe extern "C" fn vhci_hcd_filter(dirent: *const dirent) -> c_int {
    (strncmp((*dirent).d_name.as_ptr(), b"vhci_hcd.\0".as_ptr() as *const c_char, 9) == 0) as c_int
}

unsafe fn get_ncontrollers() -> c_int {
    let mut namelist: *mut *mut dirent = ptr::null_mut();
    let platform: *mut udev_device;
    let n: c_int;

    platform = udev_device_get_parent((*vhci_driver).hc_device);
    if platform.is_null() {
        return -1;
    }

    n = scandir(
        udev_device_get_syspath(platform),
        &mut namelist,
        Some(vhci_hcd_filter),
        None,
    );
    if n < 0 {
        err(b"scandir failed\0".as_ptr() as *const c_char);
    } else {
        let mut i: c_int = 0;
        while i < n {
            free(*namelist.add(i as usize) as *mut c_void);
            i += 1;
        }
        free(namelist as *mut c_void);
    }

    n
}

/*
 * Read the given port's record.
 *
 * To avoid buffer overflow we will read the entire line and
 * validate each part's size. The initial buffer is padded by 4 to
 * accommodate the 2 spaces, 1 newline and an additional character
 * which is needed to properly validate the 3rd part without it being
 * truncated to an acceptable length.
 */
unsafe fn read_record(
    rhport: c_int,
    host: *mut c_char,
    host_len: c_ulong,
    port: *mut c_char,
    port_len: c_ulong,
    busid: *mut c_char,
) -> c_int {
    let mut part: c_int;
    let mut file: *mut FILE;
    let mut path: [c_char; PATH_MAX + 1] = [0; PATH_MAX + 1];
    let mut buffer: *mut c_char;
    let mut start: *mut c_char;
    let mut end: *mut c_char;
    let delim: [c_char; 3] = [' ' as c_char, ' ' as c_char, '\n' as c_char];
    let max_len: [c_int; 3] = [host_len as c_int, port_len as c_int, SYSFS_BUS_ID_SIZE as c_int];
    let buffer_len: size_t = host_len as size_t + port_len as size_t + SYSFS_BUS_ID_SIZE + 4;

    buffer = malloc(buffer_len) as *mut c_char;
    if buffer.is_null() {
        return -1;
    }

    snprintf(
        path.as_mut_ptr(),
        PATH_MAX,
        b"/var/run/vhci_hcd/port%d\0".as_ptr() as *const c_char,
        rhport,
    );

    file = fopen(path.as_ptr(), b"r\0".as_ptr() as *const c_char);
    if file.is_null() {
        err(b"fopen\0".as_ptr() as *const c_char);
        free(buffer as *mut c_void);
        return -1;
    }

    if fgets(buffer, buffer_len as c_int, file).is_null() {
        err(b"fgets\0".as_ptr() as *const c_char);
        free(buffer as *mut c_void);
        fclose(file);
        return -1;
    }
    fclose(file);

    /* validate the length of each of the 3 parts */
    start = buffer;
    part = 0;
    while part < 3 {
        end = strchr(start, delim[part as usize] as c_int);
        if end.is_null() || end.offset_from(start) > max_len[part as usize] as isize {
            free(buffer as *mut c_void);
            return -1;
        }
        start = end.add(1);
        part += 1;
    }

    if sscanf(
        buffer,
        b"%s %s %s\n\0".as_ptr() as *const c_char,
        host,
        port,
        busid,
    ) != 3
    {
        err(b"sscanf\0".as_ptr() as *const c_char);
        free(buffer as *mut c_void);
        return -1;
    }

    free(buffer as *mut c_void);

    0
}

/* ---------------------------------------------------------------------- */

#[no_mangle]
pub unsafe extern "C" fn usbip_vhci_driver_open() -> c_int {
    let mut nports: c_int;
    let mut hc_device: *mut udev_device;

    udev_context = udev_new();
    if udev_context.is_null() {
        err(b"udev_new failed\0".as_ptr() as *const c_char);
        return -1;
    }

    /* will be freed in usbip_driver_close() */
    hc_device = udev_device_new_from_subsystem_sysname(
        udev_context,
        USBIP_VHCI_BUS_TYPE,
        USBIP_VHCI_DEVICE_NAME,
    );
    if hc_device.is_null() {
        err(b"udev_device_new_from_subsystem_sysname failed\0".as_ptr() as *const c_char);
        udev_unref(udev_context);
        return -1;
    }

    nports = get_nports(hc_device);
    if nports <= 0 {
        err(b"no available ports\0".as_ptr() as *const c_char);
        udev_device_unref(hc_device);
        udev_unref(udev_context);
        return -1;
    }
    dbg(b"available ports: %d\0".as_ptr() as *const c_char, nports);

    vhci_driver = calloc(
        1,
        size_of::<usbip_vhci_driver>() + nports as usize * size_of::<usbip_imported_device>(),
    ) as *mut usbip_vhci_driver;
    if vhci_driver.is_null() {
        err(b"vhci_driver allocation failed\0".as_ptr() as *const c_char);
        udev_device_unref(hc_device);
        udev_unref(udev_context);
        return -1;
    }

    (*vhci_driver).nports = nports;
    (*vhci_driver).hc_device = hc_device;
    (*vhci_driver).ncontrollers = get_ncontrollers();
    dbg(
        b"available controllers: %d\0".as_ptr() as *const c_char,
        (*vhci_driver).ncontrollers,
    );

    if (*vhci_driver).ncontrollers <= 0 {
        err(b"no available usb controllers\0".as_ptr() as *const c_char);
        udev_device_unref(hc_device);
        free(vhci_driver as *mut c_void);
        vhci_driver = ptr::null_mut();
        udev_unref(udev_context);
        return -1;
    }

    if refresh_imported_device_list() != 0 {
        udev_device_unref(hc_device);
        if !vhci_driver.is_null() {
            free(vhci_driver as *mut c_void);
        }
        vhci_driver = ptr::null_mut();
        udev_unref(udev_context);
        return -1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn usbip_vhci_driver_close() {
    if vhci_driver.is_null() {
        return;
    }

    udev_device_unref((*vhci_driver).hc_device);

    free(vhci_driver as *mut c_void);

    vhci_driver = ptr::null_mut();

    udev_unref(udev_context);
}

#[no_mangle]
pub unsafe extern "C" fn usbip_vhci_refresh_device_list() -> c_int {
    if refresh_imported_device_list() != 0 {
        dbg(b"failed to refresh device list\0".as_ptr() as *const c_char);
        return -1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn usbip_vhci_get_free_port(speed: uint32_t) -> c_int {
    let mut i: c_int = 0;
    while i < (*vhci_driver).nports {
        match speed {
            USB_SPEED_SUPER | USB_SPEED_SUPER_PLUS => {
                if (*idev_at(i)).hub != HUB_SPEED_SUPER {
                    i += 1;
                    continue;
                }
            }
            _ => {
                if (*idev_at(i)).hub != HUB_SPEED_HIGH {
                    i += 1;
                    continue;
                }
            }
        }

        if (*idev_at(i)).status == VDEV_ST_NULL {
            return (*idev_at(i)).port;
        }
        i += 1;
    }

    -1
}

#[no_mangle]
pub unsafe extern "C" fn usbip_vhci_attach_device2(
    port: uint8_t,
    sockfd: c_int,
    devid: uint32_t,
    speed: uint32_t,
) -> c_int {
    let mut buff: [c_char; 200] = [0; 200]; /* what size should be ? */
    let mut attach_attr_path: [c_char; SYSFS_PATH_MAX] = [0; SYSFS_PATH_MAX];
    let attr_attach: [c_char; 7] = [b'a' as c_char, b't' as c_char, b't' as c_char, b'a' as c_char, b'c' as c_char, b'h' as c_char, 0];
    let path: *const c_char;
    let mut ret: c_int;

    snprintf(
        buff.as_mut_ptr(),
        buff.len(),
        b"%u %d %u %u\0".as_ptr() as *const c_char,
        port as c_uint,
        sockfd,
        devid,
        speed,
    );
    dbg(b"writing: %s\0".as_ptr() as *const c_char, buff.as_mut_ptr());

    path = udev_device_get_syspath((*vhci_driver).hc_device);
    snprintf(
        attach_attr_path.as_mut_ptr(),
        attach_attr_path.len(),
        b"%s/%s\0".as_ptr() as *const c_char,
        path,
        attr_attach.as_ptr(),
    );
    dbg(
        b"attach attribute path: %s\0".as_ptr() as *const c_char,
        attach_attr_path.as_mut_ptr(),
    );

    ret = write_sysfs_attribute(attach_attr_path.as_ptr(), buff.as_ptr(), strlen(buff.as_ptr()));
    if ret < 0 {
        dbg(b"write_sysfs_attribute failed\0".as_ptr() as *const c_char);
        return -1;
    }

    dbg(b"attached port: %d\0".as_ptr() as *const c_char, port as c_int);

    0
}

unsafe fn get_devid(busnum: uint8_t, devnum: uint8_t) -> c_ulong {
    (((busnum as c_ulong) << 16) | devnum as c_ulong) as c_ulong
}

/* will be removed */
#[no_mangle]
pub unsafe extern "C" fn usbip_vhci_attach_device(
    port: uint8_t,
    sockfd: c_int,
    busnum: uint8_t,
    devnum: uint8_t,
    speed: uint32_t,
) -> c_int {
    let devid: c_int = get_devid(busnum, devnum) as c_int;

    usbip_vhci_attach_device2(port, sockfd, devid as uint32_t, speed)
}

#[no_mangle]
pub unsafe extern "C" fn usbip_vhci_detach_device(port: uint8_t) -> c_int {
    let mut detach_attr_path: [c_char; SYSFS_PATH_MAX] = [0; SYSFS_PATH_MAX];
    let attr_detach: [c_char; 7] = [b'd' as c_char, b'e' as c_char, b't' as c_char, b'a' as c_char, b'c' as c_char, b'h' as c_char, 0];
    let mut buff: [c_char; 200] = [0; 200]; /* what size should be ? */
    let path: *const c_char;
    let mut ret: c_int;

    snprintf(
        buff.as_mut_ptr(),
        buff.len(),
        b"%u\0".as_ptr() as *const c_char,
        port as c_uint,
    );
    dbg(b"writing: %s\0".as_ptr() as *const c_char, buff.as_mut_ptr());

    path = udev_device_get_syspath((*vhci_driver).hc_device);
    snprintf(
        detach_attr_path.as_mut_ptr(),
        detach_attr_path.len(),
        b"%s/%s\0".as_ptr() as *const c_char,
        path,
        attr_detach.as_ptr(),
    );
    dbg(
        b"detach attribute path: %s\0".as_ptr() as *const c_char,
        detach_attr_path.as_mut_ptr(),
    );

    ret = write_sysfs_attribute(detach_attr_path.as_ptr(), buff.as_ptr(), strlen(buff.as_ptr()));
    if ret < 0 {
        dbg(b"write_sysfs_attribute failed\0".as_ptr() as *const c_char);
        return -1;
    }

    dbg(b"detached port: %d\0".as_ptr() as *const c_char, port as c_int);

    0
}

#[no_mangle]
pub unsafe extern "C" fn usbip_vhci_imported_device_dump(
    idev: *mut usbip_imported_device,
) -> c_int {
    let mut product_name: [c_char; 100] = [0; 100];
    let mut host: [c_char; NI_MAXHOST] = [0; NI_MAXHOST];
    let mut serv: [c_char; NI_MAXSERV] = [0; NI_MAXSERV];
    let mut remote_busid: [c_char; SYSFS_BUS_ID_SIZE] = [0; SYSFS_BUS_ID_SIZE];
    let mut ret: c_int;
    let mut read_record_error: c_int = 0;

    ptr::copy_nonoverlapping(b"unknown host\0".as_ptr() as *const c_char, host.as_mut_ptr(), 13);
    ptr::copy_nonoverlapping(b"unknown port\0".as_ptr() as *const c_char, serv.as_mut_ptr(), 13);

    if (*idev).status == VDEV_ST_NULL || (*idev).status == VDEV_ST_NOTASSIGNED {
        return 0;
    }

    ret = read_record(
        (*idev).port,
        host.as_mut_ptr(),
        host.len() as c_ulong,
        serv.as_mut_ptr(),
        serv.len() as c_ulong,
        remote_busid.as_mut_ptr(),
    );
    if ret != 0 {
        err(b"read_record\0".as_ptr() as *const c_char);
        read_record_error = 1;
    }

    printf(
        b"Port %02d: <%s> at %s\n\0".as_ptr() as *const c_char,
        (*idev).port,
        usbip_status_string((*idev).status),
        usbip_speed_string((*idev).udev.speed),
    );

    usbip_names_get_product(
        product_name.as_mut_ptr(),
        product_name.len(),
        (*idev).udev.idVendor,
        (*idev).udev.idProduct,
    );

    printf(b"       %s\n\0".as_ptr() as *const c_char, product_name.as_mut_ptr());

    if read_record_error == 0 {
        printf(
            b"%10s -> usbip://%s:%s/%s\n\0".as_ptr() as *const c_char,
            (*idev).udev.busid.as_ptr(),
            host.as_mut_ptr(),
            serv.as_mut_ptr(),
            remote_busid.as_mut_ptr(),
        );
        printf(
            b"%10s -> remote bus/dev %03d/%03d\n\0".as_ptr() as *const c_char,
            b" \0".as_ptr() as *const c_char,
            (*idev).busnum,
            (*idev).devnum,
        );
    } else {
        printf(
            b"%10s -> unknown host, remote port and remote busid\n\0".as_ptr() as *const c_char,
            (*idev).udev.busid.as_ptr(),
        );
        printf(
            b"%10s -> remote bus/dev %03d/%03d\n\0".as_ptr() as *const c_char,
            b" \0".as_ptr() as *const c_char,
            (*idev).busnum,
            (*idev).devnum,
        );
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
