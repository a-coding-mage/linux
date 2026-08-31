/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2019  Arm Limited
 * Original author: Dave Martin <Dave.Martin@arm.com>
 */

/* C header guard SYSTEM_H omitted in Rust. */

/* Dependencies from C includes:
 * <linux/types.h>, <linux/stddef.h>, <linux/errno.h>,
 * <linux/compiler.h>, <asm/hwcap.h>, <asm/ptrace.h>, <asm/unistd.h>
 */

pub type size_t = __kernel_size_t;
pub type ssize_t = __kernel_ssize_t;

unsafe extern "C" {
    pub fn syscall(nr: core::ffi::c_int, ...) -> core::ffi::c_long;

    pub fn exit(n: core::ffi::c_int) -> !;
    pub fn write(fd: core::ffi::c_int, buf: *const core::ffi::c_void, size: size_t) -> ssize_t;
}
