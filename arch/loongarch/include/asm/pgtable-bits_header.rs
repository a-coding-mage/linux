/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

/* Page table bits */

#[cfg(feature = "CONFIG_32BIT")]
pub const _PAGE_VALID_SHIFT: u32 = 0;
#[cfg(feature = "CONFIG_32BIT")]
pub const _PAGE_ACCESSED_SHIFT: u32 = 0; /* Reuse Valid for Accessed */
#[cfg(feature = "CONFIG_32BIT")]
pub const _PAGE_DIRTY_SHIFT: u32 = 1;
#[cfg(feature = "CONFIG_32BIT")]
pub const _PAGE_PLV_SHIFT: u32 = 2; /* 2~3, two bits */
#[cfg(feature = "CONFIG_32BIT")]
pub const _CACHE_SHIFT: u32 = 4; /* 4~5, two bits */
#[cfg(feature = "CONFIG_32BIT")]
pub const _PAGE_GLOBAL_SHIFT: u32 = 6;
#[cfg(feature = "CONFIG_32BIT")]
pub const _PAGE_HUGE_SHIFT: u32 = 6; /* HUGE is a PMD bit */
#[cfg(feature = "CONFIG_32BIT")]
pub const _PAGE_PRESENT_SHIFT: u32 = 7;
#[cfg(feature = "CONFIG_32BIT")]
pub const _PAGE_PFN_SHIFT: u32 = 8;
#[cfg(feature = "CONFIG_32BIT")]
pub const _PAGE_HGLOBAL_SHIFT: u32 = 12; /* HGlobal is a PMD bit */
#[cfg(feature = "CONFIG_32BIT")]
pub const _PAGE_SWP_EXCLUSIVE_SHIFT: u32 = 13;
#[cfg(feature = "CONFIG_32BIT")]
pub const _PAGE_PFN_END_SHIFT: u32 = 28;
#[cfg(feature = "CONFIG_32BIT")]
pub const _PAGE_WRITE_SHIFT: u32 = 29;
#[cfg(feature = "CONFIG_32BIT")]
pub const _PAGE_MODIFIED_SHIFT: u32 = 30;
#[cfg(feature = "CONFIG_32BIT")]
pub const _PAGE_PRESENT_INVALID_SHIFT: u32 = 31;

#[cfg(feature = "CONFIG_64BIT")]
pub const _PAGE_VALID_SHIFT: u32 = 0;
#[cfg(feature = "CONFIG_64BIT")]
pub const _PAGE_ACCESSED_SHIFT: u32 = 0; /* Reuse Valid for Accessed */
#[cfg(feature = "CONFIG_64BIT")]
pub const _PAGE_DIRTY_SHIFT: u32 = 1;
#[cfg(feature = "CONFIG_64BIT")]
pub const _PAGE_PLV_SHIFT: u32 = 2; /* 2~3, two bits */
#[cfg(feature = "CONFIG_64BIT")]
pub const _CACHE_SHIFT: u32 = 4; /* 4~5, two bits */
#[cfg(feature = "CONFIG_64BIT")]
pub const _PAGE_GLOBAL_SHIFT: u32 = 6;
#[cfg(feature = "CONFIG_64BIT")]
pub const _PAGE_HUGE_SHIFT: u32 = 6; /* HUGE is a PMD bit */
#[cfg(feature = "CONFIG_64BIT")]
pub const _PAGE_PRESENT_SHIFT: u32 = 7;
#[cfg(feature = "CONFIG_64BIT")]
pub const _PAGE_WRITE_SHIFT: u32 = 8;
#[cfg(feature = "CONFIG_64BIT")]
pub const _PAGE_MODIFIED_SHIFT: u32 = 9;
#[cfg(feature = "CONFIG_64BIT")]
pub const _PAGE_PROTNONE_SHIFT: u32 = 10;
#[cfg(feature = "CONFIG_64BIT")]
pub const _PAGE_SPECIAL_SHIFT: u32 = 11;
#[cfg(feature = "CONFIG_64BIT")]
pub const _PAGE_PFN_SHIFT: u32 = 12;
#[cfg(feature = "CONFIG_64BIT")]
pub const _PAGE_HGLOBAL_SHIFT: u32 = 12; /* HGlobal is a PMD bit */
#[cfg(feature = "CONFIG_64BIT")]
pub const _PAGE_SWP_EXCLUSIVE_SHIFT: u32 = 23;
#[cfg(feature = "CONFIG_64BIT")]
pub const _PAGE_PFN_END_SHIFT: u32 = 48;
#[cfg(feature = "CONFIG_64BIT")]
pub const _PAGE_PRESENT_INVALID_SHIFT: u32 = 60;
#[cfg(feature = "CONFIG_64BIT")]
pub const _PAGE_NO_READ_SHIFT: u32 = 61;
#[cfg(feature = "CONFIG_64BIT")]
pub const _PAGE_NO_EXEC_SHIFT: u32 = 62;
#[cfg(feature = "CONFIG_64BIT")]
pub const _PAGE_RPLV_SHIFT: u32 = 63;

/* Used by software */
macro_rules! _PAGE_PRESENT { () => { (1usize << _PAGE_PRESENT_SHIFT) }; }
macro_rules! _PAGE_PRESENT_INVALID { () => { (1usize << _PAGE_PRESENT_INVALID_SHIFT) }; }
macro_rules! _PAGE_WRITE { () => { (1usize << _PAGE_WRITE_SHIFT) }; }
macro_rules! _PAGE_ACCESSED { () => { (1usize << _PAGE_ACCESSED_SHIFT) }; }
macro_rules! _PAGE_MODIFIED { () => { (1usize << _PAGE_MODIFIED_SHIFT) }; }
#[cfg(feature = "CONFIG_32BIT")]
macro_rules! _PAGE_PROTNONE { () => { 0usize }; }
#[cfg(feature = "CONFIG_32BIT")]
macro_rules! _PAGE_SPECIAL { () => { 0usize }; }
#[cfg(feature = "CONFIG_64BIT")]
macro_rules! _PAGE_PROTNONE { () => { (1usize << _PAGE_PROTNONE_SHIFT) }; }
#[cfg(feature = "CONFIG_64BIT")]
macro_rules! _PAGE_SPECIAL { () => { (1usize << _PAGE_SPECIAL_SHIFT) }; }

/* We borrow bit 13/23 to store the exclusive marker in swap PTEs. */
macro_rules! _PAGE_SWP_EXCLUSIVE { () => { (1usize << _PAGE_SWP_EXCLUSIVE_SHIFT) }; }

/* Used by TLB hardware (placed in EntryLo*) */
macro_rules! _PAGE_VALID { () => { (1usize << _PAGE_VALID_SHIFT) }; }
macro_rules! _PAGE_DIRTY { () => { (1usize << _PAGE_DIRTY_SHIFT) }; }
macro_rules! _PAGE_PLV { () => { (3usize << _PAGE_PLV_SHIFT) }; }
macro_rules! _PAGE_GLOBAL { () => { (1usize << _PAGE_GLOBAL_SHIFT) }; }
macro_rules! _PAGE_HUGE { () => { (1usize << _PAGE_HUGE_SHIFT) }; }
macro_rules! _PAGE_HGLOBAL { () => { (1usize << _PAGE_HGLOBAL_SHIFT) }; }
#[cfg(feature = "CONFIG_32BIT")]
macro_rules! _PAGE_NO_READ { () => { 0usize }; }
#[cfg(feature = "CONFIG_32BIT")]
macro_rules! _PAGE_NO_EXEC { () => { 0usize }; }
#[cfg(feature = "CONFIG_32BIT")]
macro_rules! _PAGE_RPLV { () => { 0usize }; }
#[cfg(feature = "CONFIG_64BIT")]
macro_rules! _PAGE_NO_READ { () => { (1usize << _PAGE_NO_READ_SHIFT) }; }
#[cfg(feature = "CONFIG_64BIT")]
macro_rules! _PAGE_NO_EXEC { () => { (1usize << _PAGE_NO_EXEC_SHIFT) }; }
#[cfg(feature = "CONFIG_64BIT")]
macro_rules! _PAGE_RPLV { () => { (1usize << _PAGE_RPLV_SHIFT) }; }
macro_rules! _CACHE_MASK { () => { (3usize << _CACHE_SHIFT) }; }
macro_rules! PFN_PTE_SHIFT { () => { (PAGE_SHIFT - 12 + _PAGE_PFN_SHIFT) }; }

macro_rules! _PAGE_USER { () => { (PLV_USER << _PAGE_PLV_SHIFT) }; }
macro_rules! _PAGE_KERN { () => { (PLV_KERN << _PAGE_PLV_SHIFT) }; }
macro_rules! _PFN_MASK { () => { ((!((1usize << PFN_PTE_SHIFT!()) - 1)) & ((1usize << _PAGE_PFN_END_SHIFT) - 1)) }; }

/*
 * Cache attributes
 */
const _CACHE_SUC: usize = 0 << _CACHE_SHIFT;
const _CACHE_CC: usize = 1 << _CACHE_SHIFT;
const _CACHE_WUC: usize = 2 << _CACHE_SHIFT;

macro_rules! __READABLE { () => { _PAGE_VALID!() }; }
macro_rules! __WRITEABLE { () => { (_PAGE_DIRTY!() | _PAGE_WRITE!()) }; }
macro_rules! _PAGE_CHG_MASK { () => { (_PAGE_MODIFIED!() | _PAGE_SPECIAL!() | _PFN_MASK!() | _CACHE_MASK!() | _PAGE_PLV!()) }; }
macro_rules! _HPAGE_CHG_MASK { () => { (_PAGE_MODIFIED!() | _PAGE_SPECIAL!() | _PFN_MASK!() | _CACHE_MASK!() | _PAGE_PLV!() | _PAGE_HUGE!()) }; }

macro_rules! PAGE_NONE { () => { __pgprot(_PAGE_PROTNONE!() | _PAGE_NO_READ!() | _PAGE_USER!() | _CACHE_CC) }; }
macro_rules! PAGE_SHARED { () => { __pgprot(_PAGE_PRESENT!() | _PAGE_WRITE!() | _PAGE_USER!() | _CACHE_CC) }; }
macro_rules! PAGE_READONLY { () => { __pgprot(_PAGE_PRESENT!() | _PAGE_USER!() | _CACHE_CC) }; }
macro_rules! PAGE_KERNEL { () => { __pgprot(_PAGE_PRESENT!() | __READABLE!() | __WRITEABLE!() | _PAGE_GLOBAL!() | _PAGE_KERN!() | _CACHE_CC) }; }
macro_rules! PAGE_KERNEL_SUC { () => { __pgprot(_PAGE_PRESENT!() | __READABLE!() | __WRITEABLE!() | _PAGE_GLOBAL!() | _PAGE_KERN!() | _CACHE_SUC) }; }
macro_rules! PAGE_KERNEL_WUC { () => { __pgprot(_PAGE_PRESENT!() | __READABLE!() | __WRITEABLE!() | _PAGE_GLOBAL!() | _PAGE_KERN!() | _CACHE_WUC) }; }

/* External dependencies supplied by other kernel translation units. */
extern "C" {
    static mut wc_enabled: bool;
}

macro_rules! _PAGE_IOREMAP { () => { pgprot_val(PAGE_KERNEL_SUC!()) }; }

#[inline]
pub unsafe fn pgprot_nx(_prot: pgprot_t) -> pgprot_t {
    __pgprot(pgprot_val(_prot) | _PAGE_NO_EXEC!())
}

#[inline]
pub unsafe fn pgprot_noncached(_prot: pgprot_t) -> pgprot_t {
    let mut prot: usize = pgprot_val(_prot);
    prot = (prot & !_CACHE_MASK!()) | _CACHE_SUC;
    __pgprot(prot)
}

#[inline]
pub unsafe fn pgprot_writecombine(_prot: pgprot_t) -> pgprot_t {
    let mut prot: usize = pgprot_val(_prot);
    prot = (prot & !_CACHE_MASK!()) | (if wc_enabled { _CACHE_WUC } else { _CACHE_SUC });
    __pgprot(prot)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
