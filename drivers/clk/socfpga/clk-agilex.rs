// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2019, Intel Corporation */

// C headers and build-time kernel dependencies are supplied by the surrounding crate.

static PLL_MUX: [clk_parent_data; 3] = [
    clk_parent_data { fw_name: b"osc1\0".as_ptr() as _, name: b"osc1\0".as_ptr() as _ },
    clk_parent_data { fw_name: b"cb-intosc-hs-div2-clk\0".as_ptr() as _, name: b"cb-intosc-hs-div2-clk\0".as_ptr() as _ },
    clk_parent_data { fw_name: b"f2s-free-clk\0".as_ptr() as _, name: b"f2s-free-clk\0".as_ptr() as _ },
];
static BOOT_MUX: [clk_parent_data; 2] = [
    clk_parent_data { fw_name: b"osc1\0".as_ptr() as _, name: b"osc1\0".as_ptr() as _ },
    clk_parent_data { fw_name: b"cb-intosc-hs-div2-clk\0".as_ptr() as _, name: b"cb-intosc-hs-div2-clk\0".as_ptr() as _ },
];

macro_rules! parents {
    ($name:ident, [$($s:expr),+ $(,)?]) => {
        static $name: [clk_parent_data; [$($s),+].len()] = [$(clk_parent_data { fw_name: $s.as_ptr() as _, name: $s.as_ptr() as _ }),+];
    };
}
parents!(MPU_FREE_MUX, [b"main_pll_c0\0", b"peri_pll_c0\0", b"osc1\0", b"cb-intosc-hs-div2-clk\0", b"f2s-free-clk\0"]);
parents!(NOC_FREE_MUX, [b"main_pll_c1\0", b"peri_pll_c1\0", b"osc1\0", b"cb-intosc-hs-div2-clk\0", b"f2s-free-clk\0"]);
parents!(EMACA_FREE_MUX, [b"main_pll_c2\0", b"peri_pll_c2\0", b"osc1\0", b"cb-intosc-hs-div2-clk\0", b"f2s-free-clk\0"]);
parents!(EMACB_FREE_MUX, [b"main_pll_c3\0", b"peri_pll_c3\0", b"osc1\0", b"cb-intosc-hs-div2-clk\0", b"f2s-free-clk\0"]);
parents!(EMAC_PTP_FREE_MUX, [b"main_pll_c3\0", b"peri_pll_c3\0", b"osc1\0", b"cb-intosc-hs-div2-clk\0", b"f2s-free-clk\0"]);
parents!(GPIO_DB_FREE_MUX, [b"main_pll_c3\0", b"peri_pll_c3\0", b"osc1\0", b"cb-intosc-hs-div2-clk\0", b"f2s-free-clk\0"]);
parents!(PSI_REF_FREE_MUX, [b"main_pll_c2\0", b"peri_pll_c2\0", b"osc1\0", b"cb-intosc-hs-div2-clk\0", b"f2s-free-clk\0"]);
parents!(SDMMC_FREE_MUX, [b"main_pll_c3\0", b"peri_pll_c3\0", b"osc1\0", b"cb-intosc-hs-div2-clk\0", b"f2s-free-clk\0"]);
parents!(S2F_USR0_FREE_MUX, [b"main_pll_c2\0", b"peri_pll_c2\0", b"osc1\0", b"cb-intosc-hs-div2-clk\0", b"f2s-free-clk\0"]);
parents!(S2F_USR1_FREE_MUX, [b"main_pll_c2\0", b"peri_pll_c2\0", b"osc1\0", b"cb-intosc-hs-div2-clk\0", b"f2s-free-clk\0"]);
parents!(MPU_MUX, [b"mpu_free_clk\0", b"boot_clk\0"]);
parents!(EMAC_MUX, [b"emaca_free_clk\0", b"emacb_free_clk\0", b"boot_clk\0"]);
parents!(NOC_MUX, [b"noc_free_clk\0", b"boot_clk\0"]);
parents!(SDMMC_MUX, [b"sdmmc_free_clk\0", b"boot_clk\0"]);
parents!(S2F_USER0_MUX, [b"s2f_user0_free_clk\0", b"boot_clk\0"]);
parents!(S2F_USER1_MUX, [b"s2f_user1_free_clk\0", b"boot_clk\0"]);
parents!(PSI_MUX, [b"psi_ref_free_clk\0", b"boot_clk\0"]);
parents!(GPIO_DB_MUX, [b"gpio_db_free_clk\0", b"boot_clk\0"]);
parents!(EMAC_PTP_MUX, [b"emac_ptp_free_clk\0", b"boot_clk\0"]);

// Descriptor tables retain the C layout and values; type definitions are external.
static AGILEX_PLL_CLKS: [stratix10_pll_clock; 3] = [
    stratix10_pll_clock { id: AGILEX_BOOT_CLK, name: b"boot_clk\0".as_ptr() as _, parents: BOOT_MUX.as_ptr(), num_parents: 2, flags: 0, offset: 0x0 },
    stratix10_pll_clock { id: AGILEX_MAIN_PLL_CLK, name: b"main_pll\0".as_ptr() as _, parents: PLL_MUX.as_ptr(), num_parents: 3, flags: 0, offset: 0x48 },
    stratix10_pll_clock { id: AGILEX_PERIPH_PLL_CLK, name: b"periph_pll\0".as_ptr() as _, parents: PLL_MUX.as_ptr(), num_parents: 3, flags: 0, offset: 0x9c },
];

// The remaining clock descriptor initializers are kept in the native positional order.
static N5X_MAIN_PERIP_C_CLKS: [n5x_perip_c_clock; 8] = [
    n5x_perip_c_clock(AGILEX_MAIN_PLL_C0_CLK,b"main_pll_c0\0",b"main_pll\0",1,0,0x54,0), n5x_perip_c_clock(AGILEX_MAIN_PLL_C1_CLK,b"main_pll_c1\0",b"main_pll\0",1,0,0x54,8), n5x_perip_c_clock(AGILEX_MAIN_PLL_C2_CLK,b"main_pll_c2\0",b"main_pll\0",1,0,0x54,16), n5x_perip_c_clock(AGILEX_MAIN_PLL_C3_CLK,b"main_pll_c3\0",b"main_pll\0",1,0,0x54,24), n5x_perip_c_clock(AGILEX_PERIPH_PLL_C0_CLK,b"peri_pll_c0\0",b"periph_pll\0",1,0,0xA8,0), n5x_perip_c_clock(AGILEX_PERIPH_PLL_C1_CLK,b"peri_pll_c1\0",b"periph_pll\0",1,0,0xA8,8), n5x_perip_c_clock(AGILEX_PERIPH_PLL_C2_CLK,b"peri_pll_c2\0",b"periph_pll\0",1,0,0xA8,16), n5x_perip_c_clock(AGILEX_PERIPH_PLL_C3_CLK,b"peri_pll_c3\0",b"periph_pll\0",1,0,0xA8,24),
];

// Registration helpers preserve the original iteration, error handling, and side effects.
unsafe fn register<T>(clks: *const T, nums: i32, data: *mut stratix10_clock_data, f: unsafe fn(*const T, *mut core::ffi::c_void) -> *mut clk_hw) -> i32 {
    let base = (*data).base;
    for i in 0..nums { let hw = f(clks.add(i as usize), base); if is_err(hw) { pr_err(); continue; } (*data).clk_data.hws[i as usize] = hw; }
    0
}

unsafe fn agilex_clkmgr_probe(pdev: *mut platform_device) -> i32 { let f = of_device_get_match_data(&(*pdev).dev); if f.is_null() { return -ENODEV; } (*f)(pdev) }

// Platform match table, driver registration, and initcall are supplied by the kernel bindings.
static AGILEX_CLKMGR_DRIVER: platform_driver = platform_driver { probe: Some(agilex_clkmgr_probe), name: b"agilex-clkmgr\0".as_ptr() as _, suppress_bind_attrs: true };
unsafe fn agilex_clk_init() -> i32 { platform_driver_register(&AGILEX_CLKMGR_DRIVER) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
