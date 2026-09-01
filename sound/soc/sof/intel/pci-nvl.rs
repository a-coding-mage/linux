// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2025 Intel Corporation.
//

// C includes translated as external dependency intent:
// linux/module.h
// linux/pci.h
// sound/soc-acpi.h
// sound/soc-acpi-intel-match.h
// sound/sof.h
// ../ops.h
// ../sof-pci-dev.h

/* platform specific devices */
// hda.h
// nvl.h

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
pub struct sof_dev_desc {
    pub use_acpi_target_states: bool,
    pub machines: *const snd_soc_acpi_mach,
    pub alt_machines: *const snd_soc_acpi_mach,
    pub resindex_lpe_base: i32,
    pub resindex_pcicfg_base: i32,
    pub resindex_imr_base: i32,
    pub irqindex_host_ipc: i32,
    pub chip_info: *const sof_intel_dsp_desc,
    pub ipc_supported_mask: u32,
    pub ipc_default: u32,
    pub dspless_mode_supported: bool,
    pub on_demand_dsp_boot: bool,
    pub default_fw_path: [*const u8; SOF_IPC_TYPE_COUNT],
    pub default_lib_path: [*const u8; SOF_IPC_TYPE_COUNT],
    pub default_tplg_path: [*const u8; SOF_IPC_TYPE_COUNT],
    pub default_fw_filename: [*const u8; SOF_IPC_TYPE_COUNT],
    pub nocodec_tplg_filename: *const u8,
    pub ops: *mut snd_sof_dsp_ops,
    pub ops_init: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> i32>,
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
    pub name: *const u8,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn() -> i32>,
    pub remove: Option<unsafe extern "C" fn()>,
    pub shutdown: Option<unsafe extern "C" fn()>,
    pub driver: device_driver,
}

const SOF_IPC_TYPE_4: usize = 4;
const SOF_IPC_TYPE_COUNT: usize = SOF_IPC_TYPE_4 + 1;

const PCI_VENDOR_ID_INTEL: u32 = 0x8086;
const PCI_ANY_ID: u32 = !0u32;

const fn BIT(nr: usize) -> u32 {
    1u32 << nr
}

const fn PCI_DEVICE_DATA(vend: u32, dev: u32, data: *const sof_dev_desc) -> pci_device_id {
    pci_device_id {
        vendor: vend,
        device: dev,
        subvendor: PCI_ANY_ID,
        subdevice: PCI_ANY_ID,
        class: 0,
        class_mask: 0,
        driver_data: data as usize,
    }
}

unsafe extern "C" {
    static snd_soc_acpi_intel_nvl_machines: snd_soc_acpi_mach;
    static snd_soc_acpi_intel_nvl_sdw_machines: snd_soc_acpi_mach;
    static nvl_chip_info: sof_intel_dsp_desc;
    static nvl_s_chip_info: sof_intel_dsp_desc;
    static sof_pci_pm: dev_pm_ops;

    static HDA_NVL: u32;
    static HDA_NVL_S: u32;

    fn sof_nvl_set_ops(sdev: *mut snd_sof_dev, ops: *mut snd_sof_dsp_ops) -> i32;
    fn hda_pci_intel_probe() -> i32;
    fn sof_pci_remove();
    fn sof_pci_shutdown();
}

/* PantherLake ops */
static mut sof_nvl_ops: snd_sof_dsp_ops = snd_sof_dsp_ops { _private: [] };

unsafe extern "C" fn sof_nvl_ops_init(sdev: *mut snd_sof_dev) -> i32 {
    unsafe { sof_nvl_set_ops(sdev, &raw mut sof_nvl_ops) }
}

static nvl_desc: sof_dev_desc = sof_dev_desc {
    use_acpi_target_states: true,
    machines: unsafe { &snd_soc_acpi_intel_nvl_machines },
    alt_machines: unsafe { &snd_soc_acpi_intel_nvl_sdw_machines },
    resindex_lpe_base: 0,
    resindex_pcicfg_base: -1,
    resindex_imr_base: -1,
    irqindex_host_ipc: -1,
    chip_info: unsafe { &nvl_chip_info },
    ipc_supported_mask: BIT(SOF_IPC_TYPE_4),
    ipc_default: SOF_IPC_TYPE_4 as u32,
    dspless_mode_supported: true,
    on_demand_dsp_boot: true,
    default_fw_path: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        c"intel/sof-ipc4/nvl".as_ptr() as *const u8,
    ],
    default_lib_path: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        c"intel/sof-ipc4-lib/nvl".as_ptr() as *const u8,
    ],
    default_tplg_path: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        c"intel/sof-ipc4-tplg".as_ptr() as *const u8,
    ],
    default_fw_filename: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        c"sof-nvl.ri".as_ptr() as *const u8,
    ],
    nocodec_tplg_filename: c"sof-nvl-nocodec.tplg".as_ptr() as *const u8,
    ops: unsafe { &raw mut sof_nvl_ops },
    ops_init: Some(sof_nvl_ops_init),
};

static nvl_s_desc: sof_dev_desc = sof_dev_desc {
    use_acpi_target_states: true,
    machines: unsafe { &snd_soc_acpi_intel_nvl_machines },
    alt_machines: unsafe { &snd_soc_acpi_intel_nvl_sdw_machines },
    resindex_lpe_base: 0,
    resindex_pcicfg_base: -1,
    resindex_imr_base: -1,
    irqindex_host_ipc: -1,
    chip_info: unsafe { &nvl_s_chip_info },
    ipc_supported_mask: BIT(SOF_IPC_TYPE_4),
    ipc_default: SOF_IPC_TYPE_4 as u32,
    dspless_mode_supported: true,
    on_demand_dsp_boot: true,
    default_fw_path: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        c"intel/sof-ipc4/nvl-s".as_ptr() as *const u8,
    ],
    default_lib_path: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        c"intel/sof-ipc4-lib/nvl-s".as_ptr() as *const u8,
    ],
    default_tplg_path: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        c"intel/sof-ipc4-tplg".as_ptr() as *const u8,
    ],
    default_fw_filename: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        c"sof-nvl-s.ri".as_ptr() as *const u8,
    ],
    nocodec_tplg_filename: c"sof-nvl-nocodec.tplg".as_ptr() as *const u8,
    ops: unsafe { &raw mut sof_nvl_ops },
    ops_init: Some(sof_nvl_ops_init),
};

/* PCI IDs */
static sof_pci_ids: [pci_device_id; 3] = [
    unsafe { PCI_DEVICE_DATA(PCI_VENDOR_ID_INTEL, HDA_NVL, &nvl_desc) }, /* NVL */
    unsafe { PCI_DEVICE_DATA(PCI_VENDOR_ID_INTEL, HDA_NVL_S, &nvl_s_desc) }, /* NVL-S */
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
static mut snd_sof_pci_intel_nvl_driver: pci_driver = pci_driver {
    name: c"sof-audio-pci-intel-nvl".as_ptr() as *const u8,
    id_table: sof_pci_ids.as_ptr(),
    probe: Some(hda_pci_intel_probe),
    remove: Some(sof_pci_remove),
    shutdown: Some(sof_pci_shutdown),
    driver: device_driver {
        pm: unsafe { &sof_pci_pm },
    },
};
// module_pci_driver(snd_sof_pci_intel_nvl_driver);

// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("SOF support for NovaLake platforms");
// MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_HDA_GENERIC");
// MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_HDA_COMMON");
// MODULE_IMPORT_NS("SND_SOC_SOF_PCI_DEV");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
