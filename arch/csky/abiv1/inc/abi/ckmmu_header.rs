/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding translation unit:
// #include <abi/reg_ops.h>

use core::ffi::{c_char, c_int, c_void};

unsafe extern "C" {
    fn cprcr(reg: *const c_char) -> c_int;
    fn cpwcr(reg: *const c_char, value: c_int);
    fn __pa(addr: *const c_void) -> usize;
}

// Dependency type supplied by the surrounding translation unit.
#[allow(non_camel_case_types)]
pub type pgd_t = crate::pgd_t;

#[inline]
pub const fn BIT(bit: usize) -> usize {
    1usize << bit
}

#[inline]
pub unsafe fn read_mmu_index() -> i32 {
    unsafe { cprcr(b"cpcr0\0".as_ptr() as *const c_char) as i32 }
}

#[inline]
pub unsafe fn write_mmu_index(value: i32) {
    unsafe { cpwcr(b"cpcr0\0".as_ptr() as *const c_char, value as c_int) }
}

#[inline]
pub unsafe fn read_mmu_entrylo0() -> i32 {
    unsafe { (cprcr(b"cpcr2\0".as_ptr() as *const c_char) << 6) as i32 }
}

#[inline]
pub unsafe fn read_mmu_entrylo1() -> i32 {
    unsafe { (cprcr(b"cpcr3\0".as_ptr() as *const c_char) << 6) as i32 }
}

#[inline]
pub unsafe fn write_mmu_pagemask(value: i32) {
    unsafe { cpwcr(b"cpcr6\0".as_ptr() as *const c_char, value as c_int) }
}

#[inline]
pub unsafe fn read_mmu_entryhi() -> i32 {
    unsafe { cprcr(b"cpcr4\0".as_ptr() as *const c_char) as i32 }
}

#[inline]
pub unsafe fn write_mmu_entryhi(value: i32) {
    unsafe { cpwcr(b"cpcr4\0".as_ptr() as *const c_char, value as c_int) }
}

#[inline]
pub unsafe fn read_mmu_msa0() -> usize {
    unsafe { cprcr(b"cpcr30\0".as_ptr() as *const c_char) as usize }
}

#[inline]
pub unsafe fn write_mmu_msa0(value: usize) {
    unsafe { cpwcr(b"cpcr30\0".as_ptr() as *const c_char, value as c_int) }
}

#[inline]
pub unsafe fn read_mmu_msa1() -> usize {
    unsafe { cprcr(b"cpcr31\0".as_ptr() as *const c_char) as usize }
}

#[inline]
pub unsafe fn write_mmu_msa1(value: usize) {
    unsafe { cpwcr(b"cpcr31\0".as_ptr() as *const c_char, value as c_int) }
}

/*
 * TLB operations.
 */
#[inline]
pub unsafe fn tlb_probe() {
    unsafe { cpwcr(b"cpcr8\0".as_ptr() as *const c_char, 0x80000000u32 as c_int) }
}

#[inline]
pub unsafe fn tlb_read() {
    unsafe { cpwcr(b"cpcr8\0".as_ptr() as *const c_char, 0x40000000u32 as c_int) }
}

#[inline]
pub unsafe fn tlb_invalid_all() {
    unsafe { cpwcr(b"cpcr8\0".as_ptr() as *const c_char, 0x04000000) }
}

#[inline]
pub unsafe fn local_tlb_invalid_all() {
    unsafe { tlb_invalid_all() }
}

#[inline]
pub unsafe fn tlb_invalid_indexed() {
    unsafe { cpwcr(b"cpcr8\0".as_ptr() as *const c_char, 0x02000000) }
}

#[inline]
pub unsafe fn setup_pgd(pgd: *mut pgd_t, asid: i32) {
    unsafe {
        cpwcr(
            b"cpcr29\0".as_ptr() as *const c_char,
            (__pa(pgd as *const c_void) | BIT(0)) as c_int,
        );
        write_mmu_entryhi(asid);
    }
}

#[inline]
pub unsafe fn get_pgd() -> *mut pgd_t {
    unsafe {
        ((cprcr(b"cpcr29\0".as_ptr() as *const c_char) as usize & !BIT(0)) as *mut pgd_t)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
