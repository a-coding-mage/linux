// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) IBM Corporation 2023 */

// Translated from the Linux kernel implementation. The included kernel and
// driver definitions are supplied by the surrounding build environment.

#[repr(C)]
pub struct i2cr_scom {
    pub dev: device,
    pub cdev: cdev,
    pub i2cr: *mut fsi_master_i2cr,
}

unsafe fn i2cr_scom_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t {
    match whence {
        SEEK_CUR => {}
        SEEK_SET => unsafe {
            (*file).f_pos = offset;
        },
        _ => return -EINVAL,
    }

    offset
}

unsafe fn i2cr_scom_read(
    filep: *mut file,
    buf: *mut core::ffi::c_void,
    len: usize,
    offset: *mut loff_t,
) -> isize {
    let scom = unsafe { (*filep).private_data as *mut i2cr_scom };
    let mut data: u64 = 0;
    let mut ret: c_int;

    if len != core::mem::size_of::<u64>() {
        return -EINVAL as isize;
    }

    ret = unsafe { fsi_master_i2cr_read((*scom).i2cr, (*offset) as u32, &mut data) };
    if ret != 0 {
        return ret as isize;
    }

    ret = unsafe { copy_to_user(buf, &data as *const u64 as *const core::ffi::c_void, len) };
    if ret != 0 {
        return ret as isize;
    }

    len as isize
}

unsafe fn i2cr_scom_write(
    filep: *mut file,
    buf: *const core::ffi::c_void,
    len: usize,
    offset: *mut loff_t,
) -> isize {
    let scom = unsafe { (*filep).private_data as *mut i2cr_scom };
    let mut data: u64 = 0;
    let mut ret: c_int;

    if len != core::mem::size_of::<u64>() {
        return -EINVAL as isize;
    }

    ret = unsafe { copy_from_user(&mut data as *mut u64 as *mut core::ffi::c_void, buf, len) };
    if ret != 0 {
        return ret as isize;
    }

    ret = unsafe { fsi_master_i2cr_write((*scom).i2cr, (*offset) as u32, data) };
    if ret != 0 {
        return ret as isize;
    }

    len as isize
}

static i2cr_scom_fops: file_operations = file_operations {
    owner: THIS_MODULE,
    open: Some(simple_open),
    llseek: Some(i2cr_scom_llseek),
    read: Some(i2cr_scom_read),
    write: Some(i2cr_scom_write),
};

unsafe fn i2cr_scom_probe(fsi_dev: *mut fsi_device) -> c_int {
    let dev = unsafe { &mut (*fsi_dev).dev as *mut device };
    let mut scom: *mut i2cr_scom;
    let mut didx: c_int = 0;
    let mut ret: c_int;

    if unsafe { !is_fsi_master_i2cr((*(*fsi_dev).slave).master) } {
        return -ENODEV;
    }

    scom = unsafe { devm_kzalloc(dev, core::mem::size_of::<i2cr_scom>(), GFP_KERNEL) as *mut i2cr_scom };
    if scom.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*scom).i2cr = to_fsi_master_i2cr((*(*fsi_dev).slave).master);
        dev_set_drvdata(dev, scom as *mut core::ffi::c_void);
        (*scom).dev.type_ = &fsi_cdev_type;
        (*scom).dev.parent = dev;
        device_initialize(&mut (*scom).dev);
    }

    ret = unsafe { fsi_get_new_minor(fsi_dev, fsi_dev_scom, &mut (*scom).dev.devt, &mut didx) };
    if ret != 0 {
        return ret;
    }

    unsafe {
        dev_set_name(&mut (*scom).dev, b"scom%d\0".as_ptr() as *const c_char, didx);
        cdev_init(&mut (*scom).cdev, &i2cr_scom_fops);
        ret = cdev_device_add(&mut (*scom).cdev, &mut (*scom).dev);
        if ret != 0 {
            fsi_free_minor((*scom).dev.devt);
        }
    }

    ret
}

unsafe fn i2cr_scom_remove(fsi_dev: *mut fsi_device) {
    let scom = unsafe { dev_get_drvdata(&mut (*fsi_dev).dev) as *mut i2cr_scom };

    unsafe {
        cdev_device_del(&mut (*scom).cdev, &mut (*scom).dev);
        fsi_free_minor((*scom).dev.devt);
    }
}

static i2cr_scom_of_ids: [of_device_id; 2] = [
    of_device_id { compatible: b"ibm,i2cr-scom\0".as_ptr() as *const c_char },
    of_device_id { compatible: core::ptr::null() },
];

// MODULE_DEVICE_TABLE(of, i2cr_scom_of_ids);

static i2cr_scom_ids: [fsi_device_id; 2] = [
    fsi_device_id { id: 0x5, version: FSI_VERSION_ANY },
    fsi_device_id { id: 0, version: 0 },
];

static mut i2cr_scom_driver: fsi_driver = fsi_driver {
    probe: Some(i2cr_scom_probe),
    remove: Some(i2cr_scom_remove),
    id_table: i2cr_scom_ids.as_ptr(),
    drv: driver {
        name: b"i2cr_scom\0".as_ptr() as *const c_char,
        of_match_table: i2cr_scom_of_ids.as_ptr(),
    },
};

// module_fsi_driver(i2cr_scom_driver);

// MODULE_AUTHOR("Eddie James <eajames@linux.ibm.com>");
// MODULE_DESCRIPTION("IBM I2C Responder SCOM driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
