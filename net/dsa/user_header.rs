/* SPDX-License-Identifier: GPL-2.0-or-later */

use core::ffi::c_void;

// Dependencies supplied by the corresponding kernel/networking headers.
#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct netlink_ext_ack {
    _private: [u8; 0],
}
#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dsa_switch {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dsa_port {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gro_cells {
    _private: [u8; 0],
}
#[repr(C)]
pub struct netpoll {
    _private: [u8; 0],
}
#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

pub type u8 = core::ffi::c_uchar;

extern "C" {
    pub static mut dsa_user_switchdev_notifier: notifier_block;
    pub static mut dsa_user_switchdev_blocking_notifier: notifier_block;

    pub fn netdev_priv(dev: *const net_device) -> *mut c_void;
    pub fn dsa_port_to_conduit(dp: *mut dsa_port) -> *mut net_device;
}

#[repr(C)]
pub struct dsa_user_priv {
    /* Copy of CPU port xmit for faster access in user transmit hot path */
    pub xmit: Option<unsafe extern "C" fn(
        skb: *mut sk_buff,
        dev: *mut net_device,
    ) -> *mut sk_buff>,

    pub gcells: gro_cells,

    /* DSA port data, such as switch, port index, etc. */
    pub dp: *mut dsa_port,

    /* CONFIG_NET_POLL_CONTROLLER */
    #[cfg(feature = "CONFIG_NET_POLL_CONTROLLER")]
    pub netpoll: *mut netpoll,

    /* TC context */
    pub mall_tc_list: list_head,
}

extern "C" {
    pub fn dsa_user_mii_bus_init(ds: *mut dsa_switch);
    pub fn dsa_user_create(dp: *mut dsa_port) -> i32;
    pub fn dsa_user_destroy(user_dev: *mut net_device);
    pub fn dsa_user_suspend(user_dev: *mut net_device) -> i32;
    pub fn dsa_user_resume(user_dev: *mut net_device) -> i32;
    pub fn dsa_user_register_notifier() -> i32;
    pub fn dsa_user_unregister_notifier();
    pub fn dsa_user_host_uc_install(dev: *mut net_device, addr: *const u8) -> i32;
    pub fn dsa_user_host_uc_uninstall(dev: *mut net_device);
    pub fn dsa_user_sync_ha(dev: *mut net_device);
    pub fn dsa_user_unsync_ha(dev: *mut net_device);
    pub fn dsa_user_setup_tagger(user: *mut net_device);
    pub fn dsa_user_change_mtu(dev: *mut net_device, new_mtu: i32) -> i32;
    pub fn dsa_user_change_conduit(
        dev: *mut net_device,
        conduit: *mut net_device,
        extack: *mut netlink_ext_ack,
    ) -> i32;
    pub fn dsa_user_manage_vlan_filtering(
        dev: *mut net_device,
        vlan_filtering: bool,
    ) -> i32;
}

#[inline]
pub unsafe fn dsa_user_to_port(dev: *const net_device) -> *mut dsa_port {
    let p = netdev_priv(dev) as *mut dsa_user_priv;
    (*p).dp
}

#[inline]
pub unsafe fn dsa_user_to_conduit(dev: *const net_device) -> *mut net_device {
    let dp = dsa_user_to_port(dev);
    dsa_port_to_conduit(dp)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
