// SPDX-License-Identifier: GPL-2.0-only
/*
 * bebob_hwdep.c - a part of driver for BeBoB based devices
 *
 * Copyright (c) 2013-2014 Takashi Sakamoto
 */

/*
 * This codes give three functionality.
 *
 * 1.get firewire node infomation
 * 2.get notification about starting/stopping stream
 * 3.lock/unlock stream
 */

// Rust translation of the implementation originally depending on "bebob.h".

unsafe extern "C" fn hwdep_read(
    hwdep: *mut snd_hwdep,
    buf: *mut c_char,
    mut count: c_long,
    offset: *mut loff_t,
) -> c_long {
    let bebob: *mut snd_bebob = unsafe { (*hwdep).private_data as *mut snd_bebob };
    let mut wait = DEFINE_WAIT();
    let mut event: snd_firewire_event = unsafe { core::mem::zeroed() };

    unsafe { spin_lock_irq(&mut (*bebob).lock) };

    while unsafe { !(*bebob).dev_lock_changed } {
        unsafe { prepare_to_wait(&mut (*bebob).hwdep_wait, &mut wait, TASK_INTERRUPTIBLE) };
        unsafe { spin_unlock_irq(&mut (*bebob).lock) };
        unsafe { schedule() };
        unsafe { finish_wait(&mut (*bebob).hwdep_wait, &mut wait) };
        if unsafe { signal_pending(current) } != 0 {
            return -ERESTARTSYS;
        }
        unsafe { spin_lock_irq(&mut (*bebob).lock) };
    }

    unsafe { memset(&mut event as *mut _ as *mut c_void, 0, core::mem::size_of_val(&event)) };
    count = min_t_long(count, core::mem::size_of_val(unsafe { &event.lock_status }) as c_long);
    unsafe {
        event.lock_status.type_ = SNDRV_FIREWIRE_EVENT_LOCK_STATUS;
        event.lock_status.status = ((*bebob).dev_lock_count > 0) as _;
        (*bebob).dev_lock_changed = false;
    }

    unsafe { spin_unlock_irq(&mut (*bebob).lock) };

    if unsafe { copy_to_user(buf as *mut c_void, &event as *const _ as *const c_void, count as usize) } != 0 {
        return -EFAULT;
    }

    count
}

unsafe extern "C" fn hwdep_poll(
    hwdep: *mut snd_hwdep,
    file: *mut file,
    wait: *mut poll_table,
) -> __poll_t {
    let bebob: *mut snd_bebob = unsafe { (*hwdep).private_data as *mut snd_bebob };

    unsafe { poll_wait(file, &mut (*bebob).hwdep_wait, wait) };

    unsafe { spin_lock_irq(&mut (*bebob).lock) };
    let ret = if unsafe { (*bebob).dev_lock_changed } {
        EPOLLIN | EPOLLRDNORM
    } else {
        0
    };
    unsafe { spin_unlock_irq(&mut (*bebob).lock) };

    ret
}

unsafe extern "C" fn hwdep_get_info(bebob: *mut snd_bebob, arg: *mut c_void) -> c_int {
    let dev: *mut fw_device = unsafe { fw_parent_device((*bebob).unit) };
    let mut info: snd_firewire_get_info = unsafe { core::mem::zeroed() };

    unsafe { memset(&mut info as *mut _ as *mut c_void, 0, core::mem::size_of_val(&info)) };
    info.type_ = SNDRV_FIREWIRE_TYPE_BEBOB;
    unsafe {
        info.card = (*(*dev).card).index;
        *(info.guid.as_mut_ptr().add(0) as *mut __be32) = cpu_to_be32((*dev).config_rom[3]);
        *(info.guid.as_mut_ptr().add(4) as *mut __be32) = cpu_to_be32((*dev).config_rom[4]);
        strscpy(
            info.device_name.as_mut_ptr(),
            dev_name(&mut (*dev).device),
            core::mem::size_of_val(&info.device_name),
        );
    }

    if unsafe {
        copy_to_user(
            arg,
            &info as *const _ as *const c_void,
            core::mem::size_of_val(&info),
        )
    } != 0
    {
        return -EFAULT;
    }

    0
}

unsafe extern "C" fn hwdep_lock(bebob: *mut snd_bebob) -> c_int {
    unsafe { spin_lock_irq(&mut (*bebob).lock) };

    let ret = if unsafe { (*bebob).dev_lock_count == 0 } {
        unsafe { (*bebob).dev_lock_count = -1 };
        0
    } else {
        -EBUSY
    };

    unsafe { spin_unlock_irq(&mut (*bebob).lock) };
    ret
}

unsafe extern "C" fn hwdep_unlock(bebob: *mut snd_bebob) -> c_int {
    unsafe { spin_lock_irq(&mut (*bebob).lock) };

    let ret = if unsafe { (*bebob).dev_lock_count == -1 } {
        unsafe { (*bebob).dev_lock_count = 0 };
        0
    } else {
        -EBADFD
    };

    unsafe { spin_unlock_irq(&mut (*bebob).lock) };
    ret
}

unsafe extern "C" fn hwdep_release(hwdep: *mut snd_hwdep, file: *mut file) -> c_int {
    let bebob: *mut snd_bebob = unsafe { (*hwdep).private_data as *mut snd_bebob };

    unsafe { spin_lock_irq(&mut (*bebob).lock) };
    if unsafe { (*bebob).dev_lock_count == -1 } {
        unsafe { (*bebob).dev_lock_count = 0 };
    }
    unsafe { spin_unlock_irq(&mut (*bebob).lock) };

    0
}

unsafe extern "C" fn hwdep_ioctl(
    hwdep: *mut snd_hwdep,
    file: *mut file,
    cmd: c_uint,
    arg: c_ulong,
) -> c_int {
    let bebob: *mut snd_bebob = unsafe { (*hwdep).private_data as *mut snd_bebob };

    match cmd {
        SNDRV_FIREWIRE_IOCTL_GET_INFO => unsafe { hwdep_get_info(bebob, arg as *mut c_void) },
        SNDRV_FIREWIRE_IOCTL_LOCK => unsafe { hwdep_lock(bebob) },
        SNDRV_FIREWIRE_IOCTL_UNLOCK => unsafe { hwdep_unlock(bebob) },
        _ => -ENOIOCTLCMD,
    }
}

// CONFIG_COMPAT: when enabled, provide the compat ioctl wrapper; otherwise this
// operation is NULL in the snd_hwdep_ops table.
#[cfg(CONFIG_COMPAT)]
unsafe extern "C" fn hwdep_compat_ioctl(
    hwdep: *mut snd_hwdep,
    file: *mut file,
    cmd: c_uint,
    arg: c_ulong,
) -> c_int {
    unsafe { hwdep_ioctl(hwdep, file, cmd, compat_ptr(arg) as c_ulong) }
}

#[no_mangle]
pub unsafe extern "C" fn snd_bebob_create_hwdep_device(bebob: *mut snd_bebob) -> c_int {
    let ops = snd_hwdep_ops {
        read: Some(hwdep_read),
        release: Some(hwdep_release),
        poll: Some(hwdep_poll),
        ioctl: Some(hwdep_ioctl),
        ioctl_compat: {
            #[cfg(CONFIG_COMPAT)]
            {
                Some(hwdep_compat_ioctl)
            }
            #[cfg(not(CONFIG_COMPAT))]
            {
                None
            }
        },
    };
    let mut hwdep: *mut snd_hwdep = core::ptr::null_mut();
    let mut err: c_int;

    err = unsafe { snd_hwdep_new((*bebob).card, c"BeBoB".as_ptr(), 0, &mut hwdep) };
    if err < 0 {
        return err;
    }
    unsafe {
        strscpy((*hwdep).name.as_mut_ptr(), c"BeBoB".as_ptr(), core::mem::size_of_val(&(*hwdep).name));
        (*hwdep).iface = SNDRV_HWDEP_IFACE_FW_BEBOB;
        (*hwdep).ops = ops;
        (*hwdep).private_data = bebob as *mut c_void;
        (*hwdep).exclusive = true;
    }

    err
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
