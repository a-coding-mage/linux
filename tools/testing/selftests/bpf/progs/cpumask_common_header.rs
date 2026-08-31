/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

// C dependencies: "errno.h", <stdbool.h>

/* Should use BTF_FIELDS_MAX, but it is not always available in vmlinux.h,
 * so use the hard-coded number as a workaround.
 */
pub const CPUMASK_KPTR_FIELDS_MAX: i32 = 11;

pub static mut err: core::ffi::c_int = 0;

// private(name) in C expands to SEC(".bss." #name) __attribute__((aligned(8))).
// private(MASK) static struct bpf_cpumask __kptr * global_mask;
#[repr(align(8))]
pub struct GlobalMaskStorage {
    pub ptr: *mut bpf_cpumask,
}

pub static mut global_mask: GlobalMaskStorage = GlobalMaskStorage {
    ptr: core::ptr::null_mut(),
};

#[repr(C)]
pub struct __cpumask_map_value {
    pub cpumask: *mut bpf_cpumask,
}

#[repr(C)]
pub struct array_map {
    // C BPF map declaration metadata:
    // __uint(type, BPF_MAP_TYPE_ARRAY);
    // __type(key, int);
    // __type(value, struct __cpumask_map_value);
    // __uint(max_entries, 1);
    pub _private: [u8; 0],
}

// struct array_map __cpumask_map SEC(".maps");
unsafe extern "C" {
    pub static mut __cpumask_map: array_map;
}

#[repr(C)]
pub struct bpf_cpumask {
    pub _private: [u8; 0],
}

#[repr(C)]
pub struct cpumask {
    pub _private: [u8; 0],
}

pub type u32 = core::ffi::c_uint;
pub type size_t = usize;

unsafe extern "C" {
    pub fn bpf_cpumask_create() -> *mut bpf_cpumask;
    pub fn bpf_cpumask_release(cpumask: *mut bpf_cpumask);
    pub fn bpf_cpumask_acquire(cpumask: *mut bpf_cpumask) -> *mut bpf_cpumask;
    pub fn bpf_cpumask_first(cpumask: *const cpumask) -> u32;
    pub fn bpf_cpumask_first_zero(cpumask: *const cpumask) -> u32;
    pub fn bpf_cpumask_first_and(src1: *const cpumask, src2: *const cpumask) -> u32;
    pub fn bpf_cpumask_set_cpu(cpu: u32, cpumask: *mut bpf_cpumask);
    pub fn bpf_cpumask_clear_cpu(cpu: u32, cpumask: *mut bpf_cpumask);
    pub fn bpf_cpumask_test_cpu(cpu: u32, cpumask: *const cpumask) -> bool;
    pub fn bpf_cpumask_test_and_set_cpu(cpu: u32, cpumask: *mut bpf_cpumask) -> bool;
    pub fn bpf_cpumask_test_and_clear_cpu(cpu: u32, cpumask: *mut bpf_cpumask) -> bool;
    pub fn bpf_cpumask_setall(cpumask: *mut bpf_cpumask);
    pub fn bpf_cpumask_clear(cpumask: *mut bpf_cpumask);
    pub fn bpf_cpumask_and(
        cpumask: *mut bpf_cpumask,
        src1: *const cpumask,
        src2: *const cpumask,
    ) -> bool;
    pub fn bpf_cpumask_or(
        cpumask: *mut bpf_cpumask,
        src1: *const cpumask,
        src2: *const cpumask,
    );
    pub fn bpf_cpumask_xor(
        cpumask: *mut bpf_cpumask,
        src1: *const cpumask,
        src2: *const cpumask,
    );
    pub fn bpf_cpumask_equal(src1: *const cpumask, src2: *const cpumask) -> bool;
    pub fn bpf_cpumask_intersects(src1: *const cpumask, src2: *const cpumask) -> bool;
    pub fn bpf_cpumask_subset(src1: *const cpumask, src2: *const cpumask) -> bool;
    pub fn bpf_cpumask_empty(cpumask: *const cpumask) -> bool;
    pub fn bpf_cpumask_full(cpumask: *const cpumask) -> bool;
    pub fn bpf_cpumask_copy(dst: *mut bpf_cpumask, src: *const cpumask);
    pub fn bpf_cpumask_any_distribute(src: *const cpumask) -> u32;
    pub fn bpf_cpumask_any_and_distribute(src1: *const cpumask, src2: *const cpumask) -> u32;
    pub fn bpf_cpumask_weight(cpumask: *const cpumask) -> u32;
    pub fn bpf_cpumask_populate(
        cpumask: *mut bpf_cpumask,
        src: *mut core::ffi::c_void,
        src__sz: size_t,
    ) -> core::ffi::c_int;

    pub fn bpf_rcu_read_lock();
    pub fn bpf_rcu_read_unlock();

    pub fn bpf_map_lookup_elem(
        map: *mut array_map,
        key: *const core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    pub fn bpf_map_update_elem(
        map: *mut array_map,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: core::ffi::c_ulonglong,
    ) -> core::ffi::c_long;
    pub fn bpf_kptr_xchg(
        kptr: *mut *mut bpf_cpumask,
        ptr: *mut bpf_cpumask,
    ) -> *mut bpf_cpumask;
}

pub const ENOENT: core::ffi::c_int = 2;
pub const EEXIST: core::ffi::c_int = 17;

#[inline]
pub unsafe fn cast(cpumask: *mut bpf_cpumask) -> *const cpumask {
    cpumask as *const cpumask
}

#[inline]
pub unsafe fn create_cpumask() -> *mut bpf_cpumask {
    let cpumask: *mut bpf_cpumask;

    cpumask = bpf_cpumask_create();
    if cpumask.is_null() {
        err = 1;
        return core::ptr::null_mut();
    }

    if !bpf_cpumask_empty(cast(cpumask)) {
        err = 2;
        bpf_cpumask_release(cpumask);
        return core::ptr::null_mut();
    }

    cpumask
}

#[inline]
pub unsafe fn cpumask_map_value_lookup() -> *mut __cpumask_map_value {
    let key: u32 = 0;

    bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(__cpumask_map),
        core::ptr::addr_of!(key) as *const core::ffi::c_void,
    ) as *mut __cpumask_map_value
}

#[inline]
pub unsafe fn cpumask_map_insert(mask: *mut bpf_cpumask) -> core::ffi::c_int {
    let mut local: __cpumask_map_value;
    let mut v: *mut __cpumask_map_value;
    let status: core::ffi::c_long;
    let old: *mut bpf_cpumask;
    let key: u32 = 0;

    local = __cpumask_map_value {
        cpumask: core::ptr::null_mut(),
    };
    status = bpf_map_update_elem(
        core::ptr::addr_of_mut!(__cpumask_map),
        core::ptr::addr_of!(key) as *const core::ffi::c_void,
        core::ptr::addr_of!(local) as *const core::ffi::c_void,
        0,
    );
    if status != 0 {
        bpf_cpumask_release(mask);
        return status as core::ffi::c_int;
    }

    v = bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(__cpumask_map),
        core::ptr::addr_of!(key) as *const core::ffi::c_void,
    ) as *mut __cpumask_map_value;
    if v.is_null() {
        bpf_cpumask_release(mask);
        return -ENOENT;
    }

    old = bpf_kptr_xchg(core::ptr::addr_of_mut!((*v).cpumask), mask);
    if !old.is_null() {
        bpf_cpumask_release(old);
        return -EEXIST;
    }

    0
}
