/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2023 Intel Corporation */

// Declarations corresponding to the Linux bitops and atomic dependencies are
// supplied by the surrounding translation unit.

#[repr(C)]
pub struct adf_accel_dev {
    _private: [u8; 0],
}

extern "C" {
    pub fn adf_sysfs_start_ras(accel_dev: *mut adf_accel_dev);
    pub fn adf_sysfs_stop_ras(accel_dev: *mut adf_accel_dev);
}

#[macro_export]
macro_rules! ADF_RAS_ERR_CTR_READ {
    ($ras_errors:expr, $err:expr) => {
        unsafe { (*($ras_errors).counter[$err].as_ptr()).load(::core::sync::atomic::Ordering::SeqCst) }
    };
}

#[macro_export]
macro_rules! ADF_RAS_ERR_CTR_CLEAR {
    ($ras_errors:expr) => {{
        let mut err = 0;
        while err < ADF_RAS_ERRORS {
            unsafe {
                (*($ras_errors).counter[err].as_ptr())
                    .store(0, ::core::sync::atomic::Ordering::SeqCst);
            }
            err += 1;
        }
    }};
}

#[macro_export]
macro_rules! ADF_RAS_ERR_CTR_INC {
    ($ras_errors:expr, $err:expr) => {
        unsafe {
            (*($ras_errors).counter[$err].as_ptr())
                .fetch_add(1, ::core::sync::atomic::Ordering::SeqCst);
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
