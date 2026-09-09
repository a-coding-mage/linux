/* SPDX-License-Identifier: GPL-1.0+ WITH Linux-syscall-note */
/*
 * include/linux/serial.h
 *
 * Copyright (C) 1992 by Theodore Ts'o.
 *
 * Redistribution of this file is permitted under the terms of the GNU
 * Public License (GPL)
 */

// Dependencies supplied by the corresponding Linux headers are intentionally
// left external to this translation.

#[repr(C)]
pub struct serial_struct {
    pub type_: i32,
    pub line: i32,
    pub port: u32,
    pub irq: i32,
    pub flags: i32,
    pub xmit_fifo_size: i32,
    pub custom_divisor: i32,
    pub baud_base: i32,
    pub close_delay: u16,
    pub io_type: i8,
    pub reserved_char: [i8; 1],
    pub hub6: i32,
    pub closing_wait: u16, // time to wait before closing
    pub closing_wait2: u16, // no longer used...
    pub iomem_base: *mut u8,
    pub iomem_reg_shift: u16,
    pub port_high: u32,
    pub iomap_base: usize, // cookie passed into ioremap
}

// For the close wait times, 0 means wait forever for serial port to
// flush its output.  65535 means don't wait at all.
pub const ASYNC_CLOSING_WAIT_INF: i32 = 0;
pub const ASYNC_CLOSING_WAIT_NONE: i32 = 65535;

// These are the supported serial types.
pub const PORT_UNKNOWN: i32 = 0;
pub const PORT_8250: i32 = 1;
pub const PORT_16450: i32 = 2;
pub const PORT_16550: i32 = 3;
pub const PORT_16550A: i32 = 4;
pub const PORT_CIRRUS: i32 = 5;
pub const PORT_16650: i32 = 6;
pub const PORT_16650V2: i32 = 7;
pub const PORT_16750: i32 = 8;
pub const PORT_STARTECH: i32 = 9;
pub const PORT_16C950: i32 = 10; // Oxford Semiconductor
pub const PORT_16654: i32 = 11;
pub const PORT_16850: i32 = 12;
pub const PORT_RSA: i32 = 13; // RSA-DV II/S card
pub const PORT_MAX: i32 = 13;

pub const SERIAL_IO_PORT: i32 = 0;
pub const SERIAL_IO_HUB6: i32 = 1;
pub const SERIAL_IO_MEM: i32 = 2;
pub const SERIAL_IO_MEM32: i32 = 3;
pub const SERIAL_IO_AU: i32 = 4;
pub const SERIAL_IO_TSI: i32 = 5;
pub const SERIAL_IO_MEM32BE: i32 = 6;
pub const SERIAL_IO_MEM16: i32 = 7;
pub const SERIAL_IO_BUS: i32 = 8;

pub const UART_CLEAR_FIFO: i32 = 0x01;
pub const UART_USE_FIFO: i32 = 0x02;
pub const UART_STARTECH: i32 = 0x04;
pub const UART_NATSEMI: i32 = 0x08;

#[repr(C)]
pub struct serial_multiport_struct {
    pub irq: i32,
    pub port1: i32,
    pub mask1: u8,
    pub match1: u8,
    pub port2: i32,
    pub mask2: u8,
    pub match2: u8,
    pub port3: i32,
    pub mask3: u8,
    pub match3: u8,
    pub port4: i32,
    pub mask4: u8,
    pub match4: u8,
    pub port_monitor: i32,
    pub reserved: [i32; 32],
}

#[repr(C)]
pub struct serial_icounter_struct {
    pub cts: i32, pub dsr: i32, pub rng: i32, pub dcd: i32,
    pub rx: i32, pub tx: i32,
    pub frame: i32, pub overrun: i32, pub parity: i32, pub brk: i32,
    pub buf_overrun: i32,
    pub reserved: [i32; 9],
}

#[repr(C)]
pub union serial_rs485_padding {
    pub padding: [u32; 5],
    pub addresses: serial_rs485_addresses,
}

#[repr(C)]
pub struct serial_rs485_addresses {
    pub addr_recv: u8,
    pub addr_dest: u8,
    pub padding0: [u8; 2],
    pub padding1: [u32; 4],
}

#[repr(C)]
pub struct serial_rs485 {
    pub flags: u32,
    pub delay_rts_before_send: u32,
    pub delay_rts_after_send: u32,
    pub padding: serial_rs485_padding,
}

pub const SER_RS485_ENABLED: u32 = 1u32 << 0;
pub const SER_RS485_RTS_ON_SEND: u32 = 1u32 << 1;
pub const SER_RS485_RTS_AFTER_SEND: u32 = 1u32 << 2;
// Placeholder for bit 3: SER_RS485_RTS_BEFORE_SEND, which isn't used anymore
pub const SER_RS485_RX_DURING_TX: u32 = 1u32 << 4;
pub const SER_RS485_TERMINATE_BUS: u32 = 1u32 << 5;
pub const SER_RS485_ADDRB: u32 = 1u32 << 6;
pub const SER_RS485_ADDR_RECV: u32 = 1u32 << 7;
pub const SER_RS485_ADDR_DEST: u32 = 1u32 << 8;
pub const SER_RS485_MODE_RS422: u32 = 1u32 << 9;

#[repr(C)]
pub struct serial_iso7816 {
    pub flags: u32, // ISO7816 feature flags
    pub tg: u32,
    pub sc_fi: u32,
    pub sc_di: u32,
    pub clk: u32,
    pub reserved: [u32; 5],
}

pub const SER_ISO7816_ENABLED: u32 = 1u32 << 0;
pub const SER_ISO7816_T_PARAM: u32 = 0x0fu32 << 4;
#[inline]
pub const fn SER_ISO7816_T(t: u32) -> u32 { (t & 0x0f) << 4 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
