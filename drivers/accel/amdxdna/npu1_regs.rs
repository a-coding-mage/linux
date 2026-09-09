// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2023-2024, Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the surrounding driver.

/* Address definition from NPU1 docs */
const MPNPU_PWAITMODE: u32 = 0x3010034;
const MPNPU_PUB_SEC_INTR: u32 = 0x3010090;
const MPNPU_PUB_PWRMGMT_INTR: u32 = 0x3010094;
const MPNPU_PUB_SCRATCH2: u32 = 0x30100A0;
const MPNPU_PUB_SCRATCH3: u32 = 0x30100A4;
const MPNPU_PUB_SCRATCH4: u32 = 0x30100A8;
const MPNPU_PUB_SCRATCH5: u32 = 0x30100AC;
const MPNPU_PUB_SCRATCH6: u32 = 0x30100B0;
const MPNPU_PUB_SCRATCH7: u32 = 0x30100B4;
const MPNPU_PUB_SCRATCH9: u32 = 0x30100BC;

const MPNPU_SRAM_X2I_MAILBOX_0: u32 = 0x30A0000;
const MPNPU_SRAM_X2I_MAILBOX_1: u32 = 0x30A2000;
const MPNPU_SRAM_I2X_MAILBOX_15: u32 = 0x30BF000;

const MPNPU_APERTURE0_BASE: u32 = 0x3000000;
const MPNPU_APERTURE1_BASE: u32 = 0x3080000;
const MPNPU_APERTURE2_BASE: u32 = 0x30C0000;

/* PCIe BAR Index for NPU1 */
const NPU1_REG_BAR_INDEX: u32 = 0;
const NPU1_MBOX_BAR_INDEX: u32 = 4;
const NPU1_PSP_BAR_INDEX: u32 = 0;
const NPU1_SMU_BAR_INDEX: u32 = 0;
const NPU1_SRAM_BAR_INDEX: u32 = 2;
/* Associated BARs and Apertures */
const NPU1_REG_BAR_BASE: u32 = MPNPU_APERTURE0_BASE;
const NPU1_MBOX_BAR_BASE: u32 = MPNPU_APERTURE2_BASE;
const NPU1_PSP_BAR_BASE: u32 = MPNPU_APERTURE0_BASE;
const NPU1_SMU_BAR_BASE: u32 = MPNPU_APERTURE0_BASE;
const NPU1_SRAM_BAR_BASE: u32 = MPNPU_APERTURE1_BASE;

pub static npu1_default_rt_cfg: [rt_config; 4] = [
    rt_config { col: 2, row: 1, config: AIE2_RT_CFG_INIT },
    rt_config { col: 4, row: 1, config: AIE2_RT_CFG_INIT },
    rt_config { col: 1, row: 1, config: AIE2_RT_CFG_CLK_GATING },
    rt_config { col: 0, row: 0, config: 0 },
];

pub static npu1_dpm_clk_table: [dpm_clk_freq; 9] = [
    dpm_clk_freq { npuclk: 400, hclk: 800 },
    dpm_clk_freq { npuclk: 600, hclk: 1024 },
    dpm_clk_freq { npuclk: 600, hclk: 1024 },
    dpm_clk_freq { npuclk: 600, hclk: 1024 },
    dpm_clk_freq { npuclk: 600, hclk: 1024 },
    dpm_clk_freq { npuclk: 720, hclk: 1309 },
    dpm_clk_freq { npuclk: 720, hclk: 1309 },
    dpm_clk_freq { npuclk: 847, hclk: 1600 },
    dpm_clk_freq { npuclk: 0, hclk: 0 },
];

static npu1_fw_feature_table: [amdxdna_fw_feature_tbl; 3] = [
    amdxdna_fw_feature_tbl { major: 5, min_minor: 7, ..Default::default() },
    amdxdna_fw_feature_tbl {
        features: BIT_U64(AIE2_NPU_COMMAND), major: 5, min_minor: 8,
        ..Default::default()
    },
    amdxdna_fw_feature_tbl { ..Default::default() },
];

unsafe fn npu1_set_dpm(ndev: *mut amdxdna_dev_hdl, dpm_level: u32) -> i32 {
    let npuclk: u32;
    let hclk: u32;
    let ret: i32;

    npuclk = (*(*ndev).priv_).dpm_clk_tbl[dpm_level as usize].npuclk;
    hclk = (*(*ndev).priv_).dpm_clk_tbl[dpm_level as usize].hclk;
    ret = aie_smu_set_clocks((*ndev).aie.smu_hdl, &npuclk as *const u32 as *mut u32,
                             &hclk as *const u32 as *mut u32);
    if ret != 0 {
        return ret;
    }

    (*ndev).npuclk_freq = npuclk;
    (*ndev).hclk_freq = hclk;
    (*ndev).max_tops = 2 * (*ndev).total_col;
    (*ndev).curr_tops = (*ndev).max_tops * hclk / 1028;

    XDNA_DBG((*ndev).aie.xdna, "MP-NPU clock %d, H clock %d\n",
             (*ndev).npuclk_freq, (*ndev).hclk_freq);
    0
}

static npu1_dev_priv: amdxdna_dev_priv = amdxdna_dev_priv {
    fw_path: "amdnpu/1502_00/",
    rt_config: &npu1_default_rt_cfg,
    dpm_clk_tbl: &npu1_dpm_clk_table,
    col_align: COL_ALIGN_NONE,
    col_opc: 2048,
    mbox_dev_addr: NPU1_MBOX_BAR_BASE,
    mbox_size: 0,
    sram_dev_addr: NPU1_SRAM_BAR_BASE,
    hwctx_limit: 6,
    sram_offs: [
        DEFINE_BAR_OFFSET(MBOX_CHANN_OFF, NPU1_SRAM, MPNPU_SRAM_X2I_MAILBOX_0),
        DEFINE_BAR_OFFSET(FW_ALIVE_OFF, NPU1_SRAM, MPNPU_SRAM_I2X_MAILBOX_15),
    ],
    psp_regs_off: [
        DEFINE_BAR_OFFSET(PSP_CMD_REG, NPU1_PSP, MPNPU_PUB_SCRATCH2),
        DEFINE_BAR_OFFSET(PSP_ARG0_REG, NPU1_PSP, MPNPU_PUB_SCRATCH3),
        DEFINE_BAR_OFFSET(PSP_ARG1_REG, NPU1_PSP, MPNPU_PUB_SCRATCH4),
        DEFINE_BAR_OFFSET(PSP_ARG2_REG, NPU1_PSP, MPNPU_PUB_SCRATCH9),
        DEFINE_BAR_OFFSET(PSP_INTR_REG, NPU1_PSP, MPNPU_PUB_SEC_INTR),
        DEFINE_BAR_OFFSET(PSP_STATUS_REG, NPU1_PSP, MPNPU_PUB_SCRATCH2),
        DEFINE_BAR_OFFSET(PSP_RESP_REG, NPU1_PSP, MPNPU_PUB_SCRATCH3),
        DEFINE_BAR_OFFSET(PSP_PWAITMODE_REG, NPU1_PSP, MPNPU_PWAITMODE),
    ],
    smu_regs_off: [
        DEFINE_BAR_OFFSET(SMU_CMD_REG, NPU1_SMU, MPNPU_PUB_SCRATCH5),
        DEFINE_BAR_OFFSET(SMU_ARG_REG, NPU1_SMU, MPNPU_PUB_SCRATCH7),
        DEFINE_BAR_OFFSET(SMU_INTR_REG, NPU1_SMU, MPNPU_PUB_PWRMGMT_INTR),
        DEFINE_BAR_OFFSET(SMU_RESP_REG, NPU1_SMU, MPNPU_PUB_SCRATCH6),
        DEFINE_BAR_OFFSET(SMU_OUT_REG, NPU1_SMU, MPNPU_PUB_SCRATCH7),
    ],
    hw_ops: &aie2_hw_ops { set_dpm: npu1_set_dpm },
};

pub static dev_npu1_info: amdxdna_dev_info = amdxdna_dev_info {
    reg_bar: NPU1_REG_BAR_INDEX,
    mbox_bar: NPU1_MBOX_BAR_INDEX,
    sram_bar: NPU1_SRAM_BAR_INDEX,
    psp_bar: NPU1_PSP_BAR_INDEX,
    smu_bar: NPU1_SMU_BAR_INDEX,
    first_col: 1,
    dev_mem_buf_shift: 15,
    dev_mem_base: AIE2_DEVM_BASE,
    dev_mem_size: AIE2_DEVM_SIZE,
    default_vbnv: "RyzenAI-npu1",
    dev_heap_max_size: AIE2_DEVM_SIZE,
    device_type: AMDXDNA_DEV_TYPE_KMQ,
    dev_priv: &npu1_dev_priv,
    fw_feature_tbl: &npu1_fw_feature_table,
    ops: &aie2_ops,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
