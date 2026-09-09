/* SPDX-License-Identifier: GPL-2.0 */

// Dependency corresponding to <uapi/asm/cmb.h>.

#[repr(C)]
pub struct ccw_device {
    _private: [u8; 0],
}

// Declared by <uapi/asm/cmb.h>.
#[repr(C)]
pub struct cmbdata {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn enable_cmf(cdev: *mut ccw_device) -> i32;
    pub fn disable_cmf(cdev: *mut ccw_device) -> i32;
    pub fn __disable_cmf(cdev: *mut ccw_device) -> i32;
    pub fn cmf_read(cdev: *mut ccw_device, index: i32) -> u64;
    pub fn cmf_readall(cdev: *mut ccw_device, data: *mut cmbdata) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
