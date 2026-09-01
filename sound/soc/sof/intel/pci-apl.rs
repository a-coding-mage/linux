// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018-2021 Intel Corporation
//
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//

// C includes translated as external dependency intent:
// linux/module.h, linux/pci.h, sound/soc-acpi.h,
// sound/soc-acpi-intel-match.h, sound/sof.h, ../ops.h,
// ../sof-pci-dev.h, and platform specific "hda.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int};

const SOF_IPC_TYPE_3: usize = 3;
const SOF_IPC_TYPE_4: usize = 4;
const SOF_IPC_TYPE_COUNT: usize = 5;

const fn BIT(n: usize) -> u32 {
    1u32 << n
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
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sof_dev_desc {
    machines: *const snd_soc_acpi_mach,
    use_acpi_target_states: bool,
    resindex_lpe_base: c_int,
    resindex_pcicfg_base: c_int,
    resindex_imr_base: c_int,
    irqindex_host_ipc: c_int,
    chip_info: *const sof_intel_dsp_desc,
    ipc_supported_mask: u32,
    ipc_default: usize,
    dspless_mode_supported: bool,
    default_fw_path: [*const c_char; SOF_IPC_TYPE_COUNT],
    default_lib_path: [*const c_char; SOF_IPC_TYPE_COUNT],
    default_tplg_path: [*const c_char; SOF_IPC_TYPE_COUNT],
    default_fw_filename: [*const c_char; SOF_IPC_TYPE_COUNT],
    nocodec_tplg_filename: *const c_char,
    ops: *const snd_sof_dsp_ops,
    ops_init: Option<unsafe extern "C" fn(*mut pci_dev) -> c_int>,
    ops_free: Option<unsafe extern "C" fn(*mut pci_dev)>,
}

#[repr(C)]
pub struct pci_device_id {
    vendor: u32,
    device: u32,
    subvendor: u32,
    subdevice: u32,
    class: u32,
    class_mask: u32,
    driver_data: usize,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct pci_driver {
    name: *const c_char,
    id_table: *const pci_device_id,
    probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut pci_dev)>,
    shutdown: Option<unsafe extern "C" fn(*mut pci_dev)>,
    driver: device_driver,
}

unsafe extern "C" {
    static snd_soc_acpi_intel_bxt_machines: snd_soc_acpi_mach;
    static snd_soc_acpi_intel_glk_machines: snd_soc_acpi_mach;
    static apl_chip_info: sof_intel_dsp_desc;
    static sof_apl_ops: snd_sof_dsp_ops;
    static sof_pci_pm: dev_pm_ops;

    fn sof_apl_ops_init(pci: *mut pci_dev) -> c_int;
    fn hda_ops_free(pci: *mut pci_dev);
    fn hda_pci_intel_probe(pci: *mut pci_dev, id: *const pci_device_id) -> c_int;
    fn sof_pci_remove(pci: *mut pci_dev);
    fn sof_pci_shutdown(pci: *mut pci_dev);
}

const fn pm_ptr(pm: *const dev_pm_ops) -> *const dev_pm_ops {
    pm
}

const PCI_VENDOR_ID_INTEL: u32 = 0x8086;
const PCI_DEVICE_ID_INTEL_HDA_APL: u32 = 0x5a98;
const PCI_DEVICE_ID_INTEL_HDA_GLK: u32 = 0x3198;

const fn PCI_DEVICE_DATA_INTEL(device: u32, desc: *const sof_dev_desc) -> pci_device_id {
    pci_device_id {
        vendor: PCI_VENDOR_ID_INTEL,
        device,
        subvendor: !0,
        subdevice: !0,
        class: 0,
        class_mask: 0,
        driver_data: desc as usize,
    }
}

/* platform specific devices */

static bxt_desc: sof_dev_desc = sof_dev_desc {
    machines: unsafe { &snd_soc_acpi_intel_bxt_machines },
    use_acpi_target_states: true,
    resindex_lpe_base: 0,
    resindex_pcicfg_base: -1,
    resindex_imr_base: -1,
    irqindex_host_ipc: -1,
    chip_info: unsafe { &apl_chip_info },
    ipc_supported_mask: BIT(SOF_IPC_TYPE_3) | BIT(SOF_IPC_TYPE_4),
    ipc_default: SOF_IPC_TYPE_3,
    dspless_mode_supported: true, /* Only supported for HDaudio */
    default_fw_path: {
        let mut default_fw_path = [core::ptr::null(); SOF_IPC_TYPE_COUNT];
        default_fw_path[SOF_IPC_TYPE_3] = b"intel/sof\0".as_ptr() as *const c_char;
        default_fw_path[SOF_IPC_TYPE_4] = b"intel/avs/apl\0".as_ptr() as *const c_char;
        default_fw_path
    },
    default_lib_path: {
        let mut default_lib_path = [core::ptr::null(); SOF_IPC_TYPE_COUNT];
        default_lib_path[SOF_IPC_TYPE_4] = b"intel/avs-lib/apl\0".as_ptr() as *const c_char;
        default_lib_path
    },
    default_tplg_path: {
        let mut default_tplg_path = [core::ptr::null(); SOF_IPC_TYPE_COUNT];
        default_tplg_path[SOF_IPC_TYPE_3] = b"intel/sof-tplg\0".as_ptr() as *const c_char;
        default_tplg_path[SOF_IPC_TYPE_4] = b"intel/avs-tplg\0".as_ptr() as *const c_char;
        default_tplg_path
    },
    default_fw_filename: {
        let mut default_fw_filename = [core::ptr::null(); SOF_IPC_TYPE_COUNT];
        default_fw_filename[SOF_IPC_TYPE_3] = b"sof-apl.ri\0".as_ptr() as *const c_char;
        default_fw_filename[SOF_IPC_TYPE_4] = b"dsp_basefw.bin\0".as_ptr() as *const c_char;
        default_fw_filename
    },
    nocodec_tplg_filename: b"sof-apl-nocodec.tplg\0".as_ptr() as *const c_char,
    ops: unsafe { &sof_apl_ops },
    ops_init: Some(sof_apl_ops_init),
    ops_free: Some(hda_ops_free),
};

static glk_desc: sof_dev_desc = sof_dev_desc {
    machines: unsafe { &snd_soc_acpi_intel_glk_machines },
    use_acpi_target_states: true,
    resindex_lpe_base: 0,
    resindex_pcicfg_base: -1,
    resindex_imr_base: -1,
    irqindex_host_ipc: -1,
    chip_info: unsafe { &apl_chip_info },
    ipc_supported_mask: BIT(SOF_IPC_TYPE_3) | BIT(SOF_IPC_TYPE_4),
    ipc_default: SOF_IPC_TYPE_3,
    dspless_mode_supported: true, /* Only supported for HDaudio */
    default_fw_path: {
        let mut default_fw_path = [core::ptr::null(); SOF_IPC_TYPE_COUNT];
        default_fw_path[SOF_IPC_TYPE_3] = b"intel/sof\0".as_ptr() as *const c_char;
        default_fw_path[SOF_IPC_TYPE_4] = b"intel/avs/glk\0".as_ptr() as *const c_char;
        default_fw_path
    },
    default_lib_path: {
        let mut default_lib_path = [core::ptr::null(); SOF_IPC_TYPE_COUNT];
        default_lib_path[SOF_IPC_TYPE_4] = b"intel/avs-lib/glk\0".as_ptr() as *const c_char;
        default_lib_path
    },
    default_tplg_path: {
        let mut default_tplg_path = [core::ptr::null(); SOF_IPC_TYPE_COUNT];
        default_tplg_path[SOF_IPC_TYPE_3] = b"intel/sof-tplg\0".as_ptr() as *const c_char;
        default_tplg_path[SOF_IPC_TYPE_4] = b"intel/avs-tplg\0".as_ptr() as *const c_char;
        default_tplg_path
    },
    default_fw_filename: {
        let mut default_fw_filename = [core::ptr::null(); SOF_IPC_TYPE_COUNT];
        default_fw_filename[SOF_IPC_TYPE_3] = b"sof-glk.ri\0".as_ptr() as *const c_char;
        default_fw_filename[SOF_IPC_TYPE_4] = b"dsp_basefw.bin\0".as_ptr() as *const c_char;
        default_fw_filename
    },
    nocodec_tplg_filename: b"sof-glk-nocodec.tplg\0".as_ptr() as *const c_char,
    ops: unsafe { &sof_apl_ops },
    ops_init: Some(sof_apl_ops_init),
    ops_free: Some(hda_ops_free),
};

/* PCI IDs */
static sof_pci_ids: [pci_device_id; 3] = [
    PCI_DEVICE_DATA_INTEL(PCI_DEVICE_ID_INTEL_HDA_APL, &bxt_desc),
    PCI_DEVICE_DATA_INTEL(PCI_DEVICE_ID_INTEL_HDA_GLK, &glk_desc),
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
static mut snd_sof_pci_intel_apl_driver: pci_driver = pci_driver {
    name: b"sof-audio-pci-intel-apl\0".as_ptr() as *const c_char,
    id_table: sof_pci_ids.as_ptr(),
    probe: Some(hda_pci_intel_probe),
    remove: Some(sof_pci_remove),
    shutdown: Some(sof_pci_shutdown),
    driver: device_driver {
        pm: unsafe { pm_ptr(&sof_pci_pm) },
    },
};
// module_pci_driver(snd_sof_pci_intel_apl_driver);

// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("SOF support for ApolloLake platforms");
// MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_HDA_GENERIC");
// MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_HDA_COMMON");
// MODULE_IMPORT_NS("SND_SOC_SOF_PCI_DEV");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
