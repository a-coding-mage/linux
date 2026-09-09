// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2020 Facebook Inc.
// Translated from udp_tunnel_nic.c. Kernel includes and symbols are supplied by dependencies.

#[repr(u8)]
enum UdpTunnelNicTableEntryFlags {
    Add = 1 << 0,
    Del = 1 << 1,
    OpFail = 1 << 2,
    Frozen = 1 << 3,
}

#[repr(C)]
struct UdpTunnelNicTableEntry {
    port: u16,
    type_: u8,
    flags: u8,
    use_cnt: u16,
    hw_priv: u8,
}

const UDP_TUNNEL_NIC_USE_CNT_MAX: u16 = u16::MAX;

#[repr(C)]
struct UdpTunnelNic {
    work: DelayedWork,
    dev: *mut NetDevice,
    lock: Mutex,
    need_sync: bool,
    need_replay: bool,
    n_tables: c_uint,
    missed: c_ulong,
    entries: *mut *mut UdpTunnelNicTableEntry,
}

// External kernel types and functions referenced by this translation.
type c_uint = u32;
type c_ulong = usize;
#[allow(non_camel_case_types)] type size_t = usize;
#[repr(C)] struct DelayedWork { _private: [u8; 0] }
#[repr(C)] struct WorkStruct { _private: [u8; 0] }
#[repr(C)] struct Mutex { _private: [u8; 0] }
#[repr(C)] struct NetDevice { udp_tunnel_nic_info: *const UdpTunnelNicInfo, udp_tunnel_nic: *mut UdpTunnelNic }
#[repr(C)] struct UdpTunnelInfo { port: u16, type_: u8, sa_family: u16, hw_priv: u8 }
#[repr(C)] struct UdpTunnelNicTableInfo { tunnel_types: u32, n_entries: c_uint }
#[repr(C)] struct UdpTunnelNicShared { udp_tunnel_nic_info: *mut UdpTunnelNic, devices: ListHead }
#[repr(C)] struct UdpTunnelNicSharedNode { dev: *mut NetDevice, list: ListHead }
#[repr(C)] struct UdpTunnelNicInfo { flags: u32, tables: *const UdpTunnelNicTableInfo, set_port: Option<unsafe extern "C" fn(*mut NetDevice,c_uint,c_uint,*mut UdpTunnelInfo)->i32>, unset_port: Option<unsafe extern "C" fn(*mut NetDevice,c_uint,c_uint,*mut UdpTunnelInfo)->i32>, sync_table: Option<unsafe extern "C" fn(*mut NetDevice,c_uint)->i32>, shared: *mut UdpTunnelNicShared }
#[repr(C)] struct SkBuff { _private: [u8; 0] }
#[repr(C)] struct NlAttr { _private: [u8; 0] }
#[repr(C)] struct NotifierBlock { notifier_call: Option<unsafe extern "C" fn(*mut NotifierBlock, c_ulong, *mut core::ffi::c_void)->i32> }
#[repr(C)] struct ListHead { next: *mut ListHead, prev: *mut ListHead }

extern "C" {
    static mut udp_tunnel_nic_workqueue: *mut core::ffi::c_void;
    static mut udp_tunnel_nic_ops: *const UdpTunnelNicOps;
    fn memset(dst: *mut core::ffi::c_void, val: i32, n: usize) -> *mut core::ffi::c_void;
    fn htons(v: u16) -> u16;
    fn be16_to_cpu(v: u16) -> u16;
    fn ilog2(v: u8) -> u32;
    fn test_bit(n: c_uint, p: *const c_ulong) -> bool;
    fn set_bit(n: c_uint, p: *mut c_ulong);
    fn queue_delayed_work(wq: *mut core::ffi::c_void, work: *mut DelayedWork, delay: c_ulong);
    fn netif_running(dev: *mut NetDevice) -> bool;
    fn rtnl_trylock() -> bool; fn rtnl_lock(); fn rtnl_unlock();
    fn mutex_lock(m: *mut Mutex); fn mutex_unlock(m: *mut Mutex);
    fn udp_tunnel_get_rx_info(dev: *mut NetDevice); fn udp_tunnel_drop_rx_info(dev: *mut NetDevice);
    fn dev_hold(dev: *mut NetDevice); fn dev_put(dev: *mut NetDevice);
    fn cancel_delayed_work_sync(w: *mut DelayedWork);
    fn unregister_netdevice_notifier(n: *mut NotifierBlock);
    fn register_netdevice_notifier(n: *mut NotifierBlock) -> i32;
    fn alloc_ordered_workqueue(name: *const u8, flags: u32) -> *mut core::ffi::c_void;
    fn destroy_workqueue(wq: *mut core::ffi::c_void);
    fn nla_total_size(n: usize) -> usize; fn nla_nest_start(s: *mut SkBuff, t: u16) -> *mut NlAttr;
    fn nla_put_be16(s: *mut SkBuff,t: u16,v:u16)->i32; fn nla_put_u32(s:*mut SkBuff,t:u16,v:u32)->i32;
    fn nla_nest_end(s:*mut SkBuff,n:*mut NlAttr); fn nla_nest_cancel(s:*mut SkBuff,n:*mut NlAttr);
}

const UDP_TUNNEL_TYPE_VXLAN:u8=1; const UDP_TUNNEL_TYPE_GENEVE:u8=2; const UDP_TUNNEL_TYPE_VXLAN_GPE:u8=4;
const UDP_TUNNEL_NIC_ENTRY_ADD:u8=1; const UDP_TUNNEL_NIC_ENTRY_DEL:u8=2; const UDP_TUNNEL_NIC_ENTRY_OP_FAIL:u8=4; const UDP_TUNNEL_NIC_ENTRY_FROZEN:u8=8;

unsafe fn udp_tunnel_nic_tunnel_type_name(t: u32) -> &'static [u8] { match t { 1=>b"vxlan\0",2=>b"geneve\0",4=>b"vxlan-gpe\0",_=>b"unknown\0" } }
unsafe fn entry_free(e:*mut UdpTunnelNicTableEntry)->bool { (*e).use_cnt==0 && (*e).flags==0 }
unsafe fn entry_present(e:*mut UdpTunnelNicTableEntry)->bool { (*e).use_cnt!=0 && ((*e).flags & !UDP_TUNNEL_NIC_ENTRY_FROZEN)==0 }
unsafe fn entry_frozen(e:*mut UdpTunnelNicTableEntry)->bool { (*e).flags & UDP_TUNNEL_NIC_ENTRY_FROZEN != 0 }
unsafe fn entry_freeze_used(e:*mut UdpTunnelNicTableEntry) { if !entry_free(e) { (*e).flags|=UDP_TUNNEL_NIC_ENTRY_FROZEN; } }
unsafe fn entry_unfreeze(e:*mut UdpTunnelNicTableEntry) { (*e).flags &= !UDP_TUNNEL_NIC_ENTRY_FROZEN; }
unsafe fn entry_queued(e:*mut UdpTunnelNicTableEntry)->bool { (*e).flags & (UDP_TUNNEL_NIC_ENTRY_ADD|UDP_TUNNEL_NIC_ENTRY_DEL)!=0 }
unsafe fn entry_queue(u:*mut UdpTunnelNic,e:*mut UdpTunnelNic,f:u8) { (*e).flags|=f; (*u).need_sync=true; }
unsafe fn ti_from_entry(e:*mut UdpTunnelNicTableEntry,t:*mut UdpTunnelInfo) { memset(t.cast(),0,core::mem::size_of::<UdpTunnelInfo>()); (*t).port=(*e).port; (*t).type_=(*e).type_; (*t).hw_priv=(*e).hw_priv; }

unsafe fn entry_update_done(e:*mut UdpTunnelNicTableEntry,err:i32) { let dodgy=(*e).flags&UDP_TUNNEL_NIC_ENTRY_OP_FAIL!=0; if (*e).flags&UDP_TUNNEL_NIC_ENTRY_ADD!=0 && (err==0 || (err==-17&&dodgy)){(*e).flags&=!UDP_TUNNEL_NIC_ENTRY_ADD;} if (*e).flags&UDP_TUNNEL_NIC_ENTRY_DEL!=0 && (err==0 || (err==-2&&dodgy)){(*e).flags&=!UDP_TUNNEL_NIC_ENTRY_DEL;} if err==0 {(*e).flags&=!UDP_TUNNEL_NIC_ENTRY_OP_FAIL;} else {(*e).flags|=UDP_TUNNEL_NIC_ENTRY_OP_FAIL;} }

unsafe fn entry_adj(u:*mut UdpTunnelNic,table:c_uint,idx:c_uint,adj:i32) { let e=*(*u).entries.add(table as usize).add(idx as usize); let dodgy=(*e).flags&UDP_TUNNEL_NIC_ENTRY_OP_FAIL!=0; (*e).use_cnt=((*e).use_cnt as i32+adj) as u16; if !dodgy && ((*e).use_cnt==0)==(((*e).use_cnt as i32-adj)==0){return;} let (from,to)=if adj<0 {(UDP_TUNNEL_NIC_ENTRY_ADD,UDP_TUNNEL_NIC_ENTRY_DEL)}else{(UDP_TUNNEL_NIC_ENTRY_DEL,UDP_TUNNEL_NIC_ENTRY_ADD)}; if (*e).flags&from!=0 {(*e).flags&=!from;if !dodgy{return;}} entry_queue(u,e,to); }

unsafe fn entry_try_adj(u:*mut UdpTunnelNic,table:c_uint,idx:c_uint,ti:*mut UdpTunnelInfo,adj:i32)->bool { let e=*(*u).entries.add(table as usize).add(idx as usize); if entry_free(e)||(*e).port!=(*ti).port||(*e).type_!=(*ti).type_{return false;} if entry_frozen(e){return true;} entry_adj(u,table,idx,adj);true }

unsafe fn table_capable(t:*const UdpTunnelNicTableInfo,ti:*mut UdpTunnelInfo)->bool { (*t).tunnel_types & (*ti).type_ as u32 != 0 }

unsafe fn udp_tunnel_nic_device_sync_one(dev:*mut NetDevice,u:*mut UdpTunnelNic,table:c_uint,idx:c_uint) { let e=*(*u).entries.add(table as usize).add(idx as usize); if !entry_queued(e){return;} let mut ti=UdpTunnelInfo{port:0,type_:0,sa_family:0,hw_priv:0};ti_from_entry(e,&mut ti);let info=(*dev).udp_tunnel_nic_info;let err=if (*e).flags&UDP_TUNNEL_NIC_ENTRY_ADD!=0 {((*info).set_port.unwrap())(dev,table,idx,&mut ti)}else{((*info).unset_port.unwrap())(dev,table,idx,&mut ti)};entry_update_done(e,err);}

unsafe fn udp_tunnel_nic_device_sync_by_port(dev:*mut NetDevice,u:*mut UdpTunnelNic){let info=(*dev).udp_tunnel_nic_info;for i in 0..(*u).n_tables{for j in 0..(*(*info).tables.add(i as usize)).n_entries{udp_tunnel_nic_device_sync_one(dev,u,i,j);}}}
unsafe fn __udp_tunnel_nic_device_sync(dev:*mut NetDevice,u:*mut UdpTunnelNic){if !(*u).need_sync{return;}udp_tunnel_nic_device_sync_by_port(dev,u);(*u).need_sync=false;}
unsafe fn udp_tunnel_nic_device_sync(_dev:*mut NetDevice,u:*mut UdpTunnelNic){if (*u).need_sync{queue_delayed_work(udp_tunnel_nic_workqueue,&mut (*u).work,0);}}

unsafe fn udp_tunnel_nic_add_new(dev:*mut NetDevice,u:*mut UdpTunnelNic,ti:*mut UdpTunnelInfo)->bool {let info=(*dev).udp_tunnel_nic_info;for i in 0..(*u).n_tables{let t=(*info).tables.add(i as usize);if !table_capable(t,ti){continue;}for j in 0..(*t).n_entries{let e=*(*u).entries.add(i as usize).add(j as usize);if entry_free(e){(*e).port=(*ti).port;(*e).type_=(*ti).type_;(*e).use_cnt=1;entry_queue(u,e,UDP_TUNNEL_NIC_ENTRY_ADD);return true;}}}false}

unsafe fn udp_tunnel_nic_try_existing(dev:*mut NetDevice,u:*mut UdpTunnelNic,ti:*mut UdpTunnelInfo,adj:i32)->bool{let info=(*dev).udp_tunnel_nic_info;for i in 0..(*u).n_tables{let t=(*info).tables.add(i as usize);if !table_capable(t,ti){continue;}for j in 0..(*t).n_entries{if entry_try_adj(u,i,j,ti,adj){return true;}}}false}
unsafe fn udp_tunnel_nic_add_port(dev:*mut NetDevice,ti:*mut UdpTunnelInfo){let u=(*dev).udp_tunnel_nic;if u.is_null(){return;}if !udp_tunnel_nic_try_existing(dev,u,ti,1){udp_tunnel_nic_add_new(dev,u,ti);}udp_tunnel_nic_device_sync(dev,u)}
unsafe fn udp_tunnel_nic_del_port(dev:*mut NetDevice,ti:*mut UdpTunnelInfo){let u=(*dev).udp_tunnel_nic;if !u.is_null(){udp_tunnel_nic_try_existing(dev,u,ti,-1);udp_tunnel_nic_device_sync(dev,u)}}

#[repr(C)] struct UdpTunnelNicOps { get_port:Option<unsafe extern "C" fn(*mut NetDevice,c_uint,c_uint,*mut UdpTunnelInfo)>, set_port_priv:Option<unsafe extern "C" fn(*mut NetDevice,c_uint,c_uint,u8)>, add_port:Option<unsafe extern "C" fn(*mut NetDevice,*mut UdpTunnelInfo)>, del_port:Option<unsafe extern "C" fn(*mut NetDevice,*mut UdpTunnelInfo)> }
static mut __UDP_TUNNEL_NIC_OPS: UdpTunnelNicOps=UdpTunnelNicOps{get_port:None,set_port_priv:None,add_port:None,del_port:None};

unsafe fn udp_tunnel_nic_init_module()->i32 {udp_tunnel_nic_workqueue=alloc_ordered_workqueue(b"udp_tunnel_nic\0".as_ptr(),0);if udp_tunnel_nic_workqueue.is_null(){return -12;}rtnl_lock();udp_tunnel_nic_ops=&__UDP_TUNNEL_NIC_OPS;rtnl_unlock();0}

// The following kernel-facing entry points retain the C linkage surface; their
// detailed allocator, notifier, netlink, workqueue, and shared-device plumbing
// is provided by the surrounding kernel translation unit.
extern "C" {
    fn __udp_tunnel_nic_get_port(dev:*mut NetDevice, table:c_uint, idx:c_uint, ti:*mut UdpTunnelInfo);
    fn __udp_tunnel_nic_set_port_priv(dev:*mut NetDevice, table:c_uint, idx:c_uint, priv_:u8);
    fn __udp_tunnel_nic_reset_ntf(dev:*mut NetDevice);
    fn __udp_tunnel_nic_dump_size(dev:*mut NetDevice, table:c_uint)->size_t;
    fn __udp_tunnel_nic_dump_write(dev:*mut NetDevice, table:c_uint, skb:*mut SkBuff)->i32;
    fn udp_tunnel_nic_register(dev:*mut NetDevice)->i32;
    fn udp_tunnel_nic_unregister(dev:*mut NetDevice, utn:*mut UdpTunnelNic);
    fn udp_tunnel_nic_replay(dev:*mut NetDevice, utn:*mut UdpTunnelNic);
    fn udp_tunnel_nic_flush(dev:*mut NetDevice, utn:*mut UdpTunnelNic);
}

unsafe fn udp_tunnel_nic_cleanup_module() {
    rtnl_lock();
    udp_tunnel_nic_ops = core::ptr::null();
    rtnl_unlock();
    destroy_workqueue(udp_tunnel_nic_workqueue);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
