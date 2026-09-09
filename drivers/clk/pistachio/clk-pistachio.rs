// SPDX-License-Identifier: GPL-2.0-only
/*
 * Pistachio SoC clock controllers
 *
 * Copyright (C) 2014 Google, Inc.
 */

// External kernel declarations and build-time clock bindings are supplied by
// the surrounding translation unit.

static mut PISTACHIO_GATES: [pistachio_gate; 29] = [
    GATE!(CLK_MIPS, "mips", "mips_div", 0x104, 0),
    GATE!(CLK_AUDIO_IN, "audio_in", "audio_clk_in_gate", 0x104, 1),
    GATE!(CLK_AUDIO, "audio", "audio_div", 0x104, 2),
    GATE!(CLK_I2S, "i2s", "i2s_div", 0x104, 3),
    GATE!(CLK_SPDIF, "spdif", "spdif_div", 0x104, 4),
    GATE!(CLK_AUDIO_DAC, "audio_dac", "audio_dac_div", 0x104, 5),
    GATE!(CLK_RPU_V, "rpu_v", "rpu_v_div", 0x104, 6),
    GATE!(CLK_RPU_L, "rpu_l", "rpu_l_div", 0x104, 7),
    GATE!(CLK_RPU_SLEEP, "rpu_sleep", "rpu_sleep_div", 0x104, 8),
    GATE!(CLK_WIFI_PLL_GATE, "wifi_pll_gate", "wifi_pll_mux", 0x104, 9),
    GATE!(CLK_RPU_CORE, "rpu_core", "rpu_core_div", 0x104, 10),
    GATE!(CLK_WIFI_ADC, "wifi_adc", "wifi_div8_mux", 0x104, 11),
    GATE!(CLK_WIFI_DAC, "wifi_dac", "wifi_div4_mux", 0x104, 12),
    GATE!(CLK_USB_PHY, "usb_phy", "usb_phy_div", 0x104, 13),
    GATE!(CLK_ENET_IN, "enet_in", "enet_clk_in_gate", 0x104, 14),
    GATE!(CLK_ENET, "enet", "enet_div", 0x104, 15),
    GATE!(CLK_UART0, "uart0", "uart0_div", 0x104, 16),
    GATE!(CLK_UART1, "uart1", "uart1_div", 0x104, 17),
    GATE!(CLK_PERIPH_SYS, "periph_sys", "sys_internal_div", 0x104, 18),
    GATE!(CLK_SPI0, "spi0", "spi0_div", 0x104, 19),
    GATE!(CLK_SPI1, "spi1", "spi1_div", 0x104, 20),
    GATE!(CLK_EVENT_TIMER, "event_timer", "event_timer_div", 0x104, 21),
    GATE!(CLK_AUX_ADC_INTERNAL, "aux_adc_internal", "sys_internal_div", 0x104, 22),
    GATE!(CLK_AUX_ADC, "aux_adc", "aux_adc_div", 0x104, 23),
    GATE!(CLK_SD_HOST, "sd_host", "sd_host_div", 0x104, 24),
    GATE!(CLK_BT, "bt", "bt_div", 0x104, 25),
    GATE!(CLK_BT_DIV4, "bt_div4", "bt_div4_div", 0x104, 26),
    GATE!(CLK_BT_DIV8, "bt_div8", "bt_div8_div", 0x104, 27),
    GATE!(CLK_BT_1MHZ, "bt_1mhz", "bt_1mhz_div", 0x104, 28),
];

static mut PISTACHIO_FFS: [pistachio_fixed_factor; 2] = [
    FIXED_FACTOR!(CLK_WIFI_DIV4, "wifi_div4", "wifi_pll", 4),
    FIXED_FACTOR!(CLK_WIFI_DIV8, "wifi_div8", "wifi_pll", 8),
];

static mut PISTACHIO_DIVS: [pistachio_div; 30] = [
    DIV!(CLK_MIPS_INTERNAL_DIV, "mips_internal_div", "mips_pll_mux", 0x204, 2),
    DIV!(CLK_MIPS_DIV, "mips_div", "mips_internal_div", 0x208, 8),
    DIV_F!(CLK_AUDIO_DIV, "audio_div", "audio_mux", 0x20c, 8, CLK_DIVIDER_ROUND_CLOSEST),
    DIV_F!(CLK_I2S_DIV, "i2s_div", "audio_pll_mux", 0x210, 8, CLK_DIVIDER_ROUND_CLOSEST),
    DIV_F!(CLK_SPDIF_DIV, "spdif_div", "audio_pll_mux", 0x214, 8, CLK_DIVIDER_ROUND_CLOSEST),
    DIV_F!(CLK_AUDIO_DAC_DIV, "audio_dac_div", "audio_pll_mux", 0x218, 8, CLK_DIVIDER_ROUND_CLOSEST),
    DIV!(CLK_RPU_V_DIV, "rpu_v_div", "rpu_v_pll_mux", 0x21c, 2),
    DIV!(CLK_RPU_L_DIV, "rpu_l_div", "rpu_l_mux", 0x220, 2),
    DIV!(CLK_RPU_SLEEP_DIV, "rpu_sleep_div", "xtal", 0x224, 10),
    DIV!(CLK_RPU_CORE_DIV, "rpu_core_div", "rpu_core_mux", 0x228, 3),
    DIV!(CLK_USB_PHY_DIV, "usb_phy_div", "sys_internal_div", 0x22c, 6),
    DIV!(CLK_ENET_DIV, "enet_div", "enet_mux", 0x230, 6),
    DIV_F!(CLK_UART0_INTERNAL_DIV, "uart0_internal_div", "sys_pll_mux", 0x234, 3, CLK_DIVIDER_ROUND_CLOSEST),
    DIV_F!(CLK_UART0_DIV, "uart0_div", "uart0_internal_div", 0x238, 10, CLK_DIVIDER_ROUND_CLOSEST),
    DIV_F!(CLK_UART1_INTERNAL_DIV, "uart1_internal_div", "sys_pll_mux", 0x23c, 3, CLK_DIVIDER_ROUND_CLOSEST),
    DIV_F!(CLK_UART1_DIV, "uart1_div", "uart1_internal_div", 0x240, 10, CLK_DIVIDER_ROUND_CLOSEST),
    DIV!(CLK_SYS_INTERNAL_DIV, "sys_internal_div", "sys_pll_mux", 0x244, 3),
    DIV!(CLK_SPI0_INTERNAL_DIV, "spi0_internal_div", "sys_pll_mux", 0x248, 3),
    DIV!(CLK_SPI0_DIV, "spi0_div", "spi0_internal_div", 0x24c, 7),
    DIV!(CLK_SPI1_INTERNAL_DIV, "spi1_internal_div", "sys_pll_mux", 0x250, 3),
    DIV!(CLK_SPI1_DIV, "spi1_div", "spi1_internal_div", 0x254, 7),
    DIV!(CLK_EVENT_TIMER_INTERNAL_DIV, "event_timer_internal_div", "event_timer_mux", 0x258, 3),
    DIV!(CLK_EVENT_TIMER_DIV, "event_timer_div", "event_timer_internal_div", 0x25c, 12),
    DIV!(CLK_AUX_ADC_INTERNAL_DIV, "aux_adc_internal_div", "aux_adc_internal", 0x260, 3),
    DIV!(CLK_AUX_ADC_DIV, "aux_adc_div", "aux_adc_internal_div", 0x264, 10),
    DIV!(CLK_SD_HOST_DIV, "sd_host_div", "sd_host_mux", 0x268, 6),
    DIV!(CLK_BT_DIV, "bt_div", "bt_pll_mux", 0x26c, 6),
    DIV!(CLK_BT_DIV4_DIV, "bt_div4_div", "bt_pll_mux", 0x270, 6),
    DIV!(CLK_BT_DIV8_DIV, "bt_div8_div", "bt_pll_mux", 0x274, 6),
    DIV!(CLK_BT_1MHZ_INTERNAL_DIV, "bt_1mhz_internal_div", "bt_pll_mux", 0x278, 3),
    DIV!(CLK_BT_1MHZ_DIV, "bt_1mhz_div", "bt_1mhz_internal_div", 0x27c, 10),
];

static MUX_XTAL_AUDIO_REFCLK: [&str; 2] = ["xtal", "audio_clk_in_gate"];
static MUX_XTAL_MIPS: [&str; 2] = ["xtal", "mips_pll"];
static MUX_XTAL_AUDIO: [&str; 3] = ["xtal", "audio_pll", "audio_in"];
static MUX_AUDIO_DEBUG: [&str; 2] = ["audio_pll_mux", "debug_mux"];
static MUX_XTAL_RPU_V: [&str; 2] = ["xtal", "rpu_v_pll"];
static MUX_XTAL_RPU_L: [&str; 2] = ["xtal", "rpu_l_pll"];
static MUX_RPU_L_MIPS: [&str; 2] = ["rpu_l_pll_mux", "mips_pll_mux"];
static MUX_XTAL_WIFI: [&str; 2] = ["xtal", "wifi_pll"];
static MUX_XTAL_WIFI_DIV4: [&str; 2] = ["xtal", "wifi_div4"];
static MUX_XTAL_WIFI_DIV8: [&str; 2] = ["xtal", "wifi_div8"];
static MUX_WIFI_DIV4_RPU_L: [&str; 3] = ["wifi_pll_gate", "wifi_div4_mux", "rpu_l_pll_mux"];
static MUX_XTAL_SYS: [&str; 2] = ["xtal", "sys_pll"];
static MUX_SYS_ENET: [&str; 2] = ["sys_internal_div", "enet_in"];
static MUX_AUDIO_SYS: [&str; 2] = ["audio_pll_mux", "sys_internal_div"];
static MUX_SYS_BT: [&str; 2] = ["sys_internal_div", "bt_pll_mux"];
static MUX_XTAL_BT: [&str; 2] = ["xtal", "bt_pll"];

static mut PISTACHIO_MUXES: [pistachio_mux; 16] = [
    MUX!(CLK_AUDIO_REF_MUX, "audio_refclk_mux", MUX_XTAL_AUDIO_REFCLK, 0x200, 0),
    MUX!(CLK_MIPS_PLL_MUX, "mips_pll_mux", MUX_XTAL_MIPS, 0x200, 1),
    MUX!(CLK_AUDIO_PLL_MUX, "audio_pll_mux", MUX_XTAL_AUDIO, 0x200, 2),
    MUX!(CLK_AUDIO_MUX, "audio_mux", MUX_AUDIO_DEBUG, 0x200, 4),
    MUX!(CLK_RPU_V_PLL_MUX, "rpu_v_pll_mux", MUX_XTAL_RPU_V, 0x200, 5),
    MUX!(CLK_RPU_L_PLL_MUX, "rpu_l_pll_mux", MUX_XTAL_RPU_L, 0x200, 6),
    MUX!(CLK_RPU_L_MUX, "rpu_l_mux", MUX_RPU_L_MIPS, 0x200, 7),
    MUX!(CLK_WIFI_PLL_MUX, "wifi_pll_mux", MUX_XTAL_WIFI, 0x200, 8),
    MUX!(CLK_WIFI_DIV4_MUX, "wifi_div4_mux", MUX_XTAL_WIFI_DIV4, 0x200, 9),
    MUX!(CLK_WIFI_DIV8_MUX, "wifi_div8_mux", MUX_XTAL_WIFI_DIV8, 0x200, 10),
    MUX!(CLK_RPU_CORE_MUX, "rpu_core_mux", MUX_WIFI_DIV4_RPU_L, 0x200, 11),
    MUX!(CLK_SYS_PLL_MUX, "sys_pll_mux", MUX_XTAL_SYS, 0x200, 13),
    MUX!(CLK_ENET_MUX, "enet_mux", MUX_SYS_ENET, 0x200, 14),
    MUX!(CLK_EVENT_TIMER_MUX, "event_timer_mux", MUX_AUDIO_SYS, 0x200, 15),
    MUX!(CLK_SD_HOST_MUX, "sd_host_mux", MUX_SYS_BT, 0x200, 16),
    MUX!(CLK_BT_PLL_MUX, "bt_pll_mux", MUX_XTAL_BT, 0x200, 17),
];

static mut PISTACHIO_PLLS: [pistachio_pll; 7] = [
    PLL_FIXED!(CLK_MIPS_PLL, "mips_pll", "xtal", PLL_GF40LP_LAINT, 0x0),
    PLL_FIXED!(CLK_AUDIO_PLL, "audio_pll", "audio_refclk_mux", PLL_GF40LP_FRAC, 0xc),
    PLL_FIXED!(CLK_RPU_V_PLL, "rpu_v_pll", "xtal", PLL_GF40LP_LAINT, 0x20),
    PLL_FIXED!(CLK_RPU_L_PLL, "rpu_l_pll", "xtal", PLL_GF40LP_LAINT, 0x2c),
    PLL_FIXED!(CLK_SYS_PLL, "sys_pll", "xtal", PLL_GF40LP_FRAC, 0x38),
    PLL_FIXED!(CLK_WIFI_PLL, "wifi_pll", "xtal", PLL_GF40LP_FRAC, 0x4c),
    PLL_FIXED!(CLK_BT_PLL, "bt_pll", "xtal", PLL_GF40LP_LAINT, 0x60),
];

static MUX_DEBUG: [&str; 6] = ["mips_pll_mux", "rpu_v_pll_mux", "rpu_l_pll_mux", "sys_pll_mux", "wifi_pll_mux", "bt_pll_mux"];
static MUX_DEBUG_IDX: [u32; 6] = [0x0, 0x1, 0x2, 0x4, 0x8, 0x10];
static mut PISTACHIO_CRITICAL_CLKS_CORE: [u32; 1] = [CLK_MIPS];
static mut PISTACHIO_CRITICAL_CLKS_SYS: [u32; 4] = [PERIPH_CLK_SYS, PERIPH_CLK_SYS_BUS, PERIPH_CLK_DDR, PERIPH_CLK_ROM];

static mut PISTACHIO_PERIPH_GATES: [pistachio_gate; 14] = [
    GATE!(PERIPH_CLK_SYS, "sys", "periph_sys", 0x100, 0),
    GATE!(PERIPH_CLK_SYS_BUS, "bus_sys", "periph_sys", 0x100, 1),
    GATE!(PERIPH_CLK_DDR, "ddr", "periph_sys", 0x100, 2),
    GATE!(PERIPH_CLK_ROM, "rom", "rom_div", 0x100, 3),
    GATE!(PERIPH_CLK_COUNTER_FAST, "counter_fast", "counter_fast_div", 0x100, 4),
    GATE!(PERIPH_CLK_COUNTER_SLOW, "counter_slow", "counter_slow_div", 0x100, 5),
    GATE!(PERIPH_CLK_IR, "ir", "ir_div", 0x100, 6),
    GATE!(PERIPH_CLK_WD, "wd", "wd_div", 0x100, 7),
    GATE!(PERIPH_CLK_PDM, "pdm", "pdm_div", 0x100, 8),
    GATE!(PERIPH_CLK_PWM, "pwm", "pwm_div", 0x100, 9),
    GATE!(PERIPH_CLK_I2C0, "i2c0", "i2c0_div", 0x100, 10),
    GATE!(PERIPH_CLK_I2C1, "i2c1", "i2c1_div", 0x100, 11),
    GATE!(PERIPH_CLK_I2C2, "i2c2", "i2c2_div", 0x100, 12),
    GATE!(PERIPH_CLK_I2C3, "i2c3", "i2c3_div", 0x100, 13),
];

static mut PISTACHIO_PERIPH_DIVS: [pistachio_div; 24] = [
    DIV!(PERIPH_CLK_ROM_DIV, "rom_div", "periph_sys", 0x10c, 7),
    DIV!(PERIPH_CLK_COUNTER_FAST_DIV, "counter_fast_div", "periph_sys", 0x110, 7),
    DIV!(PERIPH_CLK_COUNTER_SLOW_PRE_DIV, "counter_slow_pre_div", "periph_sys", 0x114, 7),
    DIV!(PERIPH_CLK_COUNTER_SLOW_DIV, "counter_slow_div", "counter_slow_pre_div", 0x118, 7),
    DIV_F!(PERIPH_CLK_IR_PRE_DIV, "ir_pre_div", "periph_sys", 0x11c, 7, CLK_DIVIDER_ROUND_CLOSEST),
    DIV_F!(PERIPH_CLK_IR_DIV, "ir_div", "ir_pre_div", 0x120, 7, CLK_DIVIDER_ROUND_CLOSEST),
    DIV_F!(PERIPH_CLK_WD_PRE_DIV, "wd_pre_div", "periph_sys", 0x124, 7, CLK_DIVIDER_ROUND_CLOSEST),
    DIV_F!(PERIPH_CLK_WD_DIV, "wd_div", "wd_pre_div", 0x128, 7, CLK_DIVIDER_ROUND_CLOSEST),
    DIV!(PERIPH_CLK_PDM_PRE_DIV, "pdm_pre_div", "periph_sys", 0x12c, 7),
    DIV!(PERIPH_CLK_PDM_DIV, "pdm_div", "pdm_pre_div", 0x130, 7),
    DIV!(PERIPH_CLK_PWM_PRE_DIV, "pwm_pre_div", "periph_sys", 0x134, 7),
    DIV!(PERIPH_CLK_PWM_DIV, "pwm_div", "pwm_pre_div", 0x138, 7),
    DIV!(PERIPH_CLK_I2C0_PRE_DIV, "i2c0_pre_div", "periph_sys", 0x13c, 7),
    DIV!(PERIPH_CLK_I2C0_DIV, "i2c0_div", "i2c0_pre_div", 0x140, 7),
    DIV!(PERIPH_CLK_I2C1_PRE_DIV, "i2c1_pre_div", "periph_sys", 0x144, 7),
    DIV!(PERIPH_CLK_I2C1_DIV, "i2c1_div", "i2c1_pre_div", 0x148, 7),
    DIV!(PERIPH_CLK_I2C2_PRE_DIV, "i2c2_pre_div", "periph_sys", 0x14c, 7),
    DIV!(PERIPH_CLK_I2C2_DIV, "i2c2_div", "i2c2_pre_div", 0x150, 7),
    DIV!(PERIPH_CLK_I2C3_PRE_DIV, "i2c3_pre_div", "periph_sys", 0x154, 7),
    DIV!(PERIPH_CLK_I2C3_DIV, "i2c3_div", "i2c3_pre_div", 0x158, 7),
];

static mut PISTACHIO_SYS_GATES: [pistachio_gate; 22] = [
    GATE!(SYS_CLK_I2C0, "i2c0_sys", "sys", 0x8, 0), GATE!(SYS_CLK_I2C1, "i2c1_sys", "sys", 0x8, 1),
    GATE!(SYS_CLK_I2C2, "i2c2_sys", "sys", 0x8, 2), GATE!(SYS_CLK_I2C3, "i2c3_sys", "sys", 0x8, 3),
    GATE!(SYS_CLK_I2S_IN, "i2s_in_sys", "sys", 0x8, 4), GATE!(SYS_CLK_PAUD_OUT, "paud_out_sys", "sys", 0x8, 5),
    GATE!(SYS_CLK_SPDIF_OUT, "spdif_out_sys", "sys", 0x8, 6), GATE!(SYS_CLK_SPI0_MASTER, "spi0_master_sys", "sys", 0x8, 7),
    GATE!(SYS_CLK_SPI0_SLAVE, "spi0_slave_sys", "sys", 0x8, 8), GATE!(SYS_CLK_PWM, "pwm_sys", "sys", 0x8, 9),
    GATE!(SYS_CLK_UART0, "uart0_sys", "sys", 0x8, 10), GATE!(SYS_CLK_UART1, "uart1_sys", "sys", 0x8, 11),
    GATE!(SYS_CLK_SPI1, "spi1_sys", "sys", 0x8, 12), GATE!(SYS_CLK_MDC, "mdc_sys", "sys", 0x8, 13),
    GATE!(SYS_CLK_SD_HOST, "sd_host_sys", "sys", 0x8, 14), GATE!(SYS_CLK_ENET, "enet_sys", "sys", 0x8, 15),
    GATE!(SYS_CLK_IR, "ir_sys", "sys", 0x8, 16), GATE!(SYS_CLK_WD, "wd_sys", "sys", 0x8, 17),
    GATE!(SYS_CLK_TIMER, "timer_sys", "sys", 0x8, 18), GATE!(SYS_CLK_I2S_OUT, "i2s_out_sys", "sys", 0x8, 24),
    GATE!(SYS_CLK_SPDIF_IN, "spdif_in_sys", "sys", 0x8, 25), GATE!(SYS_CLK_EVENT_TIMER, "event_timer_sys", "sys", 0x8, 26),
    GATE!(SYS_CLK_HASH, "hash_sys", "sys", 0x8, 27),
];

static mut PISTACHIO_EXT_GATES: [pistachio_gate; 2] = [
    GATE!(EXT_CLK_ENET_IN, "enet_clk_in_gate", "enet_clk_in", 0x58, 5),
    GATE!(EXT_CLK_AUDIO_IN, "audio_clk_in_gate", "audio_clk_in", 0x58, 8),
];

unsafe fn pistachio_clk_init(np: *mut device_node) {
    let p = pistachio_clk_alloc_provider(np, CLK_NR_CLKS);
    if p.is_null() { return; }
    pistachio_clk_register_pll(p, PISTACHIO_PLLS.as_mut_ptr(), PISTACHIO_PLLS.len());
    pistachio_clk_register_mux(p, PISTACHIO_MUXES.as_mut_ptr(), PISTACHIO_MUXES.len());
    pistachio_clk_register_div(p, PISTACHIO_DIVS.as_mut_ptr(), PISTACHIO_DIVS.len());
    pistachio_clk_register_fixed_factor(p, PISTACHIO_FFS.as_mut_ptr(), PISTACHIO_FFS.len());
    pistachio_clk_register_gate(p, PISTACHIO_GATES.as_mut_ptr(), PISTACHIO_GATES.len());
    let debug_clk = clk_register_mux_table(core::ptr::null_mut(), "debug_mux", MUX_DEBUG.as_ptr(), MUX_DEBUG.len(), CLK_SET_RATE_NO_REPARENT, (*p).base.add(0x200), 18, 0x1f, 0, MUX_DEBUG_IDX.as_ptr(), core::ptr::null_mut());
    (*p).clk_data.clks[CLK_DEBUG_MUX] = debug_clk;
    pistachio_clk_register_provider(p);
    pistachio_clk_force_enable(p, PISTACHIO_CRITICAL_CLKS_CORE.as_ptr(), PISTACHIO_CRITICAL_CLKS_CORE.len());
}

unsafe fn pistachio_clk_periph_init(np: *mut device_node) {
    let p = pistachio_clk_alloc_provider(np, PERIPH_CLK_NR_CLKS);
    if p.is_null() { return; }
    pistachio_clk_register_div(p, PISTACHIO_PERIPH_DIVS.as_mut_ptr(), PISTACHIO_PERIPH_DIVS.len());
    pistachio_clk_register_gate(p, PISTACHIO_PERIPH_GATES.as_mut_ptr(), PISTACHIO_PERIPH_GATES.len());
    pistachio_clk_register_provider(p);
    pistachio_clk_force_enable(p, PISTACHIO_CRITICAL_CLKS_SYS.as_ptr(), PISTACHIO_CRITICAL_CLKS_SYS.len());
}

unsafe fn pistachio_cr_periph_init(np: *mut device_node) {
    let p = pistachio_clk_alloc_provider(np, SYS_CLK_NR_CLKS);
    if p.is_null() { return; }
    pistachio_clk_register_gate(p, PISTACHIO_SYS_GATES.as_mut_ptr(), PISTACHIO_SYS_GATES.len());
    pistachio_clk_register_provider(p);
}

unsafe fn pistachio_cr_top_init(np: *mut device_node) {
    let p = pistachio_clk_alloc_provider(np, EXT_CLK_NR_CLKS);
    if p.is_null() { return; }
    pistachio_clk_register_gate(p, PISTACHIO_EXT_GATES.as_mut_ptr(), PISTACHIO_EXT_GATES.len());
    pistachio_clk_register_provider(p);
}

// CLK_OF_DECLARE(pistachio_clk, "img,pistachio-clk", pistachio_clk_init);
// CLK_OF_DECLARE(pistachio_clk_periph, "img,pistachio-clk-periph", pistachio_clk_periph_init);
// CLK_OF_DECLARE(pistachio_cr_periph, "img,pistachio-cr-periph", pistachio_cr_periph_init);
// CLK_OF_DECLARE(pistachio_cr_top, "img,pistachio-cr-top", pistachio_cr_top_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
