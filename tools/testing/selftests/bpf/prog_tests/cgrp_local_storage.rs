// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates.*/

/* Translated from C. Original dependencies:
 * unistd.h, sys/syscall.h, sys/types.h, test_progs.h,
 * cgrp_ls_tp_btf.skel.h, cgrp_ls_recursion.skel.h,
 * cgrp_ls_attach_cgroup.skel.h, cgrp_ls_negative.skel.h,
 * cgrp_ls_sleepable.skel.h, network_helpers.h, cgroup_helpers.h
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type __u32 = u32;
type __u64 = u64;
type socklen_t = u32;

const BPF_ANY: __u64 = 0;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const SYS_getpgid: c_long = 121;
const BPF_CGROUP_ITER_SELF_ONLY: c_uint = 0;

#[repr(C)]
struct socket_cookie {
    cookie_key: __u64,
    cookie_value: __u64,
}

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct in6_addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
struct sockaddr_in6 {
    sin6_family: u16,
    sin6_port: u16,
    sin6_flowinfo: __u32,
    sin6_addr: in6_addr,
    sin6_scope_id: __u32,
}

#[repr(C)]
struct bpf_map;
#[repr(C)]
struct bpf_program;
#[repr(C)]
struct bpf_link;

#[repr(C)]
struct cgrp_ls_tp_btf_bss {
    is_cgroup1: bool,
    target_hid: c_int,
    target_pid: c_long,
    enter_cnt: c_long,
    exit_cnt: c_long,
    mismatch_cnt: c_long,
}

#[repr(C)]
struct cgrp_ls_tp_btf_maps {
    map_b: *mut bpf_map,
}

#[repr(C)]
struct cgrp_ls_tp_btf {
    bss: *mut cgrp_ls_tp_btf_bss,
    maps: cgrp_ls_tp_btf_maps,
}

#[repr(C)]
struct cgrp_ls_recursion_bss {
    is_cgroup1: bool,
    target_hid: c_int,
}

#[repr(C)]
struct cgrp_ls_recursion {
    bss: *mut cgrp_ls_recursion_bss,
}

#[repr(C)]
struct cgrp_ls_attach_cgroup_maps {
    socket_cookies: *mut bpf_map,
}

#[repr(C)]
struct cgrp_ls_attach_cgroup_progs {
    set_cookie: *mut bpf_program,
    update_cookie_sockops: *mut bpf_program,
    update_cookie_tracing: *mut bpf_program,
}

#[repr(C)]
struct cgrp_ls_attach_cgroup_links {
    set_cookie: *mut bpf_link,
    update_cookie_sockops: *mut bpf_link,
    update_cookie_tracing: *mut bpf_link,
}

#[repr(C)]
struct cgrp_ls_attach_cgroup {
    maps: cgrp_ls_attach_cgroup_maps,
    progs: cgrp_ls_attach_cgroup_progs,
    links: cgrp_ls_attach_cgroup_links,
}

#[repr(C)]
struct cgrp_ls_negative;

#[repr(C)]
struct cgrp_ls_sleepable_bss {
    is_cgroup1: bool,
    target_hid: c_int,
    target_pid: c_long,
    update_err: c_int,
    cgroup_id: __u64,
}

#[repr(C)]
struct cgrp_ls_sleepable_progs {
    cgroup_iter: *mut bpf_program,
    fexit_update: *mut bpf_program,
    yes_rcu_lock: *mut bpf_program,
    no_rcu_lock: *mut bpf_program,
    cgrp1_no_rcu_lock: *mut bpf_program,
}

#[repr(C)]
struct cgrp_ls_sleepable {
    bss: *mut cgrp_ls_sleepable_bss,
    progs: cgrp_ls_sleepable_progs,
}

#[repr(C)]
struct bpf_iter_cgroup_info {
    cgroup_fd: c_int,
    order: c_uint,
}

#[repr(C)]
union bpf_iter_link_info {
    cgroup: bpf_iter_cgroup_info,
}

#[repr(C)]
struct bpf_iter_attach_opts {
    sz: usize,
    link_info: *mut bpf_iter_link_info,
    link_info_len: c_uint,
}

static mut is_cgroup1: bool = false;
static mut target_hid: c_int = 0;

unsafe extern "C" {
    fn cgrp_ls_tp_btf__open_and_load() -> *mut cgrp_ls_tp_btf;
    fn cgrp_ls_tp_btf__attach(skel: *mut cgrp_ls_tp_btf) -> c_int;
    fn cgrp_ls_tp_btf__destroy(skel: *mut cgrp_ls_tp_btf);

    fn cgrp_ls_recursion__open_and_load() -> *mut cgrp_ls_recursion;
    fn cgrp_ls_recursion__attach(skel: *mut cgrp_ls_recursion) -> c_int;
    fn cgrp_ls_recursion__destroy(skel: *mut cgrp_ls_recursion);

    fn cgrp_ls_attach_cgroup__open_and_load() -> *mut cgrp_ls_attach_cgroup;
    fn cgrp_ls_attach_cgroup__destroy(skel: *mut cgrp_ls_attach_cgroup);

    fn cgrp_ls_negative__open_and_load() -> *mut cgrp_ls_negative;
    fn cgrp_ls_negative__destroy(skel: *mut cgrp_ls_negative);

    fn cgrp_ls_sleepable__open() -> *mut cgrp_ls_sleepable;
    fn cgrp_ls_sleepable__load(skel: *mut cgrp_ls_sleepable) -> c_int;
    fn cgrp_ls_sleepable__attach(skel: *mut cgrp_ls_sleepable) -> c_int;
    fn cgrp_ls_sleepable__destroy(skel: *mut cgrp_ls_sleepable);

    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: __u64) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;
    fn bpf_program__attach_cgroup(prog: *mut bpf_program, cgroup_fd: c_int) -> *mut bpf_link;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_program__attach_iter(prog: *mut bpf_program, opts: *mut bpf_iter_attach_opts) -> *mut bpf_link;
    fn bpf_link__fd(link: *mut bpf_link) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_iter_create(link_fd: c_int) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_ERR_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_long, expected: c_long, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: __u64, expected: __u64, name: *const c_char) -> bool;

    fn sys_gettid() -> c_long;
    fn syscall(num: c_long, ...) -> c_long;
    fn start_server(family: c_int, socktype: c_int, addr: *const c_char, port: c_int, timeout_ms: c_int) -> c_int;
    fn connect_to_fd(server_fd: c_int, timeout_ms: c_int) -> c_int;
    fn getsockname(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn ntohs(netshort: u16) -> u16;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;

    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn get_cgroup_id(path: *const c_char) -> __u64;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn setup_classid_environment() -> c_int;
    fn join_classid() -> c_int;
    fn open_classid() -> c_int;
    fn get_classid_cgroup_id() -> c_int;
    fn get_cgroup1_hierarchy_id(name: *const c_char) -> c_int;
    fn cleanup_classid_environment();
}

unsafe fn CGROUP_MODE_SET_TP_BTF(skel: *mut cgrp_ls_tp_btf) {
    (*(*skel).bss).is_cgroup1 = is_cgroup1;
    (*(*skel).bss).target_hid = target_hid;
}

unsafe fn CGROUP_MODE_SET_RECURSION(skel: *mut cgrp_ls_recursion) {
    (*(*skel).bss).is_cgroup1 = is_cgroup1;
    (*(*skel).bss).target_hid = target_hid;
}

unsafe fn CGROUP_MODE_SET_SLEEPABLE(skel: *mut cgrp_ls_sleepable) {
    (*(*skel).bss).is_cgroup1 = is_cgroup1;
    (*(*skel).bss).target_hid = target_hid;
}

unsafe fn cgroup_mode_value_init(cgroup: bool, hid: c_int) {
    is_cgroup1 = cgroup;
    target_hid = hid;
}

unsafe fn test_tp_btf(cgroup_fd: c_int) {
    let skel: *mut cgrp_ls_tp_btf;
    let val1: c_long = 1;
    let mut val2: c_long = 0;
    let mut err: c_int;

    skel = cgrp_ls_tp_btf__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open_and_load".as_ptr()) {
        return;
    }

    CGROUP_MODE_SET_TP_BTF(skel);

    /* populate a value in map_b */
    err = bpf_map_update_elem(
        bpf_map__fd((*skel).maps.map_b),
        &cgroup_fd as *const _ as *const c_void,
        &val1 as *const _ as *const c_void,
        BPF_ANY,
    );
    if !ASSERT_OK(err, c"map_update_elem".as_ptr()) {
        cgrp_ls_tp_btf__destroy(skel);
        return;
    }

    /* check value */
    err = bpf_map_lookup_elem(
        bpf_map__fd((*skel).maps.map_b),
        &cgroup_fd as *const _ as *const c_void,
        &mut val2 as *mut _ as *mut c_void,
    );
    if !ASSERT_OK(err, c"map_lookup_elem".as_ptr()) {
        cgrp_ls_tp_btf__destroy(skel);
        return;
    }
    if !ASSERT_EQ(val2 as __u64, 1, c"map_lookup_elem, invalid val".as_ptr()) {
        cgrp_ls_tp_btf__destroy(skel);
        return;
    }

    /* delete value */
    err = bpf_map_delete_elem(
        bpf_map__fd((*skel).maps.map_b),
        &cgroup_fd as *const _ as *const c_void,
    );
    if !ASSERT_OK(err, c"map_delete_elem".as_ptr()) {
        cgrp_ls_tp_btf__destroy(skel);
        return;
    }

    (*(*skel).bss).target_pid = sys_gettid();

    err = cgrp_ls_tp_btf__attach(skel);
    if !ASSERT_OK(err, c"skel_attach".as_ptr()) {
        cgrp_ls_tp_btf__destroy(skel);
        return;
    }

    sys_gettid();
    sys_gettid();

    (*(*skel).bss).target_pid = 0;

    /* 3x syscalls: 1x attach and 2x gettid */
    ASSERT_EQ((*(*skel).bss).enter_cnt as __u64, 3, c"enter_cnt".as_ptr());
    ASSERT_EQ((*(*skel).bss).exit_cnt as __u64, 3, c"exit_cnt".as_ptr());
    ASSERT_EQ((*(*skel).bss).mismatch_cnt as __u64, 0, c"mismatch_cnt".as_ptr());
    cgrp_ls_tp_btf__destroy(skel);
}

unsafe fn test_attach_cgroup(cgroup_fd: c_int) {
    let mut server_fd: c_int = 0;
    let mut client_fd: c_int = 0;
    let mut err: c_int;
    let mut addr_len: socklen_t = size_of::<sockaddr_in6>() as socklen_t;
    let skel: *mut cgrp_ls_attach_cgroup;
    let cookie_expected_value: __u32;
    let mut addr: sockaddr_in6 = zeroed();
    let mut val: socket_cookie = zeroed();

    skel = cgrp_ls_attach_cgroup__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
        return;
    }

    (*skel).links.set_cookie = bpf_program__attach_cgroup((*skel).progs.set_cookie, cgroup_fd);
    if !ASSERT_OK_PTR((*skel).links.set_cookie as *const c_void, c"prog_attach".as_ptr()) {
        cgrp_ls_attach_cgroup__destroy(skel);
        return;
    }

    (*skel).links.update_cookie_sockops =
        bpf_program__attach_cgroup((*skel).progs.update_cookie_sockops, cgroup_fd);
    if !ASSERT_OK_PTR((*skel).links.update_cookie_sockops as *const c_void, c"prog_attach".as_ptr()) {
        cgrp_ls_attach_cgroup__destroy(skel);
        return;
    }

    (*skel).links.update_cookie_tracing = bpf_program__attach((*skel).progs.update_cookie_tracing);
    if !ASSERT_OK_PTR((*skel).links.update_cookie_tracing as *const c_void, c"prog_attach".as_ptr()) {
        cgrp_ls_attach_cgroup__destroy(skel);
        return;
    }

    server_fd = start_server(AF_INET6, SOCK_STREAM, c"::1".as_ptr(), 0, 0);
    if !ASSERT_GE(server_fd as c_long, 0, c"start_server".as_ptr()) {
        cgrp_ls_attach_cgroup__destroy(skel);
        return;
    }

    client_fd = connect_to_fd(server_fd, 0);
    if !ASSERT_GE(client_fd as c_long, 0, c"connect_to_fd".as_ptr()) {
        close(server_fd);
        cgrp_ls_attach_cgroup__destroy(skel);
        return;
    }

    err = bpf_map_lookup_elem(
        bpf_map__fd((*skel).maps.socket_cookies),
        &cgroup_fd as *const _ as *const c_void,
        &mut val as *mut _ as *mut c_void,
    );
    if !ASSERT_OK(err, c"map_lookup(socket_cookies)".as_ptr()) {
        close(client_fd);
        close(server_fd);
        cgrp_ls_attach_cgroup__destroy(skel);
        return;
    }

    err = getsockname(
        client_fd,
        &mut addr as *mut _ as *mut sockaddr,
        &mut addr_len as *mut socklen_t,
    );
    if !ASSERT_OK(err, c"getsockname".as_ptr()) {
        close(client_fd);
        close(server_fd);
        cgrp_ls_attach_cgroup__destroy(skel);
        return;
    }

    cookie_expected_value = ((ntohs(addr.sin6_port) as __u32) << 8) | 0xFF;
    ASSERT_EQ(val.cookie_value, cookie_expected_value as __u64, c"cookie_value".as_ptr());

    close(client_fd);
    close(server_fd);
    cgrp_ls_attach_cgroup__destroy(skel);
}

unsafe fn test_recursion(cgroup_fd: c_int) {
    let skel: *mut cgrp_ls_recursion;
    let err: c_int;

    skel = cgrp_ls_recursion__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open_and_load".as_ptr()) {
        return;
    }

    CGROUP_MODE_SET_RECURSION(skel);

    err = cgrp_ls_recursion__attach(skel);
    if !ASSERT_OK(err, c"skel_attach".as_ptr()) {
        cgrp_ls_recursion__destroy(skel);
        return;
    }

    /* trigger sys_enter, make sure it does not cause deadlock */
    sys_gettid();

    cgrp_ls_recursion__destroy(skel);
}

unsafe fn test_negative() {
    let skel: *mut cgrp_ls_negative;

    skel = cgrp_ls_negative__open_and_load();
    if !ASSERT_ERR_PTR(skel as *const c_void, c"skel_open_and_load".as_ptr()) {
        cgrp_ls_negative__destroy(skel);
        return;
    }
}

unsafe fn test_cgroup_iter_sleepable(cgroup_fd: c_int, cgroup_id: __u64) {
    let mut opts = bpf_iter_attach_opts {
        sz: size_of::<bpf_iter_attach_opts>(),
        link_info: ptr::null_mut(),
        link_info_len: 0,
    };
    let mut linfo: bpf_iter_link_info = zeroed();
    let skel: *mut cgrp_ls_sleepable;
    let mut link: *mut bpf_link;
    let fexit_link: *mut bpf_link;
    let mut err: c_int;
    let iter_fd: c_int;
    let mut buf: [c_char; 16] = [0; 16];

    skel = cgrp_ls_sleepable__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
        return;
    }

    CGROUP_MODE_SET_SLEEPABLE(skel);

    bpf_program__set_autoload((*skel).progs.cgroup_iter, true);
    err = cgrp_ls_sleepable__load(skel);
    if !ASSERT_OK(err, c"skel_load".as_ptr()) {
        cgrp_ls_sleepable__destroy(skel);
        return;
    }

    memset(
        &mut linfo as *mut _ as *mut c_void,
        0,
        size_of::<bpf_iter_link_info>(),
    );
    linfo.cgroup.cgroup_fd = cgroup_fd;
    linfo.cgroup.order = BPF_CGROUP_ITER_SELF_ONLY;
    opts.link_info = &mut linfo;
    opts.link_info_len = size_of::<bpf_iter_link_info>() as c_uint;
    link = bpf_program__attach_iter((*skel).progs.cgroup_iter, &mut opts);
    if !ASSERT_OK_PTR(link as *const c_void, c"attach_iter".as_ptr()) {
        cgrp_ls_sleepable__destroy(skel);
        return;
    }

    fexit_link = bpf_program__attach((*skel).progs.fexit_update);
    if !ASSERT_OK_PTR(fexit_link as *const c_void, c"attach_fexit".as_ptr()) {
        bpf_link__destroy(link);
        cgrp_ls_sleepable__destroy(skel);
        return;
    }

    iter_fd = bpf_iter_create(bpf_link__fd(link));
    if !ASSERT_GE(iter_fd as c_long, 0, c"iter_create".as_ptr()) {
        bpf_link__destroy(fexit_link);
        bpf_link__destroy(link);
        cgrp_ls_sleepable__destroy(skel);
        return;
    }

    (*(*skel).bss).target_pid = sys_gettid();

    /* trigger the program run */
    read(iter_fd, buf.as_mut_ptr() as *mut c_void, size_of::<[c_char; 16]>());

    (*(*skel).bss).target_pid = 0;

    ASSERT_EQ((*(*skel).bss).update_err as __u64, 0, c"update_err".as_ptr());
    ASSERT_EQ((*(*skel).bss).cgroup_id, cgroup_id, c"cgroup_id".as_ptr());

    close(iter_fd);
    bpf_link__destroy(fexit_link);
    bpf_link__destroy(link);
    cgrp_ls_sleepable__destroy(skel);
}

unsafe fn test_yes_rcu_lock(cgroup_id: __u64) {
    let skel: *mut cgrp_ls_sleepable;
    let mut err: c_int;

    skel = cgrp_ls_sleepable__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
        return;
    }

    CGROUP_MODE_SET_SLEEPABLE(skel);
    (*(*skel).bss).target_pid = sys_gettid();

    bpf_program__set_autoload((*skel).progs.yes_rcu_lock, true);
    err = cgrp_ls_sleepable__load(skel);
    if !ASSERT_OK(err, c"skel_load".as_ptr()) {
        cgrp_ls_sleepable__destroy(skel);
        return;
    }

    err = cgrp_ls_sleepable__attach(skel);
    if !ASSERT_OK(err, c"skel_attach".as_ptr()) {
        cgrp_ls_sleepable__destroy(skel);
        return;
    }

    syscall(SYS_getpgid);

    ASSERT_EQ((*(*skel).bss).cgroup_id, cgroup_id, c"cgroup_id".as_ptr());
    cgrp_ls_sleepable__destroy(skel);
}

unsafe fn test_no_rcu_lock() {
    let skel: *mut cgrp_ls_sleepable;
    let err: c_int;

    skel = cgrp_ls_sleepable__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
        return;
    }

    CGROUP_MODE_SET_SLEEPABLE(skel);

    bpf_program__set_autoload((*skel).progs.no_rcu_lock, true);
    err = cgrp_ls_sleepable__load(skel);
    ASSERT_ERR(err, c"skel_load".as_ptr());

    cgrp_ls_sleepable__destroy(skel);
}

unsafe fn test_cgrp1_no_rcu_lock() {
    let skel: *mut cgrp_ls_sleepable;
    let err: c_int;

    skel = cgrp_ls_sleepable__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
        return;
    }

    CGROUP_MODE_SET_SLEEPABLE(skel);

    bpf_program__set_autoload((*skel).progs.cgrp1_no_rcu_lock, true);
    err = cgrp_ls_sleepable__load(skel);
    ASSERT_OK(err, c"skel_load".as_ptr());

    cgrp_ls_sleepable__destroy(skel);
}

unsafe fn cgrp2_local_storage() {
    let cgroup_id: __u64;
    let cgroup_fd: c_int;

    cgroup_fd = test__join_cgroup(c"/cgrp_local_storage".as_ptr());
    if !ASSERT_GE(cgroup_fd as c_long, 0, c"join_cgroup /cgrp_local_storage".as_ptr()) {
        return;
    }

    cgroup_mode_value_init(false, -1);

    cgroup_id = get_cgroup_id(c"/cgrp_local_storage".as_ptr());
    if test__start_subtest(c"tp_btf".as_ptr()) {
        test_tp_btf(cgroup_fd);
    }
    if test__start_subtest(c"attach_cgroup".as_ptr()) {
        test_attach_cgroup(cgroup_fd);
    }
    if test__start_subtest(c"recursion".as_ptr()) {
        test_recursion(cgroup_fd);
    }
    if test__start_subtest(c"negative".as_ptr()) {
        test_negative();
    }
    if test__start_subtest(c"cgroup_iter_sleepable".as_ptr()) {
        test_cgroup_iter_sleepable(cgroup_fd, cgroup_id);
    }
    if test__start_subtest(c"yes_rcu_lock".as_ptr()) {
        test_yes_rcu_lock(cgroup_id);
    }
    if test__start_subtest(c"no_rcu_lock".as_ptr()) {
        test_no_rcu_lock();
    }

    close(cgroup_fd);
}

unsafe fn cgrp1_local_storage() {
    let cgrp1_fd: c_int;
    let cgrp1_hid: c_int;
    let cgrp1_id: c_int;
    let mut err: c_int;

    /* Setup cgroup1 hierarchy */
    err = setup_classid_environment();
    if !ASSERT_OK(err, c"setup_classid_environment".as_ptr()) {
        return;
    }

    err = join_classid();
    if !ASSERT_OK(err, c"join_cgroup1".as_ptr()) {
        cleanup_classid_environment();
        return;
    }

    cgrp1_fd = open_classid();
    if !ASSERT_GE(cgrp1_fd as c_long, 0, c"cgroup1 fd".as_ptr()) {
        cleanup_classid_environment();
        return;
    }

    cgrp1_id = get_classid_cgroup_id();
    if !ASSERT_GE(cgrp1_id as c_long, 0, c"cgroup1 id".as_ptr()) {
        close(cgrp1_fd);
        cleanup_classid_environment();
        return;
    }

    cgrp1_hid = get_cgroup1_hierarchy_id(c"net_cls".as_ptr());
    if !ASSERT_GE(cgrp1_hid as c_long, 0, c"cgroup1 hid".as_ptr()) {
        close(cgrp1_fd);
        cleanup_classid_environment();
        return;
    }

    cgroup_mode_value_init(true, cgrp1_hid);

    if test__start_subtest(c"cgrp1_tp_btf".as_ptr()) {
        test_tp_btf(cgrp1_fd);
    }
    if test__start_subtest(c"cgrp1_recursion".as_ptr()) {
        test_recursion(cgrp1_fd);
    }
    if test__start_subtest(c"cgrp1_negative".as_ptr()) {
        test_negative();
    }
    if test__start_subtest(c"cgrp1_iter_sleepable".as_ptr()) {
        test_cgroup_iter_sleepable(cgrp1_fd, cgrp1_id as __u64);
    }
    if test__start_subtest(c"cgrp1_yes_rcu_lock".as_ptr()) {
        test_yes_rcu_lock(cgrp1_id as __u64);
    }
    if test__start_subtest(c"cgrp1_no_rcu_lock".as_ptr()) {
        test_cgrp1_no_rcu_lock();
    }

    close(cgrp1_fd);
    cleanup_classid_environment();
}

#[no_mangle]
pub unsafe extern "C" fn test_cgrp_local_storage() {
    cgrp2_local_storage();
    cgrp1_local_storage();
}
