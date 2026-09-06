// SPDX-License-Identifier: GPL-2.0-only
/*
 * mmap based event notifications for SELinux
 *
 * Author: KaiGai Kohei <kaigai@ak.jp.nec.com>
 *
 * Copyright (C) 2010 NEC corporation
 */

/*
 * C dependencies:
 *   <linux/kernel.h>
 *   <linux/gfp.h>
 *   <linux/mm.h>
 *   <linux/mutex.h>
 *   "avc.h"
 *   "security.h"
 */

use core::ffi::c_void;

/*
 * The selinux_status_page shall be exposed to userspace applications
 * using mmap interface on /selinux/status.
 * It enables to notify applications a few events that will cause reset
 * of userspace access vector without context switching.
 *
 * The selinux_kernel_status structure on the head of status page is
 * protected from concurrent accesses using seqlock logic, so userspace
 * application should reference the status page according to the seqlock
 * logic.
 *
 * Typically, application checks status->sequence at the head of access
 * control routine. If it is odd-number, kernel is updating the status,
 * so please wait for a moment. If it is changed from the last sequence
 * number, it means something happen, so application will reset userspace
 * avc, if needed.
 * In most cases, application shall confirm the kernel status is not
 * changed without any system call invocations.
 */

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct selinux_kernel_status {
    pub version: u32,
    pub sequence: u32,
    pub enforcing: u32,
    pub policyload: u32,
    pub deny_unknown: u32,
}

#[repr(C)]
pub struct selinux_state {
    pub status_lock: mutex,
    pub status_page: *mut page,
}

extern "C" {
    static mut selinux_state: selinux_state;

    static SELINUX_KERNEL_STATUS_VERSION: u32;

    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn alloc_page(gfp_mask: u32) -> *mut page;
    fn page_address(page: *mut page) -> *mut c_void;
    fn enforcing_enabled() -> u32;
    fn security_get_allow_unknown() -> bool;
    fn smp_wmb();
}

/* GFP_KERNEL and __GFP_ZERO are C preprocessor constants from <linux/gfp.h>. */
const GFP_KERNEL: u32 = 0;
const __GFP_ZERO: u32 = 0;

/*
 * selinux_kernel_status_page
 *
 * It returns a reference to selinux_status_page. If the status page is
 * not allocated yet, it also tries to allocate it at the first time.
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_kernel_status_page() -> *mut page {
    let mut status: *mut selinux_kernel_status;
    let mut result: *mut page = core::ptr::null_mut();

    mutex_lock(core::ptr::addr_of_mut!(selinux_state.status_lock));
    if selinux_state.status_page.is_null() {
        selinux_state.status_page = alloc_page(GFP_KERNEL | __GFP_ZERO);

        if !selinux_state.status_page.is_null() {
            status = page_address(selinux_state.status_page) as *mut selinux_kernel_status;

            (*status).version = SELINUX_KERNEL_STATUS_VERSION;
            (*status).sequence = 0;
            (*status).enforcing = enforcing_enabled();
            /*
             * NOTE: the next policyload event shall set
             * a positive value on the status->policyload,
             * although it may not be 1, but never zero.
             * So, application can know it was updated.
             */
            (*status).policyload = 0;
            (*status).deny_unknown = (!security_get_allow_unknown()) as u32;
        }
    }
    result = selinux_state.status_page;
    mutex_unlock(core::ptr::addr_of_mut!(selinux_state.status_lock));

    result
}

/*
 * selinux_status_update_setenforce
 *
 * It updates status of the current enforcing/permissive mode.
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_status_update_setenforce(enforcing: bool) {
    let status: *mut selinux_kernel_status;

    mutex_lock(core::ptr::addr_of_mut!(selinux_state.status_lock));
    if !selinux_state.status_page.is_null() {
        status = page_address(selinux_state.status_page) as *mut selinux_kernel_status;

        (*status).sequence = (*status).sequence.wrapping_add(1);
        smp_wmb();

        (*status).enforcing = if enforcing { 1 } else { 0 };

        smp_wmb();
        (*status).sequence = (*status).sequence.wrapping_add(1);
    }
    mutex_unlock(core::ptr::addr_of_mut!(selinux_state.status_lock));
}

/*
 * selinux_status_update_policyload
 *
 * It updates status of the times of policy reloaded, and current
 * setting of deny_unknown.
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_status_update_policyload(seqno: u32) {
    let status: *mut selinux_kernel_status;

    mutex_lock(core::ptr::addr_of_mut!(selinux_state.status_lock));
    if !selinux_state.status_page.is_null() {
        status = page_address(selinux_state.status_page) as *mut selinux_kernel_status;

        (*status).sequence = (*status).sequence.wrapping_add(1);
        smp_wmb();

        (*status).policyload = seqno;
        (*status).deny_unknown = (!security_get_allow_unknown()) as u32;

        smp_wmb();
        (*status).sequence = (*status).sequence.wrapping_add(1);
    }
    mutex_unlock(core::ptr::addr_of_mut!(selinux_state.status_lock));
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
