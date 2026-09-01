// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PMac DBDMA lowlevel functions
 *
 * Copyright (c) by Takashi Iwai <tiwai@suse.de>
 * code based on dmasound.c.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type snd_pcm_uframes_t = c_ulong;
type dma_addr_t = c_ulong;
type irqreturn_t = c_int;

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const ENODEV: c_int = 19;
const IRQ_HANDLED: irqreturn_t = 1;

const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 2;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 3;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 0;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 1;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 2;
const SNDRV_PCM_INFO_RESUME: c_uint = 1 << 3;
const SNDRV_PCM_INFO_HALF_DUPLEX: c_uint = 1 << 4;
const SNDRV_PCM_INFO_JOINT_DUPLEX: c_uint = 1 << 5;
const SNDRV_PCM_FMTBIT_S16_BE: c_ulong = 1 << 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 1 << 1;
const SNDRV_PCM_RATE_8000_44100: c_uint = 0;
const SNDRV_PCM_HW_PARAM_PERIODS: c_int = 0;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const SNDRV_DEV_LOWLEVEL: c_int = 0;
const SNDRV_CTL_POWER_D3hot: c_int = 0;
const SNDRV_CTL_POWER_D0: c_int = 0;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 0;
const SNDRV_CTL_ELEM_ACCESS_READ: c_uint = 1;

const RUN: c_uint = 1 << 0;
const WAKE: c_uint = 1 << 1;
const FLUSH: c_uint = 1 << 2;
const PAUSE: c_uint = 1 << 3;
const DEAD: c_uint = 1 << 4;
const ACTIVE: c_uint = 1 << 5;
const DBDMA_STOP: c_uint = 0;
const DBDMA_NOP: c_uint = 1;
const BR_ALWAYS: c_uint = 1 << 12;
const INTR_ALWAYS: c_uint = 1 << 13;
const OUTPUT_MORE: c_uint = 2;
const INPUT_MORE: c_uint = 3;
const MASK_PORTCHG: c_uint = 1 << 0;
const MASK_CNTLERR: c_uint = 1 << 1;
const MASK_ERRCODE: c_uint = 0x00ff0000;
const MASK_IEPC: c_uint = 1 << 0;
const MASK_IEE: c_uint = 1 << 1;
const PMAC_AWACS: c_int = 0;
const PMAC_SCREAMER: c_int = 1;
const PMAC_BURGUNDY: c_int = 2;
const PMAC_DACA: c_int = 3;
const PMAC_TUMBLER: c_int = 4;
const PMAC_SNAPPER: c_int = 5;
const PMAC_MAX_FRAGS: c_uint = 32;
const PMAC_FTR_SOUND_CHIP_ENABLE: c_int = 0;
const macio_unknown: c_int = 0;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
}

#[repr(C)]
pub struct snd_card {
    pub driver: *const c_char,
    pub shortname: *const c_char,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm {
    pub private_data: *mut c_void,
    pub info_flags: c_uint,
    pub name: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: c_ulong,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub buffer_bytes_max: c_uint,
    pub period_bytes_min: c_uint,
    pub period_bytes_max: c_uint,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hardware,
    pub rate: c_uint,
    pub format: c_int,
    pub dma_addr: dma_addr_t,
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

#[repr(C)]
pub struct snd_device {
    pub device_data: *mut c_void,
}

#[repr(C)]
pub struct snd_device_ops {
    pub dev_free: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_integer {
    pub value: [c_long; 128],
}

type c_long = isize;

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: core::mem::ManuallyDrop<snd_ctl_elem_integer>,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub access: c_uint,
    pub info: Option<unsafe extern "C" fn() -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
pub struct resource {
    pub start: c_ulong,
}

#[repr(C)]
pub struct device_node {
    pub parent: *mut device_node,
}

#[repr(C)]
pub struct macio_chip {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct ppc_md_t {
    pub feature_call: Option<unsafe extern "C" fn(c_int, *mut device_node, c_int, c_int)>,
}

#[repr(C)]
pub struct dbdma_regs {
    pub control: c_uint,
    pub status: c_uint,
    pub cmdptr: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbdma_cmd {
    pub req_count: u16,
    pub command: u16,
    pub phy_addr: u32,
    pub cmd_dep: u32,
    pub res_count: u16,
    pub xfer_status: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmac_dbdma {
    pub space: *mut c_void,
    pub dma_base: dma_addr_t,
    pub size: c_int,
    pub cmds: *mut dbdma_cmd,
    pub addr: dma_addr_t,
}

impl Default for pmac_dbdma {
    fn default() -> Self {
        Self {
            space: ptr::null_mut(),
            dma_base: 0,
            size: 0,
            cmds: ptr::null_mut(),
            addr: 0,
        }
    }
}

#[repr(C)]
pub struct pmac_stream {
    pub dma: *mut dbdma_regs,
    pub cmd: pmac_dbdma,
    pub stream: c_int,
    pub cur_freqs: c_int,
    pub cur_formats: c_ulong,
    pub dma_size: c_int,
    pub period_size: c_int,
    pub nperiods: c_int,
    pub cur_period: c_int,
    pub running: c_int,
    pub substream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct awacs_regs {
    pub control: c_uint,
    pub byteswap: c_uint,
    pub codec_stat: c_uint,
}

#[repr(C)]
pub struct snd_pmac {
    pub card: *mut snd_card,
    pub pdev: *mut pci_dev,
    pub pcm: *mut snd_pcm,
    pub playback: pmac_stream,
    pub capture: pmac_stream,
    pub extra_dma: pmac_dbdma,
    pub awacs: *mut awacs_regs,
    pub macio_base: *mut u8,
    pub latch_base: *mut u8,
    pub node: *mut device_node,
    pub rsrc: [resource; 3],
    pub requested: c_int,
    pub reg_lock: c_int,
    pub irq: c_int,
    pub tx_irq: c_int,
    pub rx_irq: c_int,
    pub initialized: c_int,
    pub subframe: c_int,
    pub revision: c_int,
    pub freqs_ok: c_int,
    pub model: c_int,
    pub can_byte_swap: c_int,
    pub can_duplex: c_int,
    pub can_capture: c_int,
    pub num_freqs: c_int,
    pub freq_table: *const c_int,
    pub control_mask: c_uint,
    pub is_pbook_3400: c_int,
    pub is_pbook_G3: c_int,
    pub is_k2: c_int,
    pub has_iic: bool,
    pub device_id: c_uint,
    pub rate_index: c_int,
    pub format: c_int,
    pub auto_mute: c_int,
    pub hp_detect_ctl: *mut snd_kcontrol,
    pub set_format: Option<unsafe extern "C" fn(*mut snd_pmac)>,
    pub update_automute: Option<unsafe extern "C" fn(*mut snd_pmac, c_int)>,
    pub mixer_free: Option<unsafe extern "C" fn(*mut snd_pmac)>,
    pub detect_headphone: Option<unsafe extern "C" fn(*mut snd_pmac) -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_pmac)>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_pmac)>,
}

unsafe extern "C" {
    static mut ppc_md: ppc_md_t;
    fn dma_alloc_coherent(dev: *mut device, size: c_uint, dma_handle: *mut dma_addr_t, flag: c_uint) -> *mut c_void;
    fn dma_free_coherent(dev: *mut device, size: c_uint, vaddr: *mut c_void, dma_handle: dma_addr_t);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn in_le32(addr: *const c_uint) -> c_uint;
    fn out_le32(addr: *mut c_uint, val: c_uint);
    fn in_le16(addr: *const u16) -> u16;
    fn out_le16(addr: *mut u16, val: c_uint);
    fn in_8(addr: *mut u8) -> u8;
    fn out_8(addr: *mut u8, val: u8);
    fn udelay(usecs: c_ulong);
    fn mdelay(msecs: c_ulong);
    fn snd_BUG();
    fn snd_pcm_lib_buffer_bytes(subs: *mut snd_pcm_substream) -> c_int;
    fn snd_pcm_lib_period_bytes(subs: *mut snd_pcm_substream) -> c_int;
    fn snd_pcm_substream_chip(subs: *mut snd_pcm_substream) -> *mut snd_pmac;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, size: c_int) -> snd_pcm_uframes_t;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pmac_beep_stop(chip: *mut snd_pmac);
    fn snd_pcm_rate_to_rate_bit(rate: c_int) -> c_uint;
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, var: c_int) -> c_int;
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback_count: c_int, capture_count: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, ty: c_int, dev: *mut device, size: usize, max: usize);
    fn strscpy(dst: *mut c_char, src: *const c_char);
    fn free_irq(irq: c_uint, dev_id: *mut c_void);
    fn iounmap(addr: *mut c_void);
    fn release_mem_region(start: c_ulong, n: c_ulong);
    fn resource_size(res: *const resource) -> c_ulong;
    fn pci_dev_put(dev: *mut pci_dev);
    fn of_node_put(node: *mut device_node);
    fn kfree(p: *mut c_void);
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn disable_irq(irq: c_uint);
    fn enable_irq(irq: c_uint);
    fn of_node_name_eq(node: *mut device_node, name: *const c_char) -> bool;
    fn of_device_is_compatible(node: *mut device_node, compat: *const c_char) -> bool;
    fn of_machine_is_compatible(compat: *const c_char) -> bool;
    fn machine_is_powermac() -> bool;
    fn of_find_node_by_name(from: *mut device_node, name: *const c_char) -> *mut device_node;
    fn of_node_get(node: *mut device_node) -> *mut device_node;
    fn of_get_property(node: *mut device_node, name: *const c_char, lenp: *mut c_uint) -> *const c_uint;
    fn macio_find(node: *mut device_node, ty: c_int) -> *mut macio_chip;
    fn for_each_pci_dev_next(pdev: *mut pci_dev) -> *mut pci_dev;
    fn pci_device_to_OF_node(pdev: *mut pci_dev) -> *mut device_node;
    fn of_address_to_resource(dev: *mut device_node, index: c_int, r: *mut resource) -> c_int;
    fn request_mem_region(start: c_ulong, n: c_ulong, name: *const c_char) -> *mut c_void;
    fn ioremap(addr: c_ulong, size: c_ulong) -> *mut c_void;
    fn irq_of_parse_and_map(dev: *mut device_node, index: c_int) -> c_uint;
    fn request_irq(irq: c_uint, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, dev: *mut c_void) -> c_int;
    fn snd_device_new(card: *mut snd_card, ty: c_int, data: *mut c_void, ops: *const snd_device_ops) -> c_int;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_pmac;
    fn snd_pmac_boolean_mono_info() -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

const fn cpu_to_le16(v: c_uint) -> u16 {
    v as u16
}

const fn cpu_to_le32(v: c_ulong) -> u32 {
    v as u32
}

const fn le16_to_cpu(v: u16) -> c_uint {
    v as c_uint
}

const fn le32_to_cpu(v: u32) -> c_uint {
    v as c_uint
}

fn DBDMA_ALIGN(p: *mut c_void) -> *mut c_void {
    let addr = p as usize;
    ((addr + 15) & !15usize) as *mut c_void
}

/* fixed frequency table for awacs, screamer, burgundy, DACA (44100 max) */
static awacs_freqs: [c_int; 8] = [44100, 29400, 22050, 17640, 14700, 11025, 8820, 7350];
/* fixed frequency table for tumbler */
static tumbler_freqs: [c_int; 1] = [44100];

/*
 * we will allocate a single 'emergency' dbdma cmd block to use if the
 * tx status comes up "DEAD".  This happens on some PowerComputing Pmac
 * clones, either owing to a bug in dbdma or some interaction between
 * IDE and sound.  However, this measure would deal with DEAD status if
 * it appeared elsewhere.
 */
static mut emergency_dbdma: pmac_dbdma = pmac_dbdma {
    space: ptr::null_mut(),
    dma_base: 0,
    size: 0,
    cmds: ptr::null_mut(),
    addr: 0,
};
static mut emergency_in_use: c_int = 0;

/*
 * allocate DBDMA command arrays
 */
unsafe fn snd_pmac_dbdma_alloc(chip: *mut snd_pmac, rec: *mut pmac_dbdma, size: c_int) -> c_int {
    let rsize = (size_of::<dbdma_cmd>() * (size as usize + 1)) as c_uint;

    (*rec).space = dma_alloc_coherent(
        &mut (*(*chip).pdev).dev,
        rsize,
        &mut (*rec).dma_base,
        GFP_KERNEL,
    );
    if (*rec).space.is_null() {
        return -ENOMEM;
    }
    (*rec).size = size;
    memset((*rec).space, 0, rsize as usize);
    (*rec).cmds = DBDMA_ALIGN((*rec).space) as *mut dbdma_cmd;
    (*rec).addr = (*rec).dma_base + ((*rec).cmds as *mut c_char).offset_from((*rec).space as *mut c_char) as c_ulong;

    0
}

unsafe fn snd_pmac_dbdma_free(chip: *mut snd_pmac, rec: *mut pmac_dbdma) {
    if !(*rec).space.is_null() {
        let rsize = (size_of::<dbdma_cmd>() * ((*rec).size as usize + 1)) as c_uint;
        dma_free_coherent(&mut (*(*chip).pdev).dev, rsize, (*rec).space, (*rec).dma_base);
    }
}

/*
 * pcm stuff
 */

/*
 * look up frequency table
 */
#[no_mangle]
pub unsafe extern "C" fn snd_pmac_rate_index(chip: *mut snd_pmac, rec: *mut pmac_stream, rate: c_uint) -> c_uint {
    let mut ok: c_int;
    let mut found: c_int;

    ok = (*rec).cur_freqs;
    if rate > *(*chip).freq_table.offset(0) as c_uint {
        return 0;
    }
    found = 0;
    let mut i = 0;
    while i < (*chip).num_freqs {
        if (ok & 1) != 0 {
            found = i;
            if rate >= *(*chip).freq_table.offset(i as isize) as c_uint {
                break;
            }
        }
        i += 1;
        ok >>= 1;
    }
    found as c_uint
}

/*
 * check whether another stream is active
 */
#[inline]
unsafe fn another_stream(stream: c_int) -> c_int {
    if stream == SNDRV_PCM_STREAM_PLAYBACK {
        SNDRV_PCM_STREAM_CAPTURE
    } else {
        SNDRV_PCM_STREAM_PLAYBACK
    }
}

/*
 * get a stream of the opposite direction
 */
unsafe fn snd_pmac_get_stream(chip: *mut snd_pmac, stream: c_int) -> *mut pmac_stream {
    match stream {
        SNDRV_PCM_STREAM_PLAYBACK => &mut (*chip).playback,
        SNDRV_PCM_STREAM_CAPTURE => &mut (*chip).capture,
        _ => {
            snd_BUG();
            ptr::null_mut()
        }
    }
}

/*
 * wait while run status is on
 */
#[inline]
unsafe fn snd_pmac_wait_ack(rec: *mut pmac_stream) {
    let mut timeout = 50000;
    while (in_le32(&(*(*rec).dma).status) & RUN) != 0 && timeout > 0 {
        timeout -= 1;
        udelay(1);
    }
}

/*
 * set the format and rate to the chip.
 * call the lowlevel function if defined (e.g. for AWACS).
 */
unsafe fn snd_pmac_pcm_set_format(chip: *mut snd_pmac) {
    /* set up frequency and format */
    out_le32(&mut (*(*chip).awacs).control, (*chip).control_mask | (((*chip).rate_index as c_uint) << 8));
    out_le32(
        &mut (*(*chip).awacs).byteswap,
        if (*chip).format == SNDRV_PCM_FORMAT_S16_LE { 1 } else { 0 },
    );
    if let Some(set_format) = (*chip).set_format {
        set_format(chip);
    }
}

/*
 * stop the DMA transfer
 */
#[inline]
unsafe fn snd_pmac_dma_stop(rec: *mut pmac_stream) {
    out_le32(&mut (*(*rec).dma).control, (RUN | WAKE | FLUSH | PAUSE) << 16);
    snd_pmac_wait_ack(rec);
}

/*
 * set the command pointer address
 */
#[inline]
unsafe fn snd_pmac_dma_set_command(rec: *mut pmac_stream, cmd: *mut pmac_dbdma) {
    out_le32(&mut (*(*rec).dma).cmdptr, (*cmd).addr as c_uint);
}

/*
 * start the DMA
 */
#[inline]
unsafe fn snd_pmac_dma_run(rec: *mut pmac_stream, status: c_int) {
    let status = status as c_uint;
    out_le32(&mut (*(*rec).dma).control, status | (status << 16));
}

/*
 * prepare playback/capture stream
 */
unsafe fn snd_pmac_pcm_prepare(chip: *mut snd_pmac, rec: *mut pmac_stream, subs: *mut snd_pcm_substream) -> c_int {
    let runtime = (*subs).runtime;
    let rate_index: c_int;
    let mut offset: c_ulong;
    let astr: *mut pmac_stream;

    (*rec).dma_size = snd_pcm_lib_buffer_bytes(subs);
    (*rec).period_size = snd_pcm_lib_period_bytes(subs);
    (*rec).nperiods = (*rec).dma_size / (*rec).period_size;
    (*rec).cur_period = 0;
    rate_index = snd_pmac_rate_index(chip, rec, (*runtime).rate) as c_int;

    /* set up constraints */
    astr = snd_pmac_get_stream(chip, another_stream((*rec).stream));
    if astr.is_null() {
        return -EINVAL;
    }
    (*astr).cur_freqs = 1 << rate_index;
    (*astr).cur_formats = 1 << (*runtime).format;
    (*chip).rate_index = rate_index;
    (*chip).format = (*runtime).format;

    /* We really want to execute a DMA stop command, after the AWACS
     * is initialized.
     * For reasons I don't understand, it stops the hissing noise
     * common to many PowerBook G3 systems and random noise otherwise
     * captured on iBook2's about every third time. -ReneR
     */
    snd_pmac_dma_stop(rec);
    (*(*chip).extra_dma.cmds).command = cpu_to_le16(DBDMA_STOP);
    snd_pmac_dma_set_command(rec, &mut (*chip).extra_dma);
    snd_pmac_dma_run(rec, RUN as c_int);
    mdelay(5);

    /* continuous DMA memory type doesn't provide the physical address,
     * so we need to resolve the address here...
     */
    offset = (*runtime).dma_addr;
    let mut i = 0;
    let mut cp = (*rec).cmd.cmds;
    while i < (*rec).nperiods {
        (*cp).phy_addr = cpu_to_le32(offset);
        (*cp).req_count = cpu_to_le16((*rec).period_size as c_uint);
        /*(*cp).res_count = cpu_to_le16(0);*/
        (*cp).xfer_status = cpu_to_le16(0);
        offset += (*rec).period_size as c_ulong;
        i += 1;
        cp = cp.add(1);
    }
    /* make loop */
    (*cp).command = cpu_to_le16(DBDMA_NOP | BR_ALWAYS);
    (*cp).cmd_dep = cpu_to_le32((*rec).cmd.addr);

    snd_pmac_dma_stop(rec);
    snd_pmac_dma_set_command(rec, &mut (*rec).cmd);

    0
}

/*
 * PCM trigger/stop
 */
unsafe fn snd_pmac_pcm_trigger(chip: *mut snd_pmac, rec: *mut pmac_stream, subs: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let mut cp: *mut dbdma_cmd;
    let command: c_int;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
            if (*rec).running != 0 {
                return -EBUSY;
            }
            command = (if (*subs).stream == SNDRV_PCM_STREAM_PLAYBACK { OUTPUT_MORE } else { INPUT_MORE } | INTR_ALWAYS) as c_int;
            snd_pmac_beep_stop(chip);
            snd_pmac_pcm_set_format(chip);
            let mut i = 0;
            cp = (*rec).cmd.cmds;
            while i < (*rec).nperiods {
                out_le16(&mut (*cp).command, command as c_uint);
                i += 1;
                cp = cp.add(1);
            }
            snd_pmac_dma_set_command(rec, &mut (*rec).cmd);
            let _ = in_le32(&(*(*rec).dma).status);
            snd_pmac_dma_run(rec, (RUN | WAKE) as c_int);
            (*rec).running = 1;
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
            (*rec).running = 0;
            snd_pmac_dma_stop(rec);
            let mut i = 0;
            cp = (*rec).cmd.cmds;
            while i < (*rec).nperiods {
                out_le16(&mut (*cp).command, DBDMA_STOP);
                i += 1;
                cp = cp.add(1);
            }
        }
        _ => return -EINVAL,
    }

    0
}

/*
 * return the current pointer
 */
#[inline]
unsafe fn snd_pmac_pcm_pointer(_chip: *mut snd_pmac, rec: *mut pmac_stream, subs: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let mut count: c_int = 0;

    /* #if 1: hmm.. how can we get the current dma pointer?? */
    let mut stat: c_int;
    let cp = (*rec).cmd.cmds.offset((*rec).cur_period as isize);
    stat = le16_to_cpu((*cp).xfer_status) as c_int;
    if (stat as c_uint & (ACTIVE | DEAD)) != 0 {
        count = in_le16(&(*cp).res_count) as c_int;
        if count != 0 {
            count = (*rec).period_size - count;
        }
    }
    count += (*rec).cur_period * (*rec).period_size;
    bytes_to_frames((*subs).runtime, count)
}

/*
 * playback
 */

unsafe extern "C" fn snd_pmac_playback_prepare(subs: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(subs);
    snd_pmac_pcm_prepare(chip, &mut (*chip).playback, subs)
}

unsafe extern "C" fn snd_pmac_playback_trigger(subs: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(subs);
    snd_pmac_pcm_trigger(chip, &mut (*chip).playback, subs, cmd)
}

unsafe extern "C" fn snd_pmac_playback_pointer(subs: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(subs);
    snd_pmac_pcm_pointer(chip, &mut (*chip).playback, subs)
}

/*
 * capture
 */

unsafe extern "C" fn snd_pmac_capture_prepare(subs: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(subs);
    snd_pmac_pcm_prepare(chip, &mut (*chip).capture, subs)
}

unsafe extern "C" fn snd_pmac_capture_trigger(subs: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(subs);
    snd_pmac_pcm_trigger(chip, &mut (*chip).capture, subs, cmd)
}

unsafe extern "C" fn snd_pmac_capture_pointer(subs: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(subs);
    snd_pmac_pcm_pointer(chip, &mut (*chip).capture, subs)
}

/*
 * Handle DEAD DMA transfers:
 * if the TX status comes up "DEAD" - reported on some Power Computing machines
 * we need to re-start the dbdma - but from a different physical start address
 * and with a different transfer length.  It would get very messy to do this
 * with the normal dbdma_cmd blocks - we would have to re-write the buffer start
 * addresses each time.  So, we will keep a single dbdma_cmd block which can be
 * fiddled with.
 * When DEAD status is first reported the content of the faulted dbdma block is
 * copied into the emergency buffer and we note that the buffer is in use.
 * we then bump the start physical address by the amount that was successfully
 * output before it died.
 * On any subsequent DEAD result we just do the bump-ups (we know that we are
 * already using the emergency dbdma_cmd).
 * CHECK: this just tries to "do it".  It is possible that we should abandon
 * xfers when the number of residual bytes gets below a certain value - I can
 * see that this might cause a loop-forever if a too small transfer causes
 * DEAD status.  However this is a TODO for now - we'll see what gets reported.
 * When we get a successful transfer result with the emergency buffer we just
 * pretend that it completed using the original dmdma_cmd and carry on.  The
 * 'next_cmd' field will already point back to the original loop of blocks.
 */
#[inline]
unsafe fn snd_pmac_pcm_dead_xfer(rec: *mut pmac_stream, mut cp: *mut dbdma_cmd) {
    let req: c_uint;
    let res: c_uint;
    let mut phy: c_uint;

    /* to clear DEAD status we must first clear RUN
       set it to quiescent to be on the safe side */
    let _ = in_le32(&(*(*rec).dma).status);
    out_le32(&mut (*(*rec).dma).control, (RUN | PAUSE | FLUSH | WAKE) << 16);

    if emergency_in_use == 0 {
        /* new problem */
        memcpy(emergency_dbdma.cmds as *mut c_void, cp as *const c_void, size_of::<dbdma_cmd>());
        emergency_in_use = 1;
        (*cp).xfer_status = cpu_to_le16(0);
        (*cp).req_count = cpu_to_le16((*rec).period_size as c_uint);
        cp = emergency_dbdma.cmds;
    }

    /* now bump the values to reflect the amount
       we haven't yet shifted */
    req = le16_to_cpu((*cp).req_count);
    res = le16_to_cpu((*cp).res_count);
    phy = le32_to_cpu((*cp).phy_addr);
    phy = phy.wrapping_add(req.wrapping_sub(res));
    (*cp).req_count = cpu_to_le16(res);
    (*cp).res_count = cpu_to_le16(0);
    (*cp).xfer_status = cpu_to_le16(0);
    (*cp).phy_addr = cpu_to_le32(phy as c_ulong);

    (*cp).cmd_dep = cpu_to_le32(
        (*rec).cmd.addr + size_of::<dbdma_cmd>() as c_ulong * (((*rec).cur_period + 1) % (*rec).nperiods) as c_ulong,
    );

    (*cp).command = cpu_to_le16(OUTPUT_MORE | BR_ALWAYS | INTR_ALWAYS);

    /* point at our patched up command block */
    out_le32(&mut (*(*rec).dma).cmdptr, emergency_dbdma.addr as c_uint);

    /* we must re-start the controller */
    let _ = in_le32(&(*(*rec).dma).status);
    /* should complete clearing the DEAD status */
    out_le32(&mut (*(*rec).dma).control, ((RUN | WAKE) << 16) + (RUN | WAKE));
}

/*
 * update playback/capture pointer from interrupts
 */
unsafe fn snd_pmac_pcm_update(chip: *mut snd_pmac, rec: *mut pmac_stream) {
    let mut cp: *mut dbdma_cmd;
    let mut stat: c_int;

    let _ = chip;
    if (*rec).running != 0 {
        let mut c = 0;
        while c < (*rec).nperiods {
            /* at most all fragments */

            if emergency_in_use != 0 {
                /* already using DEAD xfer? */
                cp = emergency_dbdma.cmds;
            } else {
                cp = (*rec).cmd.cmds.offset((*rec).cur_period as isize);
            }

            stat = le16_to_cpu((*cp).xfer_status) as c_int;

            if (stat as c_uint & DEAD) != 0 {
                snd_pmac_pcm_dead_xfer(rec, cp);
                break; /* this block is still going */
            }

            if emergency_in_use != 0 {
                emergency_in_use = 0; /* done that */
            }

            if (stat as c_uint & ACTIVE) == 0 {
                break;
            }

            (*cp).xfer_status = cpu_to_le16(0);
            (*cp).req_count = cpu_to_le16((*rec).period_size as c_uint);
            /*(*cp).res_count = cpu_to_le16(0);*/
            (*rec).cur_period += 1;
            if (*rec).cur_period >= (*rec).nperiods {
                (*rec).cur_period = 0;
            }

            snd_pcm_period_elapsed((*rec).substream);
            c += 1;
        }
    }
}

/*
 * hw info
 */

static snd_pmac_playback: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_RESUME,
    formats: SNDRV_PCM_FMTBIT_S16_BE | SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_8000_44100,
    rate_min: 7350,
    rate_max: 44100,
    channels_min: 2,
    channels_max: 2,
    buffer_bytes_max: 131072,
    period_bytes_min: 256,
    period_bytes_max: 16384,
    periods_min: 3,
    periods_max: PMAC_MAX_FRAGS,
};

static snd_pmac_capture: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_RESUME,
    formats: SNDRV_PCM_FMTBIT_S16_BE | SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_8000_44100,
    rate_min: 7350,
    rate_max: 44100,
    channels_min: 2,
    channels_max: 2,
    buffer_bytes_max: 131072,
    period_bytes_min: 256,
    period_bytes_max: 16384,
    periods_min: 3,
    periods_max: PMAC_MAX_FRAGS,
};

/* #if 0 // NYI: snd_pmac_hw_rule_rate and snd_pmac_hw_rule_format omitted by the C preprocessor. */

unsafe fn snd_pmac_pcm_open(chip: *mut snd_pmac, rec: *mut pmac_stream, subs: *mut snd_pcm_substream) -> c_int {
    let runtime = (*subs).runtime;
    let mut i: c_int;

    /* look up frequency table and fill bit mask */
    (*runtime).hw.rates = 0;
    i = 0;
    while i < (*chip).num_freqs {
        if ((*chip).freqs_ok & (1 << i)) != 0 {
            (*runtime).hw.rates |= snd_pcm_rate_to_rate_bit(*(*chip).freq_table.offset(i as isize));
        }
        i += 1;
    }

    /* check for minimum and maximum rates */
    i = 0;
    while i < (*chip).num_freqs {
        if ((*chip).freqs_ok & (1 << i)) != 0 {
            (*runtime).hw.rate_max = *(*chip).freq_table.offset(i as isize) as c_uint;
            break;
        }
        i += 1;
    }
    i = (*chip).num_freqs - 1;
    while i >= 0 {
        if ((*chip).freqs_ok & (1 << i)) != 0 {
            (*runtime).hw.rate_min = *(*chip).freq_table.offset(i as isize) as c_uint;
            break;
        }
        i -= 1;
    }
    (*runtime).hw.formats = (*chip).formats_ok;
    if (*chip).can_capture != 0 {
        if (*chip).can_duplex == 0 {
            (*runtime).hw.info |= SNDRV_PCM_INFO_HALF_DUPLEX;
        }
        (*runtime).hw.info |= SNDRV_PCM_INFO_JOINT_DUPLEX;
    }
    (*runtime).private_data = rec as *mut c_void;
    (*rec).substream = subs;

    /* #if 0 FIXME: still under development.. */

    (*runtime).hw.periods_max = ((*rec).cmd.size - 1) as c_uint;

    /* constraints to fix choppy sound */
    snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    0
}

unsafe fn snd_pmac_pcm_close(chip: *mut snd_pmac, rec: *mut pmac_stream, _subs: *mut snd_pcm_substream) -> c_int {
    let astr: *mut pmac_stream;

    snd_pmac_dma_stop(rec);

    astr = snd_pmac_get_stream(chip, another_stream((*rec).stream));
    if astr.is_null() {
        return -EINVAL;
    }

    /* reset constraints */
    (*astr).cur_freqs = (*chip).freqs_ok;
    (*astr).cur_formats = (*chip).formats_ok;

    0
}

unsafe extern "C" fn snd_pmac_playback_open(subs: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(subs);

    (*(*subs).runtime).hw = snd_pmac_playback;
    snd_pmac_pcm_open(chip, &mut (*chip).playback, subs)
}

unsafe extern "C" fn snd_pmac_capture_open(subs: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(subs);

    (*(*subs).runtime).hw = snd_pmac_capture;
    snd_pmac_pcm_open(chip, &mut (*chip).capture, subs)
}

unsafe extern "C" fn snd_pmac_playback_close(subs: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(subs);

    snd_pmac_pcm_close(chip, &mut (*chip).playback, subs)
}

unsafe extern "C" fn snd_pmac_capture_close(subs: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(subs);

    snd_pmac_pcm_close(chip, &mut (*chip).capture, subs)
}

/*
 */

static snd_pmac_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_pmac_playback_open),
    close: Some(snd_pmac_playback_close),
    prepare: Some(snd_pmac_playback_prepare),
    trigger: Some(snd_pmac_playback_trigger),
    pointer: Some(snd_pmac_playback_pointer),
};

static snd_pmac_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_pmac_capture_open),
    close: Some(snd_pmac_capture_close),
    prepare: Some(snd_pmac_capture_prepare),
    trigger: Some(snd_pmac_capture_trigger),
    pointer: Some(snd_pmac_capture_pointer),
};

#[no_mangle]
pub unsafe extern "C" fn snd_pmac_pcm_new(chip: *mut snd_pmac) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let mut err: c_int;
    let mut num_captures = 1;

    if (*chip).can_capture == 0 {
        num_captures = 0;
    }
    err = snd_pcm_new((*chip).card, (*(*chip).card).driver, 0, 1, num_captures, &mut pcm);
    if err < 0 {
        return err;
    }

    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_pmac_playback_ops);
    if (*chip).can_capture != 0 {
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_pmac_capture_ops);
    }

    (*pcm).private_data = chip as *mut c_void;
    (*pcm).info_flags = SNDRV_PCM_INFO_JOINT_DUPLEX;
    strscpy((*pcm).name, (*(*chip).card).shortname);
    (*chip).pcm = pcm;

    (*chip).formats_ok = SNDRV_PCM_FMTBIT_S16_BE;
    if (*chip).can_byte_swap != 0 {
        (*chip).formats_ok |= SNDRV_PCM_FMTBIT_S16_LE;
    }

    (*chip).playback.cur_formats = (*chip).formats_ok;
    (*chip).capture.cur_formats = (*chip).formats_ok;
    (*chip).playback.cur_freqs = (*chip).freqs_ok;
    (*chip).capture.cur_freqs = (*chip).freqs_ok;

    /* preallocate 64k buffer */
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, &mut (*(*chip).pdev).dev, 64 * 1024, 64 * 1024);

    0
}

unsafe fn snd_pmac_dbdma_reset(chip: *mut snd_pmac) {
    out_le32(&mut (*(*chip).playback.dma).control, (RUN | PAUSE | FLUSH | WAKE | DEAD) << 16);
    snd_pmac_wait_ack(&mut (*chip).playback);
    out_le32(&mut (*(*chip).capture.dma).control, (RUN | PAUSE | FLUSH | WAKE | DEAD) << 16);
    snd_pmac_wait_ack(&mut (*chip).capture);
}

/*
 * handling beep
 */
#[no_mangle]
pub unsafe extern "C" fn snd_pmac_beep_dma_start(chip: *mut snd_pmac, bytes: c_int, addr: c_ulong, speed: c_int) {
    let rec = &mut (*chip).playback as *mut pmac_stream;

    snd_pmac_dma_stop(rec);
    (*(*chip).extra_dma.cmds).req_count = cpu_to_le16(bytes as c_uint);
    (*(*chip).extra_dma.cmds).xfer_status = cpu_to_le16(0);
    (*(*chip).extra_dma.cmds).cmd_dep = cpu_to_le32((*chip).extra_dma.addr);
    (*(*chip).extra_dma.cmds).phy_addr = cpu_to_le32(addr);
    (*(*chip).extra_dma.cmds).command = cpu_to_le16(OUTPUT_MORE | BR_ALWAYS);
    out_le32(
        &mut (*(*chip).awacs).control,
        (in_le32(&(*(*chip).awacs).control) & !0x1f00) | ((speed as c_uint) << 8),
    );
    out_le32(&mut (*(*chip).awacs).byteswap, 0);
    snd_pmac_dma_set_command(rec, &mut (*chip).extra_dma);
    snd_pmac_dma_run(rec, RUN as c_int);
}

#[no_mangle]
pub unsafe extern "C" fn snd_pmac_beep_dma_stop(chip: *mut snd_pmac) {
    snd_pmac_dma_stop(&mut (*chip).playback);
    (*(*chip).extra_dma.cmds).command = cpu_to_le16(DBDMA_STOP);
    snd_pmac_pcm_set_format(chip); /* reset format */
}

/*
 * interrupt handlers
 */
unsafe extern "C" fn snd_pmac_tx_intr(_irq: c_int, devid: *mut c_void) -> irqreturn_t {
    let chip = devid as *mut snd_pmac;
    snd_pmac_pcm_update(chip, &mut (*chip).playback);
    IRQ_HANDLED
}

unsafe extern "C" fn snd_pmac_rx_intr(_irq: c_int, devid: *mut c_void) -> irqreturn_t {
    let chip = devid as *mut snd_pmac;
    snd_pmac_pcm_update(chip, &mut (*chip).capture);
    IRQ_HANDLED
}

unsafe extern "C" fn snd_pmac_ctrl_intr(_irq: c_int, devid: *mut c_void) -> irqreturn_t {
    let chip = devid as *mut snd_pmac;
    let ctrl = in_le32(&(*(*chip).awacs).control);

    if (ctrl & MASK_PORTCHG) != 0 {
        /* do something when headphone is plugged/unplugged? */
        if let Some(update_automute) = (*chip).update_automute {
            update_automute(chip, 1);
        }
    }
    if (ctrl & MASK_CNTLERR) != 0 {
        let err = (in_le32(&(*(*chip).awacs).codec_stat) & MASK_ERRCODE) >> 16;
        if err != 0 && (*chip).model <= PMAC_SCREAMER {
            dev_dbg((*(*chip).card).dev, b"%s: error %x\n\0".as_ptr() as *const c_char, b"snd_pmac_ctrl_intr\0".as_ptr() as *const c_char, err);
        }
    }
    /* Writing 1s to the CNTLERR and PORTCHG bits clears them... */
    out_le32(&mut (*(*chip).awacs).control, ctrl);
    IRQ_HANDLED
}

/*
 * a wrapper to feature call for compatibility
 */
unsafe fn snd_pmac_sound_feature(chip: *mut snd_pmac, enable: c_int) {
    if let Some(feature_call) = ppc_md.feature_call {
        feature_call(PMAC_FTR_SOUND_CHIP_ENABLE, (*chip).node, 0, enable);
    }
}

/*
 * release resources
 */

unsafe fn snd_pmac_free(chip: *mut snd_pmac) -> c_int {
    /* stop sounds */
    if (*chip).initialized != 0 {
        snd_pmac_dbdma_reset(chip);
        /* disable interrupts from awacs interface */
        out_le32(&mut (*(*chip).awacs).control, in_le32(&(*(*chip).awacs).control) & 0xfff);
    }

    if !(*chip).node.is_null() {
        snd_pmac_sound_feature(chip, 0);
    }

    /* clean up mixer if any */
    if let Some(mixer_free) = (*chip).mixer_free {
        mixer_free(chip);
    }

    snd_pmac_detach_beep(chip);

    /* release resources */
    if (*chip).irq >= 0 {
        free_irq((*chip).irq as c_uint, chip as *mut c_void);
    }
    if (*chip).tx_irq >= 0 {
        free_irq((*chip).tx_irq as c_uint, chip as *mut c_void);
    }
    if (*chip).rx_irq >= 0 {
        free_irq((*chip).rx_irq as c_uint, chip as *mut c_void);
    }
    snd_pmac_dbdma_free(chip, &mut (*chip).playback.cmd);
    snd_pmac_dbdma_free(chip, &mut (*chip).capture.cmd);
    snd_pmac_dbdma_free(chip, &mut (*chip).extra_dma);
    snd_pmac_dbdma_free(chip, &mut emergency_dbdma);
    iounmap((*chip).macio_base as *mut c_void);
    iounmap((*chip).latch_base as *mut c_void);
    iounmap((*chip).awacs as *mut c_void);
    iounmap((*chip).playback.dma as *mut c_void);
    iounmap((*chip).capture.dma as *mut c_void);

    if !(*chip).node.is_null() {
        let mut i = 0;
        while i < 3 {
            if ((*chip).requested & (1 << i)) != 0 {
                release_mem_region((*chip).rsrc[i as usize].start, resource_size(&(*chip).rsrc[i as usize]));
            }
            i += 1;
        }
    }

    pci_dev_put((*chip).pdev);
    of_node_put((*chip).node);
    kfree(chip as *mut c_void);
    0
}

unsafe extern "C" {
    fn snd_pmac_detach_beep(chip: *mut snd_pmac);
}

/*
 * free the device
 */
unsafe extern "C" fn snd_pmac_dev_free(device: *mut snd_device) -> c_int {
    let chip = (*device).device_data as *mut snd_pmac;
    snd_pmac_free(chip)
}

/*
 * check the machine support byteswap (little-endian)
 */

unsafe fn detect_byte_swap(chip: *mut snd_pmac) {
    let mut mio: *mut device_node;

    /* if seems that Keylargo can't byte-swap  */
    mio = (*(*chip).node).parent;
    while !mio.is_null() {
        if of_node_name_eq(mio, b"mac-io\0".as_ptr() as *const c_char) {
            if of_device_is_compatible(mio, b"Keylargo\0".as_ptr() as *const c_char) {
                (*chip).can_byte_swap = 0;
            }
            break;
        }
        mio = (*mio).parent;
    }

    /* it seems the Pismo & iBook can't byte-swap in hardware. */
    if of_machine_is_compatible(b"PowerBook3,1\0".as_ptr() as *const c_char)
        || of_machine_is_compatible(b"PowerBook2,1\0".as_ptr() as *const c_char)
    {
        (*chip).can_byte_swap = 0;
    }

    if of_machine_is_compatible(b"PowerBook2,1\0".as_ptr() as *const c_char) {
        (*chip).can_duplex = 0;
    }
}

/*
 * detect a sound chip
 */
unsafe fn snd_pmac_detect(chip: *mut snd_pmac) -> c_int {
    let mut sound: *mut device_node;
    let mut dn: *mut device_node;
    let mut prop: *const c_uint;
    let mut l: c_uint = 0;
    let mut macio: *mut macio_chip;

    if !machine_is_powermac() {
        return -ENODEV;
    }

    (*chip).subframe = 0;
    (*chip).revision = 0;
    (*chip).freqs_ok = 0xff; /* all ok */
    (*chip).model = PMAC_AWACS;
    (*chip).can_byte_swap = 1;
    (*chip).can_duplex = 1;
    (*chip).can_capture = 1;
    (*chip).num_freqs = awacs_freqs.len() as c_int;
    (*chip).freq_table = awacs_freqs.as_ptr();
    (*chip).pdev = ptr::null_mut();

    (*chip).control_mask = MASK_IEPC | MASK_IEE | 0x11; /* default */

    /* check machine type */
    if of_machine_is_compatible(b"AAPL,3400/2400\0".as_ptr() as *const c_char)
        || of_machine_is_compatible(b"AAPL,3500\0".as_ptr() as *const c_char)
    {
        (*chip).is_pbook_3400 = 1;
    } else if of_machine_is_compatible(b"PowerBook1,1\0".as_ptr() as *const c_char)
        || of_machine_is_compatible(b"AAPL,PowerBook1998\0".as_ptr() as *const c_char)
    {
        (*chip).is_pbook_G3 = 1;
    }
    (*chip).node = of_find_node_by_name(ptr::null_mut(), b"awacs\0".as_ptr() as *const c_char);
    sound = of_node_get((*chip).node);

    /*
     * powermac G3 models have a node called "davbus"
     * with a child called "sound".
     */
    if (*chip).node.is_null() {
        (*chip).node = of_find_node_by_name(ptr::null_mut(), b"davbus\0".as_ptr() as *const c_char);
    }
    /*
     * if we didn't find a davbus device, try 'i2s-a' since
     * this seems to be what iBooks have
     */
    if (*chip).node.is_null() {
        (*chip).node = of_find_node_by_name(ptr::null_mut(), b"i2s-a\0".as_ptr() as *const c_char);
        if !(*chip).node.is_null() && !(*(*chip).node).parent.is_null() && !(*(*(*chip).node).parent).parent.is_null() {
            if of_device_is_compatible((*(*(*chip).node).parent).parent, b"K2-Keylargo\0".as_ptr() as *const c_char) {
                (*chip).is_k2 = 1;
            }
        }
    }
    if (*chip).node.is_null() {
        return -ENODEV;
    }

    if sound.is_null() {
        sound = of_find_node_by_name(ptr::null_mut(), b"sound\0".as_ptr() as *const c_char);
        while !sound.is_null() {
            if (*sound).parent == (*chip).node {
                break;
            }
            sound = of_find_node_by_name(sound, b"sound\0".as_ptr() as *const c_char);
        }
    }
    if sound.is_null() {
        of_node_put((*chip).node);
        (*chip).node = ptr::null_mut();
        return -ENODEV;
    }
    prop = of_get_property(sound, b"sub-frame\0".as_ptr() as *const c_char, ptr::null_mut());
    if !prop.is_null() && *prop < 16 {
        (*chip).subframe = *prop as c_int;
    }
    prop = of_get_property(sound, b"layout-id\0".as_ptr() as *const c_char, ptr::null_mut());
    if !prop.is_null() {
        /* partly deprecate snd-powermac, for those machines
         * that have a layout-id property for now */
        dev_info(
            (*(*chip).card).dev,
            b"snd-powermac no longer handles any machines with a layout-id property in the device-tree, use snd-aoa.\n\0".as_ptr() as *const c_char,
        );
        of_node_put(sound);
        of_node_put((*chip).node);
        (*chip).node = ptr::null_mut();
        return -ENODEV;
    }
    /* This should be verified on older screamers */
    if of_device_is_compatible(sound, b"screamer\0".as_ptr() as *const c_char) {
        (*chip).model = PMAC_SCREAMER;
        // chip->can_byte_swap = 0; /* FIXME: check this */
    }
    if of_device_is_compatible(sound, b"burgundy\0".as_ptr() as *const c_char) {
        (*chip).model = PMAC_BURGUNDY;
        (*chip).control_mask = MASK_IEPC | 0x11; /* disable IEE */
    }
    if of_device_is_compatible(sound, b"daca\0".as_ptr() as *const c_char) {
        (*chip).model = PMAC_DACA;
        (*chip).can_capture = 0; /* no capture */
        (*chip).can_duplex = 0;
        // chip->can_byte_swap = 0; /* FIXME: check this */
        (*chip).control_mask = MASK_IEPC | 0x11; /* disable IEE */
    }
    if of_device_is_compatible(sound, b"tumbler\0".as_ptr() as *const c_char) {
        (*chip).model = PMAC_TUMBLER;
        (*chip).can_capture = (of_machine_is_compatible(b"PowerMac4,2\0".as_ptr() as *const c_char)
            || of_machine_is_compatible(b"PowerBook3,2\0".as_ptr() as *const c_char)
            || of_machine_is_compatible(b"PowerBook3,3\0".as_ptr() as *const c_char)
            || of_machine_is_compatible(b"PowerBook4,1\0".as_ptr() as *const c_char)
            || of_machine_is_compatible(b"PowerBook4,2\0".as_ptr() as *const c_char)
            || of_machine_is_compatible(b"PowerBook4,3\0".as_ptr() as *const c_char)) as c_int;
        (*chip).can_duplex = 0;
        // chip->can_byte_swap = 0; /* FIXME: check this */
        (*chip).num_freqs = tumbler_freqs.len() as c_int;
        (*chip).freq_table = tumbler_freqs.as_ptr();
        (*chip).control_mask = MASK_IEPC | 0x11; /* disable IEE */
    }
    if of_device_is_compatible(sound, b"snapper\0".as_ptr() as *const c_char) {
        (*chip).model = PMAC_SNAPPER;
        // chip->can_byte_swap = 0; /* FIXME: check this */
        (*chip).num_freqs = tumbler_freqs.len() as c_int;
        (*chip).freq_table = tumbler_freqs.as_ptr();
        (*chip).control_mask = MASK_IEPC | 0x11; /* disable IEE */
    }
    prop = of_get_property(sound, b"device-id\0".as_ptr() as *const c_char, ptr::null_mut());
    if !prop.is_null() {
        (*chip).device_id = *prop;
    }
    dn = of_find_node_by_name(ptr::null_mut(), b"perch\0".as_ptr() as *const c_char);
    (*chip).has_iic = !dn.is_null();
    of_node_put(dn);

    /* We need the PCI device for DMA allocations, let's use a crude method
     * for now ...
     */
    macio = macio_find((*chip).node, macio_unknown);
    if macio.is_null() {
        dev_warn((*(*chip).card).dev, b"snd-powermac: can't locate macio !\n\0".as_ptr() as *const c_char);
    } else {
        let mut pdev: *mut pci_dev = ptr::null_mut();

        loop {
            pdev = for_each_pci_dev_next(pdev);
            if pdev.is_null() {
                break;
            }
            let np = pci_device_to_OF_node(pdev);
            if !np.is_null() && np == (*macio).of_node {
                (*chip).pdev = pdev;
                break;
            }
        }
    }
    if (*chip).pdev.is_null() {
        dev_warn((*(*chip).card).dev, b"snd-powermac: can't locate macio PCI device !\n\0".as_ptr() as *const c_char);
    }

    detect_byte_swap(chip);

    /* look for a property saying what sample rates
       are available */
    prop = of_get_property(sound, b"sample-rates\0".as_ptr() as *const c_char, &mut l);
    if prop.is_null() {
        prop = of_get_property(sound, b"output-frame-rates\0".as_ptr() as *const c_char, &mut l);
    }
    if !prop.is_null() {
        let mut i: c_int;
        (*chip).freqs_ok = 0;
        l /= size_of::<c_int>() as c_uint;
        while l > 0 {
            let mut r = *prop;
            prop = prop.add(1);
            /* Apple 'Fixed' format */
            if r >= 0x10000 {
                r >>= 16;
            }
            i = 0;
            while i < (*chip).num_freqs {
                if r == *(*chip).freq_table.offset(i as isize) as c_uint {
                    (*chip).freqs_ok |= 1 << i;
                    break;
                }
                i += 1;
            }
            l -= 1;
        }
    } else {
        /* assume only 44.1khz */
        (*chip).freqs_ok = 1;
    }

    of_node_put(sound);
    0
}

/* #ifdef PMAC_SUPPORT_AUTOMUTE */
/*
 * auto-mute
 */
unsafe extern "C" fn pmac_auto_mute_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    (*ucontrol).value.integer.value[0] = (*chip).auto_mute as c_long;
    0
}

unsafe extern "C" fn pmac_auto_mute_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    if (*ucontrol).value.integer.value[0] != (*chip).auto_mute as c_long {
        (*chip).auto_mute = ((*ucontrol).value.integer.value[0] != 0) as c_int;
        if let Some(update_automute) = (*chip).update_automute {
            update_automute(chip, 1);
        }
        return 1;
    }
    0
}

unsafe extern "C" fn pmac_hp_detect_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    if let Some(detect_headphone) = (*chip).detect_headphone {
        (*ucontrol).value.integer.value[0] = detect_headphone(chip) as c_long;
    } else {
        (*ucontrol).value.integer.value[0] = 0;
    }
    0
}

static auto_mute_controls: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Auto Mute Switch\0".as_ptr() as *const c_char,
        access: 0,
        info: Some(snd_pmac_boolean_mono_info),
        get: Some(pmac_auto_mute_get),
        put: Some(pmac_auto_mute_put),
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Headphone Detection\0".as_ptr() as *const c_char,
        access: SNDRV_CTL_ELEM_ACCESS_READ,
        info: Some(snd_pmac_boolean_mono_info),
        get: Some(pmac_hp_detect_get),
        put: None,
    },
];

#[no_mangle]
pub unsafe extern "C" fn snd_pmac_add_automute(chip: *mut snd_pmac) -> c_int {
    let mut err: c_int;
    (*chip).auto_mute = 1;
    err = snd_ctl_add((*chip).card, snd_ctl_new1(&auto_mute_controls[0], chip as *mut c_void));
    if err < 0 {
        dev_err((*(*chip).card).dev, b"snd-powermac: Failed to add automute control\n\0".as_ptr() as *const c_char);
        return err;
    }
    (*chip).hp_detect_ctl = snd_ctl_new1(&auto_mute_controls[1], chip as *mut c_void);
    snd_ctl_add((*chip).card, (*chip).hp_detect_ctl)
}
/* #endif PMAC_SUPPORT_AUTOMUTE */

/*
 * create and detect a pmac chip record
 */
#[no_mangle]
pub unsafe extern "C" fn snd_pmac_new(card: *mut snd_card, chip_return: *mut *mut snd_pmac) -> c_int {
    let mut chip: *mut snd_pmac;
    let np: *mut device_node;
    let mut i: c_int;
    let mut err: c_int;
    let mut irq: c_uint;
    let ctrl_addr: c_ulong;
    let txdma_addr: c_ulong;
    let rxdma_addr: c_ulong;
    static ops: snd_device_ops = snd_device_ops {
        dev_free: Some(snd_pmac_dev_free),
    };

    *chip_return = ptr::null_mut();

    chip = kzalloc(size_of::<snd_pmac>(), GFP_KERNEL) as *mut snd_pmac;
    if chip.is_null() {
        return -ENOMEM;
    }
    (*chip).card = card;

    (*chip).irq = -1;
    (*chip).tx_irq = -1;
    (*chip).rx_irq = -1;

    (*chip).playback.stream = SNDRV_PCM_STREAM_PLAYBACK;
    (*chip).capture.stream = SNDRV_PCM_STREAM_CAPTURE;

    err = snd_pmac_detect(chip);
    if err < 0 {
        snd_pmac_free(chip);
        return err;
    }

    if snd_pmac_dbdma_alloc(chip, &mut (*chip).playback.cmd, PMAC_MAX_FRAGS as c_int + 1) < 0
        || snd_pmac_dbdma_alloc(chip, &mut (*chip).capture.cmd, PMAC_MAX_FRAGS as c_int + 1) < 0
        || snd_pmac_dbdma_alloc(chip, &mut (*chip).extra_dma, 2) < 0
        || snd_pmac_dbdma_alloc(chip, &mut emergency_dbdma, 2) < 0
    {
        err = -ENOMEM;
        snd_pmac_free(chip);
        return err;
    }

    np = (*chip).node;
    (*chip).requested = 0;
    if (*chip).is_k2 != 0 {
        static rnames: [*const c_char; 2] = [
            b"Sound Control\0".as_ptr() as *const c_char,
            b"Sound DMA\0".as_ptr() as *const c_char,
        ];
        i = 0;
        while i < 2 {
            if of_address_to_resource((*np).parent, i, &mut (*chip).rsrc[i as usize]) != 0 {
                dev_err((*(*chip).card).dev, b"snd: can't translate rsrc %d (%s)\n\0".as_ptr() as *const c_char, i, rnames[i as usize]);
                err = -ENODEV;
                snd_pmac_free(chip);
                return err;
            }
            if request_mem_region((*chip).rsrc[i as usize].start, resource_size(&(*chip).rsrc[i as usize]), rnames[i as usize]).is_null() {
                dev_err((*(*chip).card).dev, b"snd: can't request rsrc %d (%s: %pR)\n\0".as_ptr() as *const c_char, i, rnames[i as usize], &mut (*chip).rsrc[i as usize]);
                err = -ENODEV;
                snd_pmac_free(chip);
                return err;
            }
            (*chip).requested |= 1 << i;
            i += 1;
        }
        ctrl_addr = (*chip).rsrc[0].start;
        txdma_addr = (*chip).rsrc[1].start;
        rxdma_addr = txdma_addr + 0x100;
    } else {
        static rnames: [*const c_char; 3] = [
            b"Sound Control\0".as_ptr() as *const c_char,
            b"Sound Tx DMA\0".as_ptr() as *const c_char,
            b"Sound Rx DMA\0".as_ptr() as *const c_char,
        ];
        i = 0;
        while i < 3 {
            if of_address_to_resource(np, i, &mut (*chip).rsrc[i as usize]) != 0 {
                dev_err((*(*chip).card).dev, b"snd: can't translate rsrc %d (%s)\n\0".as_ptr() as *const c_char, i, rnames[i as usize]);
                err = -ENODEV;
                snd_pmac_free(chip);
                return err;
            }
            if request_mem_region((*chip).rsrc[i as usize].start, resource_size(&(*chip).rsrc[i as usize]), rnames[i as usize]).is_null() {
                dev_err((*(*chip).card).dev, b"snd: can't request rsrc %d (%s: %pR)\n\0".as_ptr() as *const c_char, i, rnames[i as usize], &mut (*chip).rsrc[i as usize]);
                err = -ENODEV;
                snd_pmac_free(chip);
                return err;
            }
            (*chip).requested |= 1 << i;
            i += 1;
        }
        ctrl_addr = (*chip).rsrc[0].start;
        txdma_addr = (*chip).rsrc[1].start;
        rxdma_addr = (*chip).rsrc[2].start;
    }

    (*chip).awacs = ioremap(ctrl_addr, 0x1000) as *mut awacs_regs;
    (*chip).playback.dma = ioremap(txdma_addr, 0x100) as *mut dbdma_regs;
    (*chip).capture.dma = ioremap(rxdma_addr, 0x100) as *mut dbdma_regs;
    if (*chip).model <= PMAC_BURGUNDY {
        irq = irq_of_parse_and_map(np, 0);
        if request_irq(irq, snd_pmac_ctrl_intr, 0, b"PMac\0".as_ptr() as *const c_char, chip as *mut c_void) != 0 {
            dev_err((*(*chip).card).dev, b"pmac: unable to grab IRQ %d\n\0".as_ptr() as *const c_char, irq);
            err = -EBUSY;
            snd_pmac_free(chip);
            return err;
        }
        (*chip).irq = irq as c_int;
    }
    irq = irq_of_parse_and_map(np, 1);
    if request_irq(irq, snd_pmac_tx_intr, 0, b"PMac Output\0".as_ptr() as *const c_char, chip as *mut c_void) != 0 {
        dev_err((*(*chip).card).dev, b"pmac: unable to grab IRQ %d\n\0".as_ptr() as *const c_char, irq);
        err = -EBUSY;
        snd_pmac_free(chip);
        return err;
    }
    (*chip).tx_irq = irq as c_int;
    irq = irq_of_parse_and_map(np, 2);
    if request_irq(irq, snd_pmac_rx_intr, 0, b"PMac Input\0".as_ptr() as *const c_char, chip as *mut c_void) != 0 {
        dev_err((*(*chip).card).dev, b"pmac: unable to grab IRQ %d\n\0".as_ptr() as *const c_char, irq);
        err = -EBUSY;
        snd_pmac_free(chip);
        return err;
    }
    (*chip).rx_irq = irq as c_int;

    snd_pmac_sound_feature(chip, 1);

    /* reset & enable interrupts */
    if (*chip).model <= PMAC_BURGUNDY {
        out_le32(&mut (*(*chip).awacs).control, (*chip).control_mask);
    }

    /* Powerbooks have odd ways of enabling inputs such as
       an expansion-bay CD or sound from an internal modem
       or a PC-card modem. */
    if (*chip).is_pbook_3400 != 0 {
        /* Enable CD and PC-card sound inputs. */
        /* This is done by reading from address
         * f301a000, + 0x10 to enable the expansion-bay
         * CD sound input, + 0x80 to enable the PC-card
         * sound input.  The 0x100 enables the SCSI bus
         * terminator power.
         */
        (*chip).latch_base = ioremap(0xf301a000, 0x1000) as *mut u8;
        in_8((*chip).latch_base.add(0x190));
    } else if (*chip).is_pbook_G3 != 0 {
        let mut mio: *mut device_node;
        mio = (*(*chip).node).parent;
        while !mio.is_null() {
            if of_node_name_eq(mio, b"mac-io\0".as_ptr() as *const c_char) {
                let mut r = resource { start: 0 };
                if of_address_to_resource(mio, 0, &mut r) == 0 {
                    (*chip).macio_base = ioremap(r.start, 0x40) as *mut u8;
                }
                break;
            }
            mio = (*mio).parent;
        }
        /* Enable CD sound input. */
        /* The relevant bits for writing to this byte are 0x8f.
         * I haven't found out what the 0x80 bit does.
         * For the 0xf bits, writing 3 or 7 enables the CD
         * input, any other value disables it.  Values
         * 1, 3, 5, 7 enable the microphone.  Values 0, 2,
         * 4, 6, 8 - f enable the input from the modem.
         */
        if !(*chip).macio_base.is_null() {
            out_8((*chip).macio_base.add(0x37), 3);
        }
    }

    /* Reset dbdma channels */
    snd_pmac_dbdma_reset(chip);

    err = snd_device_new(card, SNDRV_DEV_LOWLEVEL, chip as *mut c_void, &ops);
    if err < 0 {
        snd_pmac_free(chip);
        return err;
    }

    *chip_return = chip;
    0
}

unsafe extern "C" {
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
}

/*
 * sleep notify for powerbook
 */

/* #ifdef CONFIG_PM */

/*
 * Save state when going to sleep, restore it afterwards.
 */

#[no_mangle]
pub unsafe extern "C" fn snd_pmac_suspend(chip: *mut snd_pmac) {
    snd_power_change_state((*chip).card, SNDRV_CTL_POWER_D3hot);
    if let Some(suspend) = (*chip).suspend {
        suspend(chip);
    }
    snd_pmac_beep_stop(chip);
    if (*chip).irq >= 0 {
        disable_irq((*chip).irq as c_uint);
    }
    if (*chip).tx_irq >= 0 {
        disable_irq((*chip).tx_irq as c_uint);
    }
    if (*chip).rx_irq >= 0 {
        disable_irq((*chip).rx_irq as c_uint);
    }
    snd_pmac_sound_feature(chip, 0);
}

#[no_mangle]
pub unsafe extern "C" fn snd_pmac_resume(chip: *mut snd_pmac) {
    snd_pmac_sound_feature(chip, 1);
    if let Some(resume) = (*chip).resume {
        resume(chip);
    }
    /* enable CD sound input */
    if !(*chip).macio_base.is_null() && (*chip).is_pbook_G3 != 0 {
        out_8((*chip).macio_base.add(0x37), 3);
    } else if (*chip).is_pbook_3400 != 0 {
        in_8((*chip).latch_base.add(0x190));
    }

    snd_pmac_pcm_set_format(chip);

    if (*chip).irq >= 0 {
        enable_irq((*chip).irq as c_uint);
    }
    if (*chip).tx_irq >= 0 {
        enable_irq((*chip).tx_irq as c_uint);
    }
    if (*chip).rx_irq >= 0 {
        enable_irq((*chip).rx_irq as c_uint);
    }

    snd_power_change_state((*chip).card, SNDRV_CTL_POWER_D0);
}

/* #endif CONFIG_PM */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
