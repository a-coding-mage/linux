/* SPDX-License-Identifier: GPL-1.0+ */
/* Rust translation of bonding.h. Included C headers and external symbols are
 * intentionally left as dependencies of the surrounding kernel translation. */

pub const BOND_MAX_ARP_TARGETS: usize = 16;
pub const BOND_MAX_NS_TARGETS: usize = BOND_MAX_ARP_TARGETS;
pub const BOND_DEFAULT_MIIMON: i32 = 100;

pub const BOND_LINK_NOCHANGE: i32 = -1;
pub const BOND_PRI_RESELECT_ALWAYS: i32 = 0;
pub const BOND_PRI_RESELECT_BETTER: i32 = 1;
pub const BOND_PRI_RESELECT_FAILURE: i32 = 2;
pub const BOND_FOM_NONE: i32 = 0;
pub const BOND_FOM_ACTIVE: i32 = 1;
pub const BOND_FOM_FOLLOW: i32 = 2;
pub const BOND_ARP_TARGETS_ANY: i32 = 0;
pub const BOND_ARP_TARGETS_ALL: i32 = 1;
pub const BOND_ARP_VALIDATE_NONE: i32 = 0;
pub const BOND_ARP_VALIDATE_ACTIVE: i32 = 1 << BOND_STATE_ACTIVE;
pub const BOND_ARP_VALIDATE_BACKUP: i32 = 1 << BOND_STATE_BACKUP;
pub const BOND_ARP_VALIDATE_ALL: i32 = BOND_ARP_VALIDATE_ACTIVE | BOND_ARP_VALIDATE_BACKUP;
pub const BOND_ARP_FILTER: i32 = BOND_ARP_VALIDATE_ALL + 1;
pub const BOND_ARP_FILTER_ACTIVE: i32 = BOND_ARP_VALIDATE_ACTIVE | BOND_ARP_FILTER;
pub const BOND_ARP_FILTER_BACKUP: i32 = BOND_ARP_VALIDATE_BACKUP | BOND_ARP_FILTER;
pub const BOND_SLAVE_NOTIFY_NOW: bool = true;
pub const BOND_SLAVE_NOTIFY_LATER: bool = false;

#[repr(C)]
pub struct bond_params {
    pub mode: i32, pub xmit_policy: i32, pub miimon: i32,
    pub num_peer_notif: u8, pub missed_max: u8, pub arp_interval: i32,
    pub arp_validate: i32, pub arp_all_targets: i32, pub fail_over_mac: i32,
    pub updelay: i32, pub downdelay: i32, pub peer_notif_delay: i32,
    pub lacp_active: i32, pub lacp_fast: i32, pub lacp_strict: i32,
    pub min_links: u32, pub ad_select: i32, pub primary: [u8; IFNAMSIZ],
    pub primary_reselect: i32, pub arp_targets: [__be32; BOND_MAX_ARP_TARGETS],
    pub tx_queues: i32, pub all_slaves_active: i32, pub resend_igmp: i32,
    pub lp_interval: i32, pub packets_per_slave: i32, pub tlb_dynamic_lb: i32,
    pub reciprocal_packets_per_slave: reciprocal_value, pub ad_actor_sys_prio: u16,
    pub ad_user_port_key: u16,
    #[cfg(CONFIG_IPV6)] pub ns_targets: [in6_addr; BOND_MAX_NS_TARGETS],
    pub coupled_control: i32, pub broadcast_neighbor: i32,
    pub ad_actor_system: [u8; ETH_ALEN + 2],
}

#[repr(C)]
pub struct slave {
    pub dev: *mut net_device, pub bond: *mut bonding, pub delay: i32,
    pub last_link_up: ::core::ffi::c_ulong, pub last_tx: ::core::ffi::c_ulong,
    pub last_rx: ::core::ffi::c_ulong,
    pub target_last_arp_rx: [::core::ffi::c_ulong; BOND_MAX_ARP_TARGETS],
    pub link: i8, pub link_new_state: i8,
    pub backup: u8, pub inactive: u8, pub rx_disabled: u8,
    pub should_notify: u8, pub should_notify_link: u8, pub duplex: u8,
    pub original_mtu: u32, pub link_failure_count: u32, pub speed: u32,
    pub queue_id: u16, pub perm_hwaddr: [u8; MAX_ADDR_LEN], pub prio: i32,
    pub ad_info: *mut ad_slave_info, pub tlb_info: tlb_slave_info,
    #[cfg(CONFIG_NET_POLL_CONTROLLER)] pub np: *mut netpoll,
    pub notify_work: delayed_work, pub kobj: kobject, pub slave_stats: rtnl_link_stats64,
}

#[repr(C)] pub struct bond_up_slave { pub count: u32, pub rcu: rcu_head, pub arr: [*mut slave; 0] }
#[repr(C)] pub struct bond_ipsec { pub list: list_head, pub xs: *mut xfrm_state }
#[repr(C)] pub struct bond_vlan_tag { pub vlan_proto: __be16, pub vlan_id: u16 }
#[repr(C)] pub struct bond_net {
    pub net: *mut net, pub dev_list: list_head,
    #[cfg(CONFIG_PROC_FS)] pub proc_dir: *mut proc_dir_entry,
    pub class_attr_bonding_masters: class_attribute,
}

#[repr(C)]
pub struct bonding {
    pub dev: *mut net_device, pub curr_active_slave: *mut slave,
    pub current_arp_slave: *mut slave, pub primary_slave: *mut slave,
    pub usable_slaves: *mut bond_up_slave, pub all_slaves: *mut bond_up_slave,
    pub force_primary: bool, pub notifier_ctx: bool, pub slave_cnt: i32,
    pub recv_probe: Option<unsafe extern "C" fn(*const sk_buff, *mut bonding, *mut slave) -> i32>,
    pub mode_lock: spinlock_t, pub stats_lock: spinlock_t, pub send_peer_notif: u32,
    pub igmp_retrans: u8,
    #[cfg(CONFIG_PROC_FS)] pub proc_entry: *mut proc_dir_entry,
    #[cfg(CONFIG_PROC_FS)] pub proc_file_name: [u8; IFNAMSIZ],
    pub bond_list: list_head, pub rr_tx_counter: *mut u32, pub ad_info: ad_bond_info,
    pub alb_info: alb_bond_info, pub params: bond_params, pub wq: *mut workqueue_struct,
    pub mii_work: delayed_work, pub arp_work: delayed_work, pub alb_work: delayed_work,
    pub ad_work: delayed_work, pub mcast_work: delayed_work, pub slave_arr_work: delayed_work,
    pub peer_notify_work: delayed_work,
    #[cfg(CONFIG_DEBUG_FS)] pub debug_dir: *mut dentry,
    pub bond_stats: rtnl_link_stats64,
    #[cfg(CONFIG_XFRM_OFFLOAD)] pub ipsec_list: list_head,
    #[cfg(CONFIG_XFRM_OFFLOAD)] pub ipsec_lock: mutex,
    pub xdp_prog: *mut bpf_prog,
}

extern "C" {
    pub fn bond_queue_slave_event(slave: *mut slave);
    pub fn bond_lower_state_changed(slave: *mut slave);
    pub fn bond_rcv_validate(skb: *const sk_buff, bond: *mut bonding, slave: *mut slave) -> i32;
    pub fn bond_dev_queue_xmit(bond: *mut bonding, skb: *mut sk_buff, slave_dev: *mut net_device) -> netdev_tx_t;
    pub fn bond_create(net: *mut net, name: *const ::core::ffi::c_char) -> i32;
    pub fn bond_destroy_sysfs(net: *mut bond_net);
    pub fn bond_enslave(bond_dev: *mut net_device, slave_dev: *mut net_device, extack: *mut netlink_ext_ack) -> i32;
    pub fn bond_release(bond_dev: *mut net_device, slave_dev: *mut net_device) -> i32;
}

/* The remaining inline operations and macros are direct kernel expressions;
 * these declarations preserve their source-level interfaces for the dependent
 * translation unit. */
extern "C" {
    pub fn bond_get_slave_by_dev(bond: *mut bonding, slave_dev: *mut net_device) -> *mut slave;
    pub fn bond_get_bond_by_slave(slave: *mut slave) -> *mut bonding;
    pub fn bond_slave_state_change(bond: *mut bonding);
    pub fn bond_slave_state_notify(bond: *mut bonding);
    pub fn bond_is_active_slave(slave: *mut slave) -> bool;
    pub fn bond_slave_can_tx(slave: *mut slave) -> bool;
    pub fn bond_hw_addr_copy(dst: *mut u8, src: *const u8, len: u32);
    pub fn slave_do_arp_validate(bond: *mut bonding, slave: *mut slave) -> i32;
    pub fn slave_do_arp_validate_only(bond: *mut bonding) -> i32;
    pub fn bond_is_ip_target_ok(addr: __be32) -> i32;
    pub fn slave_oldest_target_arp_rx(bond: *mut bonding, slave: *mut slave) -> ::core::ffi::c_ulong;
    pub fn slave_last_rx(bond: *mut bonding, slave: *mut slave) -> ::core::ffi::c_ulong;
    pub fn slave_update_last_tx(slave: *mut slave);
    pub fn slave_last_tx(slave: *mut slave) -> ::core::ffi::c_ulong;
    pub fn bond_confirm_addr(dev: *mut net_device, dst: __be32, local: __be32) -> __be32;
    pub fn bond_get_targets_ip(targets: *mut __be32, ip: __be32) -> i32;
    pub fn bond_tx_drop(dev: *mut net_device, skb: *mut sk_buff) -> netdev_tx_t;
    pub fn bond_create_sysfs(net: *mut bond_net) -> i32;
    pub fn bond_prepare_sysfs_group(bond: *mut bonding);
    pub fn bond_sysfs_slave_add(slave: *mut slave) -> i32;
    pub fn bond_sysfs_slave_del(slave: *mut slave);
    pub fn bond_xdp_set_features(bond_dev: *mut net_device);
    pub fn bond_xmit_hash(bond: *mut bonding, skb: *mut sk_buff) -> u32;
    pub fn bond_set_carrier(bond: *mut bonding) -> i32;
    pub fn bond_select_active_slave(bond: *mut bonding);
    pub fn bond_change_active_slave(bond: *mut bonding, new_active: *mut slave);
    pub fn bond_create_debugfs();
    pub fn bond_destroy_debugfs();
    pub fn bond_debug_register(bond: *mut bonding);
    pub fn bond_debug_unregister(bond: *mut bonding);
    pub fn bond_debug_reregister(bond: *mut bonding);
    pub fn bond_mode_name(mode: i32) -> *const ::core::ffi::c_char;
    pub fn __bond_xdp_check(mode: i32, xmit_policy: i32) -> bool;
    pub fn bond_xdp_check(bond: *mut bonding, mode: i32) -> bool;
    pub fn bond_setup(bond_dev: *mut net_device);
    pub fn bond_get_num_tx_queues() -> u32;
    pub fn bond_netlink_init() -> i32;
    pub fn bond_netlink_fini();
    pub fn bond_option_active_slave_get_rcu(bond: *const bonding) -> *mut net_device;
    pub fn bond_slave_link_status(link: i8) -> *const ::core::ffi::c_char;
    pub fn bond_verify_device_path(start_dev: *mut net_device, end_dev: *mut net_device, level: i32) -> *mut bond_vlan_tag;
    pub fn bond_update_slave_arr(bond: *mut bonding, skipslave: *mut slave) -> i32;
    pub fn bond_slave_arr_work_rearm(bond: *mut bonding, delay: ::core::ffi::c_ulong);
    pub fn bond_peer_notify_work_rearm(bond: *mut bonding, delay: ::core::ffi::c_ulong);
    pub fn bond_work_init_all(bond: *mut bonding);
    pub fn bond_work_cancel_all(bond: *mut bonding);
    pub fn bond_slave_has_mac(bond: *mut bonding, mac: *const u8) -> *mut slave;
    pub fn bond_slave_has_mac_rcu(bond: *mut bonding, mac: *const u8) -> bool;
}

/* External objects exported by the corresponding bonding implementation files. */
extern "C" {
    pub static mut bond_net_id: u32;
    pub static mut bond_link_ops: rtnl_link_ops;
    pub static slave_sysfs_ops: sysfs_ops;
    pub static lacpdu_mcast_addr: u8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
