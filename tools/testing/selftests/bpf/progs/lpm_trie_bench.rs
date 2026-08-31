// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Cloudflare */

// C dependencies: vmlinux.h, errno.h, bpf/bpf_tracing.h, bpf/bpf_helpers.h,
// bpf/bpf_core_read.h, bpf_misc.h, bpf_atomic.h, progs/lpm_trie.h.

pub const BPF_OBJ_NAME_LEN: u32 = 16;
pub const MAX_ENTRIES: u32 = 100000000;
pub const NR_LOOPS: u32 = 10000;

pub const ENOENT: i32 = 2;

extern "C" {
    pub static mut trie_map: BpfMapDef;

    fn bpf_ktime_get_ns() -> u64;
    fn bpf_get_prandom_u32() -> u32;
    fn bpf_strncmp(s1: *const i8, s1_sz: u32, s2: *const i8) -> i32;
    fn bpf_map_lookup_elem(map: *mut BpfMapDef, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_map_update_elem(
        map: *mut BpfMapDef,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> i32;
    fn bpf_map_delete_elem(map: *mut BpfMapDef, key: *const core::ffi::c_void) -> i32;
    fn bpf_loop(
        nr_loops: u32,
        callback_fn: Option<unsafe extern "C" fn(u32, *mut core::ffi::c_void) -> i32>,
        callback_ctx: *mut core::ffi::c_void,
        flags: u64,
    ) -> i32;
    fn bpf_printk(fmt: *const i8, ...) -> i32;
}

#[repr(C)]
pub struct WorkStruct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct BpfMap {
    pub map_type: u32,
    pub name: [i8; BPF_OBJ_NAME_LEN as usize],
    pub work: WorkStruct,
}

#[repr(C)]
pub struct BpfMapDef {
    _private: [u8; 0],
}

#[repr(C)]
pub struct trie_key {
    pub prefixlen: u32,
    pub data: u32,
}

pub const BPF_MAP_TYPE_LPM_TRIE: u32 = 11;
pub const BPF_F_NO_PREALLOC: u32 = 1;
pub const BPF_NOEXIST: u64 = 1;
pub const BPF_EXIST: u64 = 2;

pub const LPM_BENCH_SUCCESS: i32 = 0;
pub const LPM_BENCH_REINIT_MAP: i32 = 1;

pub const LPM_OP_NOOP: u8 = 0;
pub const LPM_OP_BASELINE: u8 = 1;
pub const LPM_OP_LOOKUP: u8 = 2;
pub const LPM_OP_INSERT: u8 = 3;
pub const LPM_OP_UPDATE: u8 = 4;
pub const LPM_OP_DELETE: u8 = 5;

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

/* Filled by userspace. See fill_map() in bench_lpm_trie_map.c */
// Original C declares trie_map with BPF map-definition macros:
// type BPF_MAP_TYPE_LPM_TRIE, key struct trie_key, value __u32,
// map_flags BPF_F_NO_PREALLOC, max_entries MAX_ENTRIES.

#[no_mangle]
pub static mut hits: i64 = 0;
#[no_mangle]
pub static mut duration_ns: i64 = 0;

/* Configured from userspace */
#[no_mangle]
pub static mut nr_entries: u32 = 0;
#[no_mangle]
pub static mut prefixlen: u32 = 0;
#[no_mangle]
pub static mut random: bool = false;
#[no_mangle]
pub static mut op: u8 = 0;

static mut latency_free_start: u64 = 0;

unsafe fn container_of_work_to_bpf_map(work: *mut WorkStruct) -> *mut BpfMap {
    // Rust translation of container_of(work, struct bpf_map, work).
    (work as *mut u8).sub(core::mem::offset_of!(BpfMap, work)) as *mut BpfMap
}

unsafe fn bpf_core_read_u32(src: *const u32) -> u32 {
    core::ptr::read_volatile(src)
}

unsafe fn bpf_core_read_str_into(dst: *mut [i8; BPF_OBJ_NAME_LEN as usize], src: *const [i8; BPF_OBJ_NAME_LEN as usize]) {
    core::ptr::copy_nonoverlapping(src as *const i8, (*dst).as_mut_ptr(), BPF_OBJ_NAME_LEN as usize);
}

unsafe fn sync_add_and_fetch_i64(ptr: *mut i64, val: i64) -> i64 {
    let old = core::ptr::read_volatile(ptr);
    let new = old.wrapping_add(val);
    core::ptr::write_volatile(ptr, new);
    new
}

#[no_mangle]
#[link_section = "fentry/bpf_map_free_deferred"]
pub unsafe extern "C" fn trie_free_entry(work: *mut WorkStruct) -> i32 {
    let map: *mut BpfMap = container_of_work_to_bpf_map(work);
    let mut name: [i8; BPF_OBJ_NAME_LEN as usize] = [0; BPF_OBJ_NAME_LEN as usize];
    let map_type: u32;

    map_type = bpf_core_read_u32(core::ptr::addr_of!((*map).map_type));
    if map_type != BPF_MAP_TYPE_LPM_TRIE {
        return 0;
    }

    /*
     * Ideally we'd have access to the map ID but that's already
     * freed before we enter trie_free().
     */
    bpf_core_read_str_into(&mut name, core::ptr::addr_of!((*map).name));
    if bpf_strncmp(name.as_ptr(), BPF_OBJ_NAME_LEN, b"trie_free_map\0".as_ptr() as *const i8) != 0 {
        return 0;
    }

    latency_free_start = bpf_ktime_get_ns();

    0
}

#[no_mangle]
#[link_section = "fexit/bpf_map_free_deferred"]
pub unsafe extern "C" fn trie_free_exit(_work: *mut WorkStruct) -> i32 {
    let val: u64;

    if latency_free_start == 0 {
        return 0;
    }

    val = bpf_ktime_get_ns().wrapping_sub(latency_free_start);
    latency_free_start = 0;

    sync_add_and_fetch_i64(core::ptr::addr_of_mut!(duration_ns), val as i64);
    sync_add_and_fetch_i64(core::ptr::addr_of_mut!(hits), 1);

    0
}

static mut cur_key: u32 = 0;

#[inline(always)]
unsafe fn generate_key(key: *mut trie_key) {
    (*key).prefixlen = prefixlen;

    if random {
        (*key).data = bpf_get_prandom_u32() % nr_entries;
    } else {
        let old = cur_key;
        cur_key = cur_key.wrapping_add(1);
        (*key).data = old % nr_entries;
    }
}

unsafe extern "C" fn noop(_index: u32, _unused: *mut core::ffi::c_void) -> i32 {
    0
}

unsafe extern "C" fn baseline(_index: u32, _unused: *mut core::ffi::c_void) -> i32 {
    let mut key: trie_key = core::mem::zeroed();
    let mut blackbox: u32 = 0;

    generate_key(&mut key);
    /* Avoid compiler optimizing out the modulo */
    core::ptr::read_volatile(&blackbox);
    blackbox = core::ptr::read_volatile(core::ptr::addr_of!(key.data));
    let _ = blackbox;

    0
}

unsafe extern "C" fn lookup(_index: u32, retval: *mut core::ffi::c_void) -> i32 {
    let mut key: trie_key = core::mem::zeroed();
    let retval = retval as *mut i32;

    generate_key(&mut key);
    if bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(trie_map),
        &key as *const _ as *const core::ffi::c_void,
    )
    .is_null()
    {
        *retval = -ENOENT;
        return 1;
    }

    0
}

unsafe extern "C" fn insert(_index: u32, retval: *mut core::ffi::c_void) -> i32 {
    let mut key: trie_key = core::mem::zeroed();
    let val: u32 = 1;
    let err: i32;
    let retval = retval as *mut i32;

    generate_key(&mut key);
    err = bpf_map_update_elem(
        core::ptr::addr_of_mut!(trie_map),
        &key as *const _ as *const core::ffi::c_void,
        &val as *const _ as *const core::ffi::c_void,
        BPF_NOEXIST,
    );
    if err != 0 {
        *retval = err;
        return 1;
    }

    /* Is this the last entry? */
    if key.data == nr_entries.wrapping_sub(1) {
        /* For atomicity concerns, see the comment in delete() */
        *retval = LPM_BENCH_REINIT_MAP;
        return 1;
    }

    0
}

unsafe extern "C" fn update(_index: u32, retval: *mut core::ffi::c_void) -> i32 {
    let mut key: trie_key = core::mem::zeroed();
    let val: u32 = 1;
    let err: i32;
    let retval = retval as *mut i32;

    generate_key(&mut key);
    err = bpf_map_update_elem(
        core::ptr::addr_of_mut!(trie_map),
        &key as *const _ as *const core::ffi::c_void,
        &val as *const _ as *const core::ffi::c_void,
        BPF_EXIST,
    );
    if err != 0 {
        *retval = err;
        return 1;
    }

    0
}

unsafe extern "C" fn delete(_index: u32, retval: *mut core::ffi::c_void) -> i32 {
    let mut key: trie_key = core::mem::zeroed();
    let err: i32;
    let retval = retval as *mut i32;

    generate_key(&mut key);
    err = bpf_map_delete_elem(
        core::ptr::addr_of_mut!(trie_map),
        &key as *const _ as *const core::ffi::c_void,
    );
    if err != 0 {
        *retval = err;
        return 1;
    }

    /* Do we need to refill the map? */
    if key.data == nr_entries.wrapping_sub(1) {
        /*
         * Atomicity isn't required because DELETE only supports
         * one producer running concurrently. What we need is a
         * way to track how many entries have been deleted from
         * the trie between consecutive invocations of the BPF
         * prog because a single bpf_loop() call might not
         * delete all entries, e.g. when NR_LOOPS < nr_entries.
         */
        *retval = LPM_BENCH_REINIT_MAP;
        return 1;
    }

    0
}

#[no_mangle]
#[link_section = "xdp"]
pub unsafe extern "C" fn run_bench() -> i32 {
    let mut err: i32 = LPM_BENCH_SUCCESS;
    let start: u64;
    let delta: u64;
    let loops: i32;

    start = bpf_ktime_get_ns();

    match op {
        LPM_OP_NOOP => {
            loops = bpf_loop(NR_LOOPS, Some(noop), core::ptr::null_mut(), 0);
        }
        LPM_OP_BASELINE => {
            loops = bpf_loop(NR_LOOPS, Some(baseline), core::ptr::null_mut(), 0);
        }
        LPM_OP_LOOKUP => {
            loops = bpf_loop(NR_LOOPS, Some(lookup), &mut err as *mut _ as *mut core::ffi::c_void, 0);
        }
        LPM_OP_INSERT => {
            loops = bpf_loop(NR_LOOPS, Some(insert), &mut err as *mut _ as *mut core::ffi::c_void, 0);
        }
        LPM_OP_UPDATE => {
            loops = bpf_loop(NR_LOOPS, Some(update), &mut err as *mut _ as *mut core::ffi::c_void, 0);
        }
        LPM_OP_DELETE => {
            loops = bpf_loop(NR_LOOPS, Some(delete), &mut err as *mut _ as *mut core::ffi::c_void, 0);
        }
        _ => {
            bpf_printk(b"invalid benchmark operation\n\0".as_ptr() as *const i8);
            return -1;
        }
    }

    delta = bpf_ktime_get_ns().wrapping_sub(start);

    sync_add_and_fetch_i64(core::ptr::addr_of_mut!(duration_ns), delta as i64);
    sync_add_and_fetch_i64(core::ptr::addr_of_mut!(hits), loops as i64);

    err
}
