/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation unit:
// linux/serial.h, linux/serial_reg.h, linux/serial_core.h, and spk_priv.h.

/*
 * this is cut&paste from 8250.h. Get rid of the structure, the definitions
 * and this whole broken driver.
 */
#[repr(C)]
pub struct old_serial_port {
    pub uart: ::core::ffi::c_uint, /* unused */
    pub baud_base: ::core::ffi::c_uint,
    pub port: ::core::ffi::c_uint,
    pub irq: ::core::ffi::c_uint,
    pub flags: upf_t, /* unused */
}

/* countdown values for serial timeouts in us */
pub const SPK_SERIAL_TIMEOUT: _ = SPK_SYNTH_TIMEOUT;
/* countdown values transmitter/dsr timeouts in us */
pub const SPK_XMITR_TIMEOUT: ::core::ffi::c_uint = 100000;
/* countdown values cts timeouts in us */
pub const SPK_CTS_TIMEOUT: ::core::ffi::c_uint = 100000;
/* check ttyS0 ... ttyS3 */
pub const SPK_LO_TTY: ::core::ffi::c_uint = 0;
pub const SPK_HI_TTY: ::core::ffi::c_uint = 3;
/* # of timeouts permitted before disable */
pub const NUM_DISABLE_TIMEOUTS: ::core::ffi::c_uint = 3;
/* buffer timeout in ms */
pub const SPK_TIMEOUT: ::core::ffi::c_uint = 100;

#[macro_export]
macro_rules! spk_serial_tx_busy {
    () => {
        (!uart_lsr_tx_empty(unsafe {
            inb(speakup_info.port_tts + UART_LSR)
        }))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
