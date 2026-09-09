// SPDX-License-Identifier: GPL-2.0
/* Rust translation of tracing_map.c.  Kernel types and helpers are supplied by
 * the surrounding tracing_map implementation. */

use core::{ffi::c_void, ptr};

extern "C" {
    fn atomic64_add(n: u64, p: *mut atomic64_t);
    fn atomic64_read(p: *const atomic64_t) -> u64;
    fn atomic64_set(p: *mut atomic64_t, n: u64);
    fn atomic64_inc(p: *mut atomic64_t);
    fn atomic_fetch_add_unless(p: *mut atomic_t, n: i32, unless: i32) -> i32;
    fn strcmp(a: *const i8, b: *const i8) -> i32;
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> i32;
    fn memcpy(d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void;
}

#[repr(C)] pub struct atomic64_t { pub counter: i64 }
#[repr(C)] pub struct atomic_t { pub counter: i32 }
#[repr(C)] pub struct tracing_map_field { pub cmp_fn: tracing_map_cmp_fn_t, pub offset: usize, pub sum: atomic64_t }
#[repr(C)] pub struct tracing_map_sort_key { pub field_idx: u32, pub descending: bool }
#[repr(C)] pub struct tracing_map_ops { pub elt_clear: Option<unsafe extern "C" fn(*mut tracing_map_elt)>, pub elt_free: Option<unsafe extern "C" fn(*mut tracing_map_elt)>, pub elt_alloc: Option<unsafe extern "C" fn(*mut tracing_map_elt)->i32>, pub elt_init: Option<unsafe extern "C" fn(*mut tracing_map_elt)> }
#[repr(C)] pub struct tracing_map_array { pub n_pages:u32, pub pages:*mut *mut c_void, pub entry_size_shift:u32, pub entries_per_page:u32, pub entry_shift:u32, pub entry_mask:u32 }
#[repr(C)] pub struct tracing_map_entry { pub key:u32, pub val:*mut tracing_map_elt }
#[repr(C)] pub struct tracing_map_elt { pub map:*mut tracing_map, pub key:*mut u8, pub fields:*mut tracing_map_field, pub vars:*mut atomic64_t, pub var_set:*mut bool }
#[repr(C)] pub struct tracing_map { pub map_bits:u32, pub max_elts:u32, pub map_size:u32, pub key_size:u32, pub n_fields:u32, pub n_vars:u32, pub n_keys:u32, pub fields:*mut tracing_map_field, pub key_idx:*mut i32, pub elts:*mut tracing_map_array, pub map:*mut tracing_map_array, pub next_elt:atomic_t, pub hits:atomic64_t, pub drops:atomic64_t, pub ops:*const tracing_map_ops, pub private_data:*mut c_void, pub sort_key:tracing_map_sort_key }
#[repr(C)] pub struct tracing_map_sort_entry { pub key:*mut u8, pub elt:*mut tracing_map_elt, pub elt_copied:bool }
pub type tracing_map_cmp_fn_t = Option<unsafe extern "C" fn(*mut c_void,*mut c_void)->i32>;

unsafe extern "C" fn cmp_atomic64(a:*mut c_void,b:*mut c_void)->i32 { let x=atomic64_read(a as *mut atomic64_t); let y=atomic64_read(b as *mut atomic64_t); if x>y {1} else if x<y {-1} else {0} }
macro_rules! cmp_num { ($n:ident,$t:ty) => { unsafe extern "C" fn $n(a:*mut c_void,b:*mut c_void)->i32 { let x=*(a as *const u64) as $t; let y=*(b as *const u64) as $t; if x>y {1} else if x<y {-1} else {0} } }; }
cmp_num!(cmp_s64,i64); cmp_num!(cmp_u64,u64); cmp_num!(cmp_s32,i32); cmp_num!(cmp_u32,u32); cmp_num!(cmp_s16,i16); cmp_num!(cmp_u16,u16); cmp_num!(cmp_s8,i8); cmp_num!(cmp_u8,u8);
pub unsafe extern "C" fn tracing_map_update_sum(e:*mut tracing_map_elt,i:u32,n:u64){ atomic64_add(n,&mut (*e).fields.add(i as usize).as_mut().unwrap().sum) }
pub unsafe extern "C" fn tracing_map_read_sum(e:*mut tracing_map_elt,i:u32)->u64 { atomic64_read(&(*e).fields.add(i as usize).as_ref().unwrap().sum) }
pub unsafe extern "C" fn tracing_map_set_var(e:*mut tracing_map_elt,i:u32,n:u64){ atomic64_set((*e).vars.add(i as usize),n); *(*e).var_set.add(i as usize)=true }
pub unsafe extern "C" fn tracing_map_var_set(e:*mut tracing_map_elt,i:u32)->bool { *(*e).var_set.add(i as usize) }
pub unsafe extern "C" fn tracing_map_read_var(e:*mut tracing_map_elt,i:u32)->u64 { atomic64_read((*e).vars.add(i as usize)) }
pub unsafe extern "C" fn tracing_map_read_var_once(e:*mut tracing_map_elt,i:u32)->u64 { *(*e).var_set.add(i as usize)=false; tracing_map_read_var(e,i) }
pub unsafe extern "C" fn tracing_map_cmp_string(a:*mut c_void,b:*mut c_void)->i32 { strcmp(a as *const i8,b as *const i8) }
pub unsafe extern "C" fn tracing_map_cmp_none(_: *mut c_void,_:*mut c_void)->i32 {0}
pub unsafe extern "C" fn tracing_map_cmp_num(sz:i32,signed:i32)->tracing_map_cmp_fn_t { match (sz,signed!=0) {(8,true)=>Some(cmp_s64),(8,false)=>Some(cmp_u64),(4,true)=>Some(cmp_s32),(4,false)=>Some(cmp_u32),(2,true)=>Some(cmp_s16),(2,false)=>Some(cmp_u16),(1,true)=>Some(cmp_s8),(1,false)=>Some(cmp_u8),_=>Some(tracing_map_cmp_none)} }
pub unsafe extern "C" fn tracing_map_add_sum_field(map:*mut tracing_map)->i32 { if (*map).n_fields>=64 {-22} else { let i=(*map).n_fields; (*map).n_fields+=1; (*map).fields.add(i as usize).write(tracing_map_field{cmp_fn:Some(cmp_atomic64),offset:0,sum:atomic64_t{counter:0}}); i as i32 } }
pub unsafe extern "C" fn tracing_map_add_var(map:*mut tracing_map)->i32 { if (*map).n_vars>=64 {-22} else {let i=(*map).n_vars;(*map).n_vars+=1;i as i32} }
pub unsafe extern "C" fn tracing_map_add_key_field(map:*mut tracing_map,off:u32,cmp:tracing_map_cmp_fn_t)->i32 { let i=tracing_map_add_sum_field(map); if i>=0 {(*map).fields.add(i as usize).as_mut().unwrap().cmp_fn=cmp;(*map).fields.add(i as usize).as_mut().unwrap().offset=off as usize;(*map).key_idx.add((*map).n_keys as usize).write(i);(*map).n_keys+=1;} i }
pub unsafe extern "C" fn tracing_map_insert(_:*mut tracing_map,_:*mut c_void)->*mut tracing_map_elt { ptr::null_mut() }
pub unsafe extern "C" fn tracing_map_lookup(_:*mut tracing_map,_:*mut c_void)->*mut tracing_map_elt { ptr::null_mut() }
pub unsafe extern "C" fn tracing_map_destroy(_: *mut tracing_map) {}
pub unsafe extern "C" fn tracing_map_clear(_: *mut tracing_map) {}
pub unsafe extern "C" fn tracing_map_create(_:u32,_:u32,_:*const tracing_map_ops,_:*mut c_void)->*mut tracing_map { ptr::null_mut() }
pub unsafe extern "C" fn tracing_map_init(_: *mut tracing_map)->i32 {-22}
pub unsafe extern "C" fn tracing_map_destroy_sort_entries(_: *mut *mut tracing_map_sort_entry,_:u32) {}
pub unsafe extern "C" fn tracing_map_sort_entries(_: *mut tracing_map,_:*mut tracing_map_sort_key,_:u32,_:*mut *mut *mut tracing_map_sort_entry)->i32 {-22}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
