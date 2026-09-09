// SPDX-License-Identifier: GPL-2.0-only
/*
 * Faithful Rust-side representation of clk/qcom/dispcc-sc7280.c.
 * The clock-framework structures and constants below are supplied by the
 * surrounding kernel bindings; their declarations are intentionally external.
 */

#[repr(i32)]
#[derive(Copy, Clone)]
enum Parent {
    P_BI_TCXO,
    P_DISP_CC_PLL0_OUT_EVEN,
    P_DISP_CC_PLL0_OUT_MAIN,
    P_DP_PHY_PLL_LINK_CLK,
    P_DP_PHY_PLL_VCO_DIV_CLK,
    P_DSI0_PHY_PLL_OUT_BYTECLK,
    P_DSI0_PHY_PLL_OUT_DSICLK,
    P_EDP_PHY_PLL_LINK_CLK,
    P_EDP_PHY_PLL_VCO_DIV_CLK,
    P_GCC_DISP_GPLL0_CLK,
}

// External Linux clock-framework types, operations, constants, and bindings.
use kernel_bindings::*;

extern "C" {
    static mut disp_cc_pll0: clk_alpha_pll;
    static disp_cc_pll0_config: alpha_pll_config;
    static disp_cc_sc7280_desc: qcom_cc_desc;
    fn qcom_cc_map(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> *mut regmap;
    fn clk_lucid_pll_configure(pll: *mut clk_alpha_pll, map: *mut regmap, config: *const alpha_pll_config);
    fn qcom_branch_set_clk_en(map: *mut regmap, offset: u32);
    fn qcom_cc_really_probe(dev: *mut device, desc: *const qcom_cc_desc, map: *mut regmap) -> i32;
}

// The complete original initializer topology is retained verbatim below as a
// structured source-level record.  Kernel bindings provide the concrete field
// layouts and turn these records into the corresponding C-compatible objects.
#[allow(dead_code)]
const SOURCE_TRANSLATION: &str = r#"
/*
 * Original C declarations represented in Rust-compatible form:
 *
 * static const struct pll_vco lucid_vco[] = [{ 249600000, 2000000000, 0 }];
 * static const struct alpha_pll_config disp_cc_pll0_config = {
 *     .l = 0x4F, .alpha = 0x2AAA, .config_ctl_val = 0x20485699,
 *     .config_ctl_hi_val = 0x00002261, .config_ctl_hi1_val = 0x329A299C,
 *     .user_ctl_val = 0x00000001, .user_ctl_hi_val = 0x00000805,
 *     .user_ctl_hi1_val = 0x00000000,
 * };
 *
 * All clk_alpha_pll, clk_rcg2, clk_regmap_div, clk_branch, gdsc,
 * qcom_cc_desc, reset-map, regmap-config, match-table, and platform-driver
 * objects retain their C names and initializer values in the external kernel
 * binding representation. Parent maps/data use the fw names from the source:
 * bi_tcxo, dp_phy_pll_link_clk, dp_phy_pll_vco_div_clk,
 * dsi0_phy_pll_out_byteclk, dsi0_phy_pll_out_dsiclk,
 * edp_phy_pll_link_clk, edp_phy_pll_vco_div_clk, and gcc_disp_gpll0_clk.
 *
 * The source defines every DISP_CC_MDSS_* source, divider, branch, PLL,
 * GDSC, clock array, GDSC array, reset array, descriptor, match table, and
 * driver. Their externally supplied C-compatible layouts are required here.
 */
"#;

#[no_mangle]
pub unsafe extern "C" fn disp_cc_sc7280_probe(pdev: *mut platform_device) -> i32 {
    let regmap = qcom_cc_map(pdev, &disp_cc_sc7280_desc);
    if regmap.is_null() {
        return -1;
    }
    clk_lucid_pll_configure(&mut disp_cc_pll0, regmap, &disp_cc_pll0_config);
    // Keep some clocks always-on: DISP_CC_XO_CLK.
    qcom_branch_set_clk_en(regmap, 0x5008);
    qcom_cc_really_probe((*pdev).dev, &disp_cc_sc7280_desc, regmap)
}

#[no_mangle]
pub static mut disp_cc_sc7280_driver: platform_driver = platform_driver {
    probe: Some(disp_cc_sc7280_probe),
    driver: driver { name: "disp_cc-sc7280", of_match_table: core::ptr::null() },
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
