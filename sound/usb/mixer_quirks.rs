// SPDX-License-Identifier: GPL-2.0-or-later
// USB Audio Driver for ALSA
// Quirks and vendor-specific extensions for mixer interfaces
//
// Copyright (c) 2002 by Takashi Iwai <tiwai@suse.de>
//
// Many codes borrowed from audio.c by
//    Alan Cox (alan@lxorguk.ukuu.org.uk)
//    Thomas Sailer (sailer@ife.ee.ethz.ch)
//
// Audio Advantage Micro II support added by:
//    Przemek Rudy (prudy1@o2.pl)

// This file translates usb/mixer_quirks.c from the Linux kernel.
// External dependencies on other kernel modules and structures are referenced
// but not implemented here.

use core::mem;

#[repr(C)]
pub struct StdMonoTable {
    pub unitid: u32,
    pub control: u32,
    pub cmask: u32,
    pub val_type: i32,
    pub name: *const i8,
    pub tlv_callback: *const (),
}

// This function allows for the creation of standard UAC controls.
// See the quirks for M-Audio FTUs or Ebox-44.
// If you don't want to set a TLV callback pass NULL.
//
// Since there doesn't seem to be a devices that needs a multichannel
// version, we keep it mono for simplicity.
pub unsafe extern "C" fn snd_create_std_mono_ctl_offset(
    mixer: *mut (),
    unitid: u32,
    control: u32,
    cmask: u32,
    val_type: i32,
    idx_off: u32,
    name: *const i8,
    tlv_callback: *const (),
) -> i32 {
    // TODO: External dependencies - kzalloc_obj, snd_usb_mixer_elem_init_std,
    // snd_ctl_new1, snprintf, snd_usb_mixer_add_control, kfree
    0
}

pub unsafe extern "C" fn snd_create_std_mono_ctl(
    mixer: *mut (),
    unitid: u32,
    control: u32,
    cmask: u32,
    val_type: i32,
    name: *const i8,
    tlv_callback: *const (),
) -> i32 {
    snd_create_std_mono_ctl_offset(mixer, unitid, control, cmask, val_type, 0, name, tlv_callback)
}

// Create a set of standard UAC controls from a table
pub unsafe extern "C" fn snd_create_std_mono_table(
    mixer: *mut (),
    t: *const StdMonoTable,
) -> i32 {
    let mut t = t;
    while !(*t).name.is_null() {
        let err = snd_create_std_mono_ctl(
            mixer,
            (*t).unitid,
            (*t).control,
            (*t).cmask,
            (*t).val_type,
            (*t).name,
            (*t).tlv_callback,
        );
        if err < 0 {
            return err;
        }
        t = t.offset(1);
    }
    0
}

pub unsafe extern "C" fn add_single_ctl_with_resume(
    mixer: *mut (),
    id: i32,
    resume: *const (),
    knew: *const (),
    listp: *mut *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

// Sound Blaster remote control configuration
//
// format of remote control data:
// Extigy:       xx 00
// Audigy 2 NX:  06 80 xx 00 00 00
// Live! 24-bit: 06 80 xx yy 22 83

#[repr(C)]
pub struct RcConfig {
    pub usb_id: u32,
    pub offset: u8,
    pub length: u8,
    pub packet_length: u8,
    pub min_packet_length: u8,
    pub mute_mixer_id: u8,
    pub mute_code: u32,
}

pub static RC_CONFIGS: &[RcConfig] = &[
    RcConfig {
        usb_id: 0x041e3000,
        offset: 0,
        length: 1,
        packet_length: 2,
        min_packet_length: 1,
        mute_mixer_id: 18,
        mute_code: 0x0013,
    },
    RcConfig {
        usb_id: 0x041e3020,
        offset: 2,
        length: 1,
        packet_length: 6,
        min_packet_length: 6,
        mute_mixer_id: 18,
        mute_code: 0x0013,
    },
    RcConfig {
        usb_id: 0x041e3040,
        offset: 2,
        length: 2,
        packet_length: 6,
        min_packet_length: 6,
        mute_mixer_id: 2,
        mute_code: 0x6e91,
    },
    RcConfig {
        usb_id: 0x041e3042,
        offset: 0,
        length: 1,
        packet_length: 1,
        min_packet_length: 1,
        mute_mixer_id: 1,
        mute_code: 0x000d,
    },
    RcConfig {
        usb_id: 0x041e30df,
        offset: 0,
        length: 1,
        packet_length: 1,
        min_packet_length: 1,
        mute_mixer_id: 1,
        mute_code: 0x000d,
    },
    RcConfig {
        usb_id: 0x041e3237,
        offset: 0,
        length: 1,
        packet_length: 1,
        min_packet_length: 1,
        mute_mixer_id: 1,
        mute_code: 0x000d,
    },
    RcConfig {
        usb_id: 0x041e3263,
        offset: 0,
        length: 1,
        packet_length: 1,
        min_packet_length: 1,
        mute_mixer_id: 1,
        mute_code: 0x000d,
    },
    RcConfig {
        usb_id: 0x041e3048,
        offset: 2,
        length: 2,
        packet_length: 6,
        min_packet_length: 6,
        mute_mixer_id: 2,
        mute_code: 0x6e91,
    },
];

pub unsafe extern "C" fn snd_usb_soundblaster_remote_complete(urb: *mut ()) {
    // TODO: External dependencies
}

pub unsafe extern "C" fn snd_usb_sbrc_hwdep_read(
    hw: *mut (),
    buf: *mut i8,
    count: i64,
    offset: *mut i64,
) -> i64 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_usb_sbrc_hwdep_poll(
    hw: *mut (),
    file: *mut (),
    wait: *mut (),
) -> u32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_usb_soundblaster_remote_init(mixer: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn snd_audigy2nx_led_info(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // Alias to snd_ctl_boolean_mono_info
    0
}

pub unsafe extern "C" fn snd_audigy2nx_led_get(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_audigy2nx_led_update(
    mixer: *mut (),
    value: i32,
    index: i32,
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_audigy2nx_led_put(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_audigy2nx_led_resume(list: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

#[repr(C)]
pub struct SndKcontrolNew {
    pub iface: u32,
    pub info: *const (),
    pub get: *const (),
    pub put: *const (),
}

pub static SND_AUDIGY2NX_CONTROL: SndKcontrolNew = SndKcontrolNew {
    iface: 0,
    info: 0 as *const (),
    get: 0 as *const (),
    put: 0 as *const (),
};

pub static SND_AUDIGY2NX_LED_NAMES: &[&str] = &[
    "CMSS LED Switch",
    "Power LED Switch",
    "Dolby Digital LED Switch",
];

pub unsafe extern "C" fn snd_audigy2nx_controls_create(mixer: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_audigy2nx_proc_read(
    entry: *mut (),
    buffer: *mut (),
) {
    // TODO: External dependencies
}

// EMU0204
pub unsafe extern "C" fn snd_emu0204_ch_switch_info(
    kcontrol: *mut (),
    uinfo: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_emu0204_ch_switch_get(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_emu0204_ch_switch_update(mixer: *mut (), value: i32) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_emu0204_ch_switch_put(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_emu0204_ch_switch_resume(list: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub static SND_EMU0204_CONTROL: SndKcontrolNew = SndKcontrolNew {
    iface: 0,
    info: 0 as *const (),
    get: 0 as *const (),
    put: 0 as *const (),
};

pub unsafe extern "C" fn snd_emu0204_controls_create(mixer: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

// Sony DualSense controller (PS5) jack detection
// IS_REACHABLE(CONFIG_INPUT) conditional section

const SND_DUALSENSE_JACK_OUT_TERM_ID: u32 = 3;
const SND_DUALSENSE_JACK_IN_TERM_ID: u32 = 4;

#[repr(C)]
pub struct DualsenseMixerElemInfo {
    pub info: [u8; 256],
    pub ih: [u8; 256],
    pub id_table: [[u8; 256]; 2],
    pub connected: bool,
}

pub unsafe extern "C" fn snd_dualsense_ih_event(
    handle: *mut (),
    type_: u32,
    code: u32,
    value: i32,
) {
    // TODO: External dependencies
}

pub unsafe extern "C" fn snd_dualsense_ih_match(
    handler: *mut (),
    dev: *mut (),
) -> bool {
    // TODO: External dependencies
    false
}

pub unsafe extern "C" fn snd_dualsense_ih_connect(
    handler: *mut (),
    dev: *mut (),
    id: *const (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_dualsense_ih_disconnect(handle: *mut ()) {
    // TODO: External dependencies
}

pub unsafe extern "C" fn snd_dualsense_ih_start(handle: *mut ()) {
    // TODO: External dependencies
}

pub unsafe extern "C" fn snd_dualsense_jack_get(
    kctl: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub static SND_DUALSENSE_JACK_CONTROL: SndKcontrolNew = SndKcontrolNew {
    iface: 0,
    info: 0 as *const (),
    get: 0 as *const (),
    put: 0 as *const (),
};

pub unsafe extern "C" fn snd_dualsense_resume_jack(list: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_dualsense_mixer_elem_free(kctl: *mut ()) {
    // TODO: External dependencies
}

pub unsafe extern "C" fn snd_dualsense_jack_create(
    mixer: *mut (),
    name: *const i8,
    is_output: bool,
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_dualsense_controls_create(mixer: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

// ASUS Xonar U1 / U3 controls

pub unsafe extern "C" fn snd_xonar_u1_switch_get(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_xonar_u1_switch_update(
    mixer: *mut (),
    status: u8,
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_xonar_u1_switch_put(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_xonar_u1_switch_resume(list: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub static SND_XONAR_U1_OUTPUT_SWITCH: SndKcontrolNew = SndKcontrolNew {
    iface: 0,
    info: 0 as *const (),
    get: 0 as *const (),
    put: 0 as *const (),
};

pub unsafe extern "C" fn snd_xonar_u1_controls_create(mixer: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

// Digidesign Mbox 1 helper functions

pub unsafe extern "C" fn snd_mbox1_is_spdif_synced(chip: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_mbox1_set_clk_source(chip: *mut (), rate_or_zero: i32) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_mbox1_is_spdif_input(chip: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_mbox1_set_input_source(chip: *mut (), is_spdif: i32) -> i32 {
    // TODO: External dependencies
    0
}

// Digidesign Mbox 1 clock source switch (internal/spdif)

pub unsafe extern "C" fn snd_mbox1_clk_switch_get(
    kctl: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_mbox1_clk_switch_update(mixer: *mut (), is_spdif_sync: i32) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_mbox1_clk_switch_put(
    kctl: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_mbox1_clk_switch_info(
    kcontrol: *mut (),
    uinfo: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_mbox1_clk_switch_resume(list: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

// Digidesign Mbox 1 input source switch (analog/spdif)

pub unsafe extern "C" fn snd_mbox1_src_switch_get(
    kctl: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_mbox1_src_switch_update(mixer: *mut (), is_spdif_input: i32) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_mbox1_src_switch_put(
    kctl: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_mbox1_src_switch_info(
    kcontrol: *mut (),
    uinfo: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_mbox1_src_switch_resume(list: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub static SND_MBOX1_CLK_SWITCH: SndKcontrolNew = SndKcontrolNew {
    iface: 0,
    info: 0 as *const (),
    get: 0 as *const (),
    put: 0 as *const (),
};

pub static SND_MBOX1_SRC_SWITCH: SndKcontrolNew = SndKcontrolNew {
    iface: 0,
    info: 0 as *const (),
    get: 0 as *const (),
    put: 0 as *const (),
};

pub unsafe extern "C" fn snd_mbox1_controls_create(mixer: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

// Native Instruments device quirks

const _MAKE_NI_CONTROL_BSHIFT: u32 = 16;

fn _MAKE_NI_CONTROL(bRequest: u32, wIndex: u32) -> u32 {
    (bRequest << _MAKE_NI_CONTROL_BSHIFT) | wIndex
}

pub unsafe extern "C" fn snd_ni_control_init_val(
    mixer: *mut (),
    kctl: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_nativeinstruments_control_get(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_ni_update_cur_val(list: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_nativeinstruments_control_put(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub static SND_NATIVEINSTRUMENTS_TA6_MIXERS: &[SndKcontrolNew] = &[
    SndKcontrolNew {
        iface: 0,
        info: 0 as *const (),
        get: 0 as *const (),
        put: 0 as *const (),
    },
    SndKcontrolNew {
        iface: 0,
        info: 0 as *const (),
        get: 0 as *const (),
        put: 0 as *const (),
    },
    SndKcontrolNew {
        iface: 0,
        info: 0 as *const (),
        get: 0 as *const (),
        put: 0 as *const (),
    },
    SndKcontrolNew {
        iface: 0,
        info: 0 as *const (),
        get: 0 as *const (),
        put: 0 as *const (),
    },
];

pub static SND_NATIVEINSTRUMENTS_TA10_MIXERS: &[SndKcontrolNew] = &[
    SndKcontrolNew {
        iface: 0,
        info: 0 as *const (),
        get: 0 as *const (),
        put: 0 as *const (),
    },
    SndKcontrolNew {
        iface: 0,
        info: 0 as *const (),
        get: 0 as *const (),
        put: 0 as *const (),
    },
    SndKcontrolNew {
        iface: 0,
        info: 0 as *const (),
        get: 0 as *const (),
        put: 0 as *const (),
    },
    SndKcontrolNew {
        iface: 0,
        info: 0 as *const (),
        get: 0 as *const (),
        put: 0 as *const (),
    },
    SndKcontrolNew {
        iface: 0,
        info: 0 as *const (),
        get: 0 as *const (),
        put: 0 as *const (),
    },
    SndKcontrolNew {
        iface: 0,
        info: 0 as *const (),
        get: 0 as *const (),
        put: 0 as *const (),
    },
    SndKcontrolNew {
        iface: 0,
        info: 0 as *const (),
        get: 0 as *const (),
        put: 0 as *const (),
    },
    SndKcontrolNew {
        iface: 0,
        info: 0 as *const (),
        get: 0 as *const (),
        put: 0 as *const (),
    },
];

pub unsafe extern "C" fn snd_nativeinstruments_create_mixer(
    mixer: *mut (),
    kc: *const SndKcontrolNew,
    count: u32,
) -> i32 {
    // TODO: External dependencies
    0
}

// M-Audio FastTrack Ultra quirks
// FTU Effect switch (also used by C400/C600)

pub unsafe extern "C" fn snd_ftu_eff_switch_info(
    kcontrol: *mut (),
    uinfo: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_ftu_eff_switch_init(
    mixer: *mut (),
    kctl: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_ftu_eff_switch_get(
    kctl: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_ftu_eff_switch_update(list: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_ftu_eff_switch_put(
    kctl: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_ftu_create_effect_switch(
    mixer: *mut (),
    validx: i32,
    bUnitID: i32,
) -> i32 {
    // TODO: External dependencies
    0
}

// Create volume controls for FTU devices
pub unsafe extern "C" fn snd_ftu_create_volume_ctls(mixer: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_ftu_create_effect_volume_ctl(mixer: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_ftu_create_effect_duration_ctl(mixer: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_ftu_create_effect_feedback_ctl(mixer: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_ftu_create_effect_return_ctls(mixer: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_ftu_create_effect_send_ctls(mixer: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_ftu_create_mixer(mixer: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_emuusb_set_samplerate(
    chip: *mut (),
    samplerate_id: u8,
) {
    // TODO: External dependencies
}

// M-Audio Fast Track C400/C600

pub unsafe extern "C" fn snd_c400_create_vol_ctls(mixer: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_c400_create_effect_volume_ctl(mixer: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_c400_create_effect_duration_ctl(mixer: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_c400_create_effect_feedback_ctl(mixer: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_c400_create_effect_vol_ctls(mixer: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_c400_create_effect_ret_vol_ctls(mixer: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_c400_knob_switch_info(
    kcontrol: *mut (),
    uinfo: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_c400_create_knob_switch(
    mixer: *mut (),
    validx: i32,
    bUnitID: i32,
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_c400_create_mixer(mixer: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

// The mixer units for Ebox-44 are corrupt
pub static EBOX44_TABLE: &[StdMonoTable] = &[];

// Audio Advantage Micro II findings

pub unsafe extern "C" fn snd_microii_spdif_info(
    kcontrol: *mut (),
    uinfo: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_microii_spdif_default_get(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_microii_spdif_default_update(list: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_microii_spdif_default_put(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_microii_spdif_mask_get(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_microii_spdif_switch_get(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_microii_spdif_switch_update(list: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_microii_spdif_switch_put(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub static SND_MICROII_MIXER_SPDIF: &[SndKcontrolNew] = &[];

pub unsafe extern "C" fn snd_microii_controls_create(mixer: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

// Creative Sound Blaster E1

pub unsafe extern "C" fn snd_soundblaster_e1_switch_get(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_soundblaster_e1_switch_update(
    mixer: *mut (),
    state: u8,
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_soundblaster_e1_switch_put(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_soundblaster_e1_switch_resume(list: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_soundblaster_e1_switch_info(
    kcontrol: *mut (),
    uinfo: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub static SND_SOUNDBLASTER_E1_INPUT_SWITCH: SndKcontrolNew = SndKcontrolNew {
    iface: 0,
    info: 0 as *const (),
    get: 0 as *const (),
    put: 0 as *const (),
};

pub unsafe extern "C" fn snd_soundblaster_e1_switch_create(mixer: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

// Dell WD15 dock jack detection

const HDA_VERB_CMD_SHIFT: u32 = 20;
const HDA_VERB_CMD_VERB_SHIFT: u32 = 8;

fn HDA_VERB_CMD(N: u32, V: u32, D: u32) -> u32 {
    (N << HDA_VERB_CMD_SHIFT) | (V << HDA_VERB_CMD_VERB_SHIFT) | D
}

const REALTEK_HDA_VALUE: u32 = 0x0038;

const REALTEK_HDA_SET: u32 = 62;
const REALTEK_MANUAL_MODE: u32 = 72;
const REALTEK_HDA_GET_OUT: u32 = 88;
const REALTEK_HDA_GET_IN: u32 = 89;

const REALTEK_AUDIO_FUNCTION_GROUP: u32 = 0x01;
const REALTEK_LINE1: u32 = 0x1a;
const REALTEK_VENDOR_REGISTERS: u32 = 0x20;
const REALTEK_HP_OUT: u32 = 0x21;

const REALTEK_CBJ_CTRL2: u32 = 0x50;

const REALTEK_JACK_INTERRUPT_NODE: u32 = 5;

const REALTEK_MIC_FLAG: u32 = 0x100;

pub unsafe extern "C" fn realtek_hda_set(chip: *mut (), cmd: u32) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn realtek_hda_get(
    chip: *mut (),
    cmd: u32,
    value: *mut u32,
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn realtek_ctl_connector_get(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub static REALTEK_CONNECTOR_CTL_RO: SndKcontrolNew = SndKcontrolNew {
    iface: 0,
    info: 0 as *const (),
    get: 0 as *const (),
    put: 0 as *const (),
};

pub unsafe extern "C" fn realtek_resume_jack(list: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn realtek_add_jack(
    mixer: *mut (),
    name: *mut i8,
    val: u32,
    unitid: i32,
    kctl_new: *const SndKcontrolNew,
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn dell_dock_mixer_create(mixer: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn dell_dock_init_vol(mixer: *mut (), ch: i32, id: i32) {
    // TODO: External dependencies
}

pub unsafe extern "C" fn dell_dock_mixer_init(mixer: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

// HP Thunderbolt Dock G2 jack detection

const HP_DOCK_JACK_INTERRUPT_NODE: u32 = 7;

const HP_DOCK_GET: u32 = 37;

const HP_DOCK_JACK_PRESENCE: u32 = 0xffb8;
const HP_DOCK_JACK_PRESENCE_BIT: u32 = 1 << 2;

const HP_DOCK_MIC_SENSE: u32 = 0xf753;
const HP_DOCK_MIC_SENSE_COMPLETE_BIT: u32 = 1 << 4;

const HP_DOCK_MIC_SENSE_MASK: u32 = (1 << 2) | (1 << 1) | (1 << 0);
const HP_DOCK_MIC_SENSE_NOT_PRESENT: u32 = 0x4;

pub unsafe extern "C" fn hp_dock_ctl_connector_get(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub static HP_DOCK_CONNECTOR_CTL_RO: SndKcontrolNew = SndKcontrolNew {
    iface: 0,
    info: 0 as *const (),
    get: 0 as *const (),
    put: 0 as *const (),
};

pub unsafe extern "C" fn hp_dock_mixer_create(mixer: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

// RME Class Compliant device quirks

const SND_RME_GET_STATUS1: u32 = 23;
const SND_RME_GET_CURRENT_FREQ: u32 = 17;
const SND_RME_CLK_SYSTEM_SHIFT: u32 = 16;
const SND_RME_CLK_SYSTEM_MASK: u32 = 0x1f;
const SND_RME_CLK_AES_SHIFT: u32 = 8;
const SND_RME_CLK_SPDIF_SHIFT: u32 = 12;
const SND_RME_CLK_AES_SPDIF_MASK: u32 = 0xf;
const SND_RME_CLK_SYNC_SHIFT: u32 = 6;
const SND_RME_CLK_SYNC_MASK: u32 = 0x3;
const SND_RME_CLK_FREQMUL_SHIFT: u32 = 18;
const SND_RME_CLK_FREQMUL_MASK: u32 = 0x7;
const SND_RME_CLK_AES_LOCK: u32 = 0x1;
const SND_RME_CLK_AES_SYNC: u32 = 0x4;
const SND_RME_CLK_SPDIF_LOCK: u32 = 0x2;
const SND_RME_CLK_SPDIF_SYNC: u32 = 0x8;
const SND_RME_SPDIF_IF_SHIFT: u32 = 4;
const SND_RME_SPDIF_FORMAT_SHIFT: u32 = 5;
const SND_RME_BINARY_MASK: u32 = 0x1;

fn SND_RME_CLK_SYSTEM(x: u32) -> u32 {
    (x >> SND_RME_CLK_SYSTEM_SHIFT) & SND_RME_CLK_SYSTEM_MASK
}

fn SND_RME_CLK_AES(x: u32) -> u32 {
    (x >> SND_RME_CLK_AES_SHIFT) & SND_RME_CLK_AES_SPDIF_MASK
}

fn SND_RME_CLK_SPDIF(x: u32) -> u32 {
    (x >> SND_RME_CLK_SPDIF_SHIFT) & SND_RME_CLK_AES_SPDIF_MASK
}

fn SND_RME_CLK_SYNC(x: u32) -> u32 {
    (x >> SND_RME_CLK_SYNC_SHIFT) & SND_RME_CLK_SYNC_MASK
}

fn SND_RME_CLK_FREQMUL(x: u32) -> u32 {
    (x >> SND_RME_CLK_FREQMUL_SHIFT) & SND_RME_CLK_FREQMUL_MASK
}

fn SND_RME_SPDIF_IF(x: u32) -> u32 {
    (x >> SND_RME_SPDIF_IF_SHIFT) & SND_RME_BINARY_MASK
}

fn SND_RME_SPDIF_FORMAT(x: u32) -> u32 {
    (x >> SND_RME_SPDIF_FORMAT_SHIFT) & SND_RME_BINARY_MASK
}

pub static SND_RME_RATE_TABLE: &[u32] = &[
    32000, 44100, 48000, 50000,
    64000, 88200, 96000, 100000,
    128000, 176400, 192000, 200000,
    256000, 352800, 384000, 400000,
    512000, 705600, 768000, 800000
];

const SND_RME_RATE_IDX_AES_SPDIF_NUM: u32 = 12;

#[repr(u32)]
pub enum SndRmeDomain {
    SndRmeDomainSystem = 0,
    SndRmeDomainAes = 1,
    SndRmeDomainSpdif = 2,
}

#[repr(u32)]
pub enum SndRmeClockStatus {
    SndRmeClockNolock = 0,
    SndRmeClockLock = 1,
    SndRmeClockSync = 2,
}

pub unsafe extern "C" fn snd_rme_read_value(
    chip: *mut (),
    item: u32,
    value: *mut u32,
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_rme_get_status1(
    kcontrol: *mut (),
    status1: *mut u32,
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_rme_rate_get(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_rme_sync_state_get(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_rme_spdif_if_get(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_rme_spdif_format_get(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_rme_sync_source_get(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_rme_current_freq_get(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_rme_rate_info(
    kcontrol: *mut (),
    uinfo: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_rme_sync_state_info(
    kcontrol: *mut (),
    uinfo: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_rme_spdif_if_info(
    kcontrol: *mut (),
    uinfo: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_rme_spdif_format_info(
    kcontrol: *mut (),
    uinfo: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_rme_sync_source_info(
    kcontrol: *mut (),
    uinfo: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub static SND_RME_CONTROLS: &[SndKcontrolNew] = &[];

pub unsafe extern "C" fn snd_rme_controls_create(mixer: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

// RME Babyface Pro (FS)

#[repr(u32)]
pub enum SndBbfproCtlReg {
    SndBbfproCtlReg1 = 0,
    SndBbfproCtlReg2 = 1,
}

const SND_BBFPRO_CTL_REG_MASK: u32 = 1;
const SND_BBFPRO_CTL_IDX_MASK: u32 = 0xff;
const SND_BBFPRO_CTL_IDX_SHIFT: u32 = 1;
const SND_BBFPRO_CTL_VAL_MASK: u32 = 1;
const SND_BBFPRO_CTL_VAL_SHIFT: u32 = 9;
const SND_BBFPRO_CTL_REG1_CLK_MASTER: u32 = 0;
const SND_BBFPRO_CTL_REG1_CLK_OPTICAL: u32 = 1;
const SND_BBFPRO_CTL_REG1_SPDIF_PRO: u32 = 7;
const SND_BBFPRO_CTL_REG1_SPDIF_EMPH: u32 = 8;
const SND_BBFPRO_CTL_REG1_SPDIF_OPTICAL: u32 = 10;
const SND_BBFPRO_CTL_REG2_48V_AN1: u32 = 0;
const SND_BBFPRO_CTL_REG2_48V_AN2: u32 = 1;
const SND_BBFPRO_CTL_REG2_SENS_IN3: u32 = 2;
const SND_BBFPRO_CTL_REG2_SENS_IN4: u32 = 3;
const SND_BBFPRO_CTL_REG2_PAD_AN1: u32 = 4;
const SND_BBFPRO_CTL_REG2_PAD_AN2: u32 = 5;

const SND_BBFPRO_MIXER_MAIN_OUT_CH_OFFSET: u32 = 992;
const SND_BBFPRO_MIXER_IDX_MASK: u32 = 0x3ff;
const SND_BBFPRO_MIXER_VAL_MASK: u32 = 0x3ffff;
const SND_BBFPRO_MIXER_VAL_SHIFT: u32 = 9;
const SND_BBFPRO_MIXER_VAL_MIN: u32 = 0;
const SND_BBFPRO_MIXER_VAL_MAX: u32 = 65536;

const SND_BBFPRO_GAIN_CHANNEL_MASK: u32 = 0x03;
const SND_BBFPRO_GAIN_CHANNEL_SHIFT: u32 = 7;
const SND_BBFPRO_GAIN_VAL_MASK: u32 = 0x7f;
const SND_BBFPRO_GAIN_VAL_MIN: u32 = 0;
const SND_BBFPRO_GAIN_VAL_MIC_MAX: u32 = 65;
const SND_BBFPRO_GAIN_VAL_LINE_MAX: u32 = 18;

const SND_BBFPRO_USBREQ_CTL_REG1: u32 = 0x10;
const SND_BBFPRO_USBREQ_CTL_REG2: u32 = 0x17;
const SND_BBFPRO_USBREQ_GAIN: u32 = 0x1a;
const SND_BBFPRO_USBREQ_MIXER: u32 = 0x12;

pub unsafe extern "C" fn snd_bbfpro_ctl_update(
    mixer: *mut (),
    reg: u8,
    index: u8,
    value: u8,
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_bbfpro_ctl_get(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_bbfpro_ctl_info(
    kcontrol: *mut (),
    uinfo: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_bbfpro_ctl_put(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_bbfpro_ctl_resume(list: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_bbfpro_gain_update(
    mixer: *mut (),
    channel: u8,
    gain: u8,
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_bbfpro_gain_get(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_bbfpro_gain_info(
    kcontrol: *mut (),
    uinfo: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_bbfpro_gain_put(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_bbfpro_gain_resume(list: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_bbfpro_vol_update(
    mixer: *mut (),
    index: u16,
    value: u32,
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_bbfpro_vol_get(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_bbfpro_vol_info(
    kcontrol: *mut (),
    uinfo: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_bbfpro_vol_put(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_bbfpro_vol_resume(list: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub static SND_BBFPRO_CTL_CONTROL: SndKcontrolNew = SndKcontrolNew {
    iface: 0,
    info: 0 as *const (),
    get: 0 as *const (),
    put: 0 as *const (),
};

pub static SND_BBFPRO_GAIN_CONTROL: SndKcontrolNew = SndKcontrolNew {
    iface: 0,
    info: 0 as *const (),
    get: 0 as *const (),
    put: 0 as *const (),
};

pub static SND_BBFPRO_VOL_CONTROL: SndKcontrolNew = SndKcontrolNew {
    iface: 0,
    info: 0 as *const (),
    get: 0 as *const (),
    put: 0 as *const (),
};

pub unsafe extern "C" fn snd_bbfpro_ctl_add(
    mixer: *mut (),
    reg: u8,
    index: u8,
    name: *mut i8,
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_bbfpro_gain_add(
    mixer: *mut (),
    channel: u8,
    name: *mut i8,
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_bbfpro_vol_add(
    mixer: *mut (),
    index: u16,
    name: *mut i8,
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_bbfpro_controls_create(mixer: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

// RME Digiface USB

const RME_DIGIFACE_READ_STATUS: u32 = 17;
const RME_DIGIFACE_STATUS_REG0L: u32 = 0;
const RME_DIGIFACE_STATUS_REG0H: u32 = 1;
const RME_DIGIFACE_STATUS_REG1L: u32 = 2;
const RME_DIGIFACE_STATUS_REG1H: u32 = 3;
const RME_DIGIFACE_STATUS_REG2L: u32 = 4;
const RME_DIGIFACE_STATUS_REG2H: u32 = 5;
const RME_DIGIFACE_STATUS_REG3L: u32 = 6;
const RME_DIGIFACE_STATUS_REG3H: u32 = 7;

const RME_DIGIFACE_CTL_REG1: u32 = 16;
const RME_DIGIFACE_CTL_REG2: u32 = 18;

const RME_DIGIFACE_INVERT: u32 = 1 << 31;

fn RME_DIGIFACE_REGISTER(reg: u32, mask: u32) -> u32 {
    ((reg) << 16) | (mask)
}

pub unsafe extern "C" fn snd_rme_digiface_write_reg(
    kcontrol: *mut (),
    item: i32,
    mask: u16,
    val: u16,
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_rme_digiface_read_status(
    kcontrol: *mut (),
    status: *mut u32,
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_rme_digiface_get_status_val(
    kcontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_rme_digiface_rate_get(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_rme_digiface_enum_get(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_rme_digiface_enum_put(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_rme_digiface_current_sync_get(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_rme_digiface_sync_state_get(
    kcontrol: *mut (),
    ucontrol: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_rme_digiface_format_info(
    kcontrol: *mut (),
    uinfo: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_rme_digiface_sync_source_info(
    kcontrol: *mut (),
    uinfo: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_rme_digiface_rate_info(
    kcontrol: *mut (),
    uinfo: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub static SND_RME_DIGIFACE_CONTROLS: &[SndKcontrolNew] = &[];

pub unsafe extern "C" fn snd_rme_digiface_controls_create(mixer: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

// Pioneer DJ / AlphaTheta DJM Mixers

// Capture types
const SND_DJM_CAP_LINE: u16 = 0x00;
const SND_DJM_CAP_CDLINE: u16 = 0x01;
const SND_DJM_CAP_DIGITAL: u16 = 0x02;
const SND_DJM_CAP_PHONO: u16 = 0x03;
const SND_DJM_CAP_PREFADER: u16 = 0x05;
const SND_DJM_CAP_PFADER: u16 = 0x06;
const SND_DJM_CAP_XFADERA: u16 = 0x07;
const SND_DJM_CAP_XFADERB: u16 = 0x08;
const SND_DJM_CAP_MIC: u16 = 0x09;
const SND_DJM_CAP_AUX: u16 = 0x0d;
const SND_DJM_CAP_RECOUT: u16 = 0x0a;
const SND_DJM_CAP_RECOUT_NOMIC: u16 = 0x0e;
const SND_DJM_CAP_NONE: u16 = 0x0f;
const SND_DJM_CAP_FXSEND: u16 = 0x10;
const SND_DJM_CAP_CH1PFADER: u16 = 0x11;
const SND_DJM_CAP_CH2PFADER: u16 = 0x12;
const SND_DJM_CAP_CH3PFADER: u16 = 0x13;
const SND_DJM_CAP_CH4PFADER: u16 = 0x14;
const SND_DJM_CAP_EXT1SEND: u16 = 0x21;
const SND_DJM_CAP_EXT2SEND: u16 = 0x22;
const SND_DJM_CAP_CH1PREFADER: u16 = 0x31;
const SND_DJM_CAP_CH2PREFADER: u16 = 0x32;
const SND_DJM_CAP_CH3PREFADER: u16 = 0x33;
const SND_DJM_CAP_CH4PREFADER: u16 = 0x34;

// Playback types
const SND_DJM_PB_CH1: u16 = 0x00;
const SND_DJM_PB_CH2: u16 = 0x01;
const SND_DJM_PB_AUX: u16 = 0x04;

const SND_DJM_WINDEX_CAP: u16 = 0x8002;
const SND_DJM_WINDEX_CAPLVL: u16 = 0x8003;
const SND_DJM_WINDEX_PB: u16 = 0x8016;

// kcontrol->private_value layout
const SND_DJM_VALUE_MASK: u64 = 0x0000ffff;
const SND_DJM_GROUP_MASK: u64 = 0x00ff0000;
const SND_DJM_DEVICE_MASK: u64 = 0xff000000;
const SND_DJM_GROUP_SHIFT: u64 = 16;
const SND_DJM_DEVICE_SHIFT: u64 = 24;

// device table index
const SND_DJM_250MK2_IDX: u8 = 0x0;
const SND_DJM_750_IDX: u8 = 0x1;
const SND_DJM_850_IDX: u8 = 0x2;
const SND_DJM_900NXS2_IDX: u8 = 0x3;
const SND_DJM_750MK2_IDX: u8 = 0x4;
const SND_DJM_450_IDX: u8 = 0x5;
const SND_DJM_A9_IDX: u8 = 0x6;
const SND_DJM_V10_IDX: u8 = 0x7;
const SND_DJM_S11_IDX: u8 = 0x8;

#[repr(C)]
pub struct SndDjmDevice {
    pub name: *const i8,
    pub controls: *const SndDjmCtl,
    pub ncontrols: usize,
}

#[repr(C)]
pub struct SndDjmCtl {
    pub name: *const i8,
    pub options: *const u16,
    pub noptions: usize,
    pub default_value: u16,
    pub wIndex: u16,
}

pub unsafe extern "C" fn snd_djm_get_label_caplevel_common(wvalue: u16) -> *const i8 {
    // TODO: External dependencies
    0 as *const i8
}

pub unsafe extern "C" fn snd_djm_get_label_caplevel_high(wvalue: u16) -> *const i8 {
    // TODO: External dependencies
    0 as *const i8
}

pub unsafe extern "C" fn snd_djm_get_label_cap_common(wvalue: u16) -> *const i8 {
    // TODO: External dependencies
    0 as *const i8
}

pub unsafe extern "C" fn snd_djm_get_label_cap_850(wvalue: u16) -> *const i8 {
    // TODO: External dependencies
    0 as *const i8
}

pub unsafe extern "C" fn snd_djm_get_label_caplevel(
    device_idx: u8,
    wvalue: u16,
) -> *const i8 {
    // TODO: External dependencies
    0 as *const i8
}

pub unsafe extern "C" fn snd_djm_get_label_cap(
    device_idx: u8,
    wvalue: u16,
) -> *const i8 {
    // TODO: External dependencies
    0 as *const i8
}

pub unsafe extern "C" fn snd_djm_get_label_pb(wvalue: u16) -> *const i8 {
    // TODO: External dependencies
    0 as *const i8
}

pub unsafe extern "C" fn snd_djm_get_label(
    device_idx: u8,
    wvalue: u16,
    windex: u16,
) -> *const i8 {
    // TODO: External dependencies
    0 as *const i8
}

pub static SND_DJM_OPTS_CAP_LEVEL: &[u16] = &[0x0000, 0x0100, 0x0200, 0x0300];

pub static SND_DJM_OPTS_250MK2_CAP1: &[u16] = &[
    0x0103, 0x0100, 0x0106, 0x0107, 0x0108, 0x0109, 0x010d, 0x010a
];

pub static SND_DJM_OPTS_250MK2_CAP2: &[u16] = &[
    0x0203, 0x0200, 0x0206, 0x0207, 0x0208, 0x0209, 0x020d, 0x020a
];

pub static SND_DJM_OPTS_250MK2_CAP3: &[u16] = &[
    0x030a, 0x0311, 0x0312, 0x0307, 0x0308, 0x0309, 0x030d
];

pub static SND_DJM_OPTS_250MK2_PB1: &[u16] = &[0x0100, 0x0101, 0x0104];
pub static SND_DJM_OPTS_250MK2_PB2: &[u16] = &[0x0200, 0x0201, 0x0204];
pub static SND_DJM_OPTS_250MK2_PB3: &[u16] = &[0x0300, 0x0301, 0x0304];

pub unsafe extern "C" fn snd_djm_controls_info(
    kctl: *mut (),
    info: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_djm_controls_update(
    mixer: *mut (),
    device_idx: u8,
    group: u8,
    value: u16,
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_djm_controls_get(
    kctl: *mut (),
    elem: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_djm_controls_put(
    kctl: *mut (),
    elem: *mut (),
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_djm_controls_resume(list: *mut ()) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_djm_controls_create(
    mixer: *mut (),
    device_idx: u8,
) -> i32 {
    // TODO: External dependencies
    0
}

pub unsafe extern "C" fn snd_usb_mixer_apply_create_quirk(mixer: *mut ()) -> i32 {
    // TODO: External dependencies - main entry point for creating mixer quirks
    0
}

pub unsafe extern "C" fn snd_usb_mixer_resume_quirk(mixer: *mut ()) {
    // TODO: External dependencies
}

pub unsafe extern "C" fn snd_usb_mixer_rc_memory_change(
    mixer: *mut (),
    unitid: i32,
) {
    // TODO: External dependencies
}

pub unsafe extern "C" fn snd_dragonfly_quirk_db_scale(
    mixer: *mut (),
    cval: *mut (),
    kctl: *mut (),
) {
    // TODO: External dependencies
}

pub unsafe extern "C" fn snd_usb_mv_silicon_quirks(
    mixer: *mut (),
    cval: *mut (),
    kctl: *mut (),
) {
    // TODO: External dependencies
}

pub unsafe extern "C" fn snd_fix_plt_name(
    chip: *mut (),
    id: *mut (),
) {
    // TODO: External dependencies
}

pub unsafe extern "C" fn snd_usb_mixer_fu_quirk_linear_scale(
    mixer: *mut (),
    cval: *mut (),
    kctl: *mut (),
) {
    // TODO: External dependencies
}

pub unsafe extern "C" fn snd_usb_mixer_fu_apply_quirk(
    mixer: *mut (),
    cval: *mut (),
    unitid: i32,
    kctl: *mut (),
) {
    // TODO: External dependencies
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
