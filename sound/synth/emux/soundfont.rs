// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Soundfont generic routines.
 *	It is intended that these should be used by any driver that is willing
 *	to accept soundfont patches.
 *
 *  Copyright (C) 1999 Steve Ratcliffe
 *  Copyright (c) 1999-2000 Takashi Iwai <tiwai@suse.de>
 */
/*
 * Deal with reading in of a soundfont.  Code follows the OSS way
 * of doing things so that the old sfxload utility can be used.
 * Everything may change when there is an alsa way of doing things.
 */

// External dependencies from Linux kernel and ALSA sound subsystem
// #include <linux/uaccess.h>
// #include <linux/slab.h>
// #include <linux/export.h>
// #include <sound/core.h>
// #include <sound/soundfont.h>
// #include <sound/seq_oss_legacy.h>

use core::ffi::c_int;
use core::ffi::c_char;
use core::ffi::c_void;
use core::ptr;
use core::mem;

// External type definitions from kernel/ALSA
#[repr(C)]
pub struct snd_card {
    pub dev: *mut c_void,
    // ... other fields
}

#[repr(C)]
pub struct snd_util_memhdr {
    // ... opaque
}

#[repr(C)]
pub struct soundfont_patch_info {
    pub key: u32,
    pub type_: i32,
    pub optarg: i32,
    pub len: i32,
}

#[repr(C)]
pub struct soundfont_open_parm {
    pub type_: i32,
    pub name: [c_char; 64],
}

#[repr(C)]
pub struct soundfont_voice_map {
    pub map_bank: i32,
    pub map_instr: i32,
    pub map_key: i32,
    pub src_bank: i32,
    pub src_instr: i32,
    pub src_key: i32,
}

#[repr(C)]
pub struct soundfont_voice_rec_hdr {
    pub bank: i32,
    pub instr: i32,
    pub nvoices: i32,
    pub write_mode: i32,
}

#[repr(C)]
pub struct soundfont_voice_parm {
    pub moddelay: u16,
    pub modatkhld: u16,
    pub moddcysus: u16,
    pub modrelease: u16,
    pub voldelay: u16,
    pub volatkhld: u16,
    pub voldcysus: u16,
    pub volrelease: u16,
    pub lfo1delay: u16,
    pub lfo2delay: u16,
    pub cutoff: u8,
    // ... other fields
}

#[repr(C)]
pub struct soundfont_voice_info {
    pub sample: i32,
    pub rate_offset: i32,
    pub root: i32,
    pub tune: i32,
    pub low: i32,
    pub high: i32,
    pub vellow: i32,
    pub velhigh: i32,
    pub fixkey: i32,
    pub fixvel: i32,
    pub fixpan: i32,
    pub pan: i32,
    pub amplitude: i32,
    pub scaleTuning: i32,
    pub start: i32,
    pub end: i32,
    pub loopstart: i32,
    pub loopend: i32,
    pub sample_mode: i32,
    pub mode: i32,
    pub sf_id: i32,
    pub parm: soundfont_voice_parm,
    pub attenuation: i32,
}

#[repr(C)]
pub struct soundfont_sample_info {
    pub sample: i32,
    pub start: i32,
    pub end: i32,
    pub loopstart: i32,
    pub loopend: i32,
    pub size: i32,
    pub mode_flags: i32,
    pub dummy: i32,
    pub truesize: i32,
    pub sf_id: i32,
}

#[repr(C)]
pub struct patch_info {
    pub len: i32,
    pub mode: i32,
    pub loop_start: i32,
    pub loop_end: i32,
    pub base_freq: i32,
    pub base_note: i32,
    pub low_note: i32,
    pub high_note: i32,
    pub panning: i32,
    pub instr_no: i32,
    pub env_rate: [i32; 6],
    pub env_offset: [i32; 6],
    pub tremolo_rate: i32,
    pub tremolo_depth: i32,
    pub vibrato_rate: i32,
    pub vibrato_depth: i32,
}

#[repr(C)]
pub struct snd_sf_sample {
    pub next: *mut snd_sf_sample,
    pub v: soundfont_sample_info,
    pub counter: i32,
}

#[repr(C)]
pub struct snd_sf_zone {
    pub next: *mut snd_sf_zone,
    pub next_zone: *mut snd_sf_zone,
    pub next_instr: *mut snd_sf_zone,
    pub v: soundfont_voice_info,
    pub sample: *mut snd_sf_sample,
    pub bank: i32,
    pub instr: i32,
    pub mapped: i32,
    pub counter: i32,
}

#[repr(C)]
pub struct snd_soundfont {
    pub next: *mut snd_soundfont,
    pub id: i32,
    pub type_: i32,
    pub zones: *mut snd_sf_zone,
    pub samples: *mut snd_sf_sample,
    pub name: [c_char; 64],
}

#[repr(C)]
pub struct snd_sf_callback {
    pub private_data: *mut c_void,
    pub sample_new: Option<unsafe extern "C" fn(*mut c_void, *mut snd_sf_sample, *mut snd_util_memhdr, *const c_void, i32) -> i32>,
    pub sample_free: Option<unsafe extern "C" fn(*mut c_void, *mut snd_sf_sample, *mut snd_util_memhdr)>,
    pub sample_reset: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
pub struct snd_sf_list {
    pub fonts: *mut snd_soundfont,
    pub fonts_size: i32,
    pub zones: *mut snd_sf_zone,
    pub presets: [*mut snd_sf_zone; 256],
    pub mem_used: i32,
    pub memhdr: *mut snd_util_memhdr,
    pub callback: snd_sf_callback,
    pub presets_locked: i32,
    pub zone_counter: i32,
    pub sample_counter: i32,
    pub zone_locked: i32,
    pub sample_locked: i32,
    pub open_client: i32,
    pub currsf: *mut snd_soundfont,
    pub lock: i32,  // spinlock_t
    pub presets_mutex: i32,  // mutex_t
}

// Forward declarations of external kernel functions
extern "C" {
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> i32;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> i32;
    fn memset(s: *mut c_void, c: i32, n: usize) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut c_void, fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
}

// Constants
const SNDRV_OSS_SOUNDFONT_PATCH: u32 = 0x07;
const SNDRV_SFNT_OPEN_PATCH: i32 = 0x01;
const SNDRV_SFNT_LOAD_INFO: i32 = 0x02;
const SNDRV_SFNT_LOAD_DATA: i32 = 0x03;
const SNDRV_SFNT_CLOSE_PATCH: i32 = 0x04;
const SNDRV_SFNT_REPLACE_DATA: i32 = 0x05;
const SNDRV_SFNT_MAP_PRESET: i32 = 0x06;
const SNDRV_SFNT_PROBE_DATA: i32 = 0x07;
const SNDRV_SFNT_REMOVE_INFO: i32 = 0x08;
const SNDRV_SFNT_PAT_TYPE_GUS: i32 = 1;
const SNDRV_SFNT_PAT_TYPE_MAP: i32 = 2;
const SNDRV_SFNT_PAT_SHARED: i32 = 0x01;
const SNDRV_SFNT_PAT_LOCKED: i32 = 0x02;
const SNDRV_SFNT_PATCH_NAME_LEN: usize = 64;
const SNDRV_SFNT_WR_EXCLUSIVE: i32 = 0;
const SNDRV_SFNT_WR_REPLACE: i32 = 1;
const SNDRV_SFNT_MODE_INIT_PARM: i32 = 1;
const SNDRV_SFNT_SAMPLE_8BITS: i32 = 0x01;
const SNDRV_SFNT_SAMPLE_UNSIGNED: i32 = 0x02;
const SNDRV_SFNT_SAMPLE_NO_BLANK: i32 = 0x04;
const SNDRV_SFNT_SAMPLE_SINGLESHOT: i32 = 0x08;
const SNDRV_SFNT_SAMPLE_BIDIR_LOOP: i32 = 0x10;
const SNDRV_SFNT_SAMPLE_REVERSE_LOOP: i32 = 0x20;
const SNDRV_SFNT_MODE_LOOPING: i32 = 0x10;
const SF_MAX_INSTRUMENTS: i32 = 128;
const SF_MAX_PRESETS: i32 = 256;
const OFFSET_MSEC: i32 = 653117;
const OFFSET_ABSCENT: i32 = 851781;
const OFFSET_SAMPLERATE: i32 = 1011119;
const ABSCENT_RATIO: i32 = 1200;
const TIMECENT_RATIO: i32 = 1200;
const SAMPLERATE_RATIO: i32 = 4096;
const WAVE_16_BITS: i32 = 1;
const WAVE_UNSIGNED: i32 = 2;
const WAVE_LOOPING: i32 = 4;
const WAVE_BIDIR_LOOP: i32 = 8;
const WAVE_LOOP_BACK: i32 = 16;
const WAVE_ENVELOPES: i32 = 32;
const WAVE_FAST_RELEASE: i32 = 64;
const WAVE_TREMOLO: i32 = 128;
const WAVE_VIBRATO: i32 = 256;

fn SF_IS_DRUM_BANK(bank: i32) -> bool {
    (bank & 0x80) != 0
}

fn kzalloc_obj<T>() -> *mut T {
    unsafe {
        let size = mem::size_of::<T>();
        let ptr = libc::malloc(size) as *mut u8;
        if !ptr.is_null() {
            ptr::write_bytes(ptr, 0, size);
        }
        ptr as *mut T
    }
}

// Static function declarations
fn open_patch(sflist: *mut snd_sf_list, data: *const c_char, count: i32, client: i32) -> i32;
fn newsf(sflist: *mut snd_sf_list, type_: i32, name: *mut c_char) -> *mut snd_soundfont;
fn is_identical_font(sf: *mut snd_soundfont, type_: i32, name: *mut u8) -> i32;
fn close_patch(sflist: *mut snd_sf_list) -> i32;
fn probe_data(sflist: *mut snd_sf_list, sample_id: i32) -> i32;
fn set_zone_counter(sflist: *mut snd_sf_list, sf: *mut snd_soundfont, zp: *mut snd_sf_zone);
fn sf_zone_new(sflist: *mut snd_sf_list, sf: *mut snd_soundfont) -> *mut snd_sf_zone;
fn set_sample_counter(sflist: *mut snd_sf_list, sf: *mut snd_soundfont, sp: *mut snd_sf_sample);
fn sf_sample_new(sflist: *mut snd_sf_list, sf: *mut snd_soundfont) -> *mut snd_sf_sample;
fn sf_sample_delete(sflist: *mut snd_sf_list, sf: *mut snd_soundfont, sp: *mut snd_sf_sample);
fn load_map(sflist: *mut snd_sf_list, data: *const c_void, count: i32) -> i32;
fn load_info(card: *mut snd_card, sflist: *mut snd_sf_list, data: *const c_void, count: i64) -> i32;
fn remove_info(sflist: *mut snd_sf_list, sf: *mut snd_soundfont, bank: i32, instr: i32) -> i32;
fn init_voice_info(avp: *mut soundfont_voice_info);
fn init_voice_parm(pp: *mut soundfont_voice_parm);
fn set_sample(sf: *mut snd_soundfont, avp: *mut soundfont_voice_info) -> *mut snd_sf_sample;
fn find_sample(sf: *mut snd_soundfont, sample_id: i32) -> *mut snd_sf_sample;
fn load_data(sflist: *mut snd_sf_list, data: *const c_void, count: i64) -> i32;
fn rebuild_presets(sflist: *mut snd_sf_list);
fn add_preset(sflist: *mut snd_sf_list, cur: *mut snd_sf_zone);
fn delete_preset(sflist: *mut snd_sf_list, zp: *mut snd_sf_zone);
fn search_first_zone(sflist: *mut snd_sf_list, bank: i32, preset: i32, key: i32) -> *mut snd_sf_zone;
fn search_zones(sflist: *mut snd_sf_list, notep: *mut i32, vel: i32, preset: i32, bank: i32, table: *mut *mut snd_sf_zone, max_layers: i32, level: i32) -> i32;
fn get_index(bank: i32, instr: i32, key: i32) -> i32;
fn snd_sf_init(sflist: *mut snd_sf_list);
fn snd_sf_clear(sflist: *mut snd_sf_list);
fn load_guspatch(card: *mut snd_card, sflist: *mut snd_sf_list, data: *const c_char, count: i64) -> i32;
fn validate_sample_info(si: *mut soundfont_sample_info) -> i32;
fn calc_gus_envelope_time(rate: i32, start: i32, end: i32) -> i32;
fn freq_to_note(mhz: i32) -> i32;
fn calc_rate_offset(hz: i32) -> i32;
fn calc_parm_search(msec: i32, table: *const i16) -> i32;

// close the patch if the patch was opened by this client.
pub unsafe extern "C" fn snd_soundfont_close_check(sflist: *mut snd_sf_list, client: i32) -> i32 {
    // scoped_guard(spinlock_irqsave, &sflist->lock)
    if (*sflist).open_client != client {
        return 0;
    }
    close_patch(sflist)
}

// Deal with a soundfont patch. Any driver could use these routines
// although it was designed for the AWE64.
//
// The sample_write and callargs parameters allow a callback into
// the actual driver to write sample data to the board or whatever
// it wants to do with it.
pub unsafe extern "C" fn snd_soundfont_load(card: *mut snd_card,
                 sflist: *mut snd_sf_list,
                 data: *const c_void,
                 count: i64,
                 client: i32) -> i32 {
    let mut patch: soundfont_patch_info = mem::zeroed();
    let mut rc: i32;

    if count < mem::size_of::<soundfont_patch_info>() as i64 {
        dev_err((*card).dev, b"patch record too small %ld\n" as *const c_char as *const c_char, count);
        return -22;  // EINVAL
    }
    if copy_from_user(&mut patch as *mut _ as *mut c_void, data, mem::size_of::<soundfont_patch_info>()) != 0 {
        return -14;  // EFAULT
    }

    let data = (data as *const u8).add(mem::size_of::<soundfont_patch_info>()) as *const c_void;
    let mut count = count - mem::size_of::<soundfont_patch_info>() as i64;

    if patch.key != SNDRV_OSS_SOUNDFONT_PATCH {
        dev_err((*card).dev, b"The wrong kind of patch %x\n" as *const c_char as *const c_char, patch.key);
        return -22;  // EINVAL
    }
    if count < patch.len as i64 {
        dev_err((*card).dev, b"Patch too short %ld, need %d\n" as *const c_char as *const c_char, count, patch.len);
        return -22;  // EINVAL
    }
    if patch.len < 0 {
        dev_err((*card).dev, b"poor length %d\n" as *const c_char as *const c_char, patch.len);
        return -22;  // EINVAL
    }

    if patch.type_ == SNDRV_SFNT_OPEN_PATCH {
        // guard(snd_soundfont_lock_preset)(sflist)
        return open_patch(sflist, data as *const c_char, count as i32, client);
    }

    // check if other client already opened patch
    if (*sflist).open_client != client {
        return -16;  // EBUSY
    }

    // guard(snd_soundfont_lock_preset)(sflist)
    rc = -22;  // EINVAL
    match patch.type_ {
        SNDRV_SFNT_LOAD_INFO => {
            rc = load_info(card, sflist, data, count);
        }
        SNDRV_SFNT_LOAD_DATA => {
            rc = load_data(sflist, data, count);
        }
        SNDRV_SFNT_CLOSE_PATCH => {
            rc = close_patch(sflist);
        }
        SNDRV_SFNT_REPLACE_DATA => {
            // rc = replace_data(&patch, data, count);
        }
        SNDRV_SFNT_MAP_PRESET => {
            rc = load_map(sflist, data, count as i32);
        }
        SNDRV_SFNT_PROBE_DATA => {
            rc = probe_data(sflist, patch.optarg);
        }
        SNDRV_SFNT_REMOVE_INFO => {
            // patch must be opened
            if (*sflist).currsf.is_null() {
                dev_err((*card).dev, b"soundfont: remove_info: patch not opened\n" as *const c_char as *const c_char);
                rc = -22;  // EINVAL
            } else {
                let bank = ((patch.optarg as u16) >> 8) as i32 & 0xff;
                let instr = (patch.optarg as u16) as i32 & 0xff;
                if remove_info(sflist, (*sflist).currsf, bank, instr) == 0 {
                    rc = -22;  // EINVAL
                } else {
                    rc = 0;
                }
            }
        }
        _ => {}
    }

    rc
}

// check if specified type is special font (GUS or preset-alias)
#[inline]
fn is_special_type(type_: i32) -> i32 {
    let type_ = type_ & 0x0f;
    if type_ == SNDRV_SFNT_PAT_TYPE_GUS || type_ == SNDRV_SFNT_PAT_TYPE_MAP {
        1
    } else {
        0
    }
}

// open patch; create sf list
unsafe fn open_patch(sflist: *mut snd_sf_list, data: *const c_char, count: i32, client: i32) -> i32 {
    let mut parm: soundfont_open_parm = mem::zeroed();
    let mut sf: *mut snd_soundfont;

    if (*sflist).open_client >= 0 || !(*sflist).currsf.is_null() {
        return -16;  // EBUSY
    }

    if copy_from_user(&mut parm as *mut _ as *mut c_void, data as *const c_void, mem::size_of::<soundfont_open_parm>()) != 0 {
        return -14;  // EFAULT
    }

    if is_special_type(parm.type_) != 0 {
        parm.type_ |= SNDRV_SFNT_PAT_SHARED;
        sf = newsf(sflist, parm.type_, ptr::null_mut());
    } else {
        sf = newsf(sflist, parm.type_, parm.name.as_mut_ptr());
    }
    if sf.is_null() {
        return -12;  // ENOMEM
    }

    (*sflist).open_client = client;
    (*sflist).currsf = sf;

    0
}

// Allocate a new soundfont structure.
unsafe fn newsf(sflist: *mut snd_sf_list, type_: i32, name: *mut c_char) -> *mut snd_soundfont {
    let mut sf: *mut snd_soundfont;

    // check the shared fonts
    if (type_ & SNDRV_SFNT_PAT_SHARED) != 0 {
        let mut curr_sf = (*sflist).fonts;
        while !curr_sf.is_null() {
            if is_identical_font(curr_sf, type_, name as *mut u8) != 0 {
                return curr_sf;
            }
            curr_sf = (*curr_sf).next;
        }
    }

    // not found -- create a new one
    sf = kzalloc_obj();
    if sf.is_null() {
        return ptr::null_mut();
    }
    (*sf).id = (*sflist).fonts_size;
    (*sflist).fonts_size += 1;

    // prepend this record
    (*sf).next = (*sflist).fonts;
    (*sflist).fonts = sf;

    (*sf).type_ = type_;
    (*sf).zones = ptr::null_mut();
    (*sf).samples = ptr::null_mut();
    if !name.is_null() {
        memcpy((*sf).name.as_mut_ptr() as *mut c_void, name as *const c_void, SNDRV_SFNT_PATCH_NAME_LEN);
    }

    sf
}

// check if the given name matches to the existing list
unsafe fn is_identical_font(sf: *mut snd_soundfont, type_: i32, name: *mut u8) -> i32 {
    if (((*sf).type_ & SNDRV_SFNT_PAT_SHARED) != 0) &&
       (((*sf).type_ & 0x0f) == (type_ & 0x0f)) &&
       (name.is_null() || memcmp((*sf).name.as_ptr() as *const c_void, name as *const c_void, SNDRV_SFNT_PATCH_NAME_LEN) == 0) {
        1
    } else {
        0
    }
}

// Close the current patch.
unsafe fn close_patch(sflist: *mut snd_sf_list) -> i32 {
    (*sflist).currsf = ptr::null_mut();
    (*sflist).open_client = -1;

    rebuild_presets(sflist);

    0
}

// probe sample in the current list -- nothing to be loaded
unsafe fn probe_data(sflist: *mut snd_sf_list, sample_id: i32) -> i32 {
    // patch must be opened
    if !(*sflist).currsf.is_null() {
        // search the specified sample by optarg
        if !find_sample((*sflist).currsf, sample_id).is_null() {
            return 0;
        }
    }
    -22  // EINVAL
}

// increment zone counter
unsafe fn set_zone_counter(sflist: *mut snd_sf_list, sf: *mut snd_soundfont, zp: *mut snd_sf_zone) {
    (*zp).counter = (*sflist).zone_counter;
    (*sflist).zone_counter += 1;
    if ((*sf).type_ & SNDRV_SFNT_PAT_LOCKED) != 0 {
        (*sflist).zone_locked = (*sflist).zone_counter;
    }
}

// allocate a new zone record
unsafe fn sf_zone_new(sflist: *mut snd_sf_list, sf: *mut snd_soundfont) -> *mut snd_sf_zone {
    let mut zp: *mut snd_sf_zone;

    zp = kzalloc_obj();
    if zp.is_null() {
        return ptr::null_mut();
    }
    (*zp).next = (*sf).zones;
    (*sf).zones = zp;

    init_voice_info(&mut (*zp).v);

    set_zone_counter(sflist, sf, zp);
    zp
}

// increment sample counter
unsafe fn set_sample_counter(sflist: *mut snd_sf_list, sf: *mut snd_soundfont, sp: *mut snd_sf_sample) {
    (*sp).counter = (*sflist).sample_counter;
    (*sflist).sample_counter += 1;
    if ((*sf).type_ & SNDRV_SFNT_PAT_LOCKED) != 0 {
        (*sflist).sample_locked = (*sflist).sample_counter;
    }
}

// allocate a new sample list record
unsafe fn sf_sample_new(sflist: *mut snd_sf_list, sf: *mut snd_soundfont) -> *mut snd_sf_sample {
    let mut sp: *mut snd_sf_sample;

    sp = kzalloc_obj();
    if sp.is_null() {
        return ptr::null_mut();
    }

    (*sp).next = (*sf).samples;
    (*sf).samples = sp;

    set_sample_counter(sflist, sf, sp);
    sp
}

// delete sample list -- this is an exceptional job.
// only the last allocated sample can be deleted.
unsafe fn sf_sample_delete(sflist: *mut snd_sf_list, sf: *mut snd_soundfont, sp: *mut snd_sf_sample) {
    // only last sample is accepted
    if sp == (*sf).samples {
        (*sf).samples = (*sp).next;
        kfree(sp as *mut c_void);
    }
}

// load voice map
unsafe fn load_map(sflist: *mut snd_sf_list, data: *const c_void, count: i32) -> i32 {
    let mut zp: *mut snd_sf_zone;
    let mut prevp: *mut snd_sf_zone;
    let mut sf: *mut snd_soundfont;
    let mut map: soundfont_voice_map = mem::zeroed();

    // get the link info
    if count < mem::size_of::<soundfont_voice_map>() as i32 {
        return -22;  // EINVAL
    }
    if copy_from_user(&mut map as *mut _ as *mut c_void, data, mem::size_of::<soundfont_voice_map>()) != 0 {
        return -14;  // EFAULT
    }

    if map.map_instr < 0 || map.map_instr >= SF_MAX_INSTRUMENTS {
        return -22;  // EINVAL
    }

    sf = newsf(sflist, SNDRV_SFNT_PAT_TYPE_MAP | SNDRV_SFNT_PAT_SHARED, ptr::null_mut());
    if sf.is_null() {
        return -12;  // ENOMEM
    }

    prevp = ptr::null_mut();
    zp = (*sf).zones;
    while !zp.is_null() {
        if (*zp).mapped != 0 &&
           (*zp).instr == map.map_instr &&
           (*zp).bank == map.map_bank &&
           (*zp).v.low == map.map_key &&
           (*zp).v.start == map.src_instr &&
           (*zp).v.end == map.src_bank &&
           (*zp).v.fixkey == map.src_key {
            // the same mapping is already present
            // relink this record to the link head
            if !prevp.is_null() {
                (*prevp).next = (*zp).next;
                (*zp).next = (*sf).zones;
                (*sf).zones = zp;
            }
            // update the counter
            set_zone_counter(sflist, sf, zp);
            return 0;
        }
        prevp = zp;
        zp = (*zp).next;
    }

    // create a new zone
    zp = sf_zone_new(sflist, sf);
    if zp.is_null() {
        return -12;  // ENOMEM
    }

    (*zp).bank = map.map_bank;
    (*zp).instr = map.map_instr;
    (*zp).mapped = 1;
    if map.map_key >= 0 {
        (*zp).v.low = map.map_key;
        (*zp).v.high = map.map_key;
    }
    (*zp).v.start = map.src_instr;
    (*zp).v.end = map.src_bank;
    (*zp).v.fixkey = map.src_key;
    (*zp).v.sf_id = (*sf).id;

    add_preset(sflist, zp);

    0
}

// remove the present instrument layers
unsafe fn remove_info(sflist: *mut snd_sf_list, sf: *mut snd_soundfont, bank: i32, instr: i32) -> i32 {
    let mut prev: *mut snd_sf_zone = ptr::null_mut();
    let mut next: *mut snd_sf_zone;
    let mut p: *mut snd_sf_zone;
    let mut removed: i32 = 0;

    p = (*sf).zones;
    while !p.is_null() {
        next = (*p).next;
        if (*p).mapped == 0 &&
           (*p).bank == bank && (*p).instr == instr {
            // remove this layer
            if !prev.is_null() {
                (*prev).next = next;
            } else {
                (*sf).zones = next;
            }
            removed += 1;
            kfree(p as *mut c_void);
        } else {
            prev = p;
        }
        p = next;
    }
    if removed != 0 {
        rebuild_presets(sflist);
    }
    removed
}

// Read an info record from the user buffer and save it on the current
// open soundfont.
unsafe fn load_info(card: *mut snd_card, sflist: *mut snd_sf_list, data: *const c_void, count: i64) -> i32 {
    let mut sf: *mut snd_soundfont;
    let mut zone: *mut snd_sf_zone;
    let mut hdr: soundfont_voice_rec_hdr = mem::zeroed();
    let mut i: i32;

    // patch must be opened
    sf = (*sflist).currsf;
    if sf.is_null() {
        return -22;  // EINVAL
    }

    if is_special_type((*sf).type_) != 0 {
        return -22;  // EINVAL
    }

    if count < mem::size_of::<soundfont_voice_rec_hdr>() as i64 {
        dev_err((*card).dev, b"Soundfont error: invalid patch zone length\n" as *const c_char as *const c_char);
        return -22;  // EINVAL
    }
    if copy_from_user(&mut hdr as *mut _ as *mut c_void, data, mem::size_of::<soundfont_voice_rec_hdr>()) != 0 {
        return -14;  // EFAULT
    }

    let data = (data as *const u8).add(mem::size_of::<soundfont_voice_rec_hdr>()) as *const c_void;
    let mut count = count - mem::size_of::<soundfont_voice_rec_hdr>() as i64;

    if hdr.nvoices <= 0 || hdr.nvoices >= 100 {
        dev_err((*card).dev, b"Soundfont error: Illegal voice number %d\n" as *const c_char as *const c_char, hdr.nvoices);
        return -22;  // EINVAL
    }

    if count < (mem::size_of::<soundfont_voice_info>() as i64 * hdr.nvoices as i64) {
        dev_err((*card).dev,
            b"Soundfont Error: patch length(%ld) is smaller than nvoices(%d)\n" as *const c_char as *const c_char,
            count, hdr.nvoices);
        return -22;  // EINVAL
    }

    match hdr.write_mode {
        SNDRV_SFNT_WR_EXCLUSIVE => {
            // exclusive mode - if the instrument already exists, return error
            zone = (*sf).zones;
            while !zone.is_null() {
                if (*zone).mapped == 0 &&
                   (*zone).bank == hdr.bank &&
                   (*zone).instr == hdr.instr {
                    return -22;  // EINVAL
                }
                zone = (*zone).next;
            }
        }
        SNDRV_SFNT_WR_REPLACE => {
            // replace mode - remove the instrument if it already exists
            remove_info(sflist, sf, hdr.bank, hdr.instr);
        }
        _ => {}
    }

    i = 0;
    while i < hdr.nvoices {
        let mut tmpzone: snd_sf_zone = mem::zeroed();

        // copy awe_voice_info parameters
        if copy_from_user(&mut tmpzone.v as *mut _ as *mut c_void, data, mem::size_of::<soundfont_voice_info>()) != 0 {
            return -14;  // EFAULT
        }

        let data = (data as *const u8).add(mem::size_of::<soundfont_voice_info>()) as *const c_void;
        count -= mem::size_of::<soundfont_voice_info>() as i64;

        tmpzone.bank = hdr.bank;
        tmpzone.instr = hdr.instr;
        tmpzone.mapped = 0;
        tmpzone.v.sf_id = (*sf).id;
        if (tmpzone.v.mode & SNDRV_SFNT_MODE_INIT_PARM) != 0 {
            init_voice_parm(&mut tmpzone.v.parm);
        }

        // create a new zone
        zone = sf_zone_new(sflist, sf);
        if zone.is_null() {
            return -12;  // ENOMEM
        }

        // copy the temporary data
        (*zone).bank = tmpzone.bank;
        (*zone).instr = tmpzone.instr;
        (*zone).v = tmpzone.v;

        // look up the sample
        (*zone).sample = set_sample(sf, &mut (*zone).v);

        i += 1;
    }

    0
}

// initialize voice_info record
unsafe fn init_voice_info(avp: *mut soundfont_voice_info) {
    memset(avp as *mut c_void, 0, mem::size_of::<soundfont_voice_info>());

    (*avp).root = 60;
    (*avp).high = 127;
    (*avp).velhigh = 127;
    (*avp).fixkey = -1;
    (*avp).fixvel = -1;
    (*avp).fixpan = -1;
    (*avp).pan = -1;
    (*avp).amplitude = 127;
    (*avp).scaleTuning = 100;

    init_voice_parm(&mut (*avp).parm);
}

// initialize voice_parm record:
// Env1/2: delay=0, attack=0, hold=0, sustain=0, decay=0, release=0.
// Vibrato and Tremolo effects are zero.
// Cutoff is maximum.
// Chorus and Reverb effects are zero.
unsafe fn init_voice_parm(pp: *mut soundfont_voice_parm) {
    memset(pp as *mut c_void, 0, mem::size_of::<soundfont_voice_parm>());

    (*pp).moddelay = 0x8000;
    (*pp).modatkhld = 0x7f7f;
    (*pp).moddcysus = 0x7f7f;
    (*pp).modrelease = 0x807f;

    (*pp).voldelay = 0x8000;
    (*pp).volatkhld = 0x7f7f;
    (*pp).voldcysus = 0x7f7f;
    (*pp).volrelease = 0x807f;

    (*pp).lfo1delay = 0x8000;
    (*pp).lfo2delay = 0x8000;

    (*pp).cutoff = 0xff;
}

// search the specified sample
unsafe fn set_sample(sf: *mut snd_soundfont, avp: *mut soundfont_voice_info) -> *mut snd_sf_sample {
    let mut sample: *mut snd_sf_sample;

    sample = find_sample(sf, (*avp).sample);
    if sample.is_null() {
        return ptr::null_mut();
    }

    // add in the actual sample offsets:
    // The voice_info addresses define only the relative offset
    // from sample pointers.  Here we calculate the actual DRAM
    // offset from sample pointers.
    (*avp).start = (*avp).start.wrapping_add((*sample).v.start);
    (*avp).end = (*avp).end.wrapping_add((*sample).v.end);
    (*avp).loopstart = (*avp).loopstart.wrapping_add((*sample).v.loopstart);
    (*avp).loopend = (*avp).loopend.wrapping_add((*sample).v.loopend);

    // copy mode flags
    (*avp).sample_mode = (*sample).v.mode_flags;

    sample
}

// find the sample pointer with the given id in the soundfont
unsafe fn find_sample(sf: *mut snd_soundfont, sample_id: i32) -> *mut snd_sf_sample {
    let mut p: *mut snd_sf_sample;

    if sf.is_null() {
        return ptr::null_mut();
    }

    p = (*sf).samples;
    while !p.is_null() {
        if (*p).v.sample == sample_id {
            return p;
        }
        p = (*p).next;
    }
    ptr::null_mut()
}

unsafe fn validate_sample_info(si: *mut soundfont_sample_info) -> i32 {
    if (*si).end < 0 || (*si).end > (*si).size {
        return -22;  // EINVAL
    }
    if (*si).loopstart < 0 || (*si).loopstart > (*si).end {
        return -22;  // EINVAL
    }
    if (*si).loopend < 0 || (*si).loopend > (*si).end {
        return -22;  // EINVAL
    }
    // be sure loop points start < end
    if (*si).loopstart > (*si).loopend {
        let tmp = (*si).loopstart;
        (*si).loopstart = (*si).loopend;
        (*si).loopend = tmp;
    }
    0
}

// Load sample information, this can include data to be loaded onto
// the soundcard.  It can also just be a pointer into soundcard ROM.
// If there is data it will be written to the soundcard via the callback
// routine.
unsafe fn load_data(sflist: *mut snd_sf_list, data: *const c_void, count: i64) -> i32 {
    let mut sf: *mut snd_soundfont;
    let mut sample_info: soundfont_sample_info = mem::zeroed();
    let mut sp: *mut snd_sf_sample;

    // patch must be opened
    sf = (*sflist).currsf;
    if sf.is_null() {
        return -22;  // EINVAL
    }

    if is_special_type((*sf).type_) != 0 {
        return -22;  // EINVAL
    }

    if count < mem::size_of::<soundfont_sample_info>() as i64 {
        return -22;  // EINVAL
    }
    if copy_from_user(&mut sample_info as *mut _ as *mut c_void, data, mem::size_of::<soundfont_sample_info>()) != 0 {
        return -14;  // EFAULT
    }
    let data = (data as *const u8).add(mem::size_of::<soundfont_sample_info>()) as *const c_void;
    let mut count = count - mem::size_of::<soundfont_sample_info>() as i64;

    // SoundFont uses S16LE samples.
    if (sample_info.size as i64) * 2 != count {
        return -22;  // EINVAL
    }

    // Check for dup
    if !find_sample(sf, sample_info.sample).is_null() {
        // if shared sample, skip this data
        if ((*sf).type_ & SNDRV_SFNT_PAT_SHARED) != 0 {
            return 0;
        }
        return -22;  // EINVAL
    }

    if sample_info.size > 0 {
        if sample_info.start < 0 {
            return -22;  // EINVAL
        }

        // Here we "rebase out" the start address, because the
        // real start is the start of the provided sample data.
        sample_info.end -= sample_info.start;
        sample_info.loopstart -= sample_info.start;
        sample_info.loopend -= sample_info.start;
        sample_info.start = 0;

        if validate_sample_info(&mut sample_info) < 0 {
            return -22;  // EINVAL
        }
    }

    // Allocate a new sample structure
    sp = sf_sample_new(sflist, sf);
    if sp.is_null() {
        return -12;  // ENOMEM
    }

    (*sp).v = sample_info;
    (*sp).v.sf_id = (*sf).id;
    (*sp).v.dummy = 0;
    (*sp).v.truesize = 0;

    // If there is wave data then load it.
    if (*sp).v.size > 0 {
        let mut rc: i32;
        if let Some(sample_new) = (*sflist).callback.sample_new {
            rc = sample_new((*sflist).callback.private_data, sp, (*sflist).memhdr, data, count as i32);
            if rc < 0 {
                sf_sample_delete(sflist, sf, sp);
                return rc;
            }
            (*sflist).mem_used += (*sp).v.truesize;
        }
    }

    count as i32
}

// log2_tbl[i] = log2(i+128) * 0x10000
static log_tbl: [i32; 129] = [
    0x70000, 0x702df, 0x705b9, 0x7088e, 0x70b5d, 0x70e26, 0x710eb, 0x713aa,
    0x71663, 0x71918, 0x71bc8, 0x71e72, 0x72118, 0x723b9, 0x72655, 0x728ed,
    0x72b80, 0x72e0e, 0x73098, 0x7331d, 0x7359e, 0x7381b, 0x73a93, 0x73d08,
    0x73f78, 0x741e4, 0x7444c, 0x746b0, 0x74910, 0x74b6c, 0x74dc4, 0x75019,
    0x75269, 0x754b6, 0x75700, 0x75946, 0x75b88, 0x75dc7, 0x76002, 0x7623a,
    0x7646e, 0x766a0, 0x768cd, 0x76af8, 0x76d1f, 0x76f43, 0x77164, 0x77382,
    0x7759d, 0x777b4, 0x779c9, 0x77bdb, 0x77dea, 0x77ff5, 0x781fe, 0x78404,
    0x78608, 0x78808, 0x78a06, 0x78c01, 0x78df9, 0x78fef, 0x791e2, 0x793d2,
    0x795c0, 0x797ab, 0x79993, 0x79b79, 0x79d5d, 0x79f3e, 0x7a11d, 0x7a2f9,
    0x7a4d3, 0x7a6ab, 0x7a880, 0x7aa53, 0x7ac24, 0x7adf2, 0x7afbe, 0x7b188,
    0x7b350, 0x7b515, 0x7b6d8, 0x7b899, 0x7ba58, 0x7bc15, 0x7bdd0, 0x7bf89,
    0x7c140, 0x7c2f5, 0x7c4a7, 0x7c658, 0x7c807, 0x7c9b3, 0x7cb5e, 0x7cd07,
    0x7ceae, 0x7d053, 0x7d1f7, 0x7d398, 0x7d538, 0x7d6d6, 0x7d872, 0x7da0c,
    0x7dba4, 0x7dd3b, 0x7ded0, 0x7e063, 0x7e1f4, 0x7e384, 0x7e512, 0x7e69f,
    0x7e829, 0x7e9b3, 0x7eb3a, 0x7ecc0, 0x7ee44, 0x7efc7, 0x7f148, 0x7f2c8,
    0x7f446, 0x7f5c2, 0x7f73d, 0x7f8b7, 0x7fa2f, 0x7fba5, 0x7fd1a, 0x7fe8d,
    0x80000,
];

// convert from linear to log value
//
// conversion: value = log2(amount / base) * ratio
//
// argument:
//   amount = linear value (unsigned, 32bit max)
//   offset = base offset (:= log2(base) * 0x10000)
//   ratio = division ratio
//
pub extern "C" fn snd_sf_linear_to_log(mut amount: u32, offset: i32, ratio: i32) -> i32 {
    let v: i32;
    let mut s: i32;
    let mut low: i32;
    let mut bit: i32 = 0;

    if amount < 2 {
        return 0;
    }
    while (amount & 0x80000000) == 0 {
        bit += 1;
        amount <<= 1;
    }
    s = ((amount >> 24) & 0x7f) as i32;
    low = ((amount >> 16) & 0xff) as i32;
    // linear approximation by lower 8 bit
    let v = ((log_tbl[(s + 1) as usize] as i64 * low as i64 + log_tbl[s as usize] as i64 * (0x100 - low) as i64) >> 8) as i32;
    let v = v - offset;
    let v = (v as i64 * ratio as i64) >> 16;
    let v = v + (24 - bit) as i64 * ratio as i64;
    v as i32
}

// mHz to abscent
// conversion: abscent = log2(MHz / 8176) * 1200
unsafe fn freq_to_note(mhz: i32) -> i32 {
    snd_sf_linear_to_log(mhz as u32, OFFSET_ABSCENT, ABSCENT_RATIO)
}

// convert Hz to AWE32 rate offset:
// sample pitch offset for the specified sample rate
// rate=44100 is no offset, each 4096 is 1 octave (twice).
// eg, when rate is 22050, this offset becomes -4096.
//
// conversion: offset = log2(Hz / 44100) * 4096
unsafe fn calc_rate_offset(hz: i32) -> i32 {
    snd_sf_linear_to_log(hz as u32, OFFSET_SAMPLERATE, SAMPLERATE_RATIO)
}

// calculate GUS envelope time
unsafe fn calc_gus_envelope_time(rate: i32, start: i32, end: i32) -> i32 {
    let r = (3 - ((rate >> 6) & 3)) * 3;
    let mut p = rate & 0x3f;
    if p == 0 {
        p = 1;
    }
    let mut t = end - start;
    if t < 0 {
        t = -t;
    }
    if 13 > r {
        t = t << (13 - r);
    } else {
        t = t >> (r - 13);
    }
    (t * 10) / (p * 441)
}

// attack & decay/release time table (msec)
static attack_time_tbl: [i16; 128] = [
    32767, 32767, 5989, 4235, 2994, 2518, 2117, 1780, 1497, 1373, 1259, 1154, 1058, 970, 890, 816,
    707, 691, 662, 634, 607, 581, 557, 533, 510, 489, 468, 448, 429, 411, 393, 377,
    361, 345, 331, 317, 303, 290, 278, 266, 255, 244, 234, 224, 214, 205, 196, 188,
    180, 172, 165, 158, 151, 145, 139, 133, 127, 122, 117, 112, 107, 102, 98, 94,
    90, 86, 82, 79, 75, 72, 69, 66, 63, 61, 58, 56, 53, 51, 49, 47,
    45, 43, 41, 39, 37, 36, 34, 33, 31, 30, 29, 28, 26, 25, 24, 23,
    22, 21, 20, 19, 19, 18, 17, 16, 16, 15, 15, 14, 13, 13, 12, 12,
    11, 11, 10, 10, 10, 9, 9, 8, 8, 8, 8, 7, 7, 7, 6, 0,
];

static decay_time_tbl: [i16; 128] = [
    32767, 32767, 22614, 15990, 11307, 9508, 7995, 6723, 5653, 5184, 4754, 4359, 3997, 3665, 3361, 3082,
    2828, 2765, 2648, 2535, 2428, 2325, 2226, 2132, 2042, 1955, 1872, 1793, 1717, 1644, 1574, 1507,
    1443, 1382, 1324, 1267, 1214, 1162, 1113, 1066, 978, 936, 897, 859, 822, 787, 754, 722,
    691, 662, 634, 607, 581, 557, 533, 510, 489, 468, 448, 429, 411, 393, 377, 361,
    345, 331, 317, 303, 290, 278, 266, 255, 244, 234, 224, 214, 205, 196, 188, 180,
    172, 165, 158, 151, 145, 139, 133, 127, 122, 117, 112, 107, 102, 98, 94, 90,
    86, 82, 79, 75, 72, 69, 66, 63, 61, 58, 56, 53, 51, 49, 47, 45,
    43, 41, 39, 37, 36, 34, 33, 31, 30, 29, 28, 26, 25, 24, 23, 22,
];

// delay time = 0x8000 - msec/92
pub extern "C" fn snd_sf_calc_parm_hold(msec: i32) -> i32 {
    let mut val = (0x7f * 92 - msec) / 92;
    if val < 1 {
        val = 1;
    }
    if val >= 126 {
        val = 126;
    }
    val
}

// search an index for specified time from given time table
unsafe fn calc_parm_search(msec: i32, table: *const i16) -> i32 {
    let mut left = 1;
    let mut right = 127;
    while left < right {
        let mid = (left + right) / 2;
        if msec < (*table.add(mid as usize)) as i32 {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

// attack time: search from time table
pub extern "C" fn snd_sf_calc_parm_attack(msec: i32) -> i32 {
    unsafe { calc_parm_search(msec, attack_time_tbl.as_ptr()) }
}

// decay/release time: search from time table
pub extern "C" fn snd_sf_calc_parm_decay(msec: i32) -> i32 {
    unsafe { calc_parm_search(msec, decay_time_tbl.as_ptr()) }
}

pub static snd_sf_vol_table: [i32; 128] = [
    255, 111, 95, 86, 79, 74, 70, 66, 63, 61, 58, 56, 54, 52, 50, 49,
    47, 46, 45, 43, 42, 41, 40, 39, 38, 37, 36, 35, 34, 34, 33, 32,
    31, 31, 30, 29, 29, 28, 27, 27, 26, 26, 25, 24, 24, 23, 23, 22,
    22, 21, 21, 21, 20, 20, 19, 19, 18, 18, 18, 17, 17, 16, 16, 16,
    15, 15, 15, 14, 14, 14, 13, 13, 13, 12, 12, 12, 11, 11, 11, 10,
    10, 10, 10, 9, 9, 9, 8, 8, 8, 8, 7, 7, 7, 7, 6, 6,
    6, 6, 5, 5, 5, 5, 5, 4, 4, 4, 4, 3, 3, 3, 3, 3,
    2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0,
];

fn calc_gus_sustain(val: i32) -> i32 {
    0x7f - snd_sf_vol_table[(val / 2) as usize]
}

fn calc_gus_attenuation(val: i32) -> i32 {
    snd_sf_vol_table[(val / 2) as usize]
}

// load GUS patch
unsafe fn load_guspatch(card: *mut snd_card, sflist: *mut snd_sf_list, data: *const c_char, count: i64) -> i32 {
    let mut patch: patch_info = mem::zeroed();
    let mut sf: *mut snd_soundfont;
    let mut zone: *mut snd_sf_zone;
    let mut smp: *mut snd_sf_sample;
    let mut note: i32;
    let mut sample_id: i32;
    let mut rc: i32;

    if count < mem::size_of::<patch_info>() as i64 {
        dev_err((*card).dev, b"patch record too small %ld\n" as *const c_char as *const c_char, count);
        return -22;  // EINVAL
    }
    if copy_from_user(&mut patch as *mut _ as *mut c_void, data as *const c_void, mem::size_of::<patch_info>()) != 0 {
        return -14;  // EFAULT
    }
    let data = (data as *const u8).add(mem::size_of::<patch_info>()) as *const c_char;
    let mut count = count - mem::size_of::<patch_info>() as i64;

    let shift = if (patch.mode & WAVE_16_BITS) != 0 { 1 } else { 0 };
    if (patch.len << shift) != count as i32 {
        return -22;  // EINVAL
    }

    sf = newsf(sflist, SNDRV_SFNT_PAT_TYPE_GUS | SNDRV_SFNT_PAT_SHARED, ptr::null_mut());
    if sf.is_null() {
        return -12;  // ENOMEM
    }
    smp = sf_sample_new(sflist, sf);
    if smp.is_null() {
        return -12;  // ENOMEM
    }
    sample_id = (*sflist).sample_counter;
    (*smp).v.sample = sample_id;
    (*smp).v.start = 0;
    (*smp).v.end = patch.len;
    (*smp).v.loopstart = patch.loop_start;
    (*smp).v.loopend = patch.loop_end;
    (*smp).v.size = patch.len;

    if validate_sample_info(&mut (*smp).v) < 0 {
        sf_sample_delete(sflist, sf, smp);
        return -22;  // EINVAL
    }

    // set up mode flags
    (*smp).v.mode_flags = 0;
    if (patch.mode & WAVE_16_BITS) == 0 {
        (*smp).v.mode_flags |= SNDRV_SFNT_SAMPLE_8BITS;
    }
    if (patch.mode & WAVE_UNSIGNED) != 0 {
        (*smp).v.mode_flags |= SNDRV_SFNT_SAMPLE_UNSIGNED;
    }
    (*smp).v.mode_flags |= SNDRV_SFNT_SAMPLE_NO_BLANK;
    if (patch.mode & (WAVE_LOOPING | WAVE_BIDIR_LOOP | WAVE_LOOP_BACK)) == 0 {
        (*smp).v.mode_flags |= SNDRV_SFNT_SAMPLE_SINGLESHOT;
    }
    if (patch.mode & WAVE_BIDIR_LOOP) != 0 {
        (*smp).v.mode_flags |= SNDRV_SFNT_SAMPLE_BIDIR_LOOP;
    }
    if (patch.mode & WAVE_LOOP_BACK) != 0 {
        (*smp).v.mode_flags |= SNDRV_SFNT_SAMPLE_REVERSE_LOOP;
    }

    if (patch.mode & WAVE_16_BITS) != 0 {
        // convert to word offsets
        (*smp).v.size /= 2;
        (*smp).v.end /= 2;
        (*smp).v.loopstart /= 2;
        (*smp).v.loopend /= 2;
    }

    (*smp).v.dummy = 0;
    (*smp).v.truesize = 0;
    (*smp).v.sf_id = (*sf).id;

    // set up voice info
    zone = sf_zone_new(sflist, sf);
    if zone.is_null() {
        sf_sample_delete(sflist, sf, smp);
        return -12;  // ENOMEM
    }

    // load wave data
    if (*smp).v.size > 0 {
        if let Some(sample_new) = (*sflist).callback.sample_new {
            rc = sample_new((*sflist).callback.private_data, smp, (*sflist).memhdr, data as *const c_void, count as i32);
            if rc < 0 {
                sf_sample_delete(sflist, sf, smp);
                kfree(zone as *mut c_void);
                return rc;
            }
        }
    }

    // update the memory offset here
    (*sflist).mem_used += (*smp).v.truesize;

    (*zone).v.sample = sample_id;  // the last sample
    (*zone).v.rate_offset = calc_rate_offset(patch.base_freq);
    note = freq_to_note(patch.base_note);
    (*zone).v.root = note / 100;
    (*zone).v.tune = -(note % 100);
    (*zone).v.low = (freq_to_note(patch.low_note) + 99) / 100;
    (*zone).v.high = freq_to_note(patch.high_note) / 100;
    // panning position; -128 - 127 => 0-127
    (*zone).v.pan = (patch.panning + 128) / 2;

    // detuning is ignored
    // 6points volume envelope
    if (patch.mode & WAVE_ENVELOPES) != 0 {
        let attack = calc_gus_envelope_time(patch.env_rate[0], 0, patch.env_offset[0]);
        let hold = calc_gus_envelope_time(patch.env_rate[1], patch.env_offset[0], patch.env_offset[1]);
        let decay = calc_gus_envelope_time(patch.env_rate[2], patch.env_offset[1], patch.env_offset[2]);
        let release = calc_gus_envelope_time(patch.env_rate[3], patch.env_offset[1], patch.env_offset[4])
            + calc_gus_envelope_time(patch.env_rate[4], patch.env_offset[3], patch.env_offset[4])
            + calc_gus_envelope_time(patch.env_rate[5], patch.env_offset[4], patch.env_offset[5]);
        (*zone).v.parm.volatkhld = ((snd_sf_calc_parm_hold(hold) as u16) << 8) | (snd_sf_calc_parm_attack(attack) as u16);
        (*zone).v.parm.voldcysus = ((calc_gus_sustain(patch.env_offset[2]) as u16) << 8) | (snd_sf_calc_parm_decay(decay) as u16);
        (*zone).v.parm.volrelease = 0x8000 | (snd_sf_calc_parm_decay(release) as u16);
        (*zone).v.attenuation = calc_gus_attenuation(patch.env_offset[0]);
    }

    // fast release
    if (patch.mode & WAVE_FAST_RELEASE) != 0 {
        (*zone).v.parm.volrelease = 0x807f;
    }

    // tremolo effect
    if (patch.mode & WAVE_TREMOLO) != 0 {
        let rate = ((patch.tremolo_rate * 1000 / 38) / 42) as u8;
        (*zone).v.parm.tremfrq = ((patch.tremolo_depth / 2) as u16) << 8 | rate as u16;
    }
    // vibrato effect
    if (patch.mode & WAVE_VIBRATO) != 0 {
        let rate = ((patch.vibrato_rate * 1000 / 38) / 42) as u8;
        (*zone).v.parm.fm2frq2 = ((patch.vibrato_depth / 6) as u16) << 8 | rate as u16;
    }

    if ((*smp).v.mode_flags & SNDRV_SFNT_SAMPLE_SINGLESHOT) == 0 {
        (*zone).v.mode = SNDRV_SFNT_MODE_LOOPING;
    } else {
        (*zone).v.mode = 0;
    }

    (*zone).bank = 0;
    (*zone).instr = patch.instr_no;
    (*zone).mapped = 0;
    (*zone).v.sf_id = (*sf).id;

    (*zone).sample = set_sample(sf, &mut (*zone).v);

    // rebuild preset now
    add_preset(sflist, zone);

    0
}

// load GUS patch
pub unsafe extern "C" fn snd_soundfont_load_guspatch(card: *mut snd_card,
                     sflist: *mut snd_sf_list,
                     data: *const c_char,
                     count: i64) -> i32 {
    // guard(snd_soundfont_lock_preset)(sflist)
    load_guspatch(card, sflist, data, count)
}

// Rebuild the preset table.  This is like a hash table in that it allows
// quick access to the zone information.  For each preset there are zone
// structures linked by next_instr and by next_zone.  Former is the whole
// link for this preset, and latter is the link for zone (i.e. instrument/
// bank/key combination).
unsafe fn rebuild_presets(sflist: *mut snd_sf_list) {
    let mut sf: *mut snd_soundfont;
    let mut cur: *mut snd_sf_zone;

    // clear preset table
    memset((*sflist).presets.as_mut_ptr() as *mut c_void, 0, mem::size_of_val(&(*sflist).presets));

    // search all fonts and insert each font
    sf = (*sflist).fonts;
    while !sf.is_null() {
        cur = (*sf).zones;
        while !cur.is_null() {
            if (*cur).mapped == 0 && (*cur).sample.is_null() {
                // try again to search the corresponding sample
                (*cur).sample = set_sample(sf, &mut (*cur).v);
                if (*cur).sample.is_null() {
                    cur = (*cur).next;
                    continue;
                }
            }

            add_preset(sflist, cur);
            cur = (*cur).next;
        }
        sf = (*sf).next;
    }
}

// add the given zone to preset table
unsafe fn add_preset(sflist: *mut snd_sf_list, cur: *mut snd_sf_zone) {
    let mut zone: *mut snd_sf_zone;
    let index: i32;

    zone = search_first_zone(sflist, (*cur).bank, (*cur).instr, (*cur).v.low);
    if !zone.is_null() && (*zone).v.sf_id != (*cur).v.sf_id {
        // different instrument was already defined
        let mut p: *mut snd_sf_zone;
        // compare the allocated time
        p = zone;
        while !p.is_null() {
            if (*p).counter > (*cur).counter {
                // the current is older.. skipped
                return;
            }
            p = (*p).next_zone;
        }
        // remove old zones
        delete_preset(sflist, zone);
        zone = ptr::null_mut();  // do not forget to clear this!
    }

    // prepend this zone
    index = get_index((*cur).bank, (*cur).instr, (*cur).v.low);
    if index < 0 {
        return;
    }
    (*cur).next_zone = zone;  // zone link
    (*cur).next_instr = (*sflist).presets[index as usize];  // preset table link
    (*sflist).presets[index as usize] = cur;
}

// delete the given zones from preset_table
unsafe fn delete_preset(sflist: *mut snd_sf_list, zp: *mut snd_sf_zone) {
    let index: i32;
    let mut p: *mut snd_sf_zone;

    index = get_index((*zp).bank, (*zp).instr, (*zp).v.low);
    if index < 0 {
        return;
    }
    p = (*sflist).presets[index as usize];
    while !p.is_null() {
        while (*p).next_instr == zp {
            (*p).next_instr = (*zp).next_instr;
            let mut new_zp = (*zp).next_zone;
            if new_zp.is_null() {
                return;
            }
            // Continue with next_zone for outer loop
            p = (*sflist).presets[index as usize];
            zp = new_zp;
            loop {
                if (*p).next_instr == zp {
                    continue;
                }
                break;
            }
        }
        p = (*p).next_instr;
    }
}

// Search matching zones from preset table.
// The note can be rewritten by preset mapping (alias).
// The found zones are stored on 'table' array.  max_layers defines
// the maximum number of elements in this array.
// This function returns the number of found zones.  0 if not found.
pub unsafe extern "C" fn snd_soundfont_search_zone(sflist: *mut snd_sf_list,
              notep: *mut i32,
              vel: i32,
              preset: i32,
              bank: i32,
              def_preset: i32,
              def_bank: i32,
              table: *mut *mut snd_sf_zone,
              max_layers: i32) -> i32 {
    let nvoices: i32;

    // this function is supposed to be called atomically,
    // so we check the lock.  if it's busy, just returns 0 to
    // tell the caller the busy state
    if (*sflist).presets_locked != 0 {
        return 0;
    }
    nvoices = search_zones(sflist, notep, vel, preset, bank, table, max_layers, 0);
    if nvoices == 0 {
        if preset != def_preset || bank != def_bank {
            return search_zones(sflist, notep, vel, def_preset, def_bank, table, max_layers, 0);
        }
    }
    nvoices
}

// search the first matching zone
unsafe fn search_first_zone(sflist: *mut snd_sf_list, bank: i32, preset: i32, key: i32) -> *mut snd_sf_zone {
    let index: i32;
    let mut zp: *mut snd_sf_zone;

    index = get_index(bank, preset, key);
    if index < 0 {
        return ptr::null_mut();
    }
    zp = (*sflist).presets[index as usize];
    while !zp.is_null() {
        if (*zp).instr == preset && (*zp).bank == bank {
            return zp;
        }
        zp = (*zp).next_instr;
    }
    ptr::null_mut()
}

// search matching zones from sflist.  can be called recursively.
unsafe fn search_zones(sflist: *mut snd_sf_list, notep: *mut i32, vel: i32,
         preset: i32, bank: i32, table: *mut *mut snd_sf_zone,
         max_layers: i32, level: i32) -> i32 {
    let mut zp: *mut snd_sf_zone;
    let mut nvoices: i32;

    zp = search_first_zone(sflist, bank, preset, *notep);
    nvoices = 0;
    while !zp.is_null() {
        if *notep >= (*zp).v.low && *notep <= (*zp).v.high &&
           vel >= (*zp).v.vellow && vel <= (*zp).v.velhigh {
            if (*zp).mapped != 0 {
                // search preset mapping (aliasing)
                let mut key = (*zp).v.fixkey;
                let new_preset = (*zp).v.start;
                let new_bank = (*zp).v.end;

                if level > 5 {
                    // too deep alias level
                    return 0;
                }
                if key < 0 {
                    key = *notep;
                }
                nvoices = search_zones(sflist, &mut key, vel, new_preset, new_bank, table, max_layers, level + 1);
                if nvoices > 0 {
                    *notep = key;
                }
                break;
            }
            *table.add(nvoices as usize) = zp;
            nvoices += 1;
            if nvoices >= max_layers {
                break;
            }
        }
        zp = (*zp).next_zone;
    }

    nvoices
}

// calculate the index of preset table:
// drums are mapped from 128 to 255 according to its note key.
// other instruments are mapped from 0 to 127.
// if the index is out of range, return -1.
unsafe fn get_index(bank: i32, instr: i32, key: i32) -> i32 {
    let index: i32;
    if SF_IS_DRUM_BANK(bank) {
        index = key + SF_MAX_INSTRUMENTS;
    } else {
        index = instr;
    }
    let index = index % SF_MAX_PRESETS;
    if index < 0 {
        return -1;
    }
    index
}

// Initialise the sflist structure.
unsafe fn snd_sf_init(sflist: *mut snd_sf_list) {
    memset((*sflist).presets.as_mut_ptr() as *mut c_void, 0, mem::size_of_val(&(*sflist).presets));

    (*sflist).mem_used = 0;
    (*sflist).currsf = ptr::null_mut();
    (*sflist).open_client = -1;
    (*sflist).fonts = ptr::null_mut();
    (*sflist).fonts_size = 0;
    (*sflist).zone_counter = 0;
    (*sflist).sample_counter = 0;
    (*sflist).zone_locked = 0;
    (*sflist).sample_locked = 0;
}

// Release all list records
unsafe fn snd_sf_clear(sflist: *mut snd_sf_list) {
    let mut sf: *mut snd_soundfont;
    let mut nextsf: *mut snd_soundfont;
    let mut zp: *mut snd_sf_zone;
    let mut nextzp: *mut snd_sf_zone;
    let mut sp: *mut snd_sf_sample;
    let mut nextsp: *mut snd_sf_sample;

    sf = (*sflist).fonts;
    while !sf.is_null() {
        nextsf = (*sf).next;
        zp = (*sf).zones;
        while !zp.is_null() {
            nextzp = (*zp).next;
            kfree(zp as *mut c_void);
            zp = nextzp;
        }
        sp = (*sf).samples;
        while !sp.is_null() {
            nextsp = (*sp).next;
            if let Some(sample_free) = (*sflist).callback.sample_free {
                sample_free((*sflist).callback.private_data, sp, (*sflist).memhdr);
            }
            kfree(sp as *mut c_void);
            sp = nextsp;
        }
        kfree(sf as *mut c_void);
        sf = nextsf;
    }

    snd_sf_init(sflist);
}

// Create a new sflist structure
pub unsafe extern "C" fn snd_sf_new(callback: *mut snd_sf_callback, hdr: *mut snd_util_memhdr) -> *mut snd_sf_list {
    let mut sflist: *mut snd_sf_list;

    sflist = kzalloc_obj();
    if sflist.is_null() {
        return ptr::null_mut();
    }

    // mutex_init(&sflist->presets_mutex);
    // spin_lock_init(&sflist->lock);
    (*sflist).memhdr = hdr;

    if !callback.is_null() {
        (*sflist).callback = *callback;
    }

    snd_sf_init(sflist);
    sflist
}

// Free everything allocated off the sflist structure.
pub unsafe extern "C" fn snd_sf_free(sflist: *mut snd_sf_list) {
    if sflist.is_null() {
        return;
    }

    // guard(snd_soundfont_lock_preset, sflist)
    if let Some(sample_reset) = (*sflist).callback.sample_reset {
        sample_reset((*sflist).callback.private_data);
    }
    snd_sf_clear(sflist);

    kfree(sflist as *mut c_void);
}

// Remove all samples
// The soundcard should be silent before calling this function.
pub unsafe extern "C" fn snd_soundfont_remove_samples(sflist: *mut snd_sf_list) -> i32 {
    // guard(snd_soundfont_lock_preset)(sflist)
    if let Some(sample_reset) = (*sflist).callback.sample_reset {
        sample_reset((*sflist).callback.private_data);
    }
    snd_sf_clear(sflist);

    0
}

// Remove unlocked samples.
// The soundcard should be silent before calling this function.
pub unsafe extern "C" fn snd_soundfont_remove_unlocked(sflist: *mut snd_sf_list) -> i32 {
    let mut sf: *mut snd_soundfont;
    let mut zp: *mut snd_sf_zone;
    let mut nextzp: *mut snd_sf_zone;
    let mut sp: *mut snd_sf_sample;
    let mut nextsp: *mut snd_sf_sample;

    // guard(snd_soundfont_lock_preset)(sflist)

    if let Some(sample_reset) = (*sflist).callback.sample_reset {
        sample_reset((*sflist).callback.private_data);
    }

    // to be sure
    memset((*sflist).presets.as_mut_ptr() as *mut c_void, 0, mem::size_of_val(&(*sflist).presets));

    sf = (*sflist).fonts;
    while !sf.is_null() {
        zp = (*sf).zones;
        while !zp.is_null() {
            if (*zp).counter < (*sflist).zone_locked {
                break;
            }
            nextzp = (*zp).next;
            (*sf).zones = nextzp;
            kfree(zp as *mut c_void);
            zp = nextzp;
        }

        sp = (*sf).samples;
        while !sp.is_null() {
            if (*sp).counter < (*sflist).sample_locked {
                break;
            }
            nextsp = (*sp).next;
            (*sf).samples = nextsp;
            (*sflist).mem_used -= (*sp).v.truesize;
            if let Some(sample_free) = (*sflist).callback.sample_free {
                sample_free((*sflist).callback.private_data, sp, (*sflist).memhdr);
            }
            kfree(sp as *mut c_void);
            sp = nextsp;
        }
        sf = (*sf).next;
    }

    (*sflist).zone_counter = (*sflist).zone_locked;
    (*sflist).sample_counter = (*sflist).sample_locked;

    rebuild_presets(sflist);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
