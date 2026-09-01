// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 Google, Inc.
 *
 * Selftests for execveat(2).
 */

/* C source defined _GNU_SOURCE to get O_PATH and AT_EMPTY_PATH. */

use libc::{
    c_char, c_int, c_long, c_void, mode_t, off_t, pid_t, size_t, ssize_t, stat,
    AT_EMPTY_PATH, AT_FDCWD, AT_SYMLINK_NOFOLLOW, EACCES, EBADF, EFAULT, EINVAL, ELOOP, ENOENT,
    ENOSYS, ENOTDIR, O_CLOEXEC, O_CREAT, O_DIRECTORY, O_PATH, O_RDONLY, O_RDWR, O_TRUNC,
    PATH_MAX, SEEK_SET,
};

const TESTS_EXPECTED: c_int = 54;
const TEST_NAME_LEN: usize = (PATH_MAX as usize) * 4;

const CHECK_COMM: &[u8] = b"CHECK_COMM\0";

static mut LONGPATH: [c_char; 2 * PATH_MAX as usize] = [0; 2 * PATH_MAX as usize];

static mut ENV_IN_TEST: [c_char; 12] = *b"IN_TEST=yes\0";
static mut ENV_VERBOSE: [c_char; 10] = *b"VERBOSE=1\0";
static mut ARG_EXECVEAT: [c_char; 9] = *b"execveat\0";
static mut ARG_99: [c_char; 3] = *b"99\0";

static mut ENVP: [*mut c_char; 3] = unsafe {
    [
        ENV_IN_TEST.as_mut_ptr(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    ]
};
static mut ARGV: [*mut c_char; 3] =
    unsafe { [ARG_EXECVEAT.as_mut_ptr(), ARG_99.as_mut_ptr(), std::ptr::null_mut()] };

unsafe extern "C" {
    fn syscall(num: c_long, ...) -> c_long;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn sendfile(out_fd: c_int, in_fd: c_int, offset: *mut off_t, count: size_t) -> ssize_t;
    fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn mkdir(pathname: *const c_char, mode: mode_t) -> c_int;
    fn fork() -> pid_t;
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;
    fn exit(status: c_int) -> !;
    fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    fn rename(oldpath: *const c_char, newpath: *const c_char) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
    fn realpath(path: *const c_char, resolved_path: *mut c_char) -> *mut c_char;
    fn mkfifo(pathname: *const c_char, mode: mode_t) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn getenv(name: *const c_char) -> *mut c_char;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;

    fn __errno_location() -> *mut c_int;

    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_perror(msg: *const c_char);
    fn ksft_exit_fail_msg(fmt: *const c_char, ...) -> !;
    fn ksft_exit_skip(fmt: *const c_char, ...) -> !;
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn ksft_finished();
}

unsafe fn errno_ptr() -> *mut c_int {
    unsafe { __errno_location() }
}

unsafe fn get_errno() -> c_int {
    unsafe { *errno_ptr() }
}

unsafe fn set_errno(value: c_int) {
    unsafe {
        *errno_ptr() = value;
    }
}

unsafe fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn path_or_null_string(path: *const c_char) -> *const c_char {
    if path.is_null() {
        c"(null)".as_ptr()
    } else {
        path
    }
}

unsafe fn execveat_(
    fd: c_int,
    path: *const c_char,
    argv: *mut *mut c_char,
    envp: *mut *mut c_char,
    flags: c_int,
) -> c_int {
    /* If __NR_execveat is unavailable at build time, the C source returns ENOSYS. */
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "arm"))]
    {
        unsafe { syscall(libc::SYS_execveat as c_long, fd, path, argv, envp, flags) as c_int }
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "arm")))]
    {
        unsafe {
            set_errno(ENOSYS);
        }
        -1
    }
}

unsafe fn _check_execveat_fail(
    fd: c_int,
    path: *const c_char,
    flags: c_int,
    expected_errno: c_int,
    errno_str: *const c_char,
) -> c_int {
    let mut test_name: [c_char; TEST_NAME_LEN] = [0; TEST_NAME_LEN];
    let rc: c_int;

    unsafe {
        set_errno(0);
        snprintf(
            test_name.as_mut_ptr(),
            test_name.len(),
            c"Check failure of execveat(%d, '%s', %d) with %s".as_ptr(),
            fd,
            path_or_null_string(path),
            flags,
            errno_str,
        );
        rc = execveat_(fd, path, ARGV.as_mut_ptr(), ENVP.as_mut_ptr(), flags);

        if rc > 0 {
            ksft_print_msg(c"unexpected success from execveat(2)\n".as_ptr());
            ksft_test_result_fail(c"%s\n".as_ptr(), test_name.as_ptr());
            return 1;
        }
        if get_errno() != expected_errno {
            ksft_print_msg(
                c"expected errno %d (%s) not %d (%s)\n".as_ptr(),
                expected_errno,
                strerror(expected_errno),
                get_errno(),
                strerror(get_errno()),
            );
            ksft_test_result_fail(c"%s\n".as_ptr(), test_name.as_ptr());
            return 1;
        }
        ksft_test_result_pass(c"%s\n".as_ptr(), test_name.as_ptr());
    }
    0
}

macro_rules! check_execveat_fail {
    ($fd:expr, $path:expr, $flags:expr, $errno_value:ident) => {
        unsafe {
            _check_execveat_fail(
                $fd,
                $path,
                $flags,
                $errno_value,
                concat!(stringify!($errno_value), "\0").as_ptr() as *const c_char,
            )
        }
    };
}

unsafe fn check_execveat_invoked_rc(
    fd: c_int,
    path: *const c_char,
    flags: c_int,
    expected_rc: c_int,
    expected_rc2: c_int,
) -> c_int {
    let mut test_name: [c_char; TEST_NAME_LEN] = [0; TEST_NAME_LEN];
    let mut status: c_int = 0;
    let mut rc: c_int;
    let child: pid_t;
    let pathlen: c_int = unsafe {
        if !path.is_null() {
            strlen(path) as c_int
        } else {
            0
        }
    };

    unsafe {
        if pathlen > 40 {
            snprintf(
                test_name.as_mut_ptr(),
                test_name.len(),
                c"Check success of execveat(%d, '%.20s...%s', %d)... ".as_ptr(),
                fd,
                path,
                path.add(pathlen as usize - 20),
                flags,
            );
        } else {
            snprintf(
                test_name.as_mut_ptr(),
                test_name.len(),
                c"Check success of execveat(%d, '%s', %d)... ".as_ptr(),
                fd,
                path_or_null_string(path),
                flags,
            );
        }

        child = fork();
        if child < 0 {
            ksft_perror(c"fork() failed".as_ptr());
            ksft_test_result_fail(c"%s\n".as_ptr(), test_name.as_ptr());
            return 1;
        }
        if child == 0 {
            /* Child: do execveat(). */
            rc = execveat_(fd, path, ARGV.as_mut_ptr(), ENVP.as_mut_ptr(), flags);
            ksft_print_msg(
                c"child execveat() failed, rc=%d errno=%d (%s)\n".as_ptr(),
                rc,
                get_errno(),
                strerror(get_errno()),
            );
            exit(get_errno());
        }
        /* Parent: wait for & check child's exit status. */
        rc = waitpid(child, &mut status, 0);
        if rc != child {
            ksft_print_msg(c"waitpid(%d,...) returned %d\n".as_ptr(), child, rc);
            ksft_test_result_fail(c"%s\n".as_ptr(), test_name.as_ptr());
            return 1;
        }
        if !wifexited(status) {
            ksft_print_msg(
                c"child %d did not exit cleanly, status=%08x\n".as_ptr(),
                child,
                status,
            );
            ksft_test_result_fail(c"%s\n".as_ptr(), test_name.as_ptr());
            return 1;
        }
        if (wexitstatus(status) != expected_rc) && (wexitstatus(status) != expected_rc2) {
            ksft_print_msg(
                c"child %d exited with %d neither %d nor %d\n".as_ptr(),
                child,
                wexitstatus(status),
                expected_rc,
                expected_rc2,
            );
            ksft_test_result_fail(c"%s\n".as_ptr(), test_name.as_ptr());
            return 1;
        }
        ksft_test_result_pass(c"%s\n".as_ptr(), test_name.as_ptr());
    }
    0
}

unsafe fn check_execveat(fd: c_int, path: *const c_char, flags: c_int) -> c_int {
    unsafe { check_execveat_invoked_rc(fd, path, flags, 99, 99) }
}

unsafe fn concat(left: *const c_char, right: *const c_char) -> *mut c_char {
    unsafe {
        let result = malloc(strlen(left) + strlen(right) + 1) as *mut c_char;

        strcpy(result, left);
        strcat(result, right);
        result
    }
}

unsafe fn open_or_die(filename: *const c_char, flags: c_int) -> c_int {
    let fd = unsafe { open(filename, flags) };

    if fd < 0 {
        unsafe {
            ksft_exit_fail_msg(
                c"Failed to open '%s'; check prerequisites are available\n".as_ptr(),
                filename,
            );
        }
    }
    fd
}

unsafe fn exe_cp(src: *const c_char, dest: *const c_char) {
    let in_fd = unsafe { open_or_die(src, O_RDONLY) };
    let out_fd = unsafe { open(dest, O_RDWR | O_CREAT | O_TRUNC, 0o755 as mode_t) };
    let mut info: stat = unsafe { std::mem::zeroed() };

    unsafe {
        fstat(in_fd, &mut info);
        sendfile(out_fd, in_fd, std::ptr::null_mut(), info.st_size as size_t);
        close(in_fd);
        close(out_fd);
    }
}

const XX_DIR_LEN: usize = 200;

unsafe fn check_execveat_pathmax(root_dfd: c_int, src: *const c_char, is_script: c_int) -> c_int {
    let mut fail: c_int = 0;
    let mut ii: c_int;
    let count: c_int;
    let mut len: c_int;
    let mut longname: [c_char; XX_DIR_LEN + 1] = [0; XX_DIR_LEN + 1];
    let fd: c_int;

    unsafe {
        if LONGPATH[0] == 0 {
            /* Create a filename close to PATH_MAX in length */
            let cwd = getcwd(std::ptr::null_mut(), 0);

            if cwd.is_null() {
                ksft_perror(c"Failed to getcwd()".as_ptr());
                return 2;
            }
            strcpy(LONGPATH.as_mut_ptr(), cwd);
            strcat(LONGPATH.as_mut_ptr(), c"/".as_ptr());
            memset(
                longname.as_mut_ptr() as *mut c_void,
                b'x' as c_int,
                XX_DIR_LEN - 1,
            );
            longname[XX_DIR_LEN - 1] = b'/' as c_char;
            longname[XX_DIR_LEN] = 0;
            count = ((PATH_MAX - 3 - strlen(cwd) as c_int) / XX_DIR_LEN as c_int) as c_int;
            ii = 0;
            while ii < count {
                strcat(LONGPATH.as_mut_ptr(), longname.as_ptr());
                mkdir(LONGPATH.as_ptr(), 0o755 as mode_t);
                ii += 1;
            }
            len = (PATH_MAX - 3 - strlen(cwd) as c_int) - (count * XX_DIR_LEN as c_int);
            if len <= 0 {
                len = 1;
            }
            memset(longname.as_mut_ptr() as *mut c_void, b'y' as c_int, len as size_t);
            longname[len as usize] = 0;
            strcat(LONGPATH.as_mut_ptr(), longname.as_ptr());
            free(cwd as *mut c_void);
        }
        exe_cp(src, LONGPATH.as_ptr());

        /*
         * Execute as a pre-opened file descriptor, which works whether this is
         * a script or not (because the interpreter sees a filename like
         * "/dev/fd/20").
         */
        fd = open(LONGPATH.as_ptr(), O_RDONLY);
        if fd > 0 {
            ksft_print_msg(
                c"Invoke copy of '%s' via filename of length %zu:\n".as_ptr(),
                src,
                strlen(LONGPATH.as_ptr()),
            );
            fail += check_execveat(fd, c"".as_ptr(), AT_EMPTY_PATH);
        } else {
            ksft_print_msg(
                c"Failed to open length %zu filename, errno=%d (%s)\n".as_ptr(),
                strlen(LONGPATH.as_ptr()),
                get_errno(),
                strerror(get_errno()),
            );
            fail += 1;
        }

        /*
         * Execute as a long pathname relative to "/".  If this is a script,
         * the interpreter will launch but fail to open the script because its
         * name ("/dev/fd/5/xxx....") is bigger than PATH_MAX.
         *
         * The failure code is usually 127 (POSIX: "If a command is not found,
         * the exit status shall be 127."), but some systems give 126 (POSIX:
         * "If the command name is found, but it is not an executable utility,
         * the exit status shall be 126."), so allow either.
         */
        if is_script != 0 {
            ksft_print_msg(c"Invoke script via root_dfd and relative filename\n".as_ptr());
            fail += check_execveat_invoked_rc(root_dfd, LONGPATH.as_ptr().add(1), 0, 127, 126);
        } else {
            ksft_print_msg(c"Invoke exec via root_dfd and relative filename\n".as_ptr());
            fail += check_execveat(root_dfd, LONGPATH.as_ptr().add(1), 0);
        }
    }

    fail
}

unsafe fn check_execveat_comm(fd: c_int, argv0: *mut c_char, expected: *mut c_char) -> c_int {
    let mut buf: [c_char; 128] = [0; 128];
    let old_env: *mut c_char;
    let old_argv0: *mut c_char;
    let ret: c_int;

    unsafe {
        snprintf(buf.as_mut_ptr(), buf.len(), c"CHECK_COMM=%s".as_ptr(), expected);

        old_env = ENVP[1];
        ENVP[1] = buf.as_mut_ptr();

        old_argv0 = ARGV[0];
        ARGV[0] = argv0;

        ksft_print_msg(
            c"Check execveat(AT_EMPTY_PATH)'s comm is %s\n".as_ptr(),
            expected,
        );
        ret = check_execveat_invoked_rc(fd, c"".as_ptr(), AT_EMPTY_PATH, 0, 0);

        ENVP[1] = old_env;
        ARGV[0] = old_argv0;
    }

    ret
}

unsafe fn run_tests() -> c_int {
    let mut fail: c_int = 0;
    let fullname = unsafe { realpath(c"execveat".as_ptr(), std::ptr::null_mut()) };
    let fullname_script = unsafe { realpath(c"script".as_ptr(), std::ptr::null_mut()) };
    let fullname_symlink = unsafe { concat(fullname, c".symlink".as_ptr()) };
    let subdir_dfd = unsafe { open_or_die(c"subdir".as_ptr(), O_DIRECTORY | O_RDONLY) };
    let subdir_dfd_ephemeral =
        unsafe { open_or_die(c"subdir.ephemeral".as_ptr(), O_DIRECTORY | O_RDONLY) };
    let dot_dfd = unsafe { open_or_die(c".".as_ptr(), O_DIRECTORY | O_RDONLY) };
    let root_dfd = unsafe { open_or_die(c"/".as_ptr(), O_DIRECTORY | O_RDONLY) };
    let dot_dfd_path = unsafe { open_or_die(c".".as_ptr(), O_DIRECTORY | O_RDONLY | O_PATH) };
    let dot_dfd_cloexec =
        unsafe { open_or_die(c".".as_ptr(), O_DIRECTORY | O_RDONLY | O_CLOEXEC) };
    let fd = unsafe { open_or_die(c"execveat".as_ptr(), O_RDONLY) };
    let fd_path = unsafe { open_or_die(c"execveat".as_ptr(), O_RDONLY | O_PATH) };
    let fd_symlink = unsafe { open_or_die(c"execveat.symlink".as_ptr(), O_RDONLY) };
    let fd_denatured = unsafe { open_or_die(c"execveat.denatured".as_ptr(), O_RDONLY) };
    let fd_denatured_path =
        unsafe { open_or_die(c"execveat.denatured".as_ptr(), O_RDONLY | O_PATH) };
    let fd_script = unsafe { open_or_die(c"script".as_ptr(), O_RDONLY) };
    let fd_ephemeral = unsafe { open_or_die(c"execveat.ephemeral".as_ptr(), O_RDONLY) };
    let fd_ephemeral_path =
        unsafe { open_or_die(c"execveat.path.ephemeral".as_ptr(), O_RDONLY | O_PATH) };
    let fd_script_ephemeral = unsafe { open_or_die(c"script.ephemeral".as_ptr(), O_RDONLY) };
    let fd_cloexec = unsafe { open_or_die(c"execveat".as_ptr(), O_RDONLY | O_CLOEXEC) };
    let fd_script_cloexec = unsafe { open_or_die(c"script".as_ptr(), O_RDONLY | O_CLOEXEC) };

    unsafe {
        /* Check if we have execveat at all, and bail early if not */
        set_errno(0);
        execveat_(
            -1,
            std::ptr::null(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            0,
        );
        if get_errno() == ENOSYS {
            ksft_exit_skip(c"ENOSYS calling execveat - no kernel support?\n".as_ptr());
        }

        /* Change file position to confirm it doesn't affect anything */
        lseek(fd, 10, SEEK_SET);

        /* Normal executable file: */
        /*   dfd + path */
        fail += check_execveat(subdir_dfd, c"../execveat".as_ptr(), 0);
        fail += check_execveat(dot_dfd, c"execveat".as_ptr(), 0);
        fail += check_execveat(dot_dfd_path, c"execveat".as_ptr(), 0);
        /*   absolute path */
        fail += check_execveat(AT_FDCWD, fullname, 0);
        /*   absolute path with nonsense dfd */
        fail += check_execveat(99, fullname, 0);
        /*   fd + no path */
        fail += check_execveat(fd, c"".as_ptr(), AT_EMPTY_PATH);
        /*   O_CLOEXEC fd + no path */
        fail += check_execveat(fd_cloexec, c"".as_ptr(), AT_EMPTY_PATH);
        /*   O_PATH fd */
        fail += check_execveat(fd_path, c"".as_ptr(), AT_EMPTY_PATH);

        /* Mess with executable file that's already open: */
        /*   fd + no path to a file that's been renamed */
        rename(c"execveat.ephemeral".as_ptr(), c"execveat.moved".as_ptr());
        fail += check_execveat(fd_ephemeral, c"".as_ptr(), AT_EMPTY_PATH);
        /*   fd + no path to a file that's been deleted */
        unlink(c"execveat.moved".as_ptr()); /* remove the file now fd open */
        fail += check_execveat(fd_ephemeral, c"".as_ptr(), AT_EMPTY_PATH);

        /* Mess with executable file that's already open with O_PATH */
        /*   fd + no path to a file that's been deleted */
        unlink(c"execveat.path.ephemeral".as_ptr());
        fail += check_execveat(fd_ephemeral_path, c"".as_ptr(), AT_EMPTY_PATH);

        /* Invalid argument failures */
        fail += check_execveat_fail!(fd, c"".as_ptr(), 0, ENOENT);
        fail += check_execveat_fail!(fd, std::ptr::null(), AT_EMPTY_PATH, EFAULT);

        /* Symlink to executable file: */
        /*   dfd + path */
        fail += check_execveat(dot_dfd, c"execveat.symlink".as_ptr(), 0);
        fail += check_execveat(dot_dfd_path, c"execveat.symlink".as_ptr(), 0);
        /*   absolute path */
        fail += check_execveat(AT_FDCWD, fullname_symlink, 0);
        /*   fd + no path, even with AT_SYMLINK_NOFOLLOW (already followed) */
        fail += check_execveat(fd_symlink, c"".as_ptr(), AT_EMPTY_PATH);
        fail += check_execveat(
            fd_symlink,
            c"".as_ptr(),
            AT_EMPTY_PATH | AT_SYMLINK_NOFOLLOW,
        );

        /* Symlink fails when AT_SYMLINK_NOFOLLOW set: */
        /*   dfd + path */
        fail += check_execveat_fail!(
            dot_dfd,
            c"execveat.symlink".as_ptr(),
            AT_SYMLINK_NOFOLLOW,
            ELOOP
        );
        fail += check_execveat_fail!(
            dot_dfd_path,
            c"execveat.symlink".as_ptr(),
            AT_SYMLINK_NOFOLLOW,
            ELOOP
        );
        /*   absolute path */
        fail += check_execveat_fail!(AT_FDCWD, fullname_symlink, AT_SYMLINK_NOFOLLOW, ELOOP);

        /*  Non-regular file failure */
        fail += check_execveat_fail!(dot_dfd, c"pipe".as_ptr(), 0, EACCES);
        unlink(c"pipe".as_ptr());

        /* Shell script wrapping executable file: */
        /*   dfd + path */
        fail += check_execveat(subdir_dfd, c"../script".as_ptr(), 0);
        fail += check_execveat(dot_dfd, c"script".as_ptr(), 0);
        fail += check_execveat(dot_dfd_path, c"script".as_ptr(), 0);
        /*   absolute path */
        fail += check_execveat(AT_FDCWD, fullname_script, 0);
        /*   fd + no path */
        fail += check_execveat(fd_script, c"".as_ptr(), AT_EMPTY_PATH);
        fail += check_execveat(
            fd_script,
            c"".as_ptr(),
            AT_EMPTY_PATH | AT_SYMLINK_NOFOLLOW,
        );
        /*   O_CLOEXEC fd fails for a script (as script file inaccessible) */
        fail += check_execveat_fail!(fd_script_cloexec, c"".as_ptr(), AT_EMPTY_PATH, ENOENT);
        fail += check_execveat_fail!(dot_dfd_cloexec, c"script".as_ptr(), 0, ENOENT);

        /* Mess with script file that's already open: */
        /*   fd + no path to a file that's been renamed */
        rename(c"script.ephemeral".as_ptr(), c"script.moved".as_ptr());
        fail += check_execveat(fd_script_ephemeral, c"".as_ptr(), AT_EMPTY_PATH);
        /*   fd + no path to a file that's been deleted */
        unlink(c"script.moved".as_ptr()); /* remove the file while fd open */
        fail += check_execveat(fd_script_ephemeral, c"".as_ptr(), AT_EMPTY_PATH);

        /* Rename a subdirectory in the path: */
        rename(c"subdir.ephemeral".as_ptr(), c"subdir.moved".as_ptr());
        fail += check_execveat(subdir_dfd_ephemeral, c"../script".as_ptr(), 0);
        fail += check_execveat(subdir_dfd_ephemeral, c"script".as_ptr(), 0);
        /* Remove the subdir and its contents */
        unlink(c"subdir.moved/script".as_ptr());
        unlink(c"subdir.moved".as_ptr());
        /* Shell loads via deleted subdir OK because name starts with .. */
        fail += check_execveat(subdir_dfd_ephemeral, c"../script".as_ptr(), 0);
        fail += check_execveat_fail!(subdir_dfd_ephemeral, c"script".as_ptr(), 0, ENOENT);

        /* Flag values other than AT_SYMLINK_NOFOLLOW => EINVAL */
        fail += check_execveat_fail!(dot_dfd, c"execveat".as_ptr(), 0xFFFF, EINVAL);
        /* Invalid path => ENOENT */
        fail += check_execveat_fail!(dot_dfd, c"no-such-file".as_ptr(), 0, ENOENT);
        fail += check_execveat_fail!(dot_dfd_path, c"no-such-file".as_ptr(), 0, ENOENT);
        fail += check_execveat_fail!(AT_FDCWD, c"no-such-file".as_ptr(), 0, ENOENT);
        /* Attempt to execute directory => EACCES */
        fail += check_execveat_fail!(dot_dfd, c"".as_ptr(), AT_EMPTY_PATH, EACCES);
        /* Attempt to execute non-executable => EACCES */
        fail += check_execveat_fail!(dot_dfd, c"Makefile".as_ptr(), 0, EACCES);
        fail += check_execveat_fail!(fd_denatured, c"".as_ptr(), AT_EMPTY_PATH, EACCES);
        fail += check_execveat_fail!(fd_denatured_path, c"".as_ptr(), AT_EMPTY_PATH, EACCES);
        /* Attempt to execute nonsense FD => EBADF */
        fail += check_execveat_fail!(99, c"".as_ptr(), AT_EMPTY_PATH, EBADF);
        fail += check_execveat_fail!(99, c"execveat".as_ptr(), 0, EBADF);
        /* Attempt to execute relative to non-directory => ENOTDIR */
        fail += check_execveat_fail!(fd, c"execveat".as_ptr(), 0, ENOTDIR);

        fail += check_execveat_pathmax(root_dfd, c"execveat".as_ptr(), 0);
        fail += check_execveat_pathmax(root_dfd, c"script".as_ptr(), 1);

        /* /proc/pid/comm gives filename by default */
        fail += check_execveat_comm(fd, c"sentinel".as_ptr() as *mut c_char, c"execveat".as_ptr() as *mut c_char);
        /* /proc/pid/comm gives argv[0] when invoked via link */
        fail += check_execveat_comm(fd_symlink, c"sentinel".as_ptr() as *mut c_char, c"execveat".as_ptr() as *mut c_char);
        /* /proc/pid/comm gives filename if NULL is passed */
        fail += check_execveat_comm(fd, std::ptr::null_mut(), c"execveat".as_ptr() as *mut c_char);
    }

    fail
}

unsafe fn prerequisites() {
    let fd: c_int;
    let script = c"#!/bin/bash\nexit $*\n";

    unsafe {
        /* Create ephemeral copies of files */
        exe_cp(c"execveat".as_ptr(), c"execveat.ephemeral".as_ptr());
        exe_cp(c"execveat".as_ptr(), c"execveat.path.ephemeral".as_ptr());
        exe_cp(c"script".as_ptr(), c"script.ephemeral".as_ptr());
        mkdir(c"subdir.ephemeral".as_ptr(), 0o755 as mode_t);

        fd = open(
            c"subdir.ephemeral/script".as_ptr(),
            O_RDWR | O_CREAT | O_TRUNC,
            0o755 as mode_t,
        );
        write(fd, script.as_ptr() as *const c_void, strlen(script.as_ptr()));
        close(fd);

        mkfifo(c"pipe".as_ptr(), 0o755 as mode_t);
    }
}

unsafe fn c_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut ii: c_int;
    let rc: c_int;
    let verbose = unsafe { getenv(c"VERBOSE".as_ptr()) };
    let check_comm = unsafe { getenv(CHECK_COMM.as_ptr() as *const c_char) };

    unsafe {
        if argc >= 2 || !check_comm.is_null() {
            /*
             * If we are invoked with an argument, or no arguments but a
             * command to check, don't run tests.
             */
            let in_test = getenv(c"IN_TEST".as_ptr());

            if !verbose.is_null() {
                ksft_print_msg(c"invoked with:\n".as_ptr());
                ii = 0;
                while ii < argc {
                    ksft_print_msg(c"\t[%d]='%s\n'".as_ptr(), ii, *argv.add(ii as usize));
                    ii += 1;
                }
            }

            /* If the tests wanted us to check the command, do so. */
            if !check_comm.is_null() {
                /* TASK_COMM_LEN == 16 */
                let mut buf: [c_char; 32] = [0; 32];
                let fd: c_int;
                let ret: c_int;

                fd = open(c"/proc/self/comm".as_ptr(), O_RDONLY);
                if fd < 0 {
                    ksft_perror(c"open() comm failed".as_ptr());
                    exit(1);
                }

                ret = read(fd, buf.as_mut_ptr() as *mut c_void, buf.len()) as c_int;
                if ret < 0 {
                    ksft_perror(c"read() comm failed".as_ptr());
                    close(fd);
                    exit(1);
                }
                close(fd);

                // trim off the \n
                buf[(ret - 1) as usize] = 0;

                if strcmp(buf.as_ptr(), check_comm) != 0 {
                    ksft_print_msg(
                        c"bad comm, got: %s expected: %s\n".as_ptr(),
                        buf.as_ptr(),
                        check_comm,
                    );
                    exit(1);
                }

                exit(0);
            }

            /* Check expected environment transferred. */
            if in_test.is_null() || strcmp(in_test, c"yes".as_ptr()) != 0 {
                ksft_print_msg(c"no IN_TEST=yes in env\n".as_ptr());
                return 1;
            }

            /* Use the final argument as an exit code. */
            rc = atoi(*argv.add((argc - 1) as usize));
            exit(rc);
        } else {
            ksft_print_header();
            ksft_set_plan(TESTS_EXPECTED);
            prerequisites();
            if !verbose.is_null() {
                ENVP[1] = ENV_VERBOSE.as_mut_ptr();
            }
            rc = run_tests();
            if rc > 0 {
                printf(c"%d tests failed\n".as_ptr(), rc);
            }
            ksft_finished();
        }
    }

    rc
}

fn main() {
    let mut args: Vec<*mut c_char> = std::env::args()
        .map(|arg| std::ffi::CString::new(arg).unwrap().into_raw())
        .collect();
    args.push(std::ptr::null_mut());

    let rc = unsafe { c_main((args.len() - 1) as c_int, args.as_mut_ptr()) };

    for arg in args.into_iter().take_while(|arg| !arg.is_null()) {
        unsafe {
            let _ = std::ffi::CString::from_raw(arg);
        }
    }

    std::process::exit(rc);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
