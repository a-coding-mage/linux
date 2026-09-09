// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel translation.

static DM814_DEFAULT_CLKCTRL_REGS: [omap_clkctrl_reg_data; 2] = [
    omap_clkctrl_reg_data { offset: DM814_USB_OTG_HS_CLKCTRL, optclks: core::ptr::null(), flags: CLKF_SW_SUP, parent: "pll260dcoclkldo" },
    omap_clkctrl_reg_data { offset: 0, optclks: core::ptr::null(), flags: 0, parent: core::ptr::null() },
];

static DM814_ALWON_CLKCTRL_REGS: [omap_clkctrl_reg_data; 20] = [
    omap_clkctrl_reg_data { offset: DM814_UART1_CLKCTRL, optclks: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk10_ck" },
    omap_clkctrl_reg_data { offset: DM814_UART2_CLKCTRL, optclks: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk10_ck" },
    omap_clkctrl_reg_data { offset: DM814_UART3_CLKCTRL, optclks: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk10_ck" },
    omap_clkctrl_reg_data { offset: DM814_GPIO1_CLKCTRL, optclks: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk6_ck" },
    omap_clkctrl_reg_data { offset: DM814_GPIO2_CLKCTRL, optclks: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk6_ck" },
    omap_clkctrl_reg_data { offset: DM814_I2C1_CLKCTRL, optclks: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk10_ck" },
    omap_clkctrl_reg_data { offset: DM814_I2C2_CLKCTRL, optclks: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk10_ck" },
    omap_clkctrl_reg_data { offset: DM814_WD_TIMER_CLKCTRL, optclks: core::ptr::null(), flags: CLKF_SW_SUP | CLKF_NO_IDLEST, parent: "sysclk18_ck" },
    omap_clkctrl_reg_data { offset: DM814_MCSPI1_CLKCTRL, optclks: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk10_ck" },
    omap_clkctrl_reg_data { offset: DM814_GPMC_CLKCTRL, optclks: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk6_ck" },
    omap_clkctrl_reg_data { offset: DM814_MPU_CLKCTRL, optclks: core::ptr::null(), flags: CLKF_SW_SUP, parent: "mpu_ck" },
    omap_clkctrl_reg_data { offset: DM814_RTC_CLKCTRL, optclks: core::ptr::null(), flags: CLKF_SW_SUP | CLKF_NO_IDLEST, parent: "sysclk18_ck" },
    omap_clkctrl_reg_data { offset: DM814_TPCC_CLKCTRL, optclks: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk4_ck" },
    omap_clkctrl_reg_data { offset: DM814_TPTC0_CLKCTRL, optclks: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk4_ck" },
    omap_clkctrl_reg_data { offset: DM814_TPTC1_CLKCTRL, optclks: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk4_ck" },
    omap_clkctrl_reg_data { offset: DM814_TPTC2_CLKCTRL, optclks: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk4_ck" },
    omap_clkctrl_reg_data { offset: DM814_TPTC3_CLKCTRL, optclks: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk4_ck" },
    omap_clkctrl_reg_data { offset: DM814_MMC1_CLKCTRL, optclks: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk8_ck" },
    omap_clkctrl_reg_data { offset: DM814_MMC2_CLKCTRL, optclks: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk8_ck" },
    omap_clkctrl_reg_data { offset: DM814_MMC3_CLKCTRL, optclks: core::ptr::null(), flags: CLKF_SW_SUP, parent: "sysclk8_ck" },
    omap_clkctrl_reg_data { offset: 0, optclks: core::ptr::null(), flags: 0, parent: core::ptr::null() },
];

static DM814_ALWON_ETHERNET_CLKCTRL_REGS: [omap_clkctrl_reg_data; 1] = [
    omap_clkctrl_reg_data { offset: 0, optclks: core::ptr::null(), flags: CLKF_SW_SUP, parent: "cpsw_125mhz_gclk" },
];

static DM814_CLKCTRL_DATA: [omap_clkctrl_data; 4] = [
    omap_clkctrl_data { addr: 0x48180500, regs: DM814_DEFAULT_CLKCTRL_REGS.as_ptr() },
    omap_clkctrl_data { addr: 0x48181400, regs: DM814_ALWON_CLKCTRL_REGS.as_ptr() },
    omap_clkctrl_data { addr: 0x481815d4, regs: DM814_ALWON_ETHERNET_CLKCTRL_REGS.as_ptr() },
    omap_clkctrl_data { addr: 0, regs: core::ptr::null() },
];

static mut DM814_CLKS: [ti_dt_clk; 2] = [
    DT_CLK!(core::ptr::null(), "timer_sys_ck", "devosc_ck"),
    ti_dt_clk { node_name: core::ptr::null() },
];

static mut TIMER_CLOCKS_INITIALIZED: bool = false;

unsafe fn dm814x_adpll_early_init() -> i32 {
    if !TIMER_CLOCKS_INITIALIZED { return -ENODEV; }
    let np = of_find_node_by_name(core::ptr::null_mut(), "pllss");
    if np.is_null() {
        pr_err!("Could not find node for plls\n");
        return -ENODEV;
    }
    of_platform_populate(np, core::ptr::null(), core::ptr::null(), core::ptr::null());
    of_node_put(np);
    0
}

// core_initcall(dm814x_adpll_early_init);

static INIT_CLOCKS: [&str; 2] = [
    "pll040clkout", // MPU 481c5040.adpll.clkout
    "pll290clkout", // DDR 481c5290.adpll.clkout
];

unsafe fn dm814x_adpll_enable_init_clocks() -> i32 {
    if !TIMER_CLOCKS_INITIALIZED { return -ENODEV; }
    for name in INIT_CLOCKS.iter() {
        let clock = clk_get(core::ptr::null_mut(), *name);
        if WARN!(IS_ERR(clock), "could not find init clock %s\n", *name) { continue; }
        let err = clk_prepare_enable(clock);
        if WARN!(err != 0, "could not enable init clock %s\n", *name) { continue; }
    }
    0
}

// postcore_initcall(dm814x_adpll_enable_init_clocks);

unsafe fn dm814x_dt_clk_init() -> i32 {
    ti_dt_clocks_register(DM814_CLKS.as_mut_ptr());
    omap2_clk_disable_autoidle_all();
    ti_clk_add_aliases();
    omap2_clk_enable_init_clocks(core::ptr::null_mut(), 0);
    TIMER_CLOCKS_INITIALIZED = true;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
