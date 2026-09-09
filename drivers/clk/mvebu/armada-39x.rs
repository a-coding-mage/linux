// SPDX-License-Identifier: GPL-2.0
/*
 * Marvell Armada 39x SoC clocks
 *
 * Copyright (C) 2015 Marvell
 *
 * Gregory CLEMENT <gregory.clement@free-electrons.com>
 * Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
 * Andrew Lunn <andrew@lunn.ch>
 * Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
 */

// Dependencies supplied by the surrounding kernel translation.

const SARL: usize = 0;
const SARL_A390_TCLK_FREQ_OPT: u32 = 15;
const SARL_A390_TCLK_FREQ_OPT_MASK: u32 = 0x1;
const SARL_A390_CPU_DDR_L2_FREQ_OPT: u32 = 10;
const SARL_A390_CPU_DDR_L2_FREQ_OPT_MASK: u32 = 0x1f;
const SARH: usize = 4;
const SARH_A390_REFCLK_FREQ: u32 = 1 << 0;

static ARMADA_39X_TCLK_FREQUENCIES: [u32; 2] = [250_000_000, 200_000_000];

unsafe fn armada_39x_get_tclk_freq(sar: *mut core::ffi::c_void) -> u32 {
    let tclk_freq_select: usize =
        ((core::ptr::read_volatile((sar as *mut u8).add(SARL) as *const u32)
            >> SARL_A390_TCLK_FREQ_OPT)
            & SARL_A390_TCLK_FREQ_OPT_MASK) as usize;
    ARMADA_39X_TCLK_FREQUENCIES[tclk_freq_select]
}

static ARMADA_39X_CPU_FREQUENCIES: [u32; 31] = {
    let mut frequencies = [0u32; 31];
    frequencies[0x0] = 666 * 1000 * 1000;
    frequencies[0x2] = 800 * 1000 * 1000;
    frequencies[0x3] = 800 * 1000 * 1000;
    frequencies[0x4] = 1066 * 1000 * 1000;
    frequencies[0x5] = 1066 * 1000 * 1000;
    frequencies[0x6] = 1200 * 1000 * 1000;
    frequencies[0x8] = 1332 * 1000 * 1000;
    frequencies[0xb] = 1600 * 1000 * 1000;
    frequencies[0xc] = 1600 * 1000 * 1000;
    frequencies[0x12] = 1800 * 1000 * 1000;
    frequencies[0x1e] = 1800 * 1000 * 1000;
    frequencies
};

unsafe fn armada_39x_get_cpu_freq(sar: *mut core::ffi::c_void) -> u32 {
    let cpu_freq_select =
        ((core::ptr::read_volatile((sar as *mut u8).add(SARL) as *const u32)
            >> SARL_A390_CPU_DDR_L2_FREQ_OPT)
            & SARL_A390_CPU_DDR_L2_FREQ_OPT_MASK) as usize;
    if cpu_freq_select >= ARMADA_39X_CPU_FREQUENCIES.len() {
        // pr_err("Selected CPU frequency (%d) unsupported\n", cpu_freq_select);
        return 0;
    }
    ARMADA_39X_CPU_FREQUENCIES[cpu_freq_select]
}

const A390_CPU_TO_NBCLK: i32 = 0;
const A390_CPU_TO_HCLK: i32 = 1;
const A390_CPU_TO_DCLK: i32 = 2;

// `coreclk_ratio`, `coreclk_soc_desc`, `device_node`, and setup functions are
// supplied by the surrounding kernel translation.
static ARMADA_39X_CORECLK_RATIOS: [coreclk_ratio; 3] = [
    coreclk_ratio { id: A390_CPU_TO_NBCLK, name: "nbclk" },
    coreclk_ratio { id: A390_CPU_TO_HCLK, name: "hclk" },
    coreclk_ratio { id: A390_CPU_TO_DCLK, name: "dclk" },
];

unsafe fn armada_39x_get_clk_ratio(
    _sar: *mut core::ffi::c_void,
    id: i32,
    mult: *mut i32,
    div: *mut i32,
) {
    match id {
        A390_CPU_TO_NBCLK => { *mult = 1; *div = 2; }
        A390_CPU_TO_HCLK => { *mult = 1; *div = 4; }
        A390_CPU_TO_DCLK => { *mult = 1; *div = 2; }
        _ => {}
    }
}

unsafe fn armada_39x_refclk_ratio(sar: *mut core::ffi::c_void) -> u32 {
    if core::ptr::read_volatile((sar as *mut u8).add(SARH) as *const u32) & SARH_A390_REFCLK_FREQ != 0 {
        40 * 1000 * 1000
    } else {
        25 * 1000 * 1000
    }
}

static ARMADA_39X_CORECLKS: coreclk_soc_desc = coreclk_soc_desc {
    get_tclk_freq: Some(armada_39x_get_tclk_freq),
    get_cpu_freq: Some(armada_39x_get_cpu_freq),
    get_clk_ratio: Some(armada_39x_get_clk_ratio),
    get_refclk_freq: Some(armada_39x_refclk_ratio),
    ratios: ARMADA_39X_CORECLK_RATIOS.as_ptr(),
    num_ratios: ARMADA_39X_CORECLK_RATIOS.len(),
};

unsafe fn armada_39x_coreclk_init(np: *mut device_node) {
    mvebu_coreclk_setup(np, &ARMADA_39X_CORECLKS);
}

// CLK_OF_DECLARE(armada_39x_core_clk, "marvell,armada-390-core-clock",
//                armada_39x_coreclk_init);

static ARMADA_39X_GATING_DESC: [clk_gating_soc_desc; 11] = [
    clk_gating_soc_desc { name: "pex1", parent: core::ptr::null(), bit_idx: 5 },
    clk_gating_soc_desc { name: "pex2", parent: core::ptr::null(), bit_idx: 6 },
    clk_gating_soc_desc { name: "pex3", parent: core::ptr::null(), bit_idx: 7 },
    clk_gating_soc_desc { name: "pex0", parent: core::ptr::null(), bit_idx: 8 },
    clk_gating_soc_desc { name: "usb3h0", parent: core::ptr::null(), bit_idx: 9 },
    clk_gating_soc_desc { name: "usb3h1", parent: core::ptr::null(), bit_idx: 10 },
    clk_gating_soc_desc { name: "sata0", parent: core::ptr::null(), bit_idx: 15 },
    clk_gating_soc_desc { name: "sdio", parent: core::ptr::null(), bit_idx: 17 },
    clk_gating_soc_desc { name: "xor0", parent: core::ptr::null(), bit_idx: 22 },
    clk_gating_soc_desc { name: "xor1", parent: core::ptr::null(), bit_idx: 28 },
    clk_gating_soc_desc { name: "", parent: core::ptr::null(), bit_idx: 0 },
];

unsafe fn armada_39x_clk_gating_init(np: *mut device_node) {
    mvebu_clk_gating_setup(np, ARMADA_39X_GATING_DESC.as_ptr());
}

// CLK_OF_DECLARE(armada_39x_clk_gating, "marvell,armada-390-gating-clock",
//                armada_39x_clk_gating_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
