// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Driver for A2 audio system used in SGI machines
 *  Copyright (c) 2008 Thomas Bogendoerfer <tsbogend@alpha.fanken.de>
 *
 *  Based on OSS code from Ladislav Michl <ladis@linux-mips.org>, which
 *  was based on code from Ulf Carlsson
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type u16 = u16;
type u32 = u32;
type size_t = usize;
type dma_addr_t = usize;
type snd_pcm_uframes_t = usize;
type irqreturn_t = c_uint;
type dma_data_direction = c_uint;

const SNDRV_DEFAULT_IDX1: c_int = 0;
const SNDRV_DEFAULT_STR1: *mut c_char = ptr::null_mut();

const H2_BLOCK_SIZE: c_int = 1024;
const H2_BUF_SIZE: c_int = 16384;

const H2_MIX_OUTPUT_ATT: c_int = 0;
const H2_MIX_INPUT_GAIN: c_int = 1;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const EAGAIN: c_int = 11;

const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_SHARED: c_uint = 0;

const GFP_KERNEL: c_uint = 0;
const DMA_BIDIRECTIONAL: dma_data_direction = 0;
const DMA_TO_DEVICE: dma_data_direction = 1;
const DMA_FROM_DEVICE: dma_data_direction = 2;

const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 2;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 0x3;
const SNDRV_PCM_INFO_MMAP: c_uint = 0x1;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 0x2;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 0x4;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint = 0x10000;
const SNDRV_PCM_INFO_SYNC_APPLPTR: c_uint = 0x20000000;
const SNDRV_PCM_FMTBIT_S16_BE: u64 = 1 << 3;
const SNDRV_PCM_RATE_8000_48000: c_uint = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 1;
const SNDRV_PCM_TRIGGER_STOP: c_int = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_DMA_TYPE_CONTINUOUS: c_int = 0;
const SNDRV_DEV_LOWLEVEL: c_int = 0;

const KERN_INFO: *const c_char = b"\0".as_ptr() as *const c_char;
const KERN_ERR: *const c_char = b"\0".as_ptr() as *const c_char;
const THIS_MODULE: *mut c_void = ptr::null_mut();

const HPCDMA_XIE: u32 = 0;
const HPC3_PDMACTRL_INT: u32 = 0;
const HPC3_PDMACTRL_RT: u32 = 0;
const HPC3_PDMACTRL_LD: u32 = 0;
const HPC3_PDMACTRL_RCV: u32 = 0;
const HPC3_PDMACTRL_ACT: u32 = 0;
const HPC3_DMACFG_D3R_SHIFT: c_int = 0;
const HPC3_DMACFG_D4R_SHIFT: c_int = 0;
const HPC3_DMACFG_D5R_SHIFT: c_int = 0;
const HPC3_DMACFG_D3W_SHIFT: c_int = 0;
const HPC3_DMACFG_D4W_SHIFT: c_int = 0;
const HPC3_DMACFG_D5W_SHIFT: c_int = 0;
const HPC3_DMACFG_DS16: u32 = 0;
const HPC3_DMACFG_EVENHI: u32 = 0;
const HPC3_DMACFG_RTIME: u32 = 0;
const HPC3_DMACFG_BURST_SHIFT: c_int = 0;
const HPC3_DMACFG_DRQLIVE: u32 = 0;
const SGI_HPCDMA_IRQ: c_int = 0;

const H2_ISR_TSTATUS: u32 = 0;
const H2_ISR_GLOBAL_RESET_N: u32 = 0;
const H2_ISR_CODEC_RESET_N: u32 = 0;
const H2_REV_AUDIO_PRESENT: u32 = 0;
const H2_REV_BOARD_M: u32 = 0;
const H2_REV_MAJOR_CHIP_M: u32 = 0;
const H2_REV_MINOR_CHIP_M: u32 = 0;
const H2I_DAC_C2: u16 = 0;
const H2I_ADC_C2: u16 = 0;
const H2I_C2_MUTE: u32 = 0;
const H2I_C2_L_ATT_SHIFT: c_int = 0;
const H2I_C2_R_ATT_SHIFT: c_int = 0;
const H2I_C2_L_GAIN_SHIFT: c_int = 0;
const H2I_C2_R_GAIN_SHIFT: c_int = 0;
const H2I_C2_L_ATT_M: u32 = 0;
const H2I_C2_R_ATT_M: u32 = 0;
const H2I_C2_L_GAIN_M: u32 = 0;
const H2I_C2_R_GAIN_M: u32 = 0;
const H2I_BRES1_C1: u16 = 0;
const H2I_BRES1_C2: u16 = 0;
const H2I_BRES2_C1: u16 = 0;
const H2I_BRES2_C2: u16 = 0;
const H2I_DMA_PORT_EN: u16 = 0;
const H2I_DMA_PORT_EN_CODECTX: u16 = 0;
const H2I_DMA_PORT_EN_CODECR: u16 = 0;
const H2I_DMA_END: u16 = 0;
const H2I_DMA_END_CODECTX: u16 = 0;
const H2I_DMA_END_CODECR: u16 = 0;
const H2I_DMA_DRV: u16 = 0;
const H2I_DAC_C1: u16 = 0;
const H2I_ADC_C1: u16 = 0;
const H2I_C1_DMA_SHIFT: c_int = 0;
const H2I_C1_CLKID_SHIFT: c_int = 0;
const H2I_C1_DATAT_SHIFT: c_int = 0;
const H2I_RELAY_C: u16 = 0;
const H2I_RELAY_C_STATE: u16 = 0;

static mut index: c_int = SNDRV_DEFAULT_IDX1; /* Index 0-MAX */
static mut id: *mut c_char = SNDRV_DEFAULT_STR1; /* ID for this card */

// module_param(index, int, 0444);
// MODULE_PARM_DESC(index, "Index value for SGI HAL2 soundcard.");
// module_param(id, charp, 0444);
// MODULE_PARM_DESC(id, "ID string for SGI HAL2 soundcard.");
// MODULE_DESCRIPTION("ALSA driver for SGI HAL2 audio");
// MODULE_AUTHOR("Thomas Bogendoerfer");
// MODULE_LICENSE("GPL");

#[repr(C)]
pub struct hpc3_pbus_dmacregs {
    pub pbdma_ctrl: u32,
    pub pbdma_dptr: dma_addr_t,
    pub pbdma_bptr: dma_addr_t,
}

#[repr(C)]
pub struct hpc_dma_desc {
    pub pbuf: dma_addr_t,
    pub cntinfo: u32,
    pub pnext: dma_addr_t,
}

#[repr(C)]
pub struct hal2_ctl_regs {
    pub isr: u32,
    pub rev: u32,
    pub iar: u32,
    pub idr0: u32,
    pub idr1: u32,
    pub idr2: u32,
    pub idr3: u32,
}

#[repr(C)]
pub struct hal2_aes_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hal2_vol_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hal2_syn_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hpc3_regs {
    pub pbdma: [hpc3_pbus_dmacregs; 2],
    pub pbus_extregs: [*mut c_void; 4],
    pub pbus_dmacfg: [[u32; 1]; 2],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub driver: [c_char; 80],
    pub shortname: [c_char; 80],
    pub longname: [c_char; 80],
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: isize,
}

#[repr(C)]
pub struct snd_ctl_elem_info_integer {
    pub min: i64,
    pub max: i64,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 2],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
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
    pub private_value: isize,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: u64,
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
    pub channels: c_uint,
    pub rate: c_uint,
    pub dma_area: *mut u8,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_indirect {
    pub hw_buffer_size: size_t,
    pub hw_queue_size: size_t,
    pub hw_io: dma_addr_t,
    pub sw_buffer_size: size_t,
    pub hw_data: size_t,
    pub sw_data: size_t,
}

#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    pub ack: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
}

#[repr(C)]
pub struct snd_pcm {
    pub private_data: *mut c_void,
    pub name: [c_char; 80],
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
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: device_driver,
}

#[repr(C)]
struct hal2_pbus {
    pbus: *mut hpc3_pbus_dmacregs,
    pbusnr: c_int,
    ctrl: c_uint, /* Current state of pbus->pbdma_ctrl */
}

#[repr(C)]
struct hal2_desc {
    desc: hpc_dma_desc,
    pad: u32, /* padding */
}

#[repr(C)]
struct hal2_codec {
    pcm_indirect: snd_pcm_indirect,
    substream: *mut snd_pcm_substream,
    buffer: *mut u8,
    buffer_dma: dma_addr_t,
    desc: *mut hal2_desc,
    desc_dma: dma_addr_t,
    desc_count: c_int,
    pbus: hal2_pbus,
    voices: c_int, /* mono/stereo */
    sample_rate: c_uint,
    master: c_uint, /* Master frequency */
    mod_: u16,      /* MOD value */
    inc: u16,       /* INC value */
}

#[repr(C)]
struct snd_hal2 {
    card: *mut snd_card,
    ctl_regs: *mut hal2_ctl_regs, /* HAL2 ctl registers */
    aes_regs: *mut hal2_aes_regs, /* HAL2 aes registers */
    vol_regs: *mut hal2_vol_regs, /* HAL2 vol registers */
    syn_regs: *mut hal2_syn_regs, /* HAL2 syn registers */
    dac: hal2_codec,
    adc: hal2_codec,
}

unsafe extern "C" {
    static mut hpc3c0: *mut hpc3_regs;

    fn __raw_readl(reg: *mut u32) -> u32;
    fn __raw_writel(val: u32, reg: *mut u32);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(template: *const snd_kcontrol_new, data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn dma_alloc_noncoherent(
        dev: *mut device,
        size: size_t,
        dma_handle: *mut dma_addr_t,
        dir: dma_data_direction,
        flag: c_uint,
    ) -> *mut c_void;
    fn dma_free_noncoherent(
        dev: *mut device,
        size: size_t,
        vaddr: *mut c_void,
        dma_handle: dma_addr_t,
        dir: dma_data_direction,
    );
    fn dma_sync_single_for_device(
        dev: *mut device,
        addr: dma_addr_t,
        size: size_t,
        dir: dma_data_direction,
    );
    fn dma_sync_single_for_cpu(
        dev: *mut device,
        addr: dma_addr_t,
        size: size_t,
        dir: dma_data_direction,
    );
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> size_t;
    fn snd_pcm_indirect_playback_pointer(
        substream: *mut snd_pcm_substream,
        rec: *mut snd_pcm_indirect,
        hw_pointer: dma_addr_t,
    ) -> snd_pcm_uframes_t;
    fn snd_pcm_indirect_playback_transfer(
        substream: *mut snd_pcm_substream,
        rec: *mut snd_pcm_indirect,
        transfer: unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_indirect, size_t),
    ) -> c_int;
    fn snd_pcm_indirect_capture_pointer(
        substream: *mut snd_pcm_substream,
        rec: *mut snd_pcm_indirect,
        hw_pointer: dma_addr_t,
    ) -> snd_pcm_uframes_t;
    fn snd_pcm_indirect_capture_transfer(
        substream: *mut snd_pcm_substream,
        rec: *mut snd_pcm_indirect,
        transfer: unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_indirect, size_t),
    ) -> c_int;
    fn snd_pcm_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        playback_count: c_int,
        capture_count: c_int,
        rpcm: *mut *mut snd_pcm,
    ) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        type_: c_int,
        data: *mut c_void,
        size: size_t,
        max: size_t,
    );
    fn free_irq(irq: c_int, dev_id: *mut c_void);
    fn kfree(ptr: *mut c_void);
    fn request_irq(
        irq: c_int,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        flags: c_uint,
        name: *const c_char,
        dev: *mut c_void,
    ) -> c_int;
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn snd_device_new(
        card: *mut snd_card,
        type_: c_int,
        device_data: *mut c_void,
        ops: *const snd_device_ops,
    ) -> c_int;
    fn snd_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut c_void,
        extra_size: c_int,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn snd_card_free(card: *mut snd_card);
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn printk(fmt: *const c_char, ...);
    fn strscpy(dst: *mut c_char, src: *const c_char) -> size_t;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...);
}

macro_rules! c_str {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

#[inline]
fn H2_READ_ADDR(addr: u16) -> u32 {
    addr as u32 | (1 << 7)
}

#[inline]
fn H2_WRITE_ADDR(addr: u16) -> u32 {
    addr as u32
}

#[inline]
unsafe fn H2_INDIRECT_WAIT(regs: *mut hal2_ctl_regs) {
    while hal2_read(ptr::addr_of_mut!((*regs).isr)) & H2_ISR_TSTATUS != 0 {}
}

#[inline]
unsafe fn hal2_read(reg: *mut u32) -> u32 {
    unsafe { __raw_readl(reg) }
}

#[inline]
unsafe fn hal2_write(val: u32, reg: *mut u32) {
    unsafe { __raw_writel(val, reg) };
}

unsafe fn hal2_i_read32(hal2: *mut snd_hal2, addr: u16) -> u32 {
    let mut ret: u32;
    let regs = unsafe { (*hal2).ctl_regs };

    unsafe { hal2_write(H2_READ_ADDR(addr), ptr::addr_of_mut!((*regs).iar)) };
    unsafe { H2_INDIRECT_WAIT(regs) };
    ret = unsafe { hal2_read(ptr::addr_of_mut!((*regs).idr0)) } & 0xffff;
    unsafe { hal2_write(H2_READ_ADDR(addr) | 0x1, ptr::addr_of_mut!((*regs).iar)) };
    unsafe { H2_INDIRECT_WAIT(regs) };
    ret |= (unsafe { hal2_read(ptr::addr_of_mut!((*regs).idr0)) } & 0xffff) << 16;
    ret
}

unsafe fn hal2_i_write16(hal2: *mut snd_hal2, addr: u16, val: u16) {
    let regs = unsafe { (*hal2).ctl_regs };

    unsafe { hal2_write(val as u32, ptr::addr_of_mut!((*regs).idr0)) };
    unsafe { hal2_write(0, ptr::addr_of_mut!((*regs).idr1)) };
    unsafe { hal2_write(0, ptr::addr_of_mut!((*regs).idr2)) };
    unsafe { hal2_write(0, ptr::addr_of_mut!((*regs).idr3)) };
    unsafe { hal2_write(H2_WRITE_ADDR(addr), ptr::addr_of_mut!((*regs).iar)) };
    unsafe { H2_INDIRECT_WAIT(regs) };
}

unsafe fn hal2_i_write32(hal2: *mut snd_hal2, addr: u16, val: u32) {
    let regs = unsafe { (*hal2).ctl_regs };

    unsafe { hal2_write(val & 0xffff, ptr::addr_of_mut!((*regs).idr0)) };
    unsafe { hal2_write(val >> 16, ptr::addr_of_mut!((*regs).idr1)) };
    unsafe { hal2_write(0, ptr::addr_of_mut!((*regs).idr2)) };
    unsafe { hal2_write(0, ptr::addr_of_mut!((*regs).idr3)) };
    unsafe { hal2_write(H2_WRITE_ADDR(addr), ptr::addr_of_mut!((*regs).iar)) };
    unsafe { H2_INDIRECT_WAIT(regs) };
}

unsafe fn hal2_i_setbit16(hal2: *mut snd_hal2, addr: u16, bit: u16) {
    let regs = unsafe { (*hal2).ctl_regs };

    unsafe { hal2_write(H2_READ_ADDR(addr), ptr::addr_of_mut!((*regs).iar)) };
    unsafe { H2_INDIRECT_WAIT(regs) };
    let idr0 = (unsafe { hal2_read(ptr::addr_of_mut!((*regs).idr0)) } & 0xffff) | bit as u32;
    unsafe { hal2_write(idr0, ptr::addr_of_mut!((*regs).idr0)) };
    unsafe { hal2_write(0, ptr::addr_of_mut!((*regs).idr1)) };
    unsafe { hal2_write(0, ptr::addr_of_mut!((*regs).idr2)) };
    unsafe { hal2_write(0, ptr::addr_of_mut!((*regs).idr3)) };
    unsafe { hal2_write(H2_WRITE_ADDR(addr), ptr::addr_of_mut!((*regs).iar)) };
    unsafe { H2_INDIRECT_WAIT(regs) };
}

unsafe fn hal2_i_clearbit16(hal2: *mut snd_hal2, addr: u16, bit: u16) {
    let regs = unsafe { (*hal2).ctl_regs };

    unsafe { hal2_write(H2_READ_ADDR(addr), ptr::addr_of_mut!((*regs).iar)) };
    unsafe { H2_INDIRECT_WAIT(regs) };
    let idr0 = (unsafe { hal2_read(ptr::addr_of_mut!((*regs).idr0)) } & 0xffff) & !(bit as u32);
    unsafe { hal2_write(idr0, ptr::addr_of_mut!((*regs).idr0)) };
    unsafe { hal2_write(0, ptr::addr_of_mut!((*regs).idr1)) };
    unsafe { hal2_write(0, ptr::addr_of_mut!((*regs).idr2)) };
    unsafe { hal2_write(0, ptr::addr_of_mut!((*regs).idr3)) };
    unsafe { hal2_write(H2_WRITE_ADDR(addr), ptr::addr_of_mut!((*regs).iar)) };
    unsafe { H2_INDIRECT_WAIT(regs) };
}

unsafe extern "C" fn hal2_gain_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).count = 2;
        (*uinfo).value.integer.min = 0;
        match (*kcontrol).private_value as c_int {
            H2_MIX_OUTPUT_ATT => (*uinfo).value.integer.max = 31,
            H2_MIX_INPUT_GAIN => (*uinfo).value.integer.max = 15,
            _ => {}
        }
    }
    0
}

unsafe extern "C" fn hal2_gain_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let hal2 = unsafe { snd_kcontrol_chip(kcontrol) as *mut snd_hal2 };
    let tmp: u32;
    let l: c_int;
    let r: c_int;

    unsafe {
        match (*kcontrol).private_value as c_int {
            H2_MIX_OUTPUT_ATT => {
                tmp = hal2_i_read32(hal2, H2I_DAC_C2);
                if tmp & H2I_C2_MUTE != 0 {
                    l = 0;
                    r = 0;
                } else {
                    l = (31 - ((tmp >> H2I_C2_L_ATT_SHIFT) & 31)) as c_int;
                    r = (31 - ((tmp >> H2I_C2_R_ATT_SHIFT) & 31)) as c_int;
                }
            }
            H2_MIX_INPUT_GAIN => {
                tmp = hal2_i_read32(hal2, H2I_ADC_C2);
                l = ((tmp >> H2I_C2_L_GAIN_SHIFT) & 15) as c_int;
                r = ((tmp >> H2I_C2_R_GAIN_SHIFT) & 15) as c_int;
            }
            _ => return -EINVAL,
        }
        (*ucontrol).value.integer.value[0] = l as i64;
        (*ucontrol).value.integer.value[1] = r as i64;
    }

    0
}

unsafe extern "C" fn hal2_gain_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let hal2 = unsafe { snd_kcontrol_chip(kcontrol) as *mut snd_hal2 };
    let old: u32;
    let mut new: u32;
    let mut l: c_int;
    let mut r: c_int;

    unsafe {
        l = (*ucontrol).value.integer.value[0] as c_int;
        r = (*ucontrol).value.integer.value[1] as c_int;

        match (*kcontrol).private_value as c_int {
            H2_MIX_OUTPUT_ATT => {
                old = hal2_i_read32(hal2, H2I_DAC_C2);
                new = old & !(H2I_C2_L_ATT_M | H2I_C2_R_ATT_M | H2I_C2_MUTE);
                if (l | r) != 0 {
                    l = 31 - l;
                    r = 31 - r;
                    new |= (l as u32) << H2I_C2_L_ATT_SHIFT;
                    new |= (r as u32) << H2I_C2_R_ATT_SHIFT;
                } else {
                    new |= H2I_C2_L_ATT_M | H2I_C2_R_ATT_M | H2I_C2_MUTE;
                }
                hal2_i_write32(hal2, H2I_DAC_C2, new);
            }
            H2_MIX_INPUT_GAIN => {
                old = hal2_i_read32(hal2, H2I_ADC_C2);
                new = old & !(H2I_C2_L_GAIN_M | H2I_C2_R_GAIN_M);
                new |= (l as u32) << H2I_C2_L_GAIN_SHIFT;
                new |= (r as u32) << H2I_C2_R_GAIN_SHIFT;
                hal2_i_write32(hal2, H2I_ADC_C2, new);
            }
            _ => return -EINVAL,
        }
    }
    (old != new) as c_int
}

static hal2_ctrl_headphone: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c_str!("Headphone Playback Volume"),
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    private_value: H2_MIX_OUTPUT_ATT as isize,
    info: Some(hal2_gain_info),
    get: Some(hal2_gain_get),
    put: Some(hal2_gain_put),
};

static hal2_ctrl_mic: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c_str!("Mic Capture Volume"),
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    private_value: H2_MIX_INPUT_GAIN as isize,
    info: Some(hal2_gain_info),
    get: Some(hal2_gain_get),
    put: Some(hal2_gain_put),
};

unsafe fn hal2_mixer_create(hal2: *mut snd_hal2) -> c_int {
    let mut err: c_int;

    /* mute DAC */
    unsafe { hal2_i_write32(hal2, H2I_DAC_C2, H2I_C2_L_ATT_M | H2I_C2_R_ATT_M | H2I_C2_MUTE) };
    /* mute ADC */
    unsafe { hal2_i_write32(hal2, H2I_ADC_C2, 0) };

    unsafe {
        err = snd_ctl_add((*hal2).card, snd_ctl_new1(&hal2_ctrl_headphone, hal2 as *mut c_void));
    }
    if err < 0 {
        return err;
    }

    unsafe {
        err = snd_ctl_add((*hal2).card, snd_ctl_new1(&hal2_ctrl_mic, hal2 as *mut c_void));
    }
    if err < 0 {
        return err;
    }

    0
}

unsafe extern "C" fn hal2_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let hal2 = dev_id as *mut snd_hal2;
    let mut ret = IRQ_NONE;

    /* decide what caused this interrupt */
    unsafe {
        if (*(*hal2).dac.pbus.pbus).pbdma_ctrl & HPC3_PDMACTRL_INT != 0 {
            snd_pcm_period_elapsed((*hal2).dac.substream);
            ret = IRQ_HANDLED;
        }
        if (*(*hal2).adc.pbus.pbus).pbdma_ctrl & HPC3_PDMACTRL_INT != 0 {
            snd_pcm_period_elapsed((*hal2).adc.substream);
            ret = IRQ_HANDLED;
        }
    }
    ret
}

unsafe fn hal2_compute_rate(codec: *mut hal2_codec, mut rate: c_uint) -> c_int {
    let mod_: u16;

    unsafe {
        if 44100 % rate < 48000 % rate {
            mod_ = (4 * 44100 / rate) as u16;
            (*codec).master = 44100;
        } else {
            mod_ = (4 * 48000 / rate) as u16;
            (*codec).master = 48000;
        }

        (*codec).inc = 4;
        (*codec).mod_ = mod_;
        rate = 4 * (*codec).master / mod_ as c_uint;
    }

    rate as c_int
}

unsafe fn hal2_set_dac_rate(hal2: *mut snd_hal2) {
    let master = unsafe { (*hal2).dac.master };
    let inc = unsafe { (*hal2).dac.inc as c_int };
    let mod_ = unsafe { (*hal2).dac.mod_ as c_int };

    unsafe { hal2_i_write16(hal2, H2I_BRES1_C1, if master == 44100 { 1 } else { 0 }) };
    unsafe {
        hal2_i_write32(
            hal2,
            H2I_BRES1_C2,
            (((0xffff & (inc - mod_ - 1)) as u32) << 16) | inc as u32,
        )
    };
}

unsafe fn hal2_set_adc_rate(hal2: *mut snd_hal2) {
    let master = unsafe { (*hal2).adc.master };
    let inc = unsafe { (*hal2).adc.inc as c_int };
    let mod_ = unsafe { (*hal2).adc.mod_ as c_int };

    unsafe { hal2_i_write16(hal2, H2I_BRES2_C1, if master == 44100 { 1 } else { 0 }) };
    unsafe {
        hal2_i_write32(
            hal2,
            H2I_BRES2_C2,
            (((0xffff & (inc - mod_ - 1)) as u32) << 16) | inc as u32,
        )
    };
}

unsafe fn hal2_setup_dac(hal2: *mut snd_hal2) {
    let fifobeg: c_uint;
    let fifoend: c_uint;
    let highwater: c_uint;
    let sample_size: c_uint;
    let pbus = unsafe { ptr::addr_of_mut!((*hal2).dac.pbus) };

    /* Now we set up some PBUS information. The PBUS needs information about
     * what portion of the fifo it will use. If it's receiving or
     * transmitting, and finally whether the stream is little endian or big
     * endian. The information is written later, on the start call.
     */
    unsafe {
        sample_size = (2 * (*hal2).dac.voices) as c_uint;
        /* Fifo should be set to hold exactly four samples. Highwater mark
         * should be set to two samples. */
        highwater = (sample_size * 2) >> 1; /* halfwords */
        fifobeg = 0; /* playback is first */
        fifoend = (sample_size * 4) >> 3; /* doublewords */
        (*pbus).ctrl = HPC3_PDMACTRL_RT
            | HPC3_PDMACTRL_LD
            | (highwater << 8)
            | (fifobeg << 16)
            | (fifoend << 24);
        /* We disable everything before we do anything at all */
        (*(*pbus).pbus).pbdma_ctrl = HPC3_PDMACTRL_LD;
        hal2_i_clearbit16(hal2, H2I_DMA_PORT_EN, H2I_DMA_PORT_EN_CODECTX);
        /* Setup the HAL2 for playback */
        hal2_set_dac_rate(hal2);
        /* Set endianess */
        hal2_i_clearbit16(hal2, H2I_DMA_END, H2I_DMA_END_CODECTX);
        /* Set DMA bus */
        hal2_i_setbit16(hal2, H2I_DMA_DRV, (1 << (*pbus).pbusnr) as u16);
        /* We are using 1st Bresenham clock generator for playback */
        hal2_i_write16(
            hal2,
            H2I_DAC_C1,
            (((*pbus).pbusnr << H2I_C1_DMA_SHIFT)
                | (1 << H2I_C1_CLKID_SHIFT)
                | ((*hal2).dac.voices << H2I_C1_DATAT_SHIFT)) as u16,
        );
    }
}

unsafe fn hal2_setup_adc(hal2: *mut snd_hal2) {
    let fifobeg: c_uint;
    let fifoend: c_uint;
    let highwater: c_uint;
    let sample_size: c_uint;
    let pbus = unsafe { ptr::addr_of_mut!((*hal2).adc.pbus) };

    unsafe {
        sample_size = (2 * (*hal2).adc.voices) as c_uint;
        highwater = (sample_size * 2) >> 1; /* halfwords */
        fifobeg = (4 * 4) >> 3; /* record is second */
        fifoend = (4 * 4 + sample_size * 4) >> 3; /* doublewords */
        (*pbus).ctrl = HPC3_PDMACTRL_RT
            | HPC3_PDMACTRL_RCV
            | HPC3_PDMACTRL_LD
            | (highwater << 8)
            | (fifobeg << 16)
            | (fifoend << 24);
        (*(*pbus).pbus).pbdma_ctrl = HPC3_PDMACTRL_LD;
        hal2_i_clearbit16(hal2, H2I_DMA_PORT_EN, H2I_DMA_PORT_EN_CODECR);
        /* Setup the HAL2 for record */
        hal2_set_adc_rate(hal2);
        /* Set endianess */
        hal2_i_clearbit16(hal2, H2I_DMA_END, H2I_DMA_END_CODECR);
        /* Set DMA bus */
        hal2_i_setbit16(hal2, H2I_DMA_DRV, (1 << (*pbus).pbusnr) as u16);
        /* We are using 2nd Bresenham clock generator for record */
        hal2_i_write16(
            hal2,
            H2I_ADC_C1,
            (((*pbus).pbusnr << H2I_C1_DMA_SHIFT)
                | (2 << H2I_C1_CLKID_SHIFT)
                | ((*hal2).adc.voices << H2I_C1_DATAT_SHIFT)) as u16,
        );
    }
}

unsafe fn hal2_start_dac(hal2: *mut snd_hal2) {
    let pbus = unsafe { ptr::addr_of_mut!((*hal2).dac.pbus) };

    unsafe {
        (*(*pbus).pbus).pbdma_dptr = (*hal2).dac.desc_dma;
        (*(*pbus).pbus).pbdma_ctrl = (*pbus).ctrl | HPC3_PDMACTRL_ACT;
        /* enable DAC */
        hal2_i_setbit16(hal2, H2I_DMA_PORT_EN, H2I_DMA_PORT_EN_CODECTX);
    }
}

unsafe fn hal2_start_adc(hal2: *mut snd_hal2) {
    let pbus = unsafe { ptr::addr_of_mut!((*hal2).adc.pbus) };

    unsafe {
        (*(*pbus).pbus).pbdma_dptr = (*hal2).adc.desc_dma;
        (*(*pbus).pbus).pbdma_ctrl = (*pbus).ctrl | HPC3_PDMACTRL_ACT;
        /* enable ADC */
        hal2_i_setbit16(hal2, H2I_DMA_PORT_EN, H2I_DMA_PORT_EN_CODECR);
    }
}

#[inline]
unsafe fn hal2_stop_dac(hal2: *mut snd_hal2) {
    unsafe {
        (*(*hal2).dac.pbus.pbus).pbdma_ctrl = HPC3_PDMACTRL_LD;
    }
    /* The HAL2 itself may remain enabled safely */
}

#[inline]
unsafe fn hal2_stop_adc(hal2: *mut snd_hal2) {
    unsafe {
        (*(*hal2).adc.pbus.pbus).pbdma_ctrl = HPC3_PDMACTRL_LD;
    }
}

unsafe fn hal2_alloc_dmabuf(
    hal2: *mut snd_hal2,
    codec: *mut hal2_codec,
    buffer_dir: dma_data_direction,
) -> c_int {
    let dev = unsafe { (*(*hal2).card).dev };
    let mut desc: *mut hal2_desc;
    let mut desc_dma: dma_addr_t = 0;
    let mut buffer_dma: dma_addr_t = 0;
    let count = H2_BUF_SIZE / H2_BLOCK_SIZE;
    let mut i: c_int;

    unsafe {
        (*codec).buffer = dma_alloc_noncoherent(
            dev,
            H2_BUF_SIZE as size_t,
            &mut buffer_dma,
            buffer_dir,
            GFP_KERNEL,
        ) as *mut u8;
        if (*codec).buffer.is_null() {
            return -ENOMEM;
        }
        desc = dma_alloc_noncoherent(
            dev,
            (count as usize) * size_of::<hal2_desc>(),
            &mut desc_dma,
            DMA_BIDIRECTIONAL,
            GFP_KERNEL,
        ) as *mut hal2_desc;
        if desc.is_null() {
            dma_free_noncoherent(
                dev,
                H2_BUF_SIZE as size_t,
                (*codec).buffer as *mut c_void,
                buffer_dma,
                buffer_dir,
            );
            return -ENOMEM;
        }
        (*codec).buffer_dma = buffer_dma;
        (*codec).desc_dma = desc_dma;
        (*codec).desc = desc;
        i = 0;
        while i < count {
            (*desc).desc.pbuf = buffer_dma + (i as usize) * H2_BLOCK_SIZE as usize;
            (*desc).desc.cntinfo = HPCDMA_XIE | H2_BLOCK_SIZE as u32;
            (*desc).desc.pnext = if i == count - 1 {
                desc_dma
            } else {
                desc_dma + ((i + 1) as usize) * size_of::<hal2_desc>()
            };
            desc = desc.add(1);
            i += 1;
        }
        dma_sync_single_for_device(
            dev,
            (*codec).desc_dma,
            (count as usize) * size_of::<hal2_desc>(),
            DMA_BIDIRECTIONAL,
        );
        (*codec).desc_count = count;
    }
    0
}

unsafe fn hal2_free_dmabuf(
    hal2: *mut snd_hal2,
    codec: *mut hal2_codec,
    buffer_dir: dma_data_direction,
) {
    let dev = unsafe { (*(*hal2).card).dev };

    unsafe {
        dma_free_noncoherent(
            dev,
            ((*codec).desc_count as usize) * size_of::<hal2_desc>(),
            (*codec).desc as *mut c_void,
            (*codec).desc_dma,
            DMA_BIDIRECTIONAL,
        );
        dma_free_noncoherent(
            dev,
            H2_BUF_SIZE as size_t,
            (*codec).buffer as *mut c_void,
            (*codec).buffer_dma,
            buffer_dir,
        );
    }
}

static hal2_pcm_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_SYNC_APPLPTR,
    formats: SNDRV_PCM_FMTBIT_S16_BE,
    rates: SNDRV_PCM_RATE_8000_48000,
    rate_min: 8000,
    rate_max: 48000,
    channels_min: 2,
    channels_max: 2,
    buffer_bytes_max: 65536,
    period_bytes_min: 1024,
    period_bytes_max: 65536,
    periods_min: 2,
    periods_max: 1024,
};

unsafe extern "C" fn hal2_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = unsafe { (*substream).runtime };
    let hal2 = unsafe { snd_pcm_substream_chip(substream) };

    unsafe {
        (*runtime).hw = hal2_pcm_hw;
        hal2_alloc_dmabuf(hal2, ptr::addr_of_mut!((*hal2).dac), DMA_TO_DEVICE)
    }
}

unsafe extern "C" fn hal2_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    let hal2 = unsafe { snd_pcm_substream_chip(substream) };

    unsafe { hal2_free_dmabuf(hal2, ptr::addr_of_mut!((*hal2).dac), DMA_TO_DEVICE) };
    0
}

unsafe extern "C" fn hal2_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let hal2 = unsafe { snd_pcm_substream_chip(substream) };
    let runtime = unsafe { (*substream).runtime };
    let dac = unsafe { ptr::addr_of_mut!((*hal2).dac) };

    unsafe {
        (*dac).voices = (*runtime).channels as c_int;
        (*dac).sample_rate = hal2_compute_rate(dac, (*runtime).rate) as c_uint;
        ptr::write_bytes(
            ptr::addr_of_mut!((*dac).pcm_indirect) as *mut u8,
            0,
            size_of::<snd_pcm_indirect>(),
        );
        (*dac).pcm_indirect.hw_buffer_size = H2_BUF_SIZE as size_t;
        (*dac).pcm_indirect.hw_queue_size = (H2_BUF_SIZE / 2) as size_t;
        (*dac).pcm_indirect.hw_io = (*dac).buffer_dma;
        (*dac).pcm_indirect.sw_buffer_size = snd_pcm_lib_buffer_bytes(substream);
        (*dac).substream = substream;
        hal2_setup_dac(hal2);
    }
    0
}

unsafe extern "C" fn hal2_playback_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let hal2 = unsafe { snd_pcm_substream_chip(substream) };

    unsafe {
        match cmd {
            SNDRV_PCM_TRIGGER_START => hal2_start_dac(hal2),
            SNDRV_PCM_TRIGGER_STOP => hal2_stop_dac(hal2),
            _ => return -EINVAL,
        }
    }
    0
}

unsafe extern "C" fn hal2_playback_pointer(
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let hal2 = unsafe { snd_pcm_substream_chip(substream) };
    let dac = unsafe { ptr::addr_of_mut!((*hal2).dac) };

    unsafe {
        snd_pcm_indirect_playback_pointer(
            substream,
            ptr::addr_of_mut!((*dac).pcm_indirect),
            (*(*dac).pbus.pbus).pbdma_bptr,
        )
    }
}

unsafe extern "C" fn hal2_playback_transfer(
    substream: *mut snd_pcm_substream,
    rec: *mut snd_pcm_indirect,
    bytes: size_t,
) {
    let hal2 = unsafe { snd_pcm_substream_chip(substream) };
    let buf = unsafe { (*hal2).dac.buffer.add((*rec).hw_data) };

    unsafe {
        ptr::copy_nonoverlapping((*(*substream).runtime).dma_area.add((*rec).sw_data), buf, bytes);
        dma_sync_single_for_device(
            (*(*hal2).card).dev,
            (*hal2).dac.buffer_dma + (*rec).hw_data,
            bytes,
            DMA_TO_DEVICE,
        );
    }
}

unsafe extern "C" fn hal2_playback_ack(substream: *mut snd_pcm_substream) -> c_int {
    let hal2 = unsafe { snd_pcm_substream_chip(substream) };
    let dac = unsafe { ptr::addr_of_mut!((*hal2).dac) };

    unsafe {
        snd_pcm_indirect_playback_transfer(
            substream,
            ptr::addr_of_mut!((*dac).pcm_indirect),
            hal2_playback_transfer,
        )
    }
}

unsafe extern "C" fn hal2_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = unsafe { (*substream).runtime };
    let hal2 = unsafe { snd_pcm_substream_chip(substream) };

    unsafe {
        (*runtime).hw = hal2_pcm_hw;
        hal2_alloc_dmabuf(hal2, ptr::addr_of_mut!((*hal2).adc), DMA_FROM_DEVICE)
    }
}

unsafe extern "C" fn hal2_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    let hal2 = unsafe { snd_pcm_substream_chip(substream) };

    unsafe { hal2_free_dmabuf(hal2, ptr::addr_of_mut!((*hal2).adc), DMA_FROM_DEVICE) };
    0
}

unsafe extern "C" fn hal2_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let hal2 = unsafe { snd_pcm_substream_chip(substream) };
    let runtime = unsafe { (*substream).runtime };
    let adc = unsafe { ptr::addr_of_mut!((*hal2).adc) };

    unsafe {
        (*adc).voices = (*runtime).channels as c_int;
        (*adc).sample_rate = hal2_compute_rate(adc, (*runtime).rate) as c_uint;
        ptr::write_bytes(
            ptr::addr_of_mut!((*adc).pcm_indirect) as *mut u8,
            0,
            size_of::<snd_pcm_indirect>(),
        );
        (*adc).pcm_indirect.hw_buffer_size = H2_BUF_SIZE as size_t;
        (*adc).pcm_indirect.hw_queue_size = (H2_BUF_SIZE / 2) as size_t;
        (*adc).pcm_indirect.hw_io = (*adc).buffer_dma;
        (*adc).pcm_indirect.sw_buffer_size = snd_pcm_lib_buffer_bytes(substream);
        (*adc).substream = substream;
        hal2_setup_adc(hal2);
    }
    0
}

unsafe extern "C" fn hal2_capture_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let hal2 = unsafe { snd_pcm_substream_chip(substream) };

    unsafe {
        match cmd {
            SNDRV_PCM_TRIGGER_START => hal2_start_adc(hal2),
            SNDRV_PCM_TRIGGER_STOP => hal2_stop_adc(hal2),
            _ => return -EINVAL,
        }
    }
    0
}

unsafe extern "C" fn hal2_capture_pointer(
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let hal2 = unsafe { snd_pcm_substream_chip(substream) };
    let adc = unsafe { ptr::addr_of_mut!((*hal2).adc) };

    unsafe {
        snd_pcm_indirect_capture_pointer(
            substream,
            ptr::addr_of_mut!((*adc).pcm_indirect),
            (*(*adc).pbus.pbus).pbdma_bptr,
        )
    }
}

unsafe extern "C" fn hal2_capture_transfer(
    substream: *mut snd_pcm_substream,
    rec: *mut snd_pcm_indirect,
    bytes: size_t,
) {
    let hal2 = unsafe { snd_pcm_substream_chip(substream) };
    let buf = unsafe { (*hal2).adc.buffer.add((*rec).hw_data) };

    unsafe {
        dma_sync_single_for_cpu(
            (*(*hal2).card).dev,
            (*hal2).adc.buffer_dma + (*rec).hw_data,
            bytes,
            DMA_FROM_DEVICE,
        );
        ptr::copy_nonoverlapping(buf, (*(*substream).runtime).dma_area.add((*rec).sw_data), bytes);
    }
}

unsafe extern "C" fn hal2_capture_ack(substream: *mut snd_pcm_substream) -> c_int {
    let hal2 = unsafe { snd_pcm_substream_chip(substream) };
    let adc = unsafe { ptr::addr_of_mut!((*hal2).adc) };

    unsafe {
        snd_pcm_indirect_capture_transfer(
            substream,
            ptr::addr_of_mut!((*adc).pcm_indirect),
            hal2_capture_transfer,
        )
    }
}

static hal2_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(hal2_playback_open),
    close: Some(hal2_playback_close),
    prepare: Some(hal2_playback_prepare),
    trigger: Some(hal2_playback_trigger),
    pointer: Some(hal2_playback_pointer),
    ack: Some(hal2_playback_ack),
};

static hal2_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(hal2_capture_open),
    close: Some(hal2_capture_close),
    prepare: Some(hal2_capture_prepare),
    trigger: Some(hal2_capture_trigger),
    pointer: Some(hal2_capture_pointer),
    ack: Some(hal2_capture_ack),
};

unsafe fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut snd_hal2 {
    unsafe { (*snd_pcm_parent(substream)).private_data as *mut snd_hal2 }
}

unsafe extern "C" {
    fn snd_pcm_parent(substream: *mut snd_pcm_substream) -> *mut snd_pcm;
}

unsafe fn hal2_pcm_create(hal2: *mut snd_hal2) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let mut err: c_int;

    /* create first pcm device with one outputs and one input */
    unsafe {
        err = snd_pcm_new((*hal2).card, c_str!("SGI HAL2 Audio"), 0, 1, 1, &mut pcm);
    }
    if err < 0 {
        return err;
    }

    unsafe {
        (*pcm).private_data = hal2 as *mut c_void;
        strscpy((*pcm).name.as_mut_ptr(), c_str!("SGI HAL2"));

        /* set operators */
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &hal2_playback_ops);
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &hal2_capture_ops);
        snd_pcm_set_managed_buffer_all(
            pcm,
            SNDRV_DMA_TYPE_CONTINUOUS,
            ptr::null_mut(),
            0,
            1024 * 1024,
        );
    }

    0
}

unsafe extern "C" fn hal2_dev_free(device: *mut snd_device) -> c_int {
    let hal2 = unsafe { (*device).device_data as *mut snd_hal2 };

    unsafe {
        free_irq(SGI_HPCDMA_IRQ, hal2 as *mut c_void);
        kfree(hal2 as *mut c_void);
    }
    0
}

static hal2_ops: snd_device_ops = snd_device_ops {
    dev_free: Some(hal2_dev_free),
};

unsafe fn hal2_init_codec(codec: *mut hal2_codec, hpc3: *mut hpc3_regs, index: c_int) {
    unsafe {
        (*codec).pbus.pbusnr = index;
        (*codec).pbus.pbus = ptr::addr_of_mut!((*hpc3).pbdma[index as usize]);
    }
}

unsafe fn hal2_detect(hal2: *mut snd_hal2) -> c_int {
    let board: u16;
    let major: u16;
    let minor: u16;
    let rev: u16;

    unsafe {
        /* reset HAL2 */
        hal2_write(0, ptr::addr_of_mut!((*(*hal2).ctl_regs).isr));

        /* release reset */
        hal2_write(
            H2_ISR_GLOBAL_RESET_N | H2_ISR_CODEC_RESET_N,
            ptr::addr_of_mut!((*(*hal2).ctl_regs).isr),
        );

        hal2_i_write16(hal2, H2I_RELAY_C, H2I_RELAY_C_STATE);
        rev = hal2_read(ptr::addr_of_mut!((*(*hal2).ctl_regs).rev)) as u16;
        if (rev as u32) & H2_REV_AUDIO_PRESENT != 0 {
            return -ENODEV;
        }

        board = (((rev as u32) & H2_REV_BOARD_M) >> 12) as u16;
        major = (((rev as u32) & H2_REV_MAJOR_CHIP_M) >> 4) as u16;
        minor = ((rev as u32) & H2_REV_MINOR_CHIP_M) as u16;

        printk(
            c_str!("SGI HAL2 revision %i.%i.%i\n"),
            board as c_int,
            major as c_int,
            minor as c_int,
        );
    }

    0
}

unsafe fn hal2_create(card: *mut snd_card, rchip: *mut *mut snd_hal2) -> c_int {
    let hal2: *mut snd_hal2;
    let hpc3 = unsafe { hpc3c0 };
    let mut err: c_int;

    unsafe {
        hal2 = kzalloc(size_of::<snd_hal2>(), GFP_KERNEL) as *mut snd_hal2;
    }
    if hal2.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*hal2).card = card;

        if request_irq(
            SGI_HPCDMA_IRQ,
            hal2_interrupt,
            IRQF_SHARED,
            c_str!("SGI HAL2"),
            hal2 as *mut c_void,
        ) != 0
        {
            printk(c_str!("HAL2: Can't get irq %d\n"), SGI_HPCDMA_IRQ);
            kfree(hal2 as *mut c_void);
            return -EAGAIN;
        }

        (*hal2).ctl_regs = (*hpc3).pbus_extregs[0] as *mut hal2_ctl_regs;
        (*hal2).aes_regs = (*hpc3).pbus_extregs[1] as *mut hal2_aes_regs;
        (*hal2).vol_regs = (*hpc3).pbus_extregs[2] as *mut hal2_vol_regs;
        (*hal2).syn_regs = (*hpc3).pbus_extregs[3] as *mut hal2_syn_regs;

        if hal2_detect(hal2) < 0 {
            kfree(hal2 as *mut c_void);
            return -ENODEV;
        }

        hal2_init_codec(ptr::addr_of_mut!((*hal2).dac), hpc3, 0);
        hal2_init_codec(ptr::addr_of_mut!((*hal2).adc), hpc3, 1);

        /*
         * All DMA channel interfaces in HAL2 are designed to operate with
         * PBUS programmed for 2 cycles in D3, 2 cycles in D4 and 2 cycles
         * in D5. HAL2 is a 16-bit device which can accept both big and little
         * endian format. It assumes that even address bytes are on high
         * portion of PBUS (15:8) and assumes that HPC3 is programmed to
         * accept a live (unsynchronized) version of P_DREQ_N from HAL2.
         */
        const HAL2_PBUS_DMACFG: u32 = ((0 << HPC3_DMACFG_D3R_SHIFT)
            | (2 << HPC3_DMACFG_D4R_SHIFT)
            | (2 << HPC3_DMACFG_D5R_SHIFT)
            | (0 << HPC3_DMACFG_D3W_SHIFT)
            | (2 << HPC3_DMACFG_D4W_SHIFT)
            | (2 << HPC3_DMACFG_D5W_SHIFT)
            | HPC3_DMACFG_DS16
            | HPC3_DMACFG_EVENHI
            | HPC3_DMACFG_RTIME
            | (8 << HPC3_DMACFG_BURST_SHIFT)
            | HPC3_DMACFG_DRQLIVE) as u32;
        let _ = HAL2_PBUS_DMACFG;
        /*
         * Ignore what's mentioned in the specification and write value which
         * works in The Real World (TM)
         */
        (*hpc3).pbus_dmacfg[(*hal2).dac.pbus.pbusnr as usize][0] = 0x8208844;
        (*hpc3).pbus_dmacfg[(*hal2).adc.pbus.pbusnr as usize][0] = 0x8208844;

        err = snd_device_new(card, SNDRV_DEV_LOWLEVEL, hal2 as *mut c_void, &hal2_ops);
        if err < 0 {
            free_irq(SGI_HPCDMA_IRQ, hal2 as *mut c_void);
            kfree(hal2 as *mut c_void);
            return err;
        }
        *rchip = hal2;
    }
    0
}

unsafe extern "C" fn hal2_probe(pdev: *mut platform_device) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let mut chip: *mut snd_hal2 = ptr::null_mut();
    let mut err: c_int;

    unsafe {
        err = snd_card_new(
            ptr::addr_of_mut!((*pdev).dev),
            index,
            id,
            THIS_MODULE,
            0,
            &mut card,
        );
    }
    if err < 0 {
        return err;
    }

    unsafe {
        err = hal2_create(card, &mut chip);
    }
    if err < 0 {
        unsafe { snd_card_free(card) };
        return err;
    }

    unsafe {
        err = hal2_pcm_create(chip);
    }
    if err < 0 {
        unsafe { snd_card_free(card) };
        return err;
    }
    unsafe {
        err = hal2_mixer_create(chip);
    }
    if err < 0 {
        unsafe { snd_card_free(card) };
        return err;
    }

    unsafe {
        strscpy((*card).driver.as_mut_ptr(), c_str!("SGI HAL2 Audio"));
        strscpy((*card).shortname.as_mut_ptr(), c_str!("SGI HAL2 Audio"));
        sprintf(
            (*card).longname.as_mut_ptr(),
            c_str!("%s irq %i"),
            (*card).shortname.as_ptr(),
            SGI_HPCDMA_IRQ,
        );

        err = snd_card_register(card);
    }
    if err < 0 {
        unsafe { snd_card_free(card) };
        return err;
    }
    unsafe { platform_set_drvdata(pdev, card as *mut c_void) };
    0
}

unsafe extern "C" fn hal2_remove(pdev: *mut platform_device) {
    let card = unsafe { platform_get_drvdata(pdev) as *mut snd_card };

    unsafe { snd_card_free(card) };
}

static mut hal2_driver: platform_driver = platform_driver {
    probe: Some(hal2_probe),
    remove: Some(hal2_remove),
    driver: device_driver {
        name: c_str!("sgihal2"),
    },
};

// module_platform_driver(hal2_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
