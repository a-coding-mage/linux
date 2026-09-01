// SPDX-License-Identifier: GPL-2.0-only
/*
 * motu-hwdep.c - a part of driver for MOTU FireWire series
 *
 * Copyright (c) 2015-2017 Takashi Sakamoto <o-takashi@sakamocchi.jp>
 */

/*
 * This codes have five functionalities.
 *
 * 1.get information about firewire node
 * 2.get notification about starting/stopping stream
 * 3.lock/unlock streaming
 *
 */

// C dependency intent: #include "motu.h"
use crate::*;

unsafe fn has_dsp_event(motu: *mut snd_motu) -> bool {
    if ((*(*motu).spec).flags & SND_MOTU_SPEC_REGISTER_DSP) != 0 {
        snd_motu_register_dsp_message_parser_count_event(motu) > 0
    } else {
        false
    }
}

unsafe fn hwdep_read(
    hwdep: *mut snd_hwdep,
    buf: *mut c_char,
    mut count: c_long,
    offset: *mut loff_t,
) -> c_long {
    let motu: *mut snd_motu = (*hwdep).private_data as *mut snd_motu;
    let mut wait = DEFINE_WAIT();
    let mut event: snd_firewire_event = core::mem::zeroed();

    let _ = offset;

    spin_lock_irq(&mut (*motu).lock);

    while !(*motu).dev_lock_changed && (*motu).msg == 0 && !has_dsp_event(motu) {
        prepare_to_wait(&mut (*motu).hwdep_wait, &mut wait, TASK_INTERRUPTIBLE);
        spin_unlock_irq(&mut (*motu).lock);
        schedule();
        finish_wait(&mut (*motu).hwdep_wait, &mut wait);
        if signal_pending(current) != 0 {
            return -ERESTARTSYS;
        }
        spin_lock_irq(&mut (*motu).lock);
    }

    core::ptr::write_bytes(
        &mut event as *mut snd_firewire_event as *mut u8,
        0,
        core::mem::size_of::<snd_firewire_event>(),
    );
    if (*motu).dev_lock_changed {
        event.lock_status.type_ = SNDRV_FIREWIRE_EVENT_LOCK_STATUS;
        event.lock_status.status = (*motu).dev_lock_count > 0;
        (*motu).dev_lock_changed = false;
        spin_unlock_irq(&mut (*motu).lock);

        count = min_t_long(count, core::mem::size_of_val(&event) as c_long);
        if copy_to_user(buf as *mut c_void, &event as *const _ as *const c_void, count as usize) != 0 {
            return -EFAULT;
        }
    } else if (*motu).msg > 0 {
        event.motu_notification.type_ = SNDRV_FIREWIRE_EVENT_MOTU_NOTIFICATION;
        event.motu_notification.message = (*motu).msg;
        (*motu).msg = 0;
        spin_unlock_irq(&mut (*motu).lock);

        count = min_t_long(count, core::mem::size_of_val(&event) as c_long);
        if copy_to_user(buf as *mut c_void, &event as *const _ as *const c_void, count as usize) != 0 {
            return -EFAULT;
        }
    } else if has_dsp_event(motu) {
        let mut consumed: usize = 0;
        let mut ptr: *mut u32;
        let mut ev: u32 = 0;

        spin_unlock_irq(&mut (*motu).lock);

        // Header is filled later.
        consumed += core::mem::size_of_val(&event.motu_register_dsp_change);

        while consumed < count as usize
            && snd_motu_register_dsp_message_parser_copy_event(motu, &mut ev)
        {
            ptr = buf.add(consumed) as *mut u32;
            if consumed + core::mem::size_of_val(&ev) > count as usize || put_user_u32(ev, ptr) != 0 {
                return -EFAULT;
            }
            consumed += core::mem::size_of_val(&ev);
        }

        event.motu_register_dsp_change.type_ = SNDRV_FIREWIRE_EVENT_MOTU_REGISTER_DSP_CHANGE;
        event.motu_register_dsp_change.count =
            ((consumed - core::mem::size_of_val(&event.motu_register_dsp_change)) / 4) as _;
        if copy_to_user(
            buf as *mut c_void,
            &event as *const _ as *const c_void,
            min_t_long(
                count,
                core::mem::size_of_val(&event.motu_register_dsp_change) as c_long,
            ) as usize,
        ) != 0
        {
            return -EFAULT;
        }

        count = min_t_long(count, consumed as c_long);
    } else {
        spin_unlock_irq(&mut (*motu).lock);

        count = 0;
    }

    count
}

unsafe fn hwdep_poll(
    hwdep: *mut snd_hwdep,
    file: *mut file,
    wait: *mut poll_table,
) -> __poll_t {
    let motu: *mut snd_motu = (*hwdep).private_data as *mut snd_motu;

    poll_wait(file, &mut (*motu).hwdep_wait, wait);

    spin_lock_irq(&mut (*motu).lock);
    let result = if (*motu).dev_lock_changed || (*motu).msg != 0 || has_dsp_event(motu) {
        EPOLLIN | EPOLLRDNORM
    } else {
        0
    };
    spin_unlock_irq(&mut (*motu).lock);

    result
}

unsafe fn hwdep_get_info(motu: *mut snd_motu, arg: *mut c_void) -> c_int {
    let dev: *mut fw_device = fw_parent_device((*motu).unit);
    let mut info: snd_firewire_get_info = core::mem::zeroed();

    core::ptr::write_bytes(
        &mut info as *mut snd_firewire_get_info as *mut u8,
        0,
        core::mem::size_of::<snd_firewire_get_info>(),
    );
    info.type_ = SNDRV_FIREWIRE_TYPE_MOTU;
    info.card = (*(*dev).card).index;
    *(&mut info.guid[0] as *mut _ as *mut __be32) = cpu_to_be32((*dev).config_rom[3]);
    *(&mut info.guid[4] as *mut _ as *mut __be32) = cpu_to_be32((*dev).config_rom[4]);
    strscpy(
        info.device_name.as_mut_ptr(),
        dev_name(&mut (*dev).device),
        core::mem::size_of_val(&info.device_name),
    );

    if copy_to_user(
        arg,
        &info as *const snd_firewire_get_info as *const c_void,
        core::mem::size_of_val(&info),
    ) != 0
    {
        return -EFAULT;
    }

    0
}

unsafe fn hwdep_lock(motu: *mut snd_motu) -> c_int {
    spin_lock_irq(&mut (*motu).lock);
    let result = if (*motu).dev_lock_count == 0 {
        (*motu).dev_lock_count = -1;
        0
    } else {
        -EBUSY
    };
    spin_unlock_irq(&mut (*motu).lock);

    result
}

unsafe fn hwdep_unlock(motu: *mut snd_motu) -> c_int {
    spin_lock_irq(&mut (*motu).lock);
    let result = if (*motu).dev_lock_count == -1 {
        (*motu).dev_lock_count = 0;
        0
    } else {
        -EBADFD
    };
    spin_unlock_irq(&mut (*motu).lock);

    result
}

unsafe fn hwdep_release(hwdep: *mut snd_hwdep, file: *mut file) -> c_int {
    let motu: *mut snd_motu = (*hwdep).private_data as *mut snd_motu;

    let _ = file;

    spin_lock_irq(&mut (*motu).lock);
    if (*motu).dev_lock_count == -1 {
        (*motu).dev_lock_count = 0;
    }
    spin_unlock_irq(&mut (*motu).lock);

    0
}

unsafe fn hwdep_ioctl(
    hwdep: *mut snd_hwdep,
    file: *mut file,
    cmd: c_uint,
    arg: c_ulong,
) -> c_int {
    let motu: *mut snd_motu = (*hwdep).private_data as *mut snd_motu;

    let _ = file;

    match cmd {
        SNDRV_FIREWIRE_IOCTL_GET_INFO => hwdep_get_info(motu, arg as *mut c_void),
        SNDRV_FIREWIRE_IOCTL_LOCK => hwdep_lock(motu),
        SNDRV_FIREWIRE_IOCTL_UNLOCK => hwdep_unlock(motu),
        SNDRV_FIREWIRE_IOCTL_MOTU_REGISTER_DSP_METER => {
            let meter: *mut snd_firewire_motu_register_dsp_meter;
            let err: c_int;

            if ((*(*motu).spec).flags & SND_MOTU_SPEC_REGISTER_DSP) == 0 {
                return -ENXIO;
            }

            meter = kzalloc_obj::<snd_firewire_motu_register_dsp_meter>();
            if meter.is_null() {
                return -ENOMEM;
            }

            snd_motu_register_dsp_message_parser_copy_meter(motu, meter);

            err = copy_to_user(
                arg as *mut c_void,
                meter as *const c_void,
                core::mem::size_of::<snd_firewire_motu_register_dsp_meter>(),
            );
            kfree(meter as *mut c_void);

            if err != 0 {
                return -EFAULT;
            }

            0
        }
        SNDRV_FIREWIRE_IOCTL_MOTU_COMMAND_DSP_METER => {
            let meter: *mut snd_firewire_motu_command_dsp_meter;
            let err: c_int;

            if ((*(*motu).spec).flags & SND_MOTU_SPEC_COMMAND_DSP) == 0 {
                return -ENXIO;
            }

            meter = kzalloc_obj::<snd_firewire_motu_command_dsp_meter>();
            if meter.is_null() {
                return -ENOMEM;
            }

            snd_motu_command_dsp_message_parser_copy_meter(motu, meter);

            err = copy_to_user(
                arg as *mut c_void,
                meter as *const c_void,
                core::mem::size_of::<snd_firewire_motu_command_dsp_meter>(),
            );
            kfree(meter as *mut c_void);

            if err != 0 {
                return -EFAULT;
            }

            0
        }
        SNDRV_FIREWIRE_IOCTL_MOTU_REGISTER_DSP_PARAMETER => {
            let param: *mut snd_firewire_motu_register_dsp_parameter;
            let err: c_int;

            if ((*(*motu).spec).flags & SND_MOTU_SPEC_REGISTER_DSP) == 0 {
                return -ENXIO;
            }

            param = kzalloc_obj::<snd_firewire_motu_register_dsp_parameter>();
            if param.is_null() {
                return -ENOMEM;
            }

            snd_motu_register_dsp_message_parser_copy_parameter(motu, param);

            err = copy_to_user(
                arg as *mut c_void,
                param as *const c_void,
                core::mem::size_of::<snd_firewire_motu_register_dsp_parameter>(),
            );
            kfree(param as *mut c_void);
            if err != 0 {
                return -EFAULT;
            }

            0
        }
        _ => -ENOIOCTLCMD,
    }
}

// C conditional intent: #ifdef CONFIG_COMPAT
#[cfg(CONFIG_COMPAT)]
unsafe fn hwdep_compat_ioctl(
    hwdep: *mut snd_hwdep,
    file: *mut file,
    cmd: c_uint,
    arg: c_ulong,
) -> c_int {
    hwdep_ioctl(hwdep, file, cmd, compat_ptr(arg) as c_ulong)
}

// C conditional intent: #else #define hwdep_compat_ioctl NULL
#[cfg(not(CONFIG_COMPAT))]
const hwdep_compat_ioctl: Option<
    unsafe fn(*mut snd_hwdep, *mut file, c_uint, c_ulong) -> c_int,
> = None;

pub unsafe fn snd_motu_create_hwdep_device(motu: *mut snd_motu) -> c_int {
    static OPS: snd_hwdep_ops = snd_hwdep_ops {
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
                hwdep_compat_ioctl
            }
        },
    };
    let mut hwdep: *mut snd_hwdep = core::ptr::null_mut();
    let mut err: c_int;

    err = snd_hwdep_new(
        (*motu).card,
        (*(*motu).card).driver.as_ptr(),
        0,
        &mut hwdep,
    );
    if err < 0 {
        return err;
    }

    strscpy((*hwdep).name.as_mut_ptr(), c"MOTU".as_ptr(), core::mem::size_of_val(&(*hwdep).name));
    (*hwdep).iface = SNDRV_HWDEP_IFACE_FW_MOTU;
    (*hwdep).ops = OPS;
    (*hwdep).private_data = motu as *mut c_void;
    (*hwdep).exclusive = true;

    (*motu).hwdep = hwdep;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
