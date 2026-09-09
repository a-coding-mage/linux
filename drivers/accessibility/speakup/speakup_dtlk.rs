// SPDX-License-Identifier: GPL-2.0+
/*
 * originally written by: Kirk Reiser <kirk@braille.uwo.ca>
 * this version considerably modified by David Borowski, david575@rogers.com
 *
 * Copyright (C) 1998-99  Kirk Reiser.
 * Copyright (C) 2003 David Borowski.
 *
 * specifically written as a driver for the speakup screenreview
 * package it's not a general device driver.
 * This driver is for the RC Systems DoubleTalk PC internal synthesizer.
 */

// Kernel dependencies supplied by the surrounding translation unit.

const DRV_VERSION: &str = "2.10";
const PROCSPEECH: u8 = 0x00;

static mut synth_lpc: i32 = 0;
static mut port_forced: i32 = 0;
static mut synth_portlist: [u32; 7] = [0x25e, 0x29e, 0x2de, 0x31e, 0x35e, 0x39e, 0];
static mut synth_status: u8 = 0;

#[repr(C)]
enum default_vars_id {
    CAPS_START_ID = 0, CAPS_STOP_ID, RATE_ID, PITCH_ID,
    VOL_ID, TONE_ID, PUNCT_ID, VOICE_ID, FREQUENCY_ID,
    DIRECT_ID, V_LAST_VAR_ID, NB_ID,
}

// The following declarations retain the kernel driver's native data and attribute layout.
extern "C" {
    static mut vars: [var_t; NB_ID as usize];
    static mut synth_attrs: [*mut attribute; 15];
    static mut synth_dtlk: spk_synth;
}

static mut caps_start_attribute: kobj_attribute = kobj_attribute::new("caps_start", 0o644);
static mut caps_stop_attribute: kobj_attribute = kobj_attribute::new("caps_stop", 0o644);
static mut freq_attribute: kobj_attribute = kobj_attribute::new("freq", 0o644);
static mut pitch_attribute: kobj_attribute = kobj_attribute::new("pitch", 0o644);
static mut punct_attribute: kobj_attribute = kobj_attribute::new("punct", 0o644);
static mut rate_attribute: kobj_attribute = kobj_attribute::new("rate", 0o644);
static mut tone_attribute: kobj_attribute = kobj_attribute::new("tone", 0o644);
static mut voice_attribute: kobj_attribute = kobj_attribute::new("voice", 0o644);
static mut vol_attribute: kobj_attribute = kobj_attribute::new("vol", 0o644);
static mut delay_time_attribute: kobj_attribute = kobj_attribute::new("delay_time", 0o644);
static mut direct_attribute: kobj_attribute = kobj_attribute::new("direct", 0o644);
static mut full_time_attribute: kobj_attribute = kobj_attribute::new("full_time", 0o644);
static mut jiffy_delta_attribute: kobj_attribute = kobj_attribute::new("jiffy_delta", 0o644);
static mut trigger_time_attribute: kobj_attribute = kobj_attribute::new("trigger_time", 0o644);

#[inline]
unsafe fn synth_readable() -> bool {
    synth_status = inb_p(speakup_info.port_tts + UART_RX);
    (synth_status & TTS_READABLE) != 0
}

#[inline]
unsafe fn synth_writable() -> bool {
    synth_status = inb_p(speakup_info.port_tts + UART_RX);
    (synth_status & TTS_WRITABLE) != 0
}

#[inline]
unsafe fn synth_full() -> bool {
    synth_status = inb_p(speakup_info.port_tts + UART_RX);
    (synth_status & TTS_ALMOST_FULL) != 0
}

unsafe fn spk_out(ch: u8) {
    let mut timeout = SPK_XMITR_TIMEOUT;
    while !synth_writable() {
        timeout -= 1;
        if timeout == 0 { break; }
        udelay(1);
    }
    outb_p(ch, speakup_info.port_tts);
    timeout = SPK_XMITR_TIMEOUT;
    while synth_writable() {
        timeout -= 1;
        if timeout == 0 { break; }
        udelay(1);
    }
}

unsafe fn do_catch_up(synth: *mut spk_synth) {
    let mut ch: u8;
    let mut flags: c_ulong = 0;
    let jiffy_delta = spk_get_var(JIFFY);
    let delay_time = spk_get_var(DELAY);
    spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags);
    let mut jiffy_delta_val = (*jiffy_delta).u.n.value;
    spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
    let mut jiff_max = jiffies + jiffy_delta_val as c_ulong;
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
        set_current_state(TASK_INTERRUPTIBLE);
        let mut delay_time_val = (*delay_time).u.n.value;
        spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
        if synth_full() {
            schedule_timeout(msecs_to_jiffies(delay_time_val));
            continue;
        }
        set_current_state(TASK_RUNNING);
        spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags);
        ch = synth_buffer_getc();
        spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
        if ch == b'\n' { ch = PROCSPEECH; }
        spk_out(ch);
        if time_after_eq(jiffies, jiff_max) && ch == SPACE {
            spk_out(PROCSPEECH);
            spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags);
            delay_time_val = (*delay_time).u.n.value;
            jiffy_delta_val = (*jiffy_delta).u.n.value;
            spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
            schedule_timeout(msecs_to_jiffies(delay_time_val));
            jiff_max = jiffies + jiffy_delta_val as c_ulong;
        }
    }
    spk_out(PROCSPEECH);
}

unsafe fn synth_immediate(synth: *mut spk_synth, mut buf: *const i8) -> *const i8 {
    let mut ch: u8;
    while { ch = *(buf as *const u8); ch != 0 } {
        if synth_full() { return buf; }
        if ch == b'\n' { ch = PROCSPEECH; }
        spk_out(ch);
        buf = buf.add(1);
    }
    core::ptr::null()
}

unsafe fn synth_flush(_synth: *mut spk_synth) {
    outb_p(SYNTH_CLEAR, speakup_info.port_tts);
    while synth_writable() { cpu_relax(); }
}

unsafe fn synth_read_tts() -> i8 {
    while !synth_readable() { cpu_relax(); }
    let ch = synth_status & 0x7f;
    outb_p(ch, speakup_info.port_tts);
    while synth_readable() { cpu_relax(); }
    ch as i8
}

unsafe fn synth_probe(synth: *mut spk_synth) -> i32 {
    let mut port_val: u16 = 0;
    let mut i = 0;
    pr_info!("Probing for DoubleTalk.\n");
    if port_forced != 0 {
        speakup_info.port_tts = port_forced as _;
        pr_info!("probe forced to %x by kernel command line\n", speakup_info.port_tts);
        if (port_forced & 0xf) != 0xf { pr_info!("warning: port base should probably end with f\n"); }
        if synth_request_region(speakup_info.port_tts - 1, SYNTH_IO_EXTENT) != 0 {
            pr_warn!("sorry, port already reserved\n");
            return -EBUSY;
        }
        port_val = inw(speakup_info.port_tts - 1);
        synth_lpc = speakup_info.port_tts as i32 - 1;
    } else {
        while synth_portlist[i] != 0 {
            if synth_request_region(synth_portlist[i], SYNTH_IO_EXTENT) != 0 { i += 1; continue; }
            port_val = inw(synth_portlist[i] as _) & 0xfbff;
            if port_val == 0x107f {
                synth_lpc = synth_portlist[i] as i32;
                speakup_info.port_tts = (synth_lpc + 1) as _;
                break;
            }
            synth_release_region(synth_portlist[i], SYNTH_IO_EXTENT);
            i += 1;
        }
    }
    port_val &= 0xfbff;
    if port_val != 0x107f {
        pr_info!("DoubleTalk PC: not found\n");
        if synth_lpc != 0 { synth_release_region(synth_lpc as _, SYNTH_IO_EXTENT); }
        return -ENODEV;
    }
    while inw_p(synth_lpc as _) != 0x147f { cpu_relax(); }
    let sp = synth_interrogate(synth);
    pr_info!("%s: %03x-%03x, ROM ver %s, s/n %u, driver: %s\n", (*synth).long_name,
        synth_lpc, synth_lpc + SYNTH_IO_EXTENT - 1, (*sp).rom_version,
        (*sp).serial_number, (*synth).version);
    (*synth).alive = 1;
    0
}

unsafe fn dtlk_release(_synth: *mut spk_synth) {
    spk_stop_serial_interrupt();
    if speakup_info.port_tts != 0 { synth_release_region(speakup_info.port_tts - 1, SYNTH_IO_EXTENT); }
    speakup_info.port_tts = 0;
}

/* interrogate the DoubleTalk PC and return its settings */
unsafe fn synth_interrogate(synth: *mut spk_synth) -> *mut synth_settings {
    static mut buf: [i8; core::mem::size_of::<synth_settings>() + 1] = [0; core::mem::size_of::<synth_settings>() + 1];
    static mut status: synth_settings = synth_settings::zeroed();
    synth_immediate(synth, b"\x18\x01?\0".as_ptr() as *const i8);
    let mut total = 0;
    for _ in 0..50 {
        buf[total] = synth_read_tts();
        if total > 2 && buf[total] as u8 == 0x7f { break; }
        if total < core::mem::size_of::<synth_settings>() { total += 1; }
    }
    let mut t = buf.as_ptr() as *const u8;
    status.serial_number = *t as u16 + (*t.add(1) as u16) * 256;
    t = t.add(2);
    let mut i = 0;
    while *t != b'\r' {
        status.rom_version[i] = *t;
        if i < status.rom_version.len() - 1 { i += 1; }
        t = t.add(1);
    }
    status.rom_version[i] = 0;
    t = t.add(1);
    status.mode = *t; t = t.add(1);
    status.punc_level = *t; t = t.add(1);
    status.formant_freq = *t; t = t.add(1);
    status.pitch = *t; t = t.add(1);
    status.speed = *t; t = t.add(1);
    status.volume = *t; t = t.add(1);
    status.tone = *t; t = t.add(1);
    status.expression = *t; t = t.add(1);
    status.ext_dict_loaded = *t; t = t.add(1);
    status.ext_dict_status = *t; t = t.add(1);
    status.free_ram = *t; t = t.add(1);
    status.articulation = *t; t = t.add(1);
    status.reverb = *t; t = t.add(1);
    status.eob = *t;
    &mut status
}

// Probe, release, module parameters, and registration are supplied by the kernel-facing bindings.
// Their declarations are intentionally retained as external symbols.
extern "C" {
    fn module_spk_synth(synth: spk_synth);
    fn spk_stop_serial_interrupt();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
