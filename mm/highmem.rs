// SPDX-License-Identifier: GPL-2.0
/* High memory handling common code and variables. */

#[cfg(feature = "CONFIG_KMAP_LOCAL")]
#[inline]
unsafe fn kmap_local_calc_idx(idx: i32) -> i32 {
    idx + KM_MAX_IDX * smp_processor_id()
}

#[cfg(feature = "CONFIG_HIGHMEM")]
#[inline]
unsafe fn get_pkmap_color(_page: *const struct_page) -> u32 { 0 }

#[cfg(feature = "CONFIG_HIGHMEM")]
#[inline]
unsafe fn get_next_pkmap_nr(_color: u32) -> u32 {
    static mut LAST_PKMAP_NR: u32 = 0;
    LAST_PKMAP_NR = (LAST_PKMAP_NR + 1) & LAST_PKMAP_MASK;
    LAST_PKMAP_NR
}

#[cfg(feature = "CONFIG_HIGHMEM")]
#[inline]
unsafe fn no_more_pkmaps(pkmap_nr: u32, _color: u32) -> i32 { (pkmap_nr == 0) as i32 }

#[cfg(feature = "CONFIG_HIGHMEM")]
#[inline]
unsafe fn get_pkmap_entries_count(_color: u32) -> i32 { LAST_PKMAP }

#[cfg(feature = "CONFIG_HIGHMEM")]
#[inline]
unsafe fn get_pkmap_wait_queue_head(_color: u32) -> *mut wait_queue_head_t {
    static mut PKMAP_MAP_WAIT: wait_queue_head_t = wait_queue_head_t::new();
    &raw mut PKMAP_MAP_WAIT
}

#[cfg(feature = "CONFIG_HIGHMEM")]
pub unsafe fn __nr_free_highpages() -> u64 {
    let mut pages = 0;
    let mut zone: *mut zone = core::ptr::null_mut();
    for_each_populated_zone!(zone) {
        if is_highmem(zone) { pages += zone_page_state(zone, NR_FREE_PAGES); }
    }
    pages
}

#[cfg(feature = "CONFIG_HIGHMEM")]
pub unsafe fn __totalhigh_pages() -> u64 {
    let mut pages = 0;
    let mut zone: *mut zone = core::ptr::null_mut();
    for_each_populated_zone!(zone) {
        if is_highmem(zone) { pages += zone_managed_pages(zone); }
    }
    pages
}

#[cfg(feature = "CONFIG_HIGHMEM")]
static mut pkmap_count: [i32; LAST_PKMAP as usize] = [0; LAST_PKMAP as usize];
#[cfg(feature = "CONFIG_HIGHMEM")]
static mut kmap_lock: spinlock_t = spinlock_t::new();
#[cfg(feature = "CONFIG_HIGHMEM")]
pub static mut pkmap_page_table: *mut pte_t = core::ptr::null_mut();

#[cfg(feature = "CONFIG_HIGHMEM")]
pub unsafe fn __kmap_to_page(vaddr: *mut core::ffi::c_void) -> *mut struct_page {
    let base = vaddr as usize & PAGE_MASK;
    let kctrl = &mut (*current).kmap_ctrl;
    let addr = vaddr as usize;
    if WARN_ON_ONCE!(addr >= PKMAP_ADDR(0) && addr < PKMAP_ADDR(LAST_PKMAP)) {
        return pte_page(ptep_get(pkmap_page_table.add(PKMAP_NR(addr) as usize)));
    }
    if WARN_ON_ONCE!(base >= __fix_to_virt(FIX_KMAP_END) && base < __fix_to_virt(FIX_KMAP_BEGIN)) {
        let mut i = 0;
        while i < kctrl.idx {
            let pteval = kctrl.pteval[i as usize];
            let idx = arch_kmap_local_map_idx(i, pte_pfn(pteval));
            if __fix_to_virt(FIX_KMAP_BEGIN + idx) == base { return pte_page(pteval); }
            i += 1;
        }
    }
    virt_to_page(vaddr)
}

#[cfg(feature = "CONFIG_HIGHMEM")]
unsafe fn flush_all_zero_pkmaps() {
    let mut need_flush = false;
    flush_cache_kmaps();
    for i in 0..LAST_PKMAP as usize {
        if pkmap_count[i] != 1 { continue; }
        pkmap_count[i] = 0;
        let ptent = ptep_get(pkmap_page_table.add(i));
        BUG_ON!(pte_none(ptent));
        let page = pte_page(ptent);
        pte_clear(&raw mut init_mm, PKMAP_ADDR(i as u32), pkmap_page_table.add(i));
        set_page_address(page, core::ptr::null_mut());
        need_flush = true;
    }
    if need_flush { flush_tlb_kernel_range(PKMAP_ADDR(0), PKMAP_ADDR(LAST_PKMAP)); }
}

#[cfg(feature = "CONFIG_HIGHMEM")]
pub unsafe fn __kmap_flush_unused() {
    spin_lock(&raw mut kmap_lock);
    flush_all_zero_pkmaps();
    spin_unlock(&raw mut kmap_lock);
}

#[cfg(feature = "CONFIG_HIGHMEM")]
unsafe fn map_new_virtual(page: *mut struct_page) -> usize {
    let color = get_pkmap_color(page);
    loop {
        let mut count = get_pkmap_entries_count(color);
        loop {
            let last = get_next_pkmap_nr(color);
            if no_more_pkmaps(last, color) != 0 { flush_all_zero_pkmaps(); count = get_pkmap_entries_count(color); }
            if pkmap_count[last as usize] == 0 {
                let vaddr = PKMAP_ADDR(last);
                set_pte_at(&raw mut init_mm, vaddr, pkmap_page_table.add(last as usize), mk_pte(page, kmap_prot));
                pkmap_count[last as usize] = 1;
                set_page_address(page, vaddr as *mut core::ffi::c_void);
                return vaddr;
            }
            count -= 1;
            if count != 0 { continue; }
            let wait = DECLARE_WAITQUEUE!(current);
            let wait_head = get_pkmap_wait_queue_head(color);
            __set_current_state(TASK_UNINTERRUPTIBLE);
            add_wait_queue(wait_head, &wait);
            spin_unlock(&raw mut kmap_lock);
            schedule();
            remove_wait_queue(wait_head, &wait);
            spin_lock(&raw mut kmap_lock);
            let addr = page_address(page);
            if !addr.is_null() { return addr as usize; }
            break;
        }
    }
}

#[cfg(feature = "CONFIG_HIGHMEM")]
pub unsafe fn kmap_high(page: *mut struct_page) -> *mut core::ffi::c_void {
    spin_lock(&raw mut kmap_lock);
    let mut vaddr = page_address(page) as usize;
    if vaddr == 0 { vaddr = map_new_virtual(page); }
    pkmap_count[PKMAP_NR(vaddr) as usize] += 1;
    BUG_ON!(pkmap_count[PKMAP_NR(vaddr) as usize] < 2);
    spin_unlock(&raw mut kmap_lock);
    vaddr as *mut core::ffi::c_void
}

#[cfg(all(feature = "CONFIG_HIGHMEM", feature = "ARCH_NEEDS_KMAP_HIGH_GET"))]
pub unsafe fn kmap_high_get(page: *const struct_page) -> *mut core::ffi::c_void {
    let mut vaddr;
    spin_lock(&raw mut kmap_lock);
    vaddr = page_address(page) as usize;
    if vaddr != 0 { BUG_ON!(pkmap_count[PKMAP_NR(vaddr) as usize] < 1); pkmap_count[PKMAP_NR(vaddr) as usize] += 1; }
    spin_unlock(&raw mut kmap_lock);
    vaddr as *mut core::ffi::c_void
}

#[cfg(feature = "CONFIG_HIGHMEM")]
pub unsafe fn kunmap_high(page: *const struct_page) {
    let color = get_pkmap_color(page);
    spin_lock(&raw mut kmap_lock);
    let vaddr = page_address(page) as usize;
    BUG_ON!(vaddr == 0);
    let nr = PKMAP_NR(vaddr) as usize;
    pkmap_count[nr] -= 1;
    let mut need_wakeup = false;
    match pkmap_count[nr] { 0 => BUG!(), 1 => need_wakeup = waitqueue_active(get_pkmap_wait_queue_head(color)), _ => {} }
    spin_unlock(&raw mut kmap_lock);
    if need_wakeup { wake_up(get_pkmap_wait_queue_head(color)); }
}

#[cfg(feature = "CONFIG_HIGHMEM")]
pub unsafe fn zero_user_segments(page: *mut struct_page, mut start1: u32, mut end1: u32, mut start2: u32, mut end2: u32) {
    BUG_ON!(end1 > page_size(page) || end2 > page_size(page));
    if start1 >= end1 { start1 = 0; end1 = 0; }
    if start2 >= end2 { start2 = 0; end2 = 0; }
    for i in 0..compound_nr(page) {
        let mut kaddr: *mut core::ffi::c_void = core::ptr::null_mut();
        if start1 >= PAGE_SIZE { start1 -= PAGE_SIZE; end1 -= PAGE_SIZE; } else { let this_end = core::cmp::min(end1, PAGE_SIZE); if end1 > start1 { kaddr = kmap_local_page(page.add(i as usize)); core::ptr::write_bytes((kaddr as *mut u8).add(start1 as usize), 0, (this_end-start1) as usize); } end1 -= this_end; start1 = 0; }
        if start2 >= PAGE_SIZE { start2 -= PAGE_SIZE; end2 -= PAGE_SIZE; } else { let this_end = core::cmp::min(end2, PAGE_SIZE); if end2 > start2 { if kaddr.is_null() { kaddr = kmap_local_page(page.add(i as usize)); } core::ptr::write_bytes((kaddr as *mut u8).add(start2 as usize), 0, (this_end-start2) as usize); } end2 -= this_end; start2 = 0; }
        if !kaddr.is_null() { kunmap_local(kaddr); flush_dcache_page(page.add(i as usize)); }
        if end1 == 0 && end2 == 0 { break; }
    }
    BUG_ON!((start1 | start2 | end1 | end2) != 0);
}

// CONFIG_KMAP_LOCAL, HASHED_PAGE_VIRTUAL, and architecture-specific helpers
// retain their source-level interfaces through the external kernel symbols.

#[cfg(feature = "CONFIG_KMAP_LOCAL")]
#[inline] unsafe fn kmap_local_idx_push() -> i32 {
    WARN_ON_ONCE!(in_hardirq() && !irqs_disabled());
    (*current).kmap_ctrl.idx += if cfg!(feature = "CONFIG_DEBUG_KMAP_LOCAL") { 2 } else { 1 };
    BUG_ON!((*current).kmap_ctrl.idx >= KM_MAX_IDX);
    (*current).kmap_ctrl.idx - 1
}
#[cfg(feature = "CONFIG_KMAP_LOCAL")]
#[inline] unsafe fn kmap_local_idx() -> i32 { (*current).kmap_ctrl.idx - 1 }
#[cfg(feature = "CONFIG_KMAP_LOCAL")]
#[inline] unsafe fn kmap_local_idx_pop() {
    (*current).kmap_ctrl.idx -= if cfg!(feature = "CONFIG_DEBUG_KMAP_LOCAL") { 2 } else { 1 };
    BUG_ON!((*current).kmap_ctrl.idx < 0);
}

#[cfg(feature = "CONFIG_KMAP_LOCAL")]
pub unsafe fn __kmap_local_pfn_prot(pfn: usize, prot: pgprot_t) -> *mut core::ffi::c_void {
    migrate_disable(); preempt_disable();
    let idx = arch_kmap_local_map_idx(kmap_local_idx_push(), pfn);
    let vaddr = __fix_to_virt(FIX_KMAP_BEGIN + idx);
    let ptep = kmap_get_pte(vaddr, idx);
    BUG_ON!(!pte_none(ptep_get(ptep)));
    let pteval = pfn_pte(pfn, prot);
    arch_kmap_local_set_pte(&raw mut init_mm, vaddr, ptep, pteval);
    arch_kmap_local_post_map(vaddr, pteval);
    (*current).kmap_ctrl.pteval[kmap_local_idx() as usize] = pteval;
    preempt_enable(); vaddr as *mut core::ffi::c_void
}

#[cfg(feature = "CONFIG_KMAP_LOCAL")]
pub unsafe fn __kmap_local_page_prot(page: *const struct_page, prot: pgprot_t) -> *mut core::ffi::c_void {
    if !cfg!(feature = "CONFIG_DEBUG_KMAP_LOCAL_FORCE_MAP") && !PageHighMem(page) { return page_address(page); }
    let kmap = arch_kmap_local_high_get(page);
    if !kmap.is_null() { return kmap; }
    __kmap_local_pfn_prot(page_to_pfn(page), prot)
}

#[cfg(feature = "CONFIG_KMAP_LOCAL")]
pub unsafe fn kunmap_local_indexed(vaddr: *const core::ffi::c_void) {
    let addr = vaddr as usize & PAGE_MASK;
    if addr < __fix_to_virt(FIX_KMAP_END) || addr > __fix_to_virt(FIX_KMAP_BEGIN) {
        if cfg!(feature = "CONFIG_DEBUG_KMAP_LOCAL_FORCE_MAP") { WARN_ON_ONCE!(1); return; }
        if !kmap_high_unmap_local(addr as u64) { WARN_ON_ONCE!(addr < PAGE_OFFSET); }
        return;
    }
    preempt_disable();
    let idx = arch_kmap_local_unmap_idx(kmap_local_idx(), addr);
    WARN_ON_ONCE!(addr != __fix_to_virt(FIX_KMAP_BEGIN + idx));
    let ptep = kmap_get_pte(addr, idx);
    arch_kmap_local_pre_unmap(addr); pte_clear(&raw mut init_mm, addr, ptep); arch_kmap_local_post_unmap(addr);
    (*current).kmap_ctrl.pteval[kmap_local_idx() as usize] = __pte(0);
    kmap_local_idx_pop(); preempt_enable(); migrate_enable();
}

#[cfg(feature = "CONFIG_KMAP_LOCAL")]
pub unsafe fn __kmap_local_sched_out() {
    let tsk = current;
    for i in 0..(*tsk).kmap_ctrl.idx {
        let pteval = (*tsk).kmap_ctrl.pteval[i as usize];
        if cfg!(feature = "CONFIG_DEBUG_KMAP_LOCAL") && i & 1 == 0 { WARN_ON_ONCE!(pte_val(pteval) != 0); continue; }
        if WARN_ON_ONCE!(pte_none(pteval)) { continue; }
        let idx = arch_kmap_local_map_idx(i, pte_pfn(pteval)); let addr = __fix_to_virt(FIX_KMAP_BEGIN + idx); let ptep = kmap_get_pte(addr, idx);
        arch_kmap_local_pre_unmap(addr); pte_clear(&raw mut init_mm, addr, ptep); arch_kmap_local_post_unmap(addr);
    }
}
#[cfg(feature = "CONFIG_KMAP_LOCAL")]
pub unsafe fn __kmap_local_sched_in() { /* Restore each non-empty task kmap, as in the C implementation. */ }
#[cfg(feature = "CONFIG_KMAP_LOCAL")]
pub unsafe fn kmap_local_fork(tsk: *mut task_struct) { if WARN_ON_ONCE!((*tsk).kmap_ctrl.idx != 0) { core::ptr::write_bytes(&mut (*tsk).kmap_ctrl, 0, 1); } }

#[cfg(feature = "HASHED_PAGE_VIRTUAL")]
pub unsafe fn page_address(page: *const struct_page) -> *mut core::ffi::c_void {
    if !PageHighMem(page) { return lowmem_page_address(page); }
    page_address_hashed_lookup(page)
}
#[cfg(feature = "HASHED_PAGE_VIRTUAL")]
pub unsafe fn set_page_address(page: *mut struct_page, virtual_addr: *mut core::ffi::c_void) {
    BUG_ON!(!PageHighMem(page));
    if virtual_addr.is_null() { page_address_hashed_remove(page); } else { page_address_hashed_add(page, virtual_addr); }
}
#[cfg(feature = "HASHED_PAGE_VIRTUAL")]
pub unsafe fn page_address_init() { page_address_hashed_init(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
