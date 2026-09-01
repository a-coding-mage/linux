// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018-2021 Intel Corporation
//
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//

// C dependencies:
// linux/module.h
// linux/pci.h
// sound/soc-acpi.h
// sound/soc-acpi-intel-match.h
// sound/sof.h
// ../ops.h
// ../sof-pci-dev.h
// hda.h

const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

unsafe extern "C" {
    static snd_soc_acpi_intel_tgl_machines: *const core::ffi::c_void;
    static snd_soc_acpi_intel_tgl_sdw_machines: *const core::ffi::c_void;
    static snd_soc_acpi_intel_ehl_machines: *const core::ffi::c_void;
    static snd_soc_acpi_intel_adl_machines: *const core::ffi::c_void;
    static snd_soc_acpi_intel_adl_sdw_machines: *const core::ffi::c_void;
    static snd_soc_acpi_intel_rpl_machines: *const core::ffi::c_void;
    static snd_soc_acpi_intel_rpl_sdw_machines: *const core::ffi::c_void;

    static tgl_chip_info: core::ffi::c_void;
    static tglh_chip_info: core::ffi::c_void;
    static ehl_chip_info: core::ffi::c_void;
    static adls_chip_info: core::ffi::c_void;

    static sof_tgl_ops: core::ffi::c_void;
    static sof_pci_pm: core::ffi::c_void;

    fn sof_tgl_ops_init(sdev: *mut core::ffi::c_void) -> core::ffi::c_int;
    fn hda_ops_free(sdev: *mut core::ffi::c_void);
    fn hda_pci_intel_probe(pci: *mut pci_dev, id: *const pci_device_id) -> core::ffi::c_int;
    fn sof_pci_remove(pci: *mut pci_dev);
    fn sof_pci_shutdown(pci: *mut pci_dev);
}

/* platform specific devices */

static tgl_desc: sof_dev_desc = sof_dev_desc {
    machines: unsafe { snd_soc_acpi_intel_tgl_machines },
    alt_machines: unsafe { snd_soc_acpi_intel_tgl_sdw_machines },
    use_acpi_target_states: true,
    resindex_lpe_base: 0,
    resindex_pcicfg_base: -1,
    resindex_imr_base: -1,
    irqindex_host_ipc: -1,
    chip_info: unsafe { &tgl_chip_info },
    ipc_supported_mask: BIT(SOF_IPC_TYPE_3) | BIT(SOF_IPC_TYPE_4),
    ipc_default: SOF_IPC_TYPE_3,
    dspless_mode_supported: true, /* Only supported for HDaudio */
    default_fw_path: sof_ipc_string_array! {
        SOF_IPC_TYPE_3 => c"intel/sof",
        SOF_IPC_TYPE_4 => c"intel/sof-ipc4/tgl",
    },
    default_lib_path: sof_ipc_string_array! {
        SOF_IPC_TYPE_4 => c"intel/sof-ipc4-lib/tgl",
    },
    default_tplg_path: sof_ipc_string_array! {
        SOF_IPC_TYPE_3 => c"intel/sof-tplg",
        SOF_IPC_TYPE_4 => c"intel/sof-ipc4-tplg",
    },
    default_fw_filename: sof_ipc_string_array! {
        SOF_IPC_TYPE_3 => c"sof-tgl.ri",
        SOF_IPC_TYPE_4 => c"sof-tgl.ri",
    },
    nocodec_tplg_filename: c"sof-tgl-nocodec.tplg".as_ptr(),
    ops: unsafe { &sof_tgl_ops },
    ops_init: Some(sof_tgl_ops_init),
    ops_free: Some(hda_ops_free),
};

static tglh_desc: sof_dev_desc = sof_dev_desc {
    machines: unsafe { snd_soc_acpi_intel_tgl_machines },
    alt_machines: unsafe { snd_soc_acpi_intel_tgl_sdw_machines },
    use_acpi_target_states: true,
    resindex_lpe_base: 0,
    resindex_pcicfg_base: -1,
    resindex_imr_base: -1,
    irqindex_host_ipc: -1,
    chip_info: unsafe { &tglh_chip_info },
    ipc_supported_mask: BIT(SOF_IPC_TYPE_3) | BIT(SOF_IPC_TYPE_4),
    ipc_default: SOF_IPC_TYPE_3,
    dspless_mode_supported: true, /* Only supported for HDaudio */
    default_fw_path: sof_ipc_string_array! {
        SOF_IPC_TYPE_3 => c"intel/sof",
        SOF_IPC_TYPE_4 => c"intel/sof-ipc4/tgl-h",
    },
    default_lib_path: sof_ipc_string_array! {
        SOF_IPC_TYPE_4 => c"intel/sof-ipc4-lib/tgl-h",
    },
    default_tplg_path: sof_ipc_string_array! {
        SOF_IPC_TYPE_3 => c"intel/sof-tplg",
        SOF_IPC_TYPE_4 => c"intel/sof-ipc4-tplg",
    },
    default_fw_filename: sof_ipc_string_array! {
        SOF_IPC_TYPE_3 => c"sof-tgl-h.ri",
        SOF_IPC_TYPE_4 => c"sof-tgl-h.ri",
    },
    nocodec_tplg_filename: c"sof-tgl-nocodec.tplg".as_ptr(),
    ops: unsafe { &sof_tgl_ops },
    ops_init: Some(sof_tgl_ops_init),
    ops_free: Some(hda_ops_free),
};

static ehl_desc: sof_dev_desc = sof_dev_desc {
    machines: unsafe { snd_soc_acpi_intel_ehl_machines },
    use_acpi_target_states: true,
    resindex_lpe_base: 0,
    resindex_pcicfg_base: -1,
    resindex_imr_base: -1,
    irqindex_host_ipc: -1,
    chip_info: unsafe { &ehl_chip_info },
    ipc_supported_mask: BIT(SOF_IPC_TYPE_3) | BIT(SOF_IPC_TYPE_4),
    ipc_default: SOF_IPC_TYPE_3,
    dspless_mode_supported: true, /* Only supported for HDaudio */
    default_fw_path: sof_ipc_string_array! {
        SOF_IPC_TYPE_3 => c"intel/sof",
        SOF_IPC_TYPE_4 => c"intel/sof-ipc4/ehl",
    },
    default_lib_path: sof_ipc_string_array! {
        SOF_IPC_TYPE_4 => c"intel/sof-ipc4-lib/ehl",
    },
    default_tplg_path: sof_ipc_string_array! {
        SOF_IPC_TYPE_3 => c"intel/sof-tplg",
        SOF_IPC_TYPE_4 => c"intel/sof-ipc4-tplg",
    },
    default_fw_filename: sof_ipc_string_array! {
        SOF_IPC_TYPE_3 => c"sof-ehl.ri",
        SOF_IPC_TYPE_4 => c"sof-ehl.ri",
    },
    nocodec_tplg_filename: c"sof-ehl-nocodec.tplg".as_ptr(),
    ops: unsafe { &sof_tgl_ops },
    ops_init: Some(sof_tgl_ops_init),
    ops_free: Some(hda_ops_free),
};

static adls_desc: sof_dev_desc = sof_dev_desc {
    machines: unsafe { snd_soc_acpi_intel_adl_machines },
    alt_machines: unsafe { snd_soc_acpi_intel_adl_sdw_machines },
    use_acpi_target_states: true,
    resindex_lpe_base: 0,
    resindex_pcicfg_base: -1,
    resindex_imr_base: -1,
    irqindex_host_ipc: -1,
    chip_info: unsafe { &adls_chip_info },
    ipc_supported_mask: BIT(SOF_IPC_TYPE_3) | BIT(SOF_IPC_TYPE_4),
    ipc_default: SOF_IPC_TYPE_3,
    dspless_mode_supported: true, /* Only supported for HDaudio */
    default_fw_path: sof_ipc_string_array! {
        SOF_IPC_TYPE_3 => c"intel/sof",
        SOF_IPC_TYPE_4 => c"intel/sof-ipc4/adl-s",
    },
    default_lib_path: sof_ipc_string_array! {
        SOF_IPC_TYPE_4 => c"intel/sof-ipc4-lib/adl-s",
    },
    default_tplg_path: sof_ipc_string_array! {
        SOF_IPC_TYPE_3 => c"intel/sof-tplg",
        SOF_IPC_TYPE_4 => c"intel/sof-ipc4-tplg",
    },
    default_fw_filename: sof_ipc_string_array! {
        SOF_IPC_TYPE_3 => c"sof-adl-s.ri",
        SOF_IPC_TYPE_4 => c"sof-adl-s.ri",
    },
    nocodec_tplg_filename: c"sof-adl-nocodec.tplg".as_ptr(),
    ops: unsafe { &sof_tgl_ops },
    ops_init: Some(sof_tgl_ops_init),
    ops_free: Some(hda_ops_free),
};

static adl_desc: sof_dev_desc = sof_dev_desc {
    machines: unsafe { snd_soc_acpi_intel_adl_machines },
    alt_machines: unsafe { snd_soc_acpi_intel_adl_sdw_machines },
    use_acpi_target_states: true,
    resindex_lpe_base: 0,
    resindex_pcicfg_base: -1,
    resindex_imr_base: -1,
    irqindex_host_ipc: -1,
    chip_info: unsafe { &tgl_chip_info },
    ipc_supported_mask: BIT(SOF_IPC_TYPE_3) | BIT(SOF_IPC_TYPE_4),
    ipc_default: SOF_IPC_TYPE_3,
    dspless_mode_supported: true, /* Only supported for HDaudio */
    default_fw_path: sof_ipc_string_array! {
        SOF_IPC_TYPE_3 => c"intel/sof",
        SOF_IPC_TYPE_4 => c"intel/sof-ipc4/adl",
    },
    default_lib_path: sof_ipc_string_array! {
        SOF_IPC_TYPE_4 => c"intel/sof-ipc4-lib/adl",
    },
    default_tplg_path: sof_ipc_string_array! {
        SOF_IPC_TYPE_3 => c"intel/sof-tplg",
        SOF_IPC_TYPE_4 => c"intel/sof-ipc4-tplg",
    },
    default_fw_filename: sof_ipc_string_array! {
        SOF_IPC_TYPE_3 => c"sof-adl.ri",
        SOF_IPC_TYPE_4 => c"sof-adl.ri",
    },
    nocodec_tplg_filename: c"sof-adl-nocodec.tplg".as_ptr(),
    ops: unsafe { &sof_tgl_ops },
    ops_init: Some(sof_tgl_ops_init),
    ops_free: Some(hda_ops_free),
};

static adln_desc: sof_dev_desc = sof_dev_desc {
    machines: unsafe { snd_soc_acpi_intel_adl_machines },
    alt_machines: unsafe { snd_soc_acpi_intel_adl_sdw_machines },
    use_acpi_target_states: true,
    resindex_lpe_base: 0,
    resindex_pcicfg_base: -1,
    resindex_imr_base: -1,
    irqindex_host_ipc: -1,
    chip_info: unsafe { &tgl_chip_info },
    ipc_supported_mask: BIT(SOF_IPC_TYPE_3) | BIT(SOF_IPC_TYPE_4),
    ipc_default: SOF_IPC_TYPE_3,
    dspless_mode_supported: true, /* Only supported for HDaudio */
    default_fw_path: sof_ipc_string_array! {
        SOF_IPC_TYPE_3 => c"intel/sof",
        SOF_IPC_TYPE_4 => c"intel/sof-ipc4/adl-n",
    },
    default_lib_path: sof_ipc_string_array! {
        SOF_IPC_TYPE_4 => c"intel/sof-ipc4-lib/adl-n",
    },
    default_tplg_path: sof_ipc_string_array! {
        SOF_IPC_TYPE_3 => c"intel/sof-tplg",
        SOF_IPC_TYPE_4 => c"intel/sof-ipc4-tplg",
    },
    default_fw_filename: sof_ipc_string_array! {
        SOF_IPC_TYPE_3 => c"sof-adl-n.ri",
        SOF_IPC_TYPE_4 => c"sof-adl-n.ri",
    },
    nocodec_tplg_filename: c"sof-adl-nocodec.tplg".as_ptr(),
    ops: unsafe { &sof_tgl_ops },
    ops_init: Some(sof_tgl_ops_init),
    ops_free: Some(hda_ops_free),
};

static rpls_desc: sof_dev_desc = sof_dev_desc {
    machines: unsafe { snd_soc_acpi_intel_rpl_machines },
    alt_machines: unsafe { snd_soc_acpi_intel_rpl_sdw_machines },
    use_acpi_target_states: true,
    resindex_lpe_base: 0,
    resindex_pcicfg_base: -1,
    resindex_imr_base: -1,
    irqindex_host_ipc: -1,
    chip_info: unsafe { &adls_chip_info },
    ipc_supported_mask: BIT(SOF_IPC_TYPE_3) | BIT(SOF_IPC_TYPE_4),
    ipc_default: SOF_IPC_TYPE_3,
    dspless_mode_supported: true, /* Only supported for HDaudio */
    default_fw_path: sof_ipc_string_array! {
        SOF_IPC_TYPE_3 => c"intel/sof",
        SOF_IPC_TYPE_4 => c"intel/sof-ipc4/rpl-s",
    },
    default_lib_path: sof_ipc_string_array! {
        SOF_IPC_TYPE_4 => c"intel/sof-ipc4-lib/rpl-s",
    },
    default_tplg_path: sof_ipc_string_array! {
        SOF_IPC_TYPE_3 => c"intel/sof-tplg",
        SOF_IPC_TYPE_4 => c"intel/sof-ipc4-tplg",
    },
    default_fw_filename: sof_ipc_string_array! {
        SOF_IPC_TYPE_3 => c"sof-rpl-s.ri",
        SOF_IPC_TYPE_4 => c"sof-rpl-s.ri",
    },
    nocodec_tplg_filename: c"sof-rpl-nocodec.tplg".as_ptr(),
    ops: unsafe { &sof_tgl_ops },
    ops_init: Some(sof_tgl_ops_init),
    ops_free: Some(hda_ops_free),
};

static rpl_desc: sof_dev_desc = sof_dev_desc {
    machines: unsafe { snd_soc_acpi_intel_rpl_machines },
    alt_machines: unsafe { snd_soc_acpi_intel_rpl_sdw_machines },
    use_acpi_target_states: true,
    resindex_lpe_base: 0,
    resindex_pcicfg_base: -1,
    resindex_imr_base: -1,
    irqindex_host_ipc: -1,
    chip_info: unsafe { &tgl_chip_info },
    ipc_supported_mask: BIT(SOF_IPC_TYPE_3) | BIT(SOF_IPC_TYPE_4),
    ipc_default: SOF_IPC_TYPE_3,
    dspless_mode_supported: true, /* Only supported for HDaudio */
    default_fw_path: sof_ipc_string_array! {
        SOF_IPC_TYPE_3 => c"intel/sof",
        SOF_IPC_TYPE_4 => c"intel/sof-ipc4/rpl",
    },
    default_lib_path: sof_ipc_string_array! {
        SOF_IPC_TYPE_4 => c"intel/sof-ipc4-lib/rpl",
    },
    default_tplg_path: sof_ipc_string_array! {
        SOF_IPC_TYPE_3 => c"intel/sof-tplg",
        SOF_IPC_TYPE_4 => c"intel/sof-ipc4-tplg",
    },
    default_fw_filename: sof_ipc_string_array! {
        SOF_IPC_TYPE_3 => c"sof-rpl.ri",
        SOF_IPC_TYPE_4 => c"sof-rpl.ri",
    },
    nocodec_tplg_filename: c"sof-rpl-nocodec.tplg".as_ptr(),
    ops: unsafe { &sof_tgl_ops },
    ops_init: Some(sof_tgl_ops_init),
    ops_free: Some(hda_ops_free),
};

/* PCI IDs */
static sof_pci_ids: [pci_device_id; 16] = [
    PCI_DEVICE_DATA!(INTEL, HDA_TGL_LP, &tgl_desc),
    PCI_DEVICE_DATA!(INTEL, HDA_TGL_H, &tglh_desc),
    PCI_DEVICE_DATA!(INTEL, HDA_EHL_0, &ehl_desc),
    PCI_DEVICE_DATA!(INTEL, HDA_EHL_3, &ehl_desc),
    PCI_DEVICE_DATA!(INTEL, HDA_ADL_S, &adls_desc),
    PCI_DEVICE_DATA!(INTEL, HDA_RPL_S, &rpls_desc),
    PCI_DEVICE_DATA!(INTEL, HDA_ADL_P, &adl_desc),
    PCI_DEVICE_DATA!(INTEL, HDA_ADL_PS, &adl_desc),
    PCI_DEVICE_DATA!(INTEL, HDA_RPL_P_0, &rpl_desc),
    PCI_DEVICE_DATA!(INTEL, HDA_RPL_P_1, &rpl_desc),
    PCI_DEVICE_DATA!(INTEL, HDA_ADL_M, &adl_desc),
    PCI_DEVICE_DATA!(INTEL, HDA_ADL_PX, &adl_desc),
    PCI_DEVICE_DATA!(INTEL, HDA_RPL_M, &rpl_desc),
    PCI_DEVICE_DATA!(INTEL, HDA_RPL_PX, &rpl_desc),
    PCI_DEVICE_DATA!(INTEL, HDA_ADL_N, &adln_desc),
    pci_device_id { driver_data: 0, ..PCI_DEVICE_ID_ZERO },
];
module_device_table!(pci, sof_pci_ids);

/* pci_driver definition */
static mut snd_sof_pci_intel_tgl_driver: pci_driver = pci_driver {
    name: c"sof-audio-pci-intel-tgl".as_ptr(),
    id_table: sof_pci_ids.as_ptr(),
    probe: Some(hda_pci_intel_probe),
    remove: Some(sof_pci_remove),
    shutdown: Some(sof_pci_shutdown),
    driver: device_driver {
        pm: pm_ptr!(unsafe { &sof_pci_pm }),
        ..DEVICE_DRIVER_ZERO
    },
    ..PCI_DRIVER_ZERO
};
module_pci_driver!(snd_sof_pci_intel_tgl_driver);

module_license!("Dual BSD/GPL");
module_description!("SOF support for TigerLake platforms");
module_import_ns!("SND_SOC_SOF_INTEL_HDA_GENERIC");
module_import_ns!("SND_SOC_SOF_INTEL_HDA_COMMON");
module_import_ns!("SND_SOC_SOF_INTEL_CNL");
module_import_ns!("SND_SOC_SOF_PCI_DEV");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
