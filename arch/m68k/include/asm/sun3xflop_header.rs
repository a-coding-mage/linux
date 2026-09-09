/* SPDX-License-Identifier: GPL-2.0 */
/* sun3xflop.h: Sun3/80 specific parts of the floppy driver.
 *
 * Derived partially from asm-sparc/floppy.h, which is:
 *     Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 *
 * Sun3x version 2/4/2000 Sam Creasey (sammy@sammy.net)
 */

// Dependencies supplied by the surrounding kernel translation unit.

/* default interrupt vector */
pub const SUN3X_FDC_IRQ: i32 = 0x40;

/* some constants */
pub const FCR_TC: u8 = 0x1;
pub const FCR_EJECT: u8 = 0x2;
pub const FCR_MTRON: u8 = 0x4;
pub const FCR_DSEL1: u8 = 0x8;
pub const FCR_DSEL0: u8 = 0x10;

/* We don't need no stinkin' I/O port allocation crap. */
#[inline]
pub fn release_region(_x: i32, _y: i32) {}

#[inline]
pub const fn request_region(_x: i32, _y: i32, _z: *const u8) -> i32 { 1 }

#[repr(C)]
pub struct sun3xflop_private {
    pub status_r: *mut u8,
    pub data_r: *mut u8,
    pub fcr_r: *mut u8,
    pub fvr_r: *mut u8,
    pub fcr: u8,
}

pub static mut sun3x_fdc: sun3xflop_private = sun3xflop_private {
    status_r: core::ptr::null_mut(),
    data_r: core::ptr::null_mut(),
    fcr_r: core::ptr::null_mut(),
    fvr_r: core::ptr::null_mut(),
    fcr: 0,
};

/* Super paranoid... */

/* Routines unique to each controller type on a Sun. */
pub unsafe fn sun3x_82072_fd_inb(port: i32) -> u8 {
    static mut once: i32 = 0;
    match port & 7 {
        4 => { /* FD_STATUS */
            core::ptr::read_volatile(sun3x_fdc.status_r) & !STATUS_DMA
        }
        5 => { /* FD_DATA */
            core::ptr::read_volatile(sun3x_fdc.data_r)
        }
        7 => { /* FD_DIR */
            /* ugly hack, I can't find a way to actually detect the disk */
            if once == 0 {
                once = 1;
                0x80
            } else {
                0
            }
        }
        _ => {
            pr_crit!("floppy: Asked to read unknown port %d\n", port);
            panic!("floppy: Port bolixed.");
        }
    }
}

pub unsafe fn sun3x_82072_fd_outb(value: u8, port: i32) {
    match port & 7 {
        2 => { /* FD_DOR */
            /* Oh geese, 82072 on the Sun has no DOR register,
             * so we make do with taunting the FCR.
             *
             * ASSUMPTIONS:  There will only ever be one floppy
             *               drive attached to a Sun controller
             *               and it will be at drive zero.
             */
            let mut fcr = sun3x_fdc.fcr;
            if value & 0x10 != 0 {
                fcr |= FCR_DSEL0 | FCR_MTRON;
            } else {
                fcr &= !(FCR_DSEL0 | FCR_MTRON);
            }
            if fcr != sun3x_fdc.fcr {
                core::ptr::write_volatile(sun3x_fdc.fcr_r, fcr);
                sun3x_fdc.fcr = fcr;
            }
        }
        5 => { /* FD_DATA */
            core::ptr::write_volatile(sun3x_fdc.data_r, value);
        }
        7 => { /* FD_DCR */
            core::ptr::write_volatile(sun3x_fdc.status_r, value);
        }
        4 => { /* FD_STATUS */
            core::ptr::write_volatile(sun3x_fdc.status_r, value);
        }
        _ => {
            pr_crit!("floppy: Asked to write to unknown port %d\n", port);
            panic!("floppy: Port bolixed.");
        }
    }
}

pub unsafe extern "C" fn sun3xflop_hardint(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let mut st: u8;

    if !doing_pdma {
        floppy_interrupt(irq, dev_id);
        return IRQ_HANDLED;
    }

    let mut lcount = virtual_dma_count;
    let mut lptr = virtual_dma_addr;
    while lcount != 0 {
        st = core::ptr::read_volatile(sun3x_fdc.status_r);
        if st & 0x80 == 0 {
            virtual_dma_count = lcount;
            virtual_dma_addr = lptr;
            return IRQ_HANDLED;
        }
        if st & 0x20 == 0 { break; }
        if virtual_dma_mode {
            core::ptr::write_volatile(sun3x_fdc.data_r, *lptr);
        } else {
            *lptr = core::ptr::read_volatile(sun3x_fdc.data_r);
        }
        lcount -= 1;
        lptr = lptr.add(1);
    }
    virtual_dma_count = lcount;
    virtual_dma_addr = lptr;
    st = core::ptr::read_volatile(sun3x_fdc.status_r);
    if st == 0x20 { return IRQ_HANDLED; }
    if st & 0x20 == 0 {
        virtual_dma_residue += virtual_dma_count;
        virtual_dma_count = 0;
        doing_pdma = 0;
        floppy_interrupt(irq, dev_id);
        return IRQ_HANDLED;
    }
    IRQ_HANDLED
}

pub unsafe fn sun3xflop_request_irq() -> i32 {
    static mut once: i32 = 0;
    if once == 0 {
        once = 1;
        let error = request_irq(FLOPPY_IRQ, sun3xflop_hardint, 0, "floppy", core::ptr::null_mut());
        if error == 0 { 0 } else { -1 }
    } else { 0 }
}

pub unsafe fn floppy_set_flags(ints: *mut i32, param: i32, param2: i32);

pub unsafe fn sun3xflop_init() -> i32 {
    if FLOPPY_IRQ < 0x40 { FLOPPY_IRQ = SUN3X_FDC_IRQ; }
    sun3x_fdc.status_r = SUN3X_FDC as *mut u8;
    sun3x_fdc.data_r = (SUN3X_FDC + 1) as *mut u8;
    sun3x_fdc.fcr_r = SUN3X_FDC_FCR as *mut u8;
    sun3x_fdc.fvr_r = SUN3X_FDC_FVR as *mut u8;
    sun3x_fdc.fcr = 0;
    if core::ptr::read_volatile(sun3x_fdc.status_r) == 0xff { return -1; }
    core::ptr::write_volatile(sun3x_fdc.fvr_r, FLOPPY_IRQ as u8);
    core::ptr::write_volatile(sun3x_fdc.fcr_r, FCR_TC);
    udelay(10);
    core::ptr::write_volatile(sun3x_fdc.fcr_r, 0);
    floppy_set_flags(core::ptr::null_mut(), 1, FD_BROKEN_DCL);
    allowed_drive_mask = 0x01;
    SUN3X_FDC as i32
}

/* I'm not precisely sure this eject routine works */
pub unsafe fn sun3x_eject() -> i32 {
    if MACH_IS_SUN3X {
        sun3x_fdc.fcr |= FCR_DSEL0 | FCR_EJECT;
        core::ptr::write_volatile(sun3x_fdc.fcr_r, sun3x_fdc.fcr);
        udelay(10);
        sun3x_fdc.fcr &= !(FCR_DSEL0 | FCR_EJECT);
        core::ptr::write_volatile(sun3x_fdc.fcr_r, sun3x_fdc.fcr);
    }
    0
}

#[inline]
pub unsafe fn fd_eject(_drive: i32) -> i32 { sun3x_eject() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
