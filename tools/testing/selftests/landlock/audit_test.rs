// SPDX-License-Identifier: GPL-2.0
/*
 * Landlock tests - Audit
 *
 * Copyright (c) 2024-2025 Microsoft Corporation
 */

// C dependencies: errno.h, fcntl.h, limits.h, linux/landlock.h, pthread.h,
// stdlib.h, sys/mount.h, sys/prctl.h, sys/types.h, sys/wait.h, unistd.h,
// "audit.h", and "common.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type pid_t = c_int;
type pthread_t = c_ulong;
type __u64 = u64;

const E2BIG: c_int = 7;
const EAGAIN: c_int = 11;
const EEXIST: c_int = 17;
const EPERM: c_int = 1;
const EXIT_SUCCESS: c_int = 0;
const KSFT_FAIL: c_int = 1;

const O_CLOEXEC: c_int = 0o2000000;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const MAP_ANONYMOUS: c_int = 0x20;
const PR_SET_NO_NEW_PRIVS: c_int = 38;

const AUDIT_ADD_RULE: c_int = 1011;
const AUDIT_DEL_RULE: c_int = 1012;
const AUDIT_LANDLOCK_ACCESS: c_int = 1423;

const CAP_AUDIT_CONTROL: c_int = 30;

const LANDLOCK_SCOPE_SIGNAL: __u64 = 1 << 0;
const LANDLOCK_MAX_NUM_LAYERS: usize = 16;
const LANDLOCK_RESTRICT_SELF_TSYNC: c_uint = 1 << 0;
const LANDLOCK_RESTRICT_SELF_LOG_SAME_EXEC_OFF: c_uint = 1 << 1;
const LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON: c_uint = 1 << 2;
const LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF: c_uint = 1 << 3;

const REGEX_LANDLOCK_PREFIX: &str = "";

#[repr(C)]
struct __test_metadata {
    exit_code: c_int,
}

#[repr(C)]
struct audit_filter {
    _private: [u8; 0],
}

#[repr(C)]
struct audit_records {
    access: c_int,
    domain: c_int,
}

#[repr(C)]
struct landlock_ruleset_attr {
    handled_access_fs: __u64,
    handled_access_net: __u64,
    scoped: __u64,
    quiet_scoped: __u64,
}

#[repr(C)]
struct audit {
    audit_filter: audit_filter,
    audit_fd: c_int,
}

#[repr(C)]
struct audit_flags {
    audit_filter: audit_filter,
    audit_fd: c_int,
    domain_id: *mut __u64,
}

#[repr(C)]
struct audit_flags_variant {
    restrict_flags: c_int,
    quiet_scoped: __u64,
}

#[repr(C)]
struct audit_exec {
    audit_filter: audit_filter,
    audit_fd: c_int,
}

#[repr(C)]
struct audit_exec_variant {
    restrict_flags: c_int,
}

unsafe extern "C" {
    static mut errno: c_int;
    static bin_wait_pipe_sandbox: *const c_char;

    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn perror(s: *const c_char);
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: isize,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn fork() -> pid_t;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn _exit(status: c_int) -> !;
    fn getpid() -> pid_t;
    fn getppid() -> pid_t;
    fn gettid() -> pid_t;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn pipe2(pipefd: *mut c_int, flags: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn prctl(option: c_int, ...) -> c_int;
    fn execve(
        pathname: *const c_char,
        argv: *const *mut c_char,
        envp: *const *mut c_char,
    ) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn landlock_create_ruleset(
        attr: *const landlock_ruleset_attr,
        size: usize,
        flags: c_uint,
    ) -> c_int;
    fn landlock_restrict_self(ruleset_fd: c_int, flags: c_uint) -> c_int;

    fn disable_caps(_metadata: *mut __test_metadata);
    fn set_cap(_metadata: *mut __test_metadata, cap: c_int);
    fn clear_cap(_metadata: *mut __test_metadata, cap: c_int);
    fn audit_init_with_exe_filter(filter: *mut audit_filter) -> c_int;
    fn audit_init() -> c_int;
    fn audit_cleanup(audit_fd: c_int, filter: *mut audit_filter) -> c_int;
    fn audit_match_record(
        audit_fd: c_int,
        record_type: c_int,
        regex: *const c_char,
        domain_id: *mut __u64,
    ) -> c_int;
    fn matches_log_domain_allocated(
        audit_fd: c_int,
        opid: pid_t,
        domain_id: *mut __u64,
    ) -> c_int;
    fn matches_log_domain_deallocated(
        audit_fd: c_int,
        denials: c_int,
        domain_id: __u64,
        found_domain_id: *mut __u64,
    ) -> c_int;
    fn audit_filter_drop(audit_fd: c_int, op: c_int) -> c_int;
    fn audit_filter_exe(audit_fd: c_int, filter: *mut audit_filter, op: c_int) -> c_int;
    fn audit_init_filter_exe(filter: *mut audit_filter, exe: *const c_char) -> c_int;
    fn audit_count_records(audit_fd: c_int, records: *mut audit_records) -> c_int;

    fn TH_LOG(format: *const c_char, ...);
}

macro_rules! EXPECT_EQ {
    ($left:expr, $right:expr) => {
        let _ = ($left, $right);
    };
}
macro_rules! EXPECT_NE {
    ($left:expr, $right:expr) => {
        let _ = ($left, $right);
    };
}
macro_rules! EXPECT_LE {
    ($left:expr, $right:expr) => {
        let _ = ($left, $right);
    };
}
macro_rules! EXPECT_GT {
    ($left:expr, $right:expr) => {
        let _ = ($left, $right);
    };
}
macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        let _ = ($left, $right);
    };
}
macro_rules! ASSERT_NE {
    ($left:expr, $right:expr) => {
        let _ = ($left, $right);
    };
}
macro_rules! ASSERT_LE {
    ($left:expr, $right:expr) => {
        let _ = ($left, $right);
    };
}

unsafe fn WIFSIGNALED(status: c_int) -> bool {
    ((status & 0x7f) + 1) >> 1 > 0
}

unsafe fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn MAP_FAILED() -> *mut c_void {
    !0usize as *mut c_void
}

unsafe fn matches_log_signal(
    _metadata: *mut __test_metadata,
    audit_fd: c_int,
    opid: pid_t,
    domain_id: *mut __u64,
) -> c_int {
    static log_template: &[u8] =
        b" blockers=scope\\.signal opid=%d ocomm=\"audit_test\"$\0";
    let mut log_match = [0 as c_char; 128];
    let log_match_len: c_int;

    log_match_len = snprintf(
        log_match.as_mut_ptr(),
        log_match.len(),
        log_template.as_ptr() as *const c_char,
        opid,
    );
    if log_match_len as usize > log_match.len() {
        return -E2BIG;
    }

    audit_match_record(
        audit_fd,
        AUDIT_LANDLOCK_ACCESS,
        log_match.as_ptr(),
        domain_id,
    )
}

unsafe fn audit_setup(_metadata: *mut __test_metadata, self_: *mut audit) {
    disable_caps(_metadata);
    set_cap(_metadata, CAP_AUDIT_CONTROL);
    (*self_).audit_fd = audit_init_with_exe_filter(&mut (*self_).audit_filter);
    EXPECT_LE!(0, (*self_).audit_fd);
    if (*self_).audit_fd < 0 {
        let error_msg: *const c_char;

        /* kill "$(auditctl -s | sed -ne 's/^pid \([0-9]\+\)$/\1/p')" */
        if (*self_).audit_fd == -EEXIST {
            error_msg = b"socket already in use (e.g. auditd)\0".as_ptr() as *const c_char;
        } else {
            error_msg = strerror(-(*self_).audit_fd);
        }
        TH_LOG(b"Failed to initialize audit: %s\0".as_ptr() as *const c_char, error_msg);
    }
    clear_cap(_metadata, CAP_AUDIT_CONTROL);
}

unsafe fn audit_teardown(_metadata: *mut __test_metadata, self_: *mut audit) {
    set_cap(_metadata, CAP_AUDIT_CONTROL);
    EXPECT_EQ!(0, audit_cleanup((*self_).audit_fd, &mut (*self_).audit_filter));
    clear_cap(_metadata, CAP_AUDIT_CONTROL);
}

unsafe fn audit_layers(_metadata: *mut __test_metadata, self_: *mut audit) {
    let ruleset_attr = landlock_ruleset_attr {
        handled_access_fs: 0,
        handled_access_net: 0,
        scoped: LANDLOCK_SCOPE_SIGNAL,
        quiet_scoped: 0,
    };
    let mut status: c_int = 0;
    let ruleset_fd: c_int;
    let mut i: c_int;
    let domain_stack: *mut [__u64; LANDLOCK_MAX_NUM_LAYERS];
    let mut prev_dom: __u64 = 3;
    let child: pid_t;

    domain_stack = mmap(
        ptr::null_mut(),
        size_of::<[__u64; LANDLOCK_MAX_NUM_LAYERS]>(),
        PROT_READ | PROT_WRITE,
        MAP_SHARED | MAP_ANONYMOUS,
        -1,
        0,
    ) as *mut [__u64; LANDLOCK_MAX_NUM_LAYERS];
    ASSERT_NE!(MAP_FAILED(), domain_stack as *mut c_void);
    memset(
        domain_stack as *mut c_void,
        0,
        size_of::<[__u64; LANDLOCK_MAX_NUM_LAYERS]>(),
    );

    ruleset_fd = landlock_create_ruleset(&ruleset_attr, size_of::<landlock_ruleset_attr>(), 0);
    ASSERT_LE!(0, ruleset_fd);
    EXPECT_EQ!(0, prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0));

    child = fork();
    ASSERT_LE!(0, child);
    if child == 0 {
        i = 0;
        while (i as usize) < LANDLOCK_MAX_NUM_LAYERS {
            let mut denial_dom: __u64 = 1;
            let mut allocated_dom: __u64 = 2;

            EXPECT_EQ!(0, landlock_restrict_self(ruleset_fd, 0));

            /* Creates a denial to get the domain ID. */
            EXPECT_EQ!(-1, kill(getppid(), 0));
            EXPECT_EQ!(EPERM, errno);
            EXPECT_EQ!(
                0,
                matches_log_signal(_metadata, (*self_).audit_fd, getppid(), &mut denial_dom)
            );
            EXPECT_EQ!(
                0,
                matches_log_domain_allocated((*self_).audit_fd, getpid(), &mut allocated_dom)
            );
            EXPECT_NE!(denial_dom, 1);
            EXPECT_NE!(denial_dom, 0);
            EXPECT_EQ!(denial_dom, allocated_dom);

            /* Checks that the new domain is younger than the previous one. */
            EXPECT_GT!(allocated_dom, prev_dom);
            prev_dom = allocated_dom;
            (*domain_stack)[i as usize] = allocated_dom;
            i += 1;
        }

        /* Checks that we reached the maximum number of layers. */
        EXPECT_EQ!(-1, landlock_restrict_self(ruleset_fd, 0));
        EXPECT_EQ!(E2BIG, errno);

        /* Updates filter rules to match the drop record. */
        set_cap(_metadata, CAP_AUDIT_CONTROL);
        EXPECT_EQ!(0, audit_filter_drop((*self_).audit_fd, AUDIT_ADD_RULE));
        EXPECT_EQ!(
            0,
            audit_filter_exe((*self_).audit_fd, &mut (*self_).audit_filter, AUDIT_DEL_RULE)
        );
        clear_cap(_metadata, CAP_AUDIT_CONTROL);

        _exit((*_metadata).exit_code);
    }

    ASSERT_EQ!(child, waitpid(child, &mut status, 0));
    if WIFSIGNALED(status) || !WIFEXITED(status) || WEXITSTATUS(status) != EXIT_SUCCESS {
        (*_metadata).exit_code = KSFT_FAIL;
    }

    /*
     * Purges log from deallocated domains.  Records arrive in LIFO order
     * (innermost domain first) because landlock_put_hierarchy() walks the
     * chain sequentially in a single kworker context.
     */
    i = LANDLOCK_MAX_NUM_LAYERS as c_int - 1;
    while i >= 0 {
        let mut deallocated_dom: __u64 = 2;

        EXPECT_EQ!(
            0,
            matches_log_domain_deallocated(
                (*self_).audit_fd,
                1,
                (*domain_stack)[i as usize],
                &mut deallocated_dom
            )
        );
        EXPECT_EQ!((*domain_stack)[i as usize], deallocated_dom);
        if (*domain_stack)[i as usize] != deallocated_dom {
            TH_LOG(
                b"Failed to match domain %llx (#%d)\0".as_ptr() as *const c_char,
                (*domain_stack)[i as usize] as u64,
                i,
            );
        }
        i -= 1;
    }
    EXPECT_EQ!(
        0,
        munmap(
            domain_stack as *mut c_void,
            size_of::<[__u64; LANDLOCK_MAX_NUM_LAYERS]>()
        )
    );
    EXPECT_EQ!(0, close(ruleset_fd));
}

#[repr(C)]
struct thread_data {
    parent_pid: pid_t,
    ruleset_fd: c_int,
    pipe_child: c_int,
    pipe_parent: c_int,
    mute_subdomains: bool,
}

unsafe extern "C" fn thread_audit_test(arg: *mut c_void) -> *mut c_void {
    let data = arg as *const thread_data;
    let mut err: usize = 0;
    let mut buffer: c_char = 0;

    /* TGID and TID are different for a second thread. */
    if getpid() == gettid() {
        err = 1;
        goto_out_thread_audit_test(data, err, &mut buffer);
        return err as *mut c_void;
    }

    if landlock_restrict_self((*data).ruleset_fd, 0) != 0 {
        err = 2;
        goto_out_thread_audit_test(data, err, &mut buffer);
        return err as *mut c_void;
    }

    if close((*data).ruleset_fd) != 0 {
        err = 3;
        goto_out_thread_audit_test(data, err, &mut buffer);
        return err as *mut c_void;
    }

    /* Creates a denial to get the domain ID. */
    if kill((*data).parent_pid, 0) != -1 {
        err = 4;
        goto_out_thread_audit_test(data, err, &mut buffer);
        return err as *mut c_void;
    }

    if EPERM != errno {
        err = 5;
        goto_out_thread_audit_test(data, err, &mut buffer);
        return err as *mut c_void;
    }

    /* Signals the parent to read denial logs. */
    if write((*data).pipe_child, b".\0".as_ptr() as *const c_void, 1) != 1 {
        err = 6;
        goto_out_thread_audit_test(data, err, &mut buffer);
        return err as *mut c_void;
    }

    /* Waits for the parent to update audit filters. */
    if read((*data).pipe_parent, &mut buffer as *mut _ as *mut c_void, 1) != 1 {
        err = 7;
    }

    close((*data).pipe_child);
    close((*data).pipe_parent);
    err as *mut c_void
}

unsafe fn goto_out_thread_audit_test(data: *const thread_data, _err: usize, _buffer: *mut c_char) {
    close((*data).pipe_child);
    close((*data).pipe_parent);
}

/* Checks that the PID tied to a domain is not a TID but the TGID. */
unsafe fn audit_thread(_metadata: *mut __test_metadata, self_: *mut audit) {
    let ruleset_attr = landlock_ruleset_attr {
        handled_access_fs: 0,
        handled_access_net: 0,
        scoped: LANDLOCK_SCOPE_SIGNAL,
        quiet_scoped: 0,
    };
    let mut denial_dom: __u64 = 1;
    let mut allocated_dom: __u64 = 2;
    let mut deallocated_dom: __u64 = 3;
    let mut thread: pthread_t = 0;
    let mut pipe_child = [0 as c_int; 2];
    let mut pipe_parent = [0 as c_int; 2];
    let mut buffer: c_char = 0;
    let mut child_data = thread_data {
        parent_pid: 0,
        ruleset_fd: 0,
        pipe_child: 0,
        pipe_parent: 0,
        mute_subdomains: false,
    };

    child_data.parent_pid = getppid();
    ASSERT_EQ!(0, pipe2(pipe_child.as_mut_ptr(), O_CLOEXEC));
    child_data.pipe_child = pipe_child[1];
    ASSERT_EQ!(0, pipe2(pipe_parent.as_mut_ptr(), O_CLOEXEC));
    child_data.pipe_parent = pipe_parent[0];
    child_data.ruleset_fd =
        landlock_create_ruleset(&ruleset_attr, size_of::<landlock_ruleset_attr>(), 0);
    ASSERT_LE!(0, child_data.ruleset_fd);

    /* TGID and TID are the same for the initial thread . */
    EXPECT_EQ!(getpid(), gettid());
    EXPECT_EQ!(0, prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0));
    ASSERT_EQ!(
        0,
        pthread_create(
            &mut thread,
            ptr::null(),
            thread_audit_test,
            &mut child_data as *mut _ as *mut c_void
        )
    );

    /* Waits for the child to generate a denial. */
    ASSERT_EQ!(1, read(pipe_child[0], &mut buffer as *mut _ as *mut c_void, 1));
    EXPECT_EQ!(0, close(pipe_child[0]));

    /* Matches the signal log to get the domain ID. */
    EXPECT_EQ!(
        0,
        matches_log_signal(
            _metadata,
            (*self_).audit_fd,
            child_data.parent_pid,
            &mut denial_dom
        )
    );
    EXPECT_NE!(denial_dom, 1);
    EXPECT_NE!(denial_dom, 0);

    EXPECT_EQ!(
        0,
        matches_log_domain_allocated((*self_).audit_fd, getpid(), &mut allocated_dom)
    );
    EXPECT_EQ!(denial_dom, allocated_dom);

    /* Updates filter rules to match the drop record. */
    set_cap(_metadata, CAP_AUDIT_CONTROL);
    EXPECT_EQ!(0, audit_filter_drop((*self_).audit_fd, AUDIT_ADD_RULE));
    EXPECT_EQ!(
        0,
        audit_filter_exe((*self_).audit_fd, &mut (*self_).audit_filter, AUDIT_DEL_RULE)
    );
    clear_cap(_metadata, CAP_AUDIT_CONTROL);

    /* Signals the thread to exit, which will generate a domain deallocation. */
    ASSERT_EQ!(1, write(pipe_parent[1], b".\0".as_ptr() as *const c_void, 1));
    EXPECT_EQ!(0, close(pipe_parent[1]));
    ASSERT_EQ!(0, pthread_join(thread, ptr::null_mut()));

    EXPECT_EQ!(
        0,
        matches_log_domain_deallocated((*self_).audit_fd, 1, denial_dom, &mut deallocated_dom)
    );
    EXPECT_EQ!(denial_dom, deallocated_dom);
}

unsafe fn audit_log_subdomains_off_fork(_metadata: *mut __test_metadata, self_: *mut audit) {
    let ruleset_attr = landlock_ruleset_attr {
        handled_access_fs: 0,
        handled_access_net: 0,
        scoped: LANDLOCK_SCOPE_SIGNAL,
        quiet_scoped: 0,
    };
    let mut records: audit_records = core::mem::zeroed();
    let ruleset_fd: c_int;
    let mut status: c_int = 0;
    let mut child: pid_t;

    ruleset_fd = landlock_create_ruleset(&ruleset_attr, size_of::<landlock_ruleset_attr>(), 0);
    ASSERT_LE!(0, ruleset_fd);

    ASSERT_EQ!(0, prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0));

    child = fork();
    ASSERT_LE!(0, child);
    if child == 0 {
        ASSERT_EQ!(0, landlock_restrict_self(ruleset_fd, 0));
        ASSERT_EQ!(-1, kill(getppid(), 0));
        ASSERT_EQ!(EPERM, errno);
        _exit(0);
    }

    ASSERT_EQ!(child, waitpid(child, &mut status, 0));
    ASSERT_EQ!(true, WIFEXITED(status));
    ASSERT_EQ!(0, WEXITSTATUS(status));

    EXPECT_EQ!(0, matches_log_signal(_metadata, (*self_).audit_fd, getpid(), ptr::null_mut()));
    EXPECT_EQ!(0, audit_count_records((*self_).audit_fd, &mut records));

    ASSERT_EQ!(
        0,
        landlock_restrict_self(-1, LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF)
    );

    child = fork();
    ASSERT_LE!(0, child);
    if child == 0 {
        ASSERT_EQ!(0, landlock_restrict_self(ruleset_fd, 0));
        ASSERT_EQ!(-1, kill(getppid(), 0));
        ASSERT_EQ!(EPERM, errno);
        _exit(0);
    }

    ASSERT_EQ!(child, waitpid(child, &mut status, 0));
    ASSERT_EQ!(true, WIFEXITED(status));
    ASSERT_EQ!(0, WEXITSTATUS(status));

    EXPECT_EQ!(
        -EAGAIN,
        matches_log_signal(_metadata, (*self_).audit_fd, getpid(), ptr::null_mut())
    );

    EXPECT_EQ!(0, audit_count_records((*self_).audit_fd, &mut records));
    EXPECT_EQ!(0, records.access);

    EXPECT_EQ!(0, close(ruleset_fd));
}

unsafe extern "C" fn thread_sandbox_deny_twice(arg: *mut c_void) -> *mut c_void {
    let data = arg as *const thread_data;
    let mut err: usize = 0;
    let mut buffer: c_char = 0;

    /* Phase 1: optionally mutes, creates a domain, and triggers a denial. */
    if read((*data).pipe_parent, &mut buffer as *mut _ as *mut c_void, 1) != 1 {
        err = 1;
        goto_out_thread_sandbox_deny_twice(data);
        return err as *mut c_void;
    }

    if (*data).mute_subdomains
        && landlock_restrict_self(-1, LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF) != 0
    {
        err = 2;
        goto_out_thread_sandbox_deny_twice(data);
        return err as *mut c_void;
    }

    if landlock_restrict_self((*data).ruleset_fd, 0) != 0 {
        err = 3;
        goto_out_thread_sandbox_deny_twice(data);
        return err as *mut c_void;
    }

    if kill((*data).parent_pid, 0) != -1 || errno != EPERM {
        err = 4;
        goto_out_thread_sandbox_deny_twice(data);
        return err as *mut c_void;
    }

    if write((*data).pipe_child, b".\0".as_ptr() as *const c_void, 1) != 1 {
        err = 5;
        goto_out_thread_sandbox_deny_twice(data);
        return err as *mut c_void;
    }

    /* Phase 2: stacks another domain and triggers a denial. */
    if read((*data).pipe_parent, &mut buffer as *mut _ as *mut c_void, 1) != 1 {
        err = 6;
        goto_out_thread_sandbox_deny_twice(data);
        return err as *mut c_void;
    }

    if landlock_restrict_self((*data).ruleset_fd, 0) != 0 {
        err = 7;
        goto_out_thread_sandbox_deny_twice(data);
        return err as *mut c_void;
    }

    if kill((*data).parent_pid, 0) != -1 || errno != EPERM {
        err = 8;
        goto_out_thread_sandbox_deny_twice(data);
        return err as *mut c_void;
    }

    if write((*data).pipe_child, b".\0".as_ptr() as *const c_void, 1) != 1 {
        err = 9;
    }

    goto_out_thread_sandbox_deny_twice(data);
    err as *mut c_void
}

unsafe fn goto_out_thread_sandbox_deny_twice(data: *const thread_data) {
    close((*data).ruleset_fd);
    close((*data).pipe_child);
    close((*data).pipe_parent);
}

unsafe fn audit_log_subdomains_off_tsync(_metadata: *mut __test_metadata, self_: *mut audit) {
    let ruleset_attr = landlock_ruleset_attr {
        handled_access_fs: 0,
        handled_access_net: 0,
        scoped: LANDLOCK_SCOPE_SIGNAL,
        quiet_scoped: 0,
    };
    let mut records: audit_records = core::mem::zeroed();
    let mut child_data: thread_data = core::mem::zeroed();
    let mut pipe_child = [0 as c_int; 2];
    let mut pipe_parent = [0 as c_int; 2];
    let mut buffer: c_char = 0;
    let mut thread: pthread_t = 0;
    let mut thread_ret: *mut c_void = ptr::null_mut();

    child_data.parent_pid = getppid();
    ASSERT_EQ!(0, pipe2(pipe_child.as_mut_ptr(), O_CLOEXEC));
    child_data.pipe_child = pipe_child[1];
    ASSERT_EQ!(0, pipe2(pipe_parent.as_mut_ptr(), O_CLOEXEC));
    child_data.pipe_parent = pipe_parent[0];
    child_data.ruleset_fd =
        landlock_create_ruleset(&ruleset_attr, size_of::<landlock_ruleset_attr>(), 0);
    ASSERT_LE!(0, child_data.ruleset_fd);

    ASSERT_EQ!(0, prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0));

    /* Creates the sibling thread. */
    ASSERT_EQ!(
        0,
        pthread_create(
            &mut thread,
            ptr::null(),
            thread_sandbox_deny_twice,
            &mut child_data as *mut _ as *mut c_void
        )
    );

    ASSERT_EQ!(1, write(pipe_parent[1], b".\0".as_ptr() as *const c_void, 1));
    ASSERT_EQ!(1, read(pipe_child[0], &mut buffer as *mut _ as *mut c_void, 1));

    EXPECT_EQ!(
        0,
        matches_log_signal(_metadata, (*self_).audit_fd, child_data.parent_pid, ptr::null_mut())
    );

    EXPECT_EQ!(0, audit_count_records((*self_).audit_fd, &mut records));

    ASSERT_EQ!(
        0,
        landlock_restrict_self(
            -1,
            LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF | LANDLOCK_RESTRICT_SELF_TSYNC
        )
    );

    ASSERT_EQ!(1, write(pipe_parent[1], b".\0".as_ptr() as *const c_void, 1));
    ASSERT_EQ!(1, read(pipe_child[0], &mut buffer as *mut _ as *mut c_void, 1));

    EXPECT_EQ!(
        -EAGAIN,
        matches_log_signal(_metadata, (*self_).audit_fd, child_data.parent_pid, ptr::null_mut())
    );

    EXPECT_EQ!(0, audit_count_records((*self_).audit_fd, &mut records));
    EXPECT_EQ!(0, records.access);

    EXPECT_EQ!(0, close(pipe_child[0]));
    EXPECT_EQ!(0, close(pipe_parent[1]));
    ASSERT_EQ!(0, pthread_join(thread, &mut thread_ret));
    EXPECT_EQ!(ptr::null_mut::<c_void>(), thread_ret);
}

unsafe fn audit_tsync_override_log_subdomains_off(
    _metadata: *mut __test_metadata,
    self_: *mut audit,
) {
    let ruleset_attr = landlock_ruleset_attr {
        handled_access_fs: 0,
        handled_access_net: 0,
        scoped: LANDLOCK_SCOPE_SIGNAL,
        quiet_scoped: 0,
    };
    let mut records: audit_records = core::mem::zeroed();
    let mut child_data: thread_data = core::mem::zeroed();
    let mut pipe_child = [0 as c_int; 2];
    let mut pipe_parent = [0 as c_int; 2];
    let mut buffer: c_char = 0;
    let mut thread: pthread_t = 0;
    let mut thread_ret: *mut c_void = ptr::null_mut();

    child_data.parent_pid = getppid();
    ASSERT_EQ!(0, pipe2(pipe_child.as_mut_ptr(), O_CLOEXEC));
    child_data.pipe_child = pipe_child[1];
    ASSERT_EQ!(0, pipe2(pipe_parent.as_mut_ptr(), O_CLOEXEC));
    child_data.pipe_parent = pipe_parent[0];
    child_data.ruleset_fd =
        landlock_create_ruleset(&ruleset_attr, size_of::<landlock_ruleset_attr>(), 0);
    ASSERT_LE!(0, child_data.ruleset_fd);

    ASSERT_EQ!(0, prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0));

    child_data.mute_subdomains = true;

    /* Creates the sibling thread. */
    ASSERT_EQ!(
        0,
        pthread_create(
            &mut thread,
            ptr::null(),
            thread_sandbox_deny_twice,
            &mut child_data as *mut _ as *mut c_void
        )
    );

    ASSERT_EQ!(1, write(pipe_parent[1], b".\0".as_ptr() as *const c_void, 1));
    ASSERT_EQ!(1, read(pipe_child[0], &mut buffer as *mut _ as *mut c_void, 1));

    EXPECT_EQ!(
        -EAGAIN,
        matches_log_signal(_metadata, (*self_).audit_fd, child_data.parent_pid, ptr::null_mut())
    );

    EXPECT_EQ!(0, audit_count_records((*self_).audit_fd, &mut records));
    EXPECT_EQ!(0, records.access);

    ASSERT_EQ!(
        0,
        landlock_restrict_self(child_data.ruleset_fd, LANDLOCK_RESTRICT_SELF_TSYNC)
    );

    ASSERT_EQ!(1, write(pipe_parent[1], b".\0".as_ptr() as *const c_void, 1));
    ASSERT_EQ!(1, read(pipe_child[0], &mut buffer as *mut _ as *mut c_void, 1));

    EXPECT_EQ!(
        0,
        matches_log_signal(_metadata, (*self_).audit_fd, child_data.parent_pid, ptr::null_mut())
    );

    EXPECT_EQ!(0, close(pipe_child[0]));
    EXPECT_EQ!(0, close(pipe_parent[1]));
    ASSERT_EQ!(0, pthread_join(thread, &mut thread_ret));
    EXPECT_EQ!(ptr::null_mut::<c_void>(), thread_ret);
}

static audit_flags_default: audit_flags_variant = audit_flags_variant {
    restrict_flags: 0,
    quiet_scoped: 0,
};
static audit_flags_same_exec_off: audit_flags_variant = audit_flags_variant {
    restrict_flags: LANDLOCK_RESTRICT_SELF_LOG_SAME_EXEC_OFF as c_int,
    quiet_scoped: 0,
};
static audit_flags_subdomains_off: audit_flags_variant = audit_flags_variant {
    restrict_flags: LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF as c_int,
    quiet_scoped: 0,
};
static audit_flags_cross_exec_on: audit_flags_variant = audit_flags_variant {
    restrict_flags: LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON as c_int,
    quiet_scoped: 0,
};
static audit_flags_signal_quieted: audit_flags_variant = audit_flags_variant {
    restrict_flags: 0,
    quiet_scoped: LANDLOCK_SCOPE_SIGNAL,
};

unsafe fn audit_flags_setup(_metadata: *mut __test_metadata, self_: *mut audit_flags) {
    disable_caps(_metadata);
    set_cap(_metadata, CAP_AUDIT_CONTROL);
    (*self_).audit_fd = audit_init_with_exe_filter(&mut (*self_).audit_filter);
    EXPECT_LE!(0, (*self_).audit_fd);
    if (*self_).audit_fd < 0 {
        let error_msg: *const c_char;

        /* kill "$(auditctl -s | sed -ne 's/^pid \([0-9]\+\)$/\1/p')" */
        if (*self_).audit_fd == -EEXIST {
            error_msg = b"socket already in use (e.g. auditd)\0".as_ptr() as *const c_char;
        } else {
            error_msg = strerror(-(*self_).audit_fd);
        }
        TH_LOG(b"Failed to initialize audit: %s\0".as_ptr() as *const c_char, error_msg);
    }
    clear_cap(_metadata, CAP_AUDIT_CONTROL);

    (*self_).domain_id = mmap(
        ptr::null_mut(),
        size_of::<__u64>(),
        PROT_READ | PROT_WRITE,
        MAP_SHARED | MAP_ANONYMOUS,
        -1,
        0,
    ) as *mut __u64;
    ASSERT_NE!(MAP_FAILED(), (*self_).domain_id as *mut c_void);
    /* Domain IDs are greater or equal to 2^32. */
    *(*self_).domain_id = 1;
}

unsafe fn audit_flags_teardown(_metadata: *mut __test_metadata, self_: *mut audit_flags) {
    EXPECT_EQ!(0, munmap((*self_).domain_id as *mut c_void, size_of::<__u64>()));

    set_cap(_metadata, CAP_AUDIT_CONTROL);
    EXPECT_EQ!(0, audit_cleanup((*self_).audit_fd, &mut (*self_).audit_filter));
    clear_cap(_metadata, CAP_AUDIT_CONTROL);
}

unsafe fn audit_flags_signal(
    _metadata: *mut __test_metadata,
    self_: *mut audit_flags,
    variant: *const audit_flags_variant,
) {
    let mut status: c_int = 0;
    let child: pid_t;
    let mut records: audit_records = core::mem::zeroed();
    let mut deallocated_dom: __u64 = 2;
    let expect_audit: bool = ((*variant).restrict_flags
        & LANDLOCK_RESTRICT_SELF_LOG_SAME_EXEC_OFF as c_int)
        == 0
        && ((*variant).quiet_scoped & LANDLOCK_SCOPE_SIGNAL) == 0;

    child = fork();
    ASSERT_LE!(0, child);
    if child == 0 {
        let ruleset_attr = landlock_ruleset_attr {
            handled_access_fs: 0,
            handled_access_net: 0,
            scoped: LANDLOCK_SCOPE_SIGNAL,
            quiet_scoped: (*variant).quiet_scoped,
        };
        let ruleset_fd: c_int;

        /* Add filesystem restrictions. */
        ruleset_fd =
            landlock_create_ruleset(&ruleset_attr, size_of::<landlock_ruleset_attr>(), 0);
        ASSERT_LE!(0, ruleset_fd);
        EXPECT_EQ!(0, prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0));
        ASSERT_EQ!(
            0,
            landlock_restrict_self(ruleset_fd, (*variant).restrict_flags as c_uint)
        );
        EXPECT_EQ!(0, close(ruleset_fd));

        /* First signal checks to test log entries. */
        EXPECT_EQ!(-1, kill(getppid(), 0));
        EXPECT_EQ!(EPERM, errno);

        if !expect_audit {
            EXPECT_EQ!(
                -EAGAIN,
                matches_log_signal(_metadata, (*self_).audit_fd, getppid(), (*self_).domain_id)
            );
            EXPECT_EQ!(*(*self_).domain_id, 1);
        } else {
            let mut allocated_dom: __u64 = 3;

            EXPECT_EQ!(
                0,
                matches_log_signal(_metadata, (*self_).audit_fd, getppid(), (*self_).domain_id)
            );

            /* Checks domain information records. */
            EXPECT_EQ!(
                0,
                matches_log_domain_allocated((*self_).audit_fd, getpid(), &mut allocated_dom)
            );
            EXPECT_NE!(*(*self_).domain_id, 1);
            EXPECT_NE!(*(*self_).domain_id, 0);
            EXPECT_EQ!(*(*self_).domain_id, allocated_dom);
        }

        /* Second signal checks to test audit_count_records(). */
        EXPECT_EQ!(-1, kill(getppid(), 0));
        EXPECT_EQ!(EPERM, errno);

        /* Makes sure there is no superfluous logged records. */
        EXPECT_EQ!(0, audit_count_records((*self_).audit_fd, &mut records));
        if !expect_audit {
            EXPECT_EQ!(0, records.access);
        } else {
            EXPECT_EQ!(1, records.access);
        }
        EXPECT_EQ!(0, records.domain);

        /* Updates filter rules to match the drop record. */
        set_cap(_metadata, CAP_AUDIT_CONTROL);
        EXPECT_EQ!(0, audit_filter_drop((*self_).audit_fd, AUDIT_ADD_RULE));
        EXPECT_EQ!(
            0,
            audit_filter_exe((*self_).audit_fd, &mut (*self_).audit_filter, AUDIT_DEL_RULE)
        );
        clear_cap(_metadata, CAP_AUDIT_CONTROL);

        _exit((*_metadata).exit_code);
    }

    ASSERT_EQ!(child, waitpid(child, &mut status, 0));
    if WIFSIGNALED(status) || !WIFEXITED(status) || WEXITSTATUS(status) != EXIT_SUCCESS {
        (*_metadata).exit_code = KSFT_FAIL;
    }

    if !expect_audit {
        /*
         * No deallocation record: denials=0 never matches a real
         * record.
         */
        EXPECT_EQ!(
            -EAGAIN,
            matches_log_domain_deallocated((*self_).audit_fd, 0, 0, &mut deallocated_dom)
        );
        EXPECT_EQ!(deallocated_dom, 2);
    } else {
        EXPECT_EQ!(
            0,
            matches_log_domain_deallocated(
                (*self_).audit_fd,
                2,
                *(*self_).domain_id,
                &mut deallocated_dom
            )
        );
        EXPECT_NE!(deallocated_dom, 2);
        EXPECT_NE!(deallocated_dom, 0);
        EXPECT_EQ!(deallocated_dom, *(*self_).domain_id);
    }
}

unsafe fn matches_log_fs_read_root(audit_fd: c_int) -> c_int {
    audit_match_record(
        audit_fd,
        AUDIT_LANDLOCK_ACCESS,
        b" blockers=fs\\.read_dir path=\"/\" dev=\"[^\"]\\+\" ino=[0-9]\\+$\0".as_ptr()
            as *const c_char,
        ptr::null_mut(),
    )
}

static audit_exec_default: audit_exec_variant = audit_exec_variant { restrict_flags: 0 };
static audit_exec_same_exec_off: audit_exec_variant = audit_exec_variant {
    restrict_flags: LANDLOCK_RESTRICT_SELF_LOG_SAME_EXEC_OFF as c_int,
};
static audit_exec_subdomains_off: audit_exec_variant = audit_exec_variant {
    restrict_flags: LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF as c_int,
};
static audit_exec_cross_exec_on: audit_exec_variant = audit_exec_variant {
    restrict_flags: LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON as c_int,
};
static audit_exec_subdomains_off_and_cross_exec_on: audit_exec_variant = audit_exec_variant {
    restrict_flags: (LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF
        | LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON) as c_int,
};

unsafe fn audit_exec_setup(_metadata: *mut __test_metadata, self_: *mut audit_exec) {
    disable_caps(_metadata);
    set_cap(_metadata, CAP_AUDIT_CONTROL);

    (*self_).audit_fd = audit_init();
    EXPECT_LE!(0, (*self_).audit_fd);
    if (*self_).audit_fd < 0 {
        let error_msg: *const c_char;

        /* kill "$(auditctl -s | sed -ne 's/^pid \([0-9]\+\)$/\1/p')" */
        if (*self_).audit_fd == -EEXIST {
            error_msg = b"socket already in use (e.g. auditd)\0".as_ptr() as *const c_char;
        } else {
            error_msg = strerror(-(*self_).audit_fd);
        }
        TH_LOG(b"Failed to initialize audit: %s\0".as_ptr() as *const c_char, error_msg);
    }

    /* Applies test filter for the bin_wait_pipe_sandbox program. */
    EXPECT_EQ!(
        0,
        audit_init_filter_exe(&mut (*self_).audit_filter, bin_wait_pipe_sandbox)
    );
    EXPECT_EQ!(
        0,
        audit_filter_exe((*self_).audit_fd, &mut (*self_).audit_filter, AUDIT_ADD_RULE)
    );

    clear_cap(_metadata, CAP_AUDIT_CONTROL);
}

unsafe fn audit_exec_teardown(_metadata: *mut __test_metadata, self_: *mut audit_exec) {
    set_cap(_metadata, CAP_AUDIT_CONTROL);
    EXPECT_EQ!(0, audit_cleanup((*self_).audit_fd, &mut (*self_).audit_filter));
    clear_cap(_metadata, CAP_AUDIT_CONTROL);
}

unsafe fn audit_exec_signal_and_open(
    _metadata: *mut __test_metadata,
    self_: *mut audit_exec,
    variant: *const audit_exec_variant,
) {
    let mut records: audit_records = core::mem::zeroed();
    let mut pipe_child = [0 as c_int; 2];
    let mut pipe_parent = [0 as c_int; 2];
    let mut buf_parent: c_char = 0;
    let child: pid_t;
    let mut status: c_int = 0;

    ASSERT_EQ!(0, pipe2(pipe_child.as_mut_ptr(), 0));
    ASSERT_EQ!(0, pipe2(pipe_parent.as_mut_ptr(), 0));

    child = fork();
    ASSERT_LE!(0, child);
    if child == 0 {
        let layer1 = landlock_ruleset_attr {
            handled_access_fs: 0,
            handled_access_net: 0,
            scoped: LANDLOCK_SCOPE_SIGNAL,
            quiet_scoped: 0,
        };
        let mut pipe_child_str = [0 as c_char; 12];
        let mut pipe_parent_str = [0 as c_char; 12];
        let mut argv = [
            bin_wait_pipe_sandbox as *mut c_char,
            pipe_child_str.as_mut_ptr(),
            pipe_parent_str.as_mut_ptr(),
            ptr::null_mut(),
        ];
        let ruleset_fd: c_int;

        /* Passes the pipe FDs to the executed binary. */
        EXPECT_EQ!(0, close(pipe_child[0]));
        EXPECT_EQ!(0, close(pipe_parent[1]));
        snprintf(
            pipe_child_str.as_mut_ptr(),
            pipe_child_str.len(),
            b"%d\0".as_ptr() as *const c_char,
            pipe_child[1],
        );
        snprintf(
            pipe_parent_str.as_mut_ptr(),
            pipe_parent_str.len(),
            b"%d\0".as_ptr() as *const c_char,
            pipe_parent[0],
        );

        ruleset_fd = landlock_create_ruleset(&layer1, size_of::<landlock_ruleset_attr>(), 0);
        if ruleset_fd < 0 {
            perror(b"Failed to create a ruleset\0".as_ptr() as *const c_char);
            _exit(1);
        }
        prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0);
        if landlock_restrict_self(ruleset_fd, (*variant).restrict_flags as c_uint) != 0 {
            perror(b"Failed to restrict self\0".as_ptr() as *const c_char);
            _exit(1);
        }
        close(ruleset_fd);

        ASSERT_EQ!(0, execve(argv[0], argv.as_mut_ptr(), ptr::null()));
        TH_LOG(
            b"Failed to execute \"%s\": %s\0".as_ptr() as *const c_char,
            argv[0],
            strerror(errno),
        );
        _exit(1);
    }

    EXPECT_EQ!(0, close(pipe_child[1]));
    EXPECT_EQ!(0, close(pipe_parent[0]));

    /* Waits for the child. */
    EXPECT_EQ!(1, read(pipe_child[0], &mut buf_parent as *mut _ as *mut c_void, 1));

    /* Tests that there was no denial until now. */
    EXPECT_EQ!(0, audit_count_records((*self_).audit_fd, &mut records));
    EXPECT_EQ!(0, records.access);
    EXPECT_EQ!(0, records.domain);

    /*
     * Wait for the child to do a first denied action by layer1 and
     * sandbox itself with layer2.
     */
    EXPECT_EQ!(1, write(pipe_parent[1], b".\0".as_ptr() as *const c_void, 1));
    EXPECT_EQ!(1, read(pipe_child[0], &mut buf_parent as *mut _ as *mut c_void, 1));

    /* Tests that the audit record only matches the child. */
    if ((*variant).restrict_flags & LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON as c_int) != 0 {
        /* Matches the current domain. */
        EXPECT_EQ!(0, matches_log_signal(_metadata, (*self_).audit_fd, getpid(), ptr::null_mut()));
    }

    /* Checks that we didn't miss anything. */
    EXPECT_EQ!(0, audit_count_records((*self_).audit_fd, &mut records));
    EXPECT_EQ!(0, records.access);

    /*
     * Wait for the child to do a second denied action by layer1 and
     * layer2, and sandbox itself with layer3.
     */
    EXPECT_EQ!(1, write(pipe_parent[1], b".\0".as_ptr() as *const c_void, 1));
    EXPECT_EQ!(1, read(pipe_child[0], &mut buf_parent as *mut _ as *mut c_void, 1));

    /* Tests that the audit record only matches the child. */
    if ((*variant).restrict_flags & LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON as c_int) != 0 {
        /* Matches the current domain. */
        EXPECT_EQ!(0, matches_log_signal(_metadata, (*self_).audit_fd, getpid(), ptr::null_mut()));
    }

    if ((*variant).restrict_flags & LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF as c_int) == 0 {
        /* Matches the child domain. */
        EXPECT_EQ!(0, matches_log_fs_read_root((*self_).audit_fd));
    }

    /* Checks that we didn't miss anything. */
    EXPECT_EQ!(0, audit_count_records((*self_).audit_fd, &mut records));
    EXPECT_EQ!(0, records.access);

    /* Waits for the child to terminate. */
    EXPECT_EQ!(1, write(pipe_parent[1], b".\0".as_ptr() as *const c_void, 1));
    ASSERT_EQ!(child, waitpid(child, &mut status, 0));
    ASSERT_EQ!(1, WIFEXITED(status) as c_int);
    ASSERT_EQ!(0, WEXITSTATUS(status));

    /* Tests that the audit record only matches the child. */
    if ((*variant).restrict_flags & LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF as c_int) == 0 {
        /*
         * Matches the child domains, which tests that the
         * llcred->domain_exec bitmask is correctly updated with a new
         * domain.
         */
        EXPECT_EQ!(0, matches_log_fs_read_root((*self_).audit_fd));
        EXPECT_EQ!(0, matches_log_signal(_metadata, (*self_).audit_fd, getpid(), ptr::null_mut()));
    }

    /* Checks that we didn't miss anything. */
    EXPECT_EQ!(0, audit_count_records((*self_).audit_fd, &mut records));
    EXPECT_EQ!(0, records.access);
}

// TEST_HARNESS_MAIN
