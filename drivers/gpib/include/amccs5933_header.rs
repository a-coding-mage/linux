/* SPDX-License-Identifier: GPL-2.0 */

/***************************************************************************
 * Registers and bits for amccs5933 pci chip
 *    copyright            : (C) 2002 by Frank Mori Hess
 ***************************************************************************/

// register offsets
#[repr(i32)]
enum Register {
    MBEF_REG = 0x34,   // mailbux empty/full
    INTCSR_REG = 0x38, // interrupt control and status
    BMCSR_REG = 0x3c,  // bus master control and status
}

// incoming mailbox 0-3  register offsets
#[inline]
fn INCOMING_MAILBOX_REG(mailbox: u32) -> i32 {
    (0x10u32.wrapping_add(4u32.wrapping_mul(mailbox))) as i32
}

// bit definitions

// INTCSR bits
#[repr(i32)]
enum IntcsrBit {
    OUTBOX_EMPTY_INTR_BIT = 0x10,   // enable outbox empty interrupt
    INBOX_FULL_INTR_BIT = 0x1000,   // enable inbox full interrupt
    INBOX_INTR_CS_BIT = 0x20000,    // read, or write clear inbox full interrupt
    INTR_ASSERTED_BIT = 0x800000,   // read only, interrupt asserted
}

// select byte 0 to 3 of incoming mailbox
#[inline]
fn INBOX_BYTE_BITS(byte: u32) -> i32 {
    ((byte & 0x3) << 8) as i32
}

// select incoming mailbox 0 to 3
#[inline]
fn INBOX_SELECT_BITS(mailbox: u32) -> i32 {
    ((mailbox & 0x3) << 10) as i32
}

// select byte 0 to 3 of outgoing mailbox
#[inline]
fn OUTBOX_BYTE_BITS(byte: u32) -> i32 {
    (byte & 0x3) as i32
}

// select outgoing mailbox 0 to 3
#[inline]
fn OUTBOX_SELECT_BITS(mailbox: u32) -> i32 {
    ((mailbox & 0x3) << 2) as i32
}

// BMCSR bits
#[repr(i32)]
enum BmcsrBit {
    MBOX_FLAGS_RESET_BIT = 0x08000000, // resets mailbox empty/full flags
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
