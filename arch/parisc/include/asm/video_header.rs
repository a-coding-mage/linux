/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: linux/types.h

#[repr(C)]
pub struct device;

// Preserved build-time condition: this declaration is available only when
// CONFIG_STI_CORE and CONFIG_VIDEO are both enabled.
#[cfg(all(feature = "CONFIG_STI_CORE", feature = "CONFIG_VIDEO"))]
pub fn video_is_primary_device(dev: *mut device) -> bool;

// Dependency intent: asm-generic/video.h

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
