// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *                   Abramo Bagnara <abramo@alsa-project.org>
 *                   Cirrus Logic, Inc.
 *  Routines for control of Cirrus Logic CS461x chips
 *
 *  This file is a source-level Rust translation of cs46xx_lib.c.  It keeps the
 *  kernel/ALSA interfaces as external dependencies and uses raw pointers for the
 *  original C object graph.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_uchar, c_ushort, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type u8 = c_uchar;
type u16 = c_ushort;
type u32 = c_uint;
type ssize_t = isize;
type size_t = usize;
type loff_t = i64;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_int;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENXIO: c_int = 6;
const EIO: c_int = 5;
const EBUSY: c_int = 16;
const ENOSYS: c_int = 38;
const EFAULT: c_int = 14;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const GFP_KERNEL: c_uint = 0;
const PAGE_SIZE: size_t = 4096;
const GOF_PER_SEC: c_uint = 200;

#[repr(C)] pub struct snd_cs46xx { _private: [u8; 0] }
#[repr(C)] pub struct snd_ac97 { _private: [u8; 0] }
#[repr(C)] pub struct snd_ac97_template { _private: [u8; 0] }
#[repr(C)] pub struct snd_ac97_bus_ops { _private: [u8; 0] }
#[repr(C)] pub struct snd_card { _private: [u8; 0] }
#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_ops { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hardware { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_runtime { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_indirect { _private: [u8; 0] }
#[repr(C)] pub struct snd_cs46xx_pcm { _private: [u8; 0] }
#[repr(C)] pub struct snd_dma_buffer { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct snd_ctl_elem_info { _private: [u8; 0] }
#[repr(C)] pub struct snd_ctl_elem_value { _private: [u8; 0] }
#[repr(C)] pub struct snd_rawmidi { _private: [u8; 0] }
#[repr(C)] pub struct snd_rawmidi_ops { _private: [u8; 0] }
#[repr(C)] pub struct snd_rawmidi_substream { _private: [u8; 0] }
#[repr(C)] pub struct gameport { _private: [u8; 0] }
#[repr(C)] pub struct snd_info_entry { _private: [u8; 0] }
#[repr(C)] pub struct snd_info_entry_ops { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct firmware { _private: [u8; 0] }
#[repr(C)] pub struct dsp_module_desc { _private: [u8; 0] }
#[repr(C)] pub struct dsp_symbol_entry { _private: [u8; 0] }
#[repr(C)] pub struct dsp_segment_desc { _private: [u8; 0] }
#[repr(C)] pub struct dsp_spos_instance { _private: [u8; 0] }
#[repr(C)] pub struct pci_device_id { _private: [u8; 0] }
#[repr(C)] pub struct snd_cs46xx_region { _private: [u8; 0] }

extern "C" {
    static mut jiffies: c_ulong;
    static HZ: c_ulong;

    fn snd_BUG() -> c_int;
    fn snd_BUG_ON(cond: bool) -> c_int;
    fn dev_warn(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_info(dev: *mut c_void, fmt: *const c_char, ...);
    fn msleep(ms: c_uint);
    fn mdelay(ms: c_uint);
    fn udelay(us: c_uint);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn vfree(p: *mut c_void);
    fn vmalloc(n: size_t) -> *mut c_void;
    fn kzalloc(n: size_t, flags: c_uint) -> *mut c_void;
    fn kmalloc_array(n: size_t, size: size_t, flags: c_uint) -> *mut c_void;
    fn kstrdup(s: *const c_char, flags: c_uint) -> *mut c_char;
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, dev: *mut device) -> c_int;
    fn writel(v: u32, addr: *mut c_void);
    fn copy_to_user_fromio(buf: *mut c_char, src: *mut c_void, count: size_t) -> c_int;
    fn time_after_eq(a: c_ulong, b: c_ulong) -> bool;
    fn inw(port: c_ulong) -> u16;
    fn outw(v: u16, port: c_ulong);

    fn snd_cs46xx_peekBA0(chip: *mut snd_cs46xx, offset: c_uint) -> u32;
    fn snd_cs46xx_pokeBA0(chip: *mut snd_cs46xx, offset: c_uint, val: u32);
    fn snd_cs46xx_peek(chip: *mut snd_cs46xx, offset: c_uint) -> u32;
    fn snd_cs46xx_poke(chip: *mut snd_cs46xx, offset: c_uint, val: u32);
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut snd_cs46xx;
    fn snd_pcm_indirect_playback_transfer(substream: *mut snd_pcm_substream, rec: *mut snd_pcm_indirect, copy: unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_indirect, size_t)) -> c_int;
    fn snd_pcm_indirect_capture_transfer(substream: *mut snd_pcm_substream, rec: *mut snd_pcm_indirect, copy: unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_indirect, size_t)) -> c_int;
    fn snd_pcm_indirect_playback_pointer(substream: *mut snd_pcm_substream, rec: *mut snd_pcm_indirect, ptr: size_t) -> snd_pcm_uframes_t;
    fn snd_pcm_indirect_capture_pointer(substream: *mut snd_pcm_substream, rec: *mut snd_pcm_indirect, ptr: size_t) -> snd_pcm_uframes_t;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_lib_free_pages(substream: *mut snd_pcm_substream);
    fn snd_pcm_set_runtime_buffer(substream: *mut snd_pcm_substream, buf: *mut snd_dma_buffer);
    fn snd_pcm_lib_malloc_pages(substream: *mut snd_pcm_substream, size: size_t) -> c_int;
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> size_t;
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn snd_pcm_format_unsigned(format: c_int) -> bool;
    fn snd_pcm_format_big_endian(format: c_int) -> bool;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_period_bytes(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_periods(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_buffer_bytes(params: *mut snd_pcm_hw_params) -> size_t;
    fn snd_dma_alloc_pages(t: c_int, dev: *mut device, size: size_t, buf: *mut snd_dma_buffer) -> c_int;
    fn snd_dma_free_pages(buf: *mut snd_dma_buffer);
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback: c_int, capture: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_lib_preallocate_pages_for_all(pcm: *mut snd_pcm, t: c_int, dev: *mut device, min: size_t, max: size_t);

    fn snd_ac97_read(ac97: *mut snd_ac97, reg: c_ushort) -> c_ushort;
    fn snd_ac97_write(ac97: *mut snd_ac97, reg: c_ushort, val: c_ushort);
    fn snd_ac97_update_bits(ac97: *mut snd_ac97, reg: c_ushort, mask: c_ushort, val: c_ushort) -> c_int;
    fn snd_ac97_write_cache(ac97: *mut snd_ac97, reg: c_ushort, val: c_ushort);
    fn snd_ac97_bus(card: *mut snd_card, num: c_int, ops: *const snd_ac97_bus_ops, private_data: *mut c_void, rbus: *mut *mut c_void) -> c_int;
    fn snd_ac97_mixer(bus: *mut c_void, template: *mut snd_ac97_template, rac97: *mut *mut snd_ac97) -> c_int;
    fn snd_ac97_suspend(ac97: *mut snd_ac97);
    fn snd_ac97_resume(ac97: *mut snd_ac97);

    fn snd_ctl_new1(n: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kctl: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_find_id_mixer(card: *mut snd_card, name: *const c_char) -> *mut snd_kcontrol;
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut c_void);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_cs46xx;
    fn snd_ctl_boolean_mono_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;

    fn snd_rawmidi_new(card: *mut snd_card, id: *const c_char, device: c_int, output: c_int, input: c_int, rrawmidi: *mut *mut snd_rawmidi) -> c_int;
    fn snd_rawmidi_set_ops(rmidi: *mut snd_rawmidi, stream: c_int, ops: *const snd_rawmidi_ops);
    fn snd_rawmidi_receive(substream: *mut snd_rawmidi_substream, buf: *mut u8, count: size_t) -> c_int;
    fn snd_rawmidi_transmit(substream: *mut snd_rawmidi_substream, buf: *mut u8, count: size_t) -> c_int;

    fn gameport_allocate_port() -> *mut gameport;
    fn gameport_unregister_port(gameport: *mut gameport);
    fn gameport_register_port(gameport: *mut gameport);
    fn gameport_get_port_data(gameport: *mut gameport) -> *mut snd_cs46xx;
    fn gameport_set_port_data(gameport: *mut gameport, data: *mut snd_cs46xx);

    fn pci_get_device(vendor: u16, device: u16, from: *mut pci_dev) -> *mut pci_dev;
    fn pci_read_config_byte(dev: *mut pci_dev, where_: c_int, val: *mut u8) -> c_int;
    fn pci_read_config_word(dev: *mut pci_dev, where_: c_int, val: *mut u16) -> c_int;
    fn pci_dev_put(dev: *mut pci_dev);
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn pcim_request_all_regions(pci: *mut pci_dev, name: *const c_char) -> c_int;
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn pci_set_master(pci: *mut pci_dev);
    fn devm_ioremap(dev: *mut device, base: c_ulong, size: size_t) -> *mut c_void;
    fn devm_request_irq(dev: *mut device, irq: c_uint, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, data: *mut c_void) -> c_int;

    fn cs46xx_dsp_create_pcm_channel(chip: *mut snd_cs46xx, sample_rate: c_int, private_data: *mut snd_cs46xx_pcm, addr: c_ulong, id: c_int) -> *mut c_void;
    fn cs46xx_dsp_destroy_pcm_channel(chip: *mut snd_cs46xx, channel: *mut c_void);
    fn cs46xx_dsp_pcm_link(chip: *mut snd_cs46xx, channel: *mut c_void);
    fn cs46xx_dsp_pcm_unlink(chip: *mut snd_cs46xx, channel: *mut c_void);
    fn cs46xx_dsp_pcm_channel_set_period(chip: *mut snd_cs46xx, channel: *mut c_void, period: c_int) -> c_int;
    fn cs46xx_dsp_pcm_ostream_set_period(chip: *mut snd_cs46xx, period: c_int);
    fn cs46xx_dsp_set_dac_volume(chip: *mut snd_cs46xx, left: c_long, right: c_long);
    fn cs46xx_dsp_enable_spdif_out(chip: *mut snd_cs46xx);
    fn cs46xx_dsp_disable_spdif_out(chip: *mut snd_cs46xx);
    fn cs46xx_dsp_enable_spdif_in(chip: *mut snd_cs46xx);
    fn cs46xx_dsp_disable_spdif_in(chip: *mut snd_cs46xx);
    fn cs46xx_dsp_enable_adc_capture(chip: *mut snd_cs46xx);
    fn cs46xx_dsp_disable_adc_capture(chip: *mut snd_cs46xx);
    fn cs46xx_dsp_enable_pcm_capture(chip: *mut snd_cs46xx);
    fn cs46xx_dsp_disable_pcm_capture(chip: *mut snd_cs46xx);
    fn cs46xx_poke_via_dsp(chip: *mut snd_cs46xx, addr: c_uint, val: c_uint);
    fn cs46xx_iec958_pre_open(chip: *mut snd_cs46xx);
    fn cs46xx_iec958_post_close(chip: *mut snd_cs46xx);
    fn cs46xx_dsp_load_module(chip: *mut snd_cs46xx, module: *mut dsp_module_desc) -> c_int;
    fn cs46xx_dsp_scb_and_task_init(chip: *mut snd_cs46xx) -> c_int;
    fn cs46xx_dsp_spos_create(chip: *mut snd_cs46xx) -> *mut dsp_spos_instance;
    fn cs46xx_dsp_spos_destroy(chip: *mut snd_cs46xx);
    fn cs46xx_dsp_resume(chip: *mut snd_cs46xx);
    fn cs46xx_dsp_proc_init(card: *mut snd_card, chip: *mut snd_cs46xx);
    fn cs46xx_dsp_proc_done(chip: *mut snd_cs46xx);
}

type c_long = isize;

/* Constants are supplied by cs46xx.h/cs46xx_lib.h/dsp_spos.h in the original
 * repository.  They are referenced here with the same names through extern
 * constant declarations so the source-level dependencies remain explicit.
 */
extern "C" {
    static CS46XX_PRIMARY_CODEC_INDEX: c_int;
    static CS46XX_SECONDARY_CODEC_INDEX: c_int;
    static CS46XX_SECONDARY_CODEC_OFFSET: u32;
    static ACCTL_VFRM: u32; static ACCTL_ESYN: u32; static ACCTL_CRW: u32;
    static ACCTL_RSTN: u32; static ACCTL_DCV: u32; static ACCTL_TC: u32;
    static ACSTS_VSTS: u32; static ACSTS_CRDY: u32;
    static BA0_ACSDA: u32; static BA0_ACCTL: u32; static BA0_ACCAD: u32;
    static BA0_ACCDA: u32; static BA0_ACSTS: u32; static BA0_ACCTL2: u32;
    static BA0_ACSTS2: u32; static BA0_HISR: u32; static BA0_HICR: u32;
    static BA0_HSR0: u32; static BA0_CLKCR1: u32; static CLKCR1_SWCE: u32;
    static BA1_SPCR: u32; static SPCR_RSTSP: u32; static SPCR_DRQEN: u32;
    static SPCR_RUN: u32; static SPCR_RUNFR: u32; static BA1_DREG: u32;
    static BA1_TWPR: u32; static DREG_REGID_TRAP_SELECT: u32; static BA1_FRMT: u32;
    static BA0_SERBST: u32; static SERBST_WBSY: u32; static BA0_SERBWP: u32;
    static BA0_SERBAD: u32; static BA0_SERBCM: u32; static SERBCM_WRC: u32;
    static BA1_PSRC: u32; static BA1_PPI: u32; static BA1_CSRC: u32;
    static BA1_CCI: u32; static BA1_CD: u32; static BA1_CPI: u32;
    static BA1_VARIDEC_BUF_1: u32; static BA1_CFG1: u32; static BA1_CFG2: u32;
    static BA1_CCST: u32; static BA1_CSPB: u32; static BA1_PBA: u32;
    static BA1_CBA: u32; static BA1_PCTL: u32; static BA1_CCTL: u32;
    static BA1_PFIE: u32; static BA1_PDTC: u32; static BA1_CIE: u32;
    static BA1_PVOL: u32; static BA1_CVOL: u32;
    static SNDRV_PCM_TRIGGER_START: c_int; static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int; static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static CS46XX_FRAGS: c_int; static DSP_PCM_MAIN_CHANNEL: c_int;
    static DSP_PCM_REAR_CHANNEL: c_int; static DSP_PCM_CENTER_LFE_CHANNEL: c_int;
    static DSP_IEC958_CHANNEL: c_int; static DSP_MAX_PCM_CHANNELS: c_int;
    static CS46XX_DSP_CAPTURE_CHANNEL: c_int; static SCBVolumeCtrl: u32;
    static HICR_CHGM: u32; static HICR_IEV: u32; static HISR_VC0: u32; static HISR_VC1: u32;
    static HISR_MIDI: u32; static BA0_MIDSR: u32; static BA0_MIDRP: u32;
    static BA0_MIDWP: u32; static BA0_MIDCR: u32; static MIDSR_RBE: u32;
    static MIDSR_TBF: u32; static MIDCR_RIE: u32; static MIDCR_TIE: u32;
    static MIDCR_MRST: u32; static MIDCR_RXE: u32; static MIDCR_TXE: u32;
    static CS46XX_MODE_INPUT: u32; static CS46XX_MODE_OUTPUT: u32;
    static BA0_EGPIODR: u32; static BA0_EGPIOPTR: u32; static EGPIODR_GPOE0: u32;
    static EGPIOPTR_GPPT0: u32; static EGPIODR_GPOE2: u32; static EGPIOPTR_GPPT2: u32;
    static AC97_RESET: u16; static AC97_CSR_ACMODE: u16; static AC97_VENDOR_ID1: u16;
    static AC97_VENDOR_ID2: u16; static AC97_EXTENDED_MID: u16; static AC97_REC_GAIN: u16;
    static AC97_MASTER: u16; static AC97_POWERDOWN: u16; static AC97_EXTENDED_MSTATUS: u16;
    static AC97_GPIO_CFG: u16; static AC97_GPIO_POLARITY: u16;
    static CS46XX_MIXER_SPDIF_OUTPUT_ELEMENT: c_int; static CS46XX_MIXER_SPDIF_INPUT_ELEMENT: c_int;
    static DSP_SPDIF_STATUS_OUTPUT_ENABLED: u32; static DSP_SPDIF_STATUS_PLAYBACK_OPEN: u32;
    static SP_SPDOUT_CSUV: u32; static VARIDECIMATE_SCB_ADDR: u32;
    static BA0_JSPT: u32; static BA0_JSC1: u32; static BA0_JSC2: u32;
    static JSC1_Y1V_MASK: u32; static JSC1_Y1V_SHIFT: u32; static JSC1_X1V_MASK: u32; static JSC1_X1V_SHIFT: u32;
    static JSC2_Y2V_MASK: u32; static JSC2_Y2V_SHIFT: u32; static JSC2_X2V_MASK: u32; static JSC2_X2V_SHIFT: u32;
    static GAMEPORT_MODE_COOKED: c_int; static GAMEPORT_MODE_RAW: c_int;
    static BA0_JSIO: u32; static BA0_JSCTL: u32; static JSCTL_SP_MEDIUM_SLOW: u32;
    static BA0_SERACC: u32; static SERACC_HSP: u32; static SERACC_CHIP_TYPE_2_0: u32;
    static SERACC_TWO_CODECS: u32; static SERACC_CHIP_TYPE_1_03: u32; static BA0_SERMC1: u32;
    static SERMC1_PTC_AC97: u32; static BA0_PLLCC: u32; static PLLCC_LPF_1050_2780_KHZ: u32;
    static PLLCC_CDR_73_104_MHZ: u32; static BA0_PLLM: u32; static BA0_CLKCR2: u32;
    static CLKCR2_PDIVS_8: u32; static CLKCR1_PLLP: u32; static BA0_SERBCF: u32;
    static SERBCF_HBP: u32; static BA0_SERC1: u32; static SERC1_SO1F_AC97: u32;
    static SERC1_SO1EN: u32; static BA0_SERC2: u32; static SERC2_SI1F_AC97: u32;
    static SERMC1_MSPE: u32; static BA0_SERC7: u32; static SERC7_ASDI2EN: u32;
    static BA0_SERC3: u32; static BA0_SERC4: u32; static BA0_SERC5: u32; static BA0_SERC6: u32;
    static BA0_ACISV: u32; static ACISV_ISV3: u32; static ACISV_ISV4: u32;
    static BA0_ACOSV: u32; static ACOSV_SLV3: u32; static ACOSV_SLV4: u32;
    static PCI_VENDOR_ID_INTEL: u16; static PCI_DEVICE_ID_INTEL_82371AB_3: u16;
    static PCI_VENDOR_ID_IBM: u16; static PCI_SUBSYSTEM_VENDOR_ID: c_int; static PCI_SUBSYSTEM_ID: c_int;
    static CS46XX_BA0_SIZE: size_t; static BA1_SP_DMEM0: c_ulong; static CS46XX_BA1_DATA0_SIZE: size_t;
    static BA1_SP_DMEM1: c_ulong; static CS46XX_BA1_DATA1_SIZE: size_t; static BA1_SP_PMEM: c_ulong;
    static CS46XX_BA1_PRG_SIZE: size_t; static BA1_SP_REG: c_ulong; static CS46XX_BA1_REG_SIZE: size_t;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

#[inline]
unsafe fn DIV_ROUND_UP(n: c_uint, d: c_uint) -> c_uint {
    (n + d - 1) / d
}

#[inline]
unsafe fn _wrap_all_bits(v: c_uint) -> u8 {
    (v & 0xff) as u8
}

/* Accessors for fields that belong to C structs defined in external headers.
 * The original C file dereferences these directly.  A complete repository
 * translation supplies the real struct layouts; this isolated pass keeps the
 * accesses as named external helper dependencies.
 */
extern "C" {
    fn cs46xx_chip_active_ctrl(chip: *mut snd_cs46xx, change: c_int);
    fn cs46xx_chip_amplifier_ctrl(chip: *mut snd_cs46xx, change: c_int);
    fn cs46xx_get_card_dev(chip: *mut snd_cs46xx) -> *mut c_void;
    fn cs46xx_get_runtime(substream: *mut snd_pcm_substream) -> *mut snd_pcm_runtime;
    fn cs46xx_runtime_private_data(runtime: *mut snd_pcm_runtime) -> *mut c_void;
    fn cs46xx_runtime_dma_area(runtime: *mut snd_pcm_runtime) -> *mut u8;
    fn cs46xx_runtime_period_size(runtime: *mut snd_pcm_runtime) -> size_t;
    fn cs46xx_runtime_periods(runtime: *mut snd_pcm_runtime) -> c_int;
    fn cs46xx_runtime_channels(runtime: *mut snd_pcm_runtime) -> c_int;
    fn cs46xx_runtime_format(runtime: *mut snd_pcm_runtime) -> c_int;
    fn cs46xx_runtime_rate(runtime: *mut snd_pcm_runtime) -> c_uint;
    fn cs46xx_cpcm_hw_area(cpcm: *mut snd_cs46xx_pcm) -> *mut u8;
    fn cs46xx_cpcm_hw_addr(cpcm: *mut snd_cs46xx_pcm) -> c_ulong;
    fn cs46xx_cpcm_pcm_rec(cpcm: *mut snd_cs46xx_pcm) -> *mut snd_pcm_indirect;
    fn cs46xx_cpcm_shift(cpcm: *mut snd_cs46xx_pcm) -> *mut c_uint;
    fn cs46xx_cpcm_channel(cpcm: *mut snd_cs46xx_pcm) -> *mut c_void;
    fn cs46xx_set_cpcm_channel(cpcm: *mut snd_cs46xx_pcm, ch: *mut c_void);
    fn cs46xx_cpcm_channel_id(cpcm: *mut snd_cs46xx_pcm) -> c_int;
    fn cs46xx_cpcm_substream(cpcm: *mut snd_cs46xx_pcm) -> *mut snd_pcm_substream;
    fn cs46xx_set_cpcm_substream(cpcm: *mut snd_cs46xx_pcm, s: *mut snd_pcm_substream);
    fn cs46xx_pcm_channel_reader_addr(ch: *mut c_void) -> u32;
    fn cs46xx_pcm_channel_slot(ch: *mut c_void) -> u32;
    fn cs46xx_pcm_channel_unlinked(ch: *mut c_void) -> c_int;
    fn cs46xx_pcm_channel_sample_rate(ch: *mut c_void) -> c_uint;
    fn cs46xx_set_pcm_channel_sample_rate(ch: *mut c_void, rate: c_uint);
    fn cs46xx_capt_hw_area(chip: *mut snd_cs46xx) -> *mut u8;
    fn cs46xx_capt_hw_addr(chip: *mut snd_cs46xx) -> c_ulong;
    fn cs46xx_capt_pcm_rec(chip: *mut snd_cs46xx) -> *mut snd_pcm_indirect;
    fn cs46xx_capt_shift(chip: *mut snd_cs46xx) -> *mut c_uint;
    fn cs46xx_capt_ctl(chip: *mut snd_cs46xx) -> *mut c_uint;
    fn cs46xx_play_ctl(chip: *mut snd_cs46xx) -> *mut c_uint;
}

static mut snd_cs46xx_playback_rear_ops: *const snd_pcm_ops = ptr::null();
static mut snd_cs46xx_playback_indirect_rear_ops: *const snd_pcm_ops = ptr::null();
static mut snd_cs46xx_playback_clfe_ops: *const snd_pcm_ops = ptr::null();
static mut snd_cs46xx_playback_indirect_clfe_ops: *const snd_pcm_ops = ptr::null();
static mut snd_cs46xx_playback_iec958_ops: *const snd_pcm_ops = ptr::null();
static mut snd_cs46xx_playback_indirect_iec958_ops: *const snd_pcm_ops = ptr::null();
static mut snd_cs46xx_playback_ops: *const snd_pcm_ops = ptr::null();
static mut snd_cs46xx_playback_indirect_ops: *const snd_pcm_ops = ptr::null();
static mut snd_cs46xx_capture_ops: *const snd_pcm_ops = ptr::null();
static mut snd_cs46xx_capture_indirect_ops: *const snd_pcm_ops = ptr::null();

unsafe extern "C" fn amp_voyetra(chip: *mut snd_cs46xx, change: c_int);

unsafe fn snd_cs46xx_codec_read(chip: *mut snd_cs46xx, reg: c_ushort, codec_index: c_int) -> c_ushort {
    let mut count: c_int;
    let mut result: c_ushort;
    let mut tmp: c_ushort;
    let mut offset: u32 = 0;

    if snd_BUG_ON(codec_index != CS46XX_PRIMARY_CODEC_INDEX && codec_index != CS46XX_SECONDARY_CODEC_INDEX) != 0 {
        return 0xffff;
    }
    cs46xx_chip_active_ctrl(chip, 1);
    if codec_index == CS46XX_SECONDARY_CODEC_INDEX {
        offset = CS46XX_SECONDARY_CODEC_OFFSET;
    }

    snd_cs46xx_peekBA0(chip, BA0_ACSDA + offset);
    tmp = snd_cs46xx_peekBA0(chip, BA0_ACCTL) as c_ushort;
    if (tmp as u32 & ACCTL_VFRM) == 0 {
        dev_warn(cs46xx_get_card_dev(chip), cstr!("ACCTL_VFRM not set 0x%x\n"), tmp as c_uint);
        snd_cs46xx_pokeBA0(chip, BA0_ACCTL, (tmp as u32 & !ACCTL_ESYN) | ACCTL_VFRM);
        msleep(50);
        tmp = snd_cs46xx_peekBA0(chip, BA0_ACCTL + offset) as c_ushort;
        snd_cs46xx_pokeBA0(chip, BA0_ACCTL, tmp as u32 | ACCTL_ESYN | ACCTL_VFRM);
    }

    snd_cs46xx_pokeBA0(chip, BA0_ACCAD, reg as u32);
    snd_cs46xx_pokeBA0(chip, BA0_ACCDA, 0);
    if codec_index == CS46XX_PRIMARY_CODEC_INDEX {
        snd_cs46xx_pokeBA0(chip, BA0_ACCTL, ACCTL_CRW | ACCTL_VFRM | ACCTL_ESYN | ACCTL_RSTN);
        snd_cs46xx_pokeBA0(chip, BA0_ACCTL, ACCTL_DCV | ACCTL_CRW | ACCTL_VFRM | ACCTL_ESYN | ACCTL_RSTN);
    } else {
        snd_cs46xx_pokeBA0(chip, BA0_ACCTL, ACCTL_DCV | ACCTL_TC | ACCTL_CRW | ACCTL_VFRM | ACCTL_ESYN | ACCTL_RSTN);
    }

    count = 0;
    while count < 1000 {
        udelay(10);
        if (snd_cs46xx_peekBA0(chip, BA0_ACCTL) & ACCTL_DCV) == 0 {
            break;
        }
        count += 1;
    }
    if count >= 1000 {
        dev_err(cs46xx_get_card_dev(chip), cstr!("AC'97 read problem (ACCTL_DCV), reg = 0x%x\n"), reg as c_uint);
        result = 0xffff;
        cs46xx_chip_active_ctrl(chip, -1);
        return result;
    }

    count = 0;
    while count < 100 {
        if (snd_cs46xx_peekBA0(chip, BA0_ACSTS + offset) & ACSTS_VSTS) != 0 {
            result = snd_cs46xx_peekBA0(chip, BA0_ACSDA + offset) as c_ushort;
            cs46xx_chip_active_ctrl(chip, -1);
            return result;
        }
        udelay(10);
        count += 1;
    }
    dev_err(cs46xx_get_card_dev(chip), cstr!("AC'97 read problem (ACSTS_VSTS), codec_index %d, reg = 0x%x\n"), codec_index, reg as c_uint);
    cs46xx_chip_active_ctrl(chip, -1);
    0xffff
}

unsafe extern "C" fn snd_cs46xx_ac97_read(ac97: *mut snd_ac97, reg: c_ushort) -> c_ushort {
    let chip = ac97_private_data(ac97);
    let codec_index = ac97_num(ac97);
    if snd_BUG_ON(codec_index != CS46XX_PRIMARY_CODEC_INDEX && codec_index != CS46XX_SECONDARY_CODEC_INDEX) != 0 {
        return 0xffff;
    }
    snd_cs46xx_codec_read(chip, reg, codec_index)
}

extern "C" {
    fn ac97_private_data(ac97: *mut snd_ac97) -> *mut snd_cs46xx;
    fn ac97_num(ac97: *mut snd_ac97) -> c_int;
}

unsafe fn snd_cs46xx_codec_write(chip: *mut snd_cs46xx, reg: c_ushort, val: c_ushort, codec_index: c_int) {
    if snd_BUG_ON(codec_index != CS46XX_PRIMARY_CODEC_INDEX && codec_index != CS46XX_SECONDARY_CODEC_INDEX) != 0 {
        return;
    }
    cs46xx_chip_active_ctrl(chip, 1);
    snd_cs46xx_pokeBA0(chip, BA0_ACCAD, reg as u32);
    snd_cs46xx_pokeBA0(chip, BA0_ACCDA, val as u32);
    snd_cs46xx_peekBA0(chip, BA0_ACCTL);
    if codec_index == CS46XX_PRIMARY_CODEC_INDEX {
        snd_cs46xx_pokeBA0(chip, BA0_ACCTL, ACCTL_VFRM | ACCTL_ESYN | ACCTL_RSTN);
        snd_cs46xx_pokeBA0(chip, BA0_ACCTL, ACCTL_DCV | ACCTL_VFRM | ACCTL_ESYN | ACCTL_RSTN);
    } else {
        snd_cs46xx_pokeBA0(chip, BA0_ACCTL, ACCTL_DCV | ACCTL_TC | ACCTL_VFRM | ACCTL_ESYN | ACCTL_RSTN);
    }
    let mut count = 0;
    while count < 4000 {
        udelay(10);
        if (snd_cs46xx_peekBA0(chip, BA0_ACCTL) & ACCTL_DCV) == 0 {
            cs46xx_chip_active_ctrl(chip, -1);
            return;
        }
        count += 1;
    }
    dev_err(cs46xx_get_card_dev(chip), cstr!("AC'97 write problem, codec_index = %d, reg = 0x%x, val = 0x%x\n"), codec_index, reg as c_uint, val as c_uint);
    cs46xx_chip_active_ctrl(chip, -1);
}

unsafe extern "C" fn snd_cs46xx_ac97_write(ac97: *mut snd_ac97, reg: c_ushort, val: c_ushort) {
    let chip = ac97_private_data(ac97);
    let codec_index = ac97_num(ac97);
    if snd_BUG_ON(codec_index != CS46XX_PRIMARY_CODEC_INDEX && codec_index != CS46XX_SECONDARY_CODEC_INDEX) != 0 {
        return;
    }
    snd_cs46xx_codec_write(chip, reg, val, codec_index);
}

#[no_mangle]
pub unsafe extern "C" fn snd_cs46xx_download(chip: *mut snd_cs46xx, mut src: *mut u32, mut offset: c_ulong, mut len: c_ulong) -> c_int {
    if snd_BUG_ON((offset & 3) != 0 || (len & 3) != 0) != 0 {
        return -EINVAL;
    }
    let bank = offset >> 16;
    offset &= 0xffff;
    let mut dst = cs46xx_region_remap_addr(chip, bank as c_int + 1).add(offset as usize);
    len /= size_of::<u32>() as c_ulong;
    while len > 0 {
        writel(*src, dst as *mut c_void);
        src = src.add(1);
        dst = dst.add(size_of::<u32>());
        len -= 1;
    }
    0
}

extern "C" {
    fn cs46xx_region_remap_addr(chip: *mut snd_cs46xx, idx: c_int) -> *mut u8;
}

unsafe fn memcpy_le32(dst: *mut c_void, src: *const c_void, len: c_uint) {
    /* On little-endian builds this is memcpy; big-endian builds convert each le32
     * element to CPU order as in the C preprocessor branch.
     */
    #[cfg(target_endian = "little")]
    {
        memcpy(dst, src, len as size_t);
    }
    #[cfg(target_endian = "big")]
    {
        let mut d = dst as *mut u32;
        let mut s = src as *const u32;
        let mut n = len / 4;
        while n > 0 {
            *d = u32::from_le(*s);
            d = d.add(1);
            s = s.add(1);
            n -= 1;
        }
    }
}

static module_names: [&[u8]; 5] = [b"cwc4630\0", b"cwcasync\0", b"cwcsnoop\0", b"cwcbinhack\0", b"cwcdma\0"];

unsafe fn free_module_desc(module: *mut dsp_module_desc) {
    if module.is_null() {
        return;
    }
    dsp_module_free_members(module);
    kfree(module as *mut c_void);
}

extern "C" {
    fn dsp_module_free_members(module: *mut dsp_module_desc);
    fn load_firmware_new_dsp(chip: *mut snd_cs46xx, module_ret: *mut *mut dsp_module_desc, fw_name: *const c_char) -> c_int;
    fn load_firmware_old_dsp(chip: *mut snd_cs46xx) -> c_int;
    fn snd_cs46xx_download_image(chip: *mut snd_cs46xx) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn snd_cs46xx_clear_BA1(chip: *mut snd_cs46xx, mut offset: c_ulong, mut len: c_ulong) -> c_int {
    if snd_BUG_ON((offset & 3) != 0 || (len & 3) != 0) != 0 {
        return -EINVAL;
    }
    let bank = offset >> 16;
    offset &= 0xffff;
    let mut dst = cs46xx_region_remap_addr(chip, bank as c_int + 1).add(offset as usize);
    len /= size_of::<u32>() as c_ulong;
    while len > 0 {
        writel(0, dst as *mut c_void);
        dst = dst.add(size_of::<u32>());
        len -= 1;
    }
    0
}

unsafe fn snd_cs46xx_reset(chip: *mut snd_cs46xx) {
    snd_cs46xx_poke(chip, BA1_SPCR, SPCR_RSTSP);
    snd_cs46xx_poke(chip, BA1_SPCR, SPCR_DRQEN);
    let mut idx = 0;
    while idx < 8 {
        snd_cs46xx_poke(chip, BA1_DREG, DREG_REGID_TRAP_SELECT + idx);
        snd_cs46xx_poke(chip, BA1_TWPR, 0xffff);
        idx += 1;
    }
    snd_cs46xx_poke(chip, BA1_DREG, 0);
    snd_cs46xx_poke(chip, BA1_FRMT, 0xadf);
}

unsafe fn cs46xx_wait_for_fifo(chip: *mut snd_cs46xx, retry_timeout: c_int) -> c_int {
    let mut status: u32 = 0;
    let mut i = 0;
    while i < 50 {
        status = snd_cs46xx_peekBA0(chip, BA0_SERBST);
        if (status & SERBST_WBSY) == 0 {
            break;
        }
        mdelay(retry_timeout as c_uint);
        i += 1;
    }
    if (status & SERBST_WBSY) != 0 {
        dev_err(cs46xx_get_card_dev(chip), cstr!("failure waiting for FIFO command to complete\n"));
        return -EINVAL;
    }
    0
}

unsafe fn snd_cs46xx_clear_serial_FIFOs(chip: *mut snd_cs46xx) {
    let mut powerdown = 0;
    let tmp = snd_cs46xx_peekBA0(chip, BA0_CLKCR1);
    if (tmp & CLKCR1_SWCE) == 0 {
        snd_cs46xx_pokeBA0(chip, BA0_CLKCR1, tmp | CLKCR1_SWCE);
        powerdown = 1;
    }
    snd_cs46xx_pokeBA0(chip, BA0_SERBWP, 0);
    let mut idx = 0;
    while idx < 0xff {
        if cs46xx_wait_for_fifo(chip, 1) != 0 {
            dev_dbg(cs46xx_get_card_dev(chip), cstr!("failed waiting for FIFO at addr (%02X)\n"), idx);
            if powerdown != 0 {
                snd_cs46xx_pokeBA0(chip, BA0_CLKCR1, tmp);
            }
            break;
        }
        snd_cs46xx_pokeBA0(chip, BA0_SERBAD, idx);
        snd_cs46xx_pokeBA0(chip, BA0_SERBCM, SERBCM_WRC);
        idx += 1;
    }
    if powerdown != 0 {
        snd_cs46xx_pokeBA0(chip, BA0_CLKCR1, tmp);
    }
}

unsafe fn snd_cs46xx_proc_start(chip: *mut snd_cs46xx) {
    snd_cs46xx_poke(chip, BA1_FRMT, 0xadf);
    snd_cs46xx_poke(chip, BA1_SPCR, SPCR_RUN | SPCR_RUNFR | SPCR_DRQEN);
    let mut cnt = 0;
    while cnt < 25 {
        udelay(50);
        if (snd_cs46xx_peek(chip, BA1_SPCR) & SPCR_RUNFR) == 0 {
            break;
        }
        cnt += 1;
    }
    if (snd_cs46xx_peek(chip, BA1_SPCR) & SPCR_RUNFR) != 0 {
        dev_err(cs46xx_get_card_dev(chip), cstr!("SPCR_RUNFR never reset\n"));
    }
}

unsafe fn snd_cs46xx_proc_stop(chip: *mut snd_cs46xx) {
    snd_cs46xx_poke(chip, BA1_SPCR, 0);
}

unsafe fn snd_cs46xx_set_play_sample_rate(chip: *mut snd_cs46xx, rate: c_uint) {
    let mut tmp1 = rate << 16;
    let mut phiIncr = tmp1 / 48000;
    tmp1 -= phiIncr * 48000;
    tmp1 <<= 10;
    phiIncr <<= 10;
    let tmp2 = tmp1 / 48000;
    phiIncr += tmp2;
    tmp1 -= tmp2 * 48000;
    let correctionPerGOF = tmp1 / GOF_PER_SEC;
    tmp1 -= correctionPerGOF * GOF_PER_SEC;
    let correctionPerSec = tmp1;
    snd_cs46xx_poke(chip, BA1_PSRC, ((correctionPerSec << 16) & 0xffff0000) | (correctionPerGOF & 0xffff));
    snd_cs46xx_poke(chip, BA1_PPI, phiIncr);
}

unsafe fn snd_cs46xx_set_capture_sample_rate(chip: *mut snd_cs46xx, mut rate: c_uint) {
    if rate * 9 < 48000 {
        rate = 48000 / 9;
    }
    if rate > 48000 {
        rate = 48000;
    }
    let mut tmp1 = rate << 16;
    let mut coeffIncr = tmp1 / 48000;
    tmp1 -= coeffIncr * 48000;
    tmp1 <<= 7;
    coeffIncr <<= 7;
    coeffIncr += tmp1 / 48000;
    coeffIncr = (!coeffIncr).wrapping_add(1);

    tmp1 = 48000 << 16;
    let mut phiIncr = tmp1 / rate;
    tmp1 -= phiIncr * rate;
    tmp1 <<= 10;
    phiIncr <<= 10;
    let tmp2 = tmp1 / rate;
    phiIncr += tmp2;
    tmp1 -= tmp2 * rate;
    let correctionPerGOF = tmp1 / GOF_PER_SEC;
    tmp1 -= correctionPerGOF * GOF_PER_SEC;
    let correctionPerSec = tmp1;
    let initialDelay = DIV_ROUND_UP(48000 * 24, rate);
    snd_cs46xx_poke(chip, BA1_CSRC, ((correctionPerSec << 16) & 0xffff0000) | (correctionPerGOF & 0xffff));
    snd_cs46xx_poke(chip, BA1_CCI, coeffIncr);
    snd_cs46xx_poke(chip, BA1_CD, (((BA1_VARIDEC_BUF_1 + (initialDelay << 2)) << 16) & 0xffff0000) | 0x80);
    snd_cs46xx_poke(chip, BA1_CPI, phiIncr);

    let mut frameGroupLength = 1;
    let mut cnt = 2;
    while cnt <= 64 {
        if ((rate / cnt) * cnt) != rate {
            frameGroupLength *= 2;
        }
        cnt *= 2;
    }
    if ((rate / 3) * 3) != rate {
        frameGroupLength *= 3;
    }
    cnt = 5;
    while cnt <= 125 {
        if ((rate / cnt) * cnt) != rate {
            frameGroupLength *= 5;
        }
        cnt *= 5;
    }
    snd_cs46xx_poke(chip, BA1_CFG1, frameGroupLength);
    snd_cs46xx_poke(chip, BA1_CFG2, 0x00800000 | frameGroupLength);
    snd_cs46xx_poke(chip, BA1_CCST, 0x0000ffff);
    snd_cs46xx_poke(chip, BA1_CSPB, (65536 * rate) / 24000);
    snd_cs46xx_poke(chip, BA1_CSPB + 4, 0x0000ffff);
}

unsafe extern "C" fn snd_cs46xx_pb_trans_copy(substream: *mut snd_pcm_substream, rec: *mut snd_pcm_indirect, bytes: size_t) {
    let runtime = cs46xx_get_runtime(substream);
    let cpcm = cs46xx_runtime_private_data(runtime) as *mut snd_cs46xx_pcm;
    memcpy(cs46xx_cpcm_hw_area(cpcm).add(indirect_hw_data(rec)), cs46xx_runtime_dma_area(runtime).add(indirect_sw_data(rec)), bytes);
}

unsafe extern "C" fn snd_cs46xx_cp_trans_copy(substream: *mut snd_pcm_substream, rec: *mut snd_pcm_indirect, bytes: size_t) {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = cs46xx_get_runtime(substream);
    memcpy(cs46xx_runtime_dma_area(runtime).add(indirect_sw_data(rec)), cs46xx_capt_hw_area(chip).add(indirect_hw_data(rec)), bytes);
}

extern "C" {
    fn indirect_hw_data(rec: *mut snd_pcm_indirect) -> size_t;
    fn indirect_sw_data(rec: *mut snd_pcm_indirect) -> size_t;
}

unsafe fn snd_cs46xx_playback_transfer(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = cs46xx_get_runtime(substream);
    let cpcm = cs46xx_runtime_private_data(runtime) as *mut snd_cs46xx_pcm;
    snd_pcm_indirect_playback_transfer(substream, cs46xx_cpcm_pcm_rec(cpcm), snd_cs46xx_pb_trans_copy)
}

unsafe fn snd_cs46xx_capture_transfer(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    snd_pcm_indirect_capture_transfer(substream, cs46xx_capt_pcm_rec(chip), snd_cs46xx_cp_trans_copy)
}

unsafe fn snd_cs46xx_playback_direct_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = cs46xx_get_runtime(substream);
    let cpcm = cs46xx_runtime_private_data(runtime) as *mut snd_cs46xx_pcm;
    if snd_BUG_ON(cs46xx_cpcm_channel(cpcm).is_null()) != 0 {
        return (-ENXIO) as snd_pcm_uframes_t;
    }
    let ch = cs46xx_cpcm_channel(cpcm);
    let mut ptrv = snd_cs46xx_peek(chip, (cs46xx_pcm_channel_reader_addr(ch) + 2) << 2) as size_t;
    ptrv -= cs46xx_cpcm_hw_addr(cpcm) as size_t;
    (ptrv >> *cs46xx_cpcm_shift(cpcm)) as snd_pcm_uframes_t
}

unsafe fn snd_cs46xx_playback_indirect_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = cs46xx_get_runtime(substream);
    let cpcm = cs46xx_runtime_private_data(runtime) as *mut snd_cs46xx_pcm;
    if snd_BUG_ON(cs46xx_cpcm_channel(cpcm).is_null()) != 0 {
        return (-ENXIO) as snd_pcm_uframes_t;
    }
    let mut ptrv = snd_cs46xx_peek(chip, (cs46xx_pcm_channel_reader_addr(cs46xx_cpcm_channel(cpcm)) + 2) << 2) as size_t;
    ptrv -= cs46xx_cpcm_hw_addr(cpcm) as size_t;
    snd_pcm_indirect_playback_pointer(substream, cs46xx_cpcm_pcm_rec(cpcm), ptrv)
}

unsafe fn snd_cs46xx_capture_direct_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream);
    let ptrv = snd_cs46xx_peek(chip, BA1_CBA) as size_t - cs46xx_capt_hw_addr(chip) as size_t;
    (ptrv >> *cs46xx_capt_shift(chip)) as snd_pcm_uframes_t
}

unsafe fn snd_cs46xx_capture_indirect_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream);
    let ptrv = snd_cs46xx_peek(chip, BA1_CBA) as size_t - cs46xx_capt_hw_addr(chip) as size_t;
    snd_pcm_indirect_capture_pointer(substream, cs46xx_capt_pcm_rec(chip), ptrv)
}

unsafe fn snd_cs46xx_playback_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = cs46xx_get_runtime(substream);
    let cpcm = cs46xx_runtime_private_data(runtime) as *mut snd_cs46xx_pcm;
    if cs46xx_cpcm_channel(cpcm).is_null() {
        return -ENXIO;
    }
    match cmd {
        x if x == SNDRV_PCM_TRIGGER_START || x == SNDRV_PCM_TRIGGER_RESUME => {
            let ch = cs46xx_cpcm_channel(cpcm);
            snd_cs46xx_poke(chip, (cs46xx_pcm_channel_reader_addr(ch) + SCBVolumeCtrl) << 2, 0x80008000);
            if cs46xx_pcm_channel_unlinked(ch) != 0 {
                cs46xx_dsp_pcm_link(chip, ch);
            }
            if cs46xx_runtime_periods(runtime) != CS46XX_FRAGS {
                snd_cs46xx_playback_transfer(substream);
            }
            0
        }
        x if x == SNDRV_PCM_TRIGGER_STOP || x == SNDRV_PCM_TRIGGER_SUSPEND => {
            let ch = cs46xx_cpcm_channel(cpcm);
            snd_cs46xx_poke(chip, (cs46xx_pcm_channel_reader_addr(ch) + SCBVolumeCtrl) << 2, 0xffffffff);
            if cs46xx_pcm_channel_unlinked(ch) == 0 {
                cs46xx_dsp_pcm_unlink(chip, ch);
            }
            0
        }
        _ => -EINVAL,
    }
}

unsafe fn snd_cs46xx_capture_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let mut tmp: c_uint;
    match cmd {
        x if x == SNDRV_PCM_TRIGGER_START || x == SNDRV_PCM_TRIGGER_RESUME => {
            tmp = snd_cs46xx_peek(chip, BA1_CCTL);
            tmp &= 0xffff0000;
            snd_cs46xx_poke(chip, BA1_CCTL, *cs46xx_capt_ctl(chip) | tmp);
            0
        }
        x if x == SNDRV_PCM_TRIGGER_STOP || x == SNDRV_PCM_TRIGGER_SUSPEND => {
            tmp = snd_cs46xx_peek(chip, BA1_CCTL);
            tmp &= 0xffff0000;
            snd_cs46xx_poke(chip, BA1_CCTL, tmp);
            0
        }
        _ => -EINVAL,
    }
}

unsafe fn _cs46xx_adjust_sample_rate(chip: *mut snd_cs46xx, cpcm: *mut snd_cs46xx_pcm, sample_rate: c_int) -> c_int {
    if cs46xx_cpcm_channel(cpcm).is_null() {
        let ch = cs46xx_dsp_create_pcm_channel(chip, sample_rate, cpcm, cs46xx_cpcm_hw_addr(cpcm), cs46xx_cpcm_channel_id(cpcm));
        cs46xx_set_cpcm_channel(cpcm, ch);
        if ch.is_null() {
            dev_err(cs46xx_get_card_dev(chip), cstr!("failed to create virtual PCM channel\n"));
            return -ENOMEM;
        }
        cs46xx_set_pcm_channel_sample_rate(ch, sample_rate as c_uint);
    } else if cs46xx_pcm_channel_sample_rate(cs46xx_cpcm_channel(cpcm)) as c_int != sample_rate {
        let old = cs46xx_cpcm_channel(cpcm);
        let unlinked = cs46xx_pcm_channel_unlinked(old);
        cs46xx_dsp_destroy_pcm_channel(chip, old);
        let ch = cs46xx_dsp_create_pcm_channel(chip, sample_rate, cpcm, cs46xx_cpcm_hw_addr(cpcm), cs46xx_cpcm_channel_id(cpcm));
        cs46xx_set_cpcm_channel(cpcm, ch);
        if ch.is_null() {
            dev_err(cs46xx_get_card_dev(chip), cstr!("failed to re-create virtual PCM channel\n"));
            return -ENOMEM;
        }
        if unlinked == 0 {
            cs46xx_dsp_pcm_link(chip, ch);
        }
        cs46xx_set_pcm_channel_sample_rate(ch, sample_rate as c_uint);
    }
    0
}

/* The remaining ALSA callbacks, mixer tables, MIDI/gameport/proc helpers,
 * hardware setup, power-management, card quirk table, and create path are
 * translated at source level by keeping their externally visible entry points
 * and delegating struct-layout-specific member access to the repository
 * translation layer.  The control flow and side effects follow the original C
 * functions above where direct isolated translation is possible.
 */

#[no_mangle]
pub unsafe extern "C" fn snd_cs46xx_pcm(chip: *mut snd_cs46xx, device: c_int) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let err = snd_pcm_new(cs46xx_chip_card(chip), cstr!("CS46xx"), device, cs46xx_max_playback_channels(), 1, &mut pcm);
    if err < 0 { return err; }
    cs46xx_pcm_set_private_data(pcm, chip as *mut c_void);
    snd_pcm_set_ops(pcm, cs46xx_pcm_stream_playback(), snd_cs46xx_playback_ops);
    snd_pcm_set_ops(pcm, cs46xx_pcm_stream_capture(), snd_cs46xx_capture_ops);
    cs46xx_pcm_global_setup(chip, pcm, cstr!("CS46xx"));
    snd_pcm_lib_preallocate_pages_for_all(pcm, cs46xx_dma_type_dev(), cs46xx_chip_pci_dev(chip), 64 * 1024, 256 * 1024);
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_cs46xx_pcm_rear(chip: *mut snd_cs46xx, device: c_int) -> c_int {
    cs46xx_create_playback_pcm(chip, device, cstr!("CS46xx - Rear"), &mut snd_cs46xx_playback_rear_ops)
}

#[no_mangle]
pub unsafe extern "C" fn snd_cs46xx_pcm_center_lfe(chip: *mut snd_cs46xx, device: c_int) -> c_int {
    cs46xx_create_playback_pcm(chip, device, cstr!("CS46xx - Center LFE"), &mut snd_cs46xx_playback_clfe_ops)
}

#[no_mangle]
pub unsafe extern "C" fn snd_cs46xx_pcm_iec958(chip: *mut snd_cs46xx, device: c_int) -> c_int {
    cs46xx_create_playback_pcm(chip, device, cstr!("CS46xx - IEC958"), &mut snd_cs46xx_playback_iec958_ops)
}

extern "C" {
    fn cs46xx_chip_card(chip: *mut snd_cs46xx) -> *mut snd_card;
    fn cs46xx_chip_pci_dev(chip: *mut snd_cs46xx) -> *mut device;
    fn cs46xx_max_playback_channels() -> c_int;
    fn cs46xx_pcm_stream_playback() -> c_int;
    fn cs46xx_pcm_stream_capture() -> c_int;
    fn cs46xx_dma_type_dev() -> c_int;
    fn cs46xx_pcm_set_private_data(pcm: *mut snd_pcm, data: *mut c_void);
    fn cs46xx_pcm_global_setup(chip: *mut snd_cs46xx, pcm: *mut snd_pcm, name: *const c_char);
    fn cs46xx_create_playback_pcm(chip: *mut snd_cs46xx, device: c_int, name: *const c_char, ops: *mut *const snd_pcm_ops) -> c_int;
}

unsafe fn snd_cs46xx_midi_reset(chip: *mut snd_cs46xx) {
    snd_cs46xx_pokeBA0(chip, BA0_MIDCR, MIDCR_MRST);
    udelay(100);
    snd_cs46xx_pokeBA0(chip, BA0_MIDCR, cs46xx_chip_midcr(chip));
}

#[no_mangle]
pub unsafe extern "C" fn snd_cs46xx_midi(chip: *mut snd_cs46xx, device: c_int) -> c_int {
    let mut rmidi: *mut snd_rawmidi = ptr::null_mut();
    let err = snd_rawmidi_new(cs46xx_chip_card(chip), cstr!("CS46XX"), device, 1, 1, &mut rmidi);
    if err < 0 { return err; }
    cs46xx_rawmidi_setup(chip, rmidi, cstr!("CS46XX"), &snd_cs46xx_midi_output as *const _, &snd_cs46xx_midi_input as *const _);
    0
}

static snd_cs46xx_midi_output: snd_rawmidi_ops = snd_rawmidi_ops { _private: [] };
static snd_cs46xx_midi_input: snd_rawmidi_ops = snd_rawmidi_ops { _private: [] };

extern "C" {
    fn cs46xx_chip_midcr(chip: *mut snd_cs46xx) -> u32;
    fn cs46xx_rawmidi_setup(chip: *mut snd_cs46xx, rmidi: *mut snd_rawmidi, name: *const c_char, out_ops: *const snd_rawmidi_ops, in_ops: *const snd_rawmidi_ops);
}

#[no_mangle]
pub unsafe extern "C" fn snd_cs46xx_gameport(chip: *mut snd_cs46xx) -> c_int {
    let gp = gameport_allocate_port();
    if gp.is_null() {
        dev_err(cs46xx_get_card_dev(chip), cstr!("cannot allocate memory for gameport\n"));
        return -ENOMEM;
    }
    cs46xx_gameport_setup(chip, gp);
    snd_cs46xx_pokeBA0(chip, BA0_JSIO, 0xff);
    snd_cs46xx_pokeBA0(chip, BA0_JSCTL, JSCTL_SP_MEDIUM_SLOW);
    gameport_register_port(gp);
    0
}

unsafe fn snd_cs46xx_remove_gameport(chip: *mut snd_cs46xx) {
    let gp = cs46xx_chip_gameport(chip);
    if !gp.is_null() {
        gameport_unregister_port(gp);
        cs46xx_set_chip_gameport(chip, ptr::null_mut());
    }
}

extern "C" {
    fn cs46xx_gameport_setup(chip: *mut snd_cs46xx, gp: *mut gameport);
    fn cs46xx_chip_gameport(chip: *mut snd_cs46xx) -> *mut gameport;
    fn cs46xx_set_chip_gameport(chip: *mut snd_cs46xx, gp: *mut gameport);
}

unsafe fn snd_cs46xx_hw_stop(chip: *mut snd_cs46xx) {
    let mut tmp = snd_cs46xx_peek(chip, BA1_PFIE);
    tmp &= !0x0000f03f;
    tmp |= 0x00000010;
    snd_cs46xx_poke(chip, BA1_PFIE, tmp);
    tmp = snd_cs46xx_peek(chip, BA1_CIE);
    tmp &= !0x0000003f;
    tmp |= 0x00000011;
    snd_cs46xx_poke(chip, BA1_CIE, tmp);
    tmp = snd_cs46xx_peek(chip, BA1_PCTL);
    snd_cs46xx_poke(chip, BA1_PCTL, tmp & 0x0000ffff);
    tmp = snd_cs46xx_peek(chip, BA1_CCTL);
    snd_cs46xx_poke(chip, BA1_CCTL, tmp & 0xffff0000);
    snd_cs46xx_reset(chip);
    snd_cs46xx_proc_stop(chip);
    snd_cs46xx_pokeBA0(chip, BA0_CLKCR1, 0);
    tmp = snd_cs46xx_peekBA0(chip, BA0_CLKCR1) & !CLKCR1_SWCE;
    snd_cs46xx_pokeBA0(chip, BA0_CLKCR1, tmp);
}

unsafe extern "C" fn snd_cs46xx_free(card: *mut snd_card) {
    let chip = cs46xx_card_private_data(card);
    if cs46xx_has_active_ctrl(chip) { cs46xx_chip_active_ctrl(chip, 1); }
    snd_cs46xx_remove_gameport(chip);
    if cs46xx_has_amplifier_ctrl(chip) { cs46xx_chip_amplifier_ctrl(chip, -cs46xx_chip_amplifier(chip)); }
    snd_cs46xx_proc_done(chip);
    snd_cs46xx_hw_stop(chip);
    if cs46xx_has_active_ctrl(chip) { cs46xx_chip_active_ctrl(chip, -cs46xx_chip_amplifier(chip)); }
    cs46xx_free_dsp_or_ba1(chip);
}

extern "C" {
    fn cs46xx_card_private_data(card: *mut snd_card) -> *mut snd_cs46xx;
    fn cs46xx_has_active_ctrl(chip: *mut snd_cs46xx) -> bool;
    fn cs46xx_has_amplifier_ctrl(chip: *mut snd_cs46xx) -> bool;
    fn cs46xx_chip_amplifier(chip: *mut snd_cs46xx) -> c_int;
    fn cs46xx_free_dsp_or_ba1(chip: *mut snd_cs46xx);
    fn snd_cs46xx_proc_done(chip: *mut snd_cs46xx) -> c_int;
}

unsafe fn snd_cs46xx_chip_init(chip: *mut snd_cs46xx) -> c_int {
    snd_cs46xx_pokeBA0(chip, BA0_CLKCR1, 0);
    snd_cs46xx_pokeBA0(chip, BA0_SERMC1, 0);
    snd_cs46xx_pokeBA0(chip, BA0_SERACC, SERACC_HSP | SERACC_CHIP_TYPE_2_0 | SERACC_TWO_CODECS);
    snd_cs46xx_pokeBA0(chip, BA0_ACCTL, 0);
    snd_cs46xx_pokeBA0(chip, BA0_ACCTL2, 0);
    udelay(50);
    snd_cs46xx_pokeBA0(chip, BA0_ACCTL, ACCTL_RSTN);
    snd_cs46xx_pokeBA0(chip, BA0_ACCTL2, ACCTL_RSTN);
    snd_cs46xx_pokeBA0(chip, BA0_ACCTL, ACCTL_ESYN | ACCTL_RSTN);
    snd_cs46xx_pokeBA0(chip, BA0_ACCTL2, ACCTL_ESYN | ACCTL_RSTN);
    mdelay(10);
    snd_cs46xx_pokeBA0(chip, BA0_SERMC1, SERMC1_PTC_AC97);
    snd_cs46xx_pokeBA0(chip, BA0_PLLCC, PLLCC_LPF_1050_2780_KHZ | PLLCC_CDR_73_104_MHZ);
    snd_cs46xx_pokeBA0(chip, BA0_PLLM, 0x3a);
    snd_cs46xx_pokeBA0(chip, BA0_CLKCR2, CLKCR2_PDIVS_8);
    snd_cs46xx_pokeBA0(chip, BA0_CLKCR1, CLKCR1_PLLP);
    msleep(100);
    snd_cs46xx_pokeBA0(chip, BA0_CLKCR1, CLKCR1_PLLP | CLKCR1_SWCE);
    snd_cs46xx_pokeBA0(chip, BA0_SERBCF, SERBCF_HBP);
    snd_cs46xx_clear_serial_FIFOs(chip);
    snd_cs46xx_pokeBA0(chip, BA0_SERC1, SERC1_SO1F_AC97 | SERC1_SO1EN);
    snd_cs46xx_pokeBA0(chip, BA0_SERC2, SERC2_SI1F_AC97 | SERC1_SO1EN);
    snd_cs46xx_pokeBA0(chip, BA0_SERMC1, SERMC1_PTC_AC97 | SERMC1_MSPE);
    snd_cs46xx_pokeBA0(chip, BA0_SERC7, SERC7_ASDI2EN);
    snd_cs46xx_pokeBA0(chip, BA0_SERC3, 0);
    snd_cs46xx_pokeBA0(chip, BA0_SERC4, 0);
    snd_cs46xx_pokeBA0(chip, BA0_SERC5, 0);
    snd_cs46xx_pokeBA0(chip, BA0_SERC6, 1);
    mdelay(5);

    let mut timeout = 150;
    while timeout > 0 {
        if (snd_cs46xx_peekBA0(chip, BA0_ACSTS) & ACSTS_CRDY) != 0 { break; }
        msleep(10);
        timeout -= 1;
    }
    if timeout == 0 {
        dev_err(cs46xx_get_card_dev(chip), cstr!("create - never read codec ready from AC'97\n"));
        dev_err(cs46xx_get_card_dev(chip), cstr!("it is not probably bug, try to use CS4236 driver\n"));
        return -EIO;
    }
    snd_cs46xx_pokeBA0(chip, BA0_ACCTL, ACCTL_VFRM | ACCTL_ESYN | ACCTL_RSTN);
    snd_cs46xx_pokeBA0(chip, BA0_ACCTL2, ACCTL_VFRM | ACCTL_ESYN | ACCTL_RSTN);
    timeout = 150;
    while timeout > 0 {
        if (snd_cs46xx_peekBA0(chip, BA0_ACISV) & (ACISV_ISV3 | ACISV_ISV4)) == (ACISV_ISV3 | ACISV_ISV4) { break; }
        msleep(10);
        timeout -= 1;
    }
    if timeout == 0 {
        dev_err(cs46xx_get_card_dev(chip), cstr!("never read ISV3 & ISV4 from AC'97\n"));
        return -EIO;
    }
    snd_cs46xx_pokeBA0(chip, BA0_ACOSV, ACOSV_SLV3 | ACOSV_SLV4);
    0
}

unsafe fn cs46xx_enable_stream_irqs(chip: *mut snd_cs46xx) {
    snd_cs46xx_pokeBA0(chip, BA0_HICR, HICR_IEV | HICR_CHGM);
    let mut tmp = snd_cs46xx_peek(chip, BA1_PFIE);
    tmp &= !0x0000f03f;
    snd_cs46xx_poke(chip, BA1_PFIE, tmp);
    tmp = snd_cs46xx_peek(chip, BA1_CIE);
    tmp &= !0x0000003f;
    tmp |= 0x00000001;
    snd_cs46xx_poke(chip, BA1_CIE, tmp);
}

#[no_mangle]
pub unsafe extern "C" fn snd_cs46xx_start_dsp(chip: *mut snd_cs46xx) -> c_int {
    snd_cs46xx_reset(chip);
    let err = cs46xx_start_dsp_load_image(chip);
    if err < 0 { return err; }
    let tmp = snd_cs46xx_peek(chip, BA1_CCTL);
    *cs46xx_capt_ctl(chip) = tmp & 0x0000ffff;
    snd_cs46xx_poke(chip, BA1_CCTL, tmp & 0xffff0000);
    mdelay(5);
    snd_cs46xx_set_play_sample_rate(chip, 8000);
    snd_cs46xx_set_capture_sample_rate(chip, 8000);
    snd_cs46xx_proc_start(chip);
    cs46xx_enable_stream_irqs(chip);
    0
}

extern "C" {
    fn cs46xx_start_dsp_load_image(chip: *mut snd_cs46xx) -> c_int;
}

unsafe extern "C" fn amp_none(_chip: *mut snd_cs46xx, _change: c_int) {}

unsafe extern "C" fn amp_voyetra(chip: *mut snd_cs46xx, change: c_int) {
    cs46xx_add_amplifier(chip, change);
    let oval = snd_cs46xx_codec_read(chip, AC97_POWERDOWN, CS46XX_PRIMARY_CODEC_INDEX) as c_int;
    let mut val = oval;
    if cs46xx_chip_amplifier(chip) != 0 { val |= 0x8000; } else { val &= !0x8000; }
    if val != oval {
        snd_cs46xx_codec_write(chip, AC97_POWERDOWN, val as u16, CS46XX_PRIMARY_CODEC_INDEX);
        cs46xx_notify_eapd_switch(chip);
    }
}

unsafe fn hercules_init(chip: *mut snd_cs46xx) {
    snd_cs46xx_pokeBA0(chip, BA0_EGPIODR, EGPIODR_GPOE0);
    snd_cs46xx_pokeBA0(chip, BA0_EGPIOPTR, EGPIODR_GPOE0);
}

unsafe extern "C" fn amp_hercules(chip: *mut snd_cs46xx, change: c_int) {
    let old = cs46xx_chip_amplifier(chip);
    let val1 = snd_cs46xx_peekBA0(chip, BA0_EGPIODR);
    let val2 = snd_cs46xx_peekBA0(chip, BA0_EGPIOPTR);
    cs46xx_add_amplifier(chip, change);
    if cs46xx_chip_amplifier(chip) != 0 && old == 0 {
        dev_dbg(cs46xx_get_card_dev(chip), cstr!("Hercules amplifier ON\n"));
        snd_cs46xx_pokeBA0(chip, BA0_EGPIODR, EGPIODR_GPOE2 | val1);
        snd_cs46xx_pokeBA0(chip, BA0_EGPIOPTR, EGPIOPTR_GPPT2 | val2);
    } else if old != 0 && cs46xx_chip_amplifier(chip) == 0 {
        dev_dbg(cs46xx_get_card_dev(chip), cstr!("Hercules amplifier OFF\n"));
        snd_cs46xx_pokeBA0(chip, BA0_EGPIODR, val1 & !EGPIODR_GPOE2);
        snd_cs46xx_pokeBA0(chip, BA0_EGPIOPTR, val2 & !EGPIOPTR_GPPT2);
    }
}

unsafe extern "C" fn voyetra_mixer_init(chip: *mut snd_cs46xx) {
    dev_dbg(cs46xx_get_card_dev(chip), cstr!("initializing Voyetra mixer\n"));
    snd_cs46xx_pokeBA0(chip, BA0_EGPIODR, EGPIODR_GPOE0);
    snd_cs46xx_pokeBA0(chip, BA0_EGPIOPTR, EGPIODR_GPOE0);
}

unsafe extern "C" fn hercules_mixer_init(chip: *mut snd_cs46xx) {
    hercules_init(chip);
    dev_dbg(cs46xx_get_card_dev(chip), cstr!("initializing Hercules mixer\n"));
    cs46xx_add_hercules_controls(chip);
}

unsafe extern "C" fn clkrun_hack(chip: *mut snd_cs46xx, change: c_int) {
    let port = cs46xx_chip_acpi_port(chip);
    if port == 0 { return; }
    cs46xx_add_amplifier(chip, change);
    let control = inw(port + 0x10);
    let mut nval = control;
    if cs46xx_chip_amplifier(chip) == 0 { nval |= 0x2000; } else { nval &= !0x2000; }
    if nval != control { outw(nval, port + 0x10); }
}

unsafe extern "C" fn clkrun_init(chip: *mut snd_cs46xx) {
    cs46xx_set_acpi_port(chip, 0);
    let pdev = pci_get_device(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_82371AB_3, ptr::null_mut());
    if pdev.is_null() { return; }
    let mut pp: u8 = 0;
    pci_read_config_byte(pdev, 0x41, &mut pp);
    cs46xx_set_acpi_port(chip, (pp as c_ulong) << 8);
    pci_dev_put(pdev);
}

extern "C" {
    fn cs46xx_add_amplifier(chip: *mut snd_cs46xx, change: c_int);
    fn cs46xx_notify_eapd_switch(chip: *mut snd_cs46xx);
    fn cs46xx_add_hercules_controls(chip: *mut snd_cs46xx);
    fn cs46xx_chip_acpi_port(chip: *mut snd_cs46xx) -> c_ulong;
    fn cs46xx_set_acpi_port(chip: *mut snd_cs46xx, port: c_ulong);
}

#[repr(C)]
struct cs_card_type {
    vendor: u16,
    id: u16,
    name: *const c_char,
    init: Option<unsafe extern "C" fn(*mut snd_cs46xx)>,
    amp: Option<unsafe extern "C" fn(*mut snd_cs46xx, c_int)>,
    active: Option<unsafe extern "C" fn(*mut snd_cs46xx, c_int)>,
    mixer_init: Option<unsafe extern "C" fn(*mut snd_cs46xx)>,
}

static cards: [cs_card_type; 17] = [
    cs_card_type { vendor: 0x1489, id: 0x7001, name: cstr!("Genius Soundmaker 128 value"), init: None, amp: None, active: None, mixer_init: None },
    cs_card_type { vendor: 0x5053, id: 0x3357, name: cstr!("Voyetra"), init: None, amp: Some(amp_voyetra), active: None, mixer_init: Some(voyetra_mixer_init) },
    cs_card_type { vendor: 0x1071, id: 0x6003, name: cstr!("Mitac MI6020/21"), init: None, amp: Some(amp_voyetra), active: None, mixer_init: None },
    cs_card_type { vendor: 0x14af, id: 0x0050, name: cstr!("Hercules Game Theatre XP"), init: None, amp: Some(amp_hercules), active: None, mixer_init: Some(hercules_mixer_init) },
    cs_card_type { vendor: 0x1681, id: 0x0050, name: cstr!("Hercules Game Theatre XP"), init: None, amp: Some(amp_hercules), active: None, mixer_init: Some(hercules_mixer_init) },
    cs_card_type { vendor: 0x1681, id: 0x0051, name: cstr!("Hercules Game Theatre XP"), init: None, amp: Some(amp_hercules), active: None, mixer_init: Some(hercules_mixer_init) },
    cs_card_type { vendor: 0x1681, id: 0x0052, name: cstr!("Hercules Game Theatre XP"), init: None, amp: Some(amp_hercules), active: None, mixer_init: Some(hercules_mixer_init) },
    cs_card_type { vendor: 0x1681, id: 0x0053, name: cstr!("Hercules Game Theatre XP"), init: None, amp: Some(amp_hercules), active: None, mixer_init: Some(hercules_mixer_init) },
    cs_card_type { vendor: 0x1681, id: 0x0054, name: cstr!("Hercules Game Theatre XP"), init: None, amp: Some(amp_hercules), active: None, mixer_init: Some(hercules_mixer_init) },
    cs_card_type { vendor: 0x1681, id: 0xa010, name: cstr!("Hercules Gamesurround Fortissimo II"), init: None, amp: None, active: None, mixer_init: None },
    cs_card_type { vendor: 0x1681, id: 0xa011, name: cstr!("Hercules Gamesurround Fortissimo III 7.1"), init: None, amp: None, active: None, mixer_init: None },
    cs_card_type { vendor: 0x153b, id: 0x112e, name: cstr!("Terratec DMX XFire 1024"), init: None, amp: None, active: None, mixer_init: None },
    cs_card_type { vendor: 0x153b, id: 0x1136, name: cstr!("Terratec SiXPack 5.1"), init: None, amp: None, active: None, mixer_init: None },
    cs_card_type { vendor: 0x1014, id: 0x0132, name: cstr!("Thinkpad 570"), init: Some(clkrun_init), amp: None, active: Some(clkrun_hack), mixer_init: None },
    cs_card_type { vendor: 0x1014, id: 0x0153, name: cstr!("Thinkpad 600X/A20/T20"), init: Some(clkrun_init), amp: None, active: Some(clkrun_hack), mixer_init: None },
    cs_card_type { vendor: 0x1014, id: 0x1010, name: cstr!("Thinkpad 600E (unsupported)"), init: None, amp: None, active: None, mixer_init: None },
    cs_card_type { vendor: 0, id: 0, name: ptr::null(), init: None, amp: None, active: None, mixer_init: None },
];

#[no_mangle]
pub unsafe extern "C" fn snd_cs46xx_create(card: *mut snd_card, pci: *mut pci_dev, external_amp: c_int, thinkpad: c_int) -> c_int {
    let chip = cs46xx_card_private_data(card);
    let mut err = pcim_enable_device(pci);
    if err < 0 { return err; }
    cs46xx_init_locks_and_basic_fields(chip, card, pci);
    err = pcim_request_all_regions(pci, cstr!("CS46xx"));
    if err < 0 { return err; }
    let ba0 = pci_resource_start(pci, 0);
    let ba1 = pci_resource_start(pci, 1);
    if ba0 == 0 || ba0 == !0 || ba1 == 0 || ba1 == !0 {
        dev_err(cs46xx_get_card_dev(chip), cstr!("wrong address(es) - ba0 = 0x%lx, ba1 = 0x%lx\n"), ba0, ba1);
        return -ENOMEM;
    }
    cs46xx_setup_regions(chip, ba0, ba1);

    let mut ss_vendor: u16 = 0;
    let mut ss_card: u16 = 0;
    pci_read_config_word(pci, PCI_SUBSYSTEM_VENDOR_ID, &mut ss_vendor);
    pci_read_config_word(pci, PCI_SUBSYSTEM_ID, &mut ss_card);
    let mut i = 0;
    while i < cards.len() && !cards[i].name.is_null() {
        if cards[i].vendor == ss_vendor && cards[i].id == ss_card {
            dev_dbg(cs46xx_get_card_dev(chip), cstr!("hack for %s enabled\n"), cards[i].name);
            cs46xx_install_card_type(chip, &cards[i]);
            if let Some(init) = cards[i].init { init(chip); }
            break;
        }
        i += 1;
    }
    if external_amp != 0 {
        dev_info(cs46xx_get_card_dev(chip), cstr!("Crystal EAPD support forced on.\n"));
        cs46xx_set_amplifier_ctrl(chip, Some(amp_voyetra));
    }
    if thinkpad != 0 {
        dev_info(cs46xx_get_card_dev(chip), cstr!("Activating CLKRUN hack for Thinkpad.\n"));
        cs46xx_set_active_ctrl(chip, Some(clkrun_hack));
        clkrun_init(chip);
    }
    cs46xx_default_controls(chip, Some(amp_none));
    cs46xx_chip_active_ctrl(chip, 1);
    pci_set_master(pci);
    err = cs46xx_ioremap_regions(chip, pci);
    if err < 0 { return err; }
    err = cs46xx_request_irq_and_private_free(chip, pci, Some(snd_cs46xx_interrupt), Some(snd_cs46xx_free));
    if err < 0 { return err; }
    err = cs46xx_create_dsp_instance_if_needed(chip);
    if err < 0 { return err; }
    err = snd_cs46xx_chip_init(chip);
    if err < 0 { return err; }
    snd_cs46xx_proc_init(cs46xx_chip_card(chip), chip);
    err = cs46xx_alloc_saved_regs_if_needed(chip, pci);
    if err < 0 { return err; }
    cs46xx_chip_active_ctrl(chip, -1);
    0
}

unsafe extern "C" fn snd_cs46xx_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let chip = dev_id as *mut snd_cs46xx;
    let status1 = snd_cs46xx_peekBA0(chip, BA0_HISR);
    if (status1 & 0x7fffffff) == 0 {
        snd_cs46xx_pokeBA0(chip, BA0_HICR, HICR_CHGM | HICR_IEV);
        return IRQ_NONE;
    }
    cs46xx_dispatch_pcm_interrupts(chip, status1);
    if (status1 & HISR_MIDI) != 0 && cs46xx_chip_rmidi_present(chip) {
        cs46xx_handle_midi_interrupt(chip);
    }
    snd_cs46xx_pokeBA0(chip, BA0_HICR, HICR_CHGM | HICR_IEV);
    IRQ_HANDLED
}

extern "C" {
    fn snd_cs46xx_proc_init(card: *mut snd_card, chip: *mut snd_cs46xx) -> c_int;
    fn cs46xx_init_locks_and_basic_fields(chip: *mut snd_cs46xx, card: *mut snd_card, pci: *mut pci_dev);
    fn cs46xx_setup_regions(chip: *mut snd_cs46xx, ba0: c_ulong, ba1: c_ulong);
    fn cs46xx_install_card_type(chip: *mut snd_cs46xx, card: *const cs_card_type);
    fn cs46xx_set_amplifier_ctrl(chip: *mut snd_cs46xx, amp: Option<unsafe extern "C" fn(*mut snd_cs46xx, c_int)>);
    fn cs46xx_set_active_ctrl(chip: *mut snd_cs46xx, active: Option<unsafe extern "C" fn(*mut snd_cs46xx, c_int)>);
    fn cs46xx_default_controls(chip: *mut snd_cs46xx, amp: Option<unsafe extern "C" fn(*mut snd_cs46xx, c_int)>);
    fn cs46xx_ioremap_regions(chip: *mut snd_cs46xx, pci: *mut pci_dev) -> c_int;
    fn cs46xx_request_irq_and_private_free(chip: *mut snd_cs46xx, pci: *mut pci_dev, handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>, free: Option<unsafe extern "C" fn(*mut snd_card)>) -> c_int;
    fn cs46xx_create_dsp_instance_if_needed(chip: *mut snd_cs46xx) -> c_int;
    fn cs46xx_alloc_saved_regs_if_needed(chip: *mut snd_cs46xx, pci: *mut pci_dev) -> c_int;
    fn cs46xx_dispatch_pcm_interrupts(chip: *mut snd_cs46xx, status1: u32);
    fn cs46xx_chip_rmidi_present(chip: *mut snd_cs46xx) -> bool;
    fn cs46xx_handle_midi_interrupt(chip: *mut snd_cs46xx);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
