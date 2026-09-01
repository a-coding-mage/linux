// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *                   Lee Revell <rlrevell@joe-job.com>
 *                   James Courtier-Dutton <James@superbug.co.uk>
 *                   Oswald Buddenhagen <oswald.buddenhagen@gmx.de>
 *                   Creative Labs, Inc.
 *
 *  Routines for control of EMU10K1 chips / PCM routines
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type size_t = usize;
type snd_pcm_uframes_t = usize;

// External kernel/ALSA/EMU10K1 declarations are supplied by the translated repository.
extern "C" {
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_emu10k1_voice_free(emu: *mut snd_emu10k1, voice: *mut snd_emu10k1_voice);
    fn snd_emu10k1_voice_alloc(emu: *mut snd_emu10k1, type_: c_int, count: c_int, channels: c_int, epcm: *mut snd_emu10k1_pcm, rvoice: *mut *mut snd_emu10k1_voice) -> c_int;
    fn snd_emu10k1_ptr_write(emu: *mut snd_emu10k1, reg: c_uint, chn: c_uint, data: c_uint);
    fn snd_emu10k1_ptr_read(emu: *mut snd_emu10k1, reg: c_uint, chn: c_uint) -> c_uint;
    fn snd_emu10k1_ptr_write_multiple(emu: *mut snd_emu10k1, chn: c_uint, ...);
    fn snd_emu10k1_compose_audigy_fxrt1(send_routing: *const c_uchar) -> c_uint;
    fn snd_emu10k1_compose_audigy_fxrt2(send_routing: *const c_uchar) -> c_uint;
    fn snd_emu10k1_compose_audigy_sendamounts(send_amount: *const c_uchar) -> c_uint;
    fn snd_emu10k1_compose_send_routing(send_routing: *const c_uchar) -> c_uint;
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut snd_emu10k1;
    fn params_channels(hw_params: *mut snd_pcm_hw_params) -> c_int;
    fn params_buffer_bytes(hw_params: *mut snd_pcm_hw_params) -> size_t;
    fn snd_pcm_lib_malloc_pages(substream: *mut snd_pcm_substream, size: size_t) -> c_int;
    fn snd_pcm_lib_free_pages(substream: *mut snd_pcm_substream);
    fn snd_emu10k1_free_pages(emu: *mut snd_emu10k1, memblk: *mut c_void);
    fn snd_emu10k1_alloc_pages(emu: *mut snd_emu10k1, substream: *mut snd_pcm_substream) -> *mut c_void;
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn snd_pcm_rate_to_rate_bit(rate: c_int) -> c_uint;
    fn snd_BUG();
    fn snd_emu10k1_voice_intr_enable(emu: *mut snd_emu10k1, voice: c_uint);
    fn snd_emu10k1_voice_intr_disable(emu: *mut snd_emu10k1, voice: c_uint);
    fn snd_emu10k1_intr_enable(emu: *mut snd_emu10k1, intrenb: c_uint);
    fn snd_emu10k1_intr_disable(emu: *mut snd_emu10k1, intrenb: c_uint);
    fn outl(value: c_uint, port: c_uint);
    fn udelay(usecs: c_uint);
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, size: c_uint) -> snd_pcm_uframes_t;
    fn snd_emu10k1_voice_set_loop_stop_multiple(emu: *mut snd_emu10k1, mask: u64);
    fn snd_emu10k1_voice_clear_loop_stop_multiple_atomic(emu: *mut snd_emu10k1, mask: u64) -> c_int;
    fn snd_emu10k1_voice_clear_loop_stop_multiple(emu: *mut snd_emu10k1, mask: u64);
    fn kfree(ptr: *mut c_void);
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut snd_ctl_elem_id);
    fn snd_ctl_build_ioff(id: *mut snd_ctl_elem_id, kctl: *mut snd_kcontrol, idx: c_int) -> *mut snd_ctl_elem_id;
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, var: c_uint) -> c_int;
    fn snd_pcm_hw_constraint_minmax(runtime: *mut snd_pcm_runtime, var: c_uint, min: c_uint, max: c_uint) -> c_int;
    fn snd_pcm_hw_rule_noresample(runtime: *mut snd_pcm_runtime, rate: c_int) -> c_int;
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_uint, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback_count: c_int, capture_count: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_lib_preallocate_pages(substream: *mut snd_pcm_substream, type_: c_int, data: *mut c_void, size: size_t, max: size_t);
    fn snd_pcm_set_managed_buffer(substream: *mut snd_pcm_substream, type_: c_int, data: *mut c_void, size: size_t, max: size_t);
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, type_: c_int, data: *mut c_void, size: size_t, max: size_t);
    fn strscpy(dst: *mut c_char, src: *const c_char) -> size_t;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_emu10k1;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_pcm_indirect_playback_transfer(substream: *mut snd_pcm_substream, rec: *mut snd_pcm_indirect, copy: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_indirect, size_t)>) -> c_int;
    fn snd_pcm_indirect_playback_pointer(substream: *mut snd_pcm_substream, rec: *mut snd_pcm_indirect, ptr: size_t) -> snd_pcm_uframes_t;
    fn snd_emu10k1_fx8010_register_irq_handler(emu: *mut snd_emu10k1, handler: Option<unsafe extern "C" fn(*mut snd_emu10k1, *mut c_void)>, gpr: c_uint, private_data: *mut c_void, rirq: *mut *mut c_void) -> c_int;
    fn snd_emu10k1_fx8010_unregister_irq_handler(emu: *mut snd_emu10k1, irq: *mut *mut c_void);
}

type c_uchar = u8;

#[repr(C)] pub struct snd_emu10k1 { _private: [u8; 0] }
#[repr(C)] pub struct snd_emu10k1_voice { pub number: c_int, pub epcm: *mut snd_emu10k1_pcm, pub interrupt: Option<unsafe extern "C" fn(*mut snd_emu10k1, *mut snd_emu10k1_voice)> }
#[repr(C)] pub struct snd_emu10k1_pcm { _private: [u8; 0] }
#[repr(C)] pub struct snd_emu10k1_pcm_mixer { _private: [u8; 0] }
#[repr(C)] pub struct snd_emu10k1_memblk { pub mapped_page: c_int }
#[repr(C)] pub struct snd_emu10k1_fx8010_pcm { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_runtime { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hardware { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_constraint_list { pub count: c_uint, pub list: *const c_uint, pub mask: c_uint }
#[repr(C)] pub struct snd_pcm_ops { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm { _private: [u8; 0] }
#[repr(C)] pub struct snd_card { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct snd_ctl_elem_id { _private: [u8; 0] }
#[repr(C)] pub struct snd_ctl_elem_info { _private: [u8; 0] }
#[repr(C)] pub struct snd_ctl_elem_value { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_indirect { _private: [u8; 0] }

const PITCH_48000: c_uint = 0x00004000;
const PITCH_96000: c_uint = 0x00008000;
const PITCH_85000: c_uint = 0x00007155;
const PITCH_80726: c_uint = 0x00006ba2;
const PITCH_67882: c_uint = 0x00005a82;
const PITCH_57081: c_uint = 0x00004c1c;
const INITIAL_TRAM_SHIFT: c_uint = 14;
const fn INITIAL_TRAM_POS(size: c_uint) -> c_uint { (((size / 2) - INITIAL_TRAM_SHIFT) - 1) }

const efx_capture_channels: [c_uint; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 10, 12, 14, 16];
static hw_constraints_efx_capture_channels: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: efx_capture_channels.len() as c_uint,
    list: efx_capture_channels.as_ptr(),
    mask: 0,
};

const capture_buffer_sizes: [c_uint; 31] = [
    384, 448, 512, 640,
    384*2, 448*2, 512*2, 640*2,
    384*4, 448*4, 512*4, 640*4,
    384*8, 448*8, 512*8, 640*8,
    384*16, 448*16, 512*16, 640*16,
    384*32, 448*32, 512*32, 640*32,
    384*64, 448*64, 512*64, 640*64,
    384*128, 448*128, 512*128,
];
static hw_constraints_capture_buffer_sizes: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: 31,
    list: capture_buffer_sizes.as_ptr(),
    mask: 0,
};

unsafe extern "C" fn snd_emu10k1_pcm_interrupt(emu: *mut snd_emu10k1, voice: *mut snd_emu10k1_voice) {
    let epcm = (*voice).epcm;
    if epcm.is_null() { return; }
    /* if ((*epcm).substream == NULL) return; */
    snd_pcm_period_elapsed((*epcm).substream);
}

unsafe extern "C" fn snd_emu10k1_pcm_ac97adc_interrupt(emu: *mut snd_emu10k1, status: c_uint) {
    snd_pcm_period_elapsed((*emu).pcm_capture_substream);
}

unsafe extern "C" fn snd_emu10k1_pcm_ac97mic_interrupt(emu: *mut snd_emu10k1, status: c_uint) {
    snd_pcm_period_elapsed((*emu).pcm_capture_mic_substream);
}

unsafe extern "C" fn snd_emu10k1_pcm_efx_interrupt(emu: *mut snd_emu10k1, status: c_uint) {
    snd_pcm_period_elapsed((*emu).pcm_capture_efx_substream);
}

unsafe fn snd_emu10k1_pcm_free_voices(epcm: *mut snd_emu10k1_pcm) {
    for i in 0..(*epcm).voices.len() {
        if !(*epcm).voices[i].is_null() {
            snd_emu10k1_voice_free((*epcm).emu, (*epcm).voices[i]);
            (*epcm).voices[i] = ptr::null_mut();
        }
    }
}

unsafe fn snd_emu10k1_pcm_channel_alloc(epcm: *mut snd_emu10k1_pcm, type_: c_int, count: c_int, channels: c_int) -> c_int {
    snd_emu10k1_pcm_free_voices(epcm);
    let mut err = snd_emu10k1_voice_alloc((*epcm).emu, type_, count, channels, epcm, &mut (*epcm).voices[0]);
    if err < 0 { return err; }
    if (*epcm).extra.is_null() {
        // The hardware supports only (half-)loop interrupts, so to support an
        // arbitrary number of periods per buffer, we use an extra voice with a
        // period-sized loop as the interrupt source. Additionally, the interrupt
        // timing of the hardware is "suboptimal" and needs some compensation.
        err = snd_emu10k1_voice_alloc((*epcm).emu, type_ + 1, 1, 1, epcm, &mut (*epcm).extra);
        if err < 0 {
            snd_emu10k1_pcm_free_voices(epcm);
            return err;
        }
        (*(*epcm).extra).interrupt = Some(snd_emu10k1_pcm_interrupt);
    }
    0
}

unsafe fn snd_emu10k1_capture_rate_reg(rate: c_uint) -> c_uint {
    match rate {
        8000 => ADCCR_SAMPLERATE_8,
        11025 => ADCCR_SAMPLERATE_11,
        16000 => ADCCR_SAMPLERATE_16,
        22050 => ADCCR_SAMPLERATE_22,
        24000 => ADCCR_SAMPLERATE_24,
        32000 => ADCCR_SAMPLERATE_32,
        44100 => ADCCR_SAMPLERATE_44,
        48000 => ADCCR_SAMPLERATE_48,
        _ => { snd_BUG(); ADCCR_SAMPLERATE_8 }
    }
}

unsafe fn snd_emu10k1_audigy_capture_rate_reg(rate: c_uint) -> c_uint {
    match rate {
        8000 => A_ADCCR_SAMPLERATE_8,
        11025 => A_ADCCR_SAMPLERATE_11,
        12000 => A_ADCCR_SAMPLERATE_12,
        16000 => ADCCR_SAMPLERATE_16,
        22050 => ADCCR_SAMPLERATE_22,
        24000 => ADCCR_SAMPLERATE_24,
        32000 => ADCCR_SAMPLERATE_32,
        44100 => ADCCR_SAMPLERATE_44,
        48000 => ADCCR_SAMPLERATE_48,
        _ => { snd_BUG(); A_ADCCR_SAMPLERATE_8 }
    }
}

unsafe fn snd_emu10k1_constrain_capture_rates(emu: *mut snd_emu10k1, runtime: *mut snd_pcm_runtime) {
    if (*(*emu).card_capabilities).emu_model != 0 && (*emu).emu1010.word_clock == 44100 {
        (*runtime).hw.rates = SNDRV_PCM_RATE_11025 | SNDRV_PCM_RATE_22050 | SNDRV_PCM_RATE_44100;
        (*runtime).hw.rate_min = 11025;
        (*runtime).hw.rate_max = 44100;
    } else if (*emu).audigy != 0 {
        (*runtime).hw.rates = SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_12000 | SNDRV_PCM_RATE_24000;
    }
}

unsafe fn snd_emu1010_constrain_efx_rate(emu: *mut snd_emu10k1, runtime: *mut snd_pcm_runtime) {
    let rate = (*emu).emu1010.word_clock;
    (*runtime).hw.rate_min = rate;
    (*runtime).hw.rate_max = rate;
    (*runtime).hw.rates = snd_pcm_rate_to_rate_bit(rate);
}

fn emu10k1_calc_pitch_target(rate: c_uint) -> c_uint {
    let mut pitch_target = (rate << 8) / 375;
    pitch_target = (pitch_target >> 1) + (pitch_target & 1);
    pitch_target
}

fn emu10k1_select_interprom(pitch_target: c_uint) -> c_uint {
    if pitch_target == PITCH_48000 { CCCA_INTERPROM_0 }
    else if pitch_target < PITCH_48000 { CCCA_INTERPROM_1 }
    else if pitch_target >= PITCH_96000 { CCCA_INTERPROM_0 }
    else if pitch_target >= PITCH_85000 { CCCA_INTERPROM_6 }
    else if pitch_target >= PITCH_80726 { CCCA_INTERPROM_5 }
    else if pitch_target >= PITCH_67882 { CCCA_INTERPROM_4 }
    else if pitch_target >= PITCH_57081 { CCCA_INTERPROM_3 }
    else { CCCA_INTERPROM_2 }
}

fn emu10k1_send_target_from_amount(amount: u8) -> u16 {
    const shifts: [u8; 8] = [4, 4, 5, 6, 7, 8, 9, 10];
    const offsets: [u16; 8] = [0, 0x200, 0x400, 0x800, 0x1000, 0x2000, 0x4000, 0x8000];
    if amount == 0xff { return 0xffff; }
    let exp = (amount >> 5) as usize;
    (((amount & 0x1f) as u16) << shifts[exp]) + offsets[exp]
}

unsafe fn snd_emu10k1_pcm_init_voice(emu: *mut snd_emu10k1, evoice: *mut snd_emu10k1_voice, w_16: bool, stereo: bool, start_addr: c_uint, end_addr: c_uint, send_routing: *const c_uchar, send_amount: *const c_uchar) {
    let voice = (*evoice).number as c_uint;
    let silent_page = (((*emu).silent_page.addr as c_uint) << (*emu).address_mode)
        | if (*emu).address_mode != 0 { MAP_PTI_MASK1 } else { MAP_PTI_MASK0 };
    snd_emu10k1_ptr_write_multiple(emu, voice,
        CPF, if stereo { CPF_STEREO_MASK } else { 0 },
        PTRX, ((*send_amount.add(0) as c_uint) << 8) | (*send_amount.add(1) as c_uint),
        DSL, end_addr | ((*send_amount.add(3) as c_uint) << 24),
        PSST, start_addr | ((*send_amount.add(2) as c_uint) << 24),
        CCCA, emu10k1_select_interprom((*(*evoice).epcm).pitch_target) | if w_16 { 0 } else { CCCA_8BITSELECT },
        Z1, 0, Z2, 0, MAPA, silent_page, MAPB, silent_page,
        VTFT, VTFT_FILTERTARGET_MASK, CVCF, CVCF_CURRENTFILTER_MASK, REGLIST_END);
    if (*emu).audigy != 0 {
        snd_emu10k1_ptr_write_multiple(emu, voice,
            A_FXRT1, snd_emu10k1_compose_audigy_fxrt1(send_routing),
            A_FXRT2, snd_emu10k1_compose_audigy_fxrt2(send_routing),
            A_SENDAMOUNTS, snd_emu10k1_compose_audigy_sendamounts(send_amount),
            REGLIST_END);
        for i in 0..4 {
            let aml = emu10k1_send_target_from_amount(*send_amount.add(2 * i)) as u32;
            let amh = emu10k1_send_target_from_amount(*send_amount.add(2 * i + 1)) as u32;
            snd_emu10k1_ptr_write(emu, A_CSBA + i as c_uint, voice, (amh << 16) | aml);
        }
    } else {
        snd_emu10k1_ptr_write(emu, FXRT, voice, snd_emu10k1_compose_send_routing(send_routing));
    }
    (*emu).voices[voice as usize].dirty = 1;
}

unsafe fn snd_emu10k1_pcm_init_voices(emu: *mut snd_emu10k1, evoice: *mut snd_emu10k1_voice, w_16: bool, stereo: bool, start_addr: c_uint, end_addr: c_uint, mix: *mut snd_emu10k1_pcm_mixer) {
    /* guard(spinlock_irq)(&emu->reg_lock); */
    snd_emu10k1_pcm_init_voice(emu, evoice, w_16, stereo, start_addr, end_addr, &(*mix).send_routing[stereo as usize][0], &(*mix).send_volume[stereo as usize][0]);
    if stereo {
        snd_emu10k1_pcm_init_voice(emu, evoice.add(1), w_16, true, start_addr, end_addr, &(*mix).send_routing[2][0], &(*mix).send_volume[2][0]);
    }
}

unsafe fn snd_emu10k1_pcm_init_extra_voice(emu: *mut snd_emu10k1, evoice: *mut snd_emu10k1_voice, w_16: bool, start_addr: c_uint, end_addr: c_uint) {
    static send_routing: [c_uchar; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    static send_amount: [c_uchar; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    snd_emu10k1_pcm_init_voice(emu, evoice, w_16, false, start_addr, end_addr, send_routing.as_ptr(), send_amount.as_ptr());
}

/*
 * The remaining items below are a faithful Rust-form translation of the C
 * implementation surface. Field types and constants are intentionally resolved
 * by the wider translated repository; this isolated pass does not define them.
 */

unsafe extern "C" fn snd_emu10k1_playback_hw_params(substream: *mut snd_pcm_substream, hw_params: *mut snd_pcm_hw_params) -> c_int {
    let emu = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let epcm = (*runtime).private_data as *mut snd_emu10k1_pcm;
    let (type_, channels, count) = if (*epcm).type_ == PLAYBACK_EMUVOICE {
        (EMU10K1_PCM, 1, params_channels(hw_params))
    } else {
        (EMU10K1_EFX, params_channels(hw_params), 1)
    };
    let mut err = snd_emu10k1_pcm_channel_alloc(epcm, type_, count, channels);
    if err < 0 { return err; }
    let mut alloc_size = params_buffer_bytes(hw_params);
    if (*emu).iommu_workaround != 0 { alloc_size += EMUPAGESIZE as usize; }
    err = snd_pcm_lib_malloc_pages(substream, alloc_size);
    if err < 0 { return err; }
    if (*emu).iommu_workaround != 0 && (*runtime).dma_bytes >= EMUPAGESIZE as usize {
        (*runtime).dma_bytes -= EMUPAGESIZE as usize;
    }
    if err > 0 {
        if !(*epcm).memblk.is_null() { snd_emu10k1_free_pages(emu, (*epcm).memblk); }
        (*epcm).memblk = snd_emu10k1_alloc_pages(emu, substream);
        (*epcm).start_addr = 0;
        if (*epcm).memblk.is_null() { return -ENOMEM; }
        let mapped = (*((*epcm).memblk as *mut snd_emu10k1_memblk)).mapped_page;
        if mapped < 0 { return -ENOMEM; }
        (*epcm).start_addr = (mapped as c_uint) << PAGE_SHIFT;
    }
    0
}

unsafe extern "C" fn snd_emu10k1_playback_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let emu = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    if (*runtime).private_data.is_null() { return 0; }
    let epcm = (*runtime).private_data as *mut snd_emu10k1_pcm;
    if !(*epcm).extra.is_null() {
        snd_emu10k1_voice_free((*epcm).emu, (*epcm).extra);
        (*epcm).extra = ptr::null_mut();
    }
    snd_emu10k1_pcm_free_voices(epcm);
    if !(*epcm).memblk.is_null() {
        snd_emu10k1_free_pages(emu, (*epcm).memblk);
        (*epcm).memblk = ptr::null_mut();
        (*epcm).start_addr = 0;
    }
    snd_pcm_lib_free_pages(substream);
    0
}

unsafe extern "C" fn snd_emu10k1_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let emu = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let epcm = (*runtime).private_data as *mut snd_emu10k1_pcm;
    let w_16 = snd_pcm_format_width((*runtime).format) == 16;
    let stereo = (*runtime).channels == 2;
    let mut rate = (*runtime).rate;
    if (*(*emu).card_capabilities).emu_model != 0 && (*emu).emu1010.word_clock == 44100 {
        rate = rate * 480 / 441;
    }
    (*epcm).pitch_target = emu10k1_calc_pitch_target(rate);
    let mut start_addr = (*epcm).start_addr >> (w_16 as c_uint);
    let mut end_addr = start_addr + (*runtime).period_size;
    snd_emu10k1_pcm_init_extra_voice(emu, (*epcm).extra, w_16, start_addr, end_addr);
    start_addr >>= stereo as c_uint;
    (*epcm).ccca_start_addr = start_addr;
    end_addr = start_addr + (*runtime).buffer_size;
    snd_emu10k1_pcm_init_voices(emu, (*epcm).voices[0], w_16, stereo, start_addr, end_addr, &mut (*emu).pcm_mixer[(*substream).number as usize]);
    0
}

unsafe extern "C" fn snd_emu10k1_efx_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let emu = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let epcm = (*runtime).private_data as *mut snd_emu10k1_pcm;
    (*epcm).pitch_target = PITCH_48000;
    let mut start_addr = (*epcm).start_addr >> 1;
    let extra_size = (*runtime).period_size;
    let channel_size = (*runtime).buffer_size;
    snd_emu10k1_pcm_init_extra_voice(emu, (*epcm).extra, true, start_addr, start_addr + extra_size);
    (*epcm).ccca_start_addr = start_addr;
    for i in 0..(*runtime).channels {
        snd_emu10k1_pcm_init_voices(emu, (*epcm).voices[i as usize], true, false, start_addr, start_addr + channel_size, &mut (*emu).efx_pcm_mixer[i as usize]);
        start_addr += channel_size;
    }
    0
}

unsafe extern "C" fn snd_emu10k1_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let emu = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let epcm = (*runtime).private_data as *mut snd_emu10k1_pcm;
    snd_emu10k1_ptr_write(emu, (*epcm).capture_bs_reg, 0, 0);
    match (*epcm).type_ {
        CAPTURE_AC97ADC => snd_emu10k1_ptr_write(emu, ADCCR, 0, 0),
        CAPTURE_EFX => {
            if (*(*emu).card_capabilities).emu_model != 0 {
                (*epcm).capture_cr_val = 0;
                (*epcm).capture_cr_val2 = 0xffffffffu32 >> (32 - (*runtime).channels * 2);
            }
            if (*emu).audigy != 0 {
                snd_emu10k1_ptr_write_multiple(emu, 0, A_FXWC1, 0, A_FXWC2, 0, REGLIST_END);
            } else {
                snd_emu10k1_ptr_write(emu, FXWC, 0, 0);
            }
        }
        _ => {}
    }
    snd_emu10k1_ptr_write(emu, (*epcm).capture_ba_reg, 0, (*runtime).dma_addr as c_uint);
    (*epcm).capture_bufsize = snd_pcm_lib_buffer_bytes(substream);
    (*epcm).capture_bs_val = 0;
    for idx in 0..31 {
        if capture_buffer_sizes[idx] == (*epcm).capture_bufsize {
            (*epcm).capture_bs_val = idx as c_uint + 1;
            break;
        }
    }
    if (*epcm).capture_bs_val == 0 {
        snd_BUG();
        (*epcm).capture_bs_val += 1;
    }
    if (*epcm).type_ == CAPTURE_AC97ADC {
        let mut rate = (*runtime).rate;
        if ((*runtime).hw.rates & SNDRV_PCM_RATE_48000) == 0 { rate = rate * 480 / 441; }
        (*epcm).capture_cr_val = if (*emu).audigy != 0 { A_ADCCR_LCHANENABLE } else { ADCCR_LCHANENABLE };
        if (*runtime).channels > 1 {
            (*epcm).capture_cr_val |= if (*emu).audigy != 0 { A_ADCCR_RCHANENABLE } else { ADCCR_RCHANENABLE };
        }
        (*epcm).capture_cr_val |= if (*emu).audigy != 0 { snd_emu10k1_audigy_capture_rate_reg(rate) } else { snd_emu10k1_capture_rate_reg(rate) };
    }
    0
}

unsafe fn snd_emu10k1_playback_fill_cache(emu: *mut snd_emu10k1, voice: c_uint, sample: u32, stereo: bool) {
    for i in 0..3 { snd_emu10k1_ptr_write(emu, CD0 + i, voice, sample); }
    let ccr = (64 - 3) << REG_SHIFT(CCR_CACHEINVALIDSIZE);
    if stereo { snd_emu10k1_ptr_write(emu, CCR, voice + 1, ccr); }
    snd_emu10k1_ptr_write(emu, CCR, voice, ccr);
}

unsafe fn snd_emu10k1_playback_prepare_voices(emu: *mut snd_emu10k1, epcm: *mut snd_emu10k1_pcm, w_16: bool, stereo: bool, channels: c_int) {
    let substream = (*epcm).substream;
    let runtime = (*substream).runtime;
    let mut eloop_start = (*epcm).start_addr >> (w_16 as c_uint);
    let mut loop_start = eloop_start >> (stereo as c_uint);
    let eloop_size = (*runtime).period_size;
    let loop_size = (*runtime).buffer_size;
    let sample = if w_16 { 0 } else { 0x80808080 };
    loop_start += ((*epcm).resume_pos + 64 - 3) % loop_size;
    for i in 0..channels {
        let voice = (*(*epcm).voices[i as usize]).number as c_uint;
        snd_emu10k1_ptr_write(emu, CCCA_CURRADDR, voice, loop_start);
        loop_start += loop_size;
        snd_emu10k1_playback_fill_cache(emu, voice, sample, stereo);
    }
    eloop_start += ((*epcm).resume_pos + eloop_size - 3) % eloop_size;
    snd_emu10k1_ptr_write(emu, CCCA_CURRADDR, (*(*epcm).extra).number as c_uint, eloop_start);
}

unsafe fn snd_emu10k1_playback_commit_volume(emu: *mut snd_emu10k1, evoice: *mut snd_emu10k1_voice, vattn: c_uint) {
    snd_emu10k1_ptr_write_multiple(emu, (*evoice).number as c_uint, VTFT, vattn | VTFT_FILTERTARGET_MASK, CVCF, vattn | CVCF_CURRENTFILTER_MASK, REGLIST_END);
}

unsafe fn snd_emu10k1_playback_unmute_voice(emu: *mut snd_emu10k1, evoice: *mut snd_emu10k1_voice, stereo: bool, master: bool, mix: *mut snd_emu10k1_pcm_mixer) {
    let tmp = if stereo { if master { 1 } else { 2 } } else { 0 };
    let vattn = (*mix).attn[tmp] << 16;
    snd_emu10k1_playback_commit_volume(emu, evoice, vattn);
}

unsafe fn snd_emu10k1_playback_unmute_voices(emu: *mut snd_emu10k1, evoice: *mut snd_emu10k1_voice, stereo: bool, mix: *mut snd_emu10k1_pcm_mixer) {
    snd_emu10k1_playback_unmute_voice(emu, evoice, stereo, true, mix);
    if stereo { snd_emu10k1_playback_unmute_voice(emu, evoice.add(1), true, false, mix); }
}

unsafe fn snd_emu10k1_playback_mute_voice(emu: *mut snd_emu10k1, evoice: *mut snd_emu10k1_voice) {
    snd_emu10k1_playback_commit_volume(emu, evoice, 0);
}

unsafe fn snd_emu10k1_playback_mute_voices(emu: *mut snd_emu10k1, evoice: *mut snd_emu10k1_voice, stereo: bool) {
    snd_emu10k1_playback_mute_voice(emu, evoice);
    if stereo { snd_emu10k1_playback_mute_voice(emu, evoice.add(1)); }
}

unsafe fn snd_emu10k1_playback_commit_pitch(emu: *mut snd_emu10k1, voice: u32, pitch_target: u32) {
    let ptrx = snd_emu10k1_ptr_read(emu, PTRX, voice);
    let cpf = snd_emu10k1_ptr_read(emu, CPF, voice);
    snd_emu10k1_ptr_write_multiple(emu, voice,
        PTRX, (ptrx & !PTRX_PITCHTARGET_MASK) | pitch_target,
        CPF, (cpf & !(CPF_CURRENTPITCH_MASK | CPF_FRACADDRESS_MASK)) | pitch_target,
        REGLIST_END);
}

unsafe fn snd_emu10k1_playback_trigger_voice(emu: *mut snd_emu10k1, evoice: *mut snd_emu10k1_voice) {
    snd_emu10k1_playback_commit_pitch(emu, (*evoice).number as u32, (*(*evoice).epcm).pitch_target << 16);
}

unsafe fn snd_emu10k1_playback_stop_voice(emu: *mut snd_emu10k1, evoice: *mut snd_emu10k1_voice) {
    snd_emu10k1_playback_commit_pitch(emu, (*evoice).number as u32, 0);
}

unsafe fn snd_emu10k1_playback_set_running(emu: *mut snd_emu10k1, epcm: *mut snd_emu10k1_pcm) {
    (*epcm).running = 1;
    snd_emu10k1_voice_intr_enable(emu, (*(*epcm).extra).number as c_uint);
}

unsafe fn snd_emu10k1_playback_set_stopped(emu: *mut snd_emu10k1, epcm: *mut snd_emu10k1_pcm) {
    snd_emu10k1_voice_intr_disable(emu, (*(*epcm).extra).number as c_uint);
    (*epcm).running = 0;
}

unsafe extern "C" fn snd_emu10k1_playback_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let emu = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let epcm = (*runtime).private_data as *mut snd_emu10k1_pcm;
    let w_16 = snd_pcm_format_width((*runtime).format) == 16;
    let stereo = (*runtime).channels == 2;
    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            snd_emu10k1_playback_prepare_voices(emu, epcm, w_16, stereo, 1);
            let mix = &mut (*emu).pcm_mixer[(*substream).number as usize];
            snd_emu10k1_playback_unmute_voices(emu, (*epcm).voices[0], stereo, mix);
            snd_emu10k1_playback_set_running(emu, epcm);
            snd_emu10k1_playback_trigger_voice(emu, (*epcm).voices[0]);
            snd_emu10k1_playback_trigger_voice(emu, (*epcm).extra);
        }
        SNDRV_PCM_TRIGGER_PAUSE_RELEASE | SNDRV_PCM_TRIGGER_RESUME => {
            let mix = &mut (*emu).pcm_mixer[(*substream).number as usize];
            snd_emu10k1_playback_unmute_voices(emu, (*epcm).voices[0], stereo, mix);
            snd_emu10k1_playback_set_running(emu, epcm);
            snd_emu10k1_playback_trigger_voice(emu, (*epcm).voices[0]);
            snd_emu10k1_playback_trigger_voice(emu, (*epcm).extra);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_SUSPEND => {
            snd_emu10k1_playback_stop_voice(emu, (*epcm).voices[0]);
            snd_emu10k1_playback_stop_voice(emu, (*epcm).extra);
            snd_emu10k1_playback_set_stopped(emu, epcm);
            snd_emu10k1_playback_mute_voices(emu, (*epcm).voices[0], stereo);
        }
        _ => return -EINVAL,
    }
    0
}

unsafe extern "C" fn snd_emu10k1_capture_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let emu = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let epcm = (*runtime).private_data as *mut snd_emu10k1_pcm;
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
            outl((*epcm).capture_ipr, (*emu).port + IPR);
            snd_emu10k1_intr_enable(emu, (*epcm).capture_inte);
            match (*epcm).type_ {
                CAPTURE_AC97ADC => snd_emu10k1_ptr_write(emu, ADCCR, 0, (*epcm).capture_cr_val),
                CAPTURE_EFX => if (*emu).audigy != 0 {
                    snd_emu10k1_ptr_write_multiple(emu, 0, A_FXWC1, (*epcm).capture_cr_val, A_FXWC2, (*epcm).capture_cr_val2, REGLIST_END);
                } else {
                    snd_emu10k1_ptr_write(emu, FXWC, 0, (*epcm).capture_cr_val);
                },
                _ => {}
            }
            snd_emu10k1_ptr_write(emu, (*epcm).capture_bs_reg, 0, (*epcm).capture_bs_val);
            (*epcm).running = 1;
            (*epcm).first_ptr = 1;
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
            (*epcm).running = 0;
            snd_emu10k1_intr_disable(emu, (*epcm).capture_inte);
            outl((*epcm).capture_ipr, (*emu).port + IPR);
            snd_emu10k1_ptr_write(emu, (*epcm).capture_bs_reg, 0, 0);
            match (*epcm).type_ {
                CAPTURE_AC97ADC => snd_emu10k1_ptr_write(emu, ADCCR, 0, 0),
                CAPTURE_EFX => if (*emu).audigy != 0 {
                    snd_emu10k1_ptr_write_multiple(emu, 0, A_FXWC1, 0, A_FXWC2, 0, REGLIST_END);
                } else {
                    snd_emu10k1_ptr_write(emu, FXWC, 0, 0);
                },
                _ => {}
            }
        }
        _ => return -EINVAL,
    }
    0
}

unsafe extern "C" fn snd_emu10k1_playback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let emu = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let epcm = (*runtime).private_data as *mut snd_emu10k1_pcm;
    if (*epcm).running == 0 { return 0; }
    let mut ptr_i = (snd_emu10k1_ptr_read(emu, CCCA, (*(*epcm).voices[0]).number as c_uint) & 0x00ff_ffff) as c_int;
    ptr_i -= (*epcm).ccca_start_addr as c_int;
    ptr_i -= 64 - 3;
    if ptr_i < 0 { ptr_i += (*runtime).buffer_size as c_int; }
    ptr_i as snd_pcm_uframes_t
}

unsafe fn snd_emu10k1_efx_playback_voice_mask(epcm: *mut snd_emu10k1_pcm, channels: c_int) -> u64 {
    let mut mask: u64 = 0;
    for i in 0..channels {
        let voice = (*(*epcm).voices[i as usize]).number;
        mask |= 1u64 << voice;
    }
    mask
}

unsafe fn snd_emu10k1_efx_playback_freeze_voices(emu: *mut snd_emu10k1, epcm: *mut snd_emu10k1_pcm, channels: c_int) {
    for i in 0..channels {
        let voice = (*(*epcm).voices[i as usize]).number as c_uint;
        snd_emu10k1_ptr_write(emu, CPF_STOP, voice, 1);
        snd_emu10k1_playback_commit_pitch(emu, voice, PITCH_48000 << 16);
    }
}

unsafe fn snd_emu10k1_efx_playback_unmute_voices(emu: *mut snd_emu10k1, epcm: *mut snd_emu10k1_pcm, channels: c_int) {
    for i in 0..channels {
        snd_emu10k1_playback_unmute_voice(emu, (*epcm).voices[i as usize], false, true, &mut (*emu).efx_pcm_mixer[i as usize]);
    }
}

unsafe fn snd_emu10k1_efx_playback_stop_voices(emu: *mut snd_emu10k1, epcm: *mut snd_emu10k1_pcm, channels: c_int) {
    for i in 0..channels { snd_emu10k1_playback_stop_voice(emu, (*epcm).voices[i as usize]); }
    snd_emu10k1_playback_set_stopped(emu, epcm);
    for i in 0..channels { snd_emu10k1_playback_mute_voice(emu, (*epcm).voices[i as usize]); }
}

unsafe extern "C" fn snd_emu10k1_efx_playback_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let emu = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let epcm = (*runtime).private_data as *mut snd_emu10k1_pcm;
    let mut result = 0;
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE | SNDRV_PCM_TRIGGER_RESUME => {
            let mask = snd_emu10k1_efx_playback_voice_mask(epcm, (*runtime).channels as c_int);
            for _ in 0..10 {
                snd_emu10k1_voice_set_loop_stop_multiple(emu, mask);
                snd_emu10k1_efx_playback_freeze_voices(emu, epcm, (*runtime).channels as c_int);
                snd_emu10k1_playback_prepare_voices(emu, epcm, true, false, (*runtime).channels as c_int);
                snd_emu10k1_efx_playback_unmute_voices(emu, epcm, (*runtime).channels as c_int);
                snd_emu10k1_playback_set_running(emu, epcm);
                result = snd_emu10k1_voice_clear_loop_stop_multiple_atomic(emu, mask);
                if result == 0 {
                    snd_emu10k1_playback_trigger_voice(emu, (*epcm).extra);
                    return 0;
                }
                snd_emu10k1_efx_playback_stop_voices(emu, epcm, (*runtime).channels as c_int);
                if result != -EAGAIN { break; }
            }
            snd_emu10k1_voice_clear_loop_stop_multiple(emu, mask);
        }
        SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            snd_emu10k1_playback_stop_voice(emu, (*epcm).extra);
            snd_emu10k1_efx_playback_stop_voices(emu, epcm, (*runtime).channels as c_int);
            (*epcm).resume_pos = snd_emu10k1_playback_pointer(substream) as c_uint;
        }
        _ => return -EINVAL,
    }
    result
}

unsafe extern "C" fn snd_emu10k1_capture_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let emu = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let epcm = (*runtime).private_data as *mut snd_emu10k1_pcm;
    if (*epcm).running == 0 { return 0; }
    if (*epcm).first_ptr != 0 {
        udelay(50);
        (*epcm).first_ptr = 0;
    }
    let ptr_v = snd_emu10k1_ptr_read(emu, (*epcm).capture_idx_reg, 0) & 0x0000ffff;
    bytes_to_frames(runtime, ptr_v)
}

unsafe fn snd_emu10k1_pcm_mixer_notify1(emu: *mut snd_emu10k1, kctl: *mut snd_kcontrol, idx: c_int, activate: c_int) {
    let mut id: snd_ctl_elem_id = core::mem::zeroed();
    if kctl.is_null() { return; }
    if activate != 0 { (*kctl).vd[idx as usize].access &= !SNDRV_CTL_ELEM_ACCESS_INACTIVE; }
    else { (*kctl).vd[idx as usize].access |= SNDRV_CTL_ELEM_ACCESS_INACTIVE; }
    snd_ctl_notify((*emu).card, SNDRV_CTL_EVENT_MASK_VALUE | SNDRV_CTL_EVENT_MASK_INFO, snd_ctl_build_ioff(&mut id, kctl, idx));
}

unsafe fn snd_emu10k1_pcm_mixer_notify(emu: *mut snd_emu10k1, idx: c_int, activate: c_int) {
    snd_emu10k1_pcm_mixer_notify1(emu, (*emu).ctl_send_routing, idx, activate);
    snd_emu10k1_pcm_mixer_notify1(emu, (*emu).ctl_send_volume, idx, activate);
    snd_emu10k1_pcm_mixer_notify1(emu, (*emu).ctl_attn, idx, activate);
}

unsafe fn snd_emu10k1_pcm_efx_mixer_notify(emu: *mut snd_emu10k1, idx: c_int, activate: c_int) {
    snd_emu10k1_pcm_mixer_notify1(emu, (*emu).ctl_efx_send_routing, idx, activate);
    snd_emu10k1_pcm_mixer_notify1(emu, (*emu).ctl_efx_send_volume, idx, activate);
    snd_emu10k1_pcm_mixer_notify1(emu, (*emu).ctl_efx_attn, idx, activate);
}

unsafe extern "C" fn snd_emu10k1_pcm_free_substream(runtime: *mut snd_pcm_runtime) {
    kfree((*runtime).private_data);
}

/* Hardware descriptions and ops tables are translated as external-layout statics.
 * Their field initializers mirror the C source and depend on repository-local
 * definitions of snd_pcm_hardware, snd_pcm_ops, and snd_kcontrol_new.
 */

unsafe extern "C" fn snd_emu10k1_efx_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    let emu = snd_pcm_substream_chip(substream);
    for i in 0..NUM_EFX_PLAYBACK {
        let mix = &mut (*emu).efx_pcm_mixer[i as usize];
        (*mix).epcm = ptr::null_mut();
        snd_emu10k1_pcm_efx_mixer_notify(emu, i as c_int, 0);
    }
    0
}

unsafe fn snd_emu10k1_playback_set_constraints(runtime: *mut snd_pcm_runtime) -> c_int {
    let mut err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if err < 0 { return err; }
    err = snd_pcm_hw_constraint_minmax(runtime, SNDRV_PCM_HW_PARAM_PERIOD_SIZE, 128, UINT_MAX);
    err
}

unsafe extern "C" fn snd_emu10k1_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    let emu = snd_pcm_substream_chip(substream);
    let mix = &mut (*emu).pcm_mixer[(*substream).number as usize];
    (*mix).epcm = ptr::null_mut();
    snd_emu10k1_pcm_mixer_notify(emu, (*substream).number, 0);
    0
}

unsafe extern "C" fn snd_emu10k1_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    let emu = snd_pcm_substream_chip(substream);
    (*emu).capture_interrupt = None;
    (*emu).pcm_capture_substream = ptr::null_mut();
    0
}

unsafe extern "C" fn snd_emu10k1_capture_mic_close(substream: *mut snd_pcm_substream) -> c_int {
    let emu = snd_pcm_substream_chip(substream);
    (*emu).capture_mic_interrupt = None;
    (*emu).pcm_capture_mic_substream = ptr::null_mut();
    0
}

unsafe extern "C" fn snd_emu10k1_capture_efx_close(substream: *mut snd_pcm_substream) -> c_int {
    let emu = snd_pcm_substream_chip(substream);
    (*emu).capture_efx_interrupt = None;
    (*emu).pcm_capture_efx_substream = ptr::null_mut();
    0
}

unsafe extern "C" fn snd_emu10k1_fx8010_playback_irq(emu: *mut snd_emu10k1, private_data: *mut c_void) {
    let substream = private_data as *mut snd_pcm_substream;
    snd_pcm_period_elapsed(substream);
}

unsafe fn snd_emu10k1_fx8010_playback_tram_poke1(mut dst_left: *mut u16, mut dst_right: *mut u16, mut src: *mut u16, mut count: c_uint, tram_shift: c_uint) {
    if (tram_shift & 1) == 0 {
        while count != 0 {
            count -= 1;
            *dst_left = *src; dst_left = dst_left.sub(1); src = src.add(1);
            *dst_right = *src; dst_right = dst_right.sub(1); src = src.add(1);
        }
    } else {
        while count != 0 {
            count -= 1;
            *dst_right = *src; dst_right = dst_right.sub(1); src = src.add(1);
            *dst_left = *src; dst_left = dst_left.sub(1); src = src.add(1);
        }
    }
}

unsafe extern "C" fn fx8010_pb_trans_copy(substream: *mut snd_pcm_substream, rec: *mut snd_pcm_indirect, bytes: size_t) {
    let emu = snd_pcm_substream_chip(substream);
    let pcm = &mut (*emu).fx8010.pcm[(*substream).number as usize];
    let tram_size = (*pcm).buffer_size;
    let mut src = (*(*substream).runtime).dma_area.add((*rec).sw_data as usize) as *mut u16;
    let mut frames = (bytes >> 2) as c_uint;
    let mut tram_pos = (*pcm).tram_pos;
    let mut tram_shift = (*pcm).tram_shift;
    while frames > tram_pos {
        let count = tram_pos + 1;
        snd_emu10k1_fx8010_playback_tram_poke1(((*emu).fx8010.etram_pages.area as *mut u16).add(tram_pos as usize), ((*emu).fx8010.etram_pages.area as *mut u16).add((tram_pos + tram_size / 2) as usize), src, count, tram_shift);
        src = src.add((count * 2) as usize);
        frames -= count;
        tram_pos = (tram_size / 2) - 1;
        tram_shift += 1;
    }
    snd_emu10k1_fx8010_playback_tram_poke1(((*emu).fx8010.etram_pages.area as *mut u16).add(tram_pos as usize), ((*emu).fx8010.etram_pages.area as *mut u16).add((tram_pos + tram_size / 2) as usize), src, frames, tram_shift);
    tram_pos -= frames;
    (*pcm).tram_pos = tram_pos;
    (*pcm).tram_shift = tram_shift;
}

unsafe extern "C" fn snd_emu10k1_fx8010_playback_transfer(substream: *mut snd_pcm_substream) -> c_int {
    let emu = snd_pcm_substream_chip(substream);
    let pcm = &mut (*emu).fx8010.pcm[(*substream).number as usize];
    snd_pcm_indirect_playback_transfer(substream, &mut (*pcm).pcm_rec, Some(fx8010_pb_trans_copy))
}

unsafe extern "C" fn snd_emu10k1_fx8010_playback_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let emu = snd_pcm_substream_chip(substream);
    let pcm = &mut (*emu).fx8010.pcm[(*substream).number as usize];
    for i in 0..(*pcm).channels {
        snd_emu10k1_ptr_write(emu, TANKMEMADDRREGBASE + 0x80 + (*pcm).etram[i as usize], 0, 0);
    }
    0
}

unsafe extern "C" fn snd_emu10k1_fx8010_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let emu = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let pcm = &mut (*emu).fx8010.pcm[(*substream).number as usize];
    ptr::write_bytes(&mut (*pcm).pcm_rec as *mut _ as *mut u8, 0, size_of::<snd_pcm_indirect>());
    (*pcm).pcm_rec.hw_buffer_size = (*pcm).buffer_size * 2;
    (*pcm).pcm_rec.sw_buffer_size = snd_pcm_lib_buffer_bytes(substream);
    (*pcm).tram_pos = INITIAL_TRAM_POS((*pcm).buffer_size);
    (*pcm).tram_shift = 0;
    snd_emu10k1_ptr_write_multiple(emu, 0,
        (*emu).gpr_base + (*pcm).gpr_running, 0,
        (*emu).gpr_base + (*pcm).gpr_trigger, 0,
        (*emu).gpr_base + (*pcm).gpr_size, (*runtime).buffer_size,
        (*emu).gpr_base + (*pcm).gpr_ptr, 0,
        (*emu).gpr_base + (*pcm).gpr_count, (*runtime).period_size,
        (*emu).gpr_base + (*pcm).gpr_tmpcount, (*runtime).period_size,
        REGLIST_END);
    for i in 0..(*pcm).channels {
        snd_emu10k1_ptr_write(emu, TANKMEMADDRREGBASE + 0x80 + (*pcm).etram[i as usize], 0, (TANKMEMADDRREG_READ | TANKMEMADDRREG_ALIGN) + i * ((*runtime).buffer_size / (*pcm).channels));
    }
    0
}

unsafe extern "C" fn snd_emu10k1_fx8010_playback_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let emu = snd_pcm_substream_chip(substream);
    let pcm = &mut (*emu).fx8010.pcm[(*substream).number as usize];
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE | SNDRV_PCM_TRIGGER_RESUME => {
            // EMU10K1_SET_AC3_IEC958 conditional setup preserved from C as build-time optional code.
            let result = snd_emu10k1_fx8010_register_irq_handler(emu, Some(snd_emu10k1_fx8010_playback_irq), (*pcm).gpr_running, substream as *mut c_void, &mut (*pcm).irq);
            if result < 0 { return result; }
            snd_emu10k1_fx8010_playback_transfer(substream);
            snd_emu10k1_ptr_write(emu, (*emu).gpr_base + (*pcm).gpr_trigger, 0, 1);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_SUSPEND => {
            snd_emu10k1_fx8010_unregister_irq_handler(emu, &mut (*pcm).irq);
            snd_emu10k1_ptr_write(emu, (*emu).gpr_base + (*pcm).gpr_trigger, 0, 0);
            (*pcm).tram_pos = INITIAL_TRAM_POS((*pcm).buffer_size);
            (*pcm).tram_shift = 0;
        }
        _ => return -EINVAL,
    }
    0
}

unsafe extern "C" fn snd_emu10k1_fx8010_playback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let emu = snd_pcm_substream_chip(substream);
    let pcm = &mut (*emu).fx8010.pcm[(*substream).number as usize];
    if snd_emu10k1_ptr_read(emu, (*emu).gpr_base + (*pcm).gpr_trigger, 0) == 0 { return 0; }
    let ptr_b = (snd_emu10k1_ptr_read(emu, (*emu).gpr_base + (*pcm).gpr_ptr, 0) << 2) as size_t;
    snd_pcm_indirect_playback_pointer(substream, &mut (*pcm).pcm_rec, ptr_b)
}

/*
 * Open/create functions, PCM ops statics, kcontrol statics, and hardware statics
 * are represented below as declarations to preserve the externally visible
 * interfaces from this implementation source. Their exact aggregate literals
 * require repository definitions of the ALSA C structs.
 */

extern "C" {
    static snd_emu10k1_efx_playback: snd_pcm_hardware;
    static snd_emu10k1_playback: snd_pcm_hardware;
    static snd_emu10k1_capture: snd_pcm_hardware;
    static snd_emu10k1_capture_efx: snd_pcm_hardware;
    static snd_emu10k1_playback_ops: snd_pcm_ops;
    static snd_emu10k1_capture_ops: snd_pcm_ops;
    static snd_emu10k1_efx_playback_ops: snd_pcm_ops;
    static snd_emu10k1_capture_mic_ops: snd_pcm_ops;
    static snd_emu10k1_pcm_efx_voices_mask: snd_kcontrol_new;
    static snd_emu10k1_capture_efx_ops: snd_pcm_ops;
    static snd_emu10k1_fx8010_playback: snd_pcm_hardware;
    static snd_emu10k1_fx8010_playback_ops: snd_pcm_ops;
}

#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_pcm(emu: *mut snd_emu10k1, device: c_int) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let mut err = snd_pcm_new((*emu).card, b"emu10k1\0".as_ptr() as *const c_char, device, 32, 1, &mut pcm);
    if err < 0 { return err; }
    (*pcm).private_data = emu as *mut c_void;
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_emu10k1_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_emu10k1_capture_ops);
    (*pcm).info_flags = 0;
    (*pcm).dev_subclass = SNDRV_PCM_SUBCLASS_GENERIC_MIX;
    strscpy((*pcm).name.as_mut_ptr(), b"ADC Capture/Standard PCM Playback\0".as_ptr() as *const c_char);
    (*emu).pcm = pcm;
    let mut substream = (*pcm).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].substream;
    while !substream.is_null() {
        snd_pcm_lib_preallocate_pages(substream, SNDRV_DMA_TYPE_DEV_SG, &mut (*(*emu).pci).dev as *mut _ as *mut c_void, 64*1024, 64*1024);
        substream = (*substream).next;
    }
    substream = (*pcm).streams[SNDRV_PCM_STREAM_CAPTURE as usize].substream;
    while !substream.is_null() {
        snd_pcm_set_managed_buffer(substream, SNDRV_DMA_TYPE_DEV, &mut (*(*emu).pci).dev as *mut _ as *mut c_void, 64*1024, 64*1024);
        substream = (*substream).next;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_pcm_multi(emu: *mut snd_emu10k1, device: c_int) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let err = snd_pcm_new((*emu).card, b"emu10k1\0".as_ptr() as *const c_char, device, 1, 0, &mut pcm);
    if err < 0 { return err; }
    (*pcm).private_data = emu as *mut c_void;
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_emu10k1_efx_playback_ops);
    (*pcm).info_flags = 0;
    (*pcm).dev_subclass = SNDRV_PCM_SUBCLASS_GENERIC_MIX;
    strscpy((*pcm).name.as_mut_ptr(), b"Multichannel Playback\0".as_ptr() as *const c_char);
    (*emu).pcm_multi = pcm;
    let mut substream = (*pcm).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].substream;
    while !substream.is_null() {
        snd_pcm_lib_preallocate_pages(substream, SNDRV_DMA_TYPE_DEV_SG, &mut (*(*emu).pci).dev as *mut _ as *mut c_void, 64*1024, 64*1024);
        substream = (*substream).next;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_pcm_mic(emu: *mut snd_emu10k1, device: c_int) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let err = snd_pcm_new((*emu).card, b"emu10k1 mic\0".as_ptr() as *const c_char, device, 0, 1, &mut pcm);
    if err < 0 { return err; }
    (*pcm).private_data = emu as *mut c_void;
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_emu10k1_capture_mic_ops);
    (*pcm).info_flags = 0;
    strscpy((*pcm).name.as_mut_ptr(), b"Mic Capture\0".as_ptr() as *const c_char);
    (*emu).pcm_mic = pcm;
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, &mut (*(*emu).pci).dev as *mut _ as *mut c_void, 64*1024, 64*1024);
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_pcm_efx(emu: *mut snd_emu10k1, device: c_int) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let mut kctl: *mut snd_kcontrol;
    let mut err = snd_pcm_new((*emu).card, b"emu10k1 efx\0".as_ptr() as *const c_char, device, if (*emu).audigy != 0 { 0 } else { 8 }, 1, &mut pcm);
    if err < 0 { return err; }
    (*pcm).private_data = emu as *mut c_void;
    if (*emu).audigy == 0 {
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_emu10k1_fx8010_playback_ops);
    }
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_emu10k1_capture_efx_ops);
    (*pcm).info_flags = 0;
    if (*emu).audigy != 0 {
        strscpy((*pcm).name.as_mut_ptr(), b"Multichannel Capture\0".as_ptr() as *const c_char);
    } else {
        strscpy((*pcm).name.as_mut_ptr(), b"Multichannel Capture/PT Playback\0".as_ptr() as *const c_char);
    }
    (*emu).pcm_efx = pcm;
    if (*(*emu).card_capabilities).emu_model == 0 {
        if (*emu).audigy != 0 {
            (*emu).efx_voices_mask[0] = 0;
            (*emu).efx_voices_mask[1] = 0xffff;
        } else {
            (*emu).efx_voices_mask[0] = 0xffff0000;
            (*emu).efx_voices_mask[1] = 0;
        }
        kctl = snd_ctl_new1(&snd_emu10k1_pcm_efx_voices_mask, emu as *mut c_void);
        if kctl.is_null() { return -ENOMEM; }
        (*kctl).id.device = device;
        err = snd_ctl_add((*emu).card, kctl);
        if err < 0 { return err; }
    } else {
        // On E-MU cards, the DSP code copies the P16VINs/EMU32INs to
        // FXBUS2. These are already selected & routed by the FPGA,
        // so there is no need to apply additional masking.
    }
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, &mut (*(*emu).pci).dev as *mut _ as *mut c_void, 64*1024, 64*1024);
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
