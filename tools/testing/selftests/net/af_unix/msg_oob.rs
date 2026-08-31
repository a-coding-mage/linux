// SPDX-License-Identifier: GPL-2.0
/* Copyright Amazon.com Inc. or its affiliates. */

// C dependencies translated from:
// <fcntl.h>, <string.h>, <unistd.h>, <netinet/in.h>, <sys/epoll.h>,
// <sys/ioctl.h>, <sys/signalfd.h>, <sys/socket.h>, and
// "kselftest_harness.h".

use core::ffi::{c_char, c_int, c_void};
use core::mem;
use core::ptr;

const BUF_SZ: usize = 32;

#[repr(C)]
struct msg_oob {
    fd: [c_int; 4], /* 0: AF_UNIX sender
                    * 1: AF_UNIX receiver
                    * 2: TCP sender
                    * 3: TCP receiver
                    */
    signal_fd: c_int,
    epoll_fd: [c_int; 2], /* 0: AF_UNIX receiver
                          * 1: TCP receiver
                          */
    tcp_compliant: bool,
}

#[repr(C)]
struct msg_oob_variant {
    peek: bool,
}

static no_peek: msg_oob_variant = msg_oob_variant {
    peek: false,
};

static peek: msg_oob_variant = msg_oob_variant {
    peek: true,
};

type socklen_t = libc::socklen_t;

extern "C" {
    fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn listen(sockfd: c_int, backlog: c_int) -> c_int;
    fn getsockname(sockfd: c_int, addr: *mut libc::sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn connect(sockfd: c_int, addr: *const libc::sockaddr, addrlen: socklen_t) -> c_int;
    fn accept(sockfd: c_int, addr: *mut libc::sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn getpid() -> c_int;
    fn ioctl(fd: c_int, request: libc::c_ulong, ...) -> c_int;
    fn sigemptyset(set: *mut libc::sigset_t) -> c_int;
    fn sigaddset(set: *mut libc::sigset_t, signum: c_int) -> c_int;
    fn sigprocmask(how: c_int, set: *const libc::sigset_t, oldset: *mut libc::sigset_t) -> c_int;
    fn signalfd(fd: c_int, mask: *const libc::sigset_t, flags: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn epoll_create1(flags: c_int) -> c_int;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut libc::epoll_event) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn epoll_wait(epfd: c_int, events: *mut libc::epoll_event, maxevents: c_int, timeout: c_int) -> c_int;
    fn send(sockfd: c_int, buf: *const c_void, len: usize, flags: c_int) -> isize;
    fn recv(sockfd: c_int, buf: *mut c_void, len: usize, flags: c_int) -> isize;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn setsockopt(sockfd: c_int, level: c_int, optname: c_int, optval: *const c_void, optlen: socklen_t) -> c_int;
}

extern "C" {
    static mut errno: c_int;
}

#[allow(non_camel_case_types)]
#[repr(C)]
struct __test_metadata {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct signalfd_siginfo {
    ssi_signo: u32,
    _rest: [u8; 124],
}

extern "C" {
    fn TH_LOG(fmt: *const c_char, ...);
}

macro_rules! c_str {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! ASSERT_GE {
    ($left:expr, $right:expr) => {
        assert!($left >= $right)
    };
}

unsafe fn create_unix_socketpair(_metadata: *mut __test_metadata, self_: *mut msg_oob) {
    let ret: c_int;

    ret = socketpair(
        libc::AF_UNIX,
        libc::SOCK_STREAM | libc::SOCK_NONBLOCK,
        0,
        (*self_).fd.as_mut_ptr(),
    );
    ASSERT_EQ!(ret, 0);
}

unsafe fn create_tcp_socketpair(_metadata: *mut __test_metadata, self_: *mut msg_oob) {
    let mut addr: libc::sockaddr_in = mem::zeroed();
    let mut addrlen: socklen_t;
    let listen_fd: c_int;
    let mut ret: c_int;

    listen_fd = socket(libc::AF_INET, libc::SOCK_STREAM, 0);
    ASSERT_GE!(listen_fd, 0);

    ret = listen(listen_fd, -1);
    ASSERT_EQ!(ret, 0);

    addrlen = mem::size_of_val(&addr) as socklen_t;
    ret = getsockname(
        listen_fd,
        &mut addr as *mut libc::sockaddr_in as *mut libc::sockaddr,
        &mut addrlen,
    );
    ASSERT_EQ!(ret, 0);

    (*self_).fd[2] = socket(libc::AF_INET, libc::SOCK_STREAM, 0);
    ASSERT_GE!((*self_).fd[2], 0);

    ret = connect(
        (*self_).fd[2],
        &addr as *const libc::sockaddr_in as *const libc::sockaddr,
        addrlen,
    );
    ASSERT_EQ!(ret, 0);

    (*self_).fd[3] = accept(
        listen_fd,
        &mut addr as *mut libc::sockaddr_in as *mut libc::sockaddr,
        &mut addrlen,
    );
    ASSERT_GE!((*self_).fd[3], 0);

    ret = fcntl((*self_).fd[3], libc::F_SETFL, libc::O_NONBLOCK);
    ASSERT_EQ!(ret, 0);
}

unsafe fn setup_sigurg(_metadata: *mut __test_metadata, self_: *mut msg_oob) {
    let mut siginfo: signalfd_siginfo = mem::zeroed();
    let pid: c_int = getpid();
    let mut mask: libc::sigset_t = mem::zeroed();
    let mut i: c_int;
    let mut ret: c_int;

    i = 0;
    while i < 2 {
        ret = ioctl((*self_).fd[(i * 2 + 1) as usize], libc::FIOSETOWN as libc::c_ulong, &pid);
        ASSERT_EQ!(ret, 0);
        i += 1;
    }

    ret = sigemptyset(&mut mask);
    ASSERT_EQ!(ret, 0);

    ret = sigaddset(&mut mask, libc::SIGURG);
    ASSERT_EQ!(ret, 0);

    ret = sigprocmask(libc::SIG_BLOCK, &mask, ptr::null_mut());
    ASSERT_EQ!(ret, 0);

    (*self_).signal_fd = signalfd(-1, &mask, libc::SFD_NONBLOCK);
    ASSERT_GE!((*self_).signal_fd, 0);

    ret = read(
        (*self_).signal_fd,
        &mut siginfo as *mut signalfd_siginfo as *mut c_void,
        mem::size_of_val(&siginfo),
    ) as c_int;
    ASSERT_EQ!(ret, -1);
}

unsafe fn setup_epollpri(_metadata: *mut __test_metadata, self_: *mut msg_oob) {
    let mut event: libc::epoll_event = mem::zeroed();
    let mut i: c_int;

    event.events = libc::EPOLLPRI as u32;

    i = 0;
    while i < 2 {
        let ret: c_int;

        (*self_).epoll_fd[i as usize] = epoll_create1(0);
        ASSERT_GE!((*self_).epoll_fd[i as usize], 0);

        ret = epoll_ctl(
            (*self_).epoll_fd[i as usize],
            libc::EPOLL_CTL_ADD,
            (*self_).fd[(i * 2 + 1) as usize],
            &mut event,
        );
        ASSERT_EQ!(ret, 0);
        i += 1;
    }
}

unsafe fn close_sockets(self_: *mut msg_oob) {
    let mut i: c_int;

    i = 0;
    while i < 4 {
        close((*self_).fd[i as usize]);
        i += 1;
    }
}

unsafe fn msg_oob_setup(_metadata: *mut __test_metadata, self_: *mut msg_oob) {
    create_unix_socketpair(_metadata, self_);
    create_tcp_socketpair(_metadata, self_);

    setup_sigurg(_metadata, self_);
    setup_epollpri(_metadata, self_);

    (*self_).tcp_compliant = true;
}

unsafe fn msg_oob_teardown(_metadata: *mut __test_metadata, self_: *mut msg_oob) {
    close_sockets(self_);
}

unsafe fn __epollpair(_metadata: *mut __test_metadata, self_: *mut msg_oob, oob_remaining: bool) {
    let mut event: [libc::epoll_event; 2] = mem::zeroed();
    let mut i: c_int;
    let mut ret: [c_int; 2] = [0; 2];

    i = 0;
    while i < 2 {
        ret[i as usize] = epoll_wait((*self_).epoll_fd[i as usize], &mut event[i as usize], 1, 0);
        i += 1;
    }

    ASSERT_EQ!(ret[0], oob_remaining as c_int);

    if (*self_).tcp_compliant {
        ASSERT_EQ!(ret[0], ret[1]);
    }

    if oob_remaining {
        ASSERT_EQ!(event[0].events, libc::EPOLLPRI as u32);

        if (*self_).tcp_compliant {
            ASSERT_EQ!(event[0].events, event[1].events);
        }
    }
}

unsafe fn __sendpair(
    _metadata: *mut __test_metadata,
    self_: *mut msg_oob,
    buf: *const c_void,
    len: usize,
    flags: c_int,
) {
    let mut i: c_int;
    let mut ret: [isize; 2] = [0; 2];

    i = 0;
    while i < 2 {
        let mut siginfo: signalfd_siginfo = mem::zeroed();
        let mut bytes: c_int;

        ret[i as usize] = send((*self_).fd[(i * 2) as usize], buf, len, flags);

        bytes = read(
            (*self_).signal_fd,
            &mut siginfo as *mut signalfd_siginfo as *mut c_void,
            mem::size_of_val(&siginfo),
        ) as c_int;

        if flags & libc::MSG_OOB != 0 {
            ASSERT_EQ!(bytes, mem::size_of_val(&siginfo) as c_int);
            ASSERT_EQ!(siginfo.ssi_signo, libc::SIGURG as u32);

            bytes = read(
                (*self_).signal_fd,
                &mut siginfo as *mut signalfd_siginfo as *mut c_void,
                mem::size_of_val(&siginfo),
            ) as c_int;
        }

        ASSERT_EQ!(bytes, -1);
        i += 1;
    }

    ASSERT_EQ!(ret[0], len as isize);
    ASSERT_EQ!(ret[0], ret[1]);
}

unsafe fn __recvpair(
    _metadata: *mut __test_metadata,
    self_: *mut msg_oob,
    expected_buf: *const c_char,
    mut expected_len: c_int,
    buf_len: c_int,
    flags: c_int,
    is_sender: bool,
) {
    let mut i: c_int;
    let mut ret: [isize; 2] = [0; 2];
    let mut recv_errno: [c_int; 2] = [0; 2];
    let mut expected_errno: c_int = 0;
    let mut recv_buf: [[c_char; BUF_SZ]; 2] = [[0; BUF_SZ]; 2];
    let mut printed: bool = false;

    ASSERT_GE!(BUF_SZ as c_int, buf_len);

    errno = 0;

    i = 0;
    while i < 2 {
        let index: c_int = if is_sender { i * 2 } else { i * 2 + 1 };

        ret[i as usize] = recv(
            (*self_).fd[index as usize],
            recv_buf[i as usize].as_mut_ptr() as *mut c_void,
            buf_len as usize,
            flags,
        );
        recv_errno[i as usize] = errno;
        i += 1;
    }

    if expected_len < 0 {
        expected_errno = -expected_len;
        expected_len = -1;
    }

    if ret[0] != expected_len as isize || recv_errno[0] != expected_errno {
        TH_LOG(
            c_str!("AF_UNIX :%s"),
            if ret[0] < 0 { strerror(recv_errno[0]) } else { recv_buf[0].as_ptr() as *mut c_char },
        );
        TH_LOG(
            c_str!("Expected:%s"),
            if expected_errno != 0 { strerror(expected_errno) } else { expected_buf as *mut c_char },
        );

        ASSERT_EQ!(ret[0], expected_len as isize);
        ASSERT_EQ!(recv_errno[0], expected_errno);
    }

    if ret[0] != ret[1] || recv_errno[0] != recv_errno[1] {
        TH_LOG(
            c_str!("AF_UNIX :%s"),
            if ret[0] < 0 { strerror(recv_errno[0]) } else { recv_buf[0].as_ptr() as *mut c_char },
        );
        TH_LOG(
            c_str!("TCP     :%s"),
            if ret[1] < 0 { strerror(recv_errno[1]) } else { recv_buf[1].as_ptr() as *mut c_char },
        );

        printed = true;

        if (*self_).tcp_compliant {
            ASSERT_EQ!(ret[0], ret[1]);
            ASSERT_EQ!(recv_errno[0], recv_errno[1]);
        }
    }

    if expected_len >= 0 {
        let mut cmp: c_int;

        cmp = strncmp(expected_buf, recv_buf[0].as_ptr(), expected_len as usize);
        if cmp != 0 {
            TH_LOG(
                c_str!("AF_UNIX :%s"),
                if ret[0] < 0 { strerror(recv_errno[0]) } else { recv_buf[0].as_ptr() as *mut c_char },
            );
            TH_LOG(
                c_str!("Expected:%s"),
                if expected_errno != 0 { strerror(expected_errno) } else { expected_buf as *mut c_char },
            );

            ASSERT_EQ!(cmp, 0);
        }

        cmp = strncmp(recv_buf[0].as_ptr(), recv_buf[1].as_ptr(), expected_len as usize);
        if cmp != 0 {
            if !printed {
                TH_LOG(
                    c_str!("AF_UNIX :%s"),
                    if ret[0] < 0 { strerror(recv_errno[0]) } else { recv_buf[0].as_ptr() as *mut c_char },
                );
                TH_LOG(
                    c_str!("TCP     :%s"),
                    if ret[1] < 0 { strerror(recv_errno[1]) } else { recv_buf[1].as_ptr() as *mut c_char },
                );
            }

            if (*self_).tcp_compliant {
                ASSERT_EQ!(cmp, 0);
            }
        }
    }
}

unsafe fn __setinlinepair(_metadata: *mut __test_metadata, self_: *mut msg_oob) {
    let mut i: c_int;
    let oob_inline: c_int = 1;

    i = 0;
    while i < 2 {
        let ret: c_int;

        ret = setsockopt(
            (*self_).fd[(i * 2 + 1) as usize],
            libc::SOL_SOCKET,
            libc::SO_OOBINLINE,
            &oob_inline as *const c_int as *const c_void,
            mem::size_of_val(&oob_inline) as socklen_t,
        );
        ASSERT_EQ!(ret, 0);
        i += 1;
    }
}

unsafe fn __siocatmarkpair(_metadata: *mut __test_metadata, self_: *mut msg_oob, oob_head: bool) {
    let mut answ: [c_int; 2] = [0; 2];
    let mut i: c_int;

    i = 0;
    while i < 2 {
        let ret: c_int;

        ret = ioctl((*self_).fd[(i * 2 + 1) as usize], libc::SIOCATMARK as libc::c_ulong, &mut answ[i as usize]);
        ASSERT_EQ!(ret, 0);
        i += 1;
    }

    ASSERT_EQ!(answ[0], oob_head as c_int);

    if (*self_).tcp_compliant {
        ASSERT_EQ!(answ[0], answ[1]);
    }
}

unsafe fn __resetpair(
    _metadata: *mut __test_metadata,
    self_: *mut msg_oob,
    variant: *const msg_oob_variant,
    reset: bool,
) {
    let mut i: c_int;

    i = 0;
    while i < 2 {
        close((*self_).fd[(i * 2 + 1) as usize]);
        i += 1;
    }

    __recvpair(
        _metadata,
        self_,
        c_str!(""),
        if reset { -libc::ECONNRESET } else { 0 },
        1,
        if (*variant).peek { libc::MSG_PEEK } else { 0 },
        true,
    );
}

macro_rules! sendpair {
    ($metadata:expr, $self_:expr, $buf:expr, $len:expr, $flags:expr) => {
        __sendpair($metadata, $self_, c_str!($buf) as *const c_void, $len, $flags)
    };
}

macro_rules! recvpair {
    ($metadata:expr, $self_:expr, $variant:expr, $expected_buf:expr, $expected_len:expr, $buf_len:expr, $flags:expr) => {{
        if (*$variant).peek {
            __recvpair(
                $metadata,
                $self_,
                c_str!($expected_buf),
                $expected_len,
                $buf_len,
                ($flags) | libc::MSG_PEEK,
                false,
            );
        }
        __recvpair($metadata, $self_, c_str!($expected_buf), $expected_len, $buf_len, $flags, false);
    }};
}

macro_rules! epollpair {
    ($metadata:expr, $self_:expr, $oob_remaining:expr) => {
        __epollpair($metadata, $self_, $oob_remaining)
    };
}

macro_rules! siocatmarkpair {
    ($metadata:expr, $self_:expr, $oob_head:expr) => {
        __siocatmarkpair($metadata, $self_, $oob_head)
    };
}

macro_rules! setinlinepair {
    ($metadata:expr, $self_:expr) => {
        __setinlinepair($metadata, $self_)
    };
}

macro_rules! resetpair {
    ($metadata:expr, $self_:expr, $variant:expr, $reset:expr) => {
        __resetpair($metadata, $self_, $variant, $reset)
    };
}

macro_rules! tcp_incompliant {
    ($self_:expr, $body:block) => {{
        (*$self_).tcp_compliant = false;
        while (*$self_).tcp_compliant == false {
            $body
            (*$self_).tcp_compliant = true;
        }
    }};
}

unsafe fn non_oob(_metadata: *mut __test_metadata, self_: *mut msg_oob, variant: *const msg_oob_variant) {
    sendpair!(_metadata, self_, "x", 1, 0);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, false);

    recvpair!(_metadata, self_, variant, "", -libc::EINVAL, 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, false);

    resetpair!(_metadata, self_, variant, true);
}

unsafe fn non_oob_no_reset(_metadata: *mut __test_metadata, self_: *mut msg_oob, variant: *const msg_oob_variant) {
    sendpair!(_metadata, self_, "x", 1, 0);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, false);

    recvpair!(_metadata, self_, variant, "x", 1, 1, 0);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, false);

    resetpair!(_metadata, self_, variant, false);
}

unsafe fn oob(_metadata: *mut __test_metadata, self_: *mut msg_oob, variant: *const msg_oob_variant) {
    sendpair!(_metadata, self_, "x", 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, true);

    recvpair!(_metadata, self_, variant, "x", 1, 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, true);

    tcp_incompliant!(self_, {
        resetpair!(_metadata, self_, variant, false); /* TCP sets -ECONNRESET for ex-OOB. */
    });
}

unsafe fn oob_reset(_metadata: *mut __test_metadata, self_: *mut msg_oob, variant: *const msg_oob_variant) {
    sendpair!(_metadata, self_, "x", 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, true);

    resetpair!(_metadata, self_, variant, true);
}

unsafe fn oob_drop(_metadata: *mut __test_metadata, self_: *mut msg_oob, variant: *const msg_oob_variant) {
    sendpair!(_metadata, self_, "x", 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, true);

    recvpair!(_metadata, self_, variant, "", -libc::EAGAIN, 1, 0); /* Drop OOB. */
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, false);

    recvpair!(_metadata, self_, variant, "", -libc::EINVAL, 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, false);

    resetpair!(_metadata, self_, variant, false);
}

unsafe fn oob_ahead(_metadata: *mut __test_metadata, self_: *mut msg_oob, variant: *const msg_oob_variant) {
    sendpair!(_metadata, self_, "hello", 5, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, false);

    recvpair!(_metadata, self_, variant, "o", 1, 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, false);

    recvpair!(_metadata, self_, variant, "hell", 4, 4, 0);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, true);

    tcp_incompliant!(self_, {
        resetpair!(_metadata, self_, variant, false); /* TCP sets -ECONNRESET for ex-OOB. */
    });
}

unsafe fn oob_break(_metadata: *mut __test_metadata, self_: *mut msg_oob, variant: *const msg_oob_variant) {
    sendpair!(_metadata, self_, "hello", 5, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, false);

    recvpair!(_metadata, self_, variant, "hell", 4, 5, 0); /* Break at OOB even with enough buffer. */
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, true);

    recvpair!(_metadata, self_, variant, "o", 1, 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, true);

    recvpair!(_metadata, self_, variant, "", -libc::EAGAIN, 1, 0);
    siocatmarkpair!(_metadata, self_, false);

    resetpair!(_metadata, self_, variant, false);
}

unsafe fn oob_ahead_break(_metadata: *mut __test_metadata, self_: *mut msg_oob, variant: *const msg_oob_variant) {
    sendpair!(_metadata, self_, "hello", 5, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, false);

    sendpair!(_metadata, self_, "world", 5, 0);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, false);

    recvpair!(_metadata, self_, variant, "o", 1, 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, false);

    recvpair!(_metadata, self_, variant, "hell", 4, 9, 0); /* Break at OOB even after it's recv()ed. */
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, true);

    recvpair!(_metadata, self_, variant, "world", 5, 5, 0);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, false);

    resetpair!(_metadata, self_, variant, false);
}

unsafe fn oob_break_drop(_metadata: *mut __test_metadata, self_: *mut msg_oob, variant: *const msg_oob_variant) {
    sendpair!(_metadata, self_, "hello", 5, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, false);

    sendpair!(_metadata, self_, "world", 5, 0);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, false);

    recvpair!(_metadata, self_, variant, "hell", 4, 10, 0); /* Break at OOB even with enough buffer. */
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, true);

    recvpair!(_metadata, self_, variant, "world", 5, 10, 0); /* Drop OOB and recv() the next skb. */
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, false);

    recvpair!(_metadata, self_, variant, "", -libc::EINVAL, 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, false);

    resetpair!(_metadata, self_, variant, false);
}

unsafe fn ex_oob_break(_metadata: *mut __test_metadata, self_: *mut msg_oob, variant: *const msg_oob_variant) {
    sendpair!(_metadata, self_, "hello", 5, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, false);

    sendpair!(_metadata, self_, "wor", 3, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, false);

    sendpair!(_metadata, self_, "ld", 2, 0);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, false);

    recvpair!(_metadata, self_, variant, "hellowo", 7, 10, 0); /* Break at OOB but not at ex-OOB. */
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, true);

    recvpair!(_metadata, self_, variant, "r", 1, 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, true);

    recvpair!(_metadata, self_, variant, "ld", 2, 2, 0);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, false);

    resetpair!(_metadata, self_, variant, false);
}

unsafe fn ex_oob_drop(_metadata: *mut __test_metadata, self_: *mut msg_oob, variant: *const msg_oob_variant) {
    sendpair!(_metadata, self_, "x", 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, true);

    sendpair!(_metadata, self_, "y", 1, libc::MSG_OOB); /* TCP drops "x" at this moment. */
    epollpair!(_metadata, self_, true);

    tcp_incompliant!(self_, {
        siocatmarkpair!(_metadata, self_, false);

        recvpair!(_metadata, self_, variant, "x", 1, 1, 0); /* TCP drops "y" by passing through it. */
        epollpair!(_metadata, self_, true);
        siocatmarkpair!(_metadata, self_, true);

        recvpair!(_metadata, self_, variant, "y", 1, 1, libc::MSG_OOB); /* TCP returns -EINVAL. */
        epollpair!(_metadata, self_, false);
        siocatmarkpair!(_metadata, self_, true);
    });

    resetpair!(_metadata, self_, variant, false);
}

unsafe fn ex_oob_drop_2(_metadata: *mut __test_metadata, self_: *mut msg_oob, variant: *const msg_oob_variant) {
    sendpair!(_metadata, self_, "x", 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, true);

    sendpair!(_metadata, self_, "y", 1, libc::MSG_OOB); /* TCP drops "x" at this moment. */
    epollpair!(_metadata, self_, true);

    tcp_incompliant!(self_, {
        siocatmarkpair!(_metadata, self_, false);
    });

    recvpair!(_metadata, self_, variant, "y", 1, 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, false);

    tcp_incompliant!(self_, {
        siocatmarkpair!(_metadata, self_, false);

        recvpair!(_metadata, self_, variant, "x", 1, 1, 0); /* TCP returns -EAGAIN. */
        epollpair!(_metadata, self_, false);
        siocatmarkpair!(_metadata, self_, true);
    });

    resetpair!(_metadata, self_, variant, false);
}

unsafe fn ex_oob_oob(_metadata: *mut __test_metadata, self_: *mut msg_oob, variant: *const msg_oob_variant) {
    sendpair!(_metadata, self_, "x", 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, true);

    recvpair!(_metadata, self_, variant, "x", 1, 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, true);

    sendpair!(_metadata, self_, "y", 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, true);

    recvpair!(_metadata, self_, variant, "", -libc::EAGAIN, 1, 0);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, false);

    recvpair!(_metadata, self_, variant, "", -libc::EINVAL, 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, false);

    resetpair!(_metadata, self_, variant, false);
}

unsafe fn ex_oob_ex_oob(_metadata: *mut __test_metadata, self_: *mut msg_oob, variant: *const msg_oob_variant) {
    sendpair!(_metadata, self_, "x", 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, true);

    recvpair!(_metadata, self_, variant, "x", 1, 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, true);

    sendpair!(_metadata, self_, "y", 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, true);

    recvpair!(_metadata, self_, variant, "y", 1, 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, true);

    tcp_incompliant!(self_, {
        resetpair!(_metadata, self_, variant, false); /* TCP sets -ECONNRESET for ex-OOB. */
    });
}

unsafe fn ex_oob_ex_oob_oob(_metadata: *mut __test_metadata, self_: *mut msg_oob, variant: *const msg_oob_variant) {
    sendpair!(_metadata, self_, "x", 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, true);

    recvpair!(_metadata, self_, variant, "x", 1, 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, true);

    sendpair!(_metadata, self_, "y", 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, true);

    recvpair!(_metadata, self_, variant, "y", 1, 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, true);

    sendpair!(_metadata, self_, "z", 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, true);
}

unsafe fn ex_oob_ahead_break(_metadata: *mut __test_metadata, self_: *mut msg_oob, variant: *const msg_oob_variant) {
    sendpair!(_metadata, self_, "hello", 5, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, false);

    sendpair!(_metadata, self_, "wor", 3, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, false);

    recvpair!(_metadata, self_, variant, "r", 1, 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, false);

    sendpair!(_metadata, self_, "ld", 2, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, false);

    tcp_incompliant!(self_, {
        recvpair!(_metadata, self_, variant, "hellowol", 8, 10, 0); /* TCP recv()s "helloworl", why "r" ?? */
    });

    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, true);

    recvpair!(_metadata, self_, variant, "d", 1, 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, true);

    tcp_incompliant!(self_, {
        resetpair!(_metadata, self_, variant, false); /* TCP sets -ECONNRESET for ex-OOB. */
    });
}

unsafe fn ex_oob_siocatmark(_metadata: *mut __test_metadata, self_: *mut msg_oob, variant: *const msg_oob_variant) {
    sendpair!(_metadata, self_, "hello", 5, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, false);

    recvpair!(_metadata, self_, variant, "o", 1, 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, false);

    sendpair!(_metadata, self_, "world", 5, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, false);

    recvpair!(_metadata, self_, variant, "hell", 4, 4, 0); /* Intentionally stop at ex-OOB. */
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, false);

    resetpair!(_metadata, self_, variant, true);
}

unsafe fn inline_oob(_metadata: *mut __test_metadata, self_: *mut msg_oob, variant: *const msg_oob_variant) {
    setinlinepair!(_metadata, self_);

    sendpair!(_metadata, self_, "x", 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, true);

    recvpair!(_metadata, self_, variant, "", -libc::EINVAL, 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, true);

    recvpair!(_metadata, self_, variant, "x", 1, 1, 0);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, false);

    resetpair!(_metadata, self_, variant, false);
}

unsafe fn inline_oob_break(_metadata: *mut __test_metadata, self_: *mut msg_oob, variant: *const msg_oob_variant) {
    setinlinepair!(_metadata, self_);

    sendpair!(_metadata, self_, "hello", 5, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, false);

    recvpair!(_metadata, self_, variant, "", -libc::EINVAL, 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, false);

    recvpair!(_metadata, self_, variant, "hell", 4, 5, 0); /* Break at OOB but not at ex-OOB. */
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, true);

    recvpair!(_metadata, self_, variant, "o", 1, 1, 0);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, false);

    resetpair!(_metadata, self_, variant, false);
}

unsafe fn inline_oob_ahead_break(_metadata: *mut __test_metadata, self_: *mut msg_oob, variant: *const msg_oob_variant) {
    sendpair!(_metadata, self_, "hello", 5, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, false);

    sendpair!(_metadata, self_, "world", 5, 0);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, false);

    recvpair!(_metadata, self_, variant, "o", 1, 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, false);

    setinlinepair!(_metadata, self_);

    recvpair!(_metadata, self_, variant, "hell", 4, 9, 0); /* Break at OOB even with enough buffer. */
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, true);

    tcp_incompliant!(self_, {
        recvpair!(_metadata, self_, variant, "world", 5, 6, 0); /* TCP recv()s "oworld", ... "o" ??? */
    });

    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, false);

    resetpair!(_metadata, self_, variant, false);
}

unsafe fn inline_ex_oob_break(_metadata: *mut __test_metadata, self_: *mut msg_oob, variant: *const msg_oob_variant) {
    sendpair!(_metadata, self_, "hello", 5, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, false);

    sendpair!(_metadata, self_, "wor", 3, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, false);

    sendpair!(_metadata, self_, "ld", 2, 0);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, false);

    setinlinepair!(_metadata, self_);

    recvpair!(_metadata, self_, variant, "hellowo", 7, 10, 0); /* Break at OOB but not at ex-OOB. */
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, true);

    recvpair!(_metadata, self_, variant, "rld", 3, 3, 0);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, false);

    resetpair!(_metadata, self_, variant, false);
}

unsafe fn inline_ex_oob_no_drop(_metadata: *mut __test_metadata, self_: *mut msg_oob, variant: *const msg_oob_variant) {
    sendpair!(_metadata, self_, "x", 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, true);

    setinlinepair!(_metadata, self_);

    sendpair!(_metadata, self_, "y", 1, libc::MSG_OOB); /* TCP does NOT drops "x" at this moment. */
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, false);

    recvpair!(_metadata, self_, variant, "x", 1, 1, 0);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, true);

    recvpair!(_metadata, self_, variant, "y", 1, 1, 0);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, false);

    resetpair!(_metadata, self_, variant, false);
}

unsafe fn inline_ex_oob_drop(_metadata: *mut __test_metadata, self_: *mut msg_oob, variant: *const msg_oob_variant) {
    sendpair!(_metadata, self_, "x", 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, true);

    sendpair!(_metadata, self_, "y", 1, libc::MSG_OOB); /* TCP drops "x" at this moment. */
    epollpair!(_metadata, self_, true);

    setinlinepair!(_metadata, self_);

    tcp_incompliant!(self_, {
        siocatmarkpair!(_metadata, self_, false);

        recvpair!(_metadata, self_, variant, "x", 1, 1, 0); /* TCP recv()s "y". */
        epollpair!(_metadata, self_, true);
        siocatmarkpair!(_metadata, self_, true);

        recvpair!(_metadata, self_, variant, "y", 1, 1, 0); /* TCP returns -EAGAIN. */
        epollpair!(_metadata, self_, false);
        siocatmarkpair!(_metadata, self_, false);
    });

    resetpair!(_metadata, self_, variant, false);
}

unsafe fn inline_ex_oob_siocatmark(_metadata: *mut __test_metadata, self_: *mut msg_oob, variant: *const msg_oob_variant) {
    sendpair!(_metadata, self_, "hello", 5, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, false);

    recvpair!(_metadata, self_, variant, "o", 1, 1, libc::MSG_OOB);
    epollpair!(_metadata, self_, false);
    siocatmarkpair!(_metadata, self_, false);

    setinlinepair!(_metadata, self_);

    sendpair!(_metadata, self_, "world", 5, libc::MSG_OOB);
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, false);

    recvpair!(_metadata, self_, variant, "hell", 4, 4, 0); /* Intentionally stop at ex-OOB. */
    epollpair!(_metadata, self_, true);
    siocatmarkpair!(_metadata, self_, false);

    resetpair!(_metadata, self_, variant, true);
}

// TEST_HARNESS_MAIN
