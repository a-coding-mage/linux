// SPDX-License-Identifier: GPL-2.0
// C includes translated as external dependencies:
// <vmlinux.h>, <bpf/bpf_tracing.h>, <bpf/bpf_helpers.h>,
// "../test_kmods/bpf_testmod_kfunc.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

type __u32 = u32;

#[repr(C, packed)]
pub struct map_uninit_value {
    pub unref_ptr: *mut prog_test_ref_kfunc,
    pub data: __u32,
}

#[repr(C)]
pub struct map_value {
    pub unref_ptr: *mut prog_test_ref_kfunc,
    pub ref_ptr: *mut prog_test_ref_kfunc,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub map_flags: u32,
    pub values: *const c_void,
}

#[repr(C)]
pub struct prog_test_ref_kfunc {
    pub a: c_int,
    pub b: c_int,
    pub next: *mut prog_test_ref_kfunc,
    pub cnt: refcounted,
}

#[repr(C)]
pub struct refcounted {
    pub refs: refs,
}

#[repr(C)]
pub struct refs {
    pub counter: c_int,
}

#[repr(C)]
pub struct __sk_buff {
    pub sk: *mut bpf_sock,
}

#[repr(C)]
pub struct bpf_sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cgroup {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

extern "C" {
    fn bpf_map_lookup_elem(map: *const c_void, key: *const c_void) -> *mut c_void;
    fn bpf_map_lookup_percpu_elem(
        map: *const c_void,
        key: *const c_void,
        cpu: u32,
    ) -> *mut c_void;
    fn bpf_map_update_elem(
        map: *const c_void,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;
    fn bpf_cgrp_storage_get(
        map: *const c_void,
        cgrp: *mut cgroup,
        value: *mut c_void,
        flags: u64,
    ) -> *mut c_void;
    fn bpf_task_storage_get(
        map: *const c_void,
        task: *mut task_struct,
        value: *mut c_void,
        flags: u64,
    ) -> *mut c_void;
    fn bpf_task_storage_delete(map: *const c_void, task: *mut task_struct) -> c_int;
    fn bpf_inode_storage_get(
        map: *const c_void,
        inode: *mut inode,
        value: *mut c_void,
        flags: u64,
    ) -> *mut c_void;
    fn bpf_sk_storage_get(
        map: *const c_void,
        sk: *mut bpf_sock,
        value: *mut c_void,
        flags: u64,
    ) -> *mut c_void;
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_kptr_xchg(kptr: *mut *mut prog_test_ref_kfunc, ptr: *mut prog_test_ref_kfunc)
        -> *mut prog_test_ref_kfunc;
    fn bpf_kfunc_call_test_ref(p: *mut prog_test_ref_kfunc);
    fn bpf_kfunc_call_test_release(p: *mut prog_test_ref_kfunc);
    fn bpf_kfunc_call_test_acquire(arg: *mut u64) -> *mut prog_test_ref_kfunc;
}

extern "C" {
    static BPF_MAP_TYPE_PERCPU_ARRAY: u32;
    static BPF_MAP_TYPE_ARRAY: u32;
    static BPF_MAP_TYPE_HASH: u32;
    static BPF_MAP_TYPE_PERCPU_HASH: u32;
    static BPF_MAP_TYPE_LRU_HASH: u32;
    static BPF_MAP_TYPE_LRU_PERCPU_HASH: u32;
    static BPF_MAP_TYPE_CGRP_STORAGE: u32;
    static BPF_MAP_TYPE_TASK_STORAGE: u32;
    static BPF_MAP_TYPE_INODE_STORAGE: u32;
    static BPF_MAP_TYPE_SK_STORAGE: u32;
    static BPF_MAP_TYPE_ARRAY_OF_MAPS: u32;
    static BPF_MAP_TYPE_HASH_OF_MAPS: u32;
    static BPF_F_NO_PREALLOC: u32;
    static BPF_LOCAL_STORAGE_GET_F_CREATE: u64;
    static BPF_EXIST: u64;
    static BPF_NOEXIST: u64;
}

#[link_section = ".maps"]
pub static mut pcpu_array: bpf_map_def = bpf_map_def {
    type_: 0, /* BPF_MAP_TYPE_PERCPU_ARRAY */
    max_entries: 1,
    key_size: core::mem::size_of::<c_int>() as u32,
    value_size: core::mem::size_of::<map_uninit_value>() as u32,
    map_flags: 0,
    values: ptr::null(),
};

#[link_section = ".maps"]
pub static mut array_map: bpf_map_def = bpf_map_def {
    type_: 0, /* BPF_MAP_TYPE_ARRAY */
    max_entries: 1,
    key_size: core::mem::size_of::<c_int>() as u32,
    value_size: core::mem::size_of::<map_value>() as u32,
    map_flags: 0,
    values: ptr::null(),
};

#[link_section = ".maps"]
pub static mut pcpu_array_map: bpf_map_def = bpf_map_def {
    type_: 0, /* BPF_MAP_TYPE_PERCPU_ARRAY */
    max_entries: 1,
    key_size: core::mem::size_of::<c_int>() as u32,
    value_size: core::mem::size_of::<map_value>() as u32,
    map_flags: 0,
    values: ptr::null(),
};

#[link_section = ".maps"]
pub static mut hash_map: bpf_map_def = bpf_map_def {
    type_: 0, /* BPF_MAP_TYPE_HASH */
    max_entries: 1,
    key_size: core::mem::size_of::<c_int>() as u32,
    value_size: core::mem::size_of::<map_value>() as u32,
    map_flags: 0,
    values: ptr::null(),
};

#[link_section = ".maps"]
pub static mut pcpu_hash_map: bpf_map_def = bpf_map_def {
    type_: 0, /* BPF_MAP_TYPE_PERCPU_HASH */
    max_entries: 1,
    key_size: core::mem::size_of::<c_int>() as u32,
    value_size: core::mem::size_of::<map_value>() as u32,
    map_flags: 0,
    values: ptr::null(),
};

#[link_section = ".maps"]
pub static mut hash_malloc_map: bpf_map_def = bpf_map_def {
    type_: 0, /* BPF_MAP_TYPE_HASH */
    max_entries: 1,
    key_size: core::mem::size_of::<c_int>() as u32,
    value_size: core::mem::size_of::<map_value>() as u32,
    map_flags: 0, /* BPF_F_NO_PREALLOC */
    values: ptr::null(),
};

#[link_section = ".maps"]
pub static mut pcpu_hash_malloc_map: bpf_map_def = bpf_map_def {
    type_: 0, /* BPF_MAP_TYPE_PERCPU_HASH */
    max_entries: 1,
    key_size: core::mem::size_of::<c_int>() as u32,
    value_size: core::mem::size_of::<map_value>() as u32,
    map_flags: 0, /* BPF_F_NO_PREALLOC */
    values: ptr::null(),
};

#[link_section = ".maps"]
pub static mut lru_hash_map: bpf_map_def = bpf_map_def {
    type_: 0, /* BPF_MAP_TYPE_LRU_HASH */
    max_entries: 1,
    key_size: core::mem::size_of::<c_int>() as u32,
    value_size: core::mem::size_of::<map_value>() as u32,
    map_flags: 0,
    values: ptr::null(),
};

#[link_section = ".maps"]
pub static mut lru_pcpu_hash_map: bpf_map_def = bpf_map_def {
    type_: 0, /* BPF_MAP_TYPE_LRU_PERCPU_HASH */
    max_entries: 1,
    key_size: core::mem::size_of::<c_int>() as u32,
    value_size: core::mem::size_of::<map_value>() as u32,
    map_flags: 0,
    values: ptr::null(),
};

#[link_section = ".maps"]
pub static mut cgrp_ls_map: bpf_map_def = bpf_map_def {
    type_: 0, /* BPF_MAP_TYPE_CGRP_STORAGE */
    max_entries: 0,
    key_size: core::mem::size_of::<c_int>() as u32,
    value_size: core::mem::size_of::<map_value>() as u32,
    map_flags: 0, /* BPF_F_NO_PREALLOC */
    values: ptr::null(),
};

#[link_section = ".maps"]
pub static mut task_ls_map: bpf_map_def = bpf_map_def {
    type_: 0, /* BPF_MAP_TYPE_TASK_STORAGE */
    max_entries: 0,
    key_size: core::mem::size_of::<c_int>() as u32,
    value_size: core::mem::size_of::<map_value>() as u32,
    map_flags: 0, /* BPF_F_NO_PREALLOC */
    values: ptr::null(),
};

#[link_section = ".maps"]
pub static mut inode_ls_map: bpf_map_def = bpf_map_def {
    type_: 0, /* BPF_MAP_TYPE_INODE_STORAGE */
    max_entries: 0,
    key_size: core::mem::size_of::<c_int>() as u32,
    value_size: core::mem::size_of::<map_value>() as u32,
    map_flags: 0, /* BPF_F_NO_PREALLOC */
    values: ptr::null(),
};

#[link_section = ".maps"]
pub static mut sk_ls_map: bpf_map_def = bpf_map_def {
    type_: 0, /* BPF_MAP_TYPE_SK_STORAGE */
    max_entries: 0,
    key_size: core::mem::size_of::<c_int>() as u32,
    value_size: core::mem::size_of::<map_value>() as u32,
    map_flags: 0, /* BPF_F_NO_PREALLOC */
    values: ptr::null(),
};

macro_rules! define_map_of_map {
    ($name:ident, $map_type:expr, $inner_map:ident) => {
        #[link_section = ".maps"]
        pub static mut $name: bpf_map_def = bpf_map_def {
            type_: $map_type,
            max_entries: 1,
            key_size: core::mem::size_of::<c_int>() as u32,
            value_size: core::mem::size_of::<c_int>() as u32,
            map_flags: 0,
            values: unsafe { &$inner_map as *const _ as *const c_void },
        };
    };
}

define_map_of_map!(array_of_array_maps, 0, array_map);
define_map_of_map!(array_of_hash_maps, 0, hash_map);
define_map_of_map!(array_of_hash_malloc_maps, 0, hash_malloc_map);
define_map_of_map!(array_of_lru_hash_maps, 0, lru_hash_map);
define_map_of_map!(array_of_pcpu_array_maps, 0, pcpu_array_map);
define_map_of_map!(array_of_pcpu_hash_maps, 0, pcpu_hash_map);
define_map_of_map!(hash_of_array_maps, 0, array_map);
define_map_of_map!(hash_of_hash_maps, 0, hash_map);
define_map_of_map!(hash_of_hash_malloc_maps, 0, hash_malloc_map);
define_map_of_map!(hash_of_lru_hash_maps, 0, lru_hash_map);
define_map_of_map!(hash_of_pcpu_array_maps, 0, pcpu_array_map);
define_map_of_map!(hash_of_pcpu_hash_maps, 0, pcpu_hash_map);

unsafe fn write_once<T>(dst: *mut T, val: T) {
    core::ptr::write_volatile(dst, val);
}

unsafe fn test_kptr_unref(v: *mut map_value) {
    let p: *mut prog_test_ref_kfunc;

    p = (*v).unref_ptr;
    /* store untrusted_ptr_or_null_ */
    write_once(&mut (*v).unref_ptr, p);
    if p.is_null() {
        return;
    }
    if (*p).a + (*p).b > 100 {
        return;
    }
    /* store untrusted_ptr_ */
    write_once(&mut (*v).unref_ptr, p);
    /* store NULL */
    write_once(&mut (*v).unref_ptr, ptr::null_mut());
}

unsafe fn test_kptr_ref(v: *mut map_value) {
    let mut p: *mut prog_test_ref_kfunc;

    p = (*v).ref_ptr;
    /* store ptr_or_null_ */
    write_once(&mut (*v).unref_ptr, p);
    if p.is_null() {
        return;
    }
    /*
     * p is rcu_ptr_prog_test_ref_kfunc,
     * because bpf prog is non-sleepable and runs in RCU CS.
     * p can be passed to kfunc that requires KF_RCU.
     */
    bpf_kfunc_call_test_ref(p);
    if (*p).a + (*p).b > 100 {
        return;
    }
    /* store NULL */
    p = bpf_kptr_xchg(&mut (*v).ref_ptr, ptr::null_mut());
    if p.is_null() {
        return;
    }
    /*
     * p is trusted_ptr_prog_test_ref_kfunc.
     * p can be passed to kfunc that requires KF_RCU.
     */
    bpf_kfunc_call_test_ref(p);
    if (*p).a + (*p).b > 100 {
        bpf_kfunc_call_test_release(p);
        return;
    }
    /* store ptr_ */
    write_once(&mut (*v).unref_ptr, p);
    bpf_kfunc_call_test_release(p);

    let mut zero: u64 = 0;
    p = bpf_kfunc_call_test_acquire(&mut zero);
    if p.is_null() {
        return;
    }
    /* store ptr_ */
    p = bpf_kptr_xchg(&mut (*v).ref_ptr, p);
    if p.is_null() {
        return;
    }
    if (*p).a + (*p).b > 100 {
        bpf_kfunc_call_test_release(p);
        return;
    }
    bpf_kfunc_call_test_release(p);
}

unsafe fn test_kptr(v: *mut map_value) {
    test_kptr_unref(v);
    test_kptr_ref(v);
}

#[link_section = "tc"]
pub unsafe extern "C" fn test_map_kptr(ctx: *mut __sk_buff) -> c_int {
    let mut v: *mut map_value;
    let key: c_int = 0;

    macro_rules! test {
        ($map:ident) => {{
            v = bpf_map_lookup_elem(&$map as *const _ as *const c_void, &key as *const _ as *const c_void)
                as *mut map_value;
            if v.is_null() {
                return 0;
            }
            test_kptr(v);
        }};
    }

    test!(array_map);
    test!(hash_map);
    test!(hash_malloc_map);
    test!(lru_hash_map);
    test!(pcpu_array_map);
    test!(pcpu_hash_map);

    let _ = ctx;
    return 0;
}

#[link_section = "tp_btf/cgroup_mkdir"]
pub unsafe extern "C" fn test_cgrp_map_kptr(cgrp: *mut cgroup, path: *const c_char) -> c_int {
    let v: *mut map_value;

    v = bpf_cgrp_storage_get(
        &cgrp_ls_map as *const _ as *const c_void,
        cgrp,
        ptr::null_mut(),
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    ) as *mut map_value;
    if !v.is_null() {
        test_kptr(v);
    }
    let _ = path;
    return 0;
}

#[link_section = "lsm/inode_unlink"]
pub unsafe extern "C" fn test_task_map_kptr(
    inode: *mut inode,
    victim: *mut dentry,
) -> c_int {
    let task: *mut task_struct;
    let v: *mut map_value;

    task = bpf_get_current_task_btf();
    if task.is_null() {
        return 0;
    }
    v = bpf_task_storage_get(
        &task_ls_map as *const _ as *const c_void,
        task,
        ptr::null_mut(),
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    ) as *mut map_value;
    if !v.is_null() {
        test_kptr(v);
    }
    let _ = inode;
    let _ = victim;
    return 0;
}

#[link_section = "lsm/inode_unlink"]
pub unsafe extern "C" fn test_inode_map_kptr(
    inode: *mut inode,
    victim: *mut dentry,
) -> c_int {
    let v: *mut map_value;

    v = bpf_inode_storage_get(
        &inode_ls_map as *const _ as *const c_void,
        inode,
        ptr::null_mut(),
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    ) as *mut map_value;
    if !v.is_null() {
        test_kptr(v);
    }
    let _ = victim;
    return 0;
}

#[link_section = "tc"]
pub unsafe extern "C" fn test_sk_map_kptr(ctx: *mut __sk_buff) -> c_int {
    let v: *mut map_value;
    let sk: *mut bpf_sock;

    sk = (*ctx).sk;
    if sk.is_null() {
        return 0;
    }
    v = bpf_sk_storage_get(
        &sk_ls_map as *const _ as *const c_void,
        sk,
        ptr::null_mut(),
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    ) as *mut map_value;
    if !v.is_null() {
        test_kptr(v);
    }
    return 0;
}

#[link_section = "tc"]
pub unsafe extern "C" fn test_map_in_map_kptr(ctx: *mut __sk_buff) -> c_int {
    let mut v: *mut map_value;
    let key: c_int = 0;
    let mut map: *mut c_void;

    macro_rules! test {
        ($map_in_map:ident) => {{
            map = bpf_map_lookup_elem(
                &$map_in_map as *const _ as *const c_void,
                &key as *const _ as *const c_void,
            );
            if map.is_null() {
                return 0;
            }
            v = bpf_map_lookup_elem(map as *const c_void, &key as *const _ as *const c_void)
                as *mut map_value;
            if v.is_null() {
                return 0;
            }
            test_kptr(v);
        }};
    }

    test!(array_of_array_maps);
    test!(array_of_hash_maps);
    test!(array_of_hash_malloc_maps);
    test!(array_of_lru_hash_maps);
    test!(array_of_pcpu_array_maps);
    test!(array_of_pcpu_hash_maps);
    test!(hash_of_array_maps);
    test!(hash_of_hash_maps);
    test!(hash_of_hash_malloc_maps);
    test!(hash_of_lru_hash_maps);
    test!(hash_of_pcpu_array_maps);
    test!(hash_of_pcpu_hash_maps);

    let _ = ctx;
    return 0;
}

pub static mut ref_: c_int = 1;

unsafe fn test_map_kptr_ref_pre(v: *mut map_value) -> c_int {
    let mut p: *mut prog_test_ref_kfunc;
    let p_st: *mut prog_test_ref_kfunc;
    let mut arg: u64 = 0;
    let ret: c_int;

    p = bpf_kfunc_call_test_acquire(&mut arg);
    if p.is_null() {
        return 1;
    }
    ref_ += 1;

    p_st = (*p).next;
    if (*p_st).cnt.refs.counter != ref_ {
        ret = 2;
        goto_end(ret, p);
        return ret;
    }

    p = bpf_kptr_xchg(&mut (*v).ref_ptr, p);
    if !p.is_null() {
        ret = 3;
        goto_end(ret, p);
        return ret;
    }
    if (*p_st).cnt.refs.counter != ref_ {
        return 4;
    }

    p = bpf_kptr_xchg(&mut (*v).ref_ptr, ptr::null_mut());
    if p.is_null() {
        return 5;
    }
    bpf_kfunc_call_test_release(p);
    ref_ -= 1;
    if (*p_st).cnt.refs.counter != ref_ {
        return 6;
    }

    p = bpf_kfunc_call_test_acquire(&mut arg);
    if p.is_null() {
        return 7;
    }
    ref_ += 1;
    p = bpf_kptr_xchg(&mut (*v).ref_ptr, p);
    if !p.is_null() {
        ret = 8;
        goto_end(ret, p);
        return ret;
    }
    if (*p_st).cnt.refs.counter != ref_ {
        return 9;
    }
    /* Leave in map */

    return 0;
}

unsafe fn goto_end(ret: c_int, p: *mut prog_test_ref_kfunc) {
    ref_ -= 1;
    bpf_kfunc_call_test_release(p);
    let _ = ret;
}

unsafe fn test_map_kptr_ref_post(v: *mut map_value) -> c_int {
    let mut p: *mut prog_test_ref_kfunc;
    let p_st: *mut prog_test_ref_kfunc;

    p_st = (*v).ref_ptr;
    if p_st.is_null() || (*p_st).cnt.refs.counter != ref_ {
        return 1;
    }

    p = bpf_kptr_xchg(&mut (*v).ref_ptr, ptr::null_mut());
    if p.is_null() {
        return 2;
    }
    if (*p_st).cnt.refs.counter != ref_ {
        bpf_kfunc_call_test_release(p);
        return 3;
    }

    p = bpf_kptr_xchg(&mut (*v).ref_ptr, p);
    if !p.is_null() {
        bpf_kfunc_call_test_release(p);
        return 4;
    }
    if (*p_st).cnt.refs.counter != ref_ {
        return 5;
    }

    return 0;
}

#[link_section = "tc"]
pub unsafe extern "C" fn test_map_kptr_ref1(ctx: *mut __sk_buff) -> c_int {
    let mut v: *mut map_value;
    let val: map_value = map_value {
        unref_ptr: ptr::null_mut(),
        ref_ptr: ptr::null_mut(),
    };
    let key: c_int = 0;
    let mut ret: c_int;

    bpf_map_update_elem(
        &hash_map as *const _ as *const c_void,
        &key as *const _ as *const c_void,
        &val as *const _ as *const c_void,
        0,
    );
    bpf_map_update_elem(
        &hash_malloc_map as *const _ as *const c_void,
        &key as *const _ as *const c_void,
        &val as *const _ as *const c_void,
        0,
    );
    bpf_map_update_elem(
        &lru_hash_map as *const _ as *const c_void,
        &key as *const _ as *const c_void,
        &val as *const _ as *const c_void,
        0,
    );

    bpf_map_update_elem(
        &pcpu_hash_map as *const _ as *const c_void,
        &key as *const _ as *const c_void,
        &val as *const _ as *const c_void,
        0,
    );
    bpf_map_update_elem(
        &pcpu_hash_malloc_map as *const _ as *const c_void,
        &key as *const _ as *const c_void,
        &val as *const _ as *const c_void,
        0,
    );
    bpf_map_update_elem(
        &lru_pcpu_hash_map as *const _ as *const c_void,
        &key as *const _ as *const c_void,
        &val as *const _ as *const c_void,
        0,
    );

    macro_rules! test {
        ($map:ident) => {{
            v = bpf_map_lookup_elem(&$map as *const _ as *const c_void, &key as *const _ as *const c_void)
                as *mut map_value;
            if v.is_null() {
                return -1;
            }
            ret = test_map_kptr_ref_pre(v);
            if ret != 0 {
                return ret;
            }
        }};
    }
    macro_rules! test_pcpu {
        ($map:ident) => {{
            v = bpf_map_lookup_percpu_elem(
                &$map as *const _ as *const c_void,
                &key as *const _ as *const c_void,
                0,
            ) as *mut map_value;
            if v.is_null() {
                return -1;
            }
            ret = test_map_kptr_ref_pre(v);
            if ret != 0 {
                return ret;
            }
        }};
    }

    test!(array_map);
    test!(hash_map);
    test!(hash_malloc_map);
    test!(lru_hash_map);

    test_pcpu!(pcpu_array_map);
    test_pcpu!(pcpu_hash_map);
    test_pcpu!(pcpu_hash_malloc_map);
    test_pcpu!(lru_pcpu_hash_map);

    let _ = ctx;
    return 0;
}

#[link_section = "tc"]
pub unsafe extern "C" fn test_map_kptr_ref2(ctx: *mut __sk_buff) -> c_int {
    let mut v: *mut map_value;
    let key: c_int = 0;
    let mut ret: c_int;

    macro_rules! test {
        ($map:ident) => {{
            v = bpf_map_lookup_elem(&$map as *const _ as *const c_void, &key as *const _ as *const c_void)
                as *mut map_value;
            if v.is_null() {
                return -1;
            }
            ret = test_map_kptr_ref_post(v);
            if ret != 0 {
                return ret;
            }
        }};
    }
    macro_rules! test_pcpu {
        ($map:ident) => {{
            v = bpf_map_lookup_percpu_elem(
                &$map as *const _ as *const c_void,
                &key as *const _ as *const c_void,
                0,
            ) as *mut map_value;
            if v.is_null() {
                return -1;
            }
            ret = test_map_kptr_ref_post(v);
            if ret != 0 {
                return ret;
            }
        }};
    }

    test!(array_map);
    test!(hash_map);
    test!(hash_malloc_map);
    test!(lru_hash_map);

    test_pcpu!(pcpu_array_map);
    test_pcpu!(pcpu_hash_map);
    test_pcpu!(pcpu_hash_malloc_map);
    test_pcpu!(lru_pcpu_hash_map);

    let _ = ctx;
    return 0;
}

#[link_section = "tc"]
pub unsafe extern "C" fn test_map_kptr_ref3(ctx: *mut __sk_buff) -> c_int {
    let p: *mut prog_test_ref_kfunc;
    let mut sp: u64 = 0;

    p = bpf_kfunc_call_test_acquire(&mut sp);
    if p.is_null() {
        return 1;
    }
    ref_ += 1;
    if (*p).cnt.refs.counter != ref_ {
        bpf_kfunc_call_test_release(p);
        return 2;
    }
    bpf_kfunc_call_test_release(p);
    ref_ -= 1;
    let _ = ctx;
    return 0;
}

pub static mut num_of_refs: c_int = 0;

unsafe fn read_ref_count() -> c_int {
    let p: *mut prog_test_ref_kfunc;
    let mut arg: u64 = 0;

    p = bpf_kfunc_call_test_acquire(&mut arg);
    if p.is_null() {
        return 1;
    }

    num_of_refs = (*p).cnt.refs.counter;
    bpf_kfunc_call_test_release(p);
    return 0;
}

#[link_section = "syscall"]
pub unsafe extern "C" fn count_ref(ctx: *mut c_void) -> c_int {
    let _ = ctx;
    return read_ref_count();
}

unsafe fn stash_ref_ptr(v: *mut map_value) -> c_int {
    let p: *mut prog_test_ref_kfunc;
    let mut old: *mut prog_test_ref_kfunc;
    let mut arg: u64 = 0;

    p = bpf_kfunc_call_test_acquire(&mut arg);
    if p.is_null() {
        return 1;
    }

    old = bpf_kptr_xchg(&mut (*v).ref_ptr, p);
    if !old.is_null() {
        bpf_kfunc_call_test_release(old);
        old = bpf_kptr_xchg(&mut (*v).ref_ptr, ptr::null_mut());
        if !old.is_null() {
            bpf_kfunc_call_test_release(old);
        }
        return 2;
    }
    return 0;
}

unsafe fn check_refs(expected: c_int) -> c_int {
    let ret: c_int;

    ret = read_ref_count();
    if ret != 0 {
        return ret;
    }
    return if num_of_refs == expected { 0 } else { 3 };
}

#[link_section = "syscall"]
pub unsafe extern "C" fn test_array_map_update_kptr(ctx: *mut c_void) -> c_int {
    let init: map_value = map_value {
        unref_ptr: ptr::null_mut(),
        ref_ptr: ptr::null_mut(),
    };
    let v: *mut map_value;
    let key: c_int = 0;
    let mut ret: c_int;

    v = bpf_map_lookup_elem(
        &array_map as *const _ as *const c_void,
        &key as *const _ as *const c_void,
    ) as *mut map_value;
    if v.is_null() {
        return 1;
    }
    ret = stash_ref_ptr(v);
    if ret != 0 {
        return ret;
    }
    ret = check_refs(3);
    if ret != 0 {
        return ret;
    }
    ret = bpf_map_update_elem(
        &array_map as *const _ as *const c_void,
        &key as *const _ as *const c_void,
        &init as *const _ as *const c_void,
        BPF_EXIST,
    );
    if ret != 0 {
        return 4;
    }
    let _ = ctx;
    return check_refs(3);
}

macro_rules! define_hash_update_kptr_test {
    ($name:ident, $map:ident) => {
        #[link_section = "syscall"]
        pub unsafe extern "C" fn $name(ctx: *mut c_void) -> c_int {
            let init: map_value = map_value {
                unref_ptr: ptr::null_mut(),
                ref_ptr: ptr::null_mut(),
            };
            let v: *mut map_value;
            let key: c_int = 0;
            let mut ret: c_int;

            ret = bpf_map_update_elem(
                &$map as *const _ as *const c_void,
                &key as *const _ as *const c_void,
                &init as *const _ as *const c_void,
                BPF_NOEXIST,
            );
            if ret != 0 {
                return 1;
            }
            v = bpf_map_lookup_elem(
                &$map as *const _ as *const c_void,
                &key as *const _ as *const c_void,
            ) as *mut map_value;
            if v.is_null() {
                return 2;
            }
            ret = stash_ref_ptr(v);
            if ret != 0 {
                return ret;
            }
            ret = check_refs(3);
            if ret != 0 {
                return ret;
            }
            ret = bpf_map_update_elem(
                &$map as *const _ as *const c_void,
                &key as *const _ as *const c_void,
                &init as *const _ as *const c_void,
                BPF_EXIST,
            );
            if ret != 0 {
                return 4;
            }
            let _ = ctx;
            return check_refs(3);
        }
    };
}

define_hash_update_kptr_test!(test_hash_map_update_kptr, hash_map);
define_hash_update_kptr_test!(test_hash_malloc_map_update_kptr, hash_malloc_map);

#[link_section = "syscall"]
pub unsafe extern "C" fn test_ls_map_kptr_ref1(ctx: *mut c_void) -> c_int {
    let current: *mut task_struct;
    let v: *mut map_value;

    current = bpf_get_current_task_btf();
    if current.is_null() {
        return 100;
    }
    v = bpf_task_storage_get(
        &task_ls_map as *const _ as *const c_void,
        current,
        ptr::null_mut(),
        0,
    ) as *mut map_value;
    if !v.is_null() {
        return 150;
    }
    v = bpf_task_storage_get(
        &task_ls_map as *const _ as *const c_void,
        current,
        ptr::null_mut(),
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    ) as *mut map_value;
    if v.is_null() {
        return 200;
    }
    let _ = ctx;
    return test_map_kptr_ref_pre(v);
}

#[link_section = "syscall"]
pub unsafe extern "C" fn test_ls_map_kptr_ref2(ctx: *mut c_void) -> c_int {
    let current: *mut task_struct;
    let v: *mut map_value;

    current = bpf_get_current_task_btf();
    if current.is_null() {
        return 100;
    }
    v = bpf_task_storage_get(
        &task_ls_map as *const _ as *const c_void,
        current,
        ptr::null_mut(),
        0,
    ) as *mut map_value;
    if v.is_null() {
        return 200;
    }
    let _ = ctx;
    return test_map_kptr_ref_post(v);
}

#[link_section = "syscall"]
pub unsafe extern "C" fn test_ls_map_kptr_ref_del(ctx: *mut c_void) -> c_int {
    let current: *mut task_struct;
    let v: *mut map_value;

    current = bpf_get_current_task_btf();
    if current.is_null() {
        return 100;
    }
    v = bpf_task_storage_get(
        &task_ls_map as *const _ as *const c_void,
        current,
        ptr::null_mut(),
        0,
    ) as *mut map_value;
    if v.is_null() {
        return 200;
    }
    if (*v).ref_ptr.is_null() {
        return 300;
    }
    let _ = ctx;
    return bpf_task_storage_delete(&task_ls_map as *const _ as *const c_void, current);
}

#[link_section = "license"]
pub static mut _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
