// SPDX-License-Identifier: GPL-2.0
/*
 * Marvell Armada 375 SoC clocks
 *
 * Copyright (C) 2014 Marvell
 *
 * Gregory CLEMENT <gregory.clement@free-electrons.com>
 * Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
 * Andrew Lunn <andrew@lunn.ch>
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/kernel.h, linux/clk-provider.h, linux/io.h, linux/of.h, common.h

/* Core Clocks */

// For Armada 375, CPU, DDR, and L2 frequencies are selected together.
// SAR1[21:17] selects the CPU/DDR/L2 frequency set; SAR1[22] selects TCLK.
const SAR1_A375_TCLK_FREQ_OPT: u32 = 22;
const SAR1_A375_TCLK_FREQ_OPT_MASK: u32 = 0x1;
const SAR1_A375_CPU_DDR_L2_FREQ_OPT: u32 = 17;
const SAR1_A375_CPU_DDR_L2_FREQ_OPT_MASK: u32 = 0x1f;

static armada_375_tclk_frequencies: [u32; 2] = [166_000_000, 200_000_000];

unsafe fn armada_375_get_tclk_freq(sar: *mut core::ffi::c_void) -> u32 {
    let tclk_freq_select: u8 = ((readl(sar) >> SAR1_A375_TCLK_FREQ_OPT)
        & SAR1_A375_TCLK_FREQ_OPT_MASK) as u8;
    armada_375_tclk_frequencies[tclk_freq_select as usize]
}

static armada_375_cpu_frequencies: [u32; 26] = [
    0, 0, 0, 0, 0, 0, 400_000_000, 0, 0, 0, 0, 0, 0, 0, 0,
    600_000_000, 0, 0, 0, 0, 0, 800_000_000, 0, 0, 0, 1_000_000_000,
];

unsafe fn armada_375_get_cpu_freq(sar: *mut core::ffi::c_void) -> u32 {
    let cpu_freq_select: u8 = ((readl(sar) >> SAR1_A375_CPU_DDR_L2_FREQ_OPT)
        & SAR1_A375_CPU_DDR_L2_FREQ_OPT_MASK) as u8;
    if (cpu_freq_select as usize) >= armada_375_cpu_frequencies.len() {
        pr_err!("Selected CPU frequency ({}) unsupported\n", cpu_freq_select);
        0
    } else {
        armada_375_cpu_frequencies[cpu_freq_select as usize]
    }
}

const A375_CPU_TO_DDR: i32 = 0;
const A375_CPU_TO_L2: i32 = 1;

static armada_375_coreclk_ratios: [coreclk_ratio; 2] = [
    coreclk_ratio { id: A375_CPU_TO_L2, name: "l2clk" },
    coreclk_ratio { id: A375_CPU_TO_DDR, name: "ddrclk" },
];

static armada_375_cpu_l2_ratios: [[i32; 2]; 32] = [
    [0, 1], [0, 1], [0, 1], [0, 1], [0, 1], [0, 1], [1, 2], [0, 1],
    [0, 1], [0, 1], [0, 1], [0, 1], [0, 1], [0, 1], [0, 1], [1, 2],
    [0, 1], [0, 1], [0, 1], [0, 1], [0, 1], [1, 2], [0, 1], [0, 1],
    [0, 1], [1, 2], [0, 1], [0, 1], [0, 1], [0, 1], [0, 1], [0, 1],
];

static armada_375_cpu_ddr_ratios: [[i32; 2]; 32] = [
    [0, 1], [0, 1], [0, 1], [0, 1], [0, 1], [0, 1], [1, 1], [0, 1],
    [0, 1], [0, 1], [0, 1], [0, 1], [0, 1], [0, 1], [0, 1], [2, 3],
    [0, 1], [0, 1], [0, 1], [0, 1], [0, 1], [2, 3], [0, 1], [0, 1],
    [0, 1], [1, 2], [0, 1], [0, 1], [0, 1], [0, 1], [0, 1], [0, 1],
];

unsafe fn armada_375_get_clk_ratio(
    sar: *mut core::ffi::c_void,
    id: i32,
    mult: *mut i32,
    div: *mut i32,
) {
    let opt = ((readl(sar) >> SAR1_A375_CPU_DDR_L2_FREQ_OPT)
        & SAR1_A375_CPU_DDR_L2_FREQ_OPT_MASK) as usize;
    match id {
        A375_CPU_TO_L2 => {
            *mult = armada_375_cpu_l2_ratios[opt][0];
            *div = armada_375_cpu_l2_ratios[opt][1];
        }
        A375_CPU_TO_DDR => {
            *mult = armada_375_cpu_ddr_ratios[opt][0];
            *div = armada_375_cpu_ddr_ratios[opt][1];
        }
        _ => {}
    }
}

static armada_375_coreclks: coreclk_soc_desc = coreclk_soc_desc {
    get_tclk_freq: Some(armada_375_get_tclk_freq),
    get_cpu_freq: Some(armada_375_get_cpu_freq),
    get_clk_ratio: Some(armada_375_get_clk_ratio),
    ratios: armada_375_coreclk_ratios.as_ptr(),
    num_ratios: armada_375_coreclk_ratios.len(),
};

unsafe fn armada_375_coreclk_init(np: *mut device_node) {
    mvebu_coreclk_setup(np, &armada_375_coreclks);
}

// CLK_OF_DECLARE(armada_375_core_clk, "marvell,armada-375-core-clock",
//                armada_375_coreclk_init);

/* Clock Gating Control */
static armada_375_gating_desc: [clk_gating_soc_desc; 23] = [
    clk_gating_soc_desc { name: "mu", parent: core::ptr::null(), bit_idx: 2 },
    clk_gating_soc_desc { name: "pp", parent: core::ptr::null(), bit_idx: 3 },
    clk_gating_soc_desc { name: "ptp", parent: core::ptr::null(), bit_idx: 4 },
    clk_gating_soc_desc { name: "pex0", parent: core::ptr::null(), bit_idx: 5 },
    clk_gating_soc_desc { name: "pex1", parent: core::ptr::null(), bit_idx: 6 },
    clk_gating_soc_desc { name: "audio", parent: core::ptr::null(), bit_idx: 8 },
    clk_gating_soc_desc { name: "nd_clk", parent: "nand", bit_idx: 11 },
    clk_gating_soc_desc { name: "sata0_link", parent: "sata0_core", bit_idx: 14 },
    clk_gating_soc_desc { name: "sata0_core", parent: core::ptr::null(), bit_idx: 15 },
    clk_gating_soc_desc { name: "usb3", parent: core::ptr::null(), bit_idx: 16 },
    clk_gating_soc_desc { name: "sdio", parent: core::ptr::null(), bit_idx: 17 },
    clk_gating_soc_desc { name: "usb", parent: core::ptr::null(), bit_idx: 18 },
    clk_gating_soc_desc { name: "gop", parent: core::ptr::null(), bit_idx: 19 },
    clk_gating_soc_desc { name: "sata1_link", parent: "sata1_core", bit_idx: 20 },
    clk_gating_soc_desc { name: "sata1_core", parent: core::ptr::null(), bit_idx: 21 },
    clk_gating_soc_desc { name: "xor0", parent: core::ptr::null(), bit_idx: 22 },
    clk_gating_soc_desc { name: "xor1", parent: core::ptr::null(), bit_idx: 23 },
    clk_gating_soc_desc { name: "copro", parent: core::ptr::null(), bit_idx: 24 },
    clk_gating_soc_desc { name: "tdm", parent: core::ptr::null(), bit_idx: 25 },
    clk_gating_soc_desc { name: "crypto0_enc", parent: core::ptr::null(), bit_idx: 28 },
    clk_gating_soc_desc { name: "crypto0_core", parent: core::ptr::null(), bit_idx: 29 },
    clk_gating_soc_desc { name: "crypto1_enc", parent: core::ptr::null(), bit_idx: 30 },
    clk_gating_soc_desc { name: "crypto1_core", parent: core::ptr::null(), bit_idx: 31 },
];

unsafe fn armada_375_clk_gating_init(np: *mut device_node) {
    mvebu_clk_gating_setup(np, armada_375_gating_desc.as_ptr());
}

// CLK_OF_DECLARE(armada_375_clk_gating, "marvell,armada-375-gating-clock",
//                armada_375_clk_gating_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
