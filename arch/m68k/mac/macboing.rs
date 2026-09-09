// SPDX-License-Identifier: GPL-2.0
/*
 * Mac bong noise generator. Note - we ought to put a boingy noise
 * here 8)
 *
 * Rust translation of the original implementation.
 */

// Symbols and types supplied by the surrounding kernel and Macintosh code.
use core::ptr;

extern "C" {
    static mut macintosh_config: *mut mac_config;
    static mut jiffies: c_ulong;
    fn local_irq_save(flags: *mut c_ulong);
    fn local_irq_restore(flags: c_ulong);
    fn timer_delete(timer: *mut timer_list);
    fn add_timer(timer: *mut timer_list);
}

type c_ulong = usize;
type c_uint = u32;
type c_int = i32;
type u8_ = u8;
type u32_ = u32;

#[repr(C)]
struct mac_config { ident: c_int }

#[repr(C)]
struct timer_list {
    expires: c_ulong,
    function: Option<unsafe extern "C" fn(*mut timer_list)>,
}

const MAC_MODEL_IIFX: c_int = 0;
const MAC_MODEL_Q630: c_int = 1;
const MAC_MODEL_P475: c_int = 2;
const MAC_MODEL_C660: c_int = 3;
const MAC_MODEL_Q840: c_int = 4;
const MAC_MODEL_Q650: c_int = 5;
const MAC_MODEL_Q700: c_int = 6;
const MAC_MODEL_Q800: c_int = 7;
const MAC_MODEL_Q900: c_int = 8;
const MAC_MODEL_Q950: c_int = 9;
const ASC_CONTROL: usize = 0;
const ASC_VOLUME: usize = 0;
const ASC_MODE: usize = 0;
const ASC_ENABLE: usize = 0;
const ASC_MODE_SAMPLE: u8 = 0;
const ASC_ENABLE_SAMPLE: u8 = 0;

static mut mac_asc_inited: c_int = 0;
static mut mac_asc_wave_tab: [u8_; 0x800] = [0; 0x800];
static mut mac_asc_regs: *mut u8 = 0x50F14000 as *mut u8;
static mut mac_asc_samplespersec: c_ulong = 11050;
static mut mac_bell_duration: c_int = 0;
static mut mac_bell_phase: c_ulong = 0;
static mut mac_bell_phasepersample: c_ulong = 0;
static mut mac_special_bell: Option<unsafe extern "C" fn(c_uint, c_uint, c_uint)> = None;
static mut mac_sound_timer: timer_list = timer_list { expires: 0, function: Some(mac_nosound) };

unsafe extern "C" fn mac_init_asc() {
    let mut i: c_int;
    match (*macintosh_config).ident {
        MAC_MODEL_IIFX => mac_asc_regs = 0x50010000 as *mut u8,
        MAC_MODEL_Q630 | MAC_MODEL_P475 => {
            mac_special_bell = Some(mac_quadra_start_bell);
            mac_asc_samplespersec = 22150;
        }
        MAC_MODEL_C660 | MAC_MODEL_Q840 => mac_special_bell = Some(mac_av_start_bell),
        MAC_MODEL_Q650 | MAC_MODEL_Q700 | MAC_MODEL_Q800 | MAC_MODEL_Q900 | MAC_MODEL_Q950 => {
            mac_special_bell = None;
        }
        _ => mac_special_bell = None,
    }
    i = 0;
    while i < 0x400 {
        mac_asc_wave_tab[i as usize] = (i / 4) as u8;
        mac_asc_wave_tab[(i + 0x400) as usize] = (0xFF - i / 4) as u8;
        i += 1;
    }
    mac_asc_inited = 1;
}

pub unsafe extern "C" fn mac_mksound(freq: c_uint, length: c_uint) {
    let cfreq: u32_ = (freq << 5) / 468;
    let mut flags: c_ulong = 0;
    if mac_special_bell.is_none() { return; }
    if mac_asc_inited == 0 { mac_init_asc(); }
    if let Some(f) = mac_special_bell { f(freq, length, 128); return; }
    if freq < 20 || freq > 20000 || length == 0 { mac_nosound(ptr::null_mut()); return; }
    local_irq_save(&mut flags);
    timer_delete(&mut mac_sound_timer);
    for i in 0..0x800 { *mac_asc_regs.add(i) = 0; }
    for i in 0..0x800 { *mac_asc_regs.add(i) = mac_asc_wave_tab[i]; }
    for i in 0..8 {
        *(mac_asc_regs.add(ASC_CONTROL + 0x814 + 8 * i) as *mut u32) = cfreq;
    }
    *mac_asc_regs.add(0x807) = 0;
    *mac_asc_regs.add(ASC_VOLUME) = 128;
    *mac_asc_regs.add(0x805) = 0;
    *mac_asc_regs.add(0x80F) = 0;
    *mac_asc_regs.add(ASC_MODE) = ASC_MODE_SAMPLE;
    *mac_asc_regs.add(ASC_ENABLE) = ASC_ENABLE_SAMPLE;
    mac_sound_timer.expires = jiffies + length as c_ulong;
    add_timer(&mut mac_sound_timer);
    local_irq_restore(flags);
}

unsafe extern "C" fn mac_nosound(_unused: *mut timer_list) {
    *mac_asc_regs.add(ASC_ENABLE) = 0;
}

unsafe extern "C" fn mac_quadra_start_bell(freq: c_uint, length: c_uint, volume: c_uint) {
    let mut flags: c_ulong = 0;
    if mac_bell_duration > 0 { mac_bell_duration += length as c_int; return; }
    mac_bell_duration = length as c_int;
    mac_bell_phase = 0;
    mac_bell_phasepersample = (freq as c_ulong * core::mem::size_of::<[u8_; 0x800]>()) / mac_asc_samplespersec;
    local_irq_save(&mut flags);
    *mac_asc_regs.add(0x806) = volume as u8;
    if *mac_asc_regs.add(0x801) != 1 {
        *mac_asc_regs.add(0x807) = 0;
        *mac_asc_regs.add(0x802) = 0;
        *mac_asc_regs.add(0x801) = 1;
        *mac_asc_regs.add(0x803) |= 0x80;
        *mac_asc_regs.add(0x803) &= 0x7F;
    }
    mac_sound_timer.function = Some(mac_quadra_ring_bell);
    mac_sound_timer.expires = jiffies + 1;
    add_timer(&mut mac_sound_timer);
    local_irq_restore(flags);
}

unsafe extern "C" fn mac_quadra_ring_bell(_unused: *mut timer_list) {
    let count = mac_asc_samplespersec / 100;
    let mut flags: c_ulong = 0;
    local_irq_save(&mut flags);
    timer_delete(&mut mac_sound_timer);
    if { let old = mac_bell_duration; mac_bell_duration -= 1; old > 0 } {
        for _ in 0..count {
            mac_bell_phase += mac_bell_phasepersample;
            *mac_asc_regs = mac_asc_wave_tab[mac_bell_phase & (core::mem::size_of::<[u8_; 0x800]>() - 1)];
        }
        mac_sound_timer.expires = jiffies + 1;
        add_timer(&mut mac_sound_timer);
    } else { *mac_asc_regs.add(0x801) = 0; }
    local_irq_restore(flags);
}

unsafe extern "C" fn mac_av_start_bell(_freq: c_uint, _length: c_uint, _volume: c_uint) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
