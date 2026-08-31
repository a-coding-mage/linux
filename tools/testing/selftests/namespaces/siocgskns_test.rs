// SPDX-License-Identifier: GPL-2.0
// Translated from C. Original includes supplied errno/fcntl/limits/sched/stdio/
// stdlib/string/sys socket/stat/types/wait/unistd, linux if/sockios/nsfs,
// arpa/inet, kselftest_harness, filesystems/utils, and wrappers.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

type c_int = i32;
type c_uint = u32;
type c_ulong = u64;
type c_char = i8;
type c_void = core::ffi::c_void;
type size_t = usize;
type ssize_t = isize;
type pid_t = i32;
type ino_t = u64;
type __u64 = u64;

const SIOCGSKNS: c_ulong = 0x894C;
const FD_NSFS_ROOT: c_int = -10003;
const FILEID_NSFS: c_int = 0xf1;

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const AF_LOCAL: c_int = 1;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const SOCK_RAW: c_int = 3;
const SOCK_CLOEXEC: c_int = 0o2000000;
const SOL_SOCKET: c_int = 1;
const SCM_RIGHTS: c_int = 1;
const IPPROTO_ICMP: c_int = 1;
const O_RDONLY: c_int = 0;
const CLONE_NEWNET: c_int = 0x40000000;
const CLONE_NEWUSER: c_int = 0x10000000;

const ENOTTY: c_int = 25;
const EINVAL: c_int = 22;
const EPERM: c_int = 1;
const EACCES: c_int = 13;
const ENOSYS: c_int = 38;
const EOPNOTSUPP: c_int = 95;
const EBADF: c_int = 9;

const NS_GET_USERNS: c_ulong = 0xb701;
const NS_GET_ID: c_ulong = 0xb705;

#[repr(C)]
struct stat {
    st_dev: u64,
    st_ino: ino_t,
    _rest: [u8; 128],
}

#[repr(C)]
struct iovec {
    iov_base: *mut c_void,
    iov_len: size_t,
}

#[repr(C)]
struct msghdr {
    msg_name: *mut c_void,
    msg_namelen: c_uint,
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
struct file_handle {
    handle_bytes: c_uint,
    handle_type: c_int,
    f_handle: [u8; 0],
}

#[repr(C)]
struct nsfs_file_handle {
    ns_id: __u64,
    ns_type: c_int,
    ns_inum: ino_t,
}

#[repr(C)]
struct ns_id_req {
    size: c_uint,
    spare: c_uint,
    ns_id: __u64,
    ns_type: c_uint,
    spare2: c_uint,
    user_ns_id: __u64,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn open_by_handle_at(mount_fd: c_int, handle: *mut file_handle, flags: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn fork() -> pid_t;
    fn unshare(flags: c_int) -> c_int;
    fn setns(fd: c_int, nstype: c_int) -> c_int;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn exit(status: c_int) -> !;
    fn sendmsg(sockfd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t;
    fn recvmsg(sockfd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;

    fn setup_userns() -> c_int;
    fn sys_listns(req: *mut ns_id_req, ids: *mut __u64, nr_ids: size_t, flags: c_uint) -> c_int;
}

macro_rules! TEST { ($name:ident, $body:block) => { unsafe fn $name() $body }; }
macro_rules! ASSERT_GE { ($a:expr, $b:expr) => { assert!(($a) >= ($b)); }; }
macro_rules! ASSERT_GT { ($a:expr, $b:expr) => { assert!(($a) > ($b)); }; }
macro_rules! ASSERT_LT { ($a:expr, $b:expr) => { assert!(($a) < ($b)); }; }
macro_rules! ASSERT_EQ { ($a:expr, $b:expr) => { assert_eq!(($a), ($b)); }; }
macro_rules! ASSERT_NE { ($a:expr, $b:expr) => { assert!(($a) != ($b)); }; }
macro_rules! ASSERT_TRUE { ($a:expr) => { assert!($a); }; }
macro_rules! ASSERT_FALSE { ($a:expr) => { assert!(!$a); }; }
macro_rules! EXPECT_EQ { ($a:expr, $b:expr) => { assert_eq!(($a), ($b)); }; }
macro_rules! TH_LOG { ($($arg:tt)*) => {{ let _ = format_args!($($arg)*); }}; }
macro_rules! SKIP { (return, $msg:expr) => { return; }; }

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

fn CMSG_ALIGN(len: size_t) -> size_t {
    (len + core::mem::size_of::<size_t>() - 1) & !(core::mem::size_of::<size_t>() - 1)
}

fn CMSG_SPACE(len: size_t) -> size_t {
    CMSG_ALIGN(core::mem::size_of::<cmsghdr>()) + CMSG_ALIGN(len)
}

unsafe fn CMSG_FIRSTHDR(mhdr: *mut msghdr) -> *mut cmsghdr {
    if (*mhdr).msg_controllen >= core::mem::size_of::<cmsghdr>() {
        (*mhdr).msg_control as *mut cmsghdr
    } else {
        core::ptr::null_mut()
    }
}

unsafe fn CMSG_DATA(cmsg: *mut cmsghdr) -> *mut u8 {
    (cmsg as *mut u8).add(CMSG_ALIGN(core::mem::size_of::<cmsghdr>()))
}

fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> size_t {
    N
}

unsafe fn send_fd(sock: c_int, fd: c_int, byte: u8) -> ssize_t {
    let mut msg: msghdr = core::mem::zeroed();
    let mut iov: iovec = core::mem::zeroed();
    let mut buf = [byte as c_char; 1];
    let mut cmsg_buf = [0u8; CMSG_SPACE(core::mem::size_of::<c_int>())];

    iov.iov_base = buf.as_mut_ptr() as *mut c_void;
    iov.iov_len = 1;
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    msg.msg_control = cmsg_buf.as_mut_ptr() as *mut c_void;
    msg.msg_controllen = cmsg_buf.len();

    let cmsg = CMSG_FIRSTHDR(&mut msg);
    (*cmsg).cmsg_level = SOL_SOCKET;
    (*cmsg).cmsg_type = SCM_RIGHTS;
    (*cmsg).cmsg_len = CMSG_ALIGN(core::mem::size_of::<cmsghdr>()) + core::mem::size_of::<c_int>();
    memcpy(CMSG_DATA(cmsg) as *mut c_void, &fd as *const _ as *const c_void, core::mem::size_of::<c_int>());

    sendmsg(sock, &msg, 0)
}

unsafe fn recv_fd(sock: c_int, byte: *mut c_char) -> c_int {
    let mut msg: msghdr = core::mem::zeroed();
    let mut iov: iovec = core::mem::zeroed();
    let mut buf = [0 as c_char; 1];
    let mut cmsg_buf = [0u8; CMSG_SPACE(core::mem::size_of::<c_int>())];
    let mut received_fd: c_int = -1;

    iov.iov_base = buf.as_mut_ptr() as *mut c_void;
    iov.iov_len = 1;
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    msg.msg_control = cmsg_buf.as_mut_ptr() as *mut c_void;
    msg.msg_controllen = cmsg_buf.len();

    let n = recvmsg(sock, &mut msg, 0);
    if !byte.is_null() {
        *byte = buf[0];
    }
    if n != 1 {
        return -1;
    }

    let cmsg = CMSG_FIRSTHDR(&mut msg);
    if cmsg.is_null() {
        return -1;
    }
    memcpy(&mut received_fd as *mut _ as *mut c_void, CMSG_DATA(cmsg) as *const c_void, core::mem::size_of::<c_int>());
    received_fd
}

/*
 * Test basic SIOCGSKNS functionality.
 * Create a socket and verify SIOCGSKNS returns the correct network namespace.
 */
TEST!(siocgskns_basic, {
    let sock_fd: c_int;
    let netns_fd: c_int;
    let current_netns_fd: c_int;
    let mut st1: stat = core::mem::zeroed();
    let mut st2: stat = core::mem::zeroed();

    /* Create a TCP socket */
    sock_fd = socket(AF_INET, SOCK_STREAM, 0);
    ASSERT_GE!(sock_fd, 0);

    /* Use SIOCGSKNS to get network namespace */
    netns_fd = ioctl(sock_fd, SIOCGSKNS);
    if netns_fd < 0 {
        close(sock_fd);
        if errno == ENOTTY || errno == EINVAL {
            SKIP!(return, "SIOCGSKNS not supported");
        }
        ASSERT_GE!(netns_fd, 0);
    }

    /* Get current network namespace */
    current_netns_fd = open(c"/proc/self/ns/net".as_ptr(), O_RDONLY);
    ASSERT_GE!(current_netns_fd, 0);

    /* Verify they match */
    ASSERT_EQ!(fstat(netns_fd, &mut st1), 0);
    ASSERT_EQ!(fstat(current_netns_fd, &mut st2), 0);
    ASSERT_EQ!(st1.st_ino, st2.st_ino);

    close(sock_fd);
    close(netns_fd);
    close(current_netns_fd);
});

/*
 * Test that socket file descriptors keep network namespaces active.
 * Create a network namespace, create a socket in it, then exit the namespace.
 * The namespace should remain active while the socket FD is held.
 */
TEST!(siocgskns_keeps_netns_active, {
    let mut sock_fd: c_int = 0;
    let netns_fd: c_int;
    let test_fd: c_int;
    let mut ipc_sockets = [0 as c_int; 2];
    let pid: pid_t;
    let mut status: c_int = 0;
    let mut st: stat = core::mem::zeroed();

    EXPECT_EQ!(socketpair(AF_LOCAL, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr()), 0);

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        /* Child: create new netns and socket */
        close(ipc_sockets[0]);

        if unshare(CLONE_NEWNET) < 0 {
            TH_LOG!("unshare(CLONE_NEWNET) failed: {:?}", strerror(errno));
            close(ipc_sockets[1]);
            exit(1);
        }

        /* Create a socket in the new network namespace */
        sock_fd = socket(AF_INET, SOCK_DGRAM, 0);
        if sock_fd < 0 {
            TH_LOG!("socket() failed: {:?}", strerror(errno));
            close(ipc_sockets[1]);
            exit(1);
        }

        /* Send socket FD to parent via SCM_RIGHTS */
        if send_fd(ipc_sockets[1], sock_fd, b'X') < 0 {
            close(sock_fd);
            close(ipc_sockets[1]);
            exit(1);
        }

        close(sock_fd);
        close(ipc_sockets[1]);
        exit(0);
    }

    /* Parent: receive socket FD */
    close(ipc_sockets[1]);
    sock_fd = recv_fd(ipc_sockets[0], core::ptr::null_mut());
    close(ipc_sockets[0]);
    ASSERT_GE!(sock_fd, 0);

    /* Wait for child to exit */
    waitpid(pid, &mut status, 0);
    ASSERT_TRUE!(WIFEXITED(status));
    ASSERT_EQ!(WEXITSTATUS(status), 0);

    /* Get network namespace from socket */
    netns_fd = ioctl(sock_fd, SIOCGSKNS);
    if netns_fd < 0 {
        close(sock_fd);
        if errno == ENOTTY || errno == EINVAL {
            SKIP!(return, "SIOCGSKNS not supported");
        }
        ASSERT_GE!(netns_fd, 0);
    }

    ASSERT_EQ!(fstat(netns_fd, &mut st), 0);

    /*
     * Namespace should still be active because socket FD keeps it alive.
     * Try to access it via /proc/self/fd/<fd>.
     */
    let mut path = [0 as c_char; 64];
    snprintf(path.as_mut_ptr(), path.len(), c"/proc/self/fd/%d".as_ptr(), netns_fd);
    test_fd = open(path.as_ptr(), O_RDONLY);
    ASSERT_GE!(test_fd, 0);
    close(test_fd);
    close(netns_fd);

    /* Close socket - namespace should become inactive */
    close(sock_fd);

    /* Try SIOCGSKNS again - should fail since socket is closed */
    ASSERT_LT!(ioctl(sock_fd, SIOCGSKNS), 0);
});

/*
 * Test SIOCGSKNS with different socket types (TCP, UDP, RAW).
 */
TEST!(siocgskns_socket_types, {
    let sock_tcp: c_int;
    let sock_udp: c_int;
    let mut sock_raw: c_int;
    let netns_tcp: c_int;
    let netns_udp: c_int;
    let mut netns_raw: c_int = -1;
    let mut st_tcp: stat = core::mem::zeroed();
    let mut st_udp: stat = core::mem::zeroed();
    let mut st_raw: stat = core::mem::zeroed();

    /* TCP socket */
    sock_tcp = socket(AF_INET, SOCK_STREAM, 0);
    ASSERT_GE!(sock_tcp, 0);

    /* UDP socket */
    sock_udp = socket(AF_INET, SOCK_DGRAM, 0);
    ASSERT_GE!(sock_udp, 0);

    /* RAW socket (may require privileges) */
    sock_raw = socket(AF_INET, SOCK_RAW, IPPROTO_ICMP);
    if sock_raw < 0 && (errno == EPERM || errno == EACCES) {
        sock_raw = -1; /* Skip raw socket test */
    }

    /* Test SIOCGSKNS on TCP */
    netns_tcp = ioctl(sock_tcp, SIOCGSKNS);
    if netns_tcp < 0 {
        close(sock_tcp);
        close(sock_udp);
        if sock_raw >= 0 { close(sock_raw); }
        if errno == ENOTTY || errno == EINVAL {
            SKIP!(return, "SIOCGSKNS not supported");
        }
        ASSERT_GE!(netns_tcp, 0);
    }

    /* Test SIOCGSKNS on UDP */
    netns_udp = ioctl(sock_udp, SIOCGSKNS);
    ASSERT_GE!(netns_udp, 0);

    /* Test SIOCGSKNS on RAW (if available) */
    if sock_raw >= 0 {
        netns_raw = ioctl(sock_raw, SIOCGSKNS);
        ASSERT_GE!(netns_raw, 0);
    }

    /* Verify all return the same network namespace */
    ASSERT_EQ!(fstat(netns_tcp, &mut st_tcp), 0);
    ASSERT_EQ!(fstat(netns_udp, &mut st_udp), 0);
    ASSERT_EQ!(st_tcp.st_ino, st_udp.st_ino);

    if sock_raw >= 0 {
        ASSERT_EQ!(fstat(netns_raw, &mut st_raw), 0);
        ASSERT_EQ!(st_tcp.st_ino, st_raw.st_ino);
        close(netns_raw);
        close(sock_raw);
    }

    close(netns_tcp);
    close(netns_udp);
    close(sock_tcp);
    close(sock_udp);
});

/*
 * Test SIOCGSKNS across setns.
 * Create a socket in netns A, switch to netns B, verify SIOCGSKNS still
 * returns netns A.
 */
TEST!(siocgskns_across_setns, {
    let sock_fd: c_int;
    let netns_a_fd: c_int;
    let netns_b_fd: c_int;
    let result_fd: c_int;
    let mut st_a: stat = core::mem::zeroed();

    /* Get current netns (A) */
    netns_a_fd = open(c"/proc/self/ns/net".as_ptr(), O_RDONLY);
    ASSERT_GE!(netns_a_fd, 0);
    ASSERT_EQ!(fstat(netns_a_fd, &mut st_a), 0);

    /* Create socket in netns A */
    sock_fd = socket(AF_INET, SOCK_STREAM, 0);
    ASSERT_GE!(sock_fd, 0);

    /* Create new netns (B) */
    ASSERT_EQ!(unshare(CLONE_NEWNET), 0);

    netns_b_fd = open(c"/proc/self/ns/net".as_ptr(), O_RDONLY);
    ASSERT_GE!(netns_b_fd, 0);

    /* Get netns from socket created in A */
    result_fd = ioctl(sock_fd, SIOCGSKNS);
    if result_fd < 0 {
        close(sock_fd);
        setns(netns_a_fd, CLONE_NEWNET);
        close(netns_a_fd);
        close(netns_b_fd);
        if errno == ENOTTY || errno == EINVAL {
            SKIP!(return, "SIOCGSKNS not supported");
        }
        ASSERT_GE!(result_fd, 0);
    }

    /* Verify it still points to netns A */
    let mut st_result_stat: stat = core::mem::zeroed();
    ASSERT_EQ!(fstat(result_fd, &mut st_result_stat), 0);
    ASSERT_EQ!(st_a.st_ino, st_result_stat.st_ino);

    close(result_fd);
    close(sock_fd);
    close(netns_b_fd);

    /* Restore original netns */
    ASSERT_EQ!(setns(netns_a_fd, CLONE_NEWNET), 0);
    close(netns_a_fd);
});

/*
 * Test SIOCGSKNS fails on non-socket file descriptors.
 */
TEST!(siocgskns_non_socket, {
    let fd: c_int;
    let mut pipefd = [0 as c_int; 2];

    /* Test on regular file */
    fd = open(c"/dev/null".as_ptr(), O_RDONLY);
    ASSERT_GE!(fd, 0);

    ASSERT_LT!(ioctl(fd, SIOCGSKNS), 0);
    ASSERT_TRUE!(errno == ENOTTY || errno == EINVAL);
    close(fd);

    /* Test on pipe */
    ASSERT_EQ!(pipe(pipefd.as_mut_ptr()), 0);

    ASSERT_LT!(ioctl(pipefd[0], SIOCGSKNS), 0);
    ASSERT_TRUE!(errno == ENOTTY || errno == EINVAL);

    close(pipefd[0]);
    close(pipefd[1]);
});

/*
 * Test multiple sockets keep the same network namespace active.
 * Create multiple sockets, verify closing some doesn't affect others.
 */
TEST!(siocgskns_multiple_sockets, {
    let mut socks = [0 as c_int; 5];
    let mut netns_fds = [0 as c_int; 5];
    let mut st: stat = core::mem::zeroed();
    let netns_ino: ino_t;

    /* Create new network namespace */
    ASSERT_EQ!(unshare(CLONE_NEWNET), 0);

    /* Create multiple sockets */
    for i in 0..5 {
        socks[i] = socket(AF_INET, SOCK_STREAM, 0);
        ASSERT_GE!(socks[i], 0);
    }

    /* Get netns from all sockets */
    for i in 0..5 {
        netns_fds[i] = ioctl(socks[i], SIOCGSKNS);
        if netns_fds[i] < 0 {
            for j in 0..=i {
                close(socks[j]);
                if j < i && netns_fds[j] >= 0 {
                    close(netns_fds[j]);
                }
            }
            if errno == ENOTTY || errno == EINVAL {
                SKIP!(return, "SIOCGSKNS not supported");
            }
            ASSERT_GE!(netns_fds[i], 0);
        }
    }

    /* Verify all point to same netns */
    ASSERT_EQ!(fstat(netns_fds[0], &mut st), 0);
    netns_ino = st.st_ino;

    for i in 1..5 {
        ASSERT_EQ!(fstat(netns_fds[i], &mut st), 0);
        ASSERT_EQ!(st.st_ino, netns_ino);
    }

    /* Close some sockets */
    for i in 0..3 {
        close(socks[i]);
    }

    /* Remaining netns FDs should still be valid */
    for i in 3..5 {
        let mut path = [0 as c_char; 64];
        snprintf(path.as_mut_ptr(), path.len(), c"/proc/self/fd/%d".as_ptr(), netns_fds[i]);
        let test_fd = open(path.as_ptr(), O_RDONLY);
        ASSERT_GE!(test_fd, 0);
        close(test_fd);
    }

    /* Cleanup */
    for i in 0..5 {
        if i >= 3 {
            close(socks[i]);
        }
        close(netns_fds[i]);
    }
});

/*
 * Test socket keeps netns active after creating process exits.
 * Verify that as long as the socket FD exists, the namespace remains active.
 */
TEST!(siocgskns_netns_lifecycle, {
    let mut sock_fd: c_int = 0;
    let mut netns_fd: c_int;
    let mut ipc_sockets = [0 as c_int; 2];
    let mut syncpipe = [0 as c_int; 2];
    let pid: pid_t;
    let mut status: c_int = 0;
    let mut sync_byte: c_char = 0;
    let mut st: stat = core::mem::zeroed();
    let netns_ino: ino_t;

    EXPECT_EQ!(socketpair(AF_LOCAL, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr()), 0);

    ASSERT_EQ!(pipe(syncpipe.as_mut_ptr()), 0);

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        /* Child */
        close(ipc_sockets[0]);
        close(syncpipe[1]);

        if unshare(CLONE_NEWNET) < 0 {
            close(ipc_sockets[1]);
            close(syncpipe[0]);
            exit(1);
        }

        sock_fd = socket(AF_INET, SOCK_STREAM, 0);
        if sock_fd < 0 {
            close(ipc_sockets[1]);
            close(syncpipe[0]);
            exit(1);
        }

        /* Send socket to parent */
        if send_fd(ipc_sockets[1], sock_fd, b'X') < 0 {
            close(sock_fd);
            close(ipc_sockets[1]);
            close(syncpipe[0]);
            exit(1);
        }

        close(sock_fd);
        close(ipc_sockets[1]);

        /* Wait for parent signal */
        read(syncpipe[0], &mut sync_byte as *mut _ as *mut c_void, 1);
        close(syncpipe[0]);
        exit(0);
    }

    /* Parent */
    close(ipc_sockets[1]);
    close(syncpipe[0]);

    /* Receive socket FD */
    sock_fd = recv_fd(ipc_sockets[0], core::ptr::null_mut());
    close(ipc_sockets[0]);
    ASSERT_GE!(sock_fd, 0);

    /* Get netns from socket while child is alive */
    netns_fd = ioctl(sock_fd, SIOCGSKNS);
    if netns_fd < 0 {
        sync_byte = b'G' as c_char;
        write(syncpipe[1], &sync_byte as *const _ as *const c_void, 1);
        close(syncpipe[1]);
        close(sock_fd);
        waitpid(pid, core::ptr::null_mut(), 0);
        if errno == ENOTTY || errno == EINVAL {
            SKIP!(return, "SIOCGSKNS not supported");
        }
        ASSERT_GE!(netns_fd, 0);
    }
    ASSERT_EQ!(fstat(netns_fd, &mut st), 0);
    netns_ino = st.st_ino;

    /* Signal child to exit */
    sync_byte = b'G' as c_char;
    write(syncpipe[1], &sync_byte as *const _ as *const c_void, 1);
    close(syncpipe[1]);

    waitpid(pid, &mut status, 0);
    ASSERT_TRUE!(WIFEXITED(status));

    /*
     * Socket FD should still keep namespace active even after
     * the creating process exited.
     */
    let test_fd = ioctl(sock_fd, SIOCGSKNS);
    ASSERT_GE!(test_fd, 0);

    let mut st_test: stat = core::mem::zeroed();
    ASSERT_EQ!(fstat(test_fd, &mut st_test), 0);
    ASSERT_EQ!(st_test.st_ino, netns_ino);

    close(test_fd);
    close(netns_fd);

    /* Close socket - namespace should become inactive */
    close(sock_fd);
});

/*
 * Test IPv6 sockets also work with SIOCGSKNS.
 */
TEST!(siocgskns_ipv6, {
    let sock_fd: c_int;
    let netns_fd: c_int;
    let current_netns_fd: c_int;
    let mut st1: stat = core::mem::zeroed();
    let mut st2: stat = core::mem::zeroed();

    /* Create an IPv6 TCP socket */
    sock_fd = socket(AF_INET6, SOCK_STREAM, 0);
    ASSERT_GE!(sock_fd, 0);

    /* Use SIOCGSKNS */
    netns_fd = ioctl(sock_fd, SIOCGSKNS);
    if netns_fd < 0 {
        close(sock_fd);
        if errno == ENOTTY || errno == EINVAL {
            SKIP!(return, "SIOCGSKNS not supported");
        }
        ASSERT_GE!(netns_fd, 0);
    }

    /* Verify it matches current namespace */
    current_netns_fd = open(c"/proc/self/ns/net".as_ptr(), O_RDONLY);
    ASSERT_GE!(current_netns_fd, 0);

    ASSERT_EQ!(fstat(netns_fd, &mut st1), 0);
    ASSERT_EQ!(fstat(current_netns_fd, &mut st2), 0);
    ASSERT_EQ!(st1.st_ino, st2.st_ino);

    close(sock_fd);
    close(netns_fd);
    close(current_netns_fd);
});

unsafe fn alloc_nsfs_handle() -> *mut file_handle {
    let handle = malloc(core::mem::size_of::<file_handle>() + core::mem::size_of::<nsfs_file_handle>()) as *mut file_handle;
    if !handle.is_null() {
        (*handle).handle_bytes = core::mem::size_of::<nsfs_file_handle>() as c_uint;
        (*handle).handle_type = FILEID_NSFS;
    }
    handle
}

unsafe fn nsfs_fh_from_handle(handle: *mut file_handle) -> *mut nsfs_file_handle {
    (*handle).f_handle.as_mut_ptr() as *mut nsfs_file_handle
}

/*
 * Test that socket-kept netns appears in listns() output.
 * Verify that a network namespace kept alive by a socket FD appears in
 * listns() output even after the creating process exits, and that it
 * disappears when the socket is closed.
 */
TEST!(siocgskns_listns_visibility, {
    let mut sock_fd: c_int = 0;
    let netns_fd: c_int;
    let owner_fd: c_int;
    let mut ipc_sockets = [0 as c_int; 2];
    let pid: pid_t;
    let mut status: c_int = 0;
    let mut netns_id: __u64 = 0;
    let mut owner_id: __u64 = 0;
    let mut req = ns_id_req {
        size: core::mem::size_of::<ns_id_req>() as c_uint,
        spare: 0,
        ns_id: 0,
        ns_type: CLONE_NEWNET as c_uint,
        spare2: 0,
        user_ns_id: 0,
    };
    let mut ns_ids = [0 as __u64; 256];
    let mut ret: c_int;
    let mut found_netns = false;

    EXPECT_EQ!(socketpair(AF_LOCAL, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr()), 0);

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        /* Child: create new netns and socket */
        close(ipc_sockets[0]);

        if unshare(CLONE_NEWNET) < 0 {
            close(ipc_sockets[1]);
            exit(1);
        }

        sock_fd = socket(AF_INET, SOCK_DGRAM, 0);
        if sock_fd < 0 {
            close(ipc_sockets[1]);
            exit(1);
        }

        /* Send socket FD to parent via SCM_RIGHTS */
        if send_fd(ipc_sockets[1], sock_fd, b'X') < 0 {
            close(sock_fd);
            close(ipc_sockets[1]);
            exit(1);
        }

        close(sock_fd);
        close(ipc_sockets[1]);
        exit(0);
    }

    /* Parent: receive socket FD */
    close(ipc_sockets[1]);
    sock_fd = recv_fd(ipc_sockets[0], core::ptr::null_mut());
    close(ipc_sockets[0]);
    ASSERT_GE!(sock_fd, 0);

    /* Wait for child to exit */
    waitpid(pid, &mut status, 0);
    ASSERT_TRUE!(WIFEXITED(status));
    ASSERT_EQ!(WEXITSTATUS(status), 0);

    /* Get network namespace from socket */
    netns_fd = ioctl(sock_fd, SIOCGSKNS);
    if netns_fd < 0 {
        close(sock_fd);
        if errno == ENOTTY || errno == EINVAL {
            SKIP!(return, "SIOCGSKNS not supported");
        }
        ASSERT_GE!(netns_fd, 0);
    }

    /* Get namespace ID */
    ret = ioctl(netns_fd, NS_GET_ID, &mut netns_id);
    if ret < 0 {
        close(sock_fd);
        close(netns_fd);
        if errno == ENOTTY || errno == EINVAL {
            SKIP!(return, "NS_GET_ID not supported");
        }
        ASSERT_EQ!(ret, 0);
    }

    /* Get owner user namespace */
    owner_fd = ioctl(netns_fd, NS_GET_USERNS);
    if owner_fd < 0 {
        close(sock_fd);
        close(netns_fd);
        if errno == ENOTTY || errno == EINVAL {
            SKIP!(return, "NS_GET_USERNS not supported");
        }
        ASSERT_GE!(owner_fd, 0);
    }

    /* Get owner namespace ID */
    ret = ioctl(owner_fd, NS_GET_ID, &mut owner_id);
    if ret < 0 {
        close(owner_fd);
        close(sock_fd);
        close(netns_fd);
        ASSERT_EQ!(ret, 0);
    }
    close(owner_fd);

    /* Namespace should appear in listns() output */
    ret = sys_listns(&mut req, ns_ids.as_mut_ptr(), ARRAY_SIZE(&ns_ids), 0);
    if ret < 0 {
        close(sock_fd);
        close(netns_fd);
        if errno == ENOSYS {
            SKIP!(return, "listns() not supported");
        }
        TH_LOG!("listns failed: {:?}", strerror(errno));
        ASSERT_GE!(ret, 0);
    }

    /* Search for our network namespace in the list */
    for i in 0..ret as usize {
        if ns_ids[i] == netns_id {
            found_netns = true;
            break;
        }
    }

    ASSERT_TRUE!(found_netns);
    TH_LOG!("Found netns {} in listns() output (kept alive by socket)", netns_id);

    /* Now verify with owner filtering */
    req.user_ns_id = owner_id;
    found_netns = false;

    ret = sys_listns(&mut req, ns_ids.as_mut_ptr(), ARRAY_SIZE(&ns_ids), 0);
    ASSERT_GE!(ret, 0);

    for i in 0..ret as usize {
        if ns_ids[i] == netns_id {
            found_netns = true;
            break;
        }
    }

    ASSERT_TRUE!(found_netns);
    TH_LOG!("Found netns {} owned by userns {}", netns_id, owner_id);

    /* Close socket - namespace should become inactive and disappear from listns() */
    close(sock_fd);
    close(netns_fd);

    /* Verify it's no longer in listns() output */
    req.user_ns_id = 0;
    found_netns = false;

    ret = sys_listns(&mut req, ns_ids.as_mut_ptr(), ARRAY_SIZE(&ns_ids), 0);
    ASSERT_GE!(ret, 0);

    for i in 0..ret as usize {
        if ns_ids[i] == netns_id {
            found_netns = true;
            break;
        }
    }

    ASSERT_FALSE!(found_netns);
    TH_LOG!("Netns {} correctly disappeared from listns() after socket closed", netns_id);
});

/*
 * Test that socket-kept netns can be reopened via file handle.
 * Verify that a network namespace kept alive by a socket FD can be
 * reopened using file handles even after the creating process exits.
 */
TEST!(siocgskns_file_handle, {
    let mut sock_fd: c_int = 0;
    let mut netns_fd: c_int;
    let mut reopened_fd: c_int;
    let mut ipc_sockets = [0 as c_int; 2];
    let pid: pid_t;
    let mut status: c_int = 0;
    let mut st1: stat = core::mem::zeroed();
    let mut st2: stat = core::mem::zeroed();
    let netns_ino: ino_t;
    let mut netns_id: __u64 = 0;
    let handle: *mut file_handle;
    let nsfs_fh: *mut nsfs_file_handle;
    let mut ret: c_int;

    /* Allocate file_handle structure for nsfs */
    handle = alloc_nsfs_handle();
    ASSERT_NE!(handle, core::ptr::null_mut());

    EXPECT_EQ!(socketpair(AF_LOCAL, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr()), 0);

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        /* Child: create new netns and socket */
        close(ipc_sockets[0]);

        if unshare(CLONE_NEWNET) < 0 {
            close(ipc_sockets[1]);
            exit(1);
        }

        sock_fd = socket(AF_INET, SOCK_DGRAM, 0);
        if sock_fd < 0 {
            close(ipc_sockets[1]);
            exit(1);
        }

        /* Send socket FD to parent via SCM_RIGHTS */
        if send_fd(ipc_sockets[1], sock_fd, b'X') < 0 {
            close(sock_fd);
            close(ipc_sockets[1]);
            exit(1);
        }

        close(sock_fd);
        close(ipc_sockets[1]);
        exit(0);
    }

    /* Parent: receive socket FD */
    close(ipc_sockets[1]);
    sock_fd = recv_fd(ipc_sockets[0], core::ptr::null_mut());
    close(ipc_sockets[0]);
    ASSERT_GE!(sock_fd, 0);

    /* Wait for child to exit */
    waitpid(pid, &mut status, 0);
    ASSERT_TRUE!(WIFEXITED(status));
    ASSERT_EQ!(WEXITSTATUS(status), 0);

    /* Get network namespace from socket */
    netns_fd = ioctl(sock_fd, SIOCGSKNS);
    if netns_fd < 0 {
        free(handle as *mut c_void);
        close(sock_fd);
        if errno == ENOTTY || errno == EINVAL {
            SKIP!(return, "SIOCGSKNS not supported");
        }
        ASSERT_GE!(netns_fd, 0);
    }

    ASSERT_EQ!(fstat(netns_fd, &mut st1), 0);
    netns_ino = st1.st_ino;

    /* Get namespace ID */
    ret = ioctl(netns_fd, NS_GET_ID, &mut netns_id);
    if ret < 0 {
        free(handle as *mut c_void);
        close(sock_fd);
        close(netns_fd);
        if errno == ENOTTY || errno == EINVAL {
            SKIP!(return, "NS_GET_ID not supported");
        }
        ASSERT_EQ!(ret, 0);
    }

    /* Construct file handle from namespace ID */
    nsfs_fh = nsfs_fh_from_handle(handle);
    (*nsfs_fh).ns_id = netns_id;
    (*nsfs_fh).ns_type = 0;  /* Type field not needed for reopening */
    (*nsfs_fh).ns_inum = 0;  /* Inum field not needed for reopening */

    TH_LOG!("Constructed file handle for netns {} (id={})", netns_ino, netns_id);

    /* Reopen namespace using file handle (while socket still keeps it alive) */
    reopened_fd = open_by_handle_at(FD_NSFS_ROOT, handle, O_RDONLY);
    if reopened_fd < 0 {
        free(handle as *mut c_void);
        close(sock_fd);
        if errno == EOPNOTSUPP || errno == ENOSYS || errno == EBADF {
            SKIP!(return, "open_by_handle_at with FD_NSFS_ROOT not supported");
        }
        TH_LOG!("open_by_handle_at failed: {:?}", strerror(errno));
        ASSERT_GE!(reopened_fd, 0);
    }

    /* Verify it's the same namespace */
    ASSERT_EQ!(fstat(reopened_fd, &mut st2), 0);
    ASSERT_EQ!(st1.st_ino, st2.st_ino);
    ASSERT_EQ!(st1.st_dev, st2.st_dev);

    TH_LOG!("Successfully reopened netns {} via file handle", netns_ino);

    close(reopened_fd);

    /* Close the netns FD */
    close(netns_fd);

    /* Try to reopen via file handle - should fail since namespace is now inactive */
    reopened_fd = open_by_handle_at(FD_NSFS_ROOT, handle, O_RDONLY);
    ASSERT_LT!(reopened_fd, 0);
    TH_LOG!("Correctly failed to reopen inactive netns: {:?}", strerror(errno));

    /* Get network namespace from socket */
    netns_fd = ioctl(sock_fd, SIOCGSKNS);
    if netns_fd < 0 {
        free(handle as *mut c_void);
        close(sock_fd);
        if errno == ENOTTY || errno == EINVAL {
            SKIP!(return, "SIOCGSKNS not supported");
        }
        ASSERT_GE!(netns_fd, 0);
    }

    /* Reopen namespace using file handle (while socket still keeps it alive) */
    reopened_fd = open_by_handle_at(FD_NSFS_ROOT, handle, O_RDONLY);
    if reopened_fd < 0 {
        free(handle as *mut c_void);
        close(sock_fd);
        if errno == EOPNOTSUPP || errno == ENOSYS || errno == EBADF {
            SKIP!(return, "open_by_handle_at with FD_NSFS_ROOT not supported");
        }
        TH_LOG!("open_by_handle_at failed: {:?}", strerror(errno));
        ASSERT_GE!(reopened_fd, 0);
    }

    /* Verify it's the same namespace */
    ASSERT_EQ!(fstat(reopened_fd, &mut st2), 0);
    ASSERT_EQ!(st1.st_ino, st2.st_ino);
    ASSERT_EQ!(st1.st_dev, st2.st_dev);

    TH_LOG!("Successfully reopened netns {} via file handle", netns_ino);

    /* Close socket - namespace should become inactive */
    close(sock_fd);
    free(handle as *mut c_void);
});

/*
 * Test combined listns() and file handle operations with socket-kept netns.
 * Create a netns, keep it alive with a socket, verify it appears in listns(),
 * then reopen it via file handle obtained from listns() entry.
 */
TEST!(siocgskns_listns_and_file_handle, {
    let mut sock_fd: c_int = 0;
    let mut netns_fd: c_int;
    let userns_fd: c_int;
    let mut reopened_fd: c_int;
    let mut ipc_sockets = [0 as c_int; 2];
    let pid: pid_t;
    let mut status: c_int = 0;
    let mut st: stat = core::mem::zeroed();
    let netns_ino: ino_t;
    let mut netns_id: __u64 = 0;
    let mut userns_id: __u64 = 0;
    let mut req = ns_id_req {
        size: core::mem::size_of::<ns_id_req>() as c_uint,
        spare: 0,
        ns_id: 0,
        ns_type: (CLONE_NEWNET | CLONE_NEWUSER) as c_uint,
        spare2: 0,
        user_ns_id: 0,
    };
    let mut ns_ids = [0 as __u64; 256];
    let mut ret: c_int;
    let mut found_netns = false;
    let mut found_userns = false;
    let handle: *mut file_handle;
    let nsfs_fh: *mut nsfs_file_handle;

    /* Allocate file_handle structure for nsfs */
    handle = alloc_nsfs_handle();
    ASSERT_NE!(handle, core::ptr::null_mut());

    EXPECT_EQ!(socketpair(AF_LOCAL, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr()), 0);

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        /* Child: create new userns and netns with socket */
        close(ipc_sockets[0]);

        if setup_userns() < 0 {
            close(ipc_sockets[1]);
            exit(1);
        }

        if unshare(CLONE_NEWNET) < 0 {
            close(ipc_sockets[1]);
            exit(1);
        }

        sock_fd = socket(AF_INET, SOCK_DGRAM, 0);
        if sock_fd < 0 {
            close(ipc_sockets[1]);
            exit(1);
        }

        /* Send socket FD to parent via SCM_RIGHTS */
        if send_fd(ipc_sockets[1], sock_fd, b'X') < 0 {
            close(sock_fd);
            close(ipc_sockets[1]);
            exit(1);
        }

        close(sock_fd);
        close(ipc_sockets[1]);
        exit(0);
    }

    /* Parent: receive socket FD */
    close(ipc_sockets[1]);
    sock_fd = recv_fd(ipc_sockets[0], core::ptr::null_mut());
    close(ipc_sockets[0]);
    ASSERT_GE!(sock_fd, 0);

    /* Wait for child to exit */
    waitpid(pid, &mut status, 0);
    ASSERT_TRUE!(WIFEXITED(status));
    ASSERT_EQ!(WEXITSTATUS(status), 0);

    /* Get network namespace from socket */
    netns_fd = ioctl(sock_fd, SIOCGSKNS);
    if netns_fd < 0 {
        free(handle as *mut c_void);
        close(sock_fd);
        if errno == ENOTTY || errno == EINVAL {
            SKIP!(return, "SIOCGSKNS not supported");
        }
        ASSERT_GE!(netns_fd, 0);
    }

    ASSERT_EQ!(fstat(netns_fd, &mut st), 0);
    netns_ino = st.st_ino;

    /* Get namespace ID */
    ret = ioctl(netns_fd, NS_GET_ID, &mut netns_id);
    if ret < 0 {
        free(handle as *mut c_void);
        close(sock_fd);
        close(netns_fd);
        if errno == ENOTTY || errno == EINVAL {
            SKIP!(return, "NS_GET_ID not supported");
        }
        ASSERT_EQ!(ret, 0);
    }

    /* Get owner user namespace */
    userns_fd = ioctl(netns_fd, NS_GET_USERNS);
    if userns_fd < 0 {
        free(handle as *mut c_void);
        close(sock_fd);
        close(netns_fd);
        if errno == ENOTTY || errno == EINVAL {
            SKIP!(return, "NS_GET_USERNS not supported");
        }
        ASSERT_GE!(userns_fd, 0);
    }

    /* Get owner namespace ID */
    ret = ioctl(userns_fd, NS_GET_ID, &mut userns_id);
    if ret < 0 {
        close(userns_fd);
        free(handle as *mut c_void);
        close(sock_fd);
        close(netns_fd);
        ASSERT_EQ!(ret, 0);
    }
    close(userns_fd);

    TH_LOG!("Testing netns {} (id={}) owned by userns id={}", netns_ino, netns_id, userns_id);

    /* Verify namespace appears in listns() */
    ret = sys_listns(&mut req, ns_ids.as_mut_ptr(), ARRAY_SIZE(&ns_ids), 0);
    if ret < 0 {
        free(handle as *mut c_void);
        close(sock_fd);
        close(netns_fd);
        if errno == ENOSYS {
            SKIP!(return, "listns() not supported");
        }
        TH_LOG!("listns failed: {:?}", strerror(errno));
        ASSERT_GE!(ret, 0);
    }

    found_netns = false;
    found_userns = false;
    for i in 0..ret as usize {
        if ns_ids[i] == netns_id {
            found_netns = true;
        }
        if ns_ids[i] == userns_id {
            found_userns = true;
        }
    }
    ASSERT_TRUE!(found_netns);
    ASSERT_TRUE!(found_userns);
    TH_LOG!("Found netns {} in listns() output", netns_id);

    /* Construct file handle from namespace ID */
    nsfs_fh = nsfs_fh_from_handle(handle);
    (*nsfs_fh).ns_id = netns_id;
    (*nsfs_fh).ns_type = 0;
    (*nsfs_fh).ns_inum = 0;

    reopened_fd = open_by_handle_at(FD_NSFS_ROOT, handle, O_RDONLY);
    if reopened_fd < 0 {
        free(handle as *mut c_void);
        close(sock_fd);
        if errno == EOPNOTSUPP || errno == ENOSYS || errno == EBADF {
            SKIP!(return, "open_by_handle_at with FD_NSFS_ROOT not supported");
        }
        TH_LOG!("open_by_handle_at failed: {:?}", strerror(errno));
        ASSERT_GE!(reopened_fd, 0);
    }

    let mut reopened_st: stat = core::mem::zeroed();
    ASSERT_EQ!(fstat(reopened_fd, &mut reopened_st), 0);
    ASSERT_EQ!(reopened_st.st_ino, netns_ino);

    TH_LOG!("Successfully reopened netns {} via file handle (socket-kept)", netns_ino);

    close(reopened_fd);
    close(netns_fd);

    /* Try to reopen via file handle - should fail since namespace is now inactive */
    reopened_fd = open_by_handle_at(FD_NSFS_ROOT, handle, O_RDONLY);
    ASSERT_LT!(reopened_fd, 0);
    TH_LOG!("Correctly failed to reopen inactive netns: {:?}", strerror(errno));

    /* Get network namespace from socket */
    netns_fd = ioctl(sock_fd, SIOCGSKNS);
    if netns_fd < 0 {
        free(handle as *mut c_void);
        close(sock_fd);
        if errno == ENOTTY || errno == EINVAL {
            SKIP!(return, "SIOCGSKNS not supported");
        }
        ASSERT_GE!(netns_fd, 0);
    }

    /* Verify namespace appears in listns() */
    ret = sys_listns(&mut req, ns_ids.as_mut_ptr(), ARRAY_SIZE(&ns_ids), 0);
    if ret < 0 {
        free(handle as *mut c_void);
        close(sock_fd);
        close(netns_fd);
        if errno == ENOSYS {
            SKIP!(return, "listns() not supported");
        }
        TH_LOG!("listns failed: {:?}", strerror(errno));
        ASSERT_GE!(ret, 0);
    }

    found_netns = false;
    found_userns = false;
    for i in 0..ret as usize {
        if ns_ids[i] == netns_id {
            found_netns = true;
        }
        if ns_ids[i] == userns_id {
            found_userns = true;
        }
    }
    ASSERT_TRUE!(found_netns);
    ASSERT_TRUE!(found_userns);
    TH_LOG!("Found netns {} in listns() output", netns_id);

    close(netns_fd);

    /* Verify namespace appears in listns() */
    ret = sys_listns(&mut req, ns_ids.as_mut_ptr(), ARRAY_SIZE(&ns_ids), 0);
    if ret < 0 {
        free(handle as *mut c_void);
        close(sock_fd);
        close(netns_fd);
        if errno == ENOSYS {
            SKIP!(return, "listns() not supported");
        }
        TH_LOG!("listns failed: {:?}", strerror(errno));
        ASSERT_GE!(ret, 0);
    }

    found_netns = false;
    found_userns = false;
    for i in 0..ret as usize {
        if ns_ids[i] == netns_id {
            found_netns = true;
        }
        if ns_ids[i] == userns_id {
            found_userns = true;
        }
    }
    ASSERT_FALSE!(found_netns);
    ASSERT_FALSE!(found_userns);
    TH_LOG!("Netns {} correctly disappeared from listns() after socket closed", netns_id);

    close(sock_fd);
    free(handle as *mut c_void);
});

/*
 * Test multi-level namespace resurrection across three user namespace levels.
 *
 * This test creates a complex namespace hierarchy with three levels of user
 * namespaces and a network namespace at the deepest level. It verifies that
 * the resurrection semantics work correctly when SIOCGSKNS is called on a
 * socket from an inactive namespace tree, and that listns() and
 * open_by_handle_at() correctly respect visibility rules.
 *
 * Hierarchy after child processes exit (all with 0 active refcount):
 *
 *          net_L3A (0)                <- Level 3 network namespace
 *              |
 *              +
 *          userns_L3 (0)              <- Level 3 user namespace
 *              |
 *              +
 *          userns_L2 (0)              <- Level 2 user namespace
 *              |
 *              +
 *          userns_L1 (0)              <- Level 1 user namespace
 *              |
 *              x
 *          init_user_ns
 *
 * The test verifies:
 * 1. SIOCGSKNS on a socket from inactive net_L3A resurrects the entire chain
 * 2. After resurrection, all namespaces are visible in listns()
 * 3. Resurrected namespaces can be reopened via file handles
 * 4. Closing the netns FD cascades down: the entire ownership chain
 *    (userns_L3 -> userns_L2 -> userns_L1) becomes inactive again
 * 5. Inactive namespaces disappear from listns() and cannot be reopened
 * 6. Calling SIOCGSKNS again on the same socket resurrects the tree again
 * 7. After second resurrection, namespaces are visible and can be reopened
 */
TEST!(siocgskns_multilevel_resurrection, {
    let mut ipc_sockets = [0 as c_int; 2];
    let mut pid_l1: pid_t;
    let mut pid_l2: pid_t = 0;
    let mut pid_l3: pid_t = 0;
    let mut status: c_int = 0;

    /* Namespace file descriptors to be received from child */
    let mut sock_L3A_fd: c_int = -1;
    let mut netns_L3A_fd: c_int = -1;
    let mut netns_L3A_id: __u64 = 0;
    let mut userns_L1_id: __u64 = 0;
    let mut userns_L2_id: __u64 = 0;
    let mut userns_L3_id: __u64 = 0;

    /* For listns() and file handle testing */
    let mut req = ns_id_req {
        size: core::mem::size_of::<ns_id_req>() as c_uint,
        spare: 0,
        ns_id: 0,
        ns_type: (CLONE_NEWNET | CLONE_NEWUSER) as c_uint,
        spare2: 0,
        user_ns_id: 0,
    };
    let mut ns_ids = [0 as __u64; 256];
    let mut ret: c_int;
    let handle: *mut file_handle;
    let nsfs_fh: *mut nsfs_file_handle;
    let mut reopened_fd: c_int;

    /* Allocate file handle for testing */
    handle = alloc_nsfs_handle();
    ASSERT_NE!(handle, core::ptr::null_mut());

    EXPECT_EQ!(socketpair(AF_LOCAL, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr()), 0);

    /*
     * Fork level 1 child that creates userns_L1
     */
    pid_l1 = fork();
    ASSERT_GE!(pid_l1, 0);

    if pid_l1 == 0 {
        /* Level 1 child */
        let mut ipc_L2 = [0 as c_int; 2];
        close(ipc_sockets[0]);

        /* Create userns_L1 */
        if setup_userns() < 0 {
            close(ipc_sockets[1]);
            exit(1);
        }

        /* Create socketpair for communicating with L2 child */
        if socketpair(AF_LOCAL, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_L2.as_mut_ptr()) < 0 {
            close(ipc_sockets[1]);
            exit(1);
        }

        /*
         * Fork level 2 child that creates userns_L2
         */
        pid_l2 = fork();
        if pid_l2 < 0 {
            close(ipc_sockets[1]);
            close(ipc_L2[0]);
            close(ipc_L2[1]);
            exit(1);
        }

        if pid_l2 == 0 {
            /* Level 2 child */
            let mut ipc_L3 = [0 as c_int; 2];
            close(ipc_L2[0]);

            /* Create userns_L2 (nested inside userns_L1) */
            if setup_userns() < 0 {
                close(ipc_L2[1]);
                exit(1);
            }

            /* Create socketpair for communicating with L3 child */
            if socketpair(AF_LOCAL, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_L3.as_mut_ptr()) < 0 {
                close(ipc_L2[1]);
                exit(1);
            }

            /*
             * Fork level 3 child that creates userns_L3 and network namespaces
             */
            pid_l3 = fork();
            if pid_l3 < 0 {
                close(ipc_L2[1]);
                close(ipc_L3[0]);
                close(ipc_L3[1]);
                exit(1);
            }

            if pid_l3 == 0 {
                /* Level 3 child - the deepest level */
                let sock_fd: c_int;
                close(ipc_L3[0]);

                /* Create userns_L3 (nested inside userns_L2) */
                if setup_userns() < 0 {
                    close(ipc_L3[1]);
                    exit(1);
                }

                /* Create network namespace at level 3 */
                if unshare(CLONE_NEWNET) < 0 {
                    close(ipc_L3[1]);
                    exit(1);
                }

                /* Create socket in net_L3A */
                sock_fd = socket(AF_INET, SOCK_DGRAM, 0);
                if sock_fd < 0 {
                    close(ipc_L3[1]);
                    exit(1);
                }

                /* Send socket FD to L2 parent */
                if send_fd(ipc_L3[1], sock_fd, b'X') < 0 {
                    close(sock_fd);
                    close(ipc_L3[1]);
                    exit(1);
                }

                close(sock_fd);
                close(ipc_L3[1]);
                exit(0);
            }

            /* Level 2 child - receive from L3 and forward to L1 */
            close(ipc_L3[1]);

            let received_fd = recv_fd(ipc_L3[0], core::ptr::null_mut());
            close(ipc_L3[0]);

            if received_fd < 0 {
                close(ipc_L2[1]);
                waitpid(pid_l3, core::ptr::null_mut(), 0);
                exit(1);
            }

            /* Wait for L3 child */
            waitpid(pid_l3, core::ptr::null_mut(), 0);

            /* Forward the socket FD to L1 parent */
            if send_fd(ipc_L2[1], received_fd, b'Y') < 0 {
                close(received_fd);
                close(ipc_L2[1]);
                exit(1);
            }

            close(received_fd);
            close(ipc_L2[1]);
            exit(0);
        }

        /* Level 1 child - receive from L2 and forward to parent */
        close(ipc_L2[1]);

        let received_fd = recv_fd(ipc_L2[0], core::ptr::null_mut());
        close(ipc_L2[0]);

        if received_fd < 0 {
            close(ipc_sockets[1]);
            waitpid(pid_l2, core::ptr::null_mut(), 0);
            exit(1);
        }

        /* Wait for L2 child */
        waitpid(pid_l2, core::ptr::null_mut(), 0);

        /* Forward the socket FD to parent */
        if send_fd(ipc_sockets[1], received_fd, b'Z') < 0 {
            close(received_fd);
            close(ipc_sockets[1]);
            exit(1);
        }

        close(received_fd);
        close(ipc_sockets[1]);
        exit(0);
    }

    /* Parent - receive the socket from the deepest level */
    close(ipc_sockets[1]);

    sock_L3A_fd = recv_fd(ipc_sockets[0], core::ptr::null_mut());
    close(ipc_sockets[0]);

    if sock_L3A_fd < 0 {
        free(handle as *mut c_void);
        waitpid(pid_l1, core::ptr::null_mut(), 0);
        SKIP!(return, "Failed to receive socket from child");
    }

    /* Wait for L1 child */
    waitpid(pid_l1, &mut status, 0);
    ASSERT_TRUE!(WIFEXITED(status));
    ASSERT_EQ!(WEXITSTATUS(status), 0);

    /*
     * At this point, all child processes have exited. The socket itself
     * doesn't keep the namespace active - we need to call SIOCGSKNS which
     * will resurrect the entire namespace tree by taking active references.
     */

    /* Get network namespace from socket - this resurrects the tree */
    netns_L3A_fd = ioctl(sock_L3A_fd, SIOCGSKNS);
    if netns_L3A_fd < 0 {
        free(handle as *mut c_void);
        close(sock_L3A_fd);
        if errno == ENOTTY || errno == EINVAL {
            SKIP!(return, "SIOCGSKNS not supported");
        }
        ASSERT_GE!(netns_L3A_fd, 0);
    }

    /* Get namespace ID for net_L3A */
    ret = ioctl(netns_L3A_fd, NS_GET_ID, &mut netns_L3A_id);
    if ret < 0 {
        free(handle as *mut c_void);
        close(sock_L3A_fd);
        close(netns_L3A_fd);
        if errno == ENOTTY || errno == EINVAL {
            SKIP!(return, "NS_GET_ID not supported");
        }
        ASSERT_EQ!(ret, 0);
    }

    /* Get owner user namespace chain: userns_L3 -> userns_L2 -> userns_L1 */
    let userns_L3_fd = ioctl(netns_L3A_fd, NS_GET_USERNS);
    if userns_L3_fd < 0 {
        free(handle as *mut c_void);
        close(sock_L3A_fd);
        close(netns_L3A_fd);
        if errno == ENOTTY || errno == EINVAL {
            SKIP!(return, "NS_GET_USERNS not supported");
        }
        ASSERT_GE!(userns_L3_fd, 0);
    }

    ret = ioctl(userns_L3_fd, NS_GET_ID, &mut userns_L3_id);
    ASSERT_EQ!(ret, 0);

    let userns_L2_fd = ioctl(userns_L3_fd, NS_GET_USERNS);
    ASSERT_GE!(userns_L2_fd, 0);
    ret = ioctl(userns_L2_fd, NS_GET_ID, &mut userns_L2_id);
    ASSERT_EQ!(ret, 0);

    let userns_L1_fd = ioctl(userns_L2_fd, NS_GET_USERNS);
    ASSERT_GE!(userns_L1_fd, 0);
    ret = ioctl(userns_L1_fd, NS_GET_ID, &mut userns_L1_id);
    ASSERT_EQ!(ret, 0);

    close(userns_L1_fd);
    close(userns_L2_fd);
    close(userns_L3_fd);

    TH_LOG!("Multi-level hierarchy: net_L3A (id={}) -> userns_L3 (id={}) -> userns_L2 (id={}) -> userns_L1 (id={})",
       netns_L3A_id, userns_L3_id, userns_L2_id, userns_L1_id);

    /*
     * Test 1: Verify net_L3A is visible in listns() after resurrection.
     * The entire ownership chain should be resurrected and visible.
     */
    ret = sys_listns(&mut req, ns_ids.as_mut_ptr(), ARRAY_SIZE(&ns_ids), 0);
    if ret < 0 {
        free(handle as *mut c_void);
        close(sock_L3A_fd);
        close(netns_L3A_fd);
        if errno == ENOSYS {
            SKIP!(return, "listns() not supported");
        }
        ASSERT_GE!(ret, 0);
    }

    let mut found_netns_L3A = false;
    let mut found_userns_L1 = false;
    let mut found_userns_L2 = false;
    let mut found_userns_L3 = false;

    for i in 0..ret as usize {
        if ns_ids[i] == netns_L3A_id { found_netns_L3A = true; }
        if ns_ids[i] == userns_L1_id { found_userns_L1 = true; }
        if ns_ids[i] == userns_L2_id { found_userns_L2 = true; }
        if ns_ids[i] == userns_L3_id { found_userns_L3 = true; }
    }

    ASSERT_TRUE!(found_netns_L3A);
    ASSERT_TRUE!(found_userns_L1);
    ASSERT_TRUE!(found_userns_L2);
    ASSERT_TRUE!(found_userns_L3);
    TH_LOG!("Resurrection verified: all namespaces in hierarchy visible in listns()");

    /*
     * Test 2: Verify net_L3A can be reopened via file handle.
     */
    nsfs_fh = nsfs_fh_from_handle(handle);
    (*nsfs_fh).ns_id = netns_L3A_id;
    (*nsfs_fh).ns_type = 0;
    (*nsfs_fh).ns_inum = 0;

    reopened_fd = open_by_handle_at(FD_NSFS_ROOT, handle, O_RDONLY);
    if reopened_fd < 0 {
        free(handle as *mut c_void);
        close(sock_L3A_fd);
        close(netns_L3A_fd);
        if errno == EOPNOTSUPP || errno == ENOSYS || errno == EBADF {
            SKIP!(return, "open_by_handle_at with FD_NSFS_ROOT not supported");
        }
        TH_LOG!("open_by_handle_at failed: {:?}", strerror(errno));
        ASSERT_GE!(reopened_fd, 0);
    }

    close(reopened_fd);
    TH_LOG!("File handle test passed: net_L3A can be reopened");

    /*
     * Test 3: Verify that when we close the netns FD (dropping the last
     * active reference), the entire tree becomes inactive and disappears
     * from listns(). The cascade goes: net_L3A drops -> userns_L3 drops ->
     * userns_L2 drops -> userns_L1 drops.
     */
    close(netns_L3A_fd);

    ret = sys_listns(&mut req, ns_ids.as_mut_ptr(), ARRAY_SIZE(&ns_ids), 0);
    ASSERT_GE!(ret, 0);

    found_netns_L3A = false;
    found_userns_L1 = false;
    found_userns_L2 = false;
    found_userns_L3 = false;

    for i in 0..ret as usize {
        if ns_ids[i] == netns_L3A_id { found_netns_L3A = true; }
        if ns_ids[i] == userns_L1_id { found_userns_L1 = true; }
        if ns_ids[i] == userns_L2_id { found_userns_L2 = true; }
        if ns_ids[i] == userns_L3_id { found_userns_L3 = true; }
    }

    ASSERT_FALSE!(found_netns_L3A);
    ASSERT_FALSE!(found_userns_L1);
    ASSERT_FALSE!(found_userns_L2);
    ASSERT_FALSE!(found_userns_L3);
    TH_LOG!("Cascade test passed: all namespaces disappeared after netns FD closed");

    /*
     * Test 4: Verify file handle no longer works for inactive namespace.
     */
    reopened_fd = open_by_handle_at(FD_NSFS_ROOT, handle, O_RDONLY);
    if reopened_fd >= 0 {
        close(reopened_fd);
        free(handle as *mut c_void);
        ASSERT_TRUE!(false); /* Should have failed */
    }
    TH_LOG!("Inactive namespace correctly cannot be reopened via file handle");

    /*
     * Test 5: Verify that calling SIOCGSKNS again resurrects the tree again.
     * The socket is still valid, so we can call SIOCGSKNS on it to resurrect
     * the namespace tree once more.
     */
    netns_L3A_fd = ioctl(sock_L3A_fd, SIOCGSKNS);
    ASSERT_GE!(netns_L3A_fd, 0);

    TH_LOG!("Called SIOCGSKNS again to resurrect the namespace tree");

    /* Verify the namespace tree is resurrected and visible in listns() */
    ret = sys_listns(&mut req, ns_ids.as_mut_ptr(), ARRAY_SIZE(&ns_ids), 0);
    ASSERT_GE!(ret, 0);

    found_netns_L3A = false;
    found_userns_L1 = false;
    found_userns_L2 = false;
    found_userns_L3 = false;

    for i in 0..ret as usize {
        if ns_ids[i] == netns_L3A_id { found_netns_L3A = true; }
        if ns_ids[i] == userns_L1_id { found_userns_L1 = true; }
        if ns_ids[i] == userns_L2_id { found_userns_L2 = true; }
        if ns_ids[i] == userns_L3_id { found_userns_L3 = true; }
    }

    ASSERT_TRUE!(found_netns_L3A);
    ASSERT_TRUE!(found_userns_L1);
    ASSERT_TRUE!(found_userns_L2);
    ASSERT_TRUE!(found_userns_L3);
    TH_LOG!("Second resurrection verified: all namespaces in hierarchy visible in listns() again");

    /* Verify we can reopen via file handle again */
    reopened_fd = open_by_handle_at(FD_NSFS_ROOT, handle, O_RDONLY);
    if reopened_fd < 0 {
        free(handle as *mut c_void);
        close(sock_L3A_fd);
        close(netns_L3A_fd);
        TH_LOG!("open_by_handle_at failed after second resurrection: {:?}", strerror(errno));
        ASSERT_GE!(reopened_fd, 0);
    }

    close(reopened_fd);
    TH_LOG!("File handle test passed: net_L3A can be reopened after second resurrection");

    /* Final cleanup */
    close(sock_L3A_fd);
    close(netns_L3A_fd);
    free(handle as *mut c_void);
});

// TEST_HARNESS_MAIN
