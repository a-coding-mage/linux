/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Author: Nicolas Pitre
 *
 * Moved and changed lots, Russell King
 *
 * Low level machine dependent UART functions.
 */

// Opaque declaration corresponding to `struct uart_port`.
#[repr(C)]
pub struct uart_port {
    _private: [u8; 0],
}

/*
 * This is a temporary structure for registering these
 * functions; it is intended to be discarded after boot.
 */
#[repr(C)]
pub struct sa1100_port_fns {
    pub set_mctrl:
        Option<unsafe extern "C" fn(port: *mut uart_port, mctrl: core::ffi::c_uint)>,
    pub get_mctrl:
        Option<unsafe extern "C" fn(port: *mut uart_port) -> core::ffi::c_uint>,
    pub pm: Option<unsafe extern "C" fn(
        port: *mut uart_port,
        state: core::ffi::c_uint,
        oldstate: core::ffi::c_uint,
    )>,
    pub set_wake:
        Option<unsafe extern "C" fn(port: *mut uart_port, state: core::ffi::c_uint) -> core::ffi::c_int>,
}

// Preserved from the CONFIG_SERIAL_SA1100 build-time condition.
#[cfg(CONFIG_SERIAL_SA1100)]
extern "C" {
    pub fn sa1100_register_uart_fns(fns: *mut sa1100_port_fns);
    pub fn sa1100_register_uart(idx: core::ffi::c_int, port: core::ffi::c_int);
}

// Fallback definitions when CONFIG_SERIAL_SA1100 is not enabled.
#[cfg(not(CONFIG_SERIAL_SA1100))]
#[inline]
pub unsafe fn sa1100_register_uart_fns(_fns: *mut sa1100_port_fns) {}

#[cfg(not(CONFIG_SERIAL_SA1100))]
#[inline]
pub unsafe fn sa1100_register_uart(_idx: core::ffi::c_int, _port: core::ffi::c_int) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
