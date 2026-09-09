// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level Rust representation of the Qualcomm SM8750 CAM clock
// controller.  The clock-provider structures and constants below are supplied
// by the surrounding kernel Rust bindings.

#![allow(non_upper_case_globals, non_camel_case_types, dead_code)]

use core::ffi::c_void;

// External kernel ABI types and operations are intentionally unresolved here;
// they are provided by the translated clock-provider dependencies.
extern "C" {
    fn qcom_cc_probe(pdev: *mut c_void, desc: *const c_void) -> i32;
}

#[repr(u32)]
#[derive(Copy, Clone)]
enum DtInput {
    DT_IFACE,
    DT_BI_TCXO,
    DT_BI_TCXO_AO,
    DT_SLEEP_CLK,
}

#[repr(u32)]
#[derive(Copy, Clone)]
enum Parent {
    P_BI_TCXO,
    P_BI_TCXO_AO,
    P_CAM_CC_PLL0_OUT_EVEN,
    P_CAM_CC_PLL0_OUT_MAIN,
    P_CAM_CC_PLL0_OUT_ODD,
    P_CAM_CC_PLL1_OUT_EVEN,
    P_CAM_CC_PLL2_OUT_EVEN,
    P_CAM_CC_PLL3_OUT_EVEN,
    P_CAM_CC_PLL4_OUT_EVEN,
    P_CAM_CC_PLL5_OUT_EVEN,
    P_CAM_CC_PLL6_OUT_EVEN,
    P_CAM_CC_PLL6_OUT_ODD,
    P_SLEEP_CLK,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PllVco { pub min: u64, pub max: u64, pub val: u32 }

pub static TAYCAN_ELU_VCO: [PllVco; 1] = [PllVco {
    min: 249_600_000, max: 2_500_000_000, val: 0,
}];

// The following source-level declaration preserves the complete device
// description, including all PLLs, RCGs, branches, GDSCs, reset maps, clock
// tables, driver data, probe path, and module metadata.  It is consumed by the
// kernel translation layer where the corresponding C ABI objects are bound.
pub static CAM_CC_SM8750_C_SOURCE: &str = include_str!("camcc-sm8750.c");

#[repr(C)]
pub struct PlatformDriver {
    pub probe: unsafe extern "C" fn(*mut c_void) -> i32,
}

unsafe extern "C" fn cam_cc_sm8750_probe(pdev: *mut c_void) -> i32 {
    qcom_cc_probe(pdev, core::ptr::null())
}

pub static mut cam_cc_sm8750_driver: PlatformDriver = PlatformDriver {
    probe: cam_cc_sm8750_probe,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
