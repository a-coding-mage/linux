// SPDX-License-Identifier: GPL-2.0+
/* speakup_soft.c - speakup driver to register and make available
 * a user space device for software synthesizers.  written by: Kirk
 * Reiser <kirk@braille.uwo.ca>
 *
 * Copyright (C) 2003  Kirk Reiser.
 *
 * this code is specifically written as a driver for the speakup screenreview
 * package and is not a general device driver.
 */

// Kernel dependencies supplied by the surrounding translation unit.

const DRV_VERSION: &str = "2.6";
const PROCSPEECH: u8 = 0x0d;
const CLEAR_SYNTH: u8 = 0x18;

const DIRECT_ID: usize = 0;
const CAPS_START_ID: usize = 1;
const CAPS_STOP_ID: usize = 2;
const PAUSE_ID: usize = 3;
const RATE_ID: usize = 4;
const PITCH_ID: usize = 5;
const INFLECTION_ID: usize = 6;
const VOL_ID: usize = 7;
const TONE_ID: usize = 8;
const PUNCT_ID: usize = 9;
const VOICE_ID: usize = 10;
const FREQUENCY_ID: usize = 11;
const V_LAST_VAR_ID: usize = 12;
const NB_ID: usize = 13;

static mut vars: [var_t; NB_ID] = unsafe { core::mem::MaybeUninit::uninit().assume_init() };

/* These attributes will appear in /sys/accessibility/speakup/soft. */
static mut caps_start_attribute: kobj_attribute = kobj_attribute { __private: [] };
static mut caps_stop_attribute: kobj_attribute = kobj_attribute { __private: [] };
static mut freq_attribute: kobj_attribute = kobj_attribute { __private: [] };
static mut pitch_attribute: kobj_attribute = kobj_attribute { __private: [] };
static mut inflection_attribute: kobj_attribute = kobj_attribute { __private: [] };
static mut punct_attribute: kobj_attribute = kobj_attribute { __private: [] };
static mut rate_attribute: kobj_attribute = kobj_attribute { __private: [] };
static mut tone_attribute: kobj_attribute = kobj_attribute { __private: [] };
static mut voice_attribute: kobj_attribute = kobj_attribute { __private: [] };
static mut vol_attribute: kobj_attribute = kobj_attribute { __private: [] };
static mut delay_time_attribute: kobj_attribute = kobj_attribute { __private: [] };
static mut direct_attribute: kobj_attribute = kobj_attribute { __private: [] };
static mut full_time_attribute: kobj_attribute = kobj_attribute { __private: [] };
static mut jiffy_delta_attribute: kobj_attribute = kobj_attribute { __private: [] };
static mut trigger_time_attribute: kobj_attribute = kobj_attribute { __private: [] };

/* Create a group of attributes so that we can create and destroy them all at once. */
static mut synth_attrs: [*mut attribute; 16] = [
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(),
];

static mut synth_soft: spk_synth = spk_synth { __private: [] };
static mut synth_device: miscdevice = miscdevice { __private: [] };
static mut synthu_device: miscdevice = miscdevice { __private: [] };
static mut init_pos: usize = 0;
static mut misc_registered: i32 = 0;
static mut last_index: i32 = 0;

static softsynth_fops: file_operations = file_operations { __private: [] };
static softsynthu_fops: file_operations = file_operations { __private: [] };

unsafe fn get_initstring() -> *mut i8 {
    static mut buf: [i8; 40] = [0; 40];
    let mut cp = buf.as_mut_ptr();
    let mut len = buf.len();
    let mut var = synth_soft.vars;
    while (*var).var_id != MAXVARS {
        if (*var).var_id != CAPS_START && (*var).var_id != CAPS_STOP &&
           (*var).var_id != PAUSE && (*var).var_id != DIRECT {
            let n = scnprintf(cp, len, (*var).u.n.synth_fmt, (*var).u.n.value);
            cp = cp.add(n); len -= n;
        }
        var = var.add(1);
    }
    let n = scnprintf(cp, len, b"\n\0".as_ptr() as *const i8);
    cp = cp.add(n);
    buf.as_mut_ptr()
}

unsafe fn softsynth_open(_inode: *mut inode, _fp: *mut file) -> i32 {
    let mut flags = 0ul;
    spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags);
    if synth_soft.alive != 0 { spin_unlock_irqrestore(&mut speakup_info.spinlock, flags); return -EBUSY; }
    synth_soft.alive = 1;
    spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
    0
}

unsafe fn softsynth_close(_inode: *mut inode, _fp: *mut file) -> i32 {
    let mut flags = 0ul;
    spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags);
    synth_soft.alive = 0; init_pos = 0;
    spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
    speakup_start_ttys();
    0
}

unsafe fn softsynthx_read(fp: *mut file, buf: *mut u8, count: usize, pos: *mut loff_t, unicode: bool) -> isize {
    let mut chars_sent: isize = 0;
    let bytes_per_ch = if unicode { 3 } else { 1 };
    if count < bytes_per_ch { return -EINVAL as isize; }
    let mut flags = 0ul;
    spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags);
    synth_soft.alive = 1;
    loop {
        prepare_to_wait(&mut speakup_event, core::ptr::null_mut(), TASK_INTERRUPTIBLE);
        if synth_current() == &mut synth_soft as *mut _ {
            if !unicode { synth_buffer_skip_nonlatin1(); }
            if !synth_buffer_empty() || speakup_info.flushing != 0 { break; }
        }
        spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
        if (*fp).f_flags & O_NONBLOCK != 0 { finish_wait(&mut speakup_event, core::ptr::null_mut()); return -EAGAIN as isize; }
        if signal_pending(current) != 0 { finish_wait(&mut speakup_event, core::ptr::null_mut()); return -ERESTARTSYS as isize; }
        schedule();
        spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags);
    }
    finish_wait(&mut speakup_event, core::ptr::null_mut());
    let init = get_initstring();
    while chars_sent <= (count - bytes_per_ch) as isize {
        if synth_current() != &mut synth_soft as *mut _ { break; }
        let ch: u16;
        if speakup_info.flushing != 0 { speakup_info.flushing = 0; ch = CLEAR_SYNTH as u16; }
        else if *init.add(init_pos) != 0 { ch = *init.add(init_pos) as u16; init_pos += 1; }
        else { if !unicode { synth_buffer_skip_nonlatin1(); } if synth_buffer_empty() { break; } ch = synth_buffer_getc(); }
        spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
        if (!unicode && ch < 0x100) || (unicode && ch < 0x80) { *buf.add(chars_sent as usize) = ch as u8; chars_sent += 1; }
        else if unicode && ch < 0x800 { *buf.add(chars_sent as usize) = (0xc0 | (ch >> 6)) as u8; *buf.add(chars_sent as usize + 1) = (0x80 | (ch & 0x3f)) as u8; chars_sent += 2; }
        else if unicode { *buf.add(chars_sent as usize) = (0xe0 | (ch >> 12)) as u8; *buf.add(chars_sent as usize + 1) = (0x80 | ((ch >> 6) & 0x3f)) as u8; *buf.add(chars_sent as usize + 2) = (0x80 | (ch & 0x3f)) as u8; chars_sent += 3; }
        spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags);
    }
    *pos += chars_sent as loff_t;
    let empty = synth_buffer_empty();
    spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
    if empty { speakup_start_ttys(); *pos = 0; }
    chars_sent
}

unsafe fn softsynth_read(fp: *mut file, buf: *mut u8, count: usize, pos: *mut loff_t) -> isize { softsynthx_read(fp, buf, count, pos, false) }
unsafe fn softsynthu_read(fp: *mut file, buf: *mut u8, count: usize, pos: *mut loff_t) -> isize { softsynthx_read(fp, buf, count, pos, true) }

unsafe fn softsynth_write(_fp: *mut file, buf: *const u8, count: usize, _pos: *mut loff_t) -> isize {
    let mut supplied_index = 0ul;
    let converted = kstrtoul_from_user(buf, count, 0, &mut supplied_index);
    if converted < 0 { return converted as isize; }
    last_index = supplied_index as i32; count as isize
}

unsafe fn softsynth_poll(fp: *mut file, wait: *mut poll_table_struct) -> __poll_t {
    poll_wait(fp, &mut speakup_event, wait);
    let mut flags = 0ul; let mut ret = 0;
    spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags);
    if synth_current() == &mut synth_soft as *mut _ && (!synth_buffer_empty() || speakup_info.flushing != 0) { ret = EPOLLIN | EPOLLRDNORM; }
    spin_unlock_irqrestore(&mut speakup_info.spinlock, flags); ret
}

unsafe fn get_index(_synth: *mut spk_synth) -> u8 { let rv = last_index; last_index = 0; rv as u8 }

unsafe fn softsynth_probe(_synth: *mut spk_synth) -> i32 {
    if misc_registered != 0 { return 0; }
    memset(&mut synth_device as *mut _ as *mut u8, 0, core::mem::size_of::<miscdevice>());
    synth_device.minor = MISC_DYNAMIC_MINOR; synth_device.name = b"softsynth\0".as_ptr() as *const i8; synth_device.fops = &softsynth_fops;
    if misc_register(&mut synth_device) != 0 { pr_warn!("Couldn't initialize miscdevice /dev/softsynth.\n"); return -ENODEV; }
    memset(&mut synthu_device as *mut _ as *mut u8, 0, core::mem::size_of::<miscdevice>());
    synthu_device.minor = MISC_DYNAMIC_MINOR; synthu_device.name = b"softsynthu\0".as_ptr() as *const i8; synthu_device.fops = &softsynthu_fops;
    if misc_register(&mut synthu_device) != 0 { misc_deregister(&mut synth_device); pr_warn!("Couldn't initialize miscdevice /dev/softsynthu.\n"); return -ENODEV; }
    misc_registered = 1;
    pr_info!("initialized device: /dev/softsynth, node (MAJOR 10, MINOR %d)\n", synth_device.minor);
    pr_info!("initialized device: /dev/softsynthu, node (MAJOR 10, MINOR %d)\n", synthu_device.minor); 0
}

unsafe fn softsynth_release(_synth: *mut spk_synth) { misc_deregister(&mut synth_device); misc_deregister(&mut synthu_device); misc_registered = 0; pr_info!("unregistered /dev/softsynth\n"); pr_info!("unregistered /dev/softsynthu\n"); }
unsafe fn softsynth_is_alive(_synth: *mut spk_synth) -> i32 { if synth_soft.alive != 0 { 1 } else { 0 } }

unsafe fn softsynth_adjust(_synth: *mut spk_synth, var: *mut st_var_header) -> i32 {
    if (*var).var_id != PUNC_LEVEL { return 0; }
    let var_data = (*var).data; if var_data.is_null() { return 0; }
    let punc_level_var = spk_get_var_header(PUNCT); if punc_level_var.is_null() { return 0; }
    spk_set_num_var((*var_data).u.n.value, punc_level_var, E_SET); 1
}

// module_param_named and MODULE_* metadata are supplied by the kernel build environment.
module_param_named!(start, synth_soft.startup, short, 0444);
module_param_named!(direct, vars[DIRECT_ID].u.n.default_val, int, 0444);
module_param_named!(rate, vars[RATE_ID].u.n.default_val, int, 0444);
module_param_named!(pitch, vars[PITCH_ID].u.n.default_val, int, 0444);
module_param_named!(inflection, vars[INFLECTION_ID].u.n.default_val, int, 0444);
module_param_named!(vol, vars[VOL_ID].u.n.default_val, int, 0444);
module_param_named!(tone, vars[TONE_ID].u.n.default_val, int, 0444);
module_param_named!(punct, vars[PUNCT_ID].u.n.default_val, int, 0444);
module_param_named!(voice, vars[VOICE_ID].u.n.default_val, int, 0444);
module_param_named!(frequency, vars[FREQUENCY_ID].u.n.default_val, int, 0444);
MODULE_PARM_DESC!(start, "Start the synthesizer once it is loaded.");
MODULE_PARM_DESC!(direct, "Set the direct variable on load.");
MODULE_PARM_DESC!(rate, "Sets the rate of the synthesizer.");
MODULE_PARM_DESC!(pitch, "Sets the pitch of the synthesizer.");
MODULE_PARM_DESC!(inflection, "Sets the inflection of the synthesizer.");
MODULE_PARM_DESC!(vol, "Sets the volume of the speech synthesizer.");
MODULE_PARM_DESC!(tone, "Sets the tone of the speech synthesizer.");
MODULE_PARM_DESC!(punct, "Sets the amount of punctuation spoken by the synthesizer.");
MODULE_PARM_DESC!(voice, "Sets the voice used by the synthesizer.");
MODULE_PARM_DESC!(frequency, "Sets the frequency of speech synthesizer.");
module_spk_synth!(synth_soft);
MODULE_AUTHOR!("Kirk Reiser <kirk@braille.uwo.ca>");
MODULE_DESCRIPTION!("Speakup userspace software synthesizer support");
MODULE_LICENSE!("GPL");
MODULE_VERSION!(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
