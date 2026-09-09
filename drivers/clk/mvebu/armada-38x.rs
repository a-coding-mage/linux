// SPDX-License-Identifier: GPL-2.0
/*
 * Marvell Armada 380/385 SoC clocks
 *
 * Copyright (C) 2014 Marvell
 *
 * Gregory CLEMENT <gregory.clement@free-electrons.com>
 * Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
 * Andrew Lunn <andrew@lunn.ch>
 */

// Dependencies supplied by the surrounding kernel translation.

/*
 * SAR[14:10] : Ratios between PCLK0, NBCLK, HCLK and DRAM clocks
 *
 * SAR[15]    : TCLK frequency
 *             0 = 250 MHz
 *             1 = 200 MHz
 */

const SAR_A380_TCLK_FREQ_OPT: u32 = 15;
const SAR_A380_TCLK_FREQ_OPT_MASK: u32 = 0x1;
const SAR_A380_CPU_DDR_L2_FREQ_OPT: u32 = 10;
const SAR_A380_CPU_DDR_L2_FREQ_OPT_MASK: u32 = 0x1f;

static ARMADA_38X_TCLK_FREQUENCIES: [u32; 2] = [250000000, 200000000];

unsafe fn armada_38x_get_tclk_freq(sar: *mut core::ffi::c_void) -> u32 {
    let tclk_freq_select: u8 = ((readl(sar) >> SAR_A380_TCLK_FREQ_OPT)
        & SAR_A380_TCLK_FREQ_OPT_MASK) as u8;
    ARMADA_38X_TCLK_FREQUENCIES[tclk_freq_select as usize]
}

static ARMADA_38X_CPU_FREQUENCIES: [u32; 20] = [
    666 * 1000 * 1000, 0, 800 * 1000 * 1000, 0,
    1066 * 1000 * 1000, 0, 1200 * 1000 * 1000, 0,
    1332 * 1000 * 1000, 0, 0, 0,
    1600 * 1000 * 1000, 0, 0, 0,
    1866 * 1000 * 1000, 0, 0, 2000 * 1000 * 1000,
];

unsafe fn armada_38x_get_cpu_freq(sar: *mut core::ffi::c_void) -> u32 {
    let cpu_freq_select = ((readl(sar) >> SAR_A380_CPU_DDR_L2_FREQ_OPT)
        & SAR_A380_CPU_DDR_L2_FREQ_OPT_MASK) as u8;
    if cpu_freq_select as usize >= ARMADA_38X_CPU_FREQUENCIES.len() {
        pr_err!("Selected CPU frequency ({}) unsupported\n", cpu_freq_select);
        return 0;
    }
    ARMADA_38X_CPU_FREQUENCIES[cpu_freq_select as usize]
}

enum Armada38xCoreRatio {
    A380_CPU_TO_DDR,
    A380_CPU_TO_L2,
}

static ARMADA_38X_CORECLK_RATIOS: [coreclk_ratio; 2] = [
    coreclk_ratio { id: A380_CPU_TO_L2, name: "l2clk" },
    coreclk_ratio { id: A380_CPU_TO_DDR, name: "ddrclk" },
];

static ARMADA_38X_CPU_L2_RATIOS: [[i32; 2]; 32] = [
    [1, 2], [0, 1], [1, 2], [0, 1],
    [1, 2], [0, 1], [1, 2], [0, 1],
    [1, 2], [0, 1], [0, 1], [0, 1],
    [1, 2], [0, 1], [0, 1], [0, 1],
    [1, 2], [0, 1], [0, 1], [1, 2],
    [0, 1], [0, 1], [0, 1], [0, 1],
    [0, 1], [0, 1], [0, 1], [0, 1],
    [0, 1], [0, 1], [0, 1], [0, 1],
];

static ARMADA_38X_CPU_DDR_RATIOS: [[i32; 2]; 32] = [
    [0, 1], [0, 1], [0, 1], [0, 1],
    [1, 2], [0, 1], [0, 1], [0, 1],
    [1, 2], [0, 1], [0, 1], [0, 1],
    [1, 2], [0, 1], [0, 1], [0, 1],
    [1, 2], [0, 1], [0, 1], [7, 15],
    [0, 1], [0, 1], [0, 1], [0, 1],
    [0, 1], [0, 1], [0, 1], [0, 1],
    [0, 1], [0, 1], [0, 1], [0, 1],
];

unsafe fn armada_38x_get_clk_ratio(
    sar: *mut core::ffi::c_void,
    id: i32,
    mult: *mut i32,
    div: *mut i32,
) {
    let opt = ((readl(sar) >> SAR_A380_CPU_DDR_L2_FREQ_OPT)
        & SAR_A380_CPU_DDR_L2_FREQ_OPT_MASK) as usize;
    match id {
        A380_CPU_TO_L2 => {
            *mult = ARMADA_38X_CPU_L2_RATIOS[opt][0];
            *div = ARMADA_38X_CPU_L2_RATIOS[opt][1];
        }
        A380_CPU_TO_DDR => {
            *mult = ARMADA_38X_CPU_DDR_RATIOS[opt][0];
            *div = ARMADA_38X_CPU_DDR_RATIOS[opt][1];
        }
        _ => {}
    }
}

static ARMADA_38X_CORECLKS: coreclk_soc_desc = coreclk_soc_desc {
    get_tclk_freq: armada_38x_get_tclk_freq,
    get_cpu_freq: armada_38x_get_cpu_freq,
    get_clk_ratio: armada_38x_get_clk_ratio,
    ratios: ARMADA_38X_CORECLK_RATIOS.as_ptr(),
    num_ratios: ARMADA_38X_CORECLK_RATIOS.len(),
};

unsafe fn armada_38x_coreclk_init(np: *mut device_node) {
    mvebu_coreclk_setup(np, &ARMADA_38X_CORECLKS);
}

// CLK_OF_DECLARE(armada_38x_core_clk, "marvell,armada-380-core-clock",
//                armada_38x_coreclk_init);

/* Clock Gating Control */
static ARMADA_38X_GATING_DESC: [clk_gating_soc_desc; 24] = [
    clk_gating_soc_desc { name: "audio", parent: core::ptr::null(), bit_idx: 0 },
    clk_gating_soc_desc { name: "ge2", parent: core::ptr::null(), bit_idx: 2 },
    clk_gating_soc_desc { name: "ge1", parent: core::ptr::null(), bit_idx: 3 },
    clk_gating_soc_desc { name: "ge0", parent: core::ptr::null(), bit_idx: 4 },
    clk_gating_soc_desc { name: "pex1", parent: core::ptr::null(), bit_idx: 5 },
    clk_gating_soc_desc { name: "pex2", parent: core::ptr::null(), bit_idx: 6 },
    clk_gating_soc_desc { name: "pex3", parent: core::ptr::null(), bit_idx: 7 },
    clk_gating_soc_desc { name: "pex0", parent: core::ptr::null(), bit_idx: 8 },
    clk_gating_soc_desc { name: "usb3h0", parent: core::ptr::null(), bit_idx: 9 },
    clk_gating_soc_desc { name: "usb3h1", parent: core::ptr::null(), bit_idx: 10 },
    clk_gating_soc_desc { name: "usb3d", parent: core::ptr::null(), bit_idx: 11 },
    clk_gating_soc_desc { name: "bm", parent: core::ptr::null(), bit_idx: 13 },
    clk_gating_soc_desc { name: "crypto0z", parent: core::ptr::null(), bit_idx: 14 },
    clk_gating_soc_desc { name: "sata0", parent: core::ptr::null(), bit_idx: 15 },
    clk_gating_soc_desc { name: "crypto1z", parent: core::ptr::null(), bit_idx: 16 },
    clk_gating_soc_desc { name: "sdio", parent: core::ptr::null(), bit_idx: 17 },
    clk_gating_soc_desc { name: "usb2", parent: core::ptr::null(), bit_idx: 18 },
    clk_gating_soc_desc { name: "crypto1", parent: core::ptr::null(), bit_idx: 21 },
    clk_gating_soc_desc { name: "xor0", parent: core::ptr::null(), bit_idx: 22 },
    clk_gating_soc_desc { name: "crypto0", parent: core::ptr::null(), bit_idx: 23 },
    clk_gating_soc_desc { name: "tdm", parent: core::ptr::null(), bit_idx: 25 },
    clk_gating_soc_desc { name: "xor1", parent: core::ptr::null(), bit_idx: 28 },
    clk_gating_soc_desc { name: "sata1", parent: core::ptr::null(), bit_idx: 30 },
    clk_gating_soc_desc { name: "", parent: core::ptr::null(), bit_idx: 0 },
];

unsafe fn armada_38x_clk_gating_init(np: *mut device_node) {
    mvebu_clk_gating_setup(np, ARMADA_38X_GATING_DESC.as_ptr());
}

// CLK_OF_DECLARE(armada_38x_clk_gating, "marvell,armada-380-gating-clock",
//                armada_38x_clk_gating_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
