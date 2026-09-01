// SPDX-License-Identifier: GPL-2.0-only
/*
 * oxfw.c - a part of driver for OXFW970/971 based devices
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 */

// Rust translation of implementation source ./firewire/oxfw/oxfw.c.
// C include dependency: "oxfw.h".

use core::ffi::{c_char, c_int, c_void};

const OXFORD_FIRMWARE_ID_ADDRESS: u64 = CSR_REGISTER_BASE + 0x50000;
/* 0x970?vvvv or 0x971?vvvv, where vvvv = firmware version */

const OXFORD_HARDWARE_ID_ADDRESS: u64 = CSR_REGISTER_BASE + 0x90020;
const OXFORD_HARDWARE_ID_OXFW970: u32 = 0x39443841;
const OXFORD_HARDWARE_ID_OXFW971: u32 = 0x39373100;

const VENDOR_LOUD: u32 = 0x000ff2;
const VENDOR_GRIFFIN: u32 = 0x001292;
const VENDOR_BEHRINGER: u32 = 0x001564;
const VENDOR_LACIE: u32 = 0x00d04b;
const VENDOR_TASCAM: u32 = 0x00022e;
const OUI_STANTON: u32 = 0x001260;
const OUI_APOGEE: u32 = 0x0003db;
const OUI_OXFORD: u32 = 0x0030e0;

const MODEL_SATELLITE: u32 = 0x00200f;
const MODEL_SCS1M: u32 = 0x001000;
const MODEL_DUET_FW: u32 = 0x01dddd;
const MODEL_ONYX_1640I: u32 = 0x001640;

const SPECIFIER_1394TA: u32 = 0x00a02d;
const VERSION_AVC: u32 = 0x010001;

// MODULE_DESCRIPTION("Oxford Semiconductor FW970/971 driver");
// MODULE_AUTHOR("Clemens Ladisch <clemens@ladisch.de>");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("snd-firewire-speakers");
// MODULE_ALIAS("snd-scs1x");

#[repr(C)]
struct compat_info {
    driver_name: *const c_char,
    vendor_name: *const c_char,
    model_name: *const c_char,
}

unsafe fn detect_loud_models(unit: *mut fw_unit) -> bool {
    static MODELS: [*const c_char; 5] = [
        c"Onyxi".as_ptr(),
        c"Onyx-i".as_ptr(),
        c"Onyx 1640i".as_ptr(),
        c"d.Pro".as_ptr(),
        c"U.420".as_ptr(),
    ];
    let mut model: [c_char; 32] = [0; 32];
    let err: c_int;

    err = fw_csr_string(
        (*unit).directory,
        CSR_MODEL,
        model.as_mut_ptr(),
        core::mem::size_of_val(&model),
    );
    if err < 0 {
        return false;
    }

    match_string(MODELS.as_ptr(), MODELS.len(), model.as_ptr()) >= 0
}

unsafe fn name_card(oxfw: *mut snd_oxfw, entry: *const ieee1394_device_id) -> c_int {
    let fw_dev: *mut fw_device = fw_parent_device((*oxfw).unit);
    let info: *const compat_info;
    let mut vendor: [c_char; 24] = [0; 24];
    let mut model: [c_char; 32] = [0; 32];
    let d: *const c_char;
    let v: *const c_char;
    let m: *const c_char;
    let mut firmware: u32 = 0;
    let mut err: c_int;

    /* get vendor name from root directory */
    err = fw_csr_string(
        (*fw_dev).config_rom.add(5),
        CSR_VENDOR,
        vendor.as_mut_ptr(),
        core::mem::size_of_val(&vendor),
    );
    if err < 0 {
        return err;
    }

    /* get model name from unit directory */
    err = fw_csr_string(
        (*(*oxfw).unit).directory,
        CSR_MODEL,
        model.as_mut_ptr(),
        core::mem::size_of_val(&model),
    );
    if err < 0 {
        return err;
    }

    err = snd_fw_transaction(
        (*oxfw).unit,
        TCODE_READ_QUADLET_REQUEST,
        OXFORD_FIRMWARE_ID_ADDRESS,
        &mut firmware as *mut u32 as *mut c_void,
        4,
        0,
    );
    if err < 0 {
        return err;
    }
    be32_to_cpus(&mut firmware);

    if firmware >> 20 == 0x970 {
        (*oxfw).quirks |= SND_OXFW_QUIRK_JUMBO_PAYLOAD;
    }

    /* to apply card definitions */
    if (*entry).vendor_id == VENDOR_GRIFFIN || (*entry).vendor_id == VENDOR_LACIE {
        info = (*entry).driver_data_ptr as *const compat_info;
        d = (*info).driver_name;
        v = (*info).vendor_name;
        m = (*info).model_name;
    } else {
        d = c"OXFW".as_ptr();
        v = vendor.as_ptr();
        m = model.as_ptr();
    }

    strscpy((*(*oxfw).card).driver.as_mut_ptr(), d);
    strscpy((*(*oxfw).card).mixername.as_mut_ptr(), m);
    strscpy((*(*oxfw).card).shortname.as_mut_ptr(), m);

    scnprintf(
        (*(*oxfw).card).longname.as_mut_ptr(),
        core::mem::size_of_val(&(*(*oxfw).card).longname),
        c"%s %s (OXFW%x %04x), GUID %08x%08x at %s, S%d".as_ptr(),
        v,
        m,
        firmware >> 20,
        firmware & 0xffff,
        *(*fw_dev).config_rom.add(3),
        *(*fw_dev).config_rom.add(4),
        dev_name(&mut (*(*oxfw).unit).device),
        100 << (*fw_dev).max_speed,
    );

    err
}

unsafe fn oxfw_card_free(card: *mut snd_card) {
    let oxfw: *mut snd_oxfw = (*card).private_data as *mut snd_oxfw;

    if (*oxfw).has_output || (*oxfw).has_input {
        snd_oxfw_stream_destroy_duplex(oxfw);
    }

    mutex_destroy(&mut (*oxfw).mutex);
    fw_unit_put((*oxfw).unit);
}

unsafe fn detect_quirks(
    oxfw: *mut snd_oxfw,
    entry: *const ieee1394_device_id,
) -> c_int {
    let fw_dev: *mut fw_device = fw_parent_device((*oxfw).unit);
    let mut it: fw_csr_iterator = core::mem::zeroed();
    let mut key: c_int = 0;
    let mut val: c_int = 0;
    let mut vendor: c_int;
    let mut model: c_int;

    /*
     * Add ALSA control elements for two models to keep compatibility to
     * old firewire-speaker module.
     */
    if (*entry).vendor_id == VENDOR_GRIFFIN {
        return snd_oxfw_add_spkr(oxfw, false);
    }
    if (*entry).vendor_id == VENDOR_LACIE {
        return snd_oxfw_add_spkr(oxfw, true);
    }

    /*
     * Stanton models supports asynchronous transactions for unique MIDI
     * messages.
     */
    if (*entry).vendor_id == OUI_STANTON {
        (*oxfw).quirks |= SND_OXFW_QUIRK_SCS_TRANSACTION;
        if (*entry).model_id == MODEL_SCS1M {
            (*oxfw).quirks |= SND_OXFW_QUIRK_BLOCKING_TRANSMISSION;
        }

        // No physical MIDI ports.
        (*oxfw).midi_input_ports = 0;
        (*oxfw).midi_output_ports = 0;

        return snd_oxfw_scs1x_add(oxfw);
    }

    if (*entry).vendor_id == OUI_APOGEE && (*entry).model_id == MODEL_DUET_FW {
        (*oxfw).quirks |=
            SND_OXFW_QUIRK_BLOCKING_TRANSMISSION | SND_OXFW_QUIRK_IGNORE_NO_INFO_PACKET;
    }

    /*
     * TASCAM FireOne has physical control and requires a pair of additional
     * MIDI ports.
     */
    if (*entry).vendor_id == VENDOR_TASCAM {
        (*oxfw).midi_input_ports += 1;
        (*oxfw).midi_output_ports += 1;
        return 0;
    }

    /* Seek from Root Directory of Config ROM. */
    vendor = 0;
    model = 0;
    fw_csr_iterator_init(&mut it, (*fw_dev).config_rom.add(5));
    while fw_csr_iterator_next(&mut it, &mut key, &mut val) {
        if key == CSR_VENDOR {
            vendor = val;
        } else if key == CSR_MODEL {
            model = val;
        }
    }

    if vendor as u32 == VENDOR_LOUD {
        // Mackie Onyx Satellite with base station has a quirk to report a wrong
        // value in 'dbs' field of CIP header against its format information.
        (*oxfw).quirks |= SND_OXFW_QUIRK_WRONG_DBS;

        // OXFW971-based models may transfer events by blocking method.
        if ((*oxfw).quirks & SND_OXFW_QUIRK_JUMBO_PAYLOAD) == 0 {
            (*oxfw).quirks |= SND_OXFW_QUIRK_BLOCKING_TRANSMISSION;
        }

        if model as u32 == MODEL_ONYX_1640I {
            //Unless receiving packets without NOINFO packet, the device transfers
            //mostly half of events in packets than expected.
            (*oxfw).quirks |=
                SND_OXFW_QUIRK_IGNORE_NO_INFO_PACKET | SND_OXFW_QUIRK_VOLUNTARY_RECOVERY;
        }
    }

    0
}

unsafe fn oxfw_probe(unit: *mut fw_unit, entry: *const ieee1394_device_id) -> c_int {
    let mut card: *mut snd_card = core::ptr::null_mut();
    let oxfw: *mut snd_oxfw;
    let mut err: c_int;

    if (*entry).vendor_id == VENDOR_LOUD
        && (*entry).model_id == 0
        && !detect_loud_models(unit)
    {
        return -ENODEV;
    }

    err = snd_card_new(
        &mut (*unit).device,
        -1,
        core::ptr::null(),
        THIS_MODULE,
        core::mem::size_of::<snd_oxfw>(),
        &mut card,
    );
    if err < 0 {
        return err;
    }
    (*card).private_free = Some(oxfw_card_free);

    oxfw = (*card).private_data as *mut snd_oxfw;
    (*oxfw).unit = fw_unit_get(unit);
    dev_set_drvdata(&mut (*unit).device, oxfw as *mut c_void);
    (*oxfw).card = card;

    mutex_init(&mut (*oxfw).mutex);
    spin_lock_init(&mut (*oxfw).lock);
    init_waitqueue_head(&mut (*oxfw).hwdep_wait);

    err = name_card(oxfw, entry);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    if (*entry).vendor_id == OUI_OXFORD && (*entry).model_id == 0x00f970 {
        (*oxfw).quirks |= SND_OXFW_QUIRK_STREAM_FORMAT_INFO_UNSUPPORTED
            | SND_OXFW_QUIRK_DBC_IS_TOTAL_PAYLOAD_QUADLETS;
    }

    err = snd_oxfw_stream_discover(oxfw);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    err = detect_quirks(oxfw, entry);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    if (*oxfw).has_output || (*oxfw).has_input {
        err = snd_oxfw_stream_init_duplex(oxfw);
        if err < 0 {
            snd_card_free(card);
            return err;
        }

        err = snd_oxfw_create_pcm(oxfw);
        if err < 0 {
            snd_card_free(card);
            return err;
        }

        snd_oxfw_proc_init(oxfw);

        err = snd_oxfw_create_midi(oxfw);
        if err < 0 {
            snd_card_free(card);
            return err;
        }

        err = snd_oxfw_create_hwdep(oxfw);
        if err < 0 {
            snd_card_free(card);
            return err;
        }
    }

    err = snd_card_register(card);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    0
}

unsafe fn oxfw_bus_reset(unit: *mut fw_unit) {
    let oxfw: *mut snd_oxfw = dev_get_drvdata(&mut (*unit).device) as *mut snd_oxfw;

    fcp_bus_reset((*oxfw).unit);

    if (*oxfw).has_output || (*oxfw).has_input {
        // C used guard(mutex)(&oxfw->mutex) for the following update.
        let _guard = mutex_guard(&mut (*oxfw).mutex);
        snd_oxfw_stream_update_duplex(oxfw);
    }

    if ((*oxfw).quirks & SND_OXFW_QUIRK_SCS_TRANSACTION) != 0 {
        snd_oxfw_scs1x_update(oxfw);
    }
}

unsafe fn oxfw_remove(unit: *mut fw_unit) {
    let oxfw: *mut snd_oxfw = dev_get_drvdata(&mut (*unit).device) as *mut snd_oxfw;

    // Block till all of ALSA character devices are released.
    snd_card_free((*oxfw).card);
}

static GRIFFIN_FIREWAVE: compat_info = compat_info {
    driver_name: c"FireWave".as_ptr(),
    vendor_name: c"Griffin".as_ptr(),
    model_name: c"FireWave".as_ptr(),
};

static LACIE_SPEAKERS: compat_info = compat_info {
    driver_name: c"FWSpeakers".as_ptr(),
    vendor_name: c"LaCie".as_ptr(),
    model_name: c"FireWire Speakers".as_ptr(),
};

const fn oxfw_dev_entry(
    vendor: u32,
    model: u32,
    data: *const c_void,
) -> ieee1394_device_id {
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID
            | IEEE1394_MATCH_MODEL_ID
            | IEEE1394_MATCH_SPECIFIER_ID
            | IEEE1394_MATCH_VERSION,
        vendor_id: vendor,
        model_id: model,
        specifier_id: SPECIFIER_1394TA,
        version: VERSION_AVC,
        driver_data_ptr: data,
    }
}

static OXFW_ID_TABLE: [ieee1394_device_id; 15] = [
    //
    // OXFW970 devices:
    // Initial firmware has a quirk to postpone isoc packet transmission during finishing async
    // transaction. As a result, several isochronous cycles are skipped to transfer the packets
    // and the audio data frames which should have been transferred during the cycles are put
    // into packet at the first isoc cycle after the postpone. Furthermore, the value of SYT
    // field in CIP header is not reliable as synchronization timing,
    //
    oxfw_dev_entry(
        VENDOR_GRIFFIN,
        0x00f970,
        &GRIFFIN_FIREWAVE as *const compat_info as *const c_void,
    ),
    oxfw_dev_entry(
        VENDOR_LACIE,
        0x00f970,
        &LACIE_SPEAKERS as *const compat_info as *const c_void,
    ),
    // Miglia HarmonyAudio (HA02). The numeric vendor ID is ASIC vendor and the model ID is the
    // default value of ASIC.
    oxfw_dev_entry(OUI_OXFORD, 0x00f970, core::ptr::null()),
    // Behringer,F-Control Audio 202. The value of SYT field is not reliable at all.
    oxfw_dev_entry(VENDOR_BEHRINGER, 0x00fc22, core::ptr::null()),
    // Loud Technologies, Tapco Link.FireWire 4x6. The value of SYT field is always 0xffff.
    oxfw_dev_entry(VENDOR_LOUD, 0x000460, core::ptr::null()),
    // Loud Technologies, Mackie Onyx Satellite. Although revised version of firmware is
    // installed to avoid the postpone, the value of SYT field is always 0xffff.
    oxfw_dev_entry(VENDOR_LOUD, MODEL_SATELLITE, core::ptr::null()),
    //
    // OXFW971 devices:
    // The value of SYT field in CIP header is enough reliable. Both of blocking and non-blocking
    // transmission methods are available.
    //
    // Any Mackie(Loud) models (name string/model id):
    //  Onyx-i series (former models):       0x081216
    //  Onyx 1640i:                         0x001640
    //  d.2 pro/d.4 pro (built-in card):    Unknown
    //  U.420:                              Unknown
    //  U.420d:                             Unknown
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID
            | IEEE1394_MATCH_SPECIFIER_ID
            | IEEE1394_MATCH_VERSION,
        vendor_id: VENDOR_LOUD,
        model_id: 0,
        specifier_id: SPECIFIER_1394TA,
        version: VERSION_AVC,
        driver_data_ptr: core::ptr::null(),
    },
    // TASCAM, FireOne.
    oxfw_dev_entry(VENDOR_TASCAM, 0x800007, core::ptr::null()),
    // Stanton, Stanton Controllers & Systems 1 Mixer (SCS.1m).
    oxfw_dev_entry(OUI_STANTON, MODEL_SCS1M, core::ptr::null()),
    // Stanton, Stanton Controllers & Systems 1 Deck (SCS.1d).
    oxfw_dev_entry(OUI_STANTON, 0x002000, core::ptr::null()),
    // APOGEE, duet FireWire.
    oxfw_dev_entry(OUI_APOGEE, MODEL_DUET_FW, core::ptr::null()),
    ieee1394_device_id::zeroed(),
];
// MODULE_DEVICE_TABLE(ieee1394, oxfw_id_table);

static mut OXFW_DRIVER: fw_driver = fw_driver {
    driver: device_driver {
        owner: THIS_MODULE,
        name: KBUILD_MODNAME,
        bus: &fw_bus_type,
    },
    probe: Some(oxfw_probe),
    update: Some(oxfw_bus_reset),
    remove: Some(oxfw_remove),
    id_table: OXFW_ID_TABLE.as_ptr(),
};

unsafe fn snd_oxfw_init() -> c_int {
    driver_register(&mut OXFW_DRIVER.driver)
}

unsafe fn snd_oxfw_exit() {
    driver_unregister(&mut OXFW_DRIVER.driver);
}

// module_init(snd_oxfw_init);
// module_exit(snd_oxfw_exit);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
