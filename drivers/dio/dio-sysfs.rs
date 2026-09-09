/*
 *  File Attributes for DIO Devices
 *
 *  Copyright (C) 2004 Jochen Friedrich
 *
 *  Loosely based on drivers/pci/pci-sysfs.c and drivers/zorro/zorro-sysfs.c
 *
 *  This file is subject to the terms and conditions of the GNU General Public
 *  License.  See the file COPYING in the main directory of this archive
 *  for more details.
 */

/* Dependencies supplied by the surrounding kernel translation are intentionally
 * left external, corresponding to the original Linux includes. */

use core::ffi::{c_char, c_int, c_ulong, c_void};

type ssize_t = isize;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dio_dev {
    pub dev: device,
    pub id: u16,
    pub ipl: u8,
    pub name: *const c_char,
}

extern "C" {
    fn to_dio_dev(dev: *mut device) -> *mut dio_dev;
    fn sprintf(buf: *mut c_char, format: *const c_char, ...) -> c_int;
    fn dio_resource_start(d: *mut dio_dev) -> c_ulong;
    fn dio_resource_end(d: *mut dio_dev) -> c_ulong;
    fn dio_resource_flags(d: *mut dio_dev) -> c_ulong;
    fn device_create_file(dev: *mut device, attr: *const device_attribute) -> c_int;

    static dev_attr_id: device_attribute;
    static dev_attr_ipl: device_attribute;
    static dev_attr_secid: device_attribute;
    static dev_attr_name: device_attribute;
    static dev_attr_resource: device_attribute;
}

/* show configuration fields */

unsafe extern "C" fn dio_show_id(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let d: *mut dio_dev;

    d = to_dio_dev(dev);
    sprintf(buf, b"0x%02x\0".as_ptr() as *const c_char, (*d).id & 0xff) as ssize_t
}

unsafe extern "C" fn dio_show_ipl(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let d: *mut dio_dev;

    d = to_dio_dev(dev);
    sprintf(buf, b"0x%02x\0".as_ptr() as *const c_char, (*d).ipl) as ssize_t
}

unsafe extern "C" fn dio_show_secid(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let d: *mut dio_dev;

    d = to_dio_dev(dev);
    sprintf(buf, b"0x%02x\0".as_ptr() as *const c_char, ((*d).id >> 8) & 0xff) as ssize_t
}

unsafe extern "C" fn dio_show_name(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let d: *mut dio_dev;

    d = to_dio_dev(dev);
    sprintf(buf, b"%s\n\0".as_ptr() as *const c_char, (*d).name) as ssize_t
}

unsafe extern "C" fn dio_show_resource(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let d: *mut dio_dev = to_dio_dev(dev);

    sprintf(
        buf,
        b"0x%08lx 0x%08lx 0x%08lx\n\0".as_ptr() as *const c_char,
        dio_resource_start(d),
        dio_resource_end(d),
        dio_resource_flags(d),
    ) as ssize_t
}

/* DEVICE_ATTR(id, S_IRUGO, dio_show_id, NULL); */
/* DEVICE_ATTR(ipl, S_IRUGO, dio_show_ipl, NULL); */
/* DEVICE_ATTR(secid, S_IRUGO, dio_show_secid, NULL); */
/* DEVICE_ATTR(name, S_IRUGO, dio_show_name, NULL); */
/* DEVICE_ATTR(resource, S_IRUGO, dio_show_resource, NULL); */

pub unsafe extern "C" fn dio_create_sysfs_dev_files(d: *mut dio_dev) -> c_int {
    let dev: *mut device = &mut (*d).dev;
    let mut error: c_int;

    /* current configuration's attributes */
    error = device_create_file(dev, &dev_attr_id);
    if error != 0 {
        return error;
    }
    error = device_create_file(dev, &dev_attr_ipl);
    if error != 0 {
        return error;
    }
    error = device_create_file(dev, &dev_attr_secid);
    if error != 0 {
        return error;
    }
    error = device_create_file(dev, &dev_attr_name);
    if error != 0 {
        return error;
    }
    error = device_create_file(dev, &dev_attr_resource);
    if error != 0 {
        return error;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
