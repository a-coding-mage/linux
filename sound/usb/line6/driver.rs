// SPDX-License-Identifier: GPL-2.0-only
/*
 * Line 6 Linux USB driver
 *
 * Copyright (C) 2004-2010 Markus Grabner (line6@grabner-graz.at)
 */

// Linux kernel headers: <linux/kernel.h>, <linux/module.h>, <linux/export.h>, <linux/slab.h>, <linux/usb.h>
// ALSA sound headers: <sound/core.h>, <sound/initval.h>, <sound/hwdep.h>
// Module headers: "capture.h", "driver.h", "midi.h", "playback.h"

const DRIVER_AUTHOR: &str = "Markus Grabner <line6@grabner-graz.at>";
const DRIVER_DESC: &str = "Line 6 USB Driver";

// This is Line 6's MIDI manufacturer ID.
pub const line6_midi_id: [u8; 3] = [0x00, 0x01, 0x0c];

// Code to request version of POD, Variax interface
// (and maybe other devices).
static line6_request_version: [i8; 6] = [0xf0, 0x7e, 0x7f, 0x06, 0x01, 0xf7];

// Class for asynchronous messages.
#[repr(C)]
struct message {
    line6: *mut usb_line6,
    buffer: *const i8,
    size: i32,
    done: i32,
}

// Forward declarations.
unsafe extern "C" fn line6_data_received(urb: *mut urb);
unsafe fn line6_send_raw_message_async_part(msg: *mut message, urb: *mut urb) -> i32;

// Start to listen on endpoint.
unsafe fn line6_start_listen(line6: *mut usb_line6) -> i32 {
    let mut err: i32;

    if (*(*line6).properties).capabilities & LINE6_CAP_CONTROL_MIDI != 0 {
        usb_fill_int_urb(
            (*line6).urb_listen,
            (*line6).usbdev,
            usb_rcvintpipe((*line6).usbdev, (*(*line6).properties).ep_ctrl_r),
            (*line6).buffer_listen as *mut u8,
            LINE6_BUFSIZE_LISTEN,
            Some(line6_data_received),
            line6 as *mut libc::c_void,
            (*line6).interval,
        );
    } else {
        usb_fill_bulk_urb(
            (*line6).urb_listen,
            (*line6).usbdev,
            usb_rcvbulkpipe((*line6).usbdev, (*(*line6).properties).ep_ctrl_r),
            (*line6).buffer_listen as *mut u8,
            LINE6_BUFSIZE_LISTEN,
            Some(line6_data_received),
            line6 as *mut libc::c_void,
        );
    }

    // sanity checks of EP before actually submitting
    if usb_urb_ep_type_check((*line6).urb_listen) != 0 {
        dev_err((*line6).ifcdev, "invalid control EP\n");
        return -libc::EINVAL;
    }

    (*(*line6).urb_listen).actual_length = 0;
    err = usb_submit_urb((*line6).urb_listen, GFP_ATOMIC);
    err
}

// Stop listening on endpoint.
unsafe fn line6_stop_listen(line6: *mut usb_line6) {
    usb_kill_urb((*line6).urb_listen);
}

// Send raw message in pieces of wMaxPacketSize bytes.
pub unsafe extern "C" fn line6_send_raw_message(line6: *mut usb_line6, buffer: *const i8, size: i32) -> i32 {
    let mut i = 0;
    let mut done = 0;
    let properties = (*line6).properties;

    while i < size {
        let mut partial: i32;
        let frag_buf = buffer.add(i as usize);
        let frag_size = std::cmp::min((*line6).max_packet_size, size - i);
        let retval: i32;

        if (*properties).capabilities & LINE6_CAP_CONTROL_MIDI != 0 {
            retval = usb_interrupt_msg(
                (*line6).usbdev,
                usb_sndintpipe((*line6).usbdev, (*properties).ep_ctrl_w),
                frag_buf as *mut libc::c_char,
                frag_size,
                &mut partial,
                LINE6_TIMEOUT,
            );
        } else {
            retval = usb_bulk_msg(
                (*line6).usbdev,
                usb_sndbulkpipe((*line6).usbdev, (*properties).ep_ctrl_w),
                frag_buf as *mut libc::c_char,
                frag_size,
                &mut partial,
                LINE6_TIMEOUT,
            );
        }

        if retval != 0 {
            dev_err((*line6).ifcdev, "usb_bulk_msg failed (%d)\n", retval);
            break;
        }

        done += frag_size;
        i += frag_size;
    }

    done
}

// Notification of completion of asynchronous request transmission.
unsafe extern "C" fn line6_async_request_sent(urb: *mut urb) {
    let msg = (*urb).context as *mut message;

    if (*msg).done >= (*msg).size {
        usb_free_urb(urb);
        kfree(msg as *mut libc::c_void);
    } else {
        let _ = line6_send_raw_message_async_part(msg, urb);
    }
}

// Asynchronously send part of a raw message.
unsafe fn line6_send_raw_message_async_part(msg: *mut message, urb: *mut urb) -> i32 {
    let mut retval: i32;
    let line6 = (*msg).line6;
    let done = (*msg).done;
    let bytes = std::cmp::min((*msg).size - done, (*line6).max_packet_size);

    if (*(*line6).properties).capabilities & LINE6_CAP_CONTROL_MIDI != 0 {
        usb_fill_int_urb(
            urb,
            (*line6).usbdev,
            usb_sndintpipe((*line6).usbdev, (*(*line6).properties).ep_ctrl_w),
            ((*msg).buffer as *mut i8).add(done as usize) as *mut u8,
            bytes,
            Some(line6_async_request_sent),
            msg as *mut libc::c_void,
            (*line6).interval,
        );
    } else {
        usb_fill_bulk_urb(
            urb,
            (*line6).usbdev,
            usb_sndbulkpipe((*line6).usbdev, (*(*line6).properties).ep_ctrl_w),
            ((*msg).buffer as *mut i8).add(done as usize) as *mut u8,
            bytes,
            Some(line6_async_request_sent),
            msg as *mut libc::c_void,
        );
    }

    (*msg).done += bytes;

    // sanity checks of EP before actually submitting
    retval = usb_urb_ep_type_check(urb);
    if retval < 0 {
        dev_err((*line6).ifcdev, "%s: usb_submit_urb failed (%d)\n", "__func__\0" as *const u8 as *const i8, retval);
        usb_free_urb(urb);
        kfree(msg as *mut libc::c_void);
        return retval;
    }

    retval = usb_submit_urb(urb, GFP_ATOMIC);
    if retval < 0 {
        dev_err((*line6).ifcdev, "%s: usb_submit_urb failed (%d)\n", "__func__\0" as *const u8 as *const i8, retval);
        usb_free_urb(urb);
        kfree(msg as *mut libc::c_void);
        return retval;
    }

    0
}

// Asynchronously send raw message.
pub unsafe extern "C" fn line6_send_raw_message_async(line6: *mut usb_line6, buffer: *const i8, size: i32) -> i32 {
    let msg: *mut message;
    let urb: *mut urb;

    // create message:
    msg = kzalloc_obj::<message>(GFP_ATOMIC) as *mut message;
    if msg.is_null() {
        return -libc::ENOMEM;
    }

    // create URB:
    urb = usb_alloc_urb(0, GFP_ATOMIC);

    if urb.is_null() {
        kfree(msg as *mut libc::c_void);
        return -libc::ENOMEM;
    }

    // set message data:
    (*msg).line6 = line6;
    (*msg).buffer = buffer;
    (*msg).size = size;
    (*msg).done = 0;

    // start sending:
    line6_send_raw_message_async_part(msg, urb)
}

// Send asynchronous device version request.
pub unsafe extern "C" fn line6_version_request_async(line6: *mut usb_line6) -> i32 {
    let buffer: *mut i8;
    let retval: i32;

    buffer = kmemdup(
        line6_request_version.as_ptr() as *const libc::c_void,
        std::mem::size_of_val(&line6_request_version),
        GFP_ATOMIC,
    ) as *mut i8;
    if buffer.is_null() {
        return -libc::ENOMEM;
    }

    retval = line6_send_raw_message_async(line6, buffer, std::mem::size_of_val(&line6_request_version) as i32);
    kfree(buffer as *mut libc::c_void);
    retval
}

// Send sysex message in pieces of wMaxPacketSize bytes.
pub unsafe extern "C" fn line6_send_sysex_message(line6: *mut usb_line6, buffer: *const i8, size: i32) -> i32 {
    line6_send_raw_message(line6, buffer, size + SYSEX_EXTRA_SIZE) - SYSEX_EXTRA_SIZE
}

// Allocate buffer for sysex message and prepare header.
// @param code sysex message code
// @param size number of bytes between code and sysex end
pub unsafe extern "C" fn line6_alloc_sysex_buffer(
    line6: *mut usb_line6,
    code1: i32,
    code2: i32,
    size: i32,
) -> *mut i8 {
    let buffer = kmalloc((size + SYSEX_EXTRA_SIZE) as usize, GFP_ATOMIC) as *mut i8;

    if buffer.is_null() {
        return std::ptr::null_mut();
    }

    *buffer = LINE6_SYSEX_BEGIN as i8;
    libc::memcpy(
        buffer.add(1) as *mut libc::c_void,
        line6_midi_id.as_ptr() as *const libc::c_void,
        std::mem::size_of_val(&line6_midi_id),
    );
    *buffer.add(std::mem::size_of_val(&line6_midi_id) + 1) = code1 as i8;
    *buffer.add(std::mem::size_of_val(&line6_midi_id) + 2) = code2 as i8;
    *buffer.add(std::mem::size_of_val(&line6_midi_id) + 3 + size as usize) = LINE6_SYSEX_END as i8;
    buffer
}

// Notification of data received from the Line 6 device.
unsafe extern "C" fn line6_data_received(urb: *mut urb) {
    let line6 = (*urb).context as *mut usb_line6;
    let mb = &mut (*(*line6).line6midi).midibuf_in;
    let mut done: i32;

    if (*urb).status == -libc::ESHUTDOWN {
        return;
    }

    if (*(*line6).properties).capabilities & LINE6_CAP_CONTROL_MIDI != 0 {
        // scoped_guard(spinlock_irqsave, &line6->line6midi->lock)
        done = line6_midibuf_write(mb, (*urb).transfer_buffer as *mut i8, (*urb).actual_length);

        if done < (*urb).actual_length {
            line6_midibuf_ignore(mb, done);
            dev_dbg(
                (*line6).ifcdev,
                "%d %d buffer overflow - message skipped\n",
                done,
                (*urb).actual_length,
            );
        }

        loop {
            // scoped_guard(spinlock_irqsave, &line6->line6midi->lock)
            done = line6_midibuf_read(
                mb,
                (*line6).buffer_message,
                LINE6_MIDI_MESSAGE_MAXLEN,
                LINE6_MIDIBUF_READ_RX,
            );

            if done <= 0 {
                break;
            }

            (*line6).message_length = done;
            line6_midi_receive(line6, (*line6).buffer_message, done);

            if !(*line6).process_message.is_none() {
                (*line6).process_message.unwrap()(line6);
            }
        }
    } else {
        (*line6).buffer_message = (*urb).transfer_buffer as *mut i8;
        (*line6).message_length = (*urb).actual_length;
        if !(*line6).process_message.is_none() {
            (*line6).process_message.unwrap()(line6);
        }
        (*line6).buffer_message = std::ptr::null_mut();
    }

    let _ = line6_start_listen(line6);
}

const LINE6_READ_WRITE_STATUS_DELAY: i32 = 2; // milliseconds
const LINE6_READ_WRITE_MAX_RETRIES: i32 = 50;

// Read data from device.
pub unsafe extern "C" fn line6_read_data(
    line6: *mut usb_line6,
    address: u32,
    data: *mut libc::c_void,
    datalen: u32,
) -> i32 {
    let usbdev = (*line6).usbdev;
    let mut ret: i32;
    let mut len: u8;
    let mut count: u32;

    if address > 0xffff || datalen > 0xff {
        return -libc::EINVAL;
    }

    // query the serial number:
    ret = usb_control_msg_send(
        usbdev,
        0,
        0x67,
        (libc::USB_TYPE_VENDOR | libc::USB_RECIP_DEVICE | libc::USB_DIR_OUT) as u8,
        ((datalen << 8) | 0x21) as u16,
        address as u16,
        std::ptr::null_mut(),
        0,
        LINE6_TIMEOUT,
        GFP_KERNEL,
    );
    if ret != 0 {
        dev_err((*line6).ifcdev, "read request failed (error %d)\n", ret);
        return ret;
    }

    // Wait for data length. We'll get 0xff until length arrives.
    count = 0;
    while count < LINE6_READ_WRITE_MAX_RETRIES as u32 {
        mdelay(LINE6_READ_WRITE_STATUS_DELAY);

        ret = usb_control_msg_recv(
            usbdev,
            0,
            0x67,
            (libc::USB_TYPE_VENDOR | libc::USB_RECIP_DEVICE | libc::USB_DIR_IN) as u8,
            0x0012,
            0x0000,
            &mut len as *mut u8 as *mut libc::c_void,
            1,
            LINE6_TIMEOUT,
            GFP_KERNEL,
        );
        if ret != 0 {
            dev_err(
                (*line6).ifcdev,
                "receive length failed (error %d)\n",
                ret,
            );
            return ret;
        }

        if len != 0xff {
            break;
        }
        count += 1;
    }

    ret = -libc::EIO;
    if len == 0xff {
        dev_err(
            (*line6).ifcdev,
            "read failed after %d retries\n",
            count,
        );
        ret
    } else if len != datalen as u8 {
        // should be equal or something went wrong
        dev_err(
            (*line6).ifcdev,
            "length mismatch (expected %d, got %d)\n",
            datalen as i32,
            len as i32,
        );
        -libc::EIO
    } else {
        // receive the result:
        ret = usb_control_msg_recv(
            usbdev,
            0,
            0x67,
            (libc::USB_TYPE_VENDOR | libc::USB_RECIP_DEVICE | libc::USB_DIR_IN) as u8,
            0x0013,
            0x0000,
            data,
            datalen,
            LINE6_TIMEOUT,
            GFP_KERNEL,
        );
        if ret != 0 {
            dev_err((*line6).ifcdev, "read failed (error %d)\n", ret);
        }
        ret
    }
}

// Write data to device.
pub unsafe extern "C" fn line6_write_data(
    line6: *mut usb_line6,
    address: u32,
    data: *mut libc::c_void,
    datalen: u32,
) -> i32 {
    let usbdev = (*line6).usbdev;
    let mut ret: i32;
    let status: *mut u8;
    let mut count: i32;

    if address > 0xffff || datalen > 0xffff {
        return -libc::EINVAL;
    }

    status = kmalloc(1, GFP_KERNEL) as *mut u8;
    if status.is_null() {
        return -libc::ENOMEM;
    }

    ret = usb_control_msg_send(
        usbdev,
        0,
        0x67,
        (libc::USB_TYPE_VENDOR | libc::USB_RECIP_DEVICE | libc::USB_DIR_OUT) as u8,
        0x0022,
        address as u16,
        data,
        datalen,
        LINE6_TIMEOUT,
        GFP_KERNEL,
    );
    if ret != 0 {
        dev_err(
            (*line6).ifcdev,
            "write request failed (error %d)\n",
            ret,
        );
        kfree(status as *mut libc::c_void);
        return ret;
    }

    count = 0;
    while count < LINE6_READ_WRITE_MAX_RETRIES {
        mdelay(LINE6_READ_WRITE_STATUS_DELAY);

        ret = usb_control_msg_recv(
            usbdev,
            0,
            0x67,
            (libc::USB_TYPE_VENDOR | libc::USB_RECIP_DEVICE | libc::USB_DIR_IN) as u8,
            0x0012,
            0x0000,
            status as *mut libc::c_void,
            1,
            LINE6_TIMEOUT,
            GFP_KERNEL,
        );
        if ret != 0 {
            dev_err(
                (*line6).ifcdev,
                "receiving status failed (error %d)\n",
                ret,
            );
            kfree(status as *mut libc::c_void);
            return ret;
        }

        if *status != 0xff {
            break;
        }
        count += 1;
    }

    if *status == 0xff {
        dev_err((*line6).ifcdev, "write failed after %d retries\n", count);
        ret = -libc::EIO;
    } else if *status != 0 {
        dev_err((*line6).ifcdev, "write failed (error %d)\n", ret);
        ret = -libc::EIO;
    }

    kfree(status as *mut libc::c_void);
    ret
}

// Read Line 6 device serial number.
// (POD, TonePort, GuitarPort)
pub unsafe extern "C" fn line6_read_serial_number(line6: *mut usb_line6, serial_number: *mut u32) -> i32 {
    line6_read_data(line6, 0x80d0, serial_number as *mut libc::c_void, std::mem::size_of::<u32>() as u32)
}

// Card destructor.
unsafe extern "C" fn line6_destruct(card: *mut snd_card) {
    let line6 = (*card).private_data as *mut usb_line6;
    let usbdev = (*line6).usbdev;

    // Free buffer memory first. We cannot depend on the existence of private
    // data from the (podhd) module, it may be gone already during this call
    kfree((*line6).buffer_message as *mut libc::c_void);

    kfree((*line6).buffer_listen as *mut libc::c_void);

    // then free URBs:
    usb_free_urb((*line6).urb_listen);
    (*line6).urb_listen = std::ptr::null_mut();

    // decrement reference counters:
    usb_put_dev(usbdev);
}

unsafe fn line6_get_usb_properties(line6: *mut usb_line6) {
    let usbdev = (*line6).usbdev;
    let properties = (*line6).properties;
    let mut pipe: u32;
    let mut ep: *mut usb_host_endpoint = std::ptr::null_mut();

    if (*properties).capabilities & LINE6_CAP_CONTROL != 0 {
        if (*properties).capabilities & LINE6_CAP_CONTROL_MIDI != 0 {
            pipe = usb_rcvintpipe((*line6).usbdev, (*(*line6).properties).ep_ctrl_r);
        } else {
            pipe = usb_rcvbulkpipe((*line6).usbdev, (*(*line6).properties).ep_ctrl_r);
        }
        ep = (*usbdev).ep_in[usb_pipeendpoint(pipe) as usize];
    }

    // Control data transfer properties
    if !ep.is_null() {
        (*line6).interval = (*(*ep).desc).bInterval;
        (*line6).max_packet_size = le16_to_cpu((*(*ep).desc).wMaxPacketSize);
    } else {
        if (*properties).capabilities & LINE6_CAP_CONTROL != 0 {
            dev_err(
                (*line6).ifcdev,
                "endpoint not available, using fallback values",
            );
        }
        (*line6).interval = LINE6_FALLBACK_INTERVAL;
        (*line6).max_packet_size = LINE6_FALLBACK_MAXPACKETSIZE;
    }

    // Isochronous transfer properties
    if (*usbdev).speed == USB_SPEED_LOW {
        (*line6).intervals_per_second = USB_LOW_INTERVALS_PER_SECOND;
        (*line6).iso_buffers = USB_LOW_ISO_BUFFERS;
    } else {
        (*line6).intervals_per_second = USB_HIGH_INTERVALS_PER_SECOND;
        (*line6).iso_buffers = USB_HIGH_ISO_BUFFERS;
    }
}

// Enable buffering of incoming messages, flush the buffer
unsafe extern "C" fn line6_hwdep_open(hw: *mut snd_hwdep, file: *mut libc::FILE) -> i32 {
    let line6 = (*hw).private_data as *mut usb_line6;

    // NOTE: hwdep layer provides atomicity here

    (*line6).messages.active = 1;
    (*line6).messages.nonblock = if ((*file).flags() & libc::O_NONBLOCK) != 0 { 1 } else { 0 };

    0
}

// Stop buffering
unsafe extern "C" fn line6_hwdep_release(hw: *mut snd_hwdep, file: *mut libc::FILE) -> i32 {
    let line6 = (*hw).private_data as *mut usb_line6;

    (*line6).messages.active = 0;

    0
}

// Read from circular buffer, return to user
unsafe extern "C" fn line6_hwdep_read(
    hwdep: *mut snd_hwdep,
    buf: *mut i8,
    count: i64,
    offset: *mut i64,
) -> i64 {
    let line6 = (*hwdep).private_data as *mut usb_line6;
    let mut rv: i64 = 0;
    let mut out_count: u32;

    if mutex_lock_interruptible(&mut (*line6).messages.read_lock) != 0 {
        return -libc::ERESTARTSYS as i64;
    }

    while kfifo_len(&(*line6).messages.fifo) == 0 {
        mutex_unlock(&mut (*line6).messages.read_lock);

        if (*line6).messages.nonblock != 0 {
            return -libc::EAGAIN as i64;
        }

        rv = wait_event_interruptible(&(*line6).messages.wait_queue, kfifo_len(&(*line6).messages.fifo) != 0) as i64;
        if rv < 0 {
            return rv;
        }

        if mutex_lock_interruptible(&mut (*line6).messages.read_lock) != 0 {
            return -libc::ERESTARTSYS as i64;
        }
    }

    if kfifo_peek_len(&(*line6).messages.fifo) > count as i32 {
        // Buffer too small; allow re-read of the current item...
        rv = -libc::EINVAL as i64;
    } else {
        rv = kfifo_to_user(&(*line6).messages.fifo, buf, count as i32, &mut out_count) as i64;
        if rv == 0 {
            rv = out_count as i64;
        }
    }

    mutex_unlock(&mut (*line6).messages.read_lock);
    rv
}

// Write directly (no buffering) to device by user
unsafe extern "C" fn line6_hwdep_write(
    hwdep: *mut snd_hwdep,
    data: *const i8,
    count: i64,
    offset: *mut i64,
) -> i64 {
    let line6 = (*hwdep).private_data as *mut usb_line6;
    let mut rv: i64;
    let data_copy: *mut i8;

    if count > ((*line6).max_packet_size * LINE6_RAW_MESSAGES_MAXCOUNT) as i64 {
        // This is an arbitrary limit - still better than nothing...
        return -libc::EINVAL as i64;
    }

    data_copy = memdup_user(data, count as usize) as *mut i8;
    if IS_ERR(data_copy as *const libc::c_void) != 0 {
        return PTR_ERR(data_copy as *const libc::c_void) as i64;
    }

    rv = line6_send_raw_message(line6, data_copy, count as i32) as i64;

    kfree(data_copy as *mut libc::c_void);
    rv
}

unsafe extern "C" fn line6_hwdep_poll(
    hwdep: *mut snd_hwdep,
    file: *mut libc::FILE,
    wait: *mut poll_table,
) -> u32 {
    let line6 = (*hwdep).private_data as *mut usb_line6;

    poll_wait(file, &(*line6).messages.wait_queue, wait);

    if kfifo_len(&(*line6).messages.fifo) == 0 {
        0
    } else {
        libc::EPOLLIN | libc::EPOLLRDNORM
    }
}

static hwdep_ops: snd_hwdep_ops = snd_hwdep_ops {
    open: Some(line6_hwdep_open),
    release: Some(line6_hwdep_release),
    read: Some(line6_hwdep_read),
    write: Some(line6_hwdep_write),
    poll: Some(line6_hwdep_poll),
};

// Insert into circular buffer
unsafe fn line6_hwdep_push_message(line6: *mut usb_line6) {
    if (*line6).messages.active == 0 {
        return;
    }

    if kfifo_avail(&(*line6).messages.fifo) >= (*line6).message_length {
        // No race condition here, there's only one writer
        kfifo_in(&(*line6).messages.fifo, (*line6).buffer_message, (*line6).message_length);
    }
    // else TODO: signal overflow

    wake_up_interruptible(&(*line6).messages.wait_queue);
}

unsafe fn line6_hwdep_init(line6: *mut usb_line6) -> i32 {
    let mut err: i32;
    let hwdep: *mut snd_hwdep;

    // TODO: usb_driver_claim_interface();
    (*line6).process_message = Some(line6_hwdep_push_message);
    (*line6).messages.active = 0;
    init_waitqueue_head(&mut (*line6).messages.wait_queue);
    mutex_init(&mut (*line6).messages.read_lock);
    INIT_KFIFO(&mut (*line6).messages.fifo);

    err = snd_hwdep_new((*line6).card, "config\0" as *const u8 as *const i8, 0, &mut hwdep);
    if err < 0 {
        return err;
    }
    strscpy((*hwdep).name.as_mut_ptr(), "config\0" as *const u8 as *const i8, 64);
    (*hwdep).iface = SNDRV_HWDEP_IFACE_LINE6;
    (*hwdep).ops = &hwdep_ops;
    (*hwdep).private_data = line6 as *mut libc::c_void;
    (*hwdep).exclusive = true;

    err
}

unsafe fn line6_init_cap_control(line6: *mut usb_line6) -> i32 {
    let mut ret: i32;

    // initialize USB buffers:
    (*line6).buffer_listen = kzalloc(LINE6_BUFSIZE_LISTEN as usize, GFP_KERNEL) as *mut i8;
    if (*line6).buffer_listen.is_null() {
        return -libc::ENOMEM;
    }

    (*line6).urb_listen = usb_alloc_urb(0, GFP_KERNEL);
    if (*line6).urb_listen.is_null() {
        return -libc::ENOMEM;
    }

    if (*(*line6).properties).capabilities & LINE6_CAP_CONTROL_MIDI != 0 {
        (*line6).buffer_message = kzalloc(LINE6_MIDI_MESSAGE_MAXLEN as usize, GFP_KERNEL) as *mut i8;
        if (*line6).buffer_message.is_null() {
            return -libc::ENOMEM;
        }

        ret = line6_init_midi(line6);
        if ret < 0 {
            return ret;
        }
    } else {
        ret = line6_hwdep_init(line6);
        if ret < 0 {
            return ret;
        }
    }

    ret = line6_start_listen(line6);
    if ret < 0 {
        dev_err((*line6).ifcdev, "cannot start listening: %d\n", ret);
        return ret;
    }

    0
}

unsafe extern "C" fn line6_startup_work(work: *mut work_struct) {
    let line6 = (work as *mut delayed_work as *mut u8).offset(-(std::mem::offset_of!(usb_line6, startup_work) as isize)) as *mut usb_line6;

    if !(*line6).startup.is_none() {
        (*line6).startup.unwrap()(line6);
    }
}

// Probe USB device.
pub unsafe extern "C" fn line6_probe(
    interface: *mut usb_interface,
    id: *const usb_device_id,
    driver_name: *const i8,
    properties: *const line6_properties,
    private_init: Option<unsafe extern "C" fn(*mut usb_line6, *const usb_device_id) -> i32>,
    data_size: usize,
) -> i32 {
    let usbdev = interface_to_usbdev(interface);
    let card: *mut snd_card;
    let line6: *mut usb_line6;
    let mut interface_number: i32;
    let mut ret: i32;

    if data_size < std::mem::size_of::<usb_line6>() {
        return -libc::EINVAL;
    }

    // we don't handle multiple configurations
    if (*(*usbdev).descriptor).bNumConfigurations != 1 {
        return -libc::ENODEV;
    }

    ret = snd_card_new(
        &(*interface).dev as *const device as *mut device,
        SNDRV_DEFAULT_IDX1,
        SNDRV_DEFAULT_STR1 as *const i8,
        std::ptr::null_mut(), // THIS_MODULE
        data_size,
        &mut card,
    );
    if ret < 0 {
        return ret;
    }

    // store basic data:
    line6 = (*card).private_data as *mut usb_line6;
    (*line6).card = card;
    (*line6).properties = properties;
    (*line6).usbdev = usbdev;
    (*line6).ifcdev = &(*interface).dev as *const device as *mut device;
    INIT_DELAYED_WORK(&mut (*line6).startup_work, Some(line6_startup_work));

    strscpy((*card).id.as_mut_ptr(), (*properties).id.as_ptr(), 16);
    strscpy((*card).driver.as_mut_ptr(), driver_name, 16);
    strscpy((*card).shortname.as_mut_ptr(), (*properties).name.as_ptr(), 80);
    sprintf(
        (*card).longname.as_mut_ptr(),
        "Line 6 %s at USB %s\0" as *const u8 as *const i8,
        (*properties).name.as_ptr(),
        dev_name((*line6).ifcdev),
    );
    (*card).private_free = Some(line6_destruct);

    usb_set_intfdata(interface, line6 as *mut libc::c_void);

    // increment reference counters:
    usb_get_dev(usbdev);

    // initialize device info:
    dev_info(
        &(*interface).dev,
        "Line 6 %s found\n\0" as *const u8 as *const i8,
        (*properties).name.as_ptr(),
    );

    // query interface number
    interface_number = (*(*(*interface).cur_altsetting).desc).bInterfaceNumber as i32;

    // TODO reserves the bus bandwidth even without actual transfer
    ret = usb_set_interface(usbdev, interface_number as u8, (*properties).altsetting);
    if ret < 0 {
        dev_err(&(*interface).dev, "set_interface failed\n");
        line6_disconnect(interface);
        return ret;
    }

    line6_get_usb_properties(line6);

    if (*properties).capabilities & LINE6_CAP_CONTROL != 0 {
        ret = line6_init_cap_control(line6);
        if ret < 0 {
            line6_disconnect(interface);
            return ret;
        }
    }

    // initialize device data based on device:
    if let Some(init_fn) = private_init {
        ret = init_fn(line6, id);
        if ret < 0 {
            line6_disconnect(interface);
            return ret;
        }
    }

    // creation of additional special files should go here

    dev_info(
        &(*interface).dev,
        "Line 6 %s now attached\n\0" as *const u8 as *const i8,
        (*properties).name.as_ptr(),
    );

    0
}

// Line 6 device disconnected.
pub unsafe extern "C" fn line6_disconnect(interface: *mut usb_interface) {
    let line6 = usb_get_intfdata(interface) as *mut usb_line6;
    let usbdev = interface_to_usbdev(interface);

    if line6.is_null() {
        return;
    }

    if usbdev != (*line6).usbdev {
        return;
    }

    cancel_delayed_work_sync(&mut (*line6).startup_work);

    if !(*line6).urb_listen.is_null() {
        line6_stop_listen(line6);
    }

    snd_card_disconnect((*line6).card);
    if !(*line6).line6pcm.is_null() {
        line6_pcm_disconnect((*line6).line6pcm);
    }
    if !(*line6).disconnect.is_none() {
        (*line6).disconnect.unwrap()(line6);
    }

    dev_info(
        &(*interface).dev,
        "Line 6 %s now disconnected\n\0" as *const u8 as *const i8,
        (*(*line6).properties).name.as_ptr(),
    );

    // make sure the device isn't destructed twice:
    usb_set_intfdata(interface, std::ptr::null_mut());

    snd_card_free_when_closed((*line6).card);
}

#[cfg(feature = "CONFIG_PM")]
pub unsafe extern "C" fn line6_suspend(interface: *mut usb_interface, message: pm_message_t) -> i32 {
    let line6 = usb_get_intfdata(interface) as *mut usb_line6;
    let line6pcm = (*line6).line6pcm;

    snd_power_change_state((*line6).card, SNDRV_CTL_POWER_D3hot);

    if (*(*line6).properties).capabilities & LINE6_CAP_CONTROL != 0 {
        line6_stop_listen(line6);
    }

    if !line6pcm.is_null() {
        (*line6pcm).flags = 0;
    }

    0
}

#[cfg(feature = "CONFIG_PM")]
pub unsafe extern "C" fn line6_resume(interface: *mut usb_interface) -> i32 {
    let line6 = usb_get_intfdata(interface) as *mut usb_line6;

    if (*(*line6).properties).capabilities & LINE6_CAP_CONTROL != 0 {
        let _ = line6_start_listen(line6);
    }

    snd_power_change_state((*line6).card, SNDRV_CTL_POWER_D0);
    0
}

// MODULE_AUTHOR(DRIVER_AUTHOR);
// MODULE_DESCRIPTION(DRIVER_DESC);
// MODULE_LICENSE("GPL");

// External function declarations (from other modules/headers)
extern "C" {
    fn usb_fill_int_urb(
        urb: *mut urb,
        dev: *mut usb_device,
        pipe: u32,
        transfer_buffer: *mut u8,
        buffer_length: i32,
        complete: Option<unsafe extern "C" fn(*mut urb)>,
        context: *mut libc::c_void,
        interval: u8,
    );
    fn usb_fill_bulk_urb(
        urb: *mut urb,
        dev: *mut usb_device,
        pipe: u32,
        transfer_buffer: *mut u8,
        buffer_length: i32,
        complete: Option<unsafe extern "C" fn(*mut urb)>,
        context: *mut libc::c_void,
    );
    fn usb_rcvintpipe(dev: *mut usb_device, endpoint: u8) -> u32;
    fn usb_rcvbulkpipe(dev: *mut usb_device, endpoint: u8) -> u32;
    fn usb_sndintpipe(dev: *mut usb_device, endpoint: u8) -> u32;
    fn usb_sndbulkpipe(dev: *mut usb_device, endpoint: u8) -> u32;
    fn usb_pipeendpoint(pipe: u32) -> u32;
    fn usb_urb_ep_type_check(urb: *mut urb) -> i32;
    fn usb_submit_urb(urb: *mut urb, mem_flags: u32) -> i32;
    fn usb_kill_urb(urb: *mut urb);
    fn usb_interrupt_msg(
        usb_dev: *mut usb_device,
        pipe: u32,
        data: *mut i8,
        len: i32,
        actual_length: *mut i32,
        timeout: i32,
    ) -> i32;
    fn usb_bulk_msg(
        usb_dev: *mut usb_device,
        pipe: u32,
        data: *mut i8,
        len: i32,
        actual_length: *mut i32,
        timeout: i32,
    ) -> i32;
    fn usb_free_urb(urb: *mut urb);
    fn usb_alloc_urb(iso_packets: i32, mem_flags: u32) -> *mut urb;
    fn usb_control_msg_send(
        dev: *mut usb_device,
        endpoint: u8,
        request: u8,
        requesttype: u8,
        value: u16,
        index: u16,
        data: *mut libc::c_void,
        size: u16,
        timeout: i32,
        mem_flags: u32,
    ) -> i32;
    fn usb_control_msg_recv(
        dev: *mut usb_device,
        endpoint: u8,
        request: u8,
        requesttype: u8,
        value: u16,
        index: u16,
        data: *mut libc::c_void,
        size: u16,
        timeout: i32,
        mem_flags: u32,
    ) -> i32;
    fn usb_get_dev(dev: *mut usb_device) -> *mut usb_device;
    fn usb_put_dev(dev: *mut usb_device);
    fn usb_set_interface(dev: *mut usb_device, ifnum: u8, alternate: u8) -> i32;
    fn usb_get_intfdata(intf: *mut usb_interface) -> *mut libc::c_void;
    fn usb_set_intfdata(intf: *mut usb_interface, data: *mut libc::c_void);
    fn interface_to_usbdev(intf: *mut usb_interface) -> *mut usb_device;
    fn dev_err(dev: *mut device, fmt: *const i8, ...);
    fn dev_info(dev: *mut device, fmt: *const i8, ...);
    fn dev_dbg(dev: *mut device, fmt: *const i8, ...);
    fn kmalloc(size: usize, flags: u32) -> *mut libc::c_void;
    fn kzalloc(size: usize, flags: u32) -> *mut libc::c_void;
    fn kzalloc_obj(flags: u32) -> *mut libc::c_void;
    fn kmemdup(src: *const libc::c_void, len: usize, flags: u32) -> *mut libc::c_void;
    fn kfree(ptr: *mut libc::c_void);
    fn memdup_user(src: *const i8, n: usize) -> *mut i8;
    fn IS_ERR(ptr: *const libc::c_void) -> i32;
    fn PTR_ERR(ptr: *const libc::c_void) -> i32;
    fn mdelay(msecs: i32);
    fn le16_to_cpu(x: u16) -> u16;
    fn snd_card_new(
        parent: *mut device,
        idx: i32,
        xid: *const i8,
        module: *mut libc::c_void,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> i32;
    fn snd_card_disconnect(card: *mut snd_card) -> i32;
    fn snd_card_free_when_closed(card: *mut snd_card);
    fn snd_power_change_state(card: *mut snd_card, power_state: i32);
    fn snd_hwdep_new(card: *mut snd_card, id: *const i8, device: i32, rhwdep: *mut *mut snd_hwdep) -> i32;
    fn strscpy(dest: *mut i8, src: *const i8, count: usize) -> usize;
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn dev_name(dev: *mut device) -> *const i8;
    fn line6_midibuf_write(mb: *mut midi_buffer, data: *mut i8, len: i32) -> i32;
    fn line6_midibuf_read(mb: *mut midi_buffer, data: *mut i8, len: i32, read_type: i32) -> i32;
    fn line6_midibuf_ignore(mb: *mut midi_buffer, len: i32);
    fn line6_midi_receive(line6: *mut usb_line6, data: *mut i8, len: i32);
    fn line6_init_midi(line6: *mut usb_line6) -> i32;
    fn line6_pcm_disconnect(line6pcm: *mut snd_line6_pcm);
    fn init_waitqueue_head(q: *mut wait_queue_head_t);
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock_interruptible(lock: *mut mutex) -> i32;
    fn mutex_unlock(lock: *mut mutex);
    fn wait_event_interruptible(wq: *mut wait_queue_head_t, condition: i32) -> i32;
    fn wake_up_interruptible(q: *mut wait_queue_head_t);
    fn poll_wait(filp: *mut libc::FILE, wait_address: *mut wait_queue_head_t, p: *mut poll_table);
    fn kfifo_in(fifo: *mut kfifo, buf: *mut i8, len: i32) -> i32;
    fn kfifo_out_peek(fifo: *mut kfifo, buf: *mut i8, len: i32, reclen: *mut i32) -> i32;
    fn kfifo_len(fifo: *const kfifo) -> i32;
    fn kfifo_avail(fifo: *const kfifo) -> i32;
    fn kfifo_to_user(fifo: *mut kfifo, buf: *mut i8, len: i32, copied: *mut u32) -> i32;
    fn kfifo_peek_len(fifo: *const kfifo) -> i32;
    fn INIT_KFIFO(fifo: *mut kfifo);
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;
}

// Type definitions for Linux kernel structures
#[repr(C)]
struct urb {
    // Opaque structure - declaration only
}

#[repr(C)]
struct usb_device {
    // Opaque structure - declaration only
}

#[repr(C)]
struct usb_interface {
    // Opaque structure - declaration only
}

#[repr(C)]
struct device {
    // Opaque structure - declaration only
}

#[repr(C)]
struct snd_card {
    // Opaque structure - declaration only
}

#[repr(C)]
struct snd_hwdep {
    // Opaque structure - declaration only
}

#[repr(C)]
struct usb_line6 {
    // Opaque structure - declaration only
}

#[repr(C)]
struct usb_device_id {
    // Opaque structure - declaration only
}

#[repr(C)]
struct line6_properties {
    // Opaque structure - declaration only
}

#[repr(C)]
struct usb_host_endpoint {
    // Opaque structure - declaration only
}

#[repr(C)]
struct midi_buffer {
    // Opaque structure - declaration only
}

#[repr(C)]
struct snd_line6_pcm {
    // Opaque structure - declaration only
}

#[repr(C)]
struct work_struct {
    // Opaque structure - declaration only
}

#[repr(C)]
struct delayed_work {
    // Opaque structure - declaration only
}

#[repr(C)]
struct wait_queue_head_t {
    // Opaque structure - declaration only
}

#[repr(C)]
struct mutex {
    // Opaque structure - declaration only
}

#[repr(C)]
struct kfifo {
    // Opaque structure - declaration only
}

#[repr(C)]
struct poll_table {
    // Opaque structure - declaration only
}

#[repr(C)]
struct snd_hwdep_ops {
    open: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut libc::FILE) -> i32>,
    release: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut libc::FILE) -> i32>,
    read: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut i8, i64, *mut i64) -> i64>,
    write: Option<unsafe extern "C" fn(*mut snd_hwdep, *const i8, i64, *mut i64) -> i64>,
    poll: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut libc::FILE, *mut poll_table) -> u32>,
}

#[repr(C)]
struct pm_message_t {
    event: i32,
}

#[repr(C)]
struct usb_descriptor {
    bNumConfigurations: u8,
}

const LINE6_BUFSIZE_LISTEN: i32 = 64;
const LINE6_MIDI_MESSAGE_MAXLEN: i32 = 256;
const LINE6_MIDIBUF_READ_RX: i32 = 0;
const LINE6_FALLBACK_INTERVAL: u8 = 10;
const LINE6_FALLBACK_MAXPACKETSIZE: i32 = 16;
const LINE6_TIMEOUT: i32 = 2;
const SYSEX_EXTRA_SIZE: i32 = 4;
const LINE6_SYSEX_BEGIN: u32 = 0xf0;
const LINE6_SYSEX_END: u32 = 0xf7;
const LINE6_RAW_MESSAGES_MAXCOUNT: i32 = 32;
const LINE6_CAP_CONTROL: i32 = 0x1;
const LINE6_CAP_CONTROL_MIDI: i32 = 0x2;
const GFP_ATOMIC: u32 = 0x20;
const GFP_KERNEL: u32 = 0x0;
const USB_SPEED_LOW: i32 = 1;
const USB_LOW_INTERVALS_PER_SECOND: i32 = 1000;
const USB_LOW_ISO_BUFFERS: i32 = 2;
const USB_HIGH_INTERVALS_PER_SECOND: i32 = 8000;
const USB_HIGH_ISO_BUFFERS: i32 = 10;
const SNDRV_DEFAULT_IDX1: i32 = -1;
const SNDRV_DEFAULT_STR1: &str = "";
const SNDRV_HWDEP_IFACE_LINE6: i32 = 1;
const SNDRV_CTL_POWER_D0: i32 = 0;
const SNDRV_CTL_POWER_D3hot: i32 = 3;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
