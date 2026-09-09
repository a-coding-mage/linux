/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright (c) 2016 Mellanox Technologies. All rights reserved.
 * Copyright (c) 2016 Jiri Pirko <jiri@mellanox.com>
 */

// Dependencies supplied by the surrounding kernel translation.

pub const DEVLINK_REGISTERED: u32 = 1; // XA_MARK_1
pub const DEVLINK_RELOAD_STATS_ARRAY_SIZE: usize =
    __DEVLINK_RELOAD_LIMIT_MAX as usize * __DEVLINK_RELOAD_ACTION_MAX as usize;

#[repr(C)]
pub struct devlink_dev_stats {
    pub reload_stats: [u32; DEVLINK_RELOAD_STATS_ARRAY_SIZE],
    pub remote_reload_stats: [u32; DEVLINK_RELOAD_STATS_ARRAY_SIZE],
}

#[repr(C)]
pub struct devlink {
    pub index: u32,
    pub ports: xarray,
    pub rate_list: list_head,
    pub sb_list: list_head,
    pub dpipe_table_list: list_head,
    pub resource_list: list_head,
    pub params: xarray,
    pub region_list: list_head,
    pub reporter_list: list_head,
    pub dpipe_headers: *mut devlink_dpipe_headers,
    pub trap_list: list_head,
    pub trap_group_list: list_head,
    pub trap_policer_list: list_head,
    pub linecard_list: list_head,
    pub ops: *const devlink_ops,
    pub snapshot_ids: xarray,
    pub stats: devlink_dev_stats,
    pub dev: *mut device,
    pub dev_name_index: *const core::ffi::c_char,
    pub dev_driver: *const device_driver,
    pub _net: possible_net_t,
    pub lock: mutex,
    pub lock_key: lock_class_key,
    pub reload_failed: u8,
    pub refcount: refcount_t,
    pub rwork: rcu_work,
    pub rel: *mut devlink_rel,
    pub nested_rels: xarray,
    pub priv_: [u8; 0], // flexible array, aligned to NETDEV_ALIGN
}

extern "C" {
    pub static mut devlinks: xarray;
    pub static mut devlink_nl_family: genl_family;

    pub fn __devlink_alloc(
        ops: *const devlink_ops, priv_size: usize, net: *mut net, dev: *mut device,
        dev_driver: *const device_driver,
    ) -> *mut devlink;

    pub fn devlinks_xa_find_get(net: *mut net, indexp: *mut c_ulong) -> *mut devlink;
    pub fn devlinks_xa_lookup_get(net: *mut net, index: c_ulong) -> *mut devlink;
    pub fn devlink_rel_nested_in_clear(rel_index: u32);
    pub fn devlink_rel_nested_in_add(
        rel_index: *mut u32, devlink_index: u32, obj_index: u32,
        notify_cb: devlink_rel_notify_cb_t, cleanup_cb: devlink_rel_cleanup_cb_t,
        devlink: *mut devlink,
    ) -> c_int;
    pub fn devlink_rel_nested_in_notify(devlink: *mut devlink);
    pub fn devlink_nested_in_get_lock(devlink: *mut devlink) -> *mut devlink;
    pub fn devlink_rel_devlink_handle_put(
        msg: *mut sk_buff, devlink: *mut devlink, rel_index: u32,
        attrtype: c_int, msg_updated: *mut bool,
    ) -> c_int;
}

#[repr(C)]
pub struct devlink_rel { _private: [u8; 0] }

pub type devlink_rel_notify_cb_t = unsafe extern "C" fn(*mut devlink, u32);
pub type devlink_rel_cleanup_cb_t = unsafe extern "C" fn(*mut devlink, u32, u32);

#[inline]
pub unsafe fn __devl_is_registered(devlink: *mut devlink) -> bool {
    xa_get_mark(&raw mut devlinks, (*devlink).index as c_ulong, DEVLINK_REGISTERED)
}

#[inline]
pub unsafe fn devl_is_registered(devlink: *mut devlink) -> bool {
    devl_assert_locked(devlink);
    __devl_is_registered(devlink)
}

#[inline]
pub unsafe fn devl_dev_lock(devlink: *mut devlink, dev_lock: bool) {
    if dev_lock && !(*devlink).dev.is_null() { device_lock((*devlink).dev); }
    devl_lock(devlink);
}

#[inline]
pub unsafe fn devl_dev_unlock(devlink: *mut devlink, dev_lock: bool) {
    devl_unlock(devlink);
    if dev_lock && !(*devlink).dev.is_null() { device_unlock((*devlink).dev); }
}

#[repr(C)]
pub struct devlink_nl_ctx {
    pub devlink: *mut devlink,
    pub devlink_port: *mut devlink_port,
    pub parent_devlink: *mut devlink,
}

#[inline]
pub unsafe fn devlink_nl_ctx(info: *mut genl_info) -> *mut devlink_nl_ctx {
    BUILD_BUG_ON(core::mem::size_of::<devlink_nl_ctx>() > core::mem::size_of::<genl_info_ctx>());
    (*info).ctx as *mut devlink_nl_ctx
}

#[repr(C)]
pub enum devlink_multicast_groups { DEVLINK_MCGRP_CONFIG }

#[repr(C)]
pub union devlink_nl_dump_state_data {
    pub start_offset: u64,
    pub dump_ts: u64,
    pub port_ctx: devlink_nl_dump_port_ctx,
}
#[repr(C)]
pub struct devlink_nl_dump_port_ctx { pub index: u32, pub index_valid: bool }
#[repr(C)]
pub struct devlink_nl_dump_state {
    pub instance: c_ulong,
    pub idx: c_int,
    pub data: devlink_nl_dump_state_data,
}

pub type devlink_nl_dump_one_func_t = unsafe extern "C" fn(*mut sk_buff, *mut devlink, *mut netlink_callback, c_int) -> c_int;

extern "C" {
    pub fn devlink_get_from_attrs_lock(net: *mut net, attrs: *mut *mut nlattr, dev_lock: bool) -> *mut devlink;
    pub fn devlink_get_parent_from_attrs_lock(net: *mut net, attrs: *mut *mut nlattr) -> *mut devlink;
    pub fn devlink_nl_dumpit(msg: *mut sk_buff, cb: *mut netlink_callback, dump_one: devlink_nl_dump_one_func_t) -> c_int;
    pub fn devlink_nl_put_nested_handle(msg: *mut sk_buff, net: *mut net, devlink: *mut devlink, attrtype: c_int) -> c_int;
    pub fn devlink_nl_msg_reply_and_new(msg: *mut *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn devlink_nl_notify_filter(dsk: *mut sock, skb: *mut sk_buff, data: *mut core::ffi::c_void) -> c_int;
}

#[repr(C)]
pub struct devlink_obj_desc {
    pub rcu: rcu_head,
    pub bus_name: *const core::ffi::c_char,
    pub dev_name: *const core::ffi::c_char,
    pub port_index: c_uint,
    pub port_index_valid: bool,
    pub devlink_index: c_uint,
    pub devlink_index_valid: bool,
    pub data: [c_long; 0],
}

#[inline]
pub unsafe fn devlink_nl_obj_desc_init(desc: *mut devlink_obj_desc, devlink: *mut devlink) {
    core::ptr::write_bytes(desc as *mut u8, 0, core::mem::size_of::<devlink_obj_desc>());
    (*desc).bus_name = devlink_bus_name(devlink);
    (*desc).dev_name = devlink_dev_name(devlink);
    (*desc).devlink_index = (*devlink).index;
    (*desc).devlink_index_valid = true;
}

#[inline]
pub unsafe fn devlink_nl_obj_desc_port_set(desc: *mut devlink_obj_desc, port: *mut devlink_port) {
    (*desc).port_index = (*port).index;
    (*desc).port_index_valid = true;
}

#[inline]
pub unsafe fn devlink_dump_state(cb: *mut netlink_callback) -> *mut devlink_nl_dump_state {
    NL_ASSERT_CTX_FITS::<devlink_nl_dump_state>();
    (*cb).ctx as *mut devlink_nl_dump_state
}

#[inline]
pub unsafe fn devlink_nl_put_handle(msg: *mut sk_buff, devlink: *mut devlink) -> c_int {
    if nla_put_string(msg, DEVLINK_ATTR_BUS_NAME, devlink_bus_name(devlink)) != 0 { return -EMSGSIZE; }
    if nla_put_string(msg, DEVLINK_ATTR_DEV_NAME, devlink_dev_name(devlink)) != 0 { return -EMSGSIZE; }
    if nla_put_uint(msg, DEVLINK_ATTR_INDEX, (*devlink).index) != 0 { return -EMSGSIZE; }
    0
}

#[inline]
pub unsafe fn devlink_nl_put_u64(msg: *mut sk_buff, attrtype: c_int, val: u64) -> c_int {
    nla_put_u64_64bit(msg, attrtype, val, DEVLINK_ATTR_PAD)
}

#[inline]
pub unsafe fn devlink_nl_notify_need(devlink: *mut devlink) -> bool {
    genl_has_listeners(&raw mut devlink_nl_family, devlink_net(devlink), DEVLINK_MCGRP_CONFIG)
}

#[inline]
pub unsafe fn devlink_nl_notify_send_desc(devlink: *mut devlink, msg: *mut sk_buff, desc: *mut devlink_obj_desc) {
    genlmsg_multicast_netns_filtered(&raw mut devlink_nl_family, devlink_net(devlink), msg, 0,
        DEVLINK_MCGRP_CONFIG, GFP_KERNEL, Some(devlink_nl_notify_filter), desc as *mut core::ffi::c_void);
}

// C preprocessor assertions and iteration macros are represented by their intended call sites:
// ASSERT_DEVLINK_REGISTERED, ASSERT_DEVLINK_NOT_REGISTERED, ASSERT_DEVLINK_PORT_INITIALIZED,
// and devlinks_xa_for_each_registered_get.

#[inline]
pub unsafe fn devlink_reload_supported(ops: *const devlink_ops) -> bool {
    !(*ops).reload_down.is_none() && !(*ops).reload_up.is_none()
}

#[inline]
pub unsafe fn devlink_nl_notify_send(devlink: *mut devlink, msg: *mut sk_buff) {
    let mut desc = core::mem::MaybeUninit::<devlink_obj_desc>::uninit();
    devlink_nl_obj_desc_init(desc.as_mut_ptr(), devlink);
    devlink_nl_notify_send_desc(devlink, msg, desc.as_mut_ptr());
}

extern "C" {
    pub fn devlink_notify_register(devlink: *mut devlink);
    pub fn devlink_notify_unregister(devlink: *mut devlink);
    pub fn devlink_ports_notify_register(devlink: *mut devlink);
    pub fn devlink_ports_notify_unregister(devlink: *mut devlink);
    pub fn devlink_params_notify_register(devlink: *mut devlink);
    pub fn devlink_params_notify_unregister(devlink: *mut devlink);
    pub fn devlink_regions_notify_register(devlink: *mut devlink);
    pub fn devlink_regions_notify_unregister(devlink: *mut devlink);
    pub fn devlink_trap_policers_notify_register(devlink: *mut devlink);
    pub fn devlink_trap_policers_notify_unregister(devlink: *mut devlink);
    pub fn devlink_trap_groups_notify_register(devlink: *mut devlink);
    pub fn devlink_trap_groups_notify_unregister(devlink: *mut devlink);
    pub fn devlink_traps_notify_register(devlink: *mut devlink);
    pub fn devlink_traps_notify_unregister(devlink: *mut devlink);
    pub fn devlink_rates_notify_register(devlink: *mut devlink);
    pub fn devlink_rates_notify_unregister(devlink: *mut devlink);
    pub fn devlink_linecards_notify_register(devlink: *mut devlink);
    pub fn devlink_linecards_notify_unregister(devlink: *mut devlink);
    pub fn devlink_port_get_by_index(devlink: *mut devlink, port_index: c_uint) -> *mut devlink_port;
    pub fn devlink_port_netdevice_event(nb: *mut notifier_block, event: c_ulong, ptr: *mut core::ffi::c_void) -> c_int;
    pub fn devlink_port_get_from_info(devlink: *mut devlink, info: *mut genl_info) -> *mut devlink_port;
    pub fn devlink_port_get_from_attrs(devlink: *mut devlink, attrs: *mut *mut nlattr) -> *mut devlink_port;
    pub fn devlink_reload_actions_valid(ops: *const devlink_ops) -> bool;
    pub fn devlink_reload(devlink: *mut devlink, dest_net: *mut net, action: devlink_reload_action, limit: devlink_reload_limit, actions_performed: *mut u32, extack: *mut netlink_ext_ack) -> c_int;
    pub fn devlink_params_driverinit_load_new(devlink: *mut devlink);
    pub fn devlink_resources_validate(devlink: *mut devlink, resource: *mut devlink_resource, info: *mut genl_info) -> c_int;
    pub fn devlink_rate_is_node(rate: *const devlink_rate) -> bool;
    pub fn devlink_rates_check(devlink: *mut devlink, filter: Option<unsafe extern "C" fn(*const devlink_rate) -> bool>, extack: *mut netlink_ext_ack) -> c_int;
    pub fn devlink_linecard_index(linecard: *mut devlink_linecard) -> c_uint;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
