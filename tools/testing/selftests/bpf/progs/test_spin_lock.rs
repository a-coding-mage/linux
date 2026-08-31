// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook
//
// C dependencies: <linux/bpf.h>, <linux/version.h>, <bpf/bpf_helpers.h>,
// and "bpf_misc.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type c_int = i32;
type c_uint = u32;
type c_ulonglong = u64;

const BPF_MAP_TYPE_HASH: c_uint = 1;
const BPF_MAP_TYPE_ARRAY: c_uint = 2;
const BPF_MAP_TYPE_CGROUP_STORAGE: c_uint = 19;

#[repr(C)]
pub struct bpf_spin_lock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct __sk_buff {
    pub len: c_uint,
    pub pkt_type: c_uint,
    pub mark: c_uint,
    pub queue_mapping: c_uint,
    pub protocol: c_uint,
}

#[repr(C)]
pub struct bpf_cgroup_storage_key {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hmap_elem {
    pub cnt: c_int,
    pub lock: bpf_spin_lock,
    pub test_padding: c_int,
}

#[repr(C)]
pub struct cls_elem {
    pub lock: bpf_spin_lock,
    pub cnt: c_int,
}

#[repr(C)]
pub struct bpf_vqueue {
    pub lock: bpf_spin_lock,
    /* 4 byte hole */
    pub lasttime: c_ulonglong,
    pub credit: c_int,
    pub rate: c_uint,
}

#[repr(C)]
pub struct bpf_map_def_hmap {
    pub type_: c_uint,
    pub max_entries: c_uint,
    pub key_size: c_uint,
    pub value_size: c_uint,
}

#[repr(C)]
pub struct bpf_map_def_cls_map {
    pub type_: c_uint,
    pub key_size: c_uint,
    pub value_size: c_uint,
}

#[repr(C)]
pub struct bpf_map_def_vqueue {
    pub type_: c_uint,
    pub max_entries: c_uint,
    pub key_size: c_uint,
    pub value_size: c_uint,
}

// SEC(".maps")
#[no_mangle]
pub static mut hmap: bpf_map_def_hmap = bpf_map_def_hmap {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 1,
    key_size: core::mem::size_of::<c_int>() as c_uint,
    value_size: core::mem::size_of::<hmap_elem>() as c_uint,
};

// SEC(".maps")
#[no_mangle]
pub static mut cls_map: bpf_map_def_cls_map = bpf_map_def_cls_map {
    type_: BPF_MAP_TYPE_CGROUP_STORAGE,
    key_size: core::mem::size_of::<bpf_cgroup_storage_key>() as c_uint,
    value_size: core::mem::size_of::<cls_elem>() as c_uint,
};

// SEC(".maps")
#[no_mangle]
pub static mut vqueue: bpf_map_def_vqueue = bpf_map_def_vqueue {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<c_int>() as c_uint,
    value_size: core::mem::size_of::<bpf_vqueue>() as c_uint,
};

#[inline]
unsafe fn CREDIT_PER_NS(delta: c_ulonglong, rate: c_uint) -> c_ulonglong {
    delta.wrapping_mul(rate as c_ulonglong) >> 20
}

extern "C" {
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: c_ulonglong,
    ) -> c_int;
    fn bpf_ktime_get_ns() -> c_ulonglong;
    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
    fn bpf_get_local_storage(map: *mut core::ffi::c_void, flags: c_ulonglong) -> *mut core::ffi::c_void;
    fn __sink(value: c_int);
}

// SEC("cgroup_skb/ingress")
#[no_mangle]
pub unsafe extern "C" fn bpf_spin_lock_test(skb: *mut __sk_buff) -> c_int {
    let mut credit: c_int = 0;
    let max_credit: c_int = 100;
    let pkt_len: c_int = 64;
    let mut zero: hmap_elem = core::mem::zeroed();
    let mut val: *mut hmap_elem;
    let mut curtime: c_ulonglong;
    let mut q: *mut bpf_vqueue;
    let mut cls: *mut cls_elem;
    let mut key: c_int = 0;
    let mut err: c_int = 0;

    val = bpf_map_lookup_elem(
        &mut hmap as *mut _ as *mut core::ffi::c_void,
        &key as *const _ as *const core::ffi::c_void,
    ) as *mut hmap_elem;
    if val.is_null() {
        bpf_map_update_elem(
            &mut hmap as *mut _ as *mut core::ffi::c_void,
            &key as *const _ as *const core::ffi::c_void,
            &mut zero as *mut _ as *const core::ffi::c_void,
            0,
        );
        val = bpf_map_lookup_elem(
            &mut hmap as *mut _ as *mut core::ffi::c_void,
            &key as *const _ as *const core::ffi::c_void,
        ) as *mut hmap_elem;
        if val.is_null() {
            err = 1;
            return err;
        }
    }
    /* spin_lock in hash map run time test */
    bpf_spin_lock(&mut (*val).lock);
    if core::ptr::read_volatile(&(*val).cnt) != 0 {
        core::ptr::write_volatile(&mut (*val).cnt, core::ptr::read_volatile(&(*val).cnt).wrapping_sub(1));
    } else {
        core::ptr::write_volatile(&mut (*val).cnt, core::ptr::read_volatile(&(*val).cnt).wrapping_add(1));
    }
    if core::ptr::read_volatile(&(*val).cnt) != 0 && core::ptr::read_volatile(&(*val).cnt) != 1 {
        err = 1;
    }
    bpf_spin_unlock(&mut (*val).lock);

    /* spin_lock in array. virtual queue demo */
    q = bpf_map_lookup_elem(
        &mut vqueue as *mut _ as *mut core::ffi::c_void,
        &key as *const _ as *const core::ffi::c_void,
    ) as *mut bpf_vqueue;
    if q.is_null() {
        return err;
    }
    curtime = bpf_ktime_get_ns();
    bpf_spin_lock(&mut (*q).lock);
    (*q).credit = (*q)
        .credit
        .wrapping_add(CREDIT_PER_NS(curtime.wrapping_sub((*q).lasttime), (*q).rate) as c_int);
    (*q).lasttime = curtime;
    if (*q).credit > max_credit {
        (*q).credit = max_credit;
    }
    (*q).credit = (*q).credit.wrapping_sub(pkt_len);
    credit = (*q).credit;
    bpf_spin_unlock(&mut (*q).lock);

    __sink(credit);

    /* spin_lock in cgroup local storage */
    cls = bpf_get_local_storage(&mut cls_map as *mut _ as *mut core::ffi::c_void, 0) as *mut cls_elem;
    bpf_spin_lock(&mut (*cls).lock);
    core::ptr::write_volatile(&mut (*cls).cnt, core::ptr::read_volatile(&(*cls).cnt).wrapping_add(1));
    bpf_spin_unlock(&mut (*cls).lock);

    err
}

// __hidden SEC(".data.A")
#[no_mangle]
pub static mut lockA: bpf_spin_lock = bpf_spin_lock { _private: [] };

// __noinline
#[inline(never)]
unsafe fn static_subprog(ctx: *mut __sk_buff) -> c_int {
    let ret: c_int = 0;

    if (*ctx).protocol != 0 {
        return ret;
    }
    ret.wrapping_add((*ctx).len as c_int)
}

// __noinline
#[inline(never)]
unsafe fn static_subprog_lock(ctx: *mut __sk_buff) -> c_int {
    let mut ret: c_int = 0;

    ret = static_subprog(ctx);
    bpf_spin_lock(&mut lockA);
    ret.wrapping_add((*ctx).len as c_int)
}

// __noinline
#[inline(never)]
unsafe fn static_subprog_unlock(ctx: *mut __sk_buff) -> c_int {
    let mut ret: c_int = 0;

    ret = static_subprog(ctx);
    bpf_spin_unlock(&mut lockA);
    ret.wrapping_add((*ctx).len as c_int)
}

// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn lock_static_subprog_call(ctx: *mut __sk_buff) -> c_int {
    let mut ret: c_int = 0;

    bpf_spin_lock(&mut lockA);
    if (*ctx).mark == 42 {
        ret = static_subprog(ctx);
    }
    bpf_spin_unlock(&mut lockA);
    ret
}

// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn lock_static_subprog_lock(ctx: *mut __sk_buff) -> c_int {
    let mut ret: c_int = 0;

    ret = static_subprog_lock(ctx);
    bpf_spin_unlock(&mut lockA);
    ret
}

// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn lock_static_subprog_unlock(ctx: *mut __sk_buff) -> c_int {
    let mut ret: c_int = 0;

    bpf_spin_lock(&mut lockA);
    ret = static_subprog_unlock(ctx);
    ret
}

// SEC("license")
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";
