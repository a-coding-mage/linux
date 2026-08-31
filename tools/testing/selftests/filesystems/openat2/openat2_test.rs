// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Author: Aleksa Sarai <cyphar@cyphar.com>
 * Copyright (C) 2018-2019 SUSE LLC.
 */

// C dependencies translated as external requirements:
// #define _GNU_SOURCE
// #define __SANE_USERSPACE_TYPES__ // Use ll64
// #include <fcntl.h>
// #include <sched.h>
// #include <sys/stat.h>
// #include <sys/types.h>
// #include <sys/mount.h>
// #include <stdlib.h>
// #include <stdbool.h>
// #include <string.h>
//
// #include "helpers.h"
// #include "kselftest_harness.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type uint32_t = u32;
type __u64 = u64;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct open_how {
    pub flags: __u64,
    pub mode: __u64,
    pub resolve: __u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct open_how_ext {
    inner: open_how,
    extra1: uint32_t,
    pad1: [c_char; 128],
    extra2: uint32_t,
    pad2: [c_char; 128],
    extra3: uint32_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct struct_test {
    name: *const c_char,
    arg: open_how_ext,
    size: size_t,
    err: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct flag_test {
    name: *const c_char,
    how: open_how,
    err: c_int,
}

/*
 * O_LARGEFILE is set to 0 by glibc.
 * XXX: This is wrong on {mips, parisc, powerpc, sparc}.
 */
#[cfg(target_arch = "aarch64")]
const O_LARGEFILE: __u64 = 0x20000;
#[cfg(not(target_arch = "aarch64"))]
const O_LARGEFILE: __u64 = 0x8000;

const OPENAT2_REGULAR: __u64 = 1u64 << 32;
const EFTYPE: c_int = 134;

/* Kernel-internal carrier for OPENAT2_REGULAR (see __O_REGULAR in fcntl.h). */
const __O_REGULAR: c_int = 1 << 30;

unsafe extern "C" {
    static openat2_supported: bool;
    static _metadata: c_int;

    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn close(fd: c_int) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, ...) -> c_int;

    fn raw_openat2(dirfd: c_int, pathname: *const c_char, how: *const c_void, size: size_t) -> c_int;
    fn sys_openat2(dirfd: c_int, pathname: *const c_char, how: *const open_how) -> c_int;
    fn fdreadlink(metadata: c_int, fd: c_int) -> *mut c_char;

    static mut errno: c_int;
}

// External constants normally supplied by libc, Linux UAPI headers, helpers.h,
// and kselftest_harness.h.
extern "Rust" {
    static AT_FDCWD: c_int;
    static O_RDONLY: __u64;
    static O_RDWR: __u64;
    static O_CREAT: __u64;
    static O_EXCL: __u64;
    static O_NOCTTY: __u64;
    static O_TMPFILE: __u64;
    static O_PATH: __u64;
    static O_CLOEXEC: __u64;
    static O_DIRECTORY: __u64;
    static O_NOFOLLOW: __u64;
    static O_DIRECT: __u64;
    static F_GETFL: c_int;
    static F_GETFD: c_int;
    static FD_CLOEXEC: c_int;
    static EINVAL: c_int;
    static E2BIG: c_int;
    static EOPNOTSUPP: c_int;
    static ENOENT: c_int;
    static OPEN_HOW_SIZE_VER0: size_t;
    static RESOLVE_BENEATH: __u64;
    static RESOLVE_IN_ROOT: __u64;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn fixture_setup_openat2() {
    if !openat2_supported {
        SKIP!(return, "openat2(2) not supported");
    }
}

unsafe fn fixture_teardown_openat2() {}

/*
 * Verify that the struct size and misalignment handling for openat2(2) is
 * correct, including that is_zeroed_user() works.
 */
unsafe fn test_f_openat2_struct_argument_sizes() {
    let misalignments: [c_int; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 17, 87];
    let tests: [struct_test; 8] = [
        /* Normal struct. */
        struct_test {
            name: cstr!("normal struct"),
            arg: open_how_ext {
                inner: open_how { flags: O_RDONLY, mode: 0, resolve: 0 },
                extra1: 0,
                pad1: [0; 128],
                extra2: 0,
                pad2: [0; 128],
                extra3: 0,
            },
            size: size_of::<open_how>(),
            err: 0,
        },
        /* Bigger struct, with zeroed out end. */
        struct_test {
            name: cstr!("bigger struct (zeroed out)"),
            arg: open_how_ext {
                inner: open_how { flags: O_RDONLY, mode: 0, resolve: 0 },
                extra1: 0,
                pad1: [0; 128],
                extra2: 0,
                pad2: [0; 128],
                extra3: 0,
            },
            size: size_of::<open_how_ext>(),
            err: 0,
        },

        /* TODO: Once expanded, check zero-padding. */

        /* Smaller than version-0 struct. */
        struct_test {
            name: cstr!("zero-sized 'struct'"),
            arg: open_how_ext {
                inner: open_how { flags: O_RDONLY, mode: 0, resolve: 0 },
                extra1: 0,
                pad1: [0; 128],
                extra2: 0,
                pad2: [0; 128],
                extra3: 0,
            },
            size: 0,
            err: -EINVAL,
        },
        struct_test {
            name: cstr!("smaller-than-v0 struct"),
            arg: open_how_ext {
                inner: open_how { flags: O_RDONLY, mode: 0, resolve: 0 },
                extra1: 0,
                pad1: [0; 128],
                extra2: 0,
                pad2: [0; 128],
                extra3: 0,
            },
            size: OPEN_HOW_SIZE_VER0 - 1,
            err: -EINVAL,
        },

        /* Bigger struct, with non-zero trailing bytes. */
        struct_test {
            name: cstr!("bigger struct (non-zero data in first 'future field')"),
            arg: open_how_ext {
                inner: open_how { flags: O_RDONLY, mode: 0, resolve: 0 },
                extra1: 0xdeadbeef,
                pad1: [0; 128],
                extra2: 0,
                pad2: [0; 128],
                extra3: 0,
            },
            size: size_of::<open_how_ext>(),
            err: -E2BIG,
        },
        struct_test {
            name: cstr!("bigger struct (non-zero data in middle of 'future fields')"),
            arg: open_how_ext {
                inner: open_how { flags: O_RDONLY, mode: 0, resolve: 0 },
                extra1: 0,
                pad1: [0; 128],
                extra2: 0xfeedcafe,
                pad2: [0; 128],
                extra3: 0,
            },
            size: size_of::<open_how_ext>(),
            err: -E2BIG,
        },
        struct_test {
            name: cstr!("bigger struct (non-zero data at end of 'future fields')"),
            arg: open_how_ext {
                inner: open_how { flags: O_RDONLY, mode: 0, resolve: 0 },
                extra1: 0,
                pad1: [0; 128],
                extra2: 0,
                pad2: [0; 128],
                extra3: 0xabad1dea,
            },
            size: size_of::<open_how_ext>(),
            err: -E2BIG,
        },
    ];

    for i in 0..tests.len() {
        let test: *const struct_test = &tests[i];
        let how_ext: open_how_ext = (*test).arg;

        for j in 0..misalignments.len() {
            let mut fd: c_int;
            let misalign: c_int = misalignments[j];
            let mut copy: *mut c_void = ptr::null_mut();
            let mut how_copy: *mut c_void = &how_ext as *const open_how_ext as *mut c_void;
            let mut fdpath: *mut c_char = ptr::null_mut();

            if misalign != 0 {
                /*
                 * Explicitly misalign the structure copying it with the given
                 * (mis)alignment offset. The other data is set to be non-zero to
                 * make sure that non-zero bytes outside the struct aren't checked
                 *
                 * This is effectively to check that is_zeroed_user() works.
                 */
                copy = malloc(misalign as size_t + size_of::<open_how_ext>());
                how_copy = (copy as *mut u8).add(misalign as usize) as *mut c_void;
                memset(copy, 0xff, misalign as size_t);
                memcpy(
                    how_copy,
                    &how_ext as *const open_how_ext as *const c_void,
                    size_of::<open_how_ext>(),
                );
            }

            fd = raw_openat2(AT_FDCWD, cstr!("."), how_copy, (*test).size);
            if fd >= 0 {
                fdpath = fdreadlink(_metadata, fd);
                close(fd);
            }

            if (*test).err >= 0 {
                EXPECT_GE!(fd, 0, {
                    TH_LOG!(
                        "openat2 with %s [misalign=%d] should succeed, got %d (%s)",
                        (*test).name,
                        misalign,
                        fd,
                        strerror(-fd)
                    );
                });
            } else {
                EXPECT_EQ!((*test).err, fd, {
                    if !fdpath.is_null() {
                        TH_LOG!(
                            "openat2 with %s [misalign=%d] should fail with %d (%s), got %d['%s']",
                            (*test).name,
                            misalign,
                            (*test).err,
                            strerror(-(*test).err),
                            fd,
                            fdpath
                        );
                    } else {
                        TH_LOG!(
                            "openat2 with %s [misalign=%d] should fail with %d (%s), got %d (%s)",
                            (*test).name,
                            misalign,
                            (*test).err,
                            strerror(-(*test).err),
                            fd,
                            strerror(-fd)
                        );
                    }
                });
            }

            free(copy);
            free(fdpath as *mut c_void);
        }
    }
}

/* Verify openat2(2) flag and mode validation. */
unsafe fn test_f_openat2_flag_validation() {
    let tests: [flag_test; 28] = [
        /* O_TMPFILE is incompatible with O_PATH and O_CREAT. */
        flag_test {
            name: cstr!("incompatible flags (O_TMPFILE | O_PATH)"),
            how: open_how { flags: O_TMPFILE | O_PATH | O_RDWR, mode: 0, resolve: 0 },
            err: -EINVAL,
        },
        flag_test {
            name: cstr!("incompatible flags (O_TMPFILE | O_CREAT)"),
            how: open_how { flags: O_TMPFILE | O_CREAT | O_RDWR, mode: 0, resolve: 0 },
            err: -EINVAL,
        },

        /* O_PATH only permits certain other flags to be set ... */
        flag_test {
            name: cstr!("compatible flags (O_PATH | O_CLOEXEC)"),
            how: open_how { flags: O_PATH | O_CLOEXEC, mode: 0, resolve: 0 },
            err: 0,
        },
        flag_test {
            name: cstr!("compatible flags (O_PATH | O_DIRECTORY)"),
            how: open_how { flags: O_PATH | O_DIRECTORY, mode: 0, resolve: 0 },
            err: 0,
        },
        flag_test {
            name: cstr!("compatible flags (O_PATH | O_NOFOLLOW)"),
            how: open_how { flags: O_PATH | O_NOFOLLOW, mode: 0, resolve: 0 },
            err: 0,
        },
        /* ... and others are absolutely not permitted. */
        flag_test {
            name: cstr!("incompatible flags (O_PATH | O_RDWR)"),
            how: open_how { flags: O_PATH | O_RDWR, mode: 0, resolve: 0 },
            err: -EINVAL,
        },
        flag_test {
            name: cstr!("incompatible flags (O_PATH | O_CREAT)"),
            how: open_how { flags: O_PATH | O_CREAT, mode: 0, resolve: 0 },
            err: -EINVAL,
        },
        flag_test {
            name: cstr!("incompatible flags (O_PATH | O_EXCL)"),
            how: open_how { flags: O_PATH | O_EXCL, mode: 0, resolve: 0 },
            err: -EINVAL,
        },
        flag_test {
            name: cstr!("incompatible flags (O_PATH | O_NOCTTY)"),
            how: open_how { flags: O_PATH | O_NOCTTY, mode: 0, resolve: 0 },
            err: -EINVAL,
        },
        flag_test {
            name: cstr!("incompatible flags (O_PATH | O_DIRECT)"),
            how: open_how { flags: O_PATH | O_DIRECT, mode: 0, resolve: 0 },
            err: -EINVAL,
        },
        flag_test {
            name: cstr!("incompatible flags (O_PATH | O_LARGEFILE)"),
            how: open_how { flags: O_PATH | O_LARGEFILE, mode: 0, resolve: 0 },
            err: -EINVAL,
        },

        /* ->mode must only be set with O_{CREAT,TMPFILE}. */
        flag_test {
            name: cstr!("non-zero how.mode and O_RDONLY"),
            how: open_how { flags: O_RDONLY, mode: 0o600, resolve: 0 },
            err: -EINVAL,
        },
        flag_test {
            name: cstr!("non-zero how.mode and O_PATH"),
            how: open_how { flags: O_PATH, mode: 0o600, resolve: 0 },
            err: -EINVAL,
        },
        flag_test {
            name: cstr!("valid how.mode and O_CREAT"),
            how: open_how { flags: O_CREAT, mode: 0o600, resolve: 0 },
            err: 0,
        },
        flag_test {
            name: cstr!("valid how.mode and O_TMPFILE"),
            how: open_how { flags: O_TMPFILE | O_RDWR, mode: 0o600, resolve: 0 },
            err: 0,
        },
        /* ->mode must only contain 0777 bits. */
        flag_test {
            name: cstr!("invalid how.mode and O_CREAT"),
            how: open_how { flags: O_CREAT, mode: 0xFFFF, resolve: 0 },
            err: -EINVAL,
        },
        flag_test {
            name: cstr!("invalid (very large) how.mode and O_CREAT"),
            how: open_how { flags: O_CREAT, mode: 0xC000000000000000u64, resolve: 0 },
            err: -EINVAL,
        },
        flag_test {
            name: cstr!("invalid how.mode and O_TMPFILE"),
            how: open_how { flags: O_TMPFILE | O_RDWR, mode: 0x1337, resolve: 0 },
            err: -EINVAL,
        },
        flag_test {
            name: cstr!("invalid (very large) how.mode and O_TMPFILE"),
            how: open_how { flags: O_TMPFILE | O_RDWR, mode: 0x0000A00000000000u64, resolve: 0 },
            err: -EINVAL,
        },

        /* ->resolve flags must not conflict. */
        flag_test {
            name: cstr!("incompatible resolve flags (BENEATH | IN_ROOT)"),
            how: open_how {
                flags: O_RDONLY,
                mode: 0,
                resolve: RESOLVE_BENEATH | RESOLVE_IN_ROOT,
            },
            err: -EINVAL,
        },

        /* ->resolve must only contain RESOLVE_* flags. */
        flag_test {
            name: cstr!("invalid how.resolve and O_RDONLY"),
            how: open_how { flags: O_RDONLY, mode: 0, resolve: 0x1337 },
            err: -EINVAL,
        },
        flag_test {
            name: cstr!("invalid how.resolve and O_CREAT"),
            how: open_how { flags: O_CREAT, mode: 0, resolve: 0x1337 },
            err: -EINVAL,
        },
        flag_test {
            name: cstr!("invalid how.resolve and O_TMPFILE"),
            how: open_how { flags: O_TMPFILE | O_RDWR, mode: 0, resolve: 0x1337 },
            err: -EINVAL,
        },
        flag_test {
            name: cstr!("invalid how.resolve and O_PATH"),
            how: open_how { flags: O_PATH, mode: 0, resolve: 0x1337 },
            err: -EINVAL,
        },

        /* currently unknown upper 32 bit rejected. */
        flag_test {
            name: cstr!("currently unknown bit (1 << 63)"),
            how: open_how { flags: O_RDONLY | (1u64 << 63), mode: 0, resolve: 0 },
            err: -EINVAL,
        },
    ];

    for i in 0..tests.len() {
        let mut fd: c_int;
        let mut fdflags: c_int = -1;
        let path: *const c_char;
        let mut fdpath: *mut c_char = ptr::null_mut();
        let test: *const flag_test = &tests[i];

        path = if ((*test).how.flags & O_CREAT) != 0 {
            cstr!("/tmp/ksft.openat2_tmpfile")
        } else {
            cstr!(".")
        };
        unlink(path);

        fd = sys_openat2(AT_FDCWD, path, &(*test).how);
        if fd < 0 && fd == -EOPNOTSUPP {
            /*
             * Skip the testcase if it failed because not supported
             * by FS. (e.g. a valid O_TMPFILE combination on NFS)
             */
            TH_LOG!(
                "openat2 with %s not supported by FS -- skipping",
                (*test).name
            );
            continue;
        }

        if (*test).err >= 0 {
            EXPECT_GE!(fd, 0, {
                TH_LOG!(
                    "openat2 with %s should succeed, got %d (%s)",
                    (*test).name,
                    fd,
                    strerror(-fd)
                );
            });
            if fd >= 0 {
                let mut otherflags: c_int;

                fdpath = fdreadlink(_metadata, fd);
                fdflags = fcntl(fd, F_GETFL);
                otherflags = fcntl(fd, F_GETFD);
                close(fd);

                ASSERT_GE!(fdflags, 0);
                ASSERT_GE!(otherflags, 0);

                /* O_CLOEXEC isn't shown in F_GETFL. */
                if (otherflags & FD_CLOEXEC) != 0 {
                    fdflags |= O_CLOEXEC as c_int;
                }
                /* O_CREAT is hidden from F_GETFL. */
                if ((*test).how.flags & O_CREAT) != 0 {
                    fdflags |= O_CREAT as c_int;
                }
                if ((*test).how.flags & O_LARGEFILE) == 0 {
                    fdflags &= !(O_LARGEFILE as c_int);
                }

                EXPECT_EQ!(fdflags, (*test).how.flags as c_int, {
                    TH_LOG!(
                        "openat2 with %s: flags mismatch %X != %llX",
                        (*test).name,
                        fdflags,
                        (*test).how.flags as u64
                    );
                });
            }
        } else {
            EXPECT_EQ!((*test).err, fd, {
                if fd >= 0 {
                    fdpath = fdreadlink(_metadata, fd);
                    TH_LOG!(
                        "openat2 with %s should fail with %d (%s), got %d['%s']",
                        (*test).name,
                        (*test).err,
                        strerror(-(*test).err),
                        fd,
                        fdpath
                    );
                } else {
                    TH_LOG!(
                        "openat2 with %s should fail with %d (%s), got %d (%s)",
                        (*test).name,
                        (*test).err,
                        strerror(-(*test).err),
                        fd,
                        strerror(-fd)
                    );
                }
            });
            if fd >= 0 {
                close(fd);
            }
        }

        free(fdpath as *mut c_void);
    }
}

/* Verify that OPENAT2_REGULAR rejects non-regular files with EFTYPE. */
unsafe fn test_f_openat2_regular_flag() {
    let how: open_how = open_how {
        flags: OPENAT2_REGULAR | O_RDONLY,
        mode: 0,
        resolve: 0,
    };
    let mut fd: c_int;

    fd = sys_openat2(AT_FDCWD, cstr!("/dev/null"), &how);
    if fd == -ENOENT {
        SKIP!(return, "/dev/null does not exist");
    }

    EXPECT_EQ!(-EFTYPE, fd, {
        TH_LOG!(
            "openat2 with OPENAT2_REGULAR should fail with %d (%s), got %d (%s)",
            -EFTYPE,
            strerror(EFTYPE),
            fd,
            strerror(-fd)
        );
    });
    if fd >= 0 {
        close(fd);
    }
}

/* open()/openat() must keep ignoring the internal __O_REGULAR bit. */
unsafe fn test_legacy_openat_ignores_o_regular() {
    let mut fd: c_int;

    fd = openat(AT_FDCWD, cstr!("/dev/null"), O_RDONLY as c_int | __O_REGULAR);
    if fd < 0 && errno == ENOENT {
        SKIP!(return, "/dev/null does not exist");
    }

    ASSERT_GE!(fd, 0, {
        TH_LOG!(
            "legacy openat() must ignore the __O_REGULAR carrier bit, got errno %d (%s)",
            errno,
            strerror(errno)
        );
    });
    close(fd);
}

// TEST_HARNESS_MAIN
