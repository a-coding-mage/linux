// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015, 2016 IBM Corporation
 * Copyright (C) 2016 Intel Corporation
 *
 * Author: Stefan Berger <stefanb@us.ibm.com>
 *
 * Maintained by: <tpmdd-devel@lists.sourceforge.net>
 *
 * Device driver for vTPM (vTPM proxy driver)
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

const VTPM_PROXY_REQ_COMPLETE_FLAG: u32 = BIT(0);

#[repr(C)]
struct proxy_dev {
    chip: *mut tpm_chip,
    flags: u32, // public API flags
    wq: wait_queue_head_t,
    buf_lock: mutex, // protect buffer and flags
    state: c_long, // internal state
    req_len: usize,
    resp_len: usize,
    buffer: [u8; TPM_BUFSIZE],
    work: work_struct,
}

const STATE_OPENED_FLAG: c_long = BIT(0);
const STATE_WAIT_RESPONSE_FLAG: c_long = BIT(1);
const STATE_REGISTERED_FLAG: c_long = BIT(2);
const STATE_DRIVER_COMMAND: c_long = BIT(3);
const VTPM_PROXY_FLAGS_ALL: u32 = VTPM_PROXY_FLAG_TPM2;

static mut workqueue: *mut workqueue_struct = core::ptr::null_mut();

unsafe extern "C" fn vtpm_proxy_delete_device(proxy_dev: *mut proxy_dev);

unsafe extern "C" fn vtpm_proxy_fops_read(
    filp: *mut file, buf: *mut c_char, count: usize, _off: *mut loff_t,
) -> ssize_t {
    let proxy_dev = (*filp).private_data as *mut proxy_dev;
    let sig = wait_event_interruptible((*proxy_dev).wq,
        (*proxy_dev).req_len != 0 || ((*proxy_dev).state & STATE_OPENED_FLAG) == 0);
    if sig != 0 { return -EINTR; }
    mutex_lock(&mut (*proxy_dev).buf_lock);
    if ((*proxy_dev).state & STATE_OPENED_FLAG) == 0 {
        mutex_unlock(&mut (*proxy_dev).buf_lock); return -EPIPE;
    }
    let len = (*proxy_dev).req_len;
    if count < len || len > core::mem::size_of_val(&(*proxy_dev).buffer) {
        mutex_unlock(&mut (*proxy_dev).buf_lock); return -EIO;
    }
    let rc = copy_to_user(buf as *mut c_void, (*proxy_dev).buffer.as_ptr() as *const c_void, len);
    memset((*proxy_dev).buffer.as_mut_ptr() as *mut c_void, 0, len);
    (*proxy_dev).req_len = 0;
    if rc == 0 { (*proxy_dev).state |= STATE_WAIT_RESPONSE_FLAG; }
    mutex_unlock(&mut (*proxy_dev).buf_lock);
    if rc != 0 { return -EFAULT; }
    len as ssize_t
}

unsafe extern "C" fn vtpm_proxy_fops_write(
    filp: *mut file, buf: *const c_char, count: usize, _off: *mut loff_t,
) -> ssize_t {
    let proxy_dev = (*filp).private_data as *mut proxy_dev;
    mutex_lock(&mut (*proxy_dev).buf_lock);
    if ((*proxy_dev).state & STATE_OPENED_FLAG) == 0 {
        mutex_unlock(&mut (*proxy_dev).buf_lock); return -EPIPE;
    }
    if count > core::mem::size_of_val(&(*proxy_dev).buffer)
        || ((*proxy_dev).state & STATE_WAIT_RESPONSE_FLAG) == 0 {
        mutex_unlock(&mut (*proxy_dev).buf_lock); return -EIO;
    }
    (*proxy_dev).state &= !STATE_WAIT_RESPONSE_FLAG;
    (*proxy_dev).req_len = 0;
    if copy_from_user((*proxy_dev).buffer.as_mut_ptr() as *mut c_void, buf as *const c_void, count) != 0 {
        mutex_unlock(&mut (*proxy_dev).buf_lock); return -EFAULT;
    }
    (*proxy_dev).resp_len = count;
    mutex_unlock(&mut (*proxy_dev).buf_lock);
    wake_up_interruptible(&mut (*proxy_dev).wq);
    count as ssize_t
}

unsafe extern "C" fn vtpm_proxy_fops_poll(filp: *mut file, wait: *mut poll_table) -> __poll_t {
    let proxy_dev = (*filp).private_data as *mut proxy_dev;
    poll_wait(filp, &mut (*proxy_dev).wq, wait);
    let mut ret = EPOLLOUT;
    mutex_lock(&mut (*proxy_dev).buf_lock);
    if (*proxy_dev).req_len != 0 { ret |= EPOLLIN | EPOLLRDNORM; }
    if ((*proxy_dev).state & STATE_OPENED_FLAG) == 0 { ret |= EPOLLHUP; }
    mutex_unlock(&mut (*proxy_dev).buf_lock);
    ret
}

unsafe extern "C" fn vtpm_proxy_fops_open(filp: *mut file) {
    let proxy_dev = (*filp).private_data as *mut proxy_dev;
    (*proxy_dev).state |= STATE_OPENED_FLAG;
}

unsafe extern "C" fn vtpm_proxy_fops_undo_open(proxy_dev: *mut proxy_dev) {
    mutex_lock(&mut (*proxy_dev).buf_lock);
    (*proxy_dev).state &= !STATE_OPENED_FLAG;
    mutex_unlock(&mut (*proxy_dev).buf_lock);
    wake_up_interruptible(&mut (*proxy_dev).wq);
}

unsafe extern "C" fn vtpm_proxy_fops_release(_inode: *mut inode, filp: *mut file) -> c_int {
    let proxy_dev = (*filp).private_data as *mut proxy_dev;
    (*filp).private_data = core::ptr::null_mut();
    vtpm_proxy_delete_device(proxy_dev);
    0
}

static vtpm_proxy_fops: file_operations = file_operations {
    owner: THIS_MODULE, read: Some(vtpm_proxy_fops_read), write: Some(vtpm_proxy_fops_write),
    poll: Some(vtpm_proxy_fops_poll), release: Some(vtpm_proxy_fops_release), ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn vtpm_proxy_tpm_op_recv(chip: *mut tpm_chip, buf: *mut u8, count: usize) -> c_int {
    let proxy_dev = dev_get_drvdata(&mut (*(*chip).dev));
    mutex_lock(&mut (*proxy_dev).buf_lock);
    if ((*proxy_dev).state & STATE_OPENED_FLAG) == 0 { mutex_unlock(&mut (*proxy_dev).buf_lock); return -EPIPE; }
    let len = (*proxy_dev).resp_len;
    if count < len { mutex_unlock(&mut (*proxy_dev).buf_lock); return -EIO; }
    memcpy(buf as *mut c_void, (*proxy_dev).buffer.as_ptr() as *const c_void, len);
    (*proxy_dev).resp_len = 0;
    mutex_unlock(&mut (*proxy_dev).buf_lock);
    len as c_int
}

unsafe extern "C" fn vtpm_proxy_is_driver_command(chip: *mut tpm_chip, buf: *mut u8, count: usize) -> c_int {
    if count < core::mem::size_of::<tpm_header>() { return 0; }
    let ordinal = be32_to_cpu((*(buf as *const tpm_header)).ordinal);
    if ((*chip).flags & TPM_CHIP_FLAG_TPM2) != 0 {
        (ordinal == TPM2_CC_SET_LOCALITY) as c_int
    } else { (ordinal == TPM_ORD_SET_LOCALITY) as c_int }
}

unsafe extern "C" fn vtpm_proxy_tpm_op_send(chip: *mut tpm_chip, buf: *mut u8, _bufsiz: usize, count: usize) -> c_int {
    let proxy_dev = dev_get_drvdata(&mut (*(*chip).dev));
    if count > core::mem::size_of_val(&(*proxy_dev).buffer) { return -EIO; }
    if ((*proxy_dev).state & STATE_DRIVER_COMMAND) == 0 && vtpm_proxy_is_driver_command(chip, buf, count) != 0 { return -EFAULT; }
    mutex_lock(&mut (*proxy_dev).buf_lock);
    if ((*proxy_dev).state & STATE_OPENED_FLAG) == 0 { mutex_unlock(&mut (*proxy_dev).buf_lock); return -EPIPE; }
    (*proxy_dev).resp_len = 0; (*proxy_dev).req_len = count;
    memcpy((*proxy_dev).buffer.as_mut_ptr() as *mut c_void, buf as *const c_void, count);
    (*proxy_dev).state &= !STATE_WAIT_RESPONSE_FLAG;
    mutex_unlock(&mut (*proxy_dev).buf_lock); wake_up_interruptible(&mut (*proxy_dev).wq); 0
}

unsafe extern "C" fn vtpm_proxy_tpm_op_cancel(_chip: *mut tpm_chip) { }
unsafe extern "C" fn vtpm_proxy_tpm_op_status(chip: *mut tpm_chip) -> u8 {
    let p = dev_get_drvdata(&mut (*(*chip).dev));
    if (*p).resp_len != 0 { VTPM_PROXY_REQ_COMPLETE_FLAG as u8 } else { 0 }
}
unsafe extern "C" fn vtpm_proxy_tpm_req_canceled(chip: *mut tpm_chip, _status: u8) -> bool {
    let p = dev_get_drvdata(&mut (*(*chip).dev)); mutex_lock(&mut (*p).buf_lock);
    let ret = ((*p).state & STATE_OPENED_FLAG) == 0; mutex_unlock(&mut (*p).buf_lock); ret
}

unsafe extern "C" fn vtpm_proxy_request_locality(chip: *mut tpm_chip, mut locality: c_int) -> c_int {
    let p = dev_get_drvdata(&mut (*(*chip).dev));
    let buf = kzalloc(TPM_BUFSIZE, GFP_KERNEL) as *mut tpm_buf;
    if buf.is_null() { return -ENOMEM; }
    tpm_buf_init(buf, TPM_BUFSIZE);
    if ((*chip).flags & TPM_CHIP_FLAG_TPM2) != 0 { tpm_buf_reset(buf, TPM2_ST_SESSIONS, TPM2_CC_SET_LOCALITY); }
    else { tpm_buf_reset(buf, TPM_TAG_RQU_COMMAND, TPM_ORD_SET_LOCALITY); }
    tpm_buf_append_u8(buf, locality as u8); (*p).state |= STATE_DRIVER_COMMAND;
    let rc = tpm_transmit_cmd(chip, buf, 0, c"attempting to set locality".as_ptr());
    (*p).state &= !STATE_DRIVER_COMMAND;
    if rc < 0 { kfree(buf as *mut c_void); return rc; }
    let header = (*buf).data as *const tpm_header;
    if be32_to_cpu((*header).return_code) != 0 { locality = -1; }
    kfree(buf as *mut c_void); locality
}

static vtpm_proxy_tpm_ops: tpm_class_ops = tpm_class_ops {
    flags: TPM_OPS_AUTO_STARTUP, recv: Some(vtpm_proxy_tpm_op_recv), send: Some(vtpm_proxy_tpm_op_send),
    cancel: Some(vtpm_proxy_tpm_op_cancel), status: Some(vtpm_proxy_tpm_op_status),
    req_complete_mask: VTPM_PROXY_REQ_COMPLETE_FLAG, req_complete_val: VTPM_PROXY_REQ_COMPLETE_FLAG,
    req_canceled: Some(vtpm_proxy_tpm_req_canceled), request_locality: Some(vtpm_proxy_request_locality), ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn vtpm_proxy_work(work: *mut work_struct) {
    let p = container_of!(work, proxy_dev, work); let rc = tpm_chip_register((*p).chip);
    if rc != 0 { vtpm_proxy_fops_undo_open(p); } else { (*p).state |= STATE_REGISTERED_FLAG; }
}
unsafe extern "C" fn vtpm_proxy_work_stop(p: *mut proxy_dev) { vtpm_proxy_fops_undo_open(p); flush_work(&mut (*p).work); }
unsafe extern "C" fn vtpm_proxy_work_start(p: *mut proxy_dev) { queue_work(workqueue, &mut (*p).work); }

// Device creation/deletion and control-device ioctl paths retain the kernel API
// declarations and ordering from the C implementation.
unsafe extern "C" fn vtpm_proxy_create_proxy_dev() -> *mut proxy_dev {
    let p = kzalloc(core::mem::size_of::<proxy_dev>(), GFP_KERNEL) as *mut proxy_dev;
    if p.is_null() { return ERR_PTR(-ENOMEM); }
    init_waitqueue_head(&mut (*p).wq); mutex_init(&mut (*p).buf_lock); INIT_WORK(&mut (*p).work, vtpm_proxy_work);
    let chip = tpm_chip_alloc(core::ptr::null_mut(), &vtpm_proxy_tpm_ops);
    if IS_ERR(chip) { let e = PTR_ERR(chip); kfree(p as *mut c_void); return ERR_PTR(e); }
    dev_set_drvdata(&mut (*chip).dev, p); (*p).chip = chip; p
}
unsafe extern "C" fn vtpm_proxy_delete_proxy_dev(p: *mut proxy_dev) { put_device(&mut (*(*p).chip).dev); kfree(p as *mut c_void); }
unsafe extern "C" fn vtpm_proxy_delete_device(p: *mut proxy_dev) {
    vtpm_proxy_work_stop(p); vtpm_proxy_fops_undo_open(p);
    if ((*p).state & STATE_REGISTERED_FLAG) != 0 { tpm_chip_unregister((*p).chip); }
    vtpm_proxy_delete_proxy_dev(p);
}

unsafe extern "C" fn vtpm_proxy_create_device(n: *mut vtpm_proxy_new_dev) -> *mut file {
    if ((*n).flags & !VTPM_PROXY_FLAGS_ALL) != 0 { return ERR_PTR(-EOPNOTSUPP); }
    let p = vtpm_proxy_create_proxy_dev(); if IS_ERR(p) { return ERR_CAST(p); }
    (*p).flags = (*n).flags;
    let fd = get_unused_fd_flags(O_RDWR); if fd < 0 { vtpm_proxy_delete_proxy_dev(p); return ERR_PTR(fd); }
    let f = anon_inode_getfile(c"[vtpms]".as_ptr(), &vtpm_proxy_fops, p as *mut c_void, O_RDWR);
    if IS_ERR(f) { put_unused_fd(fd); vtpm_proxy_delete_proxy_dev(p); return f; }
    vtpm_proxy_fops_open(f);
    if ((*p).flags & VTPM_PROXY_FLAG_TPM2) != 0 { (*(*p).chip).flags |= TPM_CHIP_FLAG_TPM2; }
    vtpm_proxy_work_start(p);
    (*n).fd = fd; (*n).major = MAJOR((*(*p).chip).dev.devt); (*n).minor = MINOR((*(*p).chip).dev.devt); (*n).tpm_num = (*p).chip_num;
    f
}

unsafe extern "C" fn vtpmx_ioc_new_dev(_file: *mut file, _ioctl: c_uint, arg: c_ulong) -> c_long {
    if !capable(CAP_SYS_ADMIN) { return -EPERM; }
    let p = arg as *mut vtpm_proxy_new_dev; let mut n: vtpm_proxy_new_dev = core::mem::zeroed();
    if copy_from_user(&mut n as *mut _ as *mut c_void, p as *const c_void, core::mem::size_of::<vtpm_proxy_new_dev>()) != 0 { return -EFAULT; }
    let f = vtpm_proxy_create_device(&mut n); if IS_ERR(f) { return PTR_ERR(f); }
    if copy_to_user(p as *mut c_void, &n as *const _ as *const c_void, core::mem::size_of::<vtpm_proxy_new_dev>()) != 0 {
        put_unused_fd(n.fd); fput(f); return -EFAULT;
    }
    fd_install(n.fd, f); 0
}
unsafe extern "C" fn vtpmx_fops_ioctl(f: *mut file, ioctl: c_uint, arg: c_ulong) -> c_long {
    match ioctl { VTPM_PROXY_IOC_NEW_DEV => vtpmx_ioc_new_dev(f, ioctl, arg), _ => -ENOIOCTLCMD }
}
static vtpmx_fops: file_operations = file_operations {
    owner: THIS_MODULE, unlocked_ioctl: Some(vtpmx_fops_ioctl), compat_ioctl: Some(compat_ptr_ioctl), llseek: Some(noop_llseek), ..unsafe { core::mem::zeroed() }
};
static mut vtpmx_miscdev: miscdevice = miscdevice { minor: MISC_DYNAMIC_MINOR, name: c"vtpmx".as_ptr(), fops: &vtpmx_fops, ..unsafe { core::mem::zeroed() } };
unsafe extern "C" fn vtpm_module_init() -> c_int {
    workqueue = create_workqueue(c"tpm-vtpm".as_ptr());
    if workqueue.is_null() { return -ENOMEM; }
    let rc = misc_register(&mut vtpmx_miscdev); if rc != 0 { destroy_workqueue(workqueue); } rc
}
unsafe extern "C" fn vtpm_module_exit() { destroy_workqueue(workqueue); misc_deregister(&mut vtpmx_miscdev); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
