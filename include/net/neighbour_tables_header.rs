/* SPDX-License-Identifier: GPL-2.0 */

pub const NEIGH_ARP_TABLE: i32 = 0;
pub const NEIGH_ND_TABLE: i32 = 1;
pub const NEIGH_NR_TABLES: i32 = 2;
pub const NEIGH_LINK_TABLE: i32 = NEIGH_NR_TABLES; /* Pseudo table for neigh_xmit */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
