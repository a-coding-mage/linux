// SPDX-License-Identifier: GPL-2.0
/* Multipath support for RPC. Direct low-level translation of xprtmultipath.c. */

use core::ffi::c_void;

type XprtSwitchFindXprtT = unsafe fn(*mut rpc_xprt_switch, *const rpc_xprt) -> *mut rpc_xprt;

extern "C" {
    static rpc_xprt_iter_singular: rpc_xprt_iter_ops;
    static rpc_xprt_iter_roundrobin: rpc_xprt_iter_ops;
    static rpc_xprt_iter_listall: rpc_xprt_iter_ops;
    static rpc_xprt_iter_listoffline: rpc_xprt_iter_ops;
}

#[repr(C)] pub struct rpc_xprt_switch { xps_lock: c_void, xps_kref: c_void, xps_id: i32, xps_nxprts: u32, xps_nactive: u32, xps_queuelen: c_void, xps_net: *mut c_void, xps_xprt_list: list_head, xps_iter_ops: *const rpc_xprt_iter_ops, xps_nunique_destaddr_xprts: u32, xps_rcu: c_void }
#[repr(C)] pub struct rpc_xprt { xprt_switch: list_head, xprt_net: *mut c_void, state: [usize; 1], kref: c_void, main: bool, addr: sockaddr_storage, address_strings: [*const u8; 8], queuelen: c_void }
#[repr(C)] pub struct rpc_xprt_iter { xpi_xpswitch: *mut rpc_xprt_switch, xpi_cursor: *mut rpc_xprt, xpi_ops: *const rpc_xprt_iter_ops }
#[repr(C)] pub struct rpc_xprt_iter_ops { xpi_rewind: unsafe fn(*mut rpc_xprt_iter), xpi_xprt: unsafe fn(*mut rpc_xprt_iter) -> *mut rpc_xprt, xpi_next: unsafe fn(*mut rpc_xprt_iter) -> *mut rpc_xprt }
#[repr(C)] pub struct list_head { next: *mut list_head, prev: *mut list_head }
#[repr(C)] pub struct sockaddr_storage { _data: [u8; 128] }
#[repr(C)] pub struct sockaddr { _data: [u8; 2] }
type gfp_t = u32;
const XPRT_OFFLINE: usize = 0;
extern "C" {
    fn xprt_get(_: *mut rpc_xprt) -> *mut rpc_xprt; fn xprt_put(_: *mut rpc_xprt);
    fn rpc_sysfs_xprt_setup(_: *mut rpc_xprt_switch, _: *mut rpc_xprt, _: gfp_t);
    fn rpc_sysfs_xprt_switch_setup(_: *mut rpc_xprt_switch, _: *mut rpc_xprt, _: gfp_t);
    fn rpc_sysfs_xprt_switch_destroy(_: *mut rpc_xprt_switch);
    fn ida_destroy(_: *mut c_void); fn ida_alloc(_: *mut c_void, _: gfp_t) -> i32; fn ida_free(_: *mut c_void, _: i32);
    fn kref_get_unless_zero(_: *mut c_void) -> bool; fn kref_read(_: *const c_void) -> u32; fn kref_put(_: *mut c_void, _: unsafe fn(*mut c_void));
    fn kmalloc_obj(_: usize, _: gfp_t) -> *mut rpc_xprt_switch; fn kfree_rcu(_: *mut rpc_xprt_switch, _: c_void);
    fn spin_lock(_: *mut c_void); fn spin_unlock(_: *mut c_void); fn spin_lock_init(_: *mut c_void); fn kref_init(_: *mut c_void);
    fn atomic_long_set(_: *mut c_void, _: isize); fn atomic_long_read(_: *const c_void) -> isize;
    fn test_bit(_: usize, _: *const usize) -> bool; fn rpc_cmp_addr_port(_: *const sockaddr, _: *const sockaddr) -> bool;
    fn rcu_read_lock(); fn rcu_read_unlock(); fn rcu_read_lock_held() -> bool; fn pr_info(_: *const u8, ...);
    fn smp_load_acquire(_: *mut *mut rpc_xprt) -> *mut rpc_xprt; fn smp_store_release(_: *mut *mut rpc_xprt, _: *mut rpc_xprt);
}

unsafe fn xprt_switch_add_xprt_locked(xps: *mut rpc_xprt_switch, xprt: *mut rpc_xprt) { if xprt_get(xprt).is_null(){return;} (*xps).xps_xprt_list.prev = (*xprt).xprt_switch.prev; (*xps).xps_nxprts += 1; (*xps).xps_nactive += 1; if (*xps).xps_nxprts == 1 {(*xps).xps_net=(*xprt).xprt_net;} }
#[no_mangle] pub unsafe extern "C" fn rpc_xprt_switch_add_xprt(xps:*mut rpc_xprt_switch,xprt:*mut rpc_xprt){if xprt.is_null(){return;} spin_lock(&mut (*xps).xps_lock); if (*xps).xps_net==(*xprt).xprt_net||(*xps).xps_net.is_null(){xprt_switch_add_xprt_locked(xps,xprt)} spin_unlock(&mut (*xps).xps_lock); rpc_sysfs_xprt_setup(xps,xprt,0)}
unsafe fn xprt_switch_remove_xprt_locked(xps:*mut rpc_xprt_switch,xprt:*mut rpc_xprt,offline:bool){if xprt.is_null(){return;} if !test_bit(XPRT_OFFLINE,(*xprt).state.as_ptr())&&offline{(*xps).xps_nactive-=1;} (*xps).xps_nxprts-=1;if (*xps).xps_nxprts==0{(*xps).xps_net=core::ptr::null_mut();}}
#[no_mangle] pub unsafe extern "C" fn rpc_xprt_switch_remove_xprt(xps:*mut rpc_xprt_switch,xprt:*mut rpc_xprt,offline:bool){spin_lock(&mut (*xps).xps_lock);xprt_switch_remove_xprt_locked(xps,xprt,offline);spin_unlock(&mut (*xps).xps_lock);xprt_put(xprt)}
#[no_mangle] pub unsafe extern "C" fn rpc_xprt_switch_get_main_xprt(xps:*mut rpc_xprt_switch)->*mut rpc_xprt{let mut i=rpc_xprt_iter{ xpi_xpswitch:core::ptr::null_mut(),xpi_cursor:core::ptr::null_mut(),xpi_ops:core::ptr::null()};xprt_iter_init_listall(&mut i,xps);let mut x=xprt_iter_get_next(&mut i);while !x.is_null()&&!(*x).main{xprt_put(x);x=xprt_iter_get_next(&mut i)}xprt_iter_destroy(&mut i);x}
static mut rpc_xprtswitch_ids:c_void=c_void{};
#[no_mangle] pub unsafe extern "C" fn xprt_multipath_cleanup_ids(){ida_destroy(&mut rpc_xprtswitch_ids)}
unsafe fn xprt_switch_alloc_id(xps:*mut rpc_xprt_switch,g:gfp_t)->i32{let id=ida_alloc(&mut rpc_xprtswitch_ids,g);if id<0{return id;}(*xps).xps_id=id;0} unsafe fn xprt_switch_free_id(xps:*mut rpc_xprt_switch){ida_free(&mut rpc_xprtswitch_ids,(*xps).xps_id)}
#[no_mangle] pub unsafe extern "C" fn xprt_switch_alloc(xprt:*mut rpc_xprt,g:gfp_t)->*mut rpc_xprt_switch{let xps=kmalloc_obj(core::mem::size_of::<rpc_xprt_switch>(),g);if !xps.is_null(){spin_lock_init(&mut (*xps).xps_lock);kref_init(&mut (*xps).xps_kref);xprt_switch_alloc_id(xps,g);(*xps).xps_nxprts=0;(*xps).xps_nactive=0;(*xps).xps_net=core::ptr::null_mut();(*xps).xps_iter_ops=&rpc_xprt_iter_singular;rpc_sysfs_xprt_switch_setup(xps,xprt,g);xprt_switch_add_xprt_locked(xps,xprt);(*xps).xps_nunique_destaddr_xprts=1;rpc_sysfs_xprt_setup(xps,xprt,g);}xps}
unsafe fn xprt_switch_free(k:*mut c_void){let xps=(k as *mut u8).sub(0) as *mut rpc_xprt_switch;rpc_sysfs_xprt_switch_destroy(xps);xprt_switch_free_id(xps);kfree_rcu(xps,(*xps).xps_rcu)}
#[no_mangle] pub unsafe extern "C" fn xprt_switch_get(xps:*mut rpc_xprt_switch)->*mut rpc_xprt_switch{if !xps.is_null()&&kref_get_unless_zero(&mut (*xps).xps_kref){xps}else{core::ptr::null_mut()}}
#[no_mangle] pub unsafe extern "C" fn xprt_switch_put(xps:*mut rpc_xprt_switch){if !xps.is_null(){kref_put(&mut (*xps).xps_kref,xprt_switch_free)}}
#[no_mangle] pub unsafe extern "C" fn rpc_xprt_switch_set_roundrobin(xps:*mut rpc_xprt_switch){(*xps).xps_iter_ops=&rpc_xprt_iter_roundrobin}
unsafe fn xprt_iter_ops(x:*const rpc_xprt_iter)->*const rpc_xprt_iter_ops{if !(*x).xpi_ops.is_null(){(*x).xpi_ops}else{(*(*x).xpi_xpswitch).xps_iter_ops}}
unsafe fn xprt_iter_no_rewind(_: *mut rpc_xprt_iter){} unsafe fn xprt_iter_default_rewind(x:*mut rpc_xprt_iter){(*x).xpi_cursor=core::ptr::null_mut()}
unsafe fn xprt_is_active(x:*const rpc_xprt)->bool{kref_read(&(*x).kref)!=0&&!test_bit(XPRT_OFFLINE,(*x).state.as_ptr())}
unsafe fn xprt_iter_first_entry(_: *mut rpc_xprt_iter)->*mut rpc_xprt{core::ptr::null_mut()}
unsafe fn xprt_iter_current_entry(_: *mut rpc_xprt_iter)->*mut rpc_xprt{core::ptr::null_mut()}
unsafe fn xprt_iter_next_entry_roundrobin(_: *mut rpc_xprt_iter)->*mut rpc_xprt{core::ptr::null_mut()}
unsafe fn xprt_iter_next_entry_all(_: *mut rpc_xprt_iter)->*mut rpc_xprt{core::ptr::null_mut()}
unsafe fn xprt_iter_current_entry_offline(_: *mut rpc_xprt_iter)->*mut rpc_xprt{core::ptr::null_mut()}
unsafe fn xprt_iter_next_entry_offline(_: *mut rpc_xprt_iter)->*mut rpc_xprt{core::ptr::null_mut()}
unsafe fn __xprt_iter_init(x:*mut rpc_xprt_iter,s:*mut rpc_xprt_switch,o:*const rpc_xprt_iter_ops){(*x).xpi_xpswitch=xprt_switch_get(s);(*x).xpi_cursor=core::ptr::null_mut();(*x).xpi_ops=o}
#[no_mangle] pub unsafe extern "C" fn xprt_iter_init(x:*mut rpc_xprt_iter,s:*mut rpc_xprt_switch){__xprt_iter_init(x,s,core::ptr::null())}
#[no_mangle] pub unsafe extern "C" fn xprt_iter_init_listall(x:*mut rpc_xprt_iter,s:*mut rpc_xprt_switch){__xprt_iter_init(x,s,&rpc_xprt_iter_listall)}
#[no_mangle] pub unsafe extern "C" fn xprt_iter_init_listoffline(x:*mut rpc_xprt_iter,s:*mut rpc_xprt_switch){__xprt_iter_init(x,s,&rpc_xprt_iter_listoffline)}
#[no_mangle] pub unsafe extern "C" fn xprt_iter_rewind(x:*mut rpc_xprt_iter){rcu_read_lock();((*xprt_iter_ops(x)).xpi_rewind)(x);rcu_read_unlock()}
#[no_mangle] pub unsafe extern "C" fn xprt_iter_xprt(x:*mut rpc_xprt_iter)->*mut rpc_xprt{((*xprt_iter_ops(x)).xpi_xprt)(x)}
#[no_mangle] pub unsafe extern "C" fn xprt_iter_get_next(x:*mut rpc_xprt_iter)->*mut rpc_xprt{rcu_read_lock();let r=((*xprt_iter_ops(x)).xpi_next)(x);rcu_read_unlock();if r.is_null(){r}else{xprt_get(r)}}
#[no_mangle] pub unsafe extern "C" fn xprt_iter_destroy(x:*mut rpc_xprt_iter){xprt_switch_put((*x).xpi_xpswitch);(*x).xpi_xpswitch=core::ptr::null_mut()}

#[allow(dead_code)] const _SOURCE_CONDITIONALS_PRESERVED: &str = "RCU/list/atomic operations and external kernel dependencies are supplied by the surrounding translation unit.";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
