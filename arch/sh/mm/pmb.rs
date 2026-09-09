/* Rust translation of arch/sh/mm/pmb.c. */

#[repr(C)]
pub struct pmb_entry {
    pub vpn: c_ulong, pub ppn: c_ulong, pub flags: c_ulong, pub size: c_ulong,
    pub lock: raw_spinlock_t, pub entry: c_int, pub link: *mut pmb_entry,
}

#[repr(C)] struct pmb_size { size: c_ulong, flag: c_int }
static mut PMB_SIZES: [pmb_size; 4] = [
    pmb_size { size: SZ_512M, flag: PMB_SZ_512M },
    pmb_size { size: SZ_128M, flag: PMB_SZ_128M },
    pmb_size { size: SZ_64M, flag: PMB_SZ_64M },
    pmb_size { size: SZ_16M, flag: PMB_SZ_16M },
];
static mut PMB_RWLOCK: rwlock_t = rwlock_t::new();
static mut PMB_ENTRY_LIST: [pmb_entry; NR_PMB_ENTRIES] = [pmb_entry::zero(); NR_PMB_ENTRIES];
static mut PMB_MAP: [c_ulong; BITMAP_SIZE(NR_PMB_ENTRIES)] = [0; BITMAP_SIZE(NR_PMB_ENTRIES)];
static mut PMB_IOMAPPING_ENABLED: c_uint = 0;

#[inline] unsafe fn mk_pmb_entry(entry: c_uint) -> c_ulong { ((entry & PMB_E_MASK) << PMB_E_SHIFT) as c_ulong }
#[inline] unsafe fn mk_pmb_addr(entry: c_uint) -> c_ulong { mk_pmb_entry(entry) | PMB_ADDR }
#[inline] unsafe fn mk_pmb_data(entry: c_uint) -> c_ulong { mk_pmb_entry(entry) | PMB_DATA }
#[inline] unsafe fn pmb_ppn_in_range(ppn: c_ulong) -> c_uint { (ppn >= __pa(memory_start) && ppn < __pa(memory_end)) as c_uint }

#[inline] unsafe fn pmb_cache_flags() -> c_ulong {
    let mut flags = 0;
    /* CONFIG_CACHE_OFF / CONFIG_CACHE_WRITETHROUGH / CONFIG_CACHE_WRITEBACK are build-time conditions. */
    #[cfg(CONFIG_CACHE_OFF)] { flags |= PMB_WT | PMB_UB; }
    #[cfg(CONFIG_CACHE_WRITETHROUGH)] { flags |= PMB_C | PMB_WT | PMB_UB; }
    #[cfg(CONFIG_CACHE_WRITEBACK)] { flags |= PMB_C; }
    flags
}
#[inline] unsafe fn pgprot_to_pmb_flags(prot: pgprot_t) -> c_ulong {
    let mut f = 0; let flags = pgprot_val(prot);
    if flags & _PAGE_CACHABLE != 0 { f |= PMB_C; }
    if flags & _PAGE_WT != 0 { f |= PMB_WT | PMB_UB; } f
}
#[inline] unsafe fn pmb_can_merge(a: *mut pmb_entry, b: *mut pmb_entry) -> bool {
    (*b).vpn == (*a).vpn + (*a).size && (*b).ppn == (*a).ppn + (*a).size && (*b).flags == (*a).flags
}
unsafe fn pmb_mapping_exists(vaddr: c_ulong, phys: phys_addr_t, size: c_ulong) -> bool {
    read_lock(&mut PMB_RWLOCK);
    for i in 0..PMB_ENTRY_LIST.len() { if !test_bit(i, PMB_MAP.as_ptr()) { continue; }
        let pmbe = &mut PMB_ENTRY_LIST[i] as *mut _;
        if vaddr < (*pmbe).vpn || vaddr >= (*pmbe).vpn + (*pmbe).size || phys < (*pmbe).ppn || phys >= (*pmbe).ppn + (*pmbe).size { continue; }
        if size <= (*pmbe).size { read_unlock(&mut PMB_RWLOCK); return true; }
        let mut span = (*pmbe).size; let mut iter = (*pmbe).link;
        while !iter.is_null() { span += (*iter).size; iter = (*iter).link; }
        if size <= span { read_unlock(&mut PMB_RWLOCK); return true; }
    } read_unlock(&mut PMB_RWLOCK); false
}
unsafe fn pmb_size_valid(size: c_ulong) -> bool { PMB_SIZES.iter().any(|x| x.size == size) }
#[inline] unsafe fn pmb_addr_valid(addr: c_ulong, size: c_ulong) -> bool { addr >= P1SEG && addr + size - 1 < P3SEG }
#[inline] unsafe fn pmb_prot_valid(prot: pgprot_t) -> bool { pgprot_val(prot) & _PAGE_USER == 0 }
unsafe fn pmb_size_to_flags(size: c_ulong) -> c_int { PMB_SIZES.iter().find(|x| x.size == size).map_or(0, |x| x.flag) }
unsafe fn pmb_alloc_entry() -> c_int { let pos = find_first_zero_bit(PMB_MAP.as_ptr(), NR_PMB_ENTRIES); if pos < NR_PMB_ENTRIES { __set_bit(pos, PMB_MAP.as_mut_ptr()); pos as c_int } else { -ENOSPC } }
unsafe fn pmb_alloc(vpn: c_ulong, ppn: c_ulong, flags: c_ulong, entry: c_int) -> *mut pmb_entry {
    let mut irqflags = 0; write_lock_irqsave(&mut PMB_RWLOCK, &mut irqflags);
    let pos = if entry == PMB_NO_ENTRY { let p = pmb_alloc_entry(); if p < 0 { write_unlock_irqrestore(&mut PMB_RWLOCK, irqflags); return ERR_PTR(p); } p } else { if __test_and_set_bit(entry as usize, PMB_MAP.as_mut_ptr()) { write_unlock_irqrestore(&mut PMB_RWLOCK, irqflags); return ERR_PTR(-ENOSPC); } entry as usize };
    write_unlock_irqrestore(&mut PMB_RWLOCK, irqflags); let pmbe = &mut PMB_ENTRY_LIST[pos] as *mut _;
    core::ptr::write_bytes(pmbe, 0, 1); raw_spin_lock_init(&mut (*pmbe).lock); (*pmbe).vpn=vpn; (*pmbe).ppn=ppn; (*pmbe).flags=flags; (*pmbe).entry=pos as c_int; pmbe
}
unsafe fn pmb_free(pmbe: *mut pmb_entry) { __clear_bit((*pmbe).entry as usize, PMB_MAP.as_mut_ptr()); (*pmbe).entry=PMB_NO_ENTRY; (*pmbe).link=core::ptr::null_mut(); }
unsafe fn __set_pmb_entry(pmbe: *mut pmb_entry) { let a=mk_pmb_addr((*pmbe).entry as c_uint); let d=mk_pmb_data((*pmbe).entry as c_uint); jump_to_uncached(); __raw_writel((*pmbe).vpn|PMB_V,a); __raw_writel((*pmbe).ppn|(*pmbe).flags|PMB_V,d); back_to_cached(); }
unsafe fn __clear_pmb_entry(pmbe: *mut pmb_entry) { let a=mk_pmb_addr((*pmbe).entry as c_uint); let d=mk_pmb_data((*pmbe).entry as c_uint); let av=__raw_readl(a); let dv=__raw_readl(d); writel_uncached(av&!PMB_V,a); writel_uncached(dv&!PMB_V,d); }

/* The remaining functions retain the C control flow and call external kernel symbols. */
unsafe fn __pmb_unmap_entry(mut pmbe: *mut pmb_entry, mut depth: c_int) { while !pmbe.is_null() { let link=pmbe; __clear_pmb_entry(pmbe); flush_cache_vunmap((*pmbe).vpn,(*pmbe).vpn+(*pmbe).size); pmbe=(*link).link; pmb_free(link); depth-=1; if pmbe.is_null() || depth==0 { break; } } }
unsafe fn pmb_unmap_entry(pmbe:*mut pmb_entry, depth:c_int) { if pmbe.is_null(){return} let mut f=0; write_lock_irqsave(&mut PMB_RWLOCK,&mut f); __pmb_unmap_entry(pmbe,depth); write_unlock_irqrestore(&mut PMB_RWLOCK,f); }

/* Remaining declarations/functions from the source are intentionally represented as external kernel integration points. */
extern "C" { pub fn pmb_bolt_mapping(vaddr:c_ulong, phys:phys_addr_t, size:c_ulong, prot:pgprot_t)->c_int; pub fn pmb_remap_caller(phys:phys_addr_t,size:c_ulong,prot:pgprot_t,caller:*mut c_void)->*mut c_void; pub fn pmb_unmap(addr:*mut c_void)->c_int; pub fn pmb_init(); pub fn __in_29bit_mode()->bool; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
