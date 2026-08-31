// SPDX-License-Identifier: GPL-2.0

// Translated from C implementation source. Includes from the original file:
// <test_progs.h>, "netns_cookie_prog.skel.h", and "network_helpers.h".

use core::ffi::{c_char, c_int, c_uint, c_void};

const SO_NETNS_COOKIE: c_int = 71;
const loopback: c_int = 1;

static mut duration: c_int = 0;

type socklen_t = c_uint;
type uint64_t = u64;

const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const SOL_SOCKET: c_int = 1;
const BPF_SK_MSG_VERDICT: c_int = 7;
const BPF_TCX_INGRESS: c_int = 0;

#[repr(C)]
struct bpf_prog_attach_opts {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_prog_detach_opts {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
struct netns_cookie_prog__progs {
    get_netns_cookie_sockops: *mut bpf_program,
    get_netns_cookie_sk_msg: *mut bpf_program,
    get_netns_cookie_tcx: *mut bpf_program,
    get_netns_cookie_cgroup_skb: *mut bpf_program,
}

#[repr(C)]
struct netns_cookie_prog__maps {
    sock_map: *mut bpf_map,
    sockops_netns_cookies: *mut bpf_map,
    sk_msg_netns_cookies: *mut bpf_map,
}

#[repr(C)]
struct netns_cookie_prog__links {
    get_netns_cookie_sockops: *mut bpf_link,
    get_netns_cookie_cgroup_skb: *mut bpf_link,
}

#[repr(C)]
struct netns_cookie_prog__bss {
    tcx_init_netns_cookie: uint64_t,
    tcx_netns_cookie: uint64_t,
    cgroup_skb_init_netns_cookie: uint64_t,
    cgroup_skb_netns_cookie: uint64_t,
}

#[repr(C)]
struct netns_cookie_prog {
    progs: netns_cookie_prog__progs,
    maps: netns_cookie_prog__maps,
    links: netns_cookie_prog__links,
    bss: *mut netns_cookie_prog__bss,
}

unsafe extern "C" {
    fn netns_cookie_prog__open_and_load() -> *mut netns_cookie_prog;
    fn netns_cookie_prog__destroy(skel: *mut netns_cookie_prog);

    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn bpf_program__attach_cgroup(prog: *mut bpf_program, cgroup_fd: c_int) -> *mut bpf_link;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_prog_attach(prog_fd: c_int, target_fd: c_int, attach_type: c_int, flags: c_uint) -> c_int;
    fn bpf_prog_attach_opts(
        prog_fd: c_int,
        target_fd: c_int,
        attach_type: c_int,
        opts: *const bpf_prog_attach_opts,
    ) -> c_int;
    fn bpf_prog_detach_opts(
        prog_fd: c_int,
        target_fd: c_int,
        attach_type: c_int,
        opts: *const bpf_prog_detach_opts,
    ) -> c_int;

    fn start_server(
        family: c_int,
        type_: c_int,
        addr: *const c_char,
        port: c_int,
        timeout_ms: c_int,
    ) -> c_int;
    fn connect_to_fd(fd: c_int, timeout_ms: c_int) -> c_int;
    fn send(fd: c_int, buf: *const c_void, len: usize, flags: c_int) -> isize;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn getsockopt(
        fd: c_int,
        level: c_int,
        optname: c_int,
        optval: *mut c_void,
        optlen: *mut socklen_t,
    ) -> c_int;
    fn close(fd: c_int) -> c_int;

    static mut errno: c_int;
}

pub unsafe fn test_netns_cookie() {
    let mut opta: bpf_prog_attach_opts = core::mem::zeroed();
    let mut optd: bpf_prog_detach_opts = core::mem::zeroed();
    let mut server_fd: c_int = -1;
    let mut client_fd: c_int = -1;
    let mut cgroup_fd: c_int = -1;
    let mut err: c_int;
    let mut val: c_int = 0;
    let mut ret: isize;
    let mut map: c_int;
    let mut verdict: c_int;
    let mut tc_fd: c_int = 0;
    let mut skel: *mut netns_cookie_prog;
    let mut cookie_expected_value: uint64_t = 0;
    let mut vallen: socklen_t = core::mem::size_of_val(&cookie_expected_value) as socklen_t;
    static send_msg: &[u8; 8] = b"message\0";

    skel = netns_cookie_prog__open_and_load();
    if !ASSERT_OK_PTR!(skel, "skel_open") {
        return;
    }

    cgroup_fd = test__join_cgroup(c"/netns_cookie".as_ptr());
    if CHECK!(
        cgroup_fd < 0,
        "join_cgroup",
        "cgroup creation failed\n"
    ) {
        goto_done(
            skel,
            server_fd,
            client_fd,
            cgroup_fd,
        );
        return;
    }

    (*skel).links.get_netns_cookie_sockops =
        bpf_program__attach_cgroup((*skel).progs.get_netns_cookie_sockops, cgroup_fd);
    if !ASSERT_OK_PTR!(
        (*skel).links.get_netns_cookie_sockops,
        "prog_attach_sockops"
    ) {
        goto_done(skel, server_fd, client_fd, cgroup_fd);
        return;
    }

    verdict = bpf_program__fd((*skel).progs.get_netns_cookie_sk_msg);
    map = bpf_map__fd((*skel).maps.sock_map);
    err = bpf_prog_attach(verdict, map, BPF_SK_MSG_VERDICT, 0);
    if !ASSERT_OK!(err, "prog_attach_sk_msg") {
        goto_done(skel, server_fd, client_fd, cgroup_fd);
        return;
    }

    tc_fd = bpf_program__fd((*skel).progs.get_netns_cookie_tcx);
    err = bpf_prog_attach_opts(tc_fd, loopback, BPF_TCX_INGRESS, &opta);
    if !ASSERT_OK!(err, "prog_attach_tcx") {
        goto_done(skel, server_fd, client_fd, cgroup_fd);
        return;
    }

    (*skel).links.get_netns_cookie_cgroup_skb =
        bpf_program__attach_cgroup((*skel).progs.get_netns_cookie_cgroup_skb, cgroup_fd);
    if !ASSERT_OK_PTR!(
        (*skel).links.get_netns_cookie_cgroup_skb,
        "prog_attach_cgroup_skb"
    ) {
        goto_cleanup_tc(skel, server_fd, client_fd, cgroup_fd, tc_fd, &optd);
        return;
    }

    server_fd = start_server(AF_INET6, SOCK_STREAM, c"::1".as_ptr(), 0, 0);
    if CHECK!(
        server_fd < 0,
        "start_server",
        "errno %d\n",
        errno
    ) {
        goto_cleanup_tc(skel, server_fd, client_fd, cgroup_fd, tc_fd, &optd);
        return;
    }

    client_fd = connect_to_fd(server_fd, 0);
    if CHECK!(
        client_fd < 0,
        "connect_to_fd",
        "errno %d\n",
        errno
    ) {
        goto_cleanup_tc(skel, server_fd, client_fd, cgroup_fd, tc_fd, &optd);
        return;
    }

    ret = send(
        client_fd,
        send_msg.as_ptr() as *const c_void,
        core::mem::size_of_val(send_msg),
        0,
    );
    if CHECK!(
        ret != core::mem::size_of_val(send_msg) as isize,
        "send(msg)",
        "ret:%d\n",
        ret as c_int
    ) {
        goto_cleanup_tc(skel, server_fd, client_fd, cgroup_fd, tc_fd, &optd);
        return;
    }

    err = bpf_map_lookup_elem(
        bpf_map__fd((*skel).maps.sockops_netns_cookies),
        &client_fd as *const _ as *const c_void,
        &mut val as *mut _ as *mut c_void,
    );
    if !ASSERT_OK!(err, "map_lookup(sockops_netns_cookies)") {
        goto_cleanup_tc(skel, server_fd, client_fd, cgroup_fd, tc_fd, &optd);
        return;
    }

    err = getsockopt(
        client_fd,
        SOL_SOCKET,
        SO_NETNS_COOKIE,
        &mut cookie_expected_value as *mut _ as *mut c_void,
        &mut vallen,
    );
    if !ASSERT_OK!(err, "getsockopt") {
        goto_cleanup_tc(skel, server_fd, client_fd, cgroup_fd, tc_fd, &optd);
        return;
    }

    ASSERT_EQ!(val, cookie_expected_value, "cookie_value_sockops");

    err = bpf_map_lookup_elem(
        bpf_map__fd((*skel).maps.sk_msg_netns_cookies),
        &client_fd as *const _ as *const c_void,
        &mut val as *mut _ as *mut c_void,
    );
    if !ASSERT_OK!(err, "map_lookup(sk_msg_netns_cookies)") {
        goto_cleanup_tc(skel, server_fd, client_fd, cgroup_fd, tc_fd, &optd);
        return;
    }

    ASSERT_EQ!(val, cookie_expected_value, "cookie_value_sk_msg");
    ASSERT_EQ!(
        (*(*skel).bss).tcx_init_netns_cookie,
        cookie_expected_value,
        "cookie_value_init_tcx"
    );
    ASSERT_EQ!(
        (*(*skel).bss).tcx_netns_cookie,
        cookie_expected_value,
        "cookie_value_tcx"
    );
    ASSERT_EQ!(
        (*(*skel).bss).cgroup_skb_init_netns_cookie,
        cookie_expected_value,
        "cookie_value_init_cgroup_skb"
    );
    ASSERT_EQ!(
        (*(*skel).bss).cgroup_skb_netns_cookie,
        cookie_expected_value,
        "cookie_value_cgroup_skb"
    );

    goto_cleanup_tc(skel, server_fd, client_fd, cgroup_fd, tc_fd, &optd);
}

unsafe fn goto_cleanup_tc(
    skel: *mut netns_cookie_prog,
    server_fd: c_int,
    client_fd: c_int,
    cgroup_fd: c_int,
    tc_fd: c_int,
    optd: *const bpf_prog_detach_opts,
) {
    let err = bpf_prog_detach_opts(tc_fd, loopback, BPF_TCX_INGRESS, optd);
    ASSERT_OK!(err, "prog_detach");

    goto_done(skel, server_fd, client_fd, cgroup_fd);
}

unsafe fn goto_done(
    skel: *mut netns_cookie_prog,
    server_fd: c_int,
    client_fd: c_int,
    cgroup_fd: c_int,
) {
    if server_fd != -1 {
        close(server_fd);
    }
    if client_fd != -1 {
        close(client_fd);
    }
    if cgroup_fd != -1 {
        close(cgroup_fd);
    }
    netns_cookie_prog__destroy(skel);
}
