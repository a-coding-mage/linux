// SPDX-License-Identifier: GPL-2.0
/*
 * A pre-opened interpreter - what 'F' gives a static entry and what a 'B'
 * entry binds - keeps a file open for as long as the entry lives, so it pins
 * the mount it came from. It costs no file descriptor, and binfmt_misc is
 * FS_USERNS_MOUNT, so an unprivileged user namespace can create them without
 * bound. Check that UCOUNT_BINFMT_MISC_INTERPRETERS bounds it, that an entry
 * that pre-opens nothing is not charged, that removing an entry gives the
 * charge back, and that nesting a user namespace does not evade it.
 *
 * Runs unprivileged in a user namespace.
 */

/* C dependencies: errno.h, fcntl.h, limits.h, stdio.h, string.h,
 * sys/mount.h, sys/stat.h, unistd.h, ../filesystems/utils.h,
 * and kselftest_harness.h.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

const MNT: &[u8] = b"/tmp/binfmt_interplimit\0";
const NESTED_MNT: &[u8] = b"/tmp/binfmt_interplimit_nested\0";
const LIMIT_SYSCTL: &[u8] = b"/proc/sys/user/max_binfmt_misc_interpreters\0";

const MAGIC: &str = "\\xde\\xad";
/* Not on the instance, and unlike /bin/true it always exists. */
const INTERP: &str = "/proc/self/exe";

/* Small enough to fill by hand, big enough that a refund is visible. */
const LIMIT: c_uint = 4;

/* What UCOUNT_ENTRY() lets a namespace raise its own limit to. */
const LIMIT_MAX: &[u8] = b"2147483647\0";

const PATH_MAX: usize = 4096;
const O_WRONLY: c_int = 1;
const O_CLOEXEC: c_int = 0o2000000;
const F_OK: c_int = 0;
const ENOENT: c_int = 2;
const ENOSPC: c_int = 28;
const EEXIST: c_int = 17;
const MNT_DETACH: c_int = 2;

unsafe extern "C" {
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn mkdir(pathname: *const c_char, mode: c_uint) -> c_int;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn rmdir(pathname: *const c_char) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn umount2(target: *const c_char, flags: c_int) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;

    fn __errno_location() -> *mut c_int;

    fn setup_userns() -> c_int;
}

unsafe fn errno_get() -> c_int {
    *__errno_location()
}

unsafe fn errno_set(val: c_int) {
    *__errno_location() = val;
}

unsafe fn ensure_dir(path: *const c_char) -> c_int {
    if mkdir(path, 0o755) != 0 && errno_get() != EEXIST {
        return -1;
    }
    0
}

/* Write @val to @path, preserving write(2)'s errno for the caller. */
unsafe fn write_keep_errno(path: *const c_char, val: *const c_char) -> c_int {
    let fd: c_int;
    let saved: c_int;
    let n: isize;

    fd = open(path, O_WRONLY | O_CLOEXEC);
    if fd < 0 {
        return -1;
    }
    n = write(fd, val as *const c_void, strlen(val));
    saved = errno_get();
    close(fd);
    errno_set(saved);
    if n < 0 { -1 } else { 0 }
}

unsafe fn set_limit(val: *const c_char) -> c_int {
    write_keep_errno(LIMIT_SYSCTL.as_ptr() as *const c_char, val)
}

unsafe fn register_at(mnt: *const c_char, rule: *const c_char) -> c_int {
    let mut path: [c_char; PATH_MAX] = [0; PATH_MAX];

    snprintf(
        path.as_mut_ptr(),
        path.len(),
        b"%s/register\0".as_ptr() as *const c_char,
        mnt,
    );
    write_keep_errno(path.as_ptr(), rule)
}

/* An 'F' entry: one interpreter pre-opened at registration, one charge. */
unsafe fn register_fixed(mnt: *const c_char, name: *const c_char) -> c_int {
    let mut rule: [c_char; PATH_MAX] = [0; PATH_MAX];

    snprintf(
        rule.as_mut_ptr(),
        rule.len(),
        b":%s:M::\\xde\\xad::/proc/self/exe:F\0".as_ptr() as *const c_char,
        name,
    );
    register_at(mnt, rule.as_ptr())
}

/* The same entry without 'F': the interpreter is opened per exec instead. */
unsafe fn register_plain(mnt: *const c_char, name: *const c_char) -> c_int {
    let mut rule: [c_char; PATH_MAX] = [0; PATH_MAX];

    snprintf(
        rule.as_mut_ptr(),
        rule.len(),
        b":%s:M::\\xde\\xad::/proc/self/exe:\0".as_ptr() as *const c_char,
        name,
    );
    register_at(mnt, rule.as_ptr())
}

unsafe fn remove_entry(mnt: *const c_char, name: *const c_char) -> c_int {
    let mut path: [c_char; PATH_MAX] = [0; PATH_MAX];

    snprintf(
        path.as_mut_ptr(),
        path.len(),
        b"%s/%s\0".as_ptr() as *const c_char,
        mnt,
        name,
    );
    write_keep_errno(path.as_ptr(), b"-1\n\0".as_ptr() as *const c_char)
}

unsafe fn entry_exists(mnt: *const c_char, name: *const c_char) -> bool {
    let mut path: [c_char; PATH_MAX] = [0; PATH_MAX];

    snprintf(
        path.as_mut_ptr(),
        path.len(),
        b"%s/%s\0".as_ptr() as *const c_char,
        mnt,
        name,
    );
    access(path.as_ptr(), F_OK) == 0
}

/* Register @n 'F' entries, each with a name of its own. */
unsafe fn fill_budget(mnt: *const c_char, n: c_uint) -> c_int {
    let mut name: [c_char; 32] = [0; 32];
    let mut i: c_uint;

    i = 0;
    while i < n {
        snprintf(
            name.as_mut_ptr(),
            name.len(),
            b"fixed%u\0".as_ptr() as *const c_char,
            i,
        );
        if register_fixed(mnt, name.as_ptr()) != 0 {
            return -1;
        }
        i += 1;
    }
    0
}

FIXTURE!(interp_limit, {});

FIXTURE_SETUP!(interp_limit, {
    unsafe {
        /* setup_userns() exits rather than returns if this is not there. */
        if access(b"/proc/self/ns/user\0".as_ptr() as *const c_char, F_OK) != 0 {
            SKIP!(return, "kernel without user namespaces");
        }
        ASSERT_EQ!(setup_userns(), 0);

        /* CAP_SYS_RESOURCE in this namespace is what makes it writable. */
        if set_limit(LIMIT_MAX.as_ptr() as *const c_char) != 0 {
            if errno_get() == ENOENT {
                SKIP!(return, "kernel without /proc/sys/user/max_binfmt_misc_interpreters");
            }
            SKIP!(
                return,
                "cannot set the limit: %s",
                strerror(errno_get())
            );
        }

        ASSERT_EQ!(ensure_dir(MNT.as_ptr() as *const c_char), 0);
        if mount(
            b"binfmt_misc\0".as_ptr() as *const c_char,
            MNT.as_ptr() as *const c_char,
            b"binfmt_misc\0".as_ptr() as *const c_char,
            0,
            core::ptr::null(),
        ) != 0
        {
            let saved: c_int = errno_get();

            /* Teardown doesn't run when setup skips, so clean up here. */
            rmdir(MNT.as_ptr() as *const c_char);
            SKIP!(return, "no binfmt_misc: %s", strerror(saved));
        }
    }
});

FIXTURE_TEARDOWN!(interp_limit, {
    unsafe {
        /* The namespaces go with the process; just don't litter /tmp. */
        umount2(NESTED_MNT.as_ptr() as *const c_char, MNT_DETACH);
        umount2(MNT.as_ptr() as *const c_char, MNT_DETACH);
        rmdir(NESTED_MNT.as_ptr() as *const c_char);
        rmdir(MNT.as_ptr() as *const c_char);
    }
});

/* Every pre-opened interpreter is charged, and the budget is a hard stop. */
TEST_F!(interp_limit, fixed_interpreters_are_charged, {
    unsafe {
        let mut buf: [c_char; 32] = [0; 32];

        snprintf(
            buf.as_mut_ptr(),
            buf.len(),
            b"%u\0".as_ptr() as *const c_char,
            LIMIT,
        );
        ASSERT_EQ!(set_limit(buf.as_ptr()), 0);

        ASSERT_EQ!(fill_budget(MNT.as_ptr() as *const c_char, LIMIT), 0);

        EXPECT_NE!(
            register_fixed(
                MNT.as_ptr() as *const c_char,
                b"over\0".as_ptr() as *const c_char
            ),
            0
        );
        EXPECT_EQ!(errno_get(), ENOSPC);

        /* A refused registration leaves nothing behind. */
        EXPECT_FALSE!(entry_exists(
            MNT.as_ptr() as *const c_char,
            b"over\0".as_ptr() as *const c_char
        ));
    }
});

/* An entry that pre-opens nothing pins nothing, so it is not charged. */
TEST_F!(interp_limit, plain_entries_are_not_charged, {
    unsafe {
        ASSERT_EQ!(set_limit(b"0\0".as_ptr() as *const c_char), 0);

        EXPECT_EQ!(
            register_plain(
                MNT.as_ptr() as *const c_char,
                b"plain\0".as_ptr() as *const c_char
            ),
            0
        );
        EXPECT_TRUE!(entry_exists(
            MNT.as_ptr() as *const c_char,
            b"plain\0".as_ptr() as *const c_char
        ));

        /* ... while the same entry with 'F' has nothing to spend. */
        EXPECT_NE!(
            register_fixed(
                MNT.as_ptr() as *const c_char,
                b"fixed\0".as_ptr() as *const c_char
            ),
            0
        );
        EXPECT_EQ!(errno_get(), ENOSPC);
    }
});

/* Removing an entry closes its interpreters and gives the charge back. */
TEST_F!(interp_limit, removal_refunds_the_charge, {
    unsafe {
        let mut buf: [c_char; 32] = [0; 32];

        snprintf(
            buf.as_mut_ptr(),
            buf.len(),
            b"%u\0".as_ptr() as *const c_char,
            LIMIT,
        );
        ASSERT_EQ!(set_limit(buf.as_ptr()), 0);

        ASSERT_EQ!(fill_budget(MNT.as_ptr() as *const c_char, LIMIT), 0);
        ASSERT_NE!(
            register_fixed(
                MNT.as_ptr() as *const c_char,
                b"over\0".as_ptr() as *const c_char
            ),
            0
        );

        ASSERT_EQ!(
            remove_entry(
                MNT.as_ptr() as *const c_char,
                b"fixed0\0".as_ptr() as *const c_char
            ),
            0
        );
        EXPECT_EQ!(
            register_fixed(
                MNT.as_ptr() as *const c_char,
                b"over\0".as_ptr() as *const c_char
            ),
            0
        );
    }
});

/*
 * The charge walks the ancestors, so a namespace cannot buy itself budget by
 * nesting: it may raise only its own limit, and the parent it was created
 * from is charged for every binding made below it.
 */
TEST_F!(interp_limit, nesting_does_not_evade_it, {
    unsafe {
        let mut buf: [c_char; 32] = [0; 32];

        snprintf(
            buf.as_mut_ptr(),
            buf.len(),
            b"%u\0".as_ptr() as *const c_char,
            LIMIT,
        );
        ASSERT_EQ!(set_limit(buf.as_ptr()), 0);
        ASSERT_EQ!(fill_budget(MNT.as_ptr() as *const c_char, LIMIT), 0);

        ASSERT_EQ!(setup_userns(), 0);
        ASSERT_EQ!(set_limit(LIMIT_MAX.as_ptr() as *const c_char), 0);

        ASSERT_EQ!(ensure_dir(NESTED_MNT.as_ptr() as *const c_char), 0);
        ASSERT_EQ!(
            mount(
                b"binfmt_misc\0".as_ptr() as *const c_char,
                NESTED_MNT.as_ptr() as *const c_char,
                b"binfmt_misc\0".as_ptr() as *const c_char,
                0,
                core::ptr::null(),
            ),
            0
        );

        /* A fresh instance with an unlimited budget of its own, and yet: */
        EXPECT_NE!(
            register_fixed(
                NESTED_MNT.as_ptr() as *const c_char,
                b"nested\0".as_ptr() as *const c_char
            ),
            0
        );
        EXPECT_EQ!(errno_get(), ENOSPC);

        /* The nested instance works for anything that pins no file. */
        EXPECT_EQ!(
            register_plain(
                NESTED_MNT.as_ptr() as *const c_char,
                b"nested_plain\0".as_ptr() as *const c_char
            ),
            0
        );
    }
});

TEST_HARNESS_MAIN!();

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
