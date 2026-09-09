/* SPDX-License-Identifier: ((GPL-2.0-only WITH Linux-syscall-note) OR BSD-3-Clause) */
/*
 * linux/can/error.h
 *
 * Definitions of the CAN error messages to be filtered and passed to the user.
 *
 * Author: Oliver Hartkopp <oliver.hartkopp@volkswagen.de>
 * Copyright (c) 2002-2007 Volkswagen Group Electronic Research
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 * 1. Redistributions of source code must retain the above copyright notice,
 *    this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 * 3. Neither the name of Volkswagen nor the names of its contributors may be
 *    used to endorse or promote products derived from this software without
 *    specific prior written permission.
 *
 * Alternatively, provided that this notice is retained in full, this software
 * may be distributed under the terms of the GNU General Public License ("GPL"),
 * in which case the provisions of the GPL apply INSTEAD OF those given above.
 *
 * The provided data structures and external interfaces from this code are not
 * restricted to be used by modules with a GPL compatible license.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES ARE DISCLAIMED.
 */

pub const CAN_ERR_DLC: u32 = 8; /* dlc for error message frames */

/* error class (mask) in can_id */
pub const CAN_ERR_TX_TIMEOUT: u32 = 0x00000001; /* TX timeout (by netdevice driver) */
pub const CAN_ERR_LOSTARB: u32 = 0x00000002; /* lost arbitration / data[0] */
pub const CAN_ERR_CRTL: u32 = 0x00000004; /* controller problems / data[1] */
pub const CAN_ERR_PROT: u32 = 0x00000008; /* protocol violations / data[2..3] */
pub const CAN_ERR_TRX: u32 = 0x00000010; /* transceiver status / data[4] */
pub const CAN_ERR_ACK: u32 = 0x00000020; /* received no ACK on transmission */
pub const CAN_ERR_BUSOFF: u32 = 0x00000040; /* bus off */
pub const CAN_ERR_BUSERROR: u32 = 0x00000080; /* bus error (may flood!) */
pub const CAN_ERR_RESTARTED: u32 = 0x00000100; /* controller restarted */
pub const CAN_ERR_CNT: u32 = 0x00000200; /* TX error counter / data[6] */

/* arbitration lost in bit ... / data[0] */
pub const CAN_ERR_LOSTARB_UNSPEC: u8 = 0x00; /* unspecified */

/* error status of CAN-controller / data[1] */
pub const CAN_ERR_CRTL_UNSPEC: u8 = 0x00;
pub const CAN_ERR_CRTL_RX_OVERFLOW: u8 = 0x01;
pub const CAN_ERR_CRTL_TX_OVERFLOW: u8 = 0x02;
pub const CAN_ERR_CRTL_RX_WARNING: u8 = 0x04;
pub const CAN_ERR_CRTL_TX_WARNING: u8 = 0x08;
pub const CAN_ERR_CRTL_RX_PASSIVE: u8 = 0x10;
pub const CAN_ERR_CRTL_TX_PASSIVE: u8 = 0x20;
pub const CAN_ERR_CRTL_ACTIVE: u8 = 0x40;

/* error in CAN protocol (type) / data[2] */
pub const CAN_ERR_PROT_UNSPEC: u8 = 0x00;
pub const CAN_ERR_PROT_BIT: u8 = 0x01;
pub const CAN_ERR_PROT_FORM: u8 = 0x02;
pub const CAN_ERR_PROT_STUFF: u8 = 0x04;
pub const CAN_ERR_PROT_BIT0: u8 = 0x08;
pub const CAN_ERR_PROT_BIT1: u8 = 0x10;
pub const CAN_ERR_PROT_OVERLOAD: u8 = 0x20;
pub const CAN_ERR_PROT_ACTIVE: u8 = 0x40;
pub const CAN_ERR_PROT_TX: u8 = 0x80;

/* error in CAN protocol (location) / data[3] */
pub const CAN_ERR_PROT_LOC_UNSPEC: u8 = 0x00;
pub const CAN_ERR_PROT_LOC_SOF: u8 = 0x03;
pub const CAN_ERR_PROT_LOC_ID28_21: u8 = 0x02;
pub const CAN_ERR_PROT_LOC_ID20_18: u8 = 0x06;
pub const CAN_ERR_PROT_LOC_SRTR: u8 = 0x04;
pub const CAN_ERR_PROT_LOC_IDE: u8 = 0x05;
pub const CAN_ERR_PROT_LOC_ID17_13: u8 = 0x07;
pub const CAN_ERR_PROT_LOC_ID12_05: u8 = 0x0F;
pub const CAN_ERR_PROT_LOC_ID04_00: u8 = 0x0E;
pub const CAN_ERR_PROT_LOC_RTR: u8 = 0x0C;
pub const CAN_ERR_PROT_LOC_RES1: u8 = 0x0D;
pub const CAN_ERR_PROT_LOC_RES0: u8 = 0x09;
pub const CAN_ERR_PROT_LOC_DLC: u8 = 0x0B;
pub const CAN_ERR_PROT_LOC_DATA: u8 = 0x0A;
pub const CAN_ERR_PROT_LOC_CRC_SEQ: u8 = 0x08;
pub const CAN_ERR_PROT_LOC_CRC_DEL: u8 = 0x18;
pub const CAN_ERR_PROT_LOC_ACK: u8 = 0x19;
pub const CAN_ERR_PROT_LOC_ACK_DEL: u8 = 0x1B;
pub const CAN_ERR_PROT_LOC_EOF: u8 = 0x1A;
pub const CAN_ERR_PROT_LOC_INTERM: u8 = 0x12;

/* error status of CAN-transceiver / data[4] */
pub const CAN_ERR_TRX_UNSPEC: u8 = 0x00;
pub const CAN_ERR_TRX_CANH_NO_WIRE: u8 = 0x04;
pub const CAN_ERR_TRX_CANH_SHORT_TO_BAT: u8 = 0x05;
pub const CAN_ERR_TRX_CANH_SHORT_TO_VCC: u8 = 0x06;
pub const CAN_ERR_TRX_CANH_SHORT_TO_GND: u8 = 0x07;
pub const CAN_ERR_TRX_CANL_NO_WIRE: u8 = 0x40;
pub const CAN_ERR_TRX_CANL_SHORT_TO_BAT: u8 = 0x50;
pub const CAN_ERR_TRX_CANL_SHORT_TO_VCC: u8 = 0x60;
pub const CAN_ERR_TRX_CANL_SHORT_TO_GND: u8 = 0x70;
pub const CAN_ERR_TRX_CANL_SHORT_TO_CANH: u8 = 0x80;

/* data[5] is reserved (do not use) */
/* TX error counter / data[6] */
/* RX error counter / data[7] */

/* CAN state thresholds */
pub const CAN_ERROR_WARNING_THRESHOLD: u32 = 96;
pub const CAN_ERROR_PASSIVE_THRESHOLD: u32 = 128;
pub const CAN_BUS_OFF_THRESHOLD: u32 = 256;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
