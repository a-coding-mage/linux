// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * caiaq.c: ALSA driver for caiaq/NativeInstruments devices
 *
 *   Copyright (c) 2007 Daniel Mack <daniel@caiaq.de>
 *                      Karsten Wiese <fzu@wemgehoertderstaat.de>
*/

// Linux kernel headers: moduleparam, device, interrupt, module, init, gfp, usb
// Sound ALSA headers: initval, core, pcm

// External modules: device, audio, midi, control, input

use std::ffi::c_char;

const MODNAME: &str = "caiaq";

// Static arrays with module parameters
static mut INDEX: [i32; SNDRV_CARDS] = SNDRV_DEFAULT_IDX;
static mut ID: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR;
static mut ENABLE: [bool; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP;

// module_param_array(index, int, NULL, 0444);
// MODULE_PARM_DESC(index, "Index value for the caiaq sound device");
// module_param_array(id, charp, NULL, 0444);
// MODULE_PARM_DESC(id, "ID string for the caiaq soundcard.");
// module_param_array(enable, bool, NULL, 0444);
// MODULE_PARM_DESC(enable, "Enable the caiaq soundcard.");

// Sample rate codes
const SAMPLERATE_44100: u8 = 0;
const SAMPLERATE_48000: u8 = 1;
const SAMPLERATE_96000: u8 = 2;
const SAMPLERATE_192000: u8 = 3;
const SAMPLERATE_88200: u8 = 4;
const SAMPLERATE_INVALID: u8 = 0xff;

// Audio depth codes
const DEPTH_NONE: u8 = 0;
const DEPTH_16: u8 = 1;
const DEPTH_24: u8 = 2;
const DEPTH_32: u8 = 3;

// USB device ID table
#[repr(C)]
struct usb_device_id {
    match_flags: u16,
    idVendor: u16,
    idProduct: u16,
    // Note: Full usb_device_id structure from kernel, other fields omitted
}

static SND_USB_ID_TABLE: [usb_device_id; 15] = [
    usb_device_id {
        match_flags: USB_DEVICE_ID_MATCH_DEVICE,
        idVendor: USB_VID_NATIVEINSTRUMENTS,
        idProduct: USB_PID_RIGKONTROL2,
    },
    usb_device_id {
        match_flags: USB_DEVICE_ID_MATCH_DEVICE,
        idVendor: USB_VID_NATIVEINSTRUMENTS,
        idProduct: USB_PID_RIGKONTROL3,
    },
    usb_device_id {
        match_flags: USB_DEVICE_ID_MATCH_DEVICE,
        idVendor: USB_VID_NATIVEINSTRUMENTS,
        idProduct: USB_PID_KORECONTROLLER,
    },
    usb_device_id {
        match_flags: USB_DEVICE_ID_MATCH_DEVICE,
        idVendor: USB_VID_NATIVEINSTRUMENTS,
        idProduct: USB_PID_KORECONTROLLER2,
    },
    usb_device_id {
        match_flags: USB_DEVICE_ID_MATCH_DEVICE,
        idVendor: USB_VID_NATIVEINSTRUMENTS,
        idProduct: USB_PID_AK1,
    },
    usb_device_id {
        match_flags: USB_DEVICE_ID_MATCH_DEVICE,
        idVendor: USB_VID_NATIVEINSTRUMENTS,
        idProduct: USB_PID_AUDIO8DJ,
    },
    usb_device_id {
        match_flags: USB_DEVICE_ID_MATCH_DEVICE,
        idVendor: USB_VID_NATIVEINSTRUMENTS,
        idProduct: USB_PID_SESSIONIO,
    },
    usb_device_id {
        match_flags: USB_DEVICE_ID_MATCH_DEVICE,
        idVendor: USB_VID_NATIVEINSTRUMENTS,
        idProduct: USB_PID_GUITARRIGMOBILE,
    },
    usb_device_id {
        match_flags: USB_DEVICE_ID_MATCH_DEVICE,
        idVendor: USB_VID_NATIVEINSTRUMENTS,
        idProduct: USB_PID_AUDIO4DJ,
    },
    usb_device_id {
        match_flags: USB_DEVICE_ID_MATCH_DEVICE,
        idVendor: USB_VID_NATIVEINSTRUMENTS,
        idProduct: USB_PID_AUDIO2DJ,
    },
    usb_device_id {
        match_flags: USB_DEVICE_ID_MATCH_DEVICE,
        idVendor: USB_VID_NATIVEINSTRUMENTS,
        idProduct: USB_PID_TRAKTORKONTROLX1,
    },
    usb_device_id {
        match_flags: USB_DEVICE_ID_MATCH_DEVICE,
        idVendor: USB_VID_NATIVEINSTRUMENTS,
        idProduct: USB_PID_TRAKTORKONTROLS4,
    },
    usb_device_id {
        match_flags: USB_DEVICE_ID_MATCH_DEVICE,
        idVendor: USB_VID_NATIVEINSTRUMENTS,
        idProduct: USB_PID_TRAKTORAUDIO2,
    },
    usb_device_id {
        match_flags: USB_DEVICE_ID_MATCH_DEVICE,
        idVendor: USB_VID_NATIVEINSTRUMENTS,
        idProduct: USB_PID_MASCHINECONTROLLER,
    },
    usb_device_id {
        // terminator
        match_flags: 0,
        idVendor: 0,
        idProduct: 0,
    },
];

// External types from kernel headers
#[repr(C)]
pub struct urb {
    // Opaque kernel URB structure
}

#[repr(C)]
pub struct device {
    // Opaque kernel device structure
}

#[repr(C)]
pub struct snd_card {
    // Opaque ALSA card structure
}

#[repr(C)]
pub struct usb_device {
    // Opaque USB device structure
}

#[repr(C)]
pub struct usb_interface {
    // Opaque USB interface structure
}

#[repr(C)]
pub struct snd_usb_caiaqdev {
    // Opaque caiaq device structure
}

#[repr(C)]
pub struct caiaq_device_spec {
    // Opaque device spec structure
}

// External kernel functions and macros
extern "C" {
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);

    fn snd_usb_caiaq_midi_handle_input(
        cdev: *mut snd_usb_caiaqdev,
        port: u8,
        buf: *const u8,
        len: u8,
    );
    fn snd_usb_caiaq_input_dispatch(
        cdev: *mut snd_usb_caiaqdev,
        buf: *const u8,
        len: i32,
    );
    fn snd_usb_caiaq_input_init(cdev: *mut snd_usb_caiaqdev) -> i32;
    fn snd_usb_caiaq_input_free(cdev: *mut snd_usb_caiaqdev);
    fn snd_usb_caiaq_input_disconnect(cdev: *mut snd_usb_caiaqdev);

    fn snd_usb_caiaq_audio_init(cdev: *mut snd_usb_caiaqdev) -> i32;
    fn snd_usb_caiaq_audio_free(cdev: *mut snd_usb_caiaqdev);
    fn snd_usb_caiaq_audio_disconnect(cdev: *mut snd_usb_caiaqdev);

    fn snd_usb_caiaq_midi_init(cdev: *mut snd_usb_caiaqdev) -> i32;
    fn snd_usb_caiaq_midi_output_done(urb: *mut urb);

    fn snd_usb_caiaq_control_init(cdev: *mut snd_usb_caiaqdev) -> i32;

    fn usb_submit_urb(urb: *mut urb, mem_flags: u32) -> i32;
    fn usb_kill_urb(urb: *mut urb);
    fn usb_init_urb(urb: *mut urb);
    fn usb_fill_bulk_urb(
        urb: *mut urb,
        dev: *mut usb_device,
        pipe: u32,
        transfer_buffer: *mut u8,
        transfer_buffer_length: i32,
        complete: extern "C" fn(*mut urb),
        context: *mut c_char,
    );
    fn usb_rcvbulkpipe(dev: *mut usb_device, endpoint: u32) -> u32;
    fn usb_sndbulkpipe(dev: *mut usb_device, endpoint: u32) -> u32;
    fn usb_bulk_msg(
        usb_dev: *mut usb_device,
        pipe: u32,
        data: *mut u8,
        len: i32,
        actual_length: *mut i32,
        timeout: i32,
    ) -> i32;
    fn usb_urb_ep_type_check(urb: *mut urb) -> i32;
    fn usb_set_interface(dev: *mut usb_device, ifnum: i32, alternate: i32) -> i32;
    fn usb_string(
        dev: *mut usb_device,
        index: i32,
        buf: *mut c_char,
        size: i32,
    ) -> i32;
    fn usb_make_path(dev: *mut usb_device, buf: *mut c_char, size: i32) -> i32;
    fn usb_get_dev(dev: *mut usb_device) -> *mut usb_device;
    fn usb_put_dev(dev: *mut usb_device);
    fn usb_get_intfdata(intf: *mut usb_interface) -> *mut c_char;
    fn usb_set_intfdata(intf: *mut usb_interface, data: *mut c_char);
    fn interface_to_usbdev(intf: *mut usb_interface) -> *mut usb_device;

    fn snd_card_new(
        dev: *mut device,
        idx: i32,
        xid: *const c_char,
        module: *mut c_char,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> i32;
    fn snd_card_free(card: *mut snd_card);
    fn snd_card_free_when_closed(card: *mut snd_card);
    fn snd_card_register(card: *mut snd_card) -> i32;
    fn snd_card_disconnect(card: *mut snd_card) -> i32;
    fn snd_card_set_id(card: *mut snd_card, nid: *const c_char);

    fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
    fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8;
    fn le16_to_cpu(x: u16) -> u16;
    fn wait_event_timeout(wq: *mut c_char, condition: i32, timeout: u32) -> i32;
    fn init_waitqueue_head(wq: *mut c_char);
    fn wake_up(wq: *mut c_char);
    fn spin_lock_init(lock: *mut c_char);
    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> i32;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> i32;
    fn min_t(type_: usize, x: u32, y: u32) -> u32;
}

unsafe extern "C" fn usb_ep1_command_reply_dispatch(urb: *mut urb) {
    let mut ret: i32;
    let dev: *mut device = &mut (*urb).dev.dev;
    let cdev: *mut snd_usb_caiaqdev = (*urb).context as *mut snd_usb_caiaqdev;
    let buf: *mut u8 = (*urb).transfer_buffer as *mut u8;
    let mut payload_len: u32;
    let mut copy_len: u32;

    if (*urb).status != 0 || cdev.is_null() {
        dev_warn(dev, b"received EP1 urb->status = %i\n\0".as_ptr() as *const c_char, (*urb).status);
        return;
    }
    if (*urb).actual_length < 1 {
        return;
    }

    payload_len = ((*urb).actual_length - 1) as u32;

    match *buf {
        EP1_CMD_GET_DEVICE_INFO => {
            if payload_len < std::mem::size_of::<caiaq_device_spec>() as u32 {
                return;
            }
            memcpy(
                &mut (*cdev).spec as *mut _ as *mut u8,
                buf.add(1),
                std::mem::size_of::<caiaq_device_spec>(),
            );
            (*cdev).spec.fw_version = le16_to_cpu((*cdev).spec.fw_version);
            dev_dbg(
                dev,
                b"device spec (firmware %d): audio: %d in, %d out, MIDI: %d in, %d out, data alignment %d\n\0".as_ptr() as *const c_char,
                (*cdev).spec.fw_version,
                (*cdev).spec.num_analog_audio_in,
                (*cdev).spec.num_analog_audio_out,
                (*cdev).spec.num_midi_in,
                (*cdev).spec.num_midi_out,
                (*cdev).spec.data_alignment,
            );

            (*cdev).spec_received += 1;
            wake_up(&mut (*cdev).ep1_wait_queue as *mut _ as *mut c_char);
        }
        EP1_CMD_AUDIO_PARAMS => {
            if payload_len < 1 {
                return;
            }
            (*cdev).audio_parm_answer = *buf.add(1) as i32;
            wake_up(&mut (*cdev).ep1_wait_queue as *mut _ as *mut c_char);
        }
        EP1_CMD_MIDI_READ => {
            if (*urb).actual_length < 3 || (*urb).actual_length - 3 < *buf.add(2) as i32 {
                return;
            }
            snd_usb_caiaq_midi_handle_input(cdev, *buf.add(1), buf.add(3), *buf.add(2));
        }
        EP1_CMD_READ_IO => {
            if (*cdev).chip.usb_id == USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_AUDIO8DJ) {
                copy_len = min_t(
                    std::mem::size_of::<[u8; 256]>(),
                    payload_len,
                    std::mem::size_of::<[u8; 256]>() as u32,
                );
                memcpy(
                    (*cdev).control_state.as_mut_ptr(),
                    buf.add(1),
                    copy_len as usize,
                );
                wake_up(&mut (*cdev).ep1_wait_queue as *mut _ as *mut c_char);
                return;
            }
            // CONFIG_SND_USB_CAIAQ_INPUT: fallthrough to next cases
            #[cfg(CONFIG_SND_USB_CAIAQ_INPUT)]
            {
                snd_usb_caiaq_input_dispatch(cdev, buf, (*urb).actual_length);
            }
        }
        #[cfg(CONFIG_SND_USB_CAIAQ_INPUT)]
        EP1_CMD_READ_ERP | EP1_CMD_READ_ANALOG => {
            snd_usb_caiaq_input_dispatch(cdev, buf, (*urb).actual_length);
        }
        _ => {}
    }

    (*urb).actual_length = 0;
    ret = usb_submit_urb(urb, GFP_ATOMIC);
    if ret < 0 {
        dev_err(dev, b"unable to submit urb. OOM!?\n\0".as_ptr() as *const c_char);
    }
}

pub fn snd_usb_caiaq_send_command(
    cdev: *mut snd_usb_caiaqdev,
    command: u8,
    buffer: *const u8,
    len: i32,
) -> i32 {
    let mut actual_len: i32 = 0;
    let usb_dev: *mut usb_device = (*cdev).chip.dev;

    if usb_dev.is_null() {
        return -libc::EIO;
    }

    let mut len = len;
    if len > EP1_BUFSIZE - 1 {
        len = EP1_BUFSIZE - 1;
    }

    if !buffer.is_null() && len > 0 {
        memcpy(
            (*cdev).ep1_out_buf.as_mut_ptr().add(1),
            buffer,
            len as usize,
        );
    }

    (*cdev).ep1_out_buf[0] = command;
    usb_bulk_msg(
        usb_dev,
        usb_sndbulkpipe(usb_dev, 1),
        (*cdev).ep1_out_buf.as_mut_ptr(),
        len + 1,
        &mut actual_len,
        200,
    )
}

pub fn snd_usb_caiaq_send_command_bank(
    cdev: *mut snd_usb_caiaqdev,
    command: u8,
    bank: u8,
    buffer: *const u8,
    len: i32,
) -> i32 {
    let mut actual_len: i32 = 0;
    let usb_dev: *mut usb_device = (*cdev).chip.dev;

    if usb_dev.is_null() {
        return -libc::EIO;
    }

    let mut len = len;
    if len > EP1_BUFSIZE - 2 {
        len = EP1_BUFSIZE - 2;
    }

    if !buffer.is_null() && len > 0 {
        memcpy(
            (*cdev).ep1_out_buf.as_mut_ptr().add(2),
            buffer,
            len as usize,
        );
    }

    (*cdev).ep1_out_buf[0] = command;
    (*cdev).ep1_out_buf[1] = bank;

    usb_bulk_msg(
        usb_dev,
        usb_sndbulkpipe(usb_dev, 1),
        (*cdev).ep1_out_buf.as_mut_ptr(),
        len + 2,
        &mut actual_len,
        200,
    )
}

pub fn snd_usb_caiaq_set_audio_params(
    cdev: *mut snd_usb_caiaqdev,
    rate: i32,
    depth: i32,
    bpp: i32,
) -> i32 {
    let mut ret: i32;
    let mut tmp: [u8; 5] = [0; 5];
    let dev: *mut device = caiaqdev_to_dev(cdev);

    tmp[0] = match rate {
        44100 => SAMPLERATE_44100,
        48000 => SAMPLERATE_48000,
        88200 => SAMPLERATE_88200,
        96000 => SAMPLERATE_96000,
        192000 => SAMPLERATE_192000,
        _ => return -libc::EINVAL,
    };

    tmp[1] = match depth {
        16 => DEPTH_16,
        24 => DEPTH_24,
        _ => return -libc::EINVAL,
    };

    tmp[2] = (bpp & 0xff) as u8;
    tmp[3] = (bpp >> 8) as u8;
    tmp[4] = 1;

    unsafe {
        dev_dbg(
            dev,
            b"setting audio params: %d Hz, %d bits, %d bpp\n\0".as_ptr() as *const c_char,
            rate,
            depth,
            bpp,
        );
    }

    (*cdev).audio_parm_answer = -1;
    ret = snd_usb_caiaq_send_command(cdev, EP1_CMD_AUDIO_PARAMS, tmp.as_ptr(), std::mem::size_of_val(&tmp) as i32);

    if ret != 0 {
        return ret;
    }

    if wait_event_timeout(
        &mut (*cdev).ep1_wait_queue as *mut _ as *mut c_char,
        if (*cdev).audio_parm_answer >= 0 { 1 } else { 0 },
        HZ,
    ) == 0
    {
        return -libc::EPIPE;
    }

    if (*cdev).audio_parm_answer != 1 {
        unsafe {
            dev_dbg(
                dev,
                b"unable to set the device\'s audio params\n\0".as_ptr() as *const c_char,
            );
        }
    } else {
        (*cdev).bpp = bpp;
    }

    if (*cdev).audio_parm_answer == 1 {
        0
    } else {
        -libc::EINVAL
    }
}

pub fn snd_usb_caiaq_set_auto_msg(
    cdev: *mut snd_usb_caiaqdev,
    digital: i32,
    analog: i32,
    erp: i32,
) -> i32 {
    let tmp: [u8; 3] = [digital as u8, analog as u8, erp as u8];
    snd_usb_caiaq_send_command(cdev, EP1_CMD_AUTO_MSG, tmp.as_ptr(), std::mem::size_of_val(&tmp) as i32)
}

unsafe fn setup_card(cdev: *mut snd_usb_caiaqdev) -> i32 {
    let mut ret: i32;
    let mut val: [u8; 4] = [0; 4];
    let dev: *mut device = caiaqdev_to_dev(cdev);

    match (*cdev).chip.usb_id {
        _ if (*cdev).chip.usb_id == USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_RIGKONTROL2) => {
            val[0] = 0x00;
            val[1] = 0x00;
            val[2] = 0x01;
            snd_usb_caiaq_send_command(cdev, EP1_CMD_WRITE_IO, val.as_ptr(), 3);
        }
        _ if (*cdev).chip.usb_id == USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_RIGKONTROL3) => {
            val[0] = 0x00;
            val[1] = 0x40;
            val[2] = 0x40;
            val[3] = 0x00;
            snd_usb_caiaq_send_command(cdev, EP1_CMD_WRITE_IO, val.as_ptr(), 4);
        }
        _ if (*cdev).chip.usb_id == USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_AK1) => {
            val[0] = 0x00;
            snd_usb_caiaq_send_command(cdev, EP1_CMD_WRITE_IO, val.as_ptr(), 1);
        }
        _ if (*cdev).chip.usb_id == USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_AUDIO8DJ) => {
            (*cdev).control_state[0] = 0xff;
            snd_usb_caiaq_set_auto_msg(cdev, 1, 0, 0);
            snd_usb_caiaq_send_command(cdev, EP1_CMD_READ_IO, std::ptr::null(), 0);

            if wait_event_timeout(
                &mut (*cdev).ep1_wait_queue as *mut _ as *mut c_char,
                if (*cdev).control_state[0] != 0xff { 1 } else { 0 },
                HZ,
            ) == 0
            {
                dev_err(dev, b"Read timeout for control state\n\0".as_ptr() as *const c_char);
                return -libc::EINVAL;
            }

            if ((*cdev).control_state[1] != 2)
                || ((*cdev).control_state[2] != 3)
                || ((*cdev).control_state[4] != 2)
            {
                (*cdev).control_state[1] = 2;
                (*cdev).control_state[2] = 3;
                (*cdev).control_state[4] = 2;
                snd_usb_caiaq_send_command(
                    cdev,
                    EP1_CMD_WRITE_IO,
                    (*cdev).control_state.as_ptr(),
                    6,
                );
            }
        }
        _ => {}
    }

    if (*cdev).spec.num_analog_audio_out + (*cdev).spec.num_analog_audio_in
        + (*cdev).spec.num_digital_audio_out + (*cdev).spec.num_digital_audio_in
        > 0
    {
        ret = snd_usb_caiaq_audio_init(cdev);
        if ret < 0 {
            dev_err(dev, b"Unable to set up audio system (ret=%d)\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
    }

    if (*cdev).spec.num_midi_in + (*cdev).spec.num_midi_out > 0 {
        ret = snd_usb_caiaq_midi_init(cdev);
        if ret < 0 {
            dev_err(dev, b"Unable to set up MIDI system (ret=%d)\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
    }

    #[cfg(CONFIG_SND_USB_CAIAQ_INPUT)]
    {
        ret = snd_usb_caiaq_input_init(cdev);
        if ret < 0 && ret != -libc::ENODEV {
            dev_err(dev, b"Unable to set up input system (ret=%d)\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
    }

    ret = snd_card_register((*cdev).chip.card);
    if ret < 0 {
        dev_err(dev, b"snd_card_register() returned %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = snd_usb_caiaq_control_init(cdev);
    if ret < 0 {
        dev_err(dev, b"Unable to set up control system (ret=%d)\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    0
}

unsafe extern "C" fn card_free(card: *mut snd_card) {
    let cdev: *mut snd_usb_caiaqdev = caiaqdev(card);

    #[cfg(CONFIG_SND_USB_CAIAQ_INPUT)]
    {
        snd_usb_caiaq_input_free(cdev);
    }
    snd_usb_caiaq_audio_free(cdev);
    usb_put_dev((*cdev).chip.dev);
}

unsafe fn create_card(
    usb_dev: *mut usb_device,
    intf: *mut usb_interface,
    cardp: *mut *mut snd_card,
) -> i32 {
    let mut devnum: i32;
    let mut err: i32;
    let mut card: *mut snd_card = std::ptr::null_mut();
    let mut cdev: *mut snd_usb_caiaqdev;

    devnum = 0;
    while devnum < SNDRV_CARDS {
        if ENABLE[devnum as usize] {
            break;
        }
        devnum += 1;
    }

    if devnum >= SNDRV_CARDS {
        return -libc::ENODEV;
    }

    err = snd_card_new(
        &mut (*intf).dev,
        INDEX[devnum as usize],
        ID[devnum as usize],
        std::ptr::null_mut(),
        std::mem::size_of::<snd_usb_caiaqdev>(),
        &mut card,
    );
    if err < 0 {
        return err;
    }

    cdev = caiaqdev(card);
    (*cdev).chip.dev = usb_get_dev(usb_dev);
    (*card).private_free = Some(card_free);
    (*cdev).chip.card = card;
    (*cdev).chip.usb_id = USB_ID(
        le16_to_cpu((*usb_dev).descriptor.idVendor),
        le16_to_cpu((*usb_dev).descriptor.idProduct),
    );
    spin_lock_init(&mut (*cdev).spinlock as *mut _ as *mut c_char);

    *cardp = card;
    0
}

unsafe fn init_card(cdev: *mut snd_usb_caiaqdev) -> i32 {
    let mut c: *mut c_char;
    let mut usbpath: [c_char; 32] = [0; 32];
    let usb_dev: *mut usb_device = (*cdev).chip.dev;
    let card: *mut snd_card = (*cdev).chip.card;
    let dev: *mut device = caiaqdev_to_dev(cdev);
    let mut err: i32;
    let mut len: i32;

    if usb_set_interface(usb_dev, 0, 1) != 0 {
        dev_err(dev, b"can\'t set alt interface.\n\0".as_ptr() as *const c_char);
        return -libc::EIO;
    }

    usb_init_urb(&mut (*cdev).ep1_in_urb);
    usb_init_urb(&mut (*cdev).midi_out_urb);

    usb_fill_bulk_urb(
        &mut (*cdev).ep1_in_urb,
        usb_dev,
        usb_rcvbulkpipe(usb_dev, 0x1),
        (*cdev).ep1_in_buf.as_mut_ptr(),
        EP1_BUFSIZE as i32,
        usb_ep1_command_reply_dispatch,
        cdev as *mut c_char,
    );

    usb_fill_bulk_urb(
        &mut (*cdev).midi_out_urb,
        usb_dev,
        usb_sndbulkpipe(usb_dev, 0x1),
        (*cdev).midi_out_buf.as_mut_ptr(),
        EP1_BUFSIZE as i32,
        snd_usb_caiaq_midi_output_done,
        cdev as *mut c_char,
    );

    if usb_urb_ep_type_check(&mut (*cdev).ep1_in_urb) != 0
        || usb_urb_ep_type_check(&mut (*cdev).midi_out_urb) != 0
    {
        dev_err(dev, b"invalid EPs\n\0".as_ptr() as *const c_char);
        return -libc::EINVAL;
    }

    init_waitqueue_head(&mut (*cdev).ep1_wait_queue as *mut _ as *mut c_char);
    init_waitqueue_head(&mut (*cdev).prepare_wait_queue as *mut _ as *mut c_char);

    if usb_submit_urb(&mut (*cdev).ep1_in_urb, GFP_KERNEL) != 0 {
        return -libc::EIO;
    }

    err = snd_usb_caiaq_send_command(cdev, EP1_CMD_GET_DEVICE_INFO, std::ptr::null(), 0);
    if err != 0 {
        goto_err_kill_urb(cdev);
        return err;
    }

    if wait_event_timeout(
        &mut (*cdev).ep1_wait_queue as *mut _ as *mut c_char,
        if (*cdev).spec_received != 0 { 1 } else { 0 },
        HZ,
    ) == 0
    {
        goto_err_kill_urb(cdev);
        return -libc::ENODEV;
    }

    usb_string(
        usb_dev,
        (*usb_dev).descriptor.iManufacturer,
        (*cdev).vendor_name.as_mut_ptr(),
        CAIAQ_USB_STR_LEN as i32,
    );

    usb_string(
        usb_dev,
        (*usb_dev).descriptor.iProduct,
        (*cdev).product_name.as_mut_ptr(),
        CAIAQ_USB_STR_LEN as i32,
    );

    strscpy(
        (*card).driver.as_mut_ptr(),
        MODNAME.as_ptr() as *const c_char,
        std::mem::size_of_val(&(*card).driver),
    );
    strscpy(
        (*card).shortname.as_mut_ptr(),
        (*cdev).product_name.as_ptr(),
        std::mem::size_of_val(&(*card).shortname),
    );
    strscpy(
        (*card).mixername.as_mut_ptr(),
        (*cdev).product_name.as_ptr(),
        std::mem::size_of_val(&(*card).mixername),
    );

    if *(*card).id.as_ptr() == 0 {
        let mut id: [c_char; 16] = [0; 16]; // Approximate size
        memset(id.as_mut_ptr() as *mut u8, 0, std::mem::size_of_val(&id));

        c = (*card).shortname.as_mut_ptr();
        len = 0;
        while *c != 0 && len < std::mem::size_of_val(&id) as i32 - 1 {
            if *c != b' ' as c_char {
                id[len as usize] = *c;
                len += 1;
            }
            c = c.add(1);
        }

        snd_card_set_id(card, id.as_ptr());
    }

    usb_make_path(usb_dev, usbpath.as_mut_ptr(), std::mem::size_of_val(&usbpath) as i32);
    scnprintf(
        (*card).longname.as_mut_ptr(),
        std::mem::size_of_val(&(*card).longname),
        b"%s %s (%s)\0".as_ptr() as *const c_char,
        (*cdev).vendor_name.as_ptr(),
        (*cdev).product_name.as_ptr(),
        usbpath.as_ptr(),
    );

    err = setup_card(cdev);
    if err < 0 {
        goto_err_kill_urb(cdev);
        return err;
    }

    0
}

unsafe fn goto_err_kill_urb(cdev: *mut snd_usb_caiaqdev) {
    usb_kill_urb(&mut (*cdev).ep1_in_urb);
}

unsafe extern "C" fn snd_probe(intf: *mut usb_interface, id: *const usb_device_id) -> i32 {
    let mut ret: i32;
    let mut card: *mut snd_card = std::ptr::null_mut();
    let usb_dev: *mut usb_device = interface_to_usbdev(intf);

    ret = create_card(usb_dev, intf, &mut card);

    if ret < 0 {
        return ret;
    }

    usb_set_intfdata(intf, card as *mut c_char);
    ret = init_card(caiaqdev(card));
    if ret < 0 {
        dev_err(&mut (*usb_dev).dev, b"unable to init card! (ret=%d)\n\0".as_ptr() as *const c_char, ret);
        snd_card_free(card);
        return ret;
    }

    0
}

unsafe extern "C" fn snd_disconnect(intf: *mut usb_interface) {
    let card: *mut snd_card = usb_get_intfdata(intf) as *mut snd_card;
    let dev: *mut device = &mut (*intf).usb_dev.as_ref().unwrap().dev;
    let mut cdev: *mut snd_usb_caiaqdev;

    if card.is_null() {
        return;
    }

    cdev = caiaqdev(card);
    dev_dbg(dev, b"%s(%p)\n\0".as_ptr() as *const c_char, b"snd_disconnect\0".as_ptr(), intf);

    snd_card_disconnect(card);

    #[cfg(CONFIG_SND_USB_CAIAQ_INPUT)]
    {
        snd_usb_caiaq_input_disconnect(cdev);
    }
    snd_usb_caiaq_audio_disconnect(cdev);

    usb_kill_urb(&mut (*cdev).ep1_in_urb);
    usb_kill_urb(&mut (*cdev).midi_out_urb);

    snd_card_free_when_closed(card);
}

#[repr(C)]
pub struct usb_driver {
    name: *const c_char,
    probe: extern "C" fn(*mut usb_interface, *const usb_device_id) -> i32,
    disconnect: extern "C" fn(*mut usb_interface),
    id_table: *const usb_device_id,
}

static mut SND_USB_DRIVER: usb_driver = usb_driver {
    name: MODNAME.as_ptr() as *const c_char,
    probe: snd_probe,
    disconnect: snd_disconnect,
    id_table: SND_USB_ID_TABLE.as_ptr(),
};

// module_usb_driver(snd_usb_driver);
// This would register the USB driver with the kernel module system

// Placeholder constants - these would come from external headers
const SNDRV_CARDS: usize = 32;
const SNDRV_DEFAULT_IDX: [i32; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS] = [std::ptr::null_mut(); SNDRV_CARDS];
const SNDRV_DEFAULT_ENABLE_PNP: [bool; SNDRV_CARDS] = [true; SNDRV_CARDS];

const USB_VID_NATIVEINSTRUMENTS: u16 = 0x17cc;
const USB_PID_RIGKONTROL2: u16 = 0x2710;
const USB_PID_RIGKONTROL3: u16 = 0x2720;
const USB_PID_KORECONTROLLER: u16 = 0x2711;
const USB_PID_KORECONTROLLER2: u16 = 0x2714;
const USB_PID_AK1: u16 = 0x1971;
const USB_PID_AUDIO8DJ: u16 = 0x1978;
const USB_PID_SESSIONIO: u16 = 0x1915;
const USB_PID_GUITARRIGMOBILE: u16 = 0x2801;
const USB_PID_AUDIO4DJ: u16 = 0x7410;
const USB_PID_AUDIO2DJ: u16 = 0x7411;
const USB_PID_TRAKTORKONTROLX1: u16 = 0x2305;
const USB_PID_TRAKTORKONTROLS4: u16 = 0x2360;
const USB_PID_TRAKTORAUDIO2: u16 = 0x2314;
const USB_PID_MASCHINECONTROLLER: u16 = 0x2500;

const USB_DEVICE_ID_MATCH_DEVICE: u16 = 0x0003;
const EP1_BUFSIZE: usize = 64;
const CAIAQ_USB_STR_LEN: usize = 256;

const EP1_CMD_GET_DEVICE_INFO: u8 = 0xb0;
const EP1_CMD_AUDIO_PARAMS: u8 = 0xb1;
const EP1_CMD_MIDI_READ: u8 = 0xb2;
const EP1_CMD_READ_IO: u8 = 0xb3;
const EP1_CMD_READ_ERP: u8 = 0xb4;
const EP1_CMD_READ_ANALOG: u8 = 0xb5;
const EP1_CMD_WRITE_IO: u8 = 0xb6;
const EP1_CMD_AUTO_MSG: u8 = 0xb7;

const GFP_KERNEL: u32 = 0xd0;
const GFP_ATOMIC: u32 = 0x20;
const HZ: u32 = 100;

fn USB_ID(vendor: u16, product: u16) -> u32 {
    ((vendor as u32) << 16) | (product as u32)
}

fn caiaqdev(card: *mut snd_card) -> *mut snd_usb_caiaqdev {
    unsafe {
        ((*card as *mut u8).add(std::mem::size_of::<snd_card>())) as *mut snd_usb_caiaqdev
    }
}

fn caiaqdev_to_dev(cdev: *mut snd_usb_caiaqdev) -> *mut device {
    unsafe { &mut (*cdev).chip.dev as *mut _ as *mut device }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
