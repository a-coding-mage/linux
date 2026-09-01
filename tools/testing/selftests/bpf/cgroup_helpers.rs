// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/bpf/cgroup_helpers.c.
// Original C dependencies included sched, mount/stat/types/xattr, linux limits/sched,
// stdio/stdlib, fcntl, unistd, ftw, cgroup_helpers.h, and bpf_util.h.

use libc::{
    c_char, c_int, c_long, c_uint, c_ulong, c_void, FILE, FTW, O_RDONLY, O_RDWR, O_WRONLY,
    AT_FDCWD, EBUSY, EEXIST, EINVAL, MS_PRIVATE, MS_REC, PATH_MAX,
};
use std::cell::Cell;
use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;

/*
 * To avoid relying on the system setup, when setup_cgroup_env is called
 * we create a new mount namespace, and cgroup namespace. The cgroupv2
 * root is mounted at CGROUP_MOUNT_PATH. Unfortunately, most people don't
 * have cgroupv2 enabled at this point in time. It's easier to create our
 * own mount namespace and manage it ourselves. We assume /mnt exists.
 *
 * Related cgroupv1 helpers are named *classid*(), since we only use the
 * net_cls controller for tagging net_cls.classid. We assume the default
 * mount under /sys/fs/cgroup/net_cls, which should be the case for the
 * vast majority of users.
 */

const WALK_FD_LIMIT: c_int = 16;

const CGROUP_MOUNT_PATH: &[u8] = b"/mnt\0";
const CGROUP_MOUNT_DFLT: &[u8] = b"/sys/fs/cgroup\0";
const NETCLS_MOUNT_PATH: &[u8] = b"/sys/fs/cgroup/net_cls\0";
const CGROUP_WORK_DIR: &[u8] = b"/cgroup-test-work-dir\0";

const FTW_D: c_int = 1;
const FTW_DEPTH: c_int = 8;
const FTW_MOUNT: c_int = 2;
const CLONE_NEWNS: c_int = 0x0002_0000;

thread_local! {
    static CGROUP_WORKDIR_MOUNTED: Cell<bool> = const { Cell::new(false) };
}

#[repr(C)]
struct file_handle {
    handle_bytes: c_uint,
    handle_type: c_int,
    f_handle: [u8; 0],
}

unsafe extern "C" {
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn dprintf(fd: c_int, format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn mkdir(pathname: *const c_char, mode: libc::mode_t) -> c_int;
    fn rmdir(pathname: *const c_char) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn umount(target: *const c_char) -> c_int;
    fn nftw(
        dirpath: *const c_char,
        fn_: Option<
            unsafe extern "C" fn(
                *const c_char,
                *const libc::stat,
                c_int,
                *mut FTW,
            ) -> c_int,
        >,
        nopenfd: c_int,
        flags: c_int,
    ) -> c_int;
    fn getpid() -> libc::pid_t;
    fn getppid() -> libc::pid_t;
    fn setxattr(
        path: *const c_char,
        name: *const c_char,
        value: *const c_void,
        size: usize,
        flags: c_int,
    ) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strtok_r(
        str_: *mut c_char,
        delim: *const c_char,
        saveptr: *mut *mut c_char,
    ) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn name_to_handle_at(
        dirfd: c_int,
        pathname: *const c_char,
        handle: *mut file_handle,
        mount_id: *mut c_int,
        flags: c_int,
    ) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn fclose(stream: *mut FILE) -> c_int;
    static mut stderr: *mut FILE;
}

unsafe extern "C" {
    fn log_err(format: *const c_char, ...);
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
}

unsafe fn errno_value() -> c_int {
    *libc::__errno_location()
}

unsafe fn format_cgroup_path_pid(buf: *mut c_char, size: usize, path: *const c_char, pid: libc::pid_t) {
    snprintf(
        buf,
        size,
        c"%s%s%d%s".as_ptr(),
        CGROUP_MOUNT_PATH.as_ptr() as *const c_char,
        CGROUP_WORK_DIR.as_ptr() as *const c_char,
        pid,
        path,
    );
}

unsafe fn format_cgroup_path(buf: *mut c_char, size: usize, path: *const c_char) {
    format_cgroup_path_pid(buf, size, path, getpid());
}

unsafe fn format_parent_cgroup_path(buf: *mut c_char, size: usize, path: *const c_char) {
    format_cgroup_path_pid(buf, size, path, getppid());
}

unsafe fn format_classid_path_pid(buf: *mut c_char, size: usize, pid: libc::pid_t) {
    snprintf(
        buf,
        size,
        c"%s%s%d".as_ptr(),
        NETCLS_MOUNT_PATH.as_ptr() as *const c_char,
        CGROUP_WORK_DIR.as_ptr() as *const c_char,
        pid,
    );
}

unsafe fn format_classid_path(buf: *mut c_char, size: usize) {
    format_classid_path_pid(buf, size, getpid());
}

unsafe fn __enable_controllers(cgroup_path: *const c_char, controllers: *const c_char) -> c_int {
    let mut path = [0 as c_char; PATH_MAX as usize + 1];
    let mut enable = [0 as c_char; PATH_MAX as usize + 1];
    let mut c: *mut c_char;
    let mut c2: *mut c_char = ptr::null_mut();
    let fd: c_int;
    let cfd: c_int;
    let len: isize;

    /* If not controllers are passed, enable all available controllers */
    if controllers.is_null() {
        snprintf(path.as_mut_ptr(), path.len(), c"%s/cgroup.controllers".as_ptr(), cgroup_path);
        fd = open(path.as_ptr(), O_RDONLY);
        if fd < 0 {
            log_err(c"Opening cgroup.controllers: %s".as_ptr(), path.as_ptr());
            return 1;
        }
        len = read(fd, enable.as_mut_ptr() as *mut c_void, enable.len() - 1);
        if len < 0 {
            close(fd);
            log_err(c"Reading cgroup.controllers: %s".as_ptr(), path.as_ptr());
            return 1;
        } else if len == 0 {
            /* No controllers to enable */
            close(fd);
            return 0;
        }
        enable[len as usize] = 0;
        close(fd);
    } else {
        strscpy(enable.as_mut_ptr(), controllers);
    }

    snprintf(path.as_mut_ptr(), path.len(), c"%s/cgroup.subtree_control".as_ptr(), cgroup_path);
    cfd = open(path.as_ptr(), O_RDWR);
    if cfd < 0 {
        log_err(c"Opening cgroup.subtree_control: %s".as_ptr(), path.as_ptr());
        return 1;
    }

    c = strtok_r(enable.as_mut_ptr(), c" ".as_ptr(), &mut c2);
    while !c.is_null() {
        if dprintf(cfd, c"+%s\n".as_ptr(), c) <= 0 {
            log_err(c"Enabling controller %s: %s".as_ptr(), c, path.as_ptr());
            close(cfd);
            return 1;
        }
        c = strtok_r(ptr::null_mut(), c" ".as_ptr(), &mut c2);
    }
    close(cfd);
    0
}

/**
 * enable_controllers() - Enable cgroup v2 controllers
 * @relative_path: The cgroup path, relative to the workdir
 * @controllers: List of controllers to enable in cgroup.controllers format
 *
 *
 * Enable given cgroup v2 controllers, if @controllers is NULL, enable all
 * available controllers.
 *
 * If successful, 0 is returned.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn enable_controllers(
    relative_path: *const c_char,
    controllers: *const c_char,
) -> c_int {
    let mut cgroup_path = [0 as c_char; PATH_MAX as usize + 1];

    format_cgroup_path(cgroup_path.as_mut_ptr(), cgroup_path.len(), relative_path);
    __enable_controllers(cgroup_path.as_ptr(), controllers)
}

unsafe fn __write_cgroup_file(
    cgroup_path: *const c_char,
    file: *const c_char,
    buf: *const c_char,
) -> c_int {
    let mut file_path = [0 as c_char; PATH_MAX as usize + 1];
    let fd: c_int;

    snprintf(file_path.as_mut_ptr(), file_path.len(), c"%s/%s".as_ptr(), cgroup_path, file);
    fd = open(file_path.as_ptr(), O_RDWR);
    if fd < 0 {
        log_err(c"Opening %s".as_ptr(), file_path.as_ptr());
        return 1;
    }

    if dprintf(fd, c"%s".as_ptr(), buf) <= 0 {
        log_err(c"Writing to %s".as_ptr(), file_path.as_ptr());
        close(fd);
        return 1;
    }
    close(fd);
    0
}

/**
 * write_cgroup_file() - Write to a cgroup file
 * @relative_path: The cgroup path, relative to the workdir
 * @file: The name of the file in cgroupfs to write to
 * @buf: Buffer to write to the file
 *
 * Write to a file in the given cgroup's directory.
 *
 * If successful, 0 is returned.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_cgroup_file(
    relative_path: *const c_char,
    file: *const c_char,
    buf: *const c_char,
) -> c_int {
    let mut cgroup_path = [0 as c_char; PATH_MAX as usize - 24];

    format_cgroup_path(cgroup_path.as_mut_ptr(), cgroup_path.len(), relative_path);
    __write_cgroup_file(cgroup_path.as_ptr(), file, buf)
}

/**
 * write_cgroup_file_parent() - Write to a cgroup file in the parent process
 *                              workdir
 * @relative_path: The cgroup path, relative to the parent process workdir
 * @file: The name of the file in cgroupfs to write to
 * @buf: Buffer to write to the file
 *
 * Write to a file in the given cgroup's directory under the parent process
 * workdir.
 *
 * If successful, 0 is returned.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_cgroup_file_parent(
    relative_path: *const c_char,
    file: *const c_char,
    buf: *const c_char,
) -> c_int {
    let mut cgroup_path = [0 as c_char; PATH_MAX as usize - 24];

    format_parent_cgroup_path(cgroup_path.as_mut_ptr(), cgroup_path.len(), relative_path);
    __write_cgroup_file(cgroup_path.as_ptr(), file, buf)
}

/**
 * setup_cgroup_environment() - Setup the cgroup environment
 *
 * After calling this function, cleanup_cgroup_environment should be called
 * once testing is complete.
 *
 * This function will print an error to stderr and return 1 if it is unable
 * to setup the cgroup environment. If setup is successful, 0 is returned.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn setup_cgroup_environment() -> c_int {
    let mut cgroup_workdir = [0 as c_char; PATH_MAX as usize - 24];

    format_cgroup_path(cgroup_workdir.as_mut_ptr(), cgroup_workdir.len(), c"".as_ptr());

    if mkdir(CGROUP_MOUNT_PATH.as_ptr() as *const c_char, 0o777) != 0 && errno_value() != EEXIST {
        log_err(c"mkdir mount".as_ptr());
        return 1;
    }

    if unshare(CLONE_NEWNS) != 0 {
        log_err(c"unshare".as_ptr());
        return 1;
    }

    if mount(
        c"none".as_ptr(),
        c"/".as_ptr(),
        ptr::null(),
        (MS_REC | MS_PRIVATE) as c_ulong,
        ptr::null(),
    ) != 0
    {
        log_err(c"mount fakeroot".as_ptr());
        return 1;
    }

    if mount(
        c"none".as_ptr(),
        CGROUP_MOUNT_PATH.as_ptr() as *const c_char,
        c"cgroup2".as_ptr(),
        0,
        ptr::null(),
    ) != 0
        && errno_value() != EBUSY
    {
        log_err(c"mount cgroup2".as_ptr());
        return 1;
    }
    CGROUP_WORKDIR_MOUNTED.with(|mounted| mounted.set(true));

    /* Cleanup existing failed runs, now that the environment is setup */
    __cleanup_cgroup_environment();

    if mkdir(cgroup_workdir.as_ptr(), 0o777) != 0 && errno_value() != EEXIST {
        log_err(c"mkdir cgroup work dir".as_ptr());
        return 1;
    }

    /* Enable all available controllers to increase test coverage */
    if __enable_controllers(CGROUP_MOUNT_PATH.as_ptr() as *const c_char, ptr::null()) != 0
        || __enable_controllers(cgroup_workdir.as_ptr(), ptr::null()) != 0
    {
        return 1;
    }

    0
}

unsafe extern "C" fn nftwfunc(
    filename: *const c_char,
    _statptr: *const libc::stat,
    fileflags: c_int,
    _pfwt: *mut FTW,
) -> c_int {
    if (fileflags & FTW_D) != 0 && rmdir(filename) != 0 {
        log_err(c"Removing cgroup: %s".as_ptr(), filename);
    }
    0
}

unsafe fn join_cgroup_from_top(cgroup_path: *const c_char) -> c_int {
    let mut cgroup_procs_path = [0 as c_char; PATH_MAX as usize + 1];
    let pid = getpid();
    let fd: c_int;
    let mut rc: c_int = 0;

    snprintf(
        cgroup_procs_path.as_mut_ptr(),
        cgroup_procs_path.len(),
        c"%s/cgroup.procs".as_ptr(),
        cgroup_path,
    );

    fd = open(cgroup_procs_path.as_ptr(), O_WRONLY);
    if fd < 0 {
        log_err(c"Opening Cgroup Procs: %s".as_ptr(), cgroup_procs_path.as_ptr());
        return 1;
    }

    if dprintf(fd, c"%d\n".as_ptr(), pid) < 0 {
        log_err(c"Joining Cgroup".as_ptr());
        rc = 1;
    }

    close(fd);
    rc
}

/**
 * join_cgroup() - Join a cgroup
 * @relative_path: The cgroup path, relative to the workdir, to join
 *
 * This function expects a cgroup to already be created, relative to the cgroup
 * work dir, and it joins it. For example, passing "/my-cgroup" as the path
 * would actually put the calling process into the cgroup
 * "/cgroup-test-work-dir/my-cgroup"
 *
 * On success, it returns 0, otherwise on failure it returns 1.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn join_cgroup(relative_path: *const c_char) -> c_int {
    let mut cgroup_path = [0 as c_char; PATH_MAX as usize + 1];

    format_cgroup_path(cgroup_path.as_mut_ptr(), cgroup_path.len(), relative_path);
    join_cgroup_from_top(cgroup_path.as_ptr())
}

/**
 * join_root_cgroup() - Join the root cgroup
 *
 * This function joins the root cgroup.
 *
 * On success, it returns 0, otherwise on failure it returns 1.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn join_root_cgroup() -> c_int {
    join_cgroup_from_top(CGROUP_MOUNT_PATH.as_ptr() as *const c_char)
}

/**
 * join_parent_cgroup() - Join a cgroup in the parent process workdir
 * @relative_path: The cgroup path, relative to parent process workdir, to join
 *
 * See join_cgroup().
 *
 * On success, it returns 0, otherwise on failure it returns 1.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn join_parent_cgroup(relative_path: *const c_char) -> c_int {
    let mut cgroup_path = [0 as c_char; PATH_MAX as usize + 1];

    format_parent_cgroup_path(cgroup_path.as_mut_ptr(), cgroup_path.len(), relative_path);
    join_cgroup_from_top(cgroup_path.as_ptr())
}

/**
 * set_cgroup_xattr() - Set xattr on a cgroup dir
 * @relative_path: The cgroup path, relative to the workdir, to set xattr
 * @name: xattr name
 * @value: xattr value
 *
 * This function set xattr on cgroup dir.
 *
 * On success, it returns 0, otherwise on failure it returns -1.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_cgroup_xattr(
    relative_path: *const c_char,
    name: *const c_char,
    value: *const c_char,
) -> c_int {
    let mut cgroup_path = [0 as c_char; PATH_MAX as usize + 1];

    format_cgroup_path(cgroup_path.as_mut_ptr(), cgroup_path.len(), relative_path);
    setxattr(
        cgroup_path.as_ptr(),
        name,
        value as *const c_void,
        strlen(value) + 1,
        0,
    )
}

/**
 * __cleanup_cgroup_environment() - Delete temporary cgroups
 *
 * This is a helper for cleanup_cgroup_environment() that is responsible for
 * deletion of all temporary cgroups that have been created during the test.
 */
unsafe fn __cleanup_cgroup_environment() {
    let mut cgroup_workdir = [0 as c_char; PATH_MAX as usize + 1];

    format_cgroup_path(cgroup_workdir.as_mut_ptr(), cgroup_workdir.len(), c"".as_ptr());
    join_cgroup_from_top(CGROUP_MOUNT_PATH.as_ptr() as *const c_char);
    nftw(cgroup_workdir.as_ptr(), Some(nftwfunc), WALK_FD_LIMIT, FTW_DEPTH | FTW_MOUNT);
}

/**
 * cleanup_cgroup_environment() - Cleanup Cgroup Testing Environment
 *
 * This is an idempotent function to delete all temporary cgroups that
 * have been created during the test and unmount the cgroup testing work
 * directory.
 *
 * At call time, it moves the calling process to the root cgroup, and then
 * runs the deletion process. It is idempotent, and should not fail, unless
 * a process is lingering.
 *
 * On failure, it will print an error to stderr, and try to continue.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cleanup_cgroup_environment() {
    __cleanup_cgroup_environment();
    if CGROUP_WORKDIR_MOUNTED.with(|mounted| mounted.get())
        && umount(CGROUP_MOUNT_PATH.as_ptr() as *const c_char) != 0
    {
        log_err(c"umount cgroup2".as_ptr());
    }
    CGROUP_WORKDIR_MOUNTED.with(|mounted| mounted.set(false));
}

/**
 * get_root_cgroup() - Get the FD of the root cgroup
 *
 * On success, it returns the file descriptor. On failure, it returns -1.
 * If there is a failure, it prints the error to stderr.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_root_cgroup() -> c_int {
    let fd: c_int;

    fd = open(CGROUP_MOUNT_PATH.as_ptr() as *const c_char, O_RDONLY);
    if fd < 0 {
        log_err(c"Opening root cgroup".as_ptr());
        return -1;
    }
    fd
}

/*
 * remove_cgroup() - Remove a cgroup
 * @relative_path: The cgroup path, relative to the workdir, to remove
 *
 * This function expects a cgroup to already be created, relative to the cgroup
 * work dir. It also expects the cgroup doesn't have any children or live
 * processes and it removes the cgroup.
 *
 * On failure, it will print an error to stderr.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn remove_cgroup(relative_path: *const c_char) {
    let mut cgroup_path = [0 as c_char; PATH_MAX as usize + 1];

    format_cgroup_path(cgroup_path.as_mut_ptr(), cgroup_path.len(), relative_path);
    if rmdir(cgroup_path.as_ptr()) != 0 {
        log_err(c"rmdiring cgroup %s .. %s".as_ptr(), relative_path, cgroup_path.as_ptr());
    }
}

/*
 * remove_cgroup_pid() - Remove a cgroup setup by process identified by PID
 * @relative_path: The cgroup path, relative to the workdir, to remove
 * @pid: PID to be used to find cgroup_path
 *
 * This function expects a cgroup to already be created, relative to the cgroup
 * work dir. It also expects the cgroup doesn't have any children or live
 * processes and it removes the cgroup.
 *
 * On failure, it will print an error to stderr.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn remove_cgroup_pid(relative_path: *const c_char, pid: c_int) {
    let mut cgroup_path = [0 as c_char; PATH_MAX as usize + 1];

    format_cgroup_path_pid(cgroup_path.as_mut_ptr(), cgroup_path.len(), relative_path, pid);
    if rmdir(cgroup_path.as_ptr()) != 0 {
        log_err(c"rmdiring cgroup %s .. %s".as_ptr(), relative_path, cgroup_path.as_ptr());
    }
}

/**
 * create_and_get_cgroup() - Create a cgroup, relative to workdir, and get the FD
 * @relative_path: The cgroup path, relative to the workdir, to join
 *
 * This function creates a cgroup under the top level workdir and returns the
 * file descriptor. It is idempotent.
 *
 * On success, it returns the file descriptor. On failure it returns -1.
 * If there is a failure, it prints the error to stderr.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_and_get_cgroup(relative_path: *const c_char) -> c_int {
    let mut cgroup_path = [0 as c_char; PATH_MAX as usize + 1];
    let fd: c_int;

    format_cgroup_path(cgroup_path.as_mut_ptr(), cgroup_path.len(), relative_path);
    if mkdir(cgroup_path.as_ptr(), 0o777) != 0 && errno_value() != EEXIST {
        log_err(c"mkdiring cgroup %s .. %s".as_ptr(), relative_path, cgroup_path.as_ptr());
        return -1;
    }

    fd = open(cgroup_path.as_ptr(), O_RDONLY);
    if fd < 0 {
        log_err(c"Opening Cgroup".as_ptr());
        return -1;
    }

    fd
}

#[repr(C)]
union cgroup_id {
    cgid: u64,
    raw_bytes: [u8; 8],
}

/**
 * get_cgroup_id_from_path - Get cgroup id for a particular cgroup path
 * @cgroup_workdir: The absolute cgroup path
 *
 * On success, it returns the cgroup id. On failure it returns 0,
 * which is an invalid cgroup id.
 * If there is a failure, it prints the error to stderr.
 */
unsafe fn get_cgroup_id_from_path(cgroup_workdir: *const c_char) -> u64 {
    let dirfd: c_int;
    let mut err: c_int;
    let flags: c_int;
    let mut mount_id: c_int = 0;
    let mut fhsize: c_int;
    let mut id = cgroup_id { cgid: 0 };
    let mut fhp: *mut file_handle;
    let fhp2: *mut file_handle;
    let mut ret: u64 = 0;

    dirfd = AT_FDCWD;
    flags = 0;
    fhsize = mem::size_of::<file_handle>() as c_int;
    fhp = calloc(1, fhsize as usize) as *mut file_handle;
    if fhp.is_null() {
        log_err(c"calloc".as_ptr());
        return 0;
    }
    err = name_to_handle_at(dirfd, cgroup_workdir, fhp, &mut mount_id, flags);
    if err >= 0 || (*fhp).handle_bytes != 8 {
        log_err(c"name_to_handle_at".as_ptr());
        free(fhp as *mut c_void);
        return ret;
    }

    fhsize = mem::size_of::<file_handle>() as c_int + (*fhp).handle_bytes as c_int;
    fhp2 = realloc(fhp as *mut c_void, fhsize as usize) as *mut file_handle;
    if fhp2.is_null() {
        log_err(c"realloc".as_ptr());
        free(fhp as *mut c_void);
        return ret;
    }
    err = name_to_handle_at(dirfd, cgroup_workdir, fhp2, &mut mount_id, flags);
    fhp = fhp2;
    if err < 0 {
        log_err(c"name_to_handle_at".as_ptr());
        free(fhp as *mut c_void);
        return ret;
    }

    memcpy(
        id.raw_bytes.as_mut_ptr() as *mut c_void,
        (*fhp).f_handle.as_ptr() as *const c_void,
        8,
    );
    ret = id.cgid;

    free(fhp as *mut c_void);
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_cgroup_id(relative_path: *const c_char) -> u64 {
    let mut cgroup_workdir = [0 as c_char; PATH_MAX as usize + 1];

    format_cgroup_path(cgroup_workdir.as_mut_ptr(), cgroup_workdir.len(), relative_path);
    get_cgroup_id_from_path(cgroup_workdir.as_ptr())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cgroup_setup_and_join(path: *const c_char) -> c_int {
    let cg_fd: c_int;

    if setup_cgroup_environment() != 0 {
        fprintf(stderr, c"Failed to setup cgroup environment\n".as_ptr());
        return -EINVAL;
    }

    cg_fd = create_and_get_cgroup(path);
    if cg_fd < 0 {
        fprintf(stderr, c"Failed to create test cgroup\n".as_ptr());
        cleanup_cgroup_environment();
        return cg_fd;
    }

    if join_cgroup(path) != 0 {
        fprintf(stderr, c"Failed to join cgroup\n".as_ptr());
        cleanup_cgroup_environment();
        return -EINVAL;
    }
    cg_fd
}

/**
 * setup_classid_environment() - Setup the cgroupv1 net_cls environment
 *
 * This function should only be called in a custom mount namespace, e.g.
 * created by running setup_cgroup_environment.
 *
 * After calling this function, cleanup_classid_environment should be called
 * once testing is complete.
 *
 * This function will print an error to stderr and return 1 if it is unable
 * to setup the cgroup environment. If setup is successful, 0 is returned.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn setup_classid_environment() -> c_int {
    let mut cgroup_workdir = [0 as c_char; PATH_MAX as usize + 1];

    format_classid_path(cgroup_workdir.as_mut_ptr(), cgroup_workdir.len());

    if mount(
        c"tmpfs".as_ptr(),
        CGROUP_MOUNT_DFLT.as_ptr() as *const c_char,
        c"tmpfs".as_ptr(),
        0,
        ptr::null(),
    ) != 0
        && errno_value() != EBUSY
    {
        log_err(c"mount cgroup base".as_ptr());
        return 1;
    }

    if mkdir(NETCLS_MOUNT_PATH.as_ptr() as *const c_char, 0o777) != 0 && errno_value() != EEXIST {
        log_err(c"mkdir cgroup net_cls".as_ptr());
        return 1;
    }

    if mount(
        c"net_cls".as_ptr(),
        NETCLS_MOUNT_PATH.as_ptr() as *const c_char,
        c"cgroup".as_ptr(),
        0,
        c"net_cls".as_ptr() as *const c_void,
    ) != 0
    {
        if errno_value() != EBUSY {
            log_err(c"mount cgroup net_cls".as_ptr());
            return 1;
        }

        if rmdir(NETCLS_MOUNT_PATH.as_ptr() as *const c_char) != 0 {
            log_err(c"rmdir cgroup net_cls".as_ptr());
            return 1;
        }
        if umount(CGROUP_MOUNT_DFLT.as_ptr() as *const c_char) != 0 {
            log_err(c"umount cgroup base".as_ptr());
            return 1;
        }
    }

    cleanup_classid_environment();

    if mkdir(cgroup_workdir.as_ptr(), 0o777) != 0 && errno_value() != EEXIST {
        log_err(c"mkdir cgroup work dir".as_ptr());
        return 1;
    }

    0
}

/**
 * set_classid() - Set a cgroupv1 net_cls classid
 *
 * Writes the classid into the cgroup work dir's net_cls.classid
 * file in order to later on trigger socket tagging.
 *
 * We leverage the current pid as the classid, ensuring unique identification.
 *
 * On success, it returns 0, otherwise on failure it returns 1. If there
 * is a failure, it prints the error to stderr.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_classid() -> c_int {
    let mut cgroup_workdir = [0 as c_char; PATH_MAX as usize - 42];
    let mut cgroup_classid_path = [0 as c_char; PATH_MAX as usize + 1];
    let fd: c_int;
    let mut rc: c_int = 0;

    format_classid_path(cgroup_workdir.as_mut_ptr(), cgroup_workdir.len());
    snprintf(
        cgroup_classid_path.as_mut_ptr(),
        cgroup_classid_path.len(),
        c"%s/net_cls.classid".as_ptr(),
        cgroup_workdir.as_ptr(),
    );

    fd = open(cgroup_classid_path.as_ptr(), O_WRONLY);
    if fd < 0 {
        log_err(c"Opening cgroup classid: %s".as_ptr(), cgroup_classid_path.as_ptr());
        return 1;
    }

    if dprintf(fd, c"%u\n".as_ptr(), getpid() as c_uint) < 0 {
        log_err(c"Setting cgroup classid".as_ptr());
        rc = 1;
    }

    close(fd);
    rc
}

/**
 * join_classid() - Join a cgroupv1 net_cls classid
 *
 * This function expects the cgroup work dir to be already created, as we
 * join it here. This causes the process sockets to be tagged with the given
 * net_cls classid.
 *
 * On success, it returns 0, otherwise on failure it returns 1.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn join_classid() -> c_int {
    let mut cgroup_workdir = [0 as c_char; PATH_MAX as usize + 1];

    format_classid_path(cgroup_workdir.as_mut_ptr(), cgroup_workdir.len());
    join_cgroup_from_top(cgroup_workdir.as_ptr())
}

/**
 * cleanup_classid_environment() - Cleanup the cgroupv1 net_cls environment
 *
 * At call time, it moves the calling process to the root cgroup, and then
 * runs the deletion process.
 *
 * On failure, it will print an error to stderr, and try to continue.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cleanup_classid_environment() {
    let mut cgroup_workdir = [0 as c_char; PATH_MAX as usize + 1];

    format_classid_path(cgroup_workdir.as_mut_ptr(), cgroup_workdir.len());
    join_cgroup_from_top(NETCLS_MOUNT_PATH.as_ptr() as *const c_char);
    nftw(cgroup_workdir.as_ptr(), Some(nftwfunc), WALK_FD_LIMIT, FTW_DEPTH | FTW_MOUNT);
}

/**
 * get_classid_cgroup_id - Get the cgroup id of a net_cls cgroup
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_classid_cgroup_id() -> u64 {
    let mut cgroup_workdir = [0 as c_char; PATH_MAX as usize + 1];

    format_classid_path(cgroup_workdir.as_mut_ptr(), cgroup_workdir.len());
    get_cgroup_id_from_path(cgroup_workdir.as_ptr())
}

/**
 * get_cgroup1_hierarchy_id - Retrieves the ID of a cgroup1 hierarchy from the cgroup1 subsys name.
 * @subsys_name: The cgroup1 subsys name, which can be retrieved from /proc/self/cgroup. It can be
 * a named cgroup like "name=systemd", a controller name like "net_cls", or multi-controllers like
 * "net_cls,net_prio".
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_cgroup1_hierarchy_id(subsys_name: *const c_char) -> c_int {
    let mut c: *mut c_char;
    let mut c2: *mut c_char = ptr::null_mut();
    let mut c3: *mut c_char;
    let mut c4: *mut c_char = ptr::null_mut();
    let mut found = false;
    let mut line = [0 as c_char; 1024];
    let file: *mut FILE;
    let mut i: c_int;
    let mut id: c_int = 0;

    if subsys_name.is_null() {
        return -1;
    }

    file = fopen(c"/proc/self/cgroup".as_ptr(), c"r".as_ptr());
    if file.is_null() {
        log_err(c"fopen /proc/self/cgroup".as_ptr());
        return -1;
    }

    while !fgets(line.as_mut_ptr(), 1024, file).is_null() {
        i = 0;
        c = strtok_r(line.as_mut_ptr(), c":".as_ptr(), &mut c2);
        while !c.is_null() && i < 2 {
            if i == 0 {
                id = strtol(c, ptr::null_mut(), 10) as c_int;
            } else if i == 1 {
                if strcmp(c, subsys_name) == 0 {
                    found = true;
                    break;
                }

                /* Multiple subsystems may share one single mount point */
                c3 = strtok_r(c, c",".as_ptr(), &mut c4);
                while !c3.is_null() {
                    if strcmp(c, subsys_name) == 0 {
                        found = true;
                        break;
                    }
                    c3 = strtok_r(ptr::null_mut(), c",".as_ptr(), &mut c4);
                }
            }
            i += 1;
            c = strtok_r(ptr::null_mut(), c":".as_ptr(), &mut c2);
        }
        if found {
            break;
        }
    }
    fclose(file);
    if found { id } else { -1 }
}

/**
 * open_classid() - Open a cgroupv1 net_cls classid
 *
 * This function expects the cgroup work dir to be already created, as we
 * open it here.
 *
 * On success, it returns the file descriptor. On failure it returns -1.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn open_classid() -> c_int {
    let mut cgroup_workdir = [0 as c_char; PATH_MAX as usize + 1];

    format_classid_path(cgroup_workdir.as_mut_ptr(), cgroup_workdir.len());
    open(cgroup_workdir.as_ptr(), O_RDONLY)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
