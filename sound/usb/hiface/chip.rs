// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Linux driver for M2Tech hiFace compatible devices
 *
 * Copyright 2012-2013 (C) M2TECH S.r.l and Amarula Solutions B.V.
 *
 * Authors:  Michael Trimarchi <michael@amarulasolutions.com>
 *           Antonio Ospite <ao2@amarulasolutions.com>
 *
 * The driver is based on the work done in TerraTec DMX 6Fire USB
 */

use kernel::prelude::*;
use kernel::usb;
use kernel::str::CStr;
use kernel::sync::Mutex;

module! {
    type: HifaceDriver,
    name: "snd_usb_hiface",
    author: "Michael Trimarchi <michael@amarulasolutions.com>",
    author: "Antonio Ospite <ao2@amarulasolutions.com>",
    description: "M2Tech hiFace USB-SPDIF audio driver",
    license: "GPL v2",
}

const DRIVER_NAME: &CStr = c"snd-usb-hiface";
const CARD_NAME: &CStr = c"hiFace";

// Module parameters
static INDEX: [i32; 32] = [0; 32]; // SNDRV_CARDS default
static ID: [*const u8; 32] = [0 as *const u8; 32]; // SNDRV_CARDS default
static ENABLE: [bool; 32] = [true; 32]; // SNDRV_DEFAULT_ENABLE_PNP

static REGISTER_MUTEX: Mutex<()> = Mutex::new(());

#[repr(C)]
struct HifaceVendorQuirk {
    device_name: *const u8,
    extra_freq: u8,
}

fn hiface_chip_create(
    intf: *mut usb::UsbInterface,
    device: *mut usb::UsbDevice,
    idx: i32,
    quirk: *const HifaceVendorQuirk,
    rchip: *mut *mut HifaceChip,
) -> i32 {
    let mut card: *mut SndCard = core::ptr::null_mut();
    let mut chip: *mut HifaceChip;
    let ret: i32;
    let len: usize;

    unsafe {
        *rchip = core::ptr::null_mut();

        // if we are here, card can be registered in alsa.
        ret = snd_card_new(
            &(*intf).dev,
            INDEX[idx as usize],
            ID[idx as usize] as *const i8,
            core::ptr::null_mut(), // THIS_MODULE equivalent
            core::mem::size_of_val(&*chip as *const HifaceChip) as usize,
            &mut card,
        );
        if ret < 0 {
            dev_err(
                &(*device).dev,
                c"cannot create alsa card.\n".as_ptr() as *const i8,
            );
            return ret;
        }

        strscpy(
            (*card).driver.as_mut_ptr() as *mut u8,
            DRIVER_NAME.as_ptr() as *const u8,
            core::mem::size_of_val(&(*card).driver),
        );

        if !quirk.is_null() && !(*quirk).device_name.is_null() {
            strscpy(
                (*card).shortname.as_mut_ptr() as *mut u8,
                (*quirk).device_name,
                core::mem::size_of_val(&(*card).shortname),
            );
        } else {
            strscpy(
                (*card).shortname.as_mut_ptr() as *mut u8,
                b"M2Tech generic audio".as_ptr(),
                core::mem::size_of_val(&(*card).shortname),
            );
        }

        strlcat(
            (*card).longname.as_mut_ptr() as *mut u8,
            (*card).shortname.as_ptr() as *const u8,
            core::mem::size_of_val(&(*card).longname),
        );
        len = strlcat(
            (*card).longname.as_mut_ptr() as *mut u8,
            b" at ".as_ptr(),
            core::mem::size_of_val(&(*card).longname),
        );
        if len < core::mem::size_of_val(&(*card).longname) {
            usb_make_path(
                device,
                ((*card).longname.as_mut_ptr() as *mut u8).add(len),
                core::mem::size_of_val(&(*card).longname) - len,
            );
        }

        chip = (*card).private_data as *mut HifaceChip;
        (*chip).dev = device;
        (*chip).card = card;

        *rchip = chip;
    }
    return 0;
}

fn hiface_chip_probe(intf: *mut usb::UsbInterface, usb_id: *const usb::UsbDeviceId) -> i32 {
    let quirk: *const HifaceVendorQuirk;
    let ret: i32;
    let mut i: i32;
    let mut chip: *mut HifaceChip;
    let device: *mut usb::UsbDevice;

    unsafe {
        quirk = (*usb_id).driver_info as *const HifaceVendorQuirk;
        device = interface_to_usbdev(intf);

        ret = usb_set_interface(device, 0, 0);
        if ret != 0 {
            dev_err(
                &(*device).dev,
                c"can't set first interface for hiFace device.\n".as_ptr() as *const i8,
            );
            return -5; // -EIO
        }

        // check whether the card is already registered
        chip = core::ptr::null_mut();
        let _guard = REGISTER_MUTEX.lock();

        i = 0;
        while i < 32 { // SNDRV_CARDS
            if ENABLE[i as usize] {
                break;
            }
            i += 1;
        }

        if i >= 32 { // SNDRV_CARDS
            dev_err(
                &(*device).dev,
                c"no available hiFace audio device\n".as_ptr() as *const i8,
            );
            return -19; // -ENODEV
        }

        ret = hiface_chip_create(intf, device, i, quirk, &mut chip);
        if ret < 0 {
            return ret;
        }

        ret = hiface_pcm_init(chip, if !quirk.is_null() { (*quirk).extra_freq as i32 } else { 0 });
        if ret < 0 {
            // goto err_chip_destroy
            snd_card_free((*chip).card);
            return ret;
        }

        ret = snd_card_register((*chip).card);
        if ret < 0 {
            dev_err(
                &(*device).dev,
                c"cannot register hiFace card\n".as_ptr() as *const i8,
            );
            // goto err_chip_destroy
            snd_card_free((*chip).card);
            return ret;
        }

        usb_set_intfdata(intf, chip as *mut core::ffi::c_void);
    }
    return 0;
}

fn hiface_chip_disconnect(intf: *mut usb::UsbInterface) {
    unsafe {
        let chip: *mut HifaceChip = usb_get_intfdata(intf) as *mut HifaceChip;
        if chip.is_null() {
            return;
        }

        let card = (*chip).card;

        // Make sure that the userspace cannot create new request
        snd_card_disconnect(card);

        hiface_pcm_abort(chip);
        snd_card_free_when_closed(card);
    }
}

#[repr(C)]
struct UsbDeviceIdEntry {
    device_id: u32,
    driver_info: *const HifaceVendorQuirk,
}

static DEVICE_TABLE: &[UsbDeviceIdEntry] = &[
    UsbDeviceIdEntry {
        device_id: 0x04b40384,
        driver_info: &HifaceVendorQuirk {
            device_name: b"Young\0".as_ptr(),
            extra_freq: 1,
        },
    },
    UsbDeviceIdEntry {
        device_id: 0x04b4930b,
        driver_info: &HifaceVendorQuirk {
            device_name: b"hiFace\0".as_ptr(),
            extra_freq: 0,
        },
    },
    UsbDeviceIdEntry {
        device_id: 0x04b4931b,
        driver_info: &HifaceVendorQuirk {
            device_name: b"North Star\0".as_ptr(),
            extra_freq: 0,
        },
    },
    UsbDeviceIdEntry {
        device_id: 0x04b4931c,
        driver_info: &HifaceVendorQuirk {
            device_name: b"W4S Young\0".as_ptr(),
            extra_freq: 0,
        },
    },
    UsbDeviceIdEntry {
        device_id: 0x04b4931d,
        driver_info: &HifaceVendorQuirk {
            device_name: b"Corrson\0".as_ptr(),
            extra_freq: 0,
        },
    },
    UsbDeviceIdEntry {
        device_id: 0x04b4931e,
        driver_info: &HifaceVendorQuirk {
            device_name: b"AUDIA\0".as_ptr(),
            extra_freq: 0,
        },
    },
    UsbDeviceIdEntry {
        device_id: 0x04b4931f,
        driver_info: &HifaceVendorQuirk {
            device_name: b"SL Audio\0".as_ptr(),
            extra_freq: 0,
        },
    },
    UsbDeviceIdEntry {
        device_id: 0x04b49320,
        driver_info: &HifaceVendorQuirk {
            device_name: b"Empirical\0".as_ptr(),
            extra_freq: 0,
        },
    },
    UsbDeviceIdEntry {
        device_id: 0x04b49321,
        driver_info: &HifaceVendorQuirk {
            device_name: b"Rockna\0".as_ptr(),
            extra_freq: 0,
        },
    },
    UsbDeviceIdEntry {
        device_id: 0x249c9001,
        driver_info: &HifaceVendorQuirk {
            device_name: b"Pathos\0".as_ptr(),
            extra_freq: 0,
        },
    },
    UsbDeviceIdEntry {
        device_id: 0x249c9002,
        driver_info: &HifaceVendorQuirk {
            device_name: b"Metronome\0".as_ptr(),
            extra_freq: 0,
        },
    },
    UsbDeviceIdEntry {
        device_id: 0x249c9006,
        driver_info: &HifaceVendorQuirk {
            device_name: b"CAD\0".as_ptr(),
            extra_freq: 0,
        },
    },
    UsbDeviceIdEntry {
        device_id: 0x249c9008,
        driver_info: &HifaceVendorQuirk {
            device_name: b"Audio Esclusive\0".as_ptr(),
            extra_freq: 0,
        },
    },
    UsbDeviceIdEntry {
        device_id: 0x249c931c,
        driver_info: &HifaceVendorQuirk {
            device_name: b"Rotel\0".as_ptr(),
            extra_freq: 0,
        },
    },
    UsbDeviceIdEntry {
        device_id: 0x249c932c,
        driver_info: &HifaceVendorQuirk {
            device_name: b"Eeaudio\0".as_ptr(),
            extra_freq: 0,
        },
    },
    UsbDeviceIdEntry {
        device_id: 0x245f931c,
        driver_info: &HifaceVendorQuirk {
            device_name: b"CHORD\0".as_ptr(),
            extra_freq: 0,
        },
    },
    UsbDeviceIdEntry {
        device_id: 0x25c69002,
        driver_info: &HifaceVendorQuirk {
            device_name: b"Vitus\0".as_ptr(),
            extra_freq: 0,
        },
    },
];

struct HifaceDriver;

impl usb::UsbDriver for HifaceDriver {
    fn name(&self) -> &CStr {
        DRIVER_NAME
    }

    fn probe(&self, intf: *mut usb::UsbInterface, usb_id: *const usb::UsbDeviceId) -> i32 {
        hiface_chip_probe(intf, usb_id)
    }

    fn disconnect(&self, intf: *mut usb::UsbInterface) {
        hiface_chip_disconnect(intf);
    }

    fn id_table(&self) -> &[usb::UsbDeviceId] {
        &[]
    }
}

// Forward declarations of external symbols from chip.h
#[repr(C)]
pub struct HifaceChip {
    pub dev: *mut usb::UsbDevice,
    pub card: *mut SndCard,
    // Other fields omitted - defined in chip.h
}

#[repr(C)]
pub struct SndCard {
    pub driver: [u8; 16],
    pub shortname: [u8; 32],
    pub longname: [u8; 80],
    pub private_data: *mut core::ffi::c_void,
    pub dev: *mut core::ffi::c_void,
}

// External function declarations
extern "C" {
    fn snd_card_new(
        device: *mut core::ffi::c_void,
        idx: i32,
        xid: *const i8,
        module: *mut core::ffi::c_void,
        extra_size: usize,
        card_ret: *mut *mut SndCard,
    ) -> i32;

    fn snd_card_free(card: *mut SndCard) -> i32;
    fn snd_card_free_when_closed(card: *mut SndCard) -> i32;
    fn snd_card_disconnect(card: *mut SndCard) -> i32;
    fn snd_card_register(card: *mut SndCard) -> i32;

    fn strscpy(dest: *mut u8, src: *const u8, size: usize) -> usize;
    fn strlcat(dest: *mut u8, src: *const u8, size: usize) -> usize;

    fn dev_err(dev: *mut core::ffi::c_void, fmt: *const i8, ...);

    fn usb_set_interface(dev: *mut usb::UsbDevice, ifnum: u32, alternate: u32) -> i32;
    fn usb_make_path(dev: *mut usb::UsbDevice, buf: *mut u8, size: usize) -> i32;
    fn usb_set_intfdata(intf: *mut usb::UsbInterface, data: *mut core::ffi::c_void);
    fn usb_get_intfdata(intf: *mut usb::UsbInterface) -> *mut core::ffi::c_void;
    fn interface_to_usbdev(intf: *mut usb::UsbInterface) -> *mut usb::UsbDevice;

    fn hiface_pcm_init(chip: *mut HifaceChip, extra_freq: i32) -> i32;
    fn hiface_pcm_abort(chip: *mut HifaceChip);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
