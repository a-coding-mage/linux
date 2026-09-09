// SPDX-License-Identifier: GPL-2.0+
/*
 * originally written by: Kirk Reiser <kirk@braille.uwo.ca>
 * this version considerably modified by David Borowski, david575@rogers.com
 *
 * Copyright (C) 1998-99  Kirk Reiser.
 * Copyright (C) 2003 David Borowski.
 *
 * specifically written as a driver for the speakup screenreview
 * s not a general device driver.
 */
// Dependencies supplied by the surrounding Speakup/kernel translation.
use crate::*;

const DRV_VERSION: &str = "2.11";
const PROCSPEECH: u8 = 0x0d;

static mut SYNTH_PROBE: Option<unsafe extern "C" fn(*mut spk_synth) -> i32> = None;

#[repr(C)]
#[derive(Copy, Clone)]
enum default_vars_id {
    CAPS_START_ID = 0,
    CAPS_STOP_ID,
    RATE_ID,
    PITCH_ID,
    VOL_ID,
    TONE_ID,
    PUNCT_ID,
    VOICE_ID,
    FREQUENCY_ID,
    DIRECT_ID,
    V_LAST_VAR_ID,
    NB_ID,
}

static mut vars: [var_t; default_vars_id::NB_ID as usize] = [
    var_t { var_id: CAPS_START, u: var_union { s: var_string { value: "\x01+35p" } } },
    var_t { var_id: CAPS_STOP, u: var_union { s: var_string { value: "\x01-35p" } } },
    var_t { var_id: RATE, u: var_union { n: var_num { pattern: "\x01%ds", default_val: 8, min: 0, max: 9, step: 0, value: 0, getter: core::ptr::null_mut() } } },
    var_t { var_id: PITCH, u: var_union { n: var_num { pattern: "\x01%dp", default_val: 50, min: 0, max: 99, step: 0, value: 0, getter: core::ptr::null_mut() } } },
    var_t { var_id: VOL, u: var_union { n: var_num { pattern: "\x01%dv", default_val: 5, min: 0, max: 9, step: 0, value: 0, getter: core::ptr::null_mut() } } },
    var_t { var_id: TONE, u: var_union { n: var_num { pattern: "\x01%dx", default_val: 1, min: 0, max: 2, step: 0, value: 0, getter: core::ptr::null_mut() } } },
    var_t { var_id: PUNCT, u: var_union { n: var_num { pattern: "\x01%db", default_val: 7, min: 0, max: 15, step: 0, value: 0, getter: core::ptr::null_mut() } } },
    var_t { var_id: VOICE, u: var_union { n: var_num { pattern: "\x01%do", default_val: 0, min: 0, max: 7, step: 0, value: 0, getter: core::ptr::null_mut() } } },
    var_t { var_id: FREQUENCY, u: var_union { n: var_num { pattern: "\x01%df", default_val: 5, min: 0, max: 9, step: 0, value: 0, getter: core::ptr::null_mut() } } },
    var_t { var_id: DIRECT, u: var_union { n: var_num { pattern: core::ptr::null(), default_val: 0, min: 0, max: 1, step: 0, value: 0, getter: core::ptr::null_mut() } } },
    var_t { var_id: V_LAST_VAR, u: var_union { n: var_num::default() } },
];

/* These attributes will appear in /sys/accessibility/speakup/ltlk. */
static mut caps_start_attribute: kobj_attribute = __ATTR!(caps_start, 0o644, spk_var_show, spk_var_store);
static mut caps_stop_attribute: kobj_attribute = __ATTR!(caps_stop, 0o644, spk_var_show, spk_var_store);
static mut freq_attribute: kobj_attribute = __ATTR!(freq, 0o644, spk_var_show, spk_var_store);
static mut pitch_attribute: kobj_attribute = __ATTR!(pitch, 0o644, spk_var_show, spk_var_store);
static mut punct_attribute: kobj_attribute = __ATTR!(punct, 0o644, spk_var_show, spk_var_store);
static mut rate_attribute: kobj_attribute = __ATTR!(rate, 0o644, spk_var_show, spk_var_store);
static mut tone_attribute: kobj_attribute = __ATTR!(tone, 0o644, spk_var_show, spk_var_store);
static mut voice_attribute: kobj_attribute = __ATTR!(voice, 0o644, spk_var_show, spk_var_store);
static mut vol_attribute: kobj_attribute = __ATTR!(vol, 0o644, spk_var_show, spk_var_store);
static mut delay_time_attribute: kobj_attribute = __ATTR!(delay_time, 0o644, spk_var_show, spk_var_store);
static mut direct_attribute: kobj_attribute = __ATTR!(direct, 0o644, spk_var_show, spk_var_store);
static mut full_time_attribute: kobj_attribute = __ATTR!(full_time, 0o644, spk_var_show, spk_var_store);
static mut jiffy_delta_attribute: kobj_attribute = __ATTR!(jiffy_delta, 0o644, spk_var_show, spk_var_store);
static mut trigger_time_attribute: kobj_attribute = __ATTR!(trigger_time, 0o644, spk_var_show, spk_var_store);

/* Create a group of attributes so that we can create and destroy them all at once. */
static mut synth_attrs: [*mut attribute; 15] = [
    unsafe { &mut caps_start_attribute.attr }, unsafe { &mut caps_stop_attribute.attr },
    unsafe { &mut freq_attribute.attr }, unsafe { &mut pitch_attribute.attr },
    unsafe { &mut punct_attribute.attr }, unsafe { &mut rate_attribute.attr },
    unsafe { &mut tone_attribute.attr }, unsafe { &mut voice_attribute.attr },
    unsafe { &mut vol_attribute.attr }, unsafe { &mut delay_time_attribute.attr },
    unsafe { &mut direct_attribute.attr }, unsafe { &mut full_time_attribute.attr },
    unsafe { &mut jiffy_delta_attribute.attr }, unsafe { &mut trigger_time_attribute.attr },
    core::ptr::null_mut(),
];

static mut synth_ltlk: spk_synth = spk_synth {
    name: "ltlk", version: DRV_VERSION, long_name: "LiteTalk", init: "\x01@\x011y\n\0",
    procspeech: PROCSPEECH, clear: SYNTH_CLEAR, delay: 500, trigger: 50, jiffies: 50,
    full: 40000, dev_name: SYNTH_DEFAULT_DEV, startup: SYNTH_START, checkval: SYNTH_CHECK,
    vars: unsafe { &mut vars }, io_ops: &spk_ttyio_ops, probe: Some(synth_probe),
    release: spk_ttyio_release, synth_immediate: spk_ttyio_synth_immediate,
    catch_up: spk_do_catch_up, flush: spk_synth_flush, is_alive: spk_synth_is_alive_restart,
    synth_adjust: None, read_buff_add: None, get_index: spk_synth_get_index,
    indexing: indexing_t { command: "\x01%di", lowindex: 1, highindex: 5, currindex: 1 },
    attributes: attribute_group { attrs: unsafe { synth_attrs.as_mut_ptr() }, name: "ltlk" },
};

/* interrogate the LiteTalk and print its settings */
unsafe fn synth_interrogate(synth: *mut spk_synth) {
    let mut buf = [0u8; 50];
    let mut rom_v = [0u8; 20];
    ((*(*synth).synth_immediate.unwrap())(synth, "\x18\x01?"));
    let mut i = 0usize;
    while i < 50 {
        buf[i] = ((*(*(*synth).io_ops).synth_in.unwrap())(synth));
        if i > 2 && buf[i] == 0x7f { break; }
        i += 1;
    }
    let mut t = 2usize;
    i = 0;
    while buf[t] != b'\r' {
        rom_v[i] = buf[t];
        i += 1;
        t += 1;
        if i >= 19 { break; }
    }
    rom_v[i] = 0;
    pr_info!("%s: ROM version: %s\n", (*synth).long_name, rom_v.as_ptr());
}

unsafe extern "C" fn synth_probe(synth: *mut spk_synth) -> i32 {
    let failed = spk_ttyio_synth_probe(synth);
    if failed == 0 { synth_interrogate(synth); }
    (*synth).alive = failed == 0;
    failed
}

module_param_named!(ser, synth_ltlk.ser, int, 0o444);
module_param_named!(dev, synth_ltlk.dev_name, charp, 0o444);
module_param_named!(start, synth_ltlk.startup, short, 0o444);
module_param_named!(rate, vars[default_vars_id::RATE_ID as usize].u.n.default_val, int, 0o444);
module_param_named!(pitch, vars[default_vars_id::PITCH_ID as usize].u.n.default_val, int, 0o444);
module_param_named!(vol, vars[default_vars_id::VOL_ID as usize].u.n.default_val, int, 0o444);
module_param_named!(tone, vars[default_vars_id::TONE_ID as usize].u.n.default_val, int, 0o444);
module_param_named!(punct, vars[default_vars_id::PUNCT_ID as usize].u.n.default_val, int, 0o444);
module_param_named!(voice, vars[default_vars_id::VOICE_ID as usize].u.n.default_val, int, 0o444);
module_param_named!(frequency, vars[default_vars_id::FREQUENCY_ID as usize].u.n.default_val, int, 0o444);
module_param_named!(direct, vars[default_vars_id::DIRECT_ID as usize].u.n.default_val, int, 0o444);

MODULE_PARM_DESC!(ser, "Set the serial port for the synthesizer (0-based).");
MODULE_PARM_DESC!(dev, "Set the device e.g. ttyUSB0, for the synthesizer.");
MODULE_PARM_DESC!(start, "Start the synthesizer once it is loaded.");
MODULE_PARM_DESC!(rate, "Set the rate variable on load.");
MODULE_PARM_DESC!(pitch, "Set the pitch variable on load.");
MODULE_PARM_DESC!(vol, "Set the vol variable on load.");
MODULE_PARM_DESC!(tone, "Set the tone variable on load.");
MODULE_PARM_DESC!(punct, "Set the punct variable on load.");
MODULE_PARM_DESC!(voice, "Set the voice variable on load.");
MODULE_PARM_DESC!(frequency, "Set the frequency variable on load.");
MODULE_PARM_DESC!(direct, "Set the direct variable on load.");

module_spk_synth!(synth_ltlk);
MODULE_AUTHOR!("Kirk Reiser <kirk@braille.uwo.ca>");
MODULE_AUTHOR!("David Borowski");
MODULE_DESCRIPTION!("Speakup support for DoubleTalk LT/LiteTalk synthesizers");
MODULE_LICENSE!("GPL");
MODULE_VERSION!(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
