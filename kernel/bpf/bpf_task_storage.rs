// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2020 Facebook
 * Copyright 2020 Google LLC.
 */

// C dependencies supplied by the surrounding kernel translation unit.

extern "C" {
    static mut task_cache: bpf_local_storage_cache;
    static btf_local_storage_map_btf_id: [u32; 1];
    static btf_tracing_ids: [u32; 1];

    fn bpf_rcu_lock_held() -> bool;
    fn rcu_read_lock_held() -> bool;
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn bpf_local_storage_lookup(
        storage: *mut bpf_local_storage,
        smap: *mut bpf_local_storage_map,
        cacheit_lockit: bool,
    ) -> *mut bpf_local_storage_data;
    fn bpf_local_storage_destroy(storage: *mut bpf_local_storage);
    fn pidfd_get_pid(fd: i32, f_flags: *mut u32) -> *mut pid;
    fn pid_task(pid: *mut pid, pid_type: i32) -> *mut task_struct;
    fn put_pid(pid: *mut pid);
    fn btf_record_has_field(record: *mut btf_record, field: u32) -> bool;
    fn bpf_local_storage_update(
        owner: *mut task_struct,
        smap: *mut bpf_local_storage_map,
        value: *mut core::ffi::c_void,
        map_flags: u64,
        charge_mem: bool,
    ) -> *mut bpf_local_storage_data;
    fn bpf_selem_unlink(selem: *mut bpf_storage_elem) -> i32;
    fn refcount_read(v: *mut refcount_t) -> i32;
    fn bpf_local_storage_map_alloc(
        attr: *mut bpf_attr,
        cache: *mut bpf_local_storage_cache,
    ) -> *mut bpf_map;
    fn bpf_local_storage_map_free(map: *mut bpf_map, cache: *mut bpf_local_storage_cache);
    fn bpf_map_meta_equal(a: *mut bpf_map, b: *mut bpf_map) -> bool;
    fn bpf_local_storage_map_alloc_check(map: *mut bpf_map) -> i32;
    fn bpf_local_storage_map_check_btf(map: *mut bpf_map, attr: *mut bpf_attr) -> i32;
    fn bpf_local_storage_map_mem_usage(map: *mut bpf_map) -> u64;
}

#[repr(C)] pub struct task_struct { pub bpf_storage: *mut bpf_local_storage, pub usage: refcount_t }
#[repr(C)] pub struct bpf_local_storage { _private: [u8; 0] }
#[repr(C)] pub struct bpf_local_storage_map { _private: [u8; 0] }
#[repr(C)] pub struct bpf_local_storage_data { pub data: *mut core::ffi::c_void }
#[repr(C)] pub struct bpf_storage_elem { _private: [u8; 0] }
#[repr(C)] pub struct bpf_local_storage_cache { _private: [u8; 0] }
#[repr(C)] pub struct bpf_map { pub record: *mut btf_record }
#[repr(C)] pub struct btf_record { _private: [u8; 0] }
#[repr(C)] pub struct bpf_attr { _private: [u8; 0] }
#[repr(C)] pub struct pid { _private: [u8; 0] }
#[repr(C)] pub struct refcount_t { _private: [u8; 0] }

#[repr(C)] pub struct bpf_map_ops {
    pub map_meta_equal: Option<unsafe extern "C" fn(*mut bpf_map, *mut bpf_map) -> bool>,
    pub map_alloc_check: Option<unsafe extern "C" fn(*mut bpf_map) -> i32>,
    pub map_alloc: Option<unsafe extern "C" fn(*mut bpf_attr) -> *mut bpf_map>,
    pub map_free: Option<unsafe extern "C" fn(*mut bpf_map)>,
    pub map_get_next_key: Option<unsafe extern "C" fn(*mut bpf_map, *mut core::ffi::c_void, *mut core::ffi::c_void) -> i32>,
    pub map_lookup_elem: Option<unsafe extern "C" fn(*mut bpf_map, *mut core::ffi::c_void) -> *mut core::ffi::c_void>,
    pub map_update_elem: Option<unsafe extern "C" fn(*mut bpf_map, *mut core::ffi::c_void, *mut core::ffi::c_void, u64) -> i64>,
    pub map_delete_elem: Option<unsafe extern "C" fn(*mut bpf_map, *mut core::ffi::c_void) -> i64>,
    pub map_check_btf: Option<unsafe extern "C" fn(*mut bpf_map, *mut bpf_attr) -> i32>,
    pub map_mem_usage: Option<unsafe extern "C" fn(*mut bpf_map) -> u64>,
    pub map_btf_id: *const u32,
    pub map_owner_storage_ptr: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> *mut *mut bpf_local_storage>,
}

#[repr(C)] pub struct bpf_func_proto {
    pub func: *const core::ffi::c_void,
    pub gpl_only: bool,
    pub ret_type: u32,
    pub arg1_type: u32,
    pub arg2_type: u32,
    pub arg2_btf_id: *const u32,
    pub arg3_type: u32,
    pub arg4_type: u32,
}

unsafe extern "C" fn task_storage_ptr(owner: *mut core::ffi::c_void) -> *mut *mut bpf_local_storage {
    &mut (*(owner as *mut task_struct)).bpf_storage
}

unsafe fn task_storage_lookup(task: *mut task_struct, map: *mut bpf_map, cacheit_lockit: bool) -> *mut bpf_local_storage_data {
    let task_storage = (*task).bpf_storage;
    if task_storage.is_null() { return core::ptr::null_mut(); }
    bpf_local_storage_lookup(task_storage, map as *mut bpf_local_storage_map, cacheit_lockit)
}

#[no_mangle] pub unsafe extern "C" fn bpf_task_storage_free(task: *mut task_struct) {
    rcu_read_lock();
    let local_storage = (*task).bpf_storage;
    if !local_storage.is_null() { bpf_local_storage_destroy(local_storage); }
    rcu_read_unlock();
}

unsafe extern "C" fn bpf_pid_task_storage_lookup_elem(map: *mut bpf_map, key: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    let fd = *(key as *mut i32); let mut f_flags = 0; let pid = pidfd_get_pid(fd, &mut f_flags);
    if pid.is_null() { return pid as *mut core::ffi::c_void; }
    let task = pid_task(pid, 0); if task.is_null() { put_pid(pid); return (-2isize) as *mut _; }
    let sdata = task_storage_lookup(task, map, true); put_pid(pid);
    if sdata.is_null() { core::ptr::null_mut() } else { (*sdata).data }
}

unsafe extern "C" fn bpf_pid_task_storage_update_elem(map: *mut bpf_map, key: *mut core::ffi::c_void, value: *mut core::ffi::c_void, map_flags: u64) -> i64 {
    let fd = *(key as *mut i32); let mut f_flags = 0; let pid = pidfd_get_pid(fd, &mut f_flags);
    if pid.is_null() { return -1; }
    let task = pid_task(pid, 0); if task.is_null() { put_pid(pid); return -2; }
    let sdata = bpf_local_storage_update(task, map as *mut _, value, map_flags, true); put_pid(pid);
    if sdata.is_null() { -1 } else { 0 }
}

unsafe fn task_storage_delete(task: *mut task_struct, map: *mut bpf_map) -> i32 {
    let sdata = task_storage_lookup(task, map, false); if sdata.is_null() { return -2; }
    bpf_selem_unlink(sdata as *mut bpf_storage_elem)
}

unsafe extern "C" fn bpf_pid_task_storage_delete_elem(map: *mut bpf_map, key: *mut core::ffi::c_void) -> i64 {
    let fd = *(key as *mut i32); let mut f_flags = 0; let pid = pidfd_get_pid(fd, &mut f_flags);
    if pid.is_null() { return -1; }
    let task = pid_task(pid, 0); let ret = if task.is_null() { -2 } else { task_storage_delete(task, map) as i64 }; put_pid(pid); ret
}

unsafe extern "C" fn notsupp_get_next_key(_: *mut bpf_map, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32 { -524 }
unsafe extern "C" fn task_storage_map_alloc(attr: *mut bpf_attr) -> *mut bpf_map { bpf_local_storage_map_alloc(attr, &mut task_cache) }
unsafe extern "C" fn task_storage_map_free(map: *mut bpf_map) { bpf_local_storage_map_free(map, &mut task_cache) }

#[no_mangle] pub static task_storage_map_ops: bpf_map_ops = bpf_map_ops {
    map_meta_equal: Some(bpf_map_meta_equal), map_alloc_check: Some(bpf_local_storage_map_alloc_check),
    map_alloc: Some(task_storage_map_alloc), map_free: Some(task_storage_map_free), map_get_next_key: Some(notsupp_get_next_key),
    map_lookup_elem: Some(bpf_pid_task_storage_lookup_elem), map_update_elem: Some(bpf_pid_task_storage_update_elem),
    map_delete_elem: Some(bpf_pid_task_storage_delete_elem), map_check_btf: Some(bpf_local_storage_map_check_btf),
    map_mem_usage: Some(bpf_local_storage_map_mem_usage), map_btf_id: btf_local_storage_map_btf_id.as_ptr(), map_owner_storage_ptr: Some(task_storage_ptr),
};

#[no_mangle] pub static bpf_task_storage_get_proto: bpf_func_proto = bpf_func_proto { func: bpf_task_storage_get as *const _, gpl_only: false, ret_type: 0, arg1_type: 0, arg2_type: 0, arg2_btf_id: core::ptr::null(), arg3_type: 0, arg4_type: 0 };
#[no_mangle] pub static bpf_task_storage_delete_proto: bpf_func_proto = bpf_func_proto { func: bpf_task_storage_delete as *const _, gpl_only: false, ret_type: 0, arg1_type: 0, arg2_type: 0, arg2_btf_id: core::ptr::null(), arg3_type: 0, arg4_type: 0 };

extern "C" { fn bpf_task_storage_get(map: *mut bpf_map, task: *mut task_struct, value: *mut core::ffi::c_void, flags: u64) -> u64; fn bpf_task_storage_delete(map: *mut bpf_map, task: *mut task_struct) -> i64; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
