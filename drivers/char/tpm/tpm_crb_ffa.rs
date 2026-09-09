// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2024 Arm Ltd.
 *
 * This device driver implements the TPM CRB start method
 * as defined in the TPM Service Command Response Buffer
 * Interface Over FF-A (DEN0138).
 */

// Dependency intent: Linux FF-A, delay, module parameter, and tpm_crb_ffa
// declarations are supplied by the surrounding kernel translation.

static mut busy_timeout_ms: u32 = 2000;

/* TPM service function status codes */
const CRB_FFA_OK: i32 = 0x05000001;
const CRB_FFA_OK_RESULTS_RETURNED: i32 = 0x05000002;
const CRB_FFA_NOFUNC: i32 = 0x8e000001u32 as i32;
const CRB_FFA_NOTSUP: i32 = 0x8e000002u32 as i32;
const CRB_FFA_INVARG: i32 = 0x8e000005u32 as i32;
const CRB_FFA_INV_CRB_CTRL_DATA: i32 = 0x8e000006u32 as i32;
const CRB_FFA_ALREADY: i32 = 0x8e000009u32 as i32;
const CRB_FFA_DENIED: i32 = 0x8e00000au32 as i32;
const CRB_FFA_NOMEM: i32 = 0x8e00000bu32 as i32;

const CRB_FFA_VERSION_MAJOR: u16 = 1;
const CRB_FFA_VERSION_MINOR: u16 = 0;

/* version encoding */
const CRB_FFA_MAJOR_VERSION_MASK: u32 = 0x7fff0000;
const CRB_FFA_MINOR_VERSION_MASK: u32 = 0x0000ffff;

#[inline]
fn CRB_FFA_MAJOR_VERSION(x: u32) -> u16 {
    ((x & CRB_FFA_MAJOR_VERSION_MASK) >> 16) as u16
}

#[inline]
fn CRB_FFA_MINOR_VERSION(x: u32) -> u16 {
    (x & CRB_FFA_MINOR_VERSION_MASK) as u16
}

/*
 * Normal world sends requests with FFA_MSG_SEND_DIRECT_REQ and
 * responses are returned with FFA_MSG_SEND_DIRECT_RESP for normal
 * messages. Register usage follows the FF-A specification.
 */

const CRB_FFA_GET_INTERFACE_VERSION: u64 = 0x0f000001;
const CRB_FFA_START: u64 = 0x0f000201;

#[repr(C)]
struct tpm_crb_ffa {
    ffa_dev: *mut ffa_device,
    major_version: u16,
    minor_version: u16,
    /* lock to protect sending of FF-A messages: */
    msg_data_lock: mutex,
    direct_msg_data: ffa_send_direct_data,
    direct_msg_data2: ffa_send_direct_data2,
}

static mut tpm_crb_ffa: *mut tpm_crb_ffa = core::ptr::null_mut();
static mut tpm_crb_ffa_driver: ffa_driver = ffa_driver::default();

unsafe fn tpm_crb_ffa_to_linux_errno(errno: i32) -> i32 {
    match errno {
        CRB_FFA_OK | CRB_FFA_OK_RESULTS_RETURNED => 0,
        CRB_FFA_NOFUNC => -ENOENT,
        CRB_FFA_NOTSUP => -EPERM,
        CRB_FFA_INVARG => -EINVAL,
        CRB_FFA_INV_CRB_CTRL_DATA => -ENOEXEC,
        CRB_FFA_ALREADY => -EEXIST,
        CRB_FFA_DENIED => -EACCES,
        CRB_FFA_NOMEM => -ENOMEM,
        _ => -EINVAL,
    }
}

/**
 * tpm_crb_ffa_init - called by the CRB driver to do any needed initialization
 *
 * This function is called by the tpm_crb driver during the tpm_crb
 * driver's initialization. If the tpm_crb_ffa has not been probed
 * yet, returns -ENOENT in order to force a retry. If the ffa_crb
 * driver had been probed but failed with an error, returns -ENODEV.
 *
 * Return: 0 on success, negative error code on failure.
 */
pub unsafe extern "C" fn tpm_crb_ffa_init() -> i32 {
    if tpm_crb_ffa.is_null() { return -ENOENT; }
    if IS_ERR_VALUE(tpm_crb_ffa as usize) { return -ENODEV; }
    0
}

unsafe fn __tpm_crb_ffa_try_send_receive(func_id: u64, a0: u64, a1: u64, a2: u64) -> i32 {
    let msg_ops = (*(*tpm_crb_ffa).ffa_dev).ops.msg_ops;
    if ffa_partition_supports_direct_req2_recv((*tpm_crb_ffa).ffa_dev) {
        (*tpm_crb_ffa).direct_msg_data2.data = [func_id, a0, a1, a2];
        let mut ret = (*msg_ops).sync_send_receive2((*tpm_crb_ffa).ffa_dev,
                                                     &mut (*tpm_crb_ffa).direct_msg_data2);
        if ret == 0 { ret = tpm_crb_ffa_to_linux_errno((*tpm_crb_ffa).direct_msg_data2.data[0] as i32); }
        ret
    } else {
        (*tpm_crb_ffa).direct_msg_data.data1 = func_id;
        (*tpm_crb_ffa).direct_msg_data.data2 = a0;
        (*tpm_crb_ffa).direct_msg_data.data3 = a1;
        (*tpm_crb_ffa).direct_msg_data.data4 = a2;
        let mut ret = (*msg_ops).sync_send_receive((*tpm_crb_ffa).ffa_dev,
                                                     &mut (*tpm_crb_ffa).direct_msg_data);
        if ret == 0 { ret = tpm_crb_ffa_to_linux_errno((*tpm_crb_ffa).direct_msg_data.data1 as i32); }
        ret
    }
}

unsafe fn __tpm_crb_ffa_send_receive(func_id: u64, a0: u64, a1: u64, a2: u64) -> i32 {
    if tpm_crb_ffa.is_null() { return -ENOENT; }
    let start = ktime_get();
    let stop = ktime_add(start, ms_to_ktime(busy_timeout_ms as u64));
    let mut ret;
    loop {
        ret = __tpm_crb_ffa_try_send_receive(func_id, a0, a1, a2);
        if ret != -EBUSY { break; }
        usleep_range(50, 100);
        if ktime_after(ktime_get(), stop) {
            dev_warn(&(*(*tpm_crb_ffa).ffa_dev).dev, "Busy retry timed out\n");
            break;
        }
    }
    ret
}

unsafe fn tpm_crb_ffa_get_interface_version(major: *mut u16, minor: *mut u16) -> i32 {
    if tpm_crb_ffa.is_null() { return -ENOENT; }
    if IS_ERR_VALUE(tpm_crb_ffa as usize) { return -ENODEV; }
    if major.is_null() || minor.is_null() { return -EINVAL; }
    let _guard = mutex_guard(&mut (*tpm_crb_ffa).msg_data_lock);
    let rc = __tpm_crb_ffa_send_receive(CRB_FFA_GET_INTERFACE_VERSION, 0, 0, 0);
    if rc == 0 {
        if ffa_partition_supports_direct_req2_recv((*tpm_crb_ffa).ffa_dev) {
            *major = CRB_FFA_MAJOR_VERSION((*tpm_crb_ffa).direct_msg_data2.data[1] as u32);
            *minor = CRB_FFA_MINOR_VERSION((*tpm_crb_ffa).direct_msg_data2.data[1] as u32);
        } else {
            *major = CRB_FFA_MAJOR_VERSION((*tpm_crb_ffa).direct_msg_data.data2 as u32);
            *minor = CRB_FFA_MINOR_VERSION((*tpm_crb_ffa).direct_msg_data.data2 as u32);
        }
    }
    rc
}

pub unsafe extern "C" fn tpm_crb_ffa_start(request_type: i32, locality: i32) -> i32 {
    if tpm_crb_ffa.is_null() { return -ENOENT; }
    if IS_ERR_VALUE(tpm_crb_ffa as usize) { return -ENODEV; }
    let _guard = mutex_guard(&mut (*tpm_crb_ffa).msg_data_lock);
    __tpm_crb_ffa_send_receive(CRB_FFA_START, request_type as u64, locality as u64, 0)
}

unsafe fn tpm_crb_ffa_probe(ffa_dev: *mut ffa_device) -> i32 {
    if !tpm_crb_ffa.is_null() && !IS_ERR_VALUE(tpm_crb_ffa as usize) { return -EEXIST; }
    tpm_crb_ffa = ERR_PTR(-ENODEV);
    if !ffa_partition_supports_direct_recv(ffa_dev) && !ffa_partition_supports_direct_req2_recv(ffa_dev) {
        dev_warn(&(*ffa_dev).dev, "partition doesn't support direct message receive.\n");
        return -EINVAL;
    }
    let p = kzalloc::<tpm_crb_ffa>();
    if p.is_null() { return -ENOMEM; }
    tpm_crb_ffa = p;
    mutex_init(&mut (*tpm_crb_ffa).msg_data_lock);
    (*tpm_crb_ffa).ffa_dev = ffa_dev;
    ffa_dev_set_drvdata(ffa_dev, tpm_crb_ffa);
    if !ffa_partition_check_property(ffa_dev, FFA_PARTITION_AARCH64_EXEC) {
        (*(*ffa_dev).ops).msg_ops.mode_32bit_set(ffa_dev);
    }
    let rc = tpm_crb_ffa_get_interface_version(&mut (*tpm_crb_ffa).major_version,
                                                &mut (*tpm_crb_ffa).minor_version);
    if rc != 0 { dev_err(&(*ffa_dev).dev, "failed to get crb interface version. rc:%d\n", rc); kfree(tpm_crb_ffa); tpm_crb_ffa = ERR_PTR(-ENODEV); return -EINVAL; }
    dev_info(&(*ffa_dev).dev, "ABI version %u.%u\n", (*tpm_crb_ffa).major_version, (*tpm_crb_ffa).minor_version);
    if (*tpm_crb_ffa).major_version != CRB_FFA_VERSION_MAJOR || ((*tpm_crb_ffa).minor_version > 0 && (*tpm_crb_ffa).minor_version < CRB_FFA_VERSION_MINOR) {
        dev_warn(&(*ffa_dev).dev, "Incompatible ABI version\n");
        kfree(tpm_crb_ffa); tpm_crb_ffa = ERR_PTR(-ENODEV); return -EINVAL;
    }
    0
}

unsafe fn tpm_crb_ffa_remove(_ffa_dev: *mut ffa_device) {
    kfree(tpm_crb_ffa);
    tpm_crb_ffa = core::ptr::null_mut();
}

static tpm_crb_ffa_device_id: [ffa_device_id; 2] = [
    UUID_INIT(0x17b862a4, 0x1806, 0x4faf, 0x86, 0xb3, 0x08, 0x9a, 0x58, 0x35, 0x38, 0x61),
    ffa_device_id::default(),
];

// Equivalent driver registration metadata is supplied by the kernel binding.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
