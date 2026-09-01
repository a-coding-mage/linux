// SPDX-License-Identifier: GPL-2.0-only
/*
 * digi00x-hwdep.c - a part of driver for Digidesign Digi 002/003 family
 *
 * Copyright (c) 2014-2015 Takashi Sakamoto
 */

/*
 * This codes give three functionality.
 *
 * 1.get firewire node information
 * 2.get notification about starting/stopping stream
 * 3.lock/unlock stream
 * 4.get asynchronous messaging
 */

// Rust translation of the implementation originally depending on "digi00x.h".

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type loff_t = i64;
type __poll_t = c_uint;
type __be32 = u32;

const TASK_INTERRUPTIBLE: c_int = 1;
const SNDRV_FIREWIRE_EVENT_LOCK_STATUS: c_uint = 0;
const SNDRV_FIREWIRE_EVENT_DIGI00X_MESSAGE: c_uint = 0;
const SNDRV_FIREWIRE_TYPE_DIGI00X: c_uint = 0;
const SNDRV_FIREWIRE_IOCTL_GET_INFO: c_uint = 0;
const SNDRV_FIREWIRE_IOCTL_LOCK: c_uint = 0;
const SNDRV_FIREWIRE_IOCTL_UNLOCK: c_uint = 0;
const SNDRV_HWDEP_IFACE_FW_DIGI00X: c_int = 0;
const EPOLLIN: __poll_t = 0x00000001;
const EPOLLRDNORM: __poll_t = 0x00000040;
const ERESTARTSYS: c_int = 512;
const EFAULT: c_int = 14;
const EBUSY: c_int = 16;
const EBADFD: c_int = 77;
const ENOIOCTLCMD: c_int = 515;

#[repr(C)]
pub struct snd_hwdep {
    pub private_data: *mut c_void,
    pub name: [c_char; 32],
    pub iface: c_int,
    pub ops: snd_hwdep_ops,
    pub exclusive: bool,
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
pub struct wait_queue_head_t {
    _private: [u8; 0],
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
pub struct snd_card {
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

#[repr(C)]
pub struct fw_card {
    pub index: c_int,
}

#[repr(C)]
pub struct fw_device {
    pub card: *mut fw_card,
    pub config_rom: *mut u32,
    pub device: device,
}

#[repr(C)]
pub struct snd_dg00x {
    pub lock: spinlock_t,
    pub hwdep_wait: wait_queue_head_t,
    pub dev_lock_changed: bool,
    pub msg: u32,
    pub dev_lock_count: c_int,
    pub unit: *mut fw_unit,
    pub card: *mut snd_card,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_firewire_event_lock_status {
    pub type_: c_uint,
    pub status: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_firewire_event_digi00x_message {
    pub type_: c_uint,
    pub message: u32,
}

#[repr(C)]
pub union snd_firewire_event {
    pub lock_status: snd_firewire_event_lock_status,
    pub digi00x_message: snd_firewire_event_digi00x_message,
}

#[repr(C)]
pub struct snd_firewire_get_info {
    pub type_: c_uint,
    pub card: c_int,
    pub guid: [u8; 8],
    pub device_name: [c_char; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_hwdep_ops {
    pub read: Option<
        unsafe extern "C" fn(*mut snd_hwdep, *mut c_char, c_long, *mut loff_t) -> c_long,
    >,
    pub release: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut file) -> c_int>,
    pub poll: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut file, *mut poll_table) -> __poll_t>,
    pub ioctl: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut file, c_uint, c_ulong) -> c_int>,
    pub ioctl_compat:
        Option<unsafe extern "C" fn(*mut snd_hwdep, *mut file, c_uint, c_ulong) -> c_int>,
}

extern "C" {
    static mut current: *mut c_void;

    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn prepare_to_wait(
        wq_head: *mut wait_queue_head_t,
        wq_entry: *mut wait_queue_entry_t,
        state: c_int,
    );
    fn schedule();
    fn finish_wait(wq_head: *mut wait_queue_head_t, wq_entry: *mut wait_queue_entry_t);
    fn signal_pending(task: *mut c_void) -> c_int;
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> c_ulong;
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

    // CONFIG_COMPAT dependency.
    fn compat_ptr(arg: c_ulong) -> *mut c_void;
}

unsafe extern "C" fn hwdep_read(
    hwdep: *mut snd_hwdep,
    buf: *mut c_char,
    mut count: c_long,
    _offset: *mut loff_t,
) -> c_long {
    let dg00x = (*hwdep).private_data as *mut snd_dg00x;
    let mut wait: wait_queue_entry_t = zeroed();
    let mut event: snd_firewire_event = zeroed();

    spin_lock_irq(&mut (*dg00x).lock);

    while !(*dg00x).dev_lock_changed && (*dg00x).msg == 0 {
        prepare_to_wait(
            &mut (*dg00x).hwdep_wait,
            &mut wait,
            TASK_INTERRUPTIBLE,
        );
        spin_unlock_irq(&mut (*dg00x).lock);
        schedule();
        finish_wait(&mut (*dg00x).hwdep_wait, &mut wait);
        if signal_pending(current) != 0 {
            return -(ERESTARTSYS as c_long);
        }
        spin_lock_irq(&mut (*dg00x).lock);
    }

    ptr::write_bytes(
        &mut event as *mut snd_firewire_event as *mut u8,
        0,
        size_of::<snd_firewire_event>(),
    );
    if (*dg00x).dev_lock_changed {
        event.lock_status.type_ = SNDRV_FIREWIRE_EVENT_LOCK_STATUS;
        event.lock_status.status = (*dg00x).dev_lock_count > 0;
        (*dg00x).dev_lock_changed = false;

        count = core::cmp::min(count, size_of::<snd_firewire_event_lock_status>() as c_long);
    } else {
        event.digi00x_message.type_ = SNDRV_FIREWIRE_EVENT_DIGI00X_MESSAGE;
        event.digi00x_message.message = (*dg00x).msg;
        (*dg00x).msg = 0;

        count = core::cmp::min(
            count,
            size_of::<snd_firewire_event_digi00x_message>() as c_long,
        );
    }

    spin_unlock_irq(&mut (*dg00x).lock);

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
    let dg00x = (*hwdep).private_data as *mut snd_dg00x;

    poll_wait(file, &mut (*dg00x).hwdep_wait, wait);

    spin_lock_irq(&mut (*dg00x).lock);
    let ret = if (*dg00x).dev_lock_changed || (*dg00x).msg != 0 {
        EPOLLIN | EPOLLRDNORM
    } else {
        0
    };
    spin_unlock_irq(&mut (*dg00x).lock);

    ret
}

unsafe extern "C" fn hwdep_get_info(dg00x: *mut snd_dg00x, arg: *mut c_void) -> c_int {
    let dev = fw_parent_device((*dg00x).unit);
    let mut info: snd_firewire_get_info = zeroed();

    info.type_ = SNDRV_FIREWIRE_TYPE_DIGI00X;
    info.card = (*(*dev).card).index;
    *(&mut info.guid[0] as *mut u8 as *mut __be32) =
        cpu_to_be32(*(*dev).config_rom.add(3));
    *(&mut info.guid[4] as *mut u8 as *mut __be32) =
        cpu_to_be32(*(*dev).config_rom.add(4));
    strscpy(
        info.device_name.as_mut_ptr(),
        dev_name(&(*dev).device),
        info.device_name.len(),
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

unsafe extern "C" fn hwdep_lock(dg00x: *mut snd_dg00x) -> c_int {
    spin_lock_irq(&mut (*dg00x).lock);
    let ret = if (*dg00x).dev_lock_count == 0 {
        (*dg00x).dev_lock_count = -1;
        0
    } else {
        -EBUSY
    };
    spin_unlock_irq(&mut (*dg00x).lock);

    ret
}

unsafe extern "C" fn hwdep_unlock(dg00x: *mut snd_dg00x) -> c_int {
    spin_lock_irq(&mut (*dg00x).lock);
    let ret = if (*dg00x).dev_lock_count == -1 {
        (*dg00x).dev_lock_count = 0;
        0
    } else {
        -EBADFD
    };
    spin_unlock_irq(&mut (*dg00x).lock);

    ret
}

unsafe extern "C" fn hwdep_release(hwdep: *mut snd_hwdep, _file: *mut file) -> c_int {
    let dg00x = (*hwdep).private_data as *mut snd_dg00x;

    spin_lock_irq(&mut (*dg00x).lock);
    if (*dg00x).dev_lock_count == -1 {
        (*dg00x).dev_lock_count = 0;
    }
    spin_unlock_irq(&mut (*dg00x).lock);

    0
}

unsafe extern "C" fn hwdep_ioctl(
    hwdep: *mut snd_hwdep,
    file: *mut file,
    cmd: c_uint,
    arg: c_ulong,
) -> c_int {
    let dg00x = (*hwdep).private_data as *mut snd_dg00x;

    match cmd {
        SNDRV_FIREWIRE_IOCTL_GET_INFO => hwdep_get_info(dg00x, arg as *mut c_void),
        SNDRV_FIREWIRE_IOCTL_LOCK => hwdep_lock(dg00x),
        SNDRV_FIREWIRE_IOCTL_UNLOCK => hwdep_unlock(dg00x),
        _ => -ENOIOCTLCMD,
    }
}

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
pub unsafe extern "C" fn snd_dg00x_create_hwdep_device(dg00x: *mut snd_dg00x) -> c_int {
    let ops = snd_hwdep_ops {
        read: Some(hwdep_read),
        release: Some(hwdep_release),
        poll: Some(hwdep_poll),
        ioctl: Some(hwdep_ioctl),
        ioctl_compat: Some(hwdep_compat_ioctl),
    };
    let mut hwdep: *mut snd_hwdep = ptr::null_mut();
    let mut err: c_int;

    err = snd_hwdep_new(
        (*dg00x).card,
        b"Digi00x\0".as_ptr() as *const c_char,
        0,
        &mut hwdep,
    );
    if err < 0 {
        return err;
    }

    strscpy((*hwdep).name.as_mut_ptr(), b"Digi00x\0".as_ptr() as *const c_char, (*hwdep).name.len());
    (*hwdep).iface = SNDRV_HWDEP_IFACE_FW_DIGI00X;
    (*hwdep).ops = ops;
    (*hwdep).private_data = dg00x as *mut c_void;
    (*hwdep).exclusive = true;

    err
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
