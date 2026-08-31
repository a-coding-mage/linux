// SPDX-License-Identifier: GPL-2.0

// Translated from C source using external BPF/kernel definitions from:
// <vmlinux.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>, and "bpf_misc.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::arch::asm;

extern "C" {
    static __clobber_all: ();
    fn __sink(arg: *mut inode);
}

#[repr(C)]
pub struct file {
    pub f_inode: *mut inode,
}

#[repr(C)]
pub struct inode {
    pub _bindgen_opaque_blob: [u8; 0],
}

// SEC("lsm/file_permission")
// __description("lsm bpf prog with -4095~0 retval. test 1")
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn errno_zero_retval_test1(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    unsafe {
        asm!("r0 = 0;", "exit;", options(noreturn));
    }
}

// SEC("lsm/file_permission")
// __description("lsm bpf prog with -4095~0 retval. test 2")
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn errno_zero_retval_test2(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    unsafe {
        asm!("r0 = -4095;", "exit;", options(noreturn));
    }
}

// SEC("lsm/file_mprotect")
// __description("lsm bpf prog with -4095~0 retval. test 4")
// __failure __msg("R0 has smin=-4096 smax=-4096 should have been in [-4095, 0]")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn errno_zero_retval_test4(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    unsafe {
        asm!("r0 = -4096;", "exit;", options(noreturn));
    }
}

// SEC("lsm/file_mprotect")
// __description("lsm bpf prog with -4095~0 retval. test 5")
// __failure __msg("R0 has smin=4096 smax=4096 should have been in [-4095, 0]")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn errno_zero_retval_test5(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    unsafe {
        asm!("r0 = 4096;", "exit;", options(noreturn));
    }
}

// SEC("lsm/file_mprotect")
// __description("lsm bpf prog with -4095~0 retval. test 6")
// __failure __msg("R0 has smin=1 smax=1 should have been in [-4095, 0]")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn errno_zero_retval_test6(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    unsafe {
        asm!("r0 = 1;", "exit;", options(noreturn));
    }
}

// SEC("lsm/audit_rule_known")
// __description("lsm bpf prog with bool retval. test 1")
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bool_retval_test1(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    unsafe {
        asm!("r0 = 1;", "exit;", options(noreturn));
    }
}

// SEC("lsm/audit_rule_known")
// __description("lsm bpf prog with bool retval. test 2")
// __success
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bool_retval_test2(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    unsafe {
        asm!("r0 = 0;", "exit;", options(noreturn));
    }
}

// SEC("lsm/audit_rule_known")
// __description("lsm bpf prog with bool retval. test 3")
// __failure __msg("R0 has smin=-1 smax=-1 should have been in [0, 1]")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bool_retval_test3(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    unsafe {
        asm!("r0 = -1;", "exit;", options(noreturn));
    }
}

// SEC("lsm/audit_rule_known")
// __description("lsm bpf prog with bool retval. test 4")
// __failure __msg("R0 has smin=2 smax=2 should have been in [0, 1]")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bool_retval_test4(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    unsafe {
        asm!("r0 = 2;", "exit;", options(noreturn));
    }
}

// SEC("lsm/file_free_security")
// __success
// __description("lsm bpf prog with void retval. test 1")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn void_retval_test1(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    unsafe {
        asm!("r0 = -4096;", "exit;", options(noreturn));
    }
}

// SEC("lsm/file_free_security")
// __success
// __description("lsm bpf prog with void retval. test 2")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn void_retval_test2(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    unsafe {
        asm!("r0 = 4096;", "exit;", options(noreturn));
    }
}

// SEC("lsm/getprocattr")
// __description("lsm disabled hook: getprocattr")
// __failure __msg("points to disabled hook")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn disabled_hook_test1(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    unsafe {
        asm!("r0 = 0;", "exit;", options(noreturn));
    }
}

// SEC("lsm/setprocattr")
// __description("lsm disabled hook: setprocattr")
// __failure __msg("points to disabled hook")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn disabled_hook_test2(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    unsafe {
        asm!("r0 = 0;", "exit;", options(noreturn));
    }
}

// SEC("lsm/ismaclabel")
// __description("lsm disabled hook: ismaclabel")
// __failure __msg("points to disabled hook")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn disabled_hook_test3(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    unsafe {
        asm!("r0 = 0;", "exit;", options(noreturn));
    }
}

// SEC("lsm/mmap_file")
// __description("not null checking nullable pointer in bpf_lsm_mmap_file")
// __failure __msg("R1 invalid mem access 'trusted_ptr_or_null_'")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn no_null_check(file: *mut file) -> i32 {
    let inode: *mut inode;

    inode = unsafe { (*(file)).f_inode };
    unsafe {
        __sink(inode);
    }

    0
}

// SEC("lsm/mmap_file")
// __description("null checking nullable pointer in bpf_lsm_mmap_file")
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn null_check(file: *mut file) -> i32 {
    let inode: *mut inode;

    if !file.is_null() {
        inode = unsafe { (*(file)).f_inode };
        unsafe {
            __sink(inode);
        }
    }

    0
}

// SEC("lsm_cgroup/file_open")
// __description("sleepable lsm_cgroup program is rejected")
// __failure __msg("Program of this type cannot be sleepable")
// __flag(BPF_F_SLEEPABLE)
#[unsafe(no_mangle)]
pub extern "C" fn sleepable_lsm_cgroup() -> i32 {
    0
}

// SEC("lsm/file_mprotect")
// __description("lsm retval load must reset stale register bounds")
// __failure __msg("div by zero")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn retval_load_resets_bounds(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    unsafe {
        asm!(
            "r6 = 0;",
            "r6 = *(u64 *)(r1 + 24);",
            "if r6 == 0 goto +1;",
            "r6 /= 0;",
            "r0 = 0;",
            "exit;",
            options(noreturn),
        );
    }
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";
