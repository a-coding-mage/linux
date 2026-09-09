// SPDX-License-Identifier: GPL-2.0-only
/*
 * eCryptfs: Linux filesystem encryption layer
 *
 * Copyright (C) 2008 International Business Machines Corp.
 *   Author(s): Michael A. Halcrow <mhalcrow@us.ibm.com>
 */

// Kernel dependencies and ecryptfs_kernel.h are supplied by other translation units.

static mut ECRYPTFS_NUM_MISCDEV_OPENS: atomic_t = atomic_t { counter: 0 };

static unsafe fn ecryptfs_miscdev_poll(file: *mut file, pt: *mut poll_table) -> __poll_t {
    let daemon = (*file).private_data as *mut ecryptfs_daemon;
    let mut mask: __poll_t = 0;

    mutex_lock(&mut (*daemon).mux);
    if (*daemon).flags & ECRYPTFS_DAEMON_ZOMBIE != 0 {
        printk!(KERN_WARNING, "%s: Attempt to poll on zombified daemon\n", "ecryptfs_miscdev_poll");
        goto_out_unlock_daemon!();
    }
    if (*daemon).flags & ECRYPTFS_DAEMON_IN_READ != 0 { goto_out_unlock_daemon!(); }
    if (*daemon).flags & ECRYPTFS_DAEMON_IN_POLL != 0 { goto_out_unlock_daemon!(); }
    (*daemon).flags |= ECRYPTFS_DAEMON_IN_POLL;
    mutex_unlock(&mut (*daemon).mux);
    poll_wait(file, &mut (*daemon).wait, pt);
    mutex_lock(&mut (*daemon).mux);
    if !list_empty(&(*daemon).msg_ctx_out_queue) { mask |= EPOLLIN | EPOLLRDNORM; }
    (*daemon).flags &= !ECRYPTFS_DAEMON_IN_POLL;
    mutex_unlock(&mut (*daemon).mux);
    return mask;
}

static unsafe fn ecryptfs_miscdev_open(_inode: *mut inode, file: *mut file) -> c_int {
    let mut daemon: *mut ecryptfs_daemon = core::ptr::null_mut();
    let mut rc: c_int;
    mutex_lock(&mut ecryptfs_daemon_hash_mux);
    rc = ecryptfs_find_daemon_by_euid(&mut daemon);
    if rc != 0 {
        rc = -EINVAL;
    } else {
        rc = ecryptfs_spawn_daemon(&mut daemon, file);
        if rc != 0 {
            printk!(KERN_ERR, "%s: Error attempting to spawn daemon; rc = [%d]\n", "ecryptfs_miscdev_open", rc);
        } else {
            mutex_lock(&mut (*daemon).mux);
            if (*daemon).flags & ECRYPTFS_DAEMON_MISCDEV_OPEN != 0 {
                rc = -EBUSY;
            } else {
                (*daemon).flags |= ECRYPTFS_DAEMON_MISCDEV_OPEN;
                (*file).private_data = daemon as *mut c_void;
                atomic_inc(&mut ECRYPTFS_NUM_MISCDEV_OPENS);
            }
            mutex_unlock(&mut (*daemon).mux);
        }
    }
    mutex_unlock(&mut ecryptfs_daemon_hash_mux);
    rc
}

static unsafe fn ecryptfs_miscdev_release(_inode: *mut inode, file: *mut file) -> c_int {
    let daemon = (*file).private_data as *mut ecryptfs_daemon;
    mutex_lock(&mut (*daemon).mux);
    BUG_ON!((*daemon).flags & ECRYPTFS_DAEMON_MISCDEV_OPEN == 0);
    (*daemon).flags &= !ECRYPTFS_DAEMON_MISCDEV_OPEN;
    atomic_dec(&mut ECRYPTFS_NUM_MISCDEV_OPENS);
    mutex_unlock(&mut (*daemon).mux);
    mutex_lock(&mut ecryptfs_daemon_hash_mux);
    let rc = ecryptfs_exorcise_daemon(daemon);
    mutex_unlock(&mut ecryptfs_daemon_hash_mux);
    if rc != 0 { printk!(KERN_CRIT, "%s: Fatal error whilst attempting to shut down daemon; rc = [%d]. Please report this bug.\n", "ecryptfs_miscdev_release", rc); BUG!(); }
    rc
}

pub unsafe fn ecryptfs_send_miscdev(data: *mut c_char, data_size: size_t, msg_ctx: *mut ecryptfs_msg_ctx, msg_type: u8, _msg_flags: u16, daemon: *mut ecryptfs_daemon) -> c_int {
    let msg_size = struct_size!(ecryptfs_message, data, data_size);
    let msg = kmalloc(msg_size, GFP_KERNEL) as *mut ecryptfs_message;
    if msg.is_null() { return -ENOMEM; }
    mutex_lock(&mut (*msg_ctx).mux);
    (*msg_ctx).msg = msg;
    (*msg).index = (*msg_ctx).index;
    (*msg).data_len = data_size;
    (*msg_ctx).type_ = msg_type;
    memcpy((*msg).data.as_mut_ptr() as *mut c_void, data as *const c_void, data_size);
    (*msg_ctx).msg_size = msg_size;
    list_add_tail(&mut (*msg_ctx).daemon_out_list, &mut (*daemon).msg_ctx_out_queue);
    mutex_unlock(&mut (*msg_ctx).mux);
    mutex_lock(&mut (*daemon).mux);
    (*daemon).num_queued_msg_ctx += 1;
    wake_up_interruptible(&mut (*daemon).wait);
    mutex_unlock(&mut (*daemon).mux);
    0
}

const PKT_TYPE_SIZE: usize = 1;
const PKT_CTR_SIZE: usize = 4;
const MIN_NON_MSG_PKT_SIZE: usize = PKT_TYPE_SIZE + PKT_CTR_SIZE;
const MIN_MSG_PKT_SIZE: usize = PKT_TYPE_SIZE + PKT_CTR_SIZE + ECRYPTFS_MIN_PKT_LEN_SIZE;
const MAX_MSG_PKT_SIZE: usize = PKT_TYPE_SIZE + PKT_CTR_SIZE + ECRYPTFS_MAX_PKT_LEN_SIZE + core::mem::size_of::<ecryptfs_message>() + 4 + ECRYPTFS_MAX_ENCRYPTED_KEY_BYTES;
const PKT_TYPE_OFFSET: usize = 0;
const PKT_CTR_OFFSET: usize = PKT_TYPE_SIZE;
const PKT_LEN_OFFSET: usize = PKT_TYPE_SIZE + PKT_CTR_SIZE;

unsafe fn ecryptfs_miscdev_response(daemon: *mut ecryptfs_daemon, data: *mut c_char, data_size: size_t, seq: u32) -> c_int {
    let msg = data as *mut ecryptfs_message;
    if core::mem::size_of::<ecryptfs_message>() + (*msg).data_len != data_size {
        printk!(KERN_WARNING, "%s: Invalid packet.\n", "ecryptfs_miscdev_response");
        return -EINVAL;
    }
    ecryptfs_process_response(daemon, msg, seq)
}

unsafe fn ecryptfs_miscdev_read(file: *mut file, buf: *mut c_char, count: size_t, _ppos: *mut loff_t) -> ssize_t {
    let daemon = (*file).private_data as *mut ecryptfs_daemon;
    let mut packet_length = [0i8; ECRYPTFS_MAX_PKT_LEN_SIZE];
    mutex_lock(&mut (*daemon).mux);
    if (*daemon).flags & ECRYPTFS_DAEMON_ZOMBIE != 0 || (*daemon).flags & ECRYPTFS_DAEMON_IN_READ != 0 { mutex_unlock(&mut (*daemon).mux); return 0; }
    (*daemon).flags |= ECRYPTFS_DAEMON_IN_READ;
    if list_empty(&(*daemon).msg_ctx_out_queue) {
        mutex_unlock(&mut (*daemon).mux);
        let rc = wait_event_interruptible!((*daemon).wait, !list_empty(&(*daemon).msg_ctx_out_queue));
        mutex_lock(&mut (*daemon).mux);
        if rc < 0 { (*daemon).flags &= !ECRYPTFS_DAEMON_IN_READ; mutex_unlock(&mut (*daemon).mux); return 0; }
    }
    let msg_ctx = list_first_entry!(&mut (*daemon).msg_ctx_out_queue, ecryptfs_msg_ctx, daemon_out_list);
    mutex_lock(&mut (*msg_ctx).mux);
    let packet_length_size = if !(*msg_ctx).msg.is_null() { ecryptfs_write_packet_length(packet_length.as_mut_ptr(), (*msg_ctx).msg_size, &mut 0usize) } else { 0 };
    let total_length = PKT_TYPE_SIZE + PKT_CTR_SIZE + (*msg_ctx).msg_size + packet_length_size as usize;
    if count < total_length { mutex_unlock(&mut (*msg_ctx).mux); (*daemon).flags &= !ECRYPTFS_DAEMON_IN_READ; mutex_unlock(&mut (*daemon).mux); return 0; }
    if put_user((*msg_ctx).type_, buf) != 0 || put_user(cpu_to_be32((*msg_ctx).counter), buf.add(PKT_CTR_OFFSET) as *mut __be32) != 0 { return -EFAULT; }
    let mut i = PKT_TYPE_SIZE + PKT_CTR_SIZE;
    if !(*msg_ctx).msg.is_null() { copy_to_user(buf.add(i), packet_length.as_ptr() as *const c_void, packet_length_size as usize); i += packet_length_size as usize; copy_to_user(buf.add(i), (*msg_ctx).msg as *const c_void, (*msg_ctx).msg_size); i += (*msg_ctx).msg_size; }
    list_del(&mut (*msg_ctx).daemon_out_list); kfree((*msg_ctx).msg as *mut c_void); (*msg_ctx).msg = core::ptr::null_mut();
    mutex_unlock(&mut (*msg_ctx).mux); (*daemon).flags &= !ECRYPTFS_DAEMON_IN_READ; mutex_unlock(&mut (*daemon).mux); i as ssize_t
}

unsafe fn ecryptfs_miscdev_write(file: *mut file, buf: *const c_char, count: size_t, _ppos: *mut loff_t) -> ssize_t {
    if count == 0 { return 0; }
    if count < MIN_MSG_PKT_SIZE || count > MAX_MSG_PKT_SIZE { return -EINVAL; }
    let data = memdup_user(buf as *const c_void, count);
    if IS_ERR(data) { return PTR_ERR(data) as ssize_t; }
    let ty = *(data as *const u8);
    let rc = match ty { ECRYPTFS_MSG_RESPONSE => { let mut ctr = 0u32; memcpy(&mut ctr as *mut _ as *mut c_void, data.add(PKT_CTR_OFFSET) as *const c_void, PKT_CTR_SIZE); ecryptfs_miscdev_response((*file).private_data as *mut ecryptfs_daemon, data.add(PKT_LEN_OFFSET) as *mut c_char, count - PKT_LEN_OFFSET, be32_to_cpu(ctr)) }, ECRYPTFS_MSG_HELO | ECRYPTFS_MSG_QUIT => 0, _ => -EINVAL };
    kfree(data); if rc == 0 { count as ssize_t } else { rc as ssize_t }
}

static ecryptfs_miscdev_fops: file_operations = file_operations { owner: THIS_MODULE, open: Some(ecryptfs_miscdev_open), poll: Some(ecryptfs_miscdev_poll), read: Some(ecryptfs_miscdev_read), write: Some(ecryptfs_miscdev_write), release: Some(ecryptfs_miscdev_release), llseek: Some(noop_llseek) };
static mut ecryptfs_miscdev: miscdevice = miscdevice { minor: MISC_DYNAMIC_MINOR, name: b"ecryptfs\0".as_ptr() as *const c_char, fops: &ecryptfs_miscdev_fops };

pub unsafe fn ecryptfs_init_ecryptfs_miscdev() -> c_int {
    atomic_set(&mut ECRYPTFS_NUM_MISCDEV_OPENS, 0);
    let rc = misc_register(&mut ecryptfs_miscdev);
    if rc != 0 { printk!(KERN_ERR, "%s: Failed to register miscellaneous device for communications with userspace daemons; rc = [%d]\n", "ecryptfs_init_ecryptfs_miscdev", rc); }
    rc
}

pub unsafe fn ecryptfs_destroy_ecryptfs_miscdev() {
    BUG_ON!(atomic_read(&ECRYPTFS_NUM_MISCDEV_OPENS) != 0);
    misc_deregister(&mut ecryptfs_miscdev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
