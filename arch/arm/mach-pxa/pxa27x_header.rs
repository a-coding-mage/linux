/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: <linux/suspend.h>, "addr-map.h", "pxa2xx-regs.h",
// "mfp-pxa27x.h", and "irqs.h".

/// Arbiter Control Register.
pub const ARB_CNTRL: *mut u32 = 0x4800_0048usize as *mut u32;

/// Be parked with DMA slave when idle.
pub const ARB_DMA_SLV_PARK: u32 = 1u32 << 31;
/// Be parked with Camera Interface when idle.
pub const ARB_CI_PARK: u32 = 1u32 << 30;
/// Be parked with external MEMC when idle.
pub const ARB_EX_MEM_PARK: u32 = 1u32 << 29;
/// Be parked with internal MEMC when idle.
pub const ARB_INT_MEM_PARK: u32 = 1u32 << 28;
/// Be parked with USB when idle.
pub const ARB_USB_PARK: u32 = 1u32 << 27;
/// Be parked with LCD when idle.
pub const ARB_LCD_PARK: u32 = 1u32 << 26;
/// Be parked with DMA when idle.
pub const ARB_DMA_PARK: u32 = 1u32 << 25;
/// Be parked with core when idle.
pub const ARB_CORE_PARK: u32 = 1u32 << 24;
/// Only Locking masters gain access to the bus.
pub const ARB_LOCK_FLAG: u32 = 1u32 << 23;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
