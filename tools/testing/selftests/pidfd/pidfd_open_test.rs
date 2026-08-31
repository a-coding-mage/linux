// SPDX-License-Identifier: GPL-2.0

// C dependencies removed from executable Rust:
// _GNU_SOURCE, errno/fcntl/inttypes/limits/linux types/sched/signal/stdio/
// stdlib/string/syscall/ioctl/mount/prctl/wait/unistd, plus local pidfd.h and
// kselftest.h. Symbols supplied by those headers are declared below.

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::ptr;

type SizeT = usize;
type SSizeT = isize;
type PidT = c_int;
type UidT = u32;
type GidT = u32;

const ERANGE: c_int = 34;
const EINVAL: c_int = 22;
const LONG_MAX: c_long = c_long::MAX;
const LONG_MIN: c_long = c_long::MIN;
const INT_MAX: c_long = c_int::MAX as c_long;
const INT_MIN: c_long = c_int::MIN as c_long;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct pidfd_info {
    mask: u64,
    cgroupid: u64,
    pid: u32,
    tgid: u32,
    ppid: u32,
    ruid: u32,
    rgid: u32,
    euid: u32,
    egid: u32,
    suid: u32,
    sgid: u32,
}

unsafe extern "C" {
    static mut errno: c_int;

    static PIDFD_INFO_CGROUPID: u64;
    static PIDFD_GET_INFO: c_ulong;

    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strlen(s: *const c_char) -> SizeT;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: SizeT) -> c_int;
    fn snprintf(s: *mut c_char, n: SizeT, format: *const c_char, ...) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn getline(lineptr: *mut *mut c_char, n: *mut SizeT, stream: *mut FILE) -> SSizeT;
    fn free(ptr: *mut c_void);
    fn fclose(stream: *mut FILE) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn getpid() -> PidT;
    fn getppid() -> PidT;
    fn getuid() -> UidT;
    fn getgid() -> GidT;
    fn geteuid() -> UidT;
    fn getegid() -> GidT;

    fn sys_pidfd_open(pid: PidT, flags: c_uint) -> c_int;
    fn ksft_set_plan(plan: c_uint);
    fn ksft_print_msg(format: *const c_char, ...);
    fn ksft_test_result_pass(format: *const c_char, ...);
    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;
}

type c_uint = u32;

unsafe fn safe_int(numstr: *const c_char, converted: *mut c_int) -> c_int {
    let mut err: *mut c_char = ptr::null_mut();
    let sli: c_long;

    errno = 0;
    sli = strtol(numstr, &mut err, 0);
    if errno == ERANGE && (sli == LONG_MAX || sli == LONG_MIN) {
        return -ERANGE;
    }

    if errno != 0 && sli == 0 {
        return -EINVAL;
    }

    if err == numstr as *mut c_char || *err != b'\0' as c_char {
        return -EINVAL;
    }

    if sli > INT_MAX || sli < INT_MIN {
        return -ERANGE;
    }

    *converted = sli as c_int;
    0
}

unsafe fn char_left_gc(buffer: *const c_char, len: SizeT) -> c_int {
    let mut i: SizeT = 0;

    while i < len {
        if *buffer.add(i) == b' ' as c_char || *buffer.add(i) == b'\t' as c_char {
            i += 1;
            continue;
        }

        return i as c_int;
    }

    0
}

unsafe fn char_right_gc(buffer: *const c_char, len: SizeT) -> c_int {
    let mut i: c_int = len as c_int - 1;

    while i >= 0 {
        if *buffer.add(i as SizeT) == b' ' as c_char
            || *buffer.add(i as SizeT) == b'\t' as c_char
            || *buffer.add(i as SizeT) == b'\n' as c_char
            || *buffer.add(i as SizeT) == b'\0' as c_char
        {
            i -= 1;
            continue;
        }

        return i + 1;
    }

    0
}

unsafe fn trim_whitespace_in_place(mut buffer: *mut c_char) -> *mut c_char {
    buffer = buffer.add(char_left_gc(buffer, strlen(buffer)) as SizeT);
    *buffer.add(char_right_gc(buffer, strlen(buffer)) as SizeT) = b'\0' as c_char;
    buffer
}

unsafe fn get_pid_from_fdinfo_file(pidfd: c_int, key: *const c_char, keylen: SizeT) -> PidT {
    let mut ret: c_int;
    let mut path: [c_char; 512] = [0; 512];
    let f: *mut FILE;
    let mut n: SizeT = 0;
    let mut result: PidT = -1;
    let mut line: *mut c_char = ptr::null_mut();

    snprintf(
        path.as_mut_ptr(),
        path.len(),
        c"/proc/self/fdinfo/%d".as_ptr(),
        pidfd,
    );

    f = fopen(path.as_ptr(), c"re".as_ptr());
    if f.is_null() {
        return -1;
    }

    while getline(&mut line, &mut n, f) != -1 {
        let numstr: *mut c_char;

        if strncmp(line, key, keylen) != 0 {
            continue;
        }

        numstr = trim_whitespace_in_place(line.add(4));
        ret = safe_int(numstr, &mut result);
        if ret < 0 {
            goto_out(line, f);
            return result;
        }

        break;
    }

    goto_out(line, f);
    result
}

unsafe fn goto_out(line: *mut c_char, f: *mut FILE) {
    free(line as *mut c_void);
    fclose(f);
}

unsafe fn main_impl() -> c_int {
    let mut info = pidfd_info {
        mask: PIDFD_INFO_CGROUPID,
        cgroupid: 0,
        pid: 0,
        tgid: 0,
        ppid: 0,
        ruid: 0,
        rgid: 0,
        euid: 0,
        egid: 0,
        suid: 0,
        sgid: 0,
    };
    let mut pidfd: c_int = -1;
    let mut ret: c_int = 1;
    let pid: PidT;

    ksft_set_plan(4);

    pidfd = sys_pidfd_open(-1, 0);
    if pidfd >= 0 {
        ksft_print_msg(
            c"%s - succeeded to open pidfd for invalid pid -1\n".as_ptr(),
            strerror(errno),
        );
        goto_on_error(pidfd, ret);
    }
    ksft_test_result_pass(c"do not allow invalid pid test: passed\n".as_ptr());

    pidfd = sys_pidfd_open(getpid(), 1);
    if pidfd >= 0 {
        ksft_print_msg(
            c"%s - succeeded to open pidfd with invalid flag value specified\n".as_ptr(),
            strerror(errno),
        );
        goto_on_error(pidfd, ret);
    }
    ksft_test_result_pass(c"do not allow invalid flag test: passed\n".as_ptr());

    pidfd = sys_pidfd_open(getpid(), 0);
    if pidfd < 0 {
        ksft_print_msg(c"%s - failed to open pidfd\n".as_ptr(), strerror(errno));
        goto_on_error(pidfd, ret);
    }
    ksft_test_result_pass(c"open a new pidfd test: passed\n".as_ptr());

    pid = get_pid_from_fdinfo_file(pidfd, c"Pid:".as_ptr(), 4);
    ksft_print_msg(
        c"pidfd %d refers to process with pid %d\n".as_ptr(),
        pidfd,
        pid,
    );

    if ioctl(pidfd, PIDFD_GET_INFO, &mut info as *mut pidfd_info) < 0 {
        ksft_print_msg(
            c"%s - failed to get info from pidfd\n".as_ptr(),
            strerror(errno),
        );
        goto_on_error(pidfd, ret);
    }
    if info.pid != pid as u32 {
        ksft_print_msg(
            c"pid from fdinfo file %d does not match pid from ioctl %d\n".as_ptr(),
            pid,
            info.pid,
        );
        goto_on_error(pidfd, ret);
    }
    if info.ppid != getppid() as u32 {
        ksft_print_msg(
            c"ppid %d does not match ppid from ioctl %d\n".as_ptr(),
            pid,
            info.pid,
        );
        goto_on_error(pidfd, ret);
    }
    if info.ruid != getuid() {
        ksft_print_msg(
            c"uid %d does not match uid from ioctl %d\n".as_ptr(),
            getuid(),
            info.ruid,
        );
        goto_on_error(pidfd, ret);
    }
    if info.rgid != getgid() {
        ksft_print_msg(
            c"gid %d does not match gid from ioctl %d\n".as_ptr(),
            getgid(),
            info.rgid,
        );
        goto_on_error(pidfd, ret);
    }
    if info.euid != geteuid() {
        ksft_print_msg(
            c"euid %d does not match euid from ioctl %d\n".as_ptr(),
            geteuid(),
            info.euid,
        );
        goto_on_error(pidfd, ret);
    }
    if info.egid != getegid() {
        ksft_print_msg(
            c"egid %d does not match egid from ioctl %d\n".as_ptr(),
            getegid(),
            info.egid,
        );
        goto_on_error(pidfd, ret);
    }
    if info.suid != geteuid() {
        ksft_print_msg(
            c"suid %d does not match suid from ioctl %d\n".as_ptr(),
            geteuid(),
            info.suid,
        );
        goto_on_error(pidfd, ret);
    }
    if info.sgid != getegid() {
        ksft_print_msg(
            c"sgid %d does not match sgid from ioctl %d\n".as_ptr(),
            getegid(),
            info.sgid,
        );
        goto_on_error(pidfd, ret);
    }
    if (info.mask & PIDFD_INFO_CGROUPID) != 0 && info.cgroupid == 0 {
        ksft_print_msg(c"cgroupid should not be 0 when PIDFD_INFO_CGROUPID is set\n".as_ptr());
        goto_on_error(pidfd, ret);
    }
    ksft_test_result_pass(c"get info from pidfd test: passed\n".as_ptr());

    ret = 0;

    goto_on_error(pidfd, ret);
}

unsafe fn goto_on_error(pidfd: c_int, ret: c_int) -> ! {
    if pidfd >= 0 {
        close(pidfd);
    }

    if ret != 0 {
        ksft_exit_fail();
    }
    ksft_exit_pass();
}

fn main() {
    unsafe {
        main_impl();
    }
}
