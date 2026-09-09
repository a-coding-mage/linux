/* SPDX-License-Identifier: GPL-2.0 */
/*
 * include/asm-m68k/serial.h
 *
 * currently this seems useful only for a Q40,
 * it's an almost exact copy of ../asm-alpha/serial.h
 */

/*
 * This assumes you have a 1.8432 MHz clock for your UART.
 *
 * It'd be nice if someone built a serial card with a 24.576 MHz
 * clock, since the 16550A is capable of handling a top speed of 1.5
 * megabits/second; but this requires the faster clock.
 */
pub const BASE_BAUD: i32 = 1_843_200 / 16;

/* Standard COM flags (except for COM4, because of the 8514 problem). */
/* The CONFIG_SERIAL_8250_DETECT_IRQ build-time condition is preserved here. */
#[cfg(feature = "CONFIG_SERIAL_8250_DETECT_IRQ")]
pub const STD_COM_FLAGS: _ = UPF_BOOT_AUTOCONF | UPF_SKIP_TEST | UPF_AUTO_IRQ;
#[cfg(feature = "CONFIG_SERIAL_8250_DETECT_IRQ")]
pub const STD_COM4_FLAGS: _ = UPF_BOOT_AUTOCONF | UPF_AUTO_IRQ;

#[cfg(not(feature = "CONFIG_SERIAL_8250_DETECT_IRQ"))]
pub const STD_COM_FLAGS: _ = UPF_BOOT_AUTOCONF | UPF_SKIP_TEST;
#[cfg(not(feature = "CONFIG_SERIAL_8250_DETECT_IRQ"))]
pub const STD_COM4_FLAGS: _ = UPF_BOOT_AUTOCONF;

/*
 * The CONFIG_ISA build-time condition is preserved here.  This macro
 * expands to the same four serial-port initializers as SERIAL_PORT_DFNS.
 */
#[cfg(feature = "CONFIG_ISA")]
macro_rules! SERIAL_PORT_DFNS {
    () => {
        (0, BASE_BAUD, 0x3f8, 4, STD_COM_FLAGS),
        (0, BASE_BAUD, 0x2f8, 3, STD_COM_FLAGS),
        (0, BASE_BAUD, 0x3e8, 4, STD_COM_FLAGS),
        (0, BASE_BAUD, 0x2e8, 3, STD_COM4_FLAGS),
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
