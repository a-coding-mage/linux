// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_void};

// Linux kernel declarations supplied by the surrounding build.
type SizeT = usize;
type SSizeT = isize;
type LoffT = i64;
type U8 = u8;
type U16 = u16;
type S32 = i32;
type CUser = c_char;

#[repr(C)]
pub struct File;
#[repr(C)]
pub struct Inode;
#[repr(C)]
pub struct Spinlock;

#[repr(C)]
pub struct FileOperations {
    pub read: Option<unsafe extern "C" fn(*mut File, *mut CUser, SizeT, *mut LoffT) -> SSizeT>,
    pub write: Option<unsafe extern "C" fn(*mut File, *const CUser, SizeT, *mut LoffT) -> SSizeT>,
    pub open: Option<unsafe extern "C" fn(*mut Inode, *mut File) -> c_int>,
    pub release: Option<unsafe extern "C" fn(*mut Inode, *mut File) -> c_int>,
}

#[repr(C)]
pub struct MiscDevice {
    pub minor: c_int,
    pub name: *const c_char,
    pub fops: *const FileOperations,
}

#[repr(C)]
pub struct SpeakupInfo {
    pub spinlock: Spinlock,
}

unsafe extern "C" {
    static mut synth: *mut c_void;
    static mut speakup_info: SpeakupInfo;
    static mut MISC_MAJOR: c_int;

    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
    fn spin_lock_irqsave(lock: *mut Spinlock, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut Spinlock, flags: usize);
    fn synth_write(buf: *const U8, bytes: usize);
    fn synth_utf8_get(buf: *const U8, len: usize, consumed: *mut usize, want: *mut usize) -> S32;
    fn synth_buffer_add(value: U16);
    fn synth_start();
    fn misc_register(device: *mut MiscDevice) -> c_int;
    fn misc_deregister(device: *mut MiscDevice);
    fn pr_warn(format: *const c_char, ...);
    fn pr_info(format: *const c_char, ...);
}

const ENODEV: SSizeT = 19;
const EFAULT: SSizeT = 14;
const EBUSY: SSizeT = 16;
const MISC_DYNAMIC_MINOR: c_int = 255;

static mut synth_registered: c_int = 0;
static mut synthu_registered: c_int = 0;
static mut dev_opened: c_int = 0;

/* Latin1 version */
unsafe extern "C" fn speakup_file_write(
    _fp: *mut File,
    buffer: *const CUser,
    nbytes: SizeT,
    _ppos: *mut LoffT,
) -> SSizeT {
    let mut count = nbytes;
    let mut ptr = buffer as *const U8;
    let mut buf = [0u8; 256];

    if synth.is_null() {
        return -ENODEV;
    }
    while count > 0 {
        let bytes = core::cmp::min(count, buf.len());
        if copy_from_user(buf.as_mut_ptr() as *mut c_void, ptr as *const c_void, bytes) != 0 {
            return -EFAULT;
        }
        count -= bytes;
        ptr = ptr.add(bytes);
        let mut flags = 0usize;
        spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags);
        synth_write(buf.as_ptr(), bytes);
        spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
    }
    nbytes as SSizeT
}

/* UTF-8 version */
unsafe extern "C" fn speakup_file_writeu(
    _fp: *mut File,
    buffer: *const CUser,
    nbytes: SizeT,
    _ppos: *mut LoffT,
) -> SSizeT {
    let mut count = nbytes;
    let mut ptr = buffer as *const U8;
    let mut buf = [0u8; 256];
    let mut ubuf = [0u16; 256];

    if synth.is_null() {
        return -ENODEV;
    }

    let mut want = 1usize;
    while count >= want {
        let mut bytes = core::cmp::min(count, buf.len());
        if copy_from_user(buf.as_mut_ptr() as *mut c_void, ptr as *const c_void, bytes) != 0 {
            return -EFAULT;
        }

        let (mut input, mut output) = (0usize, 0usize);
        while input < bytes {
            let mut consumed = 0usize;
            let value = synth_utf8_get(
                buf.as_ptr().add(input),
                bytes - input,
                &mut consumed,
                &mut want,
            );
            if value == -1 {
                if want > bytes - input {
                    bytes = input;
                }
                input = input.wrapping_add(consumed);
                continue;
            }
            if value < 0x10000 {
                ubuf[output] = value as U16;
                output += 1;
            }
            input = input.wrapping_add(consumed);
        }

        count -= bytes;
        ptr = ptr.add(bytes);

        if output != 0 {
            let mut flags = 0usize;
            spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags);
            for input in 0..output {
                synth_buffer_add(ubuf[input]);
            }
            synth_start();
            spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
        }
    }

    (nbytes - count) as SSizeT
}

unsafe extern "C" fn speakup_file_read(
    _fp: *mut File,
    _buf: *mut CUser,
    _nbytes: SizeT,
    _ppos: *mut LoffT,
) -> SSizeT {
    0
}

unsafe extern "C" fn speakup_file_open(_ip: *mut Inode, _fp: *mut File) -> c_int {
    if synth.is_null() {
        return -ENODEV as c_int;
    }
    if core::mem::replace(&mut dev_opened, 1) != 0 {
        return -EBUSY as c_int;
    }
    0
}

unsafe extern "C" fn speakup_file_release(_ip: *mut Inode, _fp: *mut File) -> c_int {
    dev_opened = 0;
    0
}

static synth_fops: FileOperations = FileOperations {
    read: Some(speakup_file_read),
    write: Some(speakup_file_write),
    open: Some(speakup_file_open),
    release: Some(speakup_file_release),
};

static synthu_fops: FileOperations = FileOperations {
    read: Some(speakup_file_read),
    write: Some(speakup_file_writeu),
    open: Some(speakup_file_open),
    release: Some(speakup_file_release),
};

static mut synth_device: MiscDevice = MiscDevice {
    minor: MISC_DYNAMIC_MINOR,
    name: b"synth\0".as_ptr() as *const c_char,
    fops: &synth_fops,
};

static mut synthu_device: MiscDevice = MiscDevice {
    minor: MISC_DYNAMIC_MINOR,
    name: b"synthu\0".as_ptr() as *const c_char,
    fops: &synthu_fops,
};

pub unsafe extern "C" fn speakup_register_devsynth() {
    if synth_registered == 0 {
        if misc_register(&mut synth_device) != 0 {
            pr_warn(b"Couldn't initialize miscdevice /dev/synth.\n\0".as_ptr() as *const c_char);
        } else {
            pr_info(b"initialized device: /dev/synth, node (MAJOR %d, MINOR %d)\n\0".as_ptr() as *const c_char, MISC_MAJOR, synth_device.minor);
            synth_registered = 1;
        }
    }
    if synthu_registered == 0 {
        if misc_register(&mut synthu_device) != 0 {
            pr_warn(b"Couldn't initialize miscdevice /dev/synthu.\n\0".as_ptr() as *const c_char);
        } else {
            pr_info(b"initialized device: /dev/synthu, node (MAJOR %d, MINOR %d)\n\0".as_ptr() as *const c_char, MISC_MAJOR, synthu_device.minor);
            synthu_registered = 1;
        }
    }
}

pub unsafe extern "C" fn speakup_unregister_devsynth() {
    if synth_registered != 0 {
        pr_info(b"speakup: unregistering synth device /dev/synth\n\0".as_ptr() as *const c_char);
        misc_deregister(&mut synth_device);
        synth_registered = 0;
    }
    if synthu_registered != 0 {
        pr_info(b"speakup: unregistering synth device /dev/synthu\n\0".as_ptr() as *const c_char);
        misc_deregister(&mut synthu_device);
        synthu_registered = 0;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
