// SPDX-License-Identifier: GPL-2.0
/*
 * An 'F' entry keeps its interpreter open for as long as the entry exists,
 * and the entry only goes away when the binfmt_misc superblock is destroyed.
 * An interpreter that lives on a mount which in turn keeps that superblock
 * alive therefore pins the instance that owns it, and nothing can break the
 * cycle. Check the two ways userspace could arrange for that: an interpreter
 * on the binfmt_misc instance itself, and one on a filesystem stacked on it.
 *
 * Runs unprivileged in a user namespace; binfmt_misc is FS_USERNS_MOUNT.
 */

/* C dependencies: fcntl.h, limits.h, sched.h, sys/mount.h, sys/stat.h,
 * ../filesystems/utils.h, kselftest_harness.h
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

const MNT: &str = "/tmp/binfmt_selfpin";
const BACKING: &str = "/tmp/binfmt_selfpin_back";
const LOWER: &str = "/tmp/binfmt_selfpin_back/lower";
const MERGED: &str = "/tmp/binfmt_selfpin_merged";

const MAGIC: &str = "\\xde\\xad";
/* Not on the instance, and unlike /bin/true it always exists. */
const INTERP: &str = "/proc/self/exe";

const PATH_MAX: usize = 4096;
const OPTS_MAX: usize = 3 * PATH_MAX + 64;

const F_OK: c_int = 0;
const O_WRONLY: c_int = 1;
const EACCES: c_int = 13;
const EPERM: c_int = 1;
const ENODEV: c_int = 19;
const MNT_DETACH: c_int = 2;

#[repr(C)]
pub struct __test_metadata {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn access(path: *const c_char, mode: c_int) -> c_int;
    fn chmod(path: *const c_char, mode: c_ulong) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn mkdir(path: *const c_char, mode: c_ulong) -> c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn rmdir(path: *const c_char) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn umount(target: *const c_char) -> c_int;
    fn umount2(target: *const c_char, flags: c_int) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;

    fn setup_userns() -> c_int;
}

unsafe extern "C" {
    static mut errno: c_int;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! rule {
    ($interp:expr) => {
        concat!(":selfpin:M::", MAGIC, "::", $interp, ":F\0").as_ptr() as *const c_char
    };
}

/* kselftest harness macros supplied externally in the original C source. */
macro_rules! ASSERT_GE {
    ($($tokens:tt)*) => {
        compile_error!("external kselftest_harness ASSERT_GE is required")
    };
}

macro_rules! ASSERT_EQ {
    ($($tokens:tt)*) => {
        compile_error!("external kselftest_harness ASSERT_EQ is required")
    };
}

macro_rules! ASSERT_NE {
    ($($tokens:tt)*) => {
        compile_error!("external kselftest_harness ASSERT_NE is required")
    };
}

macro_rules! EXPECT_EQ {
    ($($tokens:tt)*) => {
        compile_error!("external kselftest_harness EXPECT_EQ is required")
    };
}

macro_rules! EXPECT_NE {
    ($($tokens:tt)*) => {
        compile_error!("external kselftest_harness EXPECT_NE is required")
    };
}

macro_rules! SKIP {
    ($($tokens:tt)*) => {
        compile_error!("external kselftest_harness SKIP is required")
    };
}

macro_rules! FIXTURE {
    ($name:ident $body:block) => {
        struct $name;
    };
}

macro_rules! FIXTURE_SETUP {
    ($name:ident $body:block) => {
        unsafe fn setup($name: *mut $name) $body
    };
}

macro_rules! FIXTURE_TEARDOWN {
    ($name:ident $body:block) => {
        unsafe fn teardown($name: *mut $name) $body
    };
}

macro_rules! TEST_F {
    ($fixture:ident, $name:ident $body:block) => {
        unsafe fn $name(_metadata: *mut __test_metadata, selfpin: *mut $fixture) $body
    };
}

macro_rules! TEST_HARNESS_MAIN {
    () => {};
}

unsafe fn ensure_dir(path: *const c_char) -> c_int {
    if mkdir(path, 0o755) != 0 && errno != libc_errno_EEXIST() {
        return -1;
    }
    0
}

const fn libc_errno_EEXIST() -> c_int {
    17
}

/* Write @rule to this instance's register file, preserving write(2)'s errno. */
unsafe fn register_at(_metadata: *mut __test_metadata, rule: *const c_char) -> c_int {
    let fd: c_int;
    let saved: c_int;
    let n: isize;

    fd = open(cstr!("/tmp/binfmt_selfpin/register"), O_WRONLY);
    ASSERT_GE!(fd, 0);
    n = write(fd, rule as *const c_void, strlen(rule));
    saved = errno;
    close(fd);
    errno = saved;
    if n < 0 { -1 } else { 0 }
}

/*
 * Mount an overlay over @lower using a private upper/work pair, so the two
 * mounts this test performs cannot interfere with each other and neither
 * overlaps the lower layer.
 */
unsafe fn mount_overlay(lower: *const c_char, nr: c_int) -> c_int {
    let mut opts = [0 as c_char; OPTS_MAX];
    let mut upper = [0 as c_char; PATH_MAX];
    let mut work = [0 as c_char; PATH_MAX];

    snprintf(
        upper.as_mut_ptr(),
        upper.len(),
        cstr!("%s/upper%d"),
        cstr!("/tmp/binfmt_selfpin_back"),
        nr,
    );
    snprintf(
        work.as_mut_ptr(),
        work.len(),
        cstr!("%s/work%d"),
        cstr!("/tmp/binfmt_selfpin_back"),
        nr,
    );
    if mkdir(upper.as_ptr(), 0o755) != 0 || mkdir(work.as_ptr(), 0o755) != 0 {
        return -1;
    }

    snprintf(
        opts.as_mut_ptr(),
        opts.len(),
        cstr!("lowerdir=%s,upperdir=%s,workdir=%s"),
        lower,
        upper.as_ptr(),
        work.as_ptr(),
    );
    mount(
        cstr!("ovl"),
        cstr!("/tmp/binfmt_selfpin_merged"),
        cstr!("overlay"),
        0,
        opts.as_ptr() as *const c_void,
    )
}

FIXTURE!(selfpin {});

FIXTURE_SETUP!(selfpin {
    /* setup_userns() exits rather than returns if this is not there. */
    if access(cstr!("/proc/self/ns/user"), F_OK) != 0 {
        SKIP!(return, "kernel without user namespaces");
    }
    ASSERT_EQ!(setup_userns(), 0);

    ASSERT_EQ!(ensure_dir(cstr!("/tmp/binfmt_selfpin")), 0);
    if mount(
        cstr!("binfmt_misc"),
        cstr!("/tmp/binfmt_selfpin"),
        cstr!("binfmt_misc"),
        0,
        core::ptr::null(),
    ) != 0
    {
        let saved = errno;

        /* Teardown doesn't run when setup skips, so clean up here. */
        rmdir(cstr!("/tmp/binfmt_selfpin"));
        SKIP!(return, "no binfmt_misc: %s", strerror(saved));
    }
});

FIXTURE_TEARDOWN!(selfpin {
    /* The namespaces go with the process; just don't litter /tmp. */
    umount2(cstr!("/tmp/binfmt_selfpin_merged"), MNT_DETACH);
    umount2(cstr!("/tmp/binfmt_selfpin_back"), MNT_DETACH);
    umount2(cstr!("/tmp/binfmt_selfpin"), MNT_DETACH);
    rmdir(cstr!("/tmp/binfmt_selfpin_merged"));
    rmdir(cstr!("/tmp/binfmt_selfpin_back"));
    rmdir(cstr!("/tmp/binfmt_selfpin"));
});

/*
 * The instance's own files are regular files the mounter owns, so they can be
 * made executable. Opening one for exec still has to fail, otherwise the entry
 * pins the very superblock it lives in.
 */
TEST_F!(selfpin, interpreter_on_the_instance {
    ASSERT_EQ!(chmod(cstr!("/tmp/binfmt_selfpin/status"), 0o755), 0);

    ASSERT_NE!(
        register_at(_metadata, rule!("/tmp/binfmt_selfpin/status")),
        0
    );
    EXPECT_EQ!(errno, EACCES);
});

/* Same for an entry file rather than one of the control files. */
TEST_F!(selfpin, interpreter_on_an_entry {
    ASSERT_EQ!(
        register_at(
            _metadata,
            cstr!(":victim:M::\\xde\\xad::/proc/self/exe:")
        ),
        0
    );
    ASSERT_EQ!(chmod(cstr!("/tmp/binfmt_selfpin/victim"), 0o755), 0);

    ASSERT_NE!(
        register_at(_metadata, rule!("/tmp/binfmt_selfpin/victim")),
        0
    );
    EXPECT_EQ!(errno, EACCES);
});

/*
 * A stacking filesystem holds a private clone of each layer for its whole
 * lifetime, so an instance used as a layer can be pinned by an interpreter
 * that does not live on it at all. Refuse to be a layer.
 */
TEST_F!(selfpin, refuses_to_be_stacked_on {
    ASSERT_EQ!(ensure_dir(cstr!("/tmp/binfmt_selfpin_back")), 0);
    ASSERT_EQ!(
        mount(
            cstr!("tmpfs"),
            cstr!("/tmp/binfmt_selfpin_back"),
            cstr!("tmpfs"),
            0,
            core::ptr::null(),
        ),
        0
    );
    ASSERT_EQ!(mkdir(cstr!("/tmp/binfmt_selfpin_back/lower"), 0o755), 0);
    ASSERT_EQ!(ensure_dir(cstr!("/tmp/binfmt_selfpin_merged")), 0);

    /* Nothing to prove unless overlayfs works here at all. */
    if mount_overlay(cstr!("/tmp/binfmt_selfpin_back/lower"), 1) != 0 {
        if errno == ENODEV || errno == EPERM {
            SKIP!(return, "no unprivileged overlayfs");
        }
        SKIP!(return, "overlayfs unusable here: %s", strerror(errno));
    }
    ASSERT_EQ!(umount(cstr!("/tmp/binfmt_selfpin_merged")), 0);

    EXPECT_NE!(mount_overlay(cstr!("/tmp/binfmt_selfpin"), 2), 0);
});

/* An ordinary interpreter still registers with 'F'. */
TEST_F!(selfpin, ordinary_interpreter_still_works {
    EXPECT_EQ!(register_at(_metadata, rule!("/proc/self/exe")), 0);
});

TEST_HARNESS_MAIN!();
