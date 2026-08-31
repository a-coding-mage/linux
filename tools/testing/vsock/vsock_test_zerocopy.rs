// SPDX-License-Identifier: GPL-2.0-only
/* MSG_ZEROCOPY feature tests for vsock
 *
 * Copyright (C) 2023 SberDevices.
 *
 * Author: Arseniy Krasnov <avkrasnov@salutedevices.com>
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem;
use core::ptr;

/* Dependencies from:
 * stdio.h, stdlib.h, string.h, sys/ioctl.h, sys/mman.h, unistd.h, poll.h,
 * linux/errqueue.h, linux/kernel.h, linux/sockios.h, linux/time64.h, errno.h,
 * control.h, timeout.h, vsock_test_zerocopy.h, msg_zerocopy_common.h.
 */

const PAGE_SIZE: usize = 4096;
const VSOCK_TEST_DATA_MAX_IOV: usize = 3;
const POLL_TIMEOUT_MS: c_int = 100;
const GOOD_COPY_LEN: usize = 128; /* net/vmw_vsock/virtio_transport_common.c */

const EXIT_FAILURE: c_int = 1;
const ENOMEM: c_int = 12;
const MSG_ZEROCOPY: c_int = 0x4000000;
const MSG_ERRQUEUE: c_int = 0x2000;
const POLLERR: c_short = 0x008;
const VMADDR_CID_ANY: c_uint = 0xFFFF_FFFF;
const SIOCINQ: c_ulong = 0x541B;
const MSEC_PER_SEC: c_int = 1000;

type c_short = i16;
type c_uint = u32;
type size_t = usize;
type ssize_t = isize;

#[repr(C)]
pub struct test_opts {
    pub peer_cid: c_uint,
    pub peer_port: c_uint,
}

#[repr(C)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: size_t,
}

#[repr(C)]
pub struct msghdr {
    pub msg_name: *mut c_void,
    pub msg_namelen: c_uint,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: size_t,
    pub msg_control: *mut c_void,
    pub msg_controllen: size_t,
    pub msg_flags: c_int,
}

#[repr(C)]
pub struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}

#[repr(C)]
struct vsock_test_data {
    /* This test case if for SOCK_STREAM only. */
    stream_only: bool,
    /* Data must be zerocopied. This field is checked against
     * field 'ee_code' of the 'struct sock_extended_err', which
     * contains bit to detect that zerocopy transmission was
     * fallbacked to copy mode.
     */
    zerocopied: bool,
    /* Enable SO_ZEROCOPY option on the socket. Without enabled
     * SO_ZEROCOPY, every MSG_ZEROCOPY transmission will behave
     * like without MSG_ZEROCOPY flag.
     */
    so_zerocopy: bool,
    /* 'errno' after 'sendmsg()' call. */
    sendmsg_errno: c_int,
    /* Number of valid elements in 'vecs'. */
    vecs_cnt: c_int,
    vecs: [iovec; VSOCK_TEST_DATA_MAX_IOV],
}

unsafe extern "C" {
    static mut errno: c_int;
    static TIMEOUT: c_int;

    fn perror(s: *const c_char);
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn exit(status: c_int) -> !;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn rand() -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn sendmsg(fd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t;
    fn recvmsg(fd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t;
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;

    fn vsock_seqpacket_connect(cid: c_uint, port: c_uint) -> c_int;
    fn vsock_stream_connect(cid: c_uint, port: c_uint) -> c_int;
    fn vsock_seqpacket_accept(cid: c_uint, port: c_uint, clientaddrp: *mut c_void) -> c_int;
    fn vsock_stream_accept(cid: c_uint, port: c_uint, clientaddrp: *mut c_void) -> c_int;
    fn enable_so_zerocopy_check(fd: c_int);
    fn alloc_test_iovec(vecs: *const iovec, vecs_cnt: c_int) -> *mut iovec;
    fn free_test_iovec(orig_vecs: *const iovec, iovec: *mut iovec, vecs_cnt: c_int);
    fn iovec_bytes(iovec: *const iovec, vecs_cnt: c_int) -> ssize_t;
    fn iovec_hash_djb2(iovec: *const iovec, vecs_cnt: c_int) -> c_ulong;
    fn hash_djb2(data: *const c_void, len: size_t) -> c_ulong;
    fn vsock_recv_completion(fd: c_int, zerocopied: *const bool);
    fn control_writeulong(value: c_ulong);
    fn control_readulong() -> c_ulong;
    fn control_writeln(str_: *const c_char);
    fn control_expectln(str_: *const c_char);
    fn send_buf(fd: c_int, buf: *const c_void, len: size_t, flags: c_int, expected_ret: size_t);
    fn recv_buf(fd: c_int, buf: *mut c_void, len: size_t, flags: c_int, expected_ret: size_t);
    fn vsock_ioctl_int(fd: c_int, request: c_ulong, expected: c_int);
}

static mut test_data_array: [vsock_test_data; 7] = [
    /* Last element has non-page aligned size. */
    vsock_test_data {
        stream_only: false,
        zerocopied: true,
        so_zerocopy: true,
        sendmsg_errno: 0,
        vecs_cnt: 3,
        vecs: [
            iovec { iov_base: ptr::null_mut(), iov_len: PAGE_SIZE },
            iovec { iov_base: ptr::null_mut(), iov_len: PAGE_SIZE },
            iovec { iov_base: ptr::null_mut(), iov_len: 200 },
        ],
    },
    /* All elements have page aligned base and size. */
    vsock_test_data {
        stream_only: false,
        zerocopied: true,
        so_zerocopy: true,
        sendmsg_errno: 0,
        vecs_cnt: 3,
        vecs: [
            iovec { iov_base: ptr::null_mut(), iov_len: PAGE_SIZE },
            iovec { iov_base: ptr::null_mut(), iov_len: PAGE_SIZE * 2 },
            iovec { iov_base: ptr::null_mut(), iov_len: PAGE_SIZE * 3 },
        ],
    },
    /* All elements have page aligned base and size. But
     * data length is bigger than 64Kb.
     */
    vsock_test_data {
        stream_only: false,
        zerocopied: true,
        so_zerocopy: true,
        sendmsg_errno: 0,
        vecs_cnt: 3,
        vecs: [
            iovec { iov_base: ptr::null_mut(), iov_len: PAGE_SIZE * 16 },
            iovec { iov_base: ptr::null_mut(), iov_len: PAGE_SIZE * 16 },
            iovec { iov_base: ptr::null_mut(), iov_len: PAGE_SIZE * 16 },
        ],
    },
    /* Middle element has both non-page aligned base and size. */
    vsock_test_data {
        stream_only: false,
        zerocopied: true,
        so_zerocopy: true,
        sendmsg_errno: 0,
        vecs_cnt: 3,
        vecs: [
            iovec { iov_base: ptr::null_mut(), iov_len: PAGE_SIZE },
            iovec { iov_base: 1usize as *mut c_void, iov_len: 100 },
            iovec { iov_base: ptr::null_mut(), iov_len: PAGE_SIZE },
        ],
    },
    /* Middle element is unmapped. */
    vsock_test_data {
        stream_only: false,
        zerocopied: false,
        so_zerocopy: true,
        sendmsg_errno: ENOMEM,
        vecs_cnt: 3,
        vecs: [
            iovec { iov_base: ptr::null_mut(), iov_len: PAGE_SIZE },
            iovec { iov_base: !0usize as *mut c_void, iov_len: PAGE_SIZE },
            iovec { iov_base: ptr::null_mut(), iov_len: PAGE_SIZE },
        ],
    },
    /* Valid data, but SO_ZEROCOPY is off. This
     * will trigger fallback to copy.
     */
    vsock_test_data {
        stream_only: false,
        zerocopied: false,
        so_zerocopy: false,
        sendmsg_errno: 0,
        vecs_cnt: 1,
        vecs: [
            iovec { iov_base: ptr::null_mut(), iov_len: PAGE_SIZE },
            iovec { iov_base: ptr::null_mut(), iov_len: 0 },
            iovec { iov_base: ptr::null_mut(), iov_len: 0 },
        ],
    },
    /* Valid data, but message is bigger than peer's
     * buffer, so this will trigger fallback to copy.
     * This test is for SOCK_STREAM only, because
     * for SOCK_SEQPACKET, 'sendmsg()' returns EMSGSIZE.
     */
    vsock_test_data {
        stream_only: true,
        zerocopied: false,
        so_zerocopy: true,
        sendmsg_errno: 0,
        vecs_cnt: 1,
        vecs: [
            iovec { iov_base: ptr::null_mut(), iov_len: 100 * PAGE_SIZE },
            iovec { iov_base: ptr::null_mut(), iov_len: 0 },
            iovec { iov_base: ptr::null_mut(), iov_len: 0 },
        ],
    },
];

unsafe fn test_client(opts: *const test_opts, test_data: *const vsock_test_data, sock_seqpacket: bool) {
    let mut fds: pollfd = mem::zeroed();
    let mut msg: msghdr = mem::zeroed();
    let sendmsg_res: ssize_t;
    let iovec: *mut iovec;
    let fd: c_int;

    if sock_seqpacket {
        fd = vsock_seqpacket_connect((*opts).peer_cid, (*opts).peer_port);
    } else {
        fd = vsock_stream_connect((*opts).peer_cid, (*opts).peer_port);
    }

    if fd < 0 {
        perror(c"connect".as_ptr());
        exit(EXIT_FAILURE);
    }

    if (*test_data).so_zerocopy {
        enable_so_zerocopy_check(fd);
    }

    iovec = alloc_test_iovec((*test_data).vecs.as_ptr(), (*test_data).vecs_cnt);

    msg.msg_iov = iovec;
    msg.msg_iovlen = (*test_data).vecs_cnt as size_t;

    errno = 0;

    sendmsg_res = sendmsg(fd, &msg, MSG_ZEROCOPY);
    if errno != (*test_data).sendmsg_errno {
        fprintf(
            stderr,
            c"expected 'errno' == %i, got %i\n".as_ptr(),
            (*test_data).sendmsg_errno,
            errno,
        );
        exit(EXIT_FAILURE);
    }

    if errno == 0 {
        if sendmsg_res != iovec_bytes(iovec, (*test_data).vecs_cnt) {
            fprintf(
                stderr,
                c"expected 'sendmsg()' == %li, got %li\n".as_ptr(),
                iovec_bytes(iovec, (*test_data).vecs_cnt) as c_long,
                sendmsg_res as c_long,
            );
            exit(EXIT_FAILURE);
        }
    }

    fds.fd = fd;
    fds.events = 0;

    if poll(&mut fds, 1, POLL_TIMEOUT_MS) < 0 {
        perror(c"poll".as_ptr());
        exit(EXIT_FAILURE);
    }

    if (fds.revents & POLLERR) != 0 {
        vsock_recv_completion(fd, &(*test_data).zerocopied);
    } else if (*test_data).so_zerocopy && (*test_data).sendmsg_errno == 0 {
        /* If we don't have data in the error queue, but
         * SO_ZEROCOPY was enabled and 'sendmsg()' was
         * successful - this is an error.
         */
        fprintf(stderr, c"POLLERR expected\n".as_ptr());
        exit(EXIT_FAILURE);
    }

    if (*test_data).sendmsg_errno == 0 {
        control_writeulong(iovec_hash_djb2(iovec, (*test_data).vecs_cnt));
    } else {
        control_writeulong(0);
    }

    control_writeln(c"DONE".as_ptr());
    free_test_iovec((*test_data).vecs.as_ptr(), iovec, (*test_data).vecs_cnt);
    close(fd);
}

#[no_mangle]
pub unsafe extern "C" fn test_stream_msgzcopy_client(opts: *const test_opts) {
    let mut i: c_int;

    i = 0;
    while (i as usize) < test_data_array.len() {
        test_client(opts, &test_data_array[i as usize], false);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_seqpacket_msgzcopy_client(opts: *const test_opts) {
    let mut i: c_int;

    i = 0;
    while (i as usize) < test_data_array.len() {
        if test_data_array[i as usize].stream_only {
            i += 1;
            continue;
        }

        test_client(opts, &test_data_array[i as usize], true);
        i += 1;
    }
}

unsafe fn test_server(opts: *const test_opts, test_data: *const vsock_test_data, sock_seqpacket: bool) {
    let remote_hash: c_ulong;
    let local_hash: c_ulong;
    let mut total_bytes_rec: ssize_t;
    let data: *mut u8;
    let data_len: size_t;
    let fd: c_int;

    if sock_seqpacket {
        fd = vsock_seqpacket_accept(VMADDR_CID_ANY, (*opts).peer_port, ptr::null_mut());
    } else {
        fd = vsock_stream_accept(VMADDR_CID_ANY, (*opts).peer_port, ptr::null_mut());
    }

    if fd < 0 {
        perror(c"accept".as_ptr());
        exit(EXIT_FAILURE);
    }

    data_len = iovec_bytes((*test_data).vecs.as_ptr(), (*test_data).vecs_cnt) as size_t;

    data = malloc(data_len) as *mut u8;
    if data.is_null() {
        perror(c"malloc".as_ptr());
        exit(EXIT_FAILURE);
    }

    total_bytes_rec = 0;

    while total_bytes_rec != data_len as ssize_t {
        let bytes_rec: ssize_t;

        bytes_rec = read(
            fd,
            data.offset(total_bytes_rec) as *mut c_void,
            data_len - total_bytes_rec as size_t,
        );
        if bytes_rec <= 0 {
            break;
        }

        total_bytes_rec += bytes_rec;
    }

    if (*test_data).sendmsg_errno == 0 {
        local_hash = hash_djb2(data as *const c_void, data_len);
    } else {
        local_hash = 0;
    }

    free(data as *mut c_void);

    /* Waiting for some result. */
    remote_hash = control_readulong();
    if remote_hash != local_hash {
        fprintf(stderr, c"hash mismatch\n".as_ptr());
        exit(EXIT_FAILURE);
    }

    control_expectln(c"DONE".as_ptr());
    close(fd);
}

#[no_mangle]
pub unsafe extern "C" fn test_stream_msgzcopy_server(opts: *const test_opts) {
    let mut i: c_int;

    i = 0;
    while (i as usize) < test_data_array.len() {
        test_server(opts, &test_data_array[i as usize], false);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_seqpacket_msgzcopy_server(opts: *const test_opts) {
    let mut i: c_int;

    i = 0;
    while (i as usize) < test_data_array.len() {
        if test_data_array[i as usize].stream_only {
            i += 1;
            continue;
        }

        test_server(opts, &test_data_array[i as usize], true);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_stream_msgzcopy_empty_errq_client(opts: *const test_opts) {
    let mut msg: msghdr = mem::zeroed();
    let mut cmsg_data: [c_char; 128] = [0; 128];
    let res: ssize_t;
    let fd: c_int;

    fd = vsock_stream_connect((*opts).peer_cid, (*opts).peer_port);
    if fd < 0 {
        perror(c"connect".as_ptr());
        exit(EXIT_FAILURE);
    }

    msg.msg_control = cmsg_data.as_mut_ptr() as *mut c_void;
    msg.msg_controllen = mem::size_of_val(&cmsg_data);

    res = recvmsg(fd, &mut msg, MSG_ERRQUEUE);
    if res != -1 {
        fprintf(
            stderr,
            c"expected 'recvmsg(2)' failure, got %zi\n".as_ptr(),
            res,
        );
        exit(EXIT_FAILURE);
    }

    control_writeln(c"DONE".as_ptr());
    close(fd);
}

#[no_mangle]
pub unsafe extern "C" fn test_stream_msgzcopy_empty_errq_server(opts: *const test_opts) {
    let fd: c_int;

    fd = vsock_stream_accept(VMADDR_CID_ANY, (*opts).peer_port, ptr::null_mut());
    if fd < 0 {
        perror(c"accept".as_ptr());
        exit(EXIT_FAILURE);
    }

    control_expectln(c"DONE".as_ptr());
    close(fd);
}

#[no_mangle]
pub unsafe extern "C" fn test_stream_msgzcopy_mangle_client(opts: *const test_opts) {
    let mut sbuf1: [c_char; PAGE_SIZE + 1] = [0; PAGE_SIZE + 1];
    let mut sbuf2: [c_char; GOOD_COPY_LEN] = [0; GOOD_COPY_LEN];
    let hash: c_ulong;
    let mut fds: pollfd = mem::zeroed();
    let fd: c_int;
    let mut i: c_int;

    fd = vsock_stream_connect((*opts).peer_cid, (*opts).peer_port);
    if fd < 0 {
        perror(c"connect".as_ptr());
        exit(EXIT_FAILURE);
    }

    enable_so_zerocopy_check(fd);

    memset(sbuf1.as_mut_ptr() as *mut c_void, 'x' as c_int, mem::size_of_val(&sbuf1));
    send_buf(fd, sbuf1.as_ptr() as *const c_void, mem::size_of_val(&sbuf1), 0, mem::size_of_val(&sbuf1));

    i = 0;
    while (i as usize) < mem::size_of_val(&sbuf2) {
        sbuf2[i as usize] = (rand() & 0xff) as c_char;
        i += 1;
    }

    send_buf(
        fd,
        sbuf2.as_ptr() as *const c_void,
        mem::size_of_val(&sbuf2),
        MSG_ZEROCOPY,
        mem::size_of_val(&sbuf2),
    );

    hash = hash_djb2(sbuf2.as_ptr() as *const c_void, mem::size_of_val(&sbuf2));
    control_writeulong(hash);

    fds.fd = fd;
    fds.events = 0;

    if poll(&mut fds, 1, TIMEOUT * MSEC_PER_SEC) != 1 || (fds.revents & POLLERR) == 0 {
        perror(c"poll".as_ptr());
        exit(EXIT_FAILURE);
    }

    close(fd);
}

#[no_mangle]
pub unsafe extern "C" fn test_stream_msgzcopy_mangle_server(opts: *const test_opts) {
    let local_hash: c_ulong;
    let remote_hash: c_ulong;
    let mut rbuf: [c_char; PAGE_SIZE + 1] = [0; PAGE_SIZE + 1];
    let fd: c_int;

    fd = vsock_stream_accept(VMADDR_CID_ANY, (*opts).peer_port, ptr::null_mut());
    if fd < 0 {
        perror(c"accept".as_ptr());
        exit(EXIT_FAILURE);
    }

    /* Wait, don't race the (buggy) skbs coalescence. */
    vsock_ioctl_int(fd, SIOCINQ, (PAGE_SIZE + 1 + GOOD_COPY_LEN) as c_int);

    /* Discard the first packet. */
    recv_buf(fd, rbuf.as_mut_ptr() as *mut c_void, PAGE_SIZE + 1, 0, PAGE_SIZE + 1);

    recv_buf(fd, rbuf.as_mut_ptr() as *mut c_void, GOOD_COPY_LEN, 0, GOOD_COPY_LEN);
    remote_hash = control_readulong();
    local_hash = hash_djb2(rbuf.as_ptr() as *const c_void, GOOD_COPY_LEN);

    if local_hash != remote_hash {
        fprintf(stderr, c"Data received corrupted\n".as_ptr());
        exit(EXIT_FAILURE);
    }

    close(fd);
}
