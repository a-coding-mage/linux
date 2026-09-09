/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001, 2002 Ralf Baechle
 */

// Dependencies supplied by the corresponding platform headers:
// asm/page.h, asm/setup.h, asm/sn/addrs.h, asm/sn/agent.h,
// asm/sn/klconfig.h, asm/sn/ioc3.h, linux/serial.h,
// linux/serial_core.h, and ip27-common.h.

const IOC3_CLK: u32 = 22000000 / 3;
const IOC3_FLAGS: u32 = 0;

#[inline]
unsafe fn console_uart() -> *mut ioc3_uartregs {
    let nasid: nasid_t = if master_nasid == INVALID_NASID {
        get_nasid()
    } else {
        master_nasid
    };
    let ioc3: *mut ioc3 = KL_CONFIG_CH_CONS_INFO(nasid).memory_base as *mut ioc3;

    &mut (*ioc3).sregs.uarta
}

pub unsafe fn prom_putchar(c: core::ffi::c_char) {
    let uart: *mut ioc3_uartregs = console_uart();

    while (readb(&(*uart).iu_lsr as *const _) & 0x20) == 0 {}
    writeb(c as u8, &mut (*uart).iu_thr as *mut _);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
