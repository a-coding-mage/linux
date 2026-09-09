/* SPDX-License-Identifier: GPL-2.0 */

// The original header's Linux includes and header guard are omitted here;
// `uuid_le`, `__u8`, and `kernel_ulong_t` are supplied by the surrounding
// Linux/Rust translation environment.

#[cfg(__KERNEL__)]
pub type kernel_ulong_t = usize;

pub const MEI_CL_MODULE_PREFIX: &str = "mei:";
pub const MEI_CL_NAME_SIZE: usize = 32;
pub const MEI_CL_VERSION_ANY: u8 = 0xff;

/**
 * struct mei_cl_device_id - MEI client device identifier
 * @name: helper name
 * @uuid: client uuid
 * @version: client protocol version
 * @driver_info: information used by the driver.
 *
 * identifies mei client device by uuid and name
 */
#[repr(C)]
pub struct mei_cl_device_id {
    pub name: [core::ffi::c_char; MEI_CL_NAME_SIZE],
    pub uuid: uuid_le,
    pub version: u8,
    pub driver_info: kernel_ulong_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
