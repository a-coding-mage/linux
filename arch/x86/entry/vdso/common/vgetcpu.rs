// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2006 Andi Kleen, SUSE Labs.
 *
 * Fast user context implementation of getcpu()
 */

// Dependencies supplied by the surrounding kernel/vDSO build.

unsafe extern "C" {
    fn vdso_read_cpunode(cpu: *mut u32, node: *mut u32);
}

#[no_mangle]
pub unsafe extern "C" fn __vdso_getcpu(
    cpu: *mut u32,
    node: *mut u32,
    _unused: *mut core::ffi::c_void,
) -> isize {
    unsafe {
        vdso_read_cpunode(cpu, node);
    }

    0
}

// C declaration uses __attribute__((weak, alias("__vdso_getcpu"))).
#[no_mangle]
pub unsafe extern "C" fn getcpu(
    cpu: *mut u32,
    node: *mut u32,
    tcache: *mut core::ffi::c_void,
) -> isize {
    unsafe { __vdso_getcpu(cpu, node, tcache) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
