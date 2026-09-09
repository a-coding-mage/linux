/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */

/*
 *  Advanced Linux Sound Architecture - ALSA
 *
 *  Interface file between ALSA driver & user space
 *  Copyright (c) 1994-98 by Jaroslav Kysela <perex@perex.cz>,
 *                           4Front Technologies
 *
 *  Direct FM control
 */

pub const SNDRV_DM_FM_MODE_OPL2: u32 = 0x00;
pub const SNDRV_DM_FM_MODE_OPL3: u32 = 0x01;

#[repr(C)]
pub struct snd_dm_fm_info {
    pub fm_mode: u8,   /* OPL mode, see SNDRV_DM_FM_MODE_XXX */
    pub rhythm: u8,    /* percussion mode flag */
}

/*
 *  Data structure composing an FM "note" or sound event.
 */
#[repr(C)]
pub struct snd_dm_fm_voice {
    pub op: u8,             /* operator cell (0 or 1) */
    pub voice: u8,          /* FM voice (0 to 17) */
    pub am: u8,             /* amplitude modulation */
    pub vibrato: u8,        /* vibrato effect */
    pub do_sustain: u8,     /* sustain phase */
    pub kbd_scale: u8,      /* keyboard scaling */
    pub harmonic: u8,       /* 4 bits: harmonic and multiplier */
    pub scale_level: u8,    /* 2 bits: decrease output freq rises */
    pub volume: u8,         /* 6 bits: volume */
    pub attack: u8,         /* 4 bits: attack rate */
    pub decay: u8,          /* 4 bits: decay rate */
    pub sustain: u8,        /* 4 bits: sustain level */
    pub release: u8,        /* 4 bits: release rate */
    pub feedback: u8,       /* 3 bits: feedback for op0 */
    pub connection: u8,     /* 0 for serial, 1 for parallel */
    pub left: u8,           /* stereo left */
    pub right: u8,          /* stereo right */
    pub waveform: u8,       /* 3 bits: waveform shape */
}

/*
 *  This describes an FM note by its voice, octave, frequency number (10bit)
 *  and key on/off.
 */
#[repr(C)]
pub struct snd_dm_fm_note {
    pub voice: u8,    /* 0-17 voice channel */
    pub octave: u8,   /* 3 bits: what octave to play */
    pub fnum: u32,    /* 10 bits: frequency number */
    pub key_on: u8,   /* set for active, clear for silent */
}

/*
 *  FM parameters that apply globally to all voices, and thus are not "notes"
 */
#[repr(C)]
pub struct snd_dm_fm_params {
    pub am_depth: u8, /* amplitude modulation depth (1=hi) */
    pub vib_depth: u8, /* vibrato depth (1=hi) */
    pub kbd_split: u8, /* keyboard split */
    pub rhythm: u8,    /* percussion mode select */

    /* This block is the percussion instrument data */
    pub bass: u8,
    pub snare: u8,
    pub tomtom: u8,
    pub cymbal: u8,
    pub hihat: u8,
}

/* FM mode ioctl settings.  _IOR/_IOW/_IO are supplied by the target ABI. */
pub const SNDRV_DM_FM_IOCTL_INFO: _ = _IOR(b'H', 0x20, snd_dm_fm_info);
pub const SNDRV_DM_FM_IOCTL_RESET: _ = _IO(b'H', 0x21);
pub const SNDRV_DM_FM_IOCTL_PLAY_NOTE: _ = _IOW(b'H', 0x22, snd_dm_fm_note);
pub const SNDRV_DM_FM_IOCTL_SET_VOICE: _ = _IOW(b'H', 0x23, snd_dm_fm_voice);
pub const SNDRV_DM_FM_IOCTL_SET_PARAMS: _ = _IOW(b'H', 0x24, snd_dm_fm_params);
pub const SNDRV_DM_FM_IOCTL_SET_MODE: _ = _IOW(b'H', 0x25, i32);
/* for OPL3 only */
pub const SNDRV_DM_FM_IOCTL_SET_CONNECTION: _ = _IOW(b'H', 0x26, i32);
/* SBI patch management */
pub const SNDRV_DM_FM_IOCTL_CLEAR_PATCHES: _ = _IO(b'H', 0x40);

pub const SNDRV_DM_FM_OSS_IOCTL_RESET: u32 = 0x20;
pub const SNDRV_DM_FM_OSS_IOCTL_PLAY_NOTE: u32 = 0x21;
pub const SNDRV_DM_FM_OSS_IOCTL_SET_VOICE: u32 = 0x22;
pub const SNDRV_DM_FM_OSS_IOCTL_SET_PARAMS: u32 = 0x23;
pub const SNDRV_DM_FM_OSS_IOCTL_SET_MODE: u32 = 0x24;
pub const SNDRV_DM_FM_OSS_IOCTL_SET_OPL: u32 = 0x25;

/*
 * Patch Record - fixed size for write
 */
pub const FM_KEY_SBI: &[u8; 4] = b"SBI\x1a";
pub const FM_KEY_2OP: &[u8; 4] = b"2OP\x1a";
pub const FM_KEY_4OP: &[u8; 4] = b"4OP\x1a";

#[repr(C)]
pub struct sbi_patch {
    pub prog: u8,
    pub bank: u8,
    pub key: [i8; 4],
    pub name: [i8; 25],
    pub extension: [i8; 7],
    pub data: [u8; 32],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
