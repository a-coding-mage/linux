// SPDX-License-Identifier: GPL-2.0
/*
 * KVM coalesced MMIO
 *
 * Copyright (c) 2008 Bull S.A.S.
 * Copyright 2009 Red Hat, Inc. and/or its affiliates.
 *
 *  Author: Laurent Vivier <Laurent.Vivier@bull.net>
 *
 */

// Dependencies from:
// <kvm/iodev.h>
// <linux/kvm_host.h>
// <linux/slab.h>
// <linux/kvm.h>
// "coalesced_mmio.h"

use core::ffi::{c_int, c_ulong, c_void};
use core::mem::MaybeUninit;
use core::ptr;

type gpa_t = u64;
type __u32 = u32;

const EOPNOTSUPP: c_int = 95;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;

const GFP_KERNEL_ACCOUNT: c_ulong = 0;
const __GFP_ZERO: c_ulong = 0;
const KVM_COALESCED_MMIO_MAX: __u32 = 0;
const KVM_PIO_BUS: c_int = 0;
const KVM_MMIO_BUS: c_int = 0;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kvm_io_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kvm_vcpu {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kvm_io_device_ops {
    pub write: Option<
        unsafe extern "C" fn(
            vcpu: *mut kvm_vcpu,
            this: *mut kvm_io_device,
            addr: gpa_t,
            len: c_int,
            val: *const c_void,
        ) -> c_int,
    >,
    pub destructor: Option<unsafe extern "C" fn(this: *mut kvm_io_device)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_coalesced_mmio_zone {
    pub addr: gpa_t,
    pub size: u32,
    pub pio: u32,
}

#[repr(C)]
pub struct kvm_coalesced_mmio {
    pub phys_addr: gpa_t,
    pub len: c_int,
    pub data: [u8; 8],
    pub pio: u32,
}

#[repr(C)]
pub struct kvm_coalesced_mmio_ring {
    pub first: __u32,
    pub last: __u32,
    pub coalesced_mmio: [kvm_coalesced_mmio; KVM_COALESCED_MMIO_MAX as usize],
}

#[repr(C)]
pub struct kvm {
    pub coalesced_mmio_ring: *mut kvm_coalesced_mmio_ring,
    pub ring_lock: spinlock_t,
    pub coalesced_zones: list_head,
    pub slots_lock: mutex,
}

#[repr(C)]
pub struct kvm_coalesced_mmio_dev {
    pub dev: kvm_io_device,
    pub kvm: *mut kvm,
    pub zone: kvm_coalesced_mmio_zone,
    pub list: list_head,
}

unsafe extern "C" {
    fn alloc_page(flags: c_ulong) -> *mut page;
    fn page_address(page: *mut page) -> *mut c_void;
    fn free_page(addr: c_ulong);
    fn kfree(ptr: *const c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn list_del(entry: *mut list_head);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn kvm_iodevice_init(dev: *mut kvm_io_device, ops: *const kvm_io_device_ops);
    fn kvm_io_bus_register_dev(
        kvm: *mut kvm,
        bus_idx: c_int,
        addr: gpa_t,
        len: u32,
        dev: *mut kvm_io_device,
    ) -> c_int;
    fn kvm_io_bus_unregister_dev(
        kvm: *mut kvm,
        bus_idx: c_int,
        dev: *mut kvm_io_device,
    ) -> c_int;
    fn kzalloc(size: usize, flags: c_ulong) -> *mut c_void;
}

#[inline]
unsafe fn READ_ONCE<T: Copy>(p: *const T) -> T {
    ptr::read_volatile(p)
}

#[inline]
unsafe fn smp_wmb() {
    core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
}

#[inline]
unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    (*list).next = list;
    (*list).prev = list;
}

#[inline]
unsafe fn kzalloc_obj<T>(flags: c_ulong) -> *mut T {
    kzalloc(core::mem::size_of::<T>(), flags) as *mut T
}

#[inline]
unsafe fn container_of_kvm_coalesced_mmio_dev_dev(
    ptr: *mut kvm_io_device,
) -> *mut kvm_coalesced_mmio_dev {
    let uninit = MaybeUninit::<kvm_coalesced_mmio_dev>::uninit();
    let base = uninit.as_ptr();
    let offset = ptr::addr_of!((*base).dev) as usize - base as usize;
    (ptr as *mut u8).sub(offset) as *mut kvm_coalesced_mmio_dev
}

#[inline]
unsafe fn list_entry_kvm_coalesced_mmio_dev_list(
    ptr: *mut list_head,
) -> *mut kvm_coalesced_mmio_dev {
    let uninit = MaybeUninit::<kvm_coalesced_mmio_dev>::uninit();
    let base = uninit.as_ptr();
    let offset = ptr::addr_of!((*base).list) as usize - base as usize;
    (ptr as *mut u8).sub(offset) as *mut kvm_coalesced_mmio_dev
}

#[inline]
unsafe fn to_mmio(dev: *mut kvm_io_device) -> *mut kvm_coalesced_mmio_dev {
    container_of_kvm_coalesced_mmio_dev_dev(dev)
}

unsafe fn coalesced_mmio_in_range(
    dev: *mut kvm_coalesced_mmio_dev,
    addr: gpa_t,
    len: c_int,
) -> c_int {
    /* is it in a batchable area ?
     * (addr,len) is fully included in
     * (zone->addr, zone->size)
     */
    if len < 0 {
        return 0;
    }
    if addr.wrapping_add(len as gpa_t) < addr {
        return 0;
    }
    if addr < (*dev).zone.addr {
        return 0;
    }
    if addr.wrapping_add(len as gpa_t) > (*dev).zone.addr.wrapping_add((*dev).zone.size as gpa_t) {
        return 0;
    }
    1
}

unsafe extern "C" fn coalesced_mmio_write(
    _vcpu: *mut kvm_vcpu,
    this: *mut kvm_io_device,
    addr: gpa_t,
    len: c_int,
    val: *const c_void,
) -> c_int {
    let dev = to_mmio(this);
    let ring = (*(*dev).kvm).coalesced_mmio_ring;
    let insert: __u32;

    if coalesced_mmio_in_range(dev, addr, len) == 0 {
        return -EOPNOTSUPP;
    }

    spin_lock(ptr::addr_of_mut!((*(*dev).kvm).ring_lock));

    /*
     * last is the index of the entry to fill.  Verify userspace hasn't
     * set last to be out of range, and that there is room in the ring.
     * Leave one entry free in the ring so that userspace can differentiate
     * between an empty ring and a full ring.
     */
    insert = READ_ONCE(ptr::addr_of!((*ring).last));
    if insert >= KVM_COALESCED_MMIO_MAX
        || (insert + 1) % KVM_COALESCED_MMIO_MAX == READ_ONCE(ptr::addr_of!((*ring).first))
    {
        spin_unlock(ptr::addr_of_mut!((*(*dev).kvm).ring_lock));
        return -EOPNOTSUPP;
    }

    /* copy data in first free entry of the ring */

    (*ring).coalesced_mmio[insert as usize].phys_addr = addr;
    (*ring).coalesced_mmio[insert as usize].len = len;
    memcpy(
        (*ring).coalesced_mmio[insert as usize].data.as_mut_ptr() as *mut c_void,
        val,
        len as usize,
    );
    (*ring).coalesced_mmio[insert as usize].pio = (*dev).zone.pio;
    smp_wmb();
    (*ring).last = (insert + 1) % KVM_COALESCED_MMIO_MAX;
    spin_unlock(ptr::addr_of_mut!((*(*dev).kvm).ring_lock));
    0
}

unsafe extern "C" fn coalesced_mmio_destructor(this: *mut kvm_io_device) {
    let dev = to_mmio(this);

    list_del(ptr::addr_of_mut!((*dev).list));

    kfree(dev as *const c_void);
}

static coalesced_mmio_ops: kvm_io_device_ops = kvm_io_device_ops {
    write: Some(coalesced_mmio_write),
    destructor: Some(coalesced_mmio_destructor),
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kvm_coalesced_mmio_init(kvm: *mut kvm) -> c_int {
    let page: *mut page;

    page = alloc_page(GFP_KERNEL_ACCOUNT | __GFP_ZERO);
    if page.is_null() {
        return -ENOMEM;
    }

    (*kvm).coalesced_mmio_ring = page_address(page) as *mut kvm_coalesced_mmio_ring;

    /*
     * We're using this spinlock to sync access to the coalesced ring.
     * The list doesn't need its own lock since device registration and
     * unregistration should only happen when kvm->slots_lock is held.
     */
    spin_lock_init(ptr::addr_of_mut!((*kvm).ring_lock));
    INIT_LIST_HEAD(ptr::addr_of_mut!((*kvm).coalesced_zones));

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kvm_coalesced_mmio_free(kvm: *mut kvm) {
    if !(*kvm).coalesced_mmio_ring.is_null() {
        free_page((*kvm).coalesced_mmio_ring as c_ulong);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kvm_vm_ioctl_register_coalesced_mmio(
    kvm: *mut kvm,
    zone: *mut kvm_coalesced_mmio_zone,
) -> c_int {
    let ret: c_int;
    let dev: *mut kvm_coalesced_mmio_dev;

    if (*zone).pio != 1 && (*zone).pio != 0 {
        return -EINVAL;
    }

    dev = kzalloc_obj::<kvm_coalesced_mmio_dev>(GFP_KERNEL_ACCOUNT);
    if dev.is_null() {
        return -ENOMEM;
    }

    kvm_iodevice_init(ptr::addr_of_mut!((*dev).dev), ptr::addr_of!(coalesced_mmio_ops));
    (*dev).kvm = kvm;
    (*dev).zone = *zone;

    mutex_lock(ptr::addr_of_mut!((*kvm).slots_lock));
    ret = kvm_io_bus_register_dev(
        kvm,
        if (*zone).pio != 0 { KVM_PIO_BUS } else { KVM_MMIO_BUS },
        (*zone).addr,
        (*zone).size,
        ptr::addr_of_mut!((*dev).dev),
    );
    if ret < 0 {
        mutex_unlock(ptr::addr_of_mut!((*kvm).slots_lock));
        kfree(dev as *const c_void);

        return ret;
    }
    list_add_tail(
        ptr::addr_of_mut!((*dev).list),
        ptr::addr_of_mut!((*kvm).coalesced_zones),
    );
    mutex_unlock(ptr::addr_of_mut!((*kvm).slots_lock));

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kvm_vm_ioctl_unregister_coalesced_mmio(
    kvm: *mut kvm,
    zone: *mut kvm_coalesced_mmio_zone,
) -> c_int {
    let mut dev: *mut kvm_coalesced_mmio_dev;
    let mut tmp: *mut kvm_coalesced_mmio_dev;
    let r: c_int;

    if (*zone).pio != 1 && (*zone).pio != 0 {
        return -EINVAL;
    }

    mutex_lock(ptr::addr_of_mut!((*kvm).slots_lock));

    dev = list_entry_kvm_coalesced_mmio_dev_list((*kvm).coalesced_zones.next);
    tmp = list_entry_kvm_coalesced_mmio_dev_list((*dev).list.next);
    while ptr::addr_of_mut!((*dev).list) != ptr::addr_of_mut!((*kvm).coalesced_zones) {
        if (*zone).pio == (*dev).zone.pio
            && coalesced_mmio_in_range(dev, (*zone).addr, (*zone).size as c_int) != 0
        {
            r = kvm_io_bus_unregister_dev(
                kvm,
                if (*zone).pio != 0 { KVM_PIO_BUS } else { KVM_MMIO_BUS },
                ptr::addr_of_mut!((*dev).dev),
            );
            /*
             * On failure, unregister destroys all devices on the
             * bus, including the target device. There's no need
             * to restart the walk as there aren't any zones left.
             */
            if r != 0 {
                break;
            }
        }

        dev = tmp;
        tmp = list_entry_kvm_coalesced_mmio_dev_list((*tmp).list.next);
    }

    mutex_unlock(ptr::addr_of_mut!((*kvm).slots_lock));

    /*
     * Ignore the result of kvm_io_bus_unregister_dev(), from userspace's
     * perspective, the coalesced MMIO is most definitely unregistered.
     */
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
