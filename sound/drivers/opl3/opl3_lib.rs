// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>,
 *                   Hannu Savolainen 1993-1996,
 *                   Rob Hooft
 *
 *  Routines for control of AdLib FM cards (OPL2/OPL3/OPL4 chips)
 *
 *  Most if code is ported from OSS/Lite.
 */

// Dependencies from <sound/opl3.h>, <linux/io.h>, <linux/delay.h>,
// <linux/module.h>, <linux/init.h>, <linux/slab.h>, <linux/ioport.h>,
// <sound/minors.h>, and "opl3_voice.h" are expected from the surrounding tree.

use core::ffi::{c_char, c_int, c_uchar, c_uint, c_ulong, c_ushort, c_void};

extern "C" {
    fn outb(value: c_uchar, port: c_ulong);
    fn inb(port: c_ulong) -> c_uchar;
    fn udelay(usecs: c_ulong);

    fn snd_timer_chip(timer: *mut snd_timer) -> *mut snd_opl3;
    fn snd_timer_new(
        card: *mut snd_card,
        id: *const c_char,
        tid: *mut snd_timer_id,
        rtimer: *mut *mut snd_timer,
    ) -> c_int;
    fn snd_timer_interrupt(timer: *mut snd_timer, ticks: c_uint);
    fn snd_device_new(
        card: *mut snd_card,
        dev_type: c_int,
        device_data: *mut c_void,
        ops: *const snd_device_ops,
    ) -> c_int;
    fn snd_device_free(card: *mut snd_card, device_data: *mut c_void) -> c_int;
    fn snd_hwdep_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        rhwdep: *mut *mut snd_hwdep,
    ) -> c_int;
    fn snd_seq_device_new(
        card: *mut snd_card,
        device: c_int,
        id: *const c_char,
        argsize: usize,
        result: *mut *mut snd_seq_device,
    ) -> c_int;

    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn request_region(start: c_ulong, n: c_ulong, name: *const c_char) -> *mut resource;
    fn release_and_free_resource(res: *mut resource);
    fn kfree(ptr: *mut c_void);

    fn snd_opl3_clear_patches(opl3: *mut snd_opl3);
    fn snd_opl3_open(hw: *mut snd_hwdep, file: *mut c_void) -> c_int;
    fn snd_opl3_ioctl(hw: *mut snd_hwdep, file: *mut c_void, cmd: c_uint, arg: c_ulong) -> c_int;
    fn snd_opl3_write(hw: *mut snd_hwdep, buf: *const c_char, count: isize, offset: *mut i64) -> isize;
    fn snd_opl3_release(hw: *mut snd_hwdep, file: *mut c_void) -> c_int;

    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_BUG_ON(condition: bool) -> bool;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t) -> c_ulong;
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn SNDRV_SEQ_DEVICE_ARGPTR(dev: *mut snd_seq_device) -> *mut c_void;
}

#[repr(C)]
pub struct snd_opl3 {
    pub card: *mut snd_card,
    pub hardware: c_ushort,
    pub reg_lock: spinlock_t,
    pub timer_lock: spinlock_t,
    pub command: Option<unsafe extern "C" fn(*mut snd_opl3, c_ushort, c_uchar)>,
    pub l_port: c_ulong,
    pub r_port: c_ulong,
    pub timer_enable: c_uchar,
    pub timer1: *mut snd_timer,
    pub timer2: *mut snd_timer,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_opl3)>,
    pub res_l_port: *mut resource,
    pub res_r_port: *mut resource,
    pub max_voices: c_uint,
    pub hwdep: *mut snd_hwdep,
    pub seq_dev_num: c_int,
    pub seq_dev: *mut snd_seq_device,
}

#[repr(C)]
pub struct snd_card {
    pub number: c_int,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_timer {
    pub name: [c_char; 64],
    pub private_data: *mut c_void,
    pub hw: snd_timer_hardware,
    pub sticks: c_uint,
}

#[repr(C)]
pub struct snd_timer_id {
    pub dev_class: c_int,
    pub dev_sclass: c_int,
    pub card: c_int,
    pub device: c_int,
    pub subdevice: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_timer_hardware {
    pub flags: c_uint,
    pub resolution: c_ulong,
    pub ticks: c_ulong,
    pub start: Option<unsafe extern "C" fn(*mut snd_timer) -> c_int>,
    pub stop: Option<unsafe extern "C" fn(*mut snd_timer) -> c_int>,
}

#[repr(C)]
pub struct snd_device {
    pub device_data: *mut c_void,
}

#[repr(C)]
pub struct snd_device_ops {
    pub dev_free: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
}

#[repr(C)]
pub struct snd_hwdep {
    pub private_data: *mut c_void,
    pub exclusive: c_int,
    pub oss_type: c_int,
    pub name: [c_char; 64],
    pub id: [c_char; 64],
    pub iface: c_int,
    pub ops: snd_hwdep_ops,
}

#[repr(C)]
pub struct snd_hwdep_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut c_void) -> c_int>,
    pub ioctl: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut c_void, c_uint, c_ulong) -> c_int>,
    pub write: Option<unsafe extern "C" fn(*mut snd_hwdep, *const c_char, isize, *mut i64) -> isize>,
    pub release: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut c_void) -> c_int>,
}

#[repr(C)]
pub struct snd_seq_device {
    pub name: [c_char; 64],
}

#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

const ENODEV: c_int = 19;
const ENXIO: c_int = 6;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const GFP_KERNEL: c_uint = 0;

const OPL3_RIGHT: c_ushort = 0x100;
const OPL3_LEFT: c_ushort = 0x000;
const OPL3_REG_TIMER_CONTROL: c_ushort = 0x04;
const OPL3_REG_TIMER1: c_ushort = 0x02;
const OPL3_REG_TIMER2: c_ushort = 0x03;
const OPL3_REG_TEST: c_ushort = 0x01;
const OPL3_REG_PERCUSSION: c_ushort = 0xbd;
const OPL3_REG_MODE: c_ushort = 0x05;
const OPL3_TIMER1_MASK: c_uchar = 0x40;
const OPL3_TIMER2_MASK: c_uchar = 0x20;
const OPL3_IRQ_RESET: c_uchar = 0x80;
const OPL3_TIMER1_START: c_uchar = 0x01;
const OPL3_TIMER2_START: c_uchar = 0x02;
const OPL3_ENABLE_WAVE_SELECT: c_uchar = 0x20;
const OPL3_OPL3_ENABLE: c_uchar = 0x01;
const OPL3_HW_AUTO: c_ushort = 0;
const OPL3_HW_OPL2: c_ushort = 1;
const OPL3_HW_OPL3: c_ushort = 2;
const OPL3_HW_OPL4: c_ushort = 3;
const OPL3_HW_OPL3_SV: c_ushort = 4;
const OPL3_HW_OPL3_CS: c_ushort = 5;
const OPL3_HW_OPL3_FM801: c_ushort = 6;
const OPL3_HW_MASK: c_ushort = 0x00ff;
const MAX_OPL2_VOICES: c_uint = 9;
const MAX_OPL3_VOICES: c_uint = 18;
const SNDRV_TIMER_HW_STOP: c_uint = 1;
const SNDRV_TIMER_CLASS_CARD: c_int = 0;
const SNDRV_TIMER_SCLASS_NONE: c_int = 0;
const SNDRV_DEV_CODEC: c_int = 0;
const SNDRV_HWDEP_IFACE_OPL2: c_int = 0;
const SNDRV_HWDEP_IFACE_OPL3: c_int = 1;
const SNDRV_HWDEP_IFACE_OPL4: c_int = 2;
const SNDRV_OSS_DEVICE_TYPE_DMFM: c_int = 0;

const fn c_str(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

unsafe fn kzalloc_obj_snd_opl3() -> *mut snd_opl3 {
    kzalloc(core::mem::size_of::<snd_opl3>(), GFP_KERNEL) as *mut snd_opl3
}

unsafe extern "C" fn snd_opl2_command(opl3: *mut snd_opl3, cmd: c_ushort, val: c_uchar) {
    let port: c_ulong;

    /*
     * The original 2-OP synth requires a quite long delay
     * after writing to a register.
     */

    port = if (cmd & OPL3_RIGHT) != 0 { (*opl3).r_port } else { (*opl3).l_port };

    let flags = spin_lock_irqsave(&mut (*opl3).reg_lock);

    outb(cmd as c_uchar, port);
    udelay(10);

    outb(val as c_uchar, port + 1);
    udelay(30);

    spin_unlock_irqrestore(&mut (*opl3).reg_lock, flags);
}

unsafe extern "C" fn snd_opl3_command(opl3: *mut snd_opl3, cmd: c_ushort, val: c_uchar) {
    let port: c_ulong;

    /*
     * The OPL-3 survives with just two INBs
     * after writing to a register.
     */

    port = if (cmd & OPL3_RIGHT) != 0 { (*opl3).r_port } else { (*opl3).l_port };

    let flags = spin_lock_irqsave(&mut (*opl3).reg_lock);

    outb(cmd as c_uchar, port);
    inb((*opl3).l_port);
    inb((*opl3).l_port);

    outb(val as c_uchar, port + 1);
    inb((*opl3).l_port);
    inb((*opl3).l_port);

    spin_unlock_irqrestore(&mut (*opl3).reg_lock, flags);
}

unsafe extern "C" fn snd_opl3_detect(opl3: *mut snd_opl3) -> c_int {
    /*
     * This function returns 1 if the FM chip is present at the given I/O port
     * The detection algorithm plays with the timer built in the FM chip and
     * looks for a change in the status register.
     *
     * Note! The timers of the FM chip are not connected to AdLib (and compatible)
     * boards.
     *
     * Note2! The chip is initialized if detected.
     */

    let stat1: c_uchar;
    let stat2: c_uchar;
    let signature: c_uchar;

    /* Reset timers 1 and 2 */
    ((*opl3).command.unwrap())(opl3, OPL3_LEFT | OPL3_REG_TIMER_CONTROL, OPL3_TIMER1_MASK | OPL3_TIMER2_MASK);
    /* Reset the IRQ of the FM chip */
    ((*opl3).command.unwrap())(opl3, OPL3_LEFT | OPL3_REG_TIMER_CONTROL, OPL3_IRQ_RESET);
    stat1 = inb((*opl3).l_port);
    signature = stat1;
    if (stat1 & 0xe0) != 0x00 {
        dev_dbg((*(*opl3).card).dev, c_str(b"OPL3: stat1 = 0x%x\n\0"), stat1 as c_int);
        return -ENODEV;
    }
    /* Set timer1 to 0xff */
    ((*opl3).command.unwrap())(opl3, OPL3_LEFT | OPL3_REG_TIMER1, 0xff);
    /* Unmask and start timer 1 */
    ((*opl3).command.unwrap())(opl3, OPL3_LEFT | OPL3_REG_TIMER_CONTROL, OPL3_TIMER2_MASK | OPL3_TIMER1_START);
    /* Now we have to delay at least 80us */
    udelay(200);
    /* Read status after timers have expired */
    stat2 = inb((*opl3).l_port);
    /* Stop the timers */
    ((*opl3).command.unwrap())(opl3, OPL3_LEFT | OPL3_REG_TIMER_CONTROL, OPL3_TIMER1_MASK | OPL3_TIMER2_MASK);
    /* Reset the IRQ of the FM chip */
    ((*opl3).command.unwrap())(opl3, OPL3_LEFT | OPL3_REG_TIMER_CONTROL, OPL3_IRQ_RESET);
    if (stat2 & 0xe0) != 0xc0 {
        dev_dbg((*(*opl3).card).dev, c_str(b"OPL3: stat2 = 0x%x\n\0"), stat2 as c_int);
        return -ENODEV;
    }

    /* If the toplevel code knows exactly the type of chip, don't try
       to detect it. */
    if (*opl3).hardware != OPL3_HW_AUTO {
        return 0;
    }

    /* There is a FM chip on this address. Detect the type (OPL2 to OPL4) */
    if signature == 0x06 {
        (*opl3).hardware = OPL3_HW_OPL2;
    } else {
        /*
         * If we had an OPL4 chip, opl3->hardware would have been set
         * by the OPL4 driver; so we can assume OPL3 here.
         */
        if snd_BUG_ON((*opl3).r_port == 0) {
            return -ENODEV;
        }
        (*opl3).hardware = OPL3_HW_OPL3;
    }
    0
}

/*
 *  AdLib timers
 */

/*
 *  Timer 1 - 80us
 */

unsafe extern "C" fn snd_opl3_timer1_start(timer: *mut snd_timer) -> c_int {
    let tmp: c_uchar;
    let ticks: c_uint;
    let opl3: *mut snd_opl3;

    opl3 = snd_timer_chip(timer);
    let flags = spin_lock_irqsave(&mut (*opl3).timer_lock);
    ticks = (*timer).sticks;
    tmp = ((*opl3).timer_enable | OPL3_TIMER1_START) & !OPL3_TIMER1_MASK;
    (*opl3).timer_enable = tmp;
    ((*opl3).command.unwrap())(opl3, OPL3_LEFT | OPL3_REG_TIMER1, (256u32.wrapping_sub(ticks)) as c_uchar);
    ((*opl3).command.unwrap())(opl3, OPL3_LEFT | OPL3_REG_TIMER_CONTROL, tmp);
    spin_unlock_irqrestore(&mut (*opl3).timer_lock, flags);
    0
}

unsafe extern "C" fn snd_opl3_timer1_stop(timer: *mut snd_timer) -> c_int {
    let tmp: c_uchar;
    let opl3: *mut snd_opl3;

    opl3 = snd_timer_chip(timer);
    let flags = spin_lock_irqsave(&mut (*opl3).timer_lock);
    tmp = ((*opl3).timer_enable | OPL3_TIMER1_MASK) & !OPL3_TIMER1_START;
    (*opl3).timer_enable = tmp;
    ((*opl3).command.unwrap())(opl3, OPL3_LEFT | OPL3_REG_TIMER_CONTROL, tmp);
    spin_unlock_irqrestore(&mut (*opl3).timer_lock, flags);
    0
}

/*
 *  Timer 2 - 320us
 */

unsafe extern "C" fn snd_opl3_timer2_start(timer: *mut snd_timer) -> c_int {
    let tmp: c_uchar;
    let ticks: c_uint;
    let opl3: *mut snd_opl3;

    opl3 = snd_timer_chip(timer);
    let flags = spin_lock_irqsave(&mut (*opl3).timer_lock);
    ticks = (*timer).sticks;
    tmp = ((*opl3).timer_enable | OPL3_TIMER2_START) & !OPL3_TIMER2_MASK;
    (*opl3).timer_enable = tmp;
    ((*opl3).command.unwrap())(opl3, OPL3_LEFT | OPL3_REG_TIMER2, (256u32.wrapping_sub(ticks)) as c_uchar);
    ((*opl3).command.unwrap())(opl3, OPL3_LEFT | OPL3_REG_TIMER_CONTROL, tmp);
    spin_unlock_irqrestore(&mut (*opl3).timer_lock, flags);
    0
}

unsafe extern "C" fn snd_opl3_timer2_stop(timer: *mut snd_timer) -> c_int {
    let tmp: c_uchar;
    let opl3: *mut snd_opl3;

    opl3 = snd_timer_chip(timer);
    let flags = spin_lock_irqsave(&mut (*opl3).timer_lock);
    tmp = ((*opl3).timer_enable | OPL3_TIMER2_MASK) & !OPL3_TIMER2_START;
    (*opl3).timer_enable = tmp;
    ((*opl3).command.unwrap())(opl3, OPL3_LEFT | OPL3_REG_TIMER_CONTROL, tmp);
    spin_unlock_irqrestore(&mut (*opl3).timer_lock, flags);
    0
}

/*

 */

static mut snd_opl3_timer1: snd_timer_hardware = snd_timer_hardware {
    flags: SNDRV_TIMER_HW_STOP,
    resolution: 80000,
    ticks: 256,
    start: Some(snd_opl3_timer1_start),
    stop: Some(snd_opl3_timer1_stop),
};

static mut snd_opl3_timer2: snd_timer_hardware = snd_timer_hardware {
    flags: SNDRV_TIMER_HW_STOP,
    resolution: 320000,
    ticks: 256,
    start: Some(snd_opl3_timer2_start),
    stop: Some(snd_opl3_timer2_stop),
};

unsafe extern "C" fn snd_opl3_timer1_init(opl3: *mut snd_opl3, timer_no: c_int) -> c_int {
    let mut timer: *mut snd_timer = core::ptr::null_mut();
    let mut tid: snd_timer_id = core::mem::zeroed();
    let err: c_int;

    tid.dev_class = SNDRV_TIMER_CLASS_CARD;
    tid.dev_sclass = SNDRV_TIMER_SCLASS_NONE;
    tid.card = (*(*opl3).card).number;
    tid.device = timer_no;
    tid.subdevice = 0;
    err = snd_timer_new((*opl3).card, c_str(b"AdLib timer #1\0"), &mut tid, &mut timer);
    if err >= 0 {
        strscpy((*timer).name.as_mut_ptr(), c_str(b"AdLib timer #1\0"));
        (*timer).private_data = opl3 as *mut c_void;
        (*timer).hw = snd_opl3_timer1;
    }
    (*opl3).timer1 = timer;
    err
}

unsafe extern "C" fn snd_opl3_timer2_init(opl3: *mut snd_opl3, timer_no: c_int) -> c_int {
    let mut timer: *mut snd_timer = core::ptr::null_mut();
    let mut tid: snd_timer_id = core::mem::zeroed();
    let err: c_int;

    tid.dev_class = SNDRV_TIMER_CLASS_CARD;
    tid.dev_sclass = SNDRV_TIMER_SCLASS_NONE;
    tid.card = (*(*opl3).card).number;
    tid.device = timer_no;
    tid.subdevice = 0;
    err = snd_timer_new((*opl3).card, c_str(b"AdLib timer #2\0"), &mut tid, &mut timer);
    if err >= 0 {
        strscpy((*timer).name.as_mut_ptr(), c_str(b"AdLib timer #2\0"));
        (*timer).private_data = opl3 as *mut c_void;
        (*timer).hw = snd_opl3_timer2;
    }
    (*opl3).timer2 = timer;
    err
}

/*

 */

#[no_mangle]
pub unsafe extern "C" fn snd_opl3_interrupt(hw: *mut snd_hwdep) {
    let status: c_uchar;
    let opl3: *mut snd_opl3;
    let timer: *mut snd_timer;

    if hw.is_null() {
        return;
    }

    opl3 = (*hw).private_data as *mut snd_opl3;
    status = inb((*opl3).l_port);
    if (status & 0x80) == 0 {
        return;
    }

    if (status & 0x40) != 0 {
        timer = (*opl3).timer1;
        snd_timer_interrupt(timer, (*timer).sticks);
    }
    if (status & 0x20) != 0 {
        timer = (*opl3).timer2;
        snd_timer_interrupt(timer, (*timer).sticks);
    }
}

/* EXPORT_SYMBOL(snd_opl3_interrupt); */

/*

 */

unsafe extern "C" fn snd_opl3_free(opl3: *mut snd_opl3) -> c_int {
    if snd_BUG_ON(opl3.is_null()) {
        return -ENXIO;
    }
    if let Some(private_free) = (*opl3).private_free {
        private_free(opl3);
    }
    snd_opl3_clear_patches(opl3);
    release_and_free_resource((*opl3).res_l_port);
    release_and_free_resource((*opl3).res_r_port);
    kfree(opl3 as *mut c_void);
    0
}

unsafe extern "C" fn snd_opl3_dev_free(device: *mut snd_device) -> c_int {
    let opl3: *mut snd_opl3 = (*device).device_data as *mut snd_opl3;
    snd_opl3_free(opl3)
}

#[no_mangle]
pub unsafe extern "C" fn snd_opl3_new(
    card: *mut snd_card,
    hardware: c_ushort,
    ropl3: *mut *mut snd_opl3,
) -> c_int {
    static ops: snd_device_ops = snd_device_ops {
        dev_free: Some(snd_opl3_dev_free),
    };
    let opl3: *mut snd_opl3;
    let err: c_int;

    *ropl3 = core::ptr::null_mut();
    opl3 = kzalloc_obj_snd_opl3();
    if opl3.is_null() {
        return -ENOMEM;
    }

    (*opl3).card = card;
    (*opl3).hardware = hardware;
    spin_lock_init(&mut (*opl3).reg_lock);
    spin_lock_init(&mut (*opl3).timer_lock);

    err = snd_device_new(card, SNDRV_DEV_CODEC, opl3 as *mut c_void, &ops);
    if err < 0 {
        snd_opl3_free(opl3);
        return err;
    }

    *ropl3 = opl3;
    0
}

/* EXPORT_SYMBOL(snd_opl3_new); */

#[no_mangle]
pub unsafe extern "C" fn snd_opl3_init(opl3: *mut snd_opl3) -> c_int {
    if (*opl3).command.is_none() {
        dev_err((*(*opl3).card).dev, c_str(b"snd_opl3_init: command not defined!\n\0"));
        return -EINVAL;
    }

    ((*opl3).command.unwrap())(opl3, OPL3_LEFT | OPL3_REG_TEST, OPL3_ENABLE_WAVE_SELECT);
    /* Melodic mode */
    ((*opl3).command.unwrap())(opl3, OPL3_LEFT | OPL3_REG_PERCUSSION, 0x00);

    match (*opl3).hardware & OPL3_HW_MASK {
        OPL3_HW_OPL2 => {
            (*opl3).max_voices = MAX_OPL2_VOICES;
        }
        OPL3_HW_OPL3 | OPL3_HW_OPL4 => {
            (*opl3).max_voices = MAX_OPL3_VOICES;
            /* Enter OPL3 mode */
            ((*opl3).command.unwrap())(opl3, OPL3_RIGHT | OPL3_REG_MODE, OPL3_OPL3_ENABLE);
        }
        _ => {}
    }
    0
}

/* EXPORT_SYMBOL(snd_opl3_init); */

#[no_mangle]
pub unsafe extern "C" fn snd_opl3_create(
    card: *mut snd_card,
    l_port: c_ulong,
    r_port: c_ulong,
    hardware: c_ushort,
    integrated: c_int,
    ropl3: *mut *mut snd_opl3,
) -> c_int {
    let mut opl3: *mut snd_opl3 = core::ptr::null_mut();
    let mut err: c_int;

    *ropl3 = core::ptr::null_mut();
    err = snd_opl3_new(card, hardware, &mut opl3);
    if err < 0 {
        return err;
    }
    if integrated == 0 {
        (*opl3).res_l_port = request_region(l_port, 2, c_str(b"OPL2/3 (left)\0"));
        if (*opl3).res_l_port.is_null() {
            dev_err((*card).dev, c_str(b"opl3: can't grab left port 0x%lx\n\0"), l_port);
            snd_device_free(card, opl3 as *mut c_void);
            return -EBUSY;
        }
        if r_port != 0 {
            (*opl3).res_r_port = request_region(r_port, 2, c_str(b"OPL2/3 (right)\0"));
            if (*opl3).res_r_port.is_null() {
                dev_err((*card).dev, c_str(b"opl3: can't grab right port 0x%lx\n\0"), r_port);
                snd_device_free(card, opl3 as *mut c_void);
                return -EBUSY;
            }
        }
    }
    (*opl3).l_port = l_port;
    (*opl3).r_port = r_port;

    match (*opl3).hardware {
        /* some hardware doesn't support timers */
        OPL3_HW_OPL3_SV | OPL3_HW_OPL3_CS | OPL3_HW_OPL3_FM801 => {
            (*opl3).command = Some(snd_opl3_command);
        }
        _ => {
            (*opl3).command = Some(snd_opl2_command);
            err = snd_opl3_detect(opl3);
            if err < 0 {
                dev_dbg(
                    (*card).dev,
                    c_str(b"OPL2/3 chip not detected at 0x%lx/0x%lx\n\0"),
                    (*opl3).l_port,
                    (*opl3).r_port,
                );
                snd_device_free(card, opl3 as *mut c_void);
                return err;
            }
            /* detect routine returns correct hardware type */
            match (*opl3).hardware & OPL3_HW_MASK {
                OPL3_HW_OPL3 | OPL3_HW_OPL4 => {
                    (*opl3).command = Some(snd_opl3_command);
                }
                _ => {}
            }
        }
    }

    snd_opl3_init(opl3);

    *ropl3 = opl3;
    0
}

/* EXPORT_SYMBOL(snd_opl3_create); */

#[no_mangle]
pub unsafe extern "C" fn snd_opl3_timer_new(
    opl3: *mut snd_opl3,
    timer1_dev: c_int,
    timer2_dev: c_int,
) -> c_int {
    let mut err: c_int;

    if timer1_dev >= 0 {
        err = snd_opl3_timer1_init(opl3, timer1_dev);
        if err < 0 {
            return err;
        }
    }
    if timer2_dev >= 0 {
        err = snd_opl3_timer2_init(opl3, timer2_dev);
        if err < 0 {
            snd_device_free((*opl3).card, (*opl3).timer1 as *mut c_void);
            (*opl3).timer1 = core::ptr::null_mut();
            return err;
        }
    }
    0
}

/* EXPORT_SYMBOL(snd_opl3_timer_new); */

#[no_mangle]
pub unsafe extern "C" fn snd_opl3_hwdep_new(
    opl3: *mut snd_opl3,
    device: c_int,
    seq_device: c_int,
    rhwdep: *mut *mut snd_hwdep,
) -> c_int {
    let mut hw: *mut snd_hwdep = core::ptr::null_mut();
    let card: *mut snd_card = (*opl3).card;
    let err: c_int;

    if !rhwdep.is_null() {
        *rhwdep = core::ptr::null_mut();
    }

    /* create hardware dependent device (direct FM) */

    err = snd_hwdep_new(card, c_str(b"OPL2/OPL3\0"), device, &mut hw);
    if err < 0 {
        snd_device_free(card, opl3 as *mut c_void);
        return err;
    }
    (*hw).private_data = opl3 as *mut c_void;
    (*hw).exclusive = 1;
    /* #ifdef CONFIG_SND_OSSEMUL */
    #[cfg(CONFIG_SND_OSSEMUL)]
    {
        if device == 0 {
            (*hw).oss_type = SNDRV_OSS_DEVICE_TYPE_DMFM;
        }
    }
    /* #endif */
    strscpy((*hw).name.as_mut_ptr(), (*hw).id.as_ptr());
    match (*opl3).hardware & OPL3_HW_MASK {
        OPL3_HW_OPL2 => {
            strscpy((*hw).name.as_mut_ptr(), c_str(b"OPL2 FM\0"));
            (*hw).iface = SNDRV_HWDEP_IFACE_OPL2;
        }
        OPL3_HW_OPL3 => {
            strscpy((*hw).name.as_mut_ptr(), c_str(b"OPL3 FM\0"));
            (*hw).iface = SNDRV_HWDEP_IFACE_OPL3;
        }
        OPL3_HW_OPL4 => {
            strscpy((*hw).name.as_mut_ptr(), c_str(b"OPL4 FM\0"));
            (*hw).iface = SNDRV_HWDEP_IFACE_OPL4;
        }
        _ => {}
    }

    /* operators - only ioctl */
    (*hw).ops.open = Some(snd_opl3_open);
    (*hw).ops.ioctl = Some(snd_opl3_ioctl);
    (*hw).ops.write = Some(snd_opl3_write);
    (*hw).ops.release = Some(snd_opl3_release);

    (*opl3).hwdep = hw;
    (*opl3).seq_dev_num = seq_device;
    /* #if IS_ENABLED(CONFIG_SND_SEQUENCER) */
    #[cfg(CONFIG_SND_SEQUENCER)]
    {
        if snd_seq_device_new(
            card,
            seq_device,
            c_str(b"opl3\0"),
            core::mem::size_of::<*mut snd_opl3>(),
            &mut (*opl3).seq_dev,
        ) >= 0
        {
            strscpy((*(*opl3).seq_dev).name.as_mut_ptr(), (*hw).name.as_ptr());
            *(SNDRV_SEQ_DEVICE_ARGPTR((*opl3).seq_dev) as *mut *mut snd_opl3) = opl3;
        }
    }
    /* #endif */
    if !rhwdep.is_null() {
        *rhwdep = hw;
    }
    0
}

/* EXPORT_SYMBOL(snd_opl3_hwdep_new); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
