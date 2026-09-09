/* SPDX-License-Identifier: ((GPL-2.0-only WITH Linux-syscall-note) OR BSD-3-Clause) */
/*
 * linux/can/bcm.h
 *
 * Definitions for CAN Broadcast Manager (BCM)
 *
 * Author: Oliver Hartkopp <oliver.hartkopp@volkswagen.de>
 * Copyright (c) 2002-2007 Volkswagen Group Electronic Research
 * All rights reserved.
 */

// Dependencies supplied by the surrounding UAPI translation:
// `canid_t` and `can_frame` correspond to the declarations from <linux/can.h>.

#[repr(C)]
pub struct bcm_timeval {
    pub tv_sec: core::ffi::c_long,
    pub tv_usec: core::ffi::c_long,
}

/**
 * struct bcm_msg_head - head of messages to/from the broadcast manager
 * @opcode:    opcode, see enum below.
 * @flags:     special flags, see below.
 * @count:     number of frames to send before changing interval.
 * @ival1:     interval for the first @count frames.
 * @ival2:     interval for the following frames.
 * @can_id:    CAN ID of frames to be sent or received.
 * @nframes:   number of frames appended to the message head.
 * @frames:    array of CAN frames.
 */
#[repr(C)]
pub struct bcm_msg_head {
    pub opcode: u32,
    pub flags: u32,
    pub count: u32,
    pub ival1: bcm_timeval,
    pub ival2: bcm_timeval,
    pub can_id: canid_t,
    pub nframes: u32,
    pub frames: [can_frame; 0],
}

pub const TX_SETUP: i32 = 1; // create (cyclic) transmission task
pub const TX_DELETE: i32 = 2; // remove (cyclic) transmission task
pub const TX_READ: i32 = 3; // read properties of (cyclic) transmission task
pub const TX_SEND: i32 = 4; // send one CAN frame
pub const RX_SETUP: i32 = 5; // create RX content filter subscription
pub const RX_DELETE: i32 = 6; // remove RX content filter subscription
pub const RX_READ: i32 = 7; // read properties of RX content filter subscription
pub const TX_STATUS: i32 = 8; // reply to TX_READ request
pub const TX_EXPIRED: i32 = 9; // notification on performed transmissions (count=0)
pub const RX_STATUS: i32 = 10; // reply to RX_READ request
pub const RX_TIMEOUT: i32 = 11; // cyclic message is absent
pub const RX_CHANGED: i32 = 12; // updated CAN frame (detected content change)

pub const SETTIMER: u32 = 0x0001;
pub const STARTTIMER: u32 = 0x0002;
pub const TX_COUNTEVT: u32 = 0x0004;
pub const TX_ANNOUNCE: u32 = 0x0008;
pub const TX_CP_CAN_ID: u32 = 0x0010;
pub const RX_FILTER_ID: u32 = 0x0020;
pub const RX_CHECK_DLC: u32 = 0x0040;
pub const RX_NO_AUTOTIMER: u32 = 0x0080;
pub const RX_ANNOUNCE_RESUME: u32 = 0x0100;
pub const TX_RESET_MULTI_IDX: u32 = 0x0200;
pub const RX_RTR_FRAME: u32 = 0x0400;
pub const CAN_FD_FRAME: u32 = 0x0800;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
