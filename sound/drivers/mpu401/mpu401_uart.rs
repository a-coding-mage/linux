// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  Routines for control of MPU-401 in UART mode
 *
 *  MPU-401 supports UART mode which is not capable generate transmit
 *  interrupts thus output is done via polling. Without interrupt,
 *  input is done also via polling. Do not expect good performance.
 *
 *   13-03-2003:
 *      Added support for different kind of hardware I/O. Build in choices
 *      are port and mmio. For other kind of I/O, set mpu->read and
 *      mpu->write to your own I/O functions.
 */

use core::ffi::{c_char, c_int, c_uchar, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

// C include dependencies intentionally remain external:
// linux/io.h, linux/delay.h, linux/init.h, linux/slab.h, linux/ioport.h,
// linux/module.h, linux/interrupt.h, linux/errno.h, sound/core.h,
// sound/mpu401.h.

const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const EIO: c_int = 5;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;

const MPU401_RX_EMPTY: c_uchar = 0x80;
const MPU401_TX_FULL: c_uchar = 0x40;
const MPU401_ACK: c_uchar = 0xfe;
const MPU401_RESET: c_uchar = 0xff;
const MPU401_ENTER_UART: c_uchar = 0x3f;

const MPU401_HW_SB: c_ushort = 1;
const MPU401_HW_TRID4DWAVE: c_ushort = 2;
const MPU401_HW_PC98II: c_ushort = 3;

const MPU401_INFO_INPUT: c_uint = 1 << 0;
const MPU401_INFO_OUTPUT: c_uint = 1 << 1;
const MPU401_INFO_TX_IRQ: c_uint = 1 << 2;
const MPU401_INFO_NO_ACK: c_uint = 1 << 3;
const MPU401_INFO_USE_TIMER: c_uint = 1 << 4;
const MPU401_INFO_INTEGRATED: c_uint = 1 << 5;
const MPU401_INFO_MMIO: c_uint = 1 << 6;
const MPU401_INFO_IRQ_HOOK: c_uint = 1 << 7;

const MPU401_MODE_BIT_OUTPUT: c_int = 0;
const MPU401_MODE_BIT_OUTPUT_TRIGGER: c_int = 1;
const MPU401_MODE_BIT_INPUT: c_int = 2;
const MPU401_MODE_BIT_INPUT_TRIGGER: c_int = 3;
const MPU401_MODE_INPUT_TIMER: c_uint = 1 << 4;
const MPU401_MODE_OUTPUT_TIMER: c_uint = 1 << 5;

const SNDRV_RAWMIDI_STREAM_OUTPUT: c_int = 0;
const SNDRV_RAWMIDI_STREAM_INPUT: c_int = 1;
const SNDRV_RAWMIDI_INFO_OUTPUT: c_uint = 1 << 0;
const SNDRV_RAWMIDI_INFO_INPUT: c_uint = 1 << 1;
const SNDRV_RAWMIDI_INFO_DUPLEX: c_uint = 1 << 2;

type c_ushort = u16;
type irqreturn_t = c_int;
type gfp_t = c_uint;

#[repr(C)]
pub struct snd_card {
    pub number: c_int,
    pub shortname: [c_char; 32],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timer_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_rawmidi {
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_rawmidi)>,
    pub dev: *mut device,
    pub name: [c_char; 80],
    pub info_flags: c_uint,
}

#[repr(C)]
pub struct snd_rawmidi_substream {
    pub rmidi: *mut snd_rawmidi,
}

#[repr(C)]
pub struct snd_rawmidi_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream, c_int)>,
}

#[repr(C)]
pub struct snd_mpu401 {
    pub input_lock: spinlock_t,
    pub output_lock: spinlock_t,
    pub timer_lock: spinlock_t,
    pub mode: c_ulong,
    pub timer_invoked: c_uint,
    pub timer: timer_list,
    pub hardware: c_ushort,
    pub irq: c_int,
    pub rmidi: *mut snd_rawmidi,
    pub res: *mut resource,
    pub write: Option<unsafe extern "C" fn(*mut snd_mpu401, c_uchar, c_ulong)>,
    pub read: Option<unsafe extern "C" fn(*mut snd_mpu401, c_ulong) -> c_uchar>,
    pub port: c_ulong,
    pub cport: c_ulong,
    pub info_flags: c_uint,
    pub substream_input: *mut snd_rawmidi_substream,
    pub substream_output: *mut snd_rawmidi_substream,
    pub open_input: Option<unsafe extern "C" fn(*mut snd_mpu401) -> c_int>,
    pub close_input: Option<unsafe extern "C" fn(*mut snd_mpu401)>,
    pub open_output: Option<unsafe extern "C" fn(*mut snd_mpu401) -> c_int>,
    pub close_output: Option<unsafe extern "C" fn(*mut snd_mpu401)>,
}

extern "C" {
    static mut jiffies: c_ulong;

    fn outb(value: c_uchar, port: c_ulong);
    fn inb(port: c_ulong) -> c_uchar;
    fn writeb(value: c_uchar, addr: *mut c_void);
    fn readb(addr: *mut c_void) -> c_uchar;
    fn udelay(usecs: c_ulong);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn test_bit(nr: c_int, addr: *const c_ulong) -> bool;
    fn set_bit(nr: c_int, addr: *mut c_ulong);
    fn clear_bit(nr: c_int, addr: *mut c_ulong);
    fn test_and_set_bit(nr: c_int, addr: *mut c_ulong) -> bool;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn timer_setup(timer: *mut timer_list, callback: unsafe extern "C" fn(*mut timer_list), flags: c_uint);
    fn mod_timer(timer: *mut timer_list, expires: c_ulong) -> c_int;
    fn timer_delete(timer: *mut timer_list) -> c_int;
    fn free_irq(irq: c_uint, dev_id: *mut c_void);
    fn release_and_free_resource(res: *mut resource);
    fn kfree(ptr: *mut c_void);
    fn kzalloc(size: usize, flags: gfp_t) -> *mut c_void;
    fn request_region(start: c_ulong, n: c_ulong, name: *const c_char) -> *mut resource;
    fn request_irq(
        irq: c_uint,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        flags: c_ulong,
        name: *const c_char,
        dev: *mut c_void,
    ) -> c_int;
    fn snd_rawmidi_receive(substream: *mut snd_rawmidi_substream, buffer: *mut c_uchar, count: usize) -> c_int;
    fn snd_rawmidi_transmit_peek(substream: *mut snd_rawmidi_substream, buffer: *mut c_uchar, count: usize) -> c_int;
    fn snd_rawmidi_transmit_ack(substream: *mut snd_rawmidi_substream, count: c_int) -> c_int;
    fn snd_rawmidi_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        output_count: c_int,
        input_count: c_int,
        rrawmidi: *mut *mut snd_rawmidi,
    ) -> c_int;
    fn snd_rawmidi_set_ops(rmidi: *mut snd_rawmidi, stream: c_int, ops: *const snd_rawmidi_ops);
    fn snd_device_free(card: *mut snd_card, device_data: *mut c_void) -> c_int;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
}

#[inline]
unsafe fn MPU401D(mpu: *mut snd_mpu401) -> c_ulong {
    (*mpu).port
}

#[inline]
unsafe fn MPU401C(mpu: *mut snd_mpu401) -> c_ulong {
    (*mpu).cport
}

#[inline]
unsafe fn snd_mpu401_input_avail(mpu: *mut snd_mpu401) -> bool {
    (((*mpu).read.unwrap())(mpu, MPU401C(mpu)) & MPU401_RX_EMPTY) == 0
}

#[inline]
unsafe fn snd_mpu401_output_ready(mpu: *mut snd_mpu401) -> bool {
    (((*mpu).read.unwrap())(mpu, MPU401C(mpu)) & MPU401_TX_FULL) == 0
}

/* Build in lowlevel io */
unsafe extern "C" fn mpu401_write_port(_mpu: *mut snd_mpu401, data: c_uchar, addr: c_ulong) {
    outb(data, addr);
}

unsafe extern "C" fn mpu401_read_port(_mpu: *mut snd_mpu401, addr: c_ulong) -> c_uchar {
    inb(addr)
}

unsafe extern "C" fn mpu401_write_mmio(_mpu: *mut snd_mpu401, data: c_uchar, addr: c_ulong) {
    writeb(data, addr as *mut c_void);
}

unsafe extern "C" fn mpu401_read_mmio(_mpu: *mut snd_mpu401, addr: c_ulong) -> c_uchar {
    readb(addr as *mut c_void)
}
/*  */

unsafe fn snd_mpu401_uart_clear_rx(mpu: *mut snd_mpu401) {
    let mut timeout: c_int = 100000;
    while timeout > 0 && snd_mpu401_input_avail(mpu) {
        timeout -= 1;
        ((*mpu).read.unwrap())(mpu, MPU401D(mpu));
    }
    // CONFIG_SND_DEBUG:
    // if timeout <= 0, report "cmd: clear rx timeout".
}

unsafe fn uart_interrupt_tx(mpu: *mut snd_mpu401) {
    if test_bit(MPU401_MODE_BIT_OUTPUT, &(*mpu).mode)
        && test_bit(MPU401_MODE_BIT_OUTPUT_TRIGGER, &(*mpu).mode)
    {
        let mut flags: c_ulong = 0;
        spin_lock_irqsave(&mut (*mpu).output_lock, &mut flags);
        snd_mpu401_uart_output_write(mpu);
        spin_unlock_irqrestore(&mut (*mpu).output_lock, flags);
    }
}

unsafe fn _snd_mpu401_uart_interrupt(mpu: *mut snd_mpu401) {
    if ((*mpu).info_flags & MPU401_INFO_INPUT) != 0 {
        let mut flags: c_ulong = 0;
        spin_lock_irqsave(&mut (*mpu).input_lock, &mut flags);
        if test_bit(MPU401_MODE_BIT_INPUT, &(*mpu).mode) {
            snd_mpu401_uart_input_read(mpu);
        } else {
            snd_mpu401_uart_clear_rx(mpu);
        }
        spin_unlock_irqrestore(&mut (*mpu).input_lock, flags);
    }
    if ((*mpu).info_flags & MPU401_INFO_TX_IRQ) == 0 {
        /* ok. for better Tx performance try do some output
           when input is done */
        uart_interrupt_tx(mpu);
    }
}

/**
 * snd_mpu401_uart_interrupt - generic MPU401-UART interrupt handler
 * @irq: the irq number
 * @dev_id: mpu401 instance
 *
 * Processes the interrupt for MPU401-UART i/o.
 *
 * Return: %IRQ_HANDLED if the interrupt was handled. %IRQ_NONE otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_mpu401_uart_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let mpu = dev_id as *mut snd_mpu401;

    if mpu.is_null() {
        return IRQ_NONE;
    }
    _snd_mpu401_uart_interrupt(mpu);
    IRQ_HANDLED
}

/**
 * snd_mpu401_uart_interrupt_tx - generic MPU401-UART transmit irq handler
 * @irq: the irq number
 * @dev_id: mpu401 instance
 *
 * Processes the interrupt for MPU401-UART output.
 *
 * Return: %IRQ_HANDLED if the interrupt was handled. %IRQ_NONE otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_mpu401_uart_interrupt_tx(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let mpu = dev_id as *mut snd_mpu401;

    if mpu.is_null() {
        return IRQ_NONE;
    }
    uart_interrupt_tx(mpu);
    IRQ_HANDLED
}

/*
 * timer callback
 * reprogram the timer and call the interrupt job
 */
unsafe extern "C" fn snd_mpu401_uart_timer(t: *mut timer_list) {
    let mpu = timer_container_of_mpu_timer(t);

    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*mpu).timer_lock, &mut flags);
    /*mpu->mode |= MPU401_MODE_TIMER;*/
    mod_timer(&mut (*mpu).timer, 1 + jiffies);
    spin_unlock_irqrestore(&mut (*mpu).timer_lock, flags);
    if !(*mpu).rmidi.is_null() {
        _snd_mpu401_uart_interrupt(mpu);
    }
}

unsafe fn timer_container_of_mpu_timer(t: *mut timer_list) -> *mut snd_mpu401 {
    (t as *mut u8).sub(offset_of_timer()) as *mut snd_mpu401
}

unsafe fn offset_of_timer() -> usize {
    let uninit = core::mem::MaybeUninit::<snd_mpu401>::uninit();
    let base = uninit.as_ptr();
    (&(*base).timer as *const timer_list as usize) - (base as usize)
}

/*
 * initialize the timer callback if not programmed yet
 */
unsafe fn snd_mpu401_uart_add_timer(mpu: *mut snd_mpu401, input: c_int) {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*mpu).timer_lock, &mut flags);
    if (*mpu).timer_invoked == 0 {
        timer_setup(&mut (*mpu).timer, snd_mpu401_uart_timer, 0);
        mod_timer(&mut (*mpu).timer, 1 + jiffies);
    }
    (*mpu).timer_invoked |= if input != 0 {
        MPU401_MODE_INPUT_TIMER
    } else {
        MPU401_MODE_OUTPUT_TIMER
    };
    spin_unlock_irqrestore(&mut (*mpu).timer_lock, flags);
}

/*
 * remove the timer callback if still active
 */
unsafe fn snd_mpu401_uart_remove_timer(mpu: *mut snd_mpu401, input: c_int) {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*mpu).timer_lock, &mut flags);
    if (*mpu).timer_invoked != 0 {
        (*mpu).timer_invoked &= if input != 0 {
            !MPU401_MODE_INPUT_TIMER
        } else {
            !MPU401_MODE_OUTPUT_TIMER
        };
        if (*mpu).timer_invoked == 0 {
            timer_delete(&mut (*mpu).timer);
        }
    }
    spin_unlock_irqrestore(&mut (*mpu).timer_lock, flags);
}

/*
 * send a UART command
 * return zero if successful, non-zero for some errors
 */
unsafe fn snd_mpu401_uart_cmd(mpu: *mut snd_mpu401, cmd: c_uchar, ack: c_int) -> c_int {
    let mut timeout: c_int;
    let mut ok: c_int;

    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*mpu).input_lock, &mut flags);
    if (*mpu).hardware != MPU401_HW_TRID4DWAVE {
        ((*mpu).write.unwrap())(mpu, 0x00, MPU401D(mpu));
        /*snd_mpu401_uart_clear_rx(mpu);*/
    }
    /* ok. standard MPU-401 initialization */
    if (*mpu).hardware != MPU401_HW_SB {
        timeout = 1000;
        while timeout > 0 && !snd_mpu401_output_ready(mpu) {
            timeout -= 1;
            udelay(10);
        }
        // CONFIG_SND_DEBUG:
        // if !timeout, report "cmd: tx timeout".
    }
    ((*mpu).write.unwrap())(mpu, cmd, MPU401C(mpu));
    if ack != 0 && ((*mpu).info_flags & MPU401_INFO_NO_ACK) == 0 {
        ok = 0;
        timeout = 10000;
        while ok == 0 && {
            let old = timeout;
            timeout -= 1;
            old > 0
        } {
            if snd_mpu401_input_avail(mpu) {
                if ((*mpu).read.unwrap())(mpu, MPU401D(mpu)) == MPU401_ACK {
                    ok = 1;
                }
            }
        }
        if ok == 0 && ((*mpu).read.unwrap())(mpu, MPU401D(mpu)) == MPU401_ACK {
            ok = 1;
        }
    } else {
        ok = 1;
    }
    spin_unlock_irqrestore(&mut (*mpu).input_lock, flags);
    if ok == 0 {
        dev_err(
            (*(*mpu).rmidi).dev,
            b"cmd: 0x%x failed at 0x%lx (status = 0x%x, data = 0x%x)\n\0".as_ptr() as *const c_char,
            cmd as c_uint,
            (*mpu).port,
            ((*mpu).read.unwrap())(mpu, MPU401C(mpu)) as c_uint,
            ((*mpu).read.unwrap())(mpu, MPU401D(mpu)) as c_uint,
        );
        return 1;
    }
    0
}

unsafe fn snd_mpu401_do_reset(mpu: *mut snd_mpu401) -> c_int {
    if snd_mpu401_uart_cmd(mpu, MPU401_RESET, 1) != 0 {
        return -EIO;
    }
    if snd_mpu401_uart_cmd(mpu, MPU401_ENTER_UART, 0) != 0 {
        return -EIO;
    }
    0
}

/*
 * input/output open/close - protected by open_mutex in rawmidi.c
 */
unsafe extern "C" fn snd_mpu401_uart_input_open(substream: *mut snd_rawmidi_substream) -> c_int {
    let mpu: *mut snd_mpu401;
    let err: c_int;

    mpu = (*(*substream).rmidi).private_data as *mut snd_mpu401;
    if let Some(open_input) = (*mpu).open_input {
        err = open_input(mpu);
        if err < 0 {
            return err;
        }
    }
    if !test_bit(MPU401_MODE_BIT_OUTPUT, &(*mpu).mode) {
        if snd_mpu401_do_reset(mpu) < 0 {
            goto_error_out_input_open(mpu);
            return -EIO;
        }
    }
    (*mpu).substream_input = substream;
    set_bit(MPU401_MODE_BIT_INPUT, &mut (*mpu).mode);
    0
}

unsafe fn goto_error_out_input_open(mpu: *mut snd_mpu401) {
    if (*mpu).open_input.is_some() && (*mpu).close_input.is_some() {
        ((*mpu).close_input.unwrap())(mpu);
    }
}

unsafe extern "C" fn snd_mpu401_uart_output_open(substream: *mut snd_rawmidi_substream) -> c_int {
    let mpu: *mut snd_mpu401;
    let err: c_int;

    mpu = (*(*substream).rmidi).private_data as *mut snd_mpu401;
    if let Some(open_output) = (*mpu).open_output {
        err = open_output(mpu);
        if err < 0 {
            return err;
        }
    }
    if !test_bit(MPU401_MODE_BIT_INPUT, &(*mpu).mode) {
        if snd_mpu401_do_reset(mpu) < 0 {
            goto_error_out_output_open(mpu);
            return -EIO;
        }
    }
    (*mpu).substream_output = substream;
    set_bit(MPU401_MODE_BIT_OUTPUT, &mut (*mpu).mode);
    0
}

unsafe fn goto_error_out_output_open(mpu: *mut snd_mpu401) {
    if (*mpu).open_output.is_some() && (*mpu).close_output.is_some() {
        ((*mpu).close_output.unwrap())(mpu);
    }
}

unsafe extern "C" fn snd_mpu401_uart_input_close(substream: *mut snd_rawmidi_substream) -> c_int {
    let mpu: *mut snd_mpu401;
    let mut err: c_int = 0;

    mpu = (*(*substream).rmidi).private_data as *mut snd_mpu401;
    clear_bit(MPU401_MODE_BIT_INPUT, &mut (*mpu).mode);
    (*mpu).substream_input = ptr::null_mut();
    if !test_bit(MPU401_MODE_BIT_OUTPUT, &(*mpu).mode) {
        err = snd_mpu401_uart_cmd(mpu, MPU401_RESET, 0);
    }
    if let Some(close_input) = (*mpu).close_input {
        close_input(mpu);
    }
    if err != 0 {
        return -EIO;
    }
    0
}

unsafe extern "C" fn snd_mpu401_uart_output_close(substream: *mut snd_rawmidi_substream) -> c_int {
    let mpu: *mut snd_mpu401;
    let mut err: c_int = 0;

    mpu = (*(*substream).rmidi).private_data as *mut snd_mpu401;
    clear_bit(MPU401_MODE_BIT_OUTPUT, &mut (*mpu).mode);
    (*mpu).substream_output = ptr::null_mut();
    if !test_bit(MPU401_MODE_BIT_INPUT, &(*mpu).mode) {
        err = snd_mpu401_uart_cmd(mpu, MPU401_RESET, 0);
    }
    if let Some(close_output) = (*mpu).close_output {
        close_output(mpu);
    }
    if err != 0 {
        return -EIO;
    }
    0
}

/*
 * trigger input callback
 */
unsafe extern "C" fn snd_mpu401_uart_input_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    let mpu: *mut snd_mpu401;
    let mut max: c_int = 64;

    mpu = (*(*substream).rmidi).private_data as *mut snd_mpu401;
    if up != 0 {
        if !test_and_set_bit(MPU401_MODE_BIT_INPUT_TRIGGER, &mut (*mpu).mode) {
            /* first time - flush FIFO */
            while {
                let old = max;
                max -= 1;
                old > 0
            } {
                ((*mpu).read.unwrap())(mpu, MPU401D(mpu));
            }
            if ((*mpu).info_flags & MPU401_INFO_USE_TIMER) != 0 {
                snd_mpu401_uart_add_timer(mpu, 1);
            }
        }

        /* read data in advance */
        let mut flags: c_ulong = 0;
        spin_lock_irqsave(&mut (*mpu).input_lock, &mut flags);
        snd_mpu401_uart_input_read(mpu);
        spin_unlock_irqrestore(&mut (*mpu).input_lock, flags);
    } else {
        if ((*mpu).info_flags & MPU401_INFO_USE_TIMER) != 0 {
            snd_mpu401_uart_remove_timer(mpu, 1);
        }
        clear_bit(MPU401_MODE_BIT_INPUT_TRIGGER, &mut (*mpu).mode);
    }
}

/*
 * transfer input pending data
 * call with input_lock spinlock held
 */
unsafe fn snd_mpu401_uart_input_read(mpu: *mut snd_mpu401) {
    let mut max: c_int = 128;
    let mut byte: c_uchar;

    while {
        let old = max;
        max -= 1;
        old > 0
    } {
        if !snd_mpu401_input_avail(mpu) {
            break; /* input not available */
        }
        byte = ((*mpu).read.unwrap())(mpu, MPU401D(mpu));
        if test_bit(MPU401_MODE_BIT_INPUT_TRIGGER, &(*mpu).mode) {
            snd_rawmidi_receive((*mpu).substream_input, &mut byte, 1);
        }
    }
}

/*
 *  Tx FIFO sizes:
 *    CS4237B			- 16 bytes
 *    AudioDrive ES1688         - 12 bytes
 *    S3 SonicVibes             -  8 bytes
 *    SoundBlaster AWE 64       -  2 bytes (ugly hardware)
 */

/*
 * write output pending bytes
 * call with output_lock spinlock held
 */
unsafe fn snd_mpu401_uart_output_write(mpu: *mut snd_mpu401) {
    let mut byte: c_uchar = 0;
    let mut max: c_int = 256;

    loop {
        if snd_rawmidi_transmit_peek((*mpu).substream_output, &mut byte, 1) == 1 {
            /*
             * Try twice because there is hardware that insists on
             * setting the output busy bit after each write.
             */
            if !snd_mpu401_output_ready(mpu) && !snd_mpu401_output_ready(mpu) {
                break; /* Tx FIFO full - try again later */
            }
            ((*mpu).write.unwrap())(mpu, byte, MPU401D(mpu));
            snd_rawmidi_transmit_ack((*mpu).substream_output, 1);
        } else {
            snd_mpu401_uart_remove_timer(mpu, 0);
            break; /* no other data - leave the tx loop */
        }
        max -= 1;
        if max <= 0 {
            break;
        }
    }
}

/*
 * output trigger callback
 */
unsafe extern "C" fn snd_mpu401_uart_output_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    let mpu: *mut snd_mpu401;

    mpu = (*(*substream).rmidi).private_data as *mut snd_mpu401;
    if up != 0 {
        set_bit(MPU401_MODE_BIT_OUTPUT_TRIGGER, &mut (*mpu).mode);

        /* try to add the timer at each output trigger,
         * since the output timer might have been removed in
         * snd_mpu401_uart_output_write().
         */
        if ((*mpu).info_flags & MPU401_INFO_TX_IRQ) == 0 {
            snd_mpu401_uart_add_timer(mpu, 0);
        }

        /* output pending data */
        let mut flags: c_ulong = 0;
        spin_lock_irqsave(&mut (*mpu).output_lock, &mut flags);
        snd_mpu401_uart_output_write(mpu);
        spin_unlock_irqrestore(&mut (*mpu).output_lock, flags);
    } else {
        if ((*mpu).info_flags & MPU401_INFO_TX_IRQ) == 0 {
            snd_mpu401_uart_remove_timer(mpu, 0);
        }
        clear_bit(MPU401_MODE_BIT_OUTPUT_TRIGGER, &mut (*mpu).mode);
    }
}

/*

 */

static snd_mpu401_uart_output: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_mpu401_uart_output_open),
    close: Some(snd_mpu401_uart_output_close),
    trigger: Some(snd_mpu401_uart_output_trigger),
};

static snd_mpu401_uart_input: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_mpu401_uart_input_open),
    close: Some(snd_mpu401_uart_input_close),
    trigger: Some(snd_mpu401_uart_input_trigger),
};

unsafe extern "C" fn snd_mpu401_uart_free(rmidi: *mut snd_rawmidi) {
    let mpu = (*rmidi).private_data as *mut snd_mpu401;
    if (*mpu).irq >= 0 {
        free_irq((*mpu).irq as c_uint, mpu as *mut c_void);
    }
    release_and_free_resource((*mpu).res);
    kfree(mpu as *mut c_void);
}

/**
 * snd_mpu401_uart_new - create an MPU401-UART instance
 * @card: the card instance
 * @device: the device index, zero-based
 * @hardware: the hardware type, MPU401_HW_XXXX
 * @port: the base address of MPU401 port
 * @info_flags: bitflags MPU401_INFO_XXX
 * @irq: the ISA irq number, -1 if not to be allocated
 * @rrawmidi: the pointer to store the new rawmidi instance
 *
 * Creates a new MPU-401 instance.
 *
 * Note that the rawmidi instance is returned on the rrawmidi argument,
 * not the mpu401 instance itself.  To access to the mpu401 instance,
 * cast from rawmidi->private_data (with struct snd_mpu401 magic-cast).
 *
 * Return: Zero if successful, or a negative error code.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_mpu401_uart_new(
    card: *mut snd_card,
    device: c_int,
    hardware: c_ushort,
    port: c_ulong,
    mut info_flags: c_uint,
    irq: c_int,
    rrawmidi: *mut *mut snd_rawmidi,
) -> c_int {
    let mut mpu: *mut snd_mpu401;
    let mut rmidi: *mut snd_rawmidi = ptr::null_mut();
    let in_enable: c_int;
    let out_enable: c_int;
    let mut err: c_int;

    if !rrawmidi.is_null() {
        *rrawmidi = ptr::null_mut();
    }
    if (info_flags & (MPU401_INFO_INPUT | MPU401_INFO_OUTPUT)) == 0 {
        info_flags |= MPU401_INFO_INPUT | MPU401_INFO_OUTPUT;
    }
    in_enable = if (info_flags & MPU401_INFO_INPUT) != 0 { 1 } else { 0 };
    out_enable = if (info_flags & MPU401_INFO_OUTPUT) != 0 { 1 } else { 0 };
    err = snd_rawmidi_new(card, b"MPU-401U\0".as_ptr() as *const c_char, device, out_enable, in_enable, &mut rmidi);
    if err < 0 {
        return err;
    }
    mpu = kzalloc(size_of::<snd_mpu401>(), 0) as *mut snd_mpu401;
    if mpu.is_null() {
        err = -ENOMEM;
        snd_device_free(card, rmidi as *mut c_void);
        return err;
    }
    (*rmidi).private_data = mpu as *mut c_void;
    (*rmidi).private_free = Some(snd_mpu401_uart_free);
    spin_lock_init(&mut (*mpu).input_lock);
    spin_lock_init(&mut (*mpu).output_lock);
    spin_lock_init(&mut (*mpu).timer_lock);
    (*mpu).hardware = hardware;
    (*mpu).irq = -1;
    (*mpu).rmidi = rmidi;
    if (info_flags & MPU401_INFO_INTEGRATED) == 0 {
        let res_size: c_int = if hardware == MPU401_HW_PC98II { 4 } else { 2 };
        (*mpu).res = request_region(port, res_size as c_ulong, b"MPU401 UART\0".as_ptr() as *const c_char);
        if (*mpu).res.is_null() {
            dev_err(
                (*rmidi).dev,
                b"mpu401_uart: unable to grab port 0x%lx size %d\n\0".as_ptr() as *const c_char,
                port,
                res_size,
            );
            err = -EBUSY;
            snd_device_free(card, rmidi as *mut c_void);
            return err;
        }
    }
    if (info_flags & MPU401_INFO_MMIO) != 0 {
        (*mpu).write = Some(mpu401_write_mmio);
        (*mpu).read = Some(mpu401_read_mmio);
    } else {
        (*mpu).write = Some(mpu401_write_port);
        (*mpu).read = Some(mpu401_read_port);
    }
    (*mpu).port = port;
    if hardware == MPU401_HW_PC98II {
        (*mpu).cport = port + 2;
    } else {
        (*mpu).cport = port + 1;
    }
    if irq >= 0 {
        if request_irq(
            irq as c_uint,
            snd_mpu401_uart_interrupt,
            0,
            b"MPU401 UART\0".as_ptr() as *const c_char,
            mpu as *mut c_void,
        ) != 0 {
            dev_err(
                (*rmidi).dev,
                b"mpu401_uart: unable to grab IRQ %d\n\0".as_ptr() as *const c_char,
                irq,
            );
            err = -EBUSY;
            snd_device_free(card, rmidi as *mut c_void);
            return err;
        }
    }
    if irq < 0 && (info_flags & MPU401_INFO_IRQ_HOOK) == 0 {
        info_flags |= MPU401_INFO_USE_TIMER;
    }
    (*mpu).info_flags = info_flags;
    (*mpu).irq = irq;
    if (*card).shortname[0] != 0 {
        snprintf(
            (*rmidi).name.as_mut_ptr(),
            (*rmidi).name.len(),
            b"%s MIDI\0".as_ptr() as *const c_char,
            (*card).shortname.as_ptr(),
        );
    } else {
        sprintf(
            (*rmidi).name.as_mut_ptr(),
            b"MPU-401 MIDI %d-%d\0".as_ptr() as *const c_char,
            (*card).number,
            device,
        );
    }
    if out_enable != 0 {
        snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_OUTPUT, &snd_mpu401_uart_output);
        (*rmidi).info_flags |= SNDRV_RAWMIDI_INFO_OUTPUT;
    }
    if in_enable != 0 {
        snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_INPUT, &snd_mpu401_uart_input);
        (*rmidi).info_flags |= SNDRV_RAWMIDI_INFO_INPUT;
        if out_enable != 0 {
            (*rmidi).info_flags |= SNDRV_RAWMIDI_INFO_DUPLEX;
        }
    }
    if !rrawmidi.is_null() {
        *rrawmidi = rmidi;
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
