// SPDX-License-Identifier: GPL-2.0
// C dependency intent: _GNU_SOURCE plus errno, fcntl, limits, sched, signal,
// stdio, stdlib, string, linux/nsfs, ioctl, mmap, mount, socket, stat, syscall,
// types, wait, unistd, kselftest_harness, pidfd, and wrappers.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
#![allow(unused_unsafe)]

type __u64 = u64;
type ssize_t = isize;
type pid_t = libc::pid_t;

#[repr(C)]
struct ns_id_req {
    size: u32,
    spare: u32,
    ns_id: __u64,
    ns_type: u32,
    spare2: u32,
    user_ns_id: __u64,
}

unsafe extern "C" {
    fn sysconf(name: libc::c_int) -> libc::c_long;
    fn mmap(
        addr: *mut libc::c_void,
        length: libc::size_t,
        prot: libc::c_int,
        flags: libc::c_int,
        fd: libc::c_int,
        offset: libc::off_t,
    ) -> *mut libc::c_void;
    fn munmap(addr: *mut libc::c_void, length: libc::size_t) -> libc::c_int;
    fn usleep(usec: libc::useconds_t) -> libc::c_int;
    fn socketpair(
        domain: libc::c_int,
        type_: libc::c_int,
        protocol: libc::c_int,
        sv: *mut libc::c_int,
    ) -> libc::c_int;
    fn close(fd: libc::c_int) -> libc::c_int;
    fn mount(
        source: *const libc::c_char,
        target: *const libc::c_char,
        filesystemtype: *const libc::c_char,
        mountflags: libc::c_ulong,
        data: *const libc::c_void,
    ) -> libc::c_int;
    fn mkdir(pathname: *const libc::c_char, mode: libc::mode_t) -> libc::c_int;
    fn waitpid(pid: pid_t, wstatus: *mut libc::c_int, options: libc::c_int) -> pid_t;
    fn _exit(status: libc::c_int) -> !;
    fn __errno_location() -> *mut libc::c_int;

    fn create_child(pidfd: *mut libc::c_int, flags: libc::c_int) -> pid_t;
    fn sys_listns(
        req: *mut ns_id_req,
        ns_ids: *mut __u64,
        nr_ns_ids: libc::size_t,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn sys_pidfd_send_signal(
        pidfd: libc::c_int,
        sig: libc::c_int,
        info: *mut libc::siginfo_t,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn write_nointr(fd: libc::c_int, buf: *const libc::c_void, count: libc::size_t) -> ssize_t;
    fn read_nointr(fd: libc::c_int, buf: *mut libc::c_void, count: libc::size_t) -> ssize_t;
}

const PIDFD_SKIP: libc::c_int = 4;

unsafe fn errno() -> libc::c_int {
    *__errno_location()
}

/*
 * Test listns() error handling with invalid buffer addresses.
 *
 * When the buffer pointer is invalid (e.g., crossing page boundaries
 * into unmapped memory), listns() returns EINVAL.
 *
 * This test also creates mount namespaces that get destroyed during
 * iteration, testing that namespace cleanup happens outside the RCU
 * read lock.
 */
TEST!(listns_partial_fault_with_ns_cleanup, {
    let mut map: *mut libc::c_void;
    let mut ns_ids: *mut __u64;
    let mut ret: ssize_t;
    let mut page_size: libc::c_long;
    let mut pid: pid_t;
    let mut iter_pid: pid_t;
    let mut ns_pids: [pid_t; 5] = [0; 5];
    let mut pidfds: [libc::c_int; 5] = [0; 5];
    let mut sv: [[libc::c_int; 2]; 5] = [[0; 2]; 5];
    let mut iter_pidfd: libc::c_int = 0;
    let mut i: libc::c_int;
    let mut status: libc::c_int = 0;
    let mut c: libc::c_char = 0;

    unsafe {
        page_size = sysconf(libc::_SC_PAGESIZE);
        ASSERT_GT!(page_size, 0);

        /*
         * Map two pages:
         * - First page: readable and writable
         * - Second page: will be unmapped to trigger EFAULT
         */
        map = mmap(
            core::ptr::null_mut(),
            (page_size * 2) as libc::size_t,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        );
        ASSERT_NE!(map, libc::MAP_FAILED);

        /* Unmap the second page */
        ret = munmap((map as *mut libc::c_char).offset(page_size as isize) as *mut libc::c_void, page_size as libc::size_t) as ssize_t;
        ASSERT_EQ!(ret, 0);

        /*
         * Position the buffer pointer so there's room for exactly one u64
         * before the page boundary. The second u64 would fall into the
         * unmapped page.
         */
        ns_ids = ((map as *mut libc::c_char).offset(page_size as isize) as *mut __u64).offset(-1);

        /*
         * Create a separate process to run listns() in a loop concurrently
         * with namespace creation and destruction.
         */
        iter_pid = create_child(&mut iter_pidfd, 0);
        ASSERT_NE!(iter_pid, -1);

        if iter_pid == 0 {
            let mut req = ns_id_req {
                size: core::mem::size_of::<ns_id_req>() as u32,
                spare: 0,
                ns_id: 0,
                ns_type: 0, /* All types */
                spare2: 0,
                user_ns_id: 0, /* Global listing */
            };
            let mut iter_ret: libc::c_int;

            /*
             * Loop calling listns() until killed.
             * The kernel should:
             * 1. Successfully write the first namespace ID (within valid page)
             * 2. Fail with EFAULT when trying to write the second ID (unmapped page)
             * 3. Handle concurrent namespace destruction without deadlock
             */
            loop {
                iter_ret = sys_listns(&mut req, ns_ids, 2, 0);

                if iter_ret == -1 && errno() == libc::ENOSYS {
                    _exit(PIDFD_SKIP);
                }
            }
        }

        /* Small delay to let iterator start looping */
        usleep(50000);

        /*
         * Create several child processes, each in its own mount namespace.
         * These will be destroyed while the iterator is running listns().
         */
        i = 0;
        while i < 5 {
            /* Create socketpair for synchronization */
            ASSERT_EQ!(socketpair(libc::AF_UNIX, libc::SOCK_STREAM, 0, sv[i as usize].as_mut_ptr()), 0);

            pid = create_child(&mut pidfds[i as usize], libc::CLONE_NEWNS);
            ASSERT_NE!(pid, -1);
            ns_pids[i as usize] = pid;

            if pid == 0 {
                close(sv[i as usize][0]); /* Close parent end */

                if mount(core::ptr::null(), c"/".as_ptr(), core::ptr::null(), (libc::MS_REC | libc::MS_PRIVATE) as libc::c_ulong, core::ptr::null()) != 0 {
                    _exit(1);
                }

                /* Child: create a couple of tmpfs mounts */
                if mkdir(c"/tmp/test_mnt1".as_ptr(), 0o755) == -1 && errno() != libc::EEXIST {
                    _exit(1);
                }
                if mkdir(c"/tmp/test_mnt2".as_ptr(), 0o755) == -1 && errno() != libc::EEXIST {
                    _exit(1);
                }

                if mount(c"tmpfs".as_ptr(), c"/tmp/test_mnt1".as_ptr(), c"tmpfs".as_ptr(), 0, core::ptr::null()) == -1 {
                    _exit(1);
                }
                if mount(c"tmpfs".as_ptr(), c"/tmp/test_mnt2".as_ptr(), c"tmpfs".as_ptr(), 0, core::ptr::null()) == -1 {
                    _exit(1);
                }

                /* Signal parent that setup is complete */
                if write_nointr(sv[i as usize][1], c"R".as_ptr() as *const libc::c_void, 1) != 1 {
                    _exit(1);
                }

                /* Wait for parent to signal us to exit */
                if read_nointr(sv[i as usize][1], &mut c as *mut _ as *mut libc::c_void, 1) != 1 {
                    _exit(1);
                }

                close(sv[i as usize][1]);
                _exit(0);
            }

            close(sv[i as usize][1]); /* Close child end */
            i += 1;
        }

        /* Wait for all children to finish setup */
        i = 0;
        while i < 5 {
            ret = read_nointr(sv[i as usize][0], &mut c as *mut _ as *mut libc::c_void, 1);
            ASSERT_EQ!(ret, 1);
            ASSERT_EQ!(c, b'R' as libc::c_char);
            i += 1;
        }

        /*
         * Signal children to exit. This will destroy their mount namespaces
         * while listns() is iterating the namespace tree.
         * This tests that cleanup happens outside the RCU read lock.
         */
        i = 0;
        while i < 5 {
            write_nointr(sv[i as usize][0], c"X".as_ptr() as *const libc::c_void, 1);
            i += 1;
        }

        /* Wait for all mount namespace children to exit and cleanup */
        i = 0;
        while i < 5 {
            waitpid(ns_pids[i as usize], core::ptr::null_mut(), 0);
            close(sv[i as usize][0]);
            close(pidfds[i as usize]);
            i += 1;
        }

        /* Kill iterator and wait for it */
        sys_pidfd_send_signal(iter_pidfd, libc::SIGKILL, core::ptr::null_mut(), 0);
        ret = waitpid(iter_pid, &mut status, 0) as ssize_t;
        ASSERT_EQ!(ret, iter_pid);
        close(iter_pidfd);

        /* If listns() is not supported the iterator exits cleanly via ENOSYS */
        if libc::WIFEXITED(status) && libc::WEXITSTATUS(status) == PIDFD_SKIP {
            munmap(map, page_size as libc::size_t);
            SKIP!(return, "listns() not supported");
        }

        /* Should have been killed */
        ASSERT_TRUE!(libc::WIFSIGNALED(status));
        ASSERT_EQ!(libc::WTERMSIG(status), libc::SIGKILL);

        /* Clean up */
        munmap(map, page_size as libc::size_t);
    }
});

/*
 * Test listns() error handling when the entire buffer is invalid.
 * This is a sanity check that basic invalid pointer detection works.
 */
TEST!(listns_complete_fault, {
    let mut req = ns_id_req {
        size: core::mem::size_of::<ns_id_req>() as u32,
        spare: 0,
        ns_id: 0,
        ns_type: 0,
        spare2: 0,
        user_ns_id: 0,
    };
    let mut ns_ids: *mut __u64;
    let mut ret: ssize_t;

    unsafe {
        /* Use a clearly invalid pointer */
        ns_ids = 0xdeadbeefusize as *mut __u64;

        ret = sys_listns(&mut req, ns_ids, 10, 0) as ssize_t;

        if ret == -1 && errno() == libc::ENOSYS {
            SKIP!(return, "listns() not supported");
        }

        /* Should fail with EFAULT */
        ASSERT_EQ!(ret, -1);
        ASSERT_EQ!(errno(), libc::EFAULT);
    }
});

/*
 * Test listns() error handling when the buffer is NULL.
 */
TEST!(listns_null_buffer, {
    let mut req = ns_id_req {
        size: core::mem::size_of::<ns_id_req>() as u32,
        spare: 0,
        ns_id: 0,
        ns_type: 0,
        spare2: 0,
        user_ns_id: 0,
    };
    let mut ret: ssize_t;

    unsafe {
        /* NULL buffer with non-zero count should fail */
        ret = sys_listns(&mut req, core::ptr::null_mut(), 10, 0) as ssize_t;

        if ret == -1 && errno() == libc::ENOSYS {
            SKIP!(return, "listns() not supported");
        }

        /* Should fail with EFAULT */
        ASSERT_EQ!(ret, -1);
        ASSERT_EQ!(errno(), libc::EFAULT);
    }
});

/*
 * Test listns() with a buffer that becomes invalid mid-iteration
 * (after several successful writes), combined with mount namespace
 * destruction to test RCU cleanup logic.
 */
TEST!(listns_late_fault_with_ns_cleanup, {
    let mut map: *mut libc::c_void;
    let mut ns_ids: *mut __u64;
    let mut ret: ssize_t;
    let mut page_size: libc::c_long;
    let mut pid: pid_t;
    let mut iter_pid: pid_t;
    let mut ns_pids: [pid_t; 10] = [0; 10];
    let mut pidfds: [libc::c_int; 10] = [0; 10];
    let mut sv: [[libc::c_int; 2]; 10] = [[0; 2]; 10];
    let mut iter_pidfd: libc::c_int = 0;
    let mut i: libc::c_int;
    let mut status: libc::c_int = 0;
    let mut c: libc::c_char = 0;

    unsafe {
        page_size = sysconf(libc::_SC_PAGESIZE);
        ASSERT_GT!(page_size, 0);

        /* Map two pages */
        map = mmap(
            core::ptr::null_mut(),
            (page_size * 2) as libc::size_t,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        );
        ASSERT_NE!(map, libc::MAP_FAILED);

        /* Unmap the second page */
        ret = munmap((map as *mut libc::c_char).offset(page_size as isize) as *mut libc::c_void, page_size as libc::size_t) as ssize_t;
        ASSERT_EQ!(ret, 0);

        /*
         * Position buffer so we can write several u64s successfully
         * before hitting the page boundary.
         */
        ns_ids = ((map as *mut libc::c_char).offset(page_size as isize) as *mut __u64).offset(-5);

        /*
         * Create a separate process to run listns() concurrently.
         */
        iter_pid = create_child(&mut iter_pidfd, 0);
        ASSERT_NE!(iter_pid, -1);

        if iter_pid == 0 {
            let mut req = ns_id_req {
                size: core::mem::size_of::<ns_id_req>() as u32,
                spare: 0,
                ns_id: 0,
                ns_type: 0,
                spare2: 0,
                user_ns_id: 0,
            };
            let mut iter_ret: libc::c_int;

            /*
             * Loop calling listns() until killed.
             * Request 10 namespace IDs while namespaces are being destroyed.
             * This tests:
             * 1. EFAULT handling when buffer becomes invalid
             * 2. Namespace cleanup outside RCU read lock during iteration
             */
            loop {
                iter_ret = sys_listns(&mut req, ns_ids, 10, 0);

                if iter_ret == -1 && errno() == libc::ENOSYS {
                    _exit(PIDFD_SKIP);
                }
            }
        }

        /* Small delay to let iterator start looping */
        usleep(50000);

        /*
         * Create more children with mount namespaces to increase the
         * likelihood that namespace cleanup happens during iteration.
         */
        i = 0;
        while i < 10 {
            /* Create socketpair for synchronization */
            ASSERT_EQ!(socketpair(libc::AF_UNIX, libc::SOCK_STREAM, 0, sv[i as usize].as_mut_ptr()), 0);

            pid = create_child(&mut pidfds[i as usize], libc::CLONE_NEWNS);
            ASSERT_NE!(pid, -1);
            ns_pids[i as usize] = pid;

            if pid == 0 {
                close(sv[i as usize][0]); /* Close parent end */

                if mount(core::ptr::null(), c"/".as_ptr(), core::ptr::null(), (libc::MS_REC | libc::MS_PRIVATE) as libc::c_ulong, core::ptr::null()) != 0 {
                    _exit(1);
                }

                /* Child: create tmpfs mounts */
                if mkdir(c"/tmp/test_mnt1".as_ptr(), 0o755) == -1 && errno() != libc::EEXIST {
                    _exit(1);
                }
                if mkdir(c"/tmp/test_mnt2".as_ptr(), 0o755) == -1 && errno() != libc::EEXIST {
                    _exit(1);
                }

                if mount(c"tmpfs".as_ptr(), c"/tmp/test_mnt1".as_ptr(), c"tmpfs".as_ptr(), 0, core::ptr::null()) == -1 {
                    _exit(1);
                }
                if mount(c"tmpfs".as_ptr(), c"/tmp/test_mnt2".as_ptr(), c"tmpfs".as_ptr(), 0, core::ptr::null()) == -1 {
                    _exit(1);
                }

                /* Signal parent that setup is complete */
                if write_nointr(sv[i as usize][1], c"R".as_ptr() as *const libc::c_void, 1) != 1 {
                    _exit(1);
                }

                /* Wait for parent to signal us to exit */
                if read_nointr(sv[i as usize][1], &mut c as *mut _ as *mut libc::c_void, 1) != 1 {
                    _exit(1);
                }

                close(sv[i as usize][1]);
                _exit(0);
            }

            close(sv[i as usize][1]); /* Close child end */
            i += 1;
        }

        /* Wait for all children to finish setup */
        i = 0;
        while i < 10 {
            ret = read_nointr(sv[i as usize][0], &mut c as *mut _ as *mut libc::c_void, 1);
            ASSERT_EQ!(ret, 1);
            ASSERT_EQ!(c, b'R' as libc::c_char);
            i += 1;
        }

        /* Kill half the children */
        i = 0;
        while i < 5 {
            write_nointr(sv[i as usize][0], c"X".as_ptr() as *const libc::c_void, 1);
            i += 1;
        }

        /* Small delay to let some exit */
        usleep(10000);

        /* Kill remaining children */
        i = 5;
        while i < 10 {
            write_nointr(sv[i as usize][0], c"X".as_ptr() as *const libc::c_void, 1);
            i += 1;
        }

        /* Wait for all children and cleanup */
        i = 0;
        while i < 10 {
            waitpid(ns_pids[i as usize], core::ptr::null_mut(), 0);
            close(sv[i as usize][0]);
            close(pidfds[i as usize]);
            i += 1;
        }

        /* Kill iterator and wait for it */
        sys_pidfd_send_signal(iter_pidfd, libc::SIGKILL, core::ptr::null_mut(), 0);
        ret = waitpid(iter_pid, &mut status, 0) as ssize_t;
        ASSERT_EQ!(ret, iter_pid);
        close(iter_pidfd);

        /* If listns() is not supported the iterator exits cleanly via ENOSYS */
        if libc::WIFEXITED(status) && libc::WEXITSTATUS(status) == PIDFD_SKIP {
            munmap(map, page_size as libc::size_t);
            SKIP!(return, "listns() not supported");
        }

        /* Should have been killed */
        ASSERT_TRUE!(libc::WIFSIGNALED(status));
        ASSERT_EQ!(libc::WTERMSIG(status), libc::SIGKILL);

        /* Clean up */
        munmap(map, page_size as libc::size_t);
    }
});

/*
 * Test specifically focused on mount namespace cleanup during EFAULT.
 * Filter for mount namespaces only.
 */
TEST!(listns_mnt_ns_cleanup_on_fault, {
    let mut map: *mut libc::c_void;
    let mut ns_ids: *mut __u64;
    let mut ret: ssize_t;
    let mut page_size: libc::c_long;
    let mut pid: pid_t;
    let mut iter_pid: pid_t;
    let mut ns_pids: [pid_t; 8] = [0; 8];
    let mut pidfds: [libc::c_int; 8] = [0; 8];
    let mut sv: [[libc::c_int; 2]; 8] = [[0; 2]; 8];
    let mut iter_pidfd: libc::c_int = 0;
    let mut i: libc::c_int;
    let mut status: libc::c_int = 0;
    let mut c: libc::c_char = 0;

    unsafe {
        page_size = sysconf(libc::_SC_PAGESIZE);
        ASSERT_GT!(page_size, 0);

        /* Set up partial fault buffer */
        map = mmap(
            core::ptr::null_mut(),
            (page_size * 2) as libc::size_t,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        );
        ASSERT_NE!(map, libc::MAP_FAILED);

        ret = munmap((map as *mut libc::c_char).offset(page_size as isize) as *mut libc::c_void, page_size as libc::size_t) as ssize_t;
        ASSERT_EQ!(ret, 0);

        /* Position for 3 successful writes, then fault */
        ns_ids = ((map as *mut libc::c_char).offset(page_size as isize) as *mut __u64).offset(-3);

        /*
         * Create a separate process to run listns() concurrently.
         */
        iter_pid = create_child(&mut iter_pidfd, 0);
        ASSERT_NE!(iter_pid, -1);

        if iter_pid == 0 {
            let mut req = ns_id_req {
                size: core::mem::size_of::<ns_id_req>() as u32,
                spare: 0,
                ns_id: 0,
                ns_type: libc::CLONE_NEWNS as u32, /* Only mount namespaces */
                spare2: 0,
                user_ns_id: 0,
            };
            let mut iter_ret: libc::c_int;

            /*
             * Loop calling listns() until killed.
             * Call listns() to race with namespace destruction.
             */
            loop {
                iter_ret = sys_listns(&mut req, ns_ids, 10, 0);

                if iter_ret == -1 && errno() == libc::ENOSYS {
                    _exit(PIDFD_SKIP);
                }
            }
        }

        /* Small delay to let iterator start looping */
        usleep(50000);

        /* Create children with mount namespaces */
        i = 0;
        while i < 8 {
            /* Create socketpair for synchronization */
            ASSERT_EQ!(socketpair(libc::AF_UNIX, libc::SOCK_STREAM, 0, sv[i as usize].as_mut_ptr()), 0);

            pid = create_child(&mut pidfds[i as usize], libc::CLONE_NEWNS);
            ASSERT_NE!(pid, -1);
            ns_pids[i as usize] = pid;

            if pid == 0 {
                close(sv[i as usize][0]); /* Close parent end */

                if mount(core::ptr::null(), c"/".as_ptr(), core::ptr::null(), (libc::MS_REC | libc::MS_PRIVATE) as libc::c_ulong, core::ptr::null()) != 0 {
                    _exit(1);
                }

                /* Do some mount operations to make cleanup more interesting */
                if mkdir(c"/tmp/test_mnt1".as_ptr(), 0o755) == -1 && errno() != libc::EEXIST {
                    _exit(1);
                }
                if mkdir(c"/tmp/test_mnt2".as_ptr(), 0o755) == -1 && errno() != libc::EEXIST {
                    _exit(1);
                }

                if mount(c"tmpfs".as_ptr(), c"/tmp/test_mnt1".as_ptr(), c"tmpfs".as_ptr(), 0, core::ptr::null()) == -1 {
                    _exit(1);
                }
                if mount(c"tmpfs".as_ptr(), c"/tmp/test_mnt2".as_ptr(), c"tmpfs".as_ptr(), 0, core::ptr::null()) == -1 {
                    _exit(1);
                }

                /* Signal parent that setup is complete */
                if write_nointr(sv[i as usize][1], c"R".as_ptr() as *const libc::c_void, 1) != 1 {
                    _exit(1);
                }

                /* Wait for parent to signal us to exit */
                if read_nointr(sv[i as usize][1], &mut c as *mut _ as *mut libc::c_void, 1) != 1 {
                    _exit(1);
                }

                close(sv[i as usize][1]);
                _exit(0);
            }

            close(sv[i as usize][1]); /* Close child end */
            i += 1;
        }

        /* Wait for all children to finish setup */
        i = 0;
        while i < 8 {
            ret = read_nointr(sv[i as usize][0], &mut c as *mut _ as *mut libc::c_void, 1);
            ASSERT_EQ!(ret, 1);
            ASSERT_EQ!(c, b'R' as libc::c_char);
            i += 1;
        }

        /* Kill children to trigger namespace destruction during iteration */
        i = 0;
        while i < 8 {
            write_nointr(sv[i as usize][0], c"X".as_ptr() as *const libc::c_void, 1);
            i += 1;
        }

        /* Wait for children and cleanup */
        i = 0;
        while i < 8 {
            waitpid(ns_pids[i as usize], core::ptr::null_mut(), 0);
            close(sv[i as usize][0]);
            close(pidfds[i as usize]);
            i += 1;
        }

        /* Kill iterator and wait for it */
        sys_pidfd_send_signal(iter_pidfd, libc::SIGKILL, core::ptr::null_mut(), 0);
        ret = waitpid(iter_pid, &mut status, 0) as ssize_t;
        ASSERT_EQ!(ret, iter_pid);
        close(iter_pidfd);

        /* If listns() is not supported the iterator exits cleanly via ENOSYS */
        if libc::WIFEXITED(status) && libc::WEXITSTATUS(status) == PIDFD_SKIP {
            munmap(map, page_size as libc::size_t);
            SKIP!(return, "listns() not supported");
        }

        /* Should have been killed */
        ASSERT_TRUE!(libc::WIFSIGNALED(status));
        ASSERT_EQ!(libc::WTERMSIG(status), libc::SIGKILL);

        munmap(map, page_size as libc::size_t);
    }
});

TEST_HARNESS_MAIN!();
