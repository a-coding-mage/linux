// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *      MOTU Midi Timepiece ALSA Main routines
 *      Copyright by Michael T. Mayers (c) Jan 09, 2000
 *      mail: michael@tweakoz.com
 *      Thanks to John Galbraith
 *
 *      This driver is for the 'Mark Of The Unicorn' (MOTU)
 *      MidiTimePiece AV multiport MIDI interface
 *
 *      IOPORTS
 *      -------
 *      8 MIDI Ins and 8 MIDI outs
 *      Video Sync In (BNC), Word Sync Out (BNC),
 *      ADAT Sync Out (DB9)
 *      SMPTE in/out (1/4")
 *      2 programmable pedal/footswitch inputs and 4 programmable MIDI controller knobs.
 *      Macintosh RS422 serial port
 *      RS422 "network" port for ganging multiple MTP's
 *      PC Parallel Port ( which this driver currently uses )
 *
 *      MISC FEATURES
 *      -------------
 *      Hardware MIDI routing, merging, and filtering
 *      MIDI Synchronization to Video, ADAT, SMPTE and other Clock sources
 *      128 'scene' memories, recallable from MIDI program change
 *
 * ChangeLog
 * Jun 11 2001  Takashi Iwai <tiwai@suse.de>
 *      - Recoded & debugged
 *      - Added timer interrupt for midi outputs
 *      - hwports is between 1 and 8, which specifies the number of hardware ports.
 *        The three global ports, computer, adat and broadcast ports, are created
 *        always after h/w and remote ports.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(static_mut_refs)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = core::ffi::c_uchar;
type u16 = core::ffi::c_ushort;
type u32 = core::ffi::c_uint;
type irqreturn_t = c_uint;
type spinlock_t = c_ulong;

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
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct snd_rawmidi_stream {
    pub substreams: list_head,
}

#[repr(C)]
pub struct snd_rawmidi {
    pub private_data: *mut c_void,
    pub streams: [snd_rawmidi_stream; 2],
    pub info_flags: c_uint,
    pub name: *mut c_char,
}

#[repr(C)]
pub struct snd_rawmidi_substream {
    pub list: list_head,
    pub number: c_int,
    pub rmidi: *mut snd_rawmidi,
    pub name: *mut c_char,
    pub ops: *const snd_rawmidi_ops,
}

#[repr(C)]
pub struct snd_rawmidi_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream, c_int)>,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    pub driver: *mut c_char,
    pub shortname: *mut c_char,
    pub longname: *mut c_char,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct driver_private {
    pub name: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: driver_private,
}

unsafe extern "C" {
    static mut SNDRV_DEFAULT_IDX1: c_int;
    static mut SNDRV_DEFAULT_STR1: *mut c_char;
    static mut THIS_MODULE: *mut module;
    static mut jiffies: c_ulong;

    fn inb(port: c_ulong) -> u8;
    fn outb(value: u8, port: c_ulong);
    fn udelay(usecs: c_ulong);

    fn snd_rawmidi_transmit(substream: *mut snd_rawmidi_substream, buf: *mut u8, count: c_int) -> c_int;
    fn snd_rawmidi_receive(substream: *mut snd_rawmidi_substream, buf: *mut u8, count: c_int) -> c_int;
    fn snd_rawmidi_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        output_count: c_int,
        input_count: c_int,
        rmidi: *mut *mut snd_rawmidi,
    ) -> c_int;

    fn devm_request_region(dev: *mut device, start: c_long, n: c_ulong, name: *const c_char) -> *mut resource;
    fn devm_request_irq(
        dev: *mut device,
        irq: c_int,
        handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        flags: c_ulong,
        name: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;

    fn mod_timer(timer: *mut timer_list, expires: c_ulong) -> c_int;
    fn timer_delete(timer: *mut timer_list) -> c_int;
    fn timer_shutdown_sync(timer: *mut timer_list) -> c_int;
    fn timer_setup(timer: *mut timer_list, func: Option<unsafe extern "C" fn(*mut timer_list)>, flags: c_uint);

    fn snd_devm_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut module,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;

    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);
    fn platform_device_register_simple(
        name: *const c_char,
        id: c_int,
        res: *mut resource,
        num: c_uint,
    ) -> *mut platform_device;
    fn platform_device_unregister(pdev: *mut platform_device);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);

    fn IS_ERR(ptr: *const c_void) -> c_int;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn sprintf(dst: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snprintf(dst: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
}

// MODULE_AUTHOR("Michael T. Mayers");
// MODULE_DESCRIPTION("MOTU MidiTimePiece AV multiport MIDI");
// MODULE_LICENSE("GPL");

// io resources
const MTPAV_IOBASE: c_long = 0x378;
const MTPAV_IRQ: c_int = 7;
const MTPAV_MAX_PORTS: c_int = 8;

static mut index: c_int = 0; /* initialized from SNDRV_DEFAULT_IDX1 in C */
static mut id: *mut c_char = ptr::null_mut(); /* initialized from SNDRV_DEFAULT_STR1 in C */
static mut port: c_long = MTPAV_IOBASE; /* 0x378, 0x278 */
static mut irq: c_int = MTPAV_IRQ; /* 7, 5 */
static mut hwports: c_int = MTPAV_MAX_PORTS; /* use hardware ports 1-8 */

// module_param(index, int, 0444);
// MODULE_PARM_DESC(index, "Index value for MotuMTPAV MIDI.");
// module_param(id, charp, 0444);
// MODULE_PARM_DESC(id, "ID string for MotuMTPAV MIDI.");
// module_param_hw(port, long, ioport, 0444);
// MODULE_PARM_DESC(port, "Parallel port # for MotuMTPAV MIDI.");
// module_param_hw(irq, int, irq, 0444);
// MODULE_PARM_DESC(irq, "Parallel IRQ # for MotuMTPAV MIDI.");
// module_param(hwports, int, 0444);
// MODULE_PARM_DESC(hwports, "Hardware ports # for MotuMTPAV MIDI.");

static mut device: *mut platform_device = ptr::null_mut();

/*
 *      defines
 */
// #define USE_FAKE_MTP // don't actually read/write to MTP device (for debugging without an actual unit) (does not work yet)

// parallel port usage masks
const SIGS_BYTE: u8 = 0x08;
const SIGS_RFD: u8 = 0x80;
const SIGS_IRQ: u8 = 0x40;
const SIGS_IN0: u8 = 0x10;
const SIGS_IN1: u8 = 0x20;

const SIGC_WRITE: u8 = 0x04;
const SIGC_READ: u8 = 0x08;
const SIGC_INTEN: u8 = 0x10;

const DREG: u16 = 0;
const SREG: u16 = 1;
const CREG: u16 = 2;

const MTPAV_MODE_INPUT_OPENED: u8 = 0x01;
const MTPAV_MODE_OUTPUT_OPENED: u8 = 0x02;
const MTPAV_MODE_INPUT_TRIGGERED: u8 = 0x04;
const MTPAV_MODE_OUTPUT_TRIGGERED: u8 = 0x08;

const NUMPORTS: usize = 0x12 + 1;

#[repr(C)]
pub struct mtpav_port {
    pub number: u8,
    pub hwport: u8,
    pub mode: u8,
    pub running_status: u8,
    pub input: *mut snd_rawmidi_substream,
    pub output: *mut snd_rawmidi_substream,
}

#[repr(C)]
pub struct mtpav {
    pub card: *mut snd_card,
    pub port: c_ulong,
    pub res_port: *mut resource,
    pub irq: c_int, /* interrupt (for inputs) */
    pub spinlock: spinlock_t,
    pub share_irq: c_int, /* number of accesses to input interrupts */
    pub istimer: c_int, /* number of accesses to timer interrupts */
    pub timer: timer_list, /* timer interrupts for outputs */
    pub rmidi: *mut snd_rawmidi,
    pub num_ports: c_int, /* number of hw ports (1-8) */
    pub ports: [mtpav_port; NUMPORTS], /* all ports including computer, adat and bc */

    pub inmidiport: u32, /* selected input midi port */
    pub inmidistate: u32, /* during midi command 0xf5 */

    pub outmidihwport: u32, /* selected output midi hw port */
}

/*
 * possible hardware ports (selected by 0xf5 port message)
 *      0x00            all ports
 *      0x01 .. 0x08    this MTP's ports 1..8
 *      0x09 .. 0x10    networked MTP's ports (9..16)
 *      0x11            networked MTP's computer port
 *      0x63            to ADAT
 *
 * mappig:
 *  subdevice 0 - (X-1)    ports
 *            X - (2*X-1)  networked ports
 *            X            computer
 *            X+1          ADAT
 *            X+2          all ports
 *
 *  where X = chip->num_ports
 */

const MTPAV_PIDX_COMPUTER: c_int = 0;
const MTPAV_PIDX_ADAT: c_int = 1;
const MTPAV_PIDX_BROADCAST: c_int = 2;

unsafe fn translate_subdevice_to_hwport(chip: *mut mtpav, subdev: c_int) -> c_int {
    if subdev < 0 {
        0x01 /* invalid - use port 0 as default */
    } else if subdev < (*chip).num_ports {
        subdev + 1 /* single mtp port */
    } else if subdev < (*chip).num_ports * 2 {
        subdev - (*chip).num_ports + 0x09 /* remote port */
    } else if subdev == (*chip).num_ports * 2 + MTPAV_PIDX_COMPUTER {
        0x11 /* computer port */
    } else if subdev == (*chip).num_ports + MTPAV_PIDX_ADAT {
        0x63 /* ADAT */
    } else {
        0 /* all ports */
    }
}

unsafe fn translate_hwport_to_subdevice(chip: *mut mtpav, hwport: c_int) -> c_int {
    let mut p: c_int;
    if hwport <= 0x00 {
        /* all ports */
        (*chip).num_ports + MTPAV_PIDX_BROADCAST
    } else if hwport <= 0x08 {
        /* single port */
        p = hwport - 1;
        if p >= (*chip).num_ports {
            p = 0;
        }
        p
    } else if hwport <= 0x10 {
        /* remote port */
        p = hwport - 0x09 + (*chip).num_ports;
        if p >= (*chip).num_ports * 2 {
            p = (*chip).num_ports;
        }
        p
    } else if hwport == 0x11 {
        /* computer port */
        (*chip).num_ports + MTPAV_PIDX_COMPUTER
    } else {
        /* ADAT */
        (*chip).num_ports + MTPAV_PIDX_ADAT
    }
}

unsafe fn snd_mtpav_getreg(chip: *mut mtpav, reg: u16) -> u8 {
    let mut rval: u8 = 0;

    if reg == SREG {
        rval = inb((*chip).port + SREG as c_ulong);
        rval &= 0xf8;
    } else if reg == CREG {
        rval = inb((*chip).port + CREG as c_ulong);
        rval &= 0x1c;
    }

    rval
}

#[inline]
unsafe fn snd_mtpav_mputreg(chip: *mut mtpav, reg: u16, val: u8) {
    if reg == DREG || reg == CREG {
        outb(val, (*chip).port + reg as c_ulong);
    }
}

unsafe fn snd_mtpav_wait_rfdhi(chip: *mut mtpav) {
    let mut counts: c_int = 10000;
    let mut sbyte: u8;

    sbyte = snd_mtpav_getreg(chip, SREG);
    while (sbyte & SIGS_RFD) == 0 && counts != 0 {
        counts -= 1;
        sbyte = snd_mtpav_getreg(chip, SREG);
        udelay(10);
    }
}

unsafe fn snd_mtpav_send_byte(chip: *mut mtpav, byte: u8) {
    let tcbyt: u8;
    let clrwrite: u8;
    let setwrite: u8;

    snd_mtpav_wait_rfdhi(chip);

    //////////////////

    tcbyt = snd_mtpav_getreg(chip, CREG);
    clrwrite = tcbyt & (SIGC_WRITE ^ 0xff);
    setwrite = tcbyt | SIGC_WRITE;

    snd_mtpav_mputreg(chip, DREG, byte);
    snd_mtpav_mputreg(chip, CREG, clrwrite); // clear write bit

    snd_mtpav_mputreg(chip, CREG, setwrite); // set write bit
}

/* call this with spin lock held */
unsafe fn snd_mtpav_output_port_write(
    mtp_card: *mut mtpav,
    portp: *mut mtpav_port,
    substream: *mut snd_rawmidi_substream,
) {
    let mut outbyte: u8 = 0;

    // Get the outbyte first, so we can emulate running status if
    // necessary
    if snd_rawmidi_transmit(substream, &mut outbyte, 1) != 1 {
        return;
    }

    // send port change command if necessary

    if (*portp).hwport as u32 != (*mtp_card).outmidihwport {
        (*mtp_card).outmidihwport = (*portp).hwport as u32;

        snd_mtpav_send_byte(mtp_card, 0xf5);
        snd_mtpav_send_byte(mtp_card, (*portp).hwport);
        if (outbyte & 0x80) == 0 && (*portp).running_status != 0 {
            snd_mtpav_send_byte(mtp_card, (*portp).running_status);
        }
    }

    // send data

    loop {
        if (outbyte & 0x80) != 0 {
            (*portp).running_status = outbyte;
        }

        snd_mtpav_send_byte(mtp_card, outbyte);
        if snd_rawmidi_transmit(substream, &mut outbyte, 1) != 1 {
            break;
        }
    }
}

unsafe fn snd_mtpav_output_write(substream: *mut snd_rawmidi_substream) {
    let mtp_card: *mut mtpav = (*(*substream).rmidi).private_data as *mut mtpav;
    let portp: *mut mtpav_port = (*mtp_card).ports.as_mut_ptr().add((*substream).number as usize);

    // guard(spinlock_irqsave)(&mtp_card->spinlock);
    snd_mtpav_output_port_write(mtp_card, portp, substream);
}

/*
 *      mtpav control
 */

unsafe fn snd_mtpav_portscan(chip: *mut mtpav) {
    // put mtp into smart routing mode
    let mut p: u8;

    p = 0;
    while p < 8 {
        snd_mtpav_send_byte(chip, 0xf5);
        snd_mtpav_send_byte(chip, p);
        snd_mtpav_send_byte(chip, 0xfe);
        p = p.wrapping_add(1);
    }
}

unsafe extern "C" fn snd_mtpav_input_open(substream: *mut snd_rawmidi_substream) -> c_int {
    let mtp_card: *mut mtpav = (*(*substream).rmidi).private_data as *mut mtpav;
    let portp: *mut mtpav_port = (*mtp_card).ports.as_mut_ptr().add((*substream).number as usize);

    // guard(spinlock_irqsave)(&mtp_card->spinlock);
    (*portp).mode |= MTPAV_MODE_INPUT_OPENED;
    (*portp).input = substream;
    if (*mtp_card).share_irq == 0 {
        (*mtp_card).share_irq += 1;
        snd_mtpav_mputreg(mtp_card, CREG, SIGC_INTEN | SIGC_WRITE); // enable pport interrupts
    } else {
        (*mtp_card).share_irq += 1;
    }
    0
}

unsafe extern "C" fn snd_mtpav_input_close(substream: *mut snd_rawmidi_substream) -> c_int {
    let mtp_card: *mut mtpav = (*(*substream).rmidi).private_data as *mut mtpav;
    let portp: *mut mtpav_port = (*mtp_card).ports.as_mut_ptr().add((*substream).number as usize);

    // guard(spinlock_irqsave)(&mtp_card->spinlock);
    (*portp).mode &= !MTPAV_MODE_INPUT_OPENED;
    (*portp).input = ptr::null_mut();
    (*mtp_card).share_irq -= 1;
    if (*mtp_card).share_irq == 0 {
        snd_mtpav_mputreg(mtp_card, CREG, 0); // disable pport interrupts
    }
    0
}

unsafe extern "C" fn snd_mtpav_input_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    let mtp_card: *mut mtpav = (*(*substream).rmidi).private_data as *mut mtpav;
    let portp: *mut mtpav_port = (*mtp_card).ports.as_mut_ptr().add((*substream).number as usize);

    // guard(spinlock_irqsave)(&mtp_card->spinlock);
    if up != 0 {
        (*portp).mode |= MTPAV_MODE_INPUT_TRIGGERED;
    } else {
        (*portp).mode &= !MTPAV_MODE_INPUT_TRIGGERED;
    }
}

/*
 * timer interrupt for outputs
 */

unsafe extern "C" fn snd_mtpav_output_timer(t: *mut timer_list) {
    // struct mtpav *chip = timer_container_of(chip, t, timer);
    let chip: *mut mtpav = timer_container_of_mtpav(t);
    let mut p: c_int;

    // guard(spinlock_irqsave)(&chip->spinlock);
    /* reprogram timer */
    mod_timer(&mut (*chip).timer, 1 + jiffies);
    /* process each port */
    p = 0;
    while p <= (*chip).num_ports * 2 + MTPAV_PIDX_BROADCAST {
        let portp: *mut mtpav_port = (*chip).ports.as_mut_ptr().add(p as usize);
        if ((*portp).mode & MTPAV_MODE_OUTPUT_TRIGGERED) != 0 && !(*portp).output.is_null() {
            snd_mtpav_output_port_write(chip, portp, (*portp).output);
        }
        p += 1;
    }
}

unsafe fn timer_container_of_mtpav(_t: *mut timer_list) -> *mut mtpav {
    // Rust translation placeholder for timer_container_of(chip, t, timer).
    ptr::null_mut()
}

/* spinlock held! */
unsafe fn snd_mtpav_add_output_timer(chip: *mut mtpav) {
    mod_timer(&mut (*chip).timer, 1 + jiffies);
}

/* spinlock held! */
unsafe fn snd_mtpav_remove_output_timer(chip: *mut mtpav) {
    timer_delete(&mut (*chip).timer);
}

unsafe extern "C" fn snd_mtpav_output_open(substream: *mut snd_rawmidi_substream) -> c_int {
    let mtp_card: *mut mtpav = (*(*substream).rmidi).private_data as *mut mtpav;
    let portp: *mut mtpav_port = (*mtp_card).ports.as_mut_ptr().add((*substream).number as usize);

    // guard(spinlock_irqsave)(&mtp_card->spinlock);
    (*portp).mode |= MTPAV_MODE_OUTPUT_OPENED;
    (*portp).output = substream;
    0
}

unsafe extern "C" fn snd_mtpav_output_close(substream: *mut snd_rawmidi_substream) -> c_int {
    let mtp_card: *mut mtpav = (*(*substream).rmidi).private_data as *mut mtpav;
    let portp: *mut mtpav_port = (*mtp_card).ports.as_mut_ptr().add((*substream).number as usize);

    // guard(spinlock_irqsave)(&mtp_card->spinlock);
    (*portp).mode &= !MTPAV_MODE_OUTPUT_OPENED;
    (*portp).output = ptr::null_mut();
    0
}

unsafe extern "C" fn snd_mtpav_output_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    let mtp_card: *mut mtpav = (*(*substream).rmidi).private_data as *mut mtpav;
    let portp: *mut mtpav_port = (*mtp_card).ports.as_mut_ptr().add((*substream).number as usize);

    // scoped_guard(spinlock_irqsave, &mtp_card->spinlock) {
    if up != 0 {
        if ((*portp).mode & MTPAV_MODE_OUTPUT_TRIGGERED) != 0 {
            if (*mtp_card).istimer == 0 {
                (*mtp_card).istimer += 1;
                snd_mtpav_add_output_timer(mtp_card);
            } else {
                (*mtp_card).istimer += 1;
            }
            (*portp).mode |= MTPAV_MODE_OUTPUT_TRIGGERED;
        }
    } else {
        (*portp).mode &= !MTPAV_MODE_OUTPUT_TRIGGERED;
        (*mtp_card).istimer -= 1;
        if (*mtp_card).istimer == 0 {
            snd_mtpav_remove_output_timer(mtp_card);
        }
    }
    // }

    if up != 0 {
        snd_mtpav_output_write(substream);
    }
}

/*
 * midi interrupt for inputs
 */

unsafe fn snd_mtpav_inmidi_process(mcrd: *mut mtpav, inbyte: u8) {
    let portp: *mut mtpav_port;
    let mut byte = inbyte;

    if (*mcrd).inmidiport as c_int > (*mcrd).num_ports * 2 + MTPAV_PIDX_BROADCAST {
        return;
    }

    portp = (*mcrd).ports.as_mut_ptr().add((*mcrd).inmidiport as usize);
    if ((*portp).mode & MTPAV_MODE_INPUT_TRIGGERED) != 0 {
        snd_rawmidi_receive((*portp).input, &mut byte, 1);
    }
}

unsafe fn snd_mtpav_inmidi_h(mcrd: *mut mtpav, inbyte: u8) {
    if inbyte >= 0xf8 {
        /* real-time midi code */
        snd_mtpav_inmidi_process(mcrd, inbyte);
        return;
    }

    if (*mcrd).inmidistate == 0 {
        // awaiting command
        if inbyte == 0xf5 {
            // MTP port #
            (*mcrd).inmidistate = 1;
        } else {
            snd_mtpav_inmidi_process(mcrd, inbyte);
        }
    } else if (*mcrd).inmidistate != 0 {
        (*mcrd).inmidiport = translate_hwport_to_subdevice(mcrd, inbyte as c_int) as u32;
        (*mcrd).inmidistate = 0;
    }
}

unsafe fn snd_mtpav_read_bytes(mcrd: *mut mtpav) {
    let clrread: u8;
    let setread: u8;
    let mut mtp_read_byte: u8;
    let mut sr: u8;
    let cbyt: u8;
    let mut i: c_int;

    let mut sbyt: u8 = snd_mtpav_getreg(mcrd, SREG);

    if (sbyt & SIGS_BYTE) == 0 {
        return;
    }

    cbyt = snd_mtpav_getreg(mcrd, CREG);
    clrread = cbyt & (SIGC_READ ^ 0xff);
    setread = cbyt | SIGC_READ;

    loop {
        mtp_read_byte = 0;
        i = 0;
        while i < 4 {
            snd_mtpav_mputreg(mcrd, CREG, setread);
            sr = snd_mtpav_getreg(mcrd, SREG);
            snd_mtpav_mputreg(mcrd, CREG, clrread);

            sr &= SIGS_IN0 | SIGS_IN1;
            sr >>= 4;
            mtp_read_byte |= sr << (i * 2);
            i += 1;
        }

        snd_mtpav_inmidi_h(mcrd, mtp_read_byte);

        sbyt = snd_mtpav_getreg(mcrd, SREG);

        if (sbyt & SIGS_BYTE) == 0 {
            break;
        }
    }
}

unsafe extern "C" fn snd_mtpav_irqh(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let mcard: *mut mtpav = dev_id as *mut mtpav;

    // guard(spinlock)(&mcard->spinlock);
    snd_mtpav_read_bytes(mcard);
    IRQ_HANDLED
}

/*
 * get ISA resources
 */
unsafe fn snd_mtpav_get_ISA(mcard: *mut mtpav) -> c_int {
    (*mcard).res_port = devm_request_region(
        (*(*mcard).card).dev,
        port,
        3,
        c"MotuMTPAV MIDI".as_ptr(),
    );
    if (*mcard).res_port.is_null() {
        dev_err((*(*mcard).card).dev, c"MTVAP port 0x%lx is busy\n".as_ptr(), port);
        return -EBUSY;
    }
    (*mcard).port = port as c_ulong;
    if devm_request_irq(
        (*(*mcard).card).dev,
        irq,
        Some(snd_mtpav_irqh),
        0,
        c"MOTU MTPAV".as_ptr(),
        mcard as *mut c_void,
    ) != 0
    {
        dev_err((*(*mcard).card).dev, c"MTVAP IRQ %d busy\n".as_ptr(), irq);
        return -EBUSY;
    }
    (*mcard).irq = irq;
    0
}

static snd_mtpav_output: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_mtpav_output_open),
    close: Some(snd_mtpav_output_close),
    trigger: Some(snd_mtpav_output_trigger),
};

static snd_mtpav_input: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_mtpav_input_open),
    close: Some(snd_mtpav_input_close),
    trigger: Some(snd_mtpav_input_trigger),
};

/*
 * get RAWMIDI resources
 */

unsafe fn snd_mtpav_set_name(chip: *mut mtpav, substream: *mut snd_rawmidi_substream) {
    if (*substream).number >= 0 && (*substream).number < (*chip).num_ports {
        sprintf(
            (*substream).name,
            c"MTP direct %d".as_ptr(),
            ((*substream).number % (*chip).num_ports) + 1,
        );
    } else if (*substream).number >= 8 && (*substream).number < (*chip).num_ports * 2 {
        sprintf(
            (*substream).name,
            c"MTP remote %d".as_ptr(),
            ((*substream).number % (*chip).num_ports) + 1,
        );
    } else if (*substream).number == (*chip).num_ports * 2 {
        strscpy((*substream).name, c"MTP computer".as_ptr());
    } else if (*substream).number == (*chip).num_ports * 2 + 1 {
        strscpy((*substream).name, c"MTP ADAT".as_ptr());
    } else {
        strscpy((*substream).name, c"MTP broadcast".as_ptr());
    }
}

unsafe fn list_entry_snd_rawmidi_substream(ptr: *mut list_head) -> *mut snd_rawmidi_substream {
    // Rust translation placeholder for list_entry(list, struct snd_rawmidi_substream, list).
    ptr as *mut snd_rawmidi_substream
}

unsafe fn snd_mtpav_get_RAWMIDI(mcard: *mut mtpav) -> c_int {
    let mut rval: c_int;
    let rawmidi: *mut snd_rawmidi;
    let mut substream: *mut snd_rawmidi_substream;
    let mut list: *mut list_head;

    if hwports < 1 {
        hwports = 1;
    } else if hwports > 8 {
        hwports = 8;
    }
    (*mcard).num_ports = hwports;

    rval = snd_rawmidi_new(
        (*mcard).card,
        c"MotuMIDI".as_ptr(),
        0,
        (*mcard).num_ports * 2 + MTPAV_PIDX_BROADCAST + 1,
        (*mcard).num_ports * 2 + MTPAV_PIDX_BROADCAST + 1,
        &mut (*mcard).rmidi,
    );
    if rval < 0 {
        return rval;
    }
    rawmidi = (*mcard).rmidi;
    (*rawmidi).private_data = mcard as *mut c_void;

    list = (*rawmidi).streams[SNDRV_RAWMIDI_STREAM_INPUT as usize].substreams.next;
    while list != &mut (*rawmidi).streams[SNDRV_RAWMIDI_STREAM_INPUT as usize].substreams {
        substream = list_entry_snd_rawmidi_substream(list);
        snd_mtpav_set_name(mcard, substream);
        (*substream).ops = &snd_mtpav_input;
        list = (*list).next;
    }
    list = (*rawmidi).streams[SNDRV_RAWMIDI_STREAM_OUTPUT as usize].substreams.next;
    while list != &mut (*rawmidi).streams[SNDRV_RAWMIDI_STREAM_OUTPUT as usize].substreams {
        substream = list_entry_snd_rawmidi_substream(list);
        snd_mtpav_set_name(mcard, substream);
        (*substream).ops = &snd_mtpav_output;
        (*mcard).ports[(*substream).number as usize].hwport =
            translate_subdevice_to_hwport(mcard, (*substream).number) as u8;
        list = (*list).next;
    }
    (*rawmidi).info_flags |=
        SNDRV_RAWMIDI_INFO_OUTPUT | SNDRV_RAWMIDI_INFO_INPUT | SNDRV_RAWMIDI_INFO_DUPLEX;
    sprintf((*rawmidi).name, c"MTP AV MIDI".as_ptr());
    0
}

unsafe extern "C" fn snd_mtpav_free(card: *mut snd_card) {
    let crd: *mut mtpav = (*card).private_data as *mut mtpav;

    timer_shutdown_sync(&mut (*crd).timer);
}

unsafe extern "C" fn snd_mtpav_probe(dev: *mut platform_device) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let mut err: c_int;
    let mtp_card: *mut mtpav;

    err = snd_devm_card_new(
        &mut (*dev).dev,
        index,
        id,
        THIS_MODULE,
        size_of::<mtpav>(),
        &mut card,
    );
    if err < 0 {
        return err;
    }

    mtp_card = (*card).private_data as *mut mtpav;
    spin_lock_init(&mut (*mtp_card).spinlock);
    (*mtp_card).card = card;
    (*mtp_card).irq = -1;
    (*mtp_card).share_irq = 0;
    (*mtp_card).inmidistate = 0;
    (*mtp_card).outmidihwport = 0xffffffff;
    timer_setup(&mut (*mtp_card).timer, Some(snd_mtpav_output_timer), 0);

    err = snd_mtpav_get_RAWMIDI(mtp_card);
    if err < 0 {
        return err;
    }

    (*mtp_card).inmidiport = ((*mtp_card).num_ports + MTPAV_PIDX_BROADCAST) as u32;

    err = snd_mtpav_get_ISA(mtp_card);
    if err < 0 {
        return err;
    }

    strscpy((*card).driver, c"MTPAV".as_ptr());
    strscpy((*card).shortname, c"MTPAV on parallel port".as_ptr());
    snprintf(
        (*card).longname,
        sizeof_card_longname(card),
        c"MTPAV on parallel port at 0x%lx".as_ptr(),
        port,
    );

    snd_mtpav_portscan(mtp_card);

    err = snd_card_register((*mtp_card).card);
    if err < 0 {
        return err;
    }

    (*card).private_free = Some(snd_mtpav_free);

    platform_set_drvdata(dev, card as *mut c_void);
    dev_info(
        (*card).dev,
        c"Motu MidiTimePiece on parallel port irq: %d ioport: 0x%lx\n".as_ptr(),
        irq,
        port,
    );
    0
}

unsafe fn sizeof_card_longname(_card: *mut snd_card) -> usize {
    // Rust translation placeholder for sizeof(card->longname).
    0
}

const SND_MTPAV_DRIVER: *const c_char = c"snd_mtpav".as_ptr();

static mut snd_mtpav_driver: platform_driver = platform_driver {
    probe: Some(snd_mtpav_probe),
    driver: driver_private {
        name: SND_MTPAV_DRIVER,
    },
};

unsafe fn alsa_card_mtpav_init() -> c_int {
    let mut err: c_int;

    err = platform_driver_register(&raw mut snd_mtpav_driver);
    if err < 0 {
        return err;
    }

    device = platform_device_register_simple(SND_MTPAV_DRIVER, -1, ptr::null_mut(), 0);
    if IS_ERR(device as *const c_void) == 0 {
        if !platform_get_drvdata(device).is_null() {
            return 0;
        }
        platform_device_unregister(device);
        err = -ENODEV;
    } else {
        err = PTR_ERR(device as *const c_void);
    }
    platform_driver_unregister(&raw mut snd_mtpav_driver);
    err
}

unsafe fn alsa_card_mtpav_exit() {
    platform_device_unregister(device);
    platform_driver_unregister(&raw mut snd_mtpav_driver);
}

// module_init(alsa_card_mtpav_init)
// module_exit(alsa_card_mtpav_exit)

const IRQ_HANDLED: irqreturn_t = 1;
const EBUSY: c_int = 16;
const ENODEV: c_int = 19;
const SNDRV_RAWMIDI_STREAM_INPUT: c_int = 0;
const SNDRV_RAWMIDI_STREAM_OUTPUT: c_int = 1;
const SNDRV_RAWMIDI_INFO_OUTPUT: c_uint = 0x00000001;
const SNDRV_RAWMIDI_INFO_INPUT: c_uint = 0x00000002;
const SNDRV_RAWMIDI_INFO_DUPLEX: c_uint = 0x00000004;

unsafe fn spin_lock_init(lock: *mut spinlock_t) {
    *lock = 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
