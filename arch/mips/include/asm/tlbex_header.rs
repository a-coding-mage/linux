/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding translation unit:
// #include <asm/uasm.h>

/*
 * Write random or indexed TLB entry, and care about the hazards from
 * the preceding mtc0 and for the following eret.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum tlb_write_entry {
    tlb_random = 0,
    tlb_indexed = 1,
}

unsafe extern "C" {
    pub static mut pgd_reg: core::ffi::c_int;

    pub fn build_get_pmde64(
        p: *mut *mut u32,
        l: *mut *mut uasm_label,
        r: *mut *mut uasm_reloc,
        tmp: u32,
        ptr: u32,
    );
    pub fn build_get_pgde32(p: *mut *mut u32, tmp: u32, ptr: u32);
    pub fn build_get_ptep(p: *mut *mut u32, tmp: u32, ptr: u32);
    pub fn build_update_entries(p: *mut *mut u32, tmp: u32, ptep: u32);
    pub fn build_tlb_write_entry(
        p: *mut *mut u32,
        l: *mut *mut uasm_label,
        r: *mut *mut uasm_reloc,
        wmode: tlb_write_entry,
    );
    pub fn build_tlb_refill_handler();

    pub fn handle_tlbl();
    pub static handle_tlbl_end: [core::ffi::c_char; 0];

    pub fn handle_tlbs();
    pub static handle_tlbs_end: [core::ffi::c_char; 0];

    pub fn handle_tlbm();
    pub static handle_tlbm_end: [core::ffi::c_char; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
