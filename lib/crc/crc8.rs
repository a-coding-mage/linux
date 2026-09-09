/*
 * Copyright (c) 2011 Broadcom Corporation
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
 * OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
 * CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

/**
 * crc8_populate_msb - fill crc table for given polynomial in reverse bit order.
 *
 * @table: table to be filled.
 * @polynomial: polynomial for which table is to be filled.
 */
pub unsafe fn crc8_populate_msb(table: *mut u8, polynomial: u8) {
    let msbit: u8 = 0x80;
    let mut t: u8 = msbit;

    *table.add(0) = 0;

    let mut i: usize = 1;
    while i < CRC8_TABLE_SIZE {
        t = (t.wrapping_shl(1)) ^ (if t & msbit != 0 { polynomial } else { 0 });
        let mut j: usize = 0;
        while j < i {
            *table.add(i + j) = *table.add(j) ^ t;
            j += 1;
        }
        i *= 2;
    }
}

/**
 * crc8_populate_lsb - fill crc table for given polynomial in regular bit order.
 *
 * @table: table to be filled.
 * @polynomial: polynomial for which table is to be filled.
 */
pub unsafe fn crc8_populate_lsb(table: *mut u8, polynomial: u8) {
    let mut t: u8 = 1;

    *table.add(0) = 0;

    let mut i: usize = CRC8_TABLE_SIZE >> 1;
    while i != 0 {
        t = (t >> 1) ^ (if t & 1 != 0 { polynomial } else { 0 });
        let mut j: usize = 0;
        while j < CRC8_TABLE_SIZE {
            *table.add(i + j) = *table.add(j) ^ t;
            j += 2 * i;
        }
        i >>= 1;
    }
}

/**
 * crc8 - calculate a crc8 over the given input data.
 *
 * @table: crc table used for calculation.
 * @pdata: pointer to data buffer.
 * @nbytes: number of bytes in data buffer.
 * @crc: previous returned crc8 value.
 */
pub unsafe fn crc8(table: *const u8, mut pdata: *const u8, mut nbytes: usize, mut crc: u8) -> u8 {
    /* loop over the buffer data */
    while nbytes > 0 {
        crc = *table.add((crc ^ *pdata) as usize & 0xff);
        pdata = pdata.add(1);
        nbytes -= 1;
    }

    crc
}

// C module metadata and exported-symbol declarations have no direct Rust equivalent.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
