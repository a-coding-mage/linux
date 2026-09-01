// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2024 ARM Limited.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

const DATA_SIZE: usize = 16 * 4096;

const AF_ALG: c_int = 38;
const SOCK_SEQPACKET: c_int = 5;
const EAFNOSUPPORT: c_int = 97;
const EAGAIN: c_int = 11;
const EXIT_FAILURE: c_int = 1;
const _IOLBF: c_int = 1;
const SA_RESTART: c_int = 0x10000000;
const SA_SIGINFO: c_int = 4;
const SIGTERM: c_int = 15;
const SIGUSR1: c_int = 10;
const SIGUSR2: c_int = 12;
const SPLICE_F_GIFT: c_uint = 0x08;

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct iovec {
    iov_base: *mut c_void,
    iov_len: usize,
}

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_alg {
    salg_family: u16,
    salg_type: [u8; 14],
    salg_feat: u32,
    salg_mask: u32,
    salg_name: [u8; 64],
}

#[repr(C)]
struct siginfo_t {
    _private: [u8; 0],
}

#[repr(C)]
struct sigset_t {
    __val: [c_ulong; 16],
}

#[repr(C)]
struct sigaction {
    sa_sigaction: Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
    sa_mask: sigset_t,
    sa_flags: c_int,
    sa_restorer: Option<unsafe extern "C" fn()>,
}

unsafe extern "C" {
    static mut stdout: *mut FILE;

    fn __errno_location() -> *mut c_int;
    fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut u32) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: u32) -> c_int;
    fn exit(status: c_int) -> !;
    fn execl(path: *const c_char, arg: *const c_char, ...) -> c_int;
    fn feof(stream: *mut FILE) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn free(ptr: *mut c_void);
    fn malloc(size: usize) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn recv(sockfd: c_int, buf: *mut c_void, len: usize, flags: c_int) -> isize;
    fn setvbuf(stream: *mut FILE, buf: *mut c_char, mode: c_int, size: usize) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn splice(
        fd_in: c_int,
        off_in: *mut c_long,
        fd_out: c_int,
        off_out: *mut c_long,
        len: usize,
        flags: c_uint,
    ) -> isize;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn vmsplice(fd: c_int, iov: *const iovec, nr_segs: c_ulong, flags: c_uint) -> isize;
}

static mut BASE: c_int = 0;
static mut SOCK: c_int = 0;

static mut DIGEST_LEN: c_int = 0;
static mut REF: *mut c_char = ptr::null_mut();
static mut DIGEST: *mut c_char = ptr::null_mut();
static mut ALG_NAME: *mut c_char = ptr::null_mut();

static mut DATA_IOV: iovec = iovec {
    iov_base: ptr::null_mut(),
    iov_len: 0,
};
static mut ZEROCOPY: [c_int; 2] = [0; 2];
static mut SIGS: c_int = 0;
static mut ITER: c_int = 0;

unsafe extern "C" fn handle_exit_signal(
    sig: c_int,
    _info: *mut siginfo_t,
    _context: *mut c_void,
) {
    unsafe {
        printf(
            c"Terminated by signal %d, iterations=%d, signals=%d\n".as_ptr(),
            sig,
            ITER,
            SIGS,
        );
        exit(0);
    }
}

unsafe extern "C" fn handle_kick_signal(
    _sig: c_int,
    _info: *mut siginfo_t,
    _context: *mut c_void,
) {
    unsafe {
        SIGS += 1;
    }
}

static mut DRIVERS: [*const c_char; 15] = [
    c"sha1-ce".as_ptr(),
    c"sha224-arm64".as_ptr(),
    c"sha224-arm64-neon".as_ptr(),
    c"sha224-ce".as_ptr(),
    c"sha256-arm64".as_ptr(),
    c"sha256-arm64-neon".as_ptr(),
    c"sha256-ce".as_ptr(),
    c"sha384-ce".as_ptr(),
    c"sha512-ce".as_ptr(),
    c"sha3-224-ce".as_ptr(),
    c"sha3-256-ce".as_ptr(),
    c"sha3-384-ce".as_ptr(),
    c"sha3-512-ce".as_ptr(),
    c"sm3-ce".as_ptr(),
    c"sm3-neon".as_ptr(),
];

unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

unsafe fn create_socket() -> bool {
    let mut proc: *mut FILE;
    let mut addr: sockaddr_alg;
    let mut buf: [c_char; 1024] = [0; 1024];
    let mut c: *mut c_char;
    let mut driver_name: *mut c_char;
    let mut is_shash: bool;
    let mut match_: bool;
    let mut ret: c_int;
    let mut i: usize;

    unsafe {
        ret = socket(AF_ALG, SOCK_SEQPACKET, 0);
        if ret < 0 {
            if errno() == EAFNOSUPPORT {
                printf(c"AF_ALG not supported\n".as_ptr());
                return false;
            }

            printf(
                c"Failed to create AF_ALG socket: %s (%d)\n".as_ptr(),
                strerror(errno()),
                errno(),
            );
            return false;
        }
        BASE = ret;

        addr = mem::zeroed();
        addr.salg_family = AF_ALG as u16;
        strncpy(
            addr.salg_type.as_mut_ptr() as *mut c_char,
            c"hash".as_ptr(),
            addr.salg_type.len(),
        );

        proc = fopen(c"/proc/crypto".as_ptr(), c"r".as_ptr());
        if proc.is_null() {
            printf(c"Unable to open /proc/crypto\n".as_ptr());
            return false;
        }

        driver_name = ptr::null_mut();
        is_shash = false;
        match_ = false;

        /* Look through /proc/crypto for a driver with kernel mode FP usage */
        while !match_ {
            c = fgets(buf.as_mut_ptr(), buf.len() as c_int, proc);
            if c.is_null() {
                if feof(proc) != 0 {
                    printf(c"Nothing found in /proc/crypto\n".as_ptr());
                    return false;
                }
                continue;
            }

            /* Algorithm descriptions are separated by a blank line */
            if *c == b'\n' as c_char {
                if is_shash && !driver_name.is_null() {
                    i = 0;
                    while i < DRIVERS.len() {
                        if strcmp(DRIVERS[i], driver_name) == 0 {
                            match_ = true;
                        }
                        i += 1;
                    }
                }

                if !match_ {
                    DIGEST_LEN = 0;

                    free(driver_name as *mut c_void);
                    driver_name = ptr::null_mut();

                    free(ALG_NAME as *mut c_void);
                    ALG_NAME = ptr::null_mut();

                    is_shash = false;
                }
                continue;
            }

            /* Remove trailing newline */
            c = strchr(buf.as_ptr(), b'\n' as c_int);
            if !c.is_null() {
                *c = b'\0' as c_char;
            }

            /* Find the field/value separator and start of the value */
            c = strchr(buf.as_ptr(), b':' as c_int);
            if c.is_null() {
                continue;
            }
            c = c.add(2);

            if strncmp(
                buf.as_ptr(),
                c"digestsize".as_ptr(),
                strlen(c"digestsize".as_ptr()),
            ) == 0
            {
                sscanf(c, c"%d".as_ptr(), &raw mut DIGEST_LEN);
            }

            if strncmp(buf.as_ptr(), c"name".as_ptr(), strlen(c"name".as_ptr())) == 0 {
                ALG_NAME = strdup(c);
            }

            if strncmp(
                buf.as_ptr(),
                c"driver".as_ptr(),
                strlen(c"driver".as_ptr()),
            ) == 0
            {
                driver_name = strdup(c);
            }

            if strncmp(buf.as_ptr(), c"type".as_ptr(), strlen(c"type".as_ptr())) == 0 {
                if strncmp(c, c"shash".as_ptr(), strlen(c"shash".as_ptr())) == 0 {
                    is_shash = true;
                }
            }
        }

        strncpy(
            addr.salg_name.as_mut_ptr() as *mut c_char,
            ALG_NAME,
            addr.salg_name.len() - 1,
        );

        ret = bind(
            BASE,
            &addr as *const sockaddr_alg as *const sockaddr,
            mem::size_of::<sockaddr_alg>() as u32,
        );
        if ret < 0 {
            printf(
                c"Failed to bind %s: %s (%d)\n".as_ptr(),
                addr.salg_name.as_ptr(),
                strerror(errno()),
                errno(),
            );
            return false;
        }

        ret = accept(BASE, ptr::null_mut(), ptr::null_mut());
        if ret < 0 {
            printf(
                c"Failed to accept %s: %s (%d)\n".as_ptr(),
                addr.salg_name.as_ptr(),
                strerror(errno()),
                errno(),
            );
            return false;
        }

        SOCK = ret;

        ret = pipe((&raw mut ZEROCOPY).cast::<c_int>());
        if ret != 0 {
            printf(
                c"Failed to create zerocopy pipe: %s (%d)\n".as_ptr(),
                strerror(errno()),
                errno(),
            );
            return false;
        }

        REF = malloc(DIGEST_LEN as usize) as *mut c_char;
        if REF.is_null() {
            printf(c"Failed to allocate %d byte reference\n".as_ptr(), DIGEST_LEN);
            return false;
        }

        DIGEST = malloc(DIGEST_LEN as usize) as *mut c_char;
        if DIGEST.is_null() {
            printf(c"Failed to allocate %d byte digest\n".as_ptr(), DIGEST_LEN);
            return false;
        }

        true
    }
}

unsafe fn compute_digest(buf: *mut c_void) -> bool {
    let mut iov: iovec;
    let mut ret: isize;
    let mut wrote: c_int;

    unsafe {
        iov = DATA_IOV;
        while iov.iov_len != 0 {
            ret = vmsplice(ZEROCOPY[1], &iov, 1, SPLICE_F_GIFT);
            if ret < 0 {
                printf(
                    c"Failed to send buffer: %s (%d)\n".as_ptr(),
                    strerror(errno()),
                    errno(),
                );
                return false;
            }

            wrote = ret as c_int;
            ret = splice(
                ZEROCOPY[0],
                ptr::null_mut(),
                SOCK,
                ptr::null_mut(),
                wrote as usize,
                0,
            );
            if ret < 0 {
                printf(
                    c"Failed to splice buffer: %s (%d)\n".as_ptr(),
                    strerror(errno()),
                    errno(),
                );
            } else if ret != wrote as isize {
                printf(c"Short splice: %d < %d\n".as_ptr(), ret as c_int, wrote);
            }

            iov.iov_len -= wrote as usize;
            iov.iov_base = (iov.iov_base as *mut u8).add(wrote as usize) as *mut c_void;
        }

        loop {
            ret = recv(SOCK, buf, DIGEST_LEN as usize, 0);
            if ret == 0 {
                printf(c"No digest returned\n".as_ptr());
                return false;
            }
            if ret != DIGEST_LEN as isize {
                if errno() == -EAGAIN {
                    continue;
                }
                printf(
                    c"Failed to get digest: %s (%d)\n".as_ptr(),
                    strerror(errno()),
                    errno(),
                );
                return false;
            }

            break;
        }

        true
    }
}

fn main() -> c_int {
    let mut data: *mut c_char;
    let mut sa: sigaction;
    let mut ret: c_int;

    unsafe {
        /* Ensure we have unbuffered output */
        setvbuf(stdout, ptr::null_mut(), _IOLBF, 0);

        /* The parent will communicate with us via signals */
        sa = mem::zeroed();
        sa.sa_sigaction = Some(handle_exit_signal);
        sa.sa_flags = SA_RESTART | SA_SIGINFO;
        sigemptyset(&mut sa.sa_mask);
        ret = sigaction(SIGTERM, &sa, ptr::null_mut());
        if ret < 0 {
            printf(
                c"Failed to install SIGTERM handler: %s (%d)\n".as_ptr(),
                strerror(errno()),
                errno(),
            );
        }

        sa.sa_sigaction = Some(handle_kick_signal);
        ret = sigaction(SIGUSR1, &sa, ptr::null_mut());
        if ret < 0 {
            printf(
                c"Failed to install SIGUSR1 handler: %s (%d)\n".as_ptr(),
                strerror(errno()),
                errno(),
            );
        }
        ret = sigaction(SIGUSR2, &sa, ptr::null_mut());
        if ret < 0 {
            printf(
                c"Failed to install SIGUSR2 handler: %s (%d)\n".as_ptr(),
                strerror(errno()),
                errno(),
            );
        }

        data = malloc(DATA_SIZE) as *mut c_char;
        if data.is_null() {
            printf(c"Failed to allocate data buffer\n".as_ptr());
            return EXIT_FAILURE;
        }
        memset(data as *mut c_void, 0, DATA_SIZE);

        DATA_IOV.iov_base = data as *mut c_void;
        DATA_IOV.iov_len = DATA_SIZE;

        /*
         * If we can't create a socket assume it's a lack of system
         * support and fall back to a basic FPSIMD test for the
         * benefit of fp-stress.
         */
        if !create_socket() {
            execl(
                c"./fpsimd-test".as_ptr(),
                c"./fpsimd-test".as_ptr(),
                ptr::null::<c_char>(),
            );
            printf(
                c"Failed to fall back to fspimd-test: %d (%s)\n".as_ptr(),
                errno(),
                strerror(errno()),
            );
            return EXIT_FAILURE;
        }

        /*
         * Compute a reference digest we hope is repeatable, we do
         * this at runtime partly to make it easier to play with
         * parameters.
         */
        if !compute_digest(REF as *mut c_void) {
            printf(c"Failed to compute reference digest\n".as_ptr());
            return EXIT_FAILURE;
        }

        printf(c"AF_ALG using %s\n".as_ptr(), ALG_NAME);

        loop {
            if !compute_digest(DIGEST as *mut c_void) {
                printf(c"Failed to compute digest, iter=%d\n".as_ptr(), ITER);
                return EXIT_FAILURE;
            }

            if memcmp(REF as *const c_void, DIGEST as *const c_void, DIGEST_LEN as usize) != 0 {
                printf(c"Digest mismatch, iter=%d\n".as_ptr(), ITER);
                return EXIT_FAILURE;
            }

            ITER += 1;
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
