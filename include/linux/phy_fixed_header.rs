/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency supplied by the surrounding kernel translation. */

#[repr(C)]
pub struct fixed_phy_status {
    pub speed: ::core::ffi::c_int,
    pub duplex: ::core::ffi::c_int,
    /* C bit-fields; represented as bool fields because Rust has no native bit-fields. */
    pub link: bool,
    pub pause: bool,
    pub asym_pause: bool,
}

pub struct device_node;
pub struct net_device;
pub struct phy_device;

/* These declarations and definitions are selected by CONFIG_FIXED_PHY. */
#[cfg(feature = "CONFIG_FIXED_PHY")]
extern "C" {
    pub fn fixed_phy_change_carrier(
        dev: *mut net_device,
        new_carrier: bool,
    ) -> ::core::ffi::c_int;

    pub fn fixed_phy_register(
        status: *const fixed_phy_status,
        np: *mut device_node,
    ) -> *mut phy_device;

    pub fn fixed_phy_register_100fd() -> *mut phy_device;

    pub fn fixed_phy_unregister(phydev: *mut phy_device);

    pub fn fixed_phy_set_link_update(
        phydev: *mut phy_device,
        link_update: Option<
            unsafe extern "C" fn(
                *mut net_device,
                *mut fixed_phy_status,
            ) -> ::core::ffi::c_int,
        >,
    ) -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_FIXED_PHY"))]
#[inline]
pub unsafe fn fixed_phy_register(
    _status: *const fixed_phy_status,
    _np: *mut device_node,
) -> *mut phy_device {
    /* Equivalent to ERR_PTR(-ENODEV); ERR_PTR is supplied by the surrounding translation. */
    ERR_PTR(-ENODEV)
}

#[cfg(not(feature = "CONFIG_FIXED_PHY"))]
#[inline]
pub unsafe fn fixed_phy_register_100fd() -> *mut phy_device {
    /* Equivalent to ERR_PTR(-ENODEV); ERR_PTR is supplied by the surrounding translation. */
    ERR_PTR(-ENODEV)
}

#[cfg(not(feature = "CONFIG_FIXED_PHY"))]
#[inline]
pub unsafe fn fixed_phy_unregister(_phydev: *mut phy_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
