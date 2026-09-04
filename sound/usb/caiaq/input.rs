// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (c) 2006,2007 Daniel Mack, Tim Ruetz
*/

// Dependencies: linux/device.h, linux/gfp.h, linux/init.h, linux/usb.h, linux/usb/input.h, sound/core.h, sound/pcm.h, device.h, input.h

use std::ptr;

// External types and constants from device.h and input.h
// struct snd_usb_caiaqdev, struct input_dev, etc. are defined in other modules.
// Key code constants like KEY_C, KEY_B, KEY_A, etc. are provided by external dependencies.

static KEYCODE_AK1: &[u16] = &[KEY_C, KEY_B, KEY_A];
static KEYCODE_RK2: &[u16] = &[KEY_1, KEY_2, KEY_3, KEY_4,
                               KEY_5, KEY_6, KEY_7];
static KEYCODE_RK3: &[u16] = &[KEY_1, KEY_2, KEY_3, KEY_4,
                               KEY_5, KEY_6, KEY_7, KEY_8, KEY_9];

static KEYCODE_KORE: &[u16] = &[
    KEY_FN_F1,      // "menu"
    KEY_FN_F7,      // "lcd backlight
    KEY_FN_F2,      // "control"
    KEY_FN_F3,      // "enter"
    KEY_FN_F4,      // "view"
    KEY_FN_F5,      // "esc"
    KEY_FN_F6,      // "sound"
    KEY_FN_F8,      // array spacer, never triggered.
    KEY_RIGHT,
    KEY_DOWN,
    KEY_UP,
    KEY_LEFT,
    KEY_SOUND,      // "listen"
    KEY_RECORD,
    KEY_PLAYPAUSE,
    KEY_STOP,
    BTN_4,          // 8 softkeys
    BTN_3,
    BTN_2,
    BTN_1,
    BTN_8,
    BTN_7,
    BTN_6,
    BTN_5,
    KEY_BRL_DOT4,   // touch sensitive knobs
    KEY_BRL_DOT3,
    KEY_BRL_DOT2,
    KEY_BRL_DOT1,
    KEY_BRL_DOT8,
    KEY_BRL_DOT7,
    KEY_BRL_DOT6,
    KEY_BRL_DOT5
];

const MASCHINE_BUTTONS: usize = 42;
const MASCHINE_PADS: usize = 16;

const fn maschine_button(x: u32) -> u32 { (x) + BTN_MISC }
const fn maschine_pad(x: u32) -> u32 { (x) + ABS_PRESSURE }

static KEYCODE_MASCHINE: &[u16] = &[
    (40 + BTN_MISC) as u16, // mute
    (39 + BTN_MISC) as u16, // solo
    (38 + BTN_MISC) as u16, // select
    (37 + BTN_MISC) as u16, // duplicate
    (36 + BTN_MISC) as u16, // navigate
    (35 + BTN_MISC) as u16, // pad mode
    (34 + BTN_MISC) as u16, // pattern
    (33 + BTN_MISC) as u16, // scene
    KEY_RESERVED,           // spacer

    (30 + BTN_MISC) as u16, // rec
    (31 + BTN_MISC) as u16, // erase
    (32 + BTN_MISC) as u16, // shift
    (28 + BTN_MISC) as u16, // grid
    (27 + BTN_MISC) as u16, // >
    (26 + BTN_MISC) as u16, // <
    (25 + BTN_MISC) as u16, // restart

    (21 + BTN_MISC) as u16, // E
    (22 + BTN_MISC) as u16, // F
    (23 + BTN_MISC) as u16, // G
    (24 + BTN_MISC) as u16, // H
    (20 + BTN_MISC) as u16, // D
    (19 + BTN_MISC) as u16, // C
    (18 + BTN_MISC) as u16, // B
    (17 + BTN_MISC) as u16, // A

    (0 + BTN_MISC) as u16,  // control
    (2 + BTN_MISC) as u16,  // browse
    (4 + BTN_MISC) as u16,  // <
    (6 + BTN_MISC) as u16,  // snap
    (7 + BTN_MISC) as u16,  // autowrite
    (5 + BTN_MISC) as u16,  // >
    (3 + BTN_MISC) as u16,  // sampling
    (1 + BTN_MISC) as u16,  // step

    (15 + BTN_MISC) as u16, // 8 softkeys
    (14 + BTN_MISC) as u16,
    (13 + BTN_MISC) as u16,
    (12 + BTN_MISC) as u16,
    (11 + BTN_MISC) as u16,
    (10 + BTN_MISC) as u16,
    (9 + BTN_MISC) as u16,
    (8 + BTN_MISC) as u16,

    (16 + BTN_MISC) as u16, // note repeat
    (29 + BTN_MISC) as u16  // play
];

const KONTROLX1_INPUTS: usize = 40;
const KONTROLS4_BUTTONS: usize = 12 * 8;
const KONTROLS4_AXIS: usize = 46;

const fn kontrols4_button(x: u32) -> u32 { (x) + BTN_MISC }
const fn kontrols4_abs(x: u32) -> u32 { (x) + ABS_HAT0X }

fn decode_erp(a: u8, b: u8) -> u32 {
    let high_peak: i32 = 268;
    let low_peak: i32 = -7;
    let range = high_peak - low_peak;
    let mid_value = (high_peak + low_peak) / 2;

    let mut weight_b = ((mid_value - a as i32).abs()) - (range / 2 - 100) / 2;

    if weight_b < 0 {
        weight_b = 0;
    }

    if weight_b > 100 {
        weight_b = 100;
    }

    let weight_a = 100 - weight_b;

    let pos_b = if (a as i32) < mid_value {
        // 0..90 and 270..360 degrees
        let deg90 = range / 2;
        let deg180 = range;
        let deg270 = deg90 + deg180;
        let deg360 = deg180 * 2;
        let mut pos = (b as i32) - low_peak + deg270;
        if pos >= deg360 {
            pos -= deg360;
        }
        pos
    } else {
        // 90..270 degrees
        let deg90 = range / 2;
        high_peak - (b as i32) + deg90
    };

    let pos_a = if (b as i32) > mid_value {
        // 0..180 degrees
        a as i32 - low_peak
    } else {
        // 180..360 degrees
        let deg180 = range;
        high_peak - (a as i32) + deg180
    };

    // interpolate both slider values, depending on weight factors
    // 0..99 x DEG360
    let deg360 = (high_peak - low_peak) * 2;
    let mut ret = pos_a * weight_a + pos_b * weight_b;

    // normalize to 0..999
    ret *= 10;
    ret /= deg360;

    if ret < 0 {
        ret += 1000;
    }

    if ret >= 1000 {
        ret -= 1000;
    }

    ret as u32
}

unsafe fn snd_caiaq_input_report_abs(cdev: *mut snd_usb_caiaqdev,
                                     axis: i32,
                                     buf: *const u8,
                                     offset: i32) {
    let val = ((*buf.add((offset * 2) as usize) as i32) << 8)
            | (*buf.add((offset * 2 + 1) as usize) as i32);
    input_report_abs((*cdev).input_dev, axis, val);
}

unsafe fn snd_caiaq_input_read_analog(cdev: *mut snd_usb_caiaqdev,
                                      buf: *const u8,
                                      len: u32) {
    let input_dev = (*cdev).input_dev;

    match (*cdev).chip.usb_id {
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_RIGKONTROL2) => {
        if len < 6 {
            return;
        }
        snd_caiaq_input_report_abs(cdev, ABS_X, buf, 2);
        snd_caiaq_input_report_abs(cdev, ABS_Y, buf, 0);
        snd_caiaq_input_report_abs(cdev, ABS_Z, buf, 1);
    },
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_RIGKONTROL3) |
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_KORECONTROLLER) |
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_KORECONTROLLER2) => {
        if len < 6 {
            return;
        }
        snd_caiaq_input_report_abs(cdev, ABS_X, buf, 0);
        snd_caiaq_input_report_abs(cdev, ABS_Y, buf, 1);
        snd_caiaq_input_report_abs(cdev, ABS_Z, buf, 2);
    },
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_TRAKTORKONTROLX1) => {
        if len < 16 {
            return;
        }
        snd_caiaq_input_report_abs(cdev, ABS_HAT0X, buf, 4);
        snd_caiaq_input_report_abs(cdev, ABS_HAT0Y, buf, 2);
        snd_caiaq_input_report_abs(cdev, ABS_HAT1X, buf, 6);
        snd_caiaq_input_report_abs(cdev, ABS_HAT1Y, buf, 1);
        snd_caiaq_input_report_abs(cdev, ABS_HAT2X, buf, 7);
        snd_caiaq_input_report_abs(cdev, ABS_HAT2Y, buf, 0);
        snd_caiaq_input_report_abs(cdev, ABS_HAT3X, buf, 5);
        snd_caiaq_input_report_abs(cdev, ABS_HAT3Y, buf, 3);
    },
    _ => {}
    }

    input_sync(input_dev);
}

unsafe fn snd_caiaq_input_read_erp(cdev: *mut snd_usb_caiaqdev,
                                   buf: *const u8,
                                   len: u32) {
    let input_dev = (*cdev).input_dev;

    match (*cdev).chip.usb_id {
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_AK1) => {
        if len < 2 {
            return;
        }
        let i = decode_erp(*buf, *buf.add(1));
        input_report_abs(input_dev, ABS_X, i as i32);
        input_sync(input_dev);
    },
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_KORECONTROLLER) |
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_KORECONTROLLER2) => {
        if len < 16 {
            return;
        }
        let mut i = decode_erp(*buf.add(7), *buf.add(5));
        input_report_abs(input_dev, ABS_HAT0X, i as i32);
        i = decode_erp(*buf.add(12), *buf.add(14));
        input_report_abs(input_dev, ABS_HAT0Y, i as i32);
        i = decode_erp(*buf.add(15), *buf.add(13));
        input_report_abs(input_dev, ABS_HAT1X, i as i32);
        i = decode_erp(*buf, *buf.add(2));
        input_report_abs(input_dev, ABS_HAT1Y, i as i32);
        i = decode_erp(*buf.add(3), *buf.add(1));
        input_report_abs(input_dev, ABS_HAT2X, i as i32);
        i = decode_erp(*buf.add(8), *buf.add(10));
        input_report_abs(input_dev, ABS_HAT2Y, i as i32);
        i = decode_erp(*buf.add(11), *buf.add(9));
        input_report_abs(input_dev, ABS_HAT3X, i as i32);
        i = decode_erp(*buf.add(4), *buf.add(6));
        input_report_abs(input_dev, ABS_HAT3Y, i as i32);
        input_sync(input_dev);
    },
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_MASCHINECONTROLLER) => {
        if len < 22 {
            return;
        }
        // 4 under the left screen
        input_report_abs(input_dev, ABS_HAT0X, decode_erp(*buf.add(21), *buf.add(20)) as i32);
        input_report_abs(input_dev, ABS_HAT0Y, decode_erp(*buf.add(15), *buf.add(14)) as i32);
        input_report_abs(input_dev, ABS_HAT1X, decode_erp(*buf.add(9), *buf.add(8)) as i32);
        input_report_abs(input_dev, ABS_HAT1Y, decode_erp(*buf.add(3), *buf.add(2)) as i32);

        // 4 under the right screen
        input_report_abs(input_dev, ABS_HAT2X, decode_erp(*buf.add(19), *buf.add(18)) as i32);
        input_report_abs(input_dev, ABS_HAT2Y, decode_erp(*buf.add(13), *buf.add(12)) as i32);
        input_report_abs(input_dev, ABS_HAT3X, decode_erp(*buf.add(7), *buf.add(6)) as i32);
        input_report_abs(input_dev, ABS_HAT3Y, decode_erp(*buf.add(1), *buf) as i32);

        // volume
        input_report_abs(input_dev, ABS_RX, decode_erp(*buf.add(17), *buf.add(16)) as i32);
        // tempo
        input_report_abs(input_dev, ABS_RY, decode_erp(*buf.add(11), *buf.add(10)) as i32);
        // swing
        input_report_abs(input_dev, ABS_RZ, decode_erp(*buf.add(5), *buf.add(4)) as i32);

        input_sync(input_dev);
    },
    _ => {}
    }
}

unsafe fn snd_caiaq_input_read_io(cdev: *mut snd_usb_caiaqdev,
                                  buf: *mut u8,
                                  len: u32) {
    let input_dev = (*cdev).input_dev;
    let keycode = (*input_dev).keycode;

    if keycode.is_null() {
        return;
    }

    if (*input_dev).id.product == USB_PID_RIGKONTROL2 {
        for i in 0..(len as usize) {
            *buf.add(i) = !(*buf.add(i));
        }
    }

    for i in 0..(*input_dev).keycodemax as usize {
        if i >= (len as usize) * 8 {
            break;
        }
        let byte = *buf.add(i / 8);
        let bit = (byte >> (i % 8)) & 1;
        input_report_key(input_dev, *keycode.add(i), bit != 0);
    }

    match (*cdev).chip.usb_id {
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_KORECONTROLLER) |
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_KORECONTROLLER2) => {
        if len < 5 {
            return;
        }
        input_report_abs((*cdev).input_dev, ABS_MISC, (255 - *buf.add(4) as i32));
    },
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_TRAKTORKONTROLX1) => {
        if len < 7 {
            return;
        }
        // rotary encoders
        input_report_abs((*cdev).input_dev, ABS_X, (*buf.add(5) & 0xf) as i32);
        input_report_abs((*cdev).input_dev, ABS_Y, ((*buf.add(5) >> 4) & 0xf) as i32);
        input_report_abs((*cdev).input_dev, ABS_Z, (*buf.add(6) & 0xf) as i32);
        input_report_abs((*cdev).input_dev, ABS_MISC, ((*buf.add(6) >> 4) & 0xf) as i32);
    },
    _ => {}
    }

    input_sync(input_dev);
}

const TKS4_MSGBLOCK_SIZE: usize = 16;

unsafe fn snd_usb_caiaq_tks4_dispatch(cdev: *mut snd_usb_caiaqdev,
                                      mut buf: *const u8,
                                      mut len: u32) {
    let dev = caiaqdev_to_dev(cdev);

    while len >= TKS4_MSGBLOCK_SIZE as u32 {
        let block_id = ((*buf as u32) << 8) | (*buf.add(1) as u32);

        match block_id {
        0 => {
            // buttons
            for i in 0..KONTROLS4_BUTTONS {
                let byte = *buf.add(4 + i / 8);
                let bit = (byte >> (i % 8)) & 1;
                input_report_key((*cdev).input_dev, kontrols4_button(i as u32) as u16, bit != 0);
            }
        },
        1 => {
            // left wheel
            input_report_abs((*cdev).input_dev, kontrols4_abs(36) as i32,
                           ((*buf.add(9) as i32) | (((*buf.add(8) as i32) & 0x3) << 8)));
            // right wheel
            input_report_abs((*cdev).input_dev, kontrols4_abs(37) as i32,
                           ((*buf.add(13) as i32) | (((*buf.add(12) as i32) & 0x3) << 8)));

            // rotary encoders
            input_report_abs((*cdev).input_dev, kontrols4_abs(38) as i32, (*buf.add(3) & 0xf) as i32);
            input_report_abs((*cdev).input_dev, kontrols4_abs(39) as i32, ((*buf.add(4) >> 4) & 0xf) as i32);
            input_report_abs((*cdev).input_dev, kontrols4_abs(40) as i32, (*buf.add(4) & 0xf) as i32);
            input_report_abs((*cdev).input_dev, kontrols4_abs(41) as i32, ((*buf.add(5) >> 4) & 0xf) as i32);
            input_report_abs((*cdev).input_dev, kontrols4_abs(42) as i32, (*buf.add(5) & 0xf) as i32);
            input_report_abs((*cdev).input_dev, kontrols4_abs(43) as i32, ((*buf.add(6) >> 4) & 0xf) as i32);
            input_report_abs((*cdev).input_dev, kontrols4_abs(44) as i32, (*buf.add(6) & 0xf) as i32);
            input_report_abs((*cdev).input_dev, kontrols4_abs(45) as i32, ((*buf.add(7) >> 4) & 0xf) as i32);
            input_report_abs((*cdev).input_dev, kontrols4_abs(46) as i32, (*buf.add(7) & 0xf) as i32);
        },
        2 => {
            // Volume Fader Channel D
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(0) as i32, buf, 1);
            // Volume Fader Channel B
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(1) as i32, buf, 2);
            // Volume Fader Channel A
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(2) as i32, buf, 3);
            // Volume Fader Channel C
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(3) as i32, buf, 4);
            // Loop Volume
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(4) as i32, buf, 6);
            // Crossfader
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(7) as i32, buf, 7);
        },
        3 => {
            // Tempo Fader R
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(6) as i32, buf, 3);
            // Tempo Fader L
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(5) as i32, buf, 4);
            // Mic Volume
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(8) as i32, buf, 6);
            // Cue Mix
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(9) as i32, buf, 7);
        },
        4 => {
            // Wheel distance sensor L
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(10) as i32, buf, 1);
            // Wheel distance sensor R
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(11) as i32, buf, 2);
            // Channel D EQ - Filter
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(12) as i32, buf, 3);
            // Channel D EQ - Low
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(13) as i32, buf, 4);
            // Channel D EQ - Mid
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(14) as i32, buf, 5);
            // Channel D EQ - Hi
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(15) as i32, buf, 6);
            // FX2 - dry/wet
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(16) as i32, buf, 7);
        },
        5 => {
            // FX2 - 1
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(17) as i32, buf, 1);
            // FX2 - 2
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(18) as i32, buf, 2);
            // FX2 - 3
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(19) as i32, buf, 3);
            // Channel B EQ - Filter
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(20) as i32, buf, 4);
            // Channel B EQ - Low
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(21) as i32, buf, 5);
            // Channel B EQ - Mid
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(22) as i32, buf, 6);
            // Channel B EQ - Hi
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(23) as i32, buf, 7);
        },
        6 => {
            // Channel A EQ - Filter
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(24) as i32, buf, 1);
            // Channel A EQ - Low
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(25) as i32, buf, 2);
            // Channel A EQ - Mid
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(26) as i32, buf, 3);
            // Channel A EQ - Hi
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(27) as i32, buf, 4);
            // Channel C EQ - Filter
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(28) as i32, buf, 5);
            // Channel C EQ - Low
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(29) as i32, buf, 6);
            // Channel C EQ - Mid
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(30) as i32, buf, 7);
        },
        7 => {
            // Channel C EQ - Hi
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(31) as i32, buf, 1);
            // FX1 - wet/dry
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(32) as i32, buf, 2);
            // FX1 - 1
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(33) as i32, buf, 3);
            // FX1 - 2
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(34) as i32, buf, 4);
            // FX1 - 3
            snd_caiaq_input_report_abs(cdev, kontrols4_abs(35) as i32, buf, 5);
        },
        _ => {
            dev_dbg(dev, "%s(): bogus block (id %d)\n\0".as_ptr() as *const i8,
                   b"snd_usb_caiaq_tks4_dispatch\0".as_ptr() as *const i8, block_id as i32);
            return;
        }
        }

        len -= TKS4_MSGBLOCK_SIZE as u32;
        buf = buf.add(TKS4_MSGBLOCK_SIZE);
    }

    input_sync((*cdev).input_dev);
}

const MASCHINE_MSGBLOCK_SIZE: usize = 2;

unsafe fn snd_usb_caiaq_maschine_dispatch(cdev: *mut snd_usb_caiaqdev,
                                          buf: *const u8,
                                          len: u32) {
    let pressure = buf as *const u16;

    for i in 0..MASCHINE_PADS {
        let pad_id = (le16_to_cpu(*pressure) >> 12) as u32;
        input_report_abs((*cdev).input_dev, (pad_id + ABS_PRESSURE) as i32,
                        (le16_to_cpu(*pressure) & 0xfff) as i32);
        let pressure = pressure.add(1);
    }

    input_sync((*cdev).input_dev);
}

unsafe extern "C" fn snd_usb_caiaq_ep4_reply_dispatch(urb: *mut urb) {
    let cdev = (*urb).context as *mut snd_usb_caiaqdev;
    let buf = (*urb).transfer_buffer as *const u8;
    let dev = &mut (*(*urb).dev).dev;
    let mut ret: i32;

    if (*urb).status != 0 || cdev.is_null() || urb != (*cdev).ep4_in_urb {
        return;
    }

    match (*cdev).chip.usb_id {
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_TRAKTORKONTROLX1) => {
        if (*urb).actual_length < 24 {
            goto_requeue;
        }

        if (*buf & 0x3) != 0 {
            snd_caiaq_input_read_io(cdev, buf.add(1) as *mut u8, 7);
        }

        if (*buf & 0x4) != 0 {
            snd_caiaq_input_read_analog(cdev, buf.add(8), 16);
        }
    },
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_TRAKTORKONTROLS4) => {
        snd_usb_caiaq_tks4_dispatch(cdev, buf, (*urb).actual_length as u32);
    },
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_MASCHINECONTROLLER) => {
        if (*urb).actual_length < (MASCHINE_PADS * MASCHINE_MSGBLOCK_SIZE) as u32 {
            goto_requeue;
        }

        snd_usb_caiaq_maschine_dispatch(cdev, buf, (*urb).actual_length as u32);
    },
    _ => {}
    }

    goto_requeue:

    (*(*cdev).ep4_in_urb).actual_length = 0;
    ret = usb_submit_urb((*cdev).ep4_in_urb, GFP_ATOMIC);
    if ret < 0 {
        dev_err(dev, "unable to submit urb. OOM!?\n\0".as_ptr() as *const i8);
    }
}

unsafe fn snd_usb_caiaq_input_open(idev: *mut input_dev) -> i32 {
    let cdev = input_get_drvdata(idev) as *mut snd_usb_caiaqdev;

    if cdev.is_null() {
        return -EINVAL;
    }

    match (*cdev).chip.usb_id {
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_TRAKTORKONTROLX1) |
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_TRAKTORKONTROLS4) |
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_MASCHINECONTROLLER) => {
        if usb_submit_urb((*cdev).ep4_in_urb, GFP_KERNEL) != 0 {
            return -EIO;
        }
    },
    _ => {}
    }

    0
}

unsafe fn snd_usb_caiaq_input_close(idev: *mut input_dev) {
    let cdev = input_get_drvdata(idev) as *mut snd_usb_caiaqdev;

    if cdev.is_null() {
        return;
    }

    match (*cdev).chip.usb_id {
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_TRAKTORKONTROLX1) |
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_TRAKTORKONTROLS4) |
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_MASCHINECONTROLLER) => {
        usb_kill_urb((*cdev).ep4_in_urb);
    },
    _ => {}
    }
}

pub unsafe fn snd_usb_caiaq_input_dispatch(cdev: *mut snd_usb_caiaqdev,
                                          buf: *mut u8,
                                          len: u32) {
    if (*cdev).input_dev.is_null() || len < 1 {
        return;
    }

    match *buf {
    EP1_CMD_READ_ANALOG => {
        snd_caiaq_input_read_analog(cdev, buf.add(1), len - 1);
    },
    EP1_CMD_READ_ERP => {
        snd_caiaq_input_read_erp(cdev, buf.add(1), len - 1);
    },
    EP1_CMD_READ_IO => {
        snd_caiaq_input_read_io(cdev, buf.add(1), len - 1);
    },
    _ => {}
    }
}

pub unsafe fn snd_usb_caiaq_input_init(cdev: *mut snd_usb_caiaqdev) -> i32 {
    let usb_dev = (*cdev).chip.dev;
    let input = input_allocate_device();
    if input.is_null() {
        return -ENOMEM;
    }

    usb_make_path(usb_dev, (*cdev).phys.as_mut_ptr(), (*cdev).phys.len());
    strlcat((*cdev).phys.as_mut_ptr(), "/input0\0".as_ptr() as *const i8, (*cdev).phys.len());

    (*input).name = (*cdev).product_name;
    (*input).phys = (*cdev).phys.as_ptr();
    usb_to_input_id(usb_dev, &mut (*input).id);
    (*input).dev.parent = &(*usb_dev).dev;

    input_set_drvdata(input, cdev as *mut core::ffi::c_void);

    let mut ret: i32 = 0;

    match (*cdev).chip.usb_id {
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_RIGKONTROL2) => {
        (*input).evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_ABS);
        (*input).absbit[0] = BIT_MASK(ABS_X) | BIT_MASK(ABS_Y) |
            BIT_MASK(ABS_Z);
        // BUILD_BUG_ON(sizeof(cdev->keycode) < sizeof(keycode_rk2));
        ptr::copy_nonoverlapping(KEYCODE_RK2.as_ptr(), (*cdev).keycode.as_mut_ptr(), KEYCODE_RK2.len());
        (*input).keycodemax = KEYCODE_RK2.len() as u32;
        input_set_abs_params(input, ABS_X, 0, 4096, 0, 10);
        input_set_abs_params(input, ABS_Y, 0, 4096, 0, 10);
        input_set_abs_params(input, ABS_Z, 0, 4096, 0, 10);
        snd_usb_caiaq_set_auto_msg(cdev, 1, 10, 0);
    },
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_RIGKONTROL3) => {
        (*input).evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_ABS);
        (*input).absbit[0] = BIT_MASK(ABS_X) | BIT_MASK(ABS_Y) |
            BIT_MASK(ABS_Z);
        // BUILD_BUG_ON(sizeof(cdev->keycode) < sizeof(keycode_rk3));
        ptr::copy_nonoverlapping(KEYCODE_RK3.as_ptr(), (*cdev).keycode.as_mut_ptr(), KEYCODE_RK3.len());
        (*input).keycodemax = KEYCODE_RK3.len() as u32;
        input_set_abs_params(input, ABS_X, 0, 1024, 0, 10);
        input_set_abs_params(input, ABS_Y, 0, 1024, 0, 10);
        input_set_abs_params(input, ABS_Z, 0, 1024, 0, 10);
        snd_usb_caiaq_set_auto_msg(cdev, 1, 10, 0);
    },
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_AK1) => {
        (*input).evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_ABS);
        (*input).absbit[0] = BIT_MASK(ABS_X);
        // BUILD_BUG_ON(sizeof(cdev->keycode) < sizeof(keycode_ak1));
        ptr::copy_nonoverlapping(KEYCODE_AK1.as_ptr(), (*cdev).keycode.as_mut_ptr(), KEYCODE_AK1.len());
        (*input).keycodemax = KEYCODE_AK1.len() as u32;
        input_set_abs_params(input, ABS_X, 0, 999, 0, 10);
        snd_usb_caiaq_set_auto_msg(cdev, 1, 0, 5);
    },
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_KORECONTROLLER) |
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_KORECONTROLLER2) => {
        (*input).evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_ABS);
        (*input).absbit[0] = BIT_MASK(ABS_HAT0X) | BIT_MASK(ABS_HAT0Y) |
                   BIT_MASK(ABS_HAT1X) | BIT_MASK(ABS_HAT1Y) |
                   BIT_MASK(ABS_HAT2X) | BIT_MASK(ABS_HAT2Y) |
                   BIT_MASK(ABS_HAT3X) | BIT_MASK(ABS_HAT3Y) |
                   BIT_MASK(ABS_X) | BIT_MASK(ABS_Y) |
                   BIT_MASK(ABS_Z);
        (*input).absbit[BIT_WORD(ABS_MISC)] |= BIT_MASK(ABS_MISC);
        // BUILD_BUG_ON(sizeof(cdev->keycode) < sizeof(keycode_kore));
        ptr::copy_nonoverlapping(KEYCODE_KORE.as_ptr(), (*cdev).keycode.as_mut_ptr(), KEYCODE_KORE.len());
        (*input).keycodemax = KEYCODE_KORE.len() as u32;
        input_set_abs_params(input, ABS_HAT0X, 0, 999, 0, 10);
        input_set_abs_params(input, ABS_HAT0Y, 0, 999, 0, 10);
        input_set_abs_params(input, ABS_HAT1X, 0, 999, 0, 10);
        input_set_abs_params(input, ABS_HAT1Y, 0, 999, 0, 10);
        input_set_abs_params(input, ABS_HAT2X, 0, 999, 0, 10);
        input_set_abs_params(input, ABS_HAT2Y, 0, 999, 0, 10);
        input_set_abs_params(input, ABS_HAT3X, 0, 999, 0, 10);
        input_set_abs_params(input, ABS_HAT3Y, 0, 999, 0, 10);
        input_set_abs_params(input, ABS_X, 0, 4096, 0, 10);
        input_set_abs_params(input, ABS_Y, 0, 4096, 0, 10);
        input_set_abs_params(input, ABS_Z, 0, 4096, 0, 10);
        input_set_abs_params(input, ABS_MISC, 0, 255, 0, 1);
        snd_usb_caiaq_set_auto_msg(cdev, 1, 10, 5);
    },
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_TRAKTORKONTROLX1) => {
        (*input).evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_ABS);
        (*input).absbit[0] = BIT_MASK(ABS_HAT0X) | BIT_MASK(ABS_HAT0Y) |
                   BIT_MASK(ABS_HAT1X) | BIT_MASK(ABS_HAT1Y) |
                   BIT_MASK(ABS_HAT2X) | BIT_MASK(ABS_HAT2Y) |
                   BIT_MASK(ABS_HAT3X) | BIT_MASK(ABS_HAT3Y) |
                   BIT_MASK(ABS_X) | BIT_MASK(ABS_Y) |
                   BIT_MASK(ABS_Z);
        (*input).absbit[BIT_WORD(ABS_MISC)] |= BIT_MASK(ABS_MISC);
        // BUILD_BUG_ON(sizeof(cdev->keycode) < KONTROLX1_INPUTS);
        for i in 0..KONTROLX1_INPUTS {
            (*cdev).keycode[i] = (BTN_MISC + i as u32) as u16;
        }
        (*input).keycodemax = KONTROLX1_INPUTS as u32;

        // analog potentiometers
        input_set_abs_params(input, ABS_HAT0X, 0, 4096, 0, 10);
        input_set_abs_params(input, ABS_HAT0Y, 0, 4096, 0, 10);
        input_set_abs_params(input, ABS_HAT1X, 0, 4096, 0, 10);
        input_set_abs_params(input, ABS_HAT1Y, 0, 4096, 0, 10);
        input_set_abs_params(input, ABS_HAT2X, 0, 4096, 0, 10);
        input_set_abs_params(input, ABS_HAT2Y, 0, 4096, 0, 10);
        input_set_abs_params(input, ABS_HAT3X, 0, 4096, 0, 10);
        input_set_abs_params(input, ABS_HAT3Y, 0, 4096, 0, 10);

        // rotary encoders
        input_set_abs_params(input, ABS_X, 0, 0xf, 0, 1);
        input_set_abs_params(input, ABS_Y, 0, 0xf, 0, 1);
        input_set_abs_params(input, ABS_Z, 0, 0xf, 0, 1);
        input_set_abs_params(input, ABS_MISC, 0, 0xf, 0, 1);

        (*cdev).ep4_in_urb = usb_alloc_urb(0, GFP_KERNEL);
        if (*cdev).ep4_in_urb.is_null() {
            ret = -ENOMEM;
            goto exit_free_idev;
        }

        usb_fill_bulk_urb((*cdev).ep4_in_urb, usb_dev,
                  usb_rcvbulkpipe(usb_dev, 0x4),
                  (*cdev).ep4_in_buf.as_mut_ptr() as *mut core::ffi::c_void, EP4_BUFSIZE,
                  snd_usb_caiaq_ep4_reply_dispatch, cdev as *mut core::ffi::c_void);
        ret = usb_urb_ep_type_check((*cdev).ep4_in_urb);
        if ret < 0 {
            goto exit_free_idev;
        }

        snd_usb_caiaq_set_auto_msg(cdev, 1, 10, 5);
    },
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_TRAKTORKONTROLS4) => {
        (*input).evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_ABS);
        // BUILD_BUG_ON(sizeof(cdev->keycode) < KONTROLS4_BUTTONS);
        for i in 0..KONTROLS4_BUTTONS {
            (*cdev).keycode[i] = kontrols4_button(i as u32) as u16;
        }
        (*input).keycodemax = KONTROLS4_BUTTONS as u32;

        for i in 0..KONTROLS4_AXIS {
            let axis = kontrols4_abs(i) as i32;
            (*input).absbit[BIT_WORD(axis) as usize] |= BIT_MASK(axis);
        }

        // 36 analog potentiometers and faders
        for i in 0..36 {
            input_set_abs_params(input, kontrols4_abs(i) as i32, 0, 0xfff, 0, 10);
        }

        // 2 encoder wheels
        input_set_abs_params(input, kontrols4_abs(36) as i32, 0, 0x3ff, 0, 1);
        input_set_abs_params(input, kontrols4_abs(37) as i32, 0, 0x3ff, 0, 1);

        // 9 rotary encoders
        for i in 0..9 {
            input_set_abs_params(input, kontrols4_abs(38 + i) as i32, 0, 0xf, 0, 1);
        }

        (*cdev).ep4_in_urb = usb_alloc_urb(0, GFP_KERNEL);
        if (*cdev).ep4_in_urb.is_null() {
            ret = -ENOMEM;
            goto exit_free_idev;
        }

        usb_fill_bulk_urb((*cdev).ep4_in_urb, usb_dev,
                  usb_rcvbulkpipe(usb_dev, 0x4),
                  (*cdev).ep4_in_buf.as_mut_ptr() as *mut core::ffi::c_void, EP4_BUFSIZE,
                  snd_usb_caiaq_ep4_reply_dispatch, cdev as *mut core::ffi::c_void);
        ret = usb_urb_ep_type_check((*cdev).ep4_in_urb);
        if ret < 0 {
            goto exit_free_idev;
        }

        snd_usb_caiaq_set_auto_msg(cdev, 1, 10, 5);
    },
    USB_ID(USB_VID_NATIVEINSTRUMENTS, USB_PID_MASCHINECONTROLLER) => {
        (*input).evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_ABS);
        (*input).absbit[0] = BIT_MASK(ABS_HAT0X) | BIT_MASK(ABS_HAT0Y) |
            BIT_MASK(ABS_HAT1X) | BIT_MASK(ABS_HAT1Y) |
            BIT_MASK(ABS_HAT2X) | BIT_MASK(ABS_HAT2Y) |
            BIT_MASK(ABS_HAT3X) | BIT_MASK(ABS_HAT3Y) |
            BIT_MASK(ABS_RX) | BIT_MASK(ABS_RY) |
            BIT_MASK(ABS_RZ);

        // BUILD_BUG_ON(sizeof(cdev->keycode) < sizeof(keycode_maschine));
        ptr::copy_nonoverlapping(KEYCODE_MASCHINE.as_ptr(), (*cdev).keycode.as_mut_ptr(), KEYCODE_MASCHINE.len());
        (*input).keycodemax = KEYCODE_MASCHINE.len() as u32;

        for i in 0..MASCHINE_PADS {
            (*input).absbit[0] |= (i as u32 + ABS_PRESSURE);
            input_set_abs_params(input, (i as u32 + ABS_PRESSURE) as i32, 0, 0xfff, 5, 10);
        }

        input_set_abs_params(input, ABS_HAT0X, 0, 999, 0, 10);
        input_set_abs_params(input, ABS_HAT0Y, 0, 999, 0, 10);
        input_set_abs_params(input, ABS_HAT1X, 0, 999, 0, 10);
        input_set_abs_params(input, ABS_HAT1Y, 0, 999, 0, 10);
        input_set_abs_params(input, ABS_HAT2X, 0, 999, 0, 10);
        input_set_abs_params(input, ABS_HAT2Y, 0, 999, 0, 10);
        input_set_abs_params(input, ABS_HAT3X, 0, 999, 0, 10);
        input_set_abs_params(input, ABS_HAT3Y, 0, 999, 0, 10);
        input_set_abs_params(input, ABS_RX, 0, 999, 0, 10);
        input_set_abs_params(input, ABS_RY, 0, 999, 0, 10);
        input_set_abs_params(input, ABS_RZ, 0, 999, 0, 10);

        (*cdev).ep4_in_urb = usb_alloc_urb(0, GFP_KERNEL);
        if (*cdev).ep4_in_urb.is_null() {
            ret = -ENOMEM;
            goto exit_free_idev;
        }

        usb_fill_bulk_urb((*cdev).ep4_in_urb, usb_dev,
                  usb_rcvbulkpipe(usb_dev, 0x4),
                  (*cdev).ep4_in_buf.as_mut_ptr() as *mut core::ffi::c_void, EP4_BUFSIZE,
                  snd_usb_caiaq_ep4_reply_dispatch, cdev as *mut core::ffi::c_void);
        ret = usb_urb_ep_type_check((*cdev).ep4_in_urb);
        if ret < 0 {
            goto exit_free_idev;
        }

        snd_usb_caiaq_set_auto_msg(cdev, 1, 10, 5);
    },
    _ => {
        ret = -ENODEV;
        goto exit_free_idev;
    }
    }

    (*input).open = Some(snd_usb_caiaq_input_open);
    (*input).close = Some(snd_usb_caiaq_input_close);
    (*input).keycode = (*cdev).keycode.as_mut_ptr() as *mut core::ffi::c_void;
    (*input).keycodesize = std::mem::size_of::<u16>() as u32;
    for i in 0..(*input).keycodemax as usize {
        __set_bit((*cdev).keycode[i] as usize, (*input).keybit.as_mut_ptr());
    }

    (*cdev).input_dev = input;

    ret = input_register_device(input);
    if ret < 0 {
        goto exit_free_idev;
    }

    return 0;

    exit_free_idev:
    input_free_device(input);
    (*cdev).input_dev = ptr::null_mut();
    return ret;
}

pub unsafe fn snd_usb_caiaq_input_disconnect(cdev: *mut snd_usb_caiaqdev) {
    if cdev.is_null() || (*cdev).input_dev.is_null() {
        return;
    }

    usb_kill_urb((*cdev).ep4_in_urb);
    input_unregister_device((*cdev).input_dev);
}

pub unsafe fn snd_usb_caiaq_input_free(cdev: *mut snd_usb_caiaqdev) {
    if cdev.is_null() || (*cdev).input_dev.is_null() {
        return;
    }

    usb_free_urb((*cdev).ep4_in_urb);
    (*cdev).ep4_in_urb = ptr::null_mut();
    (*cdev).input_dev = ptr::null_mut();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
