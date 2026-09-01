// SPDX-License-Identifier: GPL-2.0
// C dependencies: sched.h, unistd.h, stdio.h, errno.h, string.h, stdlib.h,
// stdint.h, sys/types.h, sys/socket.h

use std::ffi::{c_char, c_int, c_void};

type socklen_t = u32;

const AF_INET: c_int = 2;
const SOCK_STREAM: c_int = 1;
const SOL_SOCKET: c_int = 1;
const CLONE_NEWNET: c_int = 0x40000000;

const SO_NETNS_COOKIE: c_int = 71;

unsafe extern "C" {
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn getsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *mut c_void,
        option_len: *mut socklen_t,
    ) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
}

macro_rules! pr_err {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        eprintln!(
            "{}:{}:{}: {}",
            "main",
            line!(),
            format_args!($fmt $(, $arg)*),
            std::io::Error::last_os_error()
        );
        1
    }};
}

pub unsafe fn main(_argc: c_int, _argvp: *mut *mut c_char) -> c_int {
    let mut cookie1: u64;
    let mut cookie2: u64;
    let mut vallen: socklen_t;
    let sock1: c_int;
    let sock2: c_int;

    sock1 = unsafe { socket(AF_INET, SOCK_STREAM, 0) };
    if sock1 < 0 {
        return pr_err!("Unable to create TCP socket");
    }

    vallen = std::mem::size_of::<u64>() as socklen_t;
    if unsafe {
        getsockopt(
            sock1,
            SOL_SOCKET,
            SO_NETNS_COOKIE,
            (&mut cookie1 as *mut u64).cast::<c_void>(),
            &mut vallen,
        )
    } != 0
    {
        return pr_err!("getsockopt(SOL_SOCKET, SO_NETNS_COOKIE)");
    }

    if cookie1 == 0 {
        return pr_err!("SO_NETNS_COOKIE returned zero cookie");
    }

    if unsafe { unshare(CLONE_NEWNET) } != 0 {
        return pr_err!("unshare");
    }

    sock2 = unsafe { socket(AF_INET, SOCK_STREAM, 0) };
    if sock2 < 0 {
        return pr_err!("Unable to create TCP socket");
    }

    vallen = std::mem::size_of::<u64>() as socklen_t;
    if unsafe {
        getsockopt(
            sock2,
            SOL_SOCKET,
            SO_NETNS_COOKIE,
            (&mut cookie2 as *mut u64).cast::<c_void>(),
            &mut vallen,
        )
    } != 0
    {
        return pr_err!("getsockopt(SOL_SOCKET, SO_NETNS_COOKIE)");
    }

    if cookie2 == 0 {
        return pr_err!("SO_NETNS_COOKIE returned zero cookie");
    }

    if cookie1 == cookie2 {
        return pr_err!("SO_NETNS_COOKIE returned identical cookies for distinct ns");
    }

    unsafe {
        close(sock1);
        close(sock2);
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
