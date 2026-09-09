/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2008 Maxime Bizon <mbizon@freebox.fr>
 */

// Dependencies are supplied by the corresponding platform and serial modules.

extern "C" {
    fn bcm_uart0_readl(reg: u32) -> u32;
    fn bcm_uart0_writel(value: u32, reg: u32);
}

unsafe fn wait_xfered() {
    let mut val: u32;

    /* wait for any previous char to be transmitted */
    loop {
        val = bcm_uart0_readl(UART_IR_REG);
        if val & uart_ir_stat(UART_IR_TXEMPTY) != 0 {
            break;
        }
    }
}

pub unsafe extern "C" fn prom_putchar(c: core::ffi::c_char) {
    wait_xfered();
    bcm_uart0_writel(c as u32, UART_FIFO_REG);
    wait_xfered();
}

// `UART_IR_STAT(...)` is the platform-provided C macro, represented here as
// the corresponding Rust dependency.
extern "C" {
    fn uart_ir_stat(value: u32) -> u32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
