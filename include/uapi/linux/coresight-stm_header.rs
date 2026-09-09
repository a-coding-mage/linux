/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// C dependency: <linux/const.h> (_BITUL)

pub const STM_FLAG_TIMESTAMPED: usize = 1usize << 3;
pub const STM_FLAG_MARKED: usize = 1usize << 4;
pub const STM_FLAG_GUARANTEED: usize = 1usize << 7;

/*
 * The CoreSight STM supports guaranteed and invariant timing
 * transactions.  Guaranteed transactions are guaranteed to be
 * traced, this might involve stalling the bus or system to
 * ensure the transaction is accepted by the STM.  While invariant
 * timing transactions are not guaranteed to be traced, they
 * will take an invariant amount of time regardless of the
 * state of the STM.
 */
pub const STM_OPTION_GUARANTEED: u32 = 0;
pub const STM_OPTION_INVARIANT: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
