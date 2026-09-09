// SPDX-License-Identifier: GPL-2.0
/*
 * 16550 compatible uart based serial debug support for zboot
 */

// linux/types.h, linux/serial_reg.h, asm/addrspace.h, and decompress.h
// provide the corresponding types, UART constants, address conversion, and
// configuration-dependent definitions in the surrounding build.

#[cfg(any(feature = "CONFIG_MACH_LOONGSON64", feature = "CONFIG_MIPS_MALTA"))]
const UART_BASE: usize = 0x1fd003f8;

#[cfg(feature = "CONFIG_MACH_INGENIC")]
const INGENIC_UART_BASE_ADDR: usize =
    0x10030000 + 0x1000 * CONFIG_ZBOOT_INGENIC_UART;

#[cfg(feature = "CONFIG_ECONET")]
const EN75_UART_BASE: usize = 0x1fbf0003;

// CKSEG1ADDR is supplied by asm/addrspace.h in the surrounding build.
extern "C" {
    fn CKSEG1ADDR(address: usize) -> usize;
}

#[cfg(any(feature = "CONFIG_MACH_LOONGSON64", feature = "CONFIG_MIPS_MALTA"))]
#[inline]
unsafe fn port(offset: isize) -> usize {
    CKSEG1ADDR(UART_BASE).wrapping_add(offset as usize)
}

#[cfg(feature = "CONFIG_MACH_INGENIC")]
#[inline]
unsafe fn port(offset: isize) -> usize {
    CKSEG1ADDR(INGENIC_UART_BASE_ADDR).wrapping_add((4isize * offset) as usize)
}

#[cfg(feature = "CONFIG_ECONET")]
#[inline]
unsafe fn port(offset: isize) -> usize {
    CKSEG1ADDR(EN75_UART_BASE).wrapping_add((4isize * offset) as usize)
}

#[cfg(not(any(
    feature = "CONFIG_MACH_LOONGSON64",
    feature = "CONFIG_MIPS_MALTA",
    feature = "CONFIG_MACH_INGENIC",
    feature = "CONFIG_ECONET"
)))]
compile_error!("please define the serial port address for your own machine");

// IOTYPE defaults to char when not supplied by the build.
type IoType = i8;

#[inline]
unsafe fn serial_in(offset: isize) -> u32 {
    core::ptr::read_volatile(port(offset) as *const IoType) as u8 as u32
}

#[inline]
unsafe fn serial_out(offset: isize, value: i32) {
    core::ptr::write_volatile(port(offset) as *mut IoType, (value & 0xFF) as IoType);
}

pub unsafe fn putc(c: i8) {
    let mut timeout: i32 = 1000000;

    while ((serial_in(UART_LSR) & UART_LSR_THRE) == 0) && {
        timeout -= 1;
        timeout >= 0
    } {}

    serial_out(UART_TX, c as i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
