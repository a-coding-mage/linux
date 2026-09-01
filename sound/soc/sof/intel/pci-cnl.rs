// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//

/* platform specific devices */
/* hda.h */

use core::ptr;

extern "C" {
    static snd_soc_acpi_intel_cnl_machines: [SndSocAcpiMach; 0];
    static snd_soc_acpi_intel_cnl_sdw_machines: [SndSocAcpiMach; 0];
    static snd_soc_acpi_intel_cfl_machines: [SndSocAcpiMach; 0];
    static snd_soc_acpi_intel_cfl_sdw_machines: [SndSocAcpiMach; 0];
    static snd_soc_acpi_intel_cml_machines: [SndSocAcpiMach; 0];
    static snd_soc_acpi_intel_cml_sdw_machines: [SndSocAcpiMach; 0];
    static cnl_chip_info: SofIntelDspDesc;
    static sof_cnl_ops: SndSofDspOps;
    static sof_pci_pm: DevPmOps;

    fn sof_cnl_ops_init(sdev: *mut SndSofDev) -> i32;
    fn hda_ops_free(sdev: *mut SndSofDev);
    fn hda_pci_intel_probe(pci: *mut PciDev, pci_id: *const PciDeviceId) -> i32;
    fn sof_pci_remove(pci: *mut PciDev);
    fn sof_pci_shutdown(pci: *mut PciDev);
}

const SOF_IPC_TYPE_3: usize = 3;
const SOF_IPC_TYPE_4: usize = 4;

const PCI_VENDOR_ID_INTEL: u32 = 0x8086;
const PCI_DEVICE_ID_INTEL_HDA_CNL_LP: u32 = HDA_CNL_LP;
const PCI_DEVICE_ID_INTEL_HDA_CNL_H: u32 = HDA_CNL_H;
const PCI_DEVICE_ID_INTEL_HDA_CML_LP: u32 = HDA_CML_LP;
const PCI_DEVICE_ID_INTEL_HDA_CML_H: u32 = HDA_CML_H;
const PCI_DEVICE_ID_INTEL_HDA_CML_S: u32 = HDA_CML_S;

const HDA_CNL_LP: u32 = 0x9dc8;
const HDA_CNL_H: u32 = 0xa348;
const HDA_CML_LP: u32 = 0x02c8;
const HDA_CML_H: u32 = 0x06c8;
const HDA_CML_S: u32 = 0xa3f0;

const PCI_ANY_ID: u32 = !0;
const ARRAY_SIZE_SOF_IPC_TYPE: usize = SOF_IPC_TYPE_4 + 1;

const fn bit(nr: usize) -> u32 {
    1u32 << nr
}

#[repr(C)]
pub struct SndSocAcpiMach {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SofIntelDspDesc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SndSofDspOps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SndSofDev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PciDev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct DevPmOps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct DeviceDriver {
    pub pm: *const DevPmOps,
}

#[repr(C)]
pub struct SofDevDesc {
    pub machines: *const SndSocAcpiMach,
    pub alt_machines: *const SndSocAcpiMach,
    pub use_acpi_target_states: bool,
    pub resindex_lpe_base: i32,
    pub resindex_pcicfg_base: i32,
    pub resindex_imr_base: i32,
    pub irqindex_host_ipc: i32,
    pub chip_info: *const SofIntelDspDesc,
    pub ipc_supported_mask: u32,
    pub ipc_default: usize,
    pub dspless_mode_supported: bool,
    pub default_fw_path: [*const u8; ARRAY_SIZE_SOF_IPC_TYPE],
    pub default_lib_path: [*const u8; ARRAY_SIZE_SOF_IPC_TYPE],
    pub default_tplg_path: [*const u8; ARRAY_SIZE_SOF_IPC_TYPE],
    pub default_fw_filename: [*const u8; ARRAY_SIZE_SOF_IPC_TYPE],
    pub nocodec_tplg_filename: *const u8,
    pub ops: *const SndSofDspOps,
    pub ops_init: Option<unsafe extern "C" fn(*mut SndSofDev) -> i32>,
    pub ops_free: Option<unsafe extern "C" fn(*mut SndSofDev)>,
}

#[repr(C)]
pub struct PciDeviceId {
    pub vendor: u32,
    pub device: u32,
    pub subvendor: u32,
    pub subdevice: u32,
    pub class: u32,
    pub class_mask: u32,
    pub driver_data: usize,
}

#[repr(C)]
pub struct PciDriver {
    pub name: *const u8,
    pub id_table: *const PciDeviceId,
    pub probe: Option<unsafe extern "C" fn(*mut PciDev, *const PciDeviceId) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut PciDev)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut PciDev)>,
    pub driver: DeviceDriver,
}

const fn pci_device_data(device: u32, data: *const SofDevDesc) -> PciDeviceId {
    PciDeviceId {
        vendor: PCI_VENDOR_ID_INTEL,
        device,
        subvendor: PCI_ANY_ID,
        subdevice: PCI_ANY_ID,
        class: 0,
        class_mask: 0,
        driver_data: data as usize,
    }
}

static CNL_DESC: SofDevDesc = SofDevDesc {
    machines: unsafe { snd_soc_acpi_intel_cnl_machines.as_ptr() },
    alt_machines: unsafe { snd_soc_acpi_intel_cnl_sdw_machines.as_ptr() },
    use_acpi_target_states: true,
    resindex_lpe_base: 0,
    resindex_pcicfg_base: -1,
    resindex_imr_base: -1,
    irqindex_host_ipc: -1,
    chip_info: unsafe { &cnl_chip_info },
    ipc_supported_mask: bit(SOF_IPC_TYPE_3) | bit(SOF_IPC_TYPE_4),
    ipc_default: SOF_IPC_TYPE_3,
    dspless_mode_supported: true, /* Only supported for HDaudio */
    default_fw_path: [
        ptr::null(),
        ptr::null(),
        ptr::null(),
        b"intel/sof\0".as_ptr(),
        b"intel/avs/cnl\0".as_ptr(),
    ],
    default_lib_path: [
        ptr::null(),
        ptr::null(),
        ptr::null(),
        ptr::null(),
        b"intel/avs-lib/cnl\0".as_ptr(),
    ],
    default_tplg_path: [
        ptr::null(),
        ptr::null(),
        ptr::null(),
        b"intel/sof-tplg\0".as_ptr(),
        b"intel/avs-tplg\0".as_ptr(),
    ],
    default_fw_filename: [
        ptr::null(),
        ptr::null(),
        ptr::null(),
        b"sof-cnl.ri\0".as_ptr(),
        b"dsp_basefw.bin\0".as_ptr(),
    ],
    nocodec_tplg_filename: b"sof-cnl-nocodec.tplg\0".as_ptr(),
    ops: unsafe { &sof_cnl_ops },
    ops_init: Some(sof_cnl_ops_init),
    ops_free: Some(hda_ops_free),
};

static CFL_DESC: SofDevDesc = SofDevDesc {
    machines: unsafe { snd_soc_acpi_intel_cfl_machines.as_ptr() },
    alt_machines: unsafe { snd_soc_acpi_intel_cfl_sdw_machines.as_ptr() },
    use_acpi_target_states: true,
    resindex_lpe_base: 0,
    resindex_pcicfg_base: -1,
    resindex_imr_base: -1,
    irqindex_host_ipc: -1,
    chip_info: unsafe { &cnl_chip_info },
    ipc_supported_mask: bit(SOF_IPC_TYPE_3) | bit(SOF_IPC_TYPE_4),
    ipc_default: SOF_IPC_TYPE_3,
    dspless_mode_supported: true, /* Only supported for HDaudio */
    default_fw_path: [
        ptr::null(),
        ptr::null(),
        ptr::null(),
        b"intel/sof\0".as_ptr(),
        b"intel/avs/cnl\0".as_ptr(),
    ],
    default_lib_path: [
        ptr::null(),
        ptr::null(),
        ptr::null(),
        ptr::null(),
        b"intel/avs-lib/cnl\0".as_ptr(),
    ],
    default_tplg_path: [
        ptr::null(),
        ptr::null(),
        ptr::null(),
        b"intel/sof-tplg\0".as_ptr(),
        b"intel/avs-tplg\0".as_ptr(),
    ],
    default_fw_filename: [
        ptr::null(),
        ptr::null(),
        ptr::null(),
        b"sof-cfl.ri\0".as_ptr(),
        b"dsp_basefw.bin\0".as_ptr(),
    ],
    nocodec_tplg_filename: b"sof-cnl-nocodec.tplg\0".as_ptr(),
    ops: unsafe { &sof_cnl_ops },
    ops_init: Some(sof_cnl_ops_init),
    ops_free: Some(hda_ops_free),
};

static CML_DESC: SofDevDesc = SofDevDesc {
    machines: unsafe { snd_soc_acpi_intel_cml_machines.as_ptr() },
    alt_machines: unsafe { snd_soc_acpi_intel_cml_sdw_machines.as_ptr() },
    use_acpi_target_states: true,
    resindex_lpe_base: 0,
    resindex_pcicfg_base: -1,
    resindex_imr_base: -1,
    irqindex_host_ipc: -1,
    chip_info: unsafe { &cnl_chip_info },
    ipc_supported_mask: bit(SOF_IPC_TYPE_3) | bit(SOF_IPC_TYPE_4),
    ipc_default: SOF_IPC_TYPE_3,
    dspless_mode_supported: true, /* Only supported for HDaudio */
    default_fw_path: [
        ptr::null(),
        ptr::null(),
        ptr::null(),
        b"intel/sof\0".as_ptr(),
        b"intel/avs/cnl\0".as_ptr(),
    ],
    default_lib_path: [
        ptr::null(),
        ptr::null(),
        ptr::null(),
        ptr::null(),
        b"intel/avs-lib/cnl\0".as_ptr(),
    ],
    default_tplg_path: [
        ptr::null(),
        ptr::null(),
        ptr::null(),
        b"intel/sof-tplg\0".as_ptr(),
        b"intel/avs-tplg\0".as_ptr(),
    ],
    default_fw_filename: [
        ptr::null(),
        ptr::null(),
        ptr::null(),
        b"sof-cml.ri\0".as_ptr(),
        b"dsp_basefw.bin\0".as_ptr(),
    ],
    nocodec_tplg_filename: b"sof-cnl-nocodec.tplg\0".as_ptr(),
    ops: unsafe { &sof_cnl_ops },
    ops_init: Some(sof_cnl_ops_init),
    ops_free: Some(hda_ops_free),
};

/* PCI IDs */
static SOF_PCI_IDS: [PciDeviceId; 6] = [
    pci_device_data(PCI_DEVICE_ID_INTEL_HDA_CNL_LP, &CNL_DESC),
    pci_device_data(PCI_DEVICE_ID_INTEL_HDA_CNL_H, &CFL_DESC),
    pci_device_data(PCI_DEVICE_ID_INTEL_HDA_CML_LP, &CML_DESC),
    pci_device_data(PCI_DEVICE_ID_INTEL_HDA_CML_H, &CML_DESC),
    pci_device_data(PCI_DEVICE_ID_INTEL_HDA_CML_S, &CML_DESC),
    PciDeviceId {
        vendor: 0,
        device: 0,
        subvendor: 0,
        subdevice: 0,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    },
];
/* MODULE_DEVICE_TABLE(pci, sof_pci_ids); */

/* pci_driver definition */
static mut SND_SOF_PCI_INTEL_CNL_DRIVER: PciDriver = PciDriver {
    name: b"sof-audio-pci-intel-cnl\0".as_ptr(),
    id_table: SOF_PCI_IDS.as_ptr(),
    probe: Some(hda_pci_intel_probe),
    remove: Some(sof_pci_remove),
    shutdown: Some(sof_pci_shutdown),
    driver: DeviceDriver {
        pm: unsafe { &sof_pci_pm },
    },
};
/* module_pci_driver(snd_sof_pci_intel_cnl_driver); */

/* MODULE_LICENSE("Dual BSD/GPL"); */
/* MODULE_DESCRIPTION("SOF support for CannonLake platforms"); */
/* MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_HDA_GENERIC"); */
/* MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_HDA_COMMON"); */
/* MODULE_IMPORT_NS("SND_SOC_SOF_PCI_DEV"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
