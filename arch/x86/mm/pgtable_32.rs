// SPDX-License-Identifier: GPL-2.0

// Kernel dependencies supplied by other translation units.
use core::ffi::{c_char, c_int};

pub static mut __VMALLOC_RESERVE: u32 = 128u32 << 20;

/*
 * Associate a virtual page frame with a given physical page frame
 * and protection flags for that frame.
 */
pub unsafe fn set_pte_vaddr(vaddr: usize, pteval: pte_t) {
    let mut pgd: *mut pgd_t;
    let mut p4d: *mut p4d_t;
    let mut pud: *mut pud_t;
    let mut pmd: *mut pmd_t;
    let mut pte: *mut pte_t;

    pgd = swapper_pg_dir.add(pgd_index(vaddr));
    if pgd_none(*pgd) {
        BUG();
        return;
    }
    p4d = p4d_offset(pgd, vaddr);
    if p4d_none(*p4d) {
        BUG();
        return;
    }
    pud = pud_offset(p4d, vaddr);
    if pud_none(*pud) {
        BUG();
        return;
    }
    pmd = pmd_offset(pud, vaddr);
    if pmd_none(*pmd) {
        BUG();
        return;
    }
    pte = pte_offset_kernel(pmd, vaddr);
    if !pte_none(pteval) {
        set_pte_at(&raw mut init_mm, vaddr, pte, pteval);
    } else {
        pte_clear(&raw mut init_mm, vaddr, pte);
    }

    /*
     * It's enough to flush this one mapping.
     * (PGE mappings get flushed as well)
     */
    flush_tlb_one_kernel(vaddr);
}

pub static mut __FIXADDR_TOP: usize = 0xfffff000;
// EXPORT_SYMBOL(__FIXADDR_TOP);

/*
 * vmalloc=size forces the vmalloc area to be exactly 'size'
 * bytes. This can be used to increase (or decrease) the
 * vmalloc area - the default is 128m.
 */
unsafe fn parse_vmalloc(arg: *mut c_char) -> c_int {
    if arg.is_null() {
        return -EINVAL;
    }

    /* Add VMALLOC_OFFSET to the parsed value due to vm area guard hole */
    __VMALLOC_RESERVE = memparse(arg, &mut (arg as *mut c_char)) as u32 + VMALLOC_OFFSET;
    0
}
// early_param("vmalloc", parse_vmalloc);

/*
 * reservetop=size reserves a hole at the top of the kernel address space which
 * a hypervisor can load into later.  Needed for dynamically loaded hypervisors,
 * so relocating the fixmap can be done before paging initialization.
 */
unsafe fn parse_reservetop(arg: *mut c_char) -> c_int {
    let mut address: usize;

    if arg.is_null() {
        return -EINVAL;
    }

    address = memparse(arg, &mut (arg as *mut c_char));
    reserve_top_address(address);
    early_ioremap_init();
    0
}
// early_param("reservetop", parse_reservetop);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
