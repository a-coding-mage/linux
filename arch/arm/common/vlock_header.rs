/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * vlock.h - simple voting lock implementation
 *
 * Created by: Dave Martin, 2012-08-16
 * Copyright: (C) 2012-2013  Linaro Limited
 */

// Dependency supplied by the surrounding platform: MAX_CPUS_PER_CLUSTER.

/* Offsets and sizes are rounded to a word (4 bytes) */
pub const VLOCK_OWNER_OFFSET: u32 = 0;
pub const VLOCK_VOTING_OFFSET: u32 = 4;
pub const VLOCK_VOTING_SIZE: u32 = (MAX_CPUS_PER_CLUSTER + 3) / 4 * 4;
pub const VLOCK_SIZE: u32 = VLOCK_VOTING_OFFSET + VLOCK_VOTING_SIZE;
pub const VLOCK_OWNER_NONE: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
