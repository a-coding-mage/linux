// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Misc and compatibility things
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr;

pub type u16 = u16;

pub const GFP_KERNEL: u32 = 0;
pub const ENOMEM: i32 = 12;

#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    pub subsystem_vendor: u16,
    pub subsystem_device: u16,
}

#[repr(C)]
pub struct snd_pci_quirk {
    pub subvendor: u16,
    pub subdevice: u16,
    pub subdevice_mask: u16,
}

#[repr(C)]
pub struct fasync_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct atomic_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_refcount {
    pub count: atomic_t,
    pub waiter: wait_queue_head_t,
}

unsafe extern "C" {
    fn release_resource(res: *mut resource);
    fn kfree(ptr: *const c_void);
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn fasync_helper(
        fd: i32,
        file: *mut file,
        on: i32,
        fasync: *mut *mut fasync_struct,
    ) -> i32;
    fn kill_fasync(fasync: *mut *mut fasync_struct, signal: i32, poll: i32);
    fn schedule_work(work: *mut work_struct) -> bool;
    fn flush_work(work: *mut work_struct) -> bool;

    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t) -> usize;
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);

    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_empty(head: *const list_head) -> i32;
    fn list_del_init(entry: *mut list_head);
    fn list_move(list: *mut list_head, head: *mut list_head);

    fn atomic_set(v: *mut atomic_t, i: i32);
    fn atomic_dec_and_test(v: *mut atomic_t) -> i32;
    fn atomic_read(v: *const atomic_t) -> i32;
    fn init_waitqueue_head(wq_head: *mut wait_queue_head_t);
    fn wake_up(wq_head: *mut wait_queue_head_t);
    fn wait_event_refcount_zero(wq_head: *mut wait_queue_head_t, ref_: *mut snd_refcount);
}

#[inline]
unsafe fn container_of_snd_fasync_list(ptr: *mut list_head) -> *mut snd_fasync {
    (ptr as *mut u8).sub(core::mem::offset_of!(snd_fasync, list)) as *mut snd_fasync
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn release_and_free_resource(res: *mut resource) {
    if !res.is_null() {
        unsafe {
            release_resource(res);
            kfree(res as *const c_void);
        }
    }
}
/* EXPORT_SYMBOL(release_and_free_resource); */

/* CONFIG_PCI */
/**
 * snd_pci_quirk_lookup_id - look up a PCI SSID quirk list
 * @vendor: PCI SSV id
 * @device: PCI SSD id
 * @list: quirk list, terminated by a null entry
 *
 * Look through the given quirk list and finds a matching entry
 * with the same PCI SSID.  When subdevice is 0, all subdevice
 * values may match.
 *
 * Returns the matched entry pointer, or NULL if nothing matched.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_pci_quirk_lookup_id(
    vendor: u16,
    device: u16,
    list: *const snd_pci_quirk,
) -> *const snd_pci_quirk {
    let mut q: *const snd_pci_quirk = list;

    unsafe {
        while (*q).subvendor != 0 || (*q).subdevice != 0 {
            if (*q).subvendor != vendor {
                q = q.add(1);
                continue;
            }
            if (*q).subdevice == 0 || (device & (*q).subdevice_mask) == (*q).subdevice {
                return q;
            }
            q = q.add(1);
        }
    }
    ptr::null()
}
/* EXPORT_SYMBOL(snd_pci_quirk_lookup_id); */

/**
 * snd_pci_quirk_lookup - look up a PCI SSID quirk list
 * @pci: pci_dev handle
 * @list: quirk list, terminated by a null entry
 *
 * Look through the given quirk list and finds a matching entry
 * with the same PCI SSID.  When subdevice is 0, all subdevice
 * values may match.
 *
 * Returns the matched entry pointer, or NULL if nothing matched.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_pci_quirk_lookup(
    pci: *mut pci_dev,
    list: *const snd_pci_quirk,
) -> *const snd_pci_quirk {
    if pci.is_null() {
        return ptr::null();
    }
    unsafe { snd_pci_quirk_lookup_id((*pci).subsystem_vendor, (*pci).subsystem_device, list) }
}
/* EXPORT_SYMBOL(snd_pci_quirk_lookup); */

/*
 * Deferred async signal helpers
 *
 * Below are a few helper functions to wrap the async signal handling
 * in the deferred work.  The main purpose is to avoid the messy deadlock
 * around tasklist_lock and co at the kill_fasync() invocation.
 * fasync_helper() and kill_fasync() are replaced with snd_fasync_helper()
 * and snd_kill_fasync(), respectively.  In addition, snd_fasync_free() has
 * to be called at releasing the relevant file object.
 */
#[repr(C)]
pub struct snd_fasync {
    pub fasync: *mut fasync_struct,
    pub signal: i32,
    pub poll: i32,
    pub on: i32,
    pub list: list_head,
}

/* static DEFINE_SPINLOCK(snd_fasync_lock); */
static mut snd_fasync_lock: spinlock_t = spinlock_t { _private: [] };
/* static LIST_HEAD(snd_fasync_list); */
static mut snd_fasync_list: list_head = list_head {
    next: ptr::null_mut(),
    prev: ptr::null_mut(),
};

unsafe extern "C" fn snd_fasync_work_fn(_work: *mut work_struct) {
    let mut fasync: *mut snd_fasync;
    let signal: i32;
    let poll: i32;

    unsafe {
        spin_lock_irq(&raw mut snd_fasync_lock);
        while list_empty(&raw const snd_fasync_list) == 0 {
            fasync = container_of_snd_fasync_list(snd_fasync_list.next);
            list_del_init(&mut (*fasync).list);
            if (*fasync).on == 0 {
                continue;
            }
            signal = (*fasync).signal;
            poll = (*fasync).poll;
            spin_unlock_irq(&raw mut snd_fasync_lock);
            kill_fasync(&mut (*fasync).fasync, signal, poll);
            spin_lock_irq(&raw mut snd_fasync_lock);
        }
        spin_unlock_irq(&raw mut snd_fasync_lock);
    }
}

/* static DECLARE_WORK(snd_fasync_work, snd_fasync_work_fn); */
static mut snd_fasync_work: work_struct = work_struct { _private: [] };

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_fasync_helper(
    fd: i32,
    file: *mut file,
    on: i32,
    fasyncp: *mut *mut snd_fasync,
) -> i32 {
    let mut fasync: *mut snd_fasync = ptr::null_mut();

    unsafe {
        if on != 0 {
            fasync = kzalloc(size_of::<snd_fasync>(), GFP_KERNEL) as *mut snd_fasync;
            if fasync.is_null() {
                return -ENOMEM;
            }
            INIT_LIST_HEAD(&mut (*fasync).list);
        }

        spin_lock_irq(&raw mut snd_fasync_lock);
        if !(*fasyncp).is_null() {
            kfree(fasync as *const c_void);
            fasync = *fasyncp;
        } else {
            if fasync.is_null() {
                spin_unlock_irq(&raw mut snd_fasync_lock);
                return 0;
            }
            *fasyncp = fasync;
        }
        (*fasync).on = on;
        spin_unlock_irq(&raw mut snd_fasync_lock);

        fasync_helper(fd, file, on, &mut (*fasync).fasync)
    }
}
/* EXPORT_SYMBOL_GPL(snd_fasync_helper); */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_kill_fasync(fasync: *mut snd_fasync, signal: i32, poll: i32) {
    if fasync.is_null() {
        return;
    }
    unsafe {
        let flags = spin_lock_irqsave(&raw mut snd_fasync_lock);
        if (*fasync).on == 0 {
            spin_unlock_irqrestore(&raw mut snd_fasync_lock, flags);
            return;
        }
        (*fasync).signal = signal;
        (*fasync).poll = poll;
        list_move(&mut (*fasync).list, &raw mut snd_fasync_list);
        schedule_work(&raw mut snd_fasync_work);
        spin_unlock_irqrestore(&raw mut snd_fasync_lock, flags);
    }
}
/* EXPORT_SYMBOL_GPL(snd_kill_fasync); */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_fasync_free(fasync: *mut snd_fasync) {
    if fasync.is_null() {
        return;
    }

    unsafe {
        spin_lock_irq(&raw mut snd_fasync_lock);
        (*fasync).on = 0;
        list_del_init(&mut (*fasync).list);
        spin_unlock_irq(&raw mut snd_fasync_lock);

        flush_work(&raw mut snd_fasync_work);
        kfree(fasync as *const c_void);
    }
}
/* EXPORT_SYMBOL_GPL(snd_fasync_free); */

/*
 * generic refcount helper
 */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_refcount_init(ref_: *mut snd_refcount) {
    unsafe {
        atomic_set(&mut (*ref_).count, 0);
        init_waitqueue_head(&mut (*ref_).waiter);
    }
}
/* EXPORT_SYMBOL_GPL(snd_refcount_init); */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_refcount_put(ref_: *mut snd_refcount) {
    unsafe {
        if atomic_dec_and_test(&mut (*ref_).count) != 0 {
            wake_up(&mut (*ref_).waiter);
        }
    }
}
/* EXPORT_SYMBOL_GPL(snd_refcount_put); */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_refcount_sync(ref_: *mut snd_refcount) {
    unsafe {
        /* wait_event(ref->waiter, !atomic_read(&ref->count)); */
        while atomic_read(&(*ref_).count) != 0 {
            wait_event_refcount_zero(&mut (*ref_).waiter, ref_);
        }
    }
}
/* EXPORT_SYMBOL_GPL(snd_refcount_sync); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
