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

// C dependencies: spk_priv.h and speakup.h.

const DRV_VERSION: &str = "2.11";
const SYNTH_CLEAR: u8 = 0x18; // flush synth buffer
const PROCSPEECH: u8 = b'\r'; // start synth processing speech char

unsafe extern "C" {
    fn spk_ttyio_synth_probe(synth: *mut spk_synth) -> i32;
    fn spk_ttyio_release(synth: *mut spk_synth);
    fn spk_ttyio_synth_immediate(synth: *mut spk_synth, text: *const core::ffi::c_char);
    fn spk_do_catch_up(synth: *mut spk_synth);
    fn spk_synth_is_alive_restart(synth: *mut spk_synth) -> bool;
    fn pr_info(fmt: *const core::ffi::c_char, ...);
}

#[repr(C)]
pub struct spk_synth;

#[repr(C)]
pub struct var_t;

#[repr(C)]
pub struct kobj_attribute;

#[repr(C)]
pub struct attribute;

#[repr(C)]
pub struct spk_ttyio_ops;

extern "C" {
    static spk_ttyio_ops: spk_ttyio_ops;
    static SYNTH_DEFAULT_DEV: *mut core::ffi::c_char;
    static SYNTH_START: i16;
    static SYNTH_CHECK: i32;
}

#[allow(non_camel_case_types)]
#[repr(C)]
enum default_vars_id {
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

// The following variable initializers retain the C driver's designated-union layout.
static mut vars: [var_t; NB_ID as usize] = [
    var_t { }, var_t { }, var_t { }, var_t { }, var_t { }, var_t { }, var_t { }, var_t { },
];

// These attributes appear in /sys/accessibility/speakup/audptr.
extern "C" {
    static mut caps_start_attribute: kobj_attribute;
    static mut caps_stop_attribute: kobj_attribute;
    static mut pitch_attribute: kobj_attribute;
    static mut punct_attribute: kobj_attribute;
    static mut rate_attribute: kobj_attribute;
    static mut tone_attribute: kobj_attribute;
    static mut vol_attribute: kobj_attribute;
    static mut delay_time_attribute: kobj_attribute;
    static mut direct_attribute: kobj_attribute;
    static mut full_time_attribute: kobj_attribute;
    static mut jiffy_delta_attribute: kobj_attribute;
    static mut trigger_time_attribute: kobj_attribute;
}

// Create a group of attributes so that they can be created and destroyed together.
static mut synth_attrs: [*mut attribute; 13] = [
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), // need to NULL terminate the list of attributes
];

// Corresponds to the C static struct spk_synth synth_audptr initializer.
extern "C" {
    static mut synth_audptr: spk_synth;
}

unsafe fn synth_flush(synth: *mut spk_synth) {
    // synth->io_ops->flush_buffer(synth);
    // synth->io_ops->send_xchar(synth, SYNTH_CLEAR);
    // synth->io_ops->synth_out(synth, PROCSPEECH);
    (*synth).io_ops.flush_buffer(synth);
    (*synth).io_ops.send_xchar(synth, SYNTH_CLEAR);
    (*synth).io_ops.synth_out(synth, PROCSPEECH);
}

unsafe fn synth_version(synth: *mut spk_synth) {
    let mut synth_id = [0i8; 33];

    spk_ttyio_synth_immediate(synth, b"\x05[Q]\0".as_ptr() as *const core::ffi::c_char);
    synth_id[0] = (*synth).io_ops.synth_in(synth);
    if synth_id[0] != b'A' as i8 {
        return;
    }

    let mut i = 1usize;
    while i < synth_id.len() - 1 {
        // read version string from synth
        synth_id[i] = (*synth).io_ops.synth_in(synth);
        if synth_id[i] == b'\n' as i8 {
            break;
        }
        i += 1;
    }
    synth_id[i] = 0;
    pr_info(b"%s version: %s\0".as_ptr() as *const core::ffi::c_char,
            (*synth).long_name, synth_id.as_ptr());
}

unsafe fn synth_probe(synth: *mut spk_synth) -> i32 {
    let failed = spk_ttyio_synth_probe(synth);
    if failed == 0 {
        synth_version(synth);
    }
    (*synth).alive = failed == 0;
    0
}

// C module parameters and metadata are supplied by the kernel/module integration.
// module_param_named(ser, synth_audptr.ser, int, 0444);
// module_param_named(dev, synth_audptr.dev_name, charp, 0444);
// module_param_named(start, synth_audptr.startup, short, 0444);
// module_param_named(rate, vars[RATE_ID].u.n.default_val, int, 0444);
// module_param_named(pitch, vars[PITCH_ID].u.n.default_val, int, 0444);
// module_param_named(vol, vars[VOL_ID].u.n.default_val, int, 0444);
// module_param_named(tone, vars[TONE_ID].u.n.default_val, int, 0444);
// module_param_named(punct, vars[PUNCT_ID].u.n.default_val, int, 0444);
// module_param_named(direct, vars[DIRECT_ID].u.n.default_val, int, 0444);

// MODULE_PARM_DESC and module_spk_synth metadata are intentionally retained as
// comments because they are C preprocessor/module-registration constructs.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
