// SPDX-License-Identifier: GPL-2.0
// C includes translated as external Rust dependencies:
// errno.h, fcntl.h, limits.h, sched.h, stdio.h, stdlib.h, string.h,
// sys/capability.h, sys/ioctl.h, sys/stat.h, sys/syscall.h, sys/types.h,
// sys/wait.h, unistd.h, linux/nsfs.h, ../kselftest_harness.h,
// ../filesystems/utils.h, wrappers.h

/*
 * Test credential changes and their impact on namespace active references.
 */

type __u32 = u32;
type __u64 = u64;
type pid_t = libc::pid_t;
type uid_t = libc::uid_t;
type gid_t = libc::gid_t;
type ssize_t = libc::ssize_t;

#[repr(C)]
struct ns_id_req {
    size: __u32,
    spare: __u32,
    ns_id: __u64,
    ns_type: __u32,
    spare2: __u32,
    user_ns_id: __u64,
}

extern "C" {
    fn get_userns_fd(uid: uid_t, map_uid: uid_t, count: libc::c_int) -> libc::c_int;
    fn sys_listns(
        req: *const ns_id_req,
        ns_ids: *mut __u64,
        nr_ns_ids: libc::size_t,
        flags: libc::c_uint,
    ) -> ssize_t;
    fn setfsuid(fsuid: uid_t) -> libc::c_int;
    fn setfsgid(fsgid: gid_t) -> libc::c_int;
}

const CLONE_NEWUSER: libc::c_int = libc::CLONE_NEWUSER;
const NS_GET_ID: libc::c_ulong = 0xb703;

unsafe fn errno() -> libc::c_int {
    *libc::__errno_location()
}

fn wifexited(status: libc::c_int) -> bool {
    (status & 0x7f) == 0
}

fn wexitstatus(status: libc::c_int) -> libc::c_int {
    (status & 0xff00) >> 8
}

macro_rules! th_log {
    ($($arg:tt)*) => {
        eprintln!($($arg)*)
    };
}

macro_rules! skip {
    (return, $msg:expr) => {{
        eprintln!("{}", $msg);
        return;
    }};
}

fn new_userns_req() -> ns_id_req {
    ns_id_req {
        size: core::mem::size_of::<ns_id_req>() as __u32,
        spare: 0,
        ns_id: 0,
        ns_type: CLONE_NEWUSER as __u32,
        spare2: 0,
        user_ns_id: 0,
    }
}

/*
 * Test setuid() in a user namespace properly swaps active references.
 * Create a user namespace with multiple UIDs mapped, then setuid() between them.
 * Verify that the user namespace remains active throughout.
 */
fn setuid_preserves_active_refs() {
    unsafe {
        let mut status: libc::c_int = 0;
        let mut userns_id: __u64 = 0;
        let req = new_userns_req();
        let mut ns_ids: [__u64; 256] = [0; 256];
        let mut found = false;
        let mut pipefd: [libc::c_int; 2] = [0; 2];

        assert_eq!(libc::pipe(pipefd.as_mut_ptr()), 0);

        let pid: pid_t = libc::fork();
        assert!(pid >= 0);

        if pid == 0 {
            /* Child process */
            let orig_uid: uid_t = libc::getuid();

            libc::close(pipefd[0]);

            /* Create new user namespace with multiple UIDs mapped (0-9) */
            let userns_fd = get_userns_fd(0, orig_uid, 10);
            if userns_fd < 0 {
                libc::close(pipefd[1]);
                libc::_exit(1);
            }

            if libc::setns(userns_fd, CLONE_NEWUSER) < 0 {
                libc::close(userns_fd);
                libc::close(pipefd[1]);
                libc::_exit(1);
            }
            libc::close(userns_fd);

            /* Get user namespace ID */
            let fd = libc::open(b"/proc/self/ns/user\0".as_ptr() as *const libc::c_char, libc::O_RDONLY);
            if fd < 0 {
                libc::close(pipefd[1]);
                libc::_exit(1);
            }

            let mut child_userns_id: __u64 = 0;
            if libc::ioctl(fd, NS_GET_ID, &mut child_userns_id) < 0 {
                libc::close(fd);
                libc::close(pipefd[1]);
                libc::_exit(1);
            }
            libc::close(fd);

            /* Send namespace ID to parent */
            libc::write(
                pipefd[1],
                &child_userns_id as *const __u64 as *const libc::c_void,
                core::mem::size_of_val(&child_userns_id),
            );

            /*
             * Perform multiple setuid() calls.
             * Each setuid() triggers commit_creds() which should properly
             * swap active references via switch_cred_namespaces().
             */
            for setuid_count in 0..50 {
                let target_uid: uid_t = (setuid_count % 10) as uid_t;
                if libc::setuid(target_uid) < 0 {
                    if errno() != libc::EPERM {
                        libc::close(pipefd[1]);
                        libc::_exit(1);
                    }
                }
            }

            libc::close(pipefd[1]);
            libc::_exit(0);
        }

        /* Parent process */
        libc::close(pipefd[1]);

        if libc::read(
            pipefd[0],
            &mut userns_id as *mut __u64 as *mut libc::c_void,
            core::mem::size_of_val(&userns_id),
        ) != core::mem::size_of_val(&userns_id) as ssize_t
        {
            libc::close(pipefd[0]);
            libc::kill(pid, libc::SIGKILL);
            libc::waitpid(pid, core::ptr::null_mut(), 0);
            skip!(return, "Failed to get namespace ID from child");
        }
        libc::close(pipefd[0]);

        th_log!("Child user namespace ID: {}", userns_id as libc::c_ulonglong);

        /* Verify namespace is active while child is running */
        let mut ret = sys_listns(&req, ns_ids.as_mut_ptr(), ns_ids.len(), 0);
        if ret < 0 {
            libc::kill(pid, libc::SIGKILL);
            libc::waitpid(pid, core::ptr::null_mut(), 0);
            if errno() == libc::ENOSYS {
                skip!(return, "listns() not supported");
            }
            assert!(ret >= 0);
        }

        for i in 0..ret {
            if ns_ids[i as usize] == userns_id {
                found = true;
                break;
            }
        }
        assert!(found);

        libc::waitpid(pid, &mut status, 0);
        assert!(wifexited(status));
        assert_eq!(wexitstatus(status), 0);

        /* Verify namespace becomes inactive after child exits */
        ret = sys_listns(&req, ns_ids.as_mut_ptr(), ns_ids.len(), 0);
        assert!(ret >= 0);

        found = false;
        for i in 0..ret {
            if ns_ids[i as usize] == userns_id {
                found = true;
                break;
            }
        }

        assert!(!found);
        th_log!("setuid() correctly preserved active references (no leak)");
    }
}

/*
 * Test setgid() in a user namespace properly handles active references.
 */
fn setgid_preserves_active_refs() {
    unsafe {
        let mut status: libc::c_int = 0;
        let mut userns_id: __u64 = 0;
        let req = new_userns_req();
        let mut ns_ids: [__u64; 256] = [0; 256];
        let mut found = false;
        let mut pipefd: [libc::c_int; 2] = [0; 2];

        assert_eq!(libc::pipe(pipefd.as_mut_ptr()), 0);

        let pid: pid_t = libc::fork();
        assert!(pid >= 0);

        if pid == 0 {
            /* Child process */
            let orig_uid: uid_t = libc::getuid();

            libc::close(pipefd[0]);

            /* Create new user namespace with multiple GIDs mapped */
            let userns_fd = get_userns_fd(0, orig_uid, 10);
            if userns_fd < 0 {
                libc::close(pipefd[1]);
                libc::_exit(1);
            }

            if libc::setns(userns_fd, CLONE_NEWUSER) < 0 {
                libc::close(userns_fd);
                libc::close(pipefd[1]);
                libc::_exit(1);
            }
            libc::close(userns_fd);

            /* Get user namespace ID */
            let fd = libc::open(b"/proc/self/ns/user\0".as_ptr() as *const libc::c_char, libc::O_RDONLY);
            if fd < 0 {
                libc::close(pipefd[1]);
                libc::_exit(1);
            }

            let mut child_userns_id: __u64 = 0;
            if libc::ioctl(fd, NS_GET_ID, &mut child_userns_id) < 0 {
                libc::close(fd);
                libc::close(pipefd[1]);
                libc::_exit(1);
            }
            libc::close(fd);

            libc::write(
                pipefd[1],
                &child_userns_id as *const __u64 as *const libc::c_void,
                core::mem::size_of_val(&child_userns_id),
            );

            /* Perform multiple setgid() calls */
            for setgid_count in 0..50 {
                let target_gid: gid_t = (setgid_count % 10) as gid_t;
                if libc::setgid(target_gid) < 0 {
                    if errno() != libc::EPERM {
                        libc::close(pipefd[1]);
                        libc::_exit(1);
                    }
                }
            }

            libc::close(pipefd[1]);
            libc::_exit(0);
        }

        /* Parent process */
        libc::close(pipefd[1]);

        if libc::read(
            pipefd[0],
            &mut userns_id as *mut __u64 as *mut libc::c_void,
            core::mem::size_of_val(&userns_id),
        ) != core::mem::size_of_val(&userns_id) as ssize_t
        {
            libc::close(pipefd[0]);
            libc::kill(pid, libc::SIGKILL);
            libc::waitpid(pid, core::ptr::null_mut(), 0);
            skip!(return, "Failed to get namespace ID from child");
        }
        libc::close(pipefd[0]);

        libc::waitpid(pid, &mut status, 0);
        assert!(wifexited(status));
        assert_eq!(wexitstatus(status), 0);

        /* Verify namespace becomes inactive */
        let ret = sys_listns(&req, ns_ids.as_mut_ptr(), ns_ids.len(), 0);
        if ret < 0 {
            if errno() == libc::ENOSYS {
                skip!(return, "listns() not supported");
            }
            assert!(ret >= 0);
        }

        for i in 0..ret {
            if ns_ids[i as usize] == userns_id {
                found = true;
                break;
            }
        }

        assert!(!found);
        th_log!("setgid() correctly preserved active references (no leak)");
    }
}

/*
 * Test setresuid() which changes real, effective, and saved UIDs.
 * This should properly swap active references via commit_creds().
 */
fn setresuid_preserves_active_refs() {
    unsafe {
        let mut status: libc::c_int = 0;
        let mut userns_id: __u64 = 0;
        let req = new_userns_req();
        let mut ns_ids: [__u64; 256] = [0; 256];
        let mut found = false;
        let mut pipefd: [libc::c_int; 2] = [0; 2];

        assert_eq!(libc::pipe(pipefd.as_mut_ptr()), 0);

        let pid: pid_t = libc::fork();
        assert!(pid >= 0);

        if pid == 0 {
            /* Child process */
            let orig_uid: uid_t = libc::getuid();

            libc::close(pipefd[0]);

            /* Create new user namespace */
            let userns_fd = get_userns_fd(0, orig_uid, 10);
            if userns_fd < 0 {
                libc::close(pipefd[1]);
                libc::_exit(1);
            }

            if libc::setns(userns_fd, CLONE_NEWUSER) < 0 {
                libc::close(userns_fd);
                libc::close(pipefd[1]);
                libc::_exit(1);
            }
            libc::close(userns_fd);

            /* Get user namespace ID */
            let fd = libc::open(b"/proc/self/ns/user\0".as_ptr() as *const libc::c_char, libc::O_RDONLY);
            if fd < 0 {
                libc::close(pipefd[1]);
                libc::_exit(1);
            }

            let mut child_userns_id: __u64 = 0;
            if libc::ioctl(fd, NS_GET_ID, &mut child_userns_id) < 0 {
                libc::close(fd);
                libc::close(pipefd[1]);
                libc::_exit(1);
            }
            libc::close(fd);

            libc::write(
                pipefd[1],
                &child_userns_id as *const __u64 as *const libc::c_void,
                core::mem::size_of_val(&child_userns_id),
            );

            /* Perform multiple setresuid() calls */
            for setres_count in 0..30 {
                let uid1: uid_t = (setres_count % 5) as uid_t;
                let uid2: uid_t = ((setres_count + 1) % 5) as uid_t;
                let uid3: uid_t = ((setres_count + 2) % 5) as uid_t;

                if libc::setresuid(uid1, uid2, uid3) < 0 {
                    if errno() != libc::EPERM {
                        libc::close(pipefd[1]);
                        libc::_exit(1);
                    }
                }
            }

            libc::close(pipefd[1]);
            libc::_exit(0);
        }

        /* Parent process */
        libc::close(pipefd[1]);

        if libc::read(
            pipefd[0],
            &mut userns_id as *mut __u64 as *mut libc::c_void,
            core::mem::size_of_val(&userns_id),
        ) != core::mem::size_of_val(&userns_id) as ssize_t
        {
            libc::close(pipefd[0]);
            libc::kill(pid, libc::SIGKILL);
            libc::waitpid(pid, core::ptr::null_mut(), 0);
            skip!(return, "Failed to get namespace ID from child");
        }
        libc::close(pipefd[0]);

        libc::waitpid(pid, &mut status, 0);
        assert!(wifexited(status));
        assert_eq!(wexitstatus(status), 0);

        /* Verify namespace becomes inactive */
        let ret = sys_listns(&req, ns_ids.as_mut_ptr(), ns_ids.len(), 0);
        if ret < 0 {
            if errno() == libc::ENOSYS {
                skip!(return, "listns() not supported");
            }
            assert!(ret >= 0);
        }

        for i in 0..ret {
            if ns_ids[i as usize] == userns_id {
                found = true;
                break;
            }
        }

        assert!(!found);
        th_log!("setresuid() correctly preserved active references (no leak)");
    }
}

/*
 * Test credential changes across multiple user namespaces.
 * Create nested user namespaces and verify active reference tracking.
 */
fn cred_change_nested_userns() {
    unsafe {
        let mut status: libc::c_int = 0;
        let mut parent_userns_id: __u64 = 0;
        let mut child_userns_id: __u64 = 0;
        let req = new_userns_req();
        let mut ns_ids: [__u64; 256] = [0; 256];
        let mut found_parent = false;
        let mut found_child = false;
        let mut pipefd: [libc::c_int; 2] = [0; 2];

        assert_eq!(libc::pipe(pipefd.as_mut_ptr()), 0);

        let pid: pid_t = libc::fork();
        assert!(pid >= 0);

        if pid == 0 {
            /* Child process */
            let orig_uid: uid_t = libc::getuid();

            libc::close(pipefd[0]);

            /* Create first user namespace */
            let userns_fd = get_userns_fd(0, orig_uid, 1);
            if userns_fd < 0 {
                libc::close(pipefd[1]);
                libc::_exit(1);
            }

            if libc::setns(userns_fd, CLONE_NEWUSER) < 0 {
                libc::close(userns_fd);
                libc::close(pipefd[1]);
                libc::_exit(1);
            }
            libc::close(userns_fd);

            /* Get first namespace ID */
            let fd = libc::open(b"/proc/self/ns/user\0".as_ptr() as *const libc::c_char, libc::O_RDONLY);
            if fd < 0 {
                libc::close(pipefd[1]);
                libc::_exit(1);
            }

            let mut parent_id: __u64 = 0;
            if libc::ioctl(fd, NS_GET_ID, &mut parent_id) < 0 {
                libc::close(fd);
                libc::close(pipefd[1]);
                libc::_exit(1);
            }
            libc::close(fd);

            /* Create nested user namespace */
            let userns_fd = get_userns_fd(0, 0, 1);
            if userns_fd < 0 {
                libc::close(pipefd[1]);
                libc::_exit(1);
            }

            if libc::setns(userns_fd, CLONE_NEWUSER) < 0 {
                libc::close(userns_fd);
                libc::close(pipefd[1]);
                libc::_exit(1);
            }
            libc::close(userns_fd);

            /* Get nested namespace ID */
            let fd = libc::open(b"/proc/self/ns/user\0".as_ptr() as *const libc::c_char, libc::O_RDONLY);
            if fd < 0 {
                libc::close(pipefd[1]);
                libc::_exit(1);
            }

            let mut child_id: __u64 = 0;
            if libc::ioctl(fd, NS_GET_ID, &mut child_id) < 0 {
                libc::close(fd);
                libc::close(pipefd[1]);
                libc::_exit(1);
            }
            libc::close(fd);

            /* Send both IDs to parent */
            libc::write(
                pipefd[1],
                &parent_id as *const __u64 as *const libc::c_void,
                core::mem::size_of_val(&parent_id),
            );
            libc::write(
                pipefd[1],
                &child_id as *const __u64 as *const libc::c_void,
                core::mem::size_of_val(&child_id),
            );

            /* Perform some credential changes in nested namespace */
            libc::setuid(0);
            libc::setgid(0);

            libc::close(pipefd[1]);
            libc::_exit(0);
        }

        /* Parent process */
        libc::close(pipefd[1]);

        /* Read both namespace IDs */
        if libc::read(
            pipefd[0],
            &mut parent_userns_id as *mut __u64 as *mut libc::c_void,
            core::mem::size_of_val(&parent_userns_id),
        ) != core::mem::size_of_val(&parent_userns_id) as ssize_t
        {
            libc::close(pipefd[0]);
            libc::kill(pid, libc::SIGKILL);
            libc::waitpid(pid, core::ptr::null_mut(), 0);
            skip!(return, "Failed to get parent namespace ID");
        }

        if libc::read(
            pipefd[0],
            &mut child_userns_id as *mut __u64 as *mut libc::c_void,
            core::mem::size_of_val(&child_userns_id),
        ) != core::mem::size_of_val(&child_userns_id) as ssize_t
        {
            libc::close(pipefd[0]);
            libc::kill(pid, libc::SIGKILL);
            libc::waitpid(pid, core::ptr::null_mut(), 0);
            skip!(return, "Failed to get child namespace ID");
        }
        libc::close(pipefd[0]);

        th_log!(
            "Parent userns: {}, Child userns: {}",
            parent_userns_id as libc::c_ulonglong,
            child_userns_id as libc::c_ulonglong
        );

        /* Verify both namespaces are active */
        let mut ret = sys_listns(&req, ns_ids.as_mut_ptr(), ns_ids.len(), 0);
        if ret < 0 {
            libc::kill(pid, libc::SIGKILL);
            libc::waitpid(pid, core::ptr::null_mut(), 0);
            if errno() == libc::ENOSYS {
                skip!(return, "listns() not supported");
            }
            assert!(ret >= 0);
        }

        for i in 0..ret {
            if ns_ids[i as usize] == parent_userns_id {
                found_parent = true;
            }
            if ns_ids[i as usize] == child_userns_id {
                found_child = true;
            }
        }

        assert!(found_parent);
        assert!(found_child);

        /* Wait for child */
        libc::waitpid(pid, &mut status, 0);
        assert!(wifexited(status));
        assert_eq!(wexitstatus(status), 0);

        /* Verify both namespaces become inactive */
        ret = sys_listns(&req, ns_ids.as_mut_ptr(), ns_ids.len(), 0);
        assert!(ret >= 0);

        found_parent = false;
        found_child = false;
        for i in 0..ret {
            if ns_ids[i as usize] == parent_userns_id {
                found_parent = true;
            }
            if ns_ids[i as usize] == child_userns_id {
                found_child = true;
            }
        }

        assert!(!found_parent);
        assert!(!found_child);
        th_log!("Nested user namespace credential changes preserved active refs (no leak)");
    }
}

/*
 * Test rapid credential changes don't cause refcount imbalances.
 * This stress-tests the switch_cred_namespaces() logic.
 */
fn rapid_cred_changes_no_leak() {
    unsafe {
        let mut status: libc::c_int = 0;
        let mut userns_id: __u64 = 0;
        let req = new_userns_req();
        let mut ns_ids: [__u64; 256] = [0; 256];
        let mut found = false;
        let mut pipefd: [libc::c_int; 2] = [0; 2];

        assert_eq!(libc::pipe(pipefd.as_mut_ptr()), 0);

        let pid: pid_t = libc::fork();
        assert!(pid >= 0);

        if pid == 0 {
            /* Child process */
            let orig_uid: uid_t = libc::getuid();

            libc::close(pipefd[0]);

            /* Create new user namespace with wider range of UIDs/GIDs */
            let userns_fd = get_userns_fd(0, orig_uid, 100);
            if userns_fd < 0 {
                libc::close(pipefd[1]);
                libc::_exit(1);
            }

            if libc::setns(userns_fd, CLONE_NEWUSER) < 0 {
                libc::close(userns_fd);
                libc::close(pipefd[1]);
                libc::_exit(1);
            }
            libc::close(userns_fd);

            /* Get user namespace ID */
            let fd = libc::open(b"/proc/self/ns/user\0".as_ptr() as *const libc::c_char, libc::O_RDONLY);
            if fd < 0 {
                libc::close(pipefd[1]);
                libc::_exit(1);
            }

            let mut child_userns_id: __u64 = 0;
            if libc::ioctl(fd, NS_GET_ID, &mut child_userns_id) < 0 {
                libc::close(fd);
                libc::close(pipefd[1]);
                libc::_exit(1);
            }
            libc::close(fd);

            libc::write(
                pipefd[1],
                &child_userns_id as *const __u64 as *const libc::c_void,
                core::mem::size_of_val(&child_userns_id),
            );

            /*
             * Perform many rapid credential changes.
             * Mix setuid, setgid, setreuid, setregid, setresuid, setresgid.
             */
            for change_count in 0..200 {
                match change_count % 6 {
                    0 => {
                        libc::setuid((change_count % 50) as uid_t);
                    }
                    1 => {
                        libc::setgid((change_count % 50) as gid_t);
                    }
                    2 => {
                        libc::setreuid(
                            (change_count % 50) as uid_t,
                            ((change_count + 1) % 50) as uid_t,
                        );
                    }
                    3 => {
                        libc::setregid(
                            (change_count % 50) as gid_t,
                            ((change_count + 1) % 50) as gid_t,
                        );
                    }
                    4 => {
                        libc::setresuid(
                            (change_count % 50) as uid_t,
                            ((change_count + 1) % 50) as uid_t,
                            ((change_count + 2) % 50) as uid_t,
                        );
                    }
                    5 => {
                        libc::setresgid(
                            (change_count % 50) as gid_t,
                            ((change_count + 1) % 50) as gid_t,
                            ((change_count + 2) % 50) as gid_t,
                        );
                    }
                    _ => {}
                }
            }

            libc::close(pipefd[1]);
            libc::_exit(0);
        }

        /* Parent process */
        libc::close(pipefd[1]);

        if libc::read(
            pipefd[0],
            &mut userns_id as *mut __u64 as *mut libc::c_void,
            core::mem::size_of_val(&userns_id),
        ) != core::mem::size_of_val(&userns_id) as ssize_t
        {
            libc::close(pipefd[0]);
            libc::kill(pid, libc::SIGKILL);
            libc::waitpid(pid, core::ptr::null_mut(), 0);
            skip!(return, "Failed to get namespace ID from child");
        }
        libc::close(pipefd[0]);

        th_log!("Testing with user namespace ID: {}", userns_id as libc::c_ulonglong);

        libc::waitpid(pid, &mut status, 0);
        assert!(wifexited(status));
        assert_eq!(wexitstatus(status), 0);

        /* Verify namespace becomes inactive (no leaked active refs) */
        let ret = sys_listns(&req, ns_ids.as_mut_ptr(), ns_ids.len(), 0);
        if ret < 0 {
            if errno() == libc::ENOSYS {
                skip!(return, "listns() not supported");
            }
            assert!(ret >= 0);
        }

        for i in 0..ret {
            if ns_ids[i as usize] == userns_id {
                found = true;
                break;
            }
        }

        assert!(!found);
        th_log!("200 rapid credential changes completed with no active ref leak");
    }
}

/*
 * Test setfsuid/setfsgid which change filesystem UID/GID.
 * These also trigger credential changes but may have different code paths.
 */
fn setfsuid_preserves_active_refs() {
    unsafe {
        let mut status: libc::c_int = 0;
        let mut userns_id: __u64 = 0;
        let req = new_userns_req();
        let mut ns_ids: [__u64; 256] = [0; 256];
        let mut found = false;
        let mut pipefd: [libc::c_int; 2] = [0; 2];

        assert_eq!(libc::pipe(pipefd.as_mut_ptr()), 0);

        let pid: pid_t = libc::fork();
        assert!(pid >= 0);

        if pid == 0 {
            /* Child process */
            let orig_uid: uid_t = libc::getuid();

            libc::close(pipefd[0]);

            /* Create new user namespace */
            let userns_fd = get_userns_fd(0, orig_uid, 10);
            if userns_fd < 0 {
                libc::close(pipefd[1]);
                libc::_exit(1);
            }

            if libc::setns(userns_fd, CLONE_NEWUSER) < 0 {
                libc::close(userns_fd);
                libc::close(pipefd[1]);
                libc::_exit(1);
            }
            libc::close(userns_fd);

            /* Get user namespace ID */
            let fd = libc::open(b"/proc/self/ns/user\0".as_ptr() as *const libc::c_char, libc::O_RDONLY);
            if fd < 0 {
                libc::close(pipefd[1]);
                libc::_exit(1);
            }

            let mut child_userns_id: __u64 = 0;
            if libc::ioctl(fd, NS_GET_ID, &mut child_userns_id) < 0 {
                libc::close(fd);
                libc::close(pipefd[1]);
                libc::_exit(1);
            }
            libc::close(fd);

            libc::write(
                pipefd[1],
                &child_userns_id as *const __u64 as *const libc::c_void,
                core::mem::size_of_val(&child_userns_id),
            );

            /* Perform multiple setfsuid/setfsgid calls */
            for change_count in 0..50 {
                setfsuid((change_count % 10) as uid_t);
                setfsgid((change_count % 10) as gid_t);
            }

            libc::close(pipefd[1]);
            libc::_exit(0);
        }

        /* Parent process */
        libc::close(pipefd[1]);

        if libc::read(
            pipefd[0],
            &mut userns_id as *mut __u64 as *mut libc::c_void,
            core::mem::size_of_val(&userns_id),
        ) != core::mem::size_of_val(&userns_id) as ssize_t
        {
            libc::close(pipefd[0]);
            libc::kill(pid, libc::SIGKILL);
            libc::waitpid(pid, core::ptr::null_mut(), 0);
            skip!(return, "Failed to get namespace ID from child");
        }
        libc::close(pipefd[0]);

        libc::waitpid(pid, &mut status, 0);
        assert!(wifexited(status));
        assert_eq!(wexitstatus(status), 0);

        /* Verify namespace becomes inactive */
        let ret = sys_listns(&req, ns_ids.as_mut_ptr(), ns_ids.len(), 0);
        if ret < 0 {
            if errno() == libc::ENOSYS {
                skip!(return, "listns() not supported");
            }
            assert!(ret >= 0);
        }

        for i in 0..ret {
            if ns_ids[i as usize] == userns_id {
                found = true;
                break;
            }
        }

        assert!(!found);
        th_log!("setfsuid/setfsgid correctly preserved active references (no leak)");
    }
}

fn main() {
    setuid_preserves_active_refs();
    setgid_preserves_active_refs();
    setresuid_preserves_active_refs();
    cred_change_nested_userns();
    rapid_cred_changes_no_leak();
    setfsuid_preserves_active_refs();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
