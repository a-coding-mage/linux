// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/init/version.c
 *
 *  Copyright (C) 1992  Theodore Ts'o
 *
 *  May be freely distributed as part of Linux.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// generated/compile.h, linux/build-salt.h, linux/elfnote-lto.h,
// linux/export.h, linux/init.h, linux/printk.h, linux/uts.h,
// linux/utsname.h, and linux/proc_ns.h.

extern "C" {
    static mut init_uts_ns: uts_namespace;
    fn strscpy(dst: *mut core::ffi::c_char, src: *const core::ffi::c_char, count: usize) -> isize;
    fn pr_warn(format: *const core::ffi::c_char, ...);
}

// The following type and constants are supplied by the corresponding kernel
// dependencies.
#[allow(non_camel_case_types)]
type uts_namespace = crate::uts_namespace;

unsafe fn early_hostname(arg: *mut core::ffi::c_char) -> core::ffi::c_int {
    let bufsize: usize = core::mem::size_of_val(&(*core::ptr::addr_of!(init_uts_ns)).name.nodename);
    let maxlen: usize = bufsize - 1;
    let arglen: isize;

    arglen = strscpy(
        (*core::ptr::addr_of_mut!(init_uts_ns)).name.nodename.as_mut_ptr(),
        arg,
        bufsize,
    );
    if arglen < 0 {
        pr_warn(
            b"hostname parameter exceeds %zd characters and will be truncated\0".as_ptr() as *const core::ffi::c_char,
            maxlen,
        );
    }
    0
}

// Equivalent to early_param("hostname", early_hostname).

pub static linux_proc_banner: &[u8] = concat!(
    "%s version %s",
    " (", LINUX_COMPILE_BY, "@", LINUX_COMPILE_HOST, ")",
    " (", LINUX_COMPILER, ") %s\n",
).as_bytes();

// BUILD_SALT;
// BUILD_LTO_INFO;

/*
 * init_uts_ns and linux_banner contain the build version and timestamp,
 * which are really fixed at the very last step of build process.
 * They are compiled with __weak first, and without __weak later.
 */

#[no_mangle]
pub static mut init_uts_ns_definition: uts_namespace = unsafe { core::mem::zeroed() };
#[no_mangle]
pub static linux_banner: &[u8] = b"";

// version-timestamp.c is translated and supplied separately.

// Equivalent to EXPORT_SYMBOL_GPL(init_uts_ns).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
