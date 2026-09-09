// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2014 Marvell Technology Group Ltd.
 *
 * Alexandre Belloni <alexandre.belloni@free-electrons.com>
 * Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
 */

// Linux kernel dependencies: <linux/clk.h>, <linux/clk-provider.h>,
// <linux/io.h>, <linux/kernel.h>, <linux/of.h>, <linux/of_address.h>,
// <linux/slab.h>, <dt-bindings/clock/berlin2q.h>, "berlin2-div.h",
// "berlin2-pll.h", and "common.h".

const REG_PINMUX0: usize = 0x0018;
const REG_PINMUX5: usize = 0x002c;
const REG_SYSPLLCTL0: usize = 0x0030;
const REG_SYSPLLCTL4: usize = 0x0040;
const REG_CLKENABLE: usize = 0x00e8;
const REG_CLKSELECT0: usize = 0x00ec;
const REG_CLKSELECT1: usize = 0x00f0;
const REG_CLKSELECT2: usize = 0x00f4;
const REG_CLKSWITCH0: usize = 0x00f8;
const REG_CLKSWITCH1: usize = 0x00fc;
const REG_SW_GENERIC0: usize = 0x0110;
const REG_SW_GENERIC3: usize = 0x011c;
const REG_SDIO0XIN_CLKCTL: usize = 0x0158;
const REG_SDIO1XIN_CLKCTL: usize = 0x015c;

const MAX_CLKS: usize = 28;
static mut clk_data: *mut clk_hw_onecell_data = core::ptr::null_mut();
static mut lock: spinlock_t = spinlock_t {};
static mut gbase: *mut u8 = core::ptr::null_mut();
static mut cpupll_base: *mut u8 = core::ptr::null_mut();

#[repr(usize)]
enum ClockName { REFCLK, SYSPLL, CPUPLL, AVPLL_B1, AVPLL_B2, AVPLL_B3, AVPLL_B4, AVPLL_B5, AVPLL_B6, AVPLL_B7, AVPLL_B8 }

static mut clk_names: [&'static str; 11] = [
    "refclk", "syspll", "cpupll", "avpll_b1", "avpll_b2", "avpll_b3",
    "avpll_b4", "avpll_b5", "avpll_b6", "avpll_b7", "avpll_b8",
];

static bg2q_pll_map: berlin2_pll_map = berlin2_pll_map {
    vcodiv: [1, 0, 2, 0, 3, 4, 0, 6, 8], mult: 1,
    fbdiv_shift: 7, rfdiv_shift: 2, divsel_shift: 9,
};

static default_parent_ids: [usize; 6] = [1, 6, 7, 8, 9, 1];

static bg2q_divs: [berlin2_div_data; 12] = [
    berlin2_div_data { name: "sys", parent_ids: &default_parent_ids, num_parents: 6, map: [BERLIN2_DIV_GATE!(REG_CLKENABLE,0), BERLIN2_PLL_SELECT!(REG_CLKSELECT0,0), BERLIN2_DIV_SELECT!(REG_CLKSELECT0,3), BERLIN2_PLL_SWITCH!(REG_CLKSWITCH0,3), BERLIN2_DIV_SWITCH!(REG_CLKSWITCH0,4), BERLIN2_DIV_D3SWITCH!(REG_CLKSWITCH0,5)], div_flags: BERLIN2_DIV_HAS_GATE | BERLIN2_DIV_HAS_MUX, flags: CLK_IGNORE_UNUSED },
    berlin2_div_data { name: "drmfigo", parent_ids: &default_parent_ids, num_parents: 6, map: [BERLIN2_DIV_GATE!(REG_CLKENABLE,17), BERLIN2_PLL_SELECT!(REG_CLKSELECT0,6), BERLIN2_DIV_SELECT!(REG_CLKSELECT0,9), BERLIN2_PLL_SWITCH!(REG_CLKSWITCH0,6), BERLIN2_DIV_SWITCH!(REG_CLKSWITCH0,7), BERLIN2_DIV_D3SWITCH!(REG_CLKSWITCH0,8)], div_flags: BERLIN2_DIV_HAS_GATE | BERLIN2_DIV_HAS_MUX, flags: 0 },
    berlin2_div_data { name: "cfg", parent_ids: &default_parent_ids, num_parents: 6, map: [BERLIN2_DIV_GATE!(REG_CLKENABLE,1), BERLIN2_PLL_SELECT!(REG_CLKSELECT0,12), BERLIN2_DIV_SELECT!(REG_CLKSELECT0,15), BERLIN2_PLL_SWITCH!(REG_CLKSWITCH0,9), BERLIN2_DIV_SWITCH!(REG_CLKSWITCH0,10), BERLIN2_DIV_D3SWITCH!(REG_CLKSWITCH0,11)], div_flags: BERLIN2_DIV_HAS_GATE | BERLIN2_DIV_HAS_MUX, flags: 0 },
    berlin2_div_data { name: "gfx2d", parent_ids: &default_parent_ids, num_parents: 6, map: [BERLIN2_DIV_GATE!(REG_CLKENABLE,4), BERLIN2_PLL_SELECT!(REG_CLKSELECT0,18), BERLIN2_DIV_SELECT!(REG_CLKSELECT0,21), BERLIN2_PLL_SWITCH!(REG_CLKSWITCH0,12), BERLIN2_DIV_SWITCH!(REG_CLKSWITCH0,13), BERLIN2_DIV_D3SWITCH!(REG_CLKSWITCH0,14)], div_flags: BERLIN2_DIV_HAS_GATE | BERLIN2_DIV_HAS_MUX, flags: 0 },
    berlin2_div_data { name: "zsp", parent_ids: &default_parent_ids, num_parents: 6, map: [BERLIN2_DIV_GATE!(REG_CLKENABLE,6), BERLIN2_PLL_SELECT!(REG_CLKSELECT0,24), BERLIN2_DIV_SELECT!(REG_CLKSELECT0,27), BERLIN2_PLL_SWITCH!(REG_CLKSWITCH0,15), BERLIN2_DIV_SWITCH!(REG_CLKSWITCH0,16), BERLIN2_DIV_D3SWITCH!(REG_CLKSWITCH0,17)], div_flags: BERLIN2_DIV_HAS_GATE | BERLIN2_DIV_HAS_MUX, flags: 0 },
    berlin2_div_data { name: "perif", parent_ids: &default_parent_ids, num_parents: 6, map: [BERLIN2_DIV_GATE!(REG_CLKENABLE,7), BERLIN2_PLL_SELECT!(REG_CLKSELECT1,0), BERLIN2_DIV_SELECT!(REG_CLKSELECT1,3), BERLIN2_PLL_SWITCH!(REG_CLKSWITCH0,18), BERLIN2_DIV_SWITCH!(REG_CLKSWITCH0,19), BERLIN2_DIV_D3SWITCH!(REG_CLKSWITCH0,20)], div_flags: BERLIN2_DIV_HAS_GATE | BERLIN2_DIV_HAS_MUX, flags: CLK_IGNORE_UNUSED },
    berlin2_div_data { name: "pcube", parent_ids: &default_parent_ids, num_parents: 6, map: [BERLIN2_DIV_GATE!(REG_CLKENABLE,2), BERLIN2_PLL_SELECT!(REG_CLKSELECT1,6), BERLIN2_DIV_SELECT!(REG_CLKSELECT1,9), BERLIN2_PLL_SWITCH!(REG_CLKSWITCH0,21), BERLIN2_DIV_SWITCH!(REG_CLKSWITCH0,22), BERLIN2_DIV_D3SWITCH!(REG_CLKSWITCH0,23)], div_flags: BERLIN2_DIV_HAS_GATE | BERLIN2_DIV_HAS_MUX, flags: 0 },
    berlin2_div_data { name: "vscope", parent_ids: &default_parent_ids, num_parents: 6, map: [BERLIN2_DIV_GATE!(REG_CLKENABLE,3), BERLIN2_PLL_SELECT!(REG_CLKSELECT1,12), BERLIN2_DIV_SELECT!(REG_CLKSELECT1,15), BERLIN2_PLL_SWITCH!(REG_CLKSWITCH0,24), BERLIN2_DIV_SWITCH!(REG_CLKSWITCH0,25), BERLIN2_DIV_D3SWITCH!(REG_CLKSWITCH0,26)], div_flags: BERLIN2_DIV_HAS_GATE | BERLIN2_DIV_HAS_MUX, flags: 0 },
    berlin2_div_data { name: "nfc_ecc", parent_ids: &default_parent_ids, num_parents: 6, map: [BERLIN2_DIV_GATE!(REG_CLKENABLE,19), BERLIN2_PLL_SELECT!(REG_CLKSELECT1,18), BERLIN2_DIV_SELECT!(REG_CLKSELECT1,21), BERLIN2_PLL_SWITCH!(REG_CLKSWITCH0,27), BERLIN2_DIV_SWITCH!(REG_CLKSWITCH0,28), BERLIN2_DIV_D3SWITCH!(REG_CLKSWITCH0,29)], div_flags: BERLIN2_DIV_HAS_GATE | BERLIN2_DIV_HAS_MUX, flags: 0 },
    berlin2_div_data { name: "vpp", parent_ids: &default_parent_ids, num_parents: 6, map: [BERLIN2_DIV_GATE!(REG_CLKENABLE,21), BERLIN2_PLL_SELECT!(REG_CLKSELECT1,24), BERLIN2_DIV_SELECT!(REG_CLKSELECT1,27), BERLIN2_PLL_SWITCH!(REG_CLKSWITCH0,30), BERLIN2_DIV_SWITCH!(REG_CLKSWITCH0,31), BERLIN2_DIV_D3SWITCH!(REG_CLKSWITCH1,0)], div_flags: BERLIN2_DIV_HAS_GATE | BERLIN2_DIV_HAS_MUX, flags: 0 },
    berlin2_div_data { name: "app", parent_ids: &default_parent_ids, num_parents: 6, map: [BERLIN2_DIV_GATE!(REG_CLKENABLE,20), BERLIN2_PLL_SELECT!(REG_CLKSELECT2,0), BERLIN2_DIV_SELECT!(REG_CLKSELECT2,3), BERLIN2_PLL_SWITCH!(REG_CLKSWITCH1,1), BERLIN2_DIV_SWITCH!(REG_CLKSWITCH1,2), BERLIN2_DIV_D3SWITCH!(REG_CLKSWITCH1,3)], div_flags: BERLIN2_DIV_HAS_GATE | BERLIN2_DIV_HAS_MUX, flags: 0 },
    berlin2_div_data { name: "sdio0xin", parent_ids: &default_parent_ids, num_parents: 6, map: [BERLIN2_SINGLE_DIV!(REG_SDIO0XIN_CLKCTL)], div_flags: BERLIN2_DIV_HAS_GATE | BERLIN2_DIV_HAS_MUX, flags: 0 },
    berlin2_div_data { name: "sdio1xin", parent_ids: &default_parent_ids, num_parents: 6, map: [BERLIN2_SINGLE_DIV!(REG_SDIO1XIN_CLKCTL)], div_flags: BERLIN2_DIV_HAS_GATE | BERLIN2_DIV_HAS_MUX, flags: 0 },
];

static bg2q_gates: [berlin2_gate_data; 12] = [
    berlin2_gate_data { name: "gfx2daxi", parent_name: "perif", bit_idx: 5, flags: 0 }, berlin2_gate_data { name: "geth0", parent_name: "perif", bit_idx: 8, flags: 0 }, berlin2_gate_data { name: "sata", parent_name: "perif", bit_idx: 9, flags: 0 }, berlin2_gate_data { name: "ahbapb", parent_name: "perif", bit_idx: 10, flags: CLK_IGNORE_UNUSED }, berlin2_gate_data { name: "usb0", parent_name: "perif", bit_idx: 11, flags: 0 }, berlin2_gate_data { name: "usb1", parent_name: "perif", bit_idx: 12, flags: 0 }, berlin2_gate_data { name: "usb2", parent_name: "perif", bit_idx: 13, flags: 0 }, berlin2_gate_data { name: "usb3", parent_name: "perif", bit_idx: 14, flags: 0 }, berlin2_gate_data { name: "pbridge", parent_name: "perif", bit_idx: 15, flags: CLK_IGNORE_UNUSED }, berlin2_gate_data { name: "sdio", parent_name: "perif", bit_idx: 16, flags: 0 }, berlin2_gate_data { name: "nfc", parent_name: "perif", bit_idx: 18, flags: 0 }, berlin2_gate_data { name: "pcie", parent_name: "perif", bit_idx: 22, flags: 0 },
];

unsafe fn berlin2q_clock_setup(np: *mut device_node) {
    let parent_np = of_get_parent(np);
    let mut parent_names: [&str; 9] = [""; 9];
    let mut clk: *mut clk = core::ptr::null_mut();
    let hws: *mut *mut clk_hw;
    let mut n: usize;
    let mut ret: i32;

    clk_data = kzalloc_flex(clk_data, &mut *(core::ptr::addr_of_mut!(hws)), MAX_CLKS);
    if clk_data.is_null() { of_node_put(parent_np); return; }
    (*clk_data).num = MAX_CLKS;
    let hws = (*clk_data).hws;
    gbase = of_iomap(parent_np, 0);
    if gbase.is_null() { of_node_put(parent_np); pr_err!("%pOF: Unable to map global base\n", np); return; }
    cpupll_base = of_iomap(parent_np, 1);
    of_node_put(parent_np);
    if cpupll_base.is_null() { pr_err!("%pOF: Unable to map cpupll base\n", np); iounmap(gbase); return; }
    clk = of_clk_get_by_name(np, clk_names[ClockName::REFCLK as usize]);
    if !IS_ERR(clk) { clk_names[0] = __clk_get_name(clk); clk_put(clk); }
    ret = berlin2_pll_register(&bg2q_pll_map, gbase.add(REG_SYSPLLCTL0), clk_names[1], clk_names[0], 0);
    if ret != 0 { return bg2q_fail(); }
    ret = berlin2_pll_register(&bg2q_pll_map, cpupll_base, clk_names[2], clk_names[0], 0);
    if ret != 0 { return bg2q_fail(); }
    // TODO: add BG2Q AVPLL.
    // TODO: add reference clock bypass switches: memPLLSWBypass, cpuPLLSWBypass, and sysPLLSWBypass.
    n = 0;
    while n < bg2q_divs.len() {
        let dd = &bg2q_divs[n];
        for k in 0..dd.num_parents { parent_names[k] = clk_names[dd.parent_ids[k]]; }
        *hws.add(20 + n) = berlin2_div_register(&dd.map, gbase, dd.name, dd.div_flags, parent_names.as_ptr(), dd.num_parents, dd.flags, &mut lock);
        n += 1;
    }
    n = 0;
    while n < bg2q_gates.len() {
        let gd = &bg2q_gates[n];
        *hws.add(8 + n) = clk_hw_register_gate(core::ptr::null_mut(), gd.name, gd.parent_name, gd.flags, gbase.add(REG_CLKENABLE), gd.bit_idx, 0, &mut lock);
        n += 1;
    }
    *hws.add(0) = clk_hw_register_fixed_factor(core::ptr::null_mut(), "cpu", clk_names[2], 0, 1, 1);
    *hws.add(1) = clk_hw_register_fixed_factor(core::ptr::null_mut(), "twd", "cpu", 0, 1, 3);
    for n in 0..MAX_CLKS { if IS_ERR(*hws.add(n)) { pr_err!("%pOF: Unable to register leaf clock %d\n", np, n); return bg2q_fail(); } }
    of_clk_add_hw_provider(np, of_clk_hw_onecell_get, clk_data);
}

unsafe fn bg2q_fail() { iounmap(cpupll_base); iounmap(gbase); }

// CLK_OF_DECLARE(berlin2q_clk, "marvell,berlin2q-clk", berlin2q_clock_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
