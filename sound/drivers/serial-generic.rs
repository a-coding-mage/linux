// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   serial-generic.c
 *   Copyright (c) by Daniel Kaehn <kaehndan@gmail.com
 *   Based on serial-u16550.c by Jaroslav Kysela <perex@perex.cz>,
 *		                 Isaku Yamahata <yamahata@private.email.ne.jp>,
 *		                 George Hansper <ghansper@apana.org.au>,
 *		                 Hannu Savolainen
 *
 * Generic serial MIDI driver using the serdev serial bus API for hardware interaction
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

// MODULE_DESCRIPTION("Generic serial MIDI driver");
// MODULE_LICENSE("GPL");

type size_t = usize;
type u8 = ::core::ffi::c_uchar;
type c_char = ::core::ffi::c_char;
type c_int = ::core::ffi::c_int;
type c_uint = ::core::ffi::c_uint;
type c_ulong = ::core::ffi::c_ulong;
type c_void = ::core::ffi::c_void;

const SERIAL_MODE_INPUT_OPEN: c_ulong = 1;
const SERIAL_MODE_OUTPUT_OPEN: c_ulong = 2;
const SERIAL_MODE_INPUT_TRIGGERED: c_ulong = 3;
const SERIAL_MODE_OUTPUT_TRIGGERED: c_ulong = 4;

const SERIAL_TX_STATE_ACTIVE: c_ulong = 1;
const SERIAL_TX_STATE_WAKEUP: c_ulong = 2;

const INTERNAL_BUF_SIZE: usize = 256;

const SNDRV_RAWMIDI_STREAM_OUTPUT: usize = 0;
const SNDRV_RAWMIDI_STREAM_INPUT: usize = 1;
const SNDRV_RAWMIDI_INFO_OUTPUT: c_uint = 0x00000001;
const SNDRV_RAWMIDI_INFO_INPUT: c_uint = 0x00000002;
const SNDRV_RAWMIDI_INFO_DUPLEX: c_uint = 0x00000004;
const SNDRV_DEFAULT_IDX1: c_int = -1;

unsafe extern "C" {
    static SNDRV_DEFAULT_STR1: *const c_char;
    static THIS_MODULE: *mut c_void;
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct serdev_controller {
    pub nr: c_int,
}

#[repr(C)]
pub struct serdev_device {
    pub dev: device,
    pub ctrl: *mut serdev_controller,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct snd_rawmidi_substream {
    pub rmidi: *mut snd_rawmidi,
    pub list: list_head,
    pub number: c_int,
    pub name: [c_char; 32],
}

#[repr(C)]
pub struct snd_rawmidi_str {
    pub substreams: list_head,
}

#[repr(C)]
pub struct snd_rawmidi {
    pub card: *mut snd_card,
    pub streams: [snd_rawmidi_str; 2],
    pub name: [c_char; 80],
    pub info_flags: c_uint,
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct serdev_device_ops {
    pub receive_buf:
        Option<unsafe extern "C" fn(*mut serdev_device, *const u8, size_t) -> size_t>,
    pub write_wakeup: Option<unsafe extern "C" fn(*mut serdev_device)>,
}

#[repr(C)]
pub struct snd_rawmidi_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub ioctl: Option<unsafe extern "C" fn()>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream, c_int)>,
    pub drain: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream)>,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct serdev_device_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut serdev_device) -> c_int>,
}

#[repr(C)]
pub struct snd_serial_generic {
    pub serdev: *mut serdev_device,

    pub card: *mut snd_card,
    pub rmidi: *mut snd_rawmidi,
    pub midi_output: *mut snd_rawmidi_substream,
    pub midi_input: *mut snd_rawmidi_substream,

    pub baudrate: c_uint,

    pub filemode: c_ulong, /* open status of file */
    pub tx_work: work_struct,
    pub tx_state: c_ulong,

    pub tx_buf: [c_char; INTERNAL_BUF_SIZE],
}

unsafe extern "C" {
    fn test_and_set_bit(nr: c_ulong, addr: *mut c_ulong) -> c_int;
    fn set_bit(nr: c_ulong, addr: *mut c_ulong);
    fn clear_bit(nr: c_ulong, addr: *mut c_ulong);
    fn test_bit(nr: c_ulong, addr: *const c_ulong) -> c_int;
    fn schedule_work(work: *mut work_struct) -> c_int;
    fn cancel_work_sync(work: *mut work_struct) -> bool;
    fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));

    fn serdev_device_get_drvdata(serdev: *mut serdev_device) -> *mut snd_serial_generic;
    fn serdev_device_set_drvdata(serdev: *mut serdev_device, data: *mut snd_serial_generic);
    fn serdev_device_set_client_ops(serdev: *mut serdev_device, ops: *const serdev_device_ops);
    fn serdev_device_open(serdev: *mut serdev_device) -> c_int;
    fn serdev_device_close(serdev: *mut serdev_device);
    fn serdev_device_set_baudrate(serdev: *mut serdev_device, baudrate: c_uint) -> c_uint;
    fn serdev_device_write_buf(
        serdev: *mut serdev_device,
        buf: *const c_char,
        count: c_int,
    ) -> c_int;
    fn serdev_device_write_flush(serdev: *mut serdev_device);

    fn snd_rawmidi_transmit_empty(substream: *mut snd_rawmidi_substream) -> c_int;
    fn snd_rawmidi_transmit_peek(
        substream: *mut snd_rawmidi_substream,
        buf: *mut c_char,
        count: c_int,
    ) -> c_int;
    fn snd_rawmidi_transmit_ack(substream: *mut snd_rawmidi_substream, count: c_int) -> c_int;
    fn snd_rawmidi_receive(
        substream: *mut snd_rawmidi_substream,
        buf: *const u8,
        count: size_t,
    ) -> c_int;
    fn snd_rawmidi_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        output_count: c_int,
        input_count: c_int,
        rrawmidi: *mut *mut snd_rawmidi,
    ) -> c_int;
    fn snd_rawmidi_set_ops(
        rmidi: *mut snd_rawmidi,
        stream: c_int,
        ops: *const snd_rawmidi_ops,
    );

    fn snd_devm_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *const c_char,
        module: *mut c_void,
        extra_size: size_t,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;

    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut c_uint)
        -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
}

unsafe fn container_of_tx_work(work: *mut work_struct) -> *mut snd_serial_generic {
    (work as *mut u8).offset(-(::core::mem::offset_of!(snd_serial_generic, tx_work) as isize))
        as *mut snd_serial_generic
}

unsafe extern "C" fn snd_serial_generic_tx_wakeup(drvdata: *mut snd_serial_generic) {
    if unsafe { test_and_set_bit(SERIAL_TX_STATE_ACTIVE, &mut (*drvdata).tx_state) } != 0 {
        unsafe { set_bit(SERIAL_TX_STATE_WAKEUP, &mut (*drvdata).tx_state) };
    }

    unsafe { schedule_work(&mut (*drvdata).tx_work) };
}

unsafe extern "C" fn snd_serial_generic_tx_work(work: *mut work_struct) {
    let mut num_bytes: c_int;
    let drvdata: *mut snd_serial_generic = unsafe { container_of_tx_work(work) };
    let substream: *mut snd_rawmidi_substream = unsafe { (*drvdata).midi_output };

    unsafe { clear_bit(SERIAL_TX_STATE_WAKEUP, &mut (*drvdata).tx_state) };

    while unsafe { snd_rawmidi_transmit_empty(substream) } == 0 {
        if unsafe { test_bit(SERIAL_MODE_OUTPUT_OPEN, &(*drvdata).filemode) } == 0 {
            break;
        }

        num_bytes = unsafe {
            snd_rawmidi_transmit_peek(
                substream,
                (*drvdata).tx_buf.as_mut_ptr(),
                INTERNAL_BUF_SIZE as c_int,
            )
        };
        num_bytes = unsafe {
            serdev_device_write_buf((*drvdata).serdev, (*drvdata).tx_buf.as_ptr(), num_bytes)
        };

        if num_bytes == 0 {
            break;
        }

        unsafe { snd_rawmidi_transmit_ack(substream, num_bytes) };

        if unsafe { test_bit(SERIAL_TX_STATE_WAKEUP, &(*drvdata).tx_state) } == 0 {
            break;
        }
    }

    unsafe { clear_bit(SERIAL_TX_STATE_ACTIVE, &mut (*drvdata).tx_state) };
}

unsafe extern "C" fn snd_serial_generic_write_wakeup(serdev: *mut serdev_device) {
    let drvdata: *mut snd_serial_generic = unsafe { serdev_device_get_drvdata(serdev) };

    unsafe { snd_serial_generic_tx_wakeup(drvdata) };
}

unsafe extern "C" fn snd_serial_generic_receive_buf(
    serdev: *mut serdev_device,
    buf: *const u8,
    count: size_t,
) -> size_t {
    let ret: c_int;
    let drvdata: *mut snd_serial_generic = unsafe { serdev_device_get_drvdata(serdev) };

    if unsafe { test_bit(SERIAL_MODE_INPUT_OPEN, &(*drvdata).filemode) } == 0 {
        return 0;
    }

    ret = unsafe { snd_rawmidi_receive((*drvdata).midi_input, buf, count) };
    if ret < 0 {
        0
    } else {
        ret as size_t
    }
}

static snd_serial_generic_serdev_device_ops: serdev_device_ops = serdev_device_ops {
    receive_buf: Some(snd_serial_generic_receive_buf),
    write_wakeup: Some(snd_serial_generic_write_wakeup),
};

unsafe extern "C" fn snd_serial_generic_ensure_serdev_open(
    drvdata: *mut snd_serial_generic,
) -> c_int {
    let err: c_int;
    let actual_baud: c_uint;

    if unsafe { (*drvdata).filemode } != 0 {
        return 0;
    }

    unsafe {
        dev_dbg(
            (*(*drvdata).card).dev,
            c"Opening serial port for card %s\n".as_ptr(),
            (*(*drvdata).card).shortname.as_ptr(),
        )
    };
    err = unsafe { serdev_device_open((*drvdata).serdev) };
    if err < 0 {
        return err;
    }

    actual_baud =
        unsafe { serdev_device_set_baudrate((*drvdata).serdev, (*drvdata).baudrate) };
    if actual_baud != unsafe { (*drvdata).baudrate } {
        unsafe {
            dev_warn(
                (*(*drvdata).card).dev,
                c"requested %d baud for card %s but it was actually set to %d\n".as_ptr(),
                (*drvdata).baudrate,
                (*(*drvdata).card).shortname.as_ptr(),
                actual_baud,
            )
        };
    }

    0
}

unsafe extern "C" fn snd_serial_generic_input_open(
    substream: *mut snd_rawmidi_substream,
) -> c_int {
    let err: c_int;
    let drvdata: *mut snd_serial_generic =
        unsafe { (*(*(*substream).rmidi).card).private_data as *mut snd_serial_generic };

    unsafe {
        dev_dbg(
            (*drvdata).card.as_ref().unwrap().dev,
            c"Opening input for card %s\n".as_ptr(),
            (*(*drvdata).card).shortname.as_ptr(),
        )
    };

    err = unsafe { snd_serial_generic_ensure_serdev_open(drvdata) };
    if err < 0 {
        return err;
    }

    unsafe { set_bit(SERIAL_MODE_INPUT_OPEN, &mut (*drvdata).filemode) };
    unsafe { (*drvdata).midi_input = substream };
    0
}

unsafe extern "C" fn snd_serial_generic_input_close(
    substream: *mut snd_rawmidi_substream,
) -> c_int {
    let drvdata: *mut snd_serial_generic =
        unsafe { (*(*(*substream).rmidi).card).private_data as *mut snd_serial_generic };

    unsafe {
        dev_dbg(
            (*(*drvdata).card).dev,
            c"Closing input for card %s\n".as_ptr(),
            (*(*drvdata).card).shortname.as_ptr(),
        )
    };

    unsafe { clear_bit(SERIAL_MODE_INPUT_OPEN, &mut (*drvdata).filemode) };
    unsafe { clear_bit(SERIAL_MODE_INPUT_TRIGGERED, &mut (*drvdata).filemode) };

    unsafe { (*drvdata).midi_input = ::core::ptr::null_mut() };

    if unsafe { (*drvdata).filemode } == 0 {
        unsafe { serdev_device_close((*drvdata).serdev) };
    }
    0
}

unsafe extern "C" fn snd_serial_generic_input_trigger(
    substream: *mut snd_rawmidi_substream,
    up: c_int,
) {
    let drvdata: *mut snd_serial_generic =
        unsafe { (*(*(*substream).rmidi).card).private_data as *mut snd_serial_generic };

    if up != 0 {
        unsafe { set_bit(SERIAL_MODE_INPUT_TRIGGERED, &mut (*drvdata).filemode) };
    } else {
        unsafe { clear_bit(SERIAL_MODE_INPUT_TRIGGERED, &mut (*drvdata).filemode) };
    }
}

unsafe extern "C" fn snd_serial_generic_output_open(
    substream: *mut snd_rawmidi_substream,
) -> c_int {
    let drvdata: *mut snd_serial_generic =
        unsafe { (*(*(*substream).rmidi).card).private_data as *mut snd_serial_generic };
    let err: c_int;

    unsafe {
        dev_dbg(
            (*(*drvdata).card).dev,
            c"Opening output for card %s\n".as_ptr(),
            (*(*drvdata).card).shortname.as_ptr(),
        )
    };

    err = unsafe { snd_serial_generic_ensure_serdev_open(drvdata) };
    if err < 0 {
        return err;
    }

    unsafe { set_bit(SERIAL_MODE_OUTPUT_OPEN, &mut (*drvdata).filemode) };

    unsafe { (*drvdata).midi_output = substream };
    0
}

unsafe extern "C" fn snd_serial_generic_output_close(
    substream: *mut snd_rawmidi_substream,
) -> c_int {
    let drvdata: *mut snd_serial_generic =
        unsafe { (*(*(*substream).rmidi).card).private_data as *mut snd_serial_generic };

    unsafe {
        dev_dbg(
            (*(*drvdata).card).dev,
            c"Closing output for card %s\n".as_ptr(),
            (*(*drvdata).card).shortname.as_ptr(),
        )
    };

    unsafe { clear_bit(SERIAL_MODE_OUTPUT_OPEN, &mut (*drvdata).filemode) };
    unsafe { clear_bit(SERIAL_MODE_OUTPUT_TRIGGERED, &mut (*drvdata).filemode) };

    if unsafe { (*drvdata).filemode } == 0 {
        unsafe { serdev_device_close((*drvdata).serdev) };
    }

    unsafe { (*drvdata).midi_output = ::core::ptr::null_mut() };

    0
}

unsafe extern "C" fn snd_serial_generic_output_trigger(
    substream: *mut snd_rawmidi_substream,
    up: c_int,
) {
    let drvdata: *mut snd_serial_generic =
        unsafe { (*(*(*substream).rmidi).card).private_data as *mut snd_serial_generic };

    if up != 0 {
        unsafe { set_bit(SERIAL_MODE_OUTPUT_TRIGGERED, &mut (*drvdata).filemode) };
    } else {
        unsafe { clear_bit(SERIAL_MODE_OUTPUT_TRIGGERED, &mut (*drvdata).filemode) };
    }

    if up != 0 {
        unsafe { snd_serial_generic_tx_wakeup(drvdata) };
    }
}

unsafe extern "C" fn snd_serial_generic_output_drain(
    substream: *mut snd_rawmidi_substream,
) {
    let drvdata: *mut snd_serial_generic =
        unsafe { (*(*(*substream).rmidi).card).private_data as *mut snd_serial_generic };

    /* Flush any pending characters */
    unsafe { serdev_device_write_flush((*drvdata).serdev) };
    unsafe { cancel_work_sync(&mut (*drvdata).tx_work) };
}

static snd_serial_generic_output: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_serial_generic_output_open),
    close: Some(snd_serial_generic_output_close),
    ioctl: None,
    trigger: Some(snd_serial_generic_output_trigger),
    drain: Some(snd_serial_generic_output_drain),
};

static snd_serial_generic_input: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_serial_generic_input_open),
    close: Some(snd_serial_generic_input_close),
    ioctl: None,
    trigger: Some(snd_serial_generic_input_trigger),
    drain: None,
};

unsafe extern "C" fn snd_serial_generic_parse_dt(
    serdev: *mut serdev_device,
    drvdata: *mut snd_serial_generic,
) {
    let err: c_int;

    err = unsafe {
        of_property_read_u32(
            (*serdev).dev.of_node,
            c"current-speed".as_ptr(),
            &mut (*drvdata).baudrate,
        )
    };
    if err < 0 {
        unsafe {
            dev_dbg(
                (*(*drvdata).card).dev,
                c"MIDI device reading of current-speed DT param failed with error %d, using default of 38400\n".as_ptr(),
                err,
            )
        };
        unsafe { (*drvdata).baudrate = 38400 };
    }
}

unsafe extern "C" fn snd_serial_generic_substreams(
    stream: *mut snd_rawmidi_str,
    dev_num: c_int,
) {
    let mut pos: *mut list_head = unsafe { (*stream).substreams.next };

    while pos != unsafe { &mut (*stream).substreams } {
        let substream: *mut snd_rawmidi_substream = (pos as *mut u8)
            .offset(-(::core::mem::offset_of!(snd_rawmidi_substream, list) as isize))
            as *mut snd_rawmidi_substream;
        unsafe {
            sprintf(
                (*substream).name.as_mut_ptr(),
                c"Serial MIDI %d-%d".as_ptr(),
                dev_num,
                (*substream).number,
            )
        };
        pos = unsafe { (*pos).next };
    }
}

unsafe extern "C" fn snd_serial_generic_rmidi(
    drvdata: *mut snd_serial_generic,
    outs: c_int,
    ins: c_int,
    rmidi: *mut *mut snd_rawmidi,
) -> c_int {
    let mut rrawmidi: *mut snd_rawmidi = ::core::ptr::null_mut();
    let err: c_int;

    err = unsafe {
        snd_rawmidi_new(
            (*drvdata).card,
            (*(*drvdata).card).driver.as_ptr(),
            0,
            outs,
            ins,
            &mut rrawmidi,
        )
    };

    if err < 0 {
        return err;
    }

    unsafe {
        snd_rawmidi_set_ops(
            rrawmidi,
            SNDRV_RAWMIDI_STREAM_INPUT as c_int,
            &snd_serial_generic_input,
        )
    };
    unsafe {
        snd_rawmidi_set_ops(
            rrawmidi,
            SNDRV_RAWMIDI_STREAM_OUTPUT as c_int,
            &snd_serial_generic_output,
        )
    };
    unsafe { strscpy((*rrawmidi).name.as_mut_ptr(), (*(*drvdata).card).shortname.as_ptr()) };

    unsafe {
        snd_serial_generic_substreams(
            &mut (*rrawmidi).streams[SNDRV_RAWMIDI_STREAM_OUTPUT],
            (*(*(*drvdata).serdev).ctrl).nr,
        )
    };
    unsafe {
        snd_serial_generic_substreams(
            &mut (*rrawmidi).streams[SNDRV_RAWMIDI_STREAM_INPUT],
            (*(*(*drvdata).serdev).ctrl).nr,
        )
    };

    unsafe {
        (*rrawmidi).info_flags =
            SNDRV_RAWMIDI_INFO_OUTPUT | SNDRV_RAWMIDI_INFO_INPUT | SNDRV_RAWMIDI_INFO_DUPLEX
    };

    if !rmidi.is_null() {
        unsafe { *rmidi = rrawmidi };
    }
    0
}

unsafe extern "C" fn snd_serial_generic_probe(serdev: *mut serdev_device) -> c_int {
    let mut card: *mut snd_card = ::core::ptr::null_mut();
    let drvdata: *mut snd_serial_generic;
    let mut err: c_int;

    err = unsafe {
        snd_devm_card_new(
            &mut (*serdev).dev,
            SNDRV_DEFAULT_IDX1,
            SNDRV_DEFAULT_STR1,
            THIS_MODULE,
            ::core::mem::size_of::<snd_serial_generic>(),
            &mut card,
        )
    };

    if err < 0 {
        return err;
    }

    unsafe { strscpy((*card).driver.as_mut_ptr(), c"SerialMIDI".as_ptr()) };
    unsafe {
        sprintf(
            (*card).shortname.as_mut_ptr(),
            c"SerialMIDI-%d".as_ptr(),
            (*(*serdev).ctrl).nr,
        )
    };
    unsafe {
        sprintf(
            (*card).longname.as_mut_ptr(),
            c"Serial MIDI device at serial%d".as_ptr(),
            (*(*serdev).ctrl).nr,
        )
    };

    drvdata = unsafe { (*card).private_data as *mut snd_serial_generic };

    unsafe {
        (*drvdata).serdev = serdev;
        (*drvdata).card = card;
    }

    unsafe { snd_serial_generic_parse_dt(serdev, drvdata) };

    unsafe { INIT_WORK(&mut (*drvdata).tx_work, snd_serial_generic_tx_work) };

    err = unsafe { snd_serial_generic_rmidi(drvdata, 1, 1, &mut (*drvdata).rmidi) };
    if err < 0 {
        return err;
    }

    unsafe { serdev_device_set_client_ops(serdev, &snd_serial_generic_serdev_device_ops) };
    unsafe { serdev_device_set_drvdata((*drvdata).serdev, drvdata) };

    err = unsafe { snd_card_register(card) };
    if err < 0 {
        return err;
    }

    0
}

static snd_serial_generic_dt_ids: [of_device_id; 2] = [
    of_device_id {
        compatible: c"serial-midi".as_ptr(),
    },
    of_device_id {
        compatible: ::core::ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, snd_serial_generic_dt_ids);

static mut snd_serial_generic_driver: serdev_device_driver = serdev_device_driver {
    driver: device_driver {
        name: c"snd-serial-generic".as_ptr(),
        of_match_table: snd_serial_generic_dt_ids.as_ptr(),
    },
    probe: Some(snd_serial_generic_probe),
};

// module_serdev_device_driver(snd_serial_generic_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
