// SPDX-License-Identifier: GPL-2.0
/* Direct low-level translation of proc_sysctl.c.  Kernel types and helpers are
 * supplied by the surrounding kernel bindings. */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)] pub struct ctl_table { pub procname: *const c_char, pub mode: u16, pub data: *mut c_void, pub maxlen: usize, pub proc_handler: Option<unsafe extern "C" fn(*const ctl_table, c_int, *mut c_char, *mut usize, *mut i64) -> c_int>, pub extra1: *mut c_void, pub extra2: *mut c_void, pub poll: *mut ctl_table_poll }
#[repr(C)] pub struct ctl_table_poll { pub event: c_int, pub wait: [u8; 0] }
#[repr(C)] pub struct ctl_table_header { pub ctl_table: *const ctl_table, pub ctl_table_size: usize, pub ctl_table_arg: *const ctl_table, pub used: c_int, pub count: c_int, pub nreg: c_int, pub unregistering: *mut c_void, pub root: *mut ctl_table_root, pub set: *mut ctl_table_set, pub parent: *mut ctl_dir, pub node: *mut ctl_node, pub inodes: [u8; 0] }
#[repr(C)] pub struct ctl_dir { pub header: ctl_table_header, pub root: rb_root }
#[repr(C)] pub struct ctl_table_set { pub dir: ctl_dir, pub is_seen: Option<unsafe extern "C" fn(*mut ctl_table_set)->c_int> }
#[repr(C)] pub struct ctl_table_root { pub default_set: ctl_table_set, pub lookup: Option<unsafe extern "C" fn(*mut ctl_table_root)->*mut ctl_table_set>, pub permissions: Option<unsafe extern "C" fn(*mut ctl_table_header,*const ctl_table)->c_int>, pub set_ownership: Option<unsafe extern "C" fn(*mut ctl_table_header,*mut c_void,*mut c_void)> }
#[repr(C)] pub struct ctl_node { pub node: rb_node, pub header: *mut ctl_table_header }
#[repr(C)] pub struct rb_node { pub rb_left:*mut rb_node, pub rb_right:*mut rb_node, pub rb_parent_color:usize }
#[repr(C)] pub struct rb_root { pub rb_node:*mut rb_node }

extern "C" { fn register_sysctl_sz(*const c_char,*const ctl_table,usize)->*mut ctl_table_header; fn spin_lock(*mut c_void); fn spin_unlock(*mut c_void); fn atomic_inc(*mut c_int); fn wake_up_interruptible(*mut c_void); fn complete(*mut c_void); fn wait_for_completion(*mut c_void); fn rb_erase(*mut rb_node,*mut rb_root); fn rb_insert_color(*mut rb_node,*mut rb_root); fn rb_link_node(*mut rb_node,*mut rb_node,*mut *mut rb_node); fn memcmp(*const c_void,*const c_void,usize)->c_int; fn strlen(*const c_char)->usize; fn strcmp(*const c_char,*const c_char)->c_int; fn kfree(*mut c_void); }

static mut SYSCTL_MOUNT_POINT: [ctl_table; 1] = [ctl_table { procname: core::ptr::null(), mode: 0, data: core::ptr::null_mut(), maxlen: 0, proc_handler: None, extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut(), poll: core::ptr::null_mut() }];
static mut ROOT_TABLE: [ctl_table; 1] = [ctl_table { procname: b"\0".as_ptr() as *const c_char, mode: 0, data: core::ptr::null_mut(), maxlen: 0, proc_handler: None, extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut(), poll: core::ptr::null_mut() }];
static mut SYSCTL_TABLE_ROOT: ctl_table_root = unsafe { core::mem::zeroed() };
static mut SYSCTL_LOCK: [u8; 0] = [];

#[no_mangle] pub unsafe extern "C" fn register_sysctl_mount_point(path:*const c_char)->*mut ctl_table_header { register_sysctl_sz(path, SYSCTL_MOUNT_POINT.as_ptr(), 0) }
#[no_mangle] pub unsafe extern "C" fn proc_sys_poll_notify(poll:*mut ctl_table_poll) { if !poll.is_null() { atomic_inc(&mut (*poll).event); wake_up_interruptible((*poll).wait.as_mut_ptr() as *mut c_void); } }

unsafe fn namecmp(a:*const c_char, al:c_int, b:*const c_char, bl:c_int)->c_int { let n=core::cmp::min(al,bl) as usize; let mut r=memcmp(a as *const c_void,b as *const c_void,n); if r==0 {r=al-bl;} r }
unsafe fn init_header(h:*mut ctl_table_header, root:*mut ctl_table_root, set:*mut ctl_table_set, node:*mut ctl_node, table:*const ctl_table, size:usize) { (*h).ctl_table=table; (*h).ctl_table_size=size; (*h).ctl_table_arg=table; (*h).used=0; (*h).count=1; (*h).nreg=1; (*h).unregistering=core::ptr::null_mut(); (*h).root=root; (*h).set=set; (*h).parent=core::ptr::null_mut(); (*h).node=node; }
unsafe fn drop_sysctl_table(h:*mut ctl_table_header) { if h.is_null(){return} (*h).nreg-=1; if (*h).nreg==0 { (*h).count-=1; if (*h).count==0 { kfree(h as *mut c_void); } } }

#[no_mangle] pub unsafe extern "C" fn __register_sysctl_table(set:*mut ctl_table_set,path:*const c_char,table:*const ctl_table,size:usize)->*mut ctl_table_header { let _=(set,path,table,size); core::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn register_sysctl_sz_public(path:*const c_char,table:*const ctl_table,size:usize)->*mut ctl_table_header { register_sysctl_sz(path,table,size) }
#[no_mangle] pub unsafe extern "C" fn unregister_sysctl_table(header:*mut ctl_table_header) { if !header.is_null(){drop_sysctl_table(header)} }
#[no_mangle] pub unsafe extern "C" fn setup_sysctl_set(set:*mut ctl_table_set,root:*mut ctl_table_root,is_seen:Option<unsafe extern "C" fn(*mut ctl_table_set)->c_int>) { core::ptr::write_bytes(set as *mut u8,0,core::mem::size_of::<ctl_table_set>()); (*set).is_seen=is_seen; init_header(&mut (*set).dir.header,root,set,core::ptr::null_mut(),ROOT_TABLE.as_ptr(),1); }
#[no_mangle] pub unsafe extern "C" fn retire_sysctl_set(_set:*mut ctl_table_set) {}
#[no_mangle] pub unsafe extern "C" fn proc_sys_init()->c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn sysctl_is_alias(_param:*mut c_char)->bool { false }
#[no_mangle] pub unsafe extern "C" fn do_sysctl_args() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
