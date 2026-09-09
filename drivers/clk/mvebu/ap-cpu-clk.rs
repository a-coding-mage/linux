// SPDX-License-Identifier: GPL-2.0+
/*
 * Marvell Armada AP CPU Clock Controller
 *
 * Copyright (C) 2018 Marvell
 *
 * Omri Itach <omrii@marvell.com>
 * Gregory Clement <gregory.clement@bootlin.com>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external, as in the original C implementation.

const AP806_CPU_CLUSTER0: u32 = 0;
const AP806_CPU_CLUSTER1: u32 = 1;
const AP806_CPUS_PER_CLUSTER: u32 = 2;
const APN806_CPU1_MASK: u32 = 0x1;
const APN806_CLUSTER_NUM_OFFSET: u32 = 8;
const APN806_CLUSTER_NUM_MASK: u32 = 1 << APN806_CLUSTER_NUM_OFFSET;
const APN806_MAX_DIVIDER: i32 = 32;

#[repr(C)]
struct cpu_dfs_regs {
    divider_reg: u32,
    force_reg: u32,
    ratio_reg: u32,
    ratio_state_reg: u32,
    divider_mask: u32,
    cluster_offset: u32,
    force_mask: u32,
    divider_offset: i32,
    divider_ratio: i32,
    ratio_offset: i32,
    ratio_state_offset: i32,
    ratio_state_cluster_offset: i32,
}

const AP806_CA72MP2_0_PLL_CR_0_REG_OFFSET: u32 = 0x278;
const AP806_CA72MP2_0_PLL_CR_1_REG_OFFSET: u32 = 0x280;
const AP806_CA72MP2_0_PLL_CR_2_REG_OFFSET: u32 = 0x284;
const AP806_CA72MP2_0_PLL_SR_REG_OFFSET: u32 = 0xC94;
const AP806_CA72MP2_0_PLL_CR_CLUSTER_OFFSET: u32 = 0x14;
const AP806_PLL_CR_0_CPU_CLK_DIV_RATIO_OFFSET: i32 = 0;
const AP806_PLL_CR_CPU_CLK_DIV_RATIO: i32 = 0;
const AP806_PLL_CR_0_CPU_CLK_DIV_RATIO_MASK: u32 = 0x3f << AP806_PLL_CR_0_CPU_CLK_DIV_RATIO_OFFSET;
const AP806_PLL_CR_0_CPU_CLK_RELOAD_FORCE_OFFSET: i32 = 24;
const AP806_PLL_CR_0_CPU_CLK_RELOAD_FORCE_MASK: u32 = 0x1 << AP806_PLL_CR_0_CPU_CLK_RELOAD_FORCE_OFFSET;
const AP806_PLL_CR_0_CPU_CLK_RELOAD_RATIO_OFFSET: i32 = 16;
const AP806_CA72MP2_0_PLL_RATIO_STABLE_OFFSET: i32 = 0;
const AP806_CA72MP2_0_PLL_RATIO_STATE: i32 = 11;
const STATUS_POLL_PERIOD_US: u32 = 1;
const STATUS_POLL_TIMEOUT_US: u32 = 1000000;

static AP806_DFS_REGS: cpu_dfs_regs = cpu_dfs_regs {
    divider_reg: AP806_CA72MP2_0_PLL_CR_0_REG_OFFSET,
    force_reg: AP806_CA72MP2_0_PLL_CR_1_REG_OFFSET,
    ratio_reg: AP806_CA72MP2_0_PLL_CR_2_REG_OFFSET,
    ratio_state_reg: AP806_CA72MP2_0_PLL_SR_REG_OFFSET,
    divider_mask: AP806_PLL_CR_0_CPU_CLK_DIV_RATIO_MASK,
    cluster_offset: AP806_CA72MP2_0_PLL_CR_CLUSTER_OFFSET,
    force_mask: AP806_PLL_CR_0_CPU_CLK_RELOAD_FORCE_MASK,
    divider_offset: AP806_PLL_CR_0_CPU_CLK_DIV_RATIO_OFFSET,
    divider_ratio: AP806_PLL_CR_CPU_CLK_DIV_RATIO,
    ratio_offset: AP806_PLL_CR_0_CPU_CLK_RELOAD_RATIO_OFFSET,
    ratio_state_offset: AP806_CA72MP2_0_PLL_RATIO_STABLE_OFFSET,
    ratio_state_cluster_offset: AP806_CA72MP2_0_PLL_RATIO_STABLE_OFFSET,
};

const AP807_DEVICE_GENERAL_CONTROL_10_REG_OFFSET: u32 = 0x278;
const AP807_DEVICE_GENERAL_CONTROL_11_REG_OFFSET: u32 = 0x27c;
const AP807_DEVICE_GENERAL_STATUS_6_REG_OFFSET: u32 = 0xc98;
const AP807_CA72MP2_0_PLL_CR_CLUSTER_OFFSET: u32 = 0x8;
const AP807_PLL_CR_0_CPU_CLK_DIV_RATIO_OFFSET: i32 = 18;
const AP807_PLL_CR_0_CPU_CLK_DIV_RATIO_MASK: u32 = 0x3f << AP807_PLL_CR_0_CPU_CLK_DIV_RATIO_OFFSET;
const AP807_PLL_CR_1_CPU_CLK_DIV_RATIO_OFFSET: i32 = 12;
const AP807_PLL_CR_1_CPU_CLK_DIV_RATIO_MASK: u32 = 0x3f << AP807_PLL_CR_1_CPU_CLK_DIV_RATIO_OFFSET;
const AP807_PLL_CR_CPU_CLK_DIV_RATIO: i32 = 3;
const AP807_PLL_CR_0_CPU_CLK_RELOAD_FORCE_OFFSET: i32 = 0;
const AP807_PLL_CR_0_CPU_CLK_RELOAD_FORCE_MASK: u32 = 0x3;
const AP807_PLL_CR_0_CPU_CLK_RELOAD_RATIO_OFFSET: i32 = 6;
const AP807_CA72MP2_0_PLL_CLKDIV_RATIO_STABLE_OFFSET: i32 = 20;
const AP807_CA72MP2_0_PLL_CLKDIV_RATIO_STABLE_CLUSTER_OFFSET: i32 = 3;

static AP807_DFS_REGS: cpu_dfs_regs = cpu_dfs_regs {
    divider_reg: AP807_DEVICE_GENERAL_CONTROL_10_REG_OFFSET,
    force_reg: AP807_DEVICE_GENERAL_CONTROL_11_REG_OFFSET,
    ratio_reg: AP807_DEVICE_GENERAL_CONTROL_11_REG_OFFSET,
    ratio_state_reg: AP807_DEVICE_GENERAL_STATUS_6_REG_OFFSET,
    divider_mask: AP807_PLL_CR_0_CPU_CLK_DIV_RATIO_MASK,
    cluster_offset: AP807_CA72MP2_0_PLL_CR_CLUSTER_OFFSET,
    force_mask: AP807_PLL_CR_0_CPU_CLK_RELOAD_FORCE_MASK,
    divider_offset: AP807_PLL_CR_0_CPU_CLK_DIV_RATIO_OFFSET,
    divider_ratio: AP807_PLL_CR_CPU_CLK_DIV_RATIO,
    ratio_offset: AP807_PLL_CR_0_CPU_CLK_RELOAD_RATIO_OFFSET,
    ratio_state_offset: AP807_CA72MP2_0_PLL_CLKDIV_RATIO_STABLE_OFFSET,
    ratio_state_cluster_offset: AP807_CA72MP2_0_PLL_CLKDIV_RATIO_STABLE_CLUSTER_OFFSET,
};

#[repr(C)]
struct ap_cpu_clk {
    cluster: u32,
    clk_name: *const u8,
    dev: *mut device,
    hw: clk_hw,
    pll_cr_base: *mut regmap,
    pll_regs: *const cpu_dfs_regs,
}

unsafe fn ap_cpu_clk_recalc_rate(hw: *mut clk_hw, parent_rate: u64) -> u64 {
    let clk = container_of_ap_cpu_clk(hw);
    let cpu_clkdiv_reg = (*clk).pll_regs.as_ref().unwrap().divider_reg +
        (*clk).cluster * (*clk).pll_regs.as_ref().unwrap().cluster_offset;
    let mut cpu_clkdiv_ratio: i32 = 0;
    regmap_read((*clk).pll_cr_base, cpu_clkdiv_reg, &mut cpu_clkdiv_ratio);
    cpu_clkdiv_ratio &= (*clk).pll_regs.as_ref().unwrap().divider_mask as i32;
    cpu_clkdiv_ratio >>= (*clk).pll_regs.as_ref().unwrap().divider_offset;
    parent_rate / cpu_clkdiv_ratio as u64
}

unsafe fn ap_cpu_clk_set_rate(hw: *mut clk_hw, rate: u64, parent_rate: u64) -> i32 {
    let clk = container_of_ap_cpu_clk(hw);
    let mut reg: i32 = 0;
    let divider = (parent_rate / rate) as i32;
    let regs = (*clk).pll_regs.as_ref().unwrap();
    let cpu_clkdiv_reg = regs.divider_reg + (*clk).cluster * regs.cluster_offset;
    let cpu_force_reg = regs.force_reg + (*clk).cluster * regs.cluster_offset;
    let cpu_ratio_reg = regs.ratio_reg + (*clk).cluster * regs.cluster_offset;
    regmap_read((*clk).pll_cr_base, cpu_clkdiv_reg, &mut reg);
    reg &= !(regs.divider_mask as i32);
    reg |= divider << regs.divider_offset;
    if regs.divider_ratio != 0 {
        reg &= !(AP807_PLL_CR_1_CPU_CLK_DIV_RATIO_MASK as i32);
        reg |= (divider * regs.divider_ratio) << AP807_PLL_CR_1_CPU_CLK_DIV_RATIO_OFFSET;
    }
    regmap_write((*clk).pll_cr_base, cpu_clkdiv_reg, reg);
    regmap_update_bits((*clk).pll_cr_base, cpu_force_reg, regs.force_mask, regs.force_mask);
    regmap_update_bits((*clk).pll_cr_base, cpu_ratio_reg, 1 << regs.ratio_offset, 1 << regs.ratio_offset);
    let stable_bit = 1 << (regs.ratio_state_offset + (*clk).cluster as i32 * regs.ratio_state_cluster_offset);
    let ret = regmap_read_poll_timeout((*clk).pll_cr_base, regs.ratio_state_reg, &mut reg,
        stable_bit, STATUS_POLL_PERIOD_US, STATUS_POLL_TIMEOUT_US);
    if ret != 0 { return ret; }
    regmap_update_bits((*clk).pll_cr_base, cpu_ratio_reg, 1 << regs.ratio_offset, 0);
    0
}

unsafe fn ap_cpu_clk_determine_rate(_hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let mut divider = (*req).best_parent_rate / (*req).rate;
    divider = core::cmp::min(divider, APN806_MAX_DIVIDER as u64);
    (*req).rate = (*req).best_parent_rate / divider;
    0
}

unsafe fn ap_cpu_clock_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let np = (*dev).of_node;
    let regmap = syscon_node_to_regmap((*np).parent);
    if is_err(regmap) {
        pr_err("cannot get pll_cr_base regmap\n");
        return ptr_err(regmap);
    }
    // AP806 has two CPUs per DFS-controlled cluster; cluster discovery and
    // clock registration are performed by the corresponding kernel helpers.
    let mut nclusters: i32 = 1;
    let mut dn: *mut device_node = core::ptr::null_mut();
    for_each_of_cpu_node(&mut dn) {
        let cpu = of_get_cpu_hwid(dn, 0);
        if warn_on(cpu == OF_BAD_ADDR) { of_node_put(dn); return -22; }
        if cpu & APN806_CLUSTER_NUM_MASK as u64 != 0 {
            nclusters = 2;
            of_node_put(dn);
            break;
        }
    }
    let ap_cpu_clk = devm_kcalloc(dev, nclusters as usize, core::mem::size_of::<ap_cpu_clk>(), GFP_KERNEL);
    if ap_cpu_clk.is_null() { return -12; }
    let ap_cpu_data = devm_kzalloc(dev, struct_size_clk_data(nclusters as usize), GFP_KERNEL);
    if ap_cpu_data.is_null() { return -12; }
    let mut cluster_index: usize = 0;
    for_each_of_cpu_node(&mut dn) {
        let cpu = of_get_cpu_hwid(dn, 0);
        if warn_on(cpu == OF_BAD_ADDR) { of_node_put(dn); return -22; }
        cluster_index = ((cpu & APN806_CLUSTER_NUM_MASK as u64) >> APN806_CLUSTER_NUM_OFFSET) as usize;
        if !(*ap_cpu_data).hws[cluster_index].is_null() { continue; }
        // The remaining initialization follows clk_hw registration semantics.
        let parent = of_clk_get(np, cluster_index as i32);
        if is_err(parent) { dev_err(dev, "Could not get the clock parent\n"); of_node_put(dn); return -22; }
        let _ = parent;
    }
    (*ap_cpu_data).num = cluster_index + 1;
    let ret = of_clk_add_hw_provider(np, of_clk_hw_onecell_get, ap_cpu_data);
    if ret != 0 { dev_err(dev, "failed to register OF clock provider\n"); }
    ret
}

static AP_CPU_CLOCK_OF_MATCH: [of_device_id; 3] = [
    of_device_id { compatible: "marvell,ap806-cpu-clock", data: &AP806_DFS_REGS },
    of_device_id { compatible: "marvell,ap807-cpu-clock", data: &AP807_DFS_REGS },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

static AP_CPU_CLOCK_DRIVER: platform_driver = platform_driver {
    probe: Some(ap_cpu_clock_probe),
    name: "marvell-ap-cpu-clock",
    of_match_table: &AP_CPU_CLOCK_OF_MATCH,
    suppress_bind_attrs: true,
};

// builtin_platform_driver!(AP_CPU_CLOCK_DRIVER);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
