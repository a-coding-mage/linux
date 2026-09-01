// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2023 Intel Corporation
//
// Author: Ranjani Sridharan <ranjani.sridharan@linux.intel.com>
//

// C includes translated as external dependencies:
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
// "lnl.h"

use core::ffi::{c_char, c_int, c_ulong, c_void};

type bool_ = bool;
type kernel_ulong_t = c_ulong;

const SOF_IPC_TYPE_4: usize = 4;
const SOF_IPC_TYPE_COUNT: usize = SOF_IPC_TYPE_4 + 1;

const PCI_VENDOR_ID_INTEL: u32 = 0x8086;
const PCI_ANY_ID: u32 = !0u32;
const HDA_LNL_P: u32 = 0xA828;

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
pub struct snd_soc_acpi_mach {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sof_intel_dsp_desc {
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
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_device_id {
    pub vendor: u32,
    pub device: u32,
    pub subvendor: u32,
    pub subdevice: u32,
    pub class: u32,
    pub class_mask: u32,
    pub driver_data: kernel_ulong_t,
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
    pub use_acpi_target_states: bool_,
    pub machines: *const snd_soc_acpi_mach,
    pub alt_machines: *const snd_soc_acpi_mach,
    pub resindex_lpe_base: c_int,
    pub resindex_pcicfg_base: c_int,
    pub resindex_imr_base: c_int,
    pub irqindex_host_ipc: c_int,
    pub chip_info: *const sof_intel_dsp_desc,
    pub ipc_supported_mask: u32,
    pub ipc_default: c_int,
    pub dspless_mode_supported: bool_,
    pub on_demand_dsp_boot: bool_,
    pub default_fw_path: [*const c_char; SOF_IPC_TYPE_COUNT],
    pub default_lib_path: [*const c_char; SOF_IPC_TYPE_COUNT],
    pub default_tplg_path: [*const c_char; SOF_IPC_TYPE_COUNT],
    pub default_fw_filename: [*const c_char; SOF_IPC_TYPE_COUNT],
    pub nocodec_tplg_filename: *const c_char,
    pub ops: *mut snd_sof_dsp_ops,
    pub ops_init: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
}

unsafe extern "C" {
    static snd_soc_acpi_intel_lnl_machines: [snd_soc_acpi_mach; 0];
    static snd_soc_acpi_intel_lnl_sdw_machines: [snd_soc_acpi_mach; 0];
    static lnl_chip_info: sof_intel_dsp_desc;
    static sof_pci_pm: dev_pm_ops;

    fn sof_lnl_set_ops(sdev: *mut snd_sof_dev, ops: *mut snd_sof_dsp_ops) -> c_int;
    fn hda_pci_intel_probe(pdev: *mut pci_dev, pci_id: *const pci_device_id) -> c_int;
    fn sof_pci_remove(pdev: *mut pci_dev);
    fn sof_pci_shutdown(pdev: *mut pci_dev);
}

const fn c_str(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

const fn pm_ptr(pm: *const dev_pm_ops) -> *const dev_pm_ops {
    pm
}

/* LunarLake ops */
static mut sof_lnl_ops: snd_sof_dsp_ops = snd_sof_dsp_ops { _private: [] };

unsafe extern "C" fn sof_lnl_ops_init(sdev: *mut snd_sof_dev) -> c_int {
    unsafe { sof_lnl_set_ops(sdev, &raw mut sof_lnl_ops) }
}

static lnl_desc: sof_dev_desc = sof_dev_desc {
    use_acpi_target_states: true,
    machines: unsafe { snd_soc_acpi_intel_lnl_machines.as_ptr() },
    alt_machines: unsafe { snd_soc_acpi_intel_lnl_sdw_machines.as_ptr() },
    resindex_lpe_base: 0,
    resindex_pcicfg_base: -1,
    resindex_imr_base: -1,
    irqindex_host_ipc: -1,
    chip_info: unsafe { &lnl_chip_info },
    ipc_supported_mask: BIT(SOF_IPC_TYPE_4),
    ipc_default: SOF_IPC_TYPE_4 as c_int,
    dspless_mode_supported: true,
    on_demand_dsp_boot: true,
    default_fw_path: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        c_str(b"intel/sof-ipc4/lnl\0"),
    ],
    default_lib_path: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        c_str(b"intel/sof-ipc4-lib/lnl\0"),
    ],
    default_tplg_path: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        c_str(b"intel/sof-ipc4-tplg\0"),
    ],
    default_fw_filename: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        c_str(b"sof-lnl.ri\0"),
    ],
    nocodec_tplg_filename: c_str(b"sof-lnl-nocodec.tplg\0"),
    ops: unsafe { &raw mut sof_lnl_ops },
    ops_init: Some(sof_lnl_ops_init),
};

/* PCI IDs */
static sof_pci_ids: [pci_device_id; 2] = [
    pci_device_id {
        vendor: PCI_VENDOR_ID_INTEL,
        device: HDA_LNL_P,
        subvendor: PCI_ANY_ID,
        subdevice: PCI_ANY_ID,
        class: 0,
        class_mask: 0,
        driver_data: &lnl_desc as *const sof_dev_desc as kernel_ulong_t,
    }, /* LNL-P */
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

/* pci_driver definition */
static mut snd_sof_pci_intel_lnl_driver: pci_driver = pci_driver {
    name: c_str(b"sof-audio-pci-intel-lnl\0"),
    id_table: sof_pci_ids.as_ptr(),
    probe: Some(hda_pci_intel_probe),
    remove: Some(sof_pci_remove),
    shutdown: Some(sof_pci_shutdown),
    driver: device_driver {
        pm: unsafe { pm_ptr(&sof_pci_pm) },
    },
};
// module_pci_driver(snd_sof_pci_intel_lnl_driver);

// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("SOF support for LunarLake platforms");
// MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_HDA_GENERIC");
// MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_HDA_COMMON");
// MODULE_IMPORT_NS("SND_SOC_SOF_PCI_DEV");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
