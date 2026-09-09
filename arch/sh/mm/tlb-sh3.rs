// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/sh/mm/tlb-sh3.c
 *
 * SH-3 specific TLB operations
 *
 * Copyright (C) 1999  Niibe Yutaka
 * Copyright (C) 2002  Paul Mundt
 */

// Linux and SH architecture dependencies supplied by other translated files.

#[repr(C)]
pub struct vm_area_struct {
    pub vm_mm: *mut mm_struct,
}

#[repr(C)]
pub struct mm_struct;

#[repr(C)]
pub struct task_struct {
    pub active_mm: *mut mm_struct,
}

#[repr(C)]
pub struct cpuinfo_sh {
    pub flags: libc::c_ulong,
}

#[repr(C)]
pub struct pte_t {
    pub pte: libc::c_ulong,
}

unsafe extern "C" {
    pub static mut current: *mut task_struct;
    pub static mut current_cpu_data: cpuinfo_sh;

    pub fn get_asid() -> libc::c_ulong;
    pub fn pte_val(pte: pte_t) -> libc::c_ulong;
    pub fn local_irq_save(flags: *mut libc::c_ulong);
    pub fn local_irq_restore(flags: libc::c_ulong);
    pub fn ctrl_barrier();
    pub fn __raw_readl(addr: libc::c_ulong) -> libc::c_ulong;
    pub fn __raw_writel(value: libc::c_ulong, addr: libc::c_ulong);
}

// Constants supplied by the MMU and SH architecture headers.
extern "C" {
    pub static MMU_VPN_MASK: libc::c_ulong;
    pub static MMU_PTEH: libc::c_ulong;
    pub static MMU_PTEL: libc::c_ulong;
    pub static _PAGE_FLAGS_HARDWARE_MASK: libc::c_ulong;
    pub static MMU_NTLB_WAYS: libc::c_int;
    pub static MMU_TLB_ADDRESS_ARRAY: libc::c_ulong;
    pub static MMU_PAGE_ASSOC_BIT: libc::c_ulong;
    pub static MMUCR: libc::c_ulong;
    pub static CPU_HAS_MMU_PAGE_ASSOC: libc::c_ulong;
}

pub unsafe fn __update_tlb(
    vma: *mut vm_area_struct,
    address: libc::c_ulong,
    pte: pte_t,
) {
    let mut flags: libc::c_ulong = 0;
    let mut pteval: libc::c_ulong;
    let mut vpn: libc::c_ulong;

    /*
     * Handle debugger faulting in for debugee.
     */
    if !vma.is_null() && (*current).active_mm != (*vma).vm_mm {
        return;
    }

    local_irq_save(&mut flags);

    /* Set PTEH register */
    vpn = (address & MMU_VPN_MASK) | get_asid();
    __raw_writel(vpn, MMU_PTEH);

    pteval = pte_val(pte);

    /* Set PTEL register */
    pteval &= _PAGE_FLAGS_HARDWARE_MASK; /* drop software flags */
    /* conveniently, we want all the software flags to be 0 anyway */
    __raw_writel(pteval, MMU_PTEL);

    /* Load the TLB */
    core::arch::asm!("ldtlb", options(nostack, preserves_flags));
    local_irq_restore(flags);
}

pub unsafe fn local_flush_tlb_one(asid: libc::c_ulong, page: libc::c_ulong) {
    let mut addr: libc::c_ulong;
    let data: libc::c_ulong;
    let mut ways: libc::c_int = MMU_NTLB_WAYS;

    /*
     * NOTE: PTEH.ASID should be set to this MM
     *       _AND_ we need to write ASID to the array.
     *
     * It would be simple if we didn't need to set PTEH.ASID...
     */
    addr = MMU_TLB_ADDRESS_ARRAY | (page & 0x1f000);
    data = (page & 0xfffe0000) | asid; /* VALID bit is off */

    if (current_cpu_data.flags & CPU_HAS_MMU_PAGE_ASSOC != 0) {
        addr |= MMU_PAGE_ASSOC_BIT;
        ways = 1; /* we already know the way .. */
    }

    let mut i: libc::c_int = 0;
    while i < ways {
        __raw_writel(data, addr + ((i as libc::c_ulong) << 8));
        i += 1;
    }
}

pub unsafe fn local_flush_tlb_all() {
    let mut flags: libc::c_ulong = 0;
    let mut status: libc::c_ulong;

    /*
     * Flush all the TLB.
     *
     * Write to the MMU control register's bit:
     *\tTF-bit for SH-3, TI-bit for SH-4.
     *      It's same position, bit #2.
     */
    local_irq_save(&mut flags);
    status = __raw_readl(MMUCR);
    status |= 0x04;
    __raw_writel(status, MMUCR);
    ctrl_barrier();
    local_irq_restore(flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
