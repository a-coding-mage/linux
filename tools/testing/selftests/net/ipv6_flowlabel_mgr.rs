// SPDX-License-Identifier: GPL-2.0
/* Test IPV6_FLOWINFO_MGR */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type socklen_t = c_uint;
type pid_t = c_int;
type ssize_t = isize;

const AF_LOCAL: c_int = 1;
const AF_INET6: c_int = 10;
const PF_INET6: c_int = AF_INET6;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const SOL_SOCKET: c_int = 1;
const SOL_IPV6: c_int = 41;
const SO_REUSEADDR: c_int = 2;
const O_WRONLY: c_int = 1;
const SIOCGIFFLAGS: c_ulong = 0x8913;
const SIOCSIFFLAGS: c_ulong = 0x8914;
const IFF_UP: c_short = 0x1;
const CLONE_NEWNET: c_int = 0x40000000;

const ENOENT: c_int = 2;
const ESRCH: c_int = 3;
const EINVAL: c_int = 22;
const EEXIST: c_int = 17;
const EPERM: c_int = 1;

const KSFT_SKIP: c_int = 4;

type c_short = i16;
type uint8_t = u8;
type uint16_t = u16;
type uint32_t = u32;

/* uapi/glibc weirdness may leave this undefined */
const IPV6_FLOWLABEL_MGR: c_int = 32;
const IPV6_FLOWINFO_SEND: c_int = 33;

/* from net/ipv6/ip6_flowlabel.c */
const FL_MIN_LINGER: c_uint = 6;

const IPV6_FL_A_GET: uint8_t = 0;
const IPV6_FL_A_PUT: uint8_t = 1;
const IPV6_FL_A_RENEW: uint8_t = 2;
const IPV6_FL_F_CREATE: uint16_t = 1;
const IPV6_FL_F_EXCL: uint16_t = 2;
const IPV6_FL_F_REFLECT: uint16_t = 4;
const IPV6_FL_F_REMOTE: uint16_t = 8;
const IPV6_FL_S_NONE: uint8_t = 0;
const IPV6_FL_S_EXCL: uint8_t = 1;
const IPV6_FL_S_PROCESS: uint8_t = 2;
const IPV6_FL_S_USER: uint8_t = 3;
const IPV6_FL_S_ANY: uint8_t = 255;

#[repr(C)]
#[derive(Copy, Clone)]
struct in6_addr {
    s6_addr: [u8; 16],
}

const IN6ADDR_LOOPBACK_INIT: in6_addr = in6_addr {
    s6_addr: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
};

#[repr(C)]
#[derive(Copy, Clone)]
struct sockaddr_in6 {
    sin6_family: u16,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: in6_addr,
    sin6_scope_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct in6_flowlabel_req {
    flr_dst: in6_addr,
    flr_label: u32,
    flr_action: u8,
    flr_share: u8,
    flr_flags: u16,
    flr_expires: u16,
    flr_linger: u16,
    __flr_pad: u32,
}

#[repr(C)]
union ifr_ifru {
    ifru_flags: c_short,
}

#[repr(C)]
struct ifreq {
    ifr_name: [c_char; 16],
    ifr_ifru: ifr_ifru,
}

extern "C" {
    static mut errno: c_int;

    fn htonl(hostlong: u32) -> u32;
    fn htons(hostshort: u16) -> u16;
    fn ntohl(netlong: u32) -> u32;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn getsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *mut c_void,
        option_len: *mut socklen_t,
    ) -> c_int;
    fn bind(socket: c_int, address: *const c_void, address_len: socklen_t) -> c_int;
    fn listen(socket: c_int, backlog: c_int) -> c_int;
    fn connect(socket: c_int, address: *const c_void, address_len: socklen_t) -> c_int;
    fn accept(socket: c_int, address: *mut c_void, address_len: *mut socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...);
    fn sleep(seconds: c_uint) -> c_uint;
    fn fork() -> pid_t;
    fn wait(wstatus: *mut c_int) -> pid_t;
    fn setuid(uid: c_uint) -> c_int;
    fn exit(status: c_int) -> !;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> ssize_t;
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! ASSERT_NE {
    ($left:expr, $right:expr) => {
        assert_ne!($left, $right)
    };
}

macro_rules! ASSERT_GE {
    ($left:expr, $right:expr) => {
        assert!($left >= $right)
    };
}

macro_rules! ASSERT_TRUE {
    ($expr:expr) => {
        assert!($expr)
    };
}

macro_rules! EXPECT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! EXPECT_TRUE {
    ($expr:expr) => {
        assert!($expr)
    };
}

macro_rules! TH_LOG {
    ($($arg:tt)*) => {};
}

macro_rules! SKIP {
    (return, $($arg:tt)*) => {
        return
    };
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn flowlabel_get(fd: c_int, label: uint32_t, share: uint8_t, flags: uint16_t) -> c_int {
    let mut req = in6_flowlabel_req {
        flr_action: IPV6_FL_A_GET,
        flr_label: htonl(label),
        flr_flags: flags,
        flr_share: share,
        flr_dst: in6_addr { s6_addr: [0; 16] },
        flr_expires: 0,
        flr_linger: 0,
        __flr_pad: 0,
    };

    /* do not pass IPV6_ADDR_ANY or IPV6_ADDR_MAPPED */
    req.flr_dst.s6_addr[0] = 0xfd;
    req.flr_dst.s6_addr[15] = 0x1;

    setsockopt(
        fd,
        SOL_IPV6,
        IPV6_FLOWLABEL_MGR,
        &req as *const _ as *const c_void,
        core::mem::size_of::<in6_flowlabel_req>() as socklen_t,
    )
}

unsafe fn flowlabel_put(fd: c_int, label: uint32_t) -> c_int {
    let req = in6_flowlabel_req {
        flr_action: IPV6_FL_A_PUT,
        flr_label: htonl(label),
        flr_dst: in6_addr { s6_addr: [0; 16] },
        flr_share: 0,
        flr_flags: 0,
        flr_expires: 0,
        flr_linger: 0,
        __flr_pad: 0,
    };

    setsockopt(
        fd,
        SOL_IPV6,
        IPV6_FLOWLABEL_MGR,
        &req as *const _ as *const c_void,
        core::mem::size_of::<in6_flowlabel_req>() as socklen_t,
    )
}

unsafe fn flowlabel_renew(
    fd: c_int,
    label: uint32_t,
    share: uint8_t,
    linger: uint16_t,
) -> c_int {
    let req = in6_flowlabel_req {
        flr_action: IPV6_FL_A_RENEW,
        flr_label: htonl(label),
        flr_share: share,
        flr_linger: linger,
        flr_dst: in6_addr { s6_addr: [0; 16] },
        flr_flags: 0,
        flr_expires: 0,
        __flr_pad: 0,
    };

    setsockopt(
        fd,
        SOL_IPV6,
        IPV6_FLOWLABEL_MGR,
        &req as *const _ as *const c_void,
        core::mem::size_of::<in6_flowlabel_req>() as socklen_t,
    )
}

unsafe fn loopback_addr() -> sockaddr_in6 {
    sockaddr_in6 {
        sin6_family: AF_INET6 as u16,
        sin6_addr: IN6ADDR_LOOPBACK_INIT,
        sin6_port: htons(8888),
        sin6_flowinfo: 0,
        sin6_scope_id: 0,
    }
}

unsafe fn tcp_listen() -> c_int {
    let mut addr = loopback_addr();
    let one: c_int = 1;
    let fd: c_int;

    fd = socket(PF_INET6, SOCK_STREAM, 0);
    if fd == -1 {
        error(1, errno, b"socket listener\0".as_ptr() as *const c_char);
    }
    if setsockopt(
        fd,
        SOL_SOCKET,
        SO_REUSEADDR,
        &one as *const _ as *const c_void,
        core::mem::size_of_val(&one) as socklen_t,
    ) != 0
    {
        error(
            1,
            errno,
            b"setsockopt SO_REUSEADDR\0".as_ptr() as *const c_char,
        );
    }
    if bind(
        fd,
        &mut addr as *mut _ as *const c_void,
        core::mem::size_of_val(&addr) as socklen_t,
    ) != 0
    {
        error(1, errno, b"bind\0".as_ptr() as *const c_char);
    }
    if listen(fd, 1) != 0 {
        error(1, errno, b"listen\0".as_ptr() as *const c_char);
    }

    fd
}

unsafe fn tcp_connect(
    listener: c_int,
    flowlabel: uint32_t,
    client: *mut c_int,
    accepted: *mut c_int,
) {
    let mut addr = loopback_addr();
    let one: c_int = 1;
    let cfd: c_int;
    let afd: c_int;

    cfd = socket(PF_INET6, SOCK_STREAM, 0);
    if cfd == -1 {
        error(1, errno, b"socket client\0".as_ptr() as *const c_char);
    }

    if flowlabel_get(cfd, flowlabel, IPV6_FL_S_EXCL, IPV6_FL_F_CREATE) != 0 {
        error(1, errno, b"flowlabel_get\0".as_ptr() as *const c_char);
    }
    if setsockopt(
        cfd,
        SOL_IPV6,
        IPV6_FLOWINFO_SEND,
        &one as *const _ as *const c_void,
        core::mem::size_of_val(&one) as socklen_t,
    ) != 0
    {
        error(
            1,
            errno,
            b"setsockopt flowinfo_send\0".as_ptr() as *const c_char,
        );
    }
    addr.sin6_flowinfo = htonl(flowlabel);

    if connect(
        cfd,
        &mut addr as *mut _ as *const c_void,
        core::mem::size_of_val(&addr) as socklen_t,
    ) != 0
    {
        error(1, errno, b"connect\0".as_ptr() as *const c_char);
    }

    afd = accept(listener, core::ptr::null_mut(), core::ptr::null_mut());
    if afd == -1 {
        error(1, errno, b"accept\0".as_ptr() as *const c_char);
    }

    if flowlabel_put(cfd, flowlabel) != 0 {
        error(1, errno, b"flowlabel_put\0".as_ptr() as *const c_char);
    }

    *client = cfd;
    *accepted = afd;
}

unsafe fn bringup_loopback() -> c_int {
    let mut ifr = ifreq {
        ifr_name: [0; 16],
        ifr_ifru: ifr_ifru { ifru_flags: 0 },
    };
    ifr.ifr_name[0] = b'l' as c_char;
    ifr.ifr_name[1] = b'o' as c_char;
    let fd: c_int;

    fd = socket(AF_LOCAL, SOCK_STREAM, 0);
    if fd < 0 {
        return -1;
    }

    if ioctl(fd, SIOCGIFFLAGS, &mut ifr) < 0 {
        close(fd);
        return -1;
    }

    ifr.ifr_ifru.ifru_flags = ifr.ifr_ifru.ifru_flags | IFF_UP;

    if ioctl(fd, SIOCSIFFLAGS, &mut ifr) < 0 {
        close(fd);
        return -1;
    }

    close(fd);
    0
}

struct flowlabel {}

unsafe fn flowlabel_setup() {
    let mut ret: c_int;

    ret = unshare(CLONE_NEWNET);
    ASSERT_EQ!(ret, 0);
    TH_LOG!("unshare(CLONE_NEWNET) failed: %s", strerror(errno));

    ret = bringup_loopback();
    ASSERT_EQ!(ret, 0);
    TH_LOG!("Failed to bring up loopback interface");
}

unsafe fn flowlabel_teardown() {}

unsafe fn cannot_get_non_existent_label() {
    let fd: c_int;
    let err: c_int;

    fd = socket(PF_INET6, SOCK_DGRAM, 0);
    ASSERT_GE!(fd, 0);
    TH_LOG!("socket failed");

    err = flowlabel_get(fd, 9, IPV6_FL_S_ANY, 0);
    EXPECT_TRUE!(err != 0);
    TH_LOG!("expected get of a non-existent label to fail");
    EXPECT_EQ!(ENOENT, errno);
    TH_LOG!("expected ENOENT, got %d", errno);

    EXPECT_EQ!(0, close(fd));
}

unsafe fn cannot_put_non_existent_label() {
    let fd: c_int;
    let err: c_int;

    fd = socket(PF_INET6, SOCK_DGRAM, 0);
    ASSERT_GE!(fd, 0);
    TH_LOG!("socket failed");

    err = flowlabel_put(fd, 10);
    EXPECT_TRUE!(err != 0);
    TH_LOG!("expected put of a non-existent label to fail");
    EXPECT_EQ!(ESRCH, errno);
    TH_LOG!("expected ESRCH, got %d", errno);

    EXPECT_EQ!(0, close(fd));
}

unsafe fn cannot_create_label_greater_than_20_bits() {
    let fd: c_int;
    let err: c_int;

    fd = socket(PF_INET6, SOCK_DGRAM, 0);
    ASSERT_GE!(fd, 0);
    TH_LOG!("socket failed");

    err = flowlabel_get(fd, 0x1FFFFF, IPV6_FL_S_ANY, IPV6_FL_F_CREATE);
    EXPECT_TRUE!(err != 0);
    TH_LOG!("expected label > 20 bits to be rejected");
    EXPECT_EQ!(EINVAL, errno);
    TH_LOG!("expected EINVAL, got %d", errno);

    EXPECT_EQ!(0, close(fd));
}

unsafe fn can_create_and_get_and_put_labels() {
    let fd: c_int;
    let mut err: c_int;

    fd = socket(PF_INET6, SOCK_DGRAM, 0);
    ASSERT_GE!(fd, 0);
    TH_LOG!("socket failed");

    err = flowlabel_get(fd, 1, IPV6_FL_S_ANY, IPV6_FL_F_CREATE);
    EXPECT_TRUE!(err == 0);
    TH_LOG!("failed to create label (FL_F_CREATE)");

    err = flowlabel_get(fd, 1, IPV6_FL_S_ANY, 0);
    EXPECT_TRUE!(err == 0);
    TH_LOG!("failed to get the label without FL_F_CREATE");

    err = flowlabel_get(fd, 1, IPV6_FL_S_ANY, IPV6_FL_F_CREATE);
    EXPECT_TRUE!(err == 0);
    TH_LOG!("failed to get it again with create flag set, too");

    err = flowlabel_get(fd, 1, IPV6_FL_S_ANY, IPV6_FL_F_CREATE | IPV6_FL_F_EXCL);
    EXPECT_TRUE!(err != 0);
    TH_LOG!("expected FL_F_EXCL to reject existing label");
    EXPECT_EQ!(EEXIST, errno);
    TH_LOG!("expected EEXIST, got %d", errno);

    err = flowlabel_put(fd, 1);
    EXPECT_TRUE!(err == 0);
    TH_LOG!("failed to put first reference");
    err = flowlabel_put(fd, 1);
    EXPECT_TRUE!(err == 0);
    TH_LOG!("failed to put second reference");
    err = flowlabel_put(fd, 1);
    EXPECT_TRUE!(err == 0);
    TH_LOG!("failed to put third reference");
    err = flowlabel_put(fd, 1);
    EXPECT_TRUE!(err != 0);
    TH_LOG!("expected fourth put to fail, no references left");
    EXPECT_EQ!(ESRCH, errno);
    TH_LOG!("expected ESRCH, got %d", errno);

    EXPECT_EQ!(0, close(fd));
}

unsafe fn exclusive_label_share() {
    let fd: c_int;
    let mut err: c_int;

    fd = socket(PF_INET6, SOCK_DGRAM, 0);
    ASSERT_GE!(fd, 0);
    TH_LOG!("socket failed");

    err = flowlabel_get(fd, 2, IPV6_FL_S_EXCL, IPV6_FL_F_CREATE);
    EXPECT_TRUE!(err == 0);
    TH_LOG!("failed to create a new exclusive label (FL_S_EXCL)");

    err = flowlabel_get(fd, 2, IPV6_FL_S_ANY, IPV6_FL_F_CREATE);
    EXPECT_TRUE!(err != 0);
    TH_LOG!("expected reuse in non-exclusive mode to fail");
    EXPECT_EQ!(EPERM, errno);
    TH_LOG!("expected EPERM, got %d", errno);

    err = flowlabel_get(fd, 2, IPV6_FL_S_EXCL, IPV6_FL_F_CREATE);
    EXPECT_TRUE!(err != 0);
    TH_LOG!("expected reuse in exclusive mode to fail too");
    EXPECT_EQ!(EPERM, errno);
    TH_LOG!("expected EPERM, got %d", errno);

    err = flowlabel_put(fd, 2);
    EXPECT_TRUE!(err == 0);
    TH_LOG!("failed to put the exclusive label");

    err = flowlabel_get(fd, 2, IPV6_FL_S_ANY, IPV6_FL_F_CREATE);
    EXPECT_TRUE!(err != 0);
    TH_LOG!("expected reuse to fail, due to linger");
    EXPECT_EQ!(EPERM, errno);
    TH_LOG!("expected EPERM, got %d", errno);

    sleep(FL_MIN_LINGER * 2 + 1);

    err = flowlabel_get(fd, 2, IPV6_FL_S_ANY, IPV6_FL_F_CREATE);
    EXPECT_TRUE!(err == 0);
    TH_LOG!("expected reuse to succeed after linger");

    EXPECT_EQ!(0, close(fd));
}

unsafe fn user_private_label_share() {
    let fd: c_int;
    let mut err: c_int;
    let mut wstatus: c_int = 0;
    let pid: pid_t;

    fd = socket(PF_INET6, SOCK_DGRAM, 0);
    ASSERT_GE!(fd, 0);
    TH_LOG!("socket failed");

    err = flowlabel_get(fd, 3, IPV6_FL_S_USER, IPV6_FL_F_CREATE);
    EXPECT_TRUE!(err == 0);
    TH_LOG!("failed to create a new user-private label (FL_S_USER)");

    err = flowlabel_get(fd, 3, IPV6_FL_S_ANY, 0);
    EXPECT_TRUE!(err != 0);
    TH_LOG!("expected get in non-exclusive mode to fail");
    EXPECT_EQ!(EPERM, errno);
    TH_LOG!("expected EPERM, got %d", errno);

    err = flowlabel_get(fd, 3, IPV6_FL_S_EXCL, 0);
    EXPECT_TRUE!(err != 0);
    TH_LOG!("expected get in exclusive mode to fail");
    EXPECT_EQ!(EPERM, errno);
    TH_LOG!("expected EPERM, got %d", errno);

    err = flowlabel_get(fd, 3, IPV6_FL_S_USER, 0);
    EXPECT_TRUE!(err == 0);
    TH_LOG!("failed to get it again in user mode");

    pid = fork();
    ASSERT_NE!(-1, pid);
    TH_LOG!("fork failed");
    if pid == 0 {
        err = flowlabel_get(fd, 3, IPV6_FL_S_USER, 0);
        EXPECT_TRUE!(err == 0);
        TH_LOG!("child failed to get the user-private label");

        if setuid(u16::MAX as c_uint) != 0 {
            exit(KSFT_SKIP);
        }

        err = flowlabel_get(fd, 3, IPV6_FL_S_USER, 0);
        EXPECT_TRUE!(err != 0);
        TH_LOG!("child unexpectedly got label after setuid");
        EXPECT_EQ!(EPERM, errno);
        TH_LOG!("expected EPERM, got %d", errno);
        exit(0);
    }
    ASSERT_EQ!(pid, wait(&mut wstatus));
    TH_LOG!("wait failed");
    ASSERT_TRUE!(WIFEXITED(wstatus));
    TH_LOG!("child did not exit normally");
    if WEXITSTATUS(wstatus) == KSFT_SKIP {
        SKIP!(
            return,
            "setuid(USHRT_MAX) unavailable (no CAP_SETUID or uid unmapped)"
        );
    }
    EXPECT_EQ!(0, WEXITSTATUS(wstatus));
    TH_LOG!("child reported unexpected result");

    EXPECT_EQ!(0, close(fd));
}

unsafe fn process_private_label_share() {
    let fd: c_int;
    let mut err: c_int;
    let mut wstatus: c_int = 0;
    let pid: pid_t;

    fd = socket(PF_INET6, SOCK_DGRAM, 0);
    ASSERT_GE!(fd, 0);
    TH_LOG!("socket failed");

    err = flowlabel_get(fd, 4, IPV6_FL_S_PROCESS, IPV6_FL_F_CREATE);
    EXPECT_TRUE!(err == 0);
    TH_LOG!("failed to create a new process-private label");

    err = flowlabel_get(fd, 4, IPV6_FL_S_PROCESS, 0);
    EXPECT_TRUE!(err == 0);
    TH_LOG!("failed to get it again");

    pid = fork();
    ASSERT_NE!(-1, pid);
    TH_LOG!("fork failed");
    if pid == 0 {
        err = flowlabel_get(fd, 4, IPV6_FL_S_PROCESS, 0);
        EXPECT_TRUE!(err != 0);
        TH_LOG!("child unexpectedly got process-private label");
        EXPECT_EQ!(EPERM, errno);
        TH_LOG!("expected EPERM, got %d", errno);
        exit(0);
    }
    ASSERT_EQ!(pid, wait(&mut wstatus));
    TH_LOG!("wait failed");
    ASSERT_TRUE!(WIFEXITED(wstatus));
    TH_LOG!("child did not exit normally");
    EXPECT_EQ!(0, WEXITSTATUS(wstatus));
    TH_LOG!("child reported unexpected result");

    EXPECT_EQ!(0, close(fd));
}

unsafe fn cannot_renew_non_existent_label() {
    let fd: c_int;
    let err: c_int;

    fd = socket(PF_INET6, SOCK_DGRAM, 0);
    ASSERT_GE!(fd, 0);
    TH_LOG!("socket failed");

    err = flowlabel_renew(fd, 5, IPV6_FL_S_EXCL, (2 * (FL_MIN_LINGER * 2 + 1)) as uint16_t);
    EXPECT_TRUE!(err != 0);
    TH_LOG!("expected renew of a non-existent label to fail");
    EXPECT_EQ!(ESRCH, errno);
    TH_LOG!("expected ESRCH, got %d", errno);

    EXPECT_EQ!(0, close(fd));
}

unsafe fn can_renew_existing_label() {
    let fd: c_int;
    let mut err: c_int;

    fd = socket(PF_INET6, SOCK_DGRAM, 0);
    ASSERT_GE!(fd, 0);
    TH_LOG!("socket failed");

    err = flowlabel_get(fd, 5, IPV6_FL_S_EXCL, IPV6_FL_F_CREATE);
    EXPECT_TRUE!(err == 0);
    TH_LOG!("failed to create a new label for renew validation");

    err = flowlabel_renew(fd, 5, IPV6_FL_S_EXCL, (2 * (FL_MIN_LINGER * 2 + 1)) as uint16_t);
    EXPECT_TRUE!(err == 0);
    TH_LOG!("failed to renew an existing valid label");

    err = flowlabel_put(fd, 5);
    EXPECT_TRUE!(err == 0);
    TH_LOG!("failed to put the label");

    EXPECT_EQ!(0, close(fd));
}

unsafe fn renew_label_linger() {
    /* RENEW must extend a label's linger period: putting a renewed
     * label and waiting out its original linger time must not be
     * enough to allow the label to be recreated.
     */
    let fd: c_int;
    let mut err: c_int;

    fd = socket(PF_INET6, SOCK_DGRAM, 0);
    ASSERT_GE!(fd, 0);
    TH_LOG!("socket failed");

    err = flowlabel_get(fd, 6, IPV6_FL_S_EXCL, IPV6_FL_F_CREATE);
    EXPECT_TRUE!(err == 0);
    TH_LOG!("failed to create label with FL_MIN_LINGER linger time");

    err = flowlabel_renew(fd, 6, IPV6_FL_S_EXCL, (2 * (FL_MIN_LINGER * 2 + 1)) as uint16_t);
    EXPECT_TRUE!(err == 0);
    TH_LOG!("failed to renew the label to increase its linger time");

    err = flowlabel_put(fd, 6);
    EXPECT_TRUE!(err == 0);
    TH_LOG!("failed to put the label");

    sleep(FL_MIN_LINGER * 2 + 1);

    err = flowlabel_get(fd, 6, IPV6_FL_S_ANY, IPV6_FL_F_CREATE);
    EXPECT_TRUE!(err != 0);
    TH_LOG!("expected reuse to fail, new linger time not over yet");
    EXPECT_EQ!(EPERM, errno);
    TH_LOG!("expected EPERM, got %d", errno);

    EXPECT_EQ!(0, close(fd));
}

unsafe fn remote_flag() {
    /* The REMOTE flag, used for getsockopt, is expected to retrieve the
     * label from the latest received header.
     */
    let mut freq = in6_flowlabel_req {
        flr_action: IPV6_FL_A_GET,
        flr_flags: IPV6_FL_F_REMOTE,
        flr_dst: in6_addr { s6_addr: [0; 16] },
        flr_label: 0,
        flr_share: 0,
        flr_expires: 0,
        flr_linger: 0,
        __flr_pad: 0,
    };
    let mut freq_len: socklen_t = core::mem::size_of_val(&freq) as socklen_t;
    let listener: c_int;
    let mut cfd: c_int = 0;
    let mut afd: c_int = 0;
    let err: c_int;

    listener = tcp_listen();
    tcp_connect(listener, 7, &mut cfd, &mut afd);

    err = getsockopt(
        afd,
        SOL_IPV6,
        IPV6_FLOWLABEL_MGR,
        &mut freq as *mut _ as *mut c_void,
        &mut freq_len,
    );
    EXPECT_TRUE!(err == 0);
    TH_LOG!("getsockopt with IPV6_FL_F_REMOTE failed");
    EXPECT_EQ!(7, ntohl(freq.flr_label));
    TH_LOG!("unexpected remote flow label");

    EXPECT_EQ!(0, close(afd));
    EXPECT_EQ!(0, close(cfd));
    EXPECT_EQ!(0, close(listener));
}

unsafe fn disable_flowlabel_consistency() -> bool {
    let fd: c_int;

    fd = open(
        b"/proc/sys/net/ipv6/flowlabel_consistency\0".as_ptr() as *const c_char,
        O_WRONLY,
    );
    if fd == -1 {
        return false;
    }

    if write(fd, b"0".as_ptr() as *const c_void, 1) != 1 {
        close(fd);
        return false;
    }
    close(fd);

    true
}

unsafe fn reflect_flag() {
    /* The REFLECT flag acts as a trigger to the REPFLOW bit. When REPFLOW
     * is triggered for a socket, it adopts the label received from the
     * connected socket.
     */
    let reflect_on = in6_flowlabel_req {
        flr_action: IPV6_FL_A_GET,
        flr_flags: IPV6_FL_F_REFLECT,
        flr_dst: in6_addr { s6_addr: [0; 16] },
        flr_label: 0,
        flr_share: 0,
        flr_expires: 0,
        flr_linger: 0,
        __flr_pad: 0,
    };
    let mut reflect_query = in6_flowlabel_req {
        flr_action: IPV6_FL_A_GET,
        flr_dst: in6_addr { s6_addr: [0; 16] },
        flr_label: 0,
        flr_share: 0,
        flr_flags: 0,
        flr_expires: 0,
        flr_linger: 0,
        __flr_pad: 0,
    };
    let reflect_off = in6_flowlabel_req {
        flr_action: IPV6_FL_A_PUT,
        flr_flags: IPV6_FL_F_REFLECT,
        flr_dst: in6_addr { s6_addr: [0; 16] },
        flr_label: 0,
        flr_share: 0,
        flr_expires: 0,
        flr_linger: 0,
        __flr_pad: 0,
    };
    let mut reflect_query_len: socklen_t = core::mem::size_of_val(&reflect_query) as socklen_t;
    let listener: c_int;
    let mut cfd: c_int = 0;
    let mut afd: c_int = 0;
    let mut err: c_int;

    if !disable_flowlabel_consistency() {
        SKIP!(return, "cannot disable net.ipv6.flowlabel_consistency");
    }

    listener = tcp_listen();
    err = setsockopt(
        listener,
        SOL_IPV6,
        IPV6_FLOWLABEL_MGR,
        &reflect_on as *const _ as *const c_void,
        core::mem::size_of_val(&reflect_on) as socklen_t,
    );
    EXPECT_TRUE!(err == 0);
    TH_LOG!("failed to enable REFLECT on the listener");

    tcp_connect(listener, 8, &mut cfd, &mut afd);

    err = getsockopt(
        afd,
        SOL_IPV6,
        IPV6_FLOWLABEL_MGR,
        &mut reflect_query as *mut _ as *mut c_void,
        &mut reflect_query_len,
    );
    EXPECT_TRUE!(err == 0);
    TH_LOG!("failed to query the accepted socket's outgoing label");
    EXPECT_EQ!(8, ntohl(reflect_query.flr_label));
    TH_LOG!("accepted socket did not reflect client's label");

    err = setsockopt(
        afd,
        SOL_IPV6,
        IPV6_FLOWLABEL_MGR,
        &reflect_off as *const _ as *const c_void,
        core::mem::size_of_val(&reflect_off) as socklen_t,
    );
    EXPECT_TRUE!(err == 0);
    TH_LOG!("failed to disable REFLECT on the accepted socket");

    err = setsockopt(
        afd,
        SOL_IPV6,
        IPV6_FLOWLABEL_MGR,
        &reflect_off as *const _ as *const c_void,
        core::mem::size_of_val(&reflect_off) as socklen_t,
    );
    EXPECT_TRUE!(err != 0);
    TH_LOG!("expected disabling REFLECT twice to fail");
    EXPECT_EQ!(ESRCH, errno);
    TH_LOG!("expected ESRCH, got %d", errno);

    EXPECT_EQ!(0, close(afd));
    EXPECT_EQ!(0, close(cfd));
    EXPECT_EQ!(0, close(listener));
}

fn main() {
    unsafe {
        flowlabel_setup();
        cannot_get_non_existent_label();
        cannot_put_non_existent_label();
        cannot_create_label_greater_than_20_bits();
        can_create_and_get_and_put_labels();
        exclusive_label_share();
        user_private_label_share();
        process_private_label_share();
        cannot_renew_non_existent_label();
        can_renew_existing_label();
        renew_label_linger();
        remote_flag();
        reflect_flag();
        flowlabel_teardown();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
