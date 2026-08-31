// SPDX-License-Identifier: GPL-2.0-only
/*
 * kvm asynchronous fault support
 *
 * Copyright 2010 Red Hat, Inc.
 *
 * Author:
 *      Gleb Natapov <gleb@redhat.com>
 */

// C dependencies:
// #include <linux/kvm_host.h>
// #include <linux/slab.h>
// #include <linux/module.h>
// #include <linux/mmu_context.h>
// #include <linux/sched/mm.h>
// #include "async_pf.h"
// #include <trace/events/kvm.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_int, c_ulong, c_void};
use core::ptr;

type bool_ = bool;
type gpa_t = u64;

const ENOMEM: c_int = 12;
const GFP_NOWAIT: c_int = 0;
const GFP_ATOMIC: c_int = 0;
const FOLL_WRITE: c_int = 0;
const ASYNC_PF_PER_VCPU: c_int = 64;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct work_struct {
    pub func: Option<unsafe extern "C" fn(*mut work_struct)>,
}

#[repr(C)]
pub struct kmem_cache {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kvm {
    pub mm: *mut mm_struct,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kvm_arch_async_pf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kvm_vcpu_async_pf {
    pub done: list_head,
    pub queue: list_head,
    pub lock: spinlock_t,
    pub queued: c_int,
}

#[repr(C)]
pub struct kvm_vcpu {
    pub kvm: *mut kvm,
    pub async_pf: kvm_vcpu_async_pf,
}

#[repr(C)]
pub struct kvm_async_pf {
    pub work: work_struct,
    pub queue: list_head,
    pub link: list_head,
    pub wakeup_all: bool_,
    pub vcpu: *mut kvm_vcpu,
    pub cr2_or_gpa: gpa_t,
    pub addr: c_ulong,
    pub arch: kvm_arch_async_pf,
    pub notpresent_injected: bool_,
}

unsafe extern "C" {
    fn kmem_cache_create(name: *const u8, size: usize, align: usize, flags: c_ulong, ctor: *mut c_void)
        -> *mut kmem_cache;
    fn kmem_cache_destroy(cache: *mut kmem_cache);
    fn kmem_cache_zalloc(cache: *mut kmem_cache, flags: c_int) -> *mut c_void;
    fn kmem_cache_free(cache: *mut kmem_cache, objp: *mut c_void);

    fn might_sleep();
    fn mmget_not_zero(mm: *mut mm_struct) -> c_int;
    fn mmap_read_lock(mm: *mut mm_struct);
    fn get_user_pages_remote(
        mm: *mut mm_struct,
        start: c_ulong,
        nr_pages: c_ulong,
        gup_flags: c_ulong,
        pages: *mut c_void,
        locked: *mut c_int,
    ) -> c_long;
    fn mmap_read_unlock(mm: *mut mm_struct);
    fn mmput(mm: *mut mm_struct);

    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);

    fn flush_work(work: *mut work_struct) -> bool_;
    fn cancel_work_sync(work: *mut work_struct) -> bool_;
    fn schedule_work(work: *mut work_struct) -> bool_;

    fn WARN_ON_ONCE(condition: bool_) -> bool_;
    fn kvm_is_error_hva(hva: c_ulong) -> bool_;
    fn kvm_arch_async_page_present(vcpu: *mut kvm_vcpu, work: *mut kvm_async_pf);
    fn kvm_arch_async_page_present_queued(vcpu: *mut kvm_vcpu);
    fn kvm_arch_async_page_ready(vcpu: *mut kvm_vcpu, work: *mut kvm_async_pf);
    fn kvm_arch_async_page_not_present(vcpu: *mut kvm_vcpu, work: *mut kvm_async_pf) -> bool_;
    fn kvm_arch_can_dequeue_async_page_present(vcpu: *mut kvm_vcpu) -> bool_;
    fn trace_kvm_async_pf_completed(addr: c_ulong, cr2_or_gpa: gpa_t);
    fn __kvm_vcpu_wake_up(vcpu: *mut kvm_vcpu);
}

type c_long = isize;

static mut async_pf_cache: *mut kmem_cache = ptr::null_mut();

const CONFIG_KVM_ASYNC_PF_SYNC: bool = false;

unsafe fn IS_ENABLED(option: bool) -> bool {
    option
}

unsafe fn offset_of_work() -> usize {
    core::mem::offset_of!(kvm_async_pf, work)
}

unsafe fn list_empty(head: *const list_head) -> bool {
    unsafe { (*head).next == head as *mut list_head }
}

unsafe fn list_empty_careful(head: *const list_head) -> bool {
    unsafe { list_empty(head) }
}

unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    unsafe {
        (*list).next = list;
        (*list).prev = list;
    }
}

unsafe fn __list_add(new: *mut list_head, prev: *mut list_head, next: *mut list_head) {
    unsafe {
        (*next).prev = new;
        (*new).next = next;
        (*new).prev = prev;
        (*prev).next = new;
    }
}

unsafe fn list_add_tail(new: *mut list_head, head: *mut list_head) {
    unsafe {
        __list_add(new, (*head).prev, head);
    }
}

unsafe fn __list_del(prev: *mut list_head, next: *mut list_head) {
    unsafe {
        (*next).prev = prev;
        (*prev).next = next;
    }
}

unsafe fn list_del(entry: *mut list_head) {
    unsafe {
        __list_del((*entry).prev, (*entry).next);
    }
}

unsafe fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct)) {
    unsafe {
        (*work).func = Some(func);
    }
}

unsafe fn list_first_entry_queue(head: *mut list_head) -> *mut kvm_async_pf {
    unsafe {
        ((*head).next as *mut u8).sub(core::mem::offset_of!(kvm_async_pf, queue)) as *mut kvm_async_pf
    }
}

unsafe fn list_first_entry_link(head: *mut list_head) -> *mut kvm_async_pf {
    unsafe {
        ((*head).next as *mut u8).sub(core::mem::offset_of!(kvm_async_pf, link)) as *mut kvm_async_pf
    }
}

unsafe fn unlikely(x: bool) -> bool {
    x
}

unsafe fn KMEM_CACHE_kvm_async_pf(flags: c_ulong) -> *mut kmem_cache {
    unsafe {
        kmem_cache_create(
            b"kvm_async_pf\0".as_ptr(),
            core::mem::size_of::<kvm_async_pf>(),
            core::mem::align_of::<kvm_async_pf>(),
            flags,
            ptr::null_mut(),
        )
    }
}

#[no_mangle]
pub unsafe extern "C" fn kvm_async_pf_init() -> c_int {
    unsafe {
        async_pf_cache = KMEM_CACHE_kvm_async_pf(0);

        if async_pf_cache.is_null() {
            return -ENOMEM;
        }

        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn kvm_async_pf_deinit() {
    unsafe {
        kmem_cache_destroy(async_pf_cache);
        async_pf_cache = ptr::null_mut();
    }
}

#[no_mangle]
pub unsafe extern "C" fn kvm_async_pf_vcpu_init(vcpu: *mut kvm_vcpu) {
    unsafe {
        INIT_LIST_HEAD(&mut (*vcpu).async_pf.done);
        INIT_LIST_HEAD(&mut (*vcpu).async_pf.queue);
        spin_lock_init(&mut (*vcpu).async_pf.lock);
    }
}

unsafe extern "C" fn async_pf_execute(work: *mut work_struct) {
    unsafe {
        let mut apf: *mut kvm_async_pf = (work as *mut u8).sub(offset_of_work()) as *mut kvm_async_pf;
        let vcpu: *mut kvm_vcpu = (*apf).vcpu;
        let mm: *mut mm_struct = (*(*vcpu).kvm).mm;
        let addr: c_ulong = (*apf).addr;
        let cr2_or_gpa: gpa_t = (*apf).cr2_or_gpa;
        let mut locked: c_int = 1;
        let first: bool_;

        might_sleep();

        /*
         * Attempt to pin the VM's host address space, and simply skip gup() if
         * acquiring a pin fail, i.e. if the process is exiting.  Note, KVM
         * holds a reference to its associated mm_struct until the very end of
         * kvm_destroy_vm(), i.e. the struct itself won't be freed before this
         * work item is fully processed.
         */
        if mmget_not_zero(mm) != 0 {
            mmap_read_lock(mm);
            get_user_pages_remote(mm, addr, 1, FOLL_WRITE as c_ulong, ptr::null_mut(), &mut locked);
            if locked != 0 {
                mmap_read_unlock(mm);
            }
            mmput(mm);
        }

        /*
         * Notify and kick the vCPU even if faulting in the page failed, e.g.
         * so that the vCPU can retry the fault synchronously.
         */
        if IS_ENABLED(CONFIG_KVM_ASYNC_PF_SYNC) {
            kvm_arch_async_page_present(vcpu, apf);
        }

        spin_lock(&mut (*vcpu).async_pf.lock);
        first = list_empty(&(*vcpu).async_pf.done);
        list_add_tail(&mut (*apf).link, &mut (*vcpu).async_pf.done);
        spin_unlock(&mut (*vcpu).async_pf.lock);

        /*
         * The apf struct may be freed by kvm_check_async_pf_completion() as
         * soon as the lock is dropped.  Nullify it to prevent improper usage.
         */
        apf = ptr::null_mut();
        let _ = apf;

        if !IS_ENABLED(CONFIG_KVM_ASYNC_PF_SYNC) && first {
            kvm_arch_async_page_present_queued(vcpu);
        }

        trace_kvm_async_pf_completed(addr, cr2_or_gpa);

        __kvm_vcpu_wake_up(vcpu);
    }
}

unsafe fn kvm_flush_and_free_async_pf_work(work: *mut kvm_async_pf) {
    unsafe {
        /*
         * The async #PF is "done", but KVM must wait for the work item itself,
         * i.e. async_pf_execute(), to run to completion.  If KVM is a module,
         * KVM must ensure *no* code owned by the KVM (the module) can be run
         * after the last call to module_put().  Note, flushing the work item
         * is always required when the item is taken off the completion queue.
         * E.g. even if the vCPU handles the item in the "normal" path, the VM
         * could be terminated before async_pf_execute() completes.
         *
         * Wake all events skip the queue and go straight done, i.e. don't
         * need to be flushed (but sanity check that the work wasn't queued).
         */
        if (*work).wakeup_all {
            WARN_ON_ONCE((*work).work.func.is_some());
        } else {
            flush_work(&mut (*work).work);
        }
        kmem_cache_free(async_pf_cache, work as *mut c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn kvm_clear_async_pf_completion_queue(vcpu: *mut kvm_vcpu) {
    unsafe {
        /* cancel outstanding work queue item */
        while !list_empty(&(*vcpu).async_pf.queue) {
            let work: *mut kvm_async_pf = list_first_entry_queue(&mut (*vcpu).async_pf.queue);
            list_del(&mut (*work).queue);

            // #ifdef CONFIG_KVM_ASYNC_PF_SYNC
            if CONFIG_KVM_ASYNC_PF_SYNC {
                flush_work(&mut (*work).work);
            } else {
                // #else
                if cancel_work_sync(&mut (*work).work) {
                    kmem_cache_free(async_pf_cache, work as *mut c_void);
                }
                // #endif
            }
        }

        spin_lock(&mut (*vcpu).async_pf.lock);
        while !list_empty(&(*vcpu).async_pf.done) {
            let work: *mut kvm_async_pf = list_first_entry_link(&mut (*vcpu).async_pf.done);
            list_del(&mut (*work).link);

            spin_unlock(&mut (*vcpu).async_pf.lock);
            kvm_flush_and_free_async_pf_work(work);
            spin_lock(&mut (*vcpu).async_pf.lock);
        }
        spin_unlock(&mut (*vcpu).async_pf.lock);

        (*vcpu).async_pf.queued = 0;
    }
}

#[no_mangle]
pub unsafe extern "C" fn kvm_check_async_pf_completion(vcpu: *mut kvm_vcpu) {
    unsafe {
        let mut work: *mut kvm_async_pf;

        while !list_empty_careful(&(*vcpu).async_pf.done)
            && kvm_arch_can_dequeue_async_page_present(vcpu)
        {
            spin_lock(&mut (*vcpu).async_pf.lock);
            work = list_first_entry_link(&mut (*vcpu).async_pf.done);
            list_del(&mut (*work).link);
            spin_unlock(&mut (*vcpu).async_pf.lock);

            kvm_arch_async_page_ready(vcpu, work);
            if !IS_ENABLED(CONFIG_KVM_ASYNC_PF_SYNC) {
                kvm_arch_async_page_present(vcpu, work);
            }

            list_del(&mut (*work).queue);
            (*vcpu).async_pf.queued -= 1;
            kvm_flush_and_free_async_pf_work(work);
        }
    }
}

/*
 * Try to schedule a job to handle page fault asynchronously. Returns 'true' on
 * success, 'false' on failure (page fault has to be handled synchronously).
 */
#[no_mangle]
pub unsafe extern "C" fn kvm_setup_async_pf(
    vcpu: *mut kvm_vcpu,
    cr2_or_gpa: gpa_t,
    hva: c_ulong,
    arch: *mut kvm_arch_async_pf,
) -> bool_ {
    unsafe {
        let work: *mut kvm_async_pf;

        if (*vcpu).async_pf.queued >= ASYNC_PF_PER_VCPU {
            return false;
        }

        /* Arch specific code should not do async PF in this case */
        if unlikely(kvm_is_error_hva(hva)) {
            return false;
        }

        /*
         * do alloc nowait since if we are going to sleep anyway we
         * may as well sleep faulting in page
         */
        work = kmem_cache_zalloc(async_pf_cache, GFP_NOWAIT) as *mut kvm_async_pf;
        if work.is_null() {
            return false;
        }

        (*work).wakeup_all = false;
        (*work).vcpu = vcpu;
        (*work).cr2_or_gpa = cr2_or_gpa;
        (*work).addr = hva;
        ptr::copy_nonoverlapping(arch, &mut (*work).arch, 1);

        INIT_WORK(&mut (*work).work, async_pf_execute);

        list_add_tail(&mut (*work).queue, &mut (*vcpu).async_pf.queue);
        (*vcpu).async_pf.queued += 1;
        (*work).notpresent_injected = kvm_arch_async_page_not_present(vcpu, work);

        schedule_work(&mut (*work).work);

        true
    }
}

#[no_mangle]
pub unsafe extern "C" fn kvm_async_pf_wakeup_all(vcpu: *mut kvm_vcpu) -> c_int {
    unsafe {
        let work: *mut kvm_async_pf;
        let first: bool_;

        if !list_empty_careful(&(*vcpu).async_pf.done) {
            return 0;
        }

        work = kmem_cache_zalloc(async_pf_cache, GFP_ATOMIC) as *mut kvm_async_pf;
        if work.is_null() {
            return -ENOMEM;
        }

        (*work).wakeup_all = true;
        INIT_LIST_HEAD(&mut (*work).queue); /* for list_del to work */

        spin_lock(&mut (*vcpu).async_pf.lock);
        first = list_empty(&(*vcpu).async_pf.done);
        list_add_tail(&mut (*work).link, &mut (*vcpu).async_pf.done);
        spin_unlock(&mut (*vcpu).async_pf.lock);

        if !IS_ENABLED(CONFIG_KVM_ASYNC_PF_SYNC) && first {
            kvm_arch_async_page_present_queued(vcpu);
        }

        (*vcpu).async_pf.queued += 1;
        0
    }
}
