/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2021 Cadence Design Systems Inc. */

/*
 *  System call like services offered by the simulator host.
 */

pub const SYS_nop: i32 = 0; /* unused */
pub const SYS_exit: i32 = 1; /*x*/
pub const SYS_fork: i32 = 2;
pub const SYS_read: i32 = 3; /*x*/
pub const SYS_write: i32 = 4; /*x*/
pub const SYS_open: i32 = 5; /*x*/
pub const SYS_close: i32 = 6; /*x*/
pub const SYS_rename: i32 = 7; /*x 38 - waitpid */
pub const SYS_creat: i32 = 8; /*x*/
pub const SYS_link: i32 = 9; /*x (not implemented on WIN32) */
pub const SYS_unlink: i32 = 10; /*x*/
pub const SYS_execv: i32 = 11; /* n/a - execve */
pub const SYS_execve: i32 = 12; /* 11 - chdir */
pub const SYS_pipe: i32 = 13; /* 42 - time */
pub const SYS_stat: i32 = 14; /* 106 - mknod */
pub const SYS_chmod: i32 = 15;
pub const SYS_chown: i32 = 16; /* 202 - lchown */
pub const SYS_utime: i32 = 17; /* 30 - break */
pub const SYS_wait: i32 = 18; /* n/a - oldstat */
pub const SYS_lseek: i32 = 19; /*x*/
pub const SYS_getpid: i32 = 20;
pub const SYS_isatty: i32 = 21; /* n/a - mount */
pub const SYS_fstat: i32 = 22; /* 108 - oldumount */
pub const SYS_time: i32 = 23; /* 13 - setuid */
pub const SYS_gettimeofday: i32 = 24; /*x 78 - getuid (not implemented on WIN32) */
pub const SYS_times: i32 = 25; /*X 43 - stime (Xtensa-specific implementation) */
pub const SYS_socket: i32 = 26;
pub const SYS_sendto: i32 = 27;
pub const SYS_recvfrom: i32 = 28;
pub const SYS_select_one: i32 = 29; /* not compatible select, one file descriptor at the time */
pub const SYS_bind: i32 = 30;
pub const SYS_ioctl: i32 = 31;

pub const SYS_iss_argc: i32 = 1000; /* returns value of argc */
pub const SYS_iss_argv_size: i32 = 1001; /* bytes needed for argv & arg strings */
pub const SYS_iss_set_argv: i32 = 1002; /* saves argv & arg strings at given addr */

/*
 * SYS_select_one specifiers
 */

pub const XTISS_SELECT_ONE_READ: i32 = 1;
pub const XTISS_SELECT_ONE_WRITE: i32 = 2;
pub const XTISS_SELECT_ONE_EXCEPT: i32 = 3;

static mut errno: i32 = 0;

pub unsafe fn __simc(a: i32, b: i32, c: i32, d: i32) -> i32 {
    let mut a1 = a;
    let mut b1 = b;
    let c1 = c;
    let d1 = d;
    core::arch::asm!(
        "simcall",
        inout("a2") a1,
        inout("a3") b1,
        in("a4") c1,
        in("a5") d1,
        options(nostack)
    );
    errno = b1;
    a1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
