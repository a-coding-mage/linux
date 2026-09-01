// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Routines for control of the CS8427 via i2c bus
 *  IEC958 (S/PDIF) receiver & transmitter by Cirrus Logic
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

// Dependencies from Linux/ALSA headers:
// linux/slab.h, linux/delay.h, linux/init.h, linux/bitrev.h, linux/module.h,
// linux/unaligned.h, sound/core.h, sound/control.h, sound/pcm.h,
// sound/cs8427.h, sound/asoundef.h

const CS8427_ADDR: c_uint = 0x20 >> 1; /* fixed address */
const EIO: c_int = 5;
const EFAULT: c_int = 14;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENXIO: c_int = 6;

#[repr(C)]
pub struct cs8427_stream {
    pub substream: *mut snd_pcm_substream,
    pub hw_status: [c_char; 24], /* hardware status */
    pub def_status: [c_char; 24], /* default status */
    pub pcm_status: [c_char; 24], /* PCM private status */
    pub hw_udata: [c_char; 32],
    pub pcm_ctl: *mut snd_kcontrol,
}

#[repr(C)]
pub struct cs8427 {
    pub regmap: [u8; 0x14], /* map of first 1 + 13 registers */
    pub rate: c_uint,
    pub reset_timeout: c_uint,
    pub playback: cs8427_stream,
    pub capture: cs8427_stream,
}

#[repr(C)]
pub struct snd_i2c_device {
    pub bus: *mut snd_i2c_bus,
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_i2c_device)>,
}

#[repr(C)]
pub struct snd_i2c_bus {
    pub card: *mut snd_card,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut c_void,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub pcm: *mut snd_pcm,
    pub number: c_uint,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm {
    pub device: c_int,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub id: snd_ctl_elem_id,
    pub vd: *mut snd_kcontrol_volatile,
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_kcontrol_volatile {
    pub access: c_uint,
}

#[repr(C)]
pub struct snd_ctl_elem_id {
    pub device: c_uint,
    pub subdevice: c_uint,
    pub name: [c_char; 44],
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_integer {
    pub min: i64,
    pub max: i64,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_data,
}

#[repr(C)]
pub union snd_ctl_elem_value_data {
    pub integer: snd_ctl_elem_value_integer,
    pub bytes: snd_ctl_elem_value_bytes,
    pub iec958: snd_aes_iec958,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_bytes {
    pub data: [u8; 512],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_aes_iec958 {
    pub status: [u8; 24],
    pub subcode: [u8; 147],
    pub pad: u8,
    pub dig_subframe: [u8; 4],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub name: *const c_char,
    pub access: c_uint,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
}

unsafe extern "C" {
    static mut jiffies: c_ulong;

    static CS8427_BSEL: u8;
    static CS8427_REG_CSDATABUF: usize;
    static CS8427_REG_UDATABUF: usize;
    static CS8427_UBMMASK: u8;
    static CS8427_UBMZEROS: u8;
    static CS8427_EFTUI: u8;
    static CS8427_REG_AUTOINC: u8;
    static CS8427_REG_CORU_DATABUF: u8;
    static CS8427_REG_CONTROL1: u8;
    static CS8427_SWCLK: u8;
    static CS8427_TCBLDIR: u8;
    static CS8427_REG_DATAFLOW: u8;
    static CS8427_TXDSERIAL: u8;
    static CS8427_SPDAES3RECEIVER: u8;
    static CS8427_REG_CLOCKSOURCE: usize;
    static CS8427_RXDILRCK: u8;
    static CS8427_REG_SERIALINPUT: u8;
    static CS8427_SIDEL: u8;
    static CS8427_SILRPOL: u8;
    static CS8427_REG_SERIALOUTPUT: u8;
    static CS8427_SODEL: u8;
    static CS8427_SOLRPOL: u8;
    static CS8427_REG_RECVERRMASK: u8;
    static CS8427_UNLOCK: c_int;
    static CS8427_CBMR: u8;
    static CS8427_DETCI: u8;
    static CS8427_UD: u8;
    static CS8427_DETUI: u8;
    static CS8427_REG_ID_AND_VER: u8;
    static CS8427_VER8427A: c_int;
    static SNDRV_PCM_DEFAULT_CON_SPDIF: u32;
    static CS8427_RUN: u8;
    static CS8427_RXDMASK: u8;
    static CS8427_RXDAES3INPUT: u8;
    static CS8427_REG_RECVERRORS: u8;
    static CS8427_REG_QSUBCODE: u8;
    static SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint;
    static SNDRV_CTL_ELEM_TYPE_BYTES: c_uint;
    static SNDRV_CTL_ELEM_TYPE_IEC958: c_uint;
    static SNDRV_CTL_ELEM_IFACE_PCM: c_uint;
    static SNDRV_CTL_ELEM_ACCESS_READ: c_uint;
    static SNDRV_CTL_ELEM_ACCESS_VOLATILE: c_uint;
    static SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint;
    static SNDRV_CTL_ELEM_ACCESS_INACTIVE: c_uint;
    static SNDRV_CTL_EVENT_MASK_VALUE: c_uint;
    static SNDRV_CTL_EVENT_MASK_INFO: c_uint;
    static IEC958_AES0_PROFESSIONAL: u8;
    static IEC958_AES0_PRO_FS: u8;
    static IEC958_AES0_PRO_FS_32000: u8;
    static IEC958_AES0_PRO_FS_44100: u8;
    static IEC958_AES0_PRO_FS_48000: u8;
    static IEC958_AES0_PRO_FS_NOTID: u8;
    static IEC958_AES3_CON_FS: u8;
    static IEC958_AES3_CON_FS_32000: u8;
    static IEC958_AES3_CON_FS_44100: u8;
    static IEC958_AES3_CON_FS_48000: u8;

    fn snd_i2c_sendbytes(device: *mut snd_i2c_device, bytes: *const u8, count: c_int) -> c_int;
    fn snd_i2c_readbytes(device: *mut snd_i2c_device, bytes: *mut u8, count: c_int) -> c_int;
    fn snd_i2c_lock(bus: *mut snd_i2c_bus);
    fn snd_i2c_unlock(bus: *mut snd_i2c_bus);
    fn snd_i2c_device_create(
        bus: *mut snd_i2c_bus,
        name: *const c_char,
        addr: c_uint,
        rdevice: *mut *mut snd_i2c_device,
    ) -> c_int;
    fn snd_i2c_device_free(device: *mut snd_i2c_device);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn udelay(usecs: c_ulong);
    fn schedule_timeout_uninterruptible(timeout: c_long) -> c_long;
    fn time_after_eq(a: c_ulong, b: c_ulong) -> bool;
    fn snd_BUG_ON(cond: bool) -> bool;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_i2c_device;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut snd_ctl_elem_id);
    fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    fn bitrev8(byte: u8) -> u8;
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut c_void, fmt: *const c_char, ...);
}

type c_long = i64;

const GFP_KERNEL: c_uint = 0;
const CS8427_NAME: &[u8] = b"CS8427\0";
const IEC958_INPUT_STATUS: &[u8] = b"IEC958 CS8427 Input Status\0";
const IEC958_ERROR_STATUS: &[u8] = b"IEC958 CS8427 Error Status\0";
const IEC958_PLAYBACK_MASK: &[u8] = b"IEC958 Playback Mask\0";
const IEC958_PLAYBACK_DEFAULT: &[u8] = b"IEC958 Playback Default\0";
const IEC958_PLAYBACK_PCM_STREAM: &[u8] = b"IEC958 Playback PCM Stream\0";
const IEC958_Q_SUBCODE_CAPTURE_DEFAULT: &[u8] = b"IEC958 Q-subcode Capture Default\0";

unsafe fn memcmp(a: *const c_void, b: *const c_void, count: usize) -> c_int {
    let ap = a as *const u8;
    let bp = b as *const u8;
    let mut idx = 0usize;
    while idx < count {
        let av = unsafe { *ap.add(idx) };
        let bv = unsafe { *bp.add(idx) };
        if av != bv {
            return av as c_int - bv as c_int;
        }
        idx += 1;
    }
    0
}

unsafe fn memcpy(dst: *mut c_void, src: *const c_void, count: usize) -> *mut c_void {
    unsafe { ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, count) };
    dst
}

unsafe fn memset(dst: *mut c_void, val: c_int, count: usize) -> *mut c_void {
    unsafe { ptr::write_bytes(dst as *mut u8, val as u8, count) };
    dst
}

unsafe fn put_unaligned_le32(val: u32, ptr_: *mut u8) {
    let bytes = val.to_le_bytes();
    unsafe { ptr::copy_nonoverlapping(bytes.as_ptr(), ptr_, 4) };
}

unsafe fn snd_ctl_name_iec958_playback_pcm_stream() -> *const c_char {
    IEC958_PLAYBACK_PCM_STREAM.as_ptr() as *const c_char
}

pub unsafe extern "C" fn snd_cs8427_reg_write(
    device: *mut snd_i2c_device,
    reg: u8,
    val: u8,
) -> c_int {
    let mut err: c_int;
    let mut buf = [0u8; 2];

    buf[0] = reg & 0x7f;
    buf[1] = val;
    err = unsafe { snd_i2c_sendbytes(device, buf.as_ptr(), 2) };
    if err != 2 {
        unsafe {
            dev_err(
                (*(*(*device).bus).card).dev,
                b"unable to send bytes 0x%02x:0x%02x to CS8427 (%i)\n\0".as_ptr()
                    as *const c_char,
                buf[0] as c_int,
                buf[1] as c_int,
                err,
            );
        }
        return if err < 0 { err } else { -EIO };
    }
    0
}

unsafe fn snd_cs8427_reg_read(device: *mut snd_i2c_device, reg: u8) -> c_int {
    let mut err: c_int;
    let mut buf = 0u8;
    let mut reg_mut = reg;

    err = unsafe { snd_i2c_sendbytes(device, &mut reg_mut, 1) };
    if err != 1 {
        unsafe {
            dev_err(
                (*(*(*device).bus).card).dev,
                b"unable to send register 0x%x byte to CS8427\n\0".as_ptr() as *const c_char,
                reg as c_int,
            );
        }
        return if err < 0 { err } else { -EIO };
    }
    err = unsafe { snd_i2c_readbytes(device, &mut buf, 1) };
    if err != 1 {
        unsafe {
            dev_err(
                (*(*(*device).bus).card).dev,
                b"unable to read register 0x%x byte from CS8427\n\0".as_ptr() as *const c_char,
                reg as c_int,
            );
        }
        return if err < 0 { err } else { -EIO };
    }
    buf as c_int
}

unsafe fn snd_cs8427_select_corudata(device: *mut snd_i2c_device, mut udata: c_int) -> c_int {
    let chip = unsafe { (*device).private_data as *mut cs8427 };
    let mut err: c_int;

    udata = if udata != 0 { unsafe { CS8427_BSEL as c_int } } else { 0 };
    if udata != (unsafe { (*chip).regmap[CS8427_REG_CSDATABUF] as c_int } & udata) {
        unsafe {
            (*chip).regmap[CS8427_REG_CSDATABUF] &= !CS8427_BSEL;
            (*chip).regmap[CS8427_REG_CSDATABUF] |= udata as u8;
            err = snd_cs8427_reg_write(
                device,
                CS8427_REG_CSDATABUF as u8,
                (*chip).regmap[CS8427_REG_CSDATABUF],
            );
        }
        if err < 0 {
            return err;
        }
    }
    0
}

unsafe fn snd_cs8427_send_corudata(
    device: *mut snd_i2c_device,
    udata: c_int,
    ndata: *mut u8,
    count: c_int,
) -> c_int {
    let chip = unsafe { (*device).private_data as *mut cs8427 };
    let hw_data = unsafe {
        if udata != 0 {
            (*chip).playback.hw_udata.as_mut_ptr()
        } else {
            (*chip).playback.hw_status.as_mut_ptr()
        }
    };
    let mut data = [0u8; 32];
    let mut err: c_int;
    let mut idx: c_int;

    if unsafe { memcmp(hw_data as *const c_void, ndata as *const c_void, count as usize) } == 0 {
        return 0;
    }
    err = unsafe { snd_cs8427_select_corudata(device, udata) };
    if err < 0 {
        return err;
    }
    unsafe { memcpy(hw_data as *mut c_void, ndata as *const c_void, count as usize) };
    if udata != 0 {
        unsafe { memset(data.as_mut_ptr() as *mut c_void, 0, size_of::<[u8; 32]>()) };
        if unsafe { memcmp(hw_data as *const c_void, data.as_ptr() as *const c_void, count as usize) } == 0 {
            unsafe {
                (*chip).regmap[CS8427_REG_UDATABUF] &= !CS8427_UBMMASK;
                (*chip).regmap[CS8427_REG_UDATABUF] |= CS8427_UBMZEROS | CS8427_EFTUI;
                err = snd_cs8427_reg_write(
                    device,
                    CS8427_REG_UDATABUF as u8,
                    (*chip).regmap[CS8427_REG_UDATABUF],
                );
            }
            return if err < 0 { err } else { 0 };
        }
    }
    unsafe {
        data[0] = CS8427_REG_AUTOINC | CS8427_REG_CORU_DATABUF;
    }
    idx = 0;
    while idx < count {
        unsafe {
            data[(idx + 1) as usize] = bitrev8(*ndata.add(idx as usize));
        }
        idx += 1;
    }
    if unsafe { snd_i2c_sendbytes(device, data.as_ptr(), count + 1) } != count + 1 {
        return -EIO;
    }
    1
}

unsafe extern "C" fn snd_cs8427_free(device: *mut snd_i2c_device) {
    unsafe { kfree((*device).private_data) };
}

pub unsafe extern "C" fn snd_cs8427_init(
    bus: *mut snd_i2c_bus,
    device: *mut snd_i2c_device,
) -> c_int {
    let initvals1 = unsafe {
        [
            CS8427_REG_CONTROL1 | CS8427_REG_AUTOINC,
            /* CS8427_REG_CONTROL1: RMCK to OMCK, valid PCM audio, disable mutes,
               TCBL=output */
            CS8427_SWCLK | CS8427_TCBLDIR,
            /* CS8427_REG_CONTROL2: hold last valid audio sample, RMCK=256*Fs,
               normal stereo operation */
            0x00,
            /* CS8427_REG_DATAFLOW: output drivers normal operation, Tx<=serial,
               Rx=>serial */
            CS8427_TXDSERIAL | CS8427_SPDAES3RECEIVER,
            /* CS8427_REG_CLOCKSOURCE: Run off, CMCK=256*Fs,
               output time base = OMCK, input time base = recovered input clock,
               recovered input clock source is ILRCK changed to AES3INPUT
               (workaround, see snd_cs8427_reset) */
            CS8427_RXDILRCK,
            /* CS8427_REG_SERIALINPUT: Serial audio input port data format = I2S,
               24-bit, 64*Fsi */
            CS8427_SIDEL | CS8427_SILRPOL,
            /* CS8427_REG_SERIALOUTPUT: Serial audio output port data format
               = I2S, 24-bit, 64*Fsi */
            CS8427_SODEL | CS8427_SOLRPOL,
        ]
    };
    let initvals2 = unsafe {
        [
            CS8427_REG_RECVERRMASK | CS8427_REG_AUTOINC,
            /* CS8427_REG_RECVERRMASK: unmask the input PLL clock, V, confidence,
               biphase, parity status bits */
            /* CS8427_UNLOCK | CS8427_V | CS8427_CONF | CS8427_BIP | CS8427_PAR,*/
            0xff, /* set everything */
            /* CS8427_REG_CSDATABUF:
               Registers 32-55 window to CS buffer
               Inhibit D->E transfers from overwriting first 5 bytes of CS data.
               Inhibit D->E transfers (all) of CS data.
               Allow E->F transfer of CS data.
               One byte mode; both A/B channels get same written CB data.
               A channel info is output to chip's EMPH* pin. */
            CS8427_CBMR | CS8427_DETCI,
            /* CS8427_REG_UDATABUF:
               Use internal buffer to transmit User (U) data.
               Chip's U pin is an output.
               Transmit all O's for user data.
               Inhibit D->E transfers.
               Inhibit E->F transfers. */
            CS8427_UD | CS8427_EFTUI | CS8427_DETUI,
        ]
    };
    let chip = unsafe { (*device).private_data as *mut cs8427 };
    let mut err: c_int;
    let mut buf = [0u8; 24];

    unsafe { snd_i2c_lock(bus) };
    err = unsafe { snd_cs8427_reg_read(device, CS8427_REG_ID_AND_VER) };
    if err != unsafe { CS8427_VER8427A } {
        /* give second chance */
        unsafe {
            dev_warn(
                (*(*(*device).bus).card).dev,
                b"invalid CS8427 signature 0x%x: let me try again...\n\0".as_ptr()
                    as *const c_char,
                err,
            );
        }
        err = unsafe { snd_cs8427_reg_read(device, CS8427_REG_ID_AND_VER) };
    }
    if err != unsafe { CS8427_VER8427A } {
        unsafe { snd_i2c_unlock(bus) };
        unsafe {
            dev_err(
                (*(*(*device).bus).card).dev,
                b"unable to find CS8427 signature (expected 0x%x, read 0x%x),\n\0".as_ptr()
                    as *const c_char,
                CS8427_VER8427A,
                err,
            );
            dev_err(
                (*(*(*device).bus).card).dev,
                b"   initialization is not completed\n\0".as_ptr() as *const c_char,
            );
        }
        return -EFAULT;
    }
    /* turn off run bit while making changes to configuration */
    err = unsafe { snd_cs8427_reg_write(device, CS8427_REG_CLOCKSOURCE as u8, 0x00) };
    if err < 0 {
        unsafe { snd_i2c_unlock(bus) };
        return err;
    }
    /* send initial values */
    unsafe {
        memcpy(
            (*chip).regmap.as_mut_ptr().add((initvals1[0] & 0x7f) as usize) as *mut c_void,
            initvals1.as_ptr().add(1) as *const c_void,
            6,
        );
    }
    err = unsafe { snd_i2c_sendbytes(device, initvals1.as_ptr(), 7) };
    if err != 7 {
        err = if err < 0 { err } else { -EIO };
        unsafe { snd_i2c_unlock(bus) };
        return err;
    }
    /* Turn off CS8427 interrupt stuff that is not used in hardware */
    unsafe { memset(buf.as_mut_ptr() as *mut c_void, 0, 7) };
    /* from address 9 to 15 */
    buf[0] = 9; /* register */
    err = unsafe { snd_i2c_sendbytes(device, buf.as_ptr(), 7) };
    if err != 7 {
        unsafe { snd_i2c_unlock(bus) };
        return err;
    }
    /* send transfer initialization sequence */
    unsafe {
        memcpy(
            (*chip).regmap.as_mut_ptr().add((initvals2[0] & 0x7f) as usize) as *mut c_void,
            initvals2.as_ptr().add(1) as *const c_void,
            3,
        );
    }
    err = unsafe { snd_i2c_sendbytes(device, initvals2.as_ptr(), 4) };
    if err != 4 {
        err = if err < 0 { err } else { -EIO };
        unsafe { snd_i2c_unlock(bus) };
        return err;
    }
    /* write default channel status bytes */
    unsafe { put_unaligned_le32(SNDRV_PCM_DEFAULT_CON_SPDIF, buf.as_mut_ptr()) };
    unsafe { memset(buf.as_mut_ptr().add(4) as *mut c_void, 0, 24 - 4) };
    if unsafe { snd_cs8427_send_corudata(device, 0, buf.as_mut_ptr(), 24) } < 0 {
        unsafe { snd_i2c_unlock(bus) };
        return err;
    }
    unsafe {
        memcpy((*chip).playback.def_status.as_mut_ptr() as *mut c_void, buf.as_ptr() as *const c_void, 24);
        memcpy((*chip).playback.pcm_status.as_mut_ptr() as *mut c_void, buf.as_ptr() as *const c_void, 24);
        snd_i2c_unlock(bus);
    }

    /* turn on run bit and rock'n'roll */
    unsafe { snd_cs8427_reset(device) };

    0
}

pub unsafe extern "C" fn snd_cs8427_create(
    bus: *mut snd_i2c_bus,
    addr: u8,
    mut reset_timeout: c_uint,
    r_cs8427: *mut *mut snd_i2c_device,
) -> c_int {
    let mut err: c_int;
    let chip: *mut cs8427;
    let mut device: *mut snd_i2c_device = ptr::null_mut();

    err = unsafe {
        snd_i2c_device_create(
            bus,
            CS8427_NAME.as_ptr() as *const c_char,
            CS8427_ADDR | ((addr & 7) as c_uint),
            &mut device,
        )
    };
    if err < 0 {
        return err;
    }
    chip = unsafe { kzalloc(size_of::<cs8427>(), GFP_KERNEL) as *mut cs8427 };
    unsafe { (*device).private_data = chip as *mut c_void };
    if chip.is_null() {
        unsafe { snd_i2c_device_free(device) };
        return -ENOMEM;
    }
    unsafe { (*device).private_free = Some(snd_cs8427_free) };

    if reset_timeout < 1 {
        reset_timeout = 1;
    }
    unsafe { (*chip).reset_timeout = reset_timeout };

    err = unsafe { snd_cs8427_init(bus, device) };
    if err != 0 {
        unsafe { snd_i2c_device_free(device) };
        return if err < 0 { err } else { -EIO };
    }

    /*
    #if 0   // it's nice for read tests
    {
    char buf[128];
    int xx;
    buf[0] = 0x81;
    snd_i2c_sendbytes(device, buf, 1);
    snd_i2c_readbytes(device, buf, 127);
    for (xx = 0; xx < 127; xx++)
        dev_dbg(device->bus->card->dev, "reg[0x%x] = 0x%x\n", xx+1, buf[xx]);
    }
    #endif
    */

    if !r_cs8427.is_null() {
        unsafe { *r_cs8427 = device };
    }
    0
}

/*
 * Reset the chip using run bit, also lock PLL using ILRCK and
 * put back AES3INPUT. This workaround is described in latest
 * CS8427 datasheet, otherwise TXDSERIAL will not work.
 */
unsafe fn snd_cs8427_reset(cs8427: *mut snd_i2c_device) {
    let chip: *mut cs8427;
    let end_time: c_ulong;
    let mut data: c_int;
    let mut aes3input: c_int = 0;

    if unsafe { snd_BUG_ON(cs8427.is_null()) } {
        return;
    }
    chip = unsafe { (*cs8427).private_data as *mut cs8427 };
    unsafe { snd_i2c_lock((*cs8427).bus) };
    if (unsafe { (*chip).regmap[CS8427_REG_CLOCKSOURCE] & CS8427_RXDAES3INPUT })
        == unsafe { CS8427_RXDAES3INPUT }
    {
        /* AES3 bit is set */
        aes3input = 1;
    }
    unsafe {
        (*chip).regmap[CS8427_REG_CLOCKSOURCE] &= !(CS8427_RUN | CS8427_RXDMASK);
        snd_cs8427_reg_write(
            cs8427,
            CS8427_REG_CLOCKSOURCE as u8,
            (*chip).regmap[CS8427_REG_CLOCKSOURCE],
        );
        udelay(200);
        (*chip).regmap[CS8427_REG_CLOCKSOURCE] |= CS8427_RUN | CS8427_RXDILRCK;
        snd_cs8427_reg_write(
            cs8427,
            CS8427_REG_CLOCKSOURCE as u8,
            (*chip).regmap[CS8427_REG_CLOCKSOURCE],
        );
        udelay(200);
        snd_i2c_unlock((*cs8427).bus);
        end_time = jiffies.wrapping_add((*chip).reset_timeout as c_ulong);
    }
    while unsafe { time_after_eq(end_time, jiffies) } {
        unsafe {
            snd_i2c_lock((*cs8427).bus);
            data = snd_cs8427_reg_read(cs8427, CS8427_REG_RECVERRORS);
            snd_i2c_unlock((*cs8427).bus);
        }
        if (data & unsafe { CS8427_UNLOCK }) == 0 {
            break;
        }
        unsafe { schedule_timeout_uninterruptible(1) };
    }
    unsafe {
        snd_i2c_lock((*cs8427).bus);
        (*chip).regmap[CS8427_REG_CLOCKSOURCE] &= !CS8427_RXDMASK;
        if aes3input != 0 {
            (*chip).regmap[CS8427_REG_CLOCKSOURCE] |= CS8427_RXDAES3INPUT;
        }
        snd_cs8427_reg_write(
            cs8427,
            CS8427_REG_CLOCKSOURCE as u8,
            (*chip).regmap[CS8427_REG_CLOCKSOURCE],
        );
        snd_i2c_unlock((*cs8427).bus);
    }
}

unsafe extern "C" fn snd_cs8427_in_status_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).count = 1;
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = 255;
    }
    0
}

unsafe extern "C" fn snd_cs8427_in_status_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let device = unsafe { snd_kcontrol_chip(kcontrol) };
    let data: c_int;

    unsafe {
        snd_i2c_lock((*device).bus);
        data = snd_cs8427_reg_read(device, (*kcontrol).private_value as u8);
        snd_i2c_unlock((*device).bus);
    }
    if data < 0 {
        return data;
    }
    unsafe { (*ucontrol).value.integer.value[0] = data as i64 };
    0
}

unsafe extern "C" fn snd_cs8427_qsubcode_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BYTES;
        (*uinfo).count = 10;
    }
    0
}

unsafe extern "C" fn snd_cs8427_qsubcode_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let device = unsafe { snd_kcontrol_chip(kcontrol) };
    let mut reg = unsafe { CS8427_REG_QSUBCODE };
    let mut err: c_int;

    unsafe {
        snd_i2c_lock((*device).bus);
        err = snd_i2c_sendbytes(device, &mut reg, 1);
    }
    if err != 1 {
        unsafe {
            dev_err(
                (*(*(*device).bus).card).dev,
                b"unable to send register 0x%x byte to CS8427\n\0".as_ptr() as *const c_char,
                reg as c_int,
            );
            snd_i2c_unlock((*device).bus);
        }
        return if err < 0 { err } else { -EIO };
    }
    unsafe {
        err = snd_i2c_readbytes(device, (*ucontrol).value.bytes.data.as_mut_ptr(), 10);
    }
    if err != 10 {
        unsafe {
            dev_err(
                (*(*(*device).bus).card).dev,
                b"unable to read Q-subcode bytes from CS8427\n\0".as_ptr() as *const c_char,
            );
            snd_i2c_unlock((*device).bus);
        }
        return if err < 0 { err } else { -EIO };
    }
    unsafe { snd_i2c_unlock((*device).bus) };
    0
}

unsafe extern "C" fn snd_cs8427_spdif_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
        (*uinfo).count = 1;
    }
    0
}

unsafe extern "C" fn snd_cs8427_spdif_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let device = unsafe { snd_kcontrol_chip(kcontrol) };
    let chip = unsafe { (*device).private_data as *mut cs8427 };

    unsafe {
        snd_i2c_lock((*device).bus);
        memcpy(
            (*ucontrol).value.iec958.status.as_mut_ptr() as *mut c_void,
            (*chip).playback.def_status.as_ptr() as *const c_void,
            24,
        );
        snd_i2c_unlock((*device).bus);
    }
    0
}

unsafe extern "C" fn snd_cs8427_spdif_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let device = unsafe { snd_kcontrol_chip(kcontrol) };
    let chip = unsafe { (*device).private_data as *mut cs8427 };
    let status = unsafe {
        if (*kcontrol).private_value != 0 {
            (*chip).playback.pcm_status.as_mut_ptr()
        } else {
            (*chip).playback.def_status.as_mut_ptr()
        }
    };
    let runtime = unsafe {
        if !(*chip).playback.substream.is_null() {
            (*(*chip).playback.substream).runtime
        } else {
            ptr::null_mut()
        }
    };
    let mut err: c_int;
    let mut change: c_int;

    unsafe {
        snd_i2c_lock((*device).bus);
        change = (memcmp(
            (*ucontrol).value.iec958.status.as_ptr() as *const c_void,
            status as *const c_void,
            24,
        ) != 0) as c_int;
        memcpy(
            status as *mut c_void,
            (*ucontrol).value.iec958.status.as_ptr() as *const c_void,
            24,
        );
        if change != 0
            && if (*kcontrol).private_value != 0 {
                !runtime.is_null()
            } else {
                runtime.is_null()
            }
        {
            err = snd_cs8427_send_corudata(device, 0, status as *mut u8, 24);
            if err < 0 {
                change = err;
            }
        }
        snd_i2c_unlock((*device).bus);
    }
    change
}

unsafe extern "C" fn snd_cs8427_spdif_mask_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
        (*uinfo).count = 1;
    }
    0
}

unsafe extern "C" fn snd_cs8427_spdif_mask_get(
    _kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        memset(
            (*ucontrol).value.iec958.status.as_mut_ptr() as *mut c_void,
            0xff,
            24,
        );
    }
    0
}

static SND_CS8427_IEC958_CONTROLS: [snd_kcontrol_new; 6] = unsafe {
    [
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_PCM,
            info: Some(snd_cs8427_in_status_info),
            name: IEC958_INPUT_STATUS.as_ptr() as *const c_char,
            access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
            get: Some(snd_cs8427_in_status_get),
            put: None,
            private_value: 15,
        },
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_PCM,
            info: Some(snd_cs8427_in_status_info),
            name: IEC958_ERROR_STATUS.as_ptr() as *const c_char,
            access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
            get: Some(snd_cs8427_in_status_get),
            put: None,
            private_value: 16,
        },
        snd_kcontrol_new {
            access: SNDRV_CTL_ELEM_ACCESS_READ,
            iface: SNDRV_CTL_ELEM_IFACE_PCM,
            name: IEC958_PLAYBACK_MASK.as_ptr() as *const c_char,
            info: Some(snd_cs8427_spdif_mask_info),
            get: Some(snd_cs8427_spdif_mask_get),
            put: None,
            private_value: 0,
        },
        snd_kcontrol_new {
            access: 0,
            iface: SNDRV_CTL_ELEM_IFACE_PCM,
            name: IEC958_PLAYBACK_DEFAULT.as_ptr() as *const c_char,
            info: Some(snd_cs8427_spdif_info),
            get: Some(snd_cs8427_spdif_get),
            put: Some(snd_cs8427_spdif_put),
            private_value: 0,
        },
        snd_kcontrol_new {
            access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_INACTIVE,
            iface: SNDRV_CTL_ELEM_IFACE_PCM,
            name: IEC958_PLAYBACK_PCM_STREAM.as_ptr() as *const c_char,
            info: Some(snd_cs8427_spdif_info),
            get: Some(snd_cs8427_spdif_get),
            put: Some(snd_cs8427_spdif_put),
            private_value: 1,
        },
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_PCM,
            info: Some(snd_cs8427_qsubcode_info),
            name: IEC958_Q_SUBCODE_CAPTURE_DEFAULT.as_ptr() as *const c_char,
            access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
            get: Some(snd_cs8427_qsubcode_get),
            put: None,
            private_value: 0,
        },
    ]
};

pub unsafe extern "C" fn snd_cs8427_iec958_build(
    cs8427: *mut snd_i2c_device,
    play_substream: *mut snd_pcm_substream,
    cap_substream: *mut snd_pcm_substream,
) -> c_int {
    let chip = unsafe { (*cs8427).private_data as *mut cs8427 };
    let mut kctl: *mut snd_kcontrol;
    let mut idx: c_uint;
    let mut err: c_int;

    if unsafe { snd_BUG_ON(play_substream.is_null() || cap_substream.is_null()) } {
        return -EINVAL;
    }
    idx = 0;
    while (idx as usize) < SND_CS8427_IEC958_CONTROLS.len() {
        kctl = unsafe {
            snd_ctl_new1(
                &SND_CS8427_IEC958_CONTROLS[idx as usize],
                cs8427 as *mut c_void,
            )
        };
        if kctl.is_null() {
            return -ENOMEM;
        }
        unsafe {
            (*kctl).id.device = (*(*play_substream).pcm).device as c_uint;
            (*kctl).id.subdevice = (*play_substream).number;
            err = snd_ctl_add((*(*cs8427).bus).card, kctl);
        }
        if err < 0 {
            return err;
        }
        if unsafe { strcmp((*kctl).id.name.as_ptr(), snd_ctl_name_iec958_playback_pcm_stream()) } == 0 {
            unsafe { (*chip).playback.pcm_ctl = kctl };
        }
        idx += 1;
    }

    unsafe {
        (*chip).playback.substream = play_substream;
        (*chip).capture.substream = cap_substream;
    }
    if unsafe { snd_BUG_ON((*chip).playback.pcm_ctl.is_null()) } {
        return -EIO;
    }
    0
}

pub unsafe extern "C" fn snd_cs8427_iec958_active(
    cs8427: *mut snd_i2c_device,
    active: c_int,
) -> c_int {
    let chip: *mut cs8427;

    if unsafe { snd_BUG_ON(cs8427.is_null()) } {
        return -ENXIO;
    }
    chip = unsafe { (*cs8427).private_data as *mut cs8427 };
    if active != 0 {
        unsafe {
            memcpy(
                (*chip).playback.pcm_status.as_mut_ptr() as *mut c_void,
                (*chip).playback.def_status.as_ptr() as *const c_void,
                24,
            );
            (*(*chip).playback.pcm_ctl).vd.add(0).as_mut().unwrap().access &=
                !SNDRV_CTL_ELEM_ACCESS_INACTIVE;
        }
    } else {
        unsafe {
            (*(*chip).playback.pcm_ctl).vd.add(0).as_mut().unwrap().access |=
                SNDRV_CTL_ELEM_ACCESS_INACTIVE;
        }
    }
    unsafe {
        snd_ctl_notify(
            (*(*cs8427).bus).card,
            SNDRV_CTL_EVENT_MASK_VALUE | SNDRV_CTL_EVENT_MASK_INFO,
            &mut (*(*chip).playback.pcm_ctl).id,
        );
    }
    0
}

pub unsafe extern "C" fn snd_cs8427_iec958_pcm(
    cs8427: *mut snd_i2c_device,
    rate: c_uint,
) -> c_int {
    let chip: *mut cs8427;
    let status: *mut c_char;
    let mut err: c_int;
    let reset: c_int;

    if unsafe { snd_BUG_ON(cs8427.is_null()) } {
        return -ENXIO;
    }
    chip = unsafe { (*cs8427).private_data as *mut cs8427 };
    status = unsafe { (*chip).playback.pcm_status.as_mut_ptr() };
    unsafe { snd_i2c_lock((*cs8427).bus) };
    if unsafe { (*(status.add(0)) as u8) & IEC958_AES0_PROFESSIONAL } != 0 {
        unsafe {
            *status.add(0) = ((*status.add(0) as u8) & !IEC958_AES0_PRO_FS) as c_char;
            match rate {
                32000 => *status.add(0) = ((*status.add(0) as u8) | IEC958_AES0_PRO_FS_32000) as c_char,
                44100 => *status.add(0) = ((*status.add(0) as u8) | IEC958_AES0_PRO_FS_44100) as c_char,
                48000 => *status.add(0) = ((*status.add(0) as u8) | IEC958_AES0_PRO_FS_48000) as c_char,
                _ => *status.add(0) = ((*status.add(0) as u8) | IEC958_AES0_PRO_FS_NOTID) as c_char,
            }
        }
    } else {
        unsafe {
            *status.add(3) = ((*status.add(3) as u8) & !IEC958_AES3_CON_FS) as c_char;
            match rate {
                32000 => *status.add(3) = ((*status.add(3) as u8) | IEC958_AES3_CON_FS_32000) as c_char,
                44100 => *status.add(3) = ((*status.add(3) as u8) | IEC958_AES3_CON_FS_44100) as c_char,
                48000 => *status.add(3) = ((*status.add(3) as u8) | IEC958_AES3_CON_FS_48000) as c_char,
                _ => {}
            }
        }
    }
    err = unsafe { snd_cs8427_send_corudata(cs8427, 0, status as *mut u8, 24) };
    if err > 0 {
        unsafe {
            snd_ctl_notify(
                (*(*cs8427).bus).card,
                SNDRV_CTL_EVENT_MASK_VALUE,
                &mut (*(*chip).playback.pcm_ctl).id,
            );
        }
    }
    reset = unsafe { ((*chip).rate != rate) as c_int };
    unsafe {
        (*chip).rate = rate;
        snd_i2c_unlock((*cs8427).bus);
    }
    if reset != 0 {
        unsafe { snd_cs8427_reset(cs8427) };
    }
    if err < 0 { err } else { 0 }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
