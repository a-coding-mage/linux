// SPDX-License-Identifier: GPL-2.0-only
/*
 * fireworks.c - a part of driver for Fireworks based devices
 *
 * Copyright (c) 2009-2010 Clemens Ladisch
 * Copyright (c) 2013-2014 Takashi Sakamoto
 */

/*
 * Fireworks is a board module which Echo Audio produced. This module consists
 * of three chipsets:
 *  - Communication chipset for IEEE1394 PHY/Link and IEC 61883-1/6
 *  - DSP or/and FPGA for signal processing
 *  - Flash Memory to store firmwares
 */

// C dependency: "fireworks.h"

// MODULE_DESCRIPTION("Echo Fireworks driver");
// MODULE_AUTHOR("Takashi Sakamoto <o-takashi@sakamocchi.jp>");
// MODULE_LICENSE("GPL");

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX;
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR;
static mut enable: [bool; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP;
pub static mut snd_efw_resp_buf_size: c_uint = 1024;
pub static mut snd_efw_resp_buf_debug: bool = false;

// module_param_array(index, int, NULL, 0444);
// MODULE_PARM_DESC(index, "card index");
// module_param_array(id, charp, NULL, 0444);
// MODULE_PARM_DESC(id, "ID string");
// module_param_array(enable, bool, NULL, 0444);
// MODULE_PARM_DESC(enable, "enable Fireworks sound card");
// module_param_named(resp_buf_size, snd_efw_resp_buf_size, uint, 0444);
// MODULE_PARM_DESC(resp_buf_size,
//                  "response buffer size (max 4096, default 1024)");
// module_param_named(resp_buf_debug, snd_efw_resp_buf_debug, bool, 0444);
// MODULE_PARM_DESC(resp_buf_debug, "store all responses to buffer");

static mut devices_mutex: mutex = DEFINE_MUTEX();
static mut devices_used: [c_ulong; BITS_TO_LONGS(SNDRV_CARDS)] = DECLARE_BITMAP();

const VENDOR_LOUD: c_uint = 0x000ff2;
const MODEL_MACKIE_400F: c_uint = 0x00400f;
const MODEL_MACKIE_1200F: c_uint = 0x01200f;

const VENDOR_ECHO: c_uint = 0x001486;
const MODEL_ECHO_AUDIOFIRE_12: c_uint = 0x00af12;
const MODEL_ECHO_AUDIOFIRE_12HD: c_uint = 0x0af12d;
const MODEL_ECHO_AUDIOFIRE_12_APPLE: c_uint = 0x0af12a;
/* This is applied for AudioFire8 (until 2009 July) */
const MODEL_ECHO_AUDIOFIRE_8: c_uint = 0x000af8;
const MODEL_ECHO_AUDIOFIRE_2: c_uint = 0x000af2;
const MODEL_ECHO_AUDIOFIRE_4: c_uint = 0x000af4;
/* AudioFire9 is applied for AudioFire8(since 2009 July) and AudioFirePre8 */
const MODEL_ECHO_AUDIOFIRE_9: c_uint = 0x000af9;
/* unknown as product */
const MODEL_ECHO_FIREWORKS_8: c_uint = 0x0000f8;
const MODEL_ECHO_FIREWORKS_HDMI: c_uint = 0x00afd1;

const VENDOR_GIBSON: c_uint = 0x00075b;
/* for Robot Interface Pack of Dark Fire, Dusk Tiger, Les Paul Standard 2010 */
const MODEL_GIBSON_RIP: c_uint = 0x00afb2;
/* unknown as product */
const MODEL_GIBSON_GOLDTOP: c_uint = 0x00afb9;

/* part of hardware capability flags */
const FLAG_RESP_ADDR_CHANGABLE: c_uint = 0;

unsafe fn get_hardware_info(efw: *mut snd_efw) -> c_int {
    let fw_dev: *mut fw_device = fw_parent_device((*efw).unit);
    let mut hwinfo: *mut snd_efw_hwinfo;
    let mut version: [c_char; 12] = [0; 12];
    let mut err: c_int;

    hwinfo = kzalloc_obj::<snd_efw_hwinfo>();
    if hwinfo.is_null() {
        return -ENOMEM;
    }

    err = snd_efw_command_get_hwinfo(efw, hwinfo);
    if err < 0 {
        goto_end(efw, hwinfo, err);
        return err;
    }

    /* firmware version for communication chipset */
    snprintf(
        version.as_mut_ptr(),
        version.len(),
        c"%u.%u".as_ptr(),
        ((*hwinfo).arm_version >> 24) & 0xff,
        ((*hwinfo).arm_version >> 16) & 0xff,
    );
    (*efw).firmware_version = (*hwinfo).arm_version;

    strscpy((*(*efw).card).driver.as_mut_ptr(), c"Fireworks".as_ptr());
    strscpy((*(*efw).card).shortname.as_mut_ptr(), (*hwinfo).model_name.as_ptr());
    strscpy((*(*efw).card).mixername.as_mut_ptr(), (*hwinfo).model_name.as_ptr());
    scnprintf(
        (*(*efw).card).longname.as_mut_ptr(),
        core::mem::size_of_val(&(*(*efw).card).longname),
        c"%s %s v%s, GUID %08x%08x at %s, S%d".as_ptr(),
        (*hwinfo).vendor_name.as_ptr(),
        (*hwinfo).model_name.as_ptr(),
        version.as_ptr(),
        (*hwinfo).guid_hi,
        (*hwinfo).guid_lo,
        dev_name(&mut (*(*efw).unit).device),
        100 << (*fw_dev).max_speed,
    );

    if ((*hwinfo).flags & BIT(FLAG_RESP_ADDR_CHANGABLE)) != 0 {
        (*efw).resp_addr_changable = true;
    }

    (*efw).supported_sampling_rate = 0;
    if (*hwinfo).min_sample_rate <= 22050 && 22050 <= (*hwinfo).max_sample_rate {
        (*efw).supported_sampling_rate |= SNDRV_PCM_RATE_22050;
    }
    if (*hwinfo).min_sample_rate <= 32000 && 32000 <= (*hwinfo).max_sample_rate {
        (*efw).supported_sampling_rate |= SNDRV_PCM_RATE_32000;
    }
    if (*hwinfo).min_sample_rate <= 44100 && 44100 <= (*hwinfo).max_sample_rate {
        (*efw).supported_sampling_rate |= SNDRV_PCM_RATE_44100;
    }
    if (*hwinfo).min_sample_rate <= 48000 && 48000 <= (*hwinfo).max_sample_rate {
        (*efw).supported_sampling_rate |= SNDRV_PCM_RATE_48000;
    }
    if (*hwinfo).min_sample_rate <= 88200 && 88200 <= (*hwinfo).max_sample_rate {
        (*efw).supported_sampling_rate |= SNDRV_PCM_RATE_88200;
    }
    if (*hwinfo).min_sample_rate <= 96000 && 96000 <= (*hwinfo).max_sample_rate {
        (*efw).supported_sampling_rate |= SNDRV_PCM_RATE_96000;
    }
    if (*hwinfo).min_sample_rate <= 176400 && 176400 <= (*hwinfo).max_sample_rate {
        (*efw).supported_sampling_rate |= SNDRV_PCM_RATE_176400;
    }
    if (*hwinfo).min_sample_rate <= 192000 && 192000 <= (*hwinfo).max_sample_rate {
        (*efw).supported_sampling_rate |= SNDRV_PCM_RATE_192000;
    }

    /* the number of MIDI ports, not of MIDI conformant data channels */
    if (*hwinfo).midi_out_ports > SND_EFW_MAX_MIDI_OUT_PORTS
        || (*hwinfo).midi_in_ports > SND_EFW_MAX_MIDI_IN_PORTS
    {
        err = -EIO;
        goto_end(efw, hwinfo, err);
        return err;
    }
    (*efw).midi_out_ports = (*hwinfo).midi_out_ports;
    (*efw).midi_in_ports = (*hwinfo).midi_in_ports;

    if (*hwinfo).amdtp_tx_pcm_channels > AM824_MAX_CHANNELS_FOR_PCM
        || (*hwinfo).amdtp_tx_pcm_channels_2x > AM824_MAX_CHANNELS_FOR_PCM
        || (*hwinfo).amdtp_tx_pcm_channels_4x > AM824_MAX_CHANNELS_FOR_PCM
        || (*hwinfo).amdtp_rx_pcm_channels > AM824_MAX_CHANNELS_FOR_PCM
        || (*hwinfo).amdtp_rx_pcm_channels_2x > AM824_MAX_CHANNELS_FOR_PCM
        || (*hwinfo).amdtp_rx_pcm_channels_4x > AM824_MAX_CHANNELS_FOR_PCM
    {
        err = -ENOSYS;
        goto_end(efw, hwinfo, err);
        return err;
    }
    (*efw).pcm_capture_channels[0] = (*hwinfo).amdtp_tx_pcm_channels;
    (*efw).pcm_capture_channels[1] = (*hwinfo).amdtp_tx_pcm_channels_2x;
    (*efw).pcm_capture_channels[2] = (*hwinfo).amdtp_tx_pcm_channels_4x;
    (*efw).pcm_playback_channels[0] = (*hwinfo).amdtp_rx_pcm_channels;
    (*efw).pcm_playback_channels[1] = (*hwinfo).amdtp_rx_pcm_channels_2x;
    (*efw).pcm_playback_channels[2] = (*hwinfo).amdtp_rx_pcm_channels_4x;

    /* Hardware metering. */
    if (*hwinfo).phys_in_grp_count > HWINFO_MAX_CAPS_GROUPS
        || (*hwinfo).phys_out_grp_count > HWINFO_MAX_CAPS_GROUPS
    {
        err = -EIO;
        goto_end(efw, hwinfo, err);
        return err;
    }
    (*efw).phys_in = (*hwinfo).phys_in;
    (*efw).phys_out = (*hwinfo).phys_out;
    (*efw).phys_in_grp_count = (*hwinfo).phys_in_grp_count;
    (*efw).phys_out_grp_count = (*hwinfo).phys_out_grp_count;
    core::ptr::copy_nonoverlapping(
        (*hwinfo).phys_in_grps.as_ptr(),
        (*efw).phys_in_grps.as_mut_ptr(),
        (*hwinfo).phys_in_grp_count as usize,
    );
    core::ptr::copy_nonoverlapping(
        (*hwinfo).phys_out_grps.as_ptr(),
        (*efw).phys_out_grps.as_mut_ptr(),
        (*hwinfo).phys_out_grp_count as usize,
    );

    /* AudioFire8 (since 2009) and AudioFirePre8 */
    if (*hwinfo).type_ == MODEL_ECHO_AUDIOFIRE_9 {
        (*efw).is_af9 = true;
    }
    /* These models uses the same firmware. */
    if (*hwinfo).type_ == MODEL_ECHO_AUDIOFIRE_2
        || (*hwinfo).type_ == MODEL_ECHO_AUDIOFIRE_4
        || (*hwinfo).type_ == MODEL_ECHO_AUDIOFIRE_9
        || (*hwinfo).type_ == MODEL_GIBSON_RIP
        || (*hwinfo).type_ == MODEL_GIBSON_GOLDTOP
    {
        (*efw).is_fireworks3 = true;
    }

    kfree(hwinfo as *mut c_void);
    err
}

unsafe fn goto_end(_efw: *mut snd_efw, hwinfo: *mut snd_efw_hwinfo, _err: c_int) {
    kfree(hwinfo as *mut c_void);
}

unsafe extern "C" fn efw_card_free(card: *mut snd_card) {
    let efw: *mut snd_efw = (*card).private_data as *mut snd_efw;

    mutex_lock(&mut devices_mutex);
    clear_bit((*efw).card_index, devices_used.as_mut_ptr());
    mutex_unlock(&mut devices_mutex);

    snd_efw_stream_destroy_duplex(efw);
    snd_efw_transaction_remove_instance(efw);

    mutex_destroy(&mut (*efw).mutex);
    fw_unit_put((*efw).unit);
}

unsafe extern "C" fn efw_probe(
    unit: *mut fw_unit,
    entry: *const ieee1394_device_id,
) -> c_int {
    let mut card_index: c_uint;
    let mut card: *mut snd_card = core::ptr::null_mut();
    let mut efw: *mut snd_efw;
    let mut err: c_int;

    // check registered cards.
    mutex_lock(&mut devices_mutex);
    card_index = 0;
    while card_index < SNDRV_CARDS as c_uint {
        if !test_bit(card_index, devices_used.as_ptr()) && enable[card_index as usize] {
            break;
        }
        card_index += 1;
    }
    if card_index >= SNDRV_CARDS as c_uint {
        mutex_unlock(&mut devices_mutex);
        return -ENOENT;
    }

    err = snd_card_new(
        &mut (*unit).device,
        index[card_index as usize],
        id[card_index as usize],
        THIS_MODULE,
        core::mem::size_of::<snd_efw>(),
        &mut card,
    );
    if err < 0 {
        mutex_unlock(&mut devices_mutex);
        return err;
    }
    (*card).private_free = Some(efw_card_free);
    set_bit(card_index, devices_used.as_mut_ptr());
    mutex_unlock(&mut devices_mutex);

    efw = (*card).private_data as *mut snd_efw;
    (*efw).unit = fw_unit_get(unit);
    dev_set_drvdata(&mut (*unit).device, efw as *mut c_void);
    (*efw).card = card;
    (*efw).card_index = card_index;

    mutex_init(&mut (*efw).mutex);
    spin_lock_init(&mut (*efw).lock);
    init_waitqueue_head(&mut (*efw).hwdep_wait);

    // prepare response buffer.
    snd_efw_resp_buf_size = clamp(
        snd_efw_resp_buf_size,
        SND_EFW_RESPONSE_MAXIMUM_BYTES,
        4096u32,
    );
    (*efw).resp_buf = devm_kzalloc(
        &mut (*card).card_dev,
        snd_efw_resp_buf_size as usize,
        GFP_KERNEL,
    );
    if (*efw).resp_buf.is_null() {
        err = -ENOMEM;
        snd_card_free(card);
        return err;
    }
    (*efw).push_ptr = (*efw).resp_buf;
    (*efw).pull_ptr = (*efw).push_ptr;
    snd_efw_transaction_add_instance(efw);

    err = get_hardware_info(efw);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    err = snd_efw_stream_init_duplex(efw);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    snd_efw_proc_init(efw);

    if (*efw).midi_out_ports != 0 || (*efw).midi_in_ports != 0 {
        err = snd_efw_create_midi_devices(efw);
        if err < 0 {
            snd_card_free(card);
            return err;
        }
    }

    err = snd_efw_create_pcm_devices(efw);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    err = snd_efw_create_hwdep_device(efw);
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

unsafe extern "C" fn efw_update(unit: *mut fw_unit) {
    let efw: *mut snd_efw = dev_get_drvdata(&mut (*unit).device) as *mut snd_efw;

    snd_efw_transaction_bus_reset((*efw).unit);

    mutex_lock(&mut (*efw).mutex);
    snd_efw_stream_update_duplex(efw);
    mutex_unlock(&mut (*efw).mutex);
}

unsafe extern "C" fn efw_remove(unit: *mut fw_unit) {
    let efw: *mut snd_efw = dev_get_drvdata(&mut (*unit).device) as *mut snd_efw;

    // Block till all of ALSA character devices are released.
    snd_card_free((*efw).card);
}

const SPECIFIER_1394TA: c_uint = 0x00a02d;
const VERSION_EFW: c_uint = 0x010000;

const fn SND_EFW_DEV_ENTRY(vendor: c_uint, model: c_uint) -> ieee1394_device_id {
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID
            | IEEE1394_MATCH_MODEL_ID
            | IEEE1394_MATCH_SPECIFIER_ID
            | IEEE1394_MATCH_VERSION,
        vendor_id: vendor,
        model_id: model,
        specifier_id: SPECIFIER_1394TA,
        version: VERSION_EFW,
    }
}

static efw_id_table: [ieee1394_device_id; 14] = [
    SND_EFW_DEV_ENTRY(VENDOR_LOUD, MODEL_MACKIE_400F),
    SND_EFW_DEV_ENTRY(VENDOR_LOUD, MODEL_MACKIE_1200F),
    SND_EFW_DEV_ENTRY(VENDOR_ECHO, MODEL_ECHO_AUDIOFIRE_8),
    SND_EFW_DEV_ENTRY(VENDOR_ECHO, MODEL_ECHO_AUDIOFIRE_12),
    SND_EFW_DEV_ENTRY(VENDOR_ECHO, MODEL_ECHO_AUDIOFIRE_12HD),
    SND_EFW_DEV_ENTRY(VENDOR_ECHO, MODEL_ECHO_AUDIOFIRE_12_APPLE),
    SND_EFW_DEV_ENTRY(VENDOR_ECHO, MODEL_ECHO_AUDIOFIRE_2),
    SND_EFW_DEV_ENTRY(VENDOR_ECHO, MODEL_ECHO_AUDIOFIRE_4),
    SND_EFW_DEV_ENTRY(VENDOR_ECHO, MODEL_ECHO_AUDIOFIRE_9),
    SND_EFW_DEV_ENTRY(VENDOR_ECHO, MODEL_ECHO_FIREWORKS_8),
    SND_EFW_DEV_ENTRY(VENDOR_ECHO, MODEL_ECHO_FIREWORKS_HDMI),
    SND_EFW_DEV_ENTRY(VENDOR_GIBSON, MODEL_GIBSON_RIP),
    SND_EFW_DEV_ENTRY(VENDOR_GIBSON, MODEL_GIBSON_GOLDTOP),
    ieee1394_device_id::zeroed(),
];
// MODULE_DEVICE_TABLE(ieee1394, efw_id_table);

static mut efw_driver: fw_driver = fw_driver {
    driver: device_driver {
        owner: THIS_MODULE,
        name: KBUILD_MODNAME,
        bus: unsafe { &mut fw_bus_type },
    },
    probe: Some(efw_probe),
    update: Some(efw_update),
    remove: Some(efw_remove),
    id_table: efw_id_table.as_ptr(),
};

unsafe fn snd_efw_init() -> c_int {
    let mut err: c_int;

    err = snd_efw_transaction_register();
    if err < 0 {
        return err;
    }

    err = driver_register(&mut efw_driver.driver);
    if err < 0 {
        snd_efw_transaction_unregister();
    }

    err
}

unsafe fn snd_efw_exit() {
    snd_efw_transaction_unregister();
    driver_unregister(&mut efw_driver.driver);
}

// module_init(snd_efw_init);
// module_exit(snd_efw_exit);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
