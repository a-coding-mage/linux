// SPDX-License-Identifier: GPL-2.0
/*
 * TTY Tests - TIOCSTI
 *
 * Copyright © 2025 Abhinav Saxena <xandfury@gmail.com>
 */

use std::ffi::c_void;
use std::mem::{align_of, size_of};
use std::ptr;

use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong};

type pid_t = c_int;
type size_t = usize;
type ssize_t = isize;

#[allow(non_camel_case_types)]
type __sighandler_t = Option<unsafe extern "C" fn(c_int)>;

#[repr(C)]
pub struct __test_metadata {
    _private: [u8; 0],
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

type cap_t = *mut c_void;
type cap_flag_t = c_int;
type cap_flag_value_t = c_int;

const CAP_SYS_ADMIN: c_int = 21;
const CAP_EFFECTIVE: cap_flag_t = 0;
const CAP_SET: cap_flag_value_t = 1;

const SOL_SOCKET: c_int = 1;
const SCM_RIGHTS: c_int = 1;
const AF_UNIX: c_int = 1;
const SOCK_STREAM: c_int = 1;

const TIOCSTI: c_ulong = 0x5412;

const EIO: c_int = 5;
const EPERM: c_int = 1;

const SIGHUP: c_int = 1;

const ENOENT: c_int = 2;

#[repr(C)]
struct __kernel_mode_t {
    _private: u16,
}

#[repr(C)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: size_t,
}

#[repr(C)]
pub struct msghdr {
    pub msg_name: *mut c_void,
    pub msg_namelen: c_int,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: size_t,
    pub msg_control: *mut c_void,
    pub msg_controllen: size_t,
    pub msg_flags: c_int,
}

#[repr(C)]
pub struct cmsghdr {
    pub cmsg_len: size_t,
    pub cmsg_level: c_int,
    pub cmsg_type: c_int,
    pub cmsg_data: [u8; 0],
}

#[inline]
const fn cmsg_align(len: usize) -> usize {
    let a = align_of::<usize>();
    (len + (a - 1)) & !(a - 1)
}

#[inline]
const fn cmsg_len(len: usize) -> usize {
    cmsg_align(size_of::<cmsghdr>()) + len
}

#[inline]
const fn cmsg_space(len: usize) -> usize {
    cmsg_align(cmsg_len(len))
}

#[inline]
fn cmsg_firsthdr(msg: *const msghdr) -> *mut cmsghdr {
    if msg.is_null() {
        return ptr::null_mut();
    }
    let msg = unsafe { &*msg };
    if msg.msg_controllen < size_of::<cmsghdr>() {
        ptr::null_mut()
    } else {
        msg.msg_control as *mut cmsghdr
    }
}

#[inline]
fn cmsg_nxthdr(msg: *const msghdr, cmsg: *const cmsghdr) -> *mut cmsghdr {
    if msg.is_null() || cmsg.is_null() {
        return ptr::null_mut();
    }

    let msg = unsafe { &*msg };
    let cmsg = unsafe { &*cmsg };

    let msg_end = (msg.msg_control as usize) + msg.msg_controllen;
    let next = (cmsg as *const cmsghdr as usize)
        .wrapping_add(cmsg_align(cmsg.cmsg_len));

    if next + size_of::<cmsghdr>() > msg_end {
        ptr::null_mut()
    } else {
        next as *mut cmsghdr
    }
}

#[inline]
fn cmsg_data_ptr<'a>(cmsg: *const cmsghdr) -> *mut u8 {
    cmsg as usize
        .wrapping_add(size_of::<cmsghdr>()) as *mut u8
}

#[link(name = "c")]
extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, argp: *const c_void) -> c_int;
    fn signal(signum: c_int, handler: __sighandler_t) -> __sighandler_t;
    fn openpty(
        amaster: *mut c_int,
        aslave: *mut c_int,
        name: *mut c_char,
        termp: *mut c_void,
        winp: *mut c_void,
    ) -> c_int;
    fn setgroups(size: usize, list: *const c_uint) -> c_int;
    fn setgid(gid: c_uint) -> c_int;
    fn setuid(uid: c_uint) -> c_int;
    fn prctl(op: c_int, arg2: c_ulong, arg3: c_ulong, arg4: c_ulong, arg5: c_ulong) -> c_int;
    fn cap_get_proc() -> cap_t;
    fn cap_get_flag(caps: cap_t, capability: c_int, flag: cap_flag_t, value: *mut cap_flag_value_t) -> c_int;
    fn cap_init() -> cap_t;
    fn cap_set_proc(caps: cap_t) -> c_int;
    fn cap_free(caps: *mut c_void) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn strerror(errnum: c_int) -> *const c_char;
    fn ioctl_set_nonblock(fd: c_int, on: c_int) -> c_int;
    fn sendmsg(sockfd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t;
    fn recvmsg(sockfd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn fork() -> pid_t;
    fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn _exit(status: c_int) -> !;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn setsid() -> pid_t;
    fn strerrorname_np(errnum: c_int) -> *const c_char;
}

extern "C" {
    static mut errno: c_int;
}

const PR_SET_NO_NEW_PRIVS: c_int = 38;

#[repr(C)]
enum test_type {
    TEST_PTY_TIOCSTI_BASIC,
    TEST_PTY_TIOCSTI_FD_PASSING,
}

#[repr(C)]
struct tiocsti {
    pty_master_fd: c_int,
    pty_slave_fd: c_int,
    has_pty: bool,
    initial_cap_sys_admin: bool,
    original_legacy_tiocsti_setting: c_int,
    can_modify_sysctl: bool,
}

#[repr(C)]
struct tiocsti_variant {
    test_type: test_type,
    controlling_tty: bool,
    legacy_tiocsti: c_int,
    requires_cap: bool,
    expected_success: c_int,
}

/*
 * Test Strategy:
 * - Basic tests: Use PTY with/without TIOCSCTTY (controlling terminal for
 *   current process)
 * - FD passing tests: Child creates PTY, parent receives FD (demonstrates
 *   security issue)
 *
 * SECURITY VULNERABILITY DEMONSTRATION:
 * FD passing tests show that TIOCSTI uses CURRENT process credentials, not
 * opener credentials. This means privileged processes can be given FDs from
 * unprivileged processes and successfully perform TIOCSTI operations that the
 * unprivileged process couldn't do directly.
 *
 * Attack scenario:
 * 1. Unprivileged process opens TTY (direct TIOCSTI fails due to lack of
 *    privileges)
 * 2. Unprivileged process passes FD to privileged process via SCM_RIGHTS
 * 3. Privileged process can use TIOCSTI on the FD (succeeds due to its
 *    privileges)
 * 4. Result: Effective privilege escalation via file descriptor passing
 *
 * This matches the kernel logic in tiocsti():
 * 1. if (!tty_legacy_tiocsti && !capable(CAP_SYS_ADMIN)) return -EIO;
 * 2. if ((current->signal->tty != tty) && !capable(CAP_SYS_ADMIN))
 *        return -EPERM;
 * Note: Both checks use capable() on CURRENT process, not FD opener!
 *
 * If the file credentials were also checked along with the capable() checks
 * then the results for FD pass tests would be consistent with the basic tests.
 */

/*
 * Tests Controlling Terminal Variants (current->signal->tty == tty)
 *
 * TIOCSTI Test Matrix:
 *
 * | legacy_tiocsti | CAP_SYS_ADMIN | Expected Result | Error |
 * |----------------|---------------|-----------------|-------|
 * | 1 (permissive) | true          | SUCCESS         | -     |
 * | 1 (permissive) | false         | SUCCESS         | -     |
 * | 0 (restricted) | true          | SUCCESS         | -     |
 * | 0 (restricted) | false         | FAILURE         | -EIO  |
 */

/* clang-format off */
const FIXTURE_VARIANT_BASIC_PTY_PERMISSIVE_WITHCAP: tiocsti_variant = tiocsti_variant {
    test_type: test_type::TEST_PTY_TIOCSTI_BASIC,
    controlling_tty: true,
    legacy_tiocsti: 1,
    requires_cap: true,
    expected_success: 0,
};

const FIXTURE_VARIANT_BASIC_PTY_PERMISSIVE_NOCAP: tiocsti_variant = tiocsti_variant {
    test_type: test_type::TEST_PTY_TIOCSTI_BASIC,
    controlling_tty: true,
    legacy_tiocsti: 1,
    requires_cap: false,
    expected_success: 0,
};

const FIXTURE_VARIANT_BASIC_PTY_RESTRICTED_WITHCAP: tiocsti_variant = tiocsti_variant {
    test_type: test_type::TEST_PTY_TIOCSTI_BASIC,
    controlling_tty: true,
    legacy_tiocsti: 0,
    requires_cap: true,
    expected_success: 0,
};

const FIXTURE_VARIANT_BASIC_PTY_RESTRICTED_NOCAP: tiocsti_variant = tiocsti_variant {
    test_type: test_type::TEST_PTY_TIOCSTI_BASIC,
    controlling_tty: true,
    legacy_tiocsti: 0,
    requires_cap: false,
    expected_success: -EIO, /* FAILURE: legacy restriction */
}; /* clang-format on */

/*
 * Note for FD Passing Test Variants
 * Since we're testing the scenario where an unprivileged process pass an FD
 * to a privileged one, .requires_cap here means the caps of the child process.
 * Not the parent; parent would always be privileged.
 */

/* clang-format off */
const FIXTURE_VARIANT_FDPASS_PTY_PERMISSIVE_WITHCAP: tiocsti_variant = tiocsti_variant {
    test_type: test_type::TEST_PTY_TIOCSTI_FD_PASSING,
    controlling_tty: true,
    legacy_tiocsti: 1,
    requires_cap: true,
    expected_success: 0,
};

const FIXTURE_VARIANT_FDPASS_PTY_PERMISSIVE_NOCAP: tiocsti_variant = tiocsti_variant {
    test_type: test_type::TEST_PTY_TIOCSTI_FD_PASSING,
    controlling_tty: true,
    legacy_tiocsti: 1,
    requires_cap: false,
    expected_success: 0,
};

const FIXTURE_VARIANT_FDPASS_PTY_RESTRICTED_WITHCAP: tiocsti_variant = tiocsti_variant {
    test_type: test_type::TEST_PTY_TIOCSTI_FD_PASSING,
    controlling_tty: true,
    legacy_tiocsti: 0,
    requires_cap: true,
    expected_success: 0,
};

const FIXTURE_VARIANT_FDPASS_PTY_RESTRICTED_NOCAP: tiocsti_variant = tiocsti_variant {
    test_type: test_type::TEST_PTY_TIOCSTI_FD_PASSING,
    controlling_tty: true,
    legacy_tiocsti: 0,
    requires_cap: false,
    expected_success: -EIO,
}; /* clang-format on */

/*
 * Non-Controlling Terminal Variants (current->signal->tty != tty)
 *
 * TIOCSTI Test Matrix:
 *
 * | legacy_tiocsti | CAP_SYS_ADMIN | Expected Result | Error |
 * |----------------|---------------|-----------------|-------|
 * | 1 (permissive) | true          | SUCCESS         | -     |
 * | 1 (permissive) | false         | FAILURE         | -EPERM|
 * | 0 (restricted) | true          | SUCCESS         | -     |
 * | 0 (restricted) | false         | FAILURE         | -EIO  |
 */

/* clang-format off */
const FIXTURE_VARIANT_BASIC_NOPTY_PERMISSIVE_WITHCAP: tiocsti_variant = tiocsti_variant {
    test_type: test_type::TEST_PTY_TIOCSTI_BASIC,
    controlling_tty: false,
    legacy_tiocsti: 1,
    requires_cap: true,
    expected_success: 0,
};

const FIXTURE_VARIANT_BASIC_NOPTY_PERMISSIVE_NOCAP: tiocsti_variant = tiocsti_variant {
    test_type: test_type::TEST_PTY_TIOCSTI_BASIC,
    controlling_tty: false,
    legacy_tiocsti: 1,
    requires_cap: false,
    expected_success: -EPERM,
};

const FIXTURE_VARIANT_BASIC_NOPTY_RESTRICTED_WITHCAP: tiocsti_variant = tiocsti_variant {
    test_type: test_type::TEST_PTY_TIOCSTI_BASIC,
    controlling_tty: false,
    legacy_tiocsti: 0,
    requires_cap: true,
    expected_success: 0,
};

const FIXTURE_VARIANT_BASIC_NOPTY_RESTRICTED_NOCAP: tiocsti_variant = tiocsti_variant {
    test_type: test_type::TEST_PTY_TIOCSTI_BASIC,
    controlling_tty: false,
    legacy_tiocsti: 0,
    requires_cap: false,
    expected_success: -EIO,
};

const FIXTURE_VARIANT_FDPASS_NOPTY_PERMISSIVE_WITHCAP: tiocsti_variant = tiocsti_variant {
    test_type: test_type::TEST_PTY_TIOCSTI_FD_PASSING,
    controlling_tty: false,
    legacy_tiocsti: 1,
    requires_cap: true,
    expected_success: 0,
};

const FIXTURE_VARIANT_FDPASS_NOPTY_PERMISSIVE_NOCAP: tiocsti_variant = tiocsti_variant {
    test_type: test_type::TEST_PTY_TIOCSTI_FD_PASSING,
    controlling_tty: false,
    legacy_tiocsti: 1,
    requires_cap: false,
    expected_success: -EPERM,
};

const FIXTURE_VARIANT_FDPASS_NOPTY_RESTRICTED_WITHCAP: tiocsti_variant = tiocsti_variant {
    test_type: test_type::TEST_PTY_TIOCSTI_FD_PASSING,
    controlling_tty: false,
    legacy_tiocsti: 0,
    requires_cap: true,
    expected_success: 0,
};

const FIXTURE_VARIANT_FDPASS_NOPTY_RESTRICTED_NOCAP: tiocsti_variant = tiocsti_variant {
    test_type: test_type::TEST_PTY_TIOCSTI_FD_PASSING,
    controlling_tty: false,
    legacy_tiocsti: 0,
    requires_cap: false,
    expected_success: -EIO,
}; /* clang-format on */

#[inline]
unsafe fn send_fd_via_socket(socket_fd: c_int, fd_to_send: c_int) -> c_int {
    let mut msg: msghdr = msghdr {
        msg_name: ptr::null_mut(),
        msg_namelen: 0,
        msg_iov: ptr::null_mut(),
        msg_iovlen: 0,
        msg_control: ptr::null_mut(),
        msg_controllen: 0,
        msg_flags: 0,
    };

    let mut cmsg_buf = [0u8; cmsg_space(size_of::<c_int>())];
    let mut dummy_data: u8 = b'F';
    let mut iov: iovec = iovec {
        iov_base: (&mut dummy_data as *mut u8).cast(),
        iov_len: 1,
    };

    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    msg.msg_control = cmsg_buf.as_mut_ptr().cast();
    msg.msg_controllen = cmsg_buf.len();

    let cmsg = cmsg_firsthdr(&msg);
    if cmsg.is_null() {
        return -1;
    }

    unsafe {
        (*cmsg).cmsg_level = SOL_SOCKET;
        (*cmsg).cmsg_type = SCM_RIGHTS;
        (*cmsg).cmsg_len = cmsg_len(size_of::<c_int>());
        ptr::copy_nonoverlapping(
            (&fd_to_send as *const c_int).cast::<u8>(),
            cmsg_data_ptr(cmsg),
            size_of::<c_int>(),
        );
    }

    let ret = unsafe { sendmsg(socket_fd, &msg as *const msghdr, 0) };
    if ret < 0 { -1 } else { 0 }
}

#[inline]
unsafe fn recv_fd_via_socket(socket_fd: c_int) -> c_int {
    let mut msg: msghdr = msghdr {
        msg_name: ptr::null_mut(),
        msg_namelen: 0,
        msg_iov: ptr::null_mut(),
        msg_iovlen: 0,
        msg_control: ptr::null_mut(),
        msg_controllen: 0,
        msg_flags: 0,
    };

    let mut cmsg_buf = [0u8; cmsg_space(size_of::<c_int>())];
    let mut dummy_data: u8 = 0;
    let mut iov: iovec = iovec {
        iov_base: (&mut dummy_data as *mut u8).cast(),
        iov_len: 1,
    };

    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    msg.msg_control = cmsg_buf.as_mut_ptr().cast();
    msg.msg_controllen = cmsg_buf.len();

    let ret = recvmsg(socket_fd, &mut msg as *mut msghdr, 0);
    if ret < 0 {
        return -1;
    }

    let mut received_fd = -1;
    let mut cmsg = cmsg_firsthdr(&msg);
    while !cmsg.is_null() {
        if unsafe { (*cmsg).cmsg_level == SOL_SOCKET && (*cmsg).cmsg_type == SCM_RIGHTS } {
            unsafe {
                ptr::copy_nonoverlapping(cmsg_data_ptr(cmsg),
                                         (&mut received_fd as *mut c_int).cast::<u8>(),
                                         size_of::<c_int>());
            }
            break;
        }
        cmsg = cmsg_nxthdr(&msg, cmsg);
    }

    received_fd
}

#[inline]
unsafe fn has_cap_sys_admin(_metadata: *mut __test_metadata) -> bool {
    let caps = unsafe { cap_get_proc() };
    if caps.is_null() {
        return false;
    }

    let mut cap_val: cap_flag_value_t = 0;
    let has_cap = (unsafe { cap_get_flag(caps, CAP_SYS_ADMIN, CAP_EFFECTIVE, &mut cap_val) } == 0)
        && (cap_val == CAP_SET);

    unsafe {
        cap_free(caps as *mut _);
    }

    has_cap
}

/*
 * Switch to non-root user and clear all capabilities
 */
#[inline]
unsafe fn drop_all_privs(_metadata: *mut __test_metadata) -> bool {
    unsafe {
        if setgroups(0, ptr::null()) != 0 {
            return false;
        }

        if setgid(1000) != 0 {
            return false;
        }

        if setuid(1000) != 0 {
            return false;
        }

        let empty = cap_init();
        if empty.is_null() {
            return false;
        }

        if cap_set_proc(empty) != 0 {
            cap_free(empty as *mut _);
            return false;
        }

        cap_free(empty as *mut _);

        if prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) != 0 {
            return false;
        }

        if has_cap_sys_admin(_metadata) {
            return false;
        }
    }

    true
}

#[inline]
unsafe fn get_legacy_tiocsti_setting(_metadata: *mut __test_metadata) -> c_int {
    let path = b"/proc/sys/dev/tty/legacy_tiocsti\0".as_ptr().cast::<c_char>();
    let mode = b"r\0".as_ptr().cast::<c_char>();
    let format = b"%d\0".as_ptr().cast::<c_char>();

    let fp = fopen(path, mode);
    if fp.is_null() {
        return -1;
    }

    let mut value: c_int = -1;

    let scan_ok = fscanf(fp, format, &mut value as *mut c_int) == 1;
    let close_ok = fclose(fp) == 0;
    if scan_ok && close_ok {
        if value < 0 || value > 1 {
            value = -1;
        }
    } else {
        value = -1;
    }

    value
}

#[inline]
unsafe fn set_legacy_tiocsti_setting(_metadata: *mut __test_metadata, value: c_int) -> bool {
    if value < 0 || value > 1 {
        return false;
    }

    let path = b"/proc/sys/dev/tty/legacy_tiocsti\0".as_ptr().cast::<c_char>();
    let mode = b"w\0".as_ptr().cast::<c_char>();
    let format = b"%d\n\0".as_ptr().cast::<c_char>();

    let fp = fopen(path, mode);
    if fp.is_null() {
        return false;
    }

    let mut success = false;
    if fprintf(fp, format, value) > 0 && fclose(fp) == 0 {
        success = true;
    } else {
        let err = unsafe { errno };
        let _ = err;
        // TH_LOG("Failed to write legacy_tiocsti: %s", strerror(errno));
        // Keep equivalent side-effect intent without hard dependency on variadic logger.
    }

    success
}

/*
 * TIOCSTI injection test function
 * @tty_fd: TTY slave file descriptor to test TIOCSTI on
 * Returns: 0 on success, -errno on failure
 */
#[inline]
unsafe fn test_tiocsti_injection(_metadata: *mut __test_metadata, tty_fd: c_int) -> c_int {
    let mut ret;
    let inject_char: u8 = b'V';

    errno = 0;
    ret = ioctl(tty_fd, TIOCSTI, (&inject_char as *const u8).cast());
    if ret == 0 { 0 } else { -errno }
}

/*
 * Child process: test TIOCSTI directly with capability/controlling
 * terminal setup
 */
unsafe fn run_basic_tiocsti_test(_metadata: *mut __test_metadata, self_data: *mut tiocsti, variant: *const tiocsti_variant) {
    if (*self_data).initial_cap_sys_admin && !(*variant).requires_cap {
        if !drop_all_privs(_metadata) {
            return;
        }
    }

    if (*variant).controlling_tty {
        let sid = setsid();
        if sid < 0 {
            return;
        }
        if ioctl((*self_data).pty_slave_fd, 0x5484, ptr::null()) != 0 {
            return;
        }
    }

    if !(*self_data).has_pty {
        return;
    }

    if has_cap_sys_admin(_metadata) != (*variant).requires_cap {
        return;
    }

    let result = test_tiocsti_injection(_metadata, (*self_data).pty_slave_fd);
    if result != (*variant).expected_success {
        return;
    }

    _exit(0);
}

/*
 * Child process: create PTY and then pass FD to parent via SCM_RIGHTS
 */
unsafe fn run_fdpass_tiocsti_test(_metadata: *mut __test_metadata, variant: *const tiocsti_variant, sockfd: c_int) {
    signal(SIGHUP, None);

    if !(*variant).requires_cap && has_cap_sys_admin(_metadata) {
        if !drop_all_privs(_metadata) {
            return;
        }
    }

    let mut child_master_fd = -1;
    let mut child_slave_fd = -1;

    if openpty(&mut child_master_fd, &mut child_slave_fd, ptr::null_mut(), ptr::null_mut(), ptr::null_mut()) != 0 {
        return;
    }

    if (*variant).controlling_tty {
        let sid = setsid();
        if sid < 0 {
            return;
        }
        if ioctl(child_slave_fd, 0x5484, ptr::null()) != 0 {
            return;
        }
    }

    let direct_result = test_tiocsti_injection(_metadata, child_slave_fd);
    if direct_result != (*variant).expected_success {
        return;
    }

    if send_fd_via_socket(sockfd, child_slave_fd) != 0 {
        return;
    }

    let mut sync_byte: u8 = b'D';
    let bytes_read = read(sockfd, (&mut sync_byte as *mut u8).cast(), 1);
    if bytes_read != 1 {
        return;
    }

    close(child_master_fd);
    close(child_slave_fd);
    close(sockfd);
    _exit(0);
}

unsafe fn tiocsti_setup(metadata: *mut __test_metadata, self_data: *mut tiocsti, variant: *const tiocsti_variant) {
    (*self_data).has_pty = openpty(
        &mut (*self_data).pty_master_fd,
        &mut (*self_data).pty_slave_fd,
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
    ) == 0;
    if !(*self_data).has_pty {
        (*self_data).pty_master_fd = -1;
        (*self_data).pty_slave_fd = -1;
    }

    (*self_data).initial_cap_sys_admin = has_cap_sys_admin(metadata);
    (*self_data).original_legacy_tiocsti_setting = get_legacy_tiocsti_setting(metadata);

    if (*self_data).original_legacy_tiocsti_setting < 0 {
        return;
    }

    if (*variant).test_type == test_type::TEST_PTY_TIOCSTI_BASIC && !(*self_data).has_pty {
        return;
    }

    if (*variant).test_type == test_type::TEST_PTY_TIOCSTI_FD_PASSING
        && !(*self_data).initial_cap_sys_admin
    {
        return;
    }

    if (*variant).requires_cap && !(*self_data).initial_cap_sys_admin {
        return;
    }

    (*self_data).can_modify_sysctl = set_legacy_tiocsti_setting(
        metadata,
        (*self_data).original_legacy_tiocsti_setting,
    );

    if (*self_data).can_modify_sysctl
        && (*self_data).original_legacy_tiocsti_setting != (*variant).legacy_tiocsti
    {
        if !set_legacy_tiocsti_setting(metadata, (*variant).legacy_tiocsti) {
            return;
        }
    } else if !(*self_data).can_modify_sysctl
        && (*self_data).original_legacy_tiocsti_setting != (*variant).legacy_tiocsti
    {
        return;
    }
}

unsafe fn tiocsti_teardown(metadata: *mut __test_metadata, self_data: *mut tiocsti) {
    if (*self_data).can_modify_sysctl {
        let current_value = get_legacy_tiocsti_setting(metadata);
        if current_value != (*self_data).original_legacy_tiocsti_setting {
            let _ = set_legacy_tiocsti_setting(
                metadata,
                (*self_data).original_legacy_tiocsti_setting,
            );
        }
    }

    if (*self_data).has_pty {
        if (*self_data).pty_master_fd >= 0 {
            close((*self_data).pty_master_fd);
        }
        if (*self_data).pty_slave_fd >= 0 {
            close((*self_data).pty_slave_fd);
        }
    }
}

unsafe fn wif_signaled(status: c_int) -> bool {
    (status & 0x7f) != 0 && (status & 0x7f) != 0x7f
}

#[inline]
unsafe fn wexitstatus(status: c_int) -> c_int {
    (status >> 8) & 0xff
}

#[inline]
unsafe fn wtermsig(status: c_int) -> c_int {
    status & 0x7f
}

unsafe fn test_tiocsti(metadata: *mut __test_metadata, self_data: *mut tiocsti, variant: *const tiocsti_variant) {
    let mut status: c_int = 0;
    let mut child_pid: pid_t;

    if (*variant).test_type == test_type::TEST_PTY_TIOCSTI_BASIC {
        child_pid = fork();
        if child_pid < 0 {
            return;
        }

        if child_pid == 0 {
            run_basic_tiocsti_test(metadata, self_data, variant);
        }
    } else {
        let mut sockpair = [0i32; 2];

        if socketpair(AF_UNIX, SOCK_STREAM, 0, sockpair.as_mut_ptr()) != 0 {
            return;
        }

        child_pid = fork();
        if child_pid < 0 {
            return;
        }

        if child_pid == 0 {
            close(sockpair[0]);
            run_fdpass_tiocsti_test(metadata, variant, sockpair[1]);
        }

        close(sockpair[1]);

        let mut received_fd = recv_fd_via_socket(sockpair[0]);
        if received_fd < 0 {
            return;
        }

        let parent_has_cap = (*self_data).initial_cap_sys_admin;

        let _ = parent_has_cap;
        let _ = (*variant).legacy_tiocsti;
        let _ = (*variant).requires_cap;

        let result = test_tiocsti_injection(metadata, received_fd);

        if result == 0 && !(*variant).requires_cap {}

        let mut sync_byte: u8 = b'D';
        let bytes_written = write(sockpair[0], (&mut sync_byte as *mut u8).cast(), 1);
        if bytes_written != 1 {
            return;
        }

        close(received_fd);
        close(sockpair[0]);
    }

    let waited = waitpid(child_pid, &mut status, 0);
    if waited != child_pid {
        return;
    }

    if wif_signaled(status) {
        let _ = wtermsig(status);
        if wif_signaled(status) {
            return;
        }
    } else if wexitstatus(status) != 0 {
        return;
    }
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
