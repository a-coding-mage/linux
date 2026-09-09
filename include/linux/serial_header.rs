/*
 * include/linux/serial.h
 *
 * Copyright (C) 1992 by Theodore Ts'o.
 * 
 * Redistribution of this file is permitted under the terms of the GNU 
 * Public License (GPL)
 */

// Dependencies supplied by the corresponding UAPI and compiler headers:
// `UART_IER_MSI`, `UART_IER_RLSI`, `UART_IER_THRI`, `UART_IER_RDI`,
// `UART_LSR_TEMT`, `UART_LSR_THRE`, `UART_MSR_DCD`, `UART_MSR_RI`,
// `UART_MSR_DSR`, and `UART_MSR_CTS`.

pub const UART_IER_ALL_INTR: u32 = UART_IER_MSI
    | UART_IER_RLSI
    | UART_IER_THRI
    | UART_IER_RDI;

/* Helper for dealing with UART_LCR_WLEN* defines */
#[inline]
pub const fn UART_LCR_WLEN(x: u32) -> u32 {
    x - 5
}

/* FIFO and shifting register empty */
pub const UART_LSR_BOTH_EMPTY: u32 = UART_LSR_TEMT | UART_LSR_THRE;

#[inline]
pub fn uart_lsr_tx_empty(lsr: u16) -> bool {
    (lsr & UART_LSR_BOTH_EMPTY as u16) == UART_LSR_BOTH_EMPTY as u16
}

pub const UART_MSR_STATUS_BITS: u32 = UART_MSR_DCD
    | UART_MSR_RI
    | UART_MSR_DSR
    | UART_MSR_CTS;

/*
 * Counters of the input lines (CTS, DSR, RI, CD) interrupts
 */
#[repr(C)]
pub struct async_icount {
    pub cts: u32,
    pub dsr: u32,
    pub rng: u32,
    pub dcd: u32,
    pub tx: u32,
    pub rx: u32,
    pub frame: u32,
    pub parity: u32,
    pub overrun: u32,
    pub brk: u32,
    pub buf_overrun: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
