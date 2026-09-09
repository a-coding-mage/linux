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

const DRV_VERSION: *const u8 = b"2.11\0".as_ptr();
const SYNTH_CLEAR: u8 = 0x18;
const PROCSPEECH: u8 = b'\r';

extern "C" {
    fn spk_var_show();
    fn spk_var_store();
}

#[repr(C)]
#[derive(Copy, Clone)]
enum DefaultVarsId {
    CAPS_START_ID = 0,
    CAPS_STOP_ID,
    RATE_ID,
    PITCH_ID,
    VOL_ID,
    TONE_ID,
    PUNCT_ID,
    DIRECT_ID,
    V_LAST_VAR_ID,
    NB_ID,
}

// The following declarations retain the kernel attribute objects and their
// __ATTR(caps..., 0644, spk_var_show, spk_var_store) initializers.
static mut caps_start_attribute: kobj_attribute = __ATTR(caps_start, 0o644, spk_var_show, spk_var_store);
static mut caps_stop_attribute: kobj_attribute = __ATTR(caps_stop, 0o644, spk_var_show, spk_var_store);
static mut pitch_attribute: kobj_attribute = __ATTR(pitch, 0o644, spk_var_show, spk_var_store);
static mut punct_attribute: kobj_attribute = __ATTR(punct, 0o644, spk_var_show, spk_var_store);
static mut rate_attribute: kobj_attribute = __ATTR(rate, 0o644, spk_var_show, spk_var_store);
static mut tone_attribute: kobj_attribute = __ATTR(tone, 0o644, spk_var_show, spk_var_store);
static mut vol_attribute: kobj_attribute = __ATTR(vol, 0o644, spk_var_show, spk_var_store);
static mut delay_time_attribute: kobj_attribute = __ATTR(delay_time, 0o644, spk_var_show, spk_var_store);
static mut direct_attribute: kobj_attribute = __ATTR(direct, 0o644, spk_var_show, spk_var_store);
static mut full_time_attribute: kobj_attribute = __ATTR(full_time, 0o644, spk_var_show, spk_var_store);
static mut jiffy_delta_attribute: kobj_attribute = __ATTR(jiffy_delta, 0o644, spk_var_show, spk_var_store);
static mut trigger_time_attribute: kobj_attribute = __ATTR(trigger_time, 0o644, spk_var_show, spk_var_store);

static mut synth_attrs: [*mut attribute; 13] = [
    unsafe { &mut caps_start_attribute.attr }, unsafe { &mut caps_stop_attribute.attr },
    unsafe { &mut pitch_attribute.attr }, unsafe { &mut punct_attribute.attr },
    unsafe { &mut rate_attribute.attr }, unsafe { &mut tone_attribute.attr },
    unsafe { &mut vol_attribute.attr }, unsafe { &mut delay_time_attribute.attr },
    unsafe { &mut direct_attribute.attr }, unsafe { &mut full_time_attribute.attr },
    unsafe { &mut jiffy_delta_attribute.attr }, unsafe { &mut trigger_time_attribute.attr },
    core::ptr::null_mut(), // need to NULL terminate the list of attributes
];

static mut vars: [var_t; NB_ID as usize] = [
    var_t::caps_start("\x05P+"), var_t::caps_stop("\x05P-"),
    var_t::number("\x05R%d", 7, 0, 9, 0, 0, core::ptr::null()),
    var_t::number("\x05P%d", 3, 0, 9, 0, 0, core::ptr::null()),
    var_t::number("\x05V%d", 9, 0, 9, 0, 0, core::ptr::null()),
    var_t::number("\x05T%c", 8, 0, 25, 65, 0, core::ptr::null()),
    var_t::number("\x05M%c", 0, 0, 3, 0, 0, "nsma"),
    var_t::number_null(core::ptr::null(), 0, 0, 1, 0, 0, core::ptr::null()),
    var_t::last(),
];

static mut synth_spkout: spk_synth = spk_synth {
    name: "spkout", version: DRV_VERSION, long_name: "Speakout",
    init: "\x05W1\x05I2\x05C3", procspeech: PROCSPEECH, clear: SYNTH_CLEAR,
    delay: 500, trigger: 50, jiffies: 50, full: 40000, dev_name: SYNTH_DEFAULT_DEV,
    startup: SYNTH_START, checkval: SYNTH_CHECK, vars: unsafe { vars.as_mut_ptr() },
    io_ops: &spk_ttyio_ops, probe: spk_ttyio_synth_probe, release: spk_ttyio_release,
    synth_immediate: spk_ttyio_synth_immediate, catch_up: spk_do_catch_up,
    flush: Some(synth_flush), is_alive: spk_synth_is_alive_restart,
    synth_adjust: None, read_buff_add: None, get_index: spk_synth_get_index,
    indexing: synth_indexing { command: "\x05[%c", lowindex: 1, highindex: 5, currindex: 1 },
    attributes: attribute_group { attrs: unsafe { synth_attrs.as_mut_ptr() }, name: "spkout" },
};

unsafe fn synth_flush(synth: *mut spk_synth) {
    ((*(*synth).io_ops).flush_buffer)(synth);
    ((*(*synth).io_ops).send_xchar)(synth, SYNTH_CLEAR);
}

// Kernel module parameters, descriptions, registration, author, description,
// license, and version retain the corresponding module_* declarations here.
module_param_named!(ser, synth_spkout.ser, int, 0o444);
module_param_named!(dev, synth_spkout.dev_name, charp, 0o444);
module_param_named!(start, synth_spkout.startup, short, 0o444);
module_param_named!(rate, vars[RateId].u.n.default_val, int, 0o444);
module_param_named!(vol, vars[PitchId].u.n.default_val, int, 0o444);
module_param_named!(tone, vars[ToneId].u.n.default_val, int, 0o444);
module_param_named!(punct, vars[PunctId].u.n.default_val, int, 0o444);
module_param_named!(direct, vars[DirectId].u.n.default_val, int, 0o444);
module_spk_synth!(synth_spkout);
MODULE_PARM_DESC!(ser, "Set the serial port for the synthesizer (0-based).");
MODULE_PARM_DESC!(dev, "Set the device e.g. ttyUSB0, for the synthesizer.");
MODULE_PARM_DESC!(start, "Start the synthesizer once it is loaded.");
MODULE_PARM_DESC!(rate, "Set the rate variable on load.");
MODULE_PARM_DESC!(vol, "Set the vol variable on load.");
MODULE_PARM_DESC!(tone, "Set the tone variable on load.");
MODULE_PARM_DESC!(punct, "Set the punct variable on load.");
MODULE_PARM_DESC!(direct, "Set the direct variable on load.");
MODULE_AUTHOR!("Kirk Reiser <kirk@braille.uwo.ca>");
MODULE_AUTHOR!("David Borowski");
MODULE_DESCRIPTION!("Speakup support for Speak Out synthesizers");
MODULE_LICENSE!("GPL");
MODULE_VERSION!(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
