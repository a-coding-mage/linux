/*
 * TLB miss handler for SH with an MMU.
 *
 *  Copyright (C) 1999  Niibe Yutaka
 *  Copyright (C) 2003 - 2012  Paul Mundt
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the surrounding kernel translation unit.
use core::ffi::c_void;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pgd_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct p4d_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pud_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pmd_t {
    _private: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pte_t {
    _private: [u8; 0],
}

extern "C" {
    static mut current: *mut c_void;

    fn pgd_offset_k(address: usize) -> *mut pgd_t;
    fn pgd_offset(mm: *mut mm_struct, address: usize) -> *mut pgd_t;
    fn p4d_offset(pgd: *mut pgd_t, address: usize) -> *mut p4d_t;
    fn pud_offset(p4d: *mut p4d_t, address: usize) -> *mut pud_t;
    fn pmd_offset(pud: *mut pud_t, address: usize) -> *mut pmd_t;
    fn pte_offset_kernel(pmd: *mut pmd_t, address: usize) -> *mut pte_t;
    fn p4d_none_or_clear_bad(p4d: *mut p4d_t) -> bool;
    fn pud_none_or_clear_bad(pud: *mut pud_t) -> bool;
    fn pmd_none_or_clear_bad(pmd: *mut pmd_t) -> bool;
    fn pte_none(entry: pte_t) -> bool;
    fn pte_not_present(entry: pte_t) -> bool;
    fn pte_write(entry: pte_t) -> bool;
    fn pte_mkdirty(entry: pte_t) -> pte_t;
    fn pte_mkyoung(entry: pte_t) -> pte_t;
    fn set_pte(pte: *mut pte_t, entry: pte_t);
    fn set_thread_fault_code(error_code: usize);
    fn update_mmu_cache(vma: *mut c_void, address: usize, pte: *mut pte_t);
    fn local_flush_tlb_one(asid: usize, address: usize);
    fn get_asid() -> usize;
}

// Architecture/build configuration constants supplied externally.
extern "C" {
    static P3SEG: usize;
    static P3_ADDR_MAX: usize;
    static TASK_SIZE: usize;
    static PAGE_MASK: usize;
    static FAULT_CODE_INITIAL: usize;
}

/*
 * Called with interrupts disabled.
 */
#[no_mangle]
pub unsafe extern "C" fn handle_tlbmiss(
    regs: *mut pt_regs,
    error_code: usize,
    address: usize,
) -> i32 {
    let _ = regs;
    let pgd: *mut pgd_t;
    let p4d: *mut p4d_t;
    let pud: *mut pud_t;
    let pmd: *mut pmd_t;
    let pte: *mut pte_t;
    let mut entry: pte_t;

    /*
     * We don't take page faults for P1, P2, and parts of P4, these
     * are always mapped, whether it be due to legacy behaviour in
     * 29-bit mode, or due to PMB configuration in 32-bit mode.
     */
    if address >= P3SEG && address < P3_ADDR_MAX {
        pgd = pgd_offset_k(address);
    } else {
        let mm = *(current as *mut *mut mm_struct).add(0);
        if address >= TASK_SIZE || mm.is_null() {
            return 1;
        }
        pgd = pgd_offset(mm, address);
    }

    p4d = p4d_offset(pgd, address);
    if p4d_none_or_clear_bad(p4d) {
        return 1;
    }
    pud = pud_offset(p4d, address);
    if pud_none_or_clear_bad(pud) {
        return 1;
    }
    pmd = pmd_offset(pud, address);
    if pmd_none_or_clear_bad(pmd) {
        return 1;
    }
    pte = pte_offset_kernel(pmd, address);
    entry = *pte;
    if pte_none(entry) || pte_not_present(entry) {
        return 1;
    }
    if error_code != 0 && !pte_write(entry) {
        return 1;
    }

    if error_code != 0 {
        entry = pte_mkdirty(entry);
    }
    entry = pte_mkyoung(entry);

    set_pte(pte, entry);

    // CONFIG_CPU_SH4 && !CONFIG_SMP
    if error_code == FAULT_CODE_INITIAL {
        local_flush_tlb_one(get_asid(), address & PAGE_MASK);
    }

    set_thread_fault_code(error_code);
    update_mmu_cache(core::ptr::null_mut(), address, pte);

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
