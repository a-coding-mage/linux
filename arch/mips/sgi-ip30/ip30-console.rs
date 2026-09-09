// SPDX-License-Identifier: GPL-2.0

// C dependencies supplied by the surrounding kernel translation:
// linux/io.h, linux/processor.h, asm/sn/ioc3.h, and asm/setup.h.
// The following declarations intentionally refer to those external symbols.

use crate::{ioc3, ioc3_uartregs};

extern "C" {
    fn readb(addr: *const u8) -> u8;
    fn writeb(value: u8, addr: *mut u8);
    fn cpu_relax();
}

#[inline]
unsafe fn console_uart() -> *mut ioc3_uartregs {
    let ioc3: *mut ioc3 = 0x900000001f600000usize as *mut ioc3;
    &mut (*ioc3).sregs.uarta
}

pub unsafe fn prom_putchar(c: i8) {
    let uart: *mut ioc3_uartregs = console_uart();

    while (readb(&(*uart).iu_lsr as *const u8) & 0x20) == 0 {
        cpu_relax();
    }

    writeb(c as u8, &mut (*uart).iu_thr as *mut u8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
