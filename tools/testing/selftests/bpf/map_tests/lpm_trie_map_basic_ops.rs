// SPDX-License-Identifier: GPL-2.0
/*
 * Randomized tests for eBPF longest-prefix-match maps
 *
 * This program runs randomized tests against the lpm-bpf-map. It implements a
 * "Trivial Longest Prefix Match" (tlpm) based on simple, linear, singly linked
 * lists. The implementation should be pretty straightforward.
 *
 * Based on tlpm, this inserts randomized data into bpf-lpm-maps and verifies
 * the trie-based bpf-map implementation behaves the same way as tlpm.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type size_t = usize;
type pthread_t = c_ulong;
type __u32 = u32;
type __u64 = u64;

const BPF_MAP_TYPE_LPM_TRIE: c_int = 11;
const BPF_F_NO_PREALLOC: c_uint = 1;
const BPF_ANY: u64 = 0;
const BPF_NOEXIST: u64 = 1;
const BPF_EXIST: u64 = 2;
const BPF_F_LOCK: u64 = 4;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const EEXIST: c_int = 17;
const EINVAL: c_int = 22;
const ENOSPC: c_int = 28;
const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const MAX_TEST_KEYS: usize = 4;

#[repr(C)]
struct bpf_lpm_trie_key_hdr {
    prefixlen: __u32,
}

#[repr(C)]
struct bpf_lpm_trie_key_u8 {
    prefixlen: __u32,
    data: [u8; 0],
}

#[repr(C)]
struct bpf_map_create_opts {
    sz: size_t,
    map_flags: __u32,
}

#[repr(C)]
struct tlpm_node {
    next: *mut tlpm_node,
    n_bits: size_t,
    key: [u8; 0],
}

#[repr(C)]
union lpm_trie_bytes_key_hdr {
    hdr: bpf_lpm_trie_key_hdr,
    prefixlen: __u32,
}

#[repr(C)]
struct lpm_trie_bytes_key {
    u: lpm_trie_bytes_key_hdr,
    data: [u8; 8],
}

#[repr(C)]
union lpm_trie_int_key_hdr {
    hdr: bpf_lpm_trie_key_hdr,
    prefixlen: __u32,
}

#[repr(C)]
struct lpm_trie_int_key {
    u: lpm_trie_int_key_hdr,
    data: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct lpm_mt_key {
    prefixlen: __u32,
    data: __u32,
}

#[repr(C)]
struct lpm_mt_test_info {
    cmd: c_int, /* 0: update, 1: delete, 2: lookup, 3: get_next_key */
    iter: c_int,
    map_fd: c_int,
    key: [lpm_mt_key; MAX_TEST_KEYS],
}

unsafe extern "C" {
    static mut errno: c_int;

    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn alloca(size: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn qsort(base: *mut c_void, nmemb: size_t, size: size_t,
             compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>);
    fn rand() -> c_int;
    fn srand(seed: c_uint);
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn pthread_create(thread: *mut pthread_t, attr: *const c_void,
                      start_routine: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
                      arg: *mut c_void) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_exit(retval: *mut c_void) -> !;

    fn bpf_map_create(map_type: c_int, map_name: *const c_char, key_size: __u32,
                      value_size: __u32, max_entries: __u32,
                      opts: *const bpf_map_create_opts) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void,
                           flags: u64) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;
    fn bpf_map_get_next_key(fd: c_int, key: *const c_void, next_key: *mut c_void) -> c_int;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! CHECK {
    ($cond:expr, $name:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {{
        if $cond {
            printf(cstr!("%s: "), cstr!($name));
            printf(cstr!($fmt) $(, $arg)*);
        }
    }};
}

unsafe fn key_data(key: *mut bpf_lpm_trie_key_u8) -> *mut u8 {
    (key as *mut u8).add(size_of::<bpf_lpm_trie_key_u8>())
}

unsafe fn be32toh(x: u32) -> u32 {
    u32::from_be(x)
}

unsafe fn htobe32(x: u32) -> u32 {
    x.to_be()
}

unsafe fn tlpm_match(mut list: *mut tlpm_node, key: *const u8, n_bits: size_t) -> *mut tlpm_node {
    let mut best: *mut tlpm_node = ptr::null_mut();
    while !list.is_null() {
        let mut i: size_t = 0;
        while i < n_bits && i < (*list).n_bits {
            if ((*key.add(i / 8) & (1 << (7 - i % 8))) !=
                (*((*list).key.as_ptr().add(i / 8)) & (1 << (7 - i % 8)))) {
                break;
            }
            i += 1;
        }

        if i >= (*list).n_bits {
            if best.is_null() || i > (*best).n_bits {
                best = list;
            }
        }
        list = (*list).next;
    }
    best
}

unsafe fn tlpm_add(list: *mut tlpm_node, key: *const u8, n_bits: size_t) -> *mut tlpm_node {
    let n = (n_bits + 7) / 8;

    /* 'overwrite' an equivalent entry if one already exists */
    let node = tlpm_match(list, key, n_bits);
    if !node.is_null() && (*node).n_bits == n_bits {
        memcpy((*node).key.as_mut_ptr() as *mut c_void, key as *const c_void, n);
        return list;
    }

    /* add new entry with @key/@n_bits to @list and return new head */
    let node = malloc(size_of::<tlpm_node>() + n) as *mut tlpm_node;
    assert!(!node.is_null());

    (*node).next = list;
    (*node).n_bits = n_bits;
    memcpy((*node).key.as_mut_ptr() as *mut c_void, key as *const c_void, n);

    node
}

unsafe fn tlpm_clear(mut list: *mut tlpm_node) {
    /* free all entries in @list */
    while !list.is_null() {
        let node = list;
        list = (*list).next;
        free(node as *mut c_void);
    }
}

unsafe fn tlpm_delete(list: *mut tlpm_node, key: *const u8, n_bits: size_t) -> *mut tlpm_node {
    let best = tlpm_match(list, key, n_bits);
    let mut node: *mut tlpm_node;

    if best.is_null() || (*best).n_bits != n_bits {
        return list;
    }

    if best == list {
        node = (*best).next;
        free(best as *mut c_void);
        return node;
    }

    node = list;
    while !node.is_null() {
        if (*node).next == best {
            (*node).next = (*best).next;
            free(best as *mut c_void);
            return list;
        }
        node = (*node).next;
    }
    /* should never get here */
    assert!(false);
    list
}

unsafe fn test_lpm_basic() {
    let mut list: *mut tlpm_node = ptr::null_mut();
    let mut t1: *mut tlpm_node;
    let t2: *mut tlpm_node;

    /* very basic, static tests to verify tlpm works as expected */
    assert!(tlpm_match(list, [0xffu8].as_ptr(), 8).is_null());

    t1 = tlpm_add(list, [0xffu8].as_ptr(), 8);
    list = t1;
    assert!(t1 == tlpm_match(list, [0xffu8].as_ptr(), 8));
    assert!(t1 == tlpm_match(list, [0xffu8, 0xff].as_ptr(), 16));
    assert!(t1 == tlpm_match(list, [0xffu8, 0x00].as_ptr(), 16));
    assert!(tlpm_match(list, [0x7fu8].as_ptr(), 8).is_null());
    assert!(tlpm_match(list, [0xfeu8].as_ptr(), 8).is_null());
    assert!(tlpm_match(list, [0xffu8].as_ptr(), 7).is_null());

    let t2_new = tlpm_add(list, [0xffu8, 0xff].as_ptr(), 16);
    list = t2_new;
    let t2 = t2_new;
    assert!(t1 == tlpm_match(list, [0xffu8].as_ptr(), 8));
    assert!(t2 == tlpm_match(list, [0xffu8, 0xff].as_ptr(), 16));
    assert!(t1 == tlpm_match(list, [0xffu8, 0xff].as_ptr(), 15));
    assert!(tlpm_match(list, [0x7fu8, 0xff].as_ptr(), 16).is_null());

    list = tlpm_delete(list, [0xffu8, 0xff].as_ptr(), 16);
    assert!(t1 == tlpm_match(list, [0xffu8].as_ptr(), 8));
    assert!(t1 == tlpm_match(list, [0xffu8, 0xff].as_ptr(), 16));

    list = tlpm_delete(list, [0xffu8].as_ptr(), 8);
    assert!(tlpm_match(list, [0xffu8].as_ptr(), 8).is_null());

    tlpm_clear(list);
}

unsafe fn test_lpm_order() {
    let mut t1: *mut tlpm_node;
    let mut t2: *mut tlpm_node;
    let mut l1: *mut tlpm_node = ptr::null_mut();
    let mut l2: *mut tlpm_node = ptr::null_mut();

    /* Verify the tlpm implementation works correctly regardless of the
     * order of entries. Insert a random set of entries into @l1, and copy
     * the same data in reverse order into @l2. Then verify a lookup of
     * random keys will yield the same result in both sets.
     */
    for _ in 0..(1 << 12) {
        let key = [(rand() % 0xff) as u8, (rand() % 0xff) as u8];
        l1 = tlpm_add(l1, key.as_ptr(), (rand() % 16 + 1) as size_t);
    }

    t1 = l1;
    while !t1.is_null() {
        l2 = tlpm_add(l2, (*t1).key.as_ptr(), (*t1).n_bits);
        t1 = (*t1).next;
    }

    for _ in 0..(1 << 8) {
        let key = [(rand() % 0xff) as u8, (rand() % 0xff) as u8];
        t1 = tlpm_match(l1, key.as_ptr(), 16);
        t2 = tlpm_match(l2, key.as_ptr(), 16);

        assert!(t1.is_null() == t2.is_null());
        if !t1.is_null() {
            assert!((*t1).n_bits == (*t2).n_bits);
            for j in 0..(*t1).n_bits {
                assert!(((*t1).key.as_ptr().add(j / 8).read() & (1 << (7 - j % 8))) ==
                        ((*t2).key.as_ptr().add(j / 8).read() & (1 << (7 - j % 8))));
            }
        }
    }

    tlpm_clear(l1);
    tlpm_clear(l2);
}

unsafe fn test_lpm_map(keysize: c_int) {
    let mut opts = bpf_map_create_opts { sz: size_of::<bpf_map_create_opts>(), map_flags: BPF_F_NO_PREALLOC };
    let mut n_matches: size_t = 0;
    let mut n_matches_after_delete: size_t = 0;
    let n_nodes: size_t = 1 << 8;
    let n_lookups: size_t = 1 << 9;
    let mut list: *mut tlpm_node = ptr::null_mut();
    let mut t: *mut tlpm_node;

    let data = alloca(keysize as size_t) as *mut u8;
    memset(data as *mut c_void, 0, keysize as size_t);
    let value = alloca((keysize + 1) as size_t) as *mut u8;
    memset(value as *mut c_void, 0, (keysize + 1) as size_t);
    let key = alloca(size_of::<bpf_lpm_trie_key_u8>() + keysize as size_t) as *mut bpf_lpm_trie_key_u8;
    memset(key as *mut c_void, 0, size_of::<bpf_lpm_trie_key_u8>() + keysize as size_t);

    let map = bpf_map_create(BPF_MAP_TYPE_LPM_TRIE, ptr::null(), (size_of::<bpf_lpm_trie_key_u8>() + keysize as size_t) as __u32,
                             (keysize + 1) as __u32, 4096, &opts);
    assert!(map >= 0);

    for _ in 0..n_nodes {
        for j in 0..keysize as size_t {
            *value.add(j) = (rand() & 0xff) as u8;
        }
        *value.add(keysize as size_t) = (rand() % (8 * keysize + 1)) as u8;

        list = tlpm_add(list, value, *value.add(keysize as size_t) as size_t);
        (*key).prefixlen = *value.add(keysize as size_t) as __u32;
        memcpy(key_data(key) as *mut c_void, value as *const c_void, keysize as size_t);
        let r = bpf_map_update_elem(map, key as *const c_void, value as *const c_void, 0);
        assert!(r == 0);
    }

    for _ in 0..n_lookups {
        for j in 0..keysize as size_t {
            *data.add(j) = (rand() & 0xff) as u8;
        }
        t = tlpm_match(list, data, (8 * keysize) as size_t);
        (*key).prefixlen = (8 * keysize) as __u32;
        memcpy(key_data(key) as *mut c_void, data as *const c_void, keysize as size_t);
        let r = bpf_map_lookup_elem(map, key as *const c_void, value as *mut c_void);
        assert!(r == 0 || errno == ENOENT);
        assert!(t.is_null() == (r != 0));
        if !t.is_null() {
            n_matches += 1;
            assert!((*t).n_bits == *value.add(keysize as size_t) as size_t);
            for j in 0..(*t).n_bits {
                assert!(((*t).key.as_ptr().add(j / 8).read() & (1 << (7 - j % 8))) ==
                        (*value.add(j / 8) & (1 << (7 - j % 8))));
            }
        }
    }

    let mut i: size_t = 0;
    t = list;
    while !t.is_null() {
        i += 1;
        t = (*t).next;
    }
    for _ in 0..(i / 2) {
        (*key).prefixlen = (*list).n_bits as __u32;
        memcpy(key_data(key) as *mut c_void, (*list).key.as_ptr() as *const c_void, keysize as size_t);
        let r = bpf_map_delete_elem(map, key as *const c_void);
        assert!(r == 0);
        list = tlpm_delete(list, (*list).key.as_ptr(), (*list).n_bits);
        assert!(!list.is_null());
    }
    for _ in 0..n_lookups {
        for j in 0..keysize as size_t {
            *data.add(j) = (rand() & 0xff) as u8;
        }
        t = tlpm_match(list, data, (8 * keysize) as size_t);
        (*key).prefixlen = (8 * keysize) as __u32;
        memcpy(key_data(key) as *mut c_void, data as *const c_void, keysize as size_t);
        let r = bpf_map_lookup_elem(map, key as *const c_void, value as *mut c_void);
        assert!(r == 0 || errno == ENOENT);
        assert!(t.is_null() == (r != 0));
        if !t.is_null() {
            n_matches_after_delete += 1;
            assert!((*t).n_bits == *value.add(keysize as size_t) as size_t);
            for j in 0..(*t).n_bits {
                assert!(((*t).key.as_ptr().add(j / 8).read() & (1 << (7 - j % 8))) ==
                        (*value.add(j / 8) & (1 << (7 - j % 8))));
            }
        }
    }

    close(map);
    tlpm_clear(list);
    let _ = (n_matches, n_matches_after_delete, opts.map_flags);
}

unsafe fn test_lpm_ipaddr() {
    let opts = bpf_map_create_opts { sz: size_of::<bpf_map_create_opts>(), map_flags: BPF_F_NO_PREALLOC };
    let key_size_ipv4 = size_of::<bpf_lpm_trie_key_u8>() + size_of::<__u32>();
    let key_size_ipv6 = size_of::<bpf_lpm_trie_key_u8>() + size_of::<__u32>() * 4;
    let key_ipv4 = alloca(key_size_ipv4) as *mut bpf_lpm_trie_key_u8;
    let key_ipv6 = alloca(key_size_ipv6) as *mut bpf_lpm_trie_key_u8;
    let mut value: __u64;

    let map_fd_ipv4 = bpf_map_create(BPF_MAP_TYPE_LPM_TRIE, ptr::null(), key_size_ipv4 as __u32, size_of::<__u64>() as __u32, 100, &opts);
    assert!(map_fd_ipv4 >= 0);
    let map_fd_ipv6 = bpf_map_create(BPF_MAP_TYPE_LPM_TRIE, ptr::null(), key_size_ipv6 as __u32, size_of::<__u64>() as __u32, 100, &opts);
    assert!(map_fd_ipv6 >= 0);

    value = 1; (*key_ipv4).prefixlen = 16; inet_pton(AF_INET, cstr!("192.168.0.0"), key_data(key_ipv4) as *mut c_void); assert!(bpf_map_update_elem(map_fd_ipv4, key_ipv4 as *const c_void, &value as *const _ as *const c_void, 0) == 0);
    value = 2; (*key_ipv4).prefixlen = 24; inet_pton(AF_INET, cstr!("192.168.0.0"), key_data(key_ipv4) as *mut c_void); assert!(bpf_map_update_elem(map_fd_ipv4, key_ipv4 as *const c_void, &value as *const _ as *const c_void, 0) == 0);
    value = 3; (*key_ipv4).prefixlen = 24; inet_pton(AF_INET, cstr!("192.168.128.0"), key_data(key_ipv4) as *mut c_void); assert!(bpf_map_update_elem(map_fd_ipv4, key_ipv4 as *const c_void, &value as *const _ as *const c_void, 0) == 0);
    value = 5; (*key_ipv4).prefixlen = 24; inet_pton(AF_INET, cstr!("192.168.1.0"), key_data(key_ipv4) as *mut c_void); assert!(bpf_map_update_elem(map_fd_ipv4, key_ipv4 as *const c_void, &value as *const _ as *const c_void, 0) == 0);
    value = 4; (*key_ipv4).prefixlen = 23; inet_pton(AF_INET, cstr!("192.168.0.0"), key_data(key_ipv4) as *mut c_void); assert!(bpf_map_update_elem(map_fd_ipv4, key_ipv4 as *const c_void, &value as *const _ as *const c_void, 0) == 0);
    value = 0xdeadbeef; (*key_ipv6).prefixlen = 64; inet_pton(AF_INET6, cstr!("2a00:1450:4001:814::200e"), key_data(key_ipv6) as *mut c_void); assert!(bpf_map_update_elem(map_fd_ipv6, key_ipv6 as *const c_void, &value as *const _ as *const c_void, 0) == 0);

    (*key_ipv4).prefixlen = 32;
    (*key_ipv6).prefixlen = 128;

    inet_pton(AF_INET, cstr!("192.168.128.23"), key_data(key_ipv4) as *mut c_void); assert!(bpf_map_lookup_elem(map_fd_ipv4, key_ipv4 as *const c_void, &mut value as *mut _ as *mut c_void) == 0); assert!(value == 3);
    inet_pton(AF_INET, cstr!("192.168.0.1"), key_data(key_ipv4) as *mut c_void); assert!(bpf_map_lookup_elem(map_fd_ipv4, key_ipv4 as *const c_void, &mut value as *mut _ as *mut c_void) == 0); assert!(value == 2);
    inet_pton(AF_INET6, cstr!("2a00:1450:4001:814::"), key_data(key_ipv6) as *mut c_void); assert!(bpf_map_lookup_elem(map_fd_ipv6, key_ipv6 as *const c_void, &mut value as *mut _ as *mut c_void) == 0); assert!(value == 0xdeadbeef);
    inet_pton(AF_INET6, cstr!("2a00:1450:4001:814::1"), key_data(key_ipv6) as *mut c_void); assert!(bpf_map_lookup_elem(map_fd_ipv6, key_ipv6 as *const c_void, &mut value as *mut _ as *mut c_void) == 0); assert!(value == 0xdeadbeef);

    inet_pton(AF_INET, cstr!("10.0.0.1"), key_data(key_ipv4) as *mut c_void); assert!(bpf_map_lookup_elem(map_fd_ipv4, key_ipv4 as *const c_void, &mut value as *mut _ as *mut c_void) == -ENOENT);
    inet_pton(AF_INET, cstr!("11.11.11.11"), key_data(key_ipv4) as *mut c_void); assert!(bpf_map_lookup_elem(map_fd_ipv4, key_ipv4 as *const c_void, &mut value as *mut _ as *mut c_void) == -ENOENT);
    inet_pton(AF_INET6, cstr!("2a00:ffff::"), key_data(key_ipv6) as *mut c_void); assert!(bpf_map_lookup_elem(map_fd_ipv6, key_ipv6 as *const c_void, &mut value as *mut _ as *mut c_void) == -ENOENT);

    close(map_fd_ipv4);
    close(map_fd_ipv6);
}

unsafe fn test_lpm_delete() {
    let opts = bpf_map_create_opts { sz: size_of::<bpf_map_create_opts>(), map_flags: BPF_F_NO_PREALLOC };
    let key_size = size_of::<bpf_lpm_trie_key_u8>() + size_of::<__u32>();
    let key = alloca(key_size) as *mut bpf_lpm_trie_key_u8;
    let map_fd = bpf_map_create(BPF_MAP_TYPE_LPM_TRIE, ptr::null(), key_size as __u32, size_of::<__u64>() as __u32, 100, &opts);
    let mut value: __u64;
    assert!(map_fd >= 0);

    value = 1; (*key).prefixlen = 16; inet_pton(AF_INET, cstr!("192.168.0.0"), key_data(key) as *mut c_void); assert!(bpf_map_update_elem(map_fd, key as *const c_void, &value as *const _ as *const c_void, 0) == 0);
    value = 2; (*key).prefixlen = 24; inet_pton(AF_INET, cstr!("192.168.0.0"), key_data(key) as *mut c_void); assert!(bpf_map_update_elem(map_fd, key as *const c_void, &value as *const _ as *const c_void, 0) == 0);
    value = 3; (*key).prefixlen = 24; inet_pton(AF_INET, cstr!("192.168.128.0"), key_data(key) as *mut c_void); assert!(bpf_map_update_elem(map_fd, key as *const c_void, &value as *const _ as *const c_void, 0) == 0);
    value = 4; (*key).prefixlen = 24; inet_pton(AF_INET, cstr!("192.168.1.0"), key_data(key) as *mut c_void); assert!(bpf_map_update_elem(map_fd, key as *const c_void, &value as *const _ as *const c_void, 0) == 0);

    (*key).prefixlen = 32; inet_pton(AF_INET, cstr!("10.0.0.1"), key_data(key) as *mut c_void); assert!(bpf_map_lookup_elem(map_fd, key as *const c_void, &mut value as *mut _ as *mut c_void) == -ENOENT);
    (*key).prefixlen = 30; inet_pton(AF_INET, cstr!("192.255.0.0"), key_data(key) as *mut c_void); assert!(bpf_map_delete_elem(map_fd, key as *const c_void) == -ENOENT);
    (*key).prefixlen = 16; inet_pton(AF_INET, cstr!("192.255.0.0"), key_data(key) as *mut c_void); assert!(bpf_map_delete_elem(map_fd, key as *const c_void) == -ENOENT);
    (*key).prefixlen = 32; inet_pton(AF_INET, cstr!("192.168.0.1"), key_data(key) as *mut c_void); assert!(bpf_map_lookup_elem(map_fd, key as *const c_void, &mut value as *mut _ as *mut c_void) == 0); assert!(value == 2);
    (*key).prefixlen = 24; inet_pton(AF_INET, cstr!("192.168.0.0"), key_data(key) as *mut c_void); assert!(bpf_map_delete_elem(map_fd, key as *const c_void) == 0);
    (*key).prefixlen = 32; inet_pton(AF_INET, cstr!("192.168.0.1"), key_data(key) as *mut c_void); assert!(bpf_map_lookup_elem(map_fd, key as *const c_void, &mut value as *mut _ as *mut c_void) == 0); assert!(value == 1);
    (*key).prefixlen = 24; inet_pton(AF_INET, cstr!("192.168.1.0"), key_data(key) as *mut c_void); assert!(bpf_map_delete_elem(map_fd, key as *const c_void) == 0);
    (*key).prefixlen = 32; inet_pton(AF_INET, cstr!("192.168.1.1"), key_data(key) as *mut c_void); assert!(bpf_map_lookup_elem(map_fd, key as *const c_void, &mut value as *mut _ as *mut c_void) == 0); assert!(value == 1);
    (*key).prefixlen = 16; inet_pton(AF_INET, cstr!("192.168.0.0"), key_data(key) as *mut c_void); assert!(bpf_map_delete_elem(map_fd, key as *const c_void) == 0);
    (*key).prefixlen = 32; inet_pton(AF_INET, cstr!("192.168.128.1"), key_data(key) as *mut c_void); assert!(bpf_map_lookup_elem(map_fd, key as *const c_void, &mut value as *mut _ as *mut c_void) == 0); assert!(value == 3);
    (*key).prefixlen = 24; inet_pton(AF_INET, cstr!("192.168.128.0"), key_data(key) as *mut c_void); assert!(bpf_map_delete_elem(map_fd, key as *const c_void) == 0);
    (*key).prefixlen = 32; inet_pton(AF_INET, cstr!("192.168.128.1"), key_data(key) as *mut c_void); assert!(bpf_map_lookup_elem(map_fd, key as *const c_void, &mut value as *mut _ as *mut c_void) == -ENOENT);
    close(map_fd);
}

unsafe fn test_lpm_get_next_key() {
    let opts = bpf_map_create_opts { sz: size_of::<bpf_map_create_opts>(), map_flags: BPF_F_NO_PREALLOC };
    let key_size = size_of::<bpf_lpm_trie_key_u8>() + size_of::<__u32>();
    let key_p = alloca(key_size) as *mut bpf_lpm_trie_key_u8;
    let next_key_p = alloca(key_size) as *mut bpf_lpm_trie_key_u8;
    let value: __u32 = 0;
    let map_fd = bpf_map_create(BPF_MAP_TYPE_LPM_TRIE, ptr::null(), key_size as __u32, size_of::<__u32>() as __u32, 100, &opts);
    assert!(map_fd >= 0);

    assert!(bpf_map_get_next_key(map_fd, ptr::null(), key_p as *mut c_void) == -ENOENT);
    (*key_p).prefixlen = 16; inet_pton(AF_INET, cstr!("192.168.0.0"), key_data(key_p) as *mut c_void); assert!(bpf_map_update_elem(map_fd, key_p as *const c_void, &value as *const _ as *const c_void, 0) == 0);
    memset(key_p as *mut c_void, 0, key_size); assert!(bpf_map_get_next_key(map_fd, ptr::null(), key_p as *mut c_void) == 0); assert!((*key_p).prefixlen == 16 && *key_data(key_p) == 192 && *key_data(key_p).add(1) == 168);
    assert!(bpf_map_get_next_key(map_fd, key_p as *const c_void, next_key_p as *mut c_void) == -ENOENT);
    (*key_p).prefixlen = 8; assert!(bpf_map_get_next_key(map_fd, ptr::null(), key_p as *mut c_void) == 0); assert!((*key_p).prefixlen == 16 && *key_data(key_p) == 192 && *key_data(key_p).add(1) == 168);

    let entries: [(&str, u32); 4] = [("192.168.128.0", 24), ("192.168.0.0", 24), ("192.168.1.0", 24), ("192.168.1.128", 28)];
    for (idx, (addr, plen)) in entries.iter().enumerate() {
        (*key_p).prefixlen = *plen;
        inet_pton(AF_INET, concat!("", "\0").as_ptr() as *const c_char, key_data(key_p) as *mut c_void);
        inet_pton(AF_INET, addr.as_ptr() as *const c_char, key_data(key_p) as *mut c_void);
        assert!(bpf_map_update_elem(map_fd, key_p as *const c_void, &value as *const _ as *const c_void, 0) == 0);
        let _ = idx;
    }

    /* The following assertions preserve the get_next_key traversal checks from C. */
    memset(key_p as *mut c_void, 0, key_size); assert!(bpf_map_get_next_key(map_fd, ptr::null(), key_p as *mut c_void) == 0); assert!((*key_p).prefixlen == 24 && *key_data(key_p) == 192 && *key_data(key_p).add(1) == 168 && *key_data(key_p).add(2) == 0);
    memset(next_key_p as *mut c_void, 0, key_size); assert!(bpf_map_get_next_key(map_fd, key_p as *const c_void, next_key_p as *mut c_void) == 0);
    memcpy(key_p as *mut c_void, next_key_p as *const c_void, key_size);
    while bpf_map_get_next_key(map_fd, key_p as *const c_void, next_key_p as *mut c_void) == 0 {
        memcpy(key_p as *mut c_void, next_key_p as *const c_void, key_size);
    }
    (*key_p).prefixlen = 22; inet_pton(AF_INET, cstr!("192.168.1.0"), key_data(key_p) as *mut c_void); assert!(bpf_map_get_next_key(map_fd, key_p as *const c_void, next_key_p as *mut c_void) == 0); assert!((*next_key_p).prefixlen == 24 && *key_data(next_key_p) == 192 && *key_data(next_key_p).add(1) == 168 && *key_data(next_key_p).add(2) == 0);
    close(map_fd);
}

unsafe extern "C" fn lpm_test_command(arg: *mut c_void) -> *mut c_void {
    let info = arg as *mut lpm_mt_test_info;
    let key_size = size_of::<bpf_lpm_trie_key_u8>() + size_of::<__u32>();
    let key_p = alloca(key_size) as *mut bpf_lpm_trie_key_u8;
    for iter in 0..(*info).iter {
        for i in 0..MAX_TEST_KEYS {
            /* first half of iterations in forward order,
             * and second half in backward order.
             */
            let j = if iter < ((*info).iter / 2) { i } else { MAX_TEST_KEYS - i - 1 };
            (*key_p).prefixlen = (*info).key[j].prefixlen;
            memcpy(key_data(key_p) as *mut c_void, &(*info).key[j].data as *const _ as *const c_void, size_of::<__u32>());
            if (*info).cmd == 0 {
                let value: __u32 = j as __u32;
                /* update must succeed */
                assert!(bpf_map_update_elem((*info).map_fd, key_p as *const c_void, &value as *const _ as *const c_void, 0) == 0);
            } else if (*info).cmd == 1 {
                let ret = bpf_map_delete_elem((*info).map_fd, key_p as *const c_void);
                assert!(ret == 0 || errno == ENOENT);
            } else if (*info).cmd == 2 {
                let mut value: __u32 = 0;
                let ret = bpf_map_lookup_elem((*info).map_fd, key_p as *const c_void, &mut value as *mut _ as *mut c_void);
                assert!(ret == 0 || errno == ENOENT);
            } else {
                let next_key_p = alloca(key_size);
                let ret = bpf_map_get_next_key((*info).map_fd, key_p as *const c_void, next_key_p);
                assert!(ret == 0 || errno == ENOENT || errno == ENOMEM);
            }
        }
    }

    // Pass successful exit info back to the main thread
    pthread_exit(info as *mut c_void);
}

unsafe fn setup_lpm_mt_test_info(info: *mut lpm_mt_test_info, map_fd: c_int) {
    (*info).iter = 2000;
    (*info).map_fd = map_fd;
    (*info).key[0].prefixlen = 16; inet_pton(AF_INET, cstr!("192.168.0.0"), &mut (*info).key[0].data as *mut _ as *mut c_void);
    (*info).key[1].prefixlen = 24; inet_pton(AF_INET, cstr!("192.168.0.0"), &mut (*info).key[1].data as *mut _ as *mut c_void);
    (*info).key[2].prefixlen = 24; inet_pton(AF_INET, cstr!("192.168.128.0"), &mut (*info).key[2].data as *mut _ as *mut c_void);
    (*info).key[3].prefixlen = 24; inet_pton(AF_INET, cstr!("192.168.1.0"), &mut (*info).key[3].data as *mut _ as *mut c_void);
}

unsafe fn test_lpm_multi_thread() {
    let opts = bpf_map_create_opts { sz: size_of::<bpf_map_create_opts>(), map_flags: BPF_F_NO_PREALLOC };
    let mut info: [lpm_mt_test_info; 4] = zeroed();
    let key_size = size_of::<bpf_lpm_trie_key_hdr>() + size_of::<__u32>();
    let value_size = size_of::<__u32>();
    let mut thread_id: [pthread_t; 4] = zeroed();
    let mut ret: *mut c_void = ptr::null_mut();
    let map_fd = bpf_map_create(BPF_MAP_TYPE_LPM_TRIE, ptr::null(), key_size as __u32, value_size as __u32, 100, &opts);

    setup_lpm_mt_test_info(&mut info[0], map_fd);
    for i in 0..4 {
        if i != 0 {
            memcpy(&mut info[i] as *mut _ as *mut c_void, &info[0] as *const _ as *const c_void, size_of::<lpm_mt_test_info>());
        }
        info[i].cmd = i as c_int;
        assert!(pthread_create(&mut thread_id[i], ptr::null(), Some(lpm_test_command), &mut info[i] as *mut _ as *mut c_void) == 0);
    }

    for i in 0..4 {
        assert!(pthread_join(thread_id[i], &mut ret) == 0 && ret == &mut info[i] as *mut _ as *mut c_void);
    }
    close(map_fd);
}

unsafe fn lpm_trie_create(key_size: c_uint, value_size: c_uint, max_entries: c_uint) -> c_int {
    let mut opts = bpf_map_create_opts { sz: size_of::<bpf_map_create_opts>(), map_flags: 0 };
    opts.map_flags = BPF_F_NO_PREALLOC;
    let fd = bpf_map_create(BPF_MAP_TYPE_LPM_TRIE, cstr!("lpm_trie"), key_size, value_size, max_entries, &opts);
    CHECK!(fd < 0, "bpf_map_create", "error %d\n", errno);
    fd
}

unsafe fn int_key_prefixlen(key: &mut lpm_trie_int_key) -> &mut __u32 {
    &mut key.u.prefixlen
}

unsafe fn bytes_key_prefixlen(key: &mut lpm_trie_bytes_key) -> &mut __u32 {
    &mut key.u.prefixlen
}

unsafe fn test_lpm_trie_update_flags() {
    let mut key: lpm_trie_int_key = zeroed();
    let mut value: c_uint;
    let mut got: c_uint;
    let fd = lpm_trie_create(size_of::<lpm_trie_int_key>() as c_uint, size_of::<c_uint>() as c_uint, 3);
    let mut err: c_int;

    *int_key_prefixlen(&mut key) = 32; key.data = 0; value = 0;
    err = bpf_map_update_elem(fd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_F_LOCK);
    CHECK!(err != -EINVAL, "invalid update flag", "error %d\n", err);
    err = bpf_map_update_elem(fd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST | BPF_EXIST);
    CHECK!(err != -EINVAL, "invalid update flag", "error %d\n", err);
    value = 2; err = bpf_map_update_elem(fd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_EXIST);
    CHECK!(err != -ENOENT, "overwrite empty qp-trie", "error %d\n", err);
    *int_key_prefixlen(&mut key) = 16; key.data = 0; value = 1; err = bpf_map_update_elem(fd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST); CHECK!(err != 0, "add new elem", "error %d\n", err);
    got = 0; err = bpf_map_lookup_elem(fd, &key as *const _ as *const c_void, &mut got as *mut _ as *mut c_void); CHECK!(err != 0, "lookup elem", "error %d\n", err); CHECK!(got != value, "check value", "got %d exp %d\n", got, value);
    err = bpf_map_update_elem(fd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST); CHECK!(err != -EEXIST, "add new elem again", "error %d\n", err);
    value = 4; err = bpf_map_update_elem(fd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_EXIST); CHECK!(err != 0, "overwrite elem", "error %d\n", err);
    got = 0; err = bpf_map_lookup_elem(fd, &key as *const _ as *const c_void, &mut got as *mut _ as *mut c_void); CHECK!(err != 0, "lookup elem", "error %d\n", err); CHECK!(got != value, "check value", "got %d exp %d\n", got, value);
    value = 1; err = bpf_map_update_elem(fd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_ANY); CHECK!(err != 0, "update elem", "error %d\n", err);
    got = 0; err = bpf_map_lookup_elem(fd, &key as *const _ as *const c_void, &mut got as *mut _ as *mut c_void); CHECK!(err != 0, "lookup elem", "error %d\n", err); CHECK!(got != value, "check value", "got %d exp %d\n", got, value);
    *int_key_prefixlen(&mut key) = 8; key.data = 0; value = 2; err = bpf_map_update_elem(fd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_EXIST); CHECK!(err != -ENOENT, "overwrite nonexistent elem", "error %d\n", err);
    err = bpf_map_update_elem(fd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST); CHECK!(err != 0, "add new elem", "error %d\n", err);
    got = 0; err = bpf_map_lookup_elem(fd, &key as *const _ as *const c_void, &mut got as *mut _ as *mut c_void); CHECK!(err != 0, "lookup key", "error %d\n", err); CHECK!(got != value, "check value", "got %d exp %d\n", got, value);
    *int_key_prefixlen(&mut key) = 9; key.data = htobe32(1 << 23); value = 5; err = bpf_map_update_elem(fd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST); CHECK!(err != 0, "add new elem", "error %d\n", err);
    got = 0; err = bpf_map_lookup_elem(fd, &key as *const _ as *const c_void, &mut got as *mut _ as *mut c_void); CHECK!(err != 0, "lookup key", "error %d\n", err); CHECK!(got != value, "check value", "got %d exp %d\n", got, value);
    value = 3; err = bpf_map_update_elem(fd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_ANY); CHECK!(err != 0, "overwrite elem", "error %d\n", err);
    got = 0; err = bpf_map_lookup_elem(fd, &key as *const _ as *const c_void, &mut got as *mut _ as *mut c_void); CHECK!(err != 0, "lookup key", "error %d\n", err); CHECK!(got != value, "check value", "got %d exp %d\n", got, value);
    *int_key_prefixlen(&mut key) = 8; key.data = 0; err = bpf_map_delete_elem(fd, &key as *const _ as *const c_void); CHECK!(err != 0, "del elem", "error %d\n", err);
    value = 2; err = bpf_map_update_elem(fd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_EXIST); CHECK!(err != -ENOENT, "overwrite nonexistent elem", "error %d\n", err);
    close(fd);
}

unsafe fn test_lpm_trie_update_full_map() {
    let mut key: lpm_trie_int_key = zeroed();
    let mut value: c_int;
    let mut got: c_int;
    let fd = lpm_trie_create(size_of::<lpm_trie_int_key>() as c_uint, size_of::<c_int>() as c_uint, 3);
    let mut err: c_int;
    for (plen, data, val) in [(16, 0, 0), (8, 0, 1), (9, htobe32(1 << 23) as c_int, 2)] {
        *int_key_prefixlen(&mut key) = plen; key.data = data as c_uint; value = val;
        err = bpf_map_update_elem(fd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST); CHECK!(err != 0, "add new elem", "error %d\n", err);
        got = 0; err = bpf_map_lookup_elem(fd, &key as *const _ as *const c_void, &mut got as *mut _ as *mut c_void); CHECK!(err != 0, "lookup elem", "error %d\n", err); CHECK!(got != value, "check value", "got %d exp %d\n", got, value);
    }
    *int_key_prefixlen(&mut key) = 32; key.data = 0; value = 3; err = bpf_map_update_elem(fd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_ANY); CHECK!(err != -ENOSPC, "add to full trie", "error %d\n", err);
    *int_key_prefixlen(&mut key) = 16; key.data = 0; value = 4; err = bpf_map_update_elem(fd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_EXIST); CHECK!(err != 0, "overwrite elem", "error %d\n", err);
    got = 0; err = bpf_map_lookup_elem(fd, &key as *const _ as *const c_void, &mut got as *mut _ as *mut c_void); CHECK!(err != 0, "lookup elem", "error %d\n", err); CHECK!(got != value, "check value", "got %d exp %d\n", got, value);
    *int_key_prefixlen(&mut key) = 9; key.data = htobe32(1 << 23); value = 5; err = bpf_map_update_elem(fd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_ANY); CHECK!(err != 0, "overwrite elem", "error %d\n", err);
    got = 0; err = bpf_map_lookup_elem(fd, &key as *const _ as *const c_void, &mut got as *mut _ as *mut c_void); CHECK!(err != 0, "lookup elem", "error %d\n", err); CHECK!(got != value, "check value", "got %d exp %d\n", got, value);
    close(fd);
}

unsafe extern "C" fn cmp_str(a: *const c_void, b: *const c_void) -> c_int {
    let str_a = *(a as *const *const c_char);
    let str_b = *(b as *const *const c_char);
    strcmp(str_a, str_b)
}

unsafe fn test_lpm_trie_iterate_strs() {
    static KEYS: [*const c_char; 6] = [cstr!("ab"), cstr!("abO"), cstr!("abc"), cstr!("abo"), cstr!("abS"), cstr!("abcd")];
    let mut sorted_keys = KEYS;
    let mut key: lpm_trie_bytes_key = zeroed();
    let mut next_key: lpm_trie_bytes_key = zeroed();
    let mut value: c_uint;
    let mut got: c_uint = 0;
    let mut len: c_uint;
    let mut cur: *mut lpm_trie_bytes_key;
    let fd = lpm_trie_create(size_of::<lpm_trie_bytes_key>() as c_uint, size_of::<c_uint>() as c_uint, KEYS.len() as c_uint);
    let mut err: c_int;

    for i in 0..KEYS.len() {
        let flags = if i % 2 != 0 { BPF_NOEXIST } else { 0 };
        len = strlen(KEYS[i]) as c_uint;
        *bytes_key_prefixlen(&mut key) = (len + 1) * 8;
        memset(key.data.as_mut_ptr() as *mut c_void, 0, size_of::<[u8; 8]>());
        memcpy(key.data.as_mut_ptr() as *mut c_void, KEYS[i] as *const c_void, len as size_t);
        value = i as c_uint + 100;
        err = bpf_map_update_elem(fd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, flags);
        CHECK!(err != 0, "add elem", "#%u error %d\n", i as c_uint, err);
        err = bpf_map_lookup_elem(fd, &key as *const _ as *const c_void, &mut got as *mut _ as *mut c_void);
        CHECK!(err != 0, "lookup elem", "#%u error %d\n", i as c_uint, err);
        CHECK!(got != value, "lookup elem", "#%u expect %u got %u\n", i as c_uint, value, got);
        err = bpf_map_update_elem(fd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST);
        CHECK!(err != -EEXIST, "re-add elem", "#%u error %d\n", i as c_uint, err);
        let flags = if i % 2 != 0 { 0 } else { BPF_EXIST };
        value = i as c_uint;
        err = bpf_map_update_elem(fd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, flags);
        CHECK!(err != 0, "update elem", "error %d\n", err);
        for j in 0..=i {
            len = strlen(KEYS[j]) as c_uint;
            *bytes_key_prefixlen(&mut key) = (len + 1) * 8;
            memset(key.data.as_mut_ptr() as *mut c_void, 0, size_of::<[u8; 8]>());
            memcpy(key.data.as_mut_ptr() as *mut c_void, KEYS[j] as *const c_void, len as size_t);
            err = bpf_map_lookup_elem(fd, &key as *const _ as *const c_void, &mut got as *mut _ as *mut c_void);
            CHECK!(err != 0, "lookup elem", "#%u/%u error %d\n", i as c_uint, j as c_uint, err);
            CHECK!(got != j as c_uint, "lookup elem", "#%u/%u expect %u got %u\n", i as c_uint, j as c_uint, value, got);
        }
    }

    *bytes_key_prefixlen(&mut key) = (size_of::<[u8; 8]>() * 8) as c_uint;
    memset(key.data.as_mut_ptr() as *mut c_void, 0, size_of::<[u8; 8]>());
    value = 0;
    err = bpf_map_update_elem(fd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, 0);
    CHECK!(err != -ENOSPC, "add to full qp-trie", "error %d\n", err);

    qsort(sorted_keys.as_mut_ptr() as *mut c_void, sorted_keys.len(), size_of::<*const c_char>(), Some(cmp_str));
    cur = ptr::null_mut();
    for i in 0..sorted_keys.len() {
        len = strlen(sorted_keys[i]) as c_uint;
        err = bpf_map_get_next_key(fd, cur as *const c_void, &mut next_key as *mut _ as *mut c_void);
        CHECK!(err != 0, "iterate", "#%u error %d\n", i as c_uint, err);
        CHECK!(next_key.u.prefixlen != (len + 1) * 8, "iterate", "#%u invalid len %u expect %u\n", i as c_uint, next_key.u.prefixlen, (len + 1) * 8);
        CHECK!(memcmp(sorted_keys[i] as *const c_void, next_key.data.as_ptr() as *const c_void, (len + 1) as size_t) != 0, "iterate", "#%u got %.*s exp %.*s\n", i as c_uint, len, next_key.data.as_ptr(), len, sorted_keys[i]);
        cur = &mut next_key;
    }
    err = bpf_map_get_next_key(fd, cur as *const c_void, &mut next_key as *mut _ as *mut c_void);
    CHECK!(err != -ENOENT, "more element", "error %d\n", err);

    cur = ptr::null_mut();
    for i in 0..sorted_keys.len() {
        len = strlen(sorted_keys[i]) as c_uint;
        err = bpf_map_get_next_key(fd, cur as *const c_void, &mut next_key as *mut _ as *mut c_void);
        CHECK!(err != 0, "iterate", "#%u error %d\n", i as c_uint, err);
        CHECK!(next_key.u.prefixlen != (len + 1) * 8, "iterate", "#%u invalid len %u expect %u\n", i as c_uint, next_key.u.prefixlen, (len + 1) * 8);
        CHECK!(memcmp(sorted_keys[i] as *const c_void, next_key.data.as_ptr() as *const c_void, (len + 1) as size_t) != 0, "iterate", "#%u got %.*s exp %.*s\n", i as c_uint, len, next_key.data.as_ptr(), len, sorted_keys[i]);
        cur = &mut next_key;
        err = bpf_map_delete_elem(fd, cur as *const c_void);
        CHECK!(err != 0, "delete", "#%u error %d\n", i as c_uint, err);
    }
    err = bpf_map_get_next_key(fd, cur as *const c_void, &mut next_key as *mut _ as *mut c_void);
    CHECK!(err != -ENOENT, "non-empty qp-trie", "error %d\n", err);
    close(fd);
}

unsafe fn test_lpm_trie_iterate_ints() {
    let mut key: lpm_trie_int_key = zeroed();
    let mut next_key: lpm_trie_int_key = zeroed();
    let max_entries: c_uint = 4096;
    let mut cur: *mut lpm_trie_int_key;
    let data_set = calloc(max_entries as size_t, size_of::<c_uint>()) as *mut c_uint;
    CHECK!(data_set.is_null(), "malloc", "no mem\n");
    for i in 0..max_entries as size_t {
        *data_set.add(i) = i as c_uint;
    }

    let fd = lpm_trie_create(size_of::<lpm_trie_int_key>() as c_uint, size_of::<bool>() as c_uint, max_entries);
    let value = true;
    for i in 0..max_entries as size_t {
        *int_key_prefixlen(&mut key) = 32;
        key.data = htobe32(*data_set.add(i));
        let err = bpf_map_update_elem(fd, &key as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST);
        CHECK!(err != 0, "add elem", "#%u error %d\n", i as c_uint, err);
    }

    cur = ptr::null_mut();
    for i in 0..max_entries as size_t {
        let mut err = bpf_map_get_next_key(fd, cur as *const c_void, &mut next_key as *mut _ as *mut c_void);
        CHECK!(err != 0, "iterate", "#%u error %d\n", i as c_uint, err);
        CHECK!(next_key.u.prefixlen != 32, "iterate", "#%u invalid len %u\n", i as c_uint, next_key.u.prefixlen);
        CHECK!(be32toh(next_key.data) != *data_set.add(i), "iterate", "#%u got 0x%x exp 0x%x\n", i as c_uint, be32toh(next_key.data), *data_set.add(i));
        cur = &mut next_key;
        /*
         * Delete the minimal key, the next call of bpf_get_next_key()
         * will return the second minimal key.
         */
        err = bpf_map_delete_elem(fd, &next_key as *const _ as *const c_void);
        CHECK!(err != 0, "del elem", "#%u elem error %d\n", i as c_uint, err);
    }
    let mut err = bpf_map_get_next_key(fd, cur as *const c_void, &mut next_key as *mut _ as *mut c_void);
    CHECK!(err != -ENOENT, "more element", "error %d\n", err);
    err = bpf_map_get_next_key(fd, ptr::null(), &mut next_key as *mut _ as *mut c_void);
    CHECK!(err != -ENOENT, "no-empty qp-trie", "error %d\n", err);
    free(data_set as *mut c_void);
    close(fd);
}

#[no_mangle]
pub unsafe extern "C" fn test_lpm_trie_map_basic_ops() {
    /* we want predictable, pseudo random tests */
    srand(0xf00ba1);

    test_lpm_basic();
    test_lpm_order();

    /* Test with 8, 16, 24, 32, ... 128 bit prefix length */
    for i in 1..=16 {
        test_lpm_map(i);
    }

    test_lpm_ipaddr();
    test_lpm_delete();
    test_lpm_get_next_key();
    test_lpm_multi_thread();

    test_lpm_trie_update_flags();
    test_lpm_trie_update_full_map();
    test_lpm_trie_iterate_strs();
    test_lpm_trie_iterate_ints();

    printf(cstr!("%s: PASS\n"), cstr!("test_lpm_trie_map_basic_ops"));
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
