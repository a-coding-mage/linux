// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2023 Advanced Micro Devices, Inc. All rights reserved.
//
// Authors: Venkata Prasad Potturu <venkataprasad.potturu@amd.com>

/*.
 * PCI interface for Vangogh ACP device
 */

// C dependencies:
// #include <linux/module.h>
// #include <linux/pci.h>
// #include <sound/sof.h>
// #include <sound/soc-acpi.h>
// #include "../sof-pci-dev.h"
// #include "../../amd/mach-config.h"
// #include "acp.h"
// #include "acp-dsp-offset.h"

const ACP5X_FUTURE_REG_ACLK_0: u32 = 0x1864;

extern "C" {
    static snd_soc_acpi_amd_vangogh_sof_machines: [SndSocAcpiMach; 0];
    static sof_vangogh_ops: SndSofDspOps;
    static sof_pci_pm: DevPmOps;

    fn sof_vangogh_ops_init(sdev: *mut SndSofDev) -> c_int;
    fn snd_amd_acp_find_config(pci: *mut PciDev) -> c_uint;
    fn sof_pci_probe(pci: *mut PciDev, pci_id: *const PciDeviceId) -> c_int;
    fn sof_pci_remove(pci: *mut PciDev);
}

static vangogh_chip_info: SofAmdAcpDesc = SofAmdAcpDesc {
    name: c_str!("vangogh"),
    pgfsm_base: ACP5X_PGFSM_BASE,
    ext_intr_stat: ACP5X_EXT_INTR_STAT,
    dsp_intr_base: ACP5X_DSP_SW_INTR_BASE,
    sram_pte_offset: ACP5X_SRAM_PTE_OFFSET,
    hw_semaphore_offset: ACP5X_AXI2DAGB_SEM_0,
    probe_reg_offset: ACP5X_FUTURE_REG_ACLK_0,
};

static vangogh_desc: SofDevDesc = SofDevDesc {
    machines: unsafe { snd_soc_acpi_amd_vangogh_sof_machines.as_ptr() },
    resindex_lpe_base: 0,
    resindex_pcicfg_base: -1,
    resindex_imr_base: -1,
    irqindex_host_ipc: -1,
    chip_info: &vangogh_chip_info,
    ipc_supported_mask: BIT(SOF_IPC_TYPE_3),
    ipc_default: SOF_IPC_TYPE_3,
    default_fw_path: {
        let mut value = [core::ptr::null(); SOF_IPC_TYPE_COUNT];
        value[SOF_IPC_TYPE_3 as usize] = c_str!("amd/sof");
        value
    },
    default_tplg_path: {
        let mut value = [core::ptr::null(); SOF_IPC_TYPE_COUNT];
        value[SOF_IPC_TYPE_3 as usize] = c_str!("amd/sof-tplg");
        value
    },
    default_fw_filename: {
        let mut value = [core::ptr::null(); SOF_IPC_TYPE_COUNT];
        value[SOF_IPC_TYPE_3 as usize] = c_str!("sof-vangogh.ri");
        value
    },
    nocodec_tplg_filename: c_str!("sof-acp.tplg"),
    ops: unsafe { &sof_vangogh_ops },
    ops_init: Some(sof_vangogh_ops_init),
};

unsafe extern "C" fn acp_pci_vgh_probe(
    pci: *mut PciDev,
    pci_id: *const PciDeviceId,
) -> c_int {
    let flag: c_uint;

    if unsafe { (*pci).revision } != ACP_VANGOGH_PCI_ID {
        return -ENODEV;
    }

    flag = unsafe { snd_amd_acp_find_config(pci) };
    if flag != FLAG_AMD_SOF && flag != FLAG_AMD_SOF_ONLY_DMIC {
        return -ENODEV;
    }

    unsafe { sof_pci_probe(pci, pci_id) }
}

unsafe extern "C" fn acp_pci_vgh_remove(pci: *mut PciDev) {
    unsafe {
        sof_pci_remove(pci);
    }
}

/* PCI IDs */
static vgh_pci_ids: [PciDeviceId; 2] = [
    PciDeviceId {
        vendor: PCI_VENDOR_ID_AMD,
        device: ACP_PCI_DEV_ID,
        subvendor: PCI_ANY_ID,
        subdevice: PCI_ANY_ID,
        class: 0,
        class_mask: 0,
        driver_data: &vangogh_desc as *const SofDevDesc as c_ulong,
    },
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
module_device_table!(pci, vgh_pci_ids);

/* pci_driver definition */
static mut snd_sof_pci_amd_vgh_driver: PciDriver = PciDriver {
    name: KBUILD_MODNAME,
    id_table: vgh_pci_ids.as_ptr(),
    probe: Some(acp_pci_vgh_probe),
    remove: Some(acp_pci_vgh_remove),
    driver: DeviceDriver {
        pm: pm_ptr(unsafe { &sof_pci_pm }),
    },
};
module_pci_driver!(snd_sof_pci_amd_vgh_driver);

module_license!("Dual BSD/GPL");
module_description!("VANGOGH SOF Driver");
module_import_ns!("SND_SOC_SOF_AMD_COMMON");
module_import_ns!("SND_SOC_SOF_PCI_DEV");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
