// SPDX-License-Identifier: GPL-2.0-only
/*
 * digi00x.rs - a part of driver for Digidesign Digi 002/003 family
 *
 * Copyright (c) 2014-2015 Takashi Sakamoto
 */

// Translated from digi00x.c. The original included "digi00x.h"; declarations
// supplied by that header and by the kernel/ALSA/FireWire subsystems are kept
// as external dependencies here.

// MODULE_DESCRIPTION("Digidesign Digi 002/003 family Driver");
// MODULE_AUTHOR("Takashi Sakamoto <o-takashi@sakamocchi.jp>");
// MODULE_LICENSE("GPL");

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;

const VENDOR_DIGIDESIGN: u32 = 0x00a07e;
const MODEL_CONSOLE: u32 = 0x000001;
const MODEL_RACK: u32 = 0x000002;
const SPEC_VERSION: u32 = 0x000001;

// Values supplied by external kernel headers/macros in the original C build.
const CSR_MODEL: c_int = 0;
const IEEE1394_MATCH_VENDOR_ID: u32 = 0;
const IEEE1394_MATCH_VERSION: u32 = 0;
const IEEE1394_MATCH_MODEL_ID: u32 = 0;
const THIS_MODULE: *mut module = core::ptr::null_mut();
const KBUILD_MODNAME: *const c_char = core::ptr::null();

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_unit {
    pub device: device,
    pub directory: *mut c_void,
}

#[repr(C)]
pub struct fw_device {
    pub config_rom: *mut u32,
    pub max_speed: c_int,
}

#[repr(C)]
pub struct snd_card {
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub mixername: [c_char; 80],
    pub longname: [c_char; 80],
    pub private_data: *mut snd_dg00x,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_dg00x {
    pub unit: *mut fw_unit,
    pub card: *mut snd_card,
    pub mutex: mutex,
    pub lock: spinlock_t,
    pub hwdep_wait: wait_queue_head_t,
    pub is_console: bool,
}

#[repr(C)]
pub struct ieee1394_device_id {
    pub match_flags: u32,
    pub vendor_id: u32,
    pub version: u32,
    pub model_id: u32,
}

#[repr(C)]
pub struct device_driver {
    pub owner: *mut module,
    pub name: *const c_char,
    pub bus: *mut bus_type,
}

#[repr(C)]
pub struct bus_type {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut fw_unit, *const ieee1394_device_id) -> c_int>,
    pub update: Option<unsafe extern "C" fn(*mut fw_unit)>,
    pub remove: Option<unsafe extern "C" fn(*mut fw_unit)>,
    pub id_table: *const ieee1394_device_id,
}

unsafe extern "C" {
    static mut fw_bus_type: bus_type;

    fn fw_parent_device(unit: *mut fw_unit) -> *mut fw_device;
    fn fw_csr_string(directory: *mut c_void, key: c_int, buf: *mut c_char, size: usize) -> c_int;
    fn skip_spaces(str_: *mut c_char) -> *mut c_char;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn dev_name(dev: *const device) -> *const c_char;
    fn snd_dg00x_stream_destroy_duplex(dg00x: *mut snd_dg00x);
    fn snd_dg00x_transaction_unregister(dg00x: *mut snd_dg00x);
    fn mutex_destroy(lock: *mut mutex);
    fn fw_unit_put(unit: *mut fw_unit);
    fn snd_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *const c_char,
        module: *mut module,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn fw_unit_get(unit: *mut fw_unit) -> *mut fw_unit;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn mutex_init(lock: *mut mutex);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn init_waitqueue_head(wq_head: *mut wait_queue_head_t);
    fn snd_dg00x_stream_init_duplex(dg00x: *mut snd_dg00x) -> c_int;
    fn snd_dg00x_proc_init(dg00x: *mut snd_dg00x);
    fn snd_dg00x_create_pcm_devices(dg00x: *mut snd_dg00x) -> c_int;
    fn snd_dg00x_create_midi_devices(dg00x: *mut snd_dg00x) -> c_int;
    fn snd_dg00x_create_hwdep_device(dg00x: *mut snd_dg00x) -> c_int;
    fn snd_dg00x_transaction_register(dg00x: *mut snd_dg00x) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_card_free(card: *mut snd_card);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_dg00x_transaction_reregister(dg00x: *mut snd_dg00x);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn snd_dg00x_stream_update_duplex(dg00x: *mut snd_dg00x);
    fn driver_register(driver: *mut device_driver) -> c_int;
    fn driver_unregister(driver: *mut device_driver);
}

unsafe fn name_card(dg00x: *mut snd_dg00x) -> c_int {
    let fw_dev: *mut fw_device = fw_parent_device((*dg00x).unit);
    let mut name: [c_char; 32] = [0; 32];
    let model: *mut c_char;
    let mut err: c_int;

    err = fw_csr_string((*(*dg00x).unit).directory, CSR_MODEL, name.as_mut_ptr(), size_of::<[c_char; 32]>());
    if err < 0 {
        return err;
    }

    model = skip_spaces(name.as_mut_ptr());

    strscpy((*(*dg00x).card).driver.as_mut_ptr(), c"Digi00x".as_ptr());
    strscpy((*(*dg00x).card).shortname.as_mut_ptr(), model);
    strscpy((*(*dg00x).card).mixername.as_mut_ptr(), model);
    snprintf(
        (*(*dg00x).card).longname.as_mut_ptr(),
        size_of::<[c_char; 80]>(),
        c"Digidesign %s, GUID %08x%08x at %s, S%d".as_ptr(),
        model,
        *(*fw_dev).config_rom.add(3),
        *(*fw_dev).config_rom.add(4),
        dev_name(&(*(*dg00x).unit).device),
        100_i32 << (*fw_dev).max_speed,
    );

    0
}

unsafe extern "C" fn dg00x_card_free(card: *mut snd_card) {
    let dg00x: *mut snd_dg00x = (*card).private_data;

    snd_dg00x_stream_destroy_duplex(dg00x);
    snd_dg00x_transaction_unregister(dg00x);

    mutex_destroy(&mut (*dg00x).mutex);
    fw_unit_put((*dg00x).unit);
}

unsafe extern "C" fn snd_dg00x_probe(unit: *mut fw_unit, entry: *const ieee1394_device_id) -> c_int {
    let mut card: *mut snd_card = core::ptr::null_mut();
    let dg00x: *mut snd_dg00x;
    let mut err: c_int;

    err = snd_card_new(
        &mut (*unit).device,
        -1,
        core::ptr::null(),
        THIS_MODULE,
        size_of::<snd_dg00x>(),
        &mut card,
    );
    if err < 0 {
        return err;
    }
    (*card).private_free = Some(dg00x_card_free);

    dg00x = (*card).private_data;
    (*dg00x).unit = fw_unit_get(unit);
    dev_set_drvdata(&mut (*unit).device, dg00x.cast());
    (*dg00x).card = card;

    mutex_init(&mut (*dg00x).mutex);
    spin_lock_init(&mut (*dg00x).lock);
    init_waitqueue_head(&mut (*dg00x).hwdep_wait);

    (*dg00x).is_console = (*entry).model_id == MODEL_CONSOLE;

    err = name_card(dg00x);
    if err < 0 {
        goto_error(card, err)
    } else {
        err = snd_dg00x_stream_init_duplex(dg00x);
        if err < 0 {
            goto_error(card, err)
        } else {
            snd_dg00x_proc_init(dg00x);

            err = snd_dg00x_create_pcm_devices(dg00x);
            if err < 0 {
                goto_error(card, err)
            } else {
                err = snd_dg00x_create_midi_devices(dg00x);
                if err < 0 {
                    goto_error(card, err)
                } else {
                    err = snd_dg00x_create_hwdep_device(dg00x);
                    if err < 0 {
                        goto_error(card, err)
                    } else {
                        err = snd_dg00x_transaction_register(dg00x);
                        if err < 0 {
                            goto_error(card, err)
                        } else {
                            err = snd_card_register(card);
                            if err < 0 {
                                goto_error(card, err)
                            } else {
                                0
                            }
                        }
                    }
                }
            }
        }
    }
}

unsafe fn goto_error(card: *mut snd_card, err: c_int) -> c_int {
    snd_card_free(card);
    err
}

unsafe extern "C" fn snd_dg00x_update(unit: *mut fw_unit) {
    let dg00x: *mut snd_dg00x = dev_get_drvdata(&mut (*unit).device).cast();

    snd_dg00x_transaction_reregister(dg00x);

    // Original C used guard(mutex)(&dg00x->mutex) for scoped unlocking.
    mutex_lock(&mut (*dg00x).mutex);
    snd_dg00x_stream_update_duplex(dg00x);
    mutex_unlock(&mut (*dg00x).mutex);
}

unsafe extern "C" fn snd_dg00x_remove(unit: *mut fw_unit) {
    let dg00x: *mut snd_dg00x = dev_get_drvdata(&mut (*unit).device).cast();

    // Block till all of ALSA character devices are released.
    snd_card_free((*dg00x).card);
}

static SND_DG00X_ID_TABLE: [ieee1394_device_id; 3] = [
    /* Both of 002/003 use the same ID. */
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID | IEEE1394_MATCH_VERSION | IEEE1394_MATCH_MODEL_ID,
        vendor_id: VENDOR_DIGIDESIGN,
        version: SPEC_VERSION,
        model_id: MODEL_CONSOLE,
    },
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID | IEEE1394_MATCH_VERSION | IEEE1394_MATCH_MODEL_ID,
        vendor_id: VENDOR_DIGIDESIGN,
        version: SPEC_VERSION,
        model_id: MODEL_RACK,
    },
    ieee1394_device_id {
        match_flags: 0,
        vendor_id: 0,
        version: 0,
        model_id: 0,
    },
];
// MODULE_DEVICE_TABLE(ieee1394, snd_dg00x_id_table);

static mut DG00X_DRIVER: fw_driver = fw_driver {
    driver: device_driver {
        owner: THIS_MODULE,
        name: KBUILD_MODNAME,
        bus: unsafe { &mut fw_bus_type },
    },
    probe: Some(snd_dg00x_probe),
    update: Some(snd_dg00x_update),
    remove: Some(snd_dg00x_remove),
    id_table: SND_DG00X_ID_TABLE.as_ptr(),
};

unsafe extern "C" fn snd_dg00x_init() -> c_int {
    driver_register(&mut DG00X_DRIVER.driver)
}

unsafe extern "C" fn snd_dg00x_exit() {
    driver_unregister(&mut DG00X_DRIVER.driver);
}

// module_init(snd_dg00x_init);
// module_exit(snd_dg00x_exit);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
