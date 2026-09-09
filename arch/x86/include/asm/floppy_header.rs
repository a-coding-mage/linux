/*
 * Architecture specific parts of the Floppy driver
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995
 */

// C dependencies supplied by the surrounding kernel translation unit.

pub const FLOPPY_CAN_FALLBACK_ON_NODMA: bool = true;

// C macros fd_request_dma, fd_free_dma, fd_enable_irq, fd_disable_irq,
// fd_free_irq, fd_get_dma_residue, fd_dma_mem_alloc, fd_dma_setup, SW and CSW
// select fd_routine[use_virtual_dma & 1] or fd_routine[can_use_virtual_dma & 1].

#[inline]
pub unsafe fn _cross_64kb(a: usize, s: usize, vdma: bool) -> bool {
    !vdma && (a / SZ_64K != (a.wrapping_add(s).wrapping_sub(1)) / SZ_64K)
}

static mut virtual_dma_count: i32 = 0;
static mut virtual_dma_residue: i32 = 0;
static mut virtual_dma_addr: *mut i8 = core::ptr::null_mut();
static mut virtual_dma_mode: i32 = 0;
static mut doing_pdma: i32 = 0;

#[inline]
pub unsafe fn fd_inb(base: u16, reg: u16) -> u8 {
    let ret = inb_p(base.wrapping_add(reg));
    native_io_delay();
    native_io_delay();
    native_io_delay();
    ret
}

#[inline]
pub unsafe fn fd_outb(value: u8, base: u16, reg: u16) {
    outb_p(value, base.wrapping_add(reg));
    native_io_delay();
    native_io_delay();
    native_io_delay();
}

pub unsafe fn floppy_hardint(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let mut st: u8;
    if doing_pdma == 0 {
        return floppy_interrupt(irq, dev_id);
    }
    let mut lcount = virtual_dma_count;
    let mut lptr = virtual_dma_addr;
    while lcount != 0 {
        st = inb(virtual_dma_port.wrapping_add(FD_STATUS));
        st &= STATUS_DMA | STATUS_READY;
        if st != (STATUS_DMA | STATUS_READY) { break; }
        if virtual_dma_mode != 0 {
            fd_outb(*lptr as u8, virtual_dma_port, FD_DATA);
        } else {
            *lptr = fd_inb(virtual_dma_port, FD_DATA) as i8;
        }
        lcount -= 1;
        lptr = lptr.add(1);
    }
    virtual_dma_count = lcount;
    virtual_dma_addr = lptr;
    st = inb(virtual_dma_port.wrapping_add(FD_STATUS));
    if st == STATUS_DMA { return IRQ_HANDLED; }
    if st & STATUS_DMA == 0 {
        virtual_dma_residue += virtual_dma_count;
        virtual_dma_count = 0;
        doing_pdma = 0;
        floppy_interrupt(irq, dev_id);
        return IRQ_HANDLED;
    }
    IRQ_HANDLED
}

pub unsafe fn fd_disable_dma() {
    if can_use_virtual_dma & 1 == 0 { disable_dma(FLOPPY_DMA); }
    doing_pdma = 0;
    virtual_dma_residue += virtual_dma_count;
    virtual_dma_count = 0;
}

pub unsafe fn vdma_request_dma(_dmanr: u32, _device_id: *const i8) -> i32 { 0 }
pub unsafe fn vdma_nop(_dummy: u32) {}
pub unsafe fn vdma_get_dma_residue(_dummy: u32) -> i32 { virtual_dma_count + virtual_dma_residue }

pub unsafe fn fd_request_irq() -> i32 {
    if can_use_virtual_dma != 0 {
        request_irq(FLOPPY_IRQ, floppy_hardint, 0, b"floppy\0".as_ptr() as *const i8, core::ptr::null_mut())
    } else {
        request_irq(FLOPPY_IRQ, floppy_interrupt, 0, b"floppy\0".as_ptr() as *const i8, core::ptr::null_mut())
    }
}

pub unsafe fn dma_mem_alloc(size: u64) -> u64 { __get_dma_pages(GFP_KERNEL | __GFP_NORETRY, get_order(size)) }
pub unsafe fn vdma_mem_alloc(size: u64) -> u64 { vmalloc(size) as u64 }
pub unsafe fn nodma_mem_alloc(size: u64) -> u64 { vdma_mem_alloc(size) }

pub unsafe fn _fd_dma_mem_free(addr: u64, size: u64) {
    if addr >= high_memory as u64 { vfree(addr as *mut core::ffi::c_void); }
    else { free_pages(addr, get_order(size)); }
}

pub unsafe fn _fd_chose_dma_mode(addr: *mut i8, size: u64) {
    if can_use_virtual_dma == 2 {
        if addr as u64 >= high_memory as u64 || isa_virt_to_bus(addr) >= 0x1000000 || _cross_64kb(addr as usize, size as usize, false) { use_virtual_dma = 1; }
        else { use_virtual_dma = 0; }
    } else { use_virtual_dma = can_use_virtual_dma & 1; }
}

pub unsafe fn vdma_dma_setup(addr: *mut i8, size: u64, mode: i32, io: u16) -> i32 {
    doing_pdma = 1;
    virtual_dma_port = io;
    virtual_dma_mode = (mode == DMA_MODE_WRITE) as i32;
    virtual_dma_addr = addr;
    virtual_dma_count = size as i32;
    virtual_dma_residue = 0;
    0
}

pub unsafe fn hard_dma_setup(addr: *mut i8, size: u64, mode: i32, _io: u16) -> i32 {
    doing_pdma = 0;
    clear_dma_ff(FLOPPY_DMA);
    set_dma_mode(FLOPPY_DMA, mode);
    set_dma_addr(FLOPPY_DMA, isa_virt_to_bus(addr));
    set_dma_count(FLOPPY_DMA, size);
    enable_dma(FLOPPY_DMA);
    0
}

// FLOPPY_SANITY_CHECK conditionally rejects a DMA transfer crossing 64 KiB.

#[repr(C)]
pub struct fd_routine_l {
    pub _request_dma: unsafe fn(u32, *const i8) -> i32,
    pub _free_dma: unsafe fn(u32),
    pub _get_dma_residue: unsafe fn(u32) -> i32,
    pub _dma_mem_alloc: unsafe fn(u64) -> u64,
    pub _dma_setup: unsafe fn(*mut i8, u64, i32, u16) -> i32,
}

// The C initializer contains the hardware and virtual-DMA routine tables;
// their external kernel callbacks are represented by the declarations below.
extern "C" {
    pub static mut fd_routine: [fd_routine_l; 2];
}

#[inline]
pub unsafe fn floppy0_type() -> u8 {
    let mut flags: u64 = 0;
    spin_lock_irqsave(&mut rtc_lock, &mut flags);
    let val = ((CMOS_READ(0x10) >> 4) & 15) as u8;
    spin_unlock_irqrestore(&mut rtc_lock, flags);
    val
}

#[inline]
pub unsafe fn floppy1_type() -> u8 {
    let mut flags: u64 = 0;
    spin_lock_irqsave(&mut rtc_lock, &mut flags);
    let val = (CMOS_READ(0x10) & 15) as u8;
    spin_unlock_irqrestore(&mut rtc_lock, flags);
    val
}

// C macro: EXTRA_FLOPPY_PARAMS

static mut FDC1: i32 = 0x3f0;
static mut FDC2: i32 = -1;

pub const N_FDC: i32 = 2;
pub const N_DRIVE: i32 = 8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
