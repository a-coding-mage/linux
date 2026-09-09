// SPDX-License-Identifier: GPL-2.0-only
/* mmp2 clock framework source file */

const APBC_RTC: usize = 0x0;
const APBC_TWSI0: usize = 0x4;
const APBC_TWSI1: usize = 0x8;
const APBC_TWSI2: usize = 0xc;
const APBC_TWSI3: usize = 0x10;
const APBC_TWSI4: usize = 0x7c;
const APBC_TWSI5: usize = 0x80;
const APBC_KPC: usize = 0x18;
const APBC_TIMER: usize = 0x24;
const APBC_UART0: usize = 0x2c;
const APBC_UART1: usize = 0x30;
const APBC_UART2: usize = 0x34;
const APBC_UART3: usize = 0x88;
const APBC_GPIO: usize = 0x38;
const APBC_PWM0: usize = 0x3c;
const APBC_PWM1: usize = 0x40;
const APBC_PWM2: usize = 0x44;
const APBC_PWM3: usize = 0x48;
const APBC_SSP0: usize = 0x50;
const APBC_SSP1: usize = 0x54;
const APBC_SSP2: usize = 0x58;
const APBC_SSP3: usize = 0x5c;
const APBC_THERMAL0: usize = 0x90;
const APBC_THERMAL1: usize = 0x98;
const APBC_THERMAL2: usize = 0x9c;
const APBC_THERMAL3: usize = 0xa0;
const APMU_SDH0: usize = 0x54;
const APMU_SDH1: usize = 0x58;
const APMU_SDH2: usize = 0xe8;
const APMU_SDH3: usize = 0xec;
const APMU_SDH4: usize = 0x15c;
const APMU_USB: usize = 0x5c;
const APMU_DISP0: usize = 0x4c;
const APMU_DISP1: usize = 0x110;
const APMU_CCIC0: usize = 0x50;
const APMU_CCIC1: usize = 0xf4;
const APMU_USBHSIC0: usize = 0xf8;
const APMU_USBHSIC1: usize = 0xfc;
const APMU_GPU: usize = 0xcc;
const APMU_AUDIO: usize = 0x10c;
const APMU_CAMERA: usize = 0x1fc;
const MPMU_FCCR: usize = 0x8;
const MPMU_POSR: usize = 0x10;
const MPMU_UART_PLL: usize = 0x14;
const MPMU_PLL2_CR: usize = 0x34;
const MPMU_I2S0_PLL: usize = 0x40;
const MPMU_I2S1_PLL: usize = 0x44;
const MPMU_ACGR: usize = 0x1024;
const MPMU_PLL3_CR: usize = 0x50;
const MPMU_PLL3_CTRL1: usize = 0x58;
const MPMU_PLL1_CTRL: usize = 0x5c;
const MPMU_PLL_DIFF_CTRL: usize = 0x68;
const MPMU_PLL2_CTRL1: usize = 0x414;
const NR_CLKS: usize = 200;

#[repr(C)]
enum mmp2_clk_model { CLK_MODEL_MMP2, CLK_MODEL_MMP3 }

#[repr(C)]
struct mmp2_clk_unit {
    unit: mmp_clk_unit,
    model: mmp2_clk_model,
    pd_data: genpd_onecell_data,
    pm_domains: [*mut generic_pm_domain; MMP2_NR_POWER_DOMAINS],
    mpmu_base: *mut core::ffi::c_void,
    apmu_base: *mut core::ffi::c_void,
    apbc_base: *mut core::ffi::c_void,
}

static mut fixed_rate_clks: [mmp_param_fixed_rate_clk; 4] = [
    mmp_param_fixed_rate_clk { id: MMP2_CLK_CLK32, name: "clk32", parent_name: core::ptr::null(), flags: 0, rate: 32768 },
    mmp_param_fixed_rate_clk { id: MMP2_CLK_VCTCXO, name: "vctcxo", parent_name: core::ptr::null(), flags: 0, rate: 26000000 },
    mmp_param_fixed_rate_clk { id: MMP2_CLK_USB_PLL, name: "usb_pll", parent_name: core::ptr::null(), flags: 0, rate: 480000000 },
    mmp_param_fixed_rate_clk { id: 0, name: "i2s_pll", parent_name: core::ptr::null(), flags: 0, rate: 99666667 },
];

static mut pll_clks: [mmp_param_pll_clk; 2] = [
    mmp_param_pll_clk { id: MMP2_CLK_PLL1, name: "pll1", rate: 797330000, reg: MPMU_FCCR, mask: 0x4000, lock_reg: MPMU_POSR, lock_bit: 0 },
    mmp_param_pll_clk { id: MMP2_CLK_PLL2, name: "pll2", rate: 0, reg: MPMU_PLL2_CR, mask: 0x0300, lock_reg: MPMU_PLL2_CR, lock_bit: 10 },
];

static mut mmp3_pll_clks: [mmp_param_pll_clk; 5] = [
    mmp_param_pll_clk { id: MMP2_CLK_PLL2, name: "pll1", rate: 797330000, reg: MPMU_FCCR, mask: 0x4000, lock_reg: MPMU_POSR, lock_bit: 0, ref_rate: 26000000, ctrl_reg: MPMU_PLL1_CTRL, ctrl_bit: 25 },
    mmp_param_pll_clk { id: MMP2_CLK_PLL2, name: "pll2", rate: 0, reg: MPMU_PLL2_CR, mask: 0x0300, lock_reg: MPMU_PLL2_CR, lock_bit: 10, ref_rate: 26000000, ctrl_reg: MPMU_PLL2_CTRL1, ctrl_bit: 25 },
    mmp_param_pll_clk { id: MMP3_CLK_PLL1_P, name: "pll1_p", rate: 0, reg: MPMU_PLL_DIFF_CTRL, mask: 0x0010, lock_reg: 0, lock_bit: 0, ref_rate: 797330000, ctrl_reg: MPMU_PLL_DIFF_CTRL, ctrl_bit: 0 },
    mmp_param_pll_clk { id: MMP3_CLK_PLL2_P, name: "pll2_p", rate: 0, reg: MPMU_PLL_DIFF_CTRL, mask: 0x0100, lock_reg: MPMU_PLL2_CR, lock_bit: 10, ref_rate: 26000000, ctrl_reg: MPMU_PLL_DIFF_CTRL, ctrl_bit: 5 },
    mmp_param_pll_clk { id: MMP3_CLK_PLL3, name: "pll3", rate: 0, reg: MPMU_PLL3_CR, mask: 0x0300, lock_reg: MPMU_PLL3_CR, lock_bit: 10, ref_rate: 26000000, ctrl_reg: MPMU_PLL3_CTRL1, ctrl_bit: 25 },
];

static mut fixed_factor_clks: [mmp_param_fixed_factor_clk; 17] = [
    (MMP2_CLK_PLL1_2,"pll1_2","pll1",1,2,0),(MMP2_CLK_PLL1_4,"pll1_4","pll1_2",1,2,0),(MMP2_CLK_PLL1_8,"pll1_8","pll1_4",1,2,0),(MMP2_CLK_PLL1_16,"pll1_16","pll1_8",1,2,0),(MMP2_CLK_PLL1_20,"pll1_20","pll1_4",1,5,0),(MMP2_CLK_PLL1_3,"pll1_3","pll1",1,3,0),(MMP2_CLK_PLL1_6,"pll1_6","pll1_3",1,2,0),(MMP2_CLK_PLL1_12,"pll1_12","pll1_6",1,2,0),(MMP2_CLK_PLL2_2,"pll2_2","pll2",1,2,0),(MMP2_CLK_PLL2_4,"pll2_4","pll2_2",1,2,0),(MMP2_CLK_PLL2_8,"pll2_8","pll2_4",1,2,0),(MMP2_CLK_PLL2_16,"pll2_16","pll2_8",1,2,0),(MMP2_CLK_PLL2_3,"pll2_3","pll2",1,3,0),(MMP2_CLK_PLL2_6,"pll2_6","pll2_3",1,2,0),(MMP2_CLK_PLL2_12,"pll2_12","pll2_6",1,2,0),(MMP2_CLK_VCTCXO_2,"vctcxo_2","vctcxo",1,2,0),(MMP2_CLK_VCTCXO_4,"vctcxo_4","vctcxo_2",1,2,0)
];
static mut uart_factor_masks: mmp_clk_factor_masks = mmp_clk_factor_masks { factor:2, num_mask:0x1fff, den_mask:0x1fff, num_shift:16, den_shift:0 };
static mut uart_factor_tbl: [u32_fract; 2] = [u32_fract { numerator:8125, denominator:1536 }, u32_fract { numerator:3521, denominator:689 }];
static mut i2s_factor_masks: mmp_clk_factor_masks = mmp_clk_factor_masks { factor:2, num_mask:0x7fff, den_mask:0x1fff, num_shift:0, den_shift:15, enable_mask:0xd0000000 };
static mut i2s_factor_tbl: [u32_fract; 9] = [
    u32_fract { numerator:24868, denominator:511 },u32_fract { numerator:28003, denominator:793 },u32_fract { numerator:24941, denominator:1025 },u32_fract { numerator:28003, denominator:1586 },u32_fract { numerator:31158, denominator:2561 },u32_fract { numerator:16288, denominator:1845 },u32_fract { numerator:20772, denominator:2561 },u32_fract { numerator:8144, denominator:1845 },u32_fract { numerator:10386, denominator:2561 }
];

extern "C" {
    static mut mpmu_gate_clks: [mmp_param_gate_clk; 2];
    static mut apbc_mux_clks: [mmp_param_mux_clk; 9];
    static mut apbc_gate_clks: [mmp_param_gate_clk; 25];
    static mut mmp3_apbc_gate_clks: [mmp_param_gate_clk; 3];
    fn mmp2_axi_periph_clk_init(*mut mmp2_clk_unit);
    fn mmp2_clk_reset_init(*mut device_node, *mut mmp2_clk_unit);
    fn mmp2_pm_domain_init(*mut device_node, *mut mmp2_clk_unit);
}

unsafe fn mmp2_main_clk_init(pxa_unit: *mut mmp2_clk_unit) {
    let unit = &mut (*pxa_unit).unit;
    mmp_register_fixed_rate_clks(unit, fixed_rate_clks.as_ptr(), fixed_rate_clks.len());
    if (*pxa_unit).model == CLK_MODEL_MMP3 {
        mmp_register_pll_clks(unit, mmp3_pll_clks.as_ptr(), (*pxa_unit).mpmu_base, mmp3_pll_clks.len());
    } else {
        mmp_register_pll_clks(unit, pll_clks.as_ptr(), (*pxa_unit).mpmu_base, pll_clks.len());
    }
    mmp_register_fixed_factor_clks(unit, fixed_factor_clks.as_ptr(), fixed_factor_clks.len());
    let clk = mmp_clk_register_factor("uart_pll", "pll1_4", CLK_SET_RATE_PARENT, (*pxa_unit).mpmu_base.add(MPMU_UART_PLL), &uart_factor_masks, uart_factor_tbl.as_ptr(), uart_factor_tbl.len(), core::ptr::null_mut());
    mmp_clk_add(unit, MMP2_CLK_UART_PLL, clk);
    mmp_clk_register_factor("i2s0_pll", "pll1_4", CLK_SET_RATE_PARENT, (*pxa_unit).mpmu_base.add(MPMU_I2S0_PLL), &i2s_factor_masks, i2s_factor_tbl.as_ptr(), i2s_factor_tbl.len(), core::ptr::null_mut());
    mmp_clk_register_factor("i2s1_pll", "pll1_4", CLK_SET_RATE_PARENT, (*pxa_unit).mpmu_base.add(MPMU_I2S1_PLL), &i2s_factor_masks, i2s_factor_tbl.as_ptr(), i2s_factor_tbl.len(), core::ptr::null_mut());
    mmp_register_gate_clks(unit, mpmu_gate_clks.as_ptr(), (*pxa_unit).mpmu_base, mpmu_gate_clks.len());
}

// The source's APBC/APMU tables, reset/power-domain initialization, and DT
// registration are represented with their original names and call ordering.
unsafe fn mmp2_apb_periph_clk_init(pxa_unit: *mut mmp2_clk_unit) {
    let unit = &mut (*pxa_unit).unit;
    mmp_register_mux_clks(unit, apbc_mux_clks.as_ptr(), (*pxa_unit).apbc_base, apbc_mux_clks.len());
    mmp_register_gate_clks(unit, apbc_gate_clks.as_ptr(), (*pxa_unit).apbc_base, apbc_gate_clks.len());
    if (*pxa_unit).model == CLK_MODEL_MMP3 { mmp_register_gate_clks(unit, mmp3_apbc_gate_clks.as_ptr(), (*pxa_unit).apbc_base, mmp3_apbc_gate_clks.len()); }
}

unsafe fn mmp2_clk_init(np: *mut device_node) {
    let p = kzalloc_obj::<mmp2_clk_unit>();
    if p.is_null() { return; }
    (*p).model = if of_device_is_compatible(np, "marvell,mmp3-clock") { CLK_MODEL_MMP3 } else { CLK_MODEL_MMP2 };
    (*p).mpmu_base = of_iomap(np, 0);
    if (*p).mpmu_base.is_null() { pr_err!("failed to map mpmu registers\n"); kfree(p); return; }
    (*p).apmu_base = of_iomap(np, 1);
    if (*p).apmu_base.is_null() { pr_err!("failed to map apmu registers\n"); iounmap((*p).mpmu_base); kfree(p); return; }
    (*p).apbc_base = of_iomap(np, 2);
    if (*p).apbc_base.is_null() { pr_err!("failed to map apbc registers\n"); iounmap((*p).apmu_base); iounmap((*p).mpmu_base); kfree(p); return; }
    mmp2_pm_domain_init(np, p);
    mmp_clk_init(np, &mut (*p).unit, NR_CLKS);
    mmp2_main_clk_init(p);
    mmp2_apb_periph_clk_init(p);
    mmp2_axi_periph_clk_init(p);
    mmp2_clk_reset_init(np, p);
}

// CLK_OF_DECLARE(mmp2_clk, "marvell,mmp2-clock", mmp2_clk_init);
// CLK_OF_DECLARE(mmp3_clk, "marvell,mmp3-clock", mmp2_clk_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
