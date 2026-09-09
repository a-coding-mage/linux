// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2002 Steve Schmidtke
 */

// Kernel/UML dependencies supplied by the surrounding build.

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

#[repr(C)]
pub struct hostaudio_state {
    pub fd: c_int,
}

#[repr(C)]
pub struct hostmixer_state {
    pub fd: c_int,
}

pub const HOSTAUDIO_DEV_DSP: &[u8] = b"/dev/sound/dsp\0";
pub const HOSTAUDIO_DEV_MIXER: &[u8] = b"/dev/sound/mixer\0";

// Changed either at boot time or module load time. At boot, this is
// single-threaded; at module load, multiple modules would each have
// their own copy of these variables.
static mut dsp: *mut c_char = HOSTAUDIO_DEV_DSP.as_ptr() as *mut c_char;
static mut mixer: *mut c_char = HOSTAUDIO_DEV_MIXER.as_ptr() as *mut c_char;

#[cfg(not(feature = "module"))]
unsafe extern "C" fn set_dsp(name: *mut c_char, add: *mut c_int) -> c_int {
    *add = 0;
    dsp = name;
    0
}

#[cfg(not(feature = "module"))]
unsafe extern "C" fn set_mixer(name: *mut c_char, add: *mut c_int) -> c_int {
    *add = 0;
    mixer = name;
    0
}

unsafe extern "C" {
    static hostaudio_mutex: c_void;

    fn kmalloc(size: usize, flags: c_ulong) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn memdup_user(buffer: *const c_void, count: usize) -> *mut c_void;
    fn ptr_err(ptr: *mut c_void) -> c_long;
    fn is_err(ptr: *mut c_void) -> bool;
    fn copy_to_user(to: *mut c_void, from: *const c_void, count: usize) -> usize;
    fn get_user(value: *mut c_ulong, from: *const c_int) -> c_int;
    fn put_user(value: c_ulong, to: *mut c_int) -> c_int;
    fn os_read_file(fd: c_int, buf: *mut c_void, count: usize) -> c_int;
    fn os_write_file(fd: c_int, buf: *const c_void, count: usize) -> c_int;
    fn os_ioctl_generic(fd: c_int, cmd: c_uint, arg: c_ulong) -> c_int;
    fn os_open_file(path: *mut c_char, flags: c_int, mode: c_int) -> c_int;
    fn os_close_file(fd: c_int);
    fn mutex_lock(lock: *const c_void);
    fn mutex_unlock(lock: *const c_void);
    fn kernel_param_lock(module: *mut c_void);
    fn kernel_param_unlock(module: *mut c_void);
    fn register_sound_dsp(fops: *const file_operations, index: c_int) -> c_int;
    fn register_sound_mixer(fops: *const file_operations, index: c_int) -> c_int;
    fn unregister_sound_dsp(dev: c_int);
    fn unregister_sound_mixer(dev: c_int);
    fn compat_ptr_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
}

type c_uint = u32;

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct poll_table_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    pub f_mode: c_ulong,
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct file_operations {
    pub owner: *mut c_void,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, usize, *mut i64) -> isize>,
    pub write: Option<unsafe extern "C" fn(*mut file, *const c_char, usize, *mut i64) -> isize>,
    pub poll: Option<unsafe extern "C" fn(*mut file, *mut poll_table_struct) -> c_ulong>,
    pub unlocked_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
    pub compat_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
    pub mmap: *const c_void,
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
}

const FMODE_READ: c_ulong = 1;
const FMODE_WRITE: c_ulong = 2;
const EFAULT: c_int = 14;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;

// Sound ioctl constants are supplied by linux/soundcard.h.
extern "C" {
    static SNDCTL_DSP_SPEED: c_uint;
    static SNDCTL_DSP_STEREO: c_uint;
    static SNDCTL_DSP_GETBLKSIZE: c_uint;
    static SNDCTL_DSP_CHANNELS: c_uint;
    static SNDCTL_DSP_SUBDIVIDE: c_uint;
    static SNDCTL_DSP_SETFRAGMENT: c_uint;
}

pub unsafe extern "C" fn hostaudio_read(file: *mut file, buffer: *mut c_char, count: usize, _ppos: *mut i64) -> isize {
    let state = (*file).private_data as *mut hostaudio_state;
    let kbuf = kmalloc(count, 0);
    if kbuf.is_null() { return -(ENOMEM as isize); }
    let mut err = os_read_file((*state).fd, kbuf, count);
    if err >= 0 && copy_to_user(buffer as *mut c_void, kbuf, err as usize) != 0 { err = -EFAULT; }
    kfree(kbuf);
    err as isize
}

pub unsafe extern "C" fn hostaudio_write(file: *mut file, buffer: *const c_char, count: usize, ppos: *mut i64) -> isize {
    let state = (*file).private_data as *mut hostaudio_state;
    let kbuf = memdup_user(buffer as *const c_void, count);
    if is_err(kbuf) { return ptr_err(kbuf) as isize; }
    let err = os_write_file((*state).fd, kbuf, count);
    if err >= 0 { *ppos += err as i64; }
    kfree(kbuf);
    err as isize
}

pub unsafe extern "C" fn hostaudio_poll(_file: *mut file, _wait: *mut poll_table_struct) -> c_ulong { 0 }

pub unsafe extern "C" fn hostaudio_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let state = (*file).private_data as *mut hostaudio_state;
    let mut data: c_ulong = 0;
    if cmd == SNDCTL_DSP_SPEED || cmd == SNDCTL_DSP_STEREO || cmd == SNDCTL_DSP_GETBLKSIZE || cmd == SNDCTL_DSP_CHANNELS || cmd == SNDCTL_DSP_SUBDIVIDE || cmd == SNDCTL_DSP_SETFRAGMENT {
        if get_user(&mut data, arg as *const c_int) != 0 { return -(EFAULT as c_long); }
    }
    let err = os_ioctl_generic((*state).fd, cmd, &mut data as *mut c_ulong as c_ulong);
    if cmd == SNDCTL_DSP_SPEED || cmd == SNDCTL_DSP_STEREO || cmd == SNDCTL_DSP_GETBLKSIZE || cmd == SNDCTL_DSP_CHANNELS || cmd == SNDCTL_DSP_SUBDIVIDE || cmd == SNDCTL_DSP_SETFRAGMENT {
        if put_user(data, arg as *mut c_int) != 0 { return -(EFAULT as c_long); }
    }
    err as c_long
}

pub unsafe extern "C" fn hostaudio_open(_inode: *mut inode, file: *mut file) -> c_int {
    let state = kmalloc(core::mem::size_of::<hostaudio_state>(), 0) as *mut hostaudio_state;
    if state.is_null() { return -ENOMEM; }
    let r = if (*file).f_mode & FMODE_READ != 0 { 1 } else { 0 };
    let w = if (*file).f_mode & FMODE_WRITE != 0 { 1 } else { 0 };
    mutex_lock(&hostaudio_mutex);
    let ret = os_open_file(dsp, 0 | (r << 1) | w, 0);
    mutex_unlock(&hostaudio_mutex);
    if ret < 0 { kfree(state as *mut c_void); return ret; }
    (*state).fd = ret;
    (*file).private_data = state as *mut c_void;
    0
}

pub unsafe extern "C" fn hostaudio_release(_inode: *mut inode, file: *mut file) -> c_int {
    let state = (*file).private_data as *mut hostaudio_state;
    os_close_file((*state).fd); kfree(state as *mut c_void); 0
}

pub unsafe extern "C" fn hostmixer_ioctl_mixdev(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let state = (*file).private_data as *mut hostmixer_state;
    os_ioctl_generic((*state).fd, cmd, arg) as c_long
}

pub unsafe extern "C" fn hostmixer_open_mixdev(_inode: *mut inode, file: *mut file) -> c_int {
    let state = kmalloc(core::mem::size_of::<hostmixer_state>(), 0) as *mut hostmixer_state;
    if state.is_null() { return -ENOMEM; }
    let r = if (*file).f_mode & FMODE_READ != 0 { 1 } else { 0 };
    let w = if (*file).f_mode & FMODE_WRITE != 0 { 1 } else { 0 };
    mutex_lock(&hostaudio_mutex);
    let ret = os_open_file(mixer, 0 | (r << 1) | w, 0);
    mutex_unlock(&hostaudio_mutex);
    if ret < 0 { kfree(state as *mut c_void); return ret; }
    (*state).fd = ret;
    (*file).private_data = state as *mut c_void;
    0
}

pub unsafe extern "C" fn hostmixer_release(_inode: *mut inode, file: *mut file) -> c_int {
    let state = (*file).private_data as *mut hostmixer_state;
    os_close_file((*state).fd); kfree(state as *mut c_void); 0
}

#[no_mangle]
pub static hostaudio_fops: file_operations = file_operations { owner: core::ptr::null_mut(), read: Some(hostaudio_read), write: Some(hostaudio_write), poll: Some(hostaudio_poll), unlocked_ioctl: Some(hostaudio_ioctl), compat_ioctl: Some(compat_ptr_ioctl), mmap: core::ptr::null(), open: Some(hostaudio_open), release: Some(hostaudio_release) };

#[no_mangle]
pub static hostmixer_fops: file_operations = file_operations { owner: core::ptr::null_mut(), read: None, write: None, poll: None, unlocked_ioctl: Some(hostmixer_ioctl_mixdev), compat_ioctl: None, mmap: core::ptr::null(), open: Some(hostmixer_open_mixdev), release: Some(hostmixer_release) };

#[repr(C)]
static mut module_data: ModuleData = ModuleData { dev_audio: 0, dev_mixer: 0 };

#[repr(C)]
struct ModuleData { dev_audio: c_int, dev_mixer: c_int }

pub unsafe extern "C" fn hostaudio_init_module() -> c_int {
    module_data.dev_audio = register_sound_dsp(&hostaudio_fops, -1);
    if module_data.dev_audio < 0 { return -ENODEV; }
    module_data.dev_mixer = register_sound_mixer(&hostmixer_fops, -1);
    if module_data.dev_mixer < 0 { unregister_sound_dsp(module_data.dev_audio); return -ENODEV; }
    0
}

pub unsafe extern "C" fn hostaudio_cleanup_module() {
    unregister_sound_mixer(module_data.dev_mixer);
    unregister_sound_dsp(module_data.dev_audio);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
