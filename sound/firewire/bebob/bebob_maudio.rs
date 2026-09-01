// SPDX-License-Identifier: GPL-2.0-only
/*
 * bebob_maudio.c - a part of driver for BeBoB based devices
 *
 * Copyright (c) 2013-2014 Takashi Sakamoto
 */

// Original C dependencies:
// #include "./bebob.h"
// #include <sound/control.h>

use core::ffi::{c_int, c_uint, c_void};

type Bool = bool;
type U8 = u8;
type U32 = u32;
type U64 = u64;
type Le32 = u32;
type Be16 = u16;

/*
 * Just powering on, Firewire 410/Audiophile/1814 and ProjectMix I/O wait to
 * download firmware blob. To enable these devices, drivers should upload
 * firmware blob and send a command to initialize configuration to factory
 * settings when completing uploading. Then these devices generate bus reset
 * and are recognized as new devices with the firmware.
 *
 * But with firmware version 5058 or later, the firmware is stored to flash
 * memory in the device and drivers can tell bootloader to load the firmware
 * by sending a cue. This cue must be sent one time.
 *
 * For streaming, both of output and input streams are needed for Firewire 410
 * and Ozonic. The single stream is OK for the other devices even if the clock
 * source is not SYT-Match (I note no devices use SYT-Match).
 *
 * Without streaming, the devices except for Firewire Audiophile can mix any
 * input and output. For this reason, Audiophile cannot be used as standalone
 * mixer.
 *
 * Firewire 1814 and ProjectMix I/O uses special firmware. It will be freezed
 * when receiving any commands which the firmware can't understand. These
 * devices utilize completely different system to control. It is some
 * write-transaction directly into a certain address. All of addresses for mixer
 * functionality is between 0xffc700700000 to 0xffc70070009c.
 */

/* Offset from information register */
const INFO_OFFSET_SW_DATE: c_uint = 0x20;

/* Bootloader Protocol Version 1 */
const MAUDIO_BOOTLOADER_CUE1: U32 = 0x00000001;
/*
 * Initializing configuration to factory settings (= 0x1101), (swapped in line),
 * Command code is zero (= 0x00),
 * the number of operands is zero (= 0x00)(at least significant byte)
 */
const MAUDIO_BOOTLOADER_CUE2: U32 = 0x01110000;
/* padding */
const MAUDIO_BOOTLOADER_CUE3: U32 = 0x00000000;

const MAUDIO_SPECIFIC_ADDRESS: U64 = 0xffc700000000;

const METER_OFFSET: U64 = 0x00600000;

/* some device has sync info after metering data */
const METER_SIZE_SPECIAL: c_uint = 84; /* with sync info */
const METER_SIZE_FW410: c_uint = 76; /* with sync info */
const METER_SIZE_AUDIOPHILE: c_uint = 60; /* with sync info */
const METER_SIZE_SOLO: c_uint = 52; /* with sync info */
const METER_SIZE_OZONIC: c_uint = 48;
const METER_SIZE_NRV10: c_uint = 80;

/* labels for metering */
const ANA_IN: *const i8 = c"Analog In".as_ptr();
const ANA_OUT: *const i8 = c"Analog Out".as_ptr();
const DIG_IN: *const i8 = c"Digital In".as_ptr();
const SPDIF_IN: *const i8 = c"S/PDIF In".as_ptr();
const ADAT_IN: *const i8 = c"ADAT In".as_ptr();
const DIG_OUT: *const i8 = c"Digital Out".as_ptr();
const SPDIF_OUT: *const i8 = c"S/PDIF Out".as_ptr();
const ADAT_OUT: *const i8 = c"ADAT Out".as_ptr();
const STRM_IN: *const i8 = c"Stream In".as_ptr();
const AUX_OUT: *const i8 = c"Aux Out".as_ptr();
const HP_OUT: *const i8 = c"HP Out".as_ptr();
/* for NRV */
const UNKNOWN_METER: *const i8 = c"Unknown".as_ptr();

#[repr(C)]
pub struct special_params {
    is1814: Bool,
    clk_src: c_uint,
    dig_in_fmt: c_uint,
    dig_out_fmt: c_uint,
    clk_lock: c_uint,
    ctl_id_sync: *mut snd_ctl_elem_id,
}

unsafe extern "C" {
    fn fw_parent_device(unit: *mut fw_unit) -> *mut fw_device;
    fn snd_bebob_read_block(unit: *mut fw_unit, offset: c_uint, buf: *mut c_void, size: usize) -> c_int;
    fn kmalloc_array(n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn kmalloc(size: c_uint, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn fw_run_transaction(
        card: *mut fw_card,
        tcode: c_uint,
        node_id: c_uint,
        generation: c_uint,
        max_speed: c_uint,
        offset: U64,
        payload: *mut c_void,
        length: usize,
    ) -> c_int;
    fn snd_fw_transaction(
        unit: *mut fw_unit,
        tcode: c_uint,
        offset: U64,
        buf: *mut c_void,
        size: c_uint,
        flags: c_uint,
    ) -> c_int;
    fn amdtp_stream_running(stream: *mut amdtp_stream) -> Bool;
    fn fcp_avc_transaction(
        unit: *mut fw_unit,
        command: *mut U8,
        command_size: c_uint,
        response: *mut U8,
        response_size: c_uint,
        mask: c_uint,
    ) -> c_int;
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut snd_ctl_elem_id);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn ERR_PTR(error: isize) -> *mut c_void;
    fn avc_general_get_sig_fmt(unit: *mut fw_unit, rate: *mut c_uint, dir: c_uint, plug: c_uint) -> c_int;
    fn avc_general_set_sig_fmt(unit: *mut fw_unit, rate: c_uint, dir: c_uint, plug: c_uint) -> c_int;
    fn msleep(msecs: c_uint);
    fn snd_ctl_enum_info(
        info: *mut snd_ctl_elem_info,
        channels: c_uint,
        items: c_uint,
        names: *const *const i8,
    ) -> c_int;
    fn snd_kcontrol_chip(kctl: *mut snd_kcontrol) -> *mut snd_bebob;
    fn avc_audio_get_selector(unit: *mut fw_unit, subunit_id: c_uint, fb_id: c_uint, value: *mut c_uint) -> c_int;
    fn avc_audio_set_selector(unit: *mut fw_unit, subunit_id: c_uint, fb_id: c_uint, value: c_uint) -> c_int;
    fn snd_ctl_new1(template: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kctl: *mut snd_kcontrol) -> c_int;
    fn snd_bebob_stream_get_rate(bebob: *mut snd_bebob, rate: *mut c_uint) -> c_int;
    fn snd_bebob_stream_set_rate(bebob: *mut snd_bebob, rate: c_uint) -> c_int;
}

#[repr(C)] pub struct fw_unit { device: device }
#[repr(C)] pub struct fw_device { card: *mut fw_card, node_id: c_uint, generation: c_uint, max_speed: c_uint }
#[repr(C)] pub struct fw_card;
#[repr(C)] pub struct device;
#[repr(C)] pub struct snd_card { card_dev: device }
#[repr(C)] pub struct snd_ctl_elem_id;
#[repr(C)] pub struct amdtp_stream { context: *mut c_void }
#[repr(C)] pub struct snd_bebob_stream_formation { pcm: c_uint, midi: c_uint }
#[repr(C)] pub struct snd_bebob {
    unit: *mut fw_unit,
    card: *mut snd_card,
    mutex: mutex,
    rx_stream: amdtp_stream,
    tx_stream: amdtp_stream,
    tx_stream_formations: [snd_bebob_stream_formation; SND_BEBOB_STRM_FMT_ENTRIES],
    rx_stream_formations: [snd_bebob_stream_formation; SND_BEBOB_STRM_FMT_ENTRIES],
    maudio_special_quirk: *mut c_void,
    midi_input_ports: c_uint,
    midi_output_ports: c_uint,
    spec: *const snd_bebob_spec,
}
#[repr(C)] pub struct mutex;
#[repr(C)] pub struct snd_kcontrol { id: snd_ctl_elem_id }
#[repr(C)] pub struct snd_ctl_elem_info {
    type_: c_uint,
    count: c_uint,
    value: snd_ctl_elem_info_value,
}
#[repr(C)] pub union snd_ctl_elem_info_value { integer: snd_ctl_elem_info_integer }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_info_integer { min: i64, max: i64 }
#[repr(C)] pub struct snd_ctl_elem_value { value: snd_ctl_elem_value_value }
#[repr(C)] pub union snd_ctl_elem_value_value {
    enumerated: snd_ctl_elem_value_enumerated,
    integer: snd_ctl_elem_value_integer,
}
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_enumerated { item: [c_uint; 128] }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_integer { value: [i64; 128] }
#[repr(C)] pub struct snd_kcontrol_new {
    name: *const i8,
    iface: c_uint,
    access: c_uint,
    info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}
#[repr(C)] pub struct snd_bebob_rate_spec {
    get: Option<unsafe extern "C" fn(*mut snd_bebob, *mut c_uint) -> c_int>,
    set: Option<unsafe extern "C" fn(*mut snd_bebob, c_uint) -> c_int>,
}
#[repr(C)] pub struct snd_bebob_clock_spec {
    num: c_uint,
    types: *const snd_bebob_clock_type,
    get: Option<unsafe extern "C" fn(*mut snd_bebob, *mut c_uint) -> c_int>,
}
#[repr(C)] pub struct snd_bebob_meter_spec {
    num: c_uint,
    labels: *const *const i8,
    get: Option<unsafe extern "C" fn(*mut snd_bebob, *mut U32, c_uint) -> c_int>,
}
#[repr(C)] pub struct snd_bebob_spec {
    clock: *const snd_bebob_clock_spec,
    rate: *const snd_bebob_rate_spec,
    meter: *const snd_bebob_meter_spec,
}

type snd_bebob_clock_type = c_uint;

const GFP_KERNEL: c_uint = 0;
const ENXIO: c_int = 6;
const ENOMEM: c_int = 12;
const EIO: c_int = 5;
const EBUSY: c_int = 16;
const ENOSYS: c_int = 38;
const EINVAL: c_int = 22;
const EAGAIN: c_int = 11;
const TCODE_WRITE_BLOCK_REQUEST: c_uint = 0;
const TCODE_READ_BLOCK_REQUEST: c_uint = 0;
const RCODE_COMPLETE: c_int = 0;
const BEBOB_ADDR_REG_REQ: U64 = 0;
const SNDRV_CTL_EVENT_MASK_VALUE: c_uint = 0;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 0;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 0;
const SNDRV_CTL_ELEM_ACCESS_READ: c_uint = 0;
const SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_uint = 0;
const AVC_GENERAL_PLUG_DIR_IN: c_uint = 0;
const AVC_GENERAL_PLUG_DIR_OUT: c_uint = 0;
const SND_BEBOB_CLOCK_TYPE_INTERNAL: snd_bebob_clock_type = 0;
const SND_BEBOB_CLOCK_TYPE_EXTERNAL: snd_bebob_clock_type = 1;
const AMDTP_OUT_STREAM: usize = 0;
const AMDTP_IN_STREAM: usize = 1;
const SND_BEBOB_STRM_FMT_ENTRIES: usize = 7;

macro_rules! BIT {
    ($n:expr) => {
        1u32 << $n
    };
}

unsafe fn cpu_to_le32(v: U32) -> Le32 {
    v.to_le()
}

unsafe fn be16_to_cpu(v: Be16) -> u16 {
    u16::from_be(v)
}

unsafe fn be32_to_cpus(v: *mut U32) {
    *v = u32::from_be(*v);
}

/*
 * For some M-Audio devices, this module just send cue to load firmware. After
 * loading, the device generates bus reset and newly detected.
 *
 * If we make any transactions to load firmware, the operation may failed.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_bebob_maudio_load_firmware(unit: *mut fw_unit) -> c_int {
    let device = fw_parent_device(unit);
    let mut err: c_int;
    let rcode: c_int;
    let mut date: U64 = 0;
    let cues: *mut Le32;

    /* check date of software used to build */
    err = snd_bebob_read_block(unit, INFO_OFFSET_SW_DATE, &mut date as *mut _ as *mut c_void, core::mem::size_of::<U64>());
    if err < 0 {
        return err;
    }
    /*
     * firmware version 5058 or later has date later than "20070401", but
     * 'date' is not null-terminated.
     */
    if date < 0x3230303730343031u64 {
        dev_err(&mut (*unit).device, c"Use firmware version 5058 or later\n".as_ptr());
        return -ENXIO;
    }

    cues = kmalloc_array(3, core::mem::size_of::<Le32>(), GFP_KERNEL) as *mut Le32;
    if cues.is_null() {
        return -ENOMEM;
    }

    *cues.add(0) = cpu_to_le32(MAUDIO_BOOTLOADER_CUE1);
    *cues.add(1) = cpu_to_le32(MAUDIO_BOOTLOADER_CUE2);
    *cues.add(2) = cpu_to_le32(MAUDIO_BOOTLOADER_CUE3);

    rcode = fw_run_transaction(
        (*device).card,
        TCODE_WRITE_BLOCK_REQUEST,
        (*device).node_id,
        (*device).generation,
        (*device).max_speed,
        BEBOB_ADDR_REG_REQ,
        cues as *mut c_void,
        3 * core::mem::size_of::<Le32>(),
    );
    kfree(cues as *mut c_void);
    if rcode != RCODE_COMPLETE {
        dev_err(&mut (*unit).device, c"Failed to send a cue to load firmware\n".as_ptr());
        err = -EIO;
    }

    err
}

unsafe fn dev_err(_dev: *mut device, _fmt: *const i8) {}
unsafe fn dev_err_i(_dev: *mut device, _fmt: *const i8, _arg: c_int) {}

#[inline]
unsafe fn get_meter(bebob: *mut snd_bebob, buf: *mut c_void, size: c_uint) -> c_int {
    snd_fw_transaction(
        (*bebob).unit,
        TCODE_READ_BLOCK_REQUEST,
        MAUDIO_SPECIFIC_ADDRESS + METER_OFFSET,
        buf,
        size,
        0,
    )
}

unsafe fn check_clk_sync(bebob: *mut snd_bebob, size: c_uint, sync: *mut Bool) -> c_int {
    let mut err: c_int;
    let buf: *mut U8;

    buf = kmalloc(size, GFP_KERNEL) as *mut U8;
    if buf.is_null() {
        return -ENOMEM;
    }

    err = get_meter(bebob, buf as *mut c_void, size);
    if err < 0 {
        kfree(buf as *mut c_void);
        return err;
    }

    /* if synced, this value is the same as SFC of FDF in CIP header */
    *sync = *buf.add(size as usize - 2) != 0xff;
    kfree(buf as *mut c_void);
    err
}

/*
 * dig_fmt: 0x00:S/PDIF, 0x01:ADAT
 * clk_lock: 0x00:unlock, 0x01:lock
 */
unsafe fn avc_maudio_set_special_clk(
    bebob: *mut snd_bebob,
    clk_src: c_uint,
    dig_in_fmt: c_uint,
    dig_out_fmt: c_uint,
    clk_lock: c_uint,
) -> c_int {
    let params = (*bebob).maudio_special_quirk as *mut special_params;
    let mut err: c_int;
    let buf: *mut U8;

    if amdtp_stream_running(&mut (*bebob).rx_stream) || amdtp_stream_running(&mut (*bebob).tx_stream) {
        return -EBUSY;
    }

    buf = kmalloc(12, GFP_KERNEL) as *mut U8;
    if buf.is_null() {
        return -ENOMEM;
    }

    *buf.add(0) = 0x00; /* CONTROL */
    *buf.add(1) = 0xff; /* UNIT */
    *buf.add(2) = 0x00; /* vendor dependent */
    *buf.add(3) = 0x04; /* company ID high */
    *buf.add(4) = 0x00; /* company ID middle */
    *buf.add(5) = 0x04; /* company ID low */
    *buf.add(6) = (0xff & clk_src) as U8; /* clock source */
    *buf.add(7) = (0xff & dig_in_fmt) as U8; /* input digital format */
    *buf.add(8) = (0xff & dig_out_fmt) as U8; /* output digital format */
    *buf.add(9) = (0xff & clk_lock) as U8; /* lock these settings */
    *buf.add(10) = 0x00; /* padding  */
    *buf.add(11) = 0x00; /* padding */

    err = fcp_avc_transaction(
        (*bebob).unit,
        buf,
        12,
        buf,
        12,
        BIT!(1) | BIT!(2) | BIT!(3) | BIT!(4) | BIT!(5) | BIT!(6) | BIT!(7) | BIT!(8) | BIT!(9),
    );
    if err > 0 && err < 10 {
        err = -EIO;
    } else if *buf.add(0) == 0x08 {
        /* NOT IMPLEMENTED */
        err = -ENOSYS;
    } else if *buf.add(0) == 0x0a {
        /* REJECTED */
        err = -EINVAL;
    }
    if err < 0 {
        kfree(buf as *mut c_void);
        return err;
    }

    (*params).clk_src = *buf.add(6) as c_uint;
    (*params).dig_in_fmt = *buf.add(7) as c_uint;
    (*params).dig_out_fmt = *buf.add(8) as c_uint;
    (*params).clk_lock = *buf.add(9) as c_uint;

    if !(*params).ctl_id_sync.is_null() {
        snd_ctl_notify((*bebob).card, SNDRV_CTL_EVENT_MASK_VALUE, (*params).ctl_id_sync);
    }

    err = 0;
    kfree(buf as *mut c_void);
    err
}

unsafe fn special_stream_formation_set(bebob: *mut snd_bebob) {
    static CH_TABLE: [[[c_uint; 3]; 2]; 2] = [
        /* AMDTP_OUT_STREAM */
        [[6, 6, 4], /* SPDIF */ [12, 8, 4]], /* ADAT */
        /* AMDTP_IN_STREAM */
        [[10, 10, 2], /* SPDIF */ [16, 12, 2]], /* ADAT */
    ];
    let params = (*bebob).maudio_special_quirk as *mut special_params;
    let mut i: c_uint;
    let mut max: c_uint;

    max = (SND_BEBOB_STRM_FMT_ENTRIES - 1) as c_uint;
    if !(*params).is1814 {
        max -= 2;
    }

    i = 0;
    while i < max {
        (*bebob).tx_stream_formations[i as usize + 1].pcm =
            CH_TABLE[AMDTP_IN_STREAM][(*params).dig_in_fmt as usize][i as usize / 2];
        (*bebob).tx_stream_formations[i as usize + 1].midi = 1;

        (*bebob).rx_stream_formations[i as usize + 1].pcm =
            CH_TABLE[AMDTP_OUT_STREAM][(*params).dig_out_fmt as usize][i as usize / 2];
        (*bebob).rx_stream_formations[i as usize + 1].midi = 1;
        i += 1;
    }
}

unsafe fn add_special_controls(bebob: *mut snd_bebob) -> c_int {
    let mut kctl: *mut snd_kcontrol;
    let params = (*bebob).maudio_special_quirk as *mut special_params;
    let mut err: c_int;

    kctl = snd_ctl_new1(&SPECIAL_CLK_CTL, bebob as *mut c_void);
    err = snd_ctl_add((*bebob).card, kctl);
    if err < 0 {
        return err;
    }

    kctl = snd_ctl_new1(&SPECIAL_SYNC_CTL, bebob as *mut c_void);
    err = snd_ctl_add((*bebob).card, kctl);
    if err < 0 {
        return err;
    }
    (*params).ctl_id_sync = &mut (*kctl).id;

    kctl = snd_ctl_new1(&SPECIAL_DIG_IN_IFACE_CTL, bebob as *mut c_void);
    err = snd_ctl_add((*bebob).card, kctl);
    if err < 0 {
        return err;
    }

    kctl = snd_ctl_new1(&SPECIAL_DIG_OUT_IFACE_CTL, bebob as *mut c_void);
    err = snd_ctl_add((*bebob).card, kctl);
    err
}

#[no_mangle]
pub unsafe extern "C" fn snd_bebob_maudio_special_discover(bebob: *mut snd_bebob, is1814: Bool) -> c_int {
    let params: *mut special_params;
    let mut err: c_int;

    params = devm_kzalloc(&mut (*(*bebob).card).card_dev, core::mem::size_of::<special_params>(), GFP_KERNEL) as *mut special_params;
    if params.is_null() {
        return -ENOMEM;
    }

    // C source uses guard(mutex)(&bebob->mutex).
    (*bebob).maudio_special_quirk = params as *mut c_void;
    (*params).is1814 = is1814;

    /* initialize these parameters because driver is not allowed to ask */
    (*bebob).rx_stream.context = ERR_PTR(-1);
    (*bebob).tx_stream.context = ERR_PTR(-1);
    err = avc_maudio_set_special_clk(bebob, 0x03, 0x00, 0x00, 0x00);
    if err < 0 {
        dev_err_i(&mut (*(*bebob).unit).device, c"fail to initialize clock params: %d\n".as_ptr(), err);
        return err;
    }

    err = add_special_controls(bebob);
    if err < 0 {
        return err;
    }

    special_stream_formation_set(bebob);

    if (*params).is1814 {
        (*bebob).midi_input_ports = 1;
        (*bebob).midi_output_ports = 1;
    } else {
        (*bebob).midi_input_ports = 2;
        (*bebob).midi_output_ports = 2;
    }
    err
}

/* Input plug shows actual rate. Output plug is needless for this purpose. */
unsafe extern "C" fn special_get_rate(bebob: *mut snd_bebob, rate: *mut c_uint) -> c_int {
    let mut err: c_int;
    let mut trials: c_int;

    trials = 0;
    loop {
        err = avc_general_get_sig_fmt((*bebob).unit, rate, AVC_GENERAL_PLUG_DIR_IN, 0);
        if !(err == -EAGAIN && {
            trials += 1;
            trials < 3
        }) {
            break;
        }
    }

    err
}

unsafe extern "C" fn special_set_rate(bebob: *mut snd_bebob, rate: c_uint) -> c_int {
    let params = (*bebob).maudio_special_quirk as *mut special_params;
    let mut err: c_int;

    err = avc_general_set_sig_fmt((*bebob).unit, rate, AVC_GENERAL_PLUG_DIR_OUT, 0);
    if err < 0 {
        return err;
    }

    /*
     * Just after changing sampling rate for output, a followed command
     * for input is easy to fail. This is a workaround fot this issue.
     */
    msleep(100);

    err = avc_general_set_sig_fmt((*bebob).unit, rate, AVC_GENERAL_PLUG_DIR_IN, 0);
    if err < 0 {
        return err;
    }

    if !(*params).ctl_id_sync.is_null() {
        snd_ctl_notify((*bebob).card, SNDRV_CTL_EVENT_MASK_VALUE, (*params).ctl_id_sync);
    }
    err
}

/* Clock source control for special firmware */
static SPECIAL_CLK_TYPES: [snd_bebob_clock_type; 4] = [
    SND_BEBOB_CLOCK_TYPE_INTERNAL, /* With digital mute */
    SND_BEBOB_CLOCK_TYPE_EXTERNAL, /* SPDIF/ADAT */
    SND_BEBOB_CLOCK_TYPE_EXTERNAL, /* Word Clock */
    SND_BEBOB_CLOCK_TYPE_INTERNAL,
];

unsafe extern "C" fn special_clk_get(bebob: *mut snd_bebob, id: *mut c_uint) -> c_int {
    let params = (*bebob).maudio_special_quirk as *mut special_params;
    *id = (*params).clk_src;
    0
}

static SPECIAL_CLK_LABELS: [*const i8; 4] = [
    c"Internal with Digital Mute".as_ptr(),
    c"Digital".as_ptr(),
    c"Word Clock".as_ptr(),
    c"Internal".as_ptr(),
];

unsafe extern "C" fn special_clk_ctl_info(_kctl: *mut snd_kcontrol, einf: *mut snd_ctl_elem_info) -> c_int {
    snd_ctl_enum_info(einf, 1, SPECIAL_CLK_TYPES.len() as c_uint, SPECIAL_CLK_LABELS.as_ptr())
}

unsafe extern "C" fn special_clk_ctl_get(kctl: *mut snd_kcontrol, uval: *mut snd_ctl_elem_value) -> c_int {
    let bebob = snd_kcontrol_chip(kctl);
    let params = (*bebob).maudio_special_quirk as *mut special_params;
    (*uval).value.enumerated.item[0] = (*params).clk_src;
    0
}

unsafe extern "C" fn special_clk_ctl_put(kctl: *mut snd_kcontrol, uval: *mut snd_ctl_elem_value) -> c_int {
    let bebob = snd_kcontrol_chip(kctl);
    let params = (*bebob).maudio_special_quirk as *mut special_params;
    let mut err: c_int;
    let id: c_int;

    id = (*uval).value.enumerated.item[0] as c_int;
    if id as usize >= SPECIAL_CLK_TYPES.len() {
        return -EINVAL;
    }

    // C source uses guard(mutex)(&bebob->mutex).
    err = avc_maudio_set_special_clk(bebob, id as c_uint, (*params).dig_in_fmt, (*params).dig_out_fmt, (*params).clk_lock);
    if err >= 0 {
        err = 1;
    }

    err
}

static SPECIAL_CLK_CTL: snd_kcontrol_new = snd_kcontrol_new {
    name: c"Clock Source".as_ptr(),
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    info: Some(special_clk_ctl_info),
    get: Some(special_clk_ctl_get),
    put: Some(special_clk_ctl_put),
};

/* Clock synchronization control for special firmware */
unsafe extern "C" fn special_sync_ctl_info(_kctl: *mut snd_kcontrol, einf: *mut snd_ctl_elem_info) -> c_int {
    (*einf).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    (*einf).count = 1;
    (*einf).value.integer.min = 0;
    (*einf).value.integer.max = 1;

    0
}

unsafe extern "C" fn special_sync_ctl_get(kctl: *mut snd_kcontrol, uval: *mut snd_ctl_elem_value) -> c_int {
    let bebob = snd_kcontrol_chip(kctl);
    let err: c_int;
    let mut synced: Bool = false;

    err = check_clk_sync(bebob, METER_SIZE_SPECIAL, &mut synced);
    if err >= 0 {
        (*uval).value.integer.value[0] = synced as i64;
    }

    0
}

static SPECIAL_SYNC_CTL: snd_kcontrol_new = snd_kcontrol_new {
    name: c"Sync Status".as_ptr(),
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    access: SNDRV_CTL_ELEM_ACCESS_READ,
    info: Some(special_sync_ctl_info),
    get: Some(special_sync_ctl_get),
    put: None,
};

/* Digital input interface control for special firmware */
static SPECIAL_DIG_IN_IFACE_LABELS: [*const i8; 3] = [
    c"S/PDIF Optical".as_ptr(),
    c"S/PDIF Coaxial".as_ptr(),
    c"ADAT Optical".as_ptr(),
];

unsafe extern "C" fn special_dig_in_iface_ctl_info(_kctl: *mut snd_kcontrol, einf: *mut snd_ctl_elem_info) -> c_int {
    snd_ctl_enum_info(einf, 1, SPECIAL_DIG_IN_IFACE_LABELS.len() as c_uint, SPECIAL_DIG_IN_IFACE_LABELS.as_ptr())
}

unsafe extern "C" fn special_dig_in_iface_ctl_get(kctl: *mut snd_kcontrol, uval: *mut snd_ctl_elem_value) -> c_int {
    let bebob = snd_kcontrol_chip(kctl);
    let params = (*bebob).maudio_special_quirk as *mut special_params;
    let mut dig_in_iface: c_uint = 0;
    let err: c_int;
    let mut val: c_int;

    // C source uses guard(mutex)(&bebob->mutex).
    err = avc_audio_get_selector((*bebob).unit, 0x00, 0x04, &mut dig_in_iface);
    if err < 0 {
        dev_err_i(&mut (*(*bebob).unit).device, c"fail to get digital input interface: %d\n".as_ptr(), err);
        return err;
    }

    /* encoded id for user value */
    val = (((*params).dig_in_fmt << 1) | (dig_in_iface & 0x01)) as c_int;

    /* for ADAT Optical */
    if val > 2 {
        val = 2;
    }

    (*uval).value.enumerated.item[0] = val as c_uint;
    0
}

unsafe extern "C" fn special_dig_in_iface_ctl_set(kctl: *mut snd_kcontrol, uval: *mut snd_ctl_elem_value) -> c_int {
    let bebob = snd_kcontrol_chip(kctl);
    let params = (*bebob).maudio_special_quirk as *mut special_params;
    let id: c_uint;
    let dig_in_fmt: c_uint;
    let dig_in_iface: c_uint;
    let mut err: c_int;

    id = (*uval).value.enumerated.item[0];
    if id as usize >= SPECIAL_DIG_IN_IFACE_LABELS.len() {
        return -EINVAL;
    }

    /* decode user value */
    dig_in_fmt = (id >> 1) & 0x01;
    dig_in_iface = id & 0x01;

    // C source uses guard(mutex)(&bebob->mutex).
    err = avc_maudio_set_special_clk(bebob, (*params).clk_src, dig_in_fmt, (*params).dig_out_fmt, (*params).clk_lock);
    if err < 0 {
        return err;
    }

    /* For ADAT, optical interface is only available. */
    if (*params).dig_in_fmt > 0 {
        return 1;
    }

    /* For S/PDIF, optical/coaxial interfaces are selectable. */
    err = avc_audio_set_selector((*bebob).unit, 0x00, 0x04, dig_in_iface);
    if err < 0 {
        dev_err_i(&mut (*(*bebob).unit).device, c"fail to set digital input interface: %d\n".as_ptr(), err);
    }
    special_stream_formation_set(bebob);
    1
}

static SPECIAL_DIG_IN_IFACE_CTL: snd_kcontrol_new = snd_kcontrol_new {
    name: c"Digital Input Interface".as_ptr(),
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    info: Some(special_dig_in_iface_ctl_info),
    get: Some(special_dig_in_iface_ctl_get),
    put: Some(special_dig_in_iface_ctl_set),
};

/* Digital output interface control for special firmware */
static SPECIAL_DIG_OUT_IFACE_LABELS: [*const i8; 2] = [
    c"S/PDIF Optical and Coaxial".as_ptr(),
    c"ADAT Optical".as_ptr(),
];

unsafe extern "C" fn special_dig_out_iface_ctl_info(_kctl: *mut snd_kcontrol, einf: *mut snd_ctl_elem_info) -> c_int {
    snd_ctl_enum_info(einf, 1, SPECIAL_DIG_OUT_IFACE_LABELS.len() as c_uint, SPECIAL_DIG_OUT_IFACE_LABELS.as_ptr())
}

unsafe extern "C" fn special_dig_out_iface_ctl_get(kctl: *mut snd_kcontrol, uval: *mut snd_ctl_elem_value) -> c_int {
    let bebob = snd_kcontrol_chip(kctl);
    let params = (*bebob).maudio_special_quirk as *mut special_params;

    // C source uses guard(mutex)(&bebob->mutex).
    (*uval).value.enumerated.item[0] = (*params).dig_out_fmt;
    0
}

unsafe extern "C" fn special_dig_out_iface_ctl_set(kctl: *mut snd_kcontrol, uval: *mut snd_ctl_elem_value) -> c_int {
    let bebob = snd_kcontrol_chip(kctl);
    let params = (*bebob).maudio_special_quirk as *mut special_params;
    let id: c_uint;
    let mut err: c_int;

    id = (*uval).value.enumerated.item[0];
    if id as usize >= SPECIAL_DIG_OUT_IFACE_LABELS.len() {
        return -EINVAL;
    }

    // C source uses guard(mutex)(&bebob->mutex).
    err = avc_maudio_set_special_clk(bebob, (*params).clk_src, (*params).dig_in_fmt, id, (*params).clk_lock);
    if err >= 0 {
        special_stream_formation_set(bebob);
        err = 1;
    }

    err
}

static SPECIAL_DIG_OUT_IFACE_CTL: snd_kcontrol_new = snd_kcontrol_new {
    name: c"Digital Output Interface".as_ptr(),
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    info: Some(special_dig_out_iface_ctl_info),
    get: Some(special_dig_out_iface_ctl_get),
    put: Some(special_dig_out_iface_ctl_set),
};

/* Hardware metering for special firmware */
static SPECIAL_METER_LABELS: [*const i8; 19] = [
    ANA_IN, ANA_IN, ANA_IN, ANA_IN,
    SPDIF_IN,
    ADAT_IN, ADAT_IN, ADAT_IN, ADAT_IN,
    ANA_OUT, ANA_OUT,
    SPDIF_OUT,
    ADAT_OUT, ADAT_OUT, ADAT_OUT, ADAT_OUT,
    HP_OUT, HP_OUT,
    AUX_OUT,
];

unsafe extern "C" fn special_meter_get(bebob: *mut snd_bebob, target: *mut U32, size: c_uint) -> c_int {
    let buf: *mut Be16;
    let mut i: c_uint;
    let mut c: c_uint;
    let channels: c_uint;
    let err: c_int;

    channels = (SPECIAL_METER_LABELS.len() * 2) as c_uint;
    if size < channels * core::mem::size_of::<U32>() as c_uint {
        return -EINVAL;
    }

    /* omit last 4 bytes because it's clock info. */
    buf = kmalloc(METER_SIZE_SPECIAL - 4, GFP_KERNEL) as *mut Be16;
    if buf.is_null() {
        return -ENOMEM;
    }

    err = get_meter(bebob, buf as *mut c_void, METER_SIZE_SPECIAL - 4);
    if err < 0 {
        kfree(buf as *mut c_void);
        return err;
    }

    /* Its format is u16 and some channels are unknown. */
    i = 0;
    c = 2;
    while c < channels + 2 {
        *target.add(i as usize) = (be16_to_cpu(*buf.add(c as usize)) as U32) << 16;
        i += 1;
        c += 1;
    }
    kfree(buf as *mut c_void);
    err
}

/* last 4 bytes are omitted because it's clock info. */
static FW410_METER_LABELS: [*const i8; 8] = [
    ANA_IN, DIG_IN,
    ANA_OUT, ANA_OUT, ANA_OUT, ANA_OUT, DIG_OUT,
    HP_OUT,
];

static AUDIOPHILE_METER_LABELS: [*const i8; 7] = [
    ANA_IN, DIG_IN,
    ANA_OUT, ANA_OUT, DIG_OUT,
    HP_OUT, AUX_OUT,
];

static SOLO_METER_LABELS: [*const i8; 6] = [
    ANA_IN, DIG_IN,
    STRM_IN, STRM_IN,
    ANA_OUT, DIG_OUT,
];

/* no clock info */
static OZONIC_METER_LABELS: [*const i8; 6] = [
    ANA_IN, ANA_IN,
    STRM_IN, STRM_IN,
    ANA_OUT, ANA_OUT,
];

/* TODO: need testers. these positions are based on authour's assumption */
static NRV10_METER_LABELS: [*const i8; 10] = [
    ANA_IN, ANA_IN, ANA_IN, ANA_IN,
    DIG_IN,
    ANA_OUT, ANA_OUT, ANA_OUT, ANA_OUT,
    DIG_IN,
];

unsafe extern "C" fn normal_meter_get(bebob: *mut snd_bebob, buf: *mut U32, size: c_uint) -> c_int {
    let spec = (*(*bebob).spec).meter;
    let mut c: c_uint;
    let channels: c_uint;
    let err: c_int;

    channels = (*spec).num * 2;
    if size < channels * core::mem::size_of::<U32>() as c_uint {
        return -EINVAL;
    }

    err = get_meter(bebob, buf as *mut c_void, size);
    if err < 0 {
        return err;
    }

    c = 0;
    while c < channels {
        be32_to_cpus(buf.add(c as usize));
        c += 1;
    }

    /* swap stream channels because inverted */
    if (*spec).labels == SOLO_METER_LABELS.as_ptr() {
        core::ptr::swap(buf.add(4), buf.add(6));
        core::ptr::swap(buf.add(5), buf.add(7));
    }
    err
}

/* for special customized devices */
static SPECIAL_RATE_SPEC: snd_bebob_rate_spec = snd_bebob_rate_spec {
    get: Some(special_get_rate),
    set: Some(special_set_rate),
};

static SPECIAL_CLK_SPEC: snd_bebob_clock_spec = snd_bebob_clock_spec {
    num: SPECIAL_CLK_TYPES.len() as c_uint,
    types: SPECIAL_CLK_TYPES.as_ptr(),
    get: Some(special_clk_get),
};

static SPECIAL_METER_SPEC: snd_bebob_meter_spec = snd_bebob_meter_spec {
    num: SPECIAL_METER_LABELS.len() as c_uint,
    labels: SPECIAL_METER_LABELS.as_ptr(),
    get: Some(special_meter_get),
};

#[no_mangle]
pub static MAUDIO_SPECIAL_SPEC: snd_bebob_spec = snd_bebob_spec {
    clock: &SPECIAL_CLK_SPEC,
    rate: &SPECIAL_RATE_SPEC,
    meter: &SPECIAL_METER_SPEC,
};

/* Firewire 410 specification */
static USUAL_RATE_SPEC: snd_bebob_rate_spec = snd_bebob_rate_spec {
    get: Some(snd_bebob_stream_get_rate),
    set: Some(snd_bebob_stream_set_rate),
};

static FW410_METER_SPEC: snd_bebob_meter_spec = snd_bebob_meter_spec {
    num: FW410_METER_LABELS.len() as c_uint,
    labels: FW410_METER_LABELS.as_ptr(),
    get: Some(normal_meter_get),
};

#[no_mangle]
pub static MAUDIO_FW410_SPEC: snd_bebob_spec = snd_bebob_spec {
    clock: core::ptr::null(),
    rate: &USUAL_RATE_SPEC,
    meter: &FW410_METER_SPEC,
};

/* Firewire Audiophile specification */
static AUDIOPHILE_METER_SPEC: snd_bebob_meter_spec = snd_bebob_meter_spec {
    num: AUDIOPHILE_METER_LABELS.len() as c_uint,
    labels: AUDIOPHILE_METER_LABELS.as_ptr(),
    get: Some(normal_meter_get),
};

#[no_mangle]
pub static MAUDIO_AUDIOPHILE_SPEC: snd_bebob_spec = snd_bebob_spec {
    clock: core::ptr::null(),
    rate: &USUAL_RATE_SPEC,
    meter: &AUDIOPHILE_METER_SPEC,
};

/* Firewire Solo specification */
static SOLO_METER_SPEC: snd_bebob_meter_spec = snd_bebob_meter_spec {
    num: SOLO_METER_LABELS.len() as c_uint,
    labels: SOLO_METER_LABELS.as_ptr(),
    get: Some(normal_meter_get),
};

#[no_mangle]
pub static MAUDIO_SOLO_SPEC: snd_bebob_spec = snd_bebob_spec {
    clock: core::ptr::null(),
    rate: &USUAL_RATE_SPEC,
    meter: &SOLO_METER_SPEC,
};

/* Ozonic specification */
static OZONIC_METER_SPEC: snd_bebob_meter_spec = snd_bebob_meter_spec {
    num: OZONIC_METER_LABELS.len() as c_uint,
    labels: OZONIC_METER_LABELS.as_ptr(),
    get: Some(normal_meter_get),
};

#[no_mangle]
pub static MAUDIO_OZONIC_SPEC: snd_bebob_spec = snd_bebob_spec {
    clock: core::ptr::null(),
    rate: &USUAL_RATE_SPEC,
    meter: &OZONIC_METER_SPEC,
};

/* NRV10 specification */
static NRV10_METER_SPEC: snd_bebob_meter_spec = snd_bebob_meter_spec {
    num: NRV10_METER_LABELS.len() as c_uint,
    labels: NRV10_METER_LABELS.as_ptr(),
    get: Some(normal_meter_get),
};

#[no_mangle]
pub static MAUDIO_NRV10_SPEC: snd_bebob_spec = snd_bebob_spec {
    clock: core::ptr::null(),
    rate: &USUAL_RATE_SPEC,
    meter: &NRV10_METER_SPEC,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
