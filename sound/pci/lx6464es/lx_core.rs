// SPDX-License-Identifier: GPL-2.0-or-later
/* -*- linux-rust -*- *
 *
 * ALSA driver for the digigram lx6464es interface
 * low-level interface
 *
 * Copyright (c) 2009 Tim Blechmann <tim@klingt.org>
 */

/* Original C dependencies:
 * linux/bitops.h, linux/module.h, linux/pci.h, linux/delay.h,
 * lx6464es.h, lx_core.h
 */

use core::ffi::c_void;

type u8 = u8;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type c_int = i32;
type c_uint = u32;
type c_ulong = usize;
type dma_addr_t = usize;
type irqreturn_t = c_int;

extern "C" {
    fn ioread32(address: *mut c_void) -> u32;
    fn iowrite32(data: u32, address: *mut c_void);
    fn udelay(usecs: c_uint);
    fn mdelay(msecs: c_uint);
    fn snd_pcm_format_little_endian(format: c_int) -> c_int;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn unpack_pointer(ptr: dma_addr_t, lo: *mut u32, hi: *mut u32);
}

extern "Rust" {
    fn guard_mutex(lock: *mut c_void);
    fn snd_BUG_ON(condition: bool);
}

extern "C" {
    fn dev_err(dev: *mut c_void, fmt: *const u8, ...);
    fn dev_warn(dev: *mut c_void, fmt: *const u8, ...);
    fn dev_dbg(dev: *mut c_void, fmt: *const u8, ...);
    fn pr_debug(fmt: *const u8, ...);
}

extern "Rust" {
    static CMD_00_INFO_DEBUG: u32;
    static CMD_01_GET_SYS_CFG: u32;
    static CMD_02_SET_GRANULARITY: u32;
    static CMD_03_SET_TIMER_IRQ: u32;
    static CMD_04_GET_EVENT: u32;
    static CMD_05_GET_PIPES: u32;
    static CMD_06_ALLOCATE_PIPE: u32;
    static CMD_07_RELEASE_PIPE: u32;
    static CMD_08_ASK_BUFFERS: u32;
    static CMD_09_STOP_PIPE: u32;
    static CMD_0A_GET_PIPE_SPL_COUNT: u32;
    static CMD_0B_TOGGLE_PIPE_STATE: u32;
    static CMD_0C_DEF_STREAM: u32;
    static CMD_0D_SET_MUTE: u32;
    static CMD_0E_GET_STREAM_SPL_COUNT: u32;
    static CMD_0F_UPDATE_BUFFER: u32;
    static CMD_10_GET_BUFFER: u32;
    static CMD_11_CANCEL_BUFFER: u32;
    static CMD_12_GET_PEAK: u32;
    static CMD_13_SET_STREAM_STATE: u32;
    static CMD_14_INVALID: u32;
    static OPCODE_OFFSET: u32;
    static REG_CRM_NUMBER: u32;
    static MAX_STREAM_BUFFER: u32;
    static eReg_CSM: c_int;
    static eReg_CRM1: c_int;
    static eReg_CRM2: c_int;
    static eReg_ADMACESMSB: c_int;
    static eReg_ADMACESLSB: c_int;
    static ePLX_IRQCS: c_int;
    static ePLX_L2PCIDB: c_int;
    static ED_DSP_TIMED_OUT: u32;
    static ED_DSP_CRASHED: u32;
    static ERROR_VALUE: u32;
    static EBUSY: c_int;
    static ETIMEDOUT: c_int;
    static EAGAIN: c_int;
    static FREQ_FIELD_OFFSET: u32;
    static XES_FREQ_COUNT8_MASK: u32;
    static XES_FREQ_COUNT8_48_MAX: u32;
    static XES_FREQ_COUNT8_44_MIN: u32;
    static XES_FREQ_COUNT8_44_MAX: u32;
    static ID_IS_CAPTURE: u32;
    static ID_OFFSET: u32;
    static BF_EOB: u32;
    static BF_VALID: u32;
    static BUFF_FLAGS_OFFSET: u32;
    static MASK_DATA_SIZE: u32;
    static MASK_SPL_COUNT_HI: u32;
    static PSTATE_OFFSET: u32;
    static PSTATE_RUN: u16;
    static PSTATE_IDLE: u16;
    static STREAM_FMT_16b: u32;
    static STREAM_FMT_intel: u32;
    static STREAM_FMT_OFFSET: u32;
    static SF_START: u32;
    static START_STATE: c_int;
    static PAUSE_STATE: c_int;
    static BF_NOTIFY_EOB: u32;
    static BF_64BITS_ADR: u32;
    static EB_RBUFFERS_TABLE_OVERFLOW: c_int;
    static EB_INVALID_STREAM: c_int;
    static EB_CMD_REFUSED: c_int;
    static MASK_BUFFER_ID: u32;
    static MASK_SYS_ASYNC_EVENTS: u32;
    static MASK_SYS_STATUS_ESA: u32;
    static MASK_SYS_STATUS_EOBO: u32;
    static MASK_SYS_STATUS_EOBI: u32;
    static MASK_SYS_STATUS_FREQ: u32;
    static MASK_SYS_STATUS_CMD_DONE: u32;
    static MASK_SYS_STATUS_URUN: u32;
    static MASK_SYS_STATUS_ORUN: u32;
    static IRQ_NONE: irqreturn_t;
    static IRQ_HANDLED: irqreturn_t;
    static IRQ_WAKE_THREAD: irqreturn_t;
}

#[repr(C)]
pub struct lx6464es {
    pub port_dsp_bar: *mut c_void,
    pub port_plx_remapped: *mut c_void,
    pub card: *mut snd_card,
    pub msg_lock: c_void,
    pub lock: c_void,
    pub rmh: lx_rmh,
    pub freq_ratio: u32,
    pub mac_address: [u8; 6],
    pub irqsrc: u32,
    pub capture_stream: lx_stream,
    pub playback_stream: lx_stream,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut c_void,
}

#[repr(C)]
pub struct lx_rmh {
    pub cmd: [u32; 32],
    pub stat: [u32; 32],
    pub cmd_len: u16,
    pub stat_len: u16,
    pub dsp_stat: u16,
    pub cmd_idx: u32,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub channels: u32,
    pub sample_bits: c_int,
    pub format: c_int,
    pub periods: u32,
}

#[repr(C)]
pub struct dma_buffer_t {
    pub addr: dma_addr_t,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub dma_buffer: dma_buffer_t,
}

#[repr(C)]
pub struct lx_stream {
    pub stream: *mut snd_pcm_substream,
    pub is_capture: c_uint,
    pub frame_pos: u32,
}

type stream_state_t = u32;
type cmd_mb_opcodes = u32;

/* low-level register access */

static DSP_PORT_OFFSETS: [c_ulong; 28] = [
    0, 0x400, 0x401, 0x402, 0x403, 0x404, 0x405, 0x406, 0x407, 0x408, 0x409,
    0x40a, 0x40b, 0x40c, 0x410, 0x411, 0x412, 0x413, 0x414, 0x415, 0x416,
    0x420, 0x430, 0x431, 0x432, 0x433, 0x434, 0x440,
];

unsafe fn lx_dsp_register(chip: *mut lx6464es, port: c_int) -> *mut c_void {
    let base_address = (*chip).port_dsp_bar as *mut u8;
    base_address.add(DSP_PORT_OFFSETS[port as usize] * 4) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn lx_dsp_reg_read(chip: *mut lx6464es, port: c_int) -> c_ulong {
    let address = lx_dsp_register(chip, port);
    ioread32(address) as c_ulong
}

unsafe fn lx_dsp_reg_readbuf(chip: *mut lx6464es, port: c_int, data: *mut u32, len: u32) {
    let address = lx_dsp_register(chip, port) as *mut u32;
    let mut i: c_int = 0;

    /* we cannot use memcpy_fromio */
    while i != len as c_int {
        *data.add(i as usize) = ioread32(address.add(i as usize) as *mut c_void);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn lx_dsp_reg_write(chip: *mut lx6464es, port: c_int, data: c_uint) {
    let address = lx_dsp_register(chip, port);
    iowrite32(data, address);
}

unsafe fn lx_dsp_reg_writebuf(chip: *mut lx6464es, port: c_int, data: *const u32, len: u32) {
    let address = lx_dsp_register(chip, port) as *mut u32;
    let mut i: c_int = 0;

    /* we cannot use memcpy_to */
    while i != len as c_int {
        iowrite32(*data.add(i as usize), address.add(i as usize) as *mut c_void);
        i += 1;
    }
}

static PLX_PORT_OFFSETS: [c_ulong; 12] = [
    0x04, 0x40, 0x44, 0x48, 0x4c, 0x50, 0x54, 0x58, 0x5c, 0x64, 0x68, 0x6C,
];

unsafe fn lx_plx_register(chip: *mut lx6464es, port: c_int) -> *mut c_void {
    let base_address = (*chip).port_plx_remapped as *mut u8;
    base_address.add(PLX_PORT_OFFSETS[port as usize]) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn lx_plx_reg_read(chip: *mut lx6464es, port: c_int) -> c_ulong {
    let address = lx_plx_register(chip, port);
    ioread32(address) as c_ulong
}

#[no_mangle]
pub unsafe extern "C" fn lx_plx_reg_write(chip: *mut lx6464es, port: c_int, data: u32) {
    let address = lx_plx_register(chip, port);
    iowrite32(data, address);
}

/* rmh */

const REG_CSM_MR: u32 = 0x00000002;
const REG_CSM_MC: u32 = 0x00000001;

#[repr(C)]
#[derive(Clone, Copy)]
struct dsp_cmd_info {
    dcCodeOp: u32,       /* Op Code of the command (usually 1st 24-bits word).*/
    dcCmdLength: u16,    /* Command length in words of 24 bits.*/
    dcStatusType: u16,   /* Status type: 0 for fixed length, 1 for random. */
    dcStatusLength: u16, /* Status length (if fixed).*/
    dcOpName: *const u8,
}

/*
  Initialization and control data for the Microblaze interface
  - OpCode:
    the opcode field of the command set at the proper offset
  - CmdLength
    the number of command words
  - StatusType
    offset in the status registers: 0 means that the return value may be
    different from 0, and must be read
  - StatusLength
    the number of status words (in addition to the return value)
*/

unsafe fn cmd_name(name: &'static [u8]) -> *const u8 {
    /* CONFIG_SND_DEBUG maps this to the string; otherwise C used NULL. */
    name.as_ptr()
}

static mut DSP_COMMANDS: [dsp_cmd_info; 20] = [
    dsp_cmd_info { dcCodeOp: 0, dcCmdLength: 1, dcStatusType: 1, dcStatusLength: 0, dcOpName: b"INFO_DEBUG\0".as_ptr() },
    dsp_cmd_info { dcCodeOp: 0, dcCmdLength: 1, dcStatusType: 1, dcStatusLength: 2, dcOpName: b"GET_SYS_CFG\0".as_ptr() },
    dsp_cmd_info { dcCodeOp: 0, dcCmdLength: 1, dcStatusType: 1, dcStatusLength: 0, dcOpName: b"SET_GRANULARITY\0".as_ptr() },
    dsp_cmd_info { dcCodeOp: 0, dcCmdLength: 1, dcStatusType: 1, dcStatusLength: 0, dcOpName: b"SET_TIMER_IRQ\0".as_ptr() },
    dsp_cmd_info { dcCodeOp: 0, dcCmdLength: 1, dcStatusType: 1, dcStatusLength: 0, dcOpName: b"GET_EVENT\0".as_ptr() },
    dsp_cmd_info { dcCodeOp: 0, dcCmdLength: 1, dcStatusType: 1, dcStatusLength: 2, dcOpName: b"GET_PIPES\0".as_ptr() },
    dsp_cmd_info { dcCodeOp: 0, dcCmdLength: 1, dcStatusType: 0, dcStatusLength: 0, dcOpName: b"ALLOCATE_PIPE\0".as_ptr() },
    dsp_cmd_info { dcCodeOp: 0, dcCmdLength: 1, dcStatusType: 0, dcStatusLength: 0, dcOpName: b"RELEASE_PIPE\0".as_ptr() },
    dsp_cmd_info { dcCodeOp: 0, dcCmdLength: 1, dcStatusType: 1, dcStatusLength: 0, dcOpName: b"ASK_BUFFERS\0".as_ptr() },
    dsp_cmd_info { dcCodeOp: 0, dcCmdLength: 1, dcStatusType: 0, dcStatusLength: 0, dcOpName: b"STOP_PIPE\0".as_ptr() },
    dsp_cmd_info { dcCodeOp: 0, dcCmdLength: 1, dcStatusType: 1, dcStatusLength: 1, dcOpName: b"GET_PIPE_SPL_COUNT\0".as_ptr() },
    dsp_cmd_info { dcCodeOp: 0, dcCmdLength: 1, dcStatusType: 1, dcStatusLength: 0, dcOpName: b"TOGGLE_PIPE_STATE\0".as_ptr() },
    dsp_cmd_info { dcCodeOp: 0, dcCmdLength: 1, dcStatusType: 1, dcStatusLength: 0, dcOpName: b"DEF_STREAM\0".as_ptr() },
    dsp_cmd_info { dcCodeOp: 0, dcCmdLength: 3, dcStatusType: 1, dcStatusLength: 0, dcOpName: b"SET_MUTE\0".as_ptr() },
    dsp_cmd_info { dcCodeOp: 0, dcCmdLength: 1, dcStatusType: 1, dcStatusLength: 2, dcOpName: b"GET_STREAM_SPL_COUNT\0".as_ptr() },
    dsp_cmd_info { dcCodeOp: 0, dcCmdLength: 3, dcStatusType: 0, dcStatusLength: 1, dcOpName: b"UPDATE_BUFFER\0".as_ptr() },
    dsp_cmd_info { dcCodeOp: 0, dcCmdLength: 1, dcStatusType: 1, dcStatusLength: 4, dcOpName: b"GET_BUFFER\0".as_ptr() },
    dsp_cmd_info { dcCodeOp: 0, dcCmdLength: 1, dcStatusType: 1, dcStatusLength: 1, dcOpName: b"CANCEL_BUFFER\0".as_ptr() },
    dsp_cmd_info { dcCodeOp: 0, dcCmdLength: 1, dcStatusType: 1, dcStatusLength: 1, dcOpName: b"GET_PEAK\0".as_ptr() },
    dsp_cmd_info { dcCodeOp: 0, dcCmdLength: 1, dcStatusType: 1, dcStatusLength: 0, dcOpName: b"SET_STREAM_STATE\0".as_ptr() },
];

unsafe fn dsp_command_info(cmd: cmd_mb_opcodes) -> dsp_cmd_info {
    let mut info = DSP_COMMANDS[cmd as usize];
    let opcodes = [
        CMD_00_INFO_DEBUG, CMD_01_GET_SYS_CFG, CMD_02_SET_GRANULARITY,
        CMD_03_SET_TIMER_IRQ, CMD_04_GET_EVENT, CMD_05_GET_PIPES,
        CMD_06_ALLOCATE_PIPE, CMD_07_RELEASE_PIPE, CMD_08_ASK_BUFFERS,
        CMD_09_STOP_PIPE, CMD_0A_GET_PIPE_SPL_COUNT, CMD_0B_TOGGLE_PIPE_STATE,
        CMD_0C_DEF_STREAM, CMD_0D_SET_MUTE, CMD_0E_GET_STREAM_SPL_COUNT,
        CMD_0F_UPDATE_BUFFER, CMD_10_GET_BUFFER, CMD_11_CANCEL_BUFFER,
        CMD_12_GET_PEAK, CMD_13_SET_STREAM_STATE,
    ];
    info.dcCodeOp = opcodes[cmd as usize] << OPCODE_OFFSET;
    if cmd == CMD_08_ASK_BUFFERS {
        info.dcStatusLength = MAX_STREAM_BUFFER as u16;
    }
    info
}

unsafe fn lx_message_init(rmh: *mut lx_rmh, cmd: cmd_mb_opcodes) {
    snd_BUG_ON(cmd >= CMD_14_INVALID);

    let info = dsp_command_info(cmd);
    (*rmh).cmd[0] = info.dcCodeOp;
    (*rmh).cmd_len = info.dcCmdLength;
    (*rmh).stat_len = info.dcStatusLength;
    (*rmh).dsp_stat = info.dcStatusType;
    (*rmh).cmd_idx = cmd;
    let mut i = 1usize;
    while i != REG_CRM_NUMBER as usize {
        (*rmh).cmd[i] = 0;
        i += 1;
    }

    /* CONFIG_SND_DEBUG: memset(rmh->stat, 0, REG_CRM_NUMBER * sizeof(u32)); */
    /* RMH_DEBUG: rmh->cmd_idx = cmd; */
}

/* RMH_DEBUG conditional dump. */
unsafe fn lx_message_dump(_rmh: *mut lx_rmh) {}

/* sleep 500 - 100 = 400 times 100us -> the timeout is >= 40 ms */
const XILINX_TIMEOUT_MS: c_int = 40;
const XILINX_POLL_NO_SLEEP: c_int = 100;
const XILINX_POLL_ITERATIONS: c_int = 150;

unsafe fn lx_message_send_atomic(chip: *mut lx6464es, rmh: *mut lx_rmh) -> c_int {
    let mut reg: u32 = ED_DSP_TIMED_OUT;
    let mut dwloop: c_int;

    if (lx_dsp_reg_read(chip, eReg_CSM) as u32 & (REG_CSM_MC | REG_CSM_MR)) != 0 {
        dev_err((*(*chip).card).dev, b"PIOSendMessage eReg_CSM %x\n\0".as_ptr(), reg);
        return -EBUSY;
    }

    /* write command */
    lx_dsp_reg_writebuf(chip, eReg_CRM1, (*rmh).cmd.as_ptr(), (*rmh).cmd_len as u32);

    /* MicoBlaze gogogo */
    lx_dsp_reg_write(chip, eReg_CSM, REG_CSM_MC);

    /* wait for device to answer */
    dwloop = 0;
    while dwloop != XILINX_TIMEOUT_MS * 1000 {
        if (lx_dsp_reg_read(chip, eReg_CSM) as u32 & REG_CSM_MR) != 0 {
            if (*rmh).dsp_stat == 0 {
                reg = lx_dsp_reg_read(chip, eReg_CRM1) as u32;
            } else {
                reg = 0;
            }
            break;
        } else {
            udelay(1);
        }
        dwloop += 1;
    }
    if dwloop == XILINX_TIMEOUT_MS * 1000 {
        dev_warn((*(*chip).card).dev, b"TIMEOUT lx_message_send_atomic! polling failed\n\0".as_ptr());
    }

    if (reg & ERROR_VALUE) == 0 {
        /* read response */
        if (*rmh).stat_len != 0 {
            snd_BUG_ON((*rmh).stat_len as u32 >= (REG_CRM_NUMBER - 1));
            lx_dsp_reg_readbuf(chip, eReg_CRM2, (*rmh).stat.as_mut_ptr(), (*rmh).stat_len as u32);
        }
    } else {
        dev_err((*(*chip).card).dev, b"rmh error: %08x\n\0".as_ptr(), reg);
    }

    /* clear Reg_CSM_MR */
    lx_dsp_reg_write(chip, eReg_CSM, 0);

    if reg == ED_DSP_TIMED_OUT {
        dev_warn((*(*chip).card).dev, b"lx_message_send: dsp timeout\n\0".as_ptr());
        return -ETIMEDOUT;
    }
    if reg == ED_DSP_CRASHED {
        dev_warn((*(*chip).card).dev, b"lx_message_send: dsp crashed\n\0".as_ptr());
        return -EAGAIN;
    }

    lx_message_dump(rmh);

    reg as c_int
}

/* low-level dsp access */
#[no_mangle]
pub unsafe extern "C" fn lx_dsp_get_version(chip: *mut lx6464es, rdsp_version: *mut u32) -> c_int {
    let ret: c_int;

    guard_mutex(&mut (*chip).msg_lock as *mut _ as *mut c_void);

    lx_message_init(&mut (*chip).rmh, CMD_01_GET_SYS_CFG);
    ret = lx_message_send_atomic(chip, &mut (*chip).rmh);

    *rdsp_version = (*chip).rmh.stat[1];
    ret
}

#[no_mangle]
pub unsafe extern "C" fn lx_dsp_get_clock_frequency(chip: *mut lx6464es, rfreq: *mut u32) -> c_int {
    let mut freq_raw: u32 = 0;
    let mut freq: u32 = 0;
    let mut frequency: u32 = 0;
    let ret: c_int;

    guard_mutex(&mut (*chip).msg_lock as *mut _ as *mut c_void);

    lx_message_init(&mut (*chip).rmh, CMD_01_GET_SYS_CFG);
    ret = lx_message_send_atomic(chip, &mut (*chip).rmh);

    if ret == 0 {
        freq_raw = (*chip).rmh.stat[0] >> FREQ_FIELD_OFFSET;
        freq = freq_raw & XES_FREQ_COUNT8_MASK;

        if freq < XES_FREQ_COUNT8_48_MAX || freq > XES_FREQ_COUNT8_44_MIN {
            frequency = 0; /* unknown */
        } else if freq >= XES_FREQ_COUNT8_44_MAX {
            frequency = 44100;
        } else {
            frequency = 48000;
        }
    }

    *rfreq = frequency.wrapping_mul((*chip).freq_ratio);

    ret
}

#[no_mangle]
pub unsafe extern "C" fn lx_dsp_get_mac(chip: *mut lx6464es) -> c_int {
    let macmsb: u32;
    let maclsb: u32;

    macmsb = lx_dsp_reg_read(chip, eReg_ADMACESMSB) as u32 & 0x00FFFFFF;
    maclsb = lx_dsp_reg_read(chip, eReg_ADMACESLSB) as u32 & 0x00FFFFFF;

    /* todo: endianess handling */
    (*chip).mac_address[5] = *(&maclsb as *const u32 as *const u8).add(0);
    (*chip).mac_address[4] = *(&maclsb as *const u32 as *const u8).add(1);
    (*chip).mac_address[3] = *(&maclsb as *const u32 as *const u8).add(2);
    (*chip).mac_address[2] = *(&macmsb as *const u32 as *const u8).add(0);
    (*chip).mac_address[1] = *(&macmsb as *const u32 as *const u8).add(1);
    (*chip).mac_address[0] = *(&macmsb as *const u32 as *const u8).add(2);

    0
}

#[no_mangle]
pub unsafe extern "C" fn lx_dsp_set_granularity(chip: *mut lx6464es, gran: u32) -> c_int {
    guard_mutex(&mut (*chip).msg_lock as *mut _ as *mut c_void);

    lx_message_init(&mut (*chip).rmh, CMD_02_SET_GRANULARITY);
    (*chip).rmh.cmd[0] |= gran;

    lx_message_send_atomic(chip, &mut (*chip).rmh)
}

#[no_mangle]
pub unsafe extern "C" fn lx_dsp_read_async_events(chip: *mut lx6464es, data: *mut u32) -> c_int {
    let ret: c_int;

    guard_mutex(&mut (*chip).msg_lock as *mut _ as *mut c_void);

    lx_message_init(&mut (*chip).rmh, CMD_04_GET_EVENT);
    (*chip).rmh.stat_len = 9; /* we don't necessarily need the full length */

    ret = lx_message_send_atomic(chip, &mut (*chip).rmh);

    if ret == 0 {
        core::ptr::copy_nonoverlapping((*chip).rmh.stat.as_ptr(), data, (*chip).rmh.stat_len as usize);
    }

    ret
}

fn PIPE_INFO_TO_CMD(capture: c_int, pipe: u32) -> u32 {
    unsafe { ((pipe | if capture != 0 { ID_IS_CAPTURE } else { 0 }) as u32) << ID_OFFSET }
}

/* low-level pipe handling */
#[no_mangle]
pub unsafe extern "C" fn lx_pipe_allocate(chip: *mut lx6464es, pipe: u32, is_capture: c_int, channels: c_int) -> c_int {
    let err: c_int;
    let pipe_cmd = PIPE_INFO_TO_CMD(is_capture, pipe);

    guard_mutex(&mut (*chip).msg_lock as *mut _ as *mut c_void);
    lx_message_init(&mut (*chip).rmh, CMD_06_ALLOCATE_PIPE);

    (*chip).rmh.cmd[0] |= pipe_cmd;
    (*chip).rmh.cmd[0] |= channels as u32;

    err = lx_message_send_atomic(chip, &mut (*chip).rmh);

    if err != 0 {
        dev_err((*(*chip).card).dev, b"could not allocate pipe\n\0".as_ptr());
    }

    err
}

#[no_mangle]
pub unsafe extern "C" fn lx_pipe_release(chip: *mut lx6464es, pipe: u32, is_capture: c_int) -> c_int {
    let pipe_cmd = PIPE_INFO_TO_CMD(is_capture, pipe);

    guard_mutex(&mut (*chip).msg_lock as *mut _ as *mut c_void);
    lx_message_init(&mut (*chip).rmh, CMD_07_RELEASE_PIPE);

    (*chip).rmh.cmd[0] |= pipe_cmd;

    lx_message_send_atomic(chip, &mut (*chip).rmh)
}

#[no_mangle]
pub unsafe extern "C" fn lx_buffer_ask(
    chip: *mut lx6464es,
    pipe: u32,
    is_capture: c_int,
    r_needed: *mut u32,
    r_freed: *mut u32,
    size_array: *mut u32,
) -> c_int {
    let err: c_int;
    let pipe_cmd = PIPE_INFO_TO_CMD(is_capture, pipe);

    /* CONFIG_SND_DEBUG:
     * if (!size_array.is_null()) memset(size_array, 0, sizeof(u32) * MAX_STREAM_BUFFER);
     */

    *r_needed = 0;
    *r_freed = 0;

    guard_mutex(&mut (*chip).msg_lock as *mut _ as *mut c_void);
    lx_message_init(&mut (*chip).rmh, CMD_08_ASK_BUFFERS);

    (*chip).rmh.cmd[0] |= pipe_cmd;

    err = lx_message_send_atomic(chip, &mut (*chip).rmh);

    if err == 0 {
        let mut i: c_int = 0;
        while i < MAX_STREAM_BUFFER as c_int {
            let stat = (*chip).rmh.stat[i as usize];
            if (stat & (BF_EOB << BUFF_FLAGS_OFFSET)) != 0 {
                /* finished */
                *r_freed = (*r_freed).wrapping_add(1);
                if !size_array.is_null() {
                    *size_array.add(i as usize) = stat & MASK_DATA_SIZE;
                }
            } else if (stat & (BF_VALID << BUFF_FLAGS_OFFSET)) == 0 {
                /* free */
                *r_needed = (*r_needed).wrapping_add(1);
            }
            i += 1;
        }

        dev_dbg((*(*chip).card).dev, b"CMD_08_ASK_BUFFERS: needed %d, freed %d\n\0".as_ptr(), *r_needed, *r_freed);
        i = 0;
        while i < MAX_STREAM_BUFFER as c_int && i < (*chip).rmh.stat_len as c_int {
            dev_dbg(
                (*(*chip).card).dev,
                b"  stat[%d]: %x, %x\n\0".as_ptr(),
                i,
                (*chip).rmh.stat[i as usize],
                (*chip).rmh.stat[i as usize] & MASK_DATA_SIZE,
            );
            i += 1;
        }
    }

    err
}

#[no_mangle]
pub unsafe extern "C" fn lx_pipe_stop(chip: *mut lx6464es, pipe: u32, is_capture: c_int) -> c_int {
    let pipe_cmd = PIPE_INFO_TO_CMD(is_capture, pipe);

    guard_mutex(&mut (*chip).msg_lock as *mut _ as *mut c_void);
    lx_message_init(&mut (*chip).rmh, CMD_09_STOP_PIPE);

    (*chip).rmh.cmd[0] |= pipe_cmd;

    lx_message_send_atomic(chip, &mut (*chip).rmh)
}

unsafe fn lx_pipe_toggle_state(chip: *mut lx6464es, pipe: u32, is_capture: c_int) -> c_int {
    let pipe_cmd = PIPE_INFO_TO_CMD(is_capture, pipe);

    guard_mutex(&mut (*chip).msg_lock as *mut _ as *mut c_void);
    lx_message_init(&mut (*chip).rmh, CMD_0B_TOGGLE_PIPE_STATE);

    (*chip).rmh.cmd[0] |= pipe_cmd;

    lx_message_send_atomic(chip, &mut (*chip).rmh)
}

#[no_mangle]
pub unsafe extern "C" fn lx_pipe_start(chip: *mut lx6464es, pipe: u32, is_capture: c_int) -> c_int {
    let mut err: c_int;

    err = lx_pipe_wait_for_idle(chip, pipe, is_capture);
    if err < 0 {
        return err;
    }

    err = lx_pipe_toggle_state(chip, pipe, is_capture);

    err
}

#[no_mangle]
pub unsafe extern "C" fn lx_pipe_pause(chip: *mut lx6464es, pipe: u32, is_capture: c_int) -> c_int {
    let mut err: c_int = 0;

    err = lx_pipe_wait_for_start(chip, pipe, is_capture);
    if err < 0 {
        return err;
    }

    err = lx_pipe_toggle_state(chip, pipe, is_capture);

    err
}

#[no_mangle]
pub unsafe extern "C" fn lx_pipe_sample_count(chip: *mut lx6464es, pipe: u32, is_capture: c_int, rsample_count: *mut u64) -> c_int {
    let err: c_int;
    let pipe_cmd = PIPE_INFO_TO_CMD(is_capture, pipe);

    guard_mutex(&mut (*chip).msg_lock as *mut _ as *mut c_void);
    lx_message_init(&mut (*chip).rmh, CMD_0A_GET_PIPE_SPL_COUNT);

    (*chip).rmh.cmd[0] |= pipe_cmd;
    (*chip).rmh.stat_len = 2; /* need all words here! */

    err = lx_message_send_atomic(chip, &mut (*chip).rmh); /* don't sleep! */

    if err != 0 {
        dev_err((*(*chip).card).dev, b"could not query pipe's sample count\n\0".as_ptr());
    } else {
        *rsample_count = ((((*chip).rmh.stat[0] & MASK_SPL_COUNT_HI) as u64) << 24)
            + (*chip).rmh.stat[1] as u64;
    }

    err
}

#[no_mangle]
pub unsafe extern "C" fn lx_pipe_state(chip: *mut lx6464es, pipe: u32, is_capture: c_int, rstate: *mut u16) -> c_int {
    let err: c_int;
    let pipe_cmd = PIPE_INFO_TO_CMD(is_capture, pipe);

    guard_mutex(&mut (*chip).msg_lock as *mut _ as *mut c_void);
    lx_message_init(&mut (*chip).rmh, CMD_0A_GET_PIPE_SPL_COUNT);

    (*chip).rmh.cmd[0] |= pipe_cmd;

    err = lx_message_send_atomic(chip, &mut (*chip).rmh);

    if err != 0 {
        dev_err((*(*chip).card).dev, b"could not query pipe's state\n\0".as_ptr());
    } else {
        *rstate = (((*chip).rmh.stat[0] >> PSTATE_OFFSET) & 0x0F) as u16;
    }

    err
}

unsafe fn lx_pipe_wait_for_state(chip: *mut lx6464es, pipe: u32, is_capture: c_int, state: u16) -> c_int {
    let mut i: c_int;

    /* max 2*PCMOnlyGranularity = 2*1024 at 44100 = < 50 ms:
     * timeout 50 ms */
    i = 0;
    while i != 50 {
        let mut current_state: u16 = 0;
        let err = lx_pipe_state(chip, pipe, is_capture, &mut current_state);

        if err < 0 {
            return err;
        }

        if err == 0 && current_state == state {
            return 0;
        }

        mdelay(1);
        i += 1;
    }

    -ETIMEDOUT
}

#[no_mangle]
pub unsafe extern "C" fn lx_pipe_wait_for_start(chip: *mut lx6464es, pipe: u32, is_capture: c_int) -> c_int {
    lx_pipe_wait_for_state(chip, pipe, is_capture, PSTATE_RUN)
}

#[no_mangle]
pub unsafe extern "C" fn lx_pipe_wait_for_idle(chip: *mut lx6464es, pipe: u32, is_capture: c_int) -> c_int {
    lx_pipe_wait_for_state(chip, pipe, is_capture, PSTATE_IDLE)
}

/* low-level stream handling */
#[no_mangle]
pub unsafe extern "C" fn lx_stream_set_state(chip: *mut lx6464es, pipe: u32, is_capture: c_int, state: stream_state_t) -> c_int {
    let pipe_cmd = PIPE_INFO_TO_CMD(is_capture, pipe);

    guard_mutex(&mut (*chip).msg_lock as *mut _ as *mut c_void);
    lx_message_init(&mut (*chip).rmh, CMD_13_SET_STREAM_STATE);

    (*chip).rmh.cmd[0] |= pipe_cmd;
    (*chip).rmh.cmd[0] |= state;

    lx_message_send_atomic(chip, &mut (*chip).rmh)
}

#[no_mangle]
pub unsafe extern "C" fn lx_stream_set_format(chip: *mut lx6464es, runtime: *mut snd_pcm_runtime, pipe: u32, is_capture: c_int) -> c_int {
    let pipe_cmd = PIPE_INFO_TO_CMD(is_capture, pipe);
    let channels = (*runtime).channels;

    guard_mutex(&mut (*chip).msg_lock as *mut _ as *mut c_void);
    lx_message_init(&mut (*chip).rmh, CMD_0C_DEF_STREAM);

    (*chip).rmh.cmd[0] |= pipe_cmd;

    if (*runtime).sample_bits == 16 {
        /* 16 bit format */
        (*chip).rmh.cmd[0] |= STREAM_FMT_16b << STREAM_FMT_OFFSET;
    }

    if snd_pcm_format_little_endian((*runtime).format) != 0 {
        /* little endian/intel format */
        (*chip).rmh.cmd[0] |= STREAM_FMT_intel << STREAM_FMT_OFFSET;
    }

    (*chip).rmh.cmd[0] |= channels.wrapping_sub(1);

    lx_message_send_atomic(chip, &mut (*chip).rmh)
}

#[no_mangle]
pub unsafe extern "C" fn lx_stream_state(chip: *mut lx6464es, pipe: u32, is_capture: c_int, rstate: *mut c_int) -> c_int {
    let err: c_int;
    let pipe_cmd = PIPE_INFO_TO_CMD(is_capture, pipe);

    guard_mutex(&mut (*chip).msg_lock as *mut _ as *mut c_void);
    lx_message_init(&mut (*chip).rmh, CMD_0E_GET_STREAM_SPL_COUNT);

    (*chip).rmh.cmd[0] |= pipe_cmd;

    err = lx_message_send_atomic(chip, &mut (*chip).rmh);

    *rstate = if ((*chip).rmh.stat[0] & SF_START) != 0 { START_STATE } else { PAUSE_STATE };

    err
}

#[no_mangle]
pub unsafe extern "C" fn lx_stream_sample_position(chip: *mut lx6464es, pipe: u32, is_capture: c_int, r_bytepos: *mut u64) -> c_int {
    let err: c_int;
    let pipe_cmd = PIPE_INFO_TO_CMD(is_capture, pipe);

    guard_mutex(&mut (*chip).msg_lock as *mut _ as *mut c_void);
    lx_message_init(&mut (*chip).rmh, CMD_0E_GET_STREAM_SPL_COUNT);

    (*chip).rmh.cmd[0] |= pipe_cmd;

    err = lx_message_send_atomic(chip, &mut (*chip).rmh);

    *r_bytepos = ((((*chip).rmh.stat[0] & MASK_SPL_COUNT_HI) as u64) << 32)
        + (*chip).rmh.stat[1] as u64;

    err
}

/* low-level buffer handling */
#[no_mangle]
pub unsafe extern "C" fn lx_buffer_give(
    chip: *mut lx6464es,
    pipe: u32,
    is_capture: c_int,
    buffer_size: u32,
    buf_address_lo: u32,
    buf_address_hi: u32,
    r_buffer_index: *mut u32,
) -> c_int {
    let err: c_int;
    let pipe_cmd = PIPE_INFO_TO_CMD(is_capture, pipe);

    guard_mutex(&mut (*chip).msg_lock as *mut _ as *mut c_void);
    lx_message_init(&mut (*chip).rmh, CMD_0F_UPDATE_BUFFER);

    (*chip).rmh.cmd[0] |= pipe_cmd;
    (*chip).rmh.cmd[0] |= BF_NOTIFY_EOB; /* request interrupt notification */

    /* todo: pause request, circular buffer */

    (*chip).rmh.cmd[1] = buffer_size & MASK_DATA_SIZE;
    (*chip).rmh.cmd[2] = buf_address_lo;

    if buf_address_hi != 0 {
        (*chip).rmh.cmd_len = 4;
        (*chip).rmh.cmd[3] = buf_address_hi;
        (*chip).rmh.cmd[0] |= BF_64BITS_ADR;
    }

    err = lx_message_send_atomic(chip, &mut (*chip).rmh);

    if err == 0 {
        *r_buffer_index = (*chip).rmh.stat[0];
        return err;
    }

    if err == EB_RBUFFERS_TABLE_OVERFLOW {
        dev_err((*(*chip).card).dev, b"lx_buffer_give EB_RBUFFERS_TABLE_OVERFLOW\n\0".as_ptr());
    }

    if err == EB_INVALID_STREAM {
        dev_err((*(*chip).card).dev, b"lx_buffer_give EB_INVALID_STREAM\n\0".as_ptr());
    }

    if err == EB_CMD_REFUSED {
        dev_err((*(*chip).card).dev, b"lx_buffer_give EB_CMD_REFUSED\n\0".as_ptr());
    }

    err
}

#[no_mangle]
pub unsafe extern "C" fn lx_buffer_free(chip: *mut lx6464es, pipe: u32, is_capture: c_int, r_buffer_size: *mut u32) -> c_int {
    let err: c_int;
    let pipe_cmd = PIPE_INFO_TO_CMD(is_capture, pipe);

    guard_mutex(&mut (*chip).msg_lock as *mut _ as *mut c_void);
    lx_message_init(&mut (*chip).rmh, CMD_11_CANCEL_BUFFER);

    (*chip).rmh.cmd[0] |= pipe_cmd;
    (*chip).rmh.cmd[0] |= MASK_BUFFER_ID; /* ask for the current buffer: the
                                           * microblaze will seek for it */

    err = lx_message_send_atomic(chip, &mut (*chip).rmh);

    if err == 0 {
        *r_buffer_size = (*chip).rmh.stat[0] & MASK_DATA_SIZE;
    }

    err
}

#[no_mangle]
pub unsafe extern "C" fn lx_buffer_cancel(chip: *mut lx6464es, pipe: u32, is_capture: c_int, buffer_index: u32) -> c_int {
    let pipe_cmd = PIPE_INFO_TO_CMD(is_capture, pipe);

    guard_mutex(&mut (*chip).msg_lock as *mut _ as *mut c_void);
    lx_message_init(&mut (*chip).rmh, CMD_11_CANCEL_BUFFER);

    (*chip).rmh.cmd[0] |= pipe_cmd;
    (*chip).rmh.cmd[0] |= buffer_index;

    lx_message_send_atomic(chip, &mut (*chip).rmh)
}

/* low-level gain/peak handling
 *
 * \todo: can we unmute capture/playback channels independently?
 *
 * */
#[no_mangle]
pub unsafe extern "C" fn lx_level_unmute(chip: *mut lx6464es, is_capture: c_int, unmute: c_int) -> c_int {
    /* bit set to 1: channel muted */
    let mute_mask: u64 = if unmute != 0 { 0 } else { 0xFFFFFFFFFFFFFFFF };

    guard_mutex(&mut (*chip).msg_lock as *mut _ as *mut c_void);
    lx_message_init(&mut (*chip).rmh, CMD_0D_SET_MUTE);

    (*chip).rmh.cmd[0] |= PIPE_INFO_TO_CMD(is_capture, 0);

    (*chip).rmh.cmd[1] = (mute_mask >> 32) as u32; /* hi part */
    (*chip).rmh.cmd[2] = (mute_mask & 0xFFFFFFFF) as u32; /* lo part */

    dev_dbg(
        (*(*chip).card).dev,
        b"mute %x %x %x\n\0".as_ptr(),
        (*chip).rmh.cmd[0],
        (*chip).rmh.cmd[1],
        (*chip).rmh.cmd[2],
    );

    lx_message_send_atomic(chip, &mut (*chip).rmh)
}

static PEAK_MAP: [u32; 16] = [
    0x00000109, /* -90.308dB */
    0x0000083B, /* -72.247dB */
    0x000020C4, /* -60.205dB */
    0x00008273, /* -48.030dB */
    0x00020756, /* -36.005dB */
    0x00040C37, /* -30.001dB */
    0x00081385, /* -24.002dB */
    0x00101D3F, /* -18.000dB */
    0x0016C310, /* -15.000dB */
    0x002026F2, /* -12.001dB */
    0x002D6A86, /* -9.000dB */
    0x004026E6, /* -6.004dB */
    0x005A9DF6, /* -3.000dB */
    0x0065AC8B, /* -2.000dB */
    0x00721481, /* -1.000dB */
    0x007FFFFF, /* FS */
];

#[no_mangle]
pub unsafe extern "C" fn lx_level_peaks(chip: *mut lx6464es, is_capture: c_int, channels: c_int, mut r_levels: *mut u32) -> c_int {
    let mut err: c_int = 0;
    let mut i: c_int;

    guard_mutex(&mut (*chip).msg_lock as *mut _ as *mut c_void);
    i = 0;
    while i < channels {
        let s0: u32;
        let s1: u32;
        let s2: u32;
        let s3: u32;

        lx_message_init(&mut (*chip).rmh, CMD_12_GET_PEAK);
        (*chip).rmh.cmd[0] |= PIPE_INFO_TO_CMD(is_capture, i as u32);

        err = lx_message_send_atomic(chip, &mut (*chip).rmh);

        if err == 0 {
            s0 = PEAK_MAP[((*chip).rmh.stat[0] & 0x0F) as usize];
            s1 = PEAK_MAP[(((*chip).rmh.stat[0] >> 4) & 0xf) as usize];
            s2 = PEAK_MAP[(((*chip).rmh.stat[0] >> 8) & 0xf) as usize];
            s3 = PEAK_MAP[(((*chip).rmh.stat[0] >> 12) & 0xf) as usize];
        } else {
            s0 = 0;
            s1 = 0;
            s2 = 0;
            s3 = 0;
        }

        *r_levels.add(0) = s0;
        *r_levels.add(1) = s1;
        *r_levels.add(2) = s2;
        *r_levels.add(3) = s3;

        r_levels = r_levels.add(4);
        i += 4;
    }

    err
}

/* interrupt handling */
const PCX_IRQ_NONE: u32 = 0;
const IRQCS_ACTIVE_PCIDB: u32 = 1 << 13;
const IRQCS_ENABLE_PCIIRQ: u32 = 1 << 8;
const IRQCS_ENABLE_PCIDB: u32 = 1 << 9;

unsafe fn lx_interrupt_test_ack(chip: *mut lx6464es) -> u32 {
    let mut irqcs = lx_plx_reg_read(chip, ePLX_IRQCS) as u32;

    /* Test if PCI Doorbell interrupt is active */
    if (irqcs & IRQCS_ACTIVE_PCIDB) != 0 {
        let mut temp: u32;
        irqcs = PCX_IRQ_NONE;

        loop {
            temp = lx_plx_reg_read(chip, ePLX_L2PCIDB) as u32;
            if temp == 0 {
                break;
            }
            /* RAZ interrupt */
            irqcs |= temp;
            lx_plx_reg_write(chip, ePLX_L2PCIDB, temp);
        }

        return irqcs;
    }
    PCX_IRQ_NONE
}

unsafe fn lx_interrupt_ack(chip: *mut lx6464es, r_irqsrc: *mut u32, r_async_pending: *mut c_int, r_async_escmd: *mut c_int) -> c_int {
    let mut irq_async: u32;
    let irqsrc = lx_interrupt_test_ack(chip);

    if irqsrc == PCX_IRQ_NONE {
        return 0;
    }

    *r_irqsrc = irqsrc;

    irq_async = irqsrc & MASK_SYS_ASYNC_EVENTS; /* + EtherSound response
                                                 * (set by xilinx) + EOB */

    if (irq_async & MASK_SYS_STATUS_ESA) != 0 {
        irq_async &= !MASK_SYS_STATUS_ESA;
        *r_async_escmd = 1;
    }

    if irq_async != 0 {
        /* dev_dbg(chip->card->dev, "interrupt: async event pending\n"); */
        *r_async_pending = 1;
    }

    1
}

unsafe fn lx_interrupt_handle_async_events(
    chip: *mut lx6464es,
    irqsrc: u32,
    r_freq_changed: *mut c_int,
    r_notified_in_pipe_mask: *mut u64,
    r_notified_out_pipe_mask: *mut u64,
) -> c_int {
    let err: c_int;
    let mut stat: [u32; 9] = [0; 9]; /* answer from CMD_04_GET_EVENT */

    /* We can optimize this to not read dumb events.
     * Answer words are in the following order:
     * Stat[0]	general status
     * Stat[1]	end of buffer OUT pF
     * Stat[2]	end of buffer OUT pf
     * Stat[3]	end of buffer IN pF
     * Stat[4]	end of buffer IN pf
     * Stat[5]	MSB underrun
     * Stat[6]	LSB underrun
     * Stat[7]	MSB overrun
     * Stat[8]	LSB overrun
     * */

    let eb_pending_out: c_int = if (irqsrc & MASK_SYS_STATUS_EOBO) != 0 { 1 } else { 0 };
    let eb_pending_in: c_int = if (irqsrc & MASK_SYS_STATUS_EOBI) != 0 { 1 } else { 0 };

    *r_freq_changed = if (irqsrc & MASK_SYS_STATUS_FREQ) != 0 { 1 } else { 0 };

    err = lx_dsp_read_async_events(chip, stat.as_mut_ptr());
    if err < 0 {
        return err;
    }

    if eb_pending_in != 0 {
        *r_notified_in_pipe_mask = ((stat[3] as u64) << 32) + stat[4] as u64;
        dev_dbg((*(*chip).card).dev, b"interrupt: EOBI pending %llx\n\0".as_ptr(), *r_notified_in_pipe_mask);
    }
    if eb_pending_out != 0 {
        *r_notified_out_pipe_mask = ((stat[1] as u64) << 32) + stat[2] as u64;
        dev_dbg((*(*chip).card).dev, b"interrupt: EOBO pending %llx\n\0".as_ptr(), *r_notified_out_pipe_mask);
    }

    /* todo: handle xrun notification */

    err
}

unsafe fn lx_interrupt_request_new_buffer(chip: *mut lx6464es, lx_stream: *mut lx_stream) -> c_int {
    let substream: *mut snd_pcm_substream = (*lx_stream).stream;
    let is_capture: c_uint = (*lx_stream).is_capture;
    let mut err: c_int;

    let period_bytes: u32 = snd_pcm_lib_period_bytes(substream) as u32;
    let pos: u32 = (*lx_stream).frame_pos;
    let next_pos: u32 = if pos.wrapping_add(1) == (*(*substream).runtime).periods { 0 } else { pos + 1 };

    let buf: dma_addr_t = (*substream).dma_buffer.addr + pos as usize * period_bytes as usize;
    let mut buf_hi: u32 = 0;
    let mut buf_lo: u32 = 0;
    let mut buffer_index: u32 = 0;

    let mut needed: u32 = 0;
    let mut freed: u32 = 0;
    let mut size_array: [u32; 32] = [0; 32];

    dev_dbg((*(*chip).card).dev, b"->lx_interrupt_request_new_buffer\n\0".as_ptr());

    guard_mutex(&mut (*chip).lock as *mut _ as *mut c_void);

    err = lx_buffer_ask(chip, 0, is_capture as c_int, &mut needed, &mut freed, size_array.as_mut_ptr());
    dev_dbg((*(*chip).card).dev, b"interrupt: needed %d, freed %d\n\0".as_ptr(), needed, freed);

    unpack_pointer(buf, &mut buf_lo, &mut buf_hi);
    err = lx_buffer_give(chip, 0, is_capture as c_int, period_bytes, buf_lo, buf_hi, &mut buffer_index);
    dev_dbg(
        (*(*chip).card).dev,
        b"interrupt: gave buffer index %x on 0x%lx (%d bytes)\n\0".as_ptr(),
        buffer_index,
        buf as c_ulong,
        period_bytes,
    );

    (*lx_stream).frame_pos = next_pos;

    err
}

#[no_mangle]
pub unsafe extern "C" fn lx_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let chip: *mut lx6464es = dev_id as *mut lx6464es;
    let mut async_pending: c_int = 0;
    let mut async_escmd: c_int = 0;
    let mut irqsrc: u32 = 0;
    let mut wake_thread = false;

    dev_dbg((*(*chip).card).dev, b"**************************************************\n\0".as_ptr());

    if lx_interrupt_ack(chip, &mut irqsrc, &mut async_pending, &mut async_escmd) == 0 {
        dev_dbg((*(*chip).card).dev, b"IRQ_NONE\n\0".as_ptr());
        return IRQ_NONE; /* this device did not cause the interrupt */
    }

    if (irqsrc & MASK_SYS_STATUS_CMD_DONE) != 0 {
        return IRQ_HANDLED;
    }

    if (irqsrc & MASK_SYS_STATUS_EOBI) != 0 {
        dev_dbg((*(*chip).card).dev, b"interrupt: EOBI\n\0".as_ptr());
    }

    if (irqsrc & MASK_SYS_STATUS_EOBO) != 0 {
        dev_dbg((*(*chip).card).dev, b"interrupt: EOBO\n\0".as_ptr());
    }

    if (irqsrc & MASK_SYS_STATUS_URUN) != 0 {
        dev_dbg((*(*chip).card).dev, b"interrupt: URUN\n\0".as_ptr());
    }

    if (irqsrc & MASK_SYS_STATUS_ORUN) != 0 {
        dev_dbg((*(*chip).card).dev, b"interrupt: ORUN\n\0".as_ptr());
    }

    if async_pending != 0 {
        wake_thread = true;
        (*chip).irqsrc = irqsrc;
    }

    if async_escmd != 0 {
        /* backdoor for ethersound commands
         *
         * for now, we do not need this
         *
         * */

        dev_dbg((*(*chip).card).dev, b"interrupt requests escmd handling\n\0".as_ptr());
    }

    if wake_thread { IRQ_WAKE_THREAD } else { IRQ_HANDLED }
}

#[no_mangle]
pub unsafe extern "C" fn lx_threaded_irq(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let chip: *mut lx6464es = dev_id as *mut lx6464es;
    let mut notified_in_pipe_mask: u64 = 0;
    let mut notified_out_pipe_mask: u64 = 0;
    let mut freq_changed: c_int = 0;
    let mut err: c_int;

    /* handle async events */
    err = lx_interrupt_handle_async_events(
        chip,
        (*chip).irqsrc,
        &mut freq_changed,
        &mut notified_in_pipe_mask,
        &mut notified_out_pipe_mask,
    );
    if err != 0 {
        dev_err((*(*chip).card).dev, b"error handling async events\n\0".as_ptr());
    }

    if notified_in_pipe_mask != 0 {
        let lx_stream: *mut lx_stream = &mut (*chip).capture_stream;

        dev_dbg((*(*chip).card).dev, b"requesting audio transfer for capture\n\0".as_ptr());
        err = lx_interrupt_request_new_buffer(chip, lx_stream);
        if err < 0 {
            dev_err((*(*chip).card).dev, b"cannot request new buffer for capture\n\0".as_ptr());
        }
        snd_pcm_period_elapsed((*lx_stream).stream);
    }

    if notified_out_pipe_mask != 0 {
        let lx_stream: *mut lx_stream = &mut (*chip).playback_stream;

        dev_dbg((*(*chip).card).dev, b"requesting audio transfer for playback\n\0".as_ptr());
        err = lx_interrupt_request_new_buffer(chip, lx_stream);
        if err < 0 {
            dev_err((*(*chip).card).dev, b"cannot request new buffer for playback\n\0".as_ptr());
        }
        snd_pcm_period_elapsed((*lx_stream).stream);
    }

    IRQ_HANDLED
}

unsafe fn lx_irq_set(chip: *mut lx6464es, enable: c_int) {
    let mut reg = lx_plx_reg_read(chip, ePLX_IRQCS) as u32;

    /* enable/disable interrupts
     *
     * Set the Doorbell and PCI interrupt enable bits
     *
     * */
    if enable != 0 {
        reg |= IRQCS_ENABLE_PCIIRQ | IRQCS_ENABLE_PCIDB;
    } else {
        reg &= !(IRQCS_ENABLE_PCIIRQ | IRQCS_ENABLE_PCIDB);
    }
    lx_plx_reg_write(chip, ePLX_IRQCS, reg);
}

#[no_mangle]
pub unsafe extern "C" fn lx_irq_enable(chip: *mut lx6464es) {
    dev_dbg((*(*chip).card).dev, b"->lx_irq_enable\n\0".as_ptr());
    lx_irq_set(chip, 1);
}

#[no_mangle]
pub unsafe extern "C" fn lx_irq_disable(chip: *mut lx6464es) {
    dev_dbg((*(*chip).card).dev, b"->lx_irq_disable\n\0".as_ptr());
    lx_irq_set(chip, 0);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
