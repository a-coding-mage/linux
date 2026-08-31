// SPDX-License-Identifier: GPL-2.0-only

// C dependencies removed from executable Rust:
// ../hwprobe/hwprobe.h, asm/vendor/thead.h, stdbool.h, stdlib.h, stdio.h,
// unistd.h, sys/wait.h. The constants and external functions below are
// expected to be supplied by the surrounding translated repository.

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

#[repr(C)]
pub struct riscv_hwprobe {
    pub key: c_long,
    pub value: c_ulong,
}

unsafe extern "C" {
    static RISCV_HWPROBE_KEY_VENDOR_EXT_THEAD_0: c_long;
    static RISCV_HWPROBE_VENDOR_EXT_XTHEADVECTOR: c_ulong;
    static RISCV_HWPROBE_KEY_IMA_EXT_0: c_long;
    static RISCV_HWPROBE_EXT_ZVE32X: c_ulong;

    fn riscv_hwprobe(
        pairs: *mut riscv_hwprobe,
        pair_count: usize,
        cpu_count: usize,
        cpus: *mut c_void,
        flags: c_uint,
    ) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn fork() -> c_int;
    fn execve(
        pathname: *const c_char,
        argv: *const *mut c_char,
        envp: *const *mut c_char,
    ) -> c_int;
    fn exit(status: c_int) -> !;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
}

type c_uint = u32;

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
    let termsig = status & 0x7f;
    termsig != 0 && termsig != 0x7f
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_xtheadvector_supported() -> bool {
    let mut pair: riscv_hwprobe = riscv_hwprobe { key: 0, value: 0 };

    unsafe {
        pair.key = RISCV_HWPROBE_KEY_VENDOR_EXT_THEAD_0;
        riscv_hwprobe(&mut pair, 1, 0, core::ptr::null_mut(), 0);
        (pair.value & RISCV_HWPROBE_VENDOR_EXT_XTHEADVECTOR) != 0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_vector_supported() -> bool {
    let mut pair: riscv_hwprobe = riscv_hwprobe { key: 0, value: 0 };

    unsafe {
        pair.key = RISCV_HWPROBE_KEY_IMA_EXT_0;
        riscv_hwprobe(&mut pair, 1, 0, core::ptr::null_mut(), 0);
        (pair.value & RISCV_HWPROBE_EXT_ZVE32X) != 0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_vr_len() -> c_ulong {
    let vlenb: c_ulong;

    if unsafe { is_vector_supported() } {
        unsafe {
            asm!("csrr {vlenb}, vlenb", vlenb = out(reg) vlenb, options(nostack, preserves_flags));
        }
        return vlenb;
    }

    if unsafe { is_xtheadvector_supported() } {
        unsafe {
            asm!(
                // 0 | zimm[10:0] | rs1 | 1 1 1 | rd | 1010111 | vsetvli
                // vsetvli	t4, x0, e8, m1, d1
                ".4byte		0b00000000000000000111111011010111",
                "mv		{vlenb}, t4",
                vlenb = out(reg) vlenb,
                lateout("t4") _,
                options(nostack)
            );
        }
        return vlenb;
    }

    unsafe {
        printf(c"WARNING: vector not supported\n".as_ptr());
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn launch_test(
    next_program: *mut c_char,
    test_inherit: c_int,
    xtheadvector: c_int,
) -> c_int {
    let mut exec_argv: [*mut c_char; 4] = [core::ptr::null_mut(); 4];
    let mut exec_envp: [*mut c_char; 1] = [core::ptr::null_mut(); 1];
    let mut rc: c_int;
    let pid: c_int;
    let mut status: c_int = 0;

    unsafe {
        pid = fork();
    }
    if pid < 0 {
        unsafe {
            printf(c"fork failed %d".as_ptr(), pid);
        }
        return -1;
    }

    if pid == 0 {
        exec_argv[0] = next_program;
        exec_argv[1] = if test_inherit != 0 {
            c"x".as_ptr() as *mut c_char
        } else {
            core::ptr::null_mut()
        };
        exec_argv[2] = if xtheadvector != 0 {
            c"x".as_ptr() as *mut c_char
        } else {
            core::ptr::null_mut()
        };
        exec_argv[3] = core::ptr::null_mut();
        exec_envp[0] = core::ptr::null_mut();
        /* launch the program again to check inherit */
        unsafe {
            rc = execve(next_program, exec_argv.as_ptr(), exec_envp.as_ptr());
        }
        if rc != 0 {
            unsafe {
                perror(c"execve".as_ptr());
                printf(c"child execve failed %d\n".as_ptr(), rc);
                exit(-1);
            }
        }
    }

    unsafe {
        rc = waitpid(-1, &mut status, 0);
    }
    if rc < 0 {
        unsafe {
            printf(c"waitpid failed\n".as_ptr());
        }
        return -3;
    }

    if (wifexited(status) && wexitstatus(status) == -1) || wifsignaled(status) {
        unsafe {
            printf(c"child exited abnormally\n".as_ptr());
        }
        return -4;
    }

    wexitstatus(status)
}
