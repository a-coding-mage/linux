/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Architecture specific parts of the Floppy driver. */
/* C dependencies: linux/sizes.h and linux/vmalloc.h. */

#[allow(non_camel_case_types, non_upper_case_globals, dead_code)]

#[inline]
pub unsafe fn _CROSS_64KB(a: *const core::ffi::c_void, s: usize, vdma: i32) -> bool {
    (!vdma != 0)
        && ((a as usize) / SZ_64K != ((a as usize).wrapping_add(s).wrapping_sub(1)) / SZ_64K)
}

/* The following declarations are supplied by the floppy driver and kernel. */
extern "C" {
    static mut virtual_dma_port: i32;
    static mut use_virtual_dma: i32;
    static mut can_use_virtual_dma: i32;
    static mut high_memory: *mut core::ffi::c_void;

    fn readb(addr: *mut core::ffi::c_void) -> u8;
    fn writeb(value: u8, addr: *mut core::ffi::c_void);
    fn floppy_interrupt(irq: i32, dev_id: *mut core::ffi::c_void, regs: *mut pt_regs);
    fn request_dma(dmanr: u32, device_id: *const i8) -> i32;
    fn free_dma(dmanr: u32);
    fn get_dma_residue(dummy: i32) -> i32;
    fn enable_irq(irq: i32);
    fn disable_irq(irq: i32);
    fn free_irq(irq: i32, dev_id: *mut core::ffi::c_void);
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void, *mut pt_regs), flags: u32, name: *const i8, dev_id: *mut core::ffi::c_void) -> i32;
    fn disable_dma(dmanr: u32);
    fn clear_dma_ff(dmanr: u32);
    fn set_dma_mode(dmanr: u32, mode: i32);
    fn set_dma_addr(dmanr: u32, addr: usize);
    fn set_dma_count(dmanr: u32, size: usize);
    fn enable_dma(dmanr: u32);
    fn __get_dma_pages(gfp_mask: u32, order: u32) -> usize;
    fn get_order(size: usize) -> u32;
    fn vmalloc(size: usize) -> *mut core::ffi::c_void;
    fn vfree(addr: *mut core::ffi::c_void);
    fn free_pages(addr: usize, order: u32);
    fn virt_to_phys(addr: *const core::ffi::c_void) -> usize;
}

#[repr(C)]
pub struct pt_regs;

pub const SZ_64K: usize = 64 * 1024;
pub const FLOPPY_CAN_FALLBACK_ON_NODMA: bool = true;

pub const FD_STATUS: i32 = 4;
pub const FD_DATA: i32 = 5;
pub const STATUS_DMA: u8 = 0x20;
pub const STATUS_READY: u8 = 0x80;
pub const DMA_MODE_WRITE: i32 = 0x04;

pub static mut virtual_dma_count: i32 = 0;
pub static mut virtual_dma_residue: i32 = 0;
pub static mut virtual_dma_addr: *mut i8 = core::ptr::null_mut();
pub static mut virtual_dma_mode: i32 = 0;
pub static mut doing_pdma: i32 = 0;

#[inline]
pub unsafe fn fd_inb(base: *mut core::ffi::c_void, reg: usize) -> u8 { readb((base as usize + reg) as *mut _) }
#[inline]
pub unsafe fn fd_outb(value: u8, base: *mut core::ffi::c_void, reg: usize) { writeb(value, (base as usize + reg) as *mut _); }

pub unsafe extern "C" fn floppy_hardint(irq: i32, dev_id: *mut core::ffi::c_void, regs: *mut pt_regs) {
    let mut st: u8;
    if doing_pdma == 0 { floppy_interrupt(irq, dev_id, regs); return; }
    let mut lcount = virtual_dma_count;
    let mut lptr = virtual_dma_addr;
    while lcount != 0 {
        st = fd_inb(virtual_dma_port as *mut _, FD_STATUS as usize) & (STATUS_DMA | STATUS_READY);
        if st != (STATUS_DMA | STATUS_READY) { break; }
        if virtual_dma_mode != 0 { fd_outb(*lptr as u8, virtual_dma_port as *mut _, FD_DATA as usize); }
        else { *lptr = fd_inb(virtual_dma_port as *mut _, FD_DATA as usize) as i8; }
        lptr = lptr.add(1); lcount -= 1;
    }
    virtual_dma_count = lcount;
    virtual_dma_addr = lptr;
    st = fd_inb(virtual_dma_port as *mut _, FD_STATUS as usize);
    if st == STATUS_DMA { return; }
    if st & STATUS_DMA == 0 {
        virtual_dma_residue += virtual_dma_count;
        virtual_dma_count = 0;
        doing_pdma = 0;
        floppy_interrupt(irq, dev_id, regs);
    }
}

pub unsafe fn fd_disable_dma() {
    if can_use_virtual_dma & 1 == 0 { disable_dma(FLOPPY_DMA); }
    doing_pdma = 0; virtual_dma_residue += virtual_dma_count; virtual_dma_count = 0;
}

pub unsafe extern "C" fn vdma_request_dma(_: u32, _: *const i8) -> i32 { 0 }
pub unsafe extern "C" fn vdma_nop(_: u32) {}
pub unsafe extern "C" fn vdma_get_dma_residue(_: i32) -> i32 { virtual_dma_count + virtual_dma_residue }

pub unsafe fn fd_request_irq() -> i32 {
    if can_use_virtual_dma != 0 { request_irq(FLOPPY_IRQ, floppy_hardint, 0, b"floppy\0".as_ptr() as *const i8, core::ptr::null_mut()) }
    else { request_irq(FLOPPY_IRQ, floppy_interrupt, 0, b"floppy\0".as_ptr() as *const i8, core::ptr::null_mut()) }
}

pub unsafe fn dma_mem_alloc(size: usize) -> usize { __get_dma_pages(GFP_KERNEL, get_order(size)) }
pub unsafe fn vdma_mem_alloc(size: usize) -> usize { vmalloc(size) as usize }
pub unsafe fn _fd_dma_mem_free(addr: usize, size: usize) { if addr as u32 >= high_memory as u32 { vfree(addr as *mut _); } else { free_pages(addr, get_order(size)); } }
pub unsafe fn _fd_chose_dma_mode(addr: *mut i8, size: usize) { if can_use_virtual_dma == 2 { if addr as u32 >= high_memory as u32 || virt_to_phys(addr as *const _) >= 0x1000000 || _CROSS_64KB(addr as *const _, size, 0) { use_virtual_dma = 1; } else { use_virtual_dma = 0; } } else { use_virtual_dma = can_use_virtual_dma & 1; } }
pub unsafe fn vdma_dma_setup(addr: *mut i8, size: usize, mode: i32, io: i32) -> i32 { doing_pdma=1; virtual_dma_port=io; virtual_dma_mode=(mode == DMA_MODE_WRITE) as i32; virtual_dma_addr=addr; virtual_dma_count=size as i32; virtual_dma_residue=0; 0 }
pub unsafe fn hard_dma_setup(addr: *mut i8, size: usize, mode: i32, _: i32) -> i32 { doing_pdma=0; clear_dma_ff(FLOPPY_DMA); set_dma_mode(FLOPPY_DMA,mode); set_dma_addr(FLOPPY_DMA,virt_to_phys(addr as *const _)); set_dma_count(FLOPPY_DMA,size); enable_dma(FLOPPY_DMA); 0 }

#[repr(C)]
pub struct fd_routine_l {
    pub _request_dma: unsafe extern "C" fn(u32, *const i8) -> i32,
    pub _free_dma: unsafe extern "C" fn(u32),
    pub _get_dma_residue: unsafe extern "C" fn(i32) -> i32,
    pub _dma_mem_alloc: unsafe extern "C" fn(usize) -> usize,
    pub _dma_setup: unsafe extern "C" fn(*mut i8, usize, i32, i32) -> i32,
}

pub static mut fd_routine: [fd_routine_l; 2] = [
    fd_routine_l { _request_dma: request_dma, _free_dma: free_dma, _get_dma_residue: get_dma_residue, _dma_mem_alloc: dma_mem_alloc, _dma_setup: hard_dma_setup },
    fd_routine_l { _request_dma: vdma_request_dma, _free_dma: vdma_nop, _get_dma_residue: vdma_get_dma_residue, _dma_mem_alloc: vdma_mem_alloc, _dma_setup: vdma_dma_setup },
];

#[inline] pub unsafe fn fd_request_dma() -> i32 { (fd_routine[(use_virtual_dma & 1) as usize]._request_dma)(FLOPPY_DMA, b"floppy\0".as_ptr() as *const i8) }
#[inline] pub unsafe fn fd_free_dma() { (fd_routine[(can_use_virtual_dma & 1) as usize]._free_dma)(FLOPPY_DMA) }
#[inline] pub unsafe fn fd_enable_irq() { enable_irq(FLOPPY_IRQ) }
#[inline] pub unsafe fn fd_disable_irq() { disable_irq(FLOPPY_IRQ) }
#[inline] pub unsafe fn fd_free_irq() { free_irq(FLOPPY_IRQ, core::ptr::null_mut()) }
#[inline] pub unsafe fn fd_get_dma_residue() -> i32 { (fd_routine[(use_virtual_dma & 1) as usize]._get_dma_residue)(FLOPPY_DMA as i32) }
#[inline] pub unsafe fn fd_dma_mem_alloc(size: usize) -> usize { (fd_routine[(use_virtual_dma & 1) as usize]._dma_mem_alloc)(size) }
#[inline] pub unsafe fn fd_dma_setup(addr: *mut i8, size: usize, mode: i32, io: i32) -> i32 { (fd_routine[(use_virtual_dma & 1) as usize]._dma_setup)(addr,size,mode,io) }

pub const FDC1: i32 = 0x3f0;
pub const FDC2: i32 = -1;

pub const FLOPPY0_TYPE: i32 = 0;
pub const FLOPPY1_TYPE: i32 = 0;
pub const N_FDC: i32 = 1;
pub const N_DRIVE: i32 = 8;
pub const EXTRA_FLOPPY_PARAMS: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
