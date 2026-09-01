// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
 *
 * @File    ctatc.c
 *
 * @Brief
 * This file contains the implementation of the device resource management
 * object.
 *
 * @Author Liu Chun
 * @Date Mar 28 2008
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

type u16 = u16;
type u32 = u32;
type snd_pcm_format_t = c_int;

const NUM_ATC_SRCS: usize = 6;
const NUM_ATC_PCM: usize = 2 * 4;

const MONO_SUM_SCALE: c_uint = 0x19a8; /* 2^(-0.5) in 14-bit floating format */
const MAX_MULTI_CHN: usize = 8;

extern "C" {
    static NUM_CTCARDS: usize;
    static NUM_CTALSADEVS: usize;
    static NUM_RSCTYP: usize;
    static NUM_DAIOTYP: usize;

    static PCI_VENDOR_ID_CREATIVE: c_uint;
    static PCI_SUBDEVICE_ID_CREATIVE_SB0760: c_uint;
    static PCI_SUBDEVICE_ID_CREATIVE_SB08801: c_uint;
    static PCI_SUBDEVICE_ID_CREATIVE_SB08802: c_uint;
    static PCI_SUBDEVICE_ID_CREATIVE_SB08803: c_uint;
    static PCI_SUBDEVICE_ID_CREATIVE_SB1270: c_uint;
    static PCI_SUBDEVICE_ID_CREATIVE_HENDRIX: c_uint;

    static IEC958_AES0_NONAUDIO: c_uint;
    static IEC958_AES0_CON_NOT_COPYRIGHT: c_uint;
    static IEC958_AES1_CON_MIXER: c_uint;
    static IEC958_AES1_CON_ORIGINAL: c_uint;
    static IEC958_AES3_CON_FS_48000: c_uint;
    static IEC958_AES3_CON_FS_44100: c_uint;
    static IEC958_AES3_CON_FS_32000: c_uint;
    static IEC958_AES3_CON_FS: c_uint;

    static SNDRV_PCM_FORMAT_U8: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_S16_LE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_S24_3LE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_S32_LE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_FLOAT_LE: snd_pcm_format_t;

    static SRC_SF_U8: c_uint;
    static SRC_SF_S16: c_uint;
    static SRC_SF_S24: c_uint;
    static SRC_SF_S32: c_uint;
    static SRC_SF_F32: c_uint;

    static SRC_STATE_INIT: c_uint;
    static SRC_STATE_OFF: c_uint;
    static SRC_STATE_RUN: c_uint;
    static INIT_VOL: c_uint;
    static MEMRD: c_uint;
    static MEMWR: c_uint;
    static ARCRW: c_uint;
    static GFP_KERNEL: c_uint;
    static ENOENT: c_int;
    static ENOMEM: c_int;

    static ADC_LINEIN: c_uint;
    static ADC_MICIN: c_uint;
    static ADC_NONE: c_uint;
    static MIX_MIC_IN: c_int;
    static MIX_LINE_IN: c_int;
    static MIX_PCMO_FRONT: c_int;
    static MIX_WAVE_FRONT: c_int;
    static MIX_SPDIF_OUT: c_int;
    static MIX_SPDIF_IN: c_int;
    static MIX_PCMI_FRONT: c_int;
    static MIX_PCMI_SURROUND: c_int;

    static FRONT: usize;
    static SURROUND: usize;
    static CLFE: usize;
    static SIDE: usize;
    static IEC958: usize;
    static MIXER: usize;
    static SRC: usize;
    static SRCIMP: usize;
    static AMIXER: usize;
    static SUM: usize;
    static DAIO: usize;
    static SPDIFOO: usize;
    static SPDIFIO: usize;
    static SPDIFI_BAY: usize;
    static RCA: usize;
    static LINEO1: usize;
    static LINEO2: usize;
    static LINEO3: usize;
    static LINEO4: usize;
    static LINEIM: usize;
    static MIC: usize;
    static CTSB046X: usize;
    static CTSB055X: usize;
    static CTSB073X: usize;
    static CTUAA: usize;
    static CT20K1_UNKNOWN: usize;
    static CTSB0760: usize;
    static CTHENDRIX: usize;
    static CTSB0880: usize;
    static CTSB1270: usize;
    static CTOK0010: usize;
    static CT20K2_UNKNOWN: usize;
    static ATC20K1: c_int;
    static ATC20K2: c_int;
    static SNDRV_CTL_POWER_D3hot: c_int;
    static SNDRV_CTL_POWER_D0: c_int;
    static SNDRV_DEV_LOWLEVEL: c_int;
}

#[repr(C)] struct snd_card { dev: *mut c_void }
#[repr(C)] struct pci_dev { subsystem_vendor: u16, subsystem_device: u16 }
#[repr(C)] struct snd_device { device_data: *mut c_void }
#[repr(C)] struct snd_device_ops { dev_free: Option<unsafe extern "C" fn(*mut snd_device) -> c_int> }
#[repr(C)] struct snd_pcm_runtime { channels: c_int, rate: c_uint, format: snd_pcm_format_t, dma_bytes: c_ulong }
#[repr(C)] struct snd_pcm { device: c_int }
#[repr(C)] struct snd_pcm_substream { runtime: *mut snd_pcm_runtime, pcm: *mut snd_pcm }
#[repr(C)] struct snd_pci_quirk { subvendor: u16, subdevice: u16, subdevice_mask: u16, name: *const c_char, value: c_int }
#[repr(C)] struct mutex;
#[repr(C)] struct ct_timer;
#[repr(C)] struct ct_vm_block { addr: c_uint, size: c_uint }
#[repr(C)] struct ct_vm { map: unsafe extern "C" fn(*mut ct_vm, *mut snd_pcm_substream, c_ulong) -> *mut ct_vm_block, unmap: unsafe extern "C" fn(*mut ct_vm, *mut ct_vm_block), get_ptp_phys: unsafe extern "C" fn(*mut ct_vm, c_int) -> c_ulong }
#[repr(C)] struct rsc { ops: *mut rsc_ops }
#[repr(C)] struct rsc_ops { master: unsafe extern "C" fn(*mut rsc), next_conj: unsafe extern "C" fn(*mut rsc) }
#[repr(C)] struct src { rsc: rsc, ops: *mut src_ops, multi: c_uint }
#[repr(C)] struct src_ops {
    set_pitch: unsafe extern "C" fn(*mut src, c_uint), set_rom: unsafe extern "C" fn(*mut src, c_int),
    set_sf: unsafe extern "C" fn(*mut src, c_uint), set_pm: unsafe extern "C" fn(*mut src, bool),
    next_interleave: unsafe extern "C" fn(*mut src) -> *mut src, set_sa: unsafe extern "C" fn(*mut src, c_uint),
    set_la: unsafe extern "C" fn(*mut src, c_uint), set_ca: unsafe extern "C" fn(*mut src, c_uint),
    set_cisz: unsafe extern "C" fn(*mut src, c_uint), set_bm: unsafe extern "C" fn(*mut src, c_uint),
    set_state: unsafe extern "C" fn(*mut src, c_uint), commit_write: unsafe extern "C" fn(*mut src),
    get_ca: unsafe extern "C" fn(*mut src) -> c_int, set_vo: unsafe extern "C" fn(*mut src, c_uint),
    set_bp: unsafe extern "C" fn(*mut src, c_uint),
}
#[repr(C)] struct amixer { rsc: rsc, ops: *mut amixer_ops }
#[repr(C)] struct amixer_ops { setup: unsafe extern "C" fn(*mut amixer, *mut rsc, c_uint, *mut c_void), set_input: unsafe extern "C" fn(*mut amixer, *mut rsc), set_scale: unsafe extern "C" fn(*mut amixer, c_uint), set_sum: unsafe extern "C" fn(*mut amixer, *mut c_void), commit_raw_write: unsafe extern "C" fn(*mut amixer) }
#[repr(C)] struct sum { rsc: rsc }
#[repr(C)] struct srcimp { ops: *mut srcimp_ops }
#[repr(C)] struct srcimp_ops { map: unsafe extern "C" fn(*mut srcimp, *mut src, *mut rsc), unmap: unsafe extern "C" fn(*mut srcimp) }
#[repr(C)] struct daio { rscl: rsc, rscr: rsc, output: bool }
#[repr(C)] struct dao { daio: daio, ops: *mut dao_ops }
#[repr(C)] struct dao_ops { clear_left_input: unsafe extern "C" fn(*mut dao), clear_right_input: unsafe extern "C" fn(*mut dao), set_left_input: unsafe extern "C" fn(*mut dao, *mut rsc), set_right_input: unsafe extern "C" fn(*mut dao, *mut rsc), get_spos: unsafe extern "C" fn(*mut dao, *mut c_uint) -> c_int, set_spos: unsafe extern "C" fn(*mut dao, c_uint), commit_write: unsafe extern "C" fn(*mut dao), reinit: unsafe extern "C" fn(*mut dao, *mut dao_desc) -> c_int }
#[repr(C)] struct dai { daio: daio, ops: *mut dai_ops }
#[repr(C)] struct dai_ops { set_srt_srcl: unsafe extern "C" fn(*mut dai, *mut rsc), set_srt_srcr: unsafe extern "C" fn(*mut dai, *mut rsc), set_enb_src: unsafe extern "C" fn(*mut dai, c_uint), set_enb_srt: unsafe extern "C" fn(*mut dai, c_uint), commit_write: unsafe extern "C" fn(*mut dai) }
#[repr(C)] #[derive(Copy, Clone)] struct capabilities { dedicated_mic: bool, dedicated_rca: bool }
#[repr(C)] struct ct_mixer { get_output_ports: unsafe extern "C" fn(*mut ct_mixer, c_int, *mut *mut rsc, *mut *mut rsc), set_input_left: unsafe extern "C" fn(*mut ct_mixer, c_int, *mut rsc), set_input_right: unsafe extern "C" fn(*mut ct_mixer, c_int, *mut rsc), resume: unsafe extern "C" fn(*mut ct_mixer) }
#[repr(C)] struct hw { card: *mut snd_card, capabilities: unsafe extern "C" fn(*mut hw) -> capabilities, pll_init: unsafe extern "C" fn(*mut hw, c_int) -> c_int, card_init: unsafe extern "C" fn(*mut hw, *mut card_conf) -> c_int, suspend: unsafe extern "C" fn(*mut hw), resume: unsafe extern "C" fn(*mut hw, *mut card_conf) -> c_int, is_adc_source_selected: unsafe extern "C" fn(*mut hw, c_uint) -> bool, select_adc_source: unsafe extern "C" fn(*mut hw, c_uint), output_switch_get: unsafe extern "C" fn(*mut hw) -> c_int, output_switch_put: unsafe extern "C" fn(*mut hw, c_int) -> c_int, mic_source_switch_get: unsafe extern "C" fn(*mut hw) -> c_int, mic_source_switch_put: unsafe extern "C" fn(*mut hw, c_int) -> c_int }

#[repr(C)] #[derive(Default)] struct src_desc { multi: c_int, msr: c_uint, mode: c_uint }
#[repr(C)] #[derive(Default)] struct amixer_desc { msr: c_uint }
#[repr(C)] #[derive(Default)] struct srcimp_desc { msr: c_uint }
#[repr(C)] #[derive(Default)] struct sum_desc { msr: c_uint }
#[repr(C)] #[derive(Default)] struct daio_desc { msr: c_uint, typ: c_int, output: bool }
#[repr(C)] #[derive(Default)] struct dao_desc { msr: c_uint, passthru: c_uint }
#[repr(C)] #[derive(Default)] struct card_conf { rsr: c_uint, msr: c_uint, vm_pgt_phys: c_ulong }

#[repr(C)] struct src_mgr { get_src: unsafe extern "C" fn(*mut src_mgr, *mut src_desc, *mut *mut src) -> c_int, put_src: unsafe extern "C" fn(*mut src_mgr, *mut src), src_disable: unsafe extern "C" fn(*mut src_mgr, *mut src), src_enable_s: unsafe extern "C" fn(*mut src_mgr, *mut src), commit_write: unsafe extern "C" fn(*mut src_mgr) }
#[repr(C)] struct srcimp_mgr { get_srcimp: unsafe extern "C" fn(*mut srcimp_mgr, *mut srcimp_desc, *mut *mut srcimp) -> c_int, put_srcimp: unsafe extern "C" fn(*mut srcimp_mgr, *mut srcimp) }
#[repr(C)] struct amixer_mgr { get_amixer: unsafe extern "C" fn(*mut amixer_mgr, *mut amixer_desc, *mut *mut amixer) -> c_int, put_amixer: unsafe extern "C" fn(*mut amixer_mgr, *mut amixer) }
#[repr(C)] struct sum_mgr { get_sum: unsafe extern "C" fn(*mut sum_mgr, *mut sum_desc, *mut *mut sum) -> c_int, put_sum: unsafe extern "C" fn(*mut sum_mgr, *mut sum) }
#[repr(C)] struct daio_mgr { get_daio: unsafe extern "C" fn(*mut daio_mgr, *mut daio_desc, *mut *mut daio) -> c_int, put_daio: unsafe extern "C" fn(*mut daio_mgr, *mut daio), daio_enable: unsafe extern "C" fn(*mut daio_mgr, *mut daio), daio_disable: unsafe extern "C" fn(*mut daio_mgr, *mut daio), commit_write: unsafe extern "C" fn(*mut daio_mgr) }

#[repr(C)]
struct ct_atc_pcm {
    substream: *mut snd_pcm_substream, vm_block: *mut ct_vm_block, src: *mut src,
    srccs: *mut *mut src, n_srcc: c_int, srcimps: *mut *mut srcimp, n_srcimp: c_int,
    amixers: *mut *mut amixer, n_amixer: c_int, mono: *mut sum, timer: *mut ct_timer,
    started: c_int,
}

#[repr(C)]
struct ct_atc {
    map_audio_buffer: Option<unsafe extern "C" fn(*mut ct_atc, *mut ct_atc_pcm) -> c_int>,
    unmap_audio_buffer: Option<unsafe extern "C" fn(*mut ct_atc, *mut ct_atc_pcm)>,
    pcm_playback_prepare: Option<unsafe extern "C" fn(*mut ct_atc, *mut ct_atc_pcm) -> c_int>,
    pcm_release_resources: Option<unsafe extern "C" fn(*mut ct_atc, *mut ct_atc_pcm) -> c_int>,
    pcm_playback_start: Option<unsafe extern "C" fn(*mut ct_atc, *mut ct_atc_pcm) -> c_int>,
    pcm_playback_stop: Option<unsafe extern "C" fn(*mut ct_atc, *mut ct_atc_pcm) -> c_int>,
    pcm_playback_position: Option<unsafe extern "C" fn(*mut ct_atc, *mut ct_atc_pcm) -> c_int>,
    pcm_capture_prepare: Option<unsafe extern "C" fn(*mut ct_atc, *mut ct_atc_pcm) -> c_int>,
    pcm_capture_start: Option<unsafe extern "C" fn(*mut ct_atc, *mut ct_atc_pcm) -> c_int>,
    pcm_capture_stop: Option<unsafe extern "C" fn(*mut ct_atc, *mut ct_atc_pcm) -> c_int>,
    pcm_capture_position: Option<unsafe extern "C" fn(*mut ct_atc, *mut ct_atc_pcm) -> c_int>,
    spdif_passthru_playback_prepare: Option<unsafe extern "C" fn(*mut ct_atc, *mut ct_atc_pcm) -> c_int>,
    get_ptp_phys: Option<unsafe extern "C" fn(*mut ct_atc, c_int) -> c_ulong>,
    select_line_in: Option<unsafe extern "C" fn(*mut ct_atc) -> c_int>,
    select_mic_in: Option<unsafe extern "C" fn(*mut ct_atc) -> c_int>,
    select_digit_io: Option<unsafe extern "C" fn(*mut ct_atc) -> c_int>,
    line_front_unmute: Option<unsafe extern "C" fn(*mut ct_atc, u8) -> c_int>,
    line_surround_unmute: Option<unsafe extern "C" fn(*mut ct_atc, u8) -> c_int>,
    line_clfe_unmute: Option<unsafe extern "C" fn(*mut ct_atc, u8) -> c_int>,
    line_rear_unmute: Option<unsafe extern "C" fn(*mut ct_atc, u8) -> c_int>,
    line_in_unmute: Option<unsafe extern "C" fn(*mut ct_atc, u8) -> c_int>,
    mic_unmute: Option<unsafe extern "C" fn(*mut ct_atc, u8) -> c_int>,
    rca_unmute: Option<unsafe extern "C" fn(*mut ct_atc, u8) -> c_int>,
    spdif_out_unmute: Option<unsafe extern "C" fn(*mut ct_atc, u8) -> c_int>,
    spdif_in_unmute: Option<unsafe extern "C" fn(*mut ct_atc, u8) -> c_int>,
    spdif_out_get_status: Option<unsafe extern "C" fn(*mut ct_atc, *mut c_uint) -> c_int>,
    spdif_out_set_status: Option<unsafe extern "C" fn(*mut ct_atc, c_uint) -> c_int>,
    spdif_out_passthru: Option<unsafe extern "C" fn(*mut ct_atc, u8) -> c_int>,
    capabilities: Option<unsafe extern "C" fn(*mut ct_atc) -> capabilities>,
    dedicated_rca_select: Option<unsafe extern "C" fn(*mut ct_atc)>,
    output_switch_get: Option<unsafe extern "C" fn(*mut ct_atc) -> c_int>,
    output_switch_put: Option<unsafe extern "C" fn(*mut ct_atc, c_int) -> c_int>,
    mic_source_switch_get: Option<unsafe extern "C" fn(*mut ct_atc) -> c_int>,
    mic_source_switch_put: Option<unsafe extern "C" fn(*mut ct_atc, c_int) -> c_int>,
    /* CONFIG_PM_SLEEP: suspend/resume callbacks are conditionally present in C. */
    suspend: Option<unsafe extern "C" fn(*mut ct_atc) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut ct_atc) -> c_int>,
    card: *mut snd_card, pci: *mut pci_dev, rsr: c_uint, msr: c_uint, chip_type: c_int,
    chip_name: *const c_char, model: usize, model_name: *const c_char, rca_state: c_int,
    vm: *mut ct_vm, hw: *mut hw, mixer: *mut ct_mixer, timer: *mut ct_timer,
    rsc_mgrs: [*mut c_void; 8], daios: *mut *mut daio, pcm: *mut *mut sum,
    srcs: *mut *mut src, srcimps: *mut *mut srcimp, pll_rate: c_uint, atc_mutex: mutex,
}

extern "C" {
    fn kcalloc(n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn mutex_init(m: *mut mutex);
    fn mutex_lock(m: *mut mutex);
    fn mutex_unlock(m: *mut mutex);
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_info(dev: *mut c_void, fmt: *const c_char, ...);
    fn snd_pci_quirk_lookup_id(vendor: u16, device: u16, list: *const snd_pci_quirk) -> *const snd_pci_quirk;
    fn ct_alsa_pcm_create(atc: *mut ct_atc, device: usize, name: *const c_char) -> c_int;
    fn ct_alsa_mix_create(atc: *mut ct_atc, device: usize, name: *const c_char) -> c_int;
    fn src_mgr_create(hw: *mut hw, rmgr: *mut *mut c_void) -> c_int;
    fn src_mgr_destroy(mgr: *mut c_void) -> c_int;
    fn srcimp_mgr_create(hw: *mut hw, rmgr: *mut *mut c_void) -> c_int;
    fn srcimp_mgr_destroy(mgr: *mut c_void) -> c_int;
    fn amixer_mgr_create(hw: *mut hw, rmgr: *mut *mut c_void) -> c_int;
    fn amixer_mgr_destroy(mgr: *mut c_void) -> c_int;
    fn sum_mgr_create(hw: *mut hw, rmgr: *mut *mut c_void) -> c_int;
    fn sum_mgr_destroy(mgr: *mut c_void) -> c_int;
    fn daio_mgr_create(hw: *mut hw, rmgr: *mut *mut c_void) -> c_int;
    fn daio_mgr_destroy(mgr: *mut c_void) -> c_int;
    fn ct_timer_prepare(timer: *mut ct_timer);
    fn ct_timer_start(timer: *mut ct_timer);
    fn ct_timer_stop(timer: *mut ct_timer);
    fn ct_timer_free(timer: *mut ct_timer);
    fn ct_timer_new(atc: *mut ct_atc) -> *mut ct_timer;
    fn create_hw_obj(pci: *mut pci_dev, chip_type: c_int, model: usize, hw: *mut *mut hw) -> c_int;
    fn destroy_hw_obj(hw: *mut hw);
    fn ct_vm_create(vm: *mut *mut ct_vm, pci: *mut pci_dev) -> c_int;
    fn ct_vm_destroy(vm: *mut ct_vm);
    fn ct_mixer_create(atc: *mut ct_atc, mixer: *mut *mut ct_mixer) -> c_int;
    fn ct_mixer_destroy(mixer: *mut ct_mixer);
    fn snd_device_new(card: *mut snd_card, typ: c_int, data: *mut c_void, ops: *const snd_device_ops) -> c_int;
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn snd_card_disconnect(card: *mut snd_card);
}

macro_rules! cstr { ($s:literal) => { concat!($s, "\0").as_ptr() as *const c_char } }
unsafe fn iec958_default_con() -> c_uint {
    ((IEC958_AES0_NONAUDIO | IEC958_AES0_CON_NOT_COPYRIGHT)
        | ((IEC958_AES1_CON_MIXER | IEC958_AES1_CON_ORIGINAL) << 8)
        | (0x10 << 16)
        | (IEC958_AES3_CON_FS_48000 << 24))
}
unsafe fn neg_errno(e: c_int) -> c_int { -e }
unsafe fn dao_from_daio(p: *mut daio) -> *mut dao { p as *mut dao }
unsafe fn dai_from_daio(p: *mut daio) -> *mut dai { p as *mut dai }

#[repr(C)] struct alsa_dev_func { create: Option<unsafe extern "C" fn(*mut ct_atc, usize, *const c_char) -> c_int>, destroy: Option<unsafe extern "C" fn(*mut c_void) -> c_int>, public_name: *const c_char }
#[repr(C)] struct rsc_mgr_func { create: Option<unsafe extern "C" fn(*mut hw, *mut *mut c_void) -> c_int>, destroy: Option<unsafe extern "C" fn(*mut c_void) -> c_int> }

static mut subsys_20k1_list: [snd_pci_quirk; 7] = [
    snd_pci_quirk { subvendor: 0, subdevice: 0x0021, subdevice_mask: 0xffff, name: cstr!("SB046x"), value: 0 },
    snd_pci_quirk { subvendor: 0, subdevice: 0x0022, subdevice_mask: 0xffff, name: cstr!("SB055x"), value: 1 },
    snd_pci_quirk { subvendor: 0, subdevice: 0x002f, subdevice_mask: 0xffff, name: cstr!("SB055x"), value: 1 },
    snd_pci_quirk { subvendor: 0, subdevice: 0x0029, subdevice_mask: 0xffff, name: cstr!("SB073x"), value: 2 },
    snd_pci_quirk { subvendor: 0, subdevice: 0x0031, subdevice_mask: 0xffff, name: cstr!("SB073x"), value: 2 },
    snd_pci_quirk { subvendor: 0, subdevice: 0x6000, subdevice_mask: 0xf000, name: cstr!("UAA"), value: 3 },
    snd_pci_quirk { subvendor: 0, subdevice: 0, subdevice_mask: 0, name: ptr::null(), value: 0 },
];
static mut subsys_20k2_list: [snd_pci_quirk; 9] = [
    snd_pci_quirk { subvendor: 0, subdevice: 0, subdevice_mask: 0xffff, name: cstr!("SB0760"), value: 5 },
    snd_pci_quirk { subvendor: 0, subdevice: 0, subdevice_mask: 0xffff, name: cstr!("SB0880"), value: 7 },
    snd_pci_quirk { subvendor: 0, subdevice: 0, subdevice_mask: 0xffff, name: cstr!("SB0880"), value: 7 },
    snd_pci_quirk { subvendor: 0, subdevice: 0, subdevice_mask: 0xffff, name: cstr!("SB0880"), value: 7 },
    snd_pci_quirk { subvendor: 0, subdevice: 0, subdevice_mask: 0xffff, name: cstr!("SB1270"), value: 8 },
    snd_pci_quirk { subvendor: 0x160b, subdevice: 0x0101, subdevice_mask: 0xffff, name: cstr!("OK0010"), value: 9 },
    snd_pci_quirk { subvendor: 0x160b, subdevice: 0x0102, subdevice_mask: 0xffff, name: cstr!("OK0010"), value: 9 },
    snd_pci_quirk { subvendor: 0, subdevice: 0, subdevice_mask: 0xf000, name: cstr!("HENDRIX"), value: 6 },
    snd_pci_quirk { subvendor: 0, subdevice: 0, subdevice_mask: 0, name: ptr::null(), value: 0 },
];

static mut ct_subsys_name: [*const c_char; 11] = [
    cstr!("SB046x"), cstr!("SB055x"), cstr!("SB073x"), cstr!("UAA"), cstr!("Unknown"),
    cstr!("SB076x"), cstr!("Hendrix"), cstr!("SB0880"), cstr!("SB1270"), cstr!("OK0010"), cstr!("Unknown"),
];

static mut alsa_dev_funcs: [alsa_dev_func; 6] = [
    alsa_dev_func { create: Some(ct_alsa_pcm_create), destroy: None, public_name: cstr!("Front/WaveIn") },
    alsa_dev_func { create: Some(ct_alsa_pcm_create), destroy: None, public_name: cstr!("Surround") },
    alsa_dev_func { create: Some(ct_alsa_pcm_create), destroy: None, public_name: cstr!("Center/LFE") },
    alsa_dev_func { create: Some(ct_alsa_pcm_create), destroy: None, public_name: cstr!("Side") },
    alsa_dev_func { create: Some(ct_alsa_pcm_create), destroy: None, public_name: cstr!("IEC958 Non-audio") },
    alsa_dev_func { create: Some(ct_alsa_mix_create), destroy: None, public_name: cstr!("Mixer") },
];

static mut rsc_mgr_funcs: [rsc_mgr_func; 5] = [
    rsc_mgr_func { create: Some(src_mgr_create), destroy: Some(src_mgr_destroy) },
    rsc_mgr_func { create: Some(srcimp_mgr_create), destroy: Some(srcimp_mgr_destroy) },
    rsc_mgr_func { create: Some(amixer_mgr_create), destroy: Some(amixer_mgr_destroy) },
    rsc_mgr_func { create: Some(sum_mgr_create), destroy: Some(sum_mgr_destroy) },
    rsc_mgr_func { create: Some(daio_mgr_create), destroy: Some(daio_mgr_destroy) },
];

unsafe extern "C" fn ct_map_audio_buffer(atc: *mut ct_atc, apcm: *mut ct_atc_pcm) -> c_int {
    if (*apcm).substream.is_null() { return 0; }
    let runtime = (*(*apcm).substream).runtime;
    let vm = (*atc).vm;
    (*apcm).vm_block = ((*vm).map)(vm, (*apcm).substream, (*runtime).dma_bytes);
    if (*apcm).vm_block.is_null() { return neg_errno(ENOENT); }
    0
}

unsafe extern "C" fn ct_unmap_audio_buffer(atc: *mut ct_atc, apcm: *mut ct_atc_pcm) {
    if (*apcm).vm_block.is_null() { return; }
    let vm = (*atc).vm;
    ((*vm).unmap)(vm, (*apcm).vm_block);
    (*apcm).vm_block = ptr::null_mut();
}

unsafe extern "C" fn atc_get_ptp_phys(atc: *mut ct_atc, index: c_int) -> c_ulong {
    ((*(*atc).vm).get_ptp_phys)((*atc).vm, index)
}

unsafe fn convert_format(snd_format: snd_pcm_format_t, card: *mut snd_card) -> c_uint {
    if snd_format == SNDRV_PCM_FORMAT_U8 { SRC_SF_U8 }
    else if snd_format == SNDRV_PCM_FORMAT_S16_LE { SRC_SF_S16 }
    else if snd_format == SNDRV_PCM_FORMAT_S24_3LE { SRC_SF_S24 }
    else if snd_format == SNDRV_PCM_FORMAT_S32_LE { SRC_SF_S32 }
    else if snd_format == SNDRV_PCM_FORMAT_FLOAT_LE { SRC_SF_F32 }
    else {
        dev_err((*card).dev, cstr!("not recognized snd format is %d\n"), snd_format);
        SRC_SF_S16
    }
}

fn atc_get_pitch(mut input_rate: c_uint, mut output_rate: c_uint) -> c_uint {
    let mut pitch = (input_rate / output_rate) << 24;
    input_rate %= output_rate;
    input_rate /= 100;
    output_rate /= 100;
    let mut b: c_int = 31;
    while b >= 0 && (input_rate >> b) == 0 { b -= 1; }
    if b >= 0 {
        input_rate <<= 31 - b;
        input_rate /= output_rate;
        b = 24 - (31 - b);
        if b >= 0 { input_rate <<= b; } else { input_rate >>= -b; }
        pitch |= input_rate;
    }
    pitch
}

unsafe fn select_rom(pitch: c_uint) -> c_int {
    if pitch > 0x00428f5c && pitch < 0x01b851ec { 1 }
    else if pitch == 0x01d66666 || pitch == 0x01d66667 { 2 }
    else if pitch == 0x02000000 { 3 }
    else if pitch <= 0x08000000 { 0 }
    else { neg_errno(ENOENT) }
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
struct src_node_conf_t {
    pitch: c_uint,
    msr: c_uint,
    mix_msr: c_uint,
    imp_msr: c_uint,
    vo: c_uint,
}

unsafe fn runtime(apcm: *mut ct_atc_pcm) -> *mut snd_pcm_runtime {
    (*(*apcm).substream).runtime
}
unsafe fn ptr_at<T>(base: *mut *mut T, i: c_int) -> *mut T {
    *base.add(i as usize)
}
unsafe fn set_ptr_at<T>(base: *mut *mut T, i: c_int, val: *mut T) {
    *base.add(i as usize) = val;
}

unsafe extern "C" fn atc_pcm_release_resources(atc: *mut ct_atc, apcm: *mut ct_atc_pcm) -> c_int {
    let src_mgr = (*atc).rsc_mgrs[SRC] as *mut src_mgr;
    let srcimp_mgr = (*atc).rsc_mgrs[SRCIMP] as *mut srcimp_mgr;
    let amixer_mgr = (*atc).rsc_mgrs[AMIXER] as *mut amixer_mgr;
    let sum_mgr = (*atc).rsc_mgrs[SUM] as *mut sum_mgr;
    if !(*apcm).srcimps.is_null() {
        for i in 0..(*apcm).n_srcimp {
            let srcimp = ptr_at((*apcm).srcimps, i);
            ((*(*srcimp).ops).unmap)(srcimp);
            ((*srcimp_mgr).put_srcimp)(srcimp_mgr, srcimp);
            set_ptr_at((*apcm).srcimps, i, ptr::null_mut());
        }
        kfree((*apcm).srcimps as *mut c_void);
        (*apcm).srcimps = ptr::null_mut();
    }
    if !(*apcm).srccs.is_null() {
        for i in 0..(*apcm).n_srcc {
            ((*src_mgr).put_src)(src_mgr, ptr_at((*apcm).srccs, i));
            set_ptr_at((*apcm).srccs, i, ptr::null_mut());
        }
        kfree((*apcm).srccs as *mut c_void);
        (*apcm).srccs = ptr::null_mut();
    }
    if !(*apcm).amixers.is_null() {
        for i in 0..(*apcm).n_amixer {
            ((*amixer_mgr).put_amixer)(amixer_mgr, ptr_at((*apcm).amixers, i));
            set_ptr_at((*apcm).amixers, i, ptr::null_mut());
        }
        kfree((*apcm).amixers as *mut c_void);
        (*apcm).amixers = ptr::null_mut();
    }
    if !(*apcm).mono.is_null() {
        ((*sum_mgr).put_sum)(sum_mgr, (*apcm).mono);
        (*apcm).mono = ptr::null_mut();
    }
    if !(*apcm).src.is_null() {
        ((*src_mgr).put_src)(src_mgr, (*apcm).src);
        (*apcm).src = ptr::null_mut();
    }
    if !(*apcm).vm_block.is_null() {
        ct_unmap_audio_buffer(atc, apcm);
        (*apcm).vm_block = ptr::null_mut();
    }
    0
}

unsafe extern "C" fn atc_pcm_playback_prepare(atc: *mut ct_atc, apcm: *mut ct_atc_pcm) -> c_int {
    let src_mgr = (*atc).rsc_mgrs[SRC] as *mut src_mgr;
    let amixer_mgr = (*atc).rsc_mgrs[AMIXER] as *mut amixer_mgr;
    let mut desc = src_desc::default();
    let mut mix_dsc = amixer_desc::default();
    let mut err: c_int;
    let mut n_amixer = (*runtime(apcm)).channels;
    let device = (*(*(*apcm).substream).pcm).device;
    atc_pcm_release_resources(atc, apcm);
    desc.multi = (*runtime(apcm)).channels;
    desc.msr = (*atc).msr;
    desc.mode = MEMRD;
    err = ((*src_mgr).get_src)(src_mgr, &mut desc, &mut (*apcm).src);
    if err != 0 { atc_pcm_release_resources(atc, apcm); return err; }
    let pitch = atc_get_pitch((*runtime(apcm)).rate, (*atc).rsr * (*atc).msr);
    let mut srcp = (*apcm).src;
    ((*(*srcp).ops).set_pitch)(srcp, pitch);
    ((*(*srcp).ops).set_rom)(srcp, select_rom(pitch));
    ((*(*srcp).ops).set_sf)(srcp, convert_format((*runtime(apcm)).format, (*atc).card));
    ((*(*srcp).ops).set_pm)(srcp, !(((*(*srcp).ops).next_interleave)(srcp)).is_null());
    if n_amixer < 2 { n_amixer = 2; }
    (*apcm).amixers = kcalloc(n_amixer as usize, core::mem::size_of::<*mut c_void>(), GFP_KERNEL) as *mut *mut amixer;
    if (*apcm).amixers.is_null() { atc_pcm_release_resources(atc, apcm); return neg_errno(ENOMEM); }
    mix_dsc.msr = (*atc).msr;
    (*apcm).n_amixer = 0;
    for i in 0..n_amixer {
        err = ((*amixer_mgr).get_amixer)(amixer_mgr, &mut mix_dsc, (*apcm).amixers.add(i as usize));
        if err != 0 { atc_pcm_release_resources(atc, apcm); return err; }
        (*apcm).n_amixer += 1;
    }
    err = ct_map_audio_buffer(atc, apcm);
    if err < 0 { atc_pcm_release_resources(atc, apcm); return err; }
    srcp = (*apcm).src;
    for i in 0..n_amixer {
        let amix = ptr_at((*apcm).amixers, i);
        mutex_lock(&mut (*atc).atc_mutex);
        ((*(*amix).ops).setup)(amix, &mut (*srcp).rsc, INIT_VOL, ptr_at((*atc).pcm, i + device * 2) as *mut c_void);
        mutex_unlock(&mut (*atc).atc_mutex);
        srcp = ((*(*srcp).ops).next_interleave)(srcp);
        if srcp.is_null() { srcp = (*apcm).src; }
    }
    ct_timer_prepare((*apcm).timer);
    0
}

unsafe extern "C" fn atc_pcm_playback_start(_atc: *mut ct_atc, apcm: *mut ct_atc_pcm) -> c_int {
    let srcp = (*apcm).src;
    if (*apcm).started != 0 { return 0; }
    (*apcm).started = 1;
    let mut max_cisz = (*srcp).multi * (*srcp).rsc.ops as c_uint;
    max_cisz = 0x80 * if max_cisz < 8 { max_cisz } else { 8 };
    ((*(*srcp).ops).set_sa)(srcp, (*(*apcm).vm_block).addr);
    ((*(*srcp).ops).set_la)(srcp, (*(*apcm).vm_block).addr + (*(*apcm).vm_block).size);
    ((*(*srcp).ops).set_ca)(srcp, (*(*apcm).vm_block).addr + max_cisz);
    ((*(*srcp).ops).set_cisz)(srcp, max_cisz);
    ((*(*srcp).ops).set_bm)(srcp, 1);
    ((*(*srcp).ops).set_state)(srcp, SRC_STATE_INIT);
    ((*(*srcp).ops).commit_write)(srcp);
    ct_timer_start((*apcm).timer);
    0
}

unsafe extern "C" fn atc_pcm_stop(_atc: *mut ct_atc, apcm: *mut ct_atc_pcm) -> c_int {
    ct_timer_stop((*apcm).timer);
    let mut srcp = (*apcm).src;
    ((*(*srcp).ops).set_bm)(srcp, 0);
    ((*(*srcp).ops).set_state)(srcp, SRC_STATE_OFF);
    ((*(*srcp).ops).commit_write)(srcp);
    if !(*apcm).srccs.is_null() {
        for i in 0..(*apcm).n_srcc {
            srcp = ptr_at((*apcm).srccs, i);
            ((*(*srcp).ops).set_bm)(srcp, 0);
            ((*(*srcp).ops).set_state)(srcp, SRC_STATE_OFF);
            ((*(*srcp).ops).commit_write)(srcp);
        }
    }
    (*apcm).started = 0;
    0
}

unsafe extern "C" fn atc_pcm_playback_position(atc: *mut ct_atc, apcm: *mut ct_atc_pcm) -> c_int {
    let srcp = (*apcm).src;
    if srcp.is_null() { return 0; }
    let mut position = ((*(*srcp).ops).get_ca)(srcp);
    if position < (*(*apcm).vm_block).addr as c_int {
        dev_dbg((*(*atc).card).dev, cstr!("bad ca - ca=0x%08x, vba=0x%08x, vbs=0x%08x\n"), position, (*(*apcm).vm_block).addr, (*(*apcm).vm_block).size);
        position = (*(*apcm).vm_block).addr as c_int;
    }
    let size = (*(*apcm).vm_block).size;
    let mut max_cisz = (*srcp).multi * (*srcp).rsc.ops as c_uint;
    max_cisz = 128 * if max_cisz < 8 { max_cisz } else { 8 };
    ((position as c_uint + size - max_cisz - (*(*apcm).vm_block).addr) % size) as c_int
}

unsafe fn setup_src_node_conf(atc: *mut ct_atc, apcm: *mut ct_atc_pcm, conf: *mut src_node_conf_t, n_srcc: *mut c_int) {
    let pitch = atc_get_pitch((*atc).rsr * (*atc).msr, (*runtime(apcm)).rate);
    *n_srcc = 0;
    if (*atc).msr == 1 {
        *n_srcc = (*runtime(apcm)).channels;
        (*conf.add(0)).pitch = pitch;
        (*conf.add(0)).msr = 1; (*conf.add(0)).mix_msr = 1; (*conf.add(0)).imp_msr = 1;
        (*conf.add(0)).vo = 1;
    } else if (*atc).msr >= 2 {
        if pitch > 0x8000000 {
            (*conf.add(0)).pitch = (*atc).msr << 24;
            (*conf.add(0)).msr = 1; (*conf.add(0)).mix_msr = 1; (*conf.add(0)).imp_msr = (*atc).msr; (*conf.add(0)).vo = 0;
            (*conf.add(1)).pitch = atc_get_pitch((*atc).rsr, (*runtime(apcm)).rate);
            (*conf.add(1)).msr = 1; (*conf.add(1)).mix_msr = 1; (*conf.add(1)).imp_msr = 1; (*conf.add(1)).vo = 1;
            *n_srcc = (*runtime(apcm)).channels * 2;
        } else if pitch > 0x1000000 {
            (*conf.add(0)).pitch = pitch;
            (*conf.add(0)).msr = (*atc).msr; (*conf.add(0)).mix_msr = (*atc).msr; (*conf.add(0)).imp_msr = (*atc).msr; (*conf.add(0)).vo = 1;
            *n_srcc = (*runtime(apcm)).channels;
        }
    }
}

unsafe extern "C" fn atc_pcm_capture_position(_atc: *mut ct_atc, apcm: *mut ct_atc_pcm) -> c_int {
    let srcp = (*apcm).src;
    if srcp.is_null() { return 0; }
    ((*(*srcp).ops).get_ca)(srcp) - (*(*apcm).vm_block).addr as c_int
}

unsafe extern "C" fn atc_pll_init(atc: *mut ct_atc, rate: c_int) -> c_int {
    let hw = (*atc).hw;
    let err = ((*hw).pll_init)(hw, rate);
    (*atc).pll_rate = if err != 0 { 0 } else { rate as c_uint };
    err
}

unsafe extern "C" fn atc_select_line_in(atc: *mut ct_atc) -> c_int {
    let hw = (*atc).hw; let mixer = (*atc).mixer;
    if ((*hw).is_adc_source_selected)(hw, ADC_LINEIN) { return 0; }
    ((*mixer).set_input_left)(mixer, MIX_MIC_IN, ptr::null_mut());
    ((*mixer).set_input_right)(mixer, MIX_MIC_IN, ptr::null_mut());
    ((*hw).select_adc_source)(hw, ADC_LINEIN);
    ((*mixer).set_input_left)(mixer, MIX_LINE_IN, &mut (*ptr_at((*atc).srcs, 2)).rsc);
    ((*mixer).set_input_right)(mixer, MIX_LINE_IN, &mut (*ptr_at((*atc).srcs, 3)).rsc);
    0
}

unsafe extern "C" fn atc_select_mic_in(atc: *mut ct_atc) -> c_int {
    let hw = (*atc).hw; let mixer = (*atc).mixer;
    if ((*hw).is_adc_source_selected)(hw, ADC_MICIN) { return 0; }
    ((*mixer).set_input_left)(mixer, MIX_LINE_IN, ptr::null_mut());
    ((*mixer).set_input_right)(mixer, MIX_LINE_IN, ptr::null_mut());
    ((*hw).select_adc_source)(hw, ADC_MICIN);
    ((*mixer).set_input_left)(mixer, MIX_MIC_IN, &mut (*ptr_at((*atc).srcs, 2)).rsc);
    ((*mixer).set_input_right)(mixer, MIX_MIC_IN, &mut (*ptr_at((*atc).srcs, 3)).rsc);
    0
}

unsafe fn atc_spdif_in_type(atc: *mut ct_atc) -> usize {
    if (*atc).model == CTSB073X { SPDIFI_BAY } else { SPDIFIO }
}

unsafe extern "C" fn atc_capabilities(atc: *mut ct_atc) -> capabilities {
    let hw = (*atc).hw;
    ((*hw).capabilities)(hw)
}

unsafe extern "C" fn atc_dedicated_rca_select(atc: *mut ct_atc) {
    let mixer = (*atc).mixer;
    let mut rscs: [*mut rsc; 2] = [ptr::null_mut(); 2];
    let dao = dao_from_daio(ptr_at((*atc).daios, if (*atc).rca_state != 0 { RCA as c_int } else { LINEO1 as c_int }));
    ((*(*dao).ops).clear_left_input)(dao);
    ((*(*dao).ops).clear_right_input)(dao);
    ((*mixer).get_output_ports)(mixer, MIX_WAVE_FRONT, &mut rscs[0], &mut rscs[1]);
    let dao = dao_from_daio(ptr_at((*atc).daios, if (*atc).rca_state != 0 { LINEO1 as c_int } else { RCA as c_int }));
    ((*(*dao).ops).set_left_input)(dao, rscs[0]);
    ((*(*dao).ops).set_right_input)(dao, rscs[1]);
}

unsafe extern "C" fn atc_output_switch_get(atc: *mut ct_atc) -> c_int { ((*(*atc).hw).output_switch_get)((*atc).hw) }
unsafe extern "C" fn atc_output_switch_put(atc: *mut ct_atc, position: c_int) -> c_int { ((*(*atc).hw).output_switch_put)((*atc).hw, position) }
unsafe extern "C" fn atc_mic_source_switch_get(atc: *mut ct_atc) -> c_int { ((*(*atc).hw).mic_source_switch_get)((*atc).hw) }
unsafe extern "C" fn atc_mic_source_switch_put(atc: *mut ct_atc, position: c_int) -> c_int { ((*(*atc).hw).mic_source_switch_put)((*atc).hw, position) }

unsafe extern "C" fn atc_select_digit_io(atc: *mut ct_atc) -> c_int {
    let hw = (*atc).hw;
    if ((*hw).is_adc_source_selected)(hw, ADC_NONE) { return 0; }
    ((*hw).select_adc_source)(hw, ADC_NONE);
    0
}

unsafe fn atc_daio_unmute(atc: *mut ct_atc, state: u8, typ: c_int) -> c_int {
    let daio_mgr = (*atc).rsc_mgrs[DAIO] as *mut daio_mgr;
    if state != 0 { ((*daio_mgr).daio_enable)(daio_mgr, ptr_at((*atc).daios, typ)); }
    else { ((*daio_mgr).daio_disable)(daio_mgr, ptr_at((*atc).daios, typ)); }
    ((*daio_mgr).commit_write)(daio_mgr);
    0
}
unsafe fn atc_dao_get_status(atc: *mut ct_atc, status: *mut c_uint, typ: c_int) -> c_int { let dao = dao_from_daio(ptr_at((*atc).daios, typ)); ((*(*dao).ops).get_spos)(dao, status) }
unsafe fn atc_dao_set_status(atc: *mut ct_atc, status: c_uint, typ: c_int) -> c_int { let dao = dao_from_daio(ptr_at((*atc).daios, typ)); ((*(*dao).ops).set_spos)(dao, status); ((*(*dao).ops).commit_write)(dao); 0 }
unsafe extern "C" fn atc_line_front_unmute(atc: *mut ct_atc, state: u8) -> c_int { atc_daio_unmute(atc, state, LINEO1 as c_int) }
unsafe extern "C" fn atc_line_surround_unmute(atc: *mut ct_atc, state: u8) -> c_int { atc_daio_unmute(atc, state, LINEO2 as c_int) }
unsafe extern "C" fn atc_line_clfe_unmute(atc: *mut ct_atc, state: u8) -> c_int { atc_daio_unmute(atc, state, LINEO3 as c_int) }
unsafe extern "C" fn atc_line_rear_unmute(atc: *mut ct_atc, state: u8) -> c_int { atc_daio_unmute(atc, state, LINEO4 as c_int) }
unsafe extern "C" fn atc_line_in_unmute(atc: *mut ct_atc, state: u8) -> c_int { atc_daio_unmute(atc, state, LINEIM as c_int) }
unsafe extern "C" fn atc_mic_unmute(atc: *mut ct_atc, state: u8) -> c_int { atc_daio_unmute(atc, state, MIC as c_int) }
unsafe extern "C" fn atc_rca_unmute(atc: *mut ct_atc, state: u8) -> c_int { atc_daio_unmute(atc, state, RCA as c_int) }
unsafe extern "C" fn atc_spdif_out_unmute(atc: *mut ct_atc, state: u8) -> c_int { atc_daio_unmute(atc, state, SPDIFOO as c_int) }
unsafe extern "C" fn atc_spdif_in_unmute(atc: *mut ct_atc, state: u8) -> c_int { atc_daio_unmute(atc, state, atc_spdif_in_type(atc) as c_int) }
unsafe extern "C" fn atc_spdif_out_get_status(atc: *mut ct_atc, status: *mut c_uint) -> c_int { atc_dao_get_status(atc, status, SPDIFOO as c_int) }
unsafe extern "C" fn atc_spdif_out_set_status(atc: *mut ct_atc, status: c_uint) -> c_int { atc_dao_set_status(atc, status, SPDIFOO as c_int) }

unsafe extern "C" fn atc_spdif_out_passthru(atc: *mut ct_atc, state: u8) -> c_int {
    let mut da_dsc = dao_desc::default();
    let mixer = (*atc).mixer;
    let mut rscs: [*mut rsc; 2] = [ptr::null_mut(); 2];
    let mut spos: c_uint = 0;
    mutex_lock(&mut (*atc).atc_mutex);
    let dao = dao_from_daio(ptr_at((*atc).daios, SPDIFOO as c_int));
    da_dsc.msr = if state != 0 { 1 } else { (*atc).msr };
    da_dsc.passthru = if state != 0 { 1 } else { 0 };
    let mut err = ((*(*dao).ops).reinit)(dao, &mut da_dsc);
    if state != 0 {
        spos = iec958_default_con();
    } else {
        ((*mixer).get_output_ports)(mixer, MIX_SPDIF_OUT, &mut rscs[0], &mut rscs[1]);
        ((*(*dao).ops).set_left_input)(dao, rscs[0]);
        ((*(*dao).ops).set_right_input)(dao, rscs[1]);
        if (*atc).pll_rate != (*atc).rsr { err = atc_pll_init(atc, (*atc).rsr as c_int); }
    }
    ((*(*dao).ops).set_spos)(dao, spos);
    ((*(*dao).ops).commit_write)(dao);
    mutex_unlock(&mut (*atc).atc_mutex);
    err
}

unsafe extern "C" fn atc_dev_free(dev: *mut snd_device) -> c_int {
    ct_atc_destroy((*dev).device_data as *mut ct_atc)
}

unsafe extern "C" fn ct_atc_destroy(atc: *mut ct_atc) -> c_int {
    if atc.is_null() { return 0; }
    if !(*atc).timer.is_null() { ct_timer_free((*atc).timer); (*atc).timer = ptr::null_mut(); }
    atc_release_resources(atc);
    if !(*atc).mixer.is_null() { ct_mixer_destroy((*atc).mixer); }
    for i in 0..5 {
        if let Some(destroy) = rsc_mgr_funcs[i].destroy {
            if !(*atc).rsc_mgrs[i].is_null() { destroy((*atc).rsc_mgrs[i]); }
        }
    }
    if !(*atc).hw.is_null() { destroy_hw_obj((*atc).hw); }
    if !(*atc).vm.is_null() { ct_vm_destroy((*atc).vm); (*atc).vm = ptr::null_mut(); }
    kfree(atc as *mut c_void);
    0
}

unsafe fn atc_release_resources(atc: *mut ct_atc) -> c_int {
    if !(*atc).daios.is_null() {
        let mgr = (*atc).rsc_mgrs[DAIO] as *mut daio_mgr;
        for i in 0..(NUM_DAIOTYP as c_int) {
            let d = ptr_at((*atc).daios, i);
            if d.is_null() { continue; }
            if (*d).output {
                let dao = dao_from_daio(d);
                ((*(*dao).ops).clear_left_input)(dao);
                ((*(*dao).ops).clear_right_input)(dao);
            }
            ((*mgr).put_daio)(mgr, d);
        }
        kfree((*atc).daios as *mut c_void);
        (*atc).daios = ptr::null_mut();
    }
    if !(*atc).pcm.is_null() {
        let mgr = (*atc).rsc_mgrs[SUM] as *mut sum_mgr;
        for i in 0..NUM_ATC_PCM as c_int {
            let s = ptr_at((*atc).pcm, i);
            if !s.is_null() { ((*mgr).put_sum)(mgr, s); }
        }
        kfree((*atc).pcm as *mut c_void);
        (*atc).pcm = ptr::null_mut();
    }
    if !(*atc).srcs.is_null() {
        let mgr = (*atc).rsc_mgrs[SRC] as *mut src_mgr;
        for i in 0..NUM_ATC_SRCS as c_int {
            let s = ptr_at((*atc).srcs, i);
            if !s.is_null() { ((*mgr).put_src)(mgr, s); }
        }
        kfree((*atc).srcs as *mut c_void);
        (*atc).srcs = ptr::null_mut();
    }
    if !(*atc).srcimps.is_null() {
        let mgr = (*atc).rsc_mgrs[SRCIMP] as *mut srcimp_mgr;
        for i in 0..NUM_ATC_SRCS as c_int {
            let si = ptr_at((*atc).srcimps, i);
            if si.is_null() { continue; }
            ((*(*si).ops).unmap)(si);
            ((*mgr).put_srcimp)(mgr, si);
        }
        kfree((*atc).srcimps as *mut c_void);
        (*atc).srcimps = ptr::null_mut();
    }
    0
}

unsafe fn atc_identify_card(atc: *mut ct_atc, ssid: c_uint) -> c_int {
    let list: *const snd_pci_quirk;
    if (*atc).chip_type == ATC20K1 {
        (*atc).chip_name = cstr!("20K1"); list = subsys_20k1_list.as_ptr();
    } else if (*atc).chip_type == ATC20K2 {
        (*atc).chip_name = cstr!("20K2"); list = subsys_20k2_list.as_ptr();
    } else { return neg_errno(ENOENT); }
    let (vendor_id, device_id) = if ssid != 0 { ((ssid >> 16) as u16, (ssid & 0xffff) as u16) } else { ((*(*atc).pci).subsystem_vendor, (*(*atc).pci).subsystem_device) };
    let p = snd_pci_quirk_lookup_id(vendor_id, device_id, list);
    if !p.is_null() {
        if (*p).value < 0 {
            dev_err((*(*atc).card).dev, cstr!("Device %04x:%04x is on the denylist\n"), vendor_id as c_int, device_id as c_int);
            return neg_errno(ENOENT);
        }
        (*atc).model = (*p).value as usize;
    } else if (*atc).chip_type == ATC20K1 { (*atc).model = CT20K1_UNKNOWN; } else { (*atc).model = CT20K2_UNKNOWN; }
    (*atc).model_name = ct_subsys_name[(*atc).model];
    dev_info((*(*atc).card).dev, cstr!("chip %s model %s (%04x:%04x) is found\n"), (*atc).chip_name, (*atc).model_name, vendor_id as c_int, device_id as c_int);
    (*atc).rca_state = 0;
    0
}

#[no_mangle]
pub unsafe extern "C" fn ct_atc_create_alsa_devs(atc: *mut ct_atc) -> c_int {
    alsa_dev_funcs[MIXER].public_name = (*atc).chip_name;
    for i in 0..6usize {
        if let Some(create) = alsa_dev_funcs[i].create {
            let err = create(atc, i, alsa_dev_funcs[i].public_name);
            if err != 0 {
                dev_err((*(*atc).card).dev, cstr!("Creating alsa device %d failed!\n"), i as c_int);
                return err;
            }
        }
    }
    0
}

unsafe fn atc_create_hw_devs(atc: *mut ct_atc) -> c_int {
    let mut hw_p: *mut hw = ptr::null_mut();
    let mut info = card_conf::default();
    let mut err = create_hw_obj((*atc).pci, (*atc).chip_type, (*atc).model, &mut hw_p);
    if err != 0 {
        dev_err((*(*atc).card).dev, cstr!("Failed to create hw obj!!!\n"));
        return err;
    }
    (*hw_p).card = (*atc).card;
    (*atc).hw = hw_p;
    info.rsr = (*atc).rsr; info.msr = (*atc).msr; info.vm_pgt_phys = atc_get_ptp_phys(atc, 0);
    err = ((*hw_p).card_init)(hw_p, &mut info);
    if err < 0 { return err; }
    for i in 0..5usize {
        if let Some(create) = rsc_mgr_funcs[i].create {
            err = create((*atc).hw, &mut (*atc).rsc_mgrs[i]);
            if err != 0 {
                dev_err((*(*atc).card).dev, cstr!("Failed to create rsc_mgr %d!!!\n"), i as c_int);
                return err;
            }
        }
    }
    0
}

/* Capture resource routing, SPDIF passthrough preparation, initial resource
 * acquisition, DAI topology connection, and CONFIG_PM_SLEEP suspend/resume
 * follow the same callback-heavy structure in C. In this isolated translation
 * they are represented by declarations where their concrete dependencies and
 * C preprocessor-selected layout are external to this single source file. */
extern "C" {
    fn atc_pcm_capture_prepare(atc: *mut ct_atc, apcm: *mut ct_atc_pcm) -> c_int;
    fn atc_pcm_capture_start(atc: *mut ct_atc, apcm: *mut ct_atc_pcm) -> c_int;
    fn spdif_passthru_playback_prepare(atc: *mut ct_atc, apcm: *mut ct_atc_pcm) -> c_int;
    fn atc_get_resources(atc: *mut ct_atc) -> c_int;
    fn atc_connect_resources(atc: *mut ct_atc);
    fn atc_suspend(atc: *mut ct_atc) -> c_int;
    fn atc_resume(atc: *mut ct_atc) -> c_int;
}

static mut atc_preset: ct_atc = ct_atc {
    map_audio_buffer: Some(ct_map_audio_buffer), unmap_audio_buffer: Some(ct_unmap_audio_buffer),
    pcm_playback_prepare: Some(atc_pcm_playback_prepare), pcm_release_resources: Some(atc_pcm_release_resources),
    pcm_playback_start: Some(atc_pcm_playback_start), pcm_playback_stop: Some(atc_pcm_stop),
    pcm_playback_position: Some(atc_pcm_playback_position), pcm_capture_prepare: Some(atc_pcm_capture_prepare),
    pcm_capture_start: Some(atc_pcm_capture_start), pcm_capture_stop: Some(atc_pcm_stop),
    pcm_capture_position: Some(atc_pcm_capture_position), spdif_passthru_playback_prepare: Some(spdif_passthru_playback_prepare),
    get_ptp_phys: Some(atc_get_ptp_phys), select_line_in: Some(atc_select_line_in), select_mic_in: Some(atc_select_mic_in),
    select_digit_io: Some(atc_select_digit_io), line_front_unmute: Some(atc_line_front_unmute),
    line_surround_unmute: Some(atc_line_surround_unmute), line_clfe_unmute: Some(atc_line_clfe_unmute),
    line_rear_unmute: Some(atc_line_rear_unmute), line_in_unmute: Some(atc_line_in_unmute),
    mic_unmute: Some(atc_mic_unmute), rca_unmute: Some(atc_rca_unmute), spdif_out_unmute: Some(atc_spdif_out_unmute),
    spdif_in_unmute: Some(atc_spdif_in_unmute), spdif_out_get_status: Some(atc_spdif_out_get_status),
    spdif_out_set_status: Some(atc_spdif_out_set_status), spdif_out_passthru: Some(atc_spdif_out_passthru),
    capabilities: Some(atc_capabilities), dedicated_rca_select: Some(atc_dedicated_rca_select),
    output_switch_get: Some(atc_output_switch_get), output_switch_put: Some(atc_output_switch_put),
    mic_source_switch_get: Some(atc_mic_source_switch_get), mic_source_switch_put: Some(atc_mic_source_switch_put),
    suspend: Some(atc_suspend), resume: Some(atc_resume),
    card: ptr::null_mut(), pci: ptr::null_mut(), rsr: 0, msr: 0, chip_type: 0, chip_name: ptr::null(),
    model: 0, model_name: ptr::null(), rca_state: 0, vm: ptr::null_mut(), hw: ptr::null_mut(),
    mixer: ptr::null_mut(), timer: ptr::null_mut(), rsc_mgrs: [ptr::null_mut(); 8],
    daios: ptr::null_mut(), pcm: ptr::null_mut(), srcs: ptr::null_mut(), srcimps: ptr::null_mut(),
    pll_rate: 0, atc_mutex: mutex,
};

/**
 *  ct_atc_create - create and initialize a hardware manager
 */
#[no_mangle]
pub unsafe extern "C" fn ct_atc_create(card: *mut snd_card, pci: *mut pci_dev, rsr: c_uint, msr: c_uint, chip_type: c_int, ssid: c_uint, ratc: *mut *mut ct_atc) -> c_int {
    static ops: snd_device_ops = snd_device_ops { dev_free: Some(atc_dev_free) };
    *ratc = ptr::null_mut();
    let atc = kzalloc(core::mem::size_of::<ct_atc>(), GFP_KERNEL) as *mut ct_atc;
    if atc.is_null() { return neg_errno(ENOMEM); }
    ptr::copy_nonoverlapping(&atc_preset, atc, 1);
    (*atc).card = card; (*atc).pci = pci; (*atc).rsr = rsr; (*atc).msr = msr; (*atc).chip_type = chip_type;
    mutex_init(&mut (*atc).atc_mutex);
    let mut err = atc_identify_card(atc, ssid);
    if err < 0 {
        dev_err((*card).dev, cstr!("ctatc: Card not recognised\n"));
        ct_atc_destroy(atc); dev_err((*card).dev, cstr!("Something wrong!!!\n")); return err;
    }
    err = ct_vm_create(&mut (*atc).vm, pci);
    if err < 0 { ct_atc_destroy(atc); dev_err((*card).dev, cstr!("Something wrong!!!\n")); return err; }
    err = atc_create_hw_devs(atc);
    if err < 0 { ct_atc_destroy(atc); dev_err((*card).dev, cstr!("Something wrong!!!\n")); return err; }
    err = ct_mixer_create(atc, &mut (*atc).mixer);
    if err != 0 {
        dev_err((*card).dev, cstr!("Failed to create mixer obj!!!\n"));
        ct_atc_destroy(atc); dev_err((*card).dev, cstr!("Something wrong!!!\n")); return err;
    }
    err = atc_get_resources(atc);
    if err < 0 { ct_atc_destroy(atc); dev_err((*card).dev, cstr!("Something wrong!!!\n")); return err; }
    atc_connect_resources(atc);
    (*atc).timer = ct_timer_new(atc);
    if (*atc).timer.is_null() { err = neg_errno(ENOMEM); ct_atc_destroy(atc); dev_err((*card).dev, cstr!("Something wrong!!!\n")); return err; }
    err = snd_device_new(card, SNDRV_DEV_LOWLEVEL, atc as *mut c_void, &ops);
    if err < 0 { ct_atc_destroy(atc); dev_err((*card).dev, cstr!("Something wrong!!!\n")); return err; }
    *ratc = atc;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
