// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (c) 2007 Daniel Mack
 *   friendly supported by NI.
 */

// Dependencies: linux/device.h, linux/init.h, linux/usb.h, sound/control.h,
// sound/core.h, sound/pcm.h, device.h, control.h

const CNT_INTVAL: u32 = 0x10000;
const MASCHINE_BANK_SIZE: usize = 32;

// External types and functions from ALSA/USB kernel modules
#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub count: u32,
    pub _type: u32,
    pub value: snd_ctl_elem_value,
    // ... other fields omitted
}

#[repr(C)]
pub union snd_ctl_elem_value {
    pub integer: snd_ctl_int_value,
}

#[repr(C)]
pub struct snd_ctl_int_value {
    pub value: [i32; 128],
}

#[repr(C)]
pub struct snd_usb_audio {
    pub card: *mut snd_card,
    // ... other fields
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_usb_caiaqdev {
    pub chip: snd_usb_caiaq_chip,
    pub control_state: [u8; 256],
    pub ep8_out_buf: [u8; 256],
    // ... other fields
}

#[repr(C)]
pub struct snd_usb_caiaq_chip {
    pub usb_id: u32,
    pub dev: *mut usb_device,
    pub card: *mut snd_card,
}

#[repr(C)]
pub struct usb_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: u32,
    pub access: u32,
    pub index: u32,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> i32>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> i32>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> i32>,
    pub name: *const i8,
    pub private_value: u32,
}

#[repr(C)]
pub struct caiaq_controller {
    pub name: *const i8,
    pub index: i32,
}

// SNDRV_CTL_ELEM_* constants from sound/control.h
const SNDRV_CTL_ELEM_IFACE_HWDEP: u32 = 4;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: u32 = 3;
const SNDRV_CTL_ELEM_TYPE_INTEGER: u32 = 1;
const SNDRV_CTL_ELEM_TYPE_BOOLEAN: u32 = 2;

// USB vendor/product IDs from device.h
const USB_VID_NATIVEINSTRUMENTS: u32 = 0x17cc;
const USB_PID_AK1: u32 = 0x0601;
const USB_PID_RIGKONTROL2: u32 = 0x0613;
const USB_PID_RIGKONTROL3: u32 = 0x1210;
const USB_PID_KORECONTROLLER: u32 = 0x0701;
const USB_PID_KORECONTROLLER2: u32 = 0x1080;
const USB_PID_AUDIO8DJ: u32 = 0x1978;
const USB_PID_AUDIO4DJ: u32 = 0x1971;
const USB_PID_TRAKTORKONTROLX1: u32 = 0x17d0;
const USB_PID_TRAKTORKONTROLS4: u32 = 0x2ae4;
const USB_PID_MASCHINECONTROLLER: u32 = 0x1f18;

fn usb_id(vid: u32, pid: u32) -> u32 {
    (vid << 16) | pid
}

// Endpoint commands from control.h
const EP1_CMD_DIMM_LEDS: u8 = 0x00;
const EP1_CMD_WRITE_IO: u8 = 0x03;

extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_usb_audio;
    fn caiaqdev(card: *mut snd_card) -> *mut snd_usb_caiaqdev;
    fn snd_ctl_new1(
        template: *const snd_kcontrol_new,
        private_data: *mut std::ffi::c_void,
    ) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> i32;
    fn usb_sndbulkpipe(dev: *mut usb_device, endpoint: u32) -> u32;
    fn usb_bulk_msg(
        dev: *mut usb_device,
        pipe: u32,
        data: *mut u8,
        len: usize,
        actual_len: *mut i32,
        timeout: u32,
    ) -> i32;
    fn snd_usb_caiaq_send_command_bank(
        dev: *mut snd_usb_caiaqdev,
        cmd: u8,
        bank: u8,
        buf: *const u8,
        len: usize,
    ) -> i32;
    fn snd_usb_caiaq_send_command(
        dev: *mut snd_usb_caiaqdev,
        cmd: u8,
        buf: *const u8,
        len: usize,
    ) -> i32;
}

unsafe extern "C" fn control_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    let chip = snd_kcontrol_chip(kcontrol);
    let cdev = caiaqdev((*chip).card);
    let mut pos = (*kcontrol).private_value as i32;
    let is_intval = pos & (CNT_INTVAL as i32);
    let mut maxval = 63;

    (*uinfo).count = 1;
    pos &= !(CNT_INTVAL as i32);

    match (*cdev).chip.usb_id {
        id if id == usb_id(USB_VID_NATIVEINSTRUMENTS, USB_PID_AUDIO8DJ)
            || id == usb_id(USB_VID_NATIVEINSTRUMENTS, USB_PID_AUDIO4DJ) =>
        {
            if pos == 0 {
                (*uinfo)._type = SNDRV_CTL_ELEM_TYPE_INTEGER;
                (*uinfo).value.integer.value[0] = 0;
                (*uinfo).value.integer.value[1] = 2;
                return 0;
            }
        }
        id if id == usb_id(USB_VID_NATIVEINSTRUMENTS, USB_PID_TRAKTORKONTROLX1) => {
            maxval = 127;
        }
        id if id == usb_id(USB_VID_NATIVEINSTRUMENTS, USB_PID_TRAKTORKONTROLS4) => {
            maxval = 31;
        }
        _ => {}
    }

    if is_intval != 0 {
        (*uinfo)._type = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).value.integer.value[0] = 0;
        (*uinfo).value.integer.value[1] = maxval;
    } else {
        (*uinfo)._type = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
        (*uinfo).value.integer.value[0] = 0;
        (*uinfo).value.integer.value[1] = 1;
    }

    0
}

unsafe extern "C" fn control_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let chip = snd_kcontrol_chip(kcontrol);
    let cdev = caiaqdev((*chip).card);
    let pos = (*kcontrol).private_value as i32;

    if pos & (CNT_INTVAL as i32) != 0 {
        (*ucontrol).integer.value[0] =
            (*cdev).control_state[(pos & !(CNT_INTVAL as i32)) as usize] as i32;
    } else {
        let idx = (pos / 8) as usize;
        let bit_pos = (pos % 8) as u32;
        (*ucontrol).integer.value[0] = if ((*cdev).control_state[idx] & (1 << bit_pos)) != 0 {
            1
        } else {
            0
        };
    }

    0
}

unsafe extern "C" fn control_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let chip = snd_kcontrol_chip(kcontrol);
    let cdev = caiaqdev((*chip).card);
    let pos = (*kcontrol).private_value as i32;
    let v = (*ucontrol).integer.value[0];
    let mut ret: i32;
    let mut cmd: u8;

    match (*cdev).chip.usb_id {
        id if id == usb_id(USB_VID_NATIVEINSTRUMENTS, USB_PID_MASCHINECONTROLLER)
            || id == usb_id(USB_VID_NATIVEINSTRUMENTS, USB_PID_TRAKTORKONTROLX1)
            || id == usb_id(USB_VID_NATIVEINSTRUMENTS, USB_PID_KORECONTROLLER2)
            || id == usb_id(USB_VID_NATIVEINSTRUMENTS, USB_PID_KORECONTROLLER) =>
        {
            cmd = EP1_CMD_DIMM_LEDS;
        }
        _ => {
            cmd = EP1_CMD_WRITE_IO;
        }
    }

    if pos & (CNT_INTVAL as i32) != 0 {
        let i = (pos & !(CNT_INTVAL as i32)) as usize;
        let old = (*cdev).control_state[i];

        if old == v as u8 {
            return 0;
        }

        (*cdev).control_state[i] = v as u8;

        if (*cdev).chip.usb_id == usb_id(USB_VID_NATIVEINSTRUMENTS, USB_PID_TRAKTORKONTROLS4) {
            let mut actual_len: i32 = 0;

            (*cdev).ep8_out_buf[0] = i as u8;
            (*cdev).ep8_out_buf[1] = v as u8;

            ret = usb_bulk_msg(
                (*cdev).chip.dev,
                usb_sndbulkpipe((*cdev).chip.dev, 8),
                (*cdev).ep8_out_buf.as_mut_ptr(),
                std::mem::size_of_val(&(*cdev).ep8_out_buf),
                &mut actual_len,
                200,
            );
        } else if (*cdev).chip.usb_id
            == usb_id(USB_VID_NATIVEINSTRUMENTS, USB_PID_MASCHINECONTROLLER)
        {
            let mut bank = 0u8;
            let mut offset = 0usize;

            if i >= MASCHINE_BANK_SIZE {
                bank = 0x1e;
                offset = MASCHINE_BANK_SIZE;
            }

            ret = snd_usb_caiaq_send_command_bank(
                cdev,
                cmd,
                bank,
                (*cdev).control_state.as_ptr().add(offset),
                MASCHINE_BANK_SIZE,
            );
        } else {
            ret = snd_usb_caiaq_send_command(
                cdev,
                cmd,
                (*cdev).control_state.as_ptr(),
                std::mem::size_of_val(&(*cdev).control_state),
            );
        }

        if ret < 0 {
            (*cdev).control_state[i] = old;
            return ret;
        }
    } else {
        let idx = (pos / 8) as usize;
        let mask = (1u8) << (pos % 8);
        let old = (*cdev).control_state[idx];
        let val = if v != 0 {
            old | mask
        } else {
            old & !mask
        };

        if old == val {
            return 0;
        }

        (*cdev).control_state[idx] = val;
        ret = snd_usb_caiaq_send_command(
            cdev,
            cmd,
            (*cdev).control_state.as_ptr(),
            std::mem::size_of_val(&(*cdev).control_state),
        );
        if ret < 0 {
            (*cdev).control_state[idx] = old;
            return ret;
        }
    }

    1
}

static mut kcontrol_template: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_HWDEP,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    index: 0,
    info: Some(control_info),
    get: Some(control_get),
    put: Some(control_put),
    name: std::ptr::null(),
    private_value: 0,
};

static ak1_controller: &[caiaq_controller] = &[
    caiaq_controller {
        name: b"LED left\0".as_ptr() as *const i8,
        index: 2,
    },
    caiaq_controller {
        name: b"LED middle\0".as_ptr() as *const i8,
        index: 1,
    },
    caiaq_controller {
        name: b"LED right\0".as_ptr() as *const i8,
        index: 0,
    },
    caiaq_controller {
        name: b"LED ring\0".as_ptr() as *const i8,
        index: 3,
    },
];

static rk2_controller: &[caiaq_controller] = &[
    caiaq_controller {
        name: b"LED 1\0".as_ptr() as *const i8,
        index: 5,
    },
    caiaq_controller {
        name: b"LED 2\0".as_ptr() as *const i8,
        index: 4,
    },
    caiaq_controller {
        name: b"LED 3\0".as_ptr() as *const i8,
        index: 3,
    },
    caiaq_controller {
        name: b"LED 4\0".as_ptr() as *const i8,
        index: 2,
    },
    caiaq_controller {
        name: b"LED 5\0".as_ptr() as *const i8,
        index: 1,
    },
    caiaq_controller {
        name: b"LED 6\0".as_ptr() as *const i8,
        index: 0,
    },
    caiaq_controller {
        name: b"LED pedal\0".as_ptr() as *const i8,
        index: 6,
    },
    caiaq_controller {
        name: b"LED 7seg_1b\0".as_ptr() as *const i8,
        index: 8,
    },
    caiaq_controller {
        name: b"LED 7seg_1c\0".as_ptr() as *const i8,
        index: 9,
    },
    caiaq_controller {
        name: b"LED 7seg_2a\0".as_ptr() as *const i8,
        index: 10,
    },
    caiaq_controller {
        name: b"LED 7seg_2b\0".as_ptr() as *const i8,
        index: 11,
    },
    caiaq_controller {
        name: b"LED 7seg_2c\0".as_ptr() as *const i8,
        index: 12,
    },
    caiaq_controller {
        name: b"LED 7seg_2d\0".as_ptr() as *const i8,
        index: 13,
    },
    caiaq_controller {
        name: b"LED 7seg_2e\0".as_ptr() as *const i8,
        index: 14,
    },
    caiaq_controller {
        name: b"LED 7seg_2f\0".as_ptr() as *const i8,
        index: 15,
    },
    caiaq_controller {
        name: b"LED 7seg_2g\0".as_ptr() as *const i8,
        index: 16,
    },
    caiaq_controller {
        name: b"LED 7seg_3a\0".as_ptr() as *const i8,
        index: 17,
    },
    caiaq_controller {
        name: b"LED 7seg_3b\0".as_ptr() as *const i8,
        index: 18,
    },
    caiaq_controller {
        name: b"LED 7seg_3c\0".as_ptr() as *const i8,
        index: 19,
    },
    caiaq_controller {
        name: b"LED 7seg_3d\0".as_ptr() as *const i8,
        index: 20,
    },
    caiaq_controller {
        name: b"LED 7seg_3e\0".as_ptr() as *const i8,
        index: 21,
    },
    caiaq_controller {
        name: b"LED 7seg_3f\0".as_ptr() as *const i8,
        index: 22,
    },
    caiaq_controller {
        name: b"LED 7seg_3g\0".as_ptr() as *const i8,
        index: 23,
    },
];

static rk3_controller: &[caiaq_controller] = &[
    caiaq_controller {
        name: b"LED 7seg_1a\0".as_ptr() as *const i8,
        index: 0 + 0,
    },
    caiaq_controller {
        name: b"LED 7seg_1b\0".as_ptr() as *const i8,
        index: 0 + 1,
    },
    caiaq_controller {
        name: b"LED 7seg_1c\0".as_ptr() as *const i8,
        index: 0 + 2,
    },
    caiaq_controller {
        name: b"LED 7seg_1d\0".as_ptr() as *const i8,
        index: 0 + 3,
    },
    caiaq_controller {
        name: b"LED 7seg_1e\0".as_ptr() as *const i8,
        index: 0 + 4,
    },
    caiaq_controller {
        name: b"LED 7seg_1f\0".as_ptr() as *const i8,
        index: 0 + 5,
    },
    caiaq_controller {
        name: b"LED 7seg_1g\0".as_ptr() as *const i8,
        index: 0 + 6,
    },
    caiaq_controller {
        name: b"LED 7seg_1p\0".as_ptr() as *const i8,
        index: 0 + 7,
    },
    caiaq_controller {
        name: b"LED 7seg_2a\0".as_ptr() as *const i8,
        index: 8 + 0,
    },
    caiaq_controller {
        name: b"LED 7seg_2b\0".as_ptr() as *const i8,
        index: 8 + 1,
    },
    caiaq_controller {
        name: b"LED 7seg_2c\0".as_ptr() as *const i8,
        index: 8 + 2,
    },
    caiaq_controller {
        name: b"LED 7seg_2d\0".as_ptr() as *const i8,
        index: 8 + 3,
    },
    caiaq_controller {
        name: b"LED 7seg_2e\0".as_ptr() as *const i8,
        index: 8 + 4,
    },
    caiaq_controller {
        name: b"LED 7seg_2f\0".as_ptr() as *const i8,
        index: 8 + 5,
    },
    caiaq_controller {
        name: b"LED 7seg_2g\0".as_ptr() as *const i8,
        index: 8 + 6,
    },
    caiaq_controller {
        name: b"LED 7seg_2p\0".as_ptr() as *const i8,
        index: 8 + 7,
    },
    caiaq_controller {
        name: b"LED 7seg_3a\0".as_ptr() as *const i8,
        index: 16 + 0,
    },
    caiaq_controller {
        name: b"LED 7seg_3b\0".as_ptr() as *const i8,
        index: 16 + 1,
    },
    caiaq_controller {
        name: b"LED 7seg_3c\0".as_ptr() as *const i8,
        index: 16 + 2,
    },
    caiaq_controller {
        name: b"LED 7seg_3d\0".as_ptr() as *const i8,
        index: 16 + 3,
    },
    caiaq_controller {
        name: b"LED 7seg_3e\0".as_ptr() as *const i8,
        index: 16 + 4,
    },
    caiaq_controller {
        name: b"LED 7seg_3f\0".as_ptr() as *const i8,
        index: 16 + 5,
    },
    caiaq_controller {
        name: b"LED 7seg_3g\0".as_ptr() as *const i8,
        index: 16 + 6,
    },
    caiaq_controller {
        name: b"LED 7seg_3p\0".as_ptr() as *const i8,
        index: 16 + 7,
    },
    caiaq_controller {
        name: b"LED 7seg_4a\0".as_ptr() as *const i8,
        index: 24 + 0,
    },
    caiaq_controller {
        name: b"LED 7seg_4b\0".as_ptr() as *const i8,
        index: 24 + 1,
    },
    caiaq_controller {
        name: b"LED 7seg_4c\0".as_ptr() as *const i8,
        index: 24 + 2,
    },
    caiaq_controller {
        name: b"LED 7seg_4d\0".as_ptr() as *const i8,
        index: 24 + 3,
    },
    caiaq_controller {
        name: b"LED 7seg_4e\0".as_ptr() as *const i8,
        index: 24 + 4,
    },
    caiaq_controller {
        name: b"LED 7seg_4f\0".as_ptr() as *const i8,
        index: 24 + 5,
    },
    caiaq_controller {
        name: b"LED 7seg_4g\0".as_ptr() as *const i8,
        index: 24 + 6,
    },
    caiaq_controller {
        name: b"LED 7seg_4p\0".as_ptr() as *const i8,
        index: 24 + 7,
    },
    caiaq_controller {
        name: b"LED 1\0".as_ptr() as *const i8,
        index: 32 + 0,
    },
    caiaq_controller {
        name: b"LED 2\0".as_ptr() as *const i8,
        index: 32 + 1,
    },
    caiaq_controller {
        name: b"LED 3\0".as_ptr() as *const i8,
        index: 32 + 2,
    },
    caiaq_controller {
        name: b"LED 4\0".as_ptr() as *const i8,
        index: 32 + 3,
    },
    caiaq_controller {
        name: b"LED 5\0".as_ptr() as *const i8,
        index: 32 + 4,
    },
    caiaq_controller {
        name: b"LED 6\0".as_ptr() as *const i8,
        index: 32 + 5,
    },
    caiaq_controller {
        name: b"LED 7\0".as_ptr() as *const i8,
        index: 32 + 6,
    },
    caiaq_controller {
        name: b"LED 8\0".as_ptr() as *const i8,
        index: 32 + 7,
    },
    caiaq_controller {
        name: b"LED pedal\0".as_ptr() as *const i8,
        index: 32 + 8,
    },
];

static kore_controller: &[caiaq_controller] = &[
    caiaq_controller {
        name: b"LED F1\0".as_ptr() as *const i8,
        index: (8 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED F2\0".as_ptr() as *const i8,
        index: (12 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED F3\0".as_ptr() as *const i8,
        index: (0 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED F4\0".as_ptr() as *const i8,
        index: (4 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED F5\0".as_ptr() as *const i8,
        index: (11 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED F6\0".as_ptr() as *const i8,
        index: (15 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED F7\0".as_ptr() as *const i8,
        index: (3 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED F8\0".as_ptr() as *const i8,
        index: (7 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED touch1\0".as_ptr() as *const i8,
        index: (10 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED touch2\0".as_ptr() as *const i8,
        index: (14 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED touch3\0".as_ptr() as *const i8,
        index: (2 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED touch4\0".as_ptr() as *const i8,
        index: (6 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED touch5\0".as_ptr() as *const i8,
        index: (9 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED touch6\0".as_ptr() as *const i8,
        index: (13 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED touch7\0".as_ptr() as *const i8,
        index: (1 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED touch8\0".as_ptr() as *const i8,
        index: (5 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED left\0".as_ptr() as *const i8,
        index: (18 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED right\0".as_ptr() as *const i8,
        index: (22 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED up\0".as_ptr() as *const i8,
        index: (16 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED down\0".as_ptr() as *const i8,
        index: (20 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED stop\0".as_ptr() as *const i8,
        index: (23 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED play\0".as_ptr() as *const i8,
        index: (21 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED record\0".as_ptr() as *const i8,
        index: (19 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED listen\0".as_ptr() as *const i8,
        index: (17 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED lcd\0".as_ptr() as *const i8,
        index: (30 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED menu\0".as_ptr() as *const i8,
        index: (28 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED sound\0".as_ptr() as *const i8,
        index: (31 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED esc\0".as_ptr() as *const i8,
        index: (29 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED view\0".as_ptr() as *const i8,
        index: (27 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED enter\0".as_ptr() as *const i8,
        index: (24 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED control\0".as_ptr() as *const i8,
        index: (26 | (CNT_INTVAL as i32)),
    },
];

static a8dj_controller: &[caiaq_controller] = &[
    caiaq_controller {
        name: b"Current input mode\0".as_ptr() as *const i8,
        index: (0 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"GND lift for TC Vinyl mode\0".as_ptr() as *const i8,
        index: 24 + 0,
    },
    caiaq_controller {
        name: b"GND lift for TC CD/Line mode\0".as_ptr() as *const i8,
        index: 24 + 1,
    },
    caiaq_controller {
        name: b"GND lift for phono mode\0".as_ptr() as *const i8,
        index: 24 + 2,
    },
    caiaq_controller {
        name: b"Software lock\0".as_ptr() as *const i8,
        index: 40,
    },
];

static a4dj_controller: &[caiaq_controller] = &[
    caiaq_controller {
        name: b"Current input mode\0".as_ptr() as *const i8,
        index: (0 | (CNT_INTVAL as i32)),
    },
];

static kontrolx1_controller: &[caiaq_controller] = &[
    caiaq_controller {
        name: b"LED FX A: ON\0".as_ptr() as *const i8,
        index: (7 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED FX A: 1\0".as_ptr() as *const i8,
        index: (6 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED FX A: 2\0".as_ptr() as *const i8,
        index: (5 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED FX A: 3\0".as_ptr() as *const i8,
        index: (4 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED FX B: ON\0".as_ptr() as *const i8,
        index: (3 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED FX B: 1\0".as_ptr() as *const i8,
        index: (2 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED FX B: 2\0".as_ptr() as *const i8,
        index: (1 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED FX B: 3\0".as_ptr() as *const i8,
        index: (0 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED Hotcue\0".as_ptr() as *const i8,
        index: (28 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED Shift (white)\0".as_ptr() as *const i8,
        index: (29 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED Shift (green)\0".as_ptr() as *const i8,
        index: (30 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED Deck A: FX1\0".as_ptr() as *const i8,
        index: (24 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED Deck A: FX2\0".as_ptr() as *const i8,
        index: (25 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED Deck A: IN\0".as_ptr() as *const i8,
        index: (17 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED Deck A: OUT\0".as_ptr() as *const i8,
        index: (16 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED Deck A: < BEAT\0".as_ptr() as *const i8,
        index: (19 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED Deck A: BEAT >\0".as_ptr() as *const i8,
        index: (18 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED Deck A: CUE/ABS\0".as_ptr() as *const i8,
        index: (21 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED Deck A: CUP/REL\0".as_ptr() as *const i8,
        index: (20 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED Deck A: PLAY\0".as_ptr() as *const i8,
        index: (23 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED Deck A: SYNC\0".as_ptr() as *const i8,
        index: (22 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED Deck B: FX1\0".as_ptr() as *const i8,
        index: (26 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED Deck B: FX2\0".as_ptr() as *const i8,
        index: (27 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED Deck B: IN\0".as_ptr() as *const i8,
        index: (15 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED Deck B: OUT\0".as_ptr() as *const i8,
        index: (14 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED Deck B: < BEAT\0".as_ptr() as *const i8,
        index: (13 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED Deck B: BEAT >\0".as_ptr() as *const i8,
        index: (12 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED Deck B: CUE/ABS\0".as_ptr() as *const i8,
        index: (11 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED Deck B: CUP/REL\0".as_ptr() as *const i8,
        index: (10 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED Deck B: PLAY\0".as_ptr() as *const i8,
        index: (9 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED Deck B: SYNC\0".as_ptr() as *const i8,
        index: (8 | (CNT_INTVAL as i32)),
    },
];

static kontrols4_controller: &[caiaq_controller] = &[
    caiaq_controller {
        name: b"LED: Master: Quant\0".as_ptr() as *const i8,
        index: (10 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Master: Headphone\0".as_ptr() as *const i8,
        index: (11 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Master: Master\0".as_ptr() as *const i8,
        index: (12 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Master: Snap\0".as_ptr() as *const i8,
        index: (14 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Master: Warning\0".as_ptr() as *const i8,
        index: (15 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Master: Master button\0".as_ptr() as *const i8,
        index: (112 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Master: Snap button\0".as_ptr() as *const i8,
        index: (113 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Master: Rec\0".as_ptr() as *const i8,
        index: (118 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Master: Size\0".as_ptr() as *const i8,
        index: (119 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Master: Quant button\0".as_ptr() as *const i8,
        index: (120 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Master: Browser button\0".as_ptr() as *const i8,
        index: (121 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Master: Play button\0".as_ptr() as *const i8,
        index: (126 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Master: Undo button\0".as_ptr() as *const i8,
        index: (127 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel A: >\0".as_ptr() as *const i8,
        index: (4 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel A: <\0".as_ptr() as *const i8,
        index: (5 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel A: Meter 1\0".as_ptr() as *const i8,
        index: (97 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel A: Meter 2\0".as_ptr() as *const i8,
        index: (98 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel A: Meter 3\0".as_ptr() as *const i8,
        index: (99 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel A: Meter 4\0".as_ptr() as *const i8,
        index: (100 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel A: Meter 5\0".as_ptr() as *const i8,
        index: (101 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel A: Meter 6\0".as_ptr() as *const i8,
        index: (102 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel A: Meter clip\0".as_ptr() as *const i8,
        index: (103 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel A: Active\0".as_ptr() as *const i8,
        index: (114 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel A: Cue\0".as_ptr() as *const i8,
        index: (116 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel A: FX1\0".as_ptr() as *const i8,
        index: (149 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel A: FX2\0".as_ptr() as *const i8,
        index: (148 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel B: >\0".as_ptr() as *const i8,
        index: (2 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel B: <\0".as_ptr() as *const i8,
        index: (3 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel B: Meter 1\0".as_ptr() as *const i8,
        index: (89 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel B: Meter 2\0".as_ptr() as *const i8,
        index: (90 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel B: Meter 3\0".as_ptr() as *const i8,
        index: (91 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel B: Meter 4\0".as_ptr() as *const i8,
        index: (92 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel B: Meter 5\0".as_ptr() as *const i8,
        index: (93 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel B: Meter 6\0".as_ptr() as *const i8,
        index: (94 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel B: Meter clip\0".as_ptr() as *const i8,
        index: (95 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel B: Active\0".as_ptr() as *const i8,
        index: (122 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel B: Cue\0".as_ptr() as *const i8,
        index: (125 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel B: FX1\0".as_ptr() as *const i8,
        index: (147 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel B: FX2\0".as_ptr() as *const i8,
        index: (146 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel C: >\0".as_ptr() as *const i8,
        index: (6 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel C: <\0".as_ptr() as *const i8,
        index: (7 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel C: Meter 1\0".as_ptr() as *const i8,
        index: (105 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel C: Meter 2\0".as_ptr() as *const i8,
        index: (106 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel C: Meter 3\0".as_ptr() as *const i8,
        index: (107 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel C: Meter 4\0".as_ptr() as *const i8,
        index: (108 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel C: Meter 5\0".as_ptr() as *const i8,
        index: (109 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel C: Meter 6\0".as_ptr() as *const i8,
        index: (110 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel C: Meter clip\0".as_ptr() as *const i8,
        index: (111 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel C: Active\0".as_ptr() as *const i8,
        index: (115 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel C: Cue\0".as_ptr() as *const i8,
        index: (117 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel C: FX1\0".as_ptr() as *const i8,
        index: (151 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel C: FX2\0".as_ptr() as *const i8,
        index: (150 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel D: >\0".as_ptr() as *const i8,
        index: (0 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel D: <\0".as_ptr() as *const i8,
        index: (1 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel D: Meter 1\0".as_ptr() as *const i8,
        index: (81 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel D: Meter 2\0".as_ptr() as *const i8,
        index: (82 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel D: Meter 3\0".as_ptr() as *const i8,
        index: (83 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel D: Meter 4\0".as_ptr() as *const i8,
        index: (84 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel D: Meter 5\0".as_ptr() as *const i8,
        index: (85 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel D: Meter 6\0".as_ptr() as *const i8,
        index: (86 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel D: Meter clip\0".as_ptr() as *const i8,
        index: (87 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel D: Active\0".as_ptr() as *const i8,
        index: (123 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel D: Cue\0".as_ptr() as *const i8,
        index: (124 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel D: FX1\0".as_ptr() as *const i8,
        index: (145 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Channel D: FX2\0".as_ptr() as *const i8,
        index: (144 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: 1 (blue)\0".as_ptr() as *const i8,
        index: (22 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: 1 (green)\0".as_ptr() as *const i8,
        index: (23 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: 2 (blue)\0".as_ptr() as *const i8,
        index: (20 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: 2 (green)\0".as_ptr() as *const i8,
        index: (21 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: 3 (blue)\0".as_ptr() as *const i8,
        index: (18 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: 3 (green)\0".as_ptr() as *const i8,
        index: (19 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: 4 (blue)\0".as_ptr() as *const i8,
        index: (16 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: 4 (green)\0".as_ptr() as *const i8,
        index: (17 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Load\0".as_ptr() as *const i8,
        index: (44 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Deck C button\0".as_ptr() as *const i8,
        index: (45 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: In\0".as_ptr() as *const i8,
        index: (47 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Out\0".as_ptr() as *const i8,
        index: (46 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Shift\0".as_ptr() as *const i8,
        index: (24 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Sync\0".as_ptr() as *const i8,
        index: (27 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Cue\0".as_ptr() as *const i8,
        index: (26 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Play\0".as_ptr() as *const i8,
        index: (25 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Tempo up\0".as_ptr() as *const i8,
        index: (33 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Tempo down\0".as_ptr() as *const i8,
        index: (32 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Master\0".as_ptr() as *const i8,
        index: (34 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Keylock\0".as_ptr() as *const i8,
        index: (35 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Deck A\0".as_ptr() as *const i8,
        index: (37 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Deck C\0".as_ptr() as *const i8,
        index: (36 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Samples\0".as_ptr() as *const i8,
        index: (38 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: On Air\0".as_ptr() as *const i8,
        index: (39 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Sample 1\0".as_ptr() as *const i8,
        index: (31 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Sample 2\0".as_ptr() as *const i8,
        index: (30 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Sample 3\0".as_ptr() as *const i8,
        index: (29 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Sample 4\0".as_ptr() as *const i8,
        index: (28 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Digit 1 - A\0".as_ptr() as *const i8,
        index: (55 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Digit 1 - B\0".as_ptr() as *const i8,
        index: (54 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Digit 1 - C\0".as_ptr() as *const i8,
        index: (53 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Digit 1 - D\0".as_ptr() as *const i8,
        index: (52 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Digit 1 - E\0".as_ptr() as *const i8,
        index: (51 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Digit 1 - F\0".as_ptr() as *const i8,
        index: (50 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Digit 1 - G\0".as_ptr() as *const i8,
        index: (49 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Digit 1 - dot\0".as_ptr() as *const i8,
        index: (48 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Digit 2 - A\0".as_ptr() as *const i8,
        index: (63 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Digit 2 - B\0".as_ptr() as *const i8,
        index: (62 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Digit 2 - C\0".as_ptr() as *const i8,
        index: (61 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Digit 2 - D\0".as_ptr() as *const i8,
        index: (60 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Digit 2 - E\0".as_ptr() as *const i8,
        index: (59 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Digit 2 - F\0".as_ptr() as *const i8,
        index: (58 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Digit 2 - G\0".as_ptr() as *const i8,
        index: (57 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck A: Digit 2 - dot\0".as_ptr() as *const i8,
        index: (56 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: 1 (blue)\0".as_ptr() as *const i8,
        index: (78 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: 1 (green)\0".as_ptr() as *const i8,
        index: (79 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: 2 (blue)\0".as_ptr() as *const i8,
        index: (76 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: 2 (green)\0".as_ptr() as *const i8,
        index: (77 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: 3 (blue)\0".as_ptr() as *const i8,
        index: (74 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: 3 (green)\0".as_ptr() as *const i8,
        index: (75 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: 4 (blue)\0".as_ptr() as *const i8,
        index: (72 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: 4 (green)\0".as_ptr() as *const i8,
        index: (73 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Load\0".as_ptr() as *const i8,
        index: (180 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Deck D button\0".as_ptr() as *const i8,
        index: (181 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: In\0".as_ptr() as *const i8,
        index: (183 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Out\0".as_ptr() as *const i8,
        index: (182 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Shift\0".as_ptr() as *const i8,
        index: (64 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Sync\0".as_ptr() as *const i8,
        index: (67 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Cue\0".as_ptr() as *const i8,
        index: (66 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Play\0".as_ptr() as *const i8,
        index: (65 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Tempo up\0".as_ptr() as *const i8,
        index: (185 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Tempo down\0".as_ptr() as *const i8,
        index: (184 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Master\0".as_ptr() as *const i8,
        index: (186 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Keylock\0".as_ptr() as *const i8,
        index: (187 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Deck B\0".as_ptr() as *const i8,
        index: (189 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Deck D\0".as_ptr() as *const i8,
        index: (188 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Samples\0".as_ptr() as *const i8,
        index: (190 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: On Air\0".as_ptr() as *const i8,
        index: (191 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Sample 1\0".as_ptr() as *const i8,
        index: (71 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Sample 2\0".as_ptr() as *const i8,
        index: (70 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Sample 3\0".as_ptr() as *const i8,
        index: (69 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Sample 4\0".as_ptr() as *const i8,
        index: (68 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Digit 1 - A\0".as_ptr() as *const i8,
        index: (175 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Digit 1 - B\0".as_ptr() as *const i8,
        index: (174 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Digit 1 - C\0".as_ptr() as *const i8,
        index: (173 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Digit 1 - D\0".as_ptr() as *const i8,
        index: (172 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Digit 1 - E\0".as_ptr() as *const i8,
        index: (171 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Digit 1 - F\0".as_ptr() as *const i8,
        index: (170 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Digit 1 - G\0".as_ptr() as *const i8,
        index: (169 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Digit 1 - dot\0".as_ptr() as *const i8,
        index: (168 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Digit 2 - A\0".as_ptr() as *const i8,
        index: (167 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Digit 2 - B\0".as_ptr() as *const i8,
        index: (166 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Digit 2 - C\0".as_ptr() as *const i8,
        index: (165 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Digit 2 - D\0".as_ptr() as *const i8,
        index: (164 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Digit 2 - E\0".as_ptr() as *const i8,
        index: (163 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Digit 2 - F\0".as_ptr() as *const i8,
        index: (162 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Digit 2 - G\0".as_ptr() as *const i8,
        index: (161 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Deck B: Digit 2 - dot\0".as_ptr() as *const i8,
        index: (160 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: FX1: dry/wet\0".as_ptr() as *const i8,
        index: (153 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: FX1: 1\0".as_ptr() as *const i8,
        index: (154 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: FX1: 2\0".as_ptr() as *const i8,
        index: (155 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: FX1: 3\0".as_ptr() as *const i8,
        index: (156 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: FX1: Mode\0".as_ptr() as *const i8,
        index: (157 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: FX2: dry/wet\0".as_ptr() as *const i8,
        index: (129 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: FX2: 1\0".as_ptr() as *const i8,
        index: (130 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: FX2: 2\0".as_ptr() as *const i8,
        index: (131 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: FX2: 3\0".as_ptr() as *const i8,
        index: (132 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: FX2: Mode\0".as_ptr() as *const i8,
        index: (133 | (CNT_INTVAL as i32)),
    },
];

static maschine_controller: &[caiaq_controller] = &[
    caiaq_controller {
        name: b"LED: Pad 1\0".as_ptr() as *const i8,
        index: (3 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Pad 2\0".as_ptr() as *const i8,
        index: (2 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Pad 3\0".as_ptr() as *const i8,
        index: (1 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Pad 4\0".as_ptr() as *const i8,
        index: (0 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Pad 5\0".as_ptr() as *const i8,
        index: (7 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Pad 6\0".as_ptr() as *const i8,
        index: (6 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Pad 7\0".as_ptr() as *const i8,
        index: (5 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Pad 8\0".as_ptr() as *const i8,
        index: (4 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Pad 9\0".as_ptr() as *const i8,
        index: (11 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Pad 10\0".as_ptr() as *const i8,
        index: (10 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Pad 11\0".as_ptr() as *const i8,
        index: (9 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Pad 12\0".as_ptr() as *const i8,
        index: (8 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Pad 13\0".as_ptr() as *const i8,
        index: (15 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Pad 14\0".as_ptr() as *const i8,
        index: (14 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Pad 15\0".as_ptr() as *const i8,
        index: (13 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Pad 16\0".as_ptr() as *const i8,
        index: (12 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Mute\0".as_ptr() as *const i8,
        index: (16 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Solo\0".as_ptr() as *const i8,
        index: (17 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Select\0".as_ptr() as *const i8,
        index: (18 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Duplicate\0".as_ptr() as *const i8,
        index: (19 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Navigate\0".as_ptr() as *const i8,
        index: (20 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Pad Mode\0".as_ptr() as *const i8,
        index: (21 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Pattern\0".as_ptr() as *const i8,
        index: (22 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Scene\0".as_ptr() as *const i8,
        index: (23 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Shift\0".as_ptr() as *const i8,
        index: (24 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Erase\0".as_ptr() as *const i8,
        index: (25 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Grid\0".as_ptr() as *const i8,
        index: (26 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Right Bottom\0".as_ptr() as *const i8,
        index: (27 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Rec\0".as_ptr() as *const i8,
        index: (28 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Play\0".as_ptr() as *const i8,
        index: (29 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Left Bottom\0".as_ptr() as *const i8,
        index: (32 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Restart\0".as_ptr() as *const i8,
        index: (33 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Group A\0".as_ptr() as *const i8,
        index: (41 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Group B\0".as_ptr() as *const i8,
        index: (40 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Group C\0".as_ptr() as *const i8,
        index: (37 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Group D\0".as_ptr() as *const i8,
        index: (36 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Group E\0".as_ptr() as *const i8,
        index: (39 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Group F\0".as_ptr() as *const i8,
        index: (38 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Group G\0".as_ptr() as *const i8,
        index: (35 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Group H\0".as_ptr() as *const i8,
        index: (34 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Auto Write\0".as_ptr() as *const i8,
        index: (42 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Snap\0".as_ptr() as *const i8,
        index: (43 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Right Top\0".as_ptr() as *const i8,
        index: (44 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Left Top\0".as_ptr() as *const i8,
        index: (45 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Sampling\0".as_ptr() as *const i8,
        index: (46 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Browse\0".as_ptr() as *const i8,
        index: (47 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Step\0".as_ptr() as *const i8,
        index: (48 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Control\0".as_ptr() as *const i8,
        index: (49 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Top Button 1\0".as_ptr() as *const i8,
        index: (57 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Top Button 2\0".as_ptr() as *const i8,
        index: (56 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Top Button 3\0".as_ptr() as *const i8,
        index: (55 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Top Button 4\0".as_ptr() as *const i8,
        index: (54 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Top Button 5\0".as_ptr() as *const i8,
        index: (53 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Top Button 6\0".as_ptr() as *const i8,
        index: (52 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Top Button 7\0".as_ptr() as *const i8,
        index: (51 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Top Button 8\0".as_ptr() as *const i8,
        index: (50 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"LED: Note Repeat\0".as_ptr() as *const i8,
        index: (58 | (CNT_INTVAL as i32)),
    },
    caiaq_controller {
        name: b"Backlight Display\0".as_ptr() as *const i8,
        index: (59 | (CNT_INTVAL as i32)),
    },
];

unsafe fn add_controls(
    mut c: *const caiaq_controller,
    num: usize,
    cdev: *mut snd_usb_caiaqdev,
) -> i32 {
    let mut i = 0;
    while i < num {
        kcontrol_template.name = (*c).name;
        kcontrol_template.private_value = (*c).index as u32;
        let kc = snd_ctl_new1(
            &kcontrol_template,
            cdev as *mut std::ffi::c_void,
        );
        let ret = snd_ctl_add((*cdev).chip.card, kc);
        if ret < 0 {
            return ret;
        }
        c = c.add(1);
        i += 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_usb_caiaq_control_init(cdev: *mut snd_usb_caiaqdev) -> i32 {
    let mut ret = 0;

    match (*cdev).chip.usb_id {
        id if id == usb_id(USB_VID_NATIVEINSTRUMENTS, USB_PID_AK1) => {
            ret = add_controls(ak1_controller.as_ptr(), ak1_controller.len(), cdev);
        }
        id if id == usb_id(USB_VID_NATIVEINSTRUMENTS, USB_PID_RIGKONTROL2) => {
            ret = add_controls(rk2_controller.as_ptr(), rk2_controller.len(), cdev);
        }
        id if id == usb_id(USB_VID_NATIVEINSTRUMENTS, USB_PID_RIGKONTROL3) => {
            ret = add_controls(rk3_controller.as_ptr(), rk3_controller.len(), cdev);
        }
        id if id == usb_id(USB_VID_NATIVEINSTRUMENTS, USB_PID_KORECONTROLLER)
            || id == usb_id(USB_VID_NATIVEINSTRUMENTS, USB_PID_KORECONTROLLER2) =>
        {
            ret = add_controls(kore_controller.as_ptr(), kore_controller.len(), cdev);
        }
        id if id == usb_id(USB_VID_NATIVEINSTRUMENTS, USB_PID_AUDIO8DJ) => {
            ret = add_controls(a8dj_controller.as_ptr(), a8dj_controller.len(), cdev);
        }
        id if id == usb_id(USB_VID_NATIVEINSTRUMENTS, USB_PID_AUDIO4DJ) => {
            ret = add_controls(a4dj_controller.as_ptr(), a4dj_controller.len(), cdev);
        }
        id if id == usb_id(USB_VID_NATIVEINSTRUMENTS, USB_PID_TRAKTORKONTROLX1) => {
            ret = add_controls(kontrolx1_controller.as_ptr(), kontrolx1_controller.len(), cdev);
        }
        id if id == usb_id(USB_VID_NATIVEINSTRUMENTS, USB_PID_TRAKTORKONTROLS4) => {
            ret = add_controls(kontrols4_controller.as_ptr(), kontrols4_controller.len(), cdev);
        }
        id if id == usb_id(USB_VID_NATIVEINSTRUMENTS, USB_PID_MASCHINECONTROLLER) => {
            ret = add_controls(maschine_controller.as_ptr(), maschine_controller.len(), cdev);
        }
        _ => {}
    }

    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
