// SPDX-License-Identifier: GPL-2.0+
/*
 * originally written by: Kirk Reiser <kirk@braille.uwo.ca>
 * this version considerably modified by David Borowski, david575@rogers.com
 * eventually modified by Samuel Thibault <samuel.thibault@ens-lyon.org>
 *
 * Copyright (C) 1998-99  Kirk Reiser.
 * Copyright (C) 2003 David Borowski.
 * Copyright (C) 2007 Samuel Thibault.
 *
 * specifically written as a driver for the speakup screenreview
 * s not a general device driver.
 */
// Dependencies supplied by spk_priv.h and speakup.h remain external.

const PROCSPEECH: u8 = b'\n';
const DRV_VERSION: &str = "2.11";
const SYNTH_CLEAR: u8 = b'!';

#[repr(C)]
#[derive(Copy, Clone)]
enum DefaultVarsId {
    CAPS_START_ID = 0,
    CAPS_STOP_ID,
    PAUSE_ID,
    RATE_ID,
    PITCH_ID,
    INFLECTION_ID,
    VOL_ID,
    TONE_ID,
    PUNCT_ID,
    DIRECT_ID,
    V_LAST_VAR_ID,
    NB_ID,
}

// The following types, constants, functions, and macros are provided by the
// Speakup and kernel interfaces included by the original C source.
extern "C" {
    fn spk_var_show();
    fn spk_var_store();
    fn spk_ttyio_synth_probe();
    fn spk_ttyio_release();
    fn spk_ttyio_synth_immediate();
    fn spk_do_catch_up_unicode();
    fn spk_synth_flush();
    fn spk_synth_is_alive_restart();
    fn pr_info(fmt: *const u8, ...);
}

// struct var_t vars[NB_ID] = ...; represented using the external Speakup
// layout and designated initializers from the C implementation.
#[allow(non_upper_case_globals)]
static mut vars: [var_t; NB_ID as usize] = [var_t::default(); NB_ID as usize];

// These attributes will appear in /sys/accessibility/speakup/dummy.
// __ATTR(name, 0644, spk_var_show, spk_var_store)
static mut caps_start_attribute: kobj_attribute = kobj_attribute::default();
static mut caps_stop_attribute: kobj_attribute = kobj_attribute::default();
static mut pitch_attribute: kobj_attribute = kobj_attribute::default();
static mut inflection_attribute: kobj_attribute = kobj_attribute::default();
static mut punct_attribute: kobj_attribute = kobj_attribute::default();
static mut rate_attribute: kobj_attribute = kobj_attribute::default();
static mut tone_attribute: kobj_attribute = kobj_attribute::default();
static mut vol_attribute: kobj_attribute = kobj_attribute::default();
static mut delay_time_attribute: kobj_attribute = kobj_attribute::default();
static mut direct_attribute: kobj_attribute = kobj_attribute::default();
static mut full_time_attribute: kobj_attribute = kobj_attribute::default();
static mut jiffy_delta_attribute: kobj_attribute = kobj_attribute::default();
static mut trigger_time_attribute: kobj_attribute = kobj_attribute::default();

/* Create a group of attributes so that we can create and destroy them all
 * at once. */
static mut synth_attrs: [*mut attribute; 14] = [
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(),
];

unsafe extern "C" {
    static spk_ttyio_ops: spk_io_ops;
}

unsafe fn read_buff_add(c: u8) {
    pr_info(b"speakup_dummy: got character %02x\n\0".as_ptr(), c as u32);
}

// static struct spk_synth synth_dummy = { ... };
static mut synth_dummy: spk_synth = spk_synth::default();

// Kernel module parameters and metadata from the original implementation.
// module_param_named(ser, synth_dummy.ser, int, 0444);
// module_param_named(dev, synth_dummy.dev_name, charp, 0444);
// module_param_named(start, synth_dummy.startup, short, 0444);
// module_param_named(rate, vars[RATE_ID].u.n.default_val, int, 0444);
// module_param_named(pitch, vars[PITCH_ID].u.n.default_val, int, 0444);
// module_param_named(inflection, vars[INFLECTION_ID].u.n.default_val, int, 0444);
// module_param_named(vol, vars[VOL_ID].u.n.default_val, int, 0444);
// module_param_named(tone, vars[TONE_ID].u.n.default_val, int, 0444);
// module_param_named(punct, vars[PUNCT_ID].u.n.default_val, int, 0444);
// module_param_named(direct, vars[DIRECT_ID].u.n.default_val, int, 0444);

// MODULE_PARM_DESC(ser, "Set the serial port for the synthesizer (0-based).");
// MODULE_PARM_DESC(dev, "Set the device e.g. ttyUSB0, for the synthesizer.");
// MODULE_PARM_DESC(start, "Start the synthesizer once it is loaded.");
// MODULE_PARM_DESC(rate, "Set the rate variable on load.");
// MODULE_PARM_DESC(pitch, "Set the pitch variable on load.");
// MODULE_PARM_DESC(inflection, "Set the inflection variable on load.");
// MODULE_PARM_DESC(vol, "Set the vol variable on load.");
// MODULE_PARM_DESC(tone, "Set the tone variable on load.");
// MODULE_PARM_DESC(punct, "Set the punct variable on load.");
// MODULE_PARM_DESC(direct, "Set the direct variable on load.");

// module_spk_synth(synth_dummy);
// MODULE_AUTHOR("Samuel Thibault <samuel.thibault@ens-lyon.org>");
// MODULE_DESCRIPTION("Speakup support for text console");
// MODULE_LICENSE("GPL");
// MODULE_VERSION(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
