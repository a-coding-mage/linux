// SPDX-License-Identifier: GPL-2.0-only
/*
 * motu.c - a part of driver for MOTU FireWire series
 *
 * Copyright (c) 2015-2017 Takashi Sakamoto <o-takashi@sakamocchi.jp>
 */

// Rust translation of the implementation source. C include dependency:
// #include "motu.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const OUI_MOTU: c_uint = 0x0001f2;

const SND_MOTU_CLOCK_RATE_COUNT: usize = 6;
// The following constants are supplied by included kernel/MOTU headers:
// CSR_MODEL, IEEE1394_MATCH_VENDOR_ID, IEEE1394_MATCH_SPECIFIER_ID,
// IEEE1394_MATCH_VERSION, SND_MOTU_SPEC_RX_MIDI_2ND_Q,
// SND_MOTU_SPEC_RX_MIDI_3RD_Q, SND_MOTU_SPEC_TX_MIDI_2ND_Q,
// SND_MOTU_SPEC_TX_MIDI_3RD_Q, SND_MOTU_SPEC_REGISTER_DSP,
// SND_MOTU_SPEC_COMMAND_DSP.

type U32 = u32;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bus_type {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
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
pub struct fw_csr_iterator {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
    pub mixername: [c_char; 80],
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
}

#[repr(C)]
pub struct fw_unit {
    pub device: device,
    pub directory: *const c_void,
}

#[repr(C)]
pub struct fw_device {
    pub config_rom: *mut U32,
    pub max_speed: c_int,
}

#[repr(C)]
pub struct snd_motu_spec {
    pub name: *const c_char,
    pub flags: u32,
}

#[repr(C)]
pub struct snd_motu {
    pub unit: *mut fw_unit,
    pub card: *mut snd_card,
    pub spec: *const snd_motu_spec,
    pub mutex: mutex,
    pub lock: spinlock_t,
    pub hwdep_wait: wait_queue_head_t,
}

#[repr(C)]
pub struct ieee1394_device_id {
    pub match_flags: u32,
    pub vendor_id: u32,
    pub specifier_id: u32,
    pub version: u32,
    pub driver_data_ptr: *const c_void,
}

#[repr(C)]
pub struct device_driver {
    pub owner: *mut module,
    pub name: *const c_char,
    pub bus: *mut bus_type,
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
    static mut THIS_MODULE: *mut module;
    static mut fw_bus_type: bus_type;
    static KBUILD_MODNAME: *const c_char;

    static snd_motu_spec_828: snd_motu_spec;
    static snd_motu_spec_896: snd_motu_spec;
    static snd_motu_spec_828mk2: snd_motu_spec;
    static snd_motu_spec_896hd: snd_motu_spec;
    static snd_motu_spec_traveler: snd_motu_spec;
    static snd_motu_spec_ultralite: snd_motu_spec;
    static snd_motu_spec_8pre: snd_motu_spec;
    static snd_motu_spec_828mk3_fw: snd_motu_spec;
    static snd_motu_spec_896mk3: snd_motu_spec;
    static snd_motu_spec_ultralite_mk3: snd_motu_spec;
    static snd_motu_spec_traveler_mk3: snd_motu_spec;
    static snd_motu_spec_828mk3_hybrid: snd_motu_spec;
    static snd_motu_spec_audio_express: snd_motu_spec;
    static snd_motu_spec_track16: snd_motu_spec;
    static snd_motu_spec_4pre: snd_motu_spec;

    fn fw_parent_device(unit: *mut fw_unit) -> *mut fw_device;
    fn fw_csr_iterator_init(it: *mut fw_csr_iterator, directory: *const c_void);
    fn fw_csr_iterator_next(it: *mut fw_csr_iterator, key: *mut c_int, val: *mut c_int) -> bool;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn dev_name(dev: *const device) -> *const c_char;
    fn snd_motu_transaction_unregister(motu: *mut snd_motu);
    fn snd_motu_stream_destroy_duplex(motu: *mut snd_motu);
    fn mutex_destroy(mutex: *mut mutex);
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
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn mutex_init(mutex: *mut mutex);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn init_waitqueue_head(wait: *mut wait_queue_head_t);
    fn snd_motu_transaction_register(motu: *mut snd_motu) -> c_int;
    fn snd_motu_stream_init_duplex(motu: *mut snd_motu) -> c_int;
    fn snd_motu_proc_init(motu: *mut snd_motu);
    fn snd_motu_create_pcm_devices(motu: *mut snd_motu) -> c_int;
    fn snd_motu_create_midi_devices(motu: *mut snd_motu) -> c_int;
    fn snd_motu_create_hwdep_device(motu: *mut snd_motu) -> c_int;
    fn snd_motu_register_dsp_message_parser_new(motu: *mut snd_motu) -> c_int;
    fn snd_motu_command_dsp_message_parser_new(motu: *mut snd_motu) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_card_free(card: *mut snd_card);
    fn snd_motu_transaction_reregister(motu: *mut snd_motu);
    fn driver_register(driver: *mut device_driver) -> c_int;
    fn driver_unregister(driver: *mut device_driver);
}

// MODULE_DESCRIPTION("MOTU FireWire driver");
// MODULE_AUTHOR("Takashi Sakamoto <o-takashi@sakamocchi.jp>");
// MODULE_LICENSE("GPL");

#[no_mangle]
pub static snd_motu_clock_rates: [c_uint; SND_MOTU_CLOCK_RATE_COUNT] = [
    /* mode 0 */
    44100,
    48000,
    /* mode 1 */
    88200,
    96000,
    /* mode 2 */
    176400,
    192000,
];

unsafe extern "C" fn name_card(motu: *mut snd_motu) {
    let fw_dev: *mut fw_device = fw_parent_device((*motu).unit);
    let mut it: fw_csr_iterator = core::mem::zeroed();
    let mut key: c_int = 0;
    let mut val: c_int = 0;
    let mut version: U32 = 0;

    fw_csr_iterator_init(&mut it, (*(*motu).unit).directory);
    while fw_csr_iterator_next(&mut it, &mut key, &mut val) {
        match key {
            CSR_MODEL => {
                version = val as U32;
            }
            _ => {}
        }
    }

    strscpy((*(*motu).card).driver.as_mut_ptr(), c"FW-MOTU".as_ptr());
    strscpy((*(*motu).card).shortname.as_mut_ptr(), (*(*motu).spec).name);
    strscpy((*(*motu).card).mixername.as_mut_ptr(), (*(*motu).spec).name);
    snprintf(
        (*(*motu).card).longname.as_mut_ptr(),
        size_of_val(&(*(*motu).card).longname),
        c"MOTU %s (version:%06x), GUID %08x%08x at %s, S%d".as_ptr(),
        (*(*motu).spec).name,
        version,
        *(*fw_dev).config_rom.add(3),
        *(*fw_dev).config_rom.add(4),
        dev_name(&(*(*motu).unit).device),
        100 << (*fw_dev).max_speed,
    );
}

unsafe extern "C" fn motu_card_free(card: *mut snd_card) {
    let motu: *mut snd_motu = (*card).private_data as *mut snd_motu;

    snd_motu_transaction_unregister(motu);
    snd_motu_stream_destroy_duplex(motu);

    mutex_destroy(&mut (*motu).mutex);
    fw_unit_put((*motu).unit);
}

unsafe extern "C" fn motu_probe(
    unit: *mut fw_unit,
    entry: *const ieee1394_device_id,
) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let motu: *mut snd_motu;
    let mut err: c_int;

    err = snd_card_new(
        &mut (*unit).device,
        -1,
        ptr::null(),
        THIS_MODULE,
        size_of::<snd_motu>(),
        &mut card,
    );
    if err < 0 {
        return err;
    }
    (*card).private_free = Some(motu_card_free);

    motu = (*card).private_data as *mut snd_motu;
    (*motu).unit = fw_unit_get(unit);
    dev_set_drvdata(&mut (*unit).device, motu as *mut c_void);
    (*motu).card = card;

    (*motu).spec = (*entry).driver_data_ptr as *const snd_motu_spec;
    mutex_init(&mut (*motu).mutex);
    spin_lock_init(&mut (*motu).lock);
    init_waitqueue_head(&mut (*motu).hwdep_wait);

    name_card(motu);

    err = snd_motu_transaction_register(motu);
    if err < 0 {
        goto_error(card, err)
    } else {
        err = snd_motu_stream_init_duplex(motu);
        if err < 0 {
            goto_error(card, err)
        } else {
            snd_motu_proc_init(motu);

            err = snd_motu_create_pcm_devices(motu);
            if err < 0 {
                goto_error(card, err)
            } else {
                if ((*(*motu).spec).flags & SND_MOTU_SPEC_RX_MIDI_2ND_Q) != 0
                    || ((*(*motu).spec).flags & SND_MOTU_SPEC_RX_MIDI_3RD_Q) != 0
                    || ((*(*motu).spec).flags & SND_MOTU_SPEC_TX_MIDI_2ND_Q) != 0
                    || ((*(*motu).spec).flags & SND_MOTU_SPEC_TX_MIDI_3RD_Q) != 0
                {
                    err = snd_motu_create_midi_devices(motu);
                    if err < 0 {
                        return goto_error(card, err);
                    }
                }

                err = snd_motu_create_hwdep_device(motu);
                if err < 0 {
                    goto_error(card, err)
                } else {
                    if ((*(*motu).spec).flags & SND_MOTU_SPEC_REGISTER_DSP) != 0 {
                        err = snd_motu_register_dsp_message_parser_new(motu);
                        if err < 0 {
                            return goto_error(card, err);
                        }
                    } else if ((*(*motu).spec).flags & SND_MOTU_SPEC_COMMAND_DSP) != 0 {
                        err = snd_motu_command_dsp_message_parser_new(motu);
                        if err < 0 {
                            return goto_error(card, err);
                        }
                    }

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

unsafe fn goto_error(card: *mut snd_card, err: c_int) -> c_int {
    snd_card_free(card);
    err
}

unsafe extern "C" fn motu_remove(unit: *mut fw_unit) {
    let motu: *mut snd_motu = dev_get_drvdata(&mut (*unit).device) as *mut snd_motu;

    // Block till all of ALSA character devices are released.
    snd_card_free((*motu).card);
}

unsafe extern "C" fn motu_bus_update(unit: *mut fw_unit) {
    let motu: *mut snd_motu = dev_get_drvdata(&mut (*unit).device) as *mut snd_motu;

    /* The handler address register becomes initialized. */
    snd_motu_transaction_reregister(motu);
}

const fn snd_motu_dev_entry(model: u32, data_ptr: *const snd_motu_spec) -> ieee1394_device_id {
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID
            | IEEE1394_MATCH_SPECIFIER_ID
            | IEEE1394_MATCH_VERSION,
        vendor_id: OUI_MOTU,
        specifier_id: OUI_MOTU,
        version: model,
        driver_data_ptr: data_ptr as *const c_void,
    }
}

static motu_id_table: [ieee1394_device_id; 18] = unsafe {
    [
        snd_motu_dev_entry(0x000001, &snd_motu_spec_828),
        snd_motu_dev_entry(0x000002, &snd_motu_spec_896),
        snd_motu_dev_entry(0x000003, &snd_motu_spec_828mk2),
        snd_motu_dev_entry(0x000005, &snd_motu_spec_896hd),
        snd_motu_dev_entry(0x000009, &snd_motu_spec_traveler),
        snd_motu_dev_entry(0x00000d, &snd_motu_spec_ultralite),
        snd_motu_dev_entry(0x00000f, &snd_motu_spec_8pre),
        snd_motu_dev_entry(0x000015, &snd_motu_spec_828mk3_fw), // FireWire only.
        snd_motu_dev_entry(0x000017, &snd_motu_spec_896mk3), // FireWire only.
        snd_motu_dev_entry(0x000019, &snd_motu_spec_ultralite_mk3), // FireWire only.
        snd_motu_dev_entry(0x00001b, &snd_motu_spec_traveler_mk3),
        snd_motu_dev_entry(0x000030, &snd_motu_spec_ultralite_mk3), // Hybrid.
        snd_motu_dev_entry(0x000035, &snd_motu_spec_828mk3_hybrid), // Hybrid.
        snd_motu_dev_entry(0x000037, &snd_motu_spec_896mk3), // Hybrid.
        snd_motu_dev_entry(0x000033, &snd_motu_spec_audio_express),
        snd_motu_dev_entry(0x000039, &snd_motu_spec_track16),
        snd_motu_dev_entry(0x000045, &snd_motu_spec_4pre),
        ieee1394_device_id {
            match_flags: 0,
            vendor_id: 0,
            specifier_id: 0,
            version: 0,
            driver_data_ptr: ptr::null(),
        },
    ]
};
// MODULE_DEVICE_TABLE(ieee1394, motu_id_table);

static mut motu_driver: fw_driver = unsafe {
    fw_driver {
        driver: device_driver {
            owner: THIS_MODULE,
            name: KBUILD_MODNAME,
            bus: &mut fw_bus_type,
        },
        probe: Some(motu_probe),
        update: Some(motu_bus_update),
        remove: Some(motu_remove),
        id_table: motu_id_table.as_ptr(),
    }
};

unsafe extern "C" fn alsa_motu_init() -> c_int {
    driver_register(&mut motu_driver.driver)
}

unsafe extern "C" fn alsa_motu_exit() {
    driver_unregister(&mut motu_driver.driver);
}

// module_init(alsa_motu_init);
// module_exit(alsa_motu_exit);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
