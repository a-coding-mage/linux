// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright IBM Corp. 2012
 *
 * Author(s):
 *   Jan Glauber <jang@linux.vnet.ibm.com>
 *
 * The System z PCI code is a rewrite from a prototype by
 * the following people (Kudoz!):
 *   Alexander Schmidt, Christoph Raisch, Hannes Hering,
 *   Hoang-Nam Nguyen, Jan-Bernd Themann, Stefan Roscher, Thomas Klein
 */

// Linux kernel headers and local headers from the original translation unit
// provide the external types, constants, macros, and functions referenced below.

static mut zpci_list: list_head = LIST_HEAD_INIT;
static mut zpci_list_lock: spinlock_t = __SPIN_LOCK_UNLOCKED;
static mut zpci_add_remove_lock: mutex = __MUTEX_INITIALIZER;
static mut zpci_domain: [unsigned_long; BITS_TO_LONGS(ZPCI_DOMAIN_BITMAP_SIZE)] = [0; BITS_TO_LONGS(ZPCI_DOMAIN_BITMAP_SIZE)];
static mut zpci_domain_lock: spinlock_t = __SPIN_LOCK_UNLOCKED;

// #define ZPCI_IOMAP_ENTRIES min((ZPCI_NR_DEVICES * PCI_STD_NUM_BARS / 2), ZPCI_IOMAP_MAX_ENTRIES)
const ZPCI_IOMAP_ENTRIES: usize = if ZPCI_NR_DEVICES * PCI_STD_NUM_BARS / 2 < ZPCI_IOMAP_MAX_ENTRIES { ZPCI_NR_DEVICES * PCI_STD_NUM_BARS / 2 } else { ZPCI_IOMAP_MAX_ENTRIES };

pub static mut s390_pci_no_rid: u32 = 0;
static mut zpci_iomap_lock: spinlock_t = __SPIN_LOCK_UNLOCKED;
static mut zpci_iomap_bitmap: *mut unsigned_long = core::ptr::null_mut();
#[no_mangle]
pub static mut zpci_iomap_start: *mut zpci_iomap_entry = core::ptr::null_mut();
pub static mut have_mio: static_key_false = STATIC_KEY_FALSE_INIT;
static mut zdev_fmb_cache: *mut kmem_cache = core::ptr::null_mut();

#[no_mangle]
pub static mut zpci_aipb: *mut zpci_sic_iib = core::ptr::null_mut();
#[no_mangle]
pub static mut zpci_aif_sbv: *mut airq_iv = core::ptr::null_mut();

pub unsafe fn zpci_zdev_put(zdev: *mut zpci_dev) {
    if zdev.is_null() { return; }
    mutex_lock(&mut zpci_add_remove_lock);
    kref_put_lock(&mut (*zdev).kref, zpci_release_device, &mut zpci_list_lock);
    mutex_unlock(&mut zpci_add_remove_lock);
}

pub unsafe fn get_zdev_by_fid(fid: u32) -> *mut zpci_dev {
    let mut zdev: *mut zpci_dev = core::ptr::null_mut();
    spin_lock(&mut zpci_list_lock);
    list_for_each_entry!(tmp, &mut zpci_list, entry, {
        if (*tmp).fid == fid { zdev = tmp; zpci_zdev_get(zdev); break; }
    });
    spin_unlock(&mut zpci_list_lock);
    zdev
}

pub unsafe fn zpci_remove_reserved_devices() {
    let mut state: zpci_state = core::mem::zeroed();
    let mut remove: list_head = LIST_HEAD_INIT;
    spin_lock(&mut zpci_list_lock);
    list_for_each_entry_safe!(zdev, tmp, &mut zpci_list, entry, {
        if (*zdev).state == ZPCI_FN_STATE_STANDBY && clp_get_state((*zdev).fid, &mut state) == 0 && state == ZPCI_FN_STATE_RESERVED {
            list_move_tail(&mut (*zdev).entry, &mut remove);
        }
    });
    spin_unlock(&mut zpci_list_lock);
    list_for_each_entry_safe!(zdev, tmp, &mut remove, entry, { zpci_device_reserved(zdev); });
}

pub unsafe fn pci_domain_nr(bus: *mut pci_bus) -> i32 { (*(bus as *mut zpci_bus)).domain_nr }
pub unsafe fn pci_proc_domain(bus: *mut pci_bus) -> i32 { pci_domain_nr(bus) }

pub unsafe fn zpci_register_ioat(zdev: *mut zpci_dev, dmaas: u8, base: u64, limit: u64, iota: u64, status: *mut u8) -> i32 {
    let req = ZPCI_CREATE_REQ((*zdev).fh, dmaas, ZPCI_MOD_FC_REG_IOAT);
    let mut fib: zpci_fib = core::mem::zeroed();
    fib.pba = base;
    fib.pal = if (*zdev).pft == PCI_FUNC_TYPE_ISM && limit > base { limit + (1 << 12) } else { limit };
    fib.iota = iota; fib.gd = (*zdev).gisa;
    let cc = zpci_mod_fc(req, &mut fib, status);
    if cc != 0 { zpci_dbg(3, "reg ioat fid:%x, cc:%d, status:%d\\n", (*zdev).fid, cc, *status); }
    cc
}

pub unsafe fn zpci_unregister_ioat(zdev: *mut zpci_dev, dmaas: u8) -> i32 {
    let req = ZPCI_CREATE_REQ((*zdev).fh, dmaas, ZPCI_MOD_FC_DEREG_IOAT);
    let mut fib: zpci_fib = core::mem::zeroed(); let mut status = 0u8; fib.gd = (*zdev).gisa;
    let cc = zpci_mod_fc(req, &mut fib, &mut status);
    if cc != 0 { zpci_dbg(3, "unreg ioat fid:%x, cc:%d, status:%d\\n", (*zdev).fid, cc, status); } cc
}

pub unsafe fn zpci_cfg_load(zdev: *mut zpci_dev, offset: i32, val: *mut u32, len: u8) -> i32 {
    let req = ZPCI_CREATE_REQ((*zdev).fh, ZPCI_PCIAS_CFGSPC, len); let mut data = 0u64;
    if !zdev_enabled(zdev) { PCI_SET_ERROR_RESPONSE(val); return -ENODEV; }
    let rc = __zpci_load(&mut data, req, offset); if rc != 0 { PCI_SET_ERROR_RESPONSE(val); return rc; }
    data = le64_to_cpu(data); data >>= (8 - len) * 8; *val = data as u32; 0
}

pub unsafe fn zpci_cfg_store(zdev: *mut zpci_dev, offset: i32, val: u32, len: u8) -> i32 {
    let req = ZPCI_CREATE_REQ((*zdev).fh, ZPCI_PCIAS_CFGSPC, len); if !zdev_enabled(zdev) { return -ENODEV; }
    let mut data = (val as u64) << ((8 - len) * 8); data = cpu_to_le64(data); __zpci_store(data, req, offset)
}

pub unsafe fn pcibios_align_resource(_data: *mut core::ffi::c_void, _res: *const resource, _empty_res: *const resource, _size: resource_size_t, _align: resource_size_t) -> resource_size_t { 0 }

pub unsafe fn ioremap_prot(phys_addr: phys_addr_t, size: usize, prot: pgprot_t) -> *mut core::ffi::c_void {
    if !static_branch_unlikely(&have_mio) { return phys_addr as *mut core::ffi::c_void; }
    generic_ioremap_prot(phys_addr, size, prot)
}
pub unsafe fn iounmap(addr: *mut core::ffi::c_void) { if static_branch_likely(&have_mio) { generic_iounmap(addr); } }

pub unsafe fn pci_iomap_range(pdev: *mut pci_dev, bar: i32, offset: usize, _max: usize) -> *mut core::ffi::c_void {
    if bar >= PCI_STD_NUM_BARS || pci_resource_len(pdev, bar) == 0 { return core::ptr::null_mut(); }
    if static_branch_likely(&have_mio) { let zdev = to_zpci(pdev); let p = ioremap((*zdev).bars[bar as usize].mio_wt as usize, pci_resource_len(pdev, bar)); return if p.is_null() { p } else { (p as *mut u8).add(offset) as *mut core::ffi::c_void }; }
    let zdev = to_zpci(pdev); let idx = (*zdev).bars[bar as usize].map_idx; spin_lock(&mut zpci_iomap_lock); (*zpci_iomap_start.add(idx)).count += 1; (*zpci_iomap_start.add(idx)).fh = (*zdev).fh; (*zpci_iomap_start.add(idx)).bar = bar; spin_unlock(&mut zpci_iomap_lock); (ZPCI_ADDR(idx) + offset) as *mut core::ffi::c_void
}
pub unsafe fn pci_iomap(pdev: *mut pci_dev, bar: i32, maxlen: usize) -> *mut core::ffi::c_void { pci_iomap_range(pdev, bar, 0, maxlen) }
pub unsafe fn pci_iomap_wc_range(pdev: *mut pci_dev, bar: i32, offset: usize, max: usize) -> *mut core::ffi::c_void { pci_iomap_range(pdev, bar, offset, max) }
pub unsafe fn pci_iomap_wc(pdev: *mut pci_dev, bar: i32, maxlen: usize) -> *mut core::ffi::c_void { pci_iomap_wc_range(pdev, bar, 0, maxlen) }
pub unsafe fn pci_iounmap(pdev: *mut pci_dev, addr: *mut core::ffi::c_void) { if static_branch_likely(&have_mio) { iounmap(addr); } else { let idx = ZPCI_IDX(addr); spin_lock(&mut zpci_iomap_lock); (*zpci_iomap_start.add(idx)).count -= 1; if (*zpci_iomap_start.add(idx)).count == 0 { (*zpci_iomap_start.add(idx)).fh = 0; (*zpci_iomap_start.add(idx)).bar = 0; } spin_unlock(&mut zpci_iomap_lock); } }

pub unsafe fn pci_read(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, val: *mut u32) -> i32 { let zdev = zdev_from_bus(bus, devfn); if zdev.is_null() || zpci_cfg_load(zdev, where_, val, size as u8) != 0 { PCIBIOS_DEVICE_NOT_FOUND } else { PCIBIOS_SUCCESSFUL } }
pub unsafe fn pci_write(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, val: u32) -> i32 { let zdev = zdev_from_bus(bus, devfn); if zdev.is_null() || zpci_cfg_store(zdev, where_, val, size as u8) != 0 { PCIBIOS_DEVICE_NOT_FOUND } else { PCIBIOS_SUCCESSFUL } }
pub static mut pci_root_ops: pci_ops = pci_ops { read: pci_read, write: pci_write };

pub unsafe fn zpci_update_fh(zdev: *mut zpci_dev, fh: u32) { if fh == 0 || (*zdev).fh == fh { return; } (*zdev).fh = fh; if zpci_use_mio(zdev) { return; } if (*zdev).has_resources != 0 && zdev_enabled(zdev) { for bar in 0..PCI_STD_NUM_BARS { if (*zdev).bars[bar].size != 0 { let idx = (*zdev).bars[bar].map_idx; if (*zpci_iomap_start.add(idx)).count != 0 { WRITE_ONCE!((*zpci_iomap_start.add(idx)).fh, (*zdev).fh); } } } } }

pub unsafe fn zpci_enable_device(zdev: *mut zpci_dev) -> i32 { let mut fh = (*zdev).fh; if clp_enable_fh(zdev, &mut fh, ZPCI_NR_DMA_SPACES) != 0 { -EIO } else { zpci_update_fh(zdev, fh); 0 } }
pub unsafe fn zpci_reenable_device(zdev: *mut zpci_dev) -> i32 { let rc = zpci_enable_device(zdev); if rc != 0 { return rc; } if (*zdev).msi_nr_irqs > 0 { let rc = zpci_set_irq(zdev); if rc != 0 { return rc; } } let mut status = 0u8; let rc = zpci_iommu_register_ioat(zdev, &mut status); if rc != 0 { zpci_disable_device(zdev); } rc }
pub unsafe fn zpci_disable_device(zdev: *mut zpci_dev) -> i32 { let mut fh = (*zdev).fh; let cc = clp_disable_fh(zdev, &mut fh); if cc == 0 { zpci_update_fh(zdev, fh); 0 } else if cc == CLP_RC_SETPCIFN_ALRDY { let rc = clp_refresh_fh((*zdev).fid, &mut fh); if rc == 0 { zpci_update_fh(zdev, fh); -EINVAL } else { 0 } } else { -EIO } }

pub unsafe fn zpci_is_device_configured(zdev: *mut zpci_dev) -> bool { (*zdev).state != ZPCI_FN_STATE_RESERVED && (*zdev).state != ZPCI_FN_STATE_STANDBY }
pub unsafe fn zpci_scan_configured_device(zdev: *mut zpci_dev, fh: u32) -> i32 { zpci_update_fh(zdev, fh); zpci_bus_scan_device(zdev) }
pub unsafe fn zpci_report_error(pdev: *mut pci_dev, report: *mut zpci_report_error_header) -> i32 { let zdev = to_zpci(pdev); sclp_pci_report(report, (*zdev).fh, (*zdev).fid) }

// The remaining subsystem lifecycle functions retain their C implementation's external dependencies.
pub unsafe fn zpci_clear_error_state(zdev: *mut zpci_dev) -> i32 { let req = ZPCI_CREATE_REQ((*zdev).fh, 0, ZPCI_MOD_FC_RESET_ERROR); let mut fib: zpci_fib = core::mem::zeroed(); let mut status = 0u8; if zpci_mod_fc(req, &mut fib, &mut status) != 0 { return -EIO; } 0 }
pub unsafe fn zpci_reset_load_store_blocked(zdev: *mut zpci_dev) -> i32 { let req = ZPCI_CREATE_REQ((*zdev).fh, 0, ZPCI_MOD_FC_RESET_BLOCK); let mut fib: zpci_fib = core::mem::zeroed(); let mut status = 0u8; if zpci_mod_fc(req, &mut fib, &mut status) != 0 { return -EIO; } 0 }

pub unsafe fn zpci_hot_reset_device(zdev: *mut zpci_dev) -> i32 { lockdep_assert_held(&(*zdev).state_lock); if zdev_enabled(zdev) { let mut rc = zpci_disable_device(zdev); if rc == -EINVAL { rc = 0; } if rc != 0 { return rc; } } zpci_reenable_device(zdev) }

pub unsafe fn zpci_create_device(fid: u32, fh: u32, state: zpci_state) -> *mut zpci_dev { let zdev = kzalloc_obj::<zpci_dev>(); if zdev.is_null() { return ERR_PTR(-ENOMEM); } (*zdev).fid = fid; (*zdev).fh = fh; let rc = clp_query_pci_fn(zdev); if rc != 0 { kfree(zdev as *mut core::ffi::c_void); return ERR_PTR(rc); } (*zdev).state = state; mutex_init(&mut (*zdev).state_lock); mutex_init(&mut (*zdev).fmb_lock); mutex_init(&mut (*zdev).kzdev_lock); mutex_init(&mut (*zdev).pending_errs_lock); zdev }

pub unsafe fn zpci_add_device(zdev: *mut zpci_dev) -> i32 { mutex_lock(&mut zpci_add_remove_lock); let mut rc = zpci_init_iommu(zdev); if rc == 0 { rc = zpci_bus_device_register(zdev, &mut pci_root_ops); } if rc == 0 { kref_init(&mut (*zdev).kref); spin_lock(&mut zpci_list_lock); list_add_tail(&mut (*zdev).entry, &mut zpci_list); spin_unlock(&mut zpci_list_lock); } else if rc != 0 { zpci_destroy_iommu(zdev); } mutex_unlock(&mut zpci_add_remove_lock); rc }

pub unsafe fn zpci_deconfigure_device(zdev: *mut zpci_dev) -> i32 { lockdep_assert_held(&(*zdev).state_lock); if (*zdev).state != ZPCI_FN_STATE_CONFIGURED { return 0; } if !(*zdev).zbus.is_null() && !(*(*zdev).zbus).bus.is_null() { zpci_bus_remove_device(zdev, false); } if zdev_enabled(zdev) { let rc = zpci_disable_device(zdev); if rc != 0 { return rc; } } let rc = sclp_pci_deconfigure((*zdev).fid); if rc != 0 { return rc; } (*zdev).state = ZPCI_FN_STATE_STANDBY; 0 }

pub unsafe fn zpci_device_reserved(zdev: *mut zpci_dev) { lockdep_assert_held(&(*zdev).state_lock); if (*zdev).state == ZPCI_FN_STATE_RESERVED { return; } (*zdev).state = ZPCI_FN_STATE_RESERVED; zpci_zdev_put(zdev); }
pub unsafe fn zpci_release_device(kref: *mut kref) { let zdev = container_of!(kref, zpci_dev, kref); list_del(&mut (*zdev).entry); spin_unlock(&mut zpci_list_lock); if (*zdev).has_hp_slot != 0 { zpci_exit_slot(zdev); } if (*zdev).has_resources != 0 { zpci_cleanup_bus_resources(zdev); } zpci_bus_device_unregister(zdev); zpci_destroy_iommu(zdev); kfree_rcu!(zdev, rcu); }

pub unsafe fn zpci_is_enabled() -> bool { s390_pci_initialized != 0 }
static mut s390_pci_probe: u32 = 1;
pub static mut s390_pci_force_floating: u32 = 0;
static mut s390_pci_initialized: u32 = 0;
pub unsafe fn pcibios_setup(str_: *mut i8) -> *mut i8 { if strcmp(str_, c"off") == 0 { s390_pci_probe = 0; core::ptr::null_mut() } else if strcmp(str_, c"nomio") == 0 { clear_machine_feature(MFEATURE_PCI_MIO); core::ptr::null_mut() } else if strcmp(str_, c"force_floating") == 0 { s390_pci_force_floating = 1; core::ptr::null_mut() } else if strcmp(str_, c"norid") == 0 { s390_pci_no_rid = 1; core::ptr::null_mut() } else { str_ } }

pub unsafe fn zpci_scan_devices() -> i32 { let mut scan_list: list_head = LIST_HEAD_INIT; let rc = clp_scan_pci_devices(&mut scan_list); if rc != 0 { return rc; } list_for_each_entry_safe!(zdev, tmp, &mut scan_list, entry, { list_del_init(&mut (*zdev).entry); if zpci_add_device(zdev) != 0 { kfree(zdev as *mut core::ffi::c_void); } }); let mut zbus: *mut zpci_bus = core::ptr::null_mut(); zpci_bus_for_each!(zbus, { zpci_bus_scan_bus(zbus); }); 0 }

pub unsafe fn pci_base_init() -> i32 { if s390_pci_probe == 0 { return 0; } if !test_facility(69) || !test_facility(71) { return 0; } if test_machine_feature(MFEATURE_PCI_MIO) { static_branch_enable(&mut have_mio); system_ctl_set_bit(2, CR2_MIO_ADDRESSING_BIT); } let mut rc = zpci_debug_init(); if rc != 0 { return rc; } rc = zpci_mem_init(); if rc != 0 { zpci_debug_exit(); return rc; } rc = zpci_irq_init(); if rc != 0 { zpci_mem_exit(); zpci_debug_exit(); return rc; } rc = zpci_scan_devices(); if rc != 0 { zpci_irq_exit(); zpci_mem_exit(); zpci_debug_exit(); return rc; } rc = zpci_fw_sysfs_init(); if rc != 0 { zpci_irq_exit(); zpci_mem_exit(); zpci_debug_exit(); return rc; } s390_pci_initialized = 1; 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
