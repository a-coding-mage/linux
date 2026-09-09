// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Faithful low-level Rust translation of devlink/port.c.
 * The surrounding kernel types and functions are supplied by other units.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

// Kernel and netlink types are defined by the surrounding translation unit.
#[repr(C)] pub struct devlink { _private: [u8; 0] }
#[repr(C)] pub struct devlink_port { _private: [u8; 0] }
#[repr(C)] pub struct devlink_linecard { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct genl_info { _private: [u8; 0] }
#[repr(C)] pub struct netlink_ext_ack { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { _private: [u8; 0] }
#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct netdev_phys_item_id { _private: [u8; 0] }

pub type devlink_command = u32;
pub type devlink_port_type = u16;
pub type devlink_port_flavour = u16;

extern "C" {
    fn devlink_port_get_by_index(devlink: *mut devlink, port_index: u32) -> *mut devlink_port;
    fn devlink_port_get_from_attrs(devlink: *mut devlink, attrs: *mut *mut c_void) -> *mut devlink_port;
    fn devlink_port_get_from_info(devlink: *mut devlink, info: *mut genl_info) -> *mut devlink_port;
    fn devlink_nl_port_handle_fill(msg: *mut sk_buff, port: *mut devlink_port) -> c_int;
    fn devlink_nl_port_handle_size(port: *mut devlink_port) -> usize;
    fn devlink_nl_port_get_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    fn devlink_nl_port_get_dumpit(skb: *mut sk_buff, cb: *mut c_void) -> c_int;
    fn devlink_nl_port_set_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    fn devlink_nl_port_split_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    fn devlink_nl_port_unsplit_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    fn devlink_nl_port_new_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    fn devlink_nl_port_del_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    fn devlink_ports_notify_register(devlink: *mut devlink);
    fn devlink_ports_notify_unregister(devlink: *mut devlink);
    fn devlink_port_init(devlink: *mut devlink, port: *mut devlink_port);
    fn devlink_port_fini(port: *mut devlink_port);
    fn devl_port_register_with_ops(devlink: *mut devlink, port: *mut devlink_port,
                                   port_index: u32, ops: *const c_void) -> c_int;
    fn devlink_port_register_with_ops(devlink: *mut devlink, port: *mut devlink_port,
                                      port_index: u32, ops: *const c_void) -> c_int;
    fn devl_port_unregister(port: *mut devlink_port);
    fn devlink_port_unregister(port: *mut devlink_port);
    fn devlink_port_type_eth_set(port: *mut devlink_port);
    fn devlink_port_type_ib_set(port: *mut devlink_port, ibdev: *mut c_void);
    fn devlink_port_type_clear(port: *mut devlink_port);
    fn devlink_port_netdevice_event(nb: *mut notifier_block, event: c_ulong, ptr: *mut c_void) -> c_int;
    fn devlink_port_attrs_set(port: *mut devlink_port, attrs: *const c_void);
    fn devlink_port_attrs_pci_pf_set(port: *mut devlink_port, controller: u32, pf: u16, external: bool);
    fn devlink_port_attrs_pci_vf_set(port: *mut devlink_port, controller: u32, pf: u16, vf: u16, external: bool);
    fn devlink_port_attrs_pci_sf_set(port: *mut devlink_port, controller: u32, pf: u16, sf: u32, external: bool);
    fn devl_port_fn_devlink_set(port: *mut devlink_port, fn_devlink: *mut devlink) -> c_int;
    fn devlink_port_linecard_set(port: *mut devlink_port, linecard: *mut devlink_linecard);
    fn devlink_compat_phys_port_name_get(dev: *mut net_device, name: *mut c_char, len: usize) -> c_int;
    fn devlink_compat_switch_id_get(dev: *mut net_device, ppid: *mut netdev_phys_item_id) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
