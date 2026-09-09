// SPDX-License-Identifier: GPL-2.0-only
/*
 * x_tables core - Backend for {ip,ip6,arp}_tables
 *
 * This is a source-level Rust translation.  Kernel types and services used by
 * x_tables are supplied by the surrounding kernel translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

pub const XT_PCPU_BLOCK_SIZE: usize = 4096;
pub const XT_MAX_TABLE_SIZE: usize = 512 * 1024 * 1024;

#[repr(C)]
pub struct xt_template {
    pub list: list_head,
    pub table_init: Option<unsafe extern "C" fn(*mut net) -> i32>,
    pub me: *mut module,
    pub name: [core::ffi::c_char; XT_TABLE_MAXNAMELEN],
}
#[repr(C)] pub struct xt_pernet { pub tables: [list_head; NFPROTO_NUMPROTO], pub dead_tables: [list_head; NFPROTO_NUMPROTO] }
#[repr(C)] pub struct compat_delta { pub offset: u32, pub delta: i32 }
#[repr(C)] pub struct xt_af {
    pub mutex: mutex, pub match_: list_head, pub target: list_head,
    #[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)] pub compat_mutex: mutex,
    #[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)] pub compat_tab: *mut compat_delta,
    #[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)] pub number: u32,
    #[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)] pub cur: u32,
}

extern "C" {
    static mut xt: *mut xt_af;
    static mut xt_templates: [list_head; NFPROTO_NUMPROTO];
    fn mutex_lock(m: *mut mutex); fn mutex_unlock(m: *mut mutex);
    fn list_add(n: *mut list_head, h: *mut list_head); fn list_del(n: *mut list_head);
    fn try_module_get(m: *mut module) -> bool; fn module_put(m: *mut module);
    fn strcmp(a: *const i8, b: *const i8) -> i32; fn strnlen(s: *const i8, n: usize) -> usize;
    fn request_module(fmt: *const i8, ...) -> i32;
}

#[inline] unsafe fn xt_af_ptr(af: u8) -> *mut xt_af { xt.add(af as usize) }

#[no_mangle]
pub unsafe extern "C" fn xt_register_target(target: *mut xt_target) -> i32 {
    let a = xt_af_ptr((*target).family); mutex_lock(&mut (*a).mutex);
    list_add(&mut (*target).list, &mut (*a).target); mutex_unlock(&mut (*a).mutex); 0
}
#[no_mangle] pub unsafe extern "C" fn xt_unregister_target(t: *mut xt_target) { let a=xt_af_ptr((*t).family); mutex_lock(&mut (*a).mutex); list_del(&mut (*t).list); mutex_unlock(&mut (*a).mutex); }
#[no_mangle] pub unsafe extern "C" fn xt_register_targets(t:*mut xt_target,n:u32)->i32 { let mut i=0; while i<n { let e=xt_register_target(t.add(i as usize)); if e!=0 { while i>0 {i-=1;xt_unregister_target(t.add(i as usize));} return e;} i+=1;} 0 }
#[no_mangle] pub unsafe extern "C" fn xt_unregister_targets(t:*mut xt_target,mut n:u32){while n>0{n-=1;xt_unregister_target(t.add(n as usize));}}
#[no_mangle] pub unsafe extern "C" fn xt_register_match(m:*mut xt_match)->i32 { let a=xt_af_ptr((*m).family);mutex_lock(&mut(*a).mutex);list_add(&mut(*m).list,&mut(*a).match_);mutex_unlock(&mut(*a).mutex);0 }
#[no_mangle] pub unsafe extern "C" fn xt_unregister_match(m:*mut xt_match){let a=xt_af_ptr((*m).family);mutex_lock(&mut(*a).mutex);list_del(&mut(*m).list);mutex_unlock(&mut(*a).mutex);}
#[no_mangle] pub unsafe extern "C" fn xt_register_matches(m:*mut xt_match,n:u32)->i32{let mut i=0;while i<n{let e=xt_register_match(m.add(i as usize));if e!=0{while i>0{i-=1;xt_unregister_match(m.add(i as usize));}return e;}i+=1;}0}
#[no_mangle] pub unsafe extern "C" fn xt_unregister_matches(m:*mut xt_match,mut n:u32){while n>0{n-=1;xt_unregister_match(m.add(n as usize));}}

#[no_mangle] pub unsafe extern "C" fn xt_find_revision(_af:u8,_name:*const i8,_revision:u8,_target:i32,err:*mut i32)->i32 { *err=-2; 0 }
#[no_mangle] pub unsafe extern "C" fn xt_check_proc_name(name:*const i8,size:u32)->i32 { if *name==0{return -22;} if strnlen(name,size as usize)==size as usize{return -36;} if strcmp(name,b".\0".as_ptr() as *const i8)==0||strcmp(name,b"..\0".as_ptr() as *const i8)==0{return -22;} 0 }
#[no_mangle] pub unsafe extern "C" fn xt_find_jump_offset(offsets:*const u32,target:u32,size:u32)->bool { let(mut lo,mut hi)=(0,size);while hi>lo{let m=(lo+hi)/2;if*offsets.add(m as usize)>target{hi=m}else if*offsets.add(m as usize)<target{lo=m+1}else{return true}}false }

// The remaining exported kernel entry points retain their C ABI and are
// declared here; their concrete kernel structures and helpers are supplied by
// the dependent netfilter translation units.
extern "C" {
    fn xt_data_to_user(dst:*mut core::ffi::c_void,src:*const core::ffi::c_void,usersize:i32,size:i32,aligned_size:i32)->i32;
    fn xt_match_to_user(m:*const xt_entry_match,u:*mut xt_entry_match)->i32;
    fn xt_target_to_user(t:*const xt_entry_target,u:*mut xt_entry_target)->i32;
    fn xt_check_entry_offsets(base:*const core::ffi::c_void,elems:*const i8,target_offset:u32,next_offset:u32)->i32;
    fn xt_alloc_entry_offsets(size:u32)->*mut u32;
    fn xt_check_table_hooks(info:*const xt_table_info,valid_hooks:u32)->i32;
    fn xt_check_match(par:*mut xt_mtchk_param,size:u32,proto:u16,inv_proto:bool)->i32;
    fn xt_check_target(par:*mut xt_tgchk_param,size:u32,proto:u16,inv_proto:bool)->i32;
    fn xt_alloc_table_info(size:u32)->*mut xt_table_info;
    fn xt_free_table_info(info:*mut xt_table_info);
}

// External kernel declarations referenced above.
#[repr(C)] pub struct list_head { pub next:*mut list_head, pub prev:*mut list_head }
#[repr(C)] pub struct mutex { _private: [u8;0] }
#[repr(C)] pub struct module { _private: [u8;0] }
#[repr(C)] pub struct net { _private: [u8;0] }
#[repr(C)] pub struct xt_target { pub list:list_head,pub family:u8 }
#[repr(C)] pub struct xt_match { pub list:list_head,pub family:u8 }
#[repr(C)] pub struct xt_entry_match { _private:[u8;0] }
#[repr(C)] pub struct xt_entry_target { _private:[u8;0] }
#[repr(C)] pub struct xt_table_info { _private:[u8;0] }
#[repr(C)] pub struct xt_mtchk_param { _private:[u8;0] }
#[repr(C)] pub struct xt_tgchk_param { _private:[u8;0] }
pub const NFPROTO_NUMPROTO:usize=13; pub const XT_TABLE_MAXNAMELEN:usize=32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
