// SPDX-License-Identifier: GPL-2.0-only

// C source included <linux/wait.h>; wait status helpers are translated below.

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long};
use core::ptr;

const THIS_PROGRAM: &[u8] = b"./vstate_exec_nolibc\0";

// Constants supplied by the surrounding Linux/nolibc build environment in C.
// TODO: bind these preprocessor constants from the repository's Rust support.
const PR_RISCV_V_GET_CONTROL: c_int = 0;
const PR_RISCV_V_VSTATE_CTRL_CUR_MASK: c_long = 0;
const PR_RISCV_V_VSTATE_CTRL_OFF: c_long = 0;
const PR_RISCV_V_VSTATE_CTRL_INHERIT: c_long = 0;
const PR_RISCV_V_VSTATE_CTRL_NEXT_MASK: c_long = 0;
const PR_RISCV_V_VSTATE_CTRL_DEFAULT: c_int = 0;
const SIGILL: c_int = 4;

unsafe extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn prctl(option: c_int, ...) -> c_long;
    fn puts(s: *const c_char) -> c_int;
    fn exit(status: c_int) -> !;
    fn fork() -> c_int;
    fn execve(
        pathname: *const c_char,
        argv: *const *mut c_char,
        envp: *const *mut c_char,
    ) -> c_int;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
}

#[inline]
fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

#[inline]
fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

#[inline]
fn wifsignaled(status: c_int) -> bool {
    (((status & 0x7f) + 1) as i8) >= 2
}

#[inline]
fn wtermsig(status: c_int) -> c_int {
    status & 0x7f
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut rc: c_int;
    let mut pid: c_int;
    let mut status: c_int = 0;
    let mut test_inherit: c_int = 0;
    let mut xtheadvector: c_int = 0;
    let ctrl: c_long;
    let ctrl_c: c_long;
    let mut exec_argv: [*mut c_char; 2] = [ptr::null_mut(); 2];
    let mut exec_envp: [*mut c_char; 2] = [ptr::null_mut(); 2];

    if argc > 1 && strcmp(*argv.add(1), c"x".as_ptr()) != 0 {
        test_inherit = 1;
    }

    if argc > 2 && strcmp(*argv.add(2), c"x".as_ptr()) != 0 {
        xtheadvector = 1;
    }

    ctrl = prctl(PR_RISCV_V_GET_CONTROL, 0, 0, 0, 0);
    if ctrl == -1 {
        puts(c"PR_RISCV_V_GET_CONTROL is not supported\n".as_ptr());
        exit(-1);
    }

    if test_inherit != 0 {
        pid = fork();
        if pid == -1 {
            puts(c"fork failed\n".as_ptr());
            exit(-1);
        }

        /* child  */
        if pid == 0 {
            exec_argv[0] = THIS_PROGRAM.as_ptr() as *mut c_char;
            exec_argv[1] = ptr::null_mut();
            exec_envp[0] = ptr::null_mut();
            exec_envp[1] = ptr::null_mut();
            /* launch the program again to check inherit */
            rc = execve(THIS_PROGRAM.as_ptr() as *const c_char, exec_argv.as_ptr(), exec_envp.as_ptr());
            if rc != 0 {
                puts(c"child execve failed\n".as_ptr());
                exit(-1);
            }
        }
    } else {
        pid = fork();
        if pid == -1 {
            puts(c"fork failed\n".as_ptr());
            exit(-1);
        }

        if pid == 0 {
            rc = prctl(PR_RISCV_V_GET_CONTROL, 0, 0, 0, 0) as c_int;
            if (rc as c_long) != ctrl {
                puts(c"child's vstate_ctrl not equal to parent's\n".as_ptr());
                exit(-1);
            }
            if xtheadvector != 0 {
                asm!(".4byte 0x00007ed7", options(nostack, preserves_flags));
            } else {
                asm!(
                    ".option push",
                    ".option arch, +v",
                    "vsetvli x0, x0, e32, m8, ta, ma",
                    ".option pop",
                    options(nostack, preserves_flags)
                );
            }
            exit(ctrl as c_int);
        }
    }

    rc = waitpid(-1, &mut status, 0);

    if wifexited(status) && wexitstatus(status) == -1 {
        puts(c"child exited abnormally\n".as_ptr());
        exit(-1);
    }

    if wifsignaled(status) {
        if wtermsig(status) != SIGILL {
            puts(c"child was terminated by unexpected signal\n".as_ptr());
            exit(-1);
        }

        if (ctrl & PR_RISCV_V_VSTATE_CTRL_CUR_MASK) != PR_RISCV_V_VSTATE_CTRL_OFF {
            puts(c"child signaled by illegal V access but vstate_ctrl is not off\n".as_ptr());
            exit(-1);
        }

        /* child terminated, and its vstate_ctrl is off */
        exit(ctrl as c_int);
    }

    ctrl_c = wexitstatus(status) as c_long;
    if test_inherit != 0 {
        if (ctrl & PR_RISCV_V_VSTATE_CTRL_INHERIT) != 0 {
            if (ctrl_c & PR_RISCV_V_VSTATE_CTRL_INHERIT) == 0 {
                puts(c"parent has inherit bit, but child has not\n".as_ptr());
                exit(-1);
            }
        }
        rc = ((ctrl & PR_RISCV_V_VSTATE_CTRL_NEXT_MASK) >> 2) as c_int;
        if rc != PR_RISCV_V_VSTATE_CTRL_DEFAULT {
            if (rc as c_long) != (ctrl_c & PR_RISCV_V_VSTATE_CTRL_CUR_MASK) {
                puts(c"parent's next setting does not equal to child's\n".as_ptr());
                exit(-1);
            }

            if (ctrl & PR_RISCV_V_VSTATE_CTRL_INHERIT) == 0 {
                if (ctrl_c & PR_RISCV_V_VSTATE_CTRL_NEXT_MASK)
                    != PR_RISCV_V_VSTATE_CTRL_DEFAULT as c_long
                {
                    puts(c"must clear child's next vstate_ctrl if !inherit\n".as_ptr());
                    exit(-1);
                }
            }
        }
    }
    ctrl as c_int
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
