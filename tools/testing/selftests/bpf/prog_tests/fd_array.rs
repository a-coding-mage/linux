// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <test_progs.h>
// #include <linux/btf.h>
// #include <bpf/bpf.h>
// #include "../test_btf.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type __u32 = u32;
type __u64 = u64;

#[repr(C)]
struct btf_header {
    magic: __u32,
    version: __u32,
    flags: __u32,
    hdr_len: __u32,
    type_off: __u32,
    type_len: __u32,
    str_off: __u32,
    str_len: __u32,
}

#[repr(C)]
struct bpf_prog_info {
    nr_map_ids: __u32,
    map_ids: __u64,
}

#[repr(C)]
struct bpf_btf_info {
    id: __u32,
}

#[repr(C)]
struct bpf_insn {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_prog_load_opts {
    fd_array: *const c_int,
    fd_array_cnt: c_int,
}

extern "C" {
    static BPF_MAP_TYPE_ARRAY: c_uint;
    static BPF_PROG_TYPE_XDP: c_uint;
    static BTF_MAGIC: __u32;
    static BTF_VERSION: __u32;
    static BTF_INT_SIGNED: __u32;
    static BPF_REG_1: c_uint;
    static BPF_REG_2: c_uint;
    static BPF_REG_10: c_uint;
    static BPF_DW: c_uint;
    static BPF_ADD: c_uint;
    static BPF_JMP: c_uint;
    static BPF_CALL: c_uint;
    static BPF_FUNC_map_lookup_elem: c_uint;
    static AF_INET: c_int;
    static SOCK_STREAM: c_int;
    static EBADF: c_int;
    static EINVAL: c_int;
    static E2BIG: c_int;

    fn bpf_map_create(
        map_type: c_uint,
        name: *const c_char,
        key_size: __u32,
        value_size: __u32,
        max_entries: __u32,
        opts: *const c_void,
    ) -> c_int;
    fn bpf_btf_load(raw_btf: *const c_void, raw_btf_size: usize, opts: *const c_void) -> c_int;
    fn bpf_map_get_fd_by_id(id: __u32) -> c_int;
    fn bpf_btf_get_fd_by_id(id: __u32) -> c_int;
    fn bpf_prog_get_info_by_fd(prog_fd: c_int, info: *mut bpf_prog_info, len: *mut __u32) -> c_int;
    fn bpf_btf_get_info_by_fd(btf_fd: c_int, info: *mut bpf_btf_info, info_len: *mut __u32) -> c_int;
    fn bpf_prog_load(
        prog_type: c_uint,
        prog_name: *const c_char,
        license: *const c_char,
        insns: *const bpf_insn,
        insn_cnt: usize,
        opts: *mut bpf_prog_load_opts,
    ) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn ptr_to_u64(ptr: *const __u32) -> __u64;
    fn kern_sync_rcu() -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ<T>(actual: T, expected: T, name: *const c_char) -> bool;
    fn PRINT_FAIL(fmt: *const c_char, ...);

    fn BTF_TYPE_INT_ENC(name: __u32, encoding: __u32, offset: __u32, bits: __u32, sz: __u32) -> __u32;
    fn BPF_LD_MAP_FD(dst: c_uint, fd: c_int) -> bpf_insn;
    fn BPF_ST_MEM(size: c_uint, dst: c_uint, off: c_int, imm: c_int) -> bpf_insn;
    fn BPF_MOV64_REG(dst: c_uint, src: c_uint) -> bpf_insn;
    fn BPF_ALU64_IMM(op: c_uint, dst: c_uint, imm: c_int) -> bpf_insn;
    fn BPF_RAW_INSN(code: c_uint, dst: c_uint, src: c_uint, off: c_int, imm: c_uint) -> bpf_insn;
    fn BPF_MOV64_IMM(dst: c_uint, imm: c_int) -> bpf_insn;
    fn BPF_EXIT_INSN() -> bpf_insn;
}

unsafe fn new_map() -> c_int {
    let name: *const c_char = ptr::null();
    let max_entries: __u32 = 1;
    let value_size: __u32 = 8;
    let key_size: __u32 = 4;

    bpf_map_create(
        BPF_MAP_TYPE_ARRAY,
        name,
        key_size,
        value_size,
        max_entries,
        ptr::null(),
    )
}

unsafe fn new_btf() -> c_int {
    #[repr(C)]
    struct btf_blob {
        btf_hdr: btf_header,
        types: [__u32; 8],
        str_: __u32,
    }

    let mut raw_btf = btf_blob {
        btf_hdr: btf_header {
            magic: BTF_MAGIC,
            version: BTF_VERSION,
            flags: 0,
            hdr_len: size_of::<btf_header>() as __u32,
            type_off: 0,
            type_len: size_of::<[__u32; 8]>() as __u32,
            str_off: (offset_of!(btf_blob, str_) - offset_of!(btf_blob, types)) as __u32,
            str_len: size_of::<__u32>() as __u32,
        },
        types: [
            /* long */
            BTF_TYPE_INT_ENC(0, BTF_INT_SIGNED, 0, 64, 8), /* [1] */
            /* unsigned long */
            BTF_TYPE_INT_ENC(0, 0, 0, 64, 8), /* [2] */
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        str_: 0,
    };

    bpf_btf_load(
        &mut raw_btf as *mut btf_blob as *const c_void,
        size_of::<btf_blob>(),
        ptr::null(),
    )
}

unsafe fn Close(fd: &mut c_int) {
    if *fd >= 0 {
        close(*fd);
        *fd = -1;
    }
}

unsafe fn map_exists(id: __u32) -> bool {
    let fd: c_int;

    fd = bpf_map_get_fd_by_id(id);
    if fd >= 0 {
        close(fd);
        return true;
    }
    false
}

unsafe fn btf_exists(id: __u32) -> bool {
    let fd: c_int;

    fd = bpf_btf_get_fd_by_id(id);
    if fd >= 0 {
        close(fd);
        return true;
    }
    false
}

unsafe fn bpf_prog_get_map_ids(prog_fd: c_int, nr_map_ids: *mut __u32, map_ids: *mut __u32) -> c_int {
    let mut len: __u32 = size_of::<bpf_prog_info>() as __u32;
    let mut info: bpf_prog_info = core::mem::zeroed();
    let err: c_int;

    memset(
        &mut info as *mut bpf_prog_info as *mut c_void,
        0,
        len as usize,
    );
    info.nr_map_ids = *nr_map_ids;
    info.map_ids = ptr_to_u64(map_ids);

    err = bpf_prog_get_info_by_fd(prog_fd, &mut info, &mut len);
    if !ASSERT_OK(err, c"bpf_prog_get_info_by_fd".as_ptr()) {
        return -1;
    }

    *nr_map_ids = info.nr_map_ids;

    0
}

unsafe fn __load_test_prog(map_fd: c_int, fd_array: *const c_int, fd_array_cnt: c_int) -> c_int {
    /* A trivial program which uses one map */
    let mut insns: [bpf_insn; 7] = [
        BPF_LD_MAP_FD(BPF_REG_1, map_fd),
        BPF_ST_MEM(BPF_DW, BPF_REG_10, -8, 0),
        BPF_MOV64_REG(BPF_REG_2, BPF_REG_10),
        BPF_ALU64_IMM(BPF_ADD, BPF_REG_2, -8),
        BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_map_lookup_elem),
        BPF_MOV64_IMM(BPF_REG_0, 0),
        BPF_EXIT_INSN(),
    ];
    let mut opts: bpf_prog_load_opts = core::mem::zeroed();

    opts.fd_array = fd_array;
    opts.fd_array_cnt = fd_array_cnt;

    bpf_prog_load(
        BPF_PROG_TYPE_XDP,
        ptr::null(),
        c"GPL".as_ptr(),
        insns.as_mut_ptr(),
        insns.len(),
        &mut opts,
    )
}

unsafe fn load_test_prog(fd_array: *const c_int, fd_array_cnt: c_int) -> c_int {
    let map_fd: c_int;
    let ret: c_int;

    map_fd = new_map();
    if !ASSERT_GE(map_fd, 0, c"new_map".as_ptr()) {
        return map_fd;
    }

    ret = __load_test_prog(map_fd, fd_array, fd_array_cnt);
    close(map_fd);
    ret
}

unsafe fn check_expected_map_ids(
    prog_fd: c_int,
    expected: c_int,
    map_ids: *mut __u32,
    nr_map_ids: *mut __u32,
) -> bool {
    let err: c_int;

    err = bpf_prog_get_map_ids(prog_fd, nr_map_ids, map_ids);
    if !ASSERT_OK(err, c"bpf_prog_get_map_ids".as_ptr()) {
        return false;
    }
    if !ASSERT_EQ(*nr_map_ids, expected as __u32, c"unexpected nr_map_ids".as_ptr()) {
        return false;
    }

    true
}

/*
 * Load a program, which uses one map. No fd_array maps are present.
 * On return only one map is expected to be bound to prog.
 */
unsafe fn check_fd_array_cnt__no_fd_array() {
    let mut map_ids: [__u32; 16] = [0; 16];
    let mut nr_map_ids: __u32;
    let mut prog_fd: c_int = -1;

    prog_fd = load_test_prog(ptr::null(), 0);
    if !ASSERT_GE(prog_fd, 0, c"BPF_PROG_LOAD".as_ptr()) {
        return;
    }
    nr_map_ids = map_ids.len() as __u32;
    check_expected_map_ids(prog_fd, 1, map_ids.as_mut_ptr(), &mut nr_map_ids);
    close(prog_fd);
}

/*
 * Load a program, which uses one map, and pass two extra, non-equal, maps in
 * fd_array with fd_array_cnt=2. On return three maps are expected to be bound
 * to the program.
 */
unsafe fn check_fd_array_cnt__fd_array_ok() {
    let mut extra_fds: [c_int; 2] = [-1, -1];
    let mut map_ids: [__u32; 16] = [0; 16];
    let mut nr_map_ids: __u32;
    let mut prog_fd: c_int = -1;

    'cleanup: {
        extra_fds[0] = new_map();
        if !ASSERT_GE(extra_fds[0], 0, c"new_map".as_ptr()) {
            break 'cleanup;
        }
        extra_fds[1] = new_map();
        if !ASSERT_GE(extra_fds[1], 0, c"new_map".as_ptr()) {
            break 'cleanup;
        }
        prog_fd = load_test_prog(extra_fds.as_ptr(), 2);
        if !ASSERT_GE(prog_fd, 0, c"BPF_PROG_LOAD".as_ptr()) {
            break 'cleanup;
        }
        nr_map_ids = map_ids.len() as __u32;
        if !check_expected_map_ids(prog_fd, 3, map_ids.as_mut_ptr(), &mut nr_map_ids) {
            break 'cleanup;
        }

        /* maps should still exist when original file descriptors are closed */
        Close(&mut extra_fds[0]);
        Close(&mut extra_fds[1]);
        if !ASSERT_EQ(map_exists(map_ids[0]), true, c"map_ids[0] should exist".as_ptr()) {
            break 'cleanup;
        }
        if !ASSERT_EQ(map_exists(map_ids[1]), true, c"map_ids[1] should exist".as_ptr()) {
            break 'cleanup;
        }
    }

    /* some fds might be invalid, so ignore return codes */
    Close(&mut extra_fds[1]);
    Close(&mut extra_fds[0]);
    Close(&mut prog_fd);
}

/*
 * Load a program with a few extra maps duplicated in the fd_array.
 * After the load maps should only be referenced once.
 */
unsafe fn check_fd_array_cnt__duplicated_maps() {
    let mut extra_fds: [c_int; 4] = [-1, -1, -1, -1];
    let mut map_ids: [__u32; 16] = [0; 16];
    let mut nr_map_ids: __u32;
    let mut prog_fd: c_int = -1;

    'cleanup: {
        extra_fds[2] = new_map();
        extra_fds[0] = extra_fds[2];
        if !ASSERT_GE(extra_fds[0], 0, c"new_map".as_ptr()) {
            break 'cleanup;
        }
        extra_fds[3] = new_map();
        extra_fds[1] = extra_fds[3];
        if !ASSERT_GE(extra_fds[1], 0, c"new_map".as_ptr()) {
            break 'cleanup;
        }
        prog_fd = load_test_prog(extra_fds.as_ptr(), 4);
        if !ASSERT_GE(prog_fd, 0, c"BPF_PROG_LOAD".as_ptr()) {
            break 'cleanup;
        }
        nr_map_ids = map_ids.len() as __u32;
        if !check_expected_map_ids(prog_fd, 3, map_ids.as_mut_ptr(), &mut nr_map_ids) {
            break 'cleanup;
        }

        /* maps should still exist when original file descriptors are closed */
        Close(&mut extra_fds[0]);
        Close(&mut extra_fds[1]);
        if !ASSERT_EQ(map_exists(map_ids[0]), true, c"map should exist".as_ptr()) {
            break 'cleanup;
        }
        if !ASSERT_EQ(map_exists(map_ids[1]), true, c"map should exist".as_ptr()) {
            break 'cleanup;
        }
    }

    /* some fds might be invalid, so ignore return codes */
    Close(&mut extra_fds[1]);
    Close(&mut extra_fds[0]);
    Close(&mut prog_fd);
}

/*
 * Check that if maps which are referenced by a program are
 * passed in fd_array, then they will be referenced only once
 */
unsafe fn check_fd_array_cnt__referenced_maps_in_fd_array() {
    let mut extra_fds: [c_int; 1] = [-1];
    let mut map_ids: [__u32; 16] = [0; 16];
    let mut nr_map_ids: __u32;
    let mut prog_fd: c_int = -1;

    'cleanup: {
        extra_fds[0] = new_map();
        if !ASSERT_GE(extra_fds[0], 0, c"new_map".as_ptr()) {
            break 'cleanup;
        }
        prog_fd = __load_test_prog(extra_fds[0], extra_fds.as_ptr(), 1);
        if !ASSERT_GE(prog_fd, 0, c"BPF_PROG_LOAD".as_ptr()) {
            break 'cleanup;
        }
        nr_map_ids = map_ids.len() as __u32;
        if !check_expected_map_ids(prog_fd, 1, map_ids.as_mut_ptr(), &mut nr_map_ids) {
            break 'cleanup;
        }

        /* map should still exist when original file descriptor is closed */
        Close(&mut extra_fds[0]);
        if !ASSERT_EQ(map_exists(map_ids[0]), true, c"map should exist".as_ptr()) {
            break 'cleanup;
        }
    }

    /* some fds might be invalid, so ignore return codes */
    Close(&mut extra_fds[0]);
    Close(&mut prog_fd);
}

unsafe fn get_btf_id_by_fd(btf_fd: c_int, id: *mut __u32) -> c_int {
    let mut info: bpf_btf_info = core::mem::zeroed();
    let mut info_len: __u32 = size_of::<bpf_btf_info>() as __u32;
    let err: c_int;

    memset(
        &mut info as *mut bpf_btf_info as *mut c_void,
        0,
        info_len as usize,
    );
    err = bpf_btf_get_info_by_fd(btf_fd, &mut info, &mut info_len);
    if err != 0 {
        return err;
    }
    if !id.is_null() {
        *id = info.id;
    }
    0
}

/*
 * Check that fd_array operates properly for btfs. Namely, to check that
 * passing a btf fd in fd_array increases its reference count, do the
 * following:
 *  1) Create a new btf, it's referenced only by a file descriptor, so refcnt=1
 *  2) Load a BPF prog with fd_array[0] = btf_fd; now btf's refcnt=2
 *  3) Close the btf_fd, now refcnt=1
 * Wait and check that BTF still exists.
 */
unsafe fn check_fd_array_cnt__referenced_btfs() {
    let mut extra_fds: [c_int; 1] = [-1];
    let mut prog_fd: c_int = -1;
    let mut btf_id: __u32 = 0;
    let mut tries: c_int;
    let err: c_int;

    'cleanup: {
        extra_fds[0] = new_btf();
        if !ASSERT_GE(extra_fds[0], 0, c"new_btf".as_ptr()) {
            break 'cleanup;
        }
        prog_fd = load_test_prog(extra_fds.as_ptr(), 1);
        if !ASSERT_GE(prog_fd, 0, c"BPF_PROG_LOAD".as_ptr()) {
            break 'cleanup;
        }

        /* btf should still exist when original file descriptor is closed */
        err = get_btf_id_by_fd(extra_fds[0], &mut btf_id);
        if !ASSERT_EQ(err, 0, c"get_btf_id_by_fd".as_ptr()) {
            break 'cleanup;
        }

        Close(&mut extra_fds[0]);

        if !ASSERT_GE(kern_sync_rcu(), 0, c"kern_sync_rcu 1".as_ptr()) {
            break 'cleanup;
        }

        if !ASSERT_EQ(btf_exists(btf_id), true, c"btf should exist".as_ptr()) {
            break 'cleanup;
        }

        Close(&mut prog_fd);

        /* The program is freed by a workqueue, so no reliable
         * way to sync, so just wait a bit (max ~1 second). */
        tries = 100;
        while tries >= 0 {
            usleep(1000);

            if !btf_exists(btf_id) {
                break;
            }

            if tries != 0 {
                tries -= 1;
                continue;
            }

            PRINT_FAIL(c"btf should have been freed".as_ptr());
            tries -= 1;
        }
    }

    /* some fds might be invalid, so ignore return codes */
    Close(&mut extra_fds[0]);
    Close(&mut prog_fd);
}

/*
 * Test that a program with trash in fd_array can't be loaded:
 * only map and BTF file descriptors should be accepted.
 */
unsafe fn check_fd_array_cnt__fd_array_with_trash() {
    let mut extra_fds: [c_int; 3] = [-1, -1, -1];
    let mut prog_fd: c_int = -1;

    'cleanup: {
        extra_fds[0] = new_map();
        if !ASSERT_GE(extra_fds[0], 0, c"new_map".as_ptr()) {
            break 'cleanup;
        }
        extra_fds[1] = new_btf();
        if !ASSERT_GE(extra_fds[1], 0, c"new_btf".as_ptr()) {
            break 'cleanup;
        }

        /* trash 1: not a file descriptor */
        extra_fds[2] = 0xbeef;
        prog_fd = load_test_prog(extra_fds.as_ptr(), 3);
        if !ASSERT_EQ(prog_fd, -EBADF, c"prog should have been rejected with -EBADF".as_ptr()) {
            break 'cleanup;
        }

        /* trash 2: not a map or btf */
        extra_fds[2] = socket(AF_INET, SOCK_STREAM, 0);
        if !ASSERT_GE(extra_fds[2], 0, c"socket".as_ptr()) {
            break 'cleanup;
        }

        prog_fd = load_test_prog(extra_fds.as_ptr(), 3);
        if !ASSERT_EQ(prog_fd, -EINVAL, c"prog should have been rejected with -EINVAL".as_ptr()) {
            break 'cleanup;
        }

        /* Validate that the prog is ok if trash is removed */
        Close(&mut extra_fds[2]);
        extra_fds[2] = new_btf();
        if !ASSERT_GE(extra_fds[2], 0, c"new_btf".as_ptr()) {
            break 'cleanup;
        }

        prog_fd = load_test_prog(extra_fds.as_ptr(), 3);
        if !ASSERT_GE(prog_fd, 0, c"prog should have been loaded".as_ptr()) {
            break 'cleanup;
        }
    }

    /* some fds might be invalid, so ignore return codes */
    Close(&mut extra_fds[2]);
    Close(&mut extra_fds[1]);
    Close(&mut extra_fds[0]);
}

/*
 * Test that a program with too big fd_array can't be loaded.
 */
unsafe fn check_fd_array_cnt__fd_array_too_big() {
    let mut extra_fds: [c_int; 65] = [0; 65];
    let mut prog_fd: c_int = -1;
    let mut i: c_int;

    i = 0;
    'cleanup_fds: {
        while i < 65 {
            extra_fds[i as usize] = new_map();
            if !ASSERT_GE(extra_fds[i as usize], 0, c"new_map".as_ptr()) {
                break 'cleanup_fds;
            }
            i += 1;
        }

        prog_fd = load_test_prog(extra_fds.as_ptr(), 65);
        ASSERT_EQ(prog_fd, -E2BIG, c"prog should have been rejected with -E2BIG".as_ptr());
    }

    while {
        let old_i = i;
        i -= 1;
        old_i > 0
    } {
        Close(&mut extra_fds[i as usize]);
    }
}

pub unsafe fn test_fd_array_cnt() {
    if test__start_subtest(c"no-fd-array".as_ptr()) {
        check_fd_array_cnt__no_fd_array();
    }

    if test__start_subtest(c"fd-array-ok".as_ptr()) {
        check_fd_array_cnt__fd_array_ok();
    }

    if test__start_subtest(c"fd-array-dup-input".as_ptr()) {
        check_fd_array_cnt__duplicated_maps();
    }

    if test__start_subtest(c"fd-array-ref-maps-in-array".as_ptr()) {
        check_fd_array_cnt__referenced_maps_in_fd_array();
    }

    if test__start_subtest(c"fd-array-ref-btfs".as_ptr()) {
        check_fd_array_cnt__referenced_btfs();
    }

    if test__start_subtest(c"fd-array-trash-input".as_ptr()) {
        check_fd_array_cnt__fd_array_with_trash();
    }

    if test__start_subtest(c"fd-array-2big".as_ptr()) {
        check_fd_array_cnt__fd_array_too_big();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
