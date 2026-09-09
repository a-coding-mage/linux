// SPDX-License-Identifier: GPL-2.0-only
/*
 * Hibernation support specific for ARM
 *
 * Derived from work on ARM hibernation support by:
 *
 * Ubuntu project, hibernation support for mach-dove
 * Copyright (C) 2010 Nokia Corporation (Hiroshi Doyu)
 * Copyright (C) 2010 Texas Instruments, Inc. (Teerth Reddy et al.)
 *  https://lkml.org/lkml/2010/6/18/4
 *  https://lists.linux-foundation.org/pipermail/linux-pm/2010-June/027422.html
 *  https://patchwork.kernel.org/patch/96442/
 *
 * Copyright (C) 2006 Rafael J. Wysocki <rjw@sisk.pl>
 */

// Dependencies supplied by the surrounding kernel translation unit.
extern "C" {
    static mut __nosave_begin: u8;
    static mut __nosave_end: u8;
    static mut cpu_resume: u8;
    static mut idmap_pgd: u8;
    static mut init_mm: u8;
    static mut restore_pblist: *mut Pbe;

    fn virt_to_pfn(address: *const u8) -> c_ulong;
    fn num_online_cpus() -> c_int;
    fn local_fiq_disable();
    fn local_fiq_enable();
    fn WARN_ON(condition: bool) -> bool;
    fn swsusp_save() -> c_int;
    fn virt_to_idmap(address: *mut u8) -> *mut u8;
    fn _soft_restart(address: *mut u8, is_hyp: bool) -> !;
    fn cpu_suspend(arg: c_ulong, fn_ptr: unsafe extern "C" fn(c_ulong) -> c_int) -> c_int;
    fn uaccess_save_and_enable();
    fn cpu_switch_mm(pgd: *mut u8, mm: *mut u8);
    fn copy_page(to: *mut u8, from: *mut u8);
    fn call_with_stack(
        function: unsafe extern "C" fn(*mut u8),
        arg: *mut u8,
        stack: *mut u64,
    );
}

use core::ffi::{c_int, c_ulong};

#[repr(C)]
pub struct Pbe {
    pub address: *mut u8,
    pub orig_address: *mut u8,
    pub next: *mut Pbe,
}

#[inline]
pub unsafe fn pfn_is_nosave(pfn: c_ulong) -> bool {
    let nosave_begin_pfn = virt_to_pfn(&raw mut __nosave_begin);
    let nosave_end_pfn = virt_to_pfn((&raw mut __nosave_end).offset(-1));

    (pfn >= nosave_begin_pfn) && (pfn <= nosave_end_pfn)
}

pub unsafe extern "C" fn save_processor_state() {
    WARN_ON(num_online_cpus() != 1);
    local_fiq_disable();
}

pub unsafe extern "C" fn restore_processor_state() {
    local_fiq_enable();
}

/*
 * Snapshot kernel memory and reset the system.
 *
 * swsusp_save() is executed in the suspend finisher so that the CPU
 * context pointer and memory are part of the saved image, which is
 * required by the resume kernel image to restart execution from
 * swsusp_arch_suspend().
 *
 * soft_restart is not technically needed, but is used to get success
 * returned from cpu_suspend.
 *
 * When soft reboot completes, the hibernation snapshot is written out.
 */
unsafe extern "C" fn arch_save_image(_unused: c_ulong) -> c_int {
    let ret = swsusp_save();
    if ret == 0 {
        _soft_restart(virt_to_idmap(&raw mut cpu_resume), false);
    }
    ret
}

/* Save the current CPU state before suspend / poweroff. */
pub unsafe extern "C" fn swsusp_arch_suspend() -> c_int {
    cpu_suspend(0, arch_save_image)
}

/*
 * Restore page contents for physical pages that were in use during loading
 * hibernation image. Switch to idmap_pgd so the physical page tables are
 * overwritten with the same contents.
 */
unsafe extern "C" fn arch_restore_image(_unused: *mut u8) {
    // CONFIG_CPU_TTBR0_PAN condition is a build-time kernel configuration.
    // With it enabled, re-enable TTBR0 page-table walks before switching.
    #[cfg(CONFIG_CPU_TTBR0_PAN)]
    uaccess_save_and_enable();

    cpu_switch_mm(&raw mut idmap_pgd, &raw mut init_mm);
    let mut pbe = restore_pblist;
    while !pbe.is_null() {
        copy_page((*pbe).orig_address, (*pbe).address);
        pbe = (*pbe).next;
    }

    _soft_restart(virt_to_idmap(&raw mut cpu_resume), false);
}

static mut resume_stack: [u64; PAGE_SIZE / 2 / core::mem::size_of::<u64>()] =
    [0; PAGE_SIZE / 2 / core::mem::size_of::<u64>()];

/*
 * Resume from the hibernation image.
 * Due to the kernel heap / data restore, stack contents change underneath
 * and that would make function calls impossible; switch to a temporary
 * stack within the nosave region to avoid that problem.
 */
pub unsafe extern "C" fn swsusp_arch_resume() -> c_int {
    call_with_stack(
        arch_restore_image,
        core::ptr::null_mut(),
        resume_stack.as_mut_ptr().add(resume_stack.len()),
    );
    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
