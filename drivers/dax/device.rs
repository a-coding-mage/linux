// SPDX-License-Identifier: GPL-2.0
/* Copyright(c) 2016-2018 Intel Corporation. All rights reserved. */
// Dependencies supplied by the corresponding kernel headers and local modules.

unsafe fn __check_vma(
    dev_dax: *mut dev_dax,
    flags: vma_flags_t,
    start: c_ulong,
    end: c_ulong,
    file: *mut file,
    func: *const c_char,
) -> c_int {
    let dev = &mut (*dev_dax).dev as *mut device;
    let mask: c_ulong;

    if !dax_alive((*dev_dax).dax_dev) { return -ENXIO; }
    if !vma_flags_test_any(&flags, VMA_MAYSHARE_BIT) {
        dev_info_ratelimited(dev, b"%s: %s: fail, attempted private mapping\0", current.comm, func);
        return -EINVAL;
    }
    mask = (*dev_dax).align - 1;
    if (start & mask) != 0 || (end & mask) != 0 {
        dev_info_ratelimited(dev, b"%s: %s: fail, unaligned vma (%#lx - %#lx, %#lx)\0", current.comm, func, start, end, mask);
        return -EINVAL;
    }
    if !file_is_dax(file) {
        dev_info_ratelimited(dev, b"%s: %s: fail, vma is not DAX capable\0", current.comm, func);
        return -EINVAL;
    }
    0
}

unsafe fn check_vma(dev_dax: *mut dev_dax, vma: *mut vm_area_struct, func: *const c_char) -> c_int {
    __check_vma(dev_dax, (*vma).flags, (*vma).vm_start, (*vma).vm_end, (*vma).vm_file, func)
}

unsafe fn dax_set_mapping(vmf: *mut vm_fault, pfn: c_ulong, fault_size: c_ulong) {
    let mut nr_pages = fault_size / PAGE_SIZE;
    let filp = (*(*vmf).vma).vm_file;
    let dev_dax = (*filp).private_data as *mut dev_dax;
    if (*(*dev_dax).pgmap).vmemmap_shift != 0 { nr_pages = 1; }
    let pgoff = linear_page_index((*vmf).vma, ALIGN_DOWN((*vmf).address, fault_size));
    for i in 0..nr_pages {
        let folio = pfn_folio(pfn + i);
        if !(*folio).mapping.is_null() { continue; }
        (*folio).mapping = (*filp).f_mapping;
        (*folio).index = pgoff + i;
    }
}

unsafe fn __dev_dax_pte_fault(dev_dax: *mut dev_dax, vmf: *mut vm_fault) -> vm_fault_t {
    let dev = &mut (*dev_dax).dev as *mut device;
    let fault_size = PAGE_SIZE;
    if check_vma(dev_dax, (*vmf).vma, c"__dev_dax_pte_fault".as_ptr()) != 0 { return VM_FAULT_SIGBUS; }
    if (*dev_dax).align > PAGE_SIZE { dev_dbg(dev, c"alignment (%#x) > fault size (%#x)".as_ptr(), (*dev_dax).align, fault_size); return VM_FAULT_SIGBUS; }
    if fault_size != (*dev_dax).align { return VM_FAULT_SIGBUS; }
    let phys = dax_pgoff_to_phys(dev_dax, (*vmf).pgoff, PAGE_SIZE);
    if phys == -1 { dev_dbg(dev, c"pgoff_to_phys(%#lx) failed".as_ptr(), (*vmf).pgoff); return VM_FAULT_SIGBUS; }
    let pfn = PHYS_PFN(phys);
    dax_set_mapping(vmf, pfn, fault_size);
    vmf_insert_page_mkwrite(vmf, pfn_to_page(pfn), (*vmf).flags & FAULT_FLAG_WRITE)
}

unsafe fn __dev_dax_pmd_fault(dev_dax: *mut dev_dax, vmf: *mut vm_fault) -> vm_fault_t {
    let pmd_addr = (*vmf).address & PMD_MASK;
    let dev = &mut (*dev_dax).dev as *mut device;
    let fault_size = PMD_SIZE;
    if check_vma(dev_dax, (*vmf).vma, c"__dev_dax_pmd_fault".as_ptr()) != 0 { return VM_FAULT_SIGBUS; }
    if (*dev_dax).align > PMD_SIZE { dev_dbg(dev, c"alignment (%#x) > fault size (%#x)".as_ptr(), (*dev_dax).align, fault_size); return VM_FAULT_SIGBUS; }
    if fault_size < (*dev_dax).align { return VM_FAULT_SIGBUS; } else if fault_size > (*dev_dax).align { return VM_FAULT_FALLBACK; }
    if pmd_addr < (*(*vmf).vma).vm_start || pmd_addr + PMD_SIZE > (*(*vmf).vma).vm_end { return VM_FAULT_SIGBUS; }
    let pgoff = linear_page_index((*vmf).vma, pmd_addr);
    let phys = dax_pgoff_to_phys(dev_dax, pgoff, PMD_SIZE);
    if phys == -1 { dev_dbg(dev, c"pgoff_to_phys(%#lx) failed".as_ptr(), pgoff); return VM_FAULT_SIGBUS; }
    let pfn = PHYS_PFN(phys);
    dax_set_mapping(vmf, pfn, fault_size);
    vmf_insert_folio_pmd(vmf, page_folio(pfn_to_page(pfn)), (*vmf).flags & FAULT_FLAG_WRITE)
}

// CONFIG_HAVE_ARCH_TRANSPARENT_HUGEPAGE_PUD selects the full implementation.
unsafe fn __dev_dax_pud_fault(dev_dax: *mut dev_dax, vmf: *mut vm_fault) -> vm_fault_t {
    let pud_addr = (*vmf).address & PUD_MASK;
    let dev = &mut (*dev_dax).dev as *mut device;
    let fault_size = PUD_SIZE;
    if check_vma(dev_dax, (*vmf).vma, c"__dev_dax_pud_fault".as_ptr()) != 0 { return VM_FAULT_SIGBUS; }
    if (*dev_dax).align > PUD_SIZE { dev_dbg(dev, c"alignment (%#x) > fault size (%#x)".as_ptr(), (*dev_dax).align, fault_size); return VM_FAULT_SIGBUS; }
    if fault_size < (*dev_dax).align { return VM_FAULT_SIGBUS; } else if fault_size > (*dev_dax).align { return VM_FAULT_FALLBACK; }
    if pud_addr < (*(*vmf).vma).vm_start || pud_addr + PUD_SIZE > (*(*vmf).vma).vm_end { return VM_FAULT_SIGBUS; }
    let pgoff = linear_page_index((*vmf).vma, pud_addr);
    let phys = dax_pgoff_to_phys(dev_dax, pgoff, PUD_SIZE);
    if phys == -1 { dev_dbg(dev, c"pgoff_to_phys(%#lx) failed".as_ptr(), pgoff); return VM_FAULT_SIGBUS; }
    let pfn = PHYS_PFN(phys);
    dax_set_mapping(vmf, pfn, fault_size);
    vmf_insert_folio_pud(vmf, page_folio(pfn_to_page(pfn)), (*vmf).flags & FAULT_FLAG_WRITE)
}

unsafe fn dev_dax_huge_fault(vmf: *mut vm_fault, order: c_uint) -> vm_fault_t {
    let filp = (*(*vmf).vma).vm_file;
    let dev_dax = (*filp).private_data as *mut dev_dax;
    dev_dbg(&mut (*dev_dax).dev, c"%s: op=%s addr=%#lx order=%d".as_ptr(), current.comm, if (*vmf).flags & FAULT_FLAG_WRITE != 0 { c"write".as_ptr() } else { c"read".as_ptr() }, (*vmf).address & !((1UL << (order + PAGE_SHIFT)) - 1), order);
    let id = dax_read_lock();
    let rc = if order == 0 { __dev_dax_pte_fault(dev_dax, vmf) } else if order == PMD_ORDER { __dev_dax_pmd_fault(dev_dax, vmf) } else if order == PUD_ORDER { __dev_dax_pud_fault(dev_dax, vmf) } else { VM_FAULT_SIGBUS };
    dax_read_unlock(id); rc
}

unsafe fn dev_dax_fault(vmf: *mut vm_fault) -> vm_fault_t { dev_dax_huge_fault(vmf, 0) }
unsafe fn dev_dax_may_split(vma: *mut vm_area_struct, addr: c_ulong) -> c_int {
    let dev_dax = (*(*vma).vm_file).private_data as *mut dev_dax;
    if !IS_ALIGNED(addr, (*dev_dax).align) { return -EINVAL; } 0
}
unsafe fn dev_dax_pagesize(vma: *mut vm_area_struct) -> c_ulong { (*((*(*vma).vm_file).private_data as *mut dev_dax)).align }

static dax_vm_ops: vm_operations_struct = vm_operations_struct { fault: Some(dev_dax_fault), huge_fault: Some(dev_dax_huge_fault), may_split: Some(dev_dax_may_split), pagesize: Some(dev_dax_pagesize) };

unsafe fn dax_mmap_prepare(desc: *mut vm_area_desc) -> c_int {
    let filp = (*desc).file; let dev_dax = (*filp).private_data as *mut dev_dax;
    dev_dbg(&mut (*dev_dax).dev, c"trace".as_ptr());
    let id = dax_read_lock(); let rc = __check_vma(dev_dax, (*desc).vma_flags, (*desc).start, (*desc).end, filp, c"dax_mmap_prepare".as_ptr()); dax_read_unlock(id);
    if rc != 0 { return rc; } (*desc).vm_ops = &dax_vm_ops; vma_desc_set_flags(desc, VMA_HUGEPAGE_BIT); 0
}

unsafe fn dax_get_unmapped_area(filp: *mut file, addr: c_ulong, len: c_ulong, pgoff: c_ulong, flags: c_ulong) -> c_ulong {
    let dev_dax = if !filp.is_null() { (*filp).private_data as *mut dev_dax } else { core::ptr::null_mut() };
    if dev_dax.is_null() || addr != 0 { return mm_get_unmapped_area(filp, addr, len, pgoff, flags); }
    let align = (*dev_dax).align; let off = pgoff << PAGE_SHIFT; let off_end = off + len; let off_align = round_up(off, align);
    if off_end <= off_align || off_end - off_align < align { return mm_get_unmapped_area(filp, addr, len, pgoff, flags); }
    let len_align = len + align; if off + len_align < off { return mm_get_unmapped_area(filp, addr, len, pgoff, flags); }
    let addr_align = mm_get_unmapped_area(filp, addr, len_align, pgoff, flags);
    if !IS_ERR_VALUE(addr_align) { return addr_align + ((off - addr_align) & (align - 1)); }
    mm_get_unmapped_area(filp, addr, len, pgoff, flags)
}

static dev_dax_aops: address_space_operations = address_space_operations { dirty_folio: Some(noop_dirty_folio) };

unsafe fn dax_open(inode: *mut inode, filp: *mut file) -> c_int {
    let dax_dev = inode_dax(inode); let dax_inode_ = dax_inode(dax_dev); let dev_dax = dax_get_private(dax_dev);
    dev_dbg(&mut (*dev_dax).dev, c"trace".as_ptr()); (*inode).i_mapping = (*dax_inode_).i_mapping; (*(*inode).i_mapping).host = dax_inode_; (*(*inode).i_mapping).a_ops = &dev_dax_aops; (*filp).f_mapping = (*inode).i_mapping; (*filp).f_wb_err = filemap_sample_wb_err((*filp).f_mapping); (*filp).f_sb_err = file_sample_sb_err(filp); (*filp).private_data = dev_dax as *mut c_void; (*inode).i_flags = S_DAX; 0
}
unsafe fn dax_release(_inode: *mut inode, filp: *mut file) -> c_int { let dev_dax = (*filp).private_data as *mut dev_dax; dev_dbg(&mut (*dev_dax).dev, c"trace".as_ptr()); 0 }

static dax_fops: file_operations = file_operations { llseek: Some(noop_llseek), owner: THIS_MODULE, open: Some(dax_open), release: Some(dax_release), get_unmapped_area: Some(dax_get_unmapped_area), mmap_prepare: Some(dax_mmap_prepare), fop_flags: FOP_MMAP_SYNC };

unsafe fn dev_dax_cdev_del(cdev: *mut c_void) { cdev_del(cdev as *mut cdev); }
unsafe fn dev_dax_kill(dev_dax: *mut c_void) { kill_dev_dax(dev_dax as *mut dev_dax); }

unsafe fn dev_dax_probe(dev_dax: *mut dev_dax) -> c_int {
    let dax_dev = (*dev_dax).dax_dev; let dev = &mut (*dev_dax).dev as *mut device; let mut pgmap: *mut dev_pagemap;
    if static_dev_dax(dev_dax) { if (*dev_dax).nr_range > 1 { dev_warn(dev, c"static pgmap / multi-range device conflict".as_ptr()); return -EINVAL; } pgmap = (*dev_dax).pgmap; }
    else { if !(*dev_dax).pgmap.is_null() { dev_warn(dev, c"dynamic-dax with pre-populated page map".as_ptr()); return -EINVAL; } pgmap = devm_kzalloc(dev, struct_size(pgmap, ranges, (*dev_dax).nr_range - 1), GFP_KERNEL); if pgmap.is_null() { return -ENOMEM; } (*pgmap).nr_range = (*dev_dax).nr_range; (*dev_dax).pgmap = pgmap; for i in 0..(*dev_dax).nr_range { (*pgmap).ranges[i] = (*dev_dax).ranges[i].range; } }
    for i in 0..(*dev_dax).nr_range { let range = &(*dev_dax).ranges[i].range; if !devm_request_mem_region(dev, range.start, range_len(range), dev_name(dev)) { dev_warn(dev, c"mapping%d: %#llx-%#llx could not reserve range".as_ptr(), i, range.start, range.end); return -EBUSY; } }
    (*pgmap).type_ = MEMORY_DEVICE_GENERIC; if (*dev_dax).align > PAGE_SIZE { (*pgmap).vmemmap_shift = order_base_2((*dev_dax).align >> PAGE_SHIFT); }
    let addr = devm_memremap_pages(dev, pgmap); if IS_ERR(addr) { return PTR_ERR(addr); }
    let inode = dax_inode(dax_dev); let cdev = (*inode).i_cdev; cdev_init(cdev, &dax_fops); (*cdev).owner = (*dev).driver.owner; cdev_set_parent(cdev, &(*dev).kobj); let mut rc = cdev_add(cdev, (*dev).devt, 1); if rc != 0 { return rc; }
    rc = devm_add_action_or_reset(dev, dev_dax_cdev_del, cdev as *mut c_void); if rc != 0 { return rc; } run_dax(dax_dev); devm_add_action_or_reset(dev, dev_dax_kill, dev_dax as *mut c_void)
}

static device_dax_driver: dax_device_driver = dax_device_driver { probe: Some(dev_dax_probe), type_: DAXDRV_DEVICE_TYPE };
unsafe fn dax_init() -> c_int { dax_driver_register(&device_dax_driver) }
unsafe fn dax_exit() { dax_driver_unregister(&device_dax_driver); }

// MODULE_AUTHOR("Intel Corporation"); MODULE_DESCRIPTION("Device DAX: direct access device driver");
// MODULE_LICENSE("GPL v2"); module_init(dax_init); module_exit(dax_exit); MODULE_ALIAS_DAX_DEVICE(0);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
