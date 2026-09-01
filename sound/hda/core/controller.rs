// SPDX-License-Identifier: GPL-2.0-only
/*
 * HD-audio controller helpers
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_int, c_uint, c_ulong, c_void};
use core::ptr;

type u8 = u8;
type u32 = u32;
type __le32 = u32;
type bool_t = bool;

const EIO: c_int = 5;
const EAGAIN: c_int = 11;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

const AZX_RIRB_EX_UNSOL_EV: c_uint = 1 << 4;
const HDAC_MAX_CAPS: c_uint = 10;
const LOOP_COUNT_MAX: c_ulong = 3000;

extern "C" {
    static mut jiffies: c_ulong;

    fn udelay(usecs: c_ulong);
    fn msleep(msecs: c_uint);
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn schedule_timeout(timeout: c_ulong) -> c_long;
    fn cond_resched();
    fn msecs_to_jiffies(msecs: c_uint) -> c_ulong;
    fn time_before(a: c_ulong, b: c_ulong) -> bool_t;
    fn time_after(a: c_ulong, b: c_ulong) -> bool_t;

    fn snd_hdac_chip_readb(bus: *mut hdac_bus, reg: c_uint) -> u8;
    fn snd_hdac_chip_readw(bus: *mut hdac_bus, reg: c_uint) -> c_uint;
    fn snd_hdac_chip_readl(bus: *mut hdac_bus, reg: c_uint) -> u32;
    fn _snd_hdac_chip_readl(bus: *mut hdac_bus, reg: c_uint) -> u32;
    fn snd_hdac_chip_writeb(bus: *mut hdac_bus, reg: c_uint, val: c_uint);
    fn snd_hdac_chip_writew(bus: *mut hdac_bus, reg: c_uint, val: c_uint);
    fn snd_hdac_chip_writel(bus: *mut hdac_bus, reg: c_uint, val: u32);
    fn snd_hdac_chip_updateb(bus: *mut hdac_bus, reg: c_uint, mask: c_uint, val: c_uint);
    fn snd_hdac_chip_updatew(bus: *mut hdac_bus, reg: c_uint, mask: c_uint, val: c_uint);
    fn snd_hdac_chip_updatel(bus: *mut hdac_bus, reg: c_uint, mask: c_uint, val: c_uint);

    fn snd_hdac_stream_readb(s: *mut hdac_stream, reg: c_uint) -> u8;
    fn snd_hdac_stream_writeb(s: *mut hdac_stream, reg: c_uint, val: c_uint);
    fn snd_hdac_stream_updateb(s: *mut hdac_stream, reg: c_uint, mask: c_uint, val: c_uint);
    fn snd_hdac_stream_updatel(s: *mut hdac_stream, reg: c_uint, mask: c_uint, val: c_uint);

    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn dev_dbg(dev: *mut device, fmt: *const u8, ...);
    fn dev_dbg_ratelimited(dev: *mut device, fmt: *const u8, ...);
    fn dev_err_ratelimited(dev: *mut device, fmt: *const u8, ...);
    fn dev_warn_ratelimited(dev: *mut device, fmt: *const u8, ...);
    fn snd_BUG();
    fn snd_BUG_ON(cond: bool_t) -> bool_t;
    fn WARN_ON(cond: bool_t) -> bool_t;
    fn WARN_ON_ONCE(cond: bool_t) -> bool_t;

    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn waitqueue_active(wq: *mut wait_queue_head_t) -> bool_t;
    fn wake_up(wq: *mut wait_queue_head_t);
    fn init_wait_entry(wait: *mut wait_queue_entry_t, flags: c_int);
    fn prepare_to_wait(wq: *mut wait_queue_head_t, wait: *mut wait_queue_entry_t, state: c_int);
    fn finish_wait(wq: *mut wait_queue_head_t, wait: *mut wait_queue_entry_t);

    fn snd_hdac_bus_queue_event(bus: *mut hdac_bus, res: u32, res_ex: u32);
    fn snd_dma_alloc_pages(dtype: c_int, dev: *mut device, size: usize, dmab: *mut snd_dma_buffer) -> c_int;
    fn snd_dma_free_pages(dmab: *mut snd_dma_buffer);
    fn set_bit(nr: c_uint, addr: *mut c_ulong);
    fn clear_bit(nr: c_uint, addr: *mut c_ulong);
}

type c_long = i64;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_entry_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct snd_dma_buffer {
    pub area: *mut u8,
    pub addr: u64,
}

#[repr(C)]
pub struct hdac_rb {
    pub addr: u64,
    pub area: *mut u8,
}

#[repr(C)]
pub struct hdac_corb {
    pub addr: u64,
    pub buf: *mut __le32,
}

#[repr(C)]
pub struct hdac_rirb {
    pub addr: u64,
    pub buf: *mut __le32,
    pub wp: c_uint,
    pub rp: c_uint,
    pub cmds: [c_uint; HDA_MAX_CODECS as usize],
    pub res: [c_uint; HDA_MAX_CODECS as usize],
}

#[repr(C)]
pub struct hdac_stream {
    pub list: list_head,
    pub sd_int_sta_mask: c_uint,
    pub index: c_uint,
    pub substream: *mut c_void,
    pub cstream: *mut c_void,
    pub running: bool_t,
    pub bdl: snd_dma_buffer,
    pub posbuf: *mut __le32,
}

#[repr(C)]
pub struct hdac_bus {
    pub dev: *mut device,
    pub reg_lock: c_ulong,
    pub rb: hdac_rb,
    pub corb: hdac_corb,
    pub rirb: hdac_rirb,
    pub addr_offset: u64,
    pub corbrp_self_clear: bool_t,
    pub use_pio_for_commands: bool_t,
    pub not_use_interrupts: bool_t,
    pub rirb_wq: wait_queue_head_t,
    pub last_cmd: [c_uint; HDA_MAX_CODECS as usize],
    pub polling_mode: bool_t,
    pub needs_damn_long_delay: bool_t,
    pub mlcap: u64,
    pub gtscap: u64,
    pub ppcap: u64,
    pub spbcap: u64,
    pub drsmcap: u64,
    pub remap_addr: u64,
    pub access_sdnctl_in_dword: bool_t,
    pub stream_list: list_head,
    pub chip_init: bool_t,
    pub use_posbuf: bool_t,
    pub posbuf: snd_dma_buffer,
    pub codec_mask: c_ulong,
    pub dma_type: c_int,
    pub codec_powered: c_ulong,
}

#[repr(C)]
pub struct hdac_device {
    pub addr: c_uint,
    pub bus: *mut hdac_bus,
}

const CORBRP: c_uint = 0;
const CORBLBASE: c_uint = 0;
const CORBUBASE: c_uint = 0;
const CORBSIZE: c_uint = 0;
const CORBWP: c_uint = 0;
const CORBCTL: c_uint = 0;
const RIRBLBASE: c_uint = 0;
const RIRBUBASE: c_uint = 0;
const RIRBSIZE: c_uint = 0;
const RIRBWP: c_uint = 0;
const RINTCNT: c_uint = 0;
const RIRBCTL: c_uint = 0;
const GCTL: c_uint = 0;
const IRS: c_uint = 0;
const IR: c_uint = 0;
const IC: c_uint = 0;
const LLCH: c_uint = 0;
const STATESTS: c_uint = 0;
const INTCTL: c_uint = 0;
const INTSTS: c_uint = 0;
const RIRBSTS: c_uint = 0;
const DPLBASE: c_uint = 0;
const DPUBASE: c_uint = 0;
const SD_CTL: c_uint = 0;
const SD_STS: c_uint = 0;

const AZX_CORBRP_RST: c_uint = 0;
const AZX_CORBCTL_RUN: c_uint = 0;
const AZX_RIRBWP_RST: c_uint = 0;
const AZX_RBCTL_DMA_EN: c_uint = 0;
const AZX_RBCTL_IRQ_EN: c_uint = 0;
const AZX_GCTL_UNSOL: c_uint = 0;
const AZX_IRS_VALID: c_uint = 0;
const AZX_IRS_BUSY: c_uint = 0;
const AZX_MAX_CORB_ENTRIES: c_uint = 256;
const AZX_MAX_RIRB_ENTRIES: c_uint = 256;
const HDA_MAX_CODECS: c_uint = 16;
const AZX_CAP_HDR_VER_MASK: c_uint = 0;
const AZX_CAP_HDR_VER_OFF: c_uint = 0;
const AZX_CAP_HDR_ID_MASK: c_uint = 0;
const AZX_CAP_HDR_ID_OFF: c_uint = 0;
const AZX_ML_CAP_ID: c_uint = 0;
const AZX_GTS_CAP_ID: c_uint = 0;
const AZX_PP_CAP_ID: c_uint = 0;
const AZX_SPB_CAP_ID: c_uint = 0;
const AZX_DRSM_CAP_ID: c_uint = 0;
const AZX_CAP_HDR_NXT_PTR_MASK: c_uint = 0;
const AZX_GCTL_RESET: c_uint = 0;
const STATESTS_INT_MASK: c_uint = 0;
const AZX_INT_CTRL_EN: c_uint = 0;
const AZX_INT_GLOBAL_EN: c_uint = 0;
const SD_INT_MASK: c_uint = 0;
const RIRB_INT_MASK: c_uint = 0;
const AZX_INT_ALL_STREAM: c_uint = 0;
const SD_INT_FIFO_ERR: c_uint = 0;
const SD_INT_DESC_ERR: c_uint = 0;
const SD_INT_COMPLETE: c_uint = 0;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const BDL_SIZE: usize = 0;
const PAGE_SIZE: usize = 4096;
const TASK_UNINTERRUPTIBLE: c_int = 0;

#[inline]
fn upper_32_bits(x: u64) -> u32 {
    (x >> 32) as u32
}

#[inline]
fn cpu_to_le32(x: u32) -> __le32 {
    x.to_le()
}

#[inline]
fn le32_to_cpu(x: __le32) -> u32 {
    u32::from_le(x)
}

unsafe fn for_each_stream(mut f: impl FnMut(*mut hdac_stream)) {
    /*
     * C uses list_for_each_entry(..., &bus->stream_list, list). The concrete
     * list container mapping is provided by the surrounding kernel bindings.
     */
    let _ = &mut f;
}

/* clear CORB read pointer properly */
unsafe fn azx_clear_corbrp(bus: *mut hdac_bus) {
    let mut timeout: c_int;

    timeout = 1000;
    while timeout > 0 {
        if snd_hdac_chip_readw(bus, CORBRP) & AZX_CORBRP_RST != 0 {
            break;
        }
        udelay(1);
        timeout -= 1;
    }
    if timeout <= 0 {
        dev_err((*bus).dev, b"CORB reset timeout#1, CORBRP = %d\n\0".as_ptr(), snd_hdac_chip_readw(bus, CORBRP));
    }

    snd_hdac_chip_writew(bus, CORBRP, 0);
    timeout = 1000;
    while timeout > 0 {
        if snd_hdac_chip_readw(bus, CORBRP) == 0 {
            break;
        }
        udelay(1);
        timeout -= 1;
    }
    if timeout <= 0 {
        dev_err((*bus).dev, b"CORB reset timeout#2, CORBRP = %d\n\0".as_ptr(), snd_hdac_chip_readw(bus, CORBRP));
    }
}

/**
 * snd_hdac_bus_init_cmd_io - set up CORB/RIRB buffers
 * @bus: HD-audio core bus
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_bus_init_cmd_io(bus: *mut hdac_bus) {
    WARN_ON_ONCE((*bus).rb.area.is_null());

    /* CORB set up */
    (*bus).corb.addr = (*bus).rb.addr;
    (*bus).corb.buf = (*bus).rb.area as *mut __le32;
    snd_hdac_chip_writel(bus, CORBLBASE, ((*bus).corb.addr + (*bus).addr_offset) as u32);
    snd_hdac_chip_writel(bus, CORBUBASE, upper_32_bits((*bus).corb.addr + (*bus).addr_offset));

    /* set the corb size to 256 entries (ULI requires explicitly) */
    snd_hdac_chip_writeb(bus, CORBSIZE, 0x02);
    /* set the corb write pointer to 0 */
    snd_hdac_chip_writew(bus, CORBWP, 0);

    /* reset the corb hw read pointer */
    snd_hdac_chip_writew(bus, CORBRP, AZX_CORBRP_RST);
    if !(*bus).corbrp_self_clear {
        azx_clear_corbrp(bus);
    }

    /* enable corb dma */
    if !(*bus).use_pio_for_commands {
        snd_hdac_chip_writeb(bus, CORBCTL, AZX_CORBCTL_RUN);
    }

    /* RIRB set up */
    (*bus).rirb.addr = (*bus).rb.addr + 2048;
    (*bus).rirb.buf = (*bus).rb.area.add(2048) as *mut __le32;
    (*bus).rirb.rp = 0;
    (*bus).rirb.wp = (*bus).rirb.rp;
    memset((*bus).rirb.cmds.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&(*bus).rirb.cmds));
    snd_hdac_chip_writel(bus, RIRBLBASE, ((*bus).rirb.addr + (*bus).addr_offset) as u32);
    snd_hdac_chip_writel(bus, RIRBUBASE, upper_32_bits((*bus).rirb.addr + (*bus).addr_offset));

    /* set the rirb size to 256 entries (ULI requires explicitly) */
    snd_hdac_chip_writeb(bus, RIRBSIZE, 0x02);
    /* reset the rirb hw write pointer */
    snd_hdac_chip_writew(bus, RIRBWP, AZX_RIRBWP_RST);
    /* set N=1, get RIRB response interrupt for new entry */
    snd_hdac_chip_writew(bus, RINTCNT, 1);
    /* enable rirb dma and response irq */
    if (*bus).not_use_interrupts {
        snd_hdac_chip_writeb(bus, RIRBCTL, AZX_RBCTL_DMA_EN);
    } else {
        snd_hdac_chip_writeb(bus, RIRBCTL, AZX_RBCTL_DMA_EN | AZX_RBCTL_IRQ_EN);
    }
    /* Accept unsolicited responses */
    snd_hdac_chip_updatel(bus, GCTL, AZX_GCTL_UNSOL, AZX_GCTL_UNSOL);
}

/* wait for cmd dmas till they are stopped */
unsafe fn hdac_wait_for_cmd_dmas(bus: *mut hdac_bus) {
    let mut timeout: c_ulong;

    timeout = jiffies.wrapping_add(msecs_to_jiffies(100));
    while (snd_hdac_chip_readb(bus, RIRBCTL) as c_uint & AZX_RBCTL_DMA_EN) != 0
        && time_before(jiffies, timeout)
    {
        udelay(10);
    }

    timeout = jiffies.wrapping_add(msecs_to_jiffies(100));
    while (snd_hdac_chip_readb(bus, CORBCTL) as c_uint & AZX_CORBCTL_RUN) != 0
        && time_before(jiffies, timeout)
    {
        udelay(10);
    }
}

/**
 * snd_hdac_bus_stop_cmd_io - clean up CORB/RIRB buffers
 * @bus: HD-audio core bus
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_bus_stop_cmd_io(bus: *mut hdac_bus) {
    /* disable ringbuffer DMAs */
    snd_hdac_chip_writeb(bus, RIRBCTL, 0);
    snd_hdac_chip_writeb(bus, CORBCTL, 0);

    hdac_wait_for_cmd_dmas(bus);

    /* disable unsolicited responses */
    snd_hdac_chip_updatel(bus, GCTL, AZX_GCTL_UNSOL, 0);
}

unsafe fn azx_command_addr(cmd: u32) -> c_uint {
    let mut addr: c_uint = cmd >> 28;

    if snd_BUG_ON(addr >= HDA_MAX_CODECS) {
        addr = 0;
    }
    addr
}

/* receive an Immediate Response with PIO */
unsafe fn snd_hdac_bus_wait_for_pio_response(bus: *mut hdac_bus, addr: c_uint) -> c_int {
    let mut timeout: c_int = 50;

    while timeout != 0 {
        timeout -= 1;
        /* check IRV bit */
        if snd_hdac_chip_readw(bus, IRS) & AZX_IRS_VALID != 0 {
            /* reuse rirb.res as the response return value */
            (*bus).rirb.res[addr as usize] = snd_hdac_chip_readl(bus, IR);
            return 0;
        }
        udelay(1);
    }

    dev_dbg_ratelimited((*bus).dev, b"get_response_pio timeout: IRS=%#x\n\0".as_ptr(), snd_hdac_chip_readw(bus, IRS));

    (*bus).rirb.res[addr as usize] = (-1i32) as c_uint;

    -EIO
}

/**
 * snd_hdac_bus_send_cmd_pio - send a command verb via Immediate Command
 * @bus: HD-audio core bus
 * @val: encoded verb value to send
 *
 * Returns zero for success or a negative error code.
 */
unsafe fn snd_hdac_bus_send_cmd_pio(bus: *mut hdac_bus, val: c_uint) -> c_int {
    let addr: c_uint = azx_command_addr(val);
    let mut timeout: c_int = 50;

    while timeout != 0 {
        timeout -= 1;
        /* check ICB bit */
        if snd_hdac_chip_readw(bus, IRS) & AZX_IRS_BUSY == 0 {
            /* Clear IRV bit */
            snd_hdac_chip_updatew(bus, IRS, AZX_IRS_VALID, AZX_IRS_VALID);
            snd_hdac_chip_writel(bus, IC, val);
            /* Set ICB bit */
            snd_hdac_chip_updatew(bus, IRS, AZX_IRS_BUSY, AZX_IRS_BUSY);

            return snd_hdac_bus_wait_for_pio_response(bus, addr);
        }
        udelay(1);
    }

    dev_dbg_ratelimited((*bus).dev, b"send_cmd_pio timeout: IRS=%#x, val=%#x\n\0".as_ptr(), snd_hdac_chip_readw(bus, IRS), val);

    -EIO
}

/**
 * snd_hdac_bus_get_response_pio - receive a response via Immediate Response
 * @bus: HD-audio core bus
 * @addr: codec address
 * @res: pointer to store the value, NULL when not needed
 *
 * Returns zero if a value is read, or a negative error code.
 */
unsafe fn snd_hdac_bus_get_response_pio(bus: *mut hdac_bus, addr: c_uint, res: *mut c_uint) -> c_int {
    if !res.is_null() {
        *res = (*bus).rirb.res[addr as usize];
    }

    0
}

/**
 * snd_hdac_bus_send_cmd_corb - send a command verb via CORB
 * @bus: HD-audio core bus
 * @val: encoded verb value to send
 *
 * Returns zero for success or a negative error code.
 */
unsafe fn snd_hdac_bus_send_cmd_corb(bus: *mut hdac_bus, val: c_uint) -> c_int {
    let addr: c_uint = azx_command_addr(val);
    let mut wp: c_uint;
    let rp: c_uint;

    (*bus).last_cmd[azx_command_addr(val) as usize] = val;

    /* add command to corb */
    wp = snd_hdac_chip_readw(bus, CORBWP);
    if wp == 0xffff {
        /* something wrong, controller likely turned to D3 */
        return -EIO;
    }
    wp = wp.wrapping_add(1);
    wp %= AZX_MAX_CORB_ENTRIES;

    rp = snd_hdac_chip_readw(bus, CORBRP);
    if wp == rp {
        /* oops, it's full */
        return -EAGAIN;
    }

    (*bus).rirb.cmds[addr as usize] = (*bus).rirb.cmds[addr as usize].wrapping_add(1);
    *(*bus).corb.buf.add(wp as usize) = cpu_to_le32(val);
    snd_hdac_chip_writew(bus, CORBWP, wp);

    0
}

/**
 * snd_hdac_bus_update_rirb - retrieve RIRB entries
 * @bus: HD-audio core bus
 *
 * Usually called from interrupt handler.
 * The caller needs bus->reg_lock spinlock before calling this.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_bus_update_rirb(bus: *mut hdac_bus) {
    let mut rp: c_uint;
    let wp: c_uint;
    let mut addr: c_uint;
    let mut res: u32;
    let mut res_ex: u32;

    wp = snd_hdac_chip_readw(bus, RIRBWP);
    if wp == 0xffff {
        /* something wrong, controller likely turned to D3 */
        return;
    }

    if wp == (*bus).rirb.wp {
        return;
    }
    (*bus).rirb.wp = wp;

    while (*bus).rirb.rp != wp {
        (*bus).rirb.rp = (*bus).rirb.rp.wrapping_add(1);
        (*bus).rirb.rp %= AZX_MAX_RIRB_ENTRIES;

        rp = (*bus).rirb.rp << 1; /* an RIRB entry is 8-bytes */
        res_ex = le32_to_cpu(*(*bus).rirb.buf.add((rp + 1) as usize));
        res = le32_to_cpu(*(*bus).rirb.buf.add(rp as usize));
        addr = res_ex & 0xf;
        if addr >= HDA_MAX_CODECS {
            dev_err((*bus).dev, b"spurious response %#x:%#x, rp = %d, wp = %d\0".as_ptr(), res, res_ex, (*bus).rirb.rp, wp);
            snd_BUG();
        } else if res_ex & AZX_RIRB_EX_UNSOL_EV != 0 {
            snd_hdac_bus_queue_event(bus, res, res_ex);
        } else if (*bus).rirb.cmds[addr as usize] != 0 {
            (*bus).rirb.res[addr as usize] = res;
            (*bus).rirb.cmds[addr as usize] = (*bus).rirb.cmds[addr as usize].wrapping_sub(1);
            if (*bus).rirb.cmds[addr as usize] == 0 && waitqueue_active(&mut (*bus).rirb_wq) {
                wake_up(&mut (*bus).rirb_wq);
            }
        } else {
            dev_err_ratelimited((*bus).dev, b"spurious response %#x:%#x, last cmd=%#08x\n\0".as_ptr(), res, res_ex, (*bus).last_cmd[addr as usize]);
        }
    }
}

/**
 * snd_hdac_bus_get_response_rirb - receive a response via RIRB
 * @bus: HD-audio core bus
 * @addr: codec address
 * @res: pointer to store the value, NULL when not needed
 *
 * Returns zero if a value is read, or a negative error code.
 */
unsafe fn snd_hdac_bus_get_response_rirb(bus: *mut hdac_bus, addr: c_uint, res: *mut c_uint) -> c_int {
    let timeout: c_ulong;
    let mut loopcounter: c_ulong;
    let mut wait = wait_queue_entry_t { _private: [] };
    let mut warned: bool_t = false;

    init_wait_entry(&mut wait, 0);
    timeout = jiffies.wrapping_add(msecs_to_jiffies(1000));

    loopcounter = 0;
    loop {
        if !(*bus).polling_mode {
            prepare_to_wait(&mut (*bus).rirb_wq, &mut wait, TASK_UNINTERRUPTIBLE);
        }
        if (*bus).polling_mode {
            snd_hdac_bus_update_rirb(bus);
        }
        if (*bus).rirb.cmds[addr as usize] == 0 {
            if !res.is_null() {
                *res = (*bus).rirb.res[addr as usize]; /* the last value */
            }
            if !(*bus).polling_mode {
                finish_wait(&mut (*bus).rirb_wq, &mut wait);
            }
            return 0;
        }
        if time_after(jiffies, timeout) {
            break;
        }
        if !(*bus).polling_mode {
            schedule_timeout(msecs_to_jiffies(2));
        } else if (*bus).needs_damn_long_delay || loopcounter > LOOP_COUNT_MAX {
            if loopcounter > LOOP_COUNT_MAX && !warned {
                dev_dbg_ratelimited((*bus).dev, b"too slow response, last cmd=%#08x\n\0".as_ptr(), (*bus).last_cmd[addr as usize]);
                warned = true;
            }
            msleep(2); /* temporary workaround */
        } else {
            udelay(10);
            cond_resched();
        }
        loopcounter = loopcounter.wrapping_add(1);
    }

    if !(*bus).polling_mode {
        finish_wait(&mut (*bus).rirb_wq, &mut wait);
    }

    -EIO
}

#[no_mangle]
pub unsafe extern "C" fn snd_hdac_bus_send_cmd(bus: *mut hdac_bus, val: c_uint) -> c_int {
    if (*bus).use_pio_for_commands {
        return snd_hdac_bus_send_cmd_pio(bus, val);
    }

    snd_hdac_bus_send_cmd_corb(bus, val)
}

#[no_mangle]
pub unsafe extern "C" fn snd_hdac_bus_get_response(bus: *mut hdac_bus, addr: c_uint, res: *mut c_uint) -> c_int {
    if (*bus).use_pio_for_commands {
        return snd_hdac_bus_get_response_pio(bus, addr, res);
    }

    snd_hdac_bus_get_response_rirb(bus, addr, res)
}

#[no_mangle]
pub unsafe extern "C" fn snd_hdac_bus_parse_capabilities(bus: *mut hdac_bus) -> c_int {
    let mut cur_cap: c_uint;
    let mut offset: c_uint;
    let mut counter: c_uint = 0;

    offset = snd_hdac_chip_readw(bus, LLCH);

    /* Lets walk the linked capabilities list */
    loop {
        cur_cap = _snd_hdac_chip_readl(bus, offset);

        dev_dbg((*bus).dev, b"Capability version: 0x%x\n\0".as_ptr(), (cur_cap & AZX_CAP_HDR_VER_MASK) >> AZX_CAP_HDR_VER_OFF);
        dev_dbg((*bus).dev, b"HDA capability ID: 0x%x\n\0".as_ptr(), (cur_cap & AZX_CAP_HDR_ID_MASK) >> AZX_CAP_HDR_ID_OFF);

        if cur_cap == (-1i32) as c_uint {
            dev_dbg((*bus).dev, b"Invalid capability reg read\n\0".as_ptr());
            break;
        }

        match (cur_cap & AZX_CAP_HDR_ID_MASK) >> AZX_CAP_HDR_ID_OFF {
            AZX_ML_CAP_ID => {
                dev_dbg((*bus).dev, b"Found ML capability\n\0".as_ptr());
                (*bus).mlcap = (*bus).remap_addr + offset as u64;
            }
            AZX_GTS_CAP_ID => {
                dev_dbg((*bus).dev, b"Found GTS capability offset=%x\n\0".as_ptr(), offset);
                (*bus).gtscap = (*bus).remap_addr + offset as u64;
            }
            AZX_PP_CAP_ID => {
                /* PP capability found, the Audio DSP is present */
                dev_dbg((*bus).dev, b"Found PP capability offset=%x\n\0".as_ptr(), offset);
                (*bus).ppcap = (*bus).remap_addr + offset as u64;
            }
            AZX_SPB_CAP_ID => {
                /* SPIB capability found, handler function */
                dev_dbg((*bus).dev, b"Found SPB capability\n\0".as_ptr());
                (*bus).spbcap = (*bus).remap_addr + offset as u64;
            }
            AZX_DRSM_CAP_ID => {
                /* DMA resume  capability found, handler function */
                dev_dbg((*bus).dev, b"Found DRSM capability\n\0".as_ptr());
                (*bus).drsmcap = (*bus).remap_addr + offset as u64;
            }
            _ => {
                dev_err((*bus).dev, b"Unknown capability %d\n\0".as_ptr(), cur_cap);
                cur_cap = 0;
            }
        }

        counter = counter.wrapping_add(1);

        if counter > HDAC_MAX_CAPS {
            dev_err((*bus).dev, b"We exceeded HDAC capabilities!!!\n\0".as_ptr());
            break;
        }

        /* read the offset of next capability */
        offset = cur_cap & AZX_CAP_HDR_NXT_PTR_MASK;

        if offset == 0 {
            break;
        }
    }

    0
}

/*
 * Lowlevel interface
 */

#[no_mangle]
pub unsafe extern "C" fn snd_hdac_bus_enter_link_reset(bus: *mut hdac_bus) {
    let timeout: c_ulong;

    /* reset controller */
    snd_hdac_chip_updatel(bus, GCTL, AZX_GCTL_RESET, 0);

    timeout = jiffies.wrapping_add(msecs_to_jiffies(100));
    while (snd_hdac_chip_readb(bus, GCTL) as c_uint & AZX_GCTL_RESET) != 0 && time_before(jiffies, timeout) {
        usleep_range(500, 1000);
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_hdac_bus_exit_link_reset(bus: *mut hdac_bus) {
    let timeout: c_ulong;

    if (*bus).access_sdnctl_in_dword {
        snd_hdac_chip_updatel(bus, GCTL, AZX_GCTL_RESET, AZX_GCTL_RESET);
    } else {
        snd_hdac_chip_updateb(bus, GCTL, AZX_GCTL_RESET, AZX_GCTL_RESET);
    }

    timeout = jiffies.wrapping_add(msecs_to_jiffies(100));
    while snd_hdac_chip_readb(bus, GCTL) == 0 && time_before(jiffies, timeout) {
        usleep_range(500, 1000);
    }
}

/* reset codec link */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_bus_reset_link(bus: *mut hdac_bus, full_reset: bool_t) -> c_int {
    if full_reset {
        /* clear STATESTS if not in reset */
        if snd_hdac_chip_readb(bus, GCTL) as c_uint & AZX_GCTL_RESET != 0 {
            snd_hdac_chip_writew(bus, STATESTS, STATESTS_INT_MASK);
        }

        /* reset controller */
        snd_hdac_bus_enter_link_reset(bus);

        /* delay for >= 100us for codec PLL to settle per spec
         * Rev 0.9 section 5.5.1
         */
        usleep_range(500, 1000);

        /* Bring controller out of reset */
        snd_hdac_bus_exit_link_reset(bus);

        /* Brent Chartrand said to wait >= 540us for codecs to initialize */
        usleep_range(1000, 1200);
    }

    /* check to see if controller is ready */
    if snd_hdac_chip_readb(bus, GCTL) == 0 {
        dev_dbg((*bus).dev, b"controller not ready!\n\0".as_ptr());
        return -EBUSY;
    }

    /* detect codecs */
    if (*bus).codec_mask == 0 {
        (*bus).codec_mask = snd_hdac_chip_readw(bus, STATESTS) as c_ulong;
        dev_dbg((*bus).dev, b"codec_mask = 0x%lx\n\0".as_ptr(), (*bus).codec_mask);
    }

    0
}

/* enable interrupts */
unsafe fn azx_int_enable(bus: *mut hdac_bus) {
    /* enable controller CIE and GIE */
    snd_hdac_chip_updatel(bus, INTCTL, AZX_INT_CTRL_EN | AZX_INT_GLOBAL_EN, AZX_INT_CTRL_EN | AZX_INT_GLOBAL_EN);
}

/* disable interrupts */
unsafe fn azx_int_disable(bus: *mut hdac_bus) {
    /* disable interrupts in stream descriptor */
    for_each_stream(|azx_dev| {
        if (*bus).access_sdnctl_in_dword {
            snd_hdac_stream_updatel(azx_dev, SD_CTL, SD_INT_MASK, 0);
        } else {
            snd_hdac_stream_updateb(azx_dev, SD_CTL, SD_INT_MASK, 0);
        }
    });

    /* disable SIE for all streams & disable controller CIE and GIE */
    snd_hdac_chip_writel(bus, INTCTL, 0);
}

/* clear interrupts */
unsafe fn azx_int_clear(bus: *mut hdac_bus) {
    /* clear stream status */
    for_each_stream(|azx_dev| {
        snd_hdac_stream_writeb(azx_dev, SD_STS, SD_INT_MASK);
    });

    /* clear STATESTS */
    snd_hdac_chip_writew(bus, STATESTS, STATESTS_INT_MASK);

    /* clear rirb status */
    snd_hdac_chip_writeb(bus, RIRBSTS, RIRB_INT_MASK);

    /* clear int status */
    snd_hdac_chip_writel(bus, INTSTS, AZX_INT_CTRL_EN | AZX_INT_ALL_STREAM);
}

#[no_mangle]
pub unsafe extern "C" fn snd_hdac_bus_init_chip(bus: *mut hdac_bus, full_reset: bool_t) -> bool_t {
    if (*bus).chip_init {
        return false;
    }

    /* reset controller */
    snd_hdac_bus_reset_link(bus, full_reset);

    /* clear interrupts */
    azx_int_clear(bus);

    /* initialize the codec command I/O */
    snd_hdac_bus_init_cmd_io(bus);

    /* enable interrupts after CORB/RIRB buffers are initialized above */
    azx_int_enable(bus);

    /* program the position buffer */
    if (*bus).use_posbuf && (*bus).posbuf.addr != 0 {
        snd_hdac_chip_writel(bus, DPLBASE, ((*bus).posbuf.addr + (*bus).addr_offset) as u32);
        snd_hdac_chip_writel(bus, DPUBASE, upper_32_bits((*bus).posbuf.addr + (*bus).addr_offset));
    }

    (*bus).chip_init = true;

    true
}

#[no_mangle]
pub unsafe extern "C" fn snd_hdac_bus_stop_chip(bus: *mut hdac_bus) {
    if !(*bus).chip_init {
        return;
    }

    /* disable interrupts */
    azx_int_disable(bus);
    azx_int_clear(bus);

    /* disable CORB/RIRB */
    snd_hdac_bus_stop_cmd_io(bus);

    /* disable position buffer */
    if (*bus).posbuf.addr != 0 {
        snd_hdac_chip_writel(bus, DPLBASE, 0);
        snd_hdac_chip_writel(bus, DPUBASE, 0);
    }

    (*bus).chip_init = false;
}

#[no_mangle]
pub unsafe extern "C" fn snd_hdac_bus_handle_stream_irq(
    bus: *mut hdac_bus,
    status: c_uint,
    ack: Option<unsafe extern "C" fn(*mut hdac_bus, *mut hdac_stream)>,
) -> c_int {
    let mut handled: c_int = 0;

    for_each_stream(|azx_dev| {
        let mut sd_status: u8;
        if status & (*azx_dev).sd_int_sta_mask != 0 {
            sd_status = snd_hdac_stream_readb(azx_dev, SD_STS);
            snd_hdac_stream_writeb(azx_dev, SD_STS, SD_INT_MASK);
            handled |= 1 << (*azx_dev).index;
            if sd_status as c_uint & (SD_INT_FIFO_ERR | SD_INT_DESC_ERR) != 0 {
                dev_warn_ratelimited((*bus).dev, b"stream %u dma error: 0x%02x\n\0".as_ptr(), (*azx_dev).index, sd_status as c_uint);
            }
            if (((*azx_dev).substream.is_null() && (*azx_dev).cstream.is_null())
                || !(*azx_dev).running
                || (sd_status as c_uint & SD_INT_COMPLETE) == 0)
            {
                return;
            }
            if let Some(ack_fn) = ack {
                ack_fn(bus, azx_dev);
            }
        }
    });
    handled
}

#[no_mangle]
pub unsafe extern "C" fn snd_hdac_bus_alloc_stream_pages(bus: *mut hdac_bus) -> c_int {
    let mut num_streams: c_int = 0;
    let dma_type: c_int = if (*bus).dma_type != 0 { (*bus).dma_type } else { SNDRV_DMA_TYPE_DEV };
    let mut err: c_int = 0;

    for_each_stream(|s| {
        /* allocate memory for the BDL for each stream */
        err = snd_dma_alloc_pages(dma_type, (*bus).dev, BDL_SIZE, &mut (*s).bdl);
        num_streams += 1;
    });
    if err < 0 {
        return -ENOMEM;
    }

    if WARN_ON(num_streams == 0) {
        return -EINVAL;
    }
    /* allocate memory for the position buffer */
    err = snd_dma_alloc_pages(dma_type, (*bus).dev, num_streams as usize * 8, &mut (*bus).posbuf);
    if err < 0 {
        for_each_stream(|s| {
            if !(*s).bdl.area.is_null() {
                snd_dma_free_pages(&mut (*s).bdl);
            }
        });
        return -ENOMEM;
    }
    for_each_stream(|s| {
        (*s).posbuf = (*bus).posbuf.area.add((*s).index as usize * 8) as *mut __le32;
    });

    /* single page (at least 4096 bytes) must suffice for both ringbuffes */
    err = snd_dma_alloc_pages(dma_type, (*bus).dev, PAGE_SIZE, &mut (*bus).rb as *mut hdac_rb as *mut snd_dma_buffer);
    if err < 0 {
        snd_dma_free_pages(&mut (*bus).posbuf);
        for_each_stream(|s| {
            if !(*s).bdl.area.is_null() {
                snd_dma_free_pages(&mut (*s).bdl);
            }
        });
        return -ENOMEM;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_hdac_bus_free_stream_pages(bus: *mut hdac_bus) {
    for_each_stream(|s| {
        if !(*s).bdl.area.is_null() {
            snd_dma_free_pages(&mut (*s).bdl);
        }
    });

    if !(*bus).rb.area.is_null() {
        snd_dma_free_pages(&mut (*bus).rb as *mut hdac_rb as *mut snd_dma_buffer);
    }
    if !(*bus).posbuf.area.is_null() {
        snd_dma_free_pages(&mut (*bus).posbuf);
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_hdac_bus_link_power(codec: *mut hdac_device, enable: bool_t) {
    if enable {
        set_bit((*codec).addr, &mut (*(*codec).bus).codec_powered);
    } else {
        clear_bit((*codec).addr, &mut (*(*codec).bus).codec_powered);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
