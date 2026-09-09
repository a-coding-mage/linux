// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/m68k/mm/motorola.c
 *
 * Routines specific to the Motorola MMU, originally from:
 * linux/arch/m68k/init.c
 * which are Copyright (C) 1995 Hamish Macdonald
 *
 * Moved 8/20/1999 Sam Creasey
 */

// C header dependencies are supplied by the surrounding kernel translation.

#[cfg(not(mm_cachebits))]
pub static mut mm_cachebits: c_ulong = 0;

#[inline]
unsafe fn nocache_page(vaddr: *mut c_void) {
    let addr = vaddr as c_ulong;
    if CPU_IS_040_OR_060 {
        let ptep: *mut pte_t = virt_to_kpte(addr);
        *ptep = pte_mknocache(*ptep);
    }
}

#[inline]
unsafe fn cache_page(vaddr: *mut c_void) {
    let addr = vaddr as c_ulong;
    if CPU_IS_040_OR_060 {
        let ptep: *mut pte_t = virt_to_kpte(addr);
        *ptep = pte_mkcache(*ptep);
    }
}

pub unsafe fn mmu_page_ctor(page: *mut c_void) {
    __flush_pages_to_ram(page, 1);
    flush_tlb_kernel_page(page);
    nocache_page(page);
}

pub unsafe fn mmu_page_dtor(page: *mut c_void) { cache_page(page); }

type ptable_desc = list_head;

static mut ptable_list: [list_head; 3] = [
    LIST_HEAD_INIT!(ptable_list[0]), LIST_HEAD_INIT!(ptable_list[1]),
    LIST_HEAD_INIT!(ptable_list[2]),
];

#[inline]
unsafe fn pd_ptable(ptdesc: *mut c_void) -> *mut ptable_desc {
    &mut (*virt_to_ptdesc(ptdesc).cast::<ptdesc>()).pt_list
}
#[inline]
unsafe fn pd_ptdesc(ptable: *mut ptable_desc) -> *mut ptdesc {
    list_entry!(ptable, ptdesc, pt_list)
}
#[inline]
unsafe fn pd_markbits(dp: *mut ptable_desc) -> *mut c_uint {
    &mut (*pd_ptdesc(dp)).pt_index.cast::<c_uint>()
}

static ptable_shift: [c_int; 3] = [7 + 2, 7 + 2, 6 + 2];

#[inline]
unsafe fn ptable_size(kind: usize) -> c_uint { 1u32 << ptable_shift[kind] }
#[inline]
unsafe fn ptable_mask(kind: usize) -> c_uint {
    (1u32 << (PAGE_SIZE / ptable_size(kind))) - 1
}

pub unsafe fn init_pointer_table(table: *mut c_void, kind: c_int) {
    let ptable = table as c_ulong;
    let pt_addr = ptable & PAGE_MASK;
    let mask = 1u32 << ((ptable - pt_addr) / ptable_size(kind as usize));
    let dp = pd_ptable(pt_addr as *mut c_void);
    if (*pd_markbits(dp) & mask) == 0 {
        *pd_markbits(dp) = ptable_mask(kind as usize);
        list_add(dp, &mut ptable_list[kind as usize]);
    }
    *pd_markbits(dp) &= !mask;
    pr_debug!("init_pointer_table: %lx, %x\n", ptable, *pd_markbits(dp));
    __ClearPageReserved(ptdesc_page(pd_ptdesc(dp)));
    init_page_count(ptdesc_page(pd_ptdesc(dp)));
}

pub unsafe fn get_pointer_table(mm: *mut mm_struct, kind: c_int) -> *mut c_void {
    let dp = ptable_list[kind as usize].next;
    let mut mask = if list_empty(&ptable_list[kind as usize]) { 0 } else { *pd_markbits(dp) };
    if mask == 0 {
        let ptdesc = pagetable_alloc(GFP_KERNEL | __GFP_ZERO, 0);
        if ptdesc.is_null() { return core::ptr::null_mut(); }
        let pt_addr = ptdesc_address(ptdesc);
        match kind {
            TABLE_PTE => pagetable_pte_ctor(mm, ptdesc),
            TABLE_PMD => pagetable_pmd_ctor(mm, ptdesc),
            TABLE_PGD => pagetable_pgd_ctor(ptdesc),
            _ => (),
        }
        mmu_page_ctor(pt_addr);
        let new = pd_ptable(pt_addr);
        *pd_markbits(new) = ptable_mask(kind as usize) - 1;
        list_add_tail(new, dp);
        return pt_addr as *mut pmd_t as *mut c_void;
    }
    let mut tmp = 1u32;
    let mut off = 0u32;
    while (mask & tmp) == 0 { tmp <<= 1; off += ptable_size(kind as usize); }
    *pd_markbits(dp) = mask & !tmp;
    if *pd_markbits(dp) == 0 { list_move_tail(dp, &mut ptable_list[kind as usize]); }
    (ptdesc_address(pd_ptdesc(dp)) as c_ulong + off as c_ulong) as *mut c_void
}

pub unsafe fn free_pointer_table(table: *mut c_void, kind: c_int) -> c_int {
    let ptable = table as c_ulong;
    let pt_addr = ptable & PAGE_MASK;
    let mask = 1u32 << ((ptable - pt_addr) / ptable_size(kind as usize));
    let dp = pd_ptable(pt_addr as *mut c_void);
    if (*pd_markbits(dp) & mask) != 0 { panic!("table already free!"); }
    *pd_markbits(dp) |= mask;
    if *pd_markbits(dp) == ptable_mask(kind as usize) {
        list_del(dp); mmu_page_dtor(pt_addr as *mut c_void);
        pagetable_dtor_free(virt_to_ptdesc(pt_addr as *mut c_void)); return 1;
    } else if ptable_list[kind as usize].next != dp { list_move(dp, &mut ptable_list[kind as usize]); }
    0
}

extern "C" {
    static mut m68k_init_mapped_size: c_ulong;
    static mut availmem: c_ulong;
}
static mut last_pte_table: *mut pte_t = core::ptr::null_mut();

unsafe fn kernel_page_table() -> *mut pte_t {
    let pte_table = last_pte_table;
    if PAGE_ALIGNED(last_pte_table) {
        let pte_table = memblock_alloc_low(PAGE_SIZE, PAGE_SIZE) as *mut pte_t;
        if pte_table.is_null() { panic!("{}: Failed to allocate {} bytes align={}\n", "kernel_page_table", PAGE_SIZE, PAGE_SIZE); }
        clear_page(pte_table as *mut c_void); mmu_page_ctor(pte_table as *mut c_void); last_pte_table = pte_table;
    }
    last_pte_table = last_pte_table.add(PTRS_PER_PTE as usize); pte_table
}

static mut last_pmd_table: *mut pmd_t = core::ptr::null_mut();
unsafe fn kernel_ptr_table() -> *mut pmd_t {
    if last_pmd_table.is_null() {
        let mut last = kernel_pg_dir as c_ulong;
        for i in 0..PTRS_PER_PGD as usize {
            let pud = &mut *(&mut kernel_pg_dir[i] as *mut _ as *mut pud_t);
            if !pud_present(*pud) { continue; }
            let pmd = pgd_page_vaddr(kernel_pg_dir[i]); if pmd > last { last = pmd; }
        }
        last_pmd_table = last as *mut pmd_t;
    }
    last_pmd_table = last_pmd_table.add(PTRS_PER_PMD as usize);
    if PAGE_ALIGNED(last_pmd_table) { last_pmd_table = memblock_alloc_low(PAGE_SIZE, PAGE_SIZE) as *mut pmd_t; if last_pmd_table.is_null() { panic!("allocation failed"); } clear_page(last_pmd_table as *mut c_void); mmu_page_ctor(last_pmd_table as *mut c_void); }
    last_pmd_table
}

// The remaining mapping and paging initialization follows the C control flow.
unsafe fn map_node(node: c_int) {
    let mut size = m68k_memory[node as usize].size;
    let mut physaddr = m68k_memory[node as usize].addr;
    let mut virtaddr = phys_to_virt(physaddr) as c_ulong;
    physaddr |= m68k_supervisor_cachemode | _PAGE_PRESENT | _PAGE_ACCESSED | _PAGE_DIRTY;
    if CPU_IS_040_OR_060 { physaddr |= _PAGE_GLOBAL040; }
    while size > 0 {
        let pgd_dir = pgd_offset_k(virtaddr);
        if virtaddr != 0 && CPU_IS_020_OR_030 && (virtaddr & (PGDIR_SIZE - 1)) == 0 && size >= PGDIR_SIZE { pgd_val(*pgd_dir) = physaddr; size -= PGDIR_SIZE; virtaddr += PGDIR_SIZE; physaddr += PGDIR_SIZE; continue; }
        let p4d_dir = p4d_offset(pgd_dir, virtaddr); let pud_dir = pud_offset(p4d_dir, virtaddr);
        let pmd_dir = if !pud_present(*pud_dir) { let p = kernel_ptr_table(); pud_set(pud_dir, p); p } else { pmd_offset(pud_dir, virtaddr) };
        if CPU_IS_020_OR_030 { if virtaddr != 0 { pmd_val(*pmd_dir) = physaddr; physaddr += PMD_SIZE; } else { let pte_dir = kernel_page_table(); pmd_set(pmd_dir, pte_dir); pte_val(*pte_dir) = 0; physaddr += PAGE_SIZE; for i in 1..PTRS_PER_PTE { pte_val(*pte_dir.add(i as usize)) = physaddr; physaddr += PAGE_SIZE; } } size -= PMD_SIZE; virtaddr += PMD_SIZE; }
        else { let pte_dir = if !pmd_present(*pmd_dir) { let p = kernel_page_table(); pmd_set(pmd_dir, p); p } else { pte_offset_kernel(pmd_dir, virtaddr) }; if virtaddr != 0 { if !pte_present(*pte_dir) { pte_val(*pte_dir) = physaddr; } } else { pte_val(*pte_dir) = 0; } size -= PAGE_SIZE; virtaddr += PAGE_SIZE; physaddr += PAGE_SIZE; }
    }
}

// Protection-map constants and paging_init are translated below; external kernel symbols retain their declarations.
pub const PAGE_NONE_C: pgprot_t = __pgprot(_PAGE_PROTNONE | _PAGE_ACCESSED);
pub const PAGE_SHARED_C: pgprot_t = __pgprot(_PAGE_PRESENT | _PAGE_ACCESSED);
pub const PAGE_COPY_C: pgprot_t = __pgprot(_PAGE_PRESENT | _PAGE_RONLY | _PAGE_ACCESSED);
pub const PAGE_READONLY_C: pgprot_t = PAGE_COPY_C;

static mut protection_map: [pgprot_t; 16] = [
    PAGE_NONE_C, PAGE_READONLY_C, PAGE_COPY_C, PAGE_COPY_C,
    PAGE_READONLY_C, PAGE_READONLY_C, PAGE_COPY_C, PAGE_COPY_C,
    PAGE_NONE_C, PAGE_READONLY_C, PAGE_SHARED_C, PAGE_SHARED_C,
    PAGE_READONLY_C, PAGE_READONLY_C, PAGE_SHARED_C, PAGE_SHARED_C,
];

pub unsafe fn paging_init() {
    let min_addr = m68k_memory[0].addr;
    let mut max_addr = min_addr + m68k_memory[0].size - 1;
    memblock_add_node(m68k_memory[0].addr, m68k_memory[0].size, 0, MEMBLOCK_NONE);
    let mut i = 1;
    while i < m68k_num_memory {
        if m68k_memory[i].addr < min_addr {
            printk!("Ignoring memory chunk at 0x%lx:0x%lx before the first chunk\n", m68k_memory[i].addr, m68k_memory[i].size);
            printk!("Fix your bootloader or use a memfile to make use of this area!\n");
            m68k_num_memory -= 1;
            memmove(m68k_memory.add(i), m68k_memory.add(i + 1), (m68k_num_memory - i) * core::mem::size_of::<m68k_mem_info>());
            continue;
        }
        memblock_add_node(m68k_memory[i].addr, m68k_memory[i].size, i, MEMBLOCK_NONE);
        let addr = m68k_memory[i].addr + m68k_memory[i].size - 1;
        if addr > max_addr { max_addr = addr; }
        i += 1;
    }
    m68k_memoffset = min_addr - PAGE_OFFSET;
    m68k_virt_to_node_shift = fls(max_addr - min_addr) - 6;
    module_fixup(core::ptr::null_mut(), __start_fixup, __stop_fixup);
    flush_icache();
    high_memory = phys_to_virt(max_addr) + 1;
    min_low_pfn = availmem >> PAGE_SHIFT;
    max_pfn = (max_addr >> PAGE_SHIFT) + 1;
    max_low_pfn = max_pfn;
    memblock_reserve(m68k_memory[0].addr, availmem - m68k_memory[0].addr);
    memblock_set_bottom_up(true);
    for n in 0..m68k_num_memory { m68k_setup_node(n); map_node(n as c_int); }
    flush_tlb_all();
    early_memtest(min_addr, max_addr);
    set_fc(USER_DATA);
    for n in 0..m68k_num_memory { if node_present_pages(n) { node_set_state(n, N_NORMAL_MEMORY); } }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
