// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2023 Advanced Micro Devices, Inc. All rights reserved.
//
// Authors: Vijendar Mukunda <Vijendar.Mukunda@amd.com>

/*.
 * PCI interface for ACP6.3 device
 */

// Rust translation of Linux kernel includes:
// <linux/module.h>, <linux/pci.h>, <linux/platform_device.h>,
// <sound/sof.h>, <sound/soc-acpi.h>, "../ops.h", "../sof-pci-dev.h",
// "../../amd/mach-config.h", "acp.h", and "acp-dsp-offset.h".

use core::ffi::{c_char, c_int, c_ulong, c_void};

const ACP6X_FUTURE_REG_ACLK_0: u32 = 0x1854;
const ACP6x_REG_START: u32 = 0x1240000;
const ACP6x_REG_END: u32 = 0x125C000;

extern "C" {
    static snd_soc_acpi_amd_acp63_sof_machines: c_void;
    static snd_soc_acpi_amd_acp63_sof_sdw_machines: c_void;
    static sof_acp63_ops: c_void;
    static sof_pci_pm: c_void;

    fn sof_acp63_ops_init(sdev: *mut c_void) -> c_int;
    fn snd_amd_acp_find_config(pci: *mut pci_dev) -> c_uint;
    fn sof_pci_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int;
    fn sof_pci_remove(pci: *mut pci_dev);
}

type c_uint = u32;

extern "C" {
    static ACP6X_PGFSM_BASE: u32;
    static ACP6X_EXTERNAL_INTR_ENB: u32;
    static ACP6X_EXTERNAL_INTR_CNTL: u32;
    static ACP6X_EXT_INTR_STAT: u32;
    static ACP6X_EXT_INTR_STAT1: u32;
    static ACP6X_ERROR_STATUS: u32;
    static ACP6X_SW0_I2S_ERROR_REASON: u32;
    static ACP6X_DSP_SW_INTR_BASE: u32;
    static ACP6X_SRAM_PTE_OFFSET: u32;
    static ACP6X_AXI2DAGB_SEM_0: u32;
    static ACP6X_DSP_FUSION_RUNSTALL: u32;
    static ACP6X_SDW_MAX_MANAGER_COUNT: u32;
    static SDW_ACPI_ADDR_ACP63: u32;
    static SOF_IPC_TYPE_3: usize;
    static ACP63_PCI_ID: u8;
    static FLAG_AMD_SOF: c_uint;
    static FLAG_AMD_SOF_ONLY_DMIC: c_uint;
    static PCI_VENDOR_ID_AMD: u32;
    static ACP_PCI_DEV_ID: u32;
    static ENODEV: c_int;
}

const SOF_IPC_TYPES: usize = 4;

const fn bit(n: usize) -> u32 {
    1u32 << n
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
pub struct pci_driver_inner {
    pub pm: *const c_void,
}

#[repr(C)]
pub struct pci_driver {
    pub name: *const c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut pci_dev)>,
    pub driver: pci_driver_inner,
}

#[repr(C)]
pub struct sof_amd_acp_desc {
    pub pgfsm_base: u32,
    pub ext_intr_enb: u32,
    pub ext_intr_cntl: u32,
    pub ext_intr_stat: u32,
    pub ext_intr_stat1: u32,
    pub acp_error_stat: u32,
    pub acp_sw0_i2s_err_reason: u32,
    pub dsp_intr_base: u32,
    pub sram_pte_offset: u32,
    pub hw_semaphore_offset: u32,
    pub fusion_dsp_offset: u32,
    pub probe_reg_offset: u32,
    pub sdw_max_link_count: u32,
    pub sdw_acpi_dev_addr: u32,
    pub reg_start_addr: u32,
    pub reg_end_addr: u32,
}

#[repr(C)]
pub struct sof_dev_desc {
    pub machines: *const c_void,
    pub alt_machines: *const c_void,
    pub resindex_lpe_base: c_int,
    pub resindex_pcicfg_base: c_int,
    pub resindex_imr_base: c_int,
    pub irqindex_host_ipc: c_int,
    pub chip_info: *const sof_amd_acp_desc,
    pub ipc_supported_mask: u32,
    pub ipc_default: usize,
    pub default_fw_path: [*const c_char; SOF_IPC_TYPES],
    pub default_tplg_path: [*const c_char; SOF_IPC_TYPES],
    pub default_fw_filename: [*const c_char; SOF_IPC_TYPES],
    pub nocodec_tplg_filename: *const c_char,
    pub ops: *const c_void,
    pub ops_init: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
}

static acp63_chip_info: sof_amd_acp_desc = unsafe {
    sof_amd_acp_desc {
        pgfsm_base: ACP6X_PGFSM_BASE,
        ext_intr_enb: ACP6X_EXTERNAL_INTR_ENB,
        ext_intr_cntl: ACP6X_EXTERNAL_INTR_CNTL,
        ext_intr_stat: ACP6X_EXT_INTR_STAT,
        ext_intr_stat1: ACP6X_EXT_INTR_STAT1,
        acp_error_stat: ACP6X_ERROR_STATUS,
        acp_sw0_i2s_err_reason: ACP6X_SW0_I2S_ERROR_REASON,
        dsp_intr_base: ACP6X_DSP_SW_INTR_BASE,
        sram_pte_offset: ACP6X_SRAM_PTE_OFFSET,
        hw_semaphore_offset: ACP6X_AXI2DAGB_SEM_0,
        fusion_dsp_offset: ACP6X_DSP_FUSION_RUNSTALL,
        probe_reg_offset: ACP6X_FUTURE_REG_ACLK_0,
        sdw_max_link_count: ACP6X_SDW_MAX_MANAGER_COUNT,
        sdw_acpi_dev_addr: SDW_ACPI_ADDR_ACP63,
        reg_start_addr: ACP6x_REG_START,
        reg_end_addr: ACP6x_REG_END,
    }
};

static acp63_desc: sof_dev_desc = unsafe {
    let mut default_fw_path = [core::ptr::null(); SOF_IPC_TYPES];
    let mut default_tplg_path = [core::ptr::null(); SOF_IPC_TYPES];
    let mut default_fw_filename = [core::ptr::null(); SOF_IPC_TYPES];

    default_fw_path[SOF_IPC_TYPE_3] = b"amd/sof\0".as_ptr() as *const c_char;
    default_tplg_path[SOF_IPC_TYPE_3] = b"amd/sof-tplg\0".as_ptr() as *const c_char;
    default_fw_filename[SOF_IPC_TYPE_3] = b"sof-acp_6_3.ri\0".as_ptr() as *const c_char;

    sof_dev_desc {
        machines: &snd_soc_acpi_amd_acp63_sof_machines as *const _ as *const c_void,
        alt_machines: &snd_soc_acpi_amd_acp63_sof_sdw_machines as *const _ as *const c_void,
        resindex_lpe_base: 0,
        resindex_pcicfg_base: -1,
        resindex_imr_base: -1,
        irqindex_host_ipc: -1,
        chip_info: &acp63_chip_info,
        ipc_supported_mask: bit(SOF_IPC_TYPE_3),
        ipc_default: SOF_IPC_TYPE_3,
        default_fw_path,
        default_tplg_path,
        default_fw_filename,
        nocodec_tplg_filename: b"sof-acp.tplg\0".as_ptr() as *const c_char,
        ops: &sof_acp63_ops as *const _ as *const c_void,
        ops_init: Some(sof_acp63_ops_init),
    }
};

unsafe extern "C" fn acp63_pci_probe(
    pci: *mut pci_dev,
    pci_id: *const pci_device_id,
) -> c_int {
    let flag: c_uint;

    if (*pci).revision != ACP63_PCI_ID {
        return -ENODEV;
    }

    flag = snd_amd_acp_find_config(pci);
    if flag != FLAG_AMD_SOF && flag != FLAG_AMD_SOF_ONLY_DMIC {
        return -ENODEV;
    }

    sof_pci_probe(pci, pci_id)
}

unsafe extern "C" fn acp63_pci_remove(pci: *mut pci_dev) {
    sof_pci_remove(pci);
}

/* PCI IDs */
static acp63_pci_ids: [pci_device_id; 2] = unsafe {
    [
        pci_device_id {
            vendor: PCI_VENDOR_ID_AMD,
            device: ACP_PCI_DEV_ID,
            subvendor: u32::MAX,
            subdevice: u32::MAX,
            class: 0,
            class_mask: 0,
            driver_data: &acp63_desc as *const _ as c_ulong,
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
// MODULE_DEVICE_TABLE(pci, acp63_pci_ids);

/* pci_driver definition */
static mut snd_sof_pci_amd_acp63_driver: pci_driver = pci_driver {
    name: KBUILD_MODNAME.as_ptr() as *const c_char,
    id_table: acp63_pci_ids.as_ptr(),
    probe: Some(acp63_pci_probe),
    remove: Some(acp63_pci_remove),
    driver: pci_driver_inner {
        pm: unsafe { &sof_pci_pm as *const _ as *const c_void },
    },
};
static KBUILD_MODNAME: &[u8] = b"snd_sof_pci_amd_acp63_driver\0";
// module_pci_driver(snd_sof_pci_amd_acp63_driver);

// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("ACP63 SOF Driver");
// MODULE_IMPORT_NS("SND_SOC_SOF_AMD_COMMON");
// MODULE_IMPORT_NS("SND_SOC_SOF_PCI_DEV");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
