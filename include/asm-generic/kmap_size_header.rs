/* SPDX-License-Identifier: GPL-2.0 */

// For debug this provides guard pages between the maps.
//
// The C build-time CONFIG_DEBUG_KMAP_LOCAL condition is represented here by
// the corresponding Cargo feature.
#[cfg(feature = "CONFIG_DEBUG_KMAP_LOCAL")]
pub const KM_MAX_IDX: usize = 33;

#[cfg(not(feature = "CONFIG_DEBUG_KMAP_LOCAL"))]
pub const KM_MAX_IDX: usize = 16;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
