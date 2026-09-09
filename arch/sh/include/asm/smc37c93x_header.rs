/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/include/asm-sh/smc37c93x.h
 *
 * Copyright (C) 2000  Kazumoto Kojima
 *
 * SMSC 37C93x Super IO Chip support
 */

/* Default base I/O address */
pub const FDC_PRIMARY_BASE: u16 = 0x3f0;
pub const IDE1_PRIMARY_BASE: u16 = 0x1f0;
pub const IDE1_SECONDARY_BASE: u16 = 0x170;
pub const PARPORT_PRIMARY_BASE: u16 = 0x378;
pub const COM1_PRIMARY_BASE: u16 = 0x2f8;
pub const COM2_PRIMARY_BASE: u16 = 0x3f8;
pub const RTC_PRIMARY_BASE: u16 = 0x070;
pub const KBC_PRIMARY_BASE: u16 = 0x060;
pub const AUXIO_PRIMARY_BASE: u16 = 0x000; /* XXX */

/* Logical device number */
pub const LDN_FDC: u16 = 0;
pub const LDN_IDE1: u16 = 1;
pub const LDN_IDE2: u16 = 2;
pub const LDN_PARPORT: u16 = 3;
pub const LDN_COM1: u16 = 4;
pub const LDN_COM2: u16 = 5;
pub const LDN_RTC: u16 = 6;
pub const LDN_KBC: u16 = 7;
pub const LDN_AUXIO: u16 = 8;

/* Configuration port and key */
pub const CONFIG_PORT: u16 = 0x3f0;
pub const INDEX_PORT: u16 = CONFIG_PORT;
pub const DATA_PORT: u16 = 0x3f1;
pub const CONFIG_ENTER: u16 = 0x55;
pub const CONFIG_EXIT: u16 = 0xaa;

/* Configuration index */
pub const CURRENT_LDN_INDEX: u16 = 0x07;
pub const POWER_CONTROL_INDEX: u16 = 0x22;
pub const ACTIVATE_INDEX: u16 = 0x30;
pub const IO_BASE_HI_INDEX: u16 = 0x60;
pub const IO_BASE_LO_INDEX: u16 = 0x61;
pub const IRQ_SELECT_INDEX: u16 = 0x70;
pub const DMA_SELECT_INDEX: u16 = 0x74;
pub const GPIO46_INDEX: u16 = 0xc6;
pub const GPIO47_INDEX: u16 = 0xc7;

/* UART stuff. Only for debugging. */
/* UART Register */
pub const UART_RBR: u16 = 0x0; /* Receiver Buffer Register (Read Only) */
pub const UART_THR: u16 = 0x0; /* Transmitter Holding Register (Write Only) */
pub const UART_IER: u16 = 0x2; /* Interrupt Enable Register */
pub const UART_IIR: u16 = 0x4; /* Interrupt Ident Register (Read Only) */
pub const UART_FCR: u16 = 0x4; /* FIFO Control Register (Write Only) */
pub const UART_LCR: u16 = 0x6; /* Line Control Register */
pub const UART_MCR: u16 = 0x8; /* MODEM Control Register */
pub const UART_LSR: u16 = 0xa; /* Line Status Register */
pub const UART_MSR: u16 = 0xc; /* MODEM Status Register */
pub const UART_SCR: u16 = 0xe; /* Scratch Register */
pub const UART_DLL: u16 = 0x0; /* Divisor Latch (LS) */
pub const UART_DLM: u16 = 0x2; /* Divisor Latch (MS) */

#[repr(C)]
pub struct uart_reg {
    pub rbr: u16,
    pub ier: u16,
    pub iir: u16,
    pub lcr: u16,
    pub mcr: u16,
    pub lsr: u16,
    pub msr: u16,
    pub scr: u16,
}

/* Alias for Write Only Register */
/* thr -> rbr; tcr -> iir */

/* Alias for Divisor Latch Register */
/* dll -> rbr; dlm -> ier; fcr -> iir */

/* Interrupt Enable Register */
pub const IER_ERDAI: u16 = 0x0100; /* Enable Received Data Available Interrupt */
pub const IER_ETHREI: u16 = 0x0200; /* Enable Transmitter Holding Register Empty Interrupt */
pub const IER_ELSI: u16 = 0x0400; /* Enable Receiver Line Status Interrupt */
pub const IER_EMSI: u16 = 0x0800; /* Enable MODEM Status Interrupt */

/* Interrupt Ident Register */
pub const IIR_IP: u16 = 0x0100; /* "0" if Interrupt Pending */
pub const IIR_IIB0: u16 = 0x0200; /* Interrupt ID Bit 0 */
pub const IIR_IIB1: u16 = 0x0400; /* Interrupt ID Bit 1 */
pub const IIR_IIB2: u16 = 0x0800; /* Interrupt ID Bit 2 */
pub const IIR_FIFO: u16 = 0xc000; /* FIFOs enabled */

/* FIFO Control Register */
pub const FCR_FEN: u16 = 0x0100; /* FIFO enable */
pub const FCR_RFRES: u16 = 0x0200; /* Receiver FIFO reset */
pub const FCR_TFRES: u16 = 0x0400; /* Transmitter FIFO reset */
pub const FCR_DMA: u16 = 0x0800; /* DMA mode select */
pub const FCR_RTL: u16 = 0x4000; /* Receiver trigger (LSB) */
pub const FCR_RTM: u16 = 0x8000; /* Receiver trigger (MSB) */

/* Line Control Register */
pub const LCR_WLS0: u16 = 0x0100;
pub const LCR_WLS1: u16 = 0x0200;
pub const LCR_STB: u16 = 0x0400;
pub const LCR_PEN: u16 = 0x0800;
pub const LCR_EPS: u16 = 0x1000;
pub const LCR_SP: u16 = 0x2000;
pub const LCR_SB: u16 = 0x4000;
pub const LCR_DLAB: u16 = 0x8000;

/* MODEM Control Register */
pub const MCR_DTR: u16 = 0x0100;
pub const MCR_RTS: u16 = 0x0200;
pub const MCR_OUT1: u16 = 0x0400;
pub const MCR_IRQEN: u16 = 0x0800;
pub const MCR_LOOP: u16 = 0x1000;

/* Line Status Register */
pub const LSR_DR: u16 = 0x0100;
pub const LSR_OE: u16 = 0x0200;
pub const LSR_PE: u16 = 0x0400;
pub const LSR_FE: u16 = 0x0800;
pub const LSR_BI: u16 = 0x1000;
pub const LSR_THRE: u16 = 0x2000;
pub const LSR_TEMT: u16 = 0x4000;
pub const LSR_FIFOE: u16 = 0x8000;

/* MODEM Status Register */
pub const MSR_DCTS: u16 = 0x0100;
pub const MSR_DDSR: u16 = 0x0200;
pub const MSR_TERI: u16 = 0x0400;
pub const MSR_DDCD: u16 = 0x0800;
pub const MSR_CTS: u16 = 0x1000;
pub const MSR_DSR: u16 = 0x2000;
pub const MSR_RI: u16 = 0x4000;
pub const MSR_DCD: u16 = 0x8000;

/* Baud Rate Divisor */
pub const UART_CLK: u32 = 1843200; /* 1.8432 MHz */
#[inline]
pub const fn UART_BAUD(x: u32) -> u32 {
    UART_CLK / (16 * x)
}

/* RTC register definition */
pub const RTC_SECONDS: u16 = 0;
pub const RTC_SECONDS_ALARM: u16 = 1;
pub const RTC_MINUTES: u16 = 2;
pub const RTC_MINUTES_ALARM: u16 = 3;
pub const RTC_HOURS: u16 = 4;
pub const RTC_HOURS_ALARM: u16 = 5;
pub const RTC_DAY_OF_WEEK: u16 = 6;
pub const RTC_DAY_OF_MONTH: u16 = 7;
pub const RTC_MONTH: u16 = 8;
pub const RTC_YEAR: u16 = 9;
pub const RTC_FREQ_SELECT: u16 = 10;
pub const RTC_UIP: u16 = 0x80;
pub const RTC_DIV_CTL: u16 = 0x70;
/* This RTC can work under 32.768KHz clock only. */
pub const RTC_OSC_ENABLE: u16 = 0x20;
pub const RTC_OSC_DISABLE: u16 = 0x00;
pub const RTC_CONTROL: u16 = 11;
pub const RTC_SET: u16 = 0x80;
pub const RTC_PIE: u16 = 0x40;
pub const RTC_AIE: u16 = 0x20;
pub const RTC_UIE: u16 = 0x10;
pub const RTC_SQWE: u16 = 0x08;
pub const RTC_DM_BINARY: u16 = 0x04;
pub const RTC_24H: u16 = 0x02;
pub const RTC_DST_EN: u16 = 0x01;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
