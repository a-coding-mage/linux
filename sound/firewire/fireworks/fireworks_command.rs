// SPDX-License-Identifier: GPL-2.0-only
/*
 * fireworks_command.c - a part of driver for Fireworks based devices
 *
 * Copyright (c) 2013-2014 Takashi Sakamoto
 */

// Translated from ./fireworks.h dependencies.
type c_char = i8;
type c_int = i32;
type c_uint = u32;
type c_void = core::ffi::c_void;
type u16 = u16;
type u32 = u32;
type __be32 = u32;

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const ENOSYS: c_int = 38;
const EIO: c_int = 5;
const EINVAL: c_int = 22;
const UINT_MAX: c_uint = c_uint::MAX;
const HWINFO_NAME_SIZE_BYTES: usize = 32;
const SND_EFW_TRANSACTION_USER_SEQNUM_MAX: u32 = 0x7fffffff;

/*
 * This driver uses transaction version 1 or later to use extended hardware
 * information. Then too old devices are not available.
 *
 * Each commands are not required to have continuous sequence numbers. This
 * number is just used to match command and response.
 *
 * This module support a part of commands. Please see FFADO if you want to see
 * whole commands. But there are some commands which FFADO don't implement.
 *
 * Fireworks also supports AV/C general commands and AV/C Stream Format
 * Information commands. But this module don't use them.
 */

const KERNEL_SEQNUM_MIN: u32 = SND_EFW_TRANSACTION_USER_SEQNUM_MAX + 2;
const KERNEL_SEQNUM_MAX: u32 = !0u32;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct unit {
    pub device: device,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_efw {
    pub lock: spinlock_t,
    pub seqnum: u32,
    pub unit: *mut unit,
    pub resp_addr_changable: bool,
}

#[repr(C)]
pub struct snd_efw_transaction {
    pub length: __be32,
    pub version: __be32,
    pub seqnum: __be32,
    pub category: __be32,
    pub command: __be32,
    pub status: __be32,
    pub params: [__be32; 0],
}

#[repr(C)]
pub struct snd_efw_hwinfo {
    pub flags: u32,
    pub guid_hi: u32,
    pub guid_lo: u32,
    pub type_: u32,
    pub version: u32,
    pub vendor_name: [c_char; HWINFO_NAME_SIZE_BYTES],
    pub model_name: [c_char; HWINFO_NAME_SIZE_BYTES],
    pub supported_clocks: u32,
    pub amdtp_rx_pcm_channels: u32,
    pub amdtp_tx_pcm_channels: u32,
    pub phys_out: u32,
    pub phys_in: u32,
    pub phys_out_grp_count: u32,
    pub phys_in_grp_count: u32,
    pub midi_out_ports: u32,
    pub midi_in_ports: u32,
    pub max_sample_rate: u32,
    pub min_sample_rate: u32,
    pub dsp_version: u32,
    pub arm_version: u32,
    pub mixer_playback_channels: u32,
    pub mixer_capture_channels: u32,
    pub fpga_version: u32,
    pub amdtp_rx_pcm_channels_2x: u32,
    pub amdtp_tx_pcm_channels_2x: u32,
    pub amdtp_rx_pcm_channels_4x: u32,
    pub amdtp_tx_pcm_channels_4x: u32,
}

#[repr(C)]
pub struct snd_efw_phys_meters {
    _private: [u8; 0],
}

#[repr(C)]
pub enum snd_efw_transport_mode {
    _SND_EFW_TRANSPORT_MODE_PLACEHOLDER = 0,
}

#[repr(C)]
pub enum snd_efw_clock_source {
    _SND_EFW_CLOCK_SOURCE_PLACEHOLDER = 0,
}

/* for clock source and sampling rate */
#[repr(C)]
struct efc_clock {
    source: u32,
    sampling_rate: u32,
    index: u32,
}

/* command categories */
const EFC_CAT_HWINFO: c_uint = 0;
const EFC_CAT_TRANSPORT: c_uint = 2;
const EFC_CAT_HWCTL: c_uint = 3;

/* hardware info category commands */
const EFC_CMD_HWINFO_GET_CAPS: c_uint = 0;
const EFC_CMD_HWINFO_GET_POLLED: c_uint = 1;
const EFC_CMD_HWINFO_SET_RESP_ADDR: c_uint = 2;

const EFC_CMD_TRANSPORT_SET_TX_MODE: c_uint = 0;

/* hardware control category commands */
const EFC_CMD_HWCTL_SET_CLOCK: c_uint = 0;
const EFC_CMD_HWCTL_GET_CLOCK: c_uint = 1;
const EFC_CMD_HWCTL_IDENTIFY: c_uint = 5;

/* return values in response */
const EFR_STATUS_OK: u32 = 0;
const EFR_STATUS_BAD: u32 = 1;
const EFR_STATUS_BAD_COMMAND: u32 = 2;
const EFR_STATUS_COMM_ERR: u32 = 3;
const EFR_STATUS_BAD_QUAD_COUNT: u32 = 4;
const EFR_STATUS_UNSUPPORTED: u32 = 5;
const EFR_STATUS_1394_TIMEOUT: u32 = 6;
const EFR_STATUS_DSP_TIMEOUT: u32 = 7;
const EFR_STATUS_BAD_RATE: u32 = 8;
const EFR_STATUS_BAD_CLOCK: u32 = 9;
const EFR_STATUS_BAD_CHANNEL: u32 = 10;
const EFR_STATUS_BAD_PAN: u32 = 11;
const EFR_STATUS_FLASH_BUSY: u32 = 12;
const EFR_STATUS_BAD_MIRROR: u32 = 13;
const EFR_STATUS_BAD_LED: u32 = 14;
const EFR_STATUS_BAD_PARAMETER: u32 = 15;
const EFR_STATUS_INCOMPLETE: u32 = 0x80000000;

static EFR_STATUS_NAMES: [&[u8]; 17] = [
    b"OK\0",
    b"bad\0",
    b"bad command\0",
    b"comm err\0",
    b"bad quad count\0",
    b"unsupported\0",
    b"1394 timeout\0",
    b"DSP timeout\0",
    b"bad rate\0",
    b"bad clock\0",
    b"bad channel\0",
    b"bad pan\0",
    b"flash busy\0",
    b"bad mirror\0",
    b"bad LED\0",
    b"bad parameter\0",
    b"incomplete\0",
];

extern "C" {
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn cpu_to_be32(val: u32) -> __be32;
    fn be32_to_cpu(val: __be32) -> u32;
    fn cpu_to_be32s(ptr: *mut u32);
    fn be32_to_cpus(ptr: *mut u32);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn snd_efw_transaction_run(
        unit: *mut unit,
        cmd: *mut __be32,
        cmd_bytes: c_uint,
        resp: *mut __be32,
        resp_bytes: c_uint,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn msleep(msecs: c_uint);
}

unsafe fn max_unsigned_int(a: c_uint, b: c_uint) -> c_uint {
    if a > b { a } else { b }
}

unsafe fn min_unsigned_int(a: c_uint, b: c_uint) -> c_uint {
    if a < b { a } else { b }
}

unsafe fn efw_transaction(
    efw: *mut snd_efw,
    category: c_uint,
    command: c_uint,
    params: *const __be32,
    param_bytes: c_uint,
    resp: *const __be32,
    mut resp_bytes: c_uint,
) -> c_int {
    let mut header: *mut snd_efw_transaction;
    let buf: *mut __be32;
    let seqnum: u32;
    let buf_bytes: c_uint;
    let cmd_bytes: c_uint;
    let mut err: c_int;

    /* calculate buffer size*/
    buf_bytes = core::mem::size_of::<snd_efw_transaction>() as c_uint
        + max_unsigned_int(param_bytes, resp_bytes);

    /* keep buffer */
    buf = kzalloc(buf_bytes as usize, GFP_KERNEL) as *mut __be32;
    if buf.is_null() {
        return -ENOMEM;
    }

    /* to keep consistency of sequence number */
    spin_lock(core::ptr::addr_of_mut!((*efw).lock));
    if (*efw).seqnum < KERNEL_SEQNUM_MIN || (*efw).seqnum >= KERNEL_SEQNUM_MAX - 2 {
        (*efw).seqnum = KERNEL_SEQNUM_MIN;
    } else {
        (*efw).seqnum = (*efw).seqnum.wrapping_add(2);
    }
    seqnum = (*efw).seqnum;
    spin_unlock(core::ptr::addr_of_mut!((*efw).lock));

    /* fill transaction header fields */
    cmd_bytes = core::mem::size_of::<snd_efw_transaction>() as c_uint + param_bytes;
    header = buf as *mut snd_efw_transaction;
    (*header).length = cpu_to_be32(cmd_bytes / core::mem::size_of::<__be32>() as c_uint);
    (*header).version = cpu_to_be32(1);
    (*header).seqnum = cpu_to_be32(seqnum);
    (*header).category = cpu_to_be32(category);
    (*header).command = cpu_to_be32(command);
    (*header).status = 0;

    /* fill transaction command parameters */
    memcpy(
        (*header).params.as_mut_ptr() as *mut c_void,
        params as *const c_void,
        param_bytes as usize,
    );

    err = snd_efw_transaction_run((*efw).unit, buf, cmd_bytes, buf, buf_bytes);
    if err < 0 {
        kfree(buf as *mut c_void);
        return err;
    }

    /* check transaction header fields */
    if be32_to_cpu((*header).version) < 1
        || be32_to_cpu((*header).category) != category
        || be32_to_cpu((*header).command) != command
        || be32_to_cpu((*header).status) != EFR_STATUS_OK
    {
        let st: u32 = be32_to_cpu((*header).status);
        let status_name = if (st as usize) < EFR_STATUS_NAMES.len() {
            EFR_STATUS_NAMES[st as usize].as_ptr() as *const c_char
        } else {
            b"unknown\0".as_ptr() as *const c_char
        };

        dev_err(
            core::ptr::addr_of_mut!((*(*efw).unit).device),
            b"EFW command failed [%u/%u]: %s\n\0".as_ptr() as *const c_char,
            be32_to_cpu((*header).category),
            be32_to_cpu((*header).command),
            status_name,
        );
        err = -EIO;
        kfree(buf as *mut c_void);
        return err;
    }

    if resp.is_null() {
        kfree(buf as *mut c_void);
        return err;
    }

    /* fill transaction response parameters */
    memset(resp as *mut c_void, 0, resp_bytes as usize);
    resp_bytes = min_unsigned_int(
        resp_bytes,
        be32_to_cpu((*header).length) * core::mem::size_of::<__be32>() as c_uint
            - core::mem::size_of::<snd_efw_transaction>() as c_uint,
    );
    memcpy(
        resp as *mut c_void,
        buf.add(6) as *const c_void,
        resp_bytes as usize,
    );

    kfree(buf as *mut c_void);
    err
}

/*
 * The address in host system for transaction response is changable when the
 * device supports. struct hwinfo.flags includes its flag. The default is
 * MEMORY_SPACE_EFW_RESPONSE.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_efw_command_set_resp_addr(
    efw: *mut snd_efw,
    addr_high: u16,
    addr_low: u32,
) -> c_int {
    let mut addr: [__be32; 2] = [0; 2];

    addr[0] = cpu_to_be32(addr_high as u32);
    addr[1] = cpu_to_be32(addr_low);

    if !(*efw).resp_addr_changable {
        return -ENOSYS;
    }

    efw_transaction(
        efw,
        EFC_CAT_HWCTL,
        EFC_CMD_HWINFO_SET_RESP_ADDR,
        addr.as_ptr(),
        core::mem::size_of_val(&addr) as c_uint,
        core::ptr::null(),
        0,
    )
}

/*
 * This is for timestamp processing. In Windows mode, all 32bit fields of second
 * CIP header in AMDTP transmit packet is used for 'presentation timestamp'. In
 * 'no data' packet the value of this field is 0x90ffffff.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_efw_command_set_tx_mode(
    efw: *mut snd_efw,
    mode: snd_efw_transport_mode,
) -> c_int {
    let param: __be32 = cpu_to_be32(mode as u32);
    efw_transaction(
        efw,
        EFC_CAT_TRANSPORT,
        EFC_CMD_TRANSPORT_SET_TX_MODE,
        core::ptr::addr_of!(param),
        core::mem::size_of_val(&param) as c_uint,
        core::ptr::null(),
        0,
    )
}

#[no_mangle]
pub unsafe extern "C" fn snd_efw_command_get_hwinfo(
    efw: *mut snd_efw,
    hwinfo: *mut snd_efw_hwinfo,
) -> c_int {
    let mut err: c_int;

    err = efw_transaction(
        efw,
        EFC_CAT_HWINFO,
        EFC_CMD_HWINFO_GET_CAPS,
        core::ptr::null(),
        0,
        hwinfo as *const __be32,
        core::mem::size_of::<snd_efw_hwinfo>() as c_uint,
    );
    if err < 0 {
        return err;
    }

    be32_to_cpus(core::ptr::addr_of_mut!((*hwinfo).flags));
    be32_to_cpus(core::ptr::addr_of_mut!((*hwinfo).guid_hi));
    be32_to_cpus(core::ptr::addr_of_mut!((*hwinfo).guid_lo));
    be32_to_cpus(core::ptr::addr_of_mut!((*hwinfo).type_));
    be32_to_cpus(core::ptr::addr_of_mut!((*hwinfo).version));
    be32_to_cpus(core::ptr::addr_of_mut!((*hwinfo).supported_clocks));
    be32_to_cpus(core::ptr::addr_of_mut!((*hwinfo).amdtp_rx_pcm_channels));
    be32_to_cpus(core::ptr::addr_of_mut!((*hwinfo).amdtp_tx_pcm_channels));
    be32_to_cpus(core::ptr::addr_of_mut!((*hwinfo).phys_out));
    be32_to_cpus(core::ptr::addr_of_mut!((*hwinfo).phys_in));
    be32_to_cpus(core::ptr::addr_of_mut!((*hwinfo).phys_out_grp_count));
    be32_to_cpus(core::ptr::addr_of_mut!((*hwinfo).phys_in_grp_count));
    be32_to_cpus(core::ptr::addr_of_mut!((*hwinfo).midi_out_ports));
    be32_to_cpus(core::ptr::addr_of_mut!((*hwinfo).midi_in_ports));
    be32_to_cpus(core::ptr::addr_of_mut!((*hwinfo).max_sample_rate));
    be32_to_cpus(core::ptr::addr_of_mut!((*hwinfo).min_sample_rate));
    be32_to_cpus(core::ptr::addr_of_mut!((*hwinfo).dsp_version));
    be32_to_cpus(core::ptr::addr_of_mut!((*hwinfo).arm_version));
    be32_to_cpus(core::ptr::addr_of_mut!((*hwinfo).mixer_playback_channels));
    be32_to_cpus(core::ptr::addr_of_mut!((*hwinfo).mixer_capture_channels));
    be32_to_cpus(core::ptr::addr_of_mut!((*hwinfo).fpga_version));
    be32_to_cpus(core::ptr::addr_of_mut!((*hwinfo).amdtp_rx_pcm_channels_2x));
    be32_to_cpus(core::ptr::addr_of_mut!((*hwinfo).amdtp_tx_pcm_channels_2x));
    be32_to_cpus(core::ptr::addr_of_mut!((*hwinfo).amdtp_rx_pcm_channels_4x));
    be32_to_cpus(core::ptr::addr_of_mut!((*hwinfo).amdtp_tx_pcm_channels_4x));

    /* ensure terminated */
    (*hwinfo).vendor_name[HWINFO_NAME_SIZE_BYTES - 1] = 0;
    (*hwinfo).model_name[HWINFO_NAME_SIZE_BYTES - 1] = 0;

    err
}

#[no_mangle]
pub unsafe extern "C" fn snd_efw_command_get_phys_meters(
    efw: *mut snd_efw,
    meters: *mut snd_efw_phys_meters,
    len: c_uint,
) -> c_int {
    let buf: *mut u32 = meters as *mut u32;
    let mut i: c_uint;
    let err: c_int;

    err = efw_transaction(
        efw,
        EFC_CAT_HWINFO,
        EFC_CMD_HWINFO_GET_POLLED,
        core::ptr::null(),
        0,
        meters as *const __be32,
        len,
    );
    if err >= 0 {
        i = 0;
        while i < len / core::mem::size_of::<u32>() as c_uint {
            be32_to_cpus(buf.add(i as usize));
            i += 1;
        }
    }

    err
}

unsafe fn command_get_clock(efw: *mut snd_efw, clock: *mut efc_clock) -> c_int {
    let err: c_int;

    err = efw_transaction(
        efw,
        EFC_CAT_HWCTL,
        EFC_CMD_HWCTL_GET_CLOCK,
        core::ptr::null(),
        0,
        clock as *const __be32,
        core::mem::size_of::<efc_clock>() as c_uint,
    );
    if err >= 0 {
        be32_to_cpus(core::ptr::addr_of_mut!((*clock).source));
        be32_to_cpus(core::ptr::addr_of_mut!((*clock).sampling_rate));
        be32_to_cpus(core::ptr::addr_of_mut!((*clock).index));
    }

    err
}

/* give UINT_MAX if set nothing */
unsafe fn command_set_clock(efw: *mut snd_efw, source: c_uint, rate: c_uint) -> c_int {
    let mut clock = efc_clock {
        source: 0,
        sampling_rate: 0,
        index: 0,
    };
    let mut err: c_int;

    /* check arguments */
    if source == UINT_MAX && rate == UINT_MAX {
        err = -EINVAL;
        return err;
    }

    /* get current status */
    err = command_get_clock(efw, core::ptr::addr_of_mut!(clock));
    if err < 0 {
        return err;
    }

    /* no need */
    if clock.source == source && clock.sampling_rate == rate {
        return err;
    }

    /* set params */
    if source != UINT_MAX && clock.source != source {
        clock.source = source;
    }
    if rate != UINT_MAX && clock.sampling_rate != rate {
        clock.sampling_rate = rate;
    }
    clock.index = 0;

    cpu_to_be32s(core::ptr::addr_of_mut!(clock.source));
    cpu_to_be32s(core::ptr::addr_of_mut!(clock.sampling_rate));
    cpu_to_be32s(core::ptr::addr_of_mut!(clock.index));

    err = efw_transaction(
        efw,
        EFC_CAT_HWCTL,
        EFC_CMD_HWCTL_SET_CLOCK,
        core::ptr::addr_of!(clock) as *const __be32,
        core::mem::size_of::<efc_clock>() as c_uint,
        core::ptr::null(),
        0,
    );
    if err < 0 {
        return err;
    }

    /*
     * With firmware version 5.8, just after changing clock state, these
     * parameters are not immediately retrieved by get command. In my
     * trial, there needs to be 100msec to get changed parameters.
     */
    msleep(150);
    err
}

#[no_mangle]
pub unsafe extern "C" fn snd_efw_command_get_clock_source(
    efw: *mut snd_efw,
    source: *mut snd_efw_clock_source,
) -> c_int {
    let err: c_int;
    let mut clock = efc_clock {
        source: 0,
        sampling_rate: 0,
        index: 0,
    };

    err = command_get_clock(efw, core::ptr::addr_of_mut!(clock));
    if err >= 0 {
        *(source as *mut u32) = clock.source;
    }

    err
}

#[no_mangle]
pub unsafe extern "C" fn snd_efw_command_get_sampling_rate(
    efw: *mut snd_efw,
    rate: *mut c_uint,
) -> c_int {
    let err: c_int;
    let mut clock = efc_clock {
        source: 0,
        sampling_rate: 0,
        index: 0,
    };

    err = command_get_clock(efw, core::ptr::addr_of_mut!(clock));
    if err >= 0 {
        *rate = clock.sampling_rate;
    }

    err
}

#[no_mangle]
pub unsafe extern "C" fn snd_efw_command_set_sampling_rate(
    efw: *mut snd_efw,
    rate: c_uint,
) -> c_int {
    command_set_clock(efw, UINT_MAX, rate)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
