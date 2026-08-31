// SPDX-License-Identifier: GPL-2.0-only
/*
 * vsock test utilities
 *
 * Copyright (C) 2017 Red Hat, Inc.
 *
 * Author: Stefan Hajnoczi <stefanha@redhat.com>
 */

use libc::{
    c_char, c_int, c_ulong, c_void, FILE, iovec, linger, size_t, sockaddr, sockaddr_vm, socklen_t,
    ssize_t, timeval,
};

const KALLSYMS_PATH: &[u8] = b"/proc/kallsyms\0";
const KALLSYMS_LINE_LEN: usize = 512;
const RECV_PEEK_RETRY_USEC: u64 = 10 * 1000;

const TEST_MODE_CLIENT: c_int = 0;
const TIMEOUT: c_int = 30;
const TRANSPORT_NUM: c_int = 0;

#[repr(C)]
pub struct test_opts {
    pub mode: c_int,
}

#[repr(C)]
pub struct test_case {
    pub name: *const c_char,
    pub run_client: Option<unsafe extern "C" fn(*const test_opts)>,
    pub run_server: Option<unsafe extern "C" fn(*const test_opts)>,
    pub skip: bool,
}

unsafe extern "C" {
    static transport_ksyms: [*const c_char; TRANSPORT_NUM as usize];

    fn sigalrm(signum: c_int);
    fn timeout_begin(seconds: c_int);
    fn timeout_check(operation: *const c_char);
    fn timeout_end();
    fn timeout_usleep(usec: u64);
    fn control_expectln(line: *const c_char);
    fn control_writeln(line: *const c_char);
    fn control_readln() -> *mut c_char;
    fn control_cmpln(line: *const c_char, expected: *const c_char, fail: bool) -> bool;
}

fn bit(nr: c_int) -> c_int {
    1 << nr
}

/* Install signal handlers */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_signals() {
    let mut act: libc::sigaction = unsafe { std::mem::zeroed() };

    act.sa_sigaction = sigalrm as usize;
    unsafe {
        libc::sigaction(libc::SIGALRM, &act, std::ptr::null_mut());
        libc::signal(libc::SIGPIPE, libc::SIG_IGN);
    }
}

unsafe fn parse_uint(str_: *const c_char, err_str: *const c_char) -> libc::c_uint {
    let mut endptr: *mut c_char = std::ptr::null_mut();
    let n: c_ulong;

    unsafe {
        *libc::__errno_location() = 0;
        n = libc::strtoul(str_, &mut endptr, 10);
        if *libc::__errno_location() != 0 || *endptr != 0 {
            libc::fprintf(
                libc::stderr,
                c"malformed %s \"%s\"\n".as_ptr(),
                err_str,
                str_,
            );
            libc::exit(libc::EXIT_FAILURE);
        }
    }
    n as libc::c_uint
}

/* Parse a CID in string representation */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_cid(str_: *const c_char) -> libc::c_uint {
    unsafe { parse_uint(str_, c"CID".as_ptr()) }
}

/* Parse a port in string representation */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_port(str_: *const c_char) -> libc::c_uint {
    unsafe { parse_uint(str_, c"port".as_ptr()) }
}

/* Wait for the remote to close the connection */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vsock_wait_remote_close(fd: c_int) {
    let mut ev: libc::epoll_event = unsafe { std::mem::zeroed() };
    let epollfd: c_int;
    let nfds: c_int;

    unsafe {
        epollfd = libc::epoll_create1(0);
        if epollfd == -1 {
            libc::perror(c"epoll_create1".as_ptr());
            libc::exit(libc::EXIT_FAILURE);
        }

        ev.events = (libc::EPOLLRDHUP | libc::EPOLLHUP) as u32;
        ev.u64 = fd as u64;
        if libc::epoll_ctl(epollfd, libc::EPOLL_CTL_ADD, fd, &mut ev) == -1 {
            libc::perror(c"epoll_ctl".as_ptr());
            libc::exit(libc::EXIT_FAILURE);
        }

        nfds = libc::epoll_wait(epollfd, &mut ev, 1, TIMEOUT * 1000);
        if nfds == -1 {
            libc::perror(c"epoll_wait".as_ptr());
            libc::exit(libc::EXIT_FAILURE);
        }

        if nfds == 0 {
            libc::fprintf(libc::stderr, c"epoll_wait timed out\n".as_ptr());
            libc::exit(libc::EXIT_FAILURE);
        }

        assert!(nfds == 1);
        assert!((ev.events & (libc::EPOLLRDHUP | libc::EPOLLHUP) as u32) != 0);
        assert!(ev.u64 as c_int == fd);

        libc::close(epollfd);
    }
}

/* Wait until ioctl gives an expected int value.
 * Return false if the op is not supported.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vsock_ioctl_int(fd: c_int, op: c_ulong, expected: c_int) -> bool {
    let mut actual: c_int = 0;
    let mut ret: c_int;
    let mut name = [0 as c_char; 32];

    unsafe {
        libc::snprintf(
            name.as_mut_ptr(),
            name.len(),
            c"ioctl(%lu)".as_ptr(),
            op,
        );

        timeout_begin(TIMEOUT);
        loop {
            ret = libc::ioctl(fd, op, &mut actual);
            if ret < 0 {
                if *libc::__errno_location() == libc::EOPNOTSUPP
                    || *libc::__errno_location() == libc::ENOTTY
                {
                    break;
                }

                libc::perror(name.as_ptr());
                libc::exit(libc::EXIT_FAILURE);
            }
            timeout_check(name.as_ptr());
            if actual == expected {
                break;
            }
        }
        timeout_end();
    }

    ret >= 0
}

/* Wait until transport reports no data left to be sent.
 * Return false if transport does not implement the unsent_bytes() callback.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vsock_wait_sent(fd: c_int) -> bool {
    unsafe { vsock_ioctl_int(fd, libc::SIOCOUTQ as c_ulong, 0) }
}

/* Create socket <type>, bind to <cid, port>.
 * Return the file descriptor, or -1 on error.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vsock_bind_try(cid: libc::c_uint, port: libc::c_uint, type_: c_int) -> c_int {
    let mut sa: sockaddr_vm = unsafe { std::mem::zeroed() };
    let fd: c_int;
    let saved_errno: c_int;

    sa.svm_family = libc::AF_VSOCK as libc::sa_family_t;
    sa.svm_cid = cid;
    sa.svm_port = port;

    unsafe {
        fd = libc::socket(libc::AF_VSOCK, type_, 0);
        if fd < 0 {
            libc::perror(c"socket".as_ptr());
            libc::exit(libc::EXIT_FAILURE);
        }

        if libc::bind(
            fd,
            &sa as *const sockaddr_vm as *const sockaddr,
            std::mem::size_of_val(&sa) as socklen_t,
        ) != 0
        {
            saved_errno = *libc::__errno_location();
            libc::close(fd);
            *libc::__errno_location() = saved_errno;
            return -1;
        }
    }

    fd
}

/* Create socket <type>, bind to <cid, port> and return the file descriptor. */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vsock_bind(cid: libc::c_uint, port: libc::c_uint, type_: c_int) -> c_int {
    let fd = unsafe { vsock_bind_try(cid, port, type_) };

    if fd < 0 {
        unsafe {
            libc::perror(c"bind".as_ptr());
            libc::exit(libc::EXIT_FAILURE);
        }
    }

    fd
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vsock_connect_fd(fd: c_int, cid: libc::c_uint, port: libc::c_uint) -> c_int {
    let mut sa: sockaddr_vm = unsafe { std::mem::zeroed() };
    let mut ret: c_int;

    sa.svm_family = libc::AF_VSOCK as libc::sa_family_t;
    sa.svm_cid = cid;
    sa.svm_port = port;

    unsafe {
        timeout_begin(TIMEOUT);
        loop {
            ret = libc::connect(
                fd,
                &sa as *const sockaddr_vm as *const sockaddr,
                std::mem::size_of_val(&sa) as socklen_t,
            );
            timeout_check(c"connect".as_ptr());
            if !(ret < 0 && *libc::__errno_location() == libc::EINTR) {
                break;
            }
        }
        timeout_end();
    }

    ret
}

/* Bind to <bind_port>, connect to <cid, port> and return the file descriptor. */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vsock_bind_connect(
    cid: libc::c_uint,
    port: libc::c_uint,
    bind_port: libc::c_uint,
    type_: c_int,
) -> c_int {
    let client_fd = unsafe { vsock_bind(libc::VMADDR_CID_ANY, bind_port, type_) };

    if unsafe { vsock_connect_fd(client_fd, cid, port) } != 0 {
        unsafe {
            libc::perror(c"connect".as_ptr());
            libc::exit(libc::EXIT_FAILURE);
        }
    }

    client_fd
}

/* Connect to <cid, port> and return the file descriptor. */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vsock_connect(cid: libc::c_uint, port: libc::c_uint, type_: c_int) -> c_int {
    let mut fd: c_int;

    unsafe {
        control_expectln(c"LISTENING".as_ptr());

        fd = libc::socket(libc::AF_VSOCK, type_, 0);
        if fd < 0 {
            libc::perror(c"socket".as_ptr());
            libc::exit(libc::EXIT_FAILURE);
        }

        if vsock_connect_fd(fd, cid, port) != 0 {
            let old_errno = *libc::__errno_location();

            libc::close(fd);
            fd = -1;
            *libc::__errno_location() = old_errno;
        }
    }

    fd
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vsock_stream_connect(cid: libc::c_uint, port: libc::c_uint) -> c_int {
    unsafe { vsock_connect(cid, port, libc::SOCK_STREAM) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vsock_seqpacket_connect(cid: libc::c_uint, port: libc::c_uint) -> c_int {
    unsafe { vsock_connect(cid, port, libc::SOCK_SEQPACKET) }
}

/* Listen on <cid, port> and return the file descriptor. */
unsafe fn vsock_listen(cid: libc::c_uint, port: libc::c_uint, type_: c_int) -> c_int {
    let fd = unsafe { vsock_bind(cid, port, type_) };

    if unsafe { libc::listen(fd, 1) } < 0 {
        unsafe {
            libc::perror(c"listen".as_ptr());
            libc::exit(libc::EXIT_FAILURE);
        }
    }

    fd
}

#[repr(C)]
union ClientAddr {
    sa: sockaddr,
    svm: sockaddr_vm,
}

/* Listen on <cid, port> and return the first incoming connection.  The remote
 * address is stored to clientaddrp.  clientaddrp may be NULL.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vsock_accept(
    cid: libc::c_uint,
    port: libc::c_uint,
    clientaddrp: *mut sockaddr_vm,
    type_: c_int,
) -> c_int {
    let mut clientaddr: ClientAddr = unsafe { std::mem::zeroed() };
    let mut clientaddr_len: socklen_t = std::mem::size_of::<sockaddr_vm>() as socklen_t;
    let fd: c_int;
    let mut client_fd: c_int;
    let old_errno: c_int;

    unsafe {
        fd = vsock_listen(cid, port, type_);

        control_writeln(c"LISTENING".as_ptr());

        timeout_begin(TIMEOUT);
        loop {
            client_fd = libc::accept(fd, &mut clientaddr.sa, &mut clientaddr_len);
            timeout_check(c"accept".as_ptr());
            if !(client_fd < 0 && *libc::__errno_location() == libc::EINTR) {
                break;
            }
        }
        timeout_end();

        old_errno = *libc::__errno_location();
        libc::close(fd);
        *libc::__errno_location() = old_errno;

        if client_fd < 0 {
            return client_fd;
        }

        if clientaddr_len as usize != std::mem::size_of::<sockaddr_vm>() {
            libc::fprintf(
                libc::stderr,
                c"unexpected addrlen from accept(2), %zu\n".as_ptr(),
                clientaddr_len as size_t,
            );
            libc::exit(libc::EXIT_FAILURE);
        }
        if clientaddr.sa.sa_family as c_int != libc::AF_VSOCK {
            libc::fprintf(
                libc::stderr,
                c"expected AF_VSOCK from accept(2), got %d\n".as_ptr(),
                clientaddr.sa.sa_family as c_int,
            );
            libc::exit(libc::EXIT_FAILURE);
        }

        if !clientaddrp.is_null() {
            *clientaddrp = clientaddr.svm;
        }
    }

    client_fd
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vsock_stream_accept(
    cid: libc::c_uint,
    port: libc::c_uint,
    clientaddrp: *mut sockaddr_vm,
) -> c_int {
    unsafe { vsock_accept(cid, port, clientaddrp, libc::SOCK_STREAM) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vsock_stream_listen(cid: libc::c_uint, port: libc::c_uint) -> c_int {
    unsafe { vsock_listen(cid, port, libc::SOCK_STREAM) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vsock_seqpacket_accept(
    cid: libc::c_uint,
    port: libc::c_uint,
    clientaddrp: *mut sockaddr_vm,
) -> c_int {
    unsafe { vsock_accept(cid, port, clientaddrp, libc::SOCK_SEQPACKET) }
}

/* Transmit bytes from a buffer and check the return value.
 *
 * expected_ret:
 *  <0 Negative errno (for testing errors)
 *   0 End-of-file
 *  >0 Success (bytes successfully written)
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn send_buf(
    fd: c_int,
    buf: *const c_void,
    len: size_t,
    flags: c_int,
    expected_ret: ssize_t,
) {
    let mut nwritten: ssize_t = 0;
    let mut ret: ssize_t;

    unsafe {
        timeout_begin(TIMEOUT);
        loop {
            ret = libc::send(
                fd,
                (buf as *const u8).offset(nwritten as isize) as *const c_void,
                len - nwritten as size_t,
                flags,
            );
            timeout_check(c"send".as_ptr());

            if ret < 0 && *libc::__errno_location() == libc::EINTR {
                continue;
            }
            if ret <= 0 {
                break;
            }

            nwritten += ret;
            if !(nwritten < len as ssize_t) {
                break;
            }
        }
        timeout_end();

        if expected_ret < 0 {
            if ret != -1 {
                libc::fprintf(
                    libc::stderr,
                    c"bogus send(2) return value %zd (expected %zd)\n".as_ptr(),
                    ret,
                    expected_ret,
                );
                libc::exit(libc::EXIT_FAILURE);
            }
            if *libc::__errno_location() != -expected_ret as c_int {
                libc::perror(c"send".as_ptr());
                libc::exit(libc::EXIT_FAILURE);
            }
            return;
        }

        if ret < 0 {
            libc::perror(c"send".as_ptr());
            libc::exit(libc::EXIT_FAILURE);
        }

        if nwritten != expected_ret {
            if ret == 0 {
                libc::fprintf(libc::stderr, c"unexpected EOF while sending bytes\n".as_ptr());
            }

            libc::fprintf(
                libc::stderr,
                c"bogus send(2) bytes written %zd (expected %zd)\n".as_ptr(),
                nwritten,
                expected_ret,
            );
            libc::exit(libc::EXIT_FAILURE);
        }
    }
}

/* Receive bytes in a buffer and check the return value.
 *
 * When MSG_PEEK is set, recv() is retried until it returns at least
 * expected_ret bytes. The function returns on error, EOF, or timeout
 * as usual.
 *
 * expected_ret:
 *  <0 Negative errno (for testing errors)
 *   0 End-of-file
 *  >0 Success (bytes successfully read)
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn recv_buf(
    fd: c_int,
    buf: *mut c_void,
    len: size_t,
    flags: c_int,
    expected_ret: ssize_t,
) {
    let mut nread: ssize_t = 0;
    let mut ret: ssize_t;

    unsafe {
        timeout_begin(TIMEOUT);
        loop {
            ret = libc::recv(
                fd,
                (buf as *mut u8).offset(nread as isize) as *mut c_void,
                len - nread as size_t,
                flags,
            );
            timeout_check(c"recv".as_ptr());

            if ret < 0 && *libc::__errno_location() == libc::EINTR {
                continue;
            }
            if ret <= 0 {
                break;
            }

            if flags & libc::MSG_PEEK != 0 {
                if ret >= expected_ret {
                    nread = ret;
                    break;
                }
                timeout_usleep(RECV_PEEK_RETRY_USEC);
                continue;
            }

            nread += ret;
            if !(nread < len as ssize_t) {
                break;
            }
        }
        timeout_end();

        if expected_ret < 0 {
            if ret != -1 {
                libc::fprintf(
                    libc::stderr,
                    c"bogus recv(2) return value %zd (expected %zd)\n".as_ptr(),
                    ret,
                    expected_ret,
                );
                libc::exit(libc::EXIT_FAILURE);
            }
            if *libc::__errno_location() != -expected_ret as c_int {
                libc::perror(c"recv".as_ptr());
                libc::exit(libc::EXIT_FAILURE);
            }
            return;
        }

        if ret < 0 {
            libc::perror(c"recv".as_ptr());
            libc::exit(libc::EXIT_FAILURE);
        }

        if nread != expected_ret {
            if ret == 0 {
                libc::fprintf(libc::stderr, c"unexpected EOF while receiving bytes\n".as_ptr());
            }

            libc::fprintf(
                libc::stderr,
                c"bogus recv(2) bytes read %zd (expected %zd)\n".as_ptr(),
                nread,
                expected_ret,
            );
            libc::exit(libc::EXIT_FAILURE);
        }
    }
}

/* Transmit one byte and check the return value.
 *
 * expected_ret:
 *  <0 Negative errno (for testing errors)
 *   0 End-of-file
 *   1 Success
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn send_byte(fd: c_int, expected_ret: c_int, flags: c_int) {
    static BYTE: u8 = b'A';

    unsafe {
        send_buf(
            fd,
            &BYTE as *const u8 as *const c_void,
            std::mem::size_of_val(&BYTE),
            flags,
            expected_ret as ssize_t,
        );
    }
}

/* Receive one byte and check the return value.
 *
 * expected_ret:
 *  <0 Negative errno (for testing errors)
 *   0 End-of-file
 *   1 Success
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn recv_byte(fd: c_int, expected_ret: c_int, flags: c_int) {
    let mut byte: u8 = 0;

    unsafe {
        recv_buf(
            fd,
            &mut byte as *mut u8 as *mut c_void,
            std::mem::size_of_val(&byte),
            flags,
            expected_ret as ssize_t,
        );

        if byte != b'A' {
            libc::fprintf(libc::stderr, c"unexpected byte read 0x%02x\n".as_ptr(), byte as c_int);
            libc::exit(libc::EXIT_FAILURE);
        }
    }
}

/* Run test cases.  The program terminates if a failure occurs. */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn run_tests(test_cases: *const test_case, opts: *const test_opts) {
    let mut i: c_int = 0;

    unsafe {
        while !(*test_cases.offset(i as isize)).name.is_null() {
            let run: Option<unsafe extern "C" fn(*const test_opts)>;
            let line: *mut c_char;

            libc::printf(c"%d - %s...".as_ptr(), i, (*test_cases.offset(i as isize)).name);
            libc::fflush(libc::stdout);

            /* Full barrier before executing the next test.  This
             * ensures that client and server are executing the
             * same test case.  In particular, it means whoever is
             * faster will not see the peer still executing the
             * last test.  This is important because port numbers
             * can be used by multiple test cases.
             */
            if (*test_cases.offset(i as isize)).skip {
                control_writeln(c"SKIP".as_ptr());
            } else {
                control_writeln(c"NEXT".as_ptr());
            }

            line = control_readln();
            if control_cmpln(line, c"SKIP".as_ptr(), false)
                || (*test_cases.offset(i as isize)).skip
            {
                libc::printf(c"skipped\n".as_ptr());

                libc::free(line as *mut c_void);
                i += 1;
                continue;
            }

            control_cmpln(line, c"NEXT".as_ptr(), true);
            libc::free(line as *mut c_void);

            if (*opts).mode == TEST_MODE_CLIENT {
                run = (*test_cases.offset(i as isize)).run_client;
            } else {
                run = (*test_cases.offset(i as isize)).run_server;
            }

            if let Some(run_fn) = run {
                run_fn(opts);
            }

            libc::printf(c"ok\n".as_ptr());
            i += 1;
        }

        libc::printf(c"All tests have been executed. Waiting other peer...".as_ptr());
        libc::fflush(libc::stdout);

        /*
         * Final full barrier, to ensure that all tests have been run and
         * that even the last one has been successful on both sides.
         */
        control_writeln(c"COMPLETED".as_ptr());
        control_expectln(c"COMPLETED".as_ptr());

        libc::printf(c"ok\n".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn list_tests(test_cases: *const test_case) {
    let mut i: c_int = 0;

    unsafe {
        libc::printf(c"ID\tTest name\n".as_ptr());

        while !(*test_cases.offset(i as isize)).name.is_null() {
            libc::printf(c"%d\t%s\n".as_ptr(), i, (*test_cases.offset(i as isize)).name);
            i += 1;
        }

        libc::exit(libc::EXIT_FAILURE);
    }
}

unsafe fn parse_test_id(test_id_str: *const c_char, test_cases_len: size_t) -> c_ulong {
    let test_id: c_ulong;
    let mut endptr: *mut c_char = std::ptr::null_mut();

    unsafe {
        *libc::__errno_location() = 0;
        test_id = libc::strtoul(test_id_str, &mut endptr, 10);
        if *libc::__errno_location() != 0 || *endptr != 0 {
            libc::fprintf(
                libc::stderr,
                c"malformed test ID \"%s\"\n".as_ptr(),
                test_id_str,
            );
            libc::exit(libc::EXIT_FAILURE);
        }

        if test_id >= test_cases_len as c_ulong {
            libc::fprintf(
                libc::stderr,
                c"test ID (%lu) larger than the max allowed (%lu)\n".as_ptr(),
                test_id,
                test_cases_len as c_ulong - 1,
            );
            libc::exit(libc::EXIT_FAILURE);
        }
    }

    test_id
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn skip_test(
    test_cases: *mut test_case,
    test_cases_len: size_t,
    test_id_str: *const c_char,
) {
    let test_id = unsafe { parse_test_id(test_id_str, test_cases_len) };
    unsafe {
        (*test_cases.offset(test_id as isize)).skip = true;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pick_test(
    test_cases: *mut test_case,
    test_cases_len: size_t,
    test_id_str: *const c_char,
) {
    static mut SKIP_ALL: bool = true;
    let test_id: c_ulong;

    unsafe {
        if SKIP_ALL {
            let mut i: c_ulong = 0;

            while i < test_cases_len as c_ulong {
                (*test_cases.offset(i as isize)).skip = true;
                i += 1;
            }

            SKIP_ALL = false;
        }

        test_id = parse_test_id(test_id_str, test_cases_len);
        (*test_cases.offset(test_id as isize)).skip = false;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hash_djb2(data: *const c_void, len: size_t) -> c_ulong {
    let mut hash: c_ulong = 5381;
    let mut i: c_int = 0;

    unsafe {
        while (i as size_t) < len {
            hash = ((hash << 5).wrapping_add(hash))
                .wrapping_add(*(data as *const u8).offset(i as isize) as c_ulong);
            i += 1;
        }
    }

    hash
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iovec_bytes(iov: *const iovec, iovnum: size_t) -> size_t {
    let mut bytes: size_t = 0;
    let mut i: c_int = 0;

    unsafe {
        while (i as size_t) < iovnum {
            bytes += (*iov.offset(i as isize)).iov_len;
            i += 1;
        }
    }

    bytes
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iovec_hash_djb2(iov: *const iovec, iovnum: size_t) -> c_ulong {
    let hash: c_ulong;
    let iov_bytes: size_t;
    let mut offs: size_t;
    let tmp: *mut c_void;
    let mut i: c_int;

    unsafe {
        iov_bytes = iovec_bytes(iov, iovnum);

        tmp = libc::malloc(iov_bytes);
        if tmp.is_null() {
            libc::perror(c"malloc".as_ptr());
            libc::exit(libc::EXIT_FAILURE);
        }

        offs = 0;
        i = 0;
        while (i as size_t) < iovnum {
            libc::memcpy(
                (tmp as *mut u8).add(offs) as *mut c_void,
                (*iov.offset(i as isize)).iov_base,
                (*iov.offset(i as isize)).iov_len,
            );
            offs += (*iov.offset(i as isize)).iov_len;
            i += 1;
        }

        hash = hash_djb2(tmp, iov_bytes);
        libc::free(tmp);
    }

    hash
}

/* Allocates and returns new 'struct iovec *' according pattern
 * in the 'test_iovec'. For each element in the 'test_iovec' it
 * allocates new element in the resulting 'iovec'. 'iov_len'
 * of the new element is copied from 'test_iovec'. 'iov_base' is
 * allocated depending on the 'iov_base' of 'test_iovec':
 *
 * 'iov_base' == NULL -> valid buf: mmap('iov_len').
 *
 * 'iov_base' == MAP_FAILED -> invalid buf:
 *               mmap('iov_len'), then munmap('iov_len').
 *               'iov_base' still contains result of
 *               mmap().
 *
 * 'iov_base' == number -> unaligned valid buf:
 *               mmap('iov_len') + number.
 *
 * 'iovnum' is number of elements in 'test_iovec'.
 *
 * Returns new 'iovec' or calls 'exit()' on error.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn alloc_test_iovec(test_iovec: *const iovec, iovnum: c_int) -> *mut iovec {
    let iovec_: *mut iovec;
    let mut i: c_int;

    unsafe {
        iovec_ = libc::malloc(std::mem::size_of::<iovec>() * iovnum as usize) as *mut iovec;
        if iovec_.is_null() {
            libc::perror(c"malloc".as_ptr());
            libc::exit(libc::EXIT_FAILURE);
        }

        i = 0;
        while i < iovnum {
            (*iovec_.offset(i as isize)).iov_len = (*test_iovec.offset(i as isize)).iov_len;

            (*iovec_.offset(i as isize)).iov_base = libc::mmap(
                std::ptr::null_mut(),
                (*iovec_.offset(i as isize)).iov_len,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_POPULATE,
                -1,
                0,
            );
            if (*iovec_.offset(i as isize)).iov_base == libc::MAP_FAILED {
                libc::perror(c"mmap".as_ptr());
                libc::exit(libc::EXIT_FAILURE);
            }

            if (*test_iovec.offset(i as isize)).iov_base != libc::MAP_FAILED {
                (*iovec_.offset(i as isize)).iov_base = ((*iovec_.offset(i as isize)).iov_base
                    as *mut u8)
                    .add((*test_iovec.offset(i as isize)).iov_base as usize)
                    as *mut c_void;
            }
            i += 1;
        }

        /* Unmap "invalid" elements. */
        i = 0;
        while i < iovnum {
            if (*test_iovec.offset(i as isize)).iov_base == libc::MAP_FAILED {
                if libc::munmap(
                    (*iovec_.offset(i as isize)).iov_base,
                    (*iovec_.offset(i as isize)).iov_len,
                ) != 0
                {
                    libc::perror(c"munmap".as_ptr());
                    libc::exit(libc::EXIT_FAILURE);
                }
            }
            i += 1;
        }

        i = 0;
        while i < iovnum {
            let mut j: c_int;

            if (*test_iovec.offset(i as isize)).iov_base == libc::MAP_FAILED {
                i += 1;
                continue;
            }

            j = 0;
            while (j as size_t) < (*iovec_.offset(i as isize)).iov_len {
                *((*iovec_.offset(i as isize)).iov_base as *mut u8).offset(j as isize) =
                    (libc::rand() & 0xff) as u8;
                j += 1;
            }
            i += 1;
        }
    }

    iovec_
}

/* Frees 'iovec *', previously allocated by 'alloc_test_iovec()'.
 * On error calls 'exit()'.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_test_iovec(
    test_iovec: *const iovec,
    iovec_: *mut iovec,
    iovnum: c_int,
) {
    let mut i: c_int = 0;

    unsafe {
        while i < iovnum {
            if (*test_iovec.offset(i as isize)).iov_base != libc::MAP_FAILED {
                if !(*test_iovec.offset(i as isize)).iov_base.is_null() {
                    (*iovec_.offset(i as isize)).iov_base = ((*iovec_.offset(i as isize)).iov_base
                        as *mut u8)
                        .sub((*test_iovec.offset(i as isize)).iov_base as usize)
                        as *mut c_void;
                }

                if libc::munmap(
                    (*iovec_.offset(i as isize)).iov_base,
                    (*iovec_.offset(i as isize)).iov_len,
                ) != 0
                {
                    libc::perror(c"munmap".as_ptr());
                    libc::exit(libc::EXIT_FAILURE);
                }
            }
            i += 1;
        }

        libc::free(iovec_ as *mut c_void);
    }
}

/* Set "unsigned long long" socket option and check that it's indeed set */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn setsockopt_ull_check(
    fd: c_int,
    level: c_int,
    optname: c_int,
    val: libc::c_ulonglong,
    errmsg: *const c_char,
) {
    let mut chkval: libc::c_ulonglong;
    let mut chklen: socklen_t;
    let mut err: c_int;

    unsafe {
        err = libc::setsockopt(
            fd,
            level,
            optname,
            &val as *const libc::c_ulonglong as *const c_void,
            std::mem::size_of_val(&val) as socklen_t,
        );
        if err != 0 {
            libc::fprintf(
                libc::stderr,
                c"setsockopt err: %s (%d)\n".as_ptr(),
                libc::strerror(*libc::__errno_location()),
                *libc::__errno_location(),
            );
            libc::fprintf(libc::stderr, c"%s  val %llu\n".as_ptr(), errmsg, val);
            libc::exit(libc::EXIT_FAILURE);
        }

        chkval = !val; /* just make storage != val */
        chklen = std::mem::size_of_val(&chkval) as socklen_t;

        err = libc::getsockopt(
            fd,
            level,
            optname,
            &mut chkval as *mut libc::c_ulonglong as *mut c_void,
            &mut chklen,
        );
        if err != 0 {
            libc::fprintf(
                libc::stderr,
                c"getsockopt err: %s (%d)\n".as_ptr(),
                libc::strerror(*libc::__errno_location()),
                *libc::__errno_location(),
            );
            libc::fprintf(libc::stderr, c"%s  val %llu\n".as_ptr(), errmsg, val);
            libc::exit(libc::EXIT_FAILURE);
        }

        if chklen as usize != std::mem::size_of_val(&chkval) {
            libc::fprintf(
                libc::stderr,
                c"size mismatch: set %zu got %d\n".as_ptr(),
                std::mem::size_of_val(&val),
                chklen,
            );
            libc::fprintf(libc::stderr, c"%s  val %llu\n".as_ptr(), errmsg, val);
            libc::exit(libc::EXIT_FAILURE);
        }

        if chkval != val {
            libc::fprintf(
                libc::stderr,
                c"value mismatch: set %llu got %llu\n".as_ptr(),
                val,
                chkval,
            );
            libc::fprintf(libc::stderr, c"%s  val %llu\n".as_ptr(), errmsg, val);
            libc::exit(libc::EXIT_FAILURE);
        }
    }
}

/* Set "int" socket option and check that it's indeed set */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn setsockopt_int_check(
    fd: c_int,
    level: c_int,
    optname: c_int,
    val: c_int,
    errmsg: *const c_char,
) {
    let mut chkval: c_int;
    let mut chklen: socklen_t;
    let mut err: c_int;

    unsafe {
        err = libc::setsockopt(
            fd,
            level,
            optname,
            &val as *const c_int as *const c_void,
            std::mem::size_of_val(&val) as socklen_t,
        );
        if err != 0 {
            libc::fprintf(
                libc::stderr,
                c"setsockopt err: %s (%d)\n".as_ptr(),
                libc::strerror(*libc::__errno_location()),
                *libc::__errno_location(),
            );
            libc::fprintf(libc::stderr, c"%s val %d\n".as_ptr(), errmsg, val);
            libc::exit(libc::EXIT_FAILURE);
        }

        chkval = !val; /* just make storage != val */
        chklen = std::mem::size_of_val(&chkval) as socklen_t;

        err = libc::getsockopt(
            fd,
            level,
            optname,
            &mut chkval as *mut c_int as *mut c_void,
            &mut chklen,
        );
        if err != 0 {
            libc::fprintf(
                libc::stderr,
                c"getsockopt err: %s (%d)\n".as_ptr(),
                libc::strerror(*libc::__errno_location()),
                *libc::__errno_location(),
            );
            libc::fprintf(libc::stderr, c"%s val %d\n".as_ptr(), errmsg, val);
            libc::exit(libc::EXIT_FAILURE);
        }

        if chklen as usize != std::mem::size_of_val(&chkval) {
            libc::fprintf(
                libc::stderr,
                c"size mismatch: set %zu got %d\n".as_ptr(),
                std::mem::size_of_val(&val),
                chklen,
            );
            libc::fprintf(libc::stderr, c"%s val %d\n".as_ptr(), errmsg, val);
            libc::exit(libc::EXIT_FAILURE);
        }

        if chkval != val {
            libc::fprintf(
                libc::stderr,
                c"value mismatch: set %d got %d\n".as_ptr(),
                val,
                chkval,
            );
            libc::fprintf(libc::stderr, c"%s val %d\n".as_ptr(), errmsg, val);
            libc::exit(libc::EXIT_FAILURE);
        }
    }
}

unsafe fn mem_invert(mem: *mut u8, size: size_t) {
    let mut i: size_t = 0;

    unsafe {
        while i < size {
            *mem.add(i) = !*mem.add(i);
            i += 1;
        }
    }
}

/* Set "timeval" socket option and check that it's indeed set */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn setsockopt_timeval_check(
    fd: c_int,
    level: c_int,
    optname: c_int,
    val: timeval,
    errmsg: *const c_char,
) {
    let mut chkval: timeval;
    let mut chklen: socklen_t;
    let mut err: c_int;

    unsafe {
        err = libc::setsockopt(
            fd,
            level,
            optname,
            &val as *const timeval as *const c_void,
            std::mem::size_of_val(&val) as socklen_t,
        );
        if err != 0 {
            libc::fprintf(
                libc::stderr,
                c"setsockopt err: %s (%d)\n".as_ptr(),
                libc::strerror(*libc::__errno_location()),
                *libc::__errno_location(),
            );
            libc::fprintf(
                libc::stderr,
                c"%s val %ld:%ld\n".as_ptr(),
                errmsg,
                val.tv_sec,
                val.tv_usec,
            );
            libc::exit(libc::EXIT_FAILURE);
        }

        /* just make storage != val */
        chkval = val;
        mem_invert(
            &mut chkval as *mut timeval as *mut u8,
            std::mem::size_of_val(&chkval),
        );
        chklen = std::mem::size_of_val(&chkval) as socklen_t;

        err = libc::getsockopt(
            fd,
            level,
            optname,
            &mut chkval as *mut timeval as *mut c_void,
            &mut chklen,
        );
        if err != 0 {
            libc::fprintf(
                libc::stderr,
                c"getsockopt err: %s (%d)\n".as_ptr(),
                libc::strerror(*libc::__errno_location()),
                *libc::__errno_location(),
            );
            libc::fprintf(
                libc::stderr,
                c"%s val %ld:%ld\n".as_ptr(),
                errmsg,
                val.tv_sec,
                val.tv_usec,
            );
            libc::exit(libc::EXIT_FAILURE);
        }

        if chklen as usize != std::mem::size_of_val(&chkval) {
            libc::fprintf(
                libc::stderr,
                c"size mismatch: set %zu got %d\n".as_ptr(),
                std::mem::size_of_val(&val),
                chklen,
            );
            libc::fprintf(
                libc::stderr,
                c"%s val %ld:%ld\n".as_ptr(),
                errmsg,
                val.tv_sec,
                val.tv_usec,
            );
            libc::exit(libc::EXIT_FAILURE);
        }

        if libc::memcmp(
            &chkval as *const timeval as *const c_void,
            &val as *const timeval as *const c_void,
            std::mem::size_of_val(&val),
        ) != 0
        {
            libc::fprintf(
                libc::stderr,
                c"value mismatch: set %ld:%ld got %ld:%ld\n".as_ptr(),
                val.tv_sec,
                val.tv_usec,
                chkval.tv_sec,
                chkval.tv_usec,
            );
            libc::fprintf(
                libc::stderr,
                c"%s val %ld:%ld\n".as_ptr(),
                errmsg,
                val.tv_sec,
                val.tv_usec,
            );
            libc::exit(libc::EXIT_FAILURE);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn enable_so_zerocopy_check(fd: c_int) {
    unsafe {
        setsockopt_int_check(
            fd,
            libc::SOL_SOCKET,
            libc::SO_ZEROCOPY,
            1,
            c"setsockopt SO_ZEROCOPY".as_ptr(),
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn enable_so_linger(fd: c_int, timeout: c_int) {
    let optval = linger {
        l_onoff: 1,
        l_linger: timeout,
    };

    unsafe {
        if libc::setsockopt(
            fd,
            libc::SOL_SOCKET,
            libc::SO_LINGER,
            &optval as *const linger as *const c_void,
            std::mem::size_of_val(&optval) as socklen_t,
        ) != 0
        {
            libc::perror(c"setsockopt(SO_LINGER)".as_ptr());
            libc::exit(libc::EXIT_FAILURE);
        }
    }
}

unsafe fn __get_transports() -> c_int {
    let mut buf = [0 as c_char; KALLSYMS_LINE_LEN];
    let mut ksym: *const c_char;
    let mut ret: c_int = 0;
    let f: *mut FILE;

    unsafe {
        f = libc::fopen(KALLSYMS_PATH.as_ptr() as *const c_char, c"r".as_ptr());
        if f.is_null() {
            libc::perror(c"Can't open /proc/kallsyms".as_ptr());
            libc::exit(libc::EXIT_FAILURE);
        }

        while !libc::fgets(buf.as_mut_ptr(), buf.len() as c_int, f).is_null() {
            let mut match_: *mut c_char;
            let mut i: c_int;

            assert!(buf[libc::strlen(buf.as_ptr()) - 1] == b'\n' as c_char);

            i = 0;
            while i < TRANSPORT_NUM {
                if ret & bit(i) != 0 {
                    i += 1;
                    continue;
                }

                /* Match should be followed by '\t' or '\n'.
                 * See kallsyms.c:s_show().
                 */
                ksym = transport_ksyms[i as usize];
                match_ = libc::strstr(buf.as_ptr(), ksym);
                if !match_.is_null()
                    && libc::isspace(*match_.add(libc::strlen(ksym)) as c_int) != 0
                {
                    ret |= bit(i);
                    break;
                }
                i += 1;
            }
        }

        libc::fclose(f);
    }

    ret
}

/* Return integer with TRANSPORT_* bit set for every (known) registered vsock
 * transport.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_transports() -> c_int {
    static mut TR: c_int = -1;

    unsafe {
        if TR == -1 {
            TR = __get_transports();
        }

        TR
    }
}
