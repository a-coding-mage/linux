/*
 * Copyright (c) 2015 Mellanox Technologies. All rights reserved.
 *
 * This software is available to you under a choice of one of two
 * licenses.  You may choose to be licensed under the terms of the GNU
 * General Public License (GPL) Version 2, available from the file
 * COPYING in the main directory of this source tree, or the
 * OpenIB.org BSD license below:
 *
 *     Redistribution and use in source and binary forms, with or
 *     without modification, are permitted provided that the following
 *     conditions are met:
 *
 *	- Redistributions of source code must retain the above
 *	  copyright notice, this list of conditions and the following
 *	  disclaimer.
 *
 *	- Redistributions in binary form must reproduce the above
 *	  copyright notice, this list of conditions and the following
 *	  disclaimer in the documentation and/or other materials
 *	  provided with the distribution.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

pub const ISER_ZBVA_NOT_SUP: u8 = 0x80;
pub const ISER_SEND_W_INV_NOT_SUP: u8 = 0x40;
pub const ISERT_ZBVA_NOT_USED: u8 = 0x80;
pub const ISERT_SEND_W_INV_NOT_USED: u8 = 0x40;

pub const ISCSI_CTRL: u8 = 0x10;
pub const ISER_HELLO: u8 = 0x20;
pub const ISER_HELLORPLY: u8 = 0x30;

pub const ISER_VER: u8 = 0x10;
pub const ISER_WSV: u8 = 0x08;
pub const ISER_RSV: u8 = 0x04;

/**
 * struct iser_cm_hdr - iSER CM header (from iSER Annex A12)
 *
 * @flags:        flags support (zbva, send_w_inv)
 * @rsvd:         reserved
 */
#[repr(C, packed)]
pub struct iser_cm_hdr {
	pub flags: u8,
	pub rsvd: [u8; 3],
}

/**
 * struct iser_ctrl - iSER header of iSCSI control PDU
 *
 * @flags:        opcode and read/write valid bits
 * @rsvd:         reserved
 * @write_stag:   write rkey
 * @write_va:     write virtual address
 * @read_stag:    read rkey
 * @read_va:      read virtual address
 */
#[repr(C, packed)]
pub struct iser_ctrl {
	pub flags: u8,
	pub rsvd: [u8; 3],
	pub write_stag: u32,
	pub write_va: u64,
	pub read_stag: u32,
	pub read_va: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
