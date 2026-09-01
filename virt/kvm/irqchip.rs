// SPDX-License-Identifier: GPL-2.0-only
/*
 * irqchip.c: Common API for in kernel interrupt controllers
 * Copyright (c) 2007, Intel Corporation.
 * Copyright 2010 Red Hat, Inc. and/or its affiliates.
 * Copyright (c) 2013, Alexander Graf <agraf@suse.de>
 *
 * This file is derived from virt/kvm/irq_comm.c.
 *
 * Authors:
 *   Yaozu (Eddie) Dong <Eddie.dong@intel.com>
 *   Alexander Graf <agraf@suse.de>
 */

use core::ffi::{c_int, c_uint, c_void};

pub type u32 = core::ffi::c_uint;

pub const EINVAL: c_int = 22;
pub const ENOMEM: c_int = 12;
pub const KVM_IRQ_ROUTING_IRQCHIP: u32 = 1;
pub const KVM_IRQ_ROUTING_MSI: u32 = 2;

extern "C" {
    pub static KVM_NR_IRQCHIPS: usize;
    pub static KVM_IRQCHIP_NUM_PINS: usize;
    pub static KVM_MAX_IRQ_ROUTES: u32;
    pub static KVM_MSI_VALID_DEVID: u32;
    pub static KVM_USERSPACE_IRQ_SOURCE_ID: c_int;
    pub static GFP_KERNEL_ACCOUNT: c_uint;

    fn lockdep_is_held(lock: *mut mutex) -> bool;
    fn trace_kvm_set_irq(irq: u32, level: c_int, irq_source_id: c_int);
    fn srcu_read_lock(srcu: *mut srcu_struct) -> c_int;
    fn srcu_read_unlock(srcu: *mut srcu_struct, idx: c_int);
    fn synchronize_srcu_expedited(srcu: *mut srcu_struct);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn kfree(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn kvm_arch_irqchip_in_kernel(kvm: *mut kvm) -> bool;
    fn kvm_set_msi(
        route: *mut kvm_kernel_irq_routing_entry,
        kvm: *mut kvm,
        irq_source_id: c_int,
        level: c_int,
        line_status: bool,
    ) -> c_int;
    fn kvm_set_routing_entry(
        kvm: *mut kvm,
        e: *mut kvm_kernel_irq_routing_entry,
        ue: *const kvm_irq_routing_entry,
    ) -> c_int;
    fn kvm_irq_routing_update(kvm: *mut kvm);
    fn kzalloc_flex_irq_routing_table(nr_rt_entries: u32, flags: c_uint)
        -> *mut kvm_irq_routing_table;
    fn kzalloc_kernel_irq_routing_entry(flags: c_uint) -> *mut kvm_kernel_irq_routing_entry;
    fn srcu_dereference_check_irq_routing(
        ptr: *mut kvm_irq_routing_table,
        srcu: *mut srcu_struct,
        check: bool,
    ) -> *mut kvm_irq_routing_table;
    fn srcu_dereference_irq_routing(
        ptr: *mut kvm_irq_routing_table,
        srcu: *mut srcu_struct,
    ) -> *mut kvm_irq_routing_table;
    fn rcu_access_pointer_irq_routing(ptr: *mut kvm_irq_routing_table)
        -> *mut kvm_irq_routing_table;
    fn rcu_dereference_protected_irq_routing(
        ptr: *mut kvm_irq_routing_table,
        check: c_int,
    ) -> *mut kvm_irq_routing_table;
    fn rcu_assign_pointer_irq_routing(
        ptr: *mut *mut kvm_irq_routing_table,
        value: *mut kvm_irq_routing_table,
    );
    fn rcu_init_pointer_irq_routing(
        ptr: *mut *mut kvm_irq_routing_table,
        value: *mut kvm_irq_routing_table,
    );
    fn array_index_nospec(index: u32, size: u32) -> u32;
    fn hlist_add_head(node: *mut hlist_node, head: *mut hlist_head);
    fn hlist_del(node: *mut hlist_node);
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct srcu_struct {
    _private: [u8; 0],
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
pub struct kvm {
    pub irq_routing: *mut kvm_irq_routing_table,
    pub irq_srcu: srcu_struct,
    pub irq_lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_kernel_irq_routing_entry_irqchip {
    pub irqchip: c_uint,
    pub pin: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_kernel_irq_routing_entry_msi {
    pub address_lo: u32,
    pub address_hi: u32,
    pub data: u32,
    pub flags: u32,
    pub devid: u32,
}

pub type kvm_irq_set_fn = unsafe extern "C" fn(
    *mut kvm_kernel_irq_routing_entry,
    *mut kvm,
    c_int,
    c_int,
    bool,
) -> c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_kernel_irq_routing_entry {
    pub link: hlist_node,
    pub gsi: u32,
    pub type_: u32,
    pub set: kvm_irq_set_fn,
    pub irqchip: kvm_kernel_irq_routing_entry_irqchip,
    pub msi: kvm_kernel_irq_routing_entry_msi,
}

#[repr(C)]
pub struct kvm_irq_routing_table {
    pub nr_rt_entries: u32,
    pub chip: [[c_int; 0]; 0],
    pub map: [hlist_head; 0],
}

#[repr(C)]
pub struct kvm_msi {
    pub address_lo: u32,
    pub address_hi: u32,
    pub data: u32,
    pub flags: u32,
    pub devid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_irq_routing_irqchip {
    pub irqchip: c_uint,
    pub pin: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_irq_routing_entry_u {
    pub irqchip: kvm_irq_routing_irqchip,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_irq_routing_entry {
    pub gsi: u32,
    pub type_: u32,
    pub flags: u32,
    pub u: kvm_irq_routing_entry_u,
}

unsafe fn hlist_entry_from_link(
    node: *mut hlist_node,
) -> *mut kvm_kernel_irq_routing_entry {
    (node as *mut u8).sub(core::mem::offset_of!(kvm_kernel_irq_routing_entry, link))
        as *mut kvm_kernel_irq_routing_entry
}

unsafe fn chip_slot(rt: *mut kvm_irq_routing_table, irqchip: c_uint, pin: c_uint) -> *mut c_int {
    ((*rt).chip.as_mut_ptr() as *mut c_int)
        .add(irqchip as usize * KVM_IRQCHIP_NUM_PINS + pin as usize)
}

unsafe fn map_slot(rt: *mut kvm_irq_routing_table, gsi: u32) -> *mut hlist_head {
    ((*rt).map.as_mut_ptr() as *mut hlist_head).add(gsi as usize)
}

pub unsafe extern "C" fn kvm_irq_map_gsi(
    kvm: *mut kvm,
    entries: *mut kvm_kernel_irq_routing_entry,
    gsi: c_int,
) -> c_int {
    let irq_rt: *mut kvm_irq_routing_table;
    let mut e: *mut kvm_kernel_irq_routing_entry;
    let mut n: c_int = 0;

    irq_rt = srcu_dereference_check_irq_routing(
        (*kvm).irq_routing,
        &mut (*kvm).irq_srcu,
        lockdep_is_held(&mut (*kvm).irq_lock),
    );
    if !irq_rt.is_null() && gsi < (*irq_rt).nr_rt_entries as c_int {
        let mut node = (*map_slot(irq_rt, gsi as u32)).first;
        while !node.is_null() {
            e = hlist_entry_from_link(node);
            *entries.add(n as usize) = *e;
            n += 1;
            node = (*node).next;
        }
    }

    n
}

pub unsafe extern "C" fn kvm_irq_map_chip_pin(
    kvm: *mut kvm,
    irqchip: c_uint,
    pin: c_uint,
) -> c_int {
    let irq_rt: *mut kvm_irq_routing_table;

    irq_rt = srcu_dereference_irq_routing((*kvm).irq_routing, &mut (*kvm).irq_srcu);
    *chip_slot(irq_rt, irqchip, pin)
}

pub unsafe extern "C" fn kvm_send_userspace_msi(kvm: *mut kvm, msi: *mut kvm_msi) -> c_int {
    let mut route: kvm_kernel_irq_routing_entry = core::mem::zeroed();

    if !kvm_arch_irqchip_in_kernel(kvm) || ((*msi).flags & !KVM_MSI_VALID_DEVID) != 0 {
        return -EINVAL;
    }

    route.msi.address_lo = (*msi).address_lo;
    route.msi.address_hi = (*msi).address_hi;
    route.msi.data = (*msi).data;
    route.msi.flags = (*msi).flags;
    route.msi.devid = (*msi).devid;

    kvm_set_msi(
        &mut route,
        kvm,
        KVM_USERSPACE_IRQ_SOURCE_ID,
        1,
        false,
    )
}

/*
 * Return value:
 *  < 0   Interrupt was ignored (masked or not delivered for other reasons)
 *  = 0   Interrupt was coalesced (previous irq is still pending)
 *  > 0   Number of CPUs interrupt was delivered to
 */
pub unsafe extern "C" fn kvm_set_irq(
    kvm: *mut kvm,
    irq_source_id: c_int,
    irq: u32,
    level: c_int,
    line_status: bool,
) -> c_int {
    let mut irq_set: [kvm_kernel_irq_routing_entry; 0] = [];
    let mut ret: c_int = -1;
    let mut i: c_int;
    let idx: c_int;

    trace_kvm_set_irq(irq, level, irq_source_id);

    /* Not possible to detect if the guest uses the PIC or the
     * IOAPIC.  So set the bit in both. The guest will ignore
     * writes to the unused one.
     */
    idx = srcu_read_lock(&mut (*kvm).irq_srcu);
    i = kvm_irq_map_gsi(kvm, irq_set.as_mut_ptr(), irq as c_int);
    srcu_read_unlock(&mut (*kvm).irq_srcu, idx);

    while {
        i -= 1;
        i >= 0
    } {
        let r: c_int;
        r = (irq_set[i as usize].set)(
            &mut irq_set[i as usize],
            kvm,
            irq_source_id,
            level,
            line_status,
        );
        if r < 0 {
            continue;
        }

        ret = r + if ret < 0 { 0 } else { ret };
    }

    ret
}

unsafe fn free_irq_routing_table(rt: *mut kvm_irq_routing_table) {
    let mut i: c_int;

    if rt.is_null() {
        return;
    }

    i = 0;
    while i < (*rt).nr_rt_entries as c_int {
        let mut node = (*map_slot(rt, i as u32)).first;
        while !node.is_null() {
            let next = (*node).next;
            let e = hlist_entry_from_link(node);
            hlist_del(&mut (*e).link);
            kfree(e as *mut c_void);
            node = next;
        }
        i += 1;
    }

    kfree(rt as *mut c_void);
}

pub unsafe extern "C" fn kvm_free_irq_routing(kvm: *mut kvm) {
    /* Called only during vm destruction. Nobody can use the pointer
       at this stage */
    let rt: *mut kvm_irq_routing_table = rcu_access_pointer_irq_routing((*kvm).irq_routing);
    free_irq_routing_table(rt);
}

unsafe fn setup_routing_entry(
    kvm: *mut kvm,
    rt: *mut kvm_irq_routing_table,
    e: *mut kvm_kernel_irq_routing_entry,
    ue: *const kvm_irq_routing_entry,
) -> c_int {
    let mut ei: *mut kvm_kernel_irq_routing_entry;
    let r: c_int;
    let gsi: u32 = array_index_nospec((*ue).gsi, KVM_MAX_IRQ_ROUTES);

    /*
     * Do not allow GSI to be mapped to the same irqchip more than once.
     * Allow only one to one mapping between GSI and non-irqchip routing.
     */
    let mut node = (*map_slot(rt, gsi)).first;
    while !node.is_null() {
        ei = hlist_entry_from_link(node);
        if (*ei).type_ != KVM_IRQ_ROUTING_IRQCHIP
            || (*ue).type_ != KVM_IRQ_ROUTING_IRQCHIP
            || (*ue).u.irqchip.irqchip == (*ei).irqchip.irqchip
        {
            return -EINVAL;
        }
        node = (*node).next;
    }

    (*e).gsi = gsi;
    (*e).type_ = (*ue).type_;
    r = kvm_set_routing_entry(kvm, e, ue);
    if r != 0 {
        return r;
    }
    if (*e).type_ == KVM_IRQ_ROUTING_IRQCHIP {
        *chip_slot(rt, (*e).irqchip.irqchip, (*e).irqchip.pin) = (*e).gsi as c_int;
    }

    hlist_add_head(&mut (*e).link, map_slot(rt, (*e).gsi));

    0
}

#[no_mangle]
pub unsafe extern "C" fn kvm_arch_irq_routing_update(_kvm: *mut kvm) {}

#[no_mangle]
pub unsafe extern "C" fn kvm_arch_can_set_irq_routing(_kvm: *mut kvm) -> bool {
    true
}

pub unsafe extern "C" fn kvm_set_irq_routing(
    kvm: *mut kvm,
    mut ue: *const kvm_irq_routing_entry,
    nr: c_uint,
    _flags: c_uint,
) -> c_int {
    let mut new: *mut kvm_irq_routing_table;
    let old: *mut kvm_irq_routing_table;
    let mut e: *mut kvm_kernel_irq_routing_entry;
    let mut i: u32;
    let mut j: u32;
    let mut nr_rt_entries: u32 = 0;
    let mut r: c_int;

    i = 0;
    while i < nr {
        if (*ue.add(i as usize)).gsi >= KVM_MAX_IRQ_ROUTES {
            return -EINVAL;
        }
        nr_rt_entries = core::cmp::max(nr_rt_entries, (*ue.add(i as usize)).gsi);
        i += 1;
    }

    nr_rt_entries += 1;

    new = kzalloc_flex_irq_routing_table(nr_rt_entries, GFP_KERNEL_ACCOUNT);
    if new.is_null() {
        return -ENOMEM;
    }

    (*new).nr_rt_entries = nr_rt_entries;
    i = 0;
    while (i as usize) < KVM_NR_IRQCHIPS {
        j = 0;
        while (j as usize) < KVM_IRQCHIP_NUM_PINS {
            *chip_slot(new, i, j) = -1;
            j += 1;
        }
        i += 1;
    }

    i = 0;
    while i < nr {
        r = -ENOMEM;
        e = kzalloc_kernel_irq_routing_entry(GFP_KERNEL_ACCOUNT);
        if e.is_null() {
            break;
        }

        r = -EINVAL;
        match (*ue).type_ {
            KVM_IRQ_ROUTING_MSI => {
                if ((*ue).flags & !KVM_MSI_VALID_DEVID) != 0 {
                    kfree(e as *mut c_void);
                    free_irq_routing_table(new);
                    return r;
                }
            }
            _ => {
                if (*ue).flags != 0 {
                    kfree(e as *mut c_void);
                    free_irq_routing_table(new);
                    return r;
                }
            }
        }
        r = setup_routing_entry(kvm, new, e, ue);
        if r != 0 {
            kfree(e as *mut c_void);
            free_irq_routing_table(new);
            return r;
        }
        ue = ue.add(1);
        i += 1;
    }

    if i != nr {
        free_irq_routing_table(new);
        return r;
    }

    mutex_lock(&mut (*kvm).irq_lock);
    old = rcu_dereference_protected_irq_routing((*kvm).irq_routing, 1);
    rcu_assign_pointer_irq_routing(&mut (*kvm).irq_routing, new);
    kvm_irq_routing_update(kvm);
    kvm_arch_irq_routing_update(kvm);
    mutex_unlock(&mut (*kvm).irq_lock);

    synchronize_srcu_expedited(&mut (*kvm).irq_srcu);

    new = old;
    r = 0;
    free_irq_routing_table(new);

    r
}

/*
 * Allocate empty IRQ routing by default so that additional setup isn't needed
 * when userspace-driven IRQ routing is activated, and so that kvm->irq_routing
 * is guaranteed to be non-NULL.
 */
pub unsafe extern "C" fn kvm_init_irq_routing(kvm: *mut kvm) -> c_int {
    let new: *mut kvm_irq_routing_table;
    let chip_size: c_int;

    new = kzalloc_flex_irq_routing_table(1, GFP_KERNEL_ACCOUNT);
    if new.is_null() {
        return -ENOMEM;
    }

    (*new).nr_rt_entries = 1;

    chip_size = (core::mem::size_of::<c_int>() * KVM_NR_IRQCHIPS * KVM_IRQCHIP_NUM_PINS) as c_int;
    memset((*new).chip.as_mut_ptr() as *mut c_void, -1, chip_size as usize);

    rcu_init_pointer_irq_routing(&mut (*kvm).irq_routing, new);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
