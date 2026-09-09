/* SPDX-License-Identifier: GPL-2.0 */
/* FIXME
 * move this to include/linux/device-id/greybus.h when merging
 */

// Translated from the C header. The linux/types.h dependency supplies the
// fixed-width integer types and kernel_ulong_t used by the original.

#[repr(C)]
pub struct greybus_bundle_id {
    pub match_flags: u16,
    pub vendor: u32,
    pub product: u32,
    pub class: u8,

    // C: kernel_ulong_t driver_info __aligned(sizeof(kernel_ulong_t));
    // usize preserves kernel_ulong_t's pointer-sized integer intent.
    pub driver_info: usize,
}

/* Used to match the greybus_bundle_id */
pub const GREYBUS_ID_MATCH_VENDOR: u32 = 1u32 << 0;
pub const GREYBUS_ID_MATCH_PRODUCT: u32 = 1u32 << 1;
pub const GREYBUS_ID_MATCH_CLASS: u32 = 1u32 << 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
