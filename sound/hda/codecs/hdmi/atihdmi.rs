// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ATI/AMD codec support
 */

// Dependencies originally included from Linux and HDA headers:
// linux/init.h, linux/slab.h, linux/module.h, linux/unaligned.h,
// sound/core.h, sound/tlv.h, sound/hdaudio.h, sound/hda_codec.h,
// hda_local.h, hdmi_local.h.

use core::ffi::{c_char, c_int, c_uint, c_void};

type hda_nid_t = c_uint;
type u32 = c_uint;

const EINVAL: c_int = 22;

#[repr(C)]
pub struct hda_codec {
    pub core: hda_codec_core,
    pub spec: *mut hdmi_spec,
    pub auto_runtime_pm: c_int,
    pub link_down_at_suspend: c_int,
}

#[repr(C)]
pub struct hda_codec_core {
    pub vendor_id: c_uint,
    pub revision_id: c_uint,
}

#[repr(C)]
pub struct hdac_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hda_device_id {
    pub vendor_id: c_uint,
    pub rev_id: c_uint,
    pub api_version: c_uint,
    pub name: *const c_char,
    pub driver_data: usize,
}

#[repr(C)]
pub struct hdac_chmap {
    pub channels_max: c_uint,
    pub ops: hdac_chmap_ops,
}

#[repr(C)]
pub struct hdac_chmap_ops {
    pub pin_get_slot_channel:
        Option<unsafe extern "C" fn(*mut hdac_device, hda_nid_t, c_int) -> c_int>,
    pub pin_set_slot_channel:
        Option<unsafe extern "C" fn(*mut hdac_device, hda_nid_t, c_int, c_int) -> c_int>,
    pub chmap_cea_alloc_validate_get_type: Option<
        unsafe extern "C" fn(
            *mut hdac_chmap,
            *mut hdac_cea_channel_speaker_allocation,
            c_int,
        ) -> c_int,
    >,
    pub cea_alloc_to_tlv_chmap: Option<
        unsafe extern "C" fn(
            *mut hdac_chmap,
            *mut hdac_cea_channel_speaker_allocation,
            *mut c_uint,
            c_int,
        ),
    >,
    pub chmap_validate:
        Option<unsafe extern "C" fn(*mut hdac_chmap, c_int, c_int, *mut u8) -> c_int>,
}

#[repr(C)]
pub struct hdac_cea_channel_speaker_allocation {
    pub speakers: [c_int; 8],
}

#[repr(C)]
pub struct hdmi_spec {
    pub num_pins: c_int,
    pub num_cvts: c_int,
    pub static_pcm_mapping: bool,
    pub ops: hdmi_spec_ops,
    pub chmap: hdac_chmap,
}

#[repr(C)]
pub struct hdmi_spec_ops {
    pub pin_get_eld:
        Option<unsafe extern "C" fn(*mut hda_codec, hda_nid_t, c_int, *mut u8, *mut c_int) -> c_int>,
    pub pin_setup_infoframe:
        Option<unsafe extern "C" fn(*mut hda_codec, hda_nid_t, c_int, c_int, c_int, c_int)>,
    pub pin_hbr_setup: Option<unsafe extern "C" fn(*mut hda_codec, hda_nid_t, c_int, bool) -> c_int>,
    pub setup_stream:
        Option<unsafe extern "C" fn(*mut hda_codec, hda_nid_t, hda_nid_t, c_int, u32, c_int) -> c_int>,
}

#[repr(C)]
pub struct hdmi_spec_per_pin {
    pub pin_nid: hda_nid_t,
}

#[repr(C)]
pub struct hdmi_spec_per_cvt {
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
    pub maxbps: c_uint,
}

#[repr(C)]
pub struct drm_audio_component_audio_ops {
    pub pin2port: Option<unsafe extern "C" fn(*mut c_void, c_int) -> c_int>,
    pub pin_eld_notify: Option<unsafe extern "C" fn()>,
    pub master_bind: Option<unsafe extern "C" fn()>,
    pub master_unbind: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct hda_codec_ops {
    pub probe: Option<unsafe extern "C" fn(*mut hda_codec, *const hda_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut hda_codec)>,
    pub init: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub build_pcms: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub build_controls: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub unsol_event: Option<unsafe extern "C" fn(*mut hda_codec, c_uint)>,
    pub suspend: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
}

#[repr(C)]
pub struct hda_codec_driver {
    pub id: *const hda_device_id,
    pub ops: *const hda_codec_ops,
}

extern "C" {
    fn snd_hda_codec_read(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        flags: c_int,
        verb: c_int,
        parm: c_int,
    ) -> c_int;
    fn snd_hda_codec_write(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        flags: c_int,
        verb: c_int,
        parm: c_int,
    ) -> c_int;
    fn codec_info(codec: *mut hda_codec, fmt: *const c_char, ...);
    fn codec_dbg(codec: *mut hda_codec, fmt: *const c_char, ...);
    fn snd_hdac_get_ch_alloc_from_ca(ca: c_int) -> *mut hdac_cea_channel_speaker_allocation;
    fn snd_hdac_chmap_to_spk_mask(chmap: u8) -> c_int;
    fn snd_hdac_spk_to_chmap(spk: c_int) -> c_uint;
    fn hdac_to_hda_codec(hdac: *mut hdac_device) -> *mut hda_codec;
    fn snd_hda_hdmi_setup_stream(
        codec: *mut hda_codec,
        cvt_nid: hda_nid_t,
        pin_nid: hda_nid_t,
        dev_id: c_int,
        stream_tag: u32,
        format: c_int,
    ) -> c_int;
    fn snd_hda_hdmi_generic_init(codec: *mut hda_codec) -> c_int;
    fn snd_hda_hdmi_generic_probe(codec: *mut hda_codec) -> c_int;
    fn snd_hda_hdmi_generic_remove(codec: *mut hda_codec);
    fn snd_hda_hdmi_generic_build_pcms(codec: *mut hda_codec) -> c_int;
    fn snd_hda_hdmi_generic_build_controls(codec: *mut hda_codec) -> c_int;
    fn snd_hda_hdmi_generic_unsol_event(codec: *mut hda_codec, res: c_uint);
    fn snd_hda_hdmi_generic_suspend(codec: *mut hda_codec) -> c_int;
    fn snd_hda_hdmi_generic_resume(codec: *mut hda_codec) -> c_int;
    fn get_pin(spec: *mut hdmi_spec, pin_idx: c_int) -> *mut hdmi_spec_per_pin;
    fn get_cvt(spec: *mut hdmi_spec, cvt_idx: c_int) -> *mut hdmi_spec_per_cvt;
    fn snd_hda_hdmi_acomp_init(
        codec: *mut hda_codec,
        ops: *const drm_audio_component_audio_ops,
        port2pin: Option<unsafe extern "C" fn(*mut hda_codec, c_int) -> c_int>,
    );
    fn snd_hda_hdmi_acomp_pin_eld_notify();
    fn snd_hda_hdmi_acomp_master_bind();
    fn snd_hda_hdmi_acomp_master_unbind();
}

const ATI_VERB_SET_CHANNEL_ALLOCATION: c_int = 0x771;
const ATI_VERB_SET_DOWNMIX_INFO: c_int = 0x772;
const ATI_VERB_SET_MULTICHANNEL_01: c_int = 0x777;
const ATI_VERB_SET_MULTICHANNEL_23: c_int = 0x778;
const ATI_VERB_SET_MULTICHANNEL_45: c_int = 0x779;
const ATI_VERB_SET_MULTICHANNEL_67: c_int = 0x77a;
const ATI_VERB_SET_HBR_CONTROL: c_int = 0x77c;
const ATI_VERB_SET_MULTICHANNEL_1: c_int = 0x785;
const ATI_VERB_SET_MULTICHANNEL_3: c_int = 0x786;
const ATI_VERB_SET_MULTICHANNEL_5: c_int = 0x787;
const ATI_VERB_SET_MULTICHANNEL_7: c_int = 0x788;
const ATI_VERB_SET_MULTICHANNEL_MODE: c_int = 0x789;
const ATI_VERB_GET_CHANNEL_ALLOCATION: c_int = 0xf71;
const ATI_VERB_GET_DOWNMIX_INFO: c_int = 0xf72;
const ATI_VERB_GET_MULTICHANNEL_01: c_int = 0xf77;
const ATI_VERB_GET_MULTICHANNEL_23: c_int = 0xf78;
const ATI_VERB_GET_MULTICHANNEL_45: c_int = 0xf79;
const ATI_VERB_GET_MULTICHANNEL_67: c_int = 0xf7a;
const ATI_VERB_GET_HBR_CONTROL: c_int = 0xf7c;
const ATI_VERB_GET_MULTICHANNEL_1: c_int = 0xf85;
const ATI_VERB_GET_MULTICHANNEL_3: c_int = 0xf86;
const ATI_VERB_GET_MULTICHANNEL_5: c_int = 0xf87;
const ATI_VERB_GET_MULTICHANNEL_7: c_int = 0xf88;
const ATI_VERB_GET_MULTICHANNEL_MODE: c_int = 0xf89;
const ATI_VERB_SET_RAMP_RATE: c_int = 0x770;
const ATI_VERB_GET_RAMP_RATE: c_int = 0xf70;
const ATI_OUT_ENABLE: c_int = 0x1;
const ATI_MULTICHANNEL_MODE_PAIRED: c_int = 0;
const ATI_MULTICHANNEL_MODE_SINGLE: c_int = 1;
const ATI_HBR_CAPABLE: c_int = 0x01;
const ATI_HBR_ENABLE: c_int = 0x10;
const ATI_VERB_SET_AUDIO_DESCRIPTOR: c_int = 0x776;
const ATI_VERB_SET_SINK_INFO_INDEX: c_int = 0x780;
const ATI_VERB_GET_SPEAKER_ALLOCATION: c_int = 0xf70;
const ATI_VERB_GET_AUDIO_DESCRIPTOR: c_int = 0xf76;
const ATI_VERB_GET_AUDIO_VIDEO_DELAY: c_int = 0xf7b;
const ATI_VERB_GET_SINK_INFO_INDEX: c_int = 0xf80;
const ATI_VERB_GET_SINK_INFO_DATA: c_int = 0xf81;
const ATI_SPKALLOC_SPKALLOC: c_int = 0x007f;
const ATI_SPKALLOC_TYPE_HDMI: c_int = 0x0100;
const ATI_SPKALLOC_TYPE_DISPLAYPORT: c_int = 0x0200;
const ATI_AUDIODESC_CHANNELS: c_int = 0x00000007;
const ATI_AUDIODESC_RATES: c_int = 0x0000ff00;
const ATI_AUDIODESC_LPCM_STEREO_RATES: c_int = 0xff000000u32 as c_int;
const ATI_DELAY_VIDEO_LATENCY: c_int = 0x000000ff;
const ATI_DELAY_AUDIO_LATENCY: c_int = 0x0000ff00;

const ELD_FIXED_BYTES: c_int = 20;
const ELD_MAX_MNL: c_int = 16;
const ELD_MAX_SAD: c_int = 16;
const ELD_VER_CEA_861D: c_int = 2;
const AUDIO_CODING_TYPE_LPCM: c_int = 1;
const AUDIO_CODING_TYPE_SACD: c_int = 9;
const AUDIO_CODING_TYPE_DST: c_int = 13;
const AUDIO_CODING_TYPE_WMAPRO: c_int = 14;
const SNDRV_CTL_TLVT_CHMAP_PAIRED: c_int = 0x103;
const SNDRV_CHMAP_NA: c_uint = 0;
const AC_FMT_TYPE_NON_PCM: c_int = 0x8000;
const SUPPORTED_RATES: c_uint = 0;
const SUPPORTED_FORMATS: c_uint = 0;

#[repr(C)]
enum ati_sink_info_idx {
    ATI_INFO_IDX_MANUFACTURER_ID = 0,
    ATI_INFO_IDX_PRODUCT_ID = 1,
    ATI_INFO_IDX_SINK_DESC_LEN = 2,
    ATI_INFO_IDX_PORT_ID_LOW = 3,
    ATI_INFO_IDX_PORT_ID_HIGH = 4,
    ATI_INFO_IDX_SINK_DESC_FIRST = 5,
    ATI_INFO_IDX_SINK_DESC_LAST = 22, /* max len 18 bytes */
}

unsafe fn is_amdhdmi_rev3_or_later(codec: *mut hda_codec) -> bool {
    (*codec).core.vendor_id == 0x1002aa01 && ((*codec).core.revision_id & 0xff00) >= 0x0300
}

unsafe fn has_amd_full_remap_support(codec: *mut hda_codec) -> bool {
    is_amdhdmi_rev3_or_later(codec)
}

unsafe fn put_unaligned_le16(val: c_int, p: *mut u8) {
    let bytes = (val as u16).to_le_bytes();
    core::ptr::copy_nonoverlapping(bytes.as_ptr(), p, 2);
}

unsafe fn put_unaligned_le32(val: c_int, p: *mut u8) {
    let bytes = (val as u32).to_le_bytes();
    core::ptr::copy_nonoverlapping(bytes.as_ptr(), p, 4);
}

fn round_up(x: c_int, y: c_int) -> c_int {
    ((x + y - 1) / y) * y
}

unsafe extern "C" fn get_eld_ati(
    codec: *mut hda_codec,
    nid: hda_nid_t,
    buf: *mut u8,
    eld_size: *mut c_int,
    rev3_or_later: bool,
) -> c_int {
    let mut sink_desc_len: c_int = 0;
    let mut pos: c_int;
    let mut i: c_int;

    /* ATI/AMD does not have ELD, emulate it */

    let spkalloc = snd_hda_codec_read(codec, nid, 0, ATI_VERB_GET_SPEAKER_ALLOCATION, 0);

    if spkalloc <= 0 {
        codec_info(codec, c"HDMI ATI/AMD: no speaker allocation for ELD\n".as_ptr());
        return -EINVAL;
    }

    core::ptr::write_bytes(
        buf,
        0,
        (ELD_FIXED_BYTES + ELD_MAX_MNL + ELD_MAX_SAD * 3) as usize,
    );

    /* version */
    *buf.add(0) = (ELD_VER_CEA_861D << 3) as u8;

    /* speaker allocation from EDID */
    *buf.add(7) = (spkalloc & ATI_SPKALLOC_SPKALLOC) as u8;

    /* is DisplayPort? */
    if (spkalloc & ATI_SPKALLOC_TYPE_DISPLAYPORT) != 0 {
        *buf.add(5) |= 0x04;
    }

    pos = ELD_FIXED_BYTES;

    if rev3_or_later {
        let mut sink_info: c_int;

        snd_hda_codec_write(
            codec,
            nid,
            0,
            ATI_VERB_SET_SINK_INFO_INDEX,
            ati_sink_info_idx::ATI_INFO_IDX_PORT_ID_LOW as c_int,
        );
        sink_info = snd_hda_codec_read(codec, nid, 0, ATI_VERB_GET_SINK_INFO_DATA, 0);
        put_unaligned_le32(sink_info, buf.add(8));

        snd_hda_codec_write(
            codec,
            nid,
            0,
            ATI_VERB_SET_SINK_INFO_INDEX,
            ati_sink_info_idx::ATI_INFO_IDX_PORT_ID_HIGH as c_int,
        );
        sink_info = snd_hda_codec_read(codec, nid, 0, ATI_VERB_GET_SINK_INFO_DATA, 0);
        put_unaligned_le32(sink_info, buf.add(12));

        snd_hda_codec_write(
            codec,
            nid,
            0,
            ATI_VERB_SET_SINK_INFO_INDEX,
            ati_sink_info_idx::ATI_INFO_IDX_MANUFACTURER_ID as c_int,
        );
        sink_info = snd_hda_codec_read(codec, nid, 0, ATI_VERB_GET_SINK_INFO_DATA, 0);
        put_unaligned_le16(sink_info, buf.add(16));

        snd_hda_codec_write(
            codec,
            nid,
            0,
            ATI_VERB_SET_SINK_INFO_INDEX,
            ati_sink_info_idx::ATI_INFO_IDX_PRODUCT_ID as c_int,
        );
        sink_info = snd_hda_codec_read(codec, nid, 0, ATI_VERB_GET_SINK_INFO_DATA, 0);
        put_unaligned_le16(sink_info, buf.add(18));

        snd_hda_codec_write(
            codec,
            nid,
            0,
            ATI_VERB_SET_SINK_INFO_INDEX,
            ati_sink_info_idx::ATI_INFO_IDX_SINK_DESC_LEN as c_int,
        );
        sink_desc_len = snd_hda_codec_read(codec, nid, 0, ATI_VERB_GET_SINK_INFO_DATA, 0);

        if sink_desc_len > ELD_MAX_MNL {
            codec_info(
                codec,
                c"HDMI ATI/AMD: Truncating HDMI sink description with length %d\n".as_ptr(),
                sink_desc_len,
            );
            sink_desc_len = ELD_MAX_MNL;
        }

        *buf.add(4) |= sink_desc_len as u8;

        i = 0;
        while i < sink_desc_len {
            snd_hda_codec_write(
                codec,
                nid,
                0,
                ATI_VERB_SET_SINK_INFO_INDEX,
                ati_sink_info_idx::ATI_INFO_IDX_SINK_DESC_FIRST as c_int + i,
            );
            *buf.add(pos as usize) =
                snd_hda_codec_read(codec, nid, 0, ATI_VERB_GET_SINK_INFO_DATA, 0) as u8;
            pos += 1;
            i += 1;
        }
    }

    i = AUDIO_CODING_TYPE_LPCM;
    while i <= AUDIO_CODING_TYPE_WMAPRO {
        if i == AUDIO_CODING_TYPE_SACD || i == AUDIO_CODING_TYPE_DST {
            i += 1;
            continue; /* not handled by ATI/AMD */
        }

        snd_hda_codec_write(codec, nid, 0, ATI_VERB_SET_AUDIO_DESCRIPTOR, i << 3);
        let ati_sad = snd_hda_codec_read(codec, nid, 0, ATI_VERB_GET_AUDIO_DESCRIPTOR, 0);

        if ati_sad <= 0 {
            i += 1;
            continue;
        }

        if (ati_sad & ATI_AUDIODESC_RATES) != 0 {
            /* format is supported, copy SAD as-is */
            *buf.add(pos as usize) = ((ati_sad & 0x0000ff) >> 0) as u8;
            pos += 1;
            *buf.add(pos as usize) = ((ati_sad & 0x00ff00) >> 8) as u8;
            pos += 1;
            *buf.add(pos as usize) = ((ati_sad & 0xff0000) >> 16) as u8;
            pos += 1;
        }

        if i == AUDIO_CODING_TYPE_LPCM
            && (ati_sad & ATI_AUDIODESC_LPCM_STEREO_RATES) != 0
            && ((ati_sad & ATI_AUDIODESC_LPCM_STEREO_RATES) >> 16)
                != (ati_sad & ATI_AUDIODESC_RATES)
        {
            /* for PCM there is a separate stereo rate mask */
            *buf.add(pos as usize) =
                (((ati_sad & 0x000000ff) & !ATI_AUDIODESC_CHANNELS) | 0x1) as u8;
            pos += 1;
            /* rates from the extra byte */
            *buf.add(pos as usize) = ((ati_sad & 0xff000000u32 as c_int) >> 24) as u8;
            pos += 1;
            *buf.add(pos as usize) = ((ati_sad & 0x00ff0000) >> 16) as u8;
            pos += 1;
        }
        i += 1;
    }

    if pos == ELD_FIXED_BYTES + sink_desc_len {
        codec_info(codec, c"HDMI ATI/AMD: no audio descriptors for ELD\n".as_ptr());
        return -EINVAL;
    }

    /*
     * HDMI VSDB latency format:
     * separately for both audio and video:
     *  0          field not valid or unknown latency
     *  [1..251]   msecs = (x-1)*2  (max 500ms with x = 251 = 0xfb)
     *  255        audio/video not supported
     *
     * HDA latency format:
     * single value indicating video latency relative to audio:
     *  0          unknown or 0ms
     *  [1..250]   msecs = x*2  (max 500ms with x = 250 = 0xfa)
     *  [251..255] reserved
     */
    let aud_synch = snd_hda_codec_read(codec, nid, 0, ATI_VERB_GET_AUDIO_VIDEO_DELAY, 0);
    if (aud_synch & ATI_DELAY_VIDEO_LATENCY) != 0 && (aud_synch & ATI_DELAY_AUDIO_LATENCY) != 0 {
        let video_latency_hdmi = aud_synch & ATI_DELAY_VIDEO_LATENCY;
        let audio_latency_hdmi = (aud_synch & ATI_DELAY_AUDIO_LATENCY) >> 8;

        if video_latency_hdmi <= 0xfb
            && audio_latency_hdmi <= 0xfb
            && video_latency_hdmi > audio_latency_hdmi
        {
            *buf.add(6) = (video_latency_hdmi - audio_latency_hdmi) as u8;
        }
        /* else unknown/invalid or 0ms or video ahead of audio, so use zero */
    }

    /* SAD count */
    *buf.add(5) |= (((pos - ELD_FIXED_BYTES - sink_desc_len) / 3) << 4) as u8;

    /* Baseline ELD block length is 4-byte aligned */
    pos = round_up(pos, 4);

    /* Baseline ELD length (4-byte header is not counted in) */
    *buf.add(2) = ((pos - 4) / 4) as u8;

    *eld_size = pos;

    0
}

unsafe extern "C" fn atihdmi_pin_get_eld(
    codec: *mut hda_codec,
    nid: hda_nid_t,
    dev_id: c_int,
    buf: *mut u8,
    eld_size: *mut c_int,
) -> c_int {
    debug_assert!(dev_id == 0);
    /* call hda_eld.c ATI/AMD-specific function */
    get_eld_ati(codec, nid, buf, eld_size, is_amdhdmi_rev3_or_later(codec))
}

unsafe extern "C" fn atihdmi_pin_setup_infoframe(
    codec: *mut hda_codec,
    pin_nid: hda_nid_t,
    dev_id: c_int,
    ca: c_int,
    _active_channels: c_int,
    _conn_type: c_int,
) {
    debug_assert!(dev_id == 0);
    snd_hda_codec_write(codec, pin_nid, 0, ATI_VERB_SET_CHANNEL_ALLOCATION, ca);
}

fn atihdmi_paired_swap_fc_lfe(pos: c_int) -> c_int {
    /*
     * ATI/AMD have automatic FC/LFE swap built-in
     * when in pairwise mapping mode.
     */
    match pos {
        /* see channel_allocations[].speakers[] */
        2 => 3,
        3 => 2,
        _ => pos,
    }
}

unsafe extern "C" fn atihdmi_paired_chmap_validate(
    _chmap: *mut hdac_chmap,
    ca: c_int,
    chs: c_int,
    map: *mut u8,
) -> c_int {
    /* check that only channel pairs need to be remapped on old pre-rev3 ATI/AMD */
    let cap = snd_hdac_get_ch_alloc_from_ca(ca);
    let mut i = 0;
    while i < chs {
        let mask = snd_hdac_chmap_to_spk_mask(*map.add(i as usize));
        let mut ok = false;
        let mut companion_ok = false;

        if mask == 0 {
            i += 1;
            continue;
        }

        let mut j = 0 + i % 2;
        while j < 8 {
            let chan_idx = 7 - atihdmi_paired_swap_fc_lfe(j);

            if (*cap).speakers[chan_idx as usize] == mask {
                /* channel is in a supported position */
                ok = true;

                if i % 2 == 0 && i + 1 < chs {
                    /* even channel, check the odd companion */
                    let comp_chan_idx = 7 - atihdmi_paired_swap_fc_lfe(j + 1);
                    let comp_mask_req = snd_hdac_chmap_to_spk_mask(*map.add((i + 1) as usize));
                    let comp_mask_act = (*cap).speakers[comp_chan_idx as usize];

                    if comp_mask_req == comp_mask_act {
                        companion_ok = true;
                    } else {
                        return -EINVAL;
                    }
                }
                break;
            }
            j += 2;
        }

        if !ok {
            return -EINVAL;
        }

        if companion_ok {
            i += 1; /* companion channel already checked */
        }
        i += 1;
    }

    0
}

unsafe extern "C" fn atihdmi_pin_set_slot_channel(
    hdac: *mut hdac_device,
    pin_nid: hda_nid_t,
    mut hdmi_slot: c_int,
    mut stream_channel: c_int,
) -> c_int {
    let codec = hdac_to_hda_codec(hdac);
    let mut ati_channel_setup = 0;

    if hdmi_slot > 7 {
        return -EINVAL;
    }

    if !has_amd_full_remap_support(codec) {
        hdmi_slot = atihdmi_paired_swap_fc_lfe(hdmi_slot);

        /* In case this is an odd slot but without stream channel, do not
         * disable the slot since the corresponding even slot could have a
         * channel. In case neither have a channel, the slot pair will be
         * disabled when this function is called for the even slot.
         */
        if hdmi_slot % 2 != 0 && stream_channel == 0xf {
            return 0;
        }

        hdmi_slot -= hdmi_slot % 2;

        if stream_channel != 0xf {
            stream_channel -= stream_channel % 2;
        }
    }

    let verb =
        ATI_VERB_SET_MULTICHANNEL_01 + hdmi_slot / 2 + (hdmi_slot % 2) * 0x00e;

    /* ati_channel_setup format: [7..4] = stream_channel_id, [1] = mute, [0] = enable */

    if stream_channel != 0xf {
        ati_channel_setup = (stream_channel << 4) | ATI_OUT_ENABLE;
    }

    snd_hda_codec_write(codec, pin_nid, 0, verb, ati_channel_setup)
}

unsafe extern "C" fn atihdmi_pin_get_slot_channel(
    hdac: *mut hdac_device,
    pin_nid: hda_nid_t,
    asp_slot: c_int,
) -> c_int {
    let codec = hdac_to_hda_codec(hdac);
    let mut was_odd = false;
    let mut ati_asp_slot = asp_slot;

    if asp_slot > 7 {
        return -EINVAL;
    }

    if !has_amd_full_remap_support(codec) {
        ati_asp_slot = atihdmi_paired_swap_fc_lfe(asp_slot);
        if ati_asp_slot % 2 != 0 {
            ati_asp_slot -= 1;
            was_odd = true;
        }
    }

    let verb =
        ATI_VERB_GET_MULTICHANNEL_01 + ati_asp_slot / 2 + (ati_asp_slot % 2) * 0x00e;

    let ati_channel_setup = snd_hda_codec_read(codec, pin_nid, 0, verb, 0);

    if (ati_channel_setup & ATI_OUT_ENABLE) == 0 {
        return 0xf;
    }

    ((ati_channel_setup & 0xf0) >> 4) + if was_odd { 1 } else { 0 }
}

unsafe extern "C" fn atihdmi_paired_chmap_cea_alloc_validate_get_type(
    _chmap: *mut hdac_chmap,
    cap: *mut hdac_cea_channel_speaker_allocation,
    channels: c_int,
) -> c_int {
    /*
     * Pre-rev3 ATI/AMD codecs operate in a paired channel mode, so
     * we need to take that into account (a single channel may take 2
     * channel slots if we need to carry a silent channel next to it).
     * On Rev3+ AMD codecs this function is not used.
     */
    let mut chanpairs = 0;

    /* We only produce even-numbered channel count TLVs */
    if (channels % 2) != 0 {
        return -1;
    }

    let mut c = 0;
    while c < 7 {
        if (*cap).speakers[c as usize] != 0 || (*cap).speakers[(c + 1) as usize] != 0 {
            chanpairs += 1;
        }
        c += 2;
    }

    if chanpairs * 2 != channels {
        return -1;
    }

    SNDRV_CTL_TLVT_CHMAP_PAIRED
}

unsafe extern "C" fn atihdmi_paired_cea_alloc_to_tlv_chmap(
    _hchmap: *mut hdac_chmap,
    cap: *mut hdac_cea_channel_speaker_allocation,
    chmap: *mut c_uint,
    channels: c_int,
) {
    /* produce paired maps for pre-rev3 ATI/AMD codecs */
    let mut count = 0;
    let mut c = 7;

    while c >= 0 {
        let chan = 7 - atihdmi_paired_swap_fc_lfe(7 - c);
        let spk = (*cap).speakers[chan as usize];

        if spk == 0 {
            /* add N/A channel if the companion channel is occupied */
            if (*cap).speakers[(chan + if chan % 2 != 0 { -1 } else { 1 }) as usize] != 0 {
                *chmap.add(count as usize) = SNDRV_CHMAP_NA;
                count += 1;
            }
            c -= 1;
            continue;
        }

        *chmap.add(count as usize) = snd_hdac_spk_to_chmap(spk);
        count += 1;
        c -= 1;
    }

    debug_assert!(count == channels);
}

unsafe extern "C" fn atihdmi_pin_hbr_setup(
    codec: *mut hda_codec,
    pin_nid: hda_nid_t,
    dev_id: c_int,
    hbr: bool,
) -> c_int {
    debug_assert!(dev_id == 0);

    let hbr_ctl = snd_hda_codec_read(codec, pin_nid, 0, ATI_VERB_GET_HBR_CONTROL, 0);
    if hbr_ctl >= 0 && (hbr_ctl & ATI_HBR_CAPABLE) != 0 {
        let hbr_ctl_new = if hbr {
            hbr_ctl | ATI_HBR_ENABLE
        } else {
            hbr_ctl & !ATI_HBR_ENABLE
        };

        codec_dbg(
            codec,
            c"%s: NID=0x%x, %shbr-ctl=0x%x\n".as_ptr(),
            c"atihdmi_pin_hbr_setup".as_ptr(),
            pin_nid,
            if hbr_ctl == hbr_ctl_new {
                c"".as_ptr()
            } else {
                c"new-".as_ptr()
            },
            hbr_ctl_new,
        );

        if hbr_ctl != hbr_ctl_new {
            snd_hda_codec_write(
                codec,
                pin_nid,
                0,
                ATI_VERB_SET_HBR_CONTROL,
                hbr_ctl_new,
            );
        }
    } else if hbr {
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn atihdmi_setup_stream(
    codec: *mut hda_codec,
    cvt_nid: hda_nid_t,
    pin_nid: hda_nid_t,
    dev_id: c_int,
    stream_tag: u32,
    format: c_int,
) -> c_int {
    if is_amdhdmi_rev3_or_later(codec) {
        let mut ramp_rate = 180; /* default as per AMD spec */
        /* disable ramp-up/down for non-pcm as per AMD spec */
        if (format & AC_FMT_TYPE_NON_PCM) != 0 {
            ramp_rate = 0;
        }

        snd_hda_codec_write(codec, cvt_nid, 0, ATI_VERB_SET_RAMP_RATE, ramp_rate);
    }

    snd_hda_hdmi_setup_stream(codec, cvt_nid, pin_nid, dev_id, stream_tag, format)
}

unsafe extern "C" fn atihdmi_init(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec;

    let err = snd_hda_hdmi_generic_init(codec);

    if err != 0 {
        return err;
    }

    let mut pin_idx = 0;
    while pin_idx < (*spec).num_pins {
        let per_pin = get_pin(spec, pin_idx);

        /* make sure downmix information in infoframe is zero */
        snd_hda_codec_write(
            codec,
            (*per_pin).pin_nid,
            0,
            ATI_VERB_SET_DOWNMIX_INFO,
            0,
        );

        /* enable channel-wise remap mode if supported */
        if has_amd_full_remap_support(codec) {
            snd_hda_codec_write(
                codec,
                (*per_pin).pin_nid,
                0,
                ATI_VERB_SET_MULTICHANNEL_MODE,
                ATI_MULTICHANNEL_MODE_SINGLE,
            );
        }
        pin_idx += 1;
    }
    (*codec).auto_runtime_pm = 1;

    0
}

/* map from pin NID to port; port is 0-based */
/* for AMD: assume widget NID starting from 3, with step 2 (3, 5, 7, ...) */
unsafe extern "C" fn atihdmi_pin2port(_audio_ptr: *mut c_void, pin_nid: c_int) -> c_int {
    pin_nid / 2 - 1
}

/* reverse-map from port to pin NID: see above */
unsafe extern "C" fn atihdmi_port2pin(_codec: *mut hda_codec, port: c_int) -> c_int {
    port * 2 + 3
}

static atihdmi_audio_ops: drm_audio_component_audio_ops = drm_audio_component_audio_ops {
    pin2port: Some(atihdmi_pin2port),
    pin_eld_notify: Some(snd_hda_hdmi_acomp_pin_eld_notify),
    master_bind: Some(snd_hda_hdmi_acomp_master_bind),
    master_unbind: Some(snd_hda_hdmi_acomp_master_unbind),
};

unsafe extern "C" fn atihdmi_probe(
    codec: *mut hda_codec,
    _id: *const hda_device_id,
) -> c_int {
    let mut per_cvt: *mut hdmi_spec_per_cvt;

    let err = snd_hda_hdmi_generic_probe(codec);
    if err != 0 {
        return err;
    }

    let spec = (*codec).spec;

    (*spec).static_pcm_mapping = true;

    (*spec).ops.pin_get_eld = Some(atihdmi_pin_get_eld);
    (*spec).ops.pin_setup_infoframe = Some(atihdmi_pin_setup_infoframe);
    (*spec).ops.pin_hbr_setup = Some(atihdmi_pin_hbr_setup);
    (*spec).ops.setup_stream = Some(atihdmi_setup_stream);

    (*spec).chmap.ops.pin_get_slot_channel = Some(atihdmi_pin_get_slot_channel);
    (*spec).chmap.ops.pin_set_slot_channel = Some(atihdmi_pin_set_slot_channel);

    if !has_amd_full_remap_support(codec) {
        /* override to ATI/AMD-specific versions with pairwise mapping */
        (*spec).chmap.ops.chmap_cea_alloc_validate_get_type =
            Some(atihdmi_paired_chmap_cea_alloc_validate_get_type);
        (*spec).chmap.ops.cea_alloc_to_tlv_chmap = Some(atihdmi_paired_cea_alloc_to_tlv_chmap);
        (*spec).chmap.ops.chmap_validate = Some(atihdmi_paired_chmap_validate);
    }

    /* ATI/AMD converters do not advertise all of their capabilities */
    let mut cvt_idx = 0;
    while cvt_idx < (*spec).num_cvts {
        per_cvt = get_cvt(spec, cvt_idx);
        (*per_cvt).channels_max = core::cmp::max((*per_cvt).channels_max, 8u32);
        (*per_cvt).rates |= SUPPORTED_RATES;
        (*per_cvt).formats |= SUPPORTED_FORMATS;
        (*per_cvt).maxbps = core::cmp::max((*per_cvt).maxbps, 24u32);
        cvt_idx += 1;
    }

    (*spec).chmap.channels_max = core::cmp::max((*spec).chmap.channels_max, 8u32);

    /* AMD GPUs have neither EPSS nor CLKSTOP bits, hence preventing
     * the link-down as is.  Tell the core to allow it.
     */
    (*codec).link_down_at_suspend = 1;

    snd_hda_hdmi_acomp_init(codec, &atihdmi_audio_ops, Some(atihdmi_port2pin));

    0
}

static atihdmi_codec_ops: hda_codec_ops = hda_codec_ops {
    probe: Some(atihdmi_probe),
    remove: Some(snd_hda_hdmi_generic_remove),
    init: Some(atihdmi_init),
    build_pcms: Some(snd_hda_hdmi_generic_build_pcms),
    build_controls: Some(snd_hda_hdmi_generic_build_controls),
    unsol_event: Some(snd_hda_hdmi_generic_unsol_event),
    suspend: Some(snd_hda_hdmi_generic_suspend),
    resume: Some(snd_hda_hdmi_generic_resume),
};

/*
 * driver entries
 */
static snd_hda_id_atihdmi: [hda_device_id; 5] = [
    hda_device_id {
        vendor_id: 0x1002793c,
        rev_id: 0,
        api_version: 0,
        name: c"RS600 HDMI".as_ptr(),
        driver_data: 0,
    },
    hda_device_id {
        vendor_id: 0x10027919,
        rev_id: 0,
        api_version: 0,
        name: c"RS600 HDMI".as_ptr(),
        driver_data: 0,
    },
    hda_device_id {
        vendor_id: 0x1002791a,
        rev_id: 0,
        api_version: 0,
        name: c"RS690/780 HDMI".as_ptr(),
        driver_data: 0,
    },
    hda_device_id {
        vendor_id: 0x1002aa01,
        rev_id: 0,
        api_version: 0,
        name: c"R6xx HDMI".as_ptr(),
        driver_data: 0,
    },
    hda_device_id {
        vendor_id: 0,
        rev_id: 0,
        api_version: 0,
        name: core::ptr::null(),
        driver_data: 0,
    }, /* terminator */
];
// MODULE_DEVICE_TABLE(hdaudio, snd_hda_id_atihdmi);

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("AMD/ATI HDMI HD-audio codec");
// MODULE_IMPORT_NS("SND_HDA_CODEC_HDMI");

static mut atihdmi_driver: hda_codec_driver = hda_codec_driver {
    id: snd_hda_id_atihdmi.as_ptr(),
    ops: &atihdmi_codec_ops,
};

// module_hda_codec_driver(atihdmi_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
