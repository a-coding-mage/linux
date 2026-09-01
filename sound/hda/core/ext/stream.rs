// SPDX-License-Identifier: GPL-2.0-only
/*
 *  hdac-ext-stream.c - HD-audio extended stream operations.
 *
 *  Copyright (C) 2015 Intel Corp
 *  Author: Jeeja KP <jeeja.kp@intel.com>
 *  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::zeroed;
use core::ptr::{addr_of_mut, null_mut};

// Dependencies originally provided by:
// <linux/delay.h>, <linux/pci.h>, <linux/pci_ids.h>, <linux/slab.h>,
// <sound/pcm.h>, <sound/hda_register.h>, <sound/hdaudio_ext.h>,
// <sound/compress_driver.h>

pub const ENOMEM: c_int = 12;
pub const PCI_DEVICE_ID_INTEL_HDA_APL: u32 = 0x5a98;

pub const AZX_PPHC_BASE: usize = 0x10;
pub const AZX_PPHC_INTERVAL: usize = 0x10;
pub const AZX_PPLC_BASE: usize = 0x10;
pub const AZX_PPLC_MULTI: usize = 0x10;
pub const AZX_PPLC_INTERVAL: usize = 0x10;
pub const AZX_REG_PP_PPCTL: usize = 0x04;
pub const AZX_REG_PPLCCTL: usize = 0x00;
pub const AZX_REG_PPLCFMT: usize = 0x02;
pub const AZX_PPLCCTL_RUN: u32 = 0x1;
pub const AZX_PPLCCTL_STRST: u32 = 0x2;
pub const AZX_PPLCCTL_STRM_SHIFT: c_uint = 20;
pub const AZX_PPLCCTL_STRM_MASK: u32 = 0xf << AZX_PPLCCTL_STRM_SHIFT;

pub const HDAC_EXT_STREAM_TYPE_COUPLED: c_int = 0;
pub const HDAC_EXT_STREAM_TYPE_HOST: c_int = 1;
pub const HDAC_EXT_STREAM_TYPE_LINK: c_int = 2;

#[inline]
pub const fn AZX_PPCTL_PROCEN(index: c_int) -> c_int {
    1 << index
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    pub device: u32,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_compr_stream {
    pub direction: c_int,
}

#[repr(C)]
pub struct hdac_bus {
    pub dev: *mut device,
    pub ppcap: usize,
    pub num_streams: c_int,
    pub stream_list: list_head,
    pub reg_lock: spinlock_t,
}

#[repr(C)]
pub struct hdac_stream {
    pub list: list_head,
    pub bus: *mut hdac_bus,
    pub index: c_int,
    pub direction: c_int,
    pub stream_tag: c_int,
    pub opened: c_int,
    pub running: c_int,
    pub substream: *mut snd_pcm_substream,
    pub cstream: *mut snd_compr_stream,
}

#[repr(C)]
pub struct hdac_ext_stream {
    pub hstream: hdac_stream,
    pub pphc_addr: usize,
    pub pplc_addr: usize,
    pub decoupled: bool,
    pub link_locked: c_int,
    pub link_substream: *mut snd_pcm_substream,
    pub host_setup: Option<unsafe extern "C" fn(*mut hdac_stream, bool) -> c_int>,
}

unsafe extern "C" {
    fn to_pci_dev(dev: *mut device) -> *mut pci_dev;
    fn snd_hdac_stream_setup(hstream: *mut hdac_stream, code_loading: bool) -> c_int;
    fn snd_hdac_stream_init(
        bus: *mut hdac_bus,
        hstream: *mut hdac_stream,
        idx: c_int,
        direction: c_int,
        tag: c_int,
    );
    fn snd_hdac_stream_assign(
        bus: *mut hdac_bus,
        substream: *mut snd_pcm_substream,
    ) -> *mut hdac_stream;
    fn snd_hdac_stream_release(hstream: *mut hdac_stream);
    fn snd_hdac_stream_release_locked(hstream: *mut hdac_stream);
    fn snd_hdac_updatel(addr: usize, reg: usize, mask: c_uint, val: c_uint);
    fn readw(addr: usize) -> u16;
    fn readl(addr: usize) -> u32;
    fn writel(val: c_uint, addr: usize);
    fn writew(val: c_int, addr: usize);
    fn udelay(usecs: c_uint);
    fn kzalloc(size: usize) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

struct SpinLockIrqGuard {
    lock: *mut spinlock_t,
}

impl SpinLockIrqGuard {
    unsafe fn new(lock: *mut spinlock_t) -> Self {
        unsafe {
            spin_lock_irq(lock);
        }
        Self { lock }
    }
}

impl Drop for SpinLockIrqGuard {
    fn drop(&mut self) {
        unsafe {
            spin_unlock_irq(self.lock);
        }
    }
}

#[inline]
unsafe fn hdac_stream(hext_stream: *mut hdac_ext_stream) -> *mut hdac_stream {
    unsafe { addr_of_mut!((*hext_stream).hstream) }
}

#[inline]
unsafe fn stream_to_hdac_ext_stream(hstream: *mut hdac_stream) -> *mut hdac_ext_stream {
    hstream.cast::<hdac_ext_stream>()
}

unsafe fn list_empty(head: *mut list_head) -> bool {
    unsafe { (*head).next == head }
}

unsafe fn list_del(entry: *mut list_head) {
    unsafe {
        (*(*entry).next).prev = (*entry).prev;
        (*(*entry).prev).next = (*entry).next;
    }
}

unsafe fn for_each_stream<F>(bus: *mut hdac_bus, mut f: F)
where
    F: FnMut(*mut hdac_stream) -> bool,
{
    unsafe {
        let head = addr_of_mut!((*bus).stream_list);
        if list_empty(head) {
            return;
        }
        let mut pos = (*head).next;
        while pos != head {
            let hstream = pos.cast::<hdac_stream>();
            if !f(hstream) {
                break;
            }
            pos = (*pos).next;
        }
    }
}

/**
 * snd_hdac_ext_host_stream_setup - Setup a HOST stream.
 * @hext_stream: HDAudio stream to set up.
 * @code_loading: Whether the stream is for PCM or code-loading.
 *
 * Return: Zero on success or negative error code.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_ext_host_stream_setup(
    hext_stream: *mut hdac_ext_stream,
    code_loading: bool,
) -> c_int {
    unsafe { ((*hext_stream).host_setup.unwrap())(hdac_stream(hext_stream), code_loading) }
}

/**
 * snd_hdac_apl_host_stream_setup - Setup a HOST stream following procedure
 *                                  recommended for ApolloLake devices.
 * @hstream: HDAudio stream to set up.
 * @code_loading: Whether the stream is for PCM or code-loading.
 *
 * Return: Zero on success or negative error code.
 */
unsafe extern "C" fn snd_hdac_apl_host_stream_setup(
    hstream: *mut hdac_stream,
    code_loading: bool,
) -> c_int {
    unsafe {
        let hext_stream = stream_to_hdac_ext_stream(hstream);
        snd_hdac_ext_stream_decouple((*hstream).bus, hext_stream, false);
        let ret = snd_hdac_stream_setup(hstream, code_loading);
        snd_hdac_ext_stream_decouple((*hstream).bus, hext_stream, true);
        ret
    }
}

/**
 * snd_hdac_ext_stream_init - initialize each stream (aka device)
 * @bus: HD-audio core bus
 * @hext_stream: HD-audio ext core stream object to initialize
 * @idx: stream index number
 * @direction: stream direction (SNDRV_PCM_STREAM_PLAYBACK or SNDRV_PCM_STREAM_CAPTURE)
 * @tag: the tag id to assign
 *
 * initialize the stream, if ppcap is enabled then init those and then
 * invoke hdac stream initialization routine
 */
unsafe fn snd_hdac_ext_stream_init(
    bus: *mut hdac_bus,
    hext_stream: *mut hdac_ext_stream,
    idx: c_int,
    direction: c_int,
    tag: c_int,
) {
    unsafe {
        if (*bus).ppcap != 0 {
            (*hext_stream).pphc_addr =
                (*bus).ppcap + AZX_PPHC_BASE + AZX_PPHC_INTERVAL * idx as usize;

            (*hext_stream).pplc_addr = (*bus).ppcap
                + AZX_PPLC_BASE
                + AZX_PPLC_MULTI * (*bus).num_streams as usize
                + AZX_PPLC_INTERVAL * idx as usize;
        }

        (*hext_stream).decoupled = false;
        snd_hdac_stream_init(bus, addr_of_mut!((*hext_stream).hstream), idx, direction, tag);
    }
}

/**
 * snd_hdac_ext_stream_init_all - create and initialize the stream objects
 *   for an extended hda bus
 * @bus: HD-audio core bus
 * @start_idx: start index for streams
 * @num_stream: number of streams to initialize
 * @dir: direction of streams
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_ext_stream_init_all(
    bus: *mut hdac_bus,
    start_idx: c_int,
    num_stream: c_int,
    dir: c_int,
) -> c_int {
    unsafe {
        let pci = to_pci_dev((*bus).dev);
        let setup_op: unsafe extern "C" fn(*mut hdac_stream, bool) -> c_int =
            if (*pci).device == PCI_DEVICE_ID_INTEL_HDA_APL {
                snd_hdac_apl_host_stream_setup
            } else {
                snd_hdac_stream_setup
            };
        let mut stream_tag = 0;
        let mut idx = start_idx;

        let mut i = 0;
        while i < num_stream {
            let hext_stream = kzalloc(core::mem::size_of::<hdac_ext_stream>()).cast::<hdac_ext_stream>();
            if hext_stream.is_null() {
                return -ENOMEM;
            }
            let tag = {
                stream_tag += 1;
                stream_tag
            };
            snd_hdac_ext_stream_init(bus, hext_stream, idx, dir, tag);
            idx += 1;
            (*hext_stream).host_setup = Some(setup_op);
            i += 1;
        }

        0
    }
}

/**
 * snd_hdac_ext_stream_free_all - free hdac extended stream objects
 *
 * @bus: HD-audio core bus
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_ext_stream_free_all(bus: *mut hdac_bus) {
    unsafe {
        let head = addr_of_mut!((*bus).stream_list);
        let mut pos = (*head).next;
        while pos != head {
            let s = pos.cast::<hdac_stream>();
            let next = (*pos).next;
            let hext_stream = stream_to_hdac_ext_stream(s);
            snd_hdac_ext_stream_decouple(bus, hext_stream, false);
            list_del(addr_of_mut!((*s).list));
            kfree(hext_stream.cast::<c_void>());
            pos = next;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_hdac_ext_stream_decouple_locked(
    bus: *mut hdac_bus,
    hext_stream: *mut hdac_ext_stream,
    decouple: bool,
) {
    unsafe {
        let hstream = addr_of_mut!((*hext_stream).hstream);
        let mask = AZX_PPCTL_PROCEN((*hstream).index);
        let val = readw((*bus).ppcap + AZX_REG_PP_PPCTL) as c_int & mask;

        if decouple && val == 0 {
            snd_hdac_updatel(
                (*bus).ppcap,
                AZX_REG_PP_PPCTL,
                mask as c_uint,
                mask as c_uint,
            );
        } else if !decouple && val != 0 {
            snd_hdac_updatel((*bus).ppcap, AZX_REG_PP_PPCTL, mask as c_uint, 0);
        }

        (*hext_stream).decoupled = decouple;
    }
}

/**
 * snd_hdac_ext_stream_decouple - decouple the hdac stream
 * @bus: HD-audio core bus
 * @hext_stream: HD-audio ext core stream object to initialize
 * @decouple: flag to decouple
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_ext_stream_decouple(
    bus: *mut hdac_bus,
    hext_stream: *mut hdac_ext_stream,
    decouple: bool,
) {
    unsafe {
        let _guard = SpinLockIrqGuard::new(addr_of_mut!((*bus).reg_lock));
        snd_hdac_ext_stream_decouple_locked(bus, hext_stream, decouple);
    }
}

/**
 * snd_hdac_ext_stream_start - start a stream
 * @hext_stream: HD-audio ext core stream to start
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_ext_stream_start(hext_stream: *mut hdac_ext_stream) {
    unsafe {
        snd_hdac_updatel(
            (*hext_stream).pplc_addr,
            AZX_REG_PPLCCTL,
            AZX_PPLCCTL_RUN,
            AZX_PPLCCTL_RUN,
        );
    }
}

/**
 * snd_hdac_ext_stream_clear - stop a stream DMA
 * @hext_stream: HD-audio ext core stream to stop
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_ext_stream_clear(hext_stream: *mut hdac_ext_stream) {
    unsafe {
        snd_hdac_updatel(
            (*hext_stream).pplc_addr,
            AZX_REG_PPLCCTL,
            AZX_PPLCCTL_RUN,
            0,
        );
    }
}

/**
 * snd_hdac_ext_stream_reset - reset a stream
 * @hext_stream: HD-audio ext core stream to reset
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_ext_stream_reset(hext_stream: *mut hdac_ext_stream) {
    unsafe {
        let mut val: u8;
        let mut timeout: c_int;

        snd_hdac_ext_stream_clear(hext_stream);

        snd_hdac_updatel(
            (*hext_stream).pplc_addr,
            AZX_REG_PPLCCTL,
            AZX_PPLCCTL_STRST,
            AZX_PPLCCTL_STRST,
        );
        udelay(3);
        timeout = 50;
        loop {
            val = (readl((*hext_stream).pplc_addr + AZX_REG_PPLCCTL) & AZX_PPLCCTL_STRST) as u8;
            if val != 0 {
                break;
            }
            udelay(3);
            timeout -= 1;
            if timeout == 0 {
                break;
            }
        }
        snd_hdac_updatel(
            (*hext_stream).pplc_addr,
            AZX_REG_PPLCCTL,
            AZX_PPLCCTL_STRST,
            0,
        );
        udelay(3);

        timeout = 50;
        /* waiting for hardware to report that the stream is out of reset */
        loop {
            val = (readl((*hext_stream).pplc_addr + AZX_REG_PPLCCTL) & AZX_PPLCCTL_STRST) as u8;
            if val == 0 {
                break;
            }
            udelay(3);
            timeout -= 1;
            if timeout == 0 {
                break;
            }
        }
    }
}

/**
 * snd_hdac_ext_stream_setup -  set up the SD for streaming
 * @hext_stream: HD-audio ext core stream to set up
 * @fmt: stream format
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_ext_stream_setup(
    hext_stream: *mut hdac_ext_stream,
    fmt: c_int,
) -> c_int {
    unsafe {
        let hstream = addr_of_mut!((*hext_stream).hstream);
        let mut val: c_uint;

        /* make sure the run bit is zero for SD */
        snd_hdac_ext_stream_clear(hext_stream);

        /* program the stream_tag */
        val = readl((*hext_stream).pplc_addr + AZX_REG_PPLCCTL);
        val = (val & !AZX_PPLCCTL_STRM_MASK)
            | (((*hstream).stream_tag as c_uint) << AZX_PPLCCTL_STRM_SHIFT);
        writel(val, (*hext_stream).pplc_addr + AZX_REG_PPLCCTL);

        /* program the stream format */
        writew(fmt, (*hext_stream).pplc_addr + AZX_REG_PPLCFMT);

        0
    }
}

unsafe fn hdac_ext_link_dma_stream_assign(
    bus: *mut hdac_bus,
    substream: *mut snd_pcm_substream,
) -> *mut hdac_ext_stream {
    unsafe {
        let mut res: *mut hdac_ext_stream = null_mut();

        if (*bus).ppcap == 0 {
            dev_err((*bus).dev, c"stream type not supported\n".as_ptr());
            return null_mut();
        }

        let _guard = SpinLockIrqGuard::new(addr_of_mut!((*bus).reg_lock));
        for_each_stream(bus, |hstream| {
            let hext_stream = hstream.cast::<hdac_ext_stream>();
            if (*hstream).direction != (*substream).stream {
                return true;
            }

            /* check if link stream is available */
            if (*hext_stream).link_locked == 0 {
                res = hext_stream;
                return false;
            }
            true
        });
        if !res.is_null() {
            snd_hdac_ext_stream_decouple_locked(bus, res, true);
            (*res).link_locked = 1;
            (*res).link_substream = substream;
        }
        res
    }
}

unsafe fn hdac_ext_host_dma_stream_assign(
    bus: *mut hdac_bus,
    substream: *mut snd_pcm_substream,
) -> *mut hdac_ext_stream {
    unsafe {
        let mut res: *mut hdac_ext_stream = null_mut();

        if (*bus).ppcap == 0 {
            dev_err((*bus).dev, c"stream type not supported\n".as_ptr());
            return null_mut();
        }

        let _guard = SpinLockIrqGuard::new(addr_of_mut!((*bus).reg_lock));
        for_each_stream(bus, |hstream| {
            let hext_stream = hstream.cast::<hdac_ext_stream>();
            if (*hstream).direction != (*substream).stream {
                return true;
            }

            if (*hstream).opened == 0 {
                res = hext_stream;
                return false;
            }
            true
        });
        if !res.is_null() {
            snd_hdac_ext_stream_decouple_locked(bus, res, true);
            (*res).hstream.opened = 1;
            (*res).hstream.running = 0;
            (*res).hstream.substream = substream;
        }

        res
    }
}

/**
 * snd_hdac_ext_stream_assign - assign a stream for the PCM
 * @bus: HD-audio core bus
 * @substream: PCM substream to assign
 * @type: type of stream (coupled, host or link stream)
 *
 * This assigns the stream based on the type (coupled/host/link), for the
 * given PCM substream, assigns it and returns the stream object
 *
 * coupled: Looks for an unused stream
 * host: Looks for an unused decoupled host stream
 * link: Looks for an unused decoupled link stream
 *
 * If no stream is free, returns NULL. The function tries to keep using
 * the same stream object when it's used beforehand.  when a stream is
 * decoupled, it becomes a host stream and link stream.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_ext_stream_assign(
    bus: *mut hdac_bus,
    substream: *mut snd_pcm_substream,
    type_: c_int,
) -> *mut hdac_ext_stream {
    unsafe {
        let mut hext_stream: *mut hdac_ext_stream = null_mut();

        match type_ {
            HDAC_EXT_STREAM_TYPE_COUPLED => {
                let hstream = snd_hdac_stream_assign(bus, substream);
                if !hstream.is_null() {
                    hext_stream = hstream.cast::<hdac_ext_stream>();
                }
                hext_stream
            }

            HDAC_EXT_STREAM_TYPE_HOST => hdac_ext_host_dma_stream_assign(bus, substream),

            HDAC_EXT_STREAM_TYPE_LINK => hdac_ext_link_dma_stream_assign(bus, substream),

            _ => null_mut(),
        }
    }
}

/**
 * snd_hdac_ext_stream_release - release the assigned stream
 * @hext_stream: HD-audio ext core stream to release
 * @type: type of stream (coupled, host or link stream)
 *
 * Release the stream that has been assigned by snd_hdac_ext_stream_assign().
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_ext_stream_release(
    hext_stream: *mut hdac_ext_stream,
    type_: c_int,
) {
    unsafe {
        let bus = (*hext_stream).hstream.bus;

        match type_ {
            HDAC_EXT_STREAM_TYPE_COUPLED => {
                snd_hdac_stream_release(addr_of_mut!((*hext_stream).hstream));
            }

            HDAC_EXT_STREAM_TYPE_HOST => {
                let _guard = SpinLockIrqGuard::new(addr_of_mut!((*bus).reg_lock));
                /* couple link only if not in use */
                if (*hext_stream).link_locked == 0 {
                    snd_hdac_ext_stream_decouple_locked(bus, hext_stream, false);
                }
                snd_hdac_stream_release_locked(addr_of_mut!((*hext_stream).hstream));
            }

            HDAC_EXT_STREAM_TYPE_LINK => {
                let _guard = SpinLockIrqGuard::new(addr_of_mut!((*bus).reg_lock));
                /* couple host only if not in use */
                if (*hext_stream).hstream.opened == 0 {
                    snd_hdac_ext_stream_decouple_locked(bus, hext_stream, false);
                }
                (*hext_stream).link_locked = 0;
                (*hext_stream).link_substream = null_mut();
            }

            _ => {
                dev_dbg((*bus).dev, c"Invalid type %d\n".as_ptr(), type_);
            }
        }
    }
}

/**
 * snd_hdac_ext_cstream_assign - assign a host stream for compress
 * @bus: HD-audio core bus
 * @cstream: Compress stream to assign
 *
 * Assign an unused host stream for the given compress stream.
 * If no stream is free, NULL is returned. Stream is decoupled
 * before assignment.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_ext_cstream_assign(
    bus: *mut hdac_bus,
    cstream: *mut snd_compr_stream,
) -> *mut hdac_ext_stream {
    unsafe {
        let mut res: *mut hdac_ext_stream = null_mut();

        let _guard = SpinLockIrqGuard::new(addr_of_mut!((*bus).reg_lock));
        for_each_stream(bus, |hstream| {
            let hext_stream = stream_to_hdac_ext_stream(hstream);

            if (*hstream).direction != (*cstream).direction {
                return true;
            }

            if (*hstream).opened == 0 {
                res = hext_stream;
                return false;
            }
            true
        });

        if !res.is_null() {
            snd_hdac_ext_stream_decouple_locked(bus, res, true);
            (*res).hstream.opened = 1;
            (*res).hstream.running = 0;
            (*res).hstream.cstream = cstream;
        }

        res
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
