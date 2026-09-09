/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: __REG is supplied by pxa-regs.h.

/*
 * Real Time Clock
 */

pub const RCNR: *mut u32 = 0x4090_0000usize as *mut u32; /* RTC Count Register */
pub const RTAR: *mut u32 = 0x4090_0004usize as *mut u32; /* RTC Alarm Register */
pub const RTSR: *mut u32 = 0x4090_0008usize as *mut u32; /* RTC Status Register */
pub const RTTR: *mut u32 = 0x4090_000Cusize as *mut u32; /* RTC Timer Trim Register */
pub const PIAR: *mut u32 = 0x4090_0038usize as *mut u32; /* Periodic Interrupt Alarm Register */

pub const RTSR_PICE: u32 = 1u32 << 15; /* Periodic interrupt count enable */
pub const RTSR_PIALE: u32 = 1u32 << 14; /* Periodic interrupt Alarm enable */
pub const RTSR_HZE: u32 = 1u32 << 3; /* HZ interrupt enable */
pub const RTSR_ALE: u32 = 1u32 << 2; /* RTC alarm interrupt enable */
pub const RTSR_HZ: u32 = 1u32 << 1; /* HZ rising-edge detected */
pub const RTSR_AL: u32 = 1u32 << 0; /* RTC alarm detected */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
