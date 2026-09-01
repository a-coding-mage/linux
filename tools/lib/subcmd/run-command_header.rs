/* SPDX-License-Identifier: GPL-2.0 */

/* C header dependency: <unistd.h> supplies pid_t. */
use core::ffi::{c_char, c_int};

pub type pid_t = c_int;

pub const ERR_RUN_COMMAND_FORK: c_int = 10000;
pub const ERR_RUN_COMMAND_EXEC: c_int = ERR_RUN_COMMAND_FORK + 1;
pub const ERR_RUN_COMMAND_PIPE: c_int = ERR_RUN_COMMAND_FORK + 2;
pub const ERR_RUN_COMMAND_WAITPID: c_int = ERR_RUN_COMMAND_FORK + 3;
pub const ERR_RUN_COMMAND_WAITPID_WRONG_PID: c_int = ERR_RUN_COMMAND_FORK + 4;
pub const ERR_RUN_COMMAND_WAITPID_SIGNAL: c_int = ERR_RUN_COMMAND_FORK + 5;
pub const ERR_RUN_COMMAND_WAITPID_NOEXIT: c_int = ERR_RUN_COMMAND_FORK + 6;

pub const fn IS_RUN_COMMAND_ERR(x: c_int) -> bool {
    -x >= ERR_RUN_COMMAND_FORK
}

#[repr(C)]
pub struct child_process {
    pub argv: *mut *const c_char,
    pub pid: pid_t,
    /*
     * Using .in, .out, .err:
     * - Specify 0 for no redirections (child inherits stdin, stdout,
     *   stderr from parent).
     * - Specify -1 to have a pipe allocated as follows:
     *     .in: returns the writable pipe end; parent writes to it,
     *          the readable pipe end becomes child's stdin
     *     .out, .err: returns the readable pipe end; parent reads from
     *          it, the writable pipe end becomes child's stdout/stderr
     *   The caller of start_command() must close the returned FDs
     *   after it has completed reading from/writing to it!
     * - Specify > 0 to set a channel to a particular FD as follows:
     *     .in: a readable FD, becomes child's stdin
     *     .out: a writable FD, becomes child's stdout/stderr
     *     .err > 0 not supported
     *   The specified FD is closed by start_command(), even in case
     *   of errors!
     */
    pub in_: c_int,
    pub out: c_int,
    pub err: c_int,
    pub dir: *const c_char,
    pub env: *const *const c_char,
    pub finish_result: c_int,
    /* C bitfields: unsigned no_stdin:1; ... finished:1; */
    pub no_stdin: c_int,
    pub no_stdout: c_int,
    pub no_stderr: c_int,
    pub exec_cmd: c_int, /* if this is to be external sub-command */
    pub stdout_to_stderr: c_int,
    pub finished: c_int,
    pub preexec_cb: Option<unsafe extern "C" fn()>,
    /* If set, call function in child rather than doing an exec. */
    pub no_exec_cmd: Option<unsafe extern "C" fn(process: *mut child_process) -> c_int>,
}

unsafe extern "C" {
    pub fn start_command(arg1: *mut child_process) -> c_int;
    pub fn check_if_command_finished(arg1: *mut child_process) -> c_int;
    pub fn finish_command(arg1: *mut child_process) -> c_int;
    pub fn run_command(arg1: *mut child_process) -> c_int;
}

pub const RUN_COMMAND_NO_STDIN: c_int = 1;
pub const RUN_EXEC_CMD: c_int = 2; /*If this is to be external sub-command */
pub const RUN_COMMAND_STDOUT_TO_STDERR: c_int = 4;

unsafe extern "C" {
    pub fn run_command_v_opt(argv: *mut *const c_char, opt: c_int) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
