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

// Dependencies supplied by spk_priv.h and speakup.h remain external.

const DRV_VERSION: &str = "2.11";
const SYNTH_CLEAR: u8 = 0x18;
const PROCSPEECH: u8 = b'\r'; /* process speech char */

#[repr(usize)]
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

/* These initializers use the var_t, kobj_attribute, and spk_synth definitions
 * supplied by the surrounding Speakup implementation. */
static mut vars: [var_t; NB_ID as usize] = [
    var_t { id: CAPS_START, u: var_data::s("\x05P8") },
    var_t { id: CAPS_STOP, u: var_data::s("\x05P5") },
    var_t { id: RATE, u: var_data::n("\x05R%d", 5, 0, 9, 0, 0, core::ptr::null_mut()) },
    var_t { id: PITCH, u: var_data::n("\x05P%d", 5, 0, 9, 0, 0, core::ptr::null_mut()) },
    var_t { id: VOL, u: var_data::n("\x05V%d", 5, 0, 9, 0, 0, core::ptr::null_mut()) },
    var_t { id: TONE, u: var_data::n("\x05T%c", 12, 0, 25, 61, 0, core::ptr::null_mut()) },
    var_t { id: DIRECT, u: var_data::n(core::ptr::null(), 0, 0, 1, 0, 0, core::ptr::null_mut()) },
    var_t::last(),
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

/* Create a group of attributes so that we can create and destroy them all at once. */
static mut synth_attrs: [*mut attribute; 12] = [
    &mut caps_start_attribute.attr, &mut caps_stop_attribute.attr,
    &mut pitch_attribute.attr, &mut rate_attribute.attr,
    &mut tone_attribute.attr, &mut vol_attribute.attr,
    &mut delay_time_attribute.attr, &mut direct_attribute.attr,
    &mut full_time_attribute.attr, &mut jiffy_delta_attribute.attr,
    &mut trigger_time_attribute.attr, core::ptr::null_mut(),
];

static mut synth_txprt: spk_synth = spk_synth {
    name: "txprt", version: DRV_VERSION, long_name: "Transport",
    init: "\x05N1", procspeech: PROCSPEECH, clear: SYNTH_CLEAR,
    delay: 500, trigger: 50, jiffies: 50, full: 40000,
    dev_name: SYNTH_DEFAULT_DEV, startup: SYNTH_START, checkval: SYNTH_CHECK,
    vars: vars.as_ptr(), io_ops: &spk_ttyio_ops,
    probe: Some(spk_ttyio_synth_probe), release: Some(spk_ttyio_release),
    synth_immediate: Some(spk_ttyio_synth_immediate), catch_up: Some(spk_do_catch_up),
    flush: Some(spk_synth_flush), is_alive: Some(spk_synth_is_alive_restart),
    synth_adjust: None, read_buff_add: None, get_index: None, indexing: indexing {
        command: None, lowindex: 0, highindex: 0, currindex: 0,
    }, attributes: attribute_group { attrs: synth_attrs.as_mut_ptr(), name: "txprt" },
};

module_param_named!(ser, synth_txprt.ser, int, 0o444);
module_param_named!(dev, synth_txprt.dev_name, charp, 0o444);
module_param_named!(start, synth_txprt.startup, short, 0o444);
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

module_spk_synth!(synth_txprt);
MODULE_AUTHOR!("Kirk Reiser <kirk@braille.uwo.ca>");
MODULE_AUTHOR!("David Borowski");
MODULE_DESCRIPTION!("Speakup support for Transport synthesizers");
MODULE_LICENSE!("GPL");
MODULE_VERSION!(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
