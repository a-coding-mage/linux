// SPDX-License-Identifier: GPL-2.0-or-later

// C dependencies: assert.h, fcntl.h, limits.h, sched.h, stdlib.h,
// sys/mount.h, sys/stat.h, sys/wait.h, linux/nsfs.h, linux/stat.h,
// "statmount.h", "../utils.h", "kselftest.h"

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;

const NSID_PASS: c_int = 0;
const NSID_FAIL: c_int = 1;
const NSID_SKIP: c_int = 2;
const NSID_ERROR: c_int = 3;

// Provided by the translated dependency corresponding to statmount.h.
#[repr(C)]
struct statmount {
    size: u32,
    mask: u64,
    mnt_ns_id: u64,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn sleep(seconds: c_ulong) -> c_ulong;
    fn mkdtemp(template: *mut c_char) -> *mut c_char;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn umount2(target: *const c_char, flags: c_int) -> c_int;
    fn rmdir(pathname: *const c_char) -> c_int;
    fn fork() -> pid_t;
    fn exit(status: c_int) -> !;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;

    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_exit_fail_msg(fmt: *const c_char, ...) -> !;
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_print_header();
    fn ksft_exit_skip(fmt: *const c_char, ...) -> !;
    fn ksft_set_plan(plan: c_int);
    fn ksft_get_fail_cnt() -> c_int;
    fn ksft_get_error_cnt() -> c_int;
    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;

    fn setup_userns() -> c_int;
    fn wait_for_pid(pid: pid_t) -> c_int;
    fn get_unique_mnt_id(path: *const c_char) -> u64;
    fn statmount(
        mnt_id: u64,
        mnt_ns_id: u64,
        mnt_fd: c_int,
        mask: u64,
        buf: *mut statmount,
        bufsize: usize,
        flags: u32,
    ) -> c_int;
    fn listmount(
        mnt_id: u64,
        mnt_ns_id: u64,
        last_mnt_id: u64,
        list: *mut u64,
        num: u32,
        flags: u32,
    ) -> u64;
}

type pid_t = c_int;

// External C constants from the included headers.
const O_RDONLY: c_int = libc::O_RDONLY;
const O_PATH: c_int = libc::O_PATH;
const MS_BIND: c_ulong = libc::MS_BIND as c_ulong;
const MNT_DETACH: c_int = libc::MNT_DETACH;
const ENOSYS: c_int = libc::ENOSYS;
const NS_GET_MNTNS_ID: c_ulong = libc::NS_GET_MNTNS_ID as c_ulong;
const STATMOUNT_MNT_NS_ID: u64 = libc::STATMOUNT_MNT_NS_ID as u64;
const STATMOUNT_BY_FD: u32 = libc::STATMOUNT_BY_FD as u32;
const LSMT_ROOT: u64 = libc::LSMT_ROOT as u64;

unsafe fn handle_result(ret: c_int, testname: *const c_char) {
    if ret == NSID_PASS {
        ksft_test_result_pass(b"%s\n\0".as_ptr() as *const c_char, testname);
    } else if ret == NSID_FAIL {
        ksft_test_result_fail(b"%s\n\0".as_ptr() as *const c_char, testname);
    } else if ret == NSID_ERROR {
        ksft_exit_fail_msg(b"%s\n\0".as_ptr() as *const c_char, testname);
    } else {
        ksft_test_result_skip(b"%s\n\0".as_ptr() as *const c_char, testname);
    }
}

unsafe fn get_mnt_ns_id(mnt_ns: *const c_char, mnt_ns_id: *mut u64) -> c_int {
    let fd = open(mnt_ns, O_RDONLY);

    if fd < 0 {
        ksft_print_msg(
            b"failed to open for ns %s: %s\n\0".as_ptr() as *const c_char,
            mnt_ns,
            strerror(errno),
        );
        sleep(60);
        return NSID_ERROR;
    }

    if ioctl(fd, NS_GET_MNTNS_ID, mnt_ns_id) < 0 {
        ksft_print_msg(
            b"failed to get the nsid for ns %s: %s\n\0".as_ptr() as *const c_char,
            mnt_ns,
            strerror(errno),
        );
        return NSID_ERROR;
    }
    close(fd);
    NSID_PASS
}

unsafe fn setup_namespace() -> c_int {
    if setup_userns() != 0 {
        return NSID_ERROR;
    }

    NSID_PASS
}

unsafe fn _test_statmount_mnt_ns_id() -> c_int {
    let mut sm: statmount = core::mem::zeroed();
    let mut mnt_ns_id: u64 = 0;
    let root_id: u64;
    let mut ret: c_int;

    ret = get_mnt_ns_id(b"/proc/self/ns/mnt\0".as_ptr() as *const c_char, &mut mnt_ns_id);
    if ret != NSID_PASS {
        return ret;
    }

    root_id = get_unique_mnt_id(b"/\0".as_ptr() as *const c_char);
    if root_id == 0 {
        return NSID_ERROR;
    }

    ret = statmount(
        root_id,
        0,
        0,
        STATMOUNT_MNT_NS_ID,
        &mut sm,
        size_of::<statmount>(),
        0,
    );
    if ret == -1 {
        ksft_print_msg(
            b"statmount mnt ns id: %s\n\0".as_ptr() as *const c_char,
            strerror(errno),
        );
        return NSID_ERROR;
    }

    if sm.size != size_of::<statmount>() as u32 {
        ksft_print_msg(
            b"unexpected size: %u != %u\n\0".as_ptr() as *const c_char,
            sm.size,
            size_of::<statmount>() as u32,
        );
        return NSID_FAIL;
    }
    if sm.mask != STATMOUNT_MNT_NS_ID {
        ksft_print_msg(b"statmount mnt ns id unavailable\n\0".as_ptr() as *const c_char);
        return NSID_SKIP;
    }

    if sm.mnt_ns_id != mnt_ns_id {
        ksft_print_msg(
            b"unexpected mnt ns ID: 0x%llx != 0x%llx\n\0".as_ptr() as *const c_char,
            sm.mnt_ns_id as libc::c_ulonglong,
            mnt_ns_id as libc::c_ulonglong,
        );
        return NSID_FAIL;
    }

    NSID_PASS
}

unsafe fn _test_statmount_mnt_ns_id_by_fd() -> c_int {
    let mut sm: statmount = core::mem::zeroed();
    let mut mnt_ns_id: u64 = 0;
    let mut ret: c_int;
    let mut fd: c_int = -1;
    let mut mounted: c_int = 1;
    let mut status: c_int = NSID_ERROR;
    let mut mnt = *b"/statmount.fd.XXXXXX\0";

    ret = get_mnt_ns_id(b"/proc/self/ns/mnt\0".as_ptr() as *const c_char, &mut mnt_ns_id);
    if ret != NSID_PASS {
        return ret;
    }

    if mkdtemp(mnt.as_mut_ptr() as *mut c_char).is_null() {
        ksft_print_msg(
            b"statmount by fd mnt ns id mkdtemp: %s\n\0".as_ptr() as *const c_char,
            strerror(errno),
        );
        return NSID_ERROR;
    }

    if mount(
        mnt.as_ptr() as *const c_char,
        mnt.as_ptr() as *const c_char,
        core::ptr::null(),
        MS_BIND,
        core::ptr::null(),
    ) != 0
    {
        ksft_print_msg(
            b"statmount by fd mnt ns id mount: %s\n\0".as_ptr() as *const c_char,
            strerror(errno),
        );
        status = NSID_ERROR;
        rmdir(mnt.as_ptr() as *const c_char);
        return status;
    }

    fd = open(mnt.as_ptr() as *const c_char, O_PATH);
    if fd < 0 {
        ksft_print_msg(
            b"statmount by fd mnt ns id open: %s\n\0".as_ptr() as *const c_char,
            strerror(errno),
        );
        if mounted != 0 {
            umount2(mnt.as_ptr() as *const c_char, MNT_DETACH);
        }
        rmdir(mnt.as_ptr() as *const c_char);
        return status;
    }

    ret = statmount(
        0,
        0,
        fd,
        STATMOUNT_MNT_NS_ID,
        &mut sm,
        size_of::<statmount>(),
        STATMOUNT_BY_FD,
    );
    if ret == -1 {
        ksft_print_msg(
            b"statmount mnt ns id statmount: %s\n\0".as_ptr() as *const c_char,
            strerror(errno),
        );
        status = NSID_ERROR;
        close(fd);
        if mounted != 0 {
            umount2(mnt.as_ptr() as *const c_char, MNT_DETACH);
        }
        rmdir(mnt.as_ptr() as *const c_char);
        return status;
    }

    if sm.size != size_of::<statmount>() as u32 {
        ksft_print_msg(
            b"unexpected size: %u != %u\n\0".as_ptr() as *const c_char,
            sm.size,
            size_of::<statmount>() as u32,
        );
        status = NSID_FAIL;
        close(fd);
        if mounted != 0 {
            umount2(mnt.as_ptr() as *const c_char, MNT_DETACH);
        }
        rmdir(mnt.as_ptr() as *const c_char);
        return status;
    }
    if sm.mask != STATMOUNT_MNT_NS_ID {
        ksft_print_msg(b"statmount mnt ns id unavailable\n\0".as_ptr() as *const c_char);
        status = NSID_SKIP;
        close(fd);
        if mounted != 0 {
            umount2(mnt.as_ptr() as *const c_char, MNT_DETACH);
        }
        rmdir(mnt.as_ptr() as *const c_char);
        return status;
    }

    if sm.mnt_ns_id != mnt_ns_id {
        ksft_print_msg(
            b"unexpected mnt ns ID: 0x%llx != 0x%llx\n\0".as_ptr() as *const c_char,
            sm.mnt_ns_id as libc::c_ulonglong,
            mnt_ns_id as libc::c_ulonglong,
        );
        status = NSID_FAIL;
        close(fd);
        if mounted != 0 {
            umount2(mnt.as_ptr() as *const c_char, MNT_DETACH);
        }
        rmdir(mnt.as_ptr() as *const c_char);
        return status;
    }

    mounted = 0;
    if umount2(mnt.as_ptr() as *const c_char, MNT_DETACH) != 0 {
        ksft_print_msg(
            b"statmount by fd mnt ns id umount2: %s\n\0".as_ptr() as *const c_char,
            strerror(errno),
        );
        close(fd);
        rmdir(mnt.as_ptr() as *const c_char);
        return status;
    }

    ret = statmount(
        0,
        0,
        fd,
        STATMOUNT_MNT_NS_ID,
        &mut sm,
        size_of::<statmount>(),
        STATMOUNT_BY_FD,
    );
    if ret == -1 {
        ksft_print_msg(
            b"statmount mnt ns id statmount: %s\n\0".as_ptr() as *const c_char,
            strerror(errno),
        );
        status = NSID_ERROR;
        close(fd);
        rmdir(mnt.as_ptr() as *const c_char);
        return status;
    }

    if sm.size != size_of::<statmount>() as u32 {
        ksft_print_msg(
            b"unexpected size: %u != %u\n\0".as_ptr() as *const c_char,
            sm.size,
            size_of::<statmount>() as u32,
        );
        status = NSID_FAIL;
        close(fd);
        rmdir(mnt.as_ptr() as *const c_char);
        return status;
    }

    if sm.mask == STATMOUNT_MNT_NS_ID {
        ksft_print_msg(b"unexpected STATMOUNT_MNT_NS_ID in mask\n\0".as_ptr() as *const c_char);
        status = NSID_FAIL;
        close(fd);
        rmdir(mnt.as_ptr() as *const c_char);
        return status;
    }

    status = NSID_PASS;
    close(fd);
    if mounted != 0 {
        umount2(mnt.as_ptr() as *const c_char, MNT_DETACH);
    }
    rmdir(mnt.as_ptr() as *const c_char);
    status
}

unsafe fn test_statmount_mnt_ns_id() {
    let pid: pid_t;
    let mut ret: c_int;

    pid = fork();
    if pid < 0 {
        ksft_exit_fail_msg(
            b"failed to fork: %s\n\0".as_ptr() as *const c_char,
            strerror(errno),
        );
    }

    /* We're the original pid, wait for the result. */
    if pid != 0 {
        ret = wait_for_pid(pid);
        handle_result(ret, b"test statmount ns id\0".as_ptr() as *const c_char);
        return;
    }

    ret = setup_namespace();
    if ret != NSID_PASS {
        exit(ret);
    }
    ret = _test_statmount_mnt_ns_id();
    if ret != NSID_PASS {
        exit(ret);
    }
    ret = _test_statmount_mnt_ns_id_by_fd();
    exit(ret);
}

unsafe fn validate_external_listmount(pid: pid_t, child_nr_mounts: u64) -> c_int {
    let mut list = [0_u64; 256];
    let mut mnt_ns_id: u64 = 0;
    let nr_mounts: u64;
    let mut buf = [0 as c_char; 256];
    let mut ret: c_int;

    /* Get the mount ns id for our child. */
    snprintf(
        buf.as_mut_ptr(),
        size_of::<[c_char; 256]>(),
        b"/proc/%lu/ns/mnt\0".as_ptr() as *const c_char,
        pid as c_ulong,
    );
    ret = get_mnt_ns_id(buf.as_ptr(), &mut mnt_ns_id);

    nr_mounts = listmount(LSMT_ROOT, mnt_ns_id, 0, list.as_mut_ptr(), 256, 0);
    if nr_mounts == u64::MAX {
        ksft_print_msg(b"listmount: %s\n\0".as_ptr() as *const c_char, strerror(errno));
        return NSID_ERROR;
    }

    if nr_mounts != child_nr_mounts {
        ksft_print_msg(
            b"listmount results is %zi != %zi\n\0".as_ptr() as *const c_char,
            nr_mounts as isize,
            child_nr_mounts as isize,
        );
        return NSID_FAIL;
    }

    /* Validate that all of our entries match our mnt_ns_id. */
    let mut i: c_int = 0;
    while (i as u64) < nr_mounts {
        let mut sm: statmount = core::mem::zeroed();

        ret = statmount(
            list[i as usize],
            mnt_ns_id,
            0,
            STATMOUNT_MNT_NS_ID,
            &mut sm,
            size_of::<statmount>(),
            0,
        );
        if ret < 0 {
            ksft_print_msg(
                b"statmount mnt ns id: %s\n\0".as_ptr() as *const c_char,
                strerror(errno),
            );
            return NSID_ERROR;
        }

        if sm.mask != STATMOUNT_MNT_NS_ID {
            ksft_print_msg(b"statmount mnt ns id unavailable\n\0".as_ptr() as *const c_char);
            return NSID_SKIP;
        }

        if sm.mnt_ns_id != mnt_ns_id {
            ksft_print_msg(
                b"listmount gave us the wrong ns id: 0x%llx != 0x%llx\n\0".as_ptr()
                    as *const c_char,
                sm.mnt_ns_id as libc::c_ulonglong,
                mnt_ns_id as libc::c_ulonglong,
            );
            return NSID_FAIL;
        }
        i += 1;
    }

    NSID_PASS
}

unsafe fn test_listmount_ns() {
    let mut nr_mounts: u64 = 0;
    let pval: c_char = 0;
    let mut child_ready_pipe = [0 as c_int; 2];
    let mut parent_ready_pipe = [0 as c_int; 2];
    let pid: pid_t;
    let mut ret: c_int = NSID_PASS;
    let child_ret: c_int;

    if pipe(child_ready_pipe.as_mut_ptr()) < 0 {
        ksft_exit_fail_msg(
            b"failed to create the child pipe: %s\n\0".as_ptr() as *const c_char,
            strerror(errno),
        );
    }
    if pipe(parent_ready_pipe.as_mut_ptr()) < 0 {
        ksft_exit_fail_msg(
            b"failed to create the parent pipe: %s\n\0".as_ptr() as *const c_char,
            strerror(errno),
        );
    }

    pid = fork();
    if pid < 0 {
        ksft_exit_fail_msg(
            b"failed to fork: %s\n\0".as_ptr() as *const c_char,
            strerror(errno),
        );
    }

    if pid == 0 {
        let mut cval: c_char = 0;
        let mut list = [0_u64; 256];

        close(child_ready_pipe[0]);
        close(parent_ready_pipe[1]);

        ret = setup_namespace();
        if ret != NSID_PASS {
            exit(ret);
        }

        nr_mounts = listmount(LSMT_ROOT, 0, 0, list.as_mut_ptr(), 256, 0);
        if nr_mounts == u64::MAX {
            ksft_print_msg(b"listmount: %s\n\0".as_ptr() as *const c_char, strerror(errno));
            exit(NSID_FAIL);
        }

        /*
         * Tell our parent how many mounts we have, and then wait for it
         * to tell us we're done.
         */
        if write(
            child_ready_pipe[1],
            &nr_mounts as *const u64 as *const c_void,
            size_of::<u64>(),
        ) != size_of::<u64>() as isize
        {
            ret = NSID_ERROR;
        }
        if read(
            parent_ready_pipe[0],
            &mut cval as *mut c_char as *mut c_void,
            size_of::<c_char>(),
        ) != size_of::<c_char>() as isize
        {
            ret = NSID_ERROR;
        }
        exit(NSID_PASS);
    }

    close(child_ready_pipe[1]);
    close(parent_ready_pipe[0]);

    /* Wait until the child has created everything. */
    if read(
        child_ready_pipe[0],
        &mut nr_mounts as *mut u64 as *mut c_void,
        size_of::<u64>(),
    ) != size_of::<u64>() as isize
    {
        ret = NSID_ERROR;
    }

    ret = validate_external_listmount(pid, nr_mounts);

    if write(
        parent_ready_pipe[1],
        &pval as *const c_char as *const c_void,
        size_of::<c_char>(),
    ) != size_of::<c_char>() as isize
    {
        ret = NSID_ERROR;
    }

    child_ret = wait_for_pid(pid);
    if child_ret != NSID_PASS {
        ret = child_ret;
    }
    handle_result(ret, b"test listmount ns id\0".as_ptr() as *const c_char);
}

fn main() {
    unsafe {
        let ret: c_int;

        ksft_print_header();
        ret = statmount(0, 0, 0, 0, core::ptr::null_mut(), 0, 0);
        assert!(ret == -1);
        if errno == ENOSYS {
            ksft_exit_skip(b"statmount() syscall not supported\n\0".as_ptr() as *const c_char);
        }

        ksft_set_plan(2);
        test_statmount_mnt_ns_id();
        test_listmount_ns();

        if ksft_get_fail_cnt() + ksft_get_error_cnt() > 0 {
            ksft_exit_fail();
        } else {
            ksft_exit_pass();
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
