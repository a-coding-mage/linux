// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Nvidia HDMI codec support
 */

// C includes removed; expected external dependencies:
// linux/init.h, linux/slab.h, linux/module.h, sound/core.h, sound/tlv.h,
// sound/hdaudio.h, sound/hda_codec.h, hda_local.h, hdmi_local.h.

const MODEL_GENERIC: i32 = 0;
const MODEL_LEGACY: i32 = 1;

/*
 * NVIDIA codecs ignore ASP mapping for 2ch - confirmed on:
 * - 0x10de0015
 * - 0x10de0040
 */
unsafe extern "C" fn nvhdmi_chmap_cea_alloc_validate_get_type(
    chmap: *mut hdac_chmap,
    cap: *mut hdac_cea_channel_speaker_allocation,
    channels: i32,
) -> i32 {
    if (*cap).ca_index == 0x00 && channels == 2 {
        return SNDRV_CTL_TLVT_CHMAP_FIXED;
    }

    /* If the speaker allocation matches the channel count, it is OK. */
    if (*cap).channels != channels {
        return -1;
    }

    /* all channels are remappable freely */
    SNDRV_CTL_TLVT_CHMAP_VAR
}

unsafe extern "C" fn nvhdmi_chmap_validate(
    chmap: *mut hdac_chmap,
    ca: i32,
    chs: i32,
    map: *mut u8,
) -> i32 {
    if ca == 0x00
        && (*map.add(0) != SNDRV_CHMAP_FL as u8 || *map.add(1) != SNDRV_CHMAP_FR as u8)
    {
        return -EINVAL;
    }

    0
}

/* map from pin NID to port; port is 0-based */
/* for Nvidia: assume widget NID starting from 4, with step 1 (4, 5, 6, ...) */
unsafe extern "C" fn nvhdmi_pin2port(audio_ptr: *mut core::ffi::c_void, pin_nid: i32) -> i32 {
    pin_nid - 4
}

/* reverse-map from port to pin NID: see above */
unsafe extern "C" fn nvhdmi_port2pin(codec: *mut hda_codec, port: i32) -> i32 {
    port + 4
}

static nvhdmi_audio_ops: drm_audio_component_audio_ops = drm_audio_component_audio_ops {
    pin2port: Some(nvhdmi_pin2port),
    pin_eld_notify: Some(snd_hda_hdmi_acomp_pin_eld_notify),
    master_bind: Some(snd_hda_hdmi_acomp_master_bind),
    master_unbind: Some(snd_hda_hdmi_acomp_master_unbind),
};

unsafe extern "C" fn probe_generic(codec: *mut hda_codec) -> i32 {
    let spec: *mut hdmi_spec;
    let mut err: i32;

    err = snd_hda_hdmi_generic_alloc(codec);
    if err < 0 {
        return err;
    }
    (*codec).dp_mst = true;

    spec = (*codec).spec as *mut hdmi_spec;

    err = snd_hda_hdmi_parse_codec(codec);
    if err < 0 {
        snd_hda_hdmi_generic_spec_free(codec);
        return err;
    }

    snd_hda_hdmi_generic_init_per_pins(codec);

    (*spec).dyn_pin_out = true;

    (*spec)
        .chmap
        .ops
        .chmap_cea_alloc_validate_get_type = Some(nvhdmi_chmap_cea_alloc_validate_get_type);
    (*spec).chmap.ops.chmap_validate = Some(nvhdmi_chmap_validate);
    (*spec).nv_dp_workaround = true;

    (*codec).link_down_at_suspend = 1;

    snd_hda_hdmi_acomp_init(
        codec,
        &nvhdmi_audio_ops,
        Some(nvhdmi_port2pin),
    );

    0
}

unsafe extern "C" fn probe_legacy(codec: *mut hda_codec) -> i32 {
    let spec: *mut hdmi_spec;
    let err: i32;

    err = snd_hda_hdmi_generic_probe(codec);
    if err != 0 {
        return err;
    }

    spec = (*codec).spec as *mut hdmi_spec;
    (*spec).dyn_pin_out = true;

    (*spec)
        .chmap
        .ops
        .chmap_cea_alloc_validate_get_type = Some(nvhdmi_chmap_cea_alloc_validate_get_type);
    (*spec).chmap.ops.chmap_validate = Some(nvhdmi_chmap_validate);
    (*spec).nv_dp_workaround = true;

    (*codec).link_down_at_suspend = 1;

    0
}

unsafe extern "C" fn nvhdmi_probe(codec: *mut hda_codec, id: *const hda_device_id) -> i32 {
    if (*id).driver_data == MODEL_LEGACY as kernel_ulong_t {
        probe_legacy(codec)
    } else {
        probe_generic(codec)
    }
}

static nvhdmi_codec_ops: hda_codec_ops = hda_codec_ops {
    probe: Some(nvhdmi_probe),
    remove: Some(snd_hda_hdmi_generic_remove),
    init: Some(snd_hda_hdmi_generic_init),
    build_pcms: Some(snd_hda_hdmi_generic_build_pcms),
    build_controls: Some(snd_hda_hdmi_generic_build_controls),
    unsol_event: Some(snd_hda_hdmi_generic_unsol_event),
    suspend: Some(snd_hda_hdmi_generic_suspend),
    resume: Some(snd_hda_hdmi_generic_resume),
};

static snd_hda_id_nvhdmi: [hda_device_id; 84] = [
    HDA_CODEC_ID_MODEL!(0x10de0008, "GPU 08 HDMI/DP", MODEL_LEGACY),
    HDA_CODEC_ID_MODEL!(0x10de0009, "GPU 09 HDMI/DP", MODEL_LEGACY),
    HDA_CODEC_ID_MODEL!(0x10de000a, "GPU 0a HDMI/DP", MODEL_LEGACY),
    HDA_CODEC_ID_MODEL!(0x10de000b, "GPU 0b HDMI/DP", MODEL_LEGACY),
    HDA_CODEC_ID_MODEL!(0x10de000c, "MCP89 HDMI", MODEL_LEGACY),
    HDA_CODEC_ID_MODEL!(0x10de000d, "GPU 0d HDMI/DP", MODEL_LEGACY),
    HDA_CODEC_ID_MODEL!(0x10de0010, "GPU 10 HDMI/DP", MODEL_LEGACY),
    HDA_CODEC_ID_MODEL!(0x10de0011, "GPU 11 HDMI/DP", MODEL_LEGACY),
    HDA_CODEC_ID_MODEL!(0x10de0012, "GPU 12 HDMI/DP", MODEL_LEGACY),
    HDA_CODEC_ID_MODEL!(0x10de0013, "GPU 13 HDMI/DP", MODEL_LEGACY),
    HDA_CODEC_ID_MODEL!(0x10de0014, "GPU 14 HDMI/DP", MODEL_LEGACY),
    HDA_CODEC_ID_MODEL!(0x10de0015, "GPU 15 HDMI/DP", MODEL_LEGACY),
    HDA_CODEC_ID_MODEL!(0x10de0016, "GPU 16 HDMI/DP", MODEL_LEGACY),
    /* 17 is known to be absent */
    HDA_CODEC_ID_MODEL!(0x10de0018, "GPU 18 HDMI/DP", MODEL_LEGACY),
    HDA_CODEC_ID_MODEL!(0x10de0019, "GPU 19 HDMI/DP", MODEL_LEGACY),
    HDA_CODEC_ID_MODEL!(0x10de001a, "GPU 1a HDMI/DP", MODEL_LEGACY),
    HDA_CODEC_ID_MODEL!(0x10de001b, "GPU 1b HDMI/DP", MODEL_LEGACY),
    HDA_CODEC_ID_MODEL!(0x10de001c, "GPU 1c HDMI/DP", MODEL_LEGACY),
    HDA_CODEC_ID_MODEL!(0x10de0040, "GPU 40 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0041, "GPU 41 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0042, "GPU 42 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0043, "GPU 43 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0044, "GPU 44 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0045, "GPU 45 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0050, "GPU 50 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0051, "GPU 51 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0052, "GPU 52 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0060, "GPU 60 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0061, "GPU 61 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0062, "GPU 62 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0070, "GPU 70 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0071, "GPU 71 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0072, "GPU 72 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0073, "GPU 73 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0074, "GPU 74 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0076, "GPU 76 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de007b, "GPU 7b HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de007c, "GPU 7c HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de007d, "GPU 7d HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de007e, "GPU 7e HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0080, "GPU 80 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0081, "GPU 81 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0082, "GPU 82 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0083, "GPU 83 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0084, "GPU 84 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0090, "GPU 90 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0091, "GPU 91 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0092, "GPU 92 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0093, "GPU 93 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0094, "GPU 94 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0095, "GPU 95 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0097, "GPU 97 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0098, "GPU 98 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de0099, "GPU 99 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de009a, "GPU 9a HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de009b, "GPU 9b HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de009c, "GPU 9c HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de009d, "GPU 9d HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de009e, "GPU 9e HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de009f, "GPU 9f HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de00a0, "GPU a0 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de00a1, "GPU a1 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de00a3, "GPU a3 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de00a4, "GPU a4 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de00a5, "GPU a5 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de00a6, "GPU a6 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de00a7, "GPU a7 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de00a8, "GPU a8 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de00a9, "GPU a9 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de00aa, "GPU aa HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de00ab, "GPU ab HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de00ad, "GPU ad HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de00ae, "GPU ae HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de00af, "GPU af HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de00b0, "GPU b0 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de00b1, "GPU b1 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de00c0, "GPU c0 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de00c1, "GPU c1 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de00c3, "GPU c3 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de00c4, "GPU c4 HDMI/DP", MODEL_GENERIC),
    HDA_CODEC_ID_MODEL!(0x10de00c5, "GPU c5 HDMI/DP", MODEL_GENERIC),
    hda_device_id::default(), /* terminator */
];
MODULE_DEVICE_TABLE!(hdaudio, snd_hda_id_nvhdmi);

MODULE_LICENSE!("GPL");
MODULE_DESCRIPTION!("Nvidia HDMI HD-audio codec");
MODULE_IMPORT_NS!("SND_HDA_CODEC_HDMI");

static mut nvhdmi_driver: hda_codec_driver = hda_codec_driver {
    id: snd_hda_id_nvhdmi.as_ptr(),
    ops: &nvhdmi_codec_ops,
};

module_hda_codec_driver!(nvhdmi_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
