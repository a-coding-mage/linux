// SPDX-License-Identifier: GPL-2.0
// C source defined _GNU_SOURCE and __SANE_USERSPACE_TYPES__ before including
// Linux and libc headers.

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem;
use core::ptr;

const MOVE_MOUNT_F_EMPTY_PATH: c_uint = 0x00000004; /* Empty from path permitted */

#[repr(C)]
struct file_stressor {
    fd_tmpfs: c_int,
    nr_procs: c_int,
    max_fds: c_int,
    pids_openers: *mut libc::pid_t,
    pids_getdents: *mut libc::pid_t,
    fd_proc_pid: *mut c_int,
}

#[inline]
unsafe fn sys_fsopen(fsname: *const c_char, flags: c_uint) -> c_int {
    unsafe { libc::syscall(libc::SYS_fsopen as c_long, fsname, flags) as c_int }
}

#[inline]
unsafe fn sys_fsconfig(
    fd: c_int,
    cmd: c_uint,
    key: *const c_char,
    value: *const c_char,
    aux: c_int,
) -> c_int {
    unsafe { libc::syscall(libc::SYS_fsconfig as c_long, fd, cmd, key, value, aux) as c_int }
}

#[inline]
unsafe fn sys_fsmount(fd: c_int, flags: c_uint, attr_flags: c_uint) -> c_int {
    unsafe { libc::syscall(libc::SYS_fsmount as c_long, fd, flags, attr_flags) as c_int }
}

#[inline]
unsafe fn sys_move_mount(
    from_dfd: c_int,
    from_pathname: *const c_char,
    to_dfd: c_int,
    to_pathname: *const c_char,
    flags: c_uint,
) -> c_int {
    unsafe {
        libc::syscall(
            libc::SYS_move_mount as c_long,
            from_dfd,
            from_pathname,
            to_dfd,
            to_pathname,
            flags,
        ) as c_int
    }
}

unsafe fn file_stressor_setup(self_: *mut file_stressor) {
    let fd_context: c_int;

    assert_eq!(unsafe { libc::unshare(libc::CLONE_NEWNS) }, 0);
    assert_eq!(
        unsafe {
            libc::mount(
                ptr::null(),
                c"/".as_ptr(),
                ptr::null(),
                (libc::MS_SLAVE | libc::MS_REC) as libc::c_ulong,
                ptr::null(),
            )
        },
        0
    );
    assert_eq!(unsafe { libc::mkdir(c"/slab_typesafe_by_rcu".as_ptr(), 0o755) }, 0);

    fd_context = unsafe { sys_fsopen(c"tmpfs".as_ptr(), 0) };
    assert!(fd_context >= 0);

    assert_eq!(
        unsafe { sys_fsconfig(fd_context, libc::FSCONFIG_CMD_CREATE as c_uint, ptr::null(), ptr::null(), 0) },
        0
    );
    unsafe {
        (*self_).fd_tmpfs = sys_fsmount(fd_context, 0, 0);
    }
    assert!(unsafe { (*self_).fd_tmpfs } >= 0);
    assert_eq!(unsafe { libc::close(fd_context) }, 0);

    assert_eq!(
        unsafe {
            sys_move_mount(
                (*self_).fd_tmpfs,
                c"".as_ptr(),
                -libc::EBADF,
                c"/slab_typesafe_by_rcu".as_ptr(),
                MOVE_MOUNT_F_EMPTY_PATH,
            )
        },
        0
    );

    unsafe {
        (*self_).nr_procs = libc::sysconf(libc::_SC_NPROCESSORS_ONLN) as c_int;
        (*self_).pids_openers =
            libc::malloc(mem::size_of::<libc::pid_t>() * (*self_).nr_procs as usize) as *mut libc::pid_t;
    }
    assert_ne!(unsafe { (*self_).pids_openers }, ptr::null_mut());
    unsafe {
        (*self_).pids_getdents =
            libc::malloc(mem::size_of::<libc::pid_t>() * (*self_).nr_procs as usize) as *mut libc::pid_t;
    }
    assert_ne!(unsafe { (*self_).pids_getdents }, ptr::null_mut());
    unsafe {
        (*self_).fd_proc_pid =
            libc::malloc(mem::size_of::<c_int>() * (*self_).nr_procs as usize) as *mut c_int;
    }
    assert_ne!(unsafe { (*self_).fd_proc_pid }, ptr::null_mut());
    unsafe {
        (*self_).max_fds = 500;
    }
}

unsafe fn file_stressor_teardown(self_: *mut file_stressor) {
    let mut i: c_int = 0;
    while i < unsafe { (*self_).nr_procs } {
        let mut wstatus: c_int = 0;
        let mut pid: libc::pid_t;

        pid = unsafe { libc::waitpid(*(*self_).pids_openers.offset(i as isize), &mut wstatus, 0) };
        assert_eq!(pid, unsafe { *(*self_).pids_openers.offset(i as isize) });
        assert!(
            unsafe { !libc::WIFEXITED(wstatus) || !libc::WIFSIGNALED(wstatus) }
        );

        pid = unsafe { libc::waitpid(*(*self_).pids_getdents.offset(i as isize), &mut wstatus, 0) };
        assert_eq!(pid, unsafe { *(*self_).pids_getdents.offset(i as isize) });
        assert!(
            unsafe { !libc::WIFEXITED(wstatus) || !libc::WIFSIGNALED(wstatus) }
        );
        i += 1;
    }
    unsafe {
        libc::free((*self_).pids_openers as *mut c_void);
        libc::free((*self_).pids_getdents as *mut c_void);
    }
    assert_eq!(unsafe { libc::close((*self_).fd_tmpfs) }, 0);

    unsafe {
        libc::umount2(c"/slab_typesafe_by_rcu".as_ptr(), 0);
    }
    assert_eq!(unsafe { libc::rmdir(c"/slab_typesafe_by_rcu".as_ptr()) }, 0);
}

unsafe fn file_stressor_slab_typesafe_by_rcu(self_: *mut file_stressor) {
    let mut i: c_int = 0;
    while i < unsafe { (*self_).nr_procs } {
        let _pid_self: libc::pid_t;

        unsafe {
            *(*self_).pids_openers.offset(i as isize) = libc::fork();
        }
        assert!(unsafe { *(*self_).pids_openers.offset(i as isize) } >= 0);

        if unsafe { *(*self_).pids_openers.offset(i as isize) } != 0 {
            i += 1;
            continue;
        }

        unsafe {
            *(*self_).pids_openers.offset(i as isize) = libc::getpid();
        }
        loop {
            let mut i: c_int = 0;
            while i < unsafe { (*self_).max_fds } {
                let mut path: [c_char; libc::PATH_MAX as usize] = [0; libc::PATH_MAX as usize];
                let fd: c_int;

                unsafe {
                    libc::sprintf(
                        path.as_mut_ptr(),
                        c"/slab_typesafe_by_rcu/file-%d-%d".as_ptr(),
                        *(*self_).pids_openers.offset(i as isize),
                        i,
                    );
                    fd = libc::open(
                        path.as_ptr(),
                        libc::O_CREAT | libc::O_RDONLY | libc::O_CLOEXEC,
                        0o644,
                    );
                }
                if fd < 0 {
                    i += 1;
                    continue;
                }
                i += 1;
            }

            unsafe {
                libc::close_range(3, !0u32, 0);
            }
        }

        #[allow(unreachable_code)]
        unsafe {
            libc::exit(0);
        }
    }

    let mut i: c_int = 0;
    while i < unsafe { (*self_).nr_procs } {
        let mut path: [c_char; libc::PATH_MAX as usize] = [0; libc::PATH_MAX as usize];

        unsafe {
            libc::sprintf(path.as_mut_ptr(), c"/proc/%d/fd/".as_ptr(), *(*self_).pids_openers.offset(i as isize));
            *(*self_).fd_proc_pid.offset(i as isize) =
                libc::open(path.as_ptr(), libc::O_DIRECTORY | libc::O_RDONLY | libc::O_CLOEXEC);
        }
        assert!(unsafe { *(*self_).fd_proc_pid.offset(i as isize) } >= 0);
        i += 1;
    }

    let mut i: c_int = 0;
    while i < unsafe { (*self_).nr_procs } {
        unsafe {
            *(*self_).pids_getdents.offset(i as isize) = libc::fork();
        }
        assert!(unsafe { *(*self_).pids_getdents.offset(i as isize) } >= 0);

        if unsafe { *(*self_).pids_getdents.offset(i as isize) } != 0 {
            i += 1;
            continue;
        }

        unsafe {
            *(*self_).pids_getdents.offset(i as isize) = libc::getpid();
        }
        loop {
            let mut ents: [c_char; 1024] = [0; 1024];
            let mut nr_read: libc::ssize_t;

            /*
             * Concurrently read /proc/<pid>/fd/ which roughly does:
             *
             * f = fget_task_next(p, &fd);
             * if (!f)
             *	break;
             * data.mode = f->f_mode;
             * fput(f);
             *
             * Which means that it'll try to get a reference to a
             * file in another task's file descriptor table.
             *
             * Under heavy file load it is increasingly likely that
             * the other task will manage to close @file and @file
             * is being recycled due to SLAB_TYPEAFE_BY_RCU
             * concurrently. This will trigger various warnings in
             * the file reference counting code.
             */
            loop {
                nr_read = unsafe {
                    libc::syscall(
                        libc::SYS_getdents64 as c_long,
                        *(*self_).fd_proc_pid.offset(i as isize),
                        ents.as_mut_ptr(),
                        mem::size_of_val(&ents),
                    ) as libc::ssize_t
                };
                if nr_read < 0 {
                    break;
                }
            }

            unsafe {
                libc::lseek(*(*self_).fd_proc_pid.offset(i as isize), 0, libc::SEEK_SET);
            }
        }

        #[allow(unreachable_code)]
        unsafe {
            libc::exit(0);
        }
    }

    assert_eq!(
        unsafe {
            libc::clock_nanosleep(
                libc::CLOCK_MONOTONIC,
                0,
                &libc::timespec {
                    tv_sec: 900, /* 15 min */
                    tv_nsec: 0,
                },
                ptr::null_mut(),
            )
        },
        0
    );

    let mut i: c_int = 0;
    while i < unsafe { (*self_).nr_procs } {
        unsafe {
            libc::kill(*(*self_).pids_openers.offset(i as isize), libc::SIGKILL);
            libc::kill(*(*self_).pids_getdents.offset(i as isize), libc::SIGKILL);
        }
        i += 1;
    }
}

// Translated from TEST_HARNESS_MAIN. The concrete kselftest Rust harness entry
// point is an external build dependency in the original repository context.
