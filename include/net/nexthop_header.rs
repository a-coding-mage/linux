/* SPDX-License-Identifier: GPL-2.0 */
/* Generic nexthop implementation; translated from nexthop.h. */

// External kernel types and functions referenced below are supplied by other translated units.
pub const NEXTHOP_VALID_USER_FLAGS: u32 = RTNH_F_ONLINK;

#[repr(C)] pub struct nexthop { pub rb_node: rb_node, pub fi_list: list_head, pub f6i_list: list_head, pub fdb_list: list_head, pub grp_list: list_head, pub net: *mut net, pub id: u32, pub protocol: u8, pub nh_flags: u8, pub is_group: bool, pub dead: bool, pub lock: spinlock_t, pub refcnt: refcount_t, pub rcu: rcu_head, pub data: nexthop_data }
#[repr(C)] pub union nexthop_data { pub nh_info: *mut nh_info, pub nh_grp: *mut nh_group }

#[repr(C)] pub union nh_gateway { pub ipv4: __be32, pub ipv6: in6_addr }
#[repr(C)] pub struct nh_config { pub nh_id: u32, pub nh_family: u8, pub nh_protocol: u8, pub nh_blackhole: u8, pub nh_fdb: u8, pub nh_dst_port: __be16, pub nh_flags: u32, pub nh_ifindex: c_int, pub dev: *mut net_device, pub gw: nh_gateway, pub nh_grp: *mut nlattr, pub nh_grp_type: u16, pub nh_grp_res_num_buckets: u16, pub nh_grp_res_idle_timer: c_ulong, pub nh_grp_res_unbalanced_timer: c_ulong, pub nh_grp_res_has_num_buckets: bool, pub nh_grp_res_has_idle_timer: bool, pub nh_grp_res_has_unbalanced_timer: bool, pub nh_hw_stats: bool, pub nh_encap: *mut nlattr, pub nh_encap_type: u16, pub nlflags: u32, pub nlinfo: nl_info }

#[repr(C)] pub union nh_info_data { pub fib_nhc: fib_nh_common, pub fib_nh: fib_nh, pub fib6_nh: fib6_nh }
#[repr(C)] pub struct nh_info { pub dev_hash: hlist_node, pub nh_parent: *mut nexthop, pub family: u8, pub reject_nh: bool, pub fdb_nh: bool, pub dst_port: __be16, pub data: nh_info_data }
#[repr(C)] pub struct nh_res_bucket { pub nh_entry: *mut nh_grp_entry, pub used_time: atomic_long_t, pub migrated_time: c_ulong, pub occupied: bool, pub nh_flags: u8 }
#[repr(C)] pub struct nh_res_table { pub net: *mut net, pub nhg_id: u32, pub upkeep_dw: delayed_work, pub uw_nh_entries: list_head, pub unbalanced_since: c_ulong, pub idle_timer: u32, pub unbalanced_timer: u32, pub num_nh_buckets: u16, pub nh_buckets: [nh_res_bucket; 0] }
#[repr(C)] pub struct nh_grp_entry_stats { pub packets: u64_stats_t, pub syncp: u64_stats_sync }
#[repr(C)] pub union nh_grp_entry_data { pub hthr: nh_grp_entry_hthr, pub res: nh_grp_entry_res }
#[repr(C)] pub struct nh_grp_entry_hthr { pub upper_bound: atomic_t }
#[repr(C)] pub struct nh_grp_entry_res { pub uw_nh_entry: list_head, pub count_buckets: u16, pub wants_buckets: u16 }
#[repr(C)] pub struct nh_grp_entry { pub nh: *mut nexthop, pub stats: *mut nh_grp_entry_stats, pub weight: u16, pub data: nh_grp_entry_data, pub nh_list: list_head, pub nh_parent: *mut nexthop, pub packets_hw: u64 }
#[repr(C)] pub struct nh_group { pub spare: *mut nh_group, pub num_nh: u16, pub is_multipath: bool, pub hash_threshold: bool, pub resilient: bool, pub fdb_nh: bool, pub has_v4: bool, pub hw_stats: bool, pub res_table: *mut nh_res_table, pub nh_entries: [nh_grp_entry; 0] }

#[repr(C)] pub enum nexthop_event_type { NEXTHOP_EVENT_DEL, NEXTHOP_EVENT_REPLACE, NEXTHOP_EVENT_RES_TABLE_PRE_REPLACE, NEXTHOP_EVENT_BUCKET_REPLACE, NEXTHOP_EVENT_HW_STATS_REPORT_DELTA }
#[repr(C)] pub enum nh_notifier_info_type { NH_NOTIFIER_INFO_TYPE_SINGLE, NH_NOTIFIER_INFO_TYPE_GRP, NH_NOTIFIER_INFO_TYPE_RES_TABLE, NH_NOTIFIER_INFO_TYPE_RES_BUCKET, NH_NOTIFIER_INFO_TYPE_GRP_HW_STATS }
#[repr(C)] pub union nh_notifier_single_gw { pub ipv4: __be32, pub ipv6: in6_addr }
#[repr(C)] pub struct nh_notifier_single_info { pub dev: *mut net_device, pub gw_family: u8, pub gw: nh_notifier_single_gw, pub id: u32, pub is_reject: bool, pub is_fdb: bool, pub has_encap: bool }
#[repr(C)] pub struct nh_notifier_grp_entry_info { pub weight: u16, pub nh: nh_notifier_single_info }
#[repr(C)] pub struct nh_notifier_grp_info { pub num_nh: u16, pub is_fdb: bool, pub hw_stats: bool, pub nh_entries: [nh_notifier_grp_entry_info; 0] }
#[repr(C)] pub struct nh_notifier_res_bucket_info { pub bucket_index: u16, pub idle_timer_ms: c_uint, pub force: bool, pub old_nh: nh_notifier_single_info, pub new_nh: nh_notifier_single_info }
#[repr(C)] pub struct nh_notifier_res_table_info { pub num_nh_buckets: u16, pub hw_stats: bool, pub nhs: [nh_notifier_single_info; 0] }
#[repr(C)] pub struct nh_notifier_grp_hw_stats_entry_info { pub id: u32, pub packets: u64 }
#[repr(C)] pub struct nh_notifier_grp_hw_stats_info { pub num_nh: u16, pub hw_stats_used: bool, pub stats: [nh_notifier_grp_hw_stats_entry_info; 0] }
#[repr(C)] pub union nh_notifier_info_data { pub nh: *mut nh_notifier_single_info, pub nh_grp: *mut nh_notifier_grp_info, pub nh_res_table: *mut nh_notifier_res_table_info, pub nh_res_bucket: *mut nh_notifier_res_bucket_info, pub nh_grp_hw_stats: *mut nh_notifier_grp_hw_stats_info }
#[repr(C)] pub struct nh_notifier_info { pub net: *mut net, pub extack: *mut netlink_ext_ack, pub id: u32, pub r#type: nh_notifier_info_type, pub data: nh_notifier_info_data }

extern "C" { pub fn register_nexthop_notifier(net: *mut net, nb: *mut notifier_block, extack: *mut netlink_ext_ack) -> c_int; pub fn __unregister_nexthop_notifier(net: *mut net, nb: *mut notifier_block) -> c_int; pub fn unregister_nexthop_notifier(net: *mut net, nb: *mut notifier_block) -> c_int; pub fn nexthop_set_hw_flags(net: *mut net, id: u32, offload: bool, trap: bool); pub fn nexthop_bucket_set_hw_flags(net: *mut net, id: u32, bucket_index: u16, offload: bool, trap: bool); pub fn nexthop_res_grp_activity_update(net: *mut net, id: u32, num_buckets: u16, activity: *mut c_ulong); pub fn nh_grp_hw_stats_report_delta(info: *mut nh_notifier_grp_hw_stats_info, nh_idx: c_uint, delta_packets: u64); pub fn nexthop_find_by_id(net: *mut net, id: u32) -> *mut nexthop; pub fn nexthop_free_rcu(head: *mut rcu_head); pub fn nexthop_select_path(nh: *mut nexthop, hash: c_int) -> *mut nexthop; pub fn fib_add_nexthop(skb: *mut sk_buff, nhc: *mut fib_nh_common, weight: c_int, rt_family: u8, flags: c_int) -> c_int; pub fn fib_check_nexthop(nh: *mut nexthop, scope: u8, extack: *mut netlink_ext_ack) -> c_int; pub fn fib6_check_nexthop(nh: *mut nexthop, cfg: *mut fib6_config, extack: *mut netlink_ext_ack) -> c_int; pub fn nexthop_for_each_fib6_nh(nh: *mut nexthop, cb: extern "C" fn(*mut fib6_nh, *mut c_void) -> c_int, arg: *mut c_void) -> c_int }

#[inline] pub unsafe fn nexthop_get(nh: *mut nexthop) -> bool { refcount_inc_not_zero(&mut (*nh).refcnt) }
#[inline] pub unsafe fn nexthop_put(nh: *mut nexthop) { if refcount_dec_and_test(&mut (*nh).refcnt) { call_rcu_hurry(&mut (*nh).rcu, nexthop_free_rcu); } }
#[inline] pub unsafe fn nexthop_cmp(nh1: *const nexthop, nh2: *const nexthop) -> bool { nh1 == nh2 }
#[inline] pub unsafe fn nexthop_mpath_select(nhg: *const nh_group, nhsel: c_int) -> *mut nexthop { if nhsel < 0 || nhsel as u16 >= (*nhg).num_nh { return core::ptr::null_mut(); } (*nhg).nh_entries.as_ptr().add(nhsel as usize).cast::<nh_grp_entry>().read().nh }
#[inline] pub unsafe fn nexthop_num_path(nh: *const nexthop) -> c_uint { if (*nh).is_group { let g = (*nh).data.nh_grp; if (*g).is_multipath { return (*g).num_nh as c_uint; } } 1 }
#[inline] pub unsafe fn fib_info_num_path(fi: *const fib_info) -> c_uint { if !(*fi).nh.is_null() { nexthop_num_path((*fi).nh) } else { (*fi).fib_nhs as c_uint } }

#[inline] pub unsafe fn nexthop_is_fdb(nh: *const nexthop) -> bool { if (*nh).is_group { (*(*nh).data.nh_grp).fdb_nh } else { (*(*nh).data.nh_info).fdb_nh } }
#[inline] pub unsafe fn nexthop_has_v4(nh: *const nexthop) -> bool { (*nh).is_group && (*(*nh).data.nh_grp).has_v4 }
#[inline] pub unsafe fn nexthop_is_multipath(nh: *const nexthop) -> bool { (*nh).is_group && (*(*nh).data.nh_grp).is_multipath }
#[inline] pub unsafe fn nexthop_get_family(nh: *mut nexthop) -> c_int { (*(*nh).data.nh_info).family as c_int }
#[inline] pub unsafe fn nexthop_is_blackhole(mut nh: *const nexthop) -> bool { if (*nh).is_group { let g=(*nh).data.nh_grp; if (*g).num_nh > 1 { return false; } nh=(*g).nh_entries.as_ptr().cast::<nh_grp_entry>().read().nh; } (*(*nh).data.nh_info).reject_nh }
#[inline] pub unsafe fn nexthop_fdb_nhc(nh: *mut nexthop) -> *mut fib_nh_common { &mut (*(*nh).data.nh_info).data.fib_nhc }
#[inline] pub unsafe fn fib_info_nhc(fi: *mut fib_info, nhsel: c_int) -> *mut fib_nh_common { if !(*fi).nh.is_null() { nexthop_fib_nhc((*fi).nh, nhsel) } else { &mut (*fi).fib_nh.add(nhsel as usize).read().nh_common } }
#[inline] pub unsafe fn fib_info_nh(fi: *mut fib_info, nhsel: c_int) -> *mut fib_nh { &mut (*fi).fib_nh.add(nhsel as usize).read().nh_common }
#[inline] pub unsafe fn nexthop_fib_nhc(nh: *mut nexthop, nhsel: c_int) -> *mut fib_nh_common { if (*nh).is_group && (*(*nh).data.nh_grp).is_multipath { nh=nexthop_mpath_select((*nh).data.nh_grp, nhsel); if nh.is_null(){return core::ptr::null_mut();} } &mut (*(*nh).data.nh_info).data.fib_nhc }
#[inline] pub unsafe fn nexthop_fib6_nh(nh: *mut nexthop) -> *mut fib6_nh { if (*nh).is_group { nh=nexthop_mpath_select((*nh).data.nh_grp,0); if nh.is_null(){return core::ptr::null_mut();} } let i=&mut *(*nh).data.nh_info; if i.family==AF_INET6 { &mut i.data.fib6_nh } else { core::ptr::null_mut() } }
#[inline] pub unsafe fn fib6_info_nh_dev(f6i: *mut fib6_info) -> *mut net_device { let n=if !(*f6i).nh.is_null(){nexthop_fib6_nh((*f6i).nh)}else{(*f6i).fib6_nh}; (*n).fib_nh_dev }
#[inline] pub unsafe fn nexthop_uses_dev(nh: *const nexthop, dev: *const net_device) -> bool { if (*nh).is_group { let g=(*nh).data.nh_grp; for i in 0..(*g).num_nh as usize { let n=(*g).nh_entries.as_ptr().add(i).read().nh; if nhc_l3mdev_matches_dev(&(*(*n).data.nh_info).data.fib_nhc,dev){return true;} } } else if nhc_l3mdev_matches_dev(&(*(*nh).data.nh_info).data.fib_nhc,dev){return true;} false }
extern "C" { pub fn nhc_l3mdev_matches_dev(nhc: *const fib_nh_common, dev: *const net_device) -> bool; pub fn refcount_inc_not_zero(r: *mut refcount_t)->bool; pub fn refcount_dec_and_test(r: *mut refcount_t)->bool; pub fn call_rcu_hurry(r: *mut rcu_head, f: unsafe extern "C" fn(*mut rcu_head)); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
