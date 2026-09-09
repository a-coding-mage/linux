/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Channel numbers used by the microchip-tcb-capture driver
 * Copyright (C) 2025 Bence Csókás
 */

/*
 * The driver defines the following components:
 *
 * Count 0
 * \__  Synapse 0 -- Signal 0 (Channel A, i.e. TIOA)
 * \__  Synapse 1 -- Signal 1 (Channel B, i.e. TIOB)
 * \__  Extension capture0    (RA register)
 * \__  Extension capture1    (RB register)
 *
 * It also supports the following events:
 *
 * Channel 0:
 * - CV register changed
 * - CV overflowed
 * - RA captured
 * Channel 1:
 * - RB captured
 * Channel 2:
 * - RC compare triggered
 */

/* Capture extensions */
pub const COUNTER_MCHP_EXCAP_RA: u32 = 0;
pub const COUNTER_MCHP_EXCAP_RB: u32 = 1;

/* Event channels */
pub const COUNTER_MCHP_EVCHN_CV: u32 = 0;
pub const COUNTER_MCHP_EVCHN_RA: u32 = 0;
pub const COUNTER_MCHP_EVCHN_RB: u32 = 1;
pub const COUNTER_MCHP_EVCHN_RC: u32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
