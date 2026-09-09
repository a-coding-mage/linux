// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2004 PathScale, Inc
 * Copyright (C) 2004 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// C dependencies: errno, string, ptrace, ptrace_user, registers, and malloc.

extern "C" {
    fn ptrace(request: ::std::os::raw::c_int, pid: ::std::os::raw::c_int, ...) -> ::std::os::raw::c_long;
    fn __errno_location() -> *mut ::std::os::raw::c_int;
    fn arch_init_registers(pid: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    fn get_fp_registers(pid: ::std::os::raw::c_int, fp_regs: *mut ::std::os::raw::c_ulong);
    fn malloc(size: usize) -> *mut ::std::os::raw::c_void;
    fn memcpy(
        destination: *mut ::std::os::raw::c_void,
        source: *const ::std::os::raw::c_void,
        size: usize,
    ) -> *mut ::std::os::raw::c_void;
}

// Supplied by the architecture-specific headers/build configuration.
const PTRACE_GETREGS: ::std::os::raw::c_int = 12;
const MAX_REG_NR: usize = 0;
extern "C" {
    static host_fp_size: usize;
}

/* This is set once at boot time and not changed thereafter */

#[no_mangle]
pub static mut exec_regs: [::std::os::raw::c_ulong; MAX_REG_NR] = [0; MAX_REG_NR];

#[no_mangle]
pub static mut exec_fp_regs: *mut ::std::os::raw::c_ulong = ::std::ptr::null_mut();

#[no_mangle]
pub unsafe extern "C" fn init_pid_registers(pid: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
    let mut err: ::std::os::raw::c_int;

    err = ptrace(PTRACE_GETREGS, pid, 0, exec_regs.as_mut_ptr()) as ::std::os::raw::c_int;
    if err < 0 {
        return -(*__errno_location());
    }

    err = arch_init_registers(pid);
    if err < 0 {
        return err;
    }

    exec_fp_regs = malloc(host_fp_size) as *mut ::std::os::raw::c_ulong;
    get_fp_registers(pid, exec_fp_regs);
    0
}

#[no_mangle]
pub unsafe extern "C" fn get_safe_registers(
    regs: *mut ::std::os::raw::c_ulong,
    fp_regs: *mut ::std::os::raw::c_ulong,
) {
    memcpy(
        regs as *mut ::std::os::raw::c_void,
        exec_regs.as_ptr() as *const ::std::os::raw::c_void,
        ::std::mem::size_of_val(&exec_regs),
    );

    if !fp_regs.is_null() {
        memcpy(
            fp_regs as *mut ::std::os::raw::c_void,
            exec_fp_regs as *const ::std::os::raw::c_void,
            host_fp_size,
        );
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
