// SPDX-License-Identifier: GPL-2.0-only
/*
 * APEI Error Record Serialization Table debug support
 *
 * ERST is a way provided by APEI to save and retrieve hardware error
 * information to and from a persistent store. This file provide the
 * debugging/testing support for ERST kernel support and firmware
 * implementation.
 *
 * Copyright 2010 Intel Corp.
 *   Author: Huang Ying <ying.huang@intel.com>
 */

// C includes and kernel-provided declarations are supplied by the surrounding build.

const ERST_DBG_PFX: &str = "ERST DBG: ";
const ERST_DBG_RECORD_LEN_MAX: usize = 0x4000;

static mut erst_dbg_buf: *mut core::ffi::c_void = core::ptr::null_mut();
static mut erst_dbg_buf_len: usize = 0;

// Prevent erst_dbg_read/write from being invoked concurrently.
static mut erst_dbg_mutex: Mutex = Mutex::new();

unsafe fn erst_dbg_open(inode: *mut inode, file: *mut file) -> i32 {
    let mut rc: i32;
    let pos: *mut i32;

    if erst_disable {
        return -ENODEV;
    }

    pos = &mut (*file).private_data as *mut _ as *mut i32;

    rc = erst_get_record_id_begin(pos);
    if rc != 0 {
        return rc;
    }

    nonseekable_open(inode, file)
}

unsafe fn erst_dbg_release(_inode: *mut inode, _file: *mut file) -> i32 {
    erst_get_record_id_end();
    0
}

unsafe fn erst_dbg_ioctl(f: *mut file, cmd: u32, arg: usize) -> isize {
    let mut rc: i32;
    let mut record_id: u64 = 0;
    let mut record_count: u32;

    match cmd {
        APEI_ERST_CLEAR_RECORD => {
            if copy_from_user(
                &mut record_id as *mut u64 as *mut core::ffi::c_void,
                arg as *const core::ffi::c_void,
                core::mem::size_of::<u64>(),
            ) != 0 {
                return -EFAULT as isize;
            }
            erst_clear(record_id) as isize
        }
        APEI_ERST_GET_RECORD_COUNT => {
            rc = erst_get_record_count();
            if rc < 0 {
                return rc as isize;
            }
            record_count = rc as u32;
            rc = put_user(record_count, arg as *mut u32);
            if rc != 0 {
                return rc as isize;
            }
            0
        }
        _ => -ENOTTY as isize,
    }
}

unsafe fn erst_dbg_read(
    filp: *mut file,
    ubuf: *mut core::ffi::c_void,
    usize_: usize,
    off: *mut loff_t,
) -> isize {
    let mut rc: i32;
    let pos: *mut i32;
    let mut len: isize = 0;
    let mut id: u64 = 0;

    if *off != 0 {
        return -EINVAL as isize;
    }
    if mutex_lock_interruptible(&mut erst_dbg_mutex) != 0 {
        return -EINTR as isize;
    }

    pos = &mut (*filp).private_data as *mut _ as *mut i32;

    loop {
        rc = erst_get_record_id_next(pos, &mut id);
        if rc != 0 {
            break;
        }
        if id == APEI_ERST_INVALID_RECORD_ID {
            // If the persistent store is empty initially, erst_read returns -ENOENT;
            // retrying then reaches this point and returns zero for EOF.
            len = 0;
            break;
        }
        loop {
            len = erst_read_record(id, erst_dbg_buf, erst_dbg_buf_len, erst_dbg_buf_len, core::ptr::null_mut()) as isize;
            if len as i32 != -ENOENT {
                break;
            }
            break;
        }
        if len as i32 == -ENOENT {
            continue;
        }
        rc = len as i32;
        if rc < 0 {
            break;
        }
        if len > ERST_DBG_RECORD_LEN_MAX as isize {
            pr_warn!("{}Record (ID: 0x{:x}) length is too long: {}\n", ERST_DBG_PFX, id, len);
            rc = -EIO;
            break;
        }
        if len as usize > erst_dbg_buf_len {
            let p = kmalloc(len as usize, GFP_KERNEL);
            rc = -ENOMEM;
            if p.is_null() {
                break;
            }
            kfree(erst_dbg_buf);
            erst_dbg_buf = p;
            erst_dbg_buf_len = len as usize;
            continue;
        }
        rc = -EINVAL;
        if len as usize > usize_ {
            break;
        }
        rc = -EFAULT;
        if copy_to_user(ubuf, erst_dbg_buf, len as usize) != 0 {
            break;
        }
        rc = 0;
        break;
    }
    mutex_unlock(&mut erst_dbg_mutex);
    if rc != 0 { rc as isize } else { len }
}

unsafe fn erst_dbg_write(
    _filp: *mut file,
    ubuf: *const core::ffi::c_void,
    usize_: usize,
    _off: *mut loff_t,
) -> isize {
    let mut rc: i32;
    let rcd: *mut cper_record_header;

    if !capable(CAP_SYS_ADMIN) { return -EPERM as isize; }
    if usize_ > ERST_DBG_RECORD_LEN_MAX {
        pr_err!("{}Too long record to be written\n", ERST_DBG_PFX);
        return -EINVAL as isize;
    }
    if mutex_lock_interruptible(&mut erst_dbg_mutex) != 0 { return -EINTR as isize; }
    if usize_ > erst_dbg_buf_len {
        let p = kmalloc(usize_, GFP_KERNEL);
        rc = -ENOMEM;
        if p.is_null() { mutex_unlock(&mut erst_dbg_mutex); return rc as isize; }
        kfree(erst_dbg_buf);
        erst_dbg_buf = p;
        erst_dbg_buf_len = usize_;
    }
    if copy_from_user(erst_dbg_buf, ubuf, usize_) != 0 { rc = -EFAULT; goto out; }
    rcd = erst_dbg_buf as *mut cper_record_header;
    rc = -EINVAL;
    if (*rcd).record_length != usize_ { goto out; }
    rc = erst_write(erst_dbg_buf);
out:
    mutex_unlock(&mut erst_dbg_mutex);
    if rc < 0 { rc as isize } else { usize_ as isize }
}

// The file-operations and misc-device registrations are retained as external-kernel
// interface declarations; their field initialization mirrors the C source.
static erst_dbg_ops: file_operations = file_operations {
    owner: THIS_MODULE,
    open: Some(erst_dbg_open),
    release: Some(erst_dbg_release),
    read: Some(erst_dbg_read),
    write: Some(erst_dbg_write),
    unlocked_ioctl: Some(erst_dbg_ioctl),
};

static mut erst_dbg_dev: miscdevice = miscdevice {
    minor: MISC_DYNAMIC_MINOR,
    name: "erst_dbg",
    fops: &erst_dbg_ops,
};

unsafe fn erst_dbg_init() -> i32 {
    if erst_disable {
        pr_info!("{}ERST support is disabled.\n", ERST_DBG_PFX);
        return -ENODEV;
    }
    misc_register(&mut erst_dbg_dev)
}

unsafe fn erst_dbg_exit() {
    misc_deregister(&mut erst_dbg_dev);
    kfree(erst_dbg_buf);
}

// module_init(erst_dbg_init);
// module_exit(erst_dbg_exit);
// MODULE_AUTHOR("Huang Ying");
// MODULE_DESCRIPTION("APEI Error Record Serialization Table debug support");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
