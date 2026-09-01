// SPDX-License-Identifier: GPL-2.0-only
/*
 * C-Media CMI8788 driver for Asus Xonar cards
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 */

// C dependencies:
// #include <linux/pci.h>
// #include <linux/delay.h>
// #include <linux/module.h>
// #include <sound/core.h>
// #include <sound/initval.h>
// #include <sound/pcm.h>
// #include "xonar.h"

use core::ffi::{c_char, c_int};

module_author!("Clemens Ladisch <clemens@ladisch.de>");
module_description!("Asus Virtuoso driver");
module_license!("GPL v2");

extern "C" {
    static THIS_MODULE: *mut module;
    static oxygen_pci_pm: dev_pm_ops;

    fn get_xonar_pcm179x_model(chip: *mut oxygen, id: *const pci_device_id) -> c_int;
    fn get_xonar_cs43xx_model(chip: *mut oxygen, id: *const pci_device_id) -> c_int;
    fn get_xonar_wm87x6_model(chip: *mut oxygen, id: *const pci_device_id) -> c_int;
    fn oxygen_pci_probe(
        pci: *mut pci_dev,
        index: c_int,
        id: *mut c_char,
        owner: *mut module,
        ids: *const pci_device_id,
        get_model: unsafe extern "C" fn(*mut oxygen, *const pci_device_id) -> c_int,
    ) -> c_int;
    fn oxygen_pci_shutdown(pci: *mut pci_dev);
}

static mut index: [c_int; SNDRV_CARDS as usize] = SNDRV_DEFAULT_IDX;
static mut id: [*mut c_char; SNDRV_CARDS as usize] = SNDRV_DEFAULT_STR;
static mut enable: [bool; SNDRV_CARDS as usize] = SNDRV_DEFAULT_ENABLE_PNP;

module_param_array!(index, int, NULL, 0o444);
module_parm_desc!(index, "card index");
module_param_array!(id, charp, NULL, 0o444);
module_parm_desc!(id, "ID string");
module_param_array!(enable, bool, NULL, 0o444);
module_parm_desc!(enable, "enable card");

static xonar_ids: [pci_device_id; 15] = [
    OXYGEN_PCI_SUBID!(0x1043, 0x8269),
    OXYGEN_PCI_SUBID!(0x1043, 0x8275),
    OXYGEN_PCI_SUBID!(0x1043, 0x82b7),
    OXYGEN_PCI_SUBID!(0x1043, 0x8314),
    OXYGEN_PCI_SUBID!(0x1043, 0x8327),
    OXYGEN_PCI_SUBID!(0x1043, 0x834f),
    OXYGEN_PCI_SUBID!(0x1043, 0x835c),
    OXYGEN_PCI_SUBID!(0x1043, 0x835d),
    OXYGEN_PCI_SUBID!(0x1043, 0x835e),
    OXYGEN_PCI_SUBID!(0x1043, 0x838e),
    OXYGEN_PCI_SUBID!(0x1043, 0x8428),
    OXYGEN_PCI_SUBID!(0x1043, 0x8522),
    OXYGEN_PCI_SUBID!(0x1043, 0x85f4),
    OXYGEN_PCI_SUBID_BROKEN_EEPROM!(),
    pci_device_id {},
];
module_device_table!(pci, xonar_ids);

unsafe extern "C" fn get_xonar_model(chip: *mut oxygen, id: *const pci_device_id) -> c_int {
    if get_xonar_pcm179x_model(chip, id) >= 0 {
        return 0;
    }
    if get_xonar_cs43xx_model(chip, id) >= 0 {
        return 0;
    }
    if get_xonar_wm87x6_model(chip, id) >= 0 {
        return 0;
    }
    -EINVAL
}

unsafe extern "C" fn xonar_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    static mut dev: c_int = 0;
    let err: c_int;

    if dev >= SNDRV_CARDS {
        return -ENODEV;
    }
    if !enable[dev as usize] {
        dev += 1;
        return -ENOENT;
    }
    err = oxygen_pci_probe(
        pci,
        index[dev as usize],
        id[dev as usize],
        THIS_MODULE,
        xonar_ids.as_ptr(),
        get_xonar_model,
    );
    if err >= 0 {
        dev += 1;
    }
    err
}

static mut xonar_driver: pci_driver = pci_driver {
    name: KBUILD_MODNAME,
    id_table: xonar_ids.as_ptr(),
    probe: Some(xonar_probe),
    driver: device_driver {
        pm: pm_sleep_ptr!(&oxygen_pci_pm),
    },
    shutdown: Some(oxygen_pci_shutdown),
};

module_pci_driver!(xonar_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
