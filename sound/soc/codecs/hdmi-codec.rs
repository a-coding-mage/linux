// SPDX-License-Identifier: GPL-2.0-only
/*
 * ALSA SoC codec for HDMI encoder drivers
 * Copyright (C) 2015 Texas Instruments Incorporated - https://www.ti.com/
 * Author: Jyri Sarha <jsarha@ti.com>
 */

// C dependencies intentionally left external:
// linux/cleanup.h, linux/module.h, linux/string.h
// sound/core.h, sound/jack.h, sound/pcm.h, sound/pcm_params.h, sound/soc.h
// sound/tlv.h, sound/pcm_drm_eld.h, sound/hdmi-codec.h, sound/pcm_iec958.h
// drm/drm_crtc.h (MAX_ELD_BYTES), drm/drm_eld.h

const HDMI_CODEC_CHMAP_IDX_UNKNOWN: i32 = -1;

const fn BIT(n: u32) -> c_ulong {
    1 as c_ulong << n
}

/*
 * CEA speaker placement for HDMI 1.4:
 *
 *  FL  FLC   FC   FRC   FR   FRW
 *
 *                                  LFE
 *
 *  RL  RLC   RC   RRC   RR
 *
 *  Speaker placement has to be extended to support HDMI 2.0
 */
const FL: c_ulong = BIT(0);  /* Front Left           */
const FC: c_ulong = BIT(1);  /* Front Center         */
const FR: c_ulong = BIT(2);  /* Front Right          */
const FLC: c_ulong = BIT(3); /* Front Left Center    */
const FRC: c_ulong = BIT(4); /* Front Right Center   */
const RL: c_ulong = BIT(5);  /* Rear Left            */
const RC: c_ulong = BIT(6);  /* Rear Center          */
const RR: c_ulong = BIT(7);  /* Rear Right           */
const RLC: c_ulong = BIT(8); /* Rear Left Center     */
const RRC: c_ulong = BIT(9); /* Rear Right Center    */
const LFE: c_ulong = BIT(10); /* Low Frequency Effect */

/*
 * cea Speaker allocation structure
 */
#[repr(C)]
struct hdmi_codec_cea_spk_alloc {
    ca_id: c_int,
    n_ch: c_uint,
    mask: c_ulong,
}

/* Channel maps  stereo HDMI */
static hdmi_codec_stereo_chmaps: [snd_pcm_chmap_elem; 2] = [
    snd_pcm_chmap_elem {
        channels: 2,
        map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, 0, 0, 0, 0, 0, 0],
    },
    snd_pcm_chmap_elem {
        channels: 0,
        map: [0; 8],
    },
];

/* Channel maps for multi-channel playbacks, up to 8 n_ch */
static hdmi_codec_8ch_chmaps: [snd_pcm_chmap_elem; 33] = [
    snd_pcm_chmap_elem { channels: 2, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, 0, 0, 0, 0, 0, 0] }, /* CA_ID 0x00 */
    snd_pcm_chmap_elem { channels: 4, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_LFE, SNDRV_CHMAP_NA, 0, 0, 0, 0] }, /* CA_ID 0x01 */
    snd_pcm_chmap_elem { channels: 4, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_NA, SNDRV_CHMAP_FC, 0, 0, 0, 0] }, /* CA_ID 0x02 */
    snd_pcm_chmap_elem { channels: 4, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_LFE, SNDRV_CHMAP_FC, 0, 0, 0, 0] }, /* CA_ID 0x03 */
    snd_pcm_chmap_elem { channels: 6, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_NA, SNDRV_CHMAP_NA, SNDRV_CHMAP_RC, SNDRV_CHMAP_NA, 0, 0] }, /* CA_ID 0x04 */
    snd_pcm_chmap_elem { channels: 6, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_LFE, SNDRV_CHMAP_NA, SNDRV_CHMAP_RC, SNDRV_CHMAP_NA, 0, 0] }, /* CA_ID 0x05 */
    snd_pcm_chmap_elem { channels: 6, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_NA, SNDRV_CHMAP_FC, SNDRV_CHMAP_RC, SNDRV_CHMAP_NA, 0, 0] }, /* CA_ID 0x06 */
    snd_pcm_chmap_elem { channels: 6, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_LFE, SNDRV_CHMAP_FC, SNDRV_CHMAP_RC, SNDRV_CHMAP_NA, 0, 0] }, /* CA_ID 0x07 */
    snd_pcm_chmap_elem { channels: 6, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_NA, SNDRV_CHMAP_NA, SNDRV_CHMAP_RL, SNDRV_CHMAP_RR, 0, 0] }, /* CA_ID 0x08 */
    snd_pcm_chmap_elem { channels: 6, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_LFE, SNDRV_CHMAP_NA, SNDRV_CHMAP_RL, SNDRV_CHMAP_RR, 0, 0] }, /* CA_ID 0x09 */
    snd_pcm_chmap_elem { channels: 6, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_NA, SNDRV_CHMAP_FC, SNDRV_CHMAP_RL, SNDRV_CHMAP_RR, 0, 0] }, /* CA_ID 0x0A */
    snd_pcm_chmap_elem { channels: 6, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_LFE, SNDRV_CHMAP_FC, SNDRV_CHMAP_RL, SNDRV_CHMAP_RR, 0, 0] }, /* CA_ID 0x0B */
    snd_pcm_chmap_elem { channels: 8, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_NA, SNDRV_CHMAP_NA, SNDRV_CHMAP_RL, SNDRV_CHMAP_RR, SNDRV_CHMAP_RC, SNDRV_CHMAP_NA] }, /* CA_ID 0x0C */
    snd_pcm_chmap_elem { channels: 8, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_LFE, SNDRV_CHMAP_NA, SNDRV_CHMAP_RL, SNDRV_CHMAP_RR, SNDRV_CHMAP_RC, SNDRV_CHMAP_NA] }, /* CA_ID 0x0D */
    snd_pcm_chmap_elem { channels: 8, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_NA, SNDRV_CHMAP_FC, SNDRV_CHMAP_RL, SNDRV_CHMAP_RR, SNDRV_CHMAP_RC, SNDRV_CHMAP_NA] }, /* CA_ID 0x0E */
    snd_pcm_chmap_elem { channels: 8, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_LFE, SNDRV_CHMAP_FC, SNDRV_CHMAP_RL, SNDRV_CHMAP_RR, SNDRV_CHMAP_RC, SNDRV_CHMAP_NA] }, /* CA_ID 0x0F */
    snd_pcm_chmap_elem { channels: 8, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_NA, SNDRV_CHMAP_NA, SNDRV_CHMAP_RL, SNDRV_CHMAP_RR, SNDRV_CHMAP_RLC, SNDRV_CHMAP_RRC] }, /* CA_ID 0x10 */
    snd_pcm_chmap_elem { channels: 8, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_LFE, SNDRV_CHMAP_NA, SNDRV_CHMAP_RL, SNDRV_CHMAP_RR, SNDRV_CHMAP_RLC, SNDRV_CHMAP_RRC] }, /* CA_ID 0x11 */
    snd_pcm_chmap_elem { channels: 8, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_NA, SNDRV_CHMAP_FC, SNDRV_CHMAP_RL, SNDRV_CHMAP_RR, SNDRV_CHMAP_RLC, SNDRV_CHMAP_RRC] }, /* CA_ID 0x12 */
    snd_pcm_chmap_elem { channels: 8, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_LFE, SNDRV_CHMAP_FC, SNDRV_CHMAP_RL, SNDRV_CHMAP_RR, SNDRV_CHMAP_RLC, SNDRV_CHMAP_RRC] }, /* CA_ID 0x13 */
    snd_pcm_chmap_elem { channels: 8, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_NA, SNDRV_CHMAP_NA, SNDRV_CHMAP_NA, SNDRV_CHMAP_NA, SNDRV_CHMAP_FLC, SNDRV_CHMAP_FRC] }, /* CA_ID 0x14 */
    snd_pcm_chmap_elem { channels: 8, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_LFE, SNDRV_CHMAP_NA, SNDRV_CHMAP_NA, SNDRV_CHMAP_NA, SNDRV_CHMAP_FLC, SNDRV_CHMAP_FRC] }, /* CA_ID 0x15 */
    snd_pcm_chmap_elem { channels: 8, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_NA, SNDRV_CHMAP_FC, SNDRV_CHMAP_NA, SNDRV_CHMAP_NA, SNDRV_CHMAP_FLC, SNDRV_CHMAP_FRC] }, /* CA_ID 0x16 */
    snd_pcm_chmap_elem { channels: 8, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_LFE, SNDRV_CHMAP_FC, SNDRV_CHMAP_NA, SNDRV_CHMAP_NA, SNDRV_CHMAP_FLC, SNDRV_CHMAP_FRC] }, /* CA_ID 0x17 */
    snd_pcm_chmap_elem { channels: 8, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_NA, SNDRV_CHMAP_NA, SNDRV_CHMAP_NA, SNDRV_CHMAP_NA, SNDRV_CHMAP_FLC, SNDRV_CHMAP_FRC] }, /* CA_ID 0x18 */
    snd_pcm_chmap_elem { channels: 8, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_LFE, SNDRV_CHMAP_NA, SNDRV_CHMAP_NA, SNDRV_CHMAP_NA, SNDRV_CHMAP_FLC, SNDRV_CHMAP_FRC] }, /* CA_ID 0x19 */
    snd_pcm_chmap_elem { channels: 8, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_NA, SNDRV_CHMAP_FC, SNDRV_CHMAP_NA, SNDRV_CHMAP_NA, SNDRV_CHMAP_FLC, SNDRV_CHMAP_FRC] }, /* CA_ID 0x1A */
    snd_pcm_chmap_elem { channels: 8, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_LFE, SNDRV_CHMAP_FC, SNDRV_CHMAP_NA, SNDRV_CHMAP_NA, SNDRV_CHMAP_FLC, SNDRV_CHMAP_FRC] }, /* CA_ID 0x1B */
    snd_pcm_chmap_elem { channels: 8, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_NA, SNDRV_CHMAP_NA, SNDRV_CHMAP_NA, SNDRV_CHMAP_NA, SNDRV_CHMAP_FLC, SNDRV_CHMAP_FRC] }, /* CA_ID 0x1C */
    snd_pcm_chmap_elem { channels: 8, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_LFE, SNDRV_CHMAP_NA, SNDRV_CHMAP_NA, SNDRV_CHMAP_NA, SNDRV_CHMAP_FLC, SNDRV_CHMAP_FRC] }, /* CA_ID 0x1D */
    snd_pcm_chmap_elem { channels: 8, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_NA, SNDRV_CHMAP_FC, SNDRV_CHMAP_NA, SNDRV_CHMAP_NA, SNDRV_CHMAP_FLC, SNDRV_CHMAP_FRC] }, /* CA_ID 0x1E */
    snd_pcm_chmap_elem { channels: 8, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_LFE, SNDRV_CHMAP_FC, SNDRV_CHMAP_NA, SNDRV_CHMAP_NA, SNDRV_CHMAP_FLC, SNDRV_CHMAP_FRC] }, /* CA_ID 0x1F */
    snd_pcm_chmap_elem { channels: 0, map: [0; 8] },
];

/*
 * hdmi_codec_channel_alloc: speaker configuration available for CEA
 *
 * This is an ordered list where ca_id must exist in hdmi_codec_8ch_chmaps
 * The preceding ones have better chances to be selected by
 * hdmi_codec_get_ch_alloc_table_idx().
 */
static hdmi_codec_channel_alloc: [hdmi_codec_cea_spk_alloc; 43] = [
    hdmi_codec_cea_spk_alloc { ca_id: 0x00, n_ch: 2, mask: FL | FR },
    hdmi_codec_cea_spk_alloc { ca_id: 0x03, n_ch: 4, mask: FL | FR | LFE | FC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x02, n_ch: 4, mask: FL | FR | FC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x01, n_ch: 4, mask: FL | FR | LFE },
    hdmi_codec_cea_spk_alloc { ca_id: 0x0b, n_ch: 6, mask: FL | FR | LFE | FC | RL | RR },
    hdmi_codec_cea_spk_alloc { ca_id: 0x0a, n_ch: 6, mask: FL | FR | FC | RL | RR },
    hdmi_codec_cea_spk_alloc { ca_id: 0x09, n_ch: 6, mask: FL | FR | LFE | RL | RR },
    hdmi_codec_cea_spk_alloc { ca_id: 0x08, n_ch: 6, mask: FL | FR | RL | RR },
    hdmi_codec_cea_spk_alloc { ca_id: 0x07, n_ch: 6, mask: FL | FR | LFE | FC | RC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x06, n_ch: 6, mask: FL | FR | FC | RC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x05, n_ch: 6, mask: FL | FR | LFE | RC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x04, n_ch: 6, mask: FL | FR | RC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x13, n_ch: 8, mask: FL | FR | LFE | FC | RL | RR | RLC | RRC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x1f, n_ch: 8, mask: FL | FR | LFE | FC | RL | RR | FLC | FRC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x12, n_ch: 8, mask: FL | FR | FC | RL | RR | RLC | RRC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x1e, n_ch: 8, mask: FL | FR | FC | RL | RR | FLC | FRC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x11, n_ch: 8, mask: FL | FR | LFE | RL | RR | RLC | RRC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x1d, n_ch: 8, mask: FL | FR | LFE | RL | RR | FLC | FRC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x10, n_ch: 8, mask: FL | FR | RL | RR | RLC | RRC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x1c, n_ch: 8, mask: FL | FR | RL | RR | FLC | FRC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x0f, n_ch: 8, mask: FL | FR | LFE | FC | RL | RR | RC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x1b, n_ch: 8, mask: FL | FR | LFE | RC | FC | FLC | FRC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x0e, n_ch: 8, mask: FL | FR | FC | RL | RR | RC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x1a, n_ch: 8, mask: FL | FR | RC | FC | FLC | FRC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x0d, n_ch: 8, mask: FL | FR | LFE | RL | RR | RC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x19, n_ch: 8, mask: FL | FR | LFE | RC | FLC | FRC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x0c, n_ch: 8, mask: FL | FR | RC | RL | RR },
    hdmi_codec_cea_spk_alloc { ca_id: 0x18, n_ch: 8, mask: FL | FR | RC | FLC | FRC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x17, n_ch: 8, mask: FL | FR | LFE | FC | FLC | FRC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x16, n_ch: 8, mask: FL | FR | FC | FLC | FRC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x15, n_ch: 8, mask: FL | FR | LFE | FLC | FRC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x14, n_ch: 8, mask: FL | FR | FLC | FRC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x0b, n_ch: 8, mask: FL | FR | LFE | FC | RL | RR },
    hdmi_codec_cea_spk_alloc { ca_id: 0x0a, n_ch: 8, mask: FL | FR | FC | RL | RR },
    hdmi_codec_cea_spk_alloc { ca_id: 0x09, n_ch: 8, mask: FL | FR | LFE | RL | RR },
    hdmi_codec_cea_spk_alloc { ca_id: 0x08, n_ch: 8, mask: FL | FR | RL | RR },
    hdmi_codec_cea_spk_alloc { ca_id: 0x07, n_ch: 8, mask: FL | FR | LFE | FC | RC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x06, n_ch: 8, mask: FL | FR | FC | RC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x05, n_ch: 8, mask: FL | FR | LFE | RC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x04, n_ch: 8, mask: FL | FR | RC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x03, n_ch: 8, mask: FL | FR | LFE | FC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x02, n_ch: 8, mask: FL | FR | FC },
    hdmi_codec_cea_spk_alloc { ca_id: 0x01, n_ch: 8, mask: FL | FR | LFE },
];

#[repr(C)]
struct hdmi_codec_priv {
    hcd: hdmi_codec_pdata,
    eld: [u8; MAX_ELD_BYTES],
    eld_parsed: snd_parsed_hdmi_eld,
    chmap_info: *mut snd_pcm_chmap,
    chmap_idx: c_uint,
    lock: mutex,
    busy: bool,
    jack: *mut snd_soc_jack,
    jack_status: c_uint,
    iec_status: [u8; AES_IEC958_STATUS_SIZE],
    proc_entry: *mut snd_info_entry,
}

static hdmi_widgets: [snd_soc_dapm_widget; 2] = [
    SND_SOC_DAPM_OUTPUT!("TX"),
    SND_SOC_DAPM_OUTPUT!("RX"),
];

const DAI_ID_I2S: c_int = 0;
const DAI_ID_SPDIF: c_int = 1;

unsafe extern "C" fn hdmi_eld_ctl_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BYTES;
        (*uinfo).count = core::mem::size_of::<[u8; MAX_ELD_BYTES]>() as _;
    }
    0
}

unsafe extern "C" fn hdmi_eld_ctl_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
        let hcp: *mut hdmi_codec_priv = snd_soc_component_get_drvdata(component);
        memcpy(
            (*ucontrol).value.bytes.data.as_mut_ptr() as *mut c_void,
            (*hcp).eld.as_ptr() as *const c_void,
            core::mem::size_of_val(&(*hcp).eld),
        );
    }
    0
}

fn hdmi_codec_spk_mask_from_alloc(spk_alloc: c_int) -> c_ulong {
    static hdmi_codec_eld_spk_alloc_bits: [c_ulong; 7] = [
        FL | FR,
        LFE,
        FC,
        RL | RR,
        RC,
        FLC | FRC,
        RLC | RRC,
    ];
    let mut spk_mask: c_ulong = 0;

    for i in 0..hdmi_codec_eld_spk_alloc_bits.len() {
        if spk_alloc & (1 << i) != 0 {
            spk_mask |= hdmi_codec_eld_spk_alloc_bits[i];
        }
    }

    spk_mask
}

unsafe fn hdmi_codec_eld_chmap(hcp: *mut hdmi_codec_priv) {
    unsafe {
        let spk_alloc: u8 = drm_eld_get_spk_alloc((*hcp).eld.as_mut_ptr());
        let spk_mask = hdmi_codec_spk_mask_from_alloc(spk_alloc as c_int);

        /* Detect if only stereo supported, else return 8 channels mappings */
        if (spk_mask & !(FL | FR)) != 0 && (*(*hcp).chmap_info).max_channels > 2 {
            (*(*hcp).chmap_info).chmap = hdmi_codec_8ch_chmaps.as_ptr();
        } else {
            (*(*hcp).chmap_info).chmap = hdmi_codec_stereo_chmaps.as_ptr();
        }
    }
}

unsafe fn hdmi_codec_get_ch_alloc_table_idx(
    hcp: *mut hdmi_codec_priv,
    channels: c_uchar,
) -> c_int {
    unsafe {
        let spk_alloc: u8 = drm_eld_get_spk_alloc((*hcp).eld.as_mut_ptr());
        let spk_mask = hdmi_codec_spk_mask_from_alloc(spk_alloc as c_int);

        for (i, cap) in hdmi_codec_channel_alloc.iter().enumerate() {
            /* If spk_alloc == 0, HDMI is unplugged return stereo config*/
            if spk_alloc == 0 && cap.ca_id == 0 {
                return i as c_int;
            }
            if cap.n_ch != channels as c_uint {
                continue;
            }
            if cap.mask != (spk_mask & cap.mask) {
                continue;
            }
            return i as c_int;
        }

        -EINVAL
    }
}

unsafe extern "C" fn hdmi_codec_chmap_ctl_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let mut map: *const c_uchar = core::ptr::null();
        let info: *mut snd_pcm_chmap = snd_kcontrol_chip(kcontrol);
        let hcp: *mut hdmi_codec_priv = (*info).private_data as *mut hdmi_codec_priv;

        if (*hcp).chmap_idx as c_int != HDMI_CODEC_CHMAP_IDX_UNKNOWN {
            map = (*(*info).chmap.add((*hcp).chmap_idx as usize)).map.as_ptr();
        }

        for i in 0..(*info).max_channels as usize {
            if (*hcp).chmap_idx as c_int == HDMI_CODEC_CHMAP_IDX_UNKNOWN {
                (*ucontrol).value.integer.value[i] = 0;
            } else {
                (*ucontrol).value.integer.value[i] = *map.add(i) as _;
            }
        }
    }
    0
}

unsafe extern "C" fn hdmi_codec_iec958_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
        (*uinfo).count = 1;
    }
    0
}

unsafe extern "C" fn hdmi_codec_iec958_default_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
        let hcp: *mut hdmi_codec_priv = snd_soc_component_get_drvdata(component);
        memcpy(
            (*ucontrol).value.iec958.status.as_mut_ptr() as *mut c_void,
            (*hcp).iec_status.as_ptr() as *const c_void,
            core::mem::size_of_val(&(*hcp).iec_status),
        );
    }
    0
}

unsafe extern "C" fn hdmi_codec_iec958_default_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
        let hcp: *mut hdmi_codec_priv = snd_soc_component_get_drvdata(component);
        memcpy(
            (*hcp).iec_status.as_mut_ptr() as *mut c_void,
            (*ucontrol).value.iec958.status.as_ptr() as *const c_void,
            core::mem::size_of_val(&(*hcp).iec_status),
        );
    }
    0
}

unsafe extern "C" fn hdmi_codec_iec958_mask_get(
    _kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        memset(
            (*ucontrol).value.iec958.status.as_mut_ptr() as *mut c_void,
            0xff,
            AES_IEC958_STATUS_SIZE,
        );
    }
    0
}

unsafe extern "C" fn hdmi_codec_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    unsafe {
        let hcp: *mut hdmi_codec_priv = snd_soc_dai_get_drvdata(dai);
        let tx = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
        let has_capture = !(*hcp).hcd.no_i2s_capture;
        let has_playback = !(*hcp).hcd.no_i2s_playback;
        let mut ret: c_int = 0;

        if !((has_playback && tx) || (has_capture && !tx)) {
            return 0;
        }

        mutex_lock(&mut (*hcp).lock);
        if (*hcp).busy {
            dev_err((*dai).dev, c"Only one simultaneous stream supported!\n".as_ptr());
            mutex_unlock(&mut (*hcp).lock);
            return -EINVAL;
        }

        if let Some(audio_startup) = (*(*hcp).hcd.ops).audio_startup {
            ret = audio_startup((*(*dai).dev).parent, (*hcp).hcd.data);
            if ret != 0 {
                mutex_unlock(&mut (*hcp).lock);
                return ret;
            }
        }

        if tx {
            if let Some(get_eld) = (*(*hcp).hcd.ops).get_eld {
                ret = get_eld(
                    (*(*dai).dev).parent,
                    (*hcp).hcd.data,
                    (*hcp).eld.as_mut_ptr(),
                    core::mem::size_of_val(&(*hcp).eld),
                );
                if ret != 0 {
                    mutex_unlock(&mut (*hcp).lock);
                    return ret;
                }

                snd_parse_eld(
                    (*dai).dev,
                    &mut (*hcp).eld_parsed,
                    (*hcp).eld.as_mut_ptr(),
                    core::mem::size_of_val(&(*hcp).eld),
                );

                ret = snd_pcm_hw_constraint_eld((*substream).runtime, (*hcp).eld.as_mut_ptr());
                if ret != 0 {
                    mutex_unlock(&mut (*hcp).lock);
                    return ret;
                }

                /* Select chmap supported */
                hdmi_codec_eld_chmap(hcp);
            }
        }

        (*hcp).busy = true;
        mutex_unlock(&mut (*hcp).lock);

        ret
    }
}

unsafe extern "C" fn hdmi_codec_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    unsafe {
        let hcp: *mut hdmi_codec_priv = snd_soc_dai_get_drvdata(dai);
        let tx = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
        let has_capture = !(*hcp).hcd.no_i2s_capture;
        let has_playback = !(*hcp).hcd.no_i2s_playback;

        if !((has_playback && tx) || (has_capture && !tx)) {
            return;
        }

        (*hcp).chmap_idx = HDMI_CODEC_CHMAP_IDX_UNKNOWN as c_uint;
        (*(*hcp).hcd.ops).audio_shutdown.unwrap()((*(*dai).dev).parent, (*hcp).hcd.data);

        mutex_lock(&mut (*hcp).lock);
        (*hcp).busy = false;
        mutex_unlock(&mut (*hcp).lock);
    }
}

unsafe fn hdmi_codec_fill_codec_params(
    dai: *mut snd_soc_dai,
    sample_width: c_uint,
    sample_rate: c_uint,
    channels: c_uint,
    hp: *mut hdmi_codec_params,
) -> c_int {
    unsafe {
        let hcp: *mut hdmi_codec_priv = snd_soc_dai_get_drvdata(dai);
        let mut idx: c_int = HDMI_CODEC_CHMAP_IDX_UNKNOWN;
        let mut ca_id: u8 = 0;
        let pcm_audio = ((*hcp).iec_status[0] & IEC958_AES0_NONAUDIO) == 0;

        if pcm_audio {
            /* Select a channel allocation that matches with ELD and pcm channels */
            idx = hdmi_codec_get_ch_alloc_table_idx(hcp, channels as c_uchar);

            if idx < 0 {
                dev_err(
                    (*dai).dev,
                    c"Not able to map channels to speakers (%d)\n".as_ptr(),
                    idx,
                );
                (*hcp).chmap_idx = HDMI_CODEC_CHMAP_IDX_UNKNOWN as c_uint;
                return idx;
            }

            ca_id = hdmi_codec_channel_alloc[idx as usize].ca_id as u8;
        }

        memset(hp as *mut c_void, 0, core::mem::size_of::<hdmi_codec_params>());

        hdmi_audio_infoframe_init(&mut (*hp).cea);

        if pcm_audio {
            (*hp).cea.channels = channels;
        } else {
            (*hp).cea.channels = 0;
        }

        (*hp).cea.coding_type = HDMI_AUDIO_CODING_TYPE_STREAM;
        (*hp).cea.sample_size = HDMI_AUDIO_SAMPLE_SIZE_STREAM;
        (*hp).cea.sample_frequency = HDMI_AUDIO_SAMPLE_FREQUENCY_STREAM;
        (*hp).cea.channel_allocation = ca_id;

        (*hp).sample_width = sample_width;
        (*hp).sample_rate = sample_rate;
        (*hp).channels = channels;

        if pcm_audio {
            (*hcp).chmap_idx = ca_id as c_uint;
        } else {
            (*hcp).chmap_idx = HDMI_CODEC_CHMAP_IDX_UNKNOWN as c_uint;
        }

        0
    }
}

unsafe extern "C" fn hdmi_codec_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    unsafe {
        let hcp: *mut hdmi_codec_priv = snd_soc_dai_get_drvdata(dai);
        let cf: *mut hdmi_codec_daifmt = snd_soc_dai_dma_data_get_playback(dai);
        let mut hp: hdmi_codec_params = core::mem::zeroed();
        let mut ret: c_int;

        if (*(*hcp).hcd.ops).hw_params.is_none() {
            return 0;
        }

        dev_dbg(
            (*dai).dev,
            c"%s() width %d rate %d channels %d\n".as_ptr(),
            c"hdmi_codec_hw_params".as_ptr(),
            params_width(params),
            params_rate(params),
            params_channels(params),
        );

        ret = hdmi_codec_fill_codec_params(
            dai,
            params_width(params),
            params_rate(params),
            params_channels(params),
            &mut hp,
        );
        if ret < 0 {
            return ret;
        }

        memcpy(
            hp.iec.status.as_mut_ptr() as *mut c_void,
            (*hcp).iec_status.as_ptr() as *const c_void,
            core::mem::size_of_val(&hp.iec.status),
        );
        ret = snd_pcm_fill_iec958_consumer_hw_params(
            params,
            hp.iec.status.as_mut_ptr(),
            core::mem::size_of_val(&hp.iec.status),
        );
        if ret < 0 {
            dev_err((*dai).dev, c"Creating IEC958 channel status failed %d\n".as_ptr(), ret);
            return ret;
        }

        (*cf).bit_fmt = params_format(params);
        (*(*hcp).hcd.ops).hw_params.unwrap()((*(*dai).dev).parent, (*hcp).hcd.data, cf, &mut hp)
    }
}

unsafe extern "C" fn hdmi_codec_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    unsafe {
        let hcp: *mut hdmi_codec_priv = snd_soc_dai_get_drvdata(dai);
        let cf: *mut hdmi_codec_daifmt = snd_soc_dai_dma_data_get_playback(dai);
        let runtime: *mut snd_pcm_runtime = (*substream).runtime;
        let channels = (*runtime).channels;
        let width = snd_pcm_format_width((*runtime).format);
        let rate = (*runtime).rate;
        let mut hp: hdmi_codec_params = core::mem::zeroed();
        let mut ret: c_int;

        if (*(*hcp).hcd.ops).prepare.is_none() {
            return 0;
        }

        dev_dbg(
            (*dai).dev,
            c"%s() width %d rate %d channels %d\n".as_ptr(),
            c"hdmi_codec_prepare".as_ptr(),
            width,
            rate,
            channels,
        );

        ret = hdmi_codec_fill_codec_params(dai, width, rate, channels, &mut hp);
        if ret < 0 {
            return ret;
        }

        memcpy(
            hp.iec.status.as_mut_ptr() as *mut c_void,
            (*hcp).iec_status.as_ptr() as *const c_void,
            core::mem::size_of_val(&hp.iec.status),
        );
        ret = snd_pcm_fill_iec958_consumer(
            runtime,
            hp.iec.status.as_mut_ptr(),
            core::mem::size_of_val(&hp.iec.status),
        );
        if ret < 0 {
            dev_err((*dai).dev, c"Creating IEC958 channel status failed %d\n".as_ptr(), ret);
            return ret;
        }

        (*cf).bit_fmt = (*runtime).format;
        (*(*hcp).hcd.ops).prepare.unwrap()((*(*dai).dev).parent, (*hcp).hcd.data, cf, &mut hp)
    }
}

unsafe extern "C" fn hdmi_codec_i2s_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    unsafe {
        let cf: *mut hdmi_codec_daifmt = snd_soc_dai_dma_data_get_playback(dai);

        /* Reset daifmt */
        memset(cf as *mut c_void, 0, core::mem::size_of::<hdmi_codec_daifmt>());

        match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
            SND_SOC_DAIFMT_CBP_CFP => {
                (*cf).bit_clk_provider = 1;
                (*cf).frame_clk_provider = 1;
            }
            SND_SOC_DAIFMT_CBC_CFP => (*cf).frame_clk_provider = 1,
            SND_SOC_DAIFMT_CBP_CFC => (*cf).bit_clk_provider = 1,
            SND_SOC_DAIFMT_CBC_CFC => {}
            _ => return -EINVAL,
        }

        match fmt & SND_SOC_DAIFMT_INV_MASK {
            SND_SOC_DAIFMT_NB_NF => {}
            SND_SOC_DAIFMT_NB_IF => (*cf).frame_clk_inv = 1,
            SND_SOC_DAIFMT_IB_NF => (*cf).bit_clk_inv = 1,
            SND_SOC_DAIFMT_IB_IF => {
                (*cf).frame_clk_inv = 1;
                (*cf).bit_clk_inv = 1;
            }
            _ => {}
        }

        match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
            SND_SOC_DAIFMT_I2S => (*cf).fmt = HDMI_I2S,
            SND_SOC_DAIFMT_DSP_A => (*cf).fmt = HDMI_DSP_A,
            SND_SOC_DAIFMT_DSP_B => (*cf).fmt = HDMI_DSP_B,
            SND_SOC_DAIFMT_RIGHT_J => (*cf).fmt = HDMI_RIGHT_J,
            SND_SOC_DAIFMT_LEFT_J => (*cf).fmt = HDMI_LEFT_J,
            SND_SOC_DAIFMT_AC97 => (*cf).fmt = HDMI_AC97,
            _ => {
                dev_err((*dai).dev, c"Invalid DAI interface format\n".as_ptr());
                return -EINVAL;
            }
        }

        0
    }
}

unsafe extern "C" fn hdmi_codec_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    direction: c_int,
) -> c_int {
    unsafe {
        let hcp: *mut hdmi_codec_priv = snd_soc_dai_get_drvdata(dai);

        /*
         * ignore if direction was CAPTURE
         * and it had .no_capture_mute flag
         * see
         *      snd_soc_dai_digital_mute()
         */
        if let Some(mute_stream) = (*(*hcp).hcd.ops).mute_stream {
            if direction == SNDRV_PCM_STREAM_PLAYBACK || !(*hcp).hcd.no_capture_mute {
                return mute_stream((*(*dai).dev).parent, (*hcp).hcd.data, mute, direction);
            }
        }

        -ENOTSUPP
    }
}

/*
 * This driver can select all SND_SOC_DAIFMT_CBx_CFx,
 * but need to be selected from Sound Card, not be auto selected.
 * Because it might be used from other driver.
 * For example,
 *      ${LINUX}/drivers/gpu/drm/bridge/synopsys/dw-hdmi-i2s-audio.c
 */
static hdmi_codec_formats: u64 =
    SND_SOC_POSSIBLE_DAIFMT_NB_NF |
    SND_SOC_POSSIBLE_DAIFMT_NB_IF |
    SND_SOC_POSSIBLE_DAIFMT_IB_NF |
    SND_SOC_POSSIBLE_DAIFMT_IB_IF |
    SND_SOC_POSSIBLE_DAIFMT_I2S |
    SND_SOC_POSSIBLE_DAIFMT_DSP_A |
    SND_SOC_POSSIBLE_DAIFMT_DSP_B |
    SND_SOC_POSSIBLE_DAIFMT_RIGHT_J |
    SND_SOC_POSSIBLE_DAIFMT_LEFT_J |
    SND_SOC_POSSIBLE_DAIFMT_AC97;

const HDMI_RATES: c_uint = SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 |
    SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_88200 |
    SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_176400 |
    SNDRV_PCM_RATE_192000;

const SPDIF_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE |
    SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_S24_LE;

/*
 * This list is only for formats allowed on the I2S bus. So there is
 * some formats listed that are not supported by HDMI interface. For
 * instance allowing the 32-bit formats enables 24-precision with CPU
 * DAIs that do not support 24-bit formats. If the extra formats cause
 * problems, we should add the video side driver an option to disable
 * them.
 */
const I2S_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE |
    SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_S24_LE |
    SNDRV_PCM_FMTBIT_S32_LE | SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE;

static mut hdmi_codec_controls: [snd_kcontrol_new; 3] = [
    snd_kcontrol_new {
        access: SNDRV_CTL_ELEM_ACCESS_READ,
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: SNDRV_CTL_NAME_IEC958!("", PLAYBACK, MASK),
        info: Some(hdmi_codec_iec958_info),
        get: Some(hdmi_codec_iec958_mask_get),
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: SNDRV_CTL_NAME_IEC958!("", PLAYBACK, DEFAULT),
        info: Some(hdmi_codec_iec958_info),
        get: Some(hdmi_codec_iec958_default_get),
        put: Some(hdmi_codec_iec958_default_put),
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: c"ELD".as_ptr(),
        info: Some(hdmi_eld_ctl_info),
        get: Some(hdmi_eld_ctl_get),
        ..unsafe { core::mem::zeroed() }
    },
];

unsafe extern "C" fn hdmi_codec_pcm_new(
    rtd: *mut snd_soc_pcm_runtime,
    dai: *mut snd_soc_dai,
) -> c_int {
    unsafe {
        let drv: *mut snd_soc_dai_driver = (*dai).driver;
        let hcp: *mut hdmi_codec_priv = snd_soc_dai_get_drvdata(dai);
        let mut ret: c_int;

        ret = snd_pcm_add_chmap_ctls(
            (*rtd).pcm,
            SNDRV_PCM_STREAM_PLAYBACK,
            core::ptr::null(),
            (*drv).playback.channels_max,
            0,
            &mut (*hcp).chmap_info,
        );
        if ret < 0 {
            return ret;
        }

        /* override handlers */
        (*(*hcp).chmap_info).private_data = hcp as *mut c_void;
        (*(*(*hcp).chmap_info).kctl).get = Some(hdmi_codec_chmap_ctl_get);

        /* default chmap supported is stereo */
        (*(*hcp).chmap_info).chmap = hdmi_codec_stereo_chmaps.as_ptr();
        (*hcp).chmap_idx = HDMI_CODEC_CHMAP_IDX_UNKNOWN as c_uint;

        for i in 0..hdmi_codec_controls.len() {
            let kctl: *mut snd_kcontrol;

            /* add ELD ctl with the device number corresponding to the PCM stream */
            kctl = snd_ctl_new1(&hdmi_codec_controls[i], (*dai).component);
            if kctl.is_null() {
                return -ENOMEM;
            }

            (*kctl).id.device = (*(*rtd).pcm).device;
            ret = snd_ctl_add((*(*rtd).card).snd_card, kctl);
            if ret < 0 {
                return ret;
            }
        }

        0
    }
}

// CONFIG_SND_PROC_FS conditional code from the C source.
#[cfg(CONFIG_SND_PROC_FS)]
unsafe extern "C" fn print_eld_info(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    unsafe {
        let hcp: *mut hdmi_codec_priv = (*entry).private_data as *mut hdmi_codec_priv;
        snd_print_eld_info(&mut (*hcp).eld_parsed, buffer);
    }
}

#[cfg(CONFIG_SND_PROC_FS)]
unsafe fn hdmi_dai_proc_new(hcp: *mut hdmi_codec_priv, dai: *mut snd_soc_dai) -> c_int {
    unsafe {
        let component: *mut snd_soc_component = (*dai).component;
        let card: *mut snd_soc_card = (*component).card;
        let mut entry: *mut snd_info_entry = core::ptr::null_mut();
        let mut name: [c_char; 32] = [0; 32];
        let mut id: c_int = 0;

        /* The C source uses for_each_card_rtds()/for_each_rtd_dais() macros here
         * to find the matching runtime and use rtd->id instead of dai->id. */
        for_each_card_rtds_and_rtd_dais_find_id(card, dai, &mut id);

        snprintf(name.as_mut_ptr(), name.len(), c"eld#%d".as_ptr(), id);
        let err = snd_card_proc_new((*card).snd_card, name.as_mut_ptr(), &mut entry);
        if err < 0 {
            return err;
        }

        snd_info_set_text_ops(entry, hcp as *mut c_void, Some(print_eld_info));
        (*hcp).proc_entry = entry;

        0
    }
}

#[cfg(CONFIG_SND_PROC_FS)]
unsafe fn hdmi_dai_proc_free(hcp: *mut hdmi_codec_priv) {
    unsafe {
        snd_info_free_entry((*hcp).proc_entry);
        (*hcp).proc_entry = core::ptr::null_mut();
    }
}

#[cfg(not(CONFIG_SND_PROC_FS))]
unsafe fn hdmi_dai_proc_new(_hcp: *mut hdmi_codec_priv, _dai: *mut snd_soc_dai) -> c_int {
    0
}

#[cfg(not(CONFIG_SND_PROC_FS))]
unsafe fn hdmi_dai_proc_free(_hcp: *mut hdmi_codec_priv) {}

unsafe extern "C" fn hdmi_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    unsafe {
        let hcp: *mut hdmi_codec_priv = snd_soc_component_get_drvdata((*dai).component);
        let dapm: *mut snd_soc_dapm_context;
        let daifmt: *mut hdmi_codec_daifmt;
        let mut route: [snd_soc_dapm_route; 2] = [
            snd_soc_dapm_route {
                sink: c"TX".as_ptr(),
                source: (*(*dai).driver).playback.stream_name,
                ..core::mem::zeroed()
            },
            snd_soc_dapm_route {
                sink: (*(*dai).driver).capture.stream_name,
                source: c"RX".as_ptr(),
                ..core::mem::zeroed()
            },
        ];
        let mut ret: c_int;

        dapm = snd_soc_component_to_dapm((*dai).component);

        /* One of the directions might be omitted for unidirectional DAIs */
        for i in 0..route.len() {
            if route[i].source.is_null() || route[i].sink.is_null() {
                continue;
            }

            ret = snd_soc_dapm_add_routes(dapm, &mut route[i], 1);
            if ret != 0 {
                return ret;
            }
        }

        daifmt = devm_kzalloc(
            (*dai).dev,
            core::mem::size_of::<hdmi_codec_daifmt>(),
            GFP_KERNEL,
        ) as *mut hdmi_codec_daifmt;
        if daifmt.is_null() {
            return -ENOMEM;
        }

        snd_soc_dai_dma_data_set_playback(dai, daifmt);

        hdmi_dai_proc_new(hcp, dai)
    }
}

unsafe extern "C" fn hdmi_dai_remove(dai: *mut snd_soc_dai) -> c_int {
    unsafe {
        let hcp: *mut hdmi_codec_priv = snd_soc_component_get_drvdata((*dai).component);
        hdmi_dai_proc_free(hcp);
    }
    0
}

unsafe fn hdmi_codec_jack_report(hcp: *mut hdmi_codec_priv, jack_status: c_uint) {
    unsafe {
        if jack_status != (*hcp).jack_status {
            if !(*hcp).jack.is_null() {
                snd_soc_jack_report((*hcp).jack, jack_status, SND_JACK_AVOUT);
            }
            (*hcp).jack_status = jack_status;
        }
    }
}

unsafe extern "C" fn plugged_cb(dev: *mut device, plugged: bool) {
    unsafe {
        let hcp: *mut hdmi_codec_priv = dev_get_drvdata(dev);
        let mut ret: c_int;

        if plugged {
            if let Some(get_eld) = (*(*hcp).hcd.ops).get_eld {
                get_eld(
                    (*dev).parent,
                    (*hcp).hcd.data,
                    (*hcp).eld.as_mut_ptr(),
                    core::mem::size_of_val(&(*hcp).eld),
                );
                ret = snd_parse_eld(
                    dev,
                    &mut (*hcp).eld_parsed,
                    (*hcp).eld.as_mut_ptr(),
                    core::mem::size_of_val(&(*hcp).eld),
                );
                if ret < 0 {
                    dev_dbg(dev, c"Failed to parse ELD: %d\n".as_ptr(), ret);
                } else {
                    snd_show_eld(dev, &mut (*hcp).eld_parsed);
                }
            }
            hdmi_codec_jack_report(hcp, SND_JACK_AVOUT);
        } else {
            hdmi_codec_jack_report(hcp, 0);
            memset(
                (*hcp).eld.as_mut_ptr() as *mut c_void,
                0,
                core::mem::size_of_val(&(*hcp).eld),
            );
        }
    }
}

unsafe extern "C" fn hdmi_codec_set_jack(
    component: *mut snd_soc_component,
    jack: *mut snd_soc_jack,
    _data: *mut c_void,
) -> c_int {
    unsafe {
        let hcp: *mut hdmi_codec_priv = snd_soc_component_get_drvdata(component);

        if (*(*hcp).hcd.ops).hook_plugged_cb.is_some() {
            (*hcp).jack = jack;

            /*
             * Report the initial jack status which may have been provided
             * by the parent hdmi driver while the hpd hook was registered.
             */
            snd_soc_jack_report(jack, (*hcp).jack_status, SND_JACK_AVOUT);

            return 0;
        }

        -ENOTSUPP
    }
}

unsafe extern "C" fn hdmi_dai_spdif_probe(dai: *mut snd_soc_dai) -> c_int {
    unsafe {
        let ret = hdmi_dai_probe(dai);
        if ret != 0 {
            return ret;
        }

        let cf: *mut hdmi_codec_daifmt = snd_soc_dai_dma_data_get_playback(dai);
        (*cf).fmt = HDMI_SPDIF;

        0
    }
}

static hdmi_codec_i2s_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(hdmi_dai_probe),
    remove: Some(hdmi_dai_remove),
    startup: Some(hdmi_codec_startup),
    shutdown: Some(hdmi_codec_shutdown),
    hw_params: Some(hdmi_codec_hw_params),
    prepare: Some(hdmi_codec_prepare),
    set_fmt: Some(hdmi_codec_i2s_set_fmt),
    mute_stream: Some(hdmi_codec_mute),
    pcm_new: Some(hdmi_codec_pcm_new),
    auto_selectable_formats: &hdmi_codec_formats,
    num_auto_selectable_formats: 1,
    ..unsafe { core::mem::zeroed() }
};

static hdmi_codec_spdif_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(hdmi_dai_spdif_probe),
    startup: Some(hdmi_codec_startup),
    shutdown: Some(hdmi_codec_shutdown),
    hw_params: Some(hdmi_codec_hw_params),
    prepare: Some(hdmi_codec_prepare),
    mute_stream: Some(hdmi_codec_mute),
    pcm_new: Some(hdmi_codec_pcm_new),
    ..unsafe { core::mem::zeroed() }
};

static hdmi_i2s_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"i2s-hifi".as_ptr(),
    id: DAI_ID_I2S,
    playback: snd_soc_pcm_stream {
        stream_name: c"I2S Playback".as_ptr(),
        channels_min: 2,
        channels_max: 8,
        rates: HDMI_RATES,
        formats: I2S_FORMATS,
        sig_bits: 24,
        ..unsafe { core::mem::zeroed() }
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 2,
        channels_max: 8,
        rates: HDMI_RATES,
        formats: I2S_FORMATS,
        sig_bits: 24,
        ..unsafe { core::mem::zeroed() }
    },
    ops: &hdmi_codec_i2s_dai_ops,
    ..unsafe { core::mem::zeroed() }
};

static hdmi_spdif_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"spdif-hifi".as_ptr(),
    id: DAI_ID_SPDIF,
    playback: snd_soc_pcm_stream {
        stream_name: c"SPDIF Playback".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: HDMI_RATES,
        formats: SPDIF_FORMATS,
        ..unsafe { core::mem::zeroed() }
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: HDMI_RATES,
        formats: SPDIF_FORMATS,
        ..unsafe { core::mem::zeroed() }
    },
    ops: &hdmi_codec_spdif_dai_ops,
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn hdmi_of_xlate_dai_id(
    component: *mut snd_soc_component,
    endpoint: *mut device_node,
) -> c_int {
    unsafe {
        let hcp: *mut hdmi_codec_priv = snd_soc_component_get_drvdata(component);
        let mut ret: c_int = -ENOTSUPP; /* see snd_soc_get_dai_id() */

        if let Some(get_dai_id) = (*(*hcp).hcd.ops).get_dai_id {
            ret = get_dai_id(component, endpoint, (*hcp).hcd.data);
        }

        ret
    }
}

unsafe extern "C" fn hdmi_probe(component: *mut snd_soc_component) -> c_int {
    unsafe {
        let hcp: *mut hdmi_codec_priv = snd_soc_component_get_drvdata(component);
        let mut ret: c_int = 0;

        if let Some(hook_plugged_cb) = (*(*hcp).hcd.ops).hook_plugged_cb {
            ret = hook_plugged_cb(
                (*(*component).dev).parent,
                (*hcp).hcd.data,
                Some(plugged_cb),
                (*component).dev,
            );
        }

        ret
    }
}

unsafe extern "C" fn hdmi_remove(component: *mut snd_soc_component) {
    unsafe {
        let hcp: *mut hdmi_codec_priv = snd_soc_component_get_drvdata(component);

        if let Some(hook_plugged_cb) = (*(*hcp).hcd.ops).hook_plugged_cb {
            hook_plugged_cb(
                (*(*component).dev).parent,
                (*hcp).hcd.data,
                None,
                core::ptr::null_mut(),
            );
        }
    }
}

static hdmi_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(hdmi_probe),
    remove: Some(hdmi_remove),
    dapm_widgets: hdmi_widgets.as_ptr(),
    num_dapm_widgets: hdmi_widgets.len() as c_uint,
    of_xlate_dai_id: Some(hdmi_of_xlate_dai_id),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
    set_jack: Some(hdmi_codec_set_jack),
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn hdmi_codec_probe(pdev: *mut platform_device) -> c_int {
    unsafe {
        let hcd: *mut hdmi_codec_pdata = (*(*pdev).dev).platform_data as *mut hdmi_codec_pdata;
        let mut daidrv: *mut snd_soc_dai_driver;
        let dev: *mut device = &mut (*pdev).dev;
        let hcp: *mut hdmi_codec_priv;
        let mut dai_count: c_int;
        let mut i: c_int = 0;
        let mut ret: c_int;

        if hcd.is_null() {
            dev_err(dev, c"%s: No platform data\n".as_ptr(), c"hdmi_codec_probe".as_ptr());
            return -EINVAL;
        }

        dai_count = (*hcd).i2s as c_int + (*hcd).spdif as c_int;
        if dai_count < 1
            || (*hcd).ops.is_null()
            || ((*(*hcd).ops).hw_params.is_none() && (*(*hcd).ops).prepare.is_none())
            || (*(*hcd).ops).audio_shutdown.is_none()
        {
            dev_err(dev, c"%s: Invalid parameters\n".as_ptr(), c"hdmi_codec_probe".as_ptr());
            return -EINVAL;
        }

        hcp = devm_kzalloc(dev, core::mem::size_of::<hdmi_codec_priv>(), GFP_KERNEL)
            as *mut hdmi_codec_priv;
        if hcp.is_null() {
            return -ENOMEM;
        }

        (*hcp).hcd = *hcd;
        mutex_init(&mut (*hcp).lock);

        ret = snd_pcm_create_iec958_consumer_default(
            (*hcp).iec_status.as_mut_ptr(),
            core::mem::size_of_val(&(*hcp).iec_status),
        );
        if ret < 0 {
            return ret;
        }

        daidrv = devm_kcalloc(
            dev,
            dai_count as usize,
            core::mem::size_of::<snd_soc_dai_driver>(),
            GFP_KERNEL,
        ) as *mut snd_soc_dai_driver;
        if daidrv.is_null() {
            return -ENOMEM;
        }

        if (*hcd).i2s {
            *daidrv.add(i as usize) = hdmi_i2s_dai;
            (*daidrv.add(i as usize)).playback.channels_max = (*hcd).max_i2s_channels;
            if (*hcd).i2s_formats != 0 {
                (*daidrv.add(i as usize)).playback.formats = (*hcd).i2s_formats;
                (*daidrv.add(i as usize)).capture.formats = (*hcd).i2s_formats;
            }
            if (*hcd).no_i2s_playback {
                memset(
                    &mut (*daidrv.add(i as usize)).playback as *mut _ as *mut c_void,
                    0,
                    core::mem::size_of_val(&(*daidrv.add(i as usize)).playback),
                );
            }
            if (*hcd).no_i2s_capture {
                memset(
                    &mut (*daidrv.add(i as usize)).capture as *mut _ as *mut c_void,
                    0,
                    core::mem::size_of_val(&(*daidrv.add(i as usize)).capture),
                );
            }
            i += 1;
        }

        if (*hcd).spdif {
            *daidrv.add(i as usize) = hdmi_spdif_dai;
            if (*hcd).no_spdif_playback {
                memset(
                    &mut (*daidrv.add(i as usize)).playback as *mut _ as *mut c_void,
                    0,
                    core::mem::size_of_val(&(*daidrv.add(i as usize)).playback),
                );
            }
            if (*hcd).no_spdif_capture {
                memset(
                    &mut (*daidrv.add(i as usize)).capture as *mut _ as *mut c_void,
                    0,
                    core::mem::size_of_val(&(*daidrv.add(i as usize)).capture),
                );
            }
        }

        dev_set_drvdata(dev, hcp as *mut c_void);

        ret = devm_snd_soc_register_component(dev, &hdmi_driver, daidrv, dai_count);
        if ret != 0 {
            dev_err(
                dev,
                c"%s: snd_soc_register_component() failed (%d)\n".as_ptr(),
                c"hdmi_codec_probe".as_ptr(),
                ret,
            );
            return ret;
        }
        0
    }
}

static mut hdmi_codec_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: HDMI_CODEC_DRV_NAME,
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(hdmi_codec_probe),
    ..unsafe { core::mem::zeroed() }
};

module_platform_driver!(hdmi_codec_driver);

MODULE_AUTHOR!("Jyri Sarha <jsarha@ti.com>");
MODULE_DESCRIPTION!("HDMI Audio Codec Driver");
MODULE_LICENSE!("GPL");
MODULE_ALIAS!(concat!("platform:", HDMI_CODEC_DRV_NAME));

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
