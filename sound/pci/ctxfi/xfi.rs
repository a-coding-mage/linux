// SPDX-License-Identifier: GPL-2.0-only
/*
 * xfi linux driver.
 *
 * Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

/* Dependencies from:
 * <linux/init.h>, <linux/pci.h>, <linux/moduleparam.h>,
 * <linux/pci_ids.h>, <linux/module.h>, <sound/core.h>,
 * <sound/initval.h>, "ctatc.h", and "cthardware.h".
 */

type bool_ = bool;

const SNDRV_CARDS: usize = 32;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;

const PCI_VENDOR_ID_CREATIVE: c_uint = 0x1102;
const PCI_DEVICE_ID_CREATIVE_20K1: c_uint = 0x0005;
const PCI_DEVICE_ID_CREATIVE_20K2: c_uint = 0x000b;
const PCI_ANY_ID: c_uint = !0u32;

extern "C" {
    static THIS_MODULE: *mut module;
    static KBUILD_MODNAME: [c_char; 0];

    fn snd_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut module,
        extra_size: c_int,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn snd_card_free(card: *mut snd_card);
    fn snd_card_register(card: *mut snd_card) -> c_int;

    fn ct_atc_create(
        card: *mut snd_card,
        pci: *mut pci_dev,
        reference_rate: c_uint,
        multiple: c_uint,
        chip_type: c_ulong,
        subsystem: c_uint,
        atc_ret: *mut *mut ct_atc,
    ) -> c_int;
    fn ct_atc_create_alsa_devs(atc: *mut ct_atc) -> c_int;

    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn pci_get_drvdata(pci: *mut pci_dev) -> *mut c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub private_data: *mut c_void,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
}

#[repr(C)]
pub struct ct_atc {
    pub chip_name: *const c_char,
    pub model_name: *const c_char,
    pub suspend: unsafe extern "C" fn(*mut ct_atc) -> c_int,
    pub resume: unsafe extern "C" fn(*mut ct_atc) -> c_int,
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
pub struct dev_pm_ops {
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
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
    pub remove: Option<unsafe extern "C" fn(*mut pci_dev)>,
    pub driver: device_driver,
}

const ATC20K1: c_ulong = 0;
const ATC20K2: c_ulong = 1;

/* MODULE_AUTHOR("Creative Technology Ltd");
 * MODULE_DESCRIPTION("X-Fi driver version 1.03");
 * MODULE_LICENSE("GPL v2");
 */

static mut reference_rate: c_uint = 48000;
static mut multiple: c_uint = 2;
/* MODULE_PARM_DESC(reference_rate, "Reference rate (default=48000)");
 * module_param(reference_rate, uint, 0444);
 * MODULE_PARM_DESC(multiple, "Rate multiplier (default=2)");
 * module_param(multiple, uint, 0444);
 */

static mut index: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
static mut id: [*mut c_char; SNDRV_CARDS] = [core::ptr::null_mut(); SNDRV_CARDS];
static mut enable: [bool_; SNDRV_CARDS] = [true; SNDRV_CARDS];
static mut subsystem: [c_uint; SNDRV_CARDS] = [0; SNDRV_CARDS];

/* module_param_array(index, int, NULL, 0444);
 * MODULE_PARM_DESC(index, "Index value for Creative X-Fi driver");
 * module_param_array(id, charp, NULL, 0444);
 * MODULE_PARM_DESC(id, "ID string for Creative X-Fi driver");
 * module_param_array(enable, bool, NULL, 0444);
 * MODULE_PARM_DESC(enable, "Enable Creative X-Fi driver");
 * module_param_array(subsystem, int, NULL, 0444);
 * MODULE_PARM_DESC(subsystem, "Override subsystem ID for Creative X-Fi driver");
 */

static ct_pci_dev_ids: [pci_device_id; 3] = [
    pci_device_id {
        /* only X-Fi is supported, so... */
        vendor: PCI_VENDOR_ID_CREATIVE,
        device: PCI_DEVICE_ID_CREATIVE_20K1,
        subvendor: PCI_ANY_ID,
        subdevice: PCI_ANY_ID,
        class: 0,
        class_mask: 0,
        driver_data: ATC20K1,
    },
    pci_device_id {
        vendor: PCI_VENDOR_ID_CREATIVE,
        device: PCI_DEVICE_ID_CREATIVE_20K2,
        subvendor: PCI_ANY_ID,
        subdevice: PCI_ANY_ID,
        class: 0,
        class_mask: 0,
        driver_data: ATC20K2,
    },
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
/* MODULE_DEVICE_TABLE(pci, ct_pci_dev_ids); */

static mut ct_card_probe_dev: c_int = 0;

unsafe extern "C" fn ct_card_probe(
    pci: *mut pci_dev,
    pci_id: *const pci_device_id,
) -> c_int {
    let mut card: *mut snd_card = core::ptr::null_mut();
    let mut atc: *mut ct_atc = core::ptr::null_mut();
    let mut err: c_int;

    if ct_card_probe_dev >= SNDRV_CARDS as c_int {
        return -ENODEV;
    }

    if !enable[ct_card_probe_dev as usize] {
        ct_card_probe_dev += 1;
        return -ENOENT;
    }

    err = snd_card_new(
        &mut (*pci).dev,
        index[ct_card_probe_dev as usize],
        id[ct_card_probe_dev as usize],
        THIS_MODULE,
        0,
        &mut card,
    );
    if err != 0 {
        return err;
    }

    if reference_rate != 48000 && reference_rate != 44100 {
        dev_err(
            (*card).dev,
            b"Invalid reference_rate value %u!!!\n\0".as_ptr() as *const c_char,
            reference_rate,
        );
        dev_err(
            (*card).dev,
            b"The valid values for reference_rate are 48000 and 44100, Value 48000 is assumed.\n\0"
                .as_ptr() as *const c_char,
        );
        reference_rate = 48000;
    }

    if multiple != 1 && multiple != 2 && multiple != 4 {
        dev_err(
            (*card).dev,
            b"Invalid multiple value %u!!!\n\0".as_ptr() as *const c_char,
            multiple,
        );
        dev_err(
            (*card).dev,
            b"The valid values for multiple are 1, 2 and 4, Value 2 is assumed.\n\0".as_ptr()
                as *const c_char,
        );
        multiple = 2;
    }

    err = ct_atc_create(
        card,
        pci,
        reference_rate,
        multiple,
        (*pci_id).driver_data,
        subsystem[ct_card_probe_dev as usize],
        &mut atc,
    );
    if err < 0 {
        goto_error(card, err)
    } else {
        (*card).private_data = atc as *mut c_void;

        /* Create alsa devices supported by this card */
        err = ct_atc_create_alsa_devs(atc);
        if err < 0 {
            goto_error(card, err)
        } else {
            strscpy(
                (*card).driver.as_mut_ptr(),
                b"SB-XFi\0".as_ptr() as *const c_char,
            );
            strscpy(
                (*card).shortname.as_mut_ptr(),
                b"Creative X-Fi\0".as_ptr() as *const c_char,
            );
            snprintf(
                (*card).longname.as_mut_ptr(),
                core::mem::size_of_val(&(*card).longname),
                b"%s %s %s\0".as_ptr() as *const c_char,
                (*card).shortname.as_ptr(),
                (*atc).chip_name,
                (*atc).model_name,
            );

            err = snd_card_register(card);
            if err < 0 {
                goto_error(card, err)
            } else {
                pci_set_drvdata(pci, card as *mut c_void);
                ct_card_probe_dev += 1;

                0
            }
        }
    }
}

unsafe fn goto_error(card: *mut snd_card, err: c_int) -> c_int {
    snd_card_free(card);
    err
}

unsafe extern "C" fn ct_card_remove(pci: *mut pci_dev) {
    snd_card_free(pci_get_drvdata(pci) as *mut snd_card);
}

/* #ifdef CONFIG_PM_SLEEP */
unsafe extern "C" fn ct_card_suspend(dev: *mut device) -> c_int {
    let card: *mut snd_card = dev_get_drvdata(dev) as *mut snd_card;
    let atc: *mut ct_atc = (*card).private_data as *mut ct_atc;

    ((*atc).suspend)(atc)
}

unsafe extern "C" fn ct_card_resume(dev: *mut device) -> c_int {
    let card: *mut snd_card = dev_get_drvdata(dev) as *mut snd_card;
    let atc: *mut ct_atc = (*card).private_data as *mut ct_atc;

    ((*atc).resume)(atc)
}

/* static SIMPLE_DEV_PM_OPS(ct_card_pm, ct_card_suspend, ct_card_resume);
 * #define CT_CARD_PM_OPS &ct_card_pm
 * #else
 * #define CT_CARD_PM_OPS NULL
 * #endif
 */
static ct_card_pm: dev_pm_ops = dev_pm_ops {
    suspend: Some(ct_card_suspend),
    resume: Some(ct_card_resume),
};
const CT_CARD_PM_OPS: *const dev_pm_ops = &ct_card_pm;

static mut ct_driver: pci_driver = pci_driver {
    name: unsafe { KBUILD_MODNAME.as_ptr() },
    id_table: ct_pci_dev_ids.as_ptr(),
    probe: Some(ct_card_probe),
    remove: Some(ct_card_remove),
    driver: device_driver { pm: CT_CARD_PM_OPS },
};

/* module_pci_driver(ct_driver); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
