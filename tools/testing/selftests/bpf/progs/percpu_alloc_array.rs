// Rust translation of testing/selftests/bpf/progs/percpu_alloc_array.c
// Source dependency intent: #include "bpf_experimental.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type u32 = ::core::ffi::c_uint;
type u64 = ::core::ffi::c_ulonglong;

const BPF_MAP_TYPE_ARRAY: u32 = 2;
const BPF_MAP_TYPE_PERCPU_ARRAY: u32 = 6;
const BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE: u32 = 21;
const BPF_ANY: u64 = 0;

#[repr(C)]
pub struct val_t {
    pub b: ::core::ffi::c_long,
    pub c: ::core::ffi::c_long,
    pub d: ::core::ffi::c_long,
}

#[repr(C)]
pub struct elem {
    pub sum: ::core::ffi::c_long,
    pub pc: *mut val_t,
}

// BPF map declaration:
// __uint(type, BPF_MAP_TYPE_ARRAY);
// __uint(max_entries, 1);
// __type(key, int);
// __type(value, struct elem);
#[repr(C)]
pub struct array_map_def {
    pub type_: u32,
    pub max_entries: u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut array: array_map_def = array_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
};

unsafe extern "C" {
    #[link_name = "bpf_rcu_read_lock"]
    fn bpf_rcu_read_lock() /* __ksym */;
    #[link_name = "bpf_rcu_read_unlock"]
    fn bpf_rcu_read_unlock() /* __ksym */;

    fn bpf_map_lookup_elem(map: *mut ::core::ffi::c_void, key: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    fn bpf_map_update_elem(
        map: *mut ::core::ffi::c_void,
        key: *const ::core::ffi::c_void,
        value: *const ::core::ffi::c_void,
        flags: u64,
    ) -> ::core::ffi::c_long;
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_get_local_storage(
        map: *mut ::core::ffi::c_void,
        flags: u64,
    ) -> *mut ::core::ffi::c_void;

    // bpf_experimental.h helpers/macros used as external dependencies.
    fn bpf_percpu_obj_new_val_t() -> *mut val_t;
    fn bpf_percpu_obj_drop(p: *mut val_t);
    fn bpf_kptr_xchg(slot: *mut *mut val_t, p: *mut val_t) -> *mut val_t;
    fn bpf_per_cpu_ptr(p: *mut val_t, cpu: ::core::ffi::c_int) -> *mut val_t;
    fn bpf_this_cpu_ptr(p: *mut val_t) -> *mut val_t;
}

unsafe extern "C" {
    pub static nr_cpus: ::core::ffi::c_int;
}

/* Initialize the percpu object */
#[link_section = "?fentry/bpf_fentry_test1"]
#[no_mangle]
pub unsafe extern "C" fn test_array_map_1() -> ::core::ffi::c_int {
    let mut p: *mut val_t;
    let e: *mut elem;
    let mut index: ::core::ffi::c_int = 0;

    e = bpf_map_lookup_elem(
        &raw mut array as *mut ::core::ffi::c_void,
        &raw const index as *const ::core::ffi::c_void,
    ) as *mut elem;
    if e.is_null() {
        return 0;
    }

    p = bpf_percpu_obj_new_val_t();
    if p.is_null() {
        return 0;
    }

    p = bpf_kptr_xchg(&raw mut (*e).pc, p);
    if !p.is_null() {
        bpf_percpu_obj_drop(p);
    }

    return 0;
}

/* Update percpu data */
#[link_section = "?fentry/bpf_fentry_test2"]
#[no_mangle]
pub unsafe extern "C" fn test_array_map_2() -> ::core::ffi::c_int {
    let mut p: *mut val_t;
    let mut v: *mut val_t;
    let e: *mut elem;
    let mut index: ::core::ffi::c_int = 0;

    e = bpf_map_lookup_elem(
        &raw mut array as *mut ::core::ffi::c_void,
        &raw const index as *const ::core::ffi::c_void,
    ) as *mut elem;
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
pub static mut cpu0_field_d: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sum_field_c: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut my_pid: ::core::ffi::c_int = 0;

/* Summarize percpu data */
#[link_section = "?fentry/bpf_fentry_test3"]
#[no_mangle]
pub unsafe extern "C" fn test_array_map_3() -> ::core::ffi::c_int {
    let mut p: *mut val_t;
    let mut i: ::core::ffi::c_int;
    let mut index: ::core::ffi::c_int = 0;
    let mut v: *mut val_t;
    let e: *mut elem;

    if ((bpf_get_current_pid_tgid() >> 32) as ::core::ffi::c_int) != my_pid {
        return 0;
    }

    e = bpf_map_lookup_elem(
        &raw mut array as *mut ::core::ffi::c_void,
        &raw const index as *const ::core::ffi::c_void,
    ) as *mut elem;
    if e.is_null() {
        return 0;
    }

    p = (*e).pc;
    if p.is_null() {
        return 0;
    }

    i = 0;
    while i < nr_cpus {
        v = bpf_per_cpu_ptr(p, i);
        if !v.is_null() {
            if i == 0 {
                cpu0_field_d = (*v).d as ::core::ffi::c_int;
            }
            sum_field_c = sum_field_c.wrapping_add((*v).c as ::core::ffi::c_int);
        }
        i = i.wrapping_add(1);
    }

    return 0;
}

/* Explicitly free allocated percpu data */
#[link_section = "?fentry/bpf_fentry_test4"]
#[no_mangle]
pub unsafe extern "C" fn test_array_map_4() -> ::core::ffi::c_int {
    let mut p: *mut val_t;
    let e: *mut elem;
    let mut index: ::core::ffi::c_int = 0;

    e = bpf_map_lookup_elem(
        &raw mut array as *mut ::core::ffi::c_void,
        &raw const index as *const ::core::ffi::c_void,
    ) as *mut elem;
    if e.is_null() {
        return 0;
    }

    /* delete */
    p = bpf_kptr_xchg(&raw mut (*e).pc, ::core::ptr::null_mut());
    if !p.is_null() {
        bpf_percpu_obj_drop(p);
    }

    return 0;
}

#[link_section = "?fentry.s/bpf_fentry_test1"]
#[no_mangle]
pub unsafe extern "C" fn test_array_map_10() -> ::core::ffi::c_int {
    let mut p: *mut val_t;
    let mut p1: *mut val_t;
    let mut i: ::core::ffi::c_int;
    let mut index: ::core::ffi::c_int = 0;
    let mut v: *mut val_t;
    let e: *mut elem;

    if ((bpf_get_current_pid_tgid() >> 32) as ::core::ffi::c_int) != my_pid {
        return 0;
    }

    e = bpf_map_lookup_elem(
        &raw mut array as *mut ::core::ffi::c_void,
        &raw const index as *const ::core::ffi::c_void,
    ) as *mut elem;
    if e.is_null() {
        return 0;
    }

    bpf_rcu_read_lock();
    p = (*e).pc;
    if p.is_null() {
        p = bpf_percpu_obj_new_val_t();
        if p.is_null() {
            bpf_rcu_read_unlock();
            return 0;
        }

        p1 = bpf_kptr_xchg(&raw mut (*e).pc, p);
        if !p1.is_null() {
            /* race condition */
            bpf_percpu_obj_drop(p1);
        }
    }

    v = bpf_this_cpu_ptr(p);
    (*v).c = 3;
    v = bpf_this_cpu_ptr(p);
    (*v).c = 0;

    v = bpf_per_cpu_ptr(p, 0);
    if v.is_null() {
        bpf_rcu_read_unlock();
        return 0;
    }
    (*v).c = 1;
    (*v).d = 2;

    /* delete */
    p1 = bpf_kptr_xchg(&raw mut (*e).pc, ::core::ptr::null_mut());
    if p1.is_null() {
        bpf_rcu_read_unlock();
        return 0;
    }

    i = 0;
    while i < nr_cpus {
        v = bpf_per_cpu_ptr(p, i);
        if !v.is_null() {
            if i == 0 {
                cpu0_field_d = (*v).d as ::core::ffi::c_int;
            }
            sum_field_c = sum_field_c.wrapping_add((*v).c as ::core::ffi::c_int);
        }
        i = i.wrapping_add(1);
    }

    /* finally release p */
    bpf_percpu_obj_drop(p1);
    bpf_rcu_read_unlock();
    return 0;
}

// BPF map declaration:
// __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
// __uint(max_entries, 2);
// __type(key, int);
// __type(value, u32);
#[repr(C)]
pub struct percpu_map_def {
    pub type_: u32,
    pub max_entries: u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut percpu: percpu_map_def = percpu_map_def {
    type_: BPF_MAP_TYPE_PERCPU_ARRAY,
    max_entries: 2,
};

#[link_section = "?fentry/bpf_fentry_test1"]
#[no_mangle]
pub unsafe extern "C" fn test_percpu_array(x: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut value: u64 = 0xDEADC0DE;
    let mut key: ::core::ffi::c_int = 0;

    bpf_map_update_elem(
        &raw mut percpu as *mut ::core::ffi::c_void,
        &raw const key as *const ::core::ffi::c_void,
        &raw const value as *const ::core::ffi::c_void,
        BPF_ANY,
    );
    return 0;
}

#[repr(C)]
pub struct bpf_cgroup_storage_key {
    _unused: [u8; 0],
}

// BPF map declaration:
// __uint(type, BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE);
// __type(key, struct bpf_cgroup_storage_key);
// __type(value, u32);
#[repr(C)]
pub struct percpu_cgroup_storage_map_def {
    pub type_: u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut percpu_cgroup_storage: percpu_cgroup_storage_map_def = percpu_cgroup_storage_map_def {
    type_: BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE,
};

#[repr(C)]
pub struct __sk_buff {
    _unused: [u8; 0],
}

#[link_section = "cgroup_skb/egress"]
#[no_mangle]
pub unsafe extern "C" fn cgroup_egress(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let val: *mut u32 = bpf_get_local_storage(
        &raw mut percpu_cgroup_storage as *mut ::core::ffi::c_void,
        0,
    ) as *mut u32;

    *val = 1;
    return 1;
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];
