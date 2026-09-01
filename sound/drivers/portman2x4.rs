// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Driver for Midiman Portman2x4 parallel port midi interface
 *
 *   Copyright (c) by Levent Guendogdu <levon@feature-it.com>
 *
 * ChangeLog
 * Jan 24 2007 Matthias Koenig <mkoenig@suse.de>
 *      - cleanup and rewrite
 * Sep 30 2004 Tobias Gehrig <tobias@gehrig.tk>
 *      - source code cleanup
 * Sep 03 2004 Tobias Gehrig <tobias@gehrig.tk>
 *      - fixed compilation problem with alsa 1.0.6a (removed MODULE_CLASSES,
 *        MODULE_PARM_SYNTAX and changed MODULE_DEVICES to
 *        MODULE_SUPPORTED_DEVICE)
 * Mar 24 2004 Tobias Gehrig <tobias@gehrig.tk>
 *      - added 2.6 kernel support
 * Mar 18 2004 Tobias Gehrig <tobias@gehrig.tk>
 *      - added parport_unregister_driver to the startup routine if the driver fails to detect a portman
 *      - added support for all 4 output ports in portman_putmidi
 * Mar 17 2004 Tobias Gehrig <tobias@gehrig.tk>
 *      - added checks for opened input device in interrupt handler
 * Feb 20 2004 Tobias Gehrig <tobias@gehrig.tk>
 *      - ported from alsa 0.5 to 1.0
 */

/* Dependencies originally included from Linux, parport, and ALSA headers:
 * linux/init.h, linux/platform_device.h, linux/parport.h, linux/spinlock.h,
 * linux/delay.h, linux/slab.h, linux/module.h, sound/core.h,
 * sound/initval.h, sound/rawmidi.h, sound/control.h.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

const CARD_NAME: &[u8] = b"Portman 2x4\0";
const DRIVER_NAME: &[u8] = b"portman\0";
const PLATFORM_DRIVER: &[u8] = b"snd_portman2x4\0";

const PORTMAN_NUM_INPUT_PORTS: usize = 2;
const PORTMAN_NUM_OUTPUT_PORTS: usize = 4;

/* Standard PC parallel port status register equates. */
const PP_STAT_BSY: u8 = 0x80; /* Busy status.  Inverted. */
const PP_STAT_ACK: u8 = 0x40; /* Acknowledge.  Non-Inverted. */
const PP_STAT_POUT: u8 = 0x20; /* Paper Out.    Non-Inverted. */
const PP_STAT_SEL: u8 = 0x10; /* Select.       Non-Inverted. */
const PP_STAT_ERR: u8 = 0x08; /* Error.        Non-Inverted. */

/* Standard PC parallel port command register equates. */
const PP_CMD_IEN: u8 = 0x10; /* IRQ Enable.   Non-Inverted. */
const PP_CMD_SELI: u8 = 0x08; /* Select Input. Inverted. */
const PP_CMD_INIT: u8 = 0x04; /* Init Printer. Non-Inverted. */
const PP_CMD_FEED: u8 = 0x02; /* Auto Feed.    Inverted. */
const PP_CMD_STB: u8 = 0x01; /* Strobe.       Inverted. */

/* Parallel Port Command Register as implemented by PCP2x4. */
const INT_EN: u8 = PP_CMD_IEN; /* Interrupt enable. */
const STROBE: u8 = PP_CMD_STB; /* Command strobe. */

/* The parallel port command register field (b1..b3) selects the
 * various "registers" within the PC/P 2x4.  These are the internal
 * address of these "registers" that must be written to the parallel
 * port command register.
 */
const RXDATA0: u8 = 0 << 1; /* PCP RxData channel 0. */
const RXDATA1: u8 = 1 << 1; /* PCP RxData channel 1. */
const GEN_CTL: u8 = 2 << 1; /* PCP General Control Register. */
const SYNC_CTL: u8 = 3 << 1; /* PCP Sync Control Register. */
const TXDATA0: u8 = 4 << 1; /* PCP TxData channel 0. */
const TXDATA1: u8 = 5 << 1; /* PCP TxData channel 1. */
const TXDATA2: u8 = 6 << 1; /* PCP TxData channel 2. */
const TXDATA3: u8 = 7 << 1; /* PCP TxData channel 3. */

/* Parallel Port Status Register as implemented by PCP2x4. */
const ESTB: u8 = PP_STAT_POUT; /* Echoed strobe. */
const INT_REQ: u8 = PP_STAT_ACK; /* Input data int request. */
const BUSY: u8 = PP_STAT_ERR; /* Interface Busy. */

/* Parallel Port Status Register BUSY and SELECT lines are multiplexed
 * between several functions.  Depending on which 2x4 "register" is
 * currently selected (b1..b3), the BUSY and SELECT lines are
 * assigned as follows:
 *
 *   SELECT LINE:                                                    A3 A2 A1
 *                                                                   --------
 */
const RXAVAIL: u8 = PP_STAT_SEL; /* Rx Available, channel 0.   0 0 0 */
//  RXAVAIL1    PP_STAT_SEL             /* Rx Available, channel 1.   0 0 1 */
const SYNC_STAT: u8 = PP_STAT_SEL; /* Reserved - Sync Status.    0 1 0 */
//                                      /* Reserved.                  0 1 1 */
const TXEMPTY: u8 = PP_STAT_SEL; /* Tx Empty, channel 0.       1 0 0 */
//      TXEMPTY1        PP_STAT_SEL     /* Tx Empty, channel 1.       1 0 1 */
//  TXEMPTY2    PP_STAT_SEL             /* Tx Empty, channel 2.       1 1 0 */
//  TXEMPTY3    PP_STAT_SEL             /* Tx Empty, channel 3.       1 1 1 */

/*   BUSY LINE:                                                      A3 A2 A1
 *                                                                   --------
 */
const RXDATA: u8 = PP_STAT_BSY; /* Rx Input Data, channel 0.  0 0 0 */
//      RXDATA1         PP_STAT_BSY     /* Rx Input Data, channel 1.  0 0 1 */
const SYNC_DATA: u8 = PP_STAT_BSY; /* Reserved - Sync Data.      0 1 0 */
                                     /* Reserved.                  0 1 1 */
const DATA_ECHO: u8 = PP_STAT_BSY; /* Parallel Port Data Echo.   1 0 0 */
const A0_ECHO: u8 = PP_STAT_BSY; /* Address 0 Echo.            1 0 1 */
const A1_ECHO: u8 = PP_STAT_BSY; /* Address 1 Echo.            1 1 0 */
const A2_ECHO: u8 = PP_STAT_BSY; /* Address 2 Echo.            1 1 1 */

const PORTMAN2X4_MODE_INPUT_TRIGGERED: c_int = 0x01;

const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const EIO: c_int = 5;
const ENOENT: c_int = 2;

extern "C" {
    static SNDRV_CARDS: c_int;
    static SNDRV_DEFAULT_IDX: [c_int; 0];
    static SNDRV_DEFAULT_STR: [*mut c_char; 0];
    static SNDRV_DEFAULT_ENABLE_PNP: [bool; 0];
    static THIS_MODULE: *mut c_void;

    static SNDRV_RAWMIDI_INFO_OUTPUT: c_uint;
    static SNDRV_RAWMIDI_INFO_INPUT: c_uint;
    static SNDRV_RAWMIDI_INFO_DUPLEX: c_uint;
    static SNDRV_RAWMIDI_STREAM_OUTPUT: c_int;
    static SNDRV_RAWMIDI_STREAM_INPUT: c_int;
    static PARPORT_DEV_EXCL: c_uint;

    fn kzalloc_obj_portman() -> *mut portman;
    fn kfree(p: *mut c_void);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t) -> c_ulong;
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn cpu_relax();

    fn parport_write_control(port: *mut parport, value: u8);
    fn parport_read_status(port: *mut parport) -> u8;
    fn parport_write_data(port: *mut parport, value: u8);
    fn parport_register_driver(driver: *mut parport_driver) -> c_int;
    fn parport_unregister_driver(driver: *mut parport_driver);
    fn parport_register_dev_model(
        port: *mut parport,
        name: *const c_char,
        callbacks: *mut pardev_cb,
        devnum: c_int,
    ) -> *mut pardevice;
    fn parport_unregister_device(pardev: *mut pardevice);
    fn parport_claim(pardev: *mut pardevice) -> c_int;
    fn parport_release(pardev: *mut pardevice);

    fn platform_device_alloc(name: *const c_char, id: c_int) -> *mut platform_device;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn platform_device_add(pdev: *mut platform_device) -> c_int;
    fn platform_device_put(pdev: *mut platform_device);
    fn platform_device_unregister(pdev: *mut platform_device);
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);

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
    fn snd_rawmidi_new(
        card: *mut snd_card,
        id: *const u8,
        device: c_int,
        output_count: c_int,
        input_count: c_int,
        rmidi: *mut *mut snd_rawmidi,
    ) -> c_int;
    fn snd_rawmidi_set_ops(rmidi: *mut snd_rawmidi, stream: c_int, ops: *const snd_rawmidi_ops);
    fn snd_rawmidi_transmit(
        substream: *mut snd_rawmidi_substream,
        buffer: *mut u8,
        count: c_int,
    ) -> c_int;
    fn snd_rawmidi_receive(
        substream: *mut snd_rawmidi_substream,
        buffer: *mut u8,
        count: c_int,
    ) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const u8) -> isize;
    fn sprintf(dst: *mut c_char, fmt: *const u8, ...) -> c_int;
    fn strcmp(a: *const c_char, b: *const u8) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const u8, ...);
    fn dev_dbg(dev: *mut device, fmt: *const u8, ...);
    fn dev_info(dev: *mut device, fmt: *const u8, ...);
}

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
    base: c_ulong,
    irq: c_int,
}

#[repr(C)]
pub struct pardevice {
    name: *const c_char,
    port: *mut parport,
}

#[repr(C)]
pub struct platform_device {
    dev: device,
    id: c_int,
}

#[repr(C)]
pub struct driver_private {
    name: *const u8,
}

#[repr(C)]
pub struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    driver: driver_private,
}

#[repr(C)]
pub struct parport_driver {
    name: *const u8,
    probe: Option<unsafe extern "C" fn(*mut pardevice) -> c_int>,
    match_port: Option<unsafe extern "C" fn(*mut parport)>,
    detach: Option<unsafe extern "C" fn(*mut parport)>,
}

#[repr(C)]
pub struct pardev_cb {
    preempt: *mut c_void,
    wakeup: *mut c_void,
    irq_func: Option<unsafe extern "C" fn(*mut c_void)>,
    flags: c_uint,
    private: *mut c_void,
}

#[repr(C)]
pub struct snd_card {
    dev: *mut device,
    private_data: *mut c_void,
    private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    driver: [c_char; 16],
    shortname: [c_char; 32],
    longname: [c_char; 80],
}

#[repr(C)]
pub struct snd_rawmidi {
    private_data: *mut c_void,
    name: [c_char; 80],
    info_flags: c_uint,
    streams: [snd_rawmidi_stream; 2],
}

#[repr(C)]
pub struct snd_rawmidi_stream {
    substreams: list_head,
}

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
pub struct snd_rawmidi_substream {
    rmidi: *mut snd_rawmidi,
    number: c_int,
    name: [c_char; 32],
    list: list_head,
}

#[repr(C)]
pub struct snd_rawmidi_ops {
    open: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream, c_int)>,
}

#[repr(C)]
struct portman {
    reg_lock: spinlock_t,
    card: *mut snd_card,
    rmidi: *mut snd_rawmidi,
    pardev: *mut pardevice,
    open_count: c_int,
    mode: [c_int; PORTMAN_NUM_INPUT_PORTS],
    midi_input: [*mut snd_rawmidi_substream; PORTMAN_NUM_INPUT_PORTS],
}

static mut index: [c_int; 0] = [];
static mut id: [*mut c_char; 0] = [];
static mut enable: [bool; 0] = [];

static mut platform_devices: [*mut platform_device; 0] = [];
static mut device_count: c_int = 0;

/* module_param_array(index, int, NULL, 0444);
 * MODULE_PARM_DESC(index, "Index value for " CARD_NAME " soundcard.");
 * module_param_array(id, charp, NULL, 0444);
 * MODULE_PARM_DESC(id, "ID string for " CARD_NAME " soundcard.");
 * module_param_array(enable, bool, NULL, 0444);
 * MODULE_PARM_DESC(enable, "Enable " CARD_NAME " soundcard.");
 *
 * MODULE_AUTHOR("Levent Guendogdu, Tobias Gehrig, Matthias Koenig");
 * MODULE_DESCRIPTION("Midiman Portman2x4");
 * MODULE_LICENSE("GPL");
 */

unsafe extern "C" fn portman_free(pm: *mut portman) -> c_int {
    kfree(pm as *mut c_void);
    0
}

unsafe extern "C" fn portman_create(
    card: *mut snd_card,
    pardev: *mut pardevice,
    rchip: *mut *mut portman,
) -> c_int {
    let pm: *mut portman;

    *rchip = ptr::null_mut();

    pm = kzalloc_obj_portman();
    if pm.is_null() {
        return -ENOMEM;
    }

    /* Init chip specific data */
    spin_lock_init(&mut (*pm).reg_lock);
    (*pm).card = card;
    (*pm).pardev = pardev;

    *rchip = pm;

    0
}

/*********************************************************************
 * Hardware specific functions
 *********************************************************************/
unsafe fn portman_write_command(pm: *mut portman, value: u8) {
    parport_write_control((*(*pm).pardev).port, value);
}

unsafe fn portman_read_status(pm: *mut portman) -> u8 {
    parport_read_status((*(*pm).pardev).port)
}

unsafe fn portman_write_data(pm: *mut portman, value: u8) {
    parport_write_data((*(*pm).pardev).port, value);
}

unsafe extern "C" fn portman_write_midi(pm: *mut portman, port: c_int, mididata: u8) {
    let mut command: c_int = (port + 4) << 1;

    /* Get entering data byte and port number in BL and BH respectively.
     * Set up Tx Channel address field for use with PP Cmd Register.
     * Store address field in BH register.
     * Inputs:      AH = Output port number (0..3).
     *              AL = Data byte.
     *    command = TXDATA0 | INT_EN;
     * Align port num with address field (b1...b3),
     * set address for TXDatax, Strobe=0
     */
    command |= INT_EN as c_int;

    /* Disable interrupts so that the process is not interrupted, then
     * write the address associated with the current Tx channel to the
     * PP Command Reg.  Do not set the Strobe signal yet.
     */
    loop {
        portman_write_command(pm, command as u8);

        /* While the address lines settle, write parallel output data to
         * PP Data Reg.  This has no effect until Strobe signal is asserted.
         */
        portman_write_data(pm, mididata);

        /* If PCP channel's TxEmpty is set (TxEmpty is read through the PP
         * Status Register), then go write data.  Else go back and wait.
         */
        if (portman_read_status(pm) & TXEMPTY) == TXEMPTY {
            break;
        }
    }

    /* TxEmpty is set.  Maintain PC/P destination address and assert
     * Strobe through the PP Command Reg.  This will Strobe data into
     * the PC/P transmitter and set the PC/P BUSY signal.
     */
    portman_write_command(pm, (command | STROBE as c_int) as u8);

    /* Wait for strobe line to settle and echo back through hardware.
     * Once it has echoed back, assume that the address and data lines
     * have settled!
     */
    while (portman_read_status(pm) & ESTB) == 0 {
        cpu_relax();
    }

    /* Release strobe and immediately re-allow interrupts. */
    portman_write_command(pm, command as u8);

    while (portman_read_status(pm) & ESTB) == ESTB {
        cpu_relax();
    }

    /* PC/P BUSY is now set.  We must wait until BUSY resets itself.
     * We'll reenable ints while we're waiting.
     */
    while (portman_read_status(pm) & BUSY) == BUSY {
        cpu_relax();
    }

    /* Data sent. */
}

/*
 *  Read MIDI byte from port
 *  Attempt to read input byte from specified hardware input port (0..).
 *  Return -1 if no data
 */
unsafe extern "C" fn portman_read_midi(pm: *mut portman, port: c_int) -> c_int {
    let mut midi_data: u8 = 0;
    let cmdout: u8; /* Saved address+IE bit. */

    /* Make sure clocking edge is down before starting... */
    portman_write_data(pm, 0); /* Make sure edge is down. */

    /* Set destination address to PCP. */
    cmdout = ((port << 1) | INT_EN as c_int) as u8; /* Address + IE + No Strobe. */
    portman_write_command(pm, cmdout);

    while (portman_read_status(pm) & ESTB) == ESTB {
        cpu_relax(); /* Wait for strobe echo. */
    }

    /* After the address lines settle, check multiplexed RxAvail signal.
     * If data is available, read it.
     */
    if (portman_read_status(pm) & RXAVAIL) == 0 {
        return -1; /* No data. */
    }

    /* Set the Strobe signal to enable the Rx clocking circuitry. */
    portman_write_command(pm, cmdout | STROBE); /* Write address+IE+Strobe. */

    while (portman_read_status(pm) & ESTB) == 0 {
        cpu_relax(); /* Wait for strobe echo. */
    }

    /* The first data bit (msb) is already sitting on the input line. */
    midi_data = portman_read_status(pm) & 128;
    portman_write_data(pm, 1); /* Cause rising edge, which shifts data. */

    /* Data bit 6. */
    portman_write_data(pm, 0); /* Cause falling edge while data settles. */
    midi_data |= (portman_read_status(pm) >> 1) & 64;
    portman_write_data(pm, 1); /* Cause rising edge, which shifts data. */

    /* Data bit 5. */
    portman_write_data(pm, 0); /* Cause falling edge while data settles. */
    midi_data |= (portman_read_status(pm) >> 2) & 32;
    portman_write_data(pm, 1); /* Cause rising edge, which shifts data. */

    /* Data bit 4. */
    portman_write_data(pm, 0); /* Cause falling edge while data settles. */
    midi_data |= (portman_read_status(pm) >> 3) & 16;
    portman_write_data(pm, 1); /* Cause rising edge, which shifts data. */

    /* Data bit 3. */
    portman_write_data(pm, 0); /* Cause falling edge while data settles. */
    midi_data |= (portman_read_status(pm) >> 4) & 8;
    portman_write_data(pm, 1); /* Cause rising edge, which shifts data. */

    /* Data bit 2. */
    portman_write_data(pm, 0); /* Cause falling edge while data settles. */
    midi_data |= (portman_read_status(pm) >> 5) & 4;
    portman_write_data(pm, 1); /* Cause rising edge, which shifts data. */

    /* Data bit 1. */
    portman_write_data(pm, 0); /* Cause falling edge while data settles. */
    midi_data |= (portman_read_status(pm) >> 6) & 2;
    portman_write_data(pm, 1); /* Cause rising edge, which shifts data. */

    /* Data bit 0. */
    portman_write_data(pm, 0); /* Cause falling edge while data settles. */
    midi_data |= (portman_read_status(pm) >> 7) & 1;
    portman_write_data(pm, 1); /* Cause rising edge, which shifts data. */
    portman_write_data(pm, 0); /* Return data clock low. */

    /* De-assert Strobe and return data. */
    portman_write_command(pm, cmdout); /* Output saved address+IE. */

    /* Wait for strobe echo. */
    while (portman_read_status(pm) & ESTB) == ESTB {
        cpu_relax();
    }

    (midi_data & 255) as c_int /* Shift back and return value. */
}

/*
 *  Checks if any input data on the given channel is available
 *  Checks RxAvail
 */
unsafe extern "C" fn portman_data_avail(pm: *mut portman, channel: c_int) -> c_int {
    let mut command: c_int = INT_EN as c_int;
    match channel {
        0 => {
            command |= RXDATA0 as c_int;
        }
        1 => {
            command |= RXDATA1 as c_int;
        }
        _ => {}
    }
    /* Write hardware (assumme STROBE=0) */
    portman_write_command(pm, command as u8);
    /* Check multiplexed RxAvail signal */
    if (portman_read_status(pm) & RXAVAIL) == RXAVAIL {
        return 1; /* Data available */
    }

    /* No Data available */
    0
}

/*
 *  Flushes any input
 */
unsafe extern "C" fn portman_flush_input(pm: *mut portman, port: u8) {
    /* Local variable for counting things */
    let mut i: c_uint = 0;
    let mut command: u8 = 0;

    match port {
        0 => {
            command = RXDATA0;
        }
        1 => {
            command = RXDATA1;
        }
        _ => {
            dev_warn(
                (*(*pm).card).dev,
                b"%s Won't flush port %i\n\0".as_ptr(),
                b"portman_flush_input\0".as_ptr(),
                port as c_int,
            );
            return;
        }
    }

    /* Set address for specified channel in port and allow to settle. */
    portman_write_command(pm, command);

    /* Assert the Strobe and wait for echo back. */
    portman_write_command(pm, command | STROBE);

    /* Wait for ESTB */
    while (portman_read_status(pm) & ESTB) == 0 {
        cpu_relax();
    }

    /* Output clock cycles to the Rx circuitry. */
    portman_write_data(pm, 0);

    /* Flush 250 bits... */
    while i < 250 {
        portman_write_data(pm, 1);
        portman_write_data(pm, 0);
        i += 1;
    }

    /* Deassert the Strobe signal of the port and wait for it to settle. */
    portman_write_command(pm, command | INT_EN);

    /* Wait for settling */
    while (portman_read_status(pm) & ESTB) == ESTB {
        cpu_relax();
    }
}

unsafe extern "C" fn portman_probe(p: *mut parport) -> c_int {
    /* Initialize the parallel port data register.  Will set Rx clocks
     * low in case we happen to be addressing the Rx ports at this time.
     */
    /* 1 */
    parport_write_data(p, 0);

    /* Initialize the parallel port command register, thus initializing
     * hardware handshake lines to midi box:
     *
     *                                  Strobe = 0
     *                                  Interrupt Enable = 0
     */
    /* 2 */
    parport_write_control(p, 0);

    /* Check if Portman PC/P 2x4 is out there. */
    /* 3 */
    parport_write_control(p, RXDATA0); /* Write Strobe=0 to command reg. */

    /* Check for ESTB to be clear */
    /* 4 */
    if (parport_read_status(p) & ESTB) == ESTB {
        return 1; /* CODE 1 - Strobe Failure. */
    }

    /* Set for RXDATA0 where no damage will be done. */
    /* 5 */
    parport_write_control(p, RXDATA0 | STROBE); /* Write Strobe=1 to command reg. */

    /* 6 */
    if (parport_read_status(p) & ESTB) != ESTB {
        return 1; /* CODE 1 - Strobe Failure. */
    }

    /* 7 */
    parport_write_control(p, 0); /* Reset Strobe=0. */

    /* Check if Tx circuitry is functioning properly.  If initialized
     * unit TxEmpty is false, send out char and see if it goes true.
     */
    /* 8 */
    parport_write_control(p, TXDATA0); /* Tx channel 0, strobe off. */

    /* If PCP channel's TxEmpty is set (TxEmpty is read through the PP
     * Status Register), then go write data.  Else go back and wait.
     */
    /* 9 */
    if (parport_read_status(p) & TXEMPTY) == 0 {
        return 2;
    }

    /* Return OK status. */
    0
}

unsafe extern "C" fn portman_device_init(pm: *mut portman) -> c_int {
    portman_flush_input(pm, 0);
    portman_flush_input(pm, 1);

    0
}

/*********************************************************************
 * Rawmidi
 *********************************************************************/
unsafe extern "C" fn snd_portman_midi_open(_substream: *mut snd_rawmidi_substream) -> c_int {
    0
}

unsafe extern "C" fn snd_portman_midi_close(_substream: *mut snd_rawmidi_substream) -> c_int {
    0
}

unsafe extern "C" fn snd_portman_midi_input_trigger(
    substream: *mut snd_rawmidi_substream,
    up: c_int,
) {
    let pm = (*(*substream).rmidi).private_data as *mut portman;

    let flags = spin_lock_irqsave(&mut (*pm).reg_lock);
    if up != 0 {
        (*pm).mode[(*substream).number as usize] |= PORTMAN2X4_MODE_INPUT_TRIGGERED;
    } else {
        (*pm).mode[(*substream).number as usize] &= !PORTMAN2X4_MODE_INPUT_TRIGGERED;
    }
    spin_unlock_irqrestore(&mut (*pm).reg_lock, flags);
}

unsafe extern "C" fn snd_portman_midi_output_trigger(
    substream: *mut snd_rawmidi_substream,
    up: c_int,
) {
    let pm = (*(*substream).rmidi).private_data as *mut portman;
    let mut byte: u8 = 0;

    let flags = spin_lock_irqsave(&mut (*pm).reg_lock);
    if up != 0 {
        while snd_rawmidi_transmit(substream, &mut byte, 1) == 1 {
            portman_write_midi(pm, (*substream).number, byte);
        }
    }
    spin_unlock_irqrestore(&mut (*pm).reg_lock, flags);
}

static snd_portman_midi_output: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_portman_midi_open),
    close: Some(snd_portman_midi_close),
    trigger: Some(snd_portman_midi_output_trigger),
};

static snd_portman_midi_input: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_portman_midi_open),
    close: Some(snd_portman_midi_close),
    trigger: Some(snd_portman_midi_input_trigger),
};

/* Create and initialize the rawmidi component */
unsafe extern "C" fn snd_portman_rawmidi_create(card: *mut snd_card) -> c_int {
    let pm = (*card).private_data as *mut portman;
    let mut rmidi: *mut snd_rawmidi = ptr::null_mut();
    let mut substream: *mut snd_rawmidi_substream;
    let mut err: c_int;

    err = snd_rawmidi_new(
        card,
        CARD_NAME.as_ptr(),
        0,
        PORTMAN_NUM_OUTPUT_PORTS as c_int,
        PORTMAN_NUM_INPUT_PORTS as c_int,
        &mut rmidi,
    );
    if err < 0 {
        return err;
    }

    (*rmidi).private_data = pm as *mut c_void;
    strscpy((*rmidi).name.as_mut_ptr(), CARD_NAME.as_ptr());
    (*rmidi).info_flags =
        SNDRV_RAWMIDI_INFO_OUTPUT | SNDRV_RAWMIDI_INFO_INPUT | SNDRV_RAWMIDI_INFO_DUPLEX;

    (*pm).rmidi = rmidi;

    /* register rawmidi ops */
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_OUTPUT, &snd_portman_midi_output);
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_INPUT, &snd_portman_midi_input);

    /* name substreams */
    /* output */
    substream =
        (*rmidi).streams[SNDRV_RAWMIDI_STREAM_OUTPUT as usize].substreams.next as *mut snd_rawmidi_substream;
    while !substream.is_null()
        && &mut (*substream).list as *mut list_head
            != &mut (*rmidi).streams[SNDRV_RAWMIDI_STREAM_OUTPUT as usize].substreams
                as *mut list_head
    {
        sprintf(
            (*substream).name.as_mut_ptr(),
            b"Portman2x4 %d\0".as_ptr(),
            (*substream).number + 1,
        );
        substream = (*substream).list.next as *mut snd_rawmidi_substream;
    }
    /* input */
    substream =
        (*rmidi).streams[SNDRV_RAWMIDI_STREAM_INPUT as usize].substreams.next as *mut snd_rawmidi_substream;
    while !substream.is_null()
        && &mut (*substream).list as *mut list_head
            != &mut (*rmidi).streams[SNDRV_RAWMIDI_STREAM_INPUT as usize].substreams
                as *mut list_head
    {
        (*pm).midi_input[(*substream).number as usize] = substream;
        sprintf(
            (*substream).name.as_mut_ptr(),
            b"Portman2x4 %d\0".as_ptr(),
            (*substream).number + 1,
        );
        substream = (*substream).list.next as *mut snd_rawmidi_substream;
    }

    err
}

/*********************************************************************
 * parport stuff
 *********************************************************************/
unsafe extern "C" fn snd_portman_interrupt(userdata: *mut c_void) {
    let mut midivalue: u8 = 0;
    let pm = (*(userdata as *mut snd_card)).private_data as *mut portman;

    spin_lock(&mut (*pm).reg_lock);

    /* While any input data is waiting */
    while (portman_read_status(pm) & INT_REQ) == INT_REQ {
        /* If data available on channel 0,
           read it and stuff it into the queue. */
        if portman_data_avail(pm, 0) != 0 {
            /* Read Midi */
            midivalue = portman_read_midi(pm, 0) as u8;
            /* put midi into queue... */
            if ((*pm).mode[0] & PORTMAN2X4_MODE_INPUT_TRIGGERED) != 0 {
                snd_rawmidi_receive((*pm).midi_input[0], &mut midivalue, 1);
            }
        }
        /* If data available on channel 1,
           read it and stuff it into the queue. */
        if portman_data_avail(pm, 1) != 0 {
            /* Read Midi */
            midivalue = portman_read_midi(pm, 1) as u8;
            /* put midi into queue... */
            if ((*pm).mode[1] & PORTMAN2X4_MODE_INPUT_TRIGGERED) != 0 {
                snd_rawmidi_receive((*pm).midi_input[1], &mut midivalue, 1);
            }
        }
    }

    spin_unlock(&mut (*pm).reg_lock);
}

unsafe extern "C" fn snd_portman_attach(p: *mut parport) {
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

unsafe extern "C" fn snd_portman_detach(_p: *mut parport) {
    /* nothing to do here */
}

unsafe extern "C" fn snd_portman_dev_probe(pardev: *mut pardevice) -> c_int {
    if strcmp((*pardev).name, DRIVER_NAME.as_ptr()) != 0 {
        return -ENODEV;
    }

    0
}

static mut portman_parport_driver: parport_driver = parport_driver {
    name: b"portman2x4\0".as_ptr(),
    probe: Some(snd_portman_dev_probe),
    match_port: Some(snd_portman_attach),
    detach: Some(snd_portman_detach),
};

/*********************************************************************
 * platform stuff
 *********************************************************************/
unsafe extern "C" fn snd_portman_card_private_free(card: *mut snd_card) {
    let pm = (*card).private_data as *mut portman;
    let pardev = (*pm).pardev;

    if !pardev.is_null() {
        parport_release(pardev);
        parport_unregister_device(pardev);
    }

    portman_free(pm);
}

unsafe extern "C" fn snd_portman_probe(pdev: *mut platform_device) -> c_int {
    let mut pardev: *mut pardevice;
    let p: *mut parport;
    let mut dev: c_int = (*pdev).id;
    let mut card: *mut snd_card = ptr::null_mut();
    let mut pm: *mut portman = ptr::null_mut();
    let mut err: c_int;
    let mut portman_cb = pardev_cb {
        preempt: ptr::null_mut(),
        wakeup: ptr::null_mut(),
        irq_func: Some(snd_portman_interrupt), /* ISR */
        flags: PARPORT_DEV_EXCL,               /* flags */
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

    if dev >= SNDRV_CARDS {
        return -ENODEV;
    }
    if !enable[dev as usize] {
        return -ENOENT;
    }

    err = snd_card_new(
        &mut (*pdev).dev,
        index[dev as usize],
        id[dev as usize],
        THIS_MODULE,
        0,
        &mut card,
    );
    if err < 0 {
        dev_dbg(&mut (*pdev).dev, b"Cannot create card\n\0".as_ptr());
        return err;
    }
    strscpy((*card).driver.as_mut_ptr(), DRIVER_NAME.as_ptr());
    strscpy((*card).shortname.as_mut_ptr(), CARD_NAME.as_ptr());
    sprintf(
        (*card).longname.as_mut_ptr(),
        b"%s at 0x%lx, irq %i\0".as_ptr(),
        (*card).shortname.as_ptr(),
        (*p).base,
        (*p).irq,
    );

    portman_cb.private = card as *mut c_void; /* private */
    pardev = parport_register_dev_model(
        p,                    /* port */
        DRIVER_NAME.as_ptr(), /* name */
        &mut portman_cb,      /* callbacks */
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

    err = portman_create(card, pardev, &mut pm);
    if err < 0 {
        dev_dbg((*card).dev, b"Cannot create main component\n\0".as_ptr());
        parport_release(pardev);
        parport_unregister_device(pardev);
        snd_card_free(card);
        return err;
    }
    (*card).private_data = pm as *mut c_void;
    (*card).private_free = Some(snd_portman_card_private_free);

    err = portman_probe(p);
    if err != 0 {
        err = -EIO;
        snd_card_free(card);
        return err;
    }

    err = snd_portman_rawmidi_create(card);
    if err < 0 {
        dev_dbg((*card).dev, b"Creating Rawmidi component failed\n\0".as_ptr());
        snd_card_free(card);
        return err;
    }

    /* init device */
    err = portman_device_init(pm);
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

    dev_info((*card).dev, b"Portman 2x4 on 0x%lx\n\0".as_ptr(), (*p).base);
    0
}

unsafe fn goto_err(card: *mut snd_card, err: c_int) -> c_int {
    snd_card_free(card);
    err
}

unsafe extern "C" fn snd_portman_remove(pdev: *mut platform_device) {
    let card = platform_get_drvdata(pdev) as *mut snd_card;

    if !card.is_null() {
        snd_card_free(card);
    }
}

static mut snd_portman_driver: platform_driver = platform_driver {
    probe: Some(snd_portman_probe),
    remove: Some(snd_portman_remove),
    driver: driver_private {
        name: PLATFORM_DRIVER.as_ptr(),
    },
};

/*********************************************************************
 * module init stuff
 *********************************************************************/
unsafe extern "C" fn snd_portman_unregister_all() {
    let mut i: c_int = 0;

    while i < SNDRV_CARDS {
        if !platform_devices[i as usize].is_null() {
            platform_device_unregister(platform_devices[i as usize]);
            platform_devices[i as usize] = ptr::null_mut();
        }
        i += 1;
    }
    platform_driver_unregister(&mut snd_portman_driver);
    parport_unregister_driver(&mut portman_parport_driver);
}

unsafe extern "C" fn snd_portman_module_init() -> c_int {
    let err: c_int;

    err = platform_driver_register(&mut snd_portman_driver);
    if err < 0 {
        return err;
    }

    if parport_register_driver(&mut portman_parport_driver) != 0 {
        platform_driver_unregister(&mut snd_portman_driver);
        return -EIO;
    }

    if device_count == 0 {
        snd_portman_unregister_all();
        return -ENODEV;
    }

    0
}

unsafe extern "C" fn snd_portman_module_exit() {
    snd_portman_unregister_all();
}

/* module_init(snd_portman_module_init);
 * module_exit(snd_portman_module_exit);
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
