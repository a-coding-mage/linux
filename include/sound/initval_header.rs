/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *  Init values for soundcard modules
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

pub const SNDRV_AUTO_PORT: i32 = 1;
pub const SNDRV_AUTO_IRQ: u32 = 0xffff;
pub const SNDRV_AUTO_DMA: u32 = 0xffff;
pub const SNDRV_AUTO_DMA_SIZE: i32 = 0x7fffffff;

pub const SNDRV_DEFAULT_IDX1: i32 = -1;
pub const SNDRV_DEFAULT_STR1: *const core::ffi::c_char = core::ptr::null();
pub const SNDRV_DEFAULT_ENABLE1: i32 = 1;
pub const SNDRV_DEFAULT_PORT1: i32 = SNDRV_AUTO_PORT;
pub const SNDRV_DEFAULT_IRQ1: u32 = SNDRV_AUTO_IRQ;
pub const SNDRV_DEFAULT_DMA1: u32 = SNDRV_AUTO_DMA;
pub const SNDRV_DEFAULT_DMA_SIZE1: i32 = SNDRV_AUTO_DMA_SIZE;
pub const SNDRV_DEFAULT_PTR1: *const core::ffi::c_char = SNDRV_DEFAULT_STR1;

/* Array initializer macros; SNDRV_CARDS is supplied by the including translation unit. */
#[macro_export]
macro_rules! SNDRV_DEFAULT_IDX { ($n:expr) => { [-1i32; $n] }; }
#[macro_export]
macro_rules! SNDRV_DEFAULT_STR { ($n:expr) => { [core::ptr::null::<core::ffi::c_char>(); $n] }; }
#[macro_export]
macro_rules! SNDRV_DEFAULT_ENABLE { ($n:expr) => {{ let mut a = [0i32; $n]; if $n > 0 { a[0] = 1; } a }}; }
#[macro_export]
macro_rules! SNDRV_DEFAULT_ENABLE_PNP { ($n:expr) => { [1i32; $n] }; }
/* CONFIG_PNP selects SNDRV_DEFAULT_ENABLE_PNP; otherwise SNDRV_DEFAULT_ENABLE. */
#[cfg(feature = "CONFIG_PNP")]
#[macro_export]
macro_rules! SNDRV_DEFAULT_ENABLE_ISAPNP { ($n:expr) => { $crate::SNDRV_DEFAULT_ENABLE_PNP!($n) }; }
#[cfg(not(feature = "CONFIG_PNP"))]
#[macro_export]
macro_rules! SNDRV_DEFAULT_ENABLE_ISAPNP { ($n:expr) => { $crate::SNDRV_DEFAULT_ENABLE!($n) }; }
#[macro_export]
macro_rules! SNDRV_DEFAULT_PORT { ($n:expr) => { [SNDRV_AUTO_PORT; $n] }; }
#[macro_export]
macro_rules! SNDRV_DEFAULT_IRQ { ($n:expr) => { [SNDRV_AUTO_IRQ; $n] }; }
#[macro_export]
macro_rules! SNDRV_DEFAULT_DMA { ($n:expr) => { [SNDRV_AUTO_DMA; $n] }; }
#[macro_export]
macro_rules! SNDRV_DEFAULT_DMA_SIZE { ($n:expr) => { [SNDRV_AUTO_DMA_SIZE; $n] }; }
#[macro_export]
macro_rules! SNDRV_DEFAULT_PTR { ($n:expr) => { $crate::SNDRV_DEFAULT_STR!($n) }; }

#[cfg(feature = "SNDRV_LEGACY_FIND_FREE_IOPORT")]
extern "C" {
    fn request_region(port: i64, size: i64, name: *const core::ffi::c_char) -> *mut core::ffi::c_void;
    fn release_region(port: i64, size: i64);
}

#[cfg(feature = "SNDRV_LEGACY_FIND_FREE_IOPORT")]
pub unsafe fn snd_legacy_find_free_ioport(port_table: *const i64, size: i64) -> i64 {
    let mut port_table = port_table;
    while *port_table != -1 {
        if !request_region(*port_table, size, b"ALSA test\0".as_ptr() as *const core::ffi::c_char).is_null() {
            release_region(*port_table, size);
            return *port_table;
        }
        port_table = port_table.add(1);
    }
    -1
}

#[cfg(feature = "SNDRV_LEGACY_FIND_FREE_IRQ")]
pub unsafe extern "C" fn snd_legacy_empty_irq_handler(_irq: i32, _dev_id: *mut core::ffi::c_void) -> i32 { 1 }

#[cfg(feature = "SNDRV_LEGACY_FIND_FREE_IRQ")]
extern "C" {
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32, flags: u64, name: *const core::ffi::c_char, dev_id: *mut core::ffi::c_void) -> i32;
    fn free_irq(irq: i32, dev_id: *mut core::ffi::c_void);
}

#[cfg(feature = "SNDRV_LEGACY_FIND_FREE_IRQ")]
pub unsafe fn snd_legacy_find_free_irq(irq_table: *const i32) -> i32 {
    let mut irq_table = irq_table;
    while *irq_table != -1 {
        if request_irq(*irq_table, snd_legacy_empty_irq_handler, 0x80, b"ALSA Test IRQ\0".as_ptr() as *const core::ffi::c_char, irq_table as *mut core::ffi::c_void) == 0 {
            free_irq(*irq_table, irq_table as *mut core::ffi::c_void);
            return *irq_table;
        }
        irq_table = irq_table.add(1);
    }
    -1
}

#[cfg(feature = "SNDRV_LEGACY_FIND_FREE_DMA")]
extern "C" { fn request_dma(dma: i32, name: *const core::ffi::c_char) -> i32; fn free_dma(dma: i32); }

#[cfg(feature = "SNDRV_LEGACY_FIND_FREE_DMA")]
pub unsafe fn snd_legacy_find_free_dma(dma_table: *const i32) -> i32 {
    let mut dma_table = dma_table;
    while *dma_table != -1 {
        if request_dma(*dma_table, b"ALSA Test DMA\0".as_ptr() as *const core::ffi::c_char) == 0 {
            free_dma(*dma_table);
            return *dma_table;
        }
        dma_table = dma_table.add(1);
    }
    -1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
