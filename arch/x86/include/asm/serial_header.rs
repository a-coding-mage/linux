/* SPDX-License-Identifier: GPL-2.0 */

// This assumes a 1.8432 MHz clock for the UART.
//
// It would be nice if someone built a serial card with a 24.576 MHz clock,
// since the 16550A is capable of handling a top speed of 1.5 megabits/second;
// but this requires a faster clock.
pub const BASE_BAUD: i32 = 1_843_200 / 16;

// Standard COM flags (except for COM4, because of the 8514 problem).
// The CONFIG_SERIAL_8250_DETECT_IRQ build condition is supplied externally.
#[cfg(feature = "CONFIG_SERIAL_8250_DETECT_IRQ")]
pub const STD_COMX_FLAGS: u32 = UPF_BOOT_AUTOCONF | UPF_SKIP_TEST | UPF_AUTO_IRQ;
#[cfg(feature = "CONFIG_SERIAL_8250_DETECT_IRQ")]
pub const STD_COM4_FLAGS: u32 = UPF_BOOT_AUTOCONF | UPF_AUTO_IRQ;

#[cfg(not(feature = "CONFIG_SERIAL_8250_DETECT_IRQ"))]
pub const STD_COMX_FLAGS: u32 = UPF_BOOT_AUTOCONF | UPF_SKIP_TEST;
#[cfg(not(feature = "CONFIG_SERIAL_8250_DETECT_IRQ"))]
pub const STD_COM4_FLAGS: u32 = UPF_BOOT_AUTOCONF;

// The flag constants above are supplied by the serial-port implementation.
// This macro corresponds to the C SERIAL_PORT_DFNS initializer list.  The
// caller supplies the Rust type corresponding to the C designated initializer.
#[macro_export]
macro_rules! SERIAL_PORT_DFNS {
    ($port_type:path) => {
        [
            $port_type { uart: 0, iobase: BASE_BAUD, ioport: 0x3F8, irq: 4, flags: STD_COMX_FLAGS },
            $port_type { uart: 0, iobase: BASE_BAUD, ioport: 0x2F8, irq: 3, flags: STD_COMX_FLAGS },
            $port_type { uart: 0, iobase: BASE_BAUD, ioport: 0x3E8, irq: 4, flags: STD_COMX_FLAGS },
            $port_type { uart: 0, iobase: BASE_BAUD, ioport: 0x2E8, irq: 3, flags: STD_COM4_FLAGS },
        ]
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
