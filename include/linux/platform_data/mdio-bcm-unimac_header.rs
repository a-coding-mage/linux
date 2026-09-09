// Translated from mdio-bcm-unimac.h.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct unimac_mdio_pdata {
    pub phy_mask: u32,
    pub wait_func: Option<unsafe extern "C" fn(data: *mut c_void) -> c_int>,
    pub wait_func_data: *mut c_void,
    pub bus_name: *const c_char,
    pub clk: *mut clk,
}

pub const UNIMAC_MDIO_DRV_NAME: &str = "unimac-mdio";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
