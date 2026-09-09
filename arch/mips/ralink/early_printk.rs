// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 *  Copyright (C) 2011-2012 Gabor Juhos <juhosg@openwrt.org>
 */

// The C source includes Linux I/O, serial-register, address-space, and setup
// definitions. Their externally supplied symbols are referenced below.

// Build-time selection preserved from CONFIG_SOC_RT288X,
// CONFIG_SOC_MT7621, and the fallback configuration.
#[cfg(CONFIG_SOC_RT288X)]
const EARLY_UART_BASE: usize = 0x300c00;
#[cfg(CONFIG_SOC_RT288X)]
const CHIPID_BASE: usize = 0x300004;
#[cfg(all(not(CONFIG_SOC_RT288X), CONFIG_SOC_MT7621))]
const EARLY_UART_BASE: usize = 0x1E000c00;
#[cfg(all(not(CONFIG_SOC_RT288X), CONFIG_SOC_MT7621))]
const CHIPID_BASE: usize = 0x1E000004;
#[cfg(all(not(CONFIG_SOC_RT288X), not(CONFIG_SOC_MT7621)))]
const EARLY_UART_BASE: usize = 0x10000c00;
#[cfg(all(not(CONFIG_SOC_RT288X), not(CONFIG_SOC_MT7621)))]
const CHIPID_BASE: usize = 0x10000004;

const MT7628_CHIP_NAME1: u32 = 0x20203832;

const UART_REG_TX: usize = 0x04;
const UART_REG_LCR: usize = 0x0c;
const UART_REG_LSR: usize = 0x14;
const UART_REG_LSR_RT2880: usize = 0x1c;

// Supplied by <asm/addrspace.h> and <linux/serial_reg.h> in the original.
const UART_TX: usize = 0;
const UART_LSR_THRE: u32 = 0x20;

static mut uart_membase: *mut u8 = EARLY_UART_BASE as *mut u8;
static mut chipid_membase: *mut u8 = CHIPID_BASE as *mut u8;
static mut init_complete: i32 = 0;

#[inline]
unsafe fn uart_w32(val: u32, reg: usize) {
    core::ptr::write_volatile(uart_membase.add(reg) as *mut u32, val);
}

#[inline]
unsafe fn uart_r32(reg: usize) -> u32 {
    core::ptr::read_volatile(uart_membase.add(reg) as *const u32)
}

#[inline]
unsafe fn soc_is_mt7628() -> bool {
    // IS_ENABLED(CONFIG_SOC_MT7620) is a build-time condition in the C code.
    cfg!(CONFIG_SOC_MT7620)
        && (core::ptr::read_volatile(chipid_membase as *const u32) == MT7628_CHIP_NAME1)
}

unsafe fn find_uart_base() {
    if !soc_is_mt7628() {
        return;
    }

    for i in 0..3 {
        let reg = uart_r32(UART_REG_LCR + (0x100 * i));

        if reg == 0 {
            continue;
        }

        uart_membase = (EARLY_UART_BASE + (0x100 * i)) as *mut u8;
        break;
    }
}

pub unsafe fn prom_putchar(ch: i8) {
    if init_complete == 0 {
        find_uart_base();
        init_complete = 1;
    }

    if cfg!(CONFIG_SOC_MT7621) || soc_is_mt7628() {
        uart_w32(ch as u8 as u32, UART_TX);
        while (uart_r32(UART_REG_LSR) & UART_LSR_THRE) == 0 {}
    } else {
        while (uart_r32(UART_REG_LSR_RT2880) & UART_LSR_THRE) == 0 {}
        uart_w32(ch as u8 as u32, UART_REG_TX);
        while (uart_r32(UART_REG_LSR_RT2880) & UART_LSR_THRE) == 0 {}
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
