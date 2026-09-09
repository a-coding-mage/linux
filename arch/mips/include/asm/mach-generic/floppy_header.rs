/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1996, 1997, 1998, 2003 by Ralf Baechle
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external; the original header included the corresponding C headers.

/*
 * How to access the FDC's registers.
 */
#[inline]
pub unsafe fn fd_inb(base: ::core::ffi::c_uint, reg: ::core::ffi::c_uint) -> u8 {
    inb_p(base.wrapping_add(reg))
}

#[inline]
pub unsafe fn fd_outb(value: u8, base: ::core::ffi::c_uint, reg: ::core::ffi::c_uint) {
    outb_p(value, base.wrapping_add(reg));
}

/*
 * How to access the floppy DMA functions.
 */
#[inline]
pub unsafe fn fd_enable_dma() {
    enable_dma(FLOPPY_DMA);
}

#[inline]
pub unsafe fn fd_disable_dma() {
    disable_dma(FLOPPY_DMA);
}

#[inline]
pub unsafe fn fd_request_dma() -> ::core::ffi::c_int {
    request_dma(FLOPPY_DMA, b"floppy\0".as_ptr() as *const ::core::ffi::c_char)
}

#[inline]
pub unsafe fn fd_free_dma() {
    free_dma(FLOPPY_DMA);
}

#[inline]
pub unsafe fn fd_clear_dma_ff() {
    clear_dma_ff(FLOPPY_DMA);
}

#[inline]
pub unsafe fn fd_set_dma_mode(mode: i8) {
    set_dma_mode(FLOPPY_DMA, mode);
}

#[inline]
pub unsafe fn fd_set_dma_addr(addr: *mut i8) {
    set_dma_addr(FLOPPY_DMA, addr as ::core::ffi::c_ulong);
}

#[inline]
pub unsafe fn fd_set_dma_count(count: ::core::ffi::c_uint) {
    set_dma_count(FLOPPY_DMA, count);
}

#[inline]
pub unsafe fn fd_get_dma_residue() -> ::core::ffi::c_int {
    get_dma_residue(FLOPPY_DMA)
}

#[inline]
pub unsafe fn fd_enable_irq() {
    enable_irq(FLOPPY_IRQ);
}

#[inline]
pub unsafe fn fd_disable_irq() {
    disable_irq(FLOPPY_IRQ);
}

#[inline]
pub unsafe fn fd_request_irq() -> ::core::ffi::c_int {
    request_irq(
        FLOPPY_IRQ,
        floppy_interrupt,
        0,
        b"floppy\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null_mut(),
    )
}

#[inline]
pub unsafe fn fd_free_irq() {
    free_irq(FLOPPY_IRQ, ::core::ptr::null_mut());
}

// The source also defines the function-like macro: #define fd_free_irq() free_irq(FLOPPY_IRQ, NULL);

#[inline]
pub unsafe fn fd_getfdaddr1() -> ::core::ffi::c_ulong {
    0x3f0
}

#[inline]
pub unsafe fn fd_dma_mem_alloc(size: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    __get_dma_pages(GFP_KERNEL, get_order(size))
}

#[inline]
pub unsafe fn fd_dma_mem_free(addr: ::core::ffi::c_ulong, size: ::core::ffi::c_ulong) {
    free_pages(addr, get_order(size));
}

#[inline]
pub unsafe fn fd_drive_type(n: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    if n == 0 {
        4 /* 3,5\", 1.44mb */
    } else {
        0
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
