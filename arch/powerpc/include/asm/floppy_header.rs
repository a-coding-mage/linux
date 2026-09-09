/*
 * Architecture specific parts of the Floppy driver
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995
 */

// C header guard: __ASM_POWERPC_FLOPPY_H
// The following declarations are present only when __KERNEL__ is defined.

// Dependency supplied externally: asm/machdep.h, linux/pci.h, asm/ppc-pci.h.

#[inline]
unsafe fn fd_inb(base: usize, reg: usize) -> u8 {
    inb_p(base.wrapping_add(reg))
}

#[inline]
unsafe fn fd_outb(value: u8, base: usize, reg: usize) {
    outb_p(value, base.wrapping_add(reg));
}

#[inline]
unsafe fn fd_enable_dma() { enable_dma(FLOPPY_DMA); }
#[inline]
unsafe fn fd_disable_dma() { ((*fd_ops)._disable_dma)(FLOPPY_DMA); }
#[inline]
unsafe fn fd_free_dma() { ((*fd_ops)._free_dma)(FLOPPY_DMA); }
#[inline]
unsafe fn fd_clear_dma_ff() { clear_dma_ff(FLOPPY_DMA); }
#[inline]
unsafe fn fd_set_dma_mode(mode: i32) { set_dma_mode(FLOPPY_DMA, mode); }
#[inline]
unsafe fn fd_set_dma_count(count: usize) { set_dma_count(FLOPPY_DMA, count); }
#[inline]
unsafe fn fd_get_dma_residue() -> i32 { ((*fd_ops)._get_dma_residue)(FLOPPY_DMA) }
#[inline]
unsafe fn fd_enable_irq() { enable_irq(FLOPPY_IRQ); }
#[inline]
unsafe fn fd_disable_irq() { disable_irq(FLOPPY_IRQ); }
#[inline]
unsafe fn fd_free_irq() { free_irq(FLOPPY_IRQ, core::ptr::null_mut()); }

#[inline]
unsafe fn fd_dma_setup(addr: *mut i8, size: usize, mode: i32, io: usize) -> i32 {
    ((*fd_ops)._dma_setup)(addr, size, mode, io)
}

struct FdDmaOps {
    _disable_dma: unsafe extern "C" fn(dmanr: u32),
    _free_dma: unsafe extern "C" fn(dmanr: u32),
    _get_dma_residue: unsafe extern "C" fn(dummy: u32) -> i32,
    _dma_setup: unsafe extern "C" fn(addr: *mut i8, size: usize, mode: i32, io: usize) -> i32,
}

static mut virtual_dma_count: usize = 0;
static mut virtual_dma_residue: i32 = 0;
static mut virtual_dma_addr: *mut i8 = core::ptr::null_mut();
static mut virtual_dma_mode: i32 = 0;
static mut doing_vdma: i32 = 0;
static mut fd_ops: *mut FdDmaOps = core::ptr::null_mut();

unsafe fn floppy_hardint(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let mut st: u8;
    let mut lcount: usize;
    let mut lptr: *mut i8;

    if doing_vdma == 0 {
        return floppy_interrupt(irq, dev_id);
    }

    st = 1;
    lcount = virtual_dma_count;
    lptr = virtual_dma_addr;
    while lcount != 0 {
        st = inb(virtual_dma_port.wrapping_add(FD_STATUS));
        st &= STATUS_DMA | STATUS_READY;
        if st != (STATUS_DMA | STATUS_READY) {
            break;
        }
        if virtual_dma_mode != 0 {
            outb_p(*lptr as u8, virtual_dma_port.wrapping_add(FD_DATA));
        } else {
            *lptr = inb_p(virtual_dma_port.wrapping_add(FD_DATA)) as i8;
        }
        lcount -= 1;
        lptr = lptr.add(1);
    }
    virtual_dma_count = lcount;
    virtual_dma_addr = lptr;
    st = inb(virtual_dma_port.wrapping_add(FD_STATUS));

    if st == STATUS_DMA {
        return IRQ_HANDLED;
    }
    if (st & STATUS_DMA) == 0 {
        virtual_dma_residue += virtual_dma_count as i32;
        virtual_dma_count = 0;
        doing_vdma = 0;
        floppy_interrupt(irq, dev_id);
        return IRQ_HANDLED;
    }
    IRQ_HANDLED
}

unsafe fn vdma_disable_dma(_dummy: u32) {
    doing_vdma = 0;
    virtual_dma_residue += virtual_dma_count as i32;
    virtual_dma_count = 0;
}

unsafe fn vdma_nop(_dummy: u32) {}

unsafe fn vdma_get_dma_residue(_dummy: u32) -> i32 {
    virtual_dma_count as i32 + virtual_dma_residue
}

unsafe fn fd_request_irq() -> i32 {
    if can_use_virtual_dma != 0 {
        request_irq(FLOPPY_IRQ, floppy_hardint, 0, b"floppy\0".as_ptr() as *const i8, core::ptr::null_mut())
    } else {
        request_irq(FLOPPY_IRQ, floppy_interrupt, 0, b"floppy\0".as_ptr() as *const i8, core::ptr::null_mut())
    }
}

unsafe fn vdma_dma_setup(addr: *mut i8, size: usize, mode: i32, io: usize) -> i32 {
    doing_vdma = 1;
    virtual_dma_port = io;
    virtual_dma_mode = (mode == DMA_MODE_WRITE) as i32;
    virtual_dma_addr = addr;
    virtual_dma_count = size;
    virtual_dma_residue = 0;
    0
}

unsafe fn hard_dma_setup(addr: *mut i8, size: usize, mode: i32, io: usize) -> i32 {
    static mut prev_size: usize = 0;
    static mut bus_addr: dma_addr_t = 0;
    static mut prev_addr: *mut i8 = core::ptr::null_mut();
    static mut prev_dir: i32 = 0;
    let dir: i32;

    doing_vdma = 0;
    dir = if mode == DMA_MODE_READ { DMA_FROM_DEVICE } else { DMA_TO_DEVICE };

    if bus_addr != 0 && (addr != prev_addr || size != prev_size || dir != prev_dir) {
        dma_unmap_single(&mut (*isa_bridge_pcidev).dev, bus_addr, prev_size, prev_dir);
        bus_addr = 0;
    }

    if bus_addr == 0 {
        bus_addr = dma_map_single(&mut (*isa_bridge_pcidev).dev, addr, size, dir);
        if dma_mapping_error(&mut (*isa_bridge_pcidev).dev, bus_addr) {
            return -ENOMEM;
        }
    }

    prev_addr = addr;
    prev_size = size;
    prev_dir = dir;

    fd_clear_dma_ff();
    fd_set_dma_mode(mode);
    set_dma_addr(FLOPPY_DMA, bus_addr);
    fd_set_dma_count(size);
    virtual_dma_port = io;
    fd_enable_dma();
    0
}

static mut real_dma_ops: FdDmaOps = FdDmaOps {
    _disable_dma: disable_dma,
    _free_dma: free_dma,
    _get_dma_residue: get_dma_residue,
    _dma_setup: hard_dma_setup,
};

static mut virt_dma_ops: FdDmaOps = FdDmaOps {
    _disable_dma: vdma_disable_dma,
    _free_dma: vdma_nop,
    _get_dma_residue: vdma_get_dma_residue,
    _dma_setup: vdma_dma_setup,
};

unsafe fn fd_request_dma() -> i32 {
    if (can_use_virtual_dma & 1) != 0 {
        fd_ops = &raw mut virt_dma_ops;
        0
    } else {
        fd_ops = &raw mut real_dma_ops;
        request_dma(FLOPPY_DMA, b"floppy\0".as_ptr() as *const i8)
    }
}

static mut FDC1: i32 = 0x3f0;
static mut FDC2: i32 = -1;

/* Again, the CMOS information not available */
const FLOPPY0_TYPE: i32 = 6;
const FLOPPY1_TYPE: i32 = 0;

const N_FDC: i32 = 2; // Don't change this!
const N_DRIVE: i32 = 8;

// #define EXTRA_FLOPPY_PARAMS

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
