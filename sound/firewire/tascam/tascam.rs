// SPDX-License-Identifier: GPL-2.0-only
/*
 * tascam.rs - a part of driver for TASCAM FireWire series
 *
 * Copyright (c) 2015 Takashi Sakamoto
 */

// Rust translation of tascam.c. External kernel/ALSA/FireWire symbols and
// types are declared here as dependencies supplied by surrounding files.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, size_of_val};
use core::ptr;

const ENODEV: c_int = 19;
const IEEE1394_MATCH_VENDOR_ID: u32 = 0x01;
const IEEE1394_MATCH_SPECIFIER_ID: u32 = 0x02;
const IEEE1394_MATCH_VERSION: u32 = 0x04;

#[repr(C)]
pub struct snd_tscm_spec {
    pub name: *const c_char,
    pub has_adat: bool,
    pub has_spdif: bool,
    pub pcm_capture_analog_channels: c_uint,
    pub pcm_playback_analog_channels: c_uint,
    pub midi_capture_ports: c_uint,
    pub midi_playback_ports: c_uint,
}

#[repr(C)]
pub struct snd_tscm {
    pub unit: *mut fw_unit,
    pub card: *mut snd_card,
    pub spec: *const snd_tscm_spec,
    pub mutex: mutex,
    pub lock: spinlock_t,
    pub hwdep_wait: wait_queue_head_t,
}

#[repr(C)]
pub struct snd_card {
    pub private_data: *mut snd_tscm,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub mixername: [c_char; 80],
    pub longname: [c_char; 80],
}

#[repr(C)]
pub struct fw_unit {
    pub device: device,
}

#[repr(C)]
pub struct fw_device {
    pub config_rom: *const u32,
    pub config_rom_length: c_uint,
    pub max_speed: c_uint,
}

#[repr(C)]
pub struct ieee1394_device_id {
    pub match_flags: u32,
    pub vendor_id: u32,
    pub specifier_id: u32,
    pub version: u32,
}

#[repr(C)]
pub struct fw_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut fw_unit, *const ieee1394_device_id) -> c_int>,
    pub update: Option<unsafe extern "C" fn(*mut fw_unit)>,
    pub remove: Option<unsafe extern "C" fn(*mut fw_unit)>,
    pub id_table: *const ieee1394_device_id,
}

#[repr(C)]
pub struct device_driver {
    pub owner: *mut c_void,
    pub name: *const c_char,
    pub bus: *mut bus_type,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
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
pub struct bus_type {
    _private: [u8; 0],
}

unsafe extern "C" {
    static THIS_MODULE: *mut c_void;
    static KBUILD_MODNAME: [c_char; 0];
    static mut fw_bus_type: bus_type;

    fn fw_parent_device(unit: *mut fw_unit) -> *mut fw_device;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;

    fn snd_tscm_transaction_unregister(tscm: *mut snd_tscm);
    fn snd_tscm_stream_destroy_duplex(tscm: *mut snd_tscm);
    fn mutex_destroy(mutex: *mut mutex);
    fn fw_unit_put(unit: *mut fw_unit);

    fn snd_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *const c_char,
        module: *mut c_void,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn fw_unit_get(unit: *mut fw_unit) -> *mut fw_unit;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn mutex_init(mutex: *mut mutex);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn init_waitqueue_head(wq_head: *mut wait_queue_head_t);
    fn snd_tscm_transaction_register(tscm: *mut snd_tscm) -> c_int;
    fn snd_tscm_stream_init_duplex(tscm: *mut snd_tscm) -> c_int;
    fn snd_tscm_proc_init(tscm: *mut snd_tscm);
    fn snd_tscm_create_pcm_devices(tscm: *mut snd_tscm) -> c_int;
    fn snd_tscm_create_midi_devices(tscm: *mut snd_tscm) -> c_int;
    fn snd_tscm_create_hwdep_device(tscm: *mut snd_tscm) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_card_free(card: *mut snd_card);

    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_tscm_transaction_reregister(tscm: *mut snd_tscm);
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
    fn snd_tscm_stream_update_duplex(tscm: *mut snd_tscm);

    fn driver_register(driver: *mut device_driver) -> c_int;
    fn driver_unregister(driver: *mut device_driver);
}

const MODEL_FW_1884: &[u8; 8] = b"FW-1884\0";
const MODEL_FW_1082: &[u8; 8] = b"FW-1082\0";
const MODEL_FW_1804: &[u8; 8] = b"FW-1804\0";

static model_specs: [snd_tscm_spec; 3] = [
    snd_tscm_spec {
        name: MODEL_FW_1884.as_ptr() as *const c_char,
        has_adat: true,
        has_spdif: true,
        pcm_capture_analog_channels: 8,
        pcm_playback_analog_channels: 8,
        midi_capture_ports: 4,
        midi_playback_ports: 4,
    },
    snd_tscm_spec {
        name: MODEL_FW_1082.as_ptr() as *const c_char,
        has_adat: false,
        has_spdif: true,
        pcm_capture_analog_channels: 8,
        pcm_playback_analog_channels: 2,
        midi_capture_ports: 2,
        midi_playback_ports: 2,
    },
    snd_tscm_spec {
        name: MODEL_FW_1804.as_ptr() as *const c_char,
        has_adat: true,
        has_spdif: true,
        pcm_capture_analog_channels: 8,
        pcm_playback_analog_channels: 2,
        midi_capture_ports: 2,
        midi_playback_ports: 4,
    },
];

unsafe extern "C" fn identify_model(tscm: *mut snd_tscm) -> c_int {
    let fw_dev = fw_parent_device((*tscm).unit);
    let config_rom = (*fw_dev).config_rom;
    let mut model: [c_char; 9] = [0; 9];
    let mut i: c_uint;
    let mut c: u8;

    if (*fw_dev).config_rom_length < 30 {
        dev_err(
            &mut (*(*tscm).unit).device,
            b"Configuration ROM is too short.\n\0".as_ptr() as *const c_char,
        );
        return -ENODEV;
    }

    /* Pick up model name from certain addresses. */
    i = 0;
    while i < 8 {
        c = ((*config_rom.add(28 + (i / 4) as usize)) >> (24 - 8 * (i % 4))) as u8;
        if c == b'\0' {
            break;
        }
        model[i as usize] = c as c_char;
        i += 1;
    }
    model[i as usize] = b'\0' as c_char;

    i = 0;
    while (i as usize) < model_specs.len() {
        if strcmp(model.as_ptr(), model_specs[i as usize].name) == 0 {
            (*tscm).spec = &model_specs[i as usize];
            break;
        }
        i += 1;
    }
    if (*tscm).spec.is_null() {
        return -ENODEV;
    }

    strscpy((*(*tscm).card).driver.as_mut_ptr(), b"FW-TASCAM\0".as_ptr() as *const c_char);
    strscpy((*(*tscm).card).shortname.as_mut_ptr(), model.as_ptr());
    strscpy((*(*tscm).card).mixername.as_mut_ptr(), model.as_ptr());
    snprintf(
        (*(*tscm).card).longname.as_mut_ptr(),
        size_of_val(&(*(*tscm).card).longname),
        b"TASCAM %s, GUID %08x%08x at %s, S%d\0".as_ptr() as *const c_char,
        model.as_ptr(),
        *(*fw_dev).config_rom.add(3),
        *(*fw_dev).config_rom.add(4),
        dev_name(&mut (*(*tscm).unit).device),
        100u32 << (*fw_dev).max_speed,
    );

    0
}

unsafe extern "C" fn tscm_card_free(card: *mut snd_card) {
    let tscm = (*card).private_data;

    snd_tscm_transaction_unregister(tscm);
    snd_tscm_stream_destroy_duplex(tscm);

    mutex_destroy(&mut (*tscm).mutex);
    fw_unit_put((*tscm).unit);
}

unsafe extern "C" fn snd_tscm_probe(
    unit: *mut fw_unit,
    _entry: *const ieee1394_device_id,
) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let tscm: *mut snd_tscm;
    let mut err: c_int;

    err = snd_card_new(
        &mut (*unit).device,
        -1,
        ptr::null(),
        THIS_MODULE,
        size_of::<snd_tscm>(),
        &mut card,
    );
    if err < 0 {
        return err;
    }
    (*card).private_free = Some(tscm_card_free);

    tscm = (*card).private_data;
    (*tscm).unit = fw_unit_get(unit);
    dev_set_drvdata(&mut (*unit).device, tscm as *mut c_void);
    (*tscm).card = card;

    mutex_init(&mut (*tscm).mutex);
    spin_lock_init(&mut (*tscm).lock);
    init_waitqueue_head(&mut (*tscm).hwdep_wait);

    err = identify_model(tscm);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    err = snd_tscm_transaction_register(tscm);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    err = snd_tscm_stream_init_duplex(tscm);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    snd_tscm_proc_init(tscm);

    err = snd_tscm_create_pcm_devices(tscm);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    err = snd_tscm_create_midi_devices(tscm);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    err = snd_tscm_create_hwdep_device(tscm);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    err = snd_card_register(card);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    0
}

unsafe extern "C" fn snd_tscm_update(unit: *mut fw_unit) {
    let tscm = dev_get_drvdata(&mut (*unit).device) as *mut snd_tscm;

    snd_tscm_transaction_reregister(tscm);

    mutex_lock(&mut (*tscm).mutex);
    snd_tscm_stream_update_duplex(tscm);
    mutex_unlock(&mut (*tscm).mutex);
}

unsafe extern "C" fn snd_tscm_remove(unit: *mut fw_unit) {
    let tscm = dev_get_drvdata(&mut (*unit).device) as *mut snd_tscm;

    // Block till all of ALSA character devices are released.
    snd_card_free((*tscm).card);
}

static snd_tscm_id_table: [ieee1394_device_id; 4] = [
    // Tascam, FW-1884.
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID
            | IEEE1394_MATCH_SPECIFIER_ID
            | IEEE1394_MATCH_VERSION,
        vendor_id: 0x00022e,
        specifier_id: 0x00022e,
        version: 0x800000,
    },
    // Tascam, FE-8 (.version = 0x800001)
    // This kernel module doesn't support FE-8 because the most of features
    // can be implemented in userspace without any specific support of this
    // module.
    //
    // .version = 0x800002 is unknown.
    //
    // Tascam, FW-1082.
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID
            | IEEE1394_MATCH_SPECIFIER_ID
            | IEEE1394_MATCH_VERSION,
        vendor_id: 0x00022e,
        specifier_id: 0x00022e,
        version: 0x800003,
    },
    // Tascam, FW-1804.
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID
            | IEEE1394_MATCH_SPECIFIER_ID
            | IEEE1394_MATCH_VERSION,
        vendor_id: 0x00022e,
        specifier_id: 0x00022e,
        version: 0x800004,
    },
    ieee1394_device_id {
        match_flags: 0,
        vendor_id: 0,
        specifier_id: 0,
        version: 0,
    },
];
// MODULE_DEVICE_TABLE(ieee1394, snd_tscm_id_table);

static mut tscm_driver: fw_driver = fw_driver {
    driver: device_driver {
        owner: unsafe { THIS_MODULE },
        name: unsafe { KBUILD_MODNAME.as_ptr() },
        bus: unsafe { &raw mut fw_bus_type },
    },
    probe: Some(snd_tscm_probe),
    update: Some(snd_tscm_update),
    remove: Some(snd_tscm_remove),
    id_table: snd_tscm_id_table.as_ptr(),
};

unsafe extern "C" fn snd_tscm_init() -> c_int {
    driver_register(&mut tscm_driver.driver)
}

unsafe extern "C" fn snd_tscm_exit() {
    driver_unregister(&mut tscm_driver.driver);
}

// module_init(snd_tscm_init);
// module_exit(snd_tscm_exit);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
