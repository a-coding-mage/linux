/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * upd64031a - NEC Electronics Ghost Reduction input defines
 *
 * 2006 by Hans Verkuil (hverkuil@kernel.org)
 */

/* Ghost reduction modes */
pub const UPD64031A_GR_ON: u32 = 0;
pub const UPD64031A_GR_OFF: u32 = 1;
pub const UPD64031A_GR_THROUGH: u32 = 3;

/* Direct 3D/YCS Connection */
pub const UPD64031A_3DYCS_DISABLE: u32 = 0 << 2;
pub const UPD64031A_3DYCS_COMPOSITE: u32 = 2 << 2;
pub const UPD64031A_3DYCS_SVIDEO: u32 = 3 << 2;

/* Composite sync digital separation circuit */
pub const UPD64031A_COMPOSITE_EXTERNAL: u32 = 1 << 4;

/* Vertical sync digital separation circuit */
pub const UPD64031A_VERTICAL_EXTERNAL: u32 = 1 << 5;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
