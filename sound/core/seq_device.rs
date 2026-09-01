// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  ALSA sequencer device management
 *  Copyright (c) 1999 by Takashi Iwai <tiwai@suse.de>
 *
 *----------------------------------------------------------------
 *
 * This device handler separates the card driver module from sequencer
 * stuff (sequencer core, synth drivers, etc), so that user can avoid
 * to spend unnecessary resources e.g. if he needs only listening to
 * MP3s.
 *
 * The card (or lowlevel) driver creates a sequencer device entry
 * via snd_seq_device_new().  This is an entry pointer to communicate
 * with the sequencer device "driver", which is involved with the
 * actual part to communicate with the sequencer core.
 * Each sequencer device entry has an id string and the corresponding
 * driver with the same id is loaded when required.  For example,
 * lowlevel codes to access emu8000 chip on sbawe card are included in
 * emu8000-synth module.  To activate this module, the hardware
 * resources like i/o port are passed via snd_seq_device argument.
 */

/* C includes removed:
 * <linux/device.h>, <linux/init.h>, <linux/module.h>, <sound/core.h>,
 * <sound/info.h>, <sound/seq_device.h>, <sound/seq_kernel.h>,
 * <sound/initval.h>, <linux/kmod.h>, <linux/slab.h>, <linux/mutex.h>
 */

use core::ffi::{c_char, c_int, c_void};

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const SNDRV_DEV_SEQUENCER: c_int = 0;
const SNDRV_INFO_CONTENT_TEXT: c_int = 0;

#[repr(C)]
pub struct device {
    pub driver: *mut device_driver,
    pub parent: *mut device,
    pub bus: *const bus_type,
    pub release: Option<unsafe extern "C" fn(*mut device)>,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub bus: *const bus_type,
    pub owner: *mut module,
    pub probe: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut device)>,
}

#[repr(C)]
pub struct bus_type {
    pub name: *const c_char,
    pub match_: Option<unsafe extern "C" fn(*mut device, *const device_driver) -> c_int>,
    pub probe: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut device)>,
}

#[repr(C)]
pub struct snd_seq_device {
    pub dev: device,
    pub card: *mut snd_card,
    pub device: c_int,
    pub id: *const c_char,
    pub argsize: c_int,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_seq_device)>,
    pub args: [u8; 0],
}

#[repr(C)]
pub struct snd_seq_driver {
    pub driver: device_driver,
    pub id: *const c_char,
    pub argsize: c_int,
    pub probe: Option<unsafe extern "C" fn(*mut snd_seq_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_seq_device)>,
}

#[repr(C)]
pub struct snd_card {
    pub card_dev: device,
    pub number: c_int,
}

#[repr(C)]
pub struct snd_device {
    pub device_data: *mut c_void,
}

#[repr(C)]
pub struct snd_device_ops {
    pub dev_free: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
    pub dev_register: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
    pub dev_disconnect: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
}

#[repr(C)]
pub struct snd_info_entry {
    pub content: c_int,
    pub c: snd_info_entry_u,
}

#[repr(C)]
pub union snd_info_entry_u {
    pub text: snd_info_entry_text,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_info_entry_text {
    pub read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct atomic_t {
    pub counter: c_int,
}

extern "C" {
    static mut THIS_MODULE: *mut module;
    static mut snd_seq_root: *mut snd_info_entry;

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn bus_for_each_dev(
        bus: *const bus_type,
        start: *mut device,
        data: *mut c_void,
        fn_: Option<unsafe extern "C" fn(*mut device, *mut c_void) -> c_int>,
    ) -> c_int;
    fn request_module(fmt: *const c_char, ...) -> c_int;
    fn schedule_work(work: *mut work_struct) -> bool;
    fn flush_work(work: *mut work_struct) -> bool;
    fn cancel_work_sync(work: *mut work_struct) -> bool;
    fn atomic_inc_return(v: *mut atomic_t) -> c_int;
    fn atomic_dec(v: *mut atomic_t);
    fn atomic_inc(v: *mut atomic_t);
    fn device_add(dev: *mut device) -> c_int;
    fn device_del(dev: *mut device);
    fn device_initialize(dev: *mut device);
    fn dev_set_name(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn put_device(dev: *mut device);
    fn kfree(ptr: *const c_void);
    fn kzalloc_flex_snd_seq_device_args(argsize: c_int) -> *mut snd_seq_device;
    fn snd_device_new(
        card: *mut snd_card,
        type_: c_int,
        device_data: *mut c_void,
        ops: *const snd_device_ops,
    ) -> c_int;
    fn driver_register(drv: *mut device_driver) -> c_int;
    fn driver_unregister(drv: *mut device_driver);
    fn snd_info_create_module_entry(
        module: *mut module,
        name: *const c_char,
        parent: *mut snd_info_entry,
    ) -> *mut snd_info_entry;
    fn snd_info_register(entry: *mut snd_info_entry) -> c_int;
    fn snd_info_free_entry(entry: *mut snd_info_entry);
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn bus_register(bus: *const bus_type) -> c_int;
    fn bus_unregister(bus: *const bus_type);
    fn snd_BUG_ON(condition: bool) -> bool;
    fn WARN_ON(condition: bool) -> bool;
}

unsafe fn to_seq_dev(dev: *mut device) -> *mut snd_seq_device {
    dev as *mut snd_seq_device
}

unsafe fn to_seq_drv(drv: *const device_driver) -> *const snd_seq_driver {
    drv as *const snd_seq_driver
}

/*
 * bus definition
 */
unsafe extern "C" fn snd_seq_bus_match(dev: *mut device, drv: *const device_driver) -> c_int {
    let sdev = to_seq_dev(dev);
    let sdrv = to_seq_drv(drv);

    ((strcmp((*sdrv).id, (*sdev).id) == 0) && (*sdrv).argsize == (*sdev).argsize) as c_int
}

unsafe extern "C" fn snd_seq_bus_probe(dev: *mut device) -> c_int {
    let sdev = to_seq_dev(dev);
    let sdrv = to_seq_drv((*dev).driver);

    if let Some(probe) = (*sdrv).probe {
        probe(sdev)
    } else {
        0
    }
}

unsafe extern "C" fn snd_seq_bus_remove(dev: *mut device) {
    let sdev = to_seq_dev(dev);
    let sdrv = to_seq_drv((*dev).driver);

    if let Some(remove) = (*sdrv).remove {
        remove(sdev);
    }
}

static snd_seq_bus_type: bus_type = bus_type {
    name: b"snd_seq\0".as_ptr() as *const c_char,
    match_: Some(snd_seq_bus_match),
    probe: Some(snd_seq_bus_probe),
    remove: Some(snd_seq_bus_remove),
};

/*
 * proc interface -- just for compatibility
 */
/* #ifdef CONFIG_SND_PROC_FS */
static mut info_entry: *mut snd_info_entry = core::ptr::null_mut();

unsafe extern "C" fn print_dev_info(dev: *mut device, data: *mut c_void) -> c_int {
    let sdev = to_seq_dev(dev);
    let buffer = data as *mut snd_info_buffer;

    snd_iprintf(
        buffer,
        b"snd-%s,%s,%d\n\0".as_ptr() as *const c_char,
        (*sdev).id,
        if !(*dev).driver.is_null() {
            b"loaded\0".as_ptr() as *const c_char
        } else {
            b"empty\0".as_ptr() as *const c_char
        },
        if !(*dev).driver.is_null() { 1 } else { 0 },
    );
    0
}

unsafe extern "C" fn snd_seq_device_info(
    _entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    bus_for_each_dev(
        &snd_seq_bus_type,
        core::ptr::null_mut(),
        buffer as *mut c_void,
        Some(print_dev_info),
    );
}
/* #endif */

/*
 * load all registered drivers (called from seq_clientmgr.c)
 */

/* #ifdef CONFIG_MODULES */
/* flag to block auto-loading */
static mut snd_seq_in_init: atomic_t = atomic_t { counter: 1 }; /* blocked as default */

unsafe extern "C" fn request_seq_drv(dev: *mut device, _data: *mut c_void) -> c_int {
    let sdev = to_seq_dev(dev);

    if (*dev).driver.is_null() {
        request_module(b"snd-%s\0".as_ptr() as *const c_char, (*sdev).id);
    }
    0
}

unsafe extern "C" fn autoload_drivers(_work: *mut work_struct) {
    /* avoid reentrance */
    if atomic_inc_return(&mut snd_seq_in_init) == 1 {
        bus_for_each_dev(
            &snd_seq_bus_type,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            Some(request_seq_drv),
        );
    }
    atomic_dec(&mut snd_seq_in_init);
}

static mut autoload_work: work_struct = work_struct { _private: [] };

unsafe fn queue_autoload_drivers() {
    schedule_work(&mut autoload_work);
}

#[no_mangle]
pub unsafe extern "C" fn snd_seq_autoload_init() {
    atomic_dec(&mut snd_seq_in_init);
    /* #ifdef CONFIG_SND_SEQUENCER_MODULE */
    /* initial autoload only when snd-seq is a module */
    queue_autoload_drivers();
    /* #endif */
}

#[no_mangle]
pub unsafe extern "C" fn snd_seq_autoload_exit() {
    atomic_inc(&mut snd_seq_in_init);
}

#[no_mangle]
pub unsafe extern "C" fn snd_seq_device_load_drivers() {
    queue_autoload_drivers();
    flush_work(&mut autoload_work);
}

unsafe fn cancel_autoload_drivers() {
    cancel_work_sync(&mut autoload_work);
}
/* #else
static inline void queue_autoload_drivers(void)
{
}

static inline void cancel_autoload_drivers(void)
{
}
#endif */

/*
 * device management
 */
unsafe extern "C" fn snd_seq_device_dev_free(device: *mut snd_device) -> c_int {
    let dev = (*device).device_data as *mut snd_seq_device;

    cancel_autoload_drivers();
    if let Some(private_free) = (*dev).private_free {
        private_free(dev);
    }
    put_device(&mut (*dev).dev);
    0
}

unsafe extern "C" fn snd_seq_device_dev_register(device: *mut snd_device) -> c_int {
    let dev = (*device).device_data as *mut snd_seq_device;
    let mut err: c_int;

    err = device_add(&mut (*dev).dev);
    if err < 0 {
        return err;
    }
    if (*dev).dev.driver.is_null() {
        queue_autoload_drivers();
    }
    0
}

unsafe extern "C" fn snd_seq_device_dev_disconnect(device: *mut snd_device) -> c_int {
    let dev = (*device).device_data as *mut snd_seq_device;

    device_del(&mut (*dev).dev);
    0
}

unsafe extern "C" fn snd_seq_dev_release(dev: *mut device) {
    kfree(to_seq_dev(dev) as *const c_void);
}

/*
 * register a sequencer device
 * card = card info
 * device = device number (if any)
 * id = id of driver
 * result = return pointer (NULL allowed if unnecessary)
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_device_new(
    card: *mut snd_card,
    device: c_int,
    id: *const c_char,
    argsize: c_int,
    result: *mut *mut snd_seq_device,
) -> c_int {
    let mut dev: *mut snd_seq_device;
    let mut err: c_int;
    static dops: snd_device_ops = snd_device_ops {
        dev_free: Some(snd_seq_device_dev_free),
        dev_register: Some(snd_seq_device_dev_register),
        dev_disconnect: Some(snd_seq_device_dev_disconnect),
    };

    if !result.is_null() {
        *result = core::ptr::null_mut();
    }

    if snd_BUG_ON(id.is_null()) {
        return -EINVAL;
    }

    if argsize < 0 {
        return -EINVAL;
    }

    dev = kzalloc_flex_snd_seq_device_args(argsize);
    if dev.is_null() {
        return -ENOMEM;
    }

    /* set up device info */
    (*dev).card = card;
    (*dev).device = device;
    (*dev).id = id;
    (*dev).argsize = argsize;

    device_initialize(&mut (*dev).dev);
    (*dev).dev.parent = &mut (*card).card_dev;
    (*dev).dev.bus = &snd_seq_bus_type;
    (*dev).dev.release = Some(snd_seq_dev_release);
    dev_set_name(
        &mut (*dev).dev,
        b"%s-%d-%d\0".as_ptr() as *const c_char,
        (*dev).id,
        (*card).number,
        device,
    );

    /* add this device to the list */
    err = snd_device_new(card, SNDRV_DEV_SEQUENCER, dev as *mut c_void, &dops);
    if err < 0 {
        put_device(&mut (*dev).dev);
        return err;
    }

    if !result.is_null() {
        *result = dev;
    }

    0
}

/*
 * driver registration
 */
#[no_mangle]
pub unsafe extern "C" fn __snd_seq_driver_register(
    drv: *mut snd_seq_driver,
    mod_: *mut module,
) -> c_int {
    if WARN_ON(
        (*drv).driver.name.is_null()
            || (*drv).id.is_null()
            || (*drv).driver.probe.is_some()
            || (*drv).driver.remove.is_some(),
    ) {
        return -EINVAL;
    }

    (*drv).driver.bus = &snd_seq_bus_type;
    (*drv).driver.owner = mod_;

    driver_register(&mut (*drv).driver)
}

#[no_mangle]
pub unsafe extern "C" fn snd_seq_driver_unregister(drv: *mut snd_seq_driver) {
    driver_unregister(&mut (*drv).driver);
}

/*
 * module part
 */

unsafe extern "C" fn seq_dev_proc_init() -> c_int {
    /* #ifdef CONFIG_SND_PROC_FS */
    info_entry = snd_info_create_module_entry(
        THIS_MODULE,
        b"drivers\0".as_ptr() as *const c_char,
        snd_seq_root,
    );
    if info_entry.is_null() {
        return -ENOMEM;
    }
    (*info_entry).content = SNDRV_INFO_CONTENT_TEXT;
    (*info_entry).c.text.read = Some(snd_seq_device_info);
    if snd_info_register(info_entry) < 0 {
        snd_info_free_entry(info_entry);
        return -ENOMEM;
    }
    /* #endif */
    0
}

unsafe extern "C" fn alsa_seq_device_init() -> c_int {
    let mut err: c_int;

    err = bus_register(&snd_seq_bus_type);
    if err < 0 {
        return err;
    }
    err = seq_dev_proc_init();
    if err < 0 {
        bus_unregister(&snd_seq_bus_type);
    }
    err
}

unsafe extern "C" fn alsa_seq_device_exit() {
    /* #ifdef CONFIG_MODULES */
    cancel_work_sync(&mut autoload_work);
    /* #endif */
    /* #ifdef CONFIG_SND_PROC_FS */
    snd_info_free_entry(info_entry);
    /* #endif */
    bus_unregister(&snd_seq_bus_type);
}

/* subsys_initcall(alsa_seq_device_init) */
/* module_exit(alsa_seq_device_exit) */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
