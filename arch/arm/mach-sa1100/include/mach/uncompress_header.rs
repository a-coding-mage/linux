/* SPDX-License-Identifier: GPL-2.0 */
/*
 * arch/arm/mach-sa1100/include/mach/uncompress.h
 *
 * (C) 1999 Nicolas Pitre <nico@fluxnic.net>
 *
 * Reorganised to be machine independent.
 */

// Dependency supplied by hardware.h.

macro_rules! IOMEM {
    ($x:expr) => {
        $x
    };
}

/*
 * The following code assumes the serial port has already been
 * initialized by the bootloader.  We search for the first enabled
 * port in the most probable order.  If you didn't setup a port in
 * your bootloader then nothing will appear (which might be desired).
 */

macro_rules! UART {
    ($serial_port:expr, $x:expr) => {
        (($serial_port as usize).wrapping_add($x as usize) as *mut usize)
    };
}

/// Declaration supplied by the platform dependencies.
unsafe extern "C" {
    fn barrier();
}

#[inline]
pub unsafe fn putc(c: i32) {
    let mut serial_port: usize;

    loop {
        serial_port = _Ser3UTCR0 as usize;
        if core::ptr::read_volatile(UART!(serial_port, UTCR3)) & UTCR3_TXE != 0 {
            break;
        }
        serial_port = _Ser1UTCR0 as usize;
        if core::ptr::read_volatile(UART!(serial_port, UTCR3)) & UTCR3_TXE != 0 {
            break;
        }
        serial_port = _Ser2UTCR0 as usize;
        if core::ptr::read_volatile(UART!(serial_port, UTCR3)) & UTCR3_TXE != 0 {
            break;
        }
        return;
    }

    /* wait for space in the UART's transmitter */
    while core::ptr::read_volatile(UART!(serial_port, UTSR1)) & UTSR1_TNF == 0 {
        barrier();
    }

    /* send the character out. */
    core::ptr::write_volatile(UART!(serial_port, UTDR), c as usize);
}

#[inline]
pub fn flush() {}

/*
 * Nothing to do for these
 */

macro_rules! arch_decomp_setup {
    () => {};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
