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

// Linux kernel headers and Speakup headers are supplied by the surrounding build.

const DRV_VERSION: &str = "2.21";
const SYNTH_CLEAR: u8 = 0x18;
const PROCSPEECH: u8 = b'\r';

#[repr(C)]
enum default_vars_id {
    CAPS_START_ID = 0,
    CAPS_STOP_ID,
    RATE_ID,
    PITCH_ID,
    VOL_ID,
    VOICE_ID,
    LANG_ID,
    DIRECT_ID,
    V_LAST_VAR_ID,
    NB_ID,
}

// The concrete definitions of these kernel/Speakup types and symbols come from the supplied headers.
static mut vars: [var_t; NB_ID as usize] = [
    var_t { var_id: CAPS_START, u: var_union { s: string_var { value: "cap, " } } },
    var_t { var_id: CAPS_STOP, u: var_union { s: string_var { value: "" } } },
    var_t { var_id: RATE, u: var_union { n: numeric_var { name: "@W%d", default_val: 6, value: 1, low: 9, high: 0, step: 0, multiplier: 0, shur: core::ptr::null_mut() } } },
    var_t { var_id: PITCH, u: var_union { n: numeric_var { name: "@F%x", default_val: 10, value: 0, low: 15, high: 0, step: 0, multiplier: 0, shur: core::ptr::null_mut() } } },
    var_t { var_id: VOL, u: var_union { n: numeric_var { name: "@A%x", default_val: 10, value: 0, low: 15, high: 0, step: 0, multiplier: 0, shur: core::ptr::null_mut() } } },
    var_t { var_id: VOICE, u: var_union { n: numeric_var { name: "@V%d", default_val: 1, value: 1, low: 6, high: 0, step: 0, multiplier: 0, shur: core::ptr::null_mut() } } },
    var_t { var_id: LANG, u: var_union { n: numeric_var { name: "@=%d,", default_val: 1, value: 1, low: 4, high: 0, step: 0, multiplier: 0, shur: core::ptr::null_mut() } } },
    var_t { var_id: DIRECT, u: var_union { n: numeric_var { name: core::ptr::null(), default_val: 0, value: 0, low: 1, high: 0, step: 0, multiplier: 0, shur: core::ptr::null_mut() } } },
];

static mut synth_attrs: [*mut attribute; 13] = [
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(),
];

static mut synth_apollo: spk_synth = spk_synth {
    name: "apollo",
    version: DRV_VERSION,
    long_name: "Apollo",
    init: "@R3@D0@K1\r",
    procspeech: PROCSPEECH,
    clear: SYNTH_CLEAR,
    delay: 500,
    trigger: 50,
    jiffies: 50,
    full: 40000,
    dev_name: SYNTH_DEFAULT_DEV,
    startup: SYNTH_START,
    checkval: SYNTH_CHECK,
    vars: core::ptr::addr_of_mut!(vars) as *mut var_t,
    io_ops: &spk_ttyio_ops,
    probe: Some(spk_ttyio_synth_probe),
    release: Some(spk_ttyio_release),
    synth_immediate: Some(spk_ttyio_synth_immediate),
    catch_up: Some(do_catch_up),
    flush: Some(spk_synth_flush),
    is_alive: Some(spk_synth_is_alive_restart),
    synth_adjust: None,
    read_buff_add: None,
    get_index: None,
    indexing: indexing { command: None, lowindex: 0, highindex: 0, currindex: 0 },
    attributes: attribute_group { attrs: core::ptr::addr_of_mut!(synth_attrs) as *mut *mut attribute, name: "apollo" },
};

unsafe fn do_catch_up(synth: *mut spk_synth) {
    let mut ch: u8;
    let mut flags: c_ulong;
    let mut jiff_max: c_ulong;
    let jiffy_delta: *mut var_t = spk_get_var(JIFFY);
    let delay_time: *mut var_t = spk_get_var(DELAY);
    let full_time: *mut var_t = spk_get_var(FULL);
    let mut full_time_val: c_int = 0;
    let mut delay_time_val: c_int = 0;
    let mut jiffy_delta_val: c_int = 0;

    spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags);
    jiffy_delta_val = (*jiffy_delta).u.n.value;
    spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
    jiff_max = jiffies.wrapping_add(jiffy_delta_val as c_ulong);

    while !kthread_should_stop() {
        spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags);
        jiffy_delta_val = (*jiffy_delta).u.n.value;
        full_time_val = (*full_time).u.n.value;
        delay_time_val = (*delay_time).u.n.value;
        if speakup_info.flushing != 0 {
            speakup_info.flushing = 0;
            spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
            ((*synth).flush.unwrap())(synth);
            continue;
        }
        synth_buffer_skip_nonlatin1();
        if synth_buffer_empty() {
            spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
            break;
        }
        ch = synth_buffer_peek();
        set_current_state(TASK_INTERRUPTIBLE);
        full_time_val = (*full_time).u.n.value;
        spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
        if !((*(*synth).io_ops).synth_out.unwrap())(synth, ch) {
            ((*(*synth).io_ops).tiocmset.unwrap())(synth, 0, UART_MCR_RTS);
            ((*(*synth).io_ops).tiocmset.unwrap())(synth, UART_MCR_RTS, 0);
            schedule_timeout(msecs_to_jiffies(full_time_val));
            continue;
        }
        if time_after_eq(jiffies, jiff_max) && ch == SPACE {
            spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags);
            jiffy_delta_val = (*jiffy_delta).u.n.value;
            full_time_val = (*full_time).u.n.value;
            delay_time_val = (*delay_time).u.n.value;
            spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
            if ((*(*synth).io_ops).synth_out.unwrap())(synth, (*synth).procspeech) {
                schedule_timeout(msecs_to_jiffies(delay_time_val));
            } else {
                schedule_timeout(msecs_to_jiffies(full_time_val));
            }
            jiff_max = jiffies.wrapping_add(jiffy_delta_val as c_ulong);
        }
        set_current_state(TASK_RUNNING);
        spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags);
        synth_buffer_getc();
        spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
    }
    ((*(*synth).io_ops).synth_out.unwrap())(synth, PROCSPEECH);
}

// module_param_named and MODULE_* declarations retain their kernel-module metadata.
// module_spk_synth(synth_apollo);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
