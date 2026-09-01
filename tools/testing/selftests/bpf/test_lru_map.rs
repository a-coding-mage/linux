// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2016 Facebook
 */

// C dependencies removed from executable Rust:
// stdio.h, unistd.h, errno.h, string.h, assert.h, sched.h, stdlib.h, time.h,
// sys/wait.h, bpf/bpf.h, bpf/libbpf.h, bpf_util.h, and linux/filter.h.
// The BPF/libc symbols below are expected to be supplied by surrounding bindings.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

const LOCAL_FREE_TARGET: c_uint = 128;
const PERCPU_FREE_TARGET: c_uint = 4;

const BPF_MAP_TYPE_ARRAY: c_int = 2;
const BPF_MAP_TYPE_HASH: c_int = 1;
const BPF_MAP_TYPE_LRU_HASH: c_int = 9;
const BPF_MAP_TYPE_LRU_PERCPU_HASH: c_int = 10;
const BPF_PROG_TYPE_SCHED_CLS: c_int = 3;
const BPF_F_NO_COMMON_LRU: c_int = 1;
const BPF_NOEXIST: c_uint = 1;
const BPF_EXIST: c_uint = 2;
const LIBBPF_STRICT_ALL: c_int = 0xffffffffu32 as c_int;

const EEXIST: c_int = 17;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;

const BPF_REG_0: c_int = 0;
const BPF_REG_1: c_int = 1;
const BPF_REG_2: c_int = 2;
const BPF_REG_3: c_int = 3;
const BPF_REG_9: c_int = 9;
const BPF_REG_10: c_int = 10;
const BPF_ADD: c_int = 0;
const BPF_DW: c_int = 0;
const BPF_JEQ: c_int = 0;
const BPF_JA: c_int = 0;
const BPF_FUNC_map_lookup_elem: c_int = 1;

type pid_t = c_int;

#[repr(C)]
struct bpf_map_create_opts {
    sz: usize,
    map_flags: c_uint,
}

#[repr(C)]
struct bpf_test_run_opts {
    sz: usize,
    data_in: *mut c_void,
    data_size_in: c_uint,
    repeat: c_uint,
    retval: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct bpf_insn {
    code: u8,
    dst_reg_src_reg: u8,
    off: i16,
    imm: i32,
}

#[repr(C)]
struct cpu_set_t {
    bits: [usize; 16],
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut stdout: *mut c_void;

    fn perror(s: *const c_char);
    fn strerror(errnum: c_int) -> *mut c_char;
    fn printf(format: *const c_char, ...) -> c_int;
    fn setbuf(stream: *mut c_void, buf: *mut c_char);
    fn close(fd: c_int) -> c_int;
    fn fork() -> pid_t;
    fn exit(status: c_int) -> !;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn sched_setaffinity(pid: pid_t, cpusetsize: usize, mask: *const cpu_set_t) -> c_int;

    fn bpf_map_create(
        map_type: c_int,
        map_name: *const c_char,
        key_size: c_uint,
        value_size: c_uint,
        max_entries: c_uint,
        opts: *const bpf_map_create_opts,
    ) -> c_int;
    fn bpf_prog_load(
        prog_type: c_int,
        prog_name: *const c_char,
        license: *const c_char,
        insns: *const bpf_insn,
        insn_cnt: usize,
        opts: *const c_void,
    ) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: c_uint) -> c_int;
    fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;
    fn bpf_map_lookup_and_delete_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_get_next_key(fd: c_int, key: *const c_void, next_key: *mut c_void) -> c_int;
    fn bpf_num_possible_cpus() -> c_int;
    fn libbpf_set_strict_mode(mode: c_int);
}

static mut nr_cpus: c_int = 0;

unsafe fn CPU_ZERO(set: *mut cpu_set_t) {
    (*set).bits = [0; 16];
}

unsafe fn CPU_SET(cpu: c_int, set: *mut cpu_set_t) {
    let cpu = cpu as usize;
    let bits_per_word = usize::BITS as usize;
    (*set).bits[cpu / bits_per_word] |= 1usize << (cpu % bits_per_word);
}

fn bpf_ld_map_value(_dst: c_int, _map_fd: c_int, _off: c_int) -> bpf_insn {
    bpf_insn { code: 0, dst_reg_src_reg: 0, off: 0, imm: 0 }
}

fn bpf_ld_map_fd(_dst: c_int, fd: c_int) -> bpf_insn {
    bpf_insn { code: 0, dst_reg_src_reg: 0, off: 0, imm: fd }
}

fn bpf_ld_imm64(_dst: c_int, imm: u64) -> bpf_insn {
    bpf_insn { code: 0, dst_reg_src_reg: 0, off: 0, imm: imm as i32 }
}

fn bpf_mov64_reg(_dst: c_int, _src: c_int) -> bpf_insn {
    bpf_insn { code: 0, dst_reg_src_reg: 0, off: 0, imm: 0 }
}

fn bpf_alu64_imm(_op: c_int, _dst: c_int, imm: c_int) -> bpf_insn {
    bpf_insn { code: 0, dst_reg_src_reg: 0, off: 0, imm }
}

fn bpf_stx_mem(_size: c_int, _dst: c_int, _src: c_int, off: c_int) -> bpf_insn {
    bpf_insn { code: 0, dst_reg_src_reg: 0, off: off as i16, imm: 0 }
}

fn bpf_emit_call(func: c_int) -> bpf_insn {
    bpf_insn { code: 0, dst_reg_src_reg: 0, off: 0, imm: func }
}

fn bpf_jmp_imm(_op: c_int, _dst: c_int, imm: c_int, off: c_int) -> bpf_insn {
    bpf_insn { code: 0, dst_reg_src_reg: 0, off: off as i16, imm }
}

fn bpf_ldx_mem(_size: c_int, _dst: c_int, _src: c_int, off: c_int) -> bpf_insn {
    bpf_insn { code: 0, dst_reg_src_reg: 0, off: off as i16, imm: 0 }
}

fn bpf_mov64_imm(_dst: c_int, imm: c_int) -> bpf_insn {
    bpf_insn { code: 0, dst_reg_src_reg: 0, off: 0, imm }
}

fn bpf_exit_insn() -> bpf_insn {
    bpf_insn { code: 0, dst_reg_src_reg: 0, off: 0, imm: 0 }
}

unsafe fn create_map(map_type: c_int, map_flags: c_int, size: c_uint) -> c_int {
    let opts = bpf_map_create_opts {
        sz: mem::size_of::<bpf_map_create_opts>(),
        map_flags: map_flags as c_uint,
    };
    let map_fd: c_int;

    map_fd = bpf_map_create(
        map_type,
        ptr::null(),
        mem::size_of::<u64>() as c_uint,
        mem::size_of::<u64>() as c_uint,
        size,
        &opts,
    );

    if map_fd == -1 {
        perror(c"bpf_map_create".as_ptr());
    }

    map_fd
}

unsafe fn bpf_map_lookup_elem_with_ref_bit(fd: c_int, key: u64, value: *mut c_void) -> c_int {
    let mut insns = [
        bpf_ld_map_value(BPF_REG_9, 0, 0),
        bpf_ld_map_fd(BPF_REG_1, fd),
        bpf_ld_imm64(BPF_REG_3, key),
        bpf_mov64_reg(BPF_REG_2, BPF_REG_10),
        bpf_alu64_imm(BPF_ADD, BPF_REG_2, -8),
        bpf_stx_mem(BPF_DW, BPF_REG_2, BPF_REG_3, 0),
        bpf_emit_call(BPF_FUNC_map_lookup_elem),
        bpf_jmp_imm(BPF_JEQ, BPF_REG_0, 0, 4),
        bpf_ldx_mem(BPF_DW, BPF_REG_1, BPF_REG_0, 0),
        bpf_stx_mem(BPF_DW, BPF_REG_9, BPF_REG_1, 0),
        bpf_mov64_imm(BPF_REG_0, 42),
        bpf_jmp_imm(BPF_JA, 0, 0, 1),
        bpf_mov64_imm(BPF_REG_0, 1),
        bpf_exit_insn(),
    ];
    let mut data = [0u8; 64];
    let mut zero: c_int = 0;
    let mut topts = bpf_test_run_opts {
        sz: mem::size_of::<bpf_test_run_opts>(),
        data_in: data.as_mut_ptr() as *mut c_void,
        data_size_in: mem::size_of_val(&data) as c_uint,
        repeat: 1,
        retval: 0,
    };

    let mfd = bpf_map_create(
        BPF_MAP_TYPE_ARRAY,
        ptr::null(),
        mem::size_of::<c_int>() as c_uint,
        mem::size_of::<u64>() as c_uint,
        1,
        ptr::null(),
    );
    if mfd < 0 {
        return -1;
    }

    insns[0].imm = mfd;

    let pfd = bpf_prog_load(
        BPF_PROG_TYPE_SCHED_CLS,
        ptr::null(),
        c"GPL".as_ptr(),
        insns.as_ptr(),
        insns.len(),
        ptr::null(),
    );
    if pfd < 0 {
        close(mfd);
        return -1;
    }

    let mut ret = bpf_prog_test_run_opts(pfd, &mut topts);
    if ret < 0 || topts.retval != 42 {
        ret = -1;
    } else {
        assert!(bpf_map_lookup_elem(mfd, &mut zero as *mut _ as *const c_void, value) == 0);
        ret = 0;
    }
    close(pfd);
    close(mfd);
    ret
}

unsafe fn map_subset(map0: c_int, map1: c_int) -> c_int {
    let mut next_key: u64 = 0;
    let mut value0 = vec![0u64; nr_cpus as usize];
    let mut value1 = vec![0u64; nr_cpus as usize];
    let mut ret: c_int;

    while bpf_map_get_next_key(map1, &next_key as *const _ as *const c_void, &mut next_key as *mut _ as *mut c_void) == 0 {
        assert!(bpf_map_lookup_elem(map1, &next_key as *const _ as *const c_void, value1.as_mut_ptr() as *mut c_void) == 0);
        ret = bpf_map_lookup_elem(map0, &next_key as *const _ as *const c_void, value0.as_mut_ptr() as *mut c_void);
        if ret != 0 {
            printf(
                c"key:%llu not found from map. %s(%d)\n".as_ptr(),
                next_key,
                strerror(errno),
                errno,
            );
            return 0;
        }
        if value0[0] != value1[0] {
            printf(
                c"key:%llu value0:%llu != value1:%llu\n".as_ptr(),
                next_key,
                value0[0],
                value1[0],
            );
            return 0;
        }
    }
    1
}

unsafe fn map_equal(lru_map: c_int, expected: c_int) -> c_int {
    ((map_subset(lru_map, expected) != 0) && (map_subset(expected, lru_map) != 0)) as c_int
}

unsafe fn sched_next_online(pid: c_int, next_to_try: *mut c_int) -> c_int {
    let mut cpuset: cpu_set_t = mem::zeroed();
    let mut next = *next_to_try;
    let mut ret = -1;

    while next < nr_cpus {
        CPU_ZERO(&mut cpuset);
        CPU_SET(next, &mut cpuset);
        next += 1;
        if sched_setaffinity(pid, mem::size_of_val(&cpuset), &cpuset) == 0 {
            ret = 0;
            break;
        }
    }

    *next_to_try = next;
    ret
}

/* Derive target_free from map_size, same as bpf_common_lru_populate */
unsafe fn __tgt_size(map_size: c_uint) -> c_uint {
    (map_size / nr_cpus as c_uint) / 2
}

/* Inverse of how bpf_common_lru_populate derives target_free from map_size. */
unsafe fn __map_size(tgt_free: c_uint) -> c_uint {
    tgt_free * nr_cpus as c_uint * 2
}

/* Size of the LRU map is 2
 * Add key=1 (+1 key)
 * Add key=2 (+1 key)
 * Lookup Key=1
 * Add Key=3
 *   => Key=2 will be removed by LRU
 * Iterate map.  Only found key=1 and key=3
 */
unsafe fn test_lru_sanity0(map_type: c_int, map_flags: c_int) {
    let mut key: u64;
    let mut value = vec![0u64; nr_cpus as usize];
    let lru_map_fd: c_int;
    let expected_map_fd: c_int;
    let mut next_cpu = 0;

    printf(c"%s (map_type:%d map_flags:0x%X): ".as_ptr(), c"test_lru_sanity0".as_ptr(), map_type, map_flags);

    assert!(sched_next_online(0, &mut next_cpu) != -1);

    if (map_flags & BPF_F_NO_COMMON_LRU) != 0 {
        lru_map_fd = create_map(map_type, map_flags, (2 * nr_cpus) as c_uint);
    } else {
        lru_map_fd = create_map(map_type, map_flags, 2);
    }
    assert!(lru_map_fd != -1);

    expected_map_fd = create_map(BPF_MAP_TYPE_HASH, 0, 2);
    assert!(expected_map_fd != -1);

    value[0] = 1234;

    /* insert key=1 element */

    key = 1;
    assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
    assert!(bpf_map_update_elem(expected_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);

    /* BPF_NOEXIST means: add new element if it doesn't exist */
    assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == -EEXIST);
    /* key=1 already exists */

    assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, -1i32 as c_uint) == -EINVAL);

    /* insert key=2 element */

    /* check that key=2 is not found */
    key = 2;
    assert!(bpf_map_lookup_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_mut_ptr() as *mut c_void) == -ENOENT);

    /* BPF_EXIST means: update existing element */
    assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_EXIST) == -ENOENT);
    /* key=2 is not there */

    assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);

    /* insert key=3 element */

    /* check that key=3 is not found */
    key = 3;
    assert!(bpf_map_lookup_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_mut_ptr() as *mut c_void) == -ENOENT);

    /* check that key=1 can be found and mark the ref bit to
     * stop LRU from removing key=1
     */
    key = 1;
    assert!(bpf_map_lookup_elem_with_ref_bit(lru_map_fd, key, value.as_mut_ptr() as *mut c_void) == 0);
    assert!(value[0] == 1234);

    key = 3;
    assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
    assert!(bpf_map_update_elem(expected_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);

    /* key=2 has been removed from the LRU */
    key = 2;
    assert!(bpf_map_lookup_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_mut_ptr() as *mut c_void) == -ENOENT);

    /* lookup elem key=1 and delete it, then check it doesn't exist */
    key = 1;
    assert!(bpf_map_lookup_and_delete_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_mut_ptr() as *mut c_void) == 0);
    assert!(value[0] == 1234);

    /* remove the same element from the expected map */
    assert!(bpf_map_delete_elem(expected_map_fd, &key as *const _ as *const c_void) == 0);

    assert!(map_equal(lru_map_fd, expected_map_fd) != 0);

    close(expected_map_fd);
    close(lru_map_fd);

    printf(c"Pass\n".as_ptr());
}

/* Verify that unreferenced elements are recycled before referenced ones.
 * Insert elements.
 * Reference a subset of these.
 * Insert more, enough to trigger recycling.
 * Verify that unreferenced are recycled.
 */
unsafe fn test_lru_sanity1(map_type: c_int, map_flags: c_int, tgt_free: c_uint) {
    let mut key: u64;
    let mut end_key: u64;
    let mut value = vec![0u64; nr_cpus as usize];
    let lru_map_fd: c_int;
    let expected_map_fd: c_int;
    let batch_size: c_uint;
    let map_size: c_uint;
    let mut next_cpu = 0;

    if (map_flags & BPF_F_NO_COMMON_LRU) != 0 {
        /* This test is only applicable to common LRU list */
        return;
    }

    printf(c"%s (map_type:%d map_flags:0x%X): ".as_ptr(), c"test_lru_sanity1".as_ptr(), map_type, map_flags);

    assert!(sched_next_online(0, &mut next_cpu) != -1);

    batch_size = tgt_free / 2;
    assert!(batch_size * 2 == tgt_free);

    map_size = __map_size(tgt_free) + batch_size;
    lru_map_fd = create_map(map_type, map_flags, map_size);
    assert!(lru_map_fd != -1);

    expected_map_fd = create_map(BPF_MAP_TYPE_HASH, 0, map_size);
    assert!(expected_map_fd != -1);

    value[0] = 1234;

    /* Insert map_size - batch_size keys */
    end_key = 1 + __map_size(tgt_free) as u64;
    key = 1;
    while key < end_key {
        assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
        key += 1;
    }

    /* Lookup 1 to batch_size */
    end_key = 1 + batch_size as u64;
    key = 1;
    while key < end_key {
        assert!(bpf_map_lookup_elem_with_ref_bit(lru_map_fd, key, value.as_mut_ptr() as *mut c_void) == 0);
        assert!(bpf_map_update_elem(expected_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
        key += 1;
    }

    /* Insert another map_size - batch_size keys
     * Map will contain 1 to batch_size plus these latest, i.e.,
     * => previous 1+batch_size to map_size - batch_size will have been
     * removed by LRU
     */
    key = 1 + __map_size(tgt_free) as u64;
    end_key = key + __map_size(tgt_free) as u64;
    while key < end_key {
        assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
        assert!(bpf_map_update_elem(expected_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
        key += 1;
    }

    assert!(map_equal(lru_map_fd, expected_map_fd) != 0);

    close(expected_map_fd);
    close(lru_map_fd);

    printf(c"Pass\n".as_ptr());
}

/* Verify that insertions exceeding map size will recycle the oldest.
 * Verify that unreferenced elements are recycled before referenced.
 */
unsafe fn test_lru_sanity2(map_type: c_int, map_flags: c_int, tgt_free: c_uint) {
    let mut key: u64;
    let mut value = vec![0u64; nr_cpus as usize];
    let mut end_key: u64;
    let lru_map_fd: c_int;
    let expected_map_fd: c_int;
    let batch_size: c_uint;
    let map_size: c_uint;
    let mut next_cpu = 0;

    if (map_flags & BPF_F_NO_COMMON_LRU) != 0 {
        /* This test is only applicable to common LRU list */
        return;
    }

    printf(c"%s (map_type:%d map_flags:0x%X): ".as_ptr(), c"test_lru_sanity2".as_ptr(), map_type, map_flags);

    assert!(sched_next_online(0, &mut next_cpu) != -1);

    batch_size = tgt_free / 2;
    assert!(batch_size * 2 == tgt_free);

    map_size = __map_size(tgt_free) + batch_size;
    lru_map_fd = create_map(map_type, map_flags, map_size);
    assert!(lru_map_fd != -1);

    expected_map_fd = create_map(BPF_MAP_TYPE_HASH, 0, map_size);
    assert!(expected_map_fd != -1);

    value[0] = 1234;

    /* Insert map_size - batch_size keys */
    end_key = 1 + __map_size(tgt_free) as u64;
    key = 1;
    while key < end_key {
        assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
        key += 1;
    }

    /* Any bpf_map_update_elem will require to acquire a new node
     * from LRU first.
     *
     * The local list is running out of free nodes.
     * It gets from the global LRU list which tries to
     * shrink the inactive list to get tgt_free
     * number of free nodes.
     *
     * Hence, the oldest key is removed from the LRU list.
     */
    key = 1;
    if map_type == BPF_MAP_TYPE_LRU_PERCPU_HASH {
        assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
        assert!(bpf_map_delete_elem(lru_map_fd, &key as *const _ as *const c_void) == 0);
    } else {
        assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_EXIST) != 0);
    }

    /* Re-insert 1 to batch_size again and do a lookup immediately.
     */
    end_key = 1 + batch_size as u64;
    value[0] = 4321;
    key = 1;
    while key < end_key {
        assert!(bpf_map_lookup_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_mut_ptr() as *mut c_void) == -ENOENT);
        assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
        assert!(bpf_map_lookup_elem_with_ref_bit(lru_map_fd, key, value.as_mut_ptr() as *mut c_void) == 0);
        assert!(value[0] == 4321);
        assert!(bpf_map_update_elem(expected_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
        key += 1;
    }

    value[0] = 1234;

    /* Insert batch_size new elements */
    key = 1 + __map_size(tgt_free) as u64;
    end_key = key + batch_size as u64;
    while key < end_key {
        /* These newly added but not referenced keys will be
         * gone during the next LRU shrink.
         */
        assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
        key += 1;
    }

    /* Insert map_size - batch_size elements */
    end_key += __map_size(tgt_free) as u64;
    while key < end_key {
        assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
        assert!(bpf_map_update_elem(expected_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
        key += 1;
    }

    assert!(map_equal(lru_map_fd, expected_map_fd) != 0);

    close(expected_map_fd);
    close(lru_map_fd);

    printf(c"Pass\n".as_ptr());
}

/* Test the active/inactive list rotation
 *
 * Fill the whole map, deplete the free list.
 * Reference all except the last lru->target_free elements.
 * Insert lru->target_free new elements. This triggers one shrink.
 * Verify that the non-referenced elements are replaced.
 */
unsafe fn test_lru_sanity3(map_type: c_int, map_flags: c_int, tgt_free: c_uint) {
    let mut key: u64;
    let mut end_key: u64;
    let mut value = vec![0u64; nr_cpus as usize];
    let lru_map_fd: c_int;
    let expected_map_fd: c_int;
    let batch_size: c_uint;
    let map_size: c_uint;
    let mut next_cpu = 0;

    if (map_flags & BPF_F_NO_COMMON_LRU) != 0 {
        /* This test is only applicable to common LRU list */
        return;
    }

    printf(c"%s (map_type:%d map_flags:0x%X): ".as_ptr(), c"test_lru_sanity3".as_ptr(), map_type, map_flags);

    assert!(sched_next_online(0, &mut next_cpu) != -1);

    batch_size = __tgt_size(tgt_free);

    map_size = tgt_free * 2;
    lru_map_fd = create_map(map_type, map_flags, map_size);
    assert!(lru_map_fd != -1);

    expected_map_fd = create_map(BPF_MAP_TYPE_HASH, 0, map_size);
    assert!(expected_map_fd != -1);

    value[0] = 1234;

    /* Fill the map */
    end_key = 1 + map_size as u64;
    key = 1;
    while key < end_key {
        assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
        key += 1;
    }

    /* Reference all but the last batch_size */
    end_key = 1 + map_size as u64 - batch_size as u64;
    key = 1;
    while key < end_key {
        assert!(bpf_map_lookup_elem_with_ref_bit(lru_map_fd, key, value.as_mut_ptr() as *mut c_void) == 0);
        assert!(bpf_map_update_elem(expected_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
        key += 1;
    }

    /* Insert new batch_size: replaces the non-referenced elements */
    key = 2 * tgt_free as u64 + 1;
    end_key = key + batch_size as u64;
    while key < end_key {
        assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
        assert!(bpf_map_update_elem(expected_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
        key += 1;
    }

    assert!(map_equal(lru_map_fd, expected_map_fd) != 0);

    close(expected_map_fd);
    close(lru_map_fd);

    printf(c"Pass\n".as_ptr());
}

/* Test deletion */
unsafe fn test_lru_sanity4(map_type: c_int, map_flags: c_int, tgt_free: c_uint) {
    let lru_map_fd: c_int;
    let expected_map_fd: c_int;
    let mut key: u64;
    let mut value = vec![0u64; nr_cpus as usize];
    let end_key: u64;
    let mut next_cpu = 0;

    printf(c"%s (map_type:%d map_flags:0x%X): ".as_ptr(), c"test_lru_sanity4".as_ptr(), map_type, map_flags);

    assert!(sched_next_online(0, &mut next_cpu) != -1);

    if (map_flags & BPF_F_NO_COMMON_LRU) != 0 {
        lru_map_fd = create_map(map_type, map_flags, 3 * tgt_free * nr_cpus as c_uint);
    } else {
        lru_map_fd = create_map(map_type, map_flags, 3 * __map_size(tgt_free));
    }
    assert!(lru_map_fd != -1);

    expected_map_fd = create_map(BPF_MAP_TYPE_HASH, 0, 3 * tgt_free);
    assert!(expected_map_fd != -1);

    value[0] = 1234;

    key = 1;
    while key <= 2 * tgt_free as u64 {
        assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
        key += 1;
    }

    key = 1;
    assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) != 0);

    key = 1;
    while key <= tgt_free as u64 {
        assert!(bpf_map_lookup_elem_with_ref_bit(lru_map_fd, key, value.as_mut_ptr() as *mut c_void) == 0);
        assert!(bpf_map_update_elem(expected_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
        key += 1;
    }

    while key <= 2 * tgt_free as u64 {
        assert!(bpf_map_delete_elem(lru_map_fd, &key as *const _ as *const c_void) == 0);
        assert!(bpf_map_delete_elem(lru_map_fd, &key as *const _ as *const c_void) != 0);
        key += 1;
    }

    end_key = key + 2 * tgt_free as u64;
    while key < end_key {
        assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
        assert!(bpf_map_update_elem(expected_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
        key += 1;
    }

    assert!(map_equal(lru_map_fd, expected_map_fd) != 0);

    close(expected_map_fd);
    close(lru_map_fd);

    printf(c"Pass\n".as_ptr());
}

unsafe fn do_test_lru_sanity5(last_key: u64, map_fd: c_int) {
    let mut key: u64;
    let mut value = vec![0u64; nr_cpus as usize];

    /* Ensure the last key inserted by previous CPU can be found */
    assert!(bpf_map_lookup_elem_with_ref_bit(map_fd, last_key, value.as_mut_ptr() as *mut c_void) == 0);
    value[0] = 1234;

    key = last_key + 1;
    assert!(bpf_map_update_elem(map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
    assert!(bpf_map_lookup_elem_with_ref_bit(map_fd, key, value.as_mut_ptr() as *mut c_void) == 0);

    /* Cannot find the last key because it was removed by LRU */
    assert!(bpf_map_lookup_elem(map_fd, &last_key as *const _ as *const c_void, value.as_mut_ptr() as *mut c_void) == -ENOENT);
}

/* Test map with only one element */
unsafe fn test_lru_sanity5(map_type: c_int, map_flags: c_int) {
    let mut key: u64;
    let mut value = vec![0u64; nr_cpus as usize];
    let mut next_cpu = 0;
    let map_fd: c_int;

    if (map_flags & BPF_F_NO_COMMON_LRU) != 0 {
        return;
    }

    printf(c"%s (map_type:%d map_flags:0x%X): ".as_ptr(), c"test_lru_sanity5".as_ptr(), map_type, map_flags);

    map_fd = create_map(map_type, map_flags, 1);
    assert!(map_fd != -1);

    value[0] = 1234;
    key = 0;
    assert!(bpf_map_update_elem(map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);

    while sched_next_online(0, &mut next_cpu) != -1 {
        let pid: pid_t;

        pid = fork();
        if pid == 0 {
            do_test_lru_sanity5(key, map_fd);
            exit(0);
        } else if pid == -1 {
            printf(c"couldn't spawn process to test key:%llu\n".as_ptr(), key);
            exit(1);
        } else {
            let mut status: c_int = 0;

            assert!(waitpid(pid, &mut status, 0) == pid);
            assert!(status == 0);
            key += 1;
        }
    }

    close(map_fd);
    /* At least one key should be tested */
    assert!(key > 0);

    printf(c"Pass\n".as_ptr());
}

/* Test list rotation for BPF_F_NO_COMMON_LRU map */
unsafe fn test_lru_sanity6(map_type: c_int, map_flags: c_int, tgt_free: c_int) {
    let lru_map_fd: c_int;
    let expected_map_fd: c_int;
    let mut key: u64;
    let mut value = vec![0u64; nr_cpus as usize];
    let map_size: c_uint = (tgt_free * 2) as c_uint;
    let mut next_cpu = 0;

    if (map_flags & BPF_F_NO_COMMON_LRU) == 0 {
        return;
    }

    printf(c"%s (map_type:%d map_flags:0x%X): ".as_ptr(), c"test_lru_sanity6".as_ptr(), map_type, map_flags);

    assert!(sched_next_online(0, &mut next_cpu) != -1);

    expected_map_fd = create_map(BPF_MAP_TYPE_HASH, 0, map_size);
    assert!(expected_map_fd != -1);

    lru_map_fd = create_map(map_type, map_flags, map_size * nr_cpus as c_uint);
    assert!(lru_map_fd != -1);

    value[0] = 1234;

    key = 1;
    while key <= tgt_free as u64 {
        assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
        assert!(bpf_map_update_elem(expected_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
        key += 1;
    }

    while key <= (tgt_free * 2) as u64 {
        let mut stable_key: u64;

        /* Make ref bit sticky for key: [1, tgt_free] */
        stable_key = 1;
        while stable_key <= tgt_free as u64 {
            /* Mark the ref bit */
            assert!(bpf_map_lookup_elem_with_ref_bit(lru_map_fd, stable_key, value.as_mut_ptr() as *mut c_void) == 0);
            stable_key += 1;
        }
        assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
        key += 1;
    }

    while key <= (tgt_free * 3) as u64 {
        assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
        assert!(bpf_map_update_elem(expected_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
        key += 1;
    }

    assert!(map_equal(lru_map_fd, expected_map_fd) != 0);

    close(expected_map_fd);
    close(lru_map_fd);

    printf(c"Pass\n".as_ptr());
}

/* Size of the LRU map is 2
 * Add key=1 (+1 key)
 * Add key=2 (+1 key)
 * Lookup Key=1 (datapath)
 * Lookup Key=2 (syscall)
 * Add Key=3
 *   => Key=2 will be removed by LRU
 * Iterate map.  Only found key=1 and key=3
 */
unsafe fn test_lru_sanity7(map_type: c_int, map_flags: c_int) {
    let mut key: u64;
    let mut value = vec![0u64; nr_cpus as usize];
    let lru_map_fd: c_int;
    let expected_map_fd: c_int;
    let mut next_cpu = 0;

    printf(c"%s (map_type:%d map_flags:0x%X): ".as_ptr(), c"test_lru_sanity7".as_ptr(), map_type, map_flags);

    assert!(sched_next_online(0, &mut next_cpu) != -1);

    if (map_flags & BPF_F_NO_COMMON_LRU) != 0 {
        lru_map_fd = create_map(map_type, map_flags, (2 * nr_cpus) as c_uint);
    } else {
        lru_map_fd = create_map(map_type, map_flags, 2);
    }
    assert!(lru_map_fd != -1);

    expected_map_fd = create_map(BPF_MAP_TYPE_HASH, 0, 2);
    assert!(expected_map_fd != -1);

    value[0] = 1234;

    /* insert key=1 element */

    key = 1;
    assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
    assert!(bpf_map_update_elem(expected_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);

    /* BPF_NOEXIST means: add new element if it doesn't exist */
    assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == -EEXIST);
    /* key=1 already exists */

    /* insert key=2 element */

    /* check that key=2 is not found */
    key = 2;
    assert!(bpf_map_lookup_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_mut_ptr() as *mut c_void) == -ENOENT);

    /* BPF_EXIST means: update existing element */
    assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_EXIST) == -ENOENT);
    /* key=2 is not there */

    assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);

    /* insert key=3 element */

    /* check that key=3 is not found */
    key = 3;
    assert!(bpf_map_lookup_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_mut_ptr() as *mut c_void) == -ENOENT);

    /* check that key=1 can be found and mark the ref bit to
     * stop LRU from removing key=1
     */
    key = 1;
    assert!(bpf_map_lookup_elem_with_ref_bit(lru_map_fd, key, value.as_mut_ptr() as *mut c_void) == 0);
    assert!(value[0] == 1234);

    /* check that key=2 can be found and do _not_ mark ref bit.
     * this will be evicted on next update.
     */
    key = 2;
    assert!(bpf_map_lookup_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_mut_ptr() as *mut c_void) == 0);
    assert!(value[0] == 1234);

    key = 3;
    assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
    assert!(bpf_map_update_elem(expected_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);

    /* key=2 has been removed from the LRU */
    key = 2;
    assert!(bpf_map_lookup_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_mut_ptr() as *mut c_void) == -ENOENT);

    assert!(map_equal(lru_map_fd, expected_map_fd) != 0);

    close(expected_map_fd);
    close(lru_map_fd);

    printf(c"Pass\n".as_ptr());
}

/* Size of the LRU map is 2
 * Add key=1 (+1 key)
 * Add key=2 (+1 key)
 * Lookup Key=1 (syscall)
 * Lookup Key=2 (datapath)
 * Add Key=3
 *   => Key=1 will be removed by LRU
 * Iterate map.  Only found key=2 and key=3
 */
unsafe fn test_lru_sanity8(map_type: c_int, map_flags: c_int) {
    let mut key: u64;
    let mut value = vec![0u64; nr_cpus as usize];
    let lru_map_fd: c_int;
    let expected_map_fd: c_int;
    let mut next_cpu = 0;

    printf(c"%s (map_type:%d map_flags:0x%X): ".as_ptr(), c"test_lru_sanity8".as_ptr(), map_type, map_flags);

    assert!(sched_next_online(0, &mut next_cpu) != -1);

    if (map_flags & BPF_F_NO_COMMON_LRU) != 0 {
        lru_map_fd = create_map(map_type, map_flags, (2 * nr_cpus) as c_uint);
    } else {
        lru_map_fd = create_map(map_type, map_flags, 2);
    }
    assert!(lru_map_fd != -1);

    expected_map_fd = create_map(BPF_MAP_TYPE_HASH, 0, 2);
    assert!(expected_map_fd != -1);

    value[0] = 1234;

    /* insert key=1 element */

    key = 1;
    assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);

    /* BPF_NOEXIST means: add new element if it doesn't exist */
    assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == -EEXIST);
    /* key=1 already exists */

    /* insert key=2 element */

    /* check that key=2 is not found */
    key = 2;
    assert!(bpf_map_lookup_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_mut_ptr() as *mut c_void) == -ENOENT);

    /* BPF_EXIST means: update existing element */
    assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_EXIST) == -ENOENT);
    /* key=2 is not there */

    assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
    assert!(bpf_map_update_elem(expected_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);

    /* insert key=3 element */

    /* check that key=3 is not found */
    key = 3;
    assert!(bpf_map_lookup_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_mut_ptr() as *mut c_void) == -ENOENT);

    /* check that key=1 can be found and do _not_ mark ref bit.
     * this will be evicted on next update.
     */
    key = 1;
    assert!(bpf_map_lookup_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_mut_ptr() as *mut c_void) == 0);
    assert!(value[0] == 1234);

    /* check that key=2 can be found and mark the ref bit to
     * stop LRU from removing key=2
     */
    key = 2;
    assert!(bpf_map_lookup_elem_with_ref_bit(lru_map_fd, key, value.as_mut_ptr() as *mut c_void) == 0);
    assert!(value[0] == 1234);

    key = 3;
    assert!(bpf_map_update_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);
    assert!(bpf_map_update_elem(expected_map_fd, &key as *const _ as *const c_void, value.as_ptr() as *const c_void, BPF_NOEXIST) == 0);

    /* key=1 has been removed from the LRU */
    key = 1;
    assert!(bpf_map_lookup_elem(lru_map_fd, &key as *const _ as *const c_void, value.as_mut_ptr() as *mut c_void) == -ENOENT);

    assert!(map_equal(lru_map_fd, expected_map_fd) != 0);

    close(expected_map_fd);
    close(lru_map_fd);

    printf(c"Pass\n".as_ptr());
}

unsafe fn main_impl(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let map_types = [BPF_MAP_TYPE_LRU_HASH, BPF_MAP_TYPE_LRU_PERCPU_HASH];
    let map_flags = [0, BPF_F_NO_COMMON_LRU];
    let mut t: usize;
    let mut f: usize;

    setbuf(stdout, ptr::null_mut());

    nr_cpus = bpf_num_possible_cpus();
    assert!(nr_cpus != -1);
    printf(c"nr_cpus:%d\n\n".as_ptr(), nr_cpus);

    /* Use libbpf 1.0 API mode */
    libbpf_set_strict_mode(LIBBPF_STRICT_ALL);

    f = 0;
    while f < map_flags.len() {
        let tgt_free: c_uint = if (map_flags[f] & BPF_F_NO_COMMON_LRU) != 0 {
            PERCPU_FREE_TARGET
        } else {
            LOCAL_FREE_TARGET
        };

        t = 0;
        while t < map_types.len() {
            test_lru_sanity0(map_types[t], map_flags[f]);
            test_lru_sanity1(map_types[t], map_flags[f], tgt_free);
            test_lru_sanity2(map_types[t], map_flags[f], tgt_free);
            test_lru_sanity3(map_types[t], map_flags[f], tgt_free);
            test_lru_sanity4(map_types[t], map_flags[f], tgt_free);
            test_lru_sanity5(map_types[t], map_flags[f]);
            test_lru_sanity6(map_types[t], map_flags[f], tgt_free as c_int);
            test_lru_sanity7(map_types[t], map_flags[f]);
            test_lru_sanity8(map_types[t], map_flags[f]);

            printf(c"\n".as_ptr());
            t += 1;
        }
        f += 1;
    }

    0
}

fn main() {
    unsafe {
        main_impl(0, ptr::null_mut());
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
