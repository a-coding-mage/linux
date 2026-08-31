// SPDX-License-Identifier: GPL-2.0-only
/* io_uring tests for vsock
 *
 * Copyright (C) 2023 SberDevices.
 *
 * Author: Arseniy Krasnov <avkrasnov@salutedevices.com>
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

/* Dependencies from C headers and local headers:
 * getopt.h, stdio.h, stdlib.h, string.h, liburing.h, unistd.h,
 * sys/mman.h, linux/kernel.h, error.h, util.h, control.h,
 * msg_zerocopy_common.h.
 */

const PAGE_SIZE: usize = 4096;
const RING_ENTRIES_NUM: c_uint = 4;
const VSOCK_TEST_DATA_MAX_IOV: usize = 3;

const EXIT_FAILURE: c_int = 1;
const VMADDR_CID_ANY: c_uint = 0xffffffff;
const DEFAULT_PEER_PORT: c_uint = 1234;

const TEST_MODE_UNSET: c_int = 0;
const TEST_MODE_CLIENT: c_int = 1;
const TEST_MODE_SERVER: c_int = 2;

const REQUIRED_ARGUMENT: c_int = 1;
const NO_ARGUMENT: c_int = 0;

#[repr(C)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}

#[repr(C)]
pub struct msghdr {
    pub msg_name: *mut c_void,
    pub msg_namelen: c_uint,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: usize,
    pub msg_control: *mut c_void,
    pub msg_controllen: usize,
    pub msg_flags: c_int,
}

#[repr(C)]
pub struct io_uring {
    /* Opaque liburing storage; real layout is supplied by liburing. */
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_uring_sqe {
    /* Opaque liburing submission queue entry. */
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_uring_cqe {
    pub user_data: u64,
    pub res: c_int,
    pub flags: c_uint,
}

#[repr(C)]
pub struct test_opts {
    pub mode: c_int,
    pub peer_cid: c_uint,
    pub peer_port: c_uint,
}

#[repr(C)]
pub struct test_case {
    pub name: *const c_char,
    pub run_server: Option<unsafe extern "C" fn(*const test_opts)>,
    pub run_client: Option<unsafe extern "C" fn(*const test_opts)>,
}

#[repr(C)]
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

#[repr(C)]
struct vsock_io_uring_test {
    /* Number of valid elements in 'vecs'. */
    vecs_cnt: c_int,
    vecs: [iovec; VSOCK_TEST_DATA_MAX_IOV],
}

static mut TEST_DATA_ARRAY: [vsock_io_uring_test; 2] = [
    /* All elements have page aligned base and size. */
    vsock_io_uring_test {
        vecs_cnt: 3,
        vecs: [
            iovec {
                iov_base: ptr::null_mut(),
                iov_len: PAGE_SIZE,
            },
            iovec {
                iov_base: ptr::null_mut(),
                iov_len: 2 * PAGE_SIZE,
            },
            iovec {
                iov_base: ptr::null_mut(),
                iov_len: 3 * PAGE_SIZE,
            },
        ],
    },
    /* Middle element has both non-page aligned base and size. */
    vsock_io_uring_test {
        vecs_cnt: 3,
        vecs: [
            iovec {
                iov_base: ptr::null_mut(),
                iov_len: PAGE_SIZE,
            },
            iovec {
                iov_base: 1 as *mut c_void,
                iov_len: 200,
            },
            iovec {
                iov_base: ptr::null_mut(),
                iov_len: 3 * PAGE_SIZE,
            },
        ],
    },
];

unsafe extern "C" {
    static mut optarg: *mut c_char;
    static mut stderr: *mut c_void;

    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn close(fd: c_int) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...);
    fn __errno_location() -> *mut c_int;

    fn io_uring_queue_init(entries: c_uint, ring: *mut io_uring, flags: c_uint) -> c_int;
    fn io_uring_register_buffers(ring: *mut io_uring, iovecs: *const iovec, nr_iovecs: c_uint)
        -> c_int;
    fn io_uring_get_sqe(ring: *mut io_uring) -> *mut io_uring_sqe;
    fn io_uring_prep_sendmsg(sqe: *mut io_uring_sqe, fd: c_int, msg: *const msghdr, flags: c_uint);
    fn io_uring_prep_sendmsg_zc(
        sqe: *mut io_uring_sqe,
        fd: c_int,
        msg: *const msghdr,
        flags: c_uint,
    );
    fn io_uring_submit(ring: *mut io_uring) -> c_int;
    fn io_uring_wait_cqe(ring: *mut io_uring, cqe_ptr: *mut *mut io_uring_cqe) -> c_int;
    fn io_uring_cqe_seen(ring: *mut io_uring, cqe: *mut io_uring_cqe);
    fn io_uring_prep_readv(
        sqe: *mut io_uring_sqe,
        fd: c_int,
        iovecs: *const iovec,
        nr_vecs: c_uint,
        offset: c_long,
    );
    fn io_uring_queue_exit(ring: *mut io_uring);

    fn vsock_stream_connect(cid: c_uint, port: c_uint) -> c_int;
    fn vsock_stream_accept(cid: c_uint, port: c_uint, addr: *mut c_void) -> c_int;
    fn enable_so_zerocopy_check(fd: c_int);
    fn alloc_test_iovec(vecs: *const iovec, vecs_cnt: c_int) -> *mut iovec;
    fn free_test_iovec(vecs: *const iovec, iovec: *mut iovec, vecs_cnt: c_int);
    fn iovec_hash_djb2(iovec: *const iovec, vecs_cnt: c_int) -> c_ulong;
    fn iovec_bytes(vecs: *const iovec, vecs_cnt: c_int) -> usize;
    fn hash_djb2(data: *const c_void, len: usize) -> c_ulong;
    fn control_writeulong(value: c_ulong);
    fn control_writeln(str_: *const c_char);
    fn control_readulong() -> c_ulong;
    fn control_expectln(str_: *const c_char);
    fn init_signals();
    fn parse_cid(str_: *const c_char) -> c_uint;
    fn parse_port(str_: *const c_char) -> c_uint;
    fn control_init(host: *const c_char, port: *const c_char, server: bool);
    fn run_tests(test_cases: *const test_case, opts: *const test_opts);
    fn control_cleanup();
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

unsafe fn vsock_io_uring_client(
    opts: *const test_opts,
    test_data: *const vsock_io_uring_test,
    msg_zerocopy: bool,
) {
    let mut sqe: *mut io_uring_sqe;
    let mut cqe: *mut io_uring_cqe = ptr::null_mut();
    let mut ring: io_uring = mem::zeroed();
    let iovec: *mut iovec;
    let mut msg: msghdr = mem::zeroed();
    let fd: c_int;

    fd = vsock_stream_connect((*opts).peer_cid, (*opts).peer_port);
    if fd < 0 {
        perror(c"connect".as_ptr());
        exit(EXIT_FAILURE);
    }

    if msg_zerocopy {
        enable_so_zerocopy_check(fd);
    }

    iovec = alloc_test_iovec((*test_data).vecs.as_ptr(), (*test_data).vecs_cnt);

    if io_uring_queue_init(RING_ENTRIES_NUM, &mut ring, 0) != 0 {
        error(1, errno(), c"io_uring_queue_init".as_ptr());
    }

    if io_uring_register_buffers(&mut ring, iovec, (*test_data).vecs_cnt as c_uint) != 0 {
        error(1, errno(), c"io_uring_register_buffers".as_ptr());
    }

    memset(
        &mut msg as *mut msghdr as *mut c_void,
        0,
        mem::size_of::<msghdr>(),
    );
    msg.msg_iov = iovec;
    msg.msg_iovlen = (*test_data).vecs_cnt as usize;
    sqe = io_uring_get_sqe(&mut ring);

    if msg_zerocopy {
        io_uring_prep_sendmsg_zc(sqe, fd, &msg, 0);
    } else {
        io_uring_prep_sendmsg(sqe, fd, &msg, 0);
    }

    if io_uring_submit(&mut ring) != 1 {
        error(1, errno(), c"io_uring_submit".as_ptr());
    }

    if io_uring_wait_cqe(&mut ring, &mut cqe) != 0 {
        error(1, errno(), c"io_uring_wait_cqe".as_ptr());
    }

    io_uring_cqe_seen(&mut ring, cqe);

    control_writeulong(iovec_hash_djb2(iovec, (*test_data).vecs_cnt));

    control_writeln(c"DONE".as_ptr());
    io_uring_queue_exit(&mut ring);
    free_test_iovec((*test_data).vecs.as_ptr(), iovec, (*test_data).vecs_cnt);
    close(fd);
}

unsafe fn vsock_io_uring_server(opts: *const test_opts, test_data: *const vsock_io_uring_test) {
    let mut remote_hash: c_ulong;
    let local_hash: c_ulong;
    let mut ring: io_uring = mem::zeroed();
    let data_len: usize;
    let mut recv_len: usize;
    let data: *mut c_void;
    let fd: c_int;

    fd = vsock_stream_accept(VMADDR_CID_ANY, (*opts).peer_port, ptr::null_mut());
    if fd < 0 {
        perror(c"accept".as_ptr());
        exit(EXIT_FAILURE);
    }

    data_len = iovec_bytes((*test_data).vecs.as_ptr(), (*test_data).vecs_cnt);

    data = malloc(data_len);
    if data.is_null() {
        perror(c"malloc".as_ptr());
        exit(EXIT_FAILURE);
    }

    if io_uring_queue_init(RING_ENTRIES_NUM, &mut ring, 0) != 0 {
        error(1, errno(), c"io_uring_queue_init".as_ptr());
    }

    recv_len = 0;

    while recv_len < data_len {
        let mut sqe: *mut io_uring_sqe;
        let mut cqe: *mut io_uring_cqe = ptr::null_mut();
        let mut iovec = iovec {
            iov_base: ptr::null_mut(),
            iov_len: 0,
        };

        sqe = io_uring_get_sqe(&mut ring);
        iovec.iov_base = (data as *mut u8).add(recv_len) as *mut c_void;
        iovec.iov_len = data_len;

        io_uring_prep_readv(sqe, fd, &iovec, 1, 0);

        if io_uring_submit(&mut ring) != 1 {
            error(1, errno(), c"io_uring_submit".as_ptr());
        }

        if io_uring_wait_cqe(&mut ring, &mut cqe) != 0 {
            error(1, errno(), c"io_uring_wait_cqe".as_ptr());
        }

        recv_len = recv_len.wrapping_add((*cqe).res as usize);
        io_uring_cqe_seen(&mut ring, cqe);
    }

    if recv_len != data_len {
        fprintf(
            stderr,
            c"expected %zu, got %zu\n".as_ptr(),
            data_len,
            recv_len,
        );
        exit(EXIT_FAILURE);
    }

    local_hash = hash_djb2(data, data_len);

    remote_hash = control_readulong();
    if remote_hash != local_hash {
        fprintf(stderr, c"hash mismatch\n".as_ptr());
        exit(EXIT_FAILURE);
    }

    control_expectln(c"DONE".as_ptr());
    io_uring_queue_exit(&mut ring);
    free(data);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_stream_uring_server(opts: *const test_opts) {
    let mut i: c_int;

    i = 0;
    while (i as usize) < TEST_DATA_ARRAY.len() {
        vsock_io_uring_server(opts, &TEST_DATA_ARRAY[i as usize]);
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_stream_uring_client(opts: *const test_opts) {
    let mut i: c_int;

    i = 0;
    while (i as usize) < TEST_DATA_ARRAY.len() {
        vsock_io_uring_client(opts, &TEST_DATA_ARRAY[i as usize], false);
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_stream_uring_msg_zc_server(opts: *const test_opts) {
    let mut i: c_int;

    i = 0;
    while (i as usize) < TEST_DATA_ARRAY.len() {
        vsock_io_uring_server(opts, &TEST_DATA_ARRAY[i as usize]);
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_stream_uring_msg_zc_client(opts: *const test_opts) {
    let mut i: c_int;

    i = 0;
    while (i as usize) < TEST_DATA_ARRAY.len() {
        vsock_io_uring_client(opts, &TEST_DATA_ARRAY[i as usize], true);
        i += 1;
    }
}

static mut TEST_CASES: [test_case; 3] = [
    test_case {
        name: c"SOCK_STREAM io_uring test".as_ptr(),
        run_server: Some(test_stream_uring_server),
        run_client: Some(test_stream_uring_client),
    },
    test_case {
        name: c"SOCK_STREAM io_uring MSG_ZEROCOPY test".as_ptr(),
        run_server: Some(test_stream_uring_msg_zc_server),
        run_client: Some(test_stream_uring_msg_zc_client),
    },
    test_case {
        name: ptr::null(),
        run_server: None,
        run_client: None,
    },
];

static OPTSTRING: &[u8] = b"\0";
static LONGOPTS: [option; 7] = [
    option {
        name: c"control-host".as_ptr(),
        has_arg: REQUIRED_ARGUMENT,
        flag: ptr::null_mut(),
        val: 'H' as c_int,
    },
    option {
        name: c"control-port".as_ptr(),
        has_arg: REQUIRED_ARGUMENT,
        flag: ptr::null_mut(),
        val: 'P' as c_int,
    },
    option {
        name: c"mode".as_ptr(),
        has_arg: REQUIRED_ARGUMENT,
        flag: ptr::null_mut(),
        val: 'm' as c_int,
    },
    option {
        name: c"peer-cid".as_ptr(),
        has_arg: REQUIRED_ARGUMENT,
        flag: ptr::null_mut(),
        val: 'p' as c_int,
    },
    option {
        name: c"peer-port".as_ptr(),
        has_arg: REQUIRED_ARGUMENT,
        flag: ptr::null_mut(),
        val: 'q' as c_int,
    },
    option {
        name: c"help".as_ptr(),
        has_arg: NO_ARGUMENT,
        flag: ptr::null_mut(),
        val: '?' as c_int,
    },
    option {
        name: ptr::null(),
        has_arg: 0,
        flag: ptr::null_mut(),
        val: 0,
    },
];

unsafe fn usage() -> ! {
    fprintf(
        stderr,
        c"Usage: vsock_uring_test [--help] [--control-host=<host>] --control-port=<port> --mode=client|server --peer-cid=<cid> [--peer-port=<port>]\n\n  Server: vsock_uring_test --control-port=1234 --mode=server --peer-cid=3\n  Client: vsock_uring_test --control-host=192.168.0.1 --control-port=1234 --mode=client --peer-cid=2\n\nRun transmission tests using io_uring. Usage is the same as\nin ./vsock_test\n\nOptions:\n  --help                 This help message\n  --control-host <host>  Server IP address to connect to\n  --control-port <port>  Server port to listen on/connect to\n  --mode client|server   Server or client mode\n  --peer-cid <cid>       CID of the other side\n  --peer-port <port>     AF_VSOCK port used for the test [default: %d]\n".as_ptr(),
        DEFAULT_PEER_PORT as c_int,
    );
    exit(EXIT_FAILURE);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut control_host: *const c_char = ptr::null();
    let mut control_port: *const c_char = ptr::null();
    let mut opts = test_opts {
        mode: TEST_MODE_UNSET,
        peer_cid: VMADDR_CID_ANY,
        peer_port: DEFAULT_PEER_PORT,
    };

    init_signals();

    loop {
        let opt: c_int = getopt_long(
            argc,
            argv,
            OPTSTRING.as_ptr() as *const c_char,
            LONGOPTS.as_ptr(),
            ptr::null_mut(),
        );

        if opt == -1 {
            break;
        }

        match opt {
            x if x == 'H' as c_int => {
                control_host = optarg;
            }
            x if x == 'm' as c_int => {
                if strcmp(optarg, c"client".as_ptr()) == 0 {
                    opts.mode = TEST_MODE_CLIENT;
                } else if strcmp(optarg, c"server".as_ptr()) == 0 {
                    opts.mode = TEST_MODE_SERVER;
                } else {
                    fprintf(stderr, c"--mode must be \"client\" or \"server\"\n".as_ptr());
                    return EXIT_FAILURE;
                }
            }
            x if x == 'p' as c_int => {
                opts.peer_cid = parse_cid(optarg);
            }
            x if x == 'q' as c_int => {
                opts.peer_port = parse_port(optarg);
            }
            x if x == 'P' as c_int => {
                control_port = optarg;
            }
            x if x == '?' as c_int => {
                usage();
            }
            _ => {
                usage();
            }
        }
    }

    if control_port.is_null() {
        usage();
    }
    if opts.mode == TEST_MODE_UNSET {
        usage();
    }
    if opts.peer_cid == VMADDR_CID_ANY {
        usage();
    }

    if control_host.is_null() {
        if opts.mode != TEST_MODE_SERVER {
            usage();
        }
        control_host = c"0.0.0.0".as_ptr();
    }

    control_init(control_host, control_port, opts.mode == TEST_MODE_SERVER);

    run_tests(TEST_CASES.as_ptr(), &opts);

    control_cleanup();

    0
}
