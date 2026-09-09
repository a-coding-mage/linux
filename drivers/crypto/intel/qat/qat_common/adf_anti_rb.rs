// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2026 Intel Corporation */

// Dependencies supplied by the surrounding QAT driver translation unit:
// adf_accel_devices, adf_admin, adf_anti_rb, adf_common_drv, and
// icp_qat_fw_init_admin.

const ADF_SVN_RETRY_MAX: u32 = 60;

pub unsafe fn adf_anti_rb_commit(accel_dev: *mut adf_accel_dev) -> i32 {
    adf_send_admin_arb_commit(accel_dev)
}

pub unsafe fn adf_anti_rb_query(
    accel_dev: *mut adf_accel_dev,
    cmd: anti_rb,
    svn: *mut u8,
) -> i32 {
    adf_send_admin_arb_query(accel_dev, cmd, svn)
}

pub unsafe fn adf_anti_rb_check(pdev: *mut pci_dev) -> i32 {
    let anti_rb: *mut adf_anti_rb_hw_data;
    let mut svncheck_sts: u32;
    let cfc_svncheck_sts: u32;
    let accel_dev: *mut adf_accel_dev;
    let pmisc_addr: *mut core::ffi::c_void;

    accel_dev = adf_devmgr_pci_to_accel_dev(pdev);
    if accel_dev.is_null() {
        return -EINVAL;
    }

    anti_rb = GET_ANTI_RB_DATA(accel_dev);
    if !(*anti_rb).anti_rb_enabled || !((*anti_rb).anti_rb_enabled.unwrap())(accel_dev) {
        return 0;
    }

    pmisc_addr = adf_get_pmisc_base(accel_dev);

    cfc_svncheck_sts = ADF_CSR_RD(pmisc_addr, (*anti_rb).svncheck_offset);

    svncheck_sts = FIELD_GET(ADF_SVN_STS_MASK, cfc_svncheck_sts);
    match svncheck_sts {
        ADF_SVN_NO_STS => 0,
        ADF_SVN_PASS_STS => {
            (*anti_rb).svncheck_retry = 0;
            0
        }
        ADF_SVN_FAIL_STS => {
            dev_err(&GET_DEV(accel_dev), "Security Version Number failure\n");
            -EIO
        }
        ADF_SVN_RETRY_STS => {
            let retry = (*anti_rb).svncheck_retry;
            (*anti_rb).svncheck_retry = (*anti_rb).svncheck_retry.wrapping_add(1);
            if retry >= ADF_SVN_RETRY_MAX {
                (*anti_rb).svncheck_retry = 0;
                -ETIMEDOUT
            } else {
                msleep(ADF_SVN_RETRY_MS);
                -EAGAIN
            }
        }
        _ => {
            dev_err(&GET_DEV(accel_dev), "Invalid SVN check status\n");
            -EINVAL
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
