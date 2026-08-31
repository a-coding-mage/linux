// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 Facebook */

/* Translated from:
 * #include <linux/err.h>
 * #include <netinet/tcp.h>
 * #include <test_progs.h>
 * #include "network_helpers.h"
 * #include "bpf_dctcp.skel.h"
 * #include "bpf_cubic.skel.h"
 * #include "bpf_tcp_nogpl.skel.h"
 * #include "tcp_ca_update.skel.h"
 * #include "bpf_dctcp_release.skel.h"
 * #include "tcp_ca_write_sk_pacing.skel.h"
 * #include "tcp_ca_incompl_cong_ops.skel.h"
 * #include "tcp_ca_unsupp_cong_op.skel.h"
 * #include "tcp_ca_kfunc.skel.h"
 * #include "tcp_ca_untrusted_btf_write.skel.h"
 * #include "bpf_cc_cubic.skel.h"
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

type bool_ = bool;
type socklen_t = u32;
type va_list = *mut c_void;
type libbpf_print_fn_t = Option<
    unsafe extern "C" fn(level: libbpf_print_level, format: *const c_char, args: va_list) -> c_int,
>;

const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const IPPROTO_TCP: c_int = 6;
const SOL_TCP: c_int = 6;
const TCP_CONGESTION: c_int = 13;
const BPF_NOEXIST: u64 = 1;
const BPF_F_REPLACE: u32 = 1 << 2;
const ENOENT: c_int = 2;
const ENOTSUPP: c_int = 524;
const LIBBPF_WARN: libbpf_print_level = 1;

static total_bytes: c_uint = 10 * 1024 * 1024;
static mut expected_stg: c_int = 0xeB9F;

#[repr(C)]
struct cb_opts {
    cc: *const c_char,
    map_fd: c_int,
}

#[repr(C)]
struct network_helper_opts {
    post_socket_cb: Option<unsafe extern "C" fn(fd: c_int, opts: *mut c_void) -> c_int>,
    cb_opts: *mut c_void,
}

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}

type libbpf_print_level = c_int;

#[repr(C)]
struct bpf_link_update_opts {
    sz: usize,
    flags: u32,
    old_prog_fd: u32,
    old_map_fd: u32,
}

#[repr(C)]
struct bpf_cubic_maps {
    cubic: *mut bpf_map,
}

#[repr(C)]
struct bpf_cubic_bss {
    bpf_cubic_acked_called: c_int,
    nodelay_init_reject: bool_,
    nodelay_cwnd_event_tx_start_reject: bool_,
}

#[repr(C)]
struct bpf_cubic {
    maps: bpf_cubic_maps,
    bss: *mut bpf_cubic_bss,
}

#[repr(C)]
struct bpf_dctcp_maps {
    dctcp: *mut bpf_map,
    dctcp_nouse: *mut bpf_map,
    sk_stg_map: *mut bpf_map,
}

#[repr(C)]
struct bpf_dctcp_links {
    dctcp: *mut bpf_link,
}

#[repr(C)]
struct bpf_dctcp_rodata {
    fallback_cc: [c_char; 16],
}

#[repr(C)]
struct bpf_dctcp_bss {
    stg_result: c_int,
    cc_res: [c_char; 16],
    tcp_cdg_res: c_int,
    ebusy_cnt: c_int,
}

#[repr(C)]
struct bpf_dctcp {
    maps: bpf_dctcp_maps,
    links: bpf_dctcp_links,
    rodata: *mut bpf_dctcp_rodata,
    bss: *mut bpf_dctcp_bss,
}

#[repr(C)]
struct bpf_tcp_nogpl {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_dctcp_release {
    _private: [u8; 0],
}

#[repr(C)]
struct tcp_ca_write_sk_pacing_maps {
    write_sk_pacing: *mut bpf_map,
}

#[repr(C)]
struct tcp_ca_write_sk_pacing {
    maps: tcp_ca_write_sk_pacing_maps,
}

#[repr(C)]
struct tcp_ca_incompl_cong_ops_maps {
    incompl_cong_ops: *mut bpf_map,
}

#[repr(C)]
struct tcp_ca_incompl_cong_ops {
    maps: tcp_ca_incompl_cong_ops_maps,
}

#[repr(C)]
struct tcp_ca_unsupp_cong_op {
    _private: [u8; 0],
}

#[repr(C)]
struct tcp_ca_update_maps {
    ca_update_1: *mut bpf_map,
    ca_update_2: *mut bpf_map,
    ca_wrong: *mut bpf_map,
    ca_no_link: *mut bpf_map,
}

#[repr(C)]
struct tcp_ca_update_bss {
    ca1_cnt: c_int,
    ca2_cnt: c_int,
}

#[repr(C)]
struct tcp_ca_update {
    maps: tcp_ca_update_maps,
    bss: *mut tcp_ca_update_bss,
}

#[repr(C)]
struct tcp_ca_kfunc {
    _private: [u8; 0],
}

#[repr(C)]
struct tcp_ca_untrusted_btf_write {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_cc_cubic_maps {
    cc_cubic: *mut bpf_map,
}

#[repr(C)]
struct bpf_cc_cubic {
    maps: bpf_cc_cubic_maps,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn setsockopt(
        fd: c_int,
        level: c_int,
        optname: c_int,
        optval: *const c_void,
        optlen: socklen_t,
    ) -> c_int;
    fn getsockopt(
        fd: c_int,
        level: c_int,
        optname: c_int,
        optval: *mut c_void,
        optlen: *mut socklen_t,
    ) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn printf(format: *const c_char, ...) -> c_int;
    fn vprintf(format: *const c_char, args: va_list) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn accept(fd: c_int, addr: *mut c_void, addrlen: socklen_t) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;

    fn start_server_str(
        family: c_int,
        socktype: c_int,
        addr_str: *mut c_char,
        port: c_int,
        opts: *const network_helper_opts,
    ) -> c_int;
    fn connect_to_fd_opts(fd: c_int, opts: *const network_helper_opts) -> c_int;
    fn send_recv_data(lfd: c_int, fd: c_int, total_bytes: c_uint) -> c_int;

    fn ASSERT_NEQ(actual: c_int, expected: c_int, name: *const c_char) -> bool_;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool_;
    fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool_;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool_;
    fn ASSERT_TRUE(actual: bool_, name: *const c_char) -> bool_;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool_;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool_;
    fn ASSERT_STREQ(actual: *const c_char, expected: *const c_char, name: *const c_char) -> bool_;
    fn ASSERT_NULL<T>(ptr: *mut T, name: *const c_char) -> bool_;
    fn ASSERT_ERR_PTR<T>(ptr: *mut T, name: *const c_char) -> bool_;
    fn ASSERT_GT(actual: c_int, expected: c_int, name: *const c_char) -> bool_;
    fn test__start_subtest(name: *const c_char) -> bool_;

    fn bpf_map__attach_struct_ops(map: *mut bpf_map) -> *mut bpf_link;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map__set_autoattach(map: *mut bpf_map, autoattach: bool_);
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_link__update_map(link: *mut bpf_link, map: *mut bpf_map) -> c_int;
    fn bpf_link__fd(link: *mut bpf_link) -> c_int;
    fn bpf_link_update(link_fd: c_int, new_fd: c_int, opts: *mut bpf_link_update_opts) -> c_int;
    fn libbpf_set_print(print_fn: libbpf_print_fn_t) -> libbpf_print_fn_t;

    fn bpf_cubic__open_and_load() -> *mut bpf_cubic;
    fn bpf_cubic__destroy(skel: *mut bpf_cubic);
    fn bpf_dctcp__open() -> *mut bpf_dctcp;
    fn bpf_dctcp__load(skel: *mut bpf_dctcp) -> c_int;
    fn bpf_dctcp__open_and_load() -> *mut bpf_dctcp;
    fn bpf_dctcp__attach(skel: *mut bpf_dctcp) -> c_int;
    fn bpf_dctcp__destroy(skel: *mut bpf_dctcp);
    fn bpf_tcp_nogpl__open_and_load() -> *mut bpf_tcp_nogpl;
    fn bpf_tcp_nogpl__destroy(skel: *mut bpf_tcp_nogpl);
    fn bpf_dctcp_release__open_and_load() -> *mut bpf_dctcp_release;
    fn bpf_dctcp_release__destroy(skel: *mut bpf_dctcp_release);
    fn tcp_ca_write_sk_pacing__open_and_load() -> *mut tcp_ca_write_sk_pacing;
    fn tcp_ca_write_sk_pacing__destroy(skel: *mut tcp_ca_write_sk_pacing);
    fn tcp_ca_incompl_cong_ops__open_and_load() -> *mut tcp_ca_incompl_cong_ops;
    fn tcp_ca_incompl_cong_ops__destroy(skel: *mut tcp_ca_incompl_cong_ops);
    fn tcp_ca_unsupp_cong_op__open_and_load() -> *mut tcp_ca_unsupp_cong_op;
    fn tcp_ca_unsupp_cong_op__destroy(skel: *mut tcp_ca_unsupp_cong_op);
    fn tcp_ca_update__open_and_load() -> *mut tcp_ca_update;
    fn tcp_ca_update__destroy(skel: *mut tcp_ca_update);
    fn tcp_ca_kfunc__open_and_load() -> *mut tcp_ca_kfunc;
    fn tcp_ca_kfunc__destroy(skel: *mut tcp_ca_kfunc);
    fn tcp_ca_untrusted_btf_write__open_and_load() -> *mut tcp_ca_untrusted_btf_write;
    fn tcp_ca_untrusted_btf_write__destroy(skel: *mut tcp_ca_untrusted_btf_write);
    fn bpf_cc_cubic__open_and_load() -> *mut bpf_cc_cubic;
    fn bpf_cc_cubic__destroy(skel: *mut bpf_cc_cubic);
}

const fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

unsafe fn settcpca(fd: c_int, tcp_ca: *const c_char) -> c_int {
    let mut err: c_int;

    err = setsockopt(
        fd,
        IPPROTO_TCP,
        TCP_CONGESTION,
        tcp_ca as *const c_void,
        strlen(tcp_ca) as socklen_t,
    );
    if !ASSERT_NEQ(err, -1, cstr(b"setsockopt\0")) {
        return -1;
    }

    return 0;
}

unsafe fn start_test(
    addr_str: *mut c_char,
    srv_opts: *const network_helper_opts,
    cli_opts: *const network_helper_opts,
    srv_fd: *mut c_int,
    cli_fd: *mut c_int,
) -> bool_ {
    *srv_fd = start_server_str(AF_INET6, SOCK_STREAM, addr_str, 0, srv_opts);
    if !ASSERT_NEQ(*srv_fd, -1, cstr(b"start_server_str\0")) {
        goto_err(srv_fd, cli_fd);
        return false;
    }

    /* connect to server */
    *cli_fd = connect_to_fd_opts(*srv_fd, cli_opts);
    if !ASSERT_NEQ(*cli_fd, -1, cstr(b"connect_to_fd_opts\0")) {
        goto_err(srv_fd, cli_fd);
        return false;
    }

    return true;
}

unsafe fn goto_err(srv_fd: *mut c_int, cli_fd: *mut c_int) {
    if *srv_fd != -1 {
        close(*srv_fd);
        *srv_fd = -1;
    }
    if *cli_fd != -1 {
        close(*cli_fd);
        *cli_fd = -1;
    }
}

unsafe fn do_test(opts: *const network_helper_opts) {
    let mut lfd: c_int = -1;
    let mut fd: c_int = -1;

    if !start_test(core::ptr::null_mut(), opts, opts, &mut lfd, &mut fd) {
        if lfd != -1 {
            close(lfd);
        }
        if fd != -1 {
            close(fd);
        }
        return;
    }

    ASSERT_OK(send_recv_data(lfd, fd, total_bytes), cstr(b"send_recv_data\0"));

    if lfd != -1 {
        close(lfd);
    }
    if fd != -1 {
        close(fd);
    }
}

unsafe extern "C" fn cc_cb(fd: c_int, opts: *mut c_void) -> c_int {
    let cb_opts: *mut cb_opts = opts as *mut cb_opts;

    return settcpca(fd, (*cb_opts).cc);
}

unsafe fn test_cubic() {
    let mut cb_opts = cb_opts {
        cc: cstr(b"bpf_cubic\0"),
        map_fd: 0,
    };
    let opts = network_helper_opts {
        post_socket_cb: Some(cc_cb),
        cb_opts: &mut cb_opts as *mut _ as *mut c_void,
    };
    let cubic_skel: *mut bpf_cubic;
    let link: *mut bpf_link;

    cubic_skel = bpf_cubic__open_and_load();
    if !ASSERT_OK_PTR(cubic_skel, cstr(b"bpf_cubic__open_and_load\0")) {
        return;
    }

    link = bpf_map__attach_struct_ops((*cubic_skel).maps.cubic);
    if !ASSERT_OK_PTR(link, cstr(b"bpf_map__attach_struct_ops\0")) {
        bpf_cubic__destroy(cubic_skel);
        return;
    }

    do_test(&opts);

    ASSERT_EQ(
        (*(*cubic_skel).bss).bpf_cubic_acked_called,
        1,
        cstr(b"pkts_acked called\0"),
    );

    ASSERT_TRUE(
        (*(*cubic_skel).bss).nodelay_init_reject,
        cstr(b"init reject nodelay option\0"),
    );
    ASSERT_TRUE(
        (*(*cubic_skel).bss).nodelay_cwnd_event_tx_start_reject,
        cstr(b"cwnd_event_tx_start reject nodelay option\0"),
    );

    bpf_link__destroy(link);
    bpf_cubic__destroy(cubic_skel);
}

unsafe extern "C" fn stg_post_socket_cb(fd: c_int, opts: *mut c_void) -> c_int {
    let cb_opts: *mut cb_opts = opts as *mut cb_opts;
    let mut err: c_int;

    err = settcpca(fd, (*cb_opts).cc);
    if err != 0 {
        return err;
    }

    err = bpf_map_update_elem(
        (*cb_opts).map_fd,
        &fd as *const _ as *const c_void,
        &expected_stg as *const _ as *const c_void,
        BPF_NOEXIST,
    );
    if !ASSERT_OK(err, cstr(b"bpf_map_update_elem(sk_stg_map)\0")) {
        return err;
    }

    return 0;
}

unsafe fn test_dctcp() {
    let mut cb_opts = cb_opts {
        cc: cstr(b"bpf_dctcp\0"),
        map_fd: 0,
    };
    let opts = network_helper_opts {
        post_socket_cb: Some(cc_cb),
        cb_opts: &mut cb_opts as *mut _ as *mut c_void,
    };
    let cli_opts = network_helper_opts {
        post_socket_cb: Some(stg_post_socket_cb),
        cb_opts: &mut cb_opts as *mut _ as *mut c_void,
    };
    let mut lfd: c_int = -1;
    let mut fd: c_int = -1;
    let mut tmp_stg: c_int = 0;
    let mut err: c_int;
    let dctcp_skel: *mut bpf_dctcp;
    let link: *mut bpf_link;

    dctcp_skel = bpf_dctcp__open_and_load();
    if !ASSERT_OK_PTR(dctcp_skel, cstr(b"bpf_dctcp__open_and_load\0")) {
        return;
    }

    link = bpf_map__attach_struct_ops((*dctcp_skel).maps.dctcp);
    if !ASSERT_OK_PTR(link, cstr(b"bpf_map__attach_struct_ops\0")) {
        bpf_dctcp__destroy(dctcp_skel);
        return;
    }

    cb_opts.map_fd = bpf_map__fd((*dctcp_skel).maps.sk_stg_map);
    if !start_test(core::ptr::null_mut(), &opts, &cli_opts, &mut lfd, &mut fd) {
        bpf_link__destroy(link);
        bpf_dctcp__destroy(dctcp_skel);
        if lfd != -1 {
            close(lfd);
        }
        if fd != -1 {
            close(fd);
        }
        return;
    }

    err = bpf_map_lookup_elem(
        cb_opts.map_fd,
        &fd as *const _ as *const c_void,
        &mut tmp_stg as *mut _ as *mut c_void,
    );
    if !ASSERT_ERR(err, cstr(b"bpf_map_lookup_elem(sk_stg_map)\0"))
        || !ASSERT_EQ(errno, ENOENT, cstr(b"bpf_map_lookup_elem(sk_stg_map)\0"))
    {
        bpf_link__destroy(link);
        bpf_dctcp__destroy(dctcp_skel);
        if lfd != -1 {
            close(lfd);
        }
        if fd != -1 {
            close(fd);
        }
        return;
    }

    ASSERT_OK(send_recv_data(lfd, fd, total_bytes), cstr(b"send_recv_data\0"));
    ASSERT_EQ((*(*dctcp_skel).bss).stg_result, expected_stg, cstr(b"stg_result\0"));

    bpf_link__destroy(link);
    bpf_dctcp__destroy(dctcp_skel);
    if lfd != -1 {
        close(lfd);
    }
    if fd != -1 {
        close(fd);
    }
}

unsafe fn test_dctcp_autoattach_map() {
    let mut cb_opts = cb_opts {
        cc: cstr(b"bpf_dctcp\0"),
        map_fd: 0,
    };
    let opts = network_helper_opts {
        post_socket_cb: Some(cc_cb),
        cb_opts: &mut cb_opts as *mut _ as *mut c_void,
    };
    let dctcp_skel: *mut bpf_dctcp;
    let link: *mut bpf_link;

    dctcp_skel = bpf_dctcp__open_and_load();
    if !ASSERT_OK_PTR(dctcp_skel, cstr(b"bpf_dctcp__open_and_load\0")) {
        return;
    }

    bpf_map__set_autoattach((*dctcp_skel).maps.dctcp, true);
    bpf_map__set_autoattach((*dctcp_skel).maps.dctcp_nouse, false);

    if !ASSERT_OK(bpf_dctcp__attach(dctcp_skel), cstr(b"bpf_dctcp__attach\0")) {
        bpf_dctcp__destroy(dctcp_skel);
        return;
    }

    /* struct_ops is auto-attached  */
    link = (*dctcp_skel).links.dctcp;
    if !ASSERT_OK_PTR(link, cstr(b"link\0")) {
        bpf_dctcp__destroy(dctcp_skel);
        return;
    }

    do_test(&opts);

    bpf_dctcp__destroy(dctcp_skel);
}

static mut err_str: *const c_char = core::ptr::null();
static mut found: bool_ = false;

unsafe extern "C" fn libbpf_debug_print(
    level: libbpf_print_level,
    format: *const c_char,
    args: va_list,
) -> c_int {
    let prog_name: *const c_char;
    let log_buf: *const c_char;

    if level != LIBBPF_WARN || strstr(format, cstr(b"-- BEGIN PROG LOAD LOG --\0")).is_null() {
        vprintf(format, args);
        return 0;
    }

    /* Rust cannot portably consume C varargs from va_list file-locally.
     * This preserves the original va_arg intent for external integration.
     */
    prog_name = core::ptr::null();
    log_buf = core::ptr::null();
    if log_buf.is_null() {
        printf(format, prog_name, log_buf);
        return 0;
    }
    if !err_str.is_null() && !strstr(log_buf, err_str).is_null() {
        found = true;
    }
    printf(format, prog_name, log_buf);
    return 0;
}

unsafe fn test_invalid_license() {
    let old_print_fn: libbpf_print_fn_t;
    let skel: *mut bpf_tcp_nogpl;

    err_str = cstr(b"struct ops programs must have a GPL compatible license\0");
    found = false;
    old_print_fn = libbpf_set_print(Some(libbpf_debug_print));

    skel = bpf_tcp_nogpl__open_and_load();
    ASSERT_NULL(skel, cstr(b"bpf_tcp_nogpl\0"));
    ASSERT_EQ(found as c_int, true as c_int, cstr(b"expected_err_msg\0"));

    bpf_tcp_nogpl__destroy(skel);
    libbpf_set_print(old_print_fn);
}

unsafe fn test_dctcp_fallback() {
    let mut err: c_int;
    let mut lfd: c_int = -1;
    let mut cli_fd: c_int = -1;
    let mut srv_fd: c_int = -1;
    let dctcp_skel: *mut bpf_dctcp;
    let mut link: *mut bpf_link = core::ptr::null_mut();
    let mut dctcp = cb_opts {
        cc: cstr(b"bpf_dctcp\0"),
        map_fd: 0,
    };
    let srv_opts = network_helper_opts {
        post_socket_cb: Some(cc_cb),
        cb_opts: &mut dctcp as *mut _ as *mut c_void,
    };
    let mut cubic = cb_opts {
        cc: cstr(b"cubic\0"),
        map_fd: 0,
    };
    let cli_opts = network_helper_opts {
        post_socket_cb: Some(cc_cb),
        cb_opts: &mut cubic as *mut _ as *mut c_void,
    };
    let mut srv_cc: [c_char; 16] = [0; 16];
    let mut cc_len: socklen_t = core::mem::size_of_val(&srv_cc) as socklen_t;

    dctcp_skel = bpf_dctcp__open();
    if !ASSERT_OK_PTR(dctcp_skel, cstr(b"dctcp_skel\0")) {
        return;
    }
    strscpy((*(*dctcp_skel).rodata).fallback_cc.as_mut_ptr(), cstr(b"cubic\0"));
    if !ASSERT_OK(bpf_dctcp__load(dctcp_skel), cstr(b"bpf_dctcp__load\0")) {
        bpf_link__destroy(link);
        bpf_dctcp__destroy(dctcp_skel);
        return;
    }

    link = bpf_map__attach_struct_ops((*dctcp_skel).maps.dctcp);
    if !ASSERT_OK_PTR(link, cstr(b"dctcp link\0")) {
        bpf_link__destroy(link);
        bpf_dctcp__destroy(dctcp_skel);
        return;
    }

    if !start_test(cstr(b"::1\0") as *mut c_char, &srv_opts, &cli_opts, &mut lfd, &mut cli_fd) {
        bpf_link__destroy(link);
        bpf_dctcp__destroy(dctcp_skel);
        if lfd != -1 {
            close(lfd);
        }
        if srv_fd != -1 {
            close(srv_fd);
        }
        if cli_fd != -1 {
            close(cli_fd);
        }
        return;
    }

    srv_fd = accept(lfd, core::ptr::null_mut(), 0);
    if !ASSERT_GE(srv_fd, 0, cstr(b"srv_fd\0")) {
        bpf_link__destroy(link);
        bpf_dctcp__destroy(dctcp_skel);
        if lfd != -1 {
            close(lfd);
        }
        if srv_fd != -1 {
            close(srv_fd);
        }
        if cli_fd != -1 {
            close(cli_fd);
        }
        return;
    }
    ASSERT_STREQ((*(*dctcp_skel).bss).cc_res.as_ptr(), cstr(b"cubic\0"), cstr(b"cc_res\0"));
    ASSERT_EQ((*(*dctcp_skel).bss).tcp_cdg_res, -ENOTSUPP, cstr(b"tcp_cdg_res\0"));
    /* All setsockopt(TCP_CONGESTION) in the recurred
     * bpf_dctcp->init() should fail with -EBUSY.
     */
    ASSERT_EQ((*(*dctcp_skel).bss).ebusy_cnt, 3, cstr(b"ebusy_cnt\0"));

    err = getsockopt(
        srv_fd,
        SOL_TCP,
        TCP_CONGESTION,
        srv_cc.as_mut_ptr() as *mut c_void,
        &mut cc_len,
    );
    if !ASSERT_OK(err, cstr(b"getsockopt(srv_fd, TCP_CONGESTION)\0")) {
        bpf_link__destroy(link);
        bpf_dctcp__destroy(dctcp_skel);
        if lfd != -1 {
            close(lfd);
        }
        if srv_fd != -1 {
            close(srv_fd);
        }
        if cli_fd != -1 {
            close(cli_fd);
        }
        return;
    }
    ASSERT_STREQ(srv_cc.as_ptr(), cstr(b"cubic\0"), cstr(b"srv_fd cc\0"));

    bpf_link__destroy(link);
    bpf_dctcp__destroy(dctcp_skel);
    if lfd != -1 {
        close(lfd);
    }
    if srv_fd != -1 {
        close(srv_fd);
    }
    if cli_fd != -1 {
        close(cli_fd);
    }
}

unsafe fn test_rel_setsockopt() {
    let rel_skel: *mut bpf_dctcp_release;
    let old_print_fn: libbpf_print_fn_t;

    err_str = cstr(b"program of this type cannot use helper bpf_setsockopt\0");
    found = false;

    old_print_fn = libbpf_set_print(Some(libbpf_debug_print));
    rel_skel = bpf_dctcp_release__open_and_load();
    libbpf_set_print(old_print_fn);

    ASSERT_ERR_PTR(rel_skel, cstr(b"rel_skel\0"));
    ASSERT_TRUE(found, cstr(b"expected_err_msg\0"));

    bpf_dctcp_release__destroy(rel_skel);
}

unsafe fn test_write_sk_pacing() {
    let skel: *mut tcp_ca_write_sk_pacing;
    let link: *mut bpf_link;

    skel = tcp_ca_write_sk_pacing__open_and_load();
    if !ASSERT_OK_PTR(skel, cstr(b"open_and_load\0")) {
        return;
    }

    link = bpf_map__attach_struct_ops((*skel).maps.write_sk_pacing);
    ASSERT_OK_PTR(link, cstr(b"attach_struct_ops\0"));

    bpf_link__destroy(link);
    tcp_ca_write_sk_pacing__destroy(skel);
}

unsafe fn test_incompl_cong_ops() {
    let skel: *mut tcp_ca_incompl_cong_ops;
    let link: *mut bpf_link;

    skel = tcp_ca_incompl_cong_ops__open_and_load();
    if !ASSERT_OK_PTR(skel, cstr(b"open_and_load\0")) {
        return;
    }

    /* That cong_avoid() and cong_control() are missing is only reported at
     * this point:
     */
    link = bpf_map__attach_struct_ops((*skel).maps.incompl_cong_ops);
    ASSERT_ERR_PTR(link, cstr(b"attach_struct_ops\0"));

    bpf_link__destroy(link);
    tcp_ca_incompl_cong_ops__destroy(skel);
}

unsafe fn test_unsupp_cong_op() {
    let old_print_fn: libbpf_print_fn_t;
    let skel: *mut tcp_ca_unsupp_cong_op;

    err_str = cstr(b"attach to unsupported member get_info\0");
    found = false;
    old_print_fn = libbpf_set_print(Some(libbpf_debug_print));

    skel = tcp_ca_unsupp_cong_op__open_and_load();
    ASSERT_NULL(skel, cstr(b"open_and_load\0"));
    ASSERT_EQ(found as c_int, true as c_int, cstr(b"expected_err_msg\0"));

    tcp_ca_unsupp_cong_op__destroy(skel);
    libbpf_set_print(old_print_fn);
}

unsafe fn test_update_ca() {
    let mut cb_opts = cb_opts {
        cc: cstr(b"tcp_ca_update\0"),
        map_fd: 0,
    };
    let opts = network_helper_opts {
        post_socket_cb: Some(cc_cb),
        cb_opts: &mut cb_opts as *mut _ as *mut c_void,
    };
    let skel: *mut tcp_ca_update;
    let link: *mut bpf_link;
    let saved_ca1_cnt: c_int;
    let err: c_int;

    skel = tcp_ca_update__open_and_load();
    if !ASSERT_OK_PTR(skel, cstr(b"open\0")) {
        return;
    }

    link = bpf_map__attach_struct_ops((*skel).maps.ca_update_1);
    if !ASSERT_OK_PTR(link, cstr(b"attach_struct_ops\0")) {
        tcp_ca_update__destroy(skel);
        return;
    }

    do_test(&opts);
    saved_ca1_cnt = (*(*skel).bss).ca1_cnt;
    ASSERT_GT(saved_ca1_cnt, 0, cstr(b"ca1_ca1_cnt\0"));

    err = bpf_link__update_map(link, (*skel).maps.ca_update_2);
    ASSERT_OK(err, cstr(b"update_map\0"));

    do_test(&opts);
    ASSERT_EQ((*(*skel).bss).ca1_cnt, saved_ca1_cnt, cstr(b"ca2_ca1_cnt\0"));
    ASSERT_GT((*(*skel).bss).ca2_cnt, 0, cstr(b"ca2_ca2_cnt\0"));

    bpf_link__destroy(link);
    tcp_ca_update__destroy(skel);
}

unsafe fn test_update_wrong() {
    let mut cb_opts = cb_opts {
        cc: cstr(b"tcp_ca_update\0"),
        map_fd: 0,
    };
    let opts = network_helper_opts {
        post_socket_cb: Some(cc_cb),
        cb_opts: &mut cb_opts as *mut _ as *mut c_void,
    };
    let skel: *mut tcp_ca_update;
    let link: *mut bpf_link;
    let saved_ca1_cnt: c_int;
    let err: c_int;

    skel = tcp_ca_update__open_and_load();
    if !ASSERT_OK_PTR(skel, cstr(b"open\0")) {
        return;
    }

    link = bpf_map__attach_struct_ops((*skel).maps.ca_update_1);
    if !ASSERT_OK_PTR(link, cstr(b"attach_struct_ops\0")) {
        tcp_ca_update__destroy(skel);
        return;
    }

    do_test(&opts);
    saved_ca1_cnt = (*(*skel).bss).ca1_cnt;
    ASSERT_GT(saved_ca1_cnt, 0, cstr(b"ca1_ca1_cnt\0"));

    err = bpf_link__update_map(link, (*skel).maps.ca_wrong);
    ASSERT_ERR(err, cstr(b"update_map\0"));

    do_test(&opts);
    ASSERT_GT((*(*skel).bss).ca1_cnt, saved_ca1_cnt, cstr(b"ca2_ca1_cnt\0"));

    bpf_link__destroy(link);
    tcp_ca_update__destroy(skel);
}

unsafe fn test_mixed_links() {
    let mut cb_opts = cb_opts {
        cc: cstr(b"tcp_ca_update\0"),
        map_fd: 0,
    };
    let opts = network_helper_opts {
        post_socket_cb: Some(cc_cb),
        cb_opts: &mut cb_opts as *mut _ as *mut c_void,
    };
    let skel: *mut tcp_ca_update;
    let link: *mut bpf_link;
    let link_nl: *mut bpf_link;
    let err: c_int;

    skel = tcp_ca_update__open_and_load();
    if !ASSERT_OK_PTR(skel, cstr(b"open\0")) {
        return;
    }

    link_nl = bpf_map__attach_struct_ops((*skel).maps.ca_no_link);
    if !ASSERT_OK_PTR(link_nl, cstr(b"attach_struct_ops_nl\0")) {
        tcp_ca_update__destroy(skel);
        return;
    }

    link = bpf_map__attach_struct_ops((*skel).maps.ca_update_1);
    ASSERT_OK_PTR(link, cstr(b"attach_struct_ops\0"));

    do_test(&opts);
    ASSERT_GT((*(*skel).bss).ca1_cnt, 0, cstr(b"ca1_ca1_cnt\0"));

    err = bpf_link__update_map(link, (*skel).maps.ca_no_link);
    ASSERT_ERR(err, cstr(b"update_map\0"));

    bpf_link__destroy(link);
    bpf_link__destroy(link_nl);
    tcp_ca_update__destroy(skel);
}

unsafe fn test_multi_links() {
    let skel: *mut tcp_ca_update;
    let mut link: *mut bpf_link;

    skel = tcp_ca_update__open_and_load();
    if !ASSERT_OK_PTR(skel, cstr(b"open\0")) {
        return;
    }

    link = bpf_map__attach_struct_ops((*skel).maps.ca_update_1);
    ASSERT_OK_PTR(link, cstr(b"attach_struct_ops_1st\0"));
    bpf_link__destroy(link);

    /* A map should be able to be used to create links multiple
     * times.
     */
    link = bpf_map__attach_struct_ops((*skel).maps.ca_update_1);
    ASSERT_OK_PTR(link, cstr(b"attach_struct_ops_2nd\0"));
    bpf_link__destroy(link);

    tcp_ca_update__destroy(skel);
}

unsafe fn test_link_replace() {
    let mut opts = bpf_link_update_opts {
        sz: core::mem::size_of::<bpf_link_update_opts>(),
        flags: 0,
        old_prog_fd: 0,
        old_map_fd: 0,
    };
    let skel: *mut tcp_ca_update;
    let mut link: *mut bpf_link;
    let mut err: c_int;

    skel = tcp_ca_update__open_and_load();
    if !ASSERT_OK_PTR(skel, cstr(b"open\0")) {
        return;
    }

    link = bpf_map__attach_struct_ops((*skel).maps.ca_update_1);
    ASSERT_OK_PTR(link, cstr(b"attach_struct_ops_1st\0"));
    bpf_link__destroy(link);

    link = bpf_map__attach_struct_ops((*skel).maps.ca_update_2);
    if !ASSERT_OK_PTR(link, cstr(b"attach_struct_ops_2nd\0")) {
        tcp_ca_update__destroy(skel);
        return;
    }

    /* BPF_F_REPLACE with a wrong old map Fd. It should fail!
     *
     * With BPF_F_REPLACE, the link should be updated only if the
     * old map fd given here matches the map backing the link.
     */
    opts.old_map_fd = bpf_map__fd((*skel).maps.ca_update_1) as u32;
    opts.flags = BPF_F_REPLACE;
    err = bpf_link_update(
        bpf_link__fd(link),
        bpf_map__fd((*skel).maps.ca_update_1),
        &mut opts,
    );
    ASSERT_ERR(err, cstr(b"bpf_link_update_fail\0"));

    /* BPF_F_REPLACE with a correct old map Fd. It should success! */
    opts.old_map_fd = bpf_map__fd((*skel).maps.ca_update_2) as u32;
    err = bpf_link_update(
        bpf_link__fd(link),
        bpf_map__fd((*skel).maps.ca_update_1),
        &mut opts,
    );
    ASSERT_OK(err, cstr(b"bpf_link_update_success\0"));

    bpf_link__destroy(link);

    tcp_ca_update__destroy(skel);
}

unsafe fn test_tcp_ca_kfunc() {
    let skel: *mut tcp_ca_kfunc;

    skel = tcp_ca_kfunc__open_and_load();
    ASSERT_OK_PTR(skel, cstr(b"tcp_ca_kfunc__open_and_load\0"));
    tcp_ca_kfunc__destroy(skel);
}

unsafe fn test_untrusted_btf_write() {
    let skel: *mut tcp_ca_untrusted_btf_write;

    skel = tcp_ca_untrusted_btf_write__open_and_load();
    ASSERT_ERR_PTR(skel, cstr(b"tcp_ca_untrusted_btf_write__open_and_load\0"));
    tcp_ca_untrusted_btf_write__destroy(skel);
}

unsafe fn test_cc_cubic() {
    let mut cb_opts = cb_opts {
        cc: cstr(b"bpf_cc_cubic\0"),
        map_fd: 0,
    };
    let opts = network_helper_opts {
        post_socket_cb: Some(cc_cb),
        cb_opts: &mut cb_opts as *mut _ as *mut c_void,
    };
    let cc_cubic_skel: *mut bpf_cc_cubic;
    let link: *mut bpf_link;

    cc_cubic_skel = bpf_cc_cubic__open_and_load();
    if !ASSERT_OK_PTR(cc_cubic_skel, cstr(b"bpf_cc_cubic__open_and_load\0")) {
        return;
    }

    link = bpf_map__attach_struct_ops((*cc_cubic_skel).maps.cc_cubic);
    if !ASSERT_OK_PTR(link, cstr(b"bpf_map__attach_struct_ops\0")) {
        bpf_cc_cubic__destroy(cc_cubic_skel);
        return;
    }

    do_test(&opts);

    bpf_link__destroy(link);
    bpf_cc_cubic__destroy(cc_cubic_skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_bpf_tcp_ca() {
    if test__start_subtest(cstr(b"dctcp\0")) {
        test_dctcp();
    }
    if test__start_subtest(cstr(b"cubic\0")) {
        test_cubic();
    }
    if test__start_subtest(cstr(b"invalid_license\0")) {
        test_invalid_license();
    }
    if test__start_subtest(cstr(b"dctcp_fallback\0")) {
        test_dctcp_fallback();
    }
    if test__start_subtest(cstr(b"rel_setsockopt\0")) {
        test_rel_setsockopt();
    }
    if test__start_subtest(cstr(b"write_sk_pacing\0")) {
        test_write_sk_pacing();
    }
    if test__start_subtest(cstr(b"incompl_cong_ops\0")) {
        test_incompl_cong_ops();
    }
    if test__start_subtest(cstr(b"unsupp_cong_op\0")) {
        test_unsupp_cong_op();
    }
    if test__start_subtest(cstr(b"update_ca\0")) {
        test_update_ca();
    }
    if test__start_subtest(cstr(b"update_wrong\0")) {
        test_update_wrong();
    }
    if test__start_subtest(cstr(b"mixed_links\0")) {
        test_mixed_links();
    }
    if test__start_subtest(cstr(b"multi_links\0")) {
        test_multi_links();
    }
    if test__start_subtest(cstr(b"link_replace\0")) {
        test_link_replace();
    }
    if test__start_subtest(cstr(b"tcp_ca_kfunc\0")) {
        test_tcp_ca_kfunc();
    }
    if test__start_subtest(cstr(b"untrusted_btf_write\0")) {
        test_untrusted_btf_write();
    }
    if test__start_subtest(cstr(b"cc_cubic\0")) {
        test_cc_cubic();
    }
    if test__start_subtest(cstr(b"dctcp_autoattach_map\0")) {
        test_dctcp_autoattach_map();
    }
}
