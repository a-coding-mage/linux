// SPDX-License-Identifier: GPL-2.0-only

// External Linux kernel and TI clock-provider declarations are supplied by
// the surrounding translation environment.

static DM816_DEFAULT_CLKCTRL_REGS: [omap_clkctrl_reg_data; 2] = [
    omap_clkctrl_reg_data { reg: DM816_USB_OTG_HS_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk6_ck" },
    omap_clkctrl_reg_data { reg: 0, bit: core::ptr::null(), flags: 0, parent: core::ptr::null() },
];

static DM816_ALWON_CLKCTRL_REGS: [omap_clkctrl_reg_data; 30] = [
    omap_clkctrl_reg_data { reg: DM816_UART1_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk10_ck" },
    omap_clkctrl_reg_data { reg: DM816_UART2_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk10_ck" },
    omap_clkctrl_reg_data { reg: DM816_UART3_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk10_ck" },
    omap_clkctrl_reg_data { reg: DM816_GPIO1_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk6_ck" },
    omap_clkctrl_reg_data { reg: DM816_GPIO2_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk6_ck" },
    omap_clkctrl_reg_data { reg: DM816_I2C1_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk10_ck" },
    omap_clkctrl_reg_data { reg: DM816_I2C2_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk10_ck" },
    omap_clkctrl_reg_data { reg: DM816_TIMER1_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP, parent: "timer1_fck" },
    omap_clkctrl_reg_data { reg: DM816_TIMER2_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP, parent: "timer2_fck" },
    omap_clkctrl_reg_data { reg: DM816_TIMER3_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP, parent: "timer3_fck" },
    omap_clkctrl_reg_data { reg: DM816_TIMER4_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP, parent: "timer4_fck" },
    omap_clkctrl_reg_data { reg: DM816_TIMER5_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP, parent: "timer5_fck" },
    omap_clkctrl_reg_data { reg: DM816_TIMER6_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP, parent: "timer6_fck" },
    omap_clkctrl_reg_data { reg: DM816_TIMER7_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP, parent: "timer7_fck" },
    omap_clkctrl_reg_data { reg: DM816_WD_TIMER_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP | CLKF_NO_IDLEST, parent: "sysclk18_ck" },
    omap_clkctrl_reg_data { reg: DM816_MCSPI1_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk10_ck" },
    omap_clkctrl_reg_data { reg: DM816_MAILBOX_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk6_ck" },
    omap_clkctrl_reg_data { reg: DM816_SPINBOX_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk6_ck" },
    omap_clkctrl_reg_data { reg: DM816_MMC1_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk10_ck" },
    omap_clkctrl_reg_data { reg: DM816_GPMC_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk6_ck" },
    omap_clkctrl_reg_data { reg: DM816_DAVINCI_MDIO_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP | CLKF_NO_IDLEST, parent: "sysclk24_ck" },
    omap_clkctrl_reg_data { reg: DM816_EMAC1_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP | CLKF_NO_IDLEST, parent: "sysclk24_ck" },
    omap_clkctrl_reg_data { reg: DM816_MPU_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk2_ck" },
    omap_clkctrl_reg_data { reg: DM816_RTC_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP | CLKF_NO_IDLEST, parent: "sysclk18_ck" },
    omap_clkctrl_reg_data { reg: DM816_TPCC_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk4_ck" },
    omap_clkctrl_reg_data { reg: DM816_TPTC0_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk4_ck" },
    omap_clkctrl_reg_data { reg: DM816_TPTC1_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk4_ck" },
    omap_clkctrl_reg_data { reg: DM816_TPTC2_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk4_ck" },
    omap_clkctrl_reg_data { reg: DM816_TPTC3_CLKCTRL, bit: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk4_ck" },
    omap_clkctrl_reg_data { reg: 0, bit: core::ptr::null(), flags: 0, parent: core::ptr::null() },
];

#[no_mangle]
pub static DM816_CLKCTRL_DATA: [omap_clkctrl_data; 3] = [
    omap_clkctrl_data { addr: 0x48180500, regs: DM816_DEFAULT_CLKCTRL_REGS.as_ptr() },
    omap_clkctrl_data { addr: 0x48181400, regs: DM816_ALWON_CLKCTRL_REGS.as_ptr() },
    omap_clkctrl_data { addr: 0, regs: core::ptr::null() },
];

static mut DM816X_CLKS: [ti_dt_clk; 5] = [
    DT_CLK!(core::ptr::null(), "sys_clkin", "sys_clkin_ck"),
    DT_CLK!(core::ptr::null(), "timer_sys_ck", "sys_clkin_ck"),
    DT_CLK!(core::ptr::null(), "timer_32k_ck", "sysclk18_ck"),
    DT_CLK!(core::ptr::null(), "timer_ext_ck", "tclkin_ck"),
    ti_dt_clk { node_name: core::ptr::null(), con_id: core::ptr::null(), clk_name: core::ptr::null() },
];

static ENABLE_INIT_CLKS: [&str; 4] = [
    "ddr_pll_clk1", "ddr_pll_clk2", "ddr_pll_clk3", "sysclk6_ck",
];

pub unsafe extern "C" fn dm816x_dt_clk_init() -> i32 {
    ti_dt_clocks_register(DM816X_CLKS.as_mut_ptr());
    omap2_clk_disable_autoidle_all();
    ti_clk_add_aliases();
    omap2_clk_enable_init_clocks(ENABLE_INIT_CLKS.as_ptr(), ENABLE_INIT_CLKS.len());
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
