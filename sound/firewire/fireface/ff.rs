// SPDX-License-Identifier: GPL-2.0-only
/*
 * ff.c - a part of driver for RME Fireface series
 *
 * Copyright (c) 2015-2017 Takashi Sakamoto
 */

// Rust translation of implementation source ./firewire/fireface/ff.c.
// C include dependency: "ff.h".

const OUI_RME: u32 = 0x000a35;

// MODULE_DESCRIPTION("RME Fireface series Driver");
// MODULE_AUTHOR("Takashi Sakamoto <o-takashi@sakamocchi.jp>");
// MODULE_LICENSE("GPL");

unsafe fn name_card(ff: *mut snd_ff) {
    let fw_dev: *mut fw_device = fw_parent_device((*ff).unit);
    static NAMES: [*const c_char; 5] = [
        b"Fireface800\0".as_ptr() as *const c_char,
        b"Fireface400\0".as_ptr() as *const c_char,
        b"FirefaceUFX\0".as_ptr() as *const c_char,
        b"FirefaceUCX\0".as_ptr() as *const c_char,
        b"Fireface802\0".as_ptr() as *const c_char,
    ];
    let name: *const c_char;

    name = NAMES[(*ff).unit_version as usize];

    strscpy((*(*ff).card).driver.as_mut_ptr(), b"Fireface\0".as_ptr() as *const c_char);
    strscpy((*(*ff).card).shortname.as_mut_ptr(), name);
    strscpy((*(*ff).card).mixername.as_mut_ptr(), name);
    snprintf(
        (*(*ff).card).longname.as_mut_ptr(),
        core::mem::size_of_val(&(*(*ff).card).longname),
        b"RME %s, GUID %08x%08x at %s, S%d\0".as_ptr() as *const c_char,
        name,
        (*fw_dev).config_rom[3],
        (*fw_dev).config_rom[4],
        dev_name(&mut (*(*ff).unit).device),
        100 << (*fw_dev).max_speed,
    );
}

unsafe fn ff_card_free(card: *mut snd_card) {
    let ff: *mut snd_ff = (*card).private_data as *mut snd_ff;

    snd_ff_stream_destroy_duplex(ff);
    snd_ff_transaction_unregister(ff);

    kfree((*ff).msg_parser as *const c_void);

    mutex_destroy(&mut (*ff).mutex);
    fw_unit_put((*ff).unit);
}

unsafe fn snd_ff_probe(unit: *mut fw_unit, entry: *const ieee1394_device_id) -> c_int {
    let mut card: *mut snd_card = core::ptr::null_mut();
    let mut ff: *mut snd_ff;
    let mut err: c_int;

    err = snd_card_new(
        &mut (*unit).device,
        -1,
        core::ptr::null(),
        THIS_MODULE,
        core::mem::size_of::<snd_ff>(),
        &mut card,
    );
    if err < 0 {
        return err;
    }
    (*card).private_free = Some(ff_card_free);

    ff = (*card).private_data as *mut snd_ff;
    (*ff).unit = fw_unit_get(unit);
    dev_set_drvdata(&mut (*unit).device, ff as *mut c_void);
    (*ff).card = card;

    mutex_init(&mut (*ff).mutex);
    spin_lock_init(&mut (*ff).lock);
    init_waitqueue_head(&mut (*ff).hwdep_wait);

    (*ff).unit_version = (*entry).version;
    (*ff).spec = (*entry).driver_data_ptr as *const snd_ff_spec;

    err = snd_ff_transaction_register(ff);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    name_card(ff);

    err = snd_ff_stream_init_duplex(ff);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    snd_ff_proc_init(ff);

    err = snd_ff_create_midi_devices(ff);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    err = snd_ff_create_pcm_devices(ff);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    err = snd_ff_create_hwdep_devices(ff);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    if (*(*(*ff).spec).protocol).msg_parser_size > 0 {
        (*ff).msg_parser = kzalloc((*(*(*ff).spec).protocol).msg_parser_size, GFP_KERNEL);
        if (*ff).msg_parser.is_null() {
            err = -ENOMEM;
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

unsafe fn snd_ff_update(unit: *mut fw_unit) {
    let ff: *mut snd_ff = dev_get_drvdata(&mut (*unit).device) as *mut snd_ff;

    snd_ff_transaction_reregister(ff);

    snd_ff_stream_update_duplex(ff);
}

unsafe fn snd_ff_remove(unit: *mut fw_unit) {
    let ff: *mut snd_ff = dev_get_drvdata(&mut (*unit).device) as *mut snd_ff;

    // Block till all of ALSA character devices are released.
    snd_card_free((*ff).card);
}

static SPEC_FF800: snd_ff_spec = snd_ff_spec {
    pcm_capture_channels: [28, 20, 12],
    pcm_playback_channels: [28, 20, 12],
    midi_in_ports: 1,
    midi_out_ports: 1,
    protocol: unsafe { &snd_ff_protocol_ff800 },
    midi_high_addr: 0x000200000320u64,
    midi_addr_range: 12,
    midi_rx_addrs: [0x000080180000u64, 0],
};

static SPEC_FF400: snd_ff_spec = snd_ff_spec {
    pcm_capture_channels: [18, 14, 10],
    pcm_playback_channels: [18, 14, 10],
    midi_in_ports: 2,
    midi_out_ports: 2,
    protocol: unsafe { &snd_ff_protocol_ff400 },
    midi_high_addr: 0x0000801003f4u64,
    midi_addr_range: SND_FF_MAXIMIM_MIDI_QUADS * 4,
    midi_rx_addrs: [0x000080180000u64, 0x000080190000u64],
};

static SPEC_UCX: snd_ff_spec = snd_ff_spec {
    pcm_capture_channels: [18, 14, 12],
    pcm_playback_channels: [18, 14, 12],
    midi_in_ports: 2,
    midi_out_ports: 2,
    protocol: unsafe { &snd_ff_protocol_latter },
    midi_high_addr: 0xffff00000034u64,
    midi_addr_range: 0x80,
    midi_rx_addrs: [0xffff00000030u64, 0xffff00000030u64],
};

static SPEC_UFX_802: snd_ff_spec = snd_ff_spec {
    pcm_capture_channels: [30, 22, 14],
    pcm_playback_channels: [30, 22, 14],
    midi_in_ports: 1,
    midi_out_ports: 1,
    protocol: unsafe { &snd_ff_protocol_latter },
    midi_high_addr: 0xffff00000034u64,
    midi_addr_range: 0x80,
    midi_rx_addrs: [0xffff00000030u64, 0xffff00000030u64],
};

static SND_FF_ID_TABLE: [ieee1394_device_id; 6] = [
    /* Fireface 800 */
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID
            | IEEE1394_MATCH_SPECIFIER_ID
            | IEEE1394_MATCH_VERSION
            | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_RME,
        specifier_id: OUI_RME,
        version: SND_FF_UNIT_VERSION_FF800,
        model_id: 0x101800,
        driver_data_ptr: &SPEC_FF800 as *const snd_ff_spec as *const c_void,
    },
    /* Fireface 400 */
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID
            | IEEE1394_MATCH_SPECIFIER_ID
            | IEEE1394_MATCH_VERSION
            | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_RME,
        specifier_id: OUI_RME,
        version: SND_FF_UNIT_VERSION_FF400,
        model_id: 0x101800,
        driver_data_ptr: &SPEC_FF400 as *const snd_ff_spec as *const c_void,
    },
    // Fireface UFX.
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID
            | IEEE1394_MATCH_SPECIFIER_ID
            | IEEE1394_MATCH_VERSION
            | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_RME,
        specifier_id: OUI_RME,
        version: SND_FF_UNIT_VERSION_UFX,
        model_id: 0x101800,
        driver_data_ptr: &SPEC_UFX_802 as *const snd_ff_spec as *const c_void,
    },
    // Fireface UCX.
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID
            | IEEE1394_MATCH_SPECIFIER_ID
            | IEEE1394_MATCH_VERSION
            | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_RME,
        specifier_id: OUI_RME,
        version: SND_FF_UNIT_VERSION_UCX,
        model_id: 0x101800,
        driver_data_ptr: &SPEC_UCX as *const snd_ff_spec as *const c_void,
    },
    // Fireface 802.
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID
            | IEEE1394_MATCH_SPECIFIER_ID
            | IEEE1394_MATCH_VERSION
            | IEEE1394_MATCH_MODEL_ID,
        vendor_id: OUI_RME,
        specifier_id: OUI_RME,
        version: SND_FF_UNIT_VERSION_802,
        model_id: 0x101800,
        driver_data_ptr: &SPEC_UFX_802 as *const snd_ff_spec as *const c_void,
    },
    ieee1394_device_id::default(),
];

// MODULE_DEVICE_TABLE(ieee1394, snd_ff_id_table);

static mut FF_DRIVER: fw_driver = fw_driver {
    driver: device_driver {
        owner: THIS_MODULE,
        name: KBUILD_MODNAME,
        bus: unsafe { &fw_bus_type },
    },
    probe: Some(snd_ff_probe),
    update: Some(snd_ff_update),
    remove: Some(snd_ff_remove),
    id_table: SND_FF_ID_TABLE.as_ptr(),
};

unsafe fn snd_ff_init() -> c_int {
    driver_register(&mut FF_DRIVER.driver)
}

unsafe fn snd_ff_exit() {
    driver_unregister(&mut FF_DRIVER.driver);
}

// module_init(snd_ff_init);
// module_exit(snd_ff_exit);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
