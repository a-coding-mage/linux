// SPDX-License-Identifier: GPL-2.0
/*
 * Marvell Armada XP SoC clocks
 *
 * Copyright (C) 2012 Marvell
 *
 * Gregory CLEMENT <gregory.clement@free-electrons.com>
 * Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
 * Andrew Lunn <andrew@lunn.ch>
 */

// Dependency declarations supplied by the surrounding kernel translation.

/* Core Clocks */
const SARL: usize = 0;
const SARL_AXP_PCLK_FREQ_OPT: u32 = 21;
const SARL_AXP_PCLK_FREQ_OPT_MASK: u32 = 0x7;
const SARL_AXP_FAB_FREQ_OPT: u32 = 24;
const SARL_AXP_FAB_FREQ_OPT_MASK: u32 = 0xF;
const SARH: usize = 4;
const SARH_AXP_PCLK_FREQ_OPT: u32 = 52 - 32;
const SARH_AXP_PCLK_FREQ_OPT_MASK: u32 = 0x1;
const SARH_AXP_PCLK_FREQ_OPT_SHIFT: u32 = 3;
const SARH_AXP_FAB_FREQ_OPT: u32 = 51 - 32;
const SARH_AXP_FAB_FREQ_OPT_MASK: u32 = 0x1;
const SARH_AXP_FAB_FREQ_OPT_SHIFT: u32 = 4;

const AXP_CPU_TO_NBCLK: i32 = 0;
const AXP_CPU_TO_HCLK: i32 = 1;
const AXP_CPU_TO_DRAMCLK: i32 = 2;

static AXP_CORECLK_RATIOS: [coreclk_ratio; 3] = [
    coreclk_ratio { id: AXP_CPU_TO_NBCLK, name: "nbclk" },
    coreclk_ratio { id: AXP_CPU_TO_HCLK, name: "hclk" },
    coreclk_ratio { id: AXP_CPU_TO_DRAMCLK, name: "dramclk" },
];

/* Armada XP TCLK frequency is fixed to 250MHz */
unsafe fn axp_get_tclk_freq(_sar: *mut core::ffi::c_void) -> u32 {
    250000000
}

static AXP_CPU_FREQS: [u32; 12] = [
    1000000000, 1066000000, 1200000000, 1333000000,
    1500000000, 1666000000, 1800000000, 2000000000,
    667000000, 0, 800000000, 1600000000,
];

unsafe fn axp_get_cpu_freq(sar: *mut u8) -> u32 {
    let mut cpu_freq: u32;
    let mut cpu_freq_select: u8 = 0;

    cpu_freq_select = ((readl(sar.add(SARL)) >> SARL_AXP_PCLK_FREQ_OPT)
        & SARL_AXP_PCLK_FREQ_OPT_MASK) as u8;
    /*
     * The upper bit is not contiguous to the other ones and
     * located in the high part of the SAR registers
     */
    cpu_freq_select |= (((readl(sar.add(SARH)) >> SARH_AXP_PCLK_FREQ_OPT)
        & SARH_AXP_PCLK_FREQ_OPT_MASK) << SARH_AXP_PCLK_FREQ_OPT_SHIFT) as u8;
    if (cpu_freq_select as usize >= AXP_CPU_FREQS.len() {
        pr_err!("CPU freq select unsupported: %d\n", cpu_freq_select);
        cpu_freq = 0;
    } else {
        cpu_freq = AXP_CPU_FREQS[cpu_freq_select as usize];
    }
    cpu_freq
}

static AXP_NBCLK_RATIOS: [[i32; 2]; 32] = [
    [0,1], [1,2], [2,2], [2,2], [1,2], [1,2], [1,1], [2,3],
    [0,1], [1,2], [2,4], [0,1], [1,2], [0,1], [0,1], [2,2],
    [0,1], [0,1], [0,1], [1,1], [2,3], [0,1], [0,1], [0,1],
    [0,1], [0,1], [0,1], [1,1], [0,1], [0,1], [0,1], [0,1],
];

static AXP_HCLK_RATIOS: [[i32; 2]; 32] = [
    [0,1], [1,2], [2,6], [2,3], [1,3], [1,4], [1,2], [2,6],
    [0,1], [1,6], [2,10], [0,1], [1,4], [0,1], [0,1], [2,5],
    [0,1], [0,1], [0,1], [1,2], [2,6], [0,1], [0,1], [0,1],
    [0,1], [0,1], [0,1], [1,1], [0,1], [0,1], [0,1], [0,1],
];

static AXP_DRAMCLK_RATIOS: [[i32; 2]; 32] = [
    [0,1], [1,2], [2,3], [2,3], [1,3], [1,2], [1,2], [2,6],
    [0,1], [1,3], [2,5], [0,1], [1,4], [0,1], [0,1], [2,5],
    [0,1], [0,1], [0,1], [1,1], [2,3], [0,1], [0,1], [0,1],
    [0,1], [0,1], [0,1], [1,1], [0,1], [0,1], [0,1], [0,1],
];

unsafe fn axp_get_clk_ratio(sar: *mut u8, id: i32, mult: *mut i32, div: *mut i32) {
    let mut opt = (readl(sar.add(SARL)) >> SARL_AXP_FAB_FREQ_OPT)
        & SARL_AXP_FAB_FREQ_OPT_MASK;
    /* The upper bit is not contiguous to the other ones and located in the high part. */
    opt |= ((readl(sar.add(SARH)) >> SARH_AXP_FAB_FREQ_OPT)
        & SARH_AXP_FAB_FREQ_OPT_MASK) << SARH_AXP_FAB_FREQ_OPT_SHIFT;
    let ratio = match id {
        AXP_CPU_TO_NBCLK => &AXP_NBCLK_RATIOS[opt as usize],
        AXP_CPU_TO_HCLK => &AXP_HCLK_RATIOS[opt as usize],
        AXP_CPU_TO_DRAMCLK => &AXP_DRAMCLK_RATIOS[opt as usize],
        _ => return,
    };
    *mult = ratio[0];
    *div = ratio[1];
}

static AXP_CORECLKS: coreclk_soc_desc = coreclk_soc_desc {
    get_tclk_freq: Some(axp_get_tclk_freq),
    get_cpu_freq: Some(axp_get_cpu_freq),
    get_clk_ratio: Some(axp_get_clk_ratio),
    ratios: AXP_CORECLK_RATIOS.as_ptr(),
    num_ratios: AXP_CORECLK_RATIOS.len(),
};

/* Clock Gating Control */
static AXP_GATING_DESC: [clk_gating_soc_desc; 30] = [
    clk_gating_soc_desc { name: "audio", parent: core::ptr::null(), bit_idx: 0, flags: 0 },
    clk_gating_soc_desc { name: "ge3", parent: core::ptr::null(), bit_idx: 1, flags: 0 },
    clk_gating_soc_desc { name: "ge2", parent: core::ptr::null(), bit_idx: 2, flags: 0 },
    clk_gating_soc_desc { name: "ge1", parent: core::ptr::null(), bit_idx: 3, flags: 0 },
    clk_gating_soc_desc { name: "ge0", parent: core::ptr::null(), bit_idx: 4, flags: 0 },
    clk_gating_soc_desc { name: "pex00", parent: core::ptr::null(), bit_idx: 5, flags: 0 },
    clk_gating_soc_desc { name: "pex01", parent: core::ptr::null(), bit_idx: 6, flags: 0 },
    clk_gating_soc_desc { name: "pex02", parent: core::ptr::null(), bit_idx: 7, flags: 0 },
    clk_gating_soc_desc { name: "pex03", parent: core::ptr::null(), bit_idx: 8, flags: 0 },
    clk_gating_soc_desc { name: "pex10", parent: core::ptr::null(), bit_idx: 9, flags: 0 },
    clk_gating_soc_desc { name: "pex11", parent: core::ptr::null(), bit_idx: 10, flags: 0 },
    clk_gating_soc_desc { name: "pex12", parent: core::ptr::null(), bit_idx: 11, flags: 0 },
    clk_gating_soc_desc { name: "pex13", parent: core::ptr::null(), bit_idx: 12, flags: 0 },
    clk_gating_soc_desc { name: "bp", parent: core::ptr::null(), bit_idx: 13, flags: 0 },
    clk_gating_soc_desc { name: "sata0lnk", parent: core::ptr::null(), bit_idx: 14, flags: 0 },
    clk_gating_soc_desc { name: "sata0", parent: "sata0lnk", bit_idx: 15, flags: 0 },
    clk_gating_soc_desc { name: "lcd", parent: core::ptr::null(), bit_idx: 16, flags: 0 },
    clk_gating_soc_desc { name: "sdio", parent: core::ptr::null(), bit_idx: 17, flags: 0 },
    clk_gating_soc_desc { name: "usb0", parent: core::ptr::null(), bit_idx: 18, flags: 0 },
    clk_gating_soc_desc { name: "usb1", parent: core::ptr::null(), bit_idx: 19, flags: 0 },
    clk_gating_soc_desc { name: "usb2", parent: core::ptr::null(), bit_idx: 20, flags: 0 },
    clk_gating_soc_desc { name: "xor0", parent: core::ptr::null(), bit_idx: 22, flags: 0 },
    clk_gating_soc_desc { name: "crypto", parent: core::ptr::null(), bit_idx: 23, flags: 0 },
    clk_gating_soc_desc { name: "tdm", parent: core::ptr::null(), bit_idx: 25, flags: 0 },
    clk_gating_soc_desc { name: "pex20", parent: core::ptr::null(), bit_idx: 26, flags: 0 },
    clk_gating_soc_desc { name: "pex30", parent: core::ptr::null(), bit_idx: 27, flags: 0 },
    clk_gating_soc_desc { name: "xor1", parent: core::ptr::null(), bit_idx: 28, flags: 0 },
    clk_gating_soc_desc { name: "sata1lnk", parent: core::ptr::null(), bit_idx: 29, flags: 0 },
    clk_gating_soc_desc { name: "sata1", parent: "sata1lnk", bit_idx: 30, flags: 0 },
    clk_gating_soc_desc { name: core::ptr::null(), parent: core::ptr::null(), bit_idx: 0, flags: 0 },
];

unsafe fn axp_clk_init(np: *mut device_node) {
    let cgnp = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(),
        "marvell,armada-xp-gating-clock");
    mvebu_coreclk_setup(np, &AXP_CORECLKS);
    if !cgnp.is_null() {
        mvebu_clk_gating_setup(cgnp, AXP_GATING_DESC.as_ptr());
        of_node_put(cgnp);
    }
}

// CLK_OF_DECLARE(axp_clk, "marvell,armada-xp-core-clock", axp_clk_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
