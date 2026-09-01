// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020, Tessares SA. */
/* Copyright (c) 2022, SUSE. */

/* Translated from testing/selftests/bpf/prog_tests/mptcp.c.
 * C include dependencies are expected to be provided by the surrounding
 * selftest bindings.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const NS_TEST: &[u8] = b"mptcp_ns\0";
const ADDR_1: &[u8] = b"10.0.1.1\0";
const ADDR_2: &[u8] = b"10.0.1.2\0";
const PORT_1: u16 = 10001;

const IPPROTO_MPTCP: c_int = 262;
const SOL_MPTCP: c_int = 284;
const MPTCP_INFO: c_int = 1;
const MPTCP_INFO_FLAG_FALLBACK: u32 = 1u32 << 0;
const MPTCP_INFO_FLAG_REMOTE_KEY_RECEIVED: u32 = 1u32 << 1;

const TCP_CA_NAME_MAX: usize = 16;

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;
type socklen_t = u32;
type size_t = usize;
type u8_ = u8;

const AF_INET: c_int = 2;
const SOCK_STREAM: c_int = 1;
const SOCK_CLOEXEC: c_int = 0o2000000;
const SOL_SOCKET: c_int = 1;
const SO_PROTOCOL: c_int = 38;
const SO_MARK: c_int = 36;
const SOL_TCP: c_int = 6;
const TCP_CONGESTION: c_int = 13;
const O_RDONLY: c_int = 0;
const EIO: c_int = 5;
const EOPNOTSUPP: c_int = 95;
const BPF_CGROUP_SOCK_OPS: c_int = 9;
const BPF_SK_SKB_STREAM_VERDICT: c_int = 14;
const BPF_NOEXIST: u64 = 1;

#[repr(C)]
struct sock;

#[repr(C)]
struct netns_obj;

#[repr(C)]
struct network_helper_opts {
    timeout_ms: c_int,
    proto: c_int,
}

#[repr(C)]
struct __mptcp_info {
    mptcpi_subflows: __u8,
    mptcpi_add_addr_signal: __u8,
    mptcpi_add_addr_accepted: __u8,
    mptcpi_subflows_max: __u8,
    mptcpi_add_addr_signal_max: __u8,
    mptcpi_add_addr_accepted_max: __u8,
    mptcpi_flags: __u32,
    mptcpi_token: __u32,
    mptcpi_write_seq: __u64,
    mptcpi_snd_una: __u64,
    mptcpi_rcv_nxt: __u64,
    mptcpi_local_addr_used: __u8,
    mptcpi_local_addr_max: __u8,
    mptcpi_csum_enabled: __u8,
    mptcpi_retransmits: __u32,
    mptcpi_bytes_retrans: __u64,
    mptcpi_bytes_sent: __u64,
    mptcpi_bytes_received: __u64,
    mptcpi_bytes_acked: __u64,
}

#[repr(C)]
struct mptcp_storage {
    invoked: __u32,
    is_mptcp: __u32,
    sk: *mut sock,
    token: __u32,
    first: *mut sock,
    ca_name: [c_char; TCP_CA_NAME_MAX],
}

#[repr(C)]
struct mptcp_sock_bss {
    token: __u32,
}

#[repr(C)]
struct mptcp_sock_progs {
    _sockops: *mut c_void,
}

#[repr(C)]
struct mptcp_sock_maps {
    socket_storage_map: *mut c_void,
}

#[repr(C)]
struct mptcp_sock {
    progs: mptcp_sock_progs,
    maps: mptcp_sock_maps,
    bss: *mut mptcp_sock_bss,
}

#[repr(C)]
struct mptcpify_bss {
    pid: c_int,
}

#[repr(C)]
struct mptcpify {
    bss: *mut mptcpify_bss,
}

#[repr(C)]
struct mptcp_subflow_bss {
    pid: c_int,
}

#[repr(C)]
struct mptcp_subflow_progs {
    mptcp_subflow: *mut c_void,
    _getsockopt_subflow: *mut c_void,
}

#[repr(C)]
struct mptcp_subflow_links {
    mptcp_subflow: *mut c_void,
    _getsockopt_subflow: *mut c_void,
}

#[repr(C)]
struct mptcp_subflow {
    bss: *mut mptcp_subflow_bss,
    progs: mptcp_subflow_progs,
    links: mptcp_subflow_links,
}

#[repr(C)]
struct mptcp_sockmap_bss {
    trace_port: __u16,
    sk_index: c_int,
    redirect_idx: c_int,
    helper_ret: c_int,
}

#[repr(C)]
struct mptcp_sockmap_progs {
    mptcp_sockmap_inject: *mut c_void,
    mptcp_sockmap_redirect: *mut c_void,
}

#[repr(C)]
struct mptcp_sockmap_maps {
    sock_map: *mut c_void,
}

#[repr(C)]
struct mptcp_sockmap_links {
    mptcp_sockmap_inject: *mut c_void,
}

#[repr(C)]
struct mptcp_sockmap {
    bss: *mut mptcp_sockmap_bss,
    progs: mptcp_sockmap_progs,
    maps: mptcp_sockmap_maps,
    links: mptcp_sockmap_links,
}

unsafe extern "C" {
    fn start_server_str(
        family: c_int,
        type_: c_int,
        addr_str: *const c_char,
        port: __u16,
        opts: *mut network_helper_opts,
    ) -> c_int;
    fn start_server(family: c_int, type_: c_int, addr: *const c_void, port: __u16, timeout: c_int)
        -> c_int;
    fn connect_to_fd(server_fd: c_int, timeout_ms: c_int) -> c_int;
    fn connect_to_fd_opts(server_fd: c_int, opts: *const c_void) -> c_int;
    fn get_socket_local_port(fd: c_int) -> __u16;
    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn test__skip();
    fn netns_new(name: *const c_char, attach: bool) -> *mut netns_obj;
    fn netns_free(netns: *mut netns_obj);

    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool;
    fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE<T>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_GT<T>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_EQ<T>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_FALSE(condition: u32, name: *const c_char) -> bool;
    fn ASSERT_TRUE(condition: u32, name: *const c_char) -> bool;
    fn ASSERT_STRNEQ(
        actual: *const c_char,
        expected: *const c_char,
        len: size_t,
        name: *const c_char,
    ) -> bool;
    fn ASSERT_STREQ(actual: *const c_char, expected: *const c_char, name: *const c_char)
        -> bool;

    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64)
        -> c_int;
    fn bpf_prog_attach(prog_fd: c_int, target_fd: c_int, type_: c_int, flags: c_uint) -> c_int;
    fn bpf_program__fd(prog: *mut c_void) -> c_int;
    fn bpf_map__fd(map: *mut c_void) -> c_int;
    fn bpf_program__attach_cgroup(prog: *mut c_void, cgroup_fd: c_int) -> *mut c_void;
    fn libbpf_get_error<T>(ptr: *mut T) -> c_int;

    fn mptcp_sock__open_and_load() -> *mut mptcp_sock;
    fn mptcp_sock__attach(skel: *mut mptcp_sock) -> c_int;
    fn mptcp_sock__destroy(skel: *mut mptcp_sock);
    fn mptcpify__open_and_load() -> *mut mptcpify;
    fn mptcpify__attach(skel: *mut mptcpify) -> c_int;
    fn mptcpify__destroy(skel: *mut mptcpify);
    fn mptcp_subflow__open_and_load() -> *mut mptcp_subflow;
    fn mptcp_subflow__destroy(skel: *mut mptcp_subflow);
    fn mptcp_sockmap__open_and_load() -> *mut mptcp_sockmap;
    fn mptcp_sockmap__destroy(skel: *mut mptcp_sockmap);

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> isize;
    fn close(fd: c_int) -> c_int;
    fn getsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *mut c_void,
        option_len: *mut socklen_t,
    ) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn getpid() -> c_int;
    fn ntohs(netshort: __u16) -> __u16;
    fn accept(sockfd: c_int, addr: *mut c_void, addrlen: *mut socklen_t) -> c_int;
    fn send(sockfd: c_int, buf: *const c_void, len: size_t, flags: c_int) -> isize;
    fn recv(sockfd: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> isize;
    fn printf(format: *const c_char, ...) -> c_int;
    fn SYS(fail: *const c_char, cmd: *const c_char, ...) -> c_int;
    fn SYS_NOFAIL(cmd: *const c_char, ...) -> c_int;
}

fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

unsafe fn start_mptcp_server(
    family: c_int,
    addr_str: *const c_char,
    port: __u16,
    timeout_ms: c_int,
) -> c_int {
    let mut opts = network_helper_opts {
        timeout_ms,
        proto: IPPROTO_MPTCP,
    };

    start_server_str(family, SOCK_STREAM, addr_str, port, &mut opts)
}

unsafe fn verify_tsk(map_fd: c_int, client_fd: c_int) -> c_int {
    let mut err: c_int;
    let cfd = client_fd;
    let mut val: mptcp_storage = core::mem::zeroed();

    err = bpf_map_lookup_elem(
        map_fd,
        &cfd as *const _ as *const c_void,
        &mut val as *mut _ as *mut c_void,
    );
    if !ASSERT_OK(err, cstr(b"bpf_map_lookup_elem\0")) {
        return err;
    }

    if !ASSERT_EQ(val.invoked, 1u32, cstr(b"unexpected invoked count\0")) {
        err += 1;
    }

    if !ASSERT_EQ(val.is_mptcp, 0u32, cstr(b"unexpected is_mptcp\0")) {
        err += 1;
    }

    err
}

unsafe fn get_msk_ca_name(ca_name: *mut c_char) {
    let len: isize;
    let fd: c_int;

    fd = open(cstr(b"/proc/sys/net/ipv4/tcp_congestion_control\0"), O_RDONLY);
    if !ASSERT_GE(fd, 0, cstr(b"failed to open tcp_congestion_control\0")) {
        return;
    }

    len = read(fd, ca_name as *mut c_void, TCP_CA_NAME_MAX);
    if !ASSERT_GT(len, 0isize, cstr(b"failed to read ca_name\0")) {
        close(fd);
        return;
    }

    if len > 0 && *ca_name.add(len as usize - 1) == b'\n' as c_char {
        *ca_name.add(len as usize - 1) = b'\0' as c_char;
    }

    close(fd);
}

unsafe fn verify_msk(map_fd: c_int, client_fd: c_int, token: __u32) -> c_int {
    let mut ca_name = [0 as c_char; TCP_CA_NAME_MAX];
    let mut err: c_int;
    let cfd = client_fd;
    let mut val: mptcp_storage = core::mem::zeroed();

    if !ASSERT_GT(token, 0u32, cstr(b"invalid token\0")) {
        return -1;
    }

    get_msk_ca_name(ca_name.as_mut_ptr());

    err = bpf_map_lookup_elem(
        map_fd,
        &cfd as *const _ as *const c_void,
        &mut val as *mut _ as *mut c_void,
    );
    if !ASSERT_OK(err, cstr(b"bpf_map_lookup_elem\0")) {
        return err;
    }

    if !ASSERT_EQ(val.invoked, 1u32, cstr(b"unexpected invoked count\0")) {
        err += 1;
    }
    if !ASSERT_EQ(val.is_mptcp, 1u32, cstr(b"unexpected is_mptcp\0")) {
        err += 1;
    }
    if !ASSERT_EQ(val.token, token, cstr(b"unexpected token\0")) {
        err += 1;
    }
    if !ASSERT_EQ(val.first, val.sk, cstr(b"unexpected first\0")) {
        err += 1;
    }
    if !ASSERT_STRNEQ(
        val.ca_name.as_ptr(),
        ca_name.as_ptr(),
        TCP_CA_NAME_MAX,
        cstr(b"unexpected ca_name\0"),
    ) {
        err += 1;
    }

    err
}

unsafe fn run_test(cgroup_fd: c_int, server_fd: c_int, is_mptcp: bool) -> c_int {
    let client_fd: c_int;
    let prog_fd: c_int;
    let map_fd: c_int;
    let mut err: c_int;
    let sock_skel: *mut mptcp_sock;

    sock_skel = mptcp_sock__open_and_load();
    if !ASSERT_OK_PTR(sock_skel, cstr(b"skel_open_load\0")) {
        return libbpf_get_error(sock_skel);
    }

    err = mptcp_sock__attach(sock_skel);
    if !ASSERT_OK(err, cstr(b"skel_attach\0")) {
        mptcp_sock__destroy(sock_skel);
        return err;
    }

    prog_fd = bpf_program__fd((*sock_skel).progs._sockops);
    map_fd = bpf_map__fd((*sock_skel).maps.socket_storage_map);
    err = bpf_prog_attach(prog_fd, cgroup_fd, BPF_CGROUP_SOCK_OPS, 0);
    if !ASSERT_OK(err, cstr(b"bpf_prog_attach\0")) {
        mptcp_sock__destroy(sock_skel);
        return err;
    }

    client_fd = connect_to_fd(server_fd, 0);
    if !ASSERT_GE(client_fd, 0, cstr(b"connect to fd\0")) {
        err = -EIO;
        mptcp_sock__destroy(sock_skel);
        return err;
    }

    err += if is_mptcp {
        verify_msk(map_fd, client_fd, (*(*sock_skel).bss).token)
    } else {
        verify_tsk(map_fd, client_fd)
    };

    close(client_fd);
    mptcp_sock__destroy(sock_skel);
    err
}

unsafe fn test_base() {
    let mut netns: *mut netns_obj = ptr::null_mut();
    let mut server_fd: c_int;
    let cgroup_fd: c_int;

    cgroup_fd = test__join_cgroup(cstr(b"/mptcp\0"));
    if !ASSERT_GE(cgroup_fd, 0, cstr(b"test__join_cgroup\0")) {
        return;
    }

    netns = netns_new(cstr(NS_TEST), true);
    if !ASSERT_OK_PTR(netns, cstr(b"netns_new\0")) {
        close(cgroup_fd);
        return;
    }

    /* without MPTCP */
    server_fd = start_server(AF_INET, SOCK_STREAM, ptr::null(), 0, 0);
    if ASSERT_GE(server_fd, 0, cstr(b"start_server\0")) {
        ASSERT_OK(run_test(cgroup_fd, server_fd, false), cstr(b"run_test tcp\0"));
        close(server_fd);
    }

    /* with MPTCP */
    server_fd = start_mptcp_server(AF_INET, ptr::null(), 0, 0);
    if ASSERT_GE(server_fd, 0, cstr(b"start_mptcp_server\0")) {
        ASSERT_OK(run_test(cgroup_fd, server_fd, true), cstr(b"run_test mptcp\0"));
        close(server_fd);
    }

    netns_free(netns);
    close(cgroup_fd);
}

unsafe fn send_byte(fd: c_int) {
    let b: c_char = 0x55;

    ASSERT_EQ(
        write(fd, &b as *const _ as *const c_void, size_of::<c_char>()),
        1isize,
        cstr(b"send single byte\0"),
    );
}

unsafe fn verify_mptcpify(server_fd: c_int, client_fd: c_int) -> c_int {
    let mut info: __mptcp_info = core::mem::zeroed();
    let mut optlen: socklen_t;
    let mut protocol: c_int = 0;
    let mut err: c_int = 0;

    optlen = size_of::<c_int>() as socklen_t;
    if !ASSERT_OK(
        getsockopt(
            server_fd,
            SOL_SOCKET,
            SO_PROTOCOL,
            &mut protocol as *mut _ as *mut c_void,
            &mut optlen,
        ),
        cstr(b"getsockopt(SOL_PROTOCOL)\0"),
    ) {
        return -1;
    }

    if !ASSERT_EQ(protocol, IPPROTO_MPTCP, cstr(b"protocol isn't MPTCP\0")) {
        err += 1;
    }

    optlen = size_of::<__mptcp_info>() as socklen_t;
    if !ASSERT_OK(
        getsockopt(
            client_fd,
            SOL_MPTCP,
            MPTCP_INFO,
            &mut info as *mut _ as *mut c_void,
            &mut optlen,
        ),
        cstr(b"getsockopt(MPTCP_INFO)\0"),
    ) {
        return -1;
    }

    if !ASSERT_GE(info.mptcpi_flags, 0u32, cstr(b"unexpected mptcpi_flags\0")) {
        err += 1;
    }
    if !ASSERT_FALSE(
        info.mptcpi_flags & MPTCP_INFO_FLAG_FALLBACK,
        cstr(b"MPTCP fallback\0"),
    ) {
        err += 1;
    }
    if !ASSERT_TRUE(
        info.mptcpi_flags & MPTCP_INFO_FLAG_REMOTE_KEY_RECEIVED,
        cstr(b"no remote key received\0"),
    ) {
        err += 1;
    }

    err
}

unsafe fn run_mptcpify(cgroup_fd: c_int, type_: c_int) -> c_int {
    let mut server_fd: c_int = -1;
    let mut client_fd: c_int = -1;
    let mut err: c_int = 0;
    let mptcpify_skel: *mut mptcpify;

    mptcpify_skel = mptcpify__open_and_load();
    if !ASSERT_OK_PTR(mptcpify_skel, cstr(b"skel_open_load\0")) {
        return libbpf_get_error(mptcpify_skel);
    }

    (*(*mptcpify_skel).bss).pid = getpid();

    err = mptcpify__attach(mptcpify_skel);
    if !ASSERT_OK(err, cstr(b"skel_attach\0")) {
        mptcpify__destroy(mptcpify_skel);
        return err;
    }

    /* without MPTCP */
    server_fd = start_server(AF_INET, type_, ptr::null(), 0, 0);
    if !ASSERT_GE(server_fd, 0, cstr(b"start_server\0")) {
        err = -EIO;
        mptcpify__destroy(mptcpify_skel);
        return err;
    }

    client_fd = connect_to_fd(server_fd, 0);
    if !ASSERT_GE(client_fd, 0, cstr(b"connect to fd\0")) {
        err = -EIO;
        close(server_fd);
        mptcpify__destroy(mptcpify_skel);
        return err;
    }

    send_byte(client_fd);

    err = verify_mptcpify(server_fd, client_fd);

    close(client_fd);
    close(server_fd);
    mptcpify__destroy(mptcpify_skel);
    err
}

unsafe fn test_mptcpify() {
    let mut netns: *mut netns_obj = ptr::null_mut();
    let cgroup_fd: c_int;

    cgroup_fd = test__join_cgroup(cstr(b"/mptcpify\0"));
    if !ASSERT_GE(cgroup_fd, 0, cstr(b"test__join_cgroup\0")) {
        return;
    }

    netns = netns_new(cstr(NS_TEST), true);
    if !ASSERT_OK_PTR(netns, cstr(b"netns_new\0")) {
        close(cgroup_fd);
        return;
    }

    ASSERT_OK(run_mptcpify(cgroup_fd, SOCK_STREAM), cstr(b"run_mptcpify\0"));
    /* userspace sets flags such as SOCK_CLOEXEC together with the type;
     * the BPF prog must still upgrade the socket to MPTCP. See
     * update_socket_protocol() in net/socket.c, which runs before the
     * type is masked with SOCK_TYPE_MASK.
     */
    ASSERT_OK(
        run_mptcpify(cgroup_fd, SOCK_STREAM | SOCK_CLOEXEC),
        cstr(b"run_mptcpify_cloexec\0"),
    );

    netns_free(netns);
    close(cgroup_fd);
}

unsafe fn endpoint_init(flags: *mut c_char) -> c_int {
    SYS(
        cstr(b"fail\0"),
        cstr(b"ip -net %s link add veth1 type veth peer name veth2\0"),
        cstr(NS_TEST),
    );
    SYS(
        cstr(b"fail\0"),
        cstr(b"ip -net %s addr add %s/24 dev veth1\0"),
        cstr(NS_TEST),
        cstr(ADDR_1),
    );
    SYS(
        cstr(b"fail\0"),
        cstr(b"ip -net %s link set dev veth1 up\0"),
        cstr(NS_TEST),
    );
    SYS(
        cstr(b"fail\0"),
        cstr(b"ip -net %s addr add %s/24 dev veth2\0"),
        cstr(NS_TEST),
        cstr(ADDR_2),
    );
    SYS(
        cstr(b"fail\0"),
        cstr(b"ip -net %s link set dev veth2 up\0"),
        cstr(NS_TEST),
    );
    if SYS_NOFAIL(
        cstr(b"ip -net %s mptcp endpoint add %s %s\0"),
        cstr(NS_TEST),
        cstr(ADDR_2),
        flags,
    ) != 0
    {
        printf(cstr(b"'ip mptcp' not supported, skip this test.\n\0"));
        test__skip();
        return -1;
    }

    0
}

unsafe fn wait_for_new_subflows(fd: c_int) {
    let mut len: socklen_t;
    let mut subflows: u8_ = 0;
    let mut err: c_int;
    let mut i: c_int;

    len = size_of::<u8_>() as socklen_t;
    /* Wait max 5 sec for new subflows to be created */
    i = 0;
    while i < 50 {
        err = getsockopt(
            fd,
            SOL_MPTCP,
            MPTCP_INFO,
            &mut subflows as *mut _ as *mut c_void,
            &mut len,
        );
        if err == 0 && subflows > 0 {
            break;
        }

        usleep(100000); /* 0.1s */
        i += 1;
    }
}

unsafe fn run_subflow() {
    let server_fd: c_int;
    let client_fd: c_int;
    let mut err: c_int;
    let mut new = [0 as c_char; TCP_CA_NAME_MAX];
    let mut cc = [0 as c_char; TCP_CA_NAME_MAX];
    let mut mark: c_uint = 0;
    let mut len: socklen_t;

    server_fd = start_mptcp_server(AF_INET, cstr(ADDR_1), PORT_1, 0);
    if !ASSERT_OK_FD(server_fd, cstr(b"start_mptcp_server\0")) {
        return;
    }

    client_fd = connect_to_fd(server_fd, 0);
    if !ASSERT_OK_FD(client_fd, cstr(b"connect_to_fd\0")) {
        close(server_fd);
        return;
    }

    send_byte(client_fd);
    wait_for_new_subflows(client_fd);

    len = size_of::<c_uint>() as socklen_t;
    err = getsockopt(
        client_fd,
        SOL_SOCKET,
        SO_MARK,
        &mut mark as *mut _ as *mut c_void,
        &mut len,
    );
    if ASSERT_OK(err, cstr(b"getsockopt(client_fd, SO_MARK)\0")) {
        ASSERT_EQ(mark, 0u32, cstr(b"mark\0"));
    }

    len = size_of::<[c_char; TCP_CA_NAME_MAX]>() as socklen_t;
    err = getsockopt(
        client_fd,
        SOL_TCP,
        TCP_CONGESTION,
        new.as_mut_ptr() as *mut c_void,
        &mut len,
    );
    if ASSERT_OK(err, cstr(b"getsockopt(client_fd, TCP_CONGESTION)\0")) {
        get_msk_ca_name(cc.as_mut_ptr());
        ASSERT_STREQ(new.as_ptr(), cc.as_ptr(), cstr(b"cc\0"));
    }

    close(client_fd);
    close(server_fd);
}

unsafe fn test_subflow() {
    let skel: *mut mptcp_subflow;
    let netns: *mut netns_obj;
    let cgroup_fd: c_int;

    cgroup_fd = test__join_cgroup(cstr(b"/mptcp_subflow\0"));
    if !ASSERT_OK_FD(cgroup_fd, cstr(b"join_cgroup: mptcp_subflow\0")) {
        return;
    }

    skel = mptcp_subflow__open_and_load();
    if !ASSERT_OK_PTR(skel, cstr(b"skel_open_load: mptcp_subflow\0")) {
        close(cgroup_fd);
        return;
    }

    (*(*skel).bss).pid = getpid();

    (*skel).links.mptcp_subflow =
        bpf_program__attach_cgroup((*skel).progs.mptcp_subflow, cgroup_fd);
    if !ASSERT_OK_PTR(
        (*skel).links.mptcp_subflow,
        cstr(b"attach mptcp_subflow\0"),
    ) {
        mptcp_subflow__destroy(skel);
        close(cgroup_fd);
        return;
    }

    (*skel).links._getsockopt_subflow =
        bpf_program__attach_cgroup((*skel).progs._getsockopt_subflow, cgroup_fd);
    if !ASSERT_OK_PTR(
        (*skel).links._getsockopt_subflow,
        cstr(b"attach _getsockopt_subflow\0"),
    ) {
        mptcp_subflow__destroy(skel);
        close(cgroup_fd);
        return;
    }

    netns = netns_new(cstr(NS_TEST), true);
    if !ASSERT_OK_PTR(netns, cstr(b"netns_new: mptcp_subflow\0")) {
        mptcp_subflow__destroy(skel);
        close(cgroup_fd);
        return;
    }

    if endpoint_init(cstr(b"subflow\0") as *mut c_char) >= 0 {
        run_subflow();
    }

    netns_free(netns);
    mptcp_subflow__destroy(skel);
    close(cgroup_fd);
}

/* Test sockmap on MPTCP server handling non-mp-capable clients. */
unsafe fn test_sockmap_with_mptcp_fallback(skel: *mut mptcp_sockmap) {
    let mut listen_fd: c_int = -1;
    let mut client_fd1: c_int = -1;
    let mut client_fd2: c_int = -1;
    let mut server_fd1: c_int = -1;
    let mut server_fd2: c_int = -1;
    let sent: isize;
    let recvd: isize;
    let snd = *b"123456789";
    let mut rcv = [0 as c_char; 10];

    /* start server with MPTCP enabled */
    listen_fd = start_mptcp_server(AF_INET, ptr::null(), 0, 0);
    if !ASSERT_OK_FD(listen_fd, cstr(b"sockmap-fb:start_mptcp_server\0")) {
        return;
    }

    (*(*skel).bss).trace_port = ntohs(get_socket_local_port(listen_fd));
    (*(*skel).bss).sk_index = 0;
    /* create client without MPTCP enabled */
    client_fd1 = connect_to_fd_opts(listen_fd, ptr::null());
    if !ASSERT_OK_FD(client_fd1, cstr(b"sockmap-fb:connect_to_fd\0")) {
        close(listen_fd);
        return;
    }

    server_fd1 = accept(listen_fd, ptr::null_mut(), ptr::null_mut());
    (*(*skel).bss).sk_index = 1;
    client_fd2 = connect_to_fd_opts(listen_fd, ptr::null());
    if !ASSERT_OK_FD(client_fd2, cstr(b"sockmap-fb:connect_to_fd\0")) {
        if client_fd1 >= 0 {
            close(client_fd1);
        }
        if server_fd1 >= 0 {
            close(server_fd1);
        }
        close(listen_fd);
        return;
    }

    server_fd2 = accept(listen_fd, ptr::null_mut(), ptr::null_mut());
    /* test normal redirect behavior: data sent by client_fd1 can be
     * received by client_fd2
     */
    (*(*skel).bss).redirect_idx = 1;
    sent = send(client_fd1, snd.as_ptr() as *const c_void, snd.len(), 0);
    if ASSERT_EQ(sent, snd.len() as isize, cstr(b"sockmap-fb:send(client_fd1)\0")) {
        /* try to recv more bytes to avoid truncation check */
        recvd = recv(client_fd2, rcv.as_mut_ptr() as *mut c_void, rcv.len(), 0);
        ASSERT_EQ(
            recvd,
            snd.len() as isize,
            cstr(b"sockmap-fb:recv(client_fd2)\0"),
        );
    }

    if client_fd1 >= 0 {
        close(client_fd1);
    }
    if client_fd2 >= 0 {
        close(client_fd2);
    }
    if server_fd1 >= 0 {
        close(server_fd1);
    }
    if server_fd2 >= 0 {
        close(server_fd2);
    }
    close(listen_fd);
}

/* Test sockmap rejection of MPTCP sockets - both server and client sides. */
unsafe fn test_sockmap_reject_mptcp(skel: *mut mptcp_sockmap) {
    let mut listen_fd: c_int = -1;
    let mut server_fd: c_int = -1;
    let mut client_fd1: c_int = -1;
    let mut err: c_int;
    let zero: c_int = 0;

    /* start server with MPTCP enabled */
    listen_fd = start_mptcp_server(AF_INET, ptr::null(), 0, 0);
    if !ASSERT_OK_FD(listen_fd, cstr(b"start_mptcp_server\0")) {
        return;
    }

    (*(*skel).bss).trace_port = ntohs(get_socket_local_port(listen_fd));
    (*(*skel).bss).sk_index = 0;
    /* create client with MPTCP enabled */
    client_fd1 = connect_to_fd(listen_fd, 0);
    if !ASSERT_OK_FD(client_fd1, cstr(b"connect_to_fd client_fd1\0")) {
        close(listen_fd);
        return;
    }

    /* bpf_sock_map_update() called from sockops should reject MPTCP sk */
    if !ASSERT_EQ((*(*skel).bss).helper_ret, -EOPNOTSUPP, cstr(b"should reject\0")) {
        close(client_fd1);
        close(listen_fd);
        return;
    }

    server_fd = accept(listen_fd, ptr::null_mut(), ptr::null_mut());
    err = bpf_map_update_elem(
        bpf_map__fd((*skel).maps.sock_map),
        &zero as *const _ as *const c_void,
        &server_fd as *const _ as *const c_void,
        BPF_NOEXIST,
    );
    if ASSERT_EQ(err, -EOPNOTSUPP, cstr(b"server should be disallowed\0")) {
        /* MPTCP client should also be disallowed */
        err = bpf_map_update_elem(
            bpf_map__fd((*skel).maps.sock_map),
            &zero as *const _ as *const c_void,
            &client_fd1 as *const _ as *const c_void,
            BPF_NOEXIST,
        );
        ASSERT_EQ(err, -EOPNOTSUPP, cstr(b"client should be disallowed\0"));
    }

    if client_fd1 >= 0 {
        close(client_fd1);
    }
    if server_fd >= 0 {
        close(server_fd);
    }
    close(listen_fd);
}

unsafe fn test_mptcp_sockmap() {
    let skel: *mut mptcp_sockmap;
    let netns: *mut netns_obj;
    let cgroup_fd: c_int;
    let err: c_int;

    cgroup_fd = test__join_cgroup(cstr(b"/mptcp_sockmap\0"));
    if !ASSERT_OK_FD(cgroup_fd, cstr(b"join_cgroup: mptcp_sockmap\0")) {
        return;
    }

    skel = mptcp_sockmap__open_and_load();
    if !ASSERT_OK_PTR(skel, cstr(b"skel_open_load: mptcp_sockmap\0")) {
        close(cgroup_fd);
        return;
    }

    (*skel).links.mptcp_sockmap_inject =
        bpf_program__attach_cgroup((*skel).progs.mptcp_sockmap_inject, cgroup_fd);
    if !ASSERT_OK_PTR((*skel).links.mptcp_sockmap_inject, cstr(b"attach sockmap\0")) {
        mptcp_sockmap__destroy(skel);
        close(cgroup_fd);
        return;
    }

    err = bpf_prog_attach(
        bpf_program__fd((*skel).progs.mptcp_sockmap_redirect),
        bpf_map__fd((*skel).maps.sock_map),
        BPF_SK_SKB_STREAM_VERDICT,
        0,
    );
    if !ASSERT_OK(err, cstr(b"bpf_prog_attach stream verdict\0")) {
        mptcp_sockmap__destroy(skel);
        close(cgroup_fd);
        return;
    }

    netns = netns_new(cstr(NS_TEST), true);
    if !ASSERT_OK_PTR(netns, cstr(b"netns_new: mptcp_sockmap\0")) {
        mptcp_sockmap__destroy(skel);
        close(cgroup_fd);
        return;
    }

    if endpoint_init(cstr(b"subflow\0") as *mut c_char) >= 0 {
        test_sockmap_with_mptcp_fallback(skel);
        test_sockmap_reject_mptcp(skel);
    }

    netns_free(netns);
    mptcp_sockmap__destroy(skel);
    close(cgroup_fd);
}

#[no_mangle]
pub unsafe extern "C" fn test_mptcp() {
    if test__start_subtest(cstr(b"base\0")) {
        test_base();
    }
    if test__start_subtest(cstr(b"mptcpify\0")) {
        test_mptcpify();
    }
    if test__start_subtest(cstr(b"subflow\0")) {
        test_subflow();
    }
    if test__start_subtest(cstr(b"sockmap\0")) {
        test_mptcp_sockmap();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
