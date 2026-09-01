// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Digigram pcxhr compatible soundcards
 *
 * low level interface with interrupt and message handling implementation
 *
 * Copyright (c) 2004 by Digigram <alsa@digigram.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type u16 = u16;
type u32 = u32;
type u_int32_t = u32;
type u_int64_t = u64;
type bool_ = bool;

const EIO: c_int = 5;
const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const HZ: c_ulong = 100;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
}

#[repr(C)]
pub struct firmware {
    pub size: usize,
    pub data: *const u8,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub period_size: u_int64_t,
    pub periods: u_int64_t,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub number: c_int,
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct pcxhr_hostport {
    pub addr: c_uint,
}

#[repr(C)]
pub struct pcxhr_pipe {
    pub is_capture: c_int,
    pub first_audio: c_uint,
}

#[repr(C)]
pub struct pcxhr_stream {
    pub pipe: *mut pcxhr_pipe,
    pub substream: *mut snd_pcm_substream,
    pub status: c_int,
    pub timer_abs_periods: u_int64_t,
    pub timer_period_frag: u_int32_t,
    pub timer_buf_periods: u_int64_t,
    pub timer_is_synced: c_int,
}

#[repr(C)]
pub struct snd_pcxhr {
    pub nb_streams_capt: c_int,
    pub nb_streams_play: c_int,
    pub capture_stream: *mut pcxhr_stream,
    pub playback_stream: *mut pcxhr_stream,
}

#[repr(C)]
pub struct pcxhr_rmh {
    pub cmd: [u32; PCXHR_SIZE_MAX_CMD],
    pub stat: [u32; PCXHR_SIZE_MAX_LONG_STATUS],
    pub cmd_len: c_int,
    pub stat_len: c_int,
    pub dsp_stat: u16,
    pub cmd_idx: c_int,
}

#[repr(C)]
pub struct pcxhr_mgr {
    pub port: [c_ulong; 3],
    pub pci: *mut pci_dev,
    pub hostport: pcxhr_hostport,
    pub msg_lock: mutex,
    pub lock: mutex,
    pub io_num_reg_cont: c_uint,
    pub src_it_dsp: c_uint,
    pub prmh: *mut pcxhr_rmh,
    pub async_err_stream_xrun: c_int,
    pub async_err_pipe_xrun: c_int,
    pub async_err_other_last: c_int,
    pub timer_toggle: c_int,
    pub dsp_time_err: c_int,
    pub dsp_time_last: c_int,
    pub granularity: c_int,
    pub num_cards: c_int,
    pub chip: *mut *mut snd_pcxhr,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irqreturn_t {
    IRQ_NONE = 0,
    IRQ_HANDLED = 1,
    IRQ_WAKE_THREAD = 2,
}

unsafe extern "C" {
    static mut jiffies: c_ulong;

    fn inb(port: c_ulong) -> u8;
    fn inl(port: c_ulong) -> c_uint;
    fn outb(data: u8, port: c_ulong);
    fn outl(data: c_uint, port: c_ulong);
    fn mdelay(msecs: c_uint);
    fn msleep(msecs: c_uint);
    fn udelay(usecs: c_uint);
    fn cond_resched();
    fn time_after_eq(a: c_ulong, b: c_ulong) -> bool_;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_BUG_ON(cond: bool_) -> c_int;
    fn dev_dbg(dev: *const device, fmt: *const c_char, ...);
    fn dev_err(dev: *const device, fmt: *const c_char, ...);
}

const CMD_VERSION: usize = 0;
const CMD_SUPPORTED: usize = 1;
const CMD_TEST_IT: usize = 2;
const CMD_SEND_IRQA: usize = 3;
const CMD_ACCESS_IO_WRITE: usize = 4;
const CMD_ACCESS_IO_READ: usize = 5;
const CMD_ASYNC: usize = 6;
const CMD_MODIFY_CLOCK: usize = 7;
const CMD_RESYNC_AUDIO_INPUTS: usize = 8;
const CMD_GET_DSP_RESOURCES: usize = 9;
const CMD_SET_TIMER_INTERRUPT: usize = 10;
const CMD_RES_PIPE: usize = 11;
const CMD_FREE_PIPE: usize = 12;
const CMD_CONF_PIPE: usize = 13;
const CMD_STOP_PIPE: usize = 14;
const CMD_PIPE_SAMPLE_COUNT: usize = 15;
const CMD_CAN_START_PIPE: usize = 16;
const CMD_START_STREAM: usize = 17;
const CMD_STREAM_OUT_LEVEL_ADJUST: usize = 18;
const CMD_STOP_STREAM: usize = 19;
const CMD_UPDATE_R_BUFFERS: usize = 20;
const CMD_FORMAT_STREAM_OUT: usize = 21;
const CMD_FORMAT_STREAM_IN: usize = 22;
const CMD_STREAM_SAMPLE_COUNT: usize = 23;
const CMD_AUDIO_LEVEL_ADJUST: usize = 24;
const CMD_GET_TIME_CODE: usize = 25;
const CMD_MANAGE_SIGNAL: usize = 26;
const CMD_LAST_INDEX: usize = 27;

const PCXHR_SIZE_MAX_CMD: usize = 16;
const PCXHR_SIZE_MAX_STATUS: c_int = 16;
const PCXHR_SIZE_MAX_LONG_STATUS: usize = 256;
const FIELD_SIZE: c_uint = 8;
const MASK_FIRST_FIELD: c_uint = 0xff;
const MASK_DSP_WORD: c_uint = 0x00ff_ffff;
const IO_NUM_REG_CONT: u32 = 0;
const PCXHR_DSP_TIME_MASK: c_int = 0x00ff_ffff;
const PCXHR_DSP_TIME_INVALID: c_int = -1;
const PCXHR_STREAM_STATUS_RUNNING: c_int = 1;

/* registers used on the PLX (port 1) */
const PCXHR_PLX_OFFSET_MIN: c_uint = 0x40;
const PCXHR_PLX_MBOX0: c_uint = 0x40;
const PCXHR_PLX_MBOX1: c_uint = 0x44;
const PCXHR_PLX_MBOX2: c_uint = 0x48;
const PCXHR_PLX_MBOX3: c_uint = 0x4C;
const PCXHR_PLX_MBOX4: c_uint = 0x50;
const PCXHR_PLX_MBOX5: c_uint = 0x54;
const PCXHR_PLX_MBOX6: c_uint = 0x58;
const PCXHR_PLX_MBOX7: c_uint = 0x5C;
const PCXHR_PLX_L2PCIDB: c_uint = 0x64;
const PCXHR_PLX_IRQCS: c_uint = 0x68;
const PCXHR_PLX_CHIPSC: c_uint = 0x6C;

/* registers used on the DSP (port 2) */
const PCXHR_DSP_ICR: c_uint = 0x00;
const PCXHR_DSP_CVR: c_uint = 0x04;
const PCXHR_DSP_ISR: c_uint = 0x08;
const PCXHR_DSP_IVR: c_uint = 0x0C;
const PCXHR_DSP_RXH: c_uint = 0x14;
const PCXHR_DSP_TXH: c_uint = 0x14;
const PCXHR_DSP_RXM: c_uint = 0x18;
const PCXHR_DSP_TXM: c_uint = 0x18;
const PCXHR_DSP_RXL: c_uint = 0x1C;
const PCXHR_DSP_TXL: c_uint = 0x1C;
const PCXHR_DSP_RESET: c_uint = 0x20;
const PCXHR_DSP_OFFSET_MAX: c_uint = 0x20;

/* access to the card */
const PCXHR_PLX: usize = 1;
const PCXHR_DSP: usize = 2;

#[inline]
fn PCXHR_REG_TO_PORT(x: c_uint) -> usize {
    if x > PCXHR_DSP_OFFSET_MAX { PCXHR_PLX } else { PCXHR_DSP }
}

#[inline]
unsafe fn PCXHR_INPB(mgr: *mut pcxhr_mgr, x: c_uint) -> u8 {
    unsafe { inb((*mgr).port[PCXHR_REG_TO_PORT(x)].wrapping_add(x as c_ulong)) }
}

#[inline]
unsafe fn PCXHR_INPL(mgr: *mut pcxhr_mgr, x: c_uint) -> c_uint {
    unsafe { inl((*mgr).port[PCXHR_REG_TO_PORT(x)].wrapping_add(x as c_ulong)) }
}

#[inline]
unsafe fn PCXHR_OUTPB(mgr: *mut pcxhr_mgr, x: c_uint, data: c_uint) {
    unsafe { outb(data as u8, (*mgr).port[PCXHR_REG_TO_PORT(x)].wrapping_add(x as c_ulong)) }
}

#[inline]
unsafe fn PCXHR_OUTPL(mgr: *mut pcxhr_mgr, x: c_uint, data: c_uint) {
    unsafe { outl(data, (*mgr).port[PCXHR_REG_TO_PORT(x)].wrapping_add(x as c_ulong)) }
}

/* params used with PCXHR_PLX_MBOX0 */
const PCXHR_MBOX0_HF5: c_uint = 1 << 0;
const PCXHR_MBOX0_HF4: c_uint = 1 << 1;
const PCXHR_MBOX0_BOOT_HERE: c_uint = 1 << 23;
/* params used with PCXHR_PLX_IRQCS */
const PCXHR_IRQCS_ENABLE_PCIIRQ: c_uint = 1 << 8;
const PCXHR_IRQCS_ENABLE_PCIDB: c_uint = 1 << 9;
const PCXHR_IRQCS_ACTIVE_PCIDB: c_uint = 1 << 13;
/* params used with PCXHR_PLX_CHIPSC */
const PCXHR_CHIPSC_INIT_VALUE: c_uint = 0x100D767E;
const PCXHR_CHIPSC_RESET_XILINX: c_uint = 1 << 16;
const PCXHR_CHIPSC_GPI_USERI: c_uint = 1 << 17;
const PCXHR_CHIPSC_DATA_CLK: c_uint = 1 << 24;
const PCXHR_CHIPSC_DATA_IN: c_uint = 1 << 26;

/* params used with PCXHR_DSP_ICR */
const PCXHR_ICR_HI08_RREQ: u8 = 0x01;
const PCXHR_ICR_HI08_TREQ: u8 = 0x02;
const PCXHR_ICR_HI08_HDRQ: u8 = 0x04;
const PCXHR_ICR_HI08_HF0: u8 = 0x08;
const PCXHR_ICR_HI08_HF1: u8 = 0x10;
const PCXHR_ICR_HI08_HLEND: u8 = 0x20;
const PCXHR_ICR_HI08_INIT: u8 = 0x80;
/* params used with PCXHR_DSP_CVR */
const PCXHR_CVR_HI08_HC: u8 = 0x80;
/* params used with PCXHR_DSP_ISR */
const PCXHR_ISR_HI08_RXDF: u8 = 0x01;
const PCXHR_ISR_HI08_TXDE: u8 = 0x02;
const PCXHR_ISR_HI08_TRDY: u8 = 0x04;
const PCXHR_ISR_HI08_ERR: u8 = 0x08;
const PCXHR_ISR_HI08_CHK: u8 = 0x10;
const PCXHR_ISR_HI08_HREQ: u8 = 0x80;

/* constants used for delay in msec */
const PCXHR_WAIT_DEFAULT: c_uint = 2;
const PCXHR_WAIT_IT: c_uint = 25;
const PCXHR_WAIT_IT_EXTRA: c_uint = 65;

/*
 * pcxhr_check_reg_bit - wait for the specified bit is set/reset on a register
 * @reg: register to check
 * @mask: bit mask
 * @bit: resultant bit to be checked
 * @time: time-out of loop in msec
 *
 * returns zero if a bit matches, or a negative error code.
 */
unsafe fn pcxhr_check_reg_bit(
    mgr: *mut pcxhr_mgr,
    reg: c_uint,
    mask: u8,
    bit: u8,
    time: c_int,
    read: *mut u8,
) -> c_int {
    unsafe {
        let mut i: c_int = 0;
        let end_time: c_ulong = jiffies.wrapping_add(((time as c_ulong) * HZ + 999) / 1000);
        loop {
            *read = PCXHR_INPB(mgr, reg);
            if (*read & mask) == bit {
                if i > 100 {
                    dev_dbg(
                        &(*(*mgr).pci).dev,
                        c"ATTENTION! check_reg(%x) loopcount=%d\n".as_ptr(),
                        reg,
                        i,
                    );
                }
                return 0;
            }
            i += 1;
            if !time_after_eq(end_time, jiffies) {
                break;
            }
        }
        dev_err(
            &(*(*mgr).pci).dev,
            c"pcxhr_check_reg_bit: timeout, reg=%x, mask=0x%x, val=%x\n".as_ptr(),
            reg,
            mask as c_uint,
            *read as c_uint,
        );
        -EIO
    }
}

/* constants used with pcxhr_check_reg_bit() */
const PCXHR_TIMEOUT_DSP: c_int = 200;

const PCXHR_MASK_EXTRA_INFO: c_uint = 0x0000FE;
const PCXHR_MASK_IT_HF0: c_uint = 0x000100;
const PCXHR_MASK_IT_HF1: c_uint = 0x000200;
const PCXHR_MASK_IT_NO_HF0_HF1: c_uint = 0x000400;
const PCXHR_MASK_IT_MANAGE_HF5: c_uint = 0x000800;
const PCXHR_MASK_IT_WAIT: c_uint = 0x010000;
const PCXHR_MASK_IT_WAIT_EXTRA: c_uint = 0x020000;

const PCXHR_IT_SEND_BYTE_XILINX: c_uint = 0x0000003C | PCXHR_MASK_IT_HF0;
const PCXHR_IT_TEST_XILINX: c_uint =
    0x0000003C | PCXHR_MASK_IT_HF1 | PCXHR_MASK_IT_MANAGE_HF5;
const PCXHR_IT_DOWNLOAD_BOOT: c_uint =
    0x0000000C | PCXHR_MASK_IT_HF1 | PCXHR_MASK_IT_MANAGE_HF5 | PCXHR_MASK_IT_WAIT;
const PCXHR_IT_RESET_BOARD_FUNC: c_uint =
    0x0000000C | PCXHR_MASK_IT_HF0 | PCXHR_MASK_IT_MANAGE_HF5 | PCXHR_MASK_IT_WAIT_EXTRA;
const PCXHR_IT_DOWNLOAD_DSP: c_uint =
    0x0000000C | PCXHR_MASK_IT_MANAGE_HF5 | PCXHR_MASK_IT_WAIT;
const PCXHR_IT_DEBUG: c_uint = 0x0000005A | PCXHR_MASK_IT_NO_HF0_HF1;
const PCXHR_IT_RESET_SEMAPHORE: c_uint = 0x0000005C | PCXHR_MASK_IT_NO_HF0_HF1;
const PCXHR_IT_MESSAGE: c_uint = 0x00000074 | PCXHR_MASK_IT_NO_HF0_HF1;
const PCXHR_IT_RESET_CHK: c_uint = 0x00000076 | PCXHR_MASK_IT_NO_HF0_HF1;
const PCXHR_IT_UPDATE_RBUFFER: c_uint = 0x00000078 | PCXHR_MASK_IT_NO_HF0_HF1;

unsafe fn pcxhr_send_it_dsp(mgr: *mut pcxhr_mgr, itdsp: c_uint, atomic: c_int) -> c_int {
    unsafe {
        let mut err: c_int;
        let mut reg: u8;

        if itdsp & PCXHR_MASK_IT_MANAGE_HF5 != 0 {
            /* clear hf5 bit */
            let val = PCXHR_INPL(mgr, PCXHR_PLX_MBOX0) & !PCXHR_MBOX0_HF5;
            PCXHR_OUTPL(mgr, PCXHR_PLX_MBOX0, val);
        }
        if (itdsp & PCXHR_MASK_IT_NO_HF0_HF1) == 0 {
            reg = PCXHR_ICR_HI08_RREQ | PCXHR_ICR_HI08_TREQ | PCXHR_ICR_HI08_HDRQ;
            if itdsp & PCXHR_MASK_IT_HF0 != 0 {
                reg |= PCXHR_ICR_HI08_HF0;
            }
            if itdsp & PCXHR_MASK_IT_HF1 != 0 {
                reg |= PCXHR_ICR_HI08_HF1;
            }
            PCXHR_OUTPB(mgr, PCXHR_DSP_ICR, reg as c_uint);
        }
        reg = (((itdsp & PCXHR_MASK_EXTRA_INFO) >> 1) as u8) | PCXHR_CVR_HI08_HC;
        PCXHR_OUTPB(mgr, PCXHR_DSP_CVR, reg as c_uint);
        if itdsp & PCXHR_MASK_IT_WAIT != 0 {
            if atomic != 0 {
                mdelay(PCXHR_WAIT_IT);
            } else {
                msleep(PCXHR_WAIT_IT);
            }
        }
        if itdsp & PCXHR_MASK_IT_WAIT_EXTRA != 0 {
            if atomic != 0 {
                mdelay(PCXHR_WAIT_IT_EXTRA);
            } else {
                msleep(PCXHR_WAIT_IT);
            }
        }
        /* wait for CVR_HI08_HC == 0 */
        err = pcxhr_check_reg_bit(
            mgr,
            PCXHR_DSP_CVR,
            PCXHR_CVR_HI08_HC,
            0,
            PCXHR_TIMEOUT_DSP,
            &mut reg,
        );
        if err != 0 {
            dev_err(&(*(*mgr).pci).dev, c"pcxhr_send_it_dsp : TIMEOUT CVR\n".as_ptr());
            return err;
        }
        if itdsp & PCXHR_MASK_IT_MANAGE_HF5 != 0 {
            /* wait for hf5 bit */
            err = pcxhr_check_reg_bit(
                mgr,
                PCXHR_PLX_MBOX0,
                PCXHR_MBOX0_HF5 as u8,
                PCXHR_MBOX0_HF5 as u8,
                PCXHR_TIMEOUT_DSP,
                &mut reg,
            );
            if err != 0 {
                dev_err(&(*(*mgr).pci).dev, c"pcxhr_send_it_dsp : TIMEOUT HF5\n".as_ptr());
                return err;
            }
        }
        0 /* retry not handled here */
    }
}

#[no_mangle]
pub unsafe extern "C" fn pcxhr_reset_xilinx_com(mgr: *mut pcxhr_mgr) {
    unsafe {
        /* reset second xilinx */
        PCXHR_OUTPL(
            mgr,
            PCXHR_PLX_CHIPSC,
            PCXHR_CHIPSC_INIT_VALUE & !PCXHR_CHIPSC_RESET_XILINX,
        );
    }
}

unsafe fn pcxhr_enable_irq(mgr: *mut pcxhr_mgr, enable: c_int) {
    unsafe {
        let mut reg = PCXHR_INPL(mgr, PCXHR_PLX_IRQCS);
        /* enable/disable interrupts */
        if enable != 0 {
            reg |= PCXHR_IRQCS_ENABLE_PCIIRQ | PCXHR_IRQCS_ENABLE_PCIDB;
        } else {
            reg &= !(PCXHR_IRQCS_ENABLE_PCIIRQ | PCXHR_IRQCS_ENABLE_PCIDB);
        }
        PCXHR_OUTPL(mgr, PCXHR_PLX_IRQCS, reg);
    }
}

#[no_mangle]
pub unsafe extern "C" fn pcxhr_reset_dsp(mgr: *mut pcxhr_mgr) {
    unsafe {
        /* disable interrupts */
        pcxhr_enable_irq(mgr, 0);

        /* let's reset the DSP */
        PCXHR_OUTPB(mgr, PCXHR_DSP_RESET, 0);
        msleep(PCXHR_WAIT_DEFAULT); /* wait 2 msec */
        PCXHR_OUTPB(mgr, PCXHR_DSP_RESET, 3);
        msleep(PCXHR_WAIT_DEFAULT); /* wait 2 msec */

        /* reset mailbox */
        PCXHR_OUTPL(mgr, PCXHR_PLX_MBOX0, 0);
    }
}

#[no_mangle]
pub unsafe extern "C" fn pcxhr_enable_dsp(mgr: *mut pcxhr_mgr) {
    unsafe {
        /* enable interrupts */
        pcxhr_enable_irq(mgr, 1);
    }
}

/*
 * load the xilinx image
 */
#[no_mangle]
pub unsafe extern "C" fn pcxhr_load_xilinx_binary(
    mgr: *mut pcxhr_mgr,
    xilinx: *const firmware,
    second: c_int,
) -> c_int {
    unsafe {
        let mut i: c_uint;
        let mut chipsc: c_uint;
        let mut data: u8;
        let mut mask: u8;
        let mut image: *const u8;

        /* test first xilinx */
        chipsc = PCXHR_INPL(mgr, PCXHR_PLX_CHIPSC);
        /* REV01 cards do not support the PCXHR_CHIPSC_GPI_USERI bit anymore */
        /* this bit will always be 1;
         * no possibility to test presence of first xilinx
         */
        if second != 0 {
            if (chipsc & PCXHR_CHIPSC_GPI_USERI) == 0 {
                dev_err(&(*(*mgr).pci).dev, c"error loading first xilinx\n".as_ptr());
                return -EINVAL;
            }
            /* activate second xilinx */
            chipsc |= PCXHR_CHIPSC_RESET_XILINX;
            PCXHR_OUTPL(mgr, PCXHR_PLX_CHIPSC, chipsc);
            msleep(PCXHR_WAIT_DEFAULT); /* wait 2 msec */
        }
        image = (*xilinx).data;
        i = 0;
        while (i as usize) < (*xilinx).size {
            data = *image;
            mask = 0x80;
            while mask != 0 {
                chipsc &= !(PCXHR_CHIPSC_DATA_CLK | PCXHR_CHIPSC_DATA_IN);
                if data & mask != 0 {
                    chipsc |= PCXHR_CHIPSC_DATA_IN;
                }
                PCXHR_OUTPL(mgr, PCXHR_PLX_CHIPSC, chipsc);
                chipsc |= PCXHR_CHIPSC_DATA_CLK;
                PCXHR_OUTPL(mgr, PCXHR_PLX_CHIPSC, chipsc);
                mask >>= 1;
            }
            /* don't take too much time in this loop... */
            cond_resched();
            i += 1;
            image = image.add(1);
        }
        chipsc &= !(PCXHR_CHIPSC_DATA_CLK | PCXHR_CHIPSC_DATA_IN);
        PCXHR_OUTPL(mgr, PCXHR_PLX_CHIPSC, chipsc);
        /* wait 2 msec (time to boot the xilinx before any access) */
        msleep(PCXHR_WAIT_DEFAULT);
        0
    }
}

/*
 * send an executable file to the DSP
 */
unsafe fn pcxhr_download_dsp(mgr: *mut pcxhr_mgr, dsp: *const firmware) -> c_int {
    unsafe {
        let mut err: c_int;
        let mut i: c_uint;
        let mut len: c_uint;
        let mut data: *const u8;
        let mut dummy: u8 = 0;
        /* check the length of boot image */
        if (*dsp).size <= 0 {
            return -EINVAL;
        }
        if (*dsp).size % 3 != 0 {
            return -EINVAL;
        }
        if snd_BUG_ON((*dsp).data.is_null()) != 0 {
            return -EINVAL;
        }
        /* transfert data buffer from PC to DSP */
        i = 0;
        while (i as usize) < (*dsp).size {
            data = (*dsp).data.add(i as usize);
            if i == 0 {
                /* test data header consistency */
                len = (((*data.add(0) as c_uint) << 16)
                    + ((*data.add(1) as c_uint) << 8)
                    + (*data.add(2) as c_uint)) as c_uint;
                if len != 0 && (*dsp).size != ((len + 2) * 3) as usize {
                    return -EINVAL;
                }
            }
            /* wait DSP ready for new transfer */
            err = pcxhr_check_reg_bit(
                mgr,
                PCXHR_DSP_ISR,
                PCXHR_ISR_HI08_TRDY,
                PCXHR_ISR_HI08_TRDY,
                PCXHR_TIMEOUT_DSP,
                &mut dummy,
            );
            if err != 0 {
                dev_err(
                    &(*(*mgr).pci).dev,
                    c"dsp loading error at position %d\n".as_ptr(),
                    i as c_int,
                );
                return err;
            }
            /* send host data */
            PCXHR_OUTPB(mgr, PCXHR_DSP_TXH, *data.add(0) as c_uint);
            PCXHR_OUTPB(mgr, PCXHR_DSP_TXM, *data.add(1) as c_uint);
            PCXHR_OUTPB(mgr, PCXHR_DSP_TXL, *data.add(2) as c_uint);

            /* don't take too much time in this loop... */
            cond_resched();
            i += 3;
        }
        /* give some time to boot the DSP */
        msleep(PCXHR_WAIT_DEFAULT);
        0
    }
}

/*
 * load the eeprom image
 */
#[no_mangle]
pub unsafe extern "C" fn pcxhr_load_eeprom_binary(
    mgr: *mut pcxhr_mgr,
    eeprom: *const firmware,
) -> c_int {
    unsafe {
        let mut err: c_int;
        let mut reg: u8;

        /* init value of the ICR register */
        reg = PCXHR_ICR_HI08_RREQ | PCXHR_ICR_HI08_TREQ | PCXHR_ICR_HI08_HDRQ;
        if PCXHR_INPL(mgr, PCXHR_PLX_MBOX0) & PCXHR_MBOX0_BOOT_HERE != 0 {
            /* no need to load the eeprom binary,
             * but init the HI08 interface
             */
            PCXHR_OUTPB(mgr, PCXHR_DSP_ICR, (reg | PCXHR_ICR_HI08_INIT) as c_uint);
            msleep(PCXHR_WAIT_DEFAULT);
            PCXHR_OUTPB(mgr, PCXHR_DSP_ICR, reg as c_uint);
            msleep(PCXHR_WAIT_DEFAULT);
            dev_dbg(&(*(*mgr).pci).dev, c"no need to load eeprom boot\n".as_ptr());
            return 0;
        }
        PCXHR_OUTPB(mgr, PCXHR_DSP_ICR, reg as c_uint);

        err = pcxhr_download_dsp(mgr, eeprom);
        if err != 0 {
            return err;
        }
        /* wait for chk bit */
        pcxhr_check_reg_bit(
            mgr,
            PCXHR_DSP_ISR,
            PCXHR_ISR_HI08_CHK,
            PCXHR_ISR_HI08_CHK,
            PCXHR_TIMEOUT_DSP,
            &mut reg,
        )
    }
}

/*
 * load the boot image
 */
#[no_mangle]
pub unsafe extern "C" fn pcxhr_load_boot_binary(
    mgr: *mut pcxhr_mgr,
    boot: *const firmware,
) -> c_int {
    unsafe {
        let mut err: c_int;
        let physaddr: c_uint = (*mgr).hostport.addr;
        let mut dummy: u8 = 0;

        /* send the hostport address to the DSP (only the upper 24 bit !) */
        if snd_BUG_ON((physaddr & 0xff) != 0) != 0 {
            return -EINVAL;
        }
        PCXHR_OUTPL(mgr, PCXHR_PLX_MBOX1, physaddr >> 8);

        err = pcxhr_send_it_dsp(mgr, PCXHR_IT_DOWNLOAD_BOOT, 0);
        if err != 0 {
            return err;
        }
        /* clear hf5 bit */
        let val = PCXHR_INPL(mgr, PCXHR_PLX_MBOX0) & !PCXHR_MBOX0_HF5;
        PCXHR_OUTPL(mgr, PCXHR_PLX_MBOX0, val);

        err = pcxhr_download_dsp(mgr, boot);
        if err != 0 {
            return err;
        }
        /* wait for hf5 bit */
        pcxhr_check_reg_bit(
            mgr,
            PCXHR_PLX_MBOX0,
            PCXHR_MBOX0_HF5 as u8,
            PCXHR_MBOX0_HF5 as u8,
            PCXHR_TIMEOUT_DSP,
            &mut dummy,
        )
    }
}

/*
 * load the final dsp image
 */
#[no_mangle]
pub unsafe extern "C" fn pcxhr_load_dsp_binary(
    mgr: *mut pcxhr_mgr,
    dsp: *const firmware,
) -> c_int {
    unsafe {
        let mut err: c_int;
        let mut dummy: u8 = 0;
        err = pcxhr_send_it_dsp(mgr, PCXHR_IT_RESET_BOARD_FUNC, 0);
        if err != 0 {
            return err;
        }
        err = pcxhr_send_it_dsp(mgr, PCXHR_IT_DOWNLOAD_DSP, 0);
        if err != 0 {
            return err;
        }
        err = pcxhr_download_dsp(mgr, dsp);
        if err != 0 {
            return err;
        }
        /* wait for chk bit */
        pcxhr_check_reg_bit(
            mgr,
            PCXHR_DSP_ISR,
            PCXHR_ISR_HI08_CHK,
            PCXHR_ISR_HI08_CHK,
            PCXHR_TIMEOUT_DSP,
            &mut dummy,
        )
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct pcxhr_cmd_info {
    opcode: u32,    /* command word */
    st_length: u16, /* status length */
    st_type: u16,   /* status type (RMH_SSIZE_XXX) */
}

/* RMH status type */
const RMH_SSIZE_FIXED: u16 = 0; /* status size fix (st_length = 0..x) */
const RMH_SSIZE_ARG: u16 = 1; /* status size given in the LSB byte */
const RMH_SSIZE_MASK: u16 = 2; /* status size given in bitmask */

/*
 * Array of DSP commands
 */
static pcxhr_dsp_cmds: [pcxhr_cmd_info; CMD_LAST_INDEX] = {
    let z = pcxhr_cmd_info { opcode: 0, st_length: 0, st_type: 0 };
    let mut a = [z; CMD_LAST_INDEX];
    a[CMD_VERSION] = pcxhr_cmd_info { opcode: 0x010000, st_length: 1, st_type: RMH_SSIZE_FIXED };
    a[CMD_SUPPORTED] = pcxhr_cmd_info { opcode: 0x020000, st_length: 4, st_type: RMH_SSIZE_FIXED };
    a[CMD_TEST_IT] = pcxhr_cmd_info { opcode: 0x040000, st_length: 1, st_type: RMH_SSIZE_FIXED };
    a[CMD_SEND_IRQA] = pcxhr_cmd_info { opcode: 0x070001, st_length: 0, st_type: RMH_SSIZE_FIXED };
    a[CMD_ACCESS_IO_WRITE] = pcxhr_cmd_info { opcode: 0x090000, st_length: 1, st_type: RMH_SSIZE_ARG };
    a[CMD_ACCESS_IO_READ] = pcxhr_cmd_info { opcode: 0x094000, st_length: 1, st_type: RMH_SSIZE_ARG };
    a[CMD_ASYNC] = pcxhr_cmd_info { opcode: 0x0a0000, st_length: 1, st_type: RMH_SSIZE_ARG };
    a[CMD_MODIFY_CLOCK] = pcxhr_cmd_info { opcode: 0x0d0000, st_length: 0, st_type: RMH_SSIZE_FIXED };
    a[CMD_RESYNC_AUDIO_INPUTS] = pcxhr_cmd_info { opcode: 0x0e0000, st_length: 0, st_type: RMH_SSIZE_FIXED };
    a[CMD_GET_DSP_RESOURCES] = pcxhr_cmd_info { opcode: 0x100000, st_length: 4, st_type: RMH_SSIZE_FIXED };
    a[CMD_SET_TIMER_INTERRUPT] = pcxhr_cmd_info { opcode: 0x110000, st_length: 0, st_type: RMH_SSIZE_FIXED };
    a[CMD_RES_PIPE] = pcxhr_cmd_info { opcode: 0x400000, st_length: 0, st_type: RMH_SSIZE_FIXED };
    a[CMD_FREE_PIPE] = pcxhr_cmd_info { opcode: 0x410000, st_length: 0, st_type: RMH_SSIZE_FIXED };
    a[CMD_CONF_PIPE] = pcxhr_cmd_info { opcode: 0x422101, st_length: 0, st_type: RMH_SSIZE_FIXED };
    a[CMD_STOP_PIPE] = pcxhr_cmd_info { opcode: 0x470004, st_length: 0, st_type: RMH_SSIZE_FIXED };
    a[CMD_PIPE_SAMPLE_COUNT] = pcxhr_cmd_info { opcode: 0x49a000, st_length: 2, st_type: RMH_SSIZE_FIXED };
    a[CMD_CAN_START_PIPE] = pcxhr_cmd_info { opcode: 0x4b0000, st_length: 1, st_type: RMH_SSIZE_FIXED };
    a[CMD_START_STREAM] = pcxhr_cmd_info { opcode: 0x802000, st_length: 0, st_type: RMH_SSIZE_FIXED };
    a[CMD_STREAM_OUT_LEVEL_ADJUST] = pcxhr_cmd_info { opcode: 0x822000, st_length: 0, st_type: RMH_SSIZE_FIXED };
    a[CMD_STOP_STREAM] = pcxhr_cmd_info { opcode: 0x832000, st_length: 0, st_type: RMH_SSIZE_FIXED };
    a[CMD_UPDATE_R_BUFFERS] = pcxhr_cmd_info { opcode: 0x840000, st_length: 0, st_type: RMH_SSIZE_FIXED };
    a[CMD_FORMAT_STREAM_OUT] = pcxhr_cmd_info { opcode: 0x860000, st_length: 0, st_type: RMH_SSIZE_FIXED };
    a[CMD_FORMAT_STREAM_IN] = pcxhr_cmd_info { opcode: 0x870000, st_length: 0, st_type: RMH_SSIZE_FIXED };
    a[CMD_STREAM_SAMPLE_COUNT] = pcxhr_cmd_info { opcode: 0x902000, st_length: 2, st_type: RMH_SSIZE_FIXED };
    a[CMD_AUDIO_LEVEL_ADJUST] = pcxhr_cmd_info { opcode: 0xc22000, st_length: 0, st_type: RMH_SSIZE_FIXED };
    a[CMD_GET_TIME_CODE] = pcxhr_cmd_info { opcode: 0x060000, st_length: 5, st_type: RMH_SSIZE_FIXED };
    a[CMD_MANAGE_SIGNAL] = pcxhr_cmd_info { opcode: 0x0f0000, st_length: 0, st_type: RMH_SSIZE_FIXED };
    a
};

/* CONFIG_SND_DEBUG_VERBOSE cmd_names[] omitted from executable Rust; debug-only names are preserved by debug comments at call sites. */

unsafe fn pcxhr_read_rmh_status(mgr: *mut pcxhr_mgr, rmh: *mut pcxhr_rmh) -> c_int {
    unsafe {
        let mut err: c_int;
        let mut i: c_int;
        let mut data: u32;
        let mut size_mask: u32;
        let mut reg: u8 = 0;
        let max_stat_len: c_int;

        if (*rmh).stat_len < PCXHR_SIZE_MAX_STATUS {
            max_stat_len = PCXHR_SIZE_MAX_STATUS;
        } else {
            max_stat_len = (*rmh).stat_len;
        }

        i = 0;
        while i < (*rmh).stat_len {
            /* wait for receiver full */
            err = pcxhr_check_reg_bit(
                mgr,
                PCXHR_DSP_ISR,
                PCXHR_ISR_HI08_RXDF,
                PCXHR_ISR_HI08_RXDF,
                PCXHR_TIMEOUT_DSP,
                &mut reg,
            );
            if err != 0 {
                dev_err(
                    &(*(*mgr).pci).dev,
                    c"ERROR RMH stat: ISR:RXDF=1 (ISR = %x; i=%d )\n".as_ptr(),
                    reg as c_uint,
                    i,
                );
                return err;
            }
            /* read data */
            data = (PCXHR_INPB(mgr, PCXHR_DSP_TXH) as u32) << 16;
            data |= (PCXHR_INPB(mgr, PCXHR_DSP_TXM) as u32) << 8;
            data |= PCXHR_INPB(mgr, PCXHR_DSP_TXL) as u32;

            /* need to update rmh->stat_len on the fly ?? */
            if i == 0 {
                if (*rmh).dsp_stat != RMH_SSIZE_FIXED {
                    if (*rmh).dsp_stat == RMH_SSIZE_ARG {
                        (*rmh).stat_len = ((data & 0x0000ff) + 1) as c_int;
                        data &= 0xffff00;
                    } else {
                        /* rmh->dsp_stat == RMH_SSIZE_MASK */
                        (*rmh).stat_len = 1;
                        size_mask = data;
                        while size_mask != 0 {
                            if size_mask & 1 != 0 {
                                (*rmh).stat_len += 1;
                            }
                            size_mask >>= 1;
                        }
                    }
                }
            }
            /* CONFIG_SND_DEBUG_VERBOSE: stat trace */
            if i < max_stat_len {
                (*rmh).stat[i as usize] = data;
            }
            i += 1;
        }
        if (*rmh).stat_len > max_stat_len {
            dev_dbg(
                &(*(*mgr).pci).dev,
                c"PCXHR : rmh->stat_len=%x too big\n".as_ptr(),
                (*rmh).stat_len,
            );
            (*rmh).stat_len = max_stat_len;
        }
        0
    }
}

unsafe fn pcxhr_send_msg_nolock(mgr: *mut pcxhr_mgr, rmh: *mut pcxhr_rmh) -> c_int {
    unsafe {
        let mut err: c_int;
        let mut i: c_int;
        let mut data: u32;
        let mut reg: u8 = 0;

        if snd_BUG_ON((*rmh).cmd_len >= PCXHR_SIZE_MAX_CMD as c_int) != 0 {
            return -EINVAL;
        }
        err = pcxhr_send_it_dsp(mgr, PCXHR_IT_MESSAGE, 1);
        if err != 0 {
            dev_err(&(*(*mgr).pci).dev, c"pcxhr_send_message : ED_DSP_CRASHED\n".as_ptr());
            return err;
        }
        /* wait for chk bit */
        err = pcxhr_check_reg_bit(
            mgr,
            PCXHR_DSP_ISR,
            PCXHR_ISR_HI08_CHK,
            PCXHR_ISR_HI08_CHK,
            PCXHR_TIMEOUT_DSP,
            &mut reg,
        );
        if err != 0 {
            return err;
        }
        /* reset irq chk */
        err = pcxhr_send_it_dsp(mgr, PCXHR_IT_RESET_CHK, 1);
        if err != 0 {
            return err;
        }
        /* wait for chk bit == 0*/
        err = pcxhr_check_reg_bit(
            mgr,
            PCXHR_DSP_ISR,
            PCXHR_ISR_HI08_CHK,
            0,
            PCXHR_TIMEOUT_DSP,
            &mut reg,
        );
        if err != 0 {
            return err;
        }

        data = (*rmh).cmd[0];

        if (*rmh).cmd_len > 1 {
            data |= 0x008000; /* MASK_MORE_THAN_1_WORD_COMMAND */
        } else {
            data &= 0xff7fff; /* MASK_1_WORD_COMMAND */
        }
        /* CONFIG_SND_DEBUG_VERBOSE: MSG cmd[0] trace */

        err = pcxhr_check_reg_bit(
            mgr,
            PCXHR_DSP_ISR,
            PCXHR_ISR_HI08_TRDY,
            PCXHR_ISR_HI08_TRDY,
            PCXHR_TIMEOUT_DSP,
            &mut reg,
        );
        if err != 0 {
            return err;
        }
        PCXHR_OUTPB(mgr, PCXHR_DSP_TXH, (data >> 16) & 0xFF);
        PCXHR_OUTPB(mgr, PCXHR_DSP_TXM, (data >> 8) & 0xFF);
        PCXHR_OUTPB(mgr, PCXHR_DSP_TXL, data & 0xFF);

        if (*rmh).cmd_len > 1 {
            /* send length */
            data = ((*rmh).cmd_len - 1) as u32;
            err = pcxhr_check_reg_bit(
                mgr,
                PCXHR_DSP_ISR,
                PCXHR_ISR_HI08_TRDY,
                PCXHR_ISR_HI08_TRDY,
                PCXHR_TIMEOUT_DSP,
                &mut reg,
            );
            if err != 0 {
                return err;
            }
            PCXHR_OUTPB(mgr, PCXHR_DSP_TXH, (data >> 16) & 0xFF);
            PCXHR_OUTPB(mgr, PCXHR_DSP_TXM, (data >> 8) & 0xFF);
            PCXHR_OUTPB(mgr, PCXHR_DSP_TXL, data & 0xFF);

            i = 1;
            while i < (*rmh).cmd_len {
                /* send other words */
                data = (*rmh).cmd[i as usize];
                /* CONFIG_SND_DEBUG_VERBOSE: command word trace */
                err = pcxhr_check_reg_bit(
                    mgr,
                    PCXHR_DSP_ISR,
                    PCXHR_ISR_HI08_TRDY,
                    PCXHR_ISR_HI08_TRDY,
                    PCXHR_TIMEOUT_DSP,
                    &mut reg,
                );
                if err != 0 {
                    return err;
                }
                PCXHR_OUTPB(mgr, PCXHR_DSP_TXH, (data >> 16) & 0xFF);
                PCXHR_OUTPB(mgr, PCXHR_DSP_TXM, (data >> 8) & 0xFF);
                PCXHR_OUTPB(mgr, PCXHR_DSP_TXL, data & 0xFF);
                i += 1;
            }
        }
        /* wait for chk bit */
        err = pcxhr_check_reg_bit(
            mgr,
            PCXHR_DSP_ISR,
            PCXHR_ISR_HI08_CHK,
            PCXHR_ISR_HI08_CHK,
            PCXHR_TIMEOUT_DSP,
            &mut reg,
        );
        if err != 0 {
            return err;
        }
        /* test status ISR */
        if reg & PCXHR_ISR_HI08_ERR != 0 {
            /* ERROR, wait for receiver full */
            err = pcxhr_check_reg_bit(
                mgr,
                PCXHR_DSP_ISR,
                PCXHR_ISR_HI08_RXDF,
                PCXHR_ISR_HI08_RXDF,
                PCXHR_TIMEOUT_DSP,
                &mut reg,
            );
            if err != 0 {
                dev_err(
                    &(*(*mgr).pci).dev,
                    c"ERROR RMH: ISR:RXDF=1 (ISR = %x)\n".as_ptr(),
                    reg as c_uint,
                );
                return err;
            }
            /* read error code */
            data = (PCXHR_INPB(mgr, PCXHR_DSP_TXH) as u32) << 16;
            data |= (PCXHR_INPB(mgr, PCXHR_DSP_TXM) as u32) << 8;
            data |= PCXHR_INPB(mgr, PCXHR_DSP_TXL) as u32;
            dev_err(
                &(*(*mgr).pci).dev,
                c"ERROR RMH(%d): 0x%x\n".as_ptr(),
                (*rmh).cmd_idx,
                data,
            );
            err = -EINVAL;
        } else {
            /* read the response data */
            err = pcxhr_read_rmh_status(mgr, rmh);
        }
        /* reset semaphore */
        if pcxhr_send_it_dsp(mgr, PCXHR_IT_RESET_SEMAPHORE, 1) < 0 {
            return -EIO;
        }
        err
    }
}

/**
 * pcxhr_init_rmh - initialize the RMH instance
 * @rmh: the rmh pointer to be initialized
 * @cmd: the rmh command to be set
 */
#[no_mangle]
pub unsafe extern "C" fn pcxhr_init_rmh(rmh: *mut pcxhr_rmh, cmd: c_int) {
    unsafe {
        if snd_BUG_ON(cmd >= CMD_LAST_INDEX as c_int) != 0 {
            return;
        }
        (*rmh).cmd[0] = pcxhr_dsp_cmds[cmd as usize].opcode;
        (*rmh).cmd_len = 1;
        (*rmh).stat_len = pcxhr_dsp_cmds[cmd as usize].st_length as c_int;
        (*rmh).dsp_stat = pcxhr_dsp_cmds[cmd as usize].st_type;
        (*rmh).cmd_idx = cmd;
    }
}

#[no_mangle]
pub unsafe extern "C" fn pcxhr_set_pipe_cmd_params(
    rmh: *mut pcxhr_rmh,
    capture: c_int,
    param1: c_uint,
    param2: c_uint,
    param3: c_uint,
) {
    unsafe {
        snd_BUG_ON(param1 > MASK_FIRST_FIELD);
        if capture != 0 {
            (*rmh).cmd[0] |= 0x800; /* COMMAND_RECORD_MASK */
        }
        if param1 != 0 {
            (*rmh).cmd[0] |= param1 << FIELD_SIZE;
        }
        if param2 != 0 {
            snd_BUG_ON(param2 > MASK_FIRST_FIELD);
            (*rmh).cmd[0] |= param2;
        }
        if param3 != 0 {
            snd_BUG_ON(param3 > MASK_DSP_WORD);
            (*rmh).cmd[1] = param3;
            (*rmh).cmd_len = 2;
        }
    }
}

/*
 * pcxhr_send_msg - send a DSP message with spinlock
 * @rmh: the rmh record to send and receive
 *
 * returns 0 if successful, or a negative error code.
 */
#[no_mangle]
pub unsafe extern "C" fn pcxhr_send_msg(mgr: *mut pcxhr_mgr, rmh: *mut pcxhr_rmh) -> c_int {
    unsafe {
        mutex_lock(&mut (*mgr).msg_lock);
        let ret = pcxhr_send_msg_nolock(mgr, rmh);
        mutex_unlock(&mut (*mgr).msg_lock);
        ret
    }
}

#[inline]
unsafe fn pcxhr_pipes_running(mgr: *mut pcxhr_mgr) -> c_int {
    unsafe {
        let mut start_mask: c_int = PCXHR_INPL(mgr, PCXHR_PLX_MBOX2) as c_int;
        /* least segnificant 12 bits are the pipe states
         * for the playback audios
         * next 12 bits are the pipe states for the capture audios
         * (PCXHR_PIPE_STATE_CAPTURE_OFFSET)
         */
        start_mask &= 0xffffff;
        dev_dbg(
            &(*(*mgr).pci).dev,
            c"CMD_PIPE_STATE MBOX2=0x%06x\n".as_ptr(),
            start_mask,
        );
        start_mask
    }
}

const PCXHR_PIPE_STATE_CAPTURE_OFFSET: c_int = 12;
const MAX_WAIT_FOR_DSP: c_int = 20;

unsafe fn pcxhr_prepair_pipe_start(
    mgr: *mut pcxhr_mgr,
    mut audio_mask: c_int,
    retry: *mut c_int,
) -> c_int {
    unsafe {
        let mut rmh: pcxhr_rmh = core::mem::zeroed();
        let mut err: c_int;
        let mut audio: c_int = 0;

        *retry = 0;
        while audio_mask != 0 {
            if audio_mask & 1 != 0 {
                pcxhr_init_rmh(&mut rmh, CMD_CAN_START_PIPE as c_int);
                if audio < PCXHR_PIPE_STATE_CAPTURE_OFFSET {
                    /* can start playback pipe */
                    pcxhr_set_pipe_cmd_params(&mut rmh, 0, audio as c_uint, 0, 0);
                } else {
                    /* can start capture pipe */
                    pcxhr_set_pipe_cmd_params(
                        &mut rmh,
                        1,
                        (audio - PCXHR_PIPE_STATE_CAPTURE_OFFSET) as c_uint,
                        0,
                        0,
                    );
                }
                err = pcxhr_send_msg(mgr, &mut rmh);
                if err != 0 {
                    dev_err(
                        &(*(*mgr).pci).dev,
                        c"error pipe start (CMD_CAN_START_PIPE) err=%x!\n".as_ptr(),
                        err,
                    );
                    return err;
                }
                /* if the pipe couldn't be prepaired for start,
                 * retry it later
                 */
                if rmh.stat[0] == 0 {
                    *retry |= 1 << audio;
                }
            }
            audio_mask >>= 1;
            audio += 1;
        }
        0
    }
}

unsafe fn pcxhr_stop_pipes(mgr: *mut pcxhr_mgr, mut audio_mask: c_int) -> c_int {
    unsafe {
        let mut rmh: pcxhr_rmh = core::mem::zeroed();
        let mut err: c_int;
        let mut audio: c_int = 0;

        while audio_mask != 0 {
            if audio_mask & 1 != 0 {
                pcxhr_init_rmh(&mut rmh, CMD_STOP_PIPE as c_int);
                if audio < PCXHR_PIPE_STATE_CAPTURE_OFFSET {
                    /* stop playback pipe */
                    pcxhr_set_pipe_cmd_params(&mut rmh, 0, audio as c_uint, 0, 0);
                } else {
                    /* stop capture pipe */
                    pcxhr_set_pipe_cmd_params(
                        &mut rmh,
                        1,
                        (audio - PCXHR_PIPE_STATE_CAPTURE_OFFSET) as c_uint,
                        0,
                        0,
                    );
                }
                err = pcxhr_send_msg(mgr, &mut rmh);
                if err != 0 {
                    dev_err(
                        &(*(*mgr).pci).dev,
                        c"error pipe stop (CMD_STOP_PIPE) err=%x!\n".as_ptr(),
                        err,
                    );
                    return err;
                }
            }
            audio_mask >>= 1;
            audio += 1;
        }
        0
    }
}

unsafe fn pcxhr_toggle_pipes(mgr: *mut pcxhr_mgr, mut audio_mask: c_int) -> c_int {
    unsafe {
        let mut rmh: pcxhr_rmh = core::mem::zeroed();
        let mut err: c_int;
        let mut audio: c_int = 0;

        while audio_mask != 0 {
            if audio_mask & 1 != 0 {
                pcxhr_init_rmh(&mut rmh, CMD_CONF_PIPE as c_int);
                if audio < PCXHR_PIPE_STATE_CAPTURE_OFFSET {
                    pcxhr_set_pipe_cmd_params(&mut rmh, 0, 0, 0, (1 << audio) as c_uint);
                } else {
                    pcxhr_set_pipe_cmd_params(
                        &mut rmh,
                        1,
                        0,
                        0,
                        (1 << (audio - PCXHR_PIPE_STATE_CAPTURE_OFFSET)) as c_uint,
                    );
                }
                err = pcxhr_send_msg(mgr, &mut rmh);
                if err != 0 {
                    dev_err(
                        &(*(*mgr).pci).dev,
                        c"error pipe start (CMD_CONF_PIPE) err=%x!\n".as_ptr(),
                        err,
                    );
                    return err;
                }
            }
            audio_mask >>= 1;
            audio += 1;
        }
        /* now fire the interrupt on the card */
        pcxhr_init_rmh(&mut rmh, CMD_SEND_IRQA as c_int);
        err = pcxhr_send_msg(mgr, &mut rmh);
        if err != 0 {
            dev_err(
                &(*(*mgr).pci).dev,
                c"error pipe start (CMD_SEND_IRQA) err=%x!\n".as_ptr(),
                err,
            );
            return err;
        }
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn pcxhr_set_pipe_state(
    mgr: *mut pcxhr_mgr,
    playback_mask: c_int,
    capture_mask: c_int,
    start: c_int,
) -> c_int {
    unsafe {
        let mut state: c_int;
        let mut i: c_int;
        let mut err: c_int;
        let mut audio_mask: c_int;

        /* CONFIG_SND_DEBUG_VERBOSE: start_time = ktime_get(); */
        audio_mask = playback_mask | (capture_mask << PCXHR_PIPE_STATE_CAPTURE_OFFSET);
        /* current pipe state (playback + record) */
        state = pcxhr_pipes_running(mgr);
        dev_dbg(
            &(*(*mgr).pci).dev,
            c"pcxhr_set_pipe_state %s (mask %x current %x)\n".as_ptr(),
            if start != 0 { c"START".as_ptr() } else { c"STOP".as_ptr() },
            audio_mask,
            state,
        );
        if start != 0 {
            /* start only pipes that are not yet started */
            audio_mask &= !state;
            state = audio_mask;
            i = 0;
            while i < MAX_WAIT_FOR_DSP {
                err = pcxhr_prepair_pipe_start(mgr, state, &mut state);
                if err != 0 {
                    return err;
                }
                if state == 0 {
                    break; /* success, all pipes prepaired */
                }
                mdelay(1); /* wait 1 millisecond and retry */
                i += 1;
            }
        } else {
            audio_mask &= state; /* stop only pipes that are started */
        }
        if audio_mask == 0 {
            return 0;
        }

        err = pcxhr_toggle_pipes(mgr, audio_mask);
        if err != 0 {
            return err;
        }

        i = 0;
        loop {
            state = pcxhr_pipes_running(mgr);
            /* have all pipes the new state ? */
            if (state & audio_mask) == if start != 0 { audio_mask } else { 0 } {
                break;
            }
            i += 1;
            if i >= MAX_WAIT_FOR_DSP * 100 {
                dev_err(&(*(*mgr).pci).dev, c"error pipe start/stop\n".as_ptr());
                return -EBUSY;
            }
            udelay(10); /* wait 10 microseconds */
        }
        if start == 0 {
            err = pcxhr_stop_pipes(mgr, audio_mask);
            if err != 0 {
                return err;
            }
        }
        /* CONFIG_SND_DEBUG_VERBOSE: stop_time/diff_time trace */
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn pcxhr_write_io_num_reg_cont(
    mgr: *mut pcxhr_mgr,
    mask: c_uint,
    value: c_uint,
    changed: *mut c_int,
) -> c_int {
    unsafe {
        let mut rmh: pcxhr_rmh = core::mem::zeroed();
        let err: c_int;

        mutex_lock(&mut (*mgr).msg_lock);
        if ((*mgr).io_num_reg_cont & mask) == value {
            dev_dbg(
                &(*(*mgr).pci).dev,
                c"IO_NUM_REG_CONT mask %x already is set to %x\n".as_ptr(),
                mask,
                value,
            );
            if !changed.is_null() {
                *changed = 0;
            }
            mutex_unlock(&mut (*mgr).msg_lock);
            return 0; /* already programmed */
        }
        pcxhr_init_rmh(&mut rmh, CMD_ACCESS_IO_WRITE as c_int);
        rmh.cmd[0] |= IO_NUM_REG_CONT;
        rmh.cmd[1] = mask;
        rmh.cmd[2] = value;
        rmh.cmd_len = 3;
        err = pcxhr_send_msg_nolock(mgr, &mut rmh);
        if err == 0 {
            (*mgr).io_num_reg_cont &= !mask;
            (*mgr).io_num_reg_cont |= value;
            if !changed.is_null() {
                *changed = 1;
            }
        }
        mutex_unlock(&mut (*mgr).msg_lock);
        err
    }
}

const PCXHR_IRQ_TIMER: c_uint = 0x000300;
const PCXHR_IRQ_FREQ_CHANGE: c_uint = 0x000800;
const PCXHR_IRQ_TIME_CODE: c_uint = 0x001000;
const PCXHR_IRQ_NOTIFY: c_uint = 0x002000;
const PCXHR_IRQ_ASYNC: c_uint = 0x008000;
const PCXHR_IRQ_MASK: c_uint = 0x00bb00;
const PCXHR_FATAL_DSP_ERR: c_uint = 0xff0000;

#[repr(C)]
enum pcxhr_async_err_src {
    PCXHR_ERR_PIPE,
    PCXHR_ERR_STREAM,
    PCXHR_ERR_AUDIO,
}

unsafe fn pcxhr_handle_async_err(
    mgr: *mut pcxhr_mgr,
    mut err: u32,
    err_src: pcxhr_async_err_src,
    pipe: c_int,
    is_capture: c_int,
) -> c_int {
    unsafe {
        static err_src_name: [*const c_char; 3] = [
            c"Pipe".as_ptr(),
            c"Stream".as_ptr(),
            c"Audio".as_ptr(),
        ];

        if err & 0xfff != 0 {
            err &= 0xfff;
        } else {
            err = (err >> 12) & 0xfff;
        }
        if err == 0 {
            return 0;
        }
        dev_dbg(
            &(*(*mgr).pci).dev,
            c"CMD_ASYNC : Error %s %s Pipe %d err=%x\n".as_ptr(),
            err_src_name[err_src as usize],
            if is_capture != 0 { c"Record".as_ptr() } else { c"Play".as_ptr() },
            pipe,
            err,
        );
        if err == 0xe01 {
            (*mgr).async_err_stream_xrun += 1;
        } else if err == 0xe10 {
            (*mgr).async_err_pipe_xrun += 1;
        } else {
            (*mgr).async_err_other_last = err as c_int;
        }
        1
    }
}

unsafe fn pcxhr_msg_thread(mgr: *mut pcxhr_mgr) {
    unsafe {
        let prmh: *mut pcxhr_rmh = (*mgr).prmh;
        let mut err: c_int;
        let mut i: c_int;
        let mut j: c_int;

        if (*mgr).src_it_dsp & PCXHR_IRQ_FREQ_CHANGE != 0 {
            dev_dbg(&(*(*mgr).pci).dev, c"PCXHR_IRQ_FREQ_CHANGE event occurred\n".as_ptr());
        }
        if (*mgr).src_it_dsp & PCXHR_IRQ_TIME_CODE != 0 {
            dev_dbg(&(*(*mgr).pci).dev, c"PCXHR_IRQ_TIME_CODE event occurred\n".as_ptr());
        }
        if (*mgr).src_it_dsp & PCXHR_IRQ_NOTIFY != 0 {
            dev_dbg(&(*(*mgr).pci).dev, c"PCXHR_IRQ_NOTIFY event occurred\n".as_ptr());
        }
        if (*mgr).src_it_dsp & (PCXHR_IRQ_FREQ_CHANGE | PCXHR_IRQ_TIME_CODE) != 0 {
            /* clear events FREQ_CHANGE and TIME_CODE */
            pcxhr_init_rmh(prmh, CMD_TEST_IT as c_int);
            err = pcxhr_send_msg(mgr, prmh);
            dev_dbg(
                &(*(*mgr).pci).dev,
                c"CMD_TEST_IT : err=%x, stat=%x\n".as_ptr(),
                err,
                (*prmh).stat[0],
            );
        }
        if (*mgr).src_it_dsp & PCXHR_IRQ_ASYNC != 0 {
            dev_dbg(&(*(*mgr).pci).dev, c"PCXHR_IRQ_ASYNC event occurred\n".as_ptr());

            pcxhr_init_rmh(prmh, CMD_ASYNC as c_int);
            (*prmh).cmd[0] |= 1; /* add SEL_ASYNC_EVENTS */
            /* this is the only one extra long response command */
            (*prmh).stat_len = PCXHR_SIZE_MAX_LONG_STATUS as c_int;
            err = pcxhr_send_msg(mgr, prmh);
            if err != 0 {
                dev_err(&(*(*mgr).pci).dev, c"ERROR pcxhr_msg_thread=%x;\n".as_ptr(), err);
            }
            i = 1;
            while i < (*prmh).stat_len {
                let nb_audio: c_int =
                    (((*prmh).stat[i as usize] >> FIELD_SIZE) & MASK_FIRST_FIELD) as c_int;
                let nb_stream: c_int =
                    (((*prmh).stat[i as usize] >> (2 * FIELD_SIZE)) & MASK_FIRST_FIELD) as c_int;
                let pipe: c_int = ((*prmh).stat[i as usize] & MASK_FIRST_FIELD) as c_int;
                let is_capture: c_int = ((*prmh).stat[i as usize] & 0x400000) as c_int;
                let mut err2: u32;

                if (*prmh).stat[i as usize] & 0x800000 != 0 {
                    /* if BIT_END */
                    dev_dbg(
                        &(*(*mgr).pci).dev,
                        c"TASKLET : End%sPipe %d\n".as_ptr(),
                        if is_capture != 0 { c"Record".as_ptr() } else { c"Play".as_ptr() },
                        pipe,
                    );
                }
                i += 1;
                err2 = if (*prmh).stat[i as usize] != 0 {
                    (*prmh).stat[i as usize]
                } else {
                    (*prmh).stat[(i + 1) as usize]
                };
                if err2 != 0 {
                    pcxhr_handle_async_err(mgr, err2, pcxhr_async_err_src::PCXHR_ERR_PIPE, pipe, is_capture);
                }
                i += 2;
                j = 0;
                while j < nb_stream {
                    err2 = if (*prmh).stat[i as usize] != 0 {
                        (*prmh).stat[i as usize]
                    } else {
                        (*prmh).stat[(i + 1) as usize]
                    };
                    if err2 != 0 {
                        pcxhr_handle_async_err(mgr, err2, pcxhr_async_err_src::PCXHR_ERR_STREAM, pipe, is_capture);
                    }
                    i += 2;
                    j += 1;
                }
                j = 0;
                while j < nb_audio {
                    err2 = if (*prmh).stat[i as usize] != 0 {
                        (*prmh).stat[i as usize]
                    } else {
                        (*prmh).stat[(i + 1) as usize]
                    };
                    if err2 != 0 {
                        pcxhr_handle_async_err(mgr, err2, pcxhr_async_err_src::PCXHR_ERR_AUDIO, pipe, is_capture);
                    }
                    i += 2;
                    j += 1;
                }
            }
        }
    }
}

unsafe fn pcxhr_stream_read_position(
    mgr: *mut pcxhr_mgr,
    stream: *mut pcxhr_stream,
) -> u_int64_t {
    unsafe {
        let mut hw_sample_count: u_int64_t;
        let mut rmh: pcxhr_rmh = core::mem::zeroed();
        let err: c_int;
        let stream_mask: c_int;

        stream_mask = if (*(*stream).pipe).is_capture != 0 {
            1
        } else {
            1 << (*(*stream).substream).number
        };

        /* get sample count for one stream */
        pcxhr_init_rmh(&mut rmh, CMD_STREAM_SAMPLE_COUNT as c_int);
        pcxhr_set_pipe_cmd_params(
            &mut rmh,
            (*(*stream).pipe).is_capture,
            (*(*stream).pipe).first_audio,
            0,
            stream_mask as c_uint,
        );
        /* rmh.stat_len = 2; */ /* 2 resp data for each stream of the pipe */

        err = pcxhr_send_msg(mgr, &mut rmh);
        if err != 0 {
            return 0;
        }

        hw_sample_count = (rmh.stat[0] as u_int64_t) << 24;
        hw_sample_count += rmh.stat[1] as u_int64_t;

        dev_dbg(
            &(*(*mgr).pci).dev,
            c"stream %c%d : abs samples real(%llu) timer(%llu)\n".as_ptr(),
            if (*(*stream).pipe).is_capture != 0 { b'C' as c_int } else { b'P' as c_int },
            (*(*stream).substream).number,
            hw_sample_count,
            (*stream).timer_abs_periods
                + (*stream).timer_period_frag as u_int64_t
                + (*mgr).granularity as u_int64_t,
        );
        hw_sample_count
    }
}

unsafe fn pcxhr_update_timer_pos(
    mgr: *mut pcxhr_mgr,
    stream: *mut pcxhr_stream,
    mut samples_to_add: c_int,
) {
    unsafe {
        if !(*stream).substream.is_null() && (*stream).status == PCXHR_STREAM_STATUS_RUNNING {
            let mut new_sample_count: u_int64_t;
            let mut elapsed: c_int = 0;
            let mut hardware_read: c_int = 0;
            let runtime: *mut snd_pcm_runtime = (*(*stream).substream).runtime;

            if samples_to_add < 0 {
                (*stream).timer_is_synced = 0;
                /* add default if no hardware_read possible */
                samples_to_add = (*mgr).granularity;
            }

            if (*stream).timer_is_synced == 0 {
                if ((*stream).timer_abs_periods != 0)
                    || (((*stream).timer_period_frag as c_int + samples_to_add) as u_int64_t
                        >= (*runtime).period_size)
                {
                    new_sample_count = pcxhr_stream_read_position(mgr, stream);
                    hardware_read = 1;
                    if new_sample_count >= (*mgr).granularity as u_int64_t {
                        /* sub security offset because of
                         * jitter and finer granularity of
                         * dsp time (MBOX4)
                         */
                        new_sample_count -= (*mgr).granularity as u_int64_t;
                        (*stream).timer_is_synced = 1;
                    }
                } else {
                    new_sample_count = 0;
                }
            } else {
                new_sample_count = 0;
            }
            if hardware_read == 0 {
                /* if we didn't try to sync the position, increment it
                 * by PCXHR_GRANULARITY every timer interrupt
                 */
                new_sample_count = (*stream).timer_abs_periods
                    + (*stream).timer_period_frag as u_int64_t
                    + samples_to_add as u_int64_t;
            }
            loop {
                let new_elapse_pos: u_int64_t = (*stream).timer_abs_periods + (*runtime).period_size;
                if new_elapse_pos > new_sample_count {
                    break;
                }
                elapsed = 1;
                (*stream).timer_buf_periods += 1;
                if (*stream).timer_buf_periods >= (*runtime).periods {
                    (*stream).timer_buf_periods = 0;
                }
                (*stream).timer_abs_periods = new_elapse_pos;
            }
            if new_sample_count >= (*stream).timer_abs_periods {
                (*stream).timer_period_frag =
                    (new_sample_count - (*stream).timer_abs_periods) as u_int32_t;
            } else {
                dev_err(
                    &(*(*mgr).pci).dev,
                    c"ERROR new_sample_count too small ??? %ld\n".as_ptr(),
                    new_sample_count as c_ulong,
                );
            }

            if elapsed != 0 {
                mutex_unlock(&mut (*mgr).lock);
                snd_pcm_period_elapsed((*stream).substream);
                mutex_lock(&mut (*mgr).lock);
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn pcxhr_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    unsafe {
        let mgr: *mut pcxhr_mgr = dev_id as *mut pcxhr_mgr;
        let mut reg: c_uint;
        let mut wake_thread: bool_ = false;

        reg = PCXHR_INPL(mgr, PCXHR_PLX_IRQCS);
        if (reg & PCXHR_IRQCS_ACTIVE_PCIDB) == 0 {
            /* this device did not cause the interrupt */
            return irqreturn_t::IRQ_NONE;
        }

        /* clear interrupt */
        reg = PCXHR_INPL(mgr, PCXHR_PLX_L2PCIDB);
        PCXHR_OUTPL(mgr, PCXHR_PLX_L2PCIDB, reg);

        /* timer irq occurred */
        if reg & PCXHR_IRQ_TIMER != 0 {
            let timer_toggle: c_int = (reg & PCXHR_IRQ_TIMER) as c_int;
            if timer_toggle == (*mgr).timer_toggle {
                dev_dbg(&(*(*mgr).pci).dev, c"ERROR TIMER TOGGLE\n".as_ptr());
                (*mgr).dsp_time_err += 1;
            }

            (*mgr).timer_toggle = timer_toggle;
            (*mgr).src_it_dsp = reg;
            wake_thread = true;
        }

        /* other irq's handled in the thread */
        if reg & PCXHR_IRQ_MASK != 0 {
            if reg & PCXHR_IRQ_ASYNC != 0 {
                /* as we didn't request any async notifications,
                 * some kind of xrun error will probably occurred
                 */
                /* better resynchronize all streams next interrupt : */
                (*mgr).dsp_time_last = PCXHR_DSP_TIME_INVALID;
            }
            (*mgr).src_it_dsp = reg;
            wake_thread = true;
        }
        /* CONFIG_SND_DEBUG_VERBOSE: fatal DSP error trace */

        if wake_thread {
            irqreturn_t::IRQ_WAKE_THREAD
        } else {
            irqreturn_t::IRQ_HANDLED
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn pcxhr_threaded_irq(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    unsafe {
        let mgr: *mut pcxhr_mgr = dev_id as *mut pcxhr_mgr;
        let mut i: c_int;
        let mut j: c_int;
        let mut chip: *mut snd_pcxhr;

        mutex_lock(&mut (*mgr).lock);
        if (*mgr).src_it_dsp & PCXHR_IRQ_TIMER != 0 {
            /* is a 24 bit counter */
            let dsp_time_new: c_int =
                (PCXHR_INPL(mgr, PCXHR_PLX_MBOX4) as c_int) & PCXHR_DSP_TIME_MASK;
            let mut dsp_time_diff: c_int = dsp_time_new - (*mgr).dsp_time_last;

            if (dsp_time_diff < 0) && ((*mgr).dsp_time_last != PCXHR_DSP_TIME_INVALID) {
                /* handle dsp counter wraparound without resync */
                let tmp_diff: c_int = dsp_time_diff + PCXHR_DSP_TIME_MASK + 1;
                dev_dbg(
                    &(*(*mgr).pci).dev,
                    c"WARNING DSP timestamp old(%d) new(%d)".as_ptr(),
                    (*mgr).dsp_time_last,
                    dsp_time_new,
                );
                if tmp_diff > 0 && tmp_diff <= (2 * (*mgr).granularity) {
                    dev_dbg(
                        &(*(*mgr).pci).dev,
                        c"-> timestamp wraparound OK: diff=%d\n".as_ptr(),
                        tmp_diff,
                    );
                    dsp_time_diff = tmp_diff;
                } else {
                    dev_dbg(
                        &(*(*mgr).pci).dev,
                        c"-> resynchronize all streams\n".as_ptr(),
                    );
                    (*mgr).dsp_time_err += 1;
                }
            }
            /* CONFIG_SND_DEBUG_VERBOSE: DSP time consistency traces */
            (*mgr).dsp_time_last = dsp_time_new;

            i = 0;
            while i < (*mgr).num_cards {
                chip = *(*mgr).chip.add(i as usize);
                j = 0;
                while j < (*chip).nb_streams_capt {
                    pcxhr_update_timer_pos(
                        mgr,
                        (*chip).capture_stream.add(j as usize),
                        dsp_time_diff,
                    );
                    j += 1;
                }
                i += 1;
            }
            i = 0;
            while i < (*mgr).num_cards {
                chip = *(*mgr).chip.add(i as usize);
                j = 0;
                while j < (*chip).nb_streams_play {
                    pcxhr_update_timer_pos(
                        mgr,
                        (*chip).playback_stream.add(j as usize),
                        dsp_time_diff,
                    );
                    j += 1;
                }
                i += 1;
            }
        }

        pcxhr_msg_thread(mgr);
        mutex_unlock(&mut (*mgr).lock);
        irqreturn_t::IRQ_HANDLED
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
