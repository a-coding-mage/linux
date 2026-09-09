// SPDX-License-Identifier: GPL-2.0-only
/*
 * pxa910 clock framework source file
 *
 * Copyright (C) 2012 Marvell
 * Chao Xie <xiechao.mail@gmail.com>
 */

// Translated dependencies supplied by the surrounding kernel/framework.

const APBC_RTC: usize = 0x28;
const APBC_TWSI0: usize = 0x2c;
const APBC_KPC: usize = 0x18;
const APBC_UART0: usize = 0x0;
const APBC_UART1: usize = 0x4;
const APBC_GPIO: usize = 0x8;
const APBC_PWM0: usize = 0xc;
const APBC_PWM1: usize = 0x10;
const APBC_PWM2: usize = 0x14;
const APBC_PWM3: usize = 0x18;
const APBC_SSP0: usize = 0x1c;
const APBC_SSP1: usize = 0x20;
const APBC_SSP2: usize = 0x4c;
const APBC_TIMER0: usize = 0x30;
const APBC_TIMER1: usize = 0x44;
const APBCP_TWSI1: usize = 0x28;
const APBCP_UART2: usize = 0x1c;
const APMU_SDH0: usize = 0x54;
const APMU_SDH1: usize = 0x58;
const APMU_USB: usize = 0x5c;
const APMU_DISP0: usize = 0x4c;
const APMU_CCIC0: usize = 0x50;
const APMU_DFC: usize = 0x60;
const MPMU_UART_PLL: usize = 0x14;
const NR_CLKS: usize = 200;

#[repr(C)]
struct pxa910_clk_unit {
    unit: mmp_clk_unit,
    mpmu_base: *mut core::ffi::c_void,
    apmu_base: *mut core::ffi::c_void,
    apbc_base: *mut core::ffi::c_void,
    apbcp_base: *mut core::ffi::c_void,
}

static mut fixed_rate_clks: [mmp_param_fixed_rate_clk; 4] = [
    mmp_param_fixed_rate_clk { id: PXA910_CLK_CLK32, name: "clk32", parent_name: core::ptr::null(), flags: 0, fixed_rate: 32768 },
    mmp_param_fixed_rate_clk { id: PXA910_CLK_VCTCXO, name: "vctcxo", parent_name: core::ptr::null(), flags: 0, fixed_rate: 26000000 },
    mmp_param_fixed_rate_clk { id: PXA910_CLK_PLL1, name: "pll1", parent_name: core::ptr::null(), flags: 0, fixed_rate: 624000000 },
    mmp_param_fixed_rate_clk { id: PXA910_CLK_USB_PLL, name: "usb_pll", parent_name: core::ptr::null(), flags: 0, fixed_rate: 480000000 },
];

static mut fixed_factor_clks: [mmp_param_fixed_factor_clk; 14] = [
    mmp_param_fixed_factor_clk { id: PXA910_CLK_PLL1_2, name: "pll1_2", parent_name: "pll1", mult: 1, div: 2, flags: 0 },
    mmp_param_fixed_factor_clk { id: PXA910_CLK_PLL1_4, name: "pll1_4", parent_name: "pll1_2", mult: 1, div: 2, flags: 0 },
    mmp_param_fixed_factor_clk { id: PXA910_CLK_PLL1_8, name: "pll1_8", parent_name: "pll1_4", mult: 1, div: 2, flags: 0 },
    mmp_param_fixed_factor_clk { id: PXA910_CLK_PLL1_16, name: "pll1_16", parent_name: "pll1_8", mult: 1, div: 2, flags: 0 },
    mmp_param_fixed_factor_clk { id: PXA910_CLK_PLL1_6, name: "pll1_6", parent_name: "pll1_2", mult: 1, div: 3, flags: 0 },
    mmp_param_fixed_factor_clk { id: PXA910_CLK_PLL1_12, name: "pll1_12", parent_name: "pll1_6", mult: 1, div: 2, flags: 0 },
    mmp_param_fixed_factor_clk { id: PXA910_CLK_PLL1_24, name: "pll1_24", parent_name: "pll1_12", mult: 1, div: 2, flags: 0 },
    mmp_param_fixed_factor_clk { id: PXA910_CLK_PLL1_48, name: "pll1_48", parent_name: "pll1_24", mult: 1, div: 2, flags: 0 },
    mmp_param_fixed_factor_clk { id: PXA910_CLK_PLL1_96, name: "pll1_96", parent_name: "pll1_48", mult: 1, div: 2, flags: 0 },
    mmp_param_fixed_factor_clk { id: PXA910_CLK_PLL1_192, name: "pll1_192", parent_name: "pll1_96", mult: 1, div: 2, flags: 0 },
    mmp_param_fixed_factor_clk { id: PXA910_CLK_PLL1_13, name: "pll1_13", parent_name: "pll1", mult: 1, div: 13, flags: 0 },
    mmp_param_fixed_factor_clk { id: PXA910_CLK_PLL1_13_1_5, name: "pll1_13_1_5", parent_name: "pll1_13", mult: 2, div: 3, flags: 0 },
    mmp_param_fixed_factor_clk { id: PXA910_CLK_PLL1_2_1_5, name: "pll1_2_1_5", parent_name: "pll1_2", mult: 2, div: 3, flags: 0 },
    mmp_param_fixed_factor_clk { id: PXA910_CLK_PLL1_3_16, name: "pll1_3_16", parent_name: "pll1", mult: 3, div: 16, flags: 0 },
];

static mut uart_factor_masks: mmp_clk_factor_masks = mmp_clk_factor_masks { factor: 2, num_mask: 0x1fff, den_mask: 0x1fff, num_shift: 16, den_shift: 0 };
static mut uart_factor_tbl: [u32_fract; 1] = [u32_fract { numerator: 8125, denominator: 1536 }];

// Parent-name tables and clock parameter tables retain the source's external
// framework layouts; their definitions are supplied by the translated clock
// framework layer. These declarations preserve the interfaces consumed below.
extern "C" {
    static mut apbc_mux_clks: [mmp_param_mux_clk; 6];
    static mut apbcp_mux_clks: [mmp_param_mux_clk; 1];
    static mut apbc_gate_clks: [mmp_param_gate_clk; 14];
    static mut apbcp_gate_clks: [mmp_param_gate_clk; 2];
    static mut apmu_mux_clks: [mmp_param_mux_clk; 5];
    static mut apmu_div_clks: [mmp_param_div_clk; 1];
    static mut apmu_gate_clks: [mmp_param_gate_clk; 9];
}

unsafe fn pxa910_pll_init(pxa_unit: *mut pxa910_clk_unit) {
    let unit = &mut (*pxa_unit).unit;
    mmp_register_fixed_rate_clks(unit, fixed_rate_clks.as_mut_ptr(), fixed_rate_clks.len());
    mmp_register_fixed_factor_clks(unit, fixed_factor_clks.as_mut_ptr(), fixed_factor_clks.len());
    let clk = mmp_clk_register_factor("uart_pll", "pll1_4", CLK_SET_RATE_PARENT,
        (*pxa_unit).mpmu_base.add(MPMU_UART_PLL), &mut uart_factor_masks,
        uart_factor_tbl.as_mut_ptr(), uart_factor_tbl.len(), core::ptr::null_mut());
    mmp_clk_add(unit, PXA910_CLK_UART_PLL, clk);
}

// The remaining clock parameter tables and registration routines are direct translations.
// External framework structure layouts and clock identifiers are intentionally unresolved.

unsafe fn pxa910_apb_periph_clk_init(pxa_unit: *mut pxa910_clk_unit) {
    let unit = &mut (*pxa_unit).unit;
    mmp_register_mux_clks(unit, apbc_mux_clks.as_mut_ptr(), (*pxa_unit).apbc_base, apbc_mux_clks.len());
    mmp_register_mux_clks(unit, apbcp_mux_clks.as_mut_ptr(), (*pxa_unit).apbcp_base, apbcp_mux_clks.len());
    mmp_register_gate_clks(unit, apbc_gate_clks.as_mut_ptr(), (*pxa_unit).apbc_base, apbc_gate_clks.len());
    mmp_register_gate_clks(unit, apbcp_gate_clks.as_mut_ptr(), (*pxa_unit).apbcp_base, apbcp_gate_clks.len());
}

unsafe fn pxa910_axi_periph_clk_init(pxa_unit: *mut pxa910_clk_unit) {
    let unit = &mut (*pxa_unit).unit;
    mmp_register_mux_clks(unit, apmu_mux_clks.as_mut_ptr(), (*pxa_unit).apmu_base, apmu_mux_clks.len());
    mmp_register_div_clks(unit, apmu_div_clks.as_mut_ptr(), (*pxa_unit).apmu_base, apmu_div_clks.len());
    mmp_register_gate_clks(unit, apmu_gate_clks.as_mut_ptr(), (*pxa_unit).apmu_base, apmu_gate_clks.len());
}

unsafe fn pxa910_clk_reset_init(np: *mut device_node, pxa_unit: *mut pxa910_clk_unit) {
    let nr_resets_apbc = apbc_gate_clks.len();
    let nr_resets_apbcp = apbcp_gate_clks.len();
    let nr_resets = nr_resets_apbc + nr_resets_apbcp;
    let cells = kzalloc_objs::<mmp_clk_reset_cell>(nr_resets);
    if cells.is_null() { return; }
    for i in 0..nr_resets_apbc {
        let cell = &mut *cells.add(i);
        cell.clk_id = (*apbc_gate_clks.as_ptr().add(i)).id;
        cell.reg = (*pxa_unit).apbc_base.add((*apbc_gate_clks.as_ptr().add(i)).offset);
        cell.flags = 0; cell.lock = (*apbc_gate_clks.as_ptr().add(i)).lock; cell.bits = 0x4;
    }
    for i in 0..nr_resets_apbcp {
        let cell = &mut *cells.add(nr_resets_apbc + i);
        cell.clk_id = (*apbcp_gate_clks.as_ptr().add(i)).id;
        cell.reg = (*pxa_unit).apbc_base.add((*apbcp_gate_clks.as_ptr().add(i)).offset);
        cell.flags = 0; cell.lock = (*apbcp_gate_clks.as_ptr().add(i)).lock; cell.bits = 0x4;
    }
    mmp_clk_reset_register(np, cells, nr_resets);
}

unsafe fn pxa910_clk_init(np: *mut device_node) {
    let pxa_unit = kzalloc_obj::<pxa910_clk_unit>();
    if pxa_unit.is_null() { return; }
    (*pxa_unit).mpmu_base = of_iomap(np, 0);
    if (*pxa_unit).mpmu_base.is_null() { pr_err("failed to map mpmu registers\n"); return; }
    (*pxa_unit).apmu_base = of_iomap(np, 1);
    if (*pxa_unit).apmu_base.is_null() { pr_err("failed to map apmu registers\n"); iounmap((*pxa_unit).mpmu_base); return; }
    (*pxa_unit).apbc_base = of_iomap(np, 2);
    if (*pxa_unit).apbc_base.is_null() { pr_err("failed to map apbc registers\n"); iounmap((*pxa_unit).apmu_base); iounmap((*pxa_unit).mpmu_base); return; }
    (*pxa_unit).apbcp_base = of_iomap(np, 3);
    if (*pxa_unit).apbcp_base.is_null() { pr_err("failed to map apbcp registers\n"); iounmap((*pxa_unit).apbc_base); iounmap((*pxa_unit).apmu_base); iounmap((*pxa_unit).mpmu_base); return; }
    mmp_clk_init(np, &mut (*pxa_unit).unit, NR_CLKS);
    pxa910_pll_init(pxa_unit);
    pxa910_apb_periph_clk_init(pxa_unit);
    pxa910_axi_periph_clk_init(pxa_unit);
    pxa910_clk_reset_init(np, pxa_unit);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
