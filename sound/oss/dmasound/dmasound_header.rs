/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  linux/sound/oss/dmasound/dmasound.h
 *
 *
 *  Minor numbers for the sound driver.
 *
 *  Unfortunately Creative called the codec chip of SB as a DSP. For this
 *  reason the /dev/dsp is reserved for digitized audio use. There is a
 *  device for true DSP processors but it will be called something else.
 *  In v3.0 it's /dev/sndproc but this could be a temporary solution.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub type size_t = usize;
pub type ssize_t = isize;
pub type u_char = u8;
pub type u_int = c_uint;
pub type u_long = c_ulong;

pub const SND_NDEVS: c_int = 256; /* Number of supported devices */
pub const SND_DEV_CTL: c_int = 0; /* Control port /dev/mixer */
pub const SND_DEV_SEQ: c_int = 1; /* Sequencer output /dev/sequencer (FM
                                   * synthesizer and MIDI output) */
pub const SND_DEV_MIDIN: c_int = 2; /* Raw midi access */
pub const SND_DEV_DSP: c_int = 3; /* Digitized voice /dev/dsp */
pub const SND_DEV_AUDIO: c_int = 4; /* Sparc compatible /dev/audio */
pub const SND_DEV_DSP16: c_int = 5; /* Like /dev/dsp but 16 bits/sample */
pub const SND_DEV_STATUS: c_int = 6; /* /dev/sndstat */
/* #7 not in use now. Was in 2.4. Free for use after v3.0. */
pub const SND_DEV_SEQ2: c_int = 8; /* /dev/sequencer, level 2 interface */
pub const SND_DEV_SNDPROC: c_int = 9; /* /dev/sndproc for programmable devices */
pub const SND_DEV_PSS: c_int = SND_DEV_SNDPROC;

/* switch on various prinks */
pub const DEBUG_DMASOUND: c_int = 1;

pub const MAX_AUDIO_DEV: c_int = 5;
pub const MAX_MIXER_DEV: c_int = 4;
pub const MAX_SYNTH_DEV: c_int = 3;
pub const MAX_MIDI_DEV: c_int = 6;
pub const MAX_TIMER_DEV: c_int = 3;

pub const MAX_CATCH_RADIUS: c_int = 10;

#[inline]
pub const fn le2be16(x: c_int) -> c_int {
    ((x << 8) & 0xff00) | ((x >> 8) & 0x00ff)
}

#[inline]
pub const fn le2be16dbl(x: c_int) -> c_int {
    ((x << 8) & 0xff00ff00u32 as c_int) | ((x >> 8) & 0x00ff00ff)
}

extern "C" {
    pub fn get_user(value: *mut c_int, addr: *const c_int) -> c_int;
    pub fn put_user(value: c_int, addr: *mut c_int) -> c_int;
    pub fn wake_up_interruptible(queue: *mut wait_queue_head_t);
}

#[macro_export]
macro_rules! IOCTL_IN {
    ($arg:expr, $ret:expr) => {{
        let error: ::core::ffi::c_int = unsafe {
            get_user(&mut $ret as *mut ::core::ffi::c_int, $arg as *const ::core::ffi::c_int)
        };
        if error != 0 {
            return error;
        }
    }};
}

#[macro_export]
macro_rules! IOCTL_OUT {
    ($arg:expr, $ret:expr) => {{
        unsafe { ioctl_return($arg as *mut ::core::ffi::c_int, $ret) }
    }};
}

#[inline]
pub unsafe fn ioctl_return(addr: *mut c_int, value: c_int) -> c_int {
    if value < 0 {
        value
    } else {
        unsafe { put_user(value, addr) }
    }
}

/*
 *  Configuration
 */

/* HAS_8BIT_TABLES is defined for Atari, Paula, and Q40 DMASOUND builds. */
pub const MIN_BUFFERS: c_int = 2;
pub const MIN_BUFSIZE: c_int = 1 << 8; /* in bytes */
pub const MIN_FRAG_SIZE: c_int = 8;
pub const MAX_BUFSIZE: c_int = 1 << 18; /* this is somewhat arbitrary for pmac */
pub const MAX_FRAG_SIZE: c_int = 16; /* need to allow *4 for mono-8 => stereo-16 */

pub const DEFAULT_N_BUFFERS: c_int = 4;
pub const DEFAULT_BUFF_SIZE: c_int = 1 << 15;

/*
 *  Initialization
 */

extern "C" {
    pub fn dmasound_init() -> c_int;
    pub fn dmasound_deinit();
}

/* description of the set-up applies to either hard or soft settings */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SETTINGS {
    pub format: c_int, /* AFMT_* */
    pub stereo: c_int, /* 0 = mono, 1 = stereo */
    pub size: c_int,   /* 8/16 bit*/
    pub speed: c_int,  /* speed */
}

/*
 *  Machine definitions
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MACHINE {
    pub name: *const c_char,
    pub name2: *const c_char,
    pub owner: *mut module,
    pub dma_alloc: Option<unsafe extern "C" fn(c_uint, gfp_t) -> *mut c_void>,
    pub dma_free: Option<unsafe extern "C" fn(*mut c_void, c_uint)>,
    pub irqinit: Option<unsafe extern "C" fn() -> c_int>,
    pub irqcleanup: Option<unsafe extern "C" fn()>,
    pub init: Option<unsafe extern "C" fn()>,
    pub silence: Option<unsafe extern "C" fn()>,
    pub setFormat: Option<unsafe extern "C" fn(c_int) -> c_int>,
    pub setVolume: Option<unsafe extern "C" fn(c_int) -> c_int>,
    pub setBass: Option<unsafe extern "C" fn(c_int) -> c_int>,
    pub setTreble: Option<unsafe extern "C" fn(c_int) -> c_int>,
    pub setGain: Option<unsafe extern "C" fn(c_int) -> c_int>,
    pub play: Option<unsafe extern "C" fn()>,
    pub record: Option<unsafe extern "C" fn()>, /* optional */
    pub mixer_init: Option<unsafe extern "C" fn()>, /* optional */
    pub mixer_ioctl: Option<unsafe extern "C" fn(u_int, u_long) -> c_int>, /* optional */
    pub write_sq_setup: Option<unsafe extern "C" fn() -> c_int>, /* optional */
    pub read_sq_setup: Option<unsafe extern "C" fn() -> c_int>, /* optional */
    pub sq_open: Option<unsafe extern "C" fn(fmode_t) -> c_int>, /* optional */
    pub state_info: Option<unsafe extern "C" fn(*mut c_char, size_t) -> c_int>, /* optional */
    pub abort_read: Option<unsafe extern "C" fn()>, /* optional */
    pub min_dsp_speed: c_int,
    pub max_dsp_speed: c_int,
    pub version: c_int,
    pub hardware_afmts: c_int, /* OSS says we only return h'ware info */
                              /* when queried via SNDCTL_DSP_GETFMTS */
    pub capabilities: c_int, /* low-level reply to SNDCTL_DSP_GETCAPS */
    pub default_hard: SETTINGS, /* open() or init() should set something valid */
    pub default_soft: SETTINGS, /* you can make it look like old OSS, if you want to */
}

/*
 *  Low level stuff
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TRANS {
    pub ct_ulaw:
        Option<unsafe extern "C" fn(*const u_char, size_t, *mut u_char, *mut ssize_t, ssize_t) -> ssize_t>,
    pub ct_alaw:
        Option<unsafe extern "C" fn(*const u_char, size_t, *mut u_char, *mut ssize_t, ssize_t) -> ssize_t>,
    pub ct_s8:
        Option<unsafe extern "C" fn(*const u_char, size_t, *mut u_char, *mut ssize_t, ssize_t) -> ssize_t>,
    pub ct_u8:
        Option<unsafe extern "C" fn(*const u_char, size_t, *mut u_char, *mut ssize_t, ssize_t) -> ssize_t>,
    pub ct_s16be:
        Option<unsafe extern "C" fn(*const u_char, size_t, *mut u_char, *mut ssize_t, ssize_t) -> ssize_t>,
    pub ct_u16be:
        Option<unsafe extern "C" fn(*const u_char, size_t, *mut u_char, *mut ssize_t, ssize_t) -> ssize_t>,
    pub ct_s16le:
        Option<unsafe extern "C" fn(*const u_char, size_t, *mut u_char, *mut ssize_t, ssize_t) -> ssize_t>,
    pub ct_u16le:
        Option<unsafe extern "C" fn(*const u_char, size_t, *mut u_char, *mut ssize_t, ssize_t) -> ssize_t>,
}

#[repr(C)]
pub struct sound_settings {
    pub mach: MACHINE,       /* machine dependent things */
    pub hard: SETTINGS,      /* hardware settings */
    pub soft: SETTINGS,      /* software settings */
    pub dsp: SETTINGS,       /* /dev/dsp default settings */
    pub trans_write: *mut TRANS, /* supported translations */
    pub volume_left: c_int,  /* volume (range is machine dependent) */
    pub volume_right: c_int,
    pub bass: c_int,         /* tone (range is machine dependent) */
    pub treble: c_int,
    pub gain: c_int,
    pub minDev: c_int,       /* minor device number currently open */
    pub lock: spinlock_t,
}

extern "C" {
    pub static mut dmasound: sound_settings;
}

/* Present only when HAS_8BIT_TABLES is enabled by the target configuration. */
extern "C" {
    pub static mut dmasound_ulaw2dma8: [c_char; 0];
    pub static mut dmasound_alaw2dma8: [c_char; 0];
}

/*
 *  Mid level stuff
 */

#[inline]
pub unsafe fn dmasound_set_volume(volume: c_int) -> c_int {
    unsafe { (dmasound.mach.setVolume.unwrap_unchecked())(volume) }
}

#[inline]
pub unsafe fn dmasound_set_bass(bass: c_int) -> c_int {
    unsafe {
        match dmasound.mach.setBass {
            Some(set_bass) => set_bass(bass),
            None => 50,
        }
    }
}

#[inline]
pub unsafe fn dmasound_set_treble(treble: c_int) -> c_int {
    unsafe {
        match dmasound.mach.setTreble {
            Some(set_treble) => set_treble(treble),
            None => 50,
        }
    }
}

#[inline]
pub unsafe fn dmasound_set_gain(gain: c_int) -> c_int {
    unsafe {
        match dmasound.mach.setGain {
            Some(set_gain) => set_gain(gain),
            None => 100,
        }
    }
}

/*
 * Sound queue stuff, the heart of the driver
 */

#[repr(C)]
pub struct sound_queue {
    /* buffers allocated for this queue */
    pub numBufs: c_int, /* real limits on what the user can have */
    pub bufSize: c_int, /* in bytes */
    pub buffers: *mut *mut c_char,

    /* current parameters */
    pub locked: c_int, /* params cannot be modified when != 0 */
    pub user_frags: c_int, /* user requests this many */
    pub user_frag_size: c_int, /* of this size */
    pub max_count: c_int, /* actual # fragments <= numBufs */
    pub block_size: c_int, /* internal block size in bytes */
    pub max_active: c_int, /* in-use fragments <= max_count */

    /* it shouldn't be necessary to declare any of these volatile */
    pub front: c_int,
    pub rear: c_int,
    pub count: c_int,
    pub rear_size: c_int,
    /*
     *  The use of the playing field depends on the hardware
     *
     *  Atari, PMac: The number of frames that are loaded/playing
     *
     *  Amiga: Bit 0 is set: a frame is loaded
     *         Bit 1 is set: a frame is playing
     */
    pub active: c_int,
    pub action_queue: wait_queue_head_t,
    pub open_queue: wait_queue_head_t,
    pub sync_queue: wait_queue_head_t,
    pub non_blocking: c_int,
    pub busy: c_int,
    pub syncing: c_int,
    pub xruns: c_int,
    pub died: c_int,
}

#[macro_export]
macro_rules! WAKE_UP {
    ($queue:expr) => {{
        unsafe { wake_up_interruptible(&mut $queue as *mut _) }
    }};
}

extern "C" {
    pub static mut dmasound_write_sq: sound_queue;
}

pub use dmasound_write_sq as write_sq;

extern "C" {
    pub static mut dmasound_catchRadius: c_int;
}

pub use dmasound_catchRadius as catchRadius;

/* define the value to be put in the byte-swap reg in mac-io
 * when we want it to swap for us.
 */
pub const BS_VAL: c_int = 1;

pub const SW_INPUT_VOLUME_SCALE: c_int = 4;
pub const SW_INPUT_VOLUME_DEFAULT: c_int = 128 / SW_INPUT_VOLUME_SCALE;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
