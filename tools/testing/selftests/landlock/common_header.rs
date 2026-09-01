/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Landlock test helpers
 *
 * Copyright (c) 2017-2020 Mickael Salaun <mic@digikod.net>
 * Copyright (c) 2019-2020 ANSSI
 * Copyright (c) 2021 Microsoft Corporation
 */

/* C dependencies: arpa/inet.h, errno.h, linux/securebits.h, sys/capability.h,
 * sys/prctl.h, sys/socket.h, sys/un.h, sys/wait.h, unistd.h,
 * kselftest_harness.h, wrappers.h.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

pub const TMP_DIR: &[u8] = b"tmp\0";

/* TEST_F_FORK() should not be used for new tests.
 * C macro: TEST_F_FORK(fixture_name, test_name) TEST_F(fixture_name, test_name)
 */

pub const LANDLOCK_MAX_NUM_LAYERS: c_int = 16;

pub static bin_sandbox_and_launch: &[u8] = b"./sandbox-and-launch\0";
pub static bin_wait_pipe: &[u8] = b"./wait-pipe\0";
pub static bin_wait_pipe_sandbox: &[u8] = b"./wait-pipe-sandbox\0";

unsafe fn _init_caps(_metadata: *mut __test_metadata, drop_all: bool) {
    let mut cap_p: cap_t;
    /* Only these three capabilities are useful for the tests. */
    let caps: [cap_value_t; 8] = [
        CAP_AUDIT_CONTROL,
        CAP_DAC_OVERRIDE,
        CAP_MKNOD,
        CAP_NET_ADMIN,
        CAP_NET_BIND_SERVICE,
        CAP_SETUID,
        CAP_SYS_ADMIN,
        CAP_SYS_CHROOT,
    ];
    let noroot: c_uint = SECBIT_NOROOT | SECBIT_NOROOT_LOCKED;

    if (cap_get_secbits() & noroot as c_int) != noroot as c_int {
        EXPECT_EQ!(0, cap_set_secbits(noroot as c_int));
    }

    cap_p = cap_get_proc();
    EXPECT_NE!(ptr::null_mut::<c_void>() as cap_t, cap_p);
    EXPECT_NE!(-1, cap_clear(cap_p));
    if !drop_all {
        EXPECT_NE!(
            -1,
            cap_set_flag(
                cap_p,
                CAP_PERMITTED,
                caps.len() as c_int,
                caps.as_ptr(),
                CAP_SET,
            )
        );
    }

    /* Automatically resets ambient capabilities. */
    EXPECT_NE!(-1, cap_set_proc(cap_p), {
        TH_LOG!(
            "Failed to set capabilities: %s\0".as_ptr() as *const c_char,
            strerror(*__errno_location())
        );
    });
    EXPECT_NE!(-1, cap_free(cap_p as *mut c_void));

    /* Quickly checks that ambient capabilities are cleared. */
    EXPECT_NE!(-1, cap_get_ambient(caps[0]));
}

/* We cannot put such helpers in a library because of kselftest_harness.h . */
unsafe fn disable_caps(_metadata: *mut __test_metadata) {
    _init_caps(_metadata, false);
}

unsafe fn drop_caps(_metadata: *mut __test_metadata) {
    _init_caps(_metadata, true);
}

unsafe fn _change_cap(
    _metadata: *mut __test_metadata,
    flag: cap_flag_t,
    cap: cap_value_t,
    value: cap_flag_value_t,
) {
    let mut cap_p: cap_t;

    cap_p = cap_get_proc();
    EXPECT_NE!(ptr::null_mut::<c_void>() as cap_t, cap_p);
    EXPECT_NE!(-1, cap_set_flag(cap_p, flag, 1, &cap, value));
    EXPECT_NE!(-1, cap_set_proc(cap_p), {
        TH_LOG!(
            "Failed to set capability %d: %s\0".as_ptr() as *const c_char,
            cap,
            strerror(*__errno_location())
        );
    });
    EXPECT_NE!(-1, cap_free(cap_p as *mut c_void));
}

unsafe fn set_cap(_metadata: *mut __test_metadata, cap: cap_value_t) {
    _change_cap(_metadata, CAP_EFFECTIVE, cap, CAP_SET);
}

unsafe fn clear_cap(_metadata: *mut __test_metadata, cap: cap_value_t) {
    _change_cap(_metadata, CAP_EFFECTIVE, cap, CAP_CLEAR);
}

unsafe fn set_ambient_cap(_metadata: *mut __test_metadata, cap: cap_value_t) {
    _change_cap(_metadata, CAP_INHERITABLE, cap, CAP_SET);

    EXPECT_NE!(-1, cap_set_ambient(cap, CAP_SET), {
        TH_LOG!(
            "Failed to set ambient capability %d: %s\0".as_ptr() as *const c_char,
            cap,
            strerror(*__errno_location())
        );
    });
}

unsafe fn clear_ambient_cap(_metadata: *mut __test_metadata, cap: cap_value_t) {
    EXPECT_EQ!(1, cap_get_ambient(cap));
    _change_cap(_metadata, CAP_INHERITABLE, cap, CAP_CLEAR);
    EXPECT_EQ!(0, cap_get_ambient(cap));
}

#[repr(C)]
union cmsg_rx_union {
    /* Aligned ancillary data buffer. */
    buf: [c_char; CMSG_SPACE_INT_SIZE],
    _align: cmsghdr,
}

#[repr(C)]
union cmsg_tx_union {
    /* Aligned ancillary data buffer. */
    buf: [c_char; CMSG_SPACE_INT_SIZE],
    _align: cmsghdr,
}

/* Receives an FD from a UNIX socket. Returns the received FD, or -errno. */
unsafe fn recv_fd(usock: c_int) -> c_int {
    let mut fd_rx: c_int = 0;
    let mut cmsg_rx: cmsg_rx_union = zeroed();
    let mut data: c_char = 0;
    let mut io = iovec {
        iov_base: &mut data as *mut c_char as *mut c_void,
        iov_len: size_of::<c_char>(),
    };
    let mut msg = msghdr {
        msg_name: ptr::null_mut(),
        msg_namelen: 0,
        msg_iov: &mut io,
        msg_iovlen: 1,
        msg_control: cmsg_rx.buf.as_mut_ptr() as *mut c_void,
        msg_controllen: size_of::<[c_char; CMSG_SPACE_INT_SIZE]>(),
        msg_flags: 0,
    };
    let mut cmsg: *mut cmsghdr;
    let mut res: c_int;

    res = recvmsg(usock, &mut msg, MSG_CMSG_CLOEXEC);
    if res < 0 {
        return -*__errno_location();
    }

    cmsg = CMSG_FIRSTHDR(&mut msg);
    if (*cmsg).cmsg_len != CMSG_LEN(size_of::<c_int>()) {
        return -EIO;
    }

    memcpy(
        &mut fd_rx as *mut c_int as *mut c_void,
        CMSG_DATA(cmsg) as *const c_void,
        size_of::<c_int>(),
    );
    fd_rx
}

/* Sends an FD on a UNIX socket. Returns 0 on success or -errno. */
unsafe fn send_fd(usock: c_int, fd_tx: c_int) -> c_int {
    let mut cmsg_tx: cmsg_tx_union = zeroed();
    let mut data_tx: c_char = b'.' as c_char;
    let mut io = iovec {
        iov_base: &mut data_tx as *mut c_char as *mut c_void,
        iov_len: size_of::<c_char>(),
    };
    let mut msg = msghdr {
        msg_name: ptr::null_mut(),
        msg_namelen: 0,
        msg_iov: &mut io,
        msg_iovlen: 1,
        msg_control: cmsg_tx.buf.as_mut_ptr() as *mut c_void,
        msg_controllen: size_of::<[c_char; CMSG_SPACE_INT_SIZE]>(),
        msg_flags: 0,
    };
    let mut cmsg: *mut cmsghdr = CMSG_FIRSTHDR(&mut msg);

    (*cmsg).cmsg_len = CMSG_LEN(size_of::<c_int>());
    (*cmsg).cmsg_level = SOL_SOCKET;
    (*cmsg).cmsg_type = SCM_RIGHTS;
    memcpy(
        CMSG_DATA(cmsg),
        &fd_tx as *const c_int as *const c_void,
        size_of::<c_int>(),
    );

    if sendmsg(usock, &msg, 0) < 0 {
        return -*__errno_location();
    }
    0
}

unsafe fn enforce_ruleset(_metadata: *mut __test_metadata, ruleset_fd: c_int) {
    ASSERT_EQ!(0, prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0));
    ASSERT_EQ!(0, landlock_restrict_self(ruleset_fd, 0), {
        TH_LOG!(
            "Failed to enforce ruleset: %s\0".as_ptr() as *const c_char,
            strerror(*__errno_location())
        );
    });
}

unsafe fn drop_access_rights(
    _metadata: *mut __test_metadata,
    ruleset_attr: *const landlock_ruleset_attr,
) {
    let mut ruleset_fd: c_int;

    ruleset_fd = landlock_create_ruleset(
        ruleset_attr,
        size_of::<landlock_ruleset_attr>(),
        0,
    );
    EXPECT_LE!(0, ruleset_fd, {
        TH_LOG!(
            "Failed to create a ruleset: %s\0".as_ptr() as *const c_char,
            strerror(*__errno_location())
        );
    });
    enforce_ruleset(_metadata, ruleset_fd);
    EXPECT_EQ!(0, close(ruleset_fd));
}

#[repr(C)]
pub struct protocol_variant {
    pub domain: c_int,
    pub type_: c_int,
    pub protocol: c_int,
}

#[repr(C)]
pub struct service_fixture {
    pub protocol: protocol_variant,
    /* port is also stored in ipv4_addr.sin_port or ipv6_addr.sin6_port */
    pub port: c_ushort,
    pub addr: service_fixture_addr,
}

#[repr(C)]
pub union service_fixture_addr {
    pub ipv4_addr: sockaddr_in,
    pub ipv6_addr: sockaddr_in6,
    pub unix: service_fixture_unix_addr,
    pub _largest: sockaddr_storage,
}

#[repr(C)]
pub struct service_fixture_unix_addr {
    pub unix_addr: sockaddr_un,
    pub unix_addr_len: socklen_t,
}

unsafe fn set_unix_address(srv: *mut service_fixture, index: c_ushort) {
    (*srv).addr.unix.unix_addr.sun_family = AF_UNIX as sa_family_t;
    sprintf(
        (*srv).addr.unix.unix_addr.sun_path.as_mut_ptr(),
        "_selftests-landlock-abstract-unix-tid%d-index%d\0".as_ptr() as *const c_char,
        sys_gettid(),
        index as c_int,
    );
    (*srv).addr.unix.unix_addr_len = SUN_LEN(&mut (*srv).addr.unix.unix_addr);
    (*srv).addr.unix.unix_addr.sun_path[0] = 0;
}

/**
 * regex_escape - Escape BRE metacharacters in a string
 *
 * @src: Source string to escape.
 * @dst: Destination buffer for the escaped string.
 * @dst_size: Size of the destination buffer.
 *
 * Escapes characters that have special meaning in POSIX Basic Regular
 * Expressions: $ * . [ \ ] ^
 *
 * Returns a pointer to the NUL terminator in @dst (cursor-style API for
 * chaining), or (char *)-ENOMEM if the buffer is too small.
 */
unsafe fn regex_escape(
    src: *const c_char,
    mut dst: *mut c_char,
    dst_size: size_t,
) -> *mut c_char {
    let mut d: *mut c_char = dst;
    let mut s: *const c_char = src;

    while *s != 0 {
        match *s {
            b'$' as c_char | b'*' as c_char | b'.' as c_char | b'[' as c_char
            | b'\\' as c_char | b']' as c_char | b'^' as c_char => {
                if d >= dst.add(dst_size).sub(2) {
                    return -(ENOMEM as isize) as *mut c_char;
                }

                *d = b'\\' as c_char;
                d = d.add(1);
                *d = *s;
                d = d.add(1);
            }
            _ => {
                if d >= dst.add(dst_size).sub(1) {
                    return -(ENOMEM as isize) as *mut c_char;
                }

                *d = *s;
                d = d.add(1);
            }
        }
        s = s.add(1);
    }
    if d >= dst.add(dst_size).sub(1) {
        return -(ENOMEM as isize) as *mut c_char;
    }

    *d = 0;
    d
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
