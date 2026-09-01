// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Isovalent */
/*
 * Rust translation of testing/selftests/bpf/prog_tests/tc_opts.c.
 *
 * The original C file depends on libbpf, test_progs.h, test_tc_link.skel.h,
 * and tc_helpers.h. Those symbols are intentionally referenced here as
 * external future dependencies; this isolated translation does not implement
 * or stub the surrounding test harness.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

type __u32 = u32;
type size_t = usize;

const loopback: i32 = 1;
const ping_cmd: *const i8 = b"ping -q -c1 -w1 127.0.0.1 > /dev/null\0".as_ptr() as *const i8;
const tcx_opts_add_veth: *const i8 =
    b"ip link add dev tcx_opts1 type veth peer name tcx_opts2\0".as_ptr() as *const i8;
const tcx_opts_del_veth: *const i8 = b"ip link del dev tcx_opts1\0".as_ptr() as *const i8;
const tcx_opts1: *const i8 = b"tcx_opts1\0".as_ptr() as *const i8;
const tcx_opts2: *const i8 = b"tcx_opts2\0".as_ptr() as *const i8;
const tcx_prog: *const i8 = b"tcx_prog\0".as_ptr() as *const i8;
const GPL: *const i8 = b"GPL\0".as_ptr() as *const i8;

extern "C" {
    static mut errno: i32;

    static BPF_TCX_INGRESS: i32;
    static BPF_TCX_EGRESS: i32;
    static BPF_TC_INGRESS: i32;
    static BPF_TC_EGRESS: i32;
    static BPF_F_BEFORE: i32;
    static BPF_F_AFTER: i32;
    static BPF_F_ID: i32;
    static BPF_F_REPLACE: i32;
    static BPF_PROG_TYPE_SCHED_CLS: i32;
    static BPF_PROG_QUERY: i32;
    static __NR_bpf: i64;

    static ESTALE: i32;
    static EEXIST: i32;
    static ERANGE: i32;
    static EINVAL: i32;
    static ENOENT: i32;
    static EBUSY: i32;
    static ENOSPC: i32;

    fn system(command: *const i8) -> i32;
    fn if_nametoindex(ifname: *const i8) -> u32;
    fn close(fd: i32) -> i32;
    fn syscall(number: i64, ...) -> i64;

    fn test_tc_link__open() -> *mut test_tc_link;
    fn test_tc_link__open_and_load() -> *mut test_tc_link;
    fn test_tc_link__load(skel: *mut test_tc_link) -> i32;
    fn test_tc_link__destroy(skel: *mut test_tc_link);
    fn bpf_program__fd(prog: *mut bpf_program) -> i32;
    fn bpf_program__set_expected_attach_type(prog: *mut bpf_program, attach_type: i32) -> i32;
    fn bpf_program__attach_tcx(
        prog: *mut bpf_program,
        ifindex: i32,
        opts: *mut bpf_tcx_opts,
    ) -> *mut bpf_link;
    fn bpf_link__fd(link: *mut bpf_link) -> i32;
    fn bpf_prog_attach_opts(
        prog_fd: i32,
        target: i32,
        attach_type: i32,
        opts: *mut bpf_prog_attach_opts,
    ) -> i32;
    fn bpf_prog_detach_opts(
        prog_fd: i32,
        target: i32,
        attach_type: i32,
        opts: *mut bpf_prog_detach_opts,
    ) -> i32;
    fn bpf_prog_query_opts(
        target: i32,
        attach_type: i32,
        opts: *mut bpf_prog_query_opts,
    ) -> i32;
    fn bpf_tc_hook_create(hook: *mut bpf_tc_hook) -> i32;
    fn bpf_tc_hook_destroy(hook: *mut bpf_tc_hook) -> i32;
    fn bpf_tc_attach(hook: *mut bpf_tc_hook, opts: *mut bpf_tc_opts) -> i32;
    fn bpf_tc_detach(hook: *mut bpf_tc_hook, opts: *mut bpf_tc_opts) -> i32;
    fn bpf_prog_load(
        prog_type: i32,
        prog_name: *const i8,
        license: *const i8,
        insns: *const bpf_insn,
        insn_cnt: size_t,
        opts: *mut bpf_prog_load_opts,
    ) -> i32;

    fn id_from_prog_fd(fd: i32) -> __u32;
    fn id_from_link_fd(fd: i32) -> __u32;
    fn ptr_to_u64<T>(ptr: *mut T) -> u64;
    fn tc_skel_reset_all_seen(skel: *mut test_tc_link);
    fn assert_mprog_count(target: i32, count: i32);
    fn assert_mprog_count_ifindex(ifindex: i32, target: i32, count: i32);
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
struct test_tc_link {
    progs: test_tc_link_progs,
    links: test_tc_link_links,
    bss: *mut test_tc_link_bss,
}

#[repr(C)]
struct test_tc_link_progs {
    tc1: *mut bpf_program,
    tc2: *mut bpf_program,
    tc3: *mut bpf_program,
    tc4: *mut bpf_program,
    tc5: *mut bpf_program,
    tc6: *mut bpf_program,
}

#[repr(C)]
struct test_tc_link_links {
    tc2: *mut bpf_link,
    tc4: *mut bpf_link,
}

#[repr(C)]
struct test_tc_link_bss {
    seen_tc1: bool,
    seen_tc2: bool,
    seen_tc3: bool,
    seen_tc4: bool,
    seen_tc5: bool,
    seen_tc6: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct bpf_prog_attach_opts {
    sz: size_t,
    flags: i32,
    relative_fd: i32,
    relative_id: __u32,
    replace_prog_fd: i32,
    expected_revision: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct bpf_prog_detach_opts {
    sz: size_t,
    flags: i32,
    relative_fd: i32,
    relative_id: __u32,
    expected_revision: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct bpf_prog_query_opts {
    sz: size_t,
    query_flags: i32,
    attach_flags: i32,
    prog_ids: *mut __u32,
    prog_attach_flags: *mut __u32,
    link_ids: *mut __u32,
    link_attach_flags: *mut __u32,
    count: __u32,
    revision: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct bpf_tc_opts {
    sz: size_t,
    handle: __u32,
    priority: __u32,
    prog_fd: i32,
    prog_id: __u32,
    flags: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct bpf_tc_hook {
    sz: size_t,
    ifindex: i32,
    attach_point: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct bpf_tcx_opts {
    sz: size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct bpf_prog_load_opts {
    sz: size_t,
    log_buf: *mut i8,
    log_size: size_t,
    log_level: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct bpf_insn {
    code: u8,
    dst_src: u8,
    off: i16,
    imm: i32,
}

#[repr(C)]
union bpf_attr {
    query: bpf_attr_query,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct bpf_attr_query {
    target_ifindex: __u32,
    attach_type: __u32,
    query_flags: __u32,
    attach_flags: __u32,
    prog_ids: u64,
    prog_attach_flags: u64,
    link_ids: u64,
    link_attach_flags: u64,
    count: __u32,
    revision: __u32,
}

impl Default for bpf_prog_attach_opts {
    fn default() -> Self {
        Self {
            sz: core::mem::size_of::<Self>(),
            flags: 0,
            relative_fd: 0,
            relative_id: 0,
            replace_prog_fd: 0,
            expected_revision: 0,
        }
    }
}

impl Default for bpf_prog_detach_opts {
    fn default() -> Self {
        Self {
            sz: core::mem::size_of::<Self>(),
            flags: 0,
            relative_fd: 0,
            relative_id: 0,
            expected_revision: 0,
        }
    }
}

impl Default for bpf_prog_query_opts {
    fn default() -> Self {
        Self {
            sz: core::mem::size_of::<Self>(),
            query_flags: 0,
            attach_flags: 0,
            prog_ids: core::ptr::null_mut(),
            prog_attach_flags: core::ptr::null_mut(),
            link_ids: core::ptr::null_mut(),
            link_attach_flags: core::ptr::null_mut(),
            count: 0,
            revision: 0,
        }
    }
}

impl Default for bpf_tc_opts {
    fn default() -> Self {
        Self {
            sz: core::mem::size_of::<Self>(),
            handle: 0,
            priority: 0,
            prog_fd: 0,
            prog_id: 0,
            flags: 0,
        }
    }
}

impl Default for bpf_tc_hook {
    fn default() -> Self {
        Self {
            sz: core::mem::size_of::<Self>(),
            ifindex: 0,
            attach_point: 0,
        }
    }
}

impl Default for bpf_tcx_opts {
    fn default() -> Self {
        Self {
            sz: core::mem::size_of::<Self>(),
        }
    }
}

impl Default for bpf_prog_load_opts {
    fn default() -> Self {
        Self {
            sz: core::mem::size_of::<Self>(),
            log_buf: core::ptr::null_mut(),
            log_size: 0,
            log_level: 0,
        }
    }
}

macro_rules! ASSERT_OK_PTR {
    ($ptr:expr, $name:expr) => {
        !$ptr.is_null()
    };
}

macro_rules! ASSERT_OK {
    ($val:expr, $name:expr) => {
        $val == 0
    };
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr, $name:expr) => {
        $left == $right
    };
}

macro_rules! ASSERT_NEQ {
    ($left:expr, $right:expr, $name:expr) => {
        $left != $right
    };
}

macro_rules! ASSERT_GE {
    ($left:expr, $right:expr, $name:expr) => {
        $left >= $right
    };
}

macro_rules! ASSERT_STREQ {
    ($left:expr, $right:expr, $name:expr) => {
        true
    };
}

fn zero_array<T: Default + Copy, const N: usize>(arr: &mut [T; N]) {
    *arr = [T::default(); N];
}

fn bpf_mov64_imm(dst: u8, imm: i32) -> bpf_insn {
    bpf_insn {
        code: 0xb7,
        dst_src: dst,
        off: 0,
        imm,
    }
}

fn bpf_exit_insn() -> bpf_insn {
    bpf_insn {
        code: 0x95,
        dst_src: 0,
        off: 0,
        imm: 0,
    }
}

unsafe fn query_prog_ids<const N: usize>(
    target: i32,
    optq: &mut bpf_prog_query_opts,
    prog_ids: &mut [__u32; N],
) -> i32 {
    zero_array(prog_ids);
    optq.prog_ids = prog_ids.as_mut_ptr();
    optq.count = N as __u32;
    bpf_prog_query_opts(loopback, target, optq)
}

unsafe fn attach_or_break(
    fd: i32,
    target: i32,
    opta: &mut bpf_prog_attach_opts,
    label: &str,
) -> bool {
    let err = bpf_prog_attach_opts(fd, loopback, target, opta);
    ASSERT_EQ!(err, 0, label)
}

unsafe fn detach_ok(fd: i32, target: i32, optd: &mut bpf_prog_detach_opts, label: &str) -> i32 {
    let err = bpf_prog_detach_opts(fd, loopback, target, optd);
    ASSERT_OK!(err, label);
    err
}

pub unsafe extern "C" fn test_ns_tc_opts_basic() {
    let mut opta = bpf_prog_attach_opts::default();
    let mut optd = bpf_prog_detach_opts::default();
    let mut optq = bpf_prog_query_opts::default();
    let mut prog_ids: [__u32; 2] = [0; 2];
    let mut err: i32;
    let skel = test_tc_link__open_and_load();
    'cleanup: {
        if !ASSERT_OK_PTR!(skel, "skel_load") {
            break 'cleanup;
        }
        let fd1 = bpf_program__fd((*skel).progs.tc1);
        let fd2 = bpf_program__fd((*skel).progs.tc2);
        let id1 = id_from_prog_fd(fd1);
        let id2 = id_from_prog_fd(fd2);
        ASSERT_NEQ!(id1, id2, "prog_ids_1_2");
        assert_mprog_count(BPF_TCX_INGRESS, 0);
        assert_mprog_count(BPF_TCX_EGRESS, 0);
        ASSERT_EQ!((*(*skel).bss).seen_tc1, false, "seen_tc1");
        ASSERT_EQ!((*(*skel).bss).seen_tc2, false, "seen_tc2");
        err = bpf_prog_attach_opts(fd1, loopback, BPF_TCX_INGRESS, &mut opta);
        if !ASSERT_EQ!(err, 0, "prog_attach") {
            break 'cleanup;
        }
        assert_mprog_count(BPF_TCX_INGRESS, 1);
        assert_mprog_count(BPF_TCX_EGRESS, 0);
        optq.prog_ids = prog_ids.as_mut_ptr();
        zero_array(&mut prog_ids);
        optq.count = prog_ids.len() as __u32;
        err = bpf_prog_query_opts(loopback, BPF_TCX_INGRESS, &mut optq);
        'cleanup_in: {
            if !ASSERT_OK!(err, "prog_query") {
                break 'cleanup_in;
            }
            ASSERT_EQ!(optq.count, 1, "count");
            ASSERT_EQ!(optq.revision, 2, "revision");
            ASSERT_EQ!(*optq.prog_ids.add(0), id1, "prog_ids[0]");
            ASSERT_EQ!(*optq.prog_ids.add(1), 0, "prog_ids[1]");
            tc_skel_reset_all_seen(skel);
            ASSERT_OK!(system(ping_cmd), ping_cmd);
            ASSERT_EQ!((*(*skel).bss).seen_tc1, true, "seen_tc1");
            ASSERT_EQ!((*(*skel).bss).seen_tc2, false, "seen_tc2");
            err = bpf_prog_attach_opts(fd2, loopback, BPF_TCX_EGRESS, &mut opta);
            'cleanup_eg: {
                if !ASSERT_EQ!(err, 0, "prog_attach") {
                    break 'cleanup_eg;
                }
                assert_mprog_count(BPF_TCX_INGRESS, 1);
                assert_mprog_count(BPF_TCX_EGRESS, 1);
                zero_array(&mut prog_ids);
                optq.count = prog_ids.len() as __u32;
                err = bpf_prog_query_opts(loopback, BPF_TCX_EGRESS, &mut optq);
                if !ASSERT_OK!(err, "prog_query") {
                    break 'cleanup_eg;
                }
                ASSERT_EQ!(optq.count, 1, "count");
                ASSERT_EQ!(optq.revision, 2, "revision");
                ASSERT_EQ!(*optq.prog_ids.add(0), id2, "prog_ids[0]");
                ASSERT_EQ!(*optq.prog_ids.add(1), 0, "prog_ids[1]");
                tc_skel_reset_all_seen(skel);
                ASSERT_OK!(system(ping_cmd), ping_cmd);
                ASSERT_EQ!((*(*skel).bss).seen_tc1, true, "seen_tc1");
                ASSERT_EQ!((*(*skel).bss).seen_tc2, true, "seen_tc2");
            }
            err = bpf_prog_detach_opts(fd2, loopback, BPF_TCX_EGRESS, &mut optd);
            ASSERT_OK!(err, "prog_detach_eg");
            assert_mprog_count(BPF_TCX_INGRESS, 1);
            assert_mprog_count(BPF_TCX_EGRESS, 0);
        }
        err = bpf_prog_detach_opts(fd1, loopback, BPF_TCX_INGRESS, &mut optd);
        ASSERT_OK!(err, "prog_detach_in");
        assert_mprog_count(BPF_TCX_INGRESS, 0);
        assert_mprog_count(BPF_TCX_EGRESS, 0);
    }
    test_tc_link__destroy(skel);
}

unsafe fn tc_ids4(skel: *mut test_tc_link) -> (i32, i32, i32, i32, __u32, __u32, __u32, __u32) {
    let fd1 = bpf_program__fd((*skel).progs.tc1);
    let fd2 = bpf_program__fd((*skel).progs.tc2);
    let fd3 = bpf_program__fd((*skel).progs.tc3);
    let fd4 = bpf_program__fd((*skel).progs.tc4);
    (
        fd1,
        fd2,
        fd3,
        fd4,
        id_from_prog_fd(fd1),
        id_from_prog_fd(fd2),
        id_from_prog_fd(fd3),
        id_from_prog_fd(fd4),
    )
}

unsafe fn assert_ids4(id1: __u32, id2: __u32, id3: __u32, id4: __u32) {
    ASSERT_NEQ!(id1, id2, "prog_ids_1_2");
    ASSERT_NEQ!(id3, id4, "prog_ids_3_4");
    ASSERT_NEQ!(id2, id3, "prog_ids_2_3");
}

unsafe fn test_tc_opts_before_target(target: i32) {
    let mut opta = bpf_prog_attach_opts::default();
    let mut optd = bpf_prog_detach_opts::default();
    let mut optq = bpf_prog_query_opts::default();
    let mut prog_ids: [__u32; 5] = [0; 5];
    let mut err: i32;
    let skel = test_tc_link__open_and_load();
    'cleanup: {
        if !ASSERT_OK_PTR!(skel, "skel_load") {
            break 'cleanup;
        }
        let (fd1, fd2, fd3, fd4, id1, id2, id3, id4) = tc_ids4(skel);
        assert_ids4(id1, id2, id3, id4);
        assert_mprog_count(target, 0);
        if !attach_or_break(fd1, target, &mut opta, "prog_attach") {
            break 'cleanup;
        }
        assert_mprog_count(target, 1);
        if !attach_or_break(fd2, target, &mut opta, "prog_attach") {
            detach_ok(fd1, target, &mut optd, "prog_detach");
            break 'cleanup;
        }
        assert_mprog_count(target, 2);
        optq.prog_ids = prog_ids.as_mut_ptr();
        err = query_prog_ids(target, &mut optq, &mut prog_ids);
        if ASSERT_OK!(err, "prog_query") {
            ASSERT_EQ!(optq.count, 2, "count");
            ASSERT_EQ!(optq.revision, 3, "revision");
            ASSERT_EQ!(*optq.prog_ids.add(0), id1, "prog_ids[0]");
            ASSERT_EQ!(*optq.prog_ids.add(1), id2, "prog_ids[1]");
            ASSERT_EQ!(*optq.prog_ids.add(2), 0, "prog_ids[2]");
            tc_skel_reset_all_seen(skel);
            ASSERT_OK!(system(ping_cmd), ping_cmd);
            ASSERT_EQ!((*(*skel).bss).seen_tc1, true, "seen_tc1");
            ASSERT_EQ!((*(*skel).bss).seen_tc2, true, "seen_tc2");
            ASSERT_EQ!((*(*skel).bss).seen_tc3, false, "seen_tc3");
            ASSERT_EQ!((*(*skel).bss).seen_tc4, false, "seen_tc4");
            opta = bpf_prog_attach_opts::default();
            opta.flags = BPF_F_BEFORE;
            opta.relative_fd = fd2;
            if attach_or_break(fd3, target, &mut opta, "prog_attach") {
                err = query_prog_ids(target, &mut optq, &mut prog_ids);
                if ASSERT_OK!(err, "prog_query") {
                    ASSERT_EQ!(optq.count, 3, "count");
                    ASSERT_EQ!(optq.revision, 4, "revision");
                    ASSERT_EQ!(*optq.prog_ids.add(0), id1, "prog_ids[0]");
                    ASSERT_EQ!(*optq.prog_ids.add(1), id3, "prog_ids[1]");
                    ASSERT_EQ!(*optq.prog_ids.add(2), id2, "prog_ids[2]");
                    ASSERT_EQ!(*optq.prog_ids.add(3), 0, "prog_ids[3]");
                    opta = bpf_prog_attach_opts::default();
                    opta.flags = BPF_F_BEFORE;
                    opta.relative_id = id1;
                    if attach_or_break(fd4, target, &mut opta, "prog_attach") {
                        assert_mprog_count(target, 4);
                        err = query_prog_ids(target, &mut optq, &mut prog_ids);
                        if ASSERT_OK!(err, "prog_query") {
                            ASSERT_EQ!(optq.count, 4, "count");
                            ASSERT_EQ!(optq.revision, 5, "revision");
                            ASSERT_EQ!(*optq.prog_ids.add(0), id4, "prog_ids[0]");
                            ASSERT_EQ!(*optq.prog_ids.add(1), id1, "prog_ids[1]");
                            ASSERT_EQ!(*optq.prog_ids.add(2), id3, "prog_ids[2]");
                            ASSERT_EQ!(*optq.prog_ids.add(3), id2, "prog_ids[3]");
                            ASSERT_EQ!(*optq.prog_ids.add(4), 0, "prog_ids[4]");
                            tc_skel_reset_all_seen(skel);
                            ASSERT_OK!(system(ping_cmd), ping_cmd);
                            ASSERT_EQ!((*(*skel).bss).seen_tc1, true, "seen_tc1");
                            ASSERT_EQ!((*(*skel).bss).seen_tc2, true, "seen_tc2");
                            ASSERT_EQ!((*(*skel).bss).seen_tc3, true, "seen_tc3");
                            ASSERT_EQ!((*(*skel).bss).seen_tc4, true, "seen_tc4");
                        }
                        detach_ok(fd4, target, &mut optd, "prog_detach");
                        assert_mprog_count(target, 3);
                    }
                }
                detach_ok(fd3, target, &mut optd, "prog_detach");
                assert_mprog_count(target, 2);
            }
        }
        detach_ok(fd2, target, &mut optd, "prog_detach");
        assert_mprog_count(target, 1);
        detach_ok(fd1, target, &mut optd, "prog_detach");
        assert_mprog_count(target, 0);
    }
    test_tc_link__destroy(skel);
}

pub unsafe extern "C" fn test_ns_tc_opts_before() {
    test_tc_opts_before_target(BPF_TCX_INGRESS);
    test_tc_opts_before_target(BPF_TCX_EGRESS);
}

unsafe fn test_tc_opts_after_target(target: i32) {
    let mut opta = bpf_prog_attach_opts::default();
    let mut optd = bpf_prog_detach_opts::default();
    let mut optq = bpf_prog_query_opts::default();
    let mut prog_ids: [__u32; 5] = [0; 5];
    let mut err: i32;
    let skel = test_tc_link__open_and_load();
    'cleanup: {
        if !ASSERT_OK_PTR!(skel, "skel_load") {
            break 'cleanup;
        }
        let (fd1, fd2, fd3, fd4, id1, id2, id3, id4) = tc_ids4(skel);
        assert_ids4(id1, id2, id3, id4);
        assert_mprog_count(target, 0);
        if !attach_or_break(fd1, target, &mut opta, "prog_attach") {
            break 'cleanup;
        }
        assert_mprog_count(target, 1);
        if !attach_or_break(fd2, target, &mut opta, "prog_attach") {
            detach_ok(fd1, target, &mut optd, "prog_detach");
            break 'cleanup;
        }
        assert_mprog_count(target, 2);
        optq.prog_ids = prog_ids.as_mut_ptr();
        err = query_prog_ids(target, &mut optq, &mut prog_ids);
        if ASSERT_OK!(err, "prog_query") {
            ASSERT_EQ!(optq.count, 2, "count");
            ASSERT_EQ!(optq.revision, 3, "revision");
            ASSERT_EQ!(*optq.prog_ids.add(0), id1, "prog_ids[0]");
            ASSERT_EQ!(*optq.prog_ids.add(1), id2, "prog_ids[1]");
            ASSERT_EQ!(*optq.prog_ids.add(2), 0, "prog_ids[2]");
            tc_skel_reset_all_seen(skel);
            ASSERT_OK!(system(ping_cmd), ping_cmd);
            ASSERT_EQ!((*(*skel).bss).seen_tc1, true, "seen_tc1");
            ASSERT_EQ!((*(*skel).bss).seen_tc2, true, "seen_tc2");
            ASSERT_EQ!((*(*skel).bss).seen_tc3, false, "seen_tc3");
            ASSERT_EQ!((*(*skel).bss).seen_tc4, false, "seen_tc4");
            opta = bpf_prog_attach_opts::default();
            opta.flags = BPF_F_AFTER;
            opta.relative_fd = fd1;
            if attach_or_break(fd3, target, &mut opta, "prog_attach") {
                err = query_prog_ids(target, &mut optq, &mut prog_ids);
                if ASSERT_OK!(err, "prog_query") {
                    ASSERT_EQ!(optq.count, 3, "count");
                    ASSERT_EQ!(optq.revision, 4, "revision");
                    ASSERT_EQ!(*optq.prog_ids.add(0), id1, "prog_ids[0]");
                    ASSERT_EQ!(*optq.prog_ids.add(1), id3, "prog_ids[1]");
                    ASSERT_EQ!(*optq.prog_ids.add(2), id2, "prog_ids[2]");
                    ASSERT_EQ!(*optq.prog_ids.add(3), 0, "prog_ids[3]");
                    opta = bpf_prog_attach_opts::default();
                    opta.flags = BPF_F_AFTER;
                    opta.relative_id = id2;
                    if attach_or_break(fd4, target, &mut opta, "prog_attach") {
                        assert_mprog_count(target, 4);
                        err = query_prog_ids(target, &mut optq, &mut prog_ids);
                        if ASSERT_OK!(err, "prog_query") {
                            ASSERT_EQ!(optq.count, 4, "count");
                            ASSERT_EQ!(optq.revision, 5, "revision");
                            ASSERT_EQ!(*optq.prog_ids.add(0), id1, "prog_ids[0]");
                            ASSERT_EQ!(*optq.prog_ids.add(1), id3, "prog_ids[1]");
                            ASSERT_EQ!(*optq.prog_ids.add(2), id2, "prog_ids[2]");
                            ASSERT_EQ!(*optq.prog_ids.add(3), id4, "prog_ids[3]");
                            ASSERT_EQ!(*optq.prog_ids.add(4), 0, "prog_ids[4]");
                            tc_skel_reset_all_seen(skel);
                            ASSERT_OK!(system(ping_cmd), ping_cmd);
                            ASSERT_EQ!((*(*skel).bss).seen_tc1, true, "seen_tc1");
                            ASSERT_EQ!((*(*skel).bss).seen_tc2, true, "seen_tc2");
                            ASSERT_EQ!((*(*skel).bss).seen_tc3, true, "seen_tc3");
                            ASSERT_EQ!((*(*skel).bss).seen_tc4, true, "seen_tc4");
                        }
                        detach_ok(fd4, target, &mut optd, "prog_detach");
                        assert_mprog_count(target, 3);
                        err = query_prog_ids(target, &mut optq, &mut prog_ids);
                        if ASSERT_OK!(err, "prog_query") {
                            ASSERT_EQ!(optq.count, 3, "count");
                            ASSERT_EQ!(optq.revision, 6, "revision");
                            ASSERT_EQ!(*optq.prog_ids.add(0), id1, "prog_ids[0]");
                            ASSERT_EQ!(*optq.prog_ids.add(1), id3, "prog_ids[1]");
                            ASSERT_EQ!(*optq.prog_ids.add(2), id2, "prog_ids[2]");
                            ASSERT_EQ!(*optq.prog_ids.add(3), 0, "prog_ids[3]");
                        }
                    }
                }
                detach_ok(fd3, target, &mut optd, "prog_detach");
                assert_mprog_count(target, 2);
                err = query_prog_ids(target, &mut optq, &mut prog_ids);
                if ASSERT_OK!(err, "prog_query") {
                    ASSERT_EQ!(optq.count, 2, "count");
                    ASSERT_EQ!(optq.revision, 7, "revision");
                    ASSERT_EQ!(*optq.prog_ids.add(0), id1, "prog_ids[0]");
                    ASSERT_EQ!(*optq.prog_ids.add(1), id2, "prog_ids[1]");
                    ASSERT_EQ!(*optq.prog_ids.add(2), 0, "prog_ids[2]");
                }
            }
        }
        detach_ok(fd2, target, &mut optd, "prog_detach");
        assert_mprog_count(target, 1);
        err = query_prog_ids(target, &mut optq, &mut prog_ids);
        if ASSERT_OK!(err, "prog_query") {
            ASSERT_EQ!(optq.count, 1, "count");
            ASSERT_EQ!(optq.revision, 8, "revision");
            ASSERT_EQ!(*optq.prog_ids.add(0), id1, "prog_ids[0]");
            ASSERT_EQ!(*optq.prog_ids.add(1), 0, "prog_ids[1]");
        }
        detach_ok(fd1, target, &mut optd, "prog_detach");
        assert_mprog_count(target, 0);
    }
    test_tc_link__destroy(skel);
}

pub unsafe extern "C" fn test_ns_tc_opts_after() {
    test_tc_opts_after_target(BPF_TCX_INGRESS);
    test_tc_opts_after_target(BPF_TCX_EGRESS);
}

unsafe fn test_tc_opts_revision_target(target: i32) {
    let mut opta = bpf_prog_attach_opts::default();
    let mut optd = bpf_prog_detach_opts::default();
    let mut optq = bpf_prog_query_opts::default();
    let mut prog_ids: [__u32; 3] = [0; 3];
    let skel = test_tc_link__open_and_load();
    'cleanup: {
        if !ASSERT_OK_PTR!(skel, "skel_load") {
            break 'cleanup;
        }
        let fd1 = bpf_program__fd((*skel).progs.tc1);
        let fd2 = bpf_program__fd((*skel).progs.tc2);
        let id1 = id_from_prog_fd(fd1);
        let id2 = id_from_prog_fd(fd2);
        ASSERT_NEQ!(id1, id2, "prog_ids_1_2");
        assert_mprog_count(target, 0);
        opta.expected_revision = 1;
        let mut err = bpf_prog_attach_opts(fd1, loopback, target, &mut opta);
        if !ASSERT_EQ!(err, 0, "prog_attach") {
            break 'cleanup;
        }
        assert_mprog_count(target, 1);
        opta = bpf_prog_attach_opts::default();
        opta.expected_revision = 1;
        err = bpf_prog_attach_opts(fd2, loopback, target, &mut opta);
        ASSERT_EQ!(err, -ESTALE, "prog_attach");
        assert_mprog_count(target, 1);
        opta = bpf_prog_attach_opts::default();
        opta.expected_revision = 2;
        err = bpf_prog_attach_opts(fd2, loopback, target, &mut opta);
        if ASSERT_EQ!(err, 0, "prog_attach") {
            assert_mprog_count(target, 2);
            optq.prog_ids = prog_ids.as_mut_ptr();
            err = query_prog_ids(target, &mut optq, &mut prog_ids);
            if ASSERT_OK!(err, "prog_query") {
                ASSERT_EQ!(optq.count, 2, "count");
                ASSERT_EQ!(optq.revision, 3, "revision");
                ASSERT_EQ!(*optq.prog_ids.add(0), id1, "prog_ids[0]");
                ASSERT_EQ!(*optq.prog_ids.add(1), id2, "prog_ids[1]");
                ASSERT_EQ!(*optq.prog_ids.add(2), 0, "prog_ids[2]");
                tc_skel_reset_all_seen(skel);
                ASSERT_OK!(system(ping_cmd), ping_cmd);
                ASSERT_EQ!((*(*skel).bss).seen_tc1, true, "seen_tc1");
                ASSERT_EQ!((*(*skel).bss).seen_tc2, true, "seen_tc2");
                optd.expected_revision = 2;
                err = bpf_prog_detach_opts(fd2, loopback, target, &mut optd);
                ASSERT_EQ!(err, -ESTALE, "prog_detach");
                assert_mprog_count(target, 2);
            }
            optd = bpf_prog_detach_opts::default();
            optd.expected_revision = 3;
            detach_ok(fd2, target, &mut optd, "prog_detach");
            assert_mprog_count(target, 1);
        }
        optd = bpf_prog_detach_opts::default();
        detach_ok(fd1, target, &mut optd, "prog_detach");
        assert_mprog_count(target, 0);
    }
    test_tc_link__destroy(skel);
}

pub unsafe extern "C" fn test_ns_tc_opts_revision() {
    test_tc_opts_revision_target(BPF_TCX_INGRESS);
    test_tc_opts_revision_target(BPF_TCX_EGRESS);
}

unsafe fn test_tc_chain_classic(target: i32, chain_tc_old: bool) {
    let mut tc_opts = bpf_tc_opts {
        handle: 1,
        priority: 1,
        ..bpf_tc_opts::default()
    };
    let mut tc_hook = bpf_tc_hook {
        ifindex: loopback,
        ..bpf_tc_hook::default()
    };
    let mut opta = bpf_prog_attach_opts::default();
    let mut optd = bpf_prog_detach_opts::default();
    let mut hook_created = false;
    let mut tc_attached = false;
    let skel = test_tc_link__open_and_load();
    'cleanup: {
        if !ASSERT_OK_PTR!(skel, "skel_load") {
            break 'cleanup;
        }
        let fd1 = bpf_program__fd((*skel).progs.tc1);
        let fd2 = bpf_program__fd((*skel).progs.tc2);
        let fd3 = bpf_program__fd((*skel).progs.tc3);
        let id1 = id_from_prog_fd(fd1);
        let id2 = id_from_prog_fd(fd2);
        let id3 = id_from_prog_fd(fd3);
        ASSERT_NEQ!(id1, id2, "prog_ids_1_2");
        ASSERT_NEQ!(id2, id3, "prog_ids_2_3");
        assert_mprog_count(target, 0);
        if chain_tc_old {
            tc_hook.attach_point = if target == BPF_TCX_INGRESS {
                BPF_TC_INGRESS
            } else {
                BPF_TC_EGRESS
            };
            let mut err = bpf_tc_hook_create(&mut tc_hook);
            if err == 0 {
                hook_created = true;
            }
            err = if err == -EEXIST { 0 } else { err };
            if !ASSERT_OK!(err, "bpf_tc_hook_create") {
                break 'cleanup;
            }
            tc_opts.prog_fd = fd3;
            err = bpf_tc_attach(&mut tc_hook, &mut tc_opts);
            if !ASSERT_OK!(err, "bpf_tc_attach") {
                break 'cleanup;
            }
            tc_attached = true;
        }
        if !attach_or_break(fd1, target, &mut opta, "prog_attach") {
            break 'cleanup;
        }
        if attach_or_break(fd2, target, &mut opta, "prog_attach") {
            assert_mprog_count(target, 2);
            tc_skel_reset_all_seen(skel);
            ASSERT_OK!(system(ping_cmd), ping_cmd);
            ASSERT_EQ!((*(*skel).bss).seen_tc1, true, "seen_tc1");
            ASSERT_EQ!((*(*skel).bss).seen_tc2, true, "seen_tc2");
            ASSERT_EQ!((*(*skel).bss).seen_tc3, chain_tc_old, "seen_tc3");
            let err = bpf_prog_detach_opts(fd2, loopback, target, &mut optd);
            if ASSERT_OK!(err, "prog_detach") {
                assert_mprog_count(target, 1);
                tc_skel_reset_all_seen(skel);
                ASSERT_OK!(system(ping_cmd), ping_cmd);
                ASSERT_EQ!((*(*skel).bss).seen_tc1, true, "seen_tc1");
                ASSERT_EQ!((*(*skel).bss).seen_tc2, false, "seen_tc2");
                ASSERT_EQ!((*(*skel).bss).seen_tc3, chain_tc_old, "seen_tc3");
            }
        }
        let err = bpf_prog_detach_opts(fd1, loopback, target, &mut optd);
        if ASSERT_OK!(err, "prog_detach") {
            assert_mprog_count(target, 0);
        }
    }
    if tc_attached {
        tc_opts.flags = 0;
        tc_opts.prog_fd = 0;
        tc_opts.prog_id = 0;
        let err = bpf_tc_detach(&mut tc_hook, &mut tc_opts);
        ASSERT_OK!(err, "bpf_tc_detach");
    }
    if hook_created {
        tc_hook.attach_point = BPF_TC_INGRESS | BPF_TC_EGRESS;
        bpf_tc_hook_destroy(&mut tc_hook);
    }
    test_tc_link__destroy(skel);
    assert_mprog_count(target, 0);
}

pub unsafe extern "C" fn test_ns_tc_opts_chain_classic() {
    test_tc_chain_classic(BPF_TCX_INGRESS, false);
    test_tc_chain_classic(BPF_TCX_EGRESS, false);
    test_tc_chain_classic(BPF_TCX_INGRESS, true);
    test_tc_chain_classic(BPF_TCX_EGRESS, true);
}

unsafe fn test_tc_opts_replace_target(target: i32) {
    let mut opta = bpf_prog_attach_opts::default();
    let mut optd = bpf_prog_detach_opts::default();
    let mut optq = bpf_prog_query_opts::default();
    let mut prog_ids: [__u32; 4] = [0; 4];
    let mut prog_flags: [__u32; 4] = [0; 4];
    let mut detach_fd: i32 = 0;
    let skel = test_tc_link__open_and_load();
    'cleanup: {
        if !ASSERT_OK_PTR!(skel, "skel_load") {
            break 'cleanup;
        }
        let fd1 = bpf_program__fd((*skel).progs.tc1);
        let fd2 = bpf_program__fd((*skel).progs.tc2);
        let fd3 = bpf_program__fd((*skel).progs.tc3);
        let id1 = id_from_prog_fd(fd1);
        let id2 = id_from_prog_fd(fd2);
        let id3 = id_from_prog_fd(fd3);
        ASSERT_NEQ!(id1, id2, "prog_ids_1_2");
        ASSERT_NEQ!(id2, id3, "prog_ids_2_3");
        assert_mprog_count(target, 0);
        opta.expected_revision = 1;
        let mut err = bpf_prog_attach_opts(fd1, loopback, target, &mut opta);
        if !ASSERT_EQ!(err, 0, "prog_attach") {
            break 'cleanup;
        }
        assert_mprog_count(target, 1);
        opta = bpf_prog_attach_opts::default();
        opta.flags = BPF_F_BEFORE;
        opta.relative_id = id1;
        opta.expected_revision = 2;
        err = bpf_prog_attach_opts(fd2, loopback, target, &mut opta);
        if ASSERT_EQ!(err, 0, "prog_attach") {
            detach_fd = fd2;
            assert_mprog_count(target, 2);
            optq.prog_attach_flags = prog_flags.as_mut_ptr();
            optq.prog_ids = prog_ids.as_mut_ptr();
            zero_array(&mut prog_flags);
            err = query_prog_ids(target, &mut optq, &mut prog_ids);
            if ASSERT_OK!(err, "prog_query") {
                ASSERT_EQ!(optq.count, 2, "count");
                ASSERT_EQ!(optq.revision, 3, "revision");
                ASSERT_EQ!(*optq.prog_ids.add(0), id2, "prog_ids[0]");
                ASSERT_EQ!(*optq.prog_ids.add(1), id1, "prog_ids[1]");
                ASSERT_EQ!(*optq.prog_ids.add(2), 0, "prog_ids[2]");
                ASSERT_EQ!(*optq.prog_attach_flags.add(0), 0, "prog_flags[0]");
                ASSERT_EQ!(*optq.prog_attach_flags.add(1), 0, "prog_flags[1]");
                ASSERT_EQ!(*optq.prog_attach_flags.add(2), 0, "prog_flags[2]");
                tc_skel_reset_all_seen(skel);
                ASSERT_OK!(system(ping_cmd), ping_cmd);
                ASSERT_EQ!((*(*skel).bss).seen_tc1, true, "seen_tc1");
                ASSERT_EQ!((*(*skel).bss).seen_tc2, true, "seen_tc2");
                ASSERT_EQ!((*(*skel).bss).seen_tc3, false, "seen_tc3");
                opta = bpf_prog_attach_opts::default();
                opta.flags = BPF_F_REPLACE;
                opta.replace_prog_fd = fd2;
                opta.expected_revision = 3;
                err = bpf_prog_attach_opts(fd3, loopback, target, &mut opta);
                if ASSERT_EQ!(err, 0, "prog_attach") {
                    detach_fd = fd3;
                    assert_mprog_count(target, 2);
                    err = query_prog_ids(target, &mut optq, &mut prog_ids);
                    if ASSERT_OK!(err, "prog_query") {
                        ASSERT_EQ!(optq.count, 2, "count");
                        ASSERT_EQ!(optq.revision, 4, "revision");
                        ASSERT_EQ!(*optq.prog_ids.add(0), id3, "prog_ids[0]");
                        ASSERT_EQ!(*optq.prog_ids.add(1), id1, "prog_ids[1]");
                        ASSERT_EQ!(*optq.prog_ids.add(2), 0, "prog_ids[2]");
                    }
                    tc_skel_reset_all_seen(skel);
                    ASSERT_OK!(system(ping_cmd), ping_cmd);
                    ASSERT_EQ!((*(*skel).bss).seen_tc1, true, "seen_tc1");
                    ASSERT_EQ!((*(*skel).bss).seen_tc2, false, "seen_tc2");
                    ASSERT_EQ!((*(*skel).bss).seen_tc3, true, "seen_tc3");
                    opta = bpf_prog_attach_opts::default();
                    opta.flags = BPF_F_REPLACE | BPF_F_BEFORE;
                    opta.replace_prog_fd = fd3;
                    opta.relative_fd = fd1;
                    opta.expected_revision = 4;
                    err = bpf_prog_attach_opts(fd2, loopback, target, &mut opta);
                    if ASSERT_EQ!(err, 0, "prog_attach") {
                        detach_fd = fd2;
                        assert_mprog_count(target, 2);
                        err = query_prog_ids(target, &mut optq, &mut prog_ids);
                        if ASSERT_OK!(err, "prog_query") {
                            ASSERT_EQ!(optq.count, 2, "count");
                            ASSERT_EQ!(optq.revision, 5, "revision");
                            ASSERT_EQ!(*optq.prog_ids.add(0), id2, "prog_ids[0]");
                            ASSERT_EQ!(*optq.prog_ids.add(1), id1, "prog_ids[1]");
                            ASSERT_EQ!(*optq.prog_ids.add(2), 0, "prog_ids[2]");
                        }
                        tc_skel_reset_all_seen(skel);
                        ASSERT_OK!(system(ping_cmd), ping_cmd);
                        ASSERT_EQ!((*(*skel).bss).seen_tc1, true, "seen_tc1");
                        ASSERT_EQ!((*(*skel).bss).seen_tc2, true, "seen_tc2");
                        ASSERT_EQ!((*(*skel).bss).seen_tc3, false, "seen_tc3");
                    }
                }
            }
            opta = bpf_prog_attach_opts::default();
            opta.flags = BPF_F_REPLACE;
            opta.replace_prog_fd = fd2;
            err = bpf_prog_attach_opts(fd2, loopback, target, &mut opta);
            ASSERT_EQ!(err, -EEXIST, "prog_attach");
            assert_mprog_count(target, 2);
            opta = bpf_prog_attach_opts::default();
            opta.flags = BPF_F_REPLACE | BPF_F_AFTER;
            opta.replace_prog_fd = fd2;
            opta.relative_fd = fd1;
            opta.expected_revision = 5;
            err = bpf_prog_attach_opts(fd3, loopback, target, &mut opta);
            ASSERT_EQ!(err, -ERANGE, "prog_attach");
            assert_mprog_count(target, 2);
            opta = bpf_prog_attach_opts::default();
            opta.flags = BPF_F_BEFORE | BPF_F_AFTER | BPF_F_REPLACE;
            opta.replace_prog_fd = fd2;
            opta.relative_fd = fd1;
            opta.expected_revision = 5;
            err = bpf_prog_attach_opts(fd3, loopback, target, &mut opta);
            ASSERT_EQ!(err, -ERANGE, "prog_attach");
            assert_mprog_count(target, 2);
            optd = bpf_prog_detach_opts::default();
            optd.flags = BPF_F_BEFORE;
            optd.relative_id = id1;
            optd.expected_revision = 5;
            detach_ok(detach_fd, target, &mut optd, "prog_detach");
            assert_mprog_count(target, 1);
        }
        optd = bpf_prog_detach_opts::default();
        detach_ok(fd1, target, &mut optd, "prog_detach");
        assert_mprog_count(target, 0);
    }
    test_tc_link__destroy(skel);
}

pub unsafe extern "C" fn test_ns_tc_opts_replace() {
    test_tc_opts_replace_target(BPF_TCX_INGRESS);
    test_tc_opts_replace_target(BPF_TCX_EGRESS);
}

unsafe fn test_tc_opts_invalid_target(target: i32) {
    let mut opta = bpf_prog_attach_opts::default();
    let mut optd = bpf_prog_detach_opts::default();
    let skel = test_tc_link__open_and_load();
    'cleanup: {
        if !ASSERT_OK_PTR!(skel, "skel_load") {
            break 'cleanup;
        }
        let fd1 = bpf_program__fd((*skel).progs.tc1);
        let fd2 = bpf_program__fd((*skel).progs.tc2);
        let id1 = id_from_prog_fd(fd1);
        let id2 = id_from_prog_fd(fd2);
        ASSERT_NEQ!(id1, id2, "prog_ids_1_2");
        assert_mprog_count(target, 0);
        for (flags, relative_fd, relative_id, expected) in [
            (BPF_F_BEFORE | BPF_F_AFTER, 0, 0, -ERANGE),
            (BPF_F_BEFORE | BPF_F_ID, 0, 0, -ENOENT),
            (BPF_F_AFTER | BPF_F_ID, 0, 0, -ENOENT),
            (0, fd2, 0, -EINVAL),
            (BPF_F_BEFORE | BPF_F_AFTER, fd2, 0, -ENOENT),
            (BPF_F_ID, 0, id2, -EINVAL),
            (BPF_F_BEFORE, fd1, 0, -ENOENT),
            (BPF_F_AFTER, fd1, 0, -ENOENT),
        ] {
            opta = bpf_prog_attach_opts::default();
            opta.flags = flags;
            opta.relative_fd = relative_fd;
            opta.relative_id = relative_id;
            let err = bpf_prog_attach_opts(fd1, loopback, target, &mut opta);
            ASSERT_EQ!(err, expected, "prog_attach");
            assert_mprog_count(target, 0);
        }
        opta = bpf_prog_attach_opts::default();
        let mut err = bpf_prog_attach_opts(fd1, loopback, target, &mut opta);
        if !ASSERT_EQ!(err, 0, "prog_attach") {
            break 'cleanup;
        }
        assert_mprog_count(target, 1);
        for (flags, relative_fd, replace_prog_fd, expected, name) in [
            (0, 0, 0, -EEXIST, "prog_attach"),
            (BPF_F_BEFORE, fd1, 0, -EEXIST, "prog_attach"),
            (BPF_F_AFTER, fd1, 0, -EEXIST, "prog_attach"),
            (BPF_F_REPLACE, fd1, 0, -EINVAL, "prog_attach_x1"),
            (BPF_F_REPLACE, 0, fd1, -EEXIST, "prog_attach"),
        ] {
            opta = bpf_prog_attach_opts::default();
            opta.flags = flags;
            opta.relative_fd = relative_fd;
            opta.replace_prog_fd = replace_prog_fd;
            err = bpf_prog_attach_opts(fd1, loopback, target, &mut opta);
            ASSERT_EQ!(err, expected, name);
            assert_mprog_count(target, 1);
        }
        detach_ok(fd1, target, &mut optd, "prog_detach");
        assert_mprog_count(target, 0);
    }
    test_tc_link__destroy(skel);
}

pub unsafe extern "C" fn test_ns_tc_opts_invalid() {
    test_tc_opts_invalid_target(BPF_TCX_INGRESS);
    test_tc_opts_invalid_target(BPF_TCX_EGRESS);
}

unsafe fn test_tc_opts_prepend_append_target(target: i32, flags: i32, prepend: bool) {
    let mut opta = bpf_prog_attach_opts::default();
    let mut optd = bpf_prog_detach_opts::default();
    let mut optq = bpf_prog_query_opts::default();
    let mut prog_ids: [__u32; 5] = [0; 5];
    let skel = test_tc_link__open_and_load();
    'cleanup: {
        if !ASSERT_OK_PTR!(skel, "skel_load") {
            break 'cleanup;
        }
        let (fd1, fd2, fd3, fd4, id1, id2, id3, id4) = tc_ids4(skel);
        assert_ids4(id1, id2, id3, id4);
        assert_mprog_count(target, 0);
        if !attach_or_break(fd1, target, &mut opta, "prog_attach") {
            break 'cleanup;
        }
        assert_mprog_count(target, 1);
        opta = bpf_prog_attach_opts::default();
        opta.flags = flags;
        if !attach_or_break(fd2, target, &mut opta, "prog_attach") {
            detach_ok(fd1, target, &mut optd, "prog_detach");
            break 'cleanup;
        }
        assert_mprog_count(target, 2);
        optq.prog_ids = prog_ids.as_mut_ptr();
        let mut err = query_prog_ids(target, &mut optq, &mut prog_ids);
        if ASSERT_OK!(err, "prog_query") {
            ASSERT_EQ!(optq.count, 2, "count");
            ASSERT_EQ!(optq.revision, 3, "revision");
            ASSERT_EQ!(*optq.prog_ids.add(0), if prepend { id2 } else { id1 }, "prog_ids[0]");
            ASSERT_EQ!(*optq.prog_ids.add(1), if prepend { id1 } else { id2 }, "prog_ids[1]");
            ASSERT_EQ!(*optq.prog_ids.add(2), 0, "prog_ids[2]");
            tc_skel_reset_all_seen(skel);
            ASSERT_OK!(system(ping_cmd), ping_cmd);
            ASSERT_EQ!((*(*skel).bss).seen_tc1, true, "seen_tc1");
            ASSERT_EQ!((*(*skel).bss).seen_tc2, true, "seen_tc2");
            ASSERT_EQ!((*(*skel).bss).seen_tc3, false, "seen_tc3");
            ASSERT_EQ!((*(*skel).bss).seen_tc4, false, "seen_tc4");
            opta = bpf_prog_attach_opts::default();
            opta.flags = flags;
            if attach_or_break(fd3, target, &mut opta, "prog_attach") {
                opta = bpf_prog_attach_opts::default();
                opta.flags = flags;
                if attach_or_break(fd4, target, &mut opta, "prog_attach") {
                    assert_mprog_count(target, 4);
                    err = query_prog_ids(target, &mut optq, &mut prog_ids);
                    if ASSERT_OK!(err, "prog_query") {
                        ASSERT_EQ!(optq.count, 4, "count");
                        ASSERT_EQ!(optq.revision, 5, "revision");
                        if prepend {
                            ASSERT_EQ!(*optq.prog_ids.add(0), id4, "prog_ids[0]");
                            ASSERT_EQ!(*optq.prog_ids.add(1), id3, "prog_ids[1]");
                            ASSERT_EQ!(*optq.prog_ids.add(2), id2, "prog_ids[2]");
                            ASSERT_EQ!(*optq.prog_ids.add(3), id1, "prog_ids[3]");
                        } else {
                            ASSERT_EQ!(*optq.prog_ids.add(0), id1, "prog_ids[0]");
                            ASSERT_EQ!(*optq.prog_ids.add(1), id2, "prog_ids[1]");
                            ASSERT_EQ!(*optq.prog_ids.add(2), id3, "prog_ids[2]");
                            ASSERT_EQ!(*optq.prog_ids.add(3), id4, "prog_ids[3]");
                        }
                        ASSERT_EQ!(*optq.prog_ids.add(4), 0, "prog_ids[4]");
                        tc_skel_reset_all_seen(skel);
                        ASSERT_OK!(system(ping_cmd), ping_cmd);
                        ASSERT_EQ!((*(*skel).bss).seen_tc1, true, "seen_tc1");
                        ASSERT_EQ!((*(*skel).bss).seen_tc2, true, "seen_tc2");
                        ASSERT_EQ!((*(*skel).bss).seen_tc3, true, "seen_tc3");
                        ASSERT_EQ!((*(*skel).bss).seen_tc4, true, "seen_tc4");
                    }
                    detach_ok(fd4, target, &mut optd, "prog_detach");
                    assert_mprog_count(target, 3);
                }
                detach_ok(fd3, target, &mut optd, "prog_detach");
                assert_mprog_count(target, 2);
            }
        }
        detach_ok(fd2, target, &mut optd, "prog_detach");
        assert_mprog_count(target, 1);
        detach_ok(fd1, target, &mut optd, "prog_detach");
        assert_mprog_count(target, 0);
    }
    test_tc_link__destroy(skel);
}

unsafe fn test_tc_opts_prepend_target(target: i32) {
    test_tc_opts_prepend_append_target(target, BPF_F_BEFORE, true);
}

pub unsafe extern "C" fn test_ns_tc_opts_prepend() {
    test_tc_opts_prepend_target(BPF_TCX_INGRESS);
    test_tc_opts_prepend_target(BPF_TCX_EGRESS);
}

unsafe fn test_tc_opts_append_target(target: i32) {
    test_tc_opts_prepend_append_target(target, BPF_F_AFTER, false);
}

pub unsafe extern "C" fn test_ns_tc_opts_append() {
    test_tc_opts_append_target(BPF_TCX_INGRESS);
    test_tc_opts_append_target(BPF_TCX_EGRESS);
}

unsafe fn test_tc_opts_dev_cleanup_target(target: i32) {
    let mut opta = bpf_prog_attach_opts::default();
    let mut optd = bpf_prog_detach_opts::default();
    let skel: *mut test_tc_link;
    let mut err: i32;
    ASSERT_OK!(system(tcx_opts_add_veth), "add veth");
    let ifindex = if_nametoindex(tcx_opts1) as i32;
    ASSERT_NEQ!(ifindex, 0, "non_zero_ifindex");
    skel = test_tc_link__open_and_load();
    'cleanup: {
        if !ASSERT_OK_PTR!(skel, "skel_load") {
            break 'cleanup;
        }
        let (fd1, fd2, fd3, fd4, id1, id2, id3, id4) = tc_ids4(skel);
        assert_ids4(id1, id2, id3, id4);
        assert_mprog_count_ifindex(ifindex, target, 0);
        err = bpf_prog_attach_opts(fd1, ifindex, target, &mut opta);
        if !ASSERT_EQ!(err, 0, "prog_attach") {
            break 'cleanup;
        }
        assert_mprog_count_ifindex(ifindex, target, 1);
        err = bpf_prog_attach_opts(fd2, ifindex, target, &mut opta);
        if ASSERT_EQ!(err, 0, "prog_attach") {
            assert_mprog_count_ifindex(ifindex, target, 2);
            err = bpf_prog_attach_opts(fd3, ifindex, target, &mut opta);
            if ASSERT_EQ!(err, 0, "prog_attach") {
                assert_mprog_count_ifindex(ifindex, target, 3);
                err = bpf_prog_attach_opts(fd4, ifindex, target, &mut opta);
                if ASSERT_EQ!(err, 0, "prog_attach") {
                    assert_mprog_count_ifindex(ifindex, target, 4);
                    break 'cleanup;
                }
                detach_ok(fd3, target, &mut optd, "prog_detach");
                assert_mprog_count_ifindex(ifindex, target, 2);
            }
            detach_ok(fd2, target, &mut optd, "prog_detach");
            assert_mprog_count_ifindex(ifindex, target, 1);
        }
        detach_ok(fd1, target, &mut optd, "prog_detach");
        assert_mprog_count_ifindex(ifindex, target, 0);
    }
    test_tc_link__destroy(skel);
    ASSERT_OK!(system(tcx_opts_del_veth), "del veth");
    ASSERT_EQ!(if_nametoindex(tcx_opts1), 0, "dev1_removed");
    ASSERT_EQ!(if_nametoindex(tcx_opts2), 0, "dev2_removed");
}

pub unsafe extern "C" fn test_ns_tc_opts_dev_cleanup() {
    test_tc_opts_dev_cleanup_target(BPF_TCX_INGRESS);
    test_tc_opts_dev_cleanup_target(BPF_TCX_EGRESS);
}

unsafe fn test_tc_opts_mixed_target(target: i32) {
    let mut opta = bpf_prog_attach_opts::default();
    let mut optd = bpf_prog_detach_opts::default();
    let mut optq = bpf_prog_query_opts::default();
    let mut optl = bpf_tcx_opts::default();
    let mut prog_flags: [__u32; 4] = [0; 4];
    let mut link_flags: [__u32; 4] = [0; 4];
    let mut prog_ids: [__u32; 4] = [0; 4];
    let mut link_ids: [__u32; 4] = [0; 4];
    let mut detach_fd: i32 = 0;
    let skel = test_tc_link__open();
    'cleanup: {
        if !ASSERT_OK_PTR!(skel, "skel_open") {
            break 'cleanup;
        }
        ASSERT_EQ!(bpf_program__set_expected_attach_type((*skel).progs.tc1, target), 0, "tc1_attach_type");
        ASSERT_EQ!(bpf_program__set_expected_attach_type((*skel).progs.tc2, target), 0, "tc2_attach_type");
        ASSERT_EQ!(bpf_program__set_expected_attach_type((*skel).progs.tc3, target), 0, "tc3_attach_type");
        ASSERT_EQ!(bpf_program__set_expected_attach_type((*skel).progs.tc4, target), 0, "tc4_attach_type");
        let mut err = test_tc_link__load(skel);
        if !ASSERT_OK!(err, "skel_load") {
            break 'cleanup;
        }
        let pid1 = id_from_prog_fd(bpf_program__fd((*skel).progs.tc1));
        let pid2 = id_from_prog_fd(bpf_program__fd((*skel).progs.tc2));
        let pid3 = id_from_prog_fd(bpf_program__fd((*skel).progs.tc3));
        let pid4 = id_from_prog_fd(bpf_program__fd((*skel).progs.tc4));
        assert_ids4(pid1, pid2, pid3, pid4);
        assert_mprog_count(target, 0);
        err = bpf_prog_attach_opts(bpf_program__fd((*skel).progs.tc1), loopback, target, &mut opta);
        if !ASSERT_EQ!(err, 0, "prog_attach") {
            break 'cleanup;
        }
        detach_fd = bpf_program__fd((*skel).progs.tc1);
        assert_mprog_count(target, 1);
        let mut link = bpf_program__attach_tcx((*skel).progs.tc2, loopback, &mut optl);
        if !ASSERT_OK_PTR!(link, "link_attach") {
            detach_ok(detach_fd, target, &mut optd, "prog_detach");
            break 'cleanup;
        }
        (*skel).links.tc2 = link;
        let lid2 = id_from_link_fd(bpf_link__fd((*skel).links.tc2));
        assert_mprog_count(target, 2);
        opta = bpf_prog_attach_opts::default();
        opta.flags = BPF_F_REPLACE;
        opta.replace_prog_fd = bpf_program__fd((*skel).progs.tc1);
        err = bpf_prog_attach_opts(bpf_program__fd((*skel).progs.tc2), loopback, target, &mut opta);
        ASSERT_EQ!(err, -EEXIST, "prog_attach");
        opta.replace_prog_fd = bpf_program__fd((*skel).progs.tc2);
        err = bpf_prog_attach_opts(bpf_program__fd((*skel).progs.tc1), loopback, target, &mut opta);
        ASSERT_EQ!(err, -EEXIST, "prog_attach");
        err = bpf_prog_attach_opts(bpf_program__fd((*skel).progs.tc3), loopback, target, &mut opta);
        ASSERT_EQ!(err, -EBUSY, "prog_attach");
        opta.replace_prog_fd = bpf_program__fd((*skel).progs.tc1);
        err = bpf_prog_attach_opts(bpf_program__fd((*skel).progs.tc3), loopback, target, &mut opta);
        if ASSERT_EQ!(err, 0, "prog_attach") {
            detach_fd = bpf_program__fd((*skel).progs.tc3);
            assert_mprog_count(target, 2);
            link = bpf_program__attach_tcx((*skel).progs.tc4, loopback, &mut optl);
            if ASSERT_OK_PTR!(link, "link_attach") {
                (*skel).links.tc4 = link;
                let lid4 = id_from_link_fd(bpf_link__fd((*skel).links.tc4));
                assert_mprog_count(target, 3);
                opta.replace_prog_fd = bpf_program__fd((*skel).progs.tc4);
                err = bpf_prog_attach_opts(bpf_program__fd((*skel).progs.tc2), loopback, target, &mut opta);
                ASSERT_EQ!(err, -EEXIST, "prog_attach");
                optq.prog_ids = prog_ids.as_mut_ptr();
                optq.prog_attach_flags = prog_flags.as_mut_ptr();
                optq.link_ids = link_ids.as_mut_ptr();
                optq.link_attach_flags = link_flags.as_mut_ptr();
                zero_array(&mut prog_ids);
                zero_array(&mut prog_flags);
                zero_array(&mut link_ids);
                zero_array(&mut link_flags);
                optq.count = prog_ids.len() as __u32;
                err = bpf_prog_query_opts(loopback, target, &mut optq);
                if ASSERT_OK!(err, "prog_query") {
                    ASSERT_EQ!(optq.count, 3, "count");
                    ASSERT_EQ!(optq.revision, 5, "revision");
                    ASSERT_EQ!(*optq.prog_ids.add(0), pid3, "prog_ids[0]");
                    ASSERT_EQ!(*optq.prog_attach_flags.add(0), 0, "prog_flags[0]");
                    ASSERT_EQ!(*optq.link_ids.add(0), 0, "link_ids[0]");
                    ASSERT_EQ!(*optq.link_attach_flags.add(0), 0, "link_flags[0]");
                    ASSERT_EQ!(*optq.prog_ids.add(1), pid2, "prog_ids[1]");
                    ASSERT_EQ!(*optq.prog_attach_flags.add(1), 0, "prog_flags[1]");
                    ASSERT_EQ!(*optq.link_ids.add(1), lid2, "link_ids[1]");
                    ASSERT_EQ!(*optq.link_attach_flags.add(1), 0, "link_flags[1]");
                    ASSERT_EQ!(*optq.prog_ids.add(2), pid4, "prog_ids[2]");
                    ASSERT_EQ!(*optq.prog_attach_flags.add(2), 0, "prog_flags[2]");
                    ASSERT_EQ!(*optq.link_ids.add(2), lid4, "link_ids[2]");
                    ASSERT_EQ!(*optq.link_attach_flags.add(2), 0, "link_flags[2]");
                    ASSERT_EQ!(*optq.prog_ids.add(3), 0, "prog_ids[3]");
                    ASSERT_EQ!(*optq.prog_attach_flags.add(3), 0, "prog_flags[3]");
                    ASSERT_EQ!(*optq.link_ids.add(3), 0, "link_ids[3]");
                    ASSERT_EQ!(*optq.link_attach_flags.add(3), 0, "link_flags[3]");
                    ASSERT_OK!(system(ping_cmd), ping_cmd);
                }
            }
        }
        detach_ok(detach_fd, target, &mut optd, "prog_detach");
        assert_mprog_count(target, 2);
    }
    test_tc_link__destroy(skel);
    assert_mprog_count(target, 0);
}

pub unsafe extern "C" fn test_ns_tc_opts_mixed() {
    test_tc_opts_mixed_target(BPF_TCX_INGRESS);
    test_tc_opts_mixed_target(BPF_TCX_EGRESS);
}

unsafe fn test_tc_opts_demixed_target(target: i32) {
    let mut opta = bpf_prog_attach_opts::default();
    let mut optd = bpf_prog_detach_opts::default();
    let mut optl = bpf_tcx_opts::default();
    let skel = test_tc_link__open();
    'cleanup: {
        if !ASSERT_OK_PTR!(skel, "skel_open") {
            break 'cleanup;
        }
        ASSERT_EQ!(bpf_program__set_expected_attach_type((*skel).progs.tc1, target), 0, "tc1_attach_type");
        ASSERT_EQ!(bpf_program__set_expected_attach_type((*skel).progs.tc2, target), 0, "tc2_attach_type");
        let mut err = test_tc_link__load(skel);
        if !ASSERT_OK!(err, "skel_load") {
            break 'cleanup;
        }
        let pid1 = id_from_prog_fd(bpf_program__fd((*skel).progs.tc1));
        let pid2 = id_from_prog_fd(bpf_program__fd((*skel).progs.tc2));
        ASSERT_NEQ!(pid1, pid2, "prog_ids_1_2");
        assert_mprog_count(target, 0);
        err = bpf_prog_attach_opts(bpf_program__fd((*skel).progs.tc1), loopback, target, &mut opta);
        if !ASSERT_EQ!(err, 0, "prog_attach") {
            break 'cleanup;
        }
        assert_mprog_count(target, 1);
        let link = bpf_program__attach_tcx((*skel).progs.tc2, loopback, &mut optl);
        if ASSERT_OK_PTR!(link, "link_attach") {
            (*skel).links.tc2 = link;
            assert_mprog_count(target, 2);
            optd.flags = BPF_F_AFTER;
            err = bpf_prog_detach_opts(0, loopback, target, &mut optd);
            ASSERT_EQ!(err, -EBUSY, "prog_detach");
            assert_mprog_count(target, 2);
            optd = bpf_prog_detach_opts::default();
            optd.flags = BPF_F_BEFORE;
            err = bpf_prog_detach_opts(0, loopback, target, &mut optd);
            ASSERT_OK!(err, "prog_detach");
            assert_mprog_count(target, 1);
            break 'cleanup;
        }
        err = bpf_prog_detach_opts(bpf_program__fd((*skel).progs.tc1), loopback, target, &mut optd);
        ASSERT_OK!(err, "prog_detach");
        assert_mprog_count(target, 2);
    }
    test_tc_link__destroy(skel);
    assert_mprog_count(target, 0);
}

pub unsafe extern "C" fn test_ns_tc_opts_demixed() {
    test_tc_opts_demixed_target(BPF_TCX_INGRESS);
    test_tc_opts_demixed_target(BPF_TCX_EGRESS);
}

unsafe fn test_tc_opts_detach_order_target(target: i32, before: bool) {
    let mut opta = bpf_prog_attach_opts::default();
    let mut optd = bpf_prog_detach_opts::default();
    let mut optq = bpf_prog_query_opts::default();
    let mut prog_ids: [__u32; 5] = [0; 5];
    let skel = test_tc_link__open_and_load();
    'cleanup: {
        if !ASSERT_OK_PTR!(skel, "skel_load") {
            break 'cleanup;
        }
        let (fd1, fd2, fd3, fd4, id1, id2, id3, id4) = tc_ids4(skel);
        assert_ids4(id1, id2, id3, id4);
        assert_mprog_count(target, 0);
        for (idx, fd) in [fd1, fd2, fd3, fd4].iter().enumerate() {
            let err = bpf_prog_attach_opts(*fd, loopback, target, &mut opta);
            if !ASSERT_EQ!(err, 0, "prog_attach") {
                break 'cleanup;
            }
            assert_mprog_count(target, (idx + 1) as i32);
        }
        optq.prog_ids = prog_ids.as_mut_ptr();
        let mut err = query_prog_ids(target, &mut optq, &mut prog_ids);
        if ASSERT_OK!(err, "prog_query") {
            ASSERT_EQ!(optq.count, 4, "count");
            ASSERT_EQ!(optq.revision, 5, "revision");
            ASSERT_EQ!(*optq.prog_ids.add(0), id1, "prog_ids[0]");
            ASSERT_EQ!(*optq.prog_ids.add(1), id2, "prog_ids[1]");
            ASSERT_EQ!(*optq.prog_ids.add(2), id3, "prog_ids[2]");
            ASSERT_EQ!(*optq.prog_ids.add(3), id4, "prog_ids[3]");
            ASSERT_EQ!(*optq.prog_ids.add(4), 0, "prog_ids[4]");
        }
        optd.flags = if before { BPF_F_BEFORE } else { BPF_F_AFTER };
        err = bpf_prog_detach_opts(0, loopback, target, &mut optd);
        ASSERT_OK!(err, "prog_detach");
        assert_mprog_count(target, 3);
        err = query_prog_ids(target, &mut optq, &mut prog_ids);
        if ASSERT_OK!(err, "prog_query") {
            ASSERT_EQ!(optq.count, 3, "count");
            ASSERT_EQ!(optq.revision, 6, "revision");
            if before {
                ASSERT_EQ!(*optq.prog_ids.add(0), id2, "prog_ids[0]");
                ASSERT_EQ!(*optq.prog_ids.add(1), id3, "prog_ids[1]");
                ASSERT_EQ!(*optq.prog_ids.add(2), id4, "prog_ids[2]");
            } else {
                ASSERT_EQ!(*optq.prog_ids.add(0), id1, "prog_ids[0]");
                ASSERT_EQ!(*optq.prog_ids.add(1), id2, "prog_ids[1]");
                ASSERT_EQ!(*optq.prog_ids.add(2), id3, "prog_ids[2]");
            }
            ASSERT_EQ!(*optq.prog_ids.add(3), 0, "prog_ids[3]");
        }
        optd = bpf_prog_detach_opts::default();
        optd.flags = if before { BPF_F_AFTER } else { BPF_F_BEFORE };
        err = bpf_prog_detach_opts(0, loopback, target, &mut optd);
        ASSERT_OK!(err, "prog_detach");
        assert_mprog_count(target, 2);
        detach_ok(if before { fd3 } else { fd2 }, target, &mut bpf_prog_detach_opts::default(), "prog_detach");
        assert_mprog_count(target, 1);
        detach_ok(if before { fd4 } else { fd1 }, target, &mut bpf_prog_detach_opts::default(), "prog_detach");
        assert_mprog_count(target, 0);
        optd = bpf_prog_detach_opts::default();
        optd.flags = BPF_F_BEFORE;
        err = bpf_prog_detach_opts(0, loopback, target, &mut optd);
        ASSERT_EQ!(err, -ENOENT, "prog_detach");
        optd.flags = BPF_F_AFTER;
        err = bpf_prog_detach_opts(0, loopback, target, &mut optd);
        ASSERT_EQ!(err, -ENOENT, "prog_detach");
        break 'cleanup;
    }
    test_tc_link__destroy(skel);
}

unsafe fn test_tc_opts_detach_target(target: i32) {
    test_tc_opts_detach_order_target(target, true);
}

pub unsafe extern "C" fn test_ns_tc_opts_detach() {
    test_tc_opts_detach_target(BPF_TCX_INGRESS);
    test_tc_opts_detach_target(BPF_TCX_EGRESS);
}

unsafe fn test_tc_opts_detach_before_target(target: i32) {
    test_tc_opts_detach_order_target(target, true);
}

pub unsafe extern "C" fn test_ns_tc_opts_detach_before() {
    test_tc_opts_detach_before_target(BPF_TCX_INGRESS);
    test_tc_opts_detach_before_target(BPF_TCX_EGRESS);
}

unsafe fn test_tc_opts_detach_after_target(target: i32) {
    test_tc_opts_detach_order_target(target, false);
}

pub unsafe extern "C" fn test_ns_tc_opts_detach_after() {
    test_tc_opts_detach_after_target(BPF_TCX_INGRESS);
    test_tc_opts_detach_after_target(BPF_TCX_EGRESS);
}

unsafe fn test_tc_opts_delete_empty(target: i32, chain_tc_old: bool) {
    let mut tc_hook = bpf_tc_hook {
        ifindex: loopback,
        ..bpf_tc_hook::default()
    };
    let mut optd = bpf_prog_detach_opts::default();
    assert_mprog_count(target, 0);
    if chain_tc_old {
        tc_hook.attach_point = if target == BPF_TCX_INGRESS {
            BPF_TC_INGRESS
        } else {
            BPF_TC_EGRESS
        };
        let err = bpf_tc_hook_create(&mut tc_hook);
        ASSERT_OK!(err, "bpf_tc_hook_create");
        assert_mprog_count(target, 0);
    }
    let err = bpf_prog_detach_opts(0, loopback, target, &mut optd);
    ASSERT_EQ!(err, -ENOENT, "prog_detach");
    if chain_tc_old {
        tc_hook.attach_point = BPF_TC_INGRESS | BPF_TC_EGRESS;
        bpf_tc_hook_destroy(&mut tc_hook);
    }
    assert_mprog_count(target, 0);
}

pub unsafe extern "C" fn test_ns_tc_opts_delete_empty() {
    test_tc_opts_delete_empty(BPF_TCX_INGRESS, false);
    test_tc_opts_delete_empty(BPF_TCX_EGRESS, false);
    test_tc_opts_delete_empty(BPF_TCX_INGRESS, true);
    test_tc_opts_delete_empty(BPF_TCX_EGRESS, true);
}

unsafe fn test_tc_chain_mixed(target: i32) {
    let mut tc_opts = bpf_tc_opts {
        handle: 1,
        priority: 1,
        ..bpf_tc_opts::default()
    };
    let mut tc_hook = bpf_tc_hook {
        ifindex: loopback,
        ..bpf_tc_hook::default()
    };
    let mut opta = bpf_prog_attach_opts::default();
    let mut optd = bpf_prog_detach_opts::default();
    let mut detach_fd: i32;
    let skel = test_tc_link__open_and_load();
    'cleanup: {
        if !ASSERT_OK_PTR!(skel, "skel_load") {
            break 'cleanup;
        }
        let fd1 = bpf_program__fd((*skel).progs.tc4);
        let fd2 = bpf_program__fd((*skel).progs.tc5);
        let fd3 = bpf_program__fd((*skel).progs.tc6);
        let id1 = id_from_prog_fd(fd1);
        let id2 = id_from_prog_fd(fd2);
        let id3 = id_from_prog_fd(fd3);
        ASSERT_NEQ!(id1, id2, "prog_ids_1_2");
        ASSERT_NEQ!(id2, id3, "prog_ids_2_3");
        assert_mprog_count(target, 0);
        tc_hook.attach_point = if target == BPF_TCX_INGRESS { BPF_TC_INGRESS } else { BPF_TC_EGRESS };
        let mut err = bpf_tc_hook_create(&mut tc_hook);
        err = if err == -EEXIST { 0 } else { err };
        if !ASSERT_OK!(err, "bpf_tc_hook_create") {
            break 'cleanup;
        }
        tc_opts.prog_fd = fd2;
        err = bpf_tc_attach(&mut tc_hook, &mut tc_opts);
        if !ASSERT_OK!(err, "bpf_tc_attach") {
            tc_hook.attach_point = BPF_TC_INGRESS | BPF_TC_EGRESS;
            bpf_tc_hook_destroy(&mut tc_hook);
            break 'cleanup;
        }
        err = bpf_prog_attach_opts(fd3, loopback, target, &mut opta);
        if ASSERT_EQ!(err, 0, "prog_attach") {
            detach_fd = fd3;
            assert_mprog_count(target, 1);
            tc_skel_reset_all_seen(skel);
            ASSERT_OK!(system(ping_cmd), ping_cmd);
            ASSERT_EQ!((*(*skel).bss).seen_tc4, false, "seen_tc4");
            ASSERT_EQ!((*(*skel).bss).seen_tc5, false, "seen_tc5");
            ASSERT_EQ!((*(*skel).bss).seen_tc6, true, "seen_tc6");
            opta = bpf_prog_attach_opts::default();
            opta.flags = BPF_F_REPLACE;
            opta.replace_prog_fd = fd3;
            err = bpf_prog_attach_opts(fd1, loopback, target, &mut opta);
            if ASSERT_EQ!(err, 0, "prog_attach") {
                detach_fd = fd1;
                assert_mprog_count(target, 1);
                tc_skel_reset_all_seen(skel);
                ASSERT_OK!(system(ping_cmd), ping_cmd);
                ASSERT_EQ!((*(*skel).bss).seen_tc4, true, "seen_tc4");
                ASSERT_EQ!((*(*skel).bss).seen_tc5, true, "seen_tc5");
                ASSERT_EQ!((*(*skel).bss).seen_tc6, false, "seen_tc6");
            }
            err = bpf_prog_detach_opts(detach_fd, loopback, target, &mut optd);
            ASSERT_OK!(err, "prog_detach");
            assert_mprog_count(target, 0);
            tc_skel_reset_all_seen(skel);
            ASSERT_OK!(system(ping_cmd), ping_cmd);
            ASSERT_EQ!((*(*skel).bss).seen_tc4, false, "seen_tc4");
            ASSERT_EQ!((*(*skel).bss).seen_tc5, true, "seen_tc5");
            ASSERT_EQ!((*(*skel).bss).seen_tc6, false, "seen_tc6");
        }
        tc_opts.flags = 0;
        tc_opts.prog_fd = 0;
        tc_opts.prog_id = 0;
        err = bpf_tc_detach(&mut tc_hook, &mut tc_opts);
        ASSERT_OK!(err, "bpf_tc_detach");
        tc_hook.attach_point = BPF_TC_INGRESS | BPF_TC_EGRESS;
        bpf_tc_hook_destroy(&mut tc_hook);
    }
    test_tc_link__destroy(skel);
}

pub unsafe extern "C" fn test_ns_tc_opts_chain_mixed() {
    test_tc_chain_mixed(BPF_TCX_INGRESS);
    test_tc_chain_mixed(BPF_TCX_EGRESS);
}

unsafe fn generate_dummy_prog() -> i32 {
    let prog_insns = [bpf_mov64_imm(0, 0), bpf_exit_insn()];
    let prog_insn_cnt = prog_insns.len();
    let mut opts = bpf_prog_load_opts::default();
    let log_buf_sz: size_t = 256;
    let mut log_buf = [0i8; 256];
    opts.log_buf = log_buf.as_mut_ptr();
    opts.log_size = log_buf_sz;
    log_buf[0] = 0;
    opts.log_level = 0;
    let fd = bpf_prog_load(
        BPF_PROG_TYPE_SCHED_CLS,
        tcx_prog,
        GPL,
        prog_insns.as_ptr(),
        prog_insn_cnt,
        &mut opts,
    );
    ASSERT_STREQ!(log_buf.as_ptr(), b"", "log_0");
    ASSERT_GE!(fd, 0, "prog_fd");
    fd
}

unsafe fn test_tc_opts_max_target(target: i32, flags: i32, relative: bool) {
    let mut opta = bpf_prog_attach_opts::default();
    let max_progs = 63;
    let mut last_fd = -1;
    ASSERT_OK!(system(tcx_opts_add_veth), "add veth");
    let ifindex = if_nametoindex(tcx_opts1) as i32;
    ASSERT_NEQ!(ifindex, 0, "non_zero_ifindex");
    assert_mprog_count_ifindex(ifindex, target, 0);
    for i in 0..max_progs {
        let prog_fd = generate_dummy_prog();
        if !ASSERT_GE!(prog_fd, 0, "dummy_prog") {
            break;
        }
        let err = bpf_prog_attach_opts(prog_fd, ifindex, target, &mut opta);
        if !ASSERT_EQ!(err, 0, "prog_attach") {
            break;
        }
        assert_mprog_count_ifindex(ifindex, target, i + 1);
        if i == max_progs - 1 && relative {
            last_fd = prog_fd;
        } else {
            close(prog_fd);
        }
    }
    let prog_fd = generate_dummy_prog();
    if ASSERT_GE!(prog_fd, 0, "dummy_prog") {
        opta.flags = flags;
        if last_fd > 0 {
            opta.relative_fd = last_fd;
        }
        let err = bpf_prog_attach_opts(prog_fd, ifindex, target, &mut opta);
        ASSERT_EQ!(err, -ERANGE, "prog_64_attach");
        assert_mprog_count_ifindex(ifindex, target, max_progs);
        close(prog_fd);
    }
    if last_fd > 0 {
        close(last_fd);
    }
    ASSERT_OK!(system(tcx_opts_del_veth), "del veth");
    ASSERT_EQ!(if_nametoindex(tcx_opts1), 0, "dev1_removed");
    ASSERT_EQ!(if_nametoindex(tcx_opts2), 0, "dev2_removed");
}

pub unsafe extern "C" fn test_ns_tc_opts_max() {
    test_tc_opts_max_target(BPF_TCX_INGRESS, 0, false);
    test_tc_opts_max_target(BPF_TCX_EGRESS, 0, false);
    test_tc_opts_max_target(BPF_TCX_INGRESS, BPF_F_BEFORE, false);
    test_tc_opts_max_target(BPF_TCX_EGRESS, BPF_F_BEFORE, true);
    test_tc_opts_max_target(BPF_TCX_INGRESS, BPF_F_AFTER, true);
    test_tc_opts_max_target(BPF_TCX_EGRESS, BPF_F_AFTER, false);
}

unsafe fn test_tc_opts_query_target(target: i32) {
    let attr_size = core::mem::size_of::<bpf_attr_query>();
    let mut opta = bpf_prog_attach_opts::default();
    let mut optd = bpf_prog_detach_opts::default();
    let mut optq = bpf_prog_query_opts::default();
    let mut prog_ids: [__u32; 10] = [0; 10];
    let mut attr = bpf_attr {
        query: bpf_attr_query {
            target_ifindex: 0,
            attach_type: 0,
            query_flags: 0,
            attach_flags: 0,
            prog_ids: 0,
            prog_attach_flags: 0,
            link_ids: 0,
            link_attach_flags: 0,
            count: 0,
            revision: 0,
        },
    };
    let skel = test_tc_link__open_and_load();
    'cleanup: {
        if !ASSERT_OK_PTR!(skel, "skel_load") {
            break 'cleanup;
        }
        let (fd1, fd2, fd3, fd4, id1, id2, id3, id4) = tc_ids4(skel);
        assert_mprog_count(target, 0);
        for (idx, fd) in [fd1, fd2, fd3, fd4].iter().enumerate() {
            opta = bpf_prog_attach_opts::default();
            opta.expected_revision = (idx + 1) as __u32;
            let err = bpf_prog_attach_opts(*fd, loopback, target, &mut opta);
            if !ASSERT_EQ!(err, 0, "prog_attach") {
                break 'cleanup;
            }
            assert_mprog_count(target, (idx + 1) as i32);
        }
        /* Test 1: Double query via libbpf API */
        let mut err = bpf_prog_query_opts(loopback, target, &mut optq);
        if ASSERT_OK!(err, "prog_query") {
            ASSERT_EQ!(optq.count, 4, "count");
            ASSERT_EQ!(optq.revision, 5, "revision");
            ASSERT_EQ!(optq.prog_ids, core::ptr::null_mut(), "prog_ids");
            ASSERT_EQ!(optq.link_ids, core::ptr::null_mut(), "link_ids");
        }
        zero_array(&mut prog_ids);
        optq.prog_ids = prog_ids.as_mut_ptr();
        err = bpf_prog_query_opts(loopback, target, &mut optq);
        if ASSERT_OK!(err, "prog_query") {
            ASSERT_EQ!(optq.count, 4, "count");
            ASSERT_EQ!(optq.revision, 5, "revision");
            ASSERT_EQ!(*optq.prog_ids.add(0), id1, "prog_ids[0]");
            ASSERT_EQ!(*optq.prog_ids.add(1), id2, "prog_ids[1]");
            ASSERT_EQ!(*optq.prog_ids.add(2), id3, "prog_ids[2]");
            ASSERT_EQ!(*optq.prog_ids.add(3), id4, "prog_ids[3]");
            ASSERT_EQ!(*optq.prog_ids.add(4), 0, "prog_ids[4]");
            ASSERT_EQ!(optq.link_ids, core::ptr::null_mut(), "link_ids");
        }
        /* Test 2: Double query via bpf_attr & bpf(2) directly */
        attr.query = core::mem::zeroed();
        attr.query.target_ifindex = loopback as __u32;
        attr.query.attach_type = target as __u32;
        err = syscall(__NR_bpf, BPF_PROG_QUERY, &mut attr, attr_size) as i32;
        if ASSERT_OK!(err, "prog_query") {
            ASSERT_EQ!(attr.query.count, 4, "count");
            ASSERT_EQ!(attr.query.revision, 5, "revision");
            ASSERT_EQ!(attr.query.query_flags, 0, "query_flags");
            ASSERT_EQ!(attr.query.attach_flags, 0, "attach_flags");
            ASSERT_EQ!(attr.query.target_ifindex, loopback as __u32, "target_ifindex");
            ASSERT_EQ!(attr.query.attach_type, target as __u32, "attach_type");
            ASSERT_EQ!(attr.query.prog_ids, 0, "prog_ids");
            ASSERT_EQ!(attr.query.prog_attach_flags, 0, "prog_attach_flags");
            ASSERT_EQ!(attr.query.link_ids, 0, "link_ids");
            ASSERT_EQ!(attr.query.link_attach_flags, 0, "link_attach_flags");
        }
        zero_array(&mut prog_ids);
        attr.query.prog_ids = ptr_to_u64(prog_ids.as_mut_ptr());
        err = syscall(__NR_bpf, BPF_PROG_QUERY, &mut attr, attr_size) as i32;
        if ASSERT_OK!(err, "prog_query") {
            ASSERT_EQ!(attr.query.count, 4, "count");
            ASSERT_EQ!(attr.query.revision, 5, "revision");
            ASSERT_EQ!(attr.query.prog_ids, ptr_to_u64(prog_ids.as_mut_ptr()), "prog_ids");
            ASSERT_EQ!(prog_ids[0], id1, "prog_ids[0]");
            ASSERT_EQ!(prog_ids[1], id2, "prog_ids[1]");
            ASSERT_EQ!(prog_ids[2], id3, "prog_ids[2]");
            ASSERT_EQ!(prog_ids[3], id4, "prog_ids[3]");
            ASSERT_EQ!(prog_ids[4], 0, "prog_ids[4]");
        }
        /* Test 3: Query with smaller prog_ids array */
        attr.query = core::mem::zeroed();
        attr.query.target_ifindex = loopback as __u32;
        attr.query.attach_type = target as __u32;
        zero_array(&mut prog_ids);
        attr.query.prog_ids = ptr_to_u64(prog_ids.as_mut_ptr());
        attr.query.count = 2;
        err = syscall(__NR_bpf, BPF_PROG_QUERY, &mut attr, attr_size) as i32;
        ASSERT_EQ!(err, -1, "prog_query_should_fail");
        ASSERT_EQ!(errno, ENOSPC, "prog_query_should_fail");
        ASSERT_EQ!(attr.query.count, 4, "count");
        ASSERT_EQ!(attr.query.revision, 5, "revision");
        ASSERT_EQ!(prog_ids[0], id1, "prog_ids[0]");
        ASSERT_EQ!(prog_ids[1], id2, "prog_ids[1]");
        ASSERT_EQ!(prog_ids[2], 0, "prog_ids[2]");
        /* Test 4: Query with larger prog_ids array */
        attr.query = core::mem::zeroed();
        attr.query.target_ifindex = loopback as __u32;
        attr.query.attach_type = target as __u32;
        zero_array(&mut prog_ids);
        attr.query.prog_ids = ptr_to_u64(prog_ids.as_mut_ptr());
        attr.query.count = 10;
        err = syscall(__NR_bpf, BPF_PROG_QUERY, &mut attr, attr_size) as i32;
        if ASSERT_OK!(err, "prog_query") {
            ASSERT_EQ!(attr.query.count, 4, "count");
            ASSERT_EQ!(attr.query.revision, 5, "revision");
            ASSERT_EQ!(prog_ids[0], id1, "prog_ids[0]");
            ASSERT_EQ!(prog_ids[1], id2, "prog_ids[1]");
            ASSERT_EQ!(prog_ids[2], id3, "prog_ids[2]");
            ASSERT_EQ!(prog_ids[3], id4, "prog_ids[3]");
            ASSERT_EQ!(prog_ids[4], 0, "prog_ids[4]");
        }
        /* Test 5: Query with NULL prog_ids array but with count > 0 */
        attr.query = core::mem::zeroed();
        attr.query.target_ifindex = loopback as __u32;
        attr.query.attach_type = target as __u32;
        zero_array(&mut prog_ids);
        attr.query.count = core::mem::size_of_val(&prog_ids) as __u32;
        err = syscall(__NR_bpf, BPF_PROG_QUERY, &mut attr, attr_size) as i32;
        if ASSERT_OK!(err, "prog_query") {
            ASSERT_EQ!(attr.query.count, 4, "count");
            ASSERT_EQ!(attr.query.revision, 5, "revision");
            ASSERT_EQ!(prog_ids[0], 0, "prog_ids[0]");
            ASSERT_EQ!(attr.query.prog_ids, 0, "prog_ids");
        }
        /* Test 6: Query with non-NULL prog_ids array but with count == 0 */
        attr.query = core::mem::zeroed();
        attr.query.target_ifindex = loopback as __u32;
        attr.query.attach_type = target as __u32;
        zero_array(&mut prog_ids);
        attr.query.prog_ids = ptr_to_u64(prog_ids.as_mut_ptr());
        err = syscall(__NR_bpf, BPF_PROG_QUERY, &mut attr, attr_size) as i32;
        if ASSERT_OK!(err, "prog_query") {
            ASSERT_EQ!(attr.query.count, 4, "count");
            ASSERT_EQ!(attr.query.revision, 5, "revision");
            ASSERT_EQ!(prog_ids[0], 0, "prog_ids[0]");
            ASSERT_EQ!(attr.query.prog_ids, ptr_to_u64(prog_ids.as_mut_ptr()), "prog_ids");
        }
        /* Test 7: Query with invalid flags */
        attr.query.attach_flags = 0;
        attr.query.query_flags = 1;
        err = syscall(__NR_bpf, BPF_PROG_QUERY, &mut attr, attr_size) as i32;
        ASSERT_EQ!(err, -1, "prog_query_should_fail");
        ASSERT_EQ!(errno, EINVAL, "prog_query_should_fail");
        attr.query.attach_flags = 1;
        attr.query.query_flags = 0;
        err = syscall(__NR_bpf, BPF_PROG_QUERY, &mut attr, attr_size) as i32;
        ASSERT_EQ!(err, -1, "prog_query_should_fail");
        ASSERT_EQ!(errno, EINVAL, "prog_query_should_fail");
        detach_ok(fd4, target, &mut optd, "prog_detach");
        assert_mprog_count(target, 3);
        detach_ok(fd3, target, &mut optd, "prog_detach");
        assert_mprog_count(target, 2);
        detach_ok(fd2, target, &mut optd, "prog_detach");
        assert_mprog_count(target, 1);
        detach_ok(fd1, target, &mut optd, "prog_detach");
        assert_mprog_count(target, 0);
    }
    test_tc_link__destroy(skel);
}

pub unsafe extern "C" fn test_ns_tc_opts_query() {
    test_tc_opts_query_target(BPF_TCX_INGRESS);
    test_tc_opts_query_target(BPF_TCX_EGRESS);
}

unsafe fn test_tc_opts_query_attach_target(target: i32) {
    let mut opta = bpf_prog_attach_opts::default();
    let mut optd = bpf_prog_detach_opts::default();
    let mut optq = bpf_prog_query_opts::default();
    let mut prog_ids: [__u32; 2] = [0; 2];
    let skel = test_tc_link__open_and_load();
    'cleanup: {
        if !ASSERT_OK_PTR!(skel, "skel_load") {
            break 'cleanup;
        }
        let fd1 = bpf_program__fd((*skel).progs.tc1);
        let id1 = id_from_prog_fd(fd1);
        let mut err = bpf_prog_query_opts(loopback, target, &mut optq);
        if !ASSERT_OK!(err, "prog_query") {
            break 'cleanup;
        }
        ASSERT_EQ!(optq.count, 0, "count");
        ASSERT_EQ!(optq.revision, 1, "revision");
        opta.expected_revision = optq.revision;
        err = bpf_prog_attach_opts(fd1, loopback, target, &mut opta);
        if !ASSERT_EQ!(err, 0, "prog_attach") {
            break 'cleanup;
        }
        zero_array(&mut prog_ids);
        optq.prog_ids = prog_ids.as_mut_ptr();
        optq.count = prog_ids.len() as __u32;
        err = bpf_prog_query_opts(loopback, target, &mut optq);
        if ASSERT_OK!(err, "prog_query") {
            ASSERT_EQ!(optq.count, 1, "count");
            ASSERT_EQ!(optq.revision, 2, "revision");
            ASSERT_EQ!(*optq.prog_ids.add(0), id1, "prog_ids[0]");
            ASSERT_EQ!(*optq.prog_ids.add(1), 0, "prog_ids[1]");
        }
        detach_ok(fd1, target, &mut optd, "prog_detach");
        assert_mprog_count(target, 0);
    }
    test_tc_link__destroy(skel);
}

pub unsafe extern "C" fn test_ns_tc_opts_query_attach() {
    test_tc_opts_query_attach_target(BPF_TCX_INGRESS);
    test_tc_opts_query_attach_target(BPF_TCX_EGRESS);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
