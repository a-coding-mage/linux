/* SPDX-License-Identifier: GPL-2.0 */

// Dependency corresponding to <linux/device.h>.
// Dependency corresponding to "sysfs.h".

/**
 * enum fw_upload_prog - firmware upload progress codes
 * @FW_UPLOAD_PROG_IDLE: there is no firmware upload in progress
 * @FW_UPLOAD_PROG_RECEIVING: worker thread is receiving firmware data
 * @FW_UPLOAD_PROG_PREPARING: target device is preparing for firmware upload
 * @FW_UPLOAD_PROG_TRANSFERRING: data is being copied to the device
 * @FW_UPLOAD_PROG_PROGRAMMING: device is performing the firmware update
 * @FW_UPLOAD_PROG_MAX: Maximum progress code marker
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum fw_upload_prog {
    FW_UPLOAD_PROG_IDLE,
    FW_UPLOAD_PROG_RECEIVING,
    FW_UPLOAD_PROG_PREPARING,
    FW_UPLOAD_PROG_TRANSFERRING,
    FW_UPLOAD_PROG_PROGRAMMING,
    FW_UPLOAD_PROG_MAX,
}

#[repr(C)]
pub struct fw_upload_priv {
    pub fw_upload: *mut fw_upload,
    pub module: *mut module,
    pub name: *const core::ffi::c_char,
    pub ops: *const fw_upload_ops,
    pub lock: mutex, // protect data structure contents
    pub work: work_struct,
    pub data: *const u8, // pointer to update data
    pub remaining_size: u32, // size remaining to transfer
    pub progress: fw_upload_prog,
    pub err_progress: fw_upload_prog, // progress at time of failure
    pub err_code: fw_upload_err, // security manager error code
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
