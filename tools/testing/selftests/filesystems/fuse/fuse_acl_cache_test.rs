// SPDX-License-Identifier: GPL-2.0
/*
 * Test: FUSE ACL caching bug triggered by AT_STATX_FORCE_SYNC
 *
 * A FUSE mount that does not negotiate FUSE_POSIX_ACL initialises every inode
 * with i_acl = i_default_acl = ACL_DONT_CACHE.  When a fresh stat is needed
 * (e.g. AT_STATX_FORCE_SYNC), fuse_update_get_attr() calls
 * forget_all_cached_acls() before issuing FUSE_GETATTR.  On an unfixed kernel,
 * __forget_cached_acl() replaces ACL_DONT_CACHE with ACL_NOT_CACHED,
 * inadvertently enabling the kernel ACL cache for that inode.  The next
 * getxattr populates the cache.  Because fuse_set_acl() skips
 * forget_all_cached_acls() for !fc->posix_acl mounts, any subsequent change to
 * the ACL leaves the stale kernel entry in place, and the next getxattr returns
 * wrong data without ever reaching the FUSE daemon.
 *
 * Fix (fs/posix_acl.c): __forget_cached_acl() returns early when *p is
 * ACL_DONT_CACHE, preserving the "never cache" invariant for the inode's
 * lifetime.
 *
 * Test outline:
 *  1. Mount a minimal FUSE fs (no FUSE_POSIX_ACL negotiated).
 *  2. lgetxattr -> daemon called, ACL_A returned, NOT cached (ACL_DONT_CACHE).
 *  3. statx(AT_STATX_FORCE_SYNC) -> forget_all_cached_acls() called.
 *     Buggy:  ACL_DONT_CACHE -> ACL_NOT_CACHED (cache enabled).
 *     Fixed:  ACL_DONT_CACHE preserved.
 *  4. lgetxattr -> daemon called, ACL_A returned.
 *     Buggy:  result now cached (ACL_NOT_CACHED -> cached).
 *     Fixed:  result still not cached.
 *  5. Daemon switches to ACL_B internally (different size).
 *  6. lgetxattr -> should return ACL_B (44 bytes).
 *     Buggy:  cache hit, returns stale ACL_A (28 bytes). FAIL.
 *     Fixed:  no cache, daemon called, returns ACL_B (44 bytes). PASS.
 */

/*
 * C dependencies translated as external declarations:
 * errno.h, fcntl.h, linux/limits.h, pthread.h, stdint.h, stdio.h, stdlib.h,
 * string.h, sys/stat.h, sys/xattr.h, unistd.h, fuse_lowlevel.h, and
 * kselftest_harness.h.
 *
 * C defined FUSE_USE_VERSION as 31 before including fuse_lowlevel.h.
 */

use core::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type fuse_ino_t = u64;
type fuse_req_t = *mut c_void;
type pthread_t = c_ulong;

const FUSE_USE_VERSION: c_int = 31;
const FILE_INO: fuse_ino_t = 2;
const FILE_NAME: &[u8] = b"testfile\0";

const FUSE_ROOT_ID: fuse_ino_t = 1;
const ENOENT: c_int = 2;
const ERANGE: c_int = 34;
const ENODATA: c_int = 61;
const AT_FDCWD: c_int = -100;
const AT_STATX_FORCE_SYNC: c_int = 0x2000;
const STATX_BASIC_STATS: c_uint = 0x0000_07ff;
const PATH_MAX: usize = 4096;
const S_IFDIR: c_uint = 0o040000;
const S_IFREG: c_uint = 0o100000;

#[repr(C)]
struct pthread_mutex_t {
    __private: [u8; 40],
}

#[repr(C)]
struct fuse_session {
    _private: [u8; 0],
}

#[repr(C)]
struct fuse_file_info {
    _private: [u8; 0],
}

#[repr(C)]
struct stat {
    st_dev: c_ulong,
    st_ino: c_ulong,
    st_nlink: c_ulong,
    st_mode: c_uint,
    st_uid: c_uint,
    st_gid: c_uint,
    __pad0: c_int,
    st_rdev: c_ulong,
    st_size: c_long,
    st_blksize: c_long,
    st_blocks: c_long,
    __private: [u8; 48],
}

#[repr(C)]
struct statx {
    _private: [u8; 256],
}

#[repr(C)]
struct fuse_entry_param {
    ino: fuse_ino_t,
    generation: u64,
    attr: stat,
    attr_timeout: c_double,
    entry_timeout: c_double,
}

#[repr(C)]
struct fuse_args {
    argc: c_int,
    argv: *mut *mut c_char,
    allocated: c_int,
}

type fuse_lookup_fn = Option<unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *const c_char)>;
type fuse_getattr_fn =
    Option<unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *mut fuse_file_info)>;
type fuse_getxattr_fn =
    Option<unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *const c_char, size_t)>;

#[repr(C)]
struct fuse_lowlevel_ops {
    lookup: fuse_lookup_fn,
    getattr: fuse_getattr_fn,
    getxattr: fuse_getxattr_fn,
}

unsafe extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn snprintf(str: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn mkdtemp(template: *mut c_char) -> *mut c_char;
    fn rmdir(path: *const c_char) -> c_int;
    fn statx(
        dirfd: c_int,
        pathname: *const c_char,
        flags: c_int,
        mask: c_uint,
        statxbuf: *mut statx,
    ) -> c_int;
    fn lgetxattr(
        path: *const c_char,
        name: *const c_char,
        value: *mut c_void,
        size: size_t,
    ) -> ssize_t;

    fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn fuse_reply_err(req: fuse_req_t, err: c_int) -> c_int;
    fn fuse_reply_entry(req: fuse_req_t, e: *const fuse_entry_param) -> c_int;
    fn fuse_reply_attr(req: fuse_req_t, attr: *const stat, attr_timeout: c_double) -> c_int;
    fn fuse_reply_xattr(req: fuse_req_t, count: size_t) -> c_int;
    fn fuse_reply_buf(req: fuse_req_t, buf: *const c_char, size: size_t) -> c_int;
    fn fuse_session_loop(se: *mut fuse_session) -> c_int;
    fn fuse_session_new(
        args: *mut fuse_args,
        op: *const fuse_lowlevel_ops,
        op_size: size_t,
        userdata: *mut c_void,
    ) -> *mut fuse_session;
    fn fuse_session_mount(se: *mut fuse_session, mountpoint: *const c_char) -> c_int;
    fn fuse_session_unmount(se: *mut fuse_session);
    fn fuse_session_destroy(se: *mut fuse_session);
    fn fuse_session_exit(se: *mut fuse_session);
    fn fuse_opt_free_args(args: *mut fuse_args);
}

unsafe extern "C" {
    static mut errno: c_int;
}

unsafe extern "C" {
    fn ksft_skip(return_value: (), fmt: *const c_char, ...);
    fn th_log(fmt: *const c_char, ...);
    fn assert_eq_ssize(left: ssize_t, right: ssize_t);
    fn assert_eq_int(left: c_int, right: c_int);
    fn expect_eq_ssize(left: ssize_t, right: ssize_t);
    fn expect_eq_int(left: c_int, right: c_int);
}

/* ---- ACL binary encoding ------------------------------------------------ */
/*
 * POSIX ACL v2 xattr format (little-endian):
 *   header: u32 version (= 0x00000002)
 *   entry:  u16 tag | u16 perm | u32 id
 *
 * Entries must appear in tag-ascending order; named USER/GROUP entries
 * require a MASK entry.  Both ACLs pass posix_acl_from_xattr() validation.
 */

/* ACL_A: 3 entries (USER_OBJ:rwx, GROUP_OBJ:r-x, OTHER:r-x) = 28 bytes */
static ACL_A: [u8; 28] = [
    0x02, 0x00, 0x00, 0x00, /* v2 header      */
    0x01, 0x00, 0x07, 0x00, 0xff, 0xff, 0xff, 0xff, /* USER_OBJ  rwx  */
    0x04, 0x00, 0x05, 0x00, 0xff, 0xff, 0xff, 0xff, /* GROUP_OBJ r-x  */
    0x20, 0x00, 0x05, 0x00, 0xff, 0xff, 0xff, 0xff, /* OTHER     r-x  */
];

/*
 * ACL_B: 5 entries - adds USER uid=1 and MASK = 44 bytes.
 * A named USER entry requires a MASK; all tags in ascending order.
 */
static ACL_B: [u8; 44] = [
    0x02, 0x00, 0x00, 0x00, /* v2 header       */
    0x01, 0x00, 0x07, 0x00, 0xff, 0xff, 0xff, 0xff, /* USER_OBJ   rwx  */
    0x02, 0x00, 0x07, 0x00, 0x01, 0x00, 0x00, 0x00, /* USER uid=1 rwx  */
    0x04, 0x00, 0x05, 0x00, 0xff, 0xff, 0xff, 0xff, /* GROUP_OBJ  r-x  */
    0x10, 0x00, 0x07, 0x00, 0xff, 0xff, 0xff, 0xff, /* MASK       rwx  */
    0x20, 0x00, 0x05, 0x00, 0xff, 0xff, 0xff, 0xff, /* OTHER      r-x  */
];

/* ---- Shared state (daemon thread <-> test thread) ----------------------- */

#[repr(C)]
struct daemon_state {
    lock: pthread_mutex_t,
    acl: *const u8,
    acl_size: size_t,
    getxattr_count: c_int,
}

/*
 * Global: callbacks are stateless fns so we use a single global.
 * Safe because only one test instance runs at a time.
 */
static mut g_ds: daemon_state = daemon_state {
    lock: pthread_mutex_t { __private: [0; 40] },
    acl: ptr::null(),
    acl_size: 0,
    getxattr_count: 0,
};

/* ---- FUSE lowlevel callbacks -------------------------------------------- */

unsafe extern "C" fn fs_lookup(req: fuse_req_t, parent: fuse_ino_t, name: *const c_char) {
    if parent != FUSE_ROOT_ID || strcmp(name, FILE_NAME.as_ptr() as *const c_char) != 0 {
        fuse_reply_err(req, ENOENT);
        return;
    }
    let mut e: fuse_entry_param = core::mem::zeroed();

    /*
     * Long attr/entry timeouts so that normal stat() calls do not
     * expire and trigger forget_all_cached_acls() on their own;
     * only the explicit AT_STATX_FORCE_SYNC should trigger it.
     */
    e.ino = FILE_INO;
    e.generation = 1;
    e.attr_timeout = 10.0;
    e.entry_timeout = 10.0;
    e.attr.st_ino = FILE_INO as c_ulong;
    e.attr.st_mode = S_IFREG | 0o644;
    e.attr.st_nlink = 1;
    fuse_reply_entry(req, &e);
}

unsafe extern "C" fn fs_getattr(req: fuse_req_t, ino: fuse_ino_t, fi: *mut fuse_file_info) {
    let mut st: stat = core::mem::zeroed();

    let _ = fi;
    if ino == FUSE_ROOT_ID {
        st.st_ino = FUSE_ROOT_ID as c_ulong;
        st.st_mode = S_IFDIR | 0o755;
        st.st_nlink = 2;
    } else if ino == FILE_INO {
        st.st_ino = FILE_INO as c_ulong;
        st.st_mode = S_IFREG | 0o644;
        st.st_nlink = 1;
    } else {
        fuse_reply_err(req, ENOENT);
        return;
    }
    fuse_reply_attr(req, &st, 10.0);
}

unsafe extern "C" fn fs_getxattr(
    req: fuse_req_t,
    ino: fuse_ino_t,
    name: *const c_char,
    size: size_t,
) {
    if ino != FILE_INO || strcmp(name, b"system.posix_acl_access\0".as_ptr() as *const c_char) != 0
    {
        fuse_reply_err(req, ENODATA);
        return;
    }

    pthread_mutex_lock(&raw mut g_ds.lock);
    let acl: *const u8 = g_ds.acl;
    let acl_size: size_t = g_ds.acl_size;
    g_ds.getxattr_count += 1;
    pthread_mutex_unlock(&raw mut g_ds.lock);

    if size == 0 {
        fuse_reply_xattr(req, acl_size);
    } else if size < acl_size {
        fuse_reply_err(req, ERANGE);
    } else {
        fuse_reply_buf(req, acl as *const c_char, acl_size);
    }
}

static fs_ops: fuse_lowlevel_ops = fuse_lowlevel_ops {
    lookup: Some(fs_lookup),
    getattr: Some(fs_getattr),
    getxattr: Some(fs_getxattr),
};

/* ---- Daemon thread ------------------------------------------------------- */

unsafe extern "C" fn run_daemon(arg: *mut c_void) -> *mut c_void {
    fuse_session_loop(arg as *mut fuse_session);
    ptr::null_mut()
}

/* ---- kselftest harness --------------------------------------------------- */

#[repr(C)]
struct acl_cache {
    se: *mut fuse_session,
    mountpoint: [c_char; PATH_MAX],
    file_path: [c_char; PATH_MAX],
    thread: pthread_t,
}

unsafe fn fixture_setup_acl_cache(self_: *mut acl_cache) {
    let mut arg0 = *b"fuse_acl_cache_test\0";
    let mut fuse_argv: [*mut c_char; 2] = [arg0.as_mut_ptr() as *mut c_char, ptr::null_mut()];
    let mut args = fuse_args {
        argc: 1,
        argv: fuse_argv.as_mut_ptr(),
        allocated: 0,
    };

    g_ds.acl = ACL_A.as_ptr();
    g_ds.acl_size = size_of::<[u8; 28]>();
    g_ds.getxattr_count = 0;

    strcpy(
        (*self_).mountpoint.as_mut_ptr(),
        b"/tmp/acl_cache_test_XXXXXX\0".as_ptr() as *const c_char,
    );
    if mkdtemp((*self_).mountpoint.as_mut_ptr()).is_null() {
        ksft_skip(
            (),
            b"mkdtemp: %s\0".as_ptr() as *const c_char,
            strerror(errno),
        );
        return;
    }

    snprintf(
        (*self_).file_path.as_mut_ptr(),
        size_of::<[c_char; PATH_MAX]>(),
        b"%s/testfile\0".as_ptr() as *const c_char,
        (*self_).mountpoint.as_ptr(),
    );

    (*self_).se = fuse_session_new(&mut args, &fs_ops, size_of::<fuse_lowlevel_ops>(), ptr::null_mut());
    if (*self_).se.is_null() {
        rmdir((*self_).mountpoint.as_ptr());
        ksft_skip((), b"fuse_session_new failed\0".as_ptr() as *const c_char);
        return;
    }

    if fuse_session_mount((*self_).se, (*self_).mountpoint.as_ptr()) != 0 {
        fuse_session_destroy((*self_).se);
        rmdir((*self_).mountpoint.as_ptr());
        ksft_skip(
            (),
            b"fuse_session_mount failed (missing fusermount3 or insufficient privileges)\0"
                .as_ptr() as *const c_char,
        );
        return;
    }

    if pthread_create(
        &mut (*self_).thread,
        ptr::null(),
        run_daemon,
        (*self_).se as *mut c_void,
    ) != 0
    {
        fuse_session_unmount((*self_).se);
        fuse_session_destroy((*self_).se);
        rmdir((*self_).mountpoint.as_ptr());
        ksft_skip(
            (),
            b"pthread_create: %s\0".as_ptr() as *const c_char,
            strerror(errno),
        );
        return;
    }

    fuse_opt_free_args(&mut args);
}

unsafe fn fixture_teardown_acl_cache(self_: *mut acl_cache) {
    fuse_session_exit((*self_).se);
    fuse_session_unmount((*self_).se);
    pthread_join((*self_).thread, ptr::null_mut());
    fuse_session_destroy((*self_).se);
    rmdir((*self_).mountpoint.as_ptr());
}

unsafe fn do_force_statx(path: *const c_char) -> c_int {
    let mut stx: statx = core::mem::zeroed();

    statx(
        AT_FDCWD,
        path,
        AT_STATX_FORCE_SYNC,
        STATX_BASIC_STATS,
        &mut stx,
    )
}

unsafe fn test_acl_cache_stale_after_force_sync(self_: *mut acl_cache) {
    let mut buf: [c_char; 512] = [0; 512];
    let mut sz: ssize_t;
    let mut count: c_int;

    /*
     * Step 1: two getxattr calls before any statx(FORCE_SYNC).
     * i_acl == ACL_DONT_CACHE.  __get_acl's cmpxchg(p, ACL_NOT_CACHED,
     * sentinel) finds *p != ACL_NOT_CACHED on every call, so the sentinel
     * is never placed and the result is never cached.  Both calls must
     * reach the daemon, proving ACL_DONT_CACHE suppresses caching.
     */
    sz = lgetxattr(
        (*self_).file_path.as_ptr(),
        b"system.posix_acl_access\0".as_ptr() as *const c_char,
        buf.as_mut_ptr() as *mut c_void,
        size_of::<[c_char; 512]>(),
    );
    assert_eq_ssize(sz, size_of::<[u8; 28]>() as ssize_t);

    sz = lgetxattr(
        (*self_).file_path.as_ptr(),
        b"system.posix_acl_access\0".as_ptr() as *const c_char,
        buf.as_mut_ptr() as *mut c_void,
        size_of::<[c_char; 512]>(),
    );
    assert_eq_ssize(sz, size_of::<[u8; 28]>() as ssize_t);

    pthread_mutex_lock(&raw mut g_ds.lock);
    count = g_ds.getxattr_count;
    pthread_mutex_unlock(&raw mut g_ds.lock);

    assert_eq_int(count, 2);
    th_log(
        b"step 1 OK: both pre-trigger getxattrs reached daemon (count=%d), ACL_DONT_CACHE is working\0"
            .as_ptr() as *const c_char,
        count,
    );

    /*
     * Step 2: statx(AT_STATX_FORCE_SYNC).
     * fuse_update_get_attr() calls forget_all_cached_acls() before sending
     * FUSE_GETATTR.
     *   Buggy kernel:  ACL_DONT_CACHE -> ACL_NOT_CACHED  (cache enabled)
     *   Fixed kernel:  ACL_DONT_CACHE preserved           (no effect)
     */
    assert_eq_int(do_force_statx((*self_).file_path.as_ptr()), 0);
    th_log(b"step 2 OK: statx(AT_STATX_FORCE_SYNC) succeeded\0".as_ptr() as *const c_char);

    /*
     * Step 3: getxattr - cache population attempt after the trigger.
     *   Buggy:  *p == ACL_NOT_CACHED -> sentinel placed -> fuse_get_inode_acl
     *           called -> ACL_A parsed and stored in the kernel cache.
     *   Fixed:  *p == ACL_DONT_CACHE -> sentinel placement skipped ->
     *           fuse_get_inode_acl called but result not cached.
     * Either way the correct ACL_A is returned here.
     */
    sz = lgetxattr(
        (*self_).file_path.as_ptr(),
        b"system.posix_acl_access\0".as_ptr() as *const c_char,
        buf.as_mut_ptr() as *mut c_void,
        size_of::<[c_char; 512]>(),
    );
    assert_eq_ssize(sz, size_of::<[u8; 28]>() as ssize_t);

    pthread_mutex_lock(&raw mut g_ds.lock);
    count = g_ds.getxattr_count;
    pthread_mutex_unlock(&raw mut g_ds.lock);

    assert_eq_int(count, 3);
    th_log(
        b"step 3 OK: post-trigger getxattr reached daemon (count=%d), returned correct ACL_A (%zd bytes)\0"
            .as_ptr() as *const c_char,
        count,
        sz,
    );

    /*
     * Step 4: switch daemon to ACL_B (different size: 44 vs 28 bytes).
     * Simulates an ACL change that fuse_set_acl() would NOT invalidate for
     * !fc->posix_acl mounts (it skips forget_all_cached_acls in that case).
     * On a fixed kernel the ACL was never cached, so this is moot.
     */
    pthread_mutex_lock(&raw mut g_ds.lock);
    g_ds.acl = ACL_B.as_ptr();
    g_ds.acl_size = size_of::<[u8; 44]>();
    pthread_mutex_unlock(&raw mut g_ds.lock);
    th_log(
        b"step 4: daemon switched to ACL_B (%zu bytes)\0".as_ptr() as *const c_char,
        size_of::<[u8; 44]>(),
    );

    /*
     * Step 5: getxattr - the decisive check.
     *   Buggy kernel:  cache hit -> stale ACL_A (28 bytes), count stays 3.
     *   Fixed kernel:  no cache -> daemon called -> ACL_B (44 bytes), count 4.
     */
    sz = lgetxattr(
        (*self_).file_path.as_ptr(),
        b"system.posix_acl_access\0".as_ptr() as *const c_char,
        buf.as_mut_ptr() as *mut c_void,
        size_of::<[c_char; 512]>(),
    );

    pthread_mutex_lock(&raw mut g_ds.lock);
    count = g_ds.getxattr_count;
    pthread_mutex_unlock(&raw mut g_ds.lock);

    if sz == size_of::<[u8; 28]>() as ssize_t {
        th_log(
            b"step 5 BUG: stale ACL_A (%zd bytes) from kernel cache (count=%d); ACL_DONT_CACHE corrupted by forget_all_cached_acls()\0"
                .as_ptr() as *const c_char,
            sz,
            count,
        );
    } else {
        th_log(
            b"step 5 OK: daemon reached (count=%d), fresh ACL_B (%zd bytes)\0"
                .as_ptr() as *const c_char,
            count,
            sz,
        );
    }

    expect_eq_ssize(sz, size_of::<[u8; 44]>() as ssize_t);
    expect_eq_int(count, 4);
}

/*
 * TEST_HARNESS_MAIN
 *
 * The original C file expands kselftest harness macros:
 *   FIXTURE(acl_cache)
 *   FIXTURE_SETUP(acl_cache)
 *   FIXTURE_TEARDOWN(acl_cache)
 *   TEST_F(acl_cache, stale_after_force_sync)
 *   TEST_HARNESS_MAIN
 * Their generated entry points are external to this isolated source-level
 * translation.
 */
