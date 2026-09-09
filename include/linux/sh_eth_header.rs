/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies:
// #include <linux/phy.h>
// #include <linux/if_ether.h>

#[repr(C)]
pub struct sh_eth_plat_data {
    pub phy: ::core::ffi::c_int,
    pub phy_irq: ::core::ffi::c_int,
    pub phy_interface: phy_interface_t,
    pub set_mdio_gate: Option<unsafe extern "C" fn(addr: *mut ::core::ffi::c_void)>,

    pub mac_addr: [::core::ffi::c_uchar; ETH_ALEN],
    // C bit-fields share one unsigned storage unit.  The low two bits
    // correspond to no_ether_link:1 and ether_link_active_low:1.
    pub link_flags: ::core::ffi::c_uint,
}

pub const SH_ETH_PLAT_DATA_NO_ETHER_LINK: ::core::ffi::c_uint = 1 << 0;
pub const SH_ETH_PLAT_DATA_ETHER_LINK_ACTIVE_LOW: ::core::ffi::c_uint = 1 << 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
