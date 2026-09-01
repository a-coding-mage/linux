// SPDX-License-Identifier: GPL-2.0-or-later
//
// Realtek ALC680 codec
//

// C dependencies:
// #include <linux/init.h>
// #include <linux/module.h>
// #include "realtek.h"

unsafe extern "C" fn alc680_parse_auto_config(codec: *mut hda_codec) -> core::ffi::c_int {
    unsafe { alc_parse_auto_config(codec, core::ptr::null(), core::ptr::null()) }
}

/*
 */
unsafe extern "C" fn alc680_probe(
    codec: *mut hda_codec,
    id: *const hda_device_id,
) -> core::ffi::c_int {
    let mut err: core::ffi::c_int;

    /* ALC680 has no aa-loopback mixer */
    err = unsafe { alc_alloc_spec(codec, 0) };
    if err < 0 {
        return err;
    }

    /* automatic parse from the BIOS config */
    err = unsafe { alc680_parse_auto_config(codec) };
    if err < 0 {
        unsafe { snd_hda_gen_remove(codec) };
        return err;
    }

    0
}

static alc680_codec_ops: hda_codec_ops = hda_codec_ops {
    probe: Some(alc680_probe),
    remove: Some(snd_hda_gen_remove),
    build_controls: Some(alc_build_controls),
    build_pcms: Some(snd_hda_gen_build_pcms),
    init: Some(alc_init),
    unsol_event: Some(snd_hda_jack_unsol_event),
    resume: Some(alc_resume),
    suspend: Some(alc_suspend),
    check_power_status: Some(snd_hda_gen_check_power_status),
    stream_pm: Some(snd_hda_gen_stream_pm),
};

/*
 * driver entries
 */
static snd_hda_id_alc680: [hda_device_id; 2] = [
    HDA_CODEC_ID!(0x10ec0680, "ALC680"),
    hda_device_id {}, /* terminator */
];

module_device_table!(hdaudio, snd_hda_id_alc680);

module_license!("GPL");
module_description!("Realtek ALC680 HD-audio codec");
module_import_ns!("SND_HDA_CODEC_REALTEK");

static mut alc680_driver: hda_codec_driver = hda_codec_driver {
    id: snd_hda_id_alc680.as_ptr(),
    ops: &alc680_codec_ops,
};

module_hda_codec_driver!(alc680_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
