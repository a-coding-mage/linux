/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Translated from sound/pcm_params.h. Definitions from sound/pcm.h are external. */

pub const SNDRV_MASK_BITS: u32 = 64;
pub const SNDRV_MASK_SIZE: usize = (SNDRV_MASK_BITS / 32) as usize;

#[inline]
pub const fn mask_ofs(i: u32) -> usize { (i >> 5) as usize }
#[inline]
pub const fn mask_bit(i: u32) -> u32 { 1u32 << (i & 31) }

extern "C" {
    pub fn snd_pcm_hw_param_first(pcm: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, var: snd_pcm_hw_param_t, dir: *mut i32) -> i32;
    pub fn snd_pcm_hw_param_last(pcm: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, var: snd_pcm_hw_param_t, dir: *mut i32) -> i32;
    pub fn snd_pcm_hw_param_value(params: *const snd_pcm_hw_params, var: snd_pcm_hw_param_t, dir: *mut i32) -> i32;
}

#[inline]
pub unsafe fn snd_mask_none(mask: *mut snd_mask) { core::ptr::write_bytes(mask as *mut u8, 0, core::mem::size_of::<snd_mask>()); }
#[inline]
pub unsafe fn snd_mask_any(mask: *mut snd_mask) { core::ptr::write_bytes(mask as *mut u8, 0xff, SNDRV_MASK_SIZE * core::mem::size_of::<u32>()); }
#[inline]
pub unsafe fn snd_mask_empty(mask: *const snd_mask) -> i32 {
    for i in 0..SNDRV_MASK_SIZE { if (*mask).bits[i] != 0 { return 0; } }
    1
}
#[inline]
pub unsafe fn snd_mask_min(mask: *const snd_mask) -> u32 {
    for i in 0..SNDRV_MASK_SIZE { if (*mask).bits[i] != 0 { return (*mask).bits[i].trailing_zeros() + ((i as u32) << 5); } }
    0
}
#[inline]
pub unsafe fn snd_mask_max(mask: *const snd_mask) -> u32 {
    let mut i = SNDRV_MASK_SIZE as isize - 1;
    while i >= 0 { if (*mask).bits[i as usize] != 0 { return 31 - (*mask).bits[i as usize].leading_zeros() + ((i as u32) << 5); } i -= 1; }
    0
}
#[inline]
pub unsafe fn snd_mask_set(mask: *mut snd_mask, val: u32) { (*mask).bits[mask_ofs(val)] |= mask_bit(val); }
#[inline]
pub unsafe fn snd_mask_set_format(mask: *mut snd_mask, format: snd_pcm_format_t) { snd_mask_set(mask, format as u32); }
#[inline]
pub unsafe fn snd_mask_reset(mask: *mut snd_mask, val: u32) { (*mask).bits[mask_ofs(val)] &= !mask_bit(val); }
#[inline]
pub unsafe fn snd_mask_set_range(mask: *mut snd_mask, from: u32, to: u32) { let mut i = from; while i <= to { snd_mask_set(mask, i); i = i.wrapping_add(1); } }
#[inline]
pub unsafe fn snd_mask_reset_range(mask: *mut snd_mask, from: u32, to: u32) { let mut i = from; while i <= to { snd_mask_reset(mask, i); i = i.wrapping_add(1); } }
#[inline]
pub unsafe fn snd_mask_leave(mask: *mut snd_mask, val: u32) { let v = (*mask).bits[mask_ofs(val)] & mask_bit(val); snd_mask_none(mask); (*mask).bits[mask_ofs(val)] = v; }
#[inline]
pub unsafe fn snd_mask_intersect(mask: *mut snd_mask, v: *const snd_mask) { for i in 0..SNDRV_MASK_SIZE { (*mask).bits[i] &= (*v).bits[i]; } }
#[inline]
pub unsafe fn snd_mask_eq(mask: *const snd_mask, v: *const snd_mask) -> i32 { (core::slice::from_raw_parts(mask as *const u8, SNDRV_MASK_SIZE * 4) == core::slice::from_raw_parts(v as *const u8, SNDRV_MASK_SIZE * 4)) as i32 }
#[inline]
pub unsafe fn snd_mask_copy(mask: *mut snd_mask, v: *const snd_mask) { *mask = *v; }
#[inline]
pub unsafe fn snd_mask_test(mask: *const snd_mask, val: u32) -> i32 { ((*mask).bits[mask_ofs(val)] & mask_bit(val)) as i32 }
#[inline]
pub unsafe fn snd_mask_test_format(mask: *const snd_mask, format: snd_pcm_format_t) -> i32 { snd_mask_test(mask, format as u32) }
#[inline]
pub unsafe fn snd_mask_single(mask: *const snd_mask) -> i32 { let mut c = 0; for i in 0..SNDRV_MASK_SIZE { let b = (*mask).bits[i]; if b == 0 { continue; } if b & (b - 1) != 0 || c != 0 { return 0; } c += 1; } 1 }
#[inline]
pub unsafe fn snd_mask_refine(mask: *mut snd_mask, v: *const snd_mask) -> i32 { let old = *mask; snd_mask_intersect(mask, v); if snd_mask_empty(mask) != 0 { return -22; } (!((snd_mask_eq(mask, &old) != 0))) as i32 }
#[inline]
pub unsafe fn snd_mask_refine_first(mask: *mut snd_mask) -> i32 { if snd_mask_single(mask) != 0 { 0 } else { snd_mask_leave(mask, snd_mask_min(mask)); 1 } }
#[inline]
pub unsafe fn snd_mask_refine_last(mask: *mut snd_mask) -> i32 { if snd_mask_single(mask) != 0 { 0 } else { snd_mask_leave(mask, snd_mask_max(mask)); 1 } }
#[inline]
pub unsafe fn snd_mask_refine_min(mask: *mut snd_mask, val: u32) -> i32 { if snd_mask_min(mask) >= val { return 0; } snd_mask_reset_range(mask, 0, val - 1); if snd_mask_empty(mask) != 0 { return -22; } 1 }
#[inline]
pub unsafe fn snd_mask_refine_max(mask: *mut snd_mask, val: u32) -> i32 { if snd_mask_max(mask) <= val { return 0; } snd_mask_reset_range(mask, val + 1, SNDRV_MASK_BITS); if snd_mask_empty(mask) != 0 { return -22; } 1 }
#[inline]
pub unsafe fn snd_mask_refine_set(mask: *mut snd_mask, val: u32) -> i32 { let changed = (snd_mask_single(mask) == 0) as i32; snd_mask_leave(mask, val); if snd_mask_empty(mask) != 0 { return -22; } changed }
#[inline] pub unsafe fn snd_mask_value(mask: *const snd_mask) -> i32 { snd_mask_min(mask) as i32 }

#[inline] pub unsafe fn snd_interval_any(i: *mut snd_interval) { (*i).min = 0; (*i).openmin = 0; (*i).max = u32::MAX; (*i).openmax = 0; (*i).integer = 0; (*i).empty = 0; }
#[inline] pub unsafe fn snd_interval_none(i: *mut snd_interval) { (*i).empty = 1; }
#[inline] pub unsafe fn snd_interval_checkempty(i: *const snd_interval) -> i32 { ((*i).min > (*i).max || ((*i).min == (*i).max && ((*i).openmin != 0 || (*i).openmax != 0))) as i32 }
#[inline] pub unsafe fn snd_interval_empty(i: *const snd_interval) -> i32 { (*i).empty }
#[inline] pub unsafe fn snd_interval_single(i: *const snd_interval) -> i32 { ((*i).min == (*i).max || ((*i).min + 1 == (*i).max && ((*i).openmin != 0 || (*i).openmax != 0))) as i32 }
#[inline] pub unsafe fn snd_interval_value(i: *const snd_interval) -> u32 { if (*i).openmin != 0 && (*i).openmax == 0 { (*i).max } else { (*i).min } }
#[inline] pub unsafe fn snd_interval_min(i: *const snd_interval) -> u32 { (*i).min }
#[inline] pub unsafe fn snd_interval_max(i: *const snd_interval) -> u32 { (*i).max - ((*i).openmax != 0) as u32 }
#[inline] pub unsafe fn snd_interval_test(i: *const snd_interval, val: u32) -> i32 { (!((*i).min > val || ((*i).min == val && (*i).openmin != 0) || (*i).max < val || ((*i).max == val && (*i).openmax != 0))) as i32) }
#[inline] pub unsafe fn snd_interval_copy(d: *mut snd_interval, s: *const snd_interval) { *d = *s; }
#[inline] pub unsafe fn snd_interval_setinteger(i: *mut snd_interval) -> i32 { if (*i).integer != 0 { return 0; } if (*i).openmin != 0 && (*i).openmax != 0 && (*i).min == (*i).max { return -22; } (*i).integer = 1; 1 }
#[inline] pub unsafe fn snd_interval_eq(i1: *const snd_interval, i2: *const snd_interval) -> i32 { if (*i1).empty != 0 { return ((*i2).empty != 0) as i32; } if (*i2).empty != 0 { return ((*i1).empty != 0) as i32; } ((*i1).min == (*i2).min && (*i1).openmin == (*i2).openmin && (*i1).max == (*i2).max && (*i1).openmax == (*i2).openmax) as i32 }

#[inline] pub unsafe fn params_access(p: *const snd_pcm_hw_params) -> snd_pcm_access_t { snd_mask_min(hw_param_mask_c(p, SNDRV_PCM_HW_PARAM_ACCESS)) as snd_pcm_access_t }
#[inline] pub unsafe fn params_format(p: *const snd_pcm_hw_params) -> snd_pcm_format_t { snd_mask_min(hw_param_mask_c(p, SNDRV_PCM_HW_PARAM_FORMAT)) as snd_pcm_format_t }
#[inline] pub unsafe fn params_subformat(p: *const snd_pcm_hw_params) -> snd_pcm_subformat_t { snd_mask_min(hw_param_mask_c(p, SNDRV_PCM_HW_PARAM_SUBFORMAT)) as snd_pcm_subformat_t }
#[inline] pub unsafe fn params_period_bytes(p: *const snd_pcm_hw_params) -> u32 { (*hw_param_interval_c(p, SNDRV_PCM_HW_PARAM_PERIOD_BYTES)).min }
#[inline] pub unsafe fn params_width(p: *const snd_pcm_hw_params) -> i32 { snd_pcm_format_width(params_format(p)) }
#[inline] pub unsafe fn params_physical_width(p: *const snd_pcm_hw_params) -> i32 { snd_pcm_format_physical_width(params_format(p)) }
extern "C" { pub fn snd_pcm_hw_params_bits(p: *const snd_pcm_hw_params) -> i32; }
#[inline] pub unsafe fn params_set_format(p: *mut snd_pcm_hw_params, fmt: snd_pcm_format_t) { snd_mask_set_format(hw_param_mask(p, SNDRV_PCM_HW_PARAM_FORMAT), fmt); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
