// SPDX-License-Identifier: GPL-2.0
// Dependency equivalent of <asm/mach-au1x00/au1000.h>.
// Dependency equivalent of "decompress.h".

extern "C" {
    fn alchemy_uart_putchar(addr: usize, c: core::ffi::c_char);
}

// AU1000_UART0_PHYS_ADDR is supplied by the architecture dependency.
extern "C" {
    static AU1000_UART0_PHYS_ADDR: usize;
}

pub unsafe fn putc(c: core::ffi::c_char) {
    alchemy_uart_putchar(AU1000_UART0_PHYS_ADDR, c);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
