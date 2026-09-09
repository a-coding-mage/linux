/* SPDX-License-Identifier: MIT */
/******************************************************************************
 * console.h
 *
 * Console I/O interface for Xen guest OSes.
 *
 * Copyright (c) 2005, Keir Fraser
 */

pub type XENCONS_RING_IDX = u32;

macro_rules! MASK_XENCONS_IDX {
    ($idx:expr, $ring:expr) => {
        (($idx) & (core::mem::size_of_val(&$ring) - 1))
    };
}

/*
 * Flag values signaling from backend to frontend whether the console is
 * connected.  i.e. Whether it will be serviced and emptied.
 *
 * The flag starts as disconnected.
 */
pub const XENCONSOLE_DISCONNECTED: u8 = 1;

/*
 * The flag is set to connected when the backend connects and the console
 * will be serviced.
 */
pub const XENCONSOLE_CONNECTED: u8 = 0;

#[repr(C)]
pub struct xencons_interface {
    pub r#in: [i8; 1024],
    pub out: [i8; 2048],
    pub in_cons: XENCONS_RING_IDX,
    pub in_prod: XENCONS_RING_IDX,
    pub out_cons: XENCONS_RING_IDX,
    pub out_prod: XENCONS_RING_IDX,
    pub connection: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
