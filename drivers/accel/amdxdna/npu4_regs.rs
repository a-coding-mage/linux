// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2023-2024, Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the surrounding kernel translation unit.

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

pub const NPU4_REG_BAR_INDEX: u32 = 0;
pub const NPU4_MBOX_BAR_INDEX: u32 = 0;
pub const NPU4_PSP_BAR_INDEX: u32 = 4;
pub const NPU4_SMU_BAR_INDEX: u32 = 5;
pub const NPU4_SRAM_BAR_INDEX: u32 = 2;
pub const NPU4_REG_BAR_BASE: u32 = MMNPU_APERTURE0_BASE;
pub const NPU4_MBOX_BAR_BASE: u32 = MMNPU_APERTURE0_BASE;
pub const NPU4_PSP_BAR_BASE: u32 = MMNPU_APERTURE3_BASE;
pub const NPU4_SMU_BAR_BASE: u32 = MMNPU_APERTURE4_BASE;
pub const NPU4_SRAM_BAR_BASE: u32 = MMNPU_APERTURE1_BASE;

#[inline]
pub fn npu4_dpm_tops(ndev: &amdxdna_dev_hdl, hclk: u32) -> u32 {
    4096u32.wrapping_mul(ndev.total_col).wrapping_mul(hclk) / 1_000_000
}

pub static npu4_default_rt_cfg: [rt_config; 10] = [
    rt_config { a: 5, b: 1, c: AIE2_RT_CFG_INIT, d: 0 },
    rt_config { a: 10, b: 1, c: AIE2_RT_CFG_INIT, d: 0 },
    rt_config { a: 14, b: 0, c: AIE2_RT_CFG_INIT, d: BIT_U64(AIE2_PREEMPT) },
    rt_config { a: 1, b: 1, c: AIE2_RT_CFG_CLK_GATING, d: 0 },
    rt_config { a: 2, b: 1, c: AIE2_RT_CFG_CLK_GATING, d: 0 },
    rt_config { a: 3, b: 1, c: AIE2_RT_CFG_CLK_GATING, d: 0 },
    rt_config { a: 4, b: 1, c: AIE2_RT_CFG_CLK_GATING, d: 0 },
    rt_config { a: 13, b: 0, c: AIE2_RT_CFG_FORCE_PREEMPT, d: 0 },
    rt_config { a: 14, b: 0, c: AIE2_RT_CFG_FRAME_BOUNDARY_PREEMPT, d: 0 },
    rt_config { a: 0, b: 0, c: 0, d: 0 },
];

pub static npu4_dpm_clk_table: [dpm_clk_freq; 9] = [
    dpm_clk_freq { npuclk: 396, hclk: 792 },
    dpm_clk_freq { npuclk: 600, hclk: 1056 },
    dpm_clk_freq { npuclk: 792, hclk: 1152 },
    dpm_clk_freq { npuclk: 975, hclk: 1267 },
    dpm_clk_freq { npuclk: 975, hclk: 1267 },
    dpm_clk_freq { npuclk: 1056, hclk: 1408 },
    dpm_clk_freq { npuclk: 1152, hclk: 1584 },
    dpm_clk_freq { npuclk: 1267, hclk: 1800 },
    dpm_clk_freq { npuclk: 0, hclk: 0 },
];

pub static npu4_fw_feature_table: [amdxdna_fw_feature_tbl; 10] = [
    amdxdna_fw_feature_tbl { features: 0, major: 6, min_minor: 12 },
    amdxdna_fw_feature_tbl { features: BIT_U64(AIE2_PREEMPT), major: 6, min_minor: 12 },
    amdxdna_fw_feature_tbl { features: BIT_U64(AIE2_TEMPORAL_ONLY), major: 6, min_minor: 12 },
    amdxdna_fw_feature_tbl { features: BIT_U64(AIE2_NPU_COMMAND), major: 6, min_minor: 15 },
    amdxdna_fw_feature_tbl { features: BIT_U64(AIE2_UPDATE_PROPERTY), major: 6, min_minor: 15 },
    amdxdna_fw_feature_tbl { features: BIT_U64(AIE2_APP_HEALTH), major: 6, min_minor: 18 },
    amdxdna_fw_feature_tbl { features: BIT_U64(AIE2_ADD_HOST_BUFFER), major: 6, min_minor: 18 },
    amdxdna_fw_feature_tbl { features: BIT_U64(AIE2_GET_DEV_REVISION), major: 6, min_minor: 24 },
    amdxdna_fw_feature_tbl { features: AIE2_ALL_FEATURES, major: 7, min_minor: 0 },
    amdxdna_fw_feature_tbl { features: 0, major: 0, min_minor: 0 },
];

unsafe fn npu4_set_dpm(ndev: *mut amdxdna_dev_hdl, dpm_level: u32) -> i32 {
    let ndev = &mut *ndev;
    let ret = aie_smu_set_dpm(ndev.aie.smu_hdl, dpm_level);
    if ret != 0 { return ret; }
    ndev.npuclk_freq = ndev.priv_.dpm_clk_tbl[dpm_level as usize].npuclk;
    ndev.hclk_freq = ndev.priv_.dpm_clk_tbl[dpm_level as usize].hclk;
    ndev.max_tops = npu4_dpm_tops(ndev, ndev.priv_.dpm_clk_tbl[ndev.max_dpm_level as usize].hclk);
    ndev.curr_tops = npu4_dpm_tops(ndev, ndev.hclk_freq);
    XDNA_DBG(ndev.aie.xdna, "MP-NPU clock %d, H clock %d\n", ndev.npuclk_freq, ndev.hclk_freq);
    0
}

unsafe fn npu4_update_counters(ndev: *mut amdxdna_dev_hdl) -> i32 {
    let ndev = &mut *ndev;
    let mut npu_metrics = amd_pmf_npu_metrics::default();
    let ret = AIE2_GET_PMF_NPU_METRICS(&mut npu_metrics);
    if ret != 0 { return ret; }
    ndev.npuclk_freq = npu_metrics.mpnpuclk_freq;
    ndev.hclk_freq = npu_metrics.npuclk_freq;
    ndev.curr_tops = npu4_dpm_tops(ndev, ndev.hclk_freq);
    0
}

pub static npu4_hw_ops: aie2_hw_ops = aie2_hw_ops {
    set_dpm: Some(npu4_set_dpm),
    update_counters: Some(npu4_update_counters),
};

pub static npu4_rev_vbnv_tbl: [amdxdna_rev_vbnv; 9] = [
    amdxdna_rev_vbnv { rev: AIE2_DEV_REVISION_STXA, vbnv: "NPU Strix\0".as_ptr() as *const i8 },
    amdxdna_rev_vbnv { rev: AIE2_DEV_REVISION_STXB, vbnv: "NPU Strix\0".as_ptr() as *const i8 },
    amdxdna_rev_vbnv { rev: AIE2_DEV_REVISION_KRK1, vbnv: "NPU Krackan 1\0".as_ptr() as *const i8 },
    amdxdna_rev_vbnv { rev: AIE2_DEV_REVISION_KRK2, vbnv: "NPU Krackan 2\0".as_ptr() as *const i8 },
    amdxdna_rev_vbnv { rev: AIE2_DEV_REVISION_HALO, vbnv: "NPU Strix Halo\0".as_ptr() as *const i8 },
    amdxdna_rev_vbnv { rev: AIE2_DEV_REVISION_GPT1, vbnv: "NPU Gorgon Point 1\0".as_ptr() as *const i8 },
    amdxdna_rev_vbnv { rev: AIE2_DEV_REVISION_GPT2, vbnv: "NPU Gorgon Point 2\0".as_ptr() as *const i8 },
    amdxdna_rev_vbnv { rev: AIE2_DEV_REVISION_GPT3, vbnv: "NPU Gorgon Point 3\0".as_ptr() as *const i8 },
    amdxdna_rev_vbnv { rev: 0, vbnv: core::ptr::null() },
];

// The remaining device-private aggregate mirrors the C designated initializer.
pub static npu4_dev_priv: amdxdna_dev_priv = amdxdna_dev_priv {
    fw_path: "amdnpu/17f0_10/\0".as_ptr() as *const i8,
    rt_config: npu4_default_rt_cfg.as_ptr(), dpm_clk_tbl: npu4_dpm_clk_table.as_ptr(),
    col_align: COL_ALIGN_NATURE, col_opc: 4096, mbox_dev_addr: NPU4_MBOX_BAR_BASE,
    mbox_size: 0, sram_dev_addr: NPU4_SRAM_BAR_BASE, hwctx_limit: 16,
    sram_offs: [DEFINE_BAR_OFFSET(MBOX_CHANN_OFF, NPU4_SRAM, MPNPU_SRAM_X2I_MAILBOX_0), DEFINE_BAR_OFFSET(FW_ALIVE_OFF, NPU4_SRAM, MPNPU_SRAM_X2I_MAILBOX_15)],
    psp_regs_off: [DEFINE_BAR_OFFSET(PSP_CMD_REG, NPU4_PSP, MP0_C2PMSG_123), DEFINE_BAR_OFFSET(PSP_ARG0_REG, NPU4_REG, MPNPU_PUB_SCRATCH3), DEFINE_BAR_OFFSET(PSP_ARG1_REG, NPU4_REG, MPNPU_PUB_SCRATCH4), DEFINE_BAR_OFFSET(PSP_ARG2_REG, NPU4_REG, MPNPU_PUB_SCRATCH9), DEFINE_BAR_OFFSET(PSP_INTR_REG, NPU4_PSP, MP0_C2PMSG_73), DEFINE_BAR_OFFSET(PSP_STATUS_REG, NPU4_PSP, MP0_C2PMSG_123), DEFINE_BAR_OFFSET(PSP_RESP_REG, NPU4_REG, MPNPU_PUB_SCRATCH3), DEFINE_BAR_OFFSET(PSP_PWAITMODE_REG, NPU4_REG, MPNPU_PWAITMODE)],
    smu_regs_off: [DEFINE_BAR_OFFSET(SMU_CMD_REG, NPU4_SMU, MP1_C2PMSG_0), DEFINE_BAR_OFFSET(SMU_ARG_REG, NPU4_SMU, MP1_C2PMSG_60), DEFINE_BAR_OFFSET(SMU_INTR_REG, NPU4_SMU, MMNPU_APERTURE4_BASE), DEFINE_BAR_OFFSET(SMU_RESP_REG, NPU4_SMU, MP1_C2PMSG_61), DEFINE_BAR_OFFSET(SMU_OUT_REG, NPU4_SMU, MP1_C2PMSG_60)],
    hw_ops: &npu4_hw_ops,
};

pub static dev_npu4_info: amdxdna_dev_info = amdxdna_dev_info {
    reg_bar: NPU4_REG_BAR_INDEX, mbox_bar: NPU4_MBOX_BAR_INDEX, sram_bar: NPU4_SRAM_BAR_INDEX,
    psp_bar: NPU4_PSP_BAR_INDEX, smu_bar: NPU4_SMU_BAR_INDEX, first_col: 0,
    dev_mem_buf_shift: 15, dev_mem_base: AIE2_DEVM_BASE, dev_mem_size: AIE2_DEVM_SIZE,
    default_vbnv: "RyzenAI-npu4\0".as_ptr() as *const i8, dev_heap_max_size: AIE2_DEVM_MAX_SIZE,
    device_type: AMDXDNA_DEV_TYPE_KMQ, rev_vbnv_tbl: npu4_rev_vbnv_tbl.as_ptr(),
    dev_priv: &npu4_dev_priv, fw_feature_tbl: npu4_fw_feature_table.as_ptr(), ops: &aie2_ops,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
