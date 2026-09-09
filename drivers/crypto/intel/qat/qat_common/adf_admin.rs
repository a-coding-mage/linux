// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2014 - 2020 Intel Corporation */

// Kernel/project dependencies supplied by the surrounding translation unit.

const ADF_ADMIN_MAILBOX_STRIDE: usize = 0x1000;
const ADF_ADMINMSG_LEN: usize = 32;
const ADF_CONST_TABLE_SIZE: usize = 1024;
const ADF_ADMIN_POLL_DELAY_US: u32 = 20;
const ADF_ADMIN_POLL_TIMEOUT_US: u32 = 5 * USEC_PER_SEC;
const ADF_ONE_AE: u32 = 1;
const ADF_ADMIN_RETRY_MAX: u32 = 60;

#[repr(align(1024))]
struct AlignedConstTab([u8; 1024]);
static CONST_TAB: AlignedConstTab = AlignedConstTab([0; 1024]);

#[repr(C)]
struct adf_admin_comms {
    phy_addr: dma_addr_t,
    const_tbl_addr: dma_addr_t,
    virt_addr: *mut core::ffi::c_void,
    virt_tbl_addr: *mut core::ffi::c_void,
    mailbox_addr: *mut core::ffi::c_void,
    lock: mutex,
}

unsafe fn adf_put_admin_msg_sync(accel_dev: *mut adf_accel_dev, ae: u32,
                                 input: *mut core::ffi::c_void,
                                 output: *mut core::ffi::c_void) -> i32 {
    let admin = (*accel_dev).admin as *mut adf_admin_comms;
    let offset = ae as usize * ADF_ADMINMSG_LEN * 2;
    let mailbox = (*admin).mailbox_addr;
    let mb_offset = ae as usize * ADF_ADMIN_MAILBOX_STRIDE;
    let request = input as *mut icp_qat_fw_init_admin_req;
    mutex_lock(&mut (*admin).lock);
    if ADF_CSR_RD(mailbox, mb_offset) == 1 {
        mutex_unlock(&mut (*admin).lock);
        return -EAGAIN;
    }
    core::ptr::copy_nonoverlapping(input as *const u8,
        ((*admin).virt_addr as *mut u8).add(offset), ADF_ADMINMSG_LEN);
    ADF_CSR_WR(mailbox, mb_offset, 1);
    let mut status: u32 = 0;
    let ret = read_poll_timeout(ADF_CSR_RD, &mut status, status == 0,
        ADF_ADMIN_POLL_DELAY_US, ADF_ADMIN_POLL_TIMEOUT_US, true,
        mailbox, mb_offset);
    if ret < 0 {
        dev_err(&GET_DEV(accel_dev), "Failed to send admin msg {} to accelerator {}\n",
                (*request).cmd_id, ae);
    } else {
        core::ptr::copy_nonoverlapping(
            ((*admin).virt_addr as *const u8).add(offset + ADF_ADMINMSG_LEN),
            output as *mut u8, ADF_ADMINMSG_LEN);
    }
    mutex_unlock(&mut (*admin).lock);
    ret
}

unsafe fn adf_send_admin(accel_dev: *mut adf_accel_dev,
                         req: *mut icp_qat_fw_init_admin_req,
                         resp: *mut icp_qat_fw_init_admin_resp,
                         ae_mask: c_ulong) -> i32 {
    let mut ae = 0u32;
    while ae < ICP_QAT_HW_AE_DELIMITER {
        if (ae_mask & (1 as c_ulong).wrapping_shl(ae)) != 0 {
            if adf_put_admin_msg_sync(accel_dev, ae, req as _, resp as _) != 0 || (*resp).status != 0 { return -EFAULT; }
        }
        ae += 1;
    }
    0
}

unsafe fn adf_init_ae(accel_dev: *mut adf_accel_dev) -> i32 {
    let mut req: icp_qat_fw_init_admin_req = core::mem::zeroed();
    let mut resp: icp_qat_fw_init_admin_resp = core::mem::zeroed();
    req.cmd_id = ICP_QAT_FW_INIT_AE;
    adf_send_admin(accel_dev, &mut req, &mut resp, (*(*accel_dev).hw_device).ae_mask as _)
}

unsafe fn adf_set_fw_constants(accel_dev: *mut adf_accel_dev) -> i32 {
    let mut req: icp_qat_fw_init_admin_req = core::mem::zeroed();
    let mut resp: icp_qat_fw_init_admin_resp = core::mem::zeroed();
    let hw = (*accel_dev).hw_device;
    req.cmd_id = ICP_QAT_FW_CONSTANTS_CFG;
    req.init_cfg_sz = ADF_CONST_TABLE_SIZE;
    req.init_cfg_ptr = (*(*accel_dev).admin).const_tbl_addr;
    adf_send_admin(accel_dev, &mut req, &mut resp, if (*hw).admin_ae_mask != 0 { (*hw).admin_ae_mask } else { (*hw).ae_mask } as _)
}

pub unsafe fn adf_get_fw_timestamp(accel_dev: *mut adf_accel_dev, timestamp: *mut u64) -> i32 {
    let mut req: icp_qat_fw_init_admin_req = core::mem::zeroed();
    let mut resp: icp_qat_fw_init_admin_resp = core::mem::zeroed();
    req.cmd_id = ICP_QAT_FW_TIMER_GET;
    let ret = adf_send_admin(accel_dev, &mut req, &mut resp, ADF_ONE_AE as _);
    if ret != 0 { return ret; }
    *timestamp = resp.timestamp;
    0
}

// The remaining exported admin operations retain the C ABI and are declared
// against the corresponding translated kernel/project structures and helpers.
extern "C" {
    pub fn adf_send_admin_init(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_init_admin_pm(accel_dev: *mut adf_accel_dev, idle_delay: u32) -> i32;
    pub fn adf_init_admin_comms(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_exit_admin_comms(accel_dev: *mut adf_accel_dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
