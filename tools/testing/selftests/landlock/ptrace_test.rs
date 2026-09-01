// SPDX-License-Identifier: GPL-2.0
/*
 * Landlock tests - Ptrace
 *
 * Copyright © 2017-2020 Mickaël Salaün <mic@digikod.net>
 * Copyright © 2019-2020 ANSSI
 * Copyright © 2024-2025 Microsoft Corporation
 */

/*
 * C dependencies removed from executable Rust:
 * errno.h, fcntl.h, linux/landlock.h, sched.h, signal.h, sys/mount.h,
 * sys/prctl.h, sys/ptrace.h, sys/types.h, sys/wait.h, unistd.h, audit.h,
 * common.h, trace.h, scoped_base_variants.h.
 */

use core::ffi::{c_char, c_int, c_long, c_void};

/* Copied from security/yama/yama_lsm.c */
const YAMA_SCOPE_DISABLED: c_int = 0;
const YAMA_SCOPE_RELATIONAL: c_int = 1;

unsafe fn create_domain(_metadata: *mut __test_metadata) {
    let mut ruleset_fd: c_int;
    let ruleset_attr = landlock_ruleset_attr {
        handled_access_fs: LANDLOCK_ACCESS_FS_MAKE_BLOCK,
        ..unsafe { core::mem::zeroed() }
    };

    ruleset_fd = unsafe {
        landlock_create_ruleset(
            &ruleset_attr as *const landlock_ruleset_attr,
            core::mem::size_of_val(&ruleset_attr),
            0,
        )
    };
    EXPECT_LE!(0, ruleset_fd, {
        TH_LOG!(
            c"Failed to create a ruleset: %s".as_ptr(),
            unsafe { strerror(errno()) }
        );
    });
    EXPECT_EQ!(0, unsafe { prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) });
    EXPECT_EQ!(0, unsafe { landlock_restrict_self(ruleset_fd, 0) });
    EXPECT_EQ!(0, unsafe { close(ruleset_fd) });
}

unsafe fn test_ptrace_read(pid: pid_t) -> c_int {
    static PATH_TEMPLATE: &[u8] = b"/proc/%d/environ\0";
    let mut procenv_path = [0 as c_char; b"/proc/%d/environ\0".len() + 10];
    let procenv_path_size: c_int;
    let fd: c_int;

    procenv_path_size = unsafe {
        snprintf(
            procenv_path.as_mut_ptr(),
            procenv_path.len(),
            PATH_TEMPLATE.as_ptr() as *const c_char,
            pid,
        )
    };
    if procenv_path_size >= procenv_path.len() as c_int {
        return E2BIG;
    }

    fd = unsafe { open(procenv_path.as_ptr(), O_RDONLY | O_CLOEXEC) };
    if fd < 0 {
        return errno();
    }
    /*
     * Mixing error codes from close(2) and open(2) should not lead to any
     * (access type) confusion for this test.
     */
    if unsafe { close(fd) } != 0 {
        return errno();
    }
    0
}

unsafe fn get_yama_ptrace_scope() -> c_int {
    let ret: c_int;
    let mut buf = [0 as c_char; 2];
    let fd: c_int =
        unsafe { open(c"/proc/sys/kernel/yama/ptrace_scope".as_ptr(), O_RDONLY) };

    if fd < 0 {
        return 0;
    }

    if unsafe { read(fd, buf.as_mut_ptr() as *mut c_void, 1) } < 0 {
        unsafe { close(fd) };
        return -1;
    }

    ret = unsafe { atoi(buf.as_ptr()) };
    unsafe { close(fd) };
    ret
}

/* clang-format off */
FIXTURE!(scoped_domains, {});
/* clang-format on */

/*
 * Test multiple tracing combinations between a parent process P1 and a child
 * process P2.
 *
 * Yama's scoped ptrace is presumed disabled.  If enabled, this optional
 * restriction is enforced in addition to any Landlock check, which means that
 * all P2 requests to trace P1 would be denied.
 */
/* scoped_base_variants.h supplies the scoped_domains variants. */

FIXTURE_SETUP!(scoped_domains, {
});

FIXTURE_TEARDOWN!(scoped_domains, {
});

/* Test PTRACE_TRACEME and PTRACE_ATTACH for parent and child. */
TEST_F!(scoped_domains, trace, {
    let mut child: pid_t;
    let parent: pid_t;
    let mut status: c_int = 0;
    let mut err_proc_read: c_int;
    let mut pipe_child = [0 as c_int; 2];
    let mut pipe_parent = [0 as c_int; 2];
    let yama_ptrace_scope: c_int;
    let mut buf_parent: c_char = 0;
    let mut ret: c_long;
    let can_read_child: bool;
    let can_trace_child: bool;
    let can_read_parent: bool;
    let can_trace_parent: bool;

    yama_ptrace_scope = unsafe { get_yama_ptrace_scope() };
    ASSERT_LE!(0, yama_ptrace_scope);

    if yama_ptrace_scope > YAMA_SCOPE_DISABLED {
        TH_LOG!(
            c"Incomplete tests due to Yama restrictions (scope %d)".as_ptr(),
            yama_ptrace_scope
        );
    }

    /*
     * can_read_child is true if a parent process can read its child
     * process, which is only the case when the parent process is not
     * isolated from the child with a dedicated Landlock domain.
     */
    can_read_child = unsafe { !(*variant).domain_parent };

    /*
     * can_trace_child is true if a parent process can trace its child
     * process.  This depends on two conditions:
     * - The parent process is not isolated from the child with a dedicated
     *   Landlock domain.
     * - Yama allows tracing children (up to YAMA_SCOPE_RELATIONAL).
     */
    can_trace_child = can_read_child && yama_ptrace_scope <= YAMA_SCOPE_RELATIONAL;

    /*
     * can_read_parent is true if a child process can read its parent
     * process, which is only the case when the child process is not
     * isolated from the parent with a dedicated Landlock domain.
     */
    can_read_parent = unsafe { !(*variant).domain_child };

    /*
     * can_trace_parent is true if a child process can trace its parent
     * process.  This depends on two conditions:
     * - The child process is not isolated from the parent with a dedicated
     *   Landlock domain.
     * - Yama is disabled (YAMA_SCOPE_DISABLED).
     */
    can_trace_parent = can_read_parent && yama_ptrace_scope <= YAMA_SCOPE_DISABLED;

    /*
     * Removes all effective and permitted capabilities to not interfere
     * with cap_ptrace_access_check() in case of PTRACE_MODE_FSCREDS.
     */
    unsafe { drop_caps(_metadata) };

    parent = unsafe { getpid() };
    ASSERT_EQ!(0, unsafe { pipe2(pipe_child.as_mut_ptr(), O_CLOEXEC) });
    ASSERT_EQ!(0, unsafe { pipe2(pipe_parent.as_mut_ptr(), O_CLOEXEC) });
    if unsafe { (*variant).domain_both } {
        unsafe { create_domain(_metadata) };
        if unsafe { !__test_passed(_metadata) } {
            /* Aborts before forking. */
            return;
        }
    }

    child = unsafe { fork() };
    ASSERT_LE!(0, child);
    if child == 0 {
        let mut buf_child: c_char = 0;

        ASSERT_EQ!(0, unsafe { close(pipe_parent[1]) });
        ASSERT_EQ!(0, unsafe { close(pipe_child[0]) });
        if unsafe { (*variant).domain_child } {
            unsafe { create_domain(_metadata) };
        }

        /* Waits for the parent to be in a domain, if any. */
        ASSERT_EQ!(1, unsafe {
            read(pipe_parent[0], &mut buf_child as *mut c_char as *mut c_void, 1)
        });

        /* Tests PTRACE_MODE_READ on the parent. */
        err_proc_read = unsafe { test_ptrace_read(parent) };
        if can_read_parent {
            EXPECT_EQ!(0, err_proc_read);
        } else {
            EXPECT_EQ!(EACCES, err_proc_read);
        }

        /* Tests PTRACE_ATTACH on the parent. */
        ret = unsafe { ptrace(PTRACE_ATTACH, parent, core::ptr::null_mut::<c_void>(), 0) };
        if can_trace_parent {
            EXPECT_EQ!(0, ret);
        } else {
            EXPECT_EQ!(-1, ret);
            EXPECT_EQ!(EPERM, errno());
        }
        if ret == 0 {
            ASSERT_EQ!(parent, unsafe { waitpid(parent, &mut status, 0) });
            ASSERT_EQ!(1, WIFSTOPPED(status));
            ASSERT_EQ!(0, unsafe {
                ptrace(PTRACE_DETACH, parent, core::ptr::null_mut::<c_void>(), 0)
            });
        }

        /* Tests child PTRACE_TRACEME. */
        ret = unsafe { ptrace(PTRACE_TRACEME, 0, core::ptr::null_mut::<c_void>(), 0) };
        if can_trace_child {
            EXPECT_EQ!(0, ret);
        } else {
            EXPECT_EQ!(-1, ret);
            EXPECT_EQ!(EPERM, errno());
        }

        /*
         * Signals that the PTRACE_ATTACH test is done and the
         * PTRACE_TRACEME test is ongoing.
         */
        ASSERT_EQ!(1, unsafe {
            write(pipe_child[1], c".".as_ptr() as *const c_void, 1)
        });

        if can_trace_child {
            ASSERT_EQ!(0, unsafe { raise(SIGSTOP) });
        }

        /* Waits for the parent PTRACE_ATTACH test. */
        ASSERT_EQ!(1, unsafe {
            read(pipe_parent[0], &mut buf_child as *mut c_char as *mut c_void, 1)
        });
        unsafe { _exit((*_metadata).exit_code) };
        return;
    }

    ASSERT_EQ!(0, unsafe { close(pipe_child[1]) });
    ASSERT_EQ!(0, unsafe { close(pipe_parent[0]) });
    if unsafe { (*variant).domain_parent } {
        unsafe { create_domain(_metadata) };
    }

    /* Signals that the parent is in a domain, if any. */
    ASSERT_EQ!(1, unsafe {
        write(pipe_parent[1], c".".as_ptr() as *const c_void, 1)
    });

    /*
     * Waits for the child to test PTRACE_ATTACH on the parent and start
     * testing PTRACE_TRACEME.
     */
    ASSERT_EQ!(1, unsafe {
        read(pipe_child[0], &mut buf_parent as *mut c_char as *mut c_void, 1)
    });

    /* Tests child PTRACE_TRACEME. */
    if can_trace_child {
        ASSERT_EQ!(child, unsafe { waitpid(child, &mut status, 0) });
        ASSERT_EQ!(1, WIFSTOPPED(status));
        ASSERT_EQ!(0, unsafe {
            ptrace(PTRACE_DETACH, child, core::ptr::null_mut::<c_void>(), 0)
        });
    } else {
        /* The child should not be traced by the parent. */
        EXPECT_EQ!(-1, unsafe {
            ptrace(PTRACE_DETACH, child, core::ptr::null_mut::<c_void>(), 0)
        });
        EXPECT_EQ!(ESRCH, errno());
    }

    /* Tests PTRACE_MODE_READ on the child. */
    err_proc_read = unsafe { test_ptrace_read(child) };
    if can_read_child {
        EXPECT_EQ!(0, err_proc_read);
    } else {
        EXPECT_EQ!(EACCES, err_proc_read);
    }

    /* Tests PTRACE_ATTACH on the child. */
    ret = unsafe { ptrace(PTRACE_ATTACH, child, core::ptr::null_mut::<c_void>(), 0) };
    if can_trace_child {
        EXPECT_EQ!(0, ret);
    } else {
        EXPECT_EQ!(-1, ret);
        EXPECT_EQ!(EPERM, errno());
    }

    if ret == 0 {
        ASSERT_EQ!(child, unsafe { waitpid(child, &mut status, 0) });
        ASSERT_EQ!(1, WIFSTOPPED(status));
        ASSERT_EQ!(0, unsafe {
            ptrace(PTRACE_DETACH, child, core::ptr::null_mut::<c_void>(), 0)
        });
    }

    /* Signals that the parent PTRACE_ATTACH test is done. */
    ASSERT_EQ!(1, unsafe {
        write(pipe_parent[1], c".".as_ptr() as *const c_void, 1)
    });
    ASSERT_EQ!(child, unsafe { waitpid(child, &mut status, 0) });

    if WIFSIGNALED(status) || !WIFEXITED(status) || WEXITSTATUS(status) != EXIT_SUCCESS {
        unsafe {
            (*_metadata).exit_code = KSFT_FAIL;
        }
    }
});

unsafe fn matches_log_ptrace(
    _metadata: *mut __test_metadata,
    audit_fd: c_int,
    opid: pid_t,
) -> c_int {
    static LOG_TEMPLATE: &[u8] =
        concat!(REGEX_LANDLOCK_PREFIX, " blockers=ptrace opid=%d ocomm=\"ptrace_test\"$\0")
            .as_bytes();
    let mut log_match = [0 as c_char; LOG_TEMPLATE.len() + 10];
    let log_match_len: c_int;

    log_match_len = unsafe {
        snprintf(
            log_match.as_mut_ptr(),
            log_match.len(),
            LOG_TEMPLATE.as_ptr() as *const c_char,
            opid,
        )
    };
    if log_match_len > log_match.len() as c_int {
        return -E2BIG;
    }

    unsafe {
        audit_match_record(
            audit_fd,
            AUDIT_LANDLOCK_ACCESS,
            log_match.as_ptr(),
            core::ptr::null_mut(),
        )
    }
}

FIXTURE!(audit, {
    audit_filter: audit_filter,
    audit_fd: c_int,
});

FIXTURE_SETUP!(audit, {
    unsafe { disable_caps(_metadata) };
    unsafe { set_cap(_metadata, CAP_AUDIT_CONTROL) };
    self_.audit_fd = unsafe { audit_init_with_exe_filter(&mut self_.audit_filter) };
    EXPECT_LE!(0, self_.audit_fd);
    unsafe { clear_cap(_metadata, CAP_AUDIT_CONTROL) };
});

FIXTURE_TEARDOWN_PARENT!(audit, {
    EXPECT_EQ!(0, unsafe { audit_cleanup(-1, core::ptr::null_mut()) });
});

/* Test PTRACE_TRACEME and PTRACE_ATTACH for parent and child. */
TEST_F!(audit, trace, {
    let child: pid_t;
    let mut status: c_int = 0;
    let mut pipe_child = [0 as c_int; 2];
    let mut pipe_parent = [0 as c_int; 2];
    let yama_ptrace_scope: c_int;
    let mut buf_parent: c_char = 0;
    let mut records: audit_records = unsafe { core::mem::zeroed() };

    /* Makes sure there is no superfluous logged records. */
    EXPECT_EQ!(0, unsafe { audit_count_records(self_.audit_fd, &mut records) });
    EXPECT_EQ!(0, records.access);
    EXPECT_EQ!(0, records.domain);

    yama_ptrace_scope = unsafe { get_yama_ptrace_scope() };
    ASSERT_LE!(0, yama_ptrace_scope);

    if yama_ptrace_scope > YAMA_SCOPE_DISABLED {
        TH_LOG!(
            c"Incomplete tests due to Yama restrictions (scope %d)".as_ptr(),
            yama_ptrace_scope
        );
    }

    /*
     * Removes all effective and permitted capabilities to not interfere
     * with cap_ptrace_access_check() in case of PTRACE_MODE_FSCREDS.
     */
    unsafe { drop_caps(_metadata) };

    ASSERT_EQ!(0, unsafe { pipe2(pipe_child.as_mut_ptr(), O_CLOEXEC) });
    ASSERT_EQ!(0, unsafe { pipe2(pipe_parent.as_mut_ptr(), O_CLOEXEC) });

    child = unsafe { fork() };
    ASSERT_LE!(0, child);
    if child == 0 {
        let mut buf_child: c_char = 0;

        ASSERT_EQ!(0, unsafe { close(pipe_parent[1]) });
        ASSERT_EQ!(0, unsafe { close(pipe_child[0]) });

        /* Waits for the parent to be in a domain, if any. */
        ASSERT_EQ!(1, unsafe {
            read(pipe_parent[0], &mut buf_child as *mut c_char as *mut c_void, 1)
        });

        /* Tests child PTRACE_TRACEME. */
        EXPECT_EQ!(-1, unsafe { ptrace(PTRACE_TRACEME, 0, core::ptr::null_mut::<c_void>(), 0) });
        EXPECT_EQ!(EPERM, errno());
        /* We should see the child process. */
        EXPECT_EQ!(0, unsafe { matches_log_ptrace(_metadata, self_.audit_fd, getpid()) });

        EXPECT_EQ!(0, unsafe { audit_count_records(self_.audit_fd, &mut records) });
        EXPECT_EQ!(0, records.access);
        /* Checks for a domain creation. */
        EXPECT_EQ!(1, records.domain);

        /*
         * Signals that the PTRACE_ATTACH test is done and the
         * PTRACE_TRACEME test is ongoing.
         */
        ASSERT_EQ!(1, unsafe { write(pipe_child[1], c".".as_ptr() as *const c_void, 1) });

        /* Waits for the parent PTRACE_ATTACH test. */
        ASSERT_EQ!(1, unsafe {
            read(pipe_parent[0], &mut buf_child as *mut c_char as *mut c_void, 1)
        });
        unsafe { _exit((*_metadata).exit_code) };
        return;
    }

    ASSERT_EQ!(0, unsafe { close(pipe_child[1]) });
    ASSERT_EQ!(0, unsafe { close(pipe_parent[0]) });
    unsafe { create_domain(_metadata) };

    /* Signals that the parent is in a domain. */
    ASSERT_EQ!(1, unsafe { write(pipe_parent[1], c".".as_ptr() as *const c_void, 1) });

    /*
     * Waits for the child to test PTRACE_ATTACH on the parent and start
     * testing PTRACE_TRACEME.
     */
    ASSERT_EQ!(1, unsafe {
        read(pipe_child[0], &mut buf_parent as *mut c_char as *mut c_void, 1)
    });

    /* The child should not be traced by the parent. */
    EXPECT_EQ!(-1, unsafe {
        ptrace(PTRACE_DETACH, child, core::ptr::null_mut::<c_void>(), 0)
    });
    EXPECT_EQ!(ESRCH, errno());

    /* Tests PTRACE_ATTACH on the child. */
    EXPECT_EQ!(-1, unsafe {
        ptrace(PTRACE_ATTACH, child, core::ptr::null_mut::<c_void>(), 0)
    });
    EXPECT_EQ!(EPERM, errno());
    EXPECT_EQ!(0, unsafe { matches_log_ptrace(_metadata, self_.audit_fd, child) });

    /* Signals that the parent PTRACE_ATTACH test is done. */
    ASSERT_EQ!(1, unsafe { write(pipe_parent[1], c".".as_ptr() as *const c_void, 1) });
    ASSERT_EQ!(child, unsafe { waitpid(child, &mut status, 0) });
    if WIFSIGNALED(status) || !WIFEXITED(status) || WEXITSTATUS(status) != EXIT_SUCCESS {
        unsafe {
            (*_metadata).exit_code = KSFT_FAIL;
        }
    }

    /* Makes sure there is no superfluous logged records. */
    EXPECT_EQ!(0, unsafe { audit_count_records(self_.audit_fd, &mut records) });
    EXPECT_EQ!(0, records.access);
    EXPECT_EQ!(0, records.domain);
});

/* Trace tests */

/* clang-format off */
FIXTURE!(trace_ptrace, {
    /* clang-format on */
    tracefs_ok: c_int,
});

FIXTURE_SETUP!(trace_ptrace, {
    let ret: c_int;

    unsafe { set_cap(_metadata, CAP_SYS_ADMIN) };
    ASSERT_EQ!(0, unsafe { unshare(CLONE_NEWNS) });
    ASSERT_EQ!(0, unsafe {
        mount(
            core::ptr::null(),
            c"/".as_ptr(),
            core::ptr::null(),
            MS_REC | MS_PRIVATE,
            core::ptr::null(),
        )
    });

    ret = unsafe { tracefs_fixture_setup() };
    if ret != 0 {
        unsafe { clear_cap(_metadata, CAP_SYS_ADMIN) };
        self_.tracefs_ok = 0;
        SKIP!(return, c"tracefs not available".as_ptr());
    }
    self_.tracefs_ok = 1;

    ASSERT_EQ!(0, unsafe { tracefs_enable_event(TRACEFS_DENY_PTRACE_ENABLE, true) });
    ASSERT_EQ!(0, unsafe { tracefs_clear() });
    unsafe { clear_cap(_metadata, CAP_SYS_ADMIN) };
});

FIXTURE_TEARDOWN!(trace_ptrace, {
    if self_.tracefs_ok == 0 {
        return;
    }

    unsafe { set_cap(_metadata, CAP_SYS_ADMIN) };
    unsafe { tracefs_enable_event(TRACEFS_DENY_PTRACE_ENABLE, false) };
    unsafe { tracefs_fixture_teardown() };
    unsafe { clear_cap(_metadata, CAP_SYS_ADMIN) };
});

/* clang-format off */
FIXTURE_VARIANT!(trace_ptrace, {
    /* clang-format on */
    sandbox: bool,
    sandbox_target: bool,
    expect_denied: c_int,
});

/* Denied: sandboxed child ptraces unsandboxed parent (tracee_domain=0). */
/* clang-format off */
FIXTURE_VARIANT_ADD!(trace_ptrace, denied, {
    /* clang-format on */
    sandbox: true,
    sandbox_target: false,
    expect_denied: 1,
});

/*
 * Denied: sandboxed child ptraces a sandboxed parent, so the tracee is in a
 * domain and tracee_domain= is non-zero.
 */
/* clang-format off */
FIXTURE_VARIANT_ADD!(trace_ptrace, denied_scoped_target, {
    /* clang-format on */
    sandbox: true,
    sandbox_target: true,
    expect_denied: 1,
});

/* Allowed: unsandboxed child uses PTRACE_TRACEME. */
/* clang-format off */
FIXTURE_VARIANT_ADD!(trace_ptrace, allowed, {
    /* clang-format on */
    sandbox: false,
    sandbox_target: false,
    expect_denied: 0,
});

TEST_F!(trace_ptrace, deny_ptrace, {
    let buf: *mut c_char;
    let mut field = [0 as c_char; 64];
    let mut expected_pid = [0 as c_char; 16];
    let count: c_int;
    let mut status: c_int = 0;
    let child: pid_t;
    let parent: pid_t;

    if self_.tracefs_ok == 0 {
        SKIP!(return, c"tracefs not available".as_ptr());
    }

    parent = unsafe { getpid() };

    /*
     * Set a known comm so the denied variant can verify both the trace line
     * task name and the tracee_comm= field.
     */
    unsafe { prctl(PR_SET_NAME, c"ll_trace_test".as_ptr(), 0, 0, 0) };

    /*
     * For the non-zero tracee_domain case, sandbox the parent (the tracee)
     * before forking.  The child inherits that domain and adds its own
     * layer, so the child (tracer) is not an ancestor of the tracee and the
     * ptrace is still denied, with tracee_domain= naming the parent's
     * domain.
     */
    if variant.sandbox_target {
        unsafe { create_domain(_metadata) };
    }

    child = unsafe { fork() };
    ASSERT_LE!(0, child);

    if child == 0 {
        if variant.sandbox {
            let ruleset_attr = landlock_ruleset_attr {
                scoped: LANDLOCK_SCOPE_SIGNAL,
                ..unsafe { core::mem::zeroed() }
            };
            let ruleset_fd: c_int;

            /*
             * Any scope creates a domain.  Ptrace denial checks
             * domain ancestry, not specific flags.
             */
            ruleset_fd = unsafe {
                landlock_create_ruleset(
                    &ruleset_attr as *const landlock_ruleset_attr,
                    core::mem::size_of_val(&ruleset_attr),
                    0,
                )
            };
            if ruleset_fd < 0 {
                unsafe { _exit(1) };
            }

            unsafe { prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) };
            if unsafe { landlock_restrict_self(ruleset_fd, 0) } != 0 {
                unsafe { close(ruleset_fd) };
                unsafe { _exit(1) };
            }
            unsafe { close(ruleset_fd) };

            /* PTRACE_ATTACH on unsandboxed parent: denied. */
            if unsafe { ptrace(PTRACE_ATTACH, parent, core::ptr::null_mut::<c_void>(), core::ptr::null_mut::<c_void>()) } == 0 {
                unsafe { ptrace(PTRACE_DETACH, parent, core::ptr::null_mut::<c_void>(), core::ptr::null_mut::<c_void>()) };
                unsafe { _exit(2) };
            }
            if errno() != EPERM {
                unsafe { _exit(3) };
            }
        } else {
            /* No sandbox: ptrace should succeed. */
            if unsafe { ptrace(PTRACE_TRACEME, 0, core::ptr::null_mut::<c_void>(), 0) } != 0 {
                unsafe { _exit(1) };
            }
        }

        unsafe { _exit(0) };
    }

    ASSERT_EQ!(child, unsafe { waitpid(child, &mut status, 0) });
    ASSERT_TRUE!(WIFEXITED(status));
    EXPECT_EQ!(0, WEXITSTATUS(status));

    buf = unsafe { tracefs_read_buf() };
    ASSERT_NE!(core::ptr::null_mut::<c_char>(), buf);

    count = unsafe { tracefs_count_matches(buf, REGEX_DENY_PTRACE!(c"ll_trace_test".as_ptr())) };
    if variant.expect_denied != 0 {
        EXPECT_EQ!(variant.expect_denied, count, {
            TH_LOG!(c"Expected deny_ptrace event, got %d\n%s".as_ptr(), count, buf);
        });

        /* Verify tracee_pid is the parent's TGID. */
        unsafe {
            snprintf(
                expected_pid.as_mut_ptr(),
                expected_pid.len(),
                c"%d".as_ptr(),
                parent,
            )
        };
        ASSERT_EQ!(0, unsafe {
            tracefs_extract_field(
                buf,
                REGEX_DENY_PTRACE!(c"ll_trace_test".as_ptr()),
                c"tracee_pid".as_ptr(),
                field.as_mut_ptr(),
                field.len(),
            )
        });
        EXPECT_STREQ!(expected_pid.as_ptr(), field.as_ptr());

        /* Verify tracee_comm matches prctl(PR_SET_NAME). */
        ASSERT_EQ!(0, unsafe {
            tracefs_extract_field(
                buf,
                REGEX_DENY_PTRACE!(c"ll_trace_test".as_ptr()),
                c"tracee_comm".as_ptr(),
                field.as_mut_ptr(),
                field.len(),
            )
        });
        EXPECT_STREQ!(c"ll_trace_test".as_ptr(), field.as_ptr());

        /*
         * Verify tracee_domain: 0 when the tracee is unsandboxed,
         * non-zero when the tracee is in a domain.
         */
        ASSERT_EQ!(0, unsafe {
            tracefs_extract_field(
                buf,
                REGEX_DENY_PTRACE!(c"ll_trace_test".as_ptr()),
                c"tracee_domain".as_ptr(),
                field.as_mut_ptr(),
                field.len(),
            )
        });
        EXPECT_EQ!(variant.sandbox_target, unsafe { strcmp(c"0".as_ptr(), field.as_ptr()) != 0 }, {
            TH_LOG!(c"Unexpected tracee_domain=%s".as_ptr(), field.as_ptr());
        });
    } else {
        EXPECT_EQ!(0, count, {
            TH_LOG!(c"Expected 0 deny_ptrace events, got %d\n%s".as_ptr(), count, buf);
        });
    }

    unsafe { free(buf as *mut c_void) };
});

/* clang-format off */
FIXTURE!(trace_ptrace_traceme, {
    /* clang-format on */
    tracefs_ok: c_int,
});

FIXTURE_SETUP!(trace_ptrace_traceme, {
    let ret: c_int;

    unsafe { set_cap(_metadata, CAP_SYS_ADMIN) };
    ASSERT_EQ!(0, unsafe { unshare(CLONE_NEWNS) });
    ASSERT_EQ!(0, unsafe {
        mount(
            core::ptr::null(),
            c"/".as_ptr(),
            core::ptr::null(),
            MS_REC | MS_PRIVATE,
            core::ptr::null(),
        )
    });

    ret = unsafe { tracefs_fixture_setup() };
    if ret != 0 {
        unsafe { clear_cap(_metadata, CAP_SYS_ADMIN) };
        self_.tracefs_ok = 0;
        SKIP!(return, c"tracefs not available".as_ptr());
    }
    self_.tracefs_ok = 1;

    ASSERT_EQ!(0, unsafe { tracefs_enable_event(TRACEFS_DENY_PTRACE_ENABLE, true) });
    ASSERT_EQ!(0, unsafe { tracefs_clear() });
    unsafe { clear_cap(_metadata, CAP_SYS_ADMIN) };
});

FIXTURE_TEARDOWN!(trace_ptrace_traceme, {
    if self_.tracefs_ok == 0 {
        return;
    }

    unsafe { set_cap(_metadata, CAP_SYS_ADMIN) };
    unsafe { tracefs_enable_event(TRACEFS_DENY_PTRACE_ENABLE, false) };
    unsafe { tracefs_fixture_teardown() };
    unsafe { clear_cap(_metadata, CAP_SYS_ADMIN) };
});

/* clang-format off */
FIXTURE_VARIANT!(trace_ptrace_traceme, {
    /* clang-format on */
    sandbox_tracer: bool,
    sandbox_tracee: bool,
    expect_denied: c_int,
});

/*
 * Denied: a sandboxed tracer cannot trace the unsandboxed child that asked to
 * be traced with PTRACE_TRACEME (tracee_domain=0).
 */
/* clang-format off */
FIXTURE_VARIANT_ADD!(trace_ptrace_traceme, denied, {
    /* clang-format on */
    sandbox_tracer: true,
    sandbox_tracee: false,
    expect_denied: 1,
});

/*
 * Denied: a sandboxed child in its own domain asks to be traced by a tracer in
 * an unrelated domain, so the tracee is in a domain and tracee_domain= is
 * non-zero.
 */
/* clang-format off */
FIXTURE_VARIANT_ADD!(trace_ptrace_traceme, denied_scoped_tracee, {
    /* clang-format on */
    sandbox_tracer: true,
    sandbox_tracee: true,
    expect_denied: 1,
});

/* Allowed: unsandboxed child uses PTRACE_TRACEME with an unsandboxed tracer. */
/* clang-format off */
FIXTURE_VARIANT_ADD!(trace_ptrace_traceme, allowed, {
    /* clang-format on */
    sandbox_tracer: false,
    sandbox_tracee: false,
    expect_denied: 0,
});

TEST_F!(trace_ptrace_traceme, deny_ptrace, {
    let buf: *mut c_char;
    let mut field = [0 as c_char; 64];
    let mut expected_pid = [0 as c_char; 16];
    let count: c_int;
    let mut status: c_int = 0;
    let mut sync_pipe = [0 as c_int; 2];
    let child: pid_t;

    if self_.tracefs_ok == 0 {
        SKIP!(return, c"tracefs not available".as_ptr());
    }

    /*
     * Set a known comm so the denied variant can verify both the trace line
     * task name and the tracee_comm= field.  The tracee is the current
     * (child) task for PTRACE_TRACEME, so the child inherits this name.
     */
    unsafe { prctl(PR_SET_NAME, c"ll_trace_test".as_ptr(), 0, 0, 0) };

    ASSERT_EQ!(0, unsafe { pipe2(sync_pipe.as_mut_ptr(), O_CLOEXEC) });

    child = unsafe { fork() };
    ASSERT_LE!(0, child);

    if child == 0 {
        let mut c: c_char = 0;

        unsafe { close(sync_pipe[1]) };

        /*
         * The tracee is the current task; for the non-zero
         * tracee_domain case it sandboxes itself in its own domain,
         * unrelated to the tracer's domain, so PTRACE_TRACEME is still
         * denied and tracee_domain= names the child's own domain.
         */
        if variant.sandbox_tracee {
            unsafe { create_domain(_metadata) };
        }

        /* Waits for the tracer (parent) to enter its domain, if any. */
        if unsafe { read(sync_pipe[0], &mut c as *mut c_char as *mut c_void, 1) } != 1 {
            unsafe { _exit(1) };
        }
        unsafe { close(sync_pipe[0]) };

        if variant.expect_denied != 0 {
            if unsafe { ptrace(PTRACE_TRACEME, 0, core::ptr::null_mut::<c_void>(), 0) } == 0 {
                unsafe { _exit(2) };
            }
            if errno() != EPERM {
                unsafe { _exit(3) };
            }
        } else {
            if unsafe { ptrace(PTRACE_TRACEME, 0, core::ptr::null_mut::<c_void>(), 0) } != 0 {
                unsafe { _exit(4) };
            }
            /* Lets the tracer reap the trace-stop and detach. */
            unsafe { raise(SIGSTOP) };
        }

        unsafe { _exit(0) };
    }

    unsafe { close(sync_pipe[0]) };

    /*
     * For a denial, the proposed tracer must be in a domain that is not an
     * ancestor of the tracee's domain.  Sandboxing the parent after the
     * fork gives it a domain unrelated to the child.
     */
    if variant.sandbox_tracer {
        unsafe { create_domain(_metadata) };
    }

    /* Signals the child that the tracer is in its domain, if any. */
    ASSERT_EQ!(1, unsafe { write(sync_pipe[1], c".".as_ptr() as *const c_void, 1) });
    unsafe { close(sync_pipe[1]) };

    if variant.expect_denied == 0 {
        /* PTRACE_TRACEME succeeded: reap the SIGSTOP and detach. */
        ASSERT_EQ!(child, unsafe { waitpid(child, &mut status, WUNTRACED) });
        ASSERT_TRUE!(WIFSTOPPED(status));
        ASSERT_EQ!(0, unsafe {
            ptrace(PTRACE_DETACH, child, core::ptr::null_mut::<c_void>(), 0)
        });
    }

    ASSERT_EQ!(child, unsafe { waitpid(child, &mut status, 0) });
    ASSERT_TRUE!(WIFEXITED(status));
    EXPECT_EQ!(0, WEXITSTATUS(status));

    buf = unsafe { tracefs_read_buf() };
    ASSERT_NE!(core::ptr::null_mut::<c_char>(), buf);

    count = unsafe { tracefs_count_matches(buf, REGEX_DENY_PTRACE!(c"ll_trace_test".as_ptr())) };
    if variant.expect_denied != 0 {
        EXPECT_EQ!(variant.expect_denied, count, {
            TH_LOG!(c"Expected deny_ptrace event, got %d\n%s".as_ptr(), count, buf);
        });

        /* Verify tracee_pid is the child's TGID (the traced task). */
        unsafe {
            snprintf(
                expected_pid.as_mut_ptr(),
                expected_pid.len(),
                c"%d".as_ptr(),
                child,
            )
        };
        ASSERT_EQ!(0, unsafe {
            tracefs_extract_field(
                buf,
                REGEX_DENY_PTRACE!(c"ll_trace_test".as_ptr()),
                c"tracee_pid".as_ptr(),
                field.as_mut_ptr(),
                field.len(),
            )
        });
        EXPECT_STREQ!(expected_pid.as_ptr(), field.as_ptr());

        /*
         * Verify tracee_domain: 0 when the tracee is unsandboxed,
         * non-zero when the tracee is in a domain.
         */
        ASSERT_EQ!(0, unsafe {
            tracefs_extract_field(
                buf,
                REGEX_DENY_PTRACE!(c"ll_trace_test".as_ptr()),
                c"tracee_domain".as_ptr(),
                field.as_mut_ptr(),
                field.len(),
            )
        });
        EXPECT_EQ!(variant.sandbox_tracee, unsafe { strcmp(c"0".as_ptr(), field.as_ptr()) != 0 }, {
            TH_LOG!(c"Unexpected tracee_domain=%s".as_ptr(), field.as_ptr());
        });
    } else {
        EXPECT_EQ!(0, count, {
            TH_LOG!(c"Expected 0 deny_ptrace events, got %d\n%s".as_ptr(), count, buf);
        });
    }

    unsafe { free(buf as *mut c_void) };
});

TEST_HARNESS_MAIN!();

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
