// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *       An implementation of a loadable kernel mode driver providing
 *       multiple kernel/user space bidirectional communications links.
 *
 *       Adapted to become the Linux 2.0 Coda pseudo device
 *       Changes for Linux 2.1
 */

// Linux kernel headers and the local Coda headers are supplied by the
// surrounding translation unit.

/* statistics */
pub static mut coda_hard: ::core::ffi::c_int = 0;
pub static mut coda_timeout: ::core::ffi::c_ulong = 30;

pub static mut coda_comms: [venus_comm; MAX_CODADEVS] = [venus_comm::ZERO; MAX_CODADEVS];
static mut coda_psdev_class: *mut class = core::ptr::null_mut();

unsafe fn coda_psdev_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    let vcp = (*file).private_data as *mut venus_comm;
    let mut mask: __poll_t = EPOLLOUT | EPOLLWRNORM;

    poll_wait(file, &mut (*vcp).vc_waitq, wait);
    mutex_lock(&mut (*vcp).vc_mutex);
    if !list_empty(&(*vcp).vc_pending) {
        mask |= EPOLLIN | EPOLLRDNORM;
    }
    mutex_unlock(&mut (*vcp).vc_mutex);
    mask
}

unsafe fn coda_psdev_ioctl(_filp: *mut file, cmd: ::core::ffi::c_uint, arg: ::core::ffi::c_ulong) -> ::core::ffi::c_long {
    let mut data: ::core::ffi::c_uint;
    match cmd {
        CIOC_KERNEL_VERSION => {
            data = CODA_KERNEL_VERSION;
            put_user(data, arg as *mut ::core::ffi::c_int)
        }
        _ => -ENOTTY,
    }
}

/* Receive a message written by Venus to the psdev. */
unsafe fn coda_psdev_write(file: *mut file, buf: *const ::core::ffi::c_char, mut nbytes: usize, _off: *mut loff_t) -> isize {
    let vcp = (*file).private_data as *mut venus_comm;
    let mut req: *mut upc_req = core::ptr::null_mut();
    let mut hdr: coda_in_hdr = core::mem::zeroed();
    let mut retval: isize = 0;
    let mut count: isize = 0;
    let mut error: ::core::ffi::c_int;

    if nbytes < 2 * core::mem::size_of::<u_int32_t>() { return -EINVAL; }
    if copy_from_user(&mut hdr as *mut _ as *mut _, buf, 2 * core::mem::size_of::<u_int32_t>()) != 0 { return -EFAULT; }

    if DOWNCALL(hdr.opcode) {
        let size = core::mem::size_of::<outputArgs>();
        if nbytes < core::mem::size_of::<coda_out_hdr>() { pr_warn!("coda_downcall opc %d uniq %d, not enough!\n", hdr.opcode, hdr.unique); return nbytes as isize; }
        if nbytes > size { pr_warn!("downcall opc %d, uniq %d, too much!", hdr.opcode, hdr.unique); nbytes = size; }
        let dcbuf = vmemdup_user(buf, nbytes);
        if IS_ERR(dcbuf) { return PTR_ERR(dcbuf); }
        error = coda_downcall(vcp, hdr.opcode, dcbuf, nbytes);
        kvfree(dcbuf);
        if error != 0 { pr_warn!("coda_downcall error: %d\n", error); return error as isize; }
        return nbytes as isize;
    }

    mutex_lock(&mut (*vcp).vc_mutex);
    let mut lh = (*vcp).vc_processing.next;
    while lh != &mut (*vcp).vc_processing as *mut _ {
        let tmp = list_entry(lh, upc_req, uc_chain);
        if (*tmp).uc_unique == hdr.unique { req = tmp; list_del(&mut (*req).uc_chain); break; }
        lh = (*lh).next;
    }
    mutex_unlock(&mut (*vcp).vc_mutex);
    if req.is_null() { pr_warn!("msg (%d, %d) not found\n", hdr.opcode, hdr.unique); return -ESRCH; }
    if (*req).uc_outSize < nbytes { pr_warn!("too much cnt: %d, cnt: %ld, opc: %d, uniq: %d.\n", (*req).uc_outSize, nbytes, hdr.opcode, hdr.unique); nbytes = (*req).uc_outSize; }
    if copy_from_user((*req).uc_data, buf, nbytes) != 0 { (*req).uc_flags |= CODA_REQ_ABORT; wake_up(&mut (*req).uc_sleep); return -EFAULT; }
    (*req).uc_outSize = nbytes; (*req).uc_flags |= CODA_REQ_WRITE;
    if (*req).uc_opcode == CODA_OPEN_BY_FD {
        let outp = (*req).uc_data as *mut coda_open_by_fd_out;
        if (*outp).oh.result == 0 { (*outp).fh = fget((*outp).fd); if (*outp).fh.is_null() { return -EBADF; } }
    }
    wake_up(&mut (*req).uc_sleep);
    count = nbytes as isize;
    count
}

/* Read a message from the kernel to Venus. */
unsafe fn coda_psdev_read(file: *mut file, buf: *mut ::core::ffi::c_char, nbytes: usize, _off: *mut loff_t) -> isize {
    let vcp = (*file).private_data as *mut venus_comm;
    if nbytes == 0 { return 0; }
    mutex_lock(&mut (*vcp).vc_mutex);
    while list_empty(&(*vcp).vc_pending) {
        if (*file).f_flags & O_NONBLOCK != 0 { mutex_unlock(&mut (*vcp).vc_mutex); return -EAGAIN; }
        if signal_pending(current) { mutex_unlock(&mut (*vcp).vc_mutex); return -ERESTARTSYS; }
        mutex_unlock(&mut (*vcp).vc_mutex); schedule(); mutex_lock(&mut (*vcp).vc_mutex);
    }
    let req = list_entry((*vcp).vc_pending.next, upc_req, uc_chain);
    list_del(&mut (*req).uc_chain);
    let count = core::cmp::min(nbytes, (*req).uc_inSize);
    let mut retval: isize = count as isize;
    if copy_to_user(buf, (*req).uc_data, count) != 0 { retval = -EFAULT; }
    if (*req).uc_flags & CODA_REQ_ASYNC == 0 { (*req).uc_flags |= CODA_REQ_READ; list_add_tail(&mut (*req).uc_chain, &mut (*vcp).vc_processing); }
    else { kvfree((*req).uc_data); kfree(req); }
    mutex_unlock(&mut (*vcp).vc_mutex);
    retval
}

unsafe fn coda_psdev_open(inode: *mut inode, file: *mut file) -> ::core::ffi::c_int {
    if task_active_pid_ns(current) != &init_pid_ns || current_user_ns() != &init_user_ns { return -EINVAL; }
    let idx = iminor(inode); if idx < 0 || idx >= MAX_CODADEVS { return -ENODEV; }
    let vcp = &mut coda_comms[idx as usize]; mutex_lock(&mut vcp.vc_mutex);
    if vcp.vc_inuse != 0 { mutex_unlock(&mut vcp.vc_mutex); return -EBUSY; }
    vcp.vc_inuse += 1; INIT_LIST_HEAD(&mut vcp.vc_pending); INIT_LIST_HEAD(&mut vcp.vc_processing); init_waitqueue_head(&mut vcp.vc_waitq); vcp.vc_sb = core::ptr::null_mut(); vcp.vc_seq = 0; (*file).private_data = vcp as *mut _ as *mut _;
    mutex_unlock(&mut vcp.vc_mutex); 0
}

unsafe fn coda_psdev_release(_inode: *mut inode, file: *mut file) -> ::core::ffi::c_int {
    let vcp = (*file).private_data as *mut venus_comm; if vcp.is_null() || (*vcp).vc_inuse == 0 { pr_warn!("Not open.\n"); return -1; }
    mutex_lock(&mut (*vcp).vc_mutex);
    let mut req = (*vcp).vc_pending.next; while req != &mut (*vcp).vc_pending as *mut _ { let next = (*req).next; let r = list_entry(req, upc_req, uc_chain); list_del(req); if (*r).uc_flags & CODA_REQ_ASYNC != 0 { kvfree((*r).uc_data); kfree(r); } else { (*r).uc_flags |= CODA_REQ_ABORT; wake_up(&mut (*r).uc_sleep); } req = next; }
    let mut req = (*vcp).vc_processing.next; while req != &mut (*vcp).vc_processing as *mut _ { let next = (*req).next; let r = list_entry(req, upc_req, uc_chain); list_del(req); (*r).uc_flags |= CODA_REQ_ABORT; wake_up(&mut (*r).uc_sleep); req = next; }
    (*file).private_data = core::ptr::null_mut(); (*vcp).vc_inuse -= 1; mutex_unlock(&mut (*vcp).vc_mutex); 0
}

// The file-operations table, module metadata, initialization, and teardown
// below retain the corresponding Linux kernel registrations.
static coda_psdev_fops: file_operations = file_operations { owner: THIS_MODULE, read: Some(coda_psdev_read), write: Some(coda_psdev_write), poll: Some(coda_psdev_poll), unlocked_ioctl: Some(coda_psdev_ioctl), open: Some(coda_psdev_open), release: Some(coda_psdev_release), llseek: Some(noop_llseek) };

unsafe fn init_coda_psdev() -> ::core::ffi::c_int { if register_chrdev(CODA_PSDEV_MAJOR, "coda", &coda_psdev_fops) != 0 { pr_err!("unable to get major %d\n", CODA_PSDEV_MAJOR); return -EIO; } coda_psdev_class = class_create("coda"); if IS_ERR(coda_psdev_class) { let e = PTR_ERR(coda_psdev_class); unregister_chrdev(CODA_PSDEV_MAJOR, "coda"); return e; } for i in 0..MAX_CODADEVS { mutex_init(&mut coda_comms[i].vc_mutex); device_create(coda_psdev_class, core::ptr::null_mut(), MKDEV(CODA_PSDEV_MAJOR, i), core::ptr::null_mut(), "cfs%d", i); } coda_sysctl_init(); 0 }

unsafe fn init_coda() -> ::core::ffi::c_int { let mut status = coda_init_inodecache(); if status != 0 { return status; } status = init_coda_psdev(); if status != 0 { coda_destroy_inodecache(); return status; } status = register_filesystem(&coda_fs_type); if status != 0 { for i in 0..MAX_CODADEVS { device_destroy(coda_psdev_class, MKDEV(CODA_PSDEV_MAJOR, i)); } class_destroy(coda_psdev_class); unregister_chrdev(CODA_PSDEV_MAJOR, "coda"); coda_sysctl_clean(); coda_destroy_inodecache(); } status }

unsafe fn exit_coda() { unregister_filesystem(&coda_fs_type); for i in 0..MAX_CODADEVS { device_destroy(coda_psdev_class, MKDEV(CODA_PSDEV_MAJOR, i)); } class_destroy(coda_psdev_class); unregister_chrdev(CODA_PSDEV_MAJOR, "coda"); coda_sysctl_clean(); coda_destroy_inodecache(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
