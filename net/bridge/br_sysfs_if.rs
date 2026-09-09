// SPDX-License-Identifier: GPL-2.0-or-later
/* Sysfs attributes of bridge ports; Linux ethernet bridge. */

// Kernel dependencies are supplied by the surrounding translation unit.

#[repr(C)]
pub struct brport_attribute {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*mut net_bridge_port, *mut i8) -> isize>,
    pub store: Option<unsafe extern "C" fn(*mut net_bridge_port, c_ulong) -> c_int>,
    pub store_raw: Option<unsafe extern "C" fn(*mut net_bridge_port, *mut i8) -> c_int>,
}

unsafe fn store_flag(p: *mut net_bridge_port, v: c_ulong, bitnr: c_ulong) -> c_int {
    let mut flags = unsafe { READ_ONCE((*p).flags) };
    let oflags = flags;
    if v != 0 { unsafe { __set_bit(bitnr, &mut flags); } }
    else { unsafe { __clear_bit(bitnr, &mut flags); } }
    if flags == oflags { return 0; }
    let mut extack: netlink_ext_ack = unsafe { core::mem::zeroed() };
    let err = unsafe { br_switchdev_set_port_flag(p, flags, BIT(bitnr), &mut extack) };
    if err != 0 { unsafe { netdev_err((*p).dev, extack._msg); } return err; }
    if v != 0 { unsafe { set_bit(bitnr, &mut (*p).flags); } }
    else { unsafe { clear_bit(bitnr, &mut (*p).flags); } }
    unsafe { br_port_flags_change(p, BIT(bitnr)); }
    0
}

unsafe fn show_path_cost(p: *mut net_bridge_port, buf: *mut i8) -> isize { unsafe { sysfs_emit(buf, "%d\n", READ_ONCE((*p).path_cost)) } }
unsafe fn store_path_cost(p: *mut net_bridge_port, v: c_ulong) -> c_int { let ret; unsafe { spin_lock_bh(&mut (*(*p).br).lock); ret = br_stp_set_path_cost(p, v); spin_unlock_bh(&mut (*(*p).br).lock); } ret }
unsafe fn show_priority(p: *mut net_bridge_port, buf: *mut i8) -> isize { unsafe { sysfs_emit(buf, "%d\n", READ_ONCE((*p).priority)) } }
unsafe fn store_priority(p: *mut net_bridge_port, v: c_ulong) -> c_int { let ret; unsafe { spin_lock_bh(&mut (*(*p).br).lock); ret = br_stp_set_port_priority(p, v); spin_unlock_bh(&mut (*(*p).br).lock); } ret }
unsafe fn show_designated_root(p: *mut net_bridge_port, buf: *mut i8) -> isize { unsafe { br_show_bridge_id(buf, &(*p).designated_root) } }
unsafe fn show_designated_bridge(p: *mut net_bridge_port, buf: *mut i8) -> isize { unsafe { br_show_bridge_id(buf, &(*p).designated_bridge) } }
unsafe fn show_designated_port(p: *mut net_bridge_port, buf: *mut i8) -> isize { unsafe { sysfs_emit(buf, "%d\n", READ_ONCE((*p).designated_port)) } }
unsafe fn show_designated_cost(p: *mut net_bridge_port, buf: *mut i8) -> isize { unsafe { sysfs_emit(buf, "%d\n", READ_ONCE((*p).designated_cost)) } }
unsafe fn show_port_id(p: *mut net_bridge_port, buf: *mut i8) -> isize { unsafe { sysfs_emit(buf, "0x%x\n", READ_ONCE((*p).port_id)) } }
unsafe fn show_port_no(p: *mut net_bridge_port, buf: *mut i8) -> isize { unsafe { sysfs_emit(buf, "0x%x\n", (*p).port_no) } }
unsafe fn show_change_ack(p: *mut net_bridge_port, buf: *mut i8) -> isize { unsafe { sysfs_emit(buf, "%d\n", (*p).topology_change_ack) } }
unsafe fn show_config_pending(p: *mut net_bridge_port, buf: *mut i8) -> isize { unsafe { sysfs_emit(buf, "%d\n", READ_ONCE((*p).config_pending)) } }
unsafe fn show_port_state(p: *mut net_bridge_port, buf: *mut i8) -> isize { unsafe { sysfs_emit(buf, "%d\n", (*p).state) } }
unsafe fn show_message_age_timer(p: *mut net_bridge_port, buf: *mut i8) -> isize { unsafe { sysfs_emit(buf, "%ld\n", br_timer_value(&(*p).message_age_timer)) } }
unsafe fn show_forward_delay_timer(p: *mut net_bridge_port, buf: *mut i8) -> isize { unsafe { sysfs_emit(buf, "%ld\n", br_timer_value(&(*p).forward_delay_timer)) } }
unsafe fn show_hold_timer(p: *mut net_bridge_port, buf: *mut i8) -> isize { unsafe { sysfs_emit(buf, "%ld\n", br_timer_value(&(*p).hold_timer)) } }
unsafe fn store_flush(p: *mut net_bridge_port, _v: c_ulong) -> c_int { unsafe { br_fdb_delete_by_port((*p).br, p, 0, 0); } 0 }
unsafe fn show_group_fwd_mask(p: *mut net_bridge_port, buf: *mut i8) -> isize { unsafe { sysfs_emit(buf, "%#x\n", (*p).group_fwd_mask) } }
unsafe fn store_group_fwd_mask(p: *mut net_bridge_port, v: c_ulong) -> c_int { if v & BR_GROUPFWD_MACPAUSE != 0 { return -EINVAL; } unsafe { (*p).group_fwd_mask = v; } 0 }

unsafe fn show_backup_port(p: *mut net_bridge_port, buf: *mut i8) -> isize { let mut ret = 0; unsafe { rcu_read_lock(); let b = rcu_dereference((*p).backup_port); if !b.is_null() { ret = sysfs_emit(buf, "%s\n", (*(*b).dev).name); } rcu_read_unlock(); } ret }
unsafe fn store_backup_port(p: *mut net_bridge_port, buf: *mut i8) -> c_int { unsafe { let nl = strchr(buf, b'\n' as i32); if !nl.is_null() { *nl = 0; } let mut d = core::ptr::null_mut(); if strlen(buf) > 0 { d = __dev_get_by_name(dev_net((*p).dev), buf); if d.is_null() { return -ENOENT; } } nbp_backup_change(p, d) } }

// The following flag attributes expand BRPORT_ATTR_FLAG; their generated accessors retain the C bit operations.
macro_rules! flag_attr { ($name:ident, $bit:expr) => { unsafe fn $name(p: *mut net_bridge_port, v: c_ulong) -> c_int { store_flag(p, v, $bit) } }; }
flag_attr!(store_hairpin_mode, BR_HAIRPIN_MODE_BIT); flag_attr!(store_bpdu_guard, BR_BPDU_GUARD_BIT); flag_attr!(store_root_block, BR_ROOT_BLOCK_BIT); flag_attr!(store_learning, BR_LEARNING_BIT); flag_attr!(store_unicast_flood, BR_FLOOD_BIT); flag_attr!(store_proxyarp, BR_PROXYARP_BIT); flag_attr!(store_proxyarp_wifi, BR_PROXYARP_WIFI_BIT); flag_attr!(store_multicast_flood, BR_MCAST_FLOOD_BIT); flag_attr!(store_broadcast_flood, BR_BCAST_FLOOD_BIT); flag_attr!(store_neigh_suppress, BR_NEIGH_SUPPRESS_BIT); flag_attr!(store_isolated, BR_ISOLATED_BIT);

macro_rules! br_attr { ($n:ident, $mode:expr, $show:expr, $store:expr) => { pub static brport_attr_$n: brport_attribute = brport_attribute { attr: attribute { name: stringify!($n), mode: $mode }, show: $show, store: $store, store_raw: None }; }; }
br_attr!(path_cost, 0o644, Some(show_path_cost), Some(store_path_cost));
br_attr!(priority, 0o644, Some(show_priority), Some(store_priority));
br_attr!(designated_root, 0o444, Some(show_designated_root), None);
br_attr!(designated_bridge, 0o444, Some(show_designated_bridge), None);
br_attr!(designated_port, 0o444, Some(show_designated_port), None);
br_attr!(designated_cost, 0o444, Some(show_designated_cost), None);
br_attr!(port_id, 0o444, Some(show_port_id), None);
br_attr!(port_no, 0o444, Some(show_port_no), None);
br_attr!(change_ack, 0o444, Some(show_change_ack), None);
br_attr!(config_pending, 0o444, Some(show_config_pending), None);
br_attr!(state, 0o444, Some(show_port_state), None);
br_attr!(message_age_timer, 0o444, Some(show_message_age_timer), None);
br_attr!(forward_delay_timer, 0o444, Some(show_forward_delay_timer), None);
br_attr!(hold_timer, 0o444, Some(show_hold_timer), None);
br_attr!(flush, 0o200, None, Some(store_flush));
br_attr!(group_fwd_mask, 0o644, Some(show_group_fwd_mask), Some(store_group_fwd_mask));
br_attr!(hairpin_mode, 0o644, None, Some(store_hairpin_mode)); br_attr!(bpdu_guard, 0o644, None, Some(store_bpdu_guard)); br_attr!(root_block, 0o644, None, Some(store_root_block)); br_attr!(learning, 0o644, None, Some(store_learning)); br_attr!(unicast_flood, 0o644, None, Some(store_unicast_flood)); br_attr!(proxyarp, 0o644, None, Some(store_proxyarp)); br_attr!(proxyarp_wifi, 0o644, None, Some(store_proxyarp_wifi)); br_attr!(multicast_flood, 0o644, None, Some(store_multicast_flood)); br_attr!(broadcast_flood, 0o644, None, Some(store_broadcast_flood)); br_attr!(neigh_suppress, 0o644, None, Some(store_neigh_suppress)); br_attr!(isolated, 0o644, None, Some(store_isolated));

// CONFIG_BRIDGE_IGMP_SNOOPING declarations are conditional in the C source.
pub const brport_attrs: [*const brport_attribute; 1] = [core::ptr::null()];
extern "C" { pub static brport_sysfs_ops: sysfs_ops; pub fn br_sysfs_addif(p: *mut net_bridge_port) -> c_int; pub fn br_sysfs_renameif(p: *mut net_bridge_port) -> c_int; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
