// SPDX-License-Identifier: GPL-2.0
/*
 * Marvell Dove SoC clocks
 *
 * Copyright (C) 2012 Marvell
 *
 * Gregory CLEMENT <gregory.clement@free-electrons.com>
 * Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
 * Andrew Lunn <andrew@lunn.ch>
 */

// Kernel dependencies supplied by the surrounding translation unit.

/*
 * Core Clocks
 *
 * Dove PLL sample-at-reset configuration
 *
 * SAR0[8:5]   : CPU frequency
 *             5  = 1000 MHz
 *             6  =  933 MHz
 *             7  =  933 MHz
 *             8  =  800 MHz
 *             9  =  800 MHz
 *             10 =  800 MHz
 *             11 = 1067 MHz
 *             12 =  667 MHz
 *             13 =  533 MHz
 *             14 =  400 MHz
 *             15 =  333 MHz
 *             others reserved.
 *
 * SAR0[11:9]  : CPU to L2 Clock divider ratio
 *             0 = (1/1) * CPU
 *             2 = (1/2) * CPU
 *             4 = (1/3) * CPU
 *             6 = (1/4) * CPU
 *             others reserved.
 *
 * SAR0[15:12] : CPU to DDR DRAM Clock divider ratio
 *             0  = (1/1) * CPU
 *             2  = (1/2) * CPU
 *             3  = (2/5) * CPU
 *             4  = (1/3) * CPU
 *             6  = (1/4) * CPU
 *             8  = (1/5) * CPU
 *             10 = (1/6) * CPU
 *             12 = (1/7) * CPU
 *             14 = (1/8) * CPU
 *             15 = (1/10) * CPU
 *             others reserved.
 *
 * SAR0[24:23] : TCLK frequency
 *             0 = 166 MHz
 *             1 = 125 MHz
 *             others reserved.
 */

const SAR_DOVE_CPU_FREQ: u32 = 5;
const SAR_DOVE_CPU_FREQ_MASK: u32 = 0xf;
const SAR_DOVE_L2_RATIO: u32 = 9;
const SAR_DOVE_L2_RATIO_MASK: u32 = 0x7;
const SAR_DOVE_DDR_RATIO: u32 = 12;
const SAR_DOVE_DDR_RATIO_MASK: u32 = 0xf;
const SAR_DOVE_TCLK_FREQ: u32 = 23;
const SAR_DOVE_TCLK_FREQ_MASK: u32 = 0x3;

const DOVE_CPU_TO_L2: i32 = 0;
const DOVE_CPU_TO_DDR: i32 = 1;

static DOVE_CORECLK_RATIOS: [CoreclkRatio; 2] = [
    CoreclkRatio { id: DOVE_CPU_TO_L2, name: "l2clk" },
    CoreclkRatio { id: DOVE_CPU_TO_DDR, name: "ddrclk" },
];

static DOVE_TCLK_FREQS: [u32; 4] = [166666667, 125000000, 0, 0];

unsafe fn dove_get_tclk_freq(sar: *mut u8) -> u32 {
    let opt = (readl(sar) >> SAR_DOVE_TCLK_FREQ) & SAR_DOVE_TCLK_FREQ_MASK;
    DOVE_TCLK_FREQS[opt as usize]
}

static DOVE_CPU_FREQS: [u32; 16] = [
    0, 0, 0, 0, 0, 1000000000, 933333333, 933333333,
    800000000, 800000000, 800000000, 1066666667, 666666667,
    533333333, 400000000, 333333333,
];

unsafe fn dove_get_cpu_freq(sar: *mut u8) -> u32 {
    let opt = (readl(sar) >> SAR_DOVE_CPU_FREQ) & SAR_DOVE_CPU_FREQ_MASK;
    DOVE_CPU_FREQS[opt as usize]
}

static DOVE_CPU_L2_RATIOS: [[i32; 2]; 8] = [
    [1, 1], [0, 1], [1, 2], [0, 1], [1, 3], [0, 1], [1, 4], [0, 1],
];

static DOVE_CPU_DDR_RATIOS: [[i32; 2]; 16] = [
    [1, 1], [0, 1], [1, 2], [2, 5], [1, 3], [0, 1], [1, 4], [0, 1],
    [1, 5], [0, 1], [1, 6], [0, 1], [1, 7], [0, 1], [1, 8], [1, 10],
];

unsafe fn dove_get_clk_ratio(sar: *mut u8, id: i32, mult: *mut i32, div: *mut i32) {
    match id {
        DOVE_CPU_TO_L2 => {
            let opt = (readl(sar) >> SAR_DOVE_L2_RATIO) & SAR_DOVE_L2_RATIO_MASK;
            *mult = DOVE_CPU_L2_RATIOS[opt as usize][0];
            *div = DOVE_CPU_L2_RATIOS[opt as usize][1];
        }
        DOVE_CPU_TO_DDR => {
            let opt = (readl(sar) >> SAR_DOVE_DDR_RATIO) & SAR_DOVE_DDR_RATIO_MASK;
            *mult = DOVE_CPU_DDR_RATIOS[opt as usize][0];
            *div = DOVE_CPU_DDR_RATIOS[opt as usize][1];
        }
        _ => {}
    }
}

static DOVE_CORECLKS: CoreclkSocDesc = CoreclkSocDesc {
    get_tclk_freq: dove_get_tclk_freq,
    get_cpu_freq: dove_get_cpu_freq,
    get_clk_ratio: dove_get_clk_ratio,
    ratios: DOVE_CORECLK_RATIOS.as_ptr(),
    num_ratios: DOVE_CORECLK_RATIOS.len(),
};

/* Clock Gating Control */

static DOVE_GATING_DESC: [ClkGatingSocDesc; 19] = [
    ClkGatingSocDesc { name: "usb0", parent: None, bit_idx: 0, flags: 0 },
    ClkGatingSocDesc { name: "usb1", parent: None, bit_idx: 1, flags: 0 },
    ClkGatingSocDesc { name: "ge", parent: Some("gephy"), bit_idx: 2, flags: 0 },
    ClkGatingSocDesc { name: "sata", parent: None, bit_idx: 3, flags: 0 },
    ClkGatingSocDesc { name: "pex0", parent: None, bit_idx: 4, flags: 0 },
    ClkGatingSocDesc { name: "pex1", parent: None, bit_idx: 5, flags: 0 },
    ClkGatingSocDesc { name: "sdio0", parent: None, bit_idx: 8, flags: 0 },
    ClkGatingSocDesc { name: "sdio1", parent: None, bit_idx: 9, flags: 0 },
    ClkGatingSocDesc { name: "nand", parent: None, bit_idx: 10, flags: 0 },
    ClkGatingSocDesc { name: "camera", parent: None, bit_idx: 11, flags: 0 },
    ClkGatingSocDesc { name: "i2s0", parent: None, bit_idx: 12, flags: 0 },
    ClkGatingSocDesc { name: "i2s1", parent: None, bit_idx: 13, flags: 0 },
    ClkGatingSocDesc { name: "crypto", parent: None, bit_idx: 15, flags: 0 },
    ClkGatingSocDesc { name: "ac97", parent: None, bit_idx: 21, flags: 0 },
    ClkGatingSocDesc { name: "pdma", parent: None, bit_idx: 22, flags: 0 },
    ClkGatingSocDesc { name: "xor0", parent: None, bit_idx: 23, flags: 0 },
    ClkGatingSocDesc { name: "xor1", parent: None, bit_idx: 24, flags: 0 },
    ClkGatingSocDesc { name: "gephy", parent: None, bit_idx: 30, flags: 0 },
    ClkGatingSocDesc::default(),
];

unsafe fn dove_clk_init(np: *mut DeviceNode) {
    let cgnp = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "marvell,dove-gating-clock");
    let ddnp = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "marvell,dove-divider-clock");

    mvebu_coreclk_setup(np, &DOVE_CORECLKS);
    if !ddnp.is_null() {
        dove_divider_clk_init(ddnp);
        of_node_put(ddnp);
    }
    if !cgnp.is_null() {
        mvebu_clk_gating_setup(cgnp, DOVE_GATING_DESC.as_ptr());
        of_node_put(cgnp);
    }
}

// Equivalent of CLK_OF_DECLARE(dove_clk, "marvell,dove-core-clock", dove_clk_init).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
