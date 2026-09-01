// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  Routines for control of GF1 chip (PCM things)
 *
 *  InterWave chips supports interleaved DMA, but this feature isn't used in
 *  this code.
 *
 *  This code emulates autoinit DMA transfer for playback, recording by GF1
 *  chip doesn't support autoinit DMA.
 */

use crate::*;

/* maximum rate */

pub const SNDRV_GF1_PCM_RATE: u32 = 48000;

pub const SNDRV_GF1_PCM_PFLG_NONE: u16 = 0;
pub const SNDRV_GF1_PCM_PFLG_ACTIVE: u16 = 1 << 0;
pub const SNDRV_GF1_PCM_PFLG_NEUTRAL: u16 = 2 << 0;

#[repr(C)]
pub struct gus_pcm_private {
    pub gus: *mut snd_gus_card,
    pub substream: *mut snd_pcm_substream,
    pub lock: spinlock_t,
    pub voices: u32,
    pub pvoices: [*mut snd_gus_voice; 2],
    pub memory: u32,
    pub flags: u16,
    pub voice_ctrl: u8,
    pub ramp_ctrl: u8,
    pub bpos: u32,
    pub blocks: u32,
    pub block_size: u32,
    pub dma_size: u32,
    pub sleep: wait_queue_head_t,
    pub dma_count: atomic_t,
    pub final_volume: i32,
}

unsafe extern "C" fn snd_gf1_pcm_block_change_ack(
    _gus: *mut snd_gus_card,
    private_data: *mut core::ffi::c_void,
) {
    let pcmp = private_data as *mut gus_pcm_private;

    if !pcmp.is_null() {
        atomic_dec(core::ptr::addr_of_mut!((*pcmp).dma_count));
        wake_up(core::ptr::addr_of_mut!((*pcmp).sleep));
    }
}

unsafe extern "C" fn snd_gf1_pcm_block_change(
    substream: *mut snd_pcm_substream,
    mut offset: u32,
    addr: u32,
    mut count: u32,
) -> i32 {
    let mut block: snd_gf1_dma_block = core::mem::zeroed();
    let runtime = (*substream).runtime;
    let pcmp = (*runtime).private_data as *mut gus_pcm_private;

    count += offset & 31;
    offset &= !31;
    block.cmd = SNDRV_GF1_DMA_IRQ;
    if snd_pcm_format_unsigned((*runtime).format) != 0 {
        block.cmd |= SNDRV_GF1_DMA_UNSIGNED;
    }
    if snd_pcm_format_width((*runtime).format) == 16 {
        block.cmd |= SNDRV_GF1_DMA_16BIT;
    }
    block.addr = addr & !31;
    block.buffer = (*runtime).dma_area.add(offset as usize);
    block.buf_addr = (*runtime).dma_addr + offset as dma_addr_t;
    block.count = count;
    block.private_data = pcmp as *mut core::ffi::c_void;
    block.ack = Some(snd_gf1_pcm_block_change_ack);
    if snd_gf1_dma_transfer_block((*pcmp).gus, core::ptr::addr_of_mut!(block), 0, 0) == 0 {
        atomic_inc(core::ptr::addr_of_mut!((*pcmp).dma_count));
    }
    0
}

unsafe extern "C" fn snd_gf1_pcm_trigger_up(substream: *mut snd_pcm_substream) {
    let runtime = (*substream).runtime;
    let pcmp = (*runtime).private_data as *mut gus_pcm_private;
    let gus = (*pcmp).gus;
    let mut voice_ctrl: u8;
    let mut ramp_ctrl: u8;
    let rate: u16;
    let mut curr: u32;
    let mut begin: u32;
    let mut end: u32;
    let mut vol: u16;
    let mut pan: u8;
    let mut voice: u32;

    {
        let _flags = spin_lock_irqsave(core::ptr::addr_of_mut!((*pcmp).lock));
        if ((*pcmp).flags & SNDRV_GF1_PCM_PFLG_ACTIVE) != 0 {
            spin_unlock_irqrestore(core::ptr::addr_of_mut!((*pcmp).lock), _flags);
            return;
        }
        (*pcmp).flags |= SNDRV_GF1_PCM_PFLG_ACTIVE;
        (*pcmp).final_volume = 0;
        spin_unlock_irqrestore(core::ptr::addr_of_mut!((*pcmp).lock), _flags);
    }
    rate = snd_gf1_translate_freq(gus, (*runtime).rate << 4);
    /* enable WAVE IRQ */
    voice_ctrl = if snd_pcm_format_width((*runtime).format) == 16 { 0x24 } else { 0x20 };
    /* enable RAMP IRQ + rollover */
    ramp_ctrl = 0x24;
    if (*pcmp).blocks == 1 {
        voice_ctrl |= 0x08; /* loop enable */
        ramp_ctrl &= !0x04; /* disable rollover */
    }
    voice = 0;
    while voice < (*pcmp).voices {
        begin = (*pcmp).memory + voice * ((*pcmp).dma_size / (*runtime).channels);
        curr = begin + ((*pcmp).bpos * (*pcmp).block_size) / (*runtime).channels;
        end = curr + ((*pcmp).block_size / (*runtime).channels);
        end -= if snd_pcm_format_width((*runtime).format) == 16 { 2 } else { 1 };
        pan = if (*runtime).channels == 2 { if voice == 0 { 1 } else { 14 } } else { 8 };
        vol = if voice == 0 { (*gus).gf1.pcm_volume_level_left } else { (*gus).gf1.pcm_volume_level_right };
        let _flags = spin_lock_irqsave(core::ptr::addr_of_mut!((*gus).reg_lock));
        snd_gf1_select_voice(gus, (*(*pcmp).pvoices[voice as usize]).number);
        snd_gf1_write8(gus, SNDRV_GF1_VB_PAN, pan);
        snd_gf1_write16(gus, SNDRV_GF1_VW_FREQUENCY, rate);
        snd_gf1_write_addr(gus, SNDRV_GF1_VA_START, begin << 4, (voice_ctrl & 4) as i32);
        snd_gf1_write_addr(gus, SNDRV_GF1_VA_END, end << 4, (voice_ctrl & 4) as i32);
        snd_gf1_write_addr(gus, SNDRV_GF1_VA_CURRENT, curr << 4, (voice_ctrl & 4) as i32);
        snd_gf1_write16(gus, SNDRV_GF1_VW_VOLUME, SNDRV_GF1_MIN_VOLUME << 4);
        snd_gf1_write8(gus, SNDRV_GF1_VB_VOLUME_RATE, 0x2f);
        snd_gf1_write8(gus, SNDRV_GF1_VB_VOLUME_START, SNDRV_GF1_MIN_OFFSET);
        snd_gf1_write8(gus, SNDRV_GF1_VB_VOLUME_END, (vol >> 8) as u8);
        snd_gf1_write8(gus, SNDRV_GF1_VB_VOLUME_CONTROL, ramp_ctrl);
        if (*gus).gf1.enh_mode == 0 {
            snd_gf1_delay(gus);
            snd_gf1_write8(gus, SNDRV_GF1_VB_VOLUME_CONTROL, ramp_ctrl);
        }
        spin_unlock_irqrestore(core::ptr::addr_of_mut!((*gus).reg_lock), _flags);
        voice += 1;
    }

    {
        let _flags = spin_lock_irqsave(core::ptr::addr_of_mut!((*gus).reg_lock));
        voice = 0;
        while voice < (*pcmp).voices {
            snd_gf1_select_voice(gus, (*(*pcmp).pvoices[voice as usize]).number);
            if (*gus).gf1.enh_mode != 0 {
                snd_gf1_write8(gus, SNDRV_GF1_VB_MODE, 0x00); /* deactivate voice */
            }
            snd_gf1_write8(gus, SNDRV_GF1_VB_ADDRESS_CONTROL, voice_ctrl);
            voice_ctrl &= !0x20;
            voice += 1;
        }
        voice_ctrl |= 0x20;
        if (*gus).gf1.enh_mode == 0 {
            snd_gf1_delay(gus);
            voice = 0;
            while voice < (*pcmp).voices {
                snd_gf1_select_voice(gus, (*(*pcmp).pvoices[voice as usize]).number);
                snd_gf1_write8(gus, SNDRV_GF1_VB_ADDRESS_CONTROL, voice_ctrl);
                voice_ctrl &= !0x20; /* disable IRQ for next voice */
                voice += 1;
            }
        }
        spin_unlock_irqrestore(core::ptr::addr_of_mut!((*gus).reg_lock), _flags);
    }
}

unsafe extern "C" fn snd_gf1_pcm_interrupt_wave(gus_arg: *mut snd_gus_card, pvoice: *mut snd_gus_voice) {
    let mut gus = gus_arg;
    let pcmp: *mut gus_pcm_private;
    let runtime: *mut snd_pcm_runtime;
    let mut voice_ctrl: u8;
    let mut ramp_ctrl: u8;
    let mut idx: u32;
    let mut end: u32;
    let step: u32;

    if (*pvoice).private_data.is_null() {
        dev_dbg((*(*gus).card).dev, c"%s: unknown wave irq?\n".as_ptr(), c"__func__".as_ptr());
        snd_gf1_smart_stop_voice(gus, (*pvoice).number);
        return;
    }
    pcmp = (*pvoice).private_data as *mut gus_pcm_private;
    if pcmp.is_null() {
        dev_dbg((*(*gus).card).dev, c"%s: unknown wave irq?\n".as_ptr(), c"__func__".as_ptr());
        snd_gf1_smart_stop_voice(gus, (*pvoice).number);
        return;
    }
    gus = (*pcmp).gus;
    runtime = (*(*pcmp).substream).runtime;

    {
        spin_lock(core::ptr::addr_of_mut!((*gus).reg_lock));
        snd_gf1_select_voice(gus, (*pvoice).number);
        voice_ctrl = snd_gf1_read8(gus, SNDRV_GF1_VB_ADDRESS_CONTROL) & !0x8b;
        ramp_ctrl = (snd_gf1_read8(gus, SNDRV_GF1_VB_VOLUME_CONTROL) & !0xa4) | 0x03;
        /*
         * Disabled C debug block (#if 0) omitted from executable Rust.
         */
        (*pcmp).bpos += 1;
        (*pcmp).bpos %= (*pcmp).blocks;
        if (*pcmp).bpos + 1 >= (*pcmp).blocks {
            voice_ctrl |= 0x08; /* enable loop */
        } else {
            ramp_ctrl |= 0x04; /* enable rollover */
        }
        end = (*pcmp).memory + ((((*pcmp).bpos + 1) * (*pcmp).block_size) / (*runtime).channels);
        end -= if (voice_ctrl & 4) != 0 { 2 } else { 1 };
        step = (*pcmp).dma_size / (*runtime).channels;
        voice_ctrl |= 0x20;
        if (*pcmp).final_volume == 0 {
            ramp_ctrl |= 0x20;
            ramp_ctrl &= !0x03;
        }
        idx = 0;
        while idx < (*pcmp).voices {
            snd_gf1_select_voice(gus, (*(*pcmp).pvoices[idx as usize]).number);
            snd_gf1_write_addr(gus, SNDRV_GF1_VA_END, end << 4, (voice_ctrl & 4) as i32);
            snd_gf1_write8(gus, SNDRV_GF1_VB_ADDRESS_CONTROL, voice_ctrl);
            snd_gf1_write8(gus, SNDRV_GF1_VB_VOLUME_CONTROL, ramp_ctrl);
            voice_ctrl &= !0x20;
            idx += 1;
            end += step;
        }
        if (*gus).gf1.enh_mode == 0 {
            snd_gf1_delay(gus);
            voice_ctrl |= 0x20;
            idx = 0;
            while idx < (*pcmp).voices {
                snd_gf1_select_voice(gus, (*(*pcmp).pvoices[idx as usize]).number);
                snd_gf1_write8(gus, SNDRV_GF1_VB_ADDRESS_CONTROL, voice_ctrl);
                snd_gf1_write8(gus, SNDRV_GF1_VB_VOLUME_CONTROL, ramp_ctrl);
                voice_ctrl &= !0x20;
                idx += 1;
            }
        }
        spin_unlock(core::ptr::addr_of_mut!((*gus).reg_lock));
    }

    snd_pcm_period_elapsed((*pcmp).substream);
    /*
     * Disabled C mmap block (#if 0) omitted from executable Rust.
     */
}

unsafe extern "C" fn snd_gf1_pcm_interrupt_volume(gus: *mut snd_gus_card, pvoice: *mut snd_gus_voice) {
    let mut vol: u16;
    let cvoice: i32;
    let pcmp = (*pvoice).private_data as *mut gus_pcm_private;

    /* stop ramp, but leave rollover bit untouched */
    spin_lock(core::ptr::addr_of_mut!((*gus).reg_lock));
    snd_gf1_select_voice(gus, (*pvoice).number);
    snd_gf1_ctrl_stop(gus, SNDRV_GF1_VB_VOLUME_CONTROL);
    spin_unlock(core::ptr::addr_of_mut!((*gus).reg_lock));
    if pcmp.is_null() {
        return;
    }
    /* are we active? */
    if ((*pcmp).flags & SNDRV_GF1_PCM_PFLG_ACTIVE) == 0 {
        return;
    }
    /* load real volume - better precision */
    cvoice = if (*pcmp).pvoices[0] == pvoice { 0 } else { 1 };
    if (*pcmp).substream.is_null() {
        return;
    }
    vol = if cvoice == 0 { (*gus).gf1.pcm_volume_level_left } else { (*gus).gf1.pcm_volume_level_right };
    spin_lock(core::ptr::addr_of_mut!((*gus).reg_lock));
    snd_gf1_select_voice(gus, (*pvoice).number);
    snd_gf1_write16(gus, SNDRV_GF1_VW_VOLUME, vol);
    spin_unlock(core::ptr::addr_of_mut!((*gus).reg_lock));
    (*pcmp).final_volume = 1;
}

unsafe extern "C" fn snd_gf1_pcm_volume_change(_gus: *mut snd_gus_card) {}

unsafe extern "C" fn snd_gf1_pcm_poke_block(
    gus: *mut snd_gus_card,
    mut buf: *mut u8,
    mut pos: u32,
    mut count: u32,
    mut w16: i32,
    mut invert: i32,
) -> i32 {
    let mut len: u32;

    while count > 0 {
        len = count;
        if len > 512 {
            /* limit, to allow IRQ */
            len = 512;
        }
        count -= len;
        if (*gus).interwave != 0 {
            let _flags = spin_lock_irqsave(core::ptr::addr_of_mut!((*gus).reg_lock));
            snd_gf1_write8(gus, SNDRV_GF1_GB_MEMORY_CONTROL, (0x01 | if invert != 0 { 0x08 } else { 0x00 }) as u8);
            snd_gf1_dram_addr(gus, pos);
            if w16 != 0 {
                outb(SNDRV_GF1_GW_DRAM_IO16, GUSP(gus, GF1REGSEL));
                outsw(GUSP(gus, GF1DATALOW), buf as *const core::ffi::c_void, len >> 1);
            } else {
                outsb(GUSP(gus, DRAM), buf as *const core::ffi::c_void, len);
            }
            spin_unlock_irqrestore(core::ptr::addr_of_mut!((*gus).reg_lock), _flags);
            buf = buf.add(512);
            pos += 512;
        } else {
            invert = if invert != 0 { 0x80 } else { 0x00 };
            if w16 != 0 {
                len >>= 1;
                while len != 0 {
                    len -= 1;
                    snd_gf1_poke(gus, pos, *buf);
                    pos += 1;
                    buf = buf.add(1);
                    snd_gf1_poke(gus, pos, *buf ^ invert as u8);
                    pos += 1;
                    buf = buf.add(1);
                }
            } else {
                while len != 0 {
                    len -= 1;
                    snd_gf1_poke(gus, pos, *buf ^ invert as u8);
                    pos += 1;
                    buf = buf.add(1);
                }
            }
        }
        if count > 0 && in_interrupt() == 0 {
            schedule_timeout_interruptible(1);
            if signal_pending(current) != 0 {
                return -EAGAIN;
            }
        }
    }
    0
}

unsafe extern "C" fn get_bpos(pcmp: *mut gus_pcm_private, voice: i32, pos: u32, len: u32) -> i32 {
    let bpos = pos + ((voice as u32) * ((*pcmp).dma_size / 2));
    if snd_BUG_ON((bpos > (*pcmp).dma_size) as i32) != 0 {
        return -EIO;
    }
    if snd_BUG_ON((bpos + len > (*pcmp).dma_size) as i32) != 0 {
        return -EIO;
    }
    bpos as i32
}

unsafe extern "C" fn playback_copy_ack(substream: *mut snd_pcm_substream, bpos: u32, len: u32) -> i32 {
    let runtime = (*substream).runtime;
    let pcmp = (*runtime).private_data as *mut gus_pcm_private;
    let gus = (*pcmp).gus;
    let w16: i32;
    let invert: i32;

    if len > 32 {
        return snd_gf1_pcm_block_change(substream, bpos, (*pcmp).memory + bpos, len);
    }

    w16 = (snd_pcm_format_width((*runtime).format) == 16) as i32;
    invert = snd_pcm_format_unsigned((*runtime).format);
    snd_gf1_pcm_poke_block(gus, (*runtime).dma_area.add(bpos as usize), (*pcmp).memory + bpos, len, w16, invert)
}

unsafe extern "C" fn snd_gf1_pcm_playback_copy(
    substream: *mut snd_pcm_substream,
    voice: i32,
    pos: core::ffi::c_ulong,
    src: *mut iov_iter,
    count: core::ffi::c_ulong,
) -> i32 {
    let runtime = (*substream).runtime;
    let pcmp = (*runtime).private_data as *mut gus_pcm_private;
    let len: u32 = count as u32;
    let bpos: i32;

    bpos = get_bpos(pcmp, voice, pos as u32, len);
    if bpos < 0 {
        return bpos;
    }
    if copy_from_iter((*runtime).dma_area.add(bpos as usize), len as usize, src) != len as usize {
        return -EFAULT;
    }
    playback_copy_ack(substream, bpos as u32, len)
}

unsafe extern "C" fn snd_gf1_pcm_playback_silence(
    substream: *mut snd_pcm_substream,
    voice: i32,
    pos: core::ffi::c_ulong,
    count: core::ffi::c_ulong,
) -> i32 {
    let runtime = (*substream).runtime;
    let pcmp = (*runtime).private_data as *mut gus_pcm_private;
    let len: u32 = count as u32;
    let bpos: i32;

    bpos = get_bpos(pcmp, voice, pos as u32, len);
    if bpos < 0 {
        return bpos;
    }
    snd_pcm_format_set_silence((*runtime).format, (*runtime).dma_area.add(bpos as usize), bytes_to_samples(runtime, count));
    playback_copy_ack(substream, bpos as u32, len)
}

unsafe extern "C" fn snd_gf1_pcm_playback_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> i32 {
    let gus = snd_pcm_substream_chip(substream) as *mut snd_gus_card;
    let runtime = (*substream).runtime;
    let pcmp = (*runtime).private_data as *mut gus_pcm_private;

    if (*runtime).buffer_changed != 0 {
        let block: *mut snd_gf1_mem_block;
        if (*pcmp).memory > 0 {
            snd_gf1_mem_free(core::ptr::addr_of_mut!((*gus).gf1.mem_alloc), (*pcmp).memory);
            (*pcmp).memory = 0;
        }
        block = snd_gf1_mem_alloc(
            core::ptr::addr_of_mut!((*gus).gf1.mem_alloc),
            SNDRV_GF1_MEM_OWNER_DRIVER,
            c"GF1 PCM".as_ptr(),
            (*runtime).dma_bytes,
            1,
            32,
            core::ptr::null_mut(),
        );
        if block.is_null() {
            return -ENOMEM;
        }
        (*pcmp).memory = (*block).ptr;
    }
    (*pcmp).voices = params_channels(hw_params);
    if (*pcmp).pvoices[0].is_null() {
        (*pcmp).pvoices[0] = snd_gf1_alloc_voice((*pcmp).gus, SNDRV_GF1_VOICE_TYPE_PCM, 0, 0);
        if (*pcmp).pvoices[0].is_null() {
            return -ENOMEM;
        }
        (*(*pcmp).pvoices[0]).handler_wave = Some(snd_gf1_pcm_interrupt_wave);
        (*(*pcmp).pvoices[0]).handler_volume = Some(snd_gf1_pcm_interrupt_volume);
        (*(*pcmp).pvoices[0]).volume_change = Some(snd_gf1_pcm_volume_change);
        (*(*pcmp).pvoices[0]).private_data = pcmp as *mut core::ffi::c_void;
    }
    if (*pcmp).voices > 1 && (*pcmp).pvoices[1].is_null() {
        (*pcmp).pvoices[1] = snd_gf1_alloc_voice((*pcmp).gus, SNDRV_GF1_VOICE_TYPE_PCM, 0, 0);
        if (*pcmp).pvoices[1].is_null() {
            return -ENOMEM;
        }
        (*(*pcmp).pvoices[1]).handler_wave = Some(snd_gf1_pcm_interrupt_wave);
        (*(*pcmp).pvoices[1]).handler_volume = Some(snd_gf1_pcm_interrupt_volume);
        (*(*pcmp).pvoices[1]).volume_change = Some(snd_gf1_pcm_volume_change);
        (*(*pcmp).pvoices[1]).private_data = pcmp as *mut core::ffi::c_void;
    } else if (*pcmp).voices == 1 {
        if !(*pcmp).pvoices[1].is_null() {
            snd_gf1_free_voice((*pcmp).gus, (*pcmp).pvoices[1]);
            (*pcmp).pvoices[1] = core::ptr::null_mut();
        }
    }
    0
}

unsafe extern "C" fn snd_gf1_pcm_playback_hw_free(substream: *mut snd_pcm_substream) -> i32 {
    let runtime = (*substream).runtime;
    let pcmp = (*runtime).private_data as *mut gus_pcm_private;

    if !(*pcmp).pvoices[0].is_null() {
        snd_gf1_free_voice((*pcmp).gus, (*pcmp).pvoices[0]);
        (*pcmp).pvoices[0] = core::ptr::null_mut();
    }
    if !(*pcmp).pvoices[1].is_null() {
        snd_gf1_free_voice((*pcmp).gus, (*pcmp).pvoices[1]);
        (*pcmp).pvoices[1] = core::ptr::null_mut();
    }
    if (*pcmp).memory > 0 {
        snd_gf1_mem_free(core::ptr::addr_of_mut!((*(*pcmp).gus).gf1.mem_alloc), (*pcmp).memory);
        (*pcmp).memory = 0;
    }
    0
}

unsafe extern "C" fn snd_gf1_pcm_playback_prepare(substream: *mut snd_pcm_substream) -> i32 {
    let pcmp = (*(*substream).runtime).private_data as *mut gus_pcm_private;

    (*pcmp).bpos = 0;
    (*pcmp).dma_size = snd_pcm_lib_buffer_bytes(substream);
    (*pcmp).block_size = snd_pcm_lib_period_bytes(substream);
    (*pcmp).blocks = (*pcmp).dma_size / (*pcmp).block_size;
    0
}

unsafe extern "C" fn snd_gf1_pcm_playback_trigger(substream: *mut snd_pcm_substream, cmd: i32) -> i32 {
    let gus = snd_pcm_substream_chip(substream) as *mut snd_gus_card;
    let runtime = (*substream).runtime;
    let pcmp = (*runtime).private_data as *mut gus_pcm_private;
    let mut voice: i32;

    if cmd == SNDRV_PCM_TRIGGER_START {
        snd_gf1_pcm_trigger_up(substream);
    } else if cmd == SNDRV_PCM_TRIGGER_STOP || cmd == SNDRV_PCM_TRIGGER_SUSPEND {
        spin_lock(core::ptr::addr_of_mut!((*pcmp).lock));
        (*pcmp).flags &= !SNDRV_GF1_PCM_PFLG_ACTIVE;
        spin_unlock(core::ptr::addr_of_mut!((*pcmp).lock));
        voice = (*(*pcmp).pvoices[0]).number;
        snd_gf1_stop_voices(gus, voice, voice);
        if !(*pcmp).pvoices[1].is_null() {
            voice = (*(*pcmp).pvoices[1]).number;
            snd_gf1_stop_voices(gus, voice, voice);
        }
    } else {
        return -EINVAL;
    }
    0
}

unsafe extern "C" fn snd_gf1_pcm_playback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let gus = snd_pcm_substream_chip(substream) as *mut snd_gus_card;
    let runtime = (*substream).runtime;
    let pcmp = (*runtime).private_data as *mut gus_pcm_private;
    let mut pos: u32;
    let voice_ctrl: u8;

    pos = 0;
    spin_lock(core::ptr::addr_of_mut!((*gus).reg_lock));
    if ((*pcmp).flags & SNDRV_GF1_PCM_PFLG_ACTIVE) != 0 {
        snd_gf1_select_voice(gus, (*(*pcmp).pvoices[0]).number);
        voice_ctrl = snd_gf1_read8(gus, SNDRV_GF1_VB_ADDRESS_CONTROL);
        pos = ((snd_gf1_read_addr(gus, SNDRV_GF1_VA_CURRENT, (voice_ctrl & 4) as i32) >> 4) - (*pcmp).memory) as u32;
        if (*(*substream).runtime).channels > 1 {
            pos <<= 1;
        }
        pos = bytes_to_frames(runtime, pos) as u32;
    }
    spin_unlock(core::ptr::addr_of_mut!((*gus).reg_lock));
    pos as snd_pcm_uframes_t
}

static clock: snd_ratnum = snd_ratnum {
    num: 9878400 / 16,
    den_min: 2,
    den_max: 257,
    den_step: 1,
};

static hw_constraints_clocks: snd_pcm_hw_constraint_ratnums = snd_pcm_hw_constraint_ratnums {
    nrats: 1,
    rats: core::ptr::addr_of!(clock),
};

unsafe extern "C" fn snd_gf1_pcm_capture_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> i32 {
    let gus = snd_pcm_substream_chip(substream) as *mut snd_gus_card;

    (*gus).c_dma_size = params_buffer_bytes(hw_params);
    (*gus).c_period_size = params_period_bytes(hw_params);
    (*gus).c_pos = 0;
    (*gus).gf1.pcm_rcntrl_reg = 0x21; /* IRQ at end, enable & start */
    if params_channels(hw_params) > 1 {
        (*gus).gf1.pcm_rcntrl_reg |= 2;
    }
    if (*gus).gf1.dma2 > 3 {
        (*gus).gf1.pcm_rcntrl_reg |= 4;
    }
    if snd_pcm_format_unsigned(params_format(hw_params)) != 0 {
        (*gus).gf1.pcm_rcntrl_reg |= 0x80;
    }
    0
}

unsafe extern "C" fn snd_gf1_pcm_capture_prepare(substream: *mut snd_pcm_substream) -> i32 {
    let gus = snd_pcm_substream_chip(substream) as *mut snd_gus_card;
    let runtime = (*substream).runtime;

    snd_gf1_i_write8(gus, SNDRV_GF1_GB_RECORD_RATE, ((*runtime).rate_den - 2) as u8);
    snd_gf1_i_write8(gus, SNDRV_GF1_GB_REC_DMA_CONTROL, 0); /* disable sampling */
    snd_gf1_i_look8(gus, SNDRV_GF1_GB_REC_DMA_CONTROL); /* Sampling Control Register */
    snd_dma_program((*gus).gf1.dma2, (*runtime).dma_addr, (*gus).c_period_size, DMA_MODE_READ);
    0
}

unsafe extern "C" fn snd_gf1_pcm_capture_trigger(substream: *mut snd_pcm_substream, cmd: i32) -> i32 {
    let gus = snd_pcm_substream_chip(substream) as *mut snd_gus_card;
    let val: i32;

    if cmd == SNDRV_PCM_TRIGGER_START {
        val = (*gus).gf1.pcm_rcntrl_reg as i32;
    } else if cmd == SNDRV_PCM_TRIGGER_STOP || cmd == SNDRV_PCM_TRIGGER_SUSPEND {
        val = 0;
    } else {
        return -EINVAL;
    }

    spin_lock(core::ptr::addr_of_mut!((*gus).reg_lock));
    snd_gf1_write8(gus, SNDRV_GF1_GB_REC_DMA_CONTROL, val as u8);
    snd_gf1_look8(gus, SNDRV_GF1_GB_REC_DMA_CONTROL);
    spin_unlock(core::ptr::addr_of_mut!((*gus).reg_lock));
    0
}

unsafe extern "C" fn snd_gf1_pcm_capture_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let gus = snd_pcm_substream_chip(substream) as *mut snd_gus_card;
    let mut pos = snd_dma_pointer((*gus).gf1.dma2, (*gus).c_period_size);
    pos = bytes_to_frames((*substream).runtime, ((*gus).c_pos + pos as u32) % (*gus).c_dma_size) as i32;
    pos as snd_pcm_uframes_t
}

unsafe extern "C" fn snd_gf1_pcm_interrupt_dma_read(gus: *mut snd_gus_card) {
    snd_gf1_i_write8(gus, SNDRV_GF1_GB_REC_DMA_CONTROL, 0); /* disable sampling */
    snd_gf1_i_look8(gus, SNDRV_GF1_GB_REC_DMA_CONTROL); /* Sampling Control Register */
    if !(*gus).pcm_cap_substream.is_null() {
        snd_gf1_pcm_capture_prepare((*gus).pcm_cap_substream);
        snd_gf1_pcm_capture_trigger((*gus).pcm_cap_substream, SNDRV_PCM_TRIGGER_START);
        (*gus).c_pos += (*gus).c_period_size;
        snd_pcm_period_elapsed((*gus).pcm_cap_substream);
    }
}

static snd_gf1_pcm_playback: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_NONINTERLEAVED,
    formats: SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_U16_LE,
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    rate_min: 5510,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 128 * 1024,
    period_bytes_min: 64,
    period_bytes_max: 128 * 1024,
    periods_min: 1,
    periods_max: 1024,
    fifo_size: 0,
};

static snd_gf1_pcm_capture: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_U8,
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_44100,
    rate_min: 5510,
    rate_max: 44100,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 128 * 1024,
    period_bytes_min: 64,
    period_bytes_max: 128 * 1024,
    periods_min: 1,
    periods_max: 1024,
    fifo_size: 0,
};

unsafe extern "C" fn snd_gf1_pcm_playback_free(runtime: *mut snd_pcm_runtime) {
    kfree((*runtime).private_data);
}

unsafe extern "C" fn snd_gf1_pcm_playback_open(substream: *mut snd_pcm_substream) -> i32 {
    let mut pcmp: *mut gus_pcm_private;
    let gus = snd_pcm_substream_chip(substream) as *mut snd_gus_card;
    let runtime = (*substream).runtime;
    let err: i32;

    pcmp = kzalloc(core::mem::size_of::<gus_pcm_private>(), GFP_KERNEL) as *mut gus_pcm_private;
    if pcmp.is_null() {
        return -ENOMEM;
    }
    (*pcmp).gus = gus;
    spin_lock_init(core::ptr::addr_of_mut!((*pcmp).lock));
    init_waitqueue_head(core::ptr::addr_of_mut!((*pcmp).sleep));
    atomic_set(core::ptr::addr_of_mut!((*pcmp).dma_count), 0);

    (*runtime).private_data = pcmp as *mut core::ffi::c_void;
    (*runtime).private_free = Some(snd_gf1_pcm_playback_free);

    /*
     * Disabled C debug block (#if 0) omitted from executable Rust.
     */
    err = snd_gf1_dma_init(gus);
    if err < 0 {
        return err;
    }
    (*pcmp).flags = SNDRV_GF1_PCM_PFLG_NONE;
    (*pcmp).substream = substream;
    (*runtime).hw = snd_gf1_pcm_playback;
    snd_pcm_limit_isa_dma_size((*gus).gf1.dma1, core::ptr::addr_of_mut!((*runtime).hw.buffer_bytes_max));
    snd_pcm_limit_isa_dma_size((*gus).gf1.dma1, core::ptr::addr_of_mut!((*runtime).hw.period_bytes_max));
    snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_BYTES, 64);
    0
}

unsafe extern "C" fn snd_gf1_pcm_playback_close(substream: *mut snd_pcm_substream) -> i32 {
    let gus = snd_pcm_substream_chip(substream) as *mut snd_gus_card;
    let runtime = (*substream).runtime;
    let pcmp = (*runtime).private_data as *mut gus_pcm_private;

    if wait_event_timeout(
        core::ptr::addr_of_mut!((*pcmp).sleep),
        (atomic_read(core::ptr::addr_of_mut!((*pcmp).dma_count)) <= 0) as i32,
        2 * HZ,
    ) == 0
    {
        dev_err((*(*gus).card).dev, c"gf1 pcm - serious DMA problem\n".as_ptr());
    }

    snd_gf1_dma_done(gus);
    0
}

unsafe extern "C" fn snd_gf1_pcm_capture_open(substream: *mut snd_pcm_substream) -> i32 {
    let runtime = (*substream).runtime;
    let gus = snd_pcm_substream_chip(substream) as *mut snd_gus_card;

    (*gus).gf1.interrupt_handler_dma_read = Some(snd_gf1_pcm_interrupt_dma_read);
    (*gus).pcm_cap_substream = substream;
    (*(*substream).runtime).hw = snd_gf1_pcm_capture;
    snd_pcm_limit_isa_dma_size((*gus).gf1.dma2, core::ptr::addr_of_mut!((*runtime).hw.buffer_bytes_max));
    snd_pcm_limit_isa_dma_size((*gus).gf1.dma2, core::ptr::addr_of_mut!((*runtime).hw.period_bytes_max));
    snd_pcm_hw_constraint_ratnums(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, core::ptr::addr_of!(hw_constraints_clocks));
    0
}

unsafe extern "C" fn snd_gf1_pcm_capture_close(substream: *mut snd_pcm_substream) -> i32 {
    let gus = snd_pcm_substream_chip(substream) as *mut snd_gus_card;

    (*gus).pcm_cap_substream = core::ptr::null_mut();
    snd_gf1_set_default_handlers(gus, SNDRV_GF1_HANDLER_DMA_READ);
    0
}

unsafe extern "C" fn snd_gf1_pcm_volume_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 127;
    0
}

unsafe extern "C" fn snd_gf1_pcm_volume_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let gus = snd_kcontrol_chip(kcontrol) as *mut snd_gus_card;

    let _flags = spin_lock_irqsave(core::ptr::addr_of_mut!((*gus).pcm_volume_level_lock));
    (*ucontrol).value.integer.value[0] = (*gus).gf1.pcm_volume_level_left1 as _;
    (*ucontrol).value.integer.value[1] = (*gus).gf1.pcm_volume_level_right1 as _;
    spin_unlock_irqrestore(core::ptr::addr_of_mut!((*gus).pcm_volume_level_lock), _flags);
    0
}

unsafe extern "C" fn snd_gf1_pcm_volume_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let gus = snd_kcontrol_chip(kcontrol) as *mut snd_gus_card;
    let change: i32;
    let mut idx: u32;
    let val1: u16;
    let val2: u16;
    let mut vol: u16;
    let mut pcmp: *mut gus_pcm_private;
    let mut pvoice: *mut snd_gus_voice;

    val1 = ((*ucontrol).value.integer.value[0] & 127) as u16;
    val2 = ((*ucontrol).value.integer.value[1] & 127) as u16;
    {
        let _flags = spin_lock_irqsave(core::ptr::addr_of_mut!((*gus).pcm_volume_level_lock));
        change = (val1 != (*gus).gf1.pcm_volume_level_left1 || val2 != (*gus).gf1.pcm_volume_level_right1) as i32;
        (*gus).gf1.pcm_volume_level_left1 = val1;
        (*gus).gf1.pcm_volume_level_right1 = val2;
        (*gus).gf1.pcm_volume_level_left = snd_gf1_lvol_to_gvol_raw((val1 as i32) << 9) << 4;
        (*gus).gf1.pcm_volume_level_right = snd_gf1_lvol_to_gvol_raw((val2 as i32) << 9) << 4;
        spin_unlock_irqrestore(core::ptr::addr_of_mut!((*gus).pcm_volume_level_lock), _flags);
    }
    /* are we active? */
    {
        let _flags = spin_lock_irqsave(core::ptr::addr_of_mut!((*gus).voice_alloc));
        idx = 0;
        while idx < 32 {
            pvoice = core::ptr::addr_of_mut!((*gus).gf1.voices[idx as usize]);
            if (*pvoice).pcm == 0 {
                idx += 1;
                continue;
            }
            pcmp = (*pvoice).private_data as *mut gus_pcm_private;
            if ((*pcmp).flags & SNDRV_GF1_PCM_PFLG_ACTIVE) == 0 {
                idx += 1;
                continue;
            }
            /* load real volume - better precision */
            spin_lock(core::ptr::addr_of_mut!((*gus).reg_lock));
            snd_gf1_select_voice(gus, (*pvoice).number);
            snd_gf1_ctrl_stop(gus, SNDRV_GF1_VB_VOLUME_CONTROL);
            vol = if pvoice == (*pcmp).pvoices[0] {
                (*gus).gf1.pcm_volume_level_left
            } else {
                (*gus).gf1.pcm_volume_level_right
            };
            snd_gf1_write16(gus, SNDRV_GF1_VW_VOLUME, vol);
            spin_unlock(core::ptr::addr_of_mut!((*gus).reg_lock));
            (*pcmp).final_volume = 1;
            idx += 1;
        }
        spin_unlock_irqrestore(core::ptr::addr_of_mut!((*gus).voice_alloc), _flags);
    }
    change
}

static snd_gf1_pcm_volume_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c"PCM Playback Volume".as_ptr(),
    info: Some(snd_gf1_pcm_volume_info),
    get: Some(snd_gf1_pcm_volume_get),
    put: Some(snd_gf1_pcm_volume_put),
};

static snd_gf1_pcm_volume_control1: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c"GPCM Playback Volume".as_ptr(),
    info: Some(snd_gf1_pcm_volume_info),
    get: Some(snd_gf1_pcm_volume_get),
    put: Some(snd_gf1_pcm_volume_put),
};

static snd_gf1_pcm_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_gf1_pcm_playback_open),
    close: Some(snd_gf1_pcm_playback_close),
    hw_params: Some(snd_gf1_pcm_playback_hw_params),
    hw_free: Some(snd_gf1_pcm_playback_hw_free),
    prepare: Some(snd_gf1_pcm_playback_prepare),
    trigger: Some(snd_gf1_pcm_playback_trigger),
    pointer: Some(snd_gf1_pcm_playback_pointer),
    copy: Some(snd_gf1_pcm_playback_copy),
    fill_silence: Some(snd_gf1_pcm_playback_silence),
};

static snd_gf1_pcm_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_gf1_pcm_capture_open),
    close: Some(snd_gf1_pcm_capture_close),
    hw_params: Some(snd_gf1_pcm_capture_hw_params),
    prepare: Some(snd_gf1_pcm_capture_prepare),
    trigger: Some(snd_gf1_pcm_capture_trigger),
    pointer: Some(snd_gf1_pcm_capture_pointer),
};

pub unsafe extern "C" fn snd_gf1_pcm_new(
    gus: *mut snd_gus_card,
    pcm_dev: i32,
    control_index: i32,
) -> i32 {
    let card: *mut snd_card;
    let mut kctl: *mut snd_kcontrol;
    let mut pcm: *mut snd_pcm = core::ptr::null_mut();
    let mut substream: *mut snd_pcm_substream;
    let capture: i32;
    let mut err: i32;

    card = (*gus).card;
    capture = if (*gus).interwave == 0 && (*gus).ess_flag == 0 && (*gus).ace_flag == 0 { 1 } else { 0 };
    err = snd_pcm_new(
        card,
        if (*gus).interwave != 0 { c"AMD InterWave".as_ptr() } else { c"GF1".as_ptr() },
        pcm_dev,
        (*gus).gf1.pcm_channels / 2,
        capture,
        core::ptr::addr_of_mut!(pcm),
    );
    if err < 0 {
        return err;
    }
    (*pcm).private_data = gus as *mut core::ffi::c_void;
    /* playback setup */
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, core::ptr::addr_of!(snd_gf1_pcm_playback_ops));

    substream = (*pcm).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].substream;
    while !substream.is_null() {
        snd_pcm_set_managed_buffer(
            substream,
            SNDRV_DMA_TYPE_DEV,
            (*card).dev,
            64 * 1024,
            if (*gus).gf1.dma1 > 3 { 128 * 1024 } else { 64 * 1024 },
        );
        substream = (*substream).next;
    }

    (*pcm).info_flags = 0;
    (*pcm).dev_subclass = SNDRV_PCM_SUBCLASS_GENERIC_MIX;
    if capture != 0 {
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, core::ptr::addr_of!(snd_gf1_pcm_capture_ops));
        if (*gus).gf1.dma2 == (*gus).gf1.dma1 {
            (*pcm).info_flags |= SNDRV_PCM_INFO_HALF_DUPLEX;
        }
        snd_pcm_set_managed_buffer(
            (*pcm).streams[SNDRV_PCM_STREAM_CAPTURE as usize].substream,
            SNDRV_DMA_TYPE_DEV,
            (*card).dev,
            64 * 1024,
            if (*gus).gf1.dma2 > 3 { 128 * 1024 } else { 64 * 1024 },
        );
    }
    strscpy((*pcm).name.as_mut_ptr(), (*pcm).id.as_ptr());
    if (*gus).interwave != 0 {
        sprintf(
            (*pcm).name.as_mut_ptr().add(strlen((*pcm).name.as_ptr()) as usize),
            c" rev %c".as_ptr(),
            (*gus).revision + b'A',
        );
    }
    strcat((*pcm).name.as_mut_ptr(), c" (synth)".as_ptr());
    (*gus).pcm = pcm;

    if (*gus).codec_flag != 0 {
        kctl = snd_ctl_new1(core::ptr::addr_of!(snd_gf1_pcm_volume_control1), gus as *mut core::ffi::c_void);
    } else {
        kctl = snd_ctl_new1(core::ptr::addr_of!(snd_gf1_pcm_volume_control), gus as *mut core::ffi::c_void);
    }
    if kctl.is_null() {
        return -ENOMEM;
    }
    (*kctl).id.index = control_index;
    err = snd_ctl_add(card, kctl);
    if err < 0 {
        return err;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
