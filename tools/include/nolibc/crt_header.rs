/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * C Run Time support for NOLIBC
 * Copyright (C) 2023 Zhangjin Wu <falcon@tinylab.org>
 */

// Header guard _NOLIBC_CRT_H omitted in Rust.

// C macro:
// #define __nolibc_arg_to_reg(_a) \
//	__builtin_choose_expr(__builtin_classify_type(_a) == __builtin_classify_type(NULL), \
//			      (unsigned long)(_a), (_a))
// This depends on C builtin type classification and is intentionally left as
// macro intent for architecture-specific call sites.

// The following runtime support is present unless NOLIBC_NO_RUNTIME is defined.
// include "compiler.h" supplies __nolibc_no_sanitize_undefined and
// __nolibc_no_stack_protector in C.

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_ulong};

// C defines these as weak globals. Stable Rust has no direct file-local weak
// definition equivalent, so preserve the externally visible names and intent.
#[no_mangle]
pub static mut environ: *mut *mut c_char = core::ptr::null_mut();

#[no_mangle]
pub static mut _auxv: *const c_ulong = core::ptr::null();

unsafe extern "C" {
    pub fn _start();
    fn __stack_chk_init();
    fn exit(status: c_int) -> !;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;

    // Weak constructor/destructor array boundaries in C.
    static __preinit_array_start: [Option<unsafe extern "C" fn(c_int, *mut *mut c_char, *mut *mut c_char)>; 0];
    static __preinit_array_end: [Option<unsafe extern "C" fn(c_int, *mut *mut c_char, *mut *mut c_char)>; 0];

    static __init_array_start: [Option<unsafe extern "C" fn(c_int, *mut *mut c_char, *mut *mut c_char)>; 0];
    static __init_array_end: [Option<unsafe extern "C" fn(c_int, *mut *mut c_char, *mut *mut c_char)>; 0];

    static __fini_array_start: [Option<unsafe extern "C" fn()>; 0];
    static __fini_array_end: [Option<unsafe extern "C" fn()>; 0];

    // C declares this local prototype as:
    // int _nolibc_main(int, char **, char **) __asm__ ("main");
    #[link_name = "main"]
    fn _nolibc_main(argc: c_int, argv: *mut *mut c_char, envp: *mut *mut c_char) -> c_int;

    // Present unless NOLIBC_IGNORE_ERRNO is defined. C declares weak globals.
    static mut program_invocation_name: *mut c_char;
    static mut program_invocation_short_name: *mut c_char;
}

// Present unless NOLIBC_IGNORE_ERRNO is defined.
#[inline]
unsafe fn __nolibc_program_invocation_short_name(long_name: *mut c_char) -> *mut c_char {
    let short_name: *mut c_char;

    short_name = unsafe { strrchr(long_name, b'/' as c_int) };
    if short_name.is_null() || unsafe { *short_name } == 0 {
        return long_name;
    }

    unsafe { short_name.add(1) }
}

#[no_mangle]
pub unsafe extern "C" fn _start_c(sp: *mut c_long) {
    let argc: c_long;
    let argv: *mut *mut c_char;
    let envp: *mut *mut c_char;
    let exitcode: c_int;
    let mut ctor_func: *const Option<unsafe extern "C" fn(c_int, *mut *mut c_char, *mut *mut c_char)>;
    let mut dtor_func: *const Option<unsafe extern "C" fn()>;
    let mut auxv: *const c_ulong;

    /* initialize stack protector */
    unsafe { __stack_chk_init() };

    /*
     * sp  :    argc          <-- argument count, required by main()
     * argv:    argv[0]       <-- argument vector, required by main()
     *          argv[1]
     *          ...
     *          argv[argc-1]
     *          null
     * environ: environ[0]    <-- environment variables, required by main() and getenv()
     *          environ[1]
     *          ...
     *          null
     * _auxv:   _auxv[0]      <-- auxiliary vector, required by getauxval()
     *          _auxv[1]
     *          ...
     *          null
     */

    /* assign argc and argv */
    argc = unsafe { *sp };
    argv = unsafe { sp.add(1) as *mut *mut c_char };

    /* find environ */
    envp = unsafe { argv.add(argc as usize + 1) };
    unsafe {
        environ = envp;
    }

    /* find _auxv */
    auxv = envp as *const c_ulong;
    while unsafe {
        let v = *auxv;
        auxv = auxv.add(1);
        v != 0
    } {
        unsafe { asm!("") };
    }
    unsafe {
        _auxv = auxv;
    }

    // Present unless NOLIBC_IGNORE_ERRNO is defined.
    if argc > 0 && unsafe { !(*argv).is_null() } {
        unsafe {
            program_invocation_name = *argv;
            program_invocation_short_name = __nolibc_program_invocation_short_name(*argv);
        }
    }

    ctor_func = unsafe { __preinit_array_start.as_ptr() };
    while ctor_func < unsafe { __preinit_array_end.as_ptr() } {
        if let Some(func) = unsafe { *ctor_func } {
            unsafe { func(argc as c_int, argv, envp) };
        }
        ctor_func = unsafe { ctor_func.add(1) };
    }

    ctor_func = unsafe { __init_array_start.as_ptr() };
    while ctor_func < unsafe { __init_array_end.as_ptr() } {
        if let Some(func) = unsafe { *ctor_func } {
            unsafe { func(argc as c_int, argv, envp) };
        }
        ctor_func = unsafe { ctor_func.add(1) };
    }

    /* go to application */
    exitcode = unsafe { _nolibc_main(argc as c_int, argv, envp) };

    dtor_func = unsafe { __fini_array_end.as_ptr() };
    while dtor_func > unsafe { __fini_array_start.as_ptr() } {
        dtor_func = unsafe { dtor_func.sub(1) };
        if let Some(func) = unsafe { *dtor_func } {
            unsafe { func() };
        }
    }

    unsafe { exit(exitcode) };
}


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
