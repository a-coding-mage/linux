/* SPDX-License-Identifier: GPL-2.0 */
/*
 * PHY device list allow maintaining a list of PHY devices that are
 * part of a netdevice's link topology. PHYs can for example be chained,
 * as is the case when using a PHY that exposes an SFP module, on which an
 * SFP transceiver that embeds a PHY is connected.
 *
 * This list can then be used by userspace to leverage individual PHY
 * capabilities.
 */

/* External declarations supplied by linux/ethtool.h and linux/netdevice.h. */
#[repr(C)]
pub struct xarray {
    _private: [u8; 0],
}

#[repr(C)]
pub struct phy_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sfp_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    pub link_topo: *mut phy_link_topology,
}

/* The definition and discriminants of this external C enum are dependency-provided. */
pub type phy_upstream = i32;

#[repr(C)]
pub struct phy_link_topology {
    pub phys: xarray,
    pub next_phy_index: u32,
}

#[repr(C)]
pub union phy_device_node_upstream {
    pub netdev: *mut net_device,
    pub phydev: *mut phy_device,
}

#[repr(C)]
pub struct phy_device_node {
    pub upstream_type: phy_upstream,
    pub upstream: phy_device_node_upstream,
    pub parent_sfp_bus: *mut sfp_bus,
    pub phy: *mut phy_device,
}

#[inline]
pub unsafe fn phy_link_topo_empty(dev: *mut net_device) -> bool {
    (*dev).link_topo.is_null()
}

#[cfg(feature = "CONFIG_PHYLIB")]
extern "C" {
    pub fn phy_link_topo_add_phy(
        dev: *mut net_device,
        phy: *mut phy_device,
        upt: phy_upstream,
        upstream: *mut core::ffi::c_void,
    ) -> i32;

    pub fn phy_link_topo_del_phy(dev: *mut net_device, phy: *mut phy_device);
}

extern "C" {
    fn xa_load(array: *const xarray, index: u32) -> *mut core::ffi::c_void;
}

#[cfg(feature = "CONFIG_PHYLIB")]
#[inline]
pub unsafe fn phy_link_topo_get_phy(
    dev: *mut net_device,
    phyindex: u32,
) -> *mut phy_device {
    let topo = (*dev).link_topo;
    let pdn: *mut phy_device_node;

    if topo.is_null() {
        return core::ptr::null_mut();
    }

    pdn = xa_load(&(*topo).phys, phyindex) as *mut phy_device_node;
    if !pdn.is_null() {
        return (*pdn).phy;
    }

    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_PHYLIB"))]
#[inline]
pub unsafe fn phy_link_topo_add_phy(
    _dev: *mut net_device,
    _phy: *mut phy_device,
    _upt: phy_upstream,
    _upstream: *mut core::ffi::c_void,
) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_PHYLIB"))]
#[inline]
pub unsafe fn phy_link_topo_del_phy(_dev: *mut net_device, _phy: *mut phy_device) {}

#[cfg(not(feature = "CONFIG_PHYLIB"))]
#[inline]
pub unsafe fn phy_link_topo_get_phy(
    _dev: *mut net_device,
    _phyindex: u32,
) -> *mut phy_device {
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
