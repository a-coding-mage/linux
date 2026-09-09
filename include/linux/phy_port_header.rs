/* SPDX-License-Identifier: GPL-2.0-or-later */

/* Translated from phy_port.h. Linux type and constant dependencies are
 * supplied by the surrounding translation unit. */

use core::ffi::c_int;

#[repr(C)]
pub struct phy_port;

/**
 * enum phy_port_parent - The device this port is attached to
 *
 * @PHY_PORT_PHY: Indicates that the port is driven by a PHY device
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum phy_port_parent {
    PHY_PORT_PHY,
}

#[repr(C)]
pub struct phy_port_ops {
    /* Sometimes, the link state can be retrieved from physical,
     * out-of-band channels such as the LOS signal on SFP. These
     * callbacks allows notifying the port about state changes
     */
    pub link_up: Option<unsafe extern "C" fn(port: *mut phy_port)>,
    pub link_down: Option<unsafe extern "C" fn(port: *mut phy_port)>,

    /* If the port acts as a Media Independent Interface (Serdes port),
     * configures the port with the relevant state and mode. When enable is
     * not set, interface should be ignored
     */
    pub configure_mii: Option<
        unsafe extern "C" fn(
            port: *mut phy_port,
            enable: bool,
            interface: phy_interface_t,
        ) -> c_int,
    >,
}

/* The following dependent Linux declarations are intentionally referenced
 * by name; their definitions are supplied by the surrounding translation. */
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct phy_device;

#[repr(C)]
pub struct device_node;

pub type phy_interface_t = u32;

/* __ETHTOOL_DECLARE_LINK_MODE_MASK and DECLARE_PHY_INTERFACE_MASK are Linux
 * macro declarations. Their storage is represented as fixed bit arrays. */
pub const PHY_PORT_LINK_MODE_WORDS: usize = 1;
pub const PHY_PORT_INTERFACE_WORDS: usize = 1;

#[repr(C)]
pub union phy_port_phy_union {
    pub phy: *mut phy_device,
}

/**
 * struct phy_port - A representation of a network device physical interface
 */
#[repr(C)]
pub struct phy_port {
    pub head: list_head,
    pub parent_type: phy_port_parent,
    pub phy: phy_port_phy_union,

    pub ops: *const phy_port_ops,

    pub pairs: c_int,
    pub mediums: usize,
    pub supported: [usize; PHY_PORT_LINK_MODE_WORDS],
    pub interfaces: [usize; PHY_PORT_INTERFACE_WORDS],

    pub not_described: u32,
    pub active: u32,
    pub is_mii: u32,
    pub is_sfp: u32,
}

extern "C" {
    pub fn phy_port_alloc() -> *mut phy_port;
    pub fn phy_port_destroy(port: *mut phy_port);

    pub fn phy_of_parse_port(dn: *mut device_node) -> *mut phy_port;

    pub fn phy_port_update_supported(port: *mut phy_port);
    pub fn phy_port_restrict_mediums(port: *mut phy_port, mediums: usize) -> c_int;

    pub fn phy_port_get_type(port: *mut phy_port) -> c_int;
}

#[inline]
pub unsafe fn port_phydev(port: *mut phy_port) -> *mut phy_device {
    (*port).phy.phy
}

#[inline]
pub unsafe fn phy_port_is_copper(port: *mut phy_port) -> bool {
    (*port).mediums == (1usize << ETHTOOL_LINK_MEDIUM_BASET)
}

#[inline]
pub unsafe fn phy_port_is_fiber(port: *mut phy_port) -> bool {
    ((*port).mediums & ETHTOOL_MEDIUM_FIBER_BITS) != 0
}

/* Supplied by the Linux ethtool dependency. */
extern "C" {
    static ETHTOOL_LINK_MEDIUM_BASET: u32;
    static ETHTOOL_MEDIUM_FIBER_BITS: usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
