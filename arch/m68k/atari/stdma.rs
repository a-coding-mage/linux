/*
 *  linux/arch/m68k/atari/stmda.c
 *
 *  Copyright (C) 1994 Roman Hodek
 *
 * This file is a direct Rust translation of the original implementation.
 */

use core::ffi::c_void;

/* Types, constants, and kernel primitives are supplied by the surrounding
 * kernel translation. */
pub type irq_handler_t = unsafe extern "C" fn(i32, *mut c_void) -> irqreturn_t;
pub type irqreturn_t = i32;

extern "C" {
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn request_irq(
        irq: i32,
        handler: irq_handler_t,
        flags: u32,
        name: *const u8,
        dev: irq_handler_t,
    ) -> i32;
    fn wake_up(wait: *mut c_void);
    fn pr_err(message: *const u8);
}

extern "C" {
    static IRQ_MFP_FDC: i32;
}

const IRQF_SHARED: u32 = 0x0000_0080;
const IRQ_HANDLED: irqreturn_t = 1;

static mut stdma_locked: i32 = 0;
static mut stdma_isr: Option<irq_handler_t> = None;
static mut stdma_isr_data: *mut c_void = core::ptr::null_mut();
static mut stdma_wait: () = ();

unsafe extern "C" fn stdma_int(irq: i32, _dummy: *mut c_void) -> irqreturn_t {
    if let Some(handler) = stdma_isr {
        handler(irq, stdma_isr_data);
    }
    IRQ_HANDLED
}

pub unsafe extern "C" fn stdma_try_lock(
    handler: irq_handler_t,
    data: *mut c_void,
) -> i32 {
    let mut flags: usize = 0;
    local_irq_save(&mut flags);
    if stdma_locked != 0 {
        local_irq_restore(flags);
        return 0;
    }

    stdma_locked = 1;
    stdma_isr = Some(handler);
    stdma_isr_data = data;
    local_irq_restore(flags);
    1
}

pub unsafe extern "C" fn stdma_lock(handler: irq_handler_t, data: *mut c_void) {
    /* wait_event(stdma_wait, stdma_try_lock(handler, data));
     * The wait-queue primitive is provided by the surrounding kernel. */
    while stdma_try_lock(handler, data) == 0 {
        core::hint::spin_loop();
    }
}

pub unsafe extern "C" fn stdma_release() {
    let mut flags: usize = 0;
    local_irq_save(&mut flags);

    stdma_locked = 0;
    stdma_isr = None;
    stdma_isr_data = core::ptr::null_mut();
    wake_up((&raw mut stdma_wait).cast::<c_void>());

    local_irq_restore(flags);
}

pub unsafe extern "C" fn stdma_is_locked_by(handler: irq_handler_t) -> i32 {
    let mut flags: usize = 0;
    let result: i32;
    local_irq_save(&mut flags);
    result = if stdma_locked != 0
        && stdma_isr.map_or(false, |registered| registered as usize == handler as usize)
    {
        1
    } else {
        0
    };
    local_irq_restore(flags);
    result
}

pub unsafe extern "C" fn stdma_islocked() -> i32 {
    stdma_locked
}

pub unsafe extern "C" fn stdma_init() {
    stdma_isr = None;
    if request_irq(
        IRQ_MFP_FDC,
        stdma_int,
        IRQF_SHARED,
        b"ST-DMA floppy,ACSI,IDE,Falcon-SCSI\0".as_ptr(),
        stdma_int,
    ) != 0
    {
        pr_err(b"Couldn't register ST-DMA interrupt\n\0".as_ptr());
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
