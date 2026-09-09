// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014  Google, Inc.
 */

// Dependencies supplied by the surrounding kernel translation are referenced
// here rather than reimplemented.

static mut PMSG_LOCK: Mutex = Mutex::new();

unsafe fn write_pmsg(
    file: *mut File,
    buf: *const core::ffi::c_char,
    count: usize,
    ppos: *mut LoffT,
) -> Isize {
    let mut record: PstoreRecord = core::mem::zeroed();
    let mut ret: i32;

    if count == 0 {
        return 0;
    }

    pstore_record_init(&mut record, psinfo);
    record.type_ = PSTORE_TYPE_PMSG;
    record.size = count;

    /* check outside lock, page in any data. write_user also checks */
    if !access_ok(buf, count) {
        return -EFAULT as Isize;
    }

    mutex_lock(&mut PMSG_LOCK);
    ret = ((*psinfo).write_user.unwrap())(&mut record, buf);
    mutex_unlock(&mut PMSG_LOCK);
    if ret != 0 { ret as Isize } else { count as Isize }
}

static PMSG_FOPS: FileOperations = FileOperations {
    owner: THIS_MODULE,
    llseek: Some(noop_llseek),
    write: Some(write_pmsg),
};

static mut PMSG_CLASS: *mut Class = core::ptr::null_mut();
static mut PMSG_MAJOR: i32 = 0;
const PMSG_NAME: &[u8] = b"pmsg\0";

// pr_fmt(fmt) expands to "pmsg: " fmt in the original source.

unsafe fn pmsg_devnode(dev: *const Device, mode: *mut UModeT) -> *mut core::ffi::c_char {
    if !mode.is_null() {
        *mode = 0o220;
    }
    core::ptr::null_mut()
}

pub unsafe fn pstore_register_pmsg() {
    let mut pmsg_device: *mut Device;

    PMSG_MAJOR = register_chrdev(0, PMSG_NAME.as_ptr(), &PMSG_FOPS);
    if PMSG_MAJOR < 0 {
        pr_err(b"register_chrdev failed\n\0".as_ptr());
        return;
    }

    PMSG_CLASS = class_create(PMSG_NAME.as_ptr());
    if is_err(PMSG_CLASS as *const core::ffi::c_void) {
        pr_err(b"device class file already in use\n\0".as_ptr());
        unregister_chrdev(PMSG_MAJOR, PMSG_NAME.as_ptr());
        return;
    }
    (*PMSG_CLASS).devnode = Some(pmsg_devnode);

    pmsg_device = device_create(
        PMSG_CLASS,
        core::ptr::null_mut(),
        mkdev(PMSG_MAJOR, 0),
        core::ptr::null_mut(),
        PMSG_NAME.as_ptr(),
        0,
    );
    if is_err(pmsg_device as *const core::ffi::c_void) {
        pr_err(b"failed to create device\n\0".as_ptr());
        class_destroy(PMSG_CLASS);
        unregister_chrdev(PMSG_MAJOR, PMSG_NAME.as_ptr());
        return;
    }
}

pub unsafe fn pstore_unregister_pmsg() {
    device_destroy(PMSG_CLASS, mkdev(PMSG_MAJOR, 0));
    class_destroy(PMSG_CLASS);
    unregister_chrdev(PMSG_MAJOR, PMSG_NAME.as_ptr());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
