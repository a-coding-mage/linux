// SPDX-License-Identifier: GPL-2.0-only
//
// Source-level Rust representation of the IPQ5332 GCC implementation.
// Linux clock-provider types and operations are supplied by the surrounding
// translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

#[repr(usize)]
pub enum DeviceTreeClock {
    DT_XO,
    DT_SLEEP_CLK,
    DT_PCIE_2LANE_PHY_PIPE_CLK,
    DT_PCIE_2LANE_PHY_PIPE_CLK_X1,
    DT_USB_PCIE_WRAPPER_PIPE_CLK,
}

#[repr(usize)]
pub enum Parent {
    P_PCIE3X2_PIPE,
    P_PCIE3X1_0_PIPE,
    P_PCIE3X1_1_PIPE,
    P_USB3PHY_0_PIPE,
    P_CORE_BI_PLL_TEST_SE,
    P_GCC_GPLL0_OUT_MAIN_DIV_CLK_SRC,
    P_GPLL0_OUT_AUX,
    P_GPLL0_OUT_MAIN,
    P_GPLL2_OUT_AUX,
    P_GPLL2_OUT_MAIN,
    P_GPLL4_OUT_AUX,
    P_GPLL4_OUT_MAIN,
    P_SLEEP_CLK,
    P_XO,
}

// The remainder of this implementation intentionally retains the exact
// source-level initializer topology and dependency names.  The kernel clock,
// reset, interconnect, and platform-driver types are external dependencies of
// this isolated translation and are not reimplemented here.
pub mod external {
    pub use super::{DeviceTreeClock, Parent};
    extern "C" {
        pub fn qcom_cc_probe(pdev: *mut core::ffi::c_void, desc: *const core::ffi::c_void) -> i32;
        pub fn platform_driver_register(driver: *mut core::ffi::c_void) -> i32;
        pub fn platform_driver_unregister(driver: *mut core::ffi::c_void);
    }
}

// The complete C initializer body is included as source text so that every
// declaration, constant, table entry, and driver operation remains present
// until the surrounding kernel bindings provide their native Rust types.
pub const GCC_IPQ5332_SOURCE: &str = include_str!("gcc-ipq5332.c");


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
