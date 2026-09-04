// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (c) 2006,2007 Daniel Mack
*/

// #include <linux/device.h>
// #include <linux/usb.h>
// #include <linux/gfp.h>
// #include <sound/rawmidi.h>
// #include <sound/core.h>
// #include <sound/pcm.h>
// #include "device.h"
// #include "midi.h"

use core::ffi::c_int;

extern "C" {
    type snd_rawmidi_substream;
    type snd_usb_caiaqdev;
    type device;
    type snd_rawmidi;
    type urb;

    fn caiaqdev_to_dev(cdev: *mut snd_usb_caiaqdev) -> *mut device;
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn usb_submit_urb(urb: *mut urb, mem_flags: u32) -> c_int;
    fn usb_kill_urb(urb: *mut urb);
    fn snd_rawmidi_transmit(substream: *mut snd_rawmidi_substream, buf: *mut u8, count: c_int) -> c_int;
    fn snd_rawmidi_receive(substream: *mut snd_rawmidi_substream, buf: *const u8, count: c_int);
    fn snd_rawmidi_new(
        card: *mut core::ffi::c_void,
        id: *const u8,
        device: u32,
        output_count: u32,
        input_count: u32,
        rrawmidi: *mut *mut snd_rawmidi,
    ) -> c_int;
    fn strscpy(dest: *mut u8, src: *const u8, count: usize) -> isize;
    fn snd_rawmidi_set_ops(
        rmidi: *mut snd_rawmidi,
        stream: u32,
        ops: *const snd_rawmidi_ops,
    );
}

#[repr(C)]
struct snd_rawmidi_ops {
    open: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream, c_int)>,
}

unsafe extern "C" fn snd_usb_caiaq_midi_input_open(substream: *mut snd_rawmidi_substream) -> c_int {
    0
}

unsafe extern "C" fn snd_usb_caiaq_midi_input_close(substream: *mut snd_rawmidi_substream) -> c_int {
    0
}

unsafe extern "C" fn snd_usb_caiaq_midi_input_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    let cdev = {
        let rmidi = (*substream).rmidi as *mut snd_rawmidi;
        (*rmidi).private_data as *mut snd_usb_caiaqdev
    };

    if cdev.is_null() {
        return;
    }

    (*cdev).midi_receive_substream = if up != 0 { substream } else { core::ptr::null_mut() };
}

unsafe extern "C" fn snd_usb_caiaq_midi_output_open(substream: *mut snd_rawmidi_substream) -> c_int {
    0
}

unsafe extern "C" fn snd_usb_caiaq_midi_output_close(substream: *mut snd_rawmidi_substream) -> c_int {
    let cdev = {
        let rmidi = (*substream).rmidi as *mut snd_rawmidi;
        (*rmidi).private_data as *mut snd_usb_caiaqdev
    };

    if (*cdev).midi_out_active != 0 {
        usb_kill_urb(&mut (*cdev).midi_out_urb as *mut urb);
        (*cdev).midi_out_active = 0;
    }
    0
}

unsafe extern "C" fn snd_usb_caiaq_midi_send(cdev: *mut snd_usb_caiaqdev, substream: *mut snd_rawmidi_substream) {
    let len: c_int;
    let ret: c_int;
    let dev = caiaqdev_to_dev(cdev);

    (*cdev).midi_out_buf[0] = 0x81; // EP1_CMD_MIDI_WRITE
    (*cdev).midi_out_buf[1] = 0; /* port */
    len = snd_rawmidi_transmit(substream, &mut (*cdev).midi_out_buf[3], 0x200 - 3); // EP1_BUFSIZE = 0x200

    if len <= 0 {
        return;
    }

    (*cdev).midi_out_buf[2] = len as u8;
    (*cdev).midi_out_urb.transfer_buffer_length = (len + 3) as usize;

    ret = usb_submit_urb(&mut (*cdev).midi_out_urb as *mut urb, 0x20); // GFP_ATOMIC = 0x20
    if ret < 0 {
        dev_err(
            dev,
            b"snd_usb_caiaq_midi_send(%p): usb_submit_urb() failed,ret=%d, len=%d\n\0".as_ptr(),
            substream as *mut core::ffi::c_void,
            ret,
            len,
        );
    } else {
        (*cdev).midi_out_active = 1;
    }
}

unsafe extern "C" fn snd_usb_caiaq_midi_output_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    let cdev = {
        let rmidi = (*substream).rmidi as *mut snd_rawmidi;
        (*rmidi).private_data as *mut snd_usb_caiaqdev
    };

    if up != 0 {
        (*cdev).midi_out_substream = substream;
        if (*cdev).midi_out_active == 0 {
            snd_usb_caiaq_midi_send(cdev, substream);
        }
    } else {
        (*cdev).midi_out_substream = core::ptr::null_mut();
    }
}

#[no_mangle]
static snd_usb_caiaq_midi_output: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_usb_caiaq_midi_output_open),
    close: Some(snd_usb_caiaq_midi_output_close),
    trigger: Some(snd_usb_caiaq_midi_output_trigger),
};

#[no_mangle]
static snd_usb_caiaq_midi_input: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_usb_caiaq_midi_input_open),
    close: Some(snd_usb_caiaq_midi_input_close),
    trigger: Some(snd_usb_caiaq_midi_input_trigger),
};

#[no_mangle]
pub unsafe extern "C" fn snd_usb_caiaq_midi_handle_input(
    cdev: *mut snd_usb_caiaqdev,
    port: c_int,
    buf: *const u8,
    len: c_int,
) {
    if (*cdev).midi_receive_substream.is_null() {
        return;
    }

    snd_rawmidi_receive((*cdev).midi_receive_substream, buf, len);
}

#[no_mangle]
pub unsafe extern "C" fn snd_usb_caiaq_midi_init(device: *mut snd_usb_caiaqdev) -> c_int {
    let mut ret: c_int;
    let mut rmidi: *mut snd_rawmidi = core::ptr::null_mut();

    let product_name = (*device).product_name as *const u8;

    ret = snd_rawmidi_new(
        (*device).chip.card as *mut core::ffi::c_void,
        product_name,
        0,
        (*device).spec.num_midi_out,
        (*device).spec.num_midi_in,
        &mut rmidi,
    );

    if ret < 0 {
        return ret;
    }

    strscpy(
        (*rmidi).name.as_mut_ptr(),
        product_name,
        (*rmidi).name.len(),
    );

    (*rmidi).info_flags = 0x00000001; // SNDRV_RAWMIDI_INFO_DUPLEX

    (*rmidi).private_data = device as *mut core::ffi::c_void;

    if (*device).spec.num_midi_out > 0 {
        (*rmidi).info_flags |= 0x00000002; // SNDRV_RAWMIDI_INFO_OUTPUT
        snd_rawmidi_set_ops(rmidi, 0, &snd_usb_caiaq_midi_output); // SNDRV_RAWMIDI_STREAM_OUTPUT = 0
    }

    if (*device).spec.num_midi_in > 0 {
        (*rmidi).info_flags |= 0x00000004; // SNDRV_RAWMIDI_INFO_INPUT
        snd_rawmidi_set_ops(rmidi, 1, &snd_usb_caiaq_midi_input); // SNDRV_RAWMIDI_STREAM_INPUT = 1
    }

    (*device).rmidi = rmidi;

    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_usb_caiaq_midi_output_done(urb_ptr: *mut urb) {
    let cdev = (*urb_ptr).context as *mut snd_usb_caiaqdev;

    (*cdev).midi_out_active = 0;
    if (*urb_ptr).status != 0 {
        return;
    }

    if (*cdev).midi_out_substream.is_null() {
        return;
    }

    snd_usb_caiaq_midi_send(cdev, (*cdev).midi_out_substream);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
