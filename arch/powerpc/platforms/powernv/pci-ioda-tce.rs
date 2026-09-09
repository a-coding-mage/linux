// SPDX-License-Identifier: GPL-2.0+
/*
 * TCE helpers for IODA PCI/PCIe on PowerNV platforms
 *
 * Copyright 2018 IBM Corp.
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version
 * 2 of the License, or (at your option) any later version.
 */

// Kernel and architecture dependencies are supplied by the surrounding tree.

pub unsafe extern "C" fn pnv_ioda_parse_tce_sizes(phb: *mut pnv_phb) -> c_ulong {
    let hose = (*phb).hose;
    let dn = (*hose).dn;
    let mut mask: c_ulong = 0;
    let mut val: u32 = 0;

    let count = of_property_count_u32_elems(dn, b"ibm,supported-tce-sizes\0".as_ptr() as *const c_char);
    if count <= 0 {
        mask = SZ_4K | SZ_64K;
        /* Add 16M for POWER8 by default */
        if cpu_has_feature(CPU_FTR_ARCH_207S) && !cpu_has_feature(CPU_FTR_ARCH_300) {
            mask |= SZ_16M | SZ_256M;
        }
        return mask;
    }

    for i in 0..count {
        let rc = of_property_read_u32_index(
            dn,
            b"ibm,supported-tce-sizes\0".as_ptr() as *const c_char,
            i,
            &mut val,
        );
        if rc == 0 {
            mask |= 1u64 << val;
        }
    }
    mask
}

pub unsafe extern "C" fn pnv_pci_setup_iommu_table(
    tbl: *mut iommu_table,
    tce_mem: *mut c_void,
    tce_size: u64,
    dma_offset: u64,
    page_shift: c_uint,
) {
    (*tbl).it_blocksize = 16;
    (*tbl).it_base = tce_mem as c_ulong;
    (*tbl).it_page_shift = page_shift;
    (*tbl).it_offset = dma_offset >> (*tbl).it_page_shift;
    (*tbl).it_index = 0;
    (*tbl).it_size = tce_size >> 3;
    (*tbl).it_busno = 0;
    (*tbl).it_type = TCE_PCI;
}

unsafe fn pnv_alloc_tce_level(nid: c_int, shift: c_uint) -> *mut be64 {
    let tce_mem: *mut page = alloc_pages_node(nid, GFP_ATOMIC | __GFP_NOWARN, shift - PAGE_SHIFT);
    if tce_mem.is_null() {
        pr_err!("Failed to allocate a TCE memory, level shift=%d\n", shift);
        return core::ptr::null_mut();
    }
    let addr = page_address(tce_mem) as *mut be64;
    memset(addr as *mut c_void, 0, 1usize << shift);
    addr
}

unsafe fn pnv_pci_ioda2_table_do_free_pages(addr: *mut be64, size: c_ulong, levels: c_uint);

unsafe fn pnv_tce(tbl: *mut iommu_table, user: bool, mut idx: c_long, alloc: bool) -> *mut be64 {
    let mut tmp = if user { (*tbl).it_userspace } else { (*tbl).it_base as *mut be64 };
    let mut level = (*tbl).it_indirect_levels;
    let shift = ilog2((*tbl).it_level_size);
    let mut mask = ((*tbl).it_level_size - 1) << (level * shift);

    while level != 0 {
        let n = ((idx as c_ulong & mask) >> (level * shift)) as usize;
        let mut tce = be64_to_cpu(core::ptr::read_volatile(tmp.add(n)));
        if tce == 0 {
            if !alloc { return core::ptr::null_mut(); }
            let tmp2 = pnv_alloc_tce_level((*tbl).it_nid, ilog2((*tbl).it_level_size) + 3);
            if tmp2.is_null() { return core::ptr::null_mut(); }
            tce = __pa(tmp2 as *const c_void) | TCE_PCI_READ | TCE_PCI_WRITE;
            let oldtce = be64_to_cpu(cmpxchg(&mut *tmp.add(n), 0, cpu_to_be64(tce)));
            if oldtce != 0 {
                pnv_pci_ioda2_table_do_free_pages(tmp2, ilog2((*tbl).it_level_size) + 3, 1);
                tce = oldtce;
            }
        }
        tmp = __va(tce & !(TCE_PCI_READ | TCE_PCI_WRITE)) as *mut be64;
        idx &= !(mask as c_long);
        mask >>= shift;
        level -= 1;
    }
    tmp.offset(idx)
}

pub unsafe extern "C" fn pnv_tce_build(tbl: *mut iommu_table, index: c_long, npages: c_long, uaddr: c_ulong, direction: dma_data_direction, _attrs: c_ulong) -> c_int {
    let mut proto_tce = iommu_direction_to_tce_perm(direction);
    let rpn = __pa(uaddr as *const c_void) >> (*tbl).it_page_shift;
    if proto_tce & TCE_PCI_WRITE != 0 { proto_tce |= TCE_PCI_READ; }
    for i in 0..npages {
        let newtce = proto_tce | ((rpn + i as u64) << (*tbl).it_page_shift);
        let idx = index - (*tbl).it_offset as c_long + i;
        *pnv_tce(tbl, false, idx, true) = cpu_to_be64(newtce);
    }
    0
}

#[cfg(CONFIG_IOMMU_API)]
pub unsafe extern "C" fn pnv_tce_xchg(tbl: *mut iommu_table, index: c_long, hpa: *mut c_ulong, direction: *mut dma_data_direction) -> c_long {
    let proto_tce = iommu_direction_to_tce_perm(*direction);
    let mut newtce = *hpa | proto_tce;
    let idx = index - (*tbl).it_offset as c_long;
    let mut ptce: *mut be64 = core::ptr::null_mut();
    BUG_ON!(*hpa & !IOMMU_PAGE_MASK(tbl));
    if *direction == DMA_NONE {
        ptce = pnv_tce(tbl, false, idx, false);
        if ptce.is_null() { *hpa = 0; return 0; }
    }
    if ptce.is_null() { ptce = pnv_tce(tbl, false, idx, true); if ptce.is_null() { return -ENOMEM; } }
    if newtce & TCE_PCI_WRITE != 0 { newtce |= TCE_PCI_READ; }
    let oldtce = be64_to_cpu(xchg(ptce, cpu_to_be64(newtce)));
    *hpa = oldtce & !(TCE_PCI_READ | TCE_PCI_WRITE);
    *direction = iommu_tce_direction(oldtce);
    0
}

#[cfg(CONFIG_IOMMU_API)]
pub unsafe extern "C" fn pnv_tce_useraddrptr(tbl: *mut iommu_table, index: c_long, alloc: bool) -> *mut be64 {
    if WARN_ON_ONCE!((*tbl).it_userspace.is_null()) { return core::ptr::null_mut(); }
    pnv_tce(tbl, true, index - (*tbl).it_offset as c_long, alloc)
}

pub unsafe extern "C" fn pnv_tce_free(tbl: *mut iommu_table, index: c_long, npages: c_long) {
    let mut i = 0;
    while i < npages {
        let idx = index - (*tbl).it_offset as c_long + i;
        let ptce = pnv_tce(tbl, false, idx, false);
        if !ptce.is_null() { *ptce = cpu_to_be64(0); } else { i |= (*tbl).it_level_size as c_long - 1; }
        i += 1;
    }
}

pub unsafe extern "C" fn pnv_tce_get(tbl: *mut iommu_table, index: c_long) -> c_ulong {
    let ptce = pnv_tce(tbl, false, index - (*tbl).it_offset as c_long, false);
    if ptce.is_null() { return 0; }
    be64_to_cpu(*ptce)
}

pub unsafe extern "C" fn pnv_pci_ioda2_table_free_pages(tbl: *mut iommu_table) {
    let size = if (*tbl).it_indirect_levels != 0 { (*tbl).it_level_size } else { (*tbl).it_size };
    if (*tbl).it_size == 0 { return; }
    pnv_pci_ioda2_table_do_free_pages((*tbl).it_base as *mut be64, size, (*tbl).it_indirect_levels);
    if !(*tbl).it_userspace.is_null() { pnv_pci_ioda2_table_do_free_pages((*tbl).it_userspace, size, (*tbl).it_indirect_levels); }
}

unsafe fn pnv_pci_ioda2_table_do_free_pages(addr: *mut be64, size: c_ulong, levels: c_uint) {
    let addr_ul = (addr as c_ulong) & !(TCE_PCI_READ | TCE_PCI_WRITE);
    if levels != 0 {
        let tmp = addr_ul as *mut u64;
        for i in 0..size {
            let hpa = be64_to_cpu(*tmp.add(i as usize));
            if hpa & (TCE_PCI_READ | TCE_PCI_WRITE) != 0 {
                pnv_pci_ioda2_table_do_free_pages(__va(hpa) as *mut be64, size, levels - 1);
            }
        }
    }
    free_pages(addr_ul, get_order(size << 3));
}

unsafe fn pnv_pci_ioda2_table_do_alloc_pages(nid: c_int, shift: c_uint, mut levels: c_uint, limit: c_ulong, current_offset: *mut c_ulong, total_allocated: *mut c_ulong) -> *mut be64 {
    let allocated = 1u64 << shift;
    let entries = 1u64 << (shift - 3);
    let addr = pnv_alloc_tce_level(nid, shift);
    *total_allocated += allocated;
    levels -= 1;
    if levels == 0 { *current_offset += allocated; return addr; }
    for i in 0..entries {
        let tmp = pnv_pci_ioda2_table_do_alloc_pages(nid, shift, levels, limit, current_offset, total_allocated);
        if tmp.is_null() { break; }
        *addr.add(i as usize) = cpu_to_be64(__pa(tmp as *const c_void) | TCE_PCI_READ | TCE_PCI_WRITE);
        if *current_offset >= limit { break; }
    }
    addr
}

pub unsafe extern "C" fn pnv_pci_ioda2_table_alloc_pages(nid: c_int, bus_offset: u64, page_shift: u32, window_size: u64, levels: u32, alloc_userspace_copy: bool, tbl: *mut iommu_table) -> c_long {
    let mut offset = 0; let mut total = 0; let mut total_uas = 0;
    let window_shift = ilog2(window_size); let entries_shift = window_shift - page_shift;
    let table_shift = core::cmp::max(entries_shift + 3, PAGE_SHIFT); let table_size = 1u64 << table_shift;
    if levels == 0 || levels > POWERNV_IOMMU_MAX_LEVELS || !is_power_of_2(window_size) { return -EINVAL; }
    let entries_shift = (entries_shift + levels - 1) / levels;
    let level_shift = core::cmp::max(entries_shift + 3, PAGE_SHIFT);
    if (level_shift - 3) * levels + page_shift >= 55 { return -EINVAL; }
    let addr = pnv_pci_ioda2_table_do_alloc_pages(nid, level_shift, 1, table_size, &mut offset, &mut total);
    if addr.is_null() { return -ENOMEM; }
    if levels == 1 && offset < table_size { pnv_pci_ioda2_table_do_free_pages(addr, 1u64 << (level_shift - 3), levels - 1); return -ENOMEM; }
    let mut uas = core::ptr::null_mut();
    if alloc_userspace_copy {
        offset = 0; uas = pnv_pci_ioda2_table_do_alloc_pages(nid, level_shift, 1, table_size, &mut offset, &mut total_uas);
        if uas.is_null() || (levels == 1 && (offset < table_size || total_uas != total)) {
            if !uas.is_null() { pnv_pci_ioda2_table_do_free_pages(uas, 1u64 << (level_shift - 3), levels - 1); }
            pnv_pci_ioda2_table_do_free_pages(addr, 1u64 << (level_shift - 3), levels - 1); return -ENOMEM;
        }
    }
    pnv_pci_setup_iommu_table(tbl, addr as *mut c_void, table_size, bus_offset, page_shift);
    (*tbl).it_level_size = 1u64 << (level_shift - 3); (*tbl).it_indirect_levels = levels - 1; (*tbl).it_userspace = uas; (*tbl).it_nid = nid;
    0
}

pub unsafe extern "C" fn pnv_pci_unlink_table_and_group(tbl: *mut iommu_table, table_group: *mut iommu_table_group) {
    if tbl.is_null() || table_group.is_null() { return; }
    let mut found = false; rcu_read_lock();
    list_for_each_entry_rcu!(tgl, (*tbl).it_group_list, next, { if (*tgl).table_group == table_group { list_del_rcu!(&mut (*tgl).next); kfree_rcu!(tgl, rcu); found = true; break; } });
    rcu_read_unlock(); if WARN_ON!(!found) { return; }
    found = false; for i in 0..IOMMU_TABLE_GROUP_MAX_TABLES { if (*table_group).tables[i] == tbl { iommu_tce_table_put(tbl); (*table_group).tables[i] = core::ptr::null_mut(); found = true; break; } } WARN_ON!(!found);
}

pub unsafe extern "C" fn pnv_pci_link_table_and_group(node: c_int, num: c_int, tbl: *mut iommu_table, table_group: *mut iommu_table_group) -> c_long {
    if WARN_ON!(tbl.is_null() || table_group.is_null()) { return -EINVAL; }
    let tgl = kzalloc_node(core::mem::size_of::<iommu_table_group_link>(), GFP_KERNEL, node); if tgl.is_null() { return -ENOMEM; }
    (*tgl).table_group = table_group; list_add_rcu!(&mut (*tgl).next, &mut (*tbl).it_group_list); (*table_group).tables[num as usize] = iommu_tce_table_get(tbl); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
