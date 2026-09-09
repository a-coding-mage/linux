// SPDX-License-Identifier: GPL-2.0
/*
 * This file implements KASLR memory randomization for x86_64. It randomizes
 * the virtual address space of kernel memory regions (physical memory
 * mapping, vmalloc & vmemmap) for x86_64. This security feature mitigates
 * exploits relying on predictable kernel addresses.
 *
 * Entropy is generated using the KASLR early boot functions now shared in
 * the lib directory (originally written by Kees Cook). Randomization is
 * done on PGD & P4D/PUD page table levels to increase possible addresses.
 * The physical memory mapping code was adapted to support P4D/PUD level
 * virtual addresses. This implementation on the best configuration provides
 * 30,000 possible virtual addresses in average for each memory region.
 * An additional low memory page is used to ensure each CPU can start with a
 * PGD aligned virtual address (for realmode).
 *
 * The order of each memory region is not changed. The feature looks at the
 * available space for the regions based on different configuration options
 * and randomizes the base and space between each. The size of the physical
 * memory mapping is the available physical memory.
 */

const TB_SHIFT: usize = 40;

/* Configuration/build-provided symbols and types are supplied by other units. */
extern "C" {
    static mut page_offset_base: usize;
    static mut direct_map_physmem_end: usize;
    static mut vmalloc_base: usize;
    static mut vmemmap_base: usize;
    static mut trampoline_pgd_entry: pgd_t;
}

#[repr(C)]
struct kaslr_memory_region {
    base: *mut usize,
    end: *mut usize,
    size_tb: usize,
}

static VADDR_END: usize = CPU_ENTRY_AREA_BASE;

static mut kaslr_regions: [kaslr_memory_region; 3] = [
    kaslr_memory_region { base: unsafe { &raw mut page_offset_base }, end: unsafe { &raw mut direct_map_physmem_end }, size_tb: 0 },
    kaslr_memory_region { base: unsafe { &raw mut vmalloc_base }, end: core::ptr::null_mut(), size_tb: 0 },
    kaslr_memory_region { base: unsafe { &raw mut vmemmap_base }, end: core::ptr::null_mut(), size_tb: 0 },
];

#[inline]
unsafe fn get_padding(region: *const kaslr_memory_region) -> usize {
    (*region).size_tb << TB_SHIFT
}

unsafe extern "C" {
    fn pgtable_l5_enabled() -> bool;
    fn kaslr_memory_enabled() -> bool;
    fn kaslr_get_random_long(name: *const u8) -> usize;
    fn prandom_seed_state(state: *mut rnd_state, seed: usize);
    fn prandom_bytes_state(state: *mut rnd_state, bytes: *mut u8, len: usize);
    fn alloc_low_page() -> *mut u8;
    fn __va(paddr: usize) -> *mut u8;
    fn pgd_offset_k(vaddr: usize) -> *mut pgd_t;
    fn p4d_offset(pgd: *mut pgd_t, vaddr: usize) -> *mut p4d_t;
    fn pud_offset(p4d: *mut p4d_t, vaddr: usize) -> *mut pud_t;
    fn pud_index(paddr: usize) -> usize;
    fn p4d_index(paddr: usize) -> usize;
    fn set_p4d(dst: *mut p4d_t, value: p4d_t);
    fn __pa_nodebug(vaddr: usize) -> usize;
    fn round_up(value: usize, align: usize) -> usize;
}

#[repr(C)] struct rnd_state { _private: [u8; 0] }
#[repr(C)] struct pgd_t { val: usize }
#[repr(C)] struct p4d_t { val: usize }
#[repr(C)] struct pud_t { val: usize }

unsafe extern "C" {
    fn __p4d(value: usize) -> p4d_t;
    fn __pgd(value: usize) -> pgd_t;
}

#[inline]
pub unsafe extern "C" fn kernel_randomize_memory() {
    let mut vaddr_start = if pgtable_l5_enabled() { __PAGE_OFFSET_BASE_L5 } else { __PAGE_OFFSET_BASE_L4 };
    let mut vaddr = vaddr_start;
    let mut rand: usize = 0;
    let mut memory_tb: usize;
    let mut rand_state = core::mem::MaybeUninit::<rnd_state>::uninit();
    let mut remain_entropy: usize;
    let mut vmemmap_size: usize;

    /* BUILD_BUG_ON checks preserve the source layout invariants. */
    debug_assert!(vaddr_start < VADDR_END);
    debug_assert!(VADDR_END == CPU_ENTRY_AREA_BASE);
    debug_assert!(VADDR_END <= __START_KERNEL_map);

    direct_map_physmem_end = (1usize << MAX_PHYSMEM_BITS) - 1;
    if !kaslr_memory_enabled() { return; }

    kaslr_regions[0].size_tb = 1usize << (MAX_PHYSMEM_BITS - TB_SHIFT);
    kaslr_regions[1].size_tb = VMALLOC_SIZE_TB;

    debug_assert!(kaslr_regions[0].base == &raw mut page_offset_base);
    memory_tb = ((max_pfn << PAGE_SHIFT) + ((1usize << TB_SHIFT) - 1)) / (1usize << TB_SHIFT)
        + CONFIG_RANDOMIZE_MEMORY_PHYSICAL_PADDING;
    if !IS_ENABLED_CONFIG_ZONE_DEVICE && memory_tb < kaslr_regions[0].size_tb {
        kaslr_regions[0].size_tb = memory_tb;
    }

    vmemmap_size = (kaslr_regions[0].size_tb << (TB_SHIFT - PAGE_SHIFT)) * core::mem::size_of::<page>();
    kaslr_regions[2].size_tb = (vmemmap_size + ((1usize << TB_SHIFT) - 1)) / (1usize << TB_SHIFT);

    remain_entropy = VADDR_END - vaddr_start;
    for region in kaslr_regions.iter() { remain_entropy -= get_padding(region); }

    prandom_seed_state(rand_state.as_mut_ptr(), kaslr_get_random_long(b"Memory\0".as_ptr()));
    for i in 0..kaslr_regions.len() {
        let mut entropy = remain_entropy / (kaslr_regions.len() - i);
        prandom_bytes_state(rand_state.as_mut_ptr(), &mut rand as *mut usize as *mut u8, core::mem::size_of::<usize>());
        entropy = (rand % (entropy + 1)) & PUD_MASK;
        vaddr += entropy;
        *kaslr_regions[i].base = vaddr;
        vaddr += get_padding(&kaslr_regions[i]);
        if !kaslr_regions[i].end.is_null() { *kaslr_regions[i].end = __pa_nodebug(vaddr - 1); }
        vaddr = round_up(vaddr + 1, PUD_SIZE);
        remain_entropy -= entropy;
    }
}

pub unsafe extern "C" fn init_trampoline_kaslr() {
    let pud_page_tramp = alloc_low_page() as *mut pud_t;
    let paddr = 0usize;
    let vaddr = __va(paddr) as usize;
    let pgd = pgd_offset_k(vaddr);
    let p4d = p4d_offset(pgd, vaddr);
    let pud = pud_offset(p4d, vaddr);
    let pud_tramp = pud_page_tramp.add(pud_index(paddr));
    *pud_tramp = *pud;

    if pgtable_l5_enabled() {
        let p4d_page_tramp = alloc_low_page() as *mut p4d_t;
        let p4d_tramp = p4d_page_tramp.add(p4d_index(paddr));
        set_p4d(p4d_tramp, __p4d(_KERNPG_TABLE | __pa(pud_page_tramp as usize)));
        trampoline_pgd_entry = __pgd(_KERNPG_TABLE | __pa(p4d_page_tramp as usize));
    } else {
        trampoline_pgd_entry = __pgd(_KERNPG_TABLE | __pa(pud_page_tramp as usize));
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
