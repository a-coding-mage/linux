// SPDX-License-Identifier: GPL-2.0
/*
 * Generic USB GNSS receiver driver
 *
 * Copyright (C) 2021 Johan Hovold <johan@kernel.org>
 */

// Dependencies supplied by the Linux kernel and other translation units.

const GNSS_USB_READ_BUF_LEN: usize = 512;
const GNSS_USB_WRITE_TIMEOUT: i32 = 1000;

static mut GNSS_USB_ID_TABLE: [usb_device_id; 2] = [
    usb_device_id { idVendor: 0x1199, idProduct: 0xb000 }, // Sierra Wireless XM1210
    usb_device_id { idVendor: 0, idProduct: 0 },
];

#[repr(C)]
struct gnss_usb {
    udev: *mut usb_device,
    intf: *mut usb_interface,
    gdev: *mut gnss_device,
    read_urb: *mut urb,
    write_pipe: u32,
}

unsafe extern "C" {
    fn dev_dbg(dev: *mut device, fmt: *const u8, ...);
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn gnss_insert_raw(gdev: *mut gnss_device, buf: *mut core::ffi::c_void, len: i32) -> i32;
    fn usb_submit_urb(urb: *mut urb, mem_flags: i32) -> i32;
    fn gnss_get_drvdata(gdev: *mut gnss_device) -> *mut gnss_usb;
    fn usb_kill_urb(urb: *mut urb);
    fn kmemdup(buf: *const core::ffi::c_void, len: usize, flags: i32) -> *mut core::ffi::c_void;
    fn usb_bulk_msg(udev: *mut usb_device, pipe: u32, data: *mut core::ffi::c_void,
                    len: usize, actual_length: *mut i32, timeout: i32) -> i32;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn usb_find_common_endpoints(altsetting: *mut usb_host_interface,
                                 in_ep: *mut *mut usb_endpoint_descriptor,
                                 out_ep: *mut *mut usb_endpoint_descriptor,
                                 int_ep: *mut *mut usb_endpoint_descriptor,
                                 iso_ep: *mut *mut usb_endpoint_descriptor) -> i32;
    fn kzalloc_obj<T>() -> *mut T;
    fn gnss_allocate_device(parent: *mut device) -> *mut gnss_device;
    fn usb_alloc_urb(iso_packets: i32, mem_flags: i32) -> *mut urb;
    fn usb_endpoint_maxp(ep: *mut usb_endpoint_descriptor) -> usize;
    fn kzalloc(size: usize, flags: i32) -> *mut core::ffi::c_void;
    fn usb_fill_bulk_urb(urb: *mut urb, udev: *mut usb_device, pipe: u32,
                         transfer_buffer: *mut core::ffi::c_void, buffer_length: usize,
                         complete: unsafe extern "C" fn(*mut urb), context: *mut core::ffi::c_void);
    fn usb_endpoint_num(ep: *mut usb_endpoint_descriptor) -> u32;
    fn usb_rcvbulkpipe(udev: *mut usb_device, endpoint: u32) -> u32;
    fn usb_sndbulkpipe(udev: *mut usb_device, endpoint: u32) -> u32;
    fn gnss_set_drvdata(gdev: *mut gnss_device, data: *mut gnss_usb);
    fn gnss_register_device(gdev: *mut gnss_device) -> i32;
    fn usb_set_intfdata(intf: *mut usb_interface, data: *mut gnss_usb);
    fn usb_free_urb(urb: *mut urb);
    fn gnss_put_device(gdev: *mut gnss_device);
    fn gnss_deregister_device(gdev: *mut gnss_device);
    fn usb_get_intfdata(intf: *mut usb_interface) -> *mut gnss_usb;
    fn interface_to_usbdev(intf: *mut usb_interface) -> *mut usb_device;
}

unsafe extern "C" fn gnss_usb_rx_complete(urb: *mut urb) {
    let gusb = (*urb).context as *mut gnss_usb;
    let gdev = (*gusb).gdev;
    let status = (*urb).status;
    let mut ret: i32;

    match status {
        0 => {}
        -2 /* -ENOENT */ | -104 /* -ECONNRESET */ | -108 /* -ESHUTDOWN */ => {
            dev_dbg(&mut (*gdev).dev, b"urb stopped: %d\0".as_ptr(), status);
            return;
        }
        -32 /* -EPIPE */ => {
            dev_err(&mut (*gdev).dev, b"urb stopped: %d\0".as_ptr(), status);
            return;
        }
        _ => {
            dev_dbg(&mut (*gdev).dev, b"nonzero urb status: %d\0".as_ptr(), status);
            goto_resubmit(urb, gdev);
            return;
        }
    }

    let len = (*urb).actual_length;
    if len == 0 {
        goto_resubmit(urb, gdev);
        return;
    }

    ret = gnss_insert_raw(gdev, (*urb).transfer_buffer, len);
    if ret < len {
        dev_dbg(&mut (*gdev).dev, b"dropped %d bytes\0".as_ptr(), len - ret);
    }
    goto_resubmit(urb, gdev);
}

unsafe fn goto_resubmit(urb: *mut urb, gdev: *mut gnss_device) {
    let ret = usb_submit_urb(urb, GFP_ATOMIC);
    if ret != 0 && ret != -1 /* -EPERM */ && ret != -19 /* -ENODEV */ {
        dev_err(&mut (*gdev).dev, b"failed to resubmit urb: %d\0".as_ptr(), ret);
    }
}

unsafe extern "C" fn gnss_usb_open(gdev: *mut gnss_device) -> i32 {
    let gusb = gnss_get_drvdata(gdev);
    let ret = usb_submit_urb((*gusb).read_urb, GFP_KERNEL);
    if ret != 0 {
        if ret != -1 && ret != -19 {
            dev_err(&mut (*gdev).dev, b"failed to submit urb: %d\0".as_ptr(), ret);
        }
        return ret;
    }
    0
}

unsafe extern "C" fn gnss_usb_close(gdev: *mut gnss_device) {
    usb_kill_urb((*gnss_get_drvdata(gdev)).read_urb);
}

unsafe extern "C" fn gnss_usb_write_raw(gdev: *mut gnss_device, buf: *const u8, count: usize) -> i32 {
    let gusb = gnss_get_drvdata(gdev);
    let tbuf = kmemdup(buf as *const _, count, GFP_KERNEL);
    if tbuf.is_null() { return -12; /* -ENOMEM */ }
    let ret = usb_bulk_msg((*gusb).udev, (*gusb).write_pipe, tbuf, count, core::ptr::null_mut(), GNSS_USB_WRITE_TIMEOUT);
    kfree(tbuf);
    if ret != 0 { return ret; }
    count as i32
}

unsafe extern "C" fn gnss_usb_probe(intf: *mut usb_interface, _id: *const usb_device_id) -> i32 {
    let udev = interface_to_usbdev(intf);
    let mut in_ep = core::ptr::null_mut();
    let mut out_ep = core::ptr::null_mut();
    let ret = usb_find_common_endpoints((*intf).cur_altsetting, &mut in_ep, &mut out_ep,
                                        core::ptr::null_mut(), core::ptr::null_mut());
    if ret != 0 { return ret; }

    let gusb = kzalloc_obj::<gnss_usb>();
    if gusb.is_null() { return -12; /* -ENOMEM */ }

    let gdev = gnss_allocate_device(&mut (*intf).dev);
    if gdev.is_null() {
        kfree(gusb as *mut _);
        return -12;
    }
    (*gdev).ops = &GNSS_USB_GNSS_OPS as *const _;
    (*gdev).type_ = GNSS_TYPE_NMEA;
    gnss_set_drvdata(gdev, gusb);

    let urb = usb_alloc_urb(0, GFP_KERNEL);
    if urb.is_null() {
        gnss_put_device(gdev); kfree(gusb as *mut _); return -12;
    }
    let buf_len = core::cmp::max(usb_endpoint_maxp(in_ep), GNSS_USB_READ_BUF_LEN);
    let buf = kzalloc(buf_len, GFP_KERNEL);
    if buf.is_null() {
        usb_free_urb(urb); gnss_put_device(gdev); kfree(gusb as *mut _); return -12;
    }
    usb_fill_bulk_urb(urb, udev, usb_rcvbulkpipe(udev, usb_endpoint_num(in_ep)), buf,
                      buf_len, gnss_usb_rx_complete, gusb as *mut _);
    (*gusb).intf = intf;
    (*gusb).udev = udev;
    (*gusb).gdev = gdev;
    (*gusb).read_urb = urb;
    (*gusb).write_pipe = usb_sndbulkpipe(udev, usb_endpoint_num(out_ep));
    let ret = gnss_register_device(gdev);
    if ret != 0 {
        kfree(buf); usb_free_urb(urb); gnss_put_device(gdev); kfree(gusb as *mut _); return ret;
    }
    usb_set_intfdata(intf, gusb);
    0
}

unsafe extern "C" fn gnss_usb_disconnect(intf: *mut usb_interface) {
    let gusb = usb_get_intfdata(intf);
    gnss_deregister_device((*gusb).gdev);
    kfree((*(*gusb).read_urb).transfer_buffer);
    usb_free_urb((*gusb).read_urb);
    gnss_put_device((*gusb).gdev);
    kfree(gusb as *mut _);
}

#[repr(C)]
struct gnss_operations {
    open: Option<unsafe extern "C" fn(*mut gnss_device) -> i32>,
    close: Option<unsafe extern "C" fn(*mut gnss_device)>,
    write_raw: Option<unsafe extern "C" fn(*mut gnss_device, *const u8, usize) -> i32>,
}

static GNSS_USB_GNSS_OPS: gnss_operations = gnss_operations {
    open: Some(gnss_usb_open), close: Some(gnss_usb_close), write_raw: Some(gnss_usb_write_raw),
};

// Equivalent to module_usb_driver(gnss_usb_driver); module metadata is supplied by the kernel build.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
