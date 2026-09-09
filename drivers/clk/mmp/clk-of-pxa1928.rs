// SPDX-License-Identifier: GPL-2.0-only
/*
 * pxa1928 clock framework source file
 *
 * Copyright (C) 2015 Linaro, Ltd.
 * Rob Herring <robh@kernel.org>
 *
 * Based on drivers/clk/mmp/clk-of-mmp2.c:
 * Copyright (C) 2012 Marvell
 * Chao Xie <xiechao.mail@gmail.com>
 */

// Linux kernel and device-tree dependencies are supplied by the surrounding
// kernel/Rust bindings.

const MPMU_UART_PLL: usize = 0x14;
const APBC_NR_CLKS: usize = 48;
const APMU_NR_CLKS: usize = 96;

#[repr(C)]
struct Pxa1928ClkUnit {
    unit: MmpClkUnit,
    mpmu_base: *mut core::ffi::c_void,
    apmu_base: *mut core::ffi::c_void,
    apbc_base: *mut core::ffi::c_void,
    apbcp_base: *mut core::ffi::c_void,
}

static mut FIXED_RATE_CLKS: [MmpParamFixedRateClk; 6] = [
    MmpParamFixedRateClk { id: 0, name: "clk32", parent_name: core::ptr::null(), flags: 0, rate: 32768 },
    MmpParamFixedRateClk { id: 0, name: "vctcxo", parent_name: core::ptr::null(), flags: 0, rate: 26000000 },
    MmpParamFixedRateClk { id: 0, name: "pll1_624", parent_name: core::ptr::null(), flags: 0, rate: 624000000 },
    MmpParamFixedRateClk { id: 0, name: "pll5p", parent_name: core::ptr::null(), flags: 0, rate: 832000000 },
    MmpParamFixedRateClk { id: 0, name: "pll5", parent_name: core::ptr::null(), flags: 0, rate: 1248000000 },
    MmpParamFixedRateClk { id: 0, name: "usb_pll", parent_name: core::ptr::null(), flags: 0, rate: 480000000 },
];

static mut FIXED_FACTOR_CLKS: [MmpParamFixedFactorClk; 8] = [
    MmpParamFixedFactorClk { id: 0, name: "pll1_d2", parent_name: "pll1_624", mult: 1, div: 2, flags: 0 },
    MmpParamFixedFactorClk { id: 0, name: "pll1_d9", parent_name: "pll1_624", mult: 1, div: 9, flags: 0 },
    MmpParamFixedFactorClk { id: 0, name: "pll1_d12", parent_name: "pll1_624", mult: 1, div: 12, flags: 0 },
    MmpParamFixedFactorClk { id: 0, name: "pll1_d16", parent_name: "pll1_624", mult: 1, div: 16, flags: 0 },
    MmpParamFixedFactorClk { id: 0, name: "pll1_d20", parent_name: "pll1_624", mult: 1, div: 20, flags: 0 },
    MmpParamFixedFactorClk { id: 0, name: "pll1_416", parent_name: "pll1_624", mult: 2, div: 3, flags: 0 },
    MmpParamFixedFactorClk { id: 0, name: "vctcxo_d2", parent_name: "vctcxo", mult: 1, div: 2, flags: 0 },
    MmpParamFixedFactorClk { id: 0, name: "vctcxo_d4", parent_name: "vctcxo", mult: 1, div: 4, flags: 0 },
];

static mut UART_FACTOR_MASKS: MmpClkFactorMasks = MmpClkFactorMasks {
    factor: 2, num_mask: 0x1fff, den_mask: 0x1fff, num_shift: 16, den_shift: 0,
};
static mut UART_FACTOR_TBL: [U32Fract; 2] = [
    U32Fract { numerator: 832, denominator: 234 },
    U32Fract { numerator: 1, denominator: 1 },
];

static mut UART0_LOCK: Spinlock = Spinlock::new();
static mut UART1_LOCK: Spinlock = Spinlock::new();
static mut UART2_LOCK: Spinlock = Spinlock::new();
static mut UART3_LOCK: Spinlock = Spinlock::new();
static UART_PARENT_NAMES: [&str; 2] = ["uart_pll", "vctcxo"];
static mut SSP0_LOCK: Spinlock = Spinlock::new();
static mut SSP1_LOCK: Spinlock = Spinlock::new();
static SSP_PARENT_NAMES: [&str; 4] = ["vctcxo_d4", "vctcxo_d2", "vctcxo", "pll1_d12"];
static mut RESET_LOCK: Spinlock = Spinlock::new();
static mut SDH0_LOCK: Spinlock = Spinlock::new();
static mut SDH1_LOCK: Spinlock = Spinlock::new();
static mut SDH2_LOCK: Spinlock = Spinlock::new();
static mut SDH3_LOCK: Spinlock = Spinlock::new();
static mut SDH4_LOCK: Spinlock = Spinlock::new();
static SDH_PARENT_NAMES: [&str; 4] = ["pll1_624", "pll5p", "pll5", "pll1_416"];
static mut USB_LOCK: Spinlock = Spinlock::new();

unsafe fn pxa1928_pll_init(pxa_unit: *mut Pxa1928ClkUnit) {
    let unit = &mut (*pxa_unit).unit;
    mmp_register_fixed_rate_clks(unit, FIXED_RATE_CLKS.as_mut_ptr(), FIXED_RATE_CLKS.len());
    mmp_register_fixed_factor_clks(unit, FIXED_FACTOR_CLKS.as_mut_ptr(), FIXED_FACTOR_CLKS.len());
    mmp_clk_register_factor(
        "uart_pll", "pll1_416", CLK_SET_RATE_PARENT,
        (*pxa_unit).mpmu_base.add(MPMU_UART_PLL), &mut UART_FACTOR_MASKS,
        UART_FACTOR_TBL.as_mut_ptr(), UART_FACTOR_TBL.len(), core::ptr::null_mut(),
    );
}

// The following clock tables retain the source definitions and are consumed
// by the external MMP clock registration interfaces.
static mut APBC_MUX_CLKS: [MmpParamMuxClk; 6] = [
    MmpParamMuxClk::new(0, "uart0_mux", &UART_PARENT_NAMES, PXA1928_CLK_UART0 * 4, 4, 3, &raw mut UART0_LOCK),
    MmpParamMuxClk::new(0, "uart1_mux", &UART_PARENT_NAMES, PXA1928_CLK_UART1 * 4, 4, 3, &raw mut UART1_LOCK),
    MmpParamMuxClk::new(0, "uart2_mux", &UART_PARENT_NAMES, PXA1928_CLK_UART2 * 4, 4, 3, &raw mut UART2_LOCK),
    MmpParamMuxClk::new(0, "uart3_mux", &UART_PARENT_NAMES, PXA1928_CLK_UART3 * 4, 4, 3, &raw mut UART3_LOCK),
    MmpParamMuxClk::new(0, "ssp0_mux", &SSP_PARENT_NAMES, PXA1928_CLK_SSP0 * 4, 4, 3, &raw mut SSP0_LOCK),
    MmpParamMuxClk::new(0, "ssp1_mux", &SSP_PARENT_NAMES, PXA1928_CLK_SSP1 * 4, 4, 3, &raw mut SSP1_LOCK),
];

static mut APBC_GATE_CLKS: [MmpParamGateClk; 19] = [
    MmpParamGateClk::new(PXA1928_CLK_TWSI0, "twsi0_clk", "vctcxo", PXA1928_CLK_TWSI0 * 4, 0x3, 0x3, 0, 0, &raw mut RESET_LOCK),
    MmpParamGateClk::new(PXA1928_CLK_TWSI1, "twsi1_clk", "vctcxo", PXA1928_CLK_TWSI1 * 4, 0x3, 0x3, 0, 0, &raw mut RESET_LOCK),
    MmpParamGateClk::new(PXA1928_CLK_TWSI2, "twsi2_clk", "vctcxo", PXA1928_CLK_TWSI2 * 4, 0x3, 0x3, 0, 0, &raw mut RESET_LOCK),
    MmpParamGateClk::new(PXA1928_CLK_TWSI3, "twsi3_clk", "vctcxo", PXA1928_CLK_TWSI3 * 4, 0x3, 0x3, 0, 0, &raw mut RESET_LOCK),
    MmpParamGateClk::new(PXA1928_CLK_TWSI4, "twsi4_clk", "vctcxo", PXA1928_CLK_TWSI4 * 4, 0x3, 0x3, 0, 0, &raw mut RESET_LOCK),
    MmpParamGateClk::new(PXA1928_CLK_TWSI5, "twsi5_clk", "vctcxo", PXA1928_CLK_TWSI5 * 4, 0x3, 0x3, 0, 0, &raw mut RESET_LOCK),
    MmpParamGateClk::new(PXA1928_CLK_GPIO, "gpio_clk", "vctcxo", PXA1928_CLK_GPIO * 4, 0x3, 0x3, 0, 0, &raw mut RESET_LOCK),
    MmpParamGateClk::new(PXA1928_CLK_KPC, "kpc_clk", "clk32", PXA1928_CLK_KPC * 4, 0x3, 0x3, 0, MMP_CLK_GATE_NEED_DELAY, core::ptr::null_mut()),
    MmpParamGateClk::new(PXA1928_CLK_RTC, "rtc_clk", "clk32", PXA1928_CLK_RTC * 4, 0x83, 0x83, 0, MMP_CLK_GATE_NEED_DELAY, core::ptr::null_mut()),
    MmpParamGateClk::new(PXA1928_CLK_PWM0, "pwm0_clk", "vctcxo", PXA1928_CLK_PWM0 * 4, 0x3, 0x3, 0, 0, &raw mut RESET_LOCK),
    MmpParamGateClk::new(PXA1928_CLK_PWM1, "pwm1_clk", "vctcxo", PXA1928_CLK_PWM1 * 4, 0x3, 0x3, 0, 0, &raw mut RESET_LOCK),
    MmpParamGateClk::new(PXA1928_CLK_PWM2, "pwm2_clk", "vctcxo", PXA1928_CLK_PWM2 * 4, 0x3, 0x3, 0, 0, &raw mut RESET_LOCK),
    MmpParamGateClk::new(PXA1928_CLK_PWM3, "pwm3_clk", "vctcxo", PXA1928_CLK_PWM3 * 4, 0x3, 0x3, 0, 0, &raw mut RESET_LOCK),
    MmpParamGateClk::new(PXA1928_CLK_UART0, "uart0_clk", "uart0_mux", PXA1928_CLK_UART0 * 4, 0x3, 0x3, 0, 0, &raw mut UART0_LOCK),
    MmpParamGateClk::new(PXA1928_CLK_UART1, "uart1_clk", "uart1_mux", PXA1928_CLK_UART1 * 4, 0x3, 0x3, 0, 0, &raw mut UART1_LOCK),
    MmpParamGateClk::new(PXA1928_CLK_UART2, "uart2_clk", "uart2_mux", PXA1928_CLK_UART2 * 4, 0x3, 0x3, 0, 0, &raw mut UART2_LOCK),
    MmpParamGateClk::new(PXA1928_CLK_UART3, "uart3_clk", "uart3_mux", PXA1928_CLK_UART3 * 4, 0x3, 0x3, 0, 0, &raw mut UART3_LOCK),
    MmpParamGateClk::new(PXA1928_CLK_SSP0, "ssp0_clk", "ssp0_mux", PXA1928_CLK_SSP0 * 4, 0x3, 0x3, 0, 0, &raw mut SSP0_LOCK),
    MmpParamGateClk::new(PXA1928_CLK_SSP1, "ssp1_clk", "ssp1_mux", PXA1928_CLK_SSP1 * 4, 0x3, 0x3, 0, 0, &raw mut SSP1_LOCK),
];

static mut APMU_MUX_CLKS: [MmpParamMuxClk; 1] = [
    MmpParamMuxClk::new(0, "sdh_mux", &SDH_PARENT_NAMES, PXA1928_CLK_SDH0 * 4, 8, 2, &raw mut SDH0_LOCK),
];
static mut APMU_DIV_CLKS: [MmpParamDivClk; 1] = [
    MmpParamDivClk::new(0, "sdh_div", "sdh_mux", PXA1928_CLK_SDH0 * 4, 10, 4, CLK_DIVIDER_ONE_BASED, &raw mut SDH0_LOCK),
];
static mut APMU_GATE_CLKS: [MmpParamGateClk; 7] = [
    MmpParamGateClk::new(PXA1928_CLK_USB, "usb_clk", "usb_pll", PXA1928_CLK_USB * 4, 0x9, 0x9, 0, 0, &raw mut USB_LOCK),
    MmpParamGateClk::new(PXA1928_CLK_HSIC, "hsic_clk", "usb_pll", PXA1928_CLK_HSIC * 4, 0x9, 0x9, 0, 0, &raw mut USB_LOCK),
    MmpParamGateClk::new(PXA1928_CLK_SDH0, "sdh0_clk", "sdh_div", PXA1928_CLK_SDH0 * 4, 0x1b, 0x1b, 0, 0, &raw mut SDH0_LOCK),
    MmpParamGateClk::new(PXA1928_CLK_SDH1, "sdh1_clk", "sdh_div", PXA1928_CLK_SDH1 * 4, 0x1b, 0x1b, 0, 0, &raw mut SDH1_LOCK),
    MmpParamGateClk::new(PXA1928_CLK_SDH2, "sdh2_clk", "sdh_div", PXA1928_CLK_SDH2 * 4, 0x1b, 0x1b, 0, 0, &raw mut SDH2_LOCK),
    MmpParamGateClk::new(PXA1928_CLK_SDH3, "sdh3_clk", "sdh_div", PXA1928_CLK_SDH3 * 4, 0x1b, 0x1b, 0, 0, &raw mut SDH3_LOCK),
    MmpParamGateClk::new(PXA1928_CLK_SDH4, "sdh4_clk", "sdh_div", PXA1928_CLK_SDH4 * 4, 0x1b, 0x1b, 0, 0, &raw mut SDH4_LOCK),
];

unsafe fn pxa1928_apb_periph_clk_init(pxa_unit: *mut Pxa1928ClkUnit) {
    let unit = &mut (*pxa_unit).unit;
    mmp_register_mux_clks(unit, APBC_MUX_CLKS.as_mut_ptr(), (*pxa_unit).apbc_base, APBC_MUX_CLKS.len());
    mmp_register_gate_clks(unit, APBC_GATE_CLKS.as_mut_ptr(), (*pxa_unit).apbc_base, APBC_GATE_CLKS.len());
}

unsafe fn pxa1928_axi_periph_clk_init(pxa_unit: *mut Pxa1928ClkUnit) {
    let unit = &mut (*pxa_unit).unit;
    mmp_register_mux_clks(unit, APMU_MUX_CLKS.as_mut_ptr(), (*pxa_unit).apmu_base, APMU_MUX_CLKS.len());
    mmp_register_div_clks(unit, APMU_DIV_CLKS.as_mut_ptr(), (*pxa_unit).apmu_base, APMU_DIV_CLKS.len());
    mmp_register_gate_clks(unit, APMU_GATE_CLKS.as_mut_ptr(), (*pxa_unit).apmu_base, APMU_GATE_CLKS.len());
}

unsafe fn pxa1928_clk_reset_init(np: *mut DeviceNode, pxa_unit: *mut Pxa1928ClkUnit) {
    let nr_resets = APBC_GATE_CLKS.len();
    let cells = kzalloc_array::<MmpClkResetCell>(nr_resets);
    if cells.is_null() { return; }
    for i in 0..nr_resets {
        let cell = &mut *cells.add(i);
        cell.clk_id = APBC_GATE_CLKS[i].id;
        cell.reg = (*pxa_unit).apbc_base.add(APBC_GATE_CLKS[i].offset);
        cell.flags = 0;
        cell.lock = APBC_GATE_CLKS[i].lock;
        cell.bits = 0x4;
    }
    mmp_clk_reset_register(np, cells, nr_resets);
}

unsafe fn pxa1928_mpmu_clk_init(np: *mut DeviceNode) {
    let pxa_unit = kzalloc::<Pxa1928ClkUnit>();
    if pxa_unit.is_null() { return; }
    (*pxa_unit).mpmu_base = of_iomap(np, 0);
    if (*pxa_unit).mpmu_base.is_null() { pr_err("failed to map mpmu registers\n"); kfree(pxa_unit); return; }
    pxa1928_pll_init(pxa_unit);
}

unsafe fn pxa1928_apmu_clk_init(np: *mut DeviceNode) {
    let pxa_unit = kzalloc::<Pxa1928ClkUnit>();
    if pxa_unit.is_null() { return; }
    (*pxa_unit).apmu_base = of_iomap(np, 0);
    if (*pxa_unit).apmu_base.is_null() { pr_err("failed to map apmu registers\n"); kfree(pxa_unit); return; }
    mmp_clk_init(np, &mut (*pxa_unit).unit, APMU_NR_CLKS);
    pxa1928_axi_periph_clk_init(pxa_unit);
}

unsafe fn pxa1928_apbc_clk_init(np: *mut DeviceNode) {
    let pxa_unit = kzalloc::<Pxa1928ClkUnit>();
    if pxa_unit.is_null() { return; }
    (*pxa_unit).apbc_base = of_iomap(np, 0);
    if (*pxa_unit).apbc_base.is_null() { pr_err("failed to map apbc registers\n"); kfree(pxa_unit); return; }
    mmp_clk_init(np, &mut (*pxa_unit).unit, APBC_NR_CLKS);
    pxa1928_apb_periph_clk_init(pxa_unit);
    pxa1928_clk_reset_init(np, pxa_unit);
}

// CLK_OF_DECLARE(pxa1928_mpmu_clk, "marvell,pxa1928-mpmu", pxa1928_mpmu_clk_init);
// CLK_OF_DECLARE(pxa1928_apmu_clk, "marvell,pxa1928-apmu", pxa1928_apmu_clk_init);
// CLK_OF_DECLARE(pxa1928_apbc_clk, "marvell,pxa1928-apbc", pxa1928_apbc_clk_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
