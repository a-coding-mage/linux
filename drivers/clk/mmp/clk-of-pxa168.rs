// SPDX-License-Identifier: GPL-2.0-only
/*
 * pxa168 clock framework source file
 *
 * Copyright (C) 2012 Marvell
 * Chao Xie <xiechao.mail@gmail.com>
 */

// Linux kernel and local clock/reset headers are supplied by the surrounding crate.

const APBC_UART0: u32 = 0x0;
const APBC_UART1: u32 = 0x4;
const APBC_GPIO: u32 = 0x8;
const APBC_PWM0: u32 = 0xc;
const APBC_PWM1: u32 = 0x10;
const APBC_PWM2: u32 = 0x14;
const APBC_PWM3: u32 = 0x18;
const APBC_RTC: u32 = 0x28;
const APBC_TWSI0: u32 = 0x2c;
const APBC_KPC: u32 = 0x30;
const APBC_TIMER: u32 = 0x34;
const APBC_AIB: u32 = 0x3c;
const APBC_SW_JTAG: u32 = 0x40;
const APBC_ONEWIRE: u32 = 0x48;
const APBC_TWSI1: u32 = 0x6c;
const APBC_UART2: u32 = 0x70;
const APBC_AC97: u32 = 0x84;
const APBC_SSP0: u32 = 0x81c;
const APBC_SSP1: u32 = 0x820;
const APBC_SSP2: u32 = 0x84c;
const APBC_SSP3: u32 = 0x858;
const APBC_SSP4: u32 = 0x85c;
const APMU_DISP0: u32 = 0x4c;
const APMU_CCIC0: u32 = 0x50;
const APMU_SDH0: u32 = 0x54;
const APMU_SDH1: u32 = 0x58;
const APMU_USB: u32 = 0x5c;
const APMU_DFC: u32 = 0x60;
const APMU_DMA: u32 = 0x64;
const APMU_BUS: u32 = 0x6c;
const APMU_GC: u32 = 0xcc;
const APMU_SMC: u32 = 0xd4;
const APMU_XD: u32 = 0xdc;
const APMU_SDH2: u32 = 0xe0;
const APMU_SDH3: u32 = 0xe4;
const APMU_CF: u32 = 0xf0;
const APMU_MSP: u32 = 0xf4;
const APMU_CMU: u32 = 0xf8;
const APMU_FE: u32 = 0xfc;
const APMU_PCIE: u32 = 0x100;
const APMU_EPD: u32 = 0x104;
const MPMU_UART_PLL: u32 = 0x14;
const NR_CLKS: usize = 200;

#[repr(C)]
struct Pxa168ClkUnit {
    unit: MmpClkUnit,
    mpmu_base: *mut core::ffi::c_void,
    apmu_base: *mut core::ffi::c_void,
    apbc_base: *mut core::ffi::c_void,
}

static mut FIXED_RATE_CLKS: [MmpParamFixedRateClk; 4] = [
    MmpParamFixedRateClk { id: PXA168_CLK_CLK32, name: "clk32", parent: core::ptr::null(), flags: 0, rate: 32768 },
    MmpParamFixedRateClk { id: PXA168_CLK_VCTCXO, name: "vctcxo", parent: core::ptr::null(), flags: 0, rate: 26000000 },
    MmpParamFixedRateClk { id: PXA168_CLK_PLL1, name: "pll1", parent: core::ptr::null(), flags: 0, rate: 624000000 },
    MmpParamFixedRateClk { id: PXA168_CLK_USB_PLL, name: "usb_pll", parent: core::ptr::null(), flags: 0, rate: 480000000 },
];

static mut FIXED_FACTOR_CLKS: [MmpParamFixedFactorClk; 17] = [
    MmpParamFixedFactorClk { id: PXA168_CLK_PLL1_2, name: "pll1_2", parent: "pll1", mult: 1, div: 2, flags: 0 },
    MmpParamFixedFactorClk { id: PXA168_CLK_PLL1_4, name: "pll1_4", parent: "pll1_2", mult: 1, div: 2, flags: 0 },
    MmpParamFixedFactorClk { id: PXA168_CLK_PLL1_8, name: "pll1_8", parent: "pll1_4", mult: 1, div: 2, flags: 0 },
    MmpParamFixedFactorClk { id: PXA168_CLK_PLL1_16, name: "pll1_16", parent: "pll1_8", mult: 1, div: 2, flags: 0 },
    MmpParamFixedFactorClk { id: PXA168_CLK_PLL1_6, name: "pll1_6", parent: "pll1_2", mult: 1, div: 3, flags: 0 },
    MmpParamFixedFactorClk { id: PXA168_CLK_PLL1_12, name: "pll1_12", parent: "pll1_6", mult: 1, div: 2, flags: 0 },
    MmpParamFixedFactorClk { id: PXA168_CLK_PLL1_24, name: "pll1_24", parent: "pll1_12", mult: 1, div: 2, flags: 0 },
    MmpParamFixedFactorClk { id: PXA168_CLK_PLL1_48, name: "pll1_48", parent: "pll1_24", mult: 1, div: 2, flags: 0 },
    MmpParamFixedFactorClk { id: PXA168_CLK_PLL1_96, name: "pll1_96", parent: "pll1_48", mult: 1, div: 2, flags: 0 },
    MmpParamFixedFactorClk { id: PXA168_CLK_PLL1_192, name: "pll1_192", parent: "pll1_96", mult: 1, div: 2, flags: 0 },
    MmpParamFixedFactorClk { id: PXA168_CLK_PLL1_13, name: "pll1_13", parent: "pll1", mult: 1, div: 13, flags: 0 },
    MmpParamFixedFactorClk { id: PXA168_CLK_PLL1_13_1_5, name: "pll1_13_1_5", parent: "pll1_13", mult: 1, div: 5, flags: 0 },
    MmpParamFixedFactorClk { id: PXA168_CLK_PLL1_2_1_5, name: "pll1_2_1_5", parent: "pll1_2", mult: 1, div: 5, flags: 0 },
    MmpParamFixedFactorClk { id: PXA168_CLK_PLL1_3_16, name: "pll1_3_16", parent: "pll1", mult: 3, div: 16, flags: 0 },
    MmpParamFixedFactorClk { id: PXA168_CLK_PLL1_2_1_10, name: "pll1_2_1_10", parent: "pll1_2", mult: 1, div: 10, flags: 0 },
    MmpParamFixedFactorClk { id: PXA168_CLK_PLL1_2_3_16, name: "pll1_2_3_16", parent: "pll1_2", mult: 3, div: 16, flags: 0 },
    MmpParamFixedFactorClk { id: PXA168_CLK_CLK32_2, name: "clk32_2", parent: "clk32", mult: 1, div: 2, flags: 0 },
];

static mut UART_FACTOR_MASKS: MmpClkFactorMasks = MmpClkFactorMasks { factor: 2, num_mask: 0x1fff, den_mask: 0x1fff, num_shift: 16, den_shift: 0 };
static mut UART_FACTOR_TBL: [U32Fract; 1] = [U32Fract { numerator: 8125, denominator: 1536 }]; // 14.745MHZ

unsafe fn pxa168_pll_init(pxa_unit: *mut Pxa168ClkUnit) {
    let unit = &mut (*pxa_unit).unit;
    mmp_register_fixed_rate_clks(unit, FIXED_RATE_CLKS.as_mut_ptr(), FIXED_RATE_CLKS.len());
    mmp_register_fixed_factor_clks(unit, FIXED_FACTOR_CLKS.as_mut_ptr(), FIXED_FACTOR_CLKS.len());
    let clk = mmp_clk_register_factor("uart_pll", "pll1_4", CLK_SET_RATE_PARENT,
        (*pxa_unit).mpmu_base.add(MPMU_UART_PLL as usize), &mut UART_FACTOR_MASKS,
        UART_FACTOR_TBL.as_mut_ptr(), UART_FACTOR_TBL.len(), core::ptr::null_mut());
    mmp_clk_add(unit, PXA168_CLK_UART_PLL, clk);
}

// DEFINE_SPINLOCK declarations and parent-name arrays retain their C linkage/state.
extern "C" {
    static mut twsi0_lock: SpinLock;
    static mut twsi1_lock: SpinLock;
    static mut kpc_lock: SpinLock;
    static mut pwm0_lock: SpinLock;
    static mut pwm1_lock: SpinLock;
    static mut pwm2_lock: SpinLock;
    static mut pwm3_lock: SpinLock;
    static mut uart0_lock: SpinLock;
    static mut uart1_lock: SpinLock;
    static mut uart2_lock: SpinLock;
    static mut ssp0_lock: SpinLock;
    static mut ssp1_lock: SpinLock;
    static mut ssp2_lock: SpinLock;
    static mut ssp3_lock: SpinLock;
    static mut ssp4_lock: SpinLock;
    static mut timer_lock: SpinLock;
    static mut reset_lock: SpinLock;
    static mut dfc_lock: SpinLock;
    static mut sdh0_lock: SpinLock;
    static mut sdh1_lock: SpinLock;
    static mut sdh2_lock: SpinLock;
    static mut sdh3_lock: SpinLock;
    static mut usb_lock: SpinLock;
    static mut disp0_lock: SpinLock;
    static mut ccic0_lock: SpinLock;
}

static TWSI_PARENT_NAMES: [&str; 2] = ["pll1_2_1_10", "pll1_2_1_5"];
static KPC_PARENT_NAMES: [&str; 3] = ["clk32", "clk32_2", "pll1_24"];
static PWM_PARENT_NAMES: [&str; 2] = ["pll1_48", "clk32"];
static UART_PARENT_NAMES: [&str; 2] = ["pll1_2_3_16", "uart_pll"];
static SSP_PARENT_NAMES: [&str; 4] = ["pll1_96", "pll1_48", "pll1_24", "pll1_12"];
static TIMER_PARENT_NAMES: [&str; 4] = ["pll1_48", "clk32", "pll1_96", "pll1_192"];
static DFC_PARENT_NAMES: [&str; 2] = ["pll1_4", "pll1_8"];
static SDH_PARENT_NAMES: [&str; 3] = ["pll1_13", "pll1_12", "pll1_8"];
static DISP_PARENT_NAMES: [&str; 2] = ["pll1", "pll1_2"];
static CCIC_PARENT_NAMES: [&str; 2] = ["pll1_4", "pll1_8"];
static CCIC_PHY_PARENT_NAMES: [&str; 2] = ["pll1_6", "pll1_12"];

// The following tables preserve the complete clock topology and register fields.
static mut APBC_MUX_CLKS: [MmpParamMuxClk; 16] = [
    mux!("twsi0_mux", TWSI_PARENT_NAMES, APBC_TWSI0, 4, 3, twsi0_lock), mux!("twsi1_mux", TWSI_PARENT_NAMES, APBC_TWSI1, 4, 3, twsi1_lock),
    mux!("kpc_mux", KPC_PARENT_NAMES, APBC_KPC, 4, 3, kpc_lock), mux!("pwm0_mux", PWM_PARENT_NAMES, APBC_PWM0, 4, 3, pwm0_lock),
    mux!("pwm1_mux", PWM_PARENT_NAMES, APBC_PWM1, 4, 3, pwm1_lock), mux!("pwm2_mux", PWM_PARENT_NAMES, APBC_PWM2, 4, 3, pwm2_lock),
    mux!("pwm3_mux", PWM_PARENT_NAMES, APBC_PWM3, 4, 3, pwm3_lock), mux!("uart0_mux", UART_PARENT_NAMES, APBC_UART0, 4, 3, uart0_lock),
    mux!("uart1_mux", UART_PARENT_NAMES, APBC_UART1, 4, 3, uart1_lock), mux!("uart2_mux", UART_PARENT_NAMES, APBC_UART2, 4, 3, uart2_lock),
    mux!("ssp0_mux", SSP_PARENT_NAMES, APBC_SSP0, 4, 3, ssp0_lock), mux!("ssp1_mux", SSP_PARENT_NAMES, APBC_SSP1, 4, 3, ssp1_lock),
    mux!("ssp2_mux", SSP_PARENT_NAMES, APBC_SSP2, 4, 3, ssp2_lock), mux!("ssp3_mux", SSP_PARENT_NAMES, APBC_SSP3, 4, 3, ssp3_lock),
    mux!("ssp4_mux", SSP_PARENT_NAMES, APBC_SSP4, 4, 3, ssp4_lock), mux!("timer_mux", TIMER_PARENT_NAMES, APBC_TIMER, 4, 3, timer_lock),
];

// Gate/divider table initializers remain literal source-level data.
static mut APBC_GATE_CLKS: [MmpParamGateClk; 17] = [
    gate!(PXA168_CLK_TWSI0,"twsi0_clk","twsi0_mux",APBC_TWSI0,0x3,twsi0_lock), gate!(PXA168_CLK_TWSI1,"twsi1_clk","twsi1_mux",APBC_TWSI1,0x3,twsi1_lock),
    gate!(PXA168_CLK_GPIO,"gpio_clk","vctcxo",APBC_GPIO,0x1,reset_lock), gate!(PXA168_CLK_KPC,"kpc_clk","kpc_mux",APBC_KPC,0x3,kpc_lock),
    gate!(PXA168_CLK_RTC,"rtc_clk","clk32",APBC_RTC,0x83,none), gate!(PXA168_CLK_PWM0,"pwm0_clk","pwm0_mux",APBC_PWM0,0x3,pwm0_lock),
    gate!(PXA168_CLK_PWM1,"pwm1_clk","pwm1_mux",APBC_PWM1,0x3,pwm1_lock), gate!(PXA168_CLK_PWM2,"pwm2_clk","pwm2_mux",APBC_PWM2,0x3,pwm2_lock),
    gate!(PXA168_CLK_PWM3,"pwm3_clk","pwm3_mux",APBC_PWM3,0x3,pwm3_lock), gate!(PXA168_CLK_UART0,"uart0_clk","uart0_mux",APBC_UART0,0x3,uart0_lock),
    gate!(PXA168_CLK_UART1,"uart1_clk","uart1_mux",APBC_UART1,0x3,uart1_lock), gate!(PXA168_CLK_UART2,"uart2_clk","uart2_mux",APBC_UART2,0x3,uart2_lock),
    gate!(PXA168_CLK_SSP0,"ssp0_clk","ssp0_mux",APBC_SSP0,0x3,ssp0_lock), gate!(PXA168_CLK_SSP1,"ssp1_clk","ssp1_mux",APBC_SSP1,0x3,ssp1_lock),
    gate!(PXA168_CLK_SSP2,"ssp2_clk","ssp2_mux",APBC_SSP2,0x3,ssp2_lock), gate!(PXA168_CLK_SSP3,"ssp3_clk","ssp3_mux",APBC_SSP3,0x3,ssp3_lock),
    gate!(PXA168_CLK_SSP4,"ssp4_clk","ssp4_mux",APBC_SSP4,0x3,ssp4_lock),
];

static mut APMU_MUX_CLKS: [MmpParamMuxClk; 8] = [
    mux!("dfc_mux", DFC_PARENT_NAMES, APMU_DFC, 6, 1, dfc_lock), mux!("sdh0_mux", SDH_PARENT_NAMES, APMU_SDH0, 6, 2, sdh0_lock),
    mux!("sdh1_mux", SDH_PARENT_NAMES, APMU_SDH1, 6, 2, sdh1_lock), mux!("sdh2_mux", SDH_PARENT_NAMES, APMU_SDH2, 6, 2, sdh2_lock),
    mux!("sdh3_mux", SDH_PARENT_NAMES, APMU_SDH3, 6, 2, sdh3_lock), mux!("disp0_mux", DISP_PARENT_NAMES, APMU_DISP0, 6, 1, disp0_lock),
    mux!("ccic0_mux", CCIC_PARENT_NAMES, APMU_CCIC0, 6, 1, ccic0_lock), mux!("ccic0_phy_mux", CCIC_PHY_PARENT_NAMES, APMU_CCIC0, 7, 1, ccic0_lock),
];
static mut APMU_DIV_CLKS: [MmpParamDivClk; 1] = [div!("ccic0_sphy_div", "ccic0_mux", APMU_CCIC0, 10, 5, ccic0_lock)];
static mut APMU_GATE_CLKS: [MmpParamGateClk; 14] = [
    gate!(PXA168_CLK_DFC,"dfc_clk","dfc_mux",APMU_DFC,0x19b,dfc_lock), gate!(PXA168_CLK_USB,"usb_clk","usb_pll",APMU_USB,0x9,usb_lock), gate!(PXA168_CLK_SPH,"sph_clk","usb_pll",APMU_USB,0x12,usb_lock),
    gate!(PXA168_CLK_SDH0,"sdh0_clk","sdh0_mux",APMU_SDH0,0x12,sdh0_lock), gate!(PXA168_CLK_SDH1,"sdh1_clk","sdh1_mux",APMU_SDH1,0x12,sdh1_lock), gate!(PXA168_CLK_SDH2,"sdh2_clk","sdh2_mux",APMU_SDH2,0x12,sdh2_lock), gate!(PXA168_CLK_SDH3,"sdh3_clk","sdh3_mux",APMU_SDH3,0x12,sdh3_lock),
    gate!(PXA168_CLK_SDH01_AXI,"sdh01_axi_clk",none,APMU_SDH0,0x9,sdh0_lock), gate!(PXA168_CLK_SDH23_AXI,"sdh23_axi_clk",none,APMU_SDH2,0x9,sdh2_lock),
    gate!(PXA168_CLK_DISP0,"disp0_clk","disp0_mux",APMU_DISP0,0x1b,disp0_lock), gate!(PXA168_CLK_CCIC0,"ccic0_clk","ccic0_mux",APMU_CCIC0,0x1b,ccic0_lock),
    gate!(PXA168_CLK_CCIC0_PHY,"ccic0_phy_clk","ccic0_phy_mux",APMU_CCIC0,0x24,ccic0_lock), gate!(PXA168_CLK_CCIC0_SPHY,"ccic0_sphy_clk","ccic0_sphy_div",APMU_CCIC0,0x300,ccic0_lock),
];

unsafe fn pxa168_apb_periph_clk_init(p: *mut Pxa168ClkUnit) {
    mmp_register_mux_clks(&mut (*p).unit, APBC_MUX_CLKS.as_mut_ptr(), (*p).apbc_base, APBC_MUX_CLKS.len());
    mmp_register_gate_clks(&mut (*p).unit, APBC_GATE_CLKS.as_mut_ptr(), (*p).apbc_base, APBC_GATE_CLKS.len());
}
unsafe fn pxa168_axi_periph_clk_init(p: *mut Pxa168ClkUnit) {
    mmp_register_mux_clks(&mut (*p).unit, APMU_MUX_CLKS.as_mut_ptr(), (*p).apmu_base, APMU_MUX_CLKS.len());
    mmp_register_div_clks(&mut (*p).unit, APMU_DIV_CLKS.as_mut_ptr(), (*p).apmu_base, APMU_DIV_CLKS.len());
    mmp_register_gate_clks(&mut (*p).unit, APMU_GATE_CLKS.as_mut_ptr(), (*p).apmu_base, APMU_GATE_CLKS.len());
}
unsafe fn pxa168_clk_reset_init(np: *mut DeviceNode, p: *mut Pxa168ClkUnit) {
    let mut cells = kzalloc_objs::<MmpClkResetCell>(APBC_GATE_CLKS.len());
    if cells.is_null() { return; }
    for i in 0..APBC_GATE_CLKS.len() {
        (*cells.add(i)).clk_id = APBC_GATE_CLKS[i].id;
        (*cells.add(i)).reg = (*p).apbc_base.add(APBC_GATE_CLKS[i].offset as usize);
        (*cells.add(i)).flags = 0;
        (*cells.add(i)).lock = APBC_GATE_CLKS[i].lock;
        (*cells.add(i)).bits = 0x4;
    }
    mmp_clk_reset_register(np, cells, APBC_GATE_CLKS.len());
}
unsafe extern "C" fn pxa168_clk_init(np: *mut DeviceNode) {
    let p = kzalloc_obj::<Pxa168ClkUnit>();
    if p.is_null() { return; }
    (*p).mpmu_base = of_iomap(np, 0);
    if (*p).mpmu_base.is_null() { pr_err!("failed to map mpmu registers\n"); kfree(p); return; }
    (*p).apmu_base = of_iomap(np, 1);
    if (*p).apmu_base.is_null() { pr_err!("failed to map apmu registers\n"); kfree(p); return; }
    (*p).apbc_base = of_iomap(np, 2);
    if (*p).apbc_base.is_null() { pr_err!("failed to map apbc registers\n"); kfree(p); return; }
    mmp_clk_init(np, &mut (*p).unit, NR_CLKS);
    pxa168_pll_init(p); pxa168_apb_periph_clk_init(p); pxa168_axi_periph_clk_init(p); pxa168_clk_reset_init(np, p);
}

// CLK_OF_DECLARE(pxa168_clk, "marvell,pxa168-clock", pxa168_clk_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
