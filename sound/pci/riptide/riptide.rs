// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Driver for the Conexant Riptide Soundchip
 *
 *	Copyright (c) 2004 Peter Gruber <nokos@gmx.net>
 */
/*
  History:
   - 02/15/2004 first release

  This Driver is based on the OSS Driver version from Linuxant (riptide-0.6lnxtbeta03111100)
  credits from the original files:

  MODULE NAME:        cnxt_rt.h
  AUTHOR:             K. Lazarev  (Transcribed by KNL)
  HISTORY:         Major Revision               Date        By
            -----------------------------     --------     -----
            Created                           02/1/2000     KNL

  MODULE NAME:     int_mdl.c
  AUTHOR:          Konstantin Lazarev    (Transcribed by KNL)
  HISTORY:         Major Revision               Date        By
            -----------------------------     --------     -----
            Created                           10/01/99      KNL

  MODULE NAME:        riptide.h
  AUTHOR:             O. Druzhinin  (Transcribed by OLD)
  HISTORY:         Major Revision               Date        By
            -----------------------------     --------     -----
            Created                           10/16/97      OLD

  MODULE NAME:        Rp_Cmdif.cpp
  AUTHOR:             O. Druzhinin  (Transcribed by OLD)
                      K. Lazarev    (Transcribed by KNL)
  HISTORY:         Major Revision               Date        By
            -----------------------------     --------     -----
            Adopted from NT4 driver            6/22/99      OLD
            Ported to Linux                    9/01/99      KNL

  MODULE NAME:        rt_hw.c
  AUTHOR:             O. Druzhinin  (Transcribed by OLD)
                      C. Lazarev    (Transcribed by CNL)
  HISTORY:         Major Revision               Date        By
            -----------------------------     --------     -----
            Created                           11/18/97      OLD
            Hardware functions for RipTide    11/24/97      CNL
            (ES1) are coded
            Hardware functions for RipTide    12/24/97      CNL
            (A0) are coded
            Hardware functions for RipTide    03/20/98      CNL
            (A1) are coded
            Boot loader is included           05/07/98      CNL
            Redesigned for WDM                07/27/98      CNL
            Redesigned for Linux              09/01/99      CNL

  MODULE NAME:        rt_hw.h
  AUTHOR:             C. Lazarev    (Transcribed by CNL)
  HISTORY:         Major Revision               Date        By
            -----------------------------     --------     -----
            Created                           11/18/97      CNL

  MODULE NAME:     rt_mdl.c
  AUTHOR:          Konstantin Lazarev    (Transcribed by KNL)
  HISTORY:         Major Revision               Date        By
            -----------------------------     --------     -----
            Created                           10/01/99      KNL

  MODULE NAME:        mixer.h
  AUTHOR:             K. Kenney
  HISTORY:         Major Revision                   Date          By
            -----------------------------          --------     -----
            Created from MS W95 Sample             11/28/95      KRS
            RipTide                                10/15/97      KRS
            Adopted for Windows NT driver          01/20/98      CNL
*/

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ushort, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type u8 = ::core::primitive::u8;
type u16 = ::core::primitive::u16;
type u32 = ::core::primitive::u32;
type __le32 = u32;
type bool_ = bool;
type snd_pcm_format_t = c_int;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_int;
type dma_addr_t = c_ulong;
type size_t = usize;
type spinlock_t = c_ulong;

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct pci_dev { pub dev: device, pub irq: c_int, pub device: c_ushort }
#[repr(C)] pub struct firmware { pub size: c_uint, pub data: *const u8 }
#[repr(C)] pub struct resource { _private: [u8; 0] }
#[repr(C)] pub struct snd_info_entry { pub private_data: *mut c_void }
#[repr(C)] pub struct snd_info_buffer { _private: [u8; 0] }
#[repr(C)] pub struct snd_rawmidi { pub private_data: *mut c_void }
#[repr(C)] pub struct snd_opl3 { _private: [u8; 0] }
#[repr(C)] pub struct snd_ac97 { pub private_data: *mut c_void, pub scaps: c_uint, pub pci: *mut pci_dev }
#[repr(C)] pub struct snd_ac97_bus { _private: [u8; 0] }
#[repr(C)] pub struct snd_ac97_bus_ops {
    pub write: Option<unsafe extern "C" fn(*mut snd_ac97, c_ushort, c_ushort)>,
    pub read: Option<unsafe extern "C" fn(*mut snd_ac97, c_ushort) -> c_ushort>,
}
#[repr(C)] pub struct snd_ac97_template {
    pub private_data: *mut c_void,
    pub scaps: c_uint,
    pub pci: *mut pci_dev,
}
#[repr(C)] pub struct snd_dma_buffer { pub area: *mut c_void, pub addr: dma_addr_t, pub bytes: size_t }
#[repr(C)] pub struct snd_pcm_hardware {
    pub info: c_uint, pub formats: c_ulong, pub rates: c_uint,
    pub rate_min: c_uint, pub rate_max: c_uint,
    pub channels_min: c_uint, pub channels_max: c_uint,
    pub buffer_bytes_max: size_t, pub period_bytes_min: size_t,
    pub period_bytes_max: size_t, pub periods_min: c_uint,
    pub periods_max: c_uint, pub fifo_size: size_t,
}
#[repr(C)] pub struct snd_pcm_runtime {
    pub private_data: *mut c_void, pub hw: snd_pcm_hardware,
    pub period_size: snd_pcm_uframes_t, pub buffer_size: snd_pcm_uframes_t,
    pub channels: c_uint, pub format: snd_pcm_format_t, pub rate: c_uint,
}
#[repr(C)] pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub number: c_int,
}
#[repr(C)] pub struct snd_pcm { pub private_data: *mut c_void, pub info_flags: c_uint, pub name: [c_char; 80] }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}
#[repr(C)] pub struct snd_card {
    pub dev: *mut device, pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    pub sync_irq: c_int, pub driver: [c_char; 16],
    pub shortname: [c_char; 32], pub longname: [c_char; 80],
}
#[repr(C)] pub struct pci_device_id { pub vendor: u32, pub device: u32, pub subvendor: u32, pub subdevice: u32, pub class: u32, pub class_mask: u32, pub driver_data: c_ulong }
#[repr(C)] pub struct dev_pm_ops { _private: [u8; 0] }
#[repr(C)] pub struct pci_driver {
    pub name: *const c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut pci_dev)>,
    pub driver: pci_driver_inner,
}
#[repr(C)] pub struct pci_driver_inner { pub pm: *const dev_pm_ops }
#[repr(C)] pub struct gameport { pub io: c_int }

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static KBUILD_MODNAME: *const c_char;
    fn inl(port: c_ulong) -> u32;
    fn outl(value: u32, port: c_ulong);
    fn udelay(usecs: c_uint);
    fn hex_to_bin(ch: u8) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn snd_BUG_ON(cond: bool) -> c_int;
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn le32_to_cpu(x: __le32) -> u32;
    fn cpu_to_le32(x: u32) -> __le32;
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, frames: snd_pcm_uframes_t) -> c_ulong;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: c_ulong) -> snd_pcm_uframes_t;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut c_void;
    fn snd_pcm_format_width(format: snd_pcm_format_t) -> c_int;
    fn snd_pcm_format_unsigned(format: snd_pcm_format_t) -> c_int;
    fn snd_pcm_format_big_endian(format: snd_pcm_format_t) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn snd_ac97_suspend(ac97: *mut snd_ac97);
    fn snd_ac97_resume(ac97: *mut snd_ac97);
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, dev: *mut device) -> c_int;
    fn release_firmware(fw: *const firmware);
    fn memcmp(a: *const c_void, b: *const c_void, n: size_t) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn snd_dma_free_pages(buf: *mut snd_dma_buffer);
    fn snd_dma_alloc_pages(t: c_int, dev: *mut device, size: size_t, buf: *mut snd_dma_buffer) -> c_int;
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, param: c_int) -> c_int;
    fn snd_pcm_sgbuf_get_addr(substream: *mut snd_pcm_substream, ofs: c_uint) -> c_uint;
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback: c_int, capture: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: c_int, ops: *const snd_pcm_ops);
    fn strscpy(dst: *mut c_char, src: *const c_char) -> size_t;
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, t: c_int, dev: *mut device, min: size_t, max: size_t);
    fn snd_mpu401_uart_interrupt(irq: c_int, private_data: *mut c_void);
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn pcim_request_all_regions(pci: *mut pci_dev, name: *const c_char) -> c_int;
    fn devm_request_threaded_irq(dev: *mut device, irq: c_int, handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>, thread_fn: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>, flags: c_ulong, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn pci_set_master(pci: *mut pci_dev);
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_card_ro_proc_new(card: *mut snd_card, name: *const c_char, data: *mut c_void, read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>);
    fn snd_ac97_bus(card: *mut snd_card, num: c_int, ops: *const snd_ac97_bus_ops, private_data: *mut c_void, rbus: *mut *mut snd_ac97_bus) -> c_int;
    fn snd_ac97_mixer(bus: *mut snd_ac97_bus, template: *mut snd_ac97_template, rac97: *mut *mut snd_ac97) -> c_int;
    fn gameport_allocate_port() -> *mut gameport;
    fn gameport_free_port(gameport: *mut gameport);
    fn request_region(start: c_int, n: c_int, name: *const c_char) -> *mut resource;
    fn release_region(start: c_int, n: c_int);
    fn gameport_register_port(gameport: *mut gameport);
    fn gameport_unregister_port(gameport: *mut gameport);
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn pci_get_drvdata(pci: *mut pci_dev) -> *mut c_void;
    fn snd_devm_card_new(dev: *mut device, idx: c_int, id: *mut c_char, module: *mut c_void, extra_size: size_t, card_ret: *mut *mut snd_card) -> c_int;
    fn pci_write_config_word(pci: *mut pci_dev, where_: c_int, val: c_ushort) -> c_int;
    fn snd_mpu401_uart_new(card: *mut snd_card, device: c_int, hardware: c_int, port: c_ushort, info_flags: c_uint, irq: c_int, rrawmidi: *mut *mut snd_rawmidi) -> c_int;
    fn snd_opl3_create(card: *mut snd_card, l_port: c_ushort, r_port: c_ushort, hardware: c_int, integrated: c_int, opl3: *mut *mut snd_opl3) -> c_int;
    fn snd_opl3_hwdep_new(opl3: *mut snd_opl3, device: c_int, seq_device: c_int, rinfo: *mut c_void) -> c_int;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_card_free_on_error(dev: *mut device, err: c_int) -> c_int;
    fn pci_register_driver(driver: *mut pci_driver) -> c_int;
    fn pci_unregister_driver(driver: *mut pci_driver);
}

const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_IDX: c_int = -1;
const SNDRV_DEFAULT_ENABLE: bool = true;
static mut index: [c_int; SNDRV_CARDS] = [SNDRV_DEFAULT_IDX; SNDRV_CARDS];
static mut id: [*mut c_char; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS];
static mut enable: [bool; SNDRV_CARDS] = [SNDRV_DEFAULT_ENABLE; SNDRV_CARDS];

// #if IS_REACHABLE(CONFIG_GAMEPORT)
const SUPPORT_JOYSTICK: bool = false;
static mut joystick_port: [c_int; SNDRV_CARDS] = [0x200; SNDRV_CARDS];
static mut mpu_port: [c_int; SNDRV_CARDS] = [0x330; SNDRV_CARDS];
static mut opl3_port: [c_int; SNDRV_CARDS] = [0x388; SNDRV_CARDS];

const MPU401_HW_MPU401: c_int = 0;
const OPL3_HW_OPL3: c_int = 0;
const MPU401_HW_RIPTIDE: c_int = MPU401_HW_MPU401;
const OPL3_HW_RIPTIDE: c_int = OPL3_HW_OPL3;

const PCI_EXT_CapId: c_int = 0x40;
const PCI_EXT_NextCapPrt: c_int = 0x41;
const PCI_EXT_PWMC: c_int = 0x42;
const PCI_EXT_PWSCR: c_int = 0x44;
const PCI_EXT_Data00: c_int = 0x46;
const PCI_EXT_PMSCR_BSE: c_int = 0x47;
const PCI_EXT_SB_Base: c_int = 0x48;
const PCI_EXT_FM_Base: c_int = 0x4a;
const PCI_EXT_MPU_Base: c_int = 0x4C;
const PCI_EXT_Game_Base: c_int = 0x4E;
const PCI_EXT_Legacy_Mask: c_int = 0x50;
const PCI_EXT_AsicRev: c_int = 0x52;
const PCI_EXT_Reserved3: c_int = 0x53;

const LEGACY_ENABLE_ALL: c_ushort = 0x8000;
const LEGACY_ENABLE_SB: c_ushort = 0x4000;
const LEGACY_ENABLE_FM: c_ushort = 0x2000;
const LEGACY_ENABLE_MPU_INT: c_ushort = 0x1000;
const LEGACY_ENABLE_MPU: c_ushort = 0x0800;
const LEGACY_ENABLE_GAMEPORT: c_ushort = 0x0400;
const MAX_WRITE_RETRY: c_uint = 10;
const MAX_ERROR_COUNT: c_uint = 10;
const CMDIF_TIMEOUT: c_uint = 50000;
const RESET_TRIES: c_int = 5;
const RESP: u32 = 0x00000001;
const PARM: u32 = 0x00000002;
const CMDA: u32 = 0x00000004;
const CMDB: u32 = 0x00000008;
const NILL: u32 = 0x00000000;

const EOB_STATUS: u32 = 0x80000000;
const EOS_STATUS: u32 = 0x40000000;
const EOC_STATUS: u32 = 0x20000000;
const ERR_STATUS: u32 = 0x10000000;
const EMPTY_STATUS: u32 = 0x08000000;
const IEOB_ENABLE: u32 = 0x1;
const IEOS_ENABLE: u32 = 0x2;
const IEOC_ENABLE: u32 = 0x4;
const RDONCE: u32 = 0x8;
const DESC_MAX_MASK: usize = 0xff;
const ST_PLAY: u8 = 0x1;
const ST_STOP: u8 = 0x2;
const ST_PAUSE: u8 = 0x4;
const I2S_INTDEC: u8 = 3;
const I2S_MERGER: u8 = 0;
const I2S_SPLITTER: u8 = 0;
const I2S_MIXER: u8 = 7;
const I2S_RATE: u32 = 44100;
const MODEM_INTDEC: u8 = 4;
const MODEM_MERGER: u8 = 3;
const MODEM_SPLITTER: u8 = 0;
const MODEM_MIXER: u8 = 11;
const FM_INTDEC: u8 = 3;
const FM_MERGER: u8 = 0;
const FM_SPLITTER: u8 = 0;
const FM_MIXER: u8 = 9;
const SPLIT_PATH: u8 = 0x80;

const EACCES: c_int = 13; const EINVAL: c_int = 22; const EIO: c_int = 5;
const EBUSY: c_int = 16; const ENOSYS: c_int = 38; const ENODATA: c_int = 61;
const ENOMEM: c_int = 12; const ENODEV: c_int = 19; const ENOENT: c_int = 2;
const IRQ_HANDLED: irqreturn_t = 1; const IRQ_WAKE_THREAD: irqreturn_t = 2;
const IRQF_SHARED: c_ulong = 0x80;
const PAGE_SIZE: usize = 4096; const PAGE_SHIFT: c_uint = 12;
const GFP_KERNEL: c_uint = 0;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 1;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint = 1 << 2;
const SNDRV_PCM_INFO_PAUSE: c_uint = 1 << 3;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 4;
const SNDRV_PCM_FMTBIT_U8: c_ulong = 1 << 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 1 << 1;
const SNDRV_PCM_FMTBIT_S8: c_ulong = 1 << 2;
const SNDRV_PCM_FMTBIT_U16_LE: c_ulong = 1 << 3;
const SNDRV_PCM_RATE_KNOT: c_uint = 1 << 0;
const SNDRV_PCM_RATE_8000_48000: c_uint = 1 << 1;
const SNDRV_PCM_FORMAT_S16_LE: snd_pcm_format_t = 2;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_PCM_HW_PARAM_PERIODS: c_int = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const SNDRV_DMA_TYPE_DEV_SG: c_int = 1;
const SNDRV_CTL_POWER_D3hot: c_int = 3;
const SNDRV_CTL_POWER_D0: c_int = 0;
const AC97_RESET: u16 = 0;
const AC97_SCAP_SKIP_MODEM: c_uint = 1;
const MPU401_INFO_IRQ_HOOK: c_uint = 1;

#[repr(u32)]
enum FIRMWARE { DATA_REC = 0, EXT_END_OF_FILE, EXT_SEG_ADDR_REC, EXT_GOTO_CMD_REC, EXT_LIN_ADDR_REC }
const DATA_REC: u32 = 0; const EXT_END_OF_FILE: u32 = 1; const EXT_SEG_ADDR_REC: u32 = 2; const EXT_GOTO_CMD_REC: u32 = 3; const EXT_LIN_ADDR_REC: u32 = 4;

const GETV: u32 = 0x00; const GETC: u32 = 0x01; const GUNS: u32 = 0x02; const SCID: u32 = 0x03;
const RMEM: u32 = 0x10; const SMEM: u32 = 0x11; const WMEM: u32 = 0x12; const SDTM: u32 = 0x13; const GOTO: u32 = 0x14;
const SSTR: u32 = 0x20; const PSTR: u32 = 0x21; const KSTR: u32 = 0x22; const KDMA: u32 = 0x23; const GPOS: u32 = 0x24; const SETF: u32 = 0x25; const GSTS: u32 = 0x26; const NGPOS: u32 = 0x27;
const PSEL: u32 = 0x30; const PCLR: u32 = 0x31; const PLST: u32 = 0x32; const RSSV: u32 = 0x33;
const LSEL: u32 = 0x34; const SSRC: u32 = 0x40; const SLST: u32 = 0x41; const RSRC: u32 = 0x42; const SSRB: u32 = 0x43;
const SDGV: u32 = 0x50; const RDGV: u32 = 0x51; const DLST: u32 = 0x52; const SACR: u32 = 0x60; const RACR: u32 = 0x61; const ALST: u32 = 0x62; const TXAC: u32 = 0x63; const RXAC: u32 = 0x64; const SI2S: u32 = 0x70; const ARM_SETDPLL: u32 = 0x72;

const ARM2LBUS_FIFO0: u8 = 0; const ARM2LBUS_FIFO1: u8 = 1; const ARM2LBUS_FIFO2: u8 = 2; const ARM2LBUS_FIFO3: u8 = 3;
const ARM2LBUS_FIFO4: u8 = 4; const ARM2LBUS_FIFO5: u8 = 5; const ARM2LBUS_FIFO6: u8 = 6; const ARM2LBUS_FIFO7: u8 = 7;
const ARM2LBUS_FIFO8: u8 = 8; const ARM2LBUS_FIFO9: u8 = 9; const ARM2LBUS_FIFO10: u8 = 10; const ARM2LBUS_FIFO11: u8 = 11;
const ARM2LBUS_FIFO12: u8 = 12; const ARM2LBUS_FIFO13: u8 = 13; const ARM2LBUS_FIFO14: u8 = 14; const ARM2LBUS_FIFO15: u8 = 15;
const INTER0_OUT: u8 = 16; const INTER1_OUT: u8 = 17; const INTER2_OUT: u8 = 18; const INTER3_OUT: u8 = 19; const INTER4_OUT: u8 = 20;
const INTERM0_OUT: u8 = 21; const INTERM1_OUT: u8 = 22; const INTERM2_OUT: u8 = 23; const INTERM3_OUT: u8 = 24; const INTERM4_OUT: u8 = 25; const INTERM5_OUT: u8 = 26; const INTERM6_OUT: u8 = 27;
const DECIMM0_OUT: u8 = 28; const DECIMM1_OUT: u8 = 29; const DECIMM2_OUT: u8 = 30; const DECIMM3_OUT: u8 = 31; const DECIM0_OUT: u8 = 32; const SR3_4_OUT: u8 = 33; const OPL3_SAMPLE: u8 = 34; const ASRC0: u8 = 35; const ASRC1: u8 = 36;
const ACLNK2PADC: u8 = 37; const ACLNK2MODEM0RX: u8 = 38; const ACLNK2MIC: u8 = 39; const ACLNK2MODEM1RX: u8 = 40; const ACLNK2HNDMIC: u8 = 41;
const DIGITAL_MIXER_OUT0: u8 = 42; const GAINFUNC0_OUT: u8 = 43; const GAINFUNC1_OUT: u8 = 44; const GAINFUNC2_OUT: u8 = 45; const GAINFUNC3_OUT: u8 = 46; const GAINFUNC4_OUT: u8 = 47; const SOFTMODEMTX: u8 = 48;
const SPLITTER0_OUTL: u8 = 49; const SPLITTER0_OUTR: u8 = 50; const SPLITTER1_OUTL: u8 = 51; const SPLITTER1_OUTR: u8 = 52; const SPLITTER2_OUTL: u8 = 53; const SPLITTER2_OUTR: u8 = 54; const SPLITTER3_OUTL: u8 = 55; const SPLITTER3_OUTR: u8 = 56;
const MERGER0_OUT: u8 = 57; const MERGER1_OUT: u8 = 59; const MERGER2_OUT: u8 = 61; const MERGER3_OUT: u8 = 63; const ARM2LBUS_FIFO_DIRECT: u8 = 65; const NO_OUT: u8 = 66;

const LBUS2ARM_FIFO0: u8 = 0; const LBUS2ARM_FIFO1: u8 = 1; const LBUS2ARM_FIFO2: u8 = 2; const LBUS2ARM_FIFO3: u8 = 3;
const LBUS2ARM_FIFO4: u8 = 4; const LBUS2ARM_FIFO5: u8 = 5; const LBUS2ARM_FIFO6: u8 = 6; const LBUS2ARM_FIFO7: u8 = 7;
const INTER0_IN: u8 = 8; const INTER1_IN: u8 = 9; const INTER2_IN: u8 = 10; const INTER3_IN: u8 = 11; const INTER4_IN: u8 = 12;
const INTERM0_IN: u8 = 13; const INTERM1_IN: u8 = 14; const INTERM2_IN: u8 = 15; const INTERM3_IN: u8 = 16; const INTERM4_IN: u8 = 17; const INTERM5_IN: u8 = 18; const INTERM6_IN: u8 = 19;
const DECIMM0_IN: u8 = 20; const DECIMM1_IN: u8 = 21; const DECIMM2_IN: u8 = 22; const DECIMM3_IN: u8 = 23; const DECIM0_IN: u8 = 24; const SR3_4_IN: u8 = 25;
const PDAC2ACLNK: u8 = 26; const MODEM0TX2ACLNK: u8 = 27; const MODEM1TX2ACLNK: u8 = 28; const HNDSPK2ACLNK: u8 = 29;
const DIGITAL_MIXER_IN0: u8 = 30; const DIGITAL_MIXER_IN1: u8 = 31; const DIGITAL_MIXER_IN2: u8 = 32; const DIGITAL_MIXER_IN3: u8 = 33; const DIGITAL_MIXER_IN4: u8 = 34; const DIGITAL_MIXER_IN5: u8 = 35; const DIGITAL_MIXER_IN6: u8 = 36; const DIGITAL_MIXER_IN7: u8 = 37; const DIGITAL_MIXER_IN8: u8 = 38; const DIGITAL_MIXER_IN9: u8 = 39; const DIGITAL_MIXER_IN10: u8 = 40; const DIGITAL_MIXER_IN11: u8 = 41;
const GAINFUNC0_IN: u8 = 42; const GAINFUNC1_IN: u8 = 43; const GAINFUNC2_IN: u8 = 44; const GAINFUNC3_IN: u8 = 45; const GAINFUNC4_IN: u8 = 46; const SOFTMODEMRX: u8 = 47;
const SPLITTER0_IN: u8 = 48; const SPLITTER1_IN: u8 = 49; const SPLITTER2_IN: u8 = 50; const SPLITTER3_IN: u8 = 51;
const MERGER0_INL: u8 = 52; const MERGER0_INR: u8 = 53; const MERGER1_INL: u8 = 54; const MERGER1_INR: u8 = 55; const MERGER2_INL: u8 = 56; const MERGER2_INR: u8 = 57; const MERGER3_INL: u8 = 58; const MERGER3_INR: u8 = 59; const E2SINK_MAX: usize = 60;

const LS_SRC_INTERPOLATOR: u8 = 0; const LS_SRC_INTERPOLATORM: u8 = 1; const LS_SRC_DECIMATOR: u8 = 2; const LS_SRC_DECIMATORM: u8 = 3; const LS_MIXER_IN: u8 = 4; const LS_MIXER_GAIN_FUNCTION: u8 = 5; const LS_SRC_SPLITTER: u8 = 6; const LS_SRC_MERGER: u8 = 7; const LS_NONE1: u8 = 8; const LS_NONE2: u8 = 9;
const M0TX: u8 = 0; const M1TX: u8 = 1; const TAMTX: u8 = 2; const HSSPKR: u8 = 3; const PDAC: u8 = 4; const DSNDTX0: u8 = 5; const DSNDTX1: u8 = 6; const DSNDTX2: u8 = 7; const DSNDTX3: u8 = 8; const DSNDTX4: u8 = 9; const DSNDTX5: u8 = 10; const DSNDTX6: u8 = 11; const DSNDTX7: u8 = 12; const WVSTRTX: u8 = 13; const COP3DTX: u8 = 14; const SPARE: u8 = 15; const M0RX: u8 = 16; const HSMIC: u8 = 17; const M1RX: u8 = 18; const CLEANRX: u8 = 19; const MICADC: u8 = 20; const PADC: u8 = 21; const COPRX1: u8 = 22; const COPRX2: u8 = 23; const CHANNEL_ID_COUNTER: u8 = 24;
const SB_CMD: u8 = 0; const MODEM_CMD: u8 = 1; const I2S_CMD0: u8 = 2; const I2S_CMD1: u8 = 3; const FM_CMD: u8 = 4; const MAX_CMD: u8 = 5;

#[repr(C)] #[derive(Copy, Clone)] struct lbuspath { noconv: *const u8, stereo: *const u8, mono: *const u8 }
#[repr(C)] struct cmdport { data1: u32, data2: u32, stat: u32, pad: [u32; 5] }
#[repr(C)] struct riptideport { audio_control: u32, audio_status: u32, pad: [u32; 2], port: [cmdport; 2] }
#[repr(C)] struct cmdif { dev: *mut device, hwport: *mut riptideport, lock: spinlock_t, cmdcnt: c_uint, cmdtime: c_uint, cmdtimemax: c_uint, cmdtimemin: c_uint, errcnt: c_uint, is_reset: c_int }
#[repr(C)] #[derive(Copy, Clone)] struct riptide_firmware { ASIC: u16, CODEC: u16, AUXDSP: u16, PROG: u16 }
#[repr(C)] union cmdret { retbytes: [u8; 8], retwords: [u16; 4], retlongs: [u32; 2] }
#[repr(C)] union firmware_version { ret: cmdret, firmware: riptide_firmware }
const PLAYBACK_SUBSTREAMS: usize = 3;
#[repr(C)] struct snd_riptide {
    card: *mut snd_card, pci: *mut pci_dev, fw_entry: *const firmware, cif: *mut cmdif,
    pcm: *mut snd_pcm, pcm_i2s: *mut snd_pcm, rmidi: *mut snd_rawmidi, opl3: *mut snd_opl3,
    ac97: *mut snd_ac97, ac97_bus: *mut snd_ac97_bus,
    playback_substream: [*mut snd_pcm_substream; PLAYBACK_SUBSTREAMS], capture_substream: *mut snd_pcm_substream,
    openstreams: c_int, irq: c_int, port: c_ulong, mpuaddr: c_ushort, opladdr: c_ushort,
    gameaddr: c_ushort, res_port: *mut resource, device_id: c_ushort, firmware: firmware_version,
    lock: spinlock_t, proc_entry: *mut snd_info_entry, received_irqs: c_ulong, handled_irqs: c_ulong, in_suspend: c_int,
}
#[repr(C)] struct sgd { dwNextLink: __le32, dwSegPtrPhys: __le32, dwSegLen: __le32, dwStat_Ctl: __le32 }
#[repr(C)] struct pcmhw {
    paths: lbuspath, lbuspath: *const u8, source: u8, intdec: [u8; 2], mixer: u8, id: u8,
    state: u8, rate: c_uint, channels: c_uint, format: snd_pcm_format_t, sgdlist: snd_dma_buffer,
    sgdbuf: *mut sgd, size: c_uint, pages: c_uint, oldpos: c_uint, pointer: c_uint,
}

const CMDRET_ZERO: cmdret = cmdret { retlongs: [0, 0] };
unsafe fn get_pcmhwdev(substream: *mut snd_pcm_substream) -> *mut pcmhw { (*(*substream).runtime).private_data as *mut pcmhw }
unsafe fn READ_PORT_ULONG(p: *const u32) -> u32 { inl(p as c_ulong) }
unsafe fn WRITE_PORT_ULONG(p: *mut u32, x: u32) { outl(x, p as c_ulong) }
unsafe fn READ_AUDIO_CONTROL(p: *mut riptideport) -> u32 { READ_PORT_ULONG(ptr::addr_of!((*p).audio_control)) }
unsafe fn WRITE_AUDIO_CONTROL(p: *mut riptideport, x: u32) { WRITE_PORT_ULONG(ptr::addr_of_mut!((*p).audio_control), x) }
unsafe fn UMASK_AUDIO_CONTROL(p: *mut riptideport, x: u32) { WRITE_AUDIO_CONTROL(p, READ_AUDIO_CONTROL(p) | x) }
unsafe fn MASK_AUDIO_CONTROL(p: *mut riptideport, x: u32) { WRITE_AUDIO_CONTROL(p, READ_AUDIO_CONTROL(p) & x) }
unsafe fn READ_AUDIO_STATUS(p: *mut riptideport) -> u32 { READ_PORT_ULONG(ptr::addr_of!((*p).audio_status)) }
unsafe fn SET_GRESET(p: *mut riptideport) { UMASK_AUDIO_CONTROL(p, 0x0001) }
unsafe fn UNSET_GRESET(p: *mut riptideport) { MASK_AUDIO_CONTROL(p, !0x0001) }
unsafe fn SET_AIE(p: *mut riptideport) { UMASK_AUDIO_CONTROL(p, 0x0004) }
unsafe fn UNSET_AIE(p: *mut riptideport) { MASK_AUDIO_CONTROL(p, !0x0004) }
unsafe fn SET_AIACK(p: *mut riptideport) { UMASK_AUDIO_CONTROL(p, 0x0008) }
unsafe fn SET_EMPUIRQ(p: *mut riptideport) { UMASK_AUDIO_CONTROL(p, 0x0200) }
unsafe fn IS_CMDE(a: *mut cmdport) -> bool { (READ_PORT_ULONG(ptr::addr_of!((*a).stat)) & 0x1) != 0 }
unsafe fn IS_DATF(a: *mut cmdport) -> bool { (READ_PORT_ULONG(ptr::addr_of!((*a).stat)) & 0x2) != 0 }
unsafe fn IS_READY(p: *mut riptideport) -> bool { (READ_AUDIO_STATUS(p) & 0x0001) != 0 }
unsafe fn IS_GERR(p: *mut riptideport) -> bool { (READ_AUDIO_STATUS(p) & 0x0008) != 0 }
unsafe fn IS_EOBIRQ(p: *mut riptideport) -> bool { (READ_AUDIO_STATUS(p) & 0x0100) != 0 }
unsafe fn IS_EOSIRQ(p: *mut riptideport) -> bool { (READ_AUDIO_STATUS(p) & 0x0200) != 0 }
unsafe fn IS_EOCIRQ(p: *mut riptideport) -> bool { (READ_AUDIO_STATUS(p) & 0x0400) != 0 }
unsafe fn IS_MPUIRQ(p: *mut riptideport) -> bool { (READ_AUDIO_STATUS(p) & 0x2000) != 0 }
fn LONG0(a: u32) -> u32 { a }
fn BYTE0(a: u32) -> u32 { LONG0(a) & 0xff }
fn BYTE1(a: u32) -> u32 { BYTE0(a) << 8 }
fn BYTE2(a: u32) -> u32 { BYTE0(a) << 16 }
fn BYTE3(a: u32) -> u32 { BYTE0(a) << 24 }
fn WORD0(a: u32) -> u32 { LONG0(a) & 0xffff }
fn WORD1(a: u32) -> u32 { WORD0(a) << 8 }
fn WORD2(a: u32) -> u32 { WORD0(a) << 16 }
fn TRINIB0(a: u32) -> u32 { LONG0(a) & 0xffffff }
fn TRINIB1(a: u32) -> u32 { TRINIB0(a) << 8 }
fn DIV_ROUND_UP(n: c_uint, d: c_uint) -> c_uint { (n + d - 1) / d }

unsafe fn SEND_GETV(p:*mut cmdif,b:*mut cmdret)->c_int{sendcmd(p,RESP,GETV,0,b)}
unsafe fn SEND_RMEM(p:*mut cmdif,b:u32,c:u32,d:*mut cmdret)->c_int{sendcmd(p,PARM|RESP,RMEM|BYTE1(b),LONG0(c),d)}
unsafe fn SEND_SMEM(p:*mut cmdif,b:u32,c:u32)->c_int{sendcmd(p,PARM,SMEM|BYTE1(b),LONG0(c),ptr::null_mut())}
unsafe fn SEND_WMEM(p:*mut cmdif,b:u32,c:u32)->c_int{sendcmd(p,PARM,WMEM|BYTE1(b),LONG0(c),ptr::null_mut())}
unsafe fn SEND_GOTO(p:*mut cmdif,b:u32)->c_int{sendcmd(p,PARM,GOTO,LONG0(b),ptr::null_mut())}
unsafe fn SEND_SETDPLL(p:*mut cmdif)->c_int{sendcmd(p,0,ARM_SETDPLL,0,ptr::null_mut())}
unsafe fn SEND_SSTR(p:*mut cmdif,b:u8,c:dma_addr_t)->c_int{sendcmd(p,PARM,SSTR|BYTE3(b as u32),LONG0(c as u32),ptr::null_mut())}
unsafe fn SEND_PSTR(p:*mut cmdif,b:u8)->c_int{sendcmd(p,PARM,PSTR,BYTE3(b as u32),ptr::null_mut())}
unsafe fn SEND_KSTR(p:*mut cmdif,b:u8)->c_int{sendcmd(p,PARM,KSTR,BYTE3(b as u32),ptr::null_mut())}
unsafe fn SEND_KDMA(p:*mut cmdif)->c_int{sendcmd(p,0,KDMA,0,ptr::null_mut())}
unsafe fn SEND_GPOS(p:*mut cmdif,b:u32,c:u8,d:*mut cmdret)->c_int{sendcmd(p,PARM|RESP,GPOS,BYTE3(c as u32)|BYTE2(b),d)}
unsafe fn SEND_SETF(p:*mut cmdif,b:u8,c:u8,d:u32,e:u8,f:u8,g:u8)->c_int{sendcmd(p,PARM,SETF|WORD1(b as u32)|BYTE3(c as u32),d|BYTE1(e as u32)|BYTE2(f as u32)|BYTE3(g as u32),ptr::null_mut())}
unsafe fn SEND_PSEL(p:*mut cmdif,b:u8,c:u8)->c_int{sendcmd(p,PARM,PSEL,BYTE2(b as u32)|BYTE3(c as u32),ptr::null_mut())}
unsafe fn SEND_PCLR(p:*mut cmdif,b:u8,c:u8)->c_int{sendcmd(p,PARM,PCLR,BYTE2(b as u32)|BYTE3(c as u32),ptr::null_mut())}
unsafe fn SEND_PLST(p:*mut cmdif,b:u32)->c_int{sendcmd(p,PARM,PLST,BYTE3(b),ptr::null_mut())}
unsafe fn SEND_RSSV(p:*mut cmdif,b:u8,c:u8,d:*mut cmdret)->c_int{sendcmd(p,PARM|RESP,RSSV,BYTE2(b as u32)|BYTE3(c as u32),d)}
unsafe fn SEND_LSEL(p:*mut cmdif,b:u8,c:u8,d:u8,e:u8,f:u8,g:u8,h:u8)->c_int{sendcmd(p,PARM,LSEL|BYTE1(b as u32)|BYTE2(c as u32)|BYTE3(d as u32),BYTE0(e as u32)|BYTE1(f as u32)|BYTE2(g as u32)|BYTE3(h as u32),ptr::null_mut())}
unsafe fn SEND_SSRC(p:*mut cmdif,b:u8,c:u32,d:u32,e:u32)->c_int{sendcmd(p,PARM,SSRC|BYTE1(b as u32)|WORD2(c),WORD0(d)|WORD2(e),ptr::null_mut())}
unsafe fn SEND_SLST(p:*mut cmdif,b:u32)->c_int{sendcmd(p,PARM,SLST,BYTE3(b),ptr::null_mut())}
unsafe fn SEND_RSRC(p:*mut cmdif,b:u8,c:*mut cmdret)->c_int{sendcmd(p,RESP,RSRC|BYTE1(b as u32),0,c)}
unsafe fn SEND_SDGV(p:*mut cmdif,b:i16,c:i16,d:u16,e:u16)->c_int{sendcmd(p,PARM,SDGV|BYTE2(b as u32)|BYTE3(c as u32),WORD0(d as u32)|WORD2(e as u32),ptr::null_mut())}
unsafe fn SEND_RDGV(p:*mut cmdif,b:i16,c:i16,d:*mut cmdret)->c_int{sendcmd(p,PARM|RESP,RDGV|BYTE2(b as u32)|BYTE3(c as u32),0,d)}
unsafe fn SEND_DLST(p:*mut cmdif,b:u32)->c_int{sendcmd(p,PARM,DLST,BYTE3(b),ptr::null_mut())}
unsafe fn SEND_SACR(p:*mut cmdif,b:u16,c:u16)->c_int{sendcmd(p,PARM,SACR,WORD0(b as u32)|WORD2(c as u32),ptr::null_mut())}
unsafe fn SEND_RACR(p:*mut cmdif,b:u16,c:*mut cmdret)->c_int{sendcmd(p,PARM|RESP,RACR,WORD2(b as u32),c)}
unsafe fn SEND_ALST(p:*mut cmdif,b:u32)->c_int{sendcmd(p,PARM,ALST,BYTE3(b),ptr::null_mut())}
unsafe fn SEND_SI2S(p:*mut cmdif,b:u32)->c_int{sendcmd(p,PARM,SI2S,WORD2(b),ptr::null_mut())}

static snd_riptide_ids: [pci_device_id; 5] = [
    pci_device_id{vendor:0x127a,device:0x4310,subvendor:0,subdevice:0,class:0,class_mask:0,driver_data:0},
    pci_device_id{vendor:0x127a,device:0x4320,subvendor:0,subdevice:0,class:0,class_mask:0,driver_data:0},
    pci_device_id{vendor:0x127a,device:0x4330,subvendor:0,subdevice:0,class:0,class_mask:0,driver_data:0},
    pci_device_id{vendor:0x127a,device:0x4340,subvendor:0,subdevice:0,class:0,class_mask:0,driver_data:0},
    pci_device_id{vendor:0,device:0,subvendor:0,subdevice:0,class:0,class_mask:0,driver_data:0},
];
static snd_riptide_joystick_ids: [pci_device_id; 5] = [
    pci_device_id{vendor:0x127a,device:0x4312,subvendor:0,subdevice:0,class:0,class_mask:0,driver_data:0},
    pci_device_id{vendor:0x127a,device:0x4322,subvendor:0,subdevice:0,class:0,class_mask:0,driver_data:0},
    pci_device_id{vendor:0x127a,device:0x4332,subvendor:0,subdevice:0,class:0,class_mask:0,driver_data:0},
    pci_device_id{vendor:0x127a,device:0x4342,subvendor:0,subdevice:0,class:0,class_mask:0,driver_data:0},
    pci_device_id{vendor:0,device:0,subvendor:0,subdevice:0,class:0,class_mask:0,driver_data:0},
];

static lbusin2out: [[u8; 2]; E2SINK_MAX + 1] = [
    [NO_OUT,LS_NONE1],[NO_OUT,LS_NONE2],[NO_OUT,LS_NONE1],[NO_OUT,LS_NONE2],
    [NO_OUT,LS_NONE1],[NO_OUT,LS_NONE2],[NO_OUT,LS_NONE1],[NO_OUT,LS_NONE2],
    [INTER0_OUT,LS_SRC_INTERPOLATOR],[INTER1_OUT,LS_SRC_INTERPOLATOR],[INTER2_OUT,LS_SRC_INTERPOLATOR],[INTER3_OUT,LS_SRC_INTERPOLATOR],[INTER4_OUT,LS_SRC_INTERPOLATOR],
    [INTERM0_OUT,LS_SRC_INTERPOLATORM],[INTERM1_OUT,LS_SRC_INTERPOLATORM],[INTERM2_OUT,LS_SRC_INTERPOLATORM],[INTERM3_OUT,LS_SRC_INTERPOLATORM],[INTERM4_OUT,LS_SRC_INTERPOLATORM],[INTERM5_OUT,LS_SRC_INTERPOLATORM],[INTERM6_OUT,LS_SRC_INTERPOLATORM],
    [DECIMM0_OUT,LS_SRC_DECIMATORM],[DECIMM1_OUT,LS_SRC_DECIMATORM],[DECIMM2_OUT,LS_SRC_DECIMATORM],[DECIMM3_OUT,LS_SRC_DECIMATORM],
    [DECIM0_OUT,LS_SRC_DECIMATOR],[SR3_4_OUT,LS_NONE1],[NO_OUT,LS_NONE2],[NO_OUT,LS_NONE1],[NO_OUT,LS_NONE2],[NO_OUT,LS_NONE1],
    [DIGITAL_MIXER_OUT0,LS_MIXER_IN],[DIGITAL_MIXER_OUT0,LS_MIXER_IN],[DIGITAL_MIXER_OUT0,LS_MIXER_IN],[DIGITAL_MIXER_OUT0,LS_MIXER_IN],[DIGITAL_MIXER_OUT0,LS_MIXER_IN],[DIGITAL_MIXER_OUT0,LS_MIXER_IN],
    [DIGITAL_MIXER_OUT0,LS_MIXER_IN],[DIGITAL_MIXER_OUT0,LS_MIXER_IN],[DIGITAL_MIXER_OUT0,LS_MIXER_IN],[DIGITAL_MIXER_OUT0,LS_MIXER_IN],[DIGITAL_MIXER_OUT0,LS_MIXER_IN],[DIGITAL_MIXER_OUT0,LS_MIXER_IN],
    [GAINFUNC0_OUT,LS_MIXER_GAIN_FUNCTION],[GAINFUNC1_OUT,LS_MIXER_GAIN_FUNCTION],[GAINFUNC2_OUT,LS_MIXER_GAIN_FUNCTION],[GAINFUNC3_OUT,LS_MIXER_GAIN_FUNCTION],[GAINFUNC4_OUT,LS_MIXER_GAIN_FUNCTION],[SOFTMODEMTX,LS_NONE1],
    [SPLITTER0_OUTL,LS_SRC_SPLITTER],[SPLITTER1_OUTL,LS_SRC_SPLITTER],[SPLITTER2_OUTL,LS_SRC_SPLITTER],[SPLITTER3_OUTL,LS_SRC_SPLITTER],
    [MERGER0_OUT,LS_SRC_MERGER],[MERGER0_OUT,LS_SRC_MERGER],[MERGER1_OUT,LS_SRC_MERGER],[MERGER1_OUT,LS_SRC_MERGER],[MERGER2_OUT,LS_SRC_MERGER],[MERGER2_OUT,LS_SRC_MERGER],[MERGER3_OUT,LS_SRC_MERGER],[MERGER3_OUT,LS_SRC_MERGER],
    [NO_OUT,LS_NONE2],
];

static lbus_play_opl3: [u8; 2] = [DIGITAL_MIXER_IN0 + FM_MIXER, 0xff];
static lbus_play_modem: [u8; 2] = [DIGITAL_MIXER_IN0 + MODEM_MIXER, 0xff];
static lbus_play_i2s: [u8; 3] = [INTER0_IN + I2S_INTDEC, DIGITAL_MIXER_IN0 + I2S_MIXER, 0xff];
static lbus_play_out: [u8; 2] = [PDAC2ACLNK, 0xff];
static lbus_play_outhp: [u8; 2] = [HNDSPK2ACLNK, 0xff];
static lbus_play_noconv1: [u8; 2] = [DIGITAL_MIXER_IN0, 0xff];
static lbus_play_stereo1: [u8; 3] = [INTER0_IN, DIGITAL_MIXER_IN0, 0xff];
static lbus_play_mono1: [u8; 3] = [INTERM0_IN, DIGITAL_MIXER_IN0, 0xff];
static lbus_play_noconv2: [u8; 2] = [DIGITAL_MIXER_IN1, 0xff];
static lbus_play_stereo2: [u8; 3] = [INTER1_IN, DIGITAL_MIXER_IN1, 0xff];
static lbus_play_mono2: [u8; 3] = [INTERM1_IN, DIGITAL_MIXER_IN1, 0xff];
static lbus_play_noconv3: [u8; 2] = [DIGITAL_MIXER_IN2, 0xff];
static lbus_play_stereo3: [u8; 3] = [INTER2_IN, DIGITAL_MIXER_IN2, 0xff];
static lbus_play_mono3: [u8; 3] = [INTERM2_IN, DIGITAL_MIXER_IN2, 0xff];
static lbus_rec_noconv1: [u8; 2] = [LBUS2ARM_FIFO5, 0xff];
static lbus_rec_stereo1: [u8; 3] = [DECIM0_IN, LBUS2ARM_FIFO5, 0xff];
static lbus_rec_mono1: [u8; 3] = [DECIMM3_IN, LBUS2ARM_FIFO5, 0xff];
static play_ids: [u8; 3] = [4, 1, 2];
static play_sources: [u8; 3] = [ARM2LBUS_FIFO4, ARM2LBUS_FIFO1, ARM2LBUS_FIFO2];
static lbus_play_paths: [lbuspath; 3] = [
    lbuspath{noconv:lbus_play_noconv1.as_ptr(),stereo:lbus_play_stereo1.as_ptr(),mono:lbus_play_mono1.as_ptr()},
    lbuspath{noconv:lbus_play_noconv2.as_ptr(),stereo:lbus_play_stereo2.as_ptr(),mono:lbus_play_mono2.as_ptr()},
    lbuspath{noconv:lbus_play_noconv3.as_ptr(),stereo:lbus_play_stereo3.as_ptr(),mono:lbus_play_mono3.as_ptr()},
];
static lbus_rec_path: lbuspath = lbuspath{noconv:lbus_rec_noconv1.as_ptr(),stereo:lbus_rec_stereo1.as_ptr(),mono:lbus_rec_mono1.as_ptr()};
const FIRMWARE_VERSIONS: usize = 1;
static mut firmware_versions: [firmware_version; 1] = [firmware_version{firmware:riptide_firmware{ASIC:3,CODEC:2,AUXDSP:3,PROG:773}}];

unsafe fn atoh(in_: *const u8, mut len: c_uint) -> u32 {
    let mut sum: u32 = 0; let mut mult: u32 = 1;
    while len != 0 {
        let c = *in_.add((len - 1) as usize);
        let value = hex_to_bin(c);
        if value >= 0 { sum = sum.wrapping_add(mult.wrapping_mul(value as u32)); }
        mult = mult.wrapping_mul(16); len -= 1;
    }
    sum
}

unsafe fn senddata(cif: *mut cmdif, in_: *const u8, offset: u32) -> c_int {
    let mut i = atoh(in_.add(1), 2);
    let addr = offset.wrapping_add(atoh(in_.add(3), 4));
    if SEND_SMEM(cif, 0, addr) != 0 { return -EACCES; }
    let mut p = in_.add(9);
    while i != 0 {
        let data = atoh(p, 8);
        if SEND_WMEM(cif, 2, ((data & 0x0f0f0f0f) << 4) | ((data & 0xf0f0f0f0) >> 4)) != 0 { return -EACCES; }
        i = i.wrapping_sub(4); p = p.add(8);
    }
    0
}

unsafe fn loadfirmware(cif: *mut cmdif, mut img: *const u8, mut size: c_uint) -> c_int {
    let mut laddr: u32 = 0; let mut saddr: u32 = 0; let mut err: c_int = 0;
    while size > 0 && err == 0 {
        let in_ = img;
        if *in_ == b':' {
            let t = atoh(in_.add(7), 2);
            match t {
                DATA_REC => err = senddata(cif, in_, laddr.wrapping_add(saddr)),
                EXT_SEG_ADDR_REC => saddr = atoh(in_.add(9), 4) << 4,
                EXT_LIN_ADDR_REC => laddr = atoh(in_.add(9), 4) << 16,
                EXT_GOTO_CMD_REC => { let val = atoh(in_.add(9), 8); if SEND_GOTO(cif, val) != 0 { err = -EACCES; } }
                EXT_END_OF_FILE => size = 0,
                _ => {}
            }
            while size > 0 { size -= 1; let ch = *img; img = img.add(1); if ch == b'\n' { break; } }
        }
    }
    dev_dbg((*cif).dev, b"load firmware return %d\n\0".as_ptr() as *const c_char, err);
    err
}

unsafe fn alloclbuspath(cif:*mut cmdif, mut source:u8, mut path:*const u8, mixer:*mut u8, s:*mut u8) {
    while *path != 0xff {
        let sink = *path & !SPLIT_PATH;
        if sink != E2SINK_MAX as u8 {
            dev_dbg((*cif).dev, b"alloc path 0x%x->0x%x\n\0".as_ptr() as *const c_char, source as c_uint, sink as c_uint);
            SEND_PSEL(cif, source, sink);
            source = lbusin2out[sink as usize][0];
            let type_ = lbusin2out[sink as usize][1];
            if type_ == LS_MIXER_IN && !mixer.is_null() { *mixer = sink - DIGITAL_MIXER_IN0; }
            if type_ == LS_SRC_DECIMATORM || type_ == LS_SRC_DECIMATOR || type_ == LS_SRC_INTERPOLATORM || type_ == LS_SRC_INTERPOLATOR {
                if !s.is_null() { if *s != 0xff { *s.add(1) = sink; } else { *s = sink; } }
            }
        }
        let old = *path; path = path.add(1);
        if (old & SPLIT_PATH) != 0 {
            let mut npath = path; while *npath != 0xff { npath = npath.add(1); }
            alloclbuspath(cif, source + 1, npath.add(1), mixer, s);
        }
    }
}

unsafe fn freelbuspath(cif:*mut cmdif, mut source:u8, mut path:*const u8) {
    while *path != 0xff {
        let sink = *path & !SPLIT_PATH;
        if sink != E2SINK_MAX as u8 {
            dev_dbg((*cif).dev, b"free path 0x%x->0x%x\n\0".as_ptr() as *const c_char, source as c_uint, sink as c_uint);
            SEND_PCLR(cif, source, sink); source = lbusin2out[sink as usize][0];
        }
        let old = *path; path = path.add(1);
        if (old & SPLIT_PATH) != 0 {
            let mut npath = path; while *npath != 0xff { npath = npath.add(1); }
            freelbuspath(cif, source + 1, npath.add(1));
        }
    }
}

unsafe fn writearm(cif:*mut cmdif, addr:u32, data:u32, mask:u32) -> c_int {
    let mut rptr = CMDRET_ZERO; let mut i = MAX_WRITE_RETRY; let mut flag = 1;
    SEND_RMEM(cif, 0x02, addr, &mut rptr);
    rptr.retlongs[0] &= !mask;
    while { i -= 1; i != 0 } {
        SEND_SMEM(cif, 0x01, addr); SEND_WMEM(cif, 0x02, rptr.retlongs[0] | data); SEND_RMEM(cif, 0x02, addr, &mut rptr);
        if (rptr.retlongs[0] & data) == data { flag = 0; break; } else { rptr.retlongs[0] &= !mask; }
    }
    dev_dbg((*cif).dev, b"send arm 0x%x 0x%x 0x%x return %d\n\0".as_ptr() as *const c_char, addr, data, mask, flag);
    flag
}

unsafe fn sendcmd(cif:*mut cmdif, flags:u32, cmd:u32, parm:u32, ret:*mut cmdret) -> c_int {
    let mut i:c_int; let mut j:c_int; let mut err:c_int; let mut time:c_uint = 0; let irqflags:c_ulong = 0;
    let mut cmdport: *mut cmdport = ptr::null_mut();
    if snd_BUG_ON(cif.is_null()) != 0 { return -EINVAL; }
    let hwport = (*cif).hwport;
    if (*cif).errcnt > MAX_ERROR_COUNT {
        if (*cif).is_reset != 0 {
            dev_err((*cif).dev, b"Riptide: Too many failed cmds, reinitializing\n\0".as_ptr() as *const c_char);
            if riptide_reset(cif, ptr::null_mut()) == 0 { (*cif).errcnt = 0; return -EIO; }
        }
        dev_err((*cif).dev, b"Riptide: Initialization failed.\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    if !ret.is_null() { (*ret).retlongs[0] = 0; (*ret).retlongs[1] = 0; }
    i = 0; spin_lock_irqsave(&mut (*cif).lock, irqflags);
    while { i += 1; (i as c_uint) < CMDIF_TIMEOUT && !IS_READY((*cif).hwport) } { udelay(10); }
    if i as c_uint > CMDIF_TIMEOUT { err = -EBUSY; goto_errout(cif, cmdport, hwport, flags, cmd, parm, ret, err); return err; }
    err = 0; j = 0; time = 0;
    while time < CMDIF_TIMEOUT {
        cmdport = &mut (*hwport).port[(j % 2) as usize];
        if IS_DATF(cmdport) { READ_PORT_ULONG(ptr::addr_of!((*cmdport).data1)); READ_PORT_ULONG(ptr::addr_of!((*cmdport).data2)); }
        if IS_CMDE(cmdport) {
            if (flags & PARM) != 0 { WRITE_PORT_ULONG(ptr::addr_of_mut!((*cmdport).data2), parm); }
            WRITE_PORT_ULONG(ptr::addr_of_mut!((*cmdport).data1), cmd);
            if (flags & RESP) != 0 && !ret.is_null() {
                while !IS_DATF(cmdport) && time < CMDIF_TIMEOUT { udelay(10); time += 1; }
                if time < CMDIF_TIMEOUT {
                    (*ret).retlongs[0] = READ_PORT_ULONG(ptr::addr_of!((*cmdport).data1));
                    (*ret).retlongs[1] = READ_PORT_ULONG(ptr::addr_of!((*cmdport).data2));
                } else { err = -ENOSYS; goto_errout(cif, cmdport, hwport, flags, cmd, parm, ret, err); return err; }
            }
            break;
        }
        udelay(20); j += 1; time += 2;
    }
    if time == CMDIF_TIMEOUT { err = -ENODATA; goto_errout(cif, cmdport, hwport, flags, cmd, parm, ret, err); return err; }
    spin_unlock_irqrestore(&mut (*cif).lock, irqflags);
    (*cif).cmdcnt += 1; (*cif).cmdtime += time;
    if time > (*cif).cmdtimemax { (*cif).cmdtimemax = time; }
    if time < (*cif).cmdtimemin { (*cif).cmdtimemin = time; }
    if (*cif).cmdcnt % 1000 == 0 {
        dev_dbg((*cif).dev, b"send cmd %d time: %d mintime: %d maxtime %d err: %d\n\0".as_ptr() as *const c_char, (*cif).cmdcnt, (*cif).cmdtime, (*cif).cmdtimemin, (*cif).cmdtimemax, (*cif).errcnt);
    }
    0
}

unsafe fn goto_errout(cif:*mut cmdif, cmdport:*mut cmdport, hwport:*mut riptideport, flags:u32, cmd:u32, parm:u32, ret:*mut cmdret, err:c_int) {
    let irqflags:c_ulong = 0; (*cif).errcnt += 1; spin_unlock_irqrestore(&mut (*cif).lock, irqflags);
    dev_dbg((*cif).dev, b"send cmd %d hw: 0x%x flag: 0x%x cmd: 0x%x parm: 0x%x ret: 0x%x 0x%x CMDE: %d DATF: %d failed %d\n\0".as_ptr() as *const c_char,
        (*cif).cmdcnt, (cmdport as isize - hwport as isize) as c_int, flags, cmd, parm,
        if ret.is_null(){0}else{(*ret).retlongs[0]}, if ret.is_null(){0}else{(*ret).retlongs[1]},
        if cmdport.is_null(){0}else{IS_CMDE(cmdport) as c_int}, if cmdport.is_null(){0}else{IS_DATF(cmdport) as c_int}, err);
}

unsafe fn setmixer(cif:*mut cmdif, num:i16, rval:u16, lval:u16) -> c_int {
    let mut rptr = CMDRET_ZERO; let mut i = 0;
    dev_dbg((*cif).dev, b"sent mixer %d: 0x%x 0x%x\n\0".as_ptr() as *const c_char, num as c_int, rval as c_uint, lval as c_uint);
    loop {
        SEND_SDGV(cif, num, num, rval, lval); SEND_RDGV(cif, num, num, &mut rptr);
        if rptr.retwords[0] == lval && rptr.retwords[1] == rval { return 0; }
        let old = i; i += 1; if !(old < MAX_WRITE_RETRY as c_int) { break; }
    }
    dev_dbg((*cif).dev, b"sent mixer failed\n\0".as_ptr() as *const c_char); -EIO
}

unsafe fn getsourcesink(cif:*mut cmdif, source:u8, sink:u8, a:*mut u8, b:*mut u8) -> c_int {
    let mut rptr = CMDRET_ZERO;
    if SEND_RSSV(cif, source, sink, &mut rptr) != 0 && SEND_RSSV(cif, source, sink, &mut rptr) != 0 { return -EIO; }
    *a = rptr.retbytes[0]; *b = rptr.retbytes[1];
    dev_dbg((*cif).dev, b"%s 0x%x 0x%x\n\0".as_ptr() as *const c_char, b"getsourcesink\0".as_ptr(), *a as c_uint, *b as c_uint);
    0
}

unsafe fn getpaths(cif:*mut cmdif, o:*mut u8) -> c_int {
    let mut src = [0u8; E2SINK_MAX]; let mut sink = [0u8; E2SINK_MAX]; let mut j = 0;
    for i in 0..E2SINK_MAX {
        getsourcesink(cif, i as u8, i as u8, &mut src[i], &mut sink[i]);
        if sink[i] < E2SINK_MAX as u8 { *o.add(j as usize) = sink[i]; j += 1; *o.add(j as usize) = i as u8; j += 1; }
    }
    j
}

unsafe fn getsamplerate(cif:*mut cmdif, intdec:*mut u8, rate:*mut c_uint) -> c_int {
    let mut p = [0u32,0u32]; let mut rptr = CMDRET_ZERO;
    for i in 0..2 {
        let s = *intdec.add(i);
        if s != 0xff {
            if SEND_RSRC(cif, s, &mut rptr) != 0 && SEND_RSRC(cif, s, &mut rptr) != 0 { return -EIO; }
            p[i] += rptr.retwords[1] as u32; p[i] *= rptr.retwords[2] as u32; p[i] += rptr.retwords[3] as u32; p[i] /= 65536;
        }
    }
    if p[0] != 0 { if p[1] != p[0] { dev_dbg((*cif).dev, b"rates differ %d %d\n\0".as_ptr() as *const c_char, p[0], p[1]); } *rate = p[0]; } else { *rate = p[1]; }
    dev_dbg((*cif).dev, b"getsampleformat %d %d %d\n\0".as_ptr() as *const c_char, *intdec as c_uint, *intdec.add(1) as c_uint, *rate);
    0
}

unsafe fn setsampleformat(cif:*mut cmdif, mixer:u8, id:u8, channels:u8, format:snd_pcm_format_t) -> c_int {
    dev_dbg((*cif).dev, b"%s mixer: %d id: %d channels: %d format: %d\n\0".as_ptr() as *const c_char, b"setsampleformat\0".as_ptr(), mixer as c_uint, id as c_uint, channels as c_uint, format);
    let ch = (channels == 1) as u8; let w = (snd_pcm_format_width(format) == 8) as u8; let sig = (snd_pcm_format_unsigned(format) != 0) as u8; let order = (snd_pcm_format_big_endian(format) != 0) as u8;
    if SEND_SETF(cif, mixer, w, ch as u32, order, sig, id) != 0 && SEND_SETF(cif, mixer, w, ch as u32, order, sig, id) != 0 {
        dev_dbg((*cif).dev, b"%s failed\n\0".as_ptr() as *const c_char, b"setsampleformat\0".as_ptr()); return -EIO;
    }
    0
}

unsafe fn setsamplerate(cif:*mut cmdif, mut intdec:*mut u8, rate:c_uint) -> c_int {
    let D:u32 = 48000; let mut M:u32 = if rate == 48000 { 47999 } else { rate } * 65536; let N:u32 = M % D; M /= D;
    dev_dbg((*cif).dev, b"%s intdec: %d,%d rate: %d\n\0".as_ptr() as *const c_char, b"setsamplerate\0".as_ptr(), *intdec as c_uint, *intdec.add(1) as c_uint, rate);
    let mut rptr = CMDRET_ZERO;
    for mut i in 0..2 {
        if *intdec != 0xff {
            loop {
                SEND_SSRC(cif, *intdec, D, M, N); SEND_RSRC(cif, *intdec, &mut rptr);
                if !(rptr.retwords[1] as u32 != D && rptr.retwords[2] as u32 != M && rptr.retwords[3] as u32 != N && { i += 1; i < MAX_WRITE_RETRY as usize }) { break; }
            }
            if i > MAX_WRITE_RETRY as usize { dev_dbg((*cif).dev, b"sent samplerate %d: %d failed\n\0".as_ptr() as *const c_char, *intdec as c_uint, rate); return -EIO; }
        }
        intdec = intdec.add(1);
    }
    0
}

unsafe fn getmixer(cif:*mut cmdif, num:i16, rval:*mut u16, lval:*mut u16) -> c_int {
    let mut rptr = CMDRET_ZERO;
    if SEND_RDGV(cif, num, num, &mut rptr) != 0 && SEND_RDGV(cif, num, num, &mut rptr) != 0 { return -EIO; }
    *rval = rptr.retwords[0]; *lval = rptr.retwords[1];
    dev_dbg((*cif).dev, b"got mixer %d: 0x%x 0x%x\n\0".as_ptr() as *const c_char, num as c_int, *rval as c_uint, *lval as c_uint);
    0
}

unsafe extern "C" fn riptide_handleirq(_irq:c_int, dev_id:*mut c_void) -> irqreturn_t {
    let chip = dev_id as *mut snd_riptide; let cif = (*chip).cif;
    if cif.is_null() { return IRQ_HANDLED; }
    let mut substream = [ptr::null_mut(); PLAYBACK_SUBSTREAMS + 1];
    for i in 0..PLAYBACK_SUBSTREAMS { substream[i] = (*chip).playback_substream[i]; }
    substream[PLAYBACK_SUBSTREAMS] = (*chip).capture_substream;
    for i in 0..PLAYBACK_SUBSTREAMS + 1 {
        if substream[i].is_null() || (*substream[i]).runtime.is_null() { continue; }
        let runtime = (*substream[i]).runtime; let data = (*runtime).private_data as *mut pcmhw; if data.is_null() { continue; }
        if (*data).state != ST_STOP {
            let mut pos:u32 = 0;
            for j in 0..(*data).pages as usize {
                let c = &mut *(*data).sgdbuf.add(j); let flag = le32_to_cpu(c.dwStat_Ctl);
                if (flag & EOB_STATUS) != 0 { pos += le32_to_cpu(c.dwSegLen); }
                if (flag & EOC_STATUS) != 0 { pos += le32_to_cpu(c.dwSegLen); }
                if (flag & EOS_STATUS) != 0 && (*data).state == ST_PLAY { (*data).state = ST_STOP; dev_err((*cif).dev, b"Riptide: DMA stopped unexpectedly\n\0".as_ptr() as *const c_char); }
                c.dwStat_Ctl = cpu_to_le32(flag & !(EOS_STATUS | EOB_STATUS | EOC_STATUS));
            }
            (*data).pointer += pos; pos += (*data).oldpos;
            if (*data).state != ST_STOP {
                let period_bytes = frames_to_bytes(runtime, (*runtime).period_size) as u32;
                let mut j = 0; if pos >= period_bytes { j += 1; while pos >= period_bytes { pos -= period_bytes; } }
                (*data).oldpos = pos; if j > 0 { snd_pcm_period_elapsed(substream[i]); }
            }
        }
    }
    IRQ_HANDLED
}

unsafe extern "C" fn riptide_suspend(dev:*mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card; let chip = (*card).private_data as *mut snd_riptide;
    (*chip).in_suspend = 1; snd_power_change_state(card, SNDRV_CTL_POWER_D3hot); snd_ac97_suspend((*chip).ac97); 0
}
unsafe extern "C" fn riptide_resume(dev:*mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card; let chip = (*card).private_data as *mut snd_riptide;
    snd_riptide_initialize(chip); snd_ac97_resume((*chip).ac97); snd_power_change_state(card, SNDRV_CTL_POWER_D0); (*chip).in_suspend = 0; 0
}
static riptide_pm: dev_pm_ops = dev_pm_ops{_private:[]};

unsafe fn try_to_load_firmware(cif:*mut cmdif, chip:*mut snd_riptide) -> c_int {
    let mut firmware = firmware_version{ret:CMDRET_ZERO};
    for i in 0..2 { WRITE_PORT_ULONG(ptr::addr_of_mut!((*(*cif).hwport).port[i].data1), 0); WRITE_PORT_ULONG(ptr::addr_of_mut!((*(*cif).hwport).port[i].data2), 0); }
    SET_GRESET((*cif).hwport); udelay(100); UNSET_GRESET((*cif).hwport); udelay(100);
    let mut timeout = 100000; while { timeout -= 1; timeout != 0 } { if IS_READY((*cif).hwport) && !IS_GERR((*cif).hwport) { break; } udelay(10); }
    if timeout == 0 {
        dev_err((*cif).dev, b"Riptide: device not ready, audio status: 0x%x ready: %d gerr: %d\n\0".as_ptr() as *const c_char, READ_AUDIO_STATUS((*cif).hwport), IS_READY((*cif).hwport) as c_int, IS_GERR((*cif).hwport) as c_int); return -EIO;
    } else {
        dev_dbg((*cif).dev, b"Riptide: audio status: 0x%x ready: %d gerr: %d\n\0".as_ptr() as *const c_char, READ_AUDIO_STATUS((*cif).hwport), IS_READY((*cif).hwport) as c_int, IS_GERR((*cif).hwport) as c_int);
    }
    SEND_GETV(cif, &mut firmware.ret);
    dev_dbg((*cif).dev, b"Firmware version: ASIC: %d CODEC %d AUXDSP %d PROG %d\n\0".as_ptr() as *const c_char, firmware.firmware.ASIC as c_uint, firmware.firmware.CODEC as c_uint, firmware.firmware.AUXDSP as c_uint, firmware.firmware.PROG as c_uint);
    if chip.is_null() { return 1; }
    for i in 0..FIRMWARE_VERSIONS { if memcmp(&firmware_versions[i] as *const _ as *const c_void, &firmware as *const _ as *const c_void, size_of::<firmware_version>()) == 0 { return 1; } }
    dev_dbg((*cif).dev, b"Writing Firmware\n\0".as_ptr() as *const c_char);
    if (*chip).fw_entry.is_null() {
        let err = request_firmware(&mut (*chip).fw_entry, b"riptide.hex\0".as_ptr() as *const c_char, &mut (*(*chip).pci).dev);
        if err != 0 { dev_err((*cif).dev, b"Riptide: Firmware not available %d\n\0".as_ptr() as *const c_char, err); return -EIO; }
    }
    let err = loadfirmware(cif, (*(*chip).fw_entry).data, (*(*chip).fw_entry).size);
    if err != 0 { dev_err((*cif).dev, b"Riptide: Could not load firmware %d\n\0".as_ptr() as *const c_char, err); return err; }
    (*chip).firmware = firmware; 1
}

unsafe fn riptide_reset(cif:*mut cmdif, chip:*mut snd_riptide) -> c_int {
    let mut rptr = CMDRET_ZERO; if cif.is_null() { return -EINVAL; }
    (*cif).cmdcnt=0; (*cif).cmdtime=0; (*cif).cmdtimemax=0; (*cif).cmdtimemin=0xffffffff; (*cif).errcnt=0; (*cif).is_reset=0;
    let mut tries = RESET_TRIES; loop { let err = try_to_load_firmware(cif, chip); if err < 0 { return err; } if !(! (err != 0) && { tries -= 1; tries != 0 }) { break; } }
    SEND_SACR(cif, 0, AC97_RESET); SEND_RACR(cif, AC97_RESET, &mut rptr);
    dev_dbg((*cif).dev, b"AC97: 0x%x 0x%x\n\0".as_ptr() as *const c_char, rptr.retlongs[0], rptr.retlongs[1]);
    SEND_PLST(cif,0); SEND_SLST(cif,0); SEND_DLST(cif,0); SEND_ALST(cif,0); SEND_KDMA(cif);
    writearm(cif,0x301F8,1,1); writearm(cif,0x301F4,1,1);
    SEND_LSEL(cif,MODEM_CMD,0,0,MODEM_INTDEC,MODEM_MERGER,MODEM_SPLITTER,MODEM_MIXER); setmixer(cif,MODEM_MIXER as i16,0x7fff,0x7fff); alloclbuspath(cif,ARM2LBUS_FIFO13,lbus_play_modem.as_ptr(),ptr::null_mut(),ptr::null_mut());
    SEND_LSEL(cif,FM_CMD,0,0,FM_INTDEC,FM_MERGER,FM_SPLITTER,FM_MIXER); setmixer(cif,FM_MIXER as i16,0x7fff,0x7fff); writearm(cif,0x30648 + FM_MIXER as u32 * 4,0x01,0x00000005); writearm(cif,0x301A8,0x02,0x00000002); writearm(cif,0x30264,0x08,0xffffffff); alloclbuspath(cif,OPL3_SAMPLE,lbus_play_opl3.as_ptr(),ptr::null_mut(),ptr::null_mut());
    SEND_SSRC(cif,I2S_INTDEC,48000,(I2S_RATE*65536)/48000,(I2S_RATE*65536)%48000); SEND_LSEL(cif,I2S_CMD0,0,0,I2S_INTDEC,I2S_MERGER,I2S_SPLITTER,I2S_MIXER); SEND_SI2S(cif,1);
    alloclbuspath(cif,ARM2LBUS_FIFO0,lbus_play_i2s.as_ptr(),ptr::null_mut(),ptr::null_mut()); alloclbuspath(cif,DIGITAL_MIXER_OUT0,lbus_play_out.as_ptr(),ptr::null_mut(),ptr::null_mut()); alloclbuspath(cif,DIGITAL_MIXER_OUT0,lbus_play_outhp.as_ptr(),ptr::null_mut(),ptr::null_mut());
    SET_AIACK((*cif).hwport); SET_AIE((*cif).hwport); SET_AIACK((*cif).hwport); (*cif).is_reset = 1; 0
}

static snd_riptide_playback: snd_pcm_hardware = snd_pcm_hardware{info:SNDRV_PCM_INFO_MMAP|SNDRV_PCM_INFO_INTERLEAVED|SNDRV_PCM_INFO_BLOCK_TRANSFER|SNDRV_PCM_INFO_PAUSE|SNDRV_PCM_INFO_MMAP_VALID,formats:SNDRV_PCM_FMTBIT_U8|SNDRV_PCM_FMTBIT_S16_LE|SNDRV_PCM_FMTBIT_S8|SNDRV_PCM_FMTBIT_U16_LE,rates:SNDRV_PCM_RATE_KNOT|SNDRV_PCM_RATE_8000_48000,rate_min:5500,rate_max:48000,channels_min:1,channels_max:2,buffer_bytes_max:64*1024,period_bytes_min:PAGE_SIZE>>1,period_bytes_max:PAGE_SIZE<<8,periods_min:2,periods_max:64,fifo_size:0};
static snd_riptide_capture: snd_pcm_hardware = snd_pcm_hardware{info:SNDRV_PCM_INFO_MMAP|SNDRV_PCM_INFO_INTERLEAVED|SNDRV_PCM_INFO_BLOCK_TRANSFER|SNDRV_PCM_INFO_PAUSE|SNDRV_PCM_INFO_MMAP_VALID,formats:SNDRV_PCM_FMTBIT_U8|SNDRV_PCM_FMTBIT_S16_LE|SNDRV_PCM_FMTBIT_S8|SNDRV_PCM_FMTBIT_U16_LE,rates:SNDRV_PCM_RATE_KNOT|SNDRV_PCM_RATE_8000_48000,rate_min:5500,rate_max:48000,channels_min:1,channels_max:2,buffer_bytes_max:64*1024,period_bytes_min:PAGE_SIZE>>1,period_bytes_max:PAGE_SIZE<<3,periods_min:2,periods_max:64,fifo_size:0};

unsafe extern "C" fn snd_riptide_pointer(substream:*mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream) as *mut snd_riptide; let runtime = (*substream).runtime; let data = get_pcmhwdev(substream); let cif = (*chip).cif; let mut rptr = CMDRET_ZERO;
    SEND_GPOS(cif,0,(*data).id,&mut rptr);
    if (*data).size != 0 && (*runtime).period_size != 0 {
        if rptr.retlongs[1] > (*data).pointer { bytes_to_frames(runtime, (rptr.retlongs[1] % (*data).size) as c_ulong) } else { bytes_to_frames(runtime, ((*data).pointer % (*data).size) as c_ulong) }
    } else { bytes_to_frames(runtime,0) }
}

unsafe extern "C" fn snd_riptide_trigger(substream:*mut snd_pcm_substream, cmd:c_int) -> c_int {
    let chip = snd_pcm_substream_chip(substream) as *mut snd_riptide; let data = get_pcmhwdev(substream); let cif = (*chip).cif; let mut rptr = CMDRET_ZERO;
    spin_lock(&mut (*chip).lock);
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => if ((*data).state & ST_PLAY) == 0 { SEND_SSTR(cif,(*data).id,(*data).sgdlist.addr); SET_AIE((*cif).hwport); (*data).state=ST_PLAY; if (*data).mixer != 0xff { setmixer(cif,(*data).mixer as i16,0x7fff,0x7fff); } (*chip).openstreams+=1; (*data).oldpos=0; (*data).pointer=0; },
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => { if (*data).mixer != 0xff { setmixer(cif,(*data).mixer as i16,0,0); } setmixer(cif,(*data).mixer as i16,0,0); SEND_KSTR(cif,(*data).id); (*data).state=ST_STOP; (*chip).openstreams-=1; let mut j=0; loop { let i = rptr.retlongs[1]; SEND_GPOS(cif,0,(*data).id,&mut rptr); udelay(1); if !(i != rptr.retlongs[1] && { j+=1; j < MAX_WRITE_RETRY as c_int }) { break; } } if j > MAX_WRITE_RETRY as c_int { dev_err((*cif).dev,b"Riptide: Could not stop stream!\0".as_ptr() as *const c_char); } },
        SNDRV_PCM_TRIGGER_PAUSE_PUSH => if ((*data).state & ST_PAUSE) == 0 { SEND_PSTR(cif,(*data).id); (*data).state |= ST_PAUSE; (*chip).openstreams-=1; },
        SNDRV_PCM_TRIGGER_PAUSE_RELEASE => if ((*data).state & ST_PAUSE) != 0 { SEND_SSTR(cif,(*data).id,(*data).sgdlist.addr); (*data).state &= !ST_PAUSE; (*chip).openstreams+=1; },
        _ => { spin_unlock(&mut (*chip).lock); return -EINVAL; }
    }
    spin_unlock(&mut (*chip).lock); 0
}

unsafe extern "C" fn snd_riptide_prepare(substream:*mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream) as *mut snd_riptide; let runtime = (*substream).runtime; let data = get_pcmhwdev(substream); let cif = (*chip).cif; let mut lbuspath: *const u8 = ptr::null(); let mut err = 0;
    if snd_BUG_ON(cif.is_null() || data.is_null()) != 0 { return -EINVAL; }
    spin_lock_irq(&mut (*chip).lock);
    let channels = (*runtime).channels; let format = (*runtime).format; let rate = (*runtime).rate;
    match channels { 1 => if rate == 48000 && format == SNDRV_PCM_FORMAT_S16_LE { lbuspath = (*data).paths.noconv; } else { lbuspath = (*data).paths.mono; }, 2 => if rate == 48000 && format == SNDRV_PCM_FORMAT_S16_LE { lbuspath = (*data).paths.noconv; } else { lbuspath = (*data).paths.stereo; }, _ => {} }
    if !(*data).sgdlist.area.is_null() {
        let mut size = frames_to_bytes(runtime, (*runtime).buffer_size) as c_uint; let period = frames_to_bytes(runtime, (*runtime).period_size) as c_uint; let mut f = PAGE_SIZE as c_uint;
        while (size + (f >> 1) - 1) <= (f << 7) && (f << 1) > period { f >>= 1; }
        let pages = DIV_ROUND_UP(size, f); (*data).size = size; (*data).pages = pages;
        let mut pt = 0; let mut j = 0; let mut p: *mut sgd = ptr::null_mut();
        for i in 0..pages as usize {
            let c = &mut *(*data).sgdbuf.add(i);
            if !p.is_null() { (*p).dwNextLink = cpu_to_le32(((*data).sgdlist.addr + (i * size_of::<sgd>()) as c_ulong) as u32); }
            c.dwNextLink = cpu_to_le32((*data).sgdlist.addr as u32);
            let ofs = j << PAGE_SHIFT; let addr = snd_pcm_sgbuf_get_addr(substream, ofs) + pt;
            c.dwSegPtrPhys = cpu_to_le32(addr); pt = (pt + f) % PAGE_SIZE as u32; if pt == 0 { j += 1; }
            c.dwSegLen = cpu_to_le32(f); c.dwStat_Ctl = cpu_to_le32(IEOB_ENABLE|IEOS_ENABLE|IEOC_ENABLE); p = c; size -= f;
        }
        (*(*data).sgdbuf.add(pages as usize)).dwSegLen = cpu_to_le32(size);
    }
    if !lbuspath.is_null() && lbuspath != (*data).lbuspath {
        if !(*data).lbuspath.is_null() { freelbuspath(cif, (*data).source, (*data).lbuspath); }
        alloclbuspath(cif, (*data).source, lbuspath, &mut (*data).mixer, (*data).intdec.as_mut_ptr()); (*data).lbuspath = lbuspath; (*data).rate = 0;
    }
    if (*data).rate != rate || (*data).format != format || (*data).channels != channels {
        (*data).rate=rate; (*data).format=format; (*data).channels=channels;
        if setsampleformat(cif,(*data).mixer,(*data).id,channels as u8,format) != 0 || setsamplerate(cif,(*data).intdec.as_mut_ptr(),rate) != 0 { err = -EIO; }
    }
    spin_unlock_irq(&mut (*chip).lock); err
}

unsafe extern "C" fn snd_riptide_hw_params(substream:*mut snd_pcm_substream, _hw_params:*mut snd_pcm_hw_params) -> c_int {
    let chip = snd_pcm_substream_chip(substream) as *mut snd_riptide; let data = get_pcmhwdev(substream); let sgdlist = &mut (*data).sgdlist;
    if !sgdlist.area.is_null() { snd_dma_free_pages(sgdlist); }
    let err = snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV, &mut (*(*chip).pci).dev, size_of::<sgd>() * (DESC_MAX_MASK + 1), sgdlist);
    if err < 0 { dev_err((*(*chip).card).dev, b"Riptide: failed to alloc %d dma bytes\n\0".as_ptr() as *const c_char, (size_of::<sgd>() * (DESC_MAX_MASK + 1)) as c_int); return err; }
    (*data).sgdbuf = sgdlist.area as *mut sgd; 0
}
unsafe extern "C" fn snd_riptide_hw_free(substream:*mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream) as *mut snd_riptide; let data = get_pcmhwdev(substream); let cif = (*chip).cif;
    if !cif.is_null() && !data.is_null() { if !(*data).lbuspath.is_null() { freelbuspath(cif,(*data).source,(*data).lbuspath); } (*data).lbuspath=ptr::null(); (*data).source=0xff; (*data).intdec=[0xff,0xff]; if !(*data).sgdlist.area.is_null() { snd_dma_free_pages(&mut (*data).sgdlist); (*data).sgdlist.area=ptr::null_mut(); } }
    0
}

unsafe extern "C" fn snd_riptide_playback_open(substream:*mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream) as *mut snd_riptide; let runtime = (*substream).runtime; let sub_num = (*substream).number as usize;
    (*chip).playback_substream[sub_num] = substream; (*runtime).hw = snd_riptide_playback;
    let data = kzalloc(size_of::<pcmhw>(), GFP_KERNEL) as *mut pcmhw; if data.is_null() { return -ENOMEM; }
    (*data).paths = lbus_play_paths[sub_num]; (*data).id = play_ids[sub_num]; (*data).source = play_sources[sub_num]; (*data).intdec=[0xff,0xff]; (*data).state=ST_STOP; (*runtime).private_data=data as *mut c_void;
    snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS)
}
unsafe extern "C" fn snd_riptide_capture_open(substream:*mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream) as *mut snd_riptide; let runtime = (*substream).runtime;
    (*chip).capture_substream = substream; (*runtime).hw = snd_riptide_capture;
    let data = kzalloc(size_of::<pcmhw>(), GFP_KERNEL) as *mut pcmhw; if data.is_null() { return -ENOMEM; }
    (*data).paths=lbus_rec_path; (*data).id=PADC; (*data).source=ACLNK2PADC; (*data).intdec=[0xff,0xff]; (*data).state=ST_STOP; (*runtime).private_data=data as *mut c_void;
    snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS)
}
unsafe extern "C" fn snd_riptide_playback_close(substream:*mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream) as *mut snd_riptide; let data = get_pcmhwdev(substream); let sub_num = (*substream).number as usize;
    (*(*substream).runtime).private_data=ptr::null_mut(); (*chip).playback_substream[sub_num]=ptr::null_mut(); kfree(data as *mut c_void); 0
}
unsafe extern "C" fn snd_riptide_capture_close(substream:*mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream) as *mut snd_riptide; let data = get_pcmhwdev(substream);
    (*(*substream).runtime).private_data=ptr::null_mut(); (*chip).capture_substream=ptr::null_mut(); kfree(data as *mut c_void); 0
}

static snd_riptide_playback_ops: snd_pcm_ops = snd_pcm_ops{open:Some(snd_riptide_playback_open),close:Some(snd_riptide_playback_close),hw_params:Some(snd_riptide_hw_params),hw_free:Some(snd_riptide_hw_free),prepare:Some(snd_riptide_prepare),trigger:Some(snd_riptide_trigger),pointer:Some(snd_riptide_pointer)};
static snd_riptide_capture_ops: snd_pcm_ops = snd_pcm_ops{open:Some(snd_riptide_capture_open),close:Some(snd_riptide_capture_close),hw_params:Some(snd_riptide_hw_params),hw_free:Some(snd_riptide_hw_free),prepare:Some(snd_riptide_prepare),trigger:Some(snd_riptide_trigger),pointer:Some(snd_riptide_pointer)};

unsafe fn snd_riptide_pcm(chip:*mut snd_riptide, device:c_int) -> c_int {
    let mut pcm:*mut snd_pcm = ptr::null_mut(); let err = snd_pcm_new((*chip).card,b"RIPTIDE\0".as_ptr() as *const c_char,device,PLAYBACK_SUBSTREAMS as c_int,1,&mut pcm); if err < 0 { return err; }
    snd_pcm_set_ops(pcm,SNDRV_PCM_STREAM_PLAYBACK,&snd_riptide_playback_ops); snd_pcm_set_ops(pcm,SNDRV_PCM_STREAM_CAPTURE,&snd_riptide_capture_ops);
    (*pcm).private_data=chip as *mut c_void; (*pcm).info_flags=0; strscpy((*pcm).name.as_mut_ptr(),b"RIPTIDE\0".as_ptr() as *const c_char); (*chip).pcm=pcm;
    snd_pcm_set_managed_buffer_all(pcm,SNDRV_DMA_TYPE_DEV_SG,&mut (*(*chip).pci).dev,64*1024,128*1024); 0
}

unsafe extern "C" fn snd_riptide_interrupt(irq:c_int, dev_id:*mut c_void) -> irqreturn_t {
    let chip = dev_id as *mut snd_riptide; let cif = (*chip).cif; let mut ret = IRQ_HANDLED;
    if !cif.is_null() { (*chip).received_irqs+=1; if IS_EOBIRQ((*cif).hwport)||IS_EOSIRQ((*cif).hwport)||IS_EOCIRQ((*cif).hwport) { (*chip).handled_irqs+=1; ret=IRQ_WAKE_THREAD; } if !(*chip).rmidi.is_null() && IS_MPUIRQ((*cif).hwport) { (*chip).handled_irqs+=1; snd_mpu401_uart_interrupt(irq,(*(*chip).rmidi).private_data); } SET_AIACK((*cif).hwport); }
    ret
}

unsafe extern "C" fn snd_riptide_codec_write(ac97:*mut snd_ac97, reg:u16, val:u16) {
    let chip = (*ac97).private_data as *mut snd_riptide; let cif = (*chip).cif; let mut rptr = CMDRET_ZERO; let mut i=0;
    if snd_BUG_ON(cif.is_null()) != 0 { return; }
    do_loop_write: loop { SEND_SACR(cif,val,reg); SEND_RACR(cif,reg,&mut rptr); if !(rptr.retwords[1] != val && { i+=1; i < MAX_WRITE_RETRY as c_int }) { break do_loop_write; } }
    if i > MAX_WRITE_RETRY as c_int { dev_dbg((*cif).dev,b"Write AC97 reg failed\n\0".as_ptr() as *const c_char); }
}
unsafe extern "C" fn snd_riptide_codec_read(ac97:*mut snd_ac97, reg:u16) -> u16 {
    let chip = (*ac97).private_data as *mut snd_riptide; let cif = (*chip).cif; let mut rptr = CMDRET_ZERO; if snd_BUG_ON(cif.is_null()) != 0 { return 0; }
    if SEND_RACR(cif,reg,&mut rptr) != 0 { SEND_RACR(cif,reg,&mut rptr); }
    dev_dbg((*cif).dev,b"Read AC97 reg 0x%x got 0x%x\n\0".as_ptr() as *const c_char,reg as c_uint,rptr.retwords[1] as c_uint); rptr.retwords[1]
}

unsafe fn snd_riptide_initialize(chip:*mut snd_riptide) -> c_int {
    if snd_BUG_ON(chip.is_null()) != 0 { return -EINVAL; }
    let mut cif = (*chip).cif;
    if cif.is_null() { cif = kzalloc(size_of::<cmdif>(),GFP_KERNEL) as *mut cmdif; if cif.is_null(){return -ENOMEM;} (*cif).dev=(*(*chip).card).dev; (*cif).hwport=(*chip).port as *mut riptideport; spin_lock_init(&mut (*cif).lock); (*chip).cif=cif; }
    (*cif).is_reset=0; let err = riptide_reset(cif,chip); if err != 0 { return err; }
    match (*chip).device_id { 0x4310|0x4320|0x4330 => { dev_dbg((*cif).dev,b"Modem enable?\n\0".as_ptr() as *const c_char); SEND_SETDPLL(cif); }, _=>{} }
    dev_dbg((*cif).dev,b"Enabling MPU IRQs\n\0".as_ptr() as *const c_char); if !(*chip).rmidi.is_null(){SET_EMPUIRQ((*cif).hwport);} err
}

unsafe extern "C" fn snd_riptide_free(card:*mut snd_card) {
    let chip = (*card).private_data as *mut snd_riptide; let cif = (*chip).cif;
    if !cif.is_null(){SET_GRESET((*cif).hwport); udelay(100); UNSET_GRESET((*cif).hwport); kfree((*chip).cif as *mut c_void);}
    release_firmware((*chip).fw_entry);
}

unsafe fn snd_riptide_create(card:*mut snd_card, pci:*mut pci_dev) -> c_int {
    let chip = (*card).private_data as *mut snd_riptide; let mut err = pcim_enable_device(pci); if err < 0 { return err; }
    spin_lock_init(&mut (*chip).lock); (*chip).card=card; (*chip).pci=pci; (*chip).irq=-1; (*chip).openstreams=0; (*chip).port=pci_resource_start(pci,0); (*chip).received_irqs=0; (*chip).handled_irqs=0; (*chip).cif=ptr::null_mut(); (*card).private_free=Some(snd_riptide_free);
    err=pcim_request_all_regions(pci,b"RIPTIDE\0".as_ptr() as *const c_char); if err < 0 { return err; }
    let hwport = (*chip).port as *mut riptideport; UNSET_AIE(hwport);
    if devm_request_threaded_irq(&mut (*pci).dev,(*pci).irq,Some(snd_riptide_interrupt),Some(riptide_handleirq),IRQF_SHARED,KBUILD_MODNAME,chip as *mut c_void) != 0 { dev_err(&mut (*pci).dev,b"Riptide: unable to grab IRQ %d\n\0".as_ptr() as *const c_char,(*pci).irq); return -EBUSY; }
    (*chip).irq=(*pci).irq; (*card).sync_irq=(*chip).irq; (*chip).device_id=(*pci).device; pci_set_master(pci); snd_riptide_initialize(chip)
}

unsafe extern "C" fn snd_riptide_proc_read(entry:*mut snd_info_entry, buffer:*mut snd_info_buffer) {
    let chip = (*entry).private_data as *mut snd_riptide; if chip.is_null(){return;} let mut p=[0u8;256]; let mut rval:u16=0; let mut lval:u16=0; let mut rate:u32=0;
    snd_iprintf(buffer,b"%s\n\n\0".as_ptr() as *const c_char,(*(*chip).card).longname.as_ptr());
    snd_iprintf(buffer,b"Device ID: 0x%x\nReceived IRQs: (%ld)%ld\nPorts:\0".as_ptr() as *const c_char,(*chip).device_id as c_uint,(*chip).handled_irqs,(*chip).received_irqs);
    for i in (0..64).step_by(4){snd_iprintf(buffer,b"%c%02x: %08x\0".as_ptr() as *const c_char,if i%16!=0{b' ' as c_int}else{b'\n' as c_int},i,inl((*chip).port+i as c_ulong));}
    let cif=(*chip).cif;
    if !cif.is_null(){snd_iprintf(buffer,b"\nVersion: ASIC: %d CODEC: %d AUXDSP: %d PROG: %d\0".as_ptr() as *const c_char,(*chip).firmware.firmware.ASIC as c_uint,(*chip).firmware.firmware.CODEC as c_uint,(*chip).firmware.firmware.AUXDSP as c_uint,(*chip).firmware.firmware.PROG as c_uint); snd_iprintf(buffer,b"\nDigital mixer:\0".as_ptr() as *const c_char); for i in 0..12{getmixer(cif,i,&mut rval,&mut lval); snd_iprintf(buffer,b"\n %d: %d %d\0".as_ptr() as *const c_char,i,rval as c_uint,lval as c_uint);} snd_iprintf(buffer,b"\nARM Commands num: %d failed: %d time: %d max: %d min: %d\0".as_ptr() as *const c_char,(*cif).cmdcnt,(*cif).errcnt,(*cif).cmdtime,(*cif).cmdtimemax,(*cif).cmdtimemin);}
    snd_iprintf(buffer,b"\nOpen streams %d:\n\0".as_ptr() as *const c_char,(*chip).openstreams);
    for i in 0..PLAYBACK_SUBSTREAMS{ if (*chip).playback_substream[i].is_null() || (*(*chip).playback_substream[i]).runtime.is_null(){continue;} let data=(*(*(*chip).playback_substream[i]).runtime).private_data as *mut pcmhw; if !data.is_null(){snd_iprintf(buffer,b"stream: %d mixer: %d source: %d (%d,%d)\n\0".as_ptr() as *const c_char,(*data).id as c_uint,(*data).mixer as c_uint,(*data).source as c_uint,(*data).intdec[0] as c_uint,(*data).intdec[1] as c_uint); if getsamplerate(cif,(*data).intdec.as_mut_ptr(),&mut rate)==0{snd_iprintf(buffer,b"rate: %d\n\0".as_ptr() as *const c_char,rate);}}}
    if !(*chip).capture_substream.is_null() && !(*(*chip).capture_substream).runtime.is_null(){let data=(*(*(*chip).capture_substream).runtime).private_data as *mut pcmhw; if !data.is_null(){snd_iprintf(buffer,b"stream: %d mixer: %d source: %d (%d,%d)\n\0".as_ptr() as *const c_char,(*data).id as c_uint,(*data).mixer as c_uint,(*data).source as c_uint,(*data).intdec[0] as c_uint,(*data).intdec[1] as c_uint); if getsamplerate(cif,(*data).intdec.as_mut_ptr(),&mut rate)==0{snd_iprintf(buffer,b"rate: %d\n\0".as_ptr() as *const c_char,rate);}}}
    snd_iprintf(buffer,b"Paths:\n\0".as_ptr() as *const c_char); let mut i=getpaths(cif,p.as_mut_ptr()); while i>=2{i-=2;snd_iprintf(buffer,b"%x->%x \0".as_ptr() as *const c_char,p[i as usize] as c_uint,p[i as usize+1] as c_uint);} snd_iprintf(buffer,b"\n\0".as_ptr() as *const c_char);
}
unsafe fn snd_riptide_proc_init(chip:*mut snd_riptide){snd_card_ro_proc_new((*chip).card,b"riptide\0".as_ptr() as *const c_char,chip as *mut c_void,Some(snd_riptide_proc_read));}
unsafe fn snd_riptide_mixer(chip:*mut snd_riptide)->c_int{
    let mut pbus:*mut snd_ac97_bus=ptr::null_mut(); let mut ac97:snd_ac97_template=zeroed(); static ops:snd_ac97_bus_ops=snd_ac97_bus_ops{write:Some(snd_riptide_codec_write),read:Some(snd_riptide_codec_read)};
    ac97.private_data=chip as *mut c_void; ac97.scaps=AC97_SCAP_SKIP_MODEM; let mut err=snd_ac97_bus((*chip).card,0,&ops,chip as *mut c_void,&mut pbus); if err<0{return err;} (*chip).ac97_bus=pbus; ac97.pci=(*chip).pci; err=snd_ac97_mixer(pbus,&mut ac97,&mut (*chip).ac97); if err<0{return err;} err
}

unsafe extern "C" fn snd_riptide_joystick_probe(pci:*mut pci_dev,_id:*const pci_device_id)->c_int{
    static mut dev:c_int=0; let mut ret:c_int;
    if dev>=SNDRV_CARDS as c_int{return -ENODEV;} if !enable[dev as usize]{ret=-ENOENT; dev+=1; return ret;} if joystick_port[dev as usize]==0{ret=0; dev+=1; return ret;}
    let gameport=gameport_allocate_port(); if gameport.is_null(){ret=-ENOMEM; dev+=1; return ret;}
    if request_region(joystick_port[dev as usize],8,b"Riptide gameport\0".as_ptr() as *const c_char).is_null(){dev_err(&mut (*pci).dev,b"Riptide: cannot grab gameport 0x%x\n\0".as_ptr() as *const c_char,joystick_port[dev as usize]); gameport_free_port(gameport); ret=-EBUSY; dev+=1; return ret;}
    (*gameport).io=joystick_port[dev as usize]; gameport_register_port(gameport); pci_set_drvdata(pci,gameport as *mut c_void); ret=0; dev+=1; ret
}
unsafe extern "C" fn snd_riptide_joystick_remove(pci:*mut pci_dev){let gameport=pci_get_drvdata(pci) as *mut gameport; if !gameport.is_null(){release_region((*gameport).io,8); gameport_unregister_port(gameport);}}

unsafe fn __snd_card_riptide_probe(pci:*mut pci_dev,_pci_id:*const pci_device_id)->c_int{
    static mut dev:c_int=0; let mut card:*mut snd_card=ptr::null_mut(); let mut err:c_int; if dev>=SNDRV_CARDS as c_int{return -ENODEV;} if !enable[dev as usize]{dev+=1; return -ENOENT;}
    err=snd_devm_card_new(&mut (*pci).dev,index[dev as usize],id[dev as usize],THIS_MODULE,size_of::<snd_riptide>(),&mut card); if err<0{return err;} let chip=(*card).private_data as *mut snd_riptide;
    err=snd_riptide_create(card,pci); if err<0{return err;} err=snd_riptide_pcm(chip,0); if err<0{return err;} err=snd_riptide_mixer(chip); if err<0{return err;}
    let mut val:c_ushort=LEGACY_ENABLE_ALL; if opl3_port[dev as usize]!=0{val|=LEGACY_ENABLE_FM;} if SUPPORT_JOYSTICK && joystick_port[dev as usize]!=0{val|=LEGACY_ENABLE_GAMEPORT;} if mpu_port[dev as usize]!=0{val|=LEGACY_ENABLE_MPU_INT|LEGACY_ENABLE_MPU;} val|=(((*chip).irq<<4)&0xf0) as c_ushort; pci_write_config_word((*chip).pci,PCI_EXT_Legacy_Mask,val);
    if mpu_port[dev as usize]!=0{val=mpu_port[dev as usize] as c_ushort; pci_write_config_word((*chip).pci,PCI_EXT_MPU_Base,val); err=snd_mpu401_uart_new(card,0,MPU401_HW_RIPTIDE,val,MPU401_INFO_IRQ_HOOK,-1,&mut (*chip).rmidi); if err<0{dev_warn(&mut (*pci).dev,b"Riptide: Can't Allocate MPU at 0x%x\n\0".as_ptr() as *const c_char,val as c_uint);}else{(*chip).mpuaddr=val;}}
    if opl3_port[dev as usize]!=0{val=opl3_port[dev as usize] as c_ushort; pci_write_config_word((*chip).pci,PCI_EXT_FM_Base,val); err=snd_opl3_create(card,val,val+2,OPL3_HW_RIPTIDE,0,&mut (*chip).opl3); if err<0{dev_warn(&mut (*pci).dev,b"Riptide: Can't Allocate OPL3 at 0x%x\n\0".as_ptr() as *const c_char,val as c_uint);}else{(*chip).opladdr=val; err=snd_opl3_hwdep_new((*chip).opl3,0,1,ptr::null_mut()); if err<0{dev_warn(&mut (*pci).dev,b"Riptide: Can't Allocate OPL3-HWDEP\n\0".as_ptr() as *const c_char);}}}
    if SUPPORT_JOYSTICK && joystick_port[dev as usize]!=0{val=joystick_port[dev as usize] as c_ushort; pci_write_config_word((*chip).pci,PCI_EXT_Game_Base,val); (*chip).gameaddr=val;}
    strscpy((*card).driver.as_mut_ptr(),b"RIPTIDE\0".as_ptr() as *const c_char); strscpy((*card).shortname.as_mut_ptr(),b"Riptide\0".as_ptr() as *const c_char);
    if SUPPORT_JOYSTICK { scnprintf((*card).longname.as_mut_ptr(),(*card).longname.len(),b"%s at 0x%lx, irq %i mpu 0x%x opl3 0x%x gameport 0x%x\0".as_ptr() as *const c_char,(*card).shortname.as_ptr(),(*chip).port,(*chip).irq,(*chip).mpuaddr as c_uint,(*chip).opladdr as c_uint,(*chip).gameaddr as c_uint); } else { scnprintf((*card).longname.as_mut_ptr(),(*card).longname.len(),b"%s at 0x%lx, irq %i mpu 0x%x opl3 0x%x\0".as_ptr() as *const c_char,(*card).shortname.as_ptr(),(*chip).port,(*chip).irq,(*chip).mpuaddr as c_uint,(*chip).opladdr as c_uint); }
    snd_riptide_proc_init(chip); err=snd_card_register(card); if err<0{return err;} pci_set_drvdata(pci,card as *mut c_void); dev+=1; 0
}
unsafe extern "C" fn snd_card_riptide_probe(pci:*mut pci_dev,pci_id:*const pci_device_id)->c_int{snd_card_free_on_error(&mut (*pci).dev,__snd_card_riptide_probe(pci,pci_id))}

static mut driver: pci_driver = pci_driver{name:ptr::null(),id_table:snd_riptide_ids.as_ptr(),probe:Some(snd_card_riptide_probe),remove:None,driver:pci_driver_inner{pm:&riptide_pm}};
static mut joystick_driver: pci_driver = pci_driver{name:ptr::null(),id_table:snd_riptide_joystick_ids.as_ptr(),probe:Some(snd_riptide_joystick_probe),remove:Some(snd_riptide_joystick_remove),driver:pci_driver_inner{pm:ptr::null()}};

unsafe extern "C" fn alsa_card_riptide_init() -> c_int {
    driver.name=KBUILD_MODNAME; joystick_driver.name=b"riptide-joystick\0".as_ptr() as *const c_char;
    let mut err=pci_register_driver(&mut driver); if err<0{return err;}
    if SUPPORT_JOYSTICK { err=pci_register_driver(&mut joystick_driver); if err<0{pci_unregister_driver(&mut driver);} }
    err
}
unsafe extern "C" fn alsa_card_riptide_exit() {
    pci_unregister_driver(&mut driver);
    if SUPPORT_JOYSTICK { pci_unregister_driver(&mut joystick_driver); }
}

// module_init(alsa_card_riptide_init);
// module_exit(alsa_card_riptide_exit);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
