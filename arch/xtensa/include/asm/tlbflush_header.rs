/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2013 Tensilica Inc.
 */

// Dependencies supplied by the surrounding kernel translation.

pub const DTLB_WAY_PGD: u32 = 7;
pub const ITLB_ARF_WAYS: u32 = 4;
pub const DTLB_ARF_WAYS: u32 = 4;
pub const ITLB_HIT_BIT: u32 = 3;
pub const DTLB_HIT_BIT: u32 = 4;

/* TLB flushing:
 *
 *  - flush_tlb_all() flushes all processes TLB entries
 *  - flush_tlb_mm(mm) flushes the specified mm context TLB entries
 *  - flush_tlb_page(vma, page) flushes a single page
 *  - flush_tlb_range(vma, vmaddr, end) flushes a range of pages
 */

unsafe extern "C" {
    pub fn local_flush_tlb_all();
    pub fn local_flush_tlb_mm(mm: *mut mm_struct);
    pub fn local_flush_tlb_page(vma: *mut vm_area_struct, page: c_ulong);
    pub fn local_flush_tlb_range(vma: *mut vm_area_struct, start: c_ulong, end: c_ulong);
    pub fn local_flush_tlb_kernel_range(start: c_ulong, end: c_ulong);

    // Under CONFIG_SMP these are external functions; otherwise the C macros
    // below resolve directly to the local variants.
    #[cfg(CONFIG_SMP)]
    pub fn flush_tlb_all();
    #[cfg(CONFIG_SMP)]
    pub fn flush_tlb_mm(mm: *mut mm_struct);
    #[cfg(CONFIG_SMP)]
    pub fn flush_tlb_page(vma: *mut vm_area_struct, page: c_ulong);
    #[cfg(CONFIG_SMP)]
    pub fn flush_tlb_range(vma: *mut vm_area_struct, start: c_ulong, end: c_ulong);
    #[cfg(CONFIG_SMP)]
    pub fn flush_tlb_kernel_range(start: c_ulong, end: c_ulong);
}

#[cfg(not(CONFIG_SMP))]
#[inline]
pub unsafe fn flush_tlb_all() { local_flush_tlb_all(); }
#[cfg(not(CONFIG_SMP))]
#[inline]
pub unsafe fn flush_tlb_mm(mm: *mut mm_struct) { local_flush_tlb_mm(mm); }
#[cfg(not(CONFIG_SMP))]
#[inline]
pub unsafe fn flush_tlb_page(vma: *mut vm_area_struct, page: c_ulong) { local_flush_tlb_page(vma, page); }
#[cfg(not(CONFIG_SMP))]
#[inline]
pub unsafe fn flush_tlb_range(vma: *mut vm_area_struct, start: c_ulong, end: c_ulong) { local_flush_tlb_range(vma, start, end); }
#[cfg(not(CONFIG_SMP))]
#[inline]
pub unsafe fn flush_tlb_kernel_range(start: c_ulong, end: c_ulong) { local_flush_tlb_kernel_range(start, end); }

/* TLB operations. */

#[inline]
pub unsafe fn itlb_probe(addr: c_ulong) -> c_ulong {
    let mut tmp: c_ulong;
    core::arch::asm!("pitlb {0}, {1}", out(reg) tmp, in(reg) addr);
    tmp
}

#[inline]
pub unsafe fn dtlb_probe(addr: c_ulong) -> c_ulong {
    let mut tmp: c_ulong;
    core::arch::asm!("pdtlb {0}, {1}", out(reg) tmp, in(reg) addr);
    tmp
}

#[inline]
pub unsafe fn invalidate_itlb_entry(probe: c_ulong) {
    core::arch::asm!("iitlb {0}; isync", in(reg) probe);
}

#[inline]
pub unsafe fn invalidate_dtlb_entry(probe: c_ulong) {
    core::arch::asm!("idtlb {0}; dsync", in(reg) probe);
}

/* Use the .._no_isync functions with caution. Generally, these are handy for
 * bulk invalidates followed by a single 'isync'. The caller must follow up
 * with an 'isync', which can be relatively expensive on some Xtensa
 * implementations.
 */
#[inline]
pub unsafe fn invalidate_itlb_entry_no_isync(entry: c_uint) {
    /* Caller must follow up with 'isync'. */
    core::arch::asm!("iitlb {0}", in(reg) entry);
}

#[inline]
pub unsafe fn invalidate_dtlb_entry_no_isync(entry: c_uint) {
    /* Caller must follow up with 'isync'. */
    core::arch::asm!("idtlb {0}", in(reg) entry);
}

#[inline]
pub unsafe fn set_itlbcfg_register(val: c_ulong) { core::arch::asm!("wsr {0}, itlbcfg; isync", in(reg) val); }
#[inline]
pub unsafe fn set_dtlbcfg_register(val: c_ulong) { core::arch::asm!("wsr {0}, dtlbcfg; dsync", in(reg) val); }
#[inline]
pub unsafe fn set_ptevaddr_register(val: c_ulong) { core::arch::asm!("wsr {0}, ptevaddr; isync", in(reg) val); }

#[inline]
pub unsafe fn read_ptevaddr_register() -> c_ulong {
    let mut tmp: c_ulong;
    core::arch::asm!("rsr {0}, ptevaddr", out(reg) tmp);
    tmp
}

#[inline]
pub unsafe fn write_dtlb_entry(entry: pte_t, way: c_int) { core::arch::asm!("wdtlb {1}, {0}; dsync", in(reg) entry, in(reg) way); }
#[inline]
pub unsafe fn write_itlb_entry(entry: pte_t, way: c_int) { core::arch::asm!("witlb {1}, {0}; isync", in(reg) entry, in(reg) way); }

#[inline]
pub unsafe fn invalidate_page_directory() {
    invalidate_dtlb_entry(DTLB_WAY_PGD as c_ulong);
    invalidate_dtlb_entry((DTLB_WAY_PGD + 1) as c_ulong);
    invalidate_dtlb_entry((DTLB_WAY_PGD + 2) as c_ulong);
}

#[inline]
pub unsafe fn invalidate_itlb_mapping(address: c_uint) {
    let tlb_entry = itlb_probe(address as c_ulong);
    if (tlb_entry & (1u64 << ITLB_HIT_BIT)) != 0 { invalidate_itlb_entry(tlb_entry); }
}

#[inline]
pub unsafe fn invalidate_dtlb_mapping(address: c_uint) {
    let tlb_entry = dtlb_probe(address as c_ulong);
    if (tlb_entry & (1u64 << DTLB_HIT_BIT)) != 0 { invalidate_dtlb_entry(tlb_entry); }
}

/* DO NOT USE THESE FUNCTIONS. These instructions aren't part of the Xtensa
 * ISA and exist only for test purposes. You may find it helpful for MMU
 * debugging, however.
 *
 * 'at' is the unmodified input register
 * 'as' is the output register, as follows (specific to the Linux config):
 *      as[31..12] contain the virtual address
 *      as[11..08] are meaningless
 *      as[07..00] contain the asid
 */

#[inline]
pub unsafe fn read_dtlb_virtual(way: c_int) -> c_ulong { let mut tmp: c_ulong; core::arch::asm!("rdtlb0 {0}, {1}", out(reg) tmp, inout(reg) way => _); tmp }
#[inline]
pub unsafe fn read_dtlb_translation(way: c_int) -> c_ulong { let mut tmp: c_ulong; core::arch::asm!("rdtlb1 {0}, {1}", out(reg) tmp, inout(reg) way => _); tmp }
#[inline]
pub unsafe fn read_itlb_virtual(way: c_int) -> c_ulong { let mut tmp: c_ulong; core::arch::asm!("ritlb0 {0}, {1}", out(reg) tmp, inout(reg) way => _); tmp }
#[inline]
pub unsafe fn read_itlb_translation(way: c_int) -> c_ulong { let mut tmp: c_ulong; core::arch::asm!("ritlb1 {0}, {1}", out(reg) tmp, inout(reg) way => _); tmp }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
