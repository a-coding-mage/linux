/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *	crc16.h - CRC-16 routine
 *
 * Implements the standard CRC-16:
 *   Width 16
 *   Poly  0x8005 (x^16 + x^15 + x^2 + 1)
 *   Init  0
 *
 * Copyright (c) 2005 Ben Gardner <bgardner@wabtec.com>
 */

// Dependency supplied by the Linux type definitions: u16, u8, and size_t.

unsafe extern "C" {
    pub fn crc16(crc: u16, p: *const u8, len: usize) -> u16;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
