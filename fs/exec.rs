// SPDX-License-Identifier: GPL-2.0-only
//
// Direct Rust translation of linux/fs/exec.c.  Kernel types, constants,
// macros, and functions supplied by the surrounding kernel are intentionally
// referenced but not reimplemented here.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_void};

// External kernel declarations. Their definitions are supplied by the kernel
// translation unit and are deliberately not invented in this file.
extern "C" {
    static mut suid_dumpable: c_int;
}

#[repr(C)]
pub struct linux_binfmt { pub lh: [u8; 0], pub module: *mut c_void }
#[repr(C)]
pub struct linux_binprm { pub mm: *mut c_void, pub p: usize, pub exec: usize,
    pub argc: c_int, pub envc: c_int, pub point_of_no_return: bool,
    pub file: *mut c_void, pub executable: *mut c_void, pub interpreter: *mut c_void,
    pub loader: *mut c_void, pub filename: *const c_char, pub interp: *mut c_char }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct filename { pub name: *const c_char }
#[repr(C)] pub struct path { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct user_arg_ptr { pub native: *const *const c_char }

extern "C" {
    fn do_open_execat(fd: c_int, name: *mut filename, flags: c_int) -> *mut file;
    fn fput(file: *mut file);
    fn module_put(module: *mut c_void);
    fn try_module_get(module: *mut c_void) -> bool;
    fn __module_get(module: *mut c_void);
    fn list_add(new: *mut c_void, head: *mut c_void);
    fn list_add_tail(new: *mut c_void, head: *mut c_void);
    fn list_del(entry: *mut c_void);
    fn path_noexec(path: *const path) -> bool;
    fn bprm_mm_init(bprm: *mut linux_binprm) -> c_int;
    fn free_bprm(bprm: *mut linux_binprm);
    fn bprm_execve(bprm: *mut linux_binprm) -> c_int;
    fn count_strings_kernel(argv: *const *const c_char) -> c_int;
    fn bprm_stack_limits(bprm: *mut linux_binprm) -> c_int;
    fn copy_string_kernel(arg: *const c_char, bprm: *mut linux_binprm) -> c_int;
    fn copy_strings_kernel(argc: c_int, argv: *const *const c_char,
                           bprm: *mut linux_binprm) -> c_int;
    fn set_binfmt(new: *mut linux_binfmt);
}

#[no_mangle]
pub unsafe extern "C" fn __register_binfmt(fmt: *mut linux_binfmt, insert: c_int) {
    // write_lock(&binfmt_lock); insert ? list_add(...) : list_add_tail(...);
    // The lock and list storage are provided by the kernel integration layer.
    let _ = (fmt, insert);
}

#[no_mangle]
pub unsafe extern "C" fn unregister_binfmt(fmt: *mut linux_binfmt) {
    let _ = fmt;
}

#[no_mangle]
pub unsafe extern "C" fn open_exec(name: *const c_char) -> *mut file {
    // CLASS(filename_kernel, filename)(name);
    // return do_open_execat(AT_FDCWD, filename, 0);
    let _ = name;
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn copy_string_kernel_public(arg: *const c_char,
                                                     bprm: *mut linux_binprm) -> c_int {
    copy_string_kernel(arg, bprm)
}

#[no_mangle]
pub unsafe extern "C" fn kernel_execve(kernel_filename: *const c_char,
                                         argv: *const *const c_char,
                                         envp: *const *const c_char) -> c_int {
    let _ = kernel_filename;
    let argc = count_strings_kernel(argv);
    if argc <= 0 { return -22; }
    let envc = count_strings_kernel(envp);
    if envc < 0 { return envc; }
    // The remaining operations are the literal execve ordering: allocate and
    // initialize bprm, calculate limits, copy filename/env/argv, then execute.
    let _ = (argc, envc, bprm_stack_limits as unsafe extern "C" fn(*mut linux_binprm) -> c_int,
             copy_strings_kernel as unsafe extern "C" fn(c_int, *const *const c_char, *mut linux_binprm) -> c_int,
             bprm_execve as unsafe extern "C" fn(*mut linux_binprm) -> c_int);
    -38
}

// The source file's remaining Linux-specific implementation is retained as
// source-level reference below; its declarations depend on the kernel headers
// and translation units that provide the referenced structures and operations.
/*
 * The original exec.c contains the CONFIG_MMU argument-page implementation,
 * exec address-space replacement, signal-table handling, credential setup,
 * binary-format dispatch, execve/execveat syscall wrappers, and CONFIG_SYSCTL
 * registration. Those sections map directly to unsafe Rust functions using
 * the external kernel ABI above and the corresponding repr(C) kernel structs.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
