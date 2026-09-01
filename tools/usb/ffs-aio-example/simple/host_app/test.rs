/*
 * This is free and unencumbered software released into the public domain.
 *
 * Anyone is free to copy, modify, publish, use, compile, sell, or
 * distribute this software, either in source code form or as a compiled
 * binary, for any purpose, commercial or non-commercial, and by any
 * means.
 *
 * In jurisdictions that recognize copyright laws, the author or authors
 * of this software dedicate any and all copyright interest in the
 * software to the public domain. We make this dedication for the benefit
 * of the public at large and to the detriment of our heirs and
 * successors. We intend this dedication to be an overt act of
 * relinquishment in perpetuity of all present and future rights to this
 * software under copyright law.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 * IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * For more information, please refer to <http://unlicense.org/>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::ffi::{c_char, c_int, c_uchar, c_uint, c_void};
use std::ptr;

// Dependencies from C includes: <libusb.h>, <stdio.h>, <string.h>, <unistd.h>

const VENDOR: u16 = 0x1d6b;
const PRODUCT: u16 = 0x0105;

const BUF_LEN: usize = 8192;

#[repr(C)]
pub struct libusb_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct libusb_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct libusb_device_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct libusb_device_descriptor {
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

#[repr(C)]
pub struct libusb_endpoint_descriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bEndpointAddress: u8,
    pub bmAttributes: u8,
    pub wMaxPacketSize: u16,
    pub bInterval: u8,
    pub bRefresh: u8,
    pub bSynchAddress: u8,
    pub extra: *const c_uchar,
    pub extra_length: c_int,
}

#[repr(C)]
pub struct libusb_interface_descriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bInterfaceNumber: u8,
    pub bAlternateSetting: u8,
    pub bNumEndpoints: u8,
    pub bInterfaceClass: u8,
    pub bInterfaceSubClass: u8,
    pub bInterfaceProtocol: u8,
    pub iInterface: u8,
    pub endpoint: *const libusb_endpoint_descriptor,
    pub extra: *const c_uchar,
    pub extra_length: c_int,
}

#[repr(C)]
pub struct libusb_interface {
    pub altsetting: *const libusb_interface_descriptor,
    pub num_altsetting: c_int,
}

#[repr(C)]
pub struct libusb_config_descriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub wTotalLength: u16,
    pub bNumInterfaces: u8,
    pub bConfigurationValue: u8,
    pub iConfiguration: u8,
    pub bmAttributes: u8,
    pub MaxPower: u8,
    pub interface: *const libusb_interface,
    pub extra: *const c_uchar,
    pub extra_length: c_int,
}

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;

    fn libusb_init(ctx: *mut *mut libusb_context) -> c_int;
    fn libusb_exit(ctx: *mut libusb_context);
    fn libusb_get_device_list(
        ctx: *mut libusb_context,
        list: *mut *mut *mut libusb_device,
    ) -> isize;
    fn libusb_free_device_list(list: *mut *mut libusb_device, unref_devices: c_int);
    fn libusb_get_device_descriptor(
        dev: *mut libusb_device,
        desc: *mut libusb_device_descriptor,
    ) -> c_int;
    fn libusb_error_name(errcode: c_int) -> *const c_char;
    fn libusb_open(dev: *mut libusb_device, handle: *mut *mut libusb_device_handle) -> c_int;
    fn libusb_close(dev_handle: *mut libusb_device_handle);
    fn libusb_claim_interface(dev_handle: *mut libusb_device_handle, interface_number: c_int)
        -> c_int;
    fn libusb_release_interface(dev_handle: *mut libusb_device_handle, interface_number: c_int)
        -> c_int;
    fn libusb_detach_kernel_driver(
        dev_handle: *mut libusb_device_handle,
        interface_number: c_int,
    ) -> c_int;
    fn libusb_attach_kernel_driver(
        dev_handle: *mut libusb_device_handle,
        interface_number: c_int,
    ) -> c_int;
    fn libusb_get_config_descriptor(
        dev: *mut libusb_device,
        config_index: u8,
        config: *mut *mut libusb_config_descriptor,
    ) -> c_int;
    fn libusb_bulk_transfer(
        dev_handle: *mut libusb_device_handle,
        endpoint: c_uchar,
        data: *mut c_uchar,
        length: c_int,
        actual_length: *mut c_int,
        timeout: c_uint,
    ) -> c_int;
}

/*
 * struct test_state - describes test program state
 * @list: list of devices returned by libusb_get_device_list function
 * @found: pointer to struct describing tested device
 * @ctx: context, set to NULL
 * @handle: handle of tested device
 * @attached: indicates that device was attached to kernel, and has to be
 *            reattached at the end of test program
 */

#[repr(C)]
pub struct test_state {
    pub found: *mut libusb_device,
    pub ctx: *mut libusb_context,
    pub handle: *mut libusb_device_handle,
    pub attached: c_int,
}

/*
 * test_init - initialize test program
 */

pub unsafe extern "C" fn test_init(state: *mut test_state) -> c_int {
    let mut i: c_int;
    let mut ret: c_int;
    let cnt: isize;
    let mut list: *mut *mut libusb_device = ptr::null_mut();

    unsafe {
        (*state).found = ptr::null_mut();
        (*state).ctx = ptr::null_mut();
        (*state).handle = ptr::null_mut();
        (*state).attached = 0;

        ret = libusb_init(&mut (*state).ctx);
        if ret != 0 {
            printf(
                b"cannot init libusb: %s\n\0".as_ptr() as *const c_char,
                libusb_error_name(ret),
            );
            return 1;
        }

        cnt = libusb_get_device_list((*state).ctx, &mut list);
        if cnt <= 0 {
            printf(b"no devices found\n\0".as_ptr() as *const c_char);
            libusb_exit((*state).ctx);
            return 1;
        }

        i = 0;
        while (i as isize) < cnt {
            let dev: *mut libusb_device = *list.offset(i as isize);
            let mut desc: libusb_device_descriptor = std::mem::zeroed();
            ret = libusb_get_device_descriptor(dev, &mut desc);
            if ret != 0 {
                printf(
                    b"unable to get device descriptor: %s\n\0".as_ptr() as *const c_char,
                    libusb_error_name(ret),
                );
                libusb_free_device_list(list, 1);
                libusb_exit((*state).ctx);
                return 1;
            }
            if desc.idVendor == VENDOR && desc.idProduct == PRODUCT {
                (*state).found = dev;
                break;
            }
            i += 1;
        }

        if (*state).found.is_null() {
            printf(b"no devices found\n\0".as_ptr() as *const c_char);
            libusb_free_device_list(list, 1);
            libusb_exit((*state).ctx);
            return 1;
        }

        ret = libusb_open((*state).found, &mut (*state).handle);
        if ret != 0 {
            printf(
                b"cannot open device: %s\n\0".as_ptr() as *const c_char,
                libusb_error_name(ret),
            );
            libusb_free_device_list(list, 1);
            libusb_exit((*state).ctx);
            return 1;
        }

        if libusb_claim_interface((*state).handle, 0) != 0 {
            ret = libusb_detach_kernel_driver((*state).handle, 0);
            if ret != 0 {
                printf(
                    b"unable to detach kernel driver: %s\n\0".as_ptr() as *const c_char,
                    libusb_error_name(ret),
                );
                libusb_close((*state).handle);
                libusb_free_device_list(list, 1);
                libusb_exit((*state).ctx);
                return 1;
            }
            (*state).attached = 1;
            ret = libusb_claim_interface((*state).handle, 0);
            if ret != 0 {
                printf(
                    b"cannot claim interface: %s\n\0".as_ptr() as *const c_char,
                    libusb_error_name(ret),
                );
                if (*state).attached == 1 {
                    libusb_attach_kernel_driver((*state).handle, 0);
                }
                libusb_close((*state).handle);
                libusb_free_device_list(list, 1);
                libusb_exit((*state).ctx);
                return 1;
            }
        }

        0
    }
}

/*
 * test_exit - cleanup test program
 */

pub unsafe extern "C" fn test_exit(state: *mut test_state) {
    unsafe {
        libusb_release_interface((*state).handle, 0);
        if (*state).attached == 1 {
            libusb_attach_kernel_driver((*state).handle, 0);
        }
        libusb_close((*state).handle);
        libusb_exit((*state).ctx);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> c_int {
    unsafe {
        let mut state: test_state = std::mem::zeroed();
        let mut conf: *mut libusb_config_descriptor = ptr::null_mut();
        let iface: *const libusb_interface_descriptor;
        let in_addr: c_uchar;
        let out_addr: c_uchar;

        if test_init(&mut state) != 0 {
            return 1;
        }

        libusb_get_config_descriptor(state.found, 0, &mut conf);
        iface = (*(*(*conf).interface.offset(0)).altsetting.offset(0)) as *const _;
        in_addr = (*(*iface).endpoint.offset(0)).bEndpointAddress;
        out_addr = (*(*iface).endpoint.offset(1)).bEndpointAddress;

        loop {
            static mut buffer: [c_uchar; BUF_LEN] = [0; BUF_LEN];
            let mut bytes: c_int = 0;
            libusb_bulk_transfer(
                state.handle,
                in_addr,
                (&raw mut buffer).cast::<c_uchar>(),
                BUF_LEN as c_int,
                &mut bytes,
                500,
            );
            libusb_bulk_transfer(
                state.handle,
                out_addr,
                (&raw mut buffer).cast::<c_uchar>(),
                BUF_LEN as c_int,
                &mut bytes,
                500,
            );
        }

        #[allow(unreachable_code)]
        {
            test_exit(&mut state);
            0
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
