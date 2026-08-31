// SPDX-License-Identifier: GPL-2.0
/*
 * Arena-based task data scheduler. This is a variation of scx_simple
 * that uses a combined allocator and indexing structure to organize
 * task data. Task context allocation is done when a task enters the
 * scheduler, while freeing is done when it exits. Task contexts are
 * retrieved from task-local storage, pointing to the allocated memory.
 *
 * The main purpose of this scheduler is to demostrate arena memory
 * management.
 *
 * Copyright (c) 2024-2025 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2024-2025 Emil Tsalapatis <etsal@meta.com>
 * Copyright (c) 2024-2025 Tejun Heo <tj@kernel.org>
 *
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

// C dependencies removed from executable Rust:
// #include <scx/common.bpf.h>
// #include <scx/bpf_arena_common.bpf.h>
// #include "scx_sdt.h"

type __u64 = u64;
type u64 = u64;
type s32 = i32;
type size_t = usize;

const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const BPF_MAP_TYPE_ARENA: u32 = 0;
const BPF_MAP_TYPE_TASK_STORAGE: u32 = 0;
const BPF_F_MMAPABLE: u32 = 0;
const BPF_F_NO_PREALLOC: u32 = 0;
const BPF_LOCAL_STORAGE_GET_F_CREATE: u64 = 0;
const PAGE_SIZE: __u64 = 4096;
const NUMA_NO_NODE: i32 = -1;
const SCX_DSQ_LOCAL: u64 = 0;
const SCX_SLICE_DFL: u64 = 0;
const SDT_TASK_ENTS_PER_CHUNK: __u64 = 0;
const SDT_TASK_LEVELS: usize = 0;
const SDT_TASK_ENTS_PER_PAGE_SHIFT: __u64 = 0;
const SDT_TASK_CHUNK_BITMAP_U64S: __u64 = 0;
const SDT_TASK_MIN_ELEM_PER_ALLOC: __u64 = 0;

#[repr(C)]
pub struct bpf_spin_lock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    pub pid: i32,
}

#[repr(C)]
pub struct scx_init_task_args {
    _private: [u8; 0],
}

#[repr(C)]
pub struct scx_exit_task_args {
    _private: [u8; 0],
}

#[repr(C)]
pub struct scx_exit_info {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sdt_id {
    pub idx: __u64,
    pub genn: __u64,
}

#[repr(C)]
pub struct sdt_data {
    pub tid: sdt_id,
    pub payload: [__u64; 0],
}

#[repr(C)]
pub struct sdt_chunk {
    pub descs: [*mut sdt_desc; SDT_TASK_ENTS_PER_CHUNK as usize],
    pub data: [*mut sdt_data; SDT_TASK_ENTS_PER_CHUNK as usize],
}

#[repr(C)]
pub struct sdt_desc {
    pub nr_free: __u64,
    pub chunk: *mut sdt_chunk,
    pub allocated: [__u64; SDT_TASK_CHUNK_BITMAP_U64S as usize],
}

type sdt_desc_t = sdt_desc;

#[repr(C)]
pub struct sdt_pool {
    pub elem_size: __u64,
    pub max_elems: __u64,
    pub slab: *mut core::ffi::c_void,
    pub idx: __u64,
}

#[repr(C)]
pub struct scx_alloc_stats {
    pub chunk_allocs: __u64,
    pub data_allocs: __u64,
    pub alloc_ops: __u64,
    pub active_allocs: __u64,
    pub free_ops: __u64,
}

#[repr(C)]
pub struct scx_allocator {
    pub pool: sdt_pool,
    pub root: *mut sdt_desc_t,
}

#[repr(C)]
pub struct scx_stats {
    pub pid: i32,
    pub enqueue: __u64,
    pub init: __u64,
    pub exit: __u64,
    pub select_idle_cpu: __u64,
    pub select_busy_cpu: __u64,
}

#[repr(C)]
pub struct ArenaMap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct TaskStorageMap {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_arena_alloc_pages(
        arena: *mut ArenaMap,
        addr: *mut core::ffi::c_void,
        pages: __u64,
        node: i32,
        flags: u32,
    ) -> *mut core::ffi::c_void;
    fn bpf_printk(fmt: *const u8, ...) -> i32;
    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
    fn bpf_task_storage_get(
        map: *mut TaskStorageMap,
        task: *mut task_struct,
        value: *mut core::ffi::c_void,
        flags: u64,
    ) -> *mut core::ffi::c_void;
    fn bpf_task_storage_delete(map: *mut TaskStorageMap, task: *mut task_struct) -> i32;
    fn scx_bpf_error(fmt: *const u8, ...) -> i32;
    fn scx_bpf_select_cpu_dfl(
        p: *mut task_struct,
        prev_cpu: s32,
        wake_flags: u64,
        is_idle: *mut bool,
    ) -> s32;
    fn scx_bpf_dsq_insert(p: *mut task_struct, dsq_id: u64, slice: u64, enq_flags: u64);
    fn scx_bpf_dsq_move_to_local(dsq_id: u64, flags: u64);
    fn scx_bpf_create_dsq(dsq_id: u64, node: i32) -> s32;
    fn UEI_RECORD(uei: *mut core::ffi::c_void, ei: *mut scx_exit_info);
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

// UEI_DEFINE(uei);
#[unsafe(no_mangle)]
pub static mut uei: core::ffi::c_void = unsafe { core::mem::zeroed() };

// Original BPF map declaration used BPF_MAP_TYPE_ARENA, BPF_F_MMAPABLE,
// max_entries depending on target architecture, and map_extra as mmap start.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut arena: ArenaMap = ArenaMap { _private: [] };

const SHARED_DSQ: u64 = 0;

#[inline(always)]
unsafe fn stat_inc_enqueue(stats: *mut scx_stats) {
    unsafe {
        (*stats).enqueue = (*stats).enqueue.wrapping_add(1);
    }
}
static mut stat_enqueue: __u64 = 0;

#[inline(always)]
unsafe fn stat_inc_init(stats: *mut scx_stats) {
    unsafe {
        (*stats).init = (*stats).init.wrapping_add(1);
    }
}
static mut stat_init: __u64 = 0;

#[inline(always)]
unsafe fn stat_inc_exit(stats: *mut scx_stats) {
    unsafe {
        (*stats).exit = (*stats).exit.wrapping_add(1);
    }
}
static mut stat_exit: __u64 = 0;

#[inline(always)]
unsafe fn stat_inc_select_idle_cpu(stats: *mut scx_stats) {
    unsafe {
        (*stats).select_idle_cpu = (*stats).select_idle_cpu.wrapping_add(1);
    }
}
static mut stat_select_idle_cpu: __u64 = 0;

#[inline(always)]
unsafe fn stat_inc_select_busy_cpu(stats: *mut scx_stats) {
    unsafe {
        (*stats).select_busy_cpu = (*stats).select_busy_cpu.wrapping_add(1);
    }
}
static mut stat_select_busy_cpu: __u64 = 0;

/*
 * Necessary for cond_break/can_loop's semantics. According to kernel commit
 * 011832b, the loop counter variable must be seen as imprecise and bounded
 * by the verifier. Initializing it from a constant (e.g., i = 0;), then,
 * makes it precise and prevents may_goto from helping with converging the
 * loop. For these loops we must initialize the loop counter from a variable
 * whose value the verifier cannot reason about when checking the program, so
 * that the loop counter's value is imprecise.
 */
static mut zero: __u64 = 0;

/*
 * XXX Hack to get the verifier to find the arena for sdt_exit_task.
 * As of 6.12-rc5, The verifier associates arenas with programs by
 * checking LD.IMM instruction operands for an arena and populating
 * the program state with the first instance it finds. This requires
 * accessing our global arena variable, but scx methods do not necessarily
 * do so while still using pointers from that arena. Insert a bpf_printk
 * statement that triggers at most once to generate an LD.IMM instruction
 * to access the arena and help the verifier.
 */
static mut scx_arena_verify_once: bool = false;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn scx_arena_subprog_init() {
    unsafe {
        if core::ptr::read_volatile(core::ptr::addr_of!(scx_arena_verify_once)) {
            return;
        }

        bpf_printk(c"%s: arena pointer %p".as_ptr() as *const u8, c"scx_arena_subprog_init".as_ptr(), core::ptr::addr_of_mut!(arena));
        core::ptr::write_volatile(core::ptr::addr_of_mut!(scx_arena_verify_once), true);
    }
}

static mut alloc_lock: bpf_spin_lock = bpf_spin_lock { _private: [] };
static mut alloc_pool_lock: bpf_spin_lock = bpf_spin_lock { _private: [] };

/* allocation pools */
static mut desc_pool: sdt_pool = sdt_pool {
    elem_size: 0,
    max_elems: 0,
    slab: core::ptr::null_mut(),
    idx: 0,
};
static mut chunk_pool: sdt_pool = sdt_pool {
    elem_size: 0,
    max_elems: 0,
    slab: core::ptr::null_mut(),
    idx: 0,
};

/* Protected by alloc_lock. */
static mut alloc_stats: scx_alloc_stats = scx_alloc_stats {
    chunk_allocs: 0,
    data_allocs: 0,
    alloc_ops: 0,
    active_allocs: 0,
    free_ops: 0,
};

#[inline(always)]
fn div_round_up(n: __u64, d: __u64) -> __u64 {
    n.wrapping_add(d).wrapping_sub(1) / d
}

#[inline(always)]
fn round_up(n: __u64, d: __u64) -> __u64 {
    div_round_up(n, d) * d
}

/* Allocate element from the pool. Must be called with a then pool lock held. */
unsafe fn scx_alloc_from_pool(pool: *mut sdt_pool) -> *mut core::ffi::c_void {
    unsafe {
        let elem_size: __u64;
        let max_elems: __u64;
        let slab: *mut core::ffi::c_void;
        let ptr: *mut core::ffi::c_void;

        elem_size = (*pool).elem_size;
        max_elems = (*pool).max_elems;

        /* If the chunk is spent, get a new one. */
        if (*pool).idx >= max_elems {
            slab = bpf_arena_alloc_pages(
                core::ptr::addr_of_mut!(arena),
                core::ptr::null_mut(),
                div_round_up(max_elems.wrapping_mul(elem_size), PAGE_SIZE),
                NUMA_NO_NODE,
                0,
            );
            if slab.is_null() {
                return core::ptr::null_mut();
            }

            (*pool).slab = slab;
            (*pool).idx = 0;
        }

        ptr = ((*pool).slab as usize).wrapping_add(elem_size.wrapping_mul((*pool).idx) as usize)
            as *mut core::ffi::c_void;
        (*pool).idx = (*pool).idx.wrapping_add(1);

        ptr
    }
}

/* Alloc desc and associated chunk. Called with the allocator spinlock held. */
unsafe fn scx_alloc_chunk() -> *mut sdt_desc_t {
    unsafe {
        let chunk: *mut sdt_chunk;
        let desc: *mut sdt_desc_t;
        let out: *mut sdt_desc_t;

        chunk = scx_alloc_from_pool(core::ptr::addr_of_mut!(chunk_pool)) as *mut sdt_chunk;
        if chunk.is_null() {
            return core::ptr::null_mut();
        }

        desc = scx_alloc_from_pool(core::ptr::addr_of_mut!(desc_pool)) as *mut sdt_desc_t;
        if desc.is_null() {
            /*
             * Effectively frees the previous chunk allocation.
             * Index cannot be 0, so decrementing is always
             * valid.
             */
            chunk_pool.idx = chunk_pool.idx.wrapping_sub(1);
            return core::ptr::null_mut();
        }

        out = desc;

        (*desc).nr_free = SDT_TASK_ENTS_PER_CHUNK;
        (*desc).chunk = chunk;

        alloc_stats.chunk_allocs = alloc_stats.chunk_allocs.wrapping_add(1);

        out
    }
}

unsafe fn pool_set_size(pool: *mut sdt_pool, data_size: __u64, nr_pages: __u64) -> i32 {
    unsafe {
        if data_size % 8 != 0 {
            return -EINVAL;
        }

        if nr_pages == 0 {
            return -EINVAL;
        }

        (*pool).elem_size = data_size;
        (*pool).max_elems = (PAGE_SIZE.wrapping_mul(nr_pages)) / (*pool).elem_size;
        /* Populate the pool slab on the first allocation. */
        (*pool).idx = (*pool).max_elems;

        0
    }
}

/* Initialize both the base pool allocators and the root chunk of the index. */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn scx_alloc_init(alloc: *mut scx_allocator, data_size: __u64) -> i32 {
    unsafe {
        let min_chunk_size: size_t;
        let mut ret: i32;
        let mut data_size = data_size;

        // _Static_assert(sizeof(struct sdt_chunk) <= PAGE_SIZE, "chunk size must fit into a page");

        ret = pool_set_size(core::ptr::addr_of_mut!(chunk_pool), core::mem::size_of::<sdt_chunk>() as __u64, 1);
        if ret != 0 {
            return ret;
        }

        ret = pool_set_size(core::ptr::addr_of_mut!(desc_pool), core::mem::size_of::<sdt_desc>() as __u64, 1);
        if ret != 0 {
            return ret;
        }

        /* Wrap data into a descriptor and word align. */
        data_size = data_size.wrapping_add(core::mem::size_of::<sdt_data>() as __u64);
        data_size = round_up(data_size, 8);

        /*
         * Ensure we allocate large enough chunks from the arena to avoid excessive
         * internal fragmentation when turning chunks it into structs.
         */
        min_chunk_size = div_round_up(SDT_TASK_MIN_ELEM_PER_ALLOC.wrapping_mul(data_size), PAGE_SIZE) as size_t;
        ret = pool_set_size(core::ptr::addr_of_mut!((*alloc).pool), data_size, min_chunk_size as __u64);
        if ret != 0 {
            return ret;
        }

        bpf_spin_lock(core::ptr::addr_of_mut!(alloc_lock));
        (*alloc).root = scx_alloc_chunk();
        bpf_spin_unlock(core::ptr::addr_of_mut!(alloc_lock));
        if (*alloc).root.is_null() {
            return -ENOMEM;
        }

        0
    }
}

unsafe fn set_idx_state(desc: *mut sdt_desc_t, pos: __u64, state: bool) -> i32 {
    unsafe {
        let allocated: *mut __u64 = (*desc).allocated.as_mut_ptr();
        let bit: __u64;

        if pos >= SDT_TASK_ENTS_PER_CHUNK {
            return -EINVAL;
        }

        bit = (1 as __u64) << (pos % 64);

        if state {
            *allocated.add((pos / 64) as usize) |= bit;
        } else {
            *allocated.add((pos / 64) as usize) &= !bit;
        }

        0
    }
}

#[inline(never)]
unsafe fn mark_nodes_avail(lv_desc: [*mut sdt_desc_t; SDT_TASK_LEVELS], lv_pos: [__u64; SDT_TASK_LEVELS]) -> i32 {
    unsafe {
        let mut desc: *mut sdt_desc_t;
        let mut u: __u64;
        let level: __u64;
        let ret: i32;

        u = zero;
        while u < SDT_TASK_LEVELS as __u64 {
            level = SDT_TASK_LEVELS as __u64 - 1 - u;

            /* Only propagate upwards if we are the parent's only free chunk. */
            desc = lv_desc[level as usize];

            ret = set_idx_state(desc, lv_pos[level as usize], false);
            if ret != 0 {
                return ret;
            }

            (*desc).nr_free = (*desc).nr_free.wrapping_add(1);
            if (*desc).nr_free > 1 {
                return 0;
            }
            u = u.wrapping_add(1);
        }

        0
    }
}

/*
 * Free the allocated struct with the given index. Called with the
 * allocator lock taken.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn scx_alloc_free_idx(alloc: *mut scx_allocator, idx: __u64) -> i32 {
    unsafe {
        let mask: __u64 = (1 << SDT_TASK_ENTS_PER_PAGE_SHIFT) - 1;
        let mut lv_desc: [*mut sdt_desc_t; SDT_TASK_LEVELS] = [core::ptr::null_mut(); SDT_TASK_LEVELS];
        let desc_children: *mut *mut sdt_desc_t;
        let mut chunk: *mut sdt_chunk;
        let mut desc: *mut sdt_desc_t;
        let data: *mut sdt_data;
        let mut level: __u64;
        let shift: __u64;
        let mut pos: __u64;
        let mut lv_pos: [__u64; SDT_TASK_LEVELS] = [0; SDT_TASK_LEVELS];
        let ret: i32;
        let mut i: i32;

        if alloc.is_null() {
            return 0;
        }

        desc = (*alloc).root;
        if desc.is_null() {
            return -EINVAL;
        }

        /* To appease the verifier. */
        level = zero;
        while level < SDT_TASK_LEVELS as __u64 {
            lv_desc[level as usize] = core::ptr::null_mut();
            lv_pos[level as usize] = 0;
            level = level.wrapping_add(1);
        }

        /* Find the leaf node containing the index. */
        level = zero;
        while level < SDT_TASK_LEVELS as __u64 {
            shift = (SDT_TASK_LEVELS as __u64 - 1 - level) * SDT_TASK_ENTS_PER_PAGE_SHIFT;
            pos = (idx >> shift) & mask;

            lv_desc[level as usize] = desc;
            lv_pos[level as usize] = pos;

            if level == SDT_TASK_LEVELS as __u64 - 1 {
                break;
            }

            chunk = (*desc).chunk;

            desc_children = (*chunk).descs.as_mut_ptr();
            desc = *desc_children.add(pos as usize);

            if desc.is_null() {
                return -EINVAL;
            }
            level = level.wrapping_add(1);
        }

        chunk = (*desc).chunk;

        pos = idx & mask;
        data = (*chunk).data[pos as usize];
        if !data.is_null() {
            *data = sdt_data {
                tid: sdt_id {
                    genn: (*data).tid.genn.wrapping_add(1),
                },
                payload: [],
            };

            /* Zero out one word at a time. */
            i = zero as i32;
            while (i as __u64) < ((*alloc).pool.elem_size.wrapping_sub(core::mem::size_of::<sdt_data>() as __u64)) / 8 {
                (*data).payload[i as usize] = 0;
                i += 1;
            }
        }

        ret = mark_nodes_avail(lv_desc, lv_pos);
        if ret != 0 {
            return ret;
        }

        alloc_stats.active_allocs = alloc_stats.active_allocs.wrapping_sub(1);
        alloc_stats.free_ops = alloc_stats.free_ops.wrapping_add(1);

        0
    }
}

#[inline(always)]
fn ffs(mut word: __u64) -> i32 {
    let mut num: u32 = 0;

    if (word & 0xffffffff) == 0 {
        num += 32;
        word >>= 32;
    }

    if (word & 0xffff) == 0 {
        num += 16;
        word >>= 16;
    }

    if (word & 0xff) == 0 {
        num += 8;
        word >>= 8;
    }

    if (word & 0xf) == 0 {
        num += 4;
        word >>= 4;
    }

    if (word & 0x3) == 0 {
        num += 2;
        word >>= 2;
    }

    if (word & 0x1) == 0 {
        num += 1;
        word >>= 1;
    }

    num as i32
}

/* find the first empty slot */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_find_empty(desc: *mut sdt_desc_t) -> __u64 {
    unsafe {
        let mut freeslots: __u64;
        let mut i: __u64;

        i = 0;
        while i < SDT_TASK_CHUNK_BITMAP_U64S {
            freeslots = !(*desc).allocated[i as usize];
            if freeslots == 0 as __u64 {
                i = i.wrapping_add(1);
                continue;
            }

            return (i * 64).wrapping_add(ffs(freeslots) as __u64);
        }

        SDT_TASK_ENTS_PER_CHUNK
    }
}

/*
 * Find and return an available idx on the allocator.
 * Called with the task spinlock held.
 */
unsafe fn desc_find_empty(mut desc: *mut sdt_desc_t, idxp: *mut __u64) -> *mut sdt_desc_t {
    unsafe {
        let mut lv_desc: [*mut sdt_desc_t; SDT_TASK_LEVELS] = [core::ptr::null_mut(); SDT_TASK_LEVELS];
        let desc_children: *mut *mut sdt_desc_t;
        let chunk: *mut sdt_chunk;
        let tmp: *mut sdt_desc_t;
        let mut lv_pos: [__u64; SDT_TASK_LEVELS] = [0; SDT_TASK_LEVELS];
        let mut u: __u64;
        let mut pos: __u64;
        let mut level: __u64;
        let mut idx: __u64 = 0;
        let ret: i32;

        level = zero;
        while level < SDT_TASK_LEVELS as __u64 {
            pos = chunk_find_empty(desc);

            /* If we error out, something has gone very wrong. */
            if pos > SDT_TASK_ENTS_PER_CHUNK {
                return core::ptr::null_mut();
            }

            if pos == SDT_TASK_ENTS_PER_CHUNK {
                return core::ptr::null_mut();
            }

            idx <<= SDT_TASK_ENTS_PER_PAGE_SHIFT;
            idx |= pos;

            /* Log the levels to complete allocation. */
            lv_desc[level as usize] = desc;
            lv_pos[level as usize] = pos;

            /* The rest of the loop is for internal node traversal. */
            if level == SDT_TASK_LEVELS as __u64 - 1 {
                break;
            }

            /* Allocate an internal node if necessary. */
            chunk = (*desc).chunk;
            desc_children = (*chunk).descs.as_mut_ptr();

            desc = *desc_children.add(pos as usize);
            if desc.is_null() {
                desc = scx_alloc_chunk();
                if desc.is_null() {
                    return core::ptr::null_mut();
                }

                *desc_children.add(pos as usize) = desc;
            }
            level = level.wrapping_add(1);
        }

        /*
         * Finding the descriptor along with any internal node
         * allocations was successful. Update all levels with
         * the new allocation.
         */
        u = 0;
        while u < SDT_TASK_LEVELS as __u64 {
            level = SDT_TASK_LEVELS as __u64 - 1 - u;
            tmp = lv_desc[level as usize];

            ret = set_idx_state(tmp, lv_pos[level as usize], true);
            if ret != 0 {
                break;
            }

            (*tmp).nr_free = (*tmp).nr_free.wrapping_sub(1);
            if (*tmp).nr_free > 0 {
                break;
            }
            u = u.wrapping_add(1);
        }

        *idxp = idx;

        desc
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn scx_alloc(alloc: *mut scx_allocator) -> *mut core::ffi::c_void {
    unsafe {
        let mut data: *mut sdt_data = core::ptr::null_mut();
        let chunk: *mut sdt_chunk;
        let desc: *mut sdt_desc_t;
        let mut idx: __u64 = 0;
        let pos: __u64;

        if alloc.is_null() {
            return core::ptr::null_mut();
        }

        bpf_spin_lock(core::ptr::addr_of_mut!(alloc_lock));

        /* We unlock if we encounter an error in the function. */
        desc = desc_find_empty((*alloc).root, core::ptr::addr_of_mut!(idx));
        if desc.is_null() {
            bpf_spin_unlock(core::ptr::addr_of_mut!(alloc_lock));
            return core::ptr::null_mut();
        }

        chunk = (*desc).chunk;

        /* Populate the leaf node if necessary. */
        pos = idx & (SDT_TASK_ENTS_PER_CHUNK - 1);
        data = (*chunk).data[pos as usize];
        if data.is_null() {
            data = scx_alloc_from_pool(core::ptr::addr_of_mut!((*alloc).pool)) as *mut sdt_data;
            if data.is_null() {
                scx_alloc_free_idx(alloc, idx);
                bpf_spin_unlock(core::ptr::addr_of_mut!(alloc_lock));
                return core::ptr::null_mut();
            }
        }

        (*chunk).data[pos as usize] = data;

        /* The data counts as a chunk */
        alloc_stats.data_allocs = alloc_stats.data_allocs.wrapping_add(1);
        alloc_stats.alloc_ops = alloc_stats.alloc_ops.wrapping_add(1);
        alloc_stats.active_allocs = alloc_stats.active_allocs.wrapping_add(1);

        (*data).tid.idx = idx;

        bpf_spin_unlock(core::ptr::addr_of_mut!(alloc_lock));

        data as *mut core::ffi::c_void
    }
}

/*
 * Task BPF map entry recording the task's assigned ID and pointing to the data
 * area allocated in arena.
 */
#[repr(C)]
pub struct scx_task_map_val {
    pub tid: sdt_id,
    pub tptr: __u64,
    pub data: *mut sdt_data,
}

// Original BPF map declaration used BPF_MAP_TYPE_TASK_STORAGE, BPF_F_NO_PREALLOC,
// key int, and value struct scx_task_map_val.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut scx_task_map: TaskStorageMap = TaskStorageMap { _private: [] };

static mut scx_task_allocator: scx_allocator = scx_allocator {
    pool: sdt_pool {
        elem_size: 0,
        max_elems: 0,
        slab: core::ptr::null_mut(),
        idx: 0,
    },
    root: core::ptr::null_mut(),
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn scx_task_alloc(p: *mut task_struct) -> *mut core::ffi::c_void {
    unsafe {
        let mut data: *mut sdt_data = core::ptr::null_mut();
        let mval: *mut scx_task_map_val;

        mval = bpf_task_storage_get(
            core::ptr::addr_of_mut!(scx_task_map),
            p,
            core::ptr::null_mut(),
            BPF_LOCAL_STORAGE_GET_F_CREATE,
        ) as *mut scx_task_map_val;
        if mval.is_null() {
            return core::ptr::null_mut();
        }

        data = scx_alloc(core::ptr::addr_of_mut!(scx_task_allocator)) as *mut sdt_data;
        if data.is_null() {
            return core::ptr::null_mut();
        }

        (*mval).tid = (*data).tid;
        (*mval).tptr = p as __u64;
        (*mval).data = data;

        (*data).payload.as_mut_ptr() as *mut core::ffi::c_void
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn scx_task_init(data_size: __u64) -> i32 {
    unsafe { scx_alloc_init(core::ptr::addr_of_mut!(scx_task_allocator), data_size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn scx_task_data(p: *mut task_struct) -> *mut core::ffi::c_void {
    unsafe {
        let data: *mut sdt_data;
        let mval: *mut scx_task_map_val;

        scx_arena_subprog_init();

        mval = bpf_task_storage_get(core::ptr::addr_of_mut!(scx_task_map), p, core::ptr::null_mut(), 0)
            as *mut scx_task_map_val;
        if mval.is_null() {
            return core::ptr::null_mut();
        }

        data = (*mval).data;

        (*data).payload.as_ptr() as *mut core::ffi::c_void
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn scx_task_free(p: *mut task_struct) {
    unsafe {
        let mval: *mut scx_task_map_val;

        scx_arena_subprog_init();

        mval = bpf_task_storage_get(core::ptr::addr_of_mut!(scx_task_map), p, core::ptr::null_mut(), 0)
            as *mut scx_task_map_val;
        if mval.is_null() {
            return;
        }

        bpf_spin_lock(core::ptr::addr_of_mut!(alloc_lock));
        scx_alloc_free_idx(core::ptr::addr_of_mut!(scx_task_allocator), (*mval).tid.idx);
        bpf_spin_unlock(core::ptr::addr_of_mut!(alloc_lock));

        bpf_task_storage_delete(core::ptr::addr_of_mut!(scx_task_map), p);
    }
}

#[inline(always)]
unsafe fn scx_stat_global_update(stats: *mut scx_stats) {
    unsafe {
        stat_enqueue = stat_enqueue.wrapping_add((*stats).enqueue);
        stat_init = stat_init.wrapping_add((*stats).init);
        stat_exit = stat_exit.wrapping_add((*stats).exit);
        stat_select_idle_cpu = stat_select_idle_cpu.wrapping_add((*stats).select_idle_cpu);
        stat_select_busy_cpu = stat_select_busy_cpu.wrapping_add((*stats).select_busy_cpu);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sdt_select_cpu(p: *mut task_struct, prev_cpu: s32, wake_flags: u64) -> s32 {
    unsafe {
        let stats: *mut scx_stats;
        let mut is_idle: bool = false;
        let cpu: s32;

        stats = scx_task_data(p) as *mut scx_stats;
        if stats.is_null() {
            scx_bpf_error(c"%s: no stats for pid %d".as_ptr() as *const u8, c"sdt_select_cpu".as_ptr(), (*p).pid);
            return 0;
        }

        cpu = scx_bpf_select_cpu_dfl(p, prev_cpu, wake_flags, core::ptr::addr_of_mut!(is_idle));
        if is_idle {
            stat_inc_select_idle_cpu(stats);
            scx_bpf_dsq_insert(p, SCX_DSQ_LOCAL, SCX_SLICE_DFL, 0);
        } else {
            stat_inc_select_busy_cpu(stats);
        }

        cpu
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sdt_enqueue(p: *mut task_struct, enq_flags: u64) {
    unsafe {
        let stats: *mut scx_stats;

        stats = scx_task_data(p) as *mut scx_stats;
        if stats.is_null() {
            scx_bpf_error(c"%s: no stats for pid %d".as_ptr() as *const u8, c"sdt_enqueue".as_ptr(), (*p).pid);
            return;
        }

        stat_inc_enqueue(stats);

        scx_bpf_dsq_insert(p, SHARED_DSQ, SCX_SLICE_DFL, enq_flags);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sdt_dispatch(cpu: s32, prev: *mut task_struct) {
    unsafe {
        let _ = cpu;
        let _ = prev;
        scx_bpf_dsq_move_to_local(SHARED_DSQ, 0);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sdt_init_task(p: *mut task_struct, args: *mut scx_init_task_args) -> s32 {
    unsafe {
        let stats: *mut scx_stats;
        let _ = args;

        stats = scx_task_alloc(p) as *mut scx_stats;
        if stats.is_null() {
            scx_bpf_error(c"arena allocator out of memory".as_ptr() as *const u8);
            return -ENOMEM;
        }

        (*stats).pid = (*p).pid;

        stat_inc_init(stats);

        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sdt_exit_task(p: *mut task_struct, args: *mut scx_exit_task_args) {
    unsafe {
        let stats: *mut scx_stats;
        let _ = args;

        stats = scx_task_data(p) as *mut scx_stats;
        if stats.is_null() {
            scx_bpf_error(c"%s: no stats for pid %d".as_ptr() as *const u8, c"sdt_exit_task".as_ptr(), (*p).pid);
            return;
        }

        stat_inc_exit(stats);
        scx_stat_global_update(stats);

        scx_task_free(p);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sdt_init() -> s32 {
    unsafe {
        let mut ret: i32;

        ret = scx_task_init(core::mem::size_of::<scx_stats>() as __u64);
        if ret < 0 {
            scx_bpf_error(c"%s: failed with %d".as_ptr() as *const u8, c"sdt_init".as_ptr(), ret);
            return ret;
        }

        ret = scx_bpf_create_dsq(SHARED_DSQ, -1);
        if ret != 0 {
            scx_bpf_error(c"failed to create DSQ %d (%d)".as_ptr() as *const u8, SHARED_DSQ, ret);
            return ret;
        }

        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sdt_exit(ei: *mut scx_exit_info) {
    unsafe {
        UEI_RECORD(core::ptr::addr_of_mut!(uei), ei);
    }
}

// SCX_OPS_DEFINE(sdt_ops,
//                .select_cpu          = (void *)sdt_select_cpu,
//                .enqueue             = (void *)sdt_enqueue,
//                .dispatch            = (void *)sdt_dispatch,
//                .init_task           = (void *)sdt_init_task,
//                .exit_task           = (void *)sdt_exit_task,
//                .init                = (void *)sdt_init,
//                .exit                = (void *)sdt_exit,
//                .name                = "sdt");
