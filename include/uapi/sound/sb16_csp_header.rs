/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 *  Copyright (c) 1999 by Uros Bizjak <uros@kss-loka.si>
 *                        Takashi Iwai <tiwai@suse.de>
 *
 *  SB16ASP/AWE32 CSP control
 */

/* CSP modes */
pub const SNDRV_SB_CSP_MODE_NONE: u32 = 0x00;
pub const SNDRV_SB_CSP_MODE_DSP_READ: u32 = 0x01; /* Record from DSP */
pub const SNDRV_SB_CSP_MODE_DSP_WRITE: u32 = 0x02; /* Play to DSP */
pub const SNDRV_SB_CSP_MODE_QSOUND: u32 = 0x04; /* QSound */

/* CSP load flags */
pub const SNDRV_SB_CSP_LOAD_FROMUSER: u32 = 0x01;
pub const SNDRV_SB_CSP_LOAD_INITBLOCK: u32 = 0x02;

/* CSP sample width */
pub const SNDRV_SB_CSP_SAMPLE_8BIT: u32 = 0x01;
pub const SNDRV_SB_CSP_SAMPLE_16BIT: u32 = 0x02;

/* CSP channels */
pub const SNDRV_SB_CSP_MONO: u32 = 0x01;
pub const SNDRV_SB_CSP_STEREO: u32 = 0x02;

/* CSP rates */
pub const SNDRV_SB_CSP_RATE_8000: u32 = 0x01;
pub const SNDRV_SB_CSP_RATE_11025: u32 = 0x02;
pub const SNDRV_SB_CSP_RATE_22050: u32 = 0x04;
pub const SNDRV_SB_CSP_RATE_44100: u32 = 0x08;
pub const SNDRV_SB_CSP_RATE_ALL: u32 = 0x0f;

/* CSP running state */
pub const SNDRV_SB_CSP_ST_IDLE: u32 = 0x00;
pub const SNDRV_SB_CSP_ST_LOADED: u32 = 0x01;
pub const SNDRV_SB_CSP_ST_RUNNING: u32 = 0x02;
pub const SNDRV_SB_CSP_ST_PAUSED: u32 = 0x04;
pub const SNDRV_SB_CSP_ST_AUTO: u32 = 0x08;
pub const SNDRV_SB_CSP_ST_QSOUND: u32 = 0x10;

/* maximum QSound value (180 degrees right) */
pub const SNDRV_SB_CSP_QSOUND_MAX_RIGHT: u32 = 0x20;

/* maximum microcode RIFF file size */
pub const SNDRV_SB_CSP_MAX_MICROCODE_FILE_SIZE: usize = 0x3000;

/* microcode header */
#[repr(C)]
pub struct snd_sb_csp_mc_header {
    pub codec_name: [i8; 16], /* id name of codec */
    pub func_req: u16, /* requested function */
}

/* microcode to be loaded */
#[repr(C)]
pub struct snd_sb_csp_microcode {
    pub info: snd_sb_csp_mc_header,
    pub data: [u8; SNDRV_SB_CSP_MAX_MICROCODE_FILE_SIZE],
}

/* start CSP with sample_width in mono/stereo */
#[repr(C)]
pub struct snd_sb_csp_start {
    pub sample_width: i32, /* sample width, look above */
    pub channels: i32, /* channels, look above */
}

/* CSP information */
#[repr(C)]
pub struct snd_sb_csp_info {
    pub codec_name: [i8; 16], /* id name of codec */
    pub func_nr: u16, /* function number */
    pub acc_format: u32, /* accepted PCM formats */
    pub acc_channels: u16, /* accepted channels */
    pub acc_width: u16, /* accepted sample width */
    pub acc_rates: u16, /* accepted sample rates */
    pub csp_mode: u16, /* CSP mode, see above */
    pub run_channels: u16, /* current channels  */
    pub run_width: u16, /* current sample width */
    pub version: u16, /* version id: 0x10 - 0x1f */
    pub state: u16, /* state bits */
}

/* HWDEP controls */
/* get CSP information */
/* `_IOR`, `_IOC`, `_IO`, and `_IOW` are supplied by the surrounding ABI. */
pub const SNDRV_SB_CSP_IOCTL_INFO: _ = _IOR('H', 0x10, snd_sb_csp_info);
/* load microcode to CSP */
/* NOTE: struct snd_sb_csp_microcode overflows the max size (13 bits)
 * defined for some architectures like MIPS, and it leads to build errors.
 * (x86 and co have 14-bit size, thus it's valid, though.)
 * As a workaround for skipping the size-limit check, here we don't use the
 * normal _IOW() macro but _IOC() with the manual argument.
 */
pub const SNDRV_SB_CSP_IOCTL_LOAD_CODE: _ =
    _IOC(_IOC_WRITE, 'H', 0x11, core::mem::size_of::<snd_sb_csp_microcode>());
/* unload microcode from CSP */
pub const SNDRV_SB_CSP_IOCTL_UNLOAD_CODE: _ = _IO('H', 0x12);
/* start CSP */
pub const SNDRV_SB_CSP_IOCTL_START: _ = _IOW('H', 0x13, snd_sb_csp_start);
/* stop CSP */
pub const SNDRV_SB_CSP_IOCTL_STOP: _ = _IO('H', 0x14);
/* pause CSP and DMA transfer */
pub const SNDRV_SB_CSP_IOCTL_PAUSE: _ = _IO('H', 0x15);
/* restart CSP and DMA transfer */
pub const SNDRV_SB_CSP_IOCTL_RESTART: _ = _IO('H', 0x16);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
