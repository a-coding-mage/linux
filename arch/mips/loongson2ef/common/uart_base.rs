// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2009 Lemote Inc.
 * Author: Wu Zhangjin, wuzhangjin@gmail.com
 */

// C dependencies: <linux/export.h>, <asm/bootinfo.h>, <asm/setup.h>, <loongson.h>

/* raw */
pub static mut loongson_uart_base: usize = 0;
/* ioremapped */
pub static mut _loongson_uart_base: usize = 0;

extern "C" {
    static mut mips_machtype: i32;

    static LOONGSON_PCIIO_BASE: usize;
    static LOONGSON_LIO1_BASE: usize;

    static MACH_LEMOTE_FL2E: i32;
    static MACH_LEMOTE_FL2F: i32;
    static MACH_LEMOTE_LL2F: i32;
    static MACH_LEMOTE_ML2F7: i32;
    static MACH_LEMOTE_YL2F89: i32;
    static MACH_DEXXON_GDIUM2F10: i32;
    static MACH_LEMOTE_NAS: i32;

    fn TO_UNCAC(address: usize) -> usize;
    fn setup_8250_early_printk_port(address: usize, offset: i32, size: i32);
}

#[no_mangle]
pub unsafe extern "C" fn prom_init_loongson_uart_base() {
    loongson_uart_base = match mips_machtype {
        x if x == MACH_LEMOTE_FL2E => LOONGSON_PCIIO_BASE + 0x3f8,
        x if x == MACH_LEMOTE_FL2F || x == MACH_LEMOTE_LL2F => {
            LOONGSON_PCIIO_BASE + 0x2f8
        }
        x if x == MACH_LEMOTE_ML2F7
            || x == MACH_LEMOTE_YL2F89
            || x == MACH_DEXXON_GDIUM2F10
            || x == MACH_LEMOTE_NAS =>
        {
            // The CPU provided serial port (LPC)
            LOONGSON_LIO1_BASE + 0x3f8
        }
        _ => {
            // The CPU provided serial port (LPC)
            LOONGSON_LIO1_BASE + 0x3f8
        }
    };

    _loongson_uart_base = TO_UNCAC(loongson_uart_base);
    setup_8250_early_printk_port(_loongson_uart_base, 0, 1024);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
