// SPDX-License-Identifier: GPL-2.0+
/*
 * originally written by: Kirk Reiser <kirk@braille.uwo.ca>
 * this version considerably modified by David Borowski, david575@rogers.com
 *
 * Copyright (C) 1998-99  Kirk Reiser.
 * Copyright (C) 2003 David Borowski.
 *
 * this code is specifically written as a driver for the speakup screenreview
 * package and is not a general device driver.
 */

// C dependencies: "spk_priv.h" and "speakup.h".

const DRV_VERSION: &str = "2.11";
const SYNTH_CLEAR: u8 = 0x18;
const PROCSPEECH: u8 = b'\r';

#[repr(C)]
#[derive(Copy, Clone)]
enum DefaultVarsId {
    CAPS_START_ID = 0,
    CAPS_STOP_ID,
    RATE_ID,
    PITCH_ID,
    VOL_ID,
    TONE_ID,
    DIRECT_ID,
    V_LAST_VAR_ID,
    NB_ID,
}

// These attributes will appear in /sys/accessibility/speakup/bns.
// The concrete types and helper macros are supplied by the surrounding kernel
// translation.
static mut vars: [var_t; NB_ID as usize] = [
    var_t { id: CAPS_START, u: var_union { s: var_string { value: "\x05\x31\x32P" } } },
    var_t { id: CAPS_STOP, u: var_union { s: var_string { value: "\x05\x38P" } } },
    var_t { id: RATE, u: var_union { n: var_num { name: "\x05%dE", default_val: 8, low: 1, high: 16, step: 0, mode: 0, reserved: core::ptr::null_mut() } } },
    var_t { id: PITCH, u: var_union { n: var_num { name: "\x05%dP", default_val: 8, low: 0, high: 16, step: 0, mode: 0, reserved: core::ptr::null_mut() } } },
    var_t { id: VOL, u: var_union { n: var_num { name: "\x05%dV", default_val: 8, low: 0, high: 16, step: 0, mode: 0, reserved: core::ptr::null_mut() } } },
    var_t { id: TONE, u: var_union { n: var_num { name: "\x05%dT", default_val: 8, low: 0, high: 16, step: 0, mode: 0, reserved: core::ptr::null_mut() } } },
    var_t { id: DIRECT, u: var_union { n: var_num { name: "", default_val: 0, low: 0, high: 1, step: 0, mode: 0, reserved: core::ptr::null_mut() } } },
];

static mut caps_start_attribute: kobj_attribute = __ATTR!(caps_start, 0o644, spk_var_show, spk_var_store);
static mut caps_stop_attribute: kobj_attribute = __ATTR!(caps_stop, 0o644, spk_var_show, spk_var_store);
static mut pitch_attribute: kobj_attribute = __ATTR!(pitch, 0o644, spk_var_show, spk_var_store);
static mut rate_attribute: kobj_attribute = __ATTR!(rate, 0o644, spk_var_show, spk_var_store);
static mut tone_attribute: kobj_attribute = __ATTR!(tone, 0o644, spk_var_show, spk_var_store);
static mut vol_attribute: kobj_attribute = __ATTR!(vol, 0o644, spk_var_show, spk_var_store);
static mut delay_time_attribute: kobj_attribute = __ATTR!(delay_time, 0o644, spk_var_show, spk_var_store);
static mut direct_attribute: kobj_attribute = __ATTR!(direct, 0o644, spk_var_show, spk_var_store);
static mut full_time_attribute: kobj_attribute = __ATTR!(full_time, 0o644, spk_var_show, spk_var_store);
static mut jiffy_delta_attribute: kobj_attribute = __ATTR!(jiffy_delta, 0o644, spk_var_show, spk_var_store);
static mut trigger_time_attribute: kobj_attribute = __ATTR!(trigger_time, 0o644, spk_var_show, spk_var_store);

// Create a group of attributes so that we can create and destroy them all at once.
static mut synth_attrs: [*mut attribute; 12] = [
    &mut caps_start_attribute.attr,
    &mut caps_stop_attribute.attr,
    &mut pitch_attribute.attr,
    &mut rate_attribute.attr,
    &mut tone_attribute.attr,
    &mut vol_attribute.attr,
    &mut delay_time_attribute.attr,
    &mut direct_attribute.attr,
    &mut full_time_attribute.attr,
    &mut jiffy_delta_attribute.attr,
    &mut trigger_time_attribute.attr,
    core::ptr::null_mut(),
];

static mut synth_bns: spk_synth = spk_synth {
    name: "bns",
    version: DRV_VERSION,
    long_name: "Braille 'N Speak",
    init: "\x05Z\x05\x43",
    procspeech: PROCSPEECH,
    clear: SYNTH_CLEAR,
    delay: 500,
    trigger: 50,
    jiffies: 50,
    full: 40000,
    dev_name: SYNTH_DEFAULT_DEV,
    startup: SYNTH_START,
    checkval: SYNTH_CHECK,
    vars: vars.as_mut_ptr(),
    io_ops: &spk_ttyio_ops,
    probe: spk_ttyio_synth_probe,
    release: spk_ttyio_release,
    synth_immediate: spk_ttyio_synth_immediate,
    catch_up: spk_do_catch_up,
    flush: spk_synth_flush,
    is_alive: spk_synth_is_alive_restart,
    synth_adjust: None,
    read_buff_add: None,
    get_index: None,
    indexing: indexing_t { command: None, lowindex: 0, highindex: 0, currindex: 0 },
    attributes: synth_attributes { attrs: synth_attrs.as_mut_ptr(), name: "bns" },
};

// C module parameters and registration metadata.
module_param_named!(ser, synth_bns.ser, int, 0o444);
module_param_named!(dev, synth_bns.dev_name, charp, 0o444);
module_param_named!(start, synth_bns.startup, short, 0o444);
module_param_named!(rate, vars[RATE_ID as usize].u.n.default_val, int, 0o444);
module_param_named!(pitch, vars[PITCH_ID as usize].u.n.default_val, int, 0o444);
module_param_named!(vol, vars[VOL_ID as usize].u.n.default_val, int, 0o444);
module_param_named!(tone, vars[TONE_ID as usize].u.n.default_val, int, 0o444);
module_param_named!(direct, vars[DIRECT_ID as usize].u.n.default_val, int, 0o444);

MODULE_PARM_DESC!(ser, "Set the serial port for the synthesizer (0-based).");
MODULE_PARM_DESC!(dev, "Set the device e.g. ttyUSB0, for the synthesizer.");
MODULE_PARM_DESC!(start, "Start the synthesizer once it is loaded.");
MODULE_PARM_DESC!(rate, "Set the rate variable on load.");
MODULE_PARM_DESC!(pitch, "Set the pitch variable on load.");
MODULE_PARM_DESC!(vol, "Set the vol variable on load.");
MODULE_PARM_DESC!(tone, "Set the tone variable on load.");
MODULE_PARM_DESC!(direct, "Set the direct variable on load.");

module_spk_synth!(synth_bns);

MODULE_AUTHOR!("Kirk Reiser <kirk@braille.uwo.ca>");
MODULE_AUTHOR!("David Borowski");
MODULE_DESCRIPTION!("Speakup support for Braille 'n Speak synthesizers");
MODULE_LICENSE!("GPL");
MODULE_VERSION!(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
