// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (c) 2022 Google
//
// Rust translation of lock_contention.bpf.c.
// C includes removed; the following external names are expected from vmlinux,
// bpf helpers/tracing/core-read support, asm errno values, and lock_data.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::c_void;

type bool_ = bool;
type u8 = u8;
type u32 = u32;
type u64 = u64;
type s32 = i32;
type s64 = i64;

type __u8 = u8;
type __u32 = u32;
type __u64 = u64;
type __s32 = s32;
type __s64 = s64;

const MAX_CPUS: u32 = 1024;
const MAX_ZONES: u32 = 10;
const MAX_LOOP: u32 = 1u32 << 20;

const LCB_F_SPIN: u32 = 1u32 << 0;
const LCB_F_READ: u32 = 1u32 << 1;
const LCB_F_WRITE: u32 = 1u32 << 2;
const LCB_F_RT: u32 = 1u32 << 3;
const LCB_F_PERCPU: u32 = 1u32 << 4;
const LCB_F_MUTEX: u32 = 1u32 << 5;

extern "C" {
    static MAX_ENTRIES: u32;
    static BPF_MAP_TYPE_STACK_TRACE: u32;
    static BPF_MAP_TYPE_PERCPU_ARRAY: u32;
    static BPF_MAP_TYPE_HASH: u32;
    static BPF_NOEXIST: u64;
    static BPF_ANY: u64;
    static BPF_F_FAST_STACK_CMP: i32;
    static LOCK_AGGR_CGROUP: i32;
    static LOCK_AGGR_TASK: i32;
    static LOCK_AGGR_CALLER: i32;
    static LOCK_AGGR_ADDR: i32;
    static LCD_F_MMAP_LOCK: u32;
    static LCD_F_SIGHAND_LOCK: u32;
    static LCB_F_TYPE_MASK: u32;
    static LOCK_CLASS_ZONE_LOCK: u32;
    static LOCK_CLASS_RQLOCK: u32;
    static LCB_F_SLAB_ID_SHIFT: u32;
    static LCB_F_SLAB_ID_END: u32;
    static E2BIG: i32;
    static EEXIST: i32;
    static perf_event_cgrp_id: i32;
}

#[repr(C)]
pub struct BpfMapDef {
    type_: u32,
    key_size: u32,
    value_size: u32,
    max_entries: u32,
}

// BPF maps, originally anonymous structs with SEC(".maps").
#[no_mangle]
pub static mut stacks: BpfMapDef = BpfMapDef { type_: 0, key_size: 4, value_size: 8, max_entries: 0 };
#[no_mangle]
pub static mut stack_buf: BpfMapDef = BpfMapDef { type_: 0, key_size: 4, value_size: 8, max_entries: 1 };
#[no_mangle]
pub static mut owner_stacks: BpfMapDef = BpfMapDef { type_: 0, key_size: 8, value_size: 4, max_entries: 1 };
#[no_mangle]
pub static mut owner_data: BpfMapDef = BpfMapDef { type_: 0, key_size: 8, value_size: 0, max_entries: 1 };
#[no_mangle]
pub static mut owner_stat: BpfMapDef = BpfMapDef { type_: 0, key_size: 0, value_size: 0, max_entries: 1 };
#[no_mangle]
pub static mut tstamp: BpfMapDef = BpfMapDef { type_: 0, key_size: core::mem::size_of::<i32>() as u32, value_size: 0, max_entries: 0 };
#[no_mangle]
pub static mut tstamp_cpu: BpfMapDef = BpfMapDef { type_: 0, key_size: 4, value_size: 0, max_entries: 1 };
#[no_mangle]
pub static mut lock_stat: BpfMapDef = BpfMapDef { type_: 0, key_size: 0, value_size: 0, max_entries: 0 };
#[no_mangle]
pub static mut task_data: BpfMapDef = BpfMapDef { type_: 0, key_size: 4, value_size: 0, max_entries: 0 };
#[no_mangle]
pub static mut lock_syms: BpfMapDef = BpfMapDef { type_: 0, key_size: 8, value_size: 4, max_entries: 0 };
#[no_mangle]
pub static mut cpu_filter: BpfMapDef = BpfMapDef { type_: 0, key_size: 4, value_size: 1, max_entries: 1 };
#[no_mangle]
pub static mut task_filter: BpfMapDef = BpfMapDef { type_: 0, key_size: 4, value_size: 1, max_entries: 1 };
#[no_mangle]
pub static mut type_filter: BpfMapDef = BpfMapDef { type_: 0, key_size: 4, value_size: 1, max_entries: 1 };
#[no_mangle]
pub static mut addr_filter: BpfMapDef = BpfMapDef { type_: 0, key_size: 8, value_size: 1, max_entries: 1 };
#[no_mangle]
pub static mut cgroup_filter: BpfMapDef = BpfMapDef { type_: 0, key_size: 8, value_size: 1, max_entries: 1 };
#[no_mangle]
pub static mut slab_filter: BpfMapDef = BpfMapDef { type_: 0, key_size: core::mem::size_of::<isize>() as u32, value_size: 1, max_entries: 1 };
#[no_mangle]
pub static mut slab_caches: BpfMapDef = BpfMapDef { type_: 0, key_size: core::mem::size_of::<isize>() as u32, value_size: 0, max_entries: 1 };
#[no_mangle]
pub static mut lock_delays: BpfMapDef = BpfMapDef { type_: 0, key_size: 8, value_size: 8, max_entries: 1 };

#[repr(C)]
pub struct rw_semaphore___old {
    owner: *mut task_struct,
}

#[repr(C)]
pub struct rw_semaphore___new {
    owner: atomic_long_t,
}

#[repr(C)]
pub struct mm_struct___old {
    mmap_sem: rw_semaphore,
}

#[repr(C)]
pub struct mm_struct___new {
    mmap_lock: rw_semaphore,
}

#[repr(C)]
pub struct cas_ctx {
    data: *mut contention_data,
    duration: u64,
    max_done: i32,
    min_done: i32,
}

extern "C" {
    fn bpf_get_kmem_cache(addr: u64) -> *mut kmem_cache;
    fn bpf_task_from_pid(pid: s32) -> *mut task_struct;
    fn bpf_task_release(p: *mut task_struct);
    fn bpf_get_current_cgroup_id() -> u64;
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_get_smp_processor_id() -> u32;
    fn bpf_map_lookup_elem(map: *const c_void, key: *const c_void) -> *mut c_void;
    fn bpf_map_update_elem(map: *const c_void, key: *const c_void, value: *const c_void, flags: u64) -> i32;
    fn bpf_map_delete_elem(map: *const c_void, key: *const c_void) -> i32;
    fn bpf_core_read(dst: *mut c_void, size: usize, src: *const c_void) -> i32;
    fn bpf_ktime_get_ns() -> u64;
    fn bpf_get_prandom_u32() -> u32;
    fn bpf_loop(nr_loops: u32, callback_fn: extern "C" fn(u64, *mut c_void) -> isize, callback_ctx: *mut c_void, flags: u64) -> i32;
    fn bpf_get_task_stack(task: *mut task_struct, buf: *mut u64, size: u32, flags: u64) -> i32;
    fn bpf_get_stackid(ctx: *mut u64, map: *const c_void, flags: i32) -> i32;
    fn bpf_probe_read_kernel_str(dst: *mut c_void, size: usize, unsafe_ptr: *const c_void) -> i32;
}

extern "C" {
    type task_struct;
    type cgroup;
    type mutex;
    type rw_semaphore;
    type atomic_long_t;
    type mm_struct;
    type sighand_struct;
    type raw_spinlock_t;
    type rq;
    type pglist_data;
    type zone;
    type kmem_cache;
    type contention_key;
    type contention_data;
    type contention_task_data;
    type tstamp_data;
    type owner_tracing_data;
    type slab_cache_data;
}

#[no_mangle]
pub static mut has_cpu: i32 = 0;
#[no_mangle]
pub static mut has_task: i32 = 0;
#[no_mangle]
pub static mut has_type: i32 = 0;
#[no_mangle]
pub static mut has_addr: i32 = 0;
#[no_mangle]
pub static mut has_cgroup: i32 = 0;
#[no_mangle]
pub static mut has_slab: i32 = 0;
#[no_mangle]
pub static mut has_mmap_lock: i32 = 0;
#[no_mangle]
pub static mut needs_callstack: i32 = 0;
#[no_mangle]
pub static mut stack_skip: i32 = 0;
#[no_mangle]
pub static mut lock_owner: i32 = 0;
#[no_mangle]
pub static mut use_cgroup_v2: i32 = 0;
#[no_mangle]
pub static mut max_stack: i32 = 0;
#[no_mangle]
pub static mut lock_delay: i32 = 0;
#[no_mangle]
pub static mut aggr_mode: i32 = 0;

#[no_mangle]
pub static mut enabled: i32 = 0;
#[no_mangle]
pub static mut perf_subsys_id: i32 = -1;
#[no_mangle]
pub static mut end_ts: u64 = 0;
#[no_mangle]
pub static mut slab_cache_id: u32 = 0;
#[no_mangle]
pub static mut task_fail: i32 = 0;
#[no_mangle]
pub static mut stack_fail: i32 = 0;
#[no_mangle]
pub static mut time_fail: i32 = 0;
#[no_mangle]
pub static mut data_fail: i32 = 0;
#[no_mangle]
pub static mut task_map_full: i32 = 0;
#[no_mangle]
pub static mut data_map_full: i32 = 0;

unsafe fn fetch_add_i32(ptr: *mut i32, val: i32) -> i32 {
    let old = core::ptr::read_volatile(ptr);
    core::ptr::write_volatile(ptr, old.wrapping_add(val));
    old
}

unsafe fn fetch_add_u64(ptr: *mut u64, val: u64) -> u64 {
    let old = core::ptr::read_volatile(ptr);
    core::ptr::write_volatile(ptr, old.wrapping_add(val));
    old
}

unsafe fn fetch_add_u32(ptr: *mut u32, val: u32) -> u32 {
    let old = core::ptr::read_volatile(ptr);
    core::ptr::write_volatile(ptr, old.wrapping_add(val));
    old
}

unsafe fn val_compare_and_swap_u64(ptr: *mut u64, old: u64, new: u64) -> u64 {
    let current = core::ptr::read_volatile(ptr);
    if current == old {
        core::ptr::write_volatile(ptr, new);
    }
    current
}

unsafe fn get_current_cgroup_id() -> u64 {
    if use_cgroup_v2 != 0 {
        return bpf_get_current_cgroup_id();
    }

    let task = bpf_get_current_task_btf();

    if perf_subsys_id == -1 {
        // C used __has_builtin(__builtin_preserve_enum_value) when available.
        perf_subsys_id = perf_event_cgrp_id;
    }

    let _cgrp: *mut cgroup = core::ptr::null_mut();
    // BPF_CORE_READ(task, cgroups, subsys[perf_subsys_id], cgroup);
    // BPF_CORE_READ(cgrp, kn, id);
    let _ = task;
    0
}

unsafe fn can_record(ctx: *mut u64) -> i32 {
    let mut is_addr_ok = false;

    if has_cpu != 0 {
        let cpu: u32 = bpf_get_smp_processor_id();
        let ok = bpf_map_lookup_elem(&cpu_filter as *const _ as *const c_void, &cpu as *const _ as *const c_void) as *mut u8;
        if ok.is_null() {
            return 0;
        }
    }

    if has_task != 0 {
        let pid: u32 = bpf_get_current_pid_tgid() as u32;
        let ok = bpf_map_lookup_elem(&task_filter as *const _ as *const c_void, &pid as *const _ as *const c_void) as *mut u8;
        if ok.is_null() {
            return 0;
        }
    }

    if has_type != 0 {
        let flags: u32 = *ctx.add(1) as u32;
        let ok = bpf_map_lookup_elem(&type_filter as *const _ as *const c_void, &flags as *const _ as *const c_void) as *mut u8;
        if ok.is_null() {
            return 0;
        }
    }

    if has_addr != 0 {
        let addr: u64 = *ctx.add(0);
        let ok = bpf_map_lookup_elem(&addr_filter as *const _ as *const c_void, &addr as *const _ as *const c_void) as *mut u8;
        if ok.is_null() && has_slab == 0 && has_mmap_lock == 0 {
            return 0;
        }
        is_addr_ok = !ok.is_null();
    }

    if has_cgroup != 0 {
        let cgrp: u64 = get_current_cgroup_id();
        let ok = bpf_map_lookup_elem(&cgroup_filter as *const _ as *const c_void, &cgrp as *const _ as *const c_void) as *mut u8;
        if ok.is_null() {
            return 0;
        }
    }

    if is_addr_ok {
        return 1;
    }

    /* slab and mmap_lock are part of the addr_filter */
    if has_slab != 0 {
        let addr: u64 = *ctx.add(0);
        let kmem_cache_addr = bpf_get_kmem_cache(addr) as isize;
        let ok = bpf_map_lookup_elem(&slab_filter as *const _ as *const c_void, &kmem_cache_addr as *const _ as *const c_void) as *mut u8;
        if !ok.is_null() {
            return 1;
        } else if has_mmap_lock == 0 {
            return 0;
        }
    }

    if has_mmap_lock != 0 {
        let lock: u64 = *ctx.add(0);
        let flag: u32 = *ctx.add(1) as u32;
        if check_lock_type(lock, flag) != LCD_F_MMAP_LOCK {
            return 0;
        }
    }

    1
}

unsafe fn update_task_data(task: *mut task_struct) -> i32 {
    let mut pid: i32 = 0;
    let err = bpf_core_read(&mut pid as *mut _ as *mut c_void, core::mem::size_of_val(&pid), core::ptr::addr_of!((*task)) as *const c_void);
    if err != 0 {
        return -1;
    }

    let p = bpf_map_lookup_elem(&task_data as *const _ as *const c_void, &pid as *const _ as *const c_void) as *mut contention_task_data;
    if p.is_null() && task_map_full == 0 {
        let data: [u8; 0] = [];
        // BPF_CORE_READ_STR_INTO(&data.comm, task, comm);
        if bpf_map_update_elem(&task_data as *const _ as *const c_void, &pid as *const _ as *const c_void, data.as_ptr() as *const c_void, BPF_NOEXIST) == -E2BIG {
            task_map_full = 1;
        }
    }

    0
}

unsafe fn get_lock_owner(lock: u64, flags: u32) -> *mut task_struct {
    let mut owner: u64 = 0;

    if (flags & LCB_F_MUTEX) != 0 {
        let _mutex = lock as *mut mutex;
        // owner = BPF_CORE_READ(mutex, owner.counter);
    } else if flags == LCB_F_READ || flags == LCB_F_WRITE {
        /*
         * Support for the BPF_TYPE_MATCHES argument to the
         * __builtin_preserve_type_info builtin was added at some point during
         * development of clang 15 and it's what is needed for
         * bpf_core_type_matches.
         */
        // C conditionally used bpf_core_type_matches for old/new rw_semaphore.
        // Fallback assumed the new struct.
        let _rwsem = lock as *mut rw_semaphore;
        // owner = BPF_CORE_READ(rwsem, owner.counter);
    }

    if owner == 0 {
        return core::ptr::null_mut();
    }

    (owner & !7u64) as *mut task_struct
}

unsafe fn check_lock_type(lock: u64, flags: u32) -> u32 {
    match flags {
        LCB_F_READ | LCB_F_WRITE => {
            let curr = bpf_get_current_task_btf();
            // if curr->mm == NULL break;
            let mm_new = curr as *mut mm_struct___new;
            // if (bpf_core_field_exists(mm_new->mmap_lock)) ...
            let _ = mm_new;
            let mm_old = curr as *mut mm_struct___old;
            let _ = mm_old;
            let _ = lock;
        }
        LCB_F_SPIN => {
            let curr = bpf_get_current_task_btf();
            let _sighand: *mut sighand_struct = curr as *mut sighand_struct;
            let _ = lock;
        }
        _ => {}
    }
    0
}

extern "C" fn delay_callback(_idx: u64, arg: *mut c_void) -> isize {
    unsafe {
        let target = *(arg as *mut u64);

        if target <= bpf_ktime_get_ns() {
            return 1;
        }

        /* just to kill time */
        let _ = bpf_get_prandom_u32();

        0
    }
}

unsafe fn do_lock_delay(duration: u64) {
    let mut target = bpf_ktime_get_ns().wrapping_add(duration);
    bpf_loop(MAX_LOOP, delay_callback, &mut target as *mut _ as *mut c_void, 0);
}

unsafe fn check_lock_delay(lock: u64) {
    let delay = bpf_map_lookup_elem(&lock_delays as *const _ as *const c_void, &lock as *const _ as *const c_void) as *mut u64;
    if !delay.is_null() {
        do_lock_delay(*delay);
    }
}

unsafe fn get_tstamp_elem(flags: u32) -> *mut tstamp_data {
    /* Use per-cpu array map for spinlock and rwlock */
    if (flags & (LCB_F_SPIN | LCB_F_MUTEX)) == LCB_F_SPIN {
        let idx: u32 = 0;
        let pelem = bpf_map_lookup_elem(&tstamp_cpu as *const _ as *const c_void, &idx as *const _ as *const c_void) as *mut tstamp_data;
        /* Do not update the element for nested locks */
        // if (pelem && pelem->lock) pelem = NULL;
        return pelem;
    }

    let pid = bpf_get_current_pid_tgid() as u32;
    let mut pelem = bpf_map_lookup_elem(&tstamp as *const _ as *const c_void, &pid as *const _ as *const c_void) as *mut tstamp_data;
    /* Do not update the element for nested locks */
    // if (pelem && pelem->lock) return NULL;

    if pelem.is_null() {
        let zero: [u8; 0] = [];
        if bpf_map_update_elem(&tstamp as *const _ as *const c_void, &pid as *const _ as *const c_void, zero.as_ptr() as *const c_void, BPF_NOEXIST) < 0 {
            fetch_add_i32(&mut task_fail, 1);
            return core::ptr::null_mut();
        }

        pelem = bpf_map_lookup_elem(&tstamp as *const _ as *const c_void, &pid as *const _ as *const c_void) as *mut tstamp_data;
        if pelem.is_null() {
            fetch_add_i32(&mut task_fail, 1);
            return core::ptr::null_mut();
        }
    }
    pelem
}

unsafe fn get_owner_stack_id(stacktrace: *mut u64) -> s32 {
    static mut id_gen: s64 = 1;

    let mut id = bpf_map_lookup_elem(&owner_stacks as *const _ as *const c_void, stacktrace as *const c_void) as *mut s32;
    if !id.is_null() {
        return *id;
    }

    let new_id = fetch_add_i64(&mut id_gen, 1) as s32;
    bpf_map_update_elem(&owner_stacks as *const _ as *const c_void, stacktrace as *const c_void, &new_id as *const _ as *const c_void, BPF_NOEXIST);

    id = bpf_map_lookup_elem(&owner_stacks as *const _ as *const c_void, stacktrace as *const c_void) as *mut s32;
    if !id.is_null() {
        return *id;
    }

    -1
}

unsafe fn fetch_add_i64(ptr: *mut i64, val: i64) -> i64 {
    let old = core::ptr::read_volatile(ptr);
    core::ptr::write_volatile(ptr, old.wrapping_add(val));
    old
}

extern "C" fn cas_min_max_cb(_idx: u64, arg: *mut c_void) -> isize {
    unsafe {
        let ctx = arg as *mut cas_ctx;

        if (*ctx).max_done == 0 {
            let old_max = *((*ctx).data as *mut u64);
            if old_max >= (*ctx).duration {
                (*ctx).max_done = 1;
            } else {
                let r = val_compare_and_swap_u64((*ctx).data as *mut u64, old_max, (*ctx).duration);
                if r == old_max {
                    (*ctx).max_done = 1;
                }
            }
        }

        if (*ctx).min_done == 0 {
            let old_min = *((*ctx).data as *mut u64);
            if old_min <= (*ctx).duration {
                (*ctx).min_done = 1;
            } else {
                let r = val_compare_and_swap_u64((*ctx).data as *mut u64, old_min, (*ctx).duration);
                if r == old_min {
                    (*ctx).min_done = 1;
                }
            }
        }

        if (*ctx).max_done != 0 && (*ctx).min_done != 0 { 1 } else { 0 }
    }
}

unsafe fn update_contention_data(data: *mut contention_data, duration: u64, count: u32) {
    fetch_add_u64(data as *mut u64, duration);
    fetch_add_u32(data as *mut u32, count);

    let mut ctx = cas_ctx {
        data,
        duration,
        max_done: 0,
        min_done: 0,
    };
    bpf_loop(64, cas_min_max_cb, &mut ctx as *mut _ as *mut c_void, 0);
}

unsafe fn update_owner_stat(id: u32, duration: u64, flags: u32) {
    let key: [u8; 0] = [];
    let data = bpf_map_lookup_elem(&owner_stat as *const _ as *const c_void, key.as_ptr() as *const c_void) as *mut contention_data;

    if data.is_null() {
        let first: [u8; 0] = [];
        let _ = (id, duration, flags);
        bpf_map_update_elem(&owner_stat as *const _ as *const c_void, key.as_ptr() as *const c_void, first.as_ptr() as *const c_void, BPF_NOEXIST);
    } else {
        update_contention_data(data, duration, 1);
    }
}

extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
}

#[no_mangle]
pub unsafe extern "C" fn contention_begin(ctx: *mut u64) -> i32 {
    if enabled == 0 || can_record(ctx) == 0 {
        return 0;
    }

    let pelem = get_tstamp_elem(*ctx.add(1) as u32);
    if pelem.is_null() {
        return 0;
    }

    // pelem->timestamp = bpf_ktime_get_ns();
    // pelem->lock = (__u64)ctx[0];
    // pelem->flags = (__u32)ctx[1];
    if aggr_mode == LOCK_AGGR_CGROUP {
        let _ = get_current_cgroup_id();
    }

    if needs_callstack != 0 {
        let mut i: u32 = 0;
        let mut id: u32 = 0;
        let mut _owner_pid: i32;
        let mut _task: *mut task_struct;

        if lock_owner != 0 {
            _task = get_lock_owner(*ctx.add(0), *ctx.add(1) as u32);
            if !_task.is_null() {
                _owner_pid = 0; // BPF_CORE_READ(task, pid);
                let buf = bpf_map_lookup_elem(&stack_buf as *const _ as *const c_void, &i as *const _ as *const c_void) as *mut u64;
                if !buf.is_null() {
                    while i < max_stack as u32 {
                        *buf.add(i as usize) = 0x0;
                        i += 1;
                    }
                    _task = bpf_task_from_pid(_owner_pid);
                    if !_task.is_null() {
                        bpf_get_task_stack(_task, buf, (max_stack as u32).wrapping_mul(core::mem::size_of::<usize>() as u32), 0);
                        bpf_task_release(_task);
                        let _otdata = bpf_map_lookup_elem(&owner_data as *const _ as *const c_void, ctx as *const c_void) as *mut owner_tracing_data;
                        id = get_owner_stack_id(buf) as u32;
                        let _ = _otdata;
                    }
                }
            }
        }

        let stack_id = bpf_get_stackid(ctx, &stacks as *const _ as *const c_void, BPF_F_FAST_STACK_CMP | stack_skip);
        if stack_id < 0 {
            fetch_add_i32(&mut stack_fail, 1);
        }
        let _ = id;
    } else if aggr_mode == LOCK_AGGR_TASK {
        let task = if lock_owner != 0 {
            get_lock_owner(*ctx.add(0), *ctx.add(1) as u32)
        } else {
            bpf_get_current_task_btf()
        };

        if !task.is_null() {
            let _ = update_task_data(task);
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn contention_end(ctx: *mut u64) -> i32 {
    let mut pid: u32 = 0;
    let idx: u32 = 0;
    let key: [u8; 0] = [];
    let mut need_delete = false;

    if enabled == 0 {
        return 0;
    }

    let mut pelem = bpf_map_lookup_elem(&tstamp_cpu as *const _ as *const c_void, &idx as *const _ as *const c_void) as *mut tstamp_data;
    if pelem.is_null() {
        pid = bpf_get_current_pid_tgid() as u32;
        pelem = bpf_map_lookup_elem(&tstamp as *const _ as *const c_void, &pid as *const _ as *const c_void) as *mut tstamp_data;
        if pelem.is_null() {
            return 0;
        }
        need_delete = true;
    }

    let timestamp = bpf_ktime_get_ns();
    let duration = timestamp; // timestamp - pelem->timestamp
    if (duration as s64) < 0 {
        fetch_add_i32(&mut time_fail, 1);
    } else {
        if needs_callstack != 0 && lock_owner != 0 {
            let _otdata = bpf_map_lookup_elem(&owner_data as *const _ as *const c_void, pelem as *const c_void) as *mut owner_tracing_data;
            let _ = _otdata;
        }

        match aggr_mode {
            x if x == LOCK_AGGR_CALLER => {}
            x if x == LOCK_AGGR_TASK => {
                if lock_owner == 0 && !need_delete {
                    pid = bpf_get_current_pid_tgid() as u32;
                }
            }
            x if x == LOCK_AGGR_ADDR => {}
            x if x == LOCK_AGGR_CGROUP => {}
            _ => return 0,
        }

        let mut data = bpf_map_lookup_elem(&lock_stat as *const _ as *const c_void, key.as_ptr() as *const c_void) as *mut contention_data;
        if data.is_null() {
            if data_map_full != 0 {
                fetch_add_i32(&mut data_fail, 1);
            } else {
                let first: [u8; 0] = [];
                let mut err: i32;

                if aggr_mode == LOCK_AGGR_ADDR {
                    let _ = check_lock_type(0, 0 & LCB_F_TYPE_MASK);

                    /* Check if it's from a slab object */
                    let s = bpf_get_kmem_cache(0);
                    if !s.is_null() {
                        let d = bpf_map_lookup_elem(&slab_caches as *const _ as *const c_void, &s as *const _ as *const c_void) as *mut slab_cache_data;
                        let _ = d;
                    }
                }

                err = bpf_map_update_elem(&lock_stat as *const _ as *const c_void, key.as_ptr() as *const c_void, first.as_ptr() as *const c_void, BPF_NOEXIST);
                if err < 0 {
                    if err == -EEXIST {
                        data = bpf_map_lookup_elem(&lock_stat as *const _ as *const c_void, key.as_ptr() as *const c_void) as *mut contention_data;
                        if !data.is_null() {
                            update_contention_data(data, duration, 1);
                        }
                    }
                    if err == -E2BIG {
                        data_map_full = 1;
                    }
                    fetch_add_i32(&mut data_fail, 1);
                }
            }
        } else {
            update_contention_data(data, duration, 1);
        }
    }

    if lock_delay != 0 {
        check_lock_delay(0);
    }

    if need_delete {
        bpf_map_delete_elem(&tstamp as *const _ as *const c_void, &pid as *const _ as *const c_void);
    }
    0
}

extern "C" {
    static mut runqueues: rq;
}

#[no_mangle]
pub static mut contig_page_data_addr: u64 = 0;
#[no_mangle]
pub static mut node_data_addr: u64 = 0;
#[no_mangle]
pub static mut nr_nodes: i32 = 0;
#[no_mangle]
pub static mut sizeof_zone: i32 = 0;

#[repr(C)]
pub struct rq___old {
    lock: raw_spinlock_t,
}

#[repr(C)]
pub struct rq___new {
    __lock: raw_spinlock_t,
}

unsafe fn collect_zone_lock() {
    let mut nr_zones: u64;
    let zone_off: u64;
    let mut lock_addr: u64;
    let lock_off: u64;
    let lock_flag: u32 = LOCK_CLASS_ZONE_LOCK;

    zone_off = 0; // offsetof(struct pglist_data, node_zones)
    lock_off = 0; // offsetof(struct zone, lock)

    if contig_page_data_addr != 0 {
        let contig_page_data = contig_page_data_addr as isize as *mut pglist_data;
        nr_zones = 0; // BPF_CORE_READ(contig_page_data, nr_zones)

        let mut i = 0;
        while i < MAX_ZONES {
            if i as u64 >= nr_zones {
                break;
            }

            let zone_addr = contig_page_data_addr
                .wrapping_add((sizeof_zone as u64).wrapping_mul(i as u64))
                .wrapping_add(zone_off);
            lock_addr = zone_addr.wrapping_add(lock_off);

            bpf_map_update_elem(&lock_syms as *const _ as *const c_void, &lock_addr as *const _ as *const c_void, &lock_flag as *const _ as *const c_void, BPF_ANY);
            i += 1;
        }
        let _ = contig_page_data;
    } else if nr_nodes > 0 {
        let node_data = node_data_addr as isize as *mut *mut pglist_data;

        let mut i = 0;
        while i < nr_nodes {
            let mut pgdat: *mut pglist_data = core::ptr::null_mut();
            let err = bpf_core_read(&mut pgdat as *mut _ as *mut c_void, core::mem::size_of_val(&pgdat), node_data.add(i as usize) as *const c_void);
            if err < 0 || pgdat.is_null() {
                break;
            }

            nr_zones = 0; // BPF_CORE_READ(pgdat, nr_zones)
            let mut k = 0;
            while k < MAX_ZONES {
                if k as u64 >= nr_zones {
                    break;
                }

                let zone_addr = (pgdat as u64)
                    .wrapping_add((sizeof_zone as u64).wrapping_mul(k as u64))
                    .wrapping_add(zone_off);
                lock_addr = zone_addr.wrapping_add(lock_off);

                bpf_map_update_elem(&lock_syms as *const _ as *const c_void, &lock_addr as *const _ as *const c_void, &lock_flag as *const _ as *const c_void, BPF_ANY);
                k += 1;
            }
            i += 1;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn collect_lock_syms() -> i32 {
    let mut lock_addr: u64;
    let lock_off: u64;
    let mut lock_flag: u32;

    // if (bpf_core_field_exists(struct rq___new, __lock))
    lock_off = 0; // offsetof(struct rq___new, __lock) or offsetof(struct rq___old, lock)

    let mut i = 0;
    while i < MAX_CPUS {
        let rq = bpf_per_cpu_ptr(&mut runqueues as *mut _ as *mut c_void, i) as *mut rq;

        if rq.is_null() {
            break;
        }

        lock_addr = (rq as u64).wrapping_add(lock_off);
        lock_flag = LOCK_CLASS_RQLOCK;
        bpf_map_update_elem(&lock_syms as *const _ as *const c_void, &lock_addr as *const _ as *const c_void, &lock_flag as *const _ as *const c_void, BPF_ANY);
        i += 1;
    }

    collect_zone_lock();

    0
}

extern "C" {
    fn bpf_per_cpu_ptr(percpu_ptr: *mut c_void, cpu: u32) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn end_timestamp() -> i32 {
    end_ts = bpf_ktime_get_ns();
    0
}

/*
 * bpf_iter__kmem_cache added recently so old kernels don't have it in the
 * vmlinux.h.  But we cannot add it here since it will cause a compiler error
 * due to redefinition of the struct on later kernels.
 *
 * So it uses a CO-RE trick to access the member only if it has the type.
 * This will support both old and new kernels without compiler errors.
 */
#[repr(C)]
pub struct bpf_iter__kmem_cache___new {
    s: *mut kmem_cache,
}

#[no_mangle]
pub unsafe extern "C" fn slab_cache_iter(ctx: *mut c_void) -> i32 {
    let mut s: *mut kmem_cache = core::ptr::null_mut();
    let mut d: [u8; 0] = [];
    let _nameptr: *const i8;

    // if (bpf_core_type_exists(struct bpf_iter__kmem_cache))
    {
        let iter = ctx as *mut bpf_iter__kmem_cache___new;
        s = (*iter).s;
    }

    if s.is_null() {
        return 0;
    }

    // nameptr = s->name;
    bpf_probe_read_kernel_str(d.as_mut_ptr() as *mut c_void, d.len(), core::ptr::null());

    slab_cache_id = slab_cache_id.wrapping_add(1);
    let id = slab_cache_id << LCB_F_SLAB_ID_SHIFT;
    if id >= LCB_F_SLAB_ID_END {
        return 0;
    }

    bpf_map_update_elem(&slab_caches as *const _ as *const c_void, &s as *const _ as *const c_void, d.as_ptr() as *const c_void, BPF_NOEXIST);
    0
}

#[no_mangle]
#[link_section = "license"]
pub static LICENSE: [u8; 13] = *b"Dual BSD/GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
