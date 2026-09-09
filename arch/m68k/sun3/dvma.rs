// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/m68k/sun3/dvma.c
 *
 * Written by Sam Creasey
 *
 * Sun3 IOMMU routines used for dvma accesses.
 *
 */

// Linux and architecture headers from the C translation unit supply the
// declarations and constants referenced below.

static mut ptelist: [u64; 120] = [0; 120];

extern "C" {
    fn virt_to_pfn(addr: *const core::ffi::c_void) -> u64;
    fn pfn_pte(pfn: u64, prot: u64) -> u64;
    fn pte_val(pte: u64) -> u64;
    fn sun3_put_pte(vaddr: u64, pte: u64);
    fn dvma_btov(baddr: u64) -> u64;
}

// PAGE_SHIFT, PAGE_SIZE, PAGE_MASK, and PAGE_KERNEL are supplied by the
// architecture headers corresponding to the original C source.
extern "C" {
    static PAGE_SHIFT: u32;
    static PAGE_SIZE: u64;
    static PAGE_MASK: u64;
    static PAGE_KERNEL: u64;
}

unsafe fn dvma_page(kaddr: u64, vaddr: u64) -> u64
{
    let pte: u64;
    let j: u64;
    let ptep: u64;

    j = core::ptr::read_volatile(kaddr as *const u64);
    core::ptr::write_volatile(kaddr as *mut u64, j);

    ptep = pfn_pte(virt_to_pfn(kaddr as *const core::ffi::c_void), PAGE_KERNEL);
    pte = pte_val(ptep);
    // pr_info("dvma_remap: addr %lx -> %lx pte %08lx\n", kaddr, vaddr, pte);
    let index = ((vaddr & 0xff000) >> PAGE_SHIFT) as usize;
    if ptelist[index] != pte {
        sun3_put_pte(vaddr, pte);
        ptelist[index] = pte;
    }

    vaddr + (kaddr & !PAGE_MASK)
}

pub unsafe fn dvma_map_iommu(mut kaddr: u64, baddr: u64, len: i32) -> i32
{
    let end: u64;
    let mut vaddr: u64;

    vaddr = dvma_btov(baddr);

    end = vaddr + len as u64;

    while vaddr < end {
        dvma_page(kaddr, vaddr);
        kaddr += PAGE_SIZE;
        vaddr += PAGE_SIZE;
    }

    0
}

pub unsafe fn sun3_dvma_init()
{
    core::ptr::write_bytes(ptelist.as_mut_ptr(), 0, ptelist.len());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
