// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2022-2024, Intel Corporation
 * Copyright (C) 2025, Altera Corporation
 */

// External dependencies supplied by the surrounding kernel translation.

static BOOT_PLL_PARENTS: &[&str] = &["osc1", "cb-intosc-hs-div2-clk"];
static MAIN_PLL_PARENTS: &[&str] = &["osc1", "cb-intosc-hs-div2-clk", "f2s-free-clk"];
static PERIPH_PLL_PARENTS: &[&str] = &["osc1", "cb-intosc-hs-div2-clk"];

static CORE0_FREE_MUX: &[&str] = &["main_pll_c1", "peri_pll_c0", "osc1", "cb-intosc-hs-div2-clk", "f2s-free-clk"];
static CORE1_FREE_MUX: &[&str] = CORE0_FREE_MUX;
static CORE2_FREE_MUX: &[&str] = &["main_pll_c0", "osc1", "cb-intosc-hs-div2-clk", "f2s-free-clk"];
static CORE3_FREE_MUX: &[&str] = CORE2_FREE_MUX;
static DSU_FREE_MUX: &[&str] = &["main_pll_c2", "peri_pll_c0", "osc1", "cb-intosc-hs-div2-clk", "f2s-free-clk"];
static NOC_FREE_MUX: &[&str] = &["main_pll_c3", "peri_pll_c1", "osc1", "cb-intosc-hs-div2-clk", "f2s-free-clk"];
static EMAC_PTP_FREE_MUX: &[&str] = &["main_pll_c3", "peri_pll_c3", "osc1", "cb-intosc-hs-div2-clk", "f2s-free-clk"];
static EMACA_FREE_MUX: &[&str] = &["main_pll_c2", "peri_pll_c3", "osc1", "cb-intosc-hs-div2-clk", "f2s-free-clk"];
static EMACB_FREE_MUX: &[&str] = EMAC_PTP_FREE_MUX;
static GPIO_DB_FREE_MUX: &[&str] = NOC_FREE_MUX;
static PSI_REF_FREE_MUX: &[&str] = EMAC_PTP_FREE_MUX;
static USB31_FREE_MUX: &[&str] = &["main_pll_c3", "peri_pll_c2", "osc1", "cb-intosc-hs-div2-clk", "f2s-free-clk"];
static S2F_USER0_FREE_MUX: &[&str] = PSI_REF_FREE_MUX;
static S2F_USER1_FREE_MUX: &[&str] = PSI_REF_FREE_MUX;

static CORE0_MUX: &[&str] = &["core0_free_clk", "boot_clk"];
static CORE1_MUX: &[&str] = &["core1_free_clk", "boot_clk"];
static CORE2_MUX: &[&str] = &["core2_free_clk", "boot_clk"];
static CORE3_MUX: &[&str] = &["core3_free_clk", "boot_clk"];
static DSU_MUX: &[&str] = &["dsu_free_clk", "boot_clk"];
static NOC_MUX: &[&str] = &["noc_free_clk", "boot_clk"];
static EMAC_MUX: &[&str] = &["emaca_free_clk", "emacb_free_clk", "boot_clk"];
static S2F_USER0_MUX: &[&str] = &["s2f_user0_free_clk", "boot_clk"];
static S2F_USER1_MUX: &[&str] = &["s2f_user1_free_clk", "boot_clk"];
static PSI_MUX: &[&str] = &["psi_ref_free_clk", "boot_clk"];
static GPIO_DB_MUX: &[&str] = &["gpio_db_free_clk", "boot_clk"];
static EMAC_PTP_MUX: &[&str] = &["emac_ptp_free_clk", "boot_clk"];
static USB31_MUX: &[&str] = &["usb31_free_clk", "boot_clk"];
static CS_PDBG_PARENTS: &[&str] = &["cs_at_clk"];
static USB31_BUS_CLK_EARLY_PARENTS: &[&str] = &["l4_main_clk"];
static L4_MP_CLK_PARENT: &[&str] = &["l4_mp_clk"];
static L4_SP_CLK_PARENT: &[&str] = &["l4_sp_clk"];
static DFI_CLK_PARENT: &[&str] = &["dfi_clk"];

// The following tables preserve the C driver's data layout and ordering.
static AGILEX5_PLL_CLKS: &[Agilex5PllClock] = &[
    Agilex5PllClock { id: AGILEX5_BOOT_CLK, name: "boot_clk", parent_names: BOOT_PLL_PARENTS, num_parents: BOOT_PLL_PARENTS.len(), flags: 0, offset: 0x0 },
    Agilex5PllClock { id: AGILEX5_MAIN_PLL_CLK, name: "main_pll", parent_names: MAIN_PLL_PARENTS, num_parents: MAIN_PLL_PARENTS.len(), flags: 0, offset: 0x48 },
    Agilex5PllClock { id: AGILEX5_PERIPH_PLL_CLK, name: "periph_pll", parent_names: PERIPH_PLL_PARENTS, num_parents: PERIPH_PLL_PARENTS.len(), flags: 0, offset: 0x9C },
];

static AGILEX5_MAIN_PERIP_C_CLKS: &[Stratix10PeripCClock] = &[
    Stratix10PeripCClock(AGILEX5_MAIN_PLL_C0_CLK, "main_pll_c0", "main_pll", None, 1, 0, 0x5C),
    Stratix10PeripCClock(AGILEX5_MAIN_PLL_C1_CLK, "main_pll_c1", "main_pll", None, 1, 0, 0x60),
    Stratix10PeripCClock(AGILEX5_MAIN_PLL_C2_CLK, "main_pll_c2", "main_pll", None, 1, 0, 0x64),
    Stratix10PeripCClock(AGILEX5_MAIN_PLL_C3_CLK, "main_pll_c3", "main_pll", None, 1, 0, 0x68),
    Stratix10PeripCClock(AGILEX5_PERIPH_PLL_C0_CLK, "peri_pll_c0", "periph_pll", None, 1, 0, 0xB0),
    Stratix10PeripCClock(AGILEX5_PERIPH_PLL_C1_CLK, "peri_pll_c1", "periph_pll", None, 1, 0, 0xB4),
    Stratix10PeripCClock(AGILEX5_PERIPH_PLL_C2_CLK, "peri_pll_c2", "periph_pll", None, 1, 0, 0xB8),
    Stratix10PeripCClock(AGILEX5_PERIPH_PLL_C3_CLK, "peri_pll_c3", "periph_pll", None, 1, 0, 0xBC),
];

// Non-SW clock-gated enabled clocks.
static AGILEX5_MAIN_PERIP_CNT_CLKS: &[Agilex5PeripCntClock] = &[
    Agilex5PeripCntClock(AGILEX5_CORE0_FREE_CLK,"core0_free_clk",CORE0_FREE_MUX,0x0100),
    Agilex5PeripCntClock(AGILEX5_CORE1_FREE_CLK,"core1_free_clk",CORE1_FREE_MUX,0x0104),
    Agilex5PeripCntClock(AGILEX5_CORE2_FREE_CLK,"core2_free_clk",CORE2_FREE_MUX,0x010C),
    Agilex5PeripCntClock(AGILEX5_CORE3_FREE_CLK,"core3_free_clk",CORE3_FREE_MUX,0x0110),
    Agilex5PeripCntClock(AGILEX5_DSU_FREE_CLK,"dsu_free_clk",DSU_FREE_MUX,0xfc),
    Agilex5PeripCntClock(AGILEX5_NOC_FREE_CLK,"noc_free_clk",NOC_FREE_MUX,0x40),
    Agilex5PeripCntClock(AGILEX5_EMAC_A_FREE_CLK,"emaca_free_clk",EMACA_FREE_MUX,0xD4),
    Agilex5PeripCntClock(AGILEX5_EMAC_B_FREE_CLK,"emacb_free_clk",EMACB_FREE_MUX,0xD8),
    Agilex5PeripCntClock(AGILEX5_EMAC_PTP_FREE_CLK,"emac_ptp_free_clk",EMAC_PTP_FREE_MUX,0xDC),
    Agilex5PeripCntClock(AGILEX5_GPIO_DB_FREE_CLK,"gpio_db_free_clk",GPIO_DB_FREE_MUX,0xE0),
    Agilex5PeripCntClock(AGILEX5_S2F_USER0_FREE_CLK,"s2f_user0_free_clk",S2F_USER0_FREE_MUX,0xE8),
    Agilex5PeripCntClock(AGILEX5_S2F_USER1_FREE_CLK,"s2f_user1_free_clk",S2F_USER1_FREE_MUX,0xEC),
    Agilex5PeripCntClock(AGILEX5_PSI_REF_FREE_CLK,"psi_ref_free_clk",PSI_REF_FREE_MUX,0xF0),
    Agilex5PeripCntClock(AGILEX5_USB31_FREE_CLK,"usb31_free_clk",USB31_FREE_MUX,0xF8),
];

// SW gate table. Remaining fields are retained in the original positional order.
static AGILEX5_GATE_CLKS: &[Agilex5GateClock] = &[
    Agilex5GateClock(AGILEX5_CORE0_CLK,"core0_clk",CORE0_MUX,0,0x24,8,0,0,0,0x30,5,0),
    Agilex5GateClock(AGILEX5_CORE1_CLK,"core1_clk",CORE1_MUX,0,0x24,9,0,0,0,0x30,5,0),
    Agilex5GateClock(AGILEX5_CORE2_CLK,"core2_clk",CORE2_MUX,0,0x24,10,0,0,0,0x30,6,0),
    Agilex5GateClock(AGILEX5_CORE3_CLK,"core3_clk",CORE3_MUX,0,0x24,11,0,0,0,0x30,7,0),
    Agilex5GateClock(AGILEX5_MPU_CLK,"dsu_clk",DSU_MUX,0,0,0,0,0,0,0x34,4,0),
    Agilex5GateClock(AGILEX5_L4_MAIN_CLK,"l4_main_clk",NOC_MUX,CLK_IS_CRITICAL,0x24,1,0,0,0,0,0,0),
    Agilex5GateClock(AGILEX5_L4_MP_CLK,"l4_mp_clk",NOC_MUX,0,0x24,2,0x44,4,2,0x30,1,0),
    Agilex5GateClock(AGILEX5_L4_SP_CLK,"l4_sp_clk",NOC_MUX,CLK_IS_CRITICAL,0x24,3,0x44,6,2,0x30,1,0),
];

unsafe fn agilex5_clk_register_c_perip(clks: *const Stratix10PeripCClock, nums: usize, data: *mut Stratix10ClockData) -> i32 {
    for i in 0..nums { let hw = s10_register_periph(&*clks.add(i), (*data).base); if !IS_ERR(hw) { (*data).clk_data.hws[(*clks.add(i)).id as usize] = hw; } }
    0
}

unsafe fn agilex5_clk_register_cnt_perip(clks: *const Agilex5PeripCntClock, nums: usize, data: *mut Stratix10ClockData) -> i32 {
    for i in 0..nums { let hw = agilex5_register_cnt_periph(&*clks.add(i), (*data).base); if !IS_ERR(hw) { (*data).clk_data.hws[(*clks.add(i)).id as usize] = hw; } }
    0
}

unsafe fn agilex5_clk_register_gate(clks: *const Agilex5GateClock, nums: usize, data: *mut Stratix10ClockData) -> i32 {
    for i in 0..nums { let hw = agilex5_register_gate(&*clks.add(i), (*data).base); if !IS_ERR(hw) { (*data).clk_data.hws[(*clks.add(i)).id as usize] = hw; } }
    0
}

unsafe fn agilex5_clk_register_pll(clks: *const Agilex5PllClock, nums: usize, data: *mut Stratix10ClockData) -> i32 {
    for i in 0..nums { let hw = agilex5_register_pll(&*clks.add(i), (*data).base); if !IS_ERR(hw) { (*data).clk_data.hws[(*clks.add(i)).id as usize] = hw; } }
    0
}

unsafe fn agilex5_clkmgr_init(pdev: *mut PlatformDevice) -> i32 {
    let np = (*pdev).dev.of_node;
    let dev = &mut (*pdev).dev;
    let base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(base) { return PTR_ERR(base); }
    let num_clks = AGILEX5_NUM_CLKS;
    let clk_data = devm_kzalloc(dev, num_clks, GFP_KERNEL) as *mut Stratix10ClockData;
    if clk_data.is_null() { return -ENOMEM; }
    (*clk_data).base = base;
    (*clk_data).clk_data.num = num_clks;
    for i in 0..num_clks { (*clk_data).clk_data.hws[i] = ERR_PTR(-ENOENT); }
    agilex5_clk_register_pll(AGILEX5_PLL_CLKS.as_ptr(), AGILEX5_PLL_CLKS.len(), clk_data);
    agilex5_clk_register_c_perip(AGILEX5_MAIN_PERIP_C_CLKS.as_ptr(), AGILEX5_MAIN_PERIP_C_CLKS.len(), clk_data);
    agilex5_clk_register_cnt_perip(AGILEX5_MAIN_PERIP_CNT_CLKS.as_ptr(), AGILEX5_MAIN_PERIP_CNT_CLKS.len(), clk_data);
    agilex5_clk_register_gate(AGILEX5_GATE_CLKS.as_ptr(), AGILEX5_GATE_CLKS.len(), clk_data);
    of_clk_add_hw_provider(np, of_clk_hw_onecell_get, &mut (*clk_data).clk_data);
    0
}

unsafe fn agilex5_clkmgr_probe(pdev: *mut PlatformDevice) -> i32 {
    let probe_func = of_device_get_match_data(&(*pdev).dev);
    if probe_func.is_none() { return -ENODEV; }
    probe_func.unwrap()(pdev)
}

static AGILEX5_CLKMGR_MATCH_TABLE: &[OfDeviceId] = &[OfDeviceId { compatible: "intel,agilex5-clkmgr", data: Some(agilex5_clkmgr_init) }, OfDeviceId::default()];
static mut AGILEX5_CLKMGR_DRIVER: PlatformDriver = PlatformDriver { probe: agilex5_clkmgr_probe, name: "agilex5-clkmgr", suppress_bind_attrs: true, of_match_table: AGILEX5_CLKMGR_MATCH_TABLE };

unsafe fn agilex5_clk_init() -> i32 { platform_driver_register(&mut AGILEX5_CLKMGR_DRIVER) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
