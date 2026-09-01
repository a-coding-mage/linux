// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA modem driver for Intel ICH (i8x0) chipsets
 *
 *	Copyright (c) 2000 Jaroslav Kysela <perex@perex.cz>
 *
 *   This is modified (by Sasha Khapyorsky <sashak@alsa-project.org>) version
 *   of ALSA ICH sound driver intel8x0.c .
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u16 = u16;
type u32 = u32;
type size_t = usize;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_int;
type bool_t = bool;
type __le32 = u32;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pci_dev {
    pub dev: device,
    pub irq: c_int,
    pub device: u32,
}
#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub private_data: *mut c_void,
    pub sync_irq: c_int,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
}
#[repr(C)]
pub struct snd_pcm {
    pub private_data: *mut c_void,
    pub info_flags: c_int,
    pub dev_class: c_int,
    pub name: [c_char; 80],
}
#[repr(C)]
pub struct snd_pcm_runtime {
    pub private_data: *mut c_void,
    pub hw: snd_pcm_hardware,
    pub dma_addr: u32,
    pub rate: u32,
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}
#[repr(C)]
pub struct snd_ac97 {
    pub private_data: *mut c_void,
    pub num: u32,
}
#[repr(C)]
pub struct snd_ac97_bus {
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_ac97_bus)>,
    pub clock: c_int,
}
#[repr(C)]
pub struct snd_info_entry {
    pub private_data: *mut c_void,
}
#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_dma_buffer {
    pub area: *mut c_void,
    pub addr: u32,
}
#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_device_id {
    pub vendor: u32,
    pub device: u32,
    pub subvendor: u32,
    pub subdevice: u32,
    pub class: u32,
    pub class_mask: u32,
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: u32,
    pub formats: u64,
    pub rates: u32,
    pub rate_min: u32,
    pub rate_max: u32,
    pub channels_min: u32,
    pub channels_max: u32,
    pub buffer_bytes_max: size_t,
    pub period_bytes_min: size_t,
    pub period_bytes_max: size_t,
    pub periods_min: u32,
    pub periods_max: u32,
    pub fifo_size: size_t,
}

#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub ioctl: Option<unsafe extern "C" fn() -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn() -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn() -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: u32,
    pub list: *const u32,
    pub mask: u32,
}

#[repr(C)]
pub struct snd_ac97_bus_ops {
    pub write: Option<unsafe extern "C" fn(*mut snd_ac97, u16, u16)>,
    pub read: Option<unsafe extern "C" fn(*mut snd_ac97, u16) -> u16>,
}

#[repr(C)]
pub struct snd_ac97_template {
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_ac97)>,
    pub scaps: u32,
    pub pci: *mut pci_dev,
    pub num: u32,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_driver_driver {
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct pci_driver {
    pub name: *const c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
    pub driver: pci_driver_driver,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;
    static KBUILD_MODNAME: *const c_char;
    static mut jiffies: c_ulong;

    fn ioread8(addr: *mut c_void) -> u8;
    fn ioread16(addr: *mut c_void) -> u16;
    fn ioread32(addr: *mut c_void) -> u32;
    fn iowrite8(val: u8, addr: *mut c_void);
    fn iowrite16(val: u16, addr: *mut c_void);
    fn iowrite32(val: u32, addr: *mut c_void);
    fn udelay(usecs: c_ulong);
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn schedule_timeout_uninterruptible(timeout: c_ulong) -> c_ulong;
    fn time_after_eq(a: c_ulong, b: c_ulong) -> bool_t;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_BUG_ON(cond: bool_t) -> bool_t;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn free_irq(irq: c_int, dev_id: *mut c_void);
    fn request_irq(
        irq: c_int,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        flags: c_ulong,
        name: *const c_char,
        dev: *mut c_void,
    ) -> c_int;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut intel8x0m;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: size_t) -> snd_pcm_uframes_t;
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> u32;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> u32;
    fn snd_ac97_write(ac97: *mut snd_ac97, reg: u16, val: u16);
    fn snd_pcm_hw_constraint_list(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_uint,
        l: *const snd_pcm_hw_constraint_list,
    ) -> c_int;
    fn snd_pcm_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        playback_count: c_int,
        capture_count: c_int,
        rpcm: *mut *mut snd_pcm,
    ) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        ty: c_int,
        dev: *mut device,
        size: size_t,
        max: size_t,
    );
    fn sprintf(s: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> size_t;
    fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn snd_ac97_bus(
        card: *mut snd_card,
        num: c_int,
        ops: *const snd_ac97_bus_ops,
        private_data: *mut c_void,
        rbus: *mut *mut snd_ac97_bus,
    ) -> c_int;
    fn snd_ac97_mixer(
        bus: *mut snd_ac97_bus,
        template: *mut snd_ac97_template,
        rac97: *mut *mut snd_ac97,
    ) -> c_int;
    fn ac97_is_modem(ac97: *mut snd_ac97) -> bool_t;
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn snd_ac97_suspend(ac97: *mut snd_ac97);
    fn snd_ac97_resume(ac97: *mut snd_ac97);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn to_pci_dev(dev: *mut device) -> *mut pci_dev;
    fn snd_card_disconnect(card: *mut snd_card);
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_card_ro_proc_new(
        card: *mut snd_card,
        name: *const c_char,
        private_data: *mut c_void,
        read: unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer),
    );
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn pcim_request_all_regions(pci: *mut pci_dev, name: *const c_char) -> c_int;
    fn pci_resource_flags(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn pcim_iomap(pci: *mut pci_dev, bar: c_int, maxlen: c_ulong) -> *mut c_void;
    fn snd_devm_alloc_pages(dev: *mut device, ty: c_int, size: size_t) -> *mut snd_dma_buffer;
    fn pci_set_master(pci: *mut pci_dev);
    fn snd_devm_card_new(
        dev: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut module,
        extra_size: size_t,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn snd_card_free_on_error(dev: *mut device, err: c_int) -> c_int;
}

type c_uint = u32;

const EIO: c_int = 5;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_SHARED: c_ulong = 0x80;
const HZ: c_ulong = 100;
const IORESOURCE_MEM: c_ulong = 0x00000200;

const SNDRV_DEFAULT_STR1: *mut c_char = ptr::null_mut();
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_PCM_INFO_MMAP: u32 = 0x00000001;
const SNDRV_PCM_INFO_INTERLEAVED: u32 = 0x00000100;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: u32 = 0x00010000;
const SNDRV_PCM_INFO_MMAP_VALID: u32 = 0x00000002;
const SNDRV_PCM_INFO_PAUSE: u32 = 0x00080000;
const SNDRV_PCM_INFO_RESUME: u32 = 0x00100000;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 2;
const SNDRV_PCM_RATE_8000: u32 = 1 << 0;
const SNDRV_PCM_RATE_16000: u32 = 1 << 4;
const SNDRV_PCM_RATE_KNOT: u32 = 1 << 31;
const SNDRV_PCM_HW_PARAM_RATE: c_uint = 10;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_CLASS_MODEM: c_int = 2;
const SNDRV_DMA_TYPE_DEV: c_int = 1;
const AC97_GPIO_STATUS: u16 = 0x54;
const AC97_LINE1_RATE: u16 = 0x40;
const AC97_LINE1_LEVEL: u16 = 0x38;
const AC97_SCAP_SKIP_AUDIO: u32 = 1 << 0;
const AC97_SCAP_POWER_SAVE: u32 = 1 << 1;
const SNDRV_CTL_POWER_D3hot: c_int = 3;
const SNDRV_CTL_POWER_D0: c_int = 0;

const PCI_VENDOR_ID_INTEL: u32 = 0x8086;
const PCI_VENDOR_ID_AMD: u32 = 0x1022;
const PCI_VENDOR_ID_SI: u32 = 0x1039;
const PCI_VENDOR_ID_NVIDIA: u32 = 0x10de;
const PCI_DEVICE_ID_INTEL_82801AA_6: u32 = 0x2416;
const PCI_DEVICE_ID_INTEL_82801AB_6: u32 = 0x2426;
const PCI_DEVICE_ID_INTEL_82801BA_6: u32 = 0x2446;
const PCI_DEVICE_ID_INTEL_440MX_6: u32 = 0x7196;
const PCI_DEVICE_ID_INTEL_82801CA_6: u32 = 0x2486;
const PCI_DEVICE_ID_INTEL_82801DB_6: u32 = 0x24c6;
const PCI_DEVICE_ID_INTEL_82801EB_6: u32 = 0x24d6;
const PCI_DEVICE_ID_INTEL_ICH6_17: u32 = 0x266d;
const PCI_DEVICE_ID_INTEL_ICH7_19: u32 = 0x27dd;
const PCI_DEVICE_ID_SI_7013: u32 = 0x7013;
const PCI_DEVICE_ID_NVIDIA_MCP1_MODEM: u32 = 0x01c1;
const PCI_DEVICE_ID_NVIDIA_MCP2_MODEM: u32 = 0x0069;
const PCI_DEVICE_ID_NVIDIA_MCP2S_MODEM: u32 = 0x0089;
const PCI_DEVICE_ID_NVIDIA_MCP3_MODEM: u32 = 0x00d9;

// Module metadata and module_param declarations are Linux module declarations in C.
static mut index: c_int = -2; /* Exclude the first card */
static mut id: *mut c_char = SNDRV_DEFAULT_STR1; /* ID for this card */
static mut ac97_clock: c_int = 0;

/* just for backward compatibility */
static mut enable: bool_t = false;

/*
 *  Direct registers
 */
const DEVICE_INTEL: u32 = 0;
const DEVICE_SIS: u32 = 1;
const DEVICE_ALI: u32 = 2;
const DEVICE_NFORCE: u32 = 3;

/* busmaster blocks */
const ICH_REG_OFF_BDBAR: u32 = 0x0; /* dword - buffer descriptor list base address */
const ICH_REG_OFF_CIV: u32 = 0x04; /* byte - current index value */
const ICH_REG_OFF_LVI: u32 = 0x05; /* byte - last valid index */
const ICH_REG_OFF_SR: u32 = 0x06; /* byte - status register */
const ICH_REG_OFF_PICB: u32 = 0x08; /* word - position in current buffer */
const ICH_REG_OFF_PIV: u32 = 0x0a; /* byte - prefetched index value */
const ICH_REG_OFF_CR: u32 = 0x0b; /* byte - control register */

/* values for each busmaster block */

/* LVI */
const ICH_REG_LVI_MASK: u32 = 0x1f;

/* SR */
const ICH_FIFOE: u32 = 0x10; /* FIFO error */
const ICH_BCIS: u32 = 0x08; /* buffer completion interrupt status */
const ICH_LVBCI: u32 = 0x04; /* last valid buffer completion interrupt */
const ICH_CELV: u32 = 0x02; /* current equals last valid */
const ICH_DCH: u32 = 0x01; /* DMA controller halted */

/* PIV */
const ICH_REG_PIV_MASK: u32 = 0x1f; /* mask */

/* CR */
const ICH_IOCE: u32 = 0x10; /* interrupt on completion enable */
const ICH_FEIE: u32 = 0x08; /* fifo error interrupt enable */
const ICH_LVBIE: u32 = 0x04; /* last valid buffer interrupt enable */
const ICH_RESETREGS: u32 = 0x02; /* reset busmaster registers */
const ICH_STARTBM: u32 = 0x01; /* start busmaster operation */

/* global block */
const ICH_REG_GLOB_CNT: u32 = 0x3c; /* dword - global control */
const ICH_TRIE: u32 = 0x00000040; /* tertiary resume interrupt enable */
const ICH_SRIE: u32 = 0x00000020; /* secondary resume interrupt enable */
const ICH_PRIE: u32 = 0x00000010; /* primary resume interrupt enable */
const ICH_ACLINK: u32 = 0x00000008; /* AClink shut off */
const ICH_AC97WARM: u32 = 0x00000004; /* AC'97 warm reset */
const ICH_AC97COLD: u32 = 0x00000002; /* AC'97 cold reset */
const ICH_GIE: u32 = 0x00000001; /* GPI interrupt enable */
const ICH_REG_GLOB_STA: u32 = 0x40; /* dword - global status */
const ICH_TRI: u32 = 0x20000000; /* ICH4: tertiary (AC_SDIN2) resume interrupt */
const ICH_TCR: u32 = 0x10000000; /* ICH4: tertiary (AC_SDIN2) codec ready */
const ICH_BCS: u32 = 0x08000000; /* ICH4: bit clock stopped */
const ICH_SPINT: u32 = 0x04000000; /* ICH4: S/PDIF interrupt */
const ICH_P2INT: u32 = 0x02000000; /* ICH4: PCM2-In interrupt */
const ICH_M2INT: u32 = 0x01000000; /* ICH4: Mic2-In interrupt */
const ICH_SAMPLE_CAP: u32 = 0x00c00000; /* ICH4: sample capability bits (RO) */
const ICH_MULTICHAN_CAP: u32 = 0x00300000; /* ICH4: multi-channel capability bits (RO) */
const ICH_MD3: u32 = 0x00020000; /* modem power down semaphore */
const ICH_AD3: u32 = 0x00010000; /* audio power down semaphore */
const ICH_RCS: u32 = 0x00008000; /* read completion status */
const ICH_BIT3: u32 = 0x00004000; /* bit 3 slot 12 */
const ICH_BIT2: u32 = 0x00002000; /* bit 2 slot 12 */
const ICH_BIT1: u32 = 0x00001000; /* bit 1 slot 12 */
const ICH_SRI: u32 = 0x00000800; /* secondary (AC_SDIN1) resume interrupt */
const ICH_PRI: u32 = 0x00000400; /* primary (AC_SDIN0) resume interrupt */
const ICH_SCR: u32 = 0x00000200; /* secondary (AC_SDIN1) codec ready */
const ICH_PCR: u32 = 0x00000100; /* primary (AC_SDIN0) codec ready */
const ICH_MCINT: u32 = 0x00000080; /* MIC capture interrupt */
const ICH_POINT: u32 = 0x00000040; /* playback interrupt */
const ICH_PIINT: u32 = 0x00000020; /* capture interrupt */
const ICH_NVSPINT: u32 = 0x00000010; /* nforce spdif interrupt */
const ICH_MOINT: u32 = 0x00000004; /* modem playback interrupt */
const ICH_MIINT: u32 = 0x00000002; /* modem capture interrupt */
const ICH_GSCI: u32 = 0x00000001; /* GPI status change interrupt */
const ICH_REG_ACC_SEMA: u32 = 0x44; /* byte - codec write semaphore */
const ICH_CAS: u32 = 0x01; /* codec access semaphore */

const ICH_MAX_FRAGS: u32 = 32; /* max hw frags */

const ICHD_MDMIN: usize = 0;
const ICHD_MDMOUT: usize = 1;
const ICHD_MDMLAST: usize = ICHD_MDMOUT;
const ALID_MDMIN: usize = 0;
const ALID_MDMOUT: usize = 1;
const ALID_MDMLAST: usize = ALID_MDMOUT;

unsafe fn get_ichdev(substream: *mut snd_pcm_substream) -> *mut ichdev {
    (*(*substream).runtime).private_data as *mut ichdev
}

#[repr(C)]
pub struct ichdev {
    pub ichd: u32, /* ich device number */
    pub reg_offset: c_ulong, /* offset to bmaddr */
    pub bdbar: *mut __le32, /* CPU address (32bit) */
    pub bdbar_addr: u32, /* PCI bus address (32bit) */
    pub substream: *mut snd_pcm_substream,
    pub physbuf: u32, /* physical address (32bit) */
    pub size: u32,
    pub fragsize: u32,
    pub fragsize1: u32,
    pub position: u32,
    pub frags: c_int,
    pub lvi: c_int,
    pub lvi_frag: c_int,
    pub civ: c_int,
    pub ack: c_int,
    pub ack_reload: c_int,
    pub ack_bit: u32,
    pub roff_sr: u32,
    pub roff_picb: u32,
    pub int_sta_mask: u32, /* interrupt status mask */
    pub ali_slot: u32, /* ALI DMA slot */
    pub ac97: *mut snd_ac97,
}

#[repr(C)]
pub struct intel8x0m {
    pub device_type: u32,
    pub irq: c_int,
    pub addr: *mut c_void,
    pub bmaddr: *mut c_void,
    pub pci: *mut pci_dev,
    pub card: *mut snd_card,
    pub pcm_devs: c_int,
    pub pcm: [*mut snd_pcm; 2],
    pub ichd: [ichdev; 2],
    pub in_ac97_init: u32,
    pub ac97_bus: *mut snd_ac97_bus,
    pub ac97: *mut snd_ac97,
    pub reg_lock: spinlock_t,
    pub bdbars: *mut snd_dma_buffer,
    pub bdbars_count: u32,
    pub int_sta_reg: u32, /* interrupt status register */
    pub int_sta_mask: u32, /* interrupt status mask */
    pub pcm_pos_shift: u32,
}

const fn pci_vdevice(vendor: u32, device: u32, driver_data: u32) -> pci_device_id {
    pci_device_id {
        vendor,
        device,
        subvendor: 0xffffffff,
        subdevice: 0xffffffff,
        class: 0,
        class_mask: 0,
        driver_data: driver_data as c_ulong,
    }
}

static snd_intel8x0m_ids: [pci_device_id; 16] = [
    pci_vdevice(PCI_VENDOR_ID_INTEL, 0x2416, DEVICE_INTEL), /* 82801AA */
    pci_vdevice(PCI_VENDOR_ID_INTEL, 0x2426, DEVICE_INTEL), /* 82901AB */
    pci_vdevice(PCI_VENDOR_ID_INTEL, 0x2446, DEVICE_INTEL), /* 82801BA */
    pci_vdevice(PCI_VENDOR_ID_INTEL, 0x2486, DEVICE_INTEL), /* ICH3 */
    pci_vdevice(PCI_VENDOR_ID_INTEL, 0x24c6, DEVICE_INTEL), /* ICH4 */
    pci_vdevice(PCI_VENDOR_ID_INTEL, 0x24d6, DEVICE_INTEL), /* ICH5 */
    pci_vdevice(PCI_VENDOR_ID_INTEL, 0x266d, DEVICE_INTEL), /* ICH6 */
    pci_vdevice(PCI_VENDOR_ID_INTEL, 0x27dd, DEVICE_INTEL), /* ICH7 */
    pci_vdevice(PCI_VENDOR_ID_INTEL, 0x7196, DEVICE_INTEL), /* 440MX */
    pci_vdevice(PCI_VENDOR_ID_AMD, 0x7446, DEVICE_INTEL), /* AMD768 */
    pci_vdevice(PCI_VENDOR_ID_SI, 0x7013, DEVICE_SIS), /* SI7013 */
    pci_vdevice(PCI_VENDOR_ID_NVIDIA, 0x01c1, DEVICE_NFORCE), /* NFORCE */
    pci_vdevice(PCI_VENDOR_ID_NVIDIA, 0x0069, DEVICE_NFORCE), /* NFORCE2 */
    pci_vdevice(PCI_VENDOR_ID_NVIDIA, 0x0089, DEVICE_NFORCE), /* NFORCE2s */
    pci_vdevice(PCI_VENDOR_ID_NVIDIA, 0x00d9, DEVICE_NFORCE), /* NFORCE3 */
    pci_vdevice(PCI_VENDOR_ID_AMD, 0x746e, DEVICE_INTEL), /* AMD8111 */
    /* #if 0: Ali5455 entry with DEVICE_ALI omitted as disabled C code */
];

/*
 *  Lowlevel I/O - busmaster
 */

unsafe fn ptr_add(base: *mut c_void, offset: u32) -> *mut c_void {
    (base as *mut u8).add(offset as usize) as *mut c_void
}

unsafe fn igetbyte(chip: *mut intel8x0m, offset: u32) -> u8 {
    ioread8(ptr_add((*chip).bmaddr, offset))
}

unsafe fn igetword(chip: *mut intel8x0m, offset: u32) -> u16 {
    ioread16(ptr_add((*chip).bmaddr, offset))
}

unsafe fn igetdword(chip: *mut intel8x0m, offset: u32) -> u32 {
    ioread32(ptr_add((*chip).bmaddr, offset))
}

unsafe fn iputbyte(chip: *mut intel8x0m, offset: u32, val: u8) {
    iowrite8(val, ptr_add((*chip).bmaddr, offset));
}

unsafe fn iputword(chip: *mut intel8x0m, offset: u32, val: u16) {
    iowrite16(val, ptr_add((*chip).bmaddr, offset));
}

unsafe fn iputdword(chip: *mut intel8x0m, offset: u32, val: u32) {
    iowrite32(val, ptr_add((*chip).bmaddr, offset));
}

/*
 *  Lowlevel I/O - AC'97 registers
 */

unsafe fn iagetword(chip: *mut intel8x0m, offset: u32) -> u16 {
    ioread16(ptr_add((*chip).addr, offset))
}

unsafe fn iaputword(chip: *mut intel8x0m, offset: u32, val: u16) {
    iowrite16(val, ptr_add((*chip).addr, offset));
}

/*
 *  Basic I/O
 */

/*
 * access to AC97 codec via normal i/o (for ICH and SIS7013)
 */

/* return the GLOB_STA bit for the corresponding codec */
unsafe fn get_ich_codec_bit(_chip: *mut intel8x0m, codec: u32) -> u32 {
    static codec_bit: [u32; 3] = [ICH_PCR, ICH_SCR, ICH_TCR];
    if snd_BUG_ON(codec >= 3) {
        return ICH_PCR;
    }
    codec_bit[codec as usize]
}

unsafe fn snd_intel8x0m_codec_semaphore(chip: *mut intel8x0m, mut codec: u32) -> c_int {
    let mut time: c_int;

    if codec > 1 {
        return -EIO;
    }
    codec = get_ich_codec_bit(chip, codec);

    /* codec ready ? */
    if (igetdword(chip, ICH_REG_GLOB_STA) & codec) == 0 {
        return -EIO;
    }

    /* Anyone holding a semaphore for 1 msec should be shot... */
    time = 100;
    loop {
        if (igetbyte(chip, ICH_REG_ACC_SEMA) & ICH_CAS as u8) == 0 {
            return 0;
        }
        udelay(10);
        let old = time;
        time -= 1;
        if old == 0 {
            break;
        }
    }

    /* access to some forbidden (non existent) ac97 registers will not
     * reset the semaphore. So even if you don't get the semaphore, still
     * continue the access. We don't need the semaphore anyway. */
    dev_err(
        (*(*chip).card).dev,
        c"codec_semaphore: semaphore is not ready [0x%x][0x%x]\n".as_ptr(),
        igetbyte(chip, ICH_REG_ACC_SEMA) as c_int,
        igetdword(chip, ICH_REG_GLOB_STA),
    );
    iagetword(chip, 0); /* clear semaphore flag */
    /* I don't care about the semaphore */
    -EBUSY
}

unsafe extern "C" fn snd_intel8x0m_codec_write(ac97: *mut snd_ac97, reg: u16, val: u16) {
    let chip = (*ac97).private_data as *mut intel8x0m;

    if snd_intel8x0m_codec_semaphore(chip, (*ac97).num) < 0 {
        if (*chip).in_ac97_init == 0 {
            dev_err(
                (*(*chip).card).dev,
                c"codec_write %d: semaphore is not ready for register 0x%x\n".as_ptr(),
                (*ac97).num,
                reg as c_int,
            );
        }
    }
    iaputword(chip, reg as u32 + (*ac97).num * 0x80, val);
}

unsafe extern "C" fn snd_intel8x0m_codec_read(ac97: *mut snd_ac97, reg: u16) -> u16 {
    let chip = (*ac97).private_data as *mut intel8x0m;
    let mut res: u16;
    let tmp: u32;

    if snd_intel8x0m_codec_semaphore(chip, (*ac97).num) < 0 {
        if (*chip).in_ac97_init == 0 {
            dev_err(
                (*(*chip).card).dev,
                c"codec_read %d: semaphore is not ready for register 0x%x\n".as_ptr(),
                (*ac97).num,
                reg as c_int,
            );
        }
        res = 0xffff;
    } else {
        res = iagetword(chip, reg as u32 + (*ac97).num * 0x80);
        tmp = igetdword(chip, ICH_REG_GLOB_STA);
        if (tmp & ICH_RCS) != 0 {
            /* reset RCS and preserve other R/WC bits */
            iputdword(chip, ICH_REG_GLOB_STA, tmp & !(ICH_SRI | ICH_PRI | ICH_TRI | ICH_GSCI));
            if (*chip).in_ac97_init == 0 {
                dev_err(
                    (*(*chip).card).dev,
                    c"codec_read %d: read timeout for register 0x%x\n".as_ptr(),
                    (*ac97).num,
                    reg as c_int,
                );
            }
            res = 0xffff;
        }
    }
    if reg == AC97_GPIO_STATUS {
        iagetword(chip, 0); /* clear semaphore */
    }
    res
}

/*
 * DMA I/O
 */
unsafe fn cpu_to_le32(x: u32) -> __le32 {
    x.to_le()
}

unsafe fn snd_intel8x0m_setup_periods(chip: *mut intel8x0m, ichdev: *mut ichdev) {
    let mut idx: c_int;
    let bdbar = (*ichdev).bdbar;
    let port = (*ichdev).reg_offset as u32;

    iputdword(chip, port + ICH_REG_OFF_BDBAR, (*ichdev).bdbar_addr);
    if (*ichdev).size == (*ichdev).fragsize {
        (*ichdev).ack = 2;
        (*ichdev).ack_reload = (*ichdev).ack;
        (*ichdev).fragsize1 = (*ichdev).fragsize >> 1;
        idx = 0;
        while idx < ((ICH_REG_LVI_MASK + 1) * 2) as c_int {
            *bdbar.add((idx + 0) as usize) = cpu_to_le32((*ichdev).physbuf);
            *bdbar.add((idx + 1) as usize) =
                cpu_to_le32(0x80000000u32 | ((*ichdev).fragsize1 >> (*chip).pcm_pos_shift));
            *bdbar.add((idx + 2) as usize) =
                cpu_to_le32((*ichdev).physbuf + ((*ichdev).size >> 1));
            *bdbar.add((idx + 3) as usize) =
                cpu_to_le32(0x80000000u32 | ((*ichdev).fragsize1 >> (*chip).pcm_pos_shift));
            idx += 4;
        }
        (*ichdev).frags = 2;
    } else {
        (*ichdev).ack = 1;
        (*ichdev).ack_reload = (*ichdev).ack;
        (*ichdev).fragsize1 = (*ichdev).fragsize;
        idx = 0;
        while idx < ((ICH_REG_LVI_MASK + 1) * 2) as c_int {
            *bdbar.add((idx + 0) as usize) = cpu_to_le32(
                (*ichdev).physbuf
                    + ((((idx >> 1) as u32) * (*ichdev).fragsize) % (*ichdev).size),
            );
            *bdbar.add((idx + 1) as usize) =
                cpu_to_le32(0x80000000u32 | ((*ichdev).fragsize >> (*chip).pcm_pos_shift));
            /*
            dev_dbg(chip->card->dev, "bdbar[%i] = 0x%x [0x%x]\n",
                   idx + 0, bdbar[idx + 0], bdbar[idx + 1]);
            */
            idx += 2;
        }
        (*ichdev).frags = ((*ichdev).size / (*ichdev).fragsize) as c_int;
    }
    (*ichdev).lvi = ICH_REG_LVI_MASK as c_int;
    iputbyte(chip, port + ICH_REG_OFF_LVI, (*ichdev).lvi as u8);
    (*ichdev).civ = 0;
    iputbyte(chip, port + ICH_REG_OFF_CIV, 0);
    (*ichdev).lvi_frag = (ICH_REG_LVI_MASK as c_int) % (*ichdev).frags;
    (*ichdev).position = 0;
    /* #if 0 debug output omitted as disabled C code */
    /* clear interrupts */
    iputbyte(chip, port + (*ichdev).roff_sr, (ICH_FIFOE | ICH_BCIS | ICH_LVBCI) as u8);
}

/*
 *  Interrupt handler
 */

unsafe fn snd_intel8x0m_update(chip: *mut intel8x0m, ichdev: *mut ichdev) {
    let port = (*ichdev).reg_offset as u32;
    let civ: c_int;
    let mut i: c_int;
    let mut step: c_int;
    let mut ack: c_int = 0;

    civ = igetbyte(chip, port + ICH_REG_OFF_CIV) as c_int;
    if civ == (*ichdev).civ {
        step = 1;
        (*ichdev).civ += 1;
        (*ichdev).civ &= ICH_REG_LVI_MASK as c_int;
    } else {
        step = civ - (*ichdev).civ;
        if step < 0 {
            step += ICH_REG_LVI_MASK as c_int + 1;
        }
        (*ichdev).civ = civ;
    }

    (*ichdev).position = ((*ichdev).position + (step as u32) * (*ichdev).fragsize1) % (*ichdev).size;
    (*ichdev).lvi += step;
    (*ichdev).lvi &= ICH_REG_LVI_MASK as c_int;
    iputbyte(chip, port + ICH_REG_OFF_LVI, (*ichdev).lvi as u8);
    i = 0;
    while i < step {
        (*ichdev).lvi_frag += 1;
        (*ichdev).lvi_frag %= (*ichdev).frags;
        *(*ichdev).bdbar.add(((*ichdev).lvi * 2) as usize) = cpu_to_le32(
            (*ichdev).physbuf + ((*ichdev).lvi_frag as u32) * (*ichdev).fragsize1,
        );
        /* #if 0 debug output omitted as disabled C code */
        (*ichdev).ack -= 1;
        if (*ichdev).ack == 0 {
            (*ichdev).ack = (*ichdev).ack_reload;
            ack = 1;
        }
        i += 1;
    }
    if ack != 0 && !(*ichdev).substream.is_null() {
        spin_unlock(&mut (*chip).reg_lock);
        snd_pcm_period_elapsed((*ichdev).substream);
        spin_lock(&mut (*chip).reg_lock);
    }
    iputbyte(chip, port + (*ichdev).roff_sr, (ICH_FIFOE | ICH_BCIS | ICH_LVBCI) as u8);
}

unsafe extern "C" fn snd_intel8x0m_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let chip = dev_id as *mut intel8x0m;
    let mut ichdev: *mut ichdev;
    let status: u32;
    let mut i: u32;

    spin_lock(&mut (*chip).reg_lock);
    status = igetdword(chip, (*chip).int_sta_reg);
    if status == 0xffffffff {
        spin_unlock(&mut (*chip).reg_lock);
        return IRQ_NONE;
    }
    if (status & (*chip).int_sta_mask) == 0 {
        if status != 0 {
            iputdword(chip, (*chip).int_sta_reg, status);
        }
        spin_unlock(&mut (*chip).reg_lock);
        return IRQ_NONE;
    }

    i = 0;
    while i < (*chip).bdbars_count {
        ichdev = &mut (*chip).ichd[i as usize];
        if (status & (*ichdev).int_sta_mask) != 0 {
            snd_intel8x0m_update(chip, ichdev);
        }
        i += 1;
    }

    /* ack them */
    iputdword(chip, (*chip).int_sta_reg, status & (*chip).int_sta_mask);
    spin_unlock(&mut (*chip).reg_lock);

    IRQ_HANDLED
}

/*
 *  PCM part
 */

unsafe extern "C" fn snd_intel8x0m_pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let ichdev = get_ichdev(substream);
    let mut val: u8 = 0;
    let port = (*ichdev).reg_offset as u32;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
            val = (ICH_IOCE | ICH_STARTBM) as u8;
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
            val = 0;
        }
        SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            val = ICH_IOCE as u8;
        }
        SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            val = (ICH_IOCE | ICH_STARTBM) as u8;
        }
        _ => return -EINVAL,
    }
    iputbyte(chip, port + ICH_REG_OFF_CR, val);
    if cmd == SNDRV_PCM_TRIGGER_STOP {
        /* wait until DMA stopped */
        while (igetbyte(chip, port + (*ichdev).roff_sr) & ICH_DCH as u8) == 0 {}
        /* reset whole DMA things */
        iputbyte(chip, port + ICH_REG_OFF_CR, ICH_RESETREGS as u8);
    }
    0
}

unsafe extern "C" fn snd_intel8x0m_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream);
    let ichdev = get_ichdev(substream);
    let ptr1: size_t;
    let mut ptr_: size_t;

    ptr1 = ((igetword(chip, (*ichdev).reg_offset as u32 + (*ichdev).roff_picb) as size_t)
        << (*chip).pcm_pos_shift) as size_t;
    if ptr1 != 0 {
        ptr_ = (*ichdev).fragsize1 as size_t - ptr1;
    } else {
        ptr_ = 0;
    }
    ptr_ += (*ichdev).position as size_t;
    if ptr_ >= (*ichdev).size as size_t {
        return 0;
    }
    bytes_to_frames((*substream).runtime, ptr_)
}

unsafe extern "C" fn snd_intel8x0m_pcm_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let ichdev = get_ichdev(substream);

    (*ichdev).physbuf = (*runtime).dma_addr;
    (*ichdev).size = snd_pcm_lib_buffer_bytes(substream);
    (*ichdev).fragsize = snd_pcm_lib_period_bytes(substream);
    snd_ac97_write((*ichdev).ac97, AC97_LINE1_RATE, (*runtime).rate as u16);
    snd_ac97_write((*ichdev).ac97, AC97_LINE1_LEVEL, 0);
    snd_intel8x0m_setup_periods(chip, ichdev);
    0
}

static snd_intel8x0m_stream: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_RESUME,
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_KNOT,
    rate_min: 8000,
    rate_max: 16000,
    channels_min: 1,
    channels_max: 1,
    buffer_bytes_max: 64 * 1024,
    period_bytes_min: 32,
    period_bytes_max: 64 * 1024,
    periods_min: 1,
    periods_max: 1024,
    fifo_size: 0,
};

unsafe fn snd_intel8x0m_pcm_open(substream: *mut snd_pcm_substream, ichdev: *mut ichdev) -> c_int {
    static rates: [u32; 4] = [8000, 9600, 12000, 16000];
    static hw_constraints_rates: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
        count: 4,
        list: rates.as_ptr(),
        mask: 0,
    };
    let runtime = (*substream).runtime;
    let err: c_int;

    (*ichdev).substream = substream;
    (*runtime).hw = snd_intel8x0m_stream;
    err = snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &hw_constraints_rates);
    if err < 0 {
        return err;
    }
    (*runtime).private_data = ichdev as *mut c_void;
    0
}

unsafe extern "C" fn snd_intel8x0m_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    snd_intel8x0m_pcm_open(substream, &mut (*chip).ichd[ICHD_MDMOUT])
}

unsafe extern "C" fn snd_intel8x0m_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    (*chip).ichd[ICHD_MDMOUT].substream = ptr::null_mut();
    0
}

unsafe extern "C" fn snd_intel8x0m_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    snd_intel8x0m_pcm_open(substream, &mut (*chip).ichd[ICHD_MDMIN])
}

unsafe extern "C" fn snd_intel8x0m_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    (*chip).ichd[ICHD_MDMIN].substream = ptr::null_mut();
    0
}

static snd_intel8x0m_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_intel8x0m_playback_open),
    close: Some(snd_intel8x0m_playback_close),
    ioctl: None,
    hw_params: None,
    hw_free: None,
    prepare: Some(snd_intel8x0m_pcm_prepare),
    trigger: Some(snd_intel8x0m_pcm_trigger),
    pointer: Some(snd_intel8x0m_pcm_pointer),
};

static snd_intel8x0m_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_intel8x0m_capture_open),
    close: Some(snd_intel8x0m_capture_close),
    ioctl: None,
    hw_params: None,
    hw_free: None,
    prepare: Some(snd_intel8x0m_pcm_prepare),
    trigger: Some(snd_intel8x0m_pcm_trigger),
    pointer: Some(snd_intel8x0m_pcm_pointer),
};

#[repr(C)]
pub struct ich_pcm_table {
    pub suffix: *const c_char,
    pub playback_ops: *const snd_pcm_ops,
    pub capture_ops: *const snd_pcm_ops,
    pub prealloc_size: size_t,
    pub prealloc_max_size: size_t,
    pub ac97_idx: c_int,
}

unsafe fn snd_intel8x0m_pcm1(chip: *mut intel8x0m, device: c_int, rec: *const ich_pcm_table) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let mut err: c_int;
    let mut name: [c_char; 32] = [0; 32];

    if !(*rec).suffix.is_null() {
        sprintf(name.as_mut_ptr(), c"Intel ICH - %s".as_ptr(), (*rec).suffix);
    } else {
        strscpy(name.as_mut_ptr(), c"Intel ICH".as_ptr());
    }
    err = snd_pcm_new(
        (*chip).card,
        name.as_ptr(),
        device,
        if !(*rec).playback_ops.is_null() { 1 } else { 0 },
        if !(*rec).capture_ops.is_null() { 1 } else { 0 },
        &mut pcm,
    );
    if err < 0 {
        return err;
    }

    if !(*rec).playback_ops.is_null() {
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, (*rec).playback_ops);
    }
    if !(*rec).capture_ops.is_null() {
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, (*rec).capture_ops);
    }

    (*pcm).private_data = chip as *mut c_void;
    (*pcm).info_flags = 0;
    (*pcm).dev_class = SNDRV_PCM_CLASS_MODEM;
    if !(*rec).suffix.is_null() {
        sprintf((*pcm).name.as_mut_ptr(), c"%s - %s".as_ptr(), (*(*chip).card).shortname.as_ptr(), (*rec).suffix);
    } else {
        strscpy((*pcm).name.as_mut_ptr(), (*(*chip).card).shortname.as_ptr());
    }
    (*chip).pcm[device as usize] = pcm;

    snd_pcm_set_managed_buffer_all(
        pcm,
        SNDRV_DMA_TYPE_DEV,
        &mut (*(*chip).pci).dev,
        (*rec).prealloc_size,
        (*rec).prealloc_max_size,
    );

    0
}

static intel_pcms: [ich_pcm_table; 1] = [ich_pcm_table {
    suffix: c"Modem".as_ptr(),
    playback_ops: &snd_intel8x0m_playback_ops,
    capture_ops: &snd_intel8x0m_capture_ops,
    prealloc_size: 32 * 1024,
    prealloc_max_size: 64 * 1024,
    ac97_idx: 0,
}];

unsafe fn snd_intel8x0m_pcm(chip: *mut intel8x0m) -> c_int {
    let mut i: c_int;
    let tblsize: c_int;
    let mut device: c_int;
    let mut err: c_int;
    let tbl: *const ich_pcm_table;
    let mut rec: *const ich_pcm_table;

    /* #if 1 */
    tbl = intel_pcms.as_ptr();
    tblsize = 1;
    /* #else: nforce/ali/default alternate PCM tables omitted as disabled C code */
    device = 0;
    i = 0;
    while i < tblsize {
        rec = tbl.add(i as usize);
        if i > 0 && (*rec).ac97_idx != 0 {
            /* activate PCM only when associated AC'97 codec */
            if (*chip).ichd[(*rec).ac97_idx as usize].ac97.is_null() {
                i += 1;
                continue;
            }
        }
        err = snd_intel8x0m_pcm1(chip, device, rec);
        if err < 0 {
            return err;
        }
        device += 1;
        i += 1;
    }

    (*chip).pcm_devs = device;
    0
}

/*
 *  Mixer part
 */

unsafe extern "C" fn snd_intel8x0m_mixer_free_ac97_bus(bus: *mut snd_ac97_bus) {
    let chip = (*bus).private_data as *mut intel8x0m;
    (*chip).ac97_bus = ptr::null_mut();
}

unsafe extern "C" fn snd_intel8x0m_mixer_free_ac97(ac97: *mut snd_ac97) {
    let chip = (*ac97).private_data as *mut intel8x0m;
    (*chip).ac97 = ptr::null_mut();
}

unsafe fn snd_intel8x0m_mixer(chip: *mut intel8x0m, ac97_clock: c_int) -> c_int {
    let mut pbus: *mut snd_ac97_bus = ptr::null_mut();
    let mut ac97: snd_ac97_template = core::mem::zeroed();
    let mut x97: *mut snd_ac97 = ptr::null_mut();
    let mut err: c_int;
    let glob_sta: u32;
    static ops: snd_ac97_bus_ops = snd_ac97_bus_ops {
        write: Some(snd_intel8x0m_codec_write),
        read: Some(snd_intel8x0m_codec_read),
    };

    (*chip).in_ac97_init = 1;

    memset(&mut ac97 as *mut _ as *mut c_void, 0, size_of::<snd_ac97_template>());
    ac97.private_data = chip as *mut c_void;
    ac97.private_free = Some(snd_intel8x0m_mixer_free_ac97);
    ac97.scaps = AC97_SCAP_SKIP_AUDIO | AC97_SCAP_POWER_SAVE;

    glob_sta = igetdword(chip, ICH_REG_GLOB_STA);

    err = snd_ac97_bus((*chip).card, 0, &ops, chip as *mut c_void, &mut pbus);
    if err < 0 {
        /* clear the cold-reset bit for the next chance */
        if (*chip).device_type != DEVICE_ALI {
            iputdword(chip, ICH_REG_GLOB_CNT, igetdword(chip, ICH_REG_GLOB_CNT) & !ICH_AC97COLD);
        }
        return err;
    }
    (*pbus).private_free = Some(snd_intel8x0m_mixer_free_ac97_bus);
    if ac97_clock >= 8000 && ac97_clock <= 48000 {
        (*pbus).clock = ac97_clock;
    }
    (*chip).ac97_bus = pbus;

    ac97.pci = (*chip).pci;
    ac97.num = if (glob_sta & ICH_SCR) != 0 { 1 } else { 0 };
    err = snd_ac97_mixer(pbus, &mut ac97, &mut x97);
    if err < 0 {
        dev_err((*(*chip).card).dev, c"Unable to initialize codec #%d\n".as_ptr(), ac97.num);
        if ac97.num == 0 {
            /* clear the cold-reset bit for the next chance */
            if (*chip).device_type != DEVICE_ALI {
                iputdword(chip, ICH_REG_GLOB_CNT, igetdword(chip, ICH_REG_GLOB_CNT) & !ICH_AC97COLD);
            }
            return err;
        }
        return err;
    }
    (*chip).ac97 = x97;
    if ac97_is_modem(x97) && (*chip).ichd[ICHD_MDMIN].ac97.is_null() {
        (*chip).ichd[ICHD_MDMIN].ac97 = x97;
        (*chip).ichd[ICHD_MDMOUT].ac97 = x97;
    }

    (*chip).in_ac97_init = 0;
    0
}

/*
 *
 */

unsafe fn snd_intel8x0m_ich_chip_init(chip: *mut intel8x0m, probing: c_int) -> c_int {
    let mut end_time: c_ulong;
    let mut cnt: u32;
    let mut status: u32;
    let mut nstatus: u32;

    /* put logic to right state */
    /* first clear status bits */
    status = ICH_RCS | ICH_MIINT | ICH_MOINT;
    cnt = igetdword(chip, ICH_REG_GLOB_STA);
    iputdword(chip, ICH_REG_GLOB_STA, cnt & status);

    /* ACLink on, 2 channels */
    cnt = igetdword(chip, ICH_REG_GLOB_CNT);
    cnt &= !ICH_ACLINK;
    /* finish cold or do warm reset */
    cnt |= if (cnt & ICH_AC97COLD) == 0 { ICH_AC97COLD } else { ICH_AC97WARM };
    iputdword(chip, ICH_REG_GLOB_CNT, cnt);
    usleep_range(500, 1000); /* give warm reset some time */
    end_time = jiffies + HZ / 4;
    loop {
        if (igetdword(chip, ICH_REG_GLOB_CNT) & ICH_AC97WARM) == 0 {
            break;
        }
        schedule_timeout_uninterruptible(1);
        if !time_after_eq(end_time, jiffies) {
            dev_err(
                (*(*chip).card).dev,
                c"AC'97 warm reset still in progress? [0x%x]\n".as_ptr(),
                igetdword(chip, ICH_REG_GLOB_CNT),
            );
            return -EIO;
        }
    }

    if probing != 0 {
        /* wait for any codec ready status.
         * Once it becomes ready it should remain ready
         * as long as we do not disable the ac97 link.
         */
        end_time = jiffies + HZ;
        loop {
            status = igetdword(chip, ICH_REG_GLOB_STA) & (ICH_PCR | ICH_SCR | ICH_TCR);
            if status != 0 {
                break;
            }
            schedule_timeout_uninterruptible(1);
            if !time_after_eq(end_time, jiffies) {
                break;
            }
        }
        if status == 0 {
            /* no codec is found */
            dev_err(
                (*(*chip).card).dev,
                c"codec_ready: codec is not ready [0x%x]\n".as_ptr(),
                igetdword(chip, ICH_REG_GLOB_STA),
            );
            return -EIO;
        }

        /* up to two codecs (modem cannot be tertiary with ICH4) */
        nstatus = ICH_PCR | ICH_SCR;

        /* wait for other codecs ready status. */
        end_time = jiffies + HZ / 4;
        while status != nstatus && time_after_eq(end_time, jiffies) {
            schedule_timeout_uninterruptible(1);
            status |= igetdword(chip, ICH_REG_GLOB_STA) & nstatus;
        }
    } else {
        /* resume phase */
        status = 0;
        if !(*chip).ac97.is_null() {
            status |= get_ich_codec_bit(chip, (*(*chip).ac97).num);
        }
        /* wait until all the probed codecs are ready */
        end_time = jiffies + HZ;
        loop {
            nstatus = igetdword(chip, ICH_REG_GLOB_STA) & (ICH_PCR | ICH_SCR | ICH_TCR);
            if status == nstatus {
                break;
            }
            schedule_timeout_uninterruptible(1);
            if !time_after_eq(end_time, jiffies) {
                break;
            }
        }
    }

    if (*chip).device_type == DEVICE_SIS {
        /* unmute the output on SIS7013 */
        iputword(chip, 0x4c, igetword(chip, 0x4c) | 1);
    }

    0
}

unsafe fn snd_intel8x0m_chip_init(chip: *mut intel8x0m, probing: c_int) -> c_int {
    let mut i: u32;
    let err: c_int;

    err = snd_intel8x0m_ich_chip_init(chip, probing);
    if err < 0 {
        return err;
    }
    iagetword(chip, 0); /* clear semaphore flag */

    /* disable interrupts */
    i = 0;
    while i < (*chip).bdbars_count {
        iputbyte(chip, ICH_REG_OFF_CR + (*chip).ichd[i as usize].reg_offset as u32, 0x00);
        i += 1;
    }
    /* reset channels */
    i = 0;
    while i < (*chip).bdbars_count {
        iputbyte(chip, ICH_REG_OFF_CR + (*chip).ichd[i as usize].reg_offset as u32, ICH_RESETREGS as u8);
        i += 1;
    }
    /* initialize Buffer Descriptor Lists */
    i = 0;
    while i < (*chip).bdbars_count {
        iputdword(
            chip,
            ICH_REG_OFF_BDBAR + (*chip).ichd[i as usize].reg_offset as u32,
            (*chip).ichd[i as usize].bdbar_addr,
        );
        i += 1;
    }
    0
}

unsafe extern "C" fn snd_intel8x0m_free(card: *mut snd_card) {
    let chip = (*card).private_data as *mut intel8x0m;
    let mut i: u32;

    if (*chip).irq < 0 {
        if (*chip).irq >= 0 {
            free_irq((*chip).irq, chip as *mut c_void);
        }
        return;
    }
    /* disable interrupts */
    i = 0;
    while i < (*chip).bdbars_count {
        iputbyte(chip, ICH_REG_OFF_CR + (*chip).ichd[i as usize].reg_offset as u32, 0x00);
        i += 1;
    }
    /* reset channels */
    i = 0;
    while i < (*chip).bdbars_count {
        iputbyte(chip, ICH_REG_OFF_CR + (*chip).ichd[i as usize].reg_offset as u32, ICH_RESETREGS as u8);
        i += 1;
    }
    if (*chip).irq >= 0 {
        free_irq((*chip).irq, chip as *mut c_void);
    }
}

/*
 * power management
 */
unsafe extern "C" fn intel8x0m_suspend(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    let chip = (*card).private_data as *mut intel8x0m;

    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    snd_ac97_suspend((*chip).ac97);
    if (*chip).irq >= 0 {
        free_irq((*chip).irq, chip as *mut c_void);
        (*chip).irq = -1;
        (*card).sync_irq = -1;
    }
    0
}

unsafe extern "C" fn intel8x0m_resume(dev: *mut device) -> c_int {
    let pci = to_pci_dev(dev);
    let card = dev_get_drvdata(dev) as *mut snd_card;
    let chip = (*card).private_data as *mut intel8x0m;

    if request_irq((*pci).irq, snd_intel8x0m_interrupt, IRQF_SHARED, KBUILD_MODNAME, chip as *mut c_void) != 0 {
        dev_err(dev, c"unable to grab IRQ %d, disabling device\n".as_ptr(), (*pci).irq);
        snd_card_disconnect(card);
        return -EIO;
    }
    (*chip).irq = (*pci).irq;
    (*card).sync_irq = (*chip).irq;
    snd_intel8x0m_chip_init(chip, 0);
    snd_ac97_resume((*chip).ac97);

    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}

// DEFINE_SIMPLE_DEV_PM_OPS(intel8x0m_pm, intel8x0m_suspend, intel8x0m_resume);
static intel8x0m_pm: dev_pm_ops = dev_pm_ops { _private: [] };

unsafe extern "C" fn snd_intel8x0m_proc_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let chip = (*entry).private_data as *mut intel8x0m;
    let tmp: u32;

    snd_iprintf(buffer, c"Intel8x0m\n\n".as_ptr());
    if (*chip).device_type == DEVICE_ALI {
        return;
    }
    tmp = igetdword(chip, ICH_REG_GLOB_STA);
    snd_iprintf(buffer, c"Global control        : 0x%08x\n".as_ptr(), igetdword(chip, ICH_REG_GLOB_CNT));
    snd_iprintf(buffer, c"Global status         : 0x%08x\n".as_ptr(), tmp);
    snd_iprintf(
        buffer,
        c"AC'97 codecs ready    :%s%s%s%s\n".as_ptr(),
        if (tmp & ICH_PCR) != 0 { c" primary".as_ptr() } else { c"".as_ptr() },
        if (tmp & ICH_SCR) != 0 { c" secondary".as_ptr() } else { c"".as_ptr() },
        if (tmp & ICH_TCR) != 0 { c" tertiary".as_ptr() } else { c"".as_ptr() },
        if (tmp & (ICH_PCR | ICH_SCR | ICH_TCR)) == 0 { c" none".as_ptr() } else { c"".as_ptr() },
    );
}

unsafe fn snd_intel8x0m_proc_init(chip: *mut intel8x0m) {
    snd_card_ro_proc_new((*chip).card, c"intel8x0m".as_ptr(), chip as *mut c_void, snd_intel8x0m_proc_read);
}

#[repr(C)]
pub struct ich_reg_info {
    pub int_sta_mask: u32,
    pub offset: u32,
}

unsafe fn snd_intel8x0m_init(card: *mut snd_card, pci: *mut pci_dev, device_type: c_ulong) -> c_int {
    let chip = (*card).private_data as *mut intel8x0m;
    let mut err: c_int;
    let mut i: u32;
    let mut int_sta_masks: u32;
    let mut ichdev: *mut ichdev;
    static intel_regs: [ich_reg_info; 2] = [
        ich_reg_info { int_sta_mask: ICH_MIINT, offset: 0 },
        ich_reg_info { int_sta_mask: ICH_MOINT, offset: 0x10 },
    ];
    let tbl: *const ich_reg_info;

    err = pcim_enable_device(pci);
    if err < 0 {
        return err;
    }

    spin_lock_init(&mut (*chip).reg_lock);
    (*chip).device_type = device_type as u32;
    (*chip).card = card;
    (*chip).pci = pci;
    (*chip).irq = -1;

    err = pcim_request_all_regions(pci, (*card).shortname.as_ptr());
    if err < 0 {
        return err;
    }

    if device_type as u32 == DEVICE_ALI {
        /* ALI5455 has no ac97 region */
        (*chip).bmaddr = pcim_iomap(pci, 0, 0);
    } else {
        if (pci_resource_flags(pci, 2) & IORESOURCE_MEM) != 0 {
            /* ICH4 and Nforce */
            (*chip).addr = pcim_iomap(pci, 2, 0);
        } else {
            (*chip).addr = pcim_iomap(pci, 0, 0);
        }
        if (pci_resource_flags(pci, 3) & IORESOURCE_MEM) != 0 {
            /* ICH4 */
            (*chip).bmaddr = pcim_iomap(pci, 3, 0);
        } else {
            (*chip).bmaddr = pcim_iomap(pci, 1, 0);
        }
    }

    /* initialize offsets */
    (*chip).bdbars_count = 2;
    tbl = intel_regs.as_ptr();

    i = 0;
    while i < (*chip).bdbars_count {
        ichdev = &mut (*chip).ichd[i as usize];
        (*ichdev).ichd = i;
        (*ichdev).reg_offset = (*tbl.add(i as usize)).offset as c_ulong;
        (*ichdev).int_sta_mask = (*tbl.add(i as usize)).int_sta_mask;
        if device_type as u32 == DEVICE_SIS {
            /* SiS 7013 swaps the registers */
            (*ichdev).roff_sr = ICH_REG_OFF_PICB;
            (*ichdev).roff_picb = ICH_REG_OFF_SR;
        } else {
            (*ichdev).roff_sr = ICH_REG_OFF_SR;
            (*ichdev).roff_picb = ICH_REG_OFF_PICB;
        }
        if device_type as u32 == DEVICE_ALI {
            (*ichdev).ali_slot = (((*ichdev).reg_offset - 0x40) / 0x10) as u32;
        }
        i += 1;
    }
    /* SIS7013 handles the pcm data in bytes, others are in words */
    (*chip).pcm_pos_shift = if device_type as u32 == DEVICE_SIS { 0 } else { 1 };

    /* allocate buffer descriptor lists */
    /* the start of each lists must be aligned to 8 bytes */
    (*chip).bdbars = snd_devm_alloc_pages(
        &mut (*pci).dev,
        SNDRV_DMA_TYPE_DEV,
        ((*chip).bdbars_count as size_t) * size_of::<u32>() * (ICH_MAX_FRAGS as size_t) * 2,
    );
    if (*chip).bdbars.is_null() {
        return -ENOMEM;
    }

    /* tables must be aligned to 8 bytes here, but the kernel pages
       are much bigger, so we don't care (on i386) */
    int_sta_masks = 0;
    i = 0;
    while i < (*chip).bdbars_count {
        ichdev = &mut (*chip).ichd[i as usize];
        (*ichdev).bdbar = ((*(*chip).bdbars).area as *mut __le32).add((i * ICH_MAX_FRAGS * 2) as usize);
        (*ichdev).bdbar_addr = (*(*chip).bdbars).addr + (i * size_of::<u32>() as u32 * ICH_MAX_FRAGS * 2);
        int_sta_masks |= (*ichdev).int_sta_mask;
        i += 1;
    }
    (*chip).int_sta_reg = ICH_REG_GLOB_STA;
    (*chip).int_sta_mask = int_sta_masks;

    pci_set_master(pci);

    err = snd_intel8x0m_chip_init(chip, 1);
    if err < 0 {
        return err;
    }

    /* NOTE: we don't use devm version here since it's released /
     * re-acquired in PM callbacks.
     * It's released explicitly in snd_intel8x0m_free(), too.
     */
    if request_irq((*pci).irq, snd_intel8x0m_interrupt, IRQF_SHARED, KBUILD_MODNAME, chip as *mut c_void) != 0 {
        dev_err((*card).dev, c"unable to grab IRQ %d\n".as_ptr(), (*pci).irq);
        return -EBUSY;
    }
    (*chip).irq = (*pci).irq;
    (*card).sync_irq = (*chip).irq;

    (*card).private_free = Some(snd_intel8x0m_free);

    0
}

#[repr(C)]
pub struct shortname_table {
    pub id: u32,
    pub s: *const c_char,
}

static mut shortnames: [shortname_table; 17] = [
    shortname_table { id: PCI_DEVICE_ID_INTEL_82801AA_6, s: c"Intel 82801AA-ICH".as_ptr() },
    shortname_table { id: PCI_DEVICE_ID_INTEL_82801AB_6, s: c"Intel 82901AB-ICH0".as_ptr() },
    shortname_table { id: PCI_DEVICE_ID_INTEL_82801BA_6, s: c"Intel 82801BA-ICH2".as_ptr() },
    shortname_table { id: PCI_DEVICE_ID_INTEL_440MX_6, s: c"Intel 440MX".as_ptr() },
    shortname_table { id: PCI_DEVICE_ID_INTEL_82801CA_6, s: c"Intel 82801CA-ICH3".as_ptr() },
    shortname_table { id: PCI_DEVICE_ID_INTEL_82801DB_6, s: c"Intel 82801DB-ICH4".as_ptr() },
    shortname_table { id: PCI_DEVICE_ID_INTEL_82801EB_6, s: c"Intel ICH5".as_ptr() },
    shortname_table { id: PCI_DEVICE_ID_INTEL_ICH6_17, s: c"Intel ICH6".as_ptr() },
    shortname_table { id: PCI_DEVICE_ID_INTEL_ICH7_19, s: c"Intel ICH7".as_ptr() },
    shortname_table { id: 0x7446, s: c"AMD AMD768".as_ptr() },
    shortname_table { id: PCI_DEVICE_ID_SI_7013, s: c"SiS SI7013".as_ptr() },
    shortname_table { id: PCI_DEVICE_ID_NVIDIA_MCP1_MODEM, s: c"NVidia nForce".as_ptr() },
    shortname_table { id: PCI_DEVICE_ID_NVIDIA_MCP2_MODEM, s: c"NVidia nForce2".as_ptr() },
    shortname_table { id: PCI_DEVICE_ID_NVIDIA_MCP2S_MODEM, s: c"NVidia nForce2s".as_ptr() },
    shortname_table { id: PCI_DEVICE_ID_NVIDIA_MCP3_MODEM, s: c"NVidia nForce3".as_ptr() },
    shortname_table { id: 0x746e, s: c"AMD AMD8111".as_ptr() },
    /* #if 0: { 0x5455, "ALi M5455" } disabled in C */
    shortname_table { id: 0, s: ptr::null() },
];

unsafe fn __snd_intel8x0m_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let chip: *mut intel8x0m;
    let mut err: c_int;
    let mut name: *mut shortname_table;

    err = snd_devm_card_new(&mut (*pci).dev, index, id, THIS_MODULE, size_of::<intel8x0m>(), &mut card);
    if err < 0 {
        return err;
    }
    chip = (*card).private_data as *mut intel8x0m;

    strscpy((*card).driver.as_mut_ptr(), c"ICH-MODEM".as_ptr());
    strscpy((*card).shortname.as_mut_ptr(), c"Intel ICH".as_ptr());
    name = shortnames.as_mut_ptr();
    while (*name).id != 0 {
        if (*pci).device == (*name).id {
            strscpy((*card).shortname.as_mut_ptr(), (*name).s);
            break;
        }
        name = name.add(1);
    }
    strcat((*card).shortname.as_mut_ptr(), c" Modem".as_ptr());

    err = snd_intel8x0m_init(card, pci, (*pci_id).driver_data);
    if err < 0 {
        return err;
    }

    err = snd_intel8x0m_mixer(chip, ac97_clock);
    if err < 0 {
        return err;
    }
    err = snd_intel8x0m_pcm(chip);
    if err < 0 {
        return err;
    }

    snd_intel8x0m_proc_init(chip);

    sprintf((*card).longname.as_mut_ptr(), c"%s at irq %i".as_ptr(), (*card).shortname.as_ptr(), (*chip).irq);

    err = snd_card_register(card);
    if err < 0 {
        return err;
    }
    pci_set_drvdata(pci, card as *mut c_void);
    0
}

unsafe extern "C" fn snd_intel8x0m_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    snd_card_free_on_error(&mut (*pci).dev, __snd_intel8x0m_probe(pci, pci_id))
}

static mut intel8x0m_driver: pci_driver = pci_driver {
    name: ptr::null(),
    id_table: snd_intel8x0m_ids.as_ptr(),
    probe: Some(snd_intel8x0m_probe),
    driver: pci_driver_driver {
        pm: &intel8x0m_pm,
    },
};

// module_pci_driver(intel8x0m_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
