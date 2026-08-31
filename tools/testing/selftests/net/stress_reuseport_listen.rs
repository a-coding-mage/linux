// SPDX-License-Identifier: GPL-2.0

/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

/* Test listening on the same port 443 with multiple VIPS.
 * Each VIP:443 will have multiple sk listening on by using
 * SO_REUSEPORT.
 */

use libc::{
    c_char, c_int, c_void, free, malloc, size_t, sockaddr, sockaddr_in6, socklen_t, timespec,
    AF_INET6, CLOCK_MONOTONIC, SOCK_STREAM, SOL_SOCKET, SO_REUSEPORT,
};

const IP6_LADDR_START: &[u8] = b"2401:dead::1\0";
const IP6_LPORT: u16 = 443;
const NSEC_PER_SEC: libc::c_long = 1000000000;
const NSEC_PER_USEC: libc::c_long = 1000;

static mut NR_SOCKS_PER_VIP: libc::c_uint = 0;
static mut NR_VIPS: libc::c_uint = 0;

unsafe extern "C" {
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...) -> !;
    fn __errno_location() -> *mut c_int;
}

unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

unsafe fn bind_reuseport_sock6() -> *mut c_int {
    let mut lfds: *mut c_int;
    let mut cur_fd: *mut c_int;
    let mut err: c_int;
    let optvalue: c_int = 1;
    let mut sa6: sockaddr_in6 = unsafe { std::mem::zeroed() };
    let mut i: libc::c_uint;
    let mut j: libc::c_uint;

    sa6.sin6_family = AF_INET6 as libc::sa_family_t;
    sa6.sin6_port = unsafe { libc::htons(IP6_LPORT) };
    err = unsafe {
        libc::inet_pton(
            AF_INET6,
            IP6_LADDR_START.as_ptr() as *const c_char,
            &mut sa6.sin6_addr as *mut _ as *mut c_void,
        )
    };
    if err != 1 {
        unsafe {
            error(
                1,
                err,
                b"inet_pton(%s)\0".as_ptr() as *const c_char,
                IP6_LADDR_START.as_ptr() as *const c_char,
            )
        };
    }

    lfds = unsafe {
        malloc(
            (NR_VIPS as size_t)
                .wrapping_mul(NR_SOCKS_PER_VIP as size_t)
                .wrapping_mul(std::mem::size_of::<c_int>() as size_t),
        ) as *mut c_int
    };
    if lfds.is_null() {
        unsafe {
            error(
                1,
                errno(),
                b"cannot alloc array of lfds\0".as_ptr() as *const c_char,
            )
        };
    }

    cur_fd = lfds;
    i = 0;
    while i < unsafe { NR_VIPS } {
        j = 0;
        while j < unsafe { NR_SOCKS_PER_VIP } {
            unsafe {
                *cur_fd = libc::socket(AF_INET6, SOCK_STREAM, 0);
            }
            if unsafe { *cur_fd } == -1 {
                unsafe {
                    error(
                        1,
                        errno(),
                        b"lfds[%u,%u] = socket(AF_INET6)\0".as_ptr() as *const c_char,
                        i,
                        j,
                    )
                };
            }

            err = unsafe {
                libc::setsockopt(
                    *cur_fd,
                    SOL_SOCKET,
                    SO_REUSEPORT,
                    &optvalue as *const _ as *const c_void,
                    std::mem::size_of_val(&optvalue) as socklen_t,
                )
            };
            if err != 0 {
                unsafe {
                    error(
                        1,
                        errno(),
                        b"setsockopt(lfds[%u,%u], SO_REUSEPORT)\0".as_ptr() as *const c_char,
                        i,
                        j,
                    )
                };
            }

            err = unsafe {
                libc::bind(
                    *cur_fd,
                    &sa6 as *const _ as *const sockaddr,
                    std::mem::size_of_val(&sa6) as socklen_t,
                )
            };
            if err != 0 {
                unsafe {
                    error(
                        1,
                        errno(),
                        b"bind(lfds[%u,%u])\0".as_ptr() as *const c_char,
                        i,
                        j,
                    )
                };
            }
            cur_fd = unsafe { cur_fd.add(1) };
            j += 1;
        }
        unsafe {
            let s6_addr32 = (&mut sa6.sin6_addr as *mut _ as *mut u32).add(3);
            *s6_addr32 = (*s6_addr32).wrapping_add(1);
        }
        i += 1;
    }

    lfds
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *const *const c_char) -> c_int {
    let mut start_ts: timespec = unsafe { std::mem::zeroed() };
    let mut end_ts: timespec = unsafe { std::mem::zeroed() };
    let start_ns: libc::c_ulong;
    let end_ns: libc::c_ulong;
    let nr_lsocks: libc::c_uint;
    let lfds: *mut c_int;
    let mut i: c_int;
    let mut err: c_int;

    if argc != 3
        || unsafe { libc::atoi(*argv.add(1)) } <= 0
        || unsafe { libc::atoi(*argv.add(2)) } <= 0
    {
        unsafe {
            error(
                1,
                0,
                b"Usage: %s <nr_vips> <nr_socks_per_vip>\n\0".as_ptr() as *const c_char,
                *argv.add(0),
            )
        };
    }

    unsafe {
        NR_VIPS = libc::atoi(*argv.add(1)) as libc::c_uint;
        NR_SOCKS_PER_VIP = libc::atoi(*argv.add(2)) as libc::c_uint;
        nr_lsocks = NR_VIPS.wrapping_mul(NR_SOCKS_PER_VIP);
    }
    lfds = unsafe { bind_reuseport_sock6() };

    unsafe {
        libc::clock_gettime(CLOCK_MONOTONIC, &mut start_ts);
    }
    i = 0;
    while (i as libc::c_uint) < nr_lsocks {
        err = unsafe { libc::listen(*lfds.add(i as usize), 0) };
        if err != 0 {
            unsafe {
                error(
                    1,
                    errno(),
                    b"listen(lfds[%d])\0".as_ptr() as *const c_char,
                    i,
                )
            };
        }
        i += 1;
    }
    unsafe {
        libc::clock_gettime(CLOCK_MONOTONIC, &mut end_ts);
    }

    start_ns = (start_ts.tv_sec * NSEC_PER_SEC + start_ts.tv_nsec) as libc::c_ulong;
    end_ns = (end_ts.tv_sec * NSEC_PER_SEC + end_ts.tv_nsec) as libc::c_ulong;

    unsafe {
        libc::printf(
            b"listen %d socks took %lu.%lu\n\0".as_ptr() as *const c_char,
            nr_lsocks,
            (end_ns - start_ns) / NSEC_PER_SEC as libc::c_ulong,
            (end_ns - start_ns) / NSEC_PER_USEC as libc::c_ulong,
        );
    }

    i = 0;
    while (i as libc::c_uint) < nr_lsocks {
        unsafe {
            libc::close(*lfds.add(i as usize));
        }
        i += 1;
    }

    unsafe {
        free(lfds as *mut c_void);
    }
    0
}
