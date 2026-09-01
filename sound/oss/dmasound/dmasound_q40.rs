// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/sound/oss/dmasound/dmasound_q40.c
 *
 *  Q40 DMA Sound Driver
 *
 *  See linux/sound/oss/dmasound/dmasound_core.c for copyright and credits
 *  prior to 28/01/2001
 *
 *  28/01/2001 [0.1] Iain Sandoe
 *             - added versioning
 *             - put in and populated the hardware_afmts field.
 *             [0.2] - put in SNDCTL_DSP_GETCAPS value.
 *         [0.3] - put in default hard/soft settings.
 */

/* Dependencies originally supplied by:
 * linux/module.h, linux/init.h, linux/slab.h, linux/soundcard.h,
 * linux/interrupt.h, linux/uaccess.h, asm/q40ints.h, asm/q40_master.h,
 * and "dmasound.h".
 */

type c_int = i32;
type c_uint = u32;
type c_ulong = u64;
type c_char = i8;
type c_void = core::ffi::c_void;
type size_t = usize;
type ssize_t = isize;
type u_char = u8;
type gfp_t = c_uint;
type irqreturn_t = c_int;

const DMASOUND_Q40_REVISION: c_int = 0;
const DMASOUND_Q40_EDITION: c_int = 3;

extern "C" {
    static mut dmasound: Dmasound;
    static mut dmasound_ulaw2dma8: *mut c_char;
    static mut dmasound_alaw2dma8: *mut c_char;
    static mut write_sq: WriteSq;
    static mut catchRadius: c_int;
    static mut DAC_LEFT: *mut u_char;
    static mut DAC_RIGHT: *mut u_char;

    static mut Q40_IRQ_SAMPLE: c_uint;
    static mut SAMPLE_ENABLE_REG: c_uint;
    static mut SAMPLE_RATE_REG: c_uint;
    static mut SAMPLE_CLEAR_REG: c_uint;

    static mut AFMT_QUERY: c_int;
    static mut AFMT_MU_LAW: c_int;
    static mut AFMT_A_LAW: c_int;
    static mut AFMT_S8: c_int;
    static mut AFMT_U8: c_int;
    static mut SND_DEV_DSP: c_int;
    static mut DSP_CAP_BATCH: c_int;
    static mut IRQ_HANDLED: irqreturn_t;
    static mut THIS_MODULE: *mut c_void;
    static mut EFAULT: c_int;
    static mut ENODEV: c_int;

    fn kmalloc(size: c_uint, flags: gfp_t) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn request_irq(
        irq: c_uint,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        flags: c_ulong,
        name: *const c_char,
        dev: *mut c_void,
    ) -> c_int;
    fn free_irq(irq: c_uint, dev: *mut c_void);
    fn master_outb(value: u_char, reg: c_uint);
    fn copy_from_user(to: *mut c_void, from: *const u_char, n: c_ulong) -> c_ulong;
    fn get_user_u8(to: *mut u_char, from: *const u_char) -> c_int;
    fn printk_ratelimit() -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn spin_lock(lock: *mut c_void);
    fn spin_unlock(lock: *mut c_void);
    fn spin_lock_irqsave(lock: *mut c_void, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut c_void, flags: c_ulong);
    fn WAKE_UP(queue: *mut c_void);
    fn dmasound_init() -> c_int;
    fn dmasound_deinit();
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Settings {
    pub format: c_int,
    pub stereo: c_int,
    pub size: c_int,
    pub speed: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Trans {
    pub ct_ulaw: Option<
        unsafe extern "C" fn(*const u_char, size_t, *mut u_char, *mut ssize_t, ssize_t) -> ssize_t,
    >,
    pub ct_alaw: Option<
        unsafe extern "C" fn(*const u_char, size_t, *mut u_char, *mut ssize_t, ssize_t) -> ssize_t,
    >,
    pub ct_s8: Option<
        unsafe extern "C" fn(*const u_char, size_t, *mut u_char, *mut ssize_t, ssize_t) -> ssize_t,
    >,
    pub ct_u8: Option<
        unsafe extern "C" fn(*const u_char, size_t, *mut u_char, *mut ssize_t, ssize_t) -> ssize_t,
    >,
    pub ct_s16be: Option<unsafe extern "C" fn()>,
    pub ct_u16be: Option<unsafe extern "C" fn()>,
    pub ct_s16le: Option<unsafe extern "C" fn()>,
    pub ct_u16le: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Machine {
    pub name: *const c_char,
    pub name2: *const c_char,
    pub owner: *mut c_void,
    pub dma_alloc: Option<unsafe extern "C" fn(c_uint, gfp_t) -> *mut c_void>,
    pub dma_free: Option<unsafe extern "C" fn(*mut c_void, c_uint)>,
    pub irqinit: Option<unsafe extern "C" fn() -> c_int>,
    /* MODULE builds also contain irqcleanup = Q40IrqCleanUp. */
    pub init: Option<unsafe extern "C" fn()>,
    pub silence: Option<unsafe extern "C" fn()>,
    pub setFormat: Option<unsafe extern "C" fn(c_int) -> c_int>,
    pub setVolume: Option<unsafe extern "C" fn(c_int) -> c_int>,
    pub play: Option<unsafe extern "C" fn()>,
    pub min_dsp_speed: c_int,
    pub version: c_int,
    pub hardware_afmts: c_int,
    pub capabilities: c_int,
    pub default_hard: Settings,
    pub default_soft: Settings,
}

#[repr(C)]
pub struct Dmasound {
    pub soft: Settings,
    pub hard: Settings,
    pub dsp: Settings,
    pub minDev: c_int,
    pub trans_write: *mut Trans,
    pub lock: *mut c_void,
    pub mach: Machine,
}

#[repr(C)]
pub struct WriteSq {
    pub buffers: *mut *mut u_char,
    pub front: c_int,
    pub count: c_int,
    pub rear_size: c_ulong,
    pub block_size: c_ulong,
    pub max_count: c_int,
    pub active: c_int,
    pub syncing: c_int,
    pub sync_queue: *mut c_void,
    pub action_queue: *mut c_void,
}

static mut expand_bal: c_int = 0; /* Balance factor for expanding (not volume!) */
static mut expand_data: c_int = 0; /* Data for expanding */

/*** Mid level stuff *********************************************************/

/* userCount, frameUsed, frameLeft == byte counts */
unsafe extern "C" fn q40_ct_law(
    mut userPtr: *const u_char,
    userCount: size_t,
    frame: *mut u_char,
    frameUsed: *mut ssize_t,
    frameLeft: ssize_t,
) -> ssize_t {
    let table: *mut c_char = if dmasound.soft.format == AFMT_MU_LAW {
        dmasound_ulaw2dma8
    } else {
        dmasound_alaw2dma8
    };
    let mut count: ssize_t;
    let used: ssize_t;
    let mut p: *mut u_char = frame.offset(*frameUsed);

    count = core::cmp::min(userCount, frameLeft as size_t) as ssize_t;
    used = count;
    if copy_from_user(p as *mut c_void, userPtr, count as c_ulong) != 0 {
        return -(EFAULT as ssize_t);
    }
    while count > 0 {
        *p = ((*table.offset(*p as isize) as c_int) + 128) as u_char;
        p = p.offset(1);
        count -= 1;
    }
    *frameUsed += used;
    used
}

unsafe extern "C" fn q40_ct_s8(
    userPtr: *const u_char,
    userCount: size_t,
    frame: *mut u_char,
    frameUsed: *mut ssize_t,
    frameLeft: ssize_t,
) -> ssize_t {
    let mut count: ssize_t;
    let used: ssize_t;
    let mut p: *mut u_char = frame.offset(*frameUsed);

    count = core::cmp::min(userCount, frameLeft as size_t) as ssize_t;
    used = count;
    if copy_from_user(p as *mut c_void, userPtr, count as c_ulong) != 0 {
        return -(EFAULT as ssize_t);
    }
    while count > 0 {
        *p = (*p).wrapping_add(128);
        p = p.offset(1);
        count -= 1;
    }
    *frameUsed += used;
    used
}

unsafe extern "C" fn q40_ct_u8(
    userPtr: *const u_char,
    userCount: size_t,
    frame: *mut u_char,
    frameUsed: *mut ssize_t,
    frameLeft: ssize_t,
) -> ssize_t {
    let mut count: ssize_t;
    let used: ssize_t;
    let p: *mut u_char = frame.offset(*frameUsed);

    count = core::cmp::min(userCount, frameLeft as size_t) as ssize_t;
    used = count;
    if copy_from_user(p as *mut c_void, userPtr, count as c_ulong) != 0 {
        return -(EFAULT as ssize_t);
    }
    *frameUsed += used;
    used
}

/* a bit too complicated to optimise right now ..*/
unsafe extern "C" fn q40_ctx_law(
    mut userPtr: *const u_char,
    mut userCount: size_t,
    frame: *mut u_char,
    frameUsed: *mut ssize_t,
    mut frameLeft: ssize_t,
) -> ssize_t {
    let table: *mut u_char = if dmasound.soft.format == AFMT_MU_LAW {
        dmasound_ulaw2dma8 as *mut u_char
    } else {
        dmasound_alaw2dma8 as *mut u_char
    };
    let mut data: c_uint = expand_data as c_uint;
    let mut p: *mut u_char = frame.offset(*frameUsed);
    let mut bal: c_int = expand_bal;
    let hSpeed: c_int = dmasound.hard.speed;
    let sSpeed: c_int = dmasound.soft.speed;
    let mut utotal: c_int;
    let ftotal: c_int;

    ftotal = frameLeft as c_int;
    utotal = userCount as c_int;
    while frameLeft != 0 {
        let mut c: u_char = 0;
        if bal < 0 {
            if userCount == 0 {
                break;
            }
            if get_user_u8(&mut c, userPtr) != 0 {
                return -(EFAULT as ssize_t);
            }
            userPtr = userPtr.offset(1);
            data = *table.offset(c as isize) as c_uint;
            data = data.wrapping_add(0x80);
            userCount -= 1;
            bal += hSpeed;
        }
        *p = data as u_char;
        p = p.offset(1);
        frameLeft -= 1;
        bal -= sSpeed;
    }
    expand_bal = bal;
    expand_data = data as c_int;
    *frameUsed += (ftotal as ssize_t) - frameLeft;
    utotal -= userCount as c_int;
    utotal as ssize_t
}

unsafe extern "C" fn q40_ctx_s8(
    mut userPtr: *const u_char,
    mut userCount: size_t,
    frame: *mut u_char,
    frameUsed: *mut ssize_t,
    mut frameLeft: ssize_t,
) -> ssize_t {
    let mut p: *mut u_char = frame.offset(*frameUsed);
    let mut data: c_uint = expand_data as c_uint;
    let mut bal: c_int = expand_bal;
    let hSpeed: c_int = dmasound.hard.speed;
    let sSpeed: c_int = dmasound.soft.speed;
    let mut utotal: c_int;
    let ftotal: c_int;

    ftotal = frameLeft as c_int;
    utotal = userCount as c_int;
    while frameLeft != 0 {
        let mut c: u_char = 0;
        if bal < 0 {
            if userCount == 0 {
                break;
            }
            if get_user_u8(&mut c, userPtr) != 0 {
                return -(EFAULT as ssize_t);
            }
            userPtr = userPtr.offset(1);
            data = c as c_uint;
            data = data.wrapping_add(0x80);
            userCount -= 1;
            bal += hSpeed;
        }
        *p = data as u_char;
        p = p.offset(1);
        frameLeft -= 1;
        bal -= sSpeed;
    }
    expand_bal = bal;
    expand_data = data as c_int;
    *frameUsed += (ftotal as ssize_t) - frameLeft;
    utotal -= userCount as c_int;
    utotal as ssize_t
}

unsafe extern "C" fn q40_ctx_u8(
    mut userPtr: *const u_char,
    mut userCount: size_t,
    frame: *mut u_char,
    frameUsed: *mut ssize_t,
    mut frameLeft: ssize_t,
) -> ssize_t {
    let mut p: *mut u_char = frame.offset(*frameUsed);
    let mut data: c_uint = expand_data as c_uint;
    let mut bal: c_int = expand_bal;
    let hSpeed: c_int = dmasound.hard.speed;
    let sSpeed: c_int = dmasound.soft.speed;
    let mut utotal: c_int;
    let ftotal: c_int;

    ftotal = frameLeft as c_int;
    utotal = userCount as c_int;
    while frameLeft != 0 {
        let mut c: u_char = 0;
        if bal < 0 {
            if userCount == 0 {
                break;
            }
            if get_user_u8(&mut c, userPtr) != 0 {
                return -(EFAULT as ssize_t);
            }
            userPtr = userPtr.offset(1);
            data = c as c_uint;
            userCount -= 1;
            bal += hSpeed;
        }
        *p = data as u_char;
        p = p.offset(1);
        frameLeft -= 1;
        bal -= sSpeed;
    }
    expand_bal = bal;
    expand_data = data as c_int;
    *frameUsed += (ftotal as ssize_t) - frameLeft;
    utotal -= userCount as c_int;
    utotal as ssize_t
}

/* compressing versions */
unsafe extern "C" fn q40_ctc_law(
    mut userPtr: *const u_char,
    mut userCount: size_t,
    frame: *mut u_char,
    frameUsed: *mut ssize_t,
    mut frameLeft: ssize_t,
) -> ssize_t {
    let table: *mut u_char = if dmasound.soft.format == AFMT_MU_LAW {
        dmasound_ulaw2dma8 as *mut u_char
    } else {
        dmasound_alaw2dma8 as *mut u_char
    };
    let mut data: c_uint = expand_data as c_uint;
    let mut p: *mut u_char = frame.offset(*frameUsed);
    let mut bal: c_int = expand_bal;
    let hSpeed: c_int = dmasound.hard.speed;
    let sSpeed: c_int = dmasound.soft.speed;
    let mut utotal: c_int;
    let ftotal: c_int;

    ftotal = frameLeft as c_int;
    utotal = userCount as c_int;
    'lout: while frameLeft != 0 {
        let mut c: u_char = 0;
        while bal < 0 {
            if userCount == 0 {
                break 'lout;
            }
            if !(bal < -hSpeed) {
                if get_user_u8(&mut c, userPtr) != 0 {
                    return -(EFAULT as ssize_t);
                }
                data = 0x80u32.wrapping_add(*table.offset(c as isize) as c_uint);
            }
            userPtr = userPtr.offset(1);
            userCount -= 1;
            bal += hSpeed;
        }
        *p = data as u_char;
        p = p.offset(1);
        frameLeft -= 1;
        bal -= sSpeed;
    }
    expand_bal = bal;
    expand_data = data as c_int;
    *frameUsed += (ftotal as ssize_t) - frameLeft;
    utotal -= userCount as c_int;
    utotal as ssize_t
}

unsafe extern "C" fn q40_ctc_s8(
    mut userPtr: *const u_char,
    mut userCount: size_t,
    frame: *mut u_char,
    frameUsed: *mut ssize_t,
    mut frameLeft: ssize_t,
) -> ssize_t {
    let mut p: *mut u_char = frame.offset(*frameUsed);
    let mut data: c_uint = expand_data as c_uint;
    let mut bal: c_int = expand_bal;
    let hSpeed: c_int = dmasound.hard.speed;
    let sSpeed: c_int = dmasound.soft.speed;
    let mut utotal: c_int;
    let ftotal: c_int;

    ftotal = frameLeft as c_int;
    utotal = userCount as c_int;
    'lout: while frameLeft != 0 {
        let mut c: u_char = 0;
        while bal < 0 {
            if userCount == 0 {
                break 'lout;
            }
            if !(bal < -hSpeed) {
                if get_user_u8(&mut c, userPtr) != 0 {
                    return -(EFAULT as ssize_t);
                }
                data = (c as c_uint).wrapping_add(0x80);
            }
            userPtr = userPtr.offset(1);
            userCount -= 1;
            bal += hSpeed;
        }
        *p = data as u_char;
        p = p.offset(1);
        frameLeft -= 1;
        bal -= sSpeed;
    }
    expand_bal = bal;
    expand_data = data as c_int;
    *frameUsed += (ftotal as ssize_t) - frameLeft;
    utotal -= userCount as c_int;
    utotal as ssize_t
}

unsafe extern "C" fn q40_ctc_u8(
    mut userPtr: *const u_char,
    mut userCount: size_t,
    frame: *mut u_char,
    frameUsed: *mut ssize_t,
    mut frameLeft: ssize_t,
) -> ssize_t {
    let mut p: *mut u_char = frame.offset(*frameUsed);
    let mut data: c_uint = expand_data as c_uint;
    let mut bal: c_int = expand_bal;
    let hSpeed: c_int = dmasound.hard.speed;
    let sSpeed: c_int = dmasound.soft.speed;
    let mut utotal: c_int;
    let ftotal: c_int;

    ftotal = frameLeft as c_int;
    utotal = userCount as c_int;
    'lout: while frameLeft != 0 {
        let mut c: u_char = 0;
        while bal < 0 {
            if userCount == 0 {
                break 'lout;
            }
            if !(bal < -hSpeed) {
                if get_user_u8(&mut c, userPtr) != 0 {
                    return -(EFAULT as ssize_t);
                }
                data = c as c_uint;
            }
            userPtr = userPtr.offset(1);
            userCount -= 1;
            bal += hSpeed;
        }
        *p = data as u_char;
        p = p.offset(1);
        frameLeft -= 1;
        bal -= sSpeed;
    }
    expand_bal = bal;
    expand_data = data as c_int;
    *frameUsed += (ftotal as ssize_t) - frameLeft;
    utotal -= userCount as c_int;
    utotal as ssize_t
}

static mut transQ40Normal: Trans = Trans {
    ct_ulaw: Some(q40_ct_law),
    ct_alaw: Some(q40_ct_law),
    ct_s8: Some(q40_ct_s8),
    ct_u8: Some(q40_ct_u8),
    ct_s16be: None,
    ct_u16be: None,
    ct_s16le: None,
    ct_u16le: None,
};

static mut transQ40Expanding: Trans = Trans {
    ct_ulaw: Some(q40_ctx_law),
    ct_alaw: Some(q40_ctx_law),
    ct_s8: Some(q40_ctx_s8),
    ct_u8: Some(q40_ctx_u8),
    ct_s16be: None,
    ct_u16be: None,
    ct_s16le: None,
    ct_u16le: None,
};

static mut transQ40Compressing: Trans = Trans {
    ct_ulaw: Some(q40_ctc_law),
    ct_alaw: Some(q40_ctc_law),
    ct_s8: Some(q40_ctc_s8),
    ct_u8: Some(q40_ctc_u8),
    ct_s16be: None,
    ct_u16be: None,
    ct_s16le: None,
    ct_u16le: None,
};

/*** Low level stuff *********************************************************/

unsafe extern "C" fn Q40Alloc(size: c_uint, flags: gfp_t) -> *mut c_void {
    kmalloc(size, flags) /* change to vmalloc */
}

unsafe extern "C" fn Q40Free(ptr: *mut c_void, _size: c_uint) {
    kfree(ptr);
}

unsafe extern "C" fn Q40IrqInit() -> c_int {
    /* Register interrupt handler. */
    if request_irq(
        Q40_IRQ_SAMPLE,
        Q40StereoInterrupt,
        0,
        b"DMA sound\0".as_ptr() as *const c_char,
        Q40Interrupt as *mut c_void,
    ) != 0
    {
        return 0;
    }

    1
}

/* MODULE-only in C. */
unsafe extern "C" fn Q40IrqCleanUp() {
    master_outb(0, SAMPLE_ENABLE_REG);
    free_irq(Q40_IRQ_SAMPLE, Q40Interrupt as *mut c_void);
}

unsafe extern "C" fn Q40Silence() {
    master_outb(0, SAMPLE_ENABLE_REG);
    *DAC_RIGHT = 127;
    *DAC_LEFT = *DAC_RIGHT;
}

static mut q40_pp: *mut c_char = core::ptr::null_mut();
static mut q40_sc: c_uint = 0;

unsafe extern "C" fn Q40PlayNextFrame(index: c_int) {
    let start: *mut u_char;
    let size: c_ulong;
    let speed: u_char;
    let error: c_int;

    /* used by Q40Play() if all doubts whether there really is something
     * to be played are already wiped out.
     */
    start = *write_sq.buffers.offset(write_sq.front as isize);
    size = if write_sq.count == index {
        write_sq.rear_size
    } else {
        write_sq.block_size
    };

    q40_pp = start as *mut c_char;
    q40_sc = size as c_uint;

    write_sq.front = (write_sq.front + 1) % write_sq.max_count;
    write_sq.active += 1;

    speed = if dmasound.hard.speed == 10000 { 0 } else { 1 };

    master_outb(0, SAMPLE_ENABLE_REG);
    free_irq(Q40_IRQ_SAMPLE, Q40Interrupt as *mut c_void);
    if dmasound.soft.stereo != 0 {
        error = request_irq(
            Q40_IRQ_SAMPLE,
            Q40StereoInterrupt,
            0,
            b"Q40 sound\0".as_ptr() as *const c_char,
            Q40Interrupt as *mut c_void,
        );
    } else {
        error = request_irq(
            Q40_IRQ_SAMPLE,
            Q40MonoInterrupt,
            0,
            b"Q40 sound\0".as_ptr() as *const c_char,
            Q40Interrupt as *mut c_void,
        );
    }
    if error != 0 && printk_ratelimit() != 0 {
        pr_err(b"Couldn't register sound interrupt\n\0".as_ptr() as *const c_char);
    }

    master_outb(speed, SAMPLE_RATE_REG);
    master_outb(1, SAMPLE_CLEAR_REG);
    master_outb(1, SAMPLE_ENABLE_REG);
}

unsafe extern "C" fn Q40Play() {
    let mut flags: c_ulong = 0;

    if write_sq.active != 0 || write_sq.count <= 0 {
        /* There's already a frame loaded */
        return;
    }

    /* nothing in the queue */
    if write_sq.count <= 1
        && write_sq.rear_size < write_sq.block_size
        && write_sq.syncing == 0
    {
        /* hmmm, the only existing frame is not
         * yet filled and we're not syncing?
         */
        return;
    }
    spin_lock_irqsave(dmasound.lock, &mut flags);
    Q40PlayNextFrame(1);
    spin_unlock_irqrestore(dmasound.lock, flags);
}

unsafe extern "C" fn Q40StereoInterrupt(_irq: c_int, _dummy: *mut c_void) -> irqreturn_t {
    spin_lock(dmasound.lock);
    if q40_sc > 1 {
        *DAC_LEFT = *(q40_pp as *mut u_char);
        q40_pp = q40_pp.offset(1);
        *DAC_RIGHT = *(q40_pp as *mut u_char);
        q40_pp = q40_pp.offset(1);
        q40_sc -= 2;
        master_outb(1, SAMPLE_CLEAR_REG);
    } else {
        Q40Interrupt();
    }
    spin_unlock(dmasound.lock);
    IRQ_HANDLED
}

unsafe extern "C" fn Q40MonoInterrupt(_irq: c_int, _dummy: *mut c_void) -> irqreturn_t {
    spin_lock(dmasound.lock);
    if q40_sc > 0 {
        *DAC_LEFT = *(q40_pp as *mut u_char);
        *DAC_RIGHT = *(q40_pp as *mut u_char);
        q40_pp = q40_pp.offset(1);
        q40_sc -= 1;
        master_outb(1, SAMPLE_CLEAR_REG);
    } else {
        Q40Interrupt();
    }
    spin_unlock(dmasound.lock);
    IRQ_HANDLED
}

unsafe extern "C" fn Q40Interrupt() {
    if write_sq.active == 0 {
        /* playing was interrupted and sq_reset() has already cleared
         * the sq variables, so better don't do anything here.
         */
        WAKE_UP(write_sq.sync_queue);
        master_outb(0, SAMPLE_ENABLE_REG); /* better safe */
    } else {
        write_sq.active = 0;
        write_sq.count -= 1;
        Q40Play();

        if q40_sc < 2 {
            /* there was nothing to play, disable irq */
            master_outb(0, SAMPLE_ENABLE_REG);
            *DAC_RIGHT = 127;
            *DAC_LEFT = *DAC_RIGHT;
        }
        WAKE_UP(write_sq.action_queue);
    }

    master_outb(1, SAMPLE_CLEAR_REG);
}

unsafe extern "C" fn Q40Init() {
    let mut i: c_int;
    let mut idx: c_int;
    let freq: [c_int; 2] = [10000, 20000];

    /* search a frequency that fits into the allowed error range */

    idx = -1;
    i = 0;
    while i < 2 {
        if (100 * (dmasound.soft.speed - freq[i as usize]).abs() / freq[i as usize]) <= catchRadius {
            idx = i;
        }
        i += 1;
    }

    dmasound.hard = dmasound.soft;
    /*sound.hard.stereo=1;*/ /* no longer true */
    dmasound.hard.size = 8;

    if idx > -1 {
        dmasound.soft.speed = freq[idx as usize];
        dmasound.trans_write = &mut transQ40Normal;
    } else {
        dmasound.trans_write = &mut transQ40Expanding;
    }

    Q40Silence();

    if dmasound.hard.speed > 20200 {
        /* squeeze the sound, we do that */
        dmasound.hard.speed = 20000;
        dmasound.trans_write = &mut transQ40Compressing;
    } else if dmasound.hard.speed > 10000 {
        dmasound.hard.speed = 20000;
    } else {
        dmasound.hard.speed = 10000;
    }
    expand_bal = -dmasound.soft.speed;
}

unsafe extern "C" fn Q40SetFormat(mut format: c_int) -> c_int {
    /* Q40 sound supports only 8bit modes */

    if format == AFMT_QUERY {
        return dmasound.soft.format;
    } else if format == AFMT_MU_LAW
        || format == AFMT_A_LAW
        || format == AFMT_S8
        || format == AFMT_U8
    {
    } else {
        format = AFMT_S8;
    }

    dmasound.soft.format = format;
    dmasound.soft.size = 8;
    if dmasound.minDev == SND_DEV_DSP {
        dmasound.dsp.format = format;
        dmasound.dsp.size = 8;
    }
    Q40Init();

    format
}

unsafe extern "C" fn Q40SetVolume(_volume: c_int) -> c_int {
    0
}

/*** Machine definitions *****************************************************/

static mut def_hard: Settings = Settings {
    format: 0, /* AFMT_U8, filled during init-equivalent use below where extern constants are readable. */
    stereo: 0,
    size: 8,
    speed: 10000,
};

static mut def_soft: Settings = Settings {
    format: 0, /* AFMT_U8, filled during init-equivalent use below where extern constants are readable. */
    stereo: 0,
    size: 8,
    speed: 8000,
};

static mut machQ40: Machine = Machine {
    name: b"Q40\0".as_ptr() as *const c_char,
    name2: b"Q40\0".as_ptr() as *const c_char,
    owner: core::ptr::null_mut(), /* THIS_MODULE */
    dma_alloc: Some(Q40Alloc),
    dma_free: Some(Q40Free),
    irqinit: Some(Q40IrqInit),
    /* MODULE: irqcleanup = Q40IrqCleanUp */
    init: Some(Q40Init),
    silence: Some(Q40Silence),
    setFormat: Some(Q40SetFormat),
    setVolume: Some(Q40SetVolume),
    play: Some(Q40Play),
    min_dsp_speed: 10000,
    version: ((DMASOUND_Q40_REVISION << 8) | DMASOUND_Q40_EDITION),
    hardware_afmts: 0, /* AFMT_U8, h'ware-supported formats *only* here */
    capabilities: 0, /* DSP_CAP_BATCH, As per SNDCTL_DSP_GETCAPS */
    default_hard: Settings {
        format: 0,
        stereo: 0,
        size: 8,
        speed: 10000,
    },
    default_soft: Settings {
        format: 0,
        stereo: 0,
        size: 8,
        speed: 8000,
    },
};

/*** Config & Setup **********************************************************/

unsafe extern "C" fn dmasound_q40_init() -> c_int {
    /* MACH_IS_Q40 is a platform macro in C. */
    if MACH_IS_Q40() {
        def_hard.format = AFMT_U8;
        def_soft.format = AFMT_U8;
        machQ40.owner = THIS_MODULE;
        machQ40.hardware_afmts = AFMT_U8; /* h'ware-supported formats *only* here */
        machQ40.capabilities = DSP_CAP_BATCH; /* As per SNDCTL_DSP_GETCAPS */
        machQ40.default_hard = def_hard;
        machQ40.default_soft = def_soft;
        dmasound.mach = machQ40;
        dmasound.mach.default_hard = def_hard;
        dmasound.mach.default_soft = def_soft;
        return dmasound_init();
    } else {
        return -ENODEV;
    }
}

unsafe extern "C" fn dmasound_q40_cleanup() {
    dmasound_deinit();
}

extern "C" {
    fn MACH_IS_Q40() -> bool;
}

/* module_init(dmasound_q40_init); */
/* module_exit(dmasound_q40_cleanup); */

/* MODULE_DESCRIPTION("Q40/Q60 sound driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
