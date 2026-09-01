// SPDX-License-Identifier: GPL-2.0-only
/*
 * tascam-hwdep.c - a part of driver for TASCAM FireWire series
 *
 * Copyright (c) 2015 Takashi Sakamoto
 */

/*
 * This codes give three functionality.
 *
 * 1.get firewire node information
 * 2.get notification about starting/stopping stream
 * 3.lock/unlock stream
 */

// Depends on declarations from "tascam.h" and Linux/ALSA FireWire kernel APIs.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

unsafe fn tscm_hwdep_read_locked(
    tscm: *mut snd_tscm,
    buf: *mut c_char,
    mut count: c_long,
    _offset: *mut loff_t,
) -> c_long {
    let mut event = snd_firewire_event_lock_status {
        type_: SNDRV_FIREWIRE_EVENT_LOCK_STATUS,
        ..unsafe { core::mem::zeroed() }
    };

    event.status = unsafe { ((*tscm).dev_lock_count > 0) as _ };
    unsafe {
        (*tscm).dev_lock_changed = false;
    }
    count = min_t_long(count, size_of::<snd_firewire_event_lock_status>() as c_long);

    unsafe {
        spin_unlock_irq(&mut (*tscm).lock);
    }

    if unsafe { copy_to_user(buf as *mut c_void, &event as *const _ as *const c_void, count as usize) } != 0 {
        return -EFAULT;
    }

    count
}

unsafe fn tscm_hwdep_read_queue(
    tscm: *mut snd_tscm,
    buf: *mut c_char,
    mut remained: c_long,
    _offset: *mut loff_t,
) -> c_long {
    let mut pos = buf;
    let type_: c_uint = SNDRV_FIREWIRE_EVENT_TASCAM_CONTROL;
    let entries = unsafe { (*tscm).queue };
    let mut count: c_long;

    // At least, one control event can be copied.
    if remained
        < (size_of::<c_uint>() + size_of::<snd_firewire_tascam_change>()) as c_long
    {
        unsafe {
            spin_unlock_irq(&mut (*tscm).lock);
        }
        return -EINVAL;
    }

    // Copy the type field later.
    count = size_of::<c_uint>() as c_long;
    remained -= size_of::<c_uint>() as c_long;
    pos = unsafe { pos.add(size_of::<c_uint>()) };

    loop {
        let head_pos: c_uint;
        let mut tail_pos: c_uint;
        let mut length: c_uint;

        if unsafe { (*tscm).pull_pos == (*tscm).push_pos } {
            break;
        } else if unsafe { (*tscm).pull_pos < (*tscm).push_pos } {
            tail_pos = unsafe { (*tscm).push_pos };
        } else {
            tail_pos = SND_TSCM_QUEUE_COUNT;
        }
        head_pos = unsafe { (*tscm).pull_pos };

        length = (tail_pos - head_pos) * size_of::<snd_firewire_tascam_change>() as c_uint;
        if remained < length as c_long {
            length = rounddown(remained as c_uint, size_of::<snd_firewire_tascam_change>() as c_uint);
        }
        if length == 0 {
            break;
        }
        tail_pos = head_pos + length / size_of::<snd_firewire_tascam_change>() as c_uint;

        unsafe {
            spin_unlock_irq(&mut (*tscm).lock);
        }
        if unsafe {
            copy_to_user(
                pos as *mut c_void,
                entries.add(head_pos as usize) as *const c_void,
                length as usize,
            )
        } != 0
        {
            return -EFAULT;
        }

        unsafe {
            spin_lock_irq(&mut (*tscm).lock);
        }

        unsafe {
            (*tscm).pull_pos = tail_pos % SND_TSCM_QUEUE_COUNT;
        }

        count += length as c_long;
        remained -= length as c_long;
        pos = unsafe { pos.add(length as usize) };
    }

    unsafe {
        spin_unlock_irq(&mut (*tscm).lock);
    }

    if unsafe {
        copy_to_user(
            buf as *mut c_void,
            &type_ as *const _ as *const c_void,
            size_of::<c_uint>(),
        )
    } != 0
    {
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
    let tscm = unsafe { (*hwdep).private_data as *mut snd_tscm };
    let mut wait: wait_queue_entry_t = unsafe { core::mem::zeroed() };

    unsafe {
        init_wait(&mut wait);
        spin_lock_irq(&mut (*tscm).lock);
    }

    while unsafe { !(*tscm).dev_lock_changed && (*tscm).push_pos == (*tscm).pull_pos } {
        unsafe {
            prepare_to_wait(&mut (*tscm).hwdep_wait, &mut wait, TASK_INTERRUPTIBLE);
            spin_unlock_irq(&mut (*tscm).lock);
            schedule();
            finish_wait(&mut (*tscm).hwdep_wait, &mut wait);
        }
        if unsafe { signal_pending(current) } != 0 {
            return -ERESTARTSYS;
        }
        unsafe {
            spin_lock_irq(&mut (*tscm).lock);
        }
    }

    // NOTE: The acquired lock should be released in callee side.
    if unsafe { (*tscm).dev_lock_changed } {
        count = unsafe { tscm_hwdep_read_locked(tscm, buf, count, offset) };
    } else if unsafe { (*tscm).push_pos != (*tscm).pull_pos } {
        count = unsafe { tscm_hwdep_read_queue(tscm, buf, count, offset) };
    } else {
        unsafe {
            spin_unlock_irq(&mut (*tscm).lock);
        }
        count = 0;
    }

    count
}

unsafe fn hwdep_poll(
    hwdep: *mut snd_hwdep,
    file: *mut file,
    wait: *mut poll_table,
) -> __poll_t {
    let tscm = unsafe { (*hwdep).private_data as *mut snd_tscm };

    unsafe {
        poll_wait(file, &mut (*tscm).hwdep_wait, wait);
    }

    unsafe {
        spin_lock_irq(&mut (*tscm).lock);
    }
    let ready = unsafe { (*tscm).dev_lock_changed || (*tscm).push_pos != (*tscm).pull_pos };
    unsafe {
        spin_unlock_irq(&mut (*tscm).lock);
    }

    if ready {
        EPOLLIN | EPOLLRDNORM
    } else {
        0
    }
}

unsafe fn hwdep_get_info(tscm: *mut snd_tscm, arg: *mut c_void) -> c_int {
    let dev = unsafe { fw_parent_device((*tscm).unit) };
    let mut info: snd_firewire_get_info = unsafe { core::mem::zeroed() };

    unsafe {
        ptr::write_bytes(
            &mut info as *mut _ as *mut u8,
            0,
            size_of::<snd_firewire_get_info>(),
        );
    }
    info.type_ = SNDRV_FIREWIRE_TYPE_TASCAM;
    info.card = unsafe { (*(*dev).card).index };
    unsafe {
        *(info.guid.as_mut_ptr().add(0) as *mut __be32) = cpu_to_be32(*(*dev).config_rom.add(3));
        *(info.guid.as_mut_ptr().add(4) as *mut __be32) = cpu_to_be32(*(*dev).config_rom.add(4));
        strscpy(
            info.device_name.as_mut_ptr(),
            dev_name(&mut (*dev).device),
            size_of_val(&info.device_name),
        );
    }

    if unsafe {
        copy_to_user(
            arg,
            &info as *const _ as *const c_void,
            size_of::<snd_firewire_get_info>(),
        )
    } != 0
    {
        return -EFAULT;
    }

    0
}

unsafe fn hwdep_lock(tscm: *mut snd_tscm) -> c_int {
    unsafe {
        spin_lock_irq(&mut (*tscm).lock);
    }

    let err = if unsafe { (*tscm).dev_lock_count == 0 } {
        unsafe {
            (*tscm).dev_lock_count = -1;
        }
        0
    } else {
        -EBUSY
    };

    unsafe {
        spin_unlock_irq(&mut (*tscm).lock);
    }

    err
}

unsafe fn hwdep_unlock(tscm: *mut snd_tscm) -> c_int {
    unsafe {
        spin_lock_irq(&mut (*tscm).lock);
    }

    let err = if unsafe { (*tscm).dev_lock_count == -1 } {
        unsafe {
            (*tscm).dev_lock_count = 0;
        }
        0
    } else {
        -EBADFD
    };

    unsafe {
        spin_unlock_irq(&mut (*tscm).lock);
    }

    err
}

unsafe fn tscm_hwdep_state(tscm: *mut snd_tscm, arg: *mut c_void) -> c_int {
    if unsafe {
        copy_to_user(
            arg,
            (*tscm).state.as_ptr() as *const c_void,
            size_of_val(&(*tscm).state),
        )
    } != 0
    {
        return -EFAULT;
    }

    0
}

unsafe fn hwdep_release(hwdep: *mut snd_hwdep, _file: *mut file) -> c_int {
    let tscm = unsafe { (*hwdep).private_data as *mut snd_tscm };

    unsafe {
        spin_lock_irq(&mut (*tscm).lock);
    }
    if unsafe { (*tscm).dev_lock_count == -1 } {
        unsafe {
            (*tscm).dev_lock_count = 0;
        }
    }
    unsafe {
        spin_unlock_irq(&mut (*tscm).lock);
    }

    0
}

unsafe fn hwdep_ioctl(
    hwdep: *mut snd_hwdep,
    file: *mut file,
    cmd: c_uint,
    arg: c_ulong,
) -> c_int {
    let tscm = unsafe { (*hwdep).private_data as *mut snd_tscm };

    match cmd {
        SNDRV_FIREWIRE_IOCTL_GET_INFO => unsafe { hwdep_get_info(tscm, arg as *mut c_void) },
        SNDRV_FIREWIRE_IOCTL_LOCK => unsafe { hwdep_lock(tscm) },
        SNDRV_FIREWIRE_IOCTL_UNLOCK => unsafe { hwdep_unlock(tscm) },
        SNDRV_FIREWIRE_IOCTL_TASCAM_STATE => unsafe { tscm_hwdep_state(tscm, arg as *mut c_void) },
        _ => -ENOIOCTLCMD,
    }
}

// CONFIG_COMPAT conditional:
// When CONFIG_COMPAT is enabled, hwdep_compat_ioctl forwards through compat_ptr(arg).
// Otherwise, the ioctl_compat operation is NULL.
#[cfg(CONFIG_COMPAT)]
unsafe fn hwdep_compat_ioctl(
    hwdep: *mut snd_hwdep,
    file: *mut file,
    cmd: c_uint,
    arg: c_ulong,
) -> c_int {
    unsafe { hwdep_ioctl(hwdep, file, cmd, compat_ptr(arg) as c_ulong) }
}

#[cfg(not(CONFIG_COMPAT))]
const hwdep_compat_ioctl: Option<
    unsafe fn(*mut snd_hwdep, *mut file, c_uint, c_ulong) -> c_int,
> = None;

#[no_mangle]
pub unsafe extern "C" fn snd_tscm_create_hwdep_device(tscm: *mut snd_tscm) -> c_int {
    let ops = snd_hwdep_ops {
        read: Some(hwdep_read),
        release: Some(hwdep_release),
        poll: Some(hwdep_poll),
        ioctl: Some(hwdep_ioctl),
        #[cfg(CONFIG_COMPAT)]
        ioctl_compat: Some(hwdep_compat_ioctl),
        #[cfg(not(CONFIG_COMPAT))]
        ioctl_compat: hwdep_compat_ioctl,
        ..unsafe { core::mem::zeroed() }
    };
    let mut hwdep: *mut snd_hwdep = ptr::null_mut();
    let mut err: c_int;

    err = unsafe {
        snd_hwdep_new(
            (*tscm).card,
            c"Tascam".as_ptr(),
            0,
            &mut hwdep as *mut *mut snd_hwdep,
        )
    };
    if err < 0 {
        return err;
    }

    unsafe {
        strscpy(
            (*hwdep).name.as_mut_ptr(),
            c"Tascam".as_ptr(),
            size_of_val(&(*hwdep).name),
        );
        (*hwdep).iface = SNDRV_HWDEP_IFACE_FW_TASCAM;
        (*hwdep).ops = ops;
        (*hwdep).private_data = tscm as *mut c_void;
        (*hwdep).exclusive = true;

        (*tscm).hwdep = hwdep;
    }

    err
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
