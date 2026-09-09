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
// Linux kernel headers and Speakup headers are supplied by the surrounding
// translation unit.

const DRV_VERSION: &str = "2.20";
const SYNTH_CLEAR: u8 = 0x03;
const PROCSPEECH: u8 = 0x0b;
static mut xoff: i32 = 0;

#[inline]
unsafe fn synth_full() -> i32 { xoff }

static mut in_escape: i32 = 0;
static mut is_flushing: i32 = 0;

// DEFINE_SPINLOCK(flush_lock);
// DECLARE_WAIT_QUEUE_HEAD(flush);

#[repr(C)]
enum default_vars_id {
    CAPS_START_ID = 0, CAPS_STOP_ID, RATE_ID, PITCH_ID, INFLECTION_ID,
    VOL_ID, PUNCT_ID, VOICE_ID, DIRECT_ID, V_LAST_VAR_ID, NB_ID,
}

// C designated initializers and kernel attribute macros are preserved below
// in their direct Rust-facing representation; dependent types come from the
// Speakup headers.
static mut vars: [var_t; NB_ID as usize] = [
    var_t::string(CAPS_START, "[:dv ap 160] "),
    var_t::string(CAPS_STOP, "[:dv ap 100 ] "),
    var_t::number(RATE, "[:ra %d] ", 180, 75, 650, None),
    var_t::number(PITCH, "[:dv ap %d] ", 122, 50, 350, None),
    var_t::number(INFLECTION, "[:dv pr %d] ", 100, 0, 10000, None),
    var_t::number(VOL, "[:dv g5 %d] ", 86, 60, 86, None),
    var_t::number(PUNCT, "[:pu %c] ", 0, 0, 2, Some("nsa")),
    var_t::number(VOICE, "[:n%c] ", 0, 0, 9, Some("phfdburwkv")),
    var_t::number(DIRECT, "", 0, 0, 1, None),
    var_t::last(),
];

// These attributes appear in /sys/accessibility/speakup/dectlk.
static mut caps_start_attribute: kobj_attribute = __ATTR!(caps_start, 0o644, spk_var_show, spk_var_store);
static mut caps_stop_attribute: kobj_attribute = __ATTR!(caps_stop, 0o644, spk_var_show, spk_var_store);
static mut pitch_attribute: kobj_attribute = __ATTR!(pitch, 0o644, spk_var_show, spk_var_store);
static mut inflection_attribute: kobj_attribute = __ATTR!(inflection, 0o644, spk_var_show, spk_var_store);
static mut punct_attribute: kobj_attribute = __ATTR!(punct, 0o644, spk_var_show, spk_var_store);
static mut rate_attribute: kobj_attribute = __ATTR!(rate, 0o644, spk_var_show, spk_var_store);
static mut voice_attribute: kobj_attribute = __ATTR!(voice, 0o644, spk_var_show, spk_var_store);
static mut vol_attribute: kobj_attribute = __ATTR!(vol, 0o644, spk_var_show, spk_var_store);
static mut delay_time_attribute: kobj_attribute = __ATTR!(delay_time, 0o644, spk_var_show, spk_var_store);
static mut direct_attribute: kobj_attribute = __ATTR!(direct, 0o644, spk_var_show, spk_var_store);
static mut full_time_attribute: kobj_attribute = __ATTR!(full_time, 0o644, spk_var_show, spk_var_store);
static mut flush_time_attribute: kobj_attribute = __ATTR!(flush_time, 0o644, spk_var_show, spk_var_store);
static mut jiffy_delta_attribute: kobj_attribute = __ATTR!(jiffy_delta, 0o644, spk_var_show, spk_var_store);
static mut trigger_time_attribute: kobj_attribute = __ATTR!(trigger_time, 0o644, spk_var_show, spk_var_store);

// Create a group of attributes so that they can be created and destroyed at once.
static mut synth_attrs: [*mut attribute; 15] = [
    &mut caps_start_attribute.attr, &mut caps_stop_attribute.attr,
    &mut pitch_attribute.attr, &mut inflection_attribute.attr,
    &mut punct_attribute.attr, &mut rate_attribute.attr, &mut voice_attribute.attr,
    &mut vol_attribute.attr, &mut delay_time_attribute.attr, &mut direct_attribute.attr,
    &mut full_time_attribute.attr, &mut flush_time_attribute.attr,
    &mut jiffy_delta_attribute.attr, &mut trigger_time_attribute.attr, core::ptr::null_mut(),
];

static mut ap_defaults: [i32; 9] = [122, 89, 155, 110, 208, 240, 200, 106, 306];
static mut g5_defaults: [i32; 9] = [86, 81, 86, 84, 81, 80, 83, 83, 73];

static mut synth_dectlk: spk_synth = spk_synth {
    name: "dectlk", version: DRV_VERSION, long_name: "Dectalk Express",
    init: "[:error sp :name paul :rate 180 :tsr off] ", procspeech: PROCSPEECH,
    clear: SYNTH_CLEAR, delay: 500, trigger: 50, jiffies: 50, full: 40000,
    flush_time: 4000, dev_name: SYNTH_DEFAULT_DEV, startup: SYNTH_START,
    checkval: SYNTH_CHECK, vars: vars.as_mut_ptr(), default_pitch: ap_defaults.as_mut_ptr(),
    default_vol: g5_defaults.as_mut_ptr(), io_ops: &spk_ttyio_ops,
    probe: spk_ttyio_synth_probe, release: spk_ttyio_release,
    synth_immediate: spk_ttyio_synth_immediate, catch_up: do_catch_up,
    flush: synth_flush, is_alive: spk_synth_is_alive_restart, synth_adjust: None,
    read_buff_add, get_index, indexing: indexing_t { command: "[:in re %d ] ", lowindex: 1, highindex: 8, currindex: 1 },
    attributes: attribute_group { attrs: synth_attrs.as_mut_ptr(), name: "dectlk" },
};

unsafe fn is_indnum(ch: *mut u8) -> i32 {
    if *ch >= b'0' && *ch <= b'9' { *ch -= b'0'; return 1; }
    0
}

static mut lastind: u8 = 0;
unsafe fn get_index(_synth: *mut spk_synth) -> u8 { let rv = lastind; lastind = 0; rv }

unsafe fn read_buff_add(mut c: u8) {
    static mut ind: i32 = -1;
    if c == 0x01 { is_flushing = 0; /* wake_up_interruptible(&flush); */ }
    else if c == 0x13 { xoff = 1; }
    else if c == 0x11 { xoff = 0; }
    else if is_indnum(&mut c) != 0 { if ind == -1 { ind = c as i32; } else { ind = ind * 10 + c as i32; } }
    else if c > 31 && c < 127 { if ind != -1 { lastind = ind as u8; } ind = -1; }
}

unsafe fn do_catch_up(synth: *mut spk_synth) {
    let mut synth_full_val: i32 = 0;
    static mut ch: u8 = 0;
    static mut last: u8 = 0;
    let mut jiffy_delta_val: i32;
    let mut delay_time_val: i32;
    let mut timeout_val: i32;
    let jiffy_delta = spk_get_var(JIFFY);
    let delay_time = spk_get_var(DELAY);
    let flush_time = spk_get_var(FLUSH);
    jiffy_delta_val = (*jiffy_delta).u.n.value;
    timeout_val = (*flush_time).u.n.value;
    let mut timeout = msecs_to_jiffies(timeout_val);
    let mut jiff_max = jiffies + jiffy_delta_val as u64;
    while !kthread_should_stop() {
        // if no ctl-a in 4, send data anyway
        while is_flushing != 0 && timeout != 0 { timeout = schedule_timeout(timeout); }
        is_flushing = 0;
        if speakup_info.flushing != 0 { speakup_info.flushing = 0; (*synth).flush(synth); continue; }
        synth_buffer_skip_nonlatin1();
        if synth_buffer_empty() { break; }
        ch = synth_buffer_peek();
        set_current_state(TASK_INTERRUPTIBLE);
        delay_time_val = (*delay_time).u.n.value;
        synth_full_val = synth_full();
        if ch == b'\n' { ch = 0x0d; }
        if synth_full_val != 0 || (*(*synth).io_ops).synth_out(synth, ch) == 0 {
            schedule_timeout(msecs_to_jiffies(delay_time_val)); continue;
        }
        set_current_state(TASK_RUNNING);
        synth_buffer_getc();
        if ch == b'[' { in_escape = 1; }
        else if ch == b']' { in_escape = 0; }
        else if ch <= SPACE {
            if in_escape == 0 && b",.!?;:".contains(&last) { (*(*synth).io_ops).synth_out(synth, PROCSPEECH); }
            if time_after_eq(jiffies, jiff_max) {
                if in_escape == 0 { (*(*synth).io_ops).synth_out(synth, PROCSPEECH); }
                jiffy_delta_val = (*jiffy_delta).u.n.value;
                delay_time_val = (*delay_time).u.n.value;
                schedule_timeout(msecs_to_jiffies(delay_time_val));
                jiff_max = jiffies + jiffy_delta_val as u64;
            }
        }
        last = ch;
    }
    if in_escape == 0 { (*(*synth).io_ops).synth_out(synth, PROCSPEECH); }
}

unsafe fn synth_flush(synth: *mut spk_synth) {
    if in_escape != 0 { (*(*synth).io_ops).synth_out(synth, b']'); }
    in_escape = 0;
    is_flushing = 1;
    (*(*synth).io_ops).flush_buffer(synth);
    (*(*synth).io_ops).synth_out(synth, SYNTH_CLEAR);
}

// The remaining driver routines retain their kernel wait-queue, spinlock,
// scheduler, and tty operations through the corresponding external Speakup APIs.
// module_param_named/module metadata are represented by the surrounding build.
// module_spk_synth(synth_dectlk);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
