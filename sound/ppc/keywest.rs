// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * common keywest i2c layer
 *
 * Copyright (c) by Takashi Iwai <tiwai@suse.de>
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem;
use core::ptr;

/*
 * C dependencies:
 *   <linux/init.h>, <linux/i2c.h>, <linux/delay.h>, <linux/module.h>,
 *   <sound/core.h>, "pmac.h"
 */

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENXIO: c_int = 6;
const EBUSY: c_int = 16;
const EPROBE_DEFER: c_int = 517;

/* from <linux/i2c.h> */
const I2C_NAME_SIZE: usize = 20;

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct device {
    pub driver: *mut device_driver,
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct i2c_adapter {
    pub name: [c_char; I2C_NAME_SIZE],
}

#[repr(C)]
pub struct i2c_board_info {
    pub type_: [c_char; I2C_NAME_SIZE],
    pub addr: c_uint,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; I2C_NAME_SIZE],
    pub driver_data: usize,
}

#[repr(C)]
pub struct i2c_driver_inner {
    pub name: *const c_char,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: i2c_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
    pub id_table: *const i2c_device_id,
}

#[repr(C)]
pub struct pmac_keywest {
    pub client: *mut i2c_client,
    pub addr: c_uint,
    pub init_client: Option<unsafe extern "C" fn(*mut pmac_keywest) -> c_int>,
}

unsafe extern "C" {
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn i2c_new_client_device(
        adapter: *mut i2c_adapter,
        info: *const i2c_board_info,
    ) -> *mut i2c_client;
    fn i2c_unregister_device(client: *mut i2c_client);
    fn i2c_del_driver(driver: *mut i2c_driver);
    fn i2c_get_adapter(nr: c_int) -> *mut i2c_adapter;
    fn i2c_put_adapter(adap: *mut i2c_adapter);
    fn i2c_add_driver(driver: *mut i2c_driver) -> c_int;

    fn strncmp(cs: *const c_char, ct: *const c_char, count: usize) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> isize;

    /* Kernel macros/functions supplied by included headers. */
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

static mut KEYWEST_CTX: *mut pmac_keywest = ptr::null_mut();
static mut KEYWEST_PROBED: bool = false;

unsafe extern "C" fn keywest_probe(client: *mut i2c_client) -> c_int {
    KEYWEST_PROBED = true;
    /* If instantiated via i2c-powermac, we still need to set the client */
    if (*KEYWEST_CTX).client.is_null() {
        (*KEYWEST_CTX).client = client;
    }
    i2c_set_clientdata(client, KEYWEST_CTX as *mut c_void);
    0
}

/*
 * This is kind of a hack, best would be to turn powermac to fixed i2c
 * bus numbers and declare the sound device as part of platform
 * initialization
 */
unsafe extern "C" fn keywest_attach_adapter(adapter: *mut i2c_adapter) -> c_int {
    let mut info: i2c_board_info;
    let mut client: *mut i2c_client;

    if KEYWEST_CTX.is_null() {
        return -EINVAL;
    }

    if strncmp(
        (*adapter).name.as_ptr(),
        c"mac-io".as_ptr(),
        6,
    ) != 0
    {
        return -EINVAL; /* ignored */
    }

    info = mem::zeroed();
    strscpy(info.type_.as_mut_ptr(), c"keywest".as_ptr(), I2C_NAME_SIZE);
    info.addr = (*KEYWEST_CTX).addr;
    client = i2c_new_client_device(adapter, &info);
    if IS_ERR(client as *const c_void) {
        return PTR_ERR(client as *const c_void) as c_int;
    }
    (*KEYWEST_CTX).client = client;

    /*
     * We know the driver is already loaded, so the device should be
     * already bound. If not it means binding failed, and then there
     * is no point in keeping the device instantiated.
     */
    if (*(*KEYWEST_CTX).client).dev.driver.is_null() {
        i2c_unregister_device((*KEYWEST_CTX).client);
        (*KEYWEST_CTX).client = ptr::null_mut();
        return -ENODEV;
    }

    0
}

unsafe extern "C" fn keywest_remove(client: *mut i2c_client) {
    if KEYWEST_CTX.is_null() {
        return;
    }
    if client == (*KEYWEST_CTX).client {
        (*KEYWEST_CTX).client = ptr::null_mut();
    }
}

static KEYWEST_I2C_ID: [i2c_device_id; 3] = [
    i2c_device_id {
        name: [
            b'M' as c_char,
            b'A' as c_char,
            b'C' as c_char,
            b',' as c_char,
            b't' as c_char,
            b'a' as c_char,
            b's' as c_char,
            b'3' as c_char,
            b'0' as c_char,
            b'0' as c_char,
            b'4' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        driver_data: 0,
    }, /* instantiated by i2c-powermac */
    i2c_device_id {
        name: [
            b'k' as c_char,
            b'e' as c_char,
            b'y' as c_char,
            b'w' as c_char,
            b'e' as c_char,
            b's' as c_char,
            b't' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        driver_data: 0,
    }, /* instantiated by us if needed */
    i2c_device_id {
        name: [0; I2C_NAME_SIZE],
        driver_data: 0,
    },
];
/* MODULE_DEVICE_TABLE(i2c, keywest_i2c_id); */

static mut KEYWEST_DRIVER: i2c_driver = i2c_driver {
    driver: i2c_driver_inner {
        name: c"PMac Keywest Audio".as_ptr(),
    },
    probe: Some(keywest_probe),
    remove: Some(keywest_remove),
    id_table: KEYWEST_I2C_ID.as_ptr(),
};

/* exported */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_pmac_keywest_cleanup(i2c: *mut pmac_keywest) {
    if !KEYWEST_CTX.is_null() && KEYWEST_CTX == i2c {
        i2c_unregister_device((*KEYWEST_CTX).client);
        i2c_del_driver(&raw mut KEYWEST_DRIVER);
        KEYWEST_CTX = ptr::null_mut();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_pmac_tumbler_post_init() -> c_int {
    let mut err: c_int;

    if KEYWEST_CTX.is_null() || (*KEYWEST_CTX).client.is_null() {
        return -ENXIO;
    }

    err = ((*KEYWEST_CTX).init_client.unwrap())(KEYWEST_CTX);
    if err < 0 {
        dev_err(
            &raw mut (*(*KEYWEST_CTX).client).dev,
            c"tumbler: %i :cannot initialize the MCS\n".as_ptr(),
            err,
        );
        return err;
    }
    0
}

/* exported */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_pmac_keywest_init(i2c: *mut pmac_keywest) -> c_int {
    let mut adap: *mut i2c_adapter;
    let mut err: c_int;
    let mut i: c_int = 0;

    if !KEYWEST_CTX.is_null() {
        return -EBUSY;
    }

    adap = i2c_get_adapter(0);
    if adap.is_null() {
        return -EPROBE_DEFER;
    }

    KEYWEST_CTX = i2c;

    err = i2c_add_driver(&raw mut KEYWEST_DRIVER);
    if err != 0 {
        dev_err(
            &raw mut (*(*i2c).client).dev,
            c"cannot register keywest i2c driver\n".as_ptr(),
        );
        i2c_put_adapter(adap);
        return err;
    }

    /* There was already a device from i2c-powermac. Great, let's return */
    if KEYWEST_PROBED {
        return 0;
    }

    /* We assume Macs have consecutive I2C bus numbers starting at 0 */
    while !adap.is_null() {
        /* Scan for devices to be bound to */
        err = keywest_attach_adapter(adap);
        if err == 0 {
            return 0;
        }
        i2c_put_adapter(adap);
        i += 1;
        adap = i2c_get_adapter(i);
    }

    -ENODEV
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
