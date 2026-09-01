/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * Reboot definitions for NOLIBC
 * Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
 */

/* make sure to include all global symbols */
/* Rust translation note: original C included "../nolibc.h", "../sys.h",
 * and <linux/reboot.h> for the external symbols referenced below.
 */

/*
 * int reboot(int cmd);
 * <cmd> is among LINUX_REBOOT_CMD_*
 */

unsafe extern "C" {
    static __NR_reboot: core::ffi::c_long;
    static LINUX_REBOOT_MAGIC1: core::ffi::c_int;
    static LINUX_REBOOT_MAGIC2: core::ffi::c_int;

    fn __nolibc_syscall4(
        nr: core::ffi::c_long,
        arg1: core::ffi::c_int,
        arg2: core::ffi::c_int,
        arg3: core::ffi::c_int,
        arg4: *mut core::ffi::c_void,
    ) -> isize;
    fn __sysret(ret: isize) -> core::ffi::c_int;
}

#[allow(dead_code)]
pub unsafe fn _sys_reboot(
    magic1: core::ffi::c_int,
    magic2: core::ffi::c_int,
    cmd: core::ffi::c_int,
    arg: *mut core::ffi::c_void,
) -> isize {
    unsafe { __nolibc_syscall4(__NR_reboot, magic1, magic2, cmd, arg) }
}

#[allow(dead_code)]
pub unsafe fn reboot(cmd: core::ffi::c_int) -> core::ffi::c_int {
    unsafe {
        __sysret(_sys_reboot(
            LINUX_REBOOT_MAGIC1,
            LINUX_REBOOT_MAGIC2,
            cmd,
            core::ptr::null_mut(),
        ))
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
