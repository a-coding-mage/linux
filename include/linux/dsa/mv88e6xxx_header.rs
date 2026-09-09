/* SPDX-License-Identifier: GPL-2.0
 * Copyright 2021 NXP
 */

// Dependency supplied by the Linux VLAN headers: VLAN_N_VID.

pub const MV88E6XXX_VID_STANDALONE: u32 = 0;
pub const MV88E6XXX_VID_BRIDGED: u32 = VLAN_N_VID - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
