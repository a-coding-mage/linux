// SPDX-License-Identifier: GPL-2.0
// Translated from C source using external declarations for test_progs.h and
// network_helpers.h dependencies.

use core::ffi::{c_char, c_int, c_uint, c_void};

type __u8 = u8;
type __u32 = u32;
type __u64 = u64;
type __be32 = u32;
type u32 = u32;

const BPF_PROG_TYPE_SCHED_CLS: c_int = 3;
const NUM_ITER: u32 = 100000;

extern "C" {
    static pkt_v4: c_void;
    static pkt_v6: c_void;

    static VIP_NUM: __u32;
    static MAGIC_VAL: u32;
    static MAGIC_BYTES: __u64;

    fn bpf_num_possible_cpus() -> c_uint;
    fn bpf_prog_test_load(
        file: *const c_char,
        prog_type: c_int,
        obj: *mut *mut bpf_object,
        prog_fd: *mut c_int,
    ) -> c_int;
    fn bpf_find_map(test_name: *const c_char, obj: *mut bpf_object, name: *const c_char) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: __u64,
    ) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);
    fn test__start_subtest(name: *const c_char) -> bool;
    fn printf(fmt: *const c_char, ...) -> c_int;

    fn CHECK_FAIL(condition: c_int) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char);
    fn ASSERT_EQ(actual: __u64, expected: __u64, name: *const c_char);
}

#[repr(C)]
struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
struct vip {
    protocol: __u8,
}

#[repr(C)]
struct vip_meta {
    flags: __u32,
    vip_num: __u32,
}

#[repr(C)]
struct vip_stats {
    bytes: __u64,
    pkts: __u64,
}

#[repr(C)]
union real_definition_anon {
    dst: __be32,
    dstv6: [__be32; 4],
}

#[repr(C)]
struct real_definition {
    u: real_definition_anon,
    flags: __u8,
}

#[repr(C)]
struct bpf_test_run_opts {
    sz: usize,
    data_in: *const c_void,
    data_out: *mut c_void,
    data_size_in: u32,
    data_size_out: u32,
    retval: u32,
    repeat: u32,
}

unsafe fn test_l4lb(file: *const c_char) {
    let nr_cpus: c_uint = bpf_num_possible_cpus();
    let key = vip { protocol: 6 };
    let value = vip_meta {
        flags: 0,
        vip_num: VIP_NUM,
    };
    let stats_key: __u32 = VIP_NUM;
    let mut stats = Vec::<vip_stats>::with_capacity(nr_cpus as usize);
    stats.resize(
        nr_cpus as usize,
        vip_stats {
            bytes: 0,
            pkts: 0,
        },
    );
    let real_def = real_definition {
        u: real_definition_anon { dst: MAGIC_VAL },
        flags: 0,
    };
    let ch_key: __u32 = 11;
    let real_num: __u32 = 3;
    let mut err: c_int;
    let mut prog_fd: c_int = 0;
    let mut map_fd: c_int;
    let mut bytes: __u64 = 0;
    let mut pkts: __u64 = 0;
    let mut obj: *mut bpf_object = core::ptr::null_mut();
    let mut buf = [0u8; 128];
    let magic = buf.as_mut_ptr() as *mut u32;
    let mut topts = bpf_test_run_opts {
        sz: core::mem::size_of::<bpf_test_run_opts>(),
        data_in: core::ptr::null(),
        data_out: buf.as_mut_ptr() as *mut c_void,
        data_size_in: 0,
        data_size_out: core::mem::size_of_val(&buf) as u32,
        retval: 0,
        repeat: NUM_ITER,
    };

    err = bpf_prog_test_load(file, BPF_PROG_TYPE_SCHED_CLS, &mut obj, &mut prog_fd);
    if CHECK_FAIL(err) {
        return;
    }

    map_fd = bpf_find_map(c"test_l4lb".as_ptr(), obj, c"vip_map".as_ptr());
    if map_fd < 0 {
        goto_out(obj);
        return;
    }
    bpf_map_update_elem(
        map_fd,
        &key as *const _ as *const c_void,
        &value as *const _ as *const c_void,
        0,
    );

    map_fd = bpf_find_map(c"test_l4lb".as_ptr(), obj, c"ch_rings".as_ptr());
    if map_fd < 0 {
        goto_out(obj);
        return;
    }
    bpf_map_update_elem(
        map_fd,
        &ch_key as *const _ as *const c_void,
        &real_num as *const _ as *const c_void,
        0,
    );

    map_fd = bpf_find_map(c"test_l4lb".as_ptr(), obj, c"reals".as_ptr());
    if map_fd < 0 {
        goto_out(obj);
        return;
    }
    bpf_map_update_elem(
        map_fd,
        &real_num as *const _ as *const c_void,
        &real_def as *const _ as *const c_void,
        0,
    );

    topts.data_in = &pkt_v4 as *const _ as *const c_void;
    topts.data_size_in = core::mem::size_of_val(&pkt_v4) as u32;

    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, c"test_run".as_ptr());
    ASSERT_EQ(
        topts.retval as __u64,
        7, /*TC_ACT_REDIRECT*/
        c"ipv4 test_run retval".as_ptr(),
    );
    ASSERT_EQ(topts.data_size_out as __u64, 54, c"ipv4 test_run data_size_out".as_ptr());
    ASSERT_EQ(*magic as __u64, MAGIC_VAL as __u64, c"ipv4 magic".as_ptr());

    topts.data_in = &pkt_v6 as *const _ as *const c_void;
    topts.data_size_in = core::mem::size_of_val(&pkt_v6) as u32;
    topts.data_size_out = core::mem::size_of_val(&buf) as u32; /* reset out size */

    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, c"test_run".as_ptr());
    ASSERT_EQ(
        topts.retval as __u64,
        7, /*TC_ACT_REDIRECT*/
        c"ipv6 test_run retval".as_ptr(),
    );
    ASSERT_EQ(topts.data_size_out as __u64, 74, c"ipv6 test_run data_size_out".as_ptr());
    ASSERT_EQ(*magic as __u64, MAGIC_VAL as __u64, c"ipv6 magic".as_ptr());

    map_fd = bpf_find_map(c"test_l4lb".as_ptr(), obj, c"stats".as_ptr());
    if map_fd < 0 {
        goto_out(obj);
        return;
    }
    bpf_map_lookup_elem(
        map_fd,
        &stats_key as *const _ as *const c_void,
        stats.as_mut_ptr() as *mut c_void,
    );
    let mut i: c_int = 0;
    while i < nr_cpus as c_int {
        bytes = bytes.wrapping_add(stats[i as usize].bytes);
        pkts = pkts.wrapping_add(stats[i as usize].pkts);
        i += 1;
    }
    if CHECK_FAIL(
        (bytes != MAGIC_BYTES.wrapping_mul(NUM_ITER as __u64).wrapping_mul(2)
            || pkts != (NUM_ITER as __u64).wrapping_mul(2)) as c_int,
    ) {
        printf(c"test_l4lb:FAIL:stats %lld %lld\n".as_ptr(), bytes, pkts);
    }

    goto_out(obj);
}

unsafe fn goto_out(obj: *mut bpf_object) {
    bpf_object__close(obj);
}

#[no_mangle]
pub unsafe extern "C" fn test_l4lb_all() {
    if test__start_subtest(c"l4lb_inline".as_ptr()) {
        test_l4lb(c"test_l4lb.bpf.o".as_ptr());
    }
    if test__start_subtest(c"l4lb_noinline".as_ptr()) {
        test_l4lb(c"test_l4lb_noinline.bpf.o".as_ptr());
    }
    if test__start_subtest(c"l4lb_noinline_dynptr".as_ptr()) {
        test_l4lb(c"test_l4lb_noinline_dynptr.bpf.o".as_ptr());
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
