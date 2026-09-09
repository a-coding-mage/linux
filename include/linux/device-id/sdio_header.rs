/* SPDX-License-Identifier: GPL-2.0 */

/* __KERNEL__: #include <linux/types.h> */
pub type kernel_ulong_t = usize;

/* SDIO */

pub const SDIO_ANY_ID: u32 = !0u32;

#[repr(C)]
pub struct sdio_device_id {
    pub class: u8,              /* Standard interface or SDIO_ANY_ID */
    pub vendor: u16,            /* Vendor or SDIO_ANY_ID */
    pub device: u16,            /* Device ID or SDIO_ANY_ID */
    pub driver_data: kernel_ulong_t, /* Data private to the driver */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
