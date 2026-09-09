// SPDX-License-Identifier: GPL-2.0+
/*
 * written by: Kirk Reiser <kirk@braille.uwo.ca>
 * this version considerably modified by David Borowski, david575@rogers.com
 *
 * Copyright (C) 1998-99  Kirk Reiser.
 * Copyright (C) 2003 David Borowski.
 *
 * this code is specifically written as a driver for the speakup screenreview
 * package and is not a general device driver.
 * This driver is for the Aicom Acent PC internal synthesizer.
 */

// Linux kernel dependencies: jiffies, scheduler, timer, kthread, spk_priv,
// serialio, speakup, and speakup_acnt declarations are supplied externally.

const DRV_VERSION: &str = "2.10";
const PROCSPEECH: u8 = b'\r';

// Function declarations from the C source are implemented below.

static mut synth_port_control: i32 = 0;
static mut port_forced: i32 = 0;
static mut synth_portlist: [u32; 2] = [0x2a8, 0];

#[repr(C)]
enum default_vars_id { CAPS_START_ID = 0, CAPS_STOP_ID, RATE_ID, PITCH_ID, VOL_ID, TONE_ID, DIRECT_ID, V_LAST_VAR_ID, NB_ID }

// These attributes appear in /sys/accessibility/speakup/acntpc.
// The kernel var_t, kobj_attribute, spk_synth, and related definitions are
// supplied by the surrounding translation unit.
static mut vars: [var_t; NB_ID as usize] = [var_t::default(); NB_ID as usize];

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

// Create a group of attributes so that they can be created and destroyed together.
static mut synth_attrs: [*mut attribute; 12] = [
    unsafe { &mut caps_start_attribute.attr }, unsafe { &mut caps_stop_attribute.attr },
    unsafe { &mut pitch_attribute.attr }, unsafe { &mut rate_attribute.attr },
    unsafe { &mut tone_attribute.attr }, unsafe { &mut vol_attribute.attr },
    unsafe { &mut delay_time_attribute.attr }, unsafe { &mut direct_attribute.attr },
    unsafe { &mut full_time_attribute.attr }, unsafe { &mut jiffy_delta_attribute.attr },
    unsafe { &mut trigger_time_attribute.attr }, core::ptr::null_mut(),
];

static mut synth_acntpc: spk_synth = spk_synth {
    name: "acntpc", version: DRV_VERSION, long_name: "Accent PC",
    init: "\033=X \033Oi\033T2\033=M\033N1\n", procspeech: PROCSPEECH,
    clear: SYNTH_CLEAR, delay: 500, trigger: 50, jiffies: 50, full: 1000,
    startup: SYNTH_START, checkval: SYNTH_CHECK, vars: unsafe { vars.as_mut_ptr() },
    io_ops: &spk_serial_io_ops, probe: Some(synth_probe), release: Some(accent_release),
    synth_immediate: Some(synth_immediate), catch_up: Some(do_catch_up), flush: Some(synth_flush),
    is_alive: Some(spk_synth_is_alive_nop), synth_adjust: None, read_buff_add: None,
    get_index: None, indexing: indexing_t { command: None, lowindex: 0, highindex: 0, currindex: 0 },
    attributes: attribute_group { attrs: synth_attrs.as_mut_ptr(), name: "acntpc" },
};

unsafe fn synth_writable() -> bool { inb_p(synth_port_control as u16) & SYNTH_WRITABLE != 0 }
unsafe fn synth_full() -> bool { inb_p(speakup_info.port_tts + UART_RX) == b'F' as u32 }

unsafe fn synth_immediate(_synth: *mut spk_synth, mut buf: *const u8) -> *const u8 {
    while *buf != 0 {
        let mut timeout = SPK_XMITR_TIMEOUT;
        let mut ch = *buf;
        if ch == b'\n' { ch = PROCSPEECH; }
        if synth_full() { return buf; }
        while synth_writable() {
            timeout -= 1;
            if timeout == 0 { return buf; }
            udelay(1);
        }
        outb_p(ch, speakup_info.port_tts);
        buf = buf.add(1);
    }
    core::ptr::null()
}

unsafe fn do_catch_up(synth: *mut spk_synth) {
    let mut flags: unsigned_long = 0;
    let mut jiff_max: unsigned_long;
    let mut timeout: i32;
    let mut delay_time_val: i32;
    let mut jiffy_delta_val: i32;
    let mut full_time_val: i32;
    let jiffy_delta = spk_get_var(JIFFY);
    let delay_time = spk_get_var(DELAY);
    let full_time = spk_get_var(FULL);
    spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags);
    jiffy_delta_val = (*jiffy_delta).u.n.value;
    spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
    jiff_max = jiffies + jiffy_delta_val as unsigned_long;
    while !kthread_should_stop() {
        spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags);
        if speakup_info.flushing != 0 {
            speakup_info.flushing = 0;
            spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
            synth_flush(synth);
            continue;
        }
        synth_buffer_skip_nonlatin1();
        if synth_buffer_empty() {
            spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
            break;
        }
        set_current_state(TASK_INTERRUPTIBLE);
        full_time_val = (*full_time).u.n.value;
        spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
        if synth_full() { schedule_timeout(msecs_to_jiffies(full_time_val)); continue; }
        set_current_state(TASK_RUNNING);
        timeout = SPK_XMITR_TIMEOUT;
        while synth_writable() { timeout -= 1; if timeout == 0 { break; } udelay(1); }
        spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags);
        let mut ch = synth_buffer_getc();
        spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
        if ch == b'\n' { ch = PROCSPEECH; }
        outb_p(ch, speakup_info.port_tts);
        if time_after_eq(jiffies, jiff_max) && ch == SPACE {
            timeout = SPK_XMITR_TIMEOUT;
            while synth_writable() { timeout -= 1; if timeout == 0 { break; } udelay(1); }
            outb_p(PROCSPEECH, speakup_info.port_tts);
            spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags);
            jiffy_delta_val = (*jiffy_delta).u.n.value;
            delay_time_val = (*delay_time).u.n.value;
            spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
            schedule_timeout(msecs_to_jiffies(delay_time_val));
            jiff_max = jiffies + jiffy_delta_val as unsigned_long;
        }
    }
    timeout = SPK_XMITR_TIMEOUT;
    while synth_writable() { timeout -= 1; if timeout == 0 { break; } udelay(1); }
    outb_p(PROCSPEECH, speakup_info.port_tts);
}

unsafe fn synth_flush(_synth: *mut spk_synth) { outb_p(SYNTH_CLEAR, speakup_info.port_tts); }

unsafe fn synth_probe(synth: *mut spk_synth) -> i32 {
    let mut port_val: u32 = 0;
    let mut i = 0usize;
    pr_info!("Probing for {}.\n", (*synth).long_name);
    if port_forced != 0 {
        speakup_info.port_tts = port_forced as u32;
        if synth_request_region(speakup_info.port_tts - 1, SYNTH_IO_EXTENT) != 0 { pr_warn!("sorry, port already reserved\n"); return -EBUSY; }
        port_val = inw(speakup_info.port_tts - 1);
        synth_port_control = speakup_info.port_tts as i32 - 1;
    } else {
        while synth_portlist[i] != 0 {
            if synth_request_region(synth_portlist[i], SYNTH_IO_EXTENT) != 0 { i += 1; continue; }
            port_val = inw(synth_portlist[i]) & 0xfffc;
            if port_val == 0x53fc { synth_port_control = synth_portlist[i] as i32; speakup_info.port_tts = synth_portlist[i] + 1; break; }
            i += 1;
        }
    }
    port_val &= 0xfffc;
    if port_val != 0x53fc { pr_info!("{}: not found\n", (*synth).long_name); synth_release_region(synth_port_control as u32, SYNTH_IO_EXTENT); synth_port_control = 0; return -ENODEV; }
    pr_info!("{}: {:03x}-{:03x}, driver version {},\n", (*synth).long_name, synth_port_control, synth_port_control + SYNTH_IO_EXTENT as i32 - 1, (*synth).version);
    (*synth).alive = 1;
    0
}

unsafe fn accent_release(_synth: *mut spk_synth) {
    spk_stop_serial_interrupt();
    if speakup_info.port_tts != 0 { synth_release_region(speakup_info.port_tts - 1, SYNTH_IO_EXTENT); }
    speakup_info.port_tts = 0;
}

// module_param/module description and module_spk_synth registrations are
// retained as build-time integration points supplied by the kernel bindings.
// module_param_hw_named(port, port_forced, int, ioport, 0444);
// module_param_named(start, synth_acntpc.startup, short, 0444);
// module_param_named(rate, vars[RATE_ID].u.n.default_val, int, 0444);
// module_param_named(pitch, vars[PITCH_ID].u.n.default_val, int, 0444);
// module_param_named(vol, vars[VOL_ID].u.n.default_val, int, 0444);
// module_param_named(tone, vars[TONE_ID].u.n.default_val, int, 0444);
// module_param_named(direct, vars[DIRECT_ID].u.n.default_val, int, 0444);
// MODULE_PARM_DESC(port, "Set the port for the synthesizer (override probing).");
// MODULE_PARM_DESC(start, "Start the synthesizer once it is loaded.");
// MODULE_PARM_DESC(rate, "Set the rate variable on load.");
// MODULE_PARM_DESC(pitch, "Set the pitch variable on load.");
// MODULE_PARM_DESC(vol, "Set the vol variable on load.");
// MODULE_PARM_DESC(tone, "Set the tone variable on load.");
// MODULE_PARM_DESC(direct, "Set the direct variable on load.");
// module_spk_synth(synth_acntpc);
// MODULE_AUTHOR("Kirk Reiser <kirk@braille.uwo.ca>");
// MODULE_AUTHOR("David Borowski");
// MODULE_DESCRIPTION("Speakup support for Accent PC synthesizer");
// MODULE_LICENSE("GPL");
// MODULE_VERSION(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
