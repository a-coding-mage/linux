/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1998, 2003 by Ralf Baechle
 */

// Dependencies supplied by the surrounding kernel translation.

#[inline]
pub unsafe fn fd_inb(base: ::core::ffi::c_uint, reg: ::core::ffi::c_uint) -> u8 {
    let c = core::ptr::read_volatile((base.wrapping_add(reg)) as *const u8);
    udelay(1);
    c
}

#[inline]
pub unsafe fn fd_outb(value: u8, base: ::core::ffi::c_uint, reg: ::core::ffi::c_uint) {
    core::ptr::write_volatile((base.wrapping_add(reg)) as *mut u8, value);
}

/*
 * How to access the floppy DMA functions.
 */
#[inline]
pub unsafe fn fd_enable_dma() {
    vdma_enable(JAZZ_FLOPPY_DMA);
}

#[inline]
pub unsafe fn fd_disable_dma() {
    vdma_disable(JAZZ_FLOPPY_DMA);
}

#[inline]
pub unsafe fn fd_request_dma() -> ::core::ffi::c_int {
    0
}

#[inline]
pub unsafe fn fd_free_dma() {}

#[inline]
pub unsafe fn fd_clear_dma_ff() {}

#[inline]
pub unsafe fn fd_set_dma_mode(mode: i8) {
    vdma_set_mode(JAZZ_FLOPPY_DMA, mode);
}

#[inline]
pub unsafe fn fd_set_dma_addr(a: *mut i8) {
    vdma_set_addr(
        JAZZ_FLOPPY_DMA,
        vdma_phys2log(CPHYSADDR(a as ::core::ffi::c_ulong)),
    );
}

#[inline]
pub unsafe fn fd_set_dma_count(count: ::core::ffi::c_uint) {
    vdma_set_count(JAZZ_FLOPPY_DMA, count);
}

#[inline]
pub unsafe fn fd_get_dma_residue() -> ::core::ffi::c_int {
    vdma_get_residue(JAZZ_FLOPPY_DMA)
}

#[inline]
pub unsafe fn fd_enable_irq() {}

#[inline]
pub unsafe fn fd_disable_irq() {}

#[inline]
pub unsafe fn fd_request_irq() -> ::core::ffi::c_int {
    request_irq(FLOPPY_IRQ, floppy_interrupt, 0, c"floppy".as_ptr(), core::ptr::null_mut())
}

#[inline]
pub unsafe fn fd_free_irq() {
    free_irq(FLOPPY_IRQ, core::ptr::null_mut());
}

#[inline]
pub unsafe fn fd_getfdaddr1() -> ::core::ffi::c_ulong {
    JAZZ_FDC_BASE
}

#[inline]
pub unsafe fn fd_dma_mem_alloc(size: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    let mem = __get_dma_pages(GFP_KERNEL, get_order(size));
    if mem == 0 {
        return 0;
    }
    vdma_alloc(CPHYSADDR(mem), size); // XXX error checking
    mem
}

#[inline]
pub unsafe fn fd_dma_mem_free(addr: ::core::ffi::c_ulong, size: ::core::ffi::c_ulong) {
    vdma_free(vdma_phys2log(CPHYSADDR(addr)));
    free_pages(addr, get_order(size));
}

#[inline]
pub unsafe fn fd_drive_type(n: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    /* XXX This is wrong for machines with ED 2.88mb disk drives like the
       Olivetti M700.  Anyway, we should suck this from the ARC
       firmware.  */
    if n == 0 {
        return 4; // 3,5", 1.44mb
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
