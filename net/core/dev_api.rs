// SPDX-License-Identifier: GPL-2.0-or-later

// C dependencies supplied by the surrounding kernel translation unit.

use core::ffi::c_char;

#[repr(C)]
pub struct net_device {
    pub netdev_ops: *const net_device_ops,
}
#[repr(C)]
pub struct net_device_ops {
    pub ndo_eth_ioctl: Option<unsafe extern "C" fn(*mut net_device, *mut ifreq, u32) -> i32>,
}
#[repr(C)] pub struct netlink_ext_ack { _private: [u8; 0] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct sockaddr_storage { _private: [u8; 0] }
#[repr(C)] pub struct ifreq { _private: [u8; 0] }
#[repr(C)] pub struct netdev_bpf { _private: [u8; 0] }
#[repr(C)] pub enum netdev_napi_threaded {}

extern "C" {
    fn netdev_lock_ops(dev: *mut net_device);
    fn netdev_unlock_ops(dev: *mut net_device);
    fn netif_change_name(dev: *mut net_device, newname: *const c_char) -> i32;
    fn netif_set_alias(dev: *mut net_device, alias: *const c_char, len: usize) -> i32;
    fn netif_change_flags(dev: *mut net_device, flags: u32, extack: *mut netlink_ext_ack) -> i32;
    fn netif_rx_mode_sync(dev: *mut net_device);
    fn netif_set_group(dev: *mut net_device, new_group: i32);
    fn down_write(sem: *mut core::ffi::c_void);
    fn up_write(sem: *mut core::ffi::c_void);
    static mut dev_addr_sem: core::ffi::c_void;
    fn netif_set_mac_address(dev: *mut net_device, ss: *mut sockaddr_storage,
                             extack: *mut netlink_ext_ack) -> i32;
    fn __dev_change_net_namespace(dev: *mut net_device, net: *mut net,
                                  pat: *const c_char, flags: i32,
                                  extack: *mut netlink_ext_ack) -> i32;
    fn netif_change_carrier(dev: *mut net_device, new_carrier: bool) -> i32;
    fn netif_change_tx_queue_len(dev: *mut net_device, new_len: usize) -> i32;
    fn netif_change_proto_down(dev: *mut net_device, proto_down: bool) -> i32;
    fn netif_open(dev: *mut net_device, extack: *mut netlink_ext_ack) -> i32;
    fn netif_close(dev: *mut net_device);
    fn netif_device_present(dev: *mut net_device) -> bool;
    fn netif_set_mtu(dev: *mut net_device, new_mtu: i32) -> i32;
    fn netif_disable_lro(dev: *mut net_device);
    fn netif_set_promiscuity(dev: *mut net_device, inc: i32) -> i32;
    fn netif_set_allmulti(dev: *mut net_device, inc: i32, notify: bool) -> i32;
    fn netif_xdp_propagate(dev: *mut net_device, bpf: *mut netdev_bpf) -> i32;
    fn netif_state_change(dev: *mut net_device);
    fn netdev_lock(dev: *mut net_device);
    fn netif_set_threaded(dev: *mut net_device, threaded: netdev_napi_threaded) -> i32;
    fn netdev_unlock(dev: *mut net_device);
}

pub unsafe fn dev_change_name(dev: *mut net_device, newname: *const c_char) -> i32 {
    netdev_lock_ops(dev);
    let ret = netif_change_name(dev, newname);
    netdev_unlock_ops(dev);
    ret
}

pub unsafe fn dev_set_alias(dev: *mut net_device, alias: *const c_char, len: usize) -> i32 {
    netdev_lock_ops(dev);
    let ret = netif_set_alias(dev, alias, len);
    netdev_unlock_ops(dev);
    ret
}

pub unsafe fn dev_change_flags(dev: *mut net_device, flags: u32,
                               extack: *mut netlink_ext_ack) -> i32 {
    netdev_lock_ops(dev);
    let ret = netif_change_flags(dev, flags, extack);
    netif_rx_mode_sync(dev);
    netdev_unlock_ops(dev);
    ret
}

pub unsafe fn dev_set_group(dev: *mut net_device, new_group: i32) {
    netdev_lock_ops(dev); netif_set_group(dev, new_group); netdev_unlock_ops(dev);
}

pub unsafe fn dev_set_mac_address_user(dev: *mut net_device, ss: *mut sockaddr_storage,
                                       extack: *mut netlink_ext_ack) -> i32 {
    down_write(&raw mut dev_addr_sem as *mut _);
    netdev_lock_ops(dev);
    let ret = netif_set_mac_address(dev, ss, extack);
    netdev_unlock_ops(dev);
    up_write(&raw mut dev_addr_sem as *mut _);
    ret
}

pub unsafe fn dev_change_net_namespace(dev: *mut net_device, net: *mut net,
                                       pat: *const c_char) -> i32 {
    __dev_change_net_namespace(dev, net, pat, 0, core::ptr::null_mut())
}

pub unsafe fn dev_change_carrier(dev: *mut net_device, new_carrier: bool) -> i32 {
    netdev_lock_ops(dev); let ret = netif_change_carrier(dev, new_carrier); netdev_unlock_ops(dev); ret
}
pub unsafe fn dev_change_tx_queue_len(dev: *mut net_device, new_len: usize) -> i32 {
    netdev_lock_ops(dev); let ret = netif_change_tx_queue_len(dev, new_len); netdev_unlock_ops(dev); ret
}
pub unsafe fn dev_change_proto_down(dev: *mut net_device, proto_down: bool) -> i32 {
    netdev_lock_ops(dev); let ret = netif_change_proto_down(dev, proto_down); netdev_unlock_ops(dev); ret
}
pub unsafe fn dev_open(dev: *mut net_device, extack: *mut netlink_ext_ack) -> i32 {
    netdev_lock_ops(dev); let ret = netif_open(dev, extack); netdev_unlock_ops(dev); ret
}
pub unsafe fn dev_close(dev: *mut net_device) {
    netdev_lock_ops(dev); netif_close(dev); netdev_unlock_ops(dev);
}

pub unsafe fn dev_eth_ioctl(dev: *mut net_device, ifr: *mut ifreq, cmd: u32) -> i32 {
    let ops = (*dev).netdev_ops;
    let mut ret = -19i32;
    let ioctl = (*ops).ndo_eth_ioctl;
    if ioctl.is_none() { return -95i32; }
    netdev_lock_ops(dev);
    if netif_device_present(dev) { ret = ioctl.unwrap()(dev, ifr, cmd); }
    netdev_unlock_ops(dev);
    ret
}

pub unsafe fn dev_set_mtu(dev: *mut net_device, new_mtu: i32) -> i32 {
    netdev_lock_ops(dev); let ret = netif_set_mtu(dev, new_mtu); netdev_unlock_ops(dev); ret
}
pub unsafe fn dev_disable_lro(dev: *mut net_device) {
    netdev_lock_ops(dev); netif_disable_lro(dev); netdev_unlock_ops(dev);
}
pub unsafe fn dev_set_promiscuity(dev: *mut net_device, inc: i32) -> i32 {
    netdev_lock_ops(dev); let ret = netif_set_promiscuity(dev, inc); netif_rx_mode_sync(dev); netdev_unlock_ops(dev); ret
}
pub unsafe fn dev_set_allmulti(dev: *mut net_device, inc: i32) -> i32 {
    netdev_lock_ops(dev); let ret = netif_set_allmulti(dev, inc, true); netif_rx_mode_sync(dev); netdev_unlock_ops(dev); ret
}
pub unsafe fn dev_set_mac_address(dev: *mut net_device, ss: *mut sockaddr_storage,
                                  extack: *mut netlink_ext_ack) -> i32 {
    netdev_lock_ops(dev); let ret = netif_set_mac_address(dev, ss, extack); netdev_unlock_ops(dev); ret
}
pub unsafe fn dev_xdp_propagate(dev: *mut net_device, bpf: *mut netdev_bpf) -> i32 {
    netdev_lock_ops(dev); let ret = netif_xdp_propagate(dev, bpf); netdev_unlock_ops(dev); ret
}
pub unsafe fn netdev_state_change(dev: *mut net_device) {
    netdev_lock_ops(dev); netif_state_change(dev); netdev_unlock_ops(dev);
}
pub unsafe fn dev_set_threaded(dev: *mut net_device, threaded: netdev_napi_threaded) -> i32 {
    netdev_lock(dev); let ret = netif_set_threaded(dev, threaded); netdev_unlock(dev); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
