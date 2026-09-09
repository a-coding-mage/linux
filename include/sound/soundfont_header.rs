/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of soundfont.h. */

use core::ffi::c_void;

pub const SF_MAX_INSTRUMENTS: usize = 128;
pub const SF_MAX_PRESETS: usize = 256;

#[inline]
pub const fn SF_IS_DRUM_BANK(z: i32) -> bool {
    z == 128
}

#[repr(C)]
pub struct snd_sf_zone {
    pub next: *mut snd_sf_zone,
    pub bank: u8,
    pub instr: u8,
    pub mapped: u8,
    pub v: soundfont_voice_info,
    pub counter: i32,
    pub sample: *mut snd_sf_sample,
    pub next_instr: *mut snd_sf_zone,
    pub next_zone: *mut snd_sf_zone,
}

#[repr(C)]
pub struct snd_sf_sample {
    pub v: soundfont_sample_info,
    pub counter: i32,
    pub block: *mut snd_util_memblk,
    pub next: *mut snd_sf_sample,
}

#[repr(C)]
pub struct snd_soundfont {
    pub next: *mut snd_soundfont,
    pub id: i16,
    pub r#type: i16,
    pub name: [u8; SNDRV_SFNT_PATCH_NAME_LEN],
    pub zones: *mut snd_sf_zone,
    pub samples: *mut snd_sf_sample,
}

#[repr(C)]
pub struct snd_sf_callback {
    pub private_data: *mut c_void,
    pub sample_new: Option<unsafe extern "C" fn(
        private_data: *mut c_void,
        sp: *mut snd_sf_sample,
        hdr: *mut snd_util_memhdr,
        buf: *const c_void,
        count: isize,
    ) -> i32>,
    pub sample_free: Option<unsafe extern "C" fn(
        private_data: *mut c_void,
        sp: *mut snd_sf_sample,
        hdr: *mut snd_util_memhdr,
    ) -> i32>,
    pub sample_reset: Option<unsafe extern "C" fn(private_data: *mut c_void)>,
}

#[repr(C)]
pub struct snd_sf_list {
    pub currsf: *mut snd_soundfont,
    pub open_client: i32,
    pub mem_used: i32,
    pub presets: [*mut snd_sf_zone; SF_MAX_PRESETS],
    pub fonts: *mut snd_soundfont,
    pub fonts_size: i32,
    pub zone_counter: i32,
    pub sample_counter: i32,
    pub zone_locked: i32,
    pub sample_locked: i32,
    pub callback: snd_sf_callback,
    pub presets_locked: i32,
    pub presets_mutex: mutex,
    pub lock: spinlock_t,
    pub memhdr: *mut snd_util_memhdr,
}

extern "C" {
    pub fn snd_soundfont_load(
        card: *mut snd_card,
        sflist: *mut snd_sf_list,
        data: *const c_void,
        count: isize,
        client: i32,
    ) -> i32;
    pub fn snd_soundfont_load_guspatch(
        card: *mut snd_card,
        sflist: *mut snd_sf_list,
        data: *const i8,
        count: isize,
    ) -> i32;
    pub fn snd_soundfont_close_check(sflist: *mut snd_sf_list, client: i32) -> i32;
    pub fn snd_sf_new(callback: *mut snd_sf_callback, hdr: *mut snd_util_memhdr) -> *mut snd_sf_list;
    pub fn snd_sf_free(sflist: *mut snd_sf_list);
    pub fn snd_soundfont_remove_samples(sflist: *mut snd_sf_list) -> i32;
    pub fn snd_soundfont_remove_unlocked(sflist: *mut snd_sf_list) -> i32;
    pub fn snd_soundfont_search_zone(
        sflist: *mut snd_sf_list,
        notep: *mut i32,
        vel: i32,
        preset: i32,
        bank: i32,
        def_preset: i32,
        def_bank: i32,
        table: *mut *mut snd_sf_zone,
        max_layers: i32,
    ) -> i32;
    pub fn snd_sf_calc_parm_hold(msec: i32) -> i32;
    pub fn snd_sf_calc_parm_attack(msec: i32) -> i32;
    pub fn snd_sf_calc_parm_decay(msec: i32) -> i32;
    pub static mut snd_sf_vol_table: [i32; 128];
    pub fn snd_sf_linear_to_log(amount: u32, offset: i32, ratio: i32) -> i32;
    pub fn mutex_lock(lock: *mut mutex);
    pub fn mutex_unlock(lock: *mut mutex);
    pub fn spin_lock_irqsave(lock: *mut spinlock_t) -> usize;
    pub fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
}

#[inline]
pub const fn snd_sf_calc_parm_delay(msec: i32) -> i32 {
    0x8000i32.wrapping_sub(msec.wrapping_mul(1000) / 725)
}

#[inline]
pub unsafe fn snd_soundfont_lock_preset(sflist: *mut snd_sf_list) {
    mutex_lock(&mut (*sflist).presets_mutex);
    // C guard(spinlock_irqsave) holds the IRQ-save spinlock until function exit.
    spin_lock_irqsave(&mut (*sflist).lock);
    (*sflist).presets_locked = 1;
}

#[inline]
pub unsafe fn snd_soundfont_unlock_preset(sflist: *mut snd_sf_list) {
    // The matching C guard releases the IRQ-save spinlock here.
    spin_unlock_irqrestore(&mut (*sflist).lock, 0);
    (*sflist).presets_locked = 0;
    mutex_unlock(&mut (*sflist).presets_mutex);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
