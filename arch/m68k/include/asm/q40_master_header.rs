/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Q40 master Chip Control
 * RTC stuff merged for compactness.
 */

// C header dependencies: <asm/raw_io.h>, <asm/kmap.h>, and <asm/q40ints.h>.

pub const q40_master_addr: usize = 0xff000000;

pub const IIRQ_REG: usize = 0x0; // internal IRQ reg
pub const EIRQ_REG: usize = 0x4; // external ...
pub const KEYCODE_REG: usize = 0x1c; // value of received scancode
pub const DISPLAY_CONTROL_REG: usize = 0x18;
pub const FRAME_CLEAR_REG: usize = 0x24;
pub const LED_REG: usize = 0x30;

// The external raw I/O functions are supplied by the surrounding platform.
unsafe extern "C" {
    pub fn in_8(addr: *const u8) -> u8;
    pub fn out_8(addr: *mut u8, value: u8);
}

#[inline]
pub unsafe fn master_inb(reg: usize) -> u8 {
    unsafe { in_8((q40_master_addr.wrapping_add(reg)) as *const u8) }
}

#[inline]
pub unsafe fn master_outb(byte: u8, reg: usize) {
    unsafe { out_8((q40_master_addr.wrapping_add(reg)) as *mut u8, byte) }
}

#[inline]
pub unsafe fn Q40_LED_ON() {
    unsafe { master_outb(1, LED_REG) }
}

#[inline]
pub unsafe fn Q40_LED_OFF() {
    unsafe { master_outb(0, LED_REG) }
}

pub const INTERRUPT_REG: usize = IIRQ_REG; // "native" ints
pub const KEY_IRQ_ENABLE_REG: usize = 0x08;
pub const KEYBOARD_UNLOCK_REG: usize = 0x20; // clear kb int

pub const SAMPLE_ENABLE_REG: usize = 0x14; // generate SAMPLE ints
pub const SAMPLE_RATE_REG: usize = 0x2c;
pub const SAMPLE_CLEAR_REG: usize = 0x28;
pub const SAMPLE_LOW: usize = 0x00;
pub const SAMPLE_HIGH: usize = 0x01;

pub const FRAME_RATE_REG: usize = 0x38; // generate FRAME ints at 200 HZ rate

// #if 0: SER_ENABLE_REG is intentionally disabled in the original header.
// pub const SER_ENABLE_REG: usize = 0x0c; // allow serial ints to be generated
pub const EXT_ENABLE_REG: usize = 0x10; // ... rest of the ISA ints ...

/* RTC defines */
pub const Q40_RTC_BASE: usize = 0xff021ffc;

pub const Q40_RTC_YEAR: *mut u8 = (Q40_RTC_BASE.wrapping_add(0)) as *mut u8;
pub const Q40_RTC_MNTH: *mut u8 = (Q40_RTC_BASE.wrapping_sub(4)) as *mut u8;
pub const Q40_RTC_DATE: *mut u8 = (Q40_RTC_BASE.wrapping_sub(8)) as *mut u8;
pub const Q40_RTC_DOW: *mut u8 = (Q40_RTC_BASE.wrapping_sub(12)) as *mut u8;
pub const Q40_RTC_HOUR: *mut u8 = (Q40_RTC_BASE.wrapping_sub(16)) as *mut u8;
pub const Q40_RTC_MINS: *mut u8 = (Q40_RTC_BASE.wrapping_sub(20)) as *mut u8;
pub const Q40_RTC_SECS: *mut u8 = (Q40_RTC_BASE.wrapping_sub(24)) as *mut u8;
pub const Q40_RTC_CTRL: *mut u8 = (Q40_RTC_BASE.wrapping_sub(28)) as *mut u8;

/* some control bits */
pub const Q40_RTC_READ: u8 = 64; // prepare for reading
pub const Q40_RTC_WRITE: u8 = 128;

/* misc defs */
pub const DAC_LEFT: *mut u8 = 0xff008000 as *mut u8;
pub const DAC_RIGHT: *mut u8 = 0xff008004 as *mut u8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
