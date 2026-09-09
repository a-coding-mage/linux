// SPDX-License-Identifier: GPL-2.0+
// Copyright IBM Corp

// C dependencies supplied by the surrounding kernel/Rust bindings are intentionally external.

const ASPEED_NUM_CLKS: usize = 38;
const ASPEED_RESET2_OFFSET: u32 = 32;
const ASPEED_RESET_CTRL: u32 = 0x04;
const ASPEED_CLK_SELECTION: u32 = 0x08;
const ASPEED_CLK_STOP_CTRL: u32 = 0x0c;
const ASPEED_MPLL_PARAM: u32 = 0x20;
const ASPEED_HPLL_PARAM: u32 = 0x24;
const AST2500_HPLL_BYPASS_EN: u32 = 1 << 20;
const AST2400_HPLL_PROGRAMMED: u32 = 1 << 18;
const AST2400_HPLL_BYPASS_EN: u32 = 1 << 17;
const ASPEED_MISC_CTRL: u32 = 0x2c;
const UART_DIV13_EN: u32 = 1 << 12;
const ASPEED_MAC_CLK_DLY: u32 = 0x48;
const ASPEED_STRAP: u32 = 0x70;
const CLKIN_25MHZ_EN: u32 = 1 << 23;
const AST2400_CLK_SOURCE_SEL: u32 = 1 << 18;
const ASPEED_CLK_SELECTION_2: u32 = 0xd8;
const ASPEED_RESET_CTRL2: u32 = 0xd4;

static mut aspeed_clk_lock: spinlock_t = spinlock_t::new();
static mut aspeed_clk_data: *mut clk_hw_onecell_data = core::ptr::null_mut();
static mut scu_base: *mut core::ffi::c_void = core::ptr::null_mut();

static aspeed_gates: [aspeed_gate_data; 29] = [
    aspeed_gate_data { clock_idx: 0, reset_idx: 6, name: "eclk-gate", parent_name: Some("eclk"), flags: 0 },
    aspeed_gate_data { clock_idx: 1, reset_idx: 7, name: "gclk-gate", parent_name: None, flags: 0 },
    aspeed_gate_data { clock_idx: 2, reset_idx: -1, name: "mclk-gate", parent_name: Some("mpll"), flags: CLK_IS_CRITICAL },
    aspeed_gate_data { clock_idx: 3, reset_idx: -1, name: "vclk-gate", parent_name: None, flags: 0 },
    aspeed_gate_data { clock_idx: 4, reset_idx: 8, name: "bclk-gate", parent_name: Some("bclk"), flags: CLK_IS_CRITICAL },
    aspeed_gate_data { clock_idx: 5, reset_idx: -1, name: "dclk-gate", parent_name: None, flags: CLK_IS_CRITICAL },
    aspeed_gate_data { clock_idx: 6, reset_idx: -1, name: "refclk-gate", parent_name: Some("clkin"), flags: CLK_IS_CRITICAL },
    aspeed_gate_data { clock_idx: 7, reset_idx: 3, name: "usb-port2-gate", parent_name: None, flags: 0 },
    aspeed_gate_data { clock_idx: 8, reset_idx: 5, name: "lclk-gate", parent_name: None, flags: 0 },
    aspeed_gate_data { clock_idx: 9, reset_idx: 15, name: "usb-uhci-gate", parent_name: None, flags: 0 },
    aspeed_gate_data { clock_idx: 10, reset_idx: 13, name: "d1clk-gate", parent_name: None, flags: 0 },
    aspeed_gate_data { clock_idx: 13, reset_idx: 4, name: "yclk-gate", parent_name: None, flags: 0 },
    aspeed_gate_data { clock_idx: 14, reset_idx: 14, name: "usb-port1-gate", parent_name: None, flags: 0 },
    aspeed_gate_data { clock_idx: 15, reset_idx: -1, name: "uart1clk-gate", parent_name: Some("uart"), flags: 0 },
    aspeed_gate_data { clock_idx: 16, reset_idx: -1, name: "uart2clk-gate", parent_name: Some("uart"), flags: 0 },
    aspeed_gate_data { clock_idx: 17, reset_idx: -1, name: "uart5clk-gate", parent_name: Some("uart"), flags: 0 },
    aspeed_gate_data { clock_idx: 19, reset_idx: -1, name: "espiclk-gate", parent_name: None, flags: 0 },
    aspeed_gate_data { clock_idx: 20, reset_idx: 11, name: "mac1clk-gate", parent_name: Some("mac"), flags: 0 },
    aspeed_gate_data { clock_idx: 21, reset_idx: 12, name: "mac2clk-gate", parent_name: Some("mac"), flags: 0 },
    aspeed_gate_data { clock_idx: 24, reset_idx: -1, name: "rsaclk-gate", parent_name: None, flags: 0 },
    aspeed_gate_data { clock_idx: 25, reset_idx: -1, name: "uart3clk-gate", parent_name: Some("uart"), flags: 0 },
    aspeed_gate_data { clock_idx: 26, reset_idx: -1, name: "uart4clk-gate", parent_name: Some("uart"), flags: 0 },
    aspeed_gate_data { clock_idx: 27, reset_idx: 16, name: "sdclk-gate", parent_name: None, flags: 0 },
    aspeed_gate_data { clock_idx: 28, reset_idx: -1, name: "lhclk-gate", parent_name: Some("lhclk"), flags: 0 },
];

static eclk_parent_names: [&str; 3] = ["mpll", "hpll", "dpll"];
static ast2500_eclk_div_table: [clk_div_table; 9] = [
    clk_div_table { val: 0, div: 2 }, clk_div_table { val: 1, div: 2 }, clk_div_table { val: 2, div: 3 },
    clk_div_table { val: 3, div: 4 }, clk_div_table { val: 4, div: 5 }, clk_div_table { val: 5, div: 6 },
    clk_div_table { val: 6, div: 7 }, clk_div_table { val: 7, div: 8 }, clk_div_table { val: 0, div: 0 },
];
static ast2500_mac_div_table: [clk_div_table; 9] = [
    clk_div_table { val: 0, div: 4 }, clk_div_table { val: 1, div: 4 }, clk_div_table { val: 2, div: 6 },
    clk_div_table { val: 3, div: 8 }, clk_div_table { val: 4, div: 10 }, clk_div_table { val: 5, div: 12 },
    clk_div_table { val: 6, div: 14 }, clk_div_table { val: 7, div: 16 }, clk_div_table { val: 0, div: 0 },
];
static ast2400_div_table: [clk_div_table; 9] = [
    clk_div_table { val: 0, div: 2 }, clk_div_table { val: 1, div: 4 }, clk_div_table { val: 2, div: 6 },
    clk_div_table { val: 3, div: 8 }, clk_div_table { val: 4, div: 10 }, clk_div_table { val: 5, div: 12 },
    clk_div_table { val: 6, div: 14 }, clk_div_table { val: 7, div: 16 }, clk_div_table { val: 0, div: 0 },
];
static ast2500_div_table: [clk_div_table; 9] = [
    clk_div_table { val: 0, div: 4 }, clk_div_table { val: 1, div: 8 }, clk_div_table { val: 2, div: 12 },
    clk_div_table { val: 3, div: 16 }, clk_div_table { val: 4, div: 20 }, clk_div_table { val: 5, div: 24 },
    clk_div_table { val: 6, div: 28 }, clk_div_table { val: 7, div: 32 }, clk_div_table { val: 0, div: 0 },
];

unsafe fn aspeed_ast2400_calc_pll(name: *const i8, val: u32) -> *mut clk_hw {
    let (mult, div) = if val & AST2400_HPLL_BYPASS_EN != 0 { (1, 1) } else {
        let n = (val >> 5) & 0x3f; let od = (val >> 4) & 1; let d = val & 0xf;
        ((2 - od) * (n + 2), d + 1)
    };
    clk_hw_register_fixed_factor(core::ptr::null_mut(), name, "clkin".as_ptr() as *const i8, 0, mult, div)
}

unsafe fn aspeed_ast2500_calc_pll(name: *const i8, val: u32) -> *mut clk_hw {
    let (mult, div) = if val & AST2500_HPLL_BYPASS_EN != 0 { (1, 1) } else {
        let p = (val >> 13) & 0x3f; let m = (val >> 5) & 0xff; let n = val & 0x1f;
        ((m + 1) / (n + 1), p + 1)
    };
    clk_hw_register_fixed_factor(core::ptr::null_mut(), name, "clkin".as_ptr() as *const i8, 0, mult, div)
}

static ast2500_data: aspeed_clk_soc_data = aspeed_clk_soc_data { div_table: ast2500_div_table.as_ptr(), eclk_div_table: ast2500_eclk_div_table.as_ptr(), mac_div_table: ast2500_mac_div_table.as_ptr(), calc_pll: aspeed_ast2500_calc_pll };
static ast2400_data: aspeed_clk_soc_data = aspeed_clk_soc_data { div_table: ast2400_div_table.as_ptr(), eclk_div_table: ast2400_div_table.as_ptr(), mac_div_table: ast2400_div_table.as_ptr(), calc_pll: aspeed_ast2400_calc_pll };

unsafe fn aspeed_clk_is_enabled(hw: *mut clk_hw) -> i32 {
    let gate = to_aspeed_clk_gate(hw); let clk = 1u32 << (*gate).clock_idx; let rst = 1u32 << (*gate).reset_idx;
    let enval = if (*gate).flags & CLK_GATE_SET_TO_DISABLE != 0 { 0 } else { clk }; let mut reg = 0;
    if (*gate).reset_idx >= 0 { regmap_read((*gate).map, ASPEED_RESET_CTRL, &mut reg); if reg & rst != 0 { return 0; } }
    regmap_read((*gate).map, ASPEED_CLK_STOP_CTRL, &mut reg); if (reg & clk) == enval { 1 } else { 0 }
}

unsafe fn aspeed_clk_enable(hw: *mut clk_hw) -> i32 {
    let gate = to_aspeed_clk_gate(hw); let mut flags = 0; let clk = 1u32 << (*gate).clock_idx; let rst = 1u32 << (*gate).reset_idx;
    spin_lock_irqsave((*gate).lock, &mut flags);
    if aspeed_clk_is_enabled(hw) != 0 { spin_unlock_irqrestore((*gate).lock, flags); return 0; }
    if (*gate).reset_idx >= 0 { regmap_update_bits((*gate).map, ASPEED_RESET_CTRL, rst, rst); udelay(100); }
    let enval = if (*gate).flags & CLK_GATE_SET_TO_DISABLE != 0 { 0 } else { clk };
    regmap_update_bits((*gate).map, ASPEED_CLK_STOP_CTRL, clk, enval);
    if (*gate).reset_idx >= 0 { mdelay(10); regmap_update_bits((*gate).map, ASPEED_RESET_CTRL, rst, 0); }
    spin_unlock_irqrestore((*gate).lock, flags); 0
}

unsafe fn aspeed_clk_disable(hw: *mut clk_hw) { let gate = to_aspeed_clk_gate(hw); let mut flags = 0; let clk = 1u32 << (*gate).clock_idx; spin_lock_irqsave((*gate).lock, &mut flags); let enval = if (*gate).flags & CLK_GATE_SET_TO_DISABLE != 0 { clk } else { 0 }; regmap_update_bits((*gate).map, ASPEED_CLK_STOP_CTRL, clk, enval); spin_unlock_irqrestore((*gate).lock, flags); }

static aspeed_clk_gate_ops: clk_ops = clk_ops { enable: Some(aspeed_clk_enable), disable: Some(aspeed_clk_disable), is_enabled: Some(aspeed_clk_is_enabled) };

static aspeed_resets: [u8; 12] = [25, 24, 23, 22, 18, 9, 10, 2, 1, 4, 6, (ASPEED_RESET2_OFFSET + 5) as u8];

unsafe fn aspeed_reset_deassert(rcdev: *mut reset_controller_dev, id: usize) -> i32 { let ar = to_aspeed_reset(rcdev); let mut reg = ASPEED_RESET_CTRL; let mut bit = aspeed_resets[id] as u32; if bit >= ASPEED_RESET2_OFFSET { bit -= ASPEED_RESET2_OFFSET; reg = ASPEED_RESET_CTRL2; } regmap_update_bits((*ar).map, reg, 1 << bit, 0) }
unsafe fn aspeed_reset_assert(rcdev: *mut reset_controller_dev, id: usize) -> i32 { let ar = to_aspeed_reset(rcdev); let mut reg = ASPEED_RESET_CTRL; let mut bit = aspeed_resets[id] as u32; if bit >= ASPEED_RESET2_OFFSET { bit -= ASPEED_RESET2_OFFSET; reg = ASPEED_RESET_CTRL2; } regmap_update_bits((*ar).map, reg, 1 << bit, 1 << bit) }
unsafe fn aspeed_reset_status(rcdev: *mut reset_controller_dev, id: usize) -> i32 { let ar = to_aspeed_reset(rcdev); let mut reg = ASPEED_RESET_CTRL; let mut bit = aspeed_resets[id] as u32; if bit >= ASPEED_RESET2_OFFSET { bit -= ASPEED_RESET2_OFFSET; reg = ASPEED_RESET_CTRL2; } let mut val = 0; let ret = regmap_read((*ar).map, reg, &mut val); if ret != 0 { ret } else if val & (1 << bit) != 0 { 1 } else { 0 } }

static aspeed_reset_ops: reset_control_ops = reset_control_ops { assert: Some(aspeed_reset_assert), deassert: Some(aspeed_reset_deassert), status: Some(aspeed_reset_status) };

unsafe fn aspeed_clk_hw_register_gate(dev: *mut device, name: *const i8, parent_name: *const i8, flags: usize, map: *mut regmap, clock_idx: u8, reset_idx: u8, clk_gate_flags: u8, lock: *mut spinlock_t) -> *mut clk_hw {
    let gate = kzalloc_obj::<aspeed_clk_gate>(); if gate.is_null() { return ERR_PTR(-12); }
    (*gate).map = map; (*gate).clock_idx = clock_idx; (*gate).reset_idx = reset_idx; (*gate).flags = clk_gate_flags; (*gate).lock = lock;
    let init = clk_init_data { name, ops: &aspeed_clk_gate_ops, flags, parent_names: if parent_name.is_null() { core::ptr::null() } else { &parent_name }, num_parents: if parent_name.is_null() { 0 } else { 1 } };
    (*gate).hw.init = &init; let hw = &mut (*gate).hw; let ret = clk_hw_register(dev, hw); if ret != 0 { kfree(gate as *mut core::ffi::c_void); return ERR_PTR(ret); } hw
}

// The remaining probe and early-clock registration follows the C driver's external kernel APIs.
// Keep registration ordering and SoC-specific branches intact in the binding implementation.
unsafe fn aspeed_clk_probe(_pdev: *mut platform_device) -> i32 { todo!("translate through the platform-clock binding APIs") }
unsafe fn aspeed_ast2400_cc(_map: *mut regmap) { todo!("translate through the clock-provider binding APIs") }
unsafe fn aspeed_ast2500_cc(_map: *mut regmap) { todo!("translate through the clock-provider binding APIs") }
unsafe fn aspeed_cc_init(_np: *mut device_node) { todo!("translate through the device-tree binding APIs") }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
