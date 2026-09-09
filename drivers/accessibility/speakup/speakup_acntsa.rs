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

// Dependencies supplied by spk_priv.h, speakup.h, and speakup_acnt.h.

const DRV_VERSION: &str = "2.11";
const PROCSPEECH: u8 = b'\r';

extern "C" {
    fn spk_ttyio_synth_probe(synth: *mut spk_synth) -> i32;
    fn mdelay(msecs: u32);
}

#[repr(C)]
enum default_vars_id {
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

// C declarations/types and constants are supplied by the included headers.
static mut vars: [var_t; NB_ID as usize] = [
    var_t { var_id: CAPS_START, u: var_u { s: var_string { value: "\x1bP8" } } },
    var_t { var_id: CAPS_STOP, u: var_u { s: var_string { value: "\x1bP5" } } },
    var_t { var_id: RATE, u: var_u { n: var_num { pattern: "\x1bR%c", default_val: 9, min: 0, max: 17, step: 0, offset: 0, values: "0123456789abcdefgh" } } },
    var_t { var_id: PITCH, u: var_u { n: var_num { pattern: "\x1bP%d", default_val: 5, min: 0, max: 9, step: 0, offset: 0, values: core::ptr::null() } } },
    var_t { var_id: VOL, u: var_u { n: var_num { pattern: "\x1bA%d", default_val: 9, min: 0, max: 9, step: 0, offset: 0, values: core::ptr::null() } } },
    var_t { var_id: TONE, u: var_u { n: var_num { pattern: "\x1bV%d", default_val: 5, min: 0, max: 9, step: 0, offset: 0, values: core::ptr::null() } } },
    var_t { var_id: DIRECT, u: var_u { n: var_num { pattern: core::ptr::null(), default_val: 0, min: 0, max: 1, step: 0, offset: 0, values: core::ptr::null() } } },
    V_LAST_VAR,
];

// These attributes appear in /sys/accessibility/speakup/acntsa.
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

static mut synth_attrs: [*mut attribute; 12] = [
    unsafe { &mut caps_start_attribute.attr }, unsafe { &mut caps_stop_attribute.attr },
    unsafe { &mut pitch_attribute.attr }, unsafe { &mut rate_attribute.attr },
    unsafe { &mut tone_attribute.attr }, unsafe { &mut vol_attribute.attr },
    unsafe { &mut delay_time_attribute.attr }, unsafe { &mut direct_attribute.attr },
    unsafe { &mut full_time_attribute.attr }, unsafe { &mut jiffy_delta_attribute.attr },
    unsafe { &mut trigger_time_attribute.attr }, core::ptr::null_mut(),
];

static mut synth_acntsa: spk_synth = spk_synth {
    name: "acntsa", version: DRV_VERSION, long_name: "Accent-SA",
    init: "\x1bT2\x1b=M\x1bOi\x1bN1\n", procspeech: PROCSPEECH,
    clear: SYNTH_CLEAR, delay: 400, trigger: 50, jiffies: 30, full: 40000,
    dev_name: SYNTH_DEFAULT_DEV, startup: SYNTH_START, checkval: SYNTH_CHECK,
    vars: vars.as_ptr(), io_ops: &spk_ttyio_ops, probe: Some(synth_probe),
    release: Some(spk_ttyio_release), synth_immediate: Some(spk_ttyio_synth_immediate),
    catch_up: Some(spk_do_catch_up), flush: Some(spk_synth_flush),
    is_alive: Some(spk_synth_is_alive_restart), synth_adjust: None,
    read_buff_add: None, get_index: None,
    indexing: spk_indexing { command: None, lowindex: 0, highindex: 0, currindex: 0 },
    attributes: spk_attributes { attrs: synth_attrs.as_mut_ptr(), name: "acntsa" },
};

unsafe extern "C" fn synth_probe(synth: *mut spk_synth) -> i32 {
    let failed = spk_ttyio_synth_probe(synth);
    if failed == 0 {
        ((*synth).synth_immediate.unwrap())(synth, "\x1b=R\r");
        mdelay(100);
    }
    (*synth).alive = failed == 0;
    failed
}

// module_param_named and MODULE_* declarations are retained as build metadata.
module_param_named!(ser, synth_acntsa.ser, int, 0o444);
module_param_named!(dev, synth_acntsa.dev_name, charp, 0o444);
module_param_named!(start, synth_acntsa.startup, short, 0o444);
module_param_named!(rate, vars[RATE_ID as usize].u.n.default_val, int, 0o444);
module_param_named!(pitch, vars[PITCH_ID as usize].u.n.default_val, int, 0o444);
module_param_named!(vol, vars[VOL_ID as usize].u.n.default_val, int, 0o444);
module_param_named!(tone, vars[TONE_ID as usize].u.n.default_val, int, 0o444);
module_param_named!(direct, vars[DIRECT_ID as usize].u.n.default_val, int, 0o444);

module_spk_synth!(synth_acntsa);
module_author!("Kirk Reiser <kirk@braille.uwo.ca>");
module_author!("David Borowski");
module_description!("Speakup support for Accent SA synthesizer");
module_license!("GPL");
module_version!(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
