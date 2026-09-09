/* SPDX-License-Identifier: GPL-2.0 */
// Header guard: _LINUX_CRC32_POLY_H

/* The polynomial used by crc32_le(), in integer form.  See crc32_le(). */
pub const CRC32_POLY_LE: u32 = 0xedb88320;

/* The polynomial used by crc32_be(), in integer form.  See crc32_be(). */
pub const CRC32_POLY_BE: u32 = 0x04c11db7;

/* The polynomial used by crc32c(), in integer form.  See crc32c(). */
pub const CRC32C_POLY_LE: u32 = 0x82f63b78;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
