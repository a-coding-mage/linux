// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2018 Google Inc.
 * Author: Eric Dumazet (edumazet@google.com)
 *
 * Reference program demonstrating tcp mmap() usage,
 * and SO_RCVLOWAT hints for receiver.
 *
 * Note : NIC with header split is needed to use mmap() on TCP :
 * Each incoming frame must be a multiple of PAGE_SIZE bytes of TCP payload.
 *
 * How to use on loopback interface :
 *
 *  ifconfig lo mtu 61512  # 15*4096 + 40 (ipv6 header) + 32 (TCP with TS option header)
 *  tcp_mmap -s -z &
 *  tcp_mmap -H ::1 -z
 *
 *  Or leave default lo mtu, but use -M option to set TCP_MAXSEG option to (4096 + 12)
 *      (4096 : page size on x86, 12: TCP TS option length)
 *  tcp_mmap -s -z -M $((4096+12)) &
 *  tcp_mmap -H ::1 -z -M $((4096+12))
 *
 * Note: -z option on sender uses MSG_ZEROCOPY, which forces a copy when packets go through loopback interface.
 *       We might use sendfile() instead, but really this test program is about mmap(), for receivers ;)
 *
 * $ ./tcp_mmap -s &                                 # Without mmap()
 * $ for i in {1..4}; do ./tcp_mmap -H ::1 -z ; done
 * received 32768 MB (0 % mmap'ed) in 14.1157 s, 19.4732 Gbit
 *   cpu usage user:0.057 sys:7.815, 240.234 usec per MB, 65531 c-switches
 * received 32768 MB (0 % mmap'ed) in 14.6833 s, 18.7204 Gbit
 *  cpu usage user:0.043 sys:8.103, 248.596 usec per MB, 65524 c-switches
 * received 32768 MB (0 % mmap'ed) in 11.143 s, 24.6682 Gbit
 *   cpu usage user:0.044 sys:6.576, 202.026 usec per MB, 65519 c-switches
 * received 32768 MB (0 % mmap'ed) in 14.9056 s, 18.4413 Gbit
 *   cpu usage user:0.036 sys:8.193, 251.129 usec per MB, 65530 c-switches
 * $ kill %1   # kill tcp_mmap server
 *
 * $ ./tcp_mmap -s -z &                              # With mmap()
 * $ for i in {1..4}; do ./tcp_mmap -H ::1 -z ; done
 * received 32768 MB (99.9939 % mmap'ed) in 6.73792 s, 40.7956 Gbit
 *   cpu usage user:0.045 sys:2.827, 87.6465 usec per MB, 65532 c-switches
 * received 32768 MB (99.9939 % mmap'ed) in 7.26732 s, 37.8238 Gbit
 *   cpu usage user:0.037 sys:3.087, 95.3369 usec per MB, 65532 c-switches
 * received 32768 MB (99.9939 % mmap'ed) in 7.61661 s, 36.0893 Gbit
 *   cpu usage user:0.046 sys:3.559, 110.016 usec per MB, 65529 c-switches
 * received 32768 MB (99.9939 % mmap'ed) in 7.43764 s, 36.9577 Gbit
 *   cpu usage user:0.035 sys:3.467, 106.873 usec per MB, 65530 c-switches
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(static_mut_refs)]

use core::arch::asm;
use core::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type size_t = usize;
type ssize_t = isize;
type socklen_t = u32;
type pthread_t = c_ulong;

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const PF_INET: c_int = AF_INET;
const PF_INET6: c_int = AF_INET6;
const SOCK_STREAM: c_int = 1;
const SOL_SOCKET: c_int = 1;
const SO_REUSEADDR: c_int = 2;
const SO_RCVBUF: c_int = 8;
const SO_SNDBUF: c_int = 7;
const SO_RCVLOWAT: c_int = 18;
const SO_ZEROCOPY: c_int = 60;
const SO_MAX_PACING_RATE: c_int = 47;
const IPPROTO_TCP: c_int = 6;
const TCP_MAXSEG: c_int = 2;
const TCP_INFO: c_int = 11;
const TCP_ZEROCOPY_RECEIVE: c_int = 35;
const O_RDONLY: c_int = 0;
const O_NDELAY: c_int = 0o4000;
const F_SETFL: c_int = 4;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_HUGETLB: c_int = 0x40000;
const MAP_POPULATE: c_int = 0x8000;
const MADV_DONTNEED: c_int = 4;
const POLLIN: i16 = 0x0001;
const RUSAGE_THREAD: c_int = 1;
const PTHREAD_CREATE_DETACHED: c_int = 1;
const SHA256_DIGEST_LENGTH: usize = 32;
const MSG_ZEROCOPY: c_int = 0x4000000;
const FILE_SZ: u64 = 1u64 << 35;

const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

#[repr(C)]
struct in_addr {
    s_addr: u32,
}

#[repr(C)]
struct in6_addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_in {
    sin_family: u16,
    sin_port: u16,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
}

#[repr(C)]
struct sockaddr_in6 {
    sin6_family: u16,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: in6_addr,
    sin6_scope_id: u32,
}

#[repr(C)]
struct sockaddr_storage {
    ss_family: u16,
    __ss_padding: [u8; 118],
    __ss_align: c_ulong,
}

#[repr(C)]
struct pollfd {
    fd: c_int,
    events: i16,
    revents: i16,
}

#[repr(C)]
struct timeval {
    tv_sec: c_long,
    tv_usec: c_long,
}

#[repr(C)]
struct rusage {
    ru_utime: timeval,
    ru_stime: timeval,
    ru_maxrss: c_long,
    ru_ixrss: c_long,
    ru_idrss: c_long,
    ru_isrss: c_long,
    ru_minflt: c_long,
    ru_majflt: c_long,
    ru_nswap: c_long,
    ru_inblock: c_long,
    ru_oublock: c_long,
    ru_msgsnd: c_long,
    ru_msgrcv: c_long,
    ru_nsignals: c_long,
    ru_nvcsw: c_long,
    ru_nivcsw: c_long,
}

#[repr(C)]
struct pthread_attr_t {
    __size: [u8; 56],
    __align: c_long,
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct EVP_MD_CTX {
    _private: [u8; 0],
}

#[repr(C)]
struct EVP_MD {
    _private: [u8; 0],
}

#[repr(C)]
struct tcp_info {
    tcpi_state: u8,
    tcpi_ca_state: u8,
    tcpi_retransmits: u8,
    tcpi_probes: u8,
    tcpi_backoff: u8,
    tcpi_options: u8,
    tcpi_wscale_bits: u8,
    tcpi_delivery_rate_app_limited_fastopen_client_fail: u8,
    tcpi_rto: u32,
    tcpi_ato: u32,
    tcpi_snd_mss: u32,
    tcpi_rcv_mss: u32,
}

#[repr(C)]
struct tcp_zerocopy_receive {
    address: u64,
    length: u32,
    recv_skip_hint: u32,
    inq: u32,
    err: i32,
    copybuf_address: u64,
    copybuf_len: i32,
    flags: u32,
}

unsafe extern "C" {
    static mut optarg: *mut c_char;

    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn madvise(addr: *mut c_void, length: size_t, advice: c_int) -> c_int;
    fn getsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *mut c_void,
        option_len: *mut socklen_t,
    ) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn bind(socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int;
    fn listen(socket: c_int, backlog: c_int) -> c_int;
    fn accept(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int;
    fn connect(socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int;
    fn send(socket: c_int, buffer: *const c_void, length: size_t, flags: c_int) -> ssize_t;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn getrusage(who: c_int, usage: *mut rusage) -> c_int;
    fn perror(s: *const c_char);
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...);
    fn exit(status: c_int) -> !;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn free(ptr: *mut c_void);
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn atol(nptr: *const c_char) -> c_long;
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    fn htons(hostshort: u16) -> u16;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn pthread_attr_init(attr: *mut pthread_attr_t) -> c_int;
    fn pthread_attr_setdetachstate(attr: *mut pthread_attr_t, detachstate: c_int) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const pthread_attr_t,
        start_routine: extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_exit(value_ptr: *mut c_void) -> !;
    fn EVP_MD_CTX_new() -> *mut EVP_MD_CTX;
    fn EVP_MD_CTX_free(ctx: *mut EVP_MD_CTX);
    fn EVP_sha256() -> *const EVP_MD;
    fn EVP_DigestInit_ex(ctx: *mut EVP_MD_CTX, type_: *const EVP_MD, impl_: *mut c_void) -> c_int;
    fn EVP_DigestUpdate(ctx: *mut EVP_MD_CTX, d: *const c_void, cnt: size_t) -> c_int;
    fn EVP_DigestFinal_ex(ctx: *mut EVP_MD_CTX, md: *mut u8, s: *mut c_uint) -> c_int;

    static mut stderr: *mut FILE;
    static mut errno: c_int;
}

static mut cfg_family: c_int = AF_INET6;
static mut cfg_alen: socklen_t = size_of::<sockaddr_in6>() as socklen_t;
static mut cfg_port: c_int = 8787;

static mut rcvbuf: c_int = 0; /* Default: autotuning.  Can be set with -r <integer> option */
static mut sndbuf: c_int = 0; /* Default: autotuning.  Can be set with -w <integer> option */
static mut zflg: c_int = 0; /* zero copy option. (MSG_ZEROCOPY for sender, mmap() for receiver */
static mut xflg: c_int = 0; /* hash received data (simple xor) (-h option) */
static mut keepflag: c_int = 0; /* -k option: receiver shall keep all received file in memory (no munmap() calls) */
static mut integrity: c_int = 0; /* -i option: sender and receiver compute sha256 over the data.*/

static mut chunk_size: size_t = 512 * 1024;

static mut map_align: size_t = 0;

static mut htotal: c_ulong = 0;
static mut digest_len: c_uint = 0;

#[inline]
unsafe fn prefetch(x: *const c_void) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        asm!("prefetcht0 [{0}]", in(reg) x, options(readonly, nostack, preserves_flags));
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = x;
    }
}

unsafe fn hash_zone(mut zone: *mut c_void, mut length: c_uint) {
    let mut temp: c_ulong = unsafe { htotal };

    while length >= (8 * size_of::<c_long>()) as c_uint {
        unsafe { prefetch((zone as *mut u8).add(384) as *const c_void) };
        unsafe {
            temp ^= *(zone as *mut c_ulong);
            temp ^= *((zone as *mut u8).add(size_of::<c_long>()) as *mut c_ulong);
            temp ^= *((zone as *mut u8).add(2 * size_of::<c_long>()) as *mut c_ulong);
            temp ^= *((zone as *mut u8).add(3 * size_of::<c_long>()) as *mut c_ulong);
            temp ^= *((zone as *mut u8).add(4 * size_of::<c_long>()) as *mut c_ulong);
            temp ^= *((zone as *mut u8).add(5 * size_of::<c_long>()) as *mut c_ulong);
            temp ^= *((zone as *mut u8).add(6 * size_of::<c_long>()) as *mut c_ulong);
            temp ^= *((zone as *mut u8).add(7 * size_of::<c_long>()) as *mut c_ulong);
            zone = (zone as *mut u8).add(8 * size_of::<c_long>()) as *mut c_void;
        }
        length -= (8 * size_of::<c_long>()) as c_uint;
    }
    while length >= 1 {
        unsafe {
            temp ^= *(zone as *mut u8) as c_ulong;
            zone = (zone as *mut u8).add(1) as *mut c_void;
        }
        length -= 1;
    }
    unsafe {
        htotal = temp;
    }
}

fn ALIGN_UP(x: size_t, align_to: size_t) -> size_t {
    (x + (align_to - 1)) & !(align_to - 1)
}

fn ALIGN_PTR_UP(p: *mut c_void, ptr_align_to: size_t) -> *mut c_void {
    ALIGN_UP(p as c_ulong as size_t, ptr_align_to) as *mut c_void
}

unsafe fn mmap_large_buffer(need: size_t, allocated: *mut size_t) -> *mut c_void {
    let mut buffer: *mut c_void;
    let mut sz: size_t;

    /* Attempt to use huge pages if possible. */
    sz = unsafe { ALIGN_UP(need, map_align) };
    buffer = unsafe {
        mmap(
            null_mut(),
            sz,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANONYMOUS | MAP_HUGETLB,
            -1,
            0,
        )
    };

    if buffer == MAP_FAILED {
        sz = need;
        buffer = unsafe {
            mmap(
                null_mut(),
                sz,
                PROT_READ | PROT_WRITE,
                MAP_PRIVATE | MAP_ANONYMOUS | MAP_POPULATE,
                -1,
                0,
            )
        };
        if buffer != MAP_FAILED {
            unsafe {
                fprintf(
                    stderr,
                    c"MAP_HUGETLB attempt failed, look at /sys/kernel/mm/hugepages for optimal performance\n".as_ptr(),
                );
            }
        }
    }
    unsafe {
        *allocated = sz;
    }
    buffer
}

unsafe fn tcp_info_get_rcv_mss(fd: c_int) -> u32 {
    let mut sz: socklen_t = size_of::<tcp_info>() as socklen_t;
    let mut info: tcp_info = unsafe { zeroed() };

    if unsafe {
        getsockopt(
            fd,
            IPPROTO_TCP,
            TCP_INFO,
            &mut info as *mut tcp_info as *mut c_void,
            &mut sz,
        )
    } != 0
    {
        unsafe {
            fprintf(stderr, c"Error fetching TCP_INFO\n".as_ptr());
        }
        return 0;
    }

    info.tcpi_rcv_mss
}

extern "C" fn child_thread(arg: *mut c_void) -> *mut c_void {
    unsafe {
        let mut digest: [u8; SHA256_DIGEST_LENGTH] = [0; SHA256_DIGEST_LENGTH];
        let mut total_mmap: c_ulong = 0;
        let mut total: c_ulong = 0;
        let mut zc: tcp_zerocopy_receive = zeroed();
        let mut buffer: *mut u8 = null_mut();
        let mut delta_usec: c_ulong;
        let mut ctx: *mut EVP_MD_CTX = null_mut();
        let flags: c_int = MAP_SHARED;
        let mut t0: timeval = zeroed();
        let mut t1: timeval = zeroed();
        let mut raddr: *mut c_void = null_mut();
        let mut addr: *mut c_void = null_mut();
        let mut throughput: c_double;
        let mut ru: rusage = zeroed();
        let mut buffer_sz: size_t = 0;
        let mut lu: c_int;
        let fd: c_int;

        fd = arg as c_ulong as c_int;

        gettimeofday(&mut t0, null_mut());

        fcntl(fd, F_SETFL, O_NDELAY);
        buffer = mmap_large_buffer(chunk_size, &mut buffer_sz) as *mut u8;
        if buffer as *mut c_void == MAP_FAILED {
            perror(c"mmap".as_ptr());
            goto_error(fd, ctx, buffer, buffer_sz, raddr);
        }
        if zflg != 0 {
            raddr = mmap(null_mut(), chunk_size + map_align, PROT_READ, flags, fd, 0);
            if raddr == MAP_FAILED {
                perror(c"mmap".as_ptr());
                zflg = 0;
            } else {
                addr = ALIGN_PTR_UP(raddr, map_align);
            }
        }
        if integrity != 0 {
            ctx = EVP_MD_CTX_new();
            if ctx.is_null() {
                perror(c"cannot enable SHA computing".as_ptr());
                goto_error(fd, ctx, buffer, buffer_sz, raddr);
            }
            EVP_DigestInit_ex(ctx, EVP_sha256(), null_mut());
        }
        loop {
            let mut pfd = pollfd {
                fd,
                events: POLLIN,
                revents: 0,
            };
            let mut sub: c_int;

            poll(&mut pfd, 1, 10000);
            if zflg != 0 {
                let mut zc_len: socklen_t = size_of::<tcp_zerocopy_receive>() as socklen_t;
                let res: c_int;

                memset(
                    &mut zc as *mut tcp_zerocopy_receive as *mut c_void,
                    0,
                    size_of::<tcp_zerocopy_receive>(),
                );
                zc.address = addr as c_ulong as u64;
                zc.length = core::cmp::min(chunk_size as u64, FILE_SZ - total as u64) as u32;

                res = getsockopt(
                    fd,
                    IPPROTO_TCP,
                    TCP_ZEROCOPY_RECEIVE,
                    &mut zc as *mut tcp_zerocopy_receive as *mut c_void,
                    &mut zc_len,
                );
                if res == -1 {
                    break;
                }

                if zc.length != 0 {
                    assert!(zc.length as size_t <= chunk_size);
                    if integrity != 0 {
                        EVP_DigestUpdate(ctx, addr, zc.length as size_t);
                    }
                    total_mmap = total_mmap.wrapping_add(zc.length as c_ulong);
                    if xflg != 0 {
                        hash_zone(addr, zc.length);
                    }
                    /* It is more efficient to unmap the pages right now,
                     * instead of doing this in next TCP_ZEROCOPY_RECEIVE.
                     */
                    madvise(addr, zc.length as size_t, MADV_DONTNEED);
                    total = total.wrapping_add(zc.length as c_ulong);
                }
                if zc.recv_skip_hint != 0 {
                    assert!(zc.recv_skip_hint as size_t <= chunk_size);
                    lu = read(
                        fd,
                        buffer as *mut c_void,
                        core::cmp::min(zc.recv_skip_hint as u64, FILE_SZ - total as u64) as size_t,
                    ) as c_int;
                    if lu > 0 {
                        if integrity != 0 {
                            EVP_DigestUpdate(ctx, buffer as *const c_void, lu as size_t);
                        }
                        if xflg != 0 {
                            hash_zone(buffer as *mut c_void, lu as c_uint);
                        }
                        total = total.wrapping_add(lu as c_ulong);
                    }
                    if lu == 0 {
                        break;
                    }
                }
                continue;
            }
            sub = 0;
            while (sub as size_t) < chunk_size {
                lu = read(
                    fd,
                    buffer.add(sub as size_t) as *mut c_void,
                    core::cmp::min(chunk_size - sub as size_t, FILE_SZ - total as u64) as size_t,
                ) as c_int;
                if lu == 0 {
                    break;
                }
                if lu < 0 {
                    break;
                }
                if integrity != 0 {
                    EVP_DigestUpdate(ctx, buffer.add(sub as size_t) as *const c_void, lu as size_t);
                }
                if xflg != 0 {
                    hash_zone(buffer.add(sub as size_t) as *mut c_void, lu as c_uint);
                }
                total = total.wrapping_add(lu as c_ulong);
                sub += lu;
            }
            if lu == 0 {
                break;
            }
        }
        gettimeofday(&mut t1, null_mut());
        delta_usec = ((t1.tv_sec - t0.tv_sec) * 1000000 + t1.tv_usec - t0.tv_usec) as c_ulong;

        if integrity != 0 {
            fcntl(fd, F_SETFL, 0);
            EVP_DigestFinal_ex(ctx, digest.as_mut_ptr(), &mut digest_len);
            lu = read(fd, buffer as *mut c_void, SHA256_DIGEST_LENGTH) as c_int;
            if lu != SHA256_DIGEST_LENGTH as c_int {
                perror(c"Error: Cannot read SHA256\n".as_ptr());
            }

            if memcmp(
                digest.as_ptr() as *const c_void,
                buffer as *const c_void,
                SHA256_DIGEST_LENGTH,
            ) != 0
            {
                fprintf(stderr, c"Error: SHA256 of the data is not right\n".as_ptr());
            } else {
                printf(c"\nSHA256 is correct\n".as_ptr());
            }
        }

        throughput = 0.0;
        if delta_usec != 0 {
            throughput = total as c_double * 8.0 / delta_usec as c_double / 1000.0;
        }
        getrusage(RUSAGE_THREAD, &mut ru);
        if total > 1024 * 1024 {
            let total_usec: c_ulong;
            let mb: c_ulong = total >> 20;
            total_usec = (1000000 * ru.ru_utime.tv_sec + ru.ru_utime.tv_usec
                + 1000000 * ru.ru_stime.tv_sec
                + ru.ru_stime.tv_usec) as c_ulong;
            printf(
                c"received %lg MB (%lg %% mmap'ed) in %lg s, %lg Gbit\n  cpu usage user:%lg sys:%lg, %lg usec per MB, %lu c-switches, rcv_mss %u\n".as_ptr(),
                total as c_double / (1024.0 * 1024.0),
                100.0 * total_mmap as c_double / total as c_double,
                delta_usec as c_double / 1000000.0,
                throughput,
                ru.ru_utime.tv_sec as c_double + ru.ru_utime.tv_usec as c_double / 1000000.0,
                ru.ru_stime.tv_sec as c_double + ru.ru_stime.tv_usec as c_double / 1000000.0,
                total_usec as c_double / mb as c_double,
                ru.ru_nvcsw as c_ulong,
                tcp_info_get_rcv_mss(fd),
            );
        }
        if !ctx.is_null() {
            EVP_MD_CTX_free(ctx);
        }
        munmap(buffer as *mut c_void, buffer_sz);
        close(fd);
        if zflg != 0 {
            munmap(raddr, chunk_size + map_align);
        }
        pthread_exit(null_mut());
    }
}

unsafe fn goto_error(
    fd: c_int,
    ctx: *mut EVP_MD_CTX,
    buffer: *mut u8,
    buffer_sz: size_t,
    raddr: *mut c_void,
) -> ! {
    unsafe {
        if !ctx.is_null() {
            EVP_MD_CTX_free(ctx);
        }
        munmap(buffer as *mut c_void, buffer_sz);
        close(fd);
        if zflg != 0 {
            munmap(raddr, chunk_size + map_align);
        }
        pthread_exit(null_mut());
    }
}

unsafe fn apply_rcvsnd_buf(fd: c_int) {
    unsafe {
        if rcvbuf != 0
            && setsockopt(
                fd,
                SOL_SOCKET,
                SO_RCVBUF,
                &rcvbuf as *const c_int as *const c_void,
                size_of::<c_int>() as socklen_t,
            ) == -1
        {
            perror(c"setsockopt SO_RCVBUF".as_ptr());
        }

        if sndbuf != 0
            && setsockopt(
                fd,
                SOL_SOCKET,
                SO_SNDBUF,
                &sndbuf as *const c_int as *const c_void,
                size_of::<c_int>() as socklen_t,
            ) == -1
        {
            perror(c"setsockopt SO_SNDBUF".as_ptr());
        }
    }
}

unsafe fn setup_sockaddr(domain: c_int, str_addr: *const c_char, sockaddr: *mut sockaddr_storage) {
    let addr6 = sockaddr as *mut sockaddr_in6;
    let addr4 = sockaddr as *mut sockaddr_in;

    unsafe {
        match domain {
            PF_INET => {
                memset(addr4 as *mut c_void, 0, size_of::<sockaddr_in>());
                (*addr4).sin_family = AF_INET as u16;
                (*addr4).sin_port = htons(cfg_port as u16);
                if !str_addr.is_null()
                    && inet_pton(
                        AF_INET,
                        str_addr,
                        &mut (*addr4).sin_addr as *mut in_addr as *mut c_void,
                    ) != 1
                {
                    error(1, 0, c"ipv4 parse error: %s".as_ptr(), str_addr);
                }
            }
            PF_INET6 => {
                memset(addr6 as *mut c_void, 0, size_of::<sockaddr_in6>());
                (*addr6).sin6_family = AF_INET6 as u16;
                (*addr6).sin6_port = htons(cfg_port as u16);
                if !str_addr.is_null()
                    && inet_pton(
                        AF_INET6,
                        str_addr,
                        &mut (*addr6).sin6_addr as *mut in6_addr as *mut c_void,
                    ) != 1
                {
                    error(1, 0, c"ipv6 parse error: %s".as_ptr(), str_addr);
                }
            }
            _ => {
                error(1, 0, c"illegal domain".as_ptr());
            }
        }
    }
}

unsafe fn do_accept(fdlisten: c_int) {
    unsafe {
        let mut attr: pthread_attr_t = zeroed();
        let mut rcvlowat: c_int;

        pthread_attr_init(&mut attr);
        pthread_attr_setdetachstate(&mut attr, PTHREAD_CREATE_DETACHED);

        rcvlowat = chunk_size as c_int;
        if setsockopt(
            fdlisten,
            SOL_SOCKET,
            SO_RCVLOWAT,
            &rcvlowat as *const c_int as *const c_void,
            size_of::<c_int>() as socklen_t,
        ) == -1
        {
            perror(c"setsockopt SO_RCVLOWAT".as_ptr());
        }

        apply_rcvsnd_buf(fdlisten);

        loop {
            let mut addr: sockaddr_in = zeroed();
            let mut addrlen: socklen_t = size_of::<sockaddr_in>() as socklen_t;
            let mut th: pthread_t = 0;
            let fd: c_int;
            let res: c_int;

            fd = accept(
                fdlisten,
                &mut addr as *mut sockaddr_in as *mut sockaddr,
                &mut addrlen,
            );
            if fd == -1 {
                perror(c"accept".as_ptr());
                continue;
            }
            res = pthread_create(
                &mut th,
                &attr,
                child_thread,
                fd as c_ulong as *mut c_void,
            );
            if res != 0 {
                errno = res;
                perror(c"pthread_create".as_ptr());
                close(fd);
            }
        }
    }
}

/* Each thread should reserve a big enough vma to avoid
 * spinlock collisions in ptl locks.
 * This size is 2MB on x86_64, and is exported in /proc/meminfo.
 */
unsafe fn default_huge_page_size() -> c_ulong {
    unsafe {
        let f = fopen(c"/proc/meminfo".as_ptr(), c"r".as_ptr());
        let mut hps: c_ulong = 0;
        let mut linelen: size_t = 0;
        let mut line: *mut c_char = null_mut();

        if f.is_null() {
            return 0;
        }
        while getline(&mut line, &mut linelen, f) > 0 {
            if sscanf(line, c"Hugepagesize:       %lu kB".as_ptr(), &mut hps) == 1 {
                hps <<= 10;
                break;
            }
        }
        free(line as *mut c_void);
        fclose(f);
        hps
    }
}

unsafe fn randomize(target: *mut c_void, count: size_t) {
    static mut urandom: c_int = -1;
    let got: ssize_t;

    unsafe {
        urandom = open(c"/dev/urandom".as_ptr(), O_RDONLY);
        if urandom < 0 {
            perror(c"open /dev/urandom".as_ptr());
            exit(1);
        }
        got = read(urandom, target, count);
        if got != count as ssize_t {
            perror(c"read /dev/urandom".as_ptr());
            exit(1);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    unsafe {
        let mut digest: [u8; SHA256_DIGEST_LENGTH] = [0; SHA256_DIGEST_LENGTH];
        let mut listenaddr: sockaddr_storage = zeroed();
        let mut addr: sockaddr_storage = zeroed();
        let mut max_pacing_rate: c_uint = 0;
        let mut ctx: *mut EVP_MD_CTX = null_mut();
        let buffer: *mut u8;
        let mut total: u64 = 0;
        let mut host: *mut c_char = null_mut();
        let fd: c_int;
        let mut c: c_int;
        let on: c_int = 1;
        let mut buffer_sz: size_t = 0;
        let mut sflg: c_int = 0;
        let mut mss: c_int = 0;

        loop {
            c = getopt(argc, argv, c"46p:svr:w:H:zxkP:M:C:a:i".as_ptr());
            if c == -1 {
                break;
            }
            match c as u8 as char {
                '4' => {
                    cfg_family = PF_INET;
                    cfg_alen = size_of::<sockaddr_in>() as socklen_t;
                }
                '6' => {
                    cfg_family = PF_INET6;
                    cfg_alen = size_of::<sockaddr_in6>() as socklen_t;
                }
                'p' => {
                    cfg_port = atoi(optarg);
                }
                'H' => {
                    host = optarg;
                }
                's' => {
                    /* server : listen for incoming connections */
                    sflg += 1;
                }
                'r' => {
                    rcvbuf = atoi(optarg);
                }
                'w' => {
                    sndbuf = atoi(optarg);
                }
                'z' => {
                    zflg = 1;
                }
                'M' => {
                    mss = atoi(optarg);
                }
                'x' => {
                    xflg = 1;
                }
                'k' => {
                    keepflag = 1;
                }
                'P' => {
                    max_pacing_rate = atoi(optarg) as c_uint;
                }
                'C' => {
                    chunk_size = atol(optarg) as size_t;
                }
                'a' => {
                    map_align = atol(optarg) as size_t;
                }
                'i' => {
                    integrity = 1;
                }
                _ => {
                    exit(1);
                }
            }
        }
        if map_align == 0 {
            map_align = default_huge_page_size() as size_t;
            /* if really /proc/meminfo is not helping,
             * we use the default x86_64 hugepagesize.
             */
            if map_align == 0 {
                map_align = 2 * 1024 * 1024;
            }
        }
        if sflg != 0 {
            let fdlisten = socket(cfg_family, SOCK_STREAM, 0);

            if fdlisten == -1 {
                perror(c"socket".as_ptr());
                exit(1);
            }
            apply_rcvsnd_buf(fdlisten);
            setsockopt(
                fdlisten,
                SOL_SOCKET,
                SO_REUSEADDR,
                &on as *const c_int as *const c_void,
                size_of::<c_int>() as socklen_t,
            );

            setup_sockaddr(cfg_family, host, &mut listenaddr);

            if mss != 0
                && setsockopt(
                    fdlisten,
                    IPPROTO_TCP,
                    TCP_MAXSEG,
                    &mss as *const c_int as *const c_void,
                    size_of::<c_int>() as socklen_t,
                ) == -1
            {
                perror(c"setsockopt TCP_MAXSEG".as_ptr());
                exit(1);
            }
            if bind(
                fdlisten,
                &listenaddr as *const sockaddr_storage as *const sockaddr,
                cfg_alen,
            ) == -1
            {
                perror(c"bind".as_ptr());
                exit(1);
            }
            if listen(fdlisten, 128) == -1 {
                perror(c"listen".as_ptr());
                exit(1);
            }
            do_accept(fdlisten);
        }

        buffer = mmap_large_buffer(chunk_size, &mut buffer_sz) as *mut u8;
        if buffer as *mut c_void == MAP_FAILED {
            perror(c"mmap".as_ptr());
            exit(1);
        }

        fd = socket(cfg_family, SOCK_STREAM, 0);
        if fd == -1 {
            perror(c"socket".as_ptr());
            exit(1);
        }
        apply_rcvsnd_buf(fd);

        setup_sockaddr(cfg_family, host, &mut addr);

        if mss != 0
            && setsockopt(
                fd,
                IPPROTO_TCP,
                TCP_MAXSEG,
                &mss as *const c_int as *const c_void,
                size_of::<c_int>() as socklen_t,
            ) == -1
        {
            perror(c"setsockopt TCP_MAXSEG".as_ptr());
            exit(1);
        }
        if connect(
            fd,
            &addr as *const sockaddr_storage as *const sockaddr,
            cfg_alen,
        ) == -1
        {
            perror(c"connect".as_ptr());
            exit(1);
        }
        if max_pacing_rate != 0
            && setsockopt(
                fd,
                SOL_SOCKET,
                SO_MAX_PACING_RATE,
                &max_pacing_rate as *const c_uint as *const c_void,
                size_of::<c_uint>() as socklen_t,
            ) == -1
        {
            perror(c"setsockopt SO_MAX_PACING_RATE".as_ptr());
        }

        if zflg != 0
            && setsockopt(
                fd,
                SOL_SOCKET,
                SO_ZEROCOPY,
                &on as *const c_int as *const c_void,
                size_of::<c_int>() as socklen_t,
            ) == -1
        {
            perror(c"setsockopt SO_ZEROCOPY, (-z option disabled)".as_ptr());
            zflg = 0;
        }
        if integrity != 0 {
            randomize(buffer as *mut c_void, buffer_sz);
            ctx = EVP_MD_CTX_new();
            if ctx.is_null() {
                perror(c"cannot enable SHA computing".as_ptr());
                exit(1);
            }
            EVP_DigestInit_ex(ctx, EVP_sha256(), null_mut());
        }
        while total < FILE_SZ {
            let offset: size_t = (total % chunk_size as u64) as size_t;
            let mut wr: i64 = (FILE_SZ - total) as i64;

            if wr > (chunk_size - offset) as i64 {
                wr = (chunk_size - offset) as i64;
            }
            /* Note : we just want to fill the pipe with random bytes */
            wr = send(
                fd,
                buffer.add(offset) as *const c_void,
                wr as size_t,
                if zflg != 0 { MSG_ZEROCOPY } else { 0 },
            ) as i64;
            if wr <= 0 {
                break;
            }
            if integrity != 0 {
                EVP_DigestUpdate(ctx, buffer.add(offset) as *const c_void, wr as size_t);
            }
            total = total.wrapping_add(wr as u64);
        }
        if integrity != 0 && total == FILE_SZ {
            EVP_DigestFinal_ex(ctx, digest.as_mut_ptr(), &mut digest_len);
            send(
                fd,
                digest.as_ptr() as *const c_void,
                SHA256_DIGEST_LENGTH as size_t,
                0,
            );
        }
        if !ctx.is_null() {
            EVP_MD_CTX_free(ctx);
        }
        close(fd);
        munmap(buffer as *mut c_void, buffer_sz);
        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
