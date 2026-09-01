// SPDX-License-Identifier: GPL-2.0-only
/*
 * vsock_perf - benchmark utility for vsock.
 *
 * Copyright (C) 2022 SberDevices.
 *
 * Author: Arseniy Krasnov <AVKrasnov@sberdevices.ru>
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_ulonglong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type time_t = i64;
type size_t = usize;
type ssize_t = isize;
type socklen_t = u32;

const EXIT_FAILURE: c_int = 1;

const AF_VSOCK: c_int = 40;
const SOCK_STREAM: c_int = 1;
const SOL_SOCKET: c_int = 1;
const SO_RCVLOWAT: c_int = 18;
const SO_ZEROCOPY: c_int = 60;
const SO_VM_SOCKETS_BUFFER_SIZE: c_int = 0;
const SO_VM_SOCKETS_BUFFER_MAX_SIZE: c_int = 2;
const VMADDR_CID_ANY: c_uint = 0xFFFF_FFFF;

const CLOCK_REALTIME: c_int = 0;

const POLLIN: i16 = 0x0001;
const POLLERR: i16 = 0x0008;
const POLLHUP: i16 = 0x0010;
const POLLRDHUP: i16 = 0x2000;

const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const MSG_ZEROCOPY: c_int = 0x4000000;

const no_argument: c_int = 0;
const required_argument: c_int = 1;

const DEFAULT_BUF_SIZE_BYTES: c_ulong = 128 * 1024;
const DEFAULT_TO_SEND_BYTES: c_ulong = 64 * 1024;
const DEFAULT_VSOCK_BUF_BYTES: c_ulonglong = 256 * 1024;
const DEFAULT_RCVLOWAT_BYTES: c_int = 1;
const DEFAULT_PORT: c_uint = 1234;

const BYTES_PER_GB: c_ulonglong = 1024 * 1024 * 1024;
const NSEC_PER_SEC: c_ulonglong = 1000000000;

static mut port: c_uint = DEFAULT_PORT;
static mut buf_size_bytes: c_ulong = DEFAULT_BUF_SIZE_BYTES;
static mut vsock_buf_bytes: c_ulonglong = DEFAULT_VSOCK_BUF_BYTES;
static mut zerocopy: bool = false;

#[repr(C)]
struct timespec {
    tv_sec: time_t,
    tv_nsec: c_long,
}

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_vm {
    svm_family: u16,
    svm_reserved1: u16,
    svm_port: c_uint,
    svm_cid: c_uint,
    svm_zero: [u8; 4],
}

#[repr(C)]
union sockaddr_union {
    sa: sockaddr,
    svm: sockaddr_vm,
}

#[repr(C)]
struct pollfd {
    fd: c_int,
    events: i16,
    revents: i16,
}

#[repr(C)]
struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

unsafe extern "C" {
    static mut optarg: *mut c_char;

    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulonglong;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strlen(s: *const c_char) -> size_t;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn connect(socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int;
    fn close(fildes: c_int) -> c_int;
    fn bind(socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int;
    fn listen(socket: c_int, backlog: c_int) -> c_int;
    fn accept(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn printf(format: *const c_char, ...) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: time_t,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn send(socket: c_int, buffer: *const c_void, length: size_t, flags: c_int) -> ssize_t;
    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;

    /* From msg_zerocopy_common.h. */
    fn vsock_recv_completion(fd: c_int, zerocopied: *mut bool);
}

unsafe fn MAP_FAILED() -> *mut c_void {
    (-1isize) as *mut c_void
}

unsafe fn error(s: *const c_char) -> ! {
    perror(s);
    exit(EXIT_FAILURE);
}

unsafe fn current_nsec() -> time_t {
    let mut ts: timespec = zeroed();

    if clock_gettime(CLOCK_REALTIME, &mut ts) != 0 {
        error(c"clock_gettime".as_ptr());
    }

    (ts.tv_sec * NSEC_PER_SEC as time_t) + ts.tv_nsec
}

/* From lib/cmdline.c. */
unsafe fn memparse(ptr: *const c_char) -> c_ulong {
    let mut endptr: *mut c_char = null_mut();

    let mut ret: c_ulonglong = strtoull(ptr, &mut endptr, 0);

    match *endptr as u8 as char {
        'E' | 'e' => {
            ret <<= 10;
            ret <<= 10;
            ret <<= 10;
            ret <<= 10;
            ret <<= 10;
            ret <<= 10;
            endptr = endptr.add(1);
        }
        'P' | 'p' => {
            ret <<= 10;
            ret <<= 10;
            ret <<= 10;
            ret <<= 10;
            ret <<= 10;
            endptr = endptr.add(1);
        }
        'T' | 't' => {
            ret <<= 10;
            ret <<= 10;
            ret <<= 10;
            ret <<= 10;
            endptr = endptr.add(1);
        }
        'G' | 'g' => {
            ret <<= 10;
            ret <<= 10;
            ret <<= 10;
            endptr = endptr.add(1);
        }
        'M' | 'm' => {
            ret <<= 10;
            ret <<= 10;
            endptr = endptr.add(1);
        }
        'K' | 'k' => {
            ret <<= 10;
            endptr = endptr.add(1);
        }
        _ => {}
    }

    ret as c_ulong
}

unsafe fn vsock_increase_buf_size(fd: c_int) {
    if setsockopt(
        fd,
        AF_VSOCK,
        SO_VM_SOCKETS_BUFFER_MAX_SIZE,
        (&raw const vsock_buf_bytes).cast::<c_void>(),
        size_of::<c_ulonglong>() as socklen_t,
    ) != 0
    {
        error(c"setsockopt(SO_VM_SOCKETS_BUFFER_MAX_SIZE)".as_ptr());
    }

    if setsockopt(
        fd,
        AF_VSOCK,
        SO_VM_SOCKETS_BUFFER_SIZE,
        (&raw const vsock_buf_bytes).cast::<c_void>(),
        size_of::<c_ulonglong>() as socklen_t,
    ) != 0
    {
        error(c"setsockopt(SO_VM_SOCKETS_BUFFER_SIZE)".as_ptr());
    }
}

unsafe fn vsock_connect(cid: c_uint, port_: c_uint) -> c_int {
    let mut addr = sockaddr_union {
        svm: sockaddr_vm {
            svm_family: AF_VSOCK as u16,
            svm_reserved1: 0,
            svm_port: port_,
            svm_cid: cid,
            svm_zero: [0; 4],
        },
    };
    let fd: c_int;

    fd = socket(AF_VSOCK, SOCK_STREAM, 0);

    if fd < 0 {
        perror(c"socket".as_ptr());
        return -1;
    }

    if connect(
        fd,
        (&raw const addr.sa),
        size_of::<sockaddr_vm>() as socklen_t,
    ) < 0
    {
        perror(c"connect".as_ptr());
        close(fd);
        return -1;
    }

    fd
}

fn get_gbps(bits: c_ulong, ns_delta: time_t) -> f32 {
    ((bits as f32) / 1000000000u64 as f32) / ((ns_delta as f32) / NSEC_PER_SEC as f32)
}

unsafe fn run_receiver(rcvlowat_bytes: c_int) {
    let mut read_cnt: c_uint;
    let rx_begin_ns: time_t;
    let mut in_read_ns: time_t;
    let mut total_recv: size_t;
    let client_fd: c_int;
    let data: *mut c_char;
    let fd: c_int;
    let mut addr = sockaddr_union {
        svm: sockaddr_vm {
            svm_family: AF_VSOCK as u16,
            svm_reserved1: 0,
            svm_port: port,
            svm_cid: VMADDR_CID_ANY,
            svm_zero: [0; 4],
        },
    };
    let mut clientaddr: sockaddr_union = zeroed();

    let mut clientaddr_len: socklen_t = size_of::<sockaddr_vm>() as socklen_t;

    printf(c"Run as receiver\n".as_ptr());
    printf(c"Listen port %u\n".as_ptr(), port);
    printf(c"RX buffer %lu bytes\n".as_ptr(), buf_size_bytes);
    printf(c"vsock buffer %llu bytes\n".as_ptr(), vsock_buf_bytes);
    printf(c"SO_RCVLOWAT %d bytes\n".as_ptr(), rcvlowat_bytes);

    fd = socket(AF_VSOCK, SOCK_STREAM, 0);

    if fd < 0 {
        error(c"socket".as_ptr());
    }

    if bind(
        fd,
        (&raw const addr.sa),
        size_of::<sockaddr_vm>() as socklen_t,
    ) < 0
    {
        error(c"bind".as_ptr());
    }

    if listen(fd, 1) < 0 {
        error(c"listen".as_ptr());
    }

    client_fd = accept(fd, &raw mut clientaddr.sa, &mut clientaddr_len);

    if client_fd < 0 {
        error(c"accept".as_ptr());
    }

    vsock_increase_buf_size(client_fd);

    if setsockopt(
        client_fd,
        SOL_SOCKET,
        SO_RCVLOWAT,
        (&raw const rcvlowat_bytes).cast::<c_void>(),
        size_of::<c_int>() as socklen_t,
    ) != 0
    {
        error(c"setsockopt(SO_RCVLOWAT)".as_ptr());
    }

    data = malloc(buf_size_bytes as size_t) as *mut c_char;

    if data.is_null() {
        fprintf(stderr, c"'malloc()' failed\n".as_ptr());
        exit(EXIT_FAILURE);
    }

    read_cnt = 0;
    in_read_ns = 0;
    total_recv = 0;
    rx_begin_ns = current_nsec();

    loop {
        let mut fds: pollfd = zeroed();

        fds.fd = client_fd;
        fds.events = POLLIN | POLLERR | POLLHUP | POLLRDHUP;

        if poll(&mut fds, 1, -1) < 0 {
            error(c"poll".as_ptr());
        }

        if (fds.revents & POLLERR) != 0 {
            fprintf(stderr, c"'poll()' error\n".as_ptr());
            exit(EXIT_FAILURE);
        }

        if (fds.revents & POLLIN) != 0 {
            let bytes_read: ssize_t;
            let t: time_t;

            t = current_nsec();
            bytes_read = read(fds.fd, data.cast::<c_void>(), buf_size_bytes as size_t);
            in_read_ns += current_nsec() - t;
            read_cnt += 1;

            if bytes_read == 0 {
                break;
            }

            if bytes_read < 0 {
                perror(c"read".as_ptr());
                exit(EXIT_FAILURE);
            }

            total_recv += bytes_read as size_t;
        }

        if (fds.revents & (POLLHUP | POLLRDHUP)) != 0 {
            break;
        }
    }

    printf(c"total bytes received: %zu\n".as_ptr(), total_recv);
    printf(
        c"rx performance: %f Gbits/s\n".as_ptr(),
        get_gbps((total_recv * 8) as c_ulong, current_nsec() - rx_begin_ns) as f64,
    );
    printf(
        c"total time in 'read()': %f sec\n".as_ptr(),
        (in_read_ns as f32 / NSEC_PER_SEC as f32) as f64,
    );
    printf(
        c"average time in 'read()': %f ns\n".as_ptr(),
        (in_read_ns as f32 / read_cnt as f32) as f64,
    );
    printf(c"POLLIN wakeups: %i\n".as_ptr(), read_cnt);

    free(data.cast::<c_void>());
    close(client_fd);
    close(fd);
}

unsafe fn enable_so_zerocopy(fd: c_int) {
    let val: c_int = 1;

    if setsockopt(
        fd,
        SOL_SOCKET,
        SO_ZEROCOPY,
        (&raw const val).cast::<c_void>(),
        size_of::<c_int>() as socklen_t,
    ) != 0
    {
        perror(c"setsockopt".as_ptr());
        exit(EXIT_FAILURE);
    }
}

unsafe fn run_sender(peer_cid: c_int, to_send_bytes: c_ulong) {
    let tx_begin_ns: time_t;
    let tx_total_ns: time_t;
    let mut total_send: size_t;
    let mut time_in_send: time_t;
    let data: *mut c_void;
    let fd: c_int;

    if zerocopy {
        printf(c"Run as sender MSG_ZEROCOPY\n".as_ptr());
    } else {
        printf(c"Run as sender\n".as_ptr());
    }

    printf(c"Connect to %i:%u\n".as_ptr(), peer_cid, port);
    printf(c"Send %lu bytes\n".as_ptr(), to_send_bytes);
    printf(c"TX buffer %lu bytes\n".as_ptr(), buf_size_bytes);

    fd = vsock_connect(peer_cid as c_uint, port);

    if fd < 0 {
        exit(EXIT_FAILURE);
    }

    if zerocopy {
        enable_so_zerocopy(fd);

        data = mmap(
            null_mut(),
            buf_size_bytes as size_t,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANONYMOUS,
            -1,
            0,
        );
        if data == MAP_FAILED() {
            perror(c"mmap".as_ptr());
            exit(EXIT_FAILURE);
        }
    } else {
        data = malloc(buf_size_bytes as size_t);

        if data.is_null() {
            fprintf(stderr, c"'malloc()' failed\n".as_ptr());
            exit(EXIT_FAILURE);
        }
    }

    memset(data, 0, buf_size_bytes as size_t);
    total_send = 0;
    time_in_send = 0;
    tx_begin_ns = current_nsec();

    while total_send < to_send_bytes as size_t {
        let sent: ssize_t;
        let rest_bytes: size_t;
        let before: time_t;

        rest_bytes = to_send_bytes as size_t - total_send;

        before = current_nsec();
        sent = send(
            fd,
            data.cast::<c_void>(),
            if rest_bytes > buf_size_bytes as size_t {
                buf_size_bytes as size_t
            } else {
                rest_bytes
            },
            if zerocopy { MSG_ZEROCOPY } else { 0 },
        );
        time_in_send += current_nsec() - before;

        if sent <= 0 {
            error(c"write".as_ptr());
        }

        total_send += sent as size_t;

        if zerocopy {
            let mut fds: pollfd = zeroed();

            fds.fd = fd;

            if poll(&mut fds, 1, -1) < 0 {
                perror(c"poll".as_ptr());
                exit(EXIT_FAILURE);
            }

            if (fds.revents & POLLERR) == 0 {
                fprintf(stderr, c"POLLERR expected\n".as_ptr());
                exit(EXIT_FAILURE);
            }

            vsock_recv_completion(fd, null_mut());
        }
    }

    tx_total_ns = current_nsec() - tx_begin_ns;

    printf(c"total bytes sent: %zu\n".as_ptr(), total_send);
    printf(
        c"tx performance: %f Gbits/s\n".as_ptr(),
        get_gbps((total_send * 8) as c_ulong, time_in_send) as f64,
    );
    printf(
        c"total time in tx loop: %f sec\n".as_ptr(),
        (tx_total_ns as f32 / NSEC_PER_SEC as f32) as f64,
    );
    printf(
        c"time in 'send()': %f sec\n".as_ptr(),
        (time_in_send as f32 / NSEC_PER_SEC as f32) as f64,
    );

    close(fd);

    if zerocopy {
        munmap(data, buf_size_bytes as size_t);
    } else {
        free(data);
    }
}

static optstring: &[u8] = b"\0";
static longopts: [option; 9] = [
    option {
        name: c"help".as_ptr(),
        has_arg: no_argument,
        flag: null_mut(),
        val: 'H' as c_int,
    },
    option {
        name: c"sender".as_ptr(),
        has_arg: required_argument,
        flag: null_mut(),
        val: 'S' as c_int,
    },
    option {
        name: c"port".as_ptr(),
        has_arg: required_argument,
        flag: null_mut(),
        val: 'P' as c_int,
    },
    option {
        name: c"bytes".as_ptr(),
        has_arg: required_argument,
        flag: null_mut(),
        val: 'M' as c_int,
    },
    option {
        name: c"buf-size".as_ptr(),
        has_arg: required_argument,
        flag: null_mut(),
        val: 'B' as c_int,
    },
    option {
        name: c"vsk-size".as_ptr(),
        has_arg: required_argument,
        flag: null_mut(),
        val: 'V' as c_int,
    },
    option {
        name: c"rcvlowat".as_ptr(),
        has_arg: required_argument,
        flag: null_mut(),
        val: 'R' as c_int,
    },
    option {
        name: c"zerocopy".as_ptr(),
        has_arg: no_argument,
        flag: null_mut(),
        val: 'Z' as c_int,
    },
    option {
        name: null(),
        has_arg: 0,
        flag: null_mut(),
        val: 0,
    },
];

unsafe fn usage() -> ! {
    printf(
        c"Usage: ./vsock_perf [--help] [options]\n\
\n\
This is benchmarking utility, to test vsock performance.\n\
It runs in two modes: sender or receiver. In sender mode, it\n\
connects to the specified CID and starts data transmission.\n\
\n\
Options:\n\
  --help\t\t\tThis message\n\
  --sender   <cid>\t\tSender mode (receiver default)\n\
                                <cid> of the receiver to connect to\n\
  --zerocopy\t\t\tEnable zerocopy (for sender mode only)\n\
  --port     <port>\t\tPort (default %d)\n\
  --bytes    <bytes>KMG\t\tBytes to send (default %d)\n\
  --buf-size <bytes>KMG\t\tData buffer size (default %d). In sender mode\n\
                                it is the buffer size, passed to 'write()'. In\n\
                                receiver mode it is the buffer size passed to 'read()'.\n\
  --vsk-size <bytes>KMG\t\tSocket buffer size (default %d)\n\
  --rcvlowat <bytes>KMG\t\tSO_RCVLOWAT value (default %d)\n\
\n"
        .as_ptr(),
        DEFAULT_PORT,
        DEFAULT_TO_SEND_BYTES as c_int,
        DEFAULT_BUF_SIZE_BYTES as c_int,
        DEFAULT_VSOCK_BUF_BYTES as c_int,
        DEFAULT_RCVLOWAT_BYTES,
    );
    exit(EXIT_FAILURE);
}

unsafe fn strtolx(arg: *const c_char) -> c_long {
    let value: c_long;
    let mut end: *mut c_char = null_mut();

    value = strtol(arg, &mut end, 10);

    if end != arg.cast_mut().add(strlen(arg)) {
        usage();
    }

    value
}

unsafe fn c_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut to_send_bytes: c_ulong = DEFAULT_TO_SEND_BYTES;
    let mut rcvlowat_bytes: c_int = DEFAULT_RCVLOWAT_BYTES;
    let mut peer_cid: c_int = -1;
    let mut sender: bool = false;

    loop {
        let opt: c_int = getopt_long(argc, argv, optstring.as_ptr().cast::<c_char>(), longopts.as_ptr(), null_mut());

        if opt == -1 {
            break;
        }

        match opt {
            x if x == 'V' as c_int => {
                /* Peer buffer size. */
                vsock_buf_bytes = memparse(optarg);
            }
            x if x == 'R' as c_int => {
                /* SO_RCVLOWAT value. */
                rcvlowat_bytes = memparse(optarg) as c_int;
            }
            x if x == 'P' as c_int => {
                /* Port to connect to. */
                port = strtolx(optarg) as c_uint;
            }
            x if x == 'M' as c_int => {
                /* Bytes to send. */
                to_send_bytes = memparse(optarg);
            }
            x if x == 'B' as c_int => {
                /* Size of rx/tx buffer. */
                buf_size_bytes = memparse(optarg);
            }
            x if x == 'S' as c_int => {
                /* Sender mode. CID to connect to. */
                peer_cid = strtolx(optarg) as c_int;
                sender = true;
            }
            x if x == 'H' as c_int => {
                /* Help. */
                usage();
            }
            x if x == 'Z' as c_int => {
                /* Zerocopy. */
                zerocopy = true;
            }
            _ => {
                usage();
            }
        }
    }

    if !sender {
        run_receiver(rcvlowat_bytes);
    } else {
        run_sender(peer_cid, to_send_bytes);
    }

    0
}

fn main() {
    unsafe {
        let mut args: Vec<*mut c_char> = std::env::args()
            .map(|arg| std::ffi::CString::new(arg).unwrap().into_raw())
            .collect();
        args.push(null_mut());
        let argc = (args.len() - 1) as c_int;
        let code = c_main(argc, args.as_mut_ptr());
        for arg in args.into_iter().filter(|arg| !arg.is_null()) {
            drop(std::ffi::CString::from_raw(arg));
        }
        std::process::exit(code);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
