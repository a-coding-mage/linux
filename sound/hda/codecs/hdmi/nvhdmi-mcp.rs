// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Legacy Nvidia HDMI codec support
 */

// C includes translated as dependency intent:
// <linux/init.h>, <linux/slab.h>, <linux/module.h>, <sound/core.h>,
// <sound/hdaudio.h>, <sound/hda_codec.h>, "hda_local.h", "hdmi_local.h"

const MODEL_2CH: u32 = 0;
const MODEL_8CH: u32 = 1;

const Nv_VERB_SET_Channel_Allocation: u32 = 0xF79;
const Nv_VERB_SET_Info_Frame_Checksum: u32 = 0xF7A;
const Nv_VERB_SET_Audio_Protection_On: u32 = 0xF98;
const Nv_VERB_SET_Audio_Protection_Off: u32 = 0xF99;

const nvhdmi_master_con_nid_7x: hda_nid_t = 0x04;
const nvhdmi_master_pin_nid_7x: hda_nid_t = 0x05;

static nvhdmi_con_nids_7x: [hda_nid_t; 4] = [
    /*front, rear, clfe, rear_surr */
    0x6, 0x8, 0xa, 0xc,
];

static nvhdmi_basic_init_7x_2ch: [hda_verb; 3] = [
    /* set audio protect on */
    hda_verb {
        nid: 0x1,
        verb: Nv_VERB_SET_Audio_Protection_On,
        param: 0x1,
    },
    /* enable digital output on pin widget */
    hda_verb {
        nid: 0x5,
        verb: AC_VERB_SET_PIN_WIDGET_CONTROL,
        param: PIN_OUT | 0x5,
    },
    hda_verb {
        nid: 0,
        verb: 0,
        param: 0,
    }, /* terminator */
];

static nvhdmi_basic_init_7x_8ch: [hda_verb; 6] = [
    /* set audio protect on */
    hda_verb {
        nid: 0x1,
        verb: Nv_VERB_SET_Audio_Protection_On,
        param: 0x1,
    },
    /* enable digital output on pin widget */
    hda_verb {
        nid: 0x5,
        verb: AC_VERB_SET_PIN_WIDGET_CONTROL,
        param: PIN_OUT | 0x5,
    },
    hda_verb {
        nid: 0x7,
        verb: AC_VERB_SET_PIN_WIDGET_CONTROL,
        param: PIN_OUT | 0x5,
    },
    hda_verb {
        nid: 0x9,
        verb: AC_VERB_SET_PIN_WIDGET_CONTROL,
        param: PIN_OUT | 0x5,
    },
    hda_verb {
        nid: 0xb,
        verb: AC_VERB_SET_PIN_WIDGET_CONTROL,
        param: PIN_OUT | 0x5,
    },
    hda_verb {
        nid: 0xd,
        verb: AC_VERB_SET_PIN_WIDGET_CONTROL,
        param: PIN_OUT | 0x5,
    },
    hda_verb {
        nid: 0,
        verb: 0,
        param: 0,
    }, /* terminator */
];

unsafe fn nvhdmi_mcp_init(codec: *mut hda_codec) -> i32 {
    let spec: *mut hdmi_spec = (*codec).spec as *mut hdmi_spec;

    if (*spec).multiout.max_channels == 2 {
        snd_hda_sequence_write(codec, nvhdmi_basic_init_7x_2ch.as_ptr());
    } else {
        snd_hda_sequence_write(codec, nvhdmi_basic_init_7x_8ch.as_ptr());
    }
    0
}

unsafe fn nvhdmi_8ch_7x_set_info_frame_parameters(codec: *mut hda_codec, channels: i32) {
    let chanmask: u32;
    let chan: i32 = if channels != 0 { channels - 1 } else { 1 };

    match channels {
        4 => {
            chanmask = 0x08;
        }
        6 => {
            chanmask = 0x0b;
        }
        8 => {
            chanmask = 0x13;
        }
        _ => {
            chanmask = 0x00;
        }
    }

    /* Set the audio infoframe channel allocation and checksum fields.  The
     * channel count is computed implicitly by the hardware.
     */
    snd_hda_codec_write(
        codec,
        0x1,
        0,
        Nv_VERB_SET_Channel_Allocation,
        chanmask,
    );

    snd_hda_codec_write(
        codec,
        0x1,
        0,
        Nv_VERB_SET_Info_Frame_Checksum,
        (0x71i32 - chan - chanmask as i32) as u32,
    );
}

unsafe fn nvhdmi_8ch_7x_pcm_close(
    hinfo: *mut hda_pcm_stream,
    codec: *mut hda_codec,
    substream: *mut snd_pcm_substream,
) -> i32 {
    let spec: *mut hdmi_spec = (*codec).spec as *mut hdmi_spec;
    let mut i: i32;

    snd_hda_codec_write(
        codec,
        nvhdmi_master_con_nid_7x,
        0,
        AC_VERB_SET_CHANNEL_STREAMID,
        0,
    );
    i = 0;
    while i < 4 {
        /* set the stream id */
        snd_hda_codec_write(
            codec,
            nvhdmi_con_nids_7x[i as usize],
            0,
            AC_VERB_SET_CHANNEL_STREAMID,
            0,
        );
        /* set the stream format */
        snd_hda_codec_write(
            codec,
            nvhdmi_con_nids_7x[i as usize],
            0,
            AC_VERB_SET_STREAM_FORMAT,
            0,
        );
        i += 1;
    }

    /* The audio hardware sends a channel count of 0x7 (8ch) when all the
     * streams are disabled.
     */
    nvhdmi_8ch_7x_set_info_frame_parameters(codec, 8);

    snd_hda_multi_out_dig_close(codec, &mut (*spec).multiout)
}

unsafe fn nvhdmi_8ch_7x_pcm_prepare(
    hinfo: *mut hda_pcm_stream,
    codec: *mut hda_codec,
    stream_tag: u32,
    format: u32,
    substream: *mut snd_pcm_substream,
) -> i32 {
    let chs: i32;
    let dataDCC2: u32;
    let mut channel_id: u32;
    let mut i: i32;
    let spec: *mut hdmi_spec = (*codec).spec as *mut hdmi_spec;
    let spdif: *mut hda_spdif_out;
    let per_cvt: *mut hdmi_spec_per_cvt;

    // C used guard(mutex)(&codec->spdif_mutex); preserve locking dependency.
    let _guard = guard_mutex(&mut (*codec).spdif_mutex);
    per_cvt = get_cvt(spec, 0);
    spdif = snd_hda_spdif_out_of_nid(codec, (*per_cvt).cvt_nid);

    chs = (*(*substream).runtime).channels;

    dataDCC2 = 0x2;

    /* turn off SPDIF once; otherwise the IEC958 bits won't be updated */
    if (*codec).spdif_status_reset && ((*spdif).ctls & AC_DIG1_ENABLE) != 0 {
        snd_hda_codec_write(
            codec,
            nvhdmi_master_con_nid_7x,
            0,
            AC_VERB_SET_DIGI_CONVERT_1,
            (*spdif).ctls & !AC_DIG1_ENABLE & 0xff,
        );
    }

    /* set the stream id */
    snd_hda_codec_write(
        codec,
        nvhdmi_master_con_nid_7x,
        0,
        AC_VERB_SET_CHANNEL_STREAMID,
        (stream_tag << 4) | 0x0,
    );

    /* set the stream format */
    snd_hda_codec_write(
        codec,
        nvhdmi_master_con_nid_7x,
        0,
        AC_VERB_SET_STREAM_FORMAT,
        format,
    );

    /* turn on again (if needed) */
    /* enable and set the channel status audio/data flag */
    if (*codec).spdif_status_reset && ((*spdif).ctls & AC_DIG1_ENABLE) != 0 {
        snd_hda_codec_write(
            codec,
            nvhdmi_master_con_nid_7x,
            0,
            AC_VERB_SET_DIGI_CONVERT_1,
            (*spdif).ctls & 0xff,
        );
        snd_hda_codec_write(
            codec,
            nvhdmi_master_con_nid_7x,
            0,
            AC_VERB_SET_DIGI_CONVERT_2,
            dataDCC2,
        );
    }

    i = 0;
    while i < 4 {
        if chs == 2 {
            channel_id = 0;
        } else {
            channel_id = (i * 2) as u32;
        }

        /* turn off SPDIF once;
         *otherwise the IEC958 bits won't be updated
         */
        if (*codec).spdif_status_reset && ((*spdif).ctls & AC_DIG1_ENABLE) != 0 {
            snd_hda_codec_write(
                codec,
                nvhdmi_con_nids_7x[i as usize],
                0,
                AC_VERB_SET_DIGI_CONVERT_1,
                (*spdif).ctls & !AC_DIG1_ENABLE & 0xff,
            );
        }
        /* set the stream id */
        snd_hda_codec_write(
            codec,
            nvhdmi_con_nids_7x[i as usize],
            0,
            AC_VERB_SET_CHANNEL_STREAMID,
            (stream_tag << 4) | channel_id,
        );
        /* set the stream format */
        snd_hda_codec_write(
            codec,
            nvhdmi_con_nids_7x[i as usize],
            0,
            AC_VERB_SET_STREAM_FORMAT,
            format,
        );
        /* turn on again (if needed) */
        /* enable and set the channel status audio/data flag */
        if (*codec).spdif_status_reset && ((*spdif).ctls & AC_DIG1_ENABLE) != 0 {
            snd_hda_codec_write(
                codec,
                nvhdmi_con_nids_7x[i as usize],
                0,
                AC_VERB_SET_DIGI_CONVERT_1,
                (*spdif).ctls & 0xff,
            );
            snd_hda_codec_write(
                codec,
                nvhdmi_con_nids_7x[i as usize],
                0,
                AC_VERB_SET_DIGI_CONVERT_2,
                dataDCC2,
            );
        }
        i += 1;
    }

    nvhdmi_8ch_7x_set_info_frame_parameters(codec, chs);

    0
}

static nvhdmi_pcm_playback_8ch_7x: hda_pcm_stream = hda_pcm_stream {
    substreams: 1,
    channels_min: 2,
    channels_max: 8,
    nid: nvhdmi_master_con_nid_7x,
    rates: SUPPORTED_RATES,
    maxbps: SUPPORTED_MAXBPS,
    formats: SUPPORTED_FORMATS,
    ops: hda_pcm_stream_ops {
        open: Some(snd_hda_hdmi_simple_pcm_open),
        close: Some(nvhdmi_8ch_7x_pcm_close),
        prepare: Some(nvhdmi_8ch_7x_pcm_prepare),
    },
};

unsafe fn nvhdmi_mcp_build_pcms(codec: *mut hda_codec) -> i32 {
    let spec: *mut hdmi_spec = (*codec).spec as *mut hdmi_spec;
    let err: i32;

    err = snd_hda_hdmi_simple_build_pcms(codec);
    if err == 0 && (*spec).multiout.max_channels == 8 {
        let info: *mut hda_pcm = get_pcm_rec(spec, 0);

        (*info).own_chmap = true;
    }
    err
}

unsafe fn nvhdmi_mcp_build_controls(codec: *mut hda_codec) -> i32 {
    let spec: *mut hdmi_spec = (*codec).spec as *mut hdmi_spec;
    let info: *mut hda_pcm;
    let mut chmap: *mut snd_pcm_chmap = core::ptr::null_mut();
    let mut err: i32;

    err = snd_hda_hdmi_simple_build_controls(codec);
    if err < 0 {
        return err;
    }

    if (*spec).multiout.max_channels != 8 {
        return 0;
    }

    /* add channel maps */
    info = get_pcm_rec(spec, 0);
    err = snd_pcm_add_chmap_ctls(
        (*info).pcm,
        SNDRV_PCM_STREAM_PLAYBACK,
        snd_pcm_alt_chmaps,
        8,
        0,
        &mut chmap,
    );
    if err < 0 {
        return err;
    }
    match (*(*codec).preset).vendor_id {
        0x10de0002 | 0x10de0003 | 0x10de0005 | 0x10de0006 => {
            (*chmap).channel_mask = (1u32 << 2) | (1u32 << 8);
        }
        0x10de0007 => {
            (*chmap).channel_mask = (1u32 << 2) | (1u32 << 6) | (1u32 << 8);
        }
        _ => {}
    }
    0
}

static channels_2_6_8: [u32; 3] = [2, 6, 8];

static channels_2_8: [u32; 2] = [2, 8];

static hw_constraints_2_6_8_channels: snd_pcm_hw_constraint_list =
    snd_pcm_hw_constraint_list {
        count: channels_2_6_8.len() as u32,
        list: channels_2_6_8.as_ptr(),
        mask: 0,
    };

static hw_constraints_2_8_channels: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: channels_2_8.len() as u32,
    list: channels_2_8.as_ptr(),
    mask: 0,
};

unsafe fn nvhdmi_mcp_probe(codec: *mut hda_codec, id: *const hda_device_id) -> i32 {
    let spec: *mut hdmi_spec;
    let mut err: i32;

    err = snd_hda_hdmi_simple_probe(
        codec,
        nvhdmi_master_con_nid_7x,
        nvhdmi_master_pin_nid_7x,
    );
    if err < 0 {
        return err;
    }

    /* override the PCM rates, etc, as the codec doesn't give full list */
    spec = (*codec).spec as *mut hdmi_spec;
    (*spec).pcm_playback.rates = SUPPORTED_RATES;
    (*spec).pcm_playback.maxbps = SUPPORTED_MAXBPS;
    (*spec).pcm_playback.formats = SUPPORTED_FORMATS;
    (*spec).nv_dp_workaround = true;

    if (*id).driver_data == MODEL_2CH as usize {
        return 0;
    }

    (*spec).multiout.max_channels = 8;
    (*spec).pcm_playback = nvhdmi_pcm_playback_8ch_7x;

    match (*(*codec).preset).vendor_id {
        0x10de0002 | 0x10de0003 | 0x10de0005 | 0x10de0006 => {
            (*spec).hw_constraints_channels = &hw_constraints_2_8_channels;
        }
        0x10de0007 => {
            (*spec).hw_constraints_channels = &hw_constraints_2_6_8_channels;
        }
        _ => {}
    }

    /* Initialize the audio infoframe channel mask and checksum to something
     * valid
     */
    nvhdmi_8ch_7x_set_info_frame_parameters(codec, 8);

    0
}

static nvhdmi_mcp_codec_ops: hda_codec_ops = hda_codec_ops {
    probe: Some(nvhdmi_mcp_probe),
    remove: Some(snd_hda_hdmi_simple_remove),
    build_pcms: Some(nvhdmi_mcp_build_pcms),
    build_controls: Some(nvhdmi_mcp_build_controls),
    init: Some(nvhdmi_mcp_init),
    unsol_event: Some(snd_hda_hdmi_simple_unsol_event),
};

static snd_hda_id_nvhdmi_mcp: [hda_device_id; 11] = [
    HDA_CODEC_ID_MODEL(0x10de0001, "MCP73 HDMI", MODEL_2CH),
    HDA_CODEC_ID_MODEL(0x10de0002, "MCP77/78 HDMI", MODEL_8CH),
    HDA_CODEC_ID_MODEL(0x10de0003, "MCP77/78 HDMI", MODEL_8CH),
    HDA_CODEC_ID_MODEL(0x10de0004, "GPU 04 HDMI", MODEL_8CH),
    HDA_CODEC_ID_MODEL(0x10de0005, "MCP77/78 HDMI", MODEL_8CH),
    HDA_CODEC_ID_MODEL(0x10de0006, "MCP77/78 HDMI", MODEL_8CH),
    HDA_CODEC_ID_MODEL(0x10de0007, "MCP79/7A HDMI", MODEL_8CH),
    HDA_CODEC_ID_MODEL(0x10de0067, "MCP67 HDMI", MODEL_2CH),
    HDA_CODEC_ID_MODEL(0x10de8001, "MCP73 HDMI", MODEL_2CH),
    HDA_CODEC_ID_MODEL(0x10de8067, "MCP67/68 HDMI", MODEL_2CH),
    hda_device_id::default(), /* terminator */
];
// MODULE_DEVICE_TABLE(hdaudio, snd_hda_id_nvhdmi_mcp);

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Legacy Nvidia HDMI HD-audio codec");
// MODULE_IMPORT_NS("SND_HDA_CODEC_HDMI");

static mut nvhdmi_mcp_driver: hda_codec_driver = hda_codec_driver {
    id: snd_hda_id_nvhdmi_mcp.as_ptr(),
    ops: &nvhdmi_mcp_codec_ops,
};

// module_hda_codec_driver(nvhdmi_mcp_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
