// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Facebook

// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>

type __u32 = u32;
type __u64 = u64;
type __s64 = i64;

const LOOP_BOUND: __u32 = 0xf;
const MAX_ENTRIES: __u32 = 8;
const HALF_ENTRIES: __u32 = MAX_ENTRIES >> 1;

const _: [(); 1] = [(); (MAX_ENTRIES < LOOP_BOUND) as usize];

type bpf_map_type = u32;

extern "C" {
    static BPF_MAP_TYPE_UNSPEC: bpf_map_type;
    static BPF_MAP_TYPE_HASH: bpf_map_type;
    static BPF_MAP_TYPE_ARRAY: bpf_map_type;
    static BPF_MAP_TYPE_PROG_ARRAY: bpf_map_type;
    static BPF_MAP_TYPE_PERF_EVENT_ARRAY: bpf_map_type;
    static BPF_MAP_TYPE_PERCPU_HASH: bpf_map_type;
    static BPF_MAP_TYPE_PERCPU_ARRAY: bpf_map_type;
    static BPF_MAP_TYPE_STACK_TRACE: bpf_map_type;
    static BPF_MAP_TYPE_CGROUP_ARRAY: bpf_map_type;
    static BPF_MAP_TYPE_LRU_HASH: bpf_map_type;
    static BPF_MAP_TYPE_LRU_PERCPU_HASH: bpf_map_type;
    static BPF_MAP_TYPE_LPM_TRIE: bpf_map_type;
    static BPF_MAP_TYPE_ARRAY_OF_MAPS: bpf_map_type;
    static BPF_MAP_TYPE_HASH_OF_MAPS: bpf_map_type;
    static BPF_MAP_TYPE_DEVMAP: bpf_map_type;
    static BPF_MAP_TYPE_SOCKMAP: bpf_map_type;
    static BPF_MAP_TYPE_CPUMAP: bpf_map_type;
    static BPF_MAP_TYPE_XSKMAP: bpf_map_type;
    static BPF_MAP_TYPE_SOCKHASH: bpf_map_type;
    static BPF_MAP_TYPE_CGROUP_STORAGE: bpf_map_type;
    static BPF_MAP_TYPE_REUSEPORT_SOCKARRAY: bpf_map_type;
    static BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE: bpf_map_type;
    static BPF_MAP_TYPE_QUEUE: bpf_map_type;
    static BPF_MAP_TYPE_STACK: bpf_map_type;
    static BPF_MAP_TYPE_SK_STORAGE: bpf_map_type;
    static BPF_MAP_TYPE_DEVMAP_HASH: bpf_map_type;
    static BPF_MAP_TYPE_RINGBUF: bpf_map_type;
    static BPF_F_NO_PREALLOC: __u32;
    static BPF_RINGBUF_HDR_SZ: usize;

    fn bpf_map_update_elem(map: *mut core::ffi::c_void, key: *const __u32, value: *const __u32, flags: u64) -> i32;
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_ringbuf_reserve(ringbuf: *mut core::ffi::c_void, size: u64, flags: u64) -> *mut core::ffi::c_void;
    fn bpf_ringbuf_discard(data: *mut core::ffi::c_void, flags: u64);
    fn bpf_map_sum_elem_count(map: *mut bpf_map) -> __s64;
}

#[no_mangle]
static mut g_map_type: bpf_map_type = 0;
#[no_mangle]
static mut g_line: __u32 = 0;
#[no_mangle]
static mut page_size: i32 = 0; /* userspace should set it */

macro_rules! verify {
    ($expr:expr) => {{
        g_line = line!() as __u32;
        if !($expr) {
            return 0;
        }
    }};
}

#[repr(C)]
struct bpf_map {
    map_type: bpf_map_type,
    key_size: __u32,
    value_size: __u32,
    max_entries: __u32,
    id: __u32,
}

unsafe fn check_bpf_map_fields(map: *mut bpf_map, key_size: __u32, value_size: __u32, max_entries: __u32) -> i32 {
    verify!((*map).map_type == g_map_type);
    verify!((*map).key_size == key_size);
    verify!((*map).value_size == value_size);
    verify!((*map).max_entries == max_entries);
    verify!((*map).id > 0);
    1
}

unsafe fn check_bpf_map_ptr(indirect: *mut bpf_map, direct: *mut bpf_map) -> i32 {
    verify!((*indirect).map_type == (*direct).map_type);
    verify!((*indirect).key_size == (*direct).key_size);
    verify!((*indirect).value_size == (*direct).value_size);
    verify!((*indirect).max_entries == (*direct).max_entries);
    verify!((*indirect).id == (*direct).id);
    1
}

unsafe fn check(indirect: *mut bpf_map, direct: *mut bpf_map, key_size: __u32, value_size: __u32, max_entries: __u32) -> i32 {
    verify!(check_bpf_map_ptr(indirect, direct) != 0);
    verify!(check_bpf_map_fields(indirect, key_size, value_size, max_entries) != 0);
    1
}

unsafe fn check_default(indirect: *mut bpf_map, direct: *mut bpf_map) -> i32 {
    verify!(check(indirect, direct, core::mem::size_of::<__u32>() as __u32, core::mem::size_of::<__u32>() as __u32, MAX_ENTRIES) != 0);
    1
}

#[inline(never)]
unsafe fn check_default_noinline(indirect: *mut bpf_map, direct: *mut bpf_map) -> i32 {
    verify!(check(indirect, direct, core::mem::size_of::<__u32>() as __u32, core::mem::size_of::<__u32>() as __u32, MAX_ENTRIES) != 0);
    1
}

#[repr(C)]
struct atomic_t {
    counter: i32,
}

#[repr(C)]
struct bpf_htab {
    map: bpf_map,
    count: atomic_t,
    n_buckets: __u32,
    elem_size: __u32,
}

#[repr(C)]
struct bpf_array {
    map: bpf_map,
    elem_size: __u32,
}

#[repr(C)]
struct bpf_stack_map {
    map: bpf_map,
}

#[repr(C)]
struct lpm_trie {
    map: bpf_map,
}

#[repr(C)]
struct bpf_lpm_trie_key_hdr {
    prefixlen: __u32,
}

#[repr(C)]
struct lpm_key {
    trie_key: bpf_lpm_trie_key_hdr,
    data: __u32,
}

const INNER_MAX_ENTRIES: __u32 = 1234;

#[repr(C)]
struct bpf_dtab {
    map: bpf_map,
}

#[repr(C)]
struct bpf_stab {
    map: bpf_map,
}

#[repr(C)]
struct bpf_cpu_map {
    map: bpf_map,
}

#[repr(C)]
struct xsk_map {
    map: bpf_map,
}

#[repr(C)]
struct bpf_shtab {
    map: bpf_map,
}

#[repr(C)]
struct bpf_cgroup_storage_key {
    _opaque: [u8; 0],
}

#[repr(C)]
struct bpf_cgroup_storage_map {
    map: bpf_map,
}

#[repr(C)]
struct reuseport_array {
    map: bpf_map,
}

#[repr(C)]
struct bpf_queue_stack {
    map: bpf_map,
}

#[repr(C)]
struct bpf_local_storage_map {
    map: bpf_map,
}

#[repr(C)]
struct bpf_ringbuf {
    consumer_pos: usize,
    producer_pos: usize,
}

#[repr(C)]
struct bpf_ringbuf_map {
    map: bpf_map,
    rb: *mut bpf_ringbuf,
}

// SEC(".maps") map definitions translated from libbpf declaration macros.
#[no_mangle] static mut m_hash: [u8; 0] = [];
#[no_mangle] static mut m_array: [u8; 0] = [];
#[no_mangle] static mut m_prog_array: [u8; 0] = [];
#[no_mangle] static mut m_perf_event_array: [u8; 0] = [];
#[no_mangle] static mut m_percpu_hash: [u8; 0] = [];
#[no_mangle] static mut m_percpu_array: [u8; 0] = [];
#[no_mangle] static mut m_stack_trace: [u8; 0] = [];
#[no_mangle] static mut m_cgroup_array: [u8; 0] = [];
#[no_mangle] static mut m_lru_hash: [u8; 0] = [];
#[no_mangle] static mut m_lru_percpu_hash: [u8; 0] = [];
#[no_mangle] static mut m_lpm_trie: [u8; 0] = [];
#[no_mangle] static mut inner_map: [u8; 0] = [];
#[no_mangle] static mut m_array_of_maps: [usize; 9] = [0; 9];
#[no_mangle] static mut m_hash_of_maps: [usize; 8] = [0; 8];
#[no_mangle] static mut m_devmap: [u8; 0] = [];
#[no_mangle] static mut m_sockmap: [u8; 0] = [];
#[no_mangle] static mut m_cpumap: [u8; 0] = [];
#[no_mangle] static mut m_xskmap: [u8; 0] = [];
#[no_mangle] static mut m_sockhash: [u8; 0] = [];
#[no_mangle] static mut m_cgroup_storage: [u8; 0] = [];
#[no_mangle] static mut m_reuseport_sockarray: [u8; 0] = [];
#[no_mangle] static mut m_percpu_cgroup_storage: [u8; 0] = [];
#[no_mangle] static mut m_queue: [u8; 0] = [];
#[no_mangle] static mut m_stack: [u8; 0] = [];
#[no_mangle] static mut m_sk_storage: [u8; 0] = [];
#[no_mangle] static mut m_devmap_hash: [u8; 0] = [];
#[no_mangle] static mut m_ringbuf: [u8; 0] = [];

unsafe fn check_hash() -> i32 {
    let hash = &mut m_hash as *mut _ as *mut bpf_htab;
    let map = &mut m_hash as *mut _ as *mut bpf_map;
    let mut i: i32;

    verify!(check_default_noinline(&mut (*hash).map, map) != 0);
    verify!((*hash).n_buckets == MAX_ENTRIES);
    verify!((*hash).elem_size == 64);
    verify!((*hash).count.counter == 0);
    verify!(bpf_map_sum_elem_count(map) == 0);

    i = 0;
    while i < HALF_ENTRIES as i32 {
        let key: __u32 = i as __u32;
        let val: __u32 = 1;
        if bpf_map_update_elem(hash as *mut core::ffi::c_void, &key, &val, 0) != 0 {
            return 0;
        }
        i += 1;
    }
    verify!((*hash).count.counter == HALF_ENTRIES as i32);
    verify!(bpf_map_sum_elem_count(map) == HALF_ENTRIES as __s64);
    1
}

unsafe fn check_array() -> i32 {
    let array = &mut m_array as *mut _ as *mut bpf_array;
    let map = &mut m_array as *mut _ as *mut bpf_map;
    let mut i: i32 = 0;
    let mut n_lookups: i32 = 0;
    let mut n_keys: i32 = 0;

    verify!(check_default(&mut (*array).map, map) != 0);
    verify!((*array).elem_size == 8);
    while i < (*array).map.max_entries as i32 && i < LOOP_BOUND as i32 {
        let key: __u32 = i as __u32;
        let val = bpf_map_lookup_elem(array as *mut core::ffi::c_void, &key as *const _ as *const core::ffi::c_void) as *mut __u32;
        n_lookups += 1;
        if !val.is_null() {
            n_keys += 1;
        }
        i += 1;
    }
    verify!(n_lookups == MAX_ENTRIES as i32);
    verify!(n_keys == MAX_ENTRIES as i32);
    1
}

unsafe fn check_prog_array() -> i32 { let prog_array = &mut m_prog_array as *mut _ as *mut bpf_array; let map = &mut m_prog_array as *mut _ as *mut bpf_map; verify!(check_default(&mut (*prog_array).map, map) != 0); 1 }
unsafe fn check_perf_event_array() -> i32 { let perf_event_array = &mut m_perf_event_array as *mut _ as *mut bpf_array; let map = &mut m_perf_event_array as *mut _ as *mut bpf_map; verify!(check_default(&mut (*perf_event_array).map, map) != 0); 1 }
unsafe fn check_percpu_hash() -> i32 { let percpu_hash = &mut m_percpu_hash as *mut _ as *mut bpf_htab; let map = &mut m_percpu_hash as *mut _ as *mut bpf_map; verify!(check_default(&mut (*percpu_hash).map, map) != 0); 1 }
unsafe fn check_percpu_array() -> i32 { let percpu_array = &mut m_percpu_array as *mut _ as *mut bpf_array; let map = &mut m_percpu_array as *mut _ as *mut bpf_map; verify!(check_default(&mut (*percpu_array).map, map) != 0); 1 }
unsafe fn check_stack_trace() -> i32 { let stack_trace = &mut m_stack_trace as *mut _ as *mut bpf_stack_map; let map = &mut m_stack_trace as *mut _ as *mut bpf_map; verify!(check(&mut (*stack_trace).map, map, core::mem::size_of::<__u32>() as __u32, core::mem::size_of::<__u64>() as __u32, MAX_ENTRIES) != 0); 1 }
unsafe fn check_cgroup_array() -> i32 { let cgroup_array = &mut m_cgroup_array as *mut _ as *mut bpf_array; let map = &mut m_cgroup_array as *mut _ as *mut bpf_map; verify!(check_default(&mut (*cgroup_array).map, map) != 0); 1 }
unsafe fn check_lru_hash() -> i32 { let lru_hash = &mut m_lru_hash as *mut _ as *mut bpf_htab; let map = &mut m_lru_hash as *mut _ as *mut bpf_map; verify!(check_default(&mut (*lru_hash).map, map) != 0); 1 }
unsafe fn check_lru_percpu_hash() -> i32 { let lru_percpu_hash = &mut m_lru_percpu_hash as *mut _ as *mut bpf_htab; let map = &mut m_lru_percpu_hash as *mut _ as *mut bpf_map; verify!(check_default(&mut (*lru_percpu_hash).map, map) != 0); 1 }
unsafe fn check_lpm_trie() -> i32 { let lpm_trie = &mut m_lpm_trie as *mut _ as *mut lpm_trie; let map = &mut m_lpm_trie as *mut _ as *mut bpf_map; verify!(check(&mut (*lpm_trie).map, map, core::mem::size_of::<lpm_key>() as __u32, core::mem::size_of::<__u32>() as __u32, MAX_ENTRIES) != 0); 1 }

unsafe fn check_array_of_maps() -> i32 {
    let array_of_maps = &mut m_array_of_maps as *mut _ as *mut bpf_array;
    let map = &mut m_array_of_maps as *mut _ as *mut bpf_map;
    let mut key: i32 = 0;
    verify!(check_default(&mut (*array_of_maps).map, map) != 0);
    let inner_map = bpf_map_lookup_elem(array_of_maps as *mut core::ffi::c_void, &mut key as *mut _ as *const core::ffi::c_void) as *mut bpf_array;
    verify!(!inner_map.is_null());
    verify!((*inner_map).map.max_entries == INNER_MAX_ENTRIES);
    1
}

unsafe fn check_hash_of_maps() -> i32 {
    let hash_of_maps = &mut m_hash_of_maps as *mut _ as *mut bpf_htab;
    let map = &mut m_hash_of_maps as *mut _ as *mut bpf_map;
    let mut key: i32 = 2;
    verify!(check_default(&mut (*hash_of_maps).map, map) != 0);
    let inner_map = bpf_map_lookup_elem(hash_of_maps as *mut core::ffi::c_void, &mut key as *mut _ as *const core::ffi::c_void) as *mut bpf_htab;
    verify!(!inner_map.is_null());
    verify!((*inner_map).map.max_entries == INNER_MAX_ENTRIES);
    1
}

unsafe fn check_devmap() -> i32 { let devmap = &mut m_devmap as *mut _ as *mut bpf_dtab; let map = &mut m_devmap as *mut _ as *mut bpf_map; verify!(check_default(&mut (*devmap).map, map) != 0); 1 }
unsafe fn check_sockmap() -> i32 { let sockmap = &mut m_sockmap as *mut _ as *mut bpf_stab; let map = &mut m_sockmap as *mut _ as *mut bpf_map; verify!(check_default(&mut (*sockmap).map, map) != 0); 1 }
unsafe fn check_cpumap() -> i32 { let cpumap = &mut m_cpumap as *mut _ as *mut bpf_cpu_map; let map = &mut m_cpumap as *mut _ as *mut bpf_map; verify!(check_default(&mut (*cpumap).map, map) != 0); 1 }
unsafe fn check_xskmap() -> i32 { let xskmap = &mut m_xskmap as *mut _ as *mut xsk_map; let map = &mut m_xskmap as *mut _ as *mut bpf_map; verify!(check_default(&mut (*xskmap).map, map) != 0); 1 }
unsafe fn check_sockhash() -> i32 { let sockhash = &mut m_sockhash as *mut _ as *mut bpf_shtab; let map = &mut m_sockhash as *mut _ as *mut bpf_map; verify!(check_default(&mut (*sockhash).map, map) != 0); 1 }
unsafe fn check_cgroup_storage() -> i32 { let cgroup_storage = &mut m_cgroup_storage as *mut _ as *mut bpf_cgroup_storage_map; let map = &mut m_cgroup_storage as *mut _ as *mut bpf_map; verify!(check(&mut (*cgroup_storage).map, map, core::mem::size_of::<bpf_cgroup_storage_key>() as __u32, core::mem::size_of::<__u32>() as __u32, 0) != 0); 1 }
unsafe fn check_reuseport_sockarray() -> i32 { let reuseport_sockarray = &mut m_reuseport_sockarray as *mut _ as *mut reuseport_array; let map = &mut m_reuseport_sockarray as *mut _ as *mut bpf_map; verify!(check_default(&mut (*reuseport_sockarray).map, map) != 0); 1 }
unsafe fn check_percpu_cgroup_storage() -> i32 { let percpu_cgroup_storage = &mut m_percpu_cgroup_storage as *mut _ as *mut bpf_cgroup_storage_map; let map = &mut m_percpu_cgroup_storage as *mut _ as *mut bpf_map; verify!(check(&mut (*percpu_cgroup_storage).map, map, core::mem::size_of::<bpf_cgroup_storage_key>() as __u32, core::mem::size_of::<__u32>() as __u32, 0) != 0); 1 }
unsafe fn check_queue() -> i32 { let queue = &mut m_queue as *mut _ as *mut bpf_queue_stack; let map = &mut m_queue as *mut _ as *mut bpf_map; verify!(check(&mut (*queue).map, map, 0, core::mem::size_of::<__u32>() as __u32, MAX_ENTRIES) != 0); 1 }
unsafe fn check_stack() -> i32 { let stack = &mut m_stack as *mut _ as *mut bpf_queue_stack; let map = &mut m_stack as *mut _ as *mut bpf_map; verify!(check(&mut (*stack).map, map, 0, core::mem::size_of::<__u32>() as __u32, MAX_ENTRIES) != 0); 1 }
unsafe fn check_sk_storage() -> i32 { let sk_storage = &mut m_sk_storage as *mut _ as *mut bpf_local_storage_map; let map = &mut m_sk_storage as *mut _ as *mut bpf_map; verify!(check(&mut (*sk_storage).map, map, core::mem::size_of::<__u32>() as __u32, core::mem::size_of::<__u32>() as __u32, 0) != 0); 1 }
unsafe fn check_devmap_hash() -> i32 { let devmap_hash = &mut m_devmap_hash as *mut _ as *mut bpf_dtab; let map = &mut m_devmap_hash as *mut _ as *mut bpf_map; verify!(check_default(&mut (*devmap_hash).map, map) != 0); 1 }

unsafe fn check_ringbuf() -> i32 {
    let ringbuf = &mut m_ringbuf as *mut _ as *mut bpf_ringbuf_map;
    let map = &mut m_ringbuf as *mut _ as *mut bpf_map;
    let rb: *mut bpf_ringbuf;
    let ptr: *mut core::ffi::c_void;

    verify!(check(&mut (*ringbuf).map, map, 0, 0, page_size as __u32) != 0);
    ptr = bpf_ringbuf_reserve(&mut m_ringbuf as *mut _ as *mut core::ffi::c_void, 128, 0);
    verify!(!ptr.is_null());
    bpf_ringbuf_discard(ptr, 0);
    rb = (*ringbuf).rb;
    verify!(!rb.is_null());
    verify!((*rb).consumer_pos == 0);
    verify!((*rb).producer_pos == 128 + BPF_RINGBUF_HDR_SZ);
    1
}

#[no_mangle]
pub unsafe extern "C" fn cg_skb(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    g_map_type = BPF_MAP_TYPE_HASH; if check_hash() == 0 { return 0; }
    g_map_type = BPF_MAP_TYPE_ARRAY; if check_array() == 0 { return 0; }
    g_map_type = BPF_MAP_TYPE_PROG_ARRAY; if check_prog_array() == 0 { return 0; }
    g_map_type = BPF_MAP_TYPE_PERF_EVENT_ARRAY; if check_perf_event_array() == 0 { return 0; }
    g_map_type = BPF_MAP_TYPE_PERCPU_HASH; if check_percpu_hash() == 0 { return 0; }
    g_map_type = BPF_MAP_TYPE_PERCPU_ARRAY; if check_percpu_array() == 0 { return 0; }
    g_map_type = BPF_MAP_TYPE_STACK_TRACE; if check_stack_trace() == 0 { return 0; }
    g_map_type = BPF_MAP_TYPE_CGROUP_ARRAY; if check_cgroup_array() == 0 { return 0; }
    g_map_type = BPF_MAP_TYPE_LRU_HASH; if check_lru_hash() == 0 { return 0; }
    g_map_type = BPF_MAP_TYPE_LRU_PERCPU_HASH; if check_lru_percpu_hash() == 0 { return 0; }
    g_map_type = BPF_MAP_TYPE_LPM_TRIE; if check_lpm_trie() == 0 { return 0; }
    g_map_type = BPF_MAP_TYPE_ARRAY_OF_MAPS; if check_array_of_maps() == 0 { return 0; }
    g_map_type = BPF_MAP_TYPE_HASH_OF_MAPS; if check_hash_of_maps() == 0 { return 0; }
    g_map_type = BPF_MAP_TYPE_DEVMAP; if check_devmap() == 0 { return 0; }
    g_map_type = BPF_MAP_TYPE_SOCKMAP; if check_sockmap() == 0 { return 0; }
    g_map_type = BPF_MAP_TYPE_CPUMAP; if check_cpumap() == 0 { return 0; }
    g_map_type = BPF_MAP_TYPE_XSKMAP; if check_xskmap() == 0 { return 0; }
    g_map_type = BPF_MAP_TYPE_SOCKHASH; if check_sockhash() == 0 { return 0; }
    g_map_type = BPF_MAP_TYPE_CGROUP_STORAGE; if check_cgroup_storage() == 0 { return 0; }
    g_map_type = BPF_MAP_TYPE_REUSEPORT_SOCKARRAY; if check_reuseport_sockarray() == 0 { return 0; }
    g_map_type = BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE; if check_percpu_cgroup_storage() == 0 { return 0; }
    g_map_type = BPF_MAP_TYPE_QUEUE; if check_queue() == 0 { return 0; }
    g_map_type = BPF_MAP_TYPE_STACK; if check_stack() == 0 { return 0; }
    g_map_type = BPF_MAP_TYPE_SK_STORAGE; if check_sk_storage() == 0 { return 0; }
    g_map_type = BPF_MAP_TYPE_DEVMAP_HASH; if check_devmap_hash() == 0 { return 0; }
    g_map_type = BPF_MAP_TYPE_RINGBUF; if check_ringbuf() == 0 { return 0; }
    1
}

#[no_mangle]
#[link_section = "license"]
static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
