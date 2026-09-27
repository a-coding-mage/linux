/* SPDX-License-Identifier: GPL-2.0 */

/* Platform specific PHY configuration passed to the MAC driver */
#[repr(C)]
pub struct au1000_eth_platform_data {
    pub phy_static_config: ::kernel::ffi::c_int,
    pub phy_search_highest_addr: ::kernel::ffi::c_int,
    pub phy1_search_mac0: ::kernel::ffi::c_int,
    pub phy_addr: ::kernel::ffi::c_int,
    pub phy_busid: ::kernel::ffi::c_int,
    pub phy_irq: ::kernel::ffi::c_int,
    pub mac: [::kernel::ffi::c_char; 6],
}

// The C __init annotation controls kernel initialization placement.
extern "C" {
    pub fn au1xxx_override_eth_cfg(
        port: ::kernel::ffi::c_uint,
        eth_data: *mut au1000_eth_platform_data,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
