// Dependency intent from C:
// #include "bpf_experimental.h"
// #include "bpf_misc.h"

#[repr(C)]
pub struct bpf_spin_lock {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct val_t {
    pub b: i64,
    pub c: i64,
    pub d: i64,
}

#[repr(C)]
pub struct val2_t {
    pub b: i64,
}

#[repr(C)]
pub struct val_with_ptr_t {
    pub p: *mut i8,
}

#[repr(C)]
pub struct val_with_rb_root_t {
    pub lock: bpf_spin_lock,
}

#[repr(C)]
pub struct val_600b_t {
    pub b: [i8; 600],
}

#[repr(C)]
pub struct elem {
    pub sum: i64,
    pub pc: *mut val_t,
}

// Original C map definition:
// struct {
//     __uint(type, BPF_MAP_TYPE_ARRAY);
//     __uint(max_entries, 1);
//     __type(key, int);
//     __type(value, struct elem);
// } array SEC(".maps");
#[repr(C)]
pub struct array_map_def {
    _opaque: [u8; 0],
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut array: array_map_def = array_map_def { _opaque: [] };

#[no_mangle]
pub static mut ret: i64 = 0;

extern "C" {
    fn bpf_map_lookup_elem(map: *mut array_map_def, key: *const i32) -> *mut elem;
    fn bpf_kptr_xchg(slot: *mut *mut val_t, ptr: *mut core::ffi::c_void) -> *mut val_t;
    fn bpf_this_cpu_ptr(ptr: *mut val_t) -> *mut val_t;

    #[link_name = "bpf_percpu_obj_new"]
    fn bpf_percpu_obj_new_val_t() -> *mut val_t;
    #[link_name = "bpf_percpu_obj_new"]
    fn bpf_percpu_obj_new_val2_t() -> *mut val2_t;
    #[link_name = "bpf_percpu_obj_new"]
    fn bpf_percpu_obj_new_val_with_ptr_t() -> *mut val_with_ptr_t;
    #[link_name = "bpf_percpu_obj_new"]
    fn bpf_percpu_obj_new_val_with_rb_root_t() -> *mut val_with_rb_root_t;
    #[link_name = "bpf_percpu_obj_new"]
    fn bpf_percpu_obj_new_val_600b_t() -> *mut val_600b_t;

    #[link_name = "bpf_obj_new"]
    fn bpf_obj_new_val_t() -> *mut val_t;

    fn bpf_percpu_obj_drop(ptr: *mut core::ffi::c_void);
    fn bpf_obj_drop(ptr: *mut core::ffi::c_void);
}

// SEC("?fentry/bpf_fentry_test1")
// __failure __msg("store to referenced kptr disallowed")
#[no_mangle]
#[link_section = "?fentry/bpf_fentry_test1"]
pub unsafe extern "C" fn test_array_map_1() -> i32 {
    let mut p: *mut val_t;
    let e: *mut elem;
    let index: i32 = 0;

    e = bpf_map_lookup_elem(&raw mut array, &index);
    if e.is_null() {
        return 0;
    }

    p = bpf_percpu_obj_new_val_t();
    if p.is_null() {
        return 0;
    }

    p = bpf_kptr_xchg(&raw mut (*e).pc, p.cast());
    if !p.is_null() {
        bpf_percpu_obj_drop(p.cast());
    }

    (*e).pc = ret as *mut val_t;
    0
}

// SEC("?fentry/bpf_fentry_test1")
// __failure __msg("invalid kptr access, R2 type=percpu_ptr_val2_t expected=ptr_val_t")
#[no_mangle]
#[link_section = "?fentry/bpf_fentry_test1"]
pub unsafe extern "C" fn test_array_map_2() -> i32 {
    let p2: *mut val2_t;
    let p: *mut val_t;
    let e: *mut elem;
    let index: i32 = 0;

    e = bpf_map_lookup_elem(&raw mut array, &index);
    if e.is_null() {
        return 0;
    }

    p2 = bpf_percpu_obj_new_val2_t();
    if p2.is_null() {
        return 0;
    }

    p = bpf_kptr_xchg(&raw mut (*e).pc, p2.cast());
    if !p.is_null() {
        bpf_percpu_obj_drop(p.cast());
    }

    0
}

// SEC("?fentry.s/bpf_fentry_test1")
// __failure __msg("R1 type=scalar expected=percpu_ptr_, percpu_rcu_ptr_, percpu_trusted_ptr_")
#[no_mangle]
#[link_section = "?fentry.s/bpf_fentry_test1"]
pub unsafe extern "C" fn test_array_map_3() -> i32 {
    let p: *mut val_t;
    let p1: *mut val_t;
    let v: *mut val_t;
    let e: *mut elem;
    let index: i32 = 0;

    e = bpf_map_lookup_elem(&raw mut array, &index);
    if e.is_null() {
        return 0;
    }

    p = bpf_percpu_obj_new_val_t();
    if p.is_null() {
        return 0;
    }

    p1 = bpf_kptr_xchg(&raw mut (*e).pc, p.cast());
    if !p1.is_null() {
        bpf_percpu_obj_drop(p1.cast());
    }

    v = bpf_this_cpu_ptr(p);
    ret = (*v).b;
    0
}

// SEC("?fentry.s/bpf_fentry_test1")
// __failure __msg("R1 expected for bpf_percpu_obj_drop()")
#[no_mangle]
#[link_section = "?fentry.s/bpf_fentry_test1"]
pub unsafe extern "C" fn test_array_map_4() -> i32 {
    let p: *mut val_t;

    p = bpf_percpu_obj_new_val_t();
    if p.is_null() {
        return 0;
    }

    bpf_obj_drop(p.cast());
    0
}

// SEC("?fentry.s/bpf_fentry_test1")
// __failure __msg("R1 expected for bpf_obj_drop()")
#[no_mangle]
#[link_section = "?fentry.s/bpf_fentry_test1"]
pub unsafe extern "C" fn test_array_map_5() -> i32 {
    let p: *mut val_t;

    p = bpf_obj_new_val_t();
    if p.is_null() {
        return 0;
    }

    bpf_percpu_obj_drop(p.cast());
    0
}

// SEC("?fentry.s/bpf_fentry_test1")
// __failure __msg("bpf_percpu_obj_new type ID argument must be of a struct of scalars")
#[no_mangle]
#[link_section = "?fentry.s/bpf_fentry_test1"]
pub unsafe extern "C" fn test_array_map_6() -> i32 {
    let p: *mut val_with_ptr_t;

    p = bpf_percpu_obj_new_val_with_ptr_t();
    if p.is_null() {
        return 0;
    }

    bpf_percpu_obj_drop(p.cast());
    0
}

// SEC("?fentry.s/bpf_fentry_test1")
// __failure __msg("bpf_percpu_obj_new type ID argument must not contain special fields")
#[no_mangle]
#[link_section = "?fentry.s/bpf_fentry_test1"]
pub unsafe extern "C" fn test_array_map_7() -> i32 {
    let p: *mut val_with_rb_root_t;

    p = bpf_percpu_obj_new_val_with_rb_root_t();
    if p.is_null() {
        return 0;
    }

    bpf_percpu_obj_drop(p.cast());
    0
}

// SEC("?fentry.s/bpf_fentry_test1")
// __failure __msg("bpf_percpu_obj_new type size (600) is greater than 512")
#[no_mangle]
#[link_section = "?fentry.s/bpf_fentry_test1"]
pub unsafe extern "C" fn test_array_map_8() -> i32 {
    let p: *mut val_600b_t;

    p = bpf_percpu_obj_new_val_600b_t();
    if p.is_null() {
        return 0;
    }

    bpf_percpu_obj_drop(p.cast());
    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
