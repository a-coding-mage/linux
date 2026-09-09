/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2026 Intel Corporation */

// Dependency supplied by the surrounding kernel/Rust translation.

/// Equivalent of the C `GET_ANTI_RB_DATA(accel_dev)` macro.
#[macro_export]
macro_rules! GET_ANTI_RB_DATA {
    ($accel_dev:expr) => {
        unsafe { &(*$accel_dev).hw_device.anti_rb_data }
    };
}

pub const ADF_SVN_NO_STS: u32 = 0x00;
pub const ADF_SVN_PASS_STS: u32 = 0x01;
pub const ADF_SVN_RETRY_STS: u32 = 0x02;
pub const ADF_SVN_FAIL_STS: u32 = 0x03;
pub const ADF_SVN_RETRY_MS: u32 = 250;
pub const ADF_SVN_STS_MASK: u32 = 0xff;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum anti_rb {
    ARB_ENFORCED_MIN_SVN,
    ARB_PERMANENT_MIN_SVN,
    ARB_ACTIVE_SVN,
}

#[repr(C)]
pub struct adf_accel_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct adf_anti_rb_hw_data {
    pub anti_rb_enabled:
        Option<unsafe extern "C" fn(accel_dev: *mut adf_accel_dev) -> bool>,
    pub svncheck_offset: u32,
    pub svncheck_retry: u32,
    pub sysfs_added: bool,
}

unsafe extern "C" {
    pub fn adf_anti_rb_commit(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_anti_rb_query(
        accel_dev: *mut adf_accel_dev,
        cmd: anti_rb,
        svn: *mut u8,
    ) -> i32;
    pub fn adf_anti_rb_check(pdev: *mut pci_dev) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
