/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * File: if_phonet.h
 *
 * Phonet interface kernel definitions
 *
 * Copyright (C) 2008 Nokia Corporation. All rights reserved.
 */

pub const PHONET_MIN_MTU: u32 = 6; // pn_length = 0
pub const PHONET_MAX_MTU: u32 = 65541; // pn_length = 0xffff
pub const PHONET_DEV_MTU: u32 = PHONET_MAX_MTU;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
