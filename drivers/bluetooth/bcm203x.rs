// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Broadcom Blutonium firmware driver
 *
 * Copyright (C) 2003  Maxim Krasnyansky <maxk@qualcomm.com>
 * Copyright (C) 2003  Marcel Holtmann <marcel@holtmann.org>
 */

// Linux kernel dependencies supplied by other translation units.

const VERSION: &str = "1.2";

static BCM203X_TABLE: [usb_device_id; 2] = [
    usb_device_id { match_flags: 0, idVendor: 0x0a5c, idProduct: 0x2033 },
    usb_device_id { match_flags: 0, idVendor: 0, idProduct: 0 },
];

const BCM203X_ERROR: c_ulong = 0;
const BCM203X_RESET: c_ulong = 1;
const BCM203X_LOAD_MINIDRV: c_ulong = 2;
const BCM203X_SELECT_MEMORY: c_ulong = 3;
const BCM203X_CHECK_MEMORY: c_ulong = 4;
const BCM203X_LOAD_FIRMWARE: c_ulong = 5;
const BCM203X_CHECK_FIRMWARE: c_ulong = 6;

const BCM203X_IN_EP: u8 = 0x81;
const BCM203X_OUT_EP: u8 = 0x02;

#[repr(C)]
struct bcm203x_data {
    udev: *mut usb_device,
    state: c_ulong,
    work: work_struct,
    shutdown: atomic_t,
    urb: *mut urb,
    buffer: *mut u8,
    fw_data: *mut u8,
    fw_size: c_uint,
    fw_sent: c_uint,
}

unsafe extern "C" fn bcm203x_complete(urb: *mut urb) {
    let data = (*urb).context as *mut bcm203x_data;
    let udev = (*urb).dev;
    let mut len: c_int;

    BT_DBG!("udev %p urb %p", udev, urb);

    if (*urb).status != 0 {
        BT_ERR!("URB failed with status %d", (*urb).status);
        (*data).state = BCM203X_ERROR;
        return;
    }

    match (*data).state {
        BCM203X_LOAD_MINIDRV => {
            core::ptr::write((*data).buffer, b'#');
            usb_fill_bulk_urb(urb, udev, usb_sndbulkpipe(udev, BCM203X_OUT_EP),
                (*data).buffer as *mut _, 1, Some(bcm203x_complete), data as *mut _);
            (*data).state = BCM203X_SELECT_MEMORY;
            schedule_work(&mut (*data).work);
        }
        BCM203X_SELECT_MEMORY => {
            usb_fill_int_urb(urb, udev, usb_rcvintpipe(udev, BCM203X_IN_EP),
                (*data).buffer as *mut _, 32, Some(bcm203x_complete), data as *mut _, 1);
            (*data).state = BCM203X_CHECK_MEMORY;
            if usb_submit_urb((*data).urb, GFP_ATOMIC) < 0 { BT_ERR!("Can't submit URB"); }
        }
        BCM203X_CHECK_MEMORY => {
            if *(*data).buffer != b'#' {
                BT_ERR!("Memory select failed");
                (*data).state = BCM203X_ERROR;
            } else {
                (*data).state = BCM203X_LOAD_FIRMWARE;
                if (*data).fw_sent == (*data).fw_size {
                    usb_fill_int_urb(urb, udev, usb_rcvintpipe(udev, BCM203X_IN_EP),
                        (*data).buffer as *mut _, 32, Some(bcm203x_complete), data as *mut _, 1);
                    (*data).state = BCM203X_CHECK_FIRMWARE;
                } else {
                    len = core::cmp::min((*data).fw_size - (*data).fw_sent, 4096) as c_int;
                    usb_fill_bulk_urb(urb, udev, usb_sndbulkpipe(udev, BCM203X_OUT_EP),
                        (*data).fw_data.add((*data).fw_sent as usize) as *mut _, len,
                        Some(bcm203x_complete), data as *mut _);
                    (*data).fw_sent += len as c_uint;
                }
                if usb_submit_urb((*data).urb, GFP_ATOMIC) < 0 { BT_ERR!("Can't submit URB"); }
            }
        }
        BCM203X_LOAD_FIRMWARE => {
            if (*data).fw_sent == (*data).fw_size {
                usb_fill_int_urb(urb, udev, usb_rcvintpipe(udev, BCM203X_IN_EP),
                    (*data).buffer as *mut _, 32, Some(bcm203x_complete), data as *mut _, 1);
                (*data).state = BCM203X_CHECK_FIRMWARE;
            } else {
                len = core::cmp::min((*data).fw_size - (*data).fw_sent, 4096) as c_int;
                usb_fill_bulk_urb(urb, udev, usb_sndbulkpipe(udev, BCM203X_OUT_EP),
                    (*data).fw_data.add((*data).fw_sent as usize) as *mut _, len,
                    Some(bcm203x_complete), data as *mut _);
                (*data).fw_sent += len as c_uint;
            }
            if usb_submit_urb((*data).urb, GFP_ATOMIC) < 0 { BT_ERR!("Can't submit URB"); }
        }
        BCM203X_CHECK_FIRMWARE => {
            if *(*data).buffer != b'.' {
                BT_ERR!("Firmware loading failed");
                (*data).state = BCM203X_ERROR;
            } else { (*data).state = BCM203X_RESET; }
        }
        _ => {}
    }
}

unsafe extern "C" fn bcm203x_work(work: *mut work_struct) {
    let data = container_of!(work, bcm203x_data, work);
    if atomic_read(&(*data).shutdown) != 0 { return; }
    if usb_submit_urb((*data).urb, GFP_KERNEL) < 0 { BT_ERR!("Can't submit URB"); }
}

unsafe extern "C" fn bcm203x_probe(intf: *mut usb_interface, id: *const usb_device_id) -> c_int {
    let udev = interface_to_usbdev(intf);
    let data = devm_kzalloc(&mut (*intf).dev, core::mem::size_of::<bcm203x_data>(), GFP_KERNEL) as *mut bcm203x_data;
    BT_DBG!("intf %p id %p", intf, id);
    if (*(*intf).cur_altsetting).desc.bInterfaceNumber != 0 { return -ENODEV; }
    if data.is_null() { return -ENOMEM; }
    (*data).udev = udev; (*data).state = BCM203X_LOAD_MINIDRV;
    (*data).urb = usb_alloc_urb(0, GFP_KERNEL);
    if (*data).urb.is_null() { return -ENOMEM; }
    let mut firmware: *const firmware = core::ptr::null();
    if request_firmware(&mut firmware, c"BCM2033-MD.hex".as_ptr(), &mut (*udev).dev) < 0 {
        BT_ERR!("Mini driver request failed"); usb_free_urb((*data).urb); return -EIO;
    }
    BT_DBG!("minidrv data %p size %zu", (*firmware).data, (*firmware).size);
    let size = core::cmp::max((*firmware).size, 4096);
    (*data).buffer = kmalloc(size, GFP_KERNEL) as *mut u8;
    if (*data).buffer.is_null() {
        BT_ERR!("Can't allocate memory for mini driver"); release_firmware(firmware);
        usb_free_urb((*data).urb); return -ENOMEM;
    }
    core::ptr::copy_nonoverlapping((*firmware).data, (*data).buffer, (*firmware).size);
    usb_fill_bulk_urb((*data).urb, udev, usb_sndbulkpipe(udev, BCM203X_OUT_EP),
        (*data).buffer as *mut _, (*firmware).size as c_int, Some(bcm203x_complete), data as *mut _);
    release_firmware(firmware);
    if request_firmware(&mut firmware, c"BCM2033-FW.bin".as_ptr(), &mut (*udev).dev) < 0 {
        BT_ERR!("Firmware request failed"); usb_free_urb((*data).urb); kfree((*data).buffer as *mut _); return -EIO;
    }
    BT_DBG!("firmware data %p size %zu", (*firmware).data, (*firmware).size);
    (*data).fw_data = kmemdup((*firmware).data, (*firmware).size, GFP_KERNEL) as *mut u8;
    if (*data).fw_data.is_null() {
        BT_ERR!("Can't allocate memory for firmware image"); release_firmware(firmware);
        usb_free_urb((*data).urb); kfree((*data).buffer as *mut _); return -ENOMEM;
    }
    (*data).fw_size = (*firmware).size as c_uint; (*data).fw_sent = 0;
    release_firmware(firmware);
    INIT_WORK(&mut (*data).work, bcm203x_work);
    usb_set_intfdata(intf, data as *mut _);
    schedule_work(&mut (*data).work);
    0
}

unsafe extern "C" fn bcm203x_disconnect(intf: *mut usb_interface) {
    let data = usb_get_intfdata(intf) as *mut bcm203x_data;
    BT_DBG!("intf %p", intf);
    atomic_inc(&mut (*data).shutdown);
    cancel_work_sync(&mut (*data).work);
    usb_kill_urb((*data).urb);
    usb_set_intfdata(intf, core::ptr::null_mut());
    usb_free_urb((*data).urb);
    kfree((*data).fw_data as *mut _);
    kfree((*data).buffer as *mut _);
}

// module_usb_driver(bcm203x_driver), module metadata, and USB table registration are
// supplied by the translated kernel module infrastructure.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
