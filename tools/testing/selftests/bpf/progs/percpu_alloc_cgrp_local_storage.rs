#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

// Source dependency intent: #include "bpf_experimental.h"

const BPF_MAP_TYPE_CGRP_STORAGE: u32 = 32;
const BPF_F_NO_PREALLOC: u32 = 1;
const BPF_LOCAL_STORAGE_GET_F_CREATE: u64 = 1;

#[repr(C)]
pub struct val_t {
    pub b: i64,
    pub c: i64,
    pub d: i64,
}

#[repr(C)]
pub struct elem {
    pub sum: i64,
    pub pc: *mut val_t,
}

#[repr(C)]
pub struct cgrp_map {
    // C BPF map definition:
    // __uint(type, BPF_MAP_TYPE_CGRP_STORAGE);
    // __uint(map_flags, BPF_F_NO_PREALLOC);
    // __type(key, int);
    // __type(value, struct elem);
    pub _type: u32,
    pub map_flags: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut cgrp: cgrp_map = cgrp_map {
    _type: BPF_MAP_TYPE_CGRP_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<elem>() as u32,
};

#[no_mangle]
pub static mut nr_cpus: i32 = 0;

#[repr(C)]
pub struct cgroup {
    _private: [u8; 0],
}

#[repr(C)]
pub struct css_set {
    pub dfl_cgrp: *mut cgroup,
}

#[repr(C)]
pub struct task_struct {
    pub cgroups: *mut css_set,
}

extern "C" {
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_cgrp_storage_get(
        map: *mut cgrp_map,
        cgroup: *mut cgroup,
        value: *mut core::ffi::c_void,
        flags: u64,
    ) -> *mut elem;
    fn bpf_percpu_obj_new_val_t() -> *mut val_t;
    fn bpf_kptr_xchg(slot: *mut *mut val_t, ptr: *mut val_t) -> *mut val_t;
    fn bpf_percpu_obj_drop(ptr: *mut val_t);
    fn bpf_per_cpu_ptr(ptr: *mut val_t, cpu: i32) -> *mut val_t;
    fn bpf_get_current_pid_tgid() -> u64;
}

/* Initialize the percpu object */
#[no_mangle]
#[link_section = "fentry/bpf_fentry_test1"]
pub unsafe extern "C" fn test_cgrp_local_storage_1() -> i32 {
    let task: *mut task_struct;
    let mut p: *mut val_t;
    let e: *mut elem;

    task = bpf_get_current_task_btf();
    e = bpf_cgrp_storage_get(
        &mut cgrp,
        (*(*task).cgroups).dfl_cgrp,
        core::ptr::null_mut(),
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    );
    if e.is_null() {
        return 0;
    }

    // C source: bpf_percpu_obj_new(struct val_t)
    p = bpf_percpu_obj_new_val_t();
    if p.is_null() {
        return 0;
    }

    p = bpf_kptr_xchg(&mut (*e).pc, p);
    if !p.is_null() {
        bpf_percpu_obj_drop(p);
    }

    return 0;
}

/* Percpu data collection */
#[no_mangle]
#[link_section = "fentry/bpf_fentry_test2"]
pub unsafe extern "C" fn test_cgrp_local_storage_2() -> i32 {
    let task: *mut task_struct;
    let p: *mut val_t;
    let v: *mut val_t;
    let e: *mut elem;

    task = bpf_get_current_task_btf();
    e = bpf_cgrp_storage_get(
        &mut cgrp,
        (*(*task).cgroups).dfl_cgrp,
        core::ptr::null_mut(),
        0,
    );
    if e.is_null() {
        return 0;
    }

    p = (*e).pc;
    if p.is_null() {
        return 0;
    }

    v = bpf_per_cpu_ptr(p, 0);
    if v.is_null() {
        return 0;
    }
    (*v).c = 1;
    (*v).d = 2;
    return 0;
}

#[no_mangle]
pub static mut cpu0_field_d: i32 = 0;
#[no_mangle]
pub static mut sum_field_c: i32 = 0;
#[no_mangle]
pub static mut my_pid: i32 = 0;

/* Summarize percpu data collection */
#[no_mangle]
#[link_section = "fentry/bpf_fentry_test3"]
pub unsafe extern "C" fn test_cgrp_local_storage_3() -> i32 {
    let task: *mut task_struct;
    let p: *mut val_t;
    let mut v: *mut val_t;
    let e: *mut elem;
    let mut i: i32;

    if (bpf_get_current_pid_tgid() >> 32) as i32 != my_pid {
        return 0;
    }

    task = bpf_get_current_task_btf();
    e = bpf_cgrp_storage_get(
        &mut cgrp,
        (*(*task).cgroups).dfl_cgrp,
        core::ptr::null_mut(),
        0,
    );
    if e.is_null() {
        return 0;
    }

    p = (*e).pc;
    if p.is_null() {
        return 0;
    }

    // C source: bpf_for(i, 0, nr_cpus)
    i = 0;
    while i < nr_cpus {
        v = bpf_per_cpu_ptr(p, i);
        if !v.is_null() {
            if i == 0 {
                cpu0_field_d = (*v).d as i32;
            }
            sum_field_c += (*v).c as i32;
        }
        i += 1;
    }

    return 0;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
