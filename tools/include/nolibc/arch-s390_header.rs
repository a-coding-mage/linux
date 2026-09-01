/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * s390 specific definitions for NOLIBC
 */

/*
 * C header dependencies removed from executable Rust:
 * "types.h", <linux/sched.h>, <linux/signal.h>, <linux/unistd.h>,
 * "compiler.h", "crt.h", and "std.h".
 */

use core::arch::asm;

/* Syscalls for s390:
 *   - registers are 64-bit
 *   - syscall number is passed in r1
 *   - arguments are in r2-r7
 *   - the system call is performed by calling the svc instruction
 *   - syscall return value is in r2
 *   - r1 and r2 are clobbered, others are preserved.
 *
 * Link s390 ABI: https://github.com/IBM/s390x-abi
 *
 */

pub unsafe fn __nolibc_syscall0(num: isize) -> isize {
    let _num: isize = num;
    let _rc: isize;

    unsafe {
        asm!(
            "svc 0",
            lateout("r2") _rc,
            in("r1") _num,
            options(nostack)
        );
    }

    _rc
}

pub unsafe fn __nolibc_syscall1(num: isize, arg1: isize) -> isize {
    let _num: isize = num;
    let mut _arg1: isize = arg1 as isize;

    unsafe {
        asm!(
            "svc 0",
            inlateout("r2") _arg1,
            in("r1") _num,
            options(nostack)
        );
    }

    _arg1
}

pub unsafe fn __nolibc_syscall2(num: isize, arg1: isize, arg2: isize) -> isize {
    let _num: isize = num;
    let mut _arg1: isize = arg1 as isize;
    let _arg2: isize = arg2 as isize;

    unsafe {
        asm!(
            "svc 0",
            inlateout("r2") _arg1,
            in("r3") _arg2,
            in("r1") _num,
            options(nostack)
        );
    }

    _arg1
}

pub unsafe fn __nolibc_syscall3(num: isize, arg1: isize, arg2: isize, arg3: isize) -> isize {
    let _num: isize = num;
    let mut _arg1: isize = arg1 as isize;
    let _arg2: isize = arg2 as isize;
    let _arg3: isize = arg3 as isize;

    unsafe {
        asm!(
            "svc 0",
            inlateout("r2") _arg1,
            in("r3") _arg2,
            in("r4") _arg3,
            in("r1") _num,
            options(nostack)
        );
    }

    _arg1
}

pub unsafe fn __nolibc_syscall4(
    num: isize,
    arg1: isize,
    arg2: isize,
    arg3: isize,
    arg4: isize,
) -> isize {
    let _num: isize = num;
    let mut _arg1: isize = arg1 as isize;
    let _arg2: isize = arg2 as isize;
    let _arg3: isize = arg3 as isize;
    let _arg4: isize = arg4 as isize;

    unsafe {
        asm!(
            "svc 0",
            inlateout("r2") _arg1,
            in("r3") _arg2,
            in("r4") _arg3,
            in("r5") _arg4,
            in("r1") _num,
            options(nostack)
        );
    }

    _arg1
}

pub unsafe fn __nolibc_syscall5(
    num: isize,
    arg1: isize,
    arg2: isize,
    arg3: isize,
    arg4: isize,
    arg5: isize,
) -> isize {
    let _num: isize = num;
    let mut _arg1: isize = arg1 as isize;
    let _arg2: isize = arg2 as isize;
    let _arg3: isize = arg3 as isize;
    let _arg4: isize = arg4 as isize;
    let _arg5: isize = arg5 as isize;

    unsafe {
        asm!(
            "svc 0",
            inlateout("r2") _arg1,
            in("r3") _arg2,
            in("r4") _arg3,
            in("r5") _arg4,
            in("r6") _arg5,
            in("r1") _num,
            options(nostack)
        );
    }

    _arg1
}

pub unsafe fn __nolibc_syscall6(
    num: isize,
    arg1: isize,
    arg2: isize,
    arg3: isize,
    arg4: isize,
    arg5: isize,
    arg6: isize,
) -> isize {
    let _num: isize = num;
    let mut _arg1: isize = arg1 as isize;
    let _arg2: isize = arg2 as isize;
    let _arg3: isize = arg3 as isize;
    let _arg4: isize = arg4 as isize;
    let _arg5: isize = arg5 as isize;
    let _arg6: isize = arg6 as isize;

    unsafe {
        asm!(
            "svc 0",
            inlateout("r2") _arg1,
            in("r3") _arg2,
            in("r4") _arg3,
            in("r5") _arg4,
            in("r6") _arg5,
            in("r7") _arg6,
            in("r1") _num,
            options(nostack)
        );
    }

    _arg1
}

/* C conditional preserved: #ifndef NOLIBC_NO_RUNTIME */
/* startup code */
#[cfg(not(NOLIBC_NO_RUNTIME))]
extern "C" {
    fn _start_c(stack: *mut core::ffi::c_void) -> !;
    fn __nolibc_entrypoint_epilogue() -> !;
}

#[cfg(not(NOLIBC_NO_RUNTIME))]
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    unsafe {
        asm!(
            "lgr %r2, %r15",
            "aghi %r15, -160",
            "xc 0(8,%r15), 0(%r15)",
            "brasl %r14, _start_c",
            options(noreturn)
        );
    }

    unsafe {
        __nolibc_entrypoint_epilogue();
    }
}

#[repr(C)]
pub struct s390_mmap_arg_struct {
    pub addr: usize,
    pub len: usize,
    pub prot: usize,
    pub flags: usize,
    pub fd: usize,
    pub offset: usize,
}

pub unsafe fn _sys_mmap(
    addr: *mut core::ffi::c_void,
    length: size_t,
    prot: core::ffi::c_int,
    flags: core::ffi::c_int,
    fd: core::ffi::c_int,
    offset: off_t,
) -> *mut core::ffi::c_void {
    let mut args = s390_mmap_arg_struct {
        addr: addr as usize,
        len: length as usize,
        prot: prot as usize,
        flags: flags as usize,
        fd: fd as usize,
        offset: offset as usize,
    };

    unsafe { __nolibc_syscall1(__NR_mmap as isize, &mut args as *mut _ as isize) as *mut core::ffi::c_void }
}
/* #define _sys_mmap _sys_mmap */

pub unsafe fn _sys_fork() -> pid_t {
    unsafe { __nolibc_syscall5(__NR_clone as isize, 0, SIGCHLD as isize, 0, 0, 0) as pid_t }
}
/* #define _sys_fork _sys_fork */

pub unsafe fn _sys_vfork() -> pid_t {
    unsafe {
        __nolibc_syscall5(
            __NR_clone as isize,
            0,
            (CLONE_VM | CLONE_VFORK | SIGCHLD) as isize,
            0,
            0,
            0,
        ) as pid_t
    }
}
/* #define _sys_vfork _sys_vfork */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
