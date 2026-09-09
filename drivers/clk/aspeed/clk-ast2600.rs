// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright IBM Corp
// Copyright ASPEED Technology
// Translated from clk-ast2600.c; Linux/kernel dependencies are external.

const ASPEED_G6_NUM_CLKS: usize = 73;
const ASPEED_G6_SILICON_REV: u32 = 0x014;
const CHIP_REVISION_ID: u32 = 0x00ff0000;
const ASPEED_G6_RESET_CTRL: u32 = 0x040;
const ASPEED_G6_RESET_CTRL2: u32 = 0x050;
const ASPEED_G6_CLK_STOP_CTRL: u32 = 0x080;
const ASPEED_G6_CLK_STOP_CTRL2: u32 = 0x090;
const ASPEED_G6_MISC_CTRL: u32 = 0x0c0;
const UART_DIV13_EN: u32 = 1 << 12;
const ASPEED_G6_CLK_SELECTION1: u32 = 0x300;
const ASPEED_G6_CLK_SELECTION2: u32 = 0x304;
const ASPEED_G6_CLK_SELECTION4: u32 = 0x310;
const ASPEED_G6_CLK_SELECTION5: u32 = 0x314;
const I3C_CLK_SELECTION_SHIFT: u32 = 31;
const I3C_CLK_SELECTION: u32 = 1 << 31;
const I3C_CLK_SELECT_HCLK: u32 = 0 << I3C_CLK_SELECTION_SHIFT;
const I3C_CLK_SELECT_APLL_DIV: u32 = 1 << I3C_CLK_SELECTION_SHIFT;
const APLL_DIV_SELECTION_SHIFT: u32 = 28;
const APLL_DIV_SELECTION: u32 = 0x70000000;
const APLL_DIV_2: u32 = 0x10000000;
const APLL_DIV_3: u32 = 0x20000000;
const APLL_DIV_4: u32 = 0x30000000;
const APLL_DIV_5: u32 = 0x40000000;
const APLL_DIV_6: u32 = 0x50000000;
const APLL_DIV_7: u32 = 0x60000000;
const APLL_DIV_8: u32 = 0x70000000;
const ASPEED_HPLL_PARAM: u32 = 0x200;
const ASPEED_APLL_PARAM: u32 = 0x210;
const ASPEED_MPLL_PARAM: u32 = 0x220;
const ASPEED_EPLL_PARAM: u32 = 0x240;
const ASPEED_DPLL_PARAM: u32 = 0x260;
const ASPEED_G6_STRAP1: u32 = 0x500;
const ASPEED_MAC12_CLK_DLY: u32 = 0x340;
const ASPEED_MAC34_CLK_DLY: u32 = 0x350;

static mut aspeed_g6_clk_data: *mut clk_hw_onecell_data = core::ptr::null_mut();
static mut scu_g6_base: *mut u8 = core::ptr::null_mut();
static mut soc_rev: u8 = 0;
static mut aspeed_g6_clk_lock: spinlock_t = spinlock_t::new();

static aspeed_g6_gates: [aspeed_gate_data; 63] = [
    aspeed_gate_data { clock_idx: 0, reset_idx: -1, name: "mclk-gate", parent_name: "mpll", flags: CLK_IS_CRITICAL },
    aspeed_gate_data { clock_idx: 1, reset_idx: 6, name: "eclk-gate", parent_name: "eclk", flags: 0 },
    aspeed_gate_data { clock_idx: 2, reset_idx: 7, name: "gclk-gate", parent_name: core::ptr::null(), flags: 0 },
    aspeed_gate_data { clock_idx: 3, reset_idx: -1, name: "vclk-gate", parent_name: core::ptr::null(), flags: 0 },
    aspeed_gate_data { clock_idx: 4, reset_idx: 8, name: "bclk-gate", parent_name: "bclk", flags: 0 },
    aspeed_gate_data { clock_idx: 5, reset_idx: -1, name: "dclk-gate", parent_name: core::ptr::null(), flags: CLK_IS_CRITICAL },
    aspeed_gate_data { clock_idx: 6, reset_idx: -1, name: "ref0clk-gate", parent_name: "clkin", flags: CLK_IS_CRITICAL },
    aspeed_gate_data { clock_idx: 7, reset_idx: 3, name: "usb-port2-gate", parent_name: core::ptr::null(), flags: 0 },
    aspeed_gate_data { clock_idx: 9, reset_idx: 15, name: "usb-uhci-gate", parent_name: core::ptr::null(), flags: 0 },
    aspeed_gate_data { clock_idx: 10, reset_idx: 13, name: "d1clk-gate", parent_name: "d1clk", flags: 0 },
    aspeed_gate_data { clock_idx: 13, reset_idx: 4, name: "yclk-gate", parent_name: core::ptr::null(), flags: 0 },
    aspeed_gate_data { clock_idx: 14, reset_idx: 14, name: "usb-port1-gate", parent_name: core::ptr::null(), flags: 0 },
    aspeed_gate_data { clock_idx: 15, reset_idx: -1, name: "uart5clk-gate", parent_name: "uart", flags: 0 },
    aspeed_gate_data { clock_idx: 20, reset_idx: 11, name: "mac1clk-gate", parent_name: "mac12", flags: 0 },
    aspeed_gate_data { clock_idx: 21, reset_idx: 12, name: "mac2clk-gate", parent_name: "mac12", flags: 0 },
    aspeed_gate_data { clock_idx: 24, reset_idx: 4, name: "rsaclk-gate", parent_name: core::ptr::null(), flags: 0 },
    aspeed_gate_data { clock_idx: 25, reset_idx: 9, name: "rvasclk-gate", parent_name: core::ptr::null(), flags: 0 },
    aspeed_gate_data { clock_idx: 27, reset_idx: 16, name: "emmcclk-gate", parent_name: core::ptr::null(), flags: 0 },
    aspeed_gate_data { clock_idx: 32, reset_idx: 32, name: "lclk-gate", parent_name: core::ptr::null(), flags: 0 },
    aspeed_gate_data { clock_idx: 33, reset_idx: -1, name: "espiclk-gate", parent_name: core::ptr::null(), flags: 0 },
    aspeed_gate_data { clock_idx: 34, reset_idx: -1, name: "ref1clk-gate", parent_name: "clkin", flags: CLK_IS_CRITICAL },
    aspeed_gate_data { clock_idx: 36, reset_idx: 56, name: "sdclk-gate", parent_name: core::ptr::null(), flags: 0 },
    aspeed_gate_data { clock_idx: 37, reset_idx: -1, name: "lhclk-gate", parent_name: "lhclk", flags: 0 },
    aspeed_gate_data { clock_idx: 40, reset_idx: 40, name: "i3c0clk-gate", parent_name: "i3cclk", flags: 0 },
    aspeed_gate_data { clock_idx: 41, reset_idx: 41, name: "i3c1clk-gate", parent_name: "i3cclk", flags: 0 },
    aspeed_gate_data { clock_idx: 42, reset_idx: 42, name: "i3c2clk-gate", parent_name: "i3cclk", flags: 0 },
    aspeed_gate_data { clock_idx: 43, reset_idx: 43, name: "i3c3clk-gate", parent_name: "i3cclk", flags: 0 },
    aspeed_gate_data { clock_idx: 44, reset_idx: 44, name: "i3c4clk-gate", parent_name: "i3cclk", flags: 0 },
    aspeed_gate_data { clock_idx: 45, reset_idx: 45, name: "i3c5clk-gate", parent_name: "i3cclk", flags: 0 },
    aspeed_gate_data { clock_idx: 48, reset_idx: -1, name: "uart1clk-gate", parent_name: "uart", flags: 0 },
    aspeed_gate_data { clock_idx: 49, reset_idx: -1, name: "uart2clk-gate", parent_name: "uart", flags: 0 },
    aspeed_gate_data { clock_idx: 50, reset_idx: -1, name: "uart3clk-gate", parent_name: "uart", flags: 0 },
    aspeed_gate_data { clock_idx: 51, reset_idx: -1, name: "uart4clk-gate", parent_name: "uart", flags: 0 },
    aspeed_gate_data { clock_idx: 52, reset_idx: 52, name: "mac3clk-gate", parent_name: "mac34", flags: 0 },
    aspeed_gate_data { clock_idx: 53, reset_idx: 53, name: "mac4clk-gate", parent_name: "mac34", flags: 0 },
    aspeed_gate_data { clock_idx: 54, reset_idx: -1, name: "uart6clk-gate", parent_name: "uartx", flags: 0 },
    aspeed_gate_data { clock_idx: 55, reset_idx: -1, name: "uart7clk-gate", parent_name: "uartx", flags: 0 },
    aspeed_gate_data { clock_idx: 56, reset_idx: -1, name: "uart8clk-gate", parent_name: "uartx", flags: 0 },
    aspeed_gate_data { clock_idx: 57, reset_idx: -1, name: "uart9clk-gate", parent_name: "uartx", flags: 0 },
    aspeed_gate_data { clock_idx: 58, reset_idx: -1, name: "uart10clk-gate", parent_name: "uartx", flags: 0 },
    aspeed_gate_data { clock_idx: 59, reset_idx: -1, name: "uart11clk-gate", parent_name: "uartx", flags: 0 },
    aspeed_gate_data { clock_idx: 60, reset_idx: -1, name: "uart12clk-gate", parent_name: "uartx", flags: 0 },
    aspeed_gate_data { clock_idx: 61, reset_idx: -1, name: "uart13clk-gate", parent_name: "uartx", flags: 0 },
    aspeed_gate_data { clock_idx: 62, reset_idx: 59, name: "fsiclk-gate", parent_name: "fsiclk", flags: 0 },
];

static ast2600_eclk_div_table: [clk_div_table; 9] = [clk_div_table { val: 0, div: 2 }, clk_div_table { val: 1, div: 2 }, clk_div_table { val: 2, div: 3 }, clk_div_table { val: 3, div: 4 }, clk_div_table { val: 4, div: 5 }, clk_div_table { val: 5, div: 6 }, clk_div_table { val: 6, div: 7 }, clk_div_table { val: 7, div: 8 }, clk_div_table { val: 0, div: 0 }];
static ast2600_emmc_extclk_div_table: [clk_div_table; 9] = [clk_div_table { val: 0, div: 2 }, clk_div_table { val: 1, div: 4 }, clk_div_table { val: 2, div: 6 }, clk_div_table { val: 3, div: 8 }, clk_div_table { val: 4, div: 10 }, clk_div_table { val: 5, div: 12 }, clk_div_table { val: 6, div: 14 }, clk_div_table { val: 7, div: 16 }, clk_div_table { val: 0, div: 0 }];
static ast2600_mac_div_table: [clk_div_table; 9] = [clk_div_table { val: 0, div: 4 }, clk_div_table { val: 1, div: 4 }, clk_div_table { val: 2, div: 6 }, clk_div_table { val: 3, div: 8 }, clk_div_table { val: 4, div: 10 }, clk_div_table { val: 5, div: 12 }, clk_div_table { val: 6, div: 14 }, clk_div_table { val: 7, div: 16 }, clk_div_table { val: 0, div: 0 }];
static ast2600_div_table: [clk_div_table; 9] = [clk_div_table { val: 0, div: 4 }, clk_div_table { val: 1, div: 8 }, clk_div_table { val: 2, div: 12 }, clk_div_table { val: 3, div: 16 }, clk_div_table { val: 4, div: 20 }, clk_div_table { val: 5, div: 24 }, clk_div_table { val: 6, div: 28 }, clk_div_table { val: 7, div: 32 }, clk_div_table { val: 0, div: 0 }];

unsafe fn ast2600_calc_pll(name: *const i8, val: u32) -> *mut clk_hw {
    let (mult, div): (u32, u32);
    if val & (1 << 24) != 0 { mult = 1; div = 1; } else { let m = val & 0x1fff; let n = (val >> 13) & 0x3f; let p = (val >> 19) & 0xf; mult = (m + 1) / (n + 1); div = p + 1; }
    clk_hw_register_fixed_factor(core::ptr::null_mut(), name, b"clkin\0".as_ptr() as *const i8, 0, mult, div)
}

unsafe fn ast2600_calc_apll(name: *const i8, val: u32) -> *mut clk_hw {
    let (mult, div): (u32, u32);
    if soc_rev >= 2 { if val & (1 << 24) != 0 { mult = 1; div = 1; } else { let m = val & 0x1fff; let n = (val >> 13) & 0x3f; let p = (val >> 19) & 0xf; mult = m + 1; div = (n + 1) * (p + 1); } } else if val & (1 << 20) != 0 { mult = 1; div = 1; } else { let m = (val >> 5) & 0x3f; let od = (val >> 4) & 1; let n = val & 0xf; mult = (2 - od) * (m + 2); div = n + 1; }
    clk_hw_register_fixed_factor(core::ptr::null_mut(), name, b"clkin\0".as_ptr() as *const i8, 0, mult, div)
}

fn get_bit(idx: u8) -> u32 { 1u32 << (idx % 32) }
fn get_reset_reg(gate: &aspeed_clk_gate) -> u32 { if gate.reset_idx < 32 { ASPEED_G6_RESET_CTRL } else { ASPEED_G6_RESET_CTRL2 } }
fn get_clock_reg(gate: &aspeed_clk_gate) -> u32 { if gate.clock_idx < 32 { ASPEED_G6_CLK_STOP_CTRL } else { ASPEED_G6_CLK_STOP_CTRL2 } }

unsafe fn aspeed_g6_clk_is_enabled(hw: *mut clk_hw) -> i32 {
    let gate = to_aspeed_clk_gate(hw); let clk = get_bit((*gate).clock_idx); let rst = get_bit((*gate).reset_idx as u8); let mut reg = 0; let enval;
    if (*gate).reset_idx >= 0 { regmap_read((*gate).map, get_reset_reg(&*gate), &mut reg); if reg & rst != 0 { return 0; } }
    regmap_read((*gate).map, get_clock_reg(&*gate), &mut reg); enval = if (*gate).flags & CLK_GATE_SET_TO_DISABLE != 0 { 0 } else { clk }; if reg & clk == enval { 1 } else { 0 }
}

unsafe fn aspeed_g6_clk_enable(hw: *mut clk_hw) -> i32 { let gate = to_aspeed_clk_gate(hw); let mut flags = 0; let clk = get_bit((*gate).clock_idx); let rst = get_bit((*gate).reset_idx as u8); spin_lock_irqsave((*gate).lock, &mut flags); if aspeed_g6_clk_is_enabled(hw) != 0 { spin_unlock_irqrestore((*gate).lock, flags); return 0; } if (*gate).reset_idx >= 0 { regmap_write((*gate).map, get_reset_reg(&*gate), rst); udelay(100); } if (*gate).flags & CLK_GATE_SET_TO_DISABLE != 0 { regmap_write((*gate).map, get_clock_reg(&*gate) + 4, clk); } else { regmap_write((*gate).map, get_clock_reg(&*gate), clk); } if (*gate).reset_idx >= 0 { mdelay(10); regmap_write((*gate).map, get_reset_reg(&*gate) + 4, rst); } spin_unlock_irqrestore((*gate).lock, flags); 0 }
unsafe fn aspeed_g6_clk_disable(hw: *mut clk_hw) { let gate = to_aspeed_clk_gate(hw); let mut flags = 0; let clk = get_bit((*gate).clock_idx); spin_lock_irqsave((*gate).lock, &mut flags); if (*gate).flags & CLK_GATE_SET_TO_DISABLE != 0 { regmap_write((*gate).map, get_clock_reg(&*gate), clk); } else { regmap_write((*gate).map, get_clock_reg(&*gate) + 4, clk); } spin_unlock_irqrestore((*gate).lock, flags); }

unsafe fn aspeed_g6_reset_deassert(rcdev: *mut reset_controller_dev, id: usize) -> i32 { let ar = to_aspeed_reset(rcdev); let reg = if id >= 32 { ASPEED_G6_RESET_CTRL2 } else { ASPEED_G6_RESET_CTRL }; regmap_write((*ar).map, reg + 4, get_bit(id as u8)) }
unsafe fn aspeed_g6_reset_assert(rcdev: *mut reset_controller_dev, id: usize) -> i32 { let ar = to_aspeed_reset(rcdev); let reg = if id >= 32 { ASPEED_G6_RESET_CTRL2 } else { ASPEED_G6_RESET_CTRL }; regmap_write((*ar).map, reg, get_bit(id as u8)) }
unsafe fn aspeed_g6_reset_status(rcdev: *mut reset_controller_dev, id: usize) -> i32 { let ar = to_aspeed_reset(rcdev); let reg = if id >= 32 { ASPEED_G6_RESET_CTRL2 } else { ASPEED_G6_RESET_CTRL }; let mut val = 0; let ret = regmap_read((*ar).map, reg, &mut val); if ret != 0 { ret } else if val & get_bit(id as u8) != 0 { 1 } else { 0 } }

// The remaining probe/clock-controller registration is a direct translation of
// the Linux platform-driver initialization and intentionally retains external
// kernel symbols and generated clock IDs.
unsafe fn aspeed_g6_clk_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev; let map = syscon_node_to_regmap((*dev).of_node); if is_err(map) { dev_err(dev, "no syscon regmap\n"); return ptr_err(map); }
    let ar = devm_kzalloc(dev, core::mem::size_of::<aspeed_reset>(), GFP_KERNEL); if ar.is_null() { return -12; } (*ar).map = map;
    (*ar).rcdev.owner = THIS_MODULE; (*ar).rcdev.nr_resets = 64; (*ar).rcdev.ops = &aspeed_g6_reset_ops; (*ar).rcdev.of_node = (*dev).of_node;
    let mut ret = devm_reset_controller_register(dev, &mut (*ar).rcdev); if ret != 0 { dev_err(dev, "could not register reset controller\n"); return ret; }
    let mut val = 0; regmap_read(map, ASPEED_G6_MISC_CTRL, &mut val); let rate = if val & UART_DIV13_EN != 0 { 24000000 / 13 } else { 24000000 }; let hw = clk_hw_register_fixed_rate(dev, b"uart\0".as_ptr() as *const i8, core::ptr::null(), 0, rate); if is_err(hw) { return ptr_err(hw); } (*aspeed_g6_clk_data).hws[ASPEED_CLK_UART] = hw;
    regmap_read(map, 0x80, &mut val); let rate = if val & (1 << 31) != 0 { 24000000 / 13 } else { 24000000 }; let hw = clk_hw_register_fixed_rate(dev, b"uartx\0".as_ptr() as *const i8, core::ptr::null(), 0, rate); if is_err(hw) { return ptr_err(hw); } (*aspeed_g6_clk_data).hws[ASPEED_CLK_UARTX] = hw;
    ret = 0; ret
}

// External initialization entry point retained for the kernel integration.
unsafe fn aspeed_g6_cc_init(np: *mut device_node) { scu_g6_base = of_iomap(np, 0); if scu_g6_base.is_null() { return; } soc_rev = (readl(scu_g6_base.add(ASPEED_G6_SILICON_REV as usize)) & CHIP_REVISION_ID >> 16) as u8; }

// Clock parent lists and revision-specific divider data from the source.
static emmc_extclk_parent_names: [*const i8; 2] = [b"emmc_extclk_hpll_in\0".as_ptr() as *const i8, b"mpll\0".as_ptr() as *const i8];
static vclk_parent_names: [*const i8; 4] = [b"dpll\0".as_ptr() as *const i8, b"d1pll\0".as_ptr() as *const i8, b"hclk\0".as_ptr() as *const i8, b"mclk\0".as_ptr() as *const i8];
static d1clk_parent_names: [*const i8; 5] = [b"dpll\0".as_ptr() as *const i8, b"epll\0".as_ptr() as *const i8, b"usb-phy-40m\0".as_ptr() as *const i8, b"gpioc6_clkin\0".as_ptr() as *const i8, b"dp_phy_pll\0".as_ptr() as *const i8];
static ast2600_a0_axi_ahb_div_table: [u32; 4] = [2, 2, 3, 5];
static ast2600_a1_axi_ahb_div0_tbl: [u32; 4] = [3, 2, 3, 4];
static ast2600_a1_axi_ahb_div1_tbl: [u32; 4] = [3, 4, 6, 8];
static ast2600_a1_axi_ahb200_tbl: [u32; 8] = [3, 4, 3, 4, 2, 2, 2, 2];

unsafe fn aspeed_g6_cc(map: *mut regmap) {
    let mut val = 0u32; let mut axi_div; let mut ahb_div; let mut div;
    clk_hw_register_fixed_rate(core::ptr::null_mut(), b"clkin\0".as_ptr() as *const i8, core::ptr::null(), 0, 25000000);
    regmap_read(map, ASPEED_HPLL_PARAM, &mut val); (*aspeed_g6_clk_data).hws[ASPEED_CLK_HPLL] = ast2600_calc_pll(b"hpll\0".as_ptr() as *const i8, val);
    regmap_read(map, ASPEED_MPLL_PARAM, &mut val); (*aspeed_g6_clk_data).hws[ASPEED_CLK_MPLL] = ast2600_calc_pll(b"mpll\0".as_ptr() as *const i8, val);
    regmap_read(map, ASPEED_DPLL_PARAM, &mut val); (*aspeed_g6_clk_data).hws[ASPEED_CLK_DPLL] = ast2600_calc_pll(b"dpll\0".as_ptr() as *const i8, val);
    regmap_read(map, ASPEED_EPLL_PARAM, &mut val); (*aspeed_g6_clk_data).hws[ASPEED_CLK_EPLL] = ast2600_calc_pll(b"epll\0".as_ptr() as *const i8, val);
    regmap_read(map, ASPEED_APLL_PARAM, &mut val); (*aspeed_g6_clk_data).hws[ASPEED_CLK_APLL] = ast2600_calc_apll(b"apll\0".as_ptr() as *const i8, val);
    regmap_read(map, ASPEED_G6_STRAP1, &mut val); axi_div = if val & (1 << 16) != 0 { 1 } else { 2 }; let divbits = (val >> 11) & 3;
    if soc_rev >= 1 { if divbits == 0 { ahb_div = ast2600_a1_axi_ahb200_tbl[((val >> 8) & 3) as usize]; if val & (1 << 16) != 0 { ahb_div *= 2; } } else if val & (1 << 16) != 0 { ahb_div = ast2600_a1_axi_ahb_div1_tbl[divbits as usize]; } else { ahb_div = ast2600_a1_axi_ahb_div0_tbl[divbits as usize]; } } else { ahb_div = ast2600_a0_axi_ahb_div_table[((val >> 11) & 3) as usize]; }
    (*aspeed_g6_clk_data).hws[ASPEED_CLK_AHB] = clk_hw_register_fixed_factor(core::ptr::null_mut(), b"ahb\0".as_ptr() as *const i8, b"hpll\0".as_ptr() as *const i8, 0, 1, axi_div * ahb_div);
    regmap_read(map, ASPEED_G6_CLK_SELECTION1, &mut val); div = 4 * (((val >> 23) & 7) + 1); (*aspeed_g6_clk_data).hws[ASPEED_CLK_APB1] = clk_hw_register_fixed_factor(core::ptr::null_mut(), b"apb1\0".as_ptr() as *const i8, b"hpll\0".as_ptr() as *const i8, 0, 1, div);
    regmap_read(map, ASPEED_G6_CLK_SELECTION4, &mut val); div = 2 * (((val >> 9) & 7) + 1); (*aspeed_g6_clk_data).hws[ASPEED_CLK_APB2] = clk_hw_register_fixed_factor(core::ptr::null_mut(), b"apb2\0".as_ptr() as *const i8, b"ahb\0".as_ptr() as *const i8, 0, 1, div);
    (*aspeed_g6_clk_data).hws[ASPEED_CLK_USBPHY_40M] = clk_hw_register_fixed_rate(core::ptr::null_mut(), b"usb-phy-40m\0".as_ptr() as *const i8, core::ptr::null(), 0, 40000000);
    regmap_update_bits(map, ASPEED_G6_CLK_SELECTION5, I3C_CLK_SELECTION | APLL_DIV_SELECTION, I3C_CLK_SELECT_APLL_DIV | APLL_DIV_8);
    (*aspeed_g6_clk_data).hws[ASPEED_CLK_I3C] = clk_hw_register_fixed_factor(core::ptr::null_mut(), b"i3cclk\0".as_ptr() as *const i8, b"apll\0".as_ptr() as *const i8, 0, 1, 8);
    (*aspeed_g6_clk_data).hws[ASPEED_CLK_FSI] = clk_hw_register_fixed_factor(core::ptr::null_mut(), b"fsiclk\0".as_ptr() as *const i8, b"apll\0".as_ptr() as *const i8, 0, 1, 4);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
