// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015, 2016 ARM Ltd.
 */

// Linux/KVM headers and "vgic.h" provide the external declarations used here.

use core::ffi::c_void;

#[repr(C)]
pub struct kvm {
    pub arch: kvm_arch,
}
#[repr(C)]
pub struct vgic_dist {
    pub nr_spis: u32,
}
#[repr(C)]
pub struct kvm_arch {
    pub vgic: vgic_dist,
}
#[repr(C)]
pub struct kvm_kernel_irq_routing_entry {
    pub set: Option<unsafe extern "C" fn(*mut kvm_kernel_irq_routing_entry, *mut kvm, i32, i32, bool) -> i32>,
    pub type_: u32,
    pub irqchip: kvm_irq_routing_irqchip,
    pub msi: kvm_msi,
}
#[repr(C)]
pub struct kvm_irq_routing_entry {
    pub gsi: u32,
    pub type_: u32,
    pub flags: u32,
    pub u: kvm_irq_routing_union,
}
#[repr(C)]
pub union kvm_irq_routing_union {
    pub irqchip: kvm_irq_routing_irqchip,
    pub msi: kvm_irq_routing_msi,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_irq_routing_irqchip {
    pub irqchip: u32,
    pub pin: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_irq_routing_msi {
    pub address_lo: u32,
    pub address_hi: u32,
    pub data: u32,
    pub devid: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_msi {
    pub address_lo: u32,
    pub address_hi: u32,
    pub data: u32,
    pub flags: u32,
    pub devid: u32,
}

extern "C" {
    fn vgic_valid_spi(kvm: *mut kvm, spi_id: u32) -> bool;
    fn vgic_lazy_init(kvm: *mut kvm) -> i32;
    fn kvm_vgic_inject_irq(kvm: *mut kvm, vcpu: *mut c_void, spi_id: u32, level: i32, owner: *mut c_void) -> i32;
    fn vgic_has_its(kvm: *mut kvm) -> bool;
    fn vgic_its_inject_msi(kvm: *mut kvm, msi: *mut kvm_msi) -> i32;
    fn vgic_initialized(kvm: *mut kvm) -> bool;
    fn vgic_its_inject_cached_translation(kvm: *mut kvm, msi: *mut kvm_msi) -> i32;
    fn kvm_set_irq_routing(kvm: *mut kvm, entries: *mut kvm_irq_routing_entry, nr: u32, flags: u32) -> i32;
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
}

const EINVAL: i32 = 22;
const ENODEV: i32 = 19;
const EWOULDBLOCK: i32 = 11;
const ENOMEM: i32 = 12;
const KVM_IRQ_ROUTING_IRQCHIP: u32 = 1;
const KVM_IRQ_ROUTING_MSI: u32 = 2;
const KVM_IRQCHIP_NUM_PINS: u32 =  KVM_IRQCHIP_NUM_PINS_EXTERNAL;
const KVM_NR_IRQCHIPS: u32 = 1;
const VGIC_NR_PRIVATE_IRQS: u32 = 32;
const GFP_KERNEL_ACCOUNT: u32 = 0;
// Supplied by the KVM headers at build time.
const KVM_IRQCHIP_NUM_PINS_EXTERNAL: u32 = 256;

unsafe fn vgic_irqfd_set_irq(e: *mut kvm_kernel_irq_routing_entry, kvm: *mut kvm, _irq_source_id: i32, level: i32, _line_status: bool) -> i32 {
    let spi_id = (*e).irqchip.pin + VGIC_NR_PRIVATE_IRQS;
    if !vgic_valid_spi(kvm, spi_id) { return -EINVAL; }
    let ret = vgic_lazy_init(kvm);
    if ret != 0 { return ret; }
    kvm_vgic_inject_irq(kvm, core::ptr::null_mut(), spi_id, level, core::ptr::null_mut())
}

pub unsafe extern "C" fn kvm_set_routing_entry(kvm: *mut kvm, e: *mut kvm_kernel_irq_routing_entry, ue: *const kvm_irq_routing_entry) -> i32 {
    let mut r = -EINVAL;
    match (*ue).type_ {
        KVM_IRQ_ROUTING_IRQCHIP => {
            (*e).set = Some(vgic_irqfd_set_irq);
            let irqchip = (*ue).u.irqchip;
            (*e).irqchip = irqchip;
            if irqchip.pin >= KVM_IRQCHIP_NUM_PINS || irqchip.irqchip >= KVM_NR_IRQCHIPS { return r; }
        }
        KVM_IRQ_ROUTING_MSI => {
            (*e).set = Some(kvm_set_msi);
            let msi = (*ue).u.msi;
            (*e).msi.address_lo = msi.address_lo;
            (*e).msi.address_hi = msi.address_hi;
            (*e).msi.data = msi.data;
            (*e).msi.flags = (*ue).flags;
            (*e).msi.devid = msi.devid;
        }
        _ => return r,
    }
    let _ = kvm;
    r = 0;
    r
}

unsafe fn kvm_populate_msi(e: *mut kvm_kernel_irq_routing_entry, msi: *mut kvm_msi) {
    (*msi).address_lo = (*e).msi.address_lo;
    (*msi).address_hi = (*e).msi.address_hi;
    (*msi).data = (*e).msi.data;
    (*msi).flags = (*e).msi.flags;
    (*msi).devid = (*e).msi.devid;
}

pub unsafe extern "C" fn kvm_set_msi(e: *mut kvm_kernel_irq_routing_entry, kvm: *mut kvm, _irq_source_id: i32, level: i32, _line_status: bool) -> i32 {
    if !vgic_has_its(kvm) { return -ENODEV; }
    if level == 0 { return -1; }
    let mut msi = core::mem::zeroed::<kvm_msi>();
    kvm_populate_msi(e, &mut msi);
    vgic_its_inject_msi(kvm, &mut msi)
}

pub unsafe extern "C" fn kvm_arch_set_irq_inatomic(e: *mut kvm_kernel_irq_routing_entry, kvm: *mut kvm, irq_source_id: i32, level: i32, line_status: bool) -> i32 {
    if level == 0 { return -EWOULDBLOCK; }
    match (*e).type_ {
        KVM_IRQ_ROUTING_MSI => {
            let mut msi = core::mem::zeroed::<kvm_msi>();
            if !vgic_has_its(kvm) { return -EWOULDBLOCK; }
            kvm_populate_msi(e, &mut msi);
            vgic_its_inject_cached_translation(kvm, &mut msi)
        }
        KVM_IRQ_ROUTING_IRQCHIP => {
            if !vgic_initialized(kvm) { return -EWOULDBLOCK; }
            vgic_irqfd_set_irq(e, kvm, irq_source_id, 1, line_status)
        }
        _ => -EWOULDBLOCK,
    }
}

pub unsafe extern "C" fn kvm_vgic_setup_default_irq_routing(kvm: *mut kvm) -> i32 {
    let nr = (*kvm).arch.vgic.nr_spis;
    let entries = kzalloc((core::mem::size_of::<kvm_irq_routing_entry>() * nr as usize), GFP_KERNEL_ACCOUNT)
        as *mut kvm_irq_routing_entry;
    if entries.is_null() { return -ENOMEM; }
    for i in 0..nr {
        let entry = entries.add(i as usize);
        (*entry).gsi = i;
        (*entry).type_ = KVM_IRQ_ROUTING_IRQCHIP;
        (*entry).u.irqchip = kvm_irq_routing_irqchip { irqchip: 0, pin: i };
    }
    let ret = kvm_set_irq_routing(kvm, entries, nr, 0);
    kfree(entries as *mut c_void);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
