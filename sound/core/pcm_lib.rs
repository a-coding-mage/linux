// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Digital Audio (PCM) abstract layer
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *                   Abramo Bagnara <abramo@alsa-project.org>
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

use crate::*;

const XRUN_DEBUG_BASIC: c_uint = 1 << 0;
const XRUN_DEBUG_STACK: c_uint = 1 << 1; /* dump also stack */
const XRUN_DEBUG_JIFFIESCHECK: c_uint = 1 << 2; /* do jiffies check */

#[inline]
unsafe fn xrun_debug(substream: *mut snd_pcm_substream, mask: c_uint) -> c_uint {
    /* CONFIG_SND_PCM_XRUN_DEBUG: use (*substream).pstr.xrun_debug & mask; otherwise zero. */
    #[cfg(CONFIG_SND_PCM_XRUN_DEBUG)]
    { (*(*substream).pstr).xrun_debug & mask }
    #[cfg(not(CONFIG_SND_PCM_XRUN_DEBUG))]
    { let _ = (substream, mask); 0 }
}

#[inline]
unsafe fn dump_stack_on_xrun(substream: *mut snd_pcm_substream) {
    if xrun_debug(substream, XRUN_DEBUG_STACK) != 0 { dump_stack(); }
}

#[inline]
unsafe fn update_silence_vars(runtime: *mut snd_pcm_runtime, ptr_: snd_pcm_uframes_t, new_ptr: snd_pcm_uframes_t) {
    let mut delta: snd_pcm_sframes_t = new_ptr as snd_pcm_sframes_t - ptr_ as snd_pcm_sframes_t;
    if delta == 0 { return; }
    if delta < 0 { delta += (*runtime).boundary as snd_pcm_sframes_t; }
    if (delta as snd_pcm_uframes_t) < (*runtime).silence_filled {
        (*runtime).silence_filled -= delta as snd_pcm_uframes_t;
    } else {
        (*runtime).silence_filled = 0;
    }
    (*runtime).silence_start = new_ptr;
}

/* fill ring buffer with silence */
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_playback_silence(substream: *mut snd_pcm_substream, mut new_hw_ptr: snd_pcm_uframes_t) {
    let runtime = (*substream).runtime;
    let mut frames: snd_pcm_uframes_t;
    let mut ofs: snd_pcm_uframes_t;
    let mut transfer: snd_pcm_uframes_t;

    if (*runtime).silence_size < (*runtime).boundary {
        let mut noise_dist: snd_pcm_sframes_t;
        let appl_ptr = READ_ONCE((*(*runtime).control).appl_ptr);
        update_silence_vars(runtime, (*runtime).silence_start, appl_ptr);
        if new_hw_ptr == ULONG_MAX { new_hw_ptr = (*(*runtime).status).hw_ptr; }
        noise_dist = appl_ptr as snd_pcm_sframes_t - new_hw_ptr as snd_pcm_sframes_t;
        if noise_dist < 0 { noise_dist += (*runtime).boundary as snd_pcm_sframes_t; }
        noise_dist += (*runtime).silence_filled as snd_pcm_sframes_t;
        if noise_dist >= (*runtime).silence_threshold as snd_pcm_sframes_t { return; }
        frames = (*runtime).silence_threshold - noise_dist as snd_pcm_uframes_t;
        if frames > (*runtime).silence_size { frames = (*runtime).silence_size; }
    } else {
        let hw_ptr = (*(*runtime).status).hw_ptr;
        if new_hw_ptr == ULONG_MAX {
            let mut avail = (*(*runtime).control).appl_ptr as snd_pcm_sframes_t - hw_ptr as snd_pcm_sframes_t;
            if avail < 0 { avail += (*runtime).boundary as snd_pcm_sframes_t; }
            (*runtime).silence_filled = if avail as snd_pcm_uframes_t > (*runtime).buffer_size { 0 } else { avail as snd_pcm_uframes_t };
            (*runtime).silence_start = hw_ptr;
        } else {
            update_silence_vars(runtime, hw_ptr, new_hw_ptr);
        }
        frames = (*runtime).buffer_size - (*runtime).silence_filled;
    }
    if snd_BUG_ON(frames > (*runtime).buffer_size) != 0 || frames == 0 { return; }
    ofs = ((*runtime).silence_start + (*runtime).silence_filled) % (*runtime).buffer_size;
    while frames > 0 {
        transfer = if ofs + frames > (*runtime).buffer_size { (*runtime).buffer_size - ofs } else { frames };
        let err = fill_silence_frames(substream, ofs, transfer);
        snd_BUG_ON(err < 0);
        (*runtime).silence_filled += transfer;
        frames -= transfer;
        ofs = 0;
    }
    snd_pcm_dma_buffer_sync(substream, SNDRV_DMA_SYNC_DEVICE);
}

#[cfg(CONFIG_SND_DEBUG)]
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_debug_name(substream: *mut snd_pcm_substream, name: *mut c_char, len: usize) {
    snprintf(name, len, c"pcmC%dD%d%c:%d".as_ptr(), (*(*(*substream).pcm).card).number, (*(*substream).pcm).device, if (*substream).stream != 0 { 'c' as c_int } else { 'p' as c_int }, (*substream).number);
}

#[no_mangle]
pub unsafe extern "C" fn __snd_pcm_xrun(substream: *mut snd_pcm_substream) {
    let runtime = (*substream).runtime;
    trace_xrun(substream);
    if (*runtime).tstamp_mode == SNDRV_PCM_TSTAMP_ENABLE {
        let mut tstamp: timespec64 = core::mem::zeroed();
        snd_pcm_gettime(runtime, &mut tstamp);
        (*(*runtime).status).tstamp.tv_sec = tstamp.tv_sec;
        (*(*runtime).status).tstamp.tv_nsec = tstamp.tv_nsec;
    }
    snd_pcm_stop(substream, SNDRV_PCM_STATE_XRUN);
    if xrun_debug(substream, XRUN_DEBUG_BASIC) != 0 {
        let mut name = [0 as c_char; 16];
        snd_pcm_debug_name(substream, name.as_mut_ptr(), name.len());
        pcm_warn((*substream).pcm, c"XRUN: %s\n".as_ptr(), name.as_ptr());
        dump_stack_on_xrun(substream);
    }
    #[cfg(CONFIG_SND_PCM_XRUN_DEBUG)] { (*substream).xrun_counter += 1; }
}

#[inline]
unsafe fn hw_ptr_error(_substream: *mut snd_pcm_substream, _in_interrupt: c_uint, reason: *const c_char) {
    trace_hw_ptr_error(_substream, reason);
    if xrun_debug(_substream, XRUN_DEBUG_BASIC) != 0 { dump_stack_on_xrun(_substream); }
}

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_update_state(substream: *mut snd_pcm_substream, runtime: *mut snd_pcm_runtime) -> c_int {
    let avail = snd_pcm_avail(substream);
    if avail > (*runtime).avail_max { (*runtime).avail_max = avail; }
    if (*runtime).state == SNDRV_PCM_STATE_DRAINING {
        if avail >= (*runtime).buffer_size { snd_pcm_drain_done(substream); return -EPIPE; }
    } else if avail >= (*runtime).stop_threshold {
        __snd_pcm_xrun(substream); return -EPIPE;
    }
    if (*runtime).twake != 0 {
        if avail >= (*runtime).twake { wake_up(&mut (*runtime).tsleep); }
    } else if avail >= (*(*runtime).control).avail_min { wake_up(&mut (*runtime).sleep); }
    0
}

unsafe fn update_audio_tstamp(substream: *mut snd_pcm_substream, curr_tstamp: *mut timespec64, audio_tstamp: *mut timespec64) {
    let runtime = (*substream).runtime;
    let mut driver_tstamp: timespec64 = core::mem::zeroed();
    if (*runtime).tstamp_mode != SNDRV_PCM_TSTAMP_ENABLE { return; }
    if (*(*substream).ops).get_time_info.is_none() || (*runtime).audio_tstamp_report.actual_type == SNDRV_PCM_AUDIO_TSTAMP_TYPE_DEFAULT {
        let mut audio_frames: u64 = ((*runtime).hw_ptr_wrap + (*(*runtime).status).hw_ptr) as u64;
        if (*runtime).audio_tstamp_config.report_delay != 0 {
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK { audio_frames -= (*runtime).delay as u64; } else { audio_frames += (*runtime).delay as u64; }
        }
        let audio_nsecs = div_u64(audio_frames.wrapping_mul(1000000000), (*runtime).rate as u64);
        *audio_tstamp = ns_to_timespec64(audio_nsecs);
    }
    if (*(*runtime).status).audio_tstamp.tv_sec != (*audio_tstamp).tv_sec || (*(*runtime).status).audio_tstamp.tv_nsec != (*audio_tstamp).tv_nsec {
        (*(*runtime).status).audio_tstamp.tv_sec = (*audio_tstamp).tv_sec;
        (*(*runtime).status).audio_tstamp.tv_nsec = (*audio_tstamp).tv_nsec;
        (*(*runtime).status).tstamp.tv_sec = (*curr_tstamp).tv_sec;
        (*(*runtime).status).tstamp.tv_nsec = (*curr_tstamp).tv_nsec;
    }
    snd_pcm_gettime((*substream).runtime, &mut driver_tstamp);
    (*runtime).driver_tstamp = driver_tstamp;
}

unsafe fn snd_pcm_update_hw_ptr0(substream: *mut snd_pcm_substream, in_interrupt: c_uint) -> c_int {
    let runtime = (*substream).runtime;
    let mut pos: snd_pcm_uframes_t;
    let old_hw_ptr = (*(*runtime).status).hw_ptr;
    let mut new_hw_ptr: snd_pcm_uframes_t;
    let mut hw_base: snd_pcm_uframes_t;
    let mut hdelta: snd_pcm_sframes_t;
    let mut delta: snd_pcm_sframes_t;
    let mut jdelta: c_ulong;
    let curr_jiffies: c_ulong;
    let mut curr_tstamp: timespec64 = core::mem::zeroed();
    let mut audio_tstamp: timespec64 = core::mem::zeroed();
    let mut crossed_boundary = 0;

    pos = ((*(*substream).ops).pointer.unwrap())(substream);
    curr_jiffies = jiffies;
    if (*runtime).tstamp_mode == SNDRV_PCM_TSTAMP_ENABLE {
        if (*(*substream).ops).get_time_info.is_some() && (*runtime).audio_tstamp_config.type_requested != SNDRV_PCM_AUDIO_TSTAMP_TYPE_DEFAULT {
            ((*(*substream).ops).get_time_info.unwrap())(substream, &mut curr_tstamp, &mut audio_tstamp, &mut (*runtime).audio_tstamp_config, &mut (*runtime).audio_tstamp_report);
            if (*runtime).audio_tstamp_report.actual_type == SNDRV_PCM_AUDIO_TSTAMP_TYPE_DEFAULT { snd_pcm_gettime(runtime, &mut curr_tstamp); }
        } else { snd_pcm_gettime(runtime, &mut curr_tstamp); }
    }
    if pos == SNDRV_PCM_POS_XRUN { __snd_pcm_xrun(substream); return -EPIPE; }
    if pos >= (*runtime).buffer_size { pos = 0; }
    pos -= pos % (*runtime).min_align;
    trace_hwptr(substream, pos, in_interrupt);
    hw_base = (*runtime).hw_ptr_base;
    new_hw_ptr = hw_base + pos;
    if in_interrupt != 0 {
        delta = ((*runtime).hw_ptr_interrupt + (*runtime).period_size) as snd_pcm_sframes_t;
        if delta > new_hw_ptr as snd_pcm_sframes_t {
            hdelta = curr_jiffies.wrapping_sub((*runtime).hw_ptr_jiffies) as snd_pcm_sframes_t;
            if hdelta > ((*runtime).hw_ptr_buffer_jiffies / 2 + 1) as snd_pcm_sframes_t {
                hw_base += (*runtime).buffer_size;
                if hw_base >= (*runtime).boundary { hw_base = 0; crossed_boundary += 1; }
                new_hw_ptr = hw_base + pos;
            }
        }
    }
    if new_hw_ptr < old_hw_ptr {
        hw_base += (*runtime).buffer_size;
        if hw_base >= (*runtime).boundary { hw_base = 0; crossed_boundary += 1; }
        new_hw_ptr = hw_base + pos;
    }
    delta = new_hw_ptr as snd_pcm_sframes_t - old_hw_ptr as snd_pcm_sframes_t;
    if delta < 0 { delta += (*runtime).boundary as snd_pcm_sframes_t; }
    if (*runtime).no_period_wakeup != 0 {
        jdelta = curr_jiffies.wrapping_sub((*runtime).hw_ptr_jiffies);
        if jdelta >= (*runtime).hw_ptr_buffer_jiffies / 2 {
            hdelta = jdelta as snd_pcm_sframes_t - delta * HZ as snd_pcm_sframes_t / (*runtime).rate as snd_pcm_sframes_t;
            let xrun_threshold = ((*runtime).hw_ptr_buffer_jiffies / 2 + 1) as snd_pcm_sframes_t;
            while hdelta > xrun_threshold {
                delta += (*runtime).buffer_size as snd_pcm_sframes_t;
                hw_base += (*runtime).buffer_size;
                if hw_base >= (*runtime).boundary { hw_base = 0; crossed_boundary += 1; }
                new_hw_ptr = hw_base + pos;
                hdelta -= (*runtime).hw_ptr_buffer_jiffies as snd_pcm_sframes_t;
            }
        }
    } else {
        if delta >= ((*runtime).buffer_size + (*runtime).period_size) as snd_pcm_sframes_t { hw_ptr_error(substream, in_interrupt, c"Unexpected hw_ptr".as_ptr()); return 0; }
        if xrun_debug(substream, XRUN_DEBUG_JIFFIESCHECK) != 0 && ((*runtime).hw.info & SNDRV_PCM_INFO_BATCH) == 0 {
            hdelta = delta;
            if hdelta >= (*runtime).delay as snd_pcm_sframes_t {
                hdelta -= (*runtime).delay as snd_pcm_sframes_t;
                jdelta = curr_jiffies.wrapping_sub((*runtime).hw_ptr_jiffies);
                if ((hdelta * HZ as snd_pcm_sframes_t) / (*runtime).rate as snd_pcm_sframes_t) as c_ulong > jdelta + HZ / 100 {
                    delta = (jdelta / (((*runtime).period_size * HZ as snd_pcm_uframes_t) / (*runtime).rate as snd_pcm_uframes_t + HZ / 100)) as snd_pcm_sframes_t;
                    new_hw_ptr = old_hw_ptr;
                    while delta > 0 {
                        new_hw_ptr += (*runtime).period_size;
                        if new_hw_ptr >= (*runtime).boundary { new_hw_ptr -= (*runtime).boundary; crossed_boundary -= 1; }
                        delta -= 1;
                    }
                    hw_ptr_error(substream, in_interrupt, c"hw_ptr skipping".as_ptr());
                    delta = 0;
                    hw_base = new_hw_ptr - (new_hw_ptr % (*runtime).buffer_size);
                }
            }
        }
        if delta > ((*runtime).period_size + (*runtime).period_size / 2) as snd_pcm_sframes_t { hw_ptr_error(substream, in_interrupt, c"Lost interrupts?".as_ptr()); }
    }
    if (*(*runtime).status).hw_ptr == new_hw_ptr { (*runtime).hw_ptr_jiffies = curr_jiffies; update_audio_tstamp(substream, &mut curr_tstamp, &mut audio_tstamp); return 0; }
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK && (*runtime).silence_size > 0 { snd_pcm_playback_silence(substream, new_hw_ptr); }
    if in_interrupt != 0 {
        delta = new_hw_ptr as snd_pcm_sframes_t - (*runtime).hw_ptr_interrupt as snd_pcm_sframes_t;
        if delta < 0 { delta += (*runtime).boundary as snd_pcm_sframes_t; }
        delta -= (delta as snd_pcm_uframes_t % (*runtime).period_size) as snd_pcm_sframes_t;
        (*runtime).hw_ptr_interrupt += delta as snd_pcm_uframes_t;
        if (*runtime).hw_ptr_interrupt >= (*runtime).boundary { (*runtime).hw_ptr_interrupt -= (*runtime).boundary; }
    }
    (*runtime).hw_ptr_base = hw_base;
    (*(*runtime).status).hw_ptr = new_hw_ptr;
    (*runtime).hw_ptr_jiffies = curr_jiffies;
    if crossed_boundary != 0 { snd_BUG_ON(crossed_boundary != 1); (*runtime).hw_ptr_wrap += (*runtime).boundary; }
    update_audio_tstamp(substream, &mut curr_tstamp, &mut audio_tstamp);
    snd_pcm_update_state(substream, runtime)
}

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_update_hw_ptr(substream: *mut snd_pcm_substream) -> c_int { snd_pcm_update_hw_ptr0(substream, 0) }

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops) {
    let stream = &mut (*pcm).streams[direction as usize] as *mut snd_pcm_str;
    let mut substream = (*stream).substream;
    while !substream.is_null() { (*substream).ops = ops; substream = (*substream).next; }
}

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_set_sync_per_card(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, id: *const u8, mut len: c_uint) {
    *(*params).sync.as_mut_ptr().cast::<__le32>() = cpu_to_le32((*(*(*substream).pcm).card).number as u32);
    len = core::cmp::min(12, len);
    memcpy((*params).sync.as_mut_ptr().add(4).cast(), id.cast(), len as usize);
    memset((*params).sync.as_mut_ptr().add(4 + len as usize).cast(), 0, (12 - len) as usize);
}

#[inline] unsafe fn div32(a: c_uint, b: c_uint, r: *mut c_uint) -> c_uint { if b == 0 { *r = 0; UINT_MAX } else { *r = a % b; a / b } }
#[inline] unsafe fn div_down(a: c_uint, b: c_uint) -> c_uint { if b == 0 { UINT_MAX } else { a / b } }
#[inline] unsafe fn div_up(a: c_uint, b: c_uint) -> c_uint { if b == 0 { UINT_MAX } else { let mut r=0; let mut q=div32(a,b,&mut r); if r!=0 { q+=1; } q } }
#[inline] unsafe fn mul(a: c_uint, b: c_uint) -> c_uint { if a == 0 { 0 } else if div_down(UINT_MAX, a) < b { UINT_MAX } else { a.wrapping_mul(b) } }
#[inline] unsafe fn muldiv32(a: c_uint, b: c_uint, c: c_uint, r: *mut c_uint) -> c_uint { let mut n=(a as u64).wrapping_mul(b as u64); if c==0 { *r=0; return UINT_MAX; } n=div_u64_rem(n,c as u64,r); if n>=UINT_MAX as u64 { *r=0; UINT_MAX } else { n as c_uint } }

#[no_mangle]
pub unsafe extern "C" fn snd_interval_refine(i: *mut snd_interval, v: *const snd_interval) -> c_int {
    let mut changed = 0;
    if snd_BUG_ON(snd_interval_empty(i)) != 0 { return -EINVAL; }
    if (*i).min < (*v).min { (*i).min=(*v).min; (*i).openmin=(*v).openmin; changed=1; } else if (*i).min == (*v).min && (*i).openmin == 0 && (*v).openmin != 0 { (*i).openmin=1; changed=1; }
    if (*i).max > (*v).max { (*i).max=(*v).max; (*i).openmax=(*v).openmax; changed=1; } else if (*i).max == (*v).max && (*i).openmax == 0 && (*v).openmax != 0 { (*i).openmax=1; changed=1; }
    if (*i).integer == 0 && (*v).integer != 0 { (*i).integer=1; changed=1; }
    if (*i).integer != 0 { if (*i).openmin != 0 { (*i).min+=1; (*i).openmin=0; } if (*i).openmax != 0 { (*i).max-=1; (*i).openmax=0; } } else if (*i).openmin == 0 && (*i).openmax == 0 && (*i).min == (*i).max { (*i).integer=1; }
    if snd_interval_checkempty(i) != 0 { snd_interval_none(i); return -EINVAL; }
    changed
}

unsafe fn snd_interval_refine_first(i: *mut snd_interval) -> c_int { let last_max=(*i).max; if snd_BUG_ON(snd_interval_empty(i))!=0 {return -EINVAL;} if snd_interval_single(i)!=0{return 0;} (*i).max=(*i).min; if (*i).openmin!=0{(*i).max+=1;} (*i).openmax=((*i).openmax!=0 && (*i).max>=last_max) as c_uint; 1 }
unsafe fn snd_interval_refine_last(i: *mut snd_interval) -> c_int { let last_min=(*i).min; if snd_BUG_ON(snd_interval_empty(i))!=0 {return -EINVAL;} if snd_interval_single(i)!=0{return 0;} (*i).min=(*i).max; if (*i).openmax!=0{(*i).min-=1;} (*i).openmin=((*i).openmin!=0 && (*i).min<=last_min) as c_uint; 1 }

#[no_mangle]
pub unsafe extern "C" fn snd_interval_mul(a:*const snd_interval,b:*const snd_interval,c:*mut snd_interval){ if (*a).empty!=0||(*b).empty!=0{snd_interval_none(c);return;} (*c).empty=0; (*c).min=mul((*a).min,(*b).min); (*c).openmin=((*a).openmin!=0||(*b).openmin!=0) as c_uint; (*c).max=mul((*a).max,(*b).max); (*c).openmax=((*a).openmax!=0||(*b).openmax!=0) as c_uint; (*c).integer=((*a).integer!=0&&(*b).integer!=0) as c_uint; }
#[no_mangle]
pub unsafe extern "C" fn snd_interval_div(a:*const snd_interval,b:*const snd_interval,c:*mut snd_interval){ let mut r=0; if (*a).empty!=0||(*b).empty!=0{snd_interval_none(c);return;} (*c).empty=0; (*c).min=div32((*a).min,(*b).max,&mut r); (*c).openmin=(r!=0||(*a).openmin!=0||(*b).openmax!=0) as c_uint; if (*b).min>0{ (*c).max=div32((*a).max,(*b).min,&mut r); if r!=0{(*c).max+=1;(*c).openmax=1;}else{(*c).openmax=((*a).openmax!=0||(*b).openmin!=0) as c_uint;} }else{(*c).max=UINT_MAX;(*c).openmax=0;} (*c).integer=0; }
#[no_mangle]
pub unsafe extern "C" fn snd_interval_muldivk(a:*const snd_interval,b:*const snd_interval,k:c_uint,c:*mut snd_interval){ let mut r=0; if (*a).empty!=0||(*b).empty!=0{snd_interval_none(c);return;} (*c).empty=0; (*c).min=muldiv32((*a).min,(*b).min,k,&mut r); (*c).openmin=(r!=0||(*a).openmin!=0||(*b).openmin!=0) as c_uint; (*c).max=muldiv32((*a).max,(*b).max,k,&mut r); if r!=0{(*c).max+=1;(*c).openmax=1;}else{(*c).openmax=((*a).openmax!=0||(*b).openmax!=0) as c_uint;} (*c).integer=0; }
#[no_mangle]
pub unsafe extern "C" fn snd_interval_mulkdiv(a:*const snd_interval,k:c_uint,b:*const snd_interval,c:*mut snd_interval){ let mut r=0; if (*a).empty!=0||(*b).empty!=0{snd_interval_none(c);return;} (*c).empty=0; (*c).min=muldiv32((*a).min,k,(*b).max,&mut r); (*c).openmin=(r!=0||(*a).openmin!=0||(*b).openmax!=0) as c_uint; if (*b).min>0{(*c).max=muldiv32((*a).max,k,(*b).min,&mut r); if r!=0{(*c).max+=1;(*c).openmax=1;}else{(*c).openmax=((*a).openmax!=0||(*b).openmin!=0) as c_uint;}}else{(*c).max=UINT_MAX;(*c).openmax=0;} (*c).integer=0; }

#[no_mangle]
pub unsafe extern "C" fn snd_interval_ratnum(i:*mut snd_interval,rats_count:c_uint,rats:*const snd_ratnum,nump:*mut c_uint,denp:*mut c_uint)->c_int{ let mut best_num=0;let mut best_den=0;let mut best_diff:c_int=0;let mut t:snd_interval=core::mem::zeroed();let mut result_num;let mut result_den;let mut result_diff; for k in 0..rats_count as usize{let num=(*rats.add(k)).num;let mut q=(*i).min;if q==0{q=1;}let mut den=div_up(num,q);if den<(*rats.add(k)).den_min{continue;}if den>(*rats.add(k)).den_max{den=(*rats.add(k)).den_max;}else{let r=(den-(*rats.add(k)).den_min)%(*rats.add(k)).den_step;if r!=0{den-=r;}}let mut diff=num as c_int-(q*den) as c_int;if diff<0{diff=-diff;}if best_num==0||diff*best_den as c_int<best_diff*den as c_int{best_diff=diff;best_den=den;best_num=num;}} if best_den==0{(*i).empty=1;return -EINVAL;} t.min=div_down(best_num,best_den);t.openmin=(best_num%best_den!=0) as c_uint; result_num=best_num;result_diff=best_diff;result_den=best_den; best_num=0;best_den=0;best_diff=0; for k in 0..rats_count as usize{let num=(*rats.add(k)).num;let q=(*i).max;if q==0{(*i).empty=1;return -EINVAL;}let mut den=div_down(num,q);if den>(*rats.add(k)).den_max{continue;}if den<(*rats.add(k)).den_min{den=(*rats.add(k)).den_min;}else{let r=(den-(*rats.add(k)).den_min)%(*rats.add(k)).den_step;if r!=0{den+=(*rats.add(k)).den_step-r;}}let mut diff=(q*den) as c_int-num as c_int;if diff<0{diff=-diff;}if best_num==0||diff*best_den as c_int<best_diff*den as c_int{best_diff=diff;best_den=den;best_num=num;}} if best_den==0{(*i).empty=1;return -EINVAL;} t.max=div_up(best_num,best_den);t.openmax=(best_num%best_den!=0) as c_uint;t.integer=0;let err=snd_interval_refine(i,&t);if err<0{return err;} if snd_interval_single(i)!=0{if best_diff*result_den as c_int<result_diff*best_den as c_int{result_num=best_num;result_den=best_den;} if !nump.is_null(){*nump=result_num;} if !denp.is_null(){*denp=result_den;}} err }

unsafe fn snd_interval_ratden(i:*mut snd_interval,rats_count:c_uint,rats:*const snd_ratden,nump:*mut c_uint,denp:*mut c_uint)->c_int{ let mut best_num=0;let mut best_den=0;let mut best_diff:c_int=0;let mut t:snd_interval=core::mem::zeroed(); for k in 0..rats_count as usize{let den=(*rats.add(k)).den;let q=(*i).min;let mut num=mul(q,den);if num>(*rats.add(k)).num_max{continue;}if num<(*rats.add(k)).num_min{num=(*rats.add(k)).num_max;}else{let r=(num-(*rats.add(k)).num_min)%(*rats.add(k)).num_step;if r!=0{num+=(*rats.add(k)).num_step-r;}}let diff=num as c_int-(q*den) as c_int;if best_num==0||diff*best_den as c_int<best_diff*den as c_int{best_diff=diff;best_den=den;best_num=num;}} if best_den==0{(*i).empty=1;return -EINVAL;} t.min=div_down(best_num,best_den);t.openmin=(best_num%best_den!=0) as c_uint; best_num=0;best_den=0;best_diff=0; for k in 0..rats_count as usize{let den=(*rats.add(k)).den;let q=(*i).max;let mut num=mul(q,den);if num<(*rats.add(k)).num_min{continue;}if num>(*rats.add(k)).num_max{num=(*rats.add(k)).num_max;}else{let r=(num-(*rats.add(k)).num_min)%(*rats.add(k)).num_step;if r!=0{num-=r;}}let diff=(q*den) as c_int-num as c_int;if best_num==0||diff*best_den as c_int<best_diff*den as c_int{best_diff=diff;best_den=den;best_num=num;}} if best_den==0{(*i).empty=1;return -EINVAL;} t.max=div_up(best_num,best_den);t.openmax=(best_num%best_den!=0) as c_uint;t.integer=0;let err=snd_interval_refine(i,&t); if err<0{return err;} if snd_interval_single(i)!=0{if !nump.is_null(){*nump=best_num;} if !denp.is_null(){*denp=best_den;}} err }

#[no_mangle]
pub unsafe extern "C" fn snd_interval_list(i:*mut snd_interval,count:c_uint,list:*const c_uint,mask:c_uint)->c_int{ let mut list_range:snd_interval=core::mem::zeroed(); if count==0{(*i).empty=1;return -EINVAL;} snd_interval_any(&mut list_range); list_range.min=UINT_MAX;list_range.max=0; for k in 0..count as usize{if mask!=0&&(mask&(1u32<<k))==0{continue;} if snd_interval_test(i,*list.add(k))==0{continue;} list_range.min=core::cmp::min(list_range.min,*list.add(k)); list_range.max=core::cmp::max(list_range.max,*list.add(k));} snd_interval_refine(i,&list_range) }
#[no_mangle]
pub unsafe extern "C" fn snd_interval_ranges(i:*mut snd_interval,count:c_uint,ranges:*const snd_interval,mask:c_uint)->c_int{ let mut range_union:snd_interval=core::mem::zeroed();let mut range:snd_interval=core::mem::zeroed(); if count==0{snd_interval_none(i);return -EINVAL;} snd_interval_any(&mut range_union);range_union.min=UINT_MAX;range_union.max=0; for k in 0..count as usize{if mask!=0&&(mask&(1u32<<k))==0{continue;} snd_interval_copy(&mut range,ranges.add(k)); if snd_interval_refine(&mut range,i)<0{continue;} if snd_interval_empty(&mut range)!=0{continue;} if range.min<range_union.min{range_union.min=range.min;range_union.openmin=1;} if range.min==range_union.min&&range.openmin==0{range_union.openmin=0;} if range.max>range_union.max{range_union.max=range.max;range_union.openmax=1;} if range.max==range_union.max&&range.openmax==0{range_union.openmax=0;}} snd_interval_refine(i,&range_union) }
unsafe fn snd_interval_step(i:*mut snd_interval,step:c_uint)->c_int{let mut changed=0;let mut n=(*i).min%step;if n!=0||(*i).openmin!=0{(*i).min+=step-n;(*i).openmin=0;changed=1;} n=(*i).max%step;if n!=0||(*i).openmax!=0{(*i).max-=n;(*i).openmax=0;changed=1;} if snd_interval_checkempty(i)!=0{(*i).empty=1;return -EINVAL;} changed}

/* Variadic C rule-add cannot be expressed portably in stable Rust; this preserves the fixed arguments and the sentinel dependency path used by this file's call sites. */
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_hw_rule_add(runtime:*mut snd_pcm_runtime,cond:c_uint,var:c_int,func:snd_pcm_hw_rule_func_t,private:*mut c_void,dep:c_int)->c_int{ let constrs=&mut (*runtime).hw_constraints as *mut snd_pcm_hw_constraints; if (*constrs).rules_num>=(*constrs).rules_all{let new_rules=(*constrs).rules_all+16;let new=krealloc_array((*constrs).rules,new_rules as usize,core::mem::size_of::<snd_pcm_hw_rule>(),GFP_KERNEL); if new.is_null(){return -ENOMEM;} (*constrs).rules=new.cast();(*constrs).rules_all=new_rules;} let c=(*constrs).rules.add((*constrs).rules_num as usize);(*c).cond=cond;(*c).func=func;(*c).var=var;(*c).private=private;(*c).deps[0]=dep;(*c).deps[1]=-1;(*constrs).rules_num+=1;0 }

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_hw_constraint_mask(runtime:*mut snd_pcm_runtime,var:snd_pcm_hw_param_t,mask:u32)->c_int{let constrs=&mut (*runtime).hw_constraints as *mut _;let maskp=constrs_mask(constrs,var);(*maskp).bits[0]&=mask;memset((*maskp).bits.as_mut_ptr().add(1).cast(),0,((SNDRV_MASK_MAX-32)/8) as usize);if (*maskp).bits[0]==0{-EINVAL}else{0}}
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_hw_constraint_mask64(runtime:*mut snd_pcm_runtime,var:snd_pcm_hw_param_t,mask:u64)->c_int{let constrs=&mut (*runtime).hw_constraints as *mut _;let maskp=constrs_mask(constrs,var);(*maskp).bits[0]&=mask as u32;(*maskp).bits[1]&=(mask>>32) as u32;memset((*maskp).bits.as_mut_ptr().add(2).cast(),0,((SNDRV_MASK_MAX-64)/8) as usize);if (*maskp).bits[0]==0&&(*maskp).bits[1]==0{-EINVAL}else{0}}
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_hw_constraint_integer(runtime:*mut snd_pcm_runtime,var:snd_pcm_hw_param_t)->c_int{let constrs=&mut (*runtime).hw_constraints as *mut _;snd_interval_setinteger(constrs_interval(constrs,var))}
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_hw_constraint_minmax(runtime:*mut snd_pcm_runtime,var:snd_pcm_hw_param_t,min:c_uint,max:c_uint)->c_int{let constrs=&mut (*runtime).hw_constraints as *mut _;let mut t:snd_interval=core::mem::zeroed();t.min=min;t.max=max;t.openmin=0;t.openmax=0;t.integer=0;snd_interval_refine(constrs_interval(constrs,var),&t)}

unsafe extern "C" fn snd_pcm_hw_rule_list(params:*mut snd_pcm_hw_params,rule:*mut snd_pcm_hw_rule)->c_int{let list=(*rule).private as *mut snd_pcm_hw_constraint_list;snd_interval_list(hw_param_interval(params,(*rule).var),(*list).count,(*list).list,(*list).mask)}
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_hw_constraint_list(runtime:*mut snd_pcm_runtime,cond:c_uint,var:snd_pcm_hw_param_t,l:*const snd_pcm_hw_constraint_list)->c_int{snd_pcm_hw_rule_add(runtime,cond,var,snd_pcm_hw_rule_list as snd_pcm_hw_rule_func_t,l as *mut c_void,var)}
unsafe extern "C" fn snd_pcm_hw_rule_ranges(params:*mut snd_pcm_hw_params,rule:*mut snd_pcm_hw_rule)->c_int{let r=(*rule).private as *mut snd_pcm_hw_constraint_ranges;snd_interval_ranges(hw_param_interval(params,(*rule).var),(*r).count,(*r).ranges,(*r).mask)}
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_hw_constraint_ranges(runtime:*mut snd_pcm_runtime,cond:c_uint,var:snd_pcm_hw_param_t,r:*const snd_pcm_hw_constraint_ranges)->c_int{snd_pcm_hw_rule_add(runtime,cond,var,snd_pcm_hw_rule_ranges as snd_pcm_hw_rule_func_t,r as *mut c_void,var)}
unsafe extern "C" fn snd_pcm_hw_rule_ratnums(params:*mut snd_pcm_hw_params,rule:*mut snd_pcm_hw_rule)->c_int{let r=(*rule).private as *const snd_pcm_hw_constraint_ratnums;let mut num=0;let mut den=0;let err=snd_interval_ratnum(hw_param_interval(params,(*rule).var),(*r).nrats,(*r).rats,&mut num,&mut den);if err>=0&&den!=0&&(*rule).var==SNDRV_PCM_HW_PARAM_RATE{(*params).rate_num=num;(*params).rate_den=den;}err}
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_hw_constraint_ratnums(runtime:*mut snd_pcm_runtime,cond:c_uint,var:snd_pcm_hw_param_t,r:*const snd_pcm_hw_constraint_ratnums)->c_int{snd_pcm_hw_rule_add(runtime,cond,var,snd_pcm_hw_rule_ratnums as snd_pcm_hw_rule_func_t,r as *mut c_void,var)}
unsafe extern "C" fn snd_pcm_hw_rule_ratdens(params:*mut snd_pcm_hw_params,rule:*mut snd_pcm_hw_rule)->c_int{let r=(*rule).private as *const snd_pcm_hw_constraint_ratdens;let mut num=0;let mut den=0;let err=snd_interval_ratden(hw_param_interval(params,(*rule).var),(*r).nrats,(*r).rats,&mut num,&mut den);if err>=0&&den!=0&&(*rule).var==SNDRV_PCM_HW_PARAM_RATE{(*params).rate_num=num;(*params).rate_den=den;}err}
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_hw_constraint_ratdens(runtime:*mut snd_pcm_runtime,cond:c_uint,var:snd_pcm_hw_param_t,r:*const snd_pcm_hw_constraint_ratdens)->c_int{snd_pcm_hw_rule_add(runtime,cond,var,snd_pcm_hw_rule_ratdens as snd_pcm_hw_rule_func_t,r as *mut c_void,var)}
unsafe extern "C" fn snd_pcm_hw_rule_msbits(params:*mut snd_pcm_hw_params,rule:*mut snd_pcm_hw_rule)->c_int{let l=(*rule).private as c_ulong;let width=(l&0xffff) as c_int;let msbits=(l>>16) as c_uint;let i=hw_param_interval_c(params,SNDRV_PCM_HW_PARAM_SAMPLE_BITS); if snd_interval_single(i)==0{return 0;} if snd_interval_value(i)==width as c_uint || (width==0&&snd_interval_value(i)>msbits){(*params).msbits=min_not_zero((*params).msbits,msbits);}0}
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_hw_constraint_msbits(runtime:*mut snd_pcm_runtime,cond:c_uint,width:c_uint,msbits:c_uint)->c_int{let l=((msbits<<16)|width) as c_ulong;snd_pcm_hw_rule_add(runtime,cond,-1,snd_pcm_hw_rule_msbits as snd_pcm_hw_rule_func_t,l as *mut c_void,SNDRV_PCM_HW_PARAM_SAMPLE_BITS)}
unsafe extern "C" fn snd_pcm_hw_rule_step(params:*mut snd_pcm_hw_params,rule:*mut snd_pcm_hw_rule)->c_int{snd_interval_step(hw_param_interval(params,(*rule).var),(*rule).private as c_uint)}
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_hw_constraint_step(runtime:*mut snd_pcm_runtime,cond:c_uint,var:snd_pcm_hw_param_t,step:c_ulong)->c_int{snd_pcm_hw_rule_add(runtime,cond,var,snd_pcm_hw_rule_step as snd_pcm_hw_rule_func_t,step as *mut c_void,var)}
unsafe extern "C" fn snd_pcm_hw_rule_pow2(params:*mut snd_pcm_hw_params,rule:*mut snd_pcm_hw_rule)->c_int{static POW2_SIZES:[c_uint;31]=[1<<0,1<<1,1<<2,1<<3,1<<4,1<<5,1<<6,1<<7,1<<8,1<<9,1<<10,1<<11,1<<12,1<<13,1<<14,1<<15,1<<16,1<<17,1<<18,1<<19,1<<20,1<<21,1<<22,1<<23,1<<24,1<<25,1<<26,1<<27,1<<28,1<<29,1<<30];snd_interval_list(hw_param_interval(params,(*rule).var),POW2_SIZES.len() as c_uint,POW2_SIZES.as_ptr(),0)}
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_hw_constraint_pow2(runtime:*mut snd_pcm_runtime,cond:c_uint,var:snd_pcm_hw_param_t)->c_int{snd_pcm_hw_rule_add(runtime,cond,var,snd_pcm_hw_rule_pow2 as snd_pcm_hw_rule_func_t,ptr::null_mut(),var)}
unsafe extern "C" fn snd_pcm_hw_rule_noresample_func(params:*mut snd_pcm_hw_params,rule:*mut snd_pcm_hw_rule)->c_int{let base_rate=(*rule).private as c_uint;let rate=hw_param_interval(params,SNDRV_PCM_HW_PARAM_RATE);snd_interval_list(rate,1,&base_rate,0)}
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_hw_rule_noresample(runtime:*mut snd_pcm_runtime,base_rate:c_uint)->c_int{snd_pcm_hw_rule_add(runtime,SNDRV_PCM_HW_PARAMS_NORESAMPLE,SNDRV_PCM_HW_PARAM_RATE,snd_pcm_hw_rule_noresample_func as snd_pcm_hw_rule_func_t,base_rate as *mut c_void,SNDRV_PCM_HW_PARAM_RATE)}

unsafe fn _snd_pcm_hw_param_any(params:*mut snd_pcm_hw_params,var:snd_pcm_hw_param_t){if hw_is_mask(var)!=0{snd_mask_any(hw_param_mask(params,var));(*params).cmask|=1u32<<var;(*params).rmask|=1u32<<var;return;} if hw_is_interval(var)!=0{snd_interval_any(hw_param_interval(params,var));(*params).cmask|=1u32<<var;(*params).rmask|=1u32<<var;return;} snd_BUG();}
#[no_mangle]
pub unsafe extern "C" fn _snd_pcm_hw_params_any(params:*mut snd_pcm_hw_params){memset(params.cast(),0,core::mem::size_of::<snd_pcm_hw_params>());for k in SNDRV_PCM_HW_PARAM_FIRST_MASK..=SNDRV_PCM_HW_PARAM_LAST_MASK{_snd_pcm_hw_param_any(params,k);}for k in SNDRV_PCM_HW_PARAM_FIRST_INTERVAL..=SNDRV_PCM_HW_PARAM_LAST_INTERVAL{_snd_pcm_hw_param_any(params,k);}(*params).info=!0u32;}
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_hw_param_value(params:*const snd_pcm_hw_params,var:snd_pcm_hw_param_t,dir:*mut c_int)->c_int{if hw_is_mask(var)!=0{let mask=hw_param_mask_c(params,var);if snd_mask_single(mask)==0{return -EINVAL;} if !dir.is_null(){*dir=0;} return snd_mask_value(mask);} if hw_is_interval(var)!=0{let i=hw_param_interval_c(params,var);if snd_interval_single(i)==0{return -EINVAL;} if !dir.is_null(){*dir=(*i).openmin as c_int;} return snd_interval_value(i) as c_int;} -EINVAL}
#[no_mangle]
pub unsafe extern "C" fn _snd_pcm_hw_param_setempty(params:*mut snd_pcm_hw_params,var:snd_pcm_hw_param_t){if hw_is_mask(var)!=0{snd_mask_none(hw_param_mask(params,var));(*params).cmask|=1u32<<var;(*params).rmask|=1u32<<var;}else if hw_is_interval(var)!=0{snd_interval_none(hw_param_interval(params,var));(*params).cmask|=1u32<<var;(*params).rmask|=1u32<<var;}else{snd_BUG();}}
unsafe fn _snd_pcm_hw_param_first(params:*mut snd_pcm_hw_params,var:snd_pcm_hw_param_t)->c_int{let changed=if hw_is_mask(var)!=0{snd_mask_refine_first(hw_param_mask(params,var))}else if hw_is_interval(var)!=0{snd_interval_refine_first(hw_param_interval(params,var))}else{return -EINVAL;}; if changed>0{(*params).cmask|=1u32<<var;(*params).rmask|=1u32<<var;} changed}
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_hw_param_first(pcm:*mut snd_pcm_substream,params:*mut snd_pcm_hw_params,var:snd_pcm_hw_param_t,dir:*mut c_int)->c_int{let changed=_snd_pcm_hw_param_first(params,var);if changed<0{return changed;} if (*params).rmask!=0{let err=snd_pcm_hw_refine(pcm,params);if err<0{return err;}} snd_pcm_hw_param_value(params,var,dir)}
unsafe fn _snd_pcm_hw_param_last(params:*mut snd_pcm_hw_params,var:snd_pcm_hw_param_t)->c_int{let changed=if hw_is_mask(var)!=0{snd_mask_refine_last(hw_param_mask(params,var))}else if hw_is_interval(var)!=0{snd_interval_refine_last(hw_param_interval(params,var))}else{return -EINVAL;}; if changed>0{(*params).cmask|=1u32<<var;(*params).rmask|=1u32<<var;} changed}
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_hw_param_last(pcm:*mut snd_pcm_substream,params:*mut snd_pcm_hw_params,var:snd_pcm_hw_param_t,dir:*mut c_int)->c_int{let changed=_snd_pcm_hw_param_last(params,var);if changed<0{return changed;} if (*params).rmask!=0{let err=snd_pcm_hw_refine(pcm,params);if err<0{return err;}} snd_pcm_hw_param_value(params,var,dir)}
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_hw_params_bits(p:*const snd_pcm_hw_params)->c_int{let subformat=params_subformat(p);let format=params_format(p);match format{SNDRV_PCM_FORMAT_S32_LE|SNDRV_PCM_FORMAT_U32_LE|SNDRV_PCM_FORMAT_S32_BE|SNDRV_PCM_FORMAT_U32_BE=>match subformat{SNDRV_PCM_SUBFORMAT_MSBITS_20=>20,SNDRV_PCM_SUBFORMAT_MSBITS_24=>24,_=>snd_pcm_format_width(format)},_=>snd_pcm_format_width(format)}}

unsafe extern "C" fn snd_pcm_lib_ioctl_reset(substream:*mut snd_pcm_substream,_arg:*mut c_void)->c_int{let runtime=(*substream).runtime;pcm_stream_lock_irqsave(substream);if snd_pcm_running(substream)!=0&&snd_pcm_update_hw_ptr(substream)>=0{(*(*runtime).status).hw_ptr%=(*runtime).buffer_size;}else{(*(*runtime).status).hw_ptr=0;(*runtime).hw_ptr_wrap=0;}pcm_stream_unlock_irqrestore(substream);0}
unsafe extern "C" fn snd_pcm_lib_ioctl_channel_info(substream:*mut snd_pcm_substream,arg:*mut c_void)->c_int{let info=arg as *mut snd_pcm_channel_info;let runtime=(*substream).runtime;if ((*runtime).info&SNDRV_PCM_INFO_MMAP)==0{(*info).offset=-1;return 0;}let width=snd_pcm_format_physical_width((*runtime).format);if width<0{return width;}(*info).offset=0;match (*runtime).access{SNDRV_PCM_ACCESS_MMAP_INTERLEAVED|SNDRV_PCM_ACCESS_RW_INTERLEAVED=>{(*info).first=(*info).channel*width as c_uint;(*info).step=(*runtime).channels*width as c_uint;}SNDRV_PCM_ACCESS_MMAP_NONINTERLEAVED|SNDRV_PCM_ACCESS_RW_NONINTERLEAVED=>{let size=(*runtime).dma_bytes/(*runtime).channels as usize;(*info).first=(*info).channel*size as c_uint*8;(*info).step=width as c_uint;} _=>{snd_BUG();}}0}
unsafe extern "C" fn snd_pcm_lib_ioctl_fifo_size(substream:*mut snd_pcm_substream,arg:*mut c_void)->c_int{let params=arg as *mut snd_pcm_hw_params;(*params).fifo_size=(*(*substream).runtime).hw.fifo_size;if ((*(*substream).runtime).hw.info&SNDRV_PCM_INFO_FIFO_IN_FRAMES)==0{let format=params_format(params);let channels=params_channels(params);let frame_size=snd_pcm_format_size(format,channels);if frame_size>0{(*params).fifo_size/=frame_size as c_uint;}}0}
unsafe extern "C" fn snd_pcm_lib_ioctl_sync_id(substream:*mut snd_pcm_substream,arg:*mut c_void)->c_int{static ID:[u8;12]=[0xff;12];if (*(*substream).runtime).std_sync_id!=0{snd_pcm_set_sync_per_card(substream,arg as *mut snd_pcm_hw_params,ID.as_ptr(),ID.len() as c_uint);}0}
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_lib_ioctl(substream:*mut snd_pcm_substream,cmd:c_uint,arg:*mut c_void)->c_int{match cmd{SNDRV_PCM_IOCTL1_RESET=>snd_pcm_lib_ioctl_reset(substream,arg),SNDRV_PCM_IOCTL1_CHANNEL_INFO=>snd_pcm_lib_ioctl_channel_info(substream,arg),SNDRV_PCM_IOCTL1_FIFO_SIZE=>snd_pcm_lib_ioctl_fifo_size(substream,arg),SNDRV_PCM_IOCTL1_SYNC_ID=>snd_pcm_lib_ioctl_sync_id(substream,arg),_=>-ENXIO}}

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_period_elapsed_under_stream_lock(substream:*mut snd_pcm_substream){if PCM_RUNTIME_CHECK(substream)!=0{return;}let runtime=(*substream).runtime;if snd_pcm_running(substream)==0||snd_pcm_update_hw_ptr0(substream,1)<0{}else{#[cfg(CONFIG_SND_PCM_TIMER)]{if (*substream).timer_running!=0{snd_timer_interrupt((*substream).timer,1);}}}snd_kill_fasync((*runtime).fasync,SIGIO,POLL_IN);}
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_period_elapsed(substream:*mut snd_pcm_substream){if snd_BUG_ON(substream.is_null())!=0{return;}pcm_stream_lock_irqsave(substream);snd_pcm_period_elapsed_under_stream_lock(substream);pcm_stream_unlock_irqrestore(substream);}

unsafe fn wait_for_avail(substream:*mut snd_pcm_substream,availp:*mut snd_pcm_uframes_t)->c_int{let runtime=(*substream).runtime;let is_playback=((*substream).stream==SNDRV_PCM_STREAM_PLAYBACK) as c_int;let mut wait:wait_queue_entry_t=core::mem::zeroed();let mut err=0;let mut avail=0;let mut wait_time:c_long;init_waitqueue_entry(&mut wait,current);set_current_state(TASK_INTERRUPTIBLE);add_wait_queue(&mut (*runtime).tsleep,&mut wait);if (*runtime).no_period_wakeup!=0{wait_time=MAX_SCHEDULE_TIMEOUT;}else{if (*substream).wait_time!=0{wait_time=(*substream).wait_time;}else{wait_time=100;if (*runtime).rate!=0{let t=(*runtime).buffer_size as c_long*1100/(*runtime).rate as c_long;wait_time=core::cmp::max(t,wait_time);}}wait_time=msecs_to_jiffies(wait_time);}loop{if signal_pending(current)!=0{err=-ERESTARTSYS;break;}avail=snd_pcm_avail(substream);if avail>=(*runtime).twake{break;}snd_pcm_stream_unlock_irq(substream);let tout=schedule_timeout(wait_time);snd_pcm_stream_lock_irq(substream);set_current_state(TASK_INTERRUPTIBLE);match (*runtime).state{SNDRV_PCM_STATE_SUSPENDED=>{err=-ESTRPIPE;break;}SNDRV_PCM_STATE_XRUN=>{err=-EPIPE;break;}SNDRV_PCM_STATE_DRAINING=>{if is_playback!=0{err=-EPIPE;}else{avail=0;}break;}SNDRV_PCM_STATE_OPEN|SNDRV_PCM_STATE_SETUP|SNDRV_PCM_STATE_DISCONNECTED=>{err=-EBADFD;break;}SNDRV_PCM_STATE_PAUSED=>continue,_=>{}}if tout==0{pcm_dbg((*substream).pcm,c"%s timeout (DMA or IRQ trouble?)\n".as_ptr(),if is_playback!=0{c"playback write".as_ptr()}else{c"capture read".as_ptr()});err=-EIO;break;}}set_current_state(TASK_RUNNING);remove_wait_queue(&mut (*runtime).tsleep,&mut wait);*availp=avail;err}

type pcm_transfer_f = unsafe extern "C" fn(*mut snd_pcm_substream,c_int,c_ulong,*mut iov_iter,c_ulong)->c_int;
type pcm_copy_f = unsafe fn(*mut snd_pcm_substream,snd_pcm_uframes_t,*mut c_void,snd_pcm_uframes_t,snd_pcm_uframes_t,pcm_transfer_f,bool)->c_int;
unsafe fn get_dma_ptr(runtime:*mut snd_pcm_runtime,channel:c_int,hwoff:c_ulong)->*mut c_void{(*runtime).dma_area.add(hwoff as usize+channel as usize*((*runtime).dma_bytes/(*runtime).channels as usize)).cast()}
unsafe extern "C" fn default_write_copy(substream:*mut snd_pcm_substream,channel:c_int,hwoff:c_ulong,iter:*mut iov_iter,bytes:c_ulong)->c_int{if copy_from_iter(get_dma_ptr((*substream).runtime,channel,hwoff),bytes,iter)!=bytes{return -EFAULT;}0}
unsafe extern "C" fn fill_silence(substream:*mut snd_pcm_substream,channel:c_int,hwoff:c_ulong,_iter:*mut iov_iter,bytes:c_ulong)->c_int{let runtime=(*substream).runtime;if (*substream).stream!=SNDRV_PCM_STREAM_PLAYBACK{return 0;}if (*(*substream).ops).fill_silence.is_some(){return ((*(*substream).ops).fill_silence.unwrap())(substream,channel,hwoff,bytes);}snd_pcm_format_set_silence((*runtime).format,get_dma_ptr(runtime,channel,hwoff),bytes_to_samples(runtime,bytes));0}
unsafe extern "C" fn default_read_copy(substream:*mut snd_pcm_substream,channel:c_int,hwoff:c_ulong,iter:*mut iov_iter,bytes:c_ulong)->c_int{if copy_to_iter(get_dma_ptr((*substream).runtime,channel,hwoff),bytes,iter)!=bytes{return -EFAULT;}0}
unsafe fn do_transfer(substream:*mut snd_pcm_substream,c:c_int,hwoff:c_ulong,data:*mut c_void,bytes:c_ulong,transfer:pcm_transfer_f,in_kernel:bool)->c_int{let mut iter:iov_iter=core::mem::zeroed();let type_=if (*substream).stream==SNDRV_PCM_STREAM_PLAYBACK{ITER_SOURCE}else{ITER_DEST};if in_kernel{let mut kvec=kvec{iov_base:data,iov_len:bytes as usize};iov_iter_kvec(&mut iter,type_,&mut kvec,1,bytes);return transfer(substream,c,hwoff,&mut iter,bytes);}let err=import_ubuf(type_,data,bytes,&mut iter);if err!=0{return err;}transfer(substream,c,hwoff,&mut iter,bytes)}
unsafe fn interleaved_copy(substream:*mut snd_pcm_substream,mut hwoff:snd_pcm_uframes_t,data:*mut c_void,mut off:snd_pcm_uframes_t,mut frames:snd_pcm_uframes_t,transfer:pcm_transfer_f,in_kernel:bool)->c_int{let runtime=(*substream).runtime;hwoff=frames_to_bytes(runtime,hwoff);off=frames_to_bytes(runtime,off);frames=frames_to_bytes(runtime,frames);if data.is_null(){return fill_silence(substream,0,hwoff as c_ulong,ptr::null_mut(),frames as c_ulong);}do_transfer(substream,0,hwoff as c_ulong,(data as *mut u8).add(off as usize).cast(),frames as c_ulong,transfer,in_kernel)}
unsafe fn noninterleaved_copy(substream:*mut snd_pcm_substream,mut hwoff:snd_pcm_uframes_t,data:*mut c_void,mut off:snd_pcm_uframes_t,mut frames:snd_pcm_uframes_t,transfer:pcm_transfer_f,in_kernel:bool)->c_int{let runtime=(*substream).runtime;let channels=(*runtime).channels as c_int;let mut bufs=data as *mut *mut c_void;off=samples_to_bytes(runtime,off);frames=samples_to_bytes(runtime,frames);hwoff=samples_to_bytes(runtime,hwoff);for c in 0..channels{let err=if data.is_null()||(*bufs).is_null(){fill_silence(substream,c,hwoff as c_ulong,ptr::null_mut(),frames as c_ulong)}else{do_transfer(substream,c,hwoff as c_ulong,((*bufs) as *mut u8).add(off as usize).cast(),frames as c_ulong,transfer,in_kernel)};if err<0{return err;}bufs=bufs.add(1);}0}
unsafe fn fill_silence_frames(substream:*mut snd_pcm_substream,off:snd_pcm_uframes_t,frames:snd_pcm_uframes_t)->c_int{if (*(*substream).runtime).access==SNDRV_PCM_ACCESS_RW_INTERLEAVED||(*(*substream).runtime).access==SNDRV_PCM_ACCESS_MMAP_INTERLEAVED{interleaved_copy(substream,off,ptr::null_mut(),0,frames,fill_silence,true)}else{noninterleaved_copy(substream,off,ptr::null_mut(),0,frames,fill_silence,true)}}
unsafe fn pcm_sanity_check(substream:*mut snd_pcm_substream)->c_int{if PCM_RUNTIME_CHECK(substream)!=0{return -ENXIO;}let runtime=(*substream).runtime;if snd_BUG_ON((*(*substream).ops).copy.is_none()&&(*runtime).dma_area.is_null())!=0{return -EINVAL;}if (*runtime).state==SNDRV_PCM_STATE_OPEN{return -EBADFD;}0}
unsafe fn pcm_accessible_state(runtime:*mut snd_pcm_runtime)->c_int{match (*runtime).state{SNDRV_PCM_STATE_PREPARED|SNDRV_PCM_STATE_RUNNING|SNDRV_PCM_STATE_PAUSED=>0,SNDRV_PCM_STATE_XRUN=>-EPIPE,SNDRV_PCM_STATE_SUSPENDED=>-ESTRPIPE,_=>-EBADFD}}
#[no_mangle]
pub unsafe extern "C" fn pcm_lib_apply_appl_ptr(substream:*mut snd_pcm_substream,appl_ptr:snd_pcm_uframes_t)->c_int{let runtime=(*substream).runtime;let old_appl_ptr=(*(*runtime).control).appl_ptr;if old_appl_ptr==appl_ptr{return 0;}if appl_ptr>=(*runtime).boundary{return -EINVAL;}if ((*runtime).info&SNDRV_PCM_INFO_NO_REWINDS)!=0{let diff=appl_ptr as snd_pcm_sframes_t-old_appl_ptr as snd_pcm_sframes_t;if diff>=0{if diff>(*runtime).buffer_size as snd_pcm_sframes_t{return -EINVAL;}}else if (*runtime).boundary as snd_pcm_sframes_t+diff>(*runtime).buffer_size as snd_pcm_sframes_t{return -EINVAL;}}(*(*runtime).control).appl_ptr=appl_ptr;if (*(*substream).ops).ack.is_some(){let ret=((*(*substream).ops).ack.unwrap())(substream);if ret<0{(*(*runtime).control).appl_ptr=old_appl_ptr;if ret==-EPIPE{__snd_pcm_xrun(substream);}return ret;}}trace_applptr(substream,old_appl_ptr,appl_ptr);0}

#[no_mangle]
pub unsafe extern "C" fn __snd_pcm_lib_xfer(substream:*mut snd_pcm_substream,data:*mut c_void,interleaved:bool,mut size:snd_pcm_uframes_t,in_kernel:bool)->snd_pcm_sframes_t{let runtime=(*substream).runtime;let mut xfer=0;let mut offset=0;let mut avail;let writer:pcm_copy_f;let transfer:pcm_transfer_f;let mut err=pcm_sanity_check(substream);if err<0{return err as snd_pcm_sframes_t;}let is_playback=(*substream).stream==SNDRV_PCM_STREAM_PLAYBACK;if interleaved{if (*runtime).access!=SNDRV_PCM_ACCESS_RW_INTERLEAVED&&(*runtime).channels>1{return -EINVAL as snd_pcm_sframes_t;}writer=interleaved_copy;}else{if (*runtime).access!=SNDRV_PCM_ACCESS_RW_NONINTERLEAVED{return -EINVAL as snd_pcm_sframes_t;}writer=noninterleaved_copy;}if data.is_null(){if is_playback{transfer=fill_silence;}else{return -EINVAL as snd_pcm_sframes_t;}}else if (*(*substream).ops).copy.is_some(){transfer=(*(*substream).ops).copy.unwrap();}else{transfer=if is_playback{default_write_copy}else{default_read_copy};}if size==0{return 0;}let nonblock=((*substream).f_flags&O_NONBLOCK)!=0;snd_pcm_stream_lock_irq(substream);err=pcm_accessible_state(runtime);if err<0{goto_end_xfer(runtime,substream,xfer,err);return err as snd_pcm_sframes_t;}(*runtime).twake=if (*(*runtime).control).avail_min!=0{(*(*runtime).control).avail_min}else{1};if (*runtime).state==SNDRV_PCM_STATE_RUNNING{snd_pcm_update_hw_ptr(substream);}if !is_playback&&(*runtime).state==SNDRV_PCM_STATE_PREPARED&&size>=(*runtime).start_threshold{err=snd_pcm_start(substream);if err<0{goto_end_xfer(runtime,substream,xfer,err);return err as snd_pcm_sframes_t;}}avail=snd_pcm_avail(substream);while size>0{if avail==0{if !is_playback&&(*runtime).state==SNDRV_PCM_STATE_DRAINING{snd_pcm_stop(substream,SNDRV_PCM_STATE_SETUP);break;}if nonblock{err=-EAGAIN;break;}(*runtime).twake=core::cmp::min(size,if (*(*runtime).control).avail_min!=0{(*(*runtime).control).avail_min}else{1});err=wait_for_avail(substream,&mut avail);if err<0{break;}if avail==0{continue;}}let mut frames=if size>avail{avail}else{size};let mut appl_ptr=READ_ONCE((*(*runtime).control).appl_ptr);let appl_ofs=appl_ptr%(*runtime).buffer_size;let cont=(*runtime).buffer_size-appl_ofs;if frames>cont{frames=cont;}if snd_BUG_ON(frames==0)!=0{err=-EINVAL;break;}if atomic_inc_unless_negative(&mut (*runtime).buffer_accessing)==0{err=-EBUSY;break;}snd_pcm_stream_unlock_irq(substream);if !is_playback{snd_pcm_dma_buffer_sync(substream,SNDRV_DMA_SYNC_CPU);}err=writer(substream,appl_ofs,data,offset,frames,transfer,in_kernel);if is_playback{snd_pcm_dma_buffer_sync(substream,SNDRV_DMA_SYNC_DEVICE);}snd_pcm_stream_lock_irq(substream);atomic_dec(&mut (*runtime).buffer_accessing);if err<0{break;}err=pcm_accessible_state(runtime);if err<0{break;}appl_ptr+=frames;if appl_ptr>=(*runtime).boundary{appl_ptr-=(*runtime).boundary;}err=pcm_lib_apply_appl_ptr(substream,appl_ptr);if err<0{break;}offset+=frames;size-=frames;xfer+=frames;avail-=frames;if is_playback&&(*runtime).state==SNDRV_PCM_STATE_PREPARED&&snd_pcm_playback_hw_avail(runtime)>=(*runtime).start_threshold as snd_pcm_sframes_t{err=snd_pcm_start(substream);if err<0{break;}}}(*runtime).twake=0;if xfer>0&&err>=0{snd_pcm_update_state(substream,runtime);}snd_pcm_stream_unlock_irq(substream);if xfer>0{xfer as snd_pcm_sframes_t}else{err as snd_pcm_sframes_t}}
unsafe fn goto_end_xfer(runtime:*mut snd_pcm_runtime,substream:*mut snd_pcm_substream,_xfer:snd_pcm_uframes_t,_err:c_int){(*runtime).twake=0;snd_pcm_stream_unlock_irq(substream);}

#[no_mangle]
pub static snd_pcm_std_chmaps:[snd_pcm_chmap_elem;6]=[
    snd_pcm_chmap_elem{channels:1,map:[SNDRV_CHMAP_MONO,0,0,0,0,0,0,0]},
    snd_pcm_chmap_elem{channels:2,map:[SNDRV_CHMAP_FL,SNDRV_CHMAP_FR,0,0,0,0,0,0]},
    snd_pcm_chmap_elem{channels:4,map:[SNDRV_CHMAP_FL,SNDRV_CHMAP_FR,SNDRV_CHMAP_RL,SNDRV_CHMAP_RR,0,0,0,0]},
    snd_pcm_chmap_elem{channels:6,map:[SNDRV_CHMAP_FL,SNDRV_CHMAP_FR,SNDRV_CHMAP_RL,SNDRV_CHMAP_RR,SNDRV_CHMAP_FC,SNDRV_CHMAP_LFE,0,0]},
    snd_pcm_chmap_elem{channels:8,map:[SNDRV_CHMAP_FL,SNDRV_CHMAP_FR,SNDRV_CHMAP_RL,SNDRV_CHMAP_RR,SNDRV_CHMAP_FC,SNDRV_CHMAP_LFE,SNDRV_CHMAP_SL,SNDRV_CHMAP_SR]},
    snd_pcm_chmap_elem{channels:0,map:[0;8]},
];
#[no_mangle]
pub static snd_pcm_alt_chmaps:[snd_pcm_chmap_elem;6]=[
    snd_pcm_chmap_elem{channels:1,map:[SNDRV_CHMAP_MONO,0,0,0,0,0,0,0]},
    snd_pcm_chmap_elem{channels:2,map:[SNDRV_CHMAP_FL,SNDRV_CHMAP_FR,0,0,0,0,0,0]},
    snd_pcm_chmap_elem{channels:4,map:[SNDRV_CHMAP_FL,SNDRV_CHMAP_FR,SNDRV_CHMAP_RL,SNDRV_CHMAP_RR,0,0,0,0]},
    snd_pcm_chmap_elem{channels:6,map:[SNDRV_CHMAP_FL,SNDRV_CHMAP_FR,SNDRV_CHMAP_FC,SNDRV_CHMAP_LFE,SNDRV_CHMAP_RL,SNDRV_CHMAP_RR,0,0]},
    snd_pcm_chmap_elem{channels:8,map:[SNDRV_CHMAP_FL,SNDRV_CHMAP_FR,SNDRV_CHMAP_FC,SNDRV_CHMAP_LFE,SNDRV_CHMAP_RL,SNDRV_CHMAP_RR,SNDRV_CHMAP_SL,SNDRV_CHMAP_SR]},
    snd_pcm_chmap_elem{channels:0,map:[0;8]},
];

unsafe fn valid_chmap_channels(info:*const snd_pcm_chmap,ch:c_int)->bool{if ch>(*info).max_channels{return false;}(*info).channel_mask==0||((*info).channel_mask&(1u32<<ch))!=0}
unsafe extern "C" fn pcm_chmap_ctl_info(kcontrol:*mut snd_kcontrol,uinfo:*mut snd_ctl_elem_info)->c_int{let info=snd_kcontrol_chip(kcontrol) as *mut snd_pcm_chmap;(*uinfo).type_=SNDRV_CTL_ELEM_TYPE_INTEGER;(*uinfo).count=(*info).max_channels as c_uint;(*uinfo).value.integer.min=0;(*uinfo).value.integer.max=SNDRV_CHMAP_LAST as c_long;0}
unsafe extern "C" fn pcm_chmap_ctl_get(kcontrol:*mut snd_kcontrol,ucontrol:*mut snd_ctl_elem_value)->c_int{let info=snd_kcontrol_chip(kcontrol) as *mut snd_pcm_chmap;let idx=snd_ctl_get_ioffidx(kcontrol,&mut (*ucontrol).id);let mut map=(*info).chmap;if map.is_null(){return -EINVAL;}let substream=snd_pcm_chmap_substream(info,idx);if substream.is_null(){return -ENODEV;}memset((*ucontrol).value.integer.value.as_mut_ptr().cast(),0,core::mem::size_of::<c_long>()*(*info).max_channels as usize);if (*substream).runtime.is_null(){return 0;}while (*map).channels!=0{if (*map).channels==(*(*substream).runtime).channels as c_int&&valid_chmap_channels(info,(*map).channels){for i in 0..(*map).channels as usize{(*ucontrol).value.integer.value[i]=(*map).map[i] as c_long;}return 0;}map=map.add(1);} -EINVAL}
unsafe extern "C" fn pcm_chmap_ctl_tlv(kcontrol:*mut snd_kcontrol,_op_flag:c_int,mut size:c_uint,tlv:*mut c_uint)->c_int{let info=snd_kcontrol_chip(kcontrol) as *mut snd_pcm_chmap;let mut map=(*info).chmap;let mut dst;let mut count:c_int=0;if map.is_null(){return -EINVAL;}if size<8{return -ENOMEM;}if put_user(SNDRV_CTL_TLVT_CONTAINER,tlv)!=0{return -EFAULT;}size-=8;dst=tlv.add(2);while (*map).channels!=0{let chs_bytes=(*map).channels as c_uint*4;if !valid_chmap_channels(info,(*map).channels){map=map.add(1);continue;}if size<8{return -ENOMEM;}if put_user(SNDRV_CTL_TLVT_CHMAP_FIXED,dst)!=0||put_user(chs_bytes,dst.add(1))!=0{return -EFAULT;}dst=dst.add(2);size-=8;count+=8;if size<chs_bytes{return -ENOMEM;}size-=chs_bytes;count+=chs_bytes as c_int;for c in 0..(*map).channels as usize{if put_user((*map).map[c],dst)!=0{return -EFAULT;}dst=dst.add(1);}map=map.add(1);}if put_user(count as c_uint,tlv.add(1))!=0{return -EFAULT;}0}
unsafe extern "C" fn pcm_chmap_ctl_private_free(kcontrol:*mut snd_kcontrol){let info=snd_kcontrol_chip(kcontrol) as *mut snd_pcm_chmap;(*(*info).pcm).streams[(*info).stream as usize].chmap_kctl=ptr::null_mut();kfree(info.cast());}
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_add_chmap_ctls(pcm:*mut snd_pcm,stream:c_int,chmap:*const snd_pcm_chmap_elem,max_channels:c_int,private_value:c_ulong,info_ret:*mut *mut snd_pcm_chmap)->c_int{let mut knew:snd_kcontrol_new=core::mem::zeroed();knew.iface=SNDRV_CTL_ELEM_IFACE_PCM;knew.access=SNDRV_CTL_ELEM_ACCESS_READ|SNDRV_CTL_ELEM_ACCESS_VOLATILE|SNDRV_CTL_ELEM_ACCESS_TLV_READ|SNDRV_CTL_ELEM_ACCESS_TLV_CALLBACK;knew.info=Some(pcm_chmap_ctl_info);knew.get=Some(pcm_chmap_ctl_get);knew.tlv.c=Some(pcm_chmap_ctl_tlv);if WARN_ON(!(*pcm).streams[stream as usize].chmap_kctl.is_null())!=0{return -EBUSY;}let info=kzalloc(core::mem::size_of::<snd_pcm_chmap>(),GFP_KERNEL) as *mut snd_pcm_chmap;if info.is_null(){return -ENOMEM;}(*info).pcm=pcm;(*info).stream=stream;(*info).chmap=chmap;(*info).max_channels=max_channels;knew.name=if stream==SNDRV_PCM_STREAM_PLAYBACK{c"Playback Channel Map".as_ptr()}else{c"Capture Channel Map".as_ptr()};knew.device=(*pcm).device;knew.count=(*pcm).streams[stream as usize].substream_count;knew.private_value=private_value;(*info).kctl=snd_ctl_new1(&mut knew,info.cast());if (*info).kctl.is_null(){kfree(info.cast());return -ENOMEM;}(*(*info).kctl).private_free=Some(pcm_chmap_ctl_private_free);let err=snd_ctl_add((*pcm).card,(*info).kctl);if err<0{return err;}(*pcm).streams[stream as usize].chmap_kctl=(*info).kctl;if !info_ret.is_null(){*info_ret=info;}0}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
