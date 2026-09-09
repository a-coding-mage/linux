/*
 * Architecture specific parts of the Floppy driver
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995
 */

/* C header guard: __ASM_ALPHA_FLOPPY_H */

#[inline]
pub unsafe fn fd_inb(base: usize, reg: usize) -> u8 {
    inb_p(base.wrapping_add(reg))
}

#[inline]
pub unsafe fn fd_outb(value: u8, base: usize, reg: usize) {
    outb_p(value, base.wrapping_add(reg));
}

#[inline]
pub unsafe fn fd_enable_dma() { enable_dma(FLOPPY_DMA); }
#[inline]
pub unsafe fn fd_disable_dma() { disable_dma(FLOPPY_DMA); }
#[inline]
pub unsafe fn fd_request_dma() -> i32 { request_dma(FLOPPY_DMA, c"floppy".as_ptr()) }
#[inline]
pub unsafe fn fd_free_dma() { free_dma(FLOPPY_DMA); }
#[inline]
pub unsafe fn fd_clear_dma_ff() { clear_dma_ff(FLOPPY_DMA); }
#[inline]
pub unsafe fn fd_set_dma_mode(mode: i32) { set_dma_mode(FLOPPY_DMA, mode); }
#[inline]
pub unsafe fn fd_set_dma_addr(addr: *mut core::ffi::c_void) {
    set_dma_addr(FLOPPY_DMA, isa_virt_to_bus(addr));
}
#[inline]
pub unsafe fn fd_set_dma_count(count: usize) { set_dma_count(FLOPPY_DMA, count); }
#[inline]
pub unsafe fn fd_enable_irq() { enable_irq(FLOPPY_IRQ); }
#[inline]
pub unsafe fn fd_disable_irq() { disable_irq(FLOPPY_IRQ); }
#[inline]
pub unsafe fn fd_request_irq() -> i32 {
    request_irq(FLOPPY_IRQ, floppy_interrupt, 0, c"floppy".as_ptr(), core::ptr::null_mut())
}
#[inline]
pub unsafe fn fd_free_irq() { free_irq(FLOPPY_IRQ, core::ptr::null_mut()); }

/* The following declarations are used when CONFIG_PCI is enabled. */
#[cfg(feature = "CONFIG_PCI")]
#[inline]
pub unsafe fn fd_dma_setup(addr: *mut i8, size: usize, mode: i32, io: usize) -> i32 {
    alpha_fd_dma_setup(addr, size, mode, io)
}

#[cfg(feature = "CONFIG_PCI")]
static mut ALPHA_FD_PREV_SIZE: usize = 0;
#[cfg(feature = "CONFIG_PCI")]
static mut ALPHA_FD_BUS_ADDR: usize = 0;
#[cfg(feature = "CONFIG_PCI")]
static mut ALPHA_FD_PREV_ADDR: *mut i8 = core::ptr::null_mut();
#[cfg(feature = "CONFIG_PCI")]
static mut ALPHA_FD_PREV_DIR: i32 = 0;

#[cfg(feature = "CONFIG_PCI")]
#[inline]
pub unsafe fn alpha_fd_dma_setup(addr: *mut i8, size: usize, mode: i32, io: usize) -> i32 {
    let dir = if mode != DMA_MODE_READ { DMA_FROM_DEVICE } else { DMA_TO_DEVICE };

    if ALPHA_FD_BUS_ADDR != 0
        && (addr != ALPHA_FD_PREV_ADDR
            || size != ALPHA_FD_PREV_SIZE
            || dir != ALPHA_FD_PREV_DIR)
    {
        /* different from last time -- unmap prev */
        dma_unmap_single(isa_bridge, ALPHA_FD_BUS_ADDR, ALPHA_FD_PREV_SIZE, ALPHA_FD_PREV_DIR);
        ALPHA_FD_BUS_ADDR = 0;
    }

    /* need to map it */
    if ALPHA_FD_BUS_ADDR == 0 {
        ALPHA_FD_BUS_ADDR = dma_map_single(isa_bridge, addr, size, dir);
    }

    /* remember this one as prev */
    ALPHA_FD_PREV_ADDR = addr;
    ALPHA_FD_PREV_SIZE = size;
    ALPHA_FD_PREV_DIR = dir;

    fd_clear_dma_ff();
    fd_set_dma_mode(mode);
    set_dma_addr(FLOPPY_DMA, ALPHA_FD_BUS_ADDR);
    fd_set_dma_count(size);
    virtual_dma_port = io;
    fd_enable_dma();

    0
}

#[inline]
pub unsafe fn virtual_dma_init() {
    /* Nothing to do on an Alpha */
}

static mut FDC1: i32 = 0x3f0;
static mut FDC2: i32 = -1;

/*
 * Again, the CMOS information doesn't work on the alpha..
 */
pub const FLOPPY0_TYPE: i32 = 6;
pub const FLOPPY1_TYPE: i32 = 0;

pub const N_FDC: i32 = 2;
pub const N_DRIVE: i32 = 8;

/* EXTRA_FLOPPY_PARAMS */

/* C header guard end: __ASM_ALPHA_FLOPPY_H */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
