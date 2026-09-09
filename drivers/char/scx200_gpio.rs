// SPDX-License-Identifier: GPL-2.0-only
/* linux/drivers/char/scx200_gpio.c

   National Semiconductor SCx200 GPIO driver.  Allows a user space
   process to play with the GPIO pins.

   Copyright (c) 2001,2002 Christer Weinigel <wingel@nano-system.com> */

// Linux kernel dependencies supplied by the surrounding translation unit.

const DRVNAME: &str = "scx200_gpio";

static mut PDEV: *mut platform_device = core::ptr::null_mut();

// MODULE_AUTHOR("Christer Weinigel <wingel@nano-system.com>");
// MODULE_DESCRIPTION("NatSemi/AMD SCx200 GPIO Pin Driver");
// MODULE_LICENSE("GPL");

static mut major: i32 = 0; // default to dynamic major
// module_param(major, int, 0);
// MODULE_PARM_DESC(major, "Major device number");

const MAX_PINS: u32 = 32; // 64 later, when known ok

#[repr(C)]
pub struct nsc_gpio_ops {
    pub owner: *mut core::ffi::c_void,
    pub gpio_config: Option<unsafe extern "C" fn()>,
    pub gpio_dump: Option<unsafe extern "C" fn()>,
    pub gpio_get: Option<unsafe extern "C" fn()>,
    pub gpio_set: Option<unsafe extern "C" fn()>,
    pub gpio_change: Option<unsafe extern "C" fn()>,
    pub gpio_current: Option<unsafe extern "C" fn()>,
    pub dev: *mut device,
}

#[no_mangle]
pub static mut scx200_gpio_ops: nsc_gpio_ops = nsc_gpio_ops {
    owner: unsafe { THIS_MODULE },
    gpio_config: Some(scx200_gpio_configure),
    gpio_dump: Some(nsc_gpio_dump),
    gpio_get: Some(scx200_gpio_get),
    gpio_set: Some(scx200_gpio_set),
    gpio_change: Some(scx200_gpio_change),
    gpio_current: Some(scx200_gpio_current),
    dev: core::ptr::null_mut(),
};

// EXPORT_SYMBOL_GPL(scx200_gpio_ops);

unsafe extern "C" fn scx200_gpio_open(inode: *mut inode, file: *mut file) -> i32 {
    let m: u32 = iminor(inode);
    (*file).private_data = &raw mut scx200_gpio_ops as *mut core::ffi::c_void;

    if m >= MAX_PINS {
        return -EINVAL;
    }
    nonseekable_open(inode, file)
}

unsafe extern "C" fn scx200_gpio_release(_inode: *mut inode, _file: *mut file) -> i32 {
    0
}

static scx200_gpio_fileops: file_operations = file_operations {
    owner: unsafe { THIS_MODULE },
    write: Some(nsc_gpio_write),
    read: Some(nsc_gpio_read),
    open: Some(scx200_gpio_open),
    release: Some(scx200_gpio_release),
};

static mut scx200_gpio_cdev: cdev = cdev::ZERO; // use 1 cdev for all pins

unsafe extern "C" fn scx200_gpio_init() -> i32 {
    let mut rc: i32;
    let mut devid: dev_t = 0;

    if !scx200_gpio_present() {
        printk!(KERN_ERR, "{}: no SCx200 gpio present\n", DRVNAME);
        return -ENODEV;
    }

    // support dev_dbg() with pdev->dev
    PDEV = platform_device_alloc(DRVNAME, 0);
    if PDEV.is_null() {
        return -ENOMEM;
    }

    rc = platform_device_add(PDEV);
    if rc != 0 {
        goto undo_malloc;
    }

    // nsc_gpio uses dev_dbg(), so needs this
    scx200_gpio_ops.dev = &mut (*PDEV).dev;

    if major != 0 {
        devid = MKDEV(major, 0);
        rc = register_chrdev_region(devid, MAX_PINS, "scx200_gpio");
    } else {
        rc = alloc_chrdev_region(&mut devid, 0, MAX_PINS, "scx200_gpio");
        major = MAJOR(devid);
    }
    if rc < 0 {
        dev_err!(&(*PDEV).dev, "SCx200 chrdev_region err: %d\n", rc);
        goto undo_platform_device_add;
    }

    cdev_init(&mut scx200_gpio_cdev, &scx200_gpio_fileops);
    cdev_add(&mut scx200_gpio_cdev, devid, MAX_PINS);

    return 0; // succeed

undo_platform_device_add:
    platform_device_del(PDEV);
undo_malloc:
    platform_device_put(PDEV);
    rc
}

unsafe extern "C" fn scx200_gpio_cleanup() {
    cdev_del(&mut scx200_gpio_cdev);
    // cdev_put(&scx200_gpio_cdev);

    unregister_chrdev_region(MKDEV(major, 0), MAX_PINS);
    platform_device_unregister(PDEV);
}

// module_init(scx200_gpio_init);
// module_exit(scx200_gpio_cleanup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
