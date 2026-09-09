// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * IOMMU helpers in MMU context.
 *
 * Copyright (C) 2015 IBM Corp. <aik@ozlabs.ru>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

static mut MEM_LIST_MUTEX: Mutex = DEFINE_MUTEX!();

const MM_IOMMU_TABLE_GROUP_PAGE_DIRTY: u64 = 0x1;
const MM_IOMMU_TABLE_GROUP_PAGE_MASK: u64 = !(SZ_4K - 1);
const MM_IOMMU_TABLE_INVALID_HPA: u64 = u64::MAX;

#[repr(C)]
pub union MmIommuPages {
    pub hpages: *mut *mut page,
    pub hpas: *mut phys_addr_t,
}

#[repr(C)]
pub struct mm_iommu_table_group_mem_t {
    pub next: list_head,
    pub rcu: rcu_head,
    pub used: c_ulong,
    pub mapped: atomic64_t,
    pub pageshift: c_uint,
    pub ua: u64,
    pub entries: u64,
    pub pages: MmIommuPages,
    pub dev_hpa: u64,
}

pub unsafe fn mm_iommu_preregistered(mm: *mut mm_struct) -> bool {
    !list_empty(unsafe { &mut (*(*mm).context).iommu_group_mem_list })
}

unsafe fn mm_iommu_do_alloc(mm: *mut mm_struct, ua: c_ulong, entries: c_ulong,
                            dev_hpa: c_ulong,
                            pmem: *mut *mut mm_iommu_table_group_mem_t) -> c_long {
    let mut mem: *mut mm_iommu_table_group_mem_t;
    let mut mem2: *mut mm_iommu_table_group_mem_t;
    let mut i: c_long;
    let mut ret: c_long;
    let mut locked_entries: c_long = 0;
    let mut pinned: c_long = 0;
    let mut pageshift: c_uint;
    let mut entry: c_ulong;
    let mut chunk: c_ulong;

    if dev_hpa == MM_IOMMU_TABLE_INVALID_HPA as c_ulong {
        ret = account_locked_vm(mm, entries, true);
        if ret != 0 { return ret; }
        locked_entries = entries as c_long;
    }
    mem = kzalloc_obj::<mm_iommu_table_group_mem_t>();
    if mem.is_null() { ret = -ENOMEM; goto_unlock_exit!(); }

    if dev_hpa != MM_IOMMU_TABLE_INVALID_HPA as c_ulong {
        (*mem).pageshift = __ffs(dev_hpa | (entries << PAGE_SHIFT));
        (*mem).dev_hpa = dev_hpa as u64;
        goto_good_exit!();
    }
    (*mem).dev_hpa = MM_IOMMU_TABLE_INVALID_HPA;
    (*mem).pageshift = __ffs(ua | (entries << PAGE_SHIFT));
    (*mem).pages.hpas = vzalloc(array_size(entries, core::mem::size_of::<phys_addr_t>()));
    if (*mem).pages.hpas.is_null() { kfree(mem); ret = -ENOMEM; goto_unlock_exit!(); }

    mmap_read_lock(mm);
    chunk = (1UL << (PAGE_SHIFT + MAX_PAGE_ORDER)) / core::mem::size_of::<*mut vm_area_struct>() as c_ulong;
    chunk = min(chunk, entries);
    entry = 0;
    while entry < entries {
        let n = min(entries - entry, chunk);
        ret = pin_user_pages(ua + (entry << PAGE_SHIFT), n, FOLL_WRITE | FOLL_LONGTERM,
                             (*mem).pages.hpages.add(entry as usize));
        if ret == n as c_long { pinned += n as c_long; entry += chunk; continue; }
        if ret > 0 { pinned += ret; }
        break;
    }
    mmap_read_unlock(mm);
    if pinned != entries as c_long {
        if ret == 0 { ret = -EFAULT; }
        goto_free_exit!();
    }

    // good_exit:
    atomic64_set(&mut (*mem).mapped, 1);
    (*mem).used = 1; (*mem).ua = ua as u64; (*mem).entries = entries as u64;
    mutex_lock(&mut MEM_LIST_MUTEX);
    list_for_each_entry_rcu!(mem2, (*mm).context.iommu_group_mem_list, next, lockdep_is_held(&MEM_LIST_MUTEX), {
        if (*mem2).ua < ua as u64 + (entries << PAGE_SHIFT) &&
           ua as u64 < (*mem2).ua + ((*mem2).entries << PAGE_SHIFT) {
            ret = -EINVAL; mutex_unlock(&mut MEM_LIST_MUTEX); goto_free_exit!();
        }
    });
    if (*mem).dev_hpa == MM_IOMMU_TABLE_INVALID_HPA {
        pageshift = PAGE_SHIFT;
        i = 0;
        while i < entries as c_long {
            let page = *(*mem).pages.hpages.offset(i as isize);
            if (*mem).pageshift > PAGE_SHIFT && PageHuge(page) { pageshift = page_shift(compound_head(page)); }
            (*mem).pageshift = min((*mem).pageshift, pageshift);
            *(*mem).pages.hpas.offset(i as isize) = page_to_pfn(page) << PAGE_SHIFT;
            i += 1;
        }
    }
    list_add_rcu(&mut (*mem).next, &mut (*mm).context.iommu_group_mem_list);
    mutex_unlock(&mut MEM_LIST_MUTEX); *pmem = mem; return 0;

    // free_exit:
    unpin_user_pages((*mem).pages.hpages, pinned as c_ulong);
    vfree((*mem).pages.hpas as *mut c_void); kfree(mem);
    // unlock_exit:
    account_locked_vm(mm, locked_entries as c_ulong, false); ret
}

pub unsafe fn mm_iommu_new(mm: *mut mm_struct, ua: c_ulong, entries: c_ulong,
                           pmem: *mut *mut mm_iommu_table_group_mem_t) -> c_long {
    mm_iommu_do_alloc(mm, ua, entries, MM_IOMMU_TABLE_INVALID_HPA as c_ulong, pmem)
}

pub unsafe fn mm_iommu_newdev(mm: *mut mm_struct, ua: c_ulong, entries: c_ulong,
                              dev_hpa: c_ulong, pmem: *mut *mut mm_iommu_table_group_mem_t) -> c_long {
    mm_iommu_do_alloc(mm, ua, entries, dev_hpa, pmem)
}

unsafe fn mm_iommu_unpin(mem: *mut mm_iommu_table_group_mem_t) {
    if (*mem).pages.hpas.is_null() { return; }
    for i in 0..(*mem).entries {
        let hpa = *(*mem).pages.hpas.add(i as usize);
        if hpa == 0 { continue; }
        let page = pfn_to_page(hpa >> PAGE_SHIFT);
        if page.is_null() { continue; }
        if hpa & MM_IOMMU_TABLE_GROUP_PAGE_DIRTY != 0 { SetPageDirty(page); }
        unpin_user_page(page); *(*mem).pages.hpas.add(i as usize) = 0;
    }
}

unsafe fn mm_iommu_do_free(mem: *mut mm_iommu_table_group_mem_t) { mm_iommu_unpin(mem); vfree((*mem).pages.hpas as *mut c_void); kfree(mem); }
unsafe fn mm_iommu_free(head: *mut rcu_head) { mm_iommu_do_free(container_of!(head, mm_iommu_table_group_mem_t, rcu)); }
unsafe fn mm_iommu_release(mem: *mut mm_iommu_table_group_mem_t) { list_del_rcu(&mut (*mem).next); call_rcu(&mut (*mem).rcu, mm_iommu_free); }

pub unsafe fn mm_iommu_put(mm: *mut mm_struct, mem: *mut mm_iommu_table_group_mem_t) -> c_long {
    let mut ret = 0; let mut unlock_entries = 0;
    mutex_lock(&mut MEM_LIST_MUTEX);
    if (*mem).used == 0 { ret = -ENOENT; goto_unlock!(); }
    (*mem).used -= 1; if (*mem).used != 0 { goto_unlock!(); }
    if atomic64_cmpxchg(&mut (*mem).mapped, 1, 0) != 1 { (*mem).used += 1; ret = -EBUSY; goto_unlock!(); }
    if (*mem).dev_hpa == MM_IOMMU_TABLE_INVALID_HPA { unlock_entries = (*mem).entries as c_ulong; }
    mm_iommu_release(mem);
    // unlock_exit:
    mutex_unlock(&mut MEM_LIST_MUTEX); account_locked_vm(mm, unlock_entries, false); ret
}

pub unsafe fn mm_iommu_lookup(mm: *mut mm_struct, ua: c_ulong, size: c_ulong) -> *mut mm_iommu_table_group_mem_t {
    let mut ret = core::ptr::null_mut(); rcu_read_lock();
    list_for_each_entry_rcu!(mem, (*mm).context.iommu_group_mem_list, next, {
        if (*mem).ua <= ua as u64 && ua as u64 + size as u64 <= (*mem).ua + ((*mem).entries << PAGE_SHIFT) { ret = mem; break; }
    }); rcu_read_unlock(); ret
}

pub unsafe fn mm_iommu_get(mm: *mut mm_struct, ua: c_ulong, entries: c_ulong) -> *mut mm_iommu_table_group_mem_t {
    let mut ret = core::ptr::null_mut(); mutex_lock(&mut MEM_LIST_MUTEX);
    list_for_each_entry_rcu!(mem, (*mm).context.iommu_group_mem_list, next, lockdep_is_held(&MEM_LIST_MUTEX), {
        if (*mem).ua == ua as u64 && (*mem).entries == entries as u64 { ret = mem; (*mem).used += 1; break; }
    }); mutex_unlock(&mut MEM_LIST_MUTEX); ret
}

pub unsafe fn mm_iommu_ua_to_hpa(mem: *mut mm_iommu_table_group_mem_t, ua: c_ulong, pageshift: c_uint, hpa: *mut c_ulong) -> c_long {
    let entry = ((ua as u64 - (*mem).ua) >> PAGE_SHIFT) as i64;
    if entry >= (*mem).entries as i64 || pageshift > (*mem).pageshift { return -EFAULT; }
    if (*mem).pages.hpas.is_null() { *hpa = ((*mem).dev_hpa + ua as u64 - (*mem).ua) as c_ulong; return 0; }
    *hpa = ((*(*mem).pages.hpas.add(entry as usize) & MM_IOMMU_TABLE_GROUP_PAGE_MASK) | (ua as u64 & !(PAGE_MASK as u64))) as c_ulong; 0
}

pub unsafe fn mm_iommu_is_devmem(mm: *mut mm_struct, hpa: c_ulong, pageshift: c_uint, size: *mut c_ulong) -> bool {
    rcu_read_lock();
    list_for_each_entry_rcu!(mem, (*mm).context.iommu_group_mem_list, next, {
        if (*mem).dev_hpa == MM_IOMMU_TABLE_INVALID_HPA { continue; }
        let end = (*mem).dev_hpa + ((*mem).entries << PAGE_SHIFT);
        if (*mem).dev_hpa <= hpa as u64 && (hpa as u64) < end { *size = min(1UL << pageshift, end - hpa as u64) as c_ulong; return true; }
    }); rcu_read_unlock(); false
}

pub unsafe fn mm_iommu_mapped_inc(mem: *mut mm_iommu_table_group_mem_t) -> c_long { if atomic64_inc_not_zero(&mut (*mem).mapped) { 0 } else { -ENXIO } }
pub unsafe fn mm_iommu_mapped_dec(mem: *mut mm_iommu_table_group_mem_t) { atomic64_add_unless(&mut (*mem).mapped, -1, 1); }
pub unsafe fn mm_iommu_init(mm: *mut mm_struct) { INIT_LIST_HEAD_RCU!(&mut (*mm).context.iommu_group_mem_list); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
