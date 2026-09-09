// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mach-sa1100/ssp.c
 *
 *  Copyright (C) 2003 Russell King.
 *
 *  Generic SSP driver.  This provides the generic core for simple
 *  IO-based SSP applications.
 */

// Linux, machine, and SSP header dependencies are supplied externally.

const TIMEOUT: i32 = 100000;

extern "C" {
    static mut Ser4SSSR: u32;
    static mut Ser4SSDR: u32;
    static mut Ser4SSCR0: u32;
    static mut Ser4SSCR1: u32;
    static mut Ser4MCCR0: u32;
    static mut PPAR: u32;

    fn printk(level: u32, fmt: *const i8, ...);
    fn cpu_relax();
    fn request_mem_region(start: usize, n: usize, name: *const i8) -> *mut core::ffi::c_void;
    fn release_mem_region(start: usize, n: usize);
    fn request_irq(
        irq: u32,
        handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32,
        flags: u32,
        name: *const i8,
        dev_id: *mut core::ffi::c_void,
    ) -> i32;
    fn free_irq(irq: u32, dev_id: *mut core::ffi::c_void);
}

#[repr(C)]
pub struct ssp_state {
    pub cr0: u32,
    pub cr1: u32,
}

const SSSR_ROR: u32 = 1 << 3;
const SSSR_TNF: u32 = 1 << 2;
const SSSR_BSY: u32 = 1 << 4;
const SSSR_RNE: u32 = 1 << 0;

// Build-provided register constants and symbols.
extern "C" {
    static __PREG_Ser4SSCR0: usize;
}

const SSCR0_SSE: u32 = 1 << 7;
const PPAR_SPR: u32 = 1 << 3;
const MCCR0_MCE: u32 = 1 << 0;
const IRQ_Ser4SSP: u32 = 0;
const IRQ_HANDLED: i32 = 1;
const IRQ_NONE: i32 = 0;
const KERN_WARNING: u32 = 0;
const ETIMEDOUT: i32 = 110;
const ENODEV: i32 = 19;
const EBUSY: i32 = 16;

unsafe extern "C" fn ssp_interrupt(_irq: i32, _dev_id: *mut core::ffi::c_void) -> i32 {
    let status = Ser4SSSR;

    if status & SSSR_ROR != 0 {
        printk(KERN_WARNING, b"SSP: receiver overrun\n\0".as_ptr() as *const i8);
    }

    Ser4SSSR = SSSR_ROR;

    if status != 0 { IRQ_HANDLED } else { IRQ_NONE }
}

/// ssp_write_word - write a word to the SSP port
/// @data: 16-bit, MSB justified data to write.
///
/// Wait for a free entry in the SSP transmit FIFO, and write a data
/// word to the SSP port.  Wait for the SSP port to start sending
/// the data.
///
/// The caller is expected to perform the necessary locking.
pub unsafe fn ssp_write_word(data: u16) -> i32 {
    let mut timeout = TIMEOUT;
    while Ser4SSSR & SSSR_TNF == 0 {
        timeout -= 1;
        if timeout == 0 { return -ETIMEDOUT; }
        cpu_relax();
    }
    Ser4SSDR = data as u32;
    timeout = TIMEOUT;
    while Ser4SSSR & SSSR_BSY == 0 {
        timeout -= 1;
        if timeout == 0 { return -ETIMEDOUT; }
        cpu_relax();
    }
    0
}

/// ssp_read_word - read a word from the SSP port
pub unsafe fn ssp_read_word(data: *mut u16) -> i32 {
    let mut timeout = TIMEOUT;
    while Ser4SSSR & SSSR_RNE == 0 {
        timeout -= 1;
        if timeout == 0 { return -ETIMEDOUT; }
        cpu_relax();
    }
    *data = Ser4SSDR as u16;
    0
}

/// ssp_flush - flush the transmit and receive FIFOs
pub unsafe fn ssp_flush() -> i32 {
    let mut timeout = TIMEOUT * 2;
    loop {
        while Ser4SSSR & SSSR_RNE != 0 {
            timeout -= 1;
            if timeout == 0 { return -ETIMEDOUT; }
            let _ = Ser4SSDR;
        }
        timeout -= 1;
        if timeout == 0 { return -ETIMEDOUT; }
        if Ser4SSSR & SSSR_BSY == 0 { break; }
    }
    0
}

/// ssp_enable - enable the SSP port
pub unsafe fn ssp_enable() { Ser4SSCR0 |= SSCR0_SSE; }

/// ssp_disable - shut down the SSP port
pub unsafe fn ssp_disable() { Ser4SSCR0 &= !SSCR0_SSE; }

/// ssp_save_state - save the SSP configuration
pub unsafe fn ssp_save_state(ssp: *mut ssp_state) {
    (*ssp).cr0 = Ser4SSCR0;
    (*ssp).cr1 = Ser4SSCR1;
    Ser4SSCR0 &= !SSCR0_SSE;
}

/// ssp_restore_state - restore a previously saved SSP configuration
pub unsafe fn ssp_restore_state(ssp: *mut ssp_state) {
    Ser4SSSR = SSSR_ROR;
    Ser4SSCR0 = (*ssp).cr0 & !SSCR0_SSE;
    Ser4SSCR1 = (*ssp).cr1;
    Ser4SSCR0 = (*ssp).cr0;
}

/// ssp_init - setup the SSP port
pub unsafe fn ssp_init() -> i32 {
    if PPAR & PPAR_SPR == 0 && Ser4MCCR0 & MCCR0_MCE != 0 { return -ENODEV; }
    if request_mem_region(__PREG_Ser4SSCR0, 0x18, b"SSP\0".as_ptr() as *const i8).is_null() {
        return -EBUSY;
    }
    Ser4SSSR = SSSR_ROR;
    let ret = request_irq(IRQ_Ser4SSP, ssp_interrupt, 0, b"SSP\0".as_ptr() as *const i8, core::ptr::null_mut());
    if ret != 0 {
        release_mem_region(__PREG_Ser4SSCR0, 0x18);
        return ret;
    }
    0
}

/// ssp_exit - undo the effects of ssp_init
pub unsafe fn ssp_exit() {
    Ser4SSCR0 &= !SSCR0_SSE;
    free_irq(IRQ_Ser4SSP, core::ptr::null_mut());
    release_mem_region(__PREG_Ser4SSCR0, 0x18);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
