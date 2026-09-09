/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependencies supplied by the surrounding UAPI translation:
// linux/types.h, asm/ioctl.h, and asm/papr-miscdev.h.

pub const LOC_CODE_SIZE: usize = 80;
pub const RTAS_GET_INDICES_BUF_SIZE: usize = 4096; // SZ_4K

#[repr(C)]
#[derive(Copy, Clone)]
pub struct papr_indices_io_block_indices {
    pub is_sensor: u8, // 0 for indicator and 1 for sensor
    pub indice_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct papr_indices_io_block_dynamic_param {
    pub token: u32, // Sensor or indicator token
    pub state: u32, // get / set state
    // PAPR+ 12.3.2.4 Converged Location Code Rules - Length
    // Restrictions. 79 characters plus null.
    pub location_code_str: [core::ffi::c_char; LOC_CODE_SIZE], // location code
}

#[repr(C)]
pub union papr_indices_io_block {
    pub indices: papr_indices_io_block_indices,
    pub dynamic_param: papr_indices_io_block_dynamic_param,
}

// ioctls for /dev/papr-indices.
// PAPR_INDICES_IOC_GET: Returns a get-indices handle fd to read data
// PAPR_DYNAMIC_SENSOR_IOC_GET: Gets the state of the input sensor
// PAPR_DYNAMIC_INDICATOR_IOC_SET: Sets the new state for the input indicator
// The _IOW/_IOWR encodings depend on asm/ioctl.h and PAPR_MISCDEV_IOC_ID.
pub const PAPR_INDICES_IOC_GET: usize = _IOW(PAPR_MISCDEV_IOC_ID, 3, papr_indices_io_block);
pub const PAPR_DYNAMIC_SENSOR_IOC_GET: usize =
    _IOWR(PAPR_MISCDEV_IOC_ID, 4, papr_indices_io_block);
pub const PAPR_DYNAMIC_INDICATOR_IOC_SET: usize =
    _IOW(PAPR_MISCDEV_IOC_ID, 5, papr_indices_io_block);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
