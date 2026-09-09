/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause */
/* Copyright (c) 2010-2014 Broadcom. All rights reserved. */

// VCHIQ_MAKE_FOURCC('V', 'C', 'H', 'I')
pub const VCHIQ_MAGIC: u32 = u32::from_le_bytes(*b"VCHI");

/* The version of VCHIQ - change with any non-trivial change */
pub const VCHIQ_VERSION: u32 = 8;

/*
 * The minimum compatible version - update to match VCHIQ_VERSION with any
 * incompatible change
 */
pub const VCHIQ_VERSION_MIN: u32 = 3;

/* The version that introduced the VCHIQ_IOC_LIB_VERSION ioctl */
pub const VCHIQ_VERSION_LIB_VERSION: u32 = 7;

/* The version that introduced the VCHIQ_IOC_CLOSE_DELIVERED ioctl */
pub const VCHIQ_VERSION_CLOSE_DELIVERED: u32 = 7;

/* The version that made it safe to use SYNCHRONOUS mode */
pub const VCHIQ_VERSION_SYNCHRONOUS_MODE: u32 = 8;

pub const VCHIQ_MAX_STATES: u32 = 1;
pub const VCHIQ_MAX_SERVICES: u32 = 4096;
pub const VCHIQ_MAX_SLOTS: u32 = 128;
pub const VCHIQ_MAX_SLOTS_PER_SIDE: u32 = 64;

pub const VCHIQ_NUM_CURRENT_BULKS: u32 = 32;
pub const VCHIQ_NUM_SERVICE_BULKS: u32 = 4;

// C preprocessor default: applied when VCHIQ_ENABLE_DEBUG is not defined.
pub const VCHIQ_ENABLE_DEBUG: u32 = 1;

// C preprocessor default: applied when VCHIQ_ENABLE_STATS is not defined.
pub const VCHIQ_ENABLE_STATS: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
