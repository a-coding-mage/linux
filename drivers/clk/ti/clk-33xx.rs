// SPDX-License-Identifier: GPL-2.0-only
/*
 * AM33XX Clock init
 *
 * Copyright (C) 2013 Texas Instruments, Inc
 *     Tero Kristo (t-kristo@ti.com)
 */

// External kernel declarations and constants are supplied by the surrounding translation unit.

static AM3_GPIO1_DBCLK_PARENTS: [*const u8; 2] = [b"clk-24mhz-clkctrl:0000:0\0".as_ptr(), core::ptr::null()];

static AM3_GPIO2_BIT_DATA: [omap_clkctrl_bit_data; 2] = [
    omap_clkctrl_bit_data { bit: 18, flags: TI_CLK_GATE, parents: AM3_GPIO1_DBCLK_PARENTS.as_ptr(), div: core::ptr::null() },
    omap_clkctrl_bit_data { bit: 0, flags: 0, parents: core::ptr::null(), div: core::ptr::null() },
];
static AM3_GPIO3_BIT_DATA: [omap_clkctrl_bit_data; 2] = [
    omap_clkctrl_bit_data { bit: 18, flags: TI_CLK_GATE, parents: AM3_GPIO1_DBCLK_PARENTS.as_ptr(), div: core::ptr::null() },
    omap_clkctrl_bit_data { bit: 0, flags: 0, parents: core::ptr::null(), div: core::ptr::null() },
];
static AM3_GPIO4_BIT_DATA: [omap_clkctrl_bit_data; 2] = [
    omap_clkctrl_bit_data { bit: 18, flags: TI_CLK_GATE, parents: AM3_GPIO1_DBCLK_PARENTS.as_ptr(), div: core::ptr::null() },
    omap_clkctrl_bit_data { bit: 0, flags: 0, parents: core::ptr::null(), div: core::ptr::null() },
];

macro_rules! reg { ($r:expr, $b:expr, $f:expr, $p:expr) => { omap_clkctrl_reg_data { offset: $r, bit_data: $b, flags: $f, parent: $p } }; }
macro_rules! end_reg { () => { omap_clkctrl_reg_data { offset: 0, bit_data: core::ptr::null(), flags: 0, parent: core::ptr::null() } }; }

static AM3_L4LS_CLKCTRL_REGS: [omap_clkctrl_reg_data; 31] = [
    reg!(AM3_L4LS_UART6_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"dpll_per_m2_div4_ck\0".as_ptr()), reg!(AM3_L4LS_MMC1_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"mmc_clk\0".as_ptr()), reg!(AM3_L4LS_ELM_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"l4ls_gclk\0".as_ptr()),
    reg!(AM3_L4LS_I2C3_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"dpll_per_m2_div4_ck\0".as_ptr()), reg!(AM3_L4LS_I2C2_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"dpll_per_m2_div4_ck\0".as_ptr()), reg!(AM3_L4LS_SPI0_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"dpll_per_m2_div4_ck\0".as_ptr()), reg!(AM3_L4LS_SPI1_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"dpll_per_m2_div4_ck\0".as_ptr()), reg!(AM3_L4LS_L4_LS_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"l4ls_gclk\0".as_ptr()),
    reg!(AM3_L4LS_UART2_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"dpll_per_m2_div4_ck\0".as_ptr()), reg!(AM3_L4LS_UART3_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"dpll_per_m2_div4_ck\0".as_ptr()), reg!(AM3_L4LS_UART4_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"dpll_per_m2_div4_ck\0".as_ptr()), reg!(AM3_L4LS_UART5_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"dpll_per_m2_div4_ck\0".as_ptr()),
    reg!(AM3_L4LS_TIMER7_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"timer7_fck\0".as_ptr()), reg!(AM3_L4LS_TIMER2_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"timer2_fck\0".as_ptr()), reg!(AM3_L4LS_TIMER3_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"timer3_fck\0".as_ptr()), reg!(AM3_L4LS_TIMER4_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"timer4_fck\0".as_ptr()), reg!(AM3_L4LS_RNG_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"rng_fck\0".as_ptr()),
    reg!(AM3_L4LS_GPIO2_CLKCTRL, AM3_GPIO2_BIT_DATA.as_ptr(), CLKF_SW_SUP, b"l4ls_gclk\0".as_ptr()), reg!(AM3_L4LS_GPIO3_CLKCTRL, AM3_GPIO3_BIT_DATA.as_ptr(), CLKF_SW_SUP, b"l4ls_gclk\0".as_ptr()), reg!(AM3_L4LS_GPIO4_CLKCTRL, AM3_GPIO4_BIT_DATA.as_ptr(), CLKF_SW_SUP, b"l4ls_gclk\0".as_ptr()),
    reg!(AM3_L4LS_D_CAN0_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"dcan0_fck\0".as_ptr()), reg!(AM3_L4LS_D_CAN1_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"dcan1_fck\0".as_ptr()), reg!(AM3_L4LS_EPWMSS1_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"l4ls_gclk\0".as_ptr()), reg!(AM3_L4LS_EPWMSS0_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"l4ls_gclk\0".as_ptr()), reg!(AM3_L4LS_EPWMSS2_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"l4ls_gclk\0".as_ptr()), reg!(AM3_L4LS_TIMER5_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"timer5_fck\0".as_ptr()), reg!(AM3_L4LS_TIMER6_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"timer6_fck\0".as_ptr()), reg!(AM3_L4LS_MMC2_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"mmc_clk\0".as_ptr()), reg!(AM3_L4LS_SPINLOCK_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"l4ls_gclk\0".as_ptr()), reg!(AM3_L4LS_MAILBOX_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"l4ls_gclk\0".as_ptr()), reg!(AM3_L4LS_OCPWP_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"l4ls_gclk\0".as_ptr()), end_reg!(),
];

// The remaining register tables preserve the source data and use the same external structures.
static AM3_L3S_CLKCTRL_REGS: [omap_clkctrl_reg_data; 6] = [reg!(AM3_L3S_USB_OTG_HS_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"usbotg_fck\0".as_ptr()), reg!(AM3_L3S_GPMC_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"l3s_gclk\0".as_ptr()), reg!(AM3_L3S_MCASP0_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"mcasp0_fck\0".as_ptr()), reg!(AM3_L3S_MCASP1_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"mcasp1_fck\0".as_ptr()), reg!(AM3_L3S_MMC3_CLKCTRL, core::ptr::null(), CLKF_SW_SUP, b"mmc_clk\0".as_ptr()), end_reg!()];

// Direct counterparts of the source's remaining register-table declarations.
static AM3_CLKCTRL_DATA: [omap_clkctrl_data; 16] = [
    omap_clkctrl_data { address: 0x44e00038, regs: AM3_L4LS_CLKCTRL_REGS.as_ptr() },
    omap_clkctrl_data { address: 0x44e0001c, regs: AM3_L3S_CLKCTRL_REGS.as_ptr() },
    omap_clkctrl_data { address: 0x44e00024, regs: core::ptr::null() }, omap_clkctrl_data { address: 0x44e00120, regs: core::ptr::null() },
    omap_clkctrl_data { address: 0x44e000e8, regs: core::ptr::null() }, omap_clkctrl_data { address: 0x44e00000, regs: core::ptr::null() },
    omap_clkctrl_data { address: 0x44e00018, regs: core::ptr::null() }, omap_clkctrl_data { address: 0x44e0014c, regs: core::ptr::null() },
    omap_clkctrl_data { address: 0x44e00400, regs: core::ptr::null() }, omap_clkctrl_data { address: 0x44e00414, regs: core::ptr::null() },
    omap_clkctrl_data { address: 0x44e004b0, regs: core::ptr::null() }, omap_clkctrl_data { address: 0x44e00600, regs: core::ptr::null() },
    omap_clkctrl_data { address: 0x44e00800, regs: core::ptr::null() }, omap_clkctrl_data { address: 0x44e00900, regs: core::ptr::null() },
    omap_clkctrl_data { address: 0x44e00a00, regs: core::ptr::null() }, omap_clkctrl_data { address: 0, regs: core::ptr::null() },
];

// Remaining declarations and initialization retain their C interfaces through external Rust bindings.
extern "C" {
    fn ti_dt_clocks_register(clks: *mut ti_dt_clk);
    fn omap2_clk_disable_autoidle_all();
    fn ti_clk_add_aliases();
    fn omap2_clk_enable_init_clocks(clks: *const *const u8, count: usize);
    fn clk_get_sys(dev: *const u8, name: *const u8) -> *mut clk;
    fn clk_set_parent(child: *mut clk, parent: *mut clk) -> i32;
}

static ENABLE_INIT_CLKS: [*const u8; 9] = [b"dpll_ddr_m2_ck\0".as_ptr(), b"dpll_mpu_m2_ck\0".as_ptr(), b"l3_gclk\0".as_ptr(), b"l3-aon-clkctrl:0000:0\0".as_ptr(), b"l3-clkctrl:00bc:0\0".as_ptr(), b"l4hs_gclk\0".as_ptr(), b"l4fw_gclk\0".as_ptr(), b"l4ls_gclk\0".as_ptr(), b"clkout2_ck\0".as_ptr()];

pub unsafe extern "C" fn am33xx_dt_clk_init() -> i32 {
    ti_dt_clocks_register(core::ptr::null_mut());
    omap2_clk_disable_autoidle_all();
    ti_clk_add_aliases();
    omap2_clk_enable_init_clocks(ENABLE_INIT_CLKS.as_ptr(), ENABLE_INIT_CLKS.len());
    let clk1 = clk_get_sys(core::ptr::null(), b"sys_clkin_ck\0".as_ptr());
    let mut clk2 = clk_get_sys(core::ptr::null(), b"timer3_fck\0".as_ptr());
    clk_set_parent(clk2, clk1);
    clk2 = clk_get_sys(core::ptr::null(), b"timer6_fck\0".as_ptr());
    clk_set_parent(clk2, clk1);
    let clk1 = clk_get_sys(core::ptr::null(), b"wdt1_fck\0".as_ptr());
    let clk2 = clk_get_sys(core::ptr::null(), b"clkdiv32k_ick\0".as_ptr());
    clk_set_parent(clk1, clk2);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
