/* SPDX-License-Identifier: GPL-2.0 */
/* asm/floppy.h: Sparc specific parts of the Floppy driver. */

// Dependencies supplied by the surrounding kernel translation unit are intentionally external.

#[repr(C)]
pub struct sun_flpy_controller {
    pub status_82072: core::cell::UnsafeCell<u8>,
    pub data_82072: core::cell::UnsafeCell<u8>,
    pub dor_82077: core::cell::UnsafeCell<u8>,
    pub tapectl_82077: core::cell::UnsafeCell<u8>,
    pub status_82077: core::cell::UnsafeCell<u8>,
    pub data_82077: core::cell::UnsafeCell<u8>,
    pub ___unused: core::cell::UnsafeCell<u8>,
    pub dir_82077: core::cell::UnsafeCell<u8>,
}

// C field aliases: dcr_82072/status1_82077 = status_82072,
// status2_82077 = data_82072, drs_82077 = status_82077, dcr_82077 = dir_82077.

#[repr(C)]
pub struct sun_floppy_ops {
    pub fd_inb: Option<unsafe extern "C" fn(port: i32) -> u8>,
    pub fd_outb: Option<unsafe extern "C" fn(value: u8, port: i32)>,
}

pub static mut sun_fdc: *mut sun_flpy_controller = core::ptr::null_mut();
pub static mut sun_fdops: sun_floppy_ops = sun_floppy_ops { fd_inb: None, fd_outb: None };

// release_region/request_region are deliberately no-ops on Sparc.
#[inline] pub unsafe fn release_region(_x: i32, _y: i32) {}
#[inline] pub unsafe fn request_region(_x: i32, _y: i32, _z: i32) -> i32 { 1 }

pub const FLOPPY0_TYPE: i32 = 4;
pub const FLOPPY1_TYPE: i32 = 0;
pub const N_FDC: i32 = 1;
pub const N_DRIVE: i32 = 8;

extern "C" {
    static mut doing_pdma: i32;
    static mut pdma_base: *mut core::ffi::c_void;
    static mut pdma_vaddr: *mut i8;
    static mut pdma_size: i32;
    static mut pdma_areasize: i32;
    static mut use_virtual_dma: i32;
    static mut FLOPPY_IRQ: u32;
    static mut fdc_status: *mut u8;
    static mut allowed_drive_mask: u32;
    static mut floppy_interrupt: unsafe extern "C" fn();
    fn udelay(usecs: u32);
    fn printk(fmt: *const i8, ...) -> i32;
    fn panic(fmt: *const i8) -> !;
    fn sparc_floppy_request_irq(irq: u32, handler: unsafe extern "C" fn()) -> i32;
    fn set_dor(a: u8, b: u8, c: u8);
}

extern "C" {
    static mut dma_spin_lock: core::ffi::c_void;
}

pub const fn get_dma_residue(_x: i32) -> i32 { 0 }

unsafe fn sun_set_dor(value: u8, fdc_82077: i32) {
    if fdc_82077 != 0 { (*sun_fdc).dor_82077.get().write(value); }
}
unsafe fn sun_read_dir() -> u8 { (*sun_fdc).dir_82077.get().read() }

unsafe fn sun_82072_fd_inb(port: i32) -> u8 {
    udelay(5);
    match port {
        FD_STATUS => (*sun_fdc).status_82072.get().read() & !STATUS_DMA as u8,
        FD_DATA => (*sun_fdc).data_82072.get().read(),
        FD_DIR => sun_read_dir(),
        _ => { printk(b"floppy: Asked to read unknown port %d\0".as_ptr() as *const i8, port); panic(b"floppy: Port bolixed.\0".as_ptr() as *const i8) }
    }
}

unsafe fn sun_82072_fd_outb(value: u8, port: i32) {
    udelay(5);
    match port {
        FD_DOR => sun_set_dor(value, 0),
        FD_DATA => (*sun_fdc).data_82072.get().write(value),
        FD_DCR => (*sun_fdc).dir_82077.get().write(value),
        FD_DSR => (*sun_fdc).status_82072.get().write(value),
        _ => { printk(b"floppy: Asked to write to unknown port %d\0".as_ptr() as *const i8, port); panic(b"floppy: Port bolixed.\0".as_ptr() as *const i8) }
    }
}

unsafe fn sun_82077_fd_inb(port: i32) -> u8 {
    udelay(5);
    match port {
        FD_SRA => (*sun_fdc).status_82072.get().read(), FD_SRB => (*sun_fdc).data_82072.get().read(),
        FD_DOR => (*sun_fdc).dor_82077.get().read(), FD_TDR => (*sun_fdc).tapectl_82077.get().read(),
        FD_STATUS => (*sun_fdc).status_82077.get().read() & !STATUS_DMA as u8,
        FD_DATA => (*sun_fdc).data_82077.get().read(), FD_DIR => sun_read_dir(),
        _ => { printk(b"floppy: Asked to read unknown port %d\0".as_ptr() as *const i8, port); panic(b"floppy: Port bolixed.\0".as_ptr() as *const i8) }
    }
}

unsafe fn sun_82077_fd_outb(value: u8, port: i32) {
    udelay(5);
    match port {
        FD_DOR => sun_set_dor(value, 1), FD_DATA => (*sun_fdc).data_82077.get().write(value),
        FD_DCR => (*sun_fdc).dir_82077.get().write(value), FD_DSR => (*sun_fdc).status_82077.get().write(value),
        FD_TDR => (*sun_fdc).tapectl_82077.get().write(value),
        _ => { printk(b"floppy: Asked to write to unknown port %d\0".as_ptr() as *const i8, port); panic(b"floppy: Port bolixed.\0".as_ptr() as *const i8) }
    }
}

#[inline] pub unsafe fn virtual_dma_init() {}
#[inline] pub unsafe fn sun_fd_disable_dma() { doing_pdma = 0; pdma_base = core::ptr::null_mut(); }
#[inline] pub unsafe fn sun_fd_set_dma_mode(mode: i32) { match mode { DMA_MODE_READ => doing_pdma=1, DMA_MODE_WRITE => doing_pdma=2, _ => panic(b"floppy: Giving up...\0".as_ptr() as *const i8) } }
#[inline] pub unsafe fn sun_fd_set_dma_addr(buffer: *mut i8) { pdma_vaddr = buffer; }
#[inline] pub unsafe fn sun_fd_set_dma_count(length: i32) { pdma_size = length; }
#[inline] pub unsafe fn sun_fd_enable_dma() { pdma_base = pdma_vaddr as *mut core::ffi::c_void; pdma_areasize = pdma_size; }

unsafe fn sun_fd_request_irq() -> i32 {
    static mut once: i32 = 0;
    if once == 0 { once = 1; sparc_floppy_request_irq(FLOPPY_IRQ, floppy_interrupt) } else { 0 }
}

unsafe fn sparc_eject() -> i32 { set_dor(0, 0xff, 0x90); udelay(500); set_dor(0, 0x6f, 0); udelay(500); 0 }

// The original header's PROM probing initializer is retained as an external kernel-provided routine.
extern "C" { fn sun_floppy_init() -> i32; }

// fd_inb/fd_outb and DMA/IRQ macros dispatch to sun_fdops and the routines above.
pub const EXTRA_FLOPPY_PARAMS: () = ();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
