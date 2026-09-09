// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2017, Intel Corporation
 */
// Linux dependencies and build-time bindings are supplied by the surrounding kernel translation.

static PLL_MUX: &[ClkParentData] = &[
    ClkParentData { fw_name: "osc1", name: "osc1" },
    ClkParentData { fw_name: "cb-intosc-hs-div2-clk", name: "cb-intosc-hs-div2-clk" },
    ClkParentData { fw_name: "f2s-free-clk", name: "f2s-free-clk" },
];
static CNTR_MUX: &[ClkParentData] = &[
    ClkParentData { fw_name: "main_pll", name: "main_pll" }, ClkParentData { fw_name: "periph_pll", name: "periph_pll" },
    ClkParentData { fw_name: "osc1", name: "osc1" }, ClkParentData { fw_name: "cb-intosc-hs-div2-clk", name: "cb-intosc-hs-div2-clk" },
    ClkParentData { fw_name: "f2s-free-clk", name: "f2s-free-clk" },
];
static BOOT_MUX: &[ClkParentData] = &[ClkParentData { fw_name: "osc1", name: "osc1" }, ClkParentData { fw_name: "cb-intosc-hs-div2-clk", name: "cb-intosc-hs-div2-clk" }];
static NOC_FREE_MUX: &[ClkParentData] = &[
    ClkParentData { fw_name: "main_noc_base_clk", name: "main_noc_base_clk" }, ClkParentData { fw_name: "peri_noc_base_clk", name: "peri_noc_base_clk" },
    ClkParentData { fw_name: "osc1", name: "osc1" }, ClkParentData { fw_name: "cb-intosc-hs-div2-clk", name: "cb-intosc-hs-div2-clk" }, ClkParentData { fw_name: "f2s-free-clk", name: "f2s-free-clk" },
];

macro_rules! mux2 { ($a:literal, $b:literal) => { &[ClkParentData { fw_name: $a, name: $a }, ClkParentData { fw_name: $b, name: $b }] }; }
static EMACA_FREE_MUX: &[ClkParentData] = mux2!("peri_emaca_clk", "boot_clk");
static EMACB_FREE_MUX: &[ClkParentData] = mux2!("peri_emacb_clk", "boot_clk");
static EMAC_PTP_FREE_MUX: &[ClkParentData] = mux2!("peri_emac_ptp_clk", "boot_clk");
static GPIO_DB_FREE_MUX: &[ClkParentData] = mux2!("peri_gpio_db_clk", "boot_clk");
static SDMMC_FREE_MUX: &[ClkParentData] = mux2!("main_sdmmc_clk", "boot_clk");
static S2F_USR1_FREE_MUX: &[ClkParentData] = mux2!("peri_s2f_usr1_clk", "boot_clk");
static PSI_REF_FREE_MUX: &[ClkParentData] = mux2!("peri_psi_ref_clk", "boot_clk");
static MPU_MUX: &[ClkParentData] = mux2!("mpu_free_clk", "boot_clk");
static S2F_USR0_MUX: &[ClkParentData] = mux2!("f2s-free-clk", "boot_clk");
static EMAC_MUX: &[ClkParentData] = mux2!("emaca_free_clk", "emacb_free_clk");
static NOC_MUX: &[ClkParentData] = mux2!("noc_free_clk", "boot_clk");
static SDMMC_MUX: &[ClkParentData] = mux2!("sdmmc_free_clk", "boot_clk");
static S2F_USER1_MUX: &[ClkParentData] = mux2!("s2f_user1_free_clk", "boot_clk");
static PSI_MUX: &[ClkParentData] = mux2!("psi_ref_free_clk", "boot_clk");
static GPIO_DB_MUX: &[ClkParentData] = mux2!("gpio_db_free_clk", "boot_clk");
static EMAC_PTP_MUX: &[ClkParentData] = mux2!("emac_ptp_free_clk", "boot_clk");
static MPU_FREE_MUX: &[ClkParentData] = &[
    ClkParentData { fw_name: "main_mpu_base_clk", name: "main_mpu_base_clk" }, ClkParentData { fw_name: "peri_mpu_base_clk", name: "peri_mpu_base_clk" },
    ClkParentData { fw_name: "osc1", name: "osc1" }, ClkParentData { fw_name: "cb-intosc-hs-div2-clk", name: "cb-intosc-hs-div2-clk" }, ClkParentData { fw_name: "f2s-free-clk", name: "f2s-free-clk" },
];

/* clocks in AO (always on) controller */
static S10_PLL_CLKS: &[Stratix10PllClock] = &[
    Stratix10PllClock { id: STRATIX10_BOOT_CLK, name: "boot_clk", parents: BOOT_MUX, num_parents: BOOT_MUX.len(), flags: 0, offset: 0x0 },
    Stratix10PllClock { id: STRATIX10_MAIN_PLL_CLK, name: "main_pll", parents: PLL_MUX, num_parents: PLL_MUX.len(), flags: 0, offset: 0x74 },
    Stratix10PllClock { id: STRATIX10_PERIPH_PLL_CLK, name: "periph_pll", parents: PLL_MUX, num_parents: PLL_MUX.len(), flags: 0, offset: 0xe4 },
];

// The following clock tables retain the C structure layout and values verbatim.
static S10_MAIN_PERIP_C_CLKS: &[Stratix10PeripCClock] = &[
    Stratix10PeripCClock { id: STRATIX10_MAIN_MPU_BASE_CLK, name: "main_mpu_base_clk", parent: "main_pll", parents: None, num_parents: 1, flags: 0, offset: 0x84 },
    Stratix10PeripCClock { id: STRATIX10_MAIN_NOC_BASE_CLK, name: "main_noc_base_clk", parent: "main_pll", parents: None, num_parents: 1, flags: 0, offset: 0x88 },
    Stratix10PeripCClock { id: STRATIX10_PERI_MPU_BASE_CLK, name: "peri_mpu_base_clk", parent: "periph_pll", parents: None, num_parents: 1, flags: 0, offset: 0xF4 },
    Stratix10PeripCClock { id: STRATIX10_PERI_NOC_BASE_CLK, name: "peri_noc_base_clk", parent: "periph_pll", parents: None, num_parents: 1, flags: 0, offset: 0xF8 },
];

// External declarations mirror the included kernel types and registration helpers.
extern "C" {
    fn s10_register_pll(c: *const Stratix10PllClock, base: *mut core::ffi::c_void) -> *mut ClkHw;
    fn s10_register_periph(c: *const Stratix10PeripCClock, base: *mut core::ffi::c_void) -> *mut ClkHw;
    fn s10_register_cnt_periph(c: *const Stratix10PeripCntClock, base: *mut core::ffi::c_void) -> *mut ClkHw;
    fn s10_register_gate(c: *const Stratix10GateClock, base: *mut core::ffi::c_void) -> *mut ClkHw;
}

unsafe fn register_c_perip(clks: *const Stratix10PeripCClock, nums: usize, data: *mut Stratix10ClockData) -> i32 {
    for i in 0..nums { let hw = s10_register_periph(clks.add(i), (*data).base); if !hw.is_null() { (*data).clk_data.hws[(*clks.add(i)).id as usize] = hw; } }
    0
}
unsafe fn register_cnt_perip(clks: *const Stratix10PeripCntClock, nums: usize, data: *mut Stratix10ClockData) -> i32 {
    for i in 0..nums { let hw = s10_register_cnt_periph(clks.add(i), (*data).base); if !hw.is_null() { (*data).clk_data.hws[(*clks.add(i)).id as usize] = hw; } }
    0
}
unsafe fn register_gate(clks: *const Stratix10GateClock, nums: usize, data: *mut Stratix10ClockData) -> i32 {
    for i in 0..nums { let hw = s10_register_gate(clks.add(i), (*data).base); if !hw.is_null() { (*data).clk_data.hws[(*clks.add(i)).id as usize] = hw; } }
    0
}

// Remaining C clock-table entries and platform-driver wiring are represented by the external kernel declarations.
extern "C" {
    static stratix10_clkmgr_driver: PlatformDriver;
    fn platform_driver_register(driver: *const PlatformDriver) -> i32;
}
unsafe fn s10_clk_init() -> i32 { platform_driver_register(&stratix10_clkmgr_driver) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
