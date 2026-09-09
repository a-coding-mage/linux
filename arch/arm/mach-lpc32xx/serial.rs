// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * arch/arm/mach-lpc32xx/serial.c
 *
 * Author: Kevin Wells <kevin.wells@nxp.com>
 *
 * Copyright (C) 2010 NXP Semiconductors
 */

// Linux kernel headers and local headers provide the types, constants,
// register accessors, and helper functions referenced below.

const LPC32XX_SUART_FIFO_SIZE: i32 = 64;

#[repr(C)]
struct uartinit {
    uart_ck_name: *mut i8,
    ck_mode_mask: u32,
    pdiv_clk_reg: *mut core::ffi::c_void,
    mapbase: resource_size_t,
}

static mut uartinit_data: [uartinit; 4] = [
    uartinit {
        uart_ck_name: b"uart5_ck\0".as_ptr() as *mut i8,
        ck_mode_mask: LPC32XX_UART_CLKMODE_LOAD(LPC32XX_UART_CLKMODE_ON, 5),
        pdiv_clk_reg: LPC32XX_CLKPWR_UART5_CLK_CTRL,
        mapbase: LPC32XX_UART5_BASE,
    },
    uartinit {
        uart_ck_name: b"uart3_ck\0".as_ptr() as *mut i8,
        ck_mode_mask: LPC32XX_UART_CLKMODE_LOAD(LPC32XX_UART_CLKMODE_ON, 3),
        pdiv_clk_reg: LPC32XX_CLKPWR_UART3_CLK_CTRL,
        mapbase: LPC32XX_UART3_BASE,
    },
    uartinit {
        uart_ck_name: b"uart4_ck\0".as_ptr() as *mut i8,
        ck_mode_mask: LPC32XX_UART_CLKMODE_LOAD(LPC32XX_UART_CLKMODE_ON, 4),
        pdiv_clk_reg: LPC32XX_CLKPWR_UART4_CLK_CTRL,
        mapbase: LPC32XX_UART4_BASE,
    },
    uartinit {
        uart_ck_name: b"uart6_ck\0".as_ptr() as *mut i8,
        ck_mode_mask: LPC32XX_UART_CLKMODE_LOAD(LPC32XX_UART_CLKMODE_ON, 6),
        pdiv_clk_reg: LPC32XX_CLKPWR_UART6_CLK_CTRL,
        mapbase: LPC32XX_UART6_BASE,
    },
];

/* LPC3250 Errata HSUART.1: Hang workaround via loopback mode on inactivity */
pub unsafe fn lpc32xx_loopback_set(mapbase: resource_size_t, state: i32) {
    let bit: i32;
    let mut tmp: u32;

    match mapbase {
        LPC32XX_HS_UART1_BASE => bit = 0,
        LPC32XX_HS_UART2_BASE => bit = 1,
        LPC32XX_HS_UART7_BASE => bit = 6,
        _ => {
            WARN!(1, "lpc32xx_hs: Warning: Unknown port at %08x\n", mapbase);
            return;
        }
    }

    tmp = readl(LPC32XX_UARTCTL_CLOOP);
    if state != 0 {
        tmp |= 1u32.wrapping_shl(bit as u32);
    } else {
        tmp &= !(1u32.wrapping_shl(bit as u32));
    }
    writel(tmp, LPC32XX_UARTCTL_CLOOP);
}

pub unsafe fn lpc32xx_serial_init() {
    let mut tmp: u32;
    let mut clkmodes: u32 = 0;
    let mut clk: *mut clk;
    let mut puart: u32;
    let mut i: usize;
    let mut j: i32;

    i = 0;
    while i < uartinit_data.len() {
        clk = clk_get(core::ptr::null_mut(), uartinit_data[i].uart_ck_name);
        if !IS_ERR(clk) {
            clk_enable(clk);
        }

        /* Setup UART clock modes for all UARTs, disable autoclock */
        clkmodes |= uartinit_data[i].ck_mode_mask;

        /* pre-UART clock divider set to 1 */
        __raw_writel(0x0101, uartinit_data[i].pdiv_clk_reg);

        /*
         * Force a flush of the RX FIFOs to work around a
         * HW bug
         */
        puart = uartinit_data[i].mapbase;
        __raw_writel(0xC1, LPC32XX_UART_IIR_FCR(puart));
        __raw_writel(0x00, LPC32XX_UART_DLL_FIFO(puart));
        j = LPC32XX_SUART_FIFO_SIZE;
        while j > 0 {
            j -= 1;
            tmp = __raw_readl(LPC32XX_UART_DLL_FIFO(puart));
        }
        __raw_writel(0, LPC32XX_UART_IIR_FCR(puart));
        i += 1;
    }

    /* This needs to be done after all UART clocks are setup */
    __raw_writel(clkmodes, LPC32XX_UARTCTL_CLKMODE);
    i = 0;
    while i < uartinit_data.len() {
        /* Force a flush of the RX FIFOs to work around a HW bug */
        puart = uartinit_data[i].mapbase;
        __raw_writel(0xC1, LPC32XX_UART_IIR_FCR(puart));
        __raw_writel(0x00, LPC32XX_UART_DLL_FIFO(puart));
        j = LPC32XX_SUART_FIFO_SIZE;
        while j > 0 {
            j -= 1;
            tmp = __raw_readl(LPC32XX_UART_DLL_FIFO(puart));
        }
        __raw_writel(0, LPC32XX_UART_IIR_FCR(puart));
        i += 1;
    }

    /* Disable IrDA pulsing support on UART6 */
    tmp = __raw_readl(LPC32XX_UARTCTL_CTRL);
    tmp |= LPC32XX_UART_UART6_IRDAMOD_BYPASS;
    __raw_writel(tmp, LPC32XX_UARTCTL_CTRL);

    /* Disable UART5->USB transparent mode or USB won't work */
    tmp = __raw_readl(LPC32XX_UARTCTL_CTRL);
    tmp &= !LPC32XX_UART_U5_ROUTE_TO_USB;
    __raw_writel(tmp, LPC32XX_UARTCTL_CTRL);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
