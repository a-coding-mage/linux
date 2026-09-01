/*
 * Local definitions for the OPL4 driver
 *
 * Copyright (c) 2003 by Clemens Ladisch <clemens@ladisch.de>
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions, and the following disclaimer,
 *    without modification.
 * 2. The name of the author may not be used to endorse or promote products
 *    derived from this software without specific prior written permission.
 *
 * Alternatively, this software may be distributed and/or modified under the
 * terms of the GNU General Public License as published by the Free Software
 * Foundation; either version 2 of the License, or (at your option) any later
 * version.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

/* Dependency intent from C header: #include <sound/opl4.h> */

/*
 * Register numbers
 */

pub const OPL4_REG_TEST0: u32 = 0x00;
pub const OPL4_REG_TEST1: u32 = 0x01;

pub const OPL4_REG_MEMORY_CONFIGURATION: u32 = 0x02;
pub const OPL4_MODE_BIT: u32 = 0x01;
pub const OPL4_MTYPE_BIT: u32 = 0x02;
pub const OPL4_TONE_HEADER_MASK: u32 = 0x1c;
pub const OPL4_DEVICE_ID_MASK: u32 = 0xe0;

pub const OPL4_REG_MEMORY_ADDRESS_HIGH: u32 = 0x03;
pub const OPL4_REG_MEMORY_ADDRESS_MID: u32 = 0x04;
pub const OPL4_REG_MEMORY_ADDRESS_LOW: u32 = 0x05;
pub const OPL4_REG_MEMORY_DATA: u32 = 0x06;

/*
 * Offsets to the register banks for voices. To get the
 * register number just add the voice number to the bank offset.
 *
 * Wave Table Number low bits (0x08 to 0x1F)
 */
pub const OPL4_REG_TONE_NUMBER: u32 = 0x08;

/* Wave Table Number high bit, F-Number low bits (0x20 to 0x37) */
pub const OPL4_REG_F_NUMBER: u32 = 0x20;
pub const OPL4_TONE_NUMBER_BIT8: u32 = 0x01;
pub const OPL4_F_NUMBER_LOW_MASK: u32 = 0xfe;

/* F-Number high bits, Octave, Pseudo-Reverb (0x38 to 0x4F) */
pub const OPL4_REG_OCTAVE: u32 = 0x38;
pub const OPL4_F_NUMBER_HIGH_MASK: u32 = 0x07;
pub const OPL4_BLOCK_MASK: u32 = 0xf0;
pub const OPL4_PSEUDO_REVERB_BIT: u32 = 0x08;

/* Total Level, Level Direct (0x50 to 0x67) */
pub const OPL4_REG_LEVEL: u32 = 0x50;
pub const OPL4_TOTAL_LEVEL_MASK: u32 = 0xfe;
pub const OPL4_LEVEL_DIRECT_BIT: u32 = 0x01;

/* Key On, Damp, LFO RST, CH, Panpot (0x68 to 0x7F) */
pub const OPL4_REG_MISC: u32 = 0x68;
pub const OPL4_KEY_ON_BIT: u32 = 0x80;
pub const OPL4_DAMP_BIT: u32 = 0x40;
pub const OPL4_LFO_RESET_BIT: u32 = 0x20;
pub const OPL4_OUTPUT_CHANNEL_BIT: u32 = 0x10;
pub const OPL4_PAN_POT_MASK: u32 = 0x0f;

/* LFO, VIB (0x80 to 0x97) */
pub const OPL4_REG_LFO_VIBRATO: u32 = 0x80;
pub const OPL4_LFO_FREQUENCY_MASK: u32 = 0x38;
pub const OPL4_VIBRATO_DEPTH_MASK: u32 = 0x07;
pub const OPL4_CHORUS_SEND_MASK: u32 = 0xc0; /* ML only */

/* Attack / Decay 1 rate (0x98 to 0xAF) */
pub const OPL4_REG_ATTACK_DECAY1: u32 = 0x98;
pub const OPL4_ATTACK_RATE_MASK: u32 = 0xf0;
pub const OPL4_DECAY1_RATE_MASK: u32 = 0x0f;

/* Decay level / 2 rate (0xB0 to 0xC7) */
pub const OPL4_REG_LEVEL_DECAY2: u32 = 0xb0;
pub const OPL4_DECAY_LEVEL_MASK: u32 = 0xf0;
pub const OPL4_DECAY2_RATE_MASK: u32 = 0x0f;

/* Release rate / Rate correction (0xC8 to 0xDF) */
pub const OPL4_REG_RELEASE_CORRECTION: u32 = 0xc8;
pub const OPL4_RELEASE_RATE_MASK: u32 = 0x0f;
pub const OPL4_RATE_INTERPOLATION_MASK: u32 = 0xf0;

/* AM (0xE0 to 0xF7) */
pub const OPL4_REG_TREMOLO: u32 = 0xe0;
pub const OPL4_TREMOLO_DEPTH_MASK: u32 = 0x07;
pub const OPL4_REVERB_SEND_MASK: u32 = 0xe0; /* ML only */

/* Mixer */
pub const OPL4_REG_MIX_CONTROL_FM: u32 = 0xf8;
pub const OPL4_REG_MIX_CONTROL_PCM: u32 = 0xf9;
pub const OPL4_MIX_LEFT_MASK: u32 = 0x07;
pub const OPL4_MIX_RIGHT_MASK: u32 = 0x38;

pub const OPL4_REG_ATC: u32 = 0xfa;
pub const OPL4_ATC_BIT: u32 = 0x01; /* ???, ML only */

/* bits in the OPL3 Status register */
pub const OPL4_STATUS_BUSY: u32 = 0x01;
pub const OPL4_STATUS_LOAD: u32 = 0x02;

pub const OPL4_MAX_VOICES: usize = 24;

pub const SNDRV_SEQ_DEV_ID_OPL4: &[u8; 11] = b"opl4-synth";

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_midi_channel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_info_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_seq_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_midi_channel_set {
    _private: [u8; 0],
}

#[repr(C)]
pub struct opl4_sound {
    pub tone: u16,
    pub pitch_offset: i16,
    pub key_scaling: u8,
    pub panpot: i8,
    pub vibrato: u8,
    pub tone_attenuate: u8,
    pub volume_factor: u8,
    pub reg_lfo_vibrato: u8,
    pub reg_attack_decay1: u8,
    pub reg_level_decay2: u8,
    pub reg_release_correction: u8,
    pub reg_tremolo: u8,
}

#[repr(C)]
pub struct opl4_region {
    pub key_min: u8,
    pub key_max: u8,
    pub sound: opl4_sound,
}

#[repr(C)]
pub struct opl4_region_ptr {
    pub count: core::ffi::c_int,
    pub regions: *const opl4_region,
}

#[repr(C)]
pub struct opl4_voice {
    pub list: list_head,
    pub number: core::ffi::c_int,
    pub chan: *mut snd_midi_channel,
    pub note: core::ffi::c_int,
    pub velocity: core::ffi::c_int,
    pub sound: *const opl4_sound,
    pub level_direct: u8,
    pub reg_f_number: u8,
    pub reg_misc: u8,
    pub reg_lfo_vibrato: u8,
}

#[repr(C)]
pub struct snd_opl4 {
    pub fm_port: core::ffi::c_ulong,
    pub pcm_port: core::ffi::c_ulong,
    pub res_fm_port: *mut resource,
    pub res_pcm_port: *mut resource,
    pub hardware: core::ffi::c_ushort,
    pub reg_lock: spinlock_t,
    pub card: *mut snd_card,

    /* Present in C when CONFIG_SND_PROC_FS is defined. */
    pub proc_entry: *mut snd_info_entry,
    pub memory_access: core::ffi::c_int,

    pub access_mutex: mutex,

    /* Present in C when IS_ENABLED(CONFIG_SND_SEQUENCER). */
    pub used: core::ffi::c_int,
    pub seq_dev_num: core::ffi::c_int,
    pub seq_client: core::ffi::c_int,
    pub seq_dev: *mut snd_seq_device,

    pub chset: *mut snd_midi_channel_set,
    pub voices: [opl4_voice; OPL4_MAX_VOICES],
    pub off_voices: list_head,
    pub on_voices: list_head,
}

unsafe extern "C" {
    /* opl4_lib.c */
    pub fn snd_opl4_write(opl4: *mut snd_opl4, reg: u8, value: u8);
    pub fn snd_opl4_read(opl4: *mut snd_opl4, reg: u8) -> u8;
    pub fn snd_opl4_read_memory(
        opl4: *mut snd_opl4,
        buf: *mut core::ffi::c_char,
        offset: core::ffi::c_int,
        size: core::ffi::c_int,
    );
    pub fn snd_opl4_write_memory(
        opl4: *mut snd_opl4,
        buf: *const core::ffi::c_char,
        offset: core::ffi::c_int,
        size: core::ffi::c_int,
    );

    /* opl4_mixer.c */
    pub fn snd_opl4_create_mixer(opl4: *mut snd_opl4) -> core::ffi::c_int;

    /*
     * opl4_proc.c
     * In C, these are external declarations when CONFIG_SND_PROC_FS is defined;
     * otherwise static inline functions below provide no-op behavior.
     */
    pub fn snd_opl4_create_proc(opl4: *mut snd_opl4) -> core::ffi::c_int;
    pub fn snd_opl4_free_proc(opl4: *mut snd_opl4);

    /* opl4_seq.c */
    pub static mut volume_boost: core::ffi::c_int;

    /* opl4_synth.c */
    pub fn snd_opl4_synth_reset(opl4: *mut snd_opl4);
    pub fn snd_opl4_synth_shutdown(opl4: *mut snd_opl4);
    pub fn snd_opl4_note_on(
        p: *mut core::ffi::c_void,
        note: core::ffi::c_int,
        vel: core::ffi::c_int,
        chan: *mut snd_midi_channel,
    );
    pub fn snd_opl4_note_off(
        p: *mut core::ffi::c_void,
        note: core::ffi::c_int,
        vel: core::ffi::c_int,
        chan: *mut snd_midi_channel,
    );
    pub fn snd_opl4_terminate_note(
        p: *mut core::ffi::c_void,
        note: core::ffi::c_int,
        chan: *mut snd_midi_channel,
    );
    pub fn snd_opl4_control(
        p: *mut core::ffi::c_void,
        type_: core::ffi::c_int,
        chan: *mut snd_midi_channel,
    );
    pub fn snd_opl4_sysex(
        p: *mut core::ffi::c_void,
        buf: *mut core::ffi::c_uchar,
        len: core::ffi::c_int,
        parsed: core::ffi::c_int,
        chset: *mut snd_midi_channel_set,
    );

    /* yrw801.c */
    pub fn snd_yrw801_detect(opl4: *mut snd_opl4) -> core::ffi::c_int;
    pub static snd_yrw801_regions: [opl4_region_ptr; 0];
}

/*
 * C fallback when CONFIG_SND_PROC_FS is not defined:
 * static inline int snd_opl4_create_proc(struct snd_opl4 *opl4) { return 0; }
 * static inline void snd_opl4_free_proc(struct snd_opl4 *opl4) {}
 */
#[inline]
pub unsafe fn snd_opl4_create_proc_noop(_opl4: *mut snd_opl4) -> core::ffi::c_int {
    0
}

#[inline]
pub unsafe fn snd_opl4_free_proc_noop(_opl4: *mut snd_opl4) {}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
