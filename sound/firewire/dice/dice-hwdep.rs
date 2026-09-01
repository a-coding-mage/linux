// SPDX-License-Identifier: GPL-2.0-only
/*
 * dice_hwdep.c - a part of driver for DICE based devices
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 * Copyright (c) 2014 Takashi Sakamoto <o-takashi@sakamocchi.jp>
 */

// Rust translation of implementation depending on declarations from "dice.h"
// and the surrounding Linux/ALSA FireWire kernel APIs.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type loff_t = i64;
type __poll_t = c_uint;
type __be32 = u32;

const TASK_INTERRUPTIBLE: c_int = 1;
const ERESTARTSYS: c_long = 512;
const EFAULT: c_int = 14;
const EBUSY: c_int = 16;
const EBADFD: c_int = 77;
const ENOIOCTLCMD: c_int = 515;

const EPOLLIN: __poll_t = 0x0000_0001;
const EPOLLRDNORM: __poll_t = 0x0000_0040;

const SNDRV_FIREWIRE_EVENT_LOCK_STATUS: c_uint = 0;
const SNDRV_FIREWIRE_EVENT_DICE_NOTIFICATION: c_uint = 0;
const SNDRV_FIREWIRE_TYPE_DICE: c_uint = 0;
const SNDRV_FIREWIRE_IOCTL_GET_INFO: c_uint = 0;
const SNDRV_FIREWIRE_IOCTL_LOCK: c_uint = 0;
const SNDRV_FIREWIRE_IOCTL_UNLOCK: c_uint = 0;
const SNDRV_HWDEP_IFACE_FW_DICE: c_int = 0;

#[repr(C)]
pub struct snd_hwdep {
    pub private_data: *mut c_void,
    pub name: [c_char; 32],
    pub iface: c_int,
    pub ops: snd_hwdep_ops,
    pub exclusive: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_hwdep_ops {
    pub read: Option<
        unsafe extern "C" fn(*mut snd_hwdep, *mut c_char, c_long, *mut loff_t) -> c_long,
    >,
    pub release: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut file) -> c_int>,
    pub poll: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut file, *mut poll_table) -> __poll_t>,
    pub ioctl:
        Option<unsafe extern "C" fn(*mut snd_hwdep, *mut file, c_uint, c_ulong) -> c_int>,
    pub ioctl_compat:
        Option<unsafe extern "C" fn(*mut snd_hwdep, *mut file, c_uint, c_ulong) -> c_int>,
}

#[repr(C)]
pub struct snd_dice {
    pub lock: spinlock_t,
    pub hwdep_wait: wait_queue_head_t,
    pub dev_lock_changed: bool,
    pub notification_bits: c_uint,
    pub dev_lock_count: c_int,
    pub unit: *mut fw_unit,
    pub card: *mut snd_card,
}

#[repr(C)]
pub struct snd_card {
    pub index: c_int,
}

#[repr(C)]
pub struct fw_device {
    pub card: *mut fw_card,
    pub config_rom: *mut u32,
    pub device: device,
}

#[repr(C)]
pub struct fw_card {
    pub index: c_int,
}

#[repr(C)]
pub struct snd_firewire_event_lock_status {
    pub type_: c_uint,
    pub status: bool,
}

#[repr(C)]
pub struct snd_firewire_event_dice_notification {
    pub type_: c_uint,
    pub notification: c_uint,
}

#[repr(C)]
pub union snd_firewire_event {
    pub lock_status: core::mem::ManuallyDrop<snd_firewire_event_lock_status>,
    pub dice_notification: core::mem::ManuallyDrop<snd_firewire_event_dice_notification>,
}

#[repr(C)]
pub struct snd_firewire_get_info {
    pub type_: c_uint,
    pub card: c_int,
    pub guid: [u8; 8],
    pub device_name: [c_char; 32],
}

#[repr(C)]
pub struct wait_queue_entry_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct poll_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_unit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

unsafe extern "C" {
    static current: *mut c_void;

    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn prepare_to_wait(
        queue: *mut wait_queue_head_t,
        wait: *mut wait_queue_entry_t,
        state: c_int,
    );
    fn schedule();
    fn finish_wait(queue: *mut wait_queue_head_t, wait: *mut wait_queue_entry_t);
    fn signal_pending(task: *mut c_void) -> c_int;
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> c_ulong;
    fn poll_wait(file: *mut file, wait_address: *mut wait_queue_head_t, p: *mut poll_table);
    fn fw_parent_device(unit: *mut fw_unit) -> *mut fw_device;
    fn cpu_to_be32(value: u32) -> __be32;
    fn dev_name(dev: *const device) -> *const c_char;
    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn compat_ptr(arg: c_ulong) -> *mut c_void;
    fn snd_hwdep_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        rhwdep: *mut *mut snd_hwdep,
    ) -> c_int;
}

unsafe extern "C" fn hwdep_read(
    hwdep: *mut snd_hwdep,
    buf: *mut c_char,
    mut count: c_long,
    _offset: *mut loff_t,
) -> c_long {
    let dice = (*hwdep).private_data as *mut snd_dice;
    let mut wait: wait_queue_entry_t = zeroed();
    let mut event: snd_firewire_event = zeroed();

    spin_lock_irq(ptr::addr_of_mut!((*dice).lock));

    while !(*dice).dev_lock_changed && (*dice).notification_bits == 0 {
        prepare_to_wait(
            ptr::addr_of_mut!((*dice).hwdep_wait),
            ptr::addr_of_mut!(wait),
            TASK_INTERRUPTIBLE,
        );
        spin_unlock_irq(ptr::addr_of_mut!((*dice).lock));
        schedule();
        finish_wait(
            ptr::addr_of_mut!((*dice).hwdep_wait),
            ptr::addr_of_mut!(wait),
        );
        if signal_pending(current) != 0 {
            return -ERESTARTSYS;
        }
        spin_lock_irq(ptr::addr_of_mut!((*dice).lock));
    }

    ptr::write_bytes(
        ptr::addr_of_mut!(event) as *mut u8,
        0,
        size_of::<snd_firewire_event>(),
    );
    if (*dice).dev_lock_changed {
        event.lock_status.type_ = SNDRV_FIREWIRE_EVENT_LOCK_STATUS;
        event.lock_status.status = (*dice).dev_lock_count > 0;
        (*dice).dev_lock_changed = false;

        count = core::cmp::min(count, size_of::<snd_firewire_event_lock_status>() as c_long);
    } else {
        event.dice_notification.type_ = SNDRV_FIREWIRE_EVENT_DICE_NOTIFICATION;
        event.dice_notification.notification = (*dice).notification_bits;
        (*dice).notification_bits = 0;

        count = core::cmp::min(
            count,
            size_of::<snd_firewire_event_dice_notification>() as c_long,
        );
    }

    spin_unlock_irq(ptr::addr_of_mut!((*dice).lock));

    if copy_to_user(
        buf as *mut c_void,
        ptr::addr_of!(event) as *const c_void,
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
    let dice = (*hwdep).private_data as *mut snd_dice;

    poll_wait(file, ptr::addr_of_mut!((*dice).hwdep_wait), wait);

    spin_lock_irq(ptr::addr_of_mut!((*dice).lock));
    let ret = if (*dice).dev_lock_changed || (*dice).notification_bits != 0 {
        EPOLLIN | EPOLLRDNORM
    } else {
        0
    };
    spin_unlock_irq(ptr::addr_of_mut!((*dice).lock));
    ret
}

unsafe fn hwdep_get_info(dice: *mut snd_dice, arg: *mut c_void) -> c_int {
    let dev = fw_parent_device((*dice).unit);
    let mut info: snd_firewire_get_info = zeroed();

    ptr::write_bytes(
        ptr::addr_of_mut!(info) as *mut u8,
        0,
        size_of::<snd_firewire_get_info>(),
    );
    info.type_ = SNDRV_FIREWIRE_TYPE_DICE;
    info.card = (*(*dev).card).index;
    *(ptr::addr_of_mut!(info.guid[0]) as *mut __be32) = cpu_to_be32(*(*dev).config_rom.add(3));
    *(ptr::addr_of_mut!(info.guid[4]) as *mut __be32) = cpu_to_be32(*(*dev).config_rom.add(4));
    strscpy(
        info.device_name.as_mut_ptr(),
        dev_name(ptr::addr_of!((*dev).device)),
        info.device_name.len(),
    );

    if copy_to_user(
        arg,
        ptr::addr_of!(info) as *const c_void,
        size_of::<snd_firewire_get_info>(),
    ) != 0
    {
        return -EFAULT;
    }

    0
}

unsafe fn hwdep_lock(dice: *mut snd_dice) -> c_int {
    spin_lock_irq(ptr::addr_of_mut!((*dice).lock));
    let ret = if (*dice).dev_lock_count == 0 {
        (*dice).dev_lock_count = -1;
        0
    } else {
        -EBUSY
    };
    spin_unlock_irq(ptr::addr_of_mut!((*dice).lock));
    ret
}

unsafe fn hwdep_unlock(dice: *mut snd_dice) -> c_int {
    spin_lock_irq(ptr::addr_of_mut!((*dice).lock));
    let ret = if (*dice).dev_lock_count == -1 {
        (*dice).dev_lock_count = 0;
        0
    } else {
        -EBADFD
    };
    spin_unlock_irq(ptr::addr_of_mut!((*dice).lock));
    ret
}

unsafe extern "C" fn hwdep_release(hwdep: *mut snd_hwdep, _file: *mut file) -> c_int {
    let dice = (*hwdep).private_data as *mut snd_dice;

    spin_lock_irq(ptr::addr_of_mut!((*dice).lock));
    if (*dice).dev_lock_count == -1 {
        (*dice).dev_lock_count = 0;
    }
    spin_unlock_irq(ptr::addr_of_mut!((*dice).lock));

    0
}

unsafe extern "C" fn hwdep_ioctl(
    hwdep: *mut snd_hwdep,
    file: *mut file,
    cmd: c_uint,
    arg: c_ulong,
) -> c_int {
    let dice = (*hwdep).private_data as *mut snd_dice;

    match cmd {
        SNDRV_FIREWIRE_IOCTL_GET_INFO => hwdep_get_info(dice, arg as *mut c_void),
        SNDRV_FIREWIRE_IOCTL_LOCK => hwdep_lock(dice),
        SNDRV_FIREWIRE_IOCTL_UNLOCK => hwdep_unlock(dice),
        _ => -ENOIOCTLCMD,
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_dice_create_hwdep(dice: *mut snd_dice) -> c_int {
    static OPS: snd_hwdep_ops = snd_hwdep_ops {
        read: Some(hwdep_read),
        release: Some(hwdep_release),
        poll: Some(hwdep_poll),
        ioctl: Some(hwdep_ioctl),
        ioctl_compat: Some(hwdep_compat_ioctl),
    };
    let mut hwdep: *mut snd_hwdep = ptr::null_mut();
    let mut err: c_int;

    err = snd_hwdep_new(
        (*dice).card,
        c"DICE".as_ptr(),
        0,
        ptr::addr_of_mut!(hwdep),
    );
    if err < 0 {
        return err;
    }
    strscpy((*hwdep).name.as_mut_ptr(), c"DICE".as_ptr(), (*hwdep).name.len());
    (*hwdep).iface = SNDRV_HWDEP_IFACE_FW_DICE;
    (*hwdep).ops = OPS;
    (*hwdep).private_data = dice as *mut c_void;
    (*hwdep).exclusive = true;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
