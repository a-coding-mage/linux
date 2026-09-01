// SPDX-License-Identifier: GPL-2.0-only
/*
 * TC Applied Technologies Digital Interface Communications Engine driver
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 */

// Rust translation of dice.c. C include dependency: "dice.h".
// MODULE_DESCRIPTION("DICE driver");
// MODULE_AUTHOR("Clemens Ladisch <clemens@ladisch.de>");
// MODULE_LICENSE("GPL");

const OUI_WEISS: u32 = 0x001c6a;
const OUI_LOUD: u32 = 0x000ff2;
const OUI_FOCUSRITE: u32 = 0x00130e;
const OUI_TCELECTRONIC: u32 = 0x000166;
const OUI_ALESIS: u32 = 0x000595;
const OUI_MAUDIO: u32 = 0x000d6c;
const OUI_MYTEK: u32 = 0x001ee8;
const OUI_SSL: u32 = 0x0050c2; // Actually ID reserved by IEEE.
const OUI_PRESONUS: u32 = 0x000a92;
const OUI_HARMAN: u32 = 0x000fd7;
const OUI_AVID: u32 = 0x00a07e;
const OUI_TEAC: u32 = 0x00022e;

const DICE_CATEGORY_ID: u32 = 0x04;
const WEISS_CATEGORY_ID: u32 = 0x00;
const LOUD_CATEGORY_ID: u32 = 0x10;
const HARMAN_CATEGORY_ID: u32 = 0x20;

const MODEL_ALESIS_IO_BOTH: u32 = 0x000001;

unsafe extern "C" fn check_dice_category(unit: *mut fw_unit) -> c_int {
    let device: *mut fw_device = fw_parent_device(unit);
    let mut it: fw_csr_iterator = core::mem::zeroed();
    let mut key: c_int = 0;
    let mut val: c_int = 0;
    let mut vendor: c_int = -1;
    let mut model: c_int = -1;
    let category: c_uint;

    /*
     * Check that GUID and unit directory are constructed according to DICE
     * rules, i.e., that the specifier ID is the GUID's OUI, and that the
     * GUID chip ID consists of the 8-bit category ID, the 10-bit product
     * ID, and a 22-bit serial number.
     */
    fw_csr_iterator_init(&mut it, (*unit).directory);
    while fw_csr_iterator_next(&mut it, &mut key, &mut val) != 0 {
        match key {
            CSR_SPECIFIER_ID => {
                vendor = val;
            }
            CSR_MODEL => {
                model = val;
            }
            _ => {}
        }
    }

    if vendor == OUI_WEISS as c_int {
        category = WEISS_CATEGORY_ID;
    } else if vendor == OUI_LOUD as c_int {
        category = LOUD_CATEGORY_ID;
    } else if vendor == OUI_HARMAN as c_int {
        category = HARMAN_CATEGORY_ID;
    } else {
        category = DICE_CATEGORY_ID;
    }
    if *(*device).config_rom.add(3) != (((vendor as u32) << 8) | category)
        || (*(*device).config_rom.add(4) >> 22) != model as u32
    {
        return -ENODEV;
    }

    0
}

unsafe extern "C" fn check_clock_caps(dice: *mut snd_dice) -> c_int {
    let mut value: __be32 = core::mem::zeroed();
    let mut err: c_int;

    /* some very old firmwares don't tell about their clock support */
    if (*dice).clock_caps > 0 {
        err = snd_dice_transaction_read_global(
            dice,
            GLOBAL_CLOCK_CAPABILITIES,
            &mut value as *mut __be32 as *mut c_void,
            4,
        );
        if err < 0 {
            return err;
        }
        (*dice).clock_caps = be32_to_cpu(value);
    } else {
        /* this should be supported by any device */
        (*dice).clock_caps = CLOCK_CAP_RATE_44100
            | CLOCK_CAP_RATE_48000
            | CLOCK_CAP_SOURCE_ARX1
            | CLOCK_CAP_SOURCE_INTERNAL;
    }

    0
}

unsafe extern "C" fn dice_card_strings(dice: *mut snd_dice) {
    let card: *mut snd_card = (*dice).card;
    let dev: *mut fw_device = fw_parent_device((*dice).unit);
    let mut vendor: [c_char; 32] = [0; 32];
    let mut model: [c_char; 32] = [0; 32];
    let mut i: c_uint;
    let err: c_int;

    strscpy((*card).driver.as_mut_ptr(), c"DICE".as_ptr());

    strscpy((*card).shortname.as_mut_ptr(), c"DICE".as_ptr());
    // BUILD_BUG_ON(NICK_NAME_SIZE < sizeof(card->shortname));
    err = snd_dice_transaction_read_global(
        dice,
        GLOBAL_NICK_NAME,
        (*card).shortname.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&(*card).shortname),
    );
    if err >= 0 {
        /* DICE strings are returned in "always-wrong" endianness */
        // BUILD_BUG_ON(sizeof(card->shortname) % 4 != 0);
        i = 0;
        while (i as usize) < core::mem::size_of_val(&(*card).shortname) {
            swab32s((*card).shortname.as_mut_ptr().add(i as usize) as *mut u32);
            i += 4;
        }
        (*card).shortname[core::mem::size_of_val(&(*card).shortname) - 1] = b'\0' as c_char;
    }

    strscpy(vendor.as_mut_ptr(), c"?".as_ptr());
    fw_csr_string(
        (*dev).config_rom.add(5),
        CSR_VENDOR,
        vendor.as_mut_ptr(),
        core::mem::size_of_val(&vendor),
    );
    strscpy(model.as_mut_ptr(), c"?".as_ptr());
    fw_csr_string(
        (*(*dice).unit).directory,
        CSR_MODEL,
        model.as_mut_ptr(),
        core::mem::size_of_val(&model),
    );
    scnprintf(
        (*card).longname.as_mut_ptr(),
        core::mem::size_of_val(&(*card).longname),
        c"%s %s (serial %u) at %s, S%d".as_ptr(),
        vendor.as_ptr(),
        model.as_ptr(),
        *(*dev).config_rom.add(4) & 0x3fffff,
        dev_name(&mut (*(*dice).unit).device),
        100 << (*dev).max_speed,
    );

    strscpy((*card).mixername.as_mut_ptr(), c"DICE".as_ptr());
}

unsafe extern "C" fn dice_card_free(card: *mut snd_card) {
    let dice: *mut snd_dice = (*card).private_data as *mut snd_dice;

    snd_dice_stream_destroy_duplex(dice);
    snd_dice_transaction_destroy(dice);

    mutex_destroy(&mut (*dice).mutex);
    fw_unit_put((*dice).unit);
}

unsafe extern "C" fn dice_probe(
    unit: *mut fw_unit,
    entry: *const ieee1394_device_id,
) -> c_int {
    let mut card: *mut snd_card = core::ptr::null_mut();
    let dice: *mut snd_dice;
    let detect_formats: snd_dice_detect_formats_t;
    let mut err: c_int;

    if (*entry).driver_data_ptr.is_null() && (*entry).vendor_id != OUI_SSL {
        err = check_dice_category(unit);
        if err < 0 {
            return -ENODEV;
        }
    }

    err = snd_card_new(
        &mut (*unit).device,
        -1,
        core::ptr::null(),
        THIS_MODULE,
        core::mem::size_of::<snd_dice>(),
        &mut card,
    );
    if err < 0 {
        return err;
    }
    (*card).private_free = Some(dice_card_free);

    dice = (*card).private_data as *mut snd_dice;
    (*dice).unit = fw_unit_get(unit);
    dev_set_drvdata(&mut (*unit).device, dice as *mut c_void);
    (*dice).card = card;

    if (*entry).driver_data_ptr.is_null() {
        detect_formats = snd_dice_stream_detect_current_formats;
    } else {
        detect_formats = core::mem::transmute((*entry).driver_data_ptr);
    }

    // Below models are compliant to IEC 61883-1/6 and have no quirk at high sampling transfer
    // frequency.
    // * Avid M-Box 3 Pro
    // * M-Audio Profire 610
    // * M-Audio Profire 2626
    if (*entry).vendor_id == OUI_MAUDIO || (*entry).vendor_id == OUI_AVID {
        (*dice).disable_double_pcm_frames = true;
    }

    spin_lock_init(&mut (*dice).lock);
    mutex_init(&mut (*dice).mutex);
    init_completion(&mut (*dice).clock_accepted);
    init_waitqueue_head(&mut (*dice).hwdep_wait);

    err = snd_dice_transaction_init(dice);
    if err < 0 {
        goto_error(card, err)
    } else {
        err = check_clock_caps(dice);
        if err < 0 {
            goto_error(card, err)
        } else {
            dice_card_strings(dice);

            err = detect_formats(dice);
            if err < 0 {
                goto_error(card, err)
            } else {
                err = snd_dice_stream_init_duplex(dice);
                if err < 0 {
                    goto_error(card, err)
                } else {
                    snd_dice_create_proc(dice);

                    err = snd_dice_create_pcm(dice);
                    if err < 0 {
                        goto_error(card, err)
                    } else {
                        err = snd_dice_create_midi(dice);
                        if err < 0 {
                            goto_error(card, err)
                        } else {
                            err = snd_dice_create_hwdep(dice);
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
}

unsafe fn goto_error(card: *mut snd_card, err: c_int) -> c_int {
    snd_card_free(card);
    err
}

unsafe extern "C" fn dice_remove(unit: *mut fw_unit) {
    let dice: *mut snd_dice = dev_get_drvdata(&mut (*unit).device) as *mut snd_dice;

    // Block till all of ALSA character devices are released.
    snd_card_free((*dice).card);
}

unsafe extern "C" fn dice_bus_reset(unit: *mut fw_unit) {
    let dice: *mut snd_dice = dev_get_drvdata(&mut (*unit).device) as *mut snd_dice;

    /* The handler address register becomes initialized. */
    snd_dice_transaction_reinit(dice);

    // C used guard(mutex)(&dice->mutex), holding the mutex through this call.
    let _guard = guard_mutex(&mut (*dice).mutex);
    snd_dice_stream_update_duplex(dice);
}

const DICE_INTERFACE: u32 = 0x000001;

macro_rules! DICE_DEV_ENTRY_TYPICAL {
    ($vendor:expr, $model:expr, $data:expr) => {
        ieee1394_device_id {
            match_flags: IEEE1394_MATCH_VENDOR_ID
                | IEEE1394_MATCH_MODEL_ID
                | IEEE1394_MATCH_SPECIFIER_ID
                | IEEE1394_MATCH_VERSION,
            vendor_id: $vendor,
            model_id: $model,
            specifier_id: $vendor,
            version: DICE_INTERFACE,
            driver_data_ptr: $data as *const c_void,
            ..unsafe { core::mem::zeroed() }
        }
    };
}

static dice_id_table: [ieee1394_device_id; 31] = [
    // Avid M-Box 3 Pro. To match in probe function.
    DICE_DEV_ENTRY_TYPICAL!(OUI_AVID, 0x000004, snd_dice_detect_extension_formats),
    /* M-Audio Profire 2626 has a different value in version field. */
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_MAUDIO,
        model_id: 0x000010,
        driver_data_ptr: snd_dice_detect_extension_formats as *const c_void,
        ..unsafe { core::mem::zeroed() }
    },
    /* M-Audio Profire 610 has a different value in version field. */
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_MAUDIO,
        model_id: 0x000011,
        driver_data_ptr: snd_dice_detect_extension_formats as *const c_void,
        ..unsafe { core::mem::zeroed() }
    },
    /* TC Electronic Konnekt 24D. */
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_TCELECTRONIC,
        model_id: 0x000020,
        driver_data_ptr: snd_dice_detect_tcelectronic_formats as *const c_void,
        ..unsafe { core::mem::zeroed() }
    },
    /* TC Electronic Konnekt 8. */
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_TCELECTRONIC,
        model_id: 0x000021,
        driver_data_ptr: snd_dice_detect_tcelectronic_formats as *const c_void,
        ..unsafe { core::mem::zeroed() }
    },
    /* TC Electronic Studio Konnekt 48. */
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_TCELECTRONIC,
        model_id: 0x000022,
        driver_data_ptr: snd_dice_detect_tcelectronic_formats as *const c_void,
        ..unsafe { core::mem::zeroed() }
    },
    /* TC Electronic Konnekt Live. */
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_TCELECTRONIC,
        model_id: 0x000023,
        driver_data_ptr: snd_dice_detect_tcelectronic_formats as *const c_void,
        ..unsafe { core::mem::zeroed() }
    },
    /* TC Electronic Desktop Konnekt 6. */
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_TCELECTRONIC,
        model_id: 0x000024,
        driver_data_ptr: snd_dice_detect_tcelectronic_formats as *const c_void,
        ..unsafe { core::mem::zeroed() }
    },
    /* TC Electronic Impact Twin. */
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_TCELECTRONIC,
        model_id: 0x000027,
        driver_data_ptr: snd_dice_detect_tcelectronic_formats as *const c_void,
        ..unsafe { core::mem::zeroed() }
    },
    /* TC Electronic Digital Konnekt x32. */
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_TCELECTRONIC,
        model_id: 0x000030,
        driver_data_ptr: snd_dice_detect_tcelectronic_formats as *const c_void,
        ..unsafe { core::mem::zeroed() }
    },
    /* Alesis iO14/iO26. */
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_ALESIS,
        model_id: MODEL_ALESIS_IO_BOTH,
        driver_data_ptr: snd_dice_detect_alesis_formats as *const c_void,
        ..unsafe { core::mem::zeroed() }
    },
    // Alesis MasterControl.
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_ALESIS,
        model_id: 0x000002,
        driver_data_ptr: snd_dice_detect_alesis_mastercontrol_formats as *const c_void,
        ..unsafe { core::mem::zeroed() }
    },
    /* Mytek Stereo 192 DSD-DAC. */
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_MYTEK,
        model_id: 0x000002,
        driver_data_ptr: snd_dice_detect_mytek_formats as *const c_void,
        ..unsafe { core::mem::zeroed() }
    },
    // Solid State Logic, Duende Classic and Mini.
    // NOTE: each field of GUID in config ROM is not compliant to standard
    // DICE scheme.
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_SSL,
        model_id: 0x000070,
        ..unsafe { core::mem::zeroed() }
    },
    // Presonus FireStudio.
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_PRESONUS,
        model_id: 0x000008,
        driver_data: snd_dice_detect_presonus_formats as kernel_ulong_t,
        ..unsafe { core::mem::zeroed() }
    },
    // Lexicon I-ONYX FW810S.
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_HARMAN,
        model_id: 0x000001,
        driver_data: snd_dice_detect_harman_formats as kernel_ulong_t,
        ..unsafe { core::mem::zeroed() }
    },
    // Focusrite Saffire Pro 40 with TCD3070-CH.
    // The model has quirk in its GUID, in which model field is 0x000013 and different from
    // model ID (0x0000de) in its root/unit directory.
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_FOCUSRITE,
        model_id: 0x0000de,
        driver_data: snd_dice_detect_focusrite_pro40_tcd3070_formats as kernel_ulong_t,
        ..unsafe { core::mem::zeroed() }
    },
    // Weiss DAC202: 192kHz 2-channel DAC
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_WEISS,
        model_id: 0x000007,
        driver_data: snd_dice_detect_weiss_formats as kernel_ulong_t,
        ..unsafe { core::mem::zeroed() }
    },
    // Weiss DAC202: 192kHz 2-channel DAC (Maya edition)
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_WEISS,
        model_id: 0x000008,
        driver_data: snd_dice_detect_weiss_formats as kernel_ulong_t,
        ..unsafe { core::mem::zeroed() }
    },
    // Weiss MAN301: 192kHz 2-channel music archive network player
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_WEISS,
        model_id: 0x00000b,
        driver_data: snd_dice_detect_weiss_formats as kernel_ulong_t,
        ..unsafe { core::mem::zeroed() }
    },
    // Weiss INT202: 192kHz unidirectional 2-channel digital Firewire face
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_WEISS,
        model_id: 0x000006,
        driver_data: snd_dice_detect_weiss_formats as kernel_ulong_t,
        ..unsafe { core::mem::zeroed() }
    },
    // Weiss INT203: 192kHz bidirectional 2-channel digital Firewire face
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_WEISS,
        model_id: 0x00000a,
        driver_data: snd_dice_detect_weiss_formats as kernel_ulong_t,
        ..unsafe { core::mem::zeroed() }
    },
    // Weiss ADC2: 192kHz A/D converter with microphone preamps and inputs
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_WEISS,
        model_id: 0x000001,
        driver_data: snd_dice_detect_weiss_formats as kernel_ulong_t,
        ..unsafe { core::mem::zeroed() }
    },
    // Weiss DAC2/Minerva: 192kHz 2-channel DAC
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_WEISS,
        model_id: 0x000003,
        driver_data: snd_dice_detect_weiss_formats as kernel_ulong_t,
        ..unsafe { core::mem::zeroed() }
    },
    // Weiss Vesta: 192kHz 2-channel Firewire to AES/EBU interface
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_WEISS,
        model_id: 0x000002,
        driver_data: snd_dice_detect_weiss_formats as kernel_ulong_t,
        ..unsafe { core::mem::zeroed() }
    },
    // Weiss AFI1: 192kHz 24-channel Firewire to ADAT or AES/EBU face
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_WEISS,
        model_id: 0x000004,
        driver_data: snd_dice_detect_weiss_formats as kernel_ulong_t,
        ..unsafe { core::mem::zeroed() }
    },
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VERSION,
        version: DICE_INTERFACE,
        ..unsafe { core::mem::zeroed() }
    },
    // Tascam IF-FW/DM MkII for DM-3200 and DM-4800.
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID
            | IEEE1394_MATCH_MODEL_ID
            | IEEE1394_MATCH_SPECIFIER_ID
            | IEEE1394_MATCH_VERSION,
        vendor_id: OUI_TEAC,
        model_id: OUI_TEAC,
        specifier_id: OUI_TEAC,
        version: 0x800006,
        driver_data_ptr: snd_dice_detect_teac_formats as *const c_void,
        ..unsafe { core::mem::zeroed() }
    },
    ieee1394_device_id {
        ..unsafe { core::mem::zeroed() }
    },
];
// MODULE_DEVICE_TABLE(ieee1394, dice_id_table);

static mut dice_driver: fw_driver = fw_driver {
    driver: device_driver {
        owner: THIS_MODULE,
        name: KBUILD_MODNAME,
        bus: &fw_bus_type,
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(dice_probe),
    update: Some(dice_bus_reset),
    remove: Some(dice_remove),
    id_table: dice_id_table.as_ptr(),
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn alsa_dice_init() -> c_int {
    driver_register(&mut dice_driver.driver)
}

unsafe extern "C" fn alsa_dice_exit() {
    driver_unregister(&mut dice_driver.driver);
}

// module_init(alsa_dice_init);
// module_exit(alsa_dice_exit);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
