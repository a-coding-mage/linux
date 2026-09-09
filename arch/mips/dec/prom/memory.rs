// SPDX-License-Identifier: GPL-2.0
/*
 * memory.c: memory initialisation code.
 *
 * Copyright (C) 1998 Harald Koerfgen, Frieder Streffer and Paul M. Antoine
 * Copyright (C) 2000, 2002  Maciej W. Rozycki
 */

// Dependencies supplied by the surrounding kernel translation unit.
use core::ffi::{c_char, c_void};

extern "C" {
    static mut mem_err: c_ulong;
    static mut genexcept_early: u8;
    static mut _text: u8;
    static mut IOASIC: bool;

    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn rex_getbitmap(bm: *mut memmap) -> i32;
    fn prom_is_rex(magic: u32) -> bool;
    fn memblock_add(base: c_ulong, size: c_ulong);
    fn free_init_pages(name: *const c_char, begin: c_ulong, end: c_ulong);
    fn __pa(addr: *const c_void) -> c_ulong;
}

type c_ulong = usize;

#[repr(C)]
struct memmap {
    bitmap: [u8; 0],
    pagesize: c_ulong,
}

const CKSEG0: c_ulong = 0xffffffff80000000usize;
const CKSEG1: c_ulong = 0xffffffffa0000000usize;
const PAGE_SIZE: c_ulong = 4096;
const CHUNK_SIZE: c_ulong = 0x400000;

#[inline]
unsafe fn ckseg0addr(addr: c_ulong) -> *mut memmap {
    (CKSEG0 | addr) as *mut memmap
}

unsafe fn pmax_setup_memory_region() {
    let mut memory_page: *mut u8;
    let mut dummy: u8;
    let mut old_handler = [0u8; 0x80];

    /* Install exception handler */
    memcpy(
        old_handler.as_mut_ptr() as *mut c_void,
        (CKSEG0 + 0x80) as *const c_void,
        0x80,
    );
    memcpy(
        (CKSEG0 + 0x80) as *mut c_void,
        (&raw mut genexcept_early) as *const c_void,
        0x80,
    );

    /* read unmapped and uncached (KSEG1)
     * DECstations have at least 4MB RAM
     * Assume less than 480MB of RAM, as this is max for 5000/2xx
     * FIXME this should be replaced by the first free page!
     */
    memory_page = (CKSEG1 + CHUNK_SIZE) as *mut u8;
    while mem_err == 0 && (memory_page as c_ulong) < CKSEG1 + 0x1e00000 {
        dummy = core::ptr::read_volatile(memory_page);
        let _ = dummy;
        memory_page = memory_page.add(CHUNK_SIZE);
    }
    memcpy(
        (CKSEG0 + 0x80) as *mut c_void,
        old_handler.as_ptr() as *const c_void,
        0x80,
    );

    memblock_add(0, memory_page as c_ulong - CKSEG1 - CHUNK_SIZE);
}

unsafe fn rex_setup_memory_region() {
    let mut mem_start: c_ulong = 0;
    let mut mem_size: c_ulong = 0;
    let bm = ckseg0addr(0x28000);
    let bitmap_size = rex_getbitmap(bm);

    for i in 0..bitmap_size {
        /* FIXME: very simplistically only add full sets of pages */
        let bitmap_byte = *((*bm).bitmap.as_ptr().add(i as usize));
        if bitmap_byte == 0xff {
            mem_size += 8 * (*bm).pagesize;
        } else if mem_size == 0 {
            mem_start += 8 * (*bm).pagesize;
        } else {
            memblock_add(mem_start, mem_size);
            mem_start += mem_size + 8 * (*bm).pagesize;
            mem_size = 0;
        }
    }
    if mem_size != 0 {
        memblock_add(mem_start, mem_size);
    }
}

pub unsafe fn prom_meminit(magic: u32) {
    if !prom_is_rex(magic) {
        pmax_setup_memory_region();
    } else {
        rex_setup_memory_region();
    }
}

pub unsafe fn prom_free_prom_memory() {
    let end: c_ulong;

    /*
     * Free everything below the kernel itself but leave
     * the first page reserved for the exception handlers.
     */

    /* CONFIG_DECLANCE conditional is supplied by the build configuration. */
    #[cfg(CONFIG_DECLANCE)]
    {
        /*
         * Leave 128 KB reserved for Lance memory for
         * IOASIC DECstations.
         *
         * XXX: save this address for use in dec_lance.c?
         */
        if IOASIC {
            end = __pa((&raw mut _text).cast::<c_void>()) - 0x00020000;
        } else {
            end = __pa((&raw mut _text).cast::<c_void>());
        }
    }
    #[cfg(not(CONFIG_DECLANCE))]
    {
        end = __pa((&raw mut _text).cast::<c_void>());
    }

    free_init_pages(
        b"unused PROM memory\0".as_ptr() as *const c_char,
        PAGE_SIZE,
        end,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
