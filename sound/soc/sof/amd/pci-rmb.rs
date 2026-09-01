// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2022 Advanced Micro Devices, Inc. All rights reserved.
//
// Authors: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>

/*.
 * PCI interface for Rembrandt ACP device
 */

// C dependencies:
// linux/module.h, linux/pci.h, linux/platform_device.h, sound/sof.h,
// sound/soc-acpi.h, ../ops.h, ../sof-pci-dev.h, ../../amd/mach-config.h,
// acp.h, acp-dsp-offset.h

pub const ACP6x_REG_START: u32 = 0x1240000;
pub const ACP6x_REG_END: u32 = 0x125C000;
pub const ACP6X_FUTURE_REG_ACLK_0: u32 = 0x1854;

#[repr(C)]
pub struct sof_amd_acp_desc {
    pub pgfsm_base: u32,
    pub ext_intr_stat: u32,
    pub dsp_intr_base: u32,
    pub acp_error_stat: u32,
    pub acp_sw0_i2s_err_reason: u32,
    pub sram_pte_offset: u32,
    pub hw_semaphore_offset: u32,
    pub fusion_dsp_offset: u32,
    pub probe_reg_offset: u32,
}

#[repr(C)]
pub struct sof_dev_desc {
    pub machines: *const core::ffi::c_void,
    pub resindex_lpe_base: core::ffi::c_int,
    pub resindex_pcicfg_base: core::ffi::c_int,
    pub resindex_imr_base: core::ffi::c_int,
    pub irqindex_host_ipc: core::ffi::c_int,
    pub chip_info: *const sof_amd_acp_desc,
    pub ipc_supported_mask: core::ffi::c_uint,
    pub ipc_default: core::ffi::c_uint,
    pub default_fw_path: [*const core::ffi::c_char; SOF_IPC_TYPE_COUNT],
    pub default_tplg_path: [*const core::ffi::c_char; SOF_IPC_TYPE_COUNT],
    pub default_fw_filename: [*const core::ffi::c_char; SOF_IPC_TYPE_COUNT],
    pub nocodec_tplg_filename: *const core::ffi::c_char,
    pub ops: *const core::ffi::c_void,
    pub ops_init: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> core::ffi::c_int>,
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
    pub driver_data: usize,
}

#[repr(C)]
pub struct pci_driver {
    pub name: *const core::ffi::c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> core::ffi::c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut pci_dev)>,
}

unsafe extern "C" {
    static snd_soc_acpi_amd_rmb_sof_machines: core::ffi::c_void;
    static sof_rembrandt_ops: core::ffi::c_void;
    static KBUILD_MODNAME: core::ffi::c_char;

    fn sof_rembrandt_ops_init(arg: *mut core::ffi::c_void) -> core::ffi::c_int;
    fn snd_amd_acp_find_config(pci: *mut pci_dev) -> core::ffi::c_uint;
    fn sof_pci_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> core::ffi::c_int;
    fn sof_pci_remove(pci: *mut pci_dev);
}

unsafe extern "C" {
    static ACP6X_PGFSM_BASE: u32;
    static ACP6X_EXT_INTR_STAT: u32;
    static ACP6X_DSP_SW_INTR_BASE: u32;
    static ACP6X_ERROR_STATUS: u32;
    static ACP6X_SW0_I2S_ERROR_REASON: u32;
    static ACP6X_SRAM_PTE_OFFSET: u32;
    static ACP6X_AXI2DAGB_SEM_0: u32;
    static ACP6X_DSP_FUSION_RUNSTALL: u32;
}

unsafe extern "C" {
    static SOF_IPC_TYPE_3: usize;
    static ACP_RMB_PCI_ID: u8;
    static FLAG_AMD_SOF: core::ffi::c_uint;
    static FLAG_AMD_SOF_ONLY_DMIC: core::ffi::c_uint;
    static ENODEV: core::ffi::c_int;
    static PCI_VENDOR_ID_AMD: u32;
    static ACP_PCI_DEV_ID: u32;
}

pub const SOF_IPC_TYPE_COUNT: usize = 4;

const fn bit(nr: usize) -> core::ffi::c_uint {
    1u32 << nr
}

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const core::ffi::c_char
    };
}

static rembrandt_chip_info: sof_amd_acp_desc = unsafe {
    sof_amd_acp_desc {
        pgfsm_base: ACP6X_PGFSM_BASE,
        ext_intr_stat: ACP6X_EXT_INTR_STAT,
        dsp_intr_base: ACP6X_DSP_SW_INTR_BASE,
        acp_error_stat: ACP6X_ERROR_STATUS,
        acp_sw0_i2s_err_reason: ACP6X_SW0_I2S_ERROR_REASON,
        sram_pte_offset: ACP6X_SRAM_PTE_OFFSET,
        hw_semaphore_offset: ACP6X_AXI2DAGB_SEM_0,
        fusion_dsp_offset: ACP6X_DSP_FUSION_RUNSTALL,
        probe_reg_offset: ACP6X_FUTURE_REG_ACLK_0,
    }
};

static rembrandt_desc: sof_dev_desc = unsafe {
    let mut default_fw_path = [core::ptr::null(); SOF_IPC_TYPE_COUNT];
    let mut default_tplg_path = [core::ptr::null(); SOF_IPC_TYPE_COUNT];
    let mut default_fw_filename = [core::ptr::null(); SOF_IPC_TYPE_COUNT];

    default_fw_path[SOF_IPC_TYPE_3] = c_str!("amd/sof");
    default_tplg_path[SOF_IPC_TYPE_3] = c_str!("amd/sof-tplg");
    default_fw_filename[SOF_IPC_TYPE_3] = c_str!("sof-rmb.ri");

    sof_dev_desc {
        machines: &snd_soc_acpi_amd_rmb_sof_machines as *const _ as *const core::ffi::c_void,
        resindex_lpe_base: 0,
        resindex_pcicfg_base: -1,
        resindex_imr_base: -1,
        irqindex_host_ipc: -1,
        chip_info: &rembrandt_chip_info,
        ipc_supported_mask: bit(SOF_IPC_TYPE_3),
        ipc_default: SOF_IPC_TYPE_3 as core::ffi::c_uint,
        default_fw_path,
        default_tplg_path,
        default_fw_filename,
        nocodec_tplg_filename: c_str!("sof-acp.tplg"),
        ops: &sof_rembrandt_ops as *const _ as *const core::ffi::c_void,
        ops_init: Some(sof_rembrandt_ops_init),
    }
};

unsafe extern "C" fn acp_pci_rmb_probe(
    pci: *mut pci_dev,
    pci_id: *const pci_device_id,
) -> core::ffi::c_int {
    let flag: core::ffi::c_uint;

    if unsafe { (*pci).revision != ACP_RMB_PCI_ID } {
        return unsafe { -ENODEV };
    }

    flag = unsafe { snd_amd_acp_find_config(pci) };
    if unsafe { flag != FLAG_AMD_SOF && flag != FLAG_AMD_SOF_ONLY_DMIC } {
        return unsafe { -ENODEV };
    }

    unsafe { sof_pci_probe(pci, pci_id) }
}

unsafe extern "C" fn acp_pci_rmb_remove(pci: *mut pci_dev) {
    unsafe {
        sof_pci_remove(pci);
    }
}

/* PCI IDs */
static rmb_pci_ids: [pci_device_id; 2] = unsafe {
    [
        pci_device_id {
            vendor: PCI_VENDOR_ID_AMD,
            device: ACP_PCI_DEV_ID,
            subvendor: 0xffffffff,
            subdevice: 0xffffffff,
            class: 0,
            class_mask: 0,
            driver_data: &rembrandt_desc as *const _ as usize,
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
// MODULE_DEVICE_TABLE(pci, rmb_pci_ids);

/* pci_driver definition */
static mut snd_sof_pci_amd_rmb_driver: pci_driver = pci_driver {
    name: unsafe { &KBUILD_MODNAME as *const core::ffi::c_char },
    id_table: rmb_pci_ids.as_ptr(),
    probe: Some(acp_pci_rmb_probe),
    remove: Some(acp_pci_rmb_remove),
};
// module_pci_driver(snd_sof_pci_amd_rmb_driver);

// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("REMBRANDT SOF Driver");
// MODULE_IMPORT_NS("SND_SOC_SOF_AMD_COMMON");
// MODULE_IMPORT_NS("SND_SOC_SOF_PCI_DEV");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
