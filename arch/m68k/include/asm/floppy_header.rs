/*
 * Implementation independent bits of the Floppy driver.
 *
 * much of this file is derived from what was originally the Q40 floppy driver.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1999, 2000, 2001
 *
 * Sun3x support added 2/4/2000 Sam Creasey (sammy@sammy.net)
 */

// <asm/io.h>, <linux/vmalloc.h>, and <asm/sun3xflop.h> dependencies are supplied externally.

extern "C" {
    fn floppy_interrupt(irq: i32, dev_id: *mut core::ffi::c_void);
    fn inb(port: i32) -> u8;
    fn inb_p(port: i32) -> u8;
    fn outb_p(value: u8, port: i32);
    fn vmalloc(size: usize) -> *mut core::ffi::c_void;
    fn vfree(ptr: *mut core::ffi::c_void);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t,
                   flags: i32, name: *const u8, dev_id: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t) -> i32;
    fn free_irq(irq: i32, dev_id: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t);
    fn sun3x_82072_fd_inb(port: i32) -> u8;
    fn sun3x_82072_fd_outb(value: u8, port: i32);
    fn sun3xflop_request_irq() -> i32;
    fn sun3xflop_init() -> i32;
}

pub type irqreturn_t = i32;
pub type spinlock_t = core::ffi::c_void;

pub const MAX_DMA_ADDRESS: usize = 0x00;
// FLOPPY0_TYPE is (MACH_IS_Q40 ? 6 : 4); FLOPPY1_TYPE is 0.
pub const FLOPPY1_TYPE: i32 = 0;
pub const N_FDC: i32 = 1;
pub const N_DRIVE: i32 = 8;
pub const DMA_MODE_READ: i32 = 0x44;
pub const DMA_MODE_WRITE: i32 = 0x48;

static mut virtual_dma_count: i32 = 0;
static mut virtual_dma_residue: i32 = 0;
static mut virtual_dma_addr: *mut i8 = core::ptr::null_mut();
static mut virtual_dma_mode: i32 = 0;
static mut doing_pdma: i32 = 0;

extern "C" {
    static mut dma_spin_lock: spinlock_t;
    static mut use_virtual_dma: i32;
    static mut can_use_virtual_dma: i32;
    static mut virtual_dma_port: i32;
}

#[inline]
pub unsafe fn claim_dma_lock() -> usize {
    let mut flags = 0usize;
    spin_lock_irqsave(&mut dma_spin_lock, &mut flags);
    flags
}

#[inline]
pub unsafe fn release_dma_lock(flags: usize) {
    spin_unlock_irqrestore(&mut dma_spin_lock, flags);
}

#[inline]
pub unsafe fn fd_inb(base: i32, reg: i32) -> u8 {
    if MACH_IS_Q40 { inb_p(base + reg) }
    else if MACH_IS_SUN3X { sun3x_82072_fd_inb(base + reg) }
    else { 0 }
}

#[inline]
pub unsafe fn fd_outb(value: u8, base: i32, reg: i32) {
    if MACH_IS_Q40 { outb_p(value, base + reg); }
    else if MACH_IS_SUN3X { sun3x_82072_fd_outb(value, base + reg); }
}

pub unsafe fn fd_request_irq() -> i32 {
    if MACH_IS_Q40 { request_irq(FLOPPY_IRQ, floppy_hardint, 0, b"floppy\0".as_ptr(), floppy_hardint) }
    else if MACH_IS_SUN3X { sun3xflop_request_irq() }
    else { -ENXIO }
}

pub unsafe fn fd_free_irq() { if MACH_IS_Q40 { free_irq(FLOPPY_IRQ, floppy_hardint); } }

pub const fn fd_enable_irq() {}
pub const fn fd_disable_irq() {}
pub const fn fd_free_dma() {}

pub unsafe fn m68k_floppy_init() -> i32 {
    use_virtual_dma = 1; can_use_virtual_dma = 1;
    if MACH_IS_Q40 { 0x3f0 } else if MACH_IS_SUN3X { sun3xflop_init() } else { -1 }
}

pub unsafe fn vdma_request_dma(_dmanr: u32, _device_id: *const u8) -> i32 { 0 }

pub unsafe fn vdma_get_dma_residue(_dummy: u32) -> i32 { virtual_dma_count + virtual_dma_residue }

pub unsafe fn vdma_mem_alloc(size: usize) -> usize { vmalloc(size) as usize }

pub unsafe fn _fd_dma_mem_free(addr: usize, _size: usize) { vfree(addr as *mut core::ffi::c_void); }

pub unsafe fn vdma_dma_setup(addr: *mut i8, size: usize, mode: i32, io: i32) -> i32 {
    doing_pdma = 1;
    virtual_dma_port = if MACH_IS_Q40 { io } else { 0 };
    virtual_dma_mode = (mode == DMA_MODE_WRITE) as i32;
    virtual_dma_addr = addr;
    virtual_dma_count = size as i32;
    virtual_dma_residue = 0;
    0
}

pub unsafe fn fd_disable_dma() {
    doing_pdma = 0;
    virtual_dma_residue += virtual_dma_count;
    virtual_dma_count = 0;
}

pub unsafe extern "C" fn floppy_hardint(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let mut st: u8;
    if doing_pdma == 0 {
        floppy_interrupt(irq, dev_id);
        return IRQ_HANDLED;
    }
    let mut lcount = virtual_dma_count;
    let mut lptr = virtual_dma_addr;
    st = 1;
    while lcount != 0 {
        st = inb(virtual_dma_port + FD_STATUS);
        st &= STATUS_DMA | STATUS_READY;
        if st != (STATUS_DMA | STATUS_READY) { break; }
        if virtual_dma_mode != 0 {
            outb_p(*lptr as u8, virtual_dma_port + FD_DATA);
        } else {
            *lptr = inb_p(virtual_dma_port + FD_DATA) as i8;
        }
        lcount -= 1;
        lptr = lptr.add(1);
    }
    virtual_dma_count = lcount;
    virtual_dma_addr = lptr;
    st = inb(virtual_dma_port + FD_STATUS);
    if st == STATUS_DMA { return IRQ_HANDLED; }
    if (st & STATUS_DMA) == 0 {
        virtual_dma_residue += virtual_dma_count;
        virtual_dma_count = 0;
        doing_pdma = 0;
        floppy_interrupt(irq, dev_id);
    }
    IRQ_HANDLED
}

pub const FD_STATUS: i32 = 0; // supplied by asm/io.h
pub const FD_DATA: i32 = 0; // supplied by asm/io.h
pub const STATUS_DMA: u8 = 0; // supplied by asm/io.h
pub const STATUS_READY: u8 = 0; // supplied by asm/io.h
pub const FLOPPY_IRQ: i32 = 0; // supplied externally
pub const FLOPPY_DMA: u32 = 0; // supplied externally
pub const ENXIO: i32 = 6;
// MACH_IS_Q40 and MACH_IS_SUN3X are supplied externally.
extern "C" {
    static MACH_IS_Q40: bool;
    static MACH_IS_SUN3X: bool;
}

pub const EXTRA_FLOPPY_PARAMS: () = ();
pub const IRQ_HANDLED: irqreturn_t = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
