// SPDX-License-Identifier: GPL-2.0
/* Copyright IBM Corp. 2006 */

// C dependencies are supplied by the surrounding kernel translation unit.

static mut vmem_mutex: mutex = DEFINE_MUTEX!();

unsafe fn vmem_alloc_pages(order: c_uint) -> *mut c_void {
    let size = PAGE_SIZE << order;
    if slab_is_available() { __get_free_pages(GFP_KERNEL, order) as *mut c_void }
    else { memblock_alloc(size, size) as *mut c_void }
}

unsafe fn vmem_free_pages(addr: c_ulong, order: c_int, altmap: *mut vmem_altmap) {
    if !altmap.is_null() { vmem_altmap_free(altmap, 1 << order); return; }
    let page = virt_to_page(addr as *mut c_void);
    if PageReserved(page) { free_reserved_pages(page, order); }
    else { free_pages(addr, order); }
}

pub unsafe fn vmem_crst_alloc(val: c_ulong) -> *mut c_void {
    let table = vmem_alloc_pages(CRST_ALLOC_ORDER);
    if table.is_null() { return core::ptr::null_mut(); }
    crst_table_init(table as *mut c_ulong, val);
    __arch_set_page_dat(table, 1UL << CRST_ALLOC_ORDER);
    table
}

pub unsafe fn vmem_pte_alloc() -> *mut pte_t {
    let pte = if slab_is_available() { page_table_alloc(&init_mm) as *mut pte_t }
              else { memblock_alloc(PAGE_SIZE, PAGE_SIZE) as *mut pte_t };
    if pte.is_null() { return core::ptr::null_mut(); }
    memset64(pte as *mut u64, _PAGE_INVALID, PTRS_PER_PTE);
    __arch_set_page_dat(pte as *mut c_void, 1);
    pte
}

unsafe fn vmem_pte_free(table: *mut c_ulong) { page_table_free(&init_mm, table); }

const PAGE_UNUSED: u8 = 0xFD;
static mut unused_sub_pmd_start: c_ulong = 0;

unsafe fn vmemmap_flush_unused_sub_pmd() {
    if unused_sub_pmd_start == 0 { return; }
    memset(unused_sub_pmd_start as *mut c_void, PAGE_UNUSED as c_int,
           ALIGN(unused_sub_pmd_start, PMD_SIZE) - unused_sub_pmd_start);
    unused_sub_pmd_start = 0;
}
unsafe fn vmemmap_mark_sub_pmd_used(start: c_ulong, _end: c_ulong) {
    memset(start as *mut c_void, 0, core::mem::size_of::<page>());
}
unsafe fn vmemmap_use_sub_pmd(start: c_ulong, end: c_ulong) {
    if unused_sub_pmd_start == start {
        unused_sub_pmd_start = end;
        if IS_ALIGNED(unused_sub_pmd_start, PMD_SIZE) { unused_sub_pmd_start = 0; }
        return;
    }
    vmemmap_flush_unused_sub_pmd();
    vmemmap_mark_sub_pmd_used(start, end);
}
unsafe fn vmemmap_use_new_sub_pmd(start: c_ulong, end: c_ulong) {
    let page = ALIGN_DOWN(start, PMD_SIZE);
    vmemmap_flush_unused_sub_pmd();
    vmemmap_mark_sub_pmd_used(start, end);
    if !IS_ALIGNED(start, PMD_SIZE) { memset(page as *mut c_void, PAGE_UNUSED as c_int, start - page); }
    if !IS_ALIGNED(end, PMD_SIZE) { unused_sub_pmd_start = end; }
}
unsafe fn vmemmap_unuse_sub_pmd(start: c_ulong, end: c_ulong) -> bool {
    let page = ALIGN_DOWN(start, PMD_SIZE);
    vmemmap_flush_unused_sub_pmd();
    memset(start as *mut c_void, PAGE_UNUSED as c_int, end - start);
    memchr_inv(page as *const c_void, PAGE_UNUSED as c_int, PMD_SIZE).is_null()
}

unsafe fn modify_pte_table(mut pmd: *mut pmd_t, mut addr: c_ulong, end: c_ulong, add: bool, direct: bool, altmap: *mut vmem_altmap) -> c_int {
    let prot = pgprot_val(PAGE_KERNEL); let mut pages = 0; let mut ret = -ENOMEM;
    let mut pte = pte_offset_kernel(pmd, addr);
    while addr < end {
        let entry = ptep_get(pte);
        if !add {
            if pte_none(entry) { addr += PAGE_SIZE; pte = pte.add(1); continue; }
            if !direct { vmem_free_pages(pfn_to_virt(pte_pfn(entry)) as c_ulong, get_order(PAGE_SIZE), altmap); }
            pte_clear(&init_mm, addr, pte);
        } else if pte_none(entry) {
            if !direct {
                let new_page = vmemmap_alloc_block_buf(PAGE_SIZE, NUMA_NO_NODE, altmap);
                if new_page.is_null() { break; }
                set_pte(pte, __pte(__pa(new_page) | prot));
            } else { set_pte(pte, __pte(__pa(addr as *mut c_void) | prot)); }
        } else { addr += PAGE_SIZE; pte = pte.add(1); continue; }
        pages += 1; addr += PAGE_SIZE; pte = pte.add(1);
    }
    if addr >= end { ret = 0; }
    if direct { update_page_count(PG_DIRECT_MAP_4K, if add { pages } else { -pages }); }
    ret
}

unsafe fn try_free_pte_table(pmd: *mut pmd_t, start: c_ulong) {
    let mut pte = pte_offset_kernel(pmd, start);
    for _ in 0..PTRS_PER_PTE { if !pte_none(ptep_get(pte)) { return; } pte = pte.add(1); }
    vmem_pte_free(pmd_deref(pmdp_get(pmd)) as *mut c_ulong); pmd_clear(pmd);
}

unsafe fn modify_pmd_table(mut pud: *mut pud_t, mut addr: c_ulong, end: c_ulong, add: bool, direct: bool, altmap: *mut vmem_altmap) -> c_int {
    let prot = pgprot_val(SEGMENT_KERNEL); let mut pages = 0; let mut ret = -ENOMEM;
    let mut pmd = pmd_offset(pud, addr);
    while addr < end {
        let next = pmd_addr_end(addr, end); let entry = pmdp_get(pmd);
        if !add {
            if pmd_none(entry) { addr = next; pmd = pmd.add(1); continue; }
            if pmd_leaf(entry) {
                if IS_ALIGNED(addr, PMD_SIZE) && IS_ALIGNED(next, PMD_SIZE) { if !direct { vmem_free_pages(pmd_deref(entry), get_order(PMD_SIZE), altmap); } pmd_clear(pmd); pages += 1; }
                else if !direct && vmemmap_unuse_sub_pmd(addr, next) { vmem_free_pages(pmd_deref(entry), get_order(PMD_SIZE), altmap); pmd_clear(pmd); }
                addr = next; pmd = pmd.add(1); continue;
            }
        } else if pmd_none(entry) {
            if IS_ALIGNED(addr, PMD_SIZE) && IS_ALIGNED(next, PMD_SIZE) && cpu_has_edat1() && direct && !debug_pagealloc_enabled() { set_pmd(pmd, __pmd(__pa(addr as *mut c_void) | prot)); pages += 1; addr = next; pmd = pmd.add(1); continue; }
            if !direct && cpu_has_edat1() { let new_page = vmemmap_alloc_block_buf(PMD_SIZE, NUMA_NO_NODE, altmap); if !new_page.is_null() { set_pmd(pmd, __pmd(__pa(new_page) | prot)); if !IS_ALIGNED(addr, PMD_SIZE) || !IS_ALIGNED(next, PMD_SIZE) { vmemmap_use_new_sub_pmd(addr, next); } addr = next; pmd = pmd.add(1); continue; } }
            let pte = vmem_pte_alloc(); if pte.is_null() { break; } pmd_populate(&init_mm, pmd, pte);
        } else if pmd_leaf(entry) { if !direct { vmemmap_use_sub_pmd(addr, next); } addr = next; pmd = pmd.add(1); continue; }
        ret = modify_pte_table(pmd, addr, next, add, direct, altmap); if ret != 0 { break; }
        if !add { try_free_pte_table(pmd, addr & PMD_MASK); }
        addr = next; pmd = pmd.add(1);
    }
    if addr >= end { ret = 0; } if direct { update_page_count(PG_DIRECT_MAP_1M, if add { pages } else { -pages }); } ret
}

unsafe fn try_free_pmd_table(pud: *mut pud_t, start: c_ulong) { let mut pmd = pmd_offset(pud, start); for _ in 0..PTRS_PER_PMD { if !pmd_none(pmdp_get(pmd)) { return; } pmd = pmd.add(1); } vmem_free_pages(pud_deref(pudp_get(pud)), CRST_ALLOC_ORDER, core::ptr::null_mut()); pud_clear(pud); }

unsafe fn modify_pud_table(mut p4d: *mut p4d_t, mut addr: c_ulong, end: c_ulong, add: bool, direct: bool, altmap: *mut vmem_altmap) -> c_int {
    let prot = pgprot_val(REGION3_KERNEL); let mut pages = 0; let mut ret = -ENOMEM; let mut pud = pud_offset(p4d, addr);
    while addr < end { let next = pud_addr_end(addr, end); let entry = pudp_get(pud);
        if !add { if pud_none(entry) { addr=next; pud=pud.add(1); continue; } if pud_leaf(entry) { if IS_ALIGNED(addr,PUD_SIZE)&&IS_ALIGNED(next,PUD_SIZE) { if !direct { vmem_free_pages(pud_deref(entry),get_order(PUD_SIZE),altmap); } pud_clear(pud); pages+=1; addr=next;pud=pud.add(1);continue; } else { split_pud_page(pud,addr&PUD_MASK); } } }
        else if pud_none(entry) { if IS_ALIGNED(addr,PUD_SIZE)&&IS_ALIGNED(next,PUD_SIZE)&&cpu_has_edat2()&&direct&&!debug_pagealloc_enabled() { set_pud(pud,__pud(__pa(addr as *mut c_void)|prot));pages+=1;addr=next;pud=pud.add(1);continue; } let pmd=vmem_crst_alloc(_SEGMENT_ENTRY_EMPTY);if pmd.is_null(){break;}pud_populate(&init_mm,pud,pmd); }
        else if pud_leaf(entry) { addr=next;pud=pud.add(1);continue; }
        ret=modify_pmd_table(pud,addr,next,add,direct,altmap);if ret!=0{break;}if !add{try_free_pmd_table(pud,addr&PUD_MASK);}addr=next;pud=pud.add(1);
    } if addr>=end{ret=0;}if direct{update_page_count(PG_DIRECT_MAP_2G,if add{pages}else{-pages});}ret
}

unsafe fn try_free_pud_table(p4d:*mut p4d_t,start:c_ulong){let mut pud=pud_offset(p4d,start);for _ in 0..PTRS_PER_PUD{if !pud_none(pudp_get(pud)){return;}pud=pud.add(1);}vmem_free_pages(p4d_deref(p4dp_get(p4d)),CRST_ALLOC_ORDER,core::ptr::null_mut());p4d_clear(p4d);}

unsafe fn modify_p4d_table(mut pgd:*mut pgd_t,mut addr:c_ulong,end:c_ulong,add:bool,direct:bool,altmap:*mut vmem_altmap)->c_int{let mut ret=-ENOMEM;let mut p4d=p4d_offset(pgd,addr);while addr<end{let next=p4d_addr_end(addr,end);let entry=p4dp_get(p4d);if !add{if p4d_none(entry){addr=next;p4d=p4d.add(1);continue;}}else if p4d_none(entry){let pud=vmem_crst_alloc(_REGION3_ENTRY_EMPTY);if pud.is_null(){break;}p4d_populate(&init_mm,p4d,pud);}ret=modify_pud_table(p4d,addr,next,add,direct,altmap);if ret!=0{break;}if !add{try_free_pud_table(p4d,addr&P4D_MASK);}addr=next;p4d=p4d.add(1);}if addr>=end{ret=0;}ret}
unsafe fn try_free_p4d_table(pgd:*mut pgd_t,start:c_ulong){let mut p4d=p4d_offset(pgd,start);for _ in 0..PTRS_PER_P4D{if !p4d_none(p4dp_get(p4d)){return;}p4d=p4d.add(1);}vmem_free_pages(pgd_deref(pgdp_get(pgd)),CRST_ALLOC_ORDER,core::ptr::null_mut());pgd_clear(pgd);}

unsafe fn modify_pagetable(start:c_ulong,end:c_ulong,add:bool,direct:bool,altmap:*mut vmem_altmap)->c_int{let mut addr=start;let mut ret=-ENOMEM;if WARN_ON_ONCE(!PAGE_ALIGNED(start|end)){return -EINVAL;}/* CONFIG_KASAN range check is supplied by the build configuration. */if WARN_ON_ONCE(end>__abs_lowcore){return -EINVAL;}while addr<end{let next=pgd_addr_end(addr,end);let pgd=pgd_offset_k(addr);if !add{if pgd_none(pgdp_get(pgd)){addr=next;continue;}}else if pgd_none(pgdp_get(pgd)){let p4d=vmem_crst_alloc(_REGION2_ENTRY_EMPTY);if p4d.is_null(){break;}pgd_populate(&init_mm,pgd,p4d);}ret=modify_p4d_table(pgd,addr,next,add,direct,altmap);if ret!=0{break;}if !add{try_free_p4d_table(pgd,addr&PGDIR_MASK);}addr=next;}if addr>=end{ret=0;}if !add{flush_tlb_kernel_range(start,end);}ret}
unsafe fn add_pagetable(start:c_ulong,end:c_ulong,direct:bool,altmap:*mut vmem_altmap)->c_int{modify_pagetable(start,end,true,direct,altmap)}
unsafe fn remove_pagetable(start:c_ulong,end:c_ulong,direct:bool,altmap:*mut vmem_altmap)->c_int{modify_pagetable(start,end,false,direct,altmap)}

unsafe fn vmem_add_range(mut start:c_ulong,size:c_ulong)->c_int{start=__va(start) as c_ulong;add_pagetable(start,start+size,true,core::ptr::null_mut())}
unsafe fn vmem_remove_range(mut start:c_ulong,size:c_ulong){start=__va(start) as c_ulong;remove_pagetable(start,start+size,true,core::ptr::null_mut());}

pub unsafe fn vmemmap_populate(start:c_ulong,end:c_ulong,_node:c_int,altmap:*mut vmem_altmap)->c_int{mutex_lock(&mut vmem_mutex);let ret=add_pagetable(start,end,false,altmap);if ret!=0{remove_pagetable(start,end,false,altmap);}mutex_unlock(&mut vmem_mutex);ret}
// CONFIG_MEMORY_HOTPLUG
pub unsafe fn vmemmap_free(start:c_ulong,end:c_ulong,altmap:*mut vmem_altmap){mutex_lock(&mut vmem_mutex);remove_pagetable(start,end,false,altmap);mutex_unlock(&mut vmem_mutex);}
pub unsafe fn vmem_remove_mapping(start:c_ulong,size:c_ulong){mutex_lock(&mut vmem_mutex);vmem_remove_range(start,size);mutex_unlock(&mut vmem_mutex);}
pub unsafe fn arch_get_mappable_range()->range{range{start:0,end:max_mappable-1}}
pub unsafe fn vmem_add_mapping(start:c_ulong,size:c_ulong)->c_int{let r=arch_get_mappable_range();if start<r.start||start+size>r.end+1||start+size<start{return -ERANGE;}mutex_lock(&mut vmem_mutex);let ret=vmem_add_range(start,size);if ret!=0{vmem_remove_range(start,size);}mutex_unlock(&mut vmem_mutex);ret}

pub unsafe fn vmem_get_alloc_pte(addr:c_ulong,alloc:bool)->*mut pte_t{let mut ptep=core::ptr::null_mut();let pgd=pgd_offset_k(addr);if pgd_none(pgdp_get(pgd)){if !alloc{return ptep;}let p4d=vmem_crst_alloc(_REGION2_ENTRY_EMPTY);if p4d.is_null(){return ptep;}pgd_populate(&init_mm,pgd,p4d);}let p4d=p4d_offset(pgd,addr);if p4d_none(p4dp_get(p4d)){if !alloc{return ptep;}let pud=vmem_crst_alloc(_REGION3_ENTRY_EMPTY);if pud.is_null(){return ptep;}p4d_populate(&init_mm,p4d,pud);}let pud=pud_offset(p4d,addr);let pe=pudp_get(pud);if pud_none(pe){if !alloc{return ptep;}let pmd=vmem_crst_alloc(_SEGMENT_ENTRY_EMPTY);if pmd.is_null(){return ptep;}pud_populate(&init_mm,pud,pmd);}else if WARN_ON_ONCE(pud_leaf(pe)){return ptep;}let pmd=pmd_offset(pud,addr);let me=pmdp_get(pmd);if pmd_none(me){if !alloc{return ptep;}let pte=vmem_pte_alloc();if pte.is_null(){return ptep;}pmd_populate(&init_mm,pmd,pte);}else if WARN_ON_ONCE(pmd_leaf(me)){return ptep;}ptep=pte_offset_kernel(pmd,addr);ptep}

pub unsafe fn __vmem_map_4k_page(addr:c_ulong,phys:c_ulong,prot:pgprot_t,alloc:bool)->c_int{if !IS_ALIGNED(addr,PAGE_SIZE){return -EINVAL;}let ptep=vmem_get_alloc_pte(addr,alloc);if ptep.is_null(){return -ENOMEM;}__ptep_ipte(addr,ptep,0,0,IPTE_GLOBAL);set_pte(ptep,mk_pte_phys(phys,prot));0}
pub unsafe fn vmem_map_4k_page(addr:c_ulong,phys:c_ulong,prot:pgprot_t)->c_int{mutex_lock(&mut vmem_mutex);let rc=__vmem_map_4k_page(addr,phys,prot,true);mutex_unlock(&mut vmem_mutex);rc}
pub unsafe fn vmem_unmap_4k_page(addr:c_ulong){mutex_lock(&mut vmem_mutex);let ptep=virt_to_kpte(addr);__ptep_ipte(addr,ptep,0,0,IPTE_GLOBAL);pte_clear(&init_mm,addr,ptep);mutex_unlock(&mut vmem_mutex);}
pub unsafe fn vmem_map_init(){__set_memory_rox(_stext,_etext);__set_memory_ro(_etext,__end_rodata);__set_memory_rox(__stext_amode31,__etext_amode31);if !cpu_has_bear(){set_memory_x(0,1);}if debug_pagealloc_enabled(){__set_memory_4k(__va(0),absolute_pointer(__va(0))+ident_map_size);}pr_info!("Write protected kernel read-only data: %luk\n",(__end_rodata-_stext)>>10);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
