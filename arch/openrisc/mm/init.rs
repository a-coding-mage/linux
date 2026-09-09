// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OpenRISC idle.c
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * Modifications for the OpenRISC architecture:
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 */

// Linux and architecture headers are supplied by other translation units.

pub static mut mem_init_done: i32 = 0;

pub unsafe fn arch_zone_limits_init(max_zone_pfns: *mut c_ulong) {
    /* We use only ZONE_NORMAL */
    *max_zone_pfns.add(ZONE_NORMAL as usize) = max_low_pfn;
}

extern "C" {
    static _s_kernel_ro: c_char;
    static _e_kernel_ro: c_char;
}

/*
 * Map all physical memory into kernel's address space.
 *
 * This is explicitly coded for two-level page tables, so if you need
 * something else then this needs to change.
 */
unsafe fn map_ram() {
    let mut start: phys_addr_t;
    let mut end: phys_addr_t;
    let mut v: c_ulong;
    let mut p: c_ulong;
    let mut e: c_ulong;
    let mut prot: pgprot_t;
    let mut pge: *mut pgd_t;
    let mut p4e: *mut p4d_t;
    let mut pue: *mut pud_t;
    let mut pme: *mut pmd_t;
    let mut pte: *mut pte_t;
    let mut i: u64 = 0;

    /* These mark extents of read-only kernel pages...
     * ...from vmlinux.lds.S
     */
    v = PAGE_OFFSET;

    for_each_mem_range!(i, &mut start, &mut end);
    {
        p = (start as u32 as c_ulong) & PAGE_MASK;
        e = end as u32 as c_ulong;
        v = __va(p);
        pge = pgd_offset_k(v);

        while p < e {
            let mut j: i32 = 0;
            p4e = p4d_offset(pge, v);
            pue = pud_offset(p4e, v);
            pme = pmd_offset(pue, v);

            if (pue as u32) != (pge as u32) || (pme as u32) != (pge as u32) {
                panic!("{}: OR1K kernel hardcoded for two-level page tables", "map_ram");
            }

            /* Alloc one page for holding PTE's... */
            pte = memblock_alloc_raw(PAGE_SIZE, PAGE_SIZE) as *mut pte_t;
            if pte.is_null() {
                panic!("{}: Failed to allocate page for PTEs\n", "map_ram");
            }
            set_pmd(pme, __pmd(_KERNPG_TABLE + __pa(pte)));

            /* Fill the newly allocated page with PTE'S */
            while p < e && j < PTRS_PER_PTE {
                if v >= (&_e_kernel_ro as *const c_char as u32 as c_ulong)
                    || v < (&_s_kernel_ro as *const c_char as u32 as c_ulong)
                {
                    prot = PAGE_KERNEL;
                } else {
                    prot = PAGE_KERNEL_RO;
                }
                set_pte(pte, mk_pte_phys(p, prot));
                v = v.wrapping_add(PAGE_SIZE);
                p = p.wrapping_add(PAGE_SIZE);
                j += 1;
                pte = pte.add(1);
            }
            pge = pge.add(1);
        }
        printk!(KERN_INFO, "{}: Memory: 0x{:x}-0x{:x}\n", "map_ram", start, end);
    }
}

pub unsafe fn paging_init() {
    printk!(KERN_INFO, "Setting up paging and PTEs.\n");

    /* clear out the init_mm.pgd that will contain the kernel's mappings */
    for i in 0..PTRS_PER_PGD {
        swapper_pg_dir[i as usize] = __pgd(0);
    }

    /* make sure the current pgd table points to something sane
     * (even if it is most probably not used until the next
     * switch_mm)
     */
    current_pgd[smp_processor_id() as usize] = init_mm.pgd;

    map_ram();

    /* self modifying code ;) */
    /* Since the old TLB miss handler has been running up until now,
     * the kernel pages are still all RW, so we can still modify the
     * text directly... after this change and a TLB flush, the kernel
     * pages will become RO.
     */
    {
        extern "C" {
            static mut dtlb_miss_handler: c_ulong;
            static mut itlb_miss_handler: c_ulong;
        }
        let dtlb_vector = __va(0x900) as *mut c_ulong;
        let itlb_vector = __va(0xa00) as *mut c_ulong;

        printk!(KERN_INFO, "itlb_miss_handler %p\n", &itlb_miss_handler);
        *itlb_vector = ((&itlb_miss_handler as *const _ as c_ulong)
            .wrapping_sub(itlb_vector as c_ulong)) >> 2;
        barrier!();
        printk!(KERN_INFO, "dtlb_miss_handler %p\n", &dtlb_miss_handler);
        *dtlb_vector = ((&dtlb_miss_handler as *const _ as c_ulong)
            .wrapping_sub(dtlb_vector as c_ulong)) >> 2;
    }

    barrier!();
    local_icache_block_inv(0x900);
    local_icache_block_inv(0xa00);
    flush_tlb_all();
}

/* References to section boundaries */
pub unsafe fn mem_init() {
    BUG_ON!(!mem_map.is_null());
    printk!("mem_init_done ...........................................\n");
    mem_init_done = 1;
    return;
}

unsafe fn map_page(va: c_ulong, pa: phys_addr_t, prot: pgprot_t) -> i32 {
    let p4d = p4d_offset(pgd_offset_k(va), va);
    let pud = pud_offset(p4d, va);
    let pmd = pmd_offset(pud, va);
    let pte = pte_alloc_kernel(pmd, va);
    if pte.is_null() { return -ENOMEM; }
    if pgprot_val(prot) != 0 {
        set_pte_at(&mut init_mm, va, pte, pfn_pte(pa >> PAGE_SHIFT, prot));
    } else {
        pte_clear(&mut init_mm, va, pte);
    }
    local_flush_tlb_page(core::ptr::null_mut(), va);
    0
}

/*
 * __set_fix must now support both EARLYCON and TEXT_POKE mappings,
 * which are used at different stages of kernel execution.
 */
pub unsafe fn __set_fixmap(idx: enum_fixed_addresses, phys: phys_addr_t, prot: pgprot_t) {
    let address = __fix_to_virt(idx);
    if idx as u32 >= __end_of_fixed_addresses {
        BUG!();
        return;
    }
    map_page(address, phys, prot);
}

static protection_map: [pgprot_t; 16] = [
    PAGE_NONE, PAGE_READONLY_X, PAGE_COPY, PAGE_COPY_X,
    PAGE_READONLY, PAGE_READONLY_X, PAGE_COPY, PAGE_COPY_X,
    PAGE_NONE, PAGE_READONLY_X, PAGE_SHARED, PAGE_SHARED_X,
    PAGE_READONLY, PAGE_READONLY_X, PAGE_SHARED, PAGE_SHARED_X,
];

DECLARE_VM_GET_PAGE_PROT!();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
