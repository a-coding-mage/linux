// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Hardware dependent layer
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type loff_t = i64;
type ssize_t = isize;
type size_t = usize;
type __poll_t = c_uint;

const ENXIO: c_int = 6;
const ENODEV: c_int = 19;
const EBUSY: c_int = 16;
const EAGAIN: c_int = 11;
const ERESTARTSYS: c_int = 512;
const EFAULT: c_int = 14;
const EINVAL: c_int = 22;
const ENOTTY: c_int = 25;
const ENOIOCTLCMD: c_int = 515;
const ENOMEM: c_int = 12;

const O_NONBLOCK: c_int = 0o0004000;
const TASK_INTERRUPTIBLE: c_long = 1;

const SNDRV_DEVICE_TYPE_HWDEP: c_int = 0;
const SNDRV_OSS_DEVICE_TYPE_DMFM: c_int = 0;
const SNDRV_MINOR_HWDEPS: c_int = 4;
const SNDRV_DEV_HWDEP: c_int = 0;
const SNDRV_HWDEP_VERSION: c_int = 0;
const SNDRV_HWDEP_IOCTL_PVERSION: c_uint = 0;
const SNDRV_HWDEP_IOCTL_INFO: c_uint = 1;
const SNDRV_HWDEP_IOCTL_DSP_STATUS: c_uint = 2;
const SNDRV_HWDEP_IOCTL_DSP_LOAD: c_uint = 3;
const SNDRV_CTL_IOCTL_HWDEP_NEXT_DEVICE: c_uint = 4;
const SNDRV_CTL_IOCTL_HWDEP_INFO: c_uint = 5;
const SOUND_MAJOR: c_int = 14;

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
pub struct mutex {
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
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    private_data: *mut c_void,
    f_flags: c_int,
}

#[repr(C)]
pub struct poll_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    number: c_int,
    module: *mut module,
    shutdown: c_int,
}

#[repr(C)]
pub struct snd_device {
    device_data: *mut c_void,
}

#[repr(C)]
pub struct snd_device_ops {
    dev_free: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
    dev_register: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
    dev_disconnect: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
}

#[repr(C)]
pub struct snd_hwdep_ops {
    llseek: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut file, loff_t, c_int) -> loff_t>,
    read: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    write: Option<unsafe extern "C" fn(*mut snd_hwdep, *const c_char, size_t, *mut loff_t) -> ssize_t>,
    open: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut file) -> c_int>,
    release: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut file) -> c_int>,
    poll: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut file, *mut poll_table) -> __poll_t>,
    ioctl: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut file, c_uint, c_ulong) -> c_long>,
    mmap: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut file, *mut vm_area_struct) -> c_int>,
    dsp_status: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut snd_hwdep_dsp_status) -> c_int>,
    dsp_load: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut snd_hwdep_dsp_image) -> c_int>,
}

#[repr(C)]
pub struct snd_hwdep {
    list: list_head,
    card: *mut snd_card,
    device: c_int,
    id: [c_char; 32],
    name: [c_char; 80],
    iface: c_int,
    ops: snd_hwdep_ops,
    exclusive: c_int,
    used: c_int,
    open_wait: wait_queue_head_t,
    open_mutex: mutex,
    dsp_loaded: c_uint,
    private_free: Option<unsafe extern "C" fn(*mut snd_hwdep)>,
    dev: *mut device,
    oss_type: c_int,
    ossreg: c_int,
}

#[repr(C)]
pub struct snd_hwdep_info {
    device: c_int,
    card: c_int,
    id: [c_char; 32],
    name: [c_char; 80],
    iface: c_int,
}

#[repr(C)]
pub struct snd_hwdep_dsp_status {
    dsp_loaded: c_uint,
}

#[repr(C)]
pub struct snd_hwdep_dsp_image {
    index: c_uint,
}

#[repr(C)]
pub struct snd_info_entry_text {
    read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
}

#[repr(C)]
pub struct snd_info_entry_c {
    text: snd_info_entry_text,
}

#[repr(C)]
pub struct snd_info_entry {
    c: snd_info_entry_c,
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file_operations {
    owner: *mut module,
    llseek: Option<unsafe extern "C" fn(*mut file, loff_t, c_int) -> loff_t>,
    read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    write: Option<unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t>,
    open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    poll: Option<unsafe extern "C" fn(*mut file, *mut poll_table) -> __poll_t>,
    unlocked_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
    compat_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
    mmap: Option<unsafe extern "C" fn(*mut file, *mut vm_area_struct) -> c_int>,
}

static mut snd_hwdep_devices: list_head = list_head {
    next: ptr::null_mut(),
    prev: ptr::null_mut(),
};
static mut register_mutex: mutex = mutex { _private: [] };
static mut snd_hwdep_proc_entry: *mut snd_info_entry = ptr::null_mut();

extern "C" {
    static mut THIS_MODULE: *mut module;
    static snd_major: c_int;
    static mut current: *mut c_void;

    fn imajor(inode: *mut inode) -> c_int;
    fn iminor(inode: *mut inode) -> c_int;
    fn snd_lookup_minor_data(minor: c_int, ty: c_int) -> *mut snd_hwdep;
    fn snd_lookup_oss_minor_data(minor: c_int, ty: c_int) -> *mut snd_hwdep;
    fn try_module_get(module: *mut module) -> bool;
    fn module_put(module: *mut module);
    fn snd_card_unref(card: *mut snd_card);
    fn init_waitqueue_entry(wait: *mut wait_queue_entry_t, task: *mut c_void);
    fn add_wait_queue(head: *mut wait_queue_head_t, wait: *mut wait_queue_entry_t);
    fn remove_wait_queue(head: *mut wait_queue_head_t, wait: *mut wait_queue_entry_t);
    fn init_waitqueue_head(head: *mut wait_queue_head_t);
    fn wake_up(head: *mut wait_queue_head_t);
    fn mutex_init(mutex: *mut mutex);
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
    fn set_current_state(state: c_long);
    fn schedule();
    fn signal_pending(task: *mut c_void) -> c_int;
    fn snd_card_file_add(card: *mut snd_card, file: *mut file) -> c_int;
    fn snd_card_file_remove(card: *mut snd_card, file: *mut file);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strscpy(dest: *mut c_char, src: *const c_char, count: size_t) -> ssize_t;
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: size_t) -> c_ulong;
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: size_t) -> c_ulong;
    fn put_user_int(value: c_int, ptr: *mut c_int) -> c_int;
    fn get_user_int(value: *mut c_int, ptr: *const c_int) -> c_int;
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn put_device(dev: *mut device);
    fn snd_device_alloc(dev: *mut *mut device, card: *mut snd_card) -> c_int;
    fn dev_set_name(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn snd_device_new(
        card: *mut snd_card,
        ty: c_int,
        device_data: *mut c_void,
        ops: *const snd_device_ops,
    ) -> c_int;
    fn snd_register_device(
        ty: c_int,
        card: *mut snd_card,
        device: c_int,
        f_ops: *const file_operations,
        private_data: *mut c_void,
        dev: *mut device,
    ) -> c_int;
    fn snd_unregister_device(dev: *mut device);
    fn snd_register_oss_device(
        ty: c_int,
        card: *mut snd_card,
        device: c_int,
        f_ops: *const file_operations,
        private_data: *mut c_void,
    ) -> c_int;
    fn snd_unregister_oss_device(ty: c_int, card: *mut snd_card, device: c_int);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn snd_BUG_ON(condition: bool) -> c_int;
    fn snd_ctl_register_ioctl(
        f: unsafe extern "C" fn(*mut snd_card, *mut snd_ctl_file, c_uint, c_ulong) -> c_int,
    );
    fn snd_ctl_register_ioctl_compat(
        f: unsafe extern "C" fn(*mut snd_card, *mut snd_ctl_file, c_uint, c_ulong) -> c_int,
    );
    fn snd_ctl_unregister_ioctl(
        f: unsafe extern "C" fn(*mut snd_card, *mut snd_ctl_file, c_uint, c_ulong) -> c_int,
    );
    fn snd_ctl_unregister_ioctl_compat(
        f: unsafe extern "C" fn(*mut snd_card, *mut snd_ctl_file, c_uint, c_ulong) -> c_int,
    );
    fn snd_info_create_module_entry(
        module: *mut module,
        name: *const c_char,
        parent: *mut snd_info_entry,
    ) -> *mut snd_info_entry;
    fn snd_info_register(entry: *mut snd_info_entry) -> c_int;
    fn snd_info_free_entry(entry: *mut snd_info_entry);
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
}

unsafe fn list_for_each_hwdep(mut f: impl FnMut(*mut snd_hwdep) -> bool) {
    let mut pos = snd_hwdep_devices.next as *mut snd_hwdep;
    while !pos.is_null() && (&mut (*pos).list as *mut list_head) != &mut snd_hwdep_devices {
        if !f(pos) {
            break;
        }
        pos = (*pos).list.next as *mut snd_hwdep;
    }
}

unsafe fn list_add_tail(new: *mut list_head, head: *mut list_head) {
    (*new).prev = (*head).prev;
    (*new).next = head;
    if !(*head).prev.is_null() {
        (*(*head).prev).next = new;
    }
    (*head).prev = new;
    if (*head).next.is_null() {
        (*head).next = new;
    }
}

unsafe fn list_del(entry: *mut list_head) {
    if !(*entry).next.is_null() {
        (*(*entry).next).prev = (*entry).prev;
    }
    if !(*entry).prev.is_null() {
        (*(*entry).prev).next = (*entry).next;
    }
}

unsafe fn list_del_init(entry: *mut list_head) {
    list_del(entry);
    (*entry).next = entry;
    (*entry).prev = entry;
}

unsafe extern "C" fn snd_hwdep_dev_free(device: *mut snd_device) -> c_int;
unsafe extern "C" fn snd_hwdep_dev_register(device: *mut snd_device) -> c_int;
unsafe extern "C" fn snd_hwdep_dev_disconnect(device: *mut snd_device) -> c_int;

unsafe fn snd_hwdep_search(card: *mut snd_card, device: c_int) -> *mut snd_hwdep {
    let mut found: *mut snd_hwdep = ptr::null_mut();

    list_for_each_hwdep(|hwdep| {
        if (*hwdep).card == card && (*hwdep).device == device {
            found = hwdep;
            return false;
        }
        true
    });
    found
}

unsafe extern "C" fn snd_hwdep_llseek(file: *mut file, offset: loff_t, orig: c_int) -> loff_t {
    let hw = (*file).private_data as *mut snd_hwdep;
    if let Some(llseek) = (*hw).ops.llseek {
        return llseek(hw, file, offset, orig);
    }
    -(ENXIO as loff_t)
}

unsafe extern "C" fn snd_hwdep_read(
    file: *mut file,
    buf: *mut c_char,
    count: size_t,
    offset: *mut loff_t,
) -> ssize_t {
    let hw = (*file).private_data as *mut snd_hwdep;
    if let Some(read) = (*hw).ops.read {
        return read(hw, buf, count, offset);
    }
    -(ENXIO as ssize_t)
}

unsafe extern "C" fn snd_hwdep_write(
    file: *mut file,
    buf: *const c_char,
    count: size_t,
    offset: *mut loff_t,
) -> ssize_t {
    let hw = (*file).private_data as *mut snd_hwdep;
    if let Some(write) = (*hw).ops.write {
        return write(hw, buf, count, offset);
    }
    -(ENXIO as ssize_t)
}

unsafe extern "C" fn snd_hwdep_open(inode: *mut inode, file: *mut file) -> c_int {
    let major = imajor(inode);
    let mut hw: *mut snd_hwdep;
    let mut err: c_int;
    let mut wait = core::mem::MaybeUninit::<wait_queue_entry_t>::uninit();

    if major == snd_major {
        hw = snd_lookup_minor_data(iminor(inode), SNDRV_DEVICE_TYPE_HWDEP);
    } else if major == SOUND_MAJOR {
        /* CONFIG_SND_OSSEMUL: OSS direct FM minor lookup. */
        hw = snd_lookup_oss_minor_data(iminor(inode), SNDRV_OSS_DEVICE_TYPE_DMFM);
    } else {
        return -ENXIO;
    }
    if hw.is_null() {
        return -ENODEV;
    }

    if !try_module_get((*(*hw).card).module) {
        snd_card_unref((*hw).card);
        return -ENODEV;
    }

    init_waitqueue_entry(wait.as_mut_ptr(), current);
    add_wait_queue(&mut (*hw).open_wait, wait.as_mut_ptr());
    mutex_lock(&mut (*hw).open_mutex);
    loop {
        if (*hw).exclusive != 0 && (*hw).used > 0 {
            err = -EBUSY;
            break;
        }
        if (*hw).ops.open.is_none() {
            err = 0;
            break;
        }
        err = (*hw).ops.open.unwrap()(hw, file);
        if err >= 0 {
            break;
        }
        if err == -EAGAIN {
            if ((*file).f_flags & O_NONBLOCK) != 0 {
                err = -EBUSY;
                break;
            }
        } else {
            break;
        }
        set_current_state(TASK_INTERRUPTIBLE);
        mutex_unlock(&mut (*hw).open_mutex);
        schedule();
        mutex_lock(&mut (*hw).open_mutex);
        if (*(*hw).card).shutdown != 0 {
            err = -ENODEV;
            break;
        }
        if signal_pending(current) != 0 {
            err = -ERESTARTSYS;
            break;
        }
    }
    remove_wait_queue(&mut (*hw).open_wait, wait.as_mut_ptr());
    if err >= 0 {
        err = snd_card_file_add((*hw).card, file);
        if err >= 0 {
            (*file).private_data = hw as *mut c_void;
            (*hw).used += 1;
        } else if let Some(release) = (*hw).ops.release {
            release(hw, file);
        }
    }
    mutex_unlock(&mut (*hw).open_mutex);
    if err < 0 {
        module_put((*(*hw).card).module);
    }
    snd_card_unref((*hw).card);
    err
}

unsafe extern "C" fn snd_hwdep_release(_inode: *mut inode, file: *mut file) -> c_int {
    let mut err: c_int = 0;
    let hw = (*file).private_data as *mut snd_hwdep;
    let mod_ = (*(*hw).card).module;

    mutex_lock(&mut (*hw).open_mutex);
    if let Some(release) = (*hw).ops.release {
        err = release(hw, file);
    }
    if (*hw).used > 0 {
        (*hw).used -= 1;
    }
    mutex_unlock(&mut (*hw).open_mutex);
    wake_up(&mut (*hw).open_wait);

    snd_card_file_remove((*hw).card, file);
    module_put(mod_);
    err
}

unsafe extern "C" fn snd_hwdep_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    let hw = (*file).private_data as *mut snd_hwdep;
    if let Some(poll) = (*hw).ops.poll {
        return poll(hw, file, wait);
    }
    0
}

unsafe fn snd_hwdep_info(hw: *mut snd_hwdep, _info: *mut snd_hwdep_info) -> c_int {
    let mut info = core::mem::MaybeUninit::<snd_hwdep_info>::uninit();

    memset(info.as_mut_ptr() as *mut c_void, 0, size_of::<snd_hwdep_info>());
    let info = info.as_mut_ptr();
    (*info).card = (*(*hw).card).number;
    strscpy((*info).id.as_mut_ptr(), (*hw).id.as_ptr(), (*info).id.len());
    strscpy((*info).name.as_mut_ptr(), (*hw).name.as_ptr(), (*info).name.len());
    (*info).iface = (*hw).iface;
    if copy_to_user(_info as *mut c_void, info as *const c_void, size_of::<snd_hwdep_info>()) != 0 {
        return -EFAULT;
    }
    0
}

unsafe fn snd_hwdep_dsp_status(
    hw: *mut snd_hwdep,
    _info: *mut snd_hwdep_dsp_status,
) -> c_int {
    let mut info = core::mem::MaybeUninit::<snd_hwdep_dsp_status>::uninit();
    let mut err: c_int;

    if (*hw).ops.dsp_status.is_none() {
        return -ENXIO;
    }
    memset(
        info.as_mut_ptr() as *mut c_void,
        0,
        size_of::<snd_hwdep_dsp_status>(),
    );
    let info = info.as_mut_ptr();
    (*info).dsp_loaded = (*hw).dsp_loaded;
    err = (*hw).ops.dsp_status.unwrap()(hw, info);
    if err < 0 {
        return err;
    }
    if copy_to_user(
        _info as *mut c_void,
        info as *const c_void,
        size_of::<snd_hwdep_dsp_status>(),
    ) != 0
    {
        return -EFAULT;
    }
    0
}

unsafe fn snd_hwdep_dsp_load(hw: *mut snd_hwdep, info: *mut snd_hwdep_dsp_image) -> c_int {
    let err: c_int;

    if (*hw).ops.dsp_load.is_none() {
        return -ENXIO;
    }
    if (*info).index >= 32 {
        return -EINVAL;
    }
    /* check whether the dsp was already loaded */
    if ((*hw).dsp_loaded & (1u32 << (*info).index)) != 0 {
        return -EBUSY;
    }
    err = (*hw).ops.dsp_load.unwrap()(hw, info);
    if err < 0 {
        return err;
    }
    (*hw).dsp_loaded |= 1u32 << (*info).index;
    0
}

unsafe fn snd_hwdep_dsp_load_user(
    hw: *mut snd_hwdep,
    _info: *mut snd_hwdep_dsp_image,
) -> c_int {
    let mut info = core::mem::MaybeUninit::<snd_hwdep_dsp_image>::zeroed();

    if copy_from_user(
        info.as_mut_ptr() as *mut c_void,
        _info as *const c_void,
        size_of::<snd_hwdep_dsp_image>(),
    ) != 0
    {
        return -EFAULT;
    }
    snd_hwdep_dsp_load(hw, info.as_mut_ptr())
}

unsafe extern "C" fn snd_hwdep_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let hw = (*file).private_data as *mut snd_hwdep;
    let argp = arg as *mut c_void;

    match cmd {
        SNDRV_HWDEP_IOCTL_PVERSION => {
            return put_user_int(SNDRV_HWDEP_VERSION, argp as *mut c_int) as c_long;
        }
        SNDRV_HWDEP_IOCTL_INFO => return snd_hwdep_info(hw, argp as *mut snd_hwdep_info) as c_long,
        SNDRV_HWDEP_IOCTL_DSP_STATUS => {
            return snd_hwdep_dsp_status(hw, argp as *mut snd_hwdep_dsp_status) as c_long;
        }
        SNDRV_HWDEP_IOCTL_DSP_LOAD => {
            return snd_hwdep_dsp_load_user(hw, argp as *mut snd_hwdep_dsp_image) as c_long;
        }
        _ => {}
    }
    if let Some(ioctl) = (*hw).ops.ioctl {
        return ioctl(hw, file, cmd, arg);
    }
    -(ENOTTY as c_long)
}

unsafe extern "C" fn snd_hwdep_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int {
    let hw = (*file).private_data as *mut snd_hwdep;
    if let Some(mmap) = (*hw).ops.mmap {
        return mmap(hw, file, vma);
    }
    -ENXIO
}

unsafe extern "C" fn snd_hwdep_control_ioctl(
    card: *mut snd_card,
    _control: *mut snd_ctl_file,
    cmd: c_uint,
    arg: c_ulong,
) -> c_int {
    match cmd {
        SNDRV_CTL_IOCTL_HWDEP_NEXT_DEVICE => {
            let mut device: c_int = 0;

            if get_user_int(&mut device, arg as *const c_int) != 0 {
                return -EFAULT;
            }

            mutex_lock(&mut register_mutex);
            if device < 0 {
                device = 0;
            } else if device < SNDRV_MINOR_HWDEPS {
                device += 1;
            } else {
                device = SNDRV_MINOR_HWDEPS;
            }

            while device < SNDRV_MINOR_HWDEPS {
                if !snd_hwdep_search(card, device).is_null() {
                    break;
                }
                device += 1;
            }
            if device >= SNDRV_MINOR_HWDEPS {
                device = -1;
            }
            mutex_unlock(&mut register_mutex);
            if put_user_int(device, arg as *mut c_int) != 0 {
                return -EFAULT;
            }
            return 0;
        }
        SNDRV_CTL_IOCTL_HWDEP_INFO => {
            let info = arg as *mut snd_hwdep_info;
            let mut device: c_int = 0;
            let hwdep: *mut snd_hwdep;

            if get_user_int(&mut device, &(*info).device as *const c_int) != 0 {
                return -EFAULT;
            }
            mutex_lock(&mut register_mutex);
            hwdep = snd_hwdep_search(card, device);
            if hwdep.is_null() {
                mutex_unlock(&mut register_mutex);
                return -ENXIO;
            }
            let ret = snd_hwdep_info(hwdep, info);
            mutex_unlock(&mut register_mutex);
            return ret;
        }
        _ => {}
    }
    -ENOIOCTLCMD
}

/* CONFIG_COMPAT includes hwdep_compat.c, otherwise snd_hwdep_ioctl_compat is NULL. */
const snd_hwdep_ioctl_compat: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long> =
    None;

/*

 */

static snd_hwdep_f_ops: file_operations = file_operations {
    owner: ptr::null_mut(),
    llseek: Some(snd_hwdep_llseek),
    read: Some(snd_hwdep_read),
    write: Some(snd_hwdep_write),
    open: Some(snd_hwdep_open),
    release: Some(snd_hwdep_release),
    poll: Some(snd_hwdep_poll),
    unlocked_ioctl: Some(snd_hwdep_ioctl),
    compat_ioctl: snd_hwdep_ioctl_compat,
    mmap: Some(snd_hwdep_mmap),
};

unsafe fn snd_hwdep_free(hwdep: *mut snd_hwdep) {
    if hwdep.is_null() {
        return;
    }
    if let Some(private_free) = (*hwdep).private_free {
        private_free(hwdep);
    }
    put_device((*hwdep).dev);
    kfree(hwdep as *mut c_void);
}

/**
 * snd_hwdep_new - create a new hwdep instance
 * @card: the card instance
 * @id: the id string
 * @device: the device index (zero-based)
 * @rhwdep: the pointer to store the new hwdep instance
 *
 * Creates a new hwdep instance with the given index on the card.
 * The callbacks (hwdep->ops) must be set on the returned instance
 * after this call manually by the caller.
 *
 * Return: Zero if successful, or a negative error code on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hwdep_new(
    card: *mut snd_card,
    id: *mut c_char,
    device: c_int,
    rhwdep: *mut *mut snd_hwdep,
) -> c_int {
    let mut hwdep: *mut snd_hwdep;
    let mut err: c_int;
    static ops: snd_device_ops = snd_device_ops {
        dev_free: Some(snd_hwdep_dev_free),
        dev_register: Some(snd_hwdep_dev_register),
        dev_disconnect: Some(snd_hwdep_dev_disconnect),
    };

    if snd_BUG_ON(card.is_null()) != 0 {
        return -ENXIO;
    }
    if !rhwdep.is_null() {
        *rhwdep = ptr::null_mut();
    }
    hwdep = kzalloc(size_of::<snd_hwdep>(), 0) as *mut snd_hwdep;
    if hwdep.is_null() {
        return -ENOMEM;
    }

    init_waitqueue_head(&mut (*hwdep).open_wait);
    mutex_init(&mut (*hwdep).open_mutex);
    (*hwdep).card = card;
    (*hwdep).device = device;
    if !id.is_null() {
        strscpy((*hwdep).id.as_mut_ptr(), id, (*hwdep).id.len());
    }

    err = snd_device_alloc(&mut (*hwdep).dev, card);
    if err < 0 {
        snd_hwdep_free(hwdep);
        return err;
    }

    dev_set_name((*hwdep).dev, b"hwC%iD%i\0".as_ptr() as *const c_char, (*card).number, device);
    /* CONFIG_SND_OSSEMUL */
    (*hwdep).oss_type = -1;

    err = snd_device_new(card, SNDRV_DEV_HWDEP, hwdep as *mut c_void, &ops);
    if err < 0 {
        snd_hwdep_free(hwdep);
        return err;
    }

    if !rhwdep.is_null() {
        *rhwdep = hwdep;
    }
    0
}

unsafe extern "C" fn snd_hwdep_dev_free(device: *mut snd_device) -> c_int {
    snd_hwdep_free((*device).device_data as *mut snd_hwdep);
    0
}

unsafe extern "C" fn snd_hwdep_dev_register(device: *mut snd_device) -> c_int {
    let hwdep = (*device).device_data as *mut snd_hwdep;
    let card = (*hwdep).card;
    let mut err: c_int;

    mutex_lock(&mut register_mutex);
    if !snd_hwdep_search(card, (*hwdep).device).is_null() {
        mutex_unlock(&mut register_mutex);
        return -EBUSY;
    }
    list_add_tail(&mut (*hwdep).list, &mut snd_hwdep_devices);
    err = snd_register_device(
        SNDRV_DEVICE_TYPE_HWDEP,
        (*hwdep).card,
        (*hwdep).device,
        &snd_hwdep_f_ops,
        hwdep as *mut c_void,
        (*hwdep).dev,
    );
    if err < 0 {
        dev_err((*hwdep).dev, b"unable to register\n\0".as_ptr() as *const c_char);
        list_del(&mut (*hwdep).list);
        mutex_unlock(&mut register_mutex);
        return err;
    }

    /* CONFIG_SND_OSSEMUL */
    (*hwdep).ossreg = 0;
    if (*hwdep).oss_type >= 0 {
        if (*hwdep).oss_type == SNDRV_OSS_DEVICE_TYPE_DMFM && (*hwdep).device != 0 {
            dev_warn(
                (*hwdep).dev,
                b"only hwdep device 0 can be registered as OSS direct FM device!\n\0".as_ptr()
                    as *const c_char,
            );
        } else if snd_register_oss_device(
            (*hwdep).oss_type,
            card,
            (*hwdep).device,
            &snd_hwdep_f_ops,
            hwdep as *mut c_void,
        ) < 0
        {
            dev_warn(
                (*hwdep).dev,
                b"unable to register OSS compatibility device\n\0".as_ptr() as *const c_char,
            );
        } else {
            (*hwdep).ossreg = 1;
        }
    }
    mutex_unlock(&mut register_mutex);
    0
}

unsafe extern "C" fn snd_hwdep_dev_disconnect(device: *mut snd_device) -> c_int {
    let hwdep = (*device).device_data as *mut snd_hwdep;

    if snd_BUG_ON(hwdep.is_null()) != 0 {
        return -ENXIO;
    }
    mutex_lock(&mut register_mutex);
    if snd_hwdep_search((*hwdep).card, (*hwdep).device) != hwdep {
        mutex_unlock(&mut register_mutex);
        return -EINVAL;
    }
    mutex_lock(&mut (*hwdep).open_mutex);
    wake_up(&mut (*hwdep).open_wait);
    /* CONFIG_SND_OSSEMUL */
    if (*hwdep).ossreg != 0 {
        snd_unregister_oss_device((*hwdep).oss_type, (*hwdep).card, (*hwdep).device);
    }
    snd_unregister_device((*hwdep).dev);
    list_del_init(&mut (*hwdep).list);
    mutex_unlock(&mut (*hwdep).open_mutex);
    mutex_unlock(&mut register_mutex);
    0
}

/*
 *  Info interface
 */

unsafe extern "C" fn snd_hwdep_proc_read(
    _entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    mutex_lock(&mut register_mutex);
    list_for_each_hwdep(|hwdep| {
        snd_iprintf(
            buffer,
            b"%02i-%02i: %s\n\0".as_ptr() as *const c_char,
            (*(*hwdep).card).number,
            (*hwdep).device,
            (*hwdep).name.as_ptr(),
        );
        true
    });
    mutex_unlock(&mut register_mutex);
}

unsafe fn snd_hwdep_proc_init() {
    let mut entry: *mut snd_info_entry;

    entry = snd_info_create_module_entry(
        THIS_MODULE,
        b"hwdep\0".as_ptr() as *const c_char,
        ptr::null_mut(),
    );
    if !entry.is_null() {
        (*entry).c.text.read = Some(snd_hwdep_proc_read);
        if snd_info_register(entry) < 0 {
            snd_info_free_entry(entry);
            entry = ptr::null_mut();
        }
    }
    snd_hwdep_proc_entry = entry;
}

unsafe fn snd_hwdep_proc_done() {
    snd_info_free_entry(snd_hwdep_proc_entry);
}

/*
 *  ENTRY functions
 */

unsafe extern "C" fn alsa_hwdep_init() -> c_int {
    snd_hwdep_proc_init();
    snd_ctl_register_ioctl(snd_hwdep_control_ioctl);
    snd_ctl_register_ioctl_compat(snd_hwdep_control_ioctl);
    0
}

unsafe extern "C" fn alsa_hwdep_exit() {
    snd_ctl_unregister_ioctl(snd_hwdep_control_ioctl);
    snd_ctl_unregister_ioctl_compat(snd_hwdep_control_ioctl);
    snd_hwdep_proc_done();
}

/* module_init(alsa_hwdep_init) */
/* module_exit(alsa_hwdep_exit) */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
