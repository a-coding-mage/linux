/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Defines for the emu8000 (AWE32/64)
 *
 *  Copyright (C) 1999 Steve Ratcliffe
 *  Copyright (C) 1999-2000 Takashi Iwai <tiwai@suse.de>
 */

/* Dependencies supplied by the surrounding kernel translation. */

/*
 * Hardware parameters.
 */
pub const EMU8000_MAX_DRAM: usize = 28 * 1024 * 1024; /* Max on-board mem is 28Mb ???*/
pub const EMU8000_DRAM_OFFSET: u32 = 0x200000; /* Beginning of on board ram */
pub const EMU8000_CHANNELS: i32 = 32; /* Number of hardware channels */
pub const EMU8000_DRAM_VOICES: i32 = 30; /* number of normal voices */

/* Flags to set a dma channel to read or write */
pub const EMU8000_RAM_READ: i32 = 0;
pub const EMU8000_RAM_WRITE: i32 = 1;
pub const EMU8000_RAM_CLOSE: i32 = 2;
pub const EMU8000_RAM_MODE_MASK: i32 = 0x03;
pub const EMU8000_RAM_RIGHT: i32 = 0x10; /* use 'right' DMA channel */

#[repr(i32)]
pub enum Emu8000Control {
    EMU8000_CONTROL_BASS = 0,
    EMU8000_CONTROL_TREBLE,
    EMU8000_CONTROL_CHORUS_MODE,
    EMU8000_CONTROL_REVERB_MODE,
    EMU8000_CONTROL_FM_CHORUS_DEPTH,
    EMU8000_CONTROL_FM_REVERB_DEPTH,
    EMU8000_NUM_CONTROLS,
}

/*
 * Structure to hold all state information for the emu8000 driver.
 *
 * Note 1: The chip supports 32 channels in hardware this is max_channels
 * some of the channels may be used for other things so max_channels is
 * the number in use for wave voices.
 */
#[repr(C)]
pub struct snd_emu8000 {
    pub emu: *mut snd_emux,

    pub index: i32, /* sequencer client index */
    pub seq_ports: i32, /* number of sequencer ports */
    pub fm_chorus_depth: i32, /* FM OPL3 chorus depth */
    pub fm_reverb_depth: i32, /* FM OPL3 reverb depth */

    pub mem_size: i32, /* memory size */
    pub port1: libc::c_ulong, /* Port usually base+0 */
    pub port2: libc::c_ulong, /* Port usually at base+0x400 */
    pub port3: libc::c_ulong, /* Port usually at base+0x800 */
    pub last_reg: u16, /* Last register command */
    pub reg_lock: spinlock_t,

    pub dram_checked: i32,

    pub card: *mut snd_card, /* The card that this belongs to */

    pub chorus_mode: i32,
    pub reverb_mode: i32,
    pub bass_level: i32,
    pub treble_level: i32,

    pub memhdr: *mut snd_util_memhdr,

    pub control_lock: spinlock_t,
    pub controls: [*mut snd_kcontrol; EMU8000_NUM_CONTROLS as usize],

    pub pcm: *mut snd_pcm, /* pcm on emu8000 wavetable */
}

/* sequencer device id */
pub const SNDRV_SEQ_DEV_ID_EMU8000: &str = "emu8000-synth";

/* exported functions */
extern "C" {
    pub fn snd_emu8000_new(card: *mut snd_card, device: i32, port: libc::c_long,
                           seq_ports: i32, ret: *mut *mut snd_seq_device) -> i32;
    pub fn snd_emu8000_poke(emu: *mut snd_emu8000, port: u32, reg: u32, val: u32);
    pub fn snd_emu8000_peek(emu: *mut snd_emu8000, port: u32, reg: u32) -> u16;
    pub fn snd_emu8000_poke_dw(emu: *mut snd_emu8000, port: u32, reg: u32, val: u32);
    pub fn snd_emu8000_peek_dw(emu: *mut snd_emu8000, port: u32, reg: u32) -> u32;
    pub fn snd_emu8000_dma_chan(emu: *mut snd_emu8000, ch: i32, mode: i32);

    pub fn snd_emu8000_init_fm(emu: *mut snd_emu8000);

    pub fn snd_emu8000_update_chorus_mode(emu: *mut snd_emu8000);
    pub fn snd_emu8000_update_reverb_mode(emu: *mut snd_emu8000);
    pub fn snd_emu8000_update_equalizer(emu: *mut snd_emu8000);
    pub fn snd_emu8000_load_chorus_fx(emu: *mut snd_emu8000, mode: i32,
                                      buf: *const core::ffi::c_void, len: libc::c_long) -> i32;
    pub fn snd_emu8000_load_reverb_fx(emu: *mut snd_emu8000, mode: i32,
                                      buf: *const core::ffi::c_void, len: libc::c_long) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
