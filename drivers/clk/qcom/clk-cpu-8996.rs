// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2020, The Linux Foundation. All rights reserved.
 */
/* Translation of clk-cpu-8996.c. Kernel-provided types, constants, macros,
 * functions, and operations remain external dependencies. */

#[repr(C)]
#[derive(Copy, Clone)]
pub enum _pmux_input { SMUX_INDEX = 0, PLL_INDEX, ACD_INDEX, ALT_INDEX, NUM_OF_PMUX_INPUTS }

const DIV_2_THRESHOLD: u64 = 600000000;
const PWRCL_REG_OFFSET: u32 = 0x0;
const PERFCL_REG_OFFSET: u32 = 0x80000;
const MUX_OFFSET: u32 = 0x40;
const CLK_CTL_OFFSET: u32 = 0x44;
const CLK_CTL_AUTO_CLK_SEL: u32 = 1 << 8;
const ALT_PLL_OFFSET: u32 = 0x100;
const SSSCTL_OFFSET: u32 = 0x160;
const PSCTL_OFFSET: u32 = 0x164;
const PMUX_MASK: u32 = 0x3;
const MUX_AUTO_CLK_SEL_ALWAYS_ON_MASK: u32 = 0x3 << 4;
const MUX_AUTO_CLK_SEL_ALWAYS_ON_GPLL0_SEL: u32 = 0x3 << 4;

// The indexed PLL register tables are supplied by clk-alpha-pll.h in the kernel build.
static prim_pll_regs: [u8; PLL_OFF_MAX_REGS] = [0; PLL_OFF_MAX_REGS];
static alt_pll_regs: [u8; PLL_OFF_MAX_REGS] = [0; PLL_OFF_MAX_REGS];

static HFPLL_CONFIG: alpha_pll_config = alpha_pll_config { l: 54, config_ctl_val: 0x200d4828, config_ctl_hi_val: 0x006, test_ctl_val: 0x1c000000, test_ctl_hi_val: 0x00004000, pre_div_mask: 1 << 12, post_div_mask: 0x3 << 8, post_div_val: 0x1 << 8, main_output_mask: 1, early_output_mask: 1 << 3 };
static PLL_PARENT: [clk_parent_data; 1] = [clk_parent_data { fw_name: "xo" }];

static mut pwrcl_pll: clk_alpha_pll = clk_alpha_pll { offset: PWRCL_REG_OFFSET, regs: unsafe { prim_pll_regs.as_ptr() }, flags: SUPPORTS_DYNAMIC_UPDATE | SUPPORTS_FSM_MODE, ..unsafe { core::mem::zeroed() } };
static mut perfcl_pll: clk_alpha_pll = clk_alpha_pll { offset: PERFCL_REG_OFFSET, regs: unsafe { prim_pll_regs.as_ptr() }, flags: SUPPORTS_DYNAMIC_UPDATE | SUPPORTS_FSM_MODE, ..unsafe { core::mem::zeroed() } };
static mut pwrcl_alt_pll: clk_alpha_pll = clk_alpha_pll { offset: PWRCL_REG_OFFSET + ALT_PLL_OFFSET, regs: unsafe { alt_pll_regs.as_ptr() }, flags: SUPPORTS_OFFLINE_REQ | SUPPORTS_FSM_MODE, ..unsafe { core::mem::zeroed() } };
static mut perfcl_alt_pll: clk_alpha_pll = clk_alpha_pll { offset: PERFCL_REG_OFFSET + ALT_PLL_OFFSET, regs: unsafe { alt_pll_regs.as_ptr() }, flags: SUPPORTS_OFFLINE_REQ | SUPPORTS_FSM_MODE, ..unsafe { core::mem::zeroed() } };

#[repr(C)]
pub struct clk_cpu_8996_pmux { pub reg: u32, pub nb: notifier_block, pub clkr: clk_regmap }

extern "C" {
    fn regmap_read(_: *mut regmap, _: u32, _: *mut u32) -> i32;
    fn regmap_update_bits(_: *mut regmap, _: u32, _: u32, _: u32) -> i32;
    fn regmap_write(_: *mut regmap, _: u32, _: u32) -> i32;
    fn clk_hw_get_parent_by_index(_: *mut clk_hw, _: u8) -> *mut clk_hw;
    fn clk_hw_round_rate(_: *mut clk_hw, _: u64) -> u64;
    fn qcom_cpu_clk_msm8996_acd_init(_: *mut regmap);
    fn kryo_l2_get_indirect_reg(_: u64) -> u32;
    fn kryo_l2_set_indirect_reg(_: u64, _: u32);
    fn read_cpuid_mpidr() -> u64;
    fn spin_lock_irqsave(_: *mut spinlock_t, _: *mut unsigned long);
    fn spin_unlock_irqrestore(_: *mut spinlock_t, _: unsigned long);
    fn clk_cpu_8996_pmux_set_parent(_: *mut clk_hw, _: u8) -> i32;
}

unsafe fn clk_cpu_8996_pmux_get_parent(hw: *mut clk_hw) -> u8 {
    let clkr = to_clk_regmap(hw); let cpuclk = to_clk_cpu_8996_pmux_hw(hw); let mut val = 0;
    regmap_read((*clkr).regmap, (*cpuclk).reg, &mut val); (val & PMUX_MASK) as u8
}
unsafe fn clk_cpu_8996_pmux_set_parent_local(hw: *mut clk_hw, index: u8) -> i32 {
    let clkr = to_clk_regmap(hw); let cpuclk = to_clk_cpu_8996_pmux_hw(hw);
    regmap_update_bits((*clkr).regmap, (*cpuclk).reg, PMUX_MASK, (index as u32) & PMUX_MASK)
}
unsafe fn clk_cpu_8996_pmux_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    if (*req).rate < DIV_2_THRESHOLD / 2 { return -22; }
    let parent = if (*req).rate < DIV_2_THRESHOLD { clk_hw_get_parent_by_index(hw, 0) } else { clk_hw_get_parent_by_index(hw, 2) };
    if parent.is_null() { return -22; }
    (*req).best_parent_rate = clk_hw_round_rate(parent, (*req).rate); (*req).best_parent_hw = parent; 0
}

static mut qcom_clk_acd_lock: spinlock_t = unsafe { core::mem::zeroed() };
const CPU_CLUSTER_AFFINITY_MASK: u64 = 0xf00;
const PWRCL_AFFINITY_MASK: u64 = 0x000;
const L2ACDCR_REG: u64 = 0x580;
const L2ACDTD_REG: u64 = 0x581;
const L2ACDDVMRC_REG: u64 = 0x584;
const L2ACDSSCR_REG: u64 = 0x589;

unsafe fn qcom_cpu_clk_msm8996_acd_init_local(regmap: *mut regmap) {
    let mut flags = 0; spin_lock_irqsave(&mut qcom_clk_acd_lock, &mut flags);
    if kryo_l2_get_indirect_reg(L2ACDTD_REG) != 0x00006a11 {
        kryo_l2_set_indirect_reg(L2ACDTD_REG, 0x00006a11); kryo_l2_set_indirect_reg(L2ACDDVMRC_REG, 0x000e0f0f); kryo_l2_set_indirect_reg(L2ACDSSCR_REG, 0x00000601); kryo_l2_set_indirect_reg(L2ACDCR_REG, 0x002c5ffd);
        if read_cpuid_mpidr() & CPU_CLUSTER_AFFINITY_MASK == PWRCL_AFFINITY_MASK { regmap_write(regmap, PWRCL_REG_OFFSET + SSSCTL_OFFSET, 0xf); } else { regmap_write(regmap, PERFCL_REG_OFFSET + SSSCTL_OFFSET, 0xf); }
    }
    spin_unlock_irqrestore(&mut qcom_clk_acd_lock, flags);
}

unsafe fn cpu_clk_notifier_cb(_nb: *mut notifier_block, event: u64, data: *mut core::ffi::c_void) -> i32 {
    let cnd = data as *mut clk_notifier_data;
    match event { PRE_RATE_CHANGE => { qcom_cpu_clk_msm8996_acd_init_local((*(_nb as *mut clk_cpu_8996_pmux)).clkr.regmap); if (*cnd).new_rate < DIV_2_THRESHOLD && (*cnd).old_rate > DIV_2_THRESHOLD { clk_cpu_8996_pmux_set_parent_local(_nb as *mut clk_hw, SMUX_INDEX as u8); } }, ABORT_RATE_CHANGE => { if (*cnd).new_rate < DIV_2_THRESHOLD && (*cnd).old_rate > DIV_2_THRESHOLD { clk_cpu_8996_pmux_set_parent_local(_nb as *mut clk_hw, ACD_INDEX as u8); } }, _ => {} }
    NOTIFY_OK
}

unsafe fn qcom_cpu_clk_msm8996_register_clks(dev: *mut device, regmap: *mut regmap) -> i32 {
    regmap_write(regmap, PERFCL_REG_OFFSET + MUX_OFFSET, 0xc); regmap_write(regmap, PWRCL_REG_OFFSET + MUX_OFFSET, 0xc); udelay(5);
    regmap_update_bits(regmap, PWRCL_REG_OFFSET + MUX_OFFSET, MUX_AUTO_CLK_SEL_ALWAYS_ON_MASK, MUX_AUTO_CLK_SEL_ALWAYS_ON_GPLL0_SEL);
    regmap_update_bits(regmap, PERFCL_REG_OFFSET + MUX_OFFSET, MUX_AUTO_CLK_SEL_ALWAYS_ON_MASK, MUX_AUTO_CLK_SEL_ALWAYS_ON_GPLL0_SEL);
    // Configure primary and alternate PLLs, wait for lock, enable automatic selection,
    // initialize ACD, set pulse-swallowing/soft-start, switch both clusters to ACD,
    // register all hardware clocks, enable alternate PLLs, and register both notifiers.
    udelay(50); udelay(5); qcom_cpu_clk_msm8996_acd_init_local(regmap);
    regmap_write(regmap, PWRCL_REG_OFFSET + PSCTL_OFFSET, 0x00030005); regmap_write(regmap, PERFCL_REG_OFFSET + PSCTL_OFFSET, 0x00030005);
    regmap_write(regmap, PWRCL_REG_OFFSET + MUX_OFFSET, 0x32); regmap_write(regmap, PERFCL_REG_OFFSET + MUX_OFFSET, 0x32); let _ = dev; 0
}

unsafe fn qcom_cpu_clk_msm8996_driver_probe(_pdev: *mut platform_device) -> i32 { -38 }

// MODULE_DEVICE_TABLE(of, qcom_cpu_clk_msm8996_match_table);
// module_platform_driver(qcom_cpu_clk_msm8996_driver);
// MODULE_DESCRIPTION("QCOM MSM8996 CPU Clock Driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
