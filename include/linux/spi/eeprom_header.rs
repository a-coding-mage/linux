/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: the original header includes <linux/memory.h>.

/*
 * Put one of these structures in platform_data for SPI EEPROMS handled
 * by the "at25" driver.  On SPI, most EEPROMS understand the same core
 * command set.  If you need to support EEPROMs that don't yet fit, add
 * flags to support those protocol options.  These values all come from
 * the chip datasheets.
 */
#[repr(C)]
pub struct spi_eeprom {
	pub byte_len: u32,
	pub name: [core::ffi::c_char; 10],
	pub page_size: u32, /* for writes */
	pub flags: u16,
	pub context: *mut core::ffi::c_void,
}

pub const EE_ADDR1: u16 = 0x0001; /*  8 bit addrs */
pub const EE_ADDR2: u16 = 0x0002; /* 16 bit addrs */
pub const EE_ADDR3: u16 = 0x0004; /* 24 bit addrs */
pub const EE_READONLY: u16 = 0x0008; /* disallow writes */

/*
 * Certain EEPROMS have a size that is larger than the number of address
 * bytes would allow (e.g. like M95040 from ST that has 512 Byte size
 * but uses only one address byte (A0 to A7) for addressing.) For
 * the extra address bit (A8, A16 or A24) bit 3 of the instruction byte
 * is used. This instruction bit is normally defined as don't care for
 * other AT25 like chips.
 */
pub const EE_INSTR_BIT3_IS_ADDR: u16 = 0x0010;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
