/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Definitions for Sun3 custom MMU.
 */

/* Dependencies: linux/types.h, asm/movs.h, and asm/sun3-head.h. */

/* MMU characteristics. */
pub const SUN3_SEGMAPS_PER_CONTEXT: usize = 2048;
pub const SUN3_PMEGS_NUM: usize = 256;
pub const SUN3_CONTEXTS_NUM: usize = 8;

pub const SUN3_PMEG_SIZE_BITS: usize = 17;
pub const SUN3_PMEG_SIZE: usize = 1usize << SUN3_PMEG_SIZE_BITS;
pub const SUN3_PMEG_MASK: usize = SUN3_PMEG_SIZE - 1;

pub const SUN3_PTE_SIZE_BITS: usize = 13;
pub const SUN3_PTE_SIZE: usize = 1usize << SUN3_PTE_SIZE_BITS;
pub const SUN3_PTE_MASK: usize = SUN3_PTE_SIZE - 1;

pub const SUN3_CONTROL_MASK: usize = 0x0FFFFFFC;
pub const SUN3_INVALID_PMEG: u8 = 255;
pub const SUN3_INVALID_CONTEXT: u8 = 255;

pub const AC_IDPROM: usize = 0x00000000;
pub const AC_PAGEMAP: usize = 0x10000000;
pub const AC_SEGMAP: usize = 0x20000000;
pub const AC_CONTEXT: usize = 0x30000000;
pub const AC_SENABLE: usize = 0x40000000;
pub const AC_UDVMA_ENB: usize = 0x50000000;
pub const AC_BUS_ERROR: usize = 0x60000000;
pub const AC_SYNC_ERR: usize = 0x60000000;
pub const AC_SYNC_VA: usize = 0x60000004;
pub const AC_ASYNC_ERR: usize = 0x60000008;
pub const AC_ASYNC_VA: usize = 0x6000000c;
pub const AC_LEDS: usize = 0x70000000;
pub const AC_CACHETAGS: usize = 0x80000000;
pub const AC_CACHEDDATA: usize = 0x90000000;
pub const AC_UDVMA_MAP: usize = 0xD0000000;
pub const AC_VME_VECTOR: usize = 0xE0000000;
pub const AC_BOOT_SCC: usize = 0xF0000000;

pub const SUN3_PAGE_CHG_MASK: usize = SUN3_PAGE_PGNUM_MASK | SUN3_PAGE_ACCESSED | SUN3_PAGE_MODIFIED;

/* Bus access type within PTE. */
pub const SUN3_PAGE_TYPE_MASK: usize = 0x0c000000;
pub const SUN3_PAGE_TYPE_MEMORY: usize = 0x00000000;
pub const SUN3_PAGE_TYPE_IO: usize = 0x04000000;
pub const SUN3_PAGE_TYPE_VME16: usize = 0x08000000;
pub const SUN3_PAGE_TYPE_VME32: usize = 0x0c000000;

/* Mask for page number within PTE. */
pub const SUN3_PAGE_PGNUM_MASK: usize = 0x0007FFFF;

/* Bits within bus-error register. */
pub const SUN3_BUSERR_WATCHDOG: u8 = 0x01;
pub const SUN3_BUSERR_UNUSED: u8 = 0x02;
pub const SUN3_BUSERR_FPAENERR: u8 = 0x04;
pub const SUN3_BUSERR_FPABERR: u8 = 0x08;
pub const SUN3_BUSERR_VMEBERR: u8 = 0x10;
pub const SUN3_BUSERR_TIMEOUT: u8 = 0x20;
pub const SUN3_BUSERR_PROTERR: u8 = 0x40;
pub const SUN3_BUSERR_INVALID: u8 = 0x80;

/* The following low-level access operations are supplied by asm/movs.h. */
pub unsafe fn sun3_get_buserr() -> u8 {
    let mut sfc: u8 = 0;
    let mut c: u8 = 0;
    GET_SFC!(sfc);
    SET_SFC!(FC_CONTROL);
    GET_CONTROL_BYTE!(AC_BUS_ERROR, c);
    SET_SFC!(sfc);
    c
}

pub unsafe fn sun3_get_segmap(addr: usize) -> usize {
    let mut entry: usize;
    let mut c: u8 = 0;
    let mut sfc: u8 = 0;
    GET_SFC!(sfc);
    SET_SFC!(FC_CONTROL);
    GET_CONTROL_BYTE!(AC_SEGMAP | (addr & SUN3_CONTROL_MASK), c);
    SET_SFC!(sfc);
    entry = c as usize;
    entry
}

pub unsafe fn sun3_put_segmap(addr: usize, entry: usize) {
    let mut sfc: u8 = 0;
    GET_DFC!(sfc);
    SET_DFC!(FC_CONTROL);
    SET_CONTROL_BYTE!(AC_SEGMAP | (addr & SUN3_CONTROL_MASK), entry);
    SET_DFC!(sfc);
}

pub unsafe fn sun3_get_pte(addr: usize) -> usize {
    let mut entry: usize = 0;
    let mut sfc: u8 = 0;
    GET_SFC!(sfc);
    SET_SFC!(FC_CONTROL);
    GET_CONTROL_WORD!(AC_PAGEMAP | (addr & SUN3_CONTROL_MASK), entry);
    SET_SFC!(sfc);
    entry
}

pub unsafe fn sun3_put_pte(addr: usize, entry: usize) {
    let mut sfc: u8 = 0;
    GET_DFC!(sfc);
    SET_DFC!(FC_CONTROL);
    SET_CONTROL_WORD!(AC_PAGEMAP | (addr & SUN3_CONTROL_MASK), entry);
    SET_DFC!(sfc);
}

/* get current context */
pub unsafe fn sun3_get_context() -> u8 {
    let mut sfc: u8 = 0;
    let mut c: u8 = 0;
    GET_SFC!(sfc);
    SET_SFC!(FC_CONTROL);
    GET_CONTROL_BYTE!(AC_CONTEXT, c);
    SET_SFC!(sfc);
    c
}

/* set alternate context */
pub unsafe fn sun3_put_context(c: u8) {
    let mut dfc: u8 = 0;
    GET_DFC!(dfc);
    SET_DFC!(FC_CONTROL);
    SET_CONTROL_BYTE!(AC_CONTEXT, c);
    SET_DFC!(dfc);
}

extern "C" {
    pub fn sun3_ioremap(phys: usize, size: usize, r#type: usize) -> *mut core::ffi::c_void;
    pub fn sun3_map_test(addr: usize, val: *mut i8) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
