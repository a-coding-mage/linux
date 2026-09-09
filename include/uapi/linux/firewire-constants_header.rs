/*
 * IEEE 1394 constants.
 *
 * Copyright (C) 2005-2007  Kristian Hoegsberg <krh@bitplanet.net>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */

pub const TCODE_WRITE_QUADLET_REQUEST: u32 = 0x0;
pub const TCODE_WRITE_BLOCK_REQUEST: u32 = 0x1;
pub const TCODE_WRITE_RESPONSE: u32 = 0x2;
pub const TCODE_READ_QUADLET_REQUEST: u32 = 0x4;
pub const TCODE_READ_BLOCK_REQUEST: u32 = 0x5;
pub const TCODE_READ_QUADLET_RESPONSE: u32 = 0x6;
pub const TCODE_READ_BLOCK_RESPONSE: u32 = 0x7;
pub const TCODE_CYCLE_START: u32 = 0x8;
pub const TCODE_LOCK_REQUEST: u32 = 0x9;
pub const TCODE_STREAM_DATA: u32 = 0xa;
pub const TCODE_LOCK_RESPONSE: u32 = 0xb;

pub const EXTCODE_MASK_SWAP: u32 = 0x1;
pub const EXTCODE_COMPARE_SWAP: u32 = 0x2;
pub const EXTCODE_FETCH_ADD: u32 = 0x3;
pub const EXTCODE_LITTLE_ADD: u32 = 0x4;
pub const EXTCODE_BOUNDED_ADD: u32 = 0x5;
pub const EXTCODE_WRAP_ADD: u32 = 0x6;
pub const EXTCODE_VENDOR_DEPENDENT: u32 = 0x7;

/* Linux firewire-core (Juju) specific tcodes */
pub const TCODE_LOCK_MASK_SWAP: u32 = 0x10 | EXTCODE_MASK_SWAP;
pub const TCODE_LOCK_COMPARE_SWAP: u32 = 0x10 | EXTCODE_COMPARE_SWAP;
pub const TCODE_LOCK_FETCH_ADD: u32 = 0x10 | EXTCODE_FETCH_ADD;
pub const TCODE_LOCK_LITTLE_ADD: u32 = 0x10 | EXTCODE_LITTLE_ADD;
pub const TCODE_LOCK_BOUNDED_ADD: u32 = 0x10 | EXTCODE_BOUNDED_ADD;
pub const TCODE_LOCK_WRAP_ADD: u32 = 0x10 | EXTCODE_WRAP_ADD;
pub const TCODE_LOCK_VENDOR_DEPENDENT: u32 = 0x10 | EXTCODE_VENDOR_DEPENDENT;

pub const RCODE_COMPLETE: u32 = 0x0;
pub const RCODE_CONFLICT_ERROR: u32 = 0x4;
pub const RCODE_DATA_ERROR: u32 = 0x5;
pub const RCODE_TYPE_ERROR: u32 = 0x6;
pub const RCODE_ADDRESS_ERROR: u32 = 0x7;

/* Linux firewire-core (Juju) specific rcodes */
pub const RCODE_SEND_ERROR: u32 = 0x10;
pub const RCODE_CANCELLED: u32 = 0x11;
pub const RCODE_BUSY: u32 = 0x12;
pub const RCODE_GENERATION: u32 = 0x13;
pub const RCODE_NO_ACK: u32 = 0x14;

pub const SCODE_100: u32 = 0x0;
pub const SCODE_200: u32 = 0x1;
pub const SCODE_400: u32 = 0x2;
pub const SCODE_800: u32 = 0x3;
pub const SCODE_1600: u32 = 0x4;
pub const SCODE_3200: u32 = 0x5;
pub const SCODE_BETA: u32 = 0x3;

pub const ACK_COMPLETE: u32 = 0x1;
pub const ACK_PENDING: u32 = 0x2;
pub const ACK_BUSY_X: u32 = 0x4;
pub const ACK_BUSY_A: u32 = 0x5;
pub const ACK_BUSY_B: u32 = 0x6;
pub const ACK_DATA_ERROR: u32 = 0xd;
pub const ACK_TYPE_ERROR: u32 = 0xe;

pub const RETRY_1: u32 = 0x00;
pub const RETRY_X: u32 = 0x01;
pub const RETRY_A: u32 = 0x02;
pub const RETRY_B: u32 = 0x03;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
