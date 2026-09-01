// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA Driver for Ego Systems Inc. (ESI) Miditerminal 4140
 *   Copyright (c) 2006 by Matthias König <mk@phasorlab.de>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

const CARD_NAME: &[u8] = b"Miditerminal 4140\0";
const DRIVER_NAME: &[u8] = b"MTS64\0";
const PLATFORM_DRIVER: &[u8] = b"snd_mts64\0";

type u8 = u8;
type u16 = u16;
type bool_ = bool;

const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS];
const SNDRV_DEFAULT_ENABLE_PNP: [bool_; SNDRV_CARDS] = [true; SNDRV_CARDS];

const ENOMEM: c_int = 12;
const EIO: c_int = 5;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;
const EINVAL: c_int = 22;

const SNDRV_CTL_ELEM_IFACE_RAWMIDI: c_int = 4;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 0x0000_0003;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_int = 2;
const SNDRV_RAWMIDI_INFO_OUTPUT: c_uint = 0x0000_0001;
const SNDRV_RAWMIDI_INFO_INPUT: c_uint = 0x0000_0002;
const SNDRV_RAWMIDI_INFO_DUPLEX: c_uint = 0x0000_0004;
const SNDRV_RAWMIDI_STREAM_OUTPUT: usize = 0;
const SNDRV_RAWMIDI_STREAM_INPUT: usize = 1;
const PARPORT_DEV_EXCL: c_uint = 1;

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX;
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR;
static mut enable: [bool_; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP;

static mut platform_devices: [*mut platform_device; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS];
static mut device_count: c_int = 0;

// module_param_array(index, int, NULL, 0444);
// MODULE_PARM_DESC(index, "Index value for " CARD_NAME " soundcard.");
// module_param_array(id, charp, NULL, 0444);
// MODULE_PARM_DESC(id, "ID string for " CARD_NAME " soundcard.");
// module_param_array(enable, bool, NULL, 0444);
// MODULE_PARM_DESC(enable, "Enable " CARD_NAME " soundcard.");
//
// MODULE_AUTHOR("Matthias Koenig <mk@phasorlab.de>");
// MODULE_DESCRIPTION("ESI Miditerminal 4140");
// MODULE_LICENSE("GPL");

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct parport {
    pub base: c_ulong,
    pub irq: c_int,
}

#[repr(C)]
pub struct pardevice {
    pub port: *mut parport,
    pub name: *const c_char,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
    pub id: c_int,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
}

#[repr(C)]
pub struct snd_rawmidi {
    pub private_data: *mut c_void,
    pub name: [c_char; 80],
    pub info_flags: c_uint,
    pub streams: [snd_rawmidi_stream; 2],
}

#[repr(C)]
pub struct snd_rawmidi_stream {
    pub substreams: list_head,
}

#[repr(C)]
pub struct snd_rawmidi_substream {
    pub rmidi: *mut snd_rawmidi,
    pub number: c_int,
    pub name: [c_char; 32],
    pub list: list_head,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_long,
}

#[repr(C)]
pub struct snd_ctl_elem_integer_info {
    pub min: c_long,
    pub max: c_long,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_integer_info,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_int,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub struct snd_ctl_elem_integer_value {
    pub value: [c_long; 128],
}

#[repr(C)]
pub struct snd_ctl_elem_enumerated_value {
    pub item: [c_uint; 128],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_integer_value,
    pub enumerated: snd_ctl_elem_enumerated_value,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_int,
    pub name: *const c_char,
    pub index: c_uint,
    pub access: c_uint,
    pub private_value: c_long,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
pub struct snd_rawmidi_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream, c_int)>,
}

#[repr(C)]
pub struct pardev_cb {
    pub preempt: Option<unsafe extern "C" fn(*mut c_void)>,
    pub wakeup: Option<unsafe extern "C" fn(*mut c_void)>,
    pub irq_func: Option<unsafe extern "C" fn(*mut c_void)>,
    pub flags: c_uint,
    pub private: *mut c_void,
}

#[repr(C)]
pub struct parport_driver {
    pub name: *const c_char,
    pub probe: Option<unsafe extern "C" fn(*mut pardevice) -> c_int>,
    pub match_port: Option<unsafe extern "C" fn(*mut parport)>,
    pub detach: Option<unsafe extern "C" fn(*mut parport)>,
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: platform_driver_inner,
}

unsafe extern "C" {
    fn kfree(ptr: *mut c_void);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn msleep(msecs: c_uint);
    fn parport_read_control(p: *mut parport) -> u8;
    fn parport_write_control(p: *mut parport, c: u8);
    fn parport_read_status(p: *mut parport) -> u8;
    fn parport_write_data(p: *mut parport, c: u8);
    fn parport_release(pardev: *mut pardevice);
    fn parport_unregister_device(pardev: *mut pardevice);
    fn parport_claim(pardev: *mut pardevice) -> c_int;
    fn parport_register_dev_model(
        port: *mut parport,
        name: *const c_char,
        callbacks: *mut pardev_cb,
        device: c_int,
    ) -> *mut pardevice;
    fn parport_register_driver(driver: *mut parport_driver) -> c_int;
    fn parport_unregister_driver(driver: *mut parport_driver);
    fn platform_device_alloc(name: *const c_char, id: c_int) -> *mut platform_device;
    fn platform_device_add(device: *mut platform_device) -> c_int;
    fn platform_device_put(device: *mut platform_device);
    fn platform_device_unregister(device: *mut platform_device);
    fn platform_set_drvdata(device: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(device: *mut platform_device) -> *mut c_void;
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);
    fn snd_kcontrol_chip(kctl: *mut snd_kcontrol) -> *mut c_void;
    fn snd_ctl_boolean_mono_info(kctl: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_ctl_enum_info(
        uinfo: *mut snd_ctl_elem_info,
        channels: c_uint,
        items: c_uint,
        texts: *const *const c_char,
    ) -> c_int;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_rawmidi_transmit_peek(
        substream: *mut snd_rawmidi_substream,
        buffer: *mut u8,
        count: c_int,
    ) -> c_int;
    fn snd_rawmidi_transmit_ack(substream: *mut snd_rawmidi_substream, count: c_int) -> c_int;
    fn snd_rawmidi_receive(
        substream: *mut snd_rawmidi_substream,
        buffer: *mut u8,
        count: c_int,
    ) -> c_int;
    fn snd_rawmidi_new(
        card: *mut snd_card,
        id: *const u8,
        device: c_int,
        output_count: c_int,
        input_count: c_int,
        rmidi: *mut *mut snd_rawmidi,
    ) -> c_int;
    fn snd_rawmidi_set_ops(rmidi: *mut snd_rawmidi, stream: c_int, ops: *const snd_rawmidi_ops);
    fn snd_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut c_void,
        extra_size: c_int,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_card_free(card: *mut snd_card);
    fn strscpy(dst: *mut c_char, src: *const u8) -> isize;
    fn sprintf(dst: *mut c_char, fmt: *const u8, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const u8) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const u8, ...);
    fn dev_warn(dev: *mut device, fmt: *const u8, ...);
    fn dev_info(dev: *mut device, fmt: *const u8, ...);
}

/*********************************************************************
 * Chip specific
 *********************************************************************/
const MTS64_NUM_INPUT_PORTS: usize = 5;
const MTS64_NUM_OUTPUT_PORTS: c_int = 4;
const MTS64_SMPTE_SUBSTREAM: c_int = 4;

#[repr(C)]
pub struct mts64 {
    lock: spinlock_t,
    card: *mut snd_card,
    rmidi: *mut snd_rawmidi,
    pardev: *mut pardevice,
    open_count: c_int,
    current_midi_output_port: c_int,
    current_midi_input_port: c_int,
    mode: [u8; MTS64_NUM_INPUT_PORTS],
    midi_input_substream: [*mut snd_rawmidi_substream; MTS64_NUM_INPUT_PORTS],
    smpte_switch: c_int,
    time: [u8; 4], /* [0]=hh, [1]=mm, [2]=ss, [3]=ff */
    fps: u8,
}

unsafe extern "C" fn snd_mts64_free(mts: *mut mts64) -> c_int {
    unsafe {
        kfree(mts as *mut c_void);
    }
    0
}

unsafe extern "C" fn snd_mts64_create(
    card: *mut snd_card,
    pardev: *mut pardevice,
    rchip: *mut *mut mts64,
) -> c_int {
    let mts: *mut mts64;

    unsafe {
        *rchip = ptr::null_mut();

        mts = kzalloc(core::mem::size_of::<mts64>(), 0) as *mut mts64;
        if mts.is_null() {
            return -ENOMEM;
        }

        /* Init chip specific data */
        spin_lock_init(&mut (*mts).lock);
        (*mts).card = card;
        (*mts).pardev = pardev;
        (*mts).current_midi_output_port = -1;
        (*mts).current_midi_input_port = -1;

        *rchip = mts;
    }

    0
}

/*********************************************************************
 * HW register related constants
 *********************************************************************/

/* Status Bits */
const MTS64_STAT_BSY: u8 = 0x80;
const MTS64_STAT_BIT_SET: u8 = 0x20; /* readout process, bit is set */
const MTS64_STAT_PORT: u8 = 0x10; /* read byte is a port number */

/* Control Bits */
const MTS64_CTL_READOUT: u8 = 0x08; /* enable readout */
const MTS64_CTL_WRITE_CMD: u8 = 0x06;
const MTS64_CTL_WRITE_DATA: u8 = 0x02;
const MTS64_CTL_STROBE: u8 = 0x01;

/* Command */
const MTS64_CMD_RESET: u8 = 0xfe;
const MTS64_CMD_PROBE: u8 = 0x8f; /* Used in probing procedure */
const MTS64_CMD_SMPTE_SET_TIME: u8 = 0xe8;
const MTS64_CMD_SMPTE_SET_FPS: u8 = 0xee;
const MTS64_CMD_SMPTE_STOP: u8 = 0xef;
const MTS64_CMD_SMPTE_FPS_24: u8 = 0xe3;
const MTS64_CMD_SMPTE_FPS_25: u8 = 0xe2;
const MTS64_CMD_SMPTE_FPS_2997: u8 = 0xe4;
const MTS64_CMD_SMPTE_FPS_30D: u8 = 0xe1;
const MTS64_CMD_SMPTE_FPS_30: u8 = 0xe0;
const MTS64_CMD_COM_OPEN: u8 = 0xf8; /* setting the communication mode */
const MTS64_CMD_COM_CLOSE1: u8 = 0xff; /* clearing communication mode */
const MTS64_CMD_COM_CLOSE2: u8 = 0xf5;

/*********************************************************************
 * Hardware specific functions
 *********************************************************************/

/*  Enables the readout procedure
 *
 *  Before we can read a midi byte from the device, we have to set
 *  bit 3 of control port.
 */
unsafe fn mts64_enable_readout(p: *mut parport) {
    let mut c: u8;

    unsafe {
        c = parport_read_control(p);
        c |= MTS64_CTL_READOUT;
        parport_write_control(p, c);
    }
}

/*  Disables readout
 *
 *  Readout is disabled by clearing bit 3 of control
 */
unsafe fn mts64_disable_readout(p: *mut parport) {
    let mut c: u8;

    unsafe {
        c = parport_read_control(p);
        c &= !MTS64_CTL_READOUT;
        parport_write_control(p, c);
    }
}

/*  waits for device ready
 *
 *  Checks if BUSY (Bit 7 of status) is clear
 *  1 device ready
 *  0 failure
 */
unsafe fn mts64_device_ready(p: *mut parport) -> c_int {
    let mut c: u8;

    for _i in 0..0xffff {
        unsafe {
            c = parport_read_status(p);
        }
        c &= MTS64_STAT_BSY;
        if c != 0 {
            return 1;
        }
    }

    0
}

/*  Init device (LED blinking startup magic)
 *
 *  Returns:
 *  0 init ok
 *  -EIO failure
 */
unsafe fn mts64_device_init(p: *mut parport) -> c_int {
    unsafe {
        mts64_write_command(p, MTS64_CMD_RESET);

        for _i in 0..64 {
            msleep(100);

            if mts64_probe(p) == 0 {
                /* success */
                mts64_disable_readout(p);
                return 0;
            }
        }
        mts64_disable_readout(p);
    }

    -EIO
}

/*
 *  Opens the device (set communication mode)
 */
unsafe fn mts64_device_open(mts: *mut mts64) -> c_int {
    unsafe {
        let p: *mut parport = (*(*mts).pardev).port;

        for _i in 0..5 {
            mts64_write_command(p, MTS64_CMD_COM_OPEN);
        }
    }

    0
}

/*
 *  Close device (clear communication mode)
 */
unsafe fn mts64_device_close(mts: *mut mts64) -> c_int {
    unsafe {
        let p: *mut parport = (*(*mts).pardev).port;

        for _i in 0..5 {
            mts64_write_command(p, MTS64_CMD_COM_CLOSE1);
            mts64_write_command(p, MTS64_CMD_COM_CLOSE2);
        }
    }

    0
}

/*  map hardware port to substream number
 *
 *  When reading a byte from the device, the device tells us
 *  on what port the byte is. This HW port has to be mapped to
 *  the midiport (substream number).
 *  substream 0-3 are Midiports 1-4
 *  substream 4 is SMPTE Timecode
 *  The mapping is done by the table:
 *  HW | 0 | 1 | 2 | 3 | 4
 *  SW | 0 | 1 | 4 | 2 | 3
 */
unsafe fn mts64_map_midi_input(c: u8) -> u8 {
    static map: [u8; 5] = [0, 1, 4, 2, 3];

    map[c as usize]
}

/*  Probe parport for device
 *
 *  Do we have a Miditerminal 4140 on parport?
 *  Returns:
 *  0       device found
 *  -ENODEV no device
 */
unsafe fn mts64_probe(p: *mut parport) -> c_int {
    let mut c: u8;

    unsafe {
        mts64_smpte_stop(p);
        mts64_write_command(p, MTS64_CMD_PROBE);

        msleep(50);

        c = mts64_read(p) as u8;
    }

    c &= 0x00ff_u16 as u8;
    if c != MTS64_CMD_PROBE {
        -ENODEV
    } else {
        0
    }
}

/*  Read byte incl. status from device
 *
 *  Returns:
 *  data in lower 8 bits and status in upper 8 bits
 */
unsafe fn mts64_read(p: *mut parport) -> u16 {
    let data: u8;
    let status: u8;

    unsafe {
        mts64_device_ready(p);
        mts64_enable_readout(p);
        status = parport_read_status(p);
        data = mts64_read_char(p);
        mts64_disable_readout(p);
    }

    ((status as u16) << 8) | data as u16
}

/*  Read a byte from device
 *
 *  Note, that readout mode has to be enabled.
 *  readout procedure is as follows:
 *  - Write number of the Bit to read to DATA
 *  - Read STATUS
 *  - Bit 5 of STATUS indicates if Bit is set
 *
 *  Returns:
 *  Byte read from device
 */
unsafe fn mts64_read_char(p: *mut parport) -> u8 {
    let mut c: u8 = 0;
    let mut status: u8;

    for i in 0..8u8 {
        unsafe {
            parport_write_data(p, i);
        }
        c >>= 1;
        unsafe {
            status = parport_read_status(p);
        }
        if status & MTS64_STAT_BIT_SET != 0 {
            c |= 0x80;
        }
    }

    c
}

/*  Starts SMPTE Timecode generation
 *
 *  The device creates SMPTE Timecode by hardware.
 *  0 24 fps
 *  1 25 fps
 *  2 29.97 fps
 *  3 30 fps (Drop-frame)
 *  4 30 fps
 */
unsafe fn mts64_smpte_start(
    p: *mut parport,
    hours: u8,
    minutes: u8,
    seconds: u8,
    frames: u8,
    idx: u8,
) {
    static fps: [u8; 5] = [
        MTS64_CMD_SMPTE_FPS_24,
        MTS64_CMD_SMPTE_FPS_25,
        MTS64_CMD_SMPTE_FPS_2997,
        MTS64_CMD_SMPTE_FPS_30D,
        MTS64_CMD_SMPTE_FPS_30,
    ];

    unsafe {
        mts64_write_command(p, MTS64_CMD_SMPTE_SET_TIME);
        mts64_write_command(p, frames);
        mts64_write_command(p, seconds);
        mts64_write_command(p, minutes);
        mts64_write_command(p, hours);

        mts64_write_command(p, MTS64_CMD_SMPTE_SET_FPS);
        mts64_write_command(p, fps[idx as usize]);
    }
}

/*  Stops SMPTE Timecode generation
 */
unsafe fn mts64_smpte_stop(p: *mut parport) {
    unsafe {
        mts64_write_command(p, MTS64_CMD_SMPTE_STOP);
    }
}

/*  Write a command byte to device
 */
unsafe fn mts64_write_command(p: *mut parport, c: u8) {
    unsafe {
        mts64_device_ready(p);

        parport_write_data(p, c);

        parport_write_control(p, MTS64_CTL_WRITE_CMD);
        parport_write_control(p, MTS64_CTL_WRITE_CMD | MTS64_CTL_STROBE);
        parport_write_control(p, MTS64_CTL_WRITE_CMD);
    }
}

/*  Write a data byte to device
 */
unsafe fn mts64_write_data(p: *mut parport, c: u8) {
    unsafe {
        mts64_device_ready(p);

        parport_write_data(p, c);

        parport_write_control(p, MTS64_CTL_WRITE_DATA);
        parport_write_control(p, MTS64_CTL_WRITE_DATA | MTS64_CTL_STROBE);
        parport_write_control(p, MTS64_CTL_WRITE_DATA);
    }
}

/*  Write a MIDI byte to midiport
 *
 *  midiport ranges from 0-3 and maps to Ports 1-4
 *  assumptions: communication mode is on
 */
unsafe fn mts64_write_midi(mts: *mut mts64, c: u8, midiport: c_int) {
    unsafe {
        let p: *mut parport = (*(*mts).pardev).port;

        /* check current midiport */
        if (*mts).current_midi_output_port != midiport {
            mts64_write_command(p, midiport as u8);
        }

        /* write midi byte */
        mts64_write_data(p, c);
    }
}

/*********************************************************************
 * Control elements
 *********************************************************************/

/* SMPTE Switch */
const snd_mts64_ctl_smpte_switch_info:
    Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int> =
    Some(snd_ctl_boolean_mono_info);

unsafe extern "C" fn snd_mts64_ctl_smpte_switch_get(
    kctl: *mut snd_kcontrol,
    uctl: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let mts: *mut mts64 = snd_kcontrol_chip(kctl) as *mut mts64;

        spin_lock_irq(&mut (*mts).lock);
        (*uctl).value.integer.value[0] = (*mts).smpte_switch as c_long;
        spin_unlock_irq(&mut (*mts).lock);
    }

    0
}

/* smpte_switch is not accessed from IRQ handler, so we just need
   to protect the HW access */
unsafe extern "C" fn snd_mts64_ctl_smpte_switch_put(
    kctl: *mut snd_kcontrol,
    uctl: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let mts: *mut mts64 = snd_kcontrol_chip(kctl) as *mut mts64;
        let val: c_int = ((*uctl).value.integer.value[0] != 0) as c_int;

        spin_lock_irq(&mut (*mts).lock);
        if (*mts).smpte_switch == val {
            spin_unlock_irq(&mut (*mts).lock);
            return 0;
        }

        (*mts).smpte_switch = val;
        if (*mts).smpte_switch != 0 {
            mts64_smpte_start(
                (*(*mts).pardev).port,
                (*mts).time[0],
                (*mts).time[1],
                (*mts).time[2],
                (*mts).time[3],
                (*mts).fps,
            );
        } else {
            mts64_smpte_stop((*(*mts).pardev).port);
        }
        spin_unlock_irq(&mut (*mts).lock);
    }
    1
}

static mts64_ctl_smpte_switch: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_RAWMIDI,
    name: b"SMPTE Playback Switch\0".as_ptr() as *const c_char,
    index: 0,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    private_value: 0,
    info: snd_mts64_ctl_smpte_switch_info,
    get: Some(snd_mts64_ctl_smpte_switch_get),
    put: Some(snd_mts64_ctl_smpte_switch_put),
};

/* Time */
unsafe extern "C" fn snd_mts64_ctl_smpte_time_h_info(
    _kctl: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).count = 1;
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = 23;
    }
    0
}

unsafe extern "C" fn snd_mts64_ctl_smpte_time_f_info(
    _kctl: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).count = 1;
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = 99;
    }
    0
}

unsafe extern "C" fn snd_mts64_ctl_smpte_time_info(
    _kctl: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).count = 1;
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = 59;
    }
    0
}

unsafe extern "C" fn snd_mts64_ctl_smpte_time_get(
    kctl: *mut snd_kcontrol,
    uctl: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let mts: *mut mts64 = snd_kcontrol_chip(kctl) as *mut mts64;
        let idx: c_int = (*kctl).private_value as c_int;

        spin_lock_irq(&mut (*mts).lock);
        (*uctl).value.integer.value[0] = (*mts).time[idx as usize] as c_long;
        spin_unlock_irq(&mut (*mts).lock);
    }

    0
}

unsafe extern "C" fn snd_mts64_ctl_smpte_time_put(
    kctl: *mut snd_kcontrol,
    uctl: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let mts: *mut mts64 = snd_kcontrol_chip(kctl) as *mut mts64;
        let idx: c_int = (*kctl).private_value as c_int;
        let time: c_uint = ((*uctl).value.integer.value[0] as c_uint) % 60;

        spin_lock_irq(&mut (*mts).lock);
        if (*mts).time[idx as usize] != time as u8 {
            (*mts).time[idx as usize] = time as u8;
            spin_unlock_irq(&mut (*mts).lock);
            return 1;
        }
        spin_unlock_irq(&mut (*mts).lock);
    }

    0
}

static mts64_ctl_smpte_time_hours: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_RAWMIDI,
    name: b"SMPTE Time Hours\0".as_ptr() as *const c_char,
    index: 0,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    private_value: 0,
    info: Some(snd_mts64_ctl_smpte_time_h_info),
    get: Some(snd_mts64_ctl_smpte_time_get),
    put: Some(snd_mts64_ctl_smpte_time_put),
};

static mts64_ctl_smpte_time_minutes: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_RAWMIDI,
    name: b"SMPTE Time Minutes\0".as_ptr() as *const c_char,
    index: 0,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    private_value: 1,
    info: Some(snd_mts64_ctl_smpte_time_info),
    get: Some(snd_mts64_ctl_smpte_time_get),
    put: Some(snd_mts64_ctl_smpte_time_put),
};

static mts64_ctl_smpte_time_seconds: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_RAWMIDI,
    name: b"SMPTE Time Seconds\0".as_ptr() as *const c_char,
    index: 0,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    private_value: 2,
    info: Some(snd_mts64_ctl_smpte_time_info),
    get: Some(snd_mts64_ctl_smpte_time_get),
    put: Some(snd_mts64_ctl_smpte_time_put),
};

static mts64_ctl_smpte_time_frames: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_RAWMIDI,
    name: b"SMPTE Time Frames\0".as_ptr() as *const c_char,
    index: 0,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    private_value: 3,
    info: Some(snd_mts64_ctl_smpte_time_f_info),
    get: Some(snd_mts64_ctl_smpte_time_get),
    put: Some(snd_mts64_ctl_smpte_time_put),
};

/* FPS */
unsafe extern "C" fn snd_mts64_ctl_smpte_fps_info(
    _kctl: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    static texts: [*const c_char; 5] = [
        b"24\0".as_ptr() as *const c_char,
        b"25\0".as_ptr() as *const c_char,
        b"29.97\0".as_ptr() as *const c_char,
        b"30D\0".as_ptr() as *const c_char,
        b"30\0".as_ptr() as *const c_char,
    ];

    unsafe { snd_ctl_enum_info(uinfo, 1, 5, texts.as_ptr()) }
}

unsafe extern "C" fn snd_mts64_ctl_smpte_fps_get(
    kctl: *mut snd_kcontrol,
    uctl: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let mts: *mut mts64 = snd_kcontrol_chip(kctl) as *mut mts64;

        spin_lock_irq(&mut (*mts).lock);
        (*uctl).value.enumerated.item[0] = (*mts).fps as c_uint;
        spin_unlock_irq(&mut (*mts).lock);
    }

    0
}

unsafe extern "C" fn snd_mts64_ctl_smpte_fps_put(
    kctl: *mut snd_kcontrol,
    uctl: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let mts: *mut mts64 = snd_kcontrol_chip(kctl) as *mut mts64;

        if (*uctl).value.enumerated.item[0] >= 5 {
            return -EINVAL;
        }
        spin_lock_irq(&mut (*mts).lock);
        if (*mts).fps != (*uctl).value.enumerated.item[0] as u8 {
            (*mts).fps = (*uctl).value.enumerated.item[0] as u8;
            spin_unlock_irq(&mut (*mts).lock);
            return 1;
        }
        spin_unlock_irq(&mut (*mts).lock);
    }

    0
}

static mts64_ctl_smpte_fps: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_RAWMIDI,
    name: b"SMPTE Fps\0".as_ptr() as *const c_char,
    index: 0,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    private_value: 0,
    info: Some(snd_mts64_ctl_smpte_fps_info),
    get: Some(snd_mts64_ctl_smpte_fps_get),
    put: Some(snd_mts64_ctl_smpte_fps_put),
};

unsafe fn snd_mts64_ctl_create(card: *mut snd_card, mts: *mut mts64) -> c_int {
    let mut err: c_int;
    let control: [*const snd_kcontrol_new; 7] = [
        &mts64_ctl_smpte_switch,
        &mts64_ctl_smpte_time_hours,
        &mts64_ctl_smpte_time_minutes,
        &mts64_ctl_smpte_time_seconds,
        &mts64_ctl_smpte_time_frames,
        &mts64_ctl_smpte_fps,
        ptr::null(),
    ];

    let mut i = 0usize;
    while !control[i].is_null() {
        unsafe {
            err = snd_ctl_add(card, snd_ctl_new1(control[i], mts as *mut c_void));
            if err < 0 {
                dev_dbg((*card).dev, b"Cannot create control: %s\n\0".as_ptr(), (*control[i]).name);
                return err;
            }
        }
        i += 1;
    }

    0
}

/*********************************************************************
 * Rawmidi
 *********************************************************************/
const MTS64_MODE_INPUT_TRIGGERED: u8 = 0x01;

unsafe extern "C" fn snd_mts64_rawmidi_open(substream: *mut snd_rawmidi_substream) -> c_int {
    unsafe {
        let mts: *mut mts64 = (*(*substream).rmidi).private_data as *mut mts64;

        if (*mts).open_count == 0 {
            /* We don't need a spinlock here, because this is just called
               if the device has not been opened before.
               So there aren't any IRQs from the device */
            mts64_device_open(mts);

            msleep(50);
        }
        (*mts).open_count += 1;
    }

    0
}

unsafe extern "C" fn snd_mts64_rawmidi_close(substream: *mut snd_rawmidi_substream) -> c_int {
    unsafe {
        let mts: *mut mts64 = (*(*substream).rmidi).private_data as *mut mts64;

        (*mts).open_count -= 1;
        if (*mts).open_count == 0 {
            /* We need the spinlock_irqsave here because we can still
               have IRQs at this point */
            let mut flags: c_ulong = 0;
            spin_lock_irqsave(&mut (*mts).lock, &mut flags);
            mts64_device_close(mts);
            spin_unlock_irqrestore(&mut (*mts).lock, flags);

            msleep(500);
        } else if (*mts).open_count < 0 {
            (*mts).open_count = 0;
        }
    }

    0
}

unsafe extern "C" fn snd_mts64_rawmidi_output_trigger(
    substream: *mut snd_rawmidi_substream,
    _up: c_int,
) {
    unsafe {
        let mts: *mut mts64 = (*(*substream).rmidi).private_data as *mut mts64;
        let mut data: u8 = 0;
        let mut flags: c_ulong = 0;

        spin_lock_irqsave(&mut (*mts).lock, &mut flags);
        while snd_rawmidi_transmit_peek(substream, &mut data, 1) == 1 {
            mts64_write_midi(mts, data, (*substream).number + 1);
            snd_rawmidi_transmit_ack(substream, 1);
        }
        spin_unlock_irqrestore(&mut (*mts).lock, flags);
    }
}

unsafe extern "C" fn snd_mts64_rawmidi_input_trigger(
    substream: *mut snd_rawmidi_substream,
    up: c_int,
) {
    unsafe {
        let mts: *mut mts64 = (*(*substream).rmidi).private_data as *mut mts64;

        let mut flags: c_ulong = 0;
        spin_lock_irqsave(&mut (*mts).lock, &mut flags);
        if up != 0 {
            (*mts).mode[(*substream).number as usize] |= MTS64_MODE_INPUT_TRIGGERED;
        } else {
            (*mts).mode[(*substream).number as usize] &= !MTS64_MODE_INPUT_TRIGGERED;
        }
        spin_unlock_irqrestore(&mut (*mts).lock, flags);
    }
}

static snd_mts64_rawmidi_output_ops: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_mts64_rawmidi_open),
    close: Some(snd_mts64_rawmidi_close),
    trigger: Some(snd_mts64_rawmidi_output_trigger),
};

static snd_mts64_rawmidi_input_ops: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_mts64_rawmidi_open),
    close: Some(snd_mts64_rawmidi_close),
    trigger: Some(snd_mts64_rawmidi_input_trigger),
};

unsafe fn list_entry_snd_rawmidi_substream(ptr: *mut list_head) -> *mut snd_rawmidi_substream {
    (ptr as *mut u8).offset(-(core::mem::offset_of!(snd_rawmidi_substream, list) as isize))
        as *mut snd_rawmidi_substream
}

/* Create and initialize the rawmidi component */
unsafe fn snd_mts64_rawmidi_create(card: *mut snd_card) -> c_int {
    unsafe {
        let mts: *mut mts64 = (*card).private_data as *mut mts64;
        let mut rmidi: *mut snd_rawmidi = ptr::null_mut();
        let mut substream: *mut snd_rawmidi_substream;
        let mut list: *mut list_head;
        let mut err: c_int;

        err = snd_rawmidi_new(
            card,
            CARD_NAME.as_ptr(),
            0,
            MTS64_NUM_OUTPUT_PORTS,
            MTS64_NUM_INPUT_PORTS as c_int,
            &mut rmidi,
        );
        if err < 0 {
            return err;
        }

        (*rmidi).private_data = mts as *mut c_void;
        strscpy((*rmidi).name.as_mut_ptr(), CARD_NAME.as_ptr());
        (*rmidi).info_flags =
            SNDRV_RAWMIDI_INFO_OUTPUT | SNDRV_RAWMIDI_INFO_INPUT | SNDRV_RAWMIDI_INFO_DUPLEX;

        (*mts).rmidi = rmidi;

        /* register rawmidi ops */
        snd_rawmidi_set_ops(
            rmidi,
            SNDRV_RAWMIDI_STREAM_OUTPUT as c_int,
            &snd_mts64_rawmidi_output_ops,
        );
        snd_rawmidi_set_ops(
            rmidi,
            SNDRV_RAWMIDI_STREAM_INPUT as c_int,
            &snd_mts64_rawmidi_input_ops,
        );

        /* name substreams */
        /* output */
        list = (*rmidi).streams[SNDRV_RAWMIDI_STREAM_OUTPUT].substreams.next;
        while list != &mut (*rmidi).streams[SNDRV_RAWMIDI_STREAM_OUTPUT].substreams {
            substream = list_entry_snd_rawmidi_substream(list);
            sprintf(
                (*substream).name.as_mut_ptr(),
                b"Miditerminal %d\0".as_ptr(),
                (*substream).number + 1,
            );
            list = (*list).next;
        }
        /* input */
        list = (*rmidi).streams[SNDRV_RAWMIDI_STREAM_INPUT].substreams.next;
        while list != &mut (*rmidi).streams[SNDRV_RAWMIDI_STREAM_INPUT].substreams {
            substream = list_entry_snd_rawmidi_substream(list);
            (*mts).midi_input_substream[(*substream).number as usize] = substream;
            match (*substream).number {
                MTS64_SMPTE_SUBSTREAM => {
                    strscpy((*substream).name.as_mut_ptr(), b"Miditerminal SMPTE\0".as_ptr());
                }
                _ => {
                    sprintf(
                        (*substream).name.as_mut_ptr(),
                        b"Miditerminal %d\0".as_ptr(),
                        (*substream).number + 1,
                    );
                }
            }
            list = (*list).next;
        }

        /* controls */
        err = snd_mts64_ctl_create(card, mts);

        err
    }
}

/*********************************************************************
 * parport stuff
 *********************************************************************/
unsafe extern "C" fn snd_mts64_interrupt(private: *mut c_void) {
    unsafe {
        let mts: *mut mts64 = (*(private as *mut snd_card)).private_data as *mut mts64;
        let ret: u16;
        let status: u8;
        let data: u8;
        let substream: *mut snd_rawmidi_substream;

        if mts.is_null() {
            return;
        }

        spin_lock(&mut (*mts).lock);
        ret = mts64_read((*(*mts).pardev).port);
        data = (ret & 0x00ff) as u8;
        status = (ret >> 8) as u8;

        if status & MTS64_STAT_PORT != 0 {
            (*mts).current_midi_input_port = mts64_map_midi_input(data) as c_int;
        } else {
            if (*mts).current_midi_input_port == -1 {
                spin_unlock(&mut (*mts).lock);
                return;
            }
            substream = (*mts).midi_input_substream[(*mts).current_midi_input_port as usize];
            if (*mts).mode[(*substream).number as usize] & MTS64_MODE_INPUT_TRIGGERED != 0 {
                snd_rawmidi_receive(substream, &data as *const u8 as *mut u8, 1);
            }
        }
        spin_unlock(&mut (*mts).lock);
    }
}

unsafe extern "C" fn snd_mts64_attach(p: *mut parport) {
    unsafe {
        let device: *mut platform_device;

        device = platform_device_alloc(PLATFORM_DRIVER.as_ptr(), device_count);
        if device.is_null() {
            return;
        }

        /* Temporary assignment to forward the parport */
        platform_set_drvdata(device, p as *mut c_void);

        if platform_device_add(device) < 0 {
            platform_device_put(device);
            return;
        }

        /* Since we dont get the return value of probe
         * We need to check if device probing succeeded or not */
        if platform_get_drvdata(device).is_null() {
            platform_device_unregister(device);
            return;
        }

        /* register device in global table */
        platform_devices[device_count as usize] = device;
        device_count += 1;
    }
}

unsafe extern "C" fn snd_mts64_detach(_p: *mut parport) {
    /* nothing to do here */
}

unsafe extern "C" fn snd_mts64_dev_probe(pardev: *mut pardevice) -> c_int {
    unsafe {
        if strcmp((*pardev).name, DRIVER_NAME.as_ptr()) != 0 {
            return -ENODEV;
        }
    }

    0
}

static mut mts64_parport_driver: parport_driver = parport_driver {
    name: b"mts64\0".as_ptr() as *const c_char,
    probe: Some(snd_mts64_dev_probe),
    match_port: Some(snd_mts64_attach),
    detach: Some(snd_mts64_detach),
};

/*********************************************************************
 * platform stuff
 *********************************************************************/
unsafe extern "C" fn snd_mts64_card_private_free(card: *mut snd_card) {
    unsafe {
        let mts: *mut mts64 = (*card).private_data as *mut mts64;
        let pardev: *mut pardevice = (*mts).pardev;

        if !pardev.is_null() {
            parport_release(pardev);
            parport_unregister_device(pardev);
        }

        snd_mts64_free(mts);
    }
}

unsafe extern "C" fn snd_mts64_probe_platform(pdev: *mut platform_device) -> c_int {
    unsafe {
        let pardev: *mut pardevice;
        let p: *mut parport;
        let mut dev: c_int = (*pdev).id;
        let mut card: *mut snd_card = ptr::null_mut();
        let mut mts: *mut mts64 = ptr::null_mut();
        let mut err: c_int;
        let mut mts64_cb: pardev_cb = pardev_cb {
            preempt: None,
            wakeup: None,
            irq_func: Some(snd_mts64_interrupt), /* ISR */
            flags: PARPORT_DEV_EXCL,             /* flags */
            private: ptr::null_mut(),
        };

        p = platform_get_drvdata(pdev) as *mut parport;
        platform_set_drvdata(pdev, ptr::null_mut());

        if dev < 0 {
            dev_warn(
                &mut (*pdev).dev,
                b"Invalid card index %d, using default 0\n\0".as_ptr(),
                dev,
            );
            dev = 0;
        }

        if dev >= SNDRV_CARDS as c_int {
            return -ENODEV;
        }
        if !enable[dev as usize] {
            return -ENOENT;
        }

        err = snd_card_new(
            &mut (*pdev).dev,
            index[dev as usize],
            id[dev as usize],
            ptr::null_mut(),
            0,
            &mut card,
        );
        if err < 0 {
            dev_dbg(&mut (*pdev).dev, b"Cannot create card\n\0".as_ptr());
            return err;
        }
        strscpy((*card).driver.as_mut_ptr(), DRIVER_NAME.as_ptr());
        strscpy((*card).shortname.as_mut_ptr(), b"ESI Miditerminal 4140\0".as_ptr());
        sprintf(
            (*card).longname.as_mut_ptr(),
            b"%s at 0x%lx, irq %i\0".as_ptr(),
            (*card).shortname.as_mut_ptr(),
            (*p).base,
            (*p).irq,
        );

        mts64_cb.private = card as *mut c_void; /* private */
        pardev = parport_register_dev_model(
            p,                    /* port */
            DRIVER_NAME.as_ptr(), /* name */
            &mut mts64_cb,        /* callbacks */
            (*pdev).id,           /* device number */
        );
        if pardev.is_null() {
            dev_dbg((*card).dev, b"Cannot register pardevice\n\0".as_ptr());
            err = -EIO;
            goto_err(card, err);
            return err;
        }

        /* claim parport */
        if parport_claim(pardev) != 0 {
            dev_dbg(
                (*card).dev,
                b"Cannot claim parport 0x%lx\n\0".as_ptr(),
                (*(*pardev).port).base,
            );
            err = -EIO;
            parport_unregister_device(pardev);
            snd_card_free(card);
            return err;
        }

        err = snd_mts64_create(card, pardev, &mut mts);
        if err < 0 {
            dev_dbg((*card).dev, b"Cannot create main component\n\0".as_ptr());
            parport_release(pardev);
            parport_unregister_device(pardev);
            snd_card_free(card);
            return err;
        }
        (*card).private_data = mts as *mut c_void;
        (*card).private_free = Some(snd_mts64_card_private_free);

        err = mts64_probe(p);
        if err != 0 {
            err = -EIO;
            snd_card_free(card);
            return err;
        }

        err = snd_mts64_rawmidi_create(card);
        if err < 0 {
            dev_dbg((*card).dev, b"Creating Rawmidi component failed\n\0".as_ptr());
            snd_card_free(card);
            return err;
        }

        /* init device */
        err = mts64_device_init(p);
        if err < 0 {
            snd_card_free(card);
            return err;
        }

        platform_set_drvdata(pdev, card as *mut c_void);

        /* At this point card will be usable */
        err = snd_card_register(card);
        if err < 0 {
            dev_dbg((*card).dev, b"Cannot register card\n\0".as_ptr());
            snd_card_free(card);
            return err;
        }

        dev_info((*card).dev, b"ESI Miditerminal 4140 on 0x%lx\n\0".as_ptr(), (*p).base);
        0
    }
}

unsafe fn goto_err(card: *mut snd_card, _err: c_int) {
    unsafe {
        snd_card_free(card);
    }
}

unsafe extern "C" fn snd_mts64_remove(pdev: *mut platform_device) {
    unsafe {
        let card: *mut snd_card = platform_get_drvdata(pdev) as *mut snd_card;

        if !card.is_null() {
            snd_card_free(card);
        }
    }
}

static mut snd_mts64_driver: platform_driver = platform_driver {
    probe: Some(snd_mts64_probe_platform),
    remove: Some(snd_mts64_remove),
    driver: platform_driver_inner {
        name: PLATFORM_DRIVER.as_ptr() as *const c_char,
    },
};

/*********************************************************************
 * module init stuff
 *********************************************************************/
unsafe fn snd_mts64_unregister_all() {
    unsafe {
        for i in 0..SNDRV_CARDS {
            if !platform_devices[i].is_null() {
                platform_device_unregister(platform_devices[i]);
                platform_devices[i] = ptr::null_mut();
            }
        }
        platform_driver_unregister(&mut snd_mts64_driver);
        parport_unregister_driver(&mut mts64_parport_driver);
    }
}

unsafe extern "C" fn snd_mts64_module_init() -> c_int {
    let err: c_int;

    unsafe {
        err = platform_driver_register(&mut snd_mts64_driver);
        if err < 0 {
            return err;
        }

        if parport_register_driver(&mut mts64_parport_driver) != 0 {
            platform_driver_unregister(&mut snd_mts64_driver);
            return -EIO;
        }

        if device_count == 0 {
            snd_mts64_unregister_all();
            return -ENODEV;
        }
    }

    0
}

unsafe extern "C" fn snd_mts64_module_exit() {
    unsafe {
        snd_mts64_unregister_all();
    }
}

// module_init(snd_mts64_module_init);
// module_exit(snd_mts64_module_exit);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
