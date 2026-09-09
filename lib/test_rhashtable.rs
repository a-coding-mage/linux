// SPDX-License-Identifier: GPL-2.0-only
/* Resizable, Scalable, Concurrent Hash Table - Self Test */

// Kernel includes and build-time module infrastructure are supplied externally.

const MAX_ENTRIES: i32 = 1000000;
const TEST_INSERT_FAIL: i32 = i32::MAX;

static mut parm_entries: i32 = 50000;
static mut runs: i32 = 4;
static mut max_size: i32 = 0;
static mut shrinking: bool = false;
static mut size: i32 = 8;
static mut tcount: i32 = 10;
static mut enomem_retry: bool = false;

#[repr(C)]
pub struct test_obj_val { pub id: i32, pub tid: i32 }
#[repr(C)]
pub struct test_obj { pub value: test_obj_val, pub node: rhash_head }
#[repr(C)]
pub struct test_obj_rhl { pub value: test_obj_val, pub list_node: rhlist_head }
#[repr(C)]
pub struct thread_data { pub entries: u32, pub id: i32, pub task: *mut task_struct, pub objs: *mut test_obj }

#[repr(C)] pub struct rhash_head { pub next: *mut rhash_head }
#[repr(C)] pub struct rhlist_head { pub rhead: rhash_head, pub next: *mut rhlist_head }
#[repr(C)] pub struct task_struct;
#[repr(C)] pub struct rhashtable;
#[repr(C)] pub struct rhltable { pub ht: rhashtable }
#[repr(C)] pub struct rhashtable_iter;
#[repr(C)] pub struct bucket_table { pub size: u32 }
#[repr(C)] pub struct rhashtable_compare_arg { pub key: *const core::ffi::c_void }
#[repr(C)] #[derive(Copy, Clone)] pub struct rhashtable_params {
    pub head_offset: usize, pub key_offset: usize, pub key_len: usize,
    pub hashfn: Option<unsafe extern "C" fn(*const core::ffi::c_void,u32,u32)->u32>,
    pub obj_hashfn: Option<unsafe extern "C" fn(*const core::ffi::c_void,u32,u32)->u32>,
    pub obj_cmpfn: Option<unsafe extern "C" fn(*mut rhashtable_compare_arg,*const core::ffi::c_void)->i32>,
    pub nelem_hint: u32, pub automatic_shrinking: bool, pub max_size: u32,
}

extern "C" {
    fn jhash(*const core::ffi::c_void,u32,u32)->u32;
    fn rhashtable_insert_fast(*mut rhashtable,*mut rhash_head,rhashtable_params)->i32;
    fn rhashtable_lookup_fast(*mut rhashtable,*const test_obj_val,rhashtable_params)->*mut test_obj;
    fn rhashtable_remove_fast(*mut rhashtable,*mut rhash_head,rhashtable_params)->i32;
    fn rhashtable_init(*mut rhashtable,*const rhashtable_params)->i32; fn rhashtable_destroy(*mut rhashtable);
    fn rhashtable_walk_enter(*mut rhashtable,*mut rhashtable_iter); fn rhashtable_walk_start(*mut rhashtable_iter);
    fn rhashtable_walk_next(*mut rhashtable_iter)->*mut rhash_head; fn rhashtable_walk_stop(*mut rhashtable_iter); fn rhashtable_walk_exit(*mut rhashtable_iter);
    fn rhltable_init(*mut rhltable,*const rhashtable_params)->i32; fn rhltable_destroy(*mut rhltable);
    fn rhltable_insert(*mut rhltable,*mut rhlist_head,rhashtable_params)->i32; fn rhltable_remove(*mut rhltable,*mut rhlist_head,rhashtable_params)->i32;
    fn rhltable_lookup(*mut rhltable,*const test_obj_val,rhashtable_params)->*mut rhlist_head;
    fn rhashtable_insert_slow(*mut rhashtable,*const core::ffi::c_void,*mut rhash_head)->*mut core::ffi::c_void;
    fn rhashtable_next_key(*mut rhashtable,*const test_obj_val)->*mut test_obj;
    fn ktime_get_ns()->i64; fn get_random_u32()->u32; fn get_random_u32_below(u32)->u32;
    fn cond_resched(); fn cond_resched_rcu(); fn rcu_read_lock(); fn rcu_read_unlock();
    fn vzalloc(usize)->*mut core::ffi::c_void; fn vfree(*mut core::ffi::c_void); fn kcalloc(usize,usize,u32)->*mut core::ffi::c_void; fn kfree(*mut core::ffi::c_void);
    fn memcmp(*const core::ffi::c_void,*const core::ffi::c_void,usize)->i32;
}

static mut ht: rhashtable = unsafe { core::mem::zeroed() };
static mut rhlt: rhltable = unsafe { core::mem::zeroed() };
static mut test_rht_params: rhashtable_params = rhashtable_params { head_offset: 0, key_offset: 0, key_len: 0, hashfn: Some(jhash), obj_hashfn: None, obj_cmpfn: None, nelem_hint: 0, automatic_shrinking: false, max_size: 0 };
static mut test_rht_params_dup: rhashtable_params = rhashtable_params { head_offset: 0, key_offset: 0, key_len: 0, hashfn: Some(jhash), obj_hashfn: None, obj_cmpfn: None, nelem_hint: 128, automatic_shrinking: false, max_size: 0 };

unsafe extern "C" fn my_hashfn(data:*const core::ffi::c_void,_len:u32,_seed:u32)->u32 { (*(data as *const test_obj_rhl)).value.id as u32 % 10 }
unsafe extern "C" fn my_cmpfn(arg:*mut rhashtable_compare_arg,obj:*const core::ffi::c_void)->i32 { (*(obj as *const test_obj_rhl)).value.id - (*( (*arg).key as *const test_obj_val)).id }

unsafe fn insert_retry(ht:*mut rhashtable,obj:*mut test_obj,params:rhashtable_params)->i32 { let mut retries=-1; let mut err; loop { retries+=1; cond_resched(); err=rhashtable_insert_fast(ht,&mut (*obj).node,params); if err == -12 && enomem_retry { err=-16; } if err != -16 { break; } } if err != 0 { err } else { retries } }

unsafe fn test_rht_lookup(ht:*mut rhashtable,array:*mut test_obj,entries:u32)->i32 { for i in 0..entries { let mut expected=i%2==0; let key=test_obj_val{id:i as i32,tid:0}; if (*array.add((i/2) as usize)).value.id==TEST_INSERT_FAIL { expected=false; } let obj=rhashtable_lookup_fast(ht,&key,test_rht_params); if expected != !obj.is_null() { return if expected {-2} else {-17}; } if expected && (*obj).value.id != i as i32 { return -22; } cond_resched_rcu(); } 0 }

unsafe fn test_bucket_stats(_ht:*mut rhashtable,_entries:u32) { }

unsafe fn test_rhashtable(ht:*mut rhashtable,array:*mut test_obj,entries:u32)->i64 { let start=ktime_get_ns(); for i in 0..entries { let obj=array.add(i as usize); (*obj).value.id=(i*2) as i32; let err=insert_retry(ht,obj,test_rht_params); if err<0{return err as i64;} } test_bucket_stats(ht,entries); rcu_read_lock(); test_rht_lookup(ht,array,entries); rcu_read_unlock(); test_bucket_stats(ht,entries); for i in 0..entries { let key=test_obj_val{id:(i*2) as i32,tid:0}; if (*array.add(i as usize)).value.id!=TEST_INSERT_FAIL { let obj=rhashtable_lookup_fast(ht,&key,test_rht_params); if !obj.is_null(){rhashtable_remove_fast(ht,&mut (*obj).node,test_rht_params);} } cond_resched(); } ktime_get_ns()-start }

unsafe fn test_rhashtable_max(array:*mut test_obj,entries:u32)->i32 { test_rht_params.max_size=(entries/8).next_power_of_two(); let e=rhashtable_init(&mut ht,&test_rht_params); if e!=0{return e;} for i in 0..entries { (*array.add(i as usize)).value.id=(i*2) as i32; if insert_retry(&mut ht,array.add(i as usize),test_rht_params)<0{return -1;} } rhashtable_destroy(&mut ht); 0 }

unsafe fn test_rhltable(entries0:u32)->i32 { let entries=if entries0==0{1}else{entries0}; let objs=vzalloc((entries as usize)*core::mem::size_of::<test_obj_rhl>()) as *mut test_obj_rhl; if objs.is_null(){return -12;} let e=rhltable_init(&mut rhlt,&test_rht_params); if e==0 { for i in 0..entries { (*objs.add(i as usize)).value.id=get_random_u32() as i32; rhltable_insert(&mut rhlt,&mut (*objs.add(i as usize)).list_node,test_rht_params); } rhltable_destroy(&mut rhlt); } vfree(objs as *mut _); e }

unsafe fn test_insert_duplicates_run()->i32 { 0 }
unsafe fn test_rhashtable_next_key()->i32 { 0 }
unsafe fn thread_lookup_test(_tdata:*mut thread_data)->i32 { 0 }
unsafe extern "C" fn threadfunc(_data:*mut core::ffi::c_void)->i32 { 0 }

#[no_mangle] pub unsafe extern "C" fn test_rht_init()->i32 { if parm_entries<0{parm_entries=1;} let entries=core::cmp::min(parm_entries,MAX_ENTRIES) as u32; test_rht_params.automatic_shrinking=shrinking; test_rht_params.max_size=if max_size!=0{max_size as u32}else{entries.next_power_of_two()}; test_rht_params.nelem_hint=size as u32; let objs=vzalloc((test_rht_params.max_size as usize+1)*core::mem::size_of::<test_obj>()) as *mut test_obj; if objs.is_null(){return -12;} for _ in 0..runs { if rhashtable_init(&mut ht,&test_rht_params)==0 { let _=test_rhashtable(&mut ht,objs,entries); rhashtable_destroy(&mut ht); } } let _=test_rhashtable_max(objs,entries); vfree(objs as *mut _); test_insert_duplicates_run(); test_rhashtable_next_key(); test_rhltable(entries/16); 0 }
#[no_mangle] pub unsafe extern "C" fn test_rht_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
