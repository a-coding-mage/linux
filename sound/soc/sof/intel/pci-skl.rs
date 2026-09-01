// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018-2022 Intel Corporation
//

// C dependencies:
// #include <linux/module.h>
// #include <linux/pci.h>
// #include <sound/soc-acpi.h>
// #include <sound/soc-acpi-intel-match.h>
// #include <sound/sof.h>
// #include "../ops.h"
// #include "../sof-pci-dev.h"
//
// platform specific devices:
// #include "hda.h"

use core::ffi::{c_char, c_int, c_uint, c_ulong};

const SOF_IPC_TYPE_4: usize = 4;
const SOF_IPC_TYPE_COUNT: usize = 8;

const PCI_VENDOR_ID_INTEL: u32 = 0x8086;
const PCI_DEVICE_ID_INTEL_HDA_SKL_LP: u32 = 0x9d70;
const PCI_DEVICE_ID_INTEL_HDA_KBL_LP: u32 = 0x9d71;

const fn bit(nr: usize) -> c_uint {
    1u32.wrapping_shl(nr as u32) as c_uint
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sof_intel_dsp_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_dsp_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
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
pub struct pci_device_id {
    pub vendor: u32,
    pub device: u32,
    pub subvendor: u32,
    pub subdevice: u32,
    pub class: u32,
    pub class_mask: u32,
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct pci_driver {
    pub name: *const c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut pci_dev)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut pci_dev)>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct sof_dev_desc {
    pub machines: *const snd_soc_acpi_mach,
    pub resindex_lpe_base: c_int,
    pub resindex_pcicfg_base: c_int,
    pub resindex_imr_base: c_int,
    pub chip_info: *const sof_intel_dsp_desc,
    pub irqindex_host_ipc: c_int,
    pub ipc_supported_mask: c_uint,
    pub ipc_default: c_uint,
    pub dspless_mode_supported: bool,
    pub default_fw_path: [*const c_char; SOF_IPC_TYPE_COUNT],
    pub default_tplg_path: [*const c_char; SOF_IPC_TYPE_COUNT],
    pub default_fw_filename: [*const c_char; SOF_IPC_TYPE_COUNT],
    pub nocodec_tplg_filename: *const c_char,
    pub ops: *const snd_sof_dsp_ops,
    pub ops_init: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub ops_free: Option<unsafe extern "C" fn(*mut snd_sof_dev)>,
}

unsafe extern "C" {
    static snd_soc_acpi_intel_skl_machines: [snd_soc_acpi_mach; 0];
    static snd_soc_acpi_intel_kbl_machines: [snd_soc_acpi_mach; 0];
    static skl_chip_info: sof_intel_dsp_desc;
    static sof_skl_ops: snd_sof_dsp_ops;
    static sof_pci_pm: dev_pm_ops;

    fn sof_skl_ops_init(sdev: *mut snd_sof_dev) -> c_int;
    fn hda_ops_free(sdev: *mut snd_sof_dev);
    fn hda_pci_intel_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int;
    fn sof_pci_remove(pci: *mut pci_dev);
    fn sof_pci_shutdown(pci: *mut pci_dev);
}

const NULL_STRINGS: [*const c_char; SOF_IPC_TYPE_COUNT] = [core::ptr::null(); SOF_IPC_TYPE_COUNT];

/* platform specific devices */

static mut skl_desc: sof_dev_desc = sof_dev_desc {
    machines: unsafe { snd_soc_acpi_intel_skl_machines.as_ptr() },
    resindex_lpe_base: 0,
    resindex_pcicfg_base: -1,
    resindex_imr_base: -1,
    chip_info: unsafe { &skl_chip_info },
    irqindex_host_ipc: -1,
    ipc_supported_mask: bit(SOF_IPC_TYPE_4),
    ipc_default: SOF_IPC_TYPE_4 as c_uint,
    dspless_mode_supported: true, /* Only supported for HDaudio */
    default_fw_path: {
        let mut paths = NULL_STRINGS;
        paths[SOF_IPC_TYPE_4] = c"intel/avs/skl".as_ptr();
        paths
    },
    default_tplg_path: {
        let mut paths = NULL_STRINGS;
        paths[SOF_IPC_TYPE_4] = c"intel/avs-tplg".as_ptr();
        paths
    },
    default_fw_filename: {
        let mut filenames = NULL_STRINGS;
        filenames[SOF_IPC_TYPE_4] = c"dsp_basefw.bin".as_ptr();
        filenames
    },
    nocodec_tplg_filename: c"sof-skl-nocodec.tplg".as_ptr(),
    ops: unsafe { &sof_skl_ops },
    ops_init: Some(sof_skl_ops_init),
    ops_free: Some(hda_ops_free),
};

static mut kbl_desc: sof_dev_desc = sof_dev_desc {
    machines: unsafe { snd_soc_acpi_intel_kbl_machines.as_ptr() },
    resindex_lpe_base: 0,
    resindex_pcicfg_base: -1,
    resindex_imr_base: -1,
    chip_info: unsafe { &skl_chip_info },
    irqindex_host_ipc: -1,
    ipc_supported_mask: bit(SOF_IPC_TYPE_4),
    ipc_default: SOF_IPC_TYPE_4 as c_uint,
    dspless_mode_supported: true, /* Only supported for HDaudio */
    default_fw_path: {
        let mut paths = NULL_STRINGS;
        paths[SOF_IPC_TYPE_4] = c"intel/avs/kbl".as_ptr();
        paths
    },
    default_tplg_path: {
        let mut paths = NULL_STRINGS;
        paths[SOF_IPC_TYPE_4] = c"intel/avs-tplg".as_ptr();
        paths
    },
    default_fw_filename: {
        let mut filenames = NULL_STRINGS;
        filenames[SOF_IPC_TYPE_4] = c"dsp_basefw.bin".as_ptr();
        filenames
    },
    nocodec_tplg_filename: c"sof-kbl-nocodec.tplg".as_ptr(),
    ops: unsafe { &sof_skl_ops },
    ops_init: Some(sof_skl_ops_init),
    ops_free: Some(hda_ops_free),
};

const fn pci_device_data_intel(device: u32, data: *const sof_dev_desc) -> pci_device_id {
    pci_device_id {
        vendor: PCI_VENDOR_ID_INTEL,
        device,
        subvendor: !0u32,
        subdevice: !0u32,
        class: 0,
        class_mask: 0,
        driver_data: data as c_ulong,
    }
}

/* PCI IDs */
static sof_pci_ids: [pci_device_id; 3] = [
    pci_device_data_intel(PCI_DEVICE_ID_INTEL_HDA_SKL_LP, unsafe { &raw const skl_desc }),
    pci_device_data_intel(PCI_DEVICE_ID_INTEL_HDA_KBL_LP, unsafe { &raw const kbl_desc }),
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
// MODULE_DEVICE_TABLE(pci, sof_pci_ids);

const fn pm_ptr(pm: *const dev_pm_ops) -> *const dev_pm_ops {
    pm
}

/* pci_driver definition */
static snd_sof_pci_intel_skl_driver: pci_driver = pci_driver {
    name: c"sof-audio-pci-intel-skl".as_ptr(),
    id_table: sof_pci_ids.as_ptr(),
    probe: Some(hda_pci_intel_probe),
    remove: Some(sof_pci_remove),
    shutdown: Some(sof_pci_shutdown),
    driver: device_driver {
        pm: pm_ptr(unsafe { &sof_pci_pm }),
    },
};
// module_pci_driver(snd_sof_pci_intel_skl_driver);

// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("SOF support for SkyLake platforms");
// MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_HDA_GENERIC");
// MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_HDA_COMMON");
// MODULE_IMPORT_NS("SND_SOC_SOF_PCI_DEV");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
