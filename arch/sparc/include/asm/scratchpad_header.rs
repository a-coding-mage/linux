/* SPDX-License-Identifier: GPL-2.0 */

// Sun4v scratchpad registers, accessed via ASI_SCRATCHPAD.

pub const SCRATCHPAD_MMU_MISS: u64 = 0x00; // Shared with OBP - set by OBP
pub const SCRATCHPAD_CPUID: u64 = 0x08; // Shared with OBP - set by hypervisor
pub const SCRATCHPAD_UTSBREG1: u64 = 0x10;
pub const SCRATCHPAD_UTSBREG2: u64 = 0x18;
// 0x20 and 0x28, hypervisor only...
pub const SCRATCHPAD_UNUSED1: u64 = 0x30;
pub const SCRATCHPAD_UNUSED2: u64 = 0x38; // Reserved for OBP

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
