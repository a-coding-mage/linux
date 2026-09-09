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

// The declarations below are supplied by the surrounding Speakup/Linux
// compatibility layer.

const DRV_VERSION: &str = "2.14";
const SYNTH_CLEAR: u8 = 0x03;
const PROCSPEECH: u8 = 0x0b;

static mut LAST_CHAR: u8 = 0;

unsafe fn read_buff_add(ch: u8) {
    LAST_CHAR = ch;
}

#[inline]
unsafe fn synth_full() -> bool {
    LAST_CHAR == 0x13
}

unsafe extern "C" {
    fn do_catch_up(synth: *mut spk_synth);
    fn synth_flush(synth: *mut spk_synth);
}

static mut IN_ESCAPE: i32 = 0;

#[repr(C)]
enum DefaultVarsId {
    CAPS_START_ID = 0,
    CAPS_STOP_ID,
    RATE_ID,
    PITCH_ID,
    INFLECTION_ID,
    VOL_ID,
    PUNCT_ID,
    VOICE_ID,
    DIRECT_ID,
    V_LAST_ID,
    NB_ID,
}

// These types, constants, and helper macros are defined by spk_priv.h and speakup.h.
static mut VARS: [var_t; DefaultVarsId::NB_ID as usize] = [var_t::default(); DefaultVarsId::NB_ID as usize];

/*
 * These attributes will appear in /sys/accessibility/speakup/decext.
 */
static mut CAPS_START_ATTRIBUTE: kobj_attribute = kobj_attribute::default();
static mut CAPS_STOP_ATTRIBUTE: kobj_attribute = kobj_attribute::default();
static mut PITCH_ATTRIBUTE: kobj_attribute = kobj_attribute::default();
static mut INFLECTION_ATTRIBUTE: kobj_attribute = kobj_attribute::default();
static mut PUNCT_ATTRIBUTE: kobj_attribute = kobj_attribute::default();
static mut RATE_ATTRIBUTE: kobj_attribute = kobj_attribute::default();
static mut VOICE_ATTRIBUTE: kobj_attribute = kobj_attribute::default();
static mut VOL_ATTRIBUTE: kobj_attribute = kobj_attribute::default();
static mut DELAY_TIME_ATTRIBUTE: kobj_attribute = kobj_attribute::default();
static mut DIRECT_ATTRIBUTE: kobj_attribute = kobj_attribute::default();
static mut FULL_TIME_ATTRIBUTE: kobj_attribute = kobj_attribute::default();
static mut JIFFY_DELTA_ATTRIBUTE: kobj_attribute = kobj_attribute::default();
static mut TRIGGER_TIME_ATTRIBUTE: kobj_attribute = kobj_attribute::default();

/* Create a group of attributes so that we can create and destroy them all at once. */
static mut SYNTH_ATTRS: [*mut attribute; 14] = [
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(),
];

static mut SYNTH_DECEXT: spk_synth = spk_synth::default();

unsafe fn do_catch_up_impl(synth: *mut spk_synth) {
    let mut ch: u8;
    static mut LAST: u8 = 0;
    let mut flags: c_ulong;
    let mut jiff_max: c_ulong;
    let jiffy_delta: *mut var_t = spk_get_var(JIFFY);
    let delay_time: *mut var_t = spk_get_var(DELAY);
    let mut jiffy_delta_val: i32 = 0;
    let mut delay_time_val: i32 = 0;

    spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags);
    jiffy_delta_val = (*jiffy_delta).u.n.value;
    spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
    jiff_max = jiffies().wrapping_add(jiffy_delta_val as c_ulong);

    while !kthread_should_stop() {
        spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags);
        if speakup_info.flushing != 0 {
            speakup_info.flushing = 0;
            spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
            ((*synth).flush)(synth);
            continue;
        }
        synth_buffer_skip_nonlatin1();
        if synth_buffer_empty() {
            spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
            break;
        }
        ch = synth_buffer_peek();
        set_current_state(TASK_INTERRUPTIBLE);
        delay_time_val = (*delay_time).u.n.value;
        spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
        if ch == b'\n' { ch = 0x0d; }
        if synth_full() || !((*(*synth).io_ops).synth_out)(synth, ch) {
            schedule_timeout(msecs_to_jiffies(delay_time_val));
            continue;
        }
        set_current_state(TASK_RUNNING);
        spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags);
        synth_buffer_getc();
        spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
        if ch == b'[' { IN_ESCAPE = 1; }
        else if ch == b']' { IN_ESCAPE = 0; }
        else if ch <= SPACE {
            if IN_ESCAPE == 0 && strchr(b",.!?;:", LAST) { ((*(*synth).io_ops).synth_out)(synth, PROCSPEECH); }
            if time_after_eq(jiffies(), jiff_max) {
                if IN_ESCAPE == 0 { ((*(*synth).io_ops).synth_out)(synth, PROCSPEECH); }
                spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags);
                jiffy_delta_val = (*jiffy_delta).u.n.value;
                delay_time_val = (*delay_time).u.n.value;
                spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
                schedule_timeout(msecs_to_jiffies(delay_time_val));
                jiff_max = jiffies().wrapping_add(jiffy_delta_val as c_ulong);
            }
        }
        LAST = ch;
    }
    if IN_ESCAPE == 0 { ((*(*synth).io_ops).synth_out)(synth, PROCSPEECH); }
}

unsafe fn synth_flush_impl(synth: *mut spk_synth) {
    IN_ESCAPE = 0;
    ((*(*synth).io_ops).flush_buffer)(synth);
    ((*synth).synth_immediate)(synth, "\x1bP;10z\x1b\\");
}

// module parameters and module metadata are provided by the kernel-facing Rust layer.
module_param_named!(ser, SYNTH_DECEXT.ser, i32, 0o444);
module_param_named!(dev, SYNTH_DECEXT.dev_name, *mut i8, 0o444);
module_param_named!(start, SYNTH_DECEXT.startup, i16, 0o444);
module_param_named!(rate, VARS[DefaultVarsId::RATE_ID as usize].u.n.default_val, i32, 0o444);
module_param_named!(pitch, VARS[DefaultVarsId::PITCH_ID as usize].u.n.default_val, i32, 0o444);
module_param_named!(inflection, VARS[DefaultVarsId::INFLECTION_ID as usize].u.n.default_val, i32, 0o444);
module_param_named!(vol, VARS[DefaultVarsId::VOL_ID as usize].u.n.default_val, i32, 0o444);
module_param_named!(punct, VARS[DefaultVarsId::PUNCT_ID as usize].u.n.default_val, i32, 0o444);
module_param_named!(voice, VARS[DefaultVarsId::VOICE_ID as usize].u.n.default_val, i32, 0o444);
module_param_named!(direct, VARS[DefaultVarsId::DIRECT_ID as usize].u.n.default_val, i32, 0o444);

module_spk_synth!(SYNTH_DECEXT);
module_author!("Kirk Reiser <kirk@braille.uwo.ca>");
module_author!("David Borowski");
module_description!("Speakup support for DECtalk External synthesizers");
module_license!("GPL");
module_version!(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
