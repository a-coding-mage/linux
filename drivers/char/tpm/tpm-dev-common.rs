// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2004 IBM Corporation
 * Authors:
 * Leendert van Doorn <leendert@watson.ibm.com>
 * Dave Safford <safford@watson.ibm.com>
 * Reiner Sailer <sailer@watson.ibm.com>
 * Kylene Hall <kjhall@us.ibm.com>
 *
 * Copyright (C) 2013 Obsidian Research Corp
 * Jason Gunthorpe <jgunthorpe@obsidianresearch.com>
 *
 * Device file system interface to the TPM
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

static mut TPM_DEV_WQ: *mut workqueue_struct = core::ptr::null_mut();

unsafe fn tpm_dev_transmit(
    chip: *mut tpm_chip,
    space: *mut tpm_space,
    buf: *mut u8,
    bufsiz: usize,
) -> isize {
    let header = buf as *mut tpm_header;
    let mut ret: isize;
    let mut len: isize;

    if (*chip).flags & TPM_CHIP_FLAG_TPM2 != 0 {
        tpm2_end_auth_session(chip);
    }

    ret = tpm2_prepare_space(chip, space, buf, bufsiz);
    /* If the command is not implemented by the TPM, synthesize a
     * response with a TPM2_RC_COMMAND_CODE return for user-space.
     */
    if ret == -EOPNOTSUPP as isize {
        (*header).length = cpu_to_be32(core::mem::size_of::<tpm_header>() as u32);
        (*header).tag = cpu_to_be16(TPM2_ST_NO_SESSIONS);
        (*header).return_code = cpu_to_be32(TPM2_RC_COMMAND_CODE | TSS2_RESMGR_TPM_RC_LAYER);
        ret = core::mem::size_of::<tpm_header>() as isize;
    }
    if ret != 0 {
        return ret;
    }

    len = tpm_transmit(chip, buf, bufsiz);
    if len < 0 {
        ret = len;
    }

    if ret == 0 {
        ret = tpm2_commit_space(chip, space, buf, &mut len);
    } else {
        tpm2_flush_space(chip);
    }

    if ret != 0 { ret } else { len }
}

unsafe fn tpm_dev_async_work(work: *mut work_struct) {
    let priv_: *mut file_priv = container_of(work, file_priv, async_work);
    let mut ret: isize;

    mutex_lock(&mut (*priv_).buffer_mutex);
    (*priv_).command_enqueued = false;
    ret = tpm_try_get_ops((*priv_).chip);
    if ret != 0 {
        (*priv_).response_length = ret;
        mutex_unlock(&mut (*priv_).buffer_mutex);
        return;
    }

    ret = tpm_dev_transmit((*priv_).chip, (*priv_).space, (*priv_).data_buffer.as_mut_ptr(),
                           (*priv_).data_buffer.len());
    tpm_put_ops((*priv_).chip);

    /*
     * If ret is > 0 then tpm_dev_transmit returned the size of the
     * response. If ret is < 0 then tpm_dev_transmit failed and
     * returned an error code.
     */
    if ret != 0 {
        (*priv_).response_length = ret;
        mod_timer(&mut (*priv_).user_read_timer, jiffies + (120 * HZ));
    }
    mutex_unlock(&mut (*priv_).buffer_mutex);
    wake_up_interruptible(&mut (*priv_).async_wait);
}

unsafe fn user_reader_timeout(t: *mut timer_list) {
    let priv_: *mut file_priv = timer_container_of(t, file_priv, user_read_timer);

    pr_warn!("TPM user space timeout is deprecated (pid={})\n", task_tgid_nr(current));
    schedule_work(&mut (*priv_).timeout_work);
}

unsafe fn tpm_timeout_work(work: *mut work_struct) {
    let priv_: *mut file_priv = container_of(work, file_priv, timeout_work);

    mutex_lock(&mut (*priv_).buffer_mutex);
    (*priv_).response_read = true;
    (*priv_).response_length = 0;
    memset((*priv_).data_buffer.as_mut_ptr(), 0, (*priv_).data_buffer.len());
    mutex_unlock(&mut (*priv_).buffer_mutex);
    wake_up_interruptible(&mut (*priv_).async_wait);
}

pub unsafe fn tpm_common_open(file: *mut file, chip: *mut tpm_chip,
                              priv_: *mut file_priv, space: *mut tpm_space) {
    (*priv_).chip = chip;
    (*priv_).space = space;
    (*priv_).response_read = true;

    mutex_init(&mut (*priv_).buffer_mutex);
    timer_setup(&mut (*priv_).user_read_timer, user_reader_timeout, 0);
    INIT_WORK(&mut (*priv_).timeout_work, tpm_timeout_work);
    INIT_WORK(&mut (*priv_).async_work, tpm_dev_async_work);
    init_waitqueue_head(&mut (*priv_).async_wait);
    (*file).private_data = priv_;
}

pub unsafe fn tpm_common_read(file: *mut file, buf: *mut core::ffi::c_void,
                              size: usize, off: *mut loff_t) -> isize {
    let priv_: *mut file_priv = (*file).private_data;
    let mut ret_size: isize = 0;
    let rc: isize;

    mutex_lock(&mut (*priv_).buffer_mutex);
    if (*priv_).response_length != 0 {
        (*priv_).response_read = true;
        ret_size = core::cmp::min(size as isize, (*priv_).response_length);
        if ret_size <= 0 {
            (*priv_).response_length = 0;
            mutex_unlock(&mut (*priv_).buffer_mutex);
            return ret_size;
        }
        rc = copy_to_user(buf, (*priv_).data_buffer.as_ptr().add(*off as usize), ret_size as usize);
        if rc != 0 {
            memset((*priv_).data_buffer.as_mut_ptr(), 0, TPM_BUFSIZE);
            (*priv_).response_length = 0;
            ret_size = -EFAULT as isize;
        } else {
            memset((*priv_).data_buffer.as_mut_ptr().add(*off as usize), 0, ret_size as usize);
            (*priv_).response_length -= ret_size;
            *off += ret_size;
        }
    }
    if (*priv_).response_length == 0 {
        *off = 0;
        timer_delete_sync(&mut (*priv_).user_read_timer);
        flush_work(&mut (*priv_).timeout_work);
    }
    mutex_unlock(&mut (*priv_).buffer_mutex);
    ret_size
}

pub unsafe fn tpm_common_write(file: *mut file, buf: *const core::ffi::c_void,
                               size: usize, off: *mut loff_t) -> isize {
    let priv_: *mut file_priv = (*file).private_data;
    let mut ret: isize = 0;
    if size > TPM_BUFSIZE { return -E2BIG as isize; }
    mutex_lock(&mut (*priv_).buffer_mutex);
    /* Cannot perform a write until the read has cleared either via
     * tpm_read or a user_read_timer timeout. This also prevents split
     * buffered writes from blocking here.
     */
    if ((!(*priv_).response_read && (*priv_).response_length != 0) || (*priv_).command_enqueued) {
        ret = -EBUSY as isize;
    } else if copy_from_user((*priv_).data_buffer.as_mut_ptr(), buf, size) != 0 {
        ret = -EFAULT as isize;
    } else if size < 6 || size < be32_to_cpu(*( (*priv_).data_buffer.as_ptr().add(2) as *const __be32)) as usize {
        ret = -EINVAL as isize;
    } else {
        (*priv_).response_length = 0;
        (*priv_).response_read = false;
        *off = 0;
        /* If in nonblocking mode schedule an async job to send the
         * command return the size. In case of error the err code will be
         * returned in the subsequent read call.
         */
        if (*file).f_flags & O_NONBLOCK != 0 {
            (*priv_).command_enqueued = true;
            queue_work(TPM_DEV_WQ, &mut (*priv_).async_work);
            mutex_unlock(&mut (*priv_).buffer_mutex);
            return size as isize;
        }
        /* atomic tpm command send and result receive. */
        if tpm_try_get_ops((*priv_).chip) != 0 {
            ret = -EPIPE as isize;
        } else {
            ret = tpm_dev_transmit((*priv_).chip, (*priv_).space, (*priv_).data_buffer.as_mut_ptr(), (*priv_).data_buffer.len());
            tpm_put_ops((*priv_).chip);
            if ret > 0 {
                (*priv_).response_length = ret;
                mod_timer(&mut (*priv_).user_read_timer, jiffies + (120 * HZ));
                ret = size as isize;
            }
        }
    }
    mutex_unlock(&mut (*priv_).buffer_mutex);
    ret
}

pub unsafe fn tpm_common_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    let priv_: *mut file_priv = (*file).private_data;
    poll_wait(file, &mut (*priv_).async_wait, wait);
    mutex_lock(&mut (*priv_).buffer_mutex);
    let mask = if (*priv_).response_length != 0 { EPOLLIN | EPOLLRDNORM } else { EPOLLOUT | EPOLLWRNORM };
    mutex_unlock(&mut (*priv_).buffer_mutex);
    mask
}

/* Called on file close */
pub unsafe fn tpm_common_release(file: *mut file, priv_: *mut file_priv) {
    flush_work(&mut (*priv_).async_work);
    timer_delete_sync(&mut (*priv_).user_read_timer);
    flush_work(&mut (*priv_).timeout_work);
    (*file).private_data = core::ptr::null_mut();
    (*priv_).response_length = 0;
}

pub unsafe fn tpm_dev_common_init() -> i32 {
    TPM_DEV_WQ = alloc_workqueue("tpm_dev_wq", WQ_MEM_RECLAIM | WQ_PERCPU, 0);
    if TPM_DEV_WQ.is_null() { -ENOMEM } else { 0 }
}

pub unsafe fn tpm_dev_common_exit() {
    if !TPM_DEV_WQ.is_null() {
        destroy_workqueue(TPM_DEV_WQ);
        TPM_DEV_WQ = core::ptr::null_mut();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
