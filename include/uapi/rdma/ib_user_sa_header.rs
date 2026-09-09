/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR Linux-OpenIB) */
/*
 * Copyright (c) 2005 Intel Corporation.  All rights reserved.
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
 *      - Redistributions of source code must retain the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer.
 *
 *      - Redistributions in binary form must reproduce the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer in the documentation and/or other materials
 *        provided with the distribution.
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

// Originally supplied by <linux/types.h>.

#[repr(u32)]
pub enum IbPath {
    IB_PATH_GMP = 1,
    IB_PATH_PRIMARY = 1 << 1,
    IB_PATH_ALTERNATE = 1 << 2,
    IB_PATH_OUTBOUND = 1 << 3,
    IB_PATH_INBOUND = 1 << 4,
    IB_PATH_INBOUND_REVERSE = 1 << 5,
    IB_PATH_BIDIRECTIONAL = (1 << 3) | (1 << 5),
}

#[repr(C)]
pub struct ib_path_rec_data {
    pub flags: u32,
    pub reserved: u32,
    pub path_rec: [u32; 16],
}

#[repr(C)]
pub struct ib_user_path_rec {
    pub dgid: [u8; 16],
    pub sgid: [u8; 16],
    pub dlid: u16,
    pub slid: u16,
    pub raw_traffic: u32,
    pub flow_label: u32,
    pub reversible: u32,
    pub mtu: u32,
    pub pkey: u16,
    pub hop_limit: u8,
    pub traffic_class: u8,
    pub numb_path: u8,
    pub sl: u8,
    pub mtu_selector: u8,
    pub rate_selector: u8,
    pub rate: u8,
    pub packet_life_time_selector: u8,
    pub packet_life_time: u8,
    pub preference: u8,
}

#[repr(C)]
pub struct ib_user_service_rec {
    pub id: u64,
    pub gid: [u8; 16],
    pub pkey: u16,
    pub reserved: [u8; 2],
    pub lease: u32,
    pub key: [u8; 16],
    pub name: [u8; 64],
    pub data_8: [u8; 16],
    pub data_16: [u16; 8],
    pub data_32: [u32; 4],
    pub data_64: [u64; 2],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
