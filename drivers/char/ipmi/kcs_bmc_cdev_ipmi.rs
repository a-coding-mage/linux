// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2015-2018, Intel Corporation.
 */

// pr_fmt(fmt) = "kcs-bmc: " fmt

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

// Kernel-provided types and functions are supplied by the surrounding repository.
use crate::*;

#[repr(C)]
#[derive(Copy, Clone)]
enum KcsIpmiPhases {
    KcsPhaseIdle,
    KcsPhaseWriteStart,
    KcsPhaseWriteData,
    KcsPhaseWriteEndCmd,
    KcsPhaseWriteDone,
    KcsPhaseWaitRead,
    KcsPhaseRead,
    KcsPhaseAbortError1,
    KcsPhaseAbortError2,
    KcsPhaseError,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum KcsIpmiErrors {
    KcsNoError = 0x00,
    KcsAbortedByCommand = 0x01,
    KcsIllegalControlCode = 0x02,
    KcsLengthError = 0x06,
    KcsUnspecifiedError = 0xff,
}

#[repr(C)]
struct KcsBmcIpmi {
    entry: list_head,
    client: kcs_bmc_client,
    lock: spinlock_t,
    phase: KcsIpmiPhases,
    error: KcsIpmiErrors,
    queue: wait_queue_head_t,
    data_in_avail: bool,
    data_in_idx: c_int,
    data_in: *mut u8,
    data_out_idx: c_int,
    data_out_len: c_int,
    data_out: *mut u8,
    mutex: mutex,
    kbuffer: *mut u8,
    miscdev: miscdevice,
}

const DEVICE_NAME: &[u8] = b"ipmi-kcs\0";
const KCS_MSG_BUFSIZ: usize = 1000;
const KCS_ZERO_DATA: u8 = 0;
const KCS_STATUS_STATE_MASK: u8 = 0xc0;
const KCS_STATUS_CMD_DAT: u8 = 1 << 3;
const KCS_STATUS_SMS_ATN: u8 = 1 << 2;
const KCS_STATUS_IBF: u8 = 1 << 1;
const KCS_STATUS_OBF: u8 = 1;

#[inline]
const fn kcs_status_state(state: u8) -> u8 { state << 6 }

#[repr(C)]
#[derive(Copy, Clone)]
enum KcsStates { IdleState = 0, ReadState = 1, WriteState = 2, ErrorState = 3 }

const KCS_CMD_GET_STATUS_ABORT: u8 = 0x60;
const KCS_CMD_WRITE_START: u8 = 0x61;
const KCS_CMD_WRITE_END: u8 = 0x62;
const KCS_CMD_READ_BYTE: u8 = 0x68;

#[inline]
unsafe fn set_state(priv_: *mut KcsBmcIpmi, state: u8) {
    kcs_bmc_update_status((*priv_).client.dev, KCS_STATUS_STATE_MASK, kcs_status_state(state));
}

unsafe fn kcs_bmc_ipmi_force_abort(priv_: *mut KcsBmcIpmi) {
    set_state(priv_, KcsStates::ErrorState as u8);
    kcs_bmc_read_data((*priv_).client.dev);
    kcs_bmc_write_data((*priv_).client.dev, KCS_ZERO_DATA);
    (*priv_).phase = KcsIpmiPhases::KcsPhaseError;
    (*priv_).data_in_avail = false;
    (*priv_).data_in_idx = 0;
}

unsafe fn kcs_bmc_ipmi_handle_data(priv_: *mut KcsBmcIpmi) {
    let dev = (*priv_).client.dev;
    match (*priv_).phase {
        KcsIpmiPhases::KcsPhaseWriteStart => {
            (*priv_).phase = KcsIpmiPhases::KcsPhaseWriteData;
            kcs_bmc_ipmi_handle_data(priv_);
        }
        KcsIpmiPhases::KcsPhaseWriteData => {
            if (*priv_).data_in_idx < KCS_MSG_BUFSIZ as c_int {
                set_state(priv_, KcsStates::WriteState as u8);
                kcs_bmc_write_data(dev, KCS_ZERO_DATA);
                *(*priv_).data_in.add((*priv_).data_in_idx as usize) = kcs_bmc_read_data(dev);
                (*priv_).data_in_idx += 1;
            } else { kcs_bmc_ipmi_force_abort(priv_); (*priv_).error = KcsIpmiErrors::KcsLengthError; }
        }
        KcsIpmiPhases::KcsPhaseWriteEndCmd => {
            if (*priv_).data_in_idx < KCS_MSG_BUFSIZ as c_int {
                set_state(priv_, KcsStates::ReadState as u8);
                *(*priv_).data_in.add((*priv_).data_in_idx as usize) = kcs_bmc_read_data(dev);
                (*priv_).data_in_idx += 1;
                (*priv_).phase = KcsIpmiPhases::KcsPhaseWriteDone;
                (*priv_).data_in_avail = true;
                wake_up_interruptible(&mut (*priv_).queue);
            } else { kcs_bmc_ipmi_force_abort(priv_); (*priv_).error = KcsIpmiErrors::KcsLengthError; }
        }
        KcsIpmiPhases::KcsPhaseRead => {
            if (*priv_).data_out_idx == (*priv_).data_out_len { set_state(priv_, KcsStates::IdleState as u8); }
            let data = kcs_bmc_read_data(dev);
            if data != KCS_CMD_READ_BYTE { set_state(priv_, KcsStates::ErrorState as u8); kcs_bmc_write_data(dev, KCS_ZERO_DATA); return; }
            if (*priv_).data_out_idx == (*priv_).data_out_len {
                kcs_bmc_write_data(dev, KCS_ZERO_DATA); (*priv_).phase = KcsIpmiPhases::KcsPhaseIdle; return;
            }
            let out = *(*priv_).data_out.add((*priv_).data_out_idx as usize);
            (*priv_).data_out_idx += 1;
            kcs_bmc_write_data(dev, out);
        }
        KcsIpmiPhases::KcsPhaseAbortError1 => {
            set_state(priv_, KcsStates::ReadState as u8); kcs_bmc_read_data(dev);
            kcs_bmc_write_data(dev, (*priv_).error as u8); (*priv_).phase = KcsIpmiPhases::KcsPhaseAbortError2;
        }
        KcsIpmiPhases::KcsPhaseAbortError2 => {
            set_state(priv_, KcsStates::IdleState as u8); kcs_bmc_read_data(dev);
            kcs_bmc_write_data(dev, KCS_ZERO_DATA); (*priv_).phase = KcsIpmiPhases::KcsPhaseIdle;
        }
        _ => kcs_bmc_ipmi_force_abort(priv_),
    }
}

unsafe fn kcs_bmc_ipmi_handle_cmd(priv_: *mut KcsBmcIpmi) {
    set_state(priv_, KcsStates::WriteState as u8);
    kcs_bmc_write_data((*priv_).client.dev, KCS_ZERO_DATA);
    match kcs_bmc_read_data((*priv_).client.dev) {
        KCS_CMD_WRITE_START => { (*priv_).phase = KcsIpmiPhases::KcsPhaseWriteStart; (*priv_).error = KcsIpmiErrors::KcsNoError; (*priv_).data_in_avail = false; (*priv_).data_in_idx = 0; }
        KCS_CMD_WRITE_END => { if (*priv_).phase != KcsIpmiPhases::KcsPhaseWriteData { kcs_bmc_ipmi_force_abort(priv_); } else { (*priv_).phase = KcsIpmiPhases::KcsPhaseWriteEndCmd; } }
        KCS_CMD_GET_STATUS_ABORT => { if (*priv_).error == KcsIpmiErrors::KcsNoError { (*priv_).error = KcsIpmiErrors::KcsAbortedByCommand; } (*priv_).phase = KcsIpmiPhases::KcsPhaseAbortError1; (*priv_).data_in_avail = false; (*priv_).data_in_idx = 0; }
        _ => { kcs_bmc_ipmi_force_abort(priv_); (*priv_).error = KcsIpmiErrors::KcsIllegalControlCode; }
    }
}

#[inline]
unsafe fn client_to_kcs_bmc_ipmi(client: *mut kcs_bmc_client) -> *mut KcsBmcIpmi {
    container_of!(client, KcsBmcIpmi, client)
}

unsafe extern "C" fn kcs_bmc_ipmi_event(client: *mut kcs_bmc_client) -> irqreturn_t {
    let priv_ = client_to_kcs_bmc_ipmi(client); if priv_.is_null() { return IRQ_NONE; }
    spin_lock(&mut (*priv_).lock);
    let status = kcs_bmc_read_status((*client).dev);
    let ret = if status & KCS_STATUS_IBF != 0 { if status & KCS_STATUS_CMD_DAT != 0 { kcs_bmc_ipmi_handle_cmd(priv_); } else { kcs_bmc_ipmi_handle_data(priv_); } IRQ_HANDLED } else { IRQ_NONE };
    spin_unlock(&mut (*priv_).lock); ret
}

static KCS_BMC_IPMI_CLIENT_OPS: kcs_bmc_client_ops = kcs_bmc_client_ops { event: Some(kcs_bmc_ipmi_event) };

unsafe extern "C" fn kcs_bmc_ipmi_open(_inode: *mut inode, filp: *mut file) -> c_int {
    let priv_ = container_of!((*filp).private_data, KcsBmcIpmi, miscdev);
    kcs_bmc_enable_device((*priv_).client.dev, &mut (*priv_).client)
}

unsafe extern "C" fn kcs_bmc_ipmi_poll(filp: *mut file, wait: *mut poll_table) -> c_ulong {
    let priv_ = container_of!((*filp).private_data, KcsBmcIpmi, miscdev);
    poll_wait(filp, &mut (*priv_).queue, wait);
    spin_lock_irq(&mut (*priv_).lock);
    let mask = if (*priv_).data_in_avail { EPOLLIN as c_ulong } else { 0 };
    spin_unlock_irq(&mut (*priv_).lock); mask
}

unsafe extern "C" fn kcs_bmc_ipmi_read(filp: *mut file, buf: *mut c_void, count: usize, _ppos: *mut loff_t) -> isize {
    let priv_ = container_of!((*filp).private_data, KcsBmcIpmi, miscdev);
    if (*filp).f_flags & O_NONBLOCK == 0 { wait_event_interruptible(&mut (*priv_).queue, (*priv_).data_in_avail); }
    mutex_lock(&mut (*priv_).mutex);
    spin_lock_irq(&mut (*priv_).lock);
    let avail = (*priv_).data_in_avail;
    let len = (*priv_).data_in_idx as usize;
    if avail { core::ptr::copy_nonoverlapping((*priv_).data_in, (*priv_).kbuffer, len); }
    spin_unlock_irq(&mut (*priv_).lock);
    if !avail { mutex_unlock(&mut (*priv_).mutex); return -EAGAIN as isize; }
    if count < len { spin_lock_irq(&mut (*priv_).lock); kcs_bmc_ipmi_force_abort(priv_); spin_unlock_irq(&mut (*priv_).lock); mutex_unlock(&mut (*priv_).mutex); return -EOVERFLOW as isize; }
    if copy_to_user(buf, (*priv_).kbuffer, len) != 0 { mutex_unlock(&mut (*priv_).mutex); return -EFAULT as isize; }
    spin_lock_irq(&mut (*priv_).lock);
    let ret = if (*priv_).phase == KcsIpmiPhases::KcsPhaseWriteDone { (*priv_).phase = KcsIpmiPhases::KcsPhaseWaitRead; (*priv_).data_in_avail = false; (*priv_).data_in_idx = 0; len as isize } else { -EAGAIN as isize };
    spin_unlock_irq(&mut (*priv_).lock); mutex_unlock(&mut (*priv_).mutex); ret
}

unsafe extern "C" fn kcs_bmc_ipmi_write(filp: *mut file, buf: *const c_void, count: usize, _ppos: *mut loff_t) -> isize {
    let priv_ = container_of!((*filp).private_data, KcsBmcIpmi, miscdev);
    if count < 3 || count > KCS_MSG_BUFSIZ { return -EINVAL as isize; }
    mutex_lock(&mut (*priv_).mutex);
    if copy_from_user((*priv_).kbuffer, buf, count) != 0 { mutex_unlock(&mut (*priv_).mutex); return -EFAULT as isize; }
    spin_lock_irq(&mut (*priv_).lock);
    let ret = if (*priv_).phase == KcsIpmiPhases::KcsPhaseWaitRead {
        (*priv_).phase = KcsIpmiPhases::KcsPhaseRead; (*priv_).data_out_idx = 1; (*priv_).data_out_len = count as c_int;
        core::ptr::copy_nonoverlapping((*priv_).kbuffer, (*priv_).data_out, count);
        kcs_bmc_write_data((*priv_).client.dev, *(*priv_).data_out); count as isize
    } else { -EINVAL as isize };
    spin_unlock_irq(&mut (*priv_).lock); mutex_unlock(&mut (*priv_).mutex); ret
}

unsafe extern "C" fn kcs_bmc_ipmi_ioctl(filp: *mut file, cmd: c_uint, _arg: c_ulong) -> c_long {
    let priv_ = container_of!((*filp).private_data, KcsBmcIpmi, miscdev); spin_lock_irq(&mut (*priv_).lock);
    let ret = match cmd { IPMI_BMC_IOCTL_SET_SMS_ATN => { kcs_bmc_update_status((*priv_).client.dev, KCS_STATUS_SMS_ATN, KCS_STATUS_SMS_ATN); 0 }, IPMI_BMC_IOCTL_CLEAR_SMS_ATN => { kcs_bmc_update_status((*priv_).client.dev, KCS_STATUS_SMS_ATN, 0); 0 }, IPMI_BMC_IOCTL_FORCE_ABORT => { kcs_bmc_ipmi_force_abort(priv_); 0 }, _ => -EINVAL as c_long };
    spin_unlock_irq(&mut (*priv_).lock); ret
}

unsafe extern "C" fn kcs_bmc_ipmi_release(_inode: *mut inode, filp: *mut file) -> c_int {
    let priv_ = container_of!((*filp).private_data, KcsBmcIpmi, miscdev); kcs_bmc_ipmi_force_abort(priv_); kcs_bmc_disable_device((*priv_).client.dev, &mut (*priv_).client); 0
}

static KCS_BMC_IPMI_FOPS: file_operations = file_operations { owner: THIS_MODULE, open: Some(kcs_bmc_ipmi_open), read: Some(kcs_bmc_ipmi_read), write: Some(kcs_bmc_ipmi_write), release: Some(kcs_bmc_ipmi_release), poll: Some(kcs_bmc_ipmi_poll), unlocked_ioctl: Some(kcs_bmc_ipmi_ioctl) };

unsafe extern "C" fn kcs_bmc_ipmi_add_device(kcs_bmc: *mut kcs_bmc_device) -> c_int {
    let priv_ = devm_kzalloc((*kcs_bmc).dev, core::mem::size_of::<KcsBmcIpmi>(), GFP_KERNEL) as *mut KcsBmcIpmi;
    if priv_.is_null() { return -ENOMEM; }
    spin_lock_init(&mut (*priv_).lock); mutex_init(&mut (*priv_).mutex); init_waitqueue_head(&mut (*priv_).queue);
    (*priv_).client.dev = kcs_bmc; (*priv_).client.ops = &KCS_BMC_IPMI_CLIENT_OPS;
    (*priv_).data_in = devm_kmalloc((*kcs_bmc).dev, KCS_MSG_BUFSIZ, GFP_KERNEL);
    (*priv_).data_out = devm_kmalloc((*kcs_bmc).dev, KCS_MSG_BUFSIZ, GFP_KERNEL);
    (*priv_).kbuffer = devm_kmalloc((*kcs_bmc).dev, KCS_MSG_BUFSIZ, GFP_KERNEL);
    (*priv_).miscdev.minor = MISC_DYNAMIC_MINOR; (*priv_).miscdev.name = devm_kasprintf((*kcs_bmc).dev, GFP_KERNEL, DEVICE_NAME.as_ptr(), (*kcs_bmc).channel); (*priv_).miscdev.fops = &KCS_BMC_IPMI_FOPS;
    if (*priv_).data_in.is_null() || (*priv_).data_out.is_null() || (*priv_).kbuffer.is_null() || (*priv_).miscdev.name.is_null() { return -EINVAL; }
    let rc = misc_register(&mut (*priv_).miscdev); if rc != 0 { return rc; }
    list_add(&mut (*priv_).entry, &mut KCS_BMC_IPMI_INSTANCES); 0
}

unsafe extern "C" fn kcs_bmc_ipmi_remove_device(kcs_bmc: *mut kcs_bmc_device) -> c_int {
    let mut pos = KCS_BMC_IPMI_INSTANCES.next as *mut KcsBmcIpmi;
    while !pos.is_null() && pos != (&mut KCS_BMC_IPMI_INSTANCES as *mut _ as *mut KcsBmcIpmi) { if (*pos).client.dev == kcs_bmc { list_del(&mut (*pos).entry); misc_deregister(&mut (*pos).miscdev); kcs_bmc_disable_device((*pos).client.dev, &mut (*pos).client); return 0; } pos = (*pos).entry.next as *mut KcsBmcIpmi; }
    -ENODEV
}

static KCS_BMC_IPMI_DRIVER_OPS: kcs_bmc_driver_ops = kcs_bmc_driver_ops { add_device: Some(kcs_bmc_ipmi_add_device), remove_device: Some(kcs_bmc_ipmi_remove_device) };
static mut KCS_BMC_IPMI_DRIVER: kcs_bmc_driver = kcs_bmc_driver { ops: &KCS_BMC_IPMI_DRIVER_OPS };
static mut KCS_BMC_IPMI_INSTANCES: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

#[no_mangle]
pub unsafe extern "C" fn kcs_bmc_ipmi_init() -> c_int {
    kcs_bmc_register_driver(&mut KCS_BMC_IPMI_DRIVER); 0
}

#[no_mangle]
pub unsafe extern "C" fn kcs_bmc_ipmi_exit() {
    kcs_bmc_unregister_driver(&mut KCS_BMC_IPMI_DRIVER);
}

// MODULE_LICENSE("GPL v2");
// MODULE_AUTHOR("Haiyue Wang <haiyue.wang@linux.intel.com>");
// MODULE_AUTHOR("Andrew Jeffery <andrew@aj.id.au>");
// MODULE_DESCRIPTION("KCS BMC to handle the IPMI request from system software");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
