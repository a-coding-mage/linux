// SPDX-License-Identifier: GPL-2.0-only
//
// Source-level Rust representation of the Qualcomm IPQ8074 GCC driver.
// The Linux clock-framework types, constants, and operations referenced by
// this translation are supplied by the surrounding kernel bindings.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/// External kernel dependencies used by this implementation.
extern "C" {
    pub fn qcom_cc_map(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> *mut regmap;
    pub fn qcom_cc_really_probe(dev: *mut device, desc: *const qcom_cc_desc, map: *mut regmap) -> i32;
    pub fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    pub fn clk_alpha_pll_configure(pll: *mut clk_alpha_pll, map: *mut regmap, config: *const pll_config);
    pub fn platform_driver_register(driver: *mut platform_driver) -> i32;
    pub fn platform_driver_unregister(driver: *mut platform_driver);
}

#[repr(C)] pub struct platform_device { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct clk_alpha_pll { _private: [u8; 0] }
#[repr(C)] pub struct pll_config { _private: [u8; 0] }
#[repr(C)] pub struct qcom_cc_desc { _private: [u8; 0] }
#[repr(C)] pub struct platform_driver { _private: [u8; 0] }

pub const P_XO: u32 = 0;
pub const P_GPLL0: u32 = 1;
pub const P_GPLL0_DIV2: u32 = 2;
pub const P_GPLL2: u32 = 3;
pub const P_GPLL4: u32 = 4;
pub const P_GPLL6: u32 = 5;
pub const P_SLEEP_CLK: u32 = 6;
pub const P_PCIE20_PHY0_PIPE: u32 = 7;
pub const P_PCIE20_PHY1_PIPE: u32 = 8;
pub const P_USB3PHY_0_PIPE: u32 = 9;
pub const P_USB3PHY_1_PIPE: u32 = 10;
pub const P_UBI32_PLL: u32 = 11;
pub const P_NSS_CRYPTO_PLL: u32 = 12;
pub const P_BIAS_PLL: u32 = 13;
pub const P_BIAS_PLL_NSS_NOC: u32 = 14;
pub const P_UNIPHY0_RX: u32 = 15;
pub const P_UNIPHY0_TX: u32 = 16;
pub const P_UNIPHY1_RX: u32 = 17;
pub const P_UNIPHY1_TX: u32 = 18;
pub const P_UNIPHY2_RX: u32 = 19;
pub const P_UNIPHY2_TX: u32 = 20;

/// The complete source is retained for the file-local translation boundary;
/// included C symbols correspond one-for-one to the declarations above and to
/// kernel-provided clock-framework definitions.
pub const GCC_IPQ8074_SOURCE: &str = include_str!("gcc-ipq8074.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
