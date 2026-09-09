/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
	Copyright (C) 2004 - 2006 rt2x00 SourceForge Project
	<http://rt2x00.serialmonkey.com>

 */

/*
	Module: eeprom_93cx6
	Abstract: EEPROM reader datastructures for 93cx6 chipsets.
	Supported chipsets: 93c46, 93c56 and 93c66.
 */

/* Dependency supplied by the surrounding kernel translation: linux/bits.h. */

/*
 * EEPROM operation defines.
 */
pub const PCI_EEPROM_WIDTH_93C46: i32 = 6;
pub const PCI_EEPROM_WIDTH_93C56: i32 = 8;
pub const PCI_EEPROM_WIDTH_93C66: i32 = 8;
pub const PCI_EEPROM_WIDTH_93C86: i32 = 8;
pub const PCI_EEPROM_WIDTH_OPCODE: i32 = 3;
pub const PCI_EEPROM_WRITE_OPCODE: i32 = 0x05;
pub const PCI_EEPROM_ERASE_OPCODE: i32 = 0x07;
pub const PCI_EEPROM_READ_OPCODE: i32 = 0x06;
pub const PCI_EEPROM_EWDS_OPCODE: i32 = 0x10;
pub const PCI_EEPROM_EWEN_OPCODE: i32 = 0x13;

/**
 * struct eeprom_93cx6 - control structure for setting the commands
 * for reading the eeprom data.
 * @data: private pointer for the driver.
 * @register_read: handler to read the eeprom register;
 * this function should set all reg_* fields.
 * @register_write: handler to write to the eeprom register by using
 * all reg_* fields.
 * @width: eeprom width, should be one of the PCI_EEPROM_WIDTH_* defines
 * @quirks: eeprom or controller quirks
 * @drive_data: Set if we're driving the data line.
 * @reg_data_in: register field to indicate data input
 * @reg_data_out: register field to indicate data output
 * @reg_data_clock: register field to set the data clock
 * @reg_chip_select: register field to set the chip select
 *
 * This structure is used for the communication between the driver
 * and the eeprom_93cx6 handlers for reading the eeprom.
 */
/* Some EEPROMs require an extra clock cycle before reading */
pub const PCI_EEPROM_QUIRK_EXTRA_READ_CYCLE: u32 = 1u32 << 0;

#[repr(C)]
pub struct eeprom_93cx6 {
	pub data: *mut core::ffi::c_void,

	pub register_read: Option<unsafe extern "C" fn(eeprom: *mut eeprom_93cx6)>,
	pub register_write: Option<unsafe extern "C" fn(eeprom: *mut eeprom_93cx6)>,

	pub width: i32,
	pub quirks: u32,

	pub drive_data: i8,
	pub reg_data_in: i8,
	pub reg_data_out: i8,
	pub reg_data_clock: i8,
	pub reg_chip_select: i8,
}

unsafe extern "C" {
	pub fn eeprom_93cx6_read(eeprom: *mut eeprom_93cx6, word: u8, data: *mut u16);
	pub fn eeprom_93cx6_multiread(
		eeprom: *mut eeprom_93cx6,
		word: u8,
		data: *mut __le16,
		words: u16,
	);
	pub fn eeprom_93cx6_readb(eeprom: *mut eeprom_93cx6, byte: u8, data: *mut u8);
	pub fn eeprom_93cx6_multireadb(
		eeprom: *mut eeprom_93cx6,
		byte: u8,
		data: *mut u8,
		bytes: u16,
	);

	pub fn eeprom_93cx6_wren(eeprom: *mut eeprom_93cx6, enable: bool);

	pub fn eeprom_93cx6_write(eeprom: *mut eeprom_93cx6, addr: u8, data: u16);
}

pub unsafe fn has_quirk_extra_read_cycle(eeprom: *mut eeprom_93cx6) -> bool {
	((*eeprom).quirks & PCI_EEPROM_QUIRK_EXTRA_READ_CYCLE) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
