// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/m68k/mm/init.c
 *
 *  Copyright (C) 1995  Hamish Macdonald
 *
 *  Contains common initialization routines, specific init code moved
 *  to motorola.c and sun3mmu.c
 */

// C headers and architecture dependencies are supplied by other translated units.

pub unsafe fn arch_zone_limits_init(max_zone_pfns: *mut core::ffi::c_ulong) {
    *max_zone_pfns.add(ZONE_DMA as usize) =
        PFN_DOWN(memblock_end_of_DRAM());
}

#[cfg(CONFIG_MMU)]
pub static mut m68k_virt_to_node_shift: core::ffi::c_int = 0;

#[cfg(CONFIG_MMU)]
pub unsafe fn m68k_setup_node(node: core::ffi::c_int) {
    node_set_online(node);
}

#[cfg(not(CONFIG_MMU))]
/*
 * paging_init() continues the virtual memory environment setup which
 * was begun by the code in arch/head.S.
 * The parameters are pointers to where to stick the starting and ending
 * addresses of available kernel virtual memory.
 */
pub unsafe fn paging_init() {
    /*
     * Make sure start_mem is page aligned, otherwise bootmem and
     * page_alloc get different views of the world.
     */
    let end_mem: core::ffi::c_ulong = memory_end & PAGE_MASK;

    high_memory = end_mem as *mut core::ffi::c_void;
}

pub unsafe fn free_initmem() {
    #[cfg(not(CONFIG_MMU_SUN3))]
    free_initmem_default(-1);
}

#[cfg(all(CONFIG_MMU, not(CONFIG_COLDFIRE)))]
const VECTORS: *mut core::ffi::c_void = unsafe { &mut vectors[0] as *mut _ as *mut core::ffi::c_void };

#[cfg(any(not(CONFIG_MMU), CONFIG_COLDFIRE))]
const VECTORS: *mut core::ffi::c_void = unsafe { _ramvec as *mut core::ffi::c_void };

unsafe fn init_pointer_tables() {
    #[cfg(all(CONFIG_MMU, not(CONFIG_SUN3), not(CONFIG_COLDFIRE)))]
    {
        let mut i: core::ffi::c_int;
        let mut j: core::ffi::c_int;

        /* insert pointer tables allocated so far into the tablelist */
        init_pointer_table(kernel_pg_dir, TABLE_PGD);
        i = 0;
        while i < PTRS_PER_PGD {
            let pud: *mut pud_t = &mut (*kernel_pg_dir.add(i as usize)) as *mut _ as *mut pud_t;
            let pmd_dir: *mut pmd_t;

            if !pud_present(*pud) {
                i += 1;
                continue;
            }

            pmd_dir = pgd_page_vaddr(*kernel_pg_dir.add(i as usize)) as *mut pmd_t;
            init_pointer_table(pmd_dir, TABLE_PMD);

            j = 0;
            while j < PTRS_PER_PMD {
                let pmd: *mut pmd_t = pmd_dir.add(j as usize);
                let pte_dir: *mut pte_t;

                if !pmd_present(*pmd) {
                    j += 1;
                    continue;
                }

                pte_dir = pmd_page_vaddr(*pmd) as *mut pte_t;
                init_pointer_table(pte_dir, TABLE_PTE);
                j += 1;
            }
            i += 1;
        }
    }
}

pub unsafe fn mem_init() {
    init_pointer_tables();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
