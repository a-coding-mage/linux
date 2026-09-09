/*
 * arch/sh/mm/tlb-urb.c
 *
 * TLB entry wiring helpers for URB-equipped parts.
 *
 * Copyright (C) 2010  Matt Fleming
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_ulong;

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pte_t {
    pub pte: c_ulong,
}

extern "C" {
    static mut MMUCR: c_ulong;
    static MMUCR_URB: c_ulong;
    static MMUCR_URB_SHIFT: u32;
    static MMUCR_URC: c_ulong;
    static MMUCR_URC_SHIFT: u32;
    static MMUCR_URB_NENTRIES: i32;

    fn local_irq_save(flags: *mut c_ulong);
    fn local_irq_restore(flags: c_ulong);
    fn __raw_readl(addr: c_ulong) -> c_ulong;
    fn __raw_writel(value: c_ulong, addr: c_ulong);
    fn ctrl_barrier();
    fn __update_tlb(vma: *mut vm_area_struct, addr: c_ulong, pte: pte_t);
    fn BUG_ON(condition: bool);
}

/*
 * Load the entry for 'addr' into the TLB and wire the entry.
 */
pub unsafe fn tlb_wire_entry(vma: *mut vm_area_struct, addr: c_ulong, pte: pte_t) {
    let mut status: c_ulong;
    let mut flags: c_ulong = 0;
    let mut urb: i32;

    local_irq_save(&mut flags);

    status = __raw_readl(MMUCR);
    urb = ((status & MMUCR_URB) >> MMUCR_URB_SHIFT) as i32;
    status &= !MMUCR_URC;

    /*
     * Make sure we're not trying to wire the last TLB entry slot.
     */
    urb -= 1;
    BUG_ON(urb == 0);

    urb %= MMUCR_URB_NENTRIES;

    /*
     * Insert this entry into the highest non-wired TLB slot (via
     * the URC field).
     */
    status |= (urb as c_ulong) << MMUCR_URC_SHIFT;
    __raw_writel(status, MMUCR);
    ctrl_barrier();

    /* Load the entry into the TLB */
    __update_tlb(vma, addr, pte);

    /* ... and wire it up. */
    status = __raw_readl(MMUCR);

    status &= !MMUCR_URB;
    status |= (urb as c_ulong) << MMUCR_URB_SHIFT;

    __raw_writel(status, MMUCR);
    ctrl_barrier();

    local_irq_restore(flags);
}

/*
 * Unwire the last wired TLB entry.
 *
 * It should also be noted that it is not possible to wire and unwire
 * TLB entries in an arbitrary order. If you wire TLB entry N, followed
 * by entry N+1, you must unwire entry N+1 first, then entry N. In this
 * respect, it works like a stack or LIFO queue.
 */
pub unsafe fn tlb_unwire_entry() {
    let mut status: c_ulong;
    let mut flags: c_ulong = 0;
    let mut urb: i32;

    local_irq_save(&mut flags);

    status = __raw_readl(MMUCR);
    urb = ((status & MMUCR_URB) >> MMUCR_URB_SHIFT) as i32;
    status &= !MMUCR_URB;

    /*
     * Make sure we're not trying to unwire a TLB entry when none
     * have been wired.
     */
    BUG_ON(urb == MMUCR_URB_NENTRIES);
    urb += 1;

    urb %= MMUCR_URB_NENTRIES;

    status |= (urb as c_ulong) << MMUCR_URB_SHIFT;
    __raw_writel(status, MMUCR);
    ctrl_barrier();

    local_irq_restore(flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
