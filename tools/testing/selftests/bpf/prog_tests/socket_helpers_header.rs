/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from testing/selftests/bpf/prog_tests/socket_helpers.h.
 * C include dependencies preserved as Rust libc-style references:
 * <sys/un.h>, <linux/vm_sockets.h>, and socket/select/errno APIs.
 */

/* include/linux/net.h */
pub const SOCK_TYPE_MASK: libc::c_int = 0xf;

pub const IO_TIMEOUT_SEC: libc::c_uint = 30;
pub const MAX_STRERR_LEN: usize = 256;

/* workaround for older vm_sockets.h */
#[allow(non_upper_case_globals)]
pub const VMADDR_CID_LOCAL: libc::c_uint = 1;

unsafe extern "C" {
    pub fn error_at_line(
        status: libc::c_int,
        errnum: libc::c_int,
        filename: *const libc::c_char,
        linenum: libc::c_uint,
        format: *const libc::c_char,
        ...
    );

    pub fn libbpf_strerror(
        err: libc::c_int,
        buf: *mut libc::c_char,
        size: libc::size_t,
    ) -> libc::c_int;
}

#[inline]
pub unsafe fn __errno_location_value() -> libc::c_int {
    unsafe { *libc::__errno_location() }
}

#[inline]
pub unsafe fn __set_errno_value(value: libc::c_int) {
    unsafe {
        *libc::__errno_location() = value;
    }
}

#[macro_export]
macro_rules! __get_and_null {
    ($p:expr, $nullvalue:expr) => {{
        let __ptr = &mut $p;
        let __val = *__ptr;
        *__ptr = $nullvalue;
        __val
    }};
}

#[macro_export]
macro_rules! take_fd {
    ($fd:expr) => {
        $crate::__get_and_null!($fd, -libc::EBADF)
    };
}

/* Wrappers that fail the test on error and report it. */

#[macro_export]
macro_rules! _FAIL {
    ($errnum:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {{
        unsafe {
            error_at_line(
                0,
                $errnum,
                concat!(module_path!(), "\0").as_ptr() as *const libc::c_char,
                line!(),
                concat!($fmt, "\0").as_ptr() as *const libc::c_char
                $(, $arg)*
            );
        }
        CHECK_FAIL!(true);
    }};
}

#[macro_export]
macro_rules! FAIL {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::_FAIL!(0, $fmt $(, $arg)*)
    };
}

#[macro_export]
macro_rules! FAIL_ERRNO {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::_FAIL!(unsafe { $crate::__errno_location_value() }, $fmt $(, $arg)*)
    };
}

#[macro_export]
macro_rules! FAIL_LIBBPF {
    ($err:expr, $msg:expr) => {{
        let mut __buf = [0 as libc::c_char; MAX_STRERR_LEN];
        unsafe {
            libbpf_strerror($err, __buf.as_mut_ptr(), __buf.len());
        }
        $crate::FAIL!("%s: %s", $msg, __buf.as_ptr());
    }};
}

#[macro_export]
macro_rules! xaccept_nonblock {
    ($fd:expr, $addr:expr, $len:expr) => {{
        let __ret = unsafe { accept_timeout($fd, $addr, $len, IO_TIMEOUT_SEC) };
        if __ret == -1 {
            FAIL_ERRNO!("accept");
        }
        __ret
    }};
}

#[macro_export]
macro_rules! xbind {
    ($fd:expr, $addr:expr, $len:expr) => {{
        let __ret = unsafe { libc::bind($fd, $addr, $len) };
        if __ret == -1 {
            FAIL_ERRNO!("bind");
        }
        __ret
    }};
}

#[macro_export]
macro_rules! xclose {
    ($fd:expr) => {{
        let __ret = unsafe { libc::close($fd) };
        if __ret == -1 {
            FAIL_ERRNO!("close");
        }
        __ret
    }};
}

#[macro_export]
macro_rules! xconnect {
    ($fd:expr, $addr:expr, $len:expr) => {{
        let __ret = unsafe { libc::connect($fd, $addr, $len) };
        if __ret == -1 {
            FAIL_ERRNO!("connect");
        }
        __ret
    }};
}

#[macro_export]
macro_rules! xgetsockname {
    ($fd:expr, $addr:expr, $len:expr) => {{
        let __ret = unsafe { libc::getsockname($fd, $addr, $len) };
        if __ret == -1 {
            FAIL_ERRNO!("getsockname");
        }
        __ret
    }};
}

#[macro_export]
macro_rules! xgetsockopt {
    ($fd:expr, $level:expr, $name:expr, $val:expr, $len:expr) => {{
        let __ret = unsafe { libc::getsockopt($fd, $level, $name, $val, $len) };
        if __ret == -1 {
            FAIL_ERRNO!(concat!("getsockopt(", stringify!($name), ")"));
        }
        __ret
    }};
}

#[macro_export]
macro_rules! xlisten {
    ($fd:expr, $backlog:expr) => {{
        let __ret = unsafe { libc::listen($fd, $backlog) };
        if __ret == -1 {
            FAIL_ERRNO!("listen");
        }
        __ret
    }};
}

#[macro_export]
macro_rules! xsetsockopt {
    ($fd:expr, $level:expr, $name:expr, $val:expr, $len:expr) => {{
        let __ret = unsafe { libc::setsockopt($fd, $level, $name, $val as *const libc::c_void, $len) };
        if __ret == -1 {
            FAIL_ERRNO!(concat!("setsockopt(", stringify!($name), ")"));
        }
        __ret
    }};
}

#[macro_export]
macro_rules! xsend {
    ($fd:expr, $buf:expr, $len:expr, $flags:expr) => {{
        let __ret = unsafe { libc::send($fd, $buf, $len, $flags) };
        if __ret == -1 {
            FAIL_ERRNO!("send");
        }
        __ret
    }};
}

#[macro_export]
macro_rules! xrecv_nonblock {
    ($fd:expr, $buf:expr, $len:expr, $flags:expr) => {{
        let __ret = unsafe { recv_timeout($fd, $buf, $len, $flags, IO_TIMEOUT_SEC) };
        if __ret == -1 {
            FAIL_ERRNO!("recv");
        }
        __ret
    }};
}

#[macro_export]
macro_rules! xsocket {
    ($family:expr, $sotype:expr, $flags:expr) => {{
        let __ret = unsafe { libc::socket($family, $sotype, $flags) };
        if __ret == -1 {
            FAIL_ERRNO!("socket");
        }
        __ret
    }};
}

#[inline]
pub unsafe fn close_fd(fd: *mut libc::c_int) {
    unsafe {
        if *fd >= 0 {
            xclose!(*fd);
        }
    }
}

/* __close_fd was a C cleanup attribute: __attribute__((cleanup(close_fd))). */

#[inline]
pub unsafe fn sockaddr(ss: *mut libc::sockaddr_storage) -> *mut libc::sockaddr {
    ss as *mut libc::sockaddr
}

#[inline]
pub unsafe fn init_addr_loopback4(ss: *mut libc::sockaddr_storage, len: *mut libc::socklen_t) {
    unsafe {
        let addr4 = libc::memset(
            ss as *mut libc::c_void,
            0,
            core::mem::size_of_val(&*ss),
        ) as *mut libc::sockaddr_in;

        (*addr4).sin_family = libc::AF_INET as libc::sa_family_t;
        (*addr4).sin_port = 0;
        (*addr4).sin_addr.s_addr = libc::htonl(libc::INADDR_LOOPBACK);
        *len = core::mem::size_of_val(&*addr4) as libc::socklen_t;
    }
}

#[inline]
pub unsafe fn init_addr_loopback6(ss: *mut libc::sockaddr_storage, len: *mut libc::socklen_t) {
    unsafe {
        let addr6 = libc::memset(
            ss as *mut libc::c_void,
            0,
            core::mem::size_of_val(&*ss),
        ) as *mut libc::sockaddr_in6;

        (*addr6).sin6_family = libc::AF_INET6 as libc::sa_family_t;
        (*addr6).sin6_port = 0;
        (*addr6).sin6_addr = libc::in6_addr {
            s6_addr: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        };
        *len = core::mem::size_of_val(&*addr6) as libc::socklen_t;
    }
}

#[inline]
pub unsafe fn init_addr_loopback_unix(ss: *mut libc::sockaddr_storage, len: *mut libc::socklen_t) {
    unsafe {
        let addr = libc::memset(
            ss as *mut libc::c_void,
            0,
            core::mem::size_of_val(&*ss),
        ) as *mut libc::sockaddr_un;

        (*addr).sun_family = libc::AF_UNIX as libc::sa_family_t;
        *len = core::mem::size_of::<libc::sa_family_t>() as libc::socklen_t;
    }
}

#[repr(C)]
pub struct sockaddr_vm {
    pub svm_family: libc::sa_family_t,
    pub svm_reserved1: libc::c_ushort,
    pub svm_port: libc::c_uint,
    pub svm_cid: libc::c_uint,
    pub svm_zero: [u8; 4],
}

pub const VMADDR_PORT_ANY: libc::c_uint = 0xffff_ffff;

#[inline]
pub unsafe fn init_addr_loopback_vsock(ss: *mut libc::sockaddr_storage, len: *mut libc::socklen_t) {
    unsafe {
        let addr = libc::memset(
            ss as *mut libc::c_void,
            0,
            core::mem::size_of_val(&*ss),
        ) as *mut sockaddr_vm;

        (*addr).svm_family = libc::AF_VSOCK as libc::sa_family_t;
        (*addr).svm_port = VMADDR_PORT_ANY;
        (*addr).svm_cid = VMADDR_CID_LOCAL;
        *len = core::mem::size_of_val(&*addr) as libc::socklen_t;
    }
}

#[inline]
pub unsafe fn init_addr_loopback(
    family: libc::c_int,
    ss: *mut libc::sockaddr_storage,
    len: *mut libc::socklen_t,
) {
    unsafe {
        match family {
            libc::AF_INET => {
                init_addr_loopback4(ss, len);
                return;
            }
            libc::AF_INET6 => {
                init_addr_loopback6(ss, len);
                return;
            }
            libc::AF_UNIX => {
                init_addr_loopback_unix(ss, len);
                return;
            }
            libc::AF_VSOCK => {
                init_addr_loopback_vsock(ss, len);
                return;
            }
            _ => {
                FAIL!("unsupported address family %d", family);
            }
        }
    }
}

#[inline]
pub unsafe fn enable_reuseport(s: libc::c_int, progfd: libc::c_int) -> libc::c_int {
    unsafe {
        let mut one: libc::c_int = 1;

        let mut err = xsetsockopt!(
            s,
            libc::SOL_SOCKET,
            libc::SO_REUSEPORT,
            &mut one,
            core::mem::size_of_val(&one) as libc::socklen_t
        );
        if err != 0 {
            return -1;
        }
        err = xsetsockopt!(
            s,
            libc::SOL_SOCKET,
            libc::SO_ATTACH_REUSEPORT_EBPF,
            &progfd,
            core::mem::size_of_val(&progfd) as libc::socklen_t
        );
        if err != 0 {
            return -1;
        }

        0
    }
}

#[inline]
pub unsafe fn socket_loopback_reuseport(
    family: libc::c_int,
    sotype: libc::c_int,
    progfd: libc::c_int,
) -> libc::c_int {
    unsafe {
        let mut addr: libc::sockaddr_storage = core::mem::zeroed();
        let mut len: libc::socklen_t = 0;

        init_addr_loopback(family, &mut addr, &mut len);

        let s = xsocket!(family, sotype, 0);
        if s == -1 {
            return -1;
        }

        if progfd >= 0 {
            enable_reuseport(s, progfd);
        }

        let mut err = xbind!(s, sockaddr(&mut addr), len);
        if err != 0 {
            xclose!(s);
            return -1;
        }

        if (sotype & libc::SOCK_DGRAM) != 0 {
            return s;
        }

        err = xlisten!(s, libc::SOMAXCONN);
        if err != 0 {
            xclose!(s);
            return -1;
        }

        s
    }
}

#[inline]
pub unsafe fn socket_loopback(family: libc::c_int, sotype: libc::c_int) -> libc::c_int {
    unsafe { socket_loopback_reuseport(family, sotype, -1) }
}

#[inline]
pub unsafe fn poll_connect(fd: libc::c_int, timeout_sec: libc::c_uint) -> libc::c_int {
    unsafe {
        let mut timeout: libc::timeval = core::mem::zeroed();
        timeout.tv_sec = timeout_sec as libc::time_t;
        let mut wfds: libc::fd_set = core::mem::zeroed();
        let mut eval: libc::c_int = 0;
        let mut esize: libc::socklen_t = core::mem::size_of_val(&eval) as libc::socklen_t;

        libc::FD_ZERO(&mut wfds);
        libc::FD_SET(fd, &mut wfds);

        let r = libc::select(
            fd + 1,
            core::ptr::null_mut(),
            &mut wfds,
            core::ptr::null_mut(),
            &mut timeout,
        );
        if r == 0 {
            __set_errno_value(libc::ETIME);
        }
        if r != 1 {
            return -1;
        }

        if libc::getsockopt(
            fd,
            libc::SOL_SOCKET,
            libc::SO_ERROR,
            &mut eval as *mut _ as *mut libc::c_void,
            &mut esize,
        ) < 0
        {
            return -1;
        }
        if eval != 0 {
            __set_errno_value(eval);
            return -1;
        }

        0
    }
}

#[inline]
pub unsafe fn poll_read(fd: libc::c_int, timeout_sec: libc::c_uint) -> libc::c_int {
    unsafe {
        let mut timeout: libc::timeval = core::mem::zeroed();
        timeout.tv_sec = timeout_sec as libc::time_t;
        let mut rfds: libc::fd_set = core::mem::zeroed();

        libc::FD_ZERO(&mut rfds);
        libc::FD_SET(fd, &mut rfds);

        let r = libc::select(
            fd + 1,
            &mut rfds,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            &mut timeout,
        );
        if r == 0 {
            __set_errno_value(libc::ETIME);
        }

        if r == 1 { 0 } else { -1 }
    }
}

#[inline]
pub unsafe fn accept_timeout(
    fd: libc::c_int,
    addr: *mut libc::sockaddr,
    len: *mut libc::socklen_t,
    timeout_sec: libc::c_uint,
) -> libc::c_int {
    unsafe {
        if poll_read(fd, timeout_sec) != 0 {
            return -1;
        }

        libc::accept(fd, addr, len)
    }
}

#[inline]
pub unsafe fn recv_timeout(
    fd: libc::c_int,
    buf: *mut libc::c_void,
    len: libc::size_t,
    flags: libc::c_int,
    timeout_sec: libc::c_uint,
) -> libc::ssize_t {
    unsafe {
        if poll_read(fd, timeout_sec) != 0 {
            return -1;
        }

        libc::recv(fd, buf, len, flags)
    }
}

#[inline]
pub unsafe fn create_pair(
    family: libc::c_int,
    sotype: libc::c_int,
    p0: *mut libc::c_int,
    p1: *mut libc::c_int,
) -> libc::c_int {
    unsafe {
        let mut c: libc::c_int = -1;
        let mut p: libc::c_int = -1;
        let mut addr: libc::sockaddr_storage = core::mem::zeroed();
        let mut len: libc::socklen_t = core::mem::zeroed();

        let mut s = socket_loopback(family, sotype);
        if s < 0 {
            return s;
        }

        c = xsocket!(family, sotype, 0);
        if c < 0 {
            close_fd(&mut s);
            return c;
        }

        init_addr_loopback(family, &mut addr, &mut len);
        let mut err = xbind!(c, sockaddr(&mut addr), len);
        if err != 0 {
            close_fd(&mut p);
            close_fd(&mut c);
            close_fd(&mut s);
            return err;
        }

        len = core::mem::size_of_val(&addr) as libc::socklen_t;
        err = xgetsockname!(s, sockaddr(&mut addr), &mut len);
        if err != 0 {
            close_fd(&mut p);
            close_fd(&mut c);
            close_fd(&mut s);
            return err;
        }

        err = libc::connect(c, sockaddr(&mut addr), len);
        if err != 0 {
            if __errno_location_value() != libc::EINPROGRESS {
                FAIL_ERRNO!("connect");
                close_fd(&mut p);
                close_fd(&mut c);
                close_fd(&mut s);
                return err;
            }

            err = poll_connect(c, IO_TIMEOUT_SEC);
            if err != 0 {
                FAIL_ERRNO!("poll_connect");
                close_fd(&mut p);
                close_fd(&mut c);
                close_fd(&mut s);
                return err;
            }
        }

        match sotype & SOCK_TYPE_MASK {
            libc::SOCK_DGRAM => {
                err = xgetsockname!(c, sockaddr(&mut addr), &mut len);
                if err != 0 {
                    close_fd(&mut p);
                    close_fd(&mut c);
                    close_fd(&mut s);
                    return err;
                }

                err = xconnect!(s, sockaddr(&mut addr), len);
                if err != 0 {
                    close_fd(&mut p);
                    close_fd(&mut c);
                    close_fd(&mut s);
                    return err;
                }

                *p0 = take_fd!(s);
            }
            libc::SOCK_STREAM | libc::SOCK_SEQPACKET => {
                p = xaccept_nonblock!(s, core::ptr::null_mut(), core::ptr::null_mut());
                if p < 0 {
                    close_fd(&mut p);
                    close_fd(&mut c);
                    close_fd(&mut s);
                    return p;
                }

                *p0 = take_fd!(p);
            }
            _ => {
                FAIL!("Unsupported socket type %#x", sotype);
                close_fd(&mut p);
                close_fd(&mut c);
                close_fd(&mut s);
                return -libc::EOPNOTSUPP;
            }
        }

        *p1 = take_fd!(c);
        close_fd(&mut p);
        close_fd(&mut c);
        close_fd(&mut s);
        0
    }
}

#[inline]
pub unsafe fn create_socket_pairs(
    family: libc::c_int,
    sotype: libc::c_int,
    c0: *mut libc::c_int,
    c1: *mut libc::c_int,
    p0: *mut libc::c_int,
    p1: *mut libc::c_int,
) -> libc::c_int {
    unsafe {
        let mut err = create_pair(family, sotype, c0, p0);
        if err != 0 {
            return err;
        }

        err = create_pair(family, sotype, c1, p1);
        if err != 0 {
            libc::close(*c0);
            libc::close(*p0);
        }

        err
    }
}

#[inline]
pub unsafe fn socket_kind_to_str(sock_fd: libc::c_int) -> *const libc::c_char {
    unsafe {
        let mut domain: libc::c_int = 0;
        let mut type_: libc::c_int = 0;

        let mut opt_len = core::mem::size_of_val(&domain) as libc::socklen_t;
        if libc::getsockopt(
            sock_fd,
            libc::SOL_SOCKET,
            libc::SO_DOMAIN,
            &mut domain as *mut _ as *mut libc::c_void,
            &mut opt_len,
        ) != 0
        {
            FAIL_ERRNO!("getsockopt(SO_DOMAIN)");
        }

        opt_len = core::mem::size_of_val(&type_) as libc::socklen_t;
        if libc::getsockopt(
            sock_fd,
            libc::SOL_SOCKET,
            libc::SO_TYPE,
            &mut type_ as *mut _ as *mut libc::c_void,
            &mut opt_len,
        ) != 0
        {
            FAIL_ERRNO!("getsockopt(SO_TYPE)");
        }

        match domain {
            libc::AF_INET => match type_ {
                libc::SOCK_STREAM => return c"tcp4".as_ptr(),
                libc::SOCK_DGRAM => return c"udp4".as_ptr(),
                _ => {}
            },
            libc::AF_INET6 => match type_ {
                libc::SOCK_STREAM => return c"tcp6".as_ptr(),
                libc::SOCK_DGRAM => return c"udp6".as_ptr(),
                _ => {}
            },
            libc::AF_UNIX => match type_ {
                libc::SOCK_STREAM => return c"u_str".as_ptr(),
                libc::SOCK_DGRAM => return c"u_dgr".as_ptr(),
                libc::SOCK_SEQPACKET => return c"u_seq".as_ptr(),
                _ => {}
            },
            libc::AF_VSOCK => match type_ {
                libc::SOCK_STREAM => return c"v_str".as_ptr(),
                libc::SOCK_DGRAM => return c"v_dgr".as_ptr(),
                libc::SOCK_SEQPACKET => return c"v_seq".as_ptr(),
                _ => {}
            },
            _ => {}
        }

        c"???".as_ptr()
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
