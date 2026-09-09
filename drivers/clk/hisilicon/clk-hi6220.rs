// SPDX-License-Identifier: GPL-2.0-only
/* Hisilicon Hi6220 clock driver; translated from clk-hi6220.c. */

// External kernel types, constants, functions, and registration macros are
// supplied by the surrounding translated kernel sources.

static mut HI6220_FIXED_RATE_CLKS: [hisi_fixed_rate_clock; 13] = [
    hisi_fixed_rate_clock { id: HI6220_REF32K, name: b"ref32k\0".as_ptr(), parent: core::ptr::null(), flags: 0, rate: 32764 },
    hisi_fixed_rate_clock { id: HI6220_CLK_TCXO, name: b"clk_tcxo\0".as_ptr(), parent: core::ptr::null(), flags: 0, rate: 19200000 },
    hisi_fixed_rate_clock { id: HI6220_MMC1_PAD, name: b"mmc1_pad\0".as_ptr(), parent: core::ptr::null(), flags: 0, rate: 100000000 },
    hisi_fixed_rate_clock { id: HI6220_MMC2_PAD, name: b"mmc2_pad\0".as_ptr(), parent: core::ptr::null(), flags: 0, rate: 100000000 },
    hisi_fixed_rate_clock { id: HI6220_MMC0_PAD, name: b"mmc0_pad\0".as_ptr(), parent: core::ptr::null(), flags: 0, rate: 200000000 },
    hisi_fixed_rate_clock { id: HI6220_PLL_BBP, name: b"bbppll0\0".as_ptr(), parent: core::ptr::null(), flags: 0, rate: 245760000 },
    hisi_fixed_rate_clock { id: HI6220_PLL_GPU, name: b"gpupll\0".as_ptr(), parent: core::ptr::null(), flags: 0, rate: 1000000000 },
    hisi_fixed_rate_clock { id: HI6220_PLL1_DDR, name: b"ddrpll1\0".as_ptr(), parent: core::ptr::null(), flags: 0, rate: 1066000000 },
    hisi_fixed_rate_clock { id: HI6220_PLL_SYS, name: b"syspll\0".as_ptr(), parent: core::ptr::null(), flags: 0, rate: 1190400000 },
    hisi_fixed_rate_clock { id: HI6220_PLL_SYS_MEDIA, name: b"media_syspll\0".as_ptr(), parent: core::ptr::null(), flags: 0, rate: 1190400000 },
    hisi_fixed_rate_clock { id: HI6220_DDR_SRC, name: b"ddr_sel_src\0".as_ptr(), parent: core::ptr::null(), flags: 0, rate: 1200000000 },
    hisi_fixed_rate_clock { id: HI6220_PLL_MEDIA, name: b"media_pll\0".as_ptr(), parent: core::ptr::null(), flags: 0, rate: 1440000000 },
    hisi_fixed_rate_clock { id: HI6220_PLL_DDR, name: b"ddrpll0\0".as_ptr(), parent: core::ptr::null(), flags: 0, rate: 1600000000 },
];

static mut HI6220_FIXED_FACTOR_CLKS: [hisi_fixed_factor_clock; 10] = [
    hisi_fixed_factor_clock { id: HI6220_300M, name: b"clk_300m\0".as_ptr(), parent: b"syspll\0".as_ptr(), mult: 1, div: 4, flags: 0 },
    hisi_fixed_factor_clock { id: HI6220_150M, name: b"clk_150m\0".as_ptr(), parent: b"clk_300m\0".as_ptr(), mult: 1, div: 2, flags: 0 },
    hisi_fixed_factor_clock { id: HI6220_PICOPHY_SRC, name: b"picophy_src\0".as_ptr(), parent: b"clk_150m\0".as_ptr(), mult: 1, div: 4, flags: 0 },
    hisi_fixed_factor_clock { id: HI6220_MMC0_SRC_SEL, name: b"mmc0srcsel\0".as_ptr(), parent: b"mmc0_sel\0".as_ptr(), mult: 1, div: 8, flags: 0 },
    hisi_fixed_factor_clock { id: HI6220_MMC1_SRC_SEL, name: b"mmc1srcsel\0".as_ptr(), parent: b"mmc1_sel\0".as_ptr(), mult: 1, div: 8, flags: 0 },
    hisi_fixed_factor_clock { id: HI6220_MMC2_SRC_SEL, name: b"mmc2srcsel\0".as_ptr(), parent: b"mmc2_sel\0".as_ptr(), mult: 1, div: 8, flags: 0 },
    hisi_fixed_factor_clock { id: HI6220_VPU_CODEC, name: b"vpucodec\0".as_ptr(), parent: b"codec_jpeg_aclk\0".as_ptr(), mult: 1, div: 2, flags: 0 },
    hisi_fixed_factor_clock { id: HI6220_MMC0_SMP, name: b"mmc0_sample\0".as_ptr(), parent: b"mmc0_sel\0".as_ptr(), mult: 1, div: 8, flags: 0 },
    hisi_fixed_factor_clock { id: HI6220_MMC1_SMP, name: b"mmc1_sample\0".as_ptr(), parent: b"mmc1_sel\0".as_ptr(), mult: 1, div: 8, flags: 0 },
    hisi_fixed_factor_clock { id: HI6220_MMC2_SMP, name: b"mmc2_sample\0".as_ptr(), parent: b"mmc2_sel\0".as_ptr(), mult: 1, div: 8, flags: 0 },
];

unsafe fn hi6220_clk_ao_init(np: *mut device_node) {
    let clk_data_ao = hisi_clk_init(np, HI6220_AO_NR_CLKS);
    if clk_data_ao.is_null() { return; }
    hisi_clk_register_fixed_rate(HI6220_FIXED_RATE_CLKS.as_ptr(), HI6220_FIXED_RATE_CLKS.len(), clk_data_ao);
    hisi_clk_register_fixed_factor(HI6220_FIXED_FACTOR_CLKS.as_ptr(), HI6220_FIXED_FACTOR_CLKS.len(), clk_data_ao);
    hisi_clk_register_gate_sep(HI6220_SEPARATED_GATE_CLKS_AO.as_ptr(), HI6220_SEPARATED_GATE_CLKS_AO.len(), clk_data_ao);
}

// The remaining descriptor tables retain the exact C table layout and values.
// Parent-name arrays are represented as NUL-terminated byte-string pointers.
static mut HI6220_SEPARATED_GATE_CLKS_AO: [hisi_gate_clock; 15] = [
    hisi_gate_clock { id: HI6220_WDT0_PCLK, name: b"wdt0_pclk\0".as_ptr(), parent: b"ref32k\0".as_ptr(), flags: CLK_SET_RATE_PARENT|CLK_IGNORE_UNUSED, reg: 0x630, bit: 12, reserved: 0 },
    hisi_gate_clock { id: HI6220_WDT1_PCLK, name: b"wdt1_pclk\0".as_ptr(), parent: b"ref32k\0".as_ptr(), flags: CLK_SET_RATE_PARENT|CLK_IGNORE_UNUSED, reg: 0x630, bit: 13, reserved: 0 },
    hisi_gate_clock { id: HI6220_WDT2_PCLK, name: b"wdt2_pclk\0".as_ptr(), parent: b"ref32k\0".as_ptr(), flags: CLK_SET_RATE_PARENT|CLK_IGNORE_UNUSED, reg: 0x630, bit: 14, reserved: 0 },
    hisi_gate_clock { id: HI6220_TIMER0_PCLK, name: b"timer0_pclk\0".as_ptr(), parent: b"clk_tcxo\0".as_ptr(), flags: CLK_SET_RATE_PARENT|CLK_IGNORE_UNUSED, reg: 0x630, bit: 15, reserved: 0 },
    hisi_gate_clock { id: HI6220_TIMER1_PCLK, name: b"timer1_pclk\0".as_ptr(), parent: b"clk_tcxo\0".as_ptr(), flags: CLK_SET_RATE_PARENT|CLK_IGNORE_UNUSED, reg: 0x630, bit: 16, reserved: 0 },
    hisi_gate_clock { id: HI6220_TIMER2_PCLK, name: b"timer2_pclk\0".as_ptr(), parent: b"clk_tcxo\0".as_ptr(), flags: CLK_SET_RATE_PARENT|CLK_IGNORE_UNUSED, reg: 0x630, bit: 17, reserved: 0 },
    hisi_gate_clock { id: HI6220_TIMER3_PCLK, name: b"timer3_pclk\0".as_ptr(), parent: b"clk_tcxo\0".as_ptr(), flags: CLK_SET_RATE_PARENT|CLK_IGNORE_UNUSED, reg: 0x630, bit: 18, reserved: 0 },
    hisi_gate_clock { id: HI6220_TIMER4_PCLK, name: b"timer4_pclk\0".as_ptr(), parent: b"clk_tcxo\0".as_ptr(), flags: CLK_SET_RATE_PARENT|CLK_IGNORE_UNUSED, reg: 0x630, bit: 19, reserved: 0 },
    hisi_gate_clock { id: HI6220_TIMER5_PCLK, name: b"timer5_pclk\0".as_ptr(), parent: b"clk_tcxo\0".as_ptr(), flags: CLK_SET_RATE_PARENT|CLK_IGNORE_UNUSED, reg: 0x630, bit: 20, reserved: 0 },
    hisi_gate_clock { id: HI6220_TIMER6_PCLK, name: b"timer6_pclk\0".as_ptr(), parent: b"clk_tcxo\0".as_ptr(), flags: CLK_SET_RATE_PARENT|CLK_IGNORE_UNUSED, reg: 0x630, bit: 21, reserved: 0 },
    hisi_gate_clock { id: HI6220_TIMER7_PCLK, name: b"timer7_pclk\0".as_ptr(), parent: b"clk_tcxo\0".as_ptr(), flags: CLK_SET_RATE_PARENT|CLK_IGNORE_UNUSED, reg: 0x630, bit: 22, reserved: 0 },
    hisi_gate_clock { id: HI6220_TIMER8_PCLK, name: b"timer8_pclk\0".as_ptr(), parent: b"clk_tcxo\0".as_ptr(), flags: CLK_SET_RATE_PARENT|CLK_IGNORE_UNUSED, reg: 0x630, bit: 23, reserved: 0 },
    hisi_gate_clock { id: HI6220_UART0_PCLK, name: b"uart0_pclk\0".as_ptr(), parent: b"clk_tcxo\0".as_ptr(), flags: CLK_SET_RATE_PARENT|CLK_IGNORE_UNUSED, reg: 0x630, bit: 24, reserved: 0 },
    hisi_gate_clock { id: HI6220_RTC0_PCLK, name: b"rtc0_pclk\0".as_ptr(), parent: b"clk_tcxo\0".as_ptr(), flags: CLK_SET_RATE_PARENT|CLK_IGNORE_UNUSED, reg: 0x630, bit: 25, reserved: 0 },
    hisi_gate_clock { id: HI6220_RTC1_PCLK, name: b"rtc1_pclk\0".as_ptr(), parent: b"clk_tcxo\0".as_ptr(), flags: CLK_SET_RATE_PARENT|CLK_IGNORE_UNUSED, reg: 0x630, bit: 26, reserved: 0 },
];

extern "C" {
    static HI6220_SEPARATED_GATE_CLKS_SYS: [hisi_gate_clock; 37];
    static HI6220_MUX_CLKS_SYS: [hisi_mux_clock; 16];
    static HI6220_DIV_CLKS_SYS: [hi6220_divider_clock; 8];
    static HI6220_SEPARATED_GATE_CLKS_MEDIA: [hisi_gate_clock; 13];
    static HI6220_MUX_CLKS_MEDIA: [hisi_mux_clock; 3];
    static HI6220_DIV_CLKS_MEDIA: [hi6220_divider_clock; 7];
    static HI6220_GATE_CLKS_POWER: [hisi_gate_clock; 5];
    static HI6220_DIV_CLKS_POWER: [hi6220_divider_clock; 2];
    static HI6220_ACPU_SC_GATE_SEP_CLKS: [hisi_gate_clock; 1];
}

// The source's sysctrl, media, power, and ACPU tables and registration calls.
// Their external descriptor types and constants are intentionally unresolved.
unsafe fn hi6220_clk_sys_init(np: *mut device_node) { let clk_data = hisi_clk_init(np, HI6220_SYS_NR_CLKS); if clk_data.is_null() { return; } hisi_clk_register_gate_sep(HI6220_SEPARATED_GATE_CLKS_SYS.as_ptr(), HI6220_SEPARATED_GATE_CLKS_SYS.len(), clk_data); hisi_clk_register_mux(HI6220_MUX_CLKS_SYS.as_ptr(), HI6220_MUX_CLKS_SYS.len(), clk_data); hi6220_clk_register_divider(HI6220_DIV_CLKS_SYS.as_ptr(), HI6220_DIV_CLKS_SYS.len(), clk_data); }
unsafe fn hi6220_clk_media_init(np: *mut device_node) { let clk_data = hisi_clk_init(np, HI6220_MEDIA_NR_CLKS); if clk_data.is_null() { return; } hisi_clk_register_gate_sep(HI6220_SEPARATED_GATE_CLKS_MEDIA.as_ptr(), HI6220_SEPARATED_GATE_CLKS_MEDIA.len(), clk_data); hisi_clk_register_mux(HI6220_MUX_CLKS_MEDIA.as_ptr(), HI6220_MUX_CLKS_MEDIA.len(), clk_data); hi6220_clk_register_divider(HI6220_DIV_CLKS_MEDIA.as_ptr(), HI6220_DIV_CLKS_MEDIA.len(), clk_data); }
unsafe fn hi6220_clk_power_init(np: *mut device_node) { let clk_data = hisi_clk_init(np, HI6220_POWER_NR_CLKS); if clk_data.is_null() { return; } hisi_clk_register_gate(HI6220_GATE_CLKS_POWER.as_ptr(), HI6220_GATE_CLKS_POWER.len(), clk_data); hi6220_clk_register_divider(HI6220_DIV_CLKS_POWER.as_ptr(), HI6220_DIV_CLKS_POWER.len(), clk_data); }
unsafe fn hi6220_clk_acpu_init(np: *mut device_node) { let nr = HI6220_ACPU_SC_GATE_SEP_CLKS.len(); let clk_data = hisi_clk_init(np, nr); if clk_data.is_null() { return; } hisi_clk_register_gate_sep(HI6220_ACPU_SC_GATE_SEP_CLKS.as_ptr(), nr, clk_data); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
