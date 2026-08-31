// SPDX-License-Identifier: GPL-2.0
// Translated from lib/subcmd/run-command.c.
// Original C includes: unistd.h, sys/types.h, sys/stat.h, ctype.h, fcntl.h,
// string.h, linux/compiler.h, linux/string.h, errno.h, sys/wait.h,
// subcmd-util.h, run-command.h, exec-cmd.h.

use std::ffi::c_void;
use std::mem::size_of;
use std::os::raw::{c_char, c_int, c_uint};
use std::ptr;

const STRERR_BUFSIZE: usize = 128;
const O_RDWR: c_int = 0o2;
const WNOHANG: c_int = 1;
const ENOENT: c_int = 2;
const EINTR: c_int = 4;
const ECHILD: c_int = 10;
const ESRCH: c_int = 3;

extern "C" {
    static mut errno: c_int;

    fn close(fd: c_int) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn fork() -> pid_t;
    fn chdir(path: *const c_char) -> c_int;
    fn putenv(string: *mut c_char) -> c_int;
    fn unsetenv(name: *const c_char) -> c_int;
    fn execvp(file: *const c_char, argv: *const *mut c_char) -> c_int;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn exit(status: c_int) -> !;
    fn fflush(stream: *mut FILE) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn sprintf(str_: *mut c_char, format: *const c_char, ...) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn isspace(c: c_int) -> c_int;

    static mut stderr: *mut FILE;

    fn die(format: *const c_char, ...) -> !;
    fn str_error_r(errnum: c_int, buf: *mut c_char, buflen: usize) -> *mut c_char;
    fn execv_cmd(argv: *const *const c_char);
}

#[allow(non_camel_case_types)]
type pid_t = c_int;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct child_process {
    pub argv: *const *const c_char,
    pub env: *mut *const c_char,
    pub pid: pid_t,
    pub in_: c_int,
    pub out: c_int,
    pub err: c_int,
    pub no_stdin: c_uint,
    pub no_stdout: c_uint,
    pub no_stderr: c_uint,
    pub exec_cmd: c_uint,
    pub no_exec_cmd: Option<unsafe extern "C" fn(*mut child_process) -> c_int>,
    pub stdout_to_stderr: c_uint,
    pub dir: *const c_char,
    pub preexec_cb: Option<unsafe extern "C" fn()>,
    pub finished: c_uint,
    pub finish_result: c_int,
}

extern "C" {
    static ERR_RUN_COMMAND_PIPE: c_int;
    static ERR_RUN_COMMAND_EXEC: c_int;
    static ERR_RUN_COMMAND_FORK: c_int;
    static ERR_RUN_COMMAND_WAITPID: c_int;
    static ERR_RUN_COMMAND_WAITPID_WRONG_PID: c_int;
    static ERR_RUN_COMMAND_WAITPID_SIGNAL: c_int;
    static ERR_RUN_COMMAND_WAITPID_NOEXIT: c_int;
    static RUN_COMMAND_NO_STDIN: c_int;
    static RUN_EXEC_CMD: c_int;
    static RUN_COMMAND_STDOUT_TO_STDERR: c_int;
}

unsafe fn wifsignaled(status: c_int) -> bool {
    (status & 0x7f) != 0 && (status & 0x7f) != 0x7f
}

unsafe fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

#[inline]
unsafe fn close_pair(fd: *mut c_int) {
    close(*fd.add(0));
    close(*fd.add(1));
}

#[inline]
unsafe fn dup_devnull(to: c_int) {
    let fd = open(c"/dev/null".as_ptr(), O_RDWR);
    dup2(fd, to);
    close(fd);
}

#[no_mangle]
pub unsafe extern "C" fn start_command(cmd: *mut child_process) -> c_int {
    let mut need_in: c_int;
    let mut need_out: c_int;
    let mut need_err: c_int;
    let mut fdin = [0 as c_int; 2];
    let mut fdout = [0 as c_int; 2];
    let mut fderr = [0 as c_int; 2];
    let mut sbuf = [0 as c_char; STRERR_BUFSIZE];

    /*
     * In case of errors we must keep the promise to close FDs
     * that have been passed in via ->in and ->out.
     */

    need_in = ((*cmd).no_stdin == 0 && (*cmd).in_ < 0) as c_int;
    if need_in != 0 {
        if pipe(fdin.as_mut_ptr()) < 0 {
            if (*cmd).out > 0 {
                close((*cmd).out);
            }
            return -ERR_RUN_COMMAND_PIPE;
        }
        (*cmd).in_ = fdin[1];
    }

    need_out = ((*cmd).no_stdout == 0 && (*cmd).stdout_to_stderr == 0 && (*cmd).out < 0) as c_int;
    if need_out != 0 {
        if pipe(fdout.as_mut_ptr()) < 0 {
            if need_in != 0 {
                close_pair(fdin.as_mut_ptr());
            } else if (*cmd).in_ != 0 {
                close((*cmd).in_);
            }
            return -ERR_RUN_COMMAND_PIPE;
        }
        (*cmd).out = fdout[0];
    }

    need_err = ((*cmd).no_stderr == 0 && (*cmd).err < 0) as c_int;
    if need_err != 0 {
        if pipe(fderr.as_mut_ptr()) < 0 {
            if need_in != 0 {
                close_pair(fdin.as_mut_ptr());
            } else if (*cmd).in_ != 0 {
                close((*cmd).in_);
            }
            if need_out != 0 {
                close_pair(fdout.as_mut_ptr());
            } else if (*cmd).out != 0 {
                close((*cmd).out);
            }
            return -ERR_RUN_COMMAND_PIPE;
        }
        (*cmd).err = fderr[0];
    }

    fflush(ptr::null_mut());
    (*cmd).pid = fork();
    if (*cmd).pid == 0 {
        if (*cmd).no_stdin != 0 {
            dup_devnull(0);
        } else if need_in != 0 {
            dup2(fdin[0], 0);
            close_pair(fdin.as_mut_ptr());
        } else if (*cmd).in_ != 0 {
            dup2((*cmd).in_, 0);
            close((*cmd).in_);
        }

        if (*cmd).no_stderr != 0 {
            dup_devnull(2);
        } else if need_err != 0 {
            dup2(fderr[1], 2);
            close_pair(fderr.as_mut_ptr());
        }

        if (*cmd).no_stdout != 0 {
            dup_devnull(1);
        } else if (*cmd).stdout_to_stderr != 0 {
            dup2(2, 1);
        } else if need_out != 0 {
            dup2(fdout[1], 1);
            close_pair(fdout.as_mut_ptr());
        } else if (*cmd).out > 1 {
            dup2((*cmd).out, 1);
            close((*cmd).out);
        }

        if !(*cmd).dir.is_null() && chdir((*cmd).dir) != 0 {
            die(
                c"exec %s: cd to %s failed (%s)".as_ptr(),
                *(*cmd).argv.add(0),
                (*cmd).dir,
                str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()),
            );
        }
        if !(*cmd).env.is_null() {
            while !(*(*cmd).env).is_null() {
                if !strchr(*(*cmd).env, b'=' as c_int).is_null() {
                    putenv(*(*cmd).env as *mut c_char);
                } else {
                    unsetenv(*(*cmd).env);
                }
                (*cmd).env = (*cmd).env.add(1);
            }
        }
        if let Some(preexec_cb) = (*cmd).preexec_cb {
            preexec_cb();
        }
        if let Some(no_exec_cmd) = (*cmd).no_exec_cmd {
            exit(no_exec_cmd(cmd));
        }
        if (*cmd).exec_cmd != 0 {
            execv_cmd((*cmd).argv);
        } else {
            execvp(*(*cmd).argv.add(0), (*cmd).argv as *const *mut c_char);
        }
        exit(127);
    }

    if (*cmd).pid < 0 {
        let err = errno;
        if need_in != 0 {
            close_pair(fdin.as_mut_ptr());
        } else if (*cmd).in_ != 0 {
            close((*cmd).in_);
        }
        if need_out != 0 {
            close_pair(fdout.as_mut_ptr());
        } else if (*cmd).out != 0 {
            close((*cmd).out);
        }
        if need_err != 0 {
            close_pair(fderr.as_mut_ptr());
        }
        return if err == ENOENT {
            -ERR_RUN_COMMAND_EXEC
        } else {
            -ERR_RUN_COMMAND_FORK
        };
    }

    if need_in != 0 {
        close(fdin[0]);
    } else if (*cmd).in_ != 0 {
        close((*cmd).in_);
    }

    if need_out != 0 {
        close(fdout[1]);
    } else if (*cmd).out != 0 {
        close((*cmd).out);
    }

    if need_err != 0 {
        close(fderr[1]);
    }

    0
}

unsafe fn wait_or_whine(cmd: *mut child_process, block: bool) -> c_int {
    let mut finished: bool;
    let mut result: c_int;

    if (*cmd).pid <= 0 {
        (*cmd).finished = 1;
        if (*cmd).pid < 0 && (*cmd).finish_result == 0 {
            (*cmd).finish_result = -ERR_RUN_COMMAND_FORK;
        }
        return (*cmd).finish_result;
    }

    finished = (*cmd).finished != 0;
    result = (*cmd).finish_result;

    while !finished {
        let mut status: c_int = 0;
        let code: c_int;
        let waiting = waitpid((*cmd).pid, &mut status, if block { 0 } else { WNOHANG });

        if !block && waiting == 0 {
            break;
        }

        if waiting < 0 && errno == EINTR {
            continue;
        }

        finished = true;
        if waiting < 0 {
            let mut sbuf = [0 as c_char; STRERR_BUFSIZE];

            fprintf(
                stderr,
                c" Error: waitpid failed (%s)".as_ptr(),
                str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()),
            );
            result = -ERR_RUN_COMMAND_WAITPID;
        } else if waiting != (*cmd).pid {
            result = -ERR_RUN_COMMAND_WAITPID_WRONG_PID;
        } else if wifsignaled(status) {
            result = -ERR_RUN_COMMAND_WAITPID_SIGNAL;
        } else if !wifexited(status) {
            result = -ERR_RUN_COMMAND_WAITPID_NOEXIT;
        } else {
            code = wexitstatus(status);
            match code {
                127 => {
                    result = -ERR_RUN_COMMAND_EXEC;
                }
                0 => {
                    result = 0;
                }
                _ => {
                    result = -code;
                }
            }
        }
    }
    if finished {
        (*cmd).finished = 1;
        (*cmd).finish_result = result;
    }
    result
}

/*
 * Conservative estimate of number of characaters needed to hold an a decoded
 * integer, assume each 3 bits needs a character byte and plus a possible sign
 * character.
 */
const fn is_signed_pid_t() -> bool {
    true
}

const fn max_strlen_pid_t() -> usize {
    size_of::<pid_t>() * 8 / 3 + if is_signed_pid_t() { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn check_if_command_finished(cmd: *mut child_process) -> c_int {
    // Original C uses an __linux__ conditional. This translation preserves the
    // Linux /proc probing branch used by the source on Linux.
    let mut filename = [0 as c_char; 6 + max_strlen_pid_t() + 7 + 1];
    let mut status_line = [0 as c_char; 256];
    let status_file: *mut FILE;

    if (*cmd).finished != 0 {
        return 1;
    }
    if (*cmd).pid <= 0 {
        (*cmd).finished = 1;
        if (*cmd).pid < 0 && (*cmd).finish_result == 0 {
            (*cmd).finish_result = -ERR_RUN_COMMAND_FORK;
        }
        return 1;
    }

    /*
     * Check by reading /proc/<pid>/status as calling waitpid causes
     * stdout/stderr to be closed and data lost.
     */
    sprintf(filename.as_mut_ptr(), c"/proc/%u/status".as_ptr(), (*cmd).pid as c_uint);
    status_file = fopen(filename.as_mut_ptr(), c"r".as_ptr());
    if status_file.is_null() {
        let mut status: c_int = 0;
        let waiting: pid_t;

        /*
         * fopen() can fail with ENOENT if the process has been reaped.
         * It can also fail with EMFILE/ENFILE if RLIMIT_NOFILE is reached.
         * In those cases, use waitpid(..., WNOHANG) to robustly check
         * and reap the process if it has exited.
         */
        if errno == ENOENT {
            return 1;
        }

        waiting = waitpid((*cmd).pid, &mut status, WNOHANG);
        if waiting == (*cmd).pid {
            let result: c_int;
            let code: c_int;

            (*cmd).finished = 1;
            if wifsignaled(status) {
                result = -ERR_RUN_COMMAND_WAITPID_SIGNAL;
            } else if !wifexited(status) {
                result = -ERR_RUN_COMMAND_WAITPID_NOEXIT;
            } else {
                code = wexitstatus(status);
                match code {
                    127 => {
                        result = -ERR_RUN_COMMAND_EXEC;
                    }
                    0 => {
                        result = 0;
                    }
                    _ => {
                        result = -code;
                    }
                }
            }
            (*cmd).finish_result = result;
            return 1;
        }
        if waiting < 0 && (errno == ECHILD || errno == ESRCH) {
            return 1;
        }
        return 0;
    }
    while !fgets(status_line.as_mut_ptr(), status_line.len() as c_int, status_file).is_null() {
        let mut p: *mut c_char;

        if strncmp(status_line.as_mut_ptr(), c"State:".as_ptr(), 6) != 0 {
            continue;
        }

        fclose(status_file);
        p = status_line.as_mut_ptr().add(6);
        while isspace(*p as c_int) != 0 {
            p = p.add(1);
        }
        return if *p == b'Z' as c_char { 1 } else { 0 };
    }
    /* Read failed assume finish_command was called. */
    fclose(status_file);
    1
}

#[no_mangle]
pub unsafe extern "C" fn finish_command(cmd: *mut child_process) -> c_int {
    wait_or_whine(cmd, true)
}

#[no_mangle]
pub unsafe extern "C" fn run_command(cmd: *mut child_process) -> c_int {
    let code = start_command(cmd);
    if code != 0 {
        return code;
    }
    finish_command(cmd)
}

unsafe fn prepare_run_command_v_opt(
    cmd: *mut child_process,
    argv: *const *const c_char,
    opt: c_int,
) {
    memset(
        cmd as *mut c_void,
        0,
        size_of::<child_process>(),
    );
    (*cmd).argv = argv;
    (*cmd).no_stdin = if opt & RUN_COMMAND_NO_STDIN != 0 { 1 } else { 0 };
    (*cmd).exec_cmd = if opt & RUN_EXEC_CMD != 0 { 1 } else { 0 };
    (*cmd).stdout_to_stderr = if opt & RUN_COMMAND_STDOUT_TO_STDERR != 0 { 1 } else { 0 };
}

#[no_mangle]
pub unsafe extern "C" fn run_command_v_opt(argv: *const *const c_char, opt: c_int) -> c_int {
    let mut cmd: child_process = std::mem::zeroed();
    prepare_run_command_v_opt(&mut cmd, argv, opt);
    run_command(&mut cmd)
}
