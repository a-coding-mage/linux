/* SPDX-License-Identifier: ((GPL-2.0-only WITH Linux-syscall-note) OR BSD-3-Clause) */
/*
 * linux/can/raw.h
 *
 * Definitions for raw CAN sockets
 *
 * Authors: Oliver Hartkopp <oliver.hartkopp@volkswagen.de>
 *          Urs Thuermann   <urs.thuermann@volkswagen.de>
 * Copyright (c) 2002-2007 Volkswagen Group Electronic Research
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of Volkswagen nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * Alternatively, provided that this notice is retained in full, this
 * software may be distributed under the terms of the GNU General
 * Public License ("GPL") version 2, in which case the provisions of the
 * GPL apply INSTEAD OF those given above.
 *
 * The provided data structures and external interfaces from this code
 * are not restricted to be used by modules with a GPL compatible license.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

// Dependency supplied by linux/can.h: SOL_CAN_BASE and CAN_RAW.
pub const SOL_CAN_RAW: u32 = SOL_CAN_BASE + CAN_RAW;
pub const CAN_RAW_FILTER_MAX: u32 = 512; /* maximum number of can_filter set via setsockopt() */

pub const SCM_CAN_RAW_ERRQUEUE: u32 = 1;

/* for socket options affecting the socket (not the global system) */
pub const CAN_RAW_FILTER: u32 = 1; /* set 0 .. n can_filter(s) */
pub const CAN_RAW_ERR_FILTER: u32 = 2; /* set filter for error frames */
pub const CAN_RAW_LOOPBACK: u32 = 3; /* local loopback (default:on) */
pub const CAN_RAW_RECV_OWN_MSGS: u32 = 4; /* receive my own msgs (default:off) */
pub const CAN_RAW_FD_FRAMES: u32 = 5; /* allow CAN FD frames (default:off) */
pub const CAN_RAW_JOIN_FILTERS: u32 = 6; /* all filters must match to trigger */
pub const CAN_RAW_XL_FRAMES: u32 = 7; /* allow CAN XL frames (default:off) */
pub const CAN_RAW_XL_VCID_OPTS: u32 = 8; /* CAN XL VCID configuration options */

/* configuration for CAN XL virtual CAN identifier (VCID) handling */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct can_raw_vcid_options {
    pub flags: u8,       /* flags for vcid (filter) behaviour */
    pub tx_vcid: u8,     /* VCID value set into canxl_frame.prio */
    pub rx_vcid: u8,     /* VCID value for VCID filter */
    pub rx_vcid_mask: u8, /* VCID mask for VCID filter */
}

/* can_raw_vcid_options.flags for CAN XL virtual CAN identifier handling */
pub const CAN_RAW_XL_VCID_TX_SET: u8 = 0x01;
pub const CAN_RAW_XL_VCID_TX_PASS: u8 = 0x02;
pub const CAN_RAW_XL_VCID_RX_FILTER: u8 = 0x04;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
