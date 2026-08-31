// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates.*/

/* Translated from C. Dependencies from test_progs.h, network_helpers.h, and
 * test_ldsx_insn.skel.h are declared here and supplied externally.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type socklen_t = c_uint;

#[repr(C)]
struct test_ldsx_insn {
    rodata: *mut test_ldsx_insn_rodata,
    bss: *mut test_ldsx_insn_bss,
    progs: test_ldsx_insn_progs,
    links: test_ldsx_insn_links,
}

#[repr(C)]
struct test_ldsx_insn_rodata {
    skip: bool,
}

#[repr(C)]
struct test_ldsx_insn_bss {
    done1: c_int,
    ret1: c_int,
    done2: c_int,
    ret2: c_int,
    int_member: c_int,
    set_optlen: c_int,
    set_retval: c_int,
    set_mark: c_int,
}

#[repr(C)]
struct test_ldsx_insn_progs {
    rdonly_map_prog: *mut bpf_program,
    map_val_prog: *mut bpf_program,
    test_ptr_struct_arg: *mut bpf_program,
    _getsockopt: *mut bpf_program,
    _tc: *mut bpf_program,
}

#[repr(C)]
struct test_ldsx_insn_links {
    _getsockopt: *mut bpf_link,
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
#[derive(Copy, Clone)]
struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_test_run_opts {
    data_in: *const c_void,
    data_size_in: c_uint,
    ctx_in: *mut c_void,
    ctx_size_in: c_uint,
}

unsafe extern "C" {
    static pkt_v4: c_void;

    static AF_INET: c_int;
    static SOCK_STREAM: c_int;
    static SOL_IP: c_int;
    static IP_TTL: c_int;

    fn test_ldsx_insn__open() -> *mut test_ldsx_insn;
    fn test_ldsx_insn__load(skel: *mut test_ldsx_insn) -> c_int;
    fn test_ldsx_insn__attach(skel: *mut test_ldsx_insn) -> c_int;
    fn test_ldsx_insn__destroy(skel: *mut test_ldsx_insn);

    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_program__attach_cgroup(prog: *mut bpf_program, cgroup_fd: c_int) -> *mut bpf_link;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn trigger_module_test_read(sz: c_int) -> c_int;
    fn test__skip();
    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;

    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn getsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *mut c_void,
        option_len: *mut socklen_t,
    ) -> c_int;
    fn close(fd: c_int) -> c_int;
}

unsafe fn test_map_val_and_probed_memory() {
    let skel: *mut test_ldsx_insn;
    let mut err: c_int;

    skel = test_ldsx_insn__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"test_ldsx_insn__open".as_ptr()) {
        return;
    }

    if (*(*skel).rodata).skip {
        test__skip();
        test_ldsx_insn__destroy(skel);
        return;
    }

    bpf_program__set_autoload((*skel).progs.rdonly_map_prog, true);
    bpf_program__set_autoload((*skel).progs.map_val_prog, true);
    bpf_program__set_autoload((*skel).progs.test_ptr_struct_arg, true);

    err = test_ldsx_insn__load(skel);
    if !ASSERT_OK(err, c"test_ldsx_insn__load".as_ptr()) {
        test_ldsx_insn__destroy(skel);
        return;
    }

    err = test_ldsx_insn__attach(skel);
    if !ASSERT_OK(err, c"test_ldsx_insn__attach".as_ptr()) {
        test_ldsx_insn__destroy(skel);
        return;
    }

    ASSERT_OK(trigger_module_test_read(256), c"trigger_read".as_ptr());

    ASSERT_EQ((*(*skel).bss).done1, 1, c"done1".as_ptr());
    ASSERT_EQ((*(*skel).bss).ret1, 1, c"ret1".as_ptr());
    ASSERT_EQ((*(*skel).bss).done2, 1, c"done2".as_ptr());
    ASSERT_EQ((*(*skel).bss).ret2, 1, c"ret2".as_ptr());
    ASSERT_EQ((*(*skel).bss).int_member, -1, c"int_member".as_ptr());

    test_ldsx_insn__destroy(skel);
}

unsafe fn test_ctx_member_sign_ext() {
    let skel: *mut test_ldsx_insn;
    let mut err: c_int;
    let fd: c_int;
    let cgroup_fd: c_int;
    let mut buf: [c_char; 16] = [0; 16];
    let mut optlen: socklen_t;

    cgroup_fd = test__join_cgroup(c"/ldsx_test".as_ptr());
    if !ASSERT_GE(cgroup_fd, 0, c"join_cgroup /ldsx_test".as_ptr()) {
        return;
    }

    skel = test_ldsx_insn__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"test_ldsx_insn__open".as_ptr()) {
        close(cgroup_fd);
        return;
    }

    if (*(*skel).rodata).skip {
        test__skip();
        test_ldsx_insn__destroy(skel);
        close(cgroup_fd);
        return;
    }

    bpf_program__set_autoload((*skel).progs._getsockopt, true);

    err = test_ldsx_insn__load(skel);
    if !ASSERT_OK(err, c"test_ldsx_insn__load".as_ptr()) {
        test_ldsx_insn__destroy(skel);
        close(cgroup_fd);
        return;
    }

    (*skel).links._getsockopt =
        bpf_program__attach_cgroup((*skel).progs._getsockopt, cgroup_fd);
    if !ASSERT_OK_PTR((*skel).links._getsockopt as *const c_void, c"getsockopt_link".as_ptr()) {
        test_ldsx_insn__destroy(skel);
        close(cgroup_fd);
        return;
    }

    fd = socket(AF_INET, SOCK_STREAM, 0);
    if !ASSERT_GE(fd, 0, c"socket".as_ptr()) {
        test_ldsx_insn__destroy(skel);
        close(cgroup_fd);
        return;
    }

    optlen = core::mem::size_of_val(&buf) as socklen_t;
    let _ = getsockopt(
        fd,
        SOL_IP,
        IP_TTL,
        buf.as_mut_ptr() as *mut c_void,
        &mut optlen,
    );

    ASSERT_EQ((*(*skel).bss).set_optlen, -1, c"optlen".as_ptr());
    ASSERT_EQ((*(*skel).bss).set_retval, -1, c"retval".as_ptr());

    close(fd);
    test_ldsx_insn__destroy(skel);
    close(cgroup_fd);
}

unsafe fn test_ctx_member_narrow_sign_ext() {
    let skel: *mut test_ldsx_insn;
    let mut skb: __sk_buff = core::mem::zeroed();
    let mut topts = bpf_test_run_opts {
        data_in: &pkt_v4 as *const c_void,
        data_size_in: core::mem::size_of_val(&pkt_v4) as c_uint,
        ctx_in: &mut skb as *mut __sk_buff as *mut c_void,
        ctx_size_in: core::mem::size_of_val(&skb) as c_uint,
    };
    let mut err: c_int;
    let prog_fd: c_int;

    skel = test_ldsx_insn__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"test_ldsx_insn__open".as_ptr()) {
        return;
    }

    if (*(*skel).rodata).skip {
        test__skip();
        test_ldsx_insn__destroy(skel);
        return;
    }

    bpf_program__set_autoload((*skel).progs._tc, true);

    err = test_ldsx_insn__load(skel);
    if !ASSERT_OK(err, c"test_ldsx_insn__load".as_ptr()) {
        test_ldsx_insn__destroy(skel);
        return;
    }

    prog_fd = bpf_program__fd((*skel).progs._tc);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, c"test_run".as_ptr());

    ASSERT_EQ((*(*skel).bss).set_mark, -2, c"set_mark".as_ptr());

    test_ldsx_insn__destroy(skel);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_ldsx_insn() {
    if test__start_subtest(c"map_val and probed_memory".as_ptr()) {
        test_map_val_and_probed_memory();
    }
    if test__start_subtest(c"ctx_member_sign_ext".as_ptr()) {
        test_ctx_member_sign_ext();
    }
    if test__start_subtest(c"ctx_member_narrow_sign_ext".as_ptr()) {
        test_ctx_member_narrow_sign_ext();
    }
}
