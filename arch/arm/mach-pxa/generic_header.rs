/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  linux/arch/arm/mach-pxa/generic.h
 *
 * Author: Nicolas Pitre
 * Copyright: MontaVista Software Inc.
 */

// Dependency corresponding to <linux/reboot.h>.

#[repr(C)]
pub struct irq_data {
    _private: [u8; 0],
}

#[allow(improper_ctypes)]
extern "C" {
    pub fn pxa_dt_irq_init(
        f: Option<unsafe extern "C" fn(*mut irq_data, ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    );
    pub fn pxa_map_io();
    pub fn pxa_timer_init();
}

#[macro_export]
macro_rules! SET_BANK {
    ($mi:expr, $nr:expr, $start:expr, $size:expr) => {{
        unsafe {
            (*$mi).bank[$nr].start = $start;
            (*$mi).bank[$nr].size = $size;
        }
    }};
}

#[macro_export]
macro_rules! ARRAY_AND_SIZE {
    ($x:expr) => {
        ($x, $x.len())
    };
}

// pxa25x_handle_irq is a preprocessor alias in the C header.
pub use icip_handle_irq as pxa25x_handle_irq;

extern "C" {
    pub fn pxa25x_init_irq();
    pub fn pxa25x_map_io();
    pub fn pxa26x_init_irq();
}

// pxa27x_handle_irq is a preprocessor alias in the C header.
pub use ichp_handle_irq as pxa27x_handle_irq;

extern "C" {
    pub fn pxa27x_init_irq();
    pub fn pxa27x_map_io();
}

// pxa3xx_handle_irq is a preprocessor alias in the C header.
pub use ichp_handle_irq as pxa3xx_handle_irq;

extern "C" {
    pub fn pxa3xx_init_irq();
    pub fn pxa3xx_map_io();
}

#[allow(improper_ctypes)]
extern "C" {
    pub static mut pxa_irq_syscore: syscore;
    pub static mut pxa2xx_mfp_syscore: syscore;
    pub static mut pxa3xx_mfp_syscore: syscore;

    pub fn pxa_set_ffuart_info(info: *mut ::core::ffi::c_void);
    pub fn pxa_set_btuart_info(info: *mut ::core::ffi::c_void);
    pub fn pxa_set_stuart_info(info: *mut ::core::ffi::c_void);
    pub fn pxa_set_hwuart_info(info: *mut ::core::ffi::c_void);

    pub fn pxa_restart(mode: reboot_mode, cmd: *const ::core::ffi::c_char);
}

// External types supplied by the kernel dependencies.
#[allow(non_camel_case_types)]
pub enum reboot_mode {}

#[allow(non_camel_case_types)]
pub enum syscore {}

// When CONFIG_PXA25x or CONFIG_PXA27x is enabled, this is supplied externally.
#[cfg(any(feature = "CONFIG_PXA25x", feature = "CONFIG_PXA27x"))]
extern "C" {
    pub fn pxa2xx_clear_reset_status(mask: ::core::ffi::c_uint);
}

// Otherwise the C header provides an empty static inline function.
#[cfg(not(any(feature = "CONFIG_PXA25x", feature = "CONFIG_PXA27x")))]
#[inline]
pub unsafe fn pxa2xx_clear_reset_status(_mask: ::core::ffi::c_uint) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
