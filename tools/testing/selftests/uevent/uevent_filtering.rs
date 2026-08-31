// SPDX-License-Identifier: GPL-2.0

use std::ffi::{c_char, c_void, CStr};
use std::mem;

#[repr(C)]
#[derive(Copy, Clone)]
struct SockAddrNl {
    nl_family: u16,
    nl_pad: u16,
    nl_pid: u32,
    nl_groups: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Iovec {
    iov_base: *mut c_void,
    iov_len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Msghdr {
    msg_name: *mut c_void,
    msg_namelen: u32,
    msg_iov: *mut Iovec,
    msg_iovlen: usize,
    msg_control: *mut c_void,
    msg_controllen: usize,
    msg_flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Cmsghdr {
    cmsg_len: usize,
    cmsg_level: i32,
    cmsg_type: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Ucred {
    pid: i32,
    uid: u32,
    gid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SigsetT {
    __val: [u64; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Sigaction {
    sa_handler: Option<extern "C" fn(i32)>,
    sa_mask: SigsetT,
    sa_flags: u32,
    sa_restorer: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Timespec {
    tv_sec: i64,
    tv_nsec: isize,
}

#[allow(non_upper_case_globals)]
const __DEV_FULL: &[u8] = b"/sys/devices/virtual/mem/full/uevent\0";
#[allow(non_upper_case_globals)]
const __UEVENT_BUFFER_SIZE: usize = 1024 * 1024;
#[allow(non_upper_case_globals)]
const __UEVENT_HEADER: &[u8] = b"add@/devices/virtual/mem/full\0";
#[allow(non_upper_case_globals)]
const __UEVENT_HEADER_LEN: usize = __UEVENT_HEADER.len();
#[allow(non_upper_case_globals)]
const __UEVENT_LISTEN_ALL: i32 = -1;

const O_RDWR: i32 = 2;
const O_CLOEXEC: i32 = 0o200000;
const F_OK: i32 = 0;
const F_SETSIG: i32 = 10;
const ENOENT: i32 = 2;
const EINVAL: i32 = 22;
const EINTR: i32 = 4;

const CLONE_NEWUSER: usize = 0x10000000;
const CLONE_NEWNET: usize = 0x40000000;

const AF_NETLINK: i32 = 16;
const SOCK_RAW: i32 = 3;
const SOCK_CLOEXEC: i32 = 0o2000000;
const NETLINK_KOBJECT_UEVENT: i32 = 15;

const SOL_SOCKET: i32 = 1;
const SO_RCVBUF: i32 = 8;

const SIG_BLOCK: i32 = 0;
const SIGCHLD: i32 = 17;
const SIGTERM: i32 = 15;
const SIGUSR1: i32 = 10;
const SIGKILL: i32 = 9;

const PR_SET_PDEATHSIG: i32 = 1;

const EFD_CLOEXEC: i32 = 0o2000000;

const EXIT_SUCCESS: i32 = 0;
const EXIT_FAILURE: i32 = 1;
const KSFT_SKIP: i32 = 4;
const KSFT_FAIL: i32 = 1;

const fn cmsg_align(len: usize) -> usize {
    let align = mem::align_of::<usize>();
    (len + align - 1) & !(align - 1)
}

const fn cmsg_space(len: usize) -> usize {
    cmsg_align(len) + cmsg_align(mem::size_of::<Cmsghdr>())
}

unsafe fn errno_val() -> i32 {
    *__errno_location()
}

unsafe fn errno_message() -> String {
    let e = errno_val();
    if e == 0 {
        return "(no error)".to_owned();
    }
    let msg = strerror(e);
    if msg.is_null() {
        return format!("errno {}", e);
    }
    CStr::from_ptr(msg).to_string_lossy().into_owned()
}

extern "C" {
    fn __errno_location() -> *mut i32;
    fn read(fd: i32, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: i32, buf: *const c_void, count: usize) -> isize;
    fn waitpid(pid: i32, status: *mut i32, options: i32) -> i32;
    fn open(path: *const c_char, flags: i32, mode: i32) -> i32;
    fn close(fd: i32) -> i32;
    fn access(path: *const c_char, mode: i32) -> i32;
    fn eventfd(initval: u64, flags: i32) -> i32;
    fn sigemptyset(set: *mut SigsetT) -> i32;
    fn sigaddset(set: *mut SigsetT, signo: i32) -> i32;
    fn sigprocmask(how: i32, set: *const SigsetT, oldset: *mut SigsetT) -> i32;
    fn sigtimedwait(set: *const SigsetT, info: *mut c_void, timeout: *const Timespec) -> i32;
    fn sigaction(signo: i32, act: *const Sigaction, oact: *mut Sigaction) -> i32;
    fn unshare(flags: usize) -> i32;
    fn socket(domain: i32, type_: i32, protocol: i32) -> i32;
    fn setsockopt(sockfd: i32, level: i32, optname: i32, optval: *const c_void, optlen: u32) -> i32;
    fn bind(sockfd: i32, addr: *const c_void, addrlen: u32) -> i32;
    fn getsockname(sockfd: i32, addr: *mut c_void, addrlen: *mut u32) -> i32;
    fn recvmsg(sockfd: i32, msg: *mut Msghdr, flags: i32) -> isize;
    fn getppid() -> i32;
    fn getpid() -> i32;
    fn kill(pid: i32, sig: i32) -> i32;
    fn prctl(option: i32, arg2: usize, arg3: usize, arg4: usize, arg5: usize) -> i32;
    fn _exit(status: i32) -> !;
    fn fork() -> i32;
    fn strerror(errnum: i32) -> *const c_char;
    fn geteuid() -> u32;
}

extern "C" {
    static TH_LOG_ENABLED: i32;
}

#[no_mangle]
unsafe extern "C" fn read_nointr(fd: i32, buf: *mut c_void, count: usize) -> isize {
    let mut ret: isize;

    loop {
        ret = read(fd, buf, count);
        if !(ret < 0 && errno_val() == EINTR) {
            return ret;
        }
    }
}

#[no_mangle]
unsafe extern "C" fn write_nointr(fd: i32, buf: *const c_void, count: usize) -> isize {
    let mut ret: isize;

    loop {
        ret = write(fd, buf, count);
        if !(ret < 0 && errno_val() == EINTR) {
            return ret;
        }
    }
}

unsafe fn wait_for_pid(pid: i32) -> i32 {
    let mut status: i32 = 0;
    loop {
        let ret = waitpid(pid, &mut status as *mut i32, 0);
        if ret == -1 {
            if errno_val() == EINTR {
                continue;
            }

            return -1;
        }

        if ret != pid {
            continue;
        }

        if (status & 0xff) != 0 || (status >> 8) != 0 {
            return -1;
        }

        return 0;
    }
}

unsafe fn uevent_listener(post_flags: usize, expect_uevent: bool, sync_fd: i32) -> i32 {
    let mut ret: i32 = 0;
    let mut sk_fd;
    let mut sk_addr_len: u32;
    let mut rcv_buf_sz: i32 = __UEVENT_BUFFER_SIZE as i32;
    let sync_add: u64 = 1;

    let mut sk_addr = SockAddrNl {
        nl_family: 0,
        nl_pad: 0,
        nl_pid: 0,
        nl_groups: 0,
    };
    let mut rcv_addr = SockAddrNl {
        nl_family: 0,
        nl_pad: 0,
        nl_pid: 0,
        nl_groups: 0,
    };
    let mut buf = [0u8; __UEVENT_BUFFER_SIZE];
    let mut iov = Iovec {
        iov_base: buf.as_mut_ptr() as *mut c_void,
        iov_len: __UEVENT_BUFFER_SIZE,
    };
    let mut control = [0u8; cmsg_space(mem::size_of::<Ucred>())];
    let mut hdr = Msghdr {
        msg_name: &mut rcv_addr as *mut SockAddrNl as *mut c_void,
        msg_namelen: mem::size_of::<SockAddrNl>() as u32,
        msg_iov: &mut iov,
        msg_iovlen: 1,
        msg_control: control.as_mut_ptr() as *mut c_void,
        msg_controllen: control.len(),
        msg_flags: 0,
    };

    sk_fd = socket(AF_NETLINK, SOCK_RAW | SOCK_CLOEXEC, NETLINK_KOBJECT_UEVENT);
    if sk_fd < 0 {
        eprintln!("{} - Failed to open uevent socket", errno_message());
        return -1;
    }

    ret = setsockopt(
        sk_fd,
        SOL_SOCKET,
        SO_RCVBUF,
        &rcv_buf_sz as *const i32 as *const c_void,
        mem::size_of::<i32>() as u32,
    );
    if ret < 0 {
        eprintln!("{} - Failed to set socket options", errno_message());
        close(sk_fd);
        return ret;
    }

    sk_addr.nl_family = AF_NETLINK as u16;
    sk_addr.nl_groups = __UEVENT_LISTEN_ALL as u32;

    sk_addr_len = mem::size_of::<SockAddrNl>() as u32;
    ret = bind(sk_fd, &sk_addr as *const SockAddrNl as *const c_void, sk_addr_len);
    if ret < 0 {
        eprintln!("{} - Failed to bind socket", errno_message());
        close(sk_fd);
        return ret;
    }

    ret = getsockname(
        sk_fd,
        &mut sk_addr as *mut SockAddrNl as *mut c_void,
        &mut sk_addr_len as *mut u32,
    );
    if ret < 0 {
        eprintln!("{} - Failed to retrieve socket name", errno_message());
        close(sk_fd);
        return ret;
    }

    if sk_addr_len as usize != mem::size_of::<SockAddrNl>() {
        eprintln!("Invalid socket address size");
        close(sk_fd);
        return -1;
    }

    if (post_flags & CLONE_NEWUSER) != 0 {
        ret = unshare(CLONE_NEWUSER);
        if ret < 0 {
            eprintln!("{} - Failed to unshare user namespace", errno_message());
            close(sk_fd);
            return ret;
        }
    }

    if (post_flags & CLONE_NEWNET) != 0 {
        ret = unshare(CLONE_NEWNET);
        if ret < 0 {
            eprintln!("{} - Failed to unshare network namespace", errno_message());
            close(sk_fd);
            return ret;
        }
    }

    let written = write_nointr(
        sync_fd,
        &sync_add as *const u64 as *const c_void,
        mem::size_of::<u64>(),
    );
    close(sync_fd);

    if written as isize != mem::size_of::<u64>() as isize {
        eprintln!("Failed to synchronize with parent process");
        close(sk_fd);
        return -1;
    }

    ret = 0;
    loop {
        let r = recvmsg(sk_fd, &mut hdr as *mut Msghdr, 0);
        if r <= 0 {
            eprintln!("{} - Failed to receive uevent", errno_message());
            ret = -1;
            break;
        }

        /* ignore libudev messages */
        if r >= 8 && &buf[..8] == b"libudev" {
            continue;
        }

        /* ignore uevents we didn't trigger */
        if (r as usize) < __UEVENT_HEADER_LEN {
            continue;
        }

        if &buf[..__UEVENT_HEADER_LEN] != __UEVENT_HEADER {
            continue;
        }

        if !expect_uevent {
            eprintln!("Received unexpected uevent:");
            ret = -1;
        }

        if TH_LOG_ENABLED != 0 {
            let _ = write_nointr(2, buf.as_ptr() as *const c_void, r as usize);
            let _ = write_nointr(2, "\n".as_ptr() as *const c_void, 1);
        }

        break;
    }

    close(sk_fd);
    ret
}

unsafe fn trigger_uevent(times: u32) -> i32 {
    let fd: i32;
    let mut ret: i32 = 0;

    fd = open(__DEV_FULL.as_ptr() as *const c_char, O_RDWR | O_CLOEXEC, 0);
    if fd < 0 {
        if errno_val() != ENOENT {
            return -EINVAL;
        }

        return -1;
    }

    let mut i = 0u32;
    while i < times {
        ret = write_nointr(fd, b"add\n".as_ptr() as *const c_void, 3) as i32;
        if ret < 0 {
            eprintln!("Failed to trigger uevent");
            break;
        }
        i += 1;
    }

    close(fd);
    ret
}

unsafe fn set_death_signal() -> i32 {
    let mut ret = prctl(PR_SET_PDEATHSIG, SIGKILL as usize, 0, 0, 0);
    let ppid = getppid();

    if ppid == 1 {
        let self_pid = getpid();
        ret = kill(self_pid, SIGKILL);
    }

    if ret < 0 {
        return -1;
    }

    0
}

unsafe fn do_test(pre_flags: usize, post_flags: usize, expect_uevent: bool, sync_fd: i32) -> i32 {
    let mut ret: i32;
    let mut wait_val: u64 = 0;

    let pid: i32;
    let mut mask = SigsetT { __val: [0u64; 16] };
    let mut orig_mask = SigsetT { __val: [0u64; 16] };
    let timeout = Timespec {
        tv_sec: 2,
        tv_nsec: 0,
    };

    sigemptyset(&mut mask as *mut SigsetT);
    sigaddset(&mut mask as *mut SigsetT, SIGCHLD);

    ret = sigprocmask(SIG_BLOCK, &mask as *const SigsetT, &mut orig_mask as *mut SigsetT);
    if ret < 0 {
        eprintln!("{}- Failed to block SIGCHLD", errno_message());
        return -1;
    }

    pid = fork();
    if pid < 0 {
        eprintln!("{} - Failed to fork() new process", errno_message());
        return -1;
    }

    if pid == 0 {
        ret = set_death_signal();
        if ret < 0 {
            eprintln!("Failed to set PR_SET_PDEATHSIG to SIGKILL");
            _exit(EXIT_FAILURE);
        }

        if (pre_flags & CLONE_NEWUSER) != 0 {
            ret = unshare(CLONE_NEWUSER);
            if ret < 0 {
                eprintln!("{} - Failed to unshare user namespace", errno_message());
                _exit(EXIT_FAILURE);
            }
        }

        if (pre_flags & CLONE_NEWNET) != 0 {
            ret = unshare(CLONE_NEWNET);
            if ret < 0 {
                eprintln!("{} - Failed to unshare network namespace", errno_message());
                _exit(EXIT_FAILURE);
            }
        }

        if uevent_listener(post_flags, expect_uevent, sync_fd) < 0 {
            _exit(EXIT_FAILURE);
        }

        _exit(EXIT_SUCCESS);
    }

    ret = read_nointr(sync_fd, &mut wait_val as *mut u64 as *mut c_void, mem::size_of::<u64>()) as i32;
    if ret != mem::size_of::<u64>() as i32 {
        eprintln!("Failed to synchronize with child process");
        _exit(EXIT_FAILURE);
    }

    ret = trigger_uevent(10);
    if ret < 0 {
        eprintln!("Failed triggering uevents");
    }

    let mut timed = timeout;
    loop {
        let r = sigtimedwait(&mask as *const SigsetT, std::ptr::null_mut(), &timed as *const Timespec);
        if r < 0 {
            if errno_val() == EINTR {
                continue;
            }

            let kill_sig = if !expect_uevent { SIGTERM } else { SIGUSR1 };
            ret = kill(pid, kill_sig);
            if ret < 0 {
                return -1;
            }
        }

        break;
    }

    ret = wait_for_pid(pid);
    if ret < 0 {
        return -1;
    }

    ret
}

unsafe extern "C" fn signal_handler(sig: i32) {
    if sig == SIGTERM {
        _exit(EXIT_SUCCESS);
    }

    _exit(EXIT_FAILURE);
}

unsafe extern "C" fn uevent_filtering() {
    let mut ret: i32;
    let mut sync_fd: i32;

    let mut act = Sigaction {
        sa_handler: Some(signal_handler),
        sa_mask: SigsetT { __val: [0u64; 16] },
        sa_flags: 0,
        sa_restorer: std::ptr::null_mut(),
    };

    if geteuid() != 0 {
        if TH_LOG_ENABLED != 0 {
            eprintln!("Uevent filtering tests require root privileges. Skipping test");
        }
        _exit(KSFT_SKIP);
    }

    ret = access(__DEV_FULL.as_ptr() as *const c_char, F_OK);
    if ret != 0 {
        if errno_val() == ENOENT {
            if TH_LOG_ENABLED != 0 {
                eprintln!("/sys/devices/virtual/mem/full/uevent does not exist. Skipping test");
            }
            _exit(KSFT_SKIP);
        }
        _exit(KSFT_FAIL);
    }

    ret = sigaction(SIGTERM, &act as *const Sigaction, std::ptr::null_mut());
    if ret != 0 {
        _exit(KSFT_FAIL);
    }

    sync_fd = eventfd(0, EFD_CLOEXEC);
    if sync_fd < 0 {
        _exit(KSFT_FAIL);
    }

    /*
     * Setup:
     * - Open uevent listening socket in initial network namespace owned by
     *   initial user namespace.
     * - Trigger uevent in initial network namespace owned by initial user
     *   namespace.
     * Expected Result:
     * - uevent listening socket receives uevent
     */
    ret = do_test(0, 0, true, sync_fd);
    if ret != 0 {
        close(sync_fd);
        return;
    }

    /*
     * Setup:
     * - Open uevent listening socket in non-initial network namespace
     *   owned by initial user namespace.
     * - Trigger uevent in initial network namespace owned by initial user
     *   namespace.
     * Expected Result:
     * - uevent listening socket receives uevent
     */
    ret = do_test(CLONE_NEWNET, 0, true, sync_fd);
    if ret != 0 {
        close(sync_fd);
        return;
    }

    /*
     * Setup:
     * - unshare user namespace
     * - Open uevent listening socket in initial network namespace
     *   owned by initial user namespace.
     * - Trigger uevent in initial network namespace owned by initial user
     *   namespace.
     * Expected Result:
     * - uevent listening socket receives uevent
     */
    ret = do_test(CLONE_NEWUSER, 0, true, sync_fd);
    if ret != 0 {
        close(sync_fd);
        return;
    }

    /*
     * Setup:
     * - Open uevent listening socket in non-initial network namespace
     *   owned by non-initial user namespace.
     * - Trigger uevent in initial network namespace owned by initial user
     *   namespace.
     * Expected Result:
     * - uevent listening socket receives no uevent
     */
    ret = do_test(CLONE_NEWUSER | CLONE_NEWNET, 0, false, sync_fd);
    if ret != 0 {
        close(sync_fd);
        return;
    }

    /*
     * Setup:
     * - Open uevent listening socket in initial network namespace
     *   owned by initial user namespace.
     * - unshare network namespace
     * - Trigger uevent in initial network namespace owned by initial user
     *   namespace.
     * Expected Result:
     * - uevent listening socket receives uevent
     */
    ret = do_test(0, CLONE_NEWNET, true, sync_fd);
    if ret != 0 {
        close(sync_fd);
        return;
    }

    /*
     * Setup:
     * - Open uevent listening socket in initial network namespace
     *   owned by initial user namespace.
     * - unshare user namespace
     * - Trigger uevent in initial network namespace owned by initial user
     *   namespace.
     * Expected Result:
     * - uevent listening socket receives uevent
     */
    ret = do_test(0, CLONE_NEWUSER, true, sync_fd);
    if ret != 0 {
        close(sync_fd);
        return;
    }

    /*
     * Setup:
     * - Open uevent listening socket in initial network namespace
     *   owned by initial user namespace.
     * - unshare user namespace
     * - unshare network namespace
     * - Trigger uevent in initial network namespace owned by initial user
     *   namespace.
     * Expected Result:
     * - uevent listening socket receives uevent
     */
    ret = do_test(0, CLONE_NEWUSER | CLONE_NEWNET, true, sync_fd);
    if ret != 0 {
        close(sync_fd);
        return;
    }

    close(sync_fd);
}

fn main() {
    unsafe {
        uevent_filtering();
    }
}
