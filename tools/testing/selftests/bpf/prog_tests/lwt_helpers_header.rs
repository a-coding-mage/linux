/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies preserved for Rust-side integration:
// <time.h>, <net/if.h>, <linux/icmp.h>, and "test_progs.h".

use core::ffi::{c_char, c_int, c_void};

pub type ssize_t = isize;

extern "C" {
    static mut errno: c_int;

    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn system(command: *const c_char) -> c_int;
    fn select(
        nfds: c_int,
        readfds: *mut fd_set,
        writefds: *mut fd_set,
        exceptfds: *mut fd_set,
        timeout: *mut timeval,
    ) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> ssize_t;

    static mut stderr: *mut c_void;
}

// Types, constants, and helpers supplied by the translated equivalents of the
// original C includes.
extern "C" {
    fn FD_ZERO(set: *mut fd_set);
    fn FD_SET(fd: c_int, set: *mut fd_set);

    fn test__start_subtest(name: *const c_char) -> bool;
    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut nstoken, name: *const c_char) -> bool;
    fn open_netns(name: *const c_char) -> *mut nstoken;
    fn close_netns(token: *mut nstoken);
}

extern "C" {
    static NETNS: *const c_char;
    static IPPROTO_ICMP: c_int;
    static ICMP_ECHO: u8;
    static EINTR: c_int;
    static EAGAIN: c_int;
}

#[repr(C)]
pub struct timeval {
    pub tv_sec: isize,
    pub tv_usec: isize,
}

#[repr(C)]
pub struct fd_set {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nstoken {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iphdr {
    pub ihl_version: u8,
    pub tos: u8,
    pub tot_len: u16,
    pub id: u16,
    pub frag_off: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub check: u16,
    pub saddr: u32,
    pub daddr: u32,
}

#[repr(C)]
pub struct icmphdr {
    pub type_: u8,
    pub code: u8,
    pub checksum: u16,
    pub un: [u8; 4],
}

macro_rules! log_err {
    ($msg:literal $(, $arg:expr)* $(,)?) => {{
        unsafe {
            fprintf(
                stderr,
                concat!("(%s:%d: errno: %s) ", $msg, "\n\0").as_ptr() as *const c_char,
                file!().as_ptr() as *const c_char,
                line!() as c_int,
                strerror(errno),
                $($arg),*
            )
        }
    }};
}

macro_rules! RUN_TEST {
    ($name:ident) => {{
        unsafe {
            if test__start_subtest(concat!(stringify!($name), "\0").as_ptr() as *const c_char) {
                if ASSERT_OK(netns_create(), b"netns_create\0".as_ptr() as *const c_char) {
                    let token: *mut nstoken = open_netns(NETNS);
                    if ASSERT_OK_PTR(token, b"setns\0".as_ptr() as *const c_char) {
                        test_ ## $name();
                        close_netns(token);
                    }
                    netns_delete();
                }
            }
        }
    }};
}

pub unsafe fn netns_create() -> c_int {
    let command = {
        let netns = core::ffi::CStr::from_ptr(NETNS);
        let mut bytes = b"ip netns add ".to_vec();
        bytes.extend_from_slice(netns.to_bytes());
        bytes.push(0);
        bytes
    };

    system(command.as_ptr() as *const c_char)
}

pub unsafe fn netns_delete() -> c_int {
    let command = {
        let netns = core::ffi::CStr::from_ptr(NETNS);
        let mut bytes = b"ip netns del ".to_vec();
        bytes.extend_from_slice(netns.to_bytes());
        bytes.extend_from_slice(b">/dev/null 2>&1\0");
        bytes
    };

    system(command.as_ptr() as *const c_char)
}

pub const ICMP_PAYLOAD_SIZE: ssize_t = 100;

/* Match an ICMP packet with payload len ICMP_PAYLOAD_SIZE */
pub unsafe fn __expect_icmp_ipv4(buf: *mut c_char, len: ssize_t) -> c_int {
    let ip: *mut iphdr = buf as *mut iphdr;
    let icmp: *mut icmphdr = ip.add(1) as *mut icmphdr;
    let min_header_len: ssize_t =
        (core::mem::size_of_val(&*ip) + core::mem::size_of_val(&*icmp)) as ssize_t;

    if len < min_header_len {
        return -1;
    }

    if (*ip).protocol != IPPROTO_ICMP as u8 {
        return -1;
    }

    if (*icmp).type_ != ICMP_ECHO {
        return -1;
    }

    (len == ICMP_PAYLOAD_SIZE + min_header_len) as c_int
}

pub type filter_t = Option<unsafe extern "C" fn(*mut c_char, ssize_t) -> c_int>;

/* wait_for_packet - wait for a packet that matches the filter
 *
 * @fd: tun fd/packet socket to read packet
 * @filter: filter function, returning 1 if matches
 * @timeout: timeout to wait for the packet
 *
 * Returns 1 if a matching packet is read, 0 if timeout expired, -1 on error.
 */
pub unsafe fn wait_for_packet(fd: c_int, filter: filter_t, timeout: *mut timeval) -> c_int {
    let mut buf: [c_char; 4096] = [0; 4096];
    let mut max_retry: c_int = 5; /* in case we read some spurious packets */
    let mut fds: fd_set = core::mem::zeroed();

    FD_ZERO(&mut fds);
    while {
        let old = max_retry;
        max_retry -= 1;
        old != 0
    } {
        /* Linux modifies timeout arg... So make a copy */
        let mut copied_timeout: timeval = *timeout;
        let mut ret: ssize_t = -1;

        FD_SET(fd, &mut fds);

        ret = select(
            1 + fd,
            &mut fds,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            &mut copied_timeout,
        ) as ssize_t;
        if ret <= 0 {
            if errno == EINTR {
                continue;
            } else if errno == EAGAIN || ret == 0 {
                return 0;
            }

            log_err!("select failed");
            return -1;
        }

        ret = read(fd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf));

        if ret <= 0 {
            log_err!("read(dev): %ld", ret);
            return -1;
        }

        if let Some(filter_fn) = filter {
            if filter_fn(buf.as_mut_ptr(), ret) > 0 {
                return 1;
            }
        }
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
