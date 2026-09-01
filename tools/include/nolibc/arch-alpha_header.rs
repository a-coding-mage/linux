/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * Alpha specific definitions for NOLIBC
 * Copyright (C) 2025 Thomas Weißschuh <linux@weissschuh.net>
 */

/*
 * C header dependencies removed from executable Rust:
 *   - compiler.h
 *   - crt.h
 */

/*
 * Syscalls for Alpha:
 *   - registers are 64-bit
 *   - syscall number is passed in $0/v0
 *   - the system call is performed by calling callsys
 *   - syscall return comes in $0/v0, error flag in $19/a3
 *   - arguments are passed in $16/a0 to $21/a5
 *   - GCC does not support symbol register names
 */

/* Original C clobber list:
 * "$1", "$2", "$3", "$4", "$5", "$6", "$7", "$8",
 * "$22", "$23", "$24", "$25", "$27", "$28", "memory", "cc"
 */

macro_rules! __nolibc_syscall0 {
    ($num:expr) => {{
        let mut _num: i64 = $num as i64;
        let mut _err: i64;

        unsafe {
            core::arch::asm!(
                "callsys",
                inout("$0") _num,
                lateout("$19") _err,
                lateout("$1") _,
                lateout("$2") _,
                lateout("$3") _,
                lateout("$4") _,
                lateout("$5") _,
                lateout("$6") _,
                lateout("$7") _,
                lateout("$8") _,
                lateout("$16") _,
                lateout("$17") _,
                lateout("$18") _,
                lateout("$20") _,
                lateout("$21") _,
                lateout("$22") _,
                lateout("$23") _,
                lateout("$24") _,
                lateout("$25") _,
                lateout("$27") _,
                lateout("$28") _,
            );
        }
        if _err != 0 { -_num } else { _num }
    }};
}

macro_rules! __nolibc_syscall1 {
    ($num:expr, $arg1:expr) => {{
        let mut _num: i64 = $num as i64;
        let mut _err: i64;
        let _arg1: i64 = $arg1 as i64;

        unsafe {
            core::arch::asm!(
                "callsys",
                inout("$0") _num,
                lateout("$19") _err,
                in("$16") _arg1,
                lateout("$1") _,
                lateout("$2") _,
                lateout("$3") _,
                lateout("$4") _,
                lateout("$5") _,
                lateout("$6") _,
                lateout("$7") _,
                lateout("$8") _,
                lateout("$17") _,
                lateout("$18") _,
                lateout("$20") _,
                lateout("$21") _,
                lateout("$22") _,
                lateout("$23") _,
                lateout("$24") _,
                lateout("$25") _,
                lateout("$27") _,
                lateout("$28") _,
            );
        }
        if _err != 0 { -_num } else { _num }
    }};
}

macro_rules! __nolibc_syscall2 {
    ($num:expr, $arg1:expr, $arg2:expr) => {{
        let mut _num: i64 = $num as i64;
        let mut _err: i64;
        let _arg1: i64 = $arg1 as i64;
        let _arg2: i64 = $arg2 as i64;

        unsafe {
            core::arch::asm!(
                "callsys",
                inout("$0") _num,
                lateout("$19") _err,
                in("$16") _arg1,
                in("$17") _arg2,
                lateout("$1") _,
                lateout("$2") _,
                lateout("$3") _,
                lateout("$4") _,
                lateout("$5") _,
                lateout("$6") _,
                lateout("$7") _,
                lateout("$8") _,
                lateout("$18") _,
                lateout("$20") _,
                lateout("$21") _,
                lateout("$22") _,
                lateout("$23") _,
                lateout("$24") _,
                lateout("$25") _,
                lateout("$27") _,
                lateout("$28") _,
            );
        }
        if _err != 0 { -_num } else { _num }
    }};
}

macro_rules! __nolibc_syscall3 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr) => {{
        let mut _num: i64 = $num as i64;
        let mut _err: i64;
        let _arg1: i64 = $arg1 as i64;
        let _arg2: i64 = $arg2 as i64;
        let _arg3: i64 = $arg3 as i64;

        unsafe {
            core::arch::asm!(
                "callsys",
                inout("$0") _num,
                lateout("$19") _err,
                in("$16") _arg1,
                in("$17") _arg2,
                in("$18") _arg3,
                lateout("$1") _,
                lateout("$2") _,
                lateout("$3") _,
                lateout("$4") _,
                lateout("$5") _,
                lateout("$6") _,
                lateout("$7") _,
                lateout("$8") _,
                lateout("$20") _,
                lateout("$21") _,
                lateout("$22") _,
                lateout("$23") _,
                lateout("$24") _,
                lateout("$25") _,
                lateout("$27") _,
                lateout("$28") _,
            );
        }
        if _err != 0 { -_num } else { _num }
    }};
}

macro_rules! __nolibc_syscall4 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {{
        let mut _num: i64 = $num as i64;
        let _arg1: i64 = $arg1 as i64;
        let _arg2: i64 = $arg2 as i64;
        let _arg3: i64 = $arg3 as i64;
        let mut _arg4: i64 = $arg4 as i64;

        unsafe {
            core::arch::asm!(
                "callsys",
                inout("$0") _num,
                inout("$19") _arg4,
                in("$16") _arg1,
                in("$17") _arg2,
                in("$18") _arg3,
                lateout("$1") _,
                lateout("$2") _,
                lateout("$3") _,
                lateout("$4") _,
                lateout("$5") _,
                lateout("$6") _,
                lateout("$7") _,
                lateout("$8") _,
                lateout("$20") _,
                lateout("$21") _,
                lateout("$22") _,
                lateout("$23") _,
                lateout("$24") _,
                lateout("$25") _,
                lateout("$27") _,
                lateout("$28") _,
            );
        }
        if _arg4 != 0 { -_num } else { _num }
    }};
}

macro_rules! __nolibc_syscall5 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {{
        let mut _num: i64 = $num as i64;
        let _arg1: i64 = $arg1 as i64;
        let _arg2: i64 = $arg2 as i64;
        let _arg3: i64 = $arg3 as i64;
        let mut _arg4: i64 = $arg4 as i64;
        let _arg5: i64 = $arg5 as i64;

        unsafe {
            core::arch::asm!(
                "callsys",
                inout("$0") _num,
                inout("$19") _arg4,
                in("$16") _arg1,
                in("$17") _arg2,
                in("$18") _arg3,
                in("$20") _arg5,
                lateout("$1") _,
                lateout("$2") _,
                lateout("$3") _,
                lateout("$4") _,
                lateout("$5") _,
                lateout("$6") _,
                lateout("$7") _,
                lateout("$8") _,
                lateout("$21") _,
                lateout("$22") _,
                lateout("$23") _,
                lateout("$24") _,
                lateout("$25") _,
                lateout("$27") _,
                lateout("$28") _,
            );
        }
        if _arg4 != 0 { -_num } else { _num }
    }};
}

macro_rules! __nolibc_syscall6 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr) => {{
        let mut _num: i64 = $num as i64;
        let _arg1: i64 = $arg1 as i64;
        let _arg2: i64 = $arg2 as i64;
        let _arg3: i64 = $arg3 as i64;
        let mut _arg4: i64 = $arg4 as i64;
        let _arg5: i64 = $arg5 as i64;
        let _arg6: i64 = $arg6 as i64;

        unsafe {
            core::arch::asm!(
                "callsys",
                inout("$0") _num,
                inout("$19") _arg4,
                in("$16") _arg1,
                in("$17") _arg2,
                in("$18") _arg3,
                in("$20") _arg5,
                in("$21") _arg6,
                lateout("$1") _,
                lateout("$2") _,
                lateout("$3") _,
                lateout("$4") _,
                lateout("$5") _,
                lateout("$6") _,
                lateout("$7") _,
                lateout("$8") _,
                lateout("$22") _,
                lateout("$23") _,
                lateout("$24") _,
                lateout("$25") _,
                lateout("$27") _,
                lateout("$28") _,
            );
        }
        if _arg4 != 0 { -_num } else { _num }
    }};
}

extern "C" {
    fn _start_c(argc: *mut core::ffi::c_void) -> !;
    fn __nolibc_entrypoint_epilogue() -> !;
}

/* startup code */
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    core::arch::asm!(
        "br $gp, 0f",        /* setup $gp, so that 'lda' works                */
        "0: ldgp $gp, 0($gp)",
        "lda $27, _start_c", /* setup current function address for _start_c   */
        "mov $sp, $16",      /* save argc pointer to $16, as arg1 of _start_c */
        "br  _start_c",      /* transfer to c runtime                         */
        options(noreturn),
    );
    __nolibc_entrypoint_epilogue();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
