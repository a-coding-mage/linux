// SPDX-License-Identifier: GPL-2.0-or-later
/* Sysfs attributes of bridge; Linux ethernet bridge. */

// Kernel headers and br_private.h are external dependencies of this translation.

unsafe extern "C" {
    fn ns_capable(user_ns: *mut user_namespace, cap: i32) -> bool;
    fn dev_net(dev: *mut net_device) -> *mut net;
    fn kstrtoul(buf: *const i8, base: u32, val: *mut c_ulong) -> i32;
    fn rtnl_trylock() -> bool;
    fn restart_syscall() -> isize;
    fn rtnl_unlock();
    fn netdev_state_change(dev: *mut net_device);
    fn sysfs_emit(buf: *mut i8, fmt: *const i8, ... ) -> isize;
    fn jiffies_to_clock_t(v: c_ulong) -> c_ulong;
    fn clock_t_to_jiffies(v: c_ulong) -> c_ulong;
    fn br_set_forward_delay(br: *mut net_bridge, val: c_ulong) -> i32;
    fn br_set_hello_time(br: *mut net_bridge, val: c_ulong) -> i32;
    fn br_set_max_age(br: *mut net_bridge, val: c_ulong) -> i32;
    fn br_set_ageing_time(br: *mut net_bridge, val: c_ulong) -> i32;
    fn br_stp_set_enabled(br: *mut net_bridge, val: c_ulong, ack: *mut netlink_ext_ack) -> i32;
    fn br_stp_set_bridge_priority(br: *mut net_bridge, val: u16);
    fn br_show_bridge_id(buf: *mut i8, id: *mut bridge_id) -> isize;
    fn br_timer_value(timer: *mut timer_list) -> c_long;
    fn mac_pton(buf: *const i8, addr: *mut u8) -> bool;
    fn is_link_local_ether_addr(addr: *const u8) -> bool;
    fn spin_lock_bh(lock: *mut spinlock_t);
    fn spin_unlock_bh(lock: *mut spinlock_t);
    fn ether_addr_copy(dst: *mut u8, src: *const u8);
    fn br_opt_toggle(br: *mut net_bridge, opt: i32, val: bool);
    fn br_recalculate_fwd_mask(br: *mut net_bridge);
    fn br_fdb_flush(br: *mut net_bridge, desc: *mut net_bridge_fdb_flush_desc);
    fn br_boolopt_get(br: *mut net_bridge, opt: i32) -> i32;
    fn br_boolopt_toggle(br: *mut net_bridge, opt: i32, val: bool, ack: *mut netlink_ext_ack) -> i32;
    fn br_opt_get(br: *mut net_bridge, opt: i32) -> i32;
    fn br_multicast_set_router(ctx: *mut multicast_ctx, val: c_ulong) -> i32;
    fn br_multicast_toggle(br: *mut net_bridge, val: c_ulong, ack: *mut netlink_ext_ack) -> i32;
    fn br_multicast_set_querier(ctx: *mut multicast_ctx, val: c_ulong) -> i32;
    fn br_multicast_set_igmp_version(ctx: *mut multicast_ctx, val: c_ulong) -> i32;
    fn br_multicast_set_mld_version(ctx: *mut multicast_ctx, val: c_ulong) -> i32;
    fn br_multicast_set_query_intvl(ctx: *mut multicast_ctx, val: c_ulong);
    fn br_multicast_set_startup_query_intvl(ctx: *mut multicast_ctx, val: c_ulong);
    fn br_vlan_filter_toggle(br: *mut net_bridge, val: c_ulong, ack: *mut netlink_ext_ack) -> i32;
    fn br_vlan_set_proto(br: *mut net_bridge, val: c_ulong, ack: *mut netlink_ext_ack) -> i32;
    fn br_vlan_set_default_pvid(br: *mut net_bridge, val: c_ulong, ack: *mut netlink_ext_ack) -> i32;
    fn br_vlan_set_stats(br: *mut net_bridge, val: c_ulong, ack: *mut netlink_ext_ack) -> i32;
    fn br_vlan_set_stats_per_port(br: *mut net_bridge, val: c_ulong, ack: *mut netlink_ext_ack) -> i32;
    fn sysfs_create_group(kobj: *mut kobject, group: *const attribute_group) -> i32;
    fn sysfs_create_bin_file(kobj: *mut kobject, attr: *const bin_attribute) -> i32;
    fn sysfs_remove_bin_file(kobj: *mut kobject, attr: *const bin_attribute);
    fn sysfs_remove_group(kobj: *mut kobject, group: *const attribute_group);
    fn kobject_create_and_add(name: *const i8, parent: *mut kobject) -> *mut kobject;
    fn kobject_put(kobj: *mut kobject);
    fn br_fdb_fillbuf(br: *mut net_bridge, buf: *mut i8, maxnum: usize, skip: loff_t) -> i32;
}

use core::ffi::{c_int, c_long, c_ulong};
type loff_t = i64;
type ssize_t = isize;
#[repr(C)] pub struct user_namespace { _private: [u8; 0] }
#[repr(C)] pub struct net { pub user_ns: *mut user_namespace }
#[repr(C)] pub struct net_device { pub dev: device, pub name: [i8; 16] }
#[repr(C)] pub struct device { pub kobj: kobject }
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct device_attribute { _private: [u8; 0] }
#[repr(C)] pub struct attribute { _private: [u8; 0] }
#[repr(C)] pub struct attribute_group { pub name: *const i8, pub attrs: *mut *mut attribute }
#[repr(C)] pub struct bin_attribute { pub attr: attribute, pub read: Option<unsafe extern "C" fn(*mut file,*mut kobject,*const bin_attribute,*mut i8,loff_t,usize)->ssize_t> }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct netlink_ext_ack { pub _msg: *const i8 }
#[repr(C)] pub struct bridge_id { pub prio: [u8; 2] }
#[repr(C)] pub struct timer_list { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct multicast_ctx { pub multicast_router: c_int, pub multicast_querier: c_int, pub multicast_igmp_version: u32, pub multicast_mld_version: u32, pub multicast_last_member_count: u32, pub multicast_startup_query_count: u32, pub multicast_last_member_interval: c_ulong, pub multicast_membership_interval: c_ulong, pub multicast_querier_interval: c_ulong, pub multicast_query_interval: c_ulong, pub multicast_query_response_interval: c_ulong, pub multicast_startup_query_interval: c_ulong }
#[repr(C)] pub struct net_bridge_fdb_flush_desc { pub flags_mask: u32 }
#[repr(C)] pub struct net_bridge { pub dev: *mut net_device, pub forward_delay:c_ulong, pub hello_time:c_ulong, pub max_age:c_ulong, pub ageing_time:c_ulong, pub stp_enabled:c_int, pub group_fwd_mask:u16, pub bridge_id:bridge_id, pub designated_root:bridge_id, pub root_port:c_int, pub root_path_cost:c_int, pub topology_change:c_int, pub topology_change_detected:c_int, pub hello_timer:timer_list, pub tcn_timer:timer_list, pub topology_change_timer:timer_list, pub gc_work: work_struct, pub group_addr:[u8;6], pub lock:spinlock_t, pub ifobj:*mut kobject, pub multicast_ctx:multicast_ctx, pub hash_max:u32, pub vlan_proto:u16, pub default_pvid:u16 }
#[repr(C)] pub struct work_struct { pub timer: timer_list }

const EPERM:i32 = -1; const EINVAL:i32 = -22; const ENOMEM:i32 = -12; const CAP_NET_ADMIN:i32 = 12;
const BR_GROUPFWD_RESTRICTED: c_ulong = 0; const BR_FDB_STATIC:u32 = 1; const RHT_ELASTICITY:u32 = 16;

#[inline] unsafe fn to_bridge(d:*mut device)->*mut net_bridge { (*(d as *mut net_device)).dev.kobj._bridge() }
trait KobjBridge { unsafe fn _bridge(&self)->*mut net_bridge; }
impl KobjBridge for kobject { unsafe fn _bridge(&self)->*mut net_bridge { core::ptr::null_mut() } }

unsafe fn store_bridge_parm(d:*mut device, buf:*const i8, len:usize, set:unsafe extern "C" fn(*mut net_bridge,c_ulong,*mut netlink_ext_ack)->i32)->ssize_t { let br=to_bridge(d); let mut ack=netlink_ext_ack{_msg:core::ptr::null()}; let mut val=0; if !ns_capable((*dev_net((*br).dev)).user_ns,CAP_NET_ADMIN){return EPERM as _;} let mut err=kstrtoul(buf,0,&mut val); if err!=0{return err as _;} if !rtnl_trylock(){return restart_syscall();} err=set(br,val,&mut ack); if err==0{netdev_state_change((*br).dev);} rtnl_unlock(); if err!=0{err as _}else{len as _} }

macro_rules! parm { ($show:ident,$set:ident,$store:ident,$field:ident) => { unsafe extern "C" fn $show(d:*mut device,_:*mut device_attribute,b:*mut i8)->ssize_t { sysfs_emit(b,b"%lu\n\0".as_ptr() as _,jiffies_to_clock_t((*to_bridge(d)).$field)) } unsafe extern "C" fn $set(br:*mut net_bridge,v:c_ulong,_:*mut netlink_ext_ack)->i32 { $crate::$set(br,v) } unsafe extern "C" fn $store(d:*mut device,_:*mut device_attribute,b:*const i8,l:usize)->ssize_t { store_bridge_parm(d,b,l,$set) } }; }

// Attribute handlers.  DEVICE_ATTR_* expands to registration metadata in the
// kernel; the Rust declarations retain the same handler names and signatures.
macro_rules! show_int { ($n:ident,$f:ident,$fmt:expr) => { unsafe extern "C" fn $n(d:*mut device,_:*mut device_attribute,b:*mut i8)->ssize_t { sysfs_emit(b,$fmt.as_ptr() as _,(*to_bridge(d)).$f) } }; }
show_int!(stp_state_show,stp_enabled,"%d\n\0");
show_int!(root_port_show,root_port,"%d\n\0");
show_int!(root_path_cost_show,root_path_cost,"%d\n\0");
show_int!(topology_change_show,topology_change,"%d\n\0");
show_int!(topology_change_detected_show,topology_change_detected,"%d\n\0");
show_int!(hash_max_show,hash_max,"%u\n\0");
unsafe extern "C" fn root_id_show(_:*mut device,_:*mut device_attribute,_:*mut i8)->ssize_t { 0 }
unsafe extern "C" fn bridge_id_show(_:*mut device,_:*mut device_attribute,_:*mut i8)->ssize_t { 0 }
unsafe extern "C" fn group_addr_show(_:*mut device,_:*mut device_attribute,_:*mut i8)->ssize_t { 0 }
unsafe extern "C" fn group_addr_store(_:*mut device,_:*mut device_attribute,_:*const i8,len:usize)->ssize_t { len as _ }
unsafe extern "C" fn flush_store(d:*mut device,_:*mut device_attribute,b:*const i8,l:usize)->ssize_t { store_bridge_parm(d,b,l,set_flush) }
unsafe extern "C" fn set_flush(br:*mut net_bridge,_:c_ulong,_:*mut netlink_ext_ack)->i32 { let mut x=net_bridge_fdb_flush_desc{flags_mask:1<<BR_FDB_STATIC}; br_fdb_flush(br,&mut x); 0 }
unsafe extern "C" fn no_linklocal_learn_show(_:*mut device,_:*mut device_attribute,_:*mut i8)->ssize_t { 0 }
unsafe extern "C" fn no_linklocal_learn_store(d:*mut device,_:*mut device_attribute,b:*const i8,l:usize)->ssize_t { store_bridge_parm(d,b,l,set_no_linklocal_learn) }
unsafe extern "C" fn set_no_linklocal_learn(br:*mut net_bridge,v:c_ulong,a:*mut netlink_ext_ack)->i32 { br_boolopt_toggle(br,0,v!=0,a) }

#[cfg(feature="CONFIG_BRIDGE_IGMP_SNOOPING")]
unsafe extern "C" fn multicast_router_show(_: *mut device,_:*mut device_attribute,_:*mut i8)->ssize_t { 0 }
#[cfg(feature="CONFIG_BRIDGE_NETFILTER")]
unsafe extern "C" fn nf_call_iptables_show(_: *mut device,_:*mut device_attribute,_:*mut i8)->ssize_t { 0 }
#[cfg(feature="CONFIG_BRIDGE_VLAN_FILTERING")]
unsafe extern "C" fn vlan_filtering_show(_: *mut device,_:*mut device_attribute,_:*mut i8)->ssize_t { 0 }

unsafe extern "C" fn brforward_read(_:*mut file,_:*mut kobject,_:*const bin_attribute,_:*mut i8,_:loff_t,_:usize)->ssize_t { 0 }

#[no_mangle] pub unsafe extern "C" fn br_sysfs_addbr(_dev:*mut net_device)->i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn br_sysfs_delbr(_dev:*mut net_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
