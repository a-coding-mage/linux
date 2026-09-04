// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (C) 2000 Takashi Iwai <tiwai@suse.de>
 *
 *  Proc interface for Emu8k/Emu10k1 WaveTable synth
 */

// External types from kernel and audio subsystems
// use linux::wait::*;
// use sound::core::*;
// use sound::emux_synth::*;
// use sound::info::*;
// use self::emux_voice::*;

unsafe extern "C" {
    fn snd_iprintf(buf: *mut snd_info_buffer, fmt: *const i8, ...) -> ();
    fn snd_info_create_card_entry(card: *mut snd_card, name: *const i8, parent: *mut snd_info_entry) -> *mut snd_info_entry;
    fn snd_info_free_entry(entry: *mut snd_info_entry) -> ();
    fn snd_util_mem_avail(hdr: *mut snd_util_memhdr) -> i32;
}

#[repr(C)]
pub struct snd_emux {
    pub name: *const i8,
    pub num_ports: i32,
    pub client: i32,
    pub ports: *mut i32,
    pub used: i32,
    pub max_voices: i32,
    pub num_voices: i32,
    pub memhdr: *mut snd_util_memhdr,
    pub sflist: *mut snd_sf_list,
    pub proc: *mut snd_info_entry,
    pub register_mutex: (),
}

#[repr(C)]
pub struct snd_info_entry {
    pub private_data: *mut core::ffi::c_void,
    pub content: i32,
    pub c: snd_info_entry_c,
}

#[repr(C)]
pub union snd_info_entry_c {
    pub text: snd_info_entry_text,
}

#[repr(C)]
pub struct snd_info_entry_text {
    pub read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer) -> ()>,
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub proc_root: *mut snd_info_entry,
}

#[repr(C)]
pub struct snd_util_memhdr {
    pub size: i32,
    pub nblocks: i32,
}

#[repr(C)]
pub struct snd_sf_list {
    pub fonts_size: i32,
    pub zone_counter: i32,
    pub sample_counter: i32,
    pub zone_locked: i32,
    pub sample_locked: i32,
    pub presets_mutex: (),
}

unsafe fn snd_emux_proc_info_read(entry: *mut snd_info_entry, buf: *mut snd_info_buffer) {
    let emu: *mut snd_emux = (*entry).private_data as *mut snd_emux;
    let mut i: i32;

    // guard(mutex)(&emu->register_mutex);
    if !(*emu).name.is_null() {
        snd_iprintf(buf, b"Device: %s\n\0".as_ptr() as *const i8, (*emu).name);
    }
    snd_iprintf(buf, b"Ports: %d\n\0".as_ptr() as *const i8, (*emu).num_ports);
    snd_iprintf(buf, b"Addresses:\0".as_ptr() as *const i8);
    i = 0;
    while i < (*emu).num_ports {
        snd_iprintf(buf, b" %d:%d\0".as_ptr() as *const i8, (*emu).client, *(*emu).ports.offset(i as isize));
        i += 1;
    }
    snd_iprintf(buf, b"\n\0".as_ptr() as *const i8);
    snd_iprintf(buf, b"Use Counter: %d\n\0".as_ptr() as *const i8, (*emu).used);
    snd_iprintf(buf, b"Max Voices: %d\n\0".as_ptr() as *const i8, (*emu).max_voices);
    snd_iprintf(buf, b"Allocated Voices: %d\n\0".as_ptr() as *const i8, (*emu).num_voices);
    if !(*emu).memhdr.is_null() {
        snd_iprintf(buf, b"Memory Size: %d\n\0".as_ptr() as *const i8, (*(*emu).memhdr).size);
        snd_iprintf(buf, b"Memory Available: %d\n\0".as_ptr() as *const i8, snd_util_mem_avail((*emu).memhdr));
        snd_iprintf(buf, b"Allocated Blocks: %d\n\0".as_ptr() as *const i8, (*(*emu).memhdr).nblocks);
    } else {
        snd_iprintf(buf, b"Memory Size: 0\n\0".as_ptr() as *const i8);
    }
    if !(*emu).sflist.is_null() {
        // guard(mutex)(&emu->sflist->presets_mutex);
        snd_iprintf(buf, b"SoundFonts: %d\n\0".as_ptr() as *const i8, (*(*emu).sflist).fonts_size);
        snd_iprintf(buf, b"Instruments: %d\n\0".as_ptr() as *const i8, (*(*emu).sflist).zone_counter);
        snd_iprintf(buf, b"Samples: %d\n\0".as_ptr() as *const i8, (*(*emu).sflist).sample_counter);
        snd_iprintf(buf, b"Locked Instruments: %d\n\0".as_ptr() as *const i8, (*(*emu).sflist).zone_locked);
        snd_iprintf(buf, b"Locked Samples: %d\n\0".as_ptr() as *const i8, (*(*emu).sflist).sample_locked);
    }
    // #if 0  /* debug */
    // ...debug code omitted...
    // #endif
}

pub unsafe fn snd_emux_proc_init(emu: *mut snd_emux, card: *mut snd_card, device: i32) {
    let mut name: [u8; 64] = [0; 64];
    let name_cstr = format!("wavetableD{}\0", device);
    let name_bytes = name_cstr.as_bytes();
    if name_bytes.len() <= 64 {
        name[..name_bytes.len()].copy_from_slice(name_bytes);
    }

    let entry: *mut snd_info_entry = snd_info_create_card_entry(
        card,
        name.as_ptr() as *const i8,
        (*card).proc_root,
    );
    if entry.is_null() {
        return;
    }

    const SNDRV_INFO_CONTENT_TEXT: i32 = 0;
    (*entry).content = SNDRV_INFO_CONTENT_TEXT;
    (*entry).private_data = emu as *mut core::ffi::c_void;
    (*entry).c.text.read = Some(snd_emux_proc_info_read);
    (*emu).proc = entry;
}

pub unsafe fn snd_emux_proc_free(emu: *mut snd_emux) {
    snd_info_free_entry((*emu).proc);
    (*emu).proc = core::ptr::null_mut();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
