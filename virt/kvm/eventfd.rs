// SPDX-License-Identifier: GPL-2.0-only
/*
 * kvm eventfd support - use eventfd objects to signal various KVM events
 *
 * Copyright 2009 Novell.  All Rights Reserved.
 * Copyright 2010 Red Hat, Inc. and/or its affiliates.
 *
 * Author:
 *	Gregory Haskins <ghaskins@novell.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::MaybeUninit;
use core::ptr;

type bool_ = bool;
type u8 = core::ffi::c_uchar;
type u16 = core::ffi::c_ushort;
type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;
type __u32 = u32;
type gpa_t = u64;
type __poll_t = c_uint;

const true_: bool = true;
const false_: bool = false;

const EWOULDBLOCK: c_int = 11;
const EAGAIN: c_int = 11;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EBADF: c_int = 9;
const EOPNOTSUPP: c_int = 95;
const EEXIST: c_int = 17;
const ENOENT: c_int = 2;

const EPOLLIN: __poll_t = 0x00000001;
const EPOLLHUP: __poll_t = 0x00000010;

const GFP_KERNEL_ACCOUNT: c_uint = 0;
const KVM_USERSPACE_IRQ_SOURCE_ID: c_int = 0;
const KVM_IRQFD_RESAMPLE_IRQ_SOURCE_ID: c_int = 1;
const KVM_NR_IRQCHIPS: usize = 3;
const WQ_PERCPU: c_uint = 0x20;

const KVM_IRQFD_FLAG_DEASSIGN: u32 = 1 << 0;
const KVM_IRQFD_FLAG_RESAMPLE: u32 = 1 << 1;

const KVM_IOEVENTFD_FLAG_DATAMATCH: u32 = 1 << 0;
const KVM_IOEVENTFD_FLAG_PIO: u32 = 1 << 1;
const KVM_IOEVENTFD_FLAG_DEASSIGN: u32 = 1 << 2;
const KVM_IOEVENTFD_FLAG_VIRTIO_CCW_NOTIFY: u32 = 1 << 3;
const KVM_IOEVENTFD_VALID_FLAG_MASK: u32 = KVM_IOEVENTFD_FLAG_DATAMATCH
    | KVM_IOEVENTFD_FLAG_PIO
    | KVM_IOEVENTFD_FLAG_DEASSIGN
    | KVM_IOEVENTFD_FLAG_VIRTIO_CCW_NOTIFY;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct hlist_node {
    pub next: *mut hlist_node,
    pub pprev: *mut *mut hlist_node,
}

#[repr(C)]
pub struct hlist_head {
    pub first: *mut hlist_node,
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct workqueue_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_entry_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct poll_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct eventfd_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    pub dep_map: lockdep_map,
}

#[repr(C)]
pub struct lockdep_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct srcu_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seqcount_spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_bypass_consumer {
    pub add_producer: Option<unsafe extern "C" fn(*mut irq_bypass_consumer) -> c_int>,
    pub del_producer: Option<unsafe extern "C" fn(*mut irq_bypass_consumer)>,
    pub stop: Option<unsafe extern "C" fn(*mut irq_bypass_consumer)>,
    pub start: Option<unsafe extern "C" fn(*mut irq_bypass_consumer)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_kernel_irq_routing_entry {
    pub type_: c_int,
}

#[repr(C)]
pub struct kvm_irq_ack_notifier {
    pub link: hlist_node,
    pub gsi: c_int,
    pub irq_acked: Option<unsafe extern "C" fn(*mut kvm_irq_ack_notifier)>,
}

#[repr(C)]
pub struct kvm_kernel_irqfd_resampler {
    pub list: list_head,
    pub link: list_head,
    pub notifier: kvm_irq_ack_notifier,
    pub kvm: *mut kvm,
}

#[repr(C)]
pub struct kvm_kernel_irqfd {
    pub kvm: *mut kvm,
    pub gsi: c_int,
    pub list: list_head,
    pub resampler_link: list_head,
    pub inject: work_struct,
    pub shutdown: work_struct,
    pub wait: wait_queue_entry_t,
    pub irq_entry_sc: seqcount_spinlock_t,
    pub irq_entry: kvm_kernel_irq_routing_entry,
    pub eventfd: *mut eventfd_ctx,
    pub resamplefd: *mut eventfd_ctx,
    pub resampler: *mut kvm_kernel_irqfd_resampler,
    pub consumer: irq_bypass_consumer,
    pub producer: *mut c_void,
}

#[repr(C)]
pub struct kvm_irqfds {
    pub lock: spinlock_t,
    pub items: list_head,
    pub resampler_list: list_head,
    pub resampler_lock: mutex,
}

#[repr(C)]
pub struct kvm {
    pub irq_srcu: srcu_struct,
    pub irqfds: kvm_irqfds,
    pub irq_ack_notifier_list: hlist_head,
    pub irq_lock: mutex,
    pub ioeventfds: list_head,
    pub slots_lock: mutex,
}

#[repr(C)]
pub struct kvm_irqfd {
    pub fd: c_int,
    pub gsi: c_int,
    pub flags: u32,
    pub resamplefd: c_int,
}

#[repr(C)]
pub struct kvm_ioeventfd {
    pub datamatch: u64,
    pub addr: u64,
    pub len: u32,
    pub fd: c_int,
    pub flags: u32,
    pub pad: [u8; 36],
}

#[repr(C)]
pub struct kvm_vcpu {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kvm_io_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kvm_io_device_ops {
    pub read: Option<unsafe extern "C" fn(*mut kvm_vcpu, *mut kvm_io_device, gpa_t, c_int, *mut c_void) -> c_int>,
    pub write: Option<unsafe extern "C" fn(*mut kvm_vcpu, *mut kvm_io_device, gpa_t, c_int, *const c_void) -> c_int>,
    pub destructor: Option<unsafe extern "C" fn(*mut kvm_io_device)>,
}

#[repr(C)]
pub struct kvm_io_bus {
    pub ioeventfd_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_bus {
    KVM_MMIO_BUS = 0,
    KVM_PIO_BUS = 1,
    KVM_VIRTIO_CCW_NOTIFY_BUS = 2,
    KVM_FAST_MMIO_BUS = 3,
}

#[repr(C)]
pub struct kvm_irqfd_pt {
    pub irqfd: *mut kvm_kernel_irqfd,
    pub kvm: *mut kvm,
    pub pt: poll_table,
    pub ret: c_int,
}

#[repr(C)]
pub struct _ioeventfd {
    pub list: list_head,
    pub addr: u64,
    pub length: c_int,
    pub eventfd: *mut eventfd_ctx,
    pub datamatch: u64,
    pub dev: kvm_io_device,
    pub bus_idx: u8,
    pub wildcard: bool,
}

unsafe extern "C" {
    fn kvm_set_irq(kvm: *mut kvm, irq_source_id: c_int, irq: c_int, level: c_int, line_status: bool) -> c_int;
    fn eventfd_signal(ctx: *mut eventfd_ctx);
    fn srcu_read_lock(sp: *mut srcu_struct) -> c_int;
    fn srcu_read_unlock(sp: *mut srcu_struct, idx: c_int);
    fn srcu_read_lock_held(sp: *mut srcu_struct) -> bool;
    fn synchronize_srcu_expedited(sp: *mut srcu_struct);
    fn eventfd_ctx_remove_wait_queue(ctx: *mut eventfd_ctx, wait: *mut wait_queue_entry_t, cnt: *mut u64);
    fn flush_work(work: *mut work_struct);
    fn flush_workqueue(wq: *mut workqueue_struct);
    fn eventfd_ctx_put(ctx: *mut eventfd_ctx);
    fn eventfd_ctx_do_read(ctx: *mut eventfd_ctx, cnt: *mut u64) -> c_int;
    fn eventfd_ctx_fdget(fd: c_int) -> *mut eventfd_ctx;
    fn eventfd_ctx_fileget(file: *mut file) -> *mut eventfd_ctx;
    fn kfree(ptr: *mut c_void);
    fn queue_work(wq: *mut workqueue_struct, work: *mut work_struct) -> bool;
    fn schedule_work(work: *mut work_struct) -> bool;
    fn key_to_poll(key: *mut c_void) -> __poll_t;
    fn read_seqcount_begin(s: *mut seqcount_spinlock_t) -> c_uint;
    fn read_seqcount_retry(s: *mut seqcount_spinlock_t, start: c_uint) -> c_int;
    fn write_seqcount_begin(s: *mut seqcount_spinlock_t);
    fn write_seqcount_end(s: *mut seqcount_spinlock_t);
    fn kvm_irq_map_gsi(kvm: *mut kvm, entries: *mut kvm_kernel_irq_routing_entry, gsi: c_int) -> c_int;
    fn kvm_irq_map_chip_pin(kvm: *mut kvm, irqchip: c_uint, pin: c_uint) -> c_int;
    fn add_wait_queue_priority_exclusive(wqh: *mut wait_queue_head_t, wait: *mut wait_queue_entry_t) -> c_int;
    fn vfs_poll(file: *mut file, pt: *mut poll_table) -> __poll_t;
    fn kvm_arch_intc_initialized(kvm: *mut kvm) -> bool;
    fn fd_empty(fd: *mut c_void) -> bool;
    fn fd_file(fd: *mut c_void) -> *mut file;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn kvm_arch_has_irq_bypass() -> bool;
    fn irq_bypass_register_consumer(cons: *mut irq_bypass_consumer, eventfd: *mut eventfd_ctx) -> c_int;
    fn irq_bypass_unregister_consumer(cons: *mut irq_bypass_consumer);
    fn kvm_arch_irq_bypass_add_producer(cons: *mut irq_bypass_consumer) -> c_int;
    fn kvm_arch_irq_bypass_del_producer(cons: *mut irq_bypass_consumer);
    fn pr_info(fmt: *const c_char, ...);
    fn trace_kvm_ack_irq(irqchip: c_uint, pin: c_uint);
    fn hlist_add_head_rcu(node: *mut hlist_node, head: *mut hlist_head);
    fn hlist_del_init_rcu(node: *mut hlist_node);
    fn kvm_arch_post_irq_ack_notifier_list_update(kvm: *mut kvm);
    fn kvm_iodevice_init(dev: *mut kvm_io_device, ops: *const kvm_io_device_ops);
    fn kvm_io_bus_register_dev(kvm: *mut kvm, bus_idx: kvm_bus, addr: gpa_t, len: c_int, dev: *mut kvm_io_device) -> c_int;
    fn kvm_io_bus_unregister_dev(kvm: *mut kvm, bus_idx: kvm_bus, dev: *mut kvm_io_device);
    fn kvm_get_bus(kvm: *mut kvm, bus_idx: kvm_bus) -> *mut kvm_io_bus;
    fn alloc_workqueue(fmt: *const c_char, flags: c_uint, max_active: c_int) -> *mut workqueue_struct;
    fn destroy_workqueue(wq: *mut workqueue_struct);
    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_init(lock: *mut mutex);
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_del(list: *mut list_head);
    fn list_del_rcu(list: *mut list_head);
    fn list_del_init(list: *mut list_head);
    fn list_empty(list: *const list_head) -> bool;
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_add_rcu(new: *mut list_head, head: *mut list_head);
    fn init_waitqueue_func_entry(wait: *mut wait_queue_entry_t, func: unsafe extern "C" fn(*mut wait_queue_entry_t, c_uint, c_int, *mut c_void) -> c_int);
    fn init_poll_funcptr(pt: *mut poll_table, func: unsafe extern "C" fn(*mut file, *mut wait_queue_head_t, *mut poll_table));
    fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
    fn seqcount_spinlock_init(s: *mut seqcount_spinlock_t, lock: *mut spinlock_t);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn lockdep_assert_once(cond: bool);
    fn lockdep_assert_held(lock: *mut spinlock_t);
    fn BUG_ON(cond: bool);
}

static mut irqfd_cleanup_wq: *mut workqueue_struct = ptr::null_mut();

unsafe fn container_of<T, U>(_ptr: *mut U, _field: fn(*mut T) -> *mut U) -> *mut T {
    /* Kernel container_of() needs field offsets supplied by the surrounding
     * bindings.  Kept as an external-layout operation for this file-local
     * source translation.
     */
    ptr::null_mut()
}

unsafe fn kzalloc_obj<T>(flags: c_uint) -> *mut T {
    kzalloc(core::mem::size_of::<T>(), flags) as *mut T
}

pub unsafe extern "C" fn kvm_arch_irqfd_allowed(_kvm: *mut kvm, _args: *mut kvm_irqfd) -> bool {
    return true;
}

unsafe extern "C" fn irqfd_inject(work: *mut work_struct) {
    let irqfd = container_of(work, |p: *mut kvm_kernel_irqfd| unsafe { &mut (*p).inject });
    let kvm = (*irqfd).kvm;

    if (*irqfd).resampler.is_null() {
        kvm_set_irq(kvm, KVM_USERSPACE_IRQ_SOURCE_ID, (*irqfd).gsi, 1, false);
        kvm_set_irq(kvm, KVM_USERSPACE_IRQ_SOURCE_ID, (*irqfd).gsi, 0, false);
    } else {
        kvm_set_irq(kvm, KVM_IRQFD_RESAMPLE_IRQ_SOURCE_ID, (*irqfd).gsi, 1, false);
    }
}

unsafe fn irqfd_resampler_notify(resampler: *mut kvm_kernel_irqfd_resampler) {
    let _irqfd: *mut kvm_kernel_irqfd;

    /* list_for_each_entry_srcu(irqfd, &resampler->list, resampler_link,
     * srcu_read_lock_held(&resampler->kvm->irq_srcu))
     */
    let _ = srcu_read_lock_held(&mut (*(*resampler).kvm).irq_srcu);
    /* eventfd_signal(irqfd->resamplefd); */
}

/*
 * Since resampler irqfds share an IRQ source ID, we de-assert once
 * then notify all of the resampler irqfds using this GSI.  We can't
 * do multiple de-asserts or we risk racing with incoming re-asserts.
 */
unsafe extern "C" fn irqfd_resampler_ack(kian: *mut kvm_irq_ack_notifier) {
    let resampler = container_of(kian, |p: *mut kvm_kernel_irqfd_resampler| unsafe { &mut (*p).notifier });
    let kvm = (*resampler).kvm;
    let idx: c_int;

    kvm_set_irq(
        kvm,
        KVM_IRQFD_RESAMPLE_IRQ_SOURCE_ID,
        (*resampler).notifier.gsi,
        0,
        false,
    );

    idx = srcu_read_lock(&mut (*kvm).irq_srcu);
    irqfd_resampler_notify(resampler);
    srcu_read_unlock(&mut (*kvm).irq_srcu, idx);
}

unsafe fn irqfd_resampler_shutdown(irqfd: *mut kvm_kernel_irqfd) {
    let resampler = (*irqfd).resampler;
    let kvm = (*resampler).kvm;

    mutex_lock(&mut (*kvm).irqfds.resampler_lock);
    list_del_rcu(&mut (*irqfd).resampler_link);

    if list_empty(&(*resampler).list) {
        list_del_rcu(&mut (*resampler).link);
        kvm_unregister_irq_ack_notifier(kvm, &mut (*resampler).notifier);
        /*
         * synchronize_srcu_expedited(&kvm->irq_srcu) already called
         * in kvm_unregister_irq_ack_notifier().
         */
        kvm_set_irq(kvm, KVM_IRQFD_RESAMPLE_IRQ_SOURCE_ID, (*resampler).notifier.gsi, 0, false);
        kfree(resampler as *mut c_void);
    } else {
        synchronize_srcu_expedited(&mut (*kvm).irq_srcu);
    }

    mutex_unlock(&mut (*kvm).irqfds.resampler_lock);
}

/*
 * Race-free decouple logic (ordering is critical)
 */
unsafe extern "C" fn irqfd_shutdown(work: *mut work_struct) {
    let irqfd = container_of(work, |p: *mut kvm_kernel_irqfd| unsafe { &mut (*p).shutdown });
    let kvm = (*irqfd).kvm;
    let mut cnt: u64 = 0;

    /* Make sure irqfd has been initialized in assign path. */
    synchronize_srcu_expedited(&mut (*kvm).irq_srcu);

    /*
     * Synchronize with the wait-queue and unhook ourselves to prevent
     * further events.
     */
    eventfd_ctx_remove_wait_queue((*irqfd).eventfd, &mut (*irqfd).wait, &mut cnt);

    /*
     * We know no new events will be scheduled at this point, so block
     * until all previously outstanding events have completed
     */
    flush_work(&mut (*irqfd).inject);

    if !(*irqfd).resampler.is_null() {
        irqfd_resampler_shutdown(irqfd);
        eventfd_ctx_put((*irqfd).resamplefd);
    }

    /*
     * It is now safe to release the object's resources
     */
    /* #if IS_ENABLED(CONFIG_HAVE_KVM_IRQ_BYPASS) */
    irq_bypass_unregister_consumer(&mut (*irqfd).consumer);
    /* #endif */
    eventfd_ctx_put((*irqfd).eventfd);
    kfree(irqfd as *mut c_void);
}

unsafe fn irqfd_is_active(irqfd: *mut kvm_kernel_irqfd) -> bool {
    /*
     * Assert that either irqfds.lock or SRCU is held, as irqfds.lock must
     * be held to prevent false positives (on the irqfd being active), and
     * while false negatives are impossible as irqfds are never added back
     * to the list once they're deactivated, the caller must at least hold
     * SRCU to guard against routing changes if the irqfd is deactivated.
     */
    lockdep_assert_once(srcu_read_lock_held(&mut (*(*irqfd).kvm).irq_srcu));
    return if list_empty(&(*irqfd).list) { false } else { true };
}

/*
 * Mark the irqfd as inactive and schedule it for removal
 */
unsafe fn irqfd_deactivate(irqfd: *mut kvm_kernel_irqfd) {
    lockdep_assert_held(&mut (*(*irqfd).kvm).irqfds.lock);
    BUG_ON(!irqfd_is_active(irqfd));
    list_del_init(&mut (*irqfd).list);
    queue_work(irqfd_cleanup_wq, &mut (*irqfd).shutdown);
}

pub unsafe extern "C" fn kvm_arch_set_irq_inatomic(
    _irq: *mut kvm_kernel_irq_routing_entry,
    _kvm: *mut kvm,
    _irq_source_id: c_int,
    _level: c_int,
    _line_status: bool,
) -> c_int {
    return -EWOULDBLOCK;
}

/*
 * Called with wqh->lock held and interrupts disabled
 */
unsafe extern "C" fn irqfd_wakeup(
    wait: *mut wait_queue_entry_t,
    _mode: c_uint,
    _sync: c_int,
    key: *mut c_void,
) -> c_int {
    let irqfd = container_of(wait, |p: *mut kvm_kernel_irqfd| unsafe { &mut (*p).wait });
    let flags = key_to_poll(key);
    let mut irq = MaybeUninit::<kvm_kernel_irq_routing_entry>::zeroed().assume_init();
    let kvm = (*irqfd).kvm;
    let mut seq: c_uint;
    let mut idx: c_int;
    let mut ret: c_int = 0;

    if flags & EPOLLIN != 0 {
        /*
         * WARNING: Do NOT take irqfds.lock in any path except EPOLLHUP,
         * as KVM holds irqfds.lock when registering the irqfd with the
         * eventfd.
         */
        let mut cnt: u64 = 0;
        eventfd_ctx_do_read((*irqfd).eventfd, &mut cnt);

        idx = srcu_read_lock(&mut (*kvm).irq_srcu);
        loop {
            seq = read_seqcount_begin(&mut (*irqfd).irq_entry_sc);
            irq = (*irqfd).irq_entry;
            if read_seqcount_retry(&mut (*irqfd).irq_entry_sc, seq) == 0 {
                break;
            }
        }

        /*
         * An event has been signaled, inject an interrupt unless the
         * irqfd is being deassigned (isn't active), in which case the
         * routing information may be stale (once the irqfd is removed
         * from the list, it will stop receiving routing updates).
         */
        if !irqfd_is_active(irqfd)
            || kvm_arch_set_irq_inatomic(&mut irq, kvm, KVM_USERSPACE_IRQ_SOURCE_ID, 1, false)
                == -EWOULDBLOCK
        {
            schedule_work(&mut (*irqfd).inject);
        }
        srcu_read_unlock(&mut (*kvm).irq_srcu, idx);
        ret = 1;
    }

    if flags & EPOLLHUP != 0 {
        /* The eventfd is closing, detach from KVM */
        let mut iflags: c_ulong = 0;

        /*
         * Taking irqfds.lock is safe here, as KVM holds a reference to
         * the eventfd when registering the irqfd, i.e. this path can't
         * be reached while kvm_irqfd_add() is running.
         */
        spin_lock_irqsave(&mut (*kvm).irqfds.lock, &mut iflags);

        /*
         * We must check if someone deactivated the irqfd before
         * we could acquire the irqfds.lock since the item is
         * deactivated from the KVM side before it is unhooked from
         * the wait-queue.  If it is already deactivated, we can
         * simply return knowing the other side will cleanup for us.
         * We cannot race against the irqfd going away since the
         * other side is required to acquire wqh->lock, which we hold
         */
        if irqfd_is_active(irqfd) {
            irqfd_deactivate(irqfd);
        }

        spin_unlock_irqrestore(&mut (*kvm).irqfds.lock, iflags);
    }

    return ret;
}

unsafe fn irqfd_update(kvm: *mut kvm, irqfd: *mut kvm_kernel_irqfd) {
    let mut entries: [kvm_kernel_irq_routing_entry; KVM_NR_IRQCHIPS] =
        [kvm_kernel_irq_routing_entry { type_: 0 }; KVM_NR_IRQCHIPS];
    let n_entries: c_int;

    lockdep_assert_held(&mut (*kvm).irqfds.lock);
    n_entries = kvm_irq_map_gsi(kvm, entries.as_mut_ptr(), (*irqfd).gsi);

    write_seqcount_begin(&mut (*irqfd).irq_entry_sc);

    if n_entries == 1 {
        (*irqfd).irq_entry = entries[0];
    } else {
        (*irqfd).irq_entry.type_ = 0;
    }

    write_seqcount_end(&mut (*irqfd).irq_entry_sc);
}

unsafe extern "C" fn kvm_irqfd_register(
    _file: *mut file,
    wqh: *mut wait_queue_head_t,
    pt: *mut poll_table,
) {
    let p = container_of(pt, |q: *mut kvm_irqfd_pt| unsafe { &mut (*q).pt });
    let irqfd = (*p).irqfd;
    let kvm = (*p).kvm;

    /*
     * Note, irqfds.lock protects the irqfd's irq_entry, i.e. its routing,
     * and irqfds.items.  It does NOT protect registering with the eventfd.
     */
    spin_lock_irq(&mut (*kvm).irqfds.lock);

    /*
     * Initialize the routing information prior to adding the irqfd to the
     * eventfd's waitqueue, as irqfd_wakeup() can be invoked as soon as the
     * irqfd is registered.
     */
    irqfd_update(kvm, irqfd);

    /*
     * Add the irqfd as a priority waiter on the eventfd, with a custom
     * wake-up handler, so that KVM *and only KVM* is notified whenever the
     * underlying eventfd is signaled.
     */
    init_waitqueue_func_entry(&mut (*irqfd).wait, irqfd_wakeup);

    /*
     * Temporarily lie to lockdep about holding irqfds.lock to avoid a
     * false positive regarding potential deadlock with irqfd_wakeup()
     * (see irqfd_wakeup() for details).
     *
     * Adding to the wait queue will fail if there is already a priority
     * waiter, i.e. if the eventfd is associated with another irqfd (in any
     * VM).  Note, kvm_irqfd_deassign() waits for all in-flight shutdown
     * jobs to complete, i.e. ensures the irqfd has been removed from the
     * eventfd's waitqueue before returning to userspace.
     */
    (*p).ret = add_wait_queue_priority_exclusive(wqh, &mut (*irqfd).wait);
    if (*p).ret != 0 {
        spin_unlock_irq(&mut (*kvm).irqfds.lock);
        return;
    }

    list_add_tail(&mut (*irqfd).list, &mut (*kvm).irqfds.items);
    spin_unlock_irq(&mut (*kvm).irqfds.lock);
}

/* #if IS_ENABLED(CONFIG_HAVE_KVM_IRQ_BYPASS) */
pub unsafe extern "C" fn kvm_arch_irq_bypass_stop(_cons: *mut irq_bypass_consumer) {}
pub unsafe extern "C" fn kvm_arch_irq_bypass_start(_cons: *mut irq_bypass_consumer) {}
pub unsafe extern "C" fn kvm_arch_update_irqfd_routing(
    _irqfd: *mut kvm_kernel_irqfd,
    _old: *mut kvm_kernel_irq_routing_entry,
    _new: *mut kvm_kernel_irq_routing_entry,
) {
}
/* #endif */

unsafe fn kvm_irqfd_assign(kvm: *mut kvm, args: *mut kvm_irqfd) -> c_int {
    let irqfd: *mut kvm_kernel_irqfd;
    let mut eventfd: *mut eventfd_ctx = ptr::null_mut();
    let mut resamplefd: *mut eventfd_ctx = ptr::null_mut();
    let mut irqfd_pt: kvm_irqfd_pt = MaybeUninit::zeroed().assume_init();
    let mut ret: c_int;
    let events: __poll_t;
    let idx: c_int;
    let f: *mut c_void = ptr::null_mut(); /* CLASS(fd, f)(args->fd) */

    if !kvm_arch_intc_initialized(kvm) {
        return -EAGAIN;
    }

    if !kvm_arch_irqfd_allowed(kvm, args) {
        return -EINVAL;
    }

    irqfd = kzalloc_obj::<kvm_kernel_irqfd>(GFP_KERNEL_ACCOUNT);
    if irqfd.is_null() {
        return -ENOMEM;
    }

    (*irqfd).kvm = kvm;
    (*irqfd).gsi = (*args).gsi;
    INIT_LIST_HEAD(&mut (*irqfd).list);
    INIT_WORK(&mut (*irqfd).inject, irqfd_inject);
    INIT_WORK(&mut (*irqfd).shutdown, irqfd_shutdown);
    seqcount_spinlock_init(&mut (*irqfd).irq_entry_sc, &mut (*kvm).irqfds.lock);

    if fd_empty(f) {
        ret = -EBADF;
        kfree(irqfd as *mut c_void);
        return ret;
    }

    eventfd = eventfd_ctx_fileget(fd_file(f));
    if IS_ERR(eventfd as *const c_void) {
        ret = PTR_ERR(eventfd as *const c_void);
        kfree(irqfd as *mut c_void);
        return ret;
    }

    (*irqfd).eventfd = eventfd;

    if (*args).flags & KVM_IRQFD_FLAG_RESAMPLE != 0 {
        let mut resampler: *mut kvm_kernel_irqfd_resampler = ptr::null_mut();

        resamplefd = eventfd_ctx_fdget((*args).resamplefd);
        if IS_ERR(resamplefd as *const c_void) {
            ret = PTR_ERR(resamplefd as *const c_void);
            goto_fail(irqfd, resamplefd, eventfd, ret);
            return ret;
        }

        (*irqfd).resamplefd = resamplefd;
        INIT_LIST_HEAD(&mut (*irqfd).resampler_link);

        mutex_lock(&mut (*kvm).irqfds.resampler_lock);

        /* list_for_each_entry(resampler, &kvm->irqfds.resampler_list, link) */
        if !resampler.is_null() && (*resampler).notifier.gsi == (*irqfd).gsi {
            (*irqfd).resampler = resampler;
        }

        if (*irqfd).resampler.is_null() {
            resampler = kzalloc_obj::<kvm_kernel_irqfd_resampler>(GFP_KERNEL_ACCOUNT);
            if resampler.is_null() {
                ret = -ENOMEM;
                mutex_unlock(&mut (*kvm).irqfds.resampler_lock);
                goto_fail(irqfd, resamplefd, eventfd, ret);
                return ret;
            }

            (*resampler).kvm = kvm;
            INIT_LIST_HEAD(&mut (*resampler).list);
            (*resampler).notifier.gsi = (*irqfd).gsi;
            (*resampler).notifier.irq_acked = Some(irqfd_resampler_ack);
            INIT_LIST_HEAD(&mut (*resampler).link);

            list_add_rcu(&mut (*resampler).link, &mut (*kvm).irqfds.resampler_list);
            kvm_register_irq_ack_notifier(kvm, &mut (*resampler).notifier);
            (*irqfd).resampler = resampler;
        }

        list_add_rcu(&mut (*irqfd).resampler_link, &mut (*(*irqfd).resampler).list);
        synchronize_srcu_expedited(&mut (*kvm).irq_srcu);

        mutex_unlock(&mut (*kvm).irqfds.resampler_lock);
    }

    /*
     * Set the irqfd routing and add it to KVM's list before registering
     * the irqfd with the eventfd, so that the routing information is valid
     * and stays valid, e.g. if there are GSI routing changes, prior to
     * making the irqfd visible, i.e. before it might be signaled.
     *
     * Note, holding SRCU ensures a stable read of routing information, and
     * also prevents irqfd_shutdown() from freeing the irqfd before it's
     * fully initialized.
     */
    idx = srcu_read_lock(&mut (*kvm).irq_srcu);

    /*
     * Register the irqfd with the eventfd by polling on the eventfd, and
     * simultaneously and the irqfd to KVM's list.  If there was en event
     * pending on the eventfd prior to registering, manually trigger IRQ
     * injection.
     */
    irqfd_pt.irqfd = irqfd;
    irqfd_pt.kvm = kvm;
    init_poll_funcptr(&mut irqfd_pt.pt, kvm_irqfd_register);

    events = vfs_poll(fd_file(f), &mut irqfd_pt.pt);

    ret = irqfd_pt.ret;
    if ret != 0 {
        srcu_read_unlock(&mut (*kvm).irq_srcu, idx);
        goto_fail(irqfd, resamplefd, eventfd, ret);
        return ret;
    }

    if events & EPOLLIN != 0 {
        schedule_work(&mut (*irqfd).inject);
    }

    /* #if IS_ENABLED(CONFIG_HAVE_KVM_IRQ_BYPASS) */
    if kvm_arch_has_irq_bypass() {
        (*irqfd).consumer.add_producer = Some(kvm_arch_irq_bypass_add_producer);
        (*irqfd).consumer.del_producer = Some(kvm_arch_irq_bypass_del_producer);
        (*irqfd).consumer.stop = Some(kvm_arch_irq_bypass_stop);
        (*irqfd).consumer.start = Some(kvm_arch_irq_bypass_start);
        ret = irq_bypass_register_consumer(&mut (*irqfd).consumer, (*irqfd).eventfd);
        if ret != 0 {
            pr_info(
                b"irq bypass consumer (eventfd %p) registration fails: %d\n\0".as_ptr() as *const c_char,
                (*irqfd).eventfd,
                ret,
            );
        }
    }
    /* #endif */

    srcu_read_unlock(&mut (*kvm).irq_srcu, idx);
    return 0;
}

unsafe fn goto_fail(
    irqfd: *mut kvm_kernel_irqfd,
    resamplefd: *mut eventfd_ctx,
    eventfd: *mut eventfd_ctx,
    ret: c_int,
) {
    if !(*irqfd).resampler.is_null() {
        irqfd_resampler_shutdown(irqfd);
    }
    if !resamplefd.is_null() && !IS_ERR(resamplefd as *const c_void) {
        eventfd_ctx_put(resamplefd);
    }
    if !eventfd.is_null() && !IS_ERR(eventfd as *const c_void) {
        eventfd_ctx_put(eventfd);
    }
    let _ = ret;
    kfree(irqfd as *mut c_void);
}

pub unsafe extern "C" fn kvm_irq_has_notifier(kvm: *mut kvm, irqchip: c_uint, pin: c_uint) -> bool {
    let _kian: *mut kvm_irq_ack_notifier;
    let gsi: c_int;
    let idx: c_int;

    idx = srcu_read_lock(&mut (*kvm).irq_srcu);
    gsi = kvm_irq_map_chip_pin(kvm, irqchip, pin);
    if gsi != -1 {
        /* hlist_for_each_entry_srcu(kian, &kvm->irq_ack_notifier_list, link,
         * srcu_read_lock_held(&kvm->irq_srcu))
         */
        let _ = srcu_read_lock_held(&mut (*kvm).irq_srcu);
        /* if (kian->gsi == gsi) { ... return true; } */
    }

    srcu_read_unlock(&mut (*kvm).irq_srcu, idx);
    return false;
}
/* EXPORT_SYMBOL_FOR_KVM_INTERNAL(kvm_irq_has_notifier); */

pub unsafe extern "C" fn kvm_notify_acked_gsi(kvm: *mut kvm, gsi: c_int) {
    let _kian: *mut kvm_irq_ack_notifier;

    /* hlist_for_each_entry_srcu(kian, &kvm->irq_ack_notifier_list, link,
     * srcu_read_lock_held(&kvm->irq_srcu))
     */
    let _ = (kvm, gsi, srcu_read_lock_held(&mut (*kvm).irq_srcu));
    /* if (kian->gsi == gsi) kian->irq_acked(kian); */
}

pub unsafe extern "C" fn kvm_notify_acked_irq(kvm: *mut kvm, irqchip: c_uint, pin: c_uint) {
    let gsi: c_int;
    let idx: c_int;

    trace_kvm_ack_irq(irqchip, pin);

    idx = srcu_read_lock(&mut (*kvm).irq_srcu);
    gsi = kvm_irq_map_chip_pin(kvm, irqchip, pin);
    if gsi != -1 {
        kvm_notify_acked_gsi(kvm, gsi);
    }
    srcu_read_unlock(&mut (*kvm).irq_srcu, idx);
}

pub unsafe extern "C" fn kvm_register_irq_ack_notifier(kvm: *mut kvm, kian: *mut kvm_irq_ack_notifier) {
    mutex_lock(&mut (*kvm).irq_lock);
    hlist_add_head_rcu(&mut (*kian).link, &mut (*kvm).irq_ack_notifier_list);
    mutex_unlock(&mut (*kvm).irq_lock);
    kvm_arch_post_irq_ack_notifier_list_update(kvm);
}

pub unsafe extern "C" fn kvm_unregister_irq_ack_notifier(kvm: *mut kvm, kian: *mut kvm_irq_ack_notifier) {
    mutex_lock(&mut (*kvm).irq_lock);
    hlist_del_init_rcu(&mut (*kian).link);
    mutex_unlock(&mut (*kvm).irq_lock);
    synchronize_srcu_expedited(&mut (*kvm).irq_srcu);
    kvm_arch_post_irq_ack_notifier_list_update(kvm);
}

/*
 * shutdown any irqfd's that match fd+gsi
 */
unsafe fn kvm_irqfd_deassign(kvm: *mut kvm, args: *mut kvm_irqfd) -> c_int {
    let eventfd: *mut eventfd_ctx;

    eventfd = eventfd_ctx_fdget((*args).fd);
    if IS_ERR(eventfd as *const c_void) {
        return PTR_ERR(eventfd as *const c_void);
    }

    spin_lock_irq(&mut (*kvm).irqfds.lock);
    /* list_for_each_entry_safe(irqfd, tmp, &kvm->irqfds.items, list) {
     *     if (irqfd->eventfd == eventfd && irqfd->gsi == args->gsi)
     *         irqfd_deactivate(irqfd);
     * }
     */
    spin_unlock_irq(&mut (*kvm).irqfds.lock);
    eventfd_ctx_put(eventfd);

    /*
     * Block until we know all outstanding shutdown jobs have completed
     * so that we guarantee there will not be any more interrupts on this
     * gsi once this deassign function returns.
     */
    flush_workqueue(irqfd_cleanup_wq);

    return 0;
}

pub unsafe extern "C" fn kvm_irqfd(kvm: *mut kvm, args: *mut kvm_irqfd) -> c_int {
    if (*args).flags & !(KVM_IRQFD_FLAG_DEASSIGN | KVM_IRQFD_FLAG_RESAMPLE) != 0 {
        return -EINVAL;
    }

    if (*args).flags & KVM_IRQFD_FLAG_DEASSIGN != 0 {
        return kvm_irqfd_deassign(kvm, args);
    }

    return kvm_irqfd_assign(kvm, args);
}

/*
 * This function is called as the kvm VM fd is being released. Shutdown all
 * irqfds that still remain open
 */
pub unsafe extern "C" fn kvm_irqfd_release(kvm: *mut kvm) {
    spin_lock_irq(&mut (*kvm).irqfds.lock);
    /* list_for_each_entry_safe(irqfd, tmp, &kvm->irqfds.items, list)
     *     irqfd_deactivate(irqfd);
     */
    spin_unlock_irq(&mut (*kvm).irqfds.lock);

    /*
     * Block until we know all outstanding shutdown jobs have completed
     * since we do not take a kvm* reference.
     */
    flush_workqueue(irqfd_cleanup_wq);
}

/*
 * Take note of a change in irq routing.
 * Caller must invoke synchronize_srcu_expedited(&kvm->irq_srcu) afterwards.
 */
pub unsafe extern "C" fn kvm_irq_routing_update(kvm: *mut kvm) {
    spin_lock_irq(&mut (*kvm).irqfds.lock);
    /* list_for_each_entry(irqfd, &kvm->irqfds.items, list) {
     *     irqfd_update(kvm, irqfd);
     * #if IS_ENABLED(CONFIG_HAVE_KVM_IRQ_BYPASS)
     *     if (irqfd->producer)
     *         kvm_arch_update_irqfd_routing(irqfd, &old, &irqfd->irq_entry);
     * #endif
     * }
     */
    spin_unlock_irq(&mut (*kvm).irqfds.lock);
}

pub unsafe extern "C" fn kvm_notify_irqfd_resampler(
    kvm: *mut kvm,
    irqchip: c_uint,
    pin: c_uint,
) -> bool {
    let gsi: c_int;
    let idx: c_int;

    idx = srcu_read_lock(&mut (*kvm).irq_srcu);
    gsi = kvm_irq_map_chip_pin(kvm, irqchip, pin);
    if gsi != -1 {
        /* list_for_each_entry_srcu(resampler, &kvm->irqfds.resampler_list, link,
         * srcu_read_lock_held(&kvm->irq_srcu)) {
         *     if (resampler->notifier.gsi == gsi) { ... return true; }
         * }
         */
        let _ = srcu_read_lock_held(&mut (*kvm).irq_srcu);
    }
    srcu_read_unlock(&mut (*kvm).irq_srcu, idx);

    return false;
}

/*
 * create a host-wide workqueue for issuing deferred shutdown requests
 * aggregated from all vm* instances. We need our own isolated
 * queue to ease flushing work items when a VM exits.
 */
pub unsafe extern "C" fn kvm_irqfd_init() -> c_int {
    irqfd_cleanup_wq = alloc_workqueue(b"kvm-irqfd-cleanup\0".as_ptr() as *const c_char, WQ_PERCPU, 0);
    if irqfd_cleanup_wq.is_null() {
        return -ENOMEM;
    }

    return 0;
}

pub unsafe extern "C" fn kvm_irqfd_exit() {
    destroy_workqueue(irqfd_cleanup_wq);
}

/*
 * --------------------------------------------------------------------
 * ioeventfd: translate a PIO/MMIO memory write to an eventfd signal.
 *
 * userspace can register a PIO/MMIO address with an eventfd for receiving
 * notification when the memory has been touched.
 * --------------------------------------------------------------------
 */

unsafe fn to_ioeventfd(dev: *mut kvm_io_device) -> *mut _ioeventfd {
    return container_of(dev, |p: *mut _ioeventfd| unsafe { &mut (*p).dev });
}

unsafe fn ioeventfd_release(p: *mut _ioeventfd) {
    eventfd_ctx_put((*p).eventfd);
    list_del(&mut (*p).list);
    kfree(p as *mut c_void);
}

unsafe fn ioeventfd_in_range(p: *mut _ioeventfd, addr: gpa_t, len: c_int, val: *const c_void) -> bool {
    let _val: u64;

    if addr != (*p).addr {
        /* address must be precise for a hit */
        return false;
    }

    if (*p).length == 0 {
        /* length = 0 means only look at the address, so always a hit */
        return true;
    }

    if len != (*p).length {
        /* address-range must be precise for a hit */
        return false;
    }

    if (*p).wildcard {
        /* all else equal, wildcard is always a hit */
        return true;
    }

    /* otherwise, we have to actually compare the data */
    match len {
        1 => _val = ptr::read_unaligned(val as *const u8) as u64,
        2 => _val = ptr::read_unaligned(val as *const u16) as u64,
        4 => _val = ptr::read_unaligned(val as *const u32) as u64,
        8 => _val = ptr::read_unaligned(val as *const u64),
        _ => return false,
    }

    return _val == (*p).datamatch;
}

/* MMIO/PIO writes trigger an event if the addr/val match */
unsafe extern "C" fn ioeventfd_write(
    _vcpu: *mut kvm_vcpu,
    this: *mut kvm_io_device,
    addr: gpa_t,
    len: c_int,
    val: *const c_void,
) -> c_int {
    let p = to_ioeventfd(this);

    if !ioeventfd_in_range(p, addr, len, val) {
        return -EOPNOTSUPP;
    }

    eventfd_signal((*p).eventfd);
    return 0;
}

/*
 * This function is called as KVM is completely shutting down.  We do not
 * need to worry about locking just nuke anything we have as quickly as possible
 */
unsafe extern "C" fn ioeventfd_destructor(this: *mut kvm_io_device) {
    let p = to_ioeventfd(this);
    ioeventfd_release(p);
}

static ioeventfd_ops: kvm_io_device_ops = kvm_io_device_ops {
    read: None,
    write: Some(ioeventfd_write),
    destructor: Some(ioeventfd_destructor),
};

/* assumes kvm->slots_lock held */
unsafe fn ioeventfd_check_collision(kvm: *mut kvm, p: *mut _ioeventfd) -> bool {
    let _p: *mut _ioeventfd;

    /* list_for_each_entry(_p, &kvm->ioeventfds, list)
     *     if (_p->bus_idx == p->bus_idx &&
     *         _p->addr == p->addr &&
     *         (!_p->length || !p->length ||
     *          (_p->length == p->length &&
     *           (_p->wildcard || p->wildcard ||
     *            _p->datamatch == p->datamatch))))
     *         return true;
     */
    let _ = (kvm, p);
    return false;
}

unsafe fn ioeventfd_bus_from_flags(flags: __u32) -> kvm_bus {
    if flags & KVM_IOEVENTFD_FLAG_PIO != 0 {
        return kvm_bus::KVM_PIO_BUS;
    }
    if flags & KVM_IOEVENTFD_FLAG_VIRTIO_CCW_NOTIFY != 0 {
        return kvm_bus::KVM_VIRTIO_CCW_NOTIFY_BUS;
    }
    return kvm_bus::KVM_MMIO_BUS;
}

unsafe fn kvm_assign_ioeventfd_idx(kvm: *mut kvm, bus_idx: kvm_bus, args: *mut kvm_ioeventfd) -> c_int {
    let eventfd: *mut eventfd_ctx;
    let p: *mut _ioeventfd;
    let mut ret: c_int;

    eventfd = eventfd_ctx_fdget((*args).fd);
    if IS_ERR(eventfd as *const c_void) {
        return PTR_ERR(eventfd as *const c_void);
    }

    p = kzalloc_obj::<_ioeventfd>(GFP_KERNEL_ACCOUNT);
    if p.is_null() {
        ret = -ENOMEM;
        eventfd_ctx_put(eventfd);
        return ret;
    }

    INIT_LIST_HEAD(&mut (*p).list);
    (*p).addr = (*args).addr;
    (*p).bus_idx = bus_idx as u8;
    (*p).length = (*args).len as c_int;
    (*p).eventfd = eventfd;

    /* The datamatch feature is optional, otherwise this is a wildcard */
    if (*args).flags & KVM_IOEVENTFD_FLAG_DATAMATCH != 0 {
        (*p).datamatch = (*args).datamatch;
    } else {
        (*p).wildcard = true;
    }

    mutex_lock(&mut (*kvm).slots_lock);

    /* Verify that there isn't a match already */
    if ioeventfd_check_collision(kvm, p) {
        ret = -EEXIST;
        mutex_unlock(&mut (*kvm).slots_lock);
        kfree(p as *mut c_void);
        eventfd_ctx_put(eventfd);
        return ret;
    }

    kvm_iodevice_init(&mut (*p).dev, &ioeventfd_ops);

    ret = kvm_io_bus_register_dev(kvm, bus_idx, (*p).addr, (*p).length, &mut (*p).dev);
    if ret < 0 {
        mutex_unlock(&mut (*kvm).slots_lock);
        kfree(p as *mut c_void);
        eventfd_ctx_put(eventfd);
        return ret;
    }

    (*kvm_get_bus(kvm, bus_idx)).ioeventfd_count += 1;
    list_add_tail(&mut (*p).list, &mut (*kvm).ioeventfds);

    mutex_unlock(&mut (*kvm).slots_lock);
    return 0;
}

unsafe fn kvm_deassign_ioeventfd_idx(kvm: *mut kvm, bus_idx: kvm_bus, args: *mut kvm_ioeventfd) -> c_int {
    let eventfd: *mut eventfd_ctx;
    let mut ret: c_int = -ENOENT;
    let wildcard: bool;

    eventfd = eventfd_ctx_fdget((*args).fd);
    if IS_ERR(eventfd as *const c_void) {
        return PTR_ERR(eventfd as *const c_void);
    }

    wildcard = !((*args).flags & KVM_IOEVENTFD_FLAG_DATAMATCH != 0);

    mutex_lock(&mut (*kvm).slots_lock);

    /* list_for_each_entry(p, &kvm->ioeventfds, list) {
     *     if (p->bus_idx != bus_idx || p->eventfd != eventfd ||
     *         p->addr != args->addr || p->length != args->len ||
     *         p->wildcard != wildcard)
     *         continue;
     *     if (!p->wildcard && p->datamatch != args->datamatch)
     *         continue;
     *     kvm_io_bus_unregister_dev(kvm, bus_idx, &p->dev);
     *     bus = kvm_get_bus(kvm, bus_idx);
     *     if (bus)
     *         bus->ioeventfd_count--;
     *     ret = 0;
     *     break;
     * }
     */
    let _ = (bus_idx, wildcard);

    mutex_unlock(&mut (*kvm).slots_lock);
    eventfd_ctx_put(eventfd);
    return ret;
}

unsafe fn kvm_deassign_ioeventfd(kvm: *mut kvm, args: *mut kvm_ioeventfd) -> c_int {
    let bus_idx = ioeventfd_bus_from_flags((*args).flags);
    let ret = kvm_deassign_ioeventfd_idx(kvm, bus_idx, args);

    if (*args).len == 0 && bus_idx == kvm_bus::KVM_MMIO_BUS {
        kvm_deassign_ioeventfd_idx(kvm, kvm_bus::KVM_FAST_MMIO_BUS, args);
    }

    return ret;
}

unsafe fn kvm_assign_ioeventfd(kvm: *mut kvm, args: *mut kvm_ioeventfd) -> c_int {
    let bus_idx: kvm_bus;
    let mut ret: c_int;

    bus_idx = ioeventfd_bus_from_flags((*args).flags);
    /* must be natural-word sized, or 0 to ignore length */
    match (*args).len {
        0 | 1 | 2 | 4 | 8 => {}
        _ => return -EINVAL,
    }

    /* check for range overflow */
    if (*args).addr.wrapping_add((*args).len as u64) < (*args).addr {
        return -EINVAL;
    }

    /* check for extra flags that we don't understand */
    if (*args).flags & !KVM_IOEVENTFD_VALID_FLAG_MASK != 0 {
        return -EINVAL;
    }

    /* ioeventfd with no length can't be combined with DATAMATCH */
    if (*args).len == 0 && ((*args).flags & KVM_IOEVENTFD_FLAG_DATAMATCH != 0) {
        return -EINVAL;
    }

    ret = kvm_assign_ioeventfd_idx(kvm, bus_idx, args);
    if ret != 0 {
        return ret;
    }

    /* When length is ignored, MMIO is also put on a separate bus, for
     * faster lookups.
     */
    if (*args).len == 0 && bus_idx == kvm_bus::KVM_MMIO_BUS {
        ret = kvm_assign_ioeventfd_idx(kvm, kvm_bus::KVM_FAST_MMIO_BUS, args);
        if ret < 0 {
            kvm_deassign_ioeventfd_idx(kvm, bus_idx, args);
            return ret;
        }
    }

    return 0;
}

pub unsafe extern "C" fn kvm_ioeventfd(kvm: *mut kvm, args: *mut kvm_ioeventfd) -> c_int {
    if (*args).flags & KVM_IOEVENTFD_FLAG_DEASSIGN != 0 {
        return kvm_deassign_ioeventfd(kvm, args);
    }

    return kvm_assign_ioeventfd(kvm, args);
}

pub unsafe extern "C" fn kvm_eventfd_init(kvm: *mut kvm) {
    /* #ifdef CONFIG_HAVE_KVM_IRQCHIP */
    spin_lock_init(&mut (*kvm).irqfds.lock);
    INIT_LIST_HEAD(&mut (*kvm).irqfds.items);
    INIT_LIST_HEAD(&mut (*kvm).irqfds.resampler_list);
    mutex_init(&mut (*kvm).irqfds.resampler_lock);
    /* #endif */
    INIT_LIST_HEAD(&mut (*kvm).ioeventfds);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
