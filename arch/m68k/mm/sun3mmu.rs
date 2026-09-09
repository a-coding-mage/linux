// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/m68k/mm/sun3mmu.c
 *
 * Implementations of mm routines specific to the sun3 MMU.
 *
 * Moved here 8/20/1999 Sam Creasey
 *
 */

// Linux and architecture dependencies supplied by the surrounding translation unit.

pub static bad_pmd_string: &[u8] = b"Bad pmd in pte_alloc: %08lx\n\0";

extern "C" {
    pub static mut num_pages: ::core::ffi::c_ulong;
}

/* For the sun3 we try to follow the i386 paging_init() more closely */
/* start_mem and end_mem have PAGE_OFFSET added already */
/* now sets up tables using sun3 PTEs rather than i386 as before. --m */
pub unsafe fn paging_init() {
    let mut pg_dir: *mut pgd_t;
    let mut pg_table: *mut pte_t;
    let mut i: ::core::ffi::c_int;
    let mut address: ::core::ffi::c_ulong;
    let mut next_pgtable: ::core::ffi::c_ulong;
    let mut bootmem_end: ::core::ffi::c_ulong;
    let mut size: ::core::ffi::c_ulong;

    address = PAGE_OFFSET;
    pg_dir = swapper_pg_dir;
    memset(
        swapper_pg_dir as *mut ::core::ffi::c_void,
        0,
        core::mem::size_of_val(&swapper_pg_dir),
    );
    memset(
        kernel_pg_dir as *mut ::core::ffi::c_void,
        0,
        core::mem::size_of_val(&kernel_pg_dir),
    );

    size = num_pages * core::mem::size_of::<pte_t>() as ::core::ffi::c_ulong;
    size = (size + PAGE_SIZE) & !(PAGE_SIZE - 1);

    next_pgtable = memblock_alloc_or_panic(size, PAGE_SIZE) as ::core::ffi::c_ulong;
    bootmem_end = (next_pgtable + size + PAGE_SIZE) & PAGE_MASK;

    /* Map whole memory from PAGE_OFFSET (0x0E000000) */
    pg_dir = pg_dir.add((PAGE_OFFSET >> PGDIR_SHIFT) as usize);

    while address < high_memory as ::core::ffi::c_ulong {
        pg_table = __pa(next_pgtable) as *mut pte_t;
        next_pgtable += PTRS_PER_PTE * core::mem::size_of::<pte_t>() as ::core::ffi::c_ulong;
        pgd_val(*pg_dir) = pg_table as ::core::ffi::c_ulong;
        pg_dir = pg_dir.add(1);

        /* now change pg_table to kernel virtual addresses */
        pg_table = __va(pg_table as ::core::ffi::c_ulong) as *mut pte_t;
        i = 0;
        while i < PTRS_PER_PTE {
            let mut pte: pte_t = pfn_pte(virt_to_pfn(address as *mut ::core::ffi::c_void), PAGE_INIT);
            if address >= high_memory as ::core::ffi::c_ulong {
                pte_val(pte) = 0;
            }
            set_pte(pg_table, pte);
            pg_table = pg_table.add(1);
            i += 1;
            address += PAGE_SIZE;
        }
    }

    mmu_emu_init(bootmem_end);

    (*current).mm = core::ptr::null_mut();
}

static protection_map: [pgprot_t; 16] = [
    [VM_NONE] = PAGE_NONE,
    [VM_READ] = PAGE_READONLY,
    [VM_WRITE] = PAGE_COPY,
    [VM_WRITE | VM_READ] = PAGE_COPY,
    [VM_EXEC] = PAGE_READONLY,
    [VM_EXEC | VM_READ] = PAGE_READONLY,
    [VM_EXEC | VM_WRITE] = PAGE_COPY,
    [VM_EXEC | VM_WRITE | VM_READ] = PAGE_COPY,
    [VM_SHARED] = PAGE_NONE,
    [VM_SHARED | VM_READ] = PAGE_READONLY,
    [VM_SHARED | VM_WRITE] = PAGE_SHARED,
    [VM_SHARED | VM_WRITE | VM_READ] = PAGE_SHARED,
    [VM_SHARED | VM_EXEC] = PAGE_READONLY,
    [VM_SHARED | VM_EXEC | VM_READ] = PAGE_READONLY,
    [VM_SHARED | VM_EXEC | VM_WRITE] = PAGE_SHARED,
    [VM_SHARED | VM_EXEC | VM_WRITE | VM_READ] = PAGE_SHARED,
];

DECLARE_VM_GET_PAGE_PROT!();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
