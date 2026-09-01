// SPDX-License-Identifier: GPL-2.0-only
/*
 * ff-hwdep.c - a part of driver for RME Fireface series
 *
 * Copyright (c) 2015-2017 Takashi Sakamoto
 */

/*
 * This codes give three functionality.
 *
 * 1.get firewire node information
 * 2.get notification about starting/stopping stream
 * 3.lock/unlock stream
 */

// C dependency: "ff.h".

use core::mem::size_of;
use core::ptr;

unsafe fn has_msg(ff: *mut snd_ff) -> bool {
    if (*(*(*ff).spec).protocol).has_msg.is_some() {
        ((*(*(*ff).spec).protocol).has_msg.unwrap())(ff)
    } else {
        false
    }
}

unsafe extern "C" fn hwdep_read(
    hwdep: *mut snd_hwdep,
    buf: *mut c_char,
    mut count: c_long,
    _offset: *mut loff_t,
) -> c_long {
    let ff = (*hwdep).private_data as *mut snd_ff;
    let mut wait: wait_queue_entry_t = core::mem::zeroed();

    init_wait(&mut wait);

    spin_lock_irq(&mut (*ff).lock);

    while !(*ff).dev_lock_changed && !has_msg(ff) {
        prepare_to_wait(&mut (*ff).hwdep_wait, &mut wait, TASK_INTERRUPTIBLE);
        spin_unlock_irq(&mut (*ff).lock);
        schedule();
        finish_wait(&mut (*ff).hwdep_wait, &mut wait);
        if signal_pending(current) {
            return -ERESTARTSYS;
        }
        spin_lock_irq(&mut (*ff).lock);
    }

    if (*ff).dev_lock_changed
        && count >= size_of::<snd_firewire_event_lock_status>() as c_long
    {
        let ev = snd_firewire_event_lock_status {
            type_: SNDRV_FIREWIRE_EVENT_LOCK_STATUS,
            status: ((*ff).dev_lock_count > 0) as _,
        };

        (*ff).dev_lock_changed = false;

        spin_unlock_irq(&mut (*ff).lock);

        if copy_to_user(
            buf as *mut c_void,
            &ev as *const _ as *const c_void,
            size_of::<snd_firewire_event_lock_status>(),
        ) != 0
        {
            return -EFAULT;
        }
        count = size_of::<snd_firewire_event_lock_status>() as c_long;
    } else if has_msg(ff) {
        // NOTE: Acquired spin lock should be released before accessing to user space in the
        // callback since the access can cause page fault.
        count = ((*(*(*ff).spec).protocol).copy_msg_to_user.unwrap())(ff, buf, count);
        spin_unlock_irq(&mut (*ff).lock);
    } else {
        spin_unlock_irq(&mut (*ff).lock);

        count = 0;
    }

    count
}

unsafe extern "C" fn hwdep_poll(
    hwdep: *mut snd_hwdep,
    file: *mut file,
    wait: *mut poll_table,
) -> __poll_t {
    let ff = (*hwdep).private_data as *mut snd_ff;

    poll_wait(file, &mut (*ff).hwdep_wait, wait);

    spin_lock_irq(&mut (*ff).lock);
    let ret = if (*ff).dev_lock_changed || has_msg(ff) {
        EPOLLIN | EPOLLRDNORM
    } else {
        0
    };
    spin_unlock_irq(&mut (*ff).lock);

    ret
}

unsafe fn hwdep_get_info(ff: *mut snd_ff, arg: *mut c_void) -> c_int {
    let dev = fw_parent_device((*ff).unit);
    let mut info: snd_firewire_get_info = core::mem::zeroed();

    ptr::write_bytes(
        &mut info as *mut _ as *mut u8,
        0,
        size_of::<snd_firewire_get_info>(),
    );
    info.type_ = SNDRV_FIREWIRE_TYPE_FIREFACE;
    info.card = (*(*dev).card).index;
    *(&mut info.guid[0] as *mut _ as *mut __be32) = cpu_to_be32((*dev).config_rom[3]);
    *(&mut info.guid[4] as *mut _ as *mut __be32) = cpu_to_be32((*dev).config_rom[4]);
    strscpy(
        info.device_name.as_mut_ptr(),
        dev_name(&mut (*dev).device),
        size_of_val(&info.device_name),
    );

    if copy_to_user(
        arg,
        &info as *const _ as *const c_void,
        size_of::<snd_firewire_get_info>(),
    ) != 0
    {
        return -EFAULT;
    }

    0
}

unsafe fn hwdep_lock(ff: *mut snd_ff) -> c_int {
    spin_lock_irq(&mut (*ff).lock);

    let ret = if (*ff).dev_lock_count == 0 {
        (*ff).dev_lock_count = -1;
        0
    } else {
        -EBUSY
    };

    spin_unlock_irq(&mut (*ff).lock);
    ret
}

unsafe fn hwdep_unlock(ff: *mut snd_ff) -> c_int {
    spin_lock_irq(&mut (*ff).lock);

    let ret = if (*ff).dev_lock_count == -1 {
        (*ff).dev_lock_count = 0;
        0
    } else {
        -EBADFD
    };

    spin_unlock_irq(&mut (*ff).lock);
    ret
}

unsafe extern "C" fn hwdep_release(_hwdep: *mut snd_hwdep, _file: *mut file) -> c_int {
    let ff = (*_hwdep).private_data as *mut snd_ff;

    spin_lock_irq(&mut (*ff).lock);
    if (*ff).dev_lock_count == -1 {
        (*ff).dev_lock_count = 0;
    }
    spin_unlock_irq(&mut (*ff).lock);

    0
}

unsafe extern "C" fn hwdep_ioctl(
    hwdep: *mut snd_hwdep,
    file: *mut file,
    cmd: c_uint,
    arg: c_ulong,
) -> c_int {
    let ff = (*hwdep).private_data as *mut snd_ff;

    match cmd {
        SNDRV_FIREWIRE_IOCTL_GET_INFO => hwdep_get_info(ff, arg as *mut c_void),
        SNDRV_FIREWIRE_IOCTL_LOCK => hwdep_lock(ff),
        SNDRV_FIREWIRE_IOCTL_UNLOCK => hwdep_unlock(ff),
        _ => -ENOIOCTLCMD,
    }
}

// C conditional: #ifdef CONFIG_COMPAT
#[cfg(CONFIG_COMPAT)]
unsafe extern "C" fn hwdep_compat_ioctl(
    hwdep: *mut snd_hwdep,
    file: *mut file,
    cmd: c_uint,
    arg: c_ulong,
) -> c_int {
    hwdep_ioctl(hwdep, file, cmd, compat_ptr(arg) as c_ulong)
}

#[cfg(CONFIG_COMPAT)]
const HWDEP_COMPAT_IOCTL: Option<
    unsafe extern "C" fn(*mut snd_hwdep, *mut file, c_uint, c_ulong) -> c_int,
> = Some(hwdep_compat_ioctl);

#[cfg(not(CONFIG_COMPAT))]
const HWDEP_COMPAT_IOCTL: Option<
    unsafe extern "C" fn(*mut snd_hwdep, *mut file, c_uint, c_ulong) -> c_int,
> = None;

pub unsafe extern "C" fn snd_ff_create_hwdep_devices(ff: *mut snd_ff) -> c_int {
    let hwdep_ops = snd_hwdep_ops {
        read: Some(hwdep_read),
        release: Some(hwdep_release),
        poll: Some(hwdep_poll),
        ioctl: Some(hwdep_ioctl),
        ioctl_compat: HWDEP_COMPAT_IOCTL,
    };
    let mut hwdep: *mut snd_hwdep = ptr::null_mut();
    let mut err: c_int;

    err = snd_hwdep_new(
        (*ff).card,
        (*(*ff).card).driver.as_mut_ptr(),
        0,
        &mut hwdep,
    );
    if err < 0 {
        return err;
    }

    strscpy((*hwdep).name.as_mut_ptr(), (*(*ff).card).driver.as_mut_ptr(), size_of_val(&(*hwdep).name));
    (*hwdep).iface = SNDRV_HWDEP_IFACE_FW_FIREFACE;
    (*hwdep).ops = hwdep_ops;
    (*hwdep).private_data = ff as *mut c_void;
    (*hwdep).exclusive = true;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
