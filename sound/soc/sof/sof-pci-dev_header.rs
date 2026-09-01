/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2021 Intel Corporation
 */

// C header guard omitted in Rust: __SOUND_SOC_SOF_PCI_H.

unsafe extern "C" {
    pub static sof_pci_pm: dev_pm_ops;

    pub fn sof_pci_probe(
        pci: *mut pci_dev,
        pci_id: *const pci_device_id,
    ) -> core::ffi::c_int;

    pub fn sof_pci_remove(pci: *mut pci_dev);

    pub fn sof_pci_shutdown(pci: *mut pci_dev);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
