// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Linux driver for TerraTec DMX 6Fire USB
 *
 * Main routines and module definitions.
 *
 * Author:	Torsten Schenk <torsten.schenk@zoho.com>
 * Created:	Jan 01, 2011
 * Copyright:	(C) Torsten Schenk
 */

// Include directives (kernel module system):
// #include "chip.h"
// #include "firmware.h"
// #include "pcm.h"
// #include "control.h"
// #include "comm.h"
// #include "midi.h"
// #include <linux/moduleparam.h>
// #include <linux/interrupt.h>
// #include <linux/module.h>
// #include <linux/init.h>
// #include <linux/gfp.h>
// #include <sound/initval.h>

use std::sync::Mutex;
use std::ptr;
use std::ffi::CStr;

// Module metadata (kernel-specific macros, preserved in comments):
// MODULE_AUTHOR("Torsten Schenk <torsten.schenk@zoho.com>");
// MODULE_DESCRIPTION("TerraTec DMX 6Fire USB audio driver");
// MODULE_LICENSE("GPL v2");

// External types from included headers
#[repr(C)]
pub struct SfireChip {
    pub pcm: *mut std::ffi::c_void,
    pub midi: *mut std::ffi::c_void,
    pub comm: *mut std::ffi::c_void,
    pub control: *mut std::ffi::c_void,
    pub dev: *mut UsbDevice,
    pub regidx: i32,
    pub intf_count: i32,
    pub card: *mut SndCard,
    pub shutdown: bool,
}

#[repr(C)]
pub struct SndCard {
    pub private_data: *mut std::ffi::c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut SndCard)>,
    pub driver: [u8; 16],
    pub shortname: [u8; 32],
    pub longname: [u8; 80],
}

#[repr(C)]
pub struct UsbDevice {
    pub bus: *mut UsbBus,
    pub devnum: u32,
}

#[repr(C)]
pub struct UsbBus {
    pub busnum: u32,
}

#[repr(C)]
pub struct UsbInterface {
    pub dev: std::ffi::c_void,
}

#[repr(C)]
pub struct UsbDeviceId {
    pub match_flags: u16,
    pub idVendor: u16,
    pub idProduct: u16,
}

const SNDRV_CARDS: usize = 32; // Typical value from sound/core/init.h

// Module parameter arrays (kernel-specific initialization, preserved with defaults)
// SNDRV_DEFAULT_IDX, SNDRV_DEFAULT_STR, SNDRV_DEFAULT_ENABLE_PNP, SNDRV_DEFAULT_PTR
// are kernel macros that initialize these arrays with standard defaults
static INDEX: [i32; SNDRV_CARDS] = [0; SNDRV_CARDS];
static ID: [*const u8; SNDRV_CARDS] = [ptr::null(); SNDRV_CARDS];
static ENABLE: [bool; SNDRV_CARDS] = [true; SNDRV_CARDS];
static CHIPS: Mutex<[*mut SfireChip; SNDRV_CARDS]> = Mutex::new([ptr::null_mut(); SNDRV_CARDS]);

// Module parameter declarations (kernel-specific, preserved in comments):
// module_param_array(index, int, NULL, 0444);
// MODULE_PARM_DESC(index, "Index value for the 6fire sound device");
// module_param_array(id, charp, NULL, 0444);
// MODULE_PARM_DESC(id, "ID string for the 6fire sound device.");
// module_param_array(enable, bool, NULL, 0444);
// MODULE_PARM_DESC(enable, "Enable the 6fire sound device.");

static REGISTER_MUTEX: Mutex<()> = Mutex::new(());

extern "C" {
    fn usb6fire_pcm_abort(chip: *mut SfireChip);
    fn usb6fire_midi_abort(chip: *mut SfireChip);
    fn usb6fire_comm_abort(chip: *mut SfireChip);
    fn usb6fire_control_abort(chip: *mut SfireChip);
    fn usb6fire_pcm_destroy(chip: *mut SfireChip);
    fn usb6fire_midi_destroy(chip: *mut SfireChip);
    fn usb6fire_comm_destroy(chip: *mut SfireChip);
    fn usb6fire_control_destroy(chip: *mut SfireChip);
    fn usb6fire_fw_init(intf: *mut UsbInterface) -> i32;
    fn usb6fire_comm_init(chip: *mut SfireChip) -> i32;
    fn usb6fire_midi_init(chip: *mut SfireChip) -> i32;
    fn usb6fire_pcm_init(chip: *mut SfireChip) -> i32;
    fn usb6fire_control_init(chip: *mut SfireChip) -> i32;
    fn interface_to_usbdev(intf: *mut UsbInterface) -> *mut UsbDevice;
    fn usb_set_intfdata(intf: *mut UsbInterface, data: *mut SfireChip);
    fn usb_get_intfdata(intf: *mut UsbInterface) -> *mut SfireChip;
    fn usb_set_interface(dev: *mut UsbDevice, ifnum: u32, alt: u32) -> i32;
    fn snd_card_new(
        device: *const std::ffi::c_void,
        idx: i32,
        xid: *const u8,
        module: *mut std::ffi::c_void,
        extra_size: usize,
        card_ret: *mut *mut SndCard,
    ) -> i32;
    fn snd_card_register(card: *mut SndCard) -> i32;
    fn snd_card_disconnect(card: *mut SndCard);
    fn snd_card_free(card: *mut SndCard);
    fn snd_card_free_when_closed(card: *mut SndCard);
    fn dev_err(dev: *const std::ffi::c_void, fmt: *const u8, ...);
    fn strscpy(dest: *mut u8, src: *const u8, count: usize) -> isize;
    fn sprintf(s: *mut u8, format: *const u8, ...) -> i32;
}

const FW_NOT_READY: i32 = 1;
const ENODEV: i32 = 19;
const EIO: i32 = 5;

unsafe fn usb6fire_chip_abort(chip: *mut SfireChip) {
    if !(*chip).pcm.is_null() {
        usb6fire_pcm_abort(chip);
    }
    if !(*chip).midi.is_null() {
        usb6fire_midi_abort(chip);
    }
    if !(*chip).comm.is_null() {
        usb6fire_comm_abort(chip);
    }
    if !(*chip).control.is_null() {
        usb6fire_control_abort(chip);
    }
}

unsafe extern "C" fn usb6fire_card_free(card: *mut SndCard) {
    let chip = (*card).private_data as *mut SfireChip;

    if !(*chip).pcm.is_null() {
        usb6fire_pcm_destroy(chip);
    }
    if !(*chip).midi.is_null() {
        usb6fire_midi_destroy(chip);
    }
    if !(*chip).comm.is_null() {
        usb6fire_comm_destroy(chip);
    }
    if !(*chip).control.is_null() {
        usb6fire_control_destroy(chip);
    }
}

unsafe extern "C" fn usb6fire_chip_probe(
    intf: *mut UsbInterface,
    usb_id: *const UsbDeviceId,
) -> i32 {
    let mut ret: i32;
    let mut i: usize;
    let mut chip: *mut SfireChip = ptr::null_mut();
    let device = interface_to_usbdev(intf);
    let mut regidx: i32 = -1;
    let mut card: *mut SndCard = ptr::null_mut();

    // guard(mutex)(&register_mutex);
    let _guard = REGISTER_MUTEX.lock().unwrap();

    {
        let mut chips_lock = CHIPS.lock().unwrap();
        for i in 0..SNDRV_CARDS {
            if !chips_lock[i].is_null() && (*chips_lock[i]).dev == device {
                (*chips_lock[i]).intf_count += 1;
                usb_set_intfdata(intf, chips_lock[i]);
                return 0;
            } else if chips_lock[i].is_null() && regidx < 0 {
                regidx = i as i32;
            }
        }
    }

    if regidx < 0 {
        dev_err(
            &(*intf).dev as *const _ as *const std::ffi::c_void,
            b"too many cards registered.\n\0".as_ptr(),
        );
        return -ENODEV;
    }

    ret = usb6fire_fw_init(intf);
    if ret < 0 {
        return ret;
    } else if ret == FW_NOT_READY {
        return 0;
    }

    if usb_set_interface(device, 0, 0) != 0 {
        dev_err(
            &(*intf).dev as *const _ as *const std::ffi::c_void,
            b"can't set first interface.\n\0".as_ptr(),
        );
        return -EIO;
    }

    ret = snd_card_new(
        &(*intf).dev as *const _,
        INDEX[regidx as usize],
        ID[regidx as usize],
        ptr::null_mut(),
        std::mem::size_of::<SfireChip>(),
        &mut card,
    );
    if ret < 0 {
        dev_err(
            &(*intf).dev as *const _ as *const std::ffi::c_void,
            b"cannot create alsa card.\n\0".as_ptr(),
        );
        return ret;
    }

    strscpy(
        (*card).driver.as_mut_ptr(),
        b"6FireUSB\0".as_ptr(),
        (*card).driver.len(),
    );
    strscpy(
        (*card).shortname.as_mut_ptr(),
        b"TerraTec DMX6FireUSB\0".as_ptr(),
        (*card).shortname.len(),
    );
    sprintf(
        (*card).longname.as_mut_ptr(),
        b"%s at %d:%d\0".as_ptr(),
        (*card).shortname.as_ptr(),
        (*(*device).bus).busnum,
        (*device).devnum,
    );

    chip = (*card).private_data as *mut SfireChip;
    (*chip).dev = device;
    (*chip).regidx = regidx;
    (*chip).intf_count = 1;
    (*chip).card = card;
    (*card).private_free = Some(usb6fire_card_free);

    ret = usb6fire_comm_init(chip);
    if ret < 0 {
        (*chip).shutdown = true;
        if !card.is_null() {
            snd_card_disconnect(card);
        }
        usb6fire_chip_abort(chip);
        snd_card_free(card);
        return ret;
    }

    ret = usb6fire_midi_init(chip);
    if ret < 0 {
        (*chip).shutdown = true;
        if !card.is_null() {
            snd_card_disconnect(card);
        }
        usb6fire_chip_abort(chip);
        snd_card_free(card);
        return ret;
    }

    ret = usb6fire_pcm_init(chip);
    if ret < 0 {
        (*chip).shutdown = true;
        if !card.is_null() {
            snd_card_disconnect(card);
        }
        usb6fire_chip_abort(chip);
        snd_card_free(card);
        return ret;
    }

    ret = usb6fire_control_init(chip);
    if ret < 0 {
        (*chip).shutdown = true;
        if !card.is_null() {
            snd_card_disconnect(card);
        }
        usb6fire_chip_abort(chip);
        snd_card_free(card);
        return ret;
    }

    ret = snd_card_register(card);
    if ret < 0 {
        dev_err(
            &(*intf).dev as *const _ as *const std::ffi::c_void,
            b"cannot register card.\0".as_ptr(),
        );
        (*chip).shutdown = true;
        if !card.is_null() {
            snd_card_disconnect(card);
        }
        usb6fire_chip_abort(chip);
        snd_card_free(card);
        return ret;
    }

    usb_set_intfdata(intf, chip);
    {
        let mut chips_lock = CHIPS.lock().unwrap();
        chips_lock[regidx as usize] = chip;
    }

    return 0;
}

unsafe extern "C" fn usb6fire_chip_disconnect(intf: *mut UsbInterface) {
    let _guard = REGISTER_MUTEX.lock().unwrap();
    let chip = usb_get_intfdata(intf);

    if chip.is_null() {
        return;
    }

    (*chip).intf_count -= 1;
    if (*chip).intf_count != 0 {
        return;
    }

    {
        let mut chips_lock = CHIPS.lock().unwrap();
        chips_lock[(*chip).regidx as usize] = ptr::null_mut();
    }

    let card = (*chip).card;
    (*chip).shutdown = true;
    if !card.is_null() {
        snd_card_disconnect(card);
    }
    usb6fire_chip_abort(chip);
    if !card.is_null() {
        snd_card_free_when_closed(card);
    }
}

static DEVICE_TABLE: [UsbDeviceId; 2] = [
    UsbDeviceId {
        match_flags: 1, // USB_DEVICE_ID_MATCH_DEVICE
        idVendor: 0x0ccd,
        idProduct: 0x0080,
    },
    UsbDeviceId {
        match_flags: 0,
        idVendor: 0,
        idProduct: 0,
    },
];

// MODULE_DEVICE_TABLE(usb, device_table);

#[repr(C)]
pub struct UsbDriver {
    pub name: *const u8,
    pub probe: unsafe extern "C" fn(*mut UsbInterface, *const UsbDeviceId) -> i32,
    pub disconnect: unsafe extern "C" fn(*mut UsbInterface),
    pub id_table: *const UsbDeviceId,
}

static USB_DRIVER: UsbDriver = UsbDriver {
    name: b"snd-usb-6fire\0".as_ptr(),
    probe: usb6fire_chip_probe,
    disconnect: usb6fire_chip_disconnect,
    id_table: DEVICE_TABLE.as_ptr(),
};

// module_usb_driver(usb_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
