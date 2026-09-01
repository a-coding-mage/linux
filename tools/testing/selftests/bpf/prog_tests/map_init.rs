// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2020 Tessares SA <http://www.tessares.net> */

// C dependencies: <test_progs.h>, "test_map_init.skel.h"

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

const TEST_VALUE: map_value_t = 0x1234;
const FILL_VALUE: map_value_t = 0xdeadbeef;

static mut nr_cpus: c_int = 0;
static mut duration: c_int = 0;

type map_key_t = u64;
type map_value_t = u64;

#[repr(C)]
struct pcpu_map_value_t {
    v: map_value_t, /* padding */
}

#[repr(C)]
struct test_map_init {
    maps: test_map_init__maps,
    bss: *mut test_map_init__bss,
}

#[repr(C)]
struct test_map_init__maps {
    hashmap1: *mut bpf_map,
}

#[repr(C)]
struct test_map_init__bss {
    inKey: map_key_t,
    inValue: map_value_t,
    inPid: c_int,
}

#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_map_create_opts {
    log_opts: *mut bpf_log_opts,
    btf_vmlinux_value_type_id: u32,
    btf_key_type_id: u32,
    btf_value_type_id: u32,
    map_extra: u64,
    map_flags: u32,
    numa_node: u32,
    token_fd: c_int,
    btf_fd: c_int,
    excl_prog_hash: *const c_char,
    excl_prog_hash_size: u32,
}

impl Default for bpf_map_create_opts {
    fn default() -> Self {
        Self {
            log_opts: ptr::null_mut(),
            btf_vmlinux_value_type_id: 0,
            btf_key_type_id: 0,
            btf_value_type_id: 0,
            map_extra: 0,
            map_flags: 0,
            numa_node: 0,
            token_fd: 0,
            btf_fd: 0,
            excl_prog_hash: ptr::null(),
            excl_prog_hash_size: 0,
        }
    }
}

#[repr(C)]
struct bpf_log_opts {
    buf: *mut c_char,
    size: usize,
    level: u32,
    true_size: usize,
}

impl Default for bpf_log_opts {
    fn default() -> Self {
        Self {
            buf: ptr::null_mut(),
            size: 0,
            level: 0,
            true_size: 0,
        }
    }
}

#[repr(C)]
struct bpf_common_attr {
    log_true_size: u32,
}

#[repr(C)]
struct bpf_common_attr_fake {
    attrs: [u8; offset_of!(bpf_common_attr, log_true_size) + size_of::<u32>()],
    pad: u32,
}

#[repr(C)]
union bpf_attr {
    map_type: bpf_map_type,
    key_size: u32,
    value_size: u32,
    max_entries: u32,
}

type bpf_map_type = u32;

const BPF_MAP_TYPE_ARRAY: bpf_map_type = 2;
const BPF_MAP_TYPE_PERCPU_HASH: bpf_map_type = 5;
const BPF_MAP_TYPE_LRU_PERCPU_HASH: bpf_map_type = 10;
const BPF_MAP_TYPE_STRUCT_OPS: bpf_map_type = 26;
const __MAX_BPF_MAP_TYPE: bpf_map_type = 36;
const BPF_NOEXIST: u64 = 1;
const BPF_F_NUMA_NODE: u32 = 1 << 2;
const BPF_F_TOKEN_FD: u32 = 1 << 16;
const BPF_MAP_CREATE: c_long = 0;
const BPF_COMMON_ATTRS: c_long = 1 << 4;
const __NR_getpgid: c_long = 121;
const __NR_bpf: c_long = 321;
const E2BIG: c_int = 7;

unsafe extern "C" {
    fn bpf_num_possible_cpus() -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map__set_type(map: *mut bpf_map, map_type: bpf_map_type) -> c_int;
    fn bpf_map__set_max_entries(map: *mut bpf_map, max_entries: c_int) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_create(
        map_type: bpf_map_type,
        map_name: *const c_char,
        key_size: c_int,
        value_size: c_int,
        max_entries: c_int,
        opts: *mut bpf_map_create_opts,
    ) -> c_int;
    fn test_map_init__open() -> *mut test_map_init;
    fn test_map_init__load(skel: *mut test_map_init) -> c_int;
    fn test_map_init__attach(skel: *mut test_map_init) -> c_int;
    fn test_map_init__detach(skel: *mut test_map_init);
    fn test_map_init__destroy(skel: *mut test_map_init);
    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;
    fn getpid() -> c_int;
    fn close(fd: c_int) -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    static mut errno: c_int;
}

unsafe fn ASSERT_OK(err: c_int, name: *const c_char) -> bool {
    err == 0
}

unsafe fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool {
    !ptr.is_null()
}

unsafe fn ASSERT_LT(left: c_int, right: c_int, name: *const c_char) -> bool {
    left < right
}

unsafe fn ASSERT_STREQ(left: *const c_char, right: *const c_char, name: *const c_char) -> bool {
    strcmp(left, right) == 0
}

unsafe fn ASSERT_EQ<T: PartialEq>(left: T, right: T, name: *const c_char) -> bool {
    left == right
}

unsafe fn CHECK(cond: bool, name: *const c_char, fmt: *const c_char) -> bool {
    cond
}

unsafe extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

unsafe fn bpf_percpu_mut(value: *mut pcpu_map_value_t, cpu: c_int) -> *mut map_value_t {
    &mut (*value.add(cpu as usize)).v
}

unsafe fn bpf_percpu(value: *const pcpu_map_value_t, cpu: c_int) -> map_value_t {
    (*value.add(cpu as usize)).v
}

unsafe fn map_populate(map_fd: c_int, num: c_int) -> c_int {
    let mut value: Vec<pcpu_map_value_t> = (0..nr_cpus)
        .map(|_| pcpu_map_value_t { v: 0 })
        .collect();
    let mut i: c_int;
    let mut err: c_int;
    let mut key: map_key_t;

    i = 0;
    while i < nr_cpus {
        *bpf_percpu_mut(value.as_mut_ptr(), i) = FILL_VALUE;
        i += 1;
    }

    key = 1;
    while key <= num as map_key_t {
        err = bpf_map_update_elem(
            map_fd,
            &key as *const _ as *const c_void,
            value.as_ptr() as *const c_void,
            BPF_NOEXIST,
        );
        if !ASSERT_OK(err, c"bpf_map_update_elem".as_ptr()) {
            return -1;
        }
        key += 1;
    }

    0
}

unsafe fn setup(
    map_type: bpf_map_type,
    map_sz: c_int,
    map_fd: *mut c_int,
    populate: c_int,
) -> *mut test_map_init {
    let mut skel: *mut test_map_init;
    let mut err: c_int;

    skel = test_map_init__open();
    if !ASSERT_OK_PTR(skel, c"skel_open".as_ptr()) {
        return ptr::null_mut();
    }

    err = bpf_map__set_type((*skel).maps.hashmap1, map_type);
    if !ASSERT_OK(err, c"bpf_map__set_type".as_ptr()) {
        test_map_init__destroy(skel);
        return ptr::null_mut();
    }

    err = bpf_map__set_max_entries((*skel).maps.hashmap1, map_sz);
    if !ASSERT_OK(err, c"bpf_map__set_max_entries".as_ptr()) {
        test_map_init__destroy(skel);
        return ptr::null_mut();
    }

    err = test_map_init__load(skel);
    if !ASSERT_OK(err, c"skel_load".as_ptr()) {
        test_map_init__destroy(skel);
        return ptr::null_mut();
    }

    *map_fd = bpf_map__fd((*skel).maps.hashmap1);
    if CHECK(*map_fd < 0, c"bpf_map__fd".as_ptr(), c"failed\n".as_ptr()) {
        test_map_init__destroy(skel);
        return ptr::null_mut();
    }

    err = map_populate(*map_fd, populate);
    if !ASSERT_OK(err, c"map_populate".as_ptr()) {
        close(*map_fd);
        test_map_init__destroy(skel);
        return ptr::null_mut();
    }

    skel
}

/* executes bpf program that updates map with key, value */
unsafe fn prog_run_insert_elem(
    skel: *mut test_map_init,
    key: map_key_t,
    value: map_value_t,
) -> c_int {
    let mut bss: *mut test_map_init__bss;

    bss = (*skel).bss;

    (*bss).inKey = key;
    (*bss).inValue = value;
    (*bss).inPid = getpid();

    if !ASSERT_OK(test_map_init__attach(skel), c"skel_attach".as_ptr()) {
        return -1;
    }

    /* Let tracepoint trigger */
    syscall(__NR_getpgid);

    test_map_init__detach(skel);

    0
}

unsafe fn check_values_one_cpu(value: *mut pcpu_map_value_t, expected: map_value_t) -> c_int {
    let mut i: c_int;
    let mut nzCnt: c_int = 0;
    let mut val: map_value_t;

    i = 0;
    while i < nr_cpus {
        val = bpf_percpu(value, i);
        if val != 0 {
            if CHECK(
                val != expected,
                c"map value".as_ptr(),
                c"unexpected for cpu %d: 0x%llx\n".as_ptr(),
            ) {
                return -1;
            }
            nzCnt += 1;
        }
        i += 1;
    }

    if CHECK(
        nzCnt != 1,
        c"map value".as_ptr(),
        c"set for %d CPUs instead of 1!\n".as_ptr(),
    ) {
        return -1;
    }

    0
}

/* Add key=1 elem with values set for all CPUs
 * Delete elem key=1
 * Run bpf prog that inserts new key=1 elem with value=0x1234
 *   (bpf prog can only set value for current CPU)
 * Lookup Key=1 and check value is as expected for all CPUs:
 *   value set by bpf prog for one CPU, 0 for all others
 */
unsafe fn test_pcpu_map_init() {
    let mut value: Vec<pcpu_map_value_t> = (0..nr_cpus)
        .map(|_| pcpu_map_value_t { v: 0 })
        .collect();
    let mut skel: *mut test_map_init;
    let mut map_fd: c_int = 0;
    let mut err: c_int;
    let mut key: map_key_t;

    /* max 1 elem in map so insertion is forced to reuse freed entry */
    skel = setup(BPF_MAP_TYPE_PERCPU_HASH, 1, &mut map_fd, 1);
    if !ASSERT_OK_PTR(skel, c"prog_setup".as_ptr()) {
        return;
    }

    /* delete element so the entry can be re-used*/
    key = 1;
    err = bpf_map_delete_elem(map_fd, &key as *const _ as *const c_void);
    if !ASSERT_OK(err, c"bpf_map_delete_elem".as_ptr()) {
        test_map_init__destroy(skel);
        return;
    }

    /* run bpf prog that inserts new elem, re-using the slot just freed */
    err = prog_run_insert_elem(skel, key, TEST_VALUE);
    if !ASSERT_OK(err, c"prog_run_insert_elem".as_ptr()) {
        test_map_init__destroy(skel);
        return;
    }

    /* check that key=1 was re-created by bpf prog */
    err = bpf_map_lookup_elem(
        map_fd,
        &key as *const _ as *const c_void,
        value.as_mut_ptr() as *mut c_void,
    );
    if !ASSERT_OK(err, c"bpf_map_lookup_elem".as_ptr()) {
        test_map_init__destroy(skel);
        return;
    }

    /* and has expected values */
    check_values_one_cpu(value.as_mut_ptr(), TEST_VALUE);

    test_map_init__destroy(skel);
}

/* Add key=1 and key=2 elems with values set for all CPUs
 * Run bpf prog that inserts new key=3 elem
 *   (only for current cpu; other cpus should have initial value = 0)
 * Lookup Key=1 and check value is as expected for all CPUs
 */
unsafe fn test_pcpu_lru_map_init() {
    let mut value: Vec<pcpu_map_value_t> = (0..nr_cpus)
        .map(|_| pcpu_map_value_t { v: 0 })
        .collect();
    let mut skel: *mut test_map_init;
    let mut map_fd: c_int = 0;
    let mut err: c_int;
    let mut key: map_key_t;

    /* Set up LRU map with 2 elements, values filled for all CPUs.
     * With these 2 elements, the LRU map is full
     */
    skel = setup(BPF_MAP_TYPE_LRU_PERCPU_HASH, 2, &mut map_fd, 2);
    if !ASSERT_OK_PTR(skel, c"prog_setup".as_ptr()) {
        return;
    }

    /* run bpf prog that inserts new key=3 element, re-using LRU slot */
    key = 3;
    err = prog_run_insert_elem(skel, key, TEST_VALUE);
    if !ASSERT_OK(err, c"prog_run_insert_elem".as_ptr()) {
        test_map_init__destroy(skel);
        return;
    }

    /* check that key=3 replaced one of earlier elements */
    err = bpf_map_lookup_elem(
        map_fd,
        &key as *const _ as *const c_void,
        value.as_mut_ptr() as *mut c_void,
    );
    if !ASSERT_OK(err, c"bpf_map_lookup_elem".as_ptr()) {
        test_map_init__destroy(skel);
        return;
    }

    /* and has expected values */
    check_values_one_cpu(value.as_mut_ptr(), TEST_VALUE);

    test_map_init__destroy(skel);
}

pub unsafe fn test_map_init() {
    nr_cpus = bpf_num_possible_cpus();
    if nr_cpus <= 1 {
        printf(c"%s:SKIP: >1 cpu needed for this test\n".as_ptr(), c"test_map_init".as_ptr());
        test__skip();
        return;
    }

    if test__start_subtest(c"pcpu_map_init".as_ptr()) {
        test_pcpu_map_init();
    }
    if test__start_subtest(c"pcpu_lru_map_init".as_ptr()) {
        test_pcpu_lru_map_init();
    }
}

unsafe fn test_map_create(
    map_type: bpf_map_type,
    map_name: *const c_char,
    opts: *mut bpf_map_create_opts,
    exp_msg: *const c_char,
) {
    let key_size: c_int = 4;
    let value_size: c_int = 4;
    let max_entries: c_int = 1;
    let mut log_buf = [0 as c_char; 128];
    let mut fd: c_int;
    let mut log_opts = bpf_log_opts::default();

    log_buf[0] = 0;
    log_opts.buf = log_buf.as_mut_ptr();
    log_opts.size = size_of_val(&log_buf);
    log_opts.level = 1;
    (*opts).log_opts = &mut log_opts;
    fd = bpf_map_create(map_type, map_name, key_size, value_size, max_entries, opts);
    if !ASSERT_LT(fd, 0, c"bpf_map_create".as_ptr()) {
        close(fd);
        return;
    }

    ASSERT_STREQ(log_buf.as_ptr(), exp_msg, c"log_buf".as_ptr());
    ASSERT_EQ(log_opts.true_size, strlen(exp_msg) + 1, c"true_size".as_ptr());
}

unsafe fn test_map_create_array(opts: *mut bpf_map_create_opts, exp_msg: *const c_char) {
    test_map_create(BPF_MAP_TYPE_ARRAY, c"test_map_create".as_ptr(), opts, exp_msg);
}

unsafe fn test_invalid_vmlinux_value_type_id_struct_ops() {
    let msg = c"btf_vmlinux_value_type_id can only be used with struct_ops maps.\n".as_ptr();
    let mut opts = bpf_map_create_opts {
        btf_vmlinux_value_type_id: 1,
        ..Default::default()
    };

    test_map_create_array(&mut opts, msg);
}

unsafe fn test_invalid_vmlinux_value_type_id_kv_type_id() {
    let msg = c"btf_vmlinux_value_type_id is mutually exclusive with btf_key_type_id and btf_value_type_id.\n".as_ptr();
    let mut opts = bpf_map_create_opts {
        btf_vmlinux_value_type_id: 1,
        btf_key_type_id: 1,
        ..Default::default()
    };

    test_map_create(BPF_MAP_TYPE_STRUCT_OPS, c"test_map_create".as_ptr(), &mut opts, msg);
}

unsafe fn test_invalid_value_type_id() {
    let msg = c"Invalid btf_value_type_id.\n".as_ptr();
    let mut opts = bpf_map_create_opts {
        btf_key_type_id: 1,
        ..Default::default()
    };

    test_map_create_array(&mut opts, msg);
}

unsafe fn test_invalid_map_extra() {
    let msg = c"Invalid map_extra.\n".as_ptr();
    let mut opts = bpf_map_create_opts {
        map_extra: 1,
        ..Default::default()
    };

    test_map_create_array(&mut opts, msg);
}

unsafe fn test_invalid_numa_node() {
    let msg = c"Invalid numa_node.\n".as_ptr();
    let mut opts = bpf_map_create_opts {
        map_flags: BPF_F_NUMA_NODE,
        numa_node: 0xFF,
        ..Default::default()
    };

    test_map_create_array(&mut opts, msg);
}

unsafe fn test_invalid_map_type() {
    let msg = c"Invalid map_type.\n".as_ptr();
    let mut opts = bpf_map_create_opts::default();

    test_map_create(__MAX_BPF_MAP_TYPE, c"test_map_create".as_ptr(), &mut opts, msg);
}

unsafe fn test_invalid_token_fd() {
    let msg = c"Invalid map_token_fd.\n".as_ptr();
    let mut opts = bpf_map_create_opts {
        map_flags: BPF_F_TOKEN_FD,
        token_fd: -1,
        ..Default::default()
    };

    test_map_create_array(&mut opts, msg);
}

unsafe fn test_invalid_map_name() {
    let msg = c"Invalid map_name.\n".as_ptr();
    let mut opts = bpf_map_create_opts::default();

    test_map_create(BPF_MAP_TYPE_ARRAY, c"test-!@#".as_ptr(), &mut opts, msg);
}

unsafe fn test_invalid_btf_fd() {
    let msg = c"Invalid btf_fd.\n".as_ptr();
    let mut opts = bpf_map_create_opts {
        btf_fd: -1,
        btf_key_type_id: 1,
        btf_value_type_id: 1,
        ..Default::default()
    };

    test_map_create_array(&mut opts, msg);
}

unsafe fn test_excl_prog_hash_size_1() {
    let msg = c"Invalid excl_prog_hash_size.\n".as_ptr();
    let hash = c"DEADCODE".as_ptr();
    let mut opts = bpf_map_create_opts {
        excl_prog_hash: hash,
        ..Default::default()
    };

    test_map_create_array(&mut opts, msg);
}

unsafe fn test_excl_prog_hash_size_2() {
    let msg = c"Invalid excl_prog_hash_size.\n".as_ptr();
    let mut opts = bpf_map_create_opts {
        excl_prog_hash_size: 1,
        ..Default::default()
    };

    test_map_create_array(&mut opts, msg);
}

unsafe fn test_common_attr_padding() {
    let mut attr_common = bpf_common_attr_fake {
        attrs: [0; offset_of!(bpf_common_attr, log_true_size) + size_of::<u32>()],
        pad: 1,
    };
    let mut attr = bpf_attr {
        map_type: BPF_MAP_TYPE_ARRAY,
    };
    attr.key_size = 4;
    attr.value_size = 4;
    attr.max_entries = 1;
    let mut fd: c_int;

    fd = syscall(
        __NR_bpf,
        BPF_MAP_CREATE | BPF_COMMON_ATTRS,
        &mut attr as *mut _,
        size_of::<bpf_attr>(),
        &mut attr_common as *mut _,
        size_of::<bpf_common_attr_fake>(),
    ) as c_int;
    if !ASSERT_LT(fd, 0, c"syscall".as_ptr()) {
        close(fd);
    } else {
        ASSERT_EQ(errno, E2BIG, c"errno".as_ptr());
    }
}

pub unsafe fn test_map_create_failure() {
    if test__start_subtest(c"invalid_vmlinux_value_type_id_struct_ops".as_ptr()) {
        test_invalid_vmlinux_value_type_id_struct_ops();
    }
    if test__start_subtest(c"invalid_vmlinux_value_type_id_kv_type_id".as_ptr()) {
        test_invalid_vmlinux_value_type_id_kv_type_id();
    }
    if test__start_subtest(c"invalid_value_type_id".as_ptr()) {
        test_invalid_value_type_id();
    }
    if test__start_subtest(c"invalid_map_extra".as_ptr()) {
        test_invalid_map_extra();
    }
    if test__start_subtest(c"invalid_numa_node".as_ptr()) {
        test_invalid_numa_node();
    }
    if test__start_subtest(c"invalid_map_type".as_ptr()) {
        test_invalid_map_type();
    }
    if test__start_subtest(c"invalid_token_fd".as_ptr()) {
        test_invalid_token_fd();
    }
    if test__start_subtest(c"invalid_map_name".as_ptr()) {
        test_invalid_map_name();
    }
    if test__start_subtest(c"invalid_btf_fd".as_ptr()) {
        test_invalid_btf_fd();
    }
    if test__start_subtest(c"invalid_excl_prog_hash_size_1".as_ptr()) {
        test_excl_prog_hash_size_1();
    }
    if test__start_subtest(c"invalid_excl_prog_hash_size_2".as_ptr()) {
        test_excl_prog_hash_size_2();
    }
    if test__start_subtest(c"common_attr_padding".as_ptr()) {
        test_common_attr_padding();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
