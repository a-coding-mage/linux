// SPDX-License-Identifier: GPL-2.0-only
/*
 * oxfw_hwdep.c - a part of driver for OXFW970/971 based devices
 *
 * Copyright (c) 2014 Takashi Sakamoto
 */

/*
 * This codes give three functionality.
 *
 * 1.get firewire node information
 * 2.get notification about starting/stopping stream
 * 3.lock/unlock stream
 */

// Translated from C source including "oxfw.h"; external kernel/ALSA types,
// constants, and helpers are expected to be supplied by surrounding bindings.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type loff_t = i64;
type __poll_t = c_uint;
type __be32 = u32;

const TASK_INTERRUPTIBLE: c_int = 1;

extern "C" {
    static current: *mut task_struct;

    static SNDRV_FIREWIRE_EVENT_LOCK_STATUS: c_uint;
    static SNDRV_FIREWIRE_TYPE_OXFW: c_uint;
    static SNDRV_FIREWIRE_IOCTL_GET_INFO: c_uint;
    static SNDRV_FIREWIRE_IOCTL_LOCK: c_uint;
    static SNDRV_FIREWIRE_IOCTL_UNLOCK: c_uint;
    static SNDRV_HWDEP_IFACE_FW_OXFW: c_int;
    static EPOLLIN: __poll_t;
    static EPOLLRDNORM: __poll_t;
    static ERESTARTSYS: c_int;
    static EFAULT: c_int;
    static EBUSY: c_int;
    static EBADFD: c_int;
    static ENOIOCTLCMD: c_int;

    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn prepare_to_wait(queue: *mut wait_queue_head_t, wait: *mut wait_queue_entry_t, state: c_int);
    fn finish_wait(queue: *mut wait_queue_head_t, wait: *mut wait_queue_entry_t);
    fn schedule();
    fn signal_pending(task: *mut task_struct) -> c_int;
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> c_ulong;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn poll_wait(file: *mut file, wait_address: *mut wait_queue_head_t, p: *mut poll_table);
    fn fw_parent_device(unit: *mut fw_unit) -> *mut fw_device;
    fn cpu_to_be32(x: u32) -> __be32;
    fn dev_name(dev: *const device) -> *const c_char;
    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn snd_hwdep_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        rhwdep: *mut *mut snd_hwdep,
    ) -> c_int;

    // Present only when CONFIG_COMPAT is enabled in the original C build.
    fn compat_ptr(uptr: c_ulong) -> *mut c_void;
}

#[repr(C)]
struct snd_hwdep {
    name: [c_char; 32],
    iface: c_int,
    ops: snd_hwdep_ops,
    private_data: *mut c_void,
    exclusive: bool,
}

#[repr(C)]
struct snd_hwdep_ops {
    read: Option<
        unsafe extern "C" fn(
            hwdep: *mut snd_hwdep,
            buf: *mut c_char,
            count: c_long,
            offset: *mut loff_t,
        ) -> c_long,
    >,
    release: Option<unsafe extern "C" fn(hwdep: *mut snd_hwdep, file: *mut file) -> c_int>,
    poll: Option<
        unsafe extern "C" fn(
            hwdep: *mut snd_hwdep,
            file: *mut file,
            wait: *mut poll_table,
        ) -> __poll_t,
    >,
    ioctl: Option<
        unsafe extern "C" fn(
            hwdep: *mut snd_hwdep,
            file: *mut file,
            cmd: c_uint,
            arg: c_ulong,
        ) -> c_int,
    >,
    ioctl_compat: Option<
        unsafe extern "C" fn(
            hwdep: *mut snd_hwdep,
            file: *mut file,
            cmd: c_uint,
            arg: c_ulong,
        ) -> c_int,
    >,
}

#[repr(C)]
struct snd_oxfw {
    lock: spinlock_t,
    hwdep_wait: wait_queue_head_t,
    dev_lock_changed: bool,
    dev_lock_count: c_int,
    unit: *mut fw_unit,
    card: *mut snd_card,
}

#[repr(C)]
struct snd_card {
    index: c_int,
    driver: [c_char; 16],
}

#[repr(C)]
struct fw_device {
    card: *mut fw_card,
    config_rom: *mut u32,
    device: device,
}

#[repr(C)]
struct fw_card {
    index: c_int,
}

#[repr(C)]
struct snd_firewire_lock_status {
    type_: c_uint,
    status: c_uint,
}

#[repr(C)]
struct snd_firewire_get_info {
    type_: c_uint,
    card: c_int,
    guid: [u8; 8],
    device_name: [c_char; 32],
}

#[repr(C)]
union snd_firewire_event {
    lock_status: core::mem::ManuallyDrop<snd_firewire_lock_status>,
}

#[repr(C)]
struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
struct wait_queue_entry_t {
    _private: [u8; 0],
}

#[repr(C)]
struct file {
    _private: [u8; 0],
}

#[repr(C)]
struct poll_table {
    _private: [u8; 0],
}

#[repr(C)]
struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
struct fw_unit {
    _private: [u8; 0],
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

unsafe extern "C" fn hwdep_read(
    hwdep: *mut snd_hwdep,
    buf: *mut c_char,
    mut count: c_long,
    _offset: *mut loff_t,
) -> c_long {
    let oxfw = (*hwdep).private_data as *mut snd_oxfw;
    let mut wait: wait_queue_entry_t = core::mem::zeroed();
    let mut event: snd_firewire_event = core::mem::zeroed();

    spin_lock_irq(&mut (*oxfw).lock);

    while !(*oxfw).dev_lock_changed {
        prepare_to_wait(
            &mut (*oxfw).hwdep_wait,
            &mut wait,
            TASK_INTERRUPTIBLE,
        );
        spin_unlock_irq(&mut (*oxfw).lock);
        schedule();
        finish_wait(&mut (*oxfw).hwdep_wait, &mut wait);
        if signal_pending(current) != 0 {
            return -(ERESTARTSYS as c_long);
        }
        spin_lock_irq(&mut (*oxfw).lock);
    }

    memset(
        &mut event as *mut snd_firewire_event as *mut c_void,
        0,
        size_of::<snd_firewire_event>(),
    );
    event.lock_status.type_ = SNDRV_FIREWIRE_EVENT_LOCK_STATUS;
    event.lock_status.status = ((*oxfw).dev_lock_count > 0) as c_uint;
    (*oxfw).dev_lock_changed = false;

    count = core::cmp::min(count, size_of::<snd_firewire_lock_status>() as c_long);

    spin_unlock_irq(&mut (*oxfw).lock);

    if copy_to_user(
        buf as *mut c_void,
        &event as *const snd_firewire_event as *const c_void,
        count as usize,
    ) != 0
    {
        return -(EFAULT as c_long);
    }

    count
}

unsafe extern "C" fn hwdep_poll(
    hwdep: *mut snd_hwdep,
    file: *mut file,
    wait: *mut poll_table,
) -> __poll_t {
    let oxfw = (*hwdep).private_data as *mut snd_oxfw;

    poll_wait(file, &mut (*oxfw).hwdep_wait, wait);

    spin_lock_irq(&mut (*oxfw).lock);
    let ret = if (*oxfw).dev_lock_changed {
        EPOLLIN | EPOLLRDNORM
    } else {
        0
    };
    spin_unlock_irq(&mut (*oxfw).lock);

    ret
}

unsafe extern "C" fn hwdep_get_info(oxfw: *mut snd_oxfw, arg: *mut c_void) -> c_int {
    let dev = fw_parent_device((*oxfw).unit);
    let mut info: snd_firewire_get_info = core::mem::zeroed();

    memset(
        &mut info as *mut snd_firewire_get_info as *mut c_void,
        0,
        size_of::<snd_firewire_get_info>(),
    );
    info.type_ = SNDRV_FIREWIRE_TYPE_OXFW;
    info.card = (*(*dev).card).index;
    ptr::write_unaligned(
        info.guid.as_mut_ptr().add(0) as *mut __be32,
        cpu_to_be32(*(*dev).config_rom.add(3)),
    );
    ptr::write_unaligned(
        info.guid.as_mut_ptr().add(4) as *mut __be32,
        cpu_to_be32(*(*dev).config_rom.add(4)),
    );
    strscpy(
        info.device_name.as_mut_ptr(),
        dev_name(&(*dev).device),
        size_of::<[c_char; 32]>(),
    );

    if copy_to_user(
        arg,
        &info as *const snd_firewire_get_info as *const c_void,
        size_of::<snd_firewire_get_info>(),
    ) != 0
    {
        return -EFAULT;
    }

    0
}

unsafe extern "C" fn hwdep_lock(oxfw: *mut snd_oxfw) -> c_int {
    spin_lock_irq(&mut (*oxfw).lock);

    let ret = if (*oxfw).dev_lock_count == 0 {
        (*oxfw).dev_lock_count = -1;
        0
    } else {
        -EBUSY
    };

    spin_unlock_irq(&mut (*oxfw).lock);
    ret
}

unsafe extern "C" fn hwdep_unlock(oxfw: *mut snd_oxfw) -> c_int {
    spin_lock_irq(&mut (*oxfw).lock);

    let ret = if (*oxfw).dev_lock_count == -1 {
        (*oxfw).dev_lock_count = 0;
        0
    } else {
        -EBADFD
    };

    spin_unlock_irq(&mut (*oxfw).lock);
    ret
}

unsafe extern "C" fn hwdep_release(hwdep: *mut snd_hwdep, _file: *mut file) -> c_int {
    let oxfw = (*hwdep).private_data as *mut snd_oxfw;

    spin_lock_irq(&mut (*oxfw).lock);
    if (*oxfw).dev_lock_count == -1 {
        (*oxfw).dev_lock_count = 0;
    }
    spin_unlock_irq(&mut (*oxfw).lock);

    0
}

unsafe extern "C" fn hwdep_ioctl(
    hwdep: *mut snd_hwdep,
    file: *mut file,
    cmd: c_uint,
    arg: c_ulong,
) -> c_int {
    let oxfw = (*hwdep).private_data as *mut snd_oxfw;
    let _ = file;

    if cmd == SNDRV_FIREWIRE_IOCTL_GET_INFO {
        hwdep_get_info(oxfw, arg as *mut c_void)
    } else if cmd == SNDRV_FIREWIRE_IOCTL_LOCK {
        hwdep_lock(oxfw)
    } else if cmd == SNDRV_FIREWIRE_IOCTL_UNLOCK {
        hwdep_unlock(oxfw)
    } else {
        -ENOIOCTLCMD
    }
}

// Original C condition:
// #ifdef CONFIG_COMPAT
unsafe extern "C" fn hwdep_compat_ioctl(
    hwdep: *mut snd_hwdep,
    file: *mut file,
    cmd: c_uint,
    arg: c_ulong,
) -> c_int {
    hwdep_ioctl(hwdep, file, cmd, compat_ptr(arg) as c_ulong)
}
// #else
// #define hwdep_compat_ioctl NULL
// #endif

#[no_mangle]
pub unsafe extern "C" fn snd_oxfw_create_hwdep(oxfw: *mut snd_oxfw) -> c_int {
    static HWDEP_OPS: snd_hwdep_ops = snd_hwdep_ops {
        read: Some(hwdep_read),
        release: Some(hwdep_release),
        poll: Some(hwdep_poll),
        ioctl: Some(hwdep_ioctl),
        ioctl_compat: Some(hwdep_compat_ioctl),
    };
    let mut hwdep: *mut snd_hwdep = ptr::null_mut();
    let mut err: c_int;

    err = snd_hwdep_new(
        (*oxfw).card,
        (*(*oxfw).card).driver.as_ptr(),
        0,
        &mut hwdep,
    );
    if err < 0 {
        return err;
    }
    strscpy(
        (*hwdep).name.as_mut_ptr(),
        (*(*oxfw).card).driver.as_ptr(),
        size_of::<[c_char; 32]>(),
    );
    (*hwdep).iface = SNDRV_HWDEP_IFACE_FW_OXFW;
    (*hwdep).ops = HWDEP_OPS;
    (*hwdep).private_data = oxfw as *mut c_void;
    (*hwdep).exclusive = true;

    err
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
