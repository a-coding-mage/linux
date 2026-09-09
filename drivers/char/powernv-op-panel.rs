// SPDX-License-Identifier: GPL-2.0-only
/*
 * OPAL Operator Panel Display Driver
 *
 * Copyright 2016, Suraj Jitindar Singh, IBM Corporation.
 */

// Dependency declarations supplied by the surrounding kernel environment.

static mut OPPANEL_MUTEX: Mutex = DEFINE_MUTEX!();

static mut NUM_LINES: u32 = 0;
static mut OPPANEL_SIZE: u32 = 0;
static mut OPPANEL_LINES: *mut oppanel_line_t = core::ptr::null_mut();
static mut OPPANEL_DATA: *mut c_char = core::ptr::null_mut();

unsafe extern "C" fn oppanel_llseek(
    filp: *mut file,
    offset: loff_t,
    whence: c_int,
) -> loff_t {
    fixed_size_llseek(filp, offset, whence, OPPANEL_SIZE as loff_t)
}

unsafe extern "C" fn oppanel_read(
    filp: *mut file,
    userbuf: *mut c_char,
    len: size_t,
    f_pos: *mut loff_t,
) -> ssize_t {
    simple_read_from_buffer(
        userbuf,
        len,
        f_pos,
        OPPANEL_DATA as *const c_void,
        OPPANEL_SIZE as size_t,
    )
}

unsafe fn __op_panel_update_display() -> c_int {
    let mut msg: opal_msg = core::mem::zeroed();
    let mut rc: c_int;
    let token = opal_async_get_token_interruptible();
    if token < 0 {
        if token != -ERESTARTSYS {
            pr_debug!("Couldn't get OPAL async token [token={}]\n", token);
        }
        return token;
    }

    rc = opal_write_oppanel_async(token, OPPANEL_LINES, NUM_LINES);
    match rc {
        OPAL_ASYNC_COMPLETION => {
            rc = opal_async_wait_response(token, &mut msg);
            if rc != 0 {
                pr_debug!("Failed to wait for async response [rc={}]\n", rc);
            } else {
                rc = opal_get_async_rc(msg);
                if rc != OPAL_SUCCESS {
                    pr_debug!("OPAL async call returned failed [rc={}]\n", rc);
                }
            }
        }
        OPAL_SUCCESS => {}
        _ => {
            pr_debug!("OPAL write op-panel call failed [rc={}]\n", rc);
        }
    }

    opal_async_release_token(token);
    rc
}

unsafe extern "C" fn oppanel_write(
    filp: *mut file,
    userbuf: *const c_char,
    len: size_t,
    f_pos: *mut loff_t,
) -> ssize_t {
    let ret: ssize_t;
    let rc: c_int;

    if *f_pos == 0 {
        memset(OPPANEL_DATA as *mut c_void, b' ' as c_int, OPPANEL_SIZE as size_t);
    } else if *f_pos >= OPPANEL_SIZE as loff_t {
        return -EFBIG as ssize_t;
    }

    ret = simple_write_to_buffer(
        OPPANEL_DATA as *mut c_void,
        OPPANEL_SIZE as size_t,
        f_pos,
        userbuf,
        len,
    );
    if ret > 0 {
        rc = __op_panel_update_display();
        if rc != OPAL_SUCCESS {
            pr_err_ratelimited!("OPAL call failed to write to op panel display [rc={}]\n", rc);
            return -EIO as ssize_t;
        }
    }
    ret
}

unsafe extern "C" fn oppanel_open(inode: *mut inode, filp: *mut file) -> c_int {
    if !mutex_trylock(&mut OPPANEL_MUTEX) {
        pr_debug!("Device Busy\n");
        return -EBUSY;
    }
    0
}

unsafe extern "C" fn oppanel_release(inode: *mut inode, filp: *mut file) -> c_int {
    mutex_unlock(&mut OPPANEL_MUTEX);
    0
}

static OPPANEL_FOPS: file_operations = file_operations {
    owner: THIS_MODULE,
    llseek: Some(oppanel_llseek),
    read: Some(oppanel_read),
    write: Some(oppanel_write),
    open: Some(oppanel_open),
    release: Some(oppanel_release),
};

static mut OPPANEL_DEV: miscdevice = miscdevice {
    minor: MISC_DYNAMIC_MINOR,
    name: b"op_panel\0".as_ptr() as *const c_char,
    fops: &OPPANEL_FOPS,
};

unsafe extern "C" fn oppanel_probe(pdev: *mut platform_device) -> c_int {
    let np = (*(*pdev).dev.of_node);
    let mut line_len: u32 = 0;
    let mut rc: c_int;

    rc = of_property_read_u32(np, b"#length\0".as_ptr() as *const c_char, &mut line_len);
    if rc != 0 {
        pr_err_ratelimited!("Operator panel length property not found\n");
        return rc;
    }
    rc = of_property_read_u32(np, b"#lines\0".as_ptr() as *const c_char, &mut NUM_LINES);
    if rc != 0 {
        pr_err_ratelimited!("Operator panel lines property not found\n");
        return rc;
    }
    OPPANEL_SIZE = line_len.wrapping_mul(NUM_LINES);

    pr_devel!("Operator panel of size {} found with {} lines of length {}\n", OPPANEL_SIZE, NUM_LINES, line_len);

    OPPANEL_DATA = kcalloc(OPPANEL_SIZE as size_t, core::mem::size_of::<c_char>(), GFP_KERNEL) as *mut c_char;
    if OPPANEL_DATA.is_null() {
        return -ENOMEM;
    }

    OPPANEL_LINES = kzalloc_objs!(oppanel_line_t, NUM_LINES) as *mut oppanel_line_t;
    if OPPANEL_LINES.is_null() {
        rc = -ENOMEM;
        goto_free_oppanel_data();
        return rc;
    }

    memset(OPPANEL_DATA as *mut c_void, b' ' as c_int, OPPANEL_SIZE as size_t);
    for i in 0..NUM_LINES {
        (*OPPANEL_LINES.add(i as usize)).line_len = cpu_to_be64(line_len as u64);
        (*OPPANEL_LINES.add(i as usize)).line = cpu_to_be64(__pa(OPPANEL_DATA.add((i * line_len) as usize)) as u64);
    }

    rc = misc_register(&mut OPPANEL_DEV);
    if rc != 0 {
        pr_err_ratelimited!("Failed to register as misc device\n");
        kfree(OPPANEL_LINES as *mut c_void);
        kfree(OPPANEL_DATA as *mut c_void);
        return rc;
    }
    0
}

unsafe fn goto_free_oppanel_data() {
    kfree(OPPANEL_DATA as *mut c_void);
}

unsafe extern "C" fn oppanel_remove(pdev: *mut platform_device) {
    misc_deregister(&mut OPPANEL_DEV);
    kfree(OPPANEL_LINES as *mut c_void);
    kfree(OPPANEL_DATA as *mut c_void);
}

static OPPANEL_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: b"ibm,opal-oppanel\0".as_ptr() as *const c_char },
    of_device_id { compatible: core::ptr::null() },
];

static mut OPPANEL_DRIVER: platform_driver = platform_driver {
    driver: device_driver {
        name: b"powernv-op-panel\0".as_ptr() as *const c_char,
        of_match_table: OPPANEL_MATCH.as_ptr(),
    },
    probe: Some(oppanel_probe),
    remove: Some(oppanel_remove),
};

module_platform_driver!(OPPANEL_DRIVER);

MODULE_DEVICE_TABLE!(of, OPPANEL_MATCH);
MODULE_LICENSE!("GPL v2");
MODULE_DESCRIPTION!("PowerNV Operator Panel LCD Display Driver");
MODULE_AUTHOR!("Suraj Jitindar Singh <sjitindarsingh@gmail.com>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
