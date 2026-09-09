// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Power Management and GPIO expander driver for MPC8349E-mITX-compatible MCU
 *
 * Copyright (c) 2008  MontaVista Software, Inc.
 *
 * Author: Anton Vorontsov <avorontsov@ru.mvista.com>
 */

// Kernel dependencies supplied by other translation units.

/*
 * I don't have specifications for the MCU firmware, I found this register
 * and bits positions by the trial&error method.
 */
const MCU_REG_CTRL: u8 = 0x20;
const MCU_CTRL_POFF: u8 = 0x40;
const MCU_CTRL_BTN: u8 = 0x80;

const MCU_NUM_GPIO: u32 = 2;

#[repr(C)]
struct mcu {
    lock: mutex,
    client: *mut i2c_client,
    gc: gpio_chip,
    reg_ctrl: u8,
}

static mut glob_mcu: *mut mcu = core::ptr::null_mut();

static mut shutdown_thread: *mut task_struct = core::ptr::null_mut();

unsafe extern "C" {
    fn i2c_smbus_read_byte_data(client: *mut i2c_client, command: u8) -> i32;
    fn i2c_smbus_write_byte_data(client: *mut i2c_client, command: u8, value: u8) -> i32;
    fn kthread_should_stop() -> bool;
    fn pr_err(fmt: *const u8, ...);
    fn pr_info(fmt: *const u8, ...);
    fn set_current_state(state: i32);
    fn schedule_timeout(timeout: i64) -> i64;
    fn ctrl_alt_del();
    fn sysfs_emit(buf: *mut u8, fmt: *const u8, ...) -> isize;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn gpiochip_get_data(gc: *mut gpio_chip) -> *mut mcu;
    fn kasprintf(flags: u32, fmt: *const u8, ...) -> *mut u8;
    fn dev_fwnode(dev: *mut device) -> *mut fwnode_handle;
    fn gpiochip_add_data(gc: *mut gpio_chip, data: *mut mcu) -> i32;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn gpiochip_remove(gc: *mut gpio_chip);
    fn kzalloc_obj<T>() -> *mut T;
    fn mutex_init(lock: *mut mutex);
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut mcu);
    fn device_create_file(dev: *mut device, attr: *mut device_attribute) -> i32;
    fn dev_info(dev: *mut device, fmt: *const u8, ...);
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn kthread_run(threadfn: unsafe extern "C" fn(*mut core::ffi::c_void) -> i32,
                   data: *mut core::ffi::c_void, name: *const u8, ...) -> *mut task_struct;
    fn kthread_stop(thread: *mut task_struct) -> i32;
    fn device_remove_file(dev: *mut device, attr: *mut device_attribute);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut mcu;
}

static mut pm_power_off: Option<unsafe extern "C" fn()> = None;

unsafe extern "C" fn shutdown_thread_fn(_data: *mut core::ffi::c_void) -> i32 {
    let mut ret: i32;
    let mcu = glob_mcu;

    while !kthread_should_stop() {
        ret = i2c_smbus_read_byte_data((*mcu).client, MCU_REG_CTRL);
        if ret < 0 {
            pr_err(b"MCU status reg read failed.\0".as_ptr());
        }
        (*mcu).reg_ctrl = ret as u8;

        if (*mcu).reg_ctrl & MCU_CTRL_BTN != 0 {
            i2c_smbus_write_byte_data((*mcu).client, MCU_REG_CTRL,
                                      (*mcu).reg_ctrl & !MCU_CTRL_BTN);
            ctrl_alt_del();
        }

        set_current_state(TASK_INTERRUPTIBLE);
        schedule_timeout(HZ);
    }

    0
}

unsafe extern "C" fn show_status(_d: *mut device,
                                  _attr: *mut device_attribute,
                                  buf: *mut u8) -> isize {
    let mcu = glob_mcu;
    let ret = i2c_smbus_read_byte_data((*mcu).client, MCU_REG_CTRL);
    if ret < 0 {
        return -ENODEV as isize;
    }
    (*mcu).reg_ctrl = ret as u8;
    sysfs_emit(buf, b"%02x\n\0".as_ptr(), ret)
}

static mut dev_attr_status: device_attribute = device_attribute::new(0o444, show_status, None);

unsafe extern "C" fn mcu_power_off() {
    let mcu = glob_mcu;
    pr_info(b"Sending power-off request to the MCU...\n\0".as_ptr());
    mutex_lock(&mut (*mcu).lock);
    i2c_smbus_write_byte_data((*mcu).client, MCU_REG_CTRL,
                              (*mcu).reg_ctrl | MCU_CTRL_POFF);
    mutex_unlock(&mut (*mcu).lock);
}

unsafe extern "C" fn mcu_gpio_set(gc: *mut gpio_chip, gpio: u32, val: i32) -> i32 {
    let mcu = gpiochip_get_data(gc);
    let bit = (1u8).wrapping_shl(4 + gpio);
    mutex_lock(&mut (*mcu).lock);
    if val != 0 {
        (*mcu).reg_ctrl &= !bit;
    } else {
        (*mcu).reg_ctrl |= bit;
    }
    let ret = i2c_smbus_write_byte_data((*mcu).client, MCU_REG_CTRL, (*mcu).reg_ctrl);
    mutex_unlock(&mut (*mcu).lock);
    ret
}

unsafe extern "C" fn mcu_gpio_dir_out(gc: *mut gpio_chip, gpio: u32, val: i32) -> i32 {
    mcu_gpio_set(gc, gpio, val)
}

unsafe fn mcu_gpiochip_add(mcu: *mut mcu) -> i32 {
    let dev = &mut (*(*mcu).client).dev;
    let gc = &mut (*mcu).gc;
    gc.owner = THIS_MODULE;
    gc.label = kasprintf(GFP_KERNEL, b"%pfw\0".as_ptr(), dev_fwnode(dev));
    if gc.label.is_null() { return -ENOMEM; }
    gc.can_sleep = true;
    gc.ngpio = MCU_NUM_GPIO;
    gc.base = -1;
    gc.set = Some(mcu_gpio_set);
    gc.direction_output = Some(mcu_gpio_dir_out);
    gc.parent = dev;
    gpiochip_add_data(gc, mcu)
}

unsafe fn mcu_gpiochip_remove(mcu: *mut mcu) {
    kfree((*mcu).gc.label.cast());
    gpiochip_remove(&mut (*mcu).gc);
}

unsafe extern "C" fn mcu_probe(client: *mut i2c_client) -> i32 {
    let mcu = kzalloc_obj::<mcu>();
    if mcu.is_null() { return -ENOMEM; }
    mutex_init(&mut (*mcu).lock);
    (*mcu).client = client;
    i2c_set_clientdata(client, mcu);
    let ret = i2c_smbus_read_byte_data((*mcu).client, MCU_REG_CTRL);
    if ret < 0 { kfree(mcu.cast()); return ret; }
    (*mcu).reg_ctrl = ret as u8;
    let ret = mcu_gpiochip_add(mcu);
    if ret != 0 { kfree(mcu.cast()); return ret; }
    /* XXX: this is potentially racy, but there is no lock for pm_power_off */
    if pm_power_off.is_none() {
        glob_mcu = mcu;
        pm_power_off = Some(mcu_power_off);
        dev_info(&mut (*client).dev, b"will provide power-off service\n\0".as_ptr());
    }
    if device_create_file(&mut (*client).dev, &mut dev_attr_status) != 0 {
        dev_err(&mut (*client).dev, b"couldn't create device file for status\n\0".as_ptr());
    }
    shutdown_thread = kthread_run(shutdown_thread_fn, core::ptr::null_mut(), b"mcu-i2c-shdn\0".as_ptr());
    0
}

unsafe extern "C" fn mcu_remove(client: *mut i2c_client) {
    let mcu = i2c_get_clientdata(client);
    kthread_stop(shutdown_thread);
    device_remove_file(&mut (*client).dev, &mut dev_attr_status);
    if glob_mcu == mcu { pm_power_off = None; glob_mcu = core::ptr::null_mut(); }
    mcu_gpiochip_remove(mcu);
    kfree(mcu.cast());
}

#[repr(C)]
struct i2c_device_id { name: *const u8, driver_data: usize }
static mcu_ids: [i2c_device_id; 2] = [
    i2c_device_id { name: b"mcu-mpc8349emitx\0".as_ptr(), driver_data: 0 },
    i2c_device_id { name: core::ptr::null(), driver_data: 0 },
];

#[repr(C)]
struct of_device_id { compatible: *const u8 }
static mcu_of_match_table: [of_device_id; 2] = [
    of_device_id { compatible: b"fsl,mcu-mpc8349emitx\0".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

#[repr(C)]
struct i2c_driver {
    name: *const u8,
    of_match_table: *const of_device_id,
    probe: Option<unsafe extern "C" fn(*mut i2c_client) -> i32>,
    remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
    id_table: *const i2c_device_id,
}

static mcu_driver: i2c_driver = i2c_driver {
    name: b"mcu-mpc8349emitx\0".as_ptr(),
    of_match_table: mcu_of_match_table.as_ptr(),
    probe: Some(mcu_probe),
    remove: Some(mcu_remove),
    id_table: mcu_ids.as_ptr(),
};

// Equivalent of module_i2c_driver(mcu_driver).
const _: &i2c_driver = &mcu_driver;

// MODULE_DESCRIPTION("Power Management and GPIO expander driver for MPC8349E-mITX-compatible MCU");
// MODULE_AUTHOR("Anton Vorontsov <avorontsov@ru.mvista.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
