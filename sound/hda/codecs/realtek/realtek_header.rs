// SPDX-License-Identifier: GPL-2.0-or-later
//
// Realtek HD-audio codec support code
//

// C header guard and include directives omitted.
// Dependencies expected from surrounding translation:
// linux/acpi.h, linux/cleanup.h, linux/init.h, linux/delay.h,
// linux/slab.h, linux/pci.h, linux/dmi.h, linux/module.h, linux/i2c.h,
// linux/input.h, linux/leds.h, linux/ctype.h, linux/spi/spi.h,
// sound/core.h, sound/jack.h, sound/hda_codec.h, hda_local.h,
// hda_auto_parser.h, hda_beep.h, hda_jack.h, generic.h,
// side-codecs/hda_component.h.

/* extra amp-initialization sequence types */
pub const ALC_INIT_UNDEFINED: ::core::ffi::c_uint = 0;
pub const ALC_INIT_NONE: ::core::ffi::c_uint = 1;
pub const ALC_INIT_DEFAULT: ::core::ffi::c_uint = 2;

pub const ALC_HEADSET_MODE_UNKNOWN: ::core::ffi::c_uint = 0;
pub const ALC_HEADSET_MODE_UNPLUGGED: ::core::ffi::c_uint = 1;
pub const ALC_HEADSET_MODE_HEADSET: ::core::ffi::c_uint = 2;
pub const ALC_HEADSET_MODE_MIC: ::core::ffi::c_uint = 3;
pub const ALC_HEADSET_MODE_HEADPHONE: ::core::ffi::c_uint = 4;

pub const ALC_HEADSET_TYPE_UNKNOWN: ::core::ffi::c_uint = 0;
pub const ALC_HEADSET_TYPE_CTIA: ::core::ffi::c_uint = 1;
pub const ALC_HEADSET_TYPE_OMTP: ::core::ffi::c_uint = 2;

pub const ALC_KEY_MICMUTE_INDEX: ::core::ffi::c_uint = 0;

#[repr(C)]
pub struct alc_customize_define {
    pub sku_cfg: ::core::ffi::c_uint,
    pub port_connectivity: ::core::ffi::c_uchar,
    pub check_sum: ::core::ffi::c_uchar,
    pub customization: ::core::ffi::c_uchar,
    pub external_amp: ::core::ffi::c_uchar,
    // C bitfields: unsigned int enable_pcbeep:1, platform_type:1,
    // swap:1, override:1, fixup:1.
    pub enable_pcbeep: ::core::ffi::c_uint,
    pub platform_type: ::core::ffi::c_uint,
    pub swap: ::core::ffi::c_uint,
    pub override_: ::core::ffi::c_uint,
    pub fixup: ::core::ffi::c_uint, /* Means that this sku is set by driver, not read from hw */
}

#[repr(C)]
pub struct alc_coef_led {
    pub idx: ::core::ffi::c_uint,
    pub mask: ::core::ffi::c_uint,
    pub on: ::core::ffi::c_uint,
    pub off: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct alc_spec {
    pub gen: hda_gen_spec, /* must be at head */

    /* codec parameterization */
    pub cdefine: alc_customize_define,
    pub parse_flags: ::core::ffi::c_uint, /* flag for snd_hda_parse_pin_defcfg() */

    /* GPIO bits */
    pub gpio_mask: ::core::ffi::c_uint,
    pub gpio_dir: ::core::ffi::c_uint,
    pub gpio_data: ::core::ffi::c_uint,
    pub gpio_write_delay: bool, /* add a delay before writing gpio_data */

    /* mute LED for HP laptops, see vref_mute_led_set() */
    pub mute_led_polarity: ::core::ffi::c_int,
    pub micmute_led_polarity: ::core::ffi::c_int,
    pub mute_led_nid: hda_nid_t,
    pub cap_mute_led_nid: hda_nid_t,

    pub gpio_mute_led_mask: ::core::ffi::c_uint,
    pub gpio_mic_led_mask: ::core::ffi::c_uint,
    pub mute_led_coef: alc_coef_led,
    pub mic_led_coef: alc_coef_led,
    pub coef_mutex: mutex,

    pub headset_mic_pin: hda_nid_t,
    pub headphone_mic_pin: hda_nid_t,
    pub current_headset_mode: ::core::ffi::c_int,
    pub current_headset_type: ::core::ffi::c_int,

    /* hooks */
    pub init_hook: Option<unsafe extern "C" fn(codec: *mut hda_codec)>,
    pub power_hook: Option<unsafe extern "C" fn(codec: *mut hda_codec)>,
    pub shutup: Option<unsafe extern "C" fn(codec: *mut hda_codec)>,

    pub init_amp: ::core::ffi::c_int,
    pub codec_variant: ::core::ffi::c_int, /* flag for other variants */
    // C bitfields: unsigned int has_alc5505_dsp:1, no_depop_delay:1,
    // done_hp_init:1, no_shutup_pins:1, ultra_low_power:1, has_hs_key:1,
    // no_internal_mic_pin:1, en_3kpull_low:1.
    pub has_alc5505_dsp: ::core::ffi::c_uint,
    pub no_depop_delay: ::core::ffi::c_uint,
    pub done_hp_init: ::core::ffi::c_uint,
    pub no_shutup_pins: ::core::ffi::c_uint,
    pub ultra_low_power: ::core::ffi::c_uint,
    pub has_hs_key: ::core::ffi::c_uint,
    pub no_internal_mic_pin: ::core::ffi::c_uint,
    pub en_3kpull_low: ::core::ffi::c_uint,
    pub num_speaker_amps: ::core::ffi::c_int,

    /* for PLL fix */
    pub pll_nid: hda_nid_t,
    pub pll_coef_idx: ::core::ffi::c_uint,
    pub pll_coef_bit: ::core::ffi::c_uint,
    pub coef0: ::core::ffi::c_uint,
    pub kb_dev: *mut input_dev,
    pub alc_mute_keycode_map: [u8; 1],

    /* component binding */
    pub comps: hda_component_parent,
}

unsafe extern "C" {
    pub fn alc_read_coefex_idx(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        coef_idx: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn alc_write_coefex_idx(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        coef_idx: ::core::ffi::c_uint,
        coef_val: ::core::ffi::c_uint,
    );
    pub fn alc_update_coefex_idx(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        coef_idx: ::core::ffi::c_uint,
        mask: ::core::ffi::c_uint,
        bits_set: ::core::ffi::c_uint,
    );
}

pub unsafe fn alc_read_coef_idx(
    codec: *mut hda_codec,
    coef_idx: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    unsafe { alc_read_coefex_idx(codec, 0x20, coef_idx) }
}

pub unsafe fn alc_write_coef_idx(
    codec: *mut hda_codec,
    coef_idx: ::core::ffi::c_uint,
    coef_val: ::core::ffi::c_uint,
) {
    unsafe { alc_write_coefex_idx(codec, 0x20, coef_idx, coef_val) }
}

pub unsafe fn alc_update_coef_idx(
    codec: *mut hda_codec,
    coef_idx: ::core::ffi::c_uint,
    mask: ::core::ffi::c_uint,
    bits_set: ::core::ffi::c_uint,
) {
    unsafe { alc_update_coefex_idx(codec, 0x20, coef_idx, mask, bits_set) }
}

unsafe extern "C" {
    pub fn alc_get_coef0(codec: *mut hda_codec) -> ::core::ffi::c_uint;
}

/* coef writes/updates batch */
#[repr(C)]
pub struct coef_fw {
    pub nid: ::core::ffi::c_uchar,
    pub idx: ::core::ffi::c_uchar,
    pub mask: ::core::ffi::c_ushort,
    pub val: ::core::ffi::c_ushort,
}

pub const fn UPDATE_COEFEX(
    _nid: ::core::ffi::c_uchar,
    _idx: ::core::ffi::c_uchar,
    _mask: ::core::ffi::c_ushort,
    _val: ::core::ffi::c_ushort,
) -> coef_fw {
    coef_fw {
        nid: _nid,
        idx: _idx,
        mask: _mask,
        val: _val,
    }
}

pub const fn WRITE_COEFEX(
    _nid: ::core::ffi::c_uchar,
    _idx: ::core::ffi::c_uchar,
    _val: ::core::ffi::c_ushort,
) -> coef_fw {
    UPDATE_COEFEX(_nid, _idx, !0u16, _val)
}

pub const fn WRITE_COEF(_idx: ::core::ffi::c_uchar, _val: ::core::ffi::c_ushort) -> coef_fw {
    WRITE_COEFEX(0x20, _idx, _val)
}

pub const fn UPDATE_COEF(
    _idx: ::core::ffi::c_uchar,
    _mask: ::core::ffi::c_ushort,
    _val: ::core::ffi::c_ushort,
) -> coef_fw {
    UPDATE_COEFEX(0x20, _idx, _mask, _val)
}

unsafe extern "C" {
    pub fn alc_process_coef_fw(codec: *mut hda_codec, fw: *const coef_fw);

    /*
     * GPIO helpers
     */
    pub fn alc_setup_gpio(codec: *mut hda_codec, mask: ::core::ffi::c_uint);
    pub fn alc_update_gpio_data(codec: *mut hda_codec, mask: ::core::ffi::c_uint, on: bool);

    /* common GPIO fixups */
    pub fn alc_fixup_gpio(codec: *mut hda_codec, action: ::core::ffi::c_int, mask: ::core::ffi::c_uint);
    pub fn alc_fixup_gpio1(codec: *mut hda_codec, fix: *const hda_fixup, action: ::core::ffi::c_int);
    pub fn alc_fixup_gpio2(codec: *mut hda_codec, fix: *const hda_fixup, action: ::core::ffi::c_int);
    pub fn alc_fixup_gpio3(codec: *mut hda_codec, fix: *const hda_fixup, action: ::core::ffi::c_int);
    pub fn alc_fixup_gpio4(codec: *mut hda_codec, fix: *const hda_fixup, action: ::core::ffi::c_int);
    pub fn alc_fixup_micmute_led(codec: *mut hda_codec, fix: *const hda_fixup, action: ::core::ffi::c_int);

    /*
     * Common init code, callbacks and helpers
     */
    pub fn alc_fix_pll(codec: *mut hda_codec);
    pub fn alc_fix_pll_init(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        coef_idx: ::core::ffi::c_uint,
        coef_bit: ::core::ffi::c_uint,
    );
    pub fn alc_fill_eapd_coef(codec: *mut hda_codec);
    pub fn alc_auto_setup_eapd(codec: *mut hda_codec, on: bool);

    pub fn alc_find_ext_mic_pin(codec: *mut hda_codec) -> ::core::ffi::c_int;
    pub fn alc_headset_mic_no_shutup(codec: *mut hda_codec);
    pub fn alc_shutup_pins(codec: *mut hda_codec);
    pub fn alc_eapd_shutup(codec: *mut hda_codec);
    pub fn alc_auto_init_amp(codec: *mut hda_codec, type_: ::core::ffi::c_int);
    pub fn alc_get_hp_pin(spec: *mut alc_spec) -> hda_nid_t;
    pub fn alc_auto_parse_customize_define(codec: *mut hda_codec) -> ::core::ffi::c_int;
    pub fn alc_subsystem_id(codec: *mut hda_codec, ports: *const hda_nid_t) -> ::core::ffi::c_int;
    pub fn alc_ssid_check(codec: *mut hda_codec, ports: *const hda_nid_t);
    pub fn alc_build_controls(codec: *mut hda_codec) -> ::core::ffi::c_int;
    pub fn alc_update_knob_master(codec: *mut hda_codec, jack: *mut hda_jack_callback);
}

pub unsafe fn alc_pre_init(codec: *mut hda_codec) {
    unsafe { alc_fill_eapd_coef(codec) }
}

pub unsafe fn is_s3_resume(codec: *mut hda_codec) -> bool {
    unsafe { (*codec).core.dev.power.power_state.event == PM_EVENT_RESUME }
}

pub unsafe fn is_s4_resume(codec: *mut hda_codec) -> bool {
    unsafe { (*codec).core.dev.power.power_state.event == PM_EVENT_RESTORE }
}

pub unsafe fn is_s4_suspend(codec: *mut hda_codec) -> bool {
    unsafe { (*codec).core.dev.power.power_state.event == PM_EVENT_FREEZE }
}

unsafe extern "C" {
    pub fn alc_init(codec: *mut hda_codec) -> ::core::ffi::c_int;
    pub fn alc_shutup(codec: *mut hda_codec);
    pub fn alc_power_eapd(codec: *mut hda_codec);
    pub fn alc_suspend(codec: *mut hda_codec) -> ::core::ffi::c_int;
    pub fn alc_resume(codec: *mut hda_codec) -> ::core::ffi::c_int;

    pub fn alc_parse_auto_config(
        codec: *mut hda_codec,
        ignore_nids: *const hda_nid_t,
        ssid_nids: *const hda_nid_t,
    ) -> ::core::ffi::c_int;
    pub fn alc_alloc_spec(codec: *mut hda_codec, mixer_nid: hda_nid_t) -> ::core::ffi::c_int;

    pub fn snd_hda_codec_set_name(codec: *mut hda_codec, name: *const ::core::ffi::c_char);
}

pub unsafe fn alc_codec_rename(codec: *mut hda_codec, name: *const ::core::ffi::c_char) {
    unsafe { snd_hda_codec_set_name(codec, name) }
}

// CONFIG_SND_HDA_INPUT_BEEP conditional:
// When enabled, set_beep_amp and has_cdefine_beep alias the ALC functions.
// Otherwise, both macros evaluate to 0.
#[cfg(CONFIG_SND_HDA_INPUT_BEEP)]
unsafe extern "C" {
    pub fn alc_set_beep_amp(
        spec: *mut alc_spec,
        nid: hda_nid_t,
        idx: ::core::ffi::c_int,
        dir: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn alc_has_cdefine_beep(codec: *mut hda_codec) -> ::core::ffi::c_int;
}

#[cfg(CONFIG_SND_HDA_INPUT_BEEP)]
pub unsafe fn set_beep_amp(
    spec: *mut alc_spec,
    nid: hda_nid_t,
    idx: ::core::ffi::c_int,
    dir: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe { alc_set_beep_amp(spec, nid, idx, dir) }
}

#[cfg(CONFIG_SND_HDA_INPUT_BEEP)]
pub unsafe fn has_cdefine_beep(codec: *mut hda_codec) -> ::core::ffi::c_int {
    unsafe { alc_has_cdefine_beep(codec) }
}

#[cfg(not(CONFIG_SND_HDA_INPUT_BEEP))]
pub fn set_beep_amp(
    _spec: *mut alc_spec,
    _nid: hda_nid_t,
    _idx: ::core::ffi::c_int,
    _dir: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_SND_HDA_INPUT_BEEP))]
pub fn has_cdefine_beep(_codec: *mut hda_codec) -> ::core::ffi::c_int {
    0
}

unsafe extern "C" {
    pub fn snd_hda_find_mixer_ctl(
        codec: *mut hda_codec,
        oldname: *const ::core::ffi::c_char,
    ) -> *mut snd_kcontrol;
    pub fn snd_ctl_rename(
        card: *mut snd_card,
        kctl: *mut snd_kcontrol,
        newname: *const ::core::ffi::c_char,
    );
}

pub unsafe fn rename_ctl(
    codec: *mut hda_codec,
    oldname: *const ::core::ffi::c_char,
    newname: *const ::core::ffi::c_char,
) {
    let kctl: *mut snd_kcontrol;

    kctl = unsafe { snd_hda_find_mixer_ctl(codec, oldname) };
    if !kctl.is_null() {
        unsafe { snd_ctl_rename((*codec).card, kctl, newname) };
    }
}

unsafe extern "C" {
    /* Common fixups */
    pub fn alc_fixup_sku_ignore(codec: *mut hda_codec, fix: *const hda_fixup, action: ::core::ffi::c_int);
    pub fn alc_fixup_no_depop_delay(codec: *mut hda_codec, fix: *const hda_fixup, action: ::core::ffi::c_int);
    pub fn alc_fixup_inv_dmic(codec: *mut hda_codec, fix: *const hda_fixup, action: ::core::ffi::c_int);
    pub fn alc_fixup_dual_codecs(codec: *mut hda_codec, fix: *const hda_fixup, action: ::core::ffi::c_int);
    pub fn alc_fixup_bass_chmap(codec: *mut hda_codec, fix: *const hda_fixup, action: ::core::ffi::c_int);
    pub fn alc_fixup_headset_mode(codec: *mut hda_codec, fix: *const hda_fixup, action: ::core::ffi::c_int);
    pub fn alc_fixup_headset_mode_no_hp_mic(codec: *mut hda_codec, fix: *const hda_fixup, action: ::core::ffi::c_int);
    pub fn alc_fixup_headset_mic(codec: *mut hda_codec, fix: *const hda_fixup, action: ::core::ffi::c_int);
    pub fn alc_update_headset_jack_cb(codec: *mut hda_codec, jack: *mut hda_jack_callback);
    pub fn alc_update_gpio_led(
        codec: *mut hda_codec,
        mask: ::core::ffi::c_uint,
        polarity: ::core::ffi::c_int,
        enabled: bool,
    );
    pub fn alc_fixup_hp_gpio_led(
        codec: *mut hda_codec,
        action: ::core::ffi::c_int,
        mute_mask: ::core::ffi::c_uint,
        micmute_mask: ::core::ffi::c_uint,
    );
    pub fn alc_fixup_no_jack_detect(codec: *mut hda_codec, fix: *const hda_fixup, action: ::core::ffi::c_int);
    pub fn alc_fixup_disable_aamix(codec: *mut hda_codec, fix: *const hda_fixup, action: ::core::ffi::c_int);
    pub fn alc_fixup_auto_mute_via_amp(codec: *mut hda_codec, fix: *const hda_fixup, action: ::core::ffi::c_int);

    /* device-specific, but used by multiple codec drivers */
    pub fn alc1220_fixup_gb_dual_codecs(codec: *mut hda_codec, fix: *const hda_fixup, action: ::core::ffi::c_int);
    pub fn alc233_alc662_fixup_lenovo_dual_codecs(
        codec: *mut hda_codec,
        fix: *const hda_fixup,
        action: ::core::ffi::c_int,
    );
    pub fn alc_fixup_dell_xps13(codec: *mut hda_codec, fix: *const hda_fixup, action: ::core::ffi::c_int);
}

/*
 * COEF access helper functions
 */
pub unsafe fn coef_mutex_lock(codec: *mut hda_codec) {
    let spec: *mut alc_spec = unsafe { (*codec).spec as *mut alc_spec };

    unsafe { snd_hda_power_up_pm(codec) };
    unsafe { mutex_lock(&mut (*spec).coef_mutex) };
}

pub unsafe fn coef_mutex_unlock(codec: *mut hda_codec) {
    let spec: *mut alc_spec = unsafe { (*codec).spec as *mut alc_spec };

    unsafe { mutex_unlock(&mut (*spec).coef_mutex) };
    unsafe { snd_hda_power_down_pm(codec) };
}

unsafe extern "C" {
    pub fn snd_hda_power_up_pm(codec: *mut hda_codec);
    pub fn snd_hda_power_down_pm(codec: *mut hda_codec);
    pub fn mutex_lock(lock: *mut mutex);
    pub fn mutex_unlock(lock: *mut mutex);
}

// DEFINE_GUARD(coef_mutex, struct hda_codec *, coef_mutex_lock(_T), coef_mutex_unlock(_T))

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
