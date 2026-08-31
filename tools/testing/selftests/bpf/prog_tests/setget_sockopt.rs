// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) Meta Platforms, Inc. and affiliates. */

/* Translated from C. Header dependencies:
 * sched.h, linux/socket.h, linux/tls.h, net/if.h,
 * test_progs.h, cgroup_helpers.h, network_helpers.h,
 * setget_sockopt.skel.h.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{addr_of_mut, null_mut};

const CG_NAME: *const c_char = b"/setget-sockopt-test\0".as_ptr() as *const c_char;

static addr4_str: &[u8; 10] = b"127.0.0.1\0";
static addr6_str: &[u8; 4] = b"::1\0";
static mut skel: *mut setget_sockopt = null_mut();
static mut cg_fd: c_int = 0;

const CLONE_NEWNET: c_int = 0x40000000;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const IPPROTO_TCP: c_int = 6;
const IPPROTO_IPV6: c_int = 41;
const SOL_TCP: c_int = 6;
const SOL_IP: c_int = 0;
const IP_TOS: c_int = 1;
const IPV6_V6ONLY: c_int = 26;
const TCP_ULP: c_int = 31;
const TCP_BPF_SOCK_OPS_CB_FLAGS: c_int = 39;
const BPF_SOCK_OPS_STATE_CB_FLAG: c_int = 1 << 4;
const SOL_TLS: c_int = 282;
const TLS_TX: c_int = 1;
const TLS_RX: c_int = 2;
const TLS_1_2_VERSION: u16 = 0x0303;
const TLS_CIPHER_AES_GCM_128: u16 = 51;

type socklen_t = u32;
type ssize_t = isize;

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[repr(C)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: u16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}

#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: u16,
    pub __ss_padding: [u8; 118],
    pub __ss_align: u64,
}

#[repr(C)]
pub struct tls_crypto_info {
    pub version: u16,
    pub cipher_type: u16,
}

#[repr(C)]
pub struct tls12_crypto_info_aes_gcm_128 {
    pub info: tls_crypto_info,
    pub iv: [u8; 8],
    pub key: [u8; 16],
    pub salt: [u8; 4],
    pub rec_seq: [u8; 8],
}

#[repr(C)]
pub struct setget_sockopt__bss {
    pub nr_listen: c_int,
    pub nr_connect: c_int,
    pub nr_active: c_int,
    pub nr_passive: c_int,
    pub nr_socket_post_create: c_int,
    pub nr_binddev: c_int,
    pub nr_fin_wait1: c_int,
    pub v4mapped_v6_ip_tos_enable: c_int,
    pub v4mapped_v6_ip_tos_ret: c_int,
    pub v4mapped_v6_ip_tos_val: c_int,
    pub v4mapped_v6_ip_tos_cnt: c_int,
}

#[repr(C)]
pub struct setget_sockopt__rodata {
    pub veth: [c_char; 16],
    pub veth_ifindex: c_uint,
}

#[repr(C)]
pub struct setget_sockopt__progs {
    pub skops_sockopt: *mut bpf_program,
    pub socket_post_create: *mut bpf_program,
    pub _getsockopt: *mut bpf_program,
}

#[repr(C)]
pub struct setget_sockopt__links {
    pub skops_sockopt: *mut bpf_link,
    pub socket_post_create: *mut bpf_link,
}

#[repr(C)]
pub struct setget_sockopt {
    pub bss: *mut setget_sockopt__bss,
    pub rodata: *mut setget_sockopt__rodata,
    pub progs: setget_sockopt__progs,
    pub links: setget_sockopt__links,
}

unsafe extern "C" {
    fn unshare(flags: c_int) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn close(fd: c_int) -> c_int;
    fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn setsockopt(
        sockfd: c_int,
        level: c_int,
        optname: c_int,
        optval: *const c_void,
        optlen: socklen_t,
    ) -> c_int;
    fn getsockopt(
        sockfd: c_int,
        level: c_int,
        optname: c_int,
        optval: *mut c_void,
        optlen: *mut socklen_t,
    ) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> ssize_t;
    fn getsockname(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn connect(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;

    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(res: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(res: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;

    fn start_server(
        family: c_int,
        type_: c_int,
        addr_str: *const c_char,
        port: c_int,
        timeout_ms: c_int,
    ) -> c_int;
    fn connect_to_fd(server_fd: c_int, timeout_ms: c_int) -> c_int;
    fn settimeo(fd: c_int, timeout_ms: c_int) -> c_int;
    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn bpf_program__attach_cgroup(prog: *mut bpf_program, cgroup_fd: c_int) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn setget_sockopt__open() -> *mut setget_sockopt;
    fn setget_sockopt__load(obj: *mut setget_sockopt) -> c_int;
    fn setget_sockopt__destroy(obj: *mut setget_sockopt);
}

unsafe fn create_netns() -> c_int {
    if !ASSERT_OK(unshare(CLONE_NEWNET), b"create netns\0".as_ptr() as *const c_char) {
        return -1;
    }

    if !ASSERT_OK(
        system(b"ip link set dev lo up\0".as_ptr() as *const c_char),
        b"set lo up\0".as_ptr() as *const c_char,
    ) {
        return -1;
    }

    if !ASSERT_OK(
        system(
            b"ip link add dev binddevtest1 type veth peer name binddevtest2\0".as_ptr()
                as *const c_char,
        ),
        b"add veth\0".as_ptr() as *const c_char,
    ) {
        return -1;
    }

    if !ASSERT_OK(
        system(b"ip link set dev binddevtest1 up\0".as_ptr() as *const c_char),
        b"bring veth up\0".as_ptr() as *const c_char,
    ) {
        return -1;
    }

    0
}

unsafe fn test_tcp(family: c_int) {
    let bss: *mut setget_sockopt__bss = (*skel).bss;
    let sfd: c_int;
    let cfd: c_int;

    memset(bss as *mut c_void, 0, size_of::<setget_sockopt__bss>());

    sfd = start_server(
        family,
        SOCK_STREAM,
        if family == AF_INET6 {
            addr6_str.as_ptr() as *const c_char
        } else {
            addr4_str.as_ptr() as *const c_char
        },
        0,
        0,
    );
    if !ASSERT_GE(sfd, 0, b"start_server\0".as_ptr() as *const c_char) {
        return;
    }

    cfd = connect_to_fd(sfd, 0);
    if !ASSERT_GE(cfd, 0, b"connect_to_fd_server\0".as_ptr() as *const c_char) {
        close(sfd);
        return;
    }
    close(sfd);
    close(cfd);

    ASSERT_EQ((*bss).nr_listen, 1, b"nr_listen\0".as_ptr() as *const c_char);
    ASSERT_EQ((*bss).nr_connect, 1, b"nr_connect\0".as_ptr() as *const c_char);
    ASSERT_EQ((*bss).nr_active, 1, b"nr_active\0".as_ptr() as *const c_char);
    ASSERT_EQ((*bss).nr_passive, 1, b"nr_passive\0".as_ptr() as *const c_char);
    ASSERT_EQ(
        (*bss).nr_socket_post_create,
        2,
        b"nr_socket_post_create\0".as_ptr() as *const c_char,
    );
    ASSERT_EQ((*bss).nr_binddev, 2, b"nr_bind\0".as_ptr() as *const c_char);
}

unsafe fn test_udp(family: c_int) {
    let bss: *mut setget_sockopt__bss = (*skel).bss;
    let sfd: c_int;

    memset(bss as *mut c_void, 0, size_of::<setget_sockopt__bss>());

    sfd = start_server(
        family,
        SOCK_DGRAM,
        if family == AF_INET6 {
            addr6_str.as_ptr() as *const c_char
        } else {
            addr4_str.as_ptr() as *const c_char
        },
        0,
        0,
    );
    if !ASSERT_GE(sfd, 0, b"start_server\0".as_ptr() as *const c_char) {
        return;
    }
    close(sfd);

    ASSERT_GE(
        (*bss).nr_socket_post_create,
        1,
        b"nr_socket_post_create\0".as_ptr() as *const c_char,
    );
    ASSERT_EQ((*bss).nr_binddev, 1, b"nr_bind\0".as_ptr() as *const c_char);
}

unsafe fn test_ktls(family: c_int) {
    let mut aes128: tls12_crypto_info_aes_gcm_128;
    let bss: *mut setget_sockopt__bss = (*skel).bss;
    let mut cfd: c_int = -1;
    let mut sfd: c_int = -1;
    let mut fd: c_int = -1;
    let mut ret: c_int;
    let mut buf: c_char = 0;

    memset(bss as *mut c_void, 0, size_of::<setget_sockopt__bss>());

    sfd = start_server(
        family,
        SOCK_STREAM,
        if family == AF_INET6 {
            addr6_str.as_ptr() as *const c_char
        } else {
            addr4_str.as_ptr() as *const c_char
        },
        0,
        0,
    );
    if !ASSERT_GE(sfd, 0, b"start_server\0".as_ptr() as *const c_char) {
        return;
    }
    fd = connect_to_fd(sfd, 0);
    if !ASSERT_GE(fd, 0, b"connect_to_fd\0".as_ptr() as *const c_char) {
        close(fd);
        close(cfd);
        close(sfd);
        return;
    }

    cfd = accept(sfd, null_mut(), null_mut());
    if !ASSERT_GE(cfd, 0, b"accept\0".as_ptr() as *const c_char) {
        close(fd);
        close(cfd);
        close(sfd);
        return;
    }

    close(sfd);
    sfd = -1;

    /* Setup KTLS */
    ret = setsockopt(
        fd,
        IPPROTO_TCP,
        TCP_ULP,
        b"tls\0".as_ptr() as *const c_void,
        size_of::<[u8; 4]>() as socklen_t,
    );
    if !ASSERT_OK(ret, b"setsockopt\0".as_ptr() as *const c_char) {
        close(fd);
        close(cfd);
        close(sfd);
        return;
    }
    ret = setsockopt(
        cfd,
        IPPROTO_TCP,
        TCP_ULP,
        b"tls\0".as_ptr() as *const c_void,
        size_of::<[u8; 4]>() as socklen_t,
    );
    if !ASSERT_OK(ret, b"setsockopt\0".as_ptr() as *const c_char) {
        close(fd);
        close(cfd);
        close(sfd);
        return;
    }

    aes128 = zeroed();
    aes128.info.version = TLS_1_2_VERSION;
    aes128.info.cipher_type = TLS_CIPHER_AES_GCM_128;

    ret = setsockopt(
        fd,
        SOL_TLS,
        TLS_TX,
        &aes128 as *const _ as *const c_void,
        size_of::<tls12_crypto_info_aes_gcm_128>() as socklen_t,
    );
    if !ASSERT_OK(ret, b"setsockopt\0".as_ptr() as *const c_char) {
        close(fd);
        close(cfd);
        close(sfd);
        return;
    }

    ret = setsockopt(
        cfd,
        SOL_TLS,
        TLS_RX,
        &aes128 as *const _ as *const c_void,
        size_of::<tls12_crypto_info_aes_gcm_128>() as socklen_t,
    );
    if !ASSERT_OK(ret, b"setsockopt\0".as_ptr() as *const c_char) {
        close(fd);
        close(cfd);
        close(sfd);
        return;
    }

    /* KTLS is enabled */

    close(fd);
    /* At this point, the cfd socket is at the CLOSE_WAIT state
     * and still run TLS protocol.  The test for
     * BPF_TCP_CLOSE_WAIT should be run at this point.
     */
    ret = read(
        cfd,
        addr_of_mut!(buf) as *mut c_void,
        size_of::<c_char>(),
    ) as c_int;
    ASSERT_EQ(ret, 0, b"read\0".as_ptr() as *const c_char);
    close(cfd);

    ASSERT_EQ((*bss).nr_listen, 1, b"nr_listen\0".as_ptr() as *const c_char);
    ASSERT_EQ((*bss).nr_connect, 1, b"nr_connect\0".as_ptr() as *const c_char);
    ASSERT_EQ((*bss).nr_active, 1, b"nr_active\0".as_ptr() as *const c_char);
    ASSERT_EQ((*bss).nr_passive, 1, b"nr_passive\0".as_ptr() as *const c_char);
    ASSERT_EQ(
        (*bss).nr_socket_post_create,
        2,
        b"nr_socket_post_create\0".as_ptr() as *const c_char,
    );
    ASSERT_EQ((*bss).nr_binddev, 2, b"nr_bind\0".as_ptr() as *const c_char);
    ASSERT_EQ((*bss).nr_fin_wait1, 1, b"nr_fin_wait1\0".as_ptr() as *const c_char);
}

unsafe fn test_nonstandard_opt(family: c_int) {
    let bss: *mut setget_sockopt__bss = (*skel).bss;
    let mut getsockopt_link: *mut bpf_link = null_mut();
    let mut sfd: c_int = -1;
    let mut fd: c_int = -1;
    let mut cfd: c_int = -1;
    let mut flags: c_int = 0;
    let mut flagslen: socklen_t = size_of::<c_int>() as socklen_t;

    memset(bss as *mut c_void, 0, size_of::<setget_sockopt__bss>());

    sfd = start_server(
        family,
        SOCK_STREAM,
        if family == AF_INET6 {
            addr6_str.as_ptr() as *const c_char
        } else {
            addr4_str.as_ptr() as *const c_char
        },
        0,
        0,
    );
    if !ASSERT_GE(sfd, 0, b"start_server\0".as_ptr() as *const c_char) {
        return;
    }

    fd = connect_to_fd(sfd, 0);
    if !ASSERT_GE(fd, 0, b"connect_to_fd_server\0".as_ptr() as *const c_char) {
        close(sfd);
        if fd != -1 {
            close(fd);
        }
        if cfd != -1 {
            close(cfd);
        }
        bpf_link__destroy(getsockopt_link);
        return;
    }

    /* cgroup/getsockopt prog will intercept getsockopt() below and
     * retrieve the tcp socket bpf_sock_ops_cb_flags value for the
     * accept()ed socket; this was set earlier in the passive established
     * callback for the accept()ed socket via bpf_setsockopt().
     */
    getsockopt_link = bpf_program__attach_cgroup((*skel).progs._getsockopt, cg_fd);
    if !ASSERT_OK_PTR(
        getsockopt_link as *mut c_void,
        b"getsockopt prog\0".as_ptr() as *const c_char,
    ) {
        close(sfd);
        if fd != -1 {
            close(fd);
        }
        if cfd != -1 {
            close(cfd);
        }
        bpf_link__destroy(getsockopt_link);
        return;
    }

    cfd = accept(sfd, null_mut(), null_mut());
    if !ASSERT_GE(cfd, 0, b"accept\0".as_ptr() as *const c_char) {
        close(sfd);
        if fd != -1 {
            close(fd);
        }
        if cfd != -1 {
            close(cfd);
        }
        bpf_link__destroy(getsockopt_link);
        return;
    }

    if !ASSERT_OK(
        getsockopt(
            cfd,
            SOL_TCP,
            TCP_BPF_SOCK_OPS_CB_FLAGS,
            addr_of_mut!(flags) as *mut c_void,
            addr_of_mut!(flagslen),
        ),
        b"getsockopt_flags\0".as_ptr() as *const c_char,
    ) {
        close(sfd);
        if fd != -1 {
            close(fd);
        }
        if cfd != -1 {
            close(cfd);
        }
        bpf_link__destroy(getsockopt_link);
        return;
    }
    ASSERT_EQ(
        flags & BPF_SOCK_OPS_STATE_CB_FLAG,
        BPF_SOCK_OPS_STATE_CB_FLAG,
        b"cb_flags_set\0".as_ptr() as *const c_char,
    );
    close(sfd);
    if fd != -1 {
        close(fd);
    }
    if cfd != -1 {
        close(cfd);
    }
    bpf_link__destroy(getsockopt_link);
}

unsafe fn connect_to_v4mapped_v6_fd(server_fd: c_int) -> c_int {
    let mut addr: sockaddr_storage = zeroed();
    let addr4: *mut sockaddr_in = addr_of_mut!(addr) as *mut sockaddr_in;
    let mut addrlen: socklen_t = size_of::<sockaddr_storage>() as socklen_t;
    let mut addr6: sockaddr_in6 = zeroed();
    let mut fd: c_int = -1;
    let mut v6only: c_int = 0;
    let mut err: c_int;

    err = getsockname(
        server_fd,
        addr_of_mut!(addr) as *mut sockaddr,
        addr_of_mut!(addrlen),
    );
    if !ASSERT_OK(err, b"getsockname\0".as_ptr() as *const c_char) {
        return -1;
    }

    fd = socket(AF_INET6, SOCK_STREAM, 0);
    if !ASSERT_GE(fd, 0, b"socket\0".as_ptr() as *const c_char) {
        return -1;
    }

    err = settimeo(fd, 0);
    if !ASSERT_OK(err, b"settimeo\0".as_ptr() as *const c_char) {
        close(fd);
        return -1;
    }

    err = setsockopt(
        fd,
        IPPROTO_IPV6,
        IPV6_V6ONLY,
        addr_of_mut!(v6only) as *const c_void,
        size_of::<c_int>() as socklen_t,
    );
    if !ASSERT_OK(err, b"clear_v6only\0".as_ptr() as *const c_char) {
        close(fd);
        return -1;
    }

    addr6.sin6_family = AF_INET6 as u16;
    addr6.sin6_port = (*addr4).sin_port;
    addr6.sin6_addr.s6_addr[10] = 0xff;
    addr6.sin6_addr.s6_addr[11] = 0xff;
    memcpy(
        addr_of_mut!(addr6.sin6_addr.s6_addr[12]) as *mut c_void,
        addr_of_mut!((*addr4).sin_addr) as *const c_void,
        size_of::<in_addr>(),
    );

    err = connect(
        fd,
        addr_of_mut!(addr6) as *const sockaddr,
        size_of::<sockaddr_in6>() as socklen_t,
    );
    if !ASSERT_OK(err, b"connect\0".as_ptr() as *const c_char) {
        close(fd);
        return -1;
    }

    fd
}

unsafe fn test_v4mapped_v6_ip_tos() {
    let bss: *mut setget_sockopt__bss = (*skel).bss;
    let mut sfd: c_int = -1;
    let mut fd: c_int = -1;
    let mut got: c_int = 0;
    let exp: c_int = 0x1c;
    let mut optlen: socklen_t;

    memset(bss as *mut c_void, 0, size_of::<setget_sockopt__bss>());
    (*bss).v4mapped_v6_ip_tos_enable = 1;
    (*bss).v4mapped_v6_ip_tos_ret = -1;
    (*bss).v4mapped_v6_ip_tos_val = exp;

    sfd = start_server(AF_INET, SOCK_STREAM, addr4_str.as_ptr() as *const c_char, 0, 0);
    if !ASSERT_GE(sfd, 0, b"start_server\0".as_ptr() as *const c_char) {
        (*bss).v4mapped_v6_ip_tos_enable = 0;
        if fd >= 0 {
            close(fd);
        }
        if sfd >= 0 {
            close(sfd);
        }
        return;
    }

    fd = connect_to_v4mapped_v6_fd(sfd);
    if !ASSERT_GE(
        fd,
        0,
        b"connect_to_v4mapped_v6_fd\0".as_ptr() as *const c_char,
    ) {
        (*bss).v4mapped_v6_ip_tos_enable = 0;
        if fd >= 0 {
            close(fd);
        }
        if sfd >= 0 {
            close(sfd);
        }
        return;
    }

    ASSERT_GT(
        (*bss).v4mapped_v6_ip_tos_cnt,
        0,
        b"v4mapped_v6_ip_tos_cnt\0".as_ptr() as *const c_char,
    );
    ASSERT_EQ(
        (*bss).v4mapped_v6_ip_tos_ret,
        0,
        b"v4mapped_v6_ip_tos_ret\0".as_ptr() as *const c_char,
    );

    optlen = size_of::<c_int>() as socklen_t;
    if !ASSERT_OK(
        getsockopt(
            fd,
            SOL_IP,
            IP_TOS,
            addr_of_mut!(got) as *mut c_void,
            addr_of_mut!(optlen),
        ),
        b"getsockopt_ip_tos\0".as_ptr() as *const c_char,
    ) {
        (*bss).v4mapped_v6_ip_tos_enable = 0;
        if fd >= 0 {
            close(fd);
        }
        if sfd >= 0 {
            close(sfd);
        }
        return;
    }

    ASSERT_EQ(got, exp, b"ip_tos\0".as_ptr() as *const c_char);

    (*bss).v4mapped_v6_ip_tos_enable = 0;
    if fd >= 0 {
        close(fd);
    }
    if sfd >= 0 {
        close(sfd);
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_setget_sockopt() {
    cg_fd = test__join_cgroup(CG_NAME);
    if !ASSERT_OK_FD(cg_fd, b"join cgroup\0".as_ptr() as *const c_char) {
        return;
    }

    if create_netns() != 0 {
        setget_sockopt__destroy(skel);
        close(cg_fd);
        return;
    }

    skel = setget_sockopt__open();
    if !ASSERT_OK_PTR(skel as *mut c_void, b"open skel\0".as_ptr() as *const c_char) {
        setget_sockopt__destroy(skel);
        close(cg_fd);
        return;
    }

    strscpy(
        (*(*skel).rodata).veth.as_mut_ptr(),
        b"binddevtest1\0".as_ptr() as *const c_char,
    );
    (*(*skel).rodata).veth_ifindex = if_nametoindex(b"binddevtest1\0".as_ptr() as *const c_char);
    if !ASSERT_GT(
        (*(*skel).rodata).veth_ifindex as c_int,
        0,
        b"if_nametoindex\0".as_ptr() as *const c_char,
    ) {
        setget_sockopt__destroy(skel);
        close(cg_fd);
        return;
    }

    if !ASSERT_OK(setget_sockopt__load(skel), b"load skel\0".as_ptr() as *const c_char) {
        setget_sockopt__destroy(skel);
        close(cg_fd);
        return;
    }

    (*skel).links.skops_sockopt =
        bpf_program__attach_cgroup((*skel).progs.skops_sockopt, cg_fd);
    if !ASSERT_OK_PTR(
        (*skel).links.skops_sockopt as *mut c_void,
        b"attach cgroup\0".as_ptr() as *const c_char,
    ) {
        setget_sockopt__destroy(skel);
        close(cg_fd);
        return;
    }

    (*skel).links.socket_post_create =
        bpf_program__attach_cgroup((*skel).progs.socket_post_create, cg_fd);
    if !ASSERT_OK_PTR(
        (*skel).links.socket_post_create as *mut c_void,
        b"attach_cgroup\0".as_ptr() as *const c_char,
    ) {
        setget_sockopt__destroy(skel);
        close(cg_fd);
        return;
    }

    test_tcp(AF_INET6);
    test_tcp(AF_INET);
    test_udp(AF_INET6);
    test_udp(AF_INET);
    test_ktls(AF_INET6);
    test_ktls(AF_INET);
    test_nonstandard_opt(AF_INET);
    test_nonstandard_opt(AF_INET6);
    test_v4mapped_v6_ip_tos();

    setget_sockopt__destroy(skel);
    close(cg_fd);
}
