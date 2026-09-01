// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018-2021 Intel Corporation
//
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//

// C includes translated as external dependencies:
// <linux/module.h>
// <linux/pci.h>
// <sound/soc-acpi.h>
// <sound/soc-acpi-intel-match.h>
// <sound/sof.h>
// "../ops.h"
// "../sof-pci-dev.h"
// "hda.h" - platform specific devices

use core::ffi::{c_char, c_int, c_ulong, c_void};

const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

#[repr(C)]
pub struct sof_dev_desc {
    pub machines: *const c_void,
    pub alt_machines: *const c_void,
    pub use_acpi_target_states: bool,
    pub resindex_lpe_base: c_int,
    pub resindex_pcicfg_base: c_int,
    pub resindex_imr_base: c_int,
    pub irqindex_host_ipc: c_int,
    pub chip_info: *const c_void,
    pub ipc_supported_mask: u32,
    pub ipc_default: u32,
    pub dspless_mode_supported: bool,
    pub default_fw_path: [*const c_char; SOF_IPC_TYPE_COUNT],
    pub default_lib_path: [*const c_char; SOF_IPC_TYPE_COUNT],
    pub default_tplg_path: [*const c_char; SOF_IPC_TYPE_COUNT],
    pub default_fw_filename: [*const c_char; SOF_IPC_TYPE_COUNT],
    pub nocodec_tplg_filename: *const c_char,
    pub ops: *const c_void,
    pub ops_init: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    pub ops_free: Option<unsafe extern "C" fn(*mut c_void)>,
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
pub struct device_driver {
    pub pm: *const c_void,
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

pub const SOF_IPC_TYPE_3: u32 = 3;
pub const SOF_IPC_TYPE_4: u32 = 4;
pub const SOF_IPC_TYPE_COUNT: usize = 5;

pub const PCI_VENDOR_ID_INTEL: u32 = 0x8086;
pub const PCI_DEVICE_ID_INTEL_HDA_ICL_LP: u32 = 0x34c8;
pub const PCI_DEVICE_ID_INTEL_HDA_ICL_H: u32 = 0x3dc8;
pub const PCI_DEVICE_ID_INTEL_HDA_ICL_N: u32 = 0x38c8;
pub const PCI_DEVICE_ID_INTEL_HDA_JSL_N: u32 = 0x4dc8;

const PCI_ANY_ID: u32 = !0u32;

const fn PCI_DEVICE_DATA_INTEL(device: u32, data: *const sof_dev_desc) -> pci_device_id {
    pci_device_id {
        vendor: PCI_VENDOR_ID_INTEL,
        device,
        subvendor: PCI_ANY_ID,
        subdevice: PCI_ANY_ID,
        class: 0,
        class_mask: 0,
        driver_data: data as c_ulong,
    }
}

extern "C" {
    static snd_soc_acpi_intel_icl_machines: c_void;
    static snd_soc_acpi_intel_icl_sdw_machines: c_void;
    static snd_soc_acpi_intel_jsl_machines: c_void;
    static icl_chip_info: c_void;
    static jsl_chip_info: c_void;
    static sof_icl_ops: c_void;
    static sof_cnl_ops: c_void;
    static sof_pci_pm: c_void;

    fn sof_icl_ops_init(sdev: *mut c_void) -> c_int;
    fn sof_cnl_ops_init(sdev: *mut c_void) -> c_int;
    fn hda_ops_free(sdev: *mut c_void);
    fn hda_pci_intel_probe(pci: *mut c_void, pci_id: *const pci_device_id) -> c_int;
    fn sof_pci_remove(pci: *mut c_void);
    fn sof_pci_shutdown(pci: *mut c_void);
}

/* platform specific devices */

static icl_desc: sof_dev_desc = sof_dev_desc {
    machines: unsafe { &snd_soc_acpi_intel_icl_machines as *const _ as *const c_void },
    alt_machines: unsafe { &snd_soc_acpi_intel_icl_sdw_machines as *const _ as *const c_void },
    use_acpi_target_states: true,
    resindex_lpe_base: 0,
    resindex_pcicfg_base: -1,
    resindex_imr_base: -1,
    irqindex_host_ipc: -1,
    chip_info: unsafe { &icl_chip_info as *const _ as *const c_void },
    ipc_supported_mask: BIT(SOF_IPC_TYPE_3) | BIT(SOF_IPC_TYPE_4),
    ipc_default: SOF_IPC_TYPE_3,
    dspless_mode_supported: true, /* Only supported for HDaudio */
    default_fw_path: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        b"intel/sof\0".as_ptr() as *const c_char,
        b"intel/avs/icl\0".as_ptr() as *const c_char,
    ],
    default_lib_path: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        b"intel/avs-lib/icl\0".as_ptr() as *const c_char,
    ],
    default_tplg_path: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        b"intel/sof-tplg\0".as_ptr() as *const c_char,
        b"intel/avs-tplg\0".as_ptr() as *const c_char,
    ],
    default_fw_filename: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        b"sof-icl.ri\0".as_ptr() as *const c_char,
        b"dsp_basefw.bin\0".as_ptr() as *const c_char,
    ],
    nocodec_tplg_filename: b"sof-icl-nocodec.tplg\0".as_ptr() as *const c_char,
    ops: unsafe { &sof_icl_ops as *const _ as *const c_void },
    ops_init: Some(sof_icl_ops_init),
    ops_free: Some(hda_ops_free),
};

static jsl_desc: sof_dev_desc = sof_dev_desc {
    machines: unsafe { &snd_soc_acpi_intel_jsl_machines as *const _ as *const c_void },
    alt_machines: core::ptr::null(),
    use_acpi_target_states: true,
    resindex_lpe_base: 0,
    resindex_pcicfg_base: -1,
    resindex_imr_base: -1,
    irqindex_host_ipc: -1,
    chip_info: unsafe { &jsl_chip_info as *const _ as *const c_void },
    ipc_supported_mask: BIT(SOF_IPC_TYPE_3) | BIT(SOF_IPC_TYPE_4),
    ipc_default: SOF_IPC_TYPE_3,
    dspless_mode_supported: true, /* Only supported for HDaudio */
    default_fw_path: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        b"intel/sof\0".as_ptr() as *const c_char,
        b"intel/avs/jsl\0".as_ptr() as *const c_char,
    ],
    default_lib_path: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        b"intel/avs-lib/jsl\0".as_ptr() as *const c_char,
    ],
    default_tplg_path: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        b"intel/sof-tplg\0".as_ptr() as *const c_char,
        b"intel/avs-tplg\0".as_ptr() as *const c_char,
    ],
    default_fw_filename: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        b"sof-jsl.ri\0".as_ptr() as *const c_char,
        b"dsp_basefw.bin\0".as_ptr() as *const c_char,
    ],
    nocodec_tplg_filename: b"sof-jsl-nocodec.tplg\0".as_ptr() as *const c_char,
    ops: unsafe { &sof_cnl_ops as *const _ as *const c_void },
    ops_init: Some(sof_cnl_ops_init),
    ops_free: Some(hda_ops_free),
};

/* PCI IDs */
static sof_pci_ids: [pci_device_id; 5] = [
    PCI_DEVICE_DATA_INTEL(PCI_DEVICE_ID_INTEL_HDA_ICL_LP, &icl_desc),
    PCI_DEVICE_DATA_INTEL(PCI_DEVICE_ID_INTEL_HDA_ICL_H, &icl_desc),
    PCI_DEVICE_DATA_INTEL(PCI_DEVICE_ID_INTEL_HDA_ICL_N, &jsl_desc),
    PCI_DEVICE_DATA_INTEL(PCI_DEVICE_ID_INTEL_HDA_JSL_N, &jsl_desc),
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
static mut snd_sof_pci_intel_icl_driver: pci_driver = pci_driver {
    name: b"sof-audio-pci-intel-icl\0".as_ptr() as *const c_char,
    id_table: sof_pci_ids.as_ptr(),
    probe: Some(hda_pci_intel_probe),
    remove: Some(sof_pci_remove),
    shutdown: Some(sof_pci_shutdown),
    driver: device_driver {
        pm: unsafe { &sof_pci_pm as *const _ as *const c_void },
    },
};
// module_pci_driver(snd_sof_pci_intel_icl_driver);

// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("SOF support for IceLake platforms");
// MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_HDA_GENERIC");
// MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_HDA_COMMON");
// MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_CNL");
// MODULE_IMPORT_NS("SND_SOC_SOF_PCI_DEV");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
