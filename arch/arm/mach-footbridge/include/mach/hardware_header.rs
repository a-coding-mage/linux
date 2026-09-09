/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/mach-footbridge/include/mach/hardware.h
 *
 *  Copyright (C) 1998-1999 Russell King.
 *
 *  This file contains the hardware definitions of the EBSA-285.
 */

/*   Virtual      Physical     Size
 * 0xff800000    0x40000000   1MB    X-Bus
 * 0xff000000    0x7c000000   1MB    PCI I/O space
 * 0xfe000000    0x42000000   1MB    CSR
 * 0xfd000000    0x78000000   1MB    Outbound write flush (not supported)
 * 0xfc000000    0x79000000   1MB    PCI IACK/special space
 * 0xfb000000    0x7a000000   16MB   PCI Config type 1
 * 0xfa000000    0x7b000000   16MB   PCI Config type 0
 * 0xf9000000    0x50000000   1MB    Cache flush
 * 0xf0000000    0x80000000   16MB   ISA memory
 */

pub const XBUS_SIZE: u32 = 0x0010_0000;
pub const XBUS_BASE: u32 = 0xff80_0000;

pub const ARMCSR_SIZE: u32 = 0x0010_0000;
pub const ARMCSR_BASE: u32 = 0xfe00_0000;

pub const WFLUSH_SIZE: u32 = 0x0010_0000;
pub const WFLUSH_BASE: u32 = 0xfd00_0000;

pub const PCIIACK_SIZE: u32 = 0x0010_0000;
pub const PCIIACK_BASE: u32 = 0xfc00_0000;

pub const PCICFG1_SIZE: u32 = 0x0100_0000;
pub const PCICFG1_BASE: u32 = 0xfb00_0000;

pub const PCICFG0_SIZE: u32 = 0x0100_0000;
pub const PCICFG0_BASE: u32 = 0xfa00_0000;

pub const PCIMEM_SIZE: u32 = 0x0100_0000;
pub const PCIMEM_BASE: u32 = 0xf000_0000;

pub const XBUS_CS2: u32 = 0x4001_2000;

pub const XBUS_SWITCH: *mut u8 = (XBUS_BASE + 0x12000) as *mut u8;

#[inline]
pub unsafe fn XBUS_SWITCH_SWITCH() -> u8 {
    core::ptr::read_volatile(XBUS_SWITCH) & 15
}

#[inline]
pub unsafe fn XBUS_SWITCH_J17_13() -> u8 {
    core::ptr::read_volatile(XBUS_SWITCH) & (1 << 4)
}

#[inline]
pub unsafe fn XBUS_SWITCH_J17_11() -> u8 {
    core::ptr::read_volatile(XBUS_SWITCH) & (1 << 5)
}

#[inline]
pub unsafe fn XBUS_SWITCH_J17_9() -> u8 {
    core::ptr::read_volatile(XBUS_SWITCH) & (1 << 6)
}

pub const UNCACHEABLE_ADDR: u32 = ARMCSR_BASE + 0x108; /* CSR_ROMBASEMASK */

/* PIC irq control */
pub const PIC_LO: u32 = 0x20;
pub const PIC_MASK_LO: u32 = 0x21;
pub const PIC_HI: u32 = 0xA0;
pub const PIC_MASK_HI: u32 = 0xA1;

/* GPIO pins */
pub const GPIO_CCLK: u32 = 0x800;
pub const GPIO_DSCLK: u32 = 0x400;
pub const GPIO_E2CLK: u32 = 0x200;
pub const GPIO_IOLOAD: u32 = 0x100;
pub const GPIO_RED_LED: u32 = 0x080;
pub const GPIO_WDTIMER: u32 = 0x040;
pub const GPIO_DATA: u32 = 0x020;
pub const GPIO_IOCLK: u32 = 0x010;
pub const GPIO_DONE: u32 = 0x008;
pub const GPIO_FAN: u32 = 0x004;
pub const GPIO_GREEN_LED: u32 = 0x002;
pub const GPIO_RESET: u32 = 0x001;

/* CPLD pins */
pub const CPLD_DS_ENABLE: u32 = 8;
pub const CPLD_7111_DISABLE: u32 = 4;
pub const CPLD_UNMUTE: u32 = 2;
pub const CPLD_FLASH_WR_ENABLE: u32 = 1;

extern "C" {
    pub static mut nw_gpio_lock: raw_spinlock_t;
    pub fn nw_gpio_modify_op(mask: core::ffi::c_uint, set: core::ffi::c_uint);
    pub fn nw_gpio_modify_io(mask: core::ffi::c_uint, in_: core::ffi::c_uint);
    pub fn nw_gpio_read() -> core::ffi::c_uint;
    pub fn nw_cpld_modify(mask: core::ffi::c_uint, set: core::ffi::c_uint);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
