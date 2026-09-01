// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2021 Advanced Micro Devices, Inc. All rights reserved.
//
// Authors: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>

/*
 * PCI interface for Renoir ACP device
 */

// C includes translated as external dependency intent:
// <linux/module.h>, <linux/pci.h>, <linux/platform_device.h>,
// <sound/sof.h>, <sound/soc-acpi.h>, "../ops.h", "../sof-pci-dev.h",
// "../../amd/mach-config.h", "acp.h", "acp-dsp-offset.h".

const ACP3x_REG_START: u32 = 0x1240000;
const ACP3x_REG_END: u32 = 0x125C000;
const ACP3X_FUTURE_REG_ACLK_0: u32 = 0x1860;

const ENODEV: i32 = 19;
const FLAG_AMD_SOF: u32 = 1;
const FLAG_AMD_SOF_ONLY_DMIC: u32 = 2;
const SOF_IPC_TYPE_3: usize = 3;
const PCI_VENDOR_ID_AMD: u32 = 0x1022;

const fn BIT(nr: usize) -> u32 {
    1u32 << nr
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
    pub name: *const ::core::ffi::c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut pci_dev)>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct device_driver {
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sof_amd_acp_desc {
    pub pgfsm_base: u32,
    pub ext_intr_stat: u32,
    pub dsp_intr_base: u32,
    pub acp_error_stat: u32,
    pub acp_sw0_i2s_err_reason: u32,
    pub sram_pte_offset: u32,
    pub hw_semaphore_offset: u32,
    pub acp_clkmux_sel: u32,
    pub probe_reg_offset: u32,
}

#[repr(C)]
pub struct sof_dev_desc {
    pub machines: *const snd_soc_acpi_mach,
    pub use_acpi_target_states: bool,
    pub resindex_lpe_base: i32,
    pub resindex_pcicfg_base: i32,
    pub resindex_imr_base: i32,
    pub irqindex_host_ipc: i32,
    pub chip_info: *const sof_amd_acp_desc,
    pub ipc_supported_mask: u32,
    pub ipc_default: usize,
    pub default_fw_path: [*const ::core::ffi::c_char; 4],
    pub default_tplg_path: [*const ::core::ffi::c_char; 4],
    pub default_fw_filename: [*const ::core::ffi::c_char; 4],
    pub nocodec_tplg_filename: *const ::core::ffi::c_char,
    pub ops: *const sof_ops,
    pub ops_init: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> i32>,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sof_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_dev {
    _private: [u8; 0],
}

unsafe extern "C" {
    static snd_soc_acpi_amd_sof_machines: snd_soc_acpi_mach;
    static sof_renoir_ops: sof_ops;
    static sof_pci_pm: dev_pm_ops;
    static KBUILD_MODNAME: ::core::ffi::c_char;

    static ACP3X_PGFSM_BASE: u32;
    static ACP3X_EXT_INTR_STAT: u32;
    static ACP3X_DSP_SW_INTR_BASE: u32;
    static ACP3X_ERROR_STATUS: u32;
    static ACP3X_SW_I2S_ERROR_REASON: u32;
    static ACP3X_SRAM_PTE_OFFSET: u32;
    static ACP3X_AXI2DAGB_SEM_0: u32;
    static ACP3X_CLKMUX_SEL: u32;
    static ACP_RN_PCI_ID: u8;
    static ACP_PCI_DEV_ID: u32;

    fn sof_renoir_ops_init(sdev: *mut snd_sof_dev) -> i32;
    fn snd_amd_acp_find_config(pci: *mut pci_dev) -> u32;
    fn sof_pci_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> i32;
    fn sof_pci_remove(pci: *mut pci_dev);
}

static renoir_chip_info: sof_amd_acp_desc = sof_amd_acp_desc {
    pgfsm_base: unsafe { ACP3X_PGFSM_BASE },
    ext_intr_stat: unsafe { ACP3X_EXT_INTR_STAT },
    dsp_intr_base: unsafe { ACP3X_DSP_SW_INTR_BASE },
    acp_error_stat: unsafe { ACP3X_ERROR_STATUS },
    acp_sw0_i2s_err_reason: unsafe { ACP3X_SW_I2S_ERROR_REASON },
    sram_pte_offset: unsafe { ACP3X_SRAM_PTE_OFFSET },
    hw_semaphore_offset: unsafe { ACP3X_AXI2DAGB_SEM_0 },
    acp_clkmux_sel: unsafe { ACP3X_CLKMUX_SEL },
    probe_reg_offset: ACP3X_FUTURE_REG_ACLK_0,
};

static renoir_desc: sof_dev_desc = sof_dev_desc {
    machines: unsafe { &snd_soc_acpi_amd_sof_machines },
    use_acpi_target_states: true,
    resindex_lpe_base: 0,
    resindex_pcicfg_base: -1,
    resindex_imr_base: -1,
    irqindex_host_ipc: -1,
    chip_info: &renoir_chip_info,
    ipc_supported_mask: BIT(SOF_IPC_TYPE_3),
    ipc_default: SOF_IPC_TYPE_3,
    default_fw_path: [
        ::core::ptr::null(),
        ::core::ptr::null(),
        ::core::ptr::null(),
        b"amd/sof\0".as_ptr() as *const ::core::ffi::c_char,
    ],
    default_tplg_path: [
        ::core::ptr::null(),
        ::core::ptr::null(),
        ::core::ptr::null(),
        b"amd/sof-tplg\0".as_ptr() as *const ::core::ffi::c_char,
    ],
    default_fw_filename: [
        ::core::ptr::null(),
        ::core::ptr::null(),
        ::core::ptr::null(),
        b"sof-rn.ri\0".as_ptr() as *const ::core::ffi::c_char,
    ],
    nocodec_tplg_filename: b"sof-acp.tplg\0".as_ptr() as *const ::core::ffi::c_char,
    ops: unsafe { &sof_renoir_ops },
    ops_init: Some(sof_renoir_ops_init),
};

unsafe extern "C" fn acp_pci_rn_probe(
    pci: *mut pci_dev,
    pci_id: *const pci_device_id,
) -> i32 {
    let flag: u32;

    if unsafe { (*pci).revision != ACP_RN_PCI_ID } {
        return -ENODEV;
    }

    flag = unsafe { snd_amd_acp_find_config(pci) };
    if flag != FLAG_AMD_SOF && flag != FLAG_AMD_SOF_ONLY_DMIC {
        return -ENODEV;
    }

    unsafe { sof_pci_probe(pci, pci_id) }
}

unsafe extern "C" fn acp_pci_rn_remove(pci: *mut pci_dev) {
    return unsafe { sof_pci_remove(pci) };
}

/* PCI IDs */
static rn_pci_ids: [pci_device_id; 2] = [
    pci_device_id {
        vendor: PCI_VENDOR_ID_AMD,
        device: unsafe { ACP_PCI_DEV_ID },
        subvendor: !0,
        subdevice: !0,
        class: 0,
        class_mask: 0,
        driver_data: &renoir_desc as *const sof_dev_desc as usize,
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
];
// MODULE_DEVICE_TABLE(pci, rn_pci_ids);

/* pci_driver definition */
static mut snd_sof_pci_amd_rn_driver: pci_driver = pci_driver {
    name: unsafe { &KBUILD_MODNAME },
    id_table: rn_pci_ids.as_ptr(),
    probe: Some(acp_pci_rn_probe),
    remove: Some(acp_pci_rn_remove),
    driver: device_driver {
        pm: unsafe { &sof_pci_pm },
    },
};
// module_pci_driver(snd_sof_pci_amd_rn_driver);

// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("RENOIR SOF Driver");
// MODULE_IMPORT_NS("SND_SOC_SOF_AMD_COMMON");
// MODULE_IMPORT_NS("SND_SOC_SOF_PCI_DEV");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
