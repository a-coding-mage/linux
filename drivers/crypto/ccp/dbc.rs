// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD Secure Processor Dynamic Boost Control interface
 *
 * Copyright (C) 2023 Advanced Micro Devices, Inc.
 *
 * Author: Mario Limonciello <mario.limonciello@amd.com>
 */

// Dependencies supplied by the surrounding kernel/Rust translation.

const DBC_DEFAULT_TIMEOUT: u32 = 10 * MSEC_PER_SEC;

#[repr(C)]
struct ErrorMap {
    psp: u32,
    ret: i32,
}

const DBC_ERROR_ACCESS_DENIED: u32 = 0x0001;
const DBC_ERROR_EXCESS_DATA: u32 = 0x0004;
const DBC_ERROR_BAD_PARAMETERS: u32 = 0x0006;
const DBC_ERROR_BAD_STATE: u32 = 0x0007;
const DBC_ERROR_NOT_IMPLEMENTED: u32 = 0x0009;
const DBC_ERROR_BUSY: u32 = 0x000D;
const DBC_ERROR_MESSAGE_FAILURE: u32 = 0x0307;
const DBC_ERROR_OVERFLOW: u32 = 0x300F;
const DBC_ERROR_SIGNATURE_INVALID: u32 = 0x3072;

static ERROR_CODES: [ErrorMap; 10] = [
    ErrorMap { psp: DBC_ERROR_ACCESS_DENIED, ret: -EACCES },
    ErrorMap { psp: DBC_ERROR_EXCESS_DATA, ret: -E2BIG },
    ErrorMap { psp: DBC_ERROR_BAD_PARAMETERS, ret: -EINVAL },
    ErrorMap { psp: DBC_ERROR_BAD_STATE, ret: -EAGAIN },
    ErrorMap { psp: DBC_ERROR_MESSAGE_FAILURE, ret: -ENOENT },
    ErrorMap { psp: DBC_ERROR_NOT_IMPLEMENTED, ret: -ENOENT },
    ErrorMap { psp: DBC_ERROR_BUSY, ret: -EBUSY },
    ErrorMap { psp: DBC_ERROR_OVERFLOW, ret: -ENFILE },
    ErrorMap { psp: DBC_ERROR_SIGNATURE_INVALID, ret: -EPERM },
    ErrorMap { psp: 0, ret: 0 },
];

#[inline]
unsafe fn send_dbc_cmd_thru_ext(dbc_dev: *mut psp_dbc_device, msg: i32) -> i32 {
    (*(*dbc_dev).mbox).ext_req.header.sub_cmd_id = msg;
    psp_extended_mailbox_cmd(
        (*dbc_dev).psp,
        DBC_DEFAULT_TIMEOUT,
        &mut (*(*dbc_dev).mbox).ext_req as *mut _ as *mut psp_ext_request,
    )
}

#[inline]
unsafe fn send_dbc_cmd_thru_pa(dbc_dev: *mut psp_dbc_device, msg: i32) -> i32 {
    psp_send_platform_access_msg(
        msg,
        &mut (*(*dbc_dev).mbox).pa_req as *mut _ as *mut psp_request,
    )
}

unsafe fn send_dbc_cmd(dbc_dev: *mut psp_dbc_device, msg: i32) -> i32 {
    let mut ret: i32;
    *(*dbc_dev).result = 0;
    ret = if (*dbc_dev).use_ext {
        send_dbc_cmd_thru_ext(dbc_dev, msg)
    } else {
        send_dbc_cmd_thru_pa(dbc_dev, msg)
    };
    if ret == -EIO {
        dev_dbg((*dbc_dev).dev, "msg 0x%x failed with PSP error: 0x%x\n", msg, *(*dbc_dev).result);
        let mut i = 0;
        while ERROR_CODES[i].psp != 0 {
            if *(*dbc_dev).result == ERROR_CODES[i].psp {
                return ERROR_CODES[i].ret;
            }
            i += 1;
        }
    }
    ret
}

unsafe fn send_dbc_nonce(dbc_dev: *mut psp_dbc_device) -> i32 {
    *(*dbc_dev).payload_size = (*dbc_dev).header_size + core::mem::size_of::<dbc_user_nonce>();
    let mut ret = send_dbc_cmd(dbc_dev, PSP_DYNAMIC_BOOST_GET_NONCE);
    if ret == -EAGAIN {
        dev_dbg((*dbc_dev).dev, "retrying get nonce\n");
        ret = send_dbc_cmd(dbc_dev, PSP_DYNAMIC_BOOST_GET_NONCE);
    }
    ret
}

unsafe fn send_dbc_parameter(dbc_dev: *mut psp_dbc_device) -> i32 {
    let user_param = (*dbc_dev).payload as *mut dbc_user_param;
    match (*user_param).msg_index {
        PARAM_SET_FMAX_CAP | PARAM_SET_PWR_CAP | PARAM_SET_GFX_MODE => {
            send_dbc_cmd(dbc_dev, PSP_DYNAMIC_BOOST_SET_PARAMETER)
        }
        PARAM_GET_FMAX_CAP | PARAM_GET_PWR_CAP | PARAM_GET_CURR_TEMP |
        PARAM_GET_FMAX_MAX | PARAM_GET_FMAX_MIN | PARAM_GET_SOC_PWR_MAX |
        PARAM_GET_SOC_PWR_MIN | PARAM_GET_SOC_PWR_CUR | PARAM_GET_GFX_MODE => {
            send_dbc_cmd(dbc_dev, PSP_DYNAMIC_BOOST_GET_PARAMETER)
        }
        _ => -EINVAL,
    }
}

pub unsafe fn dbc_dev_destroy(psp: *mut psp_device) {
    let dbc_dev = (*psp).dbc_data;
    if dbc_dev.is_null() { return; }
    misc_deregister(&mut (*dbc_dev).char_dev);
    mutex_destroy(&mut (*dbc_dev).ioctl_mutex);
    (*psp).dbc_data = core::ptr::null_mut();
}

unsafe fn dbc_ioctl(filp: *mut file, cmd: u32, arg: usize) -> isize {
    let psp_master = psp_get_master_device();
    let argp = arg as *mut core::ffi::c_void;
    if psp_master.is_null() || (*psp_master).dbc_data.is_null() { return -ENODEV as isize; }
    let dbc_dev = (*psp_master).dbc_data;
    mutex_lock(&mut (*dbc_dev).ioctl_mutex);
    let result = match cmd {
        DBCIOCNONCE => {
            if copy_from_user((*dbc_dev).payload, argp, core::mem::size_of::<dbc_user_nonce>()) != 0 { -EFAULT }
            else { let ret = send_dbc_nonce(dbc_dev); if ret != 0 { ret } else if copy_to_user(argp, (*dbc_dev).payload, core::mem::size_of::<dbc_user_nonce>()) != 0 { -EFAULT } else { 0 } }
        }
        DBCIOCUID => {
            if copy_from_user((*dbc_dev).payload, argp, core::mem::size_of::<dbc_user_setuid>()) != 0 { -EFAULT }
            else { *(*dbc_dev).payload_size = (*dbc_dev).header_size + core::mem::size_of::<dbc_user_setuid>(); let ret = send_dbc_cmd(dbc_dev, PSP_DYNAMIC_BOOST_SET_UID); if ret != 0 { ret } else if copy_to_user(argp, (*dbc_dev).payload, core::mem::size_of::<dbc_user_setuid>()) != 0 { -EFAULT } else { 0 } }
        }
        DBCIOCPARAM => {
            if copy_from_user((*dbc_dev).payload, argp, core::mem::size_of::<dbc_user_param>()) != 0 { -EFAULT }
            else { *(*dbc_dev).payload_size = (*dbc_dev).header_size + core::mem::size_of::<dbc_user_param>(); let ret = send_dbc_parameter(dbc_dev); if ret != 0 { ret } else if copy_to_user(argp, (*dbc_dev).payload, core::mem::size_of::<dbc_user_param>()) != 0 { -EFAULT } else { 0 } }
        }
        _ => -EINVAL,
    };
    mutex_unlock(&mut (*dbc_dev).ioctl_mutex);
    result as isize
}

static DBC_FOPS: file_operations = file_operations {
    owner: THIS_MODULE,
    unlocked_ioctl: Some(dbc_ioctl),
};

pub unsafe fn dbc_dev_init(psp: *mut psp_device) -> i32 {
    let dev = (*psp).dev;
    let dbc_dev = devm_kzalloc(dev, core::mem::size_of::<psp_dbc_device>(), GFP_KERNEL) as *mut psp_dbc_device;
    if dbc_dev.is_null() { return -ENOMEM; }
    // BUILD_BUG_ON(sizeof(union dbc_buffer) > PAGE_SIZE);
    (*dbc_dev).mbox = devm_get_free_pages(dev, GFP_KERNEL | __GFP_ZERO, 0) as *mut dbc_buffer;
    if (*dbc_dev).mbox.is_null() { devm_kfree(dev, dbc_dev as *mut _); return -ENOMEM; }
    (*psp).dbc_data = dbc_dev;
    (*dbc_dev).dev = dev; (*dbc_dev).psp = psp;
    if (*psp).capability.dbc_thru_ext { (*dbc_dev).use_ext = true; (*dbc_dev).payload_size = &mut (*(*dbc_dev).mbox).ext_req.header.payload_size; (*dbc_dev).result = &mut (*(*dbc_dev).mbox).ext_req.header.status; (*dbc_dev).payload = &mut (*(*dbc_dev).mbox).ext_req.buf as *mut _ as *mut _; (*dbc_dev).header_size = core::mem::size_of::<psp_ext_req_buffer_hdr>(); }
    else { (*dbc_dev).payload_size = &mut (*(*dbc_dev).mbox).pa_req.header.payload_size; (*dbc_dev).result = &mut (*(*dbc_dev).mbox).pa_req.header.status; (*dbc_dev).payload = &mut (*(*dbc_dev).mbox).pa_req.buf as *mut _ as *mut _; (*dbc_dev).header_size = core::mem::size_of::<psp_req_buffer_hdr>(); }
    let mut ret = send_dbc_nonce(dbc_dev);
    if ret == -EACCES { dev_dbg((*dbc_dev).dev, "dynamic boost control was previously authenticated\n"); ret = 0; }
    dev_dbg((*dbc_dev).dev, "dynamic boost control is %savailable\n", if ret != 0 { "un" } else { "" });
    if ret != 0 { (*psp).dbc_data = core::ptr::null_mut(); devm_free_pages(dev, (*dbc_dev).mbox as usize); devm_kfree(dev, dbc_dev as *mut _); return 0; }
    (*dbc_dev).char_dev.minor = MISC_DYNAMIC_MINOR; (*dbc_dev).char_dev.name = b"dbc\0".as_ptr() as *const _; (*dbc_dev).char_dev.fops = &DBC_FOPS; (*dbc_dev).char_dev.mode = 0o600;
    ret = misc_register(&mut (*dbc_dev).char_dev); if ret != 0 { devm_free_pages(dev, (*dbc_dev).mbox as usize); (*psp).dbc_data = core::ptr::null_mut(); devm_kfree(dev, dbc_dev as *mut _); return ret; }
    mutex_init(&mut (*dbc_dev).ioctl_mutex);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
