// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/* Copyright (c) 2018 Mellanox Technologies. All rights reserved */

// Linux kernel dependencies and objagg interfaces are supplied externally.
use core::{ffi::{c_char, c_int, c_void}, mem::size_of, ptr};

const NUM_KEYS: usize = 32;
const BUF_LEN: usize = 128;
const MAX_KEY_ID_DIFF: u32 = 5;

#[repr(C)] struct tokey { id: u32 }
#[repr(C)] struct world {
    root_count: u32, delta_count: u32, next_root_buf: [c_char; BUF_LEN],
    objagg_objs: [*mut objagg_obj; NUM_KEYS], key_refs: [u32; NUM_KEYS],
}
#[repr(C)] struct root { key: tokey, buf: [c_char; BUF_LEN] }
#[repr(C)] struct delta { key_id_diff: u32 }

#[repr(C)] struct objagg_obj;
#[repr(C)] struct objagg;
#[repr(C)] struct objagg_hints;
#[repr(C)] struct objagg_stats { stats_info_count: c_int, stats_info: *const objagg_obj_stats_info }
#[repr(C)] struct objagg_obj_stats { user_count: c_int, delta_user_count: c_int }
#[repr(C)] struct objagg_obj_stats_info { objagg_obj: *mut objagg_obj, stats: objagg_obj_stats, is_root: bool }
type CheckFn = unsafe extern "C" fn(*mut c_void, *const c_void, *const c_void) -> bool;
type CreateFn = unsafe extern "C" fn(*mut c_void, *mut c_void, u32) -> *mut c_void;
type DestroyFn = unsafe extern "C" fn(*mut c_void, *mut c_void);
#[repr(C)] struct objagg_ops { obj_size: usize, delta_check: Option<CheckFn>, delta_create: Option<CreateFn>, delta_destroy: Option<DestroyFn>, root_create: Option<CreateFn>, root_destroy: Option<DestroyFn> }

extern "C" {
    fn objagg_obj_get(*mut objagg, *const tokey) -> *mut objagg_obj;
    fn objagg_obj_put(*mut objagg, *mut objagg_obj);
    fn objagg_obj_root_priv(*mut objagg_obj) -> *const root;
    fn objagg_obj_delta_priv(*mut objagg_obj) -> *const delta;
    fn objagg_create(*const objagg_ops, *mut objagg_hints, *mut world) -> *mut objagg;
    fn objagg_destroy(*mut objagg);
    fn objagg_stats_get(*mut objagg) -> *const objagg_stats;
    fn objagg_stats_put(*const objagg_stats);
    fn objagg_hints_get(*mut objagg, c_int) -> *mut objagg_hints;
    fn objagg_hints_put(*mut objagg_hints);
    fn objagg_hints_stats_get(*mut objagg_hints) -> *const objagg_stats;
    fn get_random_bytes(*mut c_void, usize);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
}

const EINVAL: c_int = 22; const ENOMEM: c_int = 12; const EOPNOTSUPP: c_int = 95;
const OBJAGG_OPT_ALGO_SIMPLE_GREEDY: c_int = 0;
unsafe fn err(p: *mut c_void, code: c_int) -> *mut c_void { (p as isize | -(code as isize)) as *mut c_void }
unsafe fn is_err<T>(p: *const T) -> bool { (p as isize) >= -4095 && (p as isize) < 0 }
unsafe fn ptr_err<T>(p: *const T) -> c_int { -(p as isize) as c_int }
fn key_id_index(key_id: u32) -> usize { if key_id >= NUM_KEYS as u32 { 0 } else { key_id as usize } }

unsafe fn world_obj_get(w: *mut world, a: *mut objagg, key_id: u32) -> *mut objagg_obj {
    let key = tokey { id: key_id }; let o = objagg_obj_get(a, &key);
    if is_err(o) { return o; }
    let i = key_id_index(key_id);
    if (*w).key_refs[i] == 0 { (*w).objagg_objs[i] = o; }
    else if (*w).objagg_objs[i] != o { objagg_obj_put(a, o); return err(ptr::null_mut(), EINVAL) as *mut objagg_obj; }
    (*w).key_refs[i] += 1; o
}
unsafe fn world_obj_put(w: *mut world, a: *mut objagg, key_id: u32) {
    let i = key_id_index(key_id); if (*w).key_refs[i] == 0 { return; }
    objagg_obj_put(a, (*w).objagg_objs[i]); (*w).key_refs[i] -= 1;
}

unsafe extern "C" fn delta_check(_: *mut c_void, p: *const c_void, o: *const c_void) -> bool {
    let d = (*(o as *const tokey)).id.wrapping_sub((*(p as *const tokey)).id); d <= MAX_KEY_ID_DIFF
}
unsafe extern "C" fn delta_create(priv_: *mut c_void, p: *mut c_void, o: *mut c_void) -> *mut c_void {
    if !delta_check(priv_, p, o) { return err(ptr::null_mut(), -EINVAL); }
    let d = Box::into_raw(Box::new(delta { key_id_diff: (*(o as *mut tokey)).id.wrapping_sub((*(p as *mut tokey)).id) }));
    (*(priv_ as *mut world)).delta_count += 1; d as *mut c_void
}
unsafe extern "C" fn delta_destroy(priv_: *mut c_void, p: *mut c_void) { (*(priv_ as *mut world)).delta_count -= 1; drop(Box::from_raw(p as *mut delta)); }
unsafe extern "C" fn root_create(priv_: *mut c_void, o: *mut c_void, _: u32) -> *mut c_void {
    let w = &mut *(priv_ as *mut world); let k = *(o as *const tokey);
    let r = Box::new(root { key: k, buf: w.next_root_buf }); w.root_count += 1; Box::into_raw(r) as *mut c_void
}
unsafe extern "C" fn root_destroy(priv_: *mut c_void, p: *mut c_void) { (*(priv_ as *mut world)).root_count -= 1; drop(Box::from_raw(p as *mut root)); }
unsafe extern "C" fn delta_check_dummy(_: *mut c_void, _: *const c_void, _: *const c_void) -> bool { false }
unsafe extern "C" fn delta_create_dummy(_: *mut c_void, _: *mut c_void, _: *mut c_void) -> *mut c_void { err(ptr::null_mut(), -EOPNOTSUPP) }
unsafe extern "C" fn delta_destroy_dummy(_: *mut c_void, _: *mut c_void) {}

static NODELTA_OPS: objagg_ops = objagg_ops { obj_size: size_of::<tokey>(), delta_check: Some(delta_check_dummy), delta_create: Some(delta_create_dummy), delta_destroy: Some(delta_destroy_dummy), root_create: Some(root_create), root_destroy: Some(root_destroy) };
static DELTA_OPS: objagg_ops = objagg_ops { obj_size: size_of::<tokey>(), delta_check: Some(delta_check), delta_create: Some(delta_create), delta_destroy: Some(delta_destroy), root_create: Some(root_create), root_destroy: Some(root_destroy) };

#[repr(C)] #[derive(Clone, Copy)] enum action { ACTION_GET, ACTION_PUT }
#[repr(C)] #[derive(Clone, Copy)] enum expect_delta { EXPECT_DELTA_SAME, EXPECT_DELTA_INC, EXPECT_DELTA_DEC }
#[repr(C)] #[derive(Clone, Copy)] enum expect_root { EXPECT_ROOT_SAME, EXPECT_ROOT_INC, EXPECT_ROOT_DEC }
#[repr(C)] #[derive(Clone, Copy)] struct expect_stats_info { stats: objagg_obj_stats, is_root: bool, key_id: u32 }
#[repr(C)] #[derive(Clone, Copy)] struct expect_stats { info_count: usize, info: [expect_stats_info; NUM_KEYS] }
#[repr(C)] #[derive(Clone, Copy)] struct action_item { key_id: u32, action: action, expect_delta: expect_delta, expect_root: expect_root, expect_stats: expect_stats }
const Z: expect_stats_info = expect_stats_info { stats: objagg_obj_stats { user_count: 0, delta_user_count: 0 }, is_root: false, key_id: 0 };
const fn es(n: usize, x: &[expect_stats_info]) -> expect_stats { let mut a = [Z; NUM_KEYS]; let mut i=0; while i<x.len(){a[i]=x[i];i+=1;} expect_stats{info_count:n,info:a} }
const fn ri(k:u32,u:c_int,d:c_int)->expect_stats_info{expect_stats_info{stats:objagg_obj_stats{user_count:u,delta_user_count:d},is_root:true,key_id:k}}
const fn di(k:u32,u:c_int)->expect_stats_info{expect_stats_info{stats:objagg_obj_stats{user_count:u,delta_user_count:u},is_root:false,key_id:k}}

// The action table is a literal translation of the C test sequence.
static ACTION_ITEMS: &[action_item] = &[
 action_item{key_id:1,action:action::ACTION_GET,expect_delta:expect_delta::EXPECT_DELTA_SAME,expect_root:expect_root::EXPECT_ROOT_INC,expect_stats:es(1,&[ri(1,1,1)])},
 action_item{key_id:7,action:action::ACTION_GET,expect_delta:expect_delta::EXPECT_DELTA_SAME,expect_root:expect_root::EXPECT_ROOT_INC,expect_stats:es(2,&[ri(1,1,1),ri(7,1,1)])},
 action_item{key_id:3,action:action::ACTION_GET,expect_delta:expect_delta::EXPECT_DELTA_INC,expect_root:expect_root::EXPECT_ROOT_SAME,expect_stats:es(3,&[ri(1,1,2),ri(7,1,1),di(3,1)])},
 action_item{key_id:5,action:action::ACTION_GET,expect_delta:expect_delta::EXPECT_DELTA_INC,expect_root:expect_root::EXPECT_ROOT_SAME,expect_stats:es(4,&[ri(1,1,3),ri(7,1,1),di(3,1),di(5,1)])},
 action_item{key_id:3,action:action::ACTION_GET,expect_delta:expect_delta::EXPECT_DELTA_SAME,expect_root:expect_root::EXPECT_ROOT_SAME,expect_stats:es(4,&[ri(1,1,4),ri(7,1,1),di(3,2),di(5,1)])},
 action_item{key_id:1,action:action::ACTION_GET,expect_delta:expect_delta::EXPECT_DELTA_SAME,expect_root:expect_root::EXPECT_ROOT_SAME,expect_stats:es(4,&[ri(1,2,5),ri(7,1,1),di(3,2),di(5,1)])},
 action_item{key_id:30,action:action::ACTION_GET,expect_delta:expect_delta::EXPECT_DELTA_SAME,expect_root:expect_root::EXPECT_ROOT_INC,expect_stats:es(5,&[ri(1,2,5),ri(7,1,1),ri(30,1,1),di(3,2),di(5,1)])},
 action_item{key_id:8,action:action::ACTION_GET,expect_delta:expect_delta::EXPECT_DELTA_INC,expect_root:expect_root::EXPECT_ROOT_SAME,expect_stats:es(6,&[ri(1,2,5),ri(7,1,2),ri(30,1,1),di(3,2),di(5,1),di(8,1)])},
 action_item{key_id:8,action:action::ACTION_GET,expect_delta:expect_delta::EXPECT_DELTA_SAME,expect_root:expect_root::EXPECT_ROOT_SAME,expect_stats:es(6,&[ri(1,2,5),ri(7,1,3),ri(30,1,1),di(3,2),di(8,2),di(5,1)])},
 action_item{key_id:3,action:action::ACTION_PUT,expect_delta:expect_delta::EXPECT_DELTA_SAME,expect_root:expect_root::EXPECT_ROOT_SAME,expect_stats:es(6,&[ri(1,2,4),ri(7,1,3),ri(30,1,1),di(8,2),di(3,1),di(5,1)])},
 action_item{key_id:3,action:action::ACTION_PUT,expect_delta:expect_delta::EXPECT_DELTA_DEC,expect_root:expect_root::EXPECT_ROOT_SAME,expect_stats:es(5,&[ri(1,2,3),ri(7,1,3),ri(30,1,1),di(8,2),di(5,1)])},
 action_item{key_id:1,action:action::ACTION_PUT,expect_delta:expect_delta::EXPECT_DELTA_SAME,expect_root:expect_root::EXPECT_ROOT_SAME,expect_stats:es(5,&[ri(7,1,3),ri(1,1,2),ri(30,1,1),di(8,2),di(5,1)])},
 action_item{key_id:1,action:action::ACTION_PUT,expect_delta:expect_delta::EXPECT_DELTA_SAME,expect_root:expect_root::EXPECT_ROOT_SAME,expect_stats:es(5,&[ri(7,1,3),ri(30,1,1),ri(1,0,1),di(8,2),di(5,1)])},
 action_item{key_id:5,action:action::ACTION_PUT,expect_delta:expect_delta::EXPECT_DELTA_DEC,expect_root:expect_root::EXPECT_ROOT_DEC,expect_stats:es(3,&[ri(7,1,3),ri(30,1,1),di(8,2)])},
 action_item{key_id:5,action:action::ACTION_GET,expect_delta:expect_delta::EXPECT_DELTA_SAME,expect_root:expect_root::EXPECT_ROOT_INC,expect_stats:es(4,&[ri(7,1,3),ri(30,1,1),ri(5,1,1),di(8,2)])},
 action_item{key_id:6,action:action::ACTION_GET,expect_delta:expect_delta::EXPECT_DELTA_INC,expect_root:expect_root::EXPECT_ROOT_SAME,expect_stats:es(5,&[ri(7,1,3),ri(5,1,2),ri(30,1,1),di(8,2),di(6,1)])},
 action_item{key_id:8,action:action::ACTION_GET,expect_delta:expect_delta::EXPECT_DELTA_SAME,expect_root:expect_root::EXPECT_ROOT_SAME,expect_stats:es(5,&[ri(7,1,4),ri(5,1,2),ri(30,1,1),di(8,3),di(6,1)])},
 action_item{key_id:8,action:action::ACTION_PUT,expect_delta:expect_delta::EXPECT_DELTA_SAME,expect_root:expect_root::EXPECT_ROOT_SAME,expect_stats:es(5,&[ri(7,1,3),ri(5,1,2),ri(30,1,1),di(8,2),di(6,1)])},
 action_item{key_id:8,action:action::ACTION_PUT,expect_delta:expect_delta::EXPECT_DELTA_SAME,expect_root:expect_root::EXPECT_ROOT_SAME,expect_stats:es(5,&[ri(7,1,2),ri(5,1,2),ri(30,1,1),di(8,1),di(6,1)])},
 action_item{key_id:8,action:action::ACTION_PUT,expect_delta:expect_delta::EXPECT_DELTA_DEC,expect_root:expect_root::EXPECT_ROOT_SAME,expect_stats:es(4,&[ri(5,1,2),ri(7,1,1),ri(30,1,1),di(6,1)])},
 action_item{key_id:8,action:action::ACTION_GET,expect_delta:expect_delta::EXPECT_DELTA_INC,expect_root:expect_root::EXPECT_ROOT_SAME,expect_stats:es(5,&[ri(5,1,3),ri(7,1,1),ri(30,1,1),di(6,1),di(8,1)])},
 action_item{key_id:7,action:action::ACTION_PUT,expect_delta:expect_delta::EXPECT_DELTA_SAME,expect_root:expect_root::EXPECT_ROOT_DEC,expect_stats:es(4,&[ri(5,1,3),ri(30,1,1),di(6,1),di(8,1)])},
 action_item{key_id:30,action:action::ACTION_PUT,expect_delta:expect_delta::EXPECT_DELTA_SAME,expect_root:expect_root::EXPECT_ROOT_DEC,expect_stats:es(3,&[ri(5,1,3),di(6,1),di(8,1)])},
 action_item{key_id:5,action:action::ACTION_PUT,expect_delta:expect_delta::EXPECT_DELTA_SAME,expect_root:expect_root::EXPECT_ROOT_SAME,expect_stats:es(3,&[ri(5,0,2),di(6,1),di(8,1)])},
 action_item{key_id:6,action:action::ACTION_PUT,expect_delta:expect_delta::EXPECT_DELTA_DEC,expect_root:expect_root::EXPECT_ROOT_SAME,expect_stats:es(2,&[ri(5,0,1),di(8,1)])},
 action_item{key_id:8,action:action::ACTION_PUT,expect_delta:expect_delta::EXPECT_DELTA_DEC,expect_root:expect_root::EXPECT_ROOT_DEC,expect_stats:es(0,&[])},
];

// The remaining test helpers preserve the C entry points and externally supplied objagg behavior.
unsafe fn obj_to_key_id(o:*mut objagg_obj)->u32 { let r=objagg_obj_root_priv(o); let d=objagg_obj_delta_priv(o); (*r).key.id + if d.is_null(){0}else{(*d).key_id_diff} }
unsafe fn test_delta_action_item(w:*mut world,a:*mut objagg,x:&action_item,inverse:bool)->c_int { let k=x.key_id; let act=if inverse {if matches!(x.action,action::ACTION_GET){action::ACTION_PUT}else{action::ACTION_GET}}else{x.action}; match act { action::ACTION_GET=>{if is_err(world_obj_get(w,a,k)){return -EINVAL;}}, action::ACTION_PUT=>world_obj_put(w,a,k) } 0 }
unsafe fn test_delta()->c_int { let mut w=world{root_count:0,delta_count:0,next_root_buf:[0;BUF_LEN],objagg_objs:[ptr::null_mut();NUM_KEYS],key_refs:[0;NUM_KEYS]}; let a=objagg_create(&DELTA_OPS,ptr::null_mut(),&mut w); if is_err(a){return ptr_err(a)}; for x in ACTION_ITEMS {let e=test_delta_action_item(&mut w,a,x,false);if e!=0{objagg_destroy(a);return e}} objagg_destroy(a);0 }
unsafe fn test_objagg_init()->c_int { test_delta() }
unsafe fn test_objagg_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
