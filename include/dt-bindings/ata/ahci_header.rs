/* SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause */
/*
 * This header provides constants for most AHCI bindings.
 */

/* Host Bus Adapter generic platform capabilities */
pub const HBA_SSS: u32 = 1u32 << 27;
pub const HBA_SMPS: u32 = 1u32 << 28;

/* Host Bus Adapter port-specific platform capabilities */
pub const HBA_PORT_HPCP: u32 = 1u32 << 18;
pub const HBA_PORT_MPSP: u32 = 1u32 << 19;
pub const HBA_PORT_CPD: u32 = 1u32 << 20;
pub const HBA_PORT_ESP: u32 = 1u32 << 21;
pub const HBA_PORT_FBSCP: u32 = 1u32 << 22;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
