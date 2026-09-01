/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Universal Interface for Intel High Definition Audio Codec
 *
 * Local helper functions
 *
 * Copyright (c) 2004 Takashi Iwai <tiwai@suse.de>
 */

/* C include dependency: <sound/pcm_drm_eld.h> */

use core::ffi::{c_char, c_int, c_uint, c_void};

pub type bool_ = bool;
pub type size_t = usize;

/* External C/kernel types supplied by other headers. */
pub enum hda_codec {}
pub enum hda_codec_driver {}
pub enum hda_verb {}
pub enum snd_kcontrol {}
pub enum snd_kcontrol_new {}
pub enum snd_ctl_elem_info {}
pub enum snd_ctl_elem_value {}
pub enum snd_pcm_substream {}
pub enum hda_pcm_stream {}
pub enum attribute_group {}
pub enum snd_info_buffer {}
pub enum snd_parsed_hdmi_eld {}
pub enum device {}
pub enum hda_codec_ops {}

pub type hda_nid_t = u16;
pub type u32 = u32;
pub type u64 = u64;

/* We abuse kcontrol_new.subdev field to pass the NID corresponding to
 * the given new control.  If id.subdev has a bit flag HDA_SUBDEV_NID_FLAG,
 * snd_hda_ctl_add() takes the lower-bit subdev value as a valid NID.
 *
 * Note that the subdevice field is cleared again before the real registration
 * in snd_hda_ctl_add(), so that this value won't appear in the outside.
 */
pub const HDA_SUBDEV_NID_FLAG: c_uint = 1u32 << 31;
pub const HDA_SUBDEV_AMP_FLAG: c_uint = 1u32 << 30;

/*
 * for mixer controls
 */
#[macro_export]
macro_rules! HDA_COMPOSE_AMP_VAL_OFS {
    ($nid:expr, $chs:expr, $idx:expr, $dir:expr, $ofs:expr) => {
        (($nid) | (($chs) << 16) | (($dir) << 18) | (($idx) << 19) | (($ofs) << 23))
    };
}

pub const HDA_AMP_VAL_MIN_MUTE: c_int = 1 << 29;

#[macro_export]
macro_rules! HDA_COMPOSE_AMP_VAL {
    ($nid:expr, $chs:expr, $idx:expr, $dir:expr) => {
        HDA_COMPOSE_AMP_VAL_OFS!($nid, $chs, $idx, $dir, 0)
    };
}

/* The HDA_CODEC_* control initializer macros translate to Rust macros that
 * expand to struct literals for the external snd_kcontrol_new-compatible type.
 */
#[macro_export]
macro_rules! HDA_CODEC_VOLUME_MONO_IDX {
    ($xname:expr, $xcidx:expr, $nid:expr, $channel:expr, $xindex:expr, $dir:expr, $flags:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: $xname,
            index: $xcidx,
            subdevice: HDA_SUBDEV_AMP_FLAG,
            access: SNDRV_CTL_ELEM_ACCESS_READWRITE
                | SNDRV_CTL_ELEM_ACCESS_TLV_READ
                | SNDRV_CTL_ELEM_ACCESS_TLV_CALLBACK,
            info: Some(snd_hda_mixer_amp_volume_info),
            get: Some(snd_hda_mixer_amp_volume_get),
            put: Some(snd_hda_mixer_amp_volume_put),
            tlv: snd_kcontrol_new_tlv {
                c: Some(snd_hda_mixer_amp_tlv),
            },
            private_value: HDA_COMPOSE_AMP_VAL!($nid, $channel, $xindex, $dir) | $flags,
            ..Default::default()
        }
    };
}

#[macro_export]
macro_rules! HDA_CODEC_VOLUME_IDX {
    ($xname:expr, $xcidx:expr, $nid:expr, $xindex:expr, $direction:expr) => {
        HDA_CODEC_VOLUME_MONO_IDX!($xname, $xcidx, $nid, 3, $xindex, $direction, 0)
    };
}

#[macro_export]
macro_rules! HDA_CODEC_VOLUME_MONO {
    ($xname:expr, $nid:expr, $channel:expr, $xindex:expr, $direction:expr) => {
        HDA_CODEC_VOLUME_MONO_IDX!($xname, 0, $nid, $channel, $xindex, $direction, 0)
    };
}

#[macro_export]
macro_rules! HDA_CODEC_VOLUME {
    ($xname:expr, $nid:expr, $xindex:expr, $direction:expr) => {
        HDA_CODEC_VOLUME_MONO!($xname, $nid, 3, $xindex, $direction)
    };
}

#[macro_export]
macro_rules! HDA_CODEC_VOLUME_MIN_MUTE {
    ($xname:expr, $nid:expr, $xindex:expr, $direction:expr) => {
        HDA_CODEC_VOLUME_MONO_IDX!(
            $xname,
            0,
            $nid,
            3,
            $xindex,
            $direction,
            HDA_AMP_VAL_MIN_MUTE
        )
    };
}

#[macro_export]
macro_rules! HDA_CODEC_MUTE_MONO_IDX {
    ($xname:expr, $xcidx:expr, $nid:expr, $channel:expr, $xindex:expr, $direction:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: $xname,
            index: $xcidx,
            subdevice: HDA_SUBDEV_AMP_FLAG,
            info: Some(snd_hda_mixer_amp_switch_info),
            get: Some(snd_hda_mixer_amp_switch_get),
            put: Some(snd_hda_mixer_amp_switch_put),
            private_value: HDA_COMPOSE_AMP_VAL!($nid, $channel, $xindex, $direction),
            ..Default::default()
        }
    };
}

#[macro_export]
macro_rules! HDA_CODEC_MUTE_IDX {
    ($xname:expr, $xcidx:expr, $nid:expr, $xindex:expr, $direction:expr) => {
        HDA_CODEC_MUTE_MONO_IDX!($xname, $xcidx, $nid, 3, $xindex, $direction)
    };
}

#[macro_export]
macro_rules! HDA_CODEC_MUTE_MONO {
    ($xname:expr, $nid:expr, $channel:expr, $xindex:expr, $direction:expr) => {
        HDA_CODEC_MUTE_MONO_IDX!($xname, 0, $nid, $channel, $xindex, $direction)
    };
}

#[macro_export]
macro_rules! HDA_CODEC_MUTE {
    ($xname:expr, $nid:expr, $xindex:expr, $direction:expr) => {
        HDA_CODEC_MUTE_MONO!($xname, $nid, 3, $xindex, $direction)
    };
}

/* CONFIG_SND_HDA_INPUT_BEEP: use beep callbacks when configured, otherwise the
 * macro aliases to HDA_CODEC_MUTE_MONO_IDX.
 */
#[macro_export]
macro_rules! HDA_CODEC_MUTE_BEEP_MONO_IDX {
    ($xname:expr, $xcidx:expr, $nid:expr, $channel:expr, $xindex:expr, $direction:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: $xname,
            index: $xcidx,
            subdevice: HDA_SUBDEV_AMP_FLAG,
            info: Some(snd_hda_mixer_amp_switch_info),
            get: Some(snd_hda_mixer_amp_switch_get_beep),
            put: Some(snd_hda_mixer_amp_switch_put_beep),
            private_value: HDA_COMPOSE_AMP_VAL!($nid, $channel, $xindex, $direction),
            ..Default::default()
        }
    };
}

#[macro_export]
macro_rules! HDA_CODEC_MUTE_BEEP_MONO {
    ($xname:expr, $nid:expr, $channel:expr, $xindex:expr, $direction:expr) => {
        HDA_CODEC_MUTE_BEEP_MONO_IDX!($xname, 0, $nid, $channel, $xindex, $direction)
    };
}

#[macro_export]
macro_rules! HDA_CODEC_MUTE_BEEP {
    ($xname:expr, $nid:expr, $xindex:expr, $direction:expr) => {
        HDA_CODEC_MUTE_BEEP_MONO!($xname, $nid, 3, $xindex, $direction)
    };
}

extern "C" {
    pub static snd_hda_pcm_type_name: [*const c_char; 0];

    pub fn snd_hda_mixer_amp_volume_info(
        kcontrol: *mut snd_kcontrol,
        uinfo: *mut snd_ctl_elem_info,
    ) -> c_int;
    pub fn snd_hda_mixer_amp_volume_get(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;
    pub fn snd_hda_mixer_amp_volume_put(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;
    pub fn snd_hda_mixer_amp_tlv(
        kcontrol: *mut snd_kcontrol,
        op_flag: c_int,
        size: c_uint,
        _tlv: *mut c_uint,
    ) -> c_int;
    pub fn snd_hda_mixer_amp_switch_info(
        kcontrol: *mut snd_kcontrol,
        uinfo: *mut snd_ctl_elem_info,
    ) -> c_int;
    pub fn snd_hda_mixer_amp_switch_get(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;
    pub fn snd_hda_mixer_amp_switch_put(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;
    /* CONFIG_SND_HDA_INPUT_BEEP */
    pub fn snd_hda_mixer_amp_switch_get_beep(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;
    pub fn snd_hda_mixer_amp_switch_put_beep(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;
    pub fn snd_hdac_regmap_get_amp(
        core: *mut c_void,
        nid: hda_nid_t,
        ch: c_int,
        dir: c_int,
        idx: c_int,
    ) -> c_int;
    pub fn snd_hda_codec_amp_update(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        ch: c_int,
        dir: c_int,
        idx: c_int,
        mask: c_int,
        val: c_int,
    ) -> c_int;
    pub fn snd_hda_codec_amp_stereo(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        direction: c_int,
        idx: c_int,
        mask: c_int,
        val: c_int,
    ) -> c_int;
    pub fn snd_hda_codec_amp_init(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        ch: c_int,
        direction: c_int,
        idx: c_int,
        mask: c_int,
        val: c_int,
    ) -> c_int;
    pub fn snd_hda_codec_amp_init_stereo(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        dir: c_int,
        idx: c_int,
        mask: c_int,
        val: c_int,
    ) -> c_int;
    pub fn snd_hda_set_vmaster_tlv(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        dir: c_int,
        tlv: *mut c_uint,
    );
    pub fn snd_hda_find_mixer_ctl(
        codec: *mut hda_codec,
        name: *const c_char,
    ) -> *mut snd_kcontrol;
    pub fn __snd_hda_add_vmaster(
        codec: *mut hda_codec,
        name: *mut c_char,
        tlv: *mut c_uint,
        followers: *const *const c_char,
        suffix: *const c_char,
        init_follower_vol: bool,
        access: c_uint,
        ctl_ret: *mut *mut snd_kcontrol,
    ) -> c_int;
    pub fn snd_hda_codec_reset(codec: *mut hda_codec) -> c_int;
    pub fn snd_hda_codec_disconnect_pcms(codec: *mut hda_codec);
    pub fn snd_hdac_regmap_sync(core: *mut c_void) -> c_int;
}

#[macro_export]
macro_rules! snd_hda_codec_amp_read {
    ($codec:expr, $nid:expr, $ch:expr, $dir:expr, $idx:expr) => {
        snd_hdac_regmap_get_amp(core::ptr::addr_of_mut!((*$codec).core), $nid, $ch, $dir, $idx)
    };
}

#[macro_export]
macro_rules! snd_hda_add_vmaster {
    ($codec:expr, $name:expr, $tlv:expr, $followers:expr, $suffix:expr, $access:expr) => {
        __snd_hda_add_vmaster(
            $codec,
            $name,
            $tlv,
            $followers,
            $suffix,
            true,
            $access,
            core::ptr::null_mut(),
        )
    };
}

#[macro_export]
macro_rules! snd_hda_regmap_sync {
    ($codec:expr) => {
        snd_hdac_regmap_sync(core::ptr::addr_of_mut!((*$codec).core))
    };
}

#[repr(C)]
pub struct hda_vmaster_mute_hook {
    /* below two fields must be filled by the caller of
     * snd_hda_add_vmaster_hook() beforehand
     */
    pub sw_kctl: *mut snd_kcontrol,
    pub hook: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    /* below are initialized automatically */
    pub codec: *mut hda_codec,
}

extern "C" {
    pub fn snd_hda_add_vmaster_hook(
        codec: *mut hda_codec,
        hook: *mut hda_vmaster_mute_hook,
    ) -> c_int;
    pub fn snd_hda_sync_vmaster_hook(hook: *mut hda_vmaster_mute_hook);
}

/* amp value bits */
pub const HDA_AMP_MUTE: c_int = 0x80;
pub const HDA_AMP_UNMUTE: c_int = 0x00;
pub const HDA_AMP_VOLMASK: c_int = 0x7f;

/*
 * SPDIF I/O
 */
extern "C" {
    pub fn snd_hda_create_dig_out_ctls(
        codec: *mut hda_codec,
        associated_nid: hda_nid_t,
        cvt_nid: hda_nid_t,
        type_: c_int,
    ) -> c_int;
    pub fn snd_hda_create_spdif_in_ctls(codec: *mut hda_codec, nid: hda_nid_t) -> c_int;
}

#[macro_export]
macro_rules! snd_hda_create_spdif_out_ctls {
    ($codec:expr, $anid:expr, $cnid:expr) => {
        snd_hda_create_dig_out_ctls($codec, $anid, $cnid, HDA_PCM_TYPE_SPDIF)
    };
}

/*
 * input MUX helper
 */
pub const HDA_MAX_NUM_INPUTS: usize = 36;

#[repr(C)]
pub struct hda_input_mux_item {
    pub label: [c_char; 32],
    pub index: c_uint,
}

#[repr(C)]
pub struct hda_input_mux {
    pub num_items: c_uint,
    pub items: [hda_input_mux_item; HDA_MAX_NUM_INPUTS],
}

extern "C" {
    pub fn snd_hda_input_mux_info(
        imux: *const hda_input_mux,
        uinfo: *mut snd_ctl_elem_info,
    ) -> c_int;
    pub fn snd_hda_input_mux_put(
        codec: *mut hda_codec,
        imux: *const hda_input_mux,
        ucontrol: *mut snd_ctl_elem_value,
        nid: hda_nid_t,
        cur_val: *mut c_uint,
    ) -> c_int;
    pub fn snd_hda_add_imux_item(
        codec: *mut hda_codec,
        imux: *mut hda_input_mux,
        label: *const c_char,
        index: c_int,
        type_idx: *mut c_int,
    ) -> c_int;
}

/*
 * Multi-channel / digital-out PCM helper
 */
pub const HDA_FRONT: c_int = 0;
pub const HDA_REAR: c_int = 1;
pub const HDA_CLFE: c_int = 2;
pub const HDA_SIDE: c_int = 3;

pub const HDA_DIG_NONE: c_int = 0;
pub const HDA_DIG_EXCLUSIVE: c_int = 1;
pub const HDA_DIG_ANALOG_DUP: c_int = 2;

pub const HDA_MAX_OUTS: usize = 5;

#[repr(C)]
pub struct hda_multi_out {
    pub num_dacs: c_int, /* # of DACs, must be more than 1 */
    pub dac_nids: *const hda_nid_t, /* DAC list */
    pub hp_nid: hda_nid_t, /* optional DAC for HP, 0 when not exists */
    pub hp_out_nid: [hda_nid_t; HDA_MAX_OUTS], /* DACs for multiple HPs */
    pub extra_out_nid: [hda_nid_t; HDA_MAX_OUTS], /* other (e.g. speaker) DACs */
    pub dig_out_nid: hda_nid_t, /* digital out audio widget */
    pub follower_dig_outs: *const hda_nid_t,
    pub max_channels: c_int, /* currently supported analog channels */
    pub dig_out_used: c_int, /* current usage of digital out (HDA_DIG_XXX) */
    pub no_share_stream: c_int, /* don't share a stream with multiple pins */
    pub share_spdif: c_int, /* share SPDIF pin */
    /* PCM information for both analog and SPDIF DACs */
    pub analog_rates: c_uint,
    pub analog_maxbps: c_uint,
    pub analog_formats: u64,
    pub spdif_rates: c_uint,
    pub spdif_maxbps: c_uint,
    pub spdif_formats: u64,
    pub share_spdif_kctl: *mut snd_kcontrol, /* cached shared SPDIF switch */
}

extern "C" {
    pub fn snd_hda_create_spdif_share_sw(
        codec: *mut hda_codec,
        mout: *mut hda_multi_out,
    ) -> c_int;
    pub fn snd_hda_multi_out_dig_open(
        codec: *mut hda_codec,
        mout: *mut hda_multi_out,
    ) -> c_int;
    pub fn snd_hda_multi_out_dig_close(
        codec: *mut hda_codec,
        mout: *mut hda_multi_out,
    ) -> c_int;
    pub fn snd_hda_multi_out_dig_prepare(
        codec: *mut hda_codec,
        mout: *mut hda_multi_out,
        stream_tag: c_uint,
        format: c_uint,
        substream: *mut snd_pcm_substream,
    ) -> c_int;
    pub fn snd_hda_multi_out_dig_cleanup(
        codec: *mut hda_codec,
        mout: *mut hda_multi_out,
    ) -> c_int;
    pub fn snd_hda_multi_out_analog_open(
        codec: *mut hda_codec,
        mout: *mut hda_multi_out,
        substream: *mut snd_pcm_substream,
        hinfo: *mut hda_pcm_stream,
    ) -> c_int;
    pub fn snd_hda_multi_out_analog_prepare(
        codec: *mut hda_codec,
        mout: *mut hda_multi_out,
        stream_tag: c_uint,
        format: c_uint,
        substream: *mut snd_pcm_substream,
    ) -> c_int;
    pub fn snd_hda_multi_out_analog_cleanup(
        codec: *mut hda_codec,
        mout: *mut hda_multi_out,
    ) -> c_int;
}

/*
 * generic proc interface
 */
/* CONFIG_SND_PROC_FS: external when configured; inline fallback returns 0. */
pub unsafe extern "C" fn snd_hda_codec_proc_new(_codec: *mut hda_codec) -> c_int {
    0
}

pub const SND_PRINT_BITS_ADVISED_BUFSIZE: c_int = 16;

extern "C" {
    pub fn snd_print_pcm_bits(pcm: c_int, buf: *mut c_char, buflen: c_int);
    pub fn snd_hda_add_new_ctls(codec: *mut hda_codec, knew: *const snd_kcontrol_new) -> c_int;
}

/*
 * Fix-up pin default configurations and add default verbs
 */
#[repr(C)]
pub struct hda_pintbl {
    pub nid: hda_nid_t,
    pub val: u32,
}

#[repr(C)]
pub struct hda_model_fixup {
    pub id: c_int,
    pub name: *const c_char,
}

#[repr(C)]
pub union hda_fixup_v {
    pub pins: *const hda_pintbl,
    pub verbs: *const hda_verb,
    pub func: Option<unsafe extern "C" fn(*mut hda_codec, *const hda_fixup, c_int)>,
}

#[repr(C)]
pub struct hda_fixup {
    pub type_: c_int,
    /* C bitfields: bool chained:1; bool chained_before:1; */
    pub chained: bool,
    pub chained_before: bool,
    pub chain_id: c_int,
    pub v: hda_fixup_v,
}

/*
 * extended form of snd_pci_quirk:
 * for PCI SSID matching, use SND_PCI_QUIRK() like before;
 * for codec SSID matching, use the new HDA_CODEC_QUIRK() instead
 */
#[repr(C)]
pub struct hda_quirk {
    pub subvendor: u16, /* PCI subvendor ID */
    pub subdevice: u16, /* PCI subdevice ID */
    pub subdevice_mask: u16, /* bitmask to match */
    pub match_codec_ssid: bool, /* match only with codec SSID */
    pub value: c_int, /* value */
    /* CONFIG_SND_DEBUG_VERBOSE */
    pub name: *const c_char, /* name of the device (optional) */
}

#[macro_export]
macro_rules! HDA_CODEC_QUIRK {
    ($vend:expr, $dev:expr, $xname:expr, $val:expr) => {
        hda_quirk {
            subvendor: ($vend) as u16,
            subdevice: ($dev) as u16,
            subdevice_mask: !0u16,
            match_codec_ssid: true,
            value: $val,
            name: $xname,
        }
    };
}

#[repr(C)]
pub struct snd_hda_pin_quirk {
    pub codec: c_uint, /* Codec vendor/device ID */
    pub subvendor: u16, /* PCI subvendor ID */
    pub pins: *const hda_pintbl, /* list of matching pins */
    /* CONFIG_SND_DEBUG_VERBOSE */
    pub name: *const c_char,
    pub value: c_int, /* quirk value */
}

#[macro_export]
macro_rules! SND_HDA_PIN_QUIRK {
    ($codec:expr, $subvendor:expr, $name:expr, $value:expr, $($pins:expr),* $(,)?) => {
        snd_hda_pin_quirk {
            codec: $codec,
            subvendor: $subvendor,
            name: $name,
            value: $value,
            pins: &[$($pins,)* hda_pintbl { nid: 0, val: 0 }] as *const hda_pintbl,
        }
    };
}

pub const HDA_FIXUP_ID_NOT_SET: c_int = -1;
pub const HDA_FIXUP_ID_NO_FIXUP: c_int = -2;

/* fixup types */
pub const HDA_FIXUP_INVALID: c_int = 0;
pub const HDA_FIXUP_PINS: c_int = 1;
pub const HDA_FIXUP_VERBS: c_int = 2;
pub const HDA_FIXUP_FUNC: c_int = 3;
pub const HDA_FIXUP_PINCTLS: c_int = 4;

/* fixup action definitions */
pub const HDA_FIXUP_ACT_PRE_PROBE: c_int = 0;
pub const HDA_FIXUP_ACT_PROBE: c_int = 1;
pub const HDA_FIXUP_ACT_INIT: c_int = 2;
pub const HDA_FIXUP_ACT_BUILD: c_int = 3;
pub const HDA_FIXUP_ACT_FREE: c_int = 4;

extern "C" {
    pub fn snd_hda_add_verbs(codec: *mut hda_codec, list: *const hda_verb) -> c_int;
    pub fn snd_hda_apply_verbs(codec: *mut hda_codec);
    pub fn snd_hda_apply_pincfgs(codec: *mut hda_codec, cfg: *const hda_pintbl);
    pub fn snd_hda_apply_fixup(codec: *mut hda_codec, action: c_int);
    pub fn __snd_hda_apply_fixup(
        codec: *mut hda_codec,
        id: c_int,
        action: c_int,
        depth: c_int,
    );
    pub fn snd_hda_pick_fixup(
        codec: *mut hda_codec,
        models: *const hda_model_fixup,
        quirk: *const hda_quirk,
        fixlist: *const hda_fixup,
    );
    pub fn snd_hda_pick_pin_fixup(
        codec: *mut hda_codec,
        pin_quirk: *const snd_hda_pin_quirk,
        fixlist: *const hda_fixup,
        match_all_pins: bool,
    );
}

/* helper macros to retrieve pin default-config values */
#[macro_export]
macro_rules! get_defcfg_connect {
    ($cfg:expr) => {
        (($cfg & AC_DEFCFG_PORT_CONN) >> AC_DEFCFG_PORT_CONN_SHIFT)
    };
}
#[macro_export]
macro_rules! get_defcfg_association {
    ($cfg:expr) => {
        (($cfg & AC_DEFCFG_DEF_ASSOC) >> AC_DEFCFG_ASSOC_SHIFT)
    };
}
#[macro_export]
macro_rules! get_defcfg_location {
    ($cfg:expr) => {
        (($cfg & AC_DEFCFG_LOCATION) >> AC_DEFCFG_LOCATION_SHIFT)
    };
}
#[macro_export]
macro_rules! get_defcfg_sequence {
    ($cfg:expr) => {
        ($cfg & AC_DEFCFG_SEQUENCE)
    };
}
#[macro_export]
macro_rules! get_defcfg_device {
    ($cfg:expr) => {
        (($cfg & AC_DEFCFG_DEVICE) >> AC_DEFCFG_DEVICE_SHIFT)
    };
}
#[macro_export]
macro_rules! get_defcfg_misc {
    ($cfg:expr) => {
        (($cfg & AC_DEFCFG_MISC) >> AC_DEFCFG_MISC_SHIFT)
    };
}

/* amp values */
#[macro_export]
macro_rules! AMP_IN_MUTE {
    ($idx:expr) => {
        (0x7080 | (($idx) << 8))
    };
}
#[macro_export]
macro_rules! AMP_IN_UNMUTE {
    ($idx:expr) => {
        (0x7000 | (($idx) << 8))
    };
}
pub const AMP_OUT_MUTE: c_int = 0xb080;
pub const AMP_OUT_UNMUTE: c_int = 0xb000;
pub const AMP_OUT_ZERO: c_int = 0xb000;

/* pinctl values */
pub const PIN_IN: c_uint = AC_PINCTL_IN_EN;
pub const PIN_VREFHIZ: c_uint = AC_PINCTL_IN_EN | AC_PINCTL_VREF_HIZ;
pub const PIN_VREF50: c_uint = AC_PINCTL_IN_EN | AC_PINCTL_VREF_50;
pub const PIN_VREFGRD: c_uint = AC_PINCTL_IN_EN | AC_PINCTL_VREF_GRD;
pub const PIN_VREF80: c_uint = AC_PINCTL_IN_EN | AC_PINCTL_VREF_80;
pub const PIN_VREF100: c_uint = AC_PINCTL_IN_EN | AC_PINCTL_VREF_100;
pub const PIN_OUT: c_uint = AC_PINCTL_OUT_EN;
pub const PIN_HP: c_uint = AC_PINCTL_OUT_EN | AC_PINCTL_HP_EN;
pub const PIN_HP_AMP: c_uint = AC_PINCTL_HP_EN;

extern "C" {
    pub static AC_PINCTL_IN_EN: c_uint;
    pub static AC_PINCTL_VREF_HIZ: c_uint;
    pub static AC_PINCTL_VREF_50: c_uint;
    pub static AC_PINCTL_VREF_GRD: c_uint;
    pub static AC_PINCTL_VREF_80: c_uint;
    pub static AC_PINCTL_VREF_100: c_uint;
    pub static AC_PINCTL_OUT_EN: c_uint;
    pub static AC_PINCTL_HP_EN: c_uint;

    pub fn snd_hda_get_default_vref(codec: *mut hda_codec, pin: hda_nid_t) -> c_uint;
    pub fn snd_hda_correct_pin_ctl(
        codec: *mut hda_codec,
        pin: hda_nid_t,
        val: c_uint,
    ) -> c_uint;
    pub fn _snd_hda_set_pin_ctl(
        codec: *mut hda_codec,
        pin: hda_nid_t,
        val: c_uint,
        cached: bool,
    ) -> c_int;
}

/**
 * snd_hda_set_pin_ctl - Set a pin-control value safely
 * @codec: the codec instance
 * @pin: the pin NID to set the control
 * @val: the pin-control value (AC_PINCTL_* bits)
 *
 * This function sets the pin-control value to the given pin, but
 * filters out the invalid pin-control bits when the pin has no such
 * capabilities.  For example, when PIN_HP is passed but the pin has no
 * HP-drive capability, the HP bit is omitted.
 *
 * The function doesn't check the input VREF capability bits, though.
 * Use snd_hda_get_default_vref() to guess the right value.
 * Also, this function is only for analog pins, not for HDMI pins.
 */
pub unsafe extern "C" fn snd_hda_set_pin_ctl(
    codec: *mut hda_codec,
    pin: hda_nid_t,
    val: c_uint,
) -> c_int {
    _snd_hda_set_pin_ctl(codec, pin, val, false)
}

/**
 * snd_hda_set_pin_ctl_cache - Set a pin-control value safely
 * @codec: the codec instance
 * @pin: the pin NID to set the control
 * @val: the pin-control value (AC_PINCTL_* bits)
 *
 * Just like snd_hda_set_pin_ctl() but write to cache as well.
 */
pub unsafe extern "C" fn snd_hda_set_pin_ctl_cache(
    codec: *mut hda_codec,
    pin: hda_nid_t,
    val: c_uint,
) -> c_int {
    _snd_hda_set_pin_ctl(codec, pin, val, true)
}

extern "C" {
    pub fn snd_hda_codec_get_pin_target(codec: *mut hda_codec, nid: hda_nid_t) -> c_int;
    pub fn snd_hda_codec_set_pin_target(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        val: c_uint,
    ) -> c_int;
}

#[macro_export]
macro_rules! for_each_hda_codec_node {
    ($nid:ident, $codec:expr, $body:block) => {{
        $nid = (*$codec).core.start_nid;
        while $nid < (*$codec).core.end_nid {
            $body
            $nid += 1;
        }
    }};
}

/* Set the codec power_state flag to indicate to allow unsol event handling;
 * see hda_codec_unsol_event() in hda_bind.c.  Calling this might confuse the
 * state tracking, so use with care.
 */
pub unsafe extern "C" fn snd_hda_codec_allow_unsol_events(codec: *mut hda_codec) {
    (*codec).core.dev.power.power_state = PMSG_ON;
}

/*
 * get widget capabilities
 */
pub unsafe extern "C" fn get_wcaps(codec: *mut hda_codec, nid: hda_nid_t) -> u32 {
    if nid < (*codec).core.start_nid || nid >= (*codec).core.start_nid + (*codec).core.num_nodes {
        return 0;
    }
    *(*codec).wcaps.offset((nid - (*codec).core.start_nid) as isize)
}

/* get the widget type from widget capability bits */
pub unsafe extern "C" fn get_wcaps_type(wcaps: c_uint) -> c_int {
    if wcaps == 0 {
        return -1; /* invalid type */
    }
    ((wcaps & AC_WCAP_TYPE) >> AC_WCAP_TYPE_SHIFT) as c_int
}

pub unsafe extern "C" fn get_wcaps_channels(wcaps: u32) -> c_uint {
    let mut chans: c_uint;

    chans = ((wcaps & AC_WCAP_CHAN_CNT_EXT) >> 13) as c_uint;
    chans = ((chans << 1) | 1) + 1;

    chans
}

pub unsafe extern "C" fn snd_hda_override_wcaps(
    codec: *mut hda_codec,
    nid: hda_nid_t,
    val: u32,
) {
    if nid >= (*codec).core.start_nid && nid < (*codec).core.start_nid + (*codec).core.num_nodes {
        *(*codec).wcaps.offset((nid - (*codec).core.start_nid) as isize) = val;
    }
}

extern "C" {
    pub static AC_WCAP_TYPE: c_uint;
    pub static AC_WCAP_TYPE_SHIFT: c_uint;
    pub static AC_WCAP_CHAN_CNT_EXT: u32;

    pub fn query_amp_caps(codec: *mut hda_codec, nid: hda_nid_t, direction: c_int) -> u32;
    pub fn snd_hda_override_amp_caps(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        dir: c_int,
        caps: c_uint,
    ) -> c_int;
    pub fn snd_hda_param_read(codec: *mut hda_codec, nid: hda_nid_t, param: c_uint) -> u32;
    pub fn snd_hdac_override_parm(
        core: *mut c_void,
        nid: hda_nid_t,
        param: c_uint,
        caps: c_uint,
    ) -> c_int;
    pub static AC_PAR_PIN_CAP: c_uint;
}

/**
 * snd_hda_query_pin_caps - Query PIN capabilities
 * @codec: the HD-auio codec
 * @nid: the NID to query
 *
 * Query PIN capabilities for the given widget.
 * Returns the obtained capability bits.
 *
 * When cap bits have been already read, this doesn't read again but
 * returns the cached value.
 */
pub unsafe extern "C" fn snd_hda_query_pin_caps(codec: *mut hda_codec, nid: hda_nid_t) -> u32 {
    snd_hda_param_read(codec, nid, AC_PAR_PIN_CAP)
}

/**
 * snd_hda_override_pin_caps - Override the pin capabilities
 * @codec: the CODEC
 * @nid: the NID to override
 * @caps: the capability bits to set
 *
 * Override the cached PIN capabilitiy bits value by the given one.
 *
 * Returns zero if successful or a negative error code.
 */
pub unsafe extern "C" fn snd_hda_override_pin_caps(
    codec: *mut hda_codec,
    nid: hda_nid_t,
    caps: c_uint,
) -> c_int {
    snd_hdac_override_parm(core::ptr::addr_of_mut!((*codec).core), nid, AC_PAR_PIN_CAP, caps)
}

extern "C" {
    pub fn snd_hda_check_amp_caps(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        dir: c_int,
        bits: c_uint,
    ) -> bool;
    pub static AC_AMPCAP_MUTE: c_uint;
    pub static AC_AMPCAP_MIN_MUTE: c_uint;
    pub static AC_AMPCAP_NUM_STEPS: c_uint;
}

#[macro_export]
macro_rules! nid_has_mute {
    ($codec:expr, $nid:expr, $dir:expr) => {
        snd_hda_check_amp_caps($codec, $nid, $dir, (AC_AMPCAP_MUTE | AC_AMPCAP_MIN_MUTE))
    };
}
#[macro_export]
macro_rules! nid_has_volume {
    ($codec:expr, $nid:expr, $dir:expr) => {
        snd_hda_check_amp_caps($codec, $nid, $dir, AC_AMPCAP_NUM_STEPS)
    };
}

/* flags for hda_nid_item */
pub const HDA_NID_ITEM_AMP: c_int = 1 << 0;

#[repr(C)]
pub struct hda_nid_item {
    pub kctl: *mut snd_kcontrol,
    pub index: c_uint,
    pub nid: hda_nid_t,
    pub flags: u16,
}

extern "C" {
    pub fn snd_hda_ctl_add(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        kctl: *mut snd_kcontrol,
    ) -> c_int;
    pub fn snd_hda_ctls_clear(codec: *mut hda_codec);
}

/*
 * hwdep interface
 */
/* CONFIG_SND_HDA_HWDEP: external when configured; inline fallback returns 0. */
pub unsafe extern "C" fn snd_hda_create_hwdep(_codec: *mut hda_codec) -> c_int {
    0
}

extern "C" {
    pub fn snd_hda_sysfs_init(codec: *mut hda_codec);
    pub fn snd_hda_sysfs_clear(codec: *mut hda_codec);
    pub static snd_hda_dev_attr_groups: [*const attribute_group; 0];
}

/* CONFIG_SND_HDA_RECONFIG: external when configured; inline fallbacks below. */
pub unsafe extern "C" fn snd_hda_get_hint(
    _codec: *mut hda_codec,
    _key: *const c_char,
) -> *const c_char {
    core::ptr::null()
}

pub unsafe extern "C" fn snd_hda_get_bool_hint(
    _codec: *mut hda_codec,
    _key: *const c_char,
) -> c_int {
    -ENOENT
}

pub unsafe extern "C" fn snd_hda_get_int_hint(
    _codec: *mut hda_codec,
    _key: *const c_char,
    _valp: *mut c_int,
) -> c_int {
    -ENOENT
}

/*
 * power-management
 */
extern "C" {
    pub static ENOENT: c_int;
    pub fn snd_hda_schedule_power_save(codec: *mut hda_codec);
}

#[repr(C)]
pub struct hda_amp_list {
    pub nid: hda_nid_t,
    pub dir: u8,
    pub idx: u8,
}

#[repr(C)]
pub struct hda_loopback_check {
    pub amplist: *const hda_amp_list,
    pub power_on: c_int,
}

extern "C" {
    pub fn snd_hda_check_amp_list_power(
        codec: *mut hda_codec,
        check: *mut hda_loopback_check,
        nid: hda_nid_t,
    ) -> c_int;
    pub fn snd_hdac_check_power_state(
        core: *mut c_void,
        nid: hda_nid_t,
        target_state: c_uint,
    ) -> bool;
    pub fn snd_hdac_sync_power_state(
        core: *mut c_void,
        nid: hda_nid_t,
        target_state: c_uint,
    ) -> c_uint;
}

/* check whether the actual power state matches with the target state */
pub unsafe extern "C" fn snd_hda_check_power_state(
    codec: *mut hda_codec,
    nid: hda_nid_t,
    target_state: c_uint,
) -> bool {
    snd_hdac_check_power_state(core::ptr::addr_of_mut!((*codec).core), nid, target_state)
}

pub unsafe extern "C" fn snd_hda_sync_power_state(
    codec: *mut hda_codec,
    nid: hda_nid_t,
    target_state: c_uint,
) -> c_uint {
    snd_hdac_sync_power_state(core::ptr::addr_of_mut!((*codec).core), nid, target_state)
}

extern "C" {
    pub fn snd_hda_codec_eapd_power_filter(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        power_state: c_uint,
    ) -> c_uint;
    pub fn snd_hda_codec_shutdown(codec: *mut hda_codec);
    pub fn hda_codec_to_driver(codec: *mut hda_codec) -> *mut hda_codec_driver;
}

pub unsafe extern "C" fn snd_hda_codec_init(codec: *mut hda_codec) -> c_int {
    let driver: *mut hda_codec_driver = hda_codec_to_driver(codec);

    if !(*(*driver).ops).init.is_none() {
        return ((*(*driver).ops).init.unwrap())(codec);
    }
    0
}

/*
 * AMP control callbacks
 */
/* retrieve parameters from private_value */
#[macro_export]
macro_rules! get_amp_nid_ {
    ($pv:expr) => {
        (($pv) & 0xffff)
    };
}
#[macro_export]
macro_rules! get_amp_nid {
    ($kc:expr) => {
        get_amp_nid_!((*$kc).private_value)
    };
}
#[macro_export]
macro_rules! get_amp_channels {
    ($kc:expr) => {
        (((*$kc).private_value >> 16) & 0x3)
    };
}
#[macro_export]
macro_rules! get_amp_direction_ {
    ($pv:expr) => {
        ((($pv) >> 18) & 0x1)
    };
}
#[macro_export]
macro_rules! get_amp_direction {
    ($kc:expr) => {
        get_amp_direction_!((*$kc).private_value)
    };
}
#[macro_export]
macro_rules! get_amp_index_ {
    ($pv:expr) => {
        ((($pv) >> 19) & 0xf)
    };
}
#[macro_export]
macro_rules! get_amp_index {
    ($kc:expr) => {
        get_amp_index_!((*$kc).private_value)
    };
}
#[macro_export]
macro_rules! get_amp_offset {
    ($kc:expr) => {
        (((*$kc).private_value >> 23) & 0x3f)
    };
}
#[macro_export]
macro_rules! get_amp_min_mute {
    ($kc:expr) => {
        (((*$kc).private_value >> 29) & 0x1)
    };
}

/*
 * enum control helper
 */
extern "C" {
    pub fn snd_hda_enum_helper_info(
        kcontrol: *mut snd_kcontrol,
        uinfo: *mut snd_ctl_elem_info,
        num_items: c_int,
        texts: *const *const c_char,
    ) -> c_int;
}

#[macro_export]
macro_rules! snd_hda_enum_bool_helper_info {
    ($kcontrol:expr, $uinfo:expr) => {
        snd_hda_enum_helper_info($kcontrol, $uinfo, 0, core::ptr::null())
    };
}

#[repr(C)]
pub struct hdmi_eld {
    pub monitor_present: bool,
    pub eld_valid: bool,
    pub eld_size: c_int,
    pub eld_buffer: [c_char; ELD_MAX_SIZE],
    pub info: snd_parsed_hdmi_eld,
}

extern "C" {
    pub static ELD_MAX_SIZE: usize;
    pub fn snd_hdmi_get_eld_size(codec: *mut hda_codec, nid: hda_nid_t) -> c_int;
    pub fn snd_hdmi_get_eld(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        buf: *mut u8,
        eld_size: *mut c_int,
    ) -> c_int;
    pub fn snd_hdmi_eld_update_pcm_info(e: *mut snd_parsed_hdmi_eld, hinfo: *mut hda_pcm_stream);

    /* CONFIG_SND_PROC_FS */
    pub fn snd_hdmi_print_eld_info(
        eld: *mut hdmi_eld,
        buffer: *mut snd_info_buffer,
        pin_nid: hda_nid_t,
        dev_id: c_int,
        cvt_nid: hda_nid_t,
    );
    pub fn snd_hdmi_write_eld_info(eld: *mut hdmi_eld, buffer: *mut snd_info_buffer);
}

pub const SND_PRINT_CHANNEL_ALLOCATION_ADVISED_BUFSIZE: c_int = 80;

extern "C" {
    pub fn snd_print_channel_allocation(spk_alloc: c_int, buf: *mut c_char, buflen: c_int);
    pub fn snd_hda_codec_display_power(codec: *mut hda_codec, enable: bool);

    pub fn hda_codec_dev(codec: *mut hda_codec) -> *mut device;
    pub fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    pub fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    pub fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    pub fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

#[macro_export]
macro_rules! codec_err {
    ($codec:expr, $fmt:expr $(, $args:expr)* $(,)?) => {
        dev_err(hda_codec_dev($codec), $fmt $(, $args)*)
    };
}
#[macro_export]
macro_rules! codec_warn {
    ($codec:expr, $fmt:expr $(, $args:expr)* $(,)?) => {
        dev_warn(hda_codec_dev($codec), $fmt $(, $args)*)
    };
}
#[macro_export]
macro_rules! codec_info {
    ($codec:expr, $fmt:expr $(, $args:expr)* $(,)?) => {
        dev_info(hda_codec_dev($codec), $fmt $(, $args)*)
    };
}
#[macro_export]
macro_rules! codec_dbg {
    ($codec:expr, $fmt:expr $(, $args:expr)* $(,)?) => {
        dev_dbg(hda_codec_dev($codec), $fmt $(, $args)*)
    };
}

extern "C" {
    pub fn strnlen(s: *const c_char, maxlen: size_t) -> size_t;
    pub fn strscpy(dest: *mut c_char, src: *const c_char, count: size_t) -> isize;
}

/* append a suffix string safely; equivalent with strlcat() */
pub unsafe extern "C" fn hda_append_suffix(str_: *mut c_char, suffix: *const c_char, size: size_t) {
    let len: size_t = strnlen(str_, size);
    strscpy(str_.add(len), suffix, size - len);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
