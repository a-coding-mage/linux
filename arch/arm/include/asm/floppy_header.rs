/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/floppy.h
 *
 *  Copyright (C) 1996-2000 Russell King
 *
 *  Note that we don't touch FLOPPY_DMA nor FLOPPY_IRQ here
 */

// The following constants, functions, types, and globals are supplied by
// other translated headers or by the surrounding kernel implementation.

#[allow(non_upper_case_globals)]
pub static mut floppy_selects: [u8; 4] = [0x10, 0x21, 0x23, 0x33];

#[macro_export]
macro_rules! fd_outb {
    ($val:expr, $base:expr, $reg:expr) => {{
        let mut new_val: i32 = ($val) as i32;
        if ($reg) == FD_DOR {
            if new_val & 0xf0 != 0 {
                new_val = (new_val & 0x0c)
                    | (unsafe { $crate::floppy_selects[(new_val & 3) as usize] } as i32);
            } else {
                new_val &= 0x0c;
            }
        }
        unsafe { outb(new_val as u8, (($base) as usize + ($reg) as usize) as u16) };
    }};
}

#[macro_export]
macro_rules! fd_inb {
    ($base:expr, $reg:expr) => {{
        unsafe { inb((($base) as usize + ($reg) as usize) as u16) }
    }};
}

#[macro_export]
macro_rules! fd_request_irq {
    () => {{ unsafe { request_irq(IRQ_FLOPPYDISK, floppy_interrupt, 0, "floppy", core::ptr::null_mut()) } }};
}

#[macro_export]
macro_rules! fd_free_irq {
    () => {{ unsafe { free_irq(IRQ_FLOPPYDISK, core::ptr::null_mut()) } }};
}

#[macro_export]
macro_rules! fd_disable_irq {
    () => {{ unsafe { disable_irq(IRQ_FLOPPYDISK) } }};
}

#[macro_export]
macro_rules! fd_enable_irq {
    () => {{ unsafe { enable_irq(IRQ_FLOPPYDISK) } }};
}

#[inline]
pub unsafe fn fd_dma_setup(
    data: *mut core::ffi::c_void,
    length: u32,
    mode: u32,
    addr: usize,
) -> i32 {
    set_dma_mode(DMA_FLOPPY, mode);
    __set_dma_addr(DMA_FLOPPY, data);
    set_dma_count(DMA_FLOPPY, length);
    virtual_dma_port = addr;
    enable_dma(DMA_FLOPPY);
    0
}

#[macro_export]
macro_rules! fd_request_dma {
    () => {{ unsafe { request_dma(DMA_FLOPPY, "floppy") } }};
}

#[macro_export]
macro_rules! fd_free_dma {
    () => {{ unsafe { free_dma(DMA_FLOPPY) } }};
}

#[macro_export]
macro_rules! fd_disable_dma {
    () => {{ unsafe { disable_dma(DMA_FLOPPY) } }};
}

// need to clean up dma.h
pub const DMA_FLOPPYDISK: i32 = DMA_FLOPPY;

pub const FDC1: i32 = 0x3f0;
pub const FLOPPY0_TYPE: i32 = 4;
pub const FLOPPY1_TYPE: i32 = 4;
pub const N_FDC: i32 = 1;
pub const N_DRIVE: i32 = 4;

unsafe extern "C" {
    fn swap<T>(a: *mut T, b: *mut T);
}

pub unsafe extern "C" fn driveswap(_ints: *mut i32, _dummy: i32, _dummy2: i32) {
    swap(
        core::ptr::addr_of_mut!(floppy_selects[0]),
        core::ptr::addr_of_mut!(floppy_selects[1]),
    );
}

// Corresponds to:
// #define EXTRA_FLOPPY_PARAMS ,{ "driveswap", &driveswap, NULL, 0, 0 }
#[macro_export]
macro_rules! EXTRA_FLOPPY_PARAMS {
    () => { , { "driveswap", $crate::driveswap, core::ptr::null(), 0, 0 } };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
