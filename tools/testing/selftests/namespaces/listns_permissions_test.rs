// SPDX-License-Identifier: GPL-2.0
// C dependencies: errno.h, fcntl.h, limits.h, sched.h, stdio.h, stdlib.h,
// string.h, linux/nsfs.h, sys/capability.h, sys/ioctl.h, sys/prctl.h,
// sys/stat.h, sys/syscall.h, sys/types.h, sys/wait.h, unistd.h,
// ../kselftest_harness.h, ../filesystems/utils.h, wrappers.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

type __u64 = u64;
type ssize_t = isize;
type pid_t = c_int;
type cap_t = *mut c_void;
type cap_value_t = c_int;
type cap_flag_t = c_int;
type cap_flag_value_t = c_int;

#[repr(C)]
struct ns_id_req {
    size: __u64,
    spare: __u64,
    ns_id: __u64,
    ns_type: __u64,
    spare2: __u64,
    user_ns_id: __u64,
}

const O_RDONLY: c_int = 0;
const CLONE_NEWNET: c_int = 0x40000000;
const CLONE_NEWUTS: c_int = 0x04000000;
const CLONE_NEWIPC: c_int = 0x08000000;
const CLONE_NEWUSER: c_int = 0x10000000;
const NS_GET_ID: c_ulong = 0xb703;
const LISTNS_CURRENT_USER: __u64 = !0;
const CAP_SYS_ADMIN: cap_value_t = 21;
const CAP_EFFECTIVE: cap_flag_t = 0;
const CAP_PERMITTED: cap_flag_t = 1;
const CAP_CLEAR: cap_flag_value_t = 0;
const CAP_SET: cap_flag_value_t = 1;
const PR_SET_NO_NEW_PRIVS: c_int = 38;

unsafe extern "C" {
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn fork() -> pid_t;
    fn close(fd: c_int) -> c_int;
    fn exit(status: c_int) -> !;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> ssize_t;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> ssize_t;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn prctl(option: c_int, ...) -> c_int;

    fn setup_userns() -> c_int;
    fn sys_listns(req: *const ns_id_req, ids: *mut __u64, size: usize, flags: c_int) -> ssize_t;

    fn cap_get_proc() -> cap_t;
    fn cap_get_flag(
        cap_p: cap_t,
        cap: cap_value_t,
        flag: cap_flag_t,
        value_p: *mut cap_flag_value_t,
    ) -> c_int;
    fn cap_set_flag(
        cap_p: cap_t,
        flag: cap_flag_t,
        ncap: c_int,
        caps: *const cap_value_t,
        value: cap_flag_value_t,
    ) -> c_int;
    fn cap_set_proc(cap_p: cap_t) -> c_int;
    fn cap_free(cap_p: *mut c_void) -> c_int;
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

/*
 * Test that unprivileged users can only see namespaces they're currently in.
 * Create a namespace, drop privileges, verify we can only see our own namespaces.
 */
TEST!(listns_unprivileged_current_only, {
    unsafe {
        let mut req = ns_id_req {
            size: core::mem::size_of::<ns_id_req>() as __u64,
            spare: 0,
            ns_id: 0,
            ns_type: CLONE_NEWNET as __u64,
            spare2: 0,
            user_ns_id: 0,
        };
        let mut ns_ids: [__u64; 100] = [0; 100];
        let ret: ssize_t;
        let mut pipefd: [c_int; 2] = [0; 2];
        let pid: pid_t;
        let mut status: c_int = 0;
        let mut found_ours: bool;
        let mut unexpected_count: c_int;

        ASSERT_EQ!(pipe(pipefd.as_mut_ptr()), 0);

        pid = fork();
        ASSERT_GE!(pid, 0);

        if pid == 0 {
            let fd: c_int;
            let mut our_netns_id: __u64 = 0;
            let mut found_ours: bool;
            let mut unexpected_count: c_int;

            close(pipefd[0]);

            /* Create user namespace to be unprivileged */
            if setup_userns() < 0 {
                close(pipefd[1]);
                exit(1);
            }

            /* Create a network namespace */
            if unshare(CLONE_NEWNET) < 0 {
                close(pipefd[1]);
                exit(1);
            }

            /* Get our network namespace ID */
            fd = open(c"/proc/self/ns/net".as_ptr(), O_RDONLY);
            if fd < 0 {
                close(pipefd[1]);
                exit(1);
            }

            if ioctl(fd, NS_GET_ID, &mut our_netns_id as *mut __u64) < 0 {
                close(fd);
                close(pipefd[1]);
                exit(1);
            }
            close(fd);

            /* Now we're unprivileged - list all network namespaces */
            ret = sys_listns(&req, ns_ids.as_mut_ptr(), ns_ids.len(), 0);
            if ret < 0 {
                close(pipefd[1]);
                exit(1);
            }

            /* We should only see our own network namespace */
            found_ours = false;
            unexpected_count = 0;

            let mut i: ssize_t = 0;
            while i < ret {
                if ns_ids[i as usize] == our_netns_id {
                    found_ours = true;
                } else {
                    /* This is either init_net (which we can see) or unexpected */
                    unexpected_count += 1;
                }
                i += 1;
            }

            /* Send results to parent */
            write(
                pipefd[1],
                &found_ours as *const bool as *const c_void,
                core::mem::size_of_val(&found_ours),
            );
            write(
                pipefd[1],
                &unexpected_count as *const c_int as *const c_void,
                core::mem::size_of_val(&unexpected_count),
            );
            close(pipefd[1]);
            exit(0);
        }

        /* Parent */
        close(pipefd[1]);

        found_ours = false;
        unexpected_count = 0;
        read(
            pipefd[0],
            &mut found_ours as *mut bool as *mut c_void,
            core::mem::size_of_val(&found_ours),
        );
        read(
            pipefd[0],
            &mut unexpected_count as *mut c_int as *mut c_void,
            core::mem::size_of_val(&unexpected_count),
        );
        close(pipefd[0]);

        waitpid(pid, &mut status, 0);
        ASSERT_TRUE!(WIFEXITED(status));
        ASSERT_EQ!(WEXITSTATUS(status), 0);

        /* Child should have seen its own namespace */
        ASSERT_TRUE!(found_ours);

        TH_LOG!(
            "Unprivileged child saw its own namespace, plus {} others (likely init_net)",
            unexpected_count
        );
    }
});

/*
 * Test that users with CAP_SYS_ADMIN in a user namespace can see
 * all namespaces owned by that user namespace.
 */
TEST!(listns_cap_sys_admin_in_userns, {
    unsafe {
        let mut req = ns_id_req {
            size: core::mem::size_of::<ns_id_req>() as __u64,
            spare: 0,
            ns_id: 0,
            ns_type: 0, /* All types */
            spare2: 0,
            user_ns_id: 0, /* Will be set to our created user namespace */
        };
        let mut ns_ids: [__u64; 100] = [0; 100];
        let mut pipefd: [c_int; 2] = [0; 2];
        let pid: pid_t;
        let mut status: c_int = 0;
        let mut success: bool;
        let mut count: ssize_t;

        ASSERT_EQ!(pipe(pipefd.as_mut_ptr()), 0);

        pid = fork();
        ASSERT_GE!(pid, 0);

        if pid == 0 {
            let fd: c_int;
            let mut userns_id: __u64 = 0;
            let ret: ssize_t;
            let min_expected: c_int;
            let success: bool;

            close(pipefd[0]);

            /* Create user namespace - we'll have CAP_SYS_ADMIN in it */
            if setup_userns() < 0 {
                close(pipefd[1]);
                exit(1);
            }

            /* Get the user namespace ID */
            fd = open(c"/proc/self/ns/user".as_ptr(), O_RDONLY);
            if fd < 0 {
                close(pipefd[1]);
                exit(1);
            }

            if ioctl(fd, NS_GET_ID, &mut userns_id as *mut __u64) < 0 {
                close(fd);
                close(pipefd[1]);
                exit(1);
            }
            close(fd);

            /* Create several namespaces owned by this user namespace */
            unshare(CLONE_NEWNET);
            unshare(CLONE_NEWUTS);
            unshare(CLONE_NEWIPC);

            /* List namespaces owned by our user namespace */
            req.user_ns_id = userns_id;
            ret = sys_listns(&req, ns_ids.as_mut_ptr(), ns_ids.len(), 0);
            if ret < 0 {
                close(pipefd[1]);
                exit(1);
            }

            /*
             * We have CAP_SYS_ADMIN in this user namespace,
             * so we should see all namespaces owned by it.
             * That includes: net, uts, ipc, and the user namespace itself.
             */
            min_expected = 4;
            success = ret >= min_expected as ssize_t;

            write(
                pipefd[1],
                &success as *const bool as *const c_void,
                core::mem::size_of_val(&success),
            );
            write(
                pipefd[1],
                &ret as *const ssize_t as *const c_void,
                core::mem::size_of_val(&ret),
            );
            close(pipefd[1]);
            exit(0);
        }

        /* Parent */
        close(pipefd[1]);

        success = false;
        count = 0;
        read(
            pipefd[0],
            &mut success as *mut bool as *mut c_void,
            core::mem::size_of_val(&success),
        );
        read(
            pipefd[0],
            &mut count as *mut ssize_t as *mut c_void,
            core::mem::size_of_val(&count),
        );
        close(pipefd[0]);

        waitpid(pid, &mut status, 0);
        ASSERT_TRUE!(WIFEXITED(status));
        ASSERT_EQ!(WEXITSTATUS(status), 0);

        ASSERT_TRUE!(success);
        TH_LOG!(
            "User with CAP_SYS_ADMIN saw {} namespaces owned by their user namespace",
            count
        );
    }
});

/*
 * Test that users cannot see namespaces from unrelated user namespaces.
 * Create two sibling user namespaces, verify they can't see each other's
 * owned namespaces.
 */
TEST!(listns_cannot_see_sibling_userns_namespaces, {
    unsafe {
        let mut pipefd: [c_int; 2] = [0; 2];
        let pid1: pid_t;
        let pid2: pid_t;
        let mut status: c_int = 0;
        let mut netns_a_id: __u64;
        let mut pipefd2: [c_int; 2] = [0; 2];
        let mut found_sibling_netns: bool;

        ASSERT_EQ!(pipe(pipefd.as_mut_ptr()), 0);

        /* Fork first child - creates user namespace A */
        pid1 = fork();
        ASSERT_GE!(pid1, 0);

        if pid1 == 0 {
            let fd: c_int;
            let mut netns_a_id: __u64 = 0;
            let mut buf: c_char = 0;

            close(pipefd[0]);

            /* Create user namespace A */
            if setup_userns() < 0 {
                close(pipefd[1]);
                exit(1);
            }

            /* Create network namespace owned by user namespace A */
            if unshare(CLONE_NEWNET) < 0 {
                close(pipefd[1]);
                exit(1);
            }

            /* Get network namespace ID */
            fd = open(c"/proc/self/ns/net".as_ptr(), O_RDONLY);
            if fd < 0 {
                close(pipefd[1]);
                exit(1);
            }

            if ioctl(fd, NS_GET_ID, &mut netns_a_id as *mut __u64) < 0 {
                close(fd);
                close(pipefd[1]);
                exit(1);
            }
            close(fd);

            /* Send namespace ID to parent */
            write(
                pipefd[1],
                &netns_a_id as *const __u64 as *const c_void,
                core::mem::size_of_val(&netns_a_id),
            );

            /* Keep alive for sibling to check */
            read(pipefd[1], &mut buf as *mut c_char as *mut c_void, 1);
            close(pipefd[1]);
            exit(0);
        }

        /* Parent reads namespace A ID */
        close(pipefd[1]);
        netns_a_id = 0;
        read(
            pipefd[0],
            &mut netns_a_id as *mut __u64 as *mut c_void,
            core::mem::size_of_val(&netns_a_id),
        );

        TH_LOG!(
            "User namespace A created network namespace with ID {}",
            netns_a_id as u64
        );

        /* Fork second child - creates user namespace B */
        ASSERT_EQ!(pipe(pipefd2.as_mut_ptr()), 0);

        pid2 = fork();
        ASSERT_GE!(pid2, 0);

        if pid2 == 0 {
            let req = ns_id_req {
                size: core::mem::size_of::<ns_id_req>() as __u64,
                spare: 0,
                ns_id: 0,
                ns_type: CLONE_NEWNET as __u64,
                spare2: 0,
                user_ns_id: 0,
            };
            let mut ns_ids: [__u64; 100] = [0; 100];
            let ret: ssize_t;
            let mut found_sibling_netns: bool;

            close(pipefd[0]);
            close(pipefd2[0]);

            /* Create user namespace B (sibling to A) */
            if setup_userns() < 0 {
                close(pipefd2[1]);
                exit(1);
            }

            /* Try to list all network namespaces */
            ret = sys_listns(&req, ns_ids.as_mut_ptr(), ns_ids.len(), 0);

            found_sibling_netns = false;
            if ret > 0 {
                let mut i: ssize_t = 0;
                while i < ret {
                    if ns_ids[i as usize] == netns_a_id {
                        found_sibling_netns = true;
                        break;
                    }
                    i += 1;
                }
            }

            /* We should NOT see the sibling's network namespace */
            write(
                pipefd2[1],
                &found_sibling_netns as *const bool as *const c_void,
                core::mem::size_of_val(&found_sibling_netns),
            );
            close(pipefd2[1]);
            exit(0);
        }

        /* Parent reads result from second child */
        close(pipefd2[1]);
        found_sibling_netns = false;
        read(
            pipefd2[0],
            &mut found_sibling_netns as *mut bool as *mut c_void,
            core::mem::size_of_val(&found_sibling_netns),
        );
        close(pipefd2[0]);

        /* Signal first child to exit */
        close(pipefd[0]);

        /* Wait for both children */
        waitpid(pid2, &mut status, 0);
        ASSERT_TRUE!(WIFEXITED(status));

        waitpid(pid1, &mut status, 0);
        ASSERT_TRUE!(WIFEXITED(status));

        /* Second child should NOT have seen first child's namespace */
        ASSERT_FALSE!(found_sibling_netns);
        TH_LOG!("User namespace B correctly could not see sibling namespace A's network namespace");
    }
});

/*
 * Test permission checking with LISTNS_CURRENT_USER.
 * Verify that listing with LISTNS_CURRENT_USER respects permissions.
 */
TEST!(listns_current_user_permissions, {
    unsafe {
        let mut pipefd: [c_int; 2] = [0; 2];
        let pid: pid_t;
        let mut status: c_int = 0;
        let mut success: bool;
        let mut count: ssize_t;

        ASSERT_EQ!(pipe(pipefd.as_mut_ptr()), 0);

        pid = fork();
        ASSERT_GE!(pid, 0);

        if pid == 0 {
            let req = ns_id_req {
                size: core::mem::size_of::<ns_id_req>() as __u64,
                spare: 0,
                ns_id: 0,
                ns_type: 0,
                spare2: 0,
                user_ns_id: LISTNS_CURRENT_USER,
            };
            let mut ns_ids: [__u64; 100] = [0; 100];
            let ret: ssize_t;
            let success: bool;

            close(pipefd[0]);

            /* Create user namespace */
            if setup_userns() < 0 {
                close(pipefd[1]);
                exit(1);
            }

            /* Create some namespaces owned by this user namespace */
            if unshare(CLONE_NEWNET) < 0 {
                close(pipefd[1]);
                exit(1);
            }

            if unshare(CLONE_NEWUTS) < 0 {
                close(pipefd[1]);
                exit(1);
            }

            /* List with LISTNS_CURRENT_USER - should see our owned namespaces */
            ret = sys_listns(&req, ns_ids.as_mut_ptr(), ns_ids.len(), 0);

            success = ret >= 3; /* At least user, net, uts */
            write(
                pipefd[1],
                &success as *const bool as *const c_void,
                core::mem::size_of_val(&success),
            );
            write(
                pipefd[1],
                &ret as *const ssize_t as *const c_void,
                core::mem::size_of_val(&ret),
            );
            close(pipefd[1]);
            exit(0);
        }

        /* Parent */
        close(pipefd[1]);

        success = false;
        count = 0;
        read(
            pipefd[0],
            &mut success as *mut bool as *mut c_void,
            core::mem::size_of_val(&success),
        );
        read(
            pipefd[0],
            &mut count as *mut ssize_t as *mut c_void,
            core::mem::size_of_val(&count),
        );
        close(pipefd[0]);

        waitpid(pid, &mut status, 0);
        ASSERT_TRUE!(WIFEXITED(status));
        ASSERT_EQ!(WEXITSTATUS(status), 0);

        ASSERT_TRUE!(success);
        TH_LOG!("LISTNS_CURRENT_USER returned {} namespaces", count);
    }
});

/*
 * Test that CAP_SYS_ADMIN in parent user namespace allows seeing
 * child user namespace's owned namespaces.
 */
TEST!(listns_parent_userns_cap_sys_admin, {
    unsafe {
        let mut pipefd: [c_int; 2] = [0; 2];
        let pid: pid_t;
        let mut status: c_int = 0;
        let mut found_child_userns: bool;
        let mut count: ssize_t;

        ASSERT_EQ!(pipe(pipefd.as_mut_ptr()), 0);

        pid = fork();
        ASSERT_GE!(pid, 0);

        if pid == 0 {
            let fd: c_int;
            let mut parent_userns_id: __u64 = 0;
            let mut child_userns_id: __u64 = 0;
            let mut req = ns_id_req {
                size: 0,
                spare: 0,
                ns_id: 0,
                ns_type: 0,
                spare2: 0,
                user_ns_id: 0,
            };
            let mut ns_ids: [__u64; 100] = [0; 100];
            let ret: ssize_t;
            let mut found_child_userns: bool;

            close(pipefd[0]);

            /* Create parent user namespace - we have CAP_SYS_ADMIN in it */
            if setup_userns() < 0 {
                close(pipefd[1]);
                exit(1);
            }

            /* Get parent user namespace ID */
            fd = open(c"/proc/self/ns/user".as_ptr(), O_RDONLY);
            if fd < 0 {
                close(pipefd[1]);
                exit(1);
            }

            if ioctl(fd, NS_GET_ID, &mut parent_userns_id as *mut __u64) < 0 {
                close(fd);
                close(pipefd[1]);
                exit(1);
            }
            close(fd);

            /* Create child user namespace */
            if setup_userns() < 0 {
                close(pipefd[1]);
                exit(1);
            }

            /* Get child user namespace ID */
            fd = open(c"/proc/self/ns/user".as_ptr(), O_RDONLY);
            if fd < 0 {
                close(pipefd[1]);
                exit(1);
            }

            if ioctl(fd, NS_GET_ID, &mut child_userns_id as *mut __u64) < 0 {
                close(fd);
                close(pipefd[1]);
                exit(1);
            }
            close(fd);

            /* Create namespaces owned by child user namespace */
            if unshare(CLONE_NEWNET) < 0 {
                close(pipefd[1]);
                exit(1);
            }

            /* List namespaces owned by parent user namespace */
            req.size = core::mem::size_of_val(&req) as __u64;
            req.spare = 0;
            req.ns_id = 0;
            req.ns_type = 0;
            req.spare2 = 0;
            req.user_ns_id = parent_userns_id;

            ret = sys_listns(&req, ns_ids.as_mut_ptr(), ns_ids.len(), 0);

            /* Should see child user namespace in the list */
            found_child_userns = false;
            if ret > 0 {
                let mut i: ssize_t = 0;
                while i < ret {
                    if ns_ids[i as usize] == child_userns_id {
                        found_child_userns = true;
                        break;
                    }
                    i += 1;
                }
            }

            write(
                pipefd[1],
                &found_child_userns as *const bool as *const c_void,
                core::mem::size_of_val(&found_child_userns),
            );
            write(
                pipefd[1],
                &ret as *const ssize_t as *const c_void,
                core::mem::size_of_val(&ret),
            );
            close(pipefd[1]);
            exit(0);
        }

        /* Parent */
        close(pipefd[1]);

        found_child_userns = false;
        count = 0;
        read(
            pipefd[0],
            &mut found_child_userns as *mut bool as *mut c_void,
            core::mem::size_of_val(&found_child_userns),
        );
        read(
            pipefd[0],
            &mut count as *mut ssize_t as *mut c_void,
            core::mem::size_of_val(&count),
        );
        close(pipefd[0]);

        waitpid(pid, &mut status, 0);
        ASSERT_TRUE!(WIFEXITED(status));
        ASSERT_EQ!(WEXITSTATUS(status), 0);

        ASSERT_TRUE!(found_child_userns);
        TH_LOG!(
            "Process with CAP_SYS_ADMIN in parent user namespace saw child user namespace (total: {})",
            count
        );
    }
});

/*
 * Test that we can see user namespaces we have CAP_SYS_ADMIN inside of.
 * This is different from seeing namespaces owned by a user namespace.
 */
TEST!(listns_cap_sys_admin_inside_userns, {
    unsafe {
        let mut pipefd: [c_int; 2] = [0; 2];
        let pid: pid_t;
        let mut status: c_int = 0;
        let mut found_ours: bool;

        ASSERT_EQ!(pipe(pipefd.as_mut_ptr()), 0);

        pid = fork();
        ASSERT_GE!(pid, 0);

        if pid == 0 {
            let fd: c_int;
            let mut our_userns_id: __u64 = 0;
            let mut req = ns_id_req {
                size: 0,
                spare: 0,
                ns_id: 0,
                ns_type: 0,
                spare2: 0,
                user_ns_id: 0,
            };
            let mut ns_ids: [__u64; 100] = [0; 100];
            let ret: ssize_t;
            let mut found_ours: bool;

            close(pipefd[0]);

            /* Create user namespace - we have CAP_SYS_ADMIN inside it */
            if setup_userns() < 0 {
                close(pipefd[1]);
                exit(1);
            }

            /* Get our user namespace ID */
            fd = open(c"/proc/self/ns/user".as_ptr(), O_RDONLY);
            if fd < 0 {
                close(pipefd[1]);
                exit(1);
            }

            if ioctl(fd, NS_GET_ID, &mut our_userns_id as *mut __u64) < 0 {
                close(fd);
                close(pipefd[1]);
                exit(1);
            }
            close(fd);

            /* List all user namespaces globally */
            req.size = core::mem::size_of_val(&req) as __u64;
            req.spare = 0;
            req.ns_id = 0;
            req.ns_type = CLONE_NEWUSER as __u64;
            req.spare2 = 0;
            req.user_ns_id = 0;

            ret = sys_listns(&req, ns_ids.as_mut_ptr(), ns_ids.len(), 0);

            /* We should be able to see our own user namespace */
            found_ours = false;
            if ret > 0 {
                let mut i: ssize_t = 0;
                while i < ret {
                    if ns_ids[i as usize] == our_userns_id {
                        found_ours = true;
                        break;
                    }
                    i += 1;
                }
            }

            write(
                pipefd[1],
                &found_ours as *const bool as *const c_void,
                core::mem::size_of_val(&found_ours),
            );
            close(pipefd[1]);
            exit(0);
        }

        /* Parent */
        close(pipefd[1]);

        found_ours = false;
        read(
            pipefd[0],
            &mut found_ours as *mut bool as *mut c_void,
            core::mem::size_of_val(&found_ours),
        );
        close(pipefd[0]);

        waitpid(pid, &mut status, 0);
        ASSERT_TRUE!(WIFEXITED(status));
        ASSERT_EQ!(WEXITSTATUS(status), 0);

        ASSERT_TRUE!(found_ours);
        TH_LOG!("Process can see user namespace it has CAP_SYS_ADMIN inside of");
    }
});

/*
 * Test that dropping CAP_SYS_ADMIN restricts what we can see.
 */
TEST!(listns_drop_cap_sys_admin, {
    unsafe {
        let mut caps: cap_t;
        let cap_list: [cap_value_t; 1] = [CAP_SYS_ADMIN];

        /* This test needs to start with CAP_SYS_ADMIN */
        caps = cap_get_proc();
        if caps.is_null() {
            SKIP!(return, "Cannot get capabilities");
        }

        let mut cap_val: cap_flag_value_t = 0;
        if cap_get_flag(caps, CAP_SYS_ADMIN, CAP_EFFECTIVE, &mut cap_val) < 0 {
            cap_free(caps);
            SKIP!(return, "Cannot check CAP_SYS_ADMIN");
        }

        if cap_val != CAP_SET {
            cap_free(caps);
            SKIP!(return, "Test needs CAP_SYS_ADMIN to start");
        }
        cap_free(caps);

        let mut pipefd: [c_int; 2] = [0; 2];
        let pid: pid_t;
        let mut status: c_int = 0;
        let mut correct: bool;
        let mut count_before: ssize_t;
        let mut count_after: ssize_t;

        ASSERT_EQ!(pipe(pipefd.as_mut_ptr()), 0);

        pid = fork();
        ASSERT_GE!(pid, 0);

        if pid == 0 {
            let req = ns_id_req {
                size: core::mem::size_of::<ns_id_req>() as __u64,
                spare: 0,
                ns_id: 0,
                ns_type: CLONE_NEWNET as __u64,
                spare2: 0,
                user_ns_id: LISTNS_CURRENT_USER,
            };
            let mut ns_ids_before: [__u64; 100] = [0; 100];
            let count_before: ssize_t;
            let mut ns_ids_after: [__u64; 100] = [0; 100];
            let count_after: ssize_t;
            let correct: bool;

            close(pipefd[0]);

            /* Create user namespace */
            if setup_userns() < 0 {
                close(pipefd[1]);
                exit(1);
            }

            /* Count namespaces with CAP_SYS_ADMIN */
            count_before = sys_listns(&req, ns_ids_before.as_mut_ptr(), ns_ids_before.len(), 0);

            /* Drop CAP_SYS_ADMIN */
            caps = cap_get_proc();
            if !caps.is_null() {
                cap_set_flag(caps, CAP_EFFECTIVE, 1, cap_list.as_ptr(), CAP_CLEAR);
                cap_set_flag(caps, CAP_PERMITTED, 1, cap_list.as_ptr(), CAP_CLEAR);
                cap_set_proc(caps);
                cap_free(caps);
            }

            /* Ensure we can't regain the capability */
            prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0);

            /* Count namespaces without CAP_SYS_ADMIN */
            count_after = sys_listns(&req, ns_ids_after.as_mut_ptr(), ns_ids_after.len(), 0);

            /* Without CAP_SYS_ADMIN, we should see same or fewer namespaces */
            correct = count_after <= count_before;

            write(
                pipefd[1],
                &correct as *const bool as *const c_void,
                core::mem::size_of_val(&correct),
            );
            write(
                pipefd[1],
                &count_before as *const ssize_t as *const c_void,
                core::mem::size_of_val(&count_before),
            );
            write(
                pipefd[1],
                &count_after as *const ssize_t as *const c_void,
                core::mem::size_of_val(&count_after),
            );
            close(pipefd[1]);
            exit(0);
        }

        /* Parent */
        close(pipefd[1]);

        correct = false;
        count_before = 0;
        count_after = 0;
        read(
            pipefd[0],
            &mut correct as *mut bool as *mut c_void,
            core::mem::size_of_val(&correct),
        );
        read(
            pipefd[0],
            &mut count_before as *mut ssize_t as *mut c_void,
            core::mem::size_of_val(&count_before),
        );
        read(
            pipefd[0],
            &mut count_after as *mut ssize_t as *mut c_void,
            core::mem::size_of_val(&count_after),
        );
        close(pipefd[0]);

        waitpid(pid, &mut status, 0);
        ASSERT_TRUE!(WIFEXITED(status));
        ASSERT_EQ!(WEXITSTATUS(status), 0);

        ASSERT_TRUE!(correct);
        TH_LOG!(
            "With CAP_SYS_ADMIN: {} namespaces, without: {} namespaces",
            count_before,
            count_after
        );
    }
});

TEST_HARNESS_MAIN!();
