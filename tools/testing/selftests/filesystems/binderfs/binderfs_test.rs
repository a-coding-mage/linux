// SPDX-License-Identifier: GPL-2.0

// C source dependencies removed from executable Rust:
// _GNU_SOURCE, errno/fcntl/pthread/sched/stdbool/stdio/stdlib/string,
// sys/fsuid, sys/ioctl, sys/mount, sys/socket, sys/stat, sys/sysinfo,
// sys/types, sys/wait, unistd, linux/android/binder,
// linux/android/binderfs, and kselftest_harness.

const DEFAULT_THREADS: usize = 4;

macro_rules! ptr_to_int {
    ($p:expr) => {
        ($p as isize) as i32
    };
}

macro_rules! int_to_ptr {
    ($u:expr) => {
        (($u as isize) as *mut ::core::ffi::c_void)
    };
}

macro_rules! close_prot_errno_disarm {
    ($fd:expr) => {
        if $fd >= 0 {
            let _e_ = unsafe { errno };
            unsafe {
                close($fd);
                errno = _e_;
            }
            $fd = -EBADF;
        }
    };
}

unsafe fn change_mountns(_metadata: *mut __test_metadata) {
    let mut ret: i32;

    ret = unshare(CLONE_NEWNS);
    ASSERT_EQ!(ret, 0, {
        TH_LOG!(
            "%s - Failed to unshare mount namespace",
            strerror(errno)
        );
    });

    ret = mount(
        ::core::ptr::null(),
        c_str!("/").as_ptr(),
        ::core::ptr::null(),
        MS_REC | MS_PRIVATE,
        0 as *const ::core::ffi::c_void,
    );
    ASSERT_EQ!(ret, 0, {
        TH_LOG!("%s - Failed to mount / as private", strerror(errno));
    });
}

unsafe fn __do_binderfs_test(_metadata: *mut __test_metadata) -> i32 {
    let mut fd: i32;
    let mut ret: i32;
    let mut saved_errno: i32;
    let mut result: i32 = 1;
    let _len: usize;
    let mut device: binderfs_device = ::core::mem::zeroed();
    let mut version: binder_version = ::core::mem::zeroed();
    let mut binderfs_mntpt = *b"/tmp/binderfs_XXXXXX\0";
    let mut device_path = [0 as ::core::ffi::c_char; b"/tmp/binderfs_XXXXXX/".len() + BINDERFS_MAX_NAME];
    static BINDER_FEATURES: [&[u8]; 4] = [
        b"oneway_spam_detection\0",
        b"extended_error\0",
        b"freeze_notification\0",
        b"transaction_report\0",
    ];

    change_mountns(_metadata);

    EXPECT_NE!(mkdtemp(binderfs_mntpt.as_mut_ptr() as *mut ::core::ffi::c_char), ::core::ptr::null_mut(), {
        TH_LOG!(
            "%s - Failed to create binderfs mountpoint",
            strerror(errno)
        );
        goto_out!();
    });

    ret = mount(
        ::core::ptr::null(),
        binderfs_mntpt.as_ptr() as *const ::core::ffi::c_char,
        c_str!("binder").as_ptr(),
        0,
        0 as *const ::core::ffi::c_void,
    );
    EXPECT_EQ!(ret, 0, {
        if errno == ENODEV {
            SKIP!(goto_out!(), "binderfs missing");
        }
        TH_LOG!("%s - Failed to mount binderfs", strerror(errno));
        goto_rmdir!();
    });

    /* success: binderfs mounted */

    memcpy(
        device.name.as_mut_ptr() as *mut ::core::ffi::c_void,
        c_str!("my-binder").as_ptr() as *const ::core::ffi::c_void,
        strlen(c_str!("my-binder").as_ptr()),
    );

    snprintf(
        device_path.as_mut_ptr(),
        device_path.len(),
        c_str!("%s/binder-control").as_ptr(),
        binderfs_mntpt.as_ptr(),
    );
    fd = open(device_path.as_ptr(), O_RDONLY | O_CLOEXEC);
    EXPECT_GE!(fd, 0, {
        TH_LOG!(
            "%s - Failed to open binder-control device",
            strerror(errno)
        );
        goto_umount!();
    });

    ret = ioctl(fd, BINDER_CTL_ADD, &mut device);
    saved_errno = errno;
    close(fd);
    errno = saved_errno;
    EXPECT_GE!(ret, 0, {
        TH_LOG!(
            "%s - Failed to allocate new binder device",
            strerror(errno)
        );
        goto_umount!();
    });

    TH_LOG!(
        "Allocated new binder device with major %d, minor %d, and name %s",
        device.major,
        device.minor,
        device.name.as_ptr()
    );

    /* success: binder device allocation */

    snprintf(
        device_path.as_mut_ptr(),
        device_path.len(),
        c_str!("%s/my-binder").as_ptr(),
        binderfs_mntpt.as_ptr(),
    );
    fd = open(device_path.as_ptr(), O_CLOEXEC | O_RDONLY);
    EXPECT_GE!(fd, 0, {
        TH_LOG!(
            "%s - Failed to open my-binder device",
            strerror(errno)
        );
        goto_umount!();
    });

    ret = ioctl(fd, BINDER_VERSION, &mut version);
    saved_errno = errno;
    close(fd);
    errno = saved_errno;
    EXPECT_GE!(ret, 0, {
        TH_LOG!(
            "%s - Failed to open perform BINDER_VERSION request",
            strerror(errno)
        );
        goto_umount!();
    });

    TH_LOG!("Detected binder version: %d", version.protocol_version);

    /* success: binder transaction with binderfs binder device */

    ret = unlink(device_path.as_ptr());
    EXPECT_EQ!(ret, 0, {
        TH_LOG!("%s - Failed to delete binder device", strerror(errno));
        goto_umount!();
    });

    /* success: binder device removal */

    snprintf(
        device_path.as_mut_ptr(),
        device_path.len(),
        c_str!("%s/binder-control").as_ptr(),
        binderfs_mntpt.as_ptr(),
    );
    ret = unlink(device_path.as_ptr());
    EXPECT_NE!(ret, 0, {
        TH_LOG!("Managed to delete binder-control device");
        goto_umount!();
    });
    EXPECT_EQ!(errno, EPERM, {
        TH_LOG!(
            "%s - Failed to delete binder-control device but exited with unexpected error code",
            strerror(errno)
        );
        goto_umount!();
    });

    /* success: binder-control device removal failed as expected */

    for i in 0..BINDER_FEATURES.len() {
        snprintf(
            device_path.as_mut_ptr(),
            device_path.len(),
            c_str!("%s/features/%s").as_ptr(),
            binderfs_mntpt.as_ptr(),
            BINDER_FEATURES[i].as_ptr(),
        );
        fd = open(device_path.as_ptr(), O_CLOEXEC | O_RDONLY);
        EXPECT_GE!(fd, 0, {
            TH_LOG!(
                "%s - Failed to open binder feature: %s",
                strerror(errno),
                BINDER_FEATURES[i].as_ptr()
            );
            goto_umount!();
        });
        close(fd);
    }

    /* success: binder feature files found */
    result = 0;

    // umount:
    ret = umount2(binderfs_mntpt.as_ptr() as *const ::core::ffi::c_char, MNT_DETACH);
    EXPECT_EQ!(ret, 0, {
        TH_LOG!("%s - Failed to unmount binderfs", strerror(errno));
    });
    // rmdir:
    ret = rmdir(binderfs_mntpt.as_ptr() as *const ::core::ffi::c_char);
    EXPECT_EQ!(ret, 0, {
        TH_LOG!("%s - Failed to rmdir binderfs mount", strerror(errno));
    });
    // out:
    return result;
}

unsafe fn wait_for_pid(pid: pid_t) -> i32 {
    let mut status: i32 = 0;
    let mut ret: i32;

    loop {
        ret = waitpid(pid, &mut status, 0);
        if ret == -1 {
            if errno == EINTR {
                continue;
            }

            return -1;
        }
        break;
    }

    if !WIFEXITED(status) {
        return -1;
    }

    WEXITSTATUS(status)
}

unsafe fn setid_userns_root() -> i32 {
    if setuid(0) != 0 {
        return -1;
    }
    if setgid(0) != 0 {
        return -1;
    }

    setfsuid(0);
    setfsgid(0);

    0
}

#[repr(C)]
enum idmap_type {
    UID_MAP,
    GID_MAP,
}

unsafe fn read_nointr(fd: i32, buf: *mut ::core::ffi::c_void, count: usize) -> ssize_t {
    let mut ret: ssize_t;
    loop {
        ret = read(fd, buf, count);
        if !(ret < 0 && errno == EINTR) {
            break;
        }
    }

    ret
}

unsafe fn write_nointr(fd: i32, buf: *const ::core::ffi::c_void, count: usize) -> ssize_t {
    let mut ret: ssize_t;
    loop {
        ret = write(fd, buf, count);
        if !(ret < 0 && errno == EINTR) {
            break;
        }
    }

    ret
}

unsafe fn write_id_mapping(
    type_: idmap_type,
    pid: pid_t,
    buf: *const ::core::ffi::c_char,
    buf_size: usize,
) -> i32 {
    let mut fd: i32;
    let mut ret: i32;
    let mut path = [0 as ::core::ffi::c_char; 4096];

    if matches!(type_, idmap_type::GID_MAP) {
        let mut setgroups_fd: i32;

        snprintf(
            path.as_mut_ptr(),
            path.len(),
            c_str!("/proc/%d/setgroups").as_ptr(),
            pid,
        );
        setgroups_fd = open(path.as_ptr(), O_WRONLY | O_CLOEXEC | O_NOFOLLOW);
        if setgroups_fd < 0 && errno != ENOENT {
            return -1;
        }

        if setgroups_fd >= 0 {
            ret = write_nointr(
                setgroups_fd,
                c_str!("deny").as_ptr() as *const ::core::ffi::c_void,
                size_of_c_str!("deny") - 1,
            ) as i32;
            close_prot_errno_disarm!(setgroups_fd);
            if ret != (size_of_c_str!("deny") - 1) as i32 {
                return -1;
            }
        }
    }

    match type_ {
        idmap_type::UID_MAP => {
            ret = snprintf(
                path.as_mut_ptr(),
                path.len(),
                c_str!("/proc/%d/uid_map").as_ptr(),
                pid,
            );
        }
        idmap_type::GID_MAP => {
            ret = snprintf(
                path.as_mut_ptr(),
                path.len(),
                c_str!("/proc/%d/gid_map").as_ptr(),
                pid,
            );
        }
    }
    if ret < 0 || ret as usize >= path.len() {
        return -E2BIG;
    }

    fd = open(path.as_ptr(), O_WRONLY | O_CLOEXEC | O_NOFOLLOW);
    if fd < 0 {
        return -1;
    }

    ret = write_nointr(fd, buf as *const ::core::ffi::c_void, buf_size) as i32;
    close_prot_errno_disarm!(fd);
    if ret != buf_size as i32 {
        return -1;
    }

    0
}

unsafe fn change_userns(_metadata: *mut __test_metadata, syncfds: *mut i32) {
    let mut ret: i32;
    let mut buf: ::core::ffi::c_char = 0;

    close_prot_errno_disarm!(*syncfds.add(1));

    ret = unshare(CLONE_NEWUSER);
    ASSERT_EQ!(ret, 0, {
        TH_LOG!(
            "%s - Failed to unshare user namespace",
            strerror(errno)
        );
    });

    ret = write_nointr(*syncfds.add(0), c_str!("1").as_ptr() as *const ::core::ffi::c_void, 1) as i32;
    ASSERT_EQ!(ret, 1, {
        TH_LOG!("write_nointr() failed");
    });

    ret = read_nointr(*syncfds.add(0), &mut buf as *mut _ as *mut ::core::ffi::c_void, 1) as i32;
    ASSERT_EQ!(ret, 1, {
        TH_LOG!("read_nointr() failed");
    });

    close_prot_errno_disarm!(*syncfds.add(0));

    ASSERT_EQ!(setid_userns_root(), 0, {
        TH_LOG!("setid_userns_root() failed");
    });
}

unsafe fn change_idmaps(_metadata: *mut __test_metadata, syncfds: *mut i32, pid: pid_t) {
    let mut ret: i32;
    let mut buf: ::core::ffi::c_char = 0;
    let mut id_map = [0 as ::core::ffi::c_char; 4096];

    close_prot_errno_disarm!(*syncfds.add(0));

    ret = read_nointr(*syncfds.add(1), &mut buf as *mut _ as *mut ::core::ffi::c_void, 1) as i32;
    ASSERT_EQ!(ret, 1, {
        TH_LOG!("read_nointr() failed");
    });

    snprintf(
        id_map.as_mut_ptr(),
        id_map.len(),
        c_str!("0 %d 1\n").as_ptr(),
        getuid(),
    );
    ret = write_id_mapping(
        idmap_type::UID_MAP,
        pid,
        id_map.as_ptr(),
        strlen(id_map.as_ptr()),
    );
    ASSERT_EQ!(ret, 0, {
        TH_LOG!("write_id_mapping(UID_MAP) failed");
    });

    snprintf(
        id_map.as_mut_ptr(),
        id_map.len(),
        c_str!("0 %d 1\n").as_ptr(),
        getgid(),
    );
    ret = write_id_mapping(
        idmap_type::GID_MAP,
        pid,
        id_map.as_ptr(),
        strlen(id_map.as_ptr()),
    );
    ASSERT_EQ!(ret, 0, {
        TH_LOG!("write_id_mapping(GID_MAP) failed");
    });

    ret = write_nointr(*syncfds.add(1), c_str!("1").as_ptr() as *const ::core::ffi::c_void, 1) as i32;
    ASSERT_EQ!(ret, 1, {
        TH_LOG!("write_nointr() failed");
    });

    close_prot_errno_disarm!(*syncfds.add(1));
}

static mut _thread_metadata: *mut __test_metadata = ::core::ptr::null_mut();

unsafe extern "C" fn binder_version_thread(data: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    let _metadata: *mut __test_metadata = _thread_metadata;
    let fd: i32 = ptr_to_int!(data);
    let mut version: binder_version = ::core::mem::zeroed();
    let mut ret: i32;

    ret = ioctl(fd, BINDER_VERSION, &mut version);
    if ret < 0 {
        TH_LOG!(
            "%s - Failed to open perform BINDER_VERSION request\n",
            strerror(errno)
        );
    }

    pthread_exit(data);
}

/*
 * Regression test:
 * 2669b8b0c798 ("binder: prevent UAF for binderfs devices")
 * f0fe2c0f050d ("binder: prevent UAF for binderfs devices II")
 * 211b64e4b5b6 ("binderfs: use refcount for binder control devices too")
 */
TEST!(binderfs_stress, {
    let mut fds = [0_i32; 1000];
    let mut syncfds = [0_i32; 2];
    let mut pid: pid_t;
    let mut fd: i32;
    let mut ret: i32;
    let _len: usize;
    let mut device: binderfs_device = unsafe { ::core::mem::zeroed() };
    let mut binderfs_mntpt = *b"/tmp/binderfs_XXXXXX\0";
    let mut device_path = [0 as ::core::ffi::c_char; b"/tmp/binderfs_XXXXXX/".len() + BINDERFS_MAX_NAME];

    unsafe {
        ret = socketpair(PF_LOCAL, SOCK_STREAM | SOCK_CLOEXEC, 0, syncfds.as_mut_ptr());
        ASSERT_EQ!(ret, 0, {
            TH_LOG!("%s - Failed to create socket pair", strerror(errno));
        });

        pid = fork();
        ASSERT_GE!(pid, 0, {
            TH_LOG!("%s - Failed to fork", strerror(errno));
            close_prot_errno_disarm!(syncfds[0]);
            close_prot_errno_disarm!(syncfds[1]);
        });

        if pid == 0 {
            let mut i: i32;
            let mut j: i32;
            let mut k: i32;
            let mut nthreads: i32;
            let mut attr: pthread_attr_t = ::core::mem::zeroed();
            let mut threads = [::core::mem::zeroed::<pthread_t>(); DEFAULT_THREADS];
            change_userns(_metadata, syncfds.as_mut_ptr());
            change_mountns(_metadata);

            ASSERT_NE!(mkdtemp(binderfs_mntpt.as_mut_ptr() as *mut ::core::ffi::c_char), ::core::ptr::null_mut(), {
                TH_LOG!(
                    "%s - Failed to create binderfs mountpoint",
                    strerror(errno)
                );
            });

            ret = mount(
                ::core::ptr::null(),
                binderfs_mntpt.as_ptr() as *const ::core::ffi::c_char,
                c_str!("binder").as_ptr(),
                0,
                0 as *const ::core::ffi::c_void,
            );
            ASSERT_EQ!(ret, 0, {
                TH_LOG!(
                    "%s - Failed to mount binderfs, check if CONFIG_ANDROID_BINDERFS is enabled in the running kernel",
                    strerror(errno)
                );
            });

            for i_usize in 0..fds.len() {
                i = i_usize as i32;

                snprintf(
                    device_path.as_mut_ptr(),
                    device_path.len(),
                    c_str!("%s/binder-control").as_ptr(),
                    binderfs_mntpt.as_ptr(),
                );
                fd = open(device_path.as_ptr(), O_RDONLY | O_CLOEXEC);
                ASSERT_GE!(fd, 0, {
                    TH_LOG!(
                        "%s - Failed to open binder-control device",
                        strerror(errno)
                    );
                });

                memset(
                    &mut device as *mut _ as *mut ::core::ffi::c_void,
                    0,
                    ::core::mem::size_of::<binderfs_device>(),
                );
                snprintf(
                    device.name.as_mut_ptr(),
                    device.name.len(),
                    c_str!("%d").as_ptr(),
                    i,
                );
                ret = ioctl(fd, BINDER_CTL_ADD, &mut device);
                close_prot_errno_disarm!(fd);
                ASSERT_EQ!(ret, 0, {
                    TH_LOG!(
                        "%s - Failed to allocate new binder device",
                        strerror(errno)
                    );
                });

                snprintf(
                    device_path.as_mut_ptr(),
                    device_path.len(),
                    c_str!("%s/%d").as_ptr(),
                    binderfs_mntpt.as_ptr(),
                    i,
                );
                fds[i_usize] = open(device_path.as_ptr(), O_RDONLY | O_CLOEXEC);
                ASSERT_GE!(fds[i_usize], 0, {
                    TH_LOG!("%s - Failed to open binder device", strerror(errno));
                });
            }

            ret = umount2(binderfs_mntpt.as_ptr() as *const ::core::ffi::c_char, MNT_DETACH);
            ASSERT_EQ!(ret, 0, {
                TH_LOG!("%s - Failed to unmount binderfs", strerror(errno));
                rmdir(binderfs_mntpt.as_ptr() as *const ::core::ffi::c_char);
            });

            nthreads = get_nprocs_conf();
            if nthreads > DEFAULT_THREADS as i32 {
                nthreads = DEFAULT_THREADS as i32;
            }

            _thread_metadata = _metadata;
            pthread_attr_init(&mut attr);
            for k_usize in 0..fds.len() {
                k = k_usize as i32;
                i = 0;
                while i < nthreads {
                    ret = pthread_create(
                        &mut threads[i as usize],
                        &attr,
                        Some(binder_version_thread),
                        int_to_ptr!(fds[k_usize]),
                    );
                    if ret != 0 {
                        TH_LOG!(
                            "%s - Failed to create thread %d",
                            strerror(errno),
                            i
                        );
                        break;
                    }
                    i += 1;
                }

                j = 0;
                while j < i {
                    let mut fdptr: *mut ::core::ffi::c_void = ::core::ptr::null_mut();

                    ret = pthread_join(threads[j as usize], &mut fdptr);
                    if ret != 0 {
                        TH_LOG!(
                            "%s - Failed to join thread %d for fd %d",
                            strerror(errno),
                            j,
                            ptr_to_int!(fdptr)
                        );
                    }
                    j += 1;
                }
            }
            pthread_attr_destroy(&mut attr);

            for k_usize in 0..fds.len() {
                k = k_usize as i32;
                close(fds[k_usize]);
            }

            exit(EXIT_SUCCESS);
        }

        change_idmaps(_metadata, syncfds.as_mut_ptr(), pid);

        ret = wait_for_pid(pid);
        ASSERT_EQ!(ret, 0, {
            TH_LOG!("wait_for_pid() failed");
        });
    }
});

TEST!(binderfs_test_privileged, {
    unsafe {
        if geteuid() != 0 {
            SKIP!(return, "Tests are not run as root. Skipping privileged tests");
        }

        if __do_binderfs_test(_metadata) != 0 {
            SKIP!(return, "The Android binderfs filesystem is not available");
        }
    }
});

TEST!(binderfs_test_unprivileged, {
    let mut ret: i32;
    let mut syncfds = [0_i32; 2];
    let mut pid: pid_t;

    unsafe {
        ret = socketpair(PF_LOCAL, SOCK_STREAM | SOCK_CLOEXEC, 0, syncfds.as_mut_ptr());
        ASSERT_EQ!(ret, 0, {
            TH_LOG!("%s - Failed to create socket pair", strerror(errno));
        });

        pid = fork();
        ASSERT_GE!(pid, 0, {
            close_prot_errno_disarm!(syncfds[0]);
            close_prot_errno_disarm!(syncfds[1]);
            TH_LOG!("%s - Failed to fork", strerror(errno));
        });

        if pid == 0 {
            change_userns(_metadata, syncfds.as_mut_ptr());
            if __do_binderfs_test(_metadata) != 0 {
                exit(2);
            }
            exit(EXIT_SUCCESS);
        }

        change_idmaps(_metadata, syncfds.as_mut_ptr(), pid);

        ret = wait_for_pid(pid);
        if ret != 0 {
            if ret == 2 {
                SKIP!(return, "The Android binderfs filesystem is not available");
            }
            ASSERT_EQ!(ret, 0, {
                TH_LOG!("wait_for_pid() failed");
            });
        }
    }
});

TEST_HARNESS_MAIN!();
