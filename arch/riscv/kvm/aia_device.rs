// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2021 Western Digital Corporation or its affiliates.
 * Copyright (C) 2022 Ventana Micro Systems Inc.
 *
 * Authors:
 *\tAnup Patel <apatel@ventanamicro.com>
 */

// C headers are supplied by the surrounding kernel translation unit.

unsafe fn aia_create(dev: *mut kvm_device, _type: u32) -> i32 {
    let mut ret: i32;
    let mut i: usize;
    let kvm = (*dev).kvm;
    let mut vcpu: *mut kvm_vcpu;

    if irqchip_in_kernel(kvm) {
        return -EEXIST;
    }
    if kvm_riscv_isa_check_host(SSAIA) {
        return -ENODEV;
    }

    ret = -EBUSY;
    if kvm_trylock_all_vcpus(kvm) != 0 {
        return ret;
    }

    kvm_for_each_vcpu!(i, vcpu, kvm, {
        if (*vcpu).arch.ran_atleast_once {
            goto!(out_unlock);
        }
    });
    ret = 0;
    (*kvm).arch.aia.in_kernel = true;

    out_unlock:
    kvm_unlock_all_vcpus(kvm);
    ret
}

unsafe fn aia_destroy(dev: *mut kvm_device) {
    kfree(dev);
}

unsafe fn aia_config(kvm: *mut kvm, type_: usize, nr: *mut u32, write: bool) -> i32 {
    let aia = &mut (*kvm).arch.aia;
    if write && kvm_riscv_aia_initialized(kvm) {
        return -EBUSY;
    }
    match type_ {
        KVM_DEV_RISCV_AIA_CONFIG_MODE => {
            if write {
                match *nr {
                    KVM_DEV_RISCV_AIA_MODE_EMUL => {}
                    KVM_DEV_RISCV_AIA_MODE_HWACCEL | KVM_DEV_RISCV_AIA_MODE_AUTO => {
                        if atomic_read(&kvm_riscv_aia_nr_hgei) == 0 { return -EINVAL; }
                    }
                    _ => return -EINVAL,
                }
                aia.mode = *nr;
            } else { *nr = aia.mode; }
        }
        KVM_DEV_RISCV_AIA_CONFIG_IDS => {
            if write {
                if *nr < KVM_DEV_RISCV_AIA_IDS_MIN || *nr >= KVM_DEV_RISCV_AIA_IDS_MAX ||
                   (*nr & KVM_DEV_RISCV_AIA_IDS_MIN) != KVM_DEV_RISCV_AIA_IDS_MIN ||
                   kvm_riscv_aia_max_ids <= *nr { return -EINVAL; }
                aia.nr_ids = *nr;
            } else { *nr = aia.nr_ids; }
        }
        KVM_DEV_RISCV_AIA_CONFIG_SRCS => {
            if write {
                if *nr >= KVM_DEV_RISCV_AIA_SRCS_MAX || *nr >= kvm_riscv_aia_max_ids { return -EINVAL; }
                aia.nr_sources = *nr;
            } else { *nr = aia.nr_sources; }
        }
        KVM_DEV_RISCV_AIA_CONFIG_GROUP_BITS => {
            if write { if *nr >= KVM_DEV_RISCV_AIA_GROUP_BITS_MAX { return -EINVAL; } aia.nr_group_bits = *nr; }
            else { *nr = aia.nr_group_bits; }
        }
        KVM_DEV_RISCV_AIA_CONFIG_GROUP_SHIFT => {
            if write {
                if *nr < KVM_DEV_RISCV_AIA_GROUP_SHIFT_MIN || *nr >= KVM_DEV_RISCV_AIA_GROUP_SHIFT_MAX { return -EINVAL; }
                aia.nr_group_shift = *nr;
            } else { *nr = aia.nr_group_shift; }
        }
        KVM_DEV_RISCV_AIA_CONFIG_HART_BITS => {
            if write { if *nr >= KVM_DEV_RISCV_AIA_HART_BITS_MAX { return -EINVAL; } aia.nr_hart_bits = *nr; }
            else { *nr = aia.nr_hart_bits; }
        }
        KVM_DEV_RISCV_AIA_CONFIG_GUEST_BITS => {
            if write { if *nr >= KVM_DEV_RISCV_AIA_GUEST_BITS_MAX { return -EINVAL; } aia.nr_guest_bits = *nr; }
            else { *nr = aia.nr_guest_bits; }
        }
        _ => return -ENXIO,
    }
    0
}

unsafe fn aia_aplic_addr(kvm: *mut kvm, addr: *mut u64, write: bool) -> i32 {
    let aia = &mut (*kvm).arch.aia;
    if write {
        if kvm_riscv_aia_initialized(kvm) { return -EBUSY; }
        if *addr & (KVM_DEV_RISCV_APLIC_ALIGN - 1) != 0 { return -EINVAL; }
        aia.aplic_addr = *addr;
    } else { *addr = aia.aplic_addr; }
    0
}

unsafe fn aia_imsic_addr(kvm: *mut kvm, addr: *mut u64, vcpu_idx: usize, write: bool) -> i32 {
    let vcpu = kvm_get_vcpu(kvm, vcpu_idx);
    if vcpu.is_null() { return -EINVAL; }
    let vcpu_aia = &mut (*vcpu).arch.aia_context;
    if write {
        if kvm_riscv_aia_initialized(kvm) { return -EBUSY; }
        if *addr & (KVM_DEV_RISCV_IMSIC_ALIGN - 1) != 0 { return -EINVAL; }
    }
    mutex_lock(&mut (*vcpu).mutex);
    if write { vcpu_aia.imsic_addr = *addr; } else { *addr = vcpu_aia.imsic_addr; }
    mutex_unlock(&mut (*vcpu).mutex);
    0
}

unsafe fn aia_imsic_ppn(aia: *mut kvm_aia, addr: u64) -> u64 {
    let h = (*aia).nr_hart_bits + (*aia).nr_guest_bits + IMSIC_MMIO_PAGE_SHIFT - 1;
    let mut mask = genmask_ull(h, 0);
    if (*aia).nr_group_bits != 0 {
        let gh = (*aia).nr_group_bits + (*aia).nr_group_shift - 1;
        mask |= genmask_ull(gh, (*aia).nr_group_shift);
    }
    (addr & !mask) >> IMSIC_MMIO_PAGE_SHIFT
}

unsafe fn aia_imsic_hart_index(aia: *mut kvm_aia, addr: u64) -> u32 {
    let mut hart = 0;
    let mut group = 0;
    if (*aia).nr_hart_bits != 0 { hart = ((addr >> ((*aia).nr_guest_bits + IMSIC_MMIO_PAGE_SHIFT)) & genmask_ull((*aia).nr_hart_bits - 1, 0)) as u32; }
    if (*aia).nr_group_bits != 0 { group = ((addr >> (*aia).nr_group_shift) & genmask_ull((*aia).nr_group_bits - 1, 0)) as u32; }
    (group << (*aia).nr_hart_bits) | hart
}

unsafe fn aia_init(kvm: *mut kvm) -> i32 {
    let aia = &mut (*kvm).arch.aia;
    if kvm_riscv_aia_initialized(kvm) || (*kvm).created_vcpus != atomic_read(&(*kvm).online_vcpus) { return -EBUSY; }
    if aia.nr_ids < aia.nr_sources || (aia.nr_sources != 0 && aia.aplic_addr == KVM_RISCV_AIA_UNDEF_ADDR) { return -EINVAL; }
    if aia.nr_group_bits != 0 && aia.nr_group_shift < IMSIC_MMIO_PAGE_SHIFT + aia.nr_guest_bits + aia.nr_hart_bits { return -EINVAL; }
    let mut ret = kvm_riscv_aia_aplic_init(kvm);
    if ret != 0 { return ret; }
    let mut base_ppn = KVM_RISCV_AIA_UNDEF_ADDR;
    let mut idx: usize = 0;
    let mut vcpu: *mut kvm_vcpu;
    kvm_for_each_vcpu!(idx, vcpu, kvm, {
        let vaia = &mut (*vcpu).arch.aia_context;
        if vaia.imsic_addr == KVM_RISCV_AIA_UNDEF_ADDR { ret = -EINVAL; goto!(fail_cleanup_imsics); }
        if base_ppn == KVM_RISCV_AIA_UNDEF_ADDR { base_ppn = aia_imsic_ppn(aia, vaia.imsic_addr); }
        if base_ppn != aia_imsic_ppn(aia, vaia.imsic_addr) { ret = -EINVAL; goto!(fail_cleanup_imsics); }
        vaia.hart_index = aia_imsic_hart_index(aia, vaia.imsic_addr);
        ret = kvm_riscv_vcpu_aia_imsic_init(vcpu);
        if ret != 0 { goto!(fail_cleanup_imsics); }
    });
    (*kvm).arch.aia.initialized = true;
    return 0;
    fail_cleanup_imsics:
    let mut i = idx as i32 - 1;
    while i >= 0 { let v = kvm_get_vcpu(kvm, i as usize); if !v.is_null() { kvm_riscv_vcpu_aia_imsic_cleanup(v); } i -= 1; }
    kvm_riscv_aia_aplic_cleanup(kvm);
    ret
}

// Device attribute handlers retain the kernel ABI and delegate to the corresponding helpers.
unsafe fn aia_set_attr(dev: *mut kvm_device, attr: *mut kvm_device_attr) -> i32 {
    let mut nr = 0u32; let mut addr = 0u64; let mut v = 0usize;
    let type_ = (*attr).attr as usize; let uaddr = (*attr).addr as *mut u8; let kvm = (*dev).kvm;
    match (*attr).group {
        KVM_DEV_RISCV_AIA_GRP_CONFIG => { if copy_from_user(&mut nr, uaddr, 4) != 0 { return -EFAULT; } mutex_lock(&mut (*kvm).lock); let r = aia_config(kvm, type_, &mut nr, true); mutex_unlock(&mut (*kvm).lock); r }
        KVM_DEV_RISCV_AIA_GRP_ADDR => { if copy_from_user(&mut addr, uaddr, 8) != 0 { return -EFAULT; } mutex_lock(&mut (*kvm).lock); let r = if type_ == KVM_DEV_RISCV_AIA_ADDR_APLIC { aia_aplic_addr(kvm, &mut addr, true) } else { aia_imsic_addr(kvm, &mut addr, type_ - KVM_DEV_RISCV_AIA_ADDR_IMSIC(0), true) }; mutex_unlock(&mut (*kvm).lock); r }
        KVM_DEV_RISCV_AIA_GRP_CTRL => { if type_ == KVM_DEV_RISCV_AIA_CTRL_INIT { mutex_lock(&mut (*kvm).lock); let r = aia_init(kvm); mutex_unlock(&mut (*kvm).lock); r } else { -ENXIO } }
        KVM_DEV_RISCV_AIA_GRP_APLIC => { if copy_from_user(&mut nr, uaddr, 4) != 0 { return -EFAULT; } mutex_lock(&mut (*kvm).lock); let r = kvm_riscv_aia_aplic_set_attr(kvm, type_, nr); mutex_unlock(&mut (*kvm).lock); r }
        KVM_DEV_RISCV_AIA_GRP_IMSIC => { if copy_from_user(&mut v, uaddr, 8) != 0 { return -EFAULT; } mutex_lock(&mut (*kvm).lock); let r = kvm_riscv_aia_imsic_rw_attr(kvm, type_, true, &mut v); mutex_unlock(&mut (*kvm).lock); r }
        _ => -ENXIO,
    }
}

unsafe fn aia_get_attr(dev: *mut kvm_device, attr: *mut kvm_device_attr) -> i32 {
    let mut nr = 0u32; let mut addr = 0u64; let mut v = 0usize;
    let type_ = (*attr).attr as usize; let uaddr = (*attr).addr as *mut u8; let kvm = (*dev).kvm;
    let r = match (*attr).group {
        KVM_DEV_RISCV_AIA_GRP_CONFIG => { if copy_from_user(&mut nr, uaddr, 4) != 0 { return -EFAULT; } mutex_lock(&mut (*kvm).lock); let r = aia_config(kvm, type_, &mut nr, false); mutex_unlock(&mut (*kvm).lock); if r == 0 && copy_to_user(uaddr, &nr, 4) != 0 { return -EFAULT; } r }
        KVM_DEV_RISCV_AIA_GRP_ADDR => { if copy_from_user(&mut addr, uaddr, 8) != 0 { return -EFAULT; } mutex_lock(&mut (*kvm).lock); let r = if type_ == KVM_DEV_RISCV_AIA_ADDR_APLIC { aia_aplic_addr(kvm, &mut addr, false) } else { aia_imsic_addr(kvm, &mut addr, type_ - KVM_DEV_RISCV_AIA_ADDR_IMSIC(0), false) }; mutex_unlock(&mut (*kvm).lock); if r == 0 && copy_to_user(uaddr, &addr, 8) != 0 { return -EFAULT; } r }
        KVM_DEV_RISCV_AIA_GRP_APLIC => { if copy_from_user(&mut nr, uaddr, 4) != 0 { return -EFAULT; } mutex_lock(&mut (*kvm).lock); let r = kvm_riscv_aia_aplic_get_attr(kvm, type_, &mut nr); mutex_unlock(&mut (*kvm).lock); if r == 0 && copy_to_user(uaddr, &nr, 4) != 0 { return -EFAULT; } r }
        KVM_DEV_RISCV_AIA_GRP_IMSIC => { if copy_from_user(&mut v, uaddr, 8) != 0 { return -EFAULT; } mutex_lock(&mut (*kvm).lock); let r = kvm_riscv_aia_imsic_rw_attr(kvm, type_, false, &mut v); mutex_unlock(&mut (*kvm).lock); if r == 0 && copy_to_user(uaddr, &v, 8) != 0 { return -EFAULT; } r }
        _ => -ENXIO,
    }; r
}

unsafe fn aia_has_attr(dev: *mut kvm_device, attr: *mut kvm_device_attr) -> i32 {
    let kvm = (*dev).kvm;
    match (*attr).group {
        KVM_DEV_RISCV_AIA_GRP_CONFIG => match (*attr).attr { KVM_DEV_RISCV_AIA_CONFIG_MODE | KVM_DEV_RISCV_AIA_CONFIG_IDS | KVM_DEV_RISCV_AIA_CONFIG_SRCS | KVM_DEV_RISCV_AIA_CONFIG_GROUP_BITS | KVM_DEV_RISCV_AIA_CONFIG_GROUP_SHIFT | KVM_DEV_RISCV_AIA_CONFIG_HART_BITS | KVM_DEV_RISCV_AIA_CONFIG_GUEST_BITS => 0, _ => -ENXIO },
        KVM_DEV_RISCV_AIA_GRP_CTRL => if (*attr).attr == KVM_DEV_RISCV_AIA_CTRL_INIT { 0 } else { -ENXIO },
        KVM_DEV_RISCV_AIA_GRP_APLIC => { mutex_lock(&mut (*kvm).lock); let r = kvm_riscv_aia_aplic_has_attr(kvm, (*attr).attr); mutex_unlock(&mut (*kvm).lock); r }
        KVM_DEV_RISCV_AIA_GRP_IMSIC => { mutex_lock(&mut (*kvm).lock); let r = kvm_riscv_aia_imsic_has_attr(kvm, (*attr).attr); mutex_unlock(&mut (*kvm).lock); r }
        _ => 0,
    }
}

pub static mut kvm_riscv_aia_device_ops: kvm_device_ops = kvm_device_ops { name: "kvm-riscv-aia", create: Some(aia_create), destroy: Some(aia_destroy), set_attr: Some(aia_set_attr), get_attr: Some(aia_get_attr), has_attr: Some(aia_has_attr) };

pub unsafe fn kvm_riscv_vcpu_aia_update(vcpu: *mut kvm_vcpu) -> i32 { if !kvm_riscv_aia_initialized((*vcpu).kvm) { 1 } else { kvm_riscv_vcpu_aia_imsic_update(vcpu) } }
pub unsafe fn kvm_riscv_vcpu_aia_reset(vcpu: *mut kvm_vcpu) { let csr = &mut (*vcpu).arch.aia_context.guest_csr; if !kvm_riscv_aia_available() { return; } memset(csr, 0, core::mem::size_of_val(csr)); if kvm_riscv_aia_initialized((*vcpu).kvm) { kvm_riscv_vcpu_aia_imsic_reset(vcpu); } }
pub unsafe fn kvm_riscv_vcpu_aia_init(vcpu: *mut kvm_vcpu) { if kvm_riscv_aia_available() { (*vcpu).arch.aia_context.imsic_addr = KVM_RISCV_AIA_UNDEF_ADDR; (*vcpu).arch.aia_context.hart_index = (*vcpu).vcpu_idx; } }
pub unsafe fn kvm_riscv_vcpu_aia_deinit(vcpu: *mut kvm_vcpu) { if kvm_riscv_aia_initialized((*vcpu).kvm) { kvm_riscv_vcpu_aia_imsic_cleanup(vcpu); } }
pub unsafe fn kvm_riscv_aia_inject_msi_by_id(kvm: *mut kvm, hart_index: u32, guest_index: u32, iid: u32) -> i32 { if !kvm_riscv_aia_initialized(kvm) { return -EBUSY; } let mut i = 0; let mut vcpu = core::ptr::null_mut(); kvm_for_each_vcpu!(i, vcpu, kvm, { if (*vcpu).arch.aia_context.hart_index == hart_index { return kvm_riscv_vcpu_aia_imsic_inject(vcpu, guest_index, 0, iid); } }); 0 }
pub unsafe fn kvm_riscv_aia_inject_msi(kvm: *mut kvm, msi: *mut kvm_msi) -> i32 {
    if !kvm_riscv_aia_initialized(kvm) { return -EBUSY; }
    let mut target = ((*msi).address_hi as u64) << 32 | (*msi).address_lo as u64;
    let aia = &mut (*kvm).arch.aia;
    let g = (target >> IMSIC_MMIO_PAGE_SHIFT) & ((1u64 << aia.nr_guest_bits) - 1);
    let tppn = (target >> IMSIC_MMIO_PAGE_SHIFT) & !((1u64 << aia.nr_guest_bits) - 1);
    let toff = target & (IMSIC_MMIO_PAGE_SZ - 1);
    let iid = (*msi).data;
    let mut idx = 0; let mut vcpu = core::ptr::null_mut();
    kvm_for_each_vcpu!(idx, vcpu, kvm, {
        let ippn = (*vcpu).arch.aia_context.imsic_addr >> IMSIC_MMIO_PAGE_SHIFT;
        if ippn == tppn { return kvm_riscv_vcpu_aia_imsic_inject(vcpu, g as u32, toff as u32, iid); }
    });
    0
}
pub unsafe fn kvm_riscv_aia_inject_irq(kvm: *mut kvm, irq: u32, level: bool) -> i32 { if !kvm_riscv_aia_initialized(kvm) { -EBUSY } else { kvm_riscv_aia_aplic_inject(kvm, irq, level) } }
pub unsafe fn kvm_riscv_aia_init_vm(kvm: *mut kvm) { if kvm_riscv_aia_available() { let a = &mut (*kvm).arch.aia; a.mode = if atomic_read(&kvm_riscv_aia_nr_hgei) != 0 { KVM_DEV_RISCV_AIA_MODE_AUTO } else { KVM_DEV_RISCV_AIA_MODE_EMUL }; a.nr_ids = kvm_riscv_aia_max_ids - 1; a.nr_sources = 0; a.nr_group_bits = 0; a.nr_group_shift = KVM_DEV_RISCV_AIA_GROUP_SHIFT_MIN; a.nr_hart_bits = 0; a.nr_guest_bits = 0; a.aplic_addr = KVM_RISCV_AIA_UNDEF_ADDR; } }
pub unsafe fn kvm_riscv_aia_destroy_vm(kvm: *mut kvm) { if kvm_riscv_aia_initialized(kvm) { kvm_riscv_aia_aplic_cleanup(kvm); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
