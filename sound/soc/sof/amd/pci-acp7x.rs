// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2025 Advanced Micro Devices, Inc. All rights reserved.
//
// Authors: Vijendar Mukunda <Vijendar.Mukunda@amd.com>

/*
 * PCI interface for ACP7.B/7.F devices
 */

// C dependencies removed from executable Rust:
// <linux/module.h>, <linux/pci.h>, <sound/sof.h>, <sound/soc-acpi.h>,
// "../ops.h", "../sof-pci-dev.h", "../../amd/mach-config.h",
// "acp.h", "acp-dsp-offset.h".

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const ACP7X_FUTURE_REG_ACLK_0: u32 = 0x18e0;
const ACP7X_REG_START: u32 = 0x1240000;
const ACP7X_REG_END: u32 = 0x125C000;

const fn BIT(nr: u32) -> u64 {
    1u64 << nr
}

#[repr(C)]
pub struct sof_amd_acp_desc {
    pub name: *const c_char,
    pub pgfsm_base: u32,
    pub ext_intr_enb: u32,
    pub ext_intr_cntl: u32,
    pub ext_intr_stat: u32,
    pub ext_intr_stat1: u32,
    pub dsp_intr_base: u32,
    pub acp_error_stat: u32,
    pub sram_pte_offset: u32,
    pub hw_semaphore_offset: u32,
    pub fusion_dsp_offset: u32,
    pub probe_reg_offset: u32,
    pub reg_start_addr: u32,
    pub reg_end_addr: u32,
}

#[repr(C)]
pub struct sof_dev_desc {
    pub machines: *const c_void,
    pub resindex_lpe_base: c_int,
    pub resindex_pcicfg_base: c_int,
    pub resindex_imr_base: c_int,
    pub irqindex_host_ipc: c_int,
    pub chip_info: *const sof_amd_acp_desc,
    pub ipc_supported_mask: u64,
    pub ipc_default: u32,
    pub default_fw_path: [*const c_char; SOF_IPC_TYPE_COUNT],
    pub default_tplg_path: [*const c_char; SOF_IPC_TYPE_COUNT],
    pub default_fw_filename: [*const c_char; SOF_IPC_TYPE_COUNT],
    pub nocodec_tplg_filename: *const c_char,
    pub ops: *const c_void,
    pub ops_init: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
}

#[repr(C)]
pub struct pci_dev {
    pub revision: u8,
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
    pub driver: device_driver,
}

#[repr(C)]
pub struct device_driver {
    pub pm: *const c_void,
}

#[repr(C)]
pub struct snd_sof_dev {
    _private: [u8; 0],
}

extern "C" {
    static snd_soc_acpi_amd_acp7x_sof_machines: c_void;
    static sof_acp7x_ops: c_void;
    static sof_pci_pm: c_void;

    fn sof_acp7x_ops_init(sdev: *mut snd_sof_dev) -> c_int;
    fn snd_amd_acp_find_config(pci: *mut pci_dev) -> c_uint;
    fn sof_pci_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int;
    fn sof_pci_remove(pci: *mut pci_dev);
}

extern "C" {
    static KBUILD_MODNAME: c_char;
}

extern "C" {
    static ACP7X_PGFSM_BASE: u32;
    static ACP6X_EXTERNAL_INTR_ENB: u32;
    static ACP7X_EXTERNAL_INTR_CNTL: u32;
    static ACP7X_EXT_INTR_STAT: u32;
    static ACP7X_EXT_INTR_STAT1: u32;
    static ACP7X_DSP_SW_INTR_BASE: u32;
    static ACP7X_ERROR_STATUS: u32;
    static ACP7X_SRAM_PTE_OFFSET: u32;
    static ACP7X_AXI2DAGB_SEM_0: u32;
    static ACP7X_DSP_FUSION_RUNSTALL: u32;

    static SOF_IPC_TYPE_3: u32;
    static ACP7B_PCI_ID: u8;
    static ACP7F_PCI_ID: u8;
    static FLAG_AMD_SOF: c_uint;
    static FLAG_AMD_SOF_ONLY_DMIC: c_uint;
    static ENODEV: c_int;
    static PCI_VENDOR_ID_AMD: u32;
    static ACP_PCI_DEV_ID: u32;
}

const SOF_IPC_TYPE_COUNT: usize = 4;

static ACP7X_NAME: &[u8] = b"acp7x\0";
static AMD_SOF_PATH: &[u8] = b"amd/sof\0";
static AMD_SOF_TPLG_PATH: &[u8] = b"amd/sof-tplg\0";
static SOF_ACP7X_FW: &[u8] = b"sof-acp7x.ri\0";
static SOF_ACP_TPLG: &[u8] = b"sof-acp.tplg\0";

static acp7x_chip_info: sof_amd_acp_desc = unsafe {
    sof_amd_acp_desc {
        name: ACP7X_NAME.as_ptr() as *const c_char,
        pgfsm_base: ACP7X_PGFSM_BASE,
        ext_intr_enb: ACP6X_EXTERNAL_INTR_ENB,
        ext_intr_cntl: ACP7X_EXTERNAL_INTR_CNTL,
        ext_intr_stat: ACP7X_EXT_INTR_STAT,
        ext_intr_stat1: ACP7X_EXT_INTR_STAT1,
        dsp_intr_base: ACP7X_DSP_SW_INTR_BASE,
        acp_error_stat: ACP7X_ERROR_STATUS,
        sram_pte_offset: ACP7X_SRAM_PTE_OFFSET,
        hw_semaphore_offset: ACP7X_AXI2DAGB_SEM_0,
        fusion_dsp_offset: ACP7X_DSP_FUSION_RUNSTALL,
        probe_reg_offset: ACP7X_FUTURE_REG_ACLK_0,
        reg_start_addr: ACP7X_REG_START,
        reg_end_addr: ACP7X_REG_END,
    }
};

static acp7x_desc: sof_dev_desc = unsafe {
    let mut default_fw_path = [core::ptr::null(); SOF_IPC_TYPE_COUNT];
    let mut default_tplg_path = [core::ptr::null(); SOF_IPC_TYPE_COUNT];
    let mut default_fw_filename = [core::ptr::null(); SOF_IPC_TYPE_COUNT];

    default_fw_path[SOF_IPC_TYPE_3 as usize] = AMD_SOF_PATH.as_ptr() as *const c_char;
    default_tplg_path[SOF_IPC_TYPE_3 as usize] = AMD_SOF_TPLG_PATH.as_ptr() as *const c_char;
    default_fw_filename[SOF_IPC_TYPE_3 as usize] = SOF_ACP7X_FW.as_ptr() as *const c_char;

    sof_dev_desc {
        machines: &snd_soc_acpi_amd_acp7x_sof_machines as *const c_void,
        resindex_lpe_base: 0,
        resindex_pcicfg_base: -1,
        resindex_imr_base: -1,
        irqindex_host_ipc: -1,
        chip_info: &acp7x_chip_info,
        ipc_supported_mask: BIT(SOF_IPC_TYPE_3),
        ipc_default: SOF_IPC_TYPE_3,
        default_fw_path,
        default_tplg_path,
        default_fw_filename,
        nocodec_tplg_filename: SOF_ACP_TPLG.as_ptr() as *const c_char,
        ops: &sof_acp7x_ops as *const c_void,
        ops_init: Some(sof_acp7x_ops_init),
    }
};

unsafe extern "C" fn acp7x_pci_probe(
    pci: *mut pci_dev,
    pci_id: *const pci_device_id,
) -> c_int {
    let flag: c_uint;

    match (*pci).revision {
        revision if revision == ACP7B_PCI_ID || revision == ACP7F_PCI_ID => {}
        _ => return -ENODEV,
    }

    flag = snd_amd_acp_find_config(pci);
    if flag != FLAG_AMD_SOF && flag != FLAG_AMD_SOF_ONLY_DMIC {
        return -ENODEV;
    }

    sof_pci_probe(pci, pci_id)
}

unsafe extern "C" fn acp7x_pci_remove(pci: *mut pci_dev) {
    sof_pci_remove(pci);
}

/* PCI IDs */
static acp7x_pci_ids: [pci_device_id; 2] = unsafe {
    [
        pci_device_id {
            vendor: PCI_VENDOR_ID_AMD,
            device: ACP_PCI_DEV_ID,
            subvendor: !0,
            subdevice: !0,
            class: 0,
            class_mask: 0,
            driver_data: &acp7x_desc as *const sof_dev_desc as c_ulong,
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
    ]
};
// MODULE_DEVICE_TABLE(pci, acp7x_pci_ids);

/* pci_driver definition */
static snd_sof_pci_amd_acp7x_driver: pci_driver = unsafe {
    pci_driver {
        name: &KBUILD_MODNAME as *const c_char,
        id_table: acp7x_pci_ids.as_ptr(),
        probe: Some(acp7x_pci_probe),
        remove: Some(acp7x_pci_remove),
        driver: device_driver {
            pm: &sof_pci_pm as *const c_void,
        },
    }
};
// module_pci_driver(snd_sof_pci_amd_acp7x_driver);

// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("ACP7X SOF Driver");
// MODULE_IMPORT_NS("SND_SOC_SOF_AMD_COMMON");
// MODULE_IMPORT_NS("SND_SOC_SOF_PCI_DEV");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
