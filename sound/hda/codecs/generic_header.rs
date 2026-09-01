/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Generic BIOS auto-parser helper functions for HD-audio
 *
 * Copyright (c) 2012 Takashi Iwai <tiwai@suse.de>
 */

use core::ffi::{c_char, c_int, c_uint};

/* Dependencies from <linux/leds.h> and "hda_auto_parser.h" are expected to
 * provide the referenced C-compatible types and constants.
 */

pub enum hda_jack_callback {}

/* table entry for multi-io paths */
#[repr(C)]
pub struct hda_multi_io {
    pub pin: hda_nid_t,      /* multi-io widget pin NID */
    pub dac: hda_nid_t,      /* DAC to be connected */
    pub ctl_in: c_uint,      /* cached input-pin control value */
}

/* Widget connection path
 *
 * For output, stored in the order of DAC -> ... -> pin,
 * for input, pin -> ... -> ADC.
 *
 * idx[i] contains the source index number to select on of the widget path[i];
 * e.g. idx[1] is the index of the DAC (path[0]) selected by path[1] widget
 * multi[] indicates whether it's a selector widget with multi-connectors
 * (i.e. the connection selection is mandatory)
 * vol_ctl and mute_ctl contains the NIDs for the assigned mixers
 */

pub const MAX_NID_PATH_DEPTH: usize = 10;

pub const NID_PATH_VOL_CTL: c_int = 0;
pub const NID_PATH_MUTE_CTL: c_int = 1;
pub const NID_PATH_BOOST_CTL: c_int = 2;
pub const NID_PATH_NUM_CTLS: usize = 3;

#[repr(C)]
pub struct nid_path {
    pub depth: c_int,
    pub path: [hda_nid_t; MAX_NID_PATH_DEPTH],
    pub idx: [u8; MAX_NID_PATH_DEPTH],
    pub multi: [u8; MAX_NID_PATH_DEPTH],
    pub ctls: [c_uint; NID_PATH_NUM_CTLS], /* NID_PATH_XXX_CTL */
    pub active: bool,                      /* C bitfield: bool active:1; activated by driver */
    pub pin_enabled: bool,                 /* C bitfield: bool pin_enabled:1; pins are enabled */
    pub pin_fixed: bool,                   /* C bitfield: bool pin_fixed:1; path with fixed pin */
    pub stream_enabled: bool,              /* C bitfield: bool stream_enabled:1; stream is active */
}

/* mic/line-in auto switching entry */

pub const MAX_AUTO_MIC_PINS: usize = 3;

#[repr(C)]
pub struct automic_entry {
    pub pin: hda_nid_t, /* pin */
    pub idx: c_int,    /* imux index, -1 = invalid */
    pub attr: c_uint,  /* pin attribute (INPUT_PIN_ATTR_*) */
}

/* active stream id */
pub const STREAM_MULTI_OUT: c_int = 0;
pub const STREAM_INDEP_HP: c_int = 1;

/* PCM hook action */
pub const HDA_GEN_PCM_ACT_OPEN: c_int = 0;
pub const HDA_GEN_PCM_ACT_PREPARE: c_int = 1;
pub const HDA_GEN_PCM_ACT_CLEANUP: c_int = 2;
pub const HDA_GEN_PCM_ACT_CLOSE: c_int = 3;

/* DAC assignment badness table */
#[repr(C)]
pub struct badness_table {
    pub no_primary_dac: c_int,    /* no primary DAC */
    pub no_dac: c_int,            /* no secondary DACs */
    pub shared_primary: c_int,    /* primary DAC is shared with main output */
    pub shared_surr: c_int,       /* secondary DAC shared with main or primary */
    pub shared_clfe: c_int,       /* third DAC shared with main or primary */
    pub shared_surr_main: c_int,  /* secondary DAC sahred with main/DAC0 */
}

unsafe extern "C" {
    pub static hda_main_out_badness: badness_table;
    pub static hda_extra_out_badness: badness_table;
}

#[repr(C)]
pub struct hda_gen_spec {
    pub stream_name_analog: [c_char; 32], /* analog PCM stream */
    pub stream_analog_playback: *const hda_pcm_stream,
    pub stream_analog_capture: *const hda_pcm_stream,

    pub stream_name_alt_analog: [c_char; 32], /* alternative analog PCM stream */
    pub stream_analog_alt_playback: *const hda_pcm_stream,
    pub stream_analog_alt_capture: *const hda_pcm_stream,

    pub stream_name_digital: [c_char; 32], /* digital PCM stream */
    pub stream_digital_playback: *const hda_pcm_stream,
    pub stream_digital_capture: *const hda_pcm_stream,

    /* PCM */
    pub active_streams: c_uint,
    pub pcm_mutex: mutex,

    /* playback */
    pub multiout: hda_multi_out, /* playback set-up
                                  * max_channels, dacs must be set
                                  * dig_out_nid and hp_nid are optional
                                  */
    pub alt_dac_nid: hda_nid_t,
    pub follower_dig_outs: [hda_nid_t; 3], /* optional - for auto-parsing */
    pub dig_out_type: c_int,

    /* capture */
    pub num_adc_nids: c_uint,
    pub adc_nids: [hda_nid_t; AUTO_CFG_MAX_INS],
    pub dig_in_nid: hda_nid_t,      /* digital-in NID; optional */
    pub mixer_nid: hda_nid_t,       /* analog-mixer NID */
    pub mixer_merge_nid: hda_nid_t, /* aamix merge-point NID (optional) */
    pub input_labels: [*const c_char; HDA_MAX_NUM_INPUTS],
    pub input_label_idxs: [c_int; HDA_MAX_NUM_INPUTS],

    /* capture setup for dynamic dual-adc switch */
    pub cur_adc: hda_nid_t,
    pub cur_adc_stream_tag: c_uint,
    pub cur_adc_format: c_uint,

    /* capture source */
    pub input_mux: hda_input_mux,
    pub cur_mux: [c_uint; 3],

    /* channel model */
    /* min_channel_count contains the minimum channel count for primary
     * outputs.  When multi_ios is set, the channels can be configured
     * between min_channel_count and (min_channel_count + multi_ios * 2).
     *
     * ext_channel_count contains the current channel count of the primary
     * out.  This varies in the range above.
     *
     * Meanwhile, const_channel_count is the channel count for all outputs
     * including headphone and speakers.  It's a constant value, and the
     * PCM is set up as max(ext_channel_count, const_channel_count).
     */
    pub min_channel_count: c_int,   /* min. channel count for primary out */
    pub ext_channel_count: c_int,   /* current channel count for primary */
    pub const_channel_count: c_int, /* channel count for all */

    /* PCM information */
    pub pcm_rec: [*mut hda_pcm; 3], /* used in build_pcms() */

    /* dynamic controls, init_verbs and input_mux */
    pub autocfg: auto_pin_cfg,
    pub kctls: snd_array,
    pub private_dac_nids: [hda_nid_t; AUTO_CFG_MAX_OUTS],
    pub imux_pins: [hda_nid_t; HDA_MAX_NUM_INPUTS],
    pub dyn_adc_idx: [c_uint; HDA_MAX_NUM_INPUTS],
    /* shared hp/mic */
    pub shared_mic_vref_pin: hda_nid_t,
    pub hp_mic_pin: hda_nid_t,
    pub hp_mic_mux_idx: c_int,

    /* DAC/ADC lists */
    pub num_all_dacs: c_int,
    pub all_dacs: [hda_nid_t; 16],
    pub num_all_adcs: c_int,
    pub all_adcs: [hda_nid_t; AUTO_CFG_MAX_INS],

    /* path list */
    pub paths: snd_array,

    /* path indices */
    pub out_paths: [c_int; AUTO_CFG_MAX_OUTS],
    pub hp_paths: [c_int; AUTO_CFG_MAX_OUTS],
    pub speaker_paths: [c_int; AUTO_CFG_MAX_OUTS],
    pub aamix_out_paths: [c_int; 3],
    pub digout_paths: [c_int; AUTO_CFG_MAX_OUTS],
    pub input_paths: [[c_int; AUTO_CFG_MAX_INS]; HDA_MAX_NUM_INPUTS],
    pub loopback_paths: [c_int; HDA_MAX_NUM_INPUTS],
    pub loopback_merge_path: c_int,
    pub digin_path: c_int,

    /* auto-mic stuff */
    pub am_num_entries: c_int,
    pub am_entry: [automic_entry; MAX_AUTO_MIC_PINS],

    /* for pin sensing */
    /* current status; set in hda_generic.c */
    pub hp_jack_present: c_uint,   /* C bitfield: unsigned int hp_jack_present:1; */
    pub line_jack_present: c_uint, /* C bitfield: unsigned int line_jack_present:1; */
    pub speaker_muted: c_uint,     /* C bitfield: unsigned int speaker_muted:1; current status of speaker mute */
    pub line_out_muted: c_uint,    /* C bitfield: unsigned int line_out_muted:1; current status of LO mute */

    /* internal states of automute / autoswitch behavior */
    pub auto_mic: c_uint,          /* C bitfield: unsigned int auto_mic:1; */
    pub automute_speaker: c_uint,  /* C bitfield: unsigned int automute_speaker:1; automute speaker outputs */
    pub automute_lo: c_uint,       /* C bitfield: unsigned int automute_lo:1; automute LO outputs */

    /* capabilities detected by parser */
    pub detect_hp: c_uint,                  /* C bitfield: unsigned int detect_hp:1; Headphone detection enabled */
    pub detect_lo: c_uint,                  /* C bitfield: unsigned int detect_lo:1; Line-out detection enabled */
    pub automute_speaker_possible: c_uint,  /* C bitfield: unsigned int automute_speaker_possible:1; there are speakers and either LO or HP */
    pub automute_lo_possible: c_uint,       /* C bitfield: unsigned int automute_lo_possible:1; there are line outs and HP */

    /* additional parameters set by codec drivers */
    pub master_mute: c_uint,           /* C bitfield: unsigned int master_mute:1; master mute over all */
    pub keep_vref_in_automute: c_uint, /* C bitfield: unsigned int keep_vref_in_automute:1; Don't clear VREF in automute */
    pub line_in_auto_switch: c_uint,   /* C bitfield: unsigned int line_in_auto_switch:1; allow line-in auto switch */
    pub auto_mute_via_amp: c_uint,     /* C bitfield: unsigned int auto_mute_via_amp:1; auto-mute via amp instead of pinctl */

    /* parser behavior flags; set before snd_hda_gen_parse_auto_config() */
    pub suppress_auto_mute: c_uint, /* C bitfield: unsigned int suppress_auto_mute:1; suppress input jack auto mute */
    pub suppress_auto_mic: c_uint,  /* C bitfield: unsigned int suppress_auto_mic:1; suppress input jack auto switch */

    /* other parse behavior flags */
    pub need_dac_fix: c_uint,         /* C bitfield: unsigned int need_dac_fix:1; need to limit DACs for multi channels */
    pub hp_mic: c_uint,               /* C bitfield: unsigned int hp_mic:1; Allow HP as a mic-in */
    pub suppress_hp_mic_detect: c_uint, /* C bitfield: unsigned int suppress_hp_mic_detect:1; Don't detect HP/mic */
    pub no_primary_hp: c_uint,        /* C bitfield: unsigned int no_primary_hp:1; Don't prefer HP pins to speaker pins */
    pub no_multi_io: c_uint,          /* C bitfield: unsigned int no_multi_io:1; Don't try multi I/O config */
    pub multi_cap_vol: c_uint,        /* C bitfield: unsigned int multi_cap_vol:1; allow multiple capture xxx volumes */
    pub inv_dmic_split: c_uint,       /* C bitfield: unsigned int inv_dmic_split:1; inverted dmic w/a for conexant */
    pub own_eapd_ctl: c_uint,         /* C bitfield: unsigned int own_eapd_ctl:1; set EAPD by own function */
    pub keep_eapd_on: c_uint,         /* C bitfield: unsigned int keep_eapd_on:1; don't turn off EAPD automatically */
    pub vmaster_mute_led: c_uint,     /* C bitfield: unsigned int vmaster_mute_led:1; add SPK-LED flag to vmaster mute switch */
    pub mic_mute_led: c_uint,         /* C bitfield: unsigned int mic_mute_led:1; add MIC-LED flag to capture mute switch */
    pub indep_hp: c_uint,             /* C bitfield: unsigned int indep_hp:1; independent HP supported */
    pub prefer_hp_amp: c_uint,        /* C bitfield: unsigned int prefer_hp_amp:1; enable HP amp for speaker if any */
    pub add_stereo_mix_input: c_uint, /* C bitfield: unsigned int add_stereo_mix_input:2; add aamix as a capture src */
    pub add_jack_modes: c_uint,       /* C bitfield: unsigned int add_jack_modes:1; add i/o jack mode enum ctls */
    pub power_down_unused: c_uint,    /* C bitfield: unsigned int power_down_unused:1; power down unused widgets */
    pub dac_min_mute: c_uint,         /* C bitfield: unsigned int dac_min_mute:1; minimal = mute for DACs */
    pub suppress_vmaster: c_uint,     /* C bitfield: unsigned int suppress_vmaster:1; don't create vmaster kctls */

    /* other internal flags */
    pub no_analog: c_uint,       /* C bitfield: unsigned int no_analog:1; digital I/O only */
    pub dyn_adc_switch: c_uint,  /* C bitfield: unsigned int dyn_adc_switch:1; switch ADCs (for ALC275) */
    pub indep_hp_enabled: c_uint, /* C bitfield: unsigned int indep_hp_enabled:1; independent HP enabled */
    pub have_aamix_ctl: c_uint,  /* C bitfield: unsigned int have_aamix_ctl:1; */
    pub hp_mic_jack_modes: c_uint, /* C bitfield: unsigned int hp_mic_jack_modes:1; */
    pub skip_verbs: c_uint,      /* C bitfield: unsigned int skip_verbs:1; don't apply verbs at snd_hda_gen_init() */

    /* additional mute flags (only effective with auto_mute_via_amp=1) */
    pub mute_bits: u64,

    /* bitmask for skipping volume controls */
    pub out_vol_mask: u64,

    /* badness tables for output path evaluations */
    pub main_out_badness: *const badness_table,
    pub extra_out_badness: *const badness_table,

    /* preferred pin/DAC pairs; an array of paired NIDs */
    pub preferred_dacs: *const hda_nid_t,

    /* loopback mixing mode */
    pub aamix_mode: bool,

    /* digital beep */
    pub beep_nid: hda_nid_t,

    /* for virtual master */
    pub vmaster_nid: hda_nid_t,
    pub vmaster_tlv: [c_uint; 4],
    pub vmaster_mute: hda_vmaster_mute_hook,

    pub loopback: hda_loopback_check,
    pub loopback_list: snd_array,

    /* multi-io */
    pub multi_ios: c_int,
    pub multi_io: [hda_multi_io; 4],

    /* hooks */
    pub init_hook: Option<unsafe extern "C" fn(codec: *mut hda_codec)>,
    pub automute_hook: Option<unsafe extern "C" fn(codec: *mut hda_codec)>,
    pub cap_sync_hook: Option<
        unsafe extern "C" fn(
            codec: *mut hda_codec,
            kcontrol: *mut snd_kcontrol,
            ucontrol: *mut snd_ctl_elem_value,
        ),
    >,

    /* PCM hooks */
    pub pcm_playback_hook: Option<
        unsafe extern "C" fn(
            hinfo: *mut hda_pcm_stream,
            codec: *mut hda_codec,
            substream: *mut snd_pcm_substream,
            action: c_int,
        ),
    >,
    pub pcm_capture_hook: Option<
        unsafe extern "C" fn(
            hinfo: *mut hda_pcm_stream,
            codec: *mut hda_codec,
            substream: *mut snd_pcm_substream,
            action: c_int,
        ),
    >,

    /* automute / autoswitch hooks */
    pub hp_automute_hook: Option<
        unsafe extern "C" fn(codec: *mut hda_codec, cb: *mut hda_jack_callback),
    >,
    pub line_automute_hook: Option<
        unsafe extern "C" fn(codec: *mut hda_codec, cb: *mut hda_jack_callback),
    >,
    pub mic_autoswitch_hook: Option<
        unsafe extern "C" fn(codec: *mut hda_codec, cb: *mut hda_jack_callback),
    >,

    /* leds */
    pub led_cdevs: [*mut led_classdev; NUM_AUDIO_LEDS],
}

/* values for add_stereo_mix_input flag */
pub const HDA_HINT_STEREO_MIX_DISABLE: c_int = 0; /* No stereo mix input */
pub const HDA_HINT_STEREO_MIX_ENABLE: c_int = 1;  /* Add stereo mix input */
pub const HDA_HINT_STEREO_MIX_AUTO: c_int = 2;    /* Add only if auto-mic is disabled */

unsafe extern "C" {
    pub fn snd_hda_gen_spec_init(spec: *mut hda_gen_spec) -> c_int;

    pub fn snd_hda_gen_init(codec: *mut hda_codec) -> c_int;
    pub fn snd_hda_gen_remove(codec: *mut hda_codec);

    pub fn snd_hda_get_path_idx(codec: *mut hda_codec, path: *mut nid_path) -> c_int;
    pub fn snd_hda_get_path_from_idx(codec: *mut hda_codec, idx: c_int) -> *mut nid_path;
    pub fn snd_hda_add_new_path(
        codec: *mut hda_codec,
        from_nid: hda_nid_t,
        to_nid: hda_nid_t,
        anchor_nid: c_int,
    ) -> *mut nid_path;
    pub fn snd_hda_activate_path(
        codec: *mut hda_codec,
        path: *mut nid_path,
        enable: bool,
        add_aamix: bool,
    );

    pub fn snd_hda_gen_add_kctl(
        spec: *mut hda_gen_spec,
        name: *const c_char,
        temp: *const snd_kcontrol_new,
    ) -> *mut snd_kcontrol_new;

    pub fn snd_hda_gen_parse_auto_config(
        codec: *mut hda_codec,
        cfg: *mut auto_pin_cfg,
    ) -> c_int;
    pub fn snd_hda_gen_build_controls(codec: *mut hda_codec) -> c_int;
    pub fn snd_hda_gen_build_pcms(codec: *mut hda_codec) -> c_int;

    /* standard jack event callbacks */
    pub fn snd_hda_gen_hp_automute(codec: *mut hda_codec, jack: *mut hda_jack_callback);
    pub fn snd_hda_gen_line_automute(codec: *mut hda_codec, jack: *mut hda_jack_callback);
    pub fn snd_hda_gen_mic_autoswitch(codec: *mut hda_codec, jack: *mut hda_jack_callback);
    pub fn snd_hda_gen_update_outputs(codec: *mut hda_codec);

    pub fn snd_hda_gen_check_power_status(codec: *mut hda_codec, nid: hda_nid_t) -> c_int;
    pub fn snd_hda_gen_path_power_filter(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        power_state: c_uint,
    ) -> c_uint;
    pub fn snd_hda_gen_stream_pm(codec: *mut hda_codec, nid: hda_nid_t, on: bool);
    pub fn snd_hda_gen_fix_pin_power(codec: *mut hda_codec, pin: hda_nid_t) -> c_int;

    pub fn snd_hda_gen_add_mute_led_cdev(
        codec: *mut hda_codec,
        callback: Option<unsafe extern "C" fn(*mut led_classdev, led_brightness) -> c_int>,
    ) -> c_int;
    pub fn snd_hda_gen_add_micmute_led_cdev(
        codec: *mut hda_codec,
        callback: Option<unsafe extern "C" fn(*mut led_classdev, led_brightness) -> c_int>,
    ) -> c_int;
    pub fn snd_hda_gen_shutup_speakers(codec: *mut hda_codec) -> bool;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
