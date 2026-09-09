/* SPDX-License-Identifier: GPL-2.0 */
/*
 * include/asm-alpha/serial.h
 */

/*
 * This assumes you have a 1.8432 MHz clock for your UART.
 *
 * It'd be nice if someone built a serial card with a 24.576 MHz
 * clock, since the 16550A is capable of handling a top speed of 1.5
 * megabits/second; but this requires the faster clock.
 */
pub const BASE_BAUD: u32 = 1843200 / 16;

/* Standard COM flags (except for COM4, because of the 8514 problem) */
/*
 * CONFIG_SERIAL_8250_DETECT_IRQ is a build-time condition from the C
 * header; select the corresponding constants in the surrounding build.
 */
#[cfg(CONFIG_SERIAL_8250_DETECT_IRQ)]
pub const STD_COM_FLAGS: u32 = UPF_BOOT_AUTOCONF | UPF_SKIP_TEST | UPF_AUTO_IRQ;
#[cfg(CONFIG_SERIAL_8250_DETECT_IRQ)]
pub const STD_COM4_FLAGS: u32 = UPF_BOOT_AUTOCONF | UPF_AUTO_IRQ;

#[cfg(not(CONFIG_SERIAL_8250_DETECT_IRQ))]
pub const STD_COM_FLAGS: u32 = UPF_BOOT_AUTOCONF | UPF_SKIP_TEST;
#[cfg(not(CONFIG_SERIAL_8250_DETECT_IRQ))]
pub const STD_COM4_FLAGS: u32 = UPF_BOOT_AUTOCONF;

/* UART CLK   PORT IRQ     FLAGS */
#[macro_export]
macro_rules! SERIAL_PORT_DFNS {
    () => {
        [
            (0, $crate::BASE_BAUD, 0x3F8, 4, $crate::STD_COM_FLAGS),
            (0, $crate::BASE_BAUD, 0x2F8, 3, $crate::STD_COM_FLAGS),
            (0, $crate::BASE_BAUD, 0x3E8, 4, $crate::STD_COM_FLAGS),
            (0, $crate::BASE_BAUD, 0x2E8, 3, $crate::STD_COM4_FLAGS),
        ]
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
