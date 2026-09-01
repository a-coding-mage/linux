// SPDX-License-Identifier: GPL-2.0
/*
 * Landlock tests - Signal Scoping
 *
 * Copyright © 2024 Tahera Fahimi <fahimitahera@gmail.com>
 */

// C dependencies removed: errno.h, fcntl.h, linux/landlock.h, pthread.h,
// sched.h, signal.h, sys/mount.h, sys/prctl.h, sys/types.h, sys/wait.h,
// unistd.h, common.h, scoped_common.h, trace.h, scoped_base_variants.h.

const TRACE_TASK: *const i8 = c"scoped_signal_t".as_ptr();

/* This variable is used for handling several signals. */
static mut is_signaled: sig_atomic_t = 0;

FIXTURE!(scoping_signals, {});

#[repr(C)]
struct scoping_signals_variant {
    sig: c_int,
}

FIXTURE_VARIANT_ADD!(scoping_signals, sigtrap, scoping_signals_variant {
    sig: SIGTRAP,
});

FIXTURE_VARIANT_ADD!(scoping_signals, sigurg, scoping_signals_variant {
    sig: SIGURG,
});

FIXTURE_VARIANT_ADD!(scoping_signals, sighup, scoping_signals_variant {
    sig: SIGHUP,
});

FIXTURE_VARIANT_ADD!(scoping_signals, sigtstp, scoping_signals_variant {
    sig: SIGTSTP,
});

unsafe fn scoping_signals_setup(_metadata: *mut __test_metadata) {
    unsafe {
        drop_caps(_metadata);
        is_signaled = 0;
    }
}

unsafe fn scoping_signals_teardown(_metadata: *mut __test_metadata) {}

unsafe extern "C" fn scope_signal_handler(
    sig: c_int,
    _info: *mut siginfo_t,
    _ucontext: *mut c_void,
) {
    unsafe {
        if sig == SIGTRAP || sig == SIGURG || sig == SIGHUP || sig == SIGTSTP {
            is_signaled = 1;
        }
    }
}

/*
 * In this test, a child process sends a signal to parent before and
 * after getting scoped.
 */
unsafe fn scoping_signals_send_sig_to_parent(
    _metadata: *mut __test_metadata,
    variant: *const scoping_signals_variant,
) {
    unsafe {
        let mut pipe_parent: [c_int; 2] = [0; 2];
        let mut status: c_int = 0;
        let mut child: pid_t;
        let parent: pid_t = getpid();
        let mut action: sigaction = core::mem::zeroed();
        action.sa_sigaction = scope_signal_handler as usize;
        action.sa_flags = SA_SIGINFO;

        ASSERT_EQ!(0, pipe2(pipe_parent.as_mut_ptr(), O_CLOEXEC));
        ASSERT_LE!(0, sigaction((*variant).sig, &action, core::ptr::null_mut()));

        /* The process should not have already been signaled. */
        EXPECT_EQ!(0, is_signaled);

        child = fork();
        ASSERT_LE!(0, child);
        if child == 0 {
            let mut buf_child: c_char = 0;
            let mut err: c_int;

            EXPECT_EQ!(0, close(pipe_parent[1]));

            /*
             * The child process can send signal to parent when
             * domain is not scoped.
             */
            err = kill(parent, (*variant).sig);
            ASSERT_EQ!(0, err);
            ASSERT_EQ!(
                1,
                read(pipe_parent[0], &mut buf_child as *mut _ as *mut c_void, 1)
            );
            EXPECT_EQ!(0, close(pipe_parent[0]));

            create_scoped_domain(_metadata, LANDLOCK_SCOPE_SIGNAL);

            /*
             * The child process cannot send signal to the parent
             * anymore.
             */
            err = kill(parent, (*variant).sig);
            ASSERT_EQ!(-1, err);
            ASSERT_EQ!(EPERM, errno());

            /*
             * No matter of the domain, a process should be able to
             * send a signal to itself.
             */
            ASSERT_EQ!(0, is_signaled);
            ASSERT_EQ!(0, raise((*variant).sig));
            ASSERT_EQ!(1, is_signaled);

            _exit((*_metadata).exit_code);
            return;
        }
        EXPECT_EQ!(0, close(pipe_parent[0]));

        /* Waits for a first signal to be received, without race condition. */
        while is_signaled == 0 && usleep(1) == 0 {}
        ASSERT_EQ!(1, is_signaled);
        ASSERT_EQ!(1, write(pipe_parent[1], c".".as_ptr() as *const c_void, 1));
        EXPECT_EQ!(0, close(pipe_parent[1]));
        is_signaled = 0;

        ASSERT_EQ!(child, waitpid(child, &mut status, 0));
        if WIFSIGNALED(status) || !WIFEXITED(status) || WEXITSTATUS(status) != EXIT_SUCCESS {
            (*_metadata).exit_code = KSFT_FAIL;
        }

        EXPECT_EQ!(0, is_signaled);
    }
}

FIXTURE!(scoped_domains, {});

// Variants supplied by scoped_base_variants.h in the original C source.

unsafe fn scoped_domains_setup(_metadata: *mut __test_metadata) {
    unsafe {
        drop_caps(_metadata);
    }
}

unsafe fn scoped_domains_teardown(_metadata: *mut __test_metadata) {}

/*
 * This test ensures that a scoped process cannot send signal out of
 * scoped domain.
 */
unsafe fn scoped_domains_check_access_signal(
    _metadata: *mut __test_metadata,
    variant: *const scoped_domains_variant,
) {
    unsafe {
        let mut child: pid_t;
        let parent: pid_t = getpid();
        let mut status: c_int = 0;
        let can_signal_child: bool;
        let can_signal_parent: bool;
        let mut pipe_parent: [c_int; 2] = [0; 2];
        let mut pipe_child: [c_int; 2] = [0; 2];
        let mut buf_parent: c_char = 0;
        let mut err: c_int;

        can_signal_parent = !(*variant).domain_child;
        can_signal_child = !(*variant).domain_parent;

        if (*variant).domain_both {
            create_scoped_domain(_metadata, LANDLOCK_SCOPE_SIGNAL);
        }

        ASSERT_EQ!(0, pipe2(pipe_parent.as_mut_ptr(), O_CLOEXEC));
        ASSERT_EQ!(0, pipe2(pipe_child.as_mut_ptr(), O_CLOEXEC));

        child = fork();
        ASSERT_LE!(0, child);
        if child == 0 {
            let mut buf_child: c_char = 0;

            EXPECT_EQ!(0, close(pipe_child[0]));
            EXPECT_EQ!(0, close(pipe_parent[1]));

            if (*variant).domain_child {
                create_scoped_domain(_metadata, LANDLOCK_SCOPE_SIGNAL);
            }

            ASSERT_EQ!(1, write(pipe_child[1], c".".as_ptr() as *const c_void, 1));
            EXPECT_EQ!(0, close(pipe_child[1]));

            /* Waits for the parent to send signals. */
            ASSERT_EQ!(
                1,
                read(pipe_parent[0], &mut buf_child as *mut _ as *mut c_void, 1)
            );
            EXPECT_EQ!(0, close(pipe_parent[0]));

            err = kill(parent, 0);
            if can_signal_parent {
                ASSERT_EQ!(0, err);
            } else {
                ASSERT_EQ!(-1, err);
                ASSERT_EQ!(EPERM, errno());
            }
            /*
             * No matter of the domain, a process should be able to
             * send a signal to itself.
             */
            ASSERT_EQ!(0, raise(0));

            _exit((*_metadata).exit_code);
            return;
        }
        EXPECT_EQ!(0, close(pipe_parent[0]));
        EXPECT_EQ!(0, close(pipe_child[1]));

        if (*variant).domain_parent {
            create_scoped_domain(_metadata, LANDLOCK_SCOPE_SIGNAL);
        }

        ASSERT_EQ!(
            1,
            read(pipe_child[0], &mut buf_parent as *mut _ as *mut c_void, 1)
        );
        EXPECT_EQ!(0, close(pipe_child[0]));

        err = kill(child, 0);
        if can_signal_child {
            ASSERT_EQ!(0, err);
        } else {
            ASSERT_EQ!(-1, err);
            ASSERT_EQ!(EPERM, errno());
        }
        ASSERT_EQ!(0, raise(0));

        ASSERT_EQ!(1, write(pipe_parent[1], c".".as_ptr() as *const c_void, 1));
        EXPECT_EQ!(0, close(pipe_parent[1]));
        ASSERT_EQ!(child, waitpid(child, &mut status, 0));

        if WIFSIGNALED(status) || !WIFEXITED(status) || WEXITSTATUS(status) != EXIT_SUCCESS {
            (*_metadata).exit_code = KSFT_FAIL;
        }
    }
}

const THREAD_INVALID: *mut c_void = 0 as *mut c_void;
const THREAD_SUCCESS: *mut c_void = 1 as *mut c_void;
const THREAD_ERROR: *mut c_void = 2 as *mut c_void;
const THREAD_TEST_FAILED: *mut c_void = 3 as *mut c_void;

unsafe extern "C" fn thread_sync(arg: *mut c_void) -> *mut c_void {
    unsafe {
        let pipe_read: c_int = *(arg as *mut c_int);
        let mut buf: c_char = 0;

        if read(pipe_read, &mut buf as *mut _ as *mut c_void, 1) != 1 {
            return THREAD_ERROR;
        }

        THREAD_SUCCESS
    }
}

unsafe fn signal_scoping_thread_before(_metadata: *mut __test_metadata) {
    unsafe {
        let mut no_sandbox_thread: pthread_t = core::mem::zeroed();
        let mut ret: *mut c_void = THREAD_INVALID;
        let mut thread_pipe: [c_int; 2] = [0; 2];

        drop_caps(_metadata);
        ASSERT_EQ!(0, pipe2(thread_pipe.as_mut_ptr(), O_CLOEXEC));

        ASSERT_EQ!(
            0,
            pthread_create(
                &mut no_sandbox_thread,
                core::ptr::null(),
                Some(thread_sync),
                &mut thread_pipe[0] as *mut _ as *mut c_void,
            )
        );

        /* Enforces restriction after creating the thread. */
        create_scoped_domain(_metadata, LANDLOCK_SCOPE_SIGNAL);

        EXPECT_EQ!(0, pthread_kill(no_sandbox_thread, 0));
        EXPECT_EQ!(1, write(thread_pipe[1], c".".as_ptr() as *const c_void, 1));

        EXPECT_EQ!(0, pthread_join(no_sandbox_thread, &mut ret));
        EXPECT_EQ!(THREAD_SUCCESS, ret);

        EXPECT_EQ!(0, close(thread_pipe[0]));
        EXPECT_EQ!(0, close(thread_pipe[1]));
    }
}

unsafe fn signal_scoping_thread_after(_metadata: *mut __test_metadata) {
    unsafe {
        let mut scoped_thread: pthread_t = core::mem::zeroed();
        let mut ret: *mut c_void = THREAD_INVALID;
        let mut thread_pipe: [c_int; 2] = [0; 2];

        drop_caps(_metadata);
        ASSERT_EQ!(0, pipe2(thread_pipe.as_mut_ptr(), O_CLOEXEC));

        /* Enforces restriction before creating the thread. */
        create_scoped_domain(_metadata, LANDLOCK_SCOPE_SIGNAL);

        ASSERT_EQ!(
            0,
            pthread_create(
                &mut scoped_thread,
                core::ptr::null(),
                Some(thread_sync),
                &mut thread_pipe[0] as *mut _ as *mut c_void,
            )
        );

        EXPECT_EQ!(0, pthread_kill(scoped_thread, 0));
        EXPECT_EQ!(1, write(thread_pipe[1], c".".as_ptr() as *const c_void, 1));

        EXPECT_EQ!(0, pthread_join(scoped_thread, &mut ret));
        EXPECT_EQ!(THREAD_SUCCESS, ret);

        EXPECT_EQ!(0, close(thread_pipe[0]));
        EXPECT_EQ!(0, close(thread_pipe[1]));
    }
}

#[repr(C)]
struct thread_setuid_args {
    pipe_read: c_int,
    new_uid: c_int,
}

unsafe extern "C" fn thread_setuid(ptr: *mut c_void) -> *mut c_void {
    unsafe {
        let arg: *const thread_setuid_args = ptr as *const thread_setuid_args;
        let mut buf: c_char = 0;

        if read((*arg).pipe_read, &mut buf as *mut _ as *mut c_void, 1) != 1 {
            return THREAD_ERROR;
        }

        /* libc's setuid() should update all thread's credentials. */
        if getuid() != (*arg).new_uid as uid_t {
            return THREAD_TEST_FAILED;
        }

        THREAD_SUCCESS
    }
}

unsafe fn signal_scoping_thread_setuid(_metadata: *mut __test_metadata) {
    unsafe {
        let mut arg: thread_setuid_args = core::mem::zeroed();
        let mut no_sandbox_thread: pthread_t = core::mem::zeroed();
        let mut ret: *mut c_void = THREAD_INVALID;
        let mut pipe_parent: [c_int; 2] = [0; 2];
        let prev_uid: c_int;

        disable_caps(_metadata);

        /* This test does not need to be run as root. */
        prev_uid = getuid() as c_int;
        arg.new_uid = prev_uid + 1;
        EXPECT_LT!(0, arg.new_uid);

        ASSERT_EQ!(0, pipe2(pipe_parent.as_mut_ptr(), O_CLOEXEC));
        arg.pipe_read = pipe_parent[0];

        /* Capabilities must be set before creating a new thread. */
        set_cap(_metadata, CAP_SETUID);
        ASSERT_EQ!(
            0,
            pthread_create(
                &mut no_sandbox_thread,
                core::ptr::null(),
                Some(thread_setuid),
                &mut arg as *mut _ as *mut c_void,
            )
        );

        /* Enforces restriction after creating the thread. */
        create_scoped_domain(_metadata, LANDLOCK_SCOPE_SIGNAL);

        EXPECT_NE!(arg.new_uid as uid_t, getuid());
        EXPECT_EQ!(0, setuid(arg.new_uid as uid_t));
        EXPECT_EQ!(arg.new_uid as uid_t, getuid());
        EXPECT_EQ!(1, write(pipe_parent[1], c".".as_ptr() as *const c_void, 1));

        EXPECT_EQ!(0, pthread_join(no_sandbox_thread, &mut ret));
        EXPECT_EQ!(THREAD_SUCCESS, ret);

        clear_cap(_metadata, CAP_SETUID);
        EXPECT_EQ!(0, close(pipe_parent[0]));
        EXPECT_EQ!(0, close(pipe_parent[1]));
    }
}

const backlog: c_short = 10;

static mut signal_received: sig_atomic_t = 0;

unsafe extern "C" fn handle_sigurg(sig: c_int) {
    unsafe {
        if sig == SIGURG {
            signal_received = 1;
        } else {
            signal_received = -1;
        }
    }
}

unsafe fn setup_signal_handler(_signal: c_int) -> c_int {
    unsafe {
        let mut sa: sigaction = core::mem::zeroed();
        sa.sa_sigaction = handle_sigurg as usize;

        if sigemptyset(&mut sa.sa_mask) != 0 {
            return -1;
        }

        sa.sa_flags = SA_SIGINFO | SA_RESTART;
        sigaction(SIGURG, &sa, core::ptr::null_mut())
    }
}

/*
 * MSG_OOB might be disabled in the kernel via the CONFIG_AF_UNIX_OOB
 * switch, so this function can be used for probing for its availability.
 */
unsafe fn has_af_unix_oob() -> bool {
    unsafe {
        let mut available: bool = false;
        let mut sp: [c_int; 2] = [0; 2];

        if socketpair(AF_UNIX, SOCK_STREAM, 0, sp.as_mut_ptr()) == 0 {
            available = send(sp[0], c".".as_ptr() as *const c_void, 1, MSG_OOB) == 1;
            close(sp[0]);
            close(sp[1]);
        }

        available
    }
}

FIXTURE!(fown, {});

#[repr(C)]
enum fown_sandbox {
    SANDBOX_NONE,
    SANDBOX_BEFORE_FORK,
    SANDBOX_BEFORE_SETOWN,
    SANDBOX_AFTER_SETOWN,
}

#[repr(C)]
struct fown_variant {
    sandbox_setown: fown_sandbox,
}

FIXTURE_VARIANT_ADD!(fown, no_sandbox, fown_variant {
    sandbox_setown: fown_sandbox::SANDBOX_NONE,
});

FIXTURE_VARIANT_ADD!(fown, sandbox_before_fork, fown_variant {
    sandbox_setown: fown_sandbox::SANDBOX_BEFORE_FORK,
});

FIXTURE_VARIANT_ADD!(fown, sandbox_before_setown, fown_variant {
    sandbox_setown: fown_sandbox::SANDBOX_BEFORE_SETOWN,
});

FIXTURE_VARIANT_ADD!(fown, sandbox_after_setown, fown_variant {
    sandbox_setown: fown_sandbox::SANDBOX_AFTER_SETOWN,
});

unsafe fn fown_setup(_metadata: *mut __test_metadata) {
    unsafe {
        drop_caps(_metadata);
    }
}

unsafe fn fown_teardown(_metadata: *mut __test_metadata) {}

// The following test functions continue the C source-level translation. They
// intentionally preserve external test harness calls and libc/Linux symbols.

unsafe fn fown_sigurg_socket(_metadata: *mut __test_metadata, variant: *const fown_variant) {
    unsafe {
        let mut server_socket: c_int;
        let mut recv_socket: c_int;
        let mut server_address: service_fixture = core::mem::zeroed();
        let mut buffer_parent: c_char = 0;
        let mut status: c_int = 0;
        let mut pipe_parent: [c_int; 2] = [0; 2];
        let mut pipe_child: [c_int; 2] = [0; 2];
        let mut child: pid_t;

        if !has_af_unix_oob() {
            SKIP!(return, "CONFIG_AF_UNIX_OOB / MSG_OOB not available");
        }

        memset(
            &mut server_address as *mut _ as *mut c_void,
            0,
            core::mem::size_of::<service_fixture>(),
        );
        set_unix_address(&mut server_address, 0);

        ASSERT_EQ!(0, pipe2(pipe_parent.as_mut_ptr(), O_CLOEXEC));
        ASSERT_EQ!(0, pipe2(pipe_child.as_mut_ptr(), O_CLOEXEC));

        if matches!(
            (*variant).sandbox_setown,
            fown_sandbox::SANDBOX_BEFORE_FORK
        ) {
            create_scoped_domain(_metadata, LANDLOCK_SCOPE_SIGNAL);
        }

        child = fork();
        ASSERT_LE!(0, child);
        if child == 0 {
            let mut client_socket: c_int;
            let mut buffer_child: c_char = 0;

            EXPECT_EQ!(0, close(pipe_parent[1]));
            EXPECT_EQ!(0, close(pipe_child[0]));

            ASSERT_EQ!(0, setup_signal_handler(SIGURG));
            client_socket = socket(AF_UNIX, SOCK_STREAM, 0);
            ASSERT_LE!(0, client_socket);

            /* Waits for the parent to listen. */
            ASSERT_EQ!(
                1,
                read(pipe_parent[0], &mut buffer_child as *mut _ as *mut c_void, 1)
            );
            ASSERT_EQ!(
                0,
                connect(
                    client_socket,
                    &mut server_address.unix_addr as *mut _ as *mut sockaddr,
                    server_address.unix_addr_len,
                )
            );

            /*
             * Waits for the parent to accept the connection, sandbox
             * itself, and call fcntl(2).
             */
            ASSERT_EQ!(
                1,
                read(pipe_parent[0], &mut buffer_child as *mut _ as *mut c_void, 1)
            );
            /* May signal itself. */
            ASSERT_EQ!(
                1,
                send(client_socket, c".".as_ptr() as *const c_void, 1, MSG_OOB)
            );
            EXPECT_EQ!(0, close(client_socket));
            ASSERT_EQ!(1, write(pipe_child[1], c".".as_ptr() as *const c_void, 1));
            EXPECT_EQ!(0, close(pipe_child[1]));

            /* Waits for the message to be received. */
            ASSERT_EQ!(
                1,
                read(pipe_parent[0], &mut buffer_child as *mut _ as *mut c_void, 1)
            );
            EXPECT_EQ!(0, close(pipe_parent[0]));

            if matches!(
                (*variant).sandbox_setown,
                fown_sandbox::SANDBOX_BEFORE_SETOWN
            ) {
                ASSERT_EQ!(0, signal_received);
            } else {
                /*
                 * A signal is only received if fcntl(F_SETOWN) was
                 * called before any sandboxing or if the signal
                 * receiver is in the same domain.
                 */
                ASSERT_EQ!(1, signal_received);
            }
            _exit((*_metadata).exit_code);
            return;
        }
        EXPECT_EQ!(0, close(pipe_parent[0]));
        EXPECT_EQ!(0, close(pipe_child[1]));

        server_socket = socket(AF_UNIX, SOCK_STREAM, 0);
        ASSERT_LE!(0, server_socket);
        ASSERT_EQ!(
            0,
            bind(
                server_socket,
                &mut server_address.unix_addr as *mut _ as *mut sockaddr,
                server_address.unix_addr_len,
            )
        );
        ASSERT_EQ!(0, listen(server_socket, backlog as c_int));
        ASSERT_EQ!(1, write(pipe_parent[1], c".".as_ptr() as *const c_void, 1));

        recv_socket = accept(server_socket, core::ptr::null_mut(), core::ptr::null_mut());
        ASSERT_LE!(0, recv_socket);

        if matches!(
            (*variant).sandbox_setown,
            fown_sandbox::SANDBOX_BEFORE_SETOWN
        ) {
            create_scoped_domain(_metadata, LANDLOCK_SCOPE_SIGNAL);
        }

        /*
         * Sets the child to receive SIGURG for MSG_OOB.  This uncommon use is
         * a valid attack scenario which also simplifies this test.
         */
        ASSERT_EQ!(0, fcntl(recv_socket, F_SETOWN, child));

        if matches!(
            (*variant).sandbox_setown,
            fown_sandbox::SANDBOX_AFTER_SETOWN
        ) {
            create_scoped_domain(_metadata, LANDLOCK_SCOPE_SIGNAL);
        }

        ASSERT_EQ!(1, write(pipe_parent[1], c".".as_ptr() as *const c_void, 1));

        /* Waits for the child to send MSG_OOB. */
        ASSERT_EQ!(
            1,
            read(pipe_child[0], &mut buffer_parent as *mut _ as *mut c_void, 1)
        );
        EXPECT_EQ!(0, close(pipe_child[0]));
        ASSERT_EQ!(
            1,
            recv(
                recv_socket,
                &mut buffer_parent as *mut _ as *mut c_void,
                1,
                MSG_OOB,
            )
        );
        EXPECT_EQ!(0, close(recv_socket));
        EXPECT_EQ!(0, close(server_socket));
        ASSERT_EQ!(1, write(pipe_parent[1], c".".as_ptr() as *const c_void, 1));
        EXPECT_EQ!(0, close(pipe_parent[1]));

        ASSERT_EQ!(child, waitpid(child, &mut status, 0));
        if WIFSIGNALED(status) || !WIFEXITED(status) || WEXITSTATUS(status) != EXIT_SUCCESS {
            (*_metadata).exit_code = KSFT_FAIL;
        }
    }
}

// Remaining TEST bodies from the C file are represented with literal Rust
// function boundaries and preserved comments; their statements rely on the same
// external kernel selftest harness symbols as above.

unsafe fn sigio_to_pgid_members(_metadata: *mut __test_metadata) {
    unsafe {
        let mut trigger: [c_int; 2] = [0; 2];
        let mut sync_child: [c_int; 2] = [0; 2];
        let mut buf: c_char = 0;
        let mut child: pid_t;
        let mut status: c_int = 0;
        let mut i: c_int;

        drop_caps(_metadata);
        ASSERT_EQ!(0, setpgid(0, 0));
        ASSERT_EQ!(0, setup_signal_handler(SIGURG));
        signal_received = 0;
        ASSERT_EQ!(0, pipe2(trigger.as_mut_ptr(), O_CLOEXEC));
        ASSERT_EQ!(0, pipe2(sync_child.as_mut_ptr(), O_CLOEXEC));

        child = fork();
        ASSERT_LE!(0, child);
        if child == 0 {
            EXPECT_EQ!(0, close(sync_child[0]));
            ASSERT_EQ!(0, setup_signal_handler(SIGURG));
            signal_received = 0;
            create_scoped_domain(_metadata, LANDLOCK_SCOPE_SIGNAL);
            ASSERT_EQ!(0, fcntl(trigger[0], F_SETSIG, SIGURG));
            ASSERT_EQ!(0, fcntl(trigger[0], F_SETOWN, -getpgrp()));
            ASSERT_EQ!(0, fcntl(trigger[0], F_SETFL, O_ASYNC));
            ASSERT_EQ!(1, write(trigger[1], c".".as_ptr() as *const c_void, 1));

            i = 0;
            while i < 1000 && signal_received == 0 {
                usleep(1000);
                i += 1;
            }
            EXPECT_EQ!(1, signal_received);

            ASSERT_EQ!(1, write(sync_child[1], c".".as_ptr() as *const c_void, 1));
            EXPECT_EQ!(0, close(sync_child[1]));
            _exit((*_metadata).exit_code);
            return;
        }
        EXPECT_EQ!(0, close(sync_child[1]));
        EXPECT_EQ!(0, close(trigger[0]));
        EXPECT_EQ!(0, close(trigger[1]));
        ASSERT_EQ!(1, read(sync_child[0], &mut buf as *mut _ as *mut c_void, 1));
        EXPECT_EQ!(0, close(sync_child[0]));

        i = 0;
        while i < 100 && signal_received == 0 {
            usleep(1000);
            i += 1;
        }

        EXPECT_EQ!(0, signal_received);
        ASSERT_EQ!(child, waitpid(child, &mut status, 0));
        if WIFSIGNALED(status) || !WIFEXITED(status) || WEXITSTATUS(status) != EXIT_SUCCESS {
            (*_metadata).exit_code = KSFT_FAIL;
        }
    }
}

unsafe extern "C" fn thread_setown_scoped(arg: *mut c_void) -> *mut c_void {
    unsafe {
        let fd: c_int = *(arg as *mut c_int);
        let mut ruleset_fd: c_int;
        let ruleset_attr = landlock_ruleset_attr {
            scoped: LANDLOCK_SCOPE_SIGNAL,
        };

        /* Sandboxes only this non-leader thread (no thread syncing). */
        ruleset_fd = landlock_create_ruleset(
            &ruleset_attr,
            core::mem::size_of::<landlock_ruleset_attr>(),
            0,
        );
        if ruleset_fd < 0 {
            return THREAD_ERROR;
        }
        if prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) != 0
            || landlock_restrict_self(ruleset_fd, 0) != 0
        {
            close(ruleset_fd);
            return THREAD_ERROR;
        }
        close(ruleset_fd);

        /* Makes this process group own the SIGIO source. */
        if fcntl(fd, F_SETSIG, SIGURG) != 0
            || fcntl(fd, F_SETOWN, -getpgrp()) != 0
            || fcntl(fd, F_SETFL, O_ASYNC) != 0
        {
            return THREAD_ERROR;
        }

        THREAD_SUCCESS
    }
}

unsafe fn sigio_to_pgid_self(_metadata: *mut __test_metadata) {
    unsafe {
        let mut trigger: [c_int; 2] = [0; 2];
        let mut thread: pthread_t = core::mem::zeroed();
        let mut ret: *mut c_void = THREAD_INVALID;
        let mut i: c_int;

        drop_caps(_metadata);
        ASSERT_EQ!(0, setpgid(0, 0));
        ASSERT_EQ!(0, setup_signal_handler(SIGURG));
        signal_received = 0;
        ASSERT_EQ!(0, pipe2(trigger.as_mut_ptr(), O_CLOEXEC));
        ASSERT_EQ!(
            0,
            pthread_create(
                &mut thread,
                core::ptr::null(),
                Some(thread_setown_scoped),
                &mut trigger[0] as *mut _ as *mut c_void,
            )
        );
        ASSERT_EQ!(0, pthread_join(thread, &mut ret));
        ASSERT_EQ!(THREAD_SUCCESS, ret);
        ASSERT_EQ!(1, write(trigger[1], c".".as_ptr() as *const c_void, 1));

        i = 0;
        while i < 1000 && signal_received == 0 {
            usleep(1000);
            i += 1;
        }

        EXPECT_EQ!(1, signal_received);
        EXPECT_EQ!(0, close(trigger[0]));
        EXPECT_EQ!(0, close(trigger[1]));
    }
}

/* Trace tests */

#[repr(C)]
struct trace_signal {
    tracefs_ok: c_int,
}

#[repr(C)]
struct trace_signal_variant {
    sandbox: bool,
    sandbox_target: bool,
    expect_denied: c_int,
}

FIXTURE_VARIANT_ADD!(trace_signal, denied, trace_signal_variant {
    sandbox: true,
    sandbox_target: false,
    expect_denied: 1,
});

FIXTURE_VARIANT_ADD!(trace_signal, denied_scoped_target, trace_signal_variant {
    sandbox: true,
    sandbox_target: true,
    expect_denied: 1,
});

FIXTURE_VARIANT_ADD!(trace_signal, allowed, trace_signal_variant {
    sandbox: false,
    sandbox_target: false,
    expect_denied: 0,
});

unsafe fn trace_signal_setup(_metadata: *mut __test_metadata, self_: *mut trace_signal) {
    unsafe {
        let ret: c_int;

        set_cap(_metadata, CAP_SYS_ADMIN);
        ASSERT_EQ!(0, unshare(CLONE_NEWNS));
        ASSERT_EQ!(0, mount(core::ptr::null(), c"/".as_ptr(), core::ptr::null(), MS_REC | MS_PRIVATE, core::ptr::null()));

        ret = tracefs_fixture_setup();
        if ret != 0 {
            clear_cap(_metadata, CAP_SYS_ADMIN);
            (*self_).tracefs_ok = 0;
            SKIP!(return, "tracefs not available");
        }
        (*self_).tracefs_ok = 1;

        ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_DENY_SCOPE_SIGNAL_ENABLE, true));
        ASSERT_EQ!(0, tracefs_clear());
        clear_cap(_metadata, CAP_SYS_ADMIN);
    }
}

unsafe fn trace_signal_teardown(_metadata: *mut __test_metadata, self_: *mut trace_signal) {
    unsafe {
        if (*self_).tracefs_ok == 0 {
            return;
        }

        set_cap(_metadata, CAP_SYS_ADMIN);
        tracefs_enable_event(TRACEFS_DENY_SCOPE_SIGNAL_ENABLE, false);
        tracefs_fixture_teardown();
        clear_cap(_metadata, CAP_SYS_ADMIN);
    }
}

unsafe fn trace_signal_deny_scope_signal(
    _metadata: *mut __test_metadata,
    self_: *mut trace_signal,
    variant: *const trace_signal_variant,
) {
    unsafe {
        let mut buf: *mut c_char;
        let mut field: [c_char; 64] = [0; 64];
        let mut expected_pid: [c_char; 16] = [0; 16];
        let mut count: c_int;
        let mut status: c_int = 0;
        let mut child: pid_t;

        if (*self_).tracefs_ok == 0 {
            SKIP!(return, "tracefs not available");
        }

        if (*variant).sandbox_target {
            create_scoped_domain(_metadata, LANDLOCK_SCOPE_SIGNAL);
        }

        child = fork();
        ASSERT_LE!(0, child);

        if child == 0 {
            if (*variant).sandbox {
                let ruleset_attr = landlock_ruleset_attr {
                    scoped: LANDLOCK_SCOPE_SIGNAL,
                };
                let ruleset_fd: c_int;

                ruleset_fd = landlock_create_ruleset(
                    &ruleset_attr,
                    core::mem::size_of::<landlock_ruleset_attr>(),
                    0,
                );
                if ruleset_fd < 0 {
                    _exit(1);
                }

                prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0);
                if landlock_restrict_self(ruleset_fd, 0) != 0 {
                    close(ruleset_fd);
                    _exit(1);
                }
                close(ruleset_fd);
            }

            if (*variant).sandbox {
                /* Signal to unsandboxed parent should be denied. */
                if kill(getppid(), 0) == 0 {
                    _exit(2);
                }
                if errno() != EPERM {
                    _exit(3);
                }
            } else {
                /* No sandbox: kill should succeed. */
                if kill(getppid(), 0) != 0 {
                    _exit(1);
                }
            }

            _exit(0);
        }

        ASSERT_EQ!(child, waitpid(child, &mut status, 0));
        ASSERT_TRUE!(WIFEXITED(status));
        EXPECT_EQ!(0, WEXITSTATUS(status));

        buf = tracefs_read_buf();
        ASSERT_NE!(core::ptr::null_mut::<c_char>(), buf);

        count = tracefs_count_matches(buf, REGEX_DENY_SCOPE_SIGNAL!(TRACE_TASK));
        if (*variant).expect_denied != 0 {
            EXPECT_EQ!((*variant).expect_denied, count, {
                TH_LOG!("Expected deny_scope_signal event, got %d\n%s", count, buf);
            });

            /* Verify target_pid is the parent's PID. */
            snprintf(
                expected_pid.as_mut_ptr(),
                expected_pid.len(),
                c"%d".as_ptr(),
                getpid(),
            );
            ASSERT_EQ!(
                0,
                tracefs_extract_field(
                    buf,
                    REGEX_DENY_SCOPE_SIGNAL!(TRACE_TASK),
                    c"target_pid".as_ptr(),
                    field.as_mut_ptr(),
                    field.len(),
                )
            );
            EXPECT_STREQ!(expected_pid.as_ptr(), field.as_ptr());

            /*
             * Verify target_domain: 0 when the target is unsandboxed,
             * non-zero when the target is in a domain.
             */
            ASSERT_EQ!(
                0,
                tracefs_extract_field(
                    buf,
                    REGEX_DENY_SCOPE_SIGNAL!(TRACE_TASK),
                    c"target_domain".as_ptr(),
                    field.as_mut_ptr(),
                    field.len(),
                )
            );
            EXPECT_EQ!((*variant).sandbox_target, strcmp(c"0".as_ptr(), field.as_ptr()) != 0, {
                TH_LOG!("Unexpected target_domain=%s", field.as_ptr());
            });
        } else {
            EXPECT_EQ!(0, count, {
                TH_LOG!("Expected 0 deny_scope_signal events, got %d\n%s", count, buf);
            });
        }

        free(buf as *mut c_void);
    }
}

#[repr(C)]
struct trace_fown {
    tracefs_ok: c_int,
}

#[repr(C)]
struct trace_fown_variant {
    sandbox: bool,
    sandbox_target: bool,
    expect_denied: c_int,
}

FIXTURE_VARIANT_ADD!(trace_fown, denied, trace_fown_variant {
    sandbox: true,
    sandbox_target: false,
    expect_denied: 1,
});

FIXTURE_VARIANT_ADD!(trace_fown, denied_scoped_target, trace_fown_variant {
    sandbox: true,
    sandbox_target: true,
    expect_denied: 1,
});

FIXTURE_VARIANT_ADD!(trace_fown, allowed, trace_fown_variant {
    sandbox: false,
    sandbox_target: false,
    expect_denied: 0,
});

unsafe fn trace_fown_setup(_metadata: *mut __test_metadata, self_: *mut trace_fown) {
    unsafe {
        let ret: c_int;

        set_cap(_metadata, CAP_SYS_ADMIN);
        ASSERT_EQ!(0, unshare(CLONE_NEWNS));
        ASSERT_EQ!(0, mount(core::ptr::null(), c"/".as_ptr(), core::ptr::null(), MS_REC | MS_PRIVATE, core::ptr::null()));

        ret = tracefs_fixture_setup();
        if ret != 0 {
            clear_cap(_metadata, CAP_SYS_ADMIN);
            (*self_).tracefs_ok = 0;
            SKIP!(return, "tracefs not available");
        }
        (*self_).tracefs_ok = 1;

        ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_DENY_SCOPE_SIGNAL_ENABLE, true));
        ASSERT_EQ!(0, tracefs_clear());
        clear_cap(_metadata, CAP_SYS_ADMIN);
    }
}

unsafe fn trace_fown_teardown(_metadata: *mut __test_metadata, self_: *mut trace_fown) {
    unsafe {
        if (*self_).tracefs_ok == 0 {
            return;
        }

        set_cap(_metadata, CAP_SYS_ADMIN);
        tracefs_enable_event(TRACEFS_DENY_SCOPE_SIGNAL_ENABLE, false);
        tracefs_fixture_teardown();
        clear_cap(_metadata, CAP_SYS_ADMIN);
    }
}

unsafe fn trace_fown_deny_scope_fown(
    _metadata: *mut __test_metadata,
    self_: *mut trace_fown,
    variant: *const trace_fown_variant,
) {
    unsafe {
        let mut server_socket: c_int;
        let mut recv_socket: c_int;
        let mut server_address: service_fixture = core::mem::zeroed();
        let mut buffer_parent: c_char = 0;
        let mut field: [c_char; 64] = [0; 64];
        let mut buf: *mut c_char;
        let mut status: c_int = 0;
        let mut count: c_int;
        let mut pipe_parent: [c_int; 2] = [0; 2];
        let mut pipe_child: [c_int; 2] = [0; 2];
        let mut child: pid_t;

        if (*self_).tracefs_ok == 0 {
            SKIP!(return, "tracefs not available");
        }

        memset(
            &mut server_address as *mut _ as *mut c_void,
            0,
            core::mem::size_of::<service_fixture>(),
        );
        set_unix_address(&mut server_address, 0);

        ASSERT_EQ!(0, pipe2(pipe_parent.as_mut_ptr(), O_CLOEXEC));
        ASSERT_EQ!(0, pipe2(pipe_child.as_mut_ptr(), O_CLOEXEC));

        child = fork();
        ASSERT_LE!(0, child);
        if child == 0 {
            let mut client_socket: c_int;
            let mut buffer_child: c_char = 0;

            EXPECT_EQ!(0, close(pipe_parent[1]));
            EXPECT_EQ!(0, close(pipe_child[0]));

            ASSERT_EQ!(0, setup_signal_handler(SIGURG));
            client_socket = socket(AF_UNIX, SOCK_STREAM, 0);
            ASSERT_LE!(0, client_socket);

            if (*variant).sandbox_target {
                create_scoped_domain(_metadata, LANDLOCK_SCOPE_SIGNAL);
            }

            ASSERT_EQ!(1, read(pipe_parent[0], &mut buffer_child as *mut _ as *mut c_void, 1));
            ASSERT_EQ!(
                0,
                connect(
                    client_socket,
                    &mut server_address.unix_addr as *mut _ as *mut sockaddr,
                    server_address.unix_addr_len,
                )
            );
            ASSERT_EQ!(1, read(pipe_parent[0], &mut buffer_child as *mut _ as *mut c_void, 1));
            ASSERT_EQ!(1, send(client_socket, c".".as_ptr() as *const c_void, 1, MSG_OOB));
            EXPECT_EQ!(0, close(client_socket));
            ASSERT_EQ!(1, write(pipe_child[1], c".".as_ptr() as *const c_void, 1));
            EXPECT_EQ!(0, close(pipe_child[1]));

            _exit(0);
            return;
        }
        EXPECT_EQ!(0, close(pipe_parent[0]));
        EXPECT_EQ!(0, close(pipe_child[1]));

        server_socket = socket(AF_UNIX, SOCK_STREAM, 0);
        ASSERT_LE!(0, server_socket);
        ASSERT_EQ!(
            0,
            bind(
                server_socket,
                &mut server_address.unix_addr as *mut _ as *mut sockaddr,
                server_address.unix_addr_len,
            )
        );
        ASSERT_EQ!(0, listen(server_socket, backlog as c_int));
        ASSERT_EQ!(1, write(pipe_parent[1], c".".as_ptr() as *const c_void, 1));

        recv_socket = accept(server_socket, core::ptr::null_mut(), core::ptr::null_mut());
        ASSERT_LE!(0, recv_socket);

        if (*variant).sandbox {
            create_scoped_domain(_metadata, LANDLOCK_SCOPE_SIGNAL);
        }

        ASSERT_EQ!(0, fcntl(recv_socket, F_SETOWN, child));
        ASSERT_EQ!(1, write(pipe_parent[1], c".".as_ptr() as *const c_void, 1));
        ASSERT_EQ!(1, read(pipe_child[0], &mut buffer_parent as *mut _ as *mut c_void, 1));
        EXPECT_EQ!(0, close(pipe_child[0]));
        ASSERT_EQ!(1, recv(recv_socket, &mut buffer_parent as *mut _ as *mut c_void, 1, MSG_OOB));
        EXPECT_EQ!(0, close(recv_socket));
        EXPECT_EQ!(0, close(server_socket));

        ASSERT_EQ!(child, waitpid(child, &mut status, 0));
        ASSERT_TRUE!(WIFEXITED(status));
        EXPECT_EQ!(0, WEXITSTATUS(status));

        buf = tracefs_read_buf();
        ASSERT_NE!(core::ptr::null_mut::<c_char>(), buf);

        count = tracefs_count_matches(buf, REGEX_DENY_SCOPE_SIGNAL!(TRACE_TASK));
        if (*variant).expect_denied != 0 {
            EXPECT_EQ!((*variant).expect_denied, count, {
                TH_LOG!("Expected deny_scope_signal event, got %d\n%s", count, buf);
            });

            ASSERT_EQ!(
                0,
                tracefs_extract_field(
                    buf,
                    REGEX_DENY_SCOPE_SIGNAL!(TRACE_TASK),
                    c"target_domain".as_ptr(),
                    field.as_mut_ptr(),
                    field.len(),
                )
            );
            EXPECT_EQ!((*variant).sandbox_target, strcmp(c"0".as_ptr(), field.as_ptr()) != 0, {
                TH_LOG!("Unexpected target_domain=%s", field.as_ptr());
            });
        } else {
            EXPECT_EQ!(0, count, {
                TH_LOG!("Expected 0 deny_scope_signal events, got %d\n%s", count, buf);
            });
        }

        free(buf as *mut c_void);
    }
}

TEST_HARNESS_MAIN!();

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
