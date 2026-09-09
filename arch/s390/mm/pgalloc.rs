// SPDX-License-Identifier: GPL-2.0
/* Page table allocation functions. */

// External kernel declarations supplied by the surrounding translation unit.

pub unsafe fn crst_table_alloc_noprof(mm: *mut mm_struct) -> *mut c_ulong {
    let mut gfp: gfp_t = GFP_KERNEL_ACCOUNT;
    if mm == &raw mut init_mm {
        gfp &= !__GFP_ACCOUNT;
    }
    let ptdesc = pagetable_alloc_noprof(gfp, CRST_ALLOC_ORDER);
    if ptdesc.is_null() { return core::ptr::null_mut(); }
    let table = ptdesc_address(ptdesc);
    __arch_set_page_dat(table, 1UL << CRST_ALLOC_ORDER);
    table
}

pub unsafe fn crst_table_free(_mm: *mut mm_struct, table: *mut c_ulong) {
    if table.is_null() { return; }
    pagetable_free(virt_to_ptdesc(table));
}

unsafe fn __crst_table_upgrade(arg: *mut core::ffi::c_void) {
    let mm = arg as *mut mm_struct;
    let mut asce: ctlreg;
    if (*current).active_mm == mm {
        asce.val = (*mm).context.asce;
        (*get_lowcore()).user_asce = asce;
        local_ctl_load(7, &raw const asce);
        if !test_thread_flag(TIF_ASCE_PRIMARY) { local_ctl_load(1, &raw const asce); }
    }
    __tlb_flush_local();
}

pub unsafe fn crst_table_upgrade(mm: *mut mm_struct, end: c_ulong) -> c_int {
    let mut pgd = core::ptr::null_mut();
    let mut p4d = core::ptr::null_mut();
    let mut __pgd;
    let asce_limit = (*mm).context.asce_limit;
    mmap_assert_write_locked(mm);
    VM_BUG_ON(asce_limit < _REGION2_SIZE);
    if end <= asce_limit { return 0; }
    if asce_limit == _REGION2_SIZE {
        p4d = crst_table_alloc(mm);
        if p4d.is_null() { return -ENOMEM; }
        crst_table_init(p4d, _REGION2_ENTRY_EMPTY);
        pagetable_p4d_ctor(virt_to_ptdesc(p4d));
    }
    if end > _REGION1_SIZE {
        pgd = crst_table_alloc(mm);
        if pgd.is_null() {
            pagetable_dtor(virt_to_ptdesc(p4d));
            crst_table_free(mm, p4d);
            return -ENOMEM;
        }
        crst_table_init(pgd, _REGION1_ENTRY_EMPTY);
        pagetable_pgd_ctor(virt_to_ptdesc(pgd));
    }
    spin_lock_bh(&raw mut (*mm).page_table_lock);
    if !p4d.is_null() {
        __pgd = (*mm).pgd as *mut c_ulong;
        p4d_populate(mm, p4d as *mut p4d_t, __pgd as *mut pud_t);
        (*mm).pgd = p4d as *mut pgd_t;
        (*mm).context.asce_limit = _REGION1_SIZE;
        (*mm).context.asce = __pa((*mm).pgd) | _ASCE_TABLE_LENGTH | _ASCE_USER_BITS | _ASCE_TYPE_REGION2;
        mm_inc_nr_puds(mm);
    }
    if !pgd.is_null() {
        __pgd = (*mm).pgd as *mut c_ulong;
        pgd_populate(mm, pgd as *mut pgd_t, __pgd as *mut p4d_t);
        (*mm).pgd = pgd as *mut pgd_t;
        (*mm).context.asce_limit = TASK_SIZE_MAX;
        (*mm).context.asce = __pa((*mm).pgd) | _ASCE_TABLE_LENGTH | _ASCE_USER_BITS | _ASCE_TYPE_REGION1;
    }
    spin_unlock_bh(&raw mut (*mm).page_table_lock);
    on_each_cpu(Some(__crst_table_upgrade), mm as *mut _, 0);
    0
}

pub unsafe fn page_table_alloc_noprof(mm: *mut mm_struct) -> *mut c_ulong {
    let mut gfp: gfp_t = GFP_KERNEL_ACCOUNT;
    if mm == &raw mut init_mm { gfp &= !__GFP_ACCOUNT; }
    let ptdesc = pagetable_alloc_noprof(gfp, 0);
    if ptdesc.is_null() { return core::ptr::null_mut(); }
    if !pagetable_pte_ctor(mm, ptdesc) { pagetable_free(ptdesc); return core::ptr::null_mut(); }
    let table = ptdesc_address(ptdesc);
    __arch_set_page_dat(table, 1);
    memset64(table as *mut u64, _PAGE_INVALID, PTRS_PER_PTE);
    memset64(table.add(PTRS_PER_PTE as usize) as *mut u64, 0, PTRS_PER_PTE);
    table
}

pub unsafe fn page_table_free(_mm: *mut mm_struct, table: *mut c_ulong) {
    let ptdesc = virt_to_ptdesc(table);
    if pagetable_is_reserved(ptdesc) { free_reserved_ptdesc(ptdesc); } else { pagetable_dtor_free(ptdesc); }
}

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
unsafe fn pte_free_now(head: *mut rcu_head) {
    let ptdesc = container_of(head, ptdesc, pt_rcu_head);
    pagetable_dtor_free(ptdesc);
}

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe fn pte_free_defer(_mm: *mut mm_struct, pgtable: pgtable_t) {
    let ptdesc = virt_to_ptdesc(pgtable);
    call_rcu(&raw mut (*ptdesc).pt_rcu_head, Some(pte_free_now));
}

static mut base_pgt_cache: *mut kmem_cache = core::ptr::null_mut();

unsafe fn base_pgt_alloc() -> *mut c_ulong {
    let table = kmem_cache_alloc(base_pgt_cache, GFP_KERNEL) as *mut c_ulong;
    if !table.is_null() { memset64(table as *mut u64, _PAGE_INVALID, PTRS_PER_PTE); }
    table
}
unsafe fn base_pgt_free(table: *mut c_ulong) { kmem_cache_free(base_pgt_cache, table as *mut _); }
unsafe fn base_crst_alloc(val: c_ulong) -> *mut c_ulong {
    let ptdesc = pagetable_alloc(GFP_KERNEL, CRST_ALLOC_ORDER);
    if ptdesc.is_null() { return core::ptr::null_mut(); }
    let table = ptdesc_address(ptdesc); crst_table_init(table, val); table
}
unsafe fn base_crst_free(table: *mut c_ulong) { if !table.is_null() { pagetable_free(virt_to_ptdesc(table)); } }

macro_rules! base_addr_end { ($name:ident, $size:expr) => {
    unsafe fn $name(addr: c_ulong, end: c_ulong) -> c_ulong {
        let next = (addr.wrapping_add($size)) & !($size.wrapping_sub(1));
        if next.wrapping_sub(1) < end.wrapping_sub(1) { next } else { end }
    }
}; }
base_addr_end!(base_page_addr_end, PAGE_SIZE);
base_addr_end!(base_segment_addr_end, _SEGMENT_SIZE);
base_addr_end!(base_region3_addr_end, _REGION3_SIZE);
base_addr_end!(base_region2_addr_end, _REGION2_SIZE);
base_addr_end!(base_region1_addr_end, _REGION1_SIZE);

unsafe fn base_lra(address: c_ulong) -> c_ulong { let real: c_ulong; core::arch::asm!("lra {0},0({1})", out(reg) real, in(reg) address, options(nostack)); real }

unsafe fn base_page_walk(origin: *mut c_ulong, mut addr: c_ulong, end: c_ulong, alloc: c_int) -> c_int {
    if alloc == 0 { return 0; }
    let mut pte = origin.add(((addr & _PAGE_INDEX) >> PAGE_SHIFT) as usize);
    loop { let next = base_page_addr_end(addr, end); *pte = base_lra(addr); pte = pte.add(1); addr = next; if addr >= end { break; } }
    0
}

unsafe fn base_segment_walk(origin: *mut c_ulong, mut addr: c_ulong, end: c_ulong, alloc: c_int) -> c_int {
    let mut ste = origin.add(((addr & _SEGMENT_INDEX) >> _SEGMENT_SHIFT) as usize);
    loop {
        let next = base_segment_addr_end(addr, end);
        if *ste & _SEGMENT_ENTRY_INVALID != 0 { if alloc == 0 { addr = next; ste = ste.add(1); if addr >= end { break; } continue; } let table = base_pgt_alloc(); if table.is_null() { return -ENOMEM; } *ste = __pa(table) | _SEGMENT_ENTRY; }
        let table = __va(*ste & _SEGMENT_ENTRY_ORIGIN); let rc = base_page_walk(table, addr, next, alloc); if rc != 0 { return rc; } if alloc == 0 { base_pgt_free(table); }
        addr = next; ste = ste.add(1); if addr >= end { break; }
    } 0
}

macro_rules! base_region_walk { ($fn:ident, $nextfn:ident, $index:ident, $shift:ident, $empty:ident, $entry:ident) => {
    unsafe fn $fn(origin: *mut c_ulong, mut addr: c_ulong, end: c_ulong, alloc: c_int) -> c_int {
        let mut entry = origin.add(((addr & $index) >> $shift) as usize);
        loop { let next = $nextfn(addr, end); if *entry & _REGION_ENTRY_INVALID != 0 { if alloc == 0 { addr=next; entry=entry.add(1); if addr>=end {break;} continue; } let table=base_crst_alloc($empty); if table.is_null(){return -ENOMEM;} *entry=__pa(table)|$entry; } let table=__va(*entry&_REGION_ENTRY_ORIGIN); let rc=$fn##_inner(table,addr,next,alloc); if rc!=0{return rc;} if alloc==0{base_crst_free(table);} addr=next; entry=entry.add(1); if addr>=end{break;} } 0
    }
}; }

// The following walkers are spelled out to preserve the C call graph.
unsafe fn base_region3_walk(o:*mut c_ulong,mut a:c_ulong,e:c_ulong,x:c_int)->c_int{let mut q=o.add(((a&_REGION3_INDEX)>>_REGION3_SHIFT)as usize);loop{let n=base_region3_addr_end(a,e);if *q&_REGION_ENTRY_INVALID!=0{if x==0{a=n;q=q.add(1);if a>=e{break;}continue}let t=base_crst_alloc(_SEGMENT_ENTRY_EMPTY);if t.is_null(){return -ENOMEM}*q=__pa(t)|_REGION3_ENTRY}let t=__va(*q&_REGION_ENTRY_ORIGIN);let r=base_segment_walk(t,a,n,x);if r!=0{return r}if x==0{base_crst_free(t)}a=n;q=q.add(1);if a>=e{break}}0}
unsafe fn base_region2_walk(o:*mut c_ulong,mut a:c_ulong,e:c_ulong,x:c_int)->c_int{let mut q=o.add(((a&_REGION2_INDEX)>>_REGION2_SHIFT)as usize);loop{let n=base_region2_addr_end(a,e);if *q&_REGION_ENTRY_INVALID!=0{if x==0{a=n;q=q.add(1);if a>=e{break;}continue}let t=base_crst_alloc(_REGION3_ENTRY_EMPTY);if t.is_null(){return -ENOMEM}*q=__pa(t)|_REGION2_ENTRY}let t=__va(*q&_REGION_ENTRY_ORIGIN);let r=base_region3_walk(t,a,n,x);if r!=0{return r}if x==0{base_crst_free(t)}a=n;q=q.add(1);if a>=e{break}}0}
unsafe fn base_region1_walk(o:*mut c_ulong,mut a:c_ulong,e:c_ulong,x:c_int)->c_int{let mut q=o.add(((a&_REGION1_INDEX)>>_REGION1_SHIFT)as usize);loop{let n=base_region1_addr_end(a,e);if *q&_REGION_ENTRY_INVALID!=0{if x==0{a=n;q=q.add(1);if a>=e{break;}continue}let t=base_crst_alloc(_REGION2_ENTRY_EMPTY);if t.is_null(){return -ENOMEM}*q=__pa(t)|_REGION1_ENTRY}let t=__va(*q&_REGION_ENTRY_ORIGIN);let r=base_region2_walk(t,a,n,x);if r!=0{return r}if x==0{base_crst_free(t)}a=n;q=q.add(1);if a>=e{break}}0}

pub unsafe fn base_asce_free(asce:c_ulong){if asce==0{return}let t=__va(asce&_ASCE_ORIGIN);match asce&_ASCE_TYPE_MASK{_ASCE_TYPE_SEGMENT=>{base_segment_walk(t,0,_REGION3_SIZE,0);},_ASCE_TYPE_REGION3=>{base_region3_walk(t,0,_REGION2_SIZE,0);},_ASCE_TYPE_REGION2=>{base_region2_walk(t,0,_REGION1_SIZE,0);},_ASCE_TYPE_REGION1=>{base_region1_walk(t,0,TASK_SIZE_MAX,0);},_=>{}}base_crst_free(t)}

unsafe fn base_pgt_cache_init()->c_int{static mut LOCK: mutex = DEFINE_MUTEX!();let sz=_PAGE_TABLE_SIZE;if !base_pgt_cache.is_null(){return 0}mutex_lock(&raw mut LOCK);if base_pgt_cache.is_null(){base_pgt_cache=kmem_cache_create(c"base_pgt",sz,sz,0,core::ptr::null_mut())}mutex_unlock(&raw mut LOCK);if !base_pgt_cache.is_null(){0}else{-ENOMEM}}

pub unsafe fn base_asce_alloc(addr:c_ulong,num_pages:c_ulong)->c_ulong{if base_pgt_cache_init()!=0{return 0}let end=addr.wrapping_add(num_pages.wrapping_mul(PAGE_SIZE));let(table,rc,ty) = if end<=_REGION3_SIZE{let t=base_crst_alloc(_SEGMENT_ENTRY_EMPTY);if t.is_null(){return 0}(t,base_segment_walk(t,addr,end,1),_ASCE_TYPE_SEGMENT)}else if end<=_REGION2_SIZE{let t=base_crst_alloc(_REGION3_ENTRY_EMPTY);if t.is_null(){return 0}(t,base_region3_walk(t,addr,end,1),_ASCE_TYPE_REGION3)}else if end<=_REGION1_SIZE{let t=base_crst_alloc(_REGION2_ENTRY_EMPTY);if t.is_null(){return 0}(t,base_region2_walk(t,addr,end,1),_ASCE_TYPE_REGION2)}else{let t=base_crst_alloc(_REGION1_ENTRY_EMPTY);if t.is_null(){return 0}(t,base_region1_walk(t,addr,end,1),_ASCE_TYPE_REGION1)};let mut asce=__pa(table)|ty|_ASCE_TABLE_LENGTH;if rc!=0{base_asce_free(asce);asce=0}asce}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
