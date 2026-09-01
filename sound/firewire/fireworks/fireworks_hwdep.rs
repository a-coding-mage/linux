// SPDX-License-Identifier: GPL-2.0-only
/*
 * fireworks_hwdep.rs - a part of driver for Fireworks based devices
 *
 * Copyright (c) 2013-2014 Takashi Sakamoto
 */

/*
 * This codes have five functionalities.
 *
 * 1.get information about firewire node
 * 2.get notification about starting/stopping stream
 * 3.lock/unlock streaming
 * 4.transmit command of EFW transaction
 * 5.receive response of EFW transaction
 *
 */

// Rust translation of implementation depending on declarations from "fireworks.h"
// and Linux/ALSA kernel headers.

unsafe fn hwdep_read_resp_buf(
    efw: *mut snd_efw,
    mut buf: *mut c_char,
    mut remained: c_long,
    _offset: *mut loff_t,
) -> c_long {
    let mut length: c_uint;
    let mut till_end: c_uint;
    let mut t: *mut snd_efw_transaction;
    let mut pull_ptr: *mut u8;
    let mut count: c_long = 0;
    let mut type_: c_uint;

    if remained < (core::mem::size_of_val(&type_) + core::mem::size_of::<snd_efw_transaction>()) as c_long {
        return -ENOSPC;
    }

    /* data type is SNDRV_FIREWIRE_EVENT_EFW_RESPONSE */
    type_ = SNDRV_FIREWIRE_EVENT_EFW_RESPONSE;
    if copy_to_user(
        buf as *mut c_void,
        &type_ as *const _ as *const c_void,
        core::mem::size_of_val(&type_),
    ) != 0
    {
        return -EFAULT;
    }
    count += core::mem::size_of_val(&type_) as c_long;
    remained -= core::mem::size_of_val(&type_) as c_long;
    buf = buf.add(core::mem::size_of_val(&type_));

    /* write into buffer as many responses as possible */
    spin_lock_irq(&mut (*efw).lock);

    /*
     * When another task reaches here during this task's access to user
     * space, it picks up current position in buffer and can read the same
     * series of responses.
     */
    pull_ptr = (*efw).pull_ptr;

    while (*efw).push_ptr != pull_ptr {
        t = pull_ptr as *mut snd_efw_transaction;
        length = be32_to_cpu((*t).length) * core::mem::size_of::<__be32>() as c_uint;

        /* confirm enough space for this response */
        if remained < length as c_long {
            break;
        }

        /* copy from ring buffer to user buffer */
        while length > 0 {
            till_end = snd_efw_resp_buf_size
                - pull_ptr.offset_from((*efw).resp_buf) as c_uint;
            till_end = min_t_unsigned_int(length, till_end);

            spin_unlock_irq(&mut (*efw).lock);

            if copy_to_user(buf as *mut c_void, pull_ptr as *const c_void, till_end as usize) != 0 {
                return -EFAULT;
            }

            spin_lock_irq(&mut (*efw).lock);

            pull_ptr = pull_ptr.add(till_end as usize);
            if pull_ptr >= (*efw).resp_buf.add(snd_efw_resp_buf_size as usize) {
                pull_ptr = pull_ptr.sub(snd_efw_resp_buf_size as usize);
            }

            length -= till_end;
            buf = buf.add(till_end as usize);
            count += till_end as c_long;
            remained -= till_end as c_long;
        }
    }

    /*
     * All of tasks can read from the buffer nearly simultaneously, but the
     * last position for each task is different depending on the length of
     * given buffer. Here, for simplicity, a position of buffer is set by
     * the latest task. It's better for a listening application to allow one
     * thread to read from the buffer. Unless, each task can read different
     * sequence of responses depending on variation of buffer length.
     */
    (*efw).pull_ptr = pull_ptr;

    spin_unlock_irq(&mut (*efw).lock);

    count
}

unsafe fn hwdep_read_locked(
    efw: *mut snd_efw,
    buf: *mut c_char,
    mut count: c_long,
    _offset: *mut loff_t,
) -> c_long {
    let mut event: snd_firewire_event = core::mem::zeroed();
    event.lock_status.type_ = SNDRV_FIREWIRE_EVENT_LOCK_STATUS;

    spin_lock_irq(&mut (*efw).lock);
    event.lock_status.status = ((*efw).dev_lock_count > 0) as _;
    (*efw).dev_lock_changed = false;
    spin_unlock_irq(&mut (*efw).lock);

    count = min_t_long(count, core::mem::size_of::<snd_firewire_event_lock_status>() as c_long);

    if copy_to_user(buf as *mut c_void, &event as *const _ as *const c_void, count as usize) != 0 {
        return -EFAULT;
    }

    count
}

unsafe fn hwdep_read(
    hwdep: *mut snd_hwdep,
    buf: *mut c_char,
    mut count: c_long,
    offset: *mut loff_t,
) -> c_long {
    let efw: *mut snd_efw = (*hwdep).private_data as *mut snd_efw;
    let mut wait: wait_queue_entry = core::mem::zeroed();
    let mut dev_lock_changed: bool;
    let mut queued: bool;

    init_wait(&mut wait);

    spin_lock_irq(&mut (*efw).lock);

    dev_lock_changed = (*efw).dev_lock_changed;
    queued = (*efw).push_ptr != (*efw).pull_ptr;

    while !dev_lock_changed && !queued {
        prepare_to_wait(&mut (*efw).hwdep_wait, &mut wait, TASK_INTERRUPTIBLE);
        spin_unlock_irq(&mut (*efw).lock);
        schedule();
        finish_wait(&mut (*efw).hwdep_wait, &mut wait);
        if signal_pending(current) != 0 {
            return -ERESTARTSYS;
        }
        spin_lock_irq(&mut (*efw).lock);
        dev_lock_changed = (*efw).dev_lock_changed;
        queued = (*efw).push_ptr != (*efw).pull_ptr;
    }

    spin_unlock_irq(&mut (*efw).lock);

    if dev_lock_changed {
        count = hwdep_read_locked(efw, buf, count, offset);
    } else if queued {
        count = hwdep_read_resp_buf(efw, buf, count, offset);
    }

    count
}

unsafe fn hwdep_write(
    hwdep: *mut snd_hwdep,
    data: *const c_char,
    mut count: c_long,
    _offset: *mut loff_t,
) -> c_long {
    let efw: *mut snd_efw = (*hwdep).private_data as *mut snd_efw;
    let seqnum: u32;
    let buf: *mut u8;

    if count < core::mem::size_of::<snd_efw_transaction>() as c_long
        || SND_EFW_RESPONSE_MAXIMUM_BYTES < count
    {
        return -EINVAL;
    }

    buf = memdup_user(data as *const c_void, count as usize) as *mut u8;
    if IS_ERR(buf as *const c_void) {
        return PTR_ERR(buf as *const c_void);
    }

    /* check seqnum is not for kernel-land */
    seqnum = be32_to_cpu((*(buf as *mut snd_efw_transaction)).seqnum);
    if seqnum > SND_EFW_TRANSACTION_USER_SEQNUM_MAX {
        count = -EINVAL;
        kfree(buf as *const c_void);
        return count;
    }

    if snd_efw_transaction_cmd((*efw).unit, buf, count as c_uint) < 0 {
        count = -EIO;
    }
    kfree(buf as *const c_void);
    count
}

unsafe fn hwdep_poll(
    hwdep: *mut snd_hwdep,
    file: *mut file,
    wait: *mut poll_table,
) -> __poll_t {
    let efw: *mut snd_efw = (*hwdep).private_data as *mut snd_efw;
    let events: __poll_t;

    poll_wait(file, &mut (*efw).hwdep_wait, wait);

    spin_lock_irq(&mut (*efw).lock);
    if (*efw).dev_lock_changed || (*efw).pull_ptr != (*efw).push_ptr {
        events = EPOLLIN | EPOLLRDNORM;
    } else {
        events = 0;
    }
    spin_unlock_irq(&mut (*efw).lock);
    events | EPOLLOUT
}

unsafe fn hwdep_get_info(efw: *mut snd_efw, arg: *mut c_void) -> c_int {
    let dev: *mut fw_device = fw_parent_device((*efw).unit);
    let mut info: snd_firewire_get_info = core::mem::zeroed();

    info.type_ = SNDRV_FIREWIRE_TYPE_FIREWORKS;
    info.card = (*(*dev).card).index;
    *(info.guid.as_mut_ptr().add(0) as *mut __be32) = cpu_to_be32((*dev).config_rom[3]);
    *(info.guid.as_mut_ptr().add(4) as *mut __be32) = cpu_to_be32((*dev).config_rom[4]);
    strscpy(
        info.device_name.as_mut_ptr(),
        dev_name(&mut (*dev).device),
        core::mem::size_of_val(&info.device_name),
    );

    if copy_to_user(
        arg,
        &info as *const _ as *const c_void,
        core::mem::size_of::<snd_firewire_get_info>(),
    ) != 0
    {
        return -EFAULT;
    }

    0
}

unsafe fn hwdep_lock(efw: *mut snd_efw) -> c_int {
    spin_lock_irq(&mut (*efw).lock);

    let ret = if (*efw).dev_lock_count == 0 {
        (*efw).dev_lock_count = -1;
        0
    } else {
        -EBUSY
    };

    spin_unlock_irq(&mut (*efw).lock);
    ret
}

unsafe fn hwdep_unlock(efw: *mut snd_efw) -> c_int {
    spin_lock_irq(&mut (*efw).lock);

    let ret = if (*efw).dev_lock_count == -1 {
        (*efw).dev_lock_count = 0;
        0
    } else {
        -EBADFD
    };

    spin_unlock_irq(&mut (*efw).lock);
    ret
}

unsafe fn hwdep_release(hwdep: *mut snd_hwdep, _file: *mut file) -> c_int {
    let efw: *mut snd_efw = (*hwdep).private_data as *mut snd_efw;

    spin_lock_irq(&mut (*efw).lock);
    if (*efw).dev_lock_count == -1 {
        (*efw).dev_lock_count = 0;
    }
    spin_unlock_irq(&mut (*efw).lock);

    0
}

unsafe fn hwdep_ioctl(
    hwdep: *mut snd_hwdep,
    file: *mut file,
    cmd: c_uint,
    arg: c_ulong,
) -> c_int {
    let efw: *mut snd_efw = (*hwdep).private_data as *mut snd_efw;

    match cmd {
        SNDRV_FIREWIRE_IOCTL_GET_INFO => hwdep_get_info(efw, arg as *mut c_void),
        SNDRV_FIREWIRE_IOCTL_LOCK => hwdep_lock(efw),
        SNDRV_FIREWIRE_IOCTL_UNLOCK => hwdep_unlock(efw),
        _ => {
            let _ = file;
            -ENOIOCTLCMD
        }
    }
}

// C conditional preserved: when CONFIG_COMPAT is enabled, this function is used
// as ioctl_compat; otherwise the operation pointer is NULL.
#[cfg(CONFIG_COMPAT)]
unsafe fn hwdep_compat_ioctl(
    hwdep: *mut snd_hwdep,
    file: *mut file,
    cmd: c_uint,
    arg: c_ulong,
) -> c_int {
    hwdep_ioctl(hwdep, file, cmd, compat_ptr(arg) as c_ulong)
}

pub unsafe fn snd_efw_create_hwdep_device(efw: *mut snd_efw) -> c_int {
    static OPS: snd_hwdep_ops = snd_hwdep_ops {
        read: Some(hwdep_read),
        write: Some(hwdep_write),
        release: Some(hwdep_release),
        poll: Some(hwdep_poll),
        ioctl: Some(hwdep_ioctl),
        #[cfg(CONFIG_COMPAT)]
        ioctl_compat: Some(hwdep_compat_ioctl),
        #[cfg(not(CONFIG_COMPAT))]
        ioctl_compat: None,
    };
    let mut hwdep: *mut snd_hwdep = core::ptr::null_mut();
    let err: c_int;

    err = snd_hwdep_new((*efw).card, c_str!("Fireworks"), 0, &mut hwdep);
    if err < 0 {
        return err;
    }
    strscpy(
        (*hwdep).name.as_mut_ptr(),
        c_str!("Fireworks"),
        core::mem::size_of_val(&(*hwdep).name),
    );
    (*hwdep).iface = SNDRV_HWDEP_IFACE_FW_FIREWORKS;
    (*hwdep).ops = OPS;
    (*hwdep).private_data = efw as *mut c_void;
    (*hwdep).exclusive = true;

    err
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
