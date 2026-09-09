// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2024, Advanced Micro Devices, Inc.
 */

// Dependencies are supplied by the surrounding driver.

/* NPU Public Registers on MpNPUAxiXbar (refer to Diag npu_registers.h) */
pub const MPNPU_PWAITMODE: u32 = 0x301003C;
pub const MPNPU_PUB_SEC_INTR: u32 = 0x3010060;
pub const MPNPU_PUB_PWRMGMT_INTR: u32 = 0x3010064;
pub const MPNPU_PUB_SCRATCH0: u32 = 0x301006C;
pub const MPNPU_PUB_SCRATCH1: u32 = 0x3010070;
pub const MPNPU_PUB_SCRATCH2: u32 = 0x3010074;
pub const MPNPU_PUB_SCRATCH3: u32 = 0x3010078;
pub const MPNPU_PUB_SCRATCH4: u32 = 0x301007C;
pub const MPNPU_PUB_SCRATCH5: u32 = 0x3010080;
pub const MPNPU_PUB_SCRATCH6: u32 = 0x3010084;
pub const MPNPU_PUB_SCRATCH7: u32 = 0x3010088;
pub const MPNPU_PUB_SCRATCH8: u32 = 0x301008C;
pub const MPNPU_PUB_SCRATCH9: u32 = 0x3010090;
pub const MPNPU_PUB_SCRATCH10: u32 = 0x3010094;
pub const MPNPU_PUB_SCRATCH11: u32 = 0x3010098;
pub const MPNPU_PUB_SCRATCH12: u32 = 0x301009C;
pub const MPNPU_PUB_SCRATCH13: u32 = 0x30100A0;
pub const MPNPU_PUB_SCRATCH14: u32 = 0x30100A4;
pub const MPNPU_PUB_SCRATCH15: u32 = 0x30100A8;
pub const MP0_C2PMSG_73: u32 = 0x3810A24;
pub const MP0_C2PMSG_123: u32 = 0x3810AEC;

pub const MP1_C2PMSG_0: u32 = 0x3B10900;
pub const MP1_C2PMSG_60: u32 = 0x3B109F0;
pub const MP1_C2PMSG_61: u32 = 0x3B109F4;

pub const MPNPU_SRAM_X2I_MAILBOX_0: u32 = 0x3600000;
pub const MPNPU_SRAM_X2I_MAILBOX_15: u32 = 0x361E000;
pub const MPNPU_SRAM_X2I_MAILBOX_31: u32 = 0x363E000;
pub const MPNPU_SRAM_I2X_MAILBOX_31: u32 = 0x363F000;

pub const MMNPU_APERTURE0_BASE: u32 = 0x3000000;
pub const MMNPU_APERTURE1_BASE: u32 = 0x3600000;
pub const MMNPU_APERTURE3_BASE: u32 = 0x3810000;
pub const MMNPU_APERTURE4_BASE: u32 = 0x3B10000;

/* PCIe BAR Index for NPU6 */
pub const NPU6_REG_BAR_INDEX: u32 = 0;
pub const NPU6_MBOX_BAR_INDEX: u32 = 0;
pub const NPU6_PSP_BAR_INDEX: u32 = 4;
pub const NPU6_SMU_BAR_INDEX: u32 = 5;
pub const NPU6_SRAM_BAR_INDEX: u32 = 2;
/* Associated BARs and Apertures */
pub const NPU6_REG_BAR_BASE: u32 = MMNPU_APERTURE0_BASE;
pub const NPU6_MBOX_BAR_BASE: u32 = MMNPU_APERTURE0_BASE;
pub const NPU6_PSP_BAR_BASE: u32 = MMNPU_APERTURE3_BASE;
pub const NPU6_SMU_BAR_BASE: u32 = MMNPU_APERTURE4_BASE;
pub const NPU6_SRAM_BAR_BASE: u32 = MMNPU_APERTURE1_BASE;

static NPU6_DEV_PRIV: amdxdna_dev_priv = amdxdna_dev_priv {
    fw_path: "amdnpu/17f0_10/",
    rt_config: npu4_default_rt_cfg,
    dpm_clk_tbl: npu4_dpm_clk_table,
    col_align: COL_ALIGN_NATURE,
    col_opc: 4096,
    mbox_dev_addr: NPU6_MBOX_BAR_BASE,
    mbox_size: 0, /* Use BAR size */
    sram_dev_addr: NPU6_SRAM_BAR_BASE,
    hwctx_limit: 16,
    sram_offs: [
        DEFINE_BAR_OFFSET!(MBOX_CHANN_OFF, NPU6_SRAM, MPNPU_SRAM_X2I_MAILBOX_0),
        DEFINE_BAR_OFFSET!(FW_ALIVE_OFF, NPU6_SRAM, MPNPU_SRAM_X2I_MAILBOX_15),
    ],
    psp_regs_off: [
        DEFINE_BAR_OFFSET!(PSP_CMD_REG, NPU6_PSP, MP0_C2PMSG_123),
        DEFINE_BAR_OFFSET!(PSP_ARG0_REG, NPU6_REG, MPNPU_PUB_SCRATCH3),
        DEFINE_BAR_OFFSET!(PSP_ARG1_REG, NPU6_REG, MPNPU_PUB_SCRATCH4),
        DEFINE_BAR_OFFSET!(PSP_ARG2_REG, NPU6_REG, MPNPU_PUB_SCRATCH9),
        DEFINE_BAR_OFFSET!(PSP_INTR_REG, NPU6_PSP, MP0_C2PMSG_73),
        DEFINE_BAR_OFFSET!(PSP_STATUS_REG, NPU6_PSP, MP0_C2PMSG_123),
        DEFINE_BAR_OFFSET!(PSP_RESP_REG, NPU6_REG, MPNPU_PUB_SCRATCH3),
        DEFINE_BAR_OFFSET!(PSP_PWAITMODE_REG, NPU6_REG, MPNPU_PWAITMODE),
    ],
    smu_regs_off: [
        DEFINE_BAR_OFFSET!(SMU_CMD_REG, NPU6_SMU, MP1_C2PMSG_0),
        DEFINE_BAR_OFFSET!(SMU_ARG_REG, NPU6_SMU, MP1_C2PMSG_60),
        DEFINE_BAR_OFFSET!(SMU_INTR_REG, NPU6_SMU, MMNPU_APERTURE4_BASE),
        DEFINE_BAR_OFFSET!(SMU_RESP_REG, NPU6_SMU, MP1_C2PMSG_61),
        DEFINE_BAR_OFFSET!(SMU_OUT_REG, NPU6_SMU, MP1_C2PMSG_60),
    ],
    hw_ops: &npu4_hw_ops,
};

pub static DEV_NPU6_INFO: amdxdna_dev_info = amdxdna_dev_info {
    reg_bar: NPU6_REG_BAR_INDEX,
    mbox_bar: NPU6_MBOX_BAR_INDEX,
    sram_bar: NPU6_SRAM_BAR_INDEX,
    psp_bar: NPU6_PSP_BAR_INDEX,
    smu_bar: NPU6_SMU_BAR_INDEX,
    first_col: 0,
    dev_mem_buf_shift: 15, /* 32 KiB aligned */
    dev_mem_base: AIE2_DEVM_BASE,
    dev_mem_size: AIE2_DEVM_SIZE,
    default_vbnv: "RyzenAI-npu6",
    dev_heap_max_size: AIE2_DEVM_MAX_SIZE,
    device_type: AMDXDNA_DEV_TYPE_KMQ,
    rev_vbnv_tbl: npu4_rev_vbnv_tbl,
    dev_priv: &NPU6_DEV_PRIV,
    fw_feature_tbl: npu4_fw_feature_table,
    ops: &aie2_ops,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
