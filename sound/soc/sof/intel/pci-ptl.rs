// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2024 Intel Corporation.
//

// C dependency intent:
// #include <linux/module.h>
// #include <linux/pci.h>
// #include <sound/soc-acpi.h>
// #include <sound/soc-acpi-intel-match.h>
// #include <sound/sof.h>
// #include "../ops.h"
// #include "../sof-pci-dev.h"
// platform specific devices:
// #include "hda.h"
// #include "ptl.h"

extern "C" {
    static snd_soc_acpi_intel_ptl_machines: *const core::ffi::c_void;
    static snd_soc_acpi_intel_ptl_sdw_machines: *const core::ffi::c_void;
    static ptl_chip_info: core::ffi::c_void;
    static wcl_chip_info: core::ffi::c_void;
    static sof_pci_pm: core::ffi::c_void;

    fn sof_ptl_set_ops(
        sdev: *mut snd_sof_dev,
        ops: *mut snd_sof_dsp_ops,
    ) -> core::ffi::c_int;
    fn hda_pci_intel_probe(
        pci: *mut pci_dev,
        id: *const pci_device_id,
    ) -> core::ffi::c_int;
    fn sof_pci_remove(pci: *mut pci_dev);
    fn sof_pci_shutdown(pci: *mut pci_dev);
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
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sof_dev_desc {
    pub use_acpi_target_states: bool,
    pub machines: *const core::ffi::c_void,
    pub alt_machines: *const core::ffi::c_void,
    pub resindex_lpe_base: core::ffi::c_int,
    pub resindex_pcicfg_base: core::ffi::c_int,
    pub resindex_imr_base: core::ffi::c_int,
    pub irqindex_host_ipc: core::ffi::c_int,
    pub chip_info: *const core::ffi::c_void,
    pub ipc_supported_mask: core::ffi::c_uint,
    pub ipc_default: core::ffi::c_uint,
    pub dspless_mode_supported: bool,
    pub on_demand_dsp_boot: bool,
    pub default_fw_path: [*const core::ffi::c_char; SOF_IPC_TYPE_COUNT],
    pub default_lib_path: [*const core::ffi::c_char; SOF_IPC_TYPE_COUNT],
    pub default_tplg_path: [*const core::ffi::c_char; SOF_IPC_TYPE_COUNT],
    pub default_fw_filename: [*const core::ffi::c_char; SOF_IPC_TYPE_COUNT],
    pub nocodec_tplg_filename: *const core::ffi::c_char,
    pub ops: *mut snd_sof_dsp_ops,
    pub ops_init: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> core::ffi::c_int>,
}

#[repr(C)]
pub struct pci_device_id {
    pub vendor: core::ffi::c_uint,
    pub device: core::ffi::c_uint,
    pub subvendor: core::ffi::c_uint,
    pub subdevice: core::ffi::c_uint,
    pub class: core::ffi::c_uint,
    pub class_mask: core::ffi::c_uint,
    pub driver_data: usize,
}

#[repr(C)]
pub struct device_driver {
    pub pm: *const core::ffi::c_void,
}

#[repr(C)]
pub struct pci_driver {
    pub name: *const core::ffi::c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<
        unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> core::ffi::c_int,
    >,
    pub remove: Option<unsafe extern "C" fn(*mut pci_dev)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut pci_dev)>,
    pub driver: device_driver,
}

const SOF_IPC_TYPE_4: usize = 4;
const SOF_IPC_TYPE_COUNT: usize = SOF_IPC_TYPE_4 + 1;

const PCI_VENDOR_ID_INTEL: core::ffi::c_uint = 0x8086;

extern "C" {
    static HDA_PTL: core::ffi::c_uint;
    static HDA_PTL_H: core::ffi::c_uint;
    static HDA_WCL: core::ffi::c_uint;
}

const fn bit(nr: usize) -> core::ffi::c_uint {
    1u32 << nr
}

const fn null_path_array() -> [*const core::ffi::c_char; SOF_IPC_TYPE_COUNT] {
    [core::ptr::null(); SOF_IPC_TYPE_COUNT]
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const core::ffi::c_char
    };
}

// PantherLake ops
static mut sof_ptl_ops: snd_sof_dsp_ops = snd_sof_dsp_ops { _private: [] };

unsafe extern "C" fn sof_ptl_ops_init(sdev: *mut snd_sof_dev) -> core::ffi::c_int {
    unsafe { sof_ptl_set_ops(sdev, core::ptr::addr_of_mut!(sof_ptl_ops)) }
}

static ptl_desc: sof_dev_desc = {
    let mut default_fw_path = null_path_array();
    let mut default_lib_path = null_path_array();
    let mut default_tplg_path = null_path_array();
    let mut default_fw_filename = null_path_array();

    default_fw_path[SOF_IPC_TYPE_4] = cstr!("intel/sof-ipc4/ptl");
    default_lib_path[SOF_IPC_TYPE_4] = cstr!("intel/sof-ipc4-lib/ptl");
    default_tplg_path[SOF_IPC_TYPE_4] = cstr!("intel/sof-ipc4-tplg");
    default_fw_filename[SOF_IPC_TYPE_4] = cstr!("sof-ptl.ri");

    sof_dev_desc {
        use_acpi_target_states: true,
        machines: unsafe { snd_soc_acpi_intel_ptl_machines },
        alt_machines: unsafe { snd_soc_acpi_intel_ptl_sdw_machines },
        resindex_lpe_base: 0,
        resindex_pcicfg_base: -1,
        resindex_imr_base: -1,
        irqindex_host_ipc: -1,
        chip_info: unsafe { core::ptr::addr_of!(ptl_chip_info) },
        ipc_supported_mask: bit(SOF_IPC_TYPE_4),
        ipc_default: SOF_IPC_TYPE_4 as core::ffi::c_uint,
        dspless_mode_supported: true,
        on_demand_dsp_boot: true,
        default_fw_path,
        default_lib_path,
        default_tplg_path,
        default_fw_filename,
        nocodec_tplg_filename: cstr!("sof-ptl-nocodec.tplg"),
        ops: core::ptr::addr_of_mut!(sof_ptl_ops),
        ops_init: Some(sof_ptl_ops_init),
    }
};

static wcl_desc: sof_dev_desc = {
    let mut default_fw_path = null_path_array();
    let mut default_lib_path = null_path_array();
    let mut default_tplg_path = null_path_array();
    let mut default_fw_filename = null_path_array();

    default_fw_path[SOF_IPC_TYPE_4] = cstr!("intel/sof-ipc4/wcl");
    default_lib_path[SOF_IPC_TYPE_4] = cstr!("intel/sof-ipc4-lib/wcl");
    default_tplg_path[SOF_IPC_TYPE_4] = cstr!("intel/sof-ipc4-tplg");
    default_fw_filename[SOF_IPC_TYPE_4] = cstr!("sof-wcl.ri");

    sof_dev_desc {
        use_acpi_target_states: true,
        machines: unsafe { snd_soc_acpi_intel_ptl_machines },
        alt_machines: unsafe { snd_soc_acpi_intel_ptl_sdw_machines },
        resindex_lpe_base: 0,
        resindex_pcicfg_base: -1,
        resindex_imr_base: -1,
        irqindex_host_ipc: -1,
        chip_info: unsafe { core::ptr::addr_of!(wcl_chip_info) },
        ipc_supported_mask: bit(SOF_IPC_TYPE_4),
        ipc_default: SOF_IPC_TYPE_4 as core::ffi::c_uint,
        dspless_mode_supported: true,
        on_demand_dsp_boot: true,
        default_fw_path,
        default_lib_path,
        default_tplg_path,
        default_fw_filename,
        nocodec_tplg_filename: cstr!("sof-ptl-nocodec.tplg"),
        ops: core::ptr::addr_of_mut!(sof_ptl_ops),
        ops_init: Some(sof_ptl_ops_init),
    }
};

const fn pci_device_data(
    vendor: core::ffi::c_uint,
    device: core::ffi::c_uint,
    data: *const sof_dev_desc,
) -> pci_device_id {
    pci_device_id {
        vendor,
        device,
        subvendor: 0,
        subdevice: 0,
        class: 0,
        class_mask: 0,
        driver_data: data as usize,
    }
}

// PCI IDs
static sof_pci_ids: [pci_device_id; 4] = [
    unsafe { pci_device_data(PCI_VENDOR_ID_INTEL, HDA_PTL, core::ptr::addr_of!(ptl_desc)) }, // PTL
    unsafe { pci_device_data(PCI_VENDOR_ID_INTEL, HDA_PTL_H, core::ptr::addr_of!(ptl_desc)) }, // PTL-H
    unsafe { pci_device_data(PCI_VENDOR_ID_INTEL, HDA_WCL, core::ptr::addr_of!(wcl_desc)) }, // WCL
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
static mut snd_sof_pci_intel_ptl_driver: pci_driver = pci_driver {
    name: cstr!("sof-audio-pci-intel-ptl"),
    id_table: sof_pci_ids.as_ptr(),
    probe: Some(hda_pci_intel_probe),
    remove: Some(sof_pci_remove),
    shutdown: Some(sof_pci_shutdown),
    driver: device_driver {
        pm: unsafe { core::ptr::addr_of!(sof_pci_pm) },
    },
};
// module_pci_driver(snd_sof_pci_intel_ptl_driver);

// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("SOF support for PantherLake platforms");
// MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_HDA_GENERIC");
// MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_HDA_COMMON");
// MODULE_IMPORT_NS("SND_SOC_SOF_PCI_DEV");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
