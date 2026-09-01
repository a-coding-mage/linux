// SPDX-License-Identifier: GPL-2.0
/*
 * Landlock tests - Common user space base
 *
 * Copyright (c) 2017-2020 Mickael Salaun <mic@digikod.net>
 * Copyright (c) 2019-2020 ANSSI
 */

// C dependencies removed: errno.h, fcntl.h, linux/keyctl.h, linux/landlock.h,
// string.h, sys/prctl.h, sys/socket.h, sys/types.h, and "common.h".
// The syscall wrappers, constants, structs, metadata type, and test assertion
// macros are expected to be supplied by the translated test harness.

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;

const O_PATH: c_int = 0o10000000;

// Old source trees might not have the set of Kselftest fixes related to kernel
// UAPI headers.
const LANDLOCK_CREATE_RULESET_ERRATA: c_uint = 1_u32 << 1;

type pid_t = c_int;

extern "C" {
    static mut errno: c_int;

    fn sysconf(name: c_int) -> c_long;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn close(fd: c_int) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn prctl(option: c_int, ...) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn fork() -> pid_t;
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;
    fn syscall(number: c_long, ...) -> c_long;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn _exit(status: c_int) -> !;

    fn landlock_create_ruleset(
        attr: *const landlock_ruleset_attr,
        size: usize,
        flags: c_uint,
    ) -> c_int;
    fn landlock_add_rule(
        ruleset_fd: c_int,
        rule_type: c_int,
        rule_attr: *const c_void,
        flags: c_uint,
    ) -> c_int;
    fn landlock_restrict_self(ruleset_fd: c_int, flags: c_uint) -> c_int;
    fn drop_caps(metadata: *mut __test_metadata);
    fn enforce_ruleset(metadata: *mut __test_metadata, ruleset_fd: c_int);
    fn send_fd(socket: c_int, fd: c_int) -> c_int;
    fn recv_fd(socket: c_int) -> c_int;
}

#[repr(C)]
pub struct __test_metadata {
    pub exit_code: c_int,
}

#[repr(C)]
pub struct landlock_ruleset_attr {
    pub handled_access_fs: u64,
    pub handled_access_net: u64,
    pub scoped: u64,
    pub quiet_access_fs: u64,
    pub quiet_access_net: u64,
    pub quiet_scoped: u64,
}

#[repr(C)]
pub struct landlock_path_beneath_attr {
    pub allowed_access: u64,
    pub parent_fd: c_int,
}

#[repr(C)]
pub struct landlock_net_port_attr {
    pub allowed_access: u64,
    pub port: u64,
}

unsafe fn inconsistent_attr(_metadata: *mut __test_metadata) {
    let page_size: c_long = sysconf(_SC_PAGESIZE);
    let buf: *mut c_char = malloc((page_size + 1) as usize) as *mut c_char;
    let ruleset_attr: *mut landlock_ruleset_attr = buf as *mut c_void as *mut landlock_ruleset_attr;

    ASSERT_NE!(core::ptr::null_mut::<c_char>(), buf);

    /* Checks copy_from_user(). */
    ASSERT_EQ!(-1, landlock_create_ruleset(ruleset_attr, 0, 0));
    /* The size if less than sizeof(struct landlock_attr_enforce). */
    ASSERT_EQ!(EINVAL, errno);
    ASSERT_EQ!(-1, landlock_create_ruleset(ruleset_attr, 1, 0));
    ASSERT_EQ!(EINVAL, errno);
    ASSERT_EQ!(-1, landlock_create_ruleset(ruleset_attr, 7, 0));
    ASSERT_EQ!(EINVAL, errno);

    ASSERT_EQ!(-1, landlock_create_ruleset(core::ptr::null(), 1, 0));
    /* The size if less than sizeof(struct landlock_attr_enforce). */
    ASSERT_EQ!(EFAULT, errno);

    ASSERT_EQ!(
        -1,
        landlock_create_ruleset(
            core::ptr::null(),
            size_of::<landlock_ruleset_attr>(),
            0,
        )
    );
    ASSERT_EQ!(EFAULT, errno);

    ASSERT_EQ!(
        -1,
        landlock_create_ruleset(ruleset_attr, (page_size + 1) as usize, 0)
    );
    ASSERT_EQ!(E2BIG, errno);

    /* Checks minimal valid attribute size. */
    ASSERT_EQ!(-1, landlock_create_ruleset(ruleset_attr, 8, 0));
    ASSERT_EQ!(ENOMSG, errno);
    ASSERT_EQ!(
        -1,
        landlock_create_ruleset(ruleset_attr, size_of::<landlock_ruleset_attr>(), 0)
    );
    ASSERT_EQ!(ENOMSG, errno);
    ASSERT_EQ!(
        -1,
        landlock_create_ruleset(ruleset_attr, page_size as usize, 0)
    );
    ASSERT_EQ!(ENOMSG, errno);

    /* Checks non-zero value. */
    *buf.offset(page_size - 2) = b'.' as c_char;
    ASSERT_EQ!(
        -1,
        landlock_create_ruleset(ruleset_attr, page_size as usize, 0)
    );
    ASSERT_EQ!(E2BIG, errno);

    ASSERT_EQ!(
        -1,
        landlock_create_ruleset(ruleset_attr, (page_size + 1) as usize, 0)
    );
    ASSERT_EQ!(E2BIG, errno);

    free(buf as *mut c_void);
}

unsafe fn abi_version(_metadata: *mut __test_metadata) {
    let ruleset_attr = landlock_ruleset_attr {
        handled_access_fs: LANDLOCK_ACCESS_FS_READ_FILE,
        handled_access_net: 0,
        scoped: 0,
        quiet_access_fs: 0,
        quiet_access_net: 0,
        quiet_scoped: 0,
    };
    ASSERT_EQ!(
        11,
        landlock_create_ruleset(core::ptr::null(), 0, LANDLOCK_CREATE_RULESET_VERSION)
    );

    ASSERT_EQ!(
        -1,
        landlock_create_ruleset(&ruleset_attr, 0, LANDLOCK_CREATE_RULESET_VERSION)
    );
    ASSERT_EQ!(EINVAL, errno);

    ASSERT_EQ!(
        -1,
        landlock_create_ruleset(
            core::ptr::null(),
            size_of_val(&ruleset_attr),
            LANDLOCK_CREATE_RULESET_VERSION,
        )
    );
    ASSERT_EQ!(EINVAL, errno);

    ASSERT_EQ!(
        -1,
        landlock_create_ruleset(
            &ruleset_attr,
            size_of_val(&ruleset_attr),
            LANDLOCK_CREATE_RULESET_VERSION,
        )
    );
    ASSERT_EQ!(EINVAL, errno);

    ASSERT_EQ!(
        -1,
        landlock_create_ruleset(
            core::ptr::null(),
            0,
            LANDLOCK_CREATE_RULESET_VERSION | (1_u32 << 31),
        )
    );
    ASSERT_EQ!(EINVAL, errno);
}

unsafe fn errata(_metadata: *mut __test_metadata) {
    let ruleset_attr = landlock_ruleset_attr {
        handled_access_fs: LANDLOCK_ACCESS_FS_READ_FILE,
        handled_access_net: 0,
        scoped: 0,
        quiet_access_fs: 0,
        quiet_access_net: 0,
        quiet_scoped: 0,
    };
    let errata: c_int;

    errata = landlock_create_ruleset(core::ptr::null(), 0, LANDLOCK_CREATE_RULESET_ERRATA);
    /* The errata bitmask will not be backported to tests. */
    ASSERT_LE!(0, errata);
    TH_LOG!("errata: 0x%x", errata);

    ASSERT_EQ!(
        -1,
        landlock_create_ruleset(&ruleset_attr, 0, LANDLOCK_CREATE_RULESET_ERRATA)
    );
    ASSERT_EQ!(EINVAL, errno);

    ASSERT_EQ!(
        -1,
        landlock_create_ruleset(
            core::ptr::null(),
            size_of_val(&ruleset_attr),
            LANDLOCK_CREATE_RULESET_ERRATA,
        )
    );
    ASSERT_EQ!(EINVAL, errno);

    ASSERT_EQ!(
        -1,
        landlock_create_ruleset(
            &ruleset_attr,
            size_of_val(&ruleset_attr),
            LANDLOCK_CREATE_RULESET_ERRATA,
        )
    );
    ASSERT_EQ!(EINVAL, errno);

    ASSERT_EQ!(
        -1,
        landlock_create_ruleset(
            core::ptr::null(),
            0,
            LANDLOCK_CREATE_RULESET_VERSION | LANDLOCK_CREATE_RULESET_ERRATA,
        )
    );
    ASSERT_EQ!(
        -1,
        landlock_create_ruleset(
            core::ptr::null(),
            0,
            LANDLOCK_CREATE_RULESET_ERRATA | (1_u32 << 31),
        )
    );
    ASSERT_EQ!(EINVAL, errno);
}

/* Tests ordering of syscall argument checks. */
unsafe fn create_ruleset_checks_ordering(_metadata: *mut __test_metadata) {
    let last_flag: c_int = LANDLOCK_CREATE_RULESET_ERRATA as c_int;
    let invalid_flag: c_int = last_flag << 1;
    let ruleset_fd: c_int;
    let ruleset_attr = landlock_ruleset_attr {
        handled_access_fs: LANDLOCK_ACCESS_FS_READ_FILE,
        handled_access_net: 0,
        scoped: 0,
        quiet_access_fs: 0,
        quiet_access_net: 0,
        quiet_scoped: 0,
    };

    /* Checks priority for invalid flags. */
    ASSERT_EQ!(-1, landlock_create_ruleset(core::ptr::null(), 0, invalid_flag as c_uint));
    ASSERT_EQ!(EINVAL, errno);

    ASSERT_EQ!(-1, landlock_create_ruleset(&ruleset_attr, 0, invalid_flag as c_uint));
    ASSERT_EQ!(EINVAL, errno);

    ASSERT_EQ!(
        -1,
        landlock_create_ruleset(
            core::ptr::null(),
            size_of_val(&ruleset_attr),
            invalid_flag as c_uint,
        )
    );
    ASSERT_EQ!(EINVAL, errno);

    ASSERT_EQ!(
        -1,
        landlock_create_ruleset(
            &ruleset_attr,
            size_of_val(&ruleset_attr),
            invalid_flag as c_uint,
        )
    );
    ASSERT_EQ!(EINVAL, errno);

    /* Checks too big ruleset_attr size. */
    ASSERT_EQ!(-1, landlock_create_ruleset(&ruleset_attr, usize::MAX, 0));
    ASSERT_EQ!(E2BIG, errno);

    /* Checks too small ruleset_attr size. */
    ASSERT_EQ!(-1, landlock_create_ruleset(&ruleset_attr, 0, 0));
    ASSERT_EQ!(EINVAL, errno);
    ASSERT_EQ!(-1, landlock_create_ruleset(&ruleset_attr, 1, 0));
    ASSERT_EQ!(EINVAL, errno);

    /* Checks valid call. */
    ruleset_fd = landlock_create_ruleset(&ruleset_attr, size_of_val(&ruleset_attr), 0);
    ASSERT_LE!(0, ruleset_fd);
    ASSERT_EQ!(0, close(ruleset_fd));
}

/* Tests ordering of syscall argument checks. */
unsafe fn add_rule_checks_ordering(_metadata: *mut __test_metadata) {
    let ruleset_attr = landlock_ruleset_attr {
        handled_access_fs: LANDLOCK_ACCESS_FS_EXECUTE,
        handled_access_net: 0,
        scoped: 0,
        quiet_access_fs: 0,
        quiet_access_net: 0,
        quiet_scoped: 0,
    };
    let mut path_beneath_attr = landlock_path_beneath_attr {
        allowed_access: LANDLOCK_ACCESS_FS_EXECUTE,
        parent_fd: -1,
    };
    let ruleset_fd: c_int =
        landlock_create_ruleset(&ruleset_attr, size_of_val(&ruleset_attr), 0);

    ASSERT_LE!(0, ruleset_fd);

    /* Checks invalid flags. */
    ASSERT_EQ!(-1, landlock_add_rule(-1, 0, core::ptr::null(), 100));
    ASSERT_EQ!(EINVAL, errno);

    /* Checks invalid ruleset FD. */
    ASSERT_EQ!(-1, landlock_add_rule(-1, 0, core::ptr::null(), 0));
    ASSERT_EQ!(EBADF, errno);

    /* Checks invalid rule type. */
    ASSERT_EQ!(-1, landlock_add_rule(ruleset_fd, 0, core::ptr::null(), 0));
    ASSERT_EQ!(EINVAL, errno);

    /* Checks invalid rule attr. */
    ASSERT_EQ!(
        -1,
        landlock_add_rule(
            ruleset_fd,
            LANDLOCK_RULE_PATH_BENEATH,
            core::ptr::null(),
            0,
        )
    );
    ASSERT_EQ!(EFAULT, errno);

    /* Checks invalid path_beneath.parent_fd. */
    ASSERT_EQ!(
        -1,
        landlock_add_rule(
            ruleset_fd,
            LANDLOCK_RULE_PATH_BENEATH,
            &path_beneath_attr as *const _ as *const c_void,
            0,
        )
    );
    ASSERT_EQ!(EBADF, errno);

    /* Checks valid call. */
    path_beneath_attr.parent_fd = open(
        b"/tmp\0".as_ptr() as *const c_char,
        O_PATH | O_NOFOLLOW | O_DIRECTORY | O_CLOEXEC,
    );
    ASSERT_LE!(0, path_beneath_attr.parent_fd);
    ASSERT_EQ!(
        0,
        landlock_add_rule(
            ruleset_fd,
            LANDLOCK_RULE_PATH_BENEATH,
            &path_beneath_attr as *const _ as *const c_void,
            0,
        )
    );
    ASSERT_EQ!(0, close(path_beneath_attr.parent_fd));
    ASSERT_EQ!(0, close(ruleset_fd));
}

/* Tests ordering of syscall argument and permission checks. */
unsafe fn restrict_self_checks_ordering(_metadata: *mut __test_metadata) {
    let ruleset_attr = landlock_ruleset_attr {
        handled_access_fs: LANDLOCK_ACCESS_FS_EXECUTE,
        handled_access_net: 0,
        scoped: 0,
        quiet_access_fs: 0,
        quiet_access_net: 0,
        quiet_scoped: 0,
    };
    let mut path_beneath_attr = landlock_path_beneath_attr {
        allowed_access: LANDLOCK_ACCESS_FS_EXECUTE,
        parent_fd: -1,
    };
    let ruleset_fd: c_int =
        landlock_create_ruleset(&ruleset_attr, size_of_val(&ruleset_attr), 0);

    ASSERT_LE!(0, ruleset_fd);
    path_beneath_attr.parent_fd = open(
        b"/tmp\0".as_ptr() as *const c_char,
        O_PATH | O_NOFOLLOW | O_DIRECTORY | O_CLOEXEC,
    );
    ASSERT_LE!(0, path_beneath_attr.parent_fd);
    ASSERT_EQ!(
        0,
        landlock_add_rule(
            ruleset_fd,
            LANDLOCK_RULE_PATH_BENEATH,
            &path_beneath_attr as *const _ as *const c_void,
            0,
        )
    );
    ASSERT_EQ!(0, close(path_beneath_attr.parent_fd));

    /* Checks unprivileged enforcement without no_new_privs. */
    drop_caps(_metadata);
    /*
     * The flags validity is checked before the no_new_privs /
     * CAP_SYS_ADMIN requirement.
     */
    ASSERT_EQ!(-1, landlock_restrict_self(-1, -1_i32 as c_uint));
    ASSERT_EQ!(EINVAL, errno);
    ASSERT_EQ!(-1, landlock_restrict_self(-1, 0));
    ASSERT_EQ!(EPERM, errno);
    ASSERT_EQ!(-1, landlock_restrict_self(ruleset_fd, 0));
    ASSERT_EQ!(EPERM, errno);
    /*
     * LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS fulfills the no_new_privs /
     * CAP_SYS_ADMIN requirement but requires a ruleset, so the FD is
     * checked next.
     */
    ASSERT_EQ!(
        -1,
        landlock_restrict_self(-1, LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS)
    );
    ASSERT_EQ!(EBADF, errno);

    ASSERT_EQ!(0, prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0));

    /* Checks invalid flags. */
    ASSERT_EQ!(-1, landlock_restrict_self(-1, -1_i32 as c_uint));
    ASSERT_EQ!(EINVAL, errno);

    /* Checks invalid ruleset FD. */
    ASSERT_EQ!(-1, landlock_restrict_self(-1, 0));
    ASSERT_EQ!(EBADF, errno);

    /* Checks valid call. */
    ASSERT_EQ!(0, landlock_restrict_self(ruleset_fd, 0));
    ASSERT_EQ!(0, close(ruleset_fd));
}

unsafe fn restrict_self_max_layers(_metadata: *mut __test_metadata) {
    let ruleset_attr = landlock_ruleset_attr {
        handled_access_fs: LANDLOCK_ACCESS_FS_EXECUTE,
        handled_access_net: 0,
        scoped: 0,
        quiet_access_fs: 0,
        quiet_access_net: 0,
        quiet_scoped: 0,
    };
    let mut path_beneath_attr = landlock_path_beneath_attr {
        allowed_access: LANDLOCK_ACCESS_FS_EXECUTE,
        parent_fd: -1,
    };
    let ruleset_fd: c_int =
        landlock_create_ruleset(&ruleset_attr, size_of_val(&ruleset_attr), 0);
    ASSERT_LE!(0, ruleset_fd);

    path_beneath_attr.parent_fd = open(
        b"/tmp\0".as_ptr() as *const c_char,
        O_PATH | O_NOFOLLOW | O_DIRECTORY | O_CLOEXEC,
    );
    ASSERT_LE!(0, path_beneath_attr.parent_fd);
    ASSERT_EQ!(
        0,
        landlock_add_rule(
            ruleset_fd,
            LANDLOCK_RULE_PATH_BENEATH,
            &path_beneath_attr as *const _ as *const c_void,
            0,
        )
    );
    ASSERT_EQ!(0, close(path_beneath_attr.parent_fd));

    /* Enforces the maximum number of allowed layers. */
    for _i in 0..LANDLOCK_MAX_NUM_LAYERS {
        ASSERT_EQ!(0, landlock_restrict_self(ruleset_fd, 0));
    }

    /* Enforces one too many rulesets. */
    drop_caps(_metadata);
    ASSERT_EQ!(
        -1,
        landlock_restrict_self(ruleset_fd, LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS)
    );
    ASSERT_EQ!(E2BIG, errno);

    /* Checks that the failed call did not set no_new_privs. */
    ASSERT_EQ!(0, prctl(PR_GET_NO_NEW_PRIVS, 0, 0, 0, 0));
    ASSERT_EQ!(0, close(ruleset_fd));
}

unsafe fn restrict_self_fd(_metadata: *mut __test_metadata) {
    let fd: c_int;

    fd = open(b"/dev/null\0".as_ptr() as *const c_char, O_RDONLY | O_CLOEXEC);
    ASSERT_LE!(0, fd);

    EXPECT_EQ!(-1, landlock_restrict_self(fd, 0));
    EXPECT_EQ!(EBADFD, errno);
}

unsafe fn restrict_self_fd_flags(_metadata: *mut __test_metadata) {
    let fd: c_int;

    fd = open(b"/dev/null\0".as_ptr() as *const c_char, O_RDONLY | O_CLOEXEC);
    ASSERT_LE!(0, fd);

    /*
     * LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF accepts -1 but not any file
     * descriptor.
     */
    EXPECT_EQ!(
        -1,
        landlock_restrict_self(fd, LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF)
    );
    EXPECT_EQ!(EBADFD, errno);

    /* LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS requires a ruleset FD. */
    EXPECT_EQ!(
        -1,
        landlock_restrict_self(fd, LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS)
    );
    EXPECT_EQ!(EBADFD, errno);
}

unsafe fn restrict_self_flags(_metadata: *mut __test_metadata) {
    let last_flag: u32 = LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS;

    /* Tests invalid flag combinations. */

    EXPECT_EQ!(-1, landlock_restrict_self(-1, last_flag << 1));
    EXPECT_EQ!(EINVAL, errno);

    EXPECT_EQ!(-1, landlock_restrict_self(-1, -1_i32 as c_uint));
    EXPECT_EQ!(EINVAL, errno);

    /* Tests valid flag combinations. */

    EXPECT_EQ!(-1, landlock_restrict_self(-1, 0));
    EXPECT_EQ!(EBADF, errno);

    EXPECT_EQ!(
        -1,
        landlock_restrict_self(-1, LANDLOCK_RESTRICT_SELF_LOG_SAME_EXEC_OFF)
    );
    EXPECT_EQ!(EBADF, errno);

    EXPECT_EQ!(
        -1,
        landlock_restrict_self(
            -1,
            LANDLOCK_RESTRICT_SELF_LOG_SAME_EXEC_OFF
                | LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF,
        )
    );
    EXPECT_EQ!(EBADF, errno);

    EXPECT_EQ!(
        -1,
        landlock_restrict_self(
            -1,
            LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON
                | LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF,
        )
    );
    EXPECT_EQ!(EBADF, errno);

    EXPECT_EQ!(
        -1,
        landlock_restrict_self(-1, LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON)
    );
    EXPECT_EQ!(EBADF, errno);

    EXPECT_EQ!(
        -1,
        landlock_restrict_self(
            -1,
            LANDLOCK_RESTRICT_SELF_LOG_SAME_EXEC_OFF | LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON,
        )
    );
    EXPECT_EQ!(EBADF, errno);

    /* LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS requires a ruleset FD. */

    EXPECT_EQ!(
        -1,
        landlock_restrict_self(-1, LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS)
    );
    EXPECT_EQ!(EBADF, errno);

    EXPECT_EQ!(
        -1,
        landlock_restrict_self(
            -1,
            LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF | LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS,
        )
    );
    EXPECT_EQ!(EBADF, errno);

    /* Tests with an invalid ruleset_fd. */

    EXPECT_EQ!(
        -1,
        landlock_restrict_self(-2, LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF)
    );
    EXPECT_EQ!(EBADF, errno);

    EXPECT_EQ!(
        0,
        landlock_restrict_self(-1, LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF)
    );
}

unsafe fn restrict_self_no_new_privs(_metadata: *mut __test_metadata) {
    let ruleset_attr = landlock_ruleset_attr {
        handled_access_fs: LANDLOCK_ACCESS_FS_READ_FILE,
        handled_access_net: 0,
        scoped: 0,
        quiet_access_fs: 0,
        quiet_access_net: 0,
        quiet_scoped: 0,
    };
    let ruleset_fd: c_int =
        landlock_create_ruleset(&ruleset_attr, size_of_val(&ruleset_attr), 0);

    ASSERT_LE!(0, ruleset_fd);

    /*
     * The calling thread does not need CAP_SYS_ADMIN nor an explicit
     * prctl(2) PR_SET_NO_NEW_PRIVS call.
     */
    drop_caps(_metadata);
    ASSERT_EQ!(0, prctl(PR_GET_NO_NEW_PRIVS, 0, 0, 0, 0));

    /* Checks that a failed call does not set no_new_privs. */
    EXPECT_EQ!(
        -1,
        landlock_restrict_self(-1, LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS)
    );
    EXPECT_EQ!(EBADF, errno);
    EXPECT_EQ!(0, prctl(PR_GET_NO_NEW_PRIVS, 0, 0, 0, 0));

    /* Checks that a successful call sets no_new_privs. */
    ASSERT_EQ!(
        0,
        landlock_restrict_self(ruleset_fd, LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS)
    );
    EXPECT_EQ!(1, prctl(PR_GET_NO_NEW_PRIVS, 0, 0, 0, 0));

    EXPECT_EQ!(0, close(ruleset_fd));
}

unsafe fn ruleset_fd_io(_metadata: *mut __test_metadata) {
    let ruleset_attr = landlock_ruleset_attr {
        handled_access_fs: LANDLOCK_ACCESS_FS_READ_FILE,
        handled_access_net: 0,
        scoped: 0,
        quiet_access_fs: 0,
        quiet_access_net: 0,
        quiet_scoped: 0,
    };
    let ruleset_fd: c_int;
    let mut buf: c_char = 0;

    drop_caps(_metadata);
    ruleset_fd = landlock_create_ruleset(&ruleset_attr, size_of_val(&ruleset_attr), 0);
    ASSERT_LE!(0, ruleset_fd);

    ASSERT_EQ!(-1, write(ruleset_fd, b".\0".as_ptr() as *const c_void, 1));
    ASSERT_EQ!(EINVAL, errno);
    ASSERT_EQ!(-1, read(ruleset_fd, &mut buf as *mut _ as *mut c_void, 1));
    ASSERT_EQ!(EINVAL, errno);

    ASSERT_EQ!(0, close(ruleset_fd));
}

/* Tests enforcement of a ruleset FD transferred through a UNIX socket. */
unsafe fn ruleset_fd_transfer(_metadata: *mut __test_metadata) {
    let ruleset_attr = landlock_ruleset_attr {
        handled_access_fs: LANDLOCK_ACCESS_FS_READ_DIR,
        handled_access_net: 0,
        scoped: 0,
        quiet_access_fs: 0,
        quiet_access_net: 0,
        quiet_scoped: 0,
    };
    let mut path_beneath_attr = landlock_path_beneath_attr {
        allowed_access: LANDLOCK_ACCESS_FS_READ_DIR,
        parent_fd: 0,
    };
    let ruleset_fd_tx: c_int;
    let mut dir_fd: c_int;
    let mut socket_fds: [c_int; 2] = [0; 2];
    let child: pid_t;
    let mut status: c_int = 0;

    drop_caps(_metadata);

    /* Creates a test ruleset with a simple rule. */
    ruleset_fd_tx = landlock_create_ruleset(&ruleset_attr, size_of_val(&ruleset_attr), 0);
    ASSERT_LE!(0, ruleset_fd_tx);
    path_beneath_attr.parent_fd = open(
        b"/tmp\0".as_ptr() as *const c_char,
        O_PATH | O_NOFOLLOW | O_DIRECTORY | O_CLOEXEC,
    );
    ASSERT_LE!(0, path_beneath_attr.parent_fd);
    ASSERT_EQ!(
        0,
        landlock_add_rule(
            ruleset_fd_tx,
            LANDLOCK_RULE_PATH_BENEATH,
            &path_beneath_attr as *const _ as *const c_void,
            0,
        )
    );
    ASSERT_EQ!(0, close(path_beneath_attr.parent_fd));

    /* Sends the ruleset FD over a socketpair and then close it. */
    ASSERT_EQ!(
        0,
        socketpair(
            AF_UNIX,
            SOCK_STREAM | SOCK_CLOEXEC,
            0,
            socket_fds.as_mut_ptr(),
        )
    );
    ASSERT_EQ!(0, send_fd(socket_fds[0], ruleset_fd_tx));
    ASSERT_EQ!(0, close(socket_fds[0]));
    ASSERT_EQ!(0, close(ruleset_fd_tx));

    child = fork();
    ASSERT_LE!(0, child);
    if child == 0 {
        let ruleset_fd_rx: c_int = recv_fd(socket_fds[1]);

        ASSERT_LE!(0, ruleset_fd_rx);
        ASSERT_EQ!(0, close(socket_fds[1]));

        /* Enforces the received ruleset on the child. */
        ASSERT_EQ!(0, prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0));
        ASSERT_EQ!(0, landlock_restrict_self(ruleset_fd_rx, 0));
        ASSERT_EQ!(0, close(ruleset_fd_rx));

        /* Checks that the ruleset enforcement. */
        ASSERT_EQ!(
            -1,
            open(b"/\0".as_ptr() as *const c_char, O_RDONLY | O_DIRECTORY | O_CLOEXEC)
        );
        ASSERT_EQ!(EACCES, errno);
        dir_fd = open(
            b"/tmp\0".as_ptr() as *const c_char,
            O_RDONLY | O_DIRECTORY | O_CLOEXEC,
        );
        ASSERT_LE!(0, dir_fd);
        ASSERT_EQ!(0, close(dir_fd));
        _exit((*_metadata).exit_code);
        return;
    }

    ASSERT_EQ!(0, close(socket_fds[1]));

    /* Checks that the parent is unrestricted. */
    dir_fd = open(b"/\0".as_ptr() as *const c_char, O_RDONLY | O_DIRECTORY | O_CLOEXEC);
    ASSERT_LE!(0, dir_fd);
    ASSERT_EQ!(0, close(dir_fd));
    dir_fd = open(
        b"/tmp\0".as_ptr() as *const c_char,
        O_RDONLY | O_DIRECTORY | O_CLOEXEC,
    );
    ASSERT_LE!(0, dir_fd);
    ASSERT_EQ!(0, close(dir_fd));

    ASSERT_EQ!(child, waitpid(child, &mut status, 0));
    ASSERT_EQ!(1, WIFEXITED(status));
    ASSERT_EQ!(EXIT_SUCCESS, WEXITSTATUS(status));
}

unsafe fn cred_transfer(_metadata: *mut __test_metadata) {
    let ruleset_attr = landlock_ruleset_attr {
        handled_access_fs: LANDLOCK_ACCESS_FS_READ_DIR,
        handled_access_net: 0,
        scoped: 0,
        quiet_access_fs: 0,
        quiet_access_net: 0,
        quiet_scoped: 0,
    };
    let ruleset_fd: c_int;
    let mut dir_fd: c_int;
    let child: pid_t;
    let mut status: c_int = 0;

    drop_caps(_metadata);

    dir_fd = open(b"/\0".as_ptr() as *const c_char, O_RDONLY | O_DIRECTORY | O_CLOEXEC);
    EXPECT_LE!(0, dir_fd);
    EXPECT_EQ!(0, close(dir_fd));

    /* Denies opening directories. */
    ruleset_fd = landlock_create_ruleset(&ruleset_attr, size_of_val(&ruleset_attr), 0);
    ASSERT_LE!(0, ruleset_fd);
    EXPECT_EQ!(0, prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0));
    ASSERT_EQ!(0, landlock_restrict_self(ruleset_fd, 0));
    EXPECT_EQ!(0, close(ruleset_fd));

    /* Checks ruleset enforcement. */
    EXPECT_EQ!(
        -1,
        open(b"/\0".as_ptr() as *const c_char, O_RDONLY | O_DIRECTORY | O_CLOEXEC)
    );
    EXPECT_EQ!(EACCES, errno);

    /* Needed for KEYCTL_SESSION_TO_PARENT permission checks */
    EXPECT_NE!(
        -1,
        syscall(__NR_keyctl, KEYCTL_JOIN_SESSION_KEYRING, core::ptr::null::<c_void>(), 0, 0, 0)
    );
    {
        TH_LOG!(
            "Failed to join session keyring: %s",
            strerror(errno)
        );
    }

    child = fork();
    ASSERT_LE!(0, child);
    if child == 0 {
        /* Checks ruleset enforcement. */
        EXPECT_EQ!(
            -1,
            open(b"/\0".as_ptr() as *const c_char, O_RDONLY | O_DIRECTORY | O_CLOEXEC)
        );
        EXPECT_EQ!(EACCES, errno);

        /*
         * KEYCTL_SESSION_TO_PARENT is a no-op unless we have a
         * different session keyring in the child, so make that happen.
         */
        EXPECT_NE!(
            -1,
            syscall(
                __NR_keyctl,
                KEYCTL_JOIN_SESSION_KEYRING,
                core::ptr::null::<c_void>(),
                0,
                0,
                0,
            )
        );

        /*
         * KEYCTL_SESSION_TO_PARENT installs credentials on the parent
         * that never go through the cred_prepare hook, this path uses
         * cred_transfer instead.
         */
        EXPECT_EQ!(
            0,
            syscall(__NR_keyctl, KEYCTL_SESSION_TO_PARENT, 0, 0, 0, 0)
        );

        /* Re-checks ruleset enforcement. */
        EXPECT_EQ!(
            -1,
            open(b"/\0".as_ptr() as *const c_char, O_RDONLY | O_DIRECTORY | O_CLOEXEC)
        );
        EXPECT_EQ!(EACCES, errno);

        _exit((*_metadata).exit_code);
        return;
    }

    EXPECT_EQ!(child, waitpid(child, &mut status, 0));
    EXPECT_EQ!(1, WIFEXITED(status));
    EXPECT_EQ!(EXIT_SUCCESS, WEXITSTATUS(status));

    /* Re-checks ruleset enforcement. */
    EXPECT_EQ!(
        -1,
        open(b"/\0".as_ptr() as *const c_char, O_RDONLY | O_DIRECTORY | O_CLOEXEC)
    );
    EXPECT_EQ!(EACCES, errno);
}

unsafe fn useless_quiet_rule_fs(_metadata: *mut __test_metadata) {
    let ruleset_attr = landlock_ruleset_attr {
        handled_access_fs: LANDLOCK_ACCESS_FS_READ_DIR,
        handled_access_net: 0,
        scoped: 0,
        quiet_access_fs: 0,
        quiet_access_net: 0,
        quiet_scoped: 0,
    };
    let mut path_beneath_attr = landlock_path_beneath_attr {
        allowed_access: LANDLOCK_ACCESS_FS_READ_DIR,
        parent_fd: 0,
    };
    let ruleset_fd: c_int;
    let root_fd: c_int;

    drop_caps(_metadata);
    ruleset_fd = landlock_create_ruleset(&ruleset_attr, size_of_val(&ruleset_attr), 0);
    ASSERT_LE!(0, ruleset_fd);

    root_fd = open(b"/\0".as_ptr() as *const c_char, O_PATH | O_CLOEXEC);
    ASSERT_LE!(0, root_fd);
    path_beneath_attr.parent_fd = root_fd;
    ASSERT_EQ!(
        -1,
        landlock_add_rule(
            ruleset_fd,
            LANDLOCK_RULE_PATH_BENEATH,
            &path_beneath_attr as *const _ as *const c_void,
            LANDLOCK_ADD_RULE_QUIET,
        )
    );
    ASSERT_EQ!(EINVAL, errno);

    /* Check that the rule had not been added. */
    ASSERT_EQ!(0, close(root_fd));
    enforce_ruleset(_metadata, ruleset_fd);
    ASSERT_EQ!(0, close(ruleset_fd));

    ASSERT_EQ!(
        -1,
        open(b"/\0".as_ptr() as *const c_char, O_RDONLY | O_DIRECTORY | O_CLOEXEC)
    );
    ASSERT_EQ!(EACCES, errno);
}

unsafe fn useless_quiet_rule_net(_metadata: *mut __test_metadata) {
    let ruleset_attr = landlock_ruleset_attr {
        handled_access_fs: 0,
        handled_access_net: LANDLOCK_ACCESS_NET_BIND_TCP,
        scoped: 0,
        quiet_access_fs: 0,
        quiet_access_net: 0,
        quiet_scoped: 0,
    };
    let net_port_attr = landlock_net_port_attr {
        allowed_access: LANDLOCK_ACCESS_NET_BIND_TCP,
        port: 1024,
    };
    let ruleset_fd: c_int;

    drop_caps(_metadata);
    ruleset_fd = landlock_create_ruleset(&ruleset_attr, size_of_val(&ruleset_attr), 0);
    ASSERT_LE!(0, ruleset_fd);

    ASSERT_EQ!(
        -1,
        landlock_add_rule(
            ruleset_fd,
            LANDLOCK_RULE_NET_PORT,
            &net_port_attr as *const _ as *const c_void,
            LANDLOCK_ADD_RULE_QUIET,
        )
    );
    ASSERT_EQ!(EINVAL, errno);

    ASSERT_EQ!(0, close(ruleset_fd));
}

unsafe fn invalid_quiet_bits_1(_metadata: *mut __test_metadata) {
    let ruleset_attr_fs = landlock_ruleset_attr {
        handled_access_fs: LANDLOCK_ACCESS_FS_READ_DIR,
        handled_access_net: 0,
        scoped: 0,
        quiet_access_fs: LANDLOCK_ACCESS_FS_WRITE_FILE,
        quiet_access_net: 0,
        quiet_scoped: 0,
    };
    let ruleset_attr_net = landlock_ruleset_attr {
        handled_access_fs: 0,
        handled_access_net: LANDLOCK_ACCESS_NET_BIND_TCP,
        scoped: 0,
        quiet_access_fs: 0,
        quiet_access_net: LANDLOCK_ACCESS_NET_CONNECT_TCP,
        quiet_scoped: 0,
    };
    let ruleset_attr_scoped = landlock_ruleset_attr {
        handled_access_fs: 0,
        handled_access_net: 0,
        scoped: LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET,
        quiet_access_fs: 0,
        quiet_access_net: 0,
        quiet_scoped: LANDLOCK_SCOPE_SIGNAL,
    };

    /* Quiet bit set but not part of the handled mask. */
    ASSERT_EQ!(
        -1,
        landlock_create_ruleset(&ruleset_attr_fs, size_of_val(&ruleset_attr_fs), 0)
    );
    ASSERT_EQ!(EINVAL, errno);

    ASSERT_EQ!(
        -1,
        landlock_create_ruleset(&ruleset_attr_net, size_of_val(&ruleset_attr_net), 0)
    );
    ASSERT_EQ!(EINVAL, errno);

    ASSERT_EQ!(
        -1,
        landlock_create_ruleset(&ruleset_attr_scoped, size_of_val(&ruleset_attr_scoped), 0)
    );
    ASSERT_EQ!(EINVAL, errno);
}

unsafe fn invalid_quiet_bits_2(_metadata: *mut __test_metadata) {
    let ruleset_attr_fs = landlock_ruleset_attr {
        handled_access_fs: LANDLOCK_ACCESS_FS_READ_DIR,
        handled_access_net: 0,
        scoped: 0,
        quiet_access_fs: 1_u64 << 63,
        quiet_access_net: 0,
        quiet_scoped: 0,
    };
    let ruleset_attr_net = landlock_ruleset_attr {
        handled_access_fs: 0,
        handled_access_net: LANDLOCK_ACCESS_NET_BIND_TCP,
        scoped: 0,
        quiet_access_fs: 0,
        quiet_access_net: 1_u64 << 63,
        quiet_scoped: 0,
    };
    let ruleset_attr_scoped = landlock_ruleset_attr {
        handled_access_fs: 0,
        handled_access_net: 0,
        scoped: LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET,
        quiet_access_fs: 0,
        quiet_access_net: 0,
        quiet_scoped: 1_u64 << 63,
    };

    /* Quiet bit outside of the valid access range. */
    ASSERT_EQ!(
        -1,
        landlock_create_ruleset(&ruleset_attr_fs, size_of_val(&ruleset_attr_fs), 0)
    );
    ASSERT_EQ!(EINVAL, errno);

    ASSERT_EQ!(
        -1,
        landlock_create_ruleset(&ruleset_attr_net, size_of_val(&ruleset_attr_net), 0)
    );
    ASSERT_EQ!(EINVAL, errno);

    ASSERT_EQ!(
        -1,
        landlock_create_ruleset(&ruleset_attr_scoped, size_of_val(&ruleset_attr_scoped), 0)
    );
    ASSERT_EQ!(EINVAL, errno);
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
