/* SPDX-License-Identifier: GPL-2.0-only */
/* The industrial I/O core, trigger handling functions
 *
 * Copyright (c) 2008 Jonathan Cameron
 */

// C dependencies: linux/irq.h, linux/module.h, linux/atomic.h

/* CONFIG_IIO_TRIGGER conditional: declarations are present when enabled. */

#[repr(C)]
pub struct iio_subirq {
    pub enabled: bool,
}

pub struct iio_dev;
pub struct iio_trigger;

/**
 * struct iio_trigger_ops - operations structure for an iio_trigger.
 * @set_trigger_state: switch on/off the trigger on demand
 * @reenable: function to reenable the trigger when the use count is zero (may be NULL)
 * @validate_device: function to validate the device when the current trigger gets changed.
 *
 * This is typically static const within a driver and shared by instances of a given device.
 */
#[repr(C)]
pub struct iio_trigger_ops {
    pub set_trigger_state: Option<unsafe extern "C" fn(trig: *mut iio_trigger, state: bool) -> i32>,
    pub reenable: Option<unsafe extern "C" fn(trig: *mut iio_trigger)>,
    pub validate_device: Option<unsafe extern "C" fn(trig: *mut iio_trigger, indio_dev: *mut iio_dev) -> i32>,
}

/** struct iio_trigger - industrial I/O trigger device */
#[repr(C)]
pub struct iio_trigger {
    pub ops: *const iio_trigger_ops,
    pub owner: *mut module,
    pub id: i32,
    pub name: *const core::ffi::c_char,
    pub dev: device,
    pub list: list_head,
    pub alloc_list: list_head,
    pub use_count: atomic_t,
    pub subirq_chip: irq_chip,
    pub subirq_base: i32,
    pub subirqs: [iio_subirq; CONFIG_IIO_CONSUMERS_PER_TRIGGER],
    pub pool: [core::ffi::c_ulong; BITS_TO_LONGS(CONFIG_IIO_CONSUMERS_PER_TRIGGER)],
    pub pool_lock: mutex,
    pub attached_own_device: bool,
    pub reenable_work: work_struct,
}

pub unsafe fn to_iio_trigger(d: *mut device) -> *mut iio_trigger {
    container_of!(d, iio_trigger, dev)
}

pub unsafe fn iio_trigger_put(trig: *mut iio_trigger) {
    module_put((*trig).owner);
    put_device(&mut (*trig).dev);
}

pub unsafe fn iio_trigger_get(trig: *mut iio_trigger) -> *mut iio_trigger {
    get_device(&mut (*trig).dev);
    WARN_ONCE!(list_empty(&(*trig).list), "Getting non-registered iio trigger %s is prohibited\n", (*trig).name);
    __module_get((*trig).owner);
    trig
}

/** iio_trigger_set_drvdata() - Set trigger driver data */
pub unsafe fn iio_trigger_set_drvdata(trig: *mut iio_trigger, data: *mut core::ffi::c_void) {
    dev_set_drvdata(&mut (*trig).dev, data);
}

/** iio_trigger_get_drvdata() - Get trigger driver data */
pub unsafe fn iio_trigger_get_drvdata(trig: *mut iio_trigger) -> *mut core::ffi::c_void {
    dev_get_drvdata(&mut (*trig).dev)
}

pub unsafe extern "C" fn iio_trigger_register(trig_info: *mut iio_trigger) -> i32;
pub unsafe extern "C" fn devm_iio_trigger_register(dev: *mut device, trig_info: *mut iio_trigger) -> i32;
pub unsafe extern "C" fn iio_trigger_unregister(trig_info: *mut iio_trigger);
pub unsafe extern "C" fn iio_trigger_set_immutable(indio_dev: *mut iio_dev, trig: *mut iio_trigger) -> i32;
pub unsafe extern "C" fn iio_trigger_poll(trig: *mut iio_trigger);
pub unsafe extern "C" fn iio_trigger_poll_nested(trig: *mut iio_trigger);
pub unsafe extern "C" fn iio_trigger_generic_data_rdy_poll(irq: i32, private: *mut core::ffi::c_void) -> irqreturn_t;

// #define iio_trigger_alloc(parent, fmt, ...) __iio_trigger_alloc((parent), THIS_MODULE, (fmt), ##__VA_ARGS__)
pub unsafe extern "C" fn __iio_trigger_alloc(parent: *mut device, this_mod: *mut module, fmt: *const core::ffi::c_char, ...) -> *mut iio_trigger;
pub unsafe extern "C" fn iio_trigger_free(trig: *mut iio_trigger);
pub unsafe extern "C" fn iio_trigger_using_own(indio_dev: *mut iio_dev) -> bool;
pub unsafe extern "C" fn iio_validate_own_trigger(idev: *mut iio_dev, trig: *mut iio_trigger) -> i32;
pub unsafe extern "C" fn iio_trigger_validate_own_device(trig: *mut iio_trigger, indio_dev: *mut iio_dev) -> i32;

/* CONFIG_IIO_TRIGGER disabled: only opaque trigger and operations declarations remain. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
