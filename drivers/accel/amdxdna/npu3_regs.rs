// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2026, Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the surrounding driver translation unit:
// drm/amdxdna_accel.h, drm/drm_device.h, aie4_pci.h, amdxdna_pci_drv.h

const NPU3_MBOX_BAR: u32 = 0;

const NPU3_MBOX_BUFFER_BAR: u32 = 2;
const NPU3_MBOX_INFO_OFF: u32 = 0x0;

const NPU3_DOORBELL_BAR: u32 = 2;
const NPU3_DOORBELL_OFF: u32 = 0x0;

/* PCIe BAR Index for NPU3 */
const NPU3_REG_BAR_INDEX: u32 = 0;
const NPU3_PSP_BAR_INDEX: u32 = 4;
const NPU3_SMU_BAR_INDEX: u32 = 5;

const MMNPU_APERTURE3_BASE: u32 = 0x3810000;
const MMNPU_APERTURE4_BASE: u32 = 0x3B10000;

const NPU3_PSP_BAR_BASE: u32 = MMNPU_APERTURE3_BASE;
const NPU3_SMU_BAR_BASE: u32 = MMNPU_APERTURE4_BASE;

const MPASP_C2PMSG_123_ALT_1: u32 = 0x3810AEC;
const MPASP_C2PMSG_156_ALT_1: u32 = 0x3810B70;
const MPASP_C2PMSG_157_ALT_1: u32 = 0x3810B74;
const MPASP_C2PMSG_73_ALT_1: u32 = 0x3810A24;

const MP1_C2PMSG_59_ALT_1: u32 = 0x3B109EC;
const MP1_C2PMSG_61_ALT_1: u32 = 0x3B109F4;
const MP1_C2PMSG_60_ALT_1: u32 = 0x3B109F0;

static NPU3_FW_FEATURE_TABLE: [amdxdna_fw_feature_tbl; 2] = [
    amdxdna_fw_feature_tbl { major: 5, min_minor: 10 },
    amdxdna_fw_feature_tbl { major: 0, min_minor: 0 },
];

static NPU3_DEV_PRIV: amdxdna_dev_priv = amdxdna_dev_priv {
    npufw_path: "npu.dev.sbin",
    certfw_path: "cert.dev.sbin",
    mbox_bar: NPU3_MBOX_BAR,
    mbox_rbuf_bar: NPU3_MBOX_BUFFER_BAR,
    mbox_info_off: NPU3_MBOX_INFO_OFF,
    doorbell_off: NPU3_DOORBELL_OFF,
    psp_regs_off: [
        define_bar_offset!(PSP_CMD_REG, NPU3_PSP, MPASP_C2PMSG_123_ALT_1),
        define_bar_offset!(PSP_ARG0_REG, NPU3_PSP, MPASP_C2PMSG_156_ALT_1),
        define_bar_offset!(PSP_ARG1_REG, NPU3_PSP, MPASP_C2PMSG_157_ALT_1),
        define_bar_offset!(PSP_ARG2_REG, NPU3_PSP, MPASP_C2PMSG_123_ALT_1),
        define_bar_offset!(PSP_INTR_REG, NPU3_PSP, MPASP_C2PMSG_73_ALT_1),
        define_bar_offset!(PSP_STATUS_REG, NPU3_PSP, MPASP_C2PMSG_123_ALT_1),
        define_bar_offset!(PSP_RESP_REG, NPU3_PSP, MPASP_C2PMSG_156_ALT_1),
        // npu3 doesn't use 8th pwaitmode register
    ],
    smu_regs_off: [
        define_bar_offset!(SMU_CMD_REG, NPU3_SMU, MP1_C2PMSG_59_ALT_1),
        define_bar_offset!(SMU_ARG_REG, NPU3_SMU, MP1_C2PMSG_61_ALT_1),
        define_bar_offset!(SMU_INTR_REG, NPU3_SMU, MMNPU_APERTURE4_BASE),
        define_bar_offset!(SMU_RESP_REG, NPU3_SMU, MP1_C2PMSG_60_ALT_1),
        define_bar_offset!(SMU_OUT_REG, NPU3_SMU, MP1_C2PMSG_61_ALT_1),
    ],
};

static NPU3_DEV_VF_PRIV: amdxdna_dev_priv = amdxdna_dev_priv {
    // vf device does not load firmware
    mbox_bar: NPU3_MBOX_BAR,
    mbox_rbuf_bar: NPU3_MBOX_BUFFER_BAR,
    mbox_info_off: NPU3_MBOX_INFO_OFF,
    // vf device does not have smu and psp
};

pub static mut DEV_NPU3_PF_INFO: amdxdna_dev_info = amdxdna_dev_info {
    mbox_bar: NPU3_MBOX_BAR,
    sram_bar: NPU3_MBOX_BUFFER_BAR,
    psp_bar: NPU3_PSP_BAR_INDEX,
    smu_bar: NPU3_SMU_BAR_INDEX,
    default_vbnv: "RyzenAI-npu3-pf",
    device_type: AMDXDNA_DEV_TYPE_PF,
    dev_priv: &NPU3_DEV_PRIV,
    fw_feature_tbl: &NPU3_FW_FEATURE_TABLE,
    ops: &aie4_pf_ops,
};

pub static mut DEV_NPU3_VF_INFO: amdxdna_dev_info = amdxdna_dev_info {
    mbox_bar: NPU3_MBOX_BAR,
    sram_bar: NPU3_MBOX_BUFFER_BAR,
    doorbell_bar: NPU3_DOORBELL_BAR,
    default_vbnv: "RyzenAI-npu3-vf",
    device_type: AMDXDNA_DEV_TYPE_UMQ,
    dev_priv: &NPU3_DEV_VF_PRIV,
    fw_feature_tbl: &NPU3_FW_FEATURE_TABLE,
    ops: &aie4_vf_ops,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
