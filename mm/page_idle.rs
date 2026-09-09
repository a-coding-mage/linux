// SPDX-License-Identifier: GPL-2.0
// Kernel headers and build-time configuration are supplied by the surrounding
// translation unit.

const BITMAP_CHUNK_SIZE: usize = core::mem::size_of::<u64>();
const BITMAP_CHUNK_BITS: usize = BITMAP_CHUNK_SIZE * 8;

unsafe fn page_idle_get_folio(pfn: usize) -> *mut folio {
    let page = pfn_to_online_page(pfn);
    if page.is_null() || PageTail(page) { return core::ptr::null_mut(); }
    let mut folio = page_folio(page);
    if !folio_test_lru(folio) || !folio_try_get(folio) { return core::ptr::null_mut(); }
    if page_folio(page) != folio || !folio_test_lru(folio) {
        folio_put(folio);
        folio = core::ptr::null_mut();
    }
    folio
}

unsafe fn page_idle_clear_pte_refs_one(
    folio: *mut folio, vma: *mut vm_area_struct, mut addr: usize, _arg: *mut core::ffi::c_void,
) -> bool {
    // DEFINE_FOLIO_VMA_WALK(pvmw, folio, vma, addr, 0)
    let mut referenced = false;
    let mut pvmw = FolioVmaWalk::new(folio, vma, addr, 0);
    while page_vma_mapped_walk(&mut pvmw) {
        addr = pvmw.address;
        if !pvmw.pte.is_null() {
            if pte_present(ptep_get(pvmw.pte)) {
                referenced |= ptep_test_and_clear_young(vma, addr, pvmw.pte);
            }
            referenced |= mmu_notifier_clear_young((*vma).vm_mm, addr, addr + PAGE_SIZE);
        } else if IS_ENABLED_CONFIG_TRANSPARENT_HUGEPAGE {
            let pmdval = pmdp_get(pvmw.pmd);
            if pmd_present(pmdval) { referenced |= pmdp_test_and_clear_young(vma, addr, pvmw.pmd); }
            referenced |= mmu_notifier_clear_young((*vma).vm_mm, addr, addr + PMD_SIZE);
        } else {
            WARN_ON_ONCE(true);
        }
    }
    if referenced {
        folio_clear_idle(folio);
        folio_set_young(folio);
    }
    true
}

unsafe fn page_idle_clear_pte_refs(folio: *mut folio) {
    if !folio_mapped(folio) || !folio_raw_mapping(folio) { return; }
    if !folio_trylock(folio) { return; }
    let mut rwc = RmapWalkControl {
        rmap_one: Some(page_idle_clear_pte_refs_one),
        anon_lock: Some(folio_lock_anon_vma_read),
    };
    rmap_walk(folio, &mut rwc);
    folio_unlock(folio);
}

unsafe fn page_idle_bitmap_read(
    _file: *mut file, _kobj: *mut kobject, _attr: *const bin_attribute,
    buf: *mut u8, pos: i64, count: usize,
) -> isize {
    let mut out = buf as *mut u64;
    if pos as usize % BITMAP_CHUNK_SIZE != 0 || count % BITMAP_CHUNK_SIZE != 0 { return -EINVAL; }
    let pfn = (pos as usize) * 8;
    if pfn >= max_pfn { return 0; }
    let end_pfn = core::cmp::min(pfn + count * 8, max_pfn);
    for pfn in pfn..end_pfn {
        let bit = pfn % BITMAP_CHUNK_BITS;
        if bit == 0 { *out = 0; }
        let folio = page_idle_get_folio(pfn);
        if !folio.is_null() {
            if folio_test_idle(folio) {
                page_idle_clear_pte_refs(folio);
                if folio_test_idle(folio) { *out |= 1u64 << bit; }
            }
            folio_put(folio);
        }
        if bit == BITMAP_CHUNK_BITS - 1 { out = out.add(1); }
        cond_resched();
    }
    (out as *mut u8).offset_from(buf) as isize
}

unsafe fn page_idle_bitmap_write(
    _file: *mut file, _kobj: *mut kobject, _attr: *const bin_attribute,
    buf: *mut u8, pos: i64, count: usize,
) -> isize {
    let mut input = buf as *const u64;
    if pos as usize % BITMAP_CHUNK_SIZE != 0 || count % BITMAP_CHUNK_SIZE != 0 { return -EINVAL; }
    let pfn = (pos as usize) * 8;
    if pfn >= max_pfn { return -ENXIO; }
    let end_pfn = core::cmp::min(pfn + count * 8, max_pfn);
    for pfn in pfn..end_pfn {
        let bit = pfn % BITMAP_CHUNK_BITS;
        if ((*input >> bit) & 1) != 0 {
            let folio = page_idle_get_folio(pfn);
            if !folio.is_null() {
                page_idle_clear_pte_refs(folio);
                folio_set_idle(folio);
                folio_put(folio);
            }
        }
        if bit == BITMAP_CHUNK_BITS - 1 { input = input.add(1); }
        cond_resched();
    }
    (input as *const u8).offset_from(buf) as isize
}

// __BIN_ATTR(bitmap, 0600, page_idle_bitmap_read, page_idle_bitmap_write, 0)
static PAGE_IDLE_BITMAP_ATTR: bin_attribute = BinAttribute::new(
    "bitmap", 0o600, page_idle_bitmap_read, page_idle_bitmap_write, 0,
);
static PAGE_IDLE_BIN_ATTRS: [*const bin_attribute; 2] = [&PAGE_IDLE_BITMAP_ATTR, core::ptr::null()];
static PAGE_IDLE_ATTR_GROUP: attribute_group = AttributeGroup::new(PAGE_IDLE_BIN_ATTRS.as_ptr(), "page_idle");

unsafe fn page_idle_init() -> i32 {
    let err = sysfs_create_group(mm_kobj, &PAGE_IDLE_ATTR_GROUP);
    if err != 0 { pr_err!("page_idle: register sysfs failed\n"); return err; }
    0
}

// subsys_initcall(page_idle_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
