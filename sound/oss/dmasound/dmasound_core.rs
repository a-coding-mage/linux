// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/sound/oss/dmasound/dmasound_core.c
 *
 *
 *  OSS/Free compatible Atari TT/Falcon and Amiga DMA sound driver for
 *  Linux/m68k
 *  Extended to support Power Macintosh for Linux/ppc by Paul Mackerras
 *
 *  (c) 1995 by Michael Schlueter & Michael Marte
 *
 *  Michael Schlueter (michael@duck.syd.de) did the basic structure of the VFS
 *  interface and the u-law to signed byte conversion.
 *
 *  Michael Marte (marte@informatik.uni-muenchen.de) did the sound queue,
 *  /dev/mixer, /dev/sndstat and complemented the VFS interface. He would like
 *  to thank:
 *    - Michael Schlueter for initial ideas and documentation on the MFP and
 *	the DMA sound hardware.
 *    - Therapy? for their CD 'Troublegum' which really made me rock.
 *
 *  /dev/sndstat is based on code by Hannu Savolainen, the author of the
 *  VoxWare family of drivers.
 *
 *  History:
 *
 *	1995/8/25	First release
 *
 *	1995/9/02	Roman Hodek:
 *			  - Fixed atari_stram_alloc() call, the timer
 *			    programming and several race conditions
 *	1995/9/14	Roman Hodek:
 *			  - After some discussion with Michael Schlueter,
 *			    revised the interrupt disabling
 *			  - Slightly speeded up U8->S8 translation by using
 *			    long operations where possible
 *			  - Added 4:3 interpolation for /dev/audio
 *
 *	1995/9/20	Torsten Scherer:
 *			  - Fixed a bug in sq_write and changed /dev/audio
 *			    converting to play at 12517Hz instead of 6258Hz.
 *
 *	1995/9/23	Torsten Scherer:
 *			  - Changed sq_interrupt() and sq_play() to pre-program
 *			    the DMA for another frame while there's still one
 *			    running. This allows the IRQ response to be
 *			    arbitrarily delayed and playing will still continue.
 *
 *	1995/10/14	Guenther Kelleter, Torsten Scherer:
 *			  - Better support for Falcon audio (the Falcon doesn't
 *			    raise an IRQ at the end of a frame, but at the
 *			    beginning instead!). uses 'if (codec_dma)' in lots
 *			    of places to simply switch between Falcon and TT
 *			    code.
 *
 *	1995/11/06	Torsten Scherer:
 *			  - Started introducing a hardware abstraction scheme
 *			    (may perhaps also serve for Amigas?)
 *			  - Can now play samples at almost all frequencies by
 *			    means of a more generalized expand routine
 *			  - Takes a good deal of care to cut data only at
 *			    sample sizes
 *			  - Buffer size is now a kernel runtime option
 *			  - Implemented fsync() & several minor improvements
 *			Guenther Kelleter:
 *			  - Useful hints and bug fixes
 *			  - Cross-checked it for Falcons
 *
 *	1996/3/9	Geert Uytterhoeven:
 *			  - Support added for Amiga, A-law, 16-bit little
 *			    endian.
 *			  - Unification to drivers/sound/dmasound.c.
 *
 *	1996/4/6	Martin Mitchell:
 *			  - Updated to 1.3 kernel.
 *
 *	1996/6/13       Topi Kanerva:
 *			  - Fixed things that were broken (mainly the amiga
 *			    14-bit routines)
 *			  - /dev/sndstat shows now the real hardware frequency
 *			  - The lowpass filter is disabled by default now
 *
 *	1996/9/25	Geert Uytterhoeven:
 *			  - Modularization
 *
 *	1998/6/10	Andreas Schwab:
 *			  - Converted to use sound_core
 *
 *	1999/12/28	Richard Zidlicky:
 *			  - Added support for Q40
 *
 *	2000/2/27	Geert Uytterhoeven:
 *			  - Clean up and split the code into 4 parts:
 *			      o dmasound_core: machine-independent code
 *			      o dmasound_atari: Atari TT and Falcon support
 *			      o dmasound_awacs: Apple PowerMac support
 *			      o dmasound_paula: Amiga support
 *
 *	2000/3/25	Geert Uytterhoeven:
 *			  - Integration of dmasound_q40
 *			  - Small clean ups
 *
 *	2001/01/26 [1.0] Iain Sandoe
 *			  - make /dev/sndstat show revision & edition info.
 *			  - since dmasound.mach.sq_setup() can fail on pmac
 *			    its type has been changed to int and the returns
 *			    are checked.
 *		   [1.1]  - stop missing translations from being called.
 *	2001/02/08 [1.2]  - remove unused translation tables & move machine-
 *			    specific tables to low-level.
 *			  - return correct info. for SNDCTL_DSP_GETFMTS.
 *		   [1.3]  - implement SNDCTL_DSP_GETCAPS fully.
 *		   [1.4]  - make /dev/sndstat text length usage deterministic.
 *			  - make /dev/sndstat call to low-level
 *			    dmasound.mach.state_info() pass max space to ll driver.
 *			  - tidy startup banners and output info.
 *		   [1.5]  - tidy up a little (removed some unused #defines in
 *			    dmasound.h)
 *			  - fix up HAS_RECORD conditionalisation.
 *			  - add record code in places it is missing...
 *			  - change buf-sizes to bytes to allow < 1kb for pmac
 *			    if user param entry is < 256 the value is taken to
 *			    be in kb > 256 is taken to be in bytes.
 *			  - make default buff/frag params conditional on
 *			    machine to allow smaller values for pmac.
 *			  - made the ioctls, read & write comply with the OSS
 *			    rules on setting params.
 *			  - added parsing of _setup() params for record.
 *	2001/04/04 [1.6]  - fix bug where sample rates higher than maximum were
 *			    being reported as OK.
 *			  - fix open() to return -EBUSY as per OSS doc. when
 *			    audio is in use - this is independent of O_NOBLOCK.
 *			  - fix bug where SNDCTL_DSP_POST was blocking.
 */

/*
 * Record capability notes 30/01/2001:
 * At present these observations apply only to pmac LL driver (the only one
 * that can do record, at present).  However, if other LL drivers for machines
 * with record are added they may apply.
 *
 * The fragment parameters for the record and play channels are separate.
 * However, if the driver is opened O_RDWR there is no way (in the current OSS
 * API) to specify their values independently for the record and playback
 * channels.  Since the only common factor between the input & output is the
 * sample rate (on pmac) it should be possible to open /dev/dspX O_WRONLY and
 * /dev/dspY O_RDONLY.  The input & output channels could then have different
 * characteristics (other than the first that sets sample rate claiming the
 * right to set it for ever).  As it stands, the format, channels, number of
 * bits & sample rate are assumed to be common.  In the future perhaps these
 * should be the responsibility of the LL driver - and then if a card really
 * does not share items between record & playback they can be specified
 * separately.
 */

/* Thread-safeness of shared_resources notes: 31/01/2001
 * If the user opens O_RDWR and then splits record & play between two threads
 * both of which inherit the fd - and then starts changing things from both
 * - we will have difficulty telling.
 *
 * It's bad application coding - but ...
 * TODO: think about how to sort this out... without bogging everything down in
 * semaphores.
 *
 * Similarly, the OSS spec says "all changes to parameters must be between
 * open() and the first read() or write(). - and a bit later on (by
 * implication) "between SNDCTL_DSP_RESET and the first read() or write() after
 * it".  If the app is multi-threaded and this rule is broken between threads
 * we will have trouble spotting it - and the fault will be rather obscure :-(
 *
 * We will try and put out at least a kmsg if we see it happen... but I think
 * it will be quite hard to trap it with an -EXXX return... because we can't
 * see the fault until after the damage is done.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const DMASOUND_CORE_REVISION: c_int = 1;
const DMASOUND_CORE_EDITION: c_int = 6;
const STAT_BUFF_LEN: usize = 768;
const LOW_LEVEL_STAT_ALLOC: usize = 162;

type ssize_t = isize;
type size_t = usize;
type loff_t = i64;
type fmode_t = c_uint;
type u_char = u8;
type u_int = c_uint;
type u_long = c_ulong;
type __poll_t = c_uint;

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    pub f_mode: fmode_t,
    pub f_flags: c_int,
}

#[repr(C)]
pub struct poll_table_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audio_buf_info {
    pub fragments: c_int,
    pub fragstotal: c_int,
    pub fragsize: c_int,
    pub bytes: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mixer_info {
    pub id: [c_char; 16],
    pub name: [c_char; 32],
    pub modify_counter: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sound_params {
    pub format: c_int,
    pub stereo: c_int,
    pub size: c_int,
    pub speed: c_int,
}

type CtFunc = Option<
    unsafe extern "C" fn(
        *const u_char,
        size_t,
        *mut u_char,
        *mut ssize_t,
        ssize_t,
    ) -> ssize_t,
>;

#[repr(C)]
pub struct TRANS {
    pub ct_ulaw: CtFunc,
    pub ct_alaw: CtFunc,
    pub ct_s8: CtFunc,
    pub ct_u8: CtFunc,
    pub ct_s16be: CtFunc,
    pub ct_u16be: CtFunc,
    pub ct_s16le: CtFunc,
    pub ct_u16le: CtFunc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mach_settings {
    pub owner: *mut module,
    pub name: *const c_char,
    pub name2: *const c_char,
    pub version: c_int,
    pub max_dsp_speed: c_int,
    pub hardware_afmts: u_long,
    pub capabilities: c_int,
    pub default_soft: sound_params,
    pub default_hard: sound_params,
    pub silence: unsafe extern "C" fn(),
    pub setFormat: unsafe extern "C" fn(c_int) -> c_int,
    pub init: unsafe extern "C" fn(),
    pub play: unsafe extern "C" fn(),
    pub dma_alloc: unsafe extern "C" fn(c_int, c_int) -> *mut c_void,
    pub dma_free: unsafe extern "C" fn(*mut c_void, c_int),
    pub write_sq_setup: Option<unsafe extern "C" fn() -> c_int>,
    pub sq_open: Option<unsafe extern "C" fn(fmode_t)>,
    pub mixer_ioctl: Option<unsafe extern "C" fn(u_int, u_long) -> c_int>,
    pub mixer_init: Option<unsafe extern "C" fn()>,
    pub state_info: Option<unsafe extern "C" fn(*mut c_char, size_t) -> c_int>,
    pub irqinit: unsafe extern "C" fn() -> c_int,
    pub irqcleanup: unsafe extern "C" fn(),
}

#[repr(C)]
pub struct sound_settings {
    pub lock: spinlock_t,
    pub mach: mach_settings,
    pub soft: sound_params,
    pub hard: sound_params,
    pub dsp: sound_params,
    pub minDev: c_int,
    pub trans_write: *mut TRANS,
    pub treble: c_int,
    pub bass: c_int,
}

#[repr(C)]
pub struct sound_queue {
    pub buffers: *mut *mut u_char,
    pub numBufs: c_int,
    pub bufSize: c_int,
    pub locked: c_int,
    pub user_frags: c_int,
    pub max_count: c_int,
    pub max_active: c_int,
    pub block_size: c_int,
    pub user_frag_size: c_int,
    pub front: c_int,
    pub count: c_int,
    pub rear_size: c_int,
    pub syncing: c_int,
    pub active: c_int,
    pub rear: c_int,
    pub action_queue: wait_queue_head_t,
    pub open_queue: wait_queue_head_t,
    pub sync_queue: wait_queue_head_t,
    pub busy: c_int,
    pub non_blocking: c_int,
    pub xruns: c_int,
}

#[repr(C)]
pub struct file_operations {
    pub owner: *mut module,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t>,
    pub poll: Option<unsafe extern "C" fn(*mut file, *mut poll_table_struct) -> __poll_t>,
    pub unlocked_ioctl: Option<unsafe extern "C" fn(*mut file, u_int, u_long) -> c_long>,
    pub compat_ioctl: Option<unsafe extern "C" fn(*mut file, u_int, u_long) -> c_long>,
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;
    static mut current: *mut c_void;

    static mut DEFAULT_N_BUFFERS: c_uint;
    static mut DEFAULT_BUFF_SIZE: c_uint;
    static mut GFP_KERNEL: c_int;
    static mut SND_DEV_DSP: c_int;
    static mut SND_DEV_AUDIO: c_int;
    static mut SND_DEV_STATUS: c_int;
    static mut SOUND_VERSION: c_int;
    static mut OSS_GETVERSION: u_int;
    static mut SOUND_MIXER_INFO: u_int;
    static mut SNDCTL_DSP_RESET: u_int;
    static mut SNDCTL_DSP_GETFMTS: u_int;
    static mut SNDCTL_DSP_GETBLKSIZE: u_int;
    static mut SNDCTL_DSP_POST: u_int;
    static mut SNDCTL_DSP_SYNC: u_int;
    static mut SOUND_PCM_READ_RATE: u_int;
    static mut SNDCTL_DSP_SPEED: u_int;
    static mut SNDCTL_DSP_STEREO: u_int;
    static mut SOUND_PCM_WRITE_CHANNELS: u_int;
    static mut SNDCTL_DSP_SETFMT: u_int;
    static mut SNDCTL_DSP_SUBDIVIDE: u_int;
    static mut SNDCTL_DSP_SETFRAGMENT: u_int;
    static mut SNDCTL_DSP_GETOSPACE: u_int;
    static mut SNDCTL_DSP_GETCAPS: u_int;
    static mut AFMT_MU_LAW: c_int;
    static mut AFMT_A_LAW: c_int;
    static mut AFMT_S8: c_int;
    static mut AFMT_U8: c_int;
    static mut AFMT_S16_BE: c_int;
    static mut AFMT_U16_BE: c_int;
    static mut AFMT_S16_LE: c_int;
    static mut AFMT_U16_LE: c_int;
    static mut AFMT_QUERY: c_int;
    static mut FMODE_WRITE: fmode_t;
    static mut FMODE_READ: fmode_t;
    static mut O_NONBLOCK: c_int;
    static mut ENODEV: c_int;
    static mut ENOMEM: c_int;
    static mut EINVAL: c_int;
    static mut EBUSY: c_int;
    static mut EAGAIN: c_int;
    static mut EINTR: c_int;
    static mut ENXIO: c_int;
    static mut EFAULT: c_int;
    static mut EIO: c_int;
    static mut EPOLLOUT: __poll_t;
    static mut EPOLLWRNORM: __poll_t;
    static mut TASK_INTERRUPTIBLE: c_int;
    static mut HZ: c_long;
    static mut MIN_FRAG_SIZE: c_int;
    static mut MAX_FRAG_SIZE: c_int;
    static mut MIN_BUFFERS: c_uint;
    static mut MIN_BUFSIZE: c_int;
    static mut MAX_BUFSIZE: c_int;
    static mut MAX_CATCH_RADIUS: c_int;

    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
    fn try_module_get(module: *mut module) -> c_int;
    fn module_put(module: *mut module);
    fn register_sound_mixer(fops: *const file_operations, dev: c_int) -> c_int;
    fn unregister_sound_mixer(unit: c_int);
    fn register_sound_dsp(fops: *const file_operations, dev: c_int) -> c_int;
    fn unregister_sound_dsp(unit: c_int);
    fn register_sound_special(fops: *const file_operations, dev: c_int) -> c_int;
    fn unregister_sound_special(unit: c_int);
    fn compat_ptr_ioctl(file: *mut file, cmd: u_int, arg: u_long) -> c_long;
    fn kmalloc_array(n: c_int, size: usize, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn init_waitqueue_head(queue: *mut wait_queue_head_t);
    fn poll_wait(file: *mut file, queue: *mut wait_queue_head_t, wait: *mut poll_table_struct);
    fn prepare_to_wait(queue: *mut wait_queue_head_t, wait: *mut c_void, state: c_int);
    fn finish_wait(queue: *mut wait_queue_head_t, wait: *mut c_void);
    fn schedule_timeout(timeout: c_long) -> c_long;
    fn signal_pending(task: *mut c_void) -> c_int;
    fn wait_event_interruptible_timeout(queue: wait_queue_head_t, condition: c_int, timeout: c_long) -> c_long;
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn printk(fmt: *const c_char, ...) -> c_int;
    fn iminor(inode: *mut inode) -> c_int;
    fn get_options(str: *mut c_char, nints: c_int, ints: *mut c_int) -> *mut c_char;
    fn _SIOC_DIR(cmd: u_int) -> u_int;
    fn IOCTL_OUT(arg: u_long, val: u_long) -> c_int;
    fn IOCTL_IN(arg: u_long, val: *mut c_int) -> c_int;
    fn ARRAY_SIZE_ints_6(ints: *mut c_int) -> c_int;
}

// Module parameters from the C source are represented as mutable globals.
static mut dmasound_core_mutex: mutex = mutex { _private: [] };
#[no_mangle]
pub static mut dmasound_catchRadius: c_int = 0;
static mut numWriteBufs: c_uint = 0;
static mut writeBufSize: c_uint = 0;

static mut sq_unit: c_int = -1;
static mut mixer_unit: c_int = -1;
static mut state_unit: c_int = -1;
static mut irq_installed: c_int = 0;

/* control over who can modify resources shared between play/record */
static mut shared_resource_owner: fmode_t = 0;
static mut shared_resources_initialised: c_int = 0;

#[no_mangle]
pub static mut dmasound: sound_settings = sound_settings {
    lock: spinlock_t { _private: [] },
    mach: mach_settings {
        owner: ptr::null_mut(),
        name: ptr::null(),
        name2: ptr::null(),
        version: 0,
        max_dsp_speed: 0,
        hardware_afmts: 0,
        capabilities: 0,
        default_soft: sound_params { format: 0, stereo: 0, size: 0, speed: 0 },
        default_hard: sound_params { format: 0, stereo: 0, size: 0, speed: 0 },
        silence: empty_void,
        setFormat: empty_set_format,
        init: empty_void,
        play: empty_void,
        dma_alloc: empty_dma_alloc,
        dma_free: empty_dma_free,
        write_sq_setup: None,
        sq_open: None,
        mixer_ioctl: None,
        mixer_init: None,
        state_info: None,
        irqinit: empty_irqinit,
        irqcleanup: empty_void,
    },
    soft: sound_params { format: 0, stereo: 0, size: 0, speed: 0 },
    hard: sound_params { format: 0, stereo: 0, size: 0, speed: 0 },
    dsp: sound_params { format: 0, stereo: 0, size: 0, speed: 0 },
    minDev: 0,
    trans_write: ptr::null_mut(),
    treble: 0,
    bass: 0,
};

unsafe extern "C" fn empty_void() {}
unsafe extern "C" fn empty_set_format(format: c_int) -> c_int { format }
unsafe extern "C" fn empty_dma_alloc(_size: c_int, _flags: c_int) -> *mut c_void { ptr::null_mut() }
unsafe extern "C" fn empty_dma_free(_ptr: *mut c_void, _size: c_int) {}
unsafe extern "C" fn empty_irqinit() -> c_int { 0 }

unsafe fn sound_silence() {
    (dmasound.mach.silence)(); /* _MUST_ stop DMA */
}

unsafe fn sound_set_format(format: c_int) -> c_int {
    (dmasound.mach.setFormat)(format)
}

unsafe fn sound_set_speed(mut speed: c_int) -> c_int {
    if speed < 0 {
        return dmasound.soft.speed;
    }

    /* trap out-of-range speed settings.
       at present we allow (arbitrarily) low rates - using soft
       up-conversion - but we can't allow > max because there is
       no soft down-conversion.
    */
    if dmasound.mach.max_dsp_speed != 0 && speed > dmasound.mach.max_dsp_speed {
        speed = dmasound.mach.max_dsp_speed;
    }

    dmasound.soft.speed = speed;

    if dmasound.minDev == SND_DEV_DSP {
        dmasound.dsp.speed = dmasound.soft.speed;
    }

    dmasound.soft.speed
}

unsafe fn sound_set_stereo(mut stereo: c_int) -> c_int {
    if stereo < 0 {
        return dmasound.soft.stereo;
    }

    stereo = if stereo != 0 { 1 } else { 0 };    /* should be 0 or 1 now */

    dmasound.soft.stereo = stereo;
    if dmasound.minDev == SND_DEV_DSP {
        dmasound.dsp.stereo = stereo;
    }

    stereo
}

unsafe fn sound_copy_translate(
    trans: *mut TRANS,
    userPtr: *const u_char,
    userCount: size_t,
    frame: *mut u_char,
    frameUsed: *mut ssize_t,
    frameLeft: ssize_t,
) -> ssize_t {
    let ct_func: CtFunc;

    if trans.is_null() {
        return 0;
    }

    match dmasound.soft.format {
        x if x == AFMT_MU_LAW => ct_func = (*trans).ct_ulaw,
        x if x == AFMT_A_LAW => ct_func = (*trans).ct_alaw,
        x if x == AFMT_S8 => ct_func = (*trans).ct_s8,
        x if x == AFMT_U8 => ct_func = (*trans).ct_u8,
        x if x == AFMT_S16_BE => ct_func = (*trans).ct_s16be,
        x if x == AFMT_U16_BE => ct_func = (*trans).ct_u16be,
        x if x == AFMT_S16_LE => ct_func = (*trans).ct_s16le,
        x if x == AFMT_U16_LE => ct_func = (*trans).ct_u16le,
        _ => return 0,
    }
    /* if the user has requested a non-existent translation don't try
       to call it but just return 0 bytes moved
    */
    if let Some(func) = ct_func {
        return func(userPtr, userCount, frame, frameUsed, frameLeft);
    }
    0
}

#[repr(C)]
struct MixerState {
    busy: c_int,
    modify_counter: c_int,
}

static mut mixer: MixerState = MixerState { busy: 0, modify_counter: 0 };

unsafe extern "C" fn mixer_open(_inode: *mut inode, _file: *mut file) -> c_int {
    mutex_lock(&raw mut dmasound_core_mutex);
    if try_module_get(dmasound.mach.owner) == 0 {
        mutex_unlock(&raw mut dmasound_core_mutex);
        return -ENODEV;
    }
    mixer.busy = 1;
    mutex_unlock(&raw mut dmasound_core_mutex);
    0
}

unsafe extern "C" fn mixer_release(_inode: *mut inode, _file: *mut file) -> c_int {
    mutex_lock(&raw mut dmasound_core_mutex);
    mixer.busy = 0;
    module_put(dmasound.mach.owner);
    mutex_unlock(&raw mut dmasound_core_mutex);
    0
}

unsafe fn mixer_ioctl(_file: *mut file, cmd: u_int, arg: u_long) -> c_int {
    if (_SIOC_DIR(cmd) & 1) != 0 {
        mixer.modify_counter += 1;
    }
    if cmd == OSS_GETVERSION {
        return IOCTL_OUT(arg, SOUND_VERSION as u_long);
    }
    if cmd == SOUND_MIXER_INFO {
        let mut info: mixer_info = core::mem::zeroed();
        memset(&mut info as *mut _ as *mut c_void, 0, size_of::<mixer_info>());
        strscpy(info.id.as_mut_ptr(), dmasound.mach.name2, info.id.len());
        strscpy(info.name.as_mut_ptr(), dmasound.mach.name2, info.name.len());
        info.modify_counter = mixer.modify_counter;
        if copy_to_user(arg as *mut c_void, &info as *const _ as *const c_void, size_of::<mixer_info>()) != 0 {
            return -EFAULT;
        }
        return 0;
    }
    if let Some(func) = dmasound.mach.mixer_ioctl {
        return func(cmd, arg);
    }
    -EINVAL
}

unsafe extern "C" fn mixer_unlocked_ioctl(file: *mut file, cmd: u_int, arg: u_long) -> c_long {
    let ret: c_int;

    mutex_lock(&raw mut dmasound_core_mutex);
    ret = mixer_ioctl(file, cmd, arg);
    mutex_unlock(&raw mut dmasound_core_mutex);

    ret as c_long
}

static mixer_fops: file_operations = file_operations {
    owner: ptr::null_mut(),
    read: None,
    write: None,
    poll: None,
    unlocked_ioctl: Some(mixer_unlocked_ioctl),
    compat_ioctl: Some(compat_ptr_ioctl),
    open: Some(mixer_open),
    release: Some(mixer_release),
};

unsafe fn mixer_init() {
    mixer_unit = register_sound_mixer(&mixer_fops, -1);
    if mixer_unit < 0 {
        return;
    }

    mixer.busy = 0;
    dmasound.treble = 0;
    dmasound.bass = 0;
    if let Some(func) = dmasound.mach.mixer_init {
        func();
    }
}

#[no_mangle]
pub static mut dmasound_write_sq: sound_queue = sound_queue {
    buffers: ptr::null_mut(),
    numBufs: 0,
    bufSize: 0,
    locked: 0,
    user_frags: 0,
    max_count: 0,
    max_active: 0,
    block_size: 0,
    user_frag_size: 0,
    front: 0,
    count: 0,
    rear_size: 0,
    syncing: 0,
    active: 0,
    rear: 0,
    action_queue: wait_queue_head_t { _private: [] },
    open_queue: wait_queue_head_t { _private: [] },
    sync_queue: wait_queue_head_t { _private: [] },
    busy: 0,
    non_blocking: 0,
    xruns: 0,
};

unsafe fn write_sq() -> *mut sound_queue {
    &raw mut dmasound_write_sq
}

unsafe fn sq_allocate_buffers(sq: *mut sound_queue, num: c_int, size: c_int) -> c_int {
    let mut i: c_int;

    if !(*sq).buffers.is_null() {
        return 0;
    }
    (*sq).numBufs = num;
    (*sq).bufSize = size;
    (*sq).buffers = kmalloc_array(num, size_of::<*mut c_char>(), GFP_KERNEL) as *mut *mut u_char;
    if (*sq).buffers.is_null() {
        return -ENOMEM;
    }
    i = 0;
    while i < num {
        *(*sq).buffers.offset(i as isize) = (dmasound.mach.dma_alloc)(size, GFP_KERNEL) as *mut u_char;
        if (*(*sq).buffers.offset(i as isize)).is_null() {
            while {
                i -= 1;
                i >= 0
            } {
                (dmasound.mach.dma_free)(*(*sq).buffers.offset(i as isize) as *mut c_void, size);
            }
            kfree((*sq).buffers as *mut c_void);
            (*sq).buffers = ptr::null_mut();
            return -ENOMEM;
        }
        i += 1;
    }
    0
}

unsafe fn sq_release_buffers(sq: *mut sound_queue) {
    let mut i: c_int;

    if !(*sq).buffers.is_null() {
        i = 0;
        while i < (*sq).numBufs {
            (dmasound.mach.dma_free)(*(*sq).buffers.offset(i as isize) as *mut c_void, (*sq).bufSize);
            i += 1;
        }
        kfree((*sq).buffers as *mut c_void);
        (*sq).buffers = ptr::null_mut();
    }
}

unsafe fn sq_setup(sq: *mut sound_queue) -> c_int {
    let mut setup_func: Option<unsafe extern "C" fn() -> c_int> = None;
    let hard_frame: c_int;

    if (*sq).locked != 0 {
        /* DEBUG_DMASOUND: printk("dmasound_core: tried to sq_setup a locked queue\n"); */
        return -EINVAL;
    }
    (*sq).locked = 1; /* don't think we have a race prob. here _check_ */

    /* make sure that the parameters are set up
       This should have been done already...
    */

    (dmasound.mach.init)();

    /* OK.  If the user has set fragment parameters explicitly, then we
       should leave them alone... as long as they are valid.
       Invalid user fragment params can occur if we allow the whole buffer
       to be used when the user requests the fragments sizes (with no soft
       x-lation) and then the user subsequently sets a soft x-lation that
       requires increased internal buffering.

       Othwerwise (if the user did not set them) OSS says that we should
       select frag params on the basis of 0.5 s output & 0.1 s input
       latency. (TODO.  For now we will copy in the defaults.)
    */

    if (*sq).user_frags <= 0 {
        (*sq).max_count = (*sq).numBufs;
        (*sq).max_active = (*sq).numBufs;
        (*sq).block_size = (*sq).bufSize;
        /* set up the user info */
        (*sq).user_frags = (*sq).numBufs;
        (*sq).user_frag_size = (*sq).bufSize;
        (*sq).user_frag_size *= dmasound.soft.size * (dmasound.soft.stereo + 1);
        (*sq).user_frag_size /= dmasound.hard.size * (dmasound.hard.stereo + 1);
    } else {
        /* work out requested block size */
        (*sq).block_size = (*sq).user_frag_size;
        (*sq).block_size *= dmasound.hard.size * (dmasound.hard.stereo + 1);
        (*sq).block_size /= dmasound.soft.size * (dmasound.soft.stereo + 1);
        /* the user wants to write frag-size chunks */
        (*sq).block_size *= dmasound.hard.speed;
        (*sq).block_size /= dmasound.soft.speed;
        /* this only works for size values which are powers of 2 */
        hard_frame = (dmasound.hard.size * (dmasound.hard.stereo + 1)) / 8;
        (*sq).block_size += hard_frame - 1;
        (*sq).block_size &= !(hard_frame - 1); /* make sure we are aligned */
        /* let's just check for obvious mistakes */
        if (*sq).block_size <= 0 || (*sq).block_size > (*sq).bufSize {
            /* DEBUG_DMASOUND: printk("dmasound_core: invalid frag size (user set %d)\n", sq->user_frag_size); */
            (*sq).block_size = (*sq).bufSize;
        }
        if (*sq).user_frags <= (*sq).numBufs {
            (*sq).max_count = (*sq).user_frags;
            /* if user has set max_active - then use it */
            (*sq).max_active = if (*sq).max_active <= (*sq).max_count {
                (*sq).max_active
            } else {
                (*sq).max_count
            };
        } else {
            /* DEBUG_DMASOUND: printk("dmasound_core: invalid frag count (user set %d)\n", sq->user_frags); */
            (*sq).max_count = (*sq).numBufs;
            (*sq).max_active = (*sq).numBufs;
        }
    }
    (*sq).front = 0;
    (*sq).count = 0;
    (*sq).rear_size = 0;
    (*sq).syncing = 0;
    (*sq).active = 0;

    if sq == write_sq() {
        (*sq).rear = -1;
        setup_func = dmasound.mach.write_sq_setup;
    }
    if let Some(func) = setup_func {
        return func();
    }
    0
}

unsafe fn sq_play() {
    (dmasound.mach.play)();
}

unsafe extern "C" fn sq_write(file: *mut file, mut src: *const c_char, mut uLeft: size_t, _ppos: *mut loff_t) -> ssize_t {
    let mut uWritten: ssize_t = 0;
    let mut dest: *mut u_char;
    let mut uUsed: ssize_t = 0;
    let mut bUsed: ssize_t;
    let mut bLeft: ssize_t;
    let mut flags: c_ulong = 0;
    let _ = file;

    if uLeft == 0 {
        return 0;
    }

    if shared_resources_initialised == 0 {
        (dmasound.mach.init)();
        shared_resources_initialised = 1;
    }

    if (*write_sq()).locked == 0 {
        uWritten = sq_setup(write_sq()) as ssize_t;
        if uWritten < 0 {
            return uWritten;
        }
        uWritten = 0;
    }

    spin_lock_irqsave(&raw mut dmasound.lock, &mut flags);
    (*write_sq()).syncing &= !2; /* take out POST status */
    spin_unlock_irqrestore(&raw mut dmasound.lock, flags);

    sq_play();

    bLeft = ((*write_sq()).block_size - (*write_sq()).rear_size) as ssize_t;
    if (*write_sq()).count > 0 && bLeft > 0 {
        dest = *(*write_sq()).buffers.offset((*write_sq()).rear as isize);
        bUsed = (*write_sq()).rear_size as ssize_t;
        uUsed = sound_copy_translate(dmasound.trans_write, src as *const u_char, uLeft, dest, &mut bUsed, bLeft);
        if uUsed <= 0 {
            return uUsed;
        }
        src = src.offset(uUsed as isize);
        uWritten += uUsed;
        uLeft = if (uUsed as size_t) <= uLeft { uLeft - uUsed as size_t } else { 0 }; /* paranoia */
        (*write_sq()).rear_size = bUsed as c_int;
    }

    while uLeft != 0 {
        let mut wait: [u8; 0] = [];

        while (*write_sq()).count >= (*write_sq()).max_active {
            prepare_to_wait(&raw mut (*write_sq()).action_queue, wait.as_mut_ptr() as *mut c_void, TASK_INTERRUPTIBLE);
            sq_play();
            if (*write_sq()).non_blocking != 0 {
                finish_wait(&raw mut (*write_sq()).action_queue, wait.as_mut_ptr() as *mut c_void);
                return if uWritten > 0 { uWritten } else { -EAGAIN as ssize_t };
            }
            if (*write_sq()).count < (*write_sq()).max_active {
                break;
            }

            schedule_timeout(HZ);
            if signal_pending(current) != 0 {
                finish_wait(&raw mut (*write_sq()).action_queue, wait.as_mut_ptr() as *mut c_void);
                return if uWritten > 0 { uWritten } else { -EINTR as ssize_t };
            }
        }

        finish_wait(&raw mut (*write_sq()).action_queue, wait.as_mut_ptr() as *mut c_void);

        dest = *(*write_sq()).buffers.offset((((*write_sq()).rear + 1) % (*write_sq()).max_count) as isize);
        bUsed = 0;
        bLeft = (*write_sq()).block_size as ssize_t;
        uUsed = sound_copy_translate(dmasound.trans_write, src as *const u_char, uLeft, dest, &mut bUsed, bLeft);
        if uUsed <= 0 {
            break;
        }
        src = src.offset(uUsed as isize);
        uWritten += uUsed;
        uLeft = if (uUsed as size_t) <= uLeft { uLeft - uUsed as size_t } else { 0 }; /* paranoia */
        if bUsed != 0 {
            (*write_sq()).rear = ((*write_sq()).rear + 1) % (*write_sq()).max_count;
            (*write_sq()).rear_size = bUsed as c_int;
            (*write_sq()).count += 1;
        }
    } /* uUsed may have been 0 */

    sq_play();

    if uUsed < 0 { uUsed } else { uWritten }
}

unsafe extern "C" fn sq_poll(file: *mut file, wait: *mut poll_table_struct) -> __poll_t {
    let mut mask: __poll_t = 0;
    let retVal: c_int;

    if (*write_sq()).locked == 0 {
        retVal = sq_setup(write_sq());
        if retVal < 0 {
            return retVal as __poll_t;
        }
        return 0;
    }
    if ((*file).f_mode & FMODE_WRITE) != 0 {
        poll_wait(file, &raw mut (*write_sq()).action_queue, wait);
    }
    if ((*file).f_mode & FMODE_WRITE) != 0 {
        if (*write_sq()).count < (*write_sq()).max_active || (*write_sq()).block_size - (*write_sq()).rear_size > 0 {
            mask |= EPOLLOUT | EPOLLWRNORM;
        }
    }
    mask
}

unsafe fn sq_init_waitqueue(sq: *mut sound_queue) {
    init_waitqueue_head(&raw mut (*sq).action_queue);
    init_waitqueue_head(&raw mut (*sq).open_queue);
    init_waitqueue_head(&raw mut (*sq).sync_queue);
    (*sq).busy = 0;
}

/* #if 0 blocking open(): sq_wake_up omitted as disabled C code. */

unsafe fn sq_open2(sq: *mut sound_queue, file: *mut file, mode: fmode_t, numbufs: c_int, bufsize: c_int) -> c_int {
    let mut rc: c_int = 0;

    if ((*file).f_mode & mode) != 0 {
        if (*sq).busy != 0 {
            /* OSS manual says we will return EBUSY regardless
               of O_NOBLOCK.
            */
            return -EBUSY;
        }
        (*sq).busy = 1; /* Let's play spot-the-race-condition */

        /* allocate the default number & size of buffers.
           (i.e. specified in _setup() or as module params)
           can't be changed at the moment - but _could_ be perhaps
           in the setfragments ioctl.
        */
        rc = sq_allocate_buffers(sq, numbufs, bufsize);
        if rc != 0 {
            (*sq).busy = 0;
            return rc;
        }

        (*sq).non_blocking = (*file).f_flags & O_NONBLOCK;
    }
    rc
}

unsafe fn write_sq_init_waitqueue() {
    sq_init_waitqueue(write_sq());
}

unsafe fn write_sq_release_buffers() {
    sq_release_buffers(write_sq());
}

unsafe fn write_sq_open(file: *mut file) -> c_int {
    sq_open2(write_sq(), file, FMODE_WRITE, numWriteBufs as c_int, writeBufSize as c_int)
}

unsafe extern "C" fn sq_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut rc: c_int;

    mutex_lock(&raw mut dmasound_core_mutex);
    if try_module_get(dmasound.mach.owner) == 0 {
        mutex_unlock(&raw mut dmasound_core_mutex);
        return -ENODEV;
    }

    rc = write_sq_open(file); /* checks the f_mode */
    if rc != 0 {
        goto_out(rc);
        module_put(dmasound.mach.owner);
        mutex_unlock(&raw mut dmasound_core_mutex);
        return rc;
    }
    if ((*file).f_mode & FMODE_READ) != 0 {
        /* TODO: if O_RDWR, release any resources grabbed by write part */
        rc = -ENXIO; /* I think this is what is required by open(2) */
        module_put(dmasound.mach.owner);
        mutex_unlock(&raw mut dmasound_core_mutex);
        return rc;
    }

    if let Some(func) = dmasound.mach.sq_open {
        func((*file).f_mode);
    }

    dmasound.minDev = iminor(inode) & 0x0f;

    if shared_resource_owner == 0 {
        dmasound.soft = dmasound.mach.default_soft;
        dmasound.dsp = dmasound.mach.default_soft;
        dmasound.hard = dmasound.mach.default_hard;
    }

    /* !DMASOUND_STRICT_OSS_COMPLIANCE */
    if dmasound.minDev == SND_DEV_AUDIO {
        sound_set_speed(8000);
        sound_set_stereo(0);
        sound_set_format(AFMT_MU_LAW);
    }

    mutex_unlock(&raw mut dmasound_core_mutex);
    0
}

unsafe fn goto_out(_rc: c_int) {}

unsafe fn sq_reset_output() {
    sound_silence(); /* this _must_ stop DMA, we might be about to lose the buffers */
    (*write_sq()).active = 0;
    (*write_sq()).count = 0;
    (*write_sq()).rear_size = 0;
    /* write_sq.front = (write_sq.rear+1) % write_sq.max_count;*/
    (*write_sq()).front = 0;
    (*write_sq()).rear = -1; /* same as for set-up */

    /* OK - we can unlock the parameters and fragment settings */
    (*write_sq()).locked = 0;
    (*write_sq()).user_frags = 0;
    (*write_sq()).user_frag_size = 0;
}

unsafe fn sq_reset() {
    sq_reset_output();
    /* we could consider resetting the shared_resources_owner here... but I
       think it is probably still rather non-obvious to application writer
    */

    /* we release everything else though */
    shared_resources_initialised = 0;
}

unsafe fn sq_fsync() -> c_int {
    let mut rc: c_int = 0;
    let mut timeout: c_int = 5;

    (*write_sq()).syncing |= 1;
    sq_play();	/* there may be an incomplete frame waiting */

    while (*write_sq()).active != 0 {
        wait_event_interruptible_timeout((*write_sq()).sync_queue, ((*write_sq()).active == 0) as c_int, HZ);
        if signal_pending(current) != 0 {
            /* While waiting for audio output to drain, an
             * interrupt occurred.  Stop audio output immediately
             * and clear the queue. */
            sq_reset_output();
            rc = -EINTR;
            break;
        }
        timeout -= 1;
        if timeout == 0 {
            printk(c"dmasound: Timeout draining output\n".as_ptr());
            sq_reset_output();
            rc = -EIO;
            break;
        }
    }

    /* flag no sync regardless of whether we had a DSP_POST or not */
    (*write_sq()).syncing = 0;
    rc
}

unsafe extern "C" fn sq_release(_inode: *mut inode, file: *mut file) -> c_int {
    let mut rc: c_int = 0;

    mutex_lock(&raw mut dmasound_core_mutex);

    if ((*file).f_mode & FMODE_WRITE) != 0 {
        if (*write_sq()).busy != 0 {
            rc = sq_fsync();
        }

        sq_reset_output(); /* make sure dma is stopped and all is quiet */
        write_sq_release_buffers();
        (*write_sq()).busy = 0;
    }

    if ((*file).f_mode & shared_resource_owner) != 0 { /* it's us that has them */
        shared_resource_owner = 0;
        shared_resources_initialised = 0;
        dmasound.hard = dmasound.mach.default_hard;
    }

    module_put(dmasound.mach.owner);

    /* #if 0 blocking open() wake-up code was disabled in C. */

    mutex_unlock(&raw mut dmasound_core_mutex);

    rc
}

/* here we see if we have a right to modify format, channels, size and so on
   if no-one else has claimed it already then we do...

   TODO: We might change this to mask O_RDWR such that only one or the other channel
   is the owner - if we have problems.
*/

unsafe fn shared_resources_are_mine(md: fmode_t) -> c_int {
    if shared_resource_owner != 0 {
        ((shared_resource_owner & md) != 0) as c_int
    } else {
        shared_resource_owner = md;
        1
    }
}

/* if either queue is locked we must deny the right to change shared params
*/

unsafe fn queues_are_quiescent() -> c_int {
    if (*write_sq()).locked != 0 {
        return 0;
    }
    1
}

unsafe fn set_queue_frags(sq: *mut sound_queue, mut bufs: c_int, mut size: c_int) -> c_int {
    if (*sq).locked != 0 {
        /* DEBUG_DMASOUND: printk("dmasound_core: tried to set_queue_frags on a locked queue\n"); */
        return -EINVAL;
    }

    if size < MIN_FRAG_SIZE || size > MAX_FRAG_SIZE {
        return -EINVAL;
    }
    size = 1 << size; /* now in bytes */
    if size > (*sq).bufSize {
        return -EINVAL; /* this might still not work */
    }

    if bufs <= 0 {
        return -EINVAL;
    }
    if bufs > (*sq).numBufs { /* the user is allowed say "don't care" with 0x7fff */
        bufs = (*sq).numBufs;
    }

    (*sq).user_frags = bufs;
    (*sq).max_active = bufs;
    (*sq).user_frag_size = size;

    0
}

unsafe fn sq_ioctl(file: *mut file, cmd: u_int, arg: u_long) -> c_int {
    let mut val: c_int;
    let mut result: c_int;
    let fmt: u_long;
    let mut data: c_int = 0;
    let mut size: c_int;
    let mut nbufs: c_int;
    let mut info: audio_buf_info = core::mem::zeroed();

    if cmd == SNDCTL_DSP_RESET {
        sq_reset();
        return 0;
    } else if cmd == SNDCTL_DSP_GETFMTS {
        fmt = dmasound.mach.hardware_afmts; /* this is what OSS says.. */
        return IOCTL_OUT(arg, fmt);
    } else if cmd == SNDCTL_DSP_GETBLKSIZE {
        size = 0;
        if ((*file).f_mode & FMODE_WRITE) != 0 {
            if (*write_sq()).locked == 0 {
                sq_setup(write_sq());
            }
            size = (*write_sq()).user_frag_size;
        }
        return IOCTL_OUT(arg, size as u_long);
    } else if cmd == SNDCTL_DSP_POST {
        (*write_sq()).syncing |= 0x2;
        sq_play();
        return 0;
    } else if cmd == SNDCTL_DSP_SYNC {
        result = 0;
        if ((*file).f_mode & FMODE_WRITE) != 0 {
            result = sq_fsync();
            sq_reset_output();
        }
        if ((*file).f_mode & shared_resource_owner) != 0 {
            shared_resources_initialised = 0;
        }
        return result;
    } else if cmd == SOUND_PCM_READ_RATE {
        return IOCTL_OUT(arg, dmasound.soft.speed as u_long);
    } else if cmd == SNDCTL_DSP_SPEED {
        if shared_resources_are_mine((*file).f_mode) != 0 {
            IOCTL_IN(arg, &mut data);
            data = sound_set_speed(data);
            shared_resources_initialised = 0;
            return IOCTL_OUT(arg, data as u_long);
        } else {
            return -EINVAL;
        }
    } else if cmd == SNDCTL_DSP_STEREO {
        if shared_resources_are_mine((*file).f_mode) != 0 && queues_are_quiescent() != 0 {
            IOCTL_IN(arg, &mut data);
            shared_resources_initialised = 0;
            return IOCTL_OUT(arg, sound_set_stereo(data) as u_long);
        } else {
            return -EINVAL;
        }
    } else if cmd == SOUND_PCM_WRITE_CHANNELS {
        if shared_resources_are_mine((*file).f_mode) != 0 && queues_are_quiescent() != 0 {
            IOCTL_IN(arg, &mut data);
            shared_resources_initialised = 0;
            return IOCTL_OUT(arg, (sound_set_stereo(data - 1) + 1) as u_long);
        } else {
            return -EINVAL;
        }
    } else if cmd == SNDCTL_DSP_SETFMT {
        if shared_resources_are_mine((*file).f_mode) != 0 && queues_are_quiescent() != 0 {
            let format: c_int;
            IOCTL_IN(arg, &mut data);
            shared_resources_initialised = 0;
            format = sound_set_format(data);
            result = IOCTL_OUT(arg, format as u_long);
            if result < 0 {
                return result;
            }
            if format != data && data != AFMT_QUERY {
                return -EINVAL;
            }
            return 0;
        } else {
            return -EINVAL;
        }
    } else if cmd == SNDCTL_DSP_SUBDIVIDE {
        return -EINVAL;
    } else if cmd == SNDCTL_DSP_SETFRAGMENT {
        IOCTL_IN(arg, &mut data);
        result = 0;
        nbufs = (data >> 16) & 0x7fff; /* 0x7fff is 'use maximum' */
        size = data & 0xffff;
        if ((*file).f_mode & FMODE_WRITE) != 0 {
            result = set_queue_frags(write_sq(), nbufs, size);
            if result != 0 {
                return result;
            }
        }
        return IOCTL_OUT(arg, data as u_long);
    } else if cmd == SNDCTL_DSP_GETOSPACE {
        if ((*file).f_mode & FMODE_WRITE) != 0 {
            if (*write_sq()).locked == 0 {
                sq_setup(write_sq());
            }
            info.fragments = (*write_sq()).max_active - (*write_sq()).count;
            info.fragstotal = (*write_sq()).max_active;
            info.fragsize = (*write_sq()).user_frag_size;
            info.bytes = info.fragments * info.fragsize;
            if copy_to_user(arg as *mut c_void, &info as *const _ as *const c_void, size_of::<audio_buf_info>()) != 0 {
                return -EFAULT;
            }
            return 0;
        } else {
            return -EINVAL;
        }
    } else if cmd == SNDCTL_DSP_GETCAPS {
        val = dmasound.mach.capabilities & 0xffffff00u32 as c_int;
        return IOCTL_OUT(arg, val as u_long);
    } else {
        return mixer_ioctl(file, cmd, arg);
    }
}

unsafe extern "C" fn sq_unlocked_ioctl(file: *mut file, cmd: u_int, arg: u_long) -> c_long {
    let ret: c_int;

    mutex_lock(&raw mut dmasound_core_mutex);
    ret = sq_ioctl(file, cmd, arg);
    mutex_unlock(&raw mut dmasound_core_mutex);

    ret as c_long
}

static sq_fops: file_operations = file_operations {
    owner: ptr::null_mut(),
    read: None,
    write: Some(sq_write),
    poll: Some(sq_poll),
    unlocked_ioctl: Some(sq_unlocked_ioctl),
    compat_ioctl: Some(compat_ptr_ioctl),
    open: Some(sq_open),
    release: Some(sq_release),
};

unsafe fn sq_init() -> c_int {
    let fops: *const file_operations = &sq_fops;

    sq_unit = register_sound_dsp(fops, -1);
    if sq_unit < 0 {
        printk(c"dmasound_core: couldn't register fops\n".as_ptr());
        return sq_unit;
    }

    write_sq_init_waitqueue();

    if shared_resource_owner == 0 {
        dmasound.soft = dmasound.mach.default_soft;
        dmasound.hard = dmasound.mach.default_hard;
        dmasound.dsp = dmasound.mach.default_soft;
        shared_resources_initialised = 0;
    }
    0
}

#[repr(C)]
struct StateState {
    busy: c_int,
    buf: [c_char; STAT_BUFF_LEN],
    len: c_int,
    ptr: c_int,
}

static mut state: StateState = StateState {
    busy: 0,
    buf: [0; STAT_BUFF_LEN],
    len: 0,
    ptr: 0,
};

unsafe fn get_afmt_string(afmt: c_int) -> *const c_char {
    match afmt {
        x if x == AFMT_MU_LAW => c"mu-law".as_ptr(),
        x if x == AFMT_A_LAW => c"A-law".as_ptr(),
        x if x == AFMT_U8 => c"unsigned 8 bit".as_ptr(),
        x if x == AFMT_S8 => c"signed 8 bit".as_ptr(),
        x if x == AFMT_S16_BE => c"signed 16 bit BE".as_ptr(),
        x if x == AFMT_U16_BE => c"unsigned 16 bit BE".as_ptr(),
        x if x == AFMT_S16_LE => c"signed 16 bit LE".as_ptr(),
        x if x == AFMT_U16_LE => c"unsigned 16 bit LE".as_ptr(),
        0 => c"format not set".as_ptr(),
        _ => c"ERROR: Unsupported AFMT_XXXX code".as_ptr(),
    }
}

unsafe extern "C" fn state_open(_inode: *mut inode, _file: *mut file) -> c_int {
    let buffer: *mut c_char = state.buf.as_mut_ptr();
    let mut len: c_int = 0;
    let mut ret: c_int;

    mutex_lock(&raw mut dmasound_core_mutex);
    ret = -EBUSY;
    if state.busy != 0 {
        mutex_unlock(&raw mut dmasound_core_mutex);
        return ret;
    }

    ret = -ENODEV;
    if try_module_get(dmasound.mach.owner) == 0 {
        mutex_unlock(&raw mut dmasound_core_mutex);
        return ret;
    }

    state.ptr = 0;
    state.busy = 1;

    len += sprintf(buffer.offset(len as isize), c"%sDMA sound driver rev %03d :\n".as_ptr(),
        dmasound.mach.name, (DMASOUND_CORE_REVISION << 4) + ((dmasound.mach.version >> 8) & 0x0f));
    len += sprintf(buffer.offset(len as isize),
        c"Core driver edition %02d.%02d : %s driver edition %02d.%02d\n".as_ptr(),
        DMASOUND_CORE_REVISION, DMASOUND_CORE_EDITION, dmasound.mach.name2,
        (dmasound.mach.version >> 8), (dmasound.mach.version & 0xff));

    if let Some(func) = dmasound.mach.state_info {
        len += func(buffer.offset(len as isize), LOW_LEVEL_STAT_ALLOC);
    }

    len += sprintf(buffer.offset(len as isize), c"\t\t === Formats & settings ===\n".as_ptr());
    len += sprintf(buffer.offset(len as isize), c"Parameter %20s%20s\n".as_ptr(), c"soft".as_ptr(), c"hard".as_ptr());
    len += sprintf(buffer.offset(len as isize), c"Format   :%20s%20s\n".as_ptr(),
        get_afmt_string(dmasound.soft.format),
        get_afmt_string(dmasound.hard.format));

    len += sprintf(buffer.offset(len as isize), c"Samp Rate:%14d s/sec%14d s/sec\n".as_ptr(),
        dmasound.soft.speed, dmasound.hard.speed);

    len += sprintf(buffer.offset(len as isize), c"Channels :%20s%20s\n".as_ptr(),
        if dmasound.soft.stereo != 0 { c"stereo".as_ptr() } else { c"mono".as_ptr() },
        if dmasound.hard.stereo != 0 { c"stereo".as_ptr() } else { c"mono".as_ptr() });

    len += sprintf(buffer.offset(len as isize), c"\t\t === Sound Queue status ===\n".as_ptr());
    len += sprintf(buffer.offset(len as isize), c"Allocated:%8s%6s\n".as_ptr(), c"Buffers".as_ptr(), c"Size".as_ptr());
    len += sprintf(buffer.offset(len as isize), c"%9s:%8d%6d\n".as_ptr(),
        c"write".as_ptr(), (*write_sq()).numBufs, (*write_sq()).bufSize);
    len += sprintf(buffer.offset(len as isize),
        c"Current  : MaxFrg FragSiz MaxAct Frnt Rear Cnt RrSize A B S L  xruns\n".as_ptr());
    len += sprintf(buffer.offset(len as isize), c"%9s:%7d%8d%7d%5d%5d%4d%7d%2d%2d%2d%2d%7d\n".as_ptr(),
        c"write".as_ptr(), (*write_sq()).max_count, (*write_sq()).block_size,
        (*write_sq()).max_active, (*write_sq()).front, (*write_sq()).rear,
        (*write_sq()).count, (*write_sq()).rear_size, (*write_sq()).active,
        (*write_sq()).busy, (*write_sq()).syncing, (*write_sq()).locked, (*write_sq()).xruns);
    /* DEBUG_DMASOUND: printk("dmasound: stat buffer used %d bytes\n", len); */

    if len >= STAT_BUFF_LEN as c_int {
        printk(c"dmasound_core: stat buffer overflowed!\n".as_ptr());
    }

    state.len = len;
    ret = 0;
    mutex_unlock(&raw mut dmasound_core_mutex);
    ret
}

unsafe extern "C" fn state_release(_inode: *mut inode, _file: *mut file) -> c_int {
    mutex_lock(&raw mut dmasound_core_mutex);
    state.busy = 0;
    module_put(dmasound.mach.owner);
    mutex_unlock(&raw mut dmasound_core_mutex);
    0
}

unsafe extern "C" fn state_read(_file: *mut file, buf: *mut c_char, count: size_t, _ppos: *mut loff_t) -> ssize_t {
    let mut n: c_int = state.len - state.ptr;
    if (n as size_t) > count {
        n = count as c_int;
    }
    if n <= 0 {
        return 0;
    }
    if copy_to_user(buf as *mut c_void, state.buf.as_ptr().offset(state.ptr as isize) as *const c_void, n as usize) != 0 {
        return -EFAULT as ssize_t;
    }
    state.ptr += n;
    n as ssize_t
}

static state_fops: file_operations = file_operations {
    owner: ptr::null_mut(),
    read: Some(state_read),
    write: None,
    poll: None,
    unlocked_ioctl: None,
    compat_ioctl: None,
    open: Some(state_open),
    release: Some(state_release),
};

unsafe fn state_init() -> c_int {
    state_unit = register_sound_special(&state_fops, SND_DEV_STATUS);
    if state_unit < 0 {
        return state_unit;
    }
    state.busy = 0;
    0
}

#[no_mangle]
pub unsafe extern "C" fn dmasound_init() -> c_int {
    let mut res: c_int;

    if irq_installed != 0 {
        return -EBUSY;
    }

    res = sq_init();
    if res < 0 {
        return res;
    }

    res = state_init();
    if res < 0 {
        return res;
    }

    mixer_init();

    if (dmasound.mach.irqinit)() == 0 {
        printk(c"DMA sound driver: Interrupt initialization failed\n".as_ptr());
        return -ENODEV;
    }
    irq_installed = 1;

    printk(c"%s DMA sound driver rev %03d installed\n".as_ptr(),
        dmasound.mach.name, (DMASOUND_CORE_REVISION << 4) + ((dmasound.mach.version >> 8) & 0x0f));
    printk(c"Core driver edition %02d.%02d : %s driver edition %02d.%02d\n".as_ptr(),
        DMASOUND_CORE_REVISION, DMASOUND_CORE_EDITION, dmasound.mach.name2,
        (dmasound.mach.version >> 8), (dmasound.mach.version & 0xff));
    printk(c"Write will use %4d fragments of %7d bytes as default\n".as_ptr(),
        numWriteBufs, writeBufSize);
    0
}

#[no_mangle]
pub unsafe extern "C" fn dmasound_deinit() {
    if irq_installed != 0 {
        sound_silence();
        (dmasound.mach.irqcleanup)();
        irq_installed = 0;
    }

    write_sq_release_buffers();

    if mixer_unit >= 0 {
        unregister_sound_mixer(mixer_unit);
    }
    if state_unit >= 0 {
        unregister_sound_special(state_unit);
    }
    if sq_unit >= 0 {
        unregister_sound_dsp(sq_unit);
    }
}

unsafe fn dmasound_setup(mut str_: *mut c_char) -> c_int {
    let mut ints: [c_int; 6] = [0; 6];
    let mut size: c_int = 0;

    str_ = get_options(str_, 6, ints.as_mut_ptr());
    let _ = str_;

    /* check the bootstrap parameter for "dmasound=" */

    match ints[0] {
        3 => {
            if ints[3] < 0 || ints[3] > MAX_CATCH_RADIUS {
                printk(c"dmasound_setup: invalid catch radius, using default = %d\n".as_ptr(), dmasound_catchRadius);
            } else {
                dmasound_catchRadius = ints[3];
            }
            if ints[1] < MIN_BUFFERS as c_int {
                printk(c"dmasound_setup: invalid number of buffers, using default = %d\n".as_ptr(), numWriteBufs);
            } else {
                numWriteBufs = ints[1] as c_uint;
            }
            size = ints[2];
            if size < 256 { /* check for small buffer specs */
                size <<= 10;
            }
            if size < MIN_BUFSIZE || size > MAX_BUFSIZE {
                printk(c"dmasound_setup: invalid write buffer size, using default = %d\n".as_ptr(), writeBufSize);
            } else {
                writeBufSize = size as c_uint;
            }
        }
        2 => {
            if ints[1] < MIN_BUFFERS as c_int {
                printk(c"dmasound_setup: invalid number of buffers, using default = %d\n".as_ptr(), numWriteBufs);
            } else {
                numWriteBufs = ints[1] as c_uint;
            }
            size = ints[2];
            if size < 256 {
                size <<= 10;
            }
            if size < MIN_BUFSIZE || size > MAX_BUFSIZE {
                printk(c"dmasound_setup: invalid write buffer size, using default = %d\n".as_ptr(), writeBufSize);
            } else {
                writeBufSize = size as c_uint;
            }
        }
        1 => {
            size = ints[2];
            if size < 256 { /* check for small buffer specs */
                size <<= 10;
            }
            if size < MIN_BUFSIZE || size > MAX_BUFSIZE {
                printk(c"dmasound_setup: invalid write buffer size, using default = %d\n".as_ptr(), writeBufSize);
            } else {
                writeBufSize = size as c_uint;
            }
        }
        0 => {}
        _ => {
            printk(c"dmasound_setup: invalid number of arguments\n".as_ptr());
            return 0;
        }
    }
    1
}

/* __setup("dmasound=", dmasound_setup); */

/*
 * Conversion tables
 *
 * Original C condition: #ifdef HAS_8BIT_TABLES
 */

#[no_mangle]
pub static mut dmasound_ulaw2dma8: [c_char; 256] = [
    -126, -122, -118, -114, -110, -106, -102, -98,
    -94, -90, -86, -82, -78, -74, -70, -66,
    -63, -61, -59, -57, -55, -53, -51, -49,
    -47, -45, -43, -41, -39, -37, -35, -33,
    -31, -30, -29, -28, -27, -26, -25, -24,
    -23, -22, -21, -20, -19, -18, -17, -16,
    -16, -15, -15, -14, -14, -13, -13, -12,
    -12, -11, -11, -10, -10, -9, -9, -8,
    -8, -8, -7, -7, -7, -7, -6, -6,
    -6, -6, -5, -5, -5, -5, -4, -4,
    -4, -4, -4, -4, -3, -3, -3, -3,
    -3, -3, -3, -3, -2, -2, -2, -2,
    -2, -2, -2, -2, -2, -2, -2, -2,
    -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, 0,
    125, 121, 117, 113, 109, 105, 101, 97,
    93, 89, 85, 81, 77, 73, 69, 65,
    62, 60, 58, 56, 54, 52, 50, 48,
    46, 44, 42, 40, 38, 36, 34, 32,
    30, 29, 28, 27, 26, 25, 24, 23,
    22, 21, 20, 19, 18, 17, 16, 15,
    15, 14, 14, 13, 13, 12, 12, 11,
    11, 10, 10, 9, 9, 8, 8, 7,
    7, 7, 6, 6, 6, 6, 5, 5,
    5, 5, 4, 4, 4, 4, 3, 3,
    3, 3, 3, 3, 2, 2, 2, 2,
    2, 2, 2, 2, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
];

#[no_mangle]
pub static mut dmasound_alaw2dma8: [c_char; 256] = [
    -22, -21, -24, -23, -18, -17, -20, -19,
    -30, -29, -32, -31, -26, -25, -28, -27,
    -11, -11, -12, -12, -9, -9, -10, -10,
    -15, -15, -16, -16, -13, -13, -14, -14,
    -86, -82, -94, -90, -70, -66, -78, -74,
    -118, -114, -126, -122, -102, -98, -110, -106,
    -43, -41, -47, -45, -35, -33, -39, -37,
    -59, -57, -63, -61, -51, -49, -55, -53,
    -2, -2, -2, -2, -2, -2, -2, -2,
    -2, -2, -2, -2, -2, -2, -2, -2,
    -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1,
    -6, -6, -6, -6, -5, -5, -5, -5,
    -8, -8, -8, -8, -7, -7, -7, -7,
    -3, -3, -3, -3, -3, -3, -3, -3,
    -4, -4, -4, -4, -4, -4, -4, -4,
    21, 20, 23, 22, 17, 16, 19, 18,
    29, 28, 31, 30, 25, 24, 27, 26,
    10, 10, 11, 11, 8, 8, 9, 9,
    14, 14, 15, 15, 12, 12, 13, 13,
    86, 82, 94, 90, 70, 66, 78, 74,
    118, 114, 126, 122, 102, 98, 110, 106,
    43, 41, 47, 45, 35, 33, 39, 37,
    59, 57, 63, 61, 51, 49, 55, 53,
    1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    5, 5, 5, 5, 4, 4, 4, 4,
    7, 7, 7, 7, 6, 6, 6, 6,
    2, 2, 2, 2, 2, 2, 2, 2,
    3, 3, 3, 3, 3, 3, 3, 3,
];

/* Visible symbols for modules:
 * EXPORT_SYMBOL(dmasound);
 * EXPORT_SYMBOL(dmasound_init);
 * EXPORT_SYMBOL(dmasound_deinit);
 * EXPORT_SYMBOL(dmasound_write_sq);
 * EXPORT_SYMBOL(dmasound_catchRadius);
 * EXPORT_SYMBOL(dmasound_ulaw2dma8);
 * EXPORT_SYMBOL(dmasound_alaw2dma8);
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
