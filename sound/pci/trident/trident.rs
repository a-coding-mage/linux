// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Driver for Trident 4DWave DX/NX & SiS SI7018 Audio PCI soundcard
 *
 *  Driver was originated by Trident <audio@tridentmicro.com>
 *  			     Fri Feb 19 15:55:28 MST 1999
 */

// Rust translation of pci/trident/trident.c.
// Kernel/ALSA include dependencies from the C source:
// <linux/init.h>, <linux/pci.h>, <linux/time.h>, <linux/module.h>,
// <sound/core.h>, "trident.h", <sound/initval.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_IDX_VALUE: c_int = -1;
const SNDRV_DEFAULT_ENABLE_PNP_VALUE: bool = true;

const ENODEV: c_int = 19;
const ENOENT: c_int = 2;

const PCI_VENDOR_ID_TRIDENT: c_uint = 0x1023;
const PCI_DEVICE_ID_TRIDENT_4DWAVE_DX: c_uint = 0x2000;
const PCI_DEVICE_ID_TRIDENT_4DWAVE_NX: c_uint = 0x2001;
const PCI_VENDOR_ID_SI: c_uint = 0x1039;
const PCI_DEVICE_ID_SI_7018: c_uint = 0x7018;
const PCI_CLASS_MULTIMEDIA_AUDIO: c_uint = 0x0401;

const TRIDENT_DEVICE_ID_DX: c_uint = (PCI_VENDOR_ID_TRIDENT << 16) | PCI_DEVICE_ID_TRIDENT_4DWAVE_DX;
const TRIDENT_DEVICE_ID_NX: c_uint = (PCI_VENDOR_ID_TRIDENT << 16) | PCI_DEVICE_ID_TRIDENT_4DWAVE_NX;
const TRIDENT_DEVICE_ID_SI7018: c_uint = (PCI_VENDOR_ID_SI << 16) | PCI_DEVICE_ID_SI_7018;

const MPU401_HW_TRID4DWAVE: c_int = 0;
const MPU401_INFO_INTEGRATED: c_uint = 1 << 0;
const MPU401_INFO_IRQ_HOOK: c_uint = 1 << 1;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
    pub vendor: c_uint,
    pub device: c_uint,
}

#[repr(C)]
pub struct pci_device_id {
    pub vendor: c_uint,
    pub device: c_uint,
    pub subvendor: c_uint,
    pub subdevice: c_uint,
    pub class: c_uint,
    pub class_mask: c_uint,
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct snd_card {
    pub private_data: *mut c_void,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
}

#[repr(C)]
pub struct snd_trident {
    pub device: c_uint,
    pub port: c_ulong,
    pub irq: c_int,
    pub midi_port: c_ulong,
    pub rmidi: *mut c_void,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct pci_driver {
    pub name: *const c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
    // Present under CONFIG_PM_SLEEP in the C source.
    pub driver: device_driver,
}

unsafe extern "C" {
    static THIS_MODULE: *mut c_void;
    static KBUILD_MODNAME: [c_char; 0];
    static snd_trident_pm: dev_pm_ops;

    fn snd_devm_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut c_void,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn snd_trident_create(
        card: *mut snd_card,
        pci: *mut pci_dev,
        pcm_channels: c_int,
        spdif_device: c_int,
        wavetable_size: c_int,
    ) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snd_trident_pcm(trident: *mut snd_trident, device: c_int) -> c_int;
    fn snd_trident_foldback_pcm(trident: *mut snd_trident, device: c_int) -> c_int;
    fn snd_trident_spdif_pcm(trident: *mut snd_trident, device: c_int) -> c_int;
    fn snd_mpu401_uart_new(
        card: *mut snd_card,
        device: c_int,
        hardware: c_uint,
        port: c_ulong,
        info_flags: c_uint,
        irq: c_int,
        rrawmidi: *mut *mut c_void,
    ) -> c_int;
    fn snd_trident_create_gameport(trident: *mut snd_trident);
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
}

// MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>, <audio@tridentmicro.com>");
// MODULE_DESCRIPTION("Trident 4D-WaveDX/NX & SiS SI7018");
// MODULE_LICENSE("GPL");

static mut index: [c_int; SNDRV_CARDS] = [SNDRV_DEFAULT_IDX_VALUE; SNDRV_CARDS]; /* Index 0-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = [core::ptr::null_mut(); SNDRV_CARDS]; /* ID for this card */
static mut enable: [bool; SNDRV_CARDS] = [SNDRV_DEFAULT_ENABLE_PNP_VALUE; SNDRV_CARDS]; /* Enable this card */
static mut pcm_channels: [c_int; SNDRV_CARDS] = [32; SNDRV_CARDS];
static mut wavetable_size: [c_int; SNDRV_CARDS] = [8192; SNDRV_CARDS];

// module_param_array(index, int, NULL, 0444);
// MODULE_PARM_DESC(index, "Index value for Trident 4DWave PCI soundcard.");
// module_param_array(id, charp, NULL, 0444);
// MODULE_PARM_DESC(id, "ID string for Trident 4DWave PCI soundcard.");
// module_param_array(enable, bool, NULL, 0444);
// MODULE_PARM_DESC(enable, "Enable Trident 4DWave PCI soundcard.");
// module_param_array(pcm_channels, int, NULL, 0444);
// MODULE_PARM_DESC(pcm_channels, "Number of hardware channels assigned for PCM.");
// module_param_array(wavetable_size, int, NULL, 0444);
// MODULE_PARM_DESC(wavetable_size, "Maximum memory size in kB for wavetable synth.");

const fn PCI_DEVICE(vendor: c_uint, device: c_uint) -> pci_device_id {
    pci_device_id {
        vendor,
        device,
        subvendor: !0,
        subdevice: !0,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    }
}

static snd_trident_ids: [pci_device_id; 4] = [
    pci_device_id {
        class: PCI_CLASS_MULTIMEDIA_AUDIO << 8,
        class_mask: 0xffff00,
        ..PCI_DEVICE(PCI_VENDOR_ID_TRIDENT, PCI_DEVICE_ID_TRIDENT_4DWAVE_DX)
    },
    PCI_DEVICE(PCI_VENDOR_ID_TRIDENT, PCI_DEVICE_ID_TRIDENT_4DWAVE_NX),
    PCI_DEVICE(PCI_VENDOR_ID_SI, PCI_DEVICE_ID_SI_7018),
    pci_device_id {
        vendor: 0,
        device: 0,
        subvendor: 0,
        subdevice: 0,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    },
];

// MODULE_DEVICE_TABLE(pci, snd_trident_ids);

unsafe extern "C" fn snd_trident_probe(
    pci: *mut pci_dev,
    pci_id: *const pci_device_id,
) -> c_int {
    static mut dev: c_int = 0;
    let mut card: *mut snd_card = core::ptr::null_mut();
    let mut trident: *mut snd_trident;
    let str_: *const c_char;
    let mut err: c_int;
    let mut pcm_dev: c_int = 0;

    let _ = pci_id;

    if dev >= SNDRV_CARDS as c_int {
        return -ENODEV;
    }
    if !enable[dev as usize] {
        dev += 1;
        return -ENOENT;
    }

    err = snd_devm_card_new(
        &mut (*pci).dev,
        index[dev as usize],
        id[dev as usize],
        THIS_MODULE,
        core::mem::size_of::<snd_trident>(),
        &mut card,
    );
    if err < 0 {
        return err;
    }
    trident = (*card).private_data as *mut snd_trident;

    err = snd_trident_create(
        card,
        pci,
        pcm_channels[dev as usize],
        if (((*pci).vendor << 16) | (*pci).device) == TRIDENT_DEVICE_ID_SI7018 {
            1
        } else {
            2
        },
        wavetable_size[dev as usize],
    );
    if err < 0 {
        return err;
    }

    match (*trident).device {
        TRIDENT_DEVICE_ID_DX => {
            str_ = c"TRID4DWAVEDX".as_ptr();
        }
        TRIDENT_DEVICE_ID_NX => {
            str_ = c"TRID4DWAVENX".as_ptr();
        }
        TRIDENT_DEVICE_ID_SI7018 => {
            str_ = c"SI7018".as_ptr();
        }
        _ => {
            str_ = c"Unknown".as_ptr();
        }
    }
    strscpy((*card).driver.as_mut_ptr(), str_);
    if (*trident).device == TRIDENT_DEVICE_ID_SI7018 {
        strscpy((*card).shortname.as_mut_ptr(), c"SiS ".as_ptr());
    } else {
        strscpy((*card).shortname.as_mut_ptr(), c"Trident ".as_ptr());
    }
    strcat((*card).shortname.as_mut_ptr(), str_);
    sprintf(
        (*card).longname.as_mut_ptr(),
        c"%s PCI Audio at 0x%lx, irq %d".as_ptr(),
        (*card).shortname.as_mut_ptr(),
        (*trident).port as c_ulong,
        (*trident).irq,
    );

    err = snd_trident_pcm(trident, pcm_dev);
    pcm_dev += 1;
    if err < 0 {
        return err;
    }
    match (*trident).device {
        TRIDENT_DEVICE_ID_DX | TRIDENT_DEVICE_ID_NX => {
            err = snd_trident_foldback_pcm(trident, pcm_dev);
            pcm_dev += 1;
            if err < 0 {
                return err;
            }
        }
        _ => {}
    }
    if (*trident).device == TRIDENT_DEVICE_ID_NX || (*trident).device == TRIDENT_DEVICE_ID_SI7018 {
        err = snd_trident_spdif_pcm(trident, pcm_dev);
        pcm_dev += 1;
        if err < 0 {
            return err;
        }
    }
    if (*trident).device != TRIDENT_DEVICE_ID_SI7018 {
        err = snd_mpu401_uart_new(
            card,
            0,
            MPU401_HW_TRID4DWAVE as c_uint,
            (*trident).midi_port,
            MPU401_INFO_INTEGRATED | MPU401_INFO_IRQ_HOOK,
            -1,
            &mut (*trident).rmidi,
        );
        if err < 0 {
            return err;
        }
    }

    snd_trident_create_gameport(trident);

    err = snd_card_register(card);
    if err < 0 {
        return err;
    }
    pci_set_drvdata(pci, card as *mut c_void);
    dev += 1;
    0
}

static mut trident_driver: pci_driver = pci_driver {
    name: unsafe { KBUILD_MODNAME.as_ptr() },
    id_table: snd_trident_ids.as_ptr(),
    probe: Some(snd_trident_probe),
    // #ifdef CONFIG_PM_SLEEP
    driver: device_driver {
        pm: unsafe { &snd_trident_pm },
    },
    // #endif
};

// module_pci_driver(trident_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
