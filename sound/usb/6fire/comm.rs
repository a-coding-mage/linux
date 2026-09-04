// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Linux driver for TerraTec DMX 6Fire USB
 *
 * Device communications
 *
 * Author:	Torsten Schenk <torsten.schenk@zoho.com>
 * Created:	Jan 01, 2011
 * Copyright:	(C) Torsten Schenk
 */

// Equivalent to #include "comm.h", "chip.h", "midi.h"

const COMM_EP: u8 = 1;
const COMM_FPGA_EP: u8 = 2;

// External types from kernel and headers
#[repr(C)]
pub struct urb {
    // URB structure fields - kernel defined
}

#[repr(C)]
pub struct usb_device {
    // USB device structure - kernel defined
}

#[repr(C)]
pub struct comm_runtime {
    // comm_runtime structure - defined in comm.h
}

#[repr(C)]
pub struct sfire_chip {
    // sfire_chip structure - defined in chip.h
}

#[repr(C)]
pub struct midi_runtime {
    // midi_runtime structure - defined in midi.h
}

// External kernel functions
extern "C" {
    fn usb_init_urb(urb: *mut urb);
    fn usb_sndintpipe(dev: *mut usb_device, ep: u8) -> core::ffi::c_uint;
    fn usb_rcvintpipe(dev: *mut usb_device, ep: u8) -> core::ffi::c_uint;
    fn usb_interrupt_msg(
        usb_dev: *mut usb_device,
        pipe: core::ffi::c_uint,
        data: *mut core::ffi::c_void,
        len: i32,
        actual_length: *mut i32,
        timeout: i32,
    ) -> i32;
    fn usb_submit_urb(urb: *mut urb, mem_flags: i32) -> i32;
    fn usb_poison_urb(urb: *mut urb);
    fn kmalloc(size: usize, flags: i32) -> *mut core::ffi::c_void;
    fn kfree(objp: *mut core::ffi::c_void);
    fn kzalloc(size: usize, flags: i32) -> *mut core::ffi::c_void;
    fn dev_warn(dev: *mut core::ffi::c_void, format: *const i8, ...);
    fn dev_err(dev: *mut core::ffi::c_void, format: *const i8, ...);
}

const GFP_KERNEL: i32 = 0x0000d0;
const GFP_ATOMIC: i32 = 0x000020;
const EIO: i32 = -5;
const ENOMEM: i32 = -12;
const COMM_RECEIVER_BUFSIZE: usize = 64; // Typical size, defined in comm.h

unsafe fn usb6fire_comm_init_urb(
    rt: *mut comm_runtime,
    urb: *mut urb,
    buffer: *mut u8,
    context: *mut core::ffi::c_void,
    handler: unsafe extern "C" fn(*mut urb),
) {
    usb_init_urb(urb);
    (*urb).transfer_buffer = buffer as *mut core::ffi::c_void;
    (*urb).pipe = usb_sndintpipe((*(*rt).chip).dev, COMM_EP);
    (*urb).complete = handler;
    (*urb).context = context;
    (*urb).interval = 1;
    (*urb).dev = (*(*rt).chip).dev;
}

unsafe extern "C" fn usb6fire_comm_receiver_handler(urb: *mut urb) {
    let rt = (*urb).context as *mut comm_runtime;
    let midi_rt = (*(*rt).chip).midi;

    if (*urb).status == 0 {
        let receiver_buffer = (*rt).receiver_buffer as *mut u8;
        let len = *receiver_buffer.add(1);

        if *receiver_buffer == 0x10
            && len <= (COMM_RECEIVER_BUFSIZE - 2) as u8
            && (*urb).actual_length >= (len as i32 + 2)
        {
            if !midi_rt.is_null() {
                (*midi_rt).in_received(
                    midi_rt,
                    receiver_buffer.add(2),
                    len,
                );
            }
        }
    }

    if !(*(*rt).chip).shutdown {
        (*urb).status = 0;
        (*urb).actual_length = 0;
        if usb_submit_urb(urb, GFP_ATOMIC) < 0 {
            dev_warn(
                &mut (*(*urb).dev).dev as *mut _ as *mut core::ffi::c_void,
                b"comm data receiver aborted.\n" as *const u8 as *const i8,
            );
        }
    }
}

unsafe fn usb6fire_comm_init_buffer(
    buffer: *mut u8,
    id: u8,
    request: u8,
    reg: u8,
    vl: u8,
    vh: u8,
) {
    *buffer = 0x01;
    *buffer.add(2) = request;
    *buffer.add(3) = id;
    match request {
        0x02 => {
            *buffer.add(1) = 0x05;
            *buffer.add(4) = reg;
            *buffer.add(5) = vl;
            *buffer.add(6) = vh;
        }
        0x12 => {
            *buffer.add(1) = 0x0b;
            *buffer.add(4) = 0x00;
            *buffer.add(5) = 0x18;
            *buffer.add(6) = 0x05;
            *buffer.add(7) = 0x00;
            *buffer.add(8) = 0x01;
            *buffer.add(9) = 0x00;
            *buffer.add(10) = 0x9e;
            *buffer.add(11) = reg;
            *buffer.add(12) = vl;
        }
        0x20 | 0x21 | 0x22 => {
            *buffer.add(1) = 0x04;
            *buffer.add(4) = reg;
            *buffer.add(5) = vl;
        }
        _ => {}
    }
}

unsafe fn usb6fire_comm_send_buffer(
    buffer: *mut u8,
    dev: *mut usb_device,
) -> i32 {
    let mut actual_len: i32 = 0;

    let ret = usb_interrupt_msg(
        dev,
        usb_sndintpipe(dev, COMM_EP),
        buffer as *mut core::ffi::c_void,
        *buffer.add(1) as i32 + 2,
        &mut actual_len,
        1000,
    );
    if ret < 0 {
        ret
    } else if actual_len != *buffer.add(1) as i32 + 2 {
        EIO
    } else {
        0
    }
}

unsafe fn usb6fire_comm_write8(
    rt: *mut comm_runtime,
    request: u8,
    reg: u8,
    value: u8,
) -> i32 {
    let buffer = kmalloc(13, GFP_KERNEL) as *mut u8;
    if buffer.is_null() {
        return ENOMEM;
    }

    usb6fire_comm_init_buffer(buffer, 0x00, request, reg, value, 0x00);
    let ret = usb6fire_comm_send_buffer(buffer, (*rt).chip as *mut usb_device);

    kfree(buffer as *mut core::ffi::c_void);
    ret
}

unsafe fn usb6fire_comm_write16(
    rt: *mut comm_runtime,
    request: u8,
    reg: u8,
    vl: u8,
    vh: u8,
) -> i32 {
    let buffer = kmalloc(13, GFP_KERNEL) as *mut u8;
    if buffer.is_null() {
        return ENOMEM;
    }

    usb6fire_comm_init_buffer(buffer, 0x00, request, reg, vl, vh);
    let ret = usb6fire_comm_send_buffer(buffer, (*rt).chip as *mut usb_device);

    kfree(buffer as *mut core::ffi::c_void);
    ret
}

pub unsafe fn usb6fire_comm_init(chip: *mut sfire_chip) -> i32 {
    let rt = kzalloc(core::mem::size_of::<comm_runtime>(), GFP_KERNEL)
        as *mut comm_runtime;
    if rt.is_null() {
        return ENOMEM;
    }

    (*rt).receiver_buffer = kmalloc(COMM_RECEIVER_BUFSIZE, GFP_KERNEL) as *mut u8;
    if (*rt).receiver_buffer.is_null() {
        kfree(rt as *mut core::ffi::c_void);
        return ENOMEM;
    }

    let urb = &mut (*rt).receiver;
    (*rt).serial = 1;
    (*rt).chip = chip;
    usb_init_urb(urb);
    (*rt).init_urb = usb6fire_comm_init_urb;
    (*rt).write8 = usb6fire_comm_write8;
    (*rt).write16 = usb6fire_comm_write16;

    urb.transfer_buffer = (*rt).receiver_buffer as *mut core::ffi::c_void;
    urb.transfer_buffer_length = COMM_RECEIVER_BUFSIZE as i32;
    urb.pipe = usb_rcvintpipe((*chip).dev, COMM_EP);
    urb.dev = (*chip).dev;
    urb.complete = usb6fire_comm_receiver_handler;
    urb.context = rt as *mut core::ffi::c_void;
    urb.interval = 1;
    let ret = usb_submit_urb(urb, GFP_KERNEL);
    if ret < 0 {
        kfree((*rt).receiver_buffer as *mut core::ffi::c_void);
        kfree(rt as *mut core::ffi::c_void);
        dev_err(
            &mut (*(*chip).dev).dev as *mut _ as *mut core::ffi::c_void,
            b"cannot create comm data receiver." as *const u8 as *const i8,
        );
        return ret;
    }
    (*chip).comm = rt;
    0
}

pub unsafe fn usb6fire_comm_abort(chip: *mut sfire_chip) {
    let rt = (*chip).comm;

    if !rt.is_null() {
        usb_poison_urb(&mut (*rt).receiver);
    }
}

pub unsafe fn usb6fire_comm_destroy(chip: *mut sfire_chip) {
    let rt = (*chip).comm;

    kfree((*rt).receiver_buffer as *mut core::ffi::c_void);
    kfree(rt as *mut core::ffi::c_void);
    (*chip).comm = core::ptr::null_mut();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
