// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018-2022 Intel Corporation
//
// Author: Ranjani Sridharan <ranjani.sridharan@linux.intel.com>
//

// C includes translated as external dependency intent:
// <linux/module.h>
// <linux/pci.h>
// <sound/soc-acpi.h>
// <sound/soc-acpi-intel-match.h>
// <sound/sof.h>
// "../ops.h"
// "../sof-pci-dev.h"
//
// platform specific devices:
// "hda.h"
// "mtl.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_void};

const SOF_IPC_TYPE_4: usize = 4;
const SOF_IPC_TYPE_COUNT: usize = SOF_IPC_TYPE_4 + 1;

const fn BIT(nr: usize) -> u32 {
    1u32 << nr
}

#[repr(C)]
pub struct snd_sof_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_dsp_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sof_dev_desc {
    pub use_acpi_target_states: bool,
    pub machines: *const c_void,
    pub alt_machines: *const c_void,
    pub resindex_lpe_base: c_int,
    pub resindex_pcicfg_base: c_int,
    pub resindex_imr_base: c_int,
    pub irqindex_host_ipc: c_int,
    pub chip_info: *const c_void,
    pub ipc_supported_mask: u32,
    pub ipc_default: usize,
    pub dspless_mode_supported: bool,
    pub default_fw_path: [*const c_char; SOF_IPC_TYPE_COUNT],
    pub default_lib_path: [*const c_char; SOF_IPC_TYPE_COUNT],
    pub default_tplg_path: [*const c_char; SOF_IPC_TYPE_COUNT],
    pub default_fw_filename: [*const c_char; SOF_IPC_TYPE_COUNT],
    pub nocodec_tplg_filename: *const c_char,
    pub ops: *mut snd_sof_dsp_ops,
    pub ops_init: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub ops_free: Option<unsafe extern "C" fn(*mut snd_sof_dev)>,
}

#[repr(C)]
pub struct pci_device_id {
    pub vendor: u32,
    pub device: u32,
    pub subvendor: u32,
    pub subdevice: u32,
    pub class: u32,
    pub class_mask: u32,
    pub driver_data: usize,
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
    pub probe: Option<unsafe extern "C" fn(*mut c_void, *const pci_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut c_void)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut c_void)>,
    pub driver: device_driver,
}

const PCI_VENDOR_ID_INTEL: u32 = 0x8086;
const PCI_DEVICE_ID_INTEL_HDA_MTL: u32 = 0x7e28;
const PCI_DEVICE_ID_INTEL_HDA_ARL_S: u32 = 0x7f50;
const PCI_DEVICE_ID_INTEL_HDA_ARL: u32 = 0x7728;

const PCI_ANY_ID: u32 = !0u32;

const fn PCI_DEVICE_DATA_INTEL(device: u32, data: *const sof_dev_desc) -> pci_device_id {
    pci_device_id {
        vendor: PCI_VENDOR_ID_INTEL,
        device,
        subvendor: PCI_ANY_ID,
        subdevice: PCI_ANY_ID,
        class: 0,
        class_mask: 0,
        driver_data: data as usize,
    }
}

unsafe extern "C" {
    static snd_soc_acpi_intel_mtl_machines: c_void;
    static snd_soc_acpi_intel_mtl_sdw_machines: c_void;
    static snd_soc_acpi_intel_arl_machines: c_void;
    static snd_soc_acpi_intel_arl_sdw_machines: c_void;
    static mtl_chip_info: c_void;
    static arl_s_chip_info: c_void;
    static sof_pci_pm: dev_pm_ops;

    fn sof_mtl_set_ops(sdev: *mut snd_sof_dev, ops: *mut snd_sof_dsp_ops) -> c_int;
    fn hda_ops_free(sdev: *mut snd_sof_dev);
    fn hda_pci_intel_probe(pdev: *mut c_void, id: *const pci_device_id) -> c_int;
    fn sof_pci_remove(pdev: *mut c_void);
    fn sof_pci_shutdown(pdev: *mut c_void);
}

// Meteorlake ops
static mut sof_mtl_ops: snd_sof_dsp_ops = snd_sof_dsp_ops { _private: [] };

unsafe extern "C" fn sof_mtl_ops_init(sdev: *mut snd_sof_dev) -> c_int {
    unsafe { sof_mtl_set_ops(sdev, &raw mut sof_mtl_ops) }
}

static mtl_desc: sof_dev_desc = sof_dev_desc {
    use_acpi_target_states: true,
    machines: unsafe { &snd_soc_acpi_intel_mtl_machines as *const c_void },
    alt_machines: unsafe { &snd_soc_acpi_intel_mtl_sdw_machines as *const c_void },
    resindex_lpe_base: 0,
    resindex_pcicfg_base: -1,
    resindex_imr_base: -1,
    irqindex_host_ipc: -1,
    chip_info: unsafe { &mtl_chip_info as *const c_void },
    ipc_supported_mask: BIT(SOF_IPC_TYPE_4),
    ipc_default: SOF_IPC_TYPE_4,
    dspless_mode_supported: true, // Only supported for HDaudio
    default_fw_path: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        c"intel/sof-ipc4/mtl".as_ptr(),
    ],
    default_lib_path: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        c"intel/sof-ipc4-lib/mtl".as_ptr(),
    ],
    default_tplg_path: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        c"intel/sof-ipc4-tplg".as_ptr(),
    ],
    default_fw_filename: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        c"sof-mtl.ri".as_ptr(),
    ],
    nocodec_tplg_filename: c"sof-mtl-nocodec.tplg".as_ptr(),
    ops: &raw mut sof_mtl_ops,
    ops_init: Some(sof_mtl_ops_init),
    ops_free: Some(hda_ops_free),
};

static arl_desc: sof_dev_desc = sof_dev_desc {
    use_acpi_target_states: true,
    machines: unsafe { &snd_soc_acpi_intel_arl_machines as *const c_void },
    alt_machines: unsafe { &snd_soc_acpi_intel_arl_sdw_machines as *const c_void },
    resindex_lpe_base: 0,
    resindex_pcicfg_base: -1,
    resindex_imr_base: -1,
    irqindex_host_ipc: -1,
    chip_info: unsafe { &mtl_chip_info as *const c_void },
    ipc_supported_mask: BIT(SOF_IPC_TYPE_4),
    ipc_default: SOF_IPC_TYPE_4,
    dspless_mode_supported: true, // Only supported for HDaudio
    default_fw_path: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        c"intel/sof-ipc4/arl".as_ptr(),
    ],
    default_lib_path: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        c"intel/sof-ipc4-lib/arl".as_ptr(),
    ],
    default_tplg_path: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        c"intel/sof-ipc4-tplg".as_ptr(),
    ],
    default_fw_filename: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        c"sof-arl.ri".as_ptr(),
    ],
    nocodec_tplg_filename: c"sof-arl-nocodec.tplg".as_ptr(),
    ops: &raw mut sof_mtl_ops,
    ops_init: Some(sof_mtl_ops_init),
    ops_free: Some(hda_ops_free),
};

static arl_s_desc: sof_dev_desc = sof_dev_desc {
    use_acpi_target_states: true,
    machines: unsafe { &snd_soc_acpi_intel_arl_machines as *const c_void },
    alt_machines: unsafe { &snd_soc_acpi_intel_arl_sdw_machines as *const c_void },
    resindex_lpe_base: 0,
    resindex_pcicfg_base: -1,
    resindex_imr_base: -1,
    irqindex_host_ipc: -1,
    chip_info: unsafe { &arl_s_chip_info as *const c_void },
    ipc_supported_mask: BIT(SOF_IPC_TYPE_4),
    ipc_default: SOF_IPC_TYPE_4,
    dspless_mode_supported: true, // Only supported for HDaudio
    default_fw_path: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        c"intel/sof-ipc4/arl-s".as_ptr(),
    ],
    default_lib_path: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        c"intel/sof-ipc4-lib/arl-s".as_ptr(),
    ],
    default_tplg_path: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        c"intel/sof-ipc4-tplg".as_ptr(),
    ],
    default_fw_filename: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        c"sof-arl-s.ri".as_ptr(),
    ],
    nocodec_tplg_filename: c"sof-arl-nocodec.tplg".as_ptr(),
    ops: &raw mut sof_mtl_ops,
    ops_init: Some(sof_mtl_ops_init),
    ops_free: Some(hda_ops_free),
};

// PCI IDs
static sof_pci_ids: [pci_device_id; 4] = [
    PCI_DEVICE_DATA_INTEL(PCI_DEVICE_ID_INTEL_HDA_MTL, &mtl_desc),
    PCI_DEVICE_DATA_INTEL(PCI_DEVICE_ID_INTEL_HDA_ARL_S, &arl_s_desc),
    PCI_DEVICE_DATA_INTEL(PCI_DEVICE_ID_INTEL_HDA_ARL, &arl_desc),
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

// pci_driver definition
static mut snd_sof_pci_intel_mtl_driver: pci_driver = pci_driver {
    name: c"sof-audio-pci-intel-mtl".as_ptr(),
    id_table: sof_pci_ids.as_ptr(),
    probe: Some(hda_pci_intel_probe),
    remove: Some(sof_pci_remove),
    shutdown: Some(sof_pci_shutdown),
    driver: device_driver {
        pm: unsafe { &sof_pci_pm as *const dev_pm_ops },
    },
};
// module_pci_driver(snd_sof_pci_intel_mtl_driver);

// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("SOF support for MeteorLake platforms");
// MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_HDA_GENERIC");
// MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_HDA_COMMON");
// MODULE_IMPORT_NS("SND_SOC_SOF_PCI_DEV");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
