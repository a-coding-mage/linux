/* SPDX-License-Identifier: GPL-2.0 */
/*
 *    Copyright IBM Corp. 2017
 *    Author(s): Claudio Imbrenda <imbrenda@linux.vnet.ibm.com>
 */

// Dependency supplied by asm/page.h: __pa, PAGE_MASK, and PAGE_SIZE.

pub const ESSA_GET_STATE: u8 = 0;
pub const ESSA_SET_STABLE: u8 = 1;
pub const ESSA_SET_UNUSED: u8 = 2;
pub const ESSA_SET_VOLATILE: u8 = 3;
pub const ESSA_SET_POT_VOLATILE: u8 = 4;
pub const ESSA_SET_STABLE_RESIDENT: u8 = 5;
pub const ESSA_SET_STABLE_IF_RESIDENT: u8 = 6;
pub const ESSA_SET_STABLE_NODAT: u8 = 7;

pub const ESSA_MAX: u8 = ESSA_SET_STABLE_NODAT;

extern "C" {
    pub static mut cmma_flag: core::ffi::c_int;
}

#[inline(always)]
pub unsafe fn essa(paddr: usize, cmd: u8) -> usize {
    let rc: usize;
    core::arch::asm!(
        ".insn rrf,0xb9ab0000,{rc},{paddr},{cmd},0",
        rc = lateout(reg) rc,
        paddr = in(reg) paddr,
        cmd = const cmd,
    );
    rc
}

#[inline(always)]
pub unsafe fn __set_page_state(addr: *mut core::ffi::c_void, mut num_pages: usize, cmd: u8) {
    let mut paddr = (__pa(addr) & PAGE_MASK) as usize;

    while num_pages != 0 {
        num_pages = num_pages.wrapping_sub(1);
        essa(paddr, cmd);
        paddr = paddr.wrapping_add(PAGE_SIZE as usize);
    }
}

#[inline]
pub unsafe fn __set_page_unused(addr: *mut core::ffi::c_void, num_pages: usize) {
    __set_page_state(addr, num_pages, ESSA_SET_UNUSED);
}

#[inline]
pub unsafe fn __set_page_stable_dat(addr: *mut core::ffi::c_void, num_pages: usize) {
    __set_page_state(addr, num_pages, ESSA_SET_STABLE);
}

#[inline]
pub unsafe fn __set_page_stable_nodat(addr: *mut core::ffi::c_void, num_pages: usize) {
    __set_page_state(addr, num_pages, ESSA_SET_STABLE_NODAT);
}

#[inline]
pub unsafe fn __arch_set_page_nodat(addr: *mut core::ffi::c_void, num_pages: usize) {
    if cmma_flag == 0 {
        return;
    }
    if cmma_flag < 2 {
        __set_page_stable_dat(addr, num_pages);
    } else {
        __set_page_stable_nodat(addr, num_pages);
    }
}

#[inline]
pub unsafe fn __arch_set_page_dat(addr: *mut core::ffi::c_void, num_pages: usize) {
    if cmma_flag == 0 {
        return;
    }
    __set_page_stable_dat(addr, num_pages);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
