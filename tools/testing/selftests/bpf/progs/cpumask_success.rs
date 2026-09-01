// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

// C includes translated as dependency intent:
// <vmlinux.h>, <bpf/bpf_tracing.h>, <bpf/bpf_helpers.h>, "bpf_misc.h",
// and "cpumask_common.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type u8 = ::core::ffi::c_uchar;
type u32 = ::core::ffi::c_uint;
type u64 = ::core::ffi::c_ulonglong;

const EACCES: i32 = 13;
const EINVAL: i32 = 22;

const CPUMASK_TEST_MASKLEN: usize = ::core::mem::size_of::<cpumask_t>();

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_cpumask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpumask_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct __cpumask_map_value {
    pub cpumask: *mut bpf_cpumask,
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut pid: i32 = 0;
#[unsafe(no_mangle)]
pub static mut nr_cpus: i32 = 0;

#[repr(C)]
pub struct kptr_nested {
    pub mask: *mut bpf_cpumask,
}

#[repr(C)]
pub struct kptr_nested_pair {
    pub mask_1: *mut bpf_cpumask,
    pub mask_2: *mut bpf_cpumask,
}

#[repr(C)]
pub struct kptr_nested_mid {
    pub dummy: i32,
    pub m: kptr_nested,
}

#[repr(C)]
pub struct kptr_nested_deep {
    pub ptrs: [kptr_nested_mid; 2],
    pub ptr_pairs: [kptr_nested_pair; 3],
}

#[repr(C)]
pub struct kptr_nested_deep_array_1_2 {
    pub dummy: i32,
    pub mask: [*mut bpf_cpumask; CPUMASK_KPTR_FIELDS_MAX],
}

#[repr(C)]
pub struct kptr_nested_deep_array_1_1 {
    pub dummy: i32,
    pub d_2: kptr_nested_deep_array_1_2,
}

#[repr(C)]
pub struct kptr_nested_deep_array_1 {
    pub dummy: ::core::ffi::c_long,
    pub d_1: kptr_nested_deep_array_1_1,
}

#[repr(C)]
pub struct kptr_nested_deep_array_2_2 {
    pub dummy: [::core::ffi::c_long; 2],
    pub mask: *mut bpf_cpumask,
}

#[repr(C)]
pub struct kptr_nested_deep_array_2_1 {
    pub dummy: i32,
    pub d_2: [kptr_nested_deep_array_2_2; CPUMASK_KPTR_FIELDS_MAX],
}

#[repr(C)]
pub struct kptr_nested_deep_array_2 {
    pub dummy: ::core::ffi::c_long,
    pub d_1: kptr_nested_deep_array_2_1,
}

#[repr(C)]
pub struct kptr_nested_deep_array_3_2 {
    pub dummy: [::core::ffi::c_long; 2],
    pub mask: *mut bpf_cpumask,
}

#[repr(C)]
pub struct kptr_nested_deep_array_3_1 {
    pub dummy: i32,
    pub d_2: kptr_nested_deep_array_3_2,
}

#[repr(C)]
pub struct kptr_nested_deep_array_3 {
    pub dummy: ::core::ffi::c_long,
    pub d_1: [kptr_nested_deep_array_3_1; CPUMASK_KPTR_FIELDS_MAX],
}

unsafe extern "C" {
    static mut err: i32;
    static mut global_mask: *mut bpf_cpumask;
    static mut CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS: bool;

    fn bpf_get_current_pid_tgid() -> u64;
    fn create_cpumask() -> *mut bpf_cpumask;
    fn bpf_cpumask_create() -> *mut bpf_cpumask;
    fn bpf_cpumask_release(cpumask: *mut bpf_cpumask);
    fn bpf_cpumask_set_cpu(cpu: u32, cpumask: *mut bpf_cpumask);
    fn bpf_cpumask_clear_cpu(cpu: u32, cpumask: *mut bpf_cpumask);
    fn bpf_cpumask_test_cpu(cpu: u32, cpumask: *const cpumask) -> bool;
    fn bpf_cpumask_setall(cpumask: *mut bpf_cpumask);
    fn bpf_cpumask_clear(cpumask: *mut bpf_cpumask);
    fn bpf_cpumask_full(cpumask: *const cpumask) -> bool;
    fn bpf_cpumask_empty(cpumask: *const cpumask) -> bool;
    fn bpf_cpumask_first(cpumask: *const cpumask) -> u32;
    fn bpf_cpumask_first_zero(cpumask: *const cpumask) -> u32;
    fn bpf_cpumask_first_and(mask1: *const cpumask, mask2: *const cpumask) -> u32;
    fn bpf_cpumask_test_and_set_cpu(cpu: u32, cpumask: *mut bpf_cpumask) -> bool;
    fn bpf_cpumask_test_and_clear_cpu(cpu: u32, cpumask: *mut bpf_cpumask) -> bool;
    fn bpf_cpumask_and(dst: *mut bpf_cpumask, mask1: *const cpumask, mask2: *const cpumask) -> bool;
    fn bpf_cpumask_or(dst: *mut bpf_cpumask, mask1: *const cpumask, mask2: *const cpumask);
    fn bpf_cpumask_xor(dst: *mut bpf_cpumask, mask1: *const cpumask, mask2: *const cpumask);
    fn bpf_cpumask_equal(mask1: *const cpumask, mask2: *const cpumask) -> bool;
    fn bpf_cpumask_intersects(mask1: *const cpumask, mask2: *const cpumask) -> bool;
    fn bpf_cpumask_subset(mask1: *const cpumask, mask2: *const cpumask) -> bool;
    fn bpf_cpumask_any_distribute(cpumask: *const cpumask) -> i32;
    fn bpf_cpumask_any_and_distribute(mask1: *const cpumask, mask2: *const cpumask) -> i32;
    fn bpf_cpumask_copy(dst: *mut bpf_cpumask, src: *const cpumask);
    fn bpf_cpumask_weight(cpumask: *const cpumask) -> u32;
    fn bpf_cpumask_populate(cpumask: *mut bpf_cpumask, src: *const ::core::ffi::c_void, len: u32) -> i32;
    fn bpf_printk(fmt: *const u8, ...) -> i32;
    fn bpf_rcu_read_lock();
    fn bpf_rcu_read_unlock();
    fn cpumask_map_insert(cpumask: *mut bpf_cpumask) -> i32;
    fn cpumask_map_value_lookup() -> *mut __cpumask_map_value;
    fn bpf_kptr_xchg(ptr: *mut *mut bpf_cpumask, val: *mut bpf_cpumask) -> *mut bpf_cpumask;
}

// private(MASK) / private(MASK_DEEP) / private(MASK_*) storage annotations
// are preserved here as ordinary private statics with the same data layout.
static mut global_mask_array: [*mut bpf_cpumask; 2] = [::core::ptr::null_mut(); 2];
static mut global_mask_array_l2: [[*mut bpf_cpumask; 1]; 2] = [[::core::ptr::null_mut(); 1]; 2];
static mut global_mask_array_one: [*mut bpf_cpumask; 1] = [::core::ptr::null_mut(); 1];
static mut global_mask_nested: [kptr_nested; 2] = [
    kptr_nested { mask: ::core::ptr::null_mut() },
    kptr_nested { mask: ::core::ptr::null_mut() },
];
static mut global_mask_nested_deep: kptr_nested_deep = kptr_nested_deep {
    ptrs: [
        kptr_nested_mid { dummy: 0, m: kptr_nested { mask: ::core::ptr::null_mut() } },
        kptr_nested_mid { dummy: 0, m: kptr_nested { mask: ::core::ptr::null_mut() } },
    ],
    ptr_pairs: [
        kptr_nested_pair { mask_1: ::core::ptr::null_mut(), mask_2: ::core::ptr::null_mut() },
        kptr_nested_pair { mask_1: ::core::ptr::null_mut(), mask_2: ::core::ptr::null_mut() },
        kptr_nested_pair { mask_1: ::core::ptr::null_mut(), mask_2: ::core::ptr::null_mut() },
    ],
};
static mut global_mask_nested_deep_array_1: kptr_nested_deep_array_1 = unsafe { ::core::mem::zeroed() };
static mut global_mask_nested_deep_array_2: kptr_nested_deep_array_2 = unsafe { ::core::mem::zeroed() };
static mut global_mask_nested_deep_array_3: kptr_nested_deep_array_3 = unsafe { ::core::mem::zeroed() };

static mut bits: [u64; CPUMASK_TEST_MASKLEN / 8 + 1] = [0; CPUMASK_TEST_MASKLEN / 8 + 1];

#[inline(always)]
unsafe fn cast(mask: *mut bpf_cpumask) -> *const cpumask {
    mask as *const cpumask
}

unsafe fn is_test_task() -> bool {
    let cur_pid: i32 = (bpf_get_current_pid_tgid() >> 32) as i32;

    pid == cur_pid
}

unsafe fn create_cpumask_set(
    out1: *mut *mut bpf_cpumask,
    out2: *mut *mut bpf_cpumask,
    out3: *mut *mut bpf_cpumask,
    out4: *mut *mut bpf_cpumask,
) -> bool {
    let mask1: *mut bpf_cpumask;
    let mask2: *mut bpf_cpumask;
    let mask3: *mut bpf_cpumask;
    let mask4: *mut bpf_cpumask;

    mask1 = create_cpumask();
    if mask1.is_null() {
        return false;
    }

    mask2 = create_cpumask();
    if mask2.is_null() {
        bpf_cpumask_release(mask1);
        err = 3;
        return false;
    }

    mask3 = create_cpumask();
    if mask3.is_null() {
        bpf_cpumask_release(mask1);
        bpf_cpumask_release(mask2);
        err = 4;
        return false;
    }

    mask4 = create_cpumask();
    if mask4.is_null() {
        bpf_cpumask_release(mask1);
        bpf_cpumask_release(mask2);
        bpf_cpumask_release(mask3);
        err = 5;
        return false;
    }

    *out1 = mask1;
    *out2 = mask2;
    *out3 = mask3;
    *out4 = mask4;

    true
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_alloc_free_cpumask(_task: *mut task_struct, _clone_flags: u64) -> i32 {
    let cpumask: *mut bpf_cpumask;

    if !is_test_task() {
        return 0;
    }

    cpumask = create_cpumask();
    if cpumask.is_null() {
        return 0;
    }

    bpf_cpumask_release(cpumask);
    0
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_set_clear_cpu(_task: *mut task_struct, _clone_flags: u64) -> i32 {
    let cpumask: *mut bpf_cpumask;

    if !is_test_task() {
        return 0;
    }

    cpumask = create_cpumask();
    if cpumask.is_null() {
        return 0;
    }

    bpf_cpumask_set_cpu(0, cpumask);
    if !bpf_cpumask_test_cpu(0, cast(cpumask)) {
        err = 3;
    } else {
        bpf_cpumask_clear_cpu(0, cpumask);
        if bpf_cpumask_test_cpu(0, cast(cpumask)) {
            err = 4;
        }
    }

    bpf_cpumask_release(cpumask);
    0
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_setall_clear_cpu(_task: *mut task_struct, _clone_flags: u64) -> i32 {
    let cpumask: *mut bpf_cpumask;

    if !is_test_task() {
        return 0;
    }

    cpumask = create_cpumask();
    if cpumask.is_null() {
        return 0;
    }

    bpf_cpumask_setall(cpumask);
    if !bpf_cpumask_full(cast(cpumask)) {
        err = 3;
    } else {
        bpf_cpumask_clear(cpumask);
        if !bpf_cpumask_empty(cast(cpumask)) {
            err = 4;
        }
    }

    bpf_cpumask_release(cpumask);
    0
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_first_firstzero_cpu(_task: *mut task_struct, _clone_flags: u64) -> i32 {
    let cpumask: *mut bpf_cpumask;

    if !is_test_task() {
        return 0;
    }

    cpumask = create_cpumask();
    if cpumask.is_null() {
        return 0;
    }

    if (bpf_cpumask_first(cast(cpumask)) as i32) < nr_cpus {
        err = 3;
    } else if bpf_cpumask_first_zero(cast(cpumask)) != 0 {
        bpf_printk(c"first zero: %d".as_ptr() as *const u8, bpf_cpumask_first_zero(cast(cpumask)));
        err = 4;
    } else {
        bpf_cpumask_set_cpu(0, cpumask);
        if bpf_cpumask_first(cast(cpumask)) != 0 {
            err = 5;
        } else if bpf_cpumask_first_zero(cast(cpumask)) != 1 {
            err = 6;
        }
    }

    bpf_cpumask_release(cpumask);
    0
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_firstand_nocpu(_task: *mut task_struct, _clone_flags: u64) -> i32 {
    let mut mask1: *mut bpf_cpumask = ::core::ptr::null_mut();
    let mut mask2: *mut bpf_cpumask = ::core::ptr::null_mut();
    let first: u32;

    if !is_test_task() {
        return 0;
    }

    mask1 = create_cpumask();
    if mask1.is_null() {
        return 0;
    }

    mask2 = create_cpumask();
    if !mask2.is_null() {
        bpf_cpumask_set_cpu(0, mask1);
        bpf_cpumask_set_cpu(1, mask2);

        first = bpf_cpumask_first_and(cast(mask1), cast(mask2));
        if first <= 1 {
            err = 3;
        }
    }

    if !mask1.is_null() {
        bpf_cpumask_release(mask1);
    }
    if !mask2.is_null() {
        bpf_cpumask_release(mask2);
    }
    0
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_test_and_set_clear(_task: *mut task_struct, _clone_flags: u64) -> i32 {
    let cpumask: *mut bpf_cpumask;

    if !is_test_task() {
        return 0;
    }

    cpumask = create_cpumask();
    if cpumask.is_null() {
        return 0;
    }

    if bpf_cpumask_test_and_set_cpu(0, cpumask) {
        err = 3;
    } else if !bpf_cpumask_test_and_set_cpu(0, cpumask) {
        err = 4;
    } else if !bpf_cpumask_test_and_clear_cpu(0, cpumask) {
        err = 5;
    }

    bpf_cpumask_release(cpumask);
    0
}

unsafe fn release_four(mask1: *mut bpf_cpumask, mask2: *mut bpf_cpumask, dst1: *mut bpf_cpumask, dst2: *mut bpf_cpumask) {
    bpf_cpumask_release(mask1);
    bpf_cpumask_release(mask2);
    bpf_cpumask_release(dst1);
    bpf_cpumask_release(dst2);
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_and_or_xor(_task: *mut task_struct, _clone_flags: u64) -> i32 {
    let mut mask1: *mut bpf_cpumask = ::core::ptr::null_mut();
    let mut mask2: *mut bpf_cpumask = ::core::ptr::null_mut();
    let mut dst1: *mut bpf_cpumask = ::core::ptr::null_mut();
    let mut dst2: *mut bpf_cpumask = ::core::ptr::null_mut();

    if !is_test_task() {
        return 0;
    }

    if !create_cpumask_set(&mut mask1, &mut mask2, &mut dst1, &mut dst2) {
        return 0;
    }

    bpf_cpumask_set_cpu(0, mask1);
    bpf_cpumask_set_cpu(1, mask2);

    if bpf_cpumask_and(dst1, cast(mask1), cast(mask2)) {
        err = 6;
    } else if !bpf_cpumask_empty(cast(dst1)) {
        err = 7;
    } else {
        bpf_cpumask_or(dst1, cast(mask1), cast(mask2));
        if !bpf_cpumask_test_cpu(0, cast(dst1)) {
            err = 8;
        } else if !bpf_cpumask_test_cpu(1, cast(dst1)) {
            err = 9;
        } else {
            bpf_cpumask_xor(dst2, cast(mask1), cast(mask2));
            if !bpf_cpumask_equal(cast(dst1), cast(dst2)) {
                err = 10;
            }
        }
    }

    release_four(mask1, mask2, dst1, dst2);
    0
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_intersects_subset(_task: *mut task_struct, _clone_flags: u64) -> i32 {
    let mut mask1: *mut bpf_cpumask = ::core::ptr::null_mut();
    let mut mask2: *mut bpf_cpumask = ::core::ptr::null_mut();
    let mut dst1: *mut bpf_cpumask = ::core::ptr::null_mut();
    let mut dst2: *mut bpf_cpumask = ::core::ptr::null_mut();

    if !is_test_task() {
        return 0;
    }

    if !create_cpumask_set(&mut mask1, &mut mask2, &mut dst1, &mut dst2) {
        return 0;
    }

    bpf_cpumask_set_cpu(0, mask1);
    bpf_cpumask_set_cpu(1, mask2);
    if bpf_cpumask_intersects(cast(mask1), cast(mask2)) {
        err = 6;
    } else {
        bpf_cpumask_or(dst1, cast(mask1), cast(mask2));
        if !bpf_cpumask_subset(cast(mask1), cast(dst1)) {
            err = 7;
        } else if !bpf_cpumask_subset(cast(mask2), cast(dst1)) {
            err = 8;
        } else if bpf_cpumask_subset(cast(dst1), cast(mask1)) {
            err = 9;
        }
    }

    release_four(mask1, mask2, dst1, dst2);
    0
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_copy_any_anyand(_task: *mut task_struct, _clone_flags: u64) -> i32 {
    let mut mask1: *mut bpf_cpumask = ::core::ptr::null_mut();
    let mut mask2: *mut bpf_cpumask = ::core::ptr::null_mut();
    let mut dst1: *mut bpf_cpumask = ::core::ptr::null_mut();
    let mut dst2: *mut bpf_cpumask = ::core::ptr::null_mut();
    let mut cpu: i32;

    if !is_test_task() {
        return 0;
    }

    if !create_cpumask_set(&mut mask1, &mut mask2, &mut dst1, &mut dst2) {
        return 0;
    }

    bpf_cpumask_set_cpu(0, mask1);
    bpf_cpumask_set_cpu(1, mask2);
    bpf_cpumask_or(dst1, cast(mask1), cast(mask2));

    cpu = bpf_cpumask_any_distribute(cast(mask1));
    if cpu != 0 {
        err = 6;
    } else {
        cpu = bpf_cpumask_any_distribute(cast(dst2));
        if cpu < nr_cpus {
            err = 7;
        } else {
            bpf_cpumask_copy(dst2, cast(dst1));
            if !bpf_cpumask_equal(cast(dst1), cast(dst2)) {
                err = 8;
            } else {
                cpu = bpf_cpumask_any_distribute(cast(dst2));
                if cpu > 1 {
                    err = 9;
                } else {
                    cpu = bpf_cpumask_any_and_distribute(cast(mask1), cast(mask2));
                    if cpu < nr_cpus {
                        err = 10;
                    }
                }
            }
        }
    }

    release_four(mask1, mask2, dst1, dst2);
    0
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_insert_leave(_task: *mut task_struct, _clone_flags: u64) -> i32 {
    let cpumask: *mut bpf_cpumask;

    cpumask = create_cpumask();
    if cpumask.is_null() {
        return 0;
    }

    if cpumask_map_insert(cpumask) != 0 {
        err = 3;
    }

    0
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_insert_remove_release(_task: *mut task_struct, _clone_flags: u64) -> i32 {
    let mut cpumask: *mut bpf_cpumask;
    let v: *mut __cpumask_map_value;

    cpumask = create_cpumask();
    if cpumask.is_null() {
        return 0;
    }

    if cpumask_map_insert(cpumask) != 0 {
        err = 3;
        return 0;
    }

    v = cpumask_map_value_lookup();
    if v.is_null() {
        err = 4;
        return 0;
    }

    cpumask = bpf_kptr_xchg(&mut (*v).cpumask, ::core::ptr::null_mut());
    if !cpumask.is_null() {
        bpf_cpumask_release(cpumask);
    } else {
        err = 5;
    }

    0
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_global_mask_rcu(_task: *mut task_struct, _clone_flags: u64) -> i32 {
    let mut local: *mut bpf_cpumask;
    let prev: *mut bpf_cpumask;

    if !is_test_task() {
        return 0;
    }

    local = create_cpumask();
    if local.is_null() {
        return 0;
    }

    prev = bpf_kptr_xchg(&mut global_mask, local);
    if !prev.is_null() {
        bpf_cpumask_release(prev);
        err = 3;
        return 0;
    }

    bpf_rcu_read_lock();
    local = global_mask;
    if local.is_null() {
        err = 4;
        bpf_rcu_read_unlock();
        return 0;
    }

    bpf_cpumask_test_cpu(0, local as *const cpumask);
    bpf_rcu_read_unlock();

    0
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_global_mask_array_one_rcu(_task: *mut task_struct, _clone_flags: u64) -> i32 {
    let mut local: *mut bpf_cpumask;
    let prev: *mut bpf_cpumask;

    if !is_test_task() {
        return 0;
    }

    /* Kptr arrays with one element are special cased, being treated
     * just like a single pointer.
     */

    local = create_cpumask();
    if local.is_null() {
        return 0;
    }

    prev = bpf_kptr_xchg(&mut global_mask_array_one[0], local);
    if !prev.is_null() {
        bpf_cpumask_release(prev);
        err = 3;
        return 0;
    }

    bpf_rcu_read_lock();
    local = global_mask_array_one[0];
    if local.is_null() {
        err = 4;
        bpf_rcu_read_unlock();
        return 0;
    }

    bpf_rcu_read_unlock();

    0
}

unsafe fn _global_mask_array_rcu(mask0: *mut *mut bpf_cpumask, mask1: *mut *mut bpf_cpumask) -> i32 {
    let mut local: *mut bpf_cpumask;

    if !is_test_task() {
        return 0;
    }

    /* Check if two kptrs in the array work and independently */

    local = create_cpumask();
    if local.is_null() {
        return 0;
    }

    bpf_rcu_read_lock();

    local = bpf_kptr_xchg(mask0, local);
    if !local.is_null() {
        err = 1;
    } else if (*mask0).is_null() {
        /* [<mask 0>, *] */
        err = 2;
    } else if mask1.is_null() {
    } else if !(*mask1).is_null() {
        /* [*, NULL] */
        err = 3;
    } else {
        local = create_cpumask();
        if local.is_null() {
            err = 9;
        } else {
            local = bpf_kptr_xchg(mask1, local);
            if !local.is_null() {
                err = 10;
            } else if (*mask0).is_null() || (*mask1).is_null() || *mask0 == *mask1 {
                /* [<mask 0>, <mask 1>] */
                err = 11;
            }
        }
    }

    if !local.is_null() {
        bpf_cpumask_release(local);
    }
    bpf_rcu_read_unlock();
    0
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_global_mask_array_rcu(_task: *mut task_struct, _clone_flags: u64) -> i32 {
    _global_mask_array_rcu(&mut global_mask_array[0], &mut global_mask_array[1])
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_global_mask_array_l2_rcu(_task: *mut task_struct, _clone_flags: u64) -> i32 {
    _global_mask_array_rcu(&mut global_mask_array_l2[0][0], &mut global_mask_array_l2[1][0])
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_global_mask_nested_rcu(_task: *mut task_struct, _clone_flags: u64) -> i32 {
    _global_mask_array_rcu(&mut global_mask_nested[0].mask, &mut global_mask_nested[1].mask)
}

/* Ensure that the field->offset has been correctly advanced from one
 * nested struct or array sub-tree to another. In the case of
 * kptr_nested_deep, it comprises two sub-trees: ktpr_1 and kptr_2.  By
 * calling bpf_kptr_xchg() on every single kptr in both nested sub-trees,
 * the verifier should reject the program if the field->offset of any kptr
 * is incorrect.
 *
 * For instance, if we have 10 kptrs in a nested struct and a program that
 * accesses each kptr individually with bpf_kptr_xchg(), the compiler
 * should emit instructions to access 10 different offsets if it works
 * correctly. If the field->offset values of any pair of them are
 * incorrectly the same, the number of unique offsets in btf_record for
 * this nested struct should be less than 10. The verifier should fail to
 * discover some of the offsets emitted by the compiler.
 *
 * Even if the field->offset values of kptrs are not duplicated, the
 * verifier should fail to find a btf_field for the instruction accessing a
 * kptr if the corresponding field->offset is pointing to a random
 * incorrect offset.
 */
#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_global_mask_nested_deep_rcu(_task: *mut task_struct, _clone_flags: u64) -> i32 {
    let mut r: i32;
    let mut i: i32;

    r = _global_mask_array_rcu(
        &mut global_mask_nested_deep.ptrs[0].m.mask,
        &mut global_mask_nested_deep.ptrs[1].m.mask,
    );
    if r != 0 {
        return r;
    }

    i = 0;
    while i < 3 {
        r = _global_mask_array_rcu(
            &mut global_mask_nested_deep.ptr_pairs[i as usize].mask_1,
            &mut global_mask_nested_deep.ptr_pairs[i as usize].mask_2,
        );
        if r != 0 {
            return r;
        }
        i += 1;
    }
    0
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_global_mask_nested_deep_array_rcu(_task: *mut task_struct, _clone_flags: u64) -> i32 {
    let mut i: i32;

    i = 0;
    while i < CPUMASK_KPTR_FIELDS_MAX as i32 {
        _global_mask_array_rcu(&mut global_mask_nested_deep_array_1.d_1.d_2.mask[i as usize], ::core::ptr::null_mut());
        i += 1;
    }

    i = 0;
    while i < CPUMASK_KPTR_FIELDS_MAX as i32 {
        _global_mask_array_rcu(&mut global_mask_nested_deep_array_2.d_1.d_2[i as usize].mask, ::core::ptr::null_mut());
        i += 1;
    }

    i = 0;
    while i < CPUMASK_KPTR_FIELDS_MAX as i32 {
        _global_mask_array_rcu(&mut global_mask_nested_deep_array_3.d_1[i as usize].d_2.mask, ::core::ptr::null_mut());
        i += 1;
    }

    0
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_cpumask_weight(_task: *mut task_struct, _clone_flags: u64) -> i32 {
    let local: *mut bpf_cpumask;

    if !is_test_task() {
        return 0;
    }

    local = create_cpumask();
    if local.is_null() {
        return 0;
    }

    if bpf_cpumask_weight(cast(local)) != 0 {
        err = 3;
    } else {
        bpf_cpumask_set_cpu(0, local);
        if bpf_cpumask_weight(cast(local)) != 1 {
            err = 4;
        } else {
            /*
             * Make sure that adding additional CPUs changes the weight. Test to
             * see whether the CPU was set to account for running on UP machines.
             */
            bpf_cpumask_set_cpu(1, local);
            if bpf_cpumask_test_cpu(1, cast(local)) && bpf_cpumask_weight(cast(local)) != 2 {
                err = 5;
            } else {
                bpf_cpumask_clear(local);
                if bpf_cpumask_weight(cast(local)) != 0 {
                    err = 6;
                }
            }
        }
    }

    bpf_cpumask_release(local);
    0
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_refcount_null_tracking(_task: *mut task_struct, _clone_flags: u64) -> i32 {
    let mask1: *mut bpf_cpumask;
    let mask2: *mut bpf_cpumask;

    mask1 = bpf_cpumask_create();
    mask2 = bpf_cpumask_create();

    if !mask1.is_null() && !mask2.is_null() {
        bpf_cpumask_test_cpu(0, mask1 as *const cpumask);
        bpf_cpumask_test_cpu(0, mask2 as *const cpumask);
    }

    if !mask1.is_null() {
        bpf_cpumask_release(mask1);
    }
    if !mask2.is_null() {
        bpf_cpumask_release(mask2);
    }
    0
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_populate_reject_small_mask(_task: *mut task_struct, _clone_flags: u64) -> i32 {
    let local: *mut bpf_cpumask;
    let mut toofewbits: u8 = 0;
    let ret: i32;

    if !is_test_task() {
        return 0;
    }

    local = create_cpumask();
    if local.is_null() {
        return 0;
    }

    /* The kfunc should prevent this operation */
    ret = bpf_cpumask_populate(
        local,
        &mut toofewbits as *mut u8 as *const ::core::ffi::c_void,
        ::core::mem::size_of_val(&toofewbits) as u32,
    );
    if ret != -EACCES {
        err = 2;
    }

    bpf_cpumask_release(local);

    0
}

/* Mask is guaranteed to be large enough for bpf_cpumask_t. */
/* Add an extra word for the test_populate_reject_unaligned test. */
/* extern bool CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS __kconfig __weak; */

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_populate_reject_unaligned(_task: *mut task_struct, _clone_flags: u64) -> i32 {
    let mask: *mut bpf_cpumask;
    let src: *mut ::core::ffi::c_char;
    let ret: i32;

    if !is_test_task() {
        return 0;
    }

    /* Skip if unaligned accesses are fine for this arch.  */
    if CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS {
        return 0;
    }

    mask = bpf_cpumask_create();
    if mask.is_null() {
        err = 1;
        return 0;
    }

    /* Misalign the source array by a byte. */
    src = (bits.as_mut_ptr() as *mut ::core::ffi::c_char).add(1);

    ret = bpf_cpumask_populate(mask, src as *const ::core::ffi::c_void, CPUMASK_TEST_MASKLEN as u32);
    if ret != -EINVAL {
        err = 2;
    }

    bpf_cpumask_release(mask);

    0
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_populate(_task: *mut task_struct, _clone_flags: u64) -> i32 {
    let mask: *mut bpf_cpumask;
    let mut bit: bool;
    let ret: i32;
    let mut i: i32;

    if !is_test_task() {
        return 0;
    }

    /* Set only odd bits. */
    ::core::ptr::write_bytes(bits.as_mut_ptr() as *mut u8, 0xaa, CPUMASK_TEST_MASKLEN);

    mask = bpf_cpumask_create();
    if mask.is_null() {
        err = 1;
        return 0;
    }

    /* Pass the entire bits array, the kfunc will only copy the valid bits. */
    ret = bpf_cpumask_populate(mask, bits.as_ptr() as *const ::core::ffi::c_void, CPUMASK_TEST_MASKLEN as u32);
    if ret != 0 {
        err = 2;
    } else {
        /*
         * Test is there to appease the verifier. We cannot directly
         * access NR_CPUS, the upper bound for nr_cpus, so we infer
         * it from the size of cpumask_t.
         */
        if nr_cpus < 0 || nr_cpus > (CPUMASK_TEST_MASKLEN * 8) as i32 {
            err = 3;
        } else {
            i = 0;
            while i < nr_cpus {
                /* Odd-numbered bits should be set, even ones unset. */
                bit = bpf_cpumask_test_cpu(i as u32, mask as *const cpumask);
                if bit == (i % 2 != 0) {
                    i += 1;
                    continue;
                }

                err = 4;
                break;
            }
        }
    }

    bpf_cpumask_release(mask);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
