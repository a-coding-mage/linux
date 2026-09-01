// SPDX-License-Identifier: GPL-2.0
// C source defined _GNU_SOURCE and included libc, libbpf, and kselftest_harness.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type __u32 = u32;
type __u64 = u64;
type size_t = usize;
type ssize_t = isize;
type socklen_t = u32;
type pid_t = c_int;
type off_t = i64;
type ino_t = u64;

const O_RDONLY: c_int = 0;
const AF_UNIX: c_int = 1;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const SOCK_SEQPACKET: c_int = 5;
const SOL_SOCKET: c_int = 1;
const SCM_RIGHTS: c_int = 1;
const MSG_CTRUNC: c_int = 0x8;
const BPF_ANY: __u64 = 0;
const EPERM: c_int = 1;

// #ifndef SO_RIGHTS_NOTRUNC
const SO_RIGHTS_NOTRUNC: c_int = 85;
// #endif

const NR_FILES: usize = 2;

/* Per-file content, so a received fd can be matched to the file sent */
macro_rules! SECRET {
    ($n:expr) => {
        b"secret %d\0".as_ptr() as *const c_char,
        $n
    };
}

/* Indices into the socketpair */
const SK_SENDER: usize = 0;
const SK_RECEIVER: usize = 1;

#[repr(C)]
struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct stat {
    st_dev: u64,
    st_ino: ino_t,
    st_nlink: u64,
    st_mode: c_uint,
    st_uid: c_uint,
    st_gid: c_uint,
    __pad0: c_int,
    st_rdev: u64,
    st_size: c_long,
    st_blksize: c_long,
    st_blocks: c_long,
    st_atime: c_long,
    st_atime_nsec: c_long,
    st_mtime: c_long,
    st_mtime_nsec: c_long,
    st_ctime: c_long,
    st_ctime_nsec: c_long,
    __unused: [c_long; 3],
}

#[repr(C)]
struct iovec {
    iov_base: *mut c_void,
    iov_len: size_t,
}

#[repr(C)]
struct msghdr {
    msg_name: *mut c_void,
    msg_namelen: socklen_t,
    msg_iov: *mut iovec,
    msg_iovlen: size_t,
    msg_control: *mut c_void,
    msg_controllen: size_t,
    msg_flags: c_int,
}

#[repr(C)]
struct cmsghdr {
    cmsg_len: size_t,
    cmsg_level: c_int,
    cmsg_type: c_int,
}

#[repr(C)]
struct scm_rights_denial_bpf {
    obj: *mut bpf_object,
    link: *mut bpf_link,
    map_fd: c_int,
    sk: [c_int; 2],
    files: [c_int; NR_FILES],
    inos: [__u64; NR_FILES],
    paths: [[c_char; 64]; NR_FILES],
}

#[repr(C)]
struct scm_rights_denial_bpf_variant {
    sock_type: c_int,
}

static stream: scm_rights_denial_bpf_variant = scm_rights_denial_bpf_variant {
    sock_type: SOCK_STREAM,
};

static dgram: scm_rights_denial_bpf_variant = scm_rights_denial_bpf_variant {
    sock_type: SOCK_DGRAM,
};

static seqpacket: scm_rights_denial_bpf_variant = scm_rights_denial_bpf_variant {
    sock_type: SOCK_SEQPACKET,
};

unsafe extern "C" {
    fn geteuid() -> c_uint;
    fn getpid() -> pid_t;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn dprintf(fd: c_int, format: *const c_char, ...) -> c_int;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn mkstemp(template: *mut c_char) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn sendmsg(sockfd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t;
    fn recvmsg(sockfd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn pread(fd: c_int, buf: *mut c_void, count: size_t, offset: off_t) -> ssize_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

    fn bpf_object__open_file(path: *const c_char, opts: *const c_void) -> *mut bpf_object;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_program__attach_lsm(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_object__find_map_fd_by_name(obj: *mut bpf_object, name: *const c_char) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_object__close(obj: *mut bpf_object);
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: __u64,
    ) -> c_int;
}

const fn cmsg_align(len: usize) -> usize {
    (len + size_of::<usize>() - 1) & !(size_of::<usize>() - 1)
}

const fn CMSG_SPACE(len: usize) -> usize {
    cmsg_align(size_of::<cmsghdr>()) + cmsg_align(len)
}

const fn CMSG_LEN(len: usize) -> usize {
    cmsg_align(size_of::<cmsghdr>()) + len
}

unsafe fn CMSG_FIRSTHDR(mhdr: *mut msghdr) -> *mut cmsghdr {
    if (*mhdr).msg_controllen >= size_of::<cmsghdr>() {
        (*mhdr).msg_control as *mut cmsghdr
    } else {
        ptr::null_mut()
    }
}

unsafe fn CMSG_DATA(cmsg: *mut cmsghdr) -> *mut c_uchar {
    (cmsg as *mut c_uchar).add(cmsg_align(size_of::<cmsghdr>()))
}

type c_uchar = u8;

// kselftest_harness assertion and skip macros are external harness behavior in C.
macro_rules! ASSERT_GE {
    ($left:expr, $right:expr) => {
        assert!($left >= $right)
    };
}
macro_rules! ASSERT_LT {
    ($left:expr, $right:expr) => {
        assert!($left < $right)
    };
}
macro_rules! ASSERT_NE {
    ($left:expr, $right:expr) => {
        assert!($left != $right)
    };
}
macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert!($left == $right)
    };
}
macro_rules! EXPECT_EQ {
    ($left:expr, $right:expr) => {
        assert!($left == $right)
    };
}
macro_rules! EXPECT_NE {
    ($left:expr, $right:expr) => {
        assert!($left != $right)
    };
}
macro_rules! SKIP {
    (return, $msg:expr) => {
        return
    };
}

unsafe fn scm_rights_denial_bpf_setup(
    self_: *mut scm_rights_denial_bpf,
    variant: *const scm_rights_denial_bpf_variant,
) {
    let mut prog: *mut bpf_program;
    let mut lsms: [c_char; 256] = [0; 256];
    let mut i: c_int;
    let fd: c_int;

    if geteuid() != 0 {
        SKIP!(return, "requires root");
    }

    fd = open(b"/sys/kernel/security/lsm\0".as_ptr() as *const c_char, O_RDONLY);
    ASSERT_GE!(fd, 0);
    ASSERT_LT!(0, read(fd, lsms.as_mut_ptr() as *mut c_void, size_of::<[c_char; 256]>() - 1));
    close(fd);

    if strstr(lsms.as_ptr(), b"bpf\0".as_ptr() as *const c_char).is_null() {
        SKIP!(return, "BPF LSM not active (boot with lsm=...,bpf)");
    }

    (*self_).obj = bpf_object__open_file(
        b"scm_rights_denial_lsm.bpf.o\0".as_ptr() as *const c_char,
        ptr::null(),
    );
    ASSERT_NE!(ptr::null_mut(), (*self_).obj);
    ASSERT_EQ!(0, bpf_object__load((*self_).obj));

    prog = bpf_object__find_program_by_name(
        (*self_).obj,
        b"scm_rights_deny\0".as_ptr() as *const c_char,
    );
    ASSERT_NE!(ptr::null_mut(), prog);

    (*self_).link = bpf_program__attach_lsm(prog);
    ASSERT_NE!(ptr::null_mut(), (*self_).link);

    (*self_).map_fd = bpf_object__find_map_fd_by_name(
        (*self_).obj,
        b"denied_inodes\0".as_ptr() as *const c_char,
    );
    ASSERT_GE!((*self_).map_fd, 0);

    ASSERT_EQ!(0, socketpair(AF_UNIX, (*variant).sock_type, 0, (*self_).sk.as_mut_ptr()));

    i = 0;
    while i < NR_FILES as c_int {
        let mut st: stat = zeroed();
        let idx = i as usize;

        snprintf(
            (*self_).paths[idx].as_mut_ptr(),
            size_of::<[c_char; 64]>(),
            b"/tmp/scm_rights_denial_bpf.%d.XXXXXX\0".as_ptr() as *const c_char,
            i,
        );
        (*self_).files[idx] = mkstemp((*self_).paths[idx].as_mut_ptr());
        ASSERT_GE!((*self_).files[idx], 0);

        ASSERT_LT!(0, dprintf((*self_).files[idx], SECRET!(i)));

        ASSERT_EQ!(0, fstat((*self_).files[idx], &mut st));
        (*self_).inos[idx] = st.st_ino;

        i += 1;
    }
}

unsafe fn scm_rights_denial_bpf_teardown(self_: *mut scm_rights_denial_bpf) {
    bpf_link__destroy((*self_).link);
    bpf_object__close((*self_).obj);

    let mut i: c_int = 0;
    while i < NR_FILES as c_int {
        let idx = i as usize;
        if (*self_).files[idx] >= 0 {
            close((*self_).files[idx]);
            unlink((*self_).paths[idx].as_ptr());
        }
        i += 1;
    }

    close((*self_).sk[SK_SENDER]);
    close((*self_).sk[SK_RECEIVER]);
}

unsafe fn deny_inode(map_fd: c_int, ino: __u64) -> c_int {
    let tgid: __u32 = getpid() as __u32;

    bpf_map_update_elem(
        map_fd,
        &ino as *const __u64 as *const c_void,
        &tgid as *const __u32 as *const c_void,
        BPF_ANY,
    )
}

unsafe fn set_notrunc(sk: c_int) -> c_int {
    let one: c_int = 1;

    setsockopt(
        sk,
        SOL_SOCKET,
        SO_RIGHTS_NOTRUNC,
        &one as *const c_int as *const c_void,
        size_of::<c_int>() as socklen_t,
    )
}

unsafe fn send_fds(sk: c_int, fds: *mut c_int, n: c_int) -> c_int {
    let mut ctrl: [c_char; CMSG_SPACE(NR_FILES * size_of::<c_int>())] =
        [0; CMSG_SPACE(NR_FILES * size_of::<c_int>())];
    let mut data: c_char = b'x' as c_char;
    let mut iov = iovec {
        iov_base: &mut data as *mut c_char as *mut c_void,
        iov_len: size_of::<c_char>(),
    };
    let mut msg = msghdr {
        msg_name: ptr::null_mut(),
        msg_namelen: 0,
        msg_iov: &mut iov,
        msg_iovlen: 1,
        msg_control: ctrl.as_mut_ptr() as *mut c_void,
        msg_controllen: CMSG_SPACE(n as usize * size_of::<c_int>()),
        msg_flags: 0,
    };
    let cmsg: *mut cmsghdr = CMSG_FIRSTHDR(&mut msg);
    let ret: c_int;

    (*cmsg).cmsg_level = SOL_SOCKET;
    (*cmsg).cmsg_type = SCM_RIGHTS;
    (*cmsg).cmsg_len = CMSG_LEN(n as usize * size_of::<c_int>());
    memcpy(
        CMSG_DATA(cmsg) as *mut c_void,
        fds as *const c_void,
        n as usize * size_of::<c_int>(),
    );

    ret = sendmsg(sk, &msg, 0) as c_int;
    if ret != 1 {
        return -1;
    }

    0
}

unsafe fn recv_fd_slots(sk: c_int, slots: *mut c_int, msg_flags: *mut c_int) -> c_int {
    let mut nr_slots: c_int;
    let mut ctrl: [c_char; CMSG_SPACE(NR_FILES * size_of::<c_int>())] =
        [0; CMSG_SPACE(NR_FILES * size_of::<c_int>())];
    let mut data: c_char = 0;
    let mut iov = iovec {
        iov_base: &mut data as *mut c_char as *mut c_void,
        iov_len: size_of::<c_char>(),
    };
    let mut msg = msghdr {
        msg_name: ptr::null_mut(),
        msg_namelen: 0,
        msg_iov: &mut iov,
        msg_iovlen: 1,
        msg_control: ctrl.as_mut_ptr() as *mut c_void,
        msg_controllen: size_of::<[c_char; CMSG_SPACE(NR_FILES * size_of::<c_int>())]>(),
        msg_flags: 0,
    };
    let mut cmsg: *mut cmsghdr;

    if recvmsg(sk, &mut msg, 0) < 0 {
        return -1;
    }

    *msg_flags = msg.msg_flags;

    cmsg = CMSG_FIRSTHDR(&mut msg);
    if cmsg.is_null() {
        return 0;
    }

    nr_slots = (((*cmsg).cmsg_len - CMSG_LEN(0)) / size_of::<c_int>()) as c_int;
    memcpy(
        slots as *mut c_void,
        CMSG_DATA(cmsg) as *const c_void,
        nr_slots as usize * size_of::<c_int>(),
    );

    nr_slots
}

/* Prove a received fd works by reading back the file's content. */
unsafe fn check_secret(fd: c_int, idx: c_int) -> c_int {
    let mut want: [c_char; 32] = [0; 32];
    let mut got: [c_char; 32] = [0; 32];

    snprintf(want.as_mut_ptr(), size_of::<[c_char; 32]>(), SECRET!(idx));
    if pread(
        fd,
        got.as_mut_ptr() as *mut c_void,
        size_of::<[c_char; 32]>() - 1,
        0,
    ) < 0
    {
        return -1;
    }

    strcmp(want.as_ptr(), got.as_ptr())
}

unsafe fn scm_rights_denial_bpf_all_allowed(self_: *mut scm_rights_denial_bpf) {
    let mut slots: [c_int; NR_FILES] = [0; NR_FILES];
    let mut nr_slots: c_int;
    let mut flags: c_int = 0;

    ASSERT_EQ!(0, set_notrunc((*self_).sk[SK_RECEIVER]));
    ASSERT_EQ!(0, send_fds((*self_).sk[SK_SENDER], (*self_).files.as_mut_ptr(), NR_FILES as c_int));
    nr_slots = recv_fd_slots((*self_).sk[SK_RECEIVER], slots.as_mut_ptr(), &mut flags);

    ASSERT_EQ!(NR_FILES as c_int, nr_slots);
    EXPECT_EQ!(0, flags & MSG_CTRUNC);

    let mut i: c_int = 0;
    while i < nr_slots {
        ASSERT_GE!(slots[i as usize], 0);
        EXPECT_EQ!(0, check_secret(slots[i as usize], i));
        close(slots[i as usize]);
        i += 1;
    }
}

unsafe fn scm_rights_denial_bpf_first_denied(self_: *mut scm_rights_denial_bpf) {
    let mut slots: [c_int; NR_FILES] = [0; NR_FILES];
    let mut nr_slots: c_int;
    let mut flags: c_int = 0;

    ASSERT_EQ!(0, deny_inode((*self_).map_fd, (*self_).inos[0]));

    ASSERT_EQ!(0, set_notrunc((*self_).sk[SK_RECEIVER]));
    ASSERT_EQ!(0, send_fds((*self_).sk[SK_SENDER], (*self_).files.as_mut_ptr(), NR_FILES as c_int));
    nr_slots = recv_fd_slots((*self_).sk[SK_RECEIVER], slots.as_mut_ptr(), &mut flags);

    ASSERT_EQ!(NR_FILES as c_int, nr_slots);
    EXPECT_EQ!(0, flags & MSG_CTRUNC);

    EXPECT_EQ!(-EPERM, slots[0]);
    let mut i: c_int = 1;
    while i < nr_slots {
        ASSERT_GE!(slots[i as usize], 0);
        EXPECT_EQ!(0, check_secret(slots[i as usize], i));
        close(slots[i as usize]);
        i += 1;
    }
}

unsafe fn scm_rights_denial_bpf_all_denied(self_: *mut scm_rights_denial_bpf) {
    let mut slots: [c_int; NR_FILES] = [0; NR_FILES];
    let mut nr_slots: c_int;
    let mut flags: c_int = 0;
    let mut i: c_int;

    i = 0;
    while i < NR_FILES as c_int {
        ASSERT_EQ!(0, deny_inode((*self_).map_fd, (*self_).inos[i as usize]));
        i += 1;
    }

    ASSERT_EQ!(0, set_notrunc((*self_).sk[SK_RECEIVER]));
    ASSERT_EQ!(0, send_fds((*self_).sk[SK_SENDER], (*self_).files.as_mut_ptr(), NR_FILES as c_int));
    nr_slots = recv_fd_slots((*self_).sk[SK_RECEIVER], slots.as_mut_ptr(), &mut flags);

    ASSERT_EQ!(NR_FILES as c_int, nr_slots);
    EXPECT_EQ!(0, flags & MSG_CTRUNC);

    i = 0;
    while i < nr_slots {
        EXPECT_EQ!(-EPERM, slots[i as usize]);
        i += 1;
    }
}

unsafe fn scm_rights_denial_bpf_denied_without_notrunc(self_: *mut scm_rights_denial_bpf) {
    let mut slots: [c_int; NR_FILES] = [0; NR_FILES];
    let mut nr_slots: c_int;
    let mut flags: c_int = 0;

    /*
     * Baseline behaviour without SO_RIGHTS_NOTRUNC: the fd array is
     * truncated at the first denied fd and MSG_CTRUNC is set.
     */
    ASSERT_EQ!(0, deny_inode((*self_).map_fd, (*self_).inos[1]));

    ASSERT_EQ!(0, send_fds((*self_).sk[SK_SENDER], (*self_).files.as_mut_ptr(), NR_FILES as c_int));
    nr_slots = recv_fd_slots((*self_).sk[SK_RECEIVER], slots.as_mut_ptr(), &mut flags);

    ASSERT_EQ!(1, nr_slots);
    EXPECT_NE!(0, flags & MSG_CTRUNC);

    ASSERT_GE!(slots[0], 0);
    EXPECT_EQ!(0, check_secret(slots[0], 0));
    close(slots[0]);
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
