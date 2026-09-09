// SPDX-License-Identifier: GPL-2.0
/*
 * s390 kvm PCI passthrough support
 *
 * Copyright IBM Corp. 2022
 *
 *    Author(s): Matthew Rosato <mjrosato@linux.ibm.com>
 */

// Dependencies supplied by the surrounding kernel translation.

static mut aift: *mut zpci_aift = core::ptr::null_mut();

#[inline]
unsafe fn __set_irq_noiib(ctl: u16, isc: u8) -> i32 {
    let mut iib: zpci_sic_iib = core::mem::zeroed();
    zpci_set_irq_ctrl(ctl, isc, &mut iib)
}

pub unsafe fn kvm_s390_pci_aen_exit() {
    let mut flags: c_ulong = 0;
    let gait_kzdev: *mut *mut kvm_zdev;
    lockdep_assert_held(&(*aift).aift_lock);
    spin_lock_irqsave(&(*aift).gait_lock, &mut flags);
    gait_kzdev = (*aift).kzdev;
    (*aift).gait = core::ptr::null_mut();
    (*aift).sbv = core::ptr::null_mut();
    (*aift).kzdev = core::ptr::null_mut();
    spin_unlock_irqrestore(&(*aift).gait_lock, flags);
    kfree(gait_kzdev);
}

unsafe fn zpci_setup_aipb(nisc: u8) -> i32 {
    let page: *mut page;
    let (mut size, mut rc): (i32, i32);
    zpci_aipb = kzalloc_obj::<zpci_sic_iib>();
    if zpci_aipb.is_null() { return -ENOMEM; }
    (*aift).sbv = airq_iv_create(ZPCI_NR_DEVICES, AIRQ_IV_ALLOC, core::ptr::null_mut());
    if (*aift).sbv.is_null() { rc = -ENOMEM; goto!(free_aipb); }
    zpci_aif_sbv = (*aift).sbv;
    size = get_order(PAGE_ALIGN(ZPCI_NR_DEVICES * core::mem::size_of::<zpci_gaite>()));
    page = alloc_pages(GFP_KERNEL | __GFP_ZERO, size);
    if page.is_null() { rc = -ENOMEM; goto!(free_sbv); }
    (*aift).gait = page_to_virt(page) as *mut zpci_gaite;
    (*zpci_aipb).aipb.faisb = virt_to_phys((*aift).sbv.cast::<core::ffi::c_void>());
    (*zpci_aipb).aipb.gait = virt_to_phys((*aift).gait.cast::<core::ffi::c_void>());
    (*zpci_aipb).aipb.afi = nisc;
    (*zpci_aipb).aipb.faal = ZPCI_NR_DEVICES;
    if zpci_set_irq_ctrl(SIC_SET_AENI_CONTROLS, 0, zpci_aipb) != 0 { rc = -EIO; goto!(free_gait); }
    return 0;
free_gait:
    free_pages((*aift).gait as c_ulong, size);
free_sbv:
    airq_iv_release((*aift).sbv);
    zpci_aif_sbv = core::ptr::null_mut();
free_aipb:
    kfree(zpci_aipb);
    zpci_aipb = core::ptr::null_mut();
    rc
}

unsafe fn zpci_reset_aipb(nisc: u8) -> i32 {
    if (*zpci_aipb).aipb.afi != nisc { return -EINVAL; }
    (*aift).sbv = zpci_aif_sbv;
    (*aift).gait = phys_to_virt((*zpci_aipb).aipb.gait) as *mut zpci_gaite;
    0
}

pub unsafe fn kvm_s390_pci_aen_init(nisc: u8) -> i32 {
    let mut rc = 0;
    if !(*aift).gait.is_null() || !(*aift).sbv.is_null() { return -EPERM; }
    mutex_lock(&(*aift).aift_lock);
    (*aift).kzdev = kzalloc_objs::<*mut kvm_zdev>(ZPCI_NR_DEVICES);
    if (*aift).kzdev.is_null() { rc = -ENOMEM; goto!(unlock); }
    rc = if zpci_aipb.is_null() { zpci_setup_aipb(nisc) } else { zpci_reset_aipb(nisc) };
    if rc != 0 { goto!(free_zdev); }
    if __set_irq_noiib(SIC_IRQ_MODE_SINGLE, nisc) != 0 { rc = -EIO; kvm_s390_pci_aen_exit(); }
    goto!(unlock);
free_zdev:
    kfree((*aift).kzdev);
unlock:
    mutex_unlock(&(*aift).aift_lock);
    rc
}

unsafe fn kvm_zpci_set_airq(zdev: *mut zpci_dev) -> i32 {
    let req = ZPCI_CREATE_REQ((*zdev).fh, 0, ZPCI_MOD_FC_REG_INT);
    let mut fib: zpci_fib = core::mem::zeroed();
    let mut status = 0u8;
    fib.fmt0.isc = (*(*zdev).kzdev).fib.fmt0.isc;
    fib.fmt0.sum = 1;
    fib.fmt0.noi = airq_iv_end((*zdev).aibv);
    fib.fmt0.aibv = virt_to_phys((*zdev).aibv.cast::<core::ffi::c_void>());
    fib.fmt0.aibvo = 0;
    fib.fmt0.aisb = virt_to_phys((*aift).sbv.cast::<core::ffi::c_void>()) + ((*zdev).aisb / 64) * 8;
    fib.fmt0.aisbo = (*zdev).aisb & 63;
    fib.gd = (*zdev).gisa;
    if zpci_mod_fc(req, &mut fib, &mut status) != 0 { -EIO } else { 0 }
}

unsafe fn kvm_zpci_clear_airq(zdev: *mut zpci_dev) -> i32 {
    let req = ZPCI_CREATE_REQ((*zdev).fh, 0, ZPCI_MOD_FC_DEREG_INT);
    let mut fib: zpci_fib = core::mem::zeroed();
    let mut cc: u8; let mut status = 0u8;
    fib.gd = (*zdev).gisa;
    cc = zpci_mod_fc(req, &mut fib, &mut status);
    if cc == 3 || (cc == 1 && status == 24) { cc = 0; }
    if cc != 0 { -EIO } else { 0 }
}

#[inline] unsafe fn unaccount_mem(kzdev: *mut kvm_zdev, nr_pages: c_ulong) {
    let user = (*kzdev).user_account; let mm = (*kzdev).mm_account;
    if !user.is_null() { atomic_long_sub(nr_pages, &mut (*user).locked_vm); free_uid(user); (*kzdev).user_account = core::ptr::null_mut(); }
    if !mm.is_null() { atomic64_sub(nr_pages, &mut (*mm).pinned_vm); mmdrop(mm); (*kzdev).mm_account = core::ptr::null_mut(); }
}

#[inline] unsafe fn account_mem(kzdev: *mut kvm_zdev, nr_pages: c_ulong) -> i32 {
    let user = get_uid(current_user()); let limit = rlimit(RLIMIT_MEMLOCK) >> PAGE_SHIFT;
    let mut cur = atomic_long_read(&(*user).locked_vm);
    loop { let new = cur + nr_pages; if new > limit { free_uid(user); return -ENOMEM; } if atomic_long_try_cmpxchg(&mut (*user).locked_vm, &mut cur, new) { break; } }
    if !(*current).mm.is_null() { mmgrab((*current).mm); atomic64_add(nr_pages, &mut (*(*current).mm).pinned_vm); }
    (*kzdev).user_account = user; (*kzdev).mm_account = (*current).mm; 0
}

unsafe fn kvm_s390_pci_dev_open(zdev: *mut zpci_dev) -> i32 { let kz = kzalloc_obj::<kvm_zdev>(); if kz.is_null() { return -ENOMEM; } (*kz).zdev=zdev; (*zdev).kzdev=kz; 0 }
unsafe fn kvm_s390_pci_dev_release(zdev: *mut zpci_dev) { let kz=(*zdev).kzdev; WARN_ON((*kz).zdev != zdev); (*zdev).kzdev=core::ptr::null_mut(); kfree(kz); }

pub unsafe fn kvm_s390_pci_init_list(kvm: *mut kvm) { spin_lock_init(&mut (*kvm).arch.kzdev_list_lock); INIT_LIST_HEAD(&mut (*kvm).arch.kzdev_list); }
pub unsafe fn kvm_s390_pci_clear_list(kvm: *mut kvm) { spin_lock(&mut (*kvm).arch.kzdev_list_lock); WARN_ON_ONCE(!list_empty(&(*kvm).arch.kzdev_list)); spin_unlock(&mut (*kvm).arch.kzdev_list_lock); }

// Device registration, AIF enable/disable, operation dispatch, and module
// initialization callbacks are declarations supplied by the translated
// companion implementation; their C ABI-facing names are preserved here.
extern "C" {
    fn kvm_s390_pci_register_kvm(opaque: *mut core::ffi::c_void, kvm: *mut kvm) -> i32;
    fn kvm_s390_pci_unregister_kvm(opaque: *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
