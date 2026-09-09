/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2023 ARM Ltd.
 */

// Dependencies supplied by the surrounding kernel translation:
// asm/types.h, asm/uaccess.h

use core::arch::asm;

// struct kernel_clone_args;
// struct ksignal;

pub unsafe fn gcsb_dsync() {
    asm!(".inst 0xd503227f", options(nostack, preserves_flags));
}

pub unsafe fn gcsstr(addr: *mut u64, val: u64) {
    // GCSSTTR x1, [x0]
    asm!(
        ".inst 0xd91f1c01",
        in("x0") addr,
        in("x1") val,
        options(nostack, preserves_flags)
    );
}

pub unsafe fn gcsss1(xt: u64) {
    asm!(
        "sys #3, C7, C7, #2, {0}",
        in(reg) xt,
        options(nostack, preserves_flags)
    );
}

pub unsafe fn gcsss2() -> u64 {
    let xt: u64;
    asm!(
        "SYSL {0}, #3, C7, C7, #3",
        out(reg) xt,
        options(nostack, preserves_flags)
    );
    xt
}

pub const PR_SHADOW_STACK_SUPPORTED_STATUS_MASK: _ =
    PR_SHADOW_STACK_ENABLE | PR_SHADOW_STACK_WRITE | PR_SHADOW_STACK_PUSH;

#[cfg(CONFIG_ARM64_GCS)]
pub unsafe fn task_gcs_el0_enabled(task: *const task_struct) -> bool {
    ((*task).thread.gcs_el0_mode & PR_SHADOW_STACK_ENABLE) != 0
}

#[cfg(CONFIG_ARM64_GCS)]
extern "C" {
    pub fn gcs_set_el0_mode(task: *mut task_struct);
    pub fn gcs_free(task: *mut task_struct);
    pub fn gcs_preserve_current_state();
    pub fn gcs_alloc_thread_stack(
        tsk: *mut task_struct,
        args: *const kernel_clone_args,
    ) -> c_ulong;
}

#[cfg(CONFIG_ARM64_GCS)]
pub unsafe fn gcs_check_locked(task: *mut task_struct, mut new_val: c_ulong) -> c_int {
    let mut cur_val = (*task).thread.gcs_el0_mode;
    cur_val &= (*task).thread.gcs_el0_locked;
    new_val &= (*task).thread.gcs_el0_locked;
    if cur_val != new_val { -EBUSY } else { 0 }
}

#[cfg(CONFIG_ARM64_GCS)]
pub unsafe fn gcssttr(addr: *mut c_ulong, val: c_ulong) -> c_int {
    let mut err: c_int = 0;
    // GCSSTTR x1, [x0]
    asm!(
        "1: .inst 0xd91f1c01",
        "2:",
        // _ASM_EXTABLE_UACCESS_ERR(1b, 2b, %w0)
        in("x0") addr,
        in("x1") val,
        inout(reg) err,
        options(nostack, preserves_flags)
    );
    err
}

#[cfg(CONFIG_ARM64_GCS)]
pub unsafe fn put_user_gcs(val: c_ulong, addr: *mut c_ulong, err: *mut c_int) {
    if !access_ok(addr as *const u8, core::mem::size_of::<u64>()) {
        *err = -EFAULT;
        return;
    }
    uaccess_ttbr0_enable();
    let ret = gcssttr(addr, val);
    if ret != 0 { *err = ret; }
    uaccess_ttbr0_disable();
}

#[cfg(CONFIG_ARM64_GCS)]
pub unsafe fn push_user_gcs(val: c_ulong, err: *mut c_int) {
    let mut gcspr = read_sysreg_s(SYS_GCSPR_EL0);
    gcspr = gcspr.wrapping_sub(core::mem::size_of::<u64>() as u64);
    put_user_gcs(val, gcspr as *mut c_ulong, err);
    if *err == 0 { write_sysreg_s(gcspr, SYS_GCSPR_EL0); }
}

/*
 * Unlike put/push_user_gcs() above, get/pop_user_gsc() doesn't
 * validate the GCS permission is set on the page being read.  This
 * differs from how the hardware works when it consumes data stored at
 * GCSPR. Callers should ensure this is acceptable.
 */
#[cfg(CONFIG_ARM64_GCS)]
pub unsafe fn get_user_gcs(addr: *mut c_ulong, err: *mut c_int) -> u64 {
    let mut load = 0u64;
    // Ensure previous GCS operation are visible before we read the page
    gcsb_dsync();
    let ret = copy_from_user(&mut load as *mut u64, addr as *const _, core::mem::size_of::<u64>());
    if ret != 0 { *err = ret as c_int; }
    load
}

#[cfg(CONFIG_ARM64_GCS)]
pub unsafe fn pop_user_gcs(err: *mut c_int) -> u64 {
    let gcspr = read_sysreg_s(SYS_GCSPR_EL0);
    let read_val = get_user_gcs(gcspr as *mut c_ulong, err);
    if *err == 0 { write_sysreg_s(gcspr.wrapping_add(core::mem::size_of::<u64>() as u64), SYS_GCSPR_EL0); }
    read_val
}

#[cfg(not(CONFIG_ARM64_GCS))]
pub unsafe fn task_gcs_el0_enabled(_task: *const task_struct) -> bool { false }
#[cfg(not(CONFIG_ARM64_GCS))]
pub unsafe fn gcs_set_el0_mode(_task: *mut task_struct) {}
#[cfg(not(CONFIG_ARM64_GCS))]
pub unsafe fn gcs_free(_task: *mut task_struct) {}
#[cfg(not(CONFIG_ARM64_GCS))]
pub unsafe fn gcs_preserve_current_state() {}
#[cfg(not(CONFIG_ARM64_GCS))]
pub unsafe fn put_user_gcs(_val: c_ulong, _addr: *mut c_ulong, _err: *mut c_int) {}
#[cfg(not(CONFIG_ARM64_GCS))]
pub unsafe fn push_user_gcs(_val: c_ulong, _err: *mut c_int) {}
#[cfg(not(CONFIG_ARM64_GCS))]
pub unsafe fn gcs_alloc_thread_stack(_tsk: *mut task_struct, _args: *const kernel_clone_args) -> c_ulong { -ENOTSUPP as c_ulong }
#[cfg(not(CONFIG_ARM64_GCS))]
pub unsafe fn gcs_check_locked(_task: *mut task_struct, _new_val: c_ulong) -> c_int { 0 }
#[cfg(not(CONFIG_ARM64_GCS))]
pub unsafe fn get_user_gcs(_addr: *mut c_ulong, err: *mut c_int) -> u64 { *err = -EFAULT; 0 }
#[cfg(not(CONFIG_ARM64_GCS))]
pub unsafe fn pop_user_gcs(_err: *mut c_int) -> u64 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
