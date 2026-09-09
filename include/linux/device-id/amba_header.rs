/* SPDX-License-Identifier: GPL-2.0 */

/**
 * struct amba_id - identifies a device on an AMBA bus
 * @id: The significant bits if the hardware device ID
 * @mask: Bitmask specifying which bits of the id field are significant when
 *	matching.  A driver binds to a device when ((hardware device ID) & mask)
 *	== id.
 * @data: Private data used by the driver.
 */
#[repr(C)]
pub struct amba_id {
    pub id: u32,
    pub mask: u32,
    pub data: *mut core::ffi::c_void,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
